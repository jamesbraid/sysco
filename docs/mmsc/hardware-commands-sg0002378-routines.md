# MMSC hardware commands in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans end at the next text symbol or object boundary. These ends are inferred
and exclusive, and the spans can include literals or padding after returns.
Statements below concern sites reached by known control-flow edges.
The computed jump in `_cmdCOM` remains unresolved; its selector-arm effects are excluded.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/hardware-commands-sg0002378-facts.txt) include all tool sections.

## `_cmdPWR`

- `_cmdPWR`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118210 to 0x118430.
  Established. The `_msgWrite` call has leading candidates `[[sp+8]+136]` and `[0x1dc780]` (`call`
  at 0x1182f1). The eight-byte `fstp` store has unknown value at `sp -0x11c` (`fstp` at 0x1182bc).
  The `_nanosleep` site has leading candidates `sp-256` and `0x0` (`call` at 0x1183c0). The local
  `_routeProcessELSCCommand` site presents `sp-248`, `[sp+8]`, and `[sp+12]` (`call` at 0x1183d6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11821c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11821f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11825c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118295 | load | 4 | `[sp+8] +0x10` | `-` |
| `imul` at 0x1182a4 | load | 4 | `abs 0x1dc7c8` | `-` |
| `fld` at 0x1182ac | load | 8 | `abs 0x118208` | `-` |
| `fidivr` at 0x1182b3 | load | 4 | `sp -0x118` | `-` |
| `fstp` at 0x1182bc | store | 8 | `sp -0x11c` | `?` |
| `mov` at 0x1182c5 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x1182cb | load | 4 | `[[sp+8]+136] +0x8` | `-` |
| `mov` at 0x1182d7 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x1182dd | store | 4 | `[[sp+8]+136] +0x1c` | `0x1` |
| `mov` at 0x1182e4 | load | 4 | `abs 0x1dc780` | `-` |
| `mov` at 0x1182ea | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x1182fd | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x118303 | load | 4 | `[[sp+8]+136] +0x1c` | `-` |
| `mov` at 0x11831d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118322 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118329 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118340 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x11834f | load | 4 | `abs 0x1dc7c8` | `-` |
| `mov` at 0x118368 | store | 4 | `sp -0x100` | `?` |
| `mov` at 0x118395 | store | 4 | `sp -0xfc` | `?` |
| `mov` at 0x1183a8 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1183ab | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1183c8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1183d0 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1183e0 | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11822a | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x11823a | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x11818d` | `?` | `?` | `?` | `?` |
| `call` at 0x11824e | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x11818f` | `?` | `?` | `?` | `?` |
| `call` at 0x118260 | `_destNumRacks 0x11f3ac` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x11828d | `_sprintf 0x172dd8` | `sp-248` | `0x118191` | `ret(_strtok_r)` | `?` | `?` | `?` |
| `call` at 0x1182cf | `_sprintf 0x172dd8` | `[[[sp+8]+136]+8]` | `0x118198` | `?` | `?` | `?` | `?` |
| `call` at 0x1182f1 | `_msgWrite 0x1269c8` | `[[sp+8]+136]` | `[0x1dc780]` | `?` | `?` | `?` | `?` |
| `call` at 0x118310 | `_ffscMsg 0x11f040` | `0x1181d6` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11832d | `_sprintf 0x172dd8` | `?` | `0x1181fc` | `?` | `?` | `?` | `?` |
| `call` at 0x118347 | `_msgClear 0x126058` | `[[sp+8]+136]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1183c0 | `_nanosleep 0x17b920` | `sp-256` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1183d6 | `_routeProcessELSCCommand 0x129a2c` | `sp-248` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x1183e7 | `_remStartRemoteCommand 0x1290b0` | `[sp+8]` | `sp-248` | `?` | `[sp+12]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x118246 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x118258 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jg` at 0x11826d | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` |
| `je` at 0x11829b | `[[sp+8]+16] == 0x3` | `[[sp+8]+16] != 0x3` |
| `jne` at 0x1182fb | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` |
| `je` at 0x118309 | `[[[sp+8]+136]+28] == 0xb` | `[[[sp+8]+136]+28] != 0xb` |
| `je` at 0x1183b1 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x1183b5 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1183ce | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x1183f8 | `? <= 0xa (unsigned, 32-bit)` | `? > 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11840b | `eax` | `?` |

## `_cmdRACKID`

- `_cmdRACKID`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1136f8 to 0x1138b8.
  Established. The `_strtoul` site has leading candidates `ret(_strtok_r)`, `sp-48`, and `0x0`
  (`call` at 0x1137ff); its result is stored as four bytes at `sp -0x14` (`mov` at 0x113807). The
  later unsigned branch is taken when `ret(_strtoul) <= 0xa (unsigned, 32-bit)` (`jbe` at 0x113817).
  The `_addrdRequestRackID` site has candidate zero `sp-32` (`call` at 0x11383c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113701 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113704 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x113707 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113720 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113726 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x113740 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11374b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113752 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11375b | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113760 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113767 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x113778 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1137c8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1137cd | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `rep movsd` at 0x1137f6 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1137f6 | load | 4 | `abs 0x1f0bf8` | `-` |
| `mov` at 0x113807 | store | 4 | `sp -0x14` | `ret(_strtoul)` |
| `mov` at 0x11380a | load | 4 | `sp -0x30` | `-` |
| `mov` at 0x11380d | load | 1 | `[sp-48] +0x0` | `-` |
| `cmp` at 0x113813 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x11385a | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11385f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113866 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113878 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113883 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11388a | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x113712 | `_strtok_r 0x180708` | `0x0` | `0x1129da` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x113737 | `_ffscRackIDToString 0x125b08` | `[0x1f0c04]` | `sp-44` | `?` | `?` | `?` | `?` |
| `call` at 0x113756 | `_sprintf 0x172dd8` | `?` | `0x11363d` | `[0x1f0c04]` | `sp-44` | `?` | `?` |
| `call` at 0x11379b | `_destNumRacks 0x11f3ac` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1137ae | `_ffscMsg 0x11f040` | `0x113648` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x1137ff | `_strtoul 0x17ff4c` | `ret(_strtok_r)` | `sp-48` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11381f | `_ffscMsg 0x11f040` | `0x113684` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x11383c | `_addrdRequestRackID 0x10db50` | `sp-32` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11384d | `_ffscMsg 0x11f040` | `0x113699` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11386a | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11388e | `_sprintf 0x172dd8` | `?` | `0x1136e2` | `[0x1f0c04]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x11371e | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x11372c | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jg` at 0x113780 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jle` at 0x1137a6 | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` |
| `jne` at 0x1137d3 | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `jne` at 0x113811 | `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) != 0x0` | `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x113817 | `ret(_strtoul) <= 0xa (unsigned, 32-bit)` | `ret(_strtoul) > 0xa (unsigned, 32-bit)` |
| `je` at 0x113846 | `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) == 0x0` | `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11389e | `eax` | `?` |

## `_cmdSCAN`

- `_cmdSCAN`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1138b8 to 0x113a88.
  Established. The `_elscQueryModuleNum` call has leading candidates `[sp-24]` and `sp-8` (`call` at
  0x113969); its nonzero-result taken path (`jne` at 0x113973) writes `0xffffffff` through an
  unknown effective address as four bytes (`mov` at 0x1139b8). The final `_elscUpdateModuleNums`
  site has unknown candidates (`call` at 0x1139fd). Record identities and indexed addresses remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1138c1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1138c4 | store | 4 | `sp -0x1c` | `0x0` |
| `mov` at 0x1138cb | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1138da | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1138df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1138e6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1138f8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1138fd | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113900 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11390c | store | 4 | `sp -0x18` | `0x0` |
| `mov` at 0x11391c | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113922 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113925 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11392e | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x113936 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113939 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `test` at 0x113945 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x113965 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x113975 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x113978 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11397e | store | 4 | `(unknown)` | `[sp-8]` |
| `mov` at 0x113982 | store | 4 | `sp -0x1c` | `0x1` |
| `mov` at 0x11398d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11399f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1139a3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1139b8 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1139c0 | store | 4 | `sp -0x1c` | `0x1` |
| `mov` at 0x1139cc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1139d0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1139dc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1139e0 | store | 4 | `(unknown)` | `0xb` |
| `inc` at 0x1139ea | load | 4 | `sp -0x18` | `-` |
| `inc` at 0x1139ea | store | 4 | `sp -0x18` | `?` |
| `cmp` at 0x1139ed | load | 4 | `sp -0x18` | `-` |
| `cmp` at 0x1139f7 | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1138ea | `_sprintf 0x172dd8` | `?` | `0x112a5e` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x113955 | `_elscSetup 0x121970` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113969 | `_elscQueryModuleNum 0x1213e0` | `[sp-24]` | `sp-8` | `?` | `?` | `?` | `?` |
| `call` at 0x113991 | `_ffscModuleNumToString 0x125788` | `[sp-8]` | `sp-20` | `?` | `?` | `?` | `?` |
| `call` at 0x1139a7 | `_sprintf 0x172dd8` | `?` | `0x1138a7` | `sp-20` | `?` | `?` | `?` |
| `call` at 0x1139d4 | `_sprintf 0x172dd8` | `?` | `0x11389f` | `?` | `?` | `?` | `?` |
| `call` at 0x1139fd | `_elscUpdateModuleNums 0x121ae0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1138d3 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x113906 | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jb` at 0x113934 | `C == 1` | `not (C == 1)` |
| `jge` at 0x11393f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11394c | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11395f | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x113973 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x11397c | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x1139f1 | `[sp-24] <= 0x1 (unsigned, 32-bit)` | `[sp-24] > 0x1 (unsigned, 32-bit)` |
| `je` at 0x1139fb | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113a10 | `eax` | `?` |

## `_cmdIPADDR`

- `_cmdIPADDR`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117fb0 to 0x118160.
  Established. The `_ffscParseIPAddr` site presents `ret(_strtok_r)` and `sp-44` as leading
  candidates (`call` at 0x1180b8). The subsequent branch is taken when `(ret(_ffscParseIPAddr) &
  ret(_ffscParseIPAddr)) == 0x0` (`je` at 0x1180c2). That path reaches `_addrdUpdateLocalIdentity`
  with candidate zero `sp-52` (`call` at 0x1180e1). The `rep movsd` store target and value remain
  unknown (`rep movsd` at 0x1180ae).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117fb9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x117fbc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x117fbf | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117fd8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117fdd | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x117fed | load | 4 | `abs 0x1f0c00` | `-` |
| `mov` at 0x117ffc | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118007 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11800e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118017 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11801c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118023 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x118030 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x118080 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118085 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `rep movsd` at 0x1180ae | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1180ae | load | 4 | `abs 0x1f0bf8` | `-` |
| `mov` at 0x1180ff | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118104 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11810b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118124 | load | 4 | `abs 0x1f0c00` | `-` |
| `mov` at 0x118138 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11813d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118144 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117fca | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x117ff3 | `_inet_ntoa_b 0x188c90` | `[0x1f0c00]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x118012 | `_sprintf 0x172dd8` | `?` | `0x117efa` | `[0x1f0c04]` | `sp-24` | `?` | `?` |
| `call` at 0x118053 | `_destNumRacks 0x11f3ac` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x118066 | `_ffscMsg 0x11f040` | `0x117f05` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x1180b8 | `_ffscParseIPAddr 0x125848` | `ret(_strtok_r)` | `sp-44` | `?` | `?` | `?` | `?` |
| `call` at 0x1180ca | `_ffscMsg 0x11f040` | `0x117f36` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1180e1 | `_addrdUpdateLocalIdentity 0x10dbf0` | `sp-52` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1180f2 | `_ffscMsg 0x11f040` | `0x117f4e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11810f | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11812a | `_inet_ntoa_b 0x188c90` | `[0x1f0c00]` | `sp-24` | `?` | `?` | `?` | `?` |
| `call` at 0x118148 | `_sprintf 0x172dd8` | `?` | `0x117fa4` | `sp-24` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x117fd6 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x117fe3 | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jg` at 0x118038 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jle` at 0x11805e | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` |
| `jne` at 0x11808b | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `je` at 0x1180c2 | `(ret(_ffscParseIPAddr) & ret(_ffscParseIPAddr)) == 0x0` | `(ret(_ffscParseIPAddr) & ret(_ffscParseIPAddr)) != 0x0` |
| `je` at 0x1180eb | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) == 0x0` | `(ret(_addrdUpdateLocalIdentity) & ret(_addrdUpdateLocalIdentity)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118158 | `eax` | `?` |

## `_cmdCOM`

- `_cmdCOM`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x112d18 to 0x113438.
  Established. The `_comGetInfo` call uses candidate zero `ret(_strtol)` (`call` at 0x112dc6).
  Two-byte loads through `ret(_comGetInfo) +0x6` (`mov` at 0x112dd6) and `ret(_comGetInfo) +0x4`
  (`mov` at 0x112de9) precede the formatting site. The `_ffscTokenToInt` call has leading candidates
  `0x1acc10`, `ret(_strtok_r)`, and `sp-32` (`call` at 0x112e92). Selector-arm effects behind the
  unresolved jump are excluded.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x112d21 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x112d24 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x112d2a | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x112d36 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x112d79 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x112d7c | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x112da8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x112dd6 | load | 2 | `ret(_comGetInfo) +0x6` | `-` |
| `mov` at 0x112de9 | load | 2 | `ret(_comGetInfo) +0x4` | `-` |
| `mov` at 0x112e11 | load | 2 | `ret(_comGetInfo) +0x4` | `-` |
| `mov` at 0x112e25 | load | 2 | `ret(_comGetInfo) +0x4` | `-` |
| `mov` at 0x112e3a | load | 4 | `ret(_comGetInfo) +0x0` | `-` |
| `mov` at 0x112e3e | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x112e49 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112e50 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112e68 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x112e6b | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x112eb8 | load | 4 | `sp -0x20` | `-` |
| `jmp` at 0x112ec5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1133a9 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x1133bf | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1133c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1133cb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1133d8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1133dd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1133e4 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x112d41 | `_strtok_r 0x180708` | `0x0` | `0x1129da` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x112d54 | `_ffscMsg 0x11f040` | `0x1129dd` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112d6f | `_strtol 0x17fda8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x112d90 | `_ffscMsg 0x11f040` | `0x1129fd` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x112db3 | `_strtok_r 0x180708` | `0x0` | `0x1129da` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x112dc6 | `_comGetInfo 0x12b1c0` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112de1 | `_ffscIntToToken 0x125708` | `0x1acc50` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112dfe | `_comGetTxBuf 0x12b200` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112e08 | `_comGetRxBuf 0x12b1e0` | `ret(_strtol)` | `ret(_comGetTxBuf)` | `?` | `?` | `?` | `?` |
| `call` at 0x112e54 | `_sprintf 0x172dd8` | `?` | `0x112a13` | `[0x1f0c04]` | `ret(_strtol)` | `[ret(_comGetInfo)]` | `sp-28` |
| `call` at 0x112e92 | `_ffscTokenToInt 0x125ca8` | `0x1acc10` | `ret(_strtok_r)` | `sp-32` | `?` | `?` | `?` |
| `call` at 0x112ea5 | `_ffscMsg 0x11f040` | `0x112a6e` | `ret(_strtok_r)` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x1133b2 | `_ffscMsg 0x11f040` | `0x112ce0` | `[sp-32]` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x1133cf | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x112d30 | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x112d4d | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x112d80 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x112d88 | `ret(_strtol)-1 <= 0x5 (unsigned, 32-bit)` | `ret(_strtol)-1 > 0x5 (unsigned, 32-bit)` |
| `jne` at 0x112dbf | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x112df5 | `(? & 0x8) == 0x0` | `(? & 0x8) != 0x0` |
| `je` at 0x112e1d | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |
| `je` at 0x112e31 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `jg` at 0x112e73 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x112e9c | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) == 0x0` | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) != 0x0` |
| `ja` at 0x112ebf | `? > 0x6 (unsigned, 32-bit)` | `? <= 0x6 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1133f9 | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x112ec5).

## `_cmdTTY_REINIT`

- `_cmdTTY_REINIT`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118590 to 0x118658.
  Established. The `_taskDelay` site has candidate zero `0x3c` (`call` at 0x1185f2). Calls to
  `_taskLock` (`call` at 0x1185fa), `_ttyReInit` (`call` at 0x1185ff), and `_taskUnlock` (`call` at
  0x118615) retain unknown candidate values. The result branch bypasses the `_ffscMsg` site when
  `(ret(_ttyReInit) & ret(_ttyReInit)) == 0x0` (`je` at 0x118606). The later four-byte store writes
  `0xb` through an unknown effective address (`mov` at 0x118641).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118593 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x118596 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x118599 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1185a8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1185ad | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1185b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1185d0 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1185d5 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11861a | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x118625 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11862c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118635 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11863a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118641 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1185b8 | `_sprintf 0x172dd8` | `?` | `0x116e80` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1185f2 | `_taskDelay 0x1722d8` | `0x3c` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x1185fa | `_taskLock 0x1721a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1185ff | `_ttyReInit 0x12baa0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11860d | `_ffscMsg 0x11f040` | `0x11855b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118615 | `_taskUnlock 0x1721d8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118630 | `_sprintf 0x172dd8` | `?` | `0x116ff9` | `[0x1f0c04]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1185a1 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jne` at 0x1185db | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` |
