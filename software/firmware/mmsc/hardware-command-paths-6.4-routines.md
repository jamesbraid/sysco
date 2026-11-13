# MMSC hardware command paths in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/hardware-command-paths-6.4-facts.txt) include all tool sections.

## `_cmdTTY_REINIT`

- `_cmdTTY_REINIT`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118360 to 0x118428.
  Established. The signed branch takes `[[sp+8]+32] > 0x3e7` and falls through for values at
  most `0x3e7` (`jg` at 0x118371). The taken path of `jne` at 0x1183ab includes `_taskDelay`
  with leading candidate `0x3c` (`call` at 0x1183c2), `_taskLock` with unknown candidates
  (`call` at 0x1183ca), and `_ttyReInit` with unknown candidates (`call` at 0x1183cf). A
  four-byte store writes `0xb` at an unknown effective target (`mov` at 0x118411). The
  reinitialization authority, timing units and device effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118363 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x118366 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118369 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x118378 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11837d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118384 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1183a0 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1183a5 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x1183ea | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1183f5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1183fc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118405 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11840a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118411 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118388 | `_sprintf 0x172120` | `?` | `0x116c50` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1183c2 | `_taskDelay 0x171620` | `0x3c` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x1183ca | `_taskLock 0x1714f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1183cf | `_ttyReInit 0x12b240` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1183dd | `_ffscMsg 0x11edf0` | `0x11832b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1183e5 | `_taskUnlock 0x171520` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118400 | `_sprintf 0x172120` | `?` | `0x116dc9` | `[0x1ef244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x118371 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jne` at 0x1183ab | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `je` at 0x1183d6 | `(ret(_ttyReInit) & ret(_ttyReInit)) == 0x0` | `(ret(_ttyReInit) & ret(_ttyReInit)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118392 | `eax` | `0x0` |
| `ret` at 0x1183b5 | `eax` | `0x1` |
| `ret` at 0x118420 | `eax` | `0x1` |

## `_cmdRESET_NVRAM`

- `_cmdRESET_NVRAM`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1182a0 to 0x118360.
  Established. The threshold branch takes `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` and falls
  through for the complementary condition (`jg` at 0x1182b1). The fall-through of `je` at
  0x1182eb calls `_nvramReset` with leading candidate entry `ebp` and other candidates unknown
  (`call` at 0x1182ed). A four-byte store places `0xb` at an unknown effective address (`mov` at
  0x11831b). The reset selection and persistent-storage consequences are unknown from this
  caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1182a3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1182a6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1182a9 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1182b8 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1182bd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1182c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1182e0 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1182e5 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x1182f4 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1182ff | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118306 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11830f | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x118314 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11831b | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1182c8 | `_sprintf 0x172120` | `?` | `0x116c50` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1182ed | `_nvramReset 0x126e48` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11830a | `_sprintf 0x172120` | `?` | `0x11828a` | `[0x1ef244]` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1182b1 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x1182eb | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1182d2 | `eax` | `0x0` |
| `ret` at 0x11832a | `eax` | `0x1` |

## `_cmdREBOOT_FFSC`

- `_cmdREBOOT_FFSC`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118200 to 0x1182a0.
  Established. The first signed branch takes `[[sp+8]+32] > 0x3e7` and falls through for values
  no greater than `0x3e7` (`jg` at 0x118211). The fall-through of `je` at 0x11824c sets
  four-byte absolute `0x1ec85c` to `0x1` (`mov` at 0x11824e). Another four-byte store has
  unknown target and value `0xb` (`mov` at 0x11827a). The two exits carry EAX zero (`ret` at
  0x118232) and `0x1` (`ret` at 0x118289). The global request meaning and reboot mechanism
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118203 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x118206 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118209 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x118218 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11821d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118224 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118240 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x118246 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11824e | store | 4 | `abs 0x1ec85c` | `0x1` |
| `mov` at 0x11825e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118265 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11826e | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x118273 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11827a | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118228 | `_sprintf 0x172120` | `?` | `0x116c50` | `ebp` | `?` | `?` | `?` |
| `call` at 0x118269 | `_sprintf 0x172120` | `?` | `0x1181dc` | `[0x1ef244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x118211 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x11824c | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118232 | `eax` | `0x0` |
| `ret` at 0x118289 | `eax` | `0x1` |

## `_cmdPWR`

- `_cmdPWR`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117fe0 to 0x118200.
  Established. The `_strtok_r` call presents leading candidates `0x0`, `0x116c60`, and `[sp+4]`
  (`call` at 0x117ffa). `_destNumRacks` has leading candidates `[sp+12]` and `0x0` (`call` at
  0x118030); its branch takes `ret(_destNumRacks) > 0x1 (signed, 32-bit)` and falls through for
  values at most `0x1` (`jg` at 0x11803d). The fall-through of `je` at 0x11806b reaches a
  four-byte store of `0x1` at `[[sp+8]+136] +0x1c` (`mov` at 0x1180ad). The power-command
  routing and delay calculation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117fec | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x117fef | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11802c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118065 | load | 4 | `[sp+8] +0x10` | `-` |
| `imul` at 0x118074 | load | 4 | `abs 0x1dae78` | `-` |
| `fld` at 0x11807c | load | 8 | `abs 0x117fd8` | `-` |
| `fidivr` at 0x118083 | load | 4 | `sp -0x118` | `-` |
| `fstp` at 0x11808c | store | 8 | `sp -0x11c` | `?` |
| `mov` at 0x118095 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x11809b | load | 4 | `[[sp+8]+136] +0x8` | `-` |
| `mov` at 0x1180a7 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x1180ad | store | 4 | `[[sp+8]+136] +0x1c` | `0x1` |
| `mov` at 0x1180b4 | load | 4 | `abs 0x1dae30` | `-` |
| `mov` at 0x1180ba | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x1180cd | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x1180d3 | load | 4 | `[[sp+8]+136] +0x1c` | `-` |
| `mov` at 0x1180ed | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1180f2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1180f9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118110 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x11811f | load | 4 | `abs 0x1dae78` | `-` |
| `mov` at 0x118138 | store | 4 | `sp -0x100` | `?` |
| `mov` at 0x118165 | store | 4 | `sp -0xfc` | `?` |
| `mov` at 0x118178 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11817b | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x118198 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1181a0 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1181b0 | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117ffa | `_strtok_r 0x17edf8` | `0x0` | `0x116c60` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x11800a | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x117f5d` | `?` | `?` | `?` | `?` |
| `call` at 0x11801e | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x117f5f` | `?` | `?` | `?` | `?` |
| `call` at 0x118030 | `_destNumRacks 0x11f15c` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x11805d | `_sprintf 0x172120` | `sp-248` | `0x117f61` | `ret(_strtok_r)` | `?` | `?` | `?` |
| `call` at 0x11809f | `_sprintf 0x172120` | `[[[sp+8]+136]+8]` | `0x117f68` | `?` | `?` | `?` | `?` |
| `call` at 0x1180c1 | `_msgWrite 0x126698` | `[[sp+8]+136]` | `[0x1dae30]` | `?` | `?` | `?` | `?` |
| `call` at 0x1180e0 | `_ffscMsg 0x11edf0` | `0x117fa6` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1180fd | `_sprintf 0x172120` | `?` | `0x117fcc` | `?` | `?` | `?` | `?` |
| `call` at 0x118117 | `_msgClear 0x125d48` | `[[sp+8]+136]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118190 | `_nanosleep 0x17a010` | `sp-256` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1181a6 | `_routeProcessELSCCommand 0x12934c` | `sp-248` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x1181b7 | `_remStartRemoteCommand 0x1289d0` | `[sp+8]` | `sp-248` | `?` | `[sp+12]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x118016 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x118028 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jg` at 0x11803d | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` |
| `je` at 0x11806b | `[[sp+8]+16] == 0x3` | `[[sp+8]+16] != 0x3` |
| `jne` at 0x1180cb | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` |
| `je` at 0x1180d9 | `[[[sp+8]+136]+28] == 0xb` | `[[[sp+8]+136]+28] != 0xb` |
| `je` at 0x118181 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x118185 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11819e | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x1181c8 | `? <= 0x8 (unsigned, 32-bit)` | `? > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1181db | `eax` | `?` |

## `_cmdRACKID`

- `_cmdRACKID`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113508 to 0x1136c8.
  Established. The `_strtok_r` call has leading candidates `0x0`, `0x112943`, and `[sp+4]`
  (`call` at 0x113522); its branch takes `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` and falls
  through for zero (`jne` at 0x11352e). That fall-through, followed by the fall-through of `je`
  at 0x11353c, reaches `_ffscRackIDToString` with leading candidates `[0x1ef244]` and `sp-44`
  (`call` at 0x113547). A later four-byte store saves `ret(_strtoul)` at `sp -0x14` (`mov` at
  0x113617). The rack-change policy and parsed-value interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113511 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113514 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x113517 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113530 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x113536 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x113550 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11355b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113562 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11356b | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x113570 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113577 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x113588 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1135d8 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1135dd | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `rep movsd` at 0x113606 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x113606 | load | 4 | `abs 0x1ef238` | `-` |
| `mov` at 0x113617 | store | 4 | `sp -0x14` | `ret(_strtoul)` |
| `mov` at 0x11361a | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x11361d | load | 1 | `[sp-48] +0x0` | `-` |
| `cmp` at 0x113623 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x11366a | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11366f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113676 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113688 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x113693 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11369a | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x113522 | `_strtok_r 0x17edf8` | `0x0` | `0x112943` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x113547 | `_ffscRackIDToString 0x1257f8` | `[0x1ef244]` | `sp-44` | `?` | `?` | `?` | `?` |
| `call` at 0x113566 | `_sprintf 0x172120` | `?` | `0x11344d` | `[0x1ef244]` | `sp-44` | `?` | `?` |
| `call` at 0x1135ab | `_destNumRacks 0x11f15c` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1135be | `_ffscMsg 0x11edf0` | `0x113458` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x11360f | `_strtoul 0x17e63c` | `ret(_strtok_r)` | `sp-48` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11362f | `_ffscMsg 0x11edf0` | `0x113494` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x11364c | `_addrdRequestRackID 0x10dae0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11365d | `_ffscMsg 0x11edf0` | `0x1134a9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11367a | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11369e | `_sprintf 0x172120` | `?` | `0x1134f2` | `[0x1ef244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11352e | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x11353c | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jg` at 0x113590 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jle` at 0x1135b6 | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` |
| `jne` at 0x1135e3 | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `jne` at 0x113621 | `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) != 0x0` | `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x113627 | `ret(_strtoul) <= 0x9 (unsigned, 32-bit)` | `ret(_strtoul) > 0x9 (unsigned, 32-bit)` |
| `je` at 0x113656 | `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) == 0x0` | `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1136ae | `eax` | `?` |

