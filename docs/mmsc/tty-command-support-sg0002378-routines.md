# MMSC TTY and command support in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/tty-command-support-sg0002378-facts.txt) include all tool sections.

## `_sysBusIntGen`

- `_sysBusIntGen`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108818 to 0x108828.
  Established. The explicit `eax` assignment supplies `0xffffffff` (`mov` at 0x10881b), retained at
  the exit (`ret` at 0x108823). The fact sheet reports no explicit memory operand, call or conditional branch
  in this span. A bus-interrupt generation role remains unknown.

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

- `_sysMailboxConnect`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108828 to 0x108838.
  Established. The return expression is `0xffffffff` (`ret` at 0x108833), following the constant
  assignment (`mov` at 0x10882b). No mailbox operation is visible in the empty explicit-access and call
  tables. The routine role is unknown.

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

- `_sysMailboxEnable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108838 to 0x108848.
  Established. The immediate load sets `eax` to `0xffffffff` (`mov` at 0x10883b). That value reaches
  the only exit (`ret` at 0x108843). The complete span establishes no mailbox-enable effect.

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

- `_sysBusTas`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108848 to 0x108858.
  Established. The `_vxTas` call presents candidate zero `[sp+4]` (`call` at 0x10884f). The
  four-byte entry load is resolved to `sp +0x4` (`mov` at 0x10884b), and the exit retains
  `ret(_vxTas)` (`ret` at 0x108857). The callee effect is not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10884b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10884f | `_vxTas 0x16f4e8` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108857 | `eax` | `ret(_vxTas)` |

## `_ttyDrv`

- `_ttyDrv`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x19334c to 0x1933ac.
  Established. The branch enters the call path when `[0x1dbc7c] <= 0x0 (signed, 32-bit)`
  (`jle` at 0x193356). The `_iosDrvInstall` site has leading candidates `0x19347c`, `0x0`,
  `0x19347c`, `0x0`, `0x16fb48`, and `0x16fa68` (`call` at 0x193389); its result is stored as four
  bytes at absolute `0x1dbc7c` (`mov` at 0x19338e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x19334f | load | 4 | `abs 0x1dbc7c` | `-` |
| `mov` at 0x19338e | store | 4 | `abs 0x1dbc7c` | `ret(_iosDrvInstall)` |
| `cmp` at 0x193395 | load | 4 | `abs 0x1dbc7c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x193389 | `_iosDrvInstall 0x1707a8` | `0x19347c` | `0x0` | `0x19347c` | `0x0` | `0x16fb48` | `0x16fa68` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x193356 | `[0x1dbc7c] <= 0x0 (signed, 32-bit)` | `[0x1dbc7c] > 0x0 (signed, 32-bit)` |
| `jne` at 0x19339c | `[0x1dbc7c] != 0xffffffff` | `[0x1dbc7c] == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x19335d | `eax` | `0x0` |
| `ret` at 0x1933a6 | `eax` | `?` |

## `_ttyDevCreate`

- `_ttyDevCreate`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1933ac to 0x19347c.
  Established. The `_malloc` call has candidate zero `0xe4` (`call` at 0x1933e1). The four-byte
  store through `ret(_malloc) +0xe0` writes `[sp+8]` (`mov` at 0x193406). Three later indirect calls
  have unknown callees (`call` at 0x19341a, `call` at 0x19342d, `call` at 0x19343e). The third
  presents leading candidates `[sp+8]`, `0x1007`, and `0x2` (`call` at 0x19343e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1933b2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1933b5 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x1933b8 | load | 4 | `abs 0x1dbc7c` | `-` |
| `mov` at 0x1933f5 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x193406 | store | 4 | `ret(_malloc) +0xe0` | `[sp+8]` |
| `mov` at 0x19340c | load | 4 | `[sp+8] +0x0` | `-` |
| `mov` at 0x193417 | load | 4 | `[[sp+8]] +0x8` | `-` |
| `mov` at 0x19341f | load | 4 | `[sp+8] +0x0` | `-` |
| `mov` at 0x19342a | load | 4 | `[[sp+8]] +0x8` | `-` |
| `mov` at 0x193432 | load | 4 | `[sp+8] +0x0` | `-` |
| `mov` at 0x19343c | load | 4 | `[[sp+8]] +0x0` | `-` |
| `mov` at 0x193443 | load | 4 | `abs 0x1dbc7c` | `-` |
| `mov` at 0x193449 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1933c6 | `_errnoSet 0x171240` | `0xc0001` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1933e1 | `_malloc 0x17ec28` | `0xe4` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x1933fa | `_tyDevInit 0x16f4f8` | `ret(_malloc)` | `[sp+12]` | `[sp+16]` | `0x1934dc` | `?` | `?` |
| `call` at 0x19341a | `(unknown callee) (unknown target)` | `[sp+8]` | `0x1` | `0x16fd48` | `ret(_malloc)` | `?` | `?` |
| `call` at 0x19342d | `(unknown callee) (unknown target)` | `[sp+8]` | `0x2` | `0x16fe88` | `ret(_malloc)` | `?` | `?` |
| `call` at 0x19343e | `(unknown callee) (unknown target)` | `[sp+8]` | `0x1007` | `0x2` | `?` | `?` | `?` |
| `call` at 0x19344e | `_iosDevAdd 0x170968` | `ret(_malloc)` | `[sp+4]` | `[0x1dbc7c]` | `?` | `?` | `?` |
| `call` at 0x19345d | `_free 0x17ec48` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1933bf | `[0x1dbc7c] > 0x0 (signed, 32-bit)` | `[0x1dbc7c] <= 0x0 (signed, 32-bit)` |
| `je` at 0x1933ed | `(ret(_malloc) & ret(_malloc)) == 0x0` | `(ret(_malloc) & ret(_malloc)) != 0x0` |
| `jne` at 0x193404 | `(ret(_tyDevInit) & ret(_tyDevInit)) != 0x0` | `(ret(_tyDevInit) & ret(_tyDevInit)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x193470 | `eax` | `?` |

## `_ttyOpen`

- `_ttyOpen`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x19347c to 0x19348c.
  Established. The entry four-byte load reads `sp +0x4` (`mov` at 0x19347f). The exit retains
  `[sp+4]` in `eax` (`ret` at 0x193485). There are no explicit memory stores or visible call sites in this
  span.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x19347f | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x193485 | `eax` | `[sp+4]` |

## `_ttyStartup`

- `_ttyStartup`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1934dc to 0x1934f4.
  Established. The four-byte load through `[sp+4] +0xe0` feeds a nested record access (`mov` at
  0x1934e2). The indirect call has unknown callee and candidate zero `[[sp+4]+224]` (`call` at
  0x1934ee); its result reaches the exit as `ret(unknown)` (`ret` at 0x1934f3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1934df | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1934e2 | load | 4 | `[sp+4] +0xe0` | `-` |
| `mov` at 0x1934e8 | load | 4 | `[[sp+4]+224] +0x0` | `-` |
| `mov` at 0x1934eb | load | 4 | `[[[sp+4]+224]] +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1934ee | `(unknown callee) (unknown target)` | `[[sp+4]+224]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1934f3 | `eax` | `ret(unknown)` |

## `_bufAppendAddMsgs`

- `_bufAppendAddMsgs`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ef28 to 0x10f058.
  Established. The byte store clears `([[sp+8]+16] + [[sp+8]+4]) +0x0` (`mov` at 0x10ef4d). The
  `_bufAppendMsgs` call has leading candidates `[sp+4]` and `[sp+16]` (`call` at 0x10f00c). A later
  `_bufWrite` site presents `[sp+4]`, `[[sp+8]+4]`, and `[sp+12]` (`call` at 0x10f024). The entry
  record roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ef31 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ef39 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10ef3c | load | 4 | `[sp+16] +0x10` | `-` |
| `mov` at 0x10ef47 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10ef4a | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10ef4d | store | 1 | `([[sp+8]+16] + [[sp+8]+4]) +0x0` | `0x0` |
| `mov` at 0x10ef51 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef54 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ef5b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ef6d | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10ef8c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef92 | store | 4 | `sp -0x8` | `[sp+4]+36` |
| `mov` at 0x10ef96 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10efa8 | load | 4 | `[sp+8] +0x4` | `-` |
| `add` at 0x10efab | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10efb2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10efc0 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10efce | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10efd2 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10efd6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10efe8 | load | 4 | `[sp+8] +0x4` | `-` |
| `sub` at 0x10eff7 | load | 4 | `sp +0xc` | `-` |
| `sub` at 0x10eff7 | store | 4 | `sp +0xc` | `?` |
| `mov` at 0x10effa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10effd | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10f004 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10f008 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f018 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10f01c | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10f020 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f032 | load | 4 | `[sp+8] +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ef63 | `_strlen 0x17466c` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef71 | `_strncmp 0x176d68` | `[[sp+8]+4]` | `([sp+4] + [[sp+4]+32])+36` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10ef7e | `_strlen 0x17466c` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef9a | `_strstr 0x180670` | `[[sp+8]+4]` | `[sp+4]+36` | `?` | `?` | `?` | `[sp+4]+36` |
| `call` at 0x10efb6 | `_strlen 0x17466c` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10efda | `_bufWrite 0x10fc28` | `[sp+4]` | `[[sp+8]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10efef | `_bufLeftAlign 0x10f908` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |
| `call` at 0x10f00c | `_bufAppendMsgs 0x10f248` | `[sp+4]` | `[sp+16]` | `?` | `?` | `?` | `?` |
| `call` at 0x10f024 | `_bufWrite 0x10fc28` | `[sp+4]` | `[[sp+8]+4]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10f039 | `_bufLeftAlign 0x10f908` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ef41 | `([[sp+16]+16] & [[sp+16]+16]) == 0x0` | `([[sp+16]+16] & [[sp+16]+16]) != 0x0` |
| `jle` at 0x10ef59 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10ef7b | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jge` at 0x10ef8a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10efa6 | `(ret(_strstr) & ret(_strstr)) == 0x0` | `(ret(_strstr) & ret(_strstr)) != 0x0` |
| `jae` at 0x10efb0 | `ret(_strstr) >= ([[sp+8]+4] + [sp+12]) (unsigned, 32-bit)` | `ret(_strstr) < ([[sp+8]+4] + [sp+12]) (unsigned, 32-bit)` |
| `jge` at 0x10efcc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x10efe6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10f002 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10f016 | `(ret(_bufAppendMsgs) & ret(_bufAppendMsgs)) != 0x0` | `(ret(_bufAppendMsgs) & ret(_bufAppendMsgs)) == 0x0` |
| `jl` at 0x10f030 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f056 | `eax` | `?` |

## `_bufAttachLog`

- `_bufAttachLog`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f498 to 0x10f4b8.
  Established. The four-byte store through `[sp+4] +0x30` writes `[sp+8]` (`mov` at 0x10f4a5). The
  branch bypasses it when `([sp+8] & [sp+8]) == 0x0` (`je` at 0x10f4a3). The return expression
  remains `[sp+8]` (`ret` at 0x10f4ab).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f49b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f49e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f4a5 | store | 4 | `[sp+4] +0x30` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10f4a3 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f4ab | `eax` | `[sp+8]` |

## `_bufDetachLog`

- `_bufDetachLog`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f568 to 0x10f608.
  Established. The four-byte store clears `[sp+4] +0x30` (`mov` at 0x10f56e). The pointer expression
  `[sp+4]` remains in `eax` at the exit (`ret` at 0x10f578). No callee or log-device behavior is
  established in this span.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f56b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f56e | store | 4 | `[sp+4] +0x30` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f578 | `eax` | `[sp+4]` |

## `_sysModel`

- `_sysModel`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b808 to 0x10b828.
  Established. The constant assignment supplies `0x10b801` (`mov` at 0x10b80b), and the return
  retains that value (`ret` at 0x10b813); its pointed-to contents are not established by this body.
  The apparent instructions after the return are not reached by known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b813 | `eax` | `0x10b801` |

## `_sysBspRev`

- `_sysBspRev`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b828 to 0x10b838.
  Established. The return carries constant `0x10b814` (`ret` at 0x10b833), loaded by the prior
  immediate assignment (`mov` at 0x10b82b). No explicit memory operand or call appears in the fact sheet for
  this span. The revision-string contents remain unknown here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b833 | `eax` | `0x10b814` |

## `_cmdGRAPHICS_COMMAND`

- `_cmdGRAPHICS_COMMAND`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117950 to 0x117fb0.
  Established. At the `_msgWrite` site, the leading candidates are `[[0x1dcf9c]+140]` and
  `[0x1dc7b8]` (`call` at 0x117bfa); its result branch is taken when
  `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` (`je` at 0x117c04). On that path, the next branch
  reaches the later store and read-call sequence when `[[[0x1dcf9c]+140]+28] == 0xb`
  (`je` at 0x117c42). That sequence writes four bytes of value `0x6` to
  `[[0x1dcf9c]+144] +0x1c` (`mov` at 0x117c9a), then presents `[[0x1dcf9c]+144]` and
  `[0x1dc7b4]` at `_msgRead` (`call` at 0x117cb3).

  On the path reaching the later branch, the taken arm bypasses the copy, checksum-call
  and byte-store sequence when `[sp-12] == 0x0` (`je` at 0x117e94);
  its fall-through arm, `[sp-12] != 0x0`, reaches `_oobChecksumMsg` with candidate zero
  `[sp-12]` (`call` at 0x117ebd), followed by a byte store with unknown target and value
  (`mov` at 0x117ec8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117959 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11795c | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x117963 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x117969 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x117971 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x117977 | load | 4 | `abs 0x1edd54` | `-` |
| `mov` at 0x117992 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117997 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11799e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1179b0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1179db | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1179e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1179e7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a11 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x117a14 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x117a31 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117a36 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a3d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a70 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117a75 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a7c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a90 | load | 1 | `ret(_strtol) +0x0` | `-` |
| `mov` at 0x117aa8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117aad | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117ab4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117ad0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x117af9 | store | 4 | `sp -0xc` | `ret(_strtol)` |
| `mov` at 0x117afc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x117aff | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x117b18 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117b1d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b24 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b40 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117b63 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117b68 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b6f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b80 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117b85 | load | 4 | `[0x1dcf9c] +0x8c` | `-` |
| `mov` at 0x117b9a | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117b9f | load | 4 | `[0x1dcf9c] +0x8c` | `-` |
| `mov` at 0x117ba5 | load | 4 | `[[0x1dcf9c]+140] +0x8` | `-` |
| `mov` at 0x117bb1 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117bb6 | load | 4 | `[0x1dcf9c] +0x8c` | `-` |
| `mov` at 0x117bbc | store | 4 | `[[0x1dcf9c]+140] +0x1c` | `0x1` |
| `test` at 0x117bc3 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x117bcc | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117bd1 | load | 4 | `[0x1dcf9c] +0x8c` | `-` |
| `mov` at 0x117bd7 | load | 4 | `[[0x1dcf9c]+140] +0x8` | `-` |
| `mov` at 0x117be8 | load | 4 | `abs 0x1dc7b8` | `-` |
| `mov` at 0x117bee | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117bf3 | load | 4 | `[0x1dcf9c] +0x8c` | `-` |
| `mov` at 0x117c0b | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117c10 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c17 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c30 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117c36 | load | 4 | `[0x1dcf9c] +0x8c` | `-` |
| `mov` at 0x117c3c | load | 4 | `[[0x1dcf9c]+140] +0x1c` | `-` |
| `mov` at 0x117c44 | load | 4 | `[[0x1dcf9c]+140] +0x1c` | `-` |
| `mov` at 0x117c5a | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117c5f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c66 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c80 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117c8f | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117c94 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117c9a | store | 4 | `[[0x1dcf9c]+144] +0x1c` | `0x6` |
| `mov` at 0x117ca1 | load | 4 | `abs 0x1dc7b4` | `-` |
| `mov` at 0x117ca7 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117cac | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117cbf | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117cc4 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117cca | load | 4 | `[[0x1dcf9c]+144] +0x1c` | `-` |
| `mov` at 0x117cd2 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117cd7 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117cdd | load | 4 | `[[0x1dcf9c]+144] +0x1c` | `-` |
| `mov` at 0x117cee | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117cf3 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117cf9 | load | 4 | `[[0x1dcf9c]+144] +0x1c` | `-` |
| `mov` at 0x117d15 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117d1a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117d21 | load | 4 | `(unknown)` | `-` |
| `test` at 0x117d30 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x117d39 | load | 4 | `[[0x1dcf9c]+144] +0x8` | `-` |
| `mov` at 0x117d51 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117d56 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117d5c | load | 4 | `[[0x1dcf9c]+144] +0x8` | `-` |
| `mov` at 0x117d6c | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117d71 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117d77 | load | 4 | `[[0x1dcf9c]+144] +0x8` | `-` |
| `mov` at 0x117d7b | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117d80 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117d87 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117da0 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x117db0 | load | 4 | `abs 0x1dcf9c` | `-` |
| `mov` at 0x117db5 | load | 4 | `[0x1dcf9c] +0x90` | `-` |
| `mov` at 0x117dbb | load | 4 | `[[0x1dcf9c]+144] +0x8` | `-` |
| `mov` at 0x117de2 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117de7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117dee | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117e0e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x117e11 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x117e2e | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117e33 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117e3a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117e70 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117e75 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117e7c | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x117e90 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117e96 | load | 1 | `ret(_strtol) +0x2` | `-` |
| `mov` at 0x117e9c | load | 1 | `ret(_strtol) +0x3` | `-` |
| `mov` at 0x117eac | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117eb9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117ec5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117ec8 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x117ecb | load | 1 | `ret(_strtol) +0x1` | `-` |
| `mov` at 0x117ed9 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117ede | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117ee5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117985 | `_ffscMsg 0x11f040` | `0x11769c` | `[sp+8]` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x1179a2 | `_sprintf 0x172dd8` | `?` | `0x1176bf` | `?` | `?` | `?` | `?` |
| `call` at 0x1179bb | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x1179ce | `_ffscMsg 0x11f040` | `0x1176cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1179eb | `_sprintf 0x172dd8` | `?` | `0x116fb9` | `?` | `?` | `?` | `?` |
| `call` at 0x117a07 | `_strtol 0x17fda8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x117a24 | `_ffscMsg 0x11f040` | `0x1176fd` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117a41 | `_sprintf 0x172dd8` | `?` | `0x116fb9` | `?` | `?` | `?` | `?` |
| `call` at 0x117a51 | `_ffscTestAddress 0x125b88` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117a63 | `_ffscMsg 0x11f040` | `0x117723` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117a80 | `_sprintf 0x172dd8` | `?` | `0x116fb9` | `?` | `?` | `?` | `?` |
| `call` at 0x117a9b | `_ffscMsg 0x11f040` | `0x11774c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117ab8 | `_sprintf 0x172dd8` | `?` | `0x116fb9` | `?` | `?` | `?` | `?` |
| `call` at 0x117adb | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x117af1 | `_strtol 0x17fda8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x117b0b | `_ffscMsg 0x11f040` | `0x117778` | `ret(_strtok_r)` | `?` | `?` | `?` | `ret(_strtol)` |
| `call` at 0x117b28 | `_sprintf 0x172dd8` | `?` | `0x116fb9` | `?` | `?` | `?` | `?` |
| `call` at 0x117b44 | `_ffscTestAddress 0x125b88` | `ret(_strtol)` | `?` | `?` | `?` | `ret(_strtol)` | `?` |
| `call` at 0x117b56 | `_ffscMsg 0x11f040` | `0x1177a0` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117b73 | `_sprintf 0x172dd8` | `?` | `0x116fb9` | `?` | `?` | `?` | `?` |
| `call` at 0x117b8c | `_msgClear 0x126058` | `[[0x1dcf9c]+140]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117ba9 | `_sprintf 0x172dd8` | `[[[0x1dcf9c]+140]+8]` | `0x1177cb` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x117be0 | `_ffscMsg 0x11f040` | `0x1177d0` | `[[[0x1dcf9c]+140]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x117bfa | `_msgWrite 0x1269c8` | `[[0x1dcf9c]+140]` | `[0x1dc7b8]` | `?` | `?` | `?` | `?` |
| `call` at 0x117c1b | `_sprintf 0x172dd8` | `?` | `0x1177f3` | `?` | `?` | `?` | `?` |
| `call` at 0x117c4d | `_ffscMsg 0x11f040` | `0x1177fe` | `[[[0x1dcf9c]+140]+28]` | `?` | `?` | `?` | `?` |
| `call` at 0x117c6a | `_sprintf 0x172dd8` | `?` | `0x1177f3` | `?` | `?` | `?` | `?` |
| `call` at 0x117c87 | `_msgClear 0x126058` | `[[0x1dcf9c]+144]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117cb3 | `_msgRead 0x126348` | `[[0x1dcf9c]+144]` | `[0x1dc7b4]` | `?` | `?` | `?` | `?` |
| `call` at 0x117ce6 | `_ffscMsg 0x11f040` | `0x117830` | `[[[0x1dcf9c]+144]+28]` | `?` | `?` | `?` | `?` |
| `call` at 0x117d25 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117d42 | `_ffscMsg 0x11f040` | `0x117880` | `[[[0x1dcf9c]+144]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x117d60 | `_strncmp 0x176d68` | `[[[0x1dcf9c]+144]+8]` | `0x1178ab` | `0x2` | `?` | `?` | `?` |
| `call` at 0x117d8b | `_strcpy 0x18038c` | `?` | `[[[0x1dcf9c]+144]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x117dc2 | `_strtok_r 0x180708` | `[[[0x1dcf9c]+144]+8]+2` | `0x116e90` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x117dd5 | `_ffscMsg 0x11f040` | `0x1178ae` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117df2 | `_sprintf 0x172dd8` | `?` | `0x1178d4` | `?` | `?` | `?` | `?` |
| `call` at 0x117e04 | `_strtol 0x17fda8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x117e21 | `_ffscMsg 0x11f040` | `0x1178e2` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117e3e | `_sprintf 0x172dd8` | `?` | `0x1178d4` | `?` | `?` | `?` | `?` |
| `call` at 0x117e51 | `_ffscTestAddress 0x125b88` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117e63 | `_ffscMsg 0x11f040` | `0x117911` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117e80 | `_sprintf 0x172dd8` | `?` | `0x1178d4` | `?` | `?` | `?` | `?` |
| `call` at 0x117eb1 | `_bcopy 0x176bac` | `ret(_strtol)` | `[sp-12]` | `?` | `?` | `?` | `?` |
| `call` at 0x117ebd | `_oobChecksumMsg 0x12783c` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117ee9 | `_sprintf 0x172dd8` | `?` | `0x117943` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11796f | `[0x1dc204] == [[sp+8]+148]` | `[0x1dc204] != [[sp+8]+148]` |
| `je` at 0x11797d | `[0x1edd54] == [[sp+8]+148]` | `[0x1edd54] != [[sp+8]+148]` |
| `jne` at 0x1179c7 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117a18 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jne` at 0x117a1c | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x117a5b | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) == 0x0` | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) != 0x0` |
| `je` at 0x117a94 | `([ret(_strtol)]:1 & 0xff) == 0xa0` | `([ret(_strtol)]:1 & 0xff) != 0xa0` |
| `je` at 0x117ae7 | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `je` at 0x117b03 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` |
| `je` at 0x117b4e | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) == 0x0` | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) != 0x0` |
| `je` at 0x117bca | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x117c04 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x117c42 | `[[[0x1dcf9c]+140]+28] == 0xb` | `[[[0x1dcf9c]+140]+28] != 0xb` |
| `jne` at 0x117cbd | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x117cd0 | `[[[0x1dcf9c]+144]+28] == 0xb` | `[[[0x1dcf9c]+144]+28] != 0xb` |
| `jne` at 0x117cff | `[[[0x1dcf9c]+144]+28] != 0xa` | `[[[0x1dcf9c]+144]+28] == 0xa` |
| `je` at 0x117d37 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x117d6a | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x117dce | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117e15 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jne` at 0x117e19 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x117e5b | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) == 0x0` | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) != 0x0` |
| `je` at 0x117e94 | `[sp-12] == 0x0` | `[sp-12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x117ef9 | `eax` | `0x0` |

## `_cmdSTEAL`

- `_cmdSTEAL`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116b40 to 0x116d20.
  Established. The signed branch is taken when `[[sp+8]+32] > 0x1f3 (signed, 32-bit)` (`jg` at
  0x116b98). The `_userChangeInputMode` call has candidate zero `[[0x1dc204]+132]` and unknown
  candidates one and two (`call` at 0x116c48); its result branch is taken when
  `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` (`je` at 0x116c52). The global
  record ownership is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x116b43 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x116b46 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116b4e | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116b69 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116b6e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116b75 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116b90 | load | 4 | `[sp+8] +0x20` | `-` |
| `cmp` at 0x116bb0 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x116bbd | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x116bc3 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x116be0 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x116be6 | load | 4 | `[0x1dc204] +0x84` | `-` |
| `mov` at 0x116c10 | load | 4 | `[0x1dc204] +0x18` | `-` |
| `mov` at 0x116c3c | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x116c41 | load | 4 | `[0x1dc204] +0x84` | `-` |
| `cmp` at 0x116c60 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x116c69 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x116c97 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116c9c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ca3 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116b5c | `_ffscMsg 0x11f040` | `0x116380` | `[sp+8]` | `ebp` | `?` | `?` | `?` |
| `call` at 0x116b79 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116bd1 | `_ffscMsg 0x11f040` | `0x116ac0` | `[sp+8]` | `ebp` | `?` | `?` | `?` |
| `call` at 0x116bf6 | `_ffscMsg 0x11f040` | `0x116aed` | `[0x1dc204]` | `ebp` | `?` | `?` | `?` |
| `call` at 0x116c48 | `_userChangeInputMode 0x12bca4` | `[[0x1dc204]+132]` | `?` | `?` | `ebp` | `?` | `?` |
| `call` at 0x116c74 | `_ffscMsg 0x11f040` | `0x116b12` | `[0x1dc204]` | `?` | `?` | `?` | `?` |
| `call` at 0x116ca7 | `_sprintf 0x172dd8` | `?` | `0x115b9f` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x116b4c | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x116b54 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `jg` at 0x116b98 | `[[sp+8]+32] > 0x1f3 (signed, 32-bit)` | `[[sp+8]+32] <= 0x1f3 (signed, 32-bit)` |
| `je` at 0x116bb7 | `[0x1dc204] == 0x0` | `[0x1dc204] != 0x0` |
| `jne` at 0x116bc9 | `[0x1dc204] != [[sp+8]+148]` | `[0x1dc204] == [[sp+8]+148]` |
| `jne` at 0x116bee | `([[0x1dc204]+132] & [[0x1dc204]+132]) != 0x0` | `([[0x1dc204]+132] & [[0x1dc204]+132]) == 0x0` |
| `jne` at 0x116c16 | `[[0x1dc204]+24] != 0x9` | `[[0x1dc204]+24] == 0x9` |
| `je` at 0x116c52 | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `je` at 0x116c67 | `[0x1dc204] == 0x0` | `[0x1dc204] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116b83 | `eax` | `0x0` |
| `ret` at 0x116cb1 | `eax` | `0x0` |
