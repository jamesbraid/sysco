# MMSC ELSC client in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
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

[Fact sheets and full listings](disasm/elsc-client-6.5-facts.txt) include all tool sections.

## `_elscInit`

- `_elscInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122404 to 0x1226f4.
  Established. The `_nvramRead` candidates begin `0x5` and `sp-68` (`call` at 0x12242a); its branch
  takes `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` and falls through for nonzero (`je` at
  0x122434). The fall-through includes `_nvramWrite` with leading candidates `0x5` and `sp-68`
  (`call` at 0x12248b). Three later `_msgNew` sites have leading candidate values `0xa0` (`call` at
  0x1225a6), `0x50` (`call` at 0x1225da), and `0x80` (`call` at 0x122611). The saved-record
  interpretation and indexed controller-state addresses remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12240d | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x122414 | load | 4 | `abs 0x1f0adc` | `-` |
| `mov` at 0x122419 | store | 4 | `sp -0x4c` | `[0x1f0adc]` |
| `mov` at 0x12241c | load | 4 | `abs 0x1f0ae0` | `-` |
| `mov` at 0x122421 | store | 4 | `sp -0x48` | `[0x1f0ae0]` |
| `mov` at 0x122445 | store | 4 | `sp -0x7c` | `sp-68` |
| `mov` at 0x12244d | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x122453 | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x122456 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x122471 | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x122474 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x12249d | store | 4 | `sp -0x60` | `sp-68` |
| `mov` at 0x1224a3 | store | 4 | `sp -0x64` | `sp-84` |
| `mov` at 0x1224a8 | store | 4 | `sp -0x68` | `0x0` |
| `mov` at 0x1224af | store | 4 | `sp -0x6c` | `0x1df9b6` |
| `mov` at 0x1224b6 | store | 4 | `sp -0x70` | `0x0` |
| `mov` at 0x1224bd | store | 4 | `sp -0x74` | `0x1df9ac` |
| `mov` at 0x1224c4 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1224ce | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1224d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1224da | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1224de | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1224e2 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1224ea | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x1224ed | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x1224f0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1224f3 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1224f7 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1224fd | store | 1 | `(unknown)` | `?` |
| `mov` at 0x122501 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122508 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12250e | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x12251a | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x12251d | load | 4 | `sp -0x70` | `-` |
| `test` at 0x122520 | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x122527 | load | 4 | `([sp-112] + [sp-96]) +0x4` | `-` |
| `mov` at 0x12252b | store | 4 | `(unknown)` | `[([sp-112] + [sp-96])+4]` |
| `mov` at 0x122534 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x12253c | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x12253f | load | 4 | `sp -0x70` | `-` |
| `test` at 0x122542 | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x122550 | load | 4 | `sp -0x6c` | `-` |
| `mov` at 0x12255c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x122569 | load | 4 | `sp -0x6c` | `-` |
| `cmp` at 0x122575 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12257c | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x12258a | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x122596 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x12259c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1225ae | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x1225b6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1225c1 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1225cd | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1225d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1225e2 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x1225ea | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1225f5 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x122601 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x122607 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122619 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `mov` at 0x122658 | load | 4 | `sp -0x58` | `-` |
| `cmp` at 0x12265b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122661 | store | 4 | `(unknown)` | `[sp-88]` |
| `mov` at 0x122665 | store | 4 | `sp -0x5c` | `0x1` |
| `add` at 0x12266f | load | 4 | `sp -0x68` | `-` |
| `add` at 0x12266f | store | 4 | `sp -0x68` | `[sp-104]+16` |
| `add` at 0x122673 | load | 4 | `sp -0x6c` | `-` |
| `add` at 0x122673 | store | 4 | `sp -0x6c` | `[sp-108]+60` |
| `add` at 0x122677 | load | 4 | `sp -0x70` | `-` |
| `add` at 0x122677 | store | 4 | `sp -0x70` | `[sp-112]+32` |
| `add` at 0x12267b | load | 4 | `sp -0x74` | `-` |
| `add` at 0x12267b | store | 4 | `sp -0x74` | `[sp-116]+60` |
| `cmp` at 0x122689 | load | 4 | `sp -0x5c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12242a | `_nvramRead 0x128988` | `0x5` | `sp-68` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x122439 | `_bzero 0x174e40` | `sp-68` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x122469 | `_strncpy 0x182d28` | `?` | `0x1223e4` | `0x4` | `?` | `?` | `?` |
| `call` at 0x12248b | `_nvramWrite 0x128b18` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x122512 | `_bzero 0x174e40` | `[sp-116]` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x122554 | `_strncpy 0x182d28` | `[sp-108]` | `([sp-112] + [sp-96])+8` | `0x4` | `?` | `?` | `?` |
| `call` at 0x12256d | `_strcpy 0x182b54` | `[sp-108]` | `0x1223e4` | `?` | `?` | `?` | `?` |
| `call` at 0x12258e | `_sprintf 0x1755a0` | `[sp-100]` | `0x1223e9` | `?` | `?` | `?` | `?` |
| `call` at 0x1225a6 | `_msgNew 0x127ab8` | `0xa0` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x1225c5 | `_sprintf 0x1755a0` | `[sp-100]` | `0x1223f0` | `?` | `?` | `?` | `?` |
| `call` at 0x1225da | `_msgNew 0x127ab8` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x1225f9 | `_sprintf 0x1755a0` | `[sp-100]` | `0x1223f7` | `?` | `?` | `?` | `?` |
| `call` at 0x122611 | `_msgNew 0x127ab8` | `0x80` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x12263b | `_elscSetup 0x123324` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12264c | `_elscQueryModuleNum 0x122d84` | `?` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x12268f | `_elscUpdateModuleNums 0x123494` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122434 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x122483 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x122525 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x122547 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) != 0x0` |
| `jl` at 0x12257a | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x1225b4 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `je` at 0x1225e8 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `jne` at 0x12261f | `(ret(_msgNew) & ret(_msgNew)) != 0x0` | `(ret(_msgNew) & ret(_msgNew)) == 0x0` |
| `jne` at 0x122645 | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x122656 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x12265f | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x122683 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x12268d | `[sp-92] == 0x0` | `[sp-92] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1226a2 | `eax` | `?` |