## `_cmdSCAN`

- `_cmdSCAN`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1136c8 to 0x113898.
  Established. A four-byte store initializes `sp -0x1c` to zero (`mov` at 0x1136d4). The first
  branch takes `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` and falls through for values at most
  `0x3e7` (`jg` at 0x1136e3). Calls inside later conditional paths include `_elscSetup` with
  unknown candidates (`call` at 0x113765) and `_elscQueryModuleNum` with leading candidates
  `[sp-24]` and `sp-8` (`call` at 0x113779). The fall-through of `je` at 0x11378c writes four
  bytes of `[sp-8]` at an unknown target (`mov` at 0x11378e). The scan selection and
  controller-response meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1136d1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1136d4 | store | 4 | `sp -0x1c` | `0x0` |
| `mov` at 0x1136db | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1136ea | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1136ef | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1136f6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113708 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11370d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113710 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11371c | store | 4 | `sp -0x18` | `0x0` |
| `mov` at 0x11372c | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x113732 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113735 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11373e | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x113746 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113749 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `test` at 0x113755 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x113775 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x113785 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x113788 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11378e | store | 4 | `(unknown)` | `[sp-8]` |
| `mov` at 0x113792 | store | 4 | `sp -0x1c` | `0x1` |
| `mov` at 0x11379d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1137af | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137b3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137c8 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1137d0 | store | 4 | `sp -0x1c` | `0x1` |
| `mov` at 0x1137dc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137f0 | store | 4 | `(unknown)` | `0xb` |
| `inc` at 0x1137fa | load | 4 | `sp -0x18` | `-` |
| `inc` at 0x1137fa | store | 4 | `sp -0x18` | `?` |
| `cmp` at 0x1137fd | load | 4 | `sp -0x18` | `-` |
| `cmp` at 0x113807 | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1136fa | `_sprintf 0x172120` | `?` | `0x1129c3` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x113765 | `_elscSetup 0x121700` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113779 | `_elscQueryModuleNum 0x121170` | `[sp-24]` | `sp-8` | `?` | `?` | `?` | `?` |
| `call` at 0x1137a1 | `_ffscModuleNumToString 0x125478` | `[sp-8]` | `sp-20` | `?` | `?` | `?` | `?` |
| `call` at 0x1137b7 | `_sprintf 0x172120` | `?` | `0x1136b7` | `sp-20` | `?` | `?` | `?` |
| `call` at 0x1137e4 | `_sprintf 0x172120` | `?` | `0x1136af` | `?` | `?` | `?` | `?` |
| `call` at 0x11380d | `_elscUpdateModuleNums 0x121870` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1136e3 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x113716 | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jb` at 0x113744 | `C == 1` | `not (C == 1)` |
| `jge` at 0x11374f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11375c | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11376f | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x113783 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x11378c | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x113801 | `[sp-24] <= 0x1 (unsigned, 32-bit)` | `[sp-24] > 0x1 (unsigned, 32-bit)` |
| `je` at 0x11380b | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113820 | `eax` | `?` |

## `_cmdIPADDR`

- `_cmdIPADDR`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117d80 to 0x117f30.
  Established. The `_strtok_r` call has leading candidates `0x0`, `0x116c60`, and `[sp+4]`
  (`call` at 0x117d9a). The branch takes `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` and falls
  through for zero (`jne` at 0x117da6). That fall-through, followed by the fall-through of `je`
  at 0x117db3, reaches `_inet_ntoa_b` with leading candidates `[0x1ef240]` and `sp-24` (`call`
  at 0x117dc3). A four-byte store places `0xb` at an unknown target (`mov` at 0x117df3). The
  query-versus-change semantics and network effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117d89 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x117d8c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x117d8f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117da8 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117dad | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x117dbd | load | 4 | `abs 0x1ef240` | `-` |
| `mov` at 0x117dcc | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117dd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117dde | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117de7 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117dec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117df3 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x117e00 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x117e50 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117e55 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `rep movsd` at 0x117e7e | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x117e7e | load | 4 | `abs 0x1ef238` | `-` |
| `mov` at 0x117ecf | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117ed4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117edb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117ef4 | load | 4 | `abs 0x1ef240` | `-` |
| `mov` at 0x117f08 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117f0d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117f14 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117d9a | `_strtok_r 0x17edf8` | `0x0` | `0x116c60` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x117dc3 | `_inet_ntoa_b 0x187380` | `[0x1ef240]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x117de2 | `_sprintf 0x172120` | `?` | `0x117cca` | `[0x1ef244]` | `sp-24` | `?` | `?` |
| `call` at 0x117e23 | `_destNumRacks 0x11f15c` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x117e36 | `_ffscMsg 0x11edf0` | `0x117cd5` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x117e88 | `_ffscParseIPAddr 0x125538` | `ret(_strtok_r)` | `sp-44` | `?` | `?` | `?` | `?` |
| `call` at 0x117e9a | `_ffscMsg 0x11edf0` | `0x117d06` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117eb1 | `_addrdUpdateLocalIdentity 0x10db80` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117ec2 | `_ffscMsg 0x11edf0` | `0x117d1e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117edf | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117efa | `_inet_ntoa_b 0x187380` | `[0x1ef240]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x117f18 | `_sprintf 0x172120` | `?` | `0x117d74` | `sp-24` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x117da6 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x117db3 | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jg` at 0x117e08 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jle` at 0x117e2e | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` |
| `jne` at 0x117e5b | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `je` at 0x117e92 | `(ret(_ffscParseIPAddr) & ret(_ffscParseIPAddr)) == 0x0` | `(ret(_ffscParseIPAddr) & ret(_ffscParseIPAddr)) != 0x0` |
| `je` at 0x117ebb | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x117f28 | `eax` | `?` |

## `_cmdDIRECT`

- `_cmdDIRECT`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115b70 to 0x115dd0.
  Established. The branch takes `[0x1db644] != [sp+8]` and falls through for equality (`jne` at
  0x115b7d). The `_userChangeInputMode` call has leading candidates unknown, `0x8`, and `0x0`
  (`call` at 0x115bd5). The fall-through of `jne` at 0x115bfc sets four-byte absolute `0x1ef898`
  to `0x1` (`mov` at 0x115c23). The exit carries EAX zero (`ret` at 0x115c60). The direct
  connection and global flag meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x115b74 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x115b77 | load | 4 | `abs 0x1db644` | `-` |
| `mov` at 0x115b7f | load | 4 | `abs 0x1db648` | `-` |
| `cmp` at 0x115b90 | load | 4 | `abs 0x1db648` | `-` |
| `mov` at 0x115bb0 | load | 4 | `abs 0x1db644` | `-` |
| `mov` at 0x115bb6 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x115bf0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115bf6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c00 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c0e | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115c13 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c1a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c23 | store | 4 | `abs 0x1ef898` | `0x1` |
| `mov` at 0x115c43 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115c48 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c4f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c5a | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x115b9e | `_ffscMsg 0x11edf0` | `0x115ae8` | `[sp+8]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x115bd5 | `_userChangeInputMode 0x12b444` | `?` | `0x8` | `0x0` | `ebx` | `ebp` | `?` |
| `call` at 0x115c1e | `_sprintf 0x172120` | `?` | `0x115b50` | `0x8000` | `?` | `0x7` | `?` |
| `call` at 0x115c36 | `_ffscMsg 0x11edf0` | `0x115b2f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115c53 | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x115b7d | `[0x1db644] != [sp+8]` | `[0x1db644] == [sp+8]` |
| `je` at 0x115b96 | `[0x1db648] == [sp+8]` | `[0x1db648] != [sp+8]` |
| `jg` at 0x115bbe | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x115bdf | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `jne` at 0x115bfc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x115c60 | `eax` | `0x0` |

## `_cmdCOM`

- `_cmdCOM`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x112c28 to 0x113248.
  Established. Calls `_strtok_r` with leading candidates `0x0`, `0x112943`, and `[sp+4]` (`call`
  at 0x112c51). `_strtol` has leading candidates `ret(_strtok_r)`, `sp-8`, and `0x0` (`call` at
  0x112c7f). A subsequent computed transfer has an unresolved target (`jmp` at 0x112dc5). The
  recorded four-byte store at an unknown effective target writes `0xb` (`mov` at 0x1131f4).
  Effects and calls cover known edges only, leaving the selected serial-control operations
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x112c31 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x112c34 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x112c3a | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x112c46 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x112c89 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x112c8c | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x112cb8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x112ce6 | load | 2 | `ret(_comGetInfo) +0x6` | `-` |
| `mov` at 0x112d0d | load | 2 | `ret(_comGetInfo) +0x4` | `-` |
| `mov` at 0x112d21 | load | 2 | `ret(_comGetInfo) +0x4` | `-` |
| `mov` at 0x112d36 | load | 4 | `ret(_comGetInfo) +0x0` | `-` |
| `mov` at 0x112d3a | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x112d45 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112d4c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112d68 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x112d6b | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x112db8 | load | 4 | `sp -0x20` | `-` |
| `jmp` at 0x112dc5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131b9 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x1131cf | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1131d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131db | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131e8 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1131ed | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131f4 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x112c51 | `_strtok_r 0x17edf8` | `0x0` | `0x112943` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x112c64 | `_ffscMsg 0x11edf0` | `0x112946` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112c7f | `_strtol 0x17e498` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x112ca0 | `_ffscMsg 0x11edf0` | `0x112966` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x112cc3 | `_strtok_r 0x17edf8` | `0x0` | `0x112943` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x112cd6 | `_comGetInfo 0x12aae0` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112cf1 | `_ffscIntToToken 0x1253f8` | `0x1ab338` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112cfa | `_comGetTxBuf 0x12ab20` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112d04 | `_comGetRxBuf 0x12ab00` | `ret(_strtol)` | `ret(_comGetTxBuf)` | `?` | `?` | `?` | `?` |
| `call` at 0x112d50 | `_sprintf 0x172120` | `?` | `0x11297c` | `[0x1ef244]` | `ret(_strtol)` | `[ret(_comGetInfo)]` | `sp-28` |
| `call` at 0x112d92 | `_ffscTokenToInt 0x125998` | `0x1ab300` | `ret(_strtok_r)` | `sp-32` | `?` | `?` | `?` |
| `call` at 0x112da5 | `_ffscMsg 0x11edf0` | `0x1129d3` | `ret(_strtok_r)` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x1131c2 | `_ffscMsg 0x11edf0` | `0x112be2` | `[sp-32]` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x1131df | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x112c40 | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x112c5d | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x112c90 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x112c98 | `ret(_strtol)-1 <= 0x5 (unsigned, 32-bit)` | `ret(_strtol)-1 > 0x5 (unsigned, 32-bit)` |
| `jne` at 0x112ccf | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x112d19 | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |
| `je` at 0x112d2d | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `jg` at 0x112d73 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x112d9c | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) == 0x0` | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) != 0x0` |
| `ja` at 0x112dbf | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113209 | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x112dc5).

## `_cmdELSC`

- `_cmdELSC`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115dd0 to 0x116230.
  Established. The `_strspn` call has leading candidates `[[sp+4]]` and `0x115910` (`call` at
  0x115df1). The fall-through of `je` at 0x115dfe calls `_routeProcessELSCCommand` with leading
  candidates `[[sp+4]]`, `[sp+8]`, and `[sp+12]` (`call` at 0x115e03). A separate path calls
  `_destNumBays` with leading candidates `[sp+12]`, `sp-8`, and `sp-12` (`call` at 0x115e69).
  The taken path of `je` at 0x115fc0 writes four bytes of `0xffffffff` at `[[sp+8]+132] +0x14`
  (`mov` at 0x115fd6). The selected controller channel and ownership policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x115ddc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x115ddf | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x115de2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x115de5 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x115df9 | load | 1 | `(ret(_strspn) + [[sp+4]]) +0x0` | `-` |
| `mov` at 0x115e20 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x115e27 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x115e2e | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115e36 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115e90 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115e9c | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x115e9f | load | 4 | `abs 0x1ef244` | `-` |
| `test` at 0x115ec0 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x115ec9 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x115ecf | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x115ee0 | load | 4 | `sp -0xc` | `-` |
| `sub` at 0x115ee6 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x115ef1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115ef8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f0a | load | 4 | `sp -0xc` | `-` |
| `sub` at 0x115f10 | load | 4 | `sp -0xc` | `-` |
| `movsx` at 0x115f13 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x115f19 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115f24 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f2b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f34 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115f39 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f40 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x115f82 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x115f88 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x115f8c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x115fa4 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115fa9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115fb0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115fd0 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x115fd6 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x115fdd | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115fe2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115fe9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115fec | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x116000 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x116003 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11600f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116029 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11602e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116035 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116064 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116067 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11606e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116074 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11607d | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116082 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116089 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160ad | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1160b2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160c0 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x1160d2 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1160d9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1160e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160f5 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1160fa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116101 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x11611d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116124 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11612f | store | 4 | `abs 0x1ef898` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x115df1 | `_strspn 0x17ed04` | `[[sp+4]]` | `0x115910` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x115e03 | `_routeProcessELSCCommand 0x12934c` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x115e44 | `_ffscMsg 0x11edf0` | `0x115c61` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x115e69 | `_destNumBays 0x11f10c` | `[sp+12]` | `sp-8` | `sp-12` | `?` | `?` | `?` |
| `call` at 0x115e7c | `_ffscMsg 0x11edf0` | `0x115c81` | `ret(_destNumBays)` | `?` | `?` | `?` | `?` |
| `call` at 0x115eac | `_ffscMsg 0x11edf0` | `0x115cb3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115ed8 | `_ffscMsg 0x11edf0` | `0x115ce5` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x115f02 | `_ffscMsg 0x11edf0` | `0x115d11` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115f2f | `_sprintf 0x172120` | `?` | `0x115d2f` | `[0x1ef244]` | `?` | `?` | `?` |
| `call` at 0x115f61 | `_userAcquireRemote 0x12b374` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115f9c | `_sprintf 0x172120` | `sp-256` | `0x115d42` | `[sp-12]` | `[[[sp+8]+132]+20]` | `0x3` | `?` |
| `call` at 0x115fb6 | `_userSendCommand 0x12c1d4` | `ret(_userAcquireRemote)` | `sp-256` | `?` | `?` | `?` | `?` |
| `call` at 0x115fc3 | `_userReleaseRemote 0x12c134` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11601c | `_ffscMsg 0x11edf0` | `0x115d57` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116039 | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116058 | `_remStartConnectCommand 0x128620` | `[sp+8]` | `0x115da1` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x11608d | `_sprintf 0x172120` | `?` | `0x115da6` | `?` | `?` | `?` | `?` |
| `call` at 0x1160f0 | `_sprintf 0x172120` | `?` | `0x115d2f` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x116128 | `_sprintf 0x172120` | `?` | `?` | `0x8000` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x115de9 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x115dfe | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) == 0x0` | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) != 0x0` |
| `je` at 0x115e34 | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x115e3c | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x115e74 | `ret(_destNumBays) == 0x1` | `ret(_destNumBays) != 0x1` |
| `jne` at 0x115e96 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x115ea5 | `[0x1ef244] == [sp-8]` | `[0x1ef244] != [sp-8]` |
| `je` at 0x115ec7 | `(([0x1db370]:1 & 0xff) & 0x20) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x115ef6 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x115f6d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x115fc0 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x116009 | `[0x1ef244] == [sp-8]` | `[0x1ef244] != [sp-8]` |
| `jne` at 0x116015 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116062 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1160d7 | `? == 0x1` | `? != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116145 | `eax` | `?` |

