# MMSC serial settings in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved computed transfers were observed in these routine spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/serial-settings-6.4-facts.txt) include all tool sections.

The decoder leaves raw bytes at three span ends:

| Span | Raw-byte start | Exclusive end | Bytes |
|---|---|---|---|
| `_comSetFlag` | `0x12ac4e` | `0x12ac50` | `00 90` |
| `_comSetRxBuf` | `0x12ad4d` | `0x12ad50` | `25 64 00` |
| `_ttyGetTxBuf` | `0x12af7e` | `0x12af80` | `00 90` |

## `_comGetInfo`

- `_comGetInfo`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12aae0 to 0x12ab00.
  Established. The entry reads four bytes at `sp +0x4` (`mov` at 0x12aae3). The calculated EAX
  return is unknown (`ret` at 0x12aaf1). No concrete record address can be assigned from these
  facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12aae3 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12aaf1 | `eax` | `?` |

## `_comGetTxBuf`

- `_comGetTxBuf`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ab20 to 0x12ab40.
  Established. Calls `_ttyGetTxBuf` with unknown candidates (`call` at 0x12ab28). EAX returns
  `ret(_ttyGetTxBuf)` (`ret` at 0x12ab30). The selection transformation remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ab23 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ab28 | `_ttyGetTxBuf 0x12ae90` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ab30 | `eax` | `ret(_ttyGetTxBuf)` |

## `_comGetRxBuf`

- `_comGetRxBuf`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ab00 to 0x12ab20.
  Established. The `_ttyGetRxBuf` call has unknown argument candidates (`call` at 0x12ab08); its
  result reaches the return as EAX `ret(_ttyGetRxBuf)` (`ret` at 0x12ab10). The selected receive
  record is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ab03 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ab08 | `_ttyGetRxBuf 0x12ae40` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ab10 | `eax` | `ret(_ttyGetRxBuf)` |

## `_comSetFlag`

- `_comSetFlag`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12abb0 to 0x12ac50.
  Established. The two-byte OR site reads and writes an unknown effective target with unknown stored
  value (`or` at 0x12abc1). The following `_nvramWrite` candidates begin `0x4` and `0x1d3480`
  (`call` at 0x12abcf). A later `_cmCheckTTYFlags` call presents unknown candidates (`call` at
  0x12abe8). The loop branch takes `le flags condition (operands unknown)` and falls through for its
  negation (`jle` at 0x12abfc). The updated flag field remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12abb6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12abbd | load | 2 | `sp +0x8` | `-` |
| `or` at 0x12abc1 | load | 2 | `(unknown)` | `-` |
| `or` at 0x12abc1 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12abe0 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12abcf | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |
| `call` at 0x12abe8 | `_cmCheckTTYFlags 0x119e40` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x12abfc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ac11 | `eax` | `?` |

## `_comResetFlag`

- `_comResetFlag`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ab40 to 0x12abb0.
  Established. A two-byte AND site stores an unknown value at an unknown effective target (`and` at
  0x12ab54). Calling `_nvramWrite` presents leading candidates `0x4` and `0x1d3480` (`call` at
  0x12ab62). The later loop includes `_cmCheckTTYFlags` with unknown candidates (`call` at
  0x12ab7c); its branch takes `le flags condition (operands unknown)` and falls through for the
  complementary condition (`jle` at 0x12ab90). The record identity and cumulative result are
  unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ab46 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12ab4d | load | 2 | `sp +0x8` | `-` |
| `and` at 0x12ab54 | load | 2 | `(unknown)` | `-` |
| `and` at 0x12ab54 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12ab74 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ab62 | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |
| `call` at 0x12ab7c | `_cmCheckTTYFlags 0x119e40` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x12ab90 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12aba5 | `eax` | `?` |

## `_comSetFunction`

- `_comSetFunction`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ac50 to 0x12acf0.
  Established. The first two-byte store writes `[sp+8]:2` at an unknown target (`mov` at 0x12ac6b).
  A later branch takes `Z == 1` and falls through for `not (Z == 1)` (`je` at 0x12aca3). That
  fall-through includes a two-byte store of `0xffff` at an unknown target (`mov` at 0x12aca5).
  `_nvramWrite` later presents leading candidates `0x4` and `0x1d3480` (`call` at 0x12acd3). The
  assigned function identities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ac59 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12ac5c | load | 2 | `sp +0x8` | `-` |