| `je` at 0x118606 | `(ret(_ttyReInit) & ret(_ttyReInit)) == 0x0` | `(ret(_ttyReInit) & ret(_ttyReInit)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1185c2 | `eax` | `0x0` |
| `ret` at 0x1185e5 | `eax` | `0x1` |
| `ret` at 0x118650 | `eax` | `0x1` |

## `_cmdDOWNLOADER`

- `_cmdDOWNLOADER`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117010 to 0x1172f0.
  Established. The `_enable_downloader` (`call` at 0x1170f3) and `_disable_downloader` (`call` at
  0x117100) sites have unknown candidate values. They occupy alternative arms of the branch taken
  when `Z == 1` (`je` at 0x1170f1): taken reaches `_disable_downloader`, while fall-through reaches
  `_enable_downloader`. After either call, a four-byte store writes `0xb` through an unknown effective address
  (`mov` at 0x11712c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117015 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x117018 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11701b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11701e | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1170c5 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1170ca | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1170d1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1170e2 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1170e7 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x117105 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117110 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117117 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117120 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117125 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11712c | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117048 | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `ebx` | `esi` | `ebp` |
| `call` at 0x11705b | `_ffscMsg 0x11f040` | `0x116f82` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117071 | `_ffscConvertToUpperCase 0x1255f8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11707f | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x116fc3` | `?` | `?` | `?` | `?` |
| `call` at 0x1170a6 | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x116fca` | `?` | `?` | `?` | `?` |
| `call` at 0x1170b8 | `_ffscMsg 0x11f040` | `0x116fd2` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1170d5 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1170f3 | `_enable_downloader 0x10cb98` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117100 | `_disable_downloader 0x10cbd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11711b | `_sprintf 0x172dd8` | `?` | `0x116ff9` | `[0x1f0c04]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x117026 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jne` at 0x117054 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117089 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x1170b0 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `je` at 0x1170ed | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `je` at 0x1170f1 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x117140 | `eax` | `?` |

## `_cmdFLASH`

- `_cmdFLASH`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1172f0 to 0x117950.
  Established. The `_userSendCommand` site has leading candidates `ret(_userAcquireRemote)` and
  `sp-252` (`call` at 0x117502); its zero-result taken path (`je` at 0x11750c) reaches a four-byte
  store of `0xffffffff` to `[[sp+8]+132] +0x14` (`mov` at 0x117526). A separate
  `_remStartConnectCommand` site presents `[sp+8]`, `0x11729d`, `[sp-8]`, and `[sp+12]` (`call` at
  0x11758b). The name alone does not establish flash writes in this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1172fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1172ff | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x117302 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x117309 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x117318 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11731d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117324 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117344 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117462 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11746e | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x117471 | load | 4 | `abs 0x1f0c04` | `-` |
| `test` at 0x1174b0 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x1174b9 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1174bf | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1174d2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1174d8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1174f0 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1174f5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1174fc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117520 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x117526 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x11752d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117532 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117539 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11753c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x117550 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x117553 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11755f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x117580 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117599 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11759c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1175a3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1175a9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1175b2 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1175b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1175be | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1175d6 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x1175dc | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x1175e8 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1175ed | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11760d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x117612 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117619 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117624 | store | 4 | `abs 0x1f1268` | `0x1` |
| `mov` at 0x117636 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x11763c | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x117640 | load | 4 | `[[sp+8]+148] +0x4c` | `-` |
| `mov` at 0x11764b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117652 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117662 | load | 4 | `[sp+8] +0x94` | `-` |
| `mov` at 0x117668 | load | 4 | `[[sp+8]+148] +0x50` | `-` |
| `mov` at 0x117673 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11767a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117683 | store | 4 | `abs 0x1f1268` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117328 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117348 | `_destNumRacks 0x11f3ac` | `[sp+12]` | `sp-8` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x11735b | `_ffscMsg 0x11f040` | `0x117141` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x117378 | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x117391 | `_ffscConvertToUpperCase 0x1255f8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11739f | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x117168` | `?` | `?` | `?` | `?` |
| `call` at 0x1173b1 | `_ffscMsg 0x11f040` | `0x117174` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1173d8 | `_strtok_r 0x180708` | `0x0` | `0x116e90` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x1173eb | `_ffscMsg 0x11f040` | `0x117192` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117401 | `_ffscConvertToUpperCase 0x1255f8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11740f | `_strcmp 0x180230` | `?` | `0x11716d` | `?` | `?` | `?` | `?` |
| `call` at 0x117436 | `_strcmp 0x180230` | `?` | `0x1171b3` | `?` | `?` | `?` | `?` |
| `call` at 0x117448 | `_ffscMsg 0x11f040` | `0x1171bb` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11747e | `_ffscMsg 0x11f040` | `0x1171e7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117491 | `_userAcquireRemote 0x12bbd4` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1174c8 | `_ffscMsg 0x11f040` | `0x11721b` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x1174e8 | `_sprintf 0x172dd8` | `sp-252` | `0x117247` | `[[[sp+8]+132]+20]` | `0x6` | `?` | `?` |
| `call` at 0x117502 | `_userSendCommand 0x12ca44` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x11750f | `_userReleaseRemote 0x12c994` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11756c | `_ffscMsg 0x11f040` | `0x117256` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11758b | `_remStartConnectCommand 0x128d00` | `[sp+8]` | `0x11729d` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1175c2 | `_sprintf 0x172dd8` | `?` | `0x1172a3` | `?` | `?` | `?` | `?` |
| `call` at 0x11761d | `_sprintf 0x172dd8` | `?` | `0x1172d0` | `0x10c0` | `ret(_remStartConnectCommand)` | `0x4` | `?` |
| `call` at 0x117656 | `_sprintf 0x172dd8` | `?` | `0x1172b4` | `?` | `?` | `[[[sp+8]+148]+80]` | `?` |
| `call` at 0x11767e | `_sprintf 0x172dd8` | `?` | `0x1172d0` | `0x40` | `[[[sp+8]+148]+80]` | `0x6` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x117311 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x117353 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x117384 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x1173a9 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x1173e4 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117419 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x117440 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x117468 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x117477 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `jne` at 0x11749d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x1174b7 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x11750c | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x117559 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `jne` at 0x117565 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x117597 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1175d2 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x117632 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11769b | `eax` | `0x0` |

## `_cmdELSC`

- `_cmdELSC`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116000 to 0x116460.
  Established. The `_destNumBays` call presents `[sp+12]`, `sp-8`, and `sp-12` as leading candidates
  (`call` at 0x116099). The `_userSendCommand` site has candidate zero `ret(_userAcquireRemote)` and
  candidate one `sp-256` (`call` at 0x1161e6); its zero-result taken path (`je` at 0x1161f0) reaches
  a four-byte store of `0xffffffff` to `[[sp+8]+132] +0x14` (`mov` at 0x116206).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11600c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11600f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x116012 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116015 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x116029 | load | 1 | `(ret(_strspn) + [[sp+4]]) +0x0` | `-` |
| `mov` at 0x116050 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x116057 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x11605e | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116066 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1160c0 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1160cc | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x1160cf | load | 4 | `abs 0x1f0c04` | `-` |
| `test` at 0x1160f0 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x1160f9 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1160ff | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x116110 | load | 4 | `sp -0xc` | `-` |
| `sub` at 0x116116 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x116121 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116128 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11613a | load | 4 | `sp -0xc` | `-` |
| `sub` at 0x116140 | load | 4 | `sp -0xc` | `-` |
| `movsx` at 0x116143 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x116149 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116154 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11615b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116164 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116169 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116170 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1161b2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1161b8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1161bc | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1161d4 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1161d9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1161e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116200 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116206 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x11620d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116212 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116219 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11621c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x116230 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x116233 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11623f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116259 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11625e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116265 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116294 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116297 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11629e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162ad | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1162b2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162dd | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1162e2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162f0 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x116302 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x116309 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x116315 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11631c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116325 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11632a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116331 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x11634d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116354 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11635f | store | 4 | `abs 0x1f1268` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116021 | `_strspn 0x180614` | `[[sp+4]]` | `0x115b40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x116033 | `_routeProcessELSCCommand 0x129a2c` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x116074 | `_ffscMsg 0x11f040` | `0x115e91` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x116099 | `_destNumBays 0x11f35c` | `[sp+12]` | `sp-8` | `sp-12` | `?` | `?` | `?` |
| `call` at 0x1160ac | `_ffscMsg 0x11f040` | `0x115eb1` | `ret(_destNumBays)` | `?` | `?` | `?` | `?` |
| `call` at 0x1160dc | `_ffscMsg 0x11f040` | `0x115ee3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116108 | `_ffscMsg 0x11f040` | `0x115f15` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x116132 | `_ffscMsg 0x11f040` | `0x115f41` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11615f | `_sprintf 0x172dd8` | `?` | `0x115f5f` | `[0x1f0c04]` | `?` | `?` | `?` |
| `call` at 0x116191 | `_userAcquireRemote 0x12bbd4` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1161cc | `_sprintf 0x172dd8` | `sp-256` | `0x115f72` | `[sp-12]` | `[[[sp+8]+132]+20]` | `0x3` | `?` |
| `call` at 0x1161e6 | `_userSendCommand 0x12ca44` | `ret(_userAcquireRemote)` | `sp-256` | `?` | `?` | `?` | `?` |
| `call` at 0x1161f3 | `_userReleaseRemote 0x12c994` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11624c | `_ffscMsg 0x11f040` | `0x115f87` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116269 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116288 | `_remStartConnectCommand 0x128d00` | `[sp+8]` | `0x115fd1` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1162bd | `_sprintf 0x172dd8` | `?` | `0x115fd6` | `?` | `?` | `?` | `?` |
| `call` at 0x116320 | `_sprintf 0x172dd8` | `?` | `0x115f5f` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x116358 | `_sprintf 0x172dd8` | `?` | `?` | `0x8000` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x116019 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x11602e | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) == 0x0` | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) != 0x0` |
| `je` at 0x116064 | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x11606c | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x1160a4 | `ret(_destNumBays) == 0x1` | `ret(_destNumBays) != 0x1` |
| `jne` at 0x1160c6 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x1160d5 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `je` at 0x1160f7 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x116126 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x11619d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x1161f0 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x116239 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `jne` at 0x116245 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116292 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x116307 | `? == 0x1` | `? != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116375 | `eax` | `?` |