## `_cmdDOWNLOADER`

- `_cmdDOWNLOADER`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116de0 to 0x1170c0.
  Established. The signed branch takes `[[sp+8]+32] > 0x3e7` and falls through for lower or
  equal values (`jg` at 0x116df6). `_strtok_r` presents leading candidates `0x0`, `0x116c60`,
  and `[sp+4]` (`call` at 0x116e18). The result branch takes `(ret(_strtok_r) & ret(_strtok_r))
  != 0x0` and falls through for zero (`jne` at 0x116e24). The taken route calls
  `_ffscConvertToUpperCase` with leading candidate `ret(_strtok_r)` (`call` at 0x116e41). The
  downloader selection and device mode changes remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x116de5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116de8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x116deb | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116dee | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x116e95 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116e9a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ea1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116eb2 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116eb7 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x116ed5 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116ee0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ee7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ef0 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116ef5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116efc | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116e18 | `_strtok_r 0x17edf8` | `0x0` | `0x116c60` | `[sp+4]` | `ebx` | `esi` | `ebp` |
| `call` at 0x116e2b | `_ffscMsg 0x11edf0` | `0x116d52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116e41 | `_ffscConvertToUpperCase 0x1252e8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116e4f | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x116d93` | `?` | `?` | `?` | `?` |
| `call` at 0x116e76 | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x116d9a` | `?` | `?` | `?` | `?` |
| `call` at 0x116e88 | `_ffscMsg 0x11edf0` | `0x116da2` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x116ea5 | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116ec3 | `_enable_downloader 0x10cb28` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116ed0 | `_disable_downloader 0x10cb68` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116eeb | `_sprintf 0x172120` | `?` | `0x116dc9` | `[0x1ef244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x116df6 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jne` at 0x116e24 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x116e59 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x116e80 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `je` at 0x116ebd | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `je` at 0x116ec1 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116f10 | `eax` | `?` |