| `mov` at 0x12ac63 | store | 4 | `sp -0x8` | `[sp+4]-1` |
| `mov` at 0x12ac6b | store | 2 | `(unknown)` | `[sp+8]:2` |
| `test` at 0x12ac72 | load | 1 | `abs 0x1db370` | `-` |
| `cmp` at 0x12ac99 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x12aca0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x12aca5 | store | 2 | `(unknown)` | `0xffff` |
| `test` at 0x12acac | load | 1 | `abs 0x1db370` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ac85 | `_ffscMsg 0x11edf0` | `0x12ac12` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12acbe | `_ffscMsg 0x11edf0` | `0x12ac2e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12acd3 | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12ac79 | `(([0x1db370]:1 & 0xff) & 0x4) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x12ac9e | `Z == 0` | `not (Z == 0)` |
| `je` at 0x12aca3 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x12acb3 | `(([0x1db370]:1 & 0xff) & 0x4) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x4) != 0x0` |
| `jle` at 0x12acca | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ace6 | `eax` | `0x1` |

## `_comSetSpeed`

- `_comSetSpeed`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ad50 to 0x12adc0.
  Established. The `_ttySetSpeed` candidates begin unknown and `[sp+8]` (`call` at 0x12ad68); its
  branch takes `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` and falls through for zero (`jne` at
  0x12ad72). The fall-through stores four-byte `[sp+8]` at an unknown target (`mov` at 0x12ad79),
  then calls `_nvramWrite` with leading candidates `0x4` and `0x1d3480` (`call` at 0x12ad86).
  Neither the descriptor nor rate units are established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ad56 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12ad59 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12ad60 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ad79 | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ad68 | `_ttySetSpeed 0x12b2a0` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x12ad86 | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |
| `call` at 0x12ada7 | `_ffscMsgS 0x11eec0` | `0x12ad29` | `[sp+8]` | `[sp+4]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12ad72 | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) == 0x0` |
| `jne` at 0x12ad8d | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12adba | `eax` | `?` |

## `_comSetRxBuf`

- `_comSetRxBuf`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12acf0 to 0x12ad50.
  Established. Writes four-byte `[sp+8]` at an unknown target (`mov` at 0x12acfd). `_nvramWrite` has
  leading candidates `0x4` and `0x1d3480` (`call` at 0x12ad0a). The branch takes `(ret(_nvramWrite)
  & ret(_nvramWrite)) != 0x0` and falls through for zero (`jne` at 0x12ad11). Those routes return
  EAX `0xffffffff` (`ret` at 0x12ad28) and `0x0` (`ret` at 0x12ad18), respectively. The stored
  setting meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12acf3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12acf6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12acfd | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12ad0a | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12ad11 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ad18 | `eax` | `0x0` |
| `ret` at 0x12ad28 | `eax` | `0xffffffff` |

## `_comSetTxBuf`

- `_comSetTxBuf`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12adc0 to 0x12ae00.
  Established. A four-byte store puts `[sp+8]` at an unknown effective address (`mov` at 0x12adcd).
  The call to `_nvramWrite` presents leading candidates `0x4` and `0x1d3480` (`call` at 0x12adda);
  its decision takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero
  (`jne` at 0x12ade1). EAX returns `0xffffffff` on the taken route (`ret` at 0x12adf8) or `0x0` on
  the fall-through (`ret` at 0x12ade8). The transmit record is not resolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12adc3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12adc6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12adcd | store | 4 | `(unknown)` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12adda | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x12ade1 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ade8 | `eax` | `0x0` |
| `ret` at 0x12adf8 | `eax` | `0xffffffff` |

## `_ttyInit`