## `_elscSetup`

- `_elscSetup`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123324 to 0x123424.
  Established. Calling `_elscDoAdminCommand` first presents leading candidates `[[sp+4]+12]`,
  `0x1232bf`, and `sp-136` (`call` at 0x12335f); its branch takes `(ret(_elscDoAdminCommand) &
  ret(_elscDoAdminCommand)) == 0x0` and falls through for nonzero (`je` at 0x123369). Later
  alternative four-byte stores write unknown values at `[sp+4] +0x0` (`mov` at 0x1233ae, `mov` at
  0x1233e0). Another route calls `_elscDoAdminCommand` with leading candidates `[[sp+4]+12]`,
  `0x123314`, and `0x0` (`call` at 0x1233d7). The command meanings and record flag update remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12332f | load | 4 | `sp +0x4` | `-` |
| `test` at 0x123332 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x12333b | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12335b | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x12336b | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x123399 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1233aa | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1233ae | store | 4 | `[sp+4] +0x0` | `?` |
| `mov` at 0x1233c4 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1233d3 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1233dc | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1233e0 | store | 4 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123347 | `_ffscMsg 0x120504` | `0x1232ab` | `?` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x12335f | `_elscDoAdminCommand 0x1221a4` | `[[sp+4]+12]` | `0x1232bf` | `sp-136` | `?` | `?` | `?` |
| `call` at 0x123377 | `_ffscMsg 0x120504` | `0x1232c5` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12338c | `_strncmp 0x179530` | `sp-136` | `0x122d30` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1233a5 | `_ffscMsg 0x120504` | `0x1232f0` | `?` | `sp-136` | `?` | `?` | `?` |
| `call` at 0x1233d7 | `_elscDoAdminCommand 0x1221a4` | `[[sp+4]+12]` | `0x123314` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123339 | `(([0x1df500]:1 & 0xff) & 0x2) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x123369 | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x123396 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x1233ca | `[[sp+4]+4] != 0x1` | `[[sp+4]+4] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1233ef | `eax` | `?` |

## `_elscCheckResponse`