## `_cmdFLASH`

- `_cmdFLASH`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1170c0 to 0x117720.
  Established. The `_destNumRacks` call has leading candidates `[sp+12]` and `sp-8` (`call` at
  0x117118). The branch takes `ret(_destNumRacks) == 0x1` and falls through for inequality (`je`
  at 0x117123). The taken path of `je` at 0x1172dc writes four bytes of `0xffffffff` at
  `[[sp+8]+132] +0x14` (`mov` at 0x1172f6). Other paths include four-byte stores of `0x1` at
  absolute `0x1ef898` (`mov` at 0x1173f4, `mov` at 0x117453). EAX is zero at the exit (`ret` at
  0x11746b). The firmware transfer and flash-device effects are unknown from this handler.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1170cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1170cf | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1170d2 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x1170d9 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1170e8 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1170ed | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1170f4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117114 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117232 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11723e | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x117241 | load | 4 | `abs 0x1ef244` | `-` |
| `test` at 0x117280 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x117289 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x11728f | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1172a2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1172a8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1172c0 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1172c5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1172cc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1172f0 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1172f6 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x1172fd | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117302 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117309 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11730c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x117320 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x117323 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11732f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x117350 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117369 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11736c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117373 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117379 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117382 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x117387 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11738e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173a6 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x1173ac | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x1173b8 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1173bd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173dd | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1173e2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173e9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1173f4 | store | 4 | `abs 0x1ef898` | `0x1` |
| `mov` at 0x117406 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x11740c | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x117410 | load | 4 | `[[sp+8]+148] +0x4c` | `-` |
| `mov` at 0x11741b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117422 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117432 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x117438 | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x117443 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11744a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117453 | store | 4 | `abs 0x1ef898` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1170f8 | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117118 | `_destNumRacks 0x11f15c` | `[sp+12]` | `sp-8` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x11712b | `_ffscMsg 0x11edf0` | `0x116f11` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x117148 | `_strtok_r 0x17edf8` | `0x0` | `0x116c60` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x117161 | `_ffscConvertToUpperCase 0x1252e8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11716f | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x116f38` | `?` | `?` | `?` | `?` |
| `call` at 0x117181 | `_ffscMsg 0x11edf0` | `0x116f44` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1171a8 | `_strtok_r 0x17edf8` | `0x0` | `0x116c60` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x1171bb | `_ffscMsg 0x11edf0` | `0x116f62` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1171d1 | `_ffscConvertToUpperCase 0x1252e8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1171df | `_strcmp 0x17e920` | `?` | `0x116f3d` | `?` | `?` | `?` | `?` |
| `call` at 0x117206 | `_strcmp 0x17e920` | `?` | `0x116f83` | `?` | `?` | `?` | `?` |
| `call` at 0x117218 | `_ffscMsg 0x11edf0` | `0x116f8b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11724e | `_ffscMsg 0x11edf0` | `0x116fb7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117261 | `_userAcquireRemote 0x12b374` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117298 | `_ffscMsg 0x11edf0` | `0x116feb` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x1172b8 | `_sprintf 0x172120` | `sp-252` | `0x117017` | `[[[sp+8]+132]+20]` | `0x6` | `?` | `?` |
| `call` at 0x1172d2 | `_userSendCommand 0x12c1d4` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x1172df | `_userReleaseRemote 0x12c134` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11733c | `_ffscMsg 0x11edf0` | `0x117026` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11735b | `_remStartConnectCommand 0x128620` | `[sp+8]` | `0x11706d` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x117392 | `_sprintf 0x172120` | `?` | `0x117073` | `?` | `?` | `?` | `?` |
| `call` at 0x1173ed | `_sprintf 0x172120` | `?` | `0x1170a0` | `0x10c0` | `ret(_remStartConnectCommand)` | `0x4` | `?` |
| `call` at 0x117426 | `_sprintf 0x172120` | `?` | `0x117084` | `?` | `?` | `[[[sp+8]+148]+80]` | `?` |
| `call` at 0x11744e | `_sprintf 0x172120` | `?` | `0x1170a0` | `0x40` | `[[[sp+8]+148]+80]` | `0x6` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1170e1 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x117123 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x117154 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x117179 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x1171b4 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x1171e9 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x117210 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x117238 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x117247 | `[0x1ef244] == [sp-8]` | `[0x1ef244] != [sp-8]` |
| `jne` at 0x11726d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x117287 | `(([0x1db370]:1 & 0xff) & 0x20) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x1172dc | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x117329 | `[0x1ef244] == [sp-8]` | `[0x1ef244] != [sp-8]` |
| `jne` at 0x117335 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x117367 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1173a2 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x117402 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11746b | `eax` | `0x0` |