- `_ttyInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12af80 to 0x12b240.
  Established. The `_nvramRead` candidates begin `0x4` and `0x1d3480` (`call` at 0x12afb5); its
  branch takes `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` and falls through for nonzero (`je` at
  0x12afbf). The fall-through includes `_bzero` with leading candidates `0x1d3480` and `0x60`
  (`call` at 0x12afc8). Later paths include an `_ioctl` call with leading candidates unknown, `0x3`,
  and unknown (`call` at 0x12b18f), and a four-byte store of `0xffffffff` at an unknown target
  (`mov` at 0x12b1c8). The serial-object layout and option effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12af89 | store | 4 | `sp -0x20` | `0x0` |
| `mov` at 0x12afa3 | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12afdc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12afe2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12afe8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12afec | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12aff0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12aff4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12aff8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12affc | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12b000 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12b008 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12b018 | store | 4 | `sp -0x20` | `0x1` |
| `cmp` at 0x12b031 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b039 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b03e | store | 2 | `(unknown)` | `?` |
| `test` at 0x12b050 | load | 1 | `(unknown)` | `-` |
| `or` at 0x12b057 | load | 2 | `(unknown)` | `-` |
| `or` at 0x12b057 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12b05e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b063 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x12b068 | store | 4 | `sp -0x20` | `0x1` |
| `cmp` at 0x12b075 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x12b098 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x12b0b0 | store | 4 | `sp -0x1c` | `sp-24` |
| `mov` at 0x12b0be | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x12b104 | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x12b110 | store | 4 | `(unknown)` | `ret(_open)` |
| `mov` at 0x12b117 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b12d | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12b140 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b147 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b15b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b162 | load | 4 | `sp -0x1c` | `-` |
| `cmp` at 0x12b172 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b184 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b18c | store | 4 | `sp -0x24` | `?` |
| `mov` at 0x12b197 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x12b19f | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x12b1ad | load | 4 | `ret(___stderr) +0x0` | `-` |
| `mov` at 0x12b1b8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b1c8 | store | 4 | `(unknown)` | `0xffffffff` |
| `cmp` at 0x12b1e0 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x12b1ea | load | 2 | `(unknown)` | `-` |
| `mov` at 0x12b1f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b1f8 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12af90 | `_ttyDrv 0x191a3c` | `ebx` | `esi` | `edi` | `?` | `0x0` | `?` |
| `call` at 0x12af9e | `___stderr 0x18c86c` | `0x12aedd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12afa6 | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12afb5 | `_nvramRead 0x126da8` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |
| `call` at 0x12afc8 | `_bzero 0x1719c0` | `0x1d3480` | `0x60` | `?` | `?` | `?` | `?` |
| `call` at 0x12b082 | `_nvramWrite 0x126f38` | `0x4` | `0x1d3480` | `?` | `?` | `?` | `?` |
| `call` at 0x12b0c2 | `_sprintf 0x172120` | `?` | `0x12aefb` | `?` | `?` | `?` | `?` |
| `call` at 0x12b0cb | `_ttyGetTxBuf 0x12ae90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b0d5 | `_ttyGetRxBuf 0x12ae40` | `?` | `ret(_ttyGetTxBuf)` | `?` | `?` | `?` | `?` |
| `call` at 0x12b0df | `_sysSerialChanGet 0x10b788` | `?` | `ret(_ttyGetRxBuf)` | `?` | `?` | `?` | `?` |
| `call` at 0x12b0e9 | `_ttyDevCreate 0x191a9c` | `?` | `ret(_sysSerialChanGet)` | `?` | `?` | `?` | `?` |
| `call` at 0x12b108 | `_open 0x1877d0` | `[sp-28]` | `0x2` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12b128 | `___stderr 0x18c86c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b130 | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b14f | `_ttySetSpeed 0x12b2a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b18f | `_ioctl 0x187a30` | `?` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x12b1a8 | `___stderr 0x18c86c` | `?` | `[sp-28]` | `?` | `?` | `?` | `?` |
| `call` at 0x12b1b0 | `_fprintf 0x17abdc` | `[ret(___stderr)]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12b1c0 | `_close 0x187970` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12af97 | `(ret(_ttyDrv) & ret(_ttyDrv)) == 0x0` | `(ret(_ttyDrv) & ret(_ttyDrv)) != 0x0` |
| `je` at 0x12afbf | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jle` at 0x12b016 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jl` at 0x12b037 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x12b055 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x12b073 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `je` at 0x12b079 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `jle` at 0x12b0a9 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x12b0f3 | `(ret(_ttyDevCreate) & ret(_ttyDevCreate)) == 0x0` | `(ret(_ttyDevCreate) & ret(_ttyDevCreate)) != 0x0` |
| `jge` at 0x12b120 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x12b159 | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) == 0x0` | `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0` |
| `jne` at 0x12b17a | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x12b19c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x12b1e8 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jle` at 0x12b206 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b217 | `eax` | `0x0` |

## `_ttyReInit`