- `_elscCheckResponse`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122004 to 0x1221a4.
  Established. The `_strchr` call at the loop test has leading candidates unknown and `0x14` (`call`
  at 0x122053); its branch takes `(ret(_strchr) & ret(_strchr)) != 0x0` and falls through for zero
  (`jne` at 0x12205f). The taken route includes a byte store of `0x0` at `ret(_strchr) +0x0` (`mov`
  at 0x122014). A later `_strchr` site uses leading candidates unknown and `0xd` (`call` at
  0x12201a). `_elscProcessCommand` presents leading candidate `[sp+4]` and unknown remaining
  candidates (`call` at 0x122045). The response grammar beyond these observed sites is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12200a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122014 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x122026 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `test` at 0x122029 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x122041 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12201a | `_strchr 0x1829d0` | `?` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x122038 | `_ffscMsg 0x120504` | `0x121fd4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122045 | `_elscProcessCommand 0x1228e4` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122053 | `_strchr 0x1829d0` | `?` | `0x14` | `?` | `?` | `?` | `?` |
| `call` at 0x122066 | `_strlen 0x176e34` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122071 | `_memmove 0x182958` | `[sp+8]` | `?` | `ret(_strlen)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122024 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x122030 | `(([0x1df500]:1 & 0xff) & 0x10) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x12205f | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x122063 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12207f | `eax` | `?` |

## `_elscSendCommand`

- `_elscSendCommand`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122f14 to 0x1230b4.
  Established. Four-byte stores on different routes write `0xb` (`mov` at 0x122f5f), `0x1` (`mov` at
  0x122f9b), and `0x5` (`mov` at 0x123038), each at an unknown effective target. The `_msgWrite`
  candidates begin unknown and `[0x1defb8]` (`call` at 0x122fca); its branch takes `(ret(_msgWrite)
  & ret(_msgWrite)) == 0x0` and falls through for nonzero (`je` at 0x122fd4). All known routes join
  at a return carrying EAX `0x0` (`ret` at 0x123071). The indexed state records and submission
  policy remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122f1d | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122f20 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x122f26 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122f35 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x122f42 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f4e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f52 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f5b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f5f | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x122f74 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f87 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f8b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f97 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122f9b | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x122fa2 | load | 1 | `abs 0x1df500` | `-` |
| `movsx` at 0x122fab | load | 1 | `(unknown)` | `-` |
| `mov` at 0x122fbf | load | 4 | `abs 0x1defb8` | `-` |
| `mov` at 0x122fc5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122fdb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122fdf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122ff4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122ff8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123000 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x123004 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12301c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123020 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123034 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123038 | store | 4 | `(unknown)` | `0x5` |
| `cmp` at 0x12303f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123059 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122f3a | `_msgClear 0x127a08` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x122f56 | `_sprintf 0x1755a0` | `?` | `0x122ea2` | `?` | `?` | `?` | `?` |
| `call` at 0x122f79 | `_msgClear 0x127a08` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122f8f | `_sprintf 0x1755a0` | `?` | `0x1220ab` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x122fb7 | `_ffscMsg 0x120504` | `0x122eae` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x122fca | `_msgWrite 0x1282d8` | `?` | `[0x1defb8]` | `?` | `?` | `?` | `?` |
| `call` at 0x122fe3 | `_strcpy 0x182b54` | `?` | `0x1220f7` | `?` | `?` | `?` | `?` |
| `call` at 0x12300f | `_ffscMsg 0x120504` | `0x122ed1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123024 | `_strcpy 0x182b54` | `?` | `0x1220f7` | `?` | `?` | `?` | `?` |
| `call` at 0x123050 | `_sprintf 0x1755a0` | `sp-124` | `0x122efd` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12305e | `_logWriteLine 0x126b5c` | `?` | `sp-124` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122f47 | `? == 0x1` | `? != 0x1` |
| `je` at 0x122fa9 | `(([0x1df500]:1 & 0xff) & 0x10) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x122fd4 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x122ffe | `? == 0xb` | `? != 0xb` |
| `je` at 0x123044 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123071 | `eax` | `0x0` |

## `_elscSendPassword`

- `_elscSendPassword`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1230b4 to 0x123174.
  Established. The signed branch takes `[sp+8] > 0x1f3 (signed, 32-bit)` and falls through for the
  complementary comparison (`jg` at 0x1230dc). The later `_elscDoAdminCommand` call has leading
  candidates `[sp+4]`, `sp-88`, and `sp-220` (`call` at 0x123117); its result branch takes
  `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` and falls through for zero (`jne`
  at 0x123121). The fall-through includes `_strncmp` with leading candidates `sp-220`, `0x122d30`,
  and `0x2` (`call` at 0x12312b). The threshold units and authentication protocol remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1230bf | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1230cc | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1230d5 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1230e7 | `_sprintf 0x1755a0` | `sp-88` | `0x123072` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x123103 | `_sprintf 0x1755a0` | `sp-88` | `0x123076` | `?` | `ebx` | `esi` | `?` |
| `call` at 0x123117 | `_elscDoAdminCommand 0x1221a4` | `[sp+4]` | `sp-88` | `sp-220` | `?` | `?` | `?` |
| `call` at 0x12312b | `_strncmp 0x179530` | `sp-220` | `0x122d30` | `0x2` | `?` | `?` | `?` |
| `call` at 0x12314a | `_ffscMsg 0x120504` | `0x12307d` | `sp-220` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1230d3 | `? == 0x0` | `? != 0x0` |
| `jg` at 0x1230dc | `[sp+8] > 0x1f3 (signed, 32-bit)` | `[sp+8] <= 0x1f3 (signed, 32-bit)` |
| `jne` at 0x123121 | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x123135 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12315f | `eax` | `?` |

## `_elscSetPassword`

- `_elscSetPassword`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123174 to 0x123324.
  Established. The first decision takes `([sp+12] & [sp+12]) == 0x0` and falls through for nonzero
  (`je` at 0x1231a4). A later route calls `_elscSendPassword` with leading candidates `[sp+4]` and
  `0x3e8` (`call` at 0x1231fd); its branch takes `(ret(_elscSendPassword) & ret(_elscSendPassword))
  != 0x0` and falls through for zero (`jne` at 0x123207). The merged `_strncpy` candidates begin
  unknown, `[sp+8]`, and `0x4` (`call` at 0x123270). The subsequent `_elscUpdateSaveInfo` candidates
  are all unknown (`call` at 0x123278). The password-storage address and update policy remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12317d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x123180 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x123186 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123195 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1231aa | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1231b4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1231b7 | load | 1 | `[sp+8] +0x0` | `-` |
| `cmp` at 0x1231c1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1231cd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1231d1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1231da | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1231de | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1231f9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123209 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123221 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123236 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12323a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12324a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123259 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12325d | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x123263 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123287 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12328b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123294 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123298 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12319a | `_msgClear 0x127a08` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1231d5 | `_sprintf 0x1755a0` | `?` | `0x122ea2` | `?` | `?` | `?` | `?` |
| `call` at 0x1231fd | `_elscSendPassword 0x1230b4` | `[sp+4]` | `0x3e8` | `?` | `?` | `?` | `?` |
| `call` at 0x123216 | `_sprintf 0x1755a0` | `sp-88` | `0x123160` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x123225 | `_elscDoAdminCommand 0x1221a4` | `[sp+4]` | `sp-88` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12323e | `_strcmp 0x1829f8` | `?` | `0x122d30` | `?` | `?` | `?` | `?` |
| `call` at 0x12324f | `_msgClear 0x127a08` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123270 | `_strncpy 0x182d28` | `?` | `[sp+8]` | `0x4` | `?` | `?` | `?` |
| `call` at 0x123278 | `_elscUpdateSaveInfo 0x123514` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12328f | `_sprintf 0x1755a0` | `?` | `0x12316b` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1231a4 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x1231ae | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `je` at 0x1231bb | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) == 0x0` | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) != 0x0` |
| `je` at 0x1231c6 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x123207 | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` |
| `jne` at 0x12322f | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x123248 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1232aa | `eax` | `?` |