## `_cmdCONSOLE`

- `_cmdCONSOLE`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115980 to 0x115b70.
  Established. The `_strspn` call presents leading candidates `[[sp+4]]` and `0x115910` (`call`
  at 0x1159a1). A branch takes `[0x1da8b4] != [[sp+8]+148]` and falls through on equality (`jne`
  at 0x1159bd). The fall-through of `jne` at 0x1159fb calls `_userSendCommand` with leading
  candidates `[[0x1da8b4]+132]`, `sp-248`, and unknown (`call` at 0x115a33). The return carries
  EAX zero (`ret` at 0x115ae7). The console ownership and connection behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11598b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11598e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x115991 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1159b1 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x1159b7 | load | 4 | `abs 0x1da8b4` | `-` |
| `cmp` at 0x1159d0 | load | 4 | `abs 0x1da8b4` | `-` |
| `mov` at 0x1159f0 | load | 4 | `abs 0x1da8b4` | `-` |
| `mov` at 0x1159f5 | load | 4 | `[0x1da8b4] +0x18` | `-` |
| `mov` at 0x115a16 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115a1b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115a22 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115a27 | load | 4 | `abs 0x1da8b4` | `-` |
| `mov` at 0x115a2c | load | 4 | `[0x1da8b4] +0x84` | `-` |
| `mov` at 0x115a50 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115a58 | load | 4 | `[sp+8] +0x10` | `-` |
| `cmp` at 0x115a80 | load | 4 | `abs 0x1da8b4` | `-` |
| `mov` at 0x115a89 | load | 4 | `abs 0x1da8b4` | `-` |
| `mov` at 0x115a8e | load | 4 | `[0x1da8b4] +0x18` | `-` |
| `mov` at 0x115a9d | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115aa2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115aa9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115ac5 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x115aca | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115ad1 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1159a1 | `_strspn 0x17ed04` | `[[sp+4]]` | `0x115910` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x115a0e | `_sprintf 0x172120` | `sp-248` | `0x115934` | `([[sp+4]] + ret(_strspn))` | `?` | `?` | `?` |
| `call` at 0x115a33 | `_userSendCommand 0x12c1d4` | `[[0x1da8b4]+132]` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x115a66 | `_ffscMsg 0x11edf0` | `0x11593e` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x115aad | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115ad5 | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x115995 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x1159ab | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1159bd | `[0x1da8b4] != [[sp+8]+148]` | `[0x1da8b4] == [[sp+8]+148]` |
| `jne` at 0x1159d7 | `[0x1da8b4] != 0x0` | `[0x1da8b4] == 0x0` |
| `jne` at 0x1159fb | `[[0x1da8b4]+24] != 0x9` | `[[0x1da8b4]+24] == 0x9` |
| `jne` at 0x115a3d | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` |
| `je` at 0x115a56 | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x115a5e | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x115a87 | `[0x1da8b4] == 0x0` | `[0x1da8b4] != 0x0` |
| `jne` at 0x115a94 | `[[0x1da8b4]+24] != 0x9` | `[[0x1da8b4]+24] == 0x9` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x115ae7 | `eax` | `0x0` |

## `_cmdDEST`

- `_cmdDEST`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113c50 to 0x113d40.
  Established. The branch takes `(? & 0x2) != 0x0` and falls through when the expression is zero
  (`jne` at 0x113c5e); its fall-through calls `_memcpy` with leading candidates `[sp+8]+36`,
  `[sp+12]`, and `0x60` (`call` at 0x113c67), then writes four bytes of unknown value at `[sp+8]
  +0x24` (`mov` at 0x113c74). `_strlen` has leading candidate `[sp+8]+49` (`call` at 0x113c9f).
  The branch takes `ret(_strlen) <= 0x46 (unsigned, 32-bit)` and falls through for greater
  lengths (`jbe` at 0x113caf). The destination-record and saved-setting meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113c54 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x113c57 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113c5a | load | 4 | `[sp+12] +0x0` | `-` |
| `mov` at 0x113c6f | load | 4 | `[sp+8] +0x24` | `-` |
| `mov` at 0x113c74 | store | 4 | `[sp+8] +0x24` | `?` |
| `test` at 0x113c77 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x113cbd | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x113cc2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113cc9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113cd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x113c67 | `_memcpy 0x17e840` | `[sp+8]+36` | `[sp+12]` | `0x60` | `ebx` | `ebp` | `?` |
| `call` at 0x113c8a | `_ffscMsg 0x11edf0` | `0x113c0c` | `[sp+8]` | `[sp+8]+49` | `?` | `?` | `?` |
| `call` at 0x113c93 | `_userUpdateSaveInfo 0x12c2b4` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113c9f | `_strlen 0x1739b4` | `[sp+8]+49` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113ccd | `_sprintf 0x172120` | `?` | `0x113c3e` | `[sp+8]+49` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x113c5e | `(? & 0x2) != 0x0` | `(? & 0x2) == 0x0` |
| `je` at 0x113c7e | `(([0x1db370]:1 & 0xff) & 0x20) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x20) != 0x0` |
| `jbe` at 0x113caf | `ret(_strlen) <= 0x46 (unsigned, 32-bit)` | `ret(_strlen) > 0x46 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113cda | `eax` | `0x0` |