- `_ttyReInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b240 to 0x12b2a0.
  Established. The initial `_sysSerialHwInit` candidates are entry `ebx`, `esi`, `edi`, and `ebp`
  (`call` at 0x12b246). `_ioctl` in the loop presents leading candidates unknown, `0x1007`, and
  `0x2` (`call` at 0x12b267); its branch takes `(ret(_ioctl) & ret(_ioctl)) == 0x0` and falls
  through for nonzero (`je` at 0x12b271). The body reaches its return with EAX `0x0` (`ret` at
  0x12b29b). Descriptor selection and callee effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b25f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b273 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b246 | `_sysSerialHwInit 0x10b618` | `ebx` | `esi` | `edi` | `ebp` | `?` | `?` |
| `call` at 0x12b267 | `_ioctl 0x187a30` | `?` | `0x1007` | `0x2` | `?` | `?` | `?` |
| `call` at 0x12b280 | `_ffscMsg 0x11edf0` | `0x12b218` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12b271 | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `jle` at 0x12b28e | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b29b | `eax` | `0x0` |

## `_ttyGetInfo`

- `_ttyGetInfo`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ae00 to 0x12ae40.
  Established. The unsigned range branch takes `[sp+4] <= 0x5 (unsigned, 32-bit)` and falls through
  for greater values (`jbe` at 0x12ae0c). On the taken route, another branch takes `([sp+8] &
  [sp+8]) == 0x0` and falls through for nonzero (`je` at 0x12ae22). That fall-through writes an
  unknown four-byte value at `[sp+8] +0x0` (`mov` at 0x12ae2b). The selected information address
  remains unknown at the later return (`ret` at 0x12ae3a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ae03 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12ae06 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12ae24 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ae2b | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12ae0c | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12ae22 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ae13 | `eax` | `0x0` |
| `ret` at 0x12ae3a | `eax` | `?` |

## `_ttyGetRxBuf`

- `_ttyGetRxBuf`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ae40 to 0x12ae90.
  Established. The entry branch takes `[sp+4] <= 0x5 (unsigned, 32-bit)` and falls through for
  larger values (`jbe` at 0x12ae49). A later decision takes `? == 0x0` and falls through for nonzero
  (`je` at 0x12ae6f). The taken path reads four bytes from an unknown target (`mov` at 0x12ae85),
  while the fall-through reads four bytes from another unknown target (`mov` at 0x12ae71). The
  selected receive-buffer address remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ae43 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12ae6a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ae71 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12ae85 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12ae49 | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12ae6f | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12ae53 | `eax` | `0xffffffff` |
| `ret` at 0x12ae78 | `eax` | `?` |
| `ret` at 0x12ae8c | `eax` | `?` |

## `_ttyGetTxBuf`

- `_ttyGetTxBuf`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12ae90 to 0x12af80.
  Established. The initial decision takes `[sp+4] <= 0x5 (unsigned, 32-bit)` and falls through for
  greater values (`jbe` at 0x12ae99). A later branch takes `? == 0x0` and falls through for nonzero
  (`je` at 0x12aebf); its taken and fall-through paths have four-byte loads at unknown targets
  (`mov` at 0x12aed5, `mov` at 0x12aec1), respectively. The two corresponding EAX returns remain
  unknown (`ret` at 0x12aedc, `ret` at 0x12aec8). The default transmit-buffer rule is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12ae93 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12aeba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12aec1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12aed5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12ae99 | `[sp+4] <= 0x5 (unsigned, 32-bit)` | `[sp+4] > 0x5 (unsigned, 32-bit)` |
| `je` at 0x12aebf | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12aea3 | `eax` | `0xffffffff` |
| `ret` at 0x12aec8 | `eax` | `?` |
| `ret` at 0x12aedc | `eax` | `?` |

## `_ttySetSpeed`

- `_ttySetSpeed`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b2a0 to 0x12b2c4.
  Established. Calls `_ioctl` with leading candidates `[sp+4]`, `0x4`, and `[sp+8]` (`call` at
  0x12b2ad). The following decision takes `ge flags condition (operands unknown)` and falls through
  for its negation (`jge` at 0x12b2b6). EAX is unknown after the routes merge at the return (`ret`
  at 0x12b2c2). The rate encoding and ioctl effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b2a3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b2a6 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b2ad | `_ioctl 0x187a30` | `[sp+4]` | `0x4` | `[sp+8]` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x12b2b6 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b2c2 | `eax` | `?` |