## `_elscProcessInput`

- `_elscProcessInput`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122b84 to 0x122d84.
  Established. The initial branch takes `[[sp+4]+4] == 0x1` and falls through for inequality (`je`
  at 0x122b95); its taken route stores four-byte `0x4` at `[[sp+4]+48] +0x1c` (`mov` at 0x122bc3).
  `_msgRead` presents leading candidates `[[sp+4]+48]` and `[0x1defa0]` (`call` at 0x122bd4). A
  later route stores byte `0x0` at `[[[sp+4]+48]+12] +0x0` (`mov` at 0x122c74). The input-state
  values and termination policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122b8c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122b8f | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x122b97 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x122bb4 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122bc0 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122bc3 | store | 4 | `[[sp+4]+48] +0x1c` | `0x4` |
| `mov` at 0x122bca | load | 4 | `abs 0x1defa0` | `-` |
| `mov` at 0x122bd0 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122bf4 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122bf7 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x122bff | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122c03 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122c14 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x122c1c | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x122c22 | store | 4 | `[[sp+4]+48] +0xc` | `[[[sp+4]+48]+12]-2` |
| `mov` at 0x122c34 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x122c3c | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x122c3f | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122c46 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122c4d | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x122c64 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x122c68 | store | 4 | `[[sp+4]+48] +0xc` | `?` |
| `mov` at 0x122c6b | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x122c6e | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122c71 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x122c74 | store | 1 | `[[[sp+4]+48]+12] +0x0` | `0x0` |
| `mov` at 0x122c77 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122c7a | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122c7d | load | 1 | `[[[sp+4]+48]+8] +0x0` | `-` |
| `mov` at 0x122c83 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x122c86 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122c89 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122c90 | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x122ca7 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x122caa | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122cc4 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x122ccc | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122ce4 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x122ce8 | load | 4 | `[sp+4] +0x30` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122ba0 | `_ffscMsg 0x120504` | `0x122af3` | `[[sp+4]+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x122bb8 | `_msgClear 0x127a08` | `[[sp+4]+48]` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x122bd4 | `_msgRead 0x127cc8` | `[[sp+4]+48]` | `[0x1defa0]` | `?` | `?` | `?` | `?` |
| `call` at 0x122c9f | `_sprintf 0x1755a0` | `sp-16` | `0x122b50` | `[0x1f3454]` | `?` | `?` | `?` |
| `call` at 0x122cb0 | `_elscPrintMsg 0x1226f4` | `[sp+4]` | `sp-16` | `[[[sp+4]+48]+8]` | `?` | `?` | `?` |
| `call` at 0x122cd2 | `_elscProcessCommand 0x1228e4` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122cf1 | `_ffscMsg 0x120504` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122b95 | `[[sp+4]+4] == 0x1` | `[[sp+4]+4] != 0x1` |
| `je` at 0x122bde | `(ret(_msgRead) & ret(_msgRead)) == 0x0` | `(ret(_msgRead) & ret(_msgRead)) != 0x0` |
| `jne` at 0x122bfd | `[[[sp+4]+48]+28] != 0xa` | `[[[sp+4]+48]+28] == 0xa` |
| `jne` at 0x122c1a | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |
| `je` at 0x122c3a | `[[[sp+4]+48]+28] == 0x17` | `[[[sp+4]+48]+28] != 0x17` |
| `jne` at 0x122c44 | `[[[sp+4]+48]+12] != [[[sp+4]+48]+8]` | `[[[sp+4]+48]+12] == [[[sp+4]+48]+8]` |
| `je` at 0x122c81 | `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14` | `([[[[sp+4]+48]+8]]:1 & 0xff) != 0x14` |
| `jne` at 0x122cca | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122d00 | `eax` | `?` |

## `_elscProcessCommand`

- `_elscProcessCommand`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1228e4 to 0x122b84.
  Established. A four-byte store initializes `sp -0x8` to `0x0` (`mov` at 0x122914). `_strtok_r`
  then has leading candidates `[sp+8]`, `0x122838`, and `sp-8` (`call` at 0x122925). The branch
  takes `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` and falls through for zero (`jne` at 0x122931).
  On a later route, a four-byte store writes `[sp-92]` at `[sp+4] +0x10` (`mov` at 0x122a50),
  followed by `_elscUpdateModuleNums` with unknown candidates (`call` at 0x122a53). The
  token-command catalogue remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1228f0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1228f3 | load | 4 | `sp +0x8` | `-` |
| `test` at 0x1228f6 | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x122900 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122914 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x122933 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122981 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122995 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1229a2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1229ec | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1229f3 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122a0c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x122a2d | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x122a41 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x122a44 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x122a4d | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x122a50 | store | 4 | `[sp+4] +0x10` | `[sp-92]` |
| `mov` at 0x122a76 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122a7d | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x122a94 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x122aa5 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x122ac0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x122ac8 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12290c | `_ffscMsg 0x120504` | `0x12281e` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` |
| `call` at 0x122925 | `_strtok_r 0x182ed0` | `[sp+8]` | `0x122838` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x12293f | `_ffscMsg 0x120504` | `0x12283b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122955 | `_ffscConvertToUpperCase 0x126fa8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122963 | `_strcmp 0x1829f8` | `ret(_strtok_r)` | `0x122855` | `?` | `?` | `?` | `?` |
| `call` at 0x122975 | `_strcmp 0x1829f8` | `ret(_strtok_r)` | `0x122859` | `?` | `?` | `?` | `?` |
| `call` at 0x12298d | `_ffscMsg 0x120504` | `0x12285c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12299a | `_elscUpdateDisplay 0x123424` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1229ab | `_update_partition_info 0x12eea0` | `0x1af958` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1229cc | `_strncmp 0x179530` | `ret(_strtok_r)` | `0x122888` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1229e0 | `_strncmp 0x179530` | `ret(_strtok_r)` | `0x12288d` | `0x3` | `?` | `?` | `?` |
| `call` at 0x122a03 | `_sprintf 0x1755a0` | `sp-88` | `0x122891` | `?` | `?` | `?` | `?` |
| `call` at 0x122a10 | `_logWriteLine 0x126b5c` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x122a19 | `_elscSetup 0x123324` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122a31 | `_elscQueryModuleNum 0x122d84` | `[[sp+4]+12]` | `sp-92` | `?` | `?` | `?` | `?` |
| `call` at 0x122a53 | `_elscUpdateModuleNums 0x123494` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122a6a | `_strcmp 0x1829f8` | `ret(_strtok_r)` | `0x1228aa` | `?` | `?` | `?` | `?` |
| `call` at 0x122a8c | `_sprintf 0x1755a0` | `sp-88` | `0x1228ae` | `[0x1f3454]` | `?` | `?` | `?` |
| `call` at 0x122a9a | `_elscPrintMsg 0x1226f4` | `[sp+4]` | `sp-88` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x122ab8 | `_sprintf 0x1755a0` | `sp-348` | `0x1228b5` | `?` | `[sp+8]` | `?` | `?` |
| `call` at 0x122acc | `_logWriteLine 0x126b5c` | `[[sp+4]+44]` | `sp-348` | `?` | `?` | `?` | `?` |
| `call` at 0x122ad5 | `_ffscMsg 0x120504` | `sp-348` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1228fd | `(([0x1df500]:1 & 0xff) & 0x10) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x122931 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x12296d | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x12297f | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x1229d6 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x1229ea | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x1229f1 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x122a23 | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x122a3b | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x122a47 | `[sp-92] == [[sp+4]+16]` | `[sp-92] != [[sp+4]+16]` |
| `jne` at 0x122a74 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x122ac5 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122af2 | `eax` | `?` |

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1221a4 to 0x122404.
  Established. The entry comparison branch takes `? >= 0x0 (signed, 32-bit)` and falls through for
  negative values (`jge` at 0x1221ca); its taken route includes `_msgWrite` with leading candidates
  unknown and `[0x1defb8]` (`call` at 0x122276). A later `_msgRead` site instead begins unknown and
  `[0x1defa4]` (`call` at 0x122307). The branch after `_strpbrk` takes `(ret(_strpbrk) &
  ret(_strpbrk)) == 0x0` and falls through for nonzero (`je` at 0x122357); its fall-through stores
  byte `0x0` at `ret(_strpbrk) +0x0` (`mov` at 0x122359). The administrative-command framing and
  timeout units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1221ad | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1221b0 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1221b6 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1221c5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1221e4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1221f7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1221fb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122207 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12220b | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x122212 | load | 1 | `abs 0x1df500` | `-` |
