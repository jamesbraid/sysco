# MMSC 2.2 address-service message paths

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6, MMSC 2.2).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/address-service-2.2-facts.txt) include all tool sections.

## `_addrdInit`

- `_addrdInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dba0 to 0x10dd30.
  Established. Stores four-byte results of `_open`, `_wdCreate` and `_semMCreate` at 0x1b0790,
  0x1f1430 and 0x1f4e64 respectively (`call` at 0x10dbe9, `mov` at 0x10dbf1, `call` at 0x10dc10,
  `mov` at 0x10dc15, `call` at 0x10dc4f, `mov` at 0x10dc57). The `_etherInputHookAdd` call has first
  candidate 0x10e620 (`call` at 0x10dca9).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dbbd | load | 4 | `ret(___errno) +0x0` | `-` |
| `mov` at 0x10dbf1 | store | 4 | `abs 0x1b0790` | `ret(_open)` |
| `mov` at 0x10dc15 | store | 4 | `abs 0x1f1430` | `ret(_wdCreate)` |
| `mov` at 0x10dc57 | store | 4 | `abs 0x1f4e64` | `ret(_semMCreate)` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dbac | `_pipeDevCreate 0x191a24` | `0x10dad2` | `0x17` | `0x3e` | `ebp` | `?` | `?` |
| `call` at 0x10dbb8 | `___errno 0x175038` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbcb | `_ffscMsgS 0x12167c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dbe9 | `_open 0x18cf48` | `0x10dad2` | `0x1` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10dc10 | `_wdCreate 0x178d24` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc23 | `_ffscMsg 0x12154c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc45 | `_lstInit 0x193fe0` | `0x1e1128` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc4f | `_semMCreate 0x17b7d0` | `0x1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dc97 | `_taskSpawn 0x1751c0` | `0x10db6f` | `0x19` | `0x0` | `0x4e20` | `0x10e160` | `0x0` |
| `call` at 0x10dca9 | `_etherInputHookAdd 0x194a20` | `0x10e620` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10dcc5 | `_ffscMsg 0x12154c` | `0x10db76` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dbb6 | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) == 0x0` | `(ret(_pipeDevCreate) & ret(_pipeDevCreate)) != 0x0` |
| `je` at 0x10dbc4 | `[ret(___errno)] == 0xd0006` | `[ret(___errno)] != 0xd0006` |
| `jge` at 0x10dbf8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10dc1c | `(ret(_wdCreate) & ret(_wdCreate)) != 0x0` | `(ret(_wdCreate) & ret(_wdCreate)) == 0x0` |
| `jne` at 0x10dc5e | `(ret(_semMCreate) & ret(_semMCreate)) != 0x0` | `(ret(_semMCreate) & ret(_semMCreate)) == 0x0` |
| `je` at 0x10dca2 | `ret(_taskSpawn) == 0xffffffff` | `ret(_taskSpawn) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dbd8 | `eax` | `0xffffffff` |
| `ret` at 0x10dc30 | `eax` | `0xffffffff` |
| `ret` at 0x10dcb3 | `eax` | `0x0` |
| `ret` at 0x10dcd2 | `eax` | `0xffffffff` |

## `_addrd`