## `_cmdFFSC`

- `_cmdFFSC`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116230 to 0x1165e0.
  Established. The `_strspn` call presents leading candidates `[[sp+4]]` and `0x115910` (`call`
  at 0x116251). The fall-through of `je` at 0x11626b calls `_cmdProcessFFSCCommand` with leading
  candidates `[[sp+4]]`, `[sp+8]`, and `[sp+12]` (`call` at 0x116275). The taken path of `je` at
  0x1163cc includes a four-byte store of `0xffffffff` at `[[sp+8]+132] +0x14` (`mov` at
  0x1163e6). Later paths include a four-byte store of `0x1` at absolute `0x1ef898` (`mov` at
  0x1164fe). The MMSC mode transition and global scheduling meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11623c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11623f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116242 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116245 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x116259 | load | 1 | `(ret(_strspn) + [[sp+4]]) +0x0` | `-` |
| `mov` at 0x116260 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116265 | load | 1 | `([0x1ef244] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11626f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116272 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x11628b | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116290 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116297 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162c0 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x1162c7 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1162cf | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116320 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11632c | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x11632f | load | 4 | `abs 0x1ef244` | `-` |
| `test` at 0x116350 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x116359 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x11635f | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x116392 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116398 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1163b0 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1163b5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1163bc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1163e0 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1163e6 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x1163ed | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1163f2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1163f9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1163fc | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x116410 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x116413 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11641f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116439 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x11643e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116445 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116474 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116477 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11647e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116484 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11648d | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x116492 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116499 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164bd | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x1164c2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164c9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164f3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164fe | store | 4 | `abs 0x1ef898` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116251 | `_strspn 0x17ed04` | `[[sp+4]]` | `0x115910` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x116275 | `_cmdProcessFFSCCommand 0x1117f0` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x11629b | `_sprintf 0x172120` | `?` | `0x116146` | `?` | `?` | `?` | `?` |
| `call` at 0x1162dd | `_ffscMsg 0x11edf0` | `0x116150` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1162f5 | `_destNumRacks 0x11f15c` | `[sp+12]` | `sp-8` | `?` | `?` | `?` | `?` |
| `call` at 0x116308 | `_ffscMsg 0x11edf0` | `0x116171` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x11633c | `_ffscMsg 0x11edf0` | `0x11619a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116368 | `_ffscMsg 0x11edf0` | `0x1161ce` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x116371 | `_userAcquireRemote 0x12b374` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1163a8 | `_sprintf 0x172120` | `sp-252` | `0x1161fb` | `[[[sp+8]+132]+20]` | `0x2` | `?` | `?` |
| `call` at 0x1163c2 | `_userSendCommand 0x12c1d4` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x1163cf | `_userReleaseRemote 0x12c134` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11642c | `_ffscMsg 0x11edf0` | `0x115d57` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116449 | `_sprintf 0x172120` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116468 | `_remStartConnectCommand 0x128620` | `[sp+8]` | `0x11620a` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x11649d | `_sprintf 0x172120` | `?` | `0x115da6` | `?` | `?` | `?` | `?` |
| `call` at 0x1164cd | `_sprintf 0x172120` | `?` | `0x115b50` | `0x8000` | `ret(_remStartConnectCommand)` | `0x4` | `?` |
| `call` at 0x1164f7 | `_sprintf 0x172120` | `?` | `0x11620f` | `0x8000` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x116249 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x11625e | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) == 0x0` | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) != 0x0` |
| `je` at 0x11626b | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ef244] + [sp+12])+4]:1 & 0xff) & ([([0x1ef244] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x116280 | `ret(_cmdProcessFFSCCommand) != 0x2` | `ret(_cmdProcessFFSCCommand) == 0x2` |
| `je` at 0x1162cd | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x1162d5 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x116300 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x116326 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x116335 | `[0x1ef244] == [sp-8]` | `[0x1ef244] != [sp-8]` |
| `je` at 0x116357 | `(([0x1db370]:1 & 0xff) & 0x20) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x11637d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x1163cc | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x116419 | `[0x1ef244] == [sp-8]` | `[0x1ef244] != [sp-8]` |
| `jne` at 0x116425 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116472 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116514 | `eax` | `?` |