## `_cmdOPTIONS`

- `_cmdOPTIONS`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113558 to 0x1136f8.
  Established. The `_strtol` call has leading candidates `ret(_strtok_r)`, `sp-8`, and `0x0` (`call`
  at 0x1135bf). A four-byte store places `ret(_strtol)` at `[sp+8] +0x1c` (`mov` at 0x113608). The
  following `_userUpdateSaveInfo` site has candidate zero `[sp+8]` (`call` at 0x11360c). The stored
  field meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113560 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113563 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x113580 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x11358f | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113594 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11359b | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1135c9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1135cf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1135d2 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x1135eb | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1135f0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1135f7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113608 | store | 4 | `[sp+8] +0x1c` | `ret(_strtol)` |
| `mov` at 0x113614 | load | 4 | `[sp+8] +0x1c` | `-` |
| `mov` at 0x11361d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113622 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113629 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11356e | `_strtok_r 0x180708` | `0x0` | `0x1129da` | `[sp+4]` | `ebx` | `esi` | `?` |
| `call` at 0x11359f | `_sprintf 0x172dd8` | `?` | `0x112a5e` | `?` | `?` | `?` | `?` |
| `call` at 0x1135bf | `_strtol 0x17fda8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1135de | `_ffscMsg 0x11f040` | `0x11351d` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1135fb | `_sprintf 0x172dd8` | `?` | `0x1129f3` | `?` | `?` | `?` | `?` |
| `call` at 0x11360c | `_userUpdateSaveInfo 0x12cb24` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11362d | `_sprintf 0x172dd8` | `?` | `0x11353f` | `[[sp+8]+28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11357a | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `jg` at 0x113588 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x1135cd | `[sp-8] == 0x0` | `[sp-8] != 0x0` |
| `je` at 0x1135d6 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11363c | `eax` | `0x0` |

## `_cmdNAP_TIME`

- `_cmdNAP_TIME`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113438 to 0x113558.
  Established. The `_strtoul` site has leading candidates `ret(_strtok_r)`, `0x0`, and `0x0` (`call`
  at 0x113498); its result branch is taken under `g flags condition (operands unknown)` (`jg` at
  0x1134a2). The two `_sprintf` sites retain unknown candidate zero (`call` at 0x1134e3, `call` at
  0x11350d). No delay operation is visible in the retained call list.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11343d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113440 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11345d | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1134d3 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1134d8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1134df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1134fd | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x113502 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113509 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11344b | `_strtok_r 0x180708` | `0x0` | `0x1129da` | `[sp+4]` | `ebx` | `esi` | `ebp` |
| `call` at 0x113479 | `_ffscConvertToUpperCase 0x1255f8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113487 | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x112c18` | `?` | `?` | `?` | `?` |
| `call` at 0x113498 | `_strtoul 0x17ff4c` | `ret(_strtok_r)` | `0x0` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1134aa | `_ffscMsg 0x11f040` | `0x1133fa` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x1134e3 | `_sprintf 0x172dd8` | `?` | `0x11340f` | `?` | `?` | `?` | `?` |
| `call` at 0x11350d | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x113457 | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `jg` at 0x113465 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x113491 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jg` at 0x1134a2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11351c | `eax` | `0x0` |

## `_cmdCONSOLE`

- `_cmdCONSOLE`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115bb0 to 0x115da0.
  Established. The `_userSendCommand` site has leading candidates `[[0x1dc204]+132]` and `sp-248`,
  with unknown candidate two (`call` at 0x115c63); its result branch is taken when
  `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` (`jne` at 0x115c6d). The four-byte load
  through `[0x1dc204] +0x18` has an unknown value (`mov` at 0x115c25).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x115bbb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x115bbe | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x115bc1 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x115be1 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x115be7 | load | 4 | `abs 0x1dc204` | `-` |
| `cmp` at 0x115c00 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x115c20 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x115c25 | load | 4 | `[0x1dc204] +0x18` | `-` |
| `mov` at 0x115c46 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x115c4b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c52 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c57 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x115c5c | load | 4 | `[0x1dc204] +0x84` | `-` |
| `mov` at 0x115c80 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115c88 | load | 4 | `[sp+8] +0x10` | `-` |
| `cmp` at 0x115cb0 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x115cb9 | load | 4 | `abs 0x1dc204` | `-` |
| `mov` at 0x115cbe | load | 4 | `[0x1dc204] +0x18` | `-` |
| `mov` at 0x115ccd | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x115cd2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115cd9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115cf5 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x115cfa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115d01 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x115bd1 | `_strspn 0x180614` | `[[sp+4]]` | `0x115b40` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x115c3e | `_sprintf 0x172dd8` | `sp-248` | `0x115b64` | `([[sp+4]] + ret(_strspn))` | `?` | `?` | `?` |
| `call` at 0x115c63 | `_userSendCommand 0x12ca44` | `[[0x1dc204]+132]` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x115c96 | `_ffscMsg 0x11f040` | `0x115b6e` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x115cdd | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115d05 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x115bc5 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x115bdb | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x115bed | `[0x1dc204] != [[sp+8]+148]` | `[0x1dc204] == [[sp+8]+148]` |
| `jne` at 0x115c07 | `[0x1dc204] != 0x0` | `[0x1dc204] == 0x0` |
| `jne` at 0x115c2b | `[[0x1dc204]+24] != 0x9` | `[[0x1dc204]+24] == 0x9` |
| `jne` at 0x115c6d | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` |
| `je` at 0x115c86 | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x115c8e | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x115cb7 | `[0x1dc204] == 0x0` | `[0x1dc204] != 0x0` |
| `jne` at 0x115cc4 | `[[0x1dc204]+24] != 0x9` | `[[0x1dc204]+24] == 0x9` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x115d17 | `eax` | `0x0` |

## `_cmdDIRECT`

- `_cmdDIRECT`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115da0 to 0x116000.
  Established. The `_userChangeInputMode` call has unknown candidate zero, candidate one `0x8`,
  and candidate two `0x0` (`call` at 0x115e05); its branch is taken when
  `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` (`je` at 0x115e0f).
  On that taken path, the later branch falls through when `not (Z == 0)`
  (`jne` at 0x115e2c), reaching the four-byte store of `0x1` to absolute `0x1f1268`
  (`mov` at 0x115e53). The inferred span ends with four undecoded bytes,
  `25 64 00 90`, at 0x115ffc to 0x116000 (exclusive).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x115da4 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x115da7 | load | 4 | `abs 0x1dcf94` | `-` |
| `mov` at 0x115daf | load | 4 | `abs 0x1dcf98` | `-` |
| `cmp` at 0x115dc0 | load | 4 | `abs 0x1dcf98` | `-` |
| `mov` at 0x115de0 | load | 4 | `abs 0x1dcf94` | `-` |
| `mov` at 0x115de6 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x115e20 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e26 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e30 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e3e | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x115e43 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e4a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e53 | store | 4 | `abs 0x1f1268` | `0x1` |
| `mov` at 0x115e73 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x115e78 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e7f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115e8a | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x115dce | `_ffscMsg 0x11f040` | `0x115d18` | `[sp+8]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x115e05 | `_userChangeInputMode 0x12bca4` | `?` | `0x8` | `0x0` | `ebx` | `ebp` | `?` |
| `call` at 0x115e4e | `_sprintf 0x172dd8` | `?` | `0x115d80` | `0x8000` | `?` | `0x7` | `?` |
| `call` at 0x115e66 | `_ffscMsg 0x11f040` | `0x115d5f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115e83 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x115dad | `[0x1dcf94] != [sp+8]` | `[0x1dcf94] == [sp+8]` |
| `je` at 0x115dc6 | `[0x1dcf98] == [sp+8]` | `[0x1dcf98] != [sp+8]` |
| `jg` at 0x115dee | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x115e0f | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `jne` at 0x115e2c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x115e90 | `eax` | `0x0` |

## `_cmdFFSC`

- `_cmdFFSC`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116460 to 0x116810.
  Established. The `_cmdProcessFFSCCommand` site has leading candidates `[[sp+4]]`, `[sp+8]`, and
  `[sp+12]` (`call` at 0x1164a5). The alternative connection path includes `_remStartConnectCommand`
  with candidates `[sp+8]`, `0x11643a`, `[sp-8]`, and `[sp+12]` (`call` at 0x116698). A four-byte
  store writes `0x1` to absolute `0x1f1268` (`mov` at 0x11672e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11646c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11646f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116472 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116475 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x116489 | load | 1 | `(ret(_strspn) + [[sp+4]]) +0x0` | `-` |
| `mov` at 0x116490 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116495 | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11649f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1164a2 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1164bb | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1164c0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164c7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164f0 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x1164f7 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1164ff | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116550 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11655c | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x11655f | load | 4 | `abs 0x1f0c04` | `-` |
| `test` at 0x116580 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x116589 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x11658f | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1165c2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1165c8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1165e0 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1165e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1165ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116610 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116616 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x11661d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116622 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116629 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11662c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x116640 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x116643 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11664f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116669 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11666e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116675 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166a4 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1166a7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166ae | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166bd | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1166c2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166c9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166ed | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1166f2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166f9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11671c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116723 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11672e | store | 4 | `abs 0x1f1268` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116481 | `_strspn 0x180614` | `[[sp+4]]` | `0x115b40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x1164a5 | `_cmdProcessFFSCCommand 0x111880` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x1164cb | `_sprintf 0x172dd8` | `?` | `0x116376` | `?` | `?` | `?` | `?` |
| `call` at 0x11650d | `_ffscMsg 0x11f040` | `0x116380` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x116525 | `_destNumRacks 0x11f3ac` | `[sp+12]` | `sp-8` | `?` | `?` | `?` | `?` |
| `call` at 0x116538 | `_ffscMsg 0x11f040` | `0x1163a1` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x11656c | `_ffscMsg 0x11f040` | `0x1163ca` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116598 | `_ffscMsg 0x11f040` | `0x1163fe` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x1165a1 | `_userAcquireRemote 0x12bbd4` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1165d8 | `_sprintf 0x172dd8` | `sp-252` | `0x11642b` | `[[[sp+8]+132]+20]` | `0x2` | `?` | `?` |
| `call` at 0x1165f2 | `_userSendCommand 0x12ca44` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x1165ff | `_userReleaseRemote 0x12c994` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11665c | `_ffscMsg 0x11f040` | `0x115f87` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116679 | `_sprintf 0x172dd8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116698 | `_remStartConnectCommand 0x128d00` | `[sp+8]` | `0x11643a` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1166cd | `_sprintf 0x172dd8` | `?` | `0x115fd6` | `?` | `?` | `?` | `?` |
| `call` at 0x1166fd | `_sprintf 0x172dd8` | `?` | `0x115d80` | `0x8000` | `ret(_remStartConnectCommand)` | `0x4` | `?` |
| `call` at 0x116727 | `_sprintf 0x172dd8` | `?` | `0x11643f` | `0x8000` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x116479 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x11648e | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) == 0x0` | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) != 0x0` |
| `je` at 0x11649b | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x1164b0 | `ret(_cmdProcessFFSCCommand) != 0x2` | `ret(_cmdProcessFFSCCommand) == 0x2` |
| `je` at 0x1164fd | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x116505 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x116530 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x116556 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x116565 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `je` at 0x116587 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x1165ad | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x1165fc | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x116649 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `jne` at 0x116655 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x1166a2 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116744 | `eax` | `?` |

## `_cmdRAT`

- `_cmdRAT`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116810 to 0x116b40.
  Established. The `_userSendCommand` call presents `ret(_userAcquireRemote)` and `sp-252` as
  leading candidates (`call` at 0x116982); its zero-result taken path (`je` at 0x11698c) reaches a
  four-byte store of `0xffffffff` to `[[sp+8]+132] +0x14` (`mov` at 0x1169a6). The
  `_remStartConnectCommand` site has leading candidates `[sp+8]`, `0x1167ec`, `[sp-8]`,
  and `[sp+12]` (`call` at 0x116a28). Four undecoded bytes, `65 00 90 90`, end the inferred
  span at 0x116b3c to 0x116b40 (exclusive).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11681c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11681f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116822 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x116829 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116831 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11684c | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116851 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116858 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116895 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x11689a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1168a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1168b0 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1168bc | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x1168bf | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1168d9 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1168de | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1168e5 | load | 4 | `(unknown)` | `-` |
| `test` at 0x116900 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x116909 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x11690f | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x116934 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116939 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116940 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116952 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116958 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x116970 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116975 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11697c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1169a0 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1169a6 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x1169ad | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1169b2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1169b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1169bc | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x1169d0 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x1169d3 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1169df | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1169f9 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1169fe | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a05 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a34 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116a37 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a3e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a44 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a4d | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116a52 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a59 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a78 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x116a7d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a84 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a97 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a9e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116aa7 | store | 4 | `abs 0x1f1268` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11683f | `_ffscMsg 0x11f040` | `0x116745` | `[sp+8]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x11685c | `_sprintf 0x172dd8` | `?` | `0x115b92` | `?` | `?` | `?` | `?` |
| `call` at 0x116875 | `_destNumRacks 0x11f3ac` | `[sp+12]` | `sp-8` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x116888 | `_ffscMsg 0x11f040` | `0x116765` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x1168a5 | `_sprintf 0x172dd8` | `?` | `0x115ed8` | `?` | `?` | `?` | `?` |
| `call` at 0x1168cc | `_ffscMsg 0x11f040` | `0x11678d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1168e9 | `_sprintf 0x172dd8` | `?` | `0x115ed8` | `?` | `?` | `?` | `?` |
| `call` at 0x116918 | `_ffscMsg 0x11f040` | `0x1167c0` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x116921 | `_userAcquireRemote 0x12bbd4` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116944 | `_sprintf 0x172dd8` | `?` | `0x115b4d` | `?` | `?` | `?` | `?` |
| `call` at 0x116968 | `_sprintf 0x172dd8` | `sp-252` | `0x11642b` | `[[[sp+8]+132]+20]` | `0xa` | `?` | `?` |
| `call` at 0x116982 | `_userSendCommand 0x12ca44` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x11698f | `_userReleaseRemote 0x12c994` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1169ec | `_ffscMsg 0x11f040` | `0x115f87` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116a09 | `_sprintf 0x172dd8` | `?` | `0x115fc1` | `?` | `?` | `?` | `?` |
| `call` at 0x116a28 | `_remStartConnectCommand 0x128d00` | `[sp+8]` | `0x1167ec` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x116a5d | `_sprintf 0x172dd8` | `?` | `0x115fd6` | `?` | `?` | `?` | `?` |
| `call` at 0x116a88 | `_sprintf 0x172dd8` | `?` | `0x1167f0` | `ret(_remStartConnectCommand)` | `0x4` | `?` | `?` |
| `call` at 0x116aa2 | `_sprintf 0x172dd8` | `?` | `0x115b9f` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11682f | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x116837 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x116880 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x1168b6 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x1168c5 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `je` at 0x116907 | `(([0x1dccc0]:1 & 0xff) & 0x20) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x11692d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x11698c | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x1169d9 | `[0x1f0c04] == [sp-8]` | `[0x1f0c04] != [sp-8]` |
| `jne` at 0x1169e5 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116a32 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116abf | `eax` | `0x0` |