- `_addrd`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e160 to 0x10e470.
  Established. The `_select` call has candidates beginning 0x100, `sp-132`, zero, zero and zero
  (`call` at 0x10e201). The `_read` call later has candidates `ret(_open)`, `sp-68` and 0x3e (`call`
  at 0x10e257). Effects past the unresolved computed jump are incomplete here (`jmp` at 0x10e2f1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e1b3 | store | 4 | `sp -0x8c` | `?` |
| `mov` at 0x10e1c9 | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x10e1cf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e1d5 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10e1d9 | store | 4 | `sp -0x88` | `0x1` |
| `rep movsd` at 0x10e1f0 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x10e1f0 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x10e212 | load | 4 | `ret(___errno) +0x0` | `-` |
| `dec` at 0x10e233 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e233 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e243 | load | 4 | `(unknown)` | `-` |
| `dec` at 0x10e2a6 | load | 4 | `sp -0x88` | `-` |
| `dec` at 0x10e2a6 | store | 4 | `sp -0x88` | `?` |
| `mov` at 0x10e2af | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e2bc | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x10e2ca | store | 4 | `sp -0x36` | `0x2` |
| `mov` at 0x10e2d0 | load | 4 | `sp -0x36` | `-` |
| `mov` at 0x10e2d7 | store | 4 | `sp -0x1a` | `0x0` |
| `mov` at 0x10e2de | store | 4 | `sp -0x36` | `0x3` |
| `mov` at 0x10e2e4 | load | 4 | `sp -0x32` | `-` |
| `jmp` at 0x10e2f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10e390 | load | 4 | `sp -0x32` | `-` |
| `cmp` at 0x10e3a1 | load | 4 | `sp -0x88` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e175 | `_open 0x18cf48` | `0x10dad2` | `0x0` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10e188 | `_ffscMsgS 0x12167c` | `0x10e098` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e1a6 | `_bzero 0x1764e0` | `sp-100` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x10e201 | `_select 0x178588` | `0x100` | `sp-132` | `0x0` | `0x0` | `0x0` | `?` |
| `call` at 0x10e20d | `___errno 0x175038` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e222 | `_ffscMsgS 0x12167c` | `0x10e0b4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e22b | `_close 0x18d0e8` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e257 | `_read 0x18d168` | `ret(_open)` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e268 | `_ffscMsgS 0x12167c` | `0x10e0cb` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e281 | `_ffscMsg 0x12154c` | `0x10e102` | `ret(_read)` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e295 | `_ffscMsg 0x12154c` | `0x10e0ea` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e29e | `_close 0x18d0e8` | `ret(_open)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e399 | `_ffscMsg 0x12154c` | `0x10e134` | `[sp-50]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10e181 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jge` at 0x10e20b | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e217 | `[ret(___errno)] == 0x4` | `[ret(___errno)] != 0x4` |
| `jae` at 0x10e24a | `C == 0` | `not (C == 0)` |
| `jge` at 0x10e261 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10e272 | `(ret(_read) & ret(_read)) == 0x0` | `(ret(_read) & ret(_read)) != 0x0` |
| `je` at 0x10e277 | `ret(_read) == 0x3e` | `ret(_read) != 0x3e` |
| `ja` at 0x10e2b5 | `[sp-54] > 0x1 (unsigned, 32-bit)` | `[sp-54] <= 0x1 (unsigned, 32-bit)` |
| `jle` at 0x10e2c8 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `ja` at 0x10e2d5 | `[sp-54] > 0x2 (unsigned, 32-bit)` | `[sp-54] <= 0x2 (unsigned, 32-bit)` |
| `ja` at 0x10e2eb | `? > 0x6 (unsigned, 32-bit)` | `? <= 0x6 (unsigned, 32-bit)` |
| `jg` at 0x10e3a8 | `[sp-136] > 0x0 (signed, 32-bit)` | `[sp-136] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e3bc | `eax` | `?` |

The computed dispatch cases remain unclaimed because the transfer is unresolved (`jmp` at
0x10e2f1).

## `_addrdEtherInputHook`

- `_addrdEtherInputHook`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e620 to 0x10e670.
  Established. The two-byte read renders as `[sp+8] +0xc` (`mov` at 0x10e62a). `jne` bypasses the
  `_write` route when `([[sp+8]+12]:2 & 0xffff) != 0x801`, while equality falls through to a call
  whose candidates begin `[0x1b0790]`, `[sp+8]` and `[sp+12]` (`jne` at 0x10e632, `call` at
  0x10e63c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e624 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10e627 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10e62a | load | 2 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10e636 | load | 4 | `abs 0x1b0790` | `-` |
| `mov` at 0x10e662 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e63c | `_write 0x18d188` | `[0x1b0790]` | `[sp+8]` | `[sp+12]` | `ebx` | `ebp` | `?` |
| `call` at 0x10e64d | `_ffscMsgS 0x12167c` | `0x10e5fe` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e632 | `([[sp+8]+12]:2 & 0xffff) != 0x801` | `([[sp+8]+12]:2 & 0xffff) == 0x801` |
| `je` at 0x10e646 | `ret(_write) == [sp+12]` | `ret(_write) != [sp+12]` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e668 | `eax` | `?` |

## `_addrdUpdatePartInfo`

- `_addrdUpdatePartInfo`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d750 to 0x10d810.
  Established. Calls `_contains_module` with first candidates 0x1b1040, `[[sp+4]+12]` and
  `[[sp+4]+8]` (`call` at 0x10d77e). The later `_update_partition_info` call begins with 0x1b1040
  and `sp-132` (`call` at 0x10d7cf).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d75b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d771 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10d775 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10d790 | load | 4 | `abs 0x1b1040` | `-` |
| `mov` at 0x10d799 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d7a0 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10d7a4 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d7b0 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10d7b4 | load | 4 | `[sp+4] +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d769 | `_memset 0x183fd8` | `sp-132` | `0x0` | `0x80` | `ebx` | `esi` | `?` |
| `call` at 0x10d77e | `_contains_module 0x1307e0` | `0x1b1040` | `[[sp+4]+12]` | `[[sp+4]+8]` | `?` | `?` | `?` |
| `call` at 0x10d7be | `_sprintf 0x176c40` | `sp-132` | `?` | `[[sp+4]+12]` | `[[sp+4]+8]` | `?` | `?` |
| `call` at 0x10d7cf | `_update_partition_info 0x130480` | `0x1b1040` | `sp-132` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d788 | `(ret(_contains_module) & ret(_contains_module)) == 0x0` | `(ret(_contains_module) & ret(_contains_module)) != 0x0` |
| `je` at 0x10d797 | `[0x1b1040] == 0x0` | `[0x1b1040] != 0x0` |
| `je` at 0x10d79e | `([[sp+4]+16] & [[sp+4]+16]) == 0x0` | `([[sp+4]+16] & [[sp+4]+16]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d7f0 | `eax` | `?` |

## `_addrdSendPartitionInfo`

- `_addrdSendPartitionInfo`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d810 to 0x10d870.
  Established. Two explicit four-byte stores place 3 at `sp -0x34` and 7 at `sp -0x30` (`mov` at
  0x10d819, `mov` at 0x10d820). The following `_addrdBroadcastMsg` call has first candidate `sp-52`
  (`call` at 0x10d83c); its nonzero result takes `jne` to the `_ffscDebugWrite` route, while zero
  falls through to the zero-valued exit (`jne` at 0x10d846, `call` at 0x10d857, `ret` at
  0x10d84d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d816 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d819 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10d820 | store | 4 | `sp -0x30` | `0x7` |
| `mov` at 0x10d827 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x10d82a | store | 4 | `sp -0x2c` | `[[sp+4]+4]` |
| `mov` at 0x10d82d | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d82f | store | 4 | `sp -0x28` | `[[sp+4]]` |
| `mov` at 0x10d832 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x10d835 | store | 4 | `sp -0x24` | `[[sp+4]+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d83c | `_addrdBroadcastMsg 0x10ded0` | `sp-52` | `0x3` | `0x7` | `[[sp+4]+4]` | `[[sp+4]]` | `[[sp+4]+8]` |
| `call` at 0x10d857 | `_ffscDebugWrite 0x1214fc` | `0x10d7f1` | `0x13` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10d846 | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d84d | `eax` | `0x0` |
| `ret` at 0x10d864 | `eax` | `0xffffffff` |

## `_addrdBroadcastMsg`

- `_addrdBroadcastMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ded0 to 0x10df50.
  Established. A two-byte store places 0x108 at `sp -0x8` (`mov` at 0x10df01). The `_etherOutput`
  candidates begin `[0x1f141c]`, `sp-20`, `[sp+4]` and 0x30 (`call` at 0x10df11). The earlier
  `_bcopy` sites have third candidate 6 at both calls (`call` at 0x10dee9, `call` at 0x10def9).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ded8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10df01 | store | 2 | `sp -0x8` | `0x108` |
| `mov` at 0x10df0b | load | 4 | `abs 0x1f141c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dee9 | `_bcopy 0x17aa14` | `0x1f4f88` | `sp-14` | `0x6` | `ebx` | `esi` | `?` |
| `call` at 0x10def9 | `_bcopy 0x17aa14` | `0x1d73b8` | `sp-20` | `0x6` | `?` | `?` | `?` |
| `call` at 0x10df11 | `_etherOutput 0x1948e0` | `[0x1f141c]` | `sp-20` | `[sp+4]` | `0x30` | `?` | `?` |
| `call` at 0x10df35 | `_ffscMsgS 0x12167c` | `0x10deac` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10df1b | `(ret(_etherOutput) & ret(_etherOutput)) != 0x0` | `(ret(_etherOutput) & ret(_etherOutput)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10df47 | `eax` | `?` |

## `_addrdInsertIdentity`

- `_addrdInsertIdentity`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10dfb0 to 0x10e030.
  Established. Two four-byte stores write `[[sp+8]+12]` through `[sp+4] +0x2c` and `[[sp+8]+16]`
  through `[sp+4] +0x1c` (`mov` at 0x10dfef, `mov` at 0x10dff5). The earlier `_bcopy` calls have
  candidates `[sp+8]`, `[sp+4]+32`, 8 and `[sp+8]+8`, `[sp+4]+40`, 4 respectively (`call` at
  0x10dfd5, `call` at 0x10dfe7).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10dfb5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dfb8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10dfc0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10dfc4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10dfec | load | 4 | `[sp+8] +0xc` | `-` |
| `mov` at 0x10dfef | store | 4 | `[sp+4] +0x2c` | `[[sp+8]+12]` |
| `mov` at 0x10dff2 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10dff5 | store | 4 | `[sp+4] +0x1c` | `[[sp+8]+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10dfd5 | `_bcopy 0x17aa14` | `[sp+8]` | `[sp+4]+32` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10dfe7 | `_bcopy 0x17aa14` | `[sp+8]+8` | `[sp+4]+40` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10dfcc | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e000 | `eax` | `[[sp+8]+16]` |

## `_addrdLocalMsg`

- `_addrdLocalMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e030 to 0x10e160.
  Established. The two-byte local store carries 0x108 at `sp -0x38` (`mov` at 0x10e049). `_write`
  receives candidates beginning `[0x1b0790]`, `sp-68` and 0x3e, and `jne` selects the `_ffscMsgS`
  route when `ret(_write) != 0x3e` (`call` at 0x10e067, `jne` at 0x10e072, `call` at 0x10e085).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e038 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10e049 | store | 2 | `sp -0x38` | `0x108` |
| `mov` at 0x10e061 | load | 4 | `abs 0x1b0790` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e041 | `_bzero 0x1764e0` | `sp-68` | `0x3e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e056 | `_bcopy 0x17aa14` | `[sp+4]` | `sp-54` | `0x30` | `?` | `?` | `?` |
| `call` at 0x10e067 | `_write 0x18d188` | `[0x1b0790]` | `sp-68` | `0x3e` | `?` | `?` | `?` |
| `call` at 0x10e085 | `_ffscMsgS 0x12167c` | `0x10e001` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10e072 | `ret(_write) != 0x3e` | `ret(_write) == 0x3e` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e097 | `eax` | `?` |

## `_addrdUpdateLocalIdentity`

- `_addrdUpdateLocalIdentity`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ddd0 to 0x10ded0.
  Established. The three early `je` sites bypass calls for `[0x1f4b78] == [[sp+4]+8]`, `[0x1f4b7c]
  == [[sp+4]+12]` and `[0x1f4b80] == [[sp+4]+16]` respectively (`je` at 0x10dde6, `je` at 0x10de02,
  `je` at 0x10de1e). Later explicit stores place four-byte values 3 and 1 at `sp -0x34` and `sp
  -0x30` (`mov` at 0x10de79, `mov` at 0x10de80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ddd8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10dddd | load | 4 | `[sp+4] +0x8` | `-` |
| `cmp` at 0x10dde0 | load | 4 | `abs 0x1f4b78` | `-` |
| `mov` at 0x10ddf9 | load | 4 | `[sp+4] +0xc` | `-` |
| `cmp` at 0x10ddfc | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x10de04 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10de15 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x10de18 | load | 4 | `abs 0x1f4b80` | `-` |
| `mov` at 0x10de20 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10de34 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10de38 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10de79 | store | 4 | `sp -0x34` | `0x3` |
| `mov` at 0x10de80 | store | 4 | `sp -0x30` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ddec | `_identSetIPAddr 0x125d70` | `[sp+4]+8` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x10de08 | `_identSetRackID 0x125f40` | `[[sp+4]+12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de24 | `_identSetCapabilities 0x125c90` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de64 | `_identSetModuleNums 0x125e60` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de6d | `_identUpdate 0x125a50` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10de8c | `_addrdInsertIdentity 0x10dfb0` | `sp-52` | `[sp+4]` | `?` | `?` | `0x3` | `0x1` |
| `call` at 0x10de95 | `_addrdBroadcastMsg 0x10ded0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10dde6 | `[0x1f4b78] == [[sp+4]+8]` | `[0x1f4b78] != [[sp+4]+8]` |
| `je` at 0x10de02 | `[0x1f4b7c] == [[sp+4]+12]` | `[0x1f4b7c] != [[sp+4]+12]` |
| `je` at 0x10de1e | `[0x1f4b80] == [[sp+4]+16]` | `[0x1f4b80] != [[sp+4]+16]` |
| `jne` at 0x10de41 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10de47 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x10de4b | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10de77 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x10de9c | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) == 0x0` | `(ret(_addrdBroadcastMsg) & ret(_addrdBroadcastMsg)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10deab | `eax` | `?` |

## `_addrdExtractIdentity`

- `_addrdExtractIdentity`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10df50 to 0x10dfb0.
  Established. The `_bcopy` candidates are `[sp+4]+32`, `[sp+8]`, 8 at one site and `[sp+4]+40`,
  `[sp+8]+8`, 4 at the other (`call` at 0x10df75, `call` at 0x10df87). Later four-byte stores copy
  `[[sp+4]+44]` through `[sp+8] +0xc` and `[[sp+4]+28]` through `[sp+8] +0x10` (`mov` at 0x10df8f,
  `mov` at 0x10df95).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10df55 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10df58 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10df60 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10df64 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10df8c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10df8f | store | 4 | `[sp+8] +0xc` | `[[sp+4]+44]` |
| `mov` at 0x10df92 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10df95 | store | 4 | `[sp+8] +0x10` | `[[sp+4]+28]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10df75 | `_bcopy 0x17aa14` | `[sp+4]+32` | `[sp+8]` | `0x8` | `?` | `?` | `?` |
| `call` at 0x10df87 | `_bcopy 0x17aa14` | `[sp+4]+40` | `[sp+8]+8` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10df6c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10dfa0 | `eax` | `[[sp+4]+28]` |

## `_addrdProcessAssignMsg`

- `_addrdProcessAssignMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10e760 to 0x10e870.
  Established. The initial `_addrdExtractIdentity` call has first candidates `[sp+4]` and `sp-32`
  (`call` at 0x10e770). The following `_ffscCompareUniqueIDs` call begins with `[sp+4]+32` and
  0x1f4b70 (`call` at 0x10e781); its zero result takes `je` to `_addrdUpdateLocalIdentity`,
  whose first candidate is `sp-32` (`je` at 0x10e78b, `call` at 0x10e7a1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10e768 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x10e78d | load | 1 | `abs 0x1e0bf0` | `-` |
| `test` at 0x10e7ba | load | 1 | `abs 0x1e0bf0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10e770 | `_addrdExtractIdentity 0x10df50` | `[sp+4]` | `sp-32` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10e781 | `_ffscCompareUniqueIDs 0x127f50` | `[sp+4]+32` | `0x1f4b70` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7a1 | `_addrdUpdateLocalIdentity 0x10ddd0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7b2 | `_ffscMsg 0x12154c` | `0x10e71a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7c8 | `_ffscMsg 0x12154c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10e7d1 | `_identPrint 0x125b80` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10e78b | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0` | `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0` |
| `je` at 0x10e794 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x10e7ab | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |
| `je` at 0x10e7c1 | `(([0x1e0bf0]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10e7e0 | `eax` | `0x0` |

## `_addrdResolveConflicts`

- `_addrdResolveConflicts`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ef00 to 0x10ef80.
  Established. `jne` selects the `_identScanForConflicts` route when `[0x1f50e4] != 0x0`, with zero
  falling through to the first all-ones return (`jne` at 0x10ef0a, `ret` at 0x10ef1e). The scan
  call has first candidates 0x10ef80 and 0x10f020 (`call` at 0x10ef2a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10ef03 | load | 4 | `abs 0x1f50e4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ef11 | `_ffscMsg 0x12154c` | `0x10ee9c` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef2a | `_identScanForConflicts 0x1257c0` | `0x10ef80` | `0x10f020` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10ef45 | `_ffscMsg 0x12154c` | `0x10eecc` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ef0a | `[0x1f50e4] != 0x0` | `[0x1f50e4] == 0x0` |
| `jne` at 0x10ef34 | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) != 0x0` | `(ret(_identScanForConflicts) & ret(_identScanForConflicts)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ef1e | `eax` | `0xffffffff` |
| `ret` at 0x10ef3b | `eax` | `0x0` |
| `ret` at 0x10ef52 | `eax` | `0xffffffff` |

## `_addrdResolveIPAddrConflict`

- `_addrdResolveIPAddrConflict`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ef80 to 0x10f020.
  Established. The `_memcpy` call has first candidates `sp-32`, unknown and 0x1c (`call` at
  0x10efb7). `_addrdFindUnusedIPAddr` then receives `sp-24` as its first candidate (`call` at
  0x10efc6); its nonzero result takes `jne` to `_ffscMsg`, while zero falls through to
  `_addrdAssignNewIdentity` (`jne` at 0x10efd0, `call` at 0x10efe5, `call` at 0x10efd3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ef88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef8b | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ef90 | `_ffscCompareUniqueIDs 0x127f50` | `[sp+4]` | `[sp+8]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x10efb7 | `_memcpy 0x183fb8` | `sp-32` | `?` | `0x1c` | `?` | `?` | `?` |
| `call` at 0x10efc6 | `_addrdFindUnusedIPAddr 0x10e670` | `sp-24` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10efd3 | `_addrdAssignNewIdentity 0x10e470` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10efe5 | `_ffscMsg 0x12154c` | `0x10ef53` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10ef9a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10efd0 | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) != 0x0` | `(ret(_addrdFindUnusedIPAddr) & ret(_addrdFindUnusedIPAddr)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10eff7 | `eax` | `?` |

## `_identSetIPAddr`

- `_identSetIPAddr`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125d70 to 0x125e60.
  Established. Explicit four-byte stores place `[[sp+4]]` at 0x1f4b78, `ret(_inet_netof)` at
  0x1e116c and `[sp-28]` at 0x1e1168 (`mov` at 0x125de2, `mov` at 0x125de7, `mov` at 0x125df0).
  `_nvramWrite` receives first candidates 1 and 0x1e1168 (`call` at 0x125dfd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125d79 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125d7c | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x125d87 | store | 4 | `sp -0x1c` | `ret(_inet_lnaof)` |
| `mov` at 0x125d8a | load | 4 | `[sp+4] +0x0` | `-` |
| `cmp` at 0x125d97 | load | 4 | `abs 0x1e116c` | `-` |
| `mov` at 0x125db0 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x125de0 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x125de2 | store | 4 | `abs 0x1f4b78` | `[[sp+4]]` |
| `mov` at 0x125de7 | store | 4 | `abs 0x1e116c` | `ret(_inet_netof)` |
| `mov` at 0x125ded | load | 4 | `sp -0x1c` | `-` |
| `mov` at 0x125df0 | store | 4 | `abs 0x1e1168` | `[sp-28]` |
| `test` at 0x125e16 | load | 1 | `abs 0x1e0bf1` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125d7f | `_inet_lnaof 0x18c7c8` | `[[sp+4]]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x125d8d | `_inet_netof 0x18c908` | `[[sp+4]]` | `?` | `?` | `?` | `ret(_inet_lnaof)` | `?` |
| `call` at 0x125da4 | `_ffscMsg 0x12154c` | `0x125cb5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125db3 | `_inet_ntoa_b 0x18caf8` | `[[sp+4]]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x125dc1 | `_ifAddrSet 0x18d748` | `0x125cdb` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x125dd3 | `_ffscMsgS 0x12167c` | `0x125ce0` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x125dfd | `_nvramWrite 0x129b50` | `0x1` | `0x1e1168` | `?` | `?` | `?` | `?` |
| `call` at 0x125e0e | `_ffscMsg 0x12154c` | `0x125d0a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x125e25 | `_ffscMsg 0x12154c` | `0x125d48` | `sp-24` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x125d9d | `[0x1e116c] == ret(_inet_netof)` | `[0x1e116c] != ret(_inet_netof)` |
| `je` at 0x125dcb | `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0` | `(ret(_ifAddrSet) & ret(_ifAddrSet)) != 0x0` |
| `je` at 0x125e07 | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` |
| `je` at 0x125e1d | `(([0x1e0bf1]:1 & 0xff) & 0x1) == 0x0` | `(([0x1e0bf1]:1 & 0xff) & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125e35 | `eax` | `?` |

## `_identFindByModuleNum`

- `_identFindByModuleNum`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125310 to 0x125380.
  Established. The explicit four-byte store through `[sp+8] +0x0` has an unknown value (`mov` at
  0x125344). The prior `je` bypasses that site for `([sp+8] & [sp+8]) == 0x0`, while nonzero
  falls through (`je` at 0x125342). The backward `jle` retains an unknown compared value in `? <=
  0x1 (signed, 32-bit)` (`jle` at 0x125354).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x125316 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x125319 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x125338 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x125344 | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x125326 | `_lstFirst 0x194170` | `0x1d8c88` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x125357 | `_lstNext 0x194220` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12531f | `[sp+4] == 0xffffffff` | `[sp+4] != 0xffffffff` |
| `jne` at 0x12533e | `Z == 0` | `not (Z == 0)` |
| `je` at 0x125342 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `jle` at 0x125354 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `jne` at 0x125363 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x125370 | `eax` | `?` |