| `movsx` at 0x12221b | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x12222f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122249 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122256 | load | 4 | `abs 0x1defb8` | `-` |
| `mov` at 0x12225c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x122262 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122271 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x122282 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x122291 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1222a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1222a8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1222b0 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1222b4 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1222c7 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1222d6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1222e4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1222f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1222f5 | store | 4 | `(unknown)` | `0x5` |
| `mov` at 0x1222fc | load | 4 | `abs 0x1defa4` | `-` |
| `mov` at 0x122302 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122317 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12231b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122323 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122349 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122359 | store | 1 | `ret(_strpbrk) +0x0` | `0x0` |
| `test` at 0x12235c | load | 1 | `abs 0x1df500` | `-` |
| `mov` at 0x122365 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122369 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12236d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122370 | load | 1 | `[sp+4] +0x1af888` | `-` |
| `cmp` at 0x122387 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12238e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x122392 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1223a8 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1223b5 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1223bb | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1223c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1223c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1223cb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1223cf | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1221d1 | `_ffscMsg 0x120504` | `0x122080` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1221e9 | `_msgClear 0x127a08` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1221ff | `_sprintf 0x1755a0` | `?` | `0x1220ab` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x122227 | `_ffscMsg 0x120504` | `0x1220b0` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x122240 | `_sprintf 0x1755a0` | `sp-84` | `0x1220e1` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12224e | `_logWriteLine 0x126b5c` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x122276 | `_msgWrite 0x1282d8` | `?` | `[0x1defb8]` | `?` | `?` | `?` | `?` |
| `call` at 0x122295 | `_sprintf 0x1755a0` | `[sp+12]` | `0x1220f7` | `?` | `?` | `?` | `?` |
| `call` at 0x1222bf | `_ffscMsg 0x120504` | `0x122102` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1222da | `_sprintf 0x1755a0` | `[sp+12]` | `0x1220f7` | `?` | `?` | `?` | `?` |
| `call` at 0x1222e9 | `_msgClear 0x127a08` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122307 | `_msgRead 0x127cc8` | `?` | `[0x1defa4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12232c | `_ffscMsg 0x120504` | `0x122134` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12234d | `_strpbrk 0x182d6c` | `?` | `0x12215c` | `?` | `?` | `?` | `?` |
| `call` at 0x12237f | `_ffscMsg 0x120504` | `0x12215f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12239f | `_sprintf 0x1755a0` | `sp-84` | `0x122189` | `?` | `?` | `?` | `?` |
| `call` at 0x1223ad | `_logWriteLine 0x126b5c` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x1223d3 | `_strcpy 0x182b54` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1221ca | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `je` at 0x122219 | `(([0x1df500]:1 & 0xff) & 0x10) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x122234 | `? == 0x0` | `? != 0x0` |
| `je` at 0x122280 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x122286 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `je` at 0x1222ae | `? == 0xb` | `? != 0xb` |
| `je` at 0x1222cb | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jne` at 0x122311 | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x122321 | `? == 0xb` | `? != 0xb` |
| `je` at 0x122357 | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` |
| `je` at 0x122363 | `(([0x1df500]:1 & 0xff) & 0x10) == 0x0` | `(([0x1df500]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x12238c | `? == 0x0` | `? != 0x0` |
| `je` at 0x1223b9 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1223e3 | `eax` | `?` |

## `_elscPrintMsg`

- `_elscPrintMsg`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1226f4 to 0x1228e4.
  Established. The comparison branch takes `[0x1f3454] != [[sp+4]+20]` and falls through for
  equality (`jne` at 0x122723). The fall-through includes `_userSendCommand` with leading candidates
  `[0x1df7dc]`, `sp-248`, and `0x0` (`call` at 0x122747). A later alternative route calls
  `_routeDoAdminCommand` with leading candidate `sp-492` (`call` at 0x12279a). Another
  `_userSendCommand` site begins `[0x1df7e0]`, `sp-736`, and `0x0` (`call` at 0x1227cf). The
  destination-selection and message-format meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122700 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122703 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x122706 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x12270e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x12271a | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x12271d | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x122726 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122741 | load | 4 | `abs 0x1df7dc` | `-` |
| `mov` at 0x122774 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x12277d | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122781 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1227a2 | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x1227a5 | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x1227ae | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1227c9 | load | 4 | `abs 0x1df7e0` | `-` |
| `mov` at 0x1227e4 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x1227ed | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1227f1 | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122712 | `_logWriteLine 0x126b5c` | `[[sp+4]+44]` | `[sp+12]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x122736 | `_sprintf 0x1755a0` | `sp-248` | `0x1226a3` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x122747 | `_userSendCommand 0x12e418` | `[0x1df7dc]` | `sp-248` | `0x0` | `?` | `?` | `?` |
| `call` at 0x122759 | `_ffscMsg 0x120504` | `0x1226ad` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x122791 | `_sprintf 0x1755a0` | `sp-492` | `0x1226b7` | `[[sp+4]+20]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x12279a | `_routeDoAdminCommand 0x12b234` | `sp-492` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1227be | `_sprintf 0x1755a0` | `sp-736` | `0x1226a3` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1227cf | `_userSendCommand 0x12e418` | `[0x1df7e0]` | `sp-736` | `0x0` | `?` | `?` | `?` |
| `call` at 0x122801 | `_sprintf 0x1755a0` | `sp-980` | `0x1226cd` | `[[sp+4]+24]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x12280a | `_routeDoAdminCommand 0x12b234` | `sp-980` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12270b | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x122723 | `[0x1f3454] != [[sp+4]+20]` | `[0x1f3454] == [[sp+4]+20]` |
| `je` at 0x122751 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x12277a | `[[sp+4]+20] == 0xffffffff` | `[[sp+4]+20] != 0xffffffff` |
| `jne` at 0x1227ab | `[0x1f3454] != [[sp+4]+24]` | `[0x1f3454] == [[sp+4]+24]` |
| `je` at 0x1227d6 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x1227ea | `[[sp+4]+24] == 0xffffffff` | `[[sp+4]+24] != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12281d | `eax` | `?` |

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x122d84 to 0x122f14.
  Established. The entry branch takes `([sp+8] & [sp+8]) == 0x0` and falls through for nonzero (`je`
  at 0x122d98). The fall-through stores four-byte `0xffffffff` at `[sp+8] +0x0` (`mov` at 0x122d9a).
  A later `_strtol` call presents leading candidates `sp-133`, `sp-140`, and `0x10` (`call` at
  0x122e44); its unsigned range branch takes `ret(_strtol) <= 0xff (unsigned, 32-bit)` and falls
  through for larger values (`jbe` at 0x122e60). The later four-byte result store at `[sp+8] +0x0`
  has unknown value (`mov` at 0x122e91). The response-to-module interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x122d90 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x122d93 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x122d9a | store | 4 | `[sp+8] +0x0` | `0xffffffff` |
| `movsx` at 0x122dc3 | load | 1 | `(unknown)` | `-` |
| `or` at 0x122dd5 | load | 1 | `(unknown)` | `-` |
| `or` at 0x122dd5 | store | 1 | `(unknown)` | `?` |
| `movsx` at 0x122e10 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x122e4e | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x122e54 | load | 1 | `[sp-140] +0x0` | `-` |
| `movsx` at 0x122e6a | load | 1 | `(unknown)` | `-` |
| `mov` at 0x122e91 | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x122dad | `_elscDoAdminCommand 0x1221a4` | `[sp+4]` | `0x122d01` | `sp-136` | `?` | `?` | `?` |
| `call` at 0x122dd0 | `_ffscMsg 0x120504` | `0x122d05` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x122dfc | `_strncmp 0x179530` | `sp-136` | `0x122d30` | `0x2` | `?` | `?` | `?` |
| `call` at 0x122e1e | `_ffscMsg 0x120504` | `0x122d33` | `?` | `sp-136` | `?` | `?` | `?` |
| `call` at 0x122e44 | `_strtol 0x182570` | `sp-133` | `sp-140` | `0x10` | `?` | `?` | `?` |
| `call` at 0x122e78 | `_ffscMsg 0x120504` | `0x122d55` | `?` | `sp-133` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x122d98 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x122db7 | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x122e06 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x122e58 | `(([[sp-140]]:1 & 0xff) & ([[sp-140]]:1 & 0xff)) != 0x0` | `(([[sp-140]]:1 & 0xff) & ([[sp-140]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x122e60 | `ret(_strtol) <= 0xff (unsigned, 32-bit)` | `ret(_strtol) > 0xff (unsigned, 32-bit)` |
| `jne` at 0x122e86 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x122e8f | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x122ea1 | `eax` | `?` |

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123494 to 0x123514.
  Established. The repeated move site has a four-byte load at absolute `0x1f3448` and a four-byte
  store with unknown effective target and value (`rep movsd` at 0x1234aa). A later indexed four-byte
  store also has unknown target and value (`mov` at 0x1234bc). The loop branch takes `? <= 0x1
  (unsigned, 32-bit)` and falls through for larger values (`jbe` at 0x1234c7). Calling
  `_addrdUpdateLocalIdentity` presents leading candidate `sp-32` (`call` at 0x1234cd), and the
  return carries `ret(_addrdUpdateLocalIdentity)` in EAX (`ret` at 0x1234da). The copied record
  semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x1234aa | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1234aa | load | 4 | `abs 0x1f3448` | `-` |
| `mov` at 0x1234b8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1234bc | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1234cd | `_addrdUpdateLocalIdentity 0x10ddc0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1234c7 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1234da | `eax` | `ret(_addrdUpdateLocalIdentity)` |

## `_elscUpdateDisplay`

- `_elscUpdateDisplay`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123424 to 0x123494.
  Established. The `_bzero` candidates begin `[sp+4]+28` and `0xa` (`call` at 0x123439). The branch
  takes `([sp+8] & [sp+8]) == 0x0` and falls through for nonzero (`je` at 0x123443); its
  fall-through calls `_strncpy` with leading candidates `[sp+4]+28`, `[sp+8]`, and `0xa` (`call` at
  0x123449). A later route calls `_logWriteLine` with leading candidates `[[sp+4]+44]` and `sp-84`
  (`call` at 0x123476). A physical display effect is not established by these call sites.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12342d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123430 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123451 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x123458 | load | 1 | `[sp+4] +0x24` | `-` |
| `mov` at 0x123472 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123439 | `_bzero 0x174e40` | `[sp+4]+28` | `0xa` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x123449 | `_strncpy 0x182d28` | `[sp+4]+28` | `[sp+8]` | `0xa` | `?` | `?` | `?` |
| `call` at 0x123469 | `_sprintf 0x1755a0` | `sp-84` | `0x1233f0` | `[sp+4]+28` | `?` | `?` | `?` |
| `call` at 0x123476 | `_logWriteLine 0x126b5c` | `[[sp+4]+44]` | `sp-84` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123443 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x123456 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123484 | `eax` | `?` |

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123514 to 0x1235c0.
  Established. The initial `_bzero` call has leading candidates `sp-68` and `0x40` (`call` at
  0x123523). A later four-byte store writes `0x3` to an unknown effective target (`mov` at
  0x123556). `_nvramWrite` presents leading candidates `0x5` and `sp-68` (`call` at 0x123586); its
  branch takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero (`jne` at
  0x123590). The saved-record layout and indexed source addresses remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12352d | store | 4 | `sp -0x48` | `sp-68` |
| `mov` at 0x123530 | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x123533 | store | 4 | `sp -0x54` | `0x1df9b6` |
| `mov` at 0x123541 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123547 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x123549 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12354f | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x123552 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x123556 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x123560 | load | 4 | `sp -0x54` | `-` |
| `add` at 0x123573 | load | 4 | `sp -0x54` | `-` |
| `add` at 0x123573 | store | 4 | `sp -0x54` | `[sp-84]+60` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123523 | `_bzero 0x174e40` | `sp-68` | `0x40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x123568 | `_strncpy 0x182d28` | `?` | `?` | `0x4` | `?` | `?` | `?` |
| `call` at 0x123586 | `_nvramWrite 0x128b18` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x1235a9 | `_ffscMsg 0x120504` | `0x1234db` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12357e | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x123590 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1235bc | `eax` | `?` |

## `_routeProcessELSCCommand`

- `_routeProcessELSCCommand`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12b254 to 0x12b344.
  Established. A four-byte load reads absolute `0x1f3454` (`mov` at 0x12b25d). The bit-test branch
  takes `C == 0` and falls through for `not (C == 0)` (`jae` at 0x12b282); its fall-through calls
  `_elscSendPassword` with leading candidates unknown and `[[sp+8]+32]` (`call` at 0x12b28c), then
  `_elscSendCommand` with leading candidates unknown and `[sp+4]` (`call` at 0x12b299). An
  alternative route stores four-byte `0xb` at an unknown effective target (`mov` at 0x12b2c9). The
  bay-selection and reply-state meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12b25a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12b25d | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x12b262 | load | 1 | `([0x1f3454] + [sp+12]) +0x4` | `-` |
| `mov` at 0x12b270 | load | 4 | `abs 0x1f3454` | `-` |
| `mov` at 0x12b276 | load | 1 | `([0x1f3454] + [sp+12]) +0x4` | `-` |
| `mov` at 0x12b284 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12b287 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x12b294 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12b2a4 | load | 1 | `([0x1f3454] + [sp+12]) +0x4` | `-` |
| `mov` at 0x12b2b1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b2b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b2c3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12b2c9 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12b28c | `_elscSendPassword 0x1230b4` | `?` | `[[sp+8]+32]` | `?` | `?` | `?` | `?` |
| `call` at 0x12b299 | `_elscSendCommand 0x122f14` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x12b2bb | `_sprintf 0x1755a0` | `?` | `0x12b24a` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12b268 | `(([([0x1f3454] + [sp+12])+4]:1 & 0xff) & ([([0x1f3454] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f3454] + [sp+12])+4]:1 & 0xff) & ([([0x1f3454] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jae` at 0x12b282 | `C == 0` | `not (C == 0)` |
| `jge` at 0x12b2aa | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jbe` at 0x12b2d7 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12b2e2 | `eax` | `?` |
