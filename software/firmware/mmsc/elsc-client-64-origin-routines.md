# MMSC ELSC client and command paths in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Three undecoded bytes from `0x129291` to exclusive `0x129294` were read directly:
`00 90 90`. No instruction effects are credited to them.
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

[Fact sheets and complete listings](disasm/elsc-client-64-origin-facts.txt) include all tool sections.

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1202e0 to 0x1204c0.
  Established. Four-byte stores at unknown effective addresses write `0x1` (`mov` at 0x12034a) and
  `0x5` (`mov` at 0x1203f1). The byte store through `ret(_strpbrk) +0x0` writes `0x0` (`mov` at
  0x120455). The `_msgWrite` site has unknown candidate zero and candidate one `[0x1da150]` (`call`
  at 0x12037c). These facts do not identify a message format.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1202e6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1202e9 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1202ef | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1202fe | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120320 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12032d | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120336 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12033a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120346 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12034a | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x120351 | load | 1 | `abs 0x1da698` | `-` |
| `movsx` at 0x12035a | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120360 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120371 | load | 4 | `abs 0x1da150` | `-` |
| `mov` at 0x120377 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1203a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1203a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1203ac | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1203b0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1203e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1203ed | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1203f1 | store | 4 | `(unknown)` | `0x5` |
| `mov` at 0x1203f8 | load | 4 | `abs 0x1da13c` | `-` |
| `mov` at 0x1203fe | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120413 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120417 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12041f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120445 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120455 | store | 1 | `ret(_strpbrk) +0x0` | `0x0` |
| `test` at 0x120458 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x120461 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120465 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120469 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12046c | load | 1 | `[sp+4] +0x1aac80` | `-` |
| `mov` at 0x120487 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12048b | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12030a | `_ffscMsg 0x11ebd0` | `0x1201e0` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x120325 | `_msgClear 0x125ea0` | `?` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x12033e | `_sprintf 0x171680` | `?` | `0x12020b` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x120369 | `_ffscMsg 0x11ebd0` | `0x120210` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x12037c | `_msgWrite 0x126680` | `?` | `[0x1da150]` | `?` | `?` | `?` | `?` |
| `call` at 0x120392 | `_sprintf 0x171680` | `[sp+12]` | `0x120241` | `?` | `?` | `?` | `?` |
| `call` at 0x1203bb | `_ffscMsg 0x11ebd0` | `0x12024c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1203d1 | `_sprintf 0x171680` | `[sp+12]` | `0x120241` | `?` | `?` | `?` | `?` |
| `call` at 0x1203e5 | `_msgClear 0x125ea0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120403 | `_msgRead 0x126160` | `?` | `[0x1da13c]` | `?` | `?` | `?` | `?` |
| `call` at 0x120428 | `_ffscMsg 0x11ebd0` | `0x12027e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120449 | `_strpbrk 0x17e1f4` | `?` | `0x1202a6` | `?` | `?` | `?` | `?` |
| `call` at 0x12047b | `_ffscMsg 0x11ebd0` | `0x1202a9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120490 | `_strcpy 0x17dfdc` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x120303 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `je` at 0x120358 | `(([0x1da698]:1 & 0xff) & 0x10) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x120386 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x12038a | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x1203aa | `? == 0xb` | `? != 0xb` |
| `je` at 0x1203c5 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `jne` at 0x12040d | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x12041d | `? == 0xb` | `? != 0xb` |
| `je` at 0x120453 | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` |
| `je` at 0x12045f | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x120485 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1204a0 | `eax` | `?` |

## `_elscSendCommand`

- `_elscSendCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120fe0 to 0x121180.
  Established. The first comparison branch is taken when `? == 0x1` (`je` at 0x121013). Four-byte stores at
  unresolved effective addresses include `0xb` (`mov` at 0x12102b), `0x1` (`mov` at 0x121067), and
  `0x5` (`mov` at 0x121104). The `_msgWrite` result branch is taken when `(ret(_msgWrite) &
  ret(_msgWrite)) == 0x0` (`je` at 0x1210a0). The store meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120fe9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120fec | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x120ff2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121001 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x12100e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12101a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12101e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121027 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12102b | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x121040 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121053 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121057 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121063 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121067 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x12106e | load | 1 | `abs 0x1da698` | `-` |
| `movsx` at 0x121077 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12108b | load | 4 | `abs 0x1da150` | `-` |
| `mov` at 0x121091 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1210a7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1210ab | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1210c0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1210c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1210cc | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1210d0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1210e8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1210ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121100 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121104 | store | 4 | `(unknown)` | `0x5` |
| `cmp` at 0x12110b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121125 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121006 | `_msgClear 0x125ea0` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x121022 | `_sprintf 0x171680` | `?` | `0x120f6b` | `?` | `?` | `?` | `?` |
| `call` at 0x121045 | `_msgClear 0x125ea0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12105b | `_sprintf 0x171680` | `?` | `0x12020b` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121083 | `_ffscMsg 0x11ebd0` | `0x120f77` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x121096 | `_msgWrite 0x126680` | `?` | `[0x1da150]` | `?` | `?` | `?` | `?` |
| `call` at 0x1210af | `_strcpy 0x17dfdc` | `?` | `0x120241` | `?` | `?` | `?` | `?` |
| `call` at 0x1210db | `_ffscMsg 0x11ebd0` | `0x120f9a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1210f0 | `_strcpy 0x17dfdc` | `?` | `0x120241` | `?` | `?` | `?` | `?` |
| `call` at 0x12111c | `_sprintf 0x171680` | `sp-124` | `0x120fc6` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12112a | `_logWriteLine 0x124ff0` | `?` | `sp-124` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121013 | `? == 0x1` | `? != 0x1` |
| `je` at 0x121075 | `(([0x1da698]:1 & 0xff) & 0x10) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x1210a0 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x1210ca | `? == 0xb` | `? != 0xb` |
| `je` at 0x121110 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12113d | `eax` | `0x0` |

## `_elscSendPassword`

- `_elscSendPassword`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121180 to 0x121240.
  Established. The `_elscDoAdminCommand` call presents leading candidates `[sp+4]`, `sp-88`, and
  `sp-172` (`call` at 0x1211e3). The later `_strncmp` call has `sp-172`, `0x120e0d`, and `0x2` as
  its first three candidates (`call` at 0x1211f7); its result branch is taken when `(ret(_strncmp) &
  ret(_strncmp)) != 0x0` (`jne` at 0x121201). The comparison extent and password encoding are not
  established by these candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12118b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x121198 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1211a1 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1211b3 | `_sprintf 0x171680` | `sp-88` | `0x12113e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x1211cf | `_sprintf 0x171680` | `sp-88` | `0x121142` | `?` | `ebx` | `esi` | `?` |
| `call` at 0x1211e3 | `_elscDoAdminCommand 0x1202e0` | `[sp+4]` | `sp-88` | `sp-172` | `?` | `?` | `?` |
| `call` at 0x1211f7 | `_strncmp 0x1749b8` | `sp-172` | `0x120e0d` | `0x2` | `?` | `?` | `?` |
| `call` at 0x121216 | `_ffscMsg 0x11ebd0` | `0x121149` | `sp-172` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12119f | `? == 0x0` | `? != 0x0` |
| `jg` at 0x1211a8 | `[sp+8] > 0x1f3 (signed, 32-bit)` | `[sp+8] <= 0x1f3 (signed, 32-bit)` |
| `jne` at 0x1211ed | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x121201 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12122b | `eax` | `?` |

## `_elscSetPassword`

- `_elscSetPassword`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121240 to 0x1213f0.
  Established. The `_elscSendPassword` site presents `[sp+4]` and `0x3e8` as its leading candidates
  (`call` at 0x1212c9). The `_elscDoAdminCommand` site presents `[sp+4]`, `sp-88`, and `0x0` (`call`
  at 0x1212f1). Two four-byte stores write `0xb` at unknown effective addresses (`mov` at 0x1212aa,
  `mov` at 0x121364). No password-setting effect is assigned from the names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121249 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12124c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x121252 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121261 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x121276 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121280 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121283 | load | 1 | `[sp+8] +0x0` | `-` |
| `cmp` at 0x12128d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121299 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12129d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1212a6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1212aa | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1212c5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1212d5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1212ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121302 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121306 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121316 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121325 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121329 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x12132f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121353 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121357 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121360 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121364 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121266 | `_msgClear 0x125ea0` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1212a1 | `_sprintf 0x171680` | `?` | `0x120f6b` | `?` | `?` | `?` | `?` |
| `call` at 0x1212c9 | `_elscSendPassword 0x121180` | `[sp+4]` | `0x3e8` | `?` | `?` | `?` | `?` |
| `call` at 0x1212e2 | `_sprintf 0x171680` | `sp-88` | `0x12122c` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1212f1 | `_elscDoAdminCommand 0x1202e0` | `[sp+4]` | `sp-88` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12130a | `_strcmp 0x17de80` | `?` | `0x120e0d` | `?` | `?` | `?` | `?` |
| `call` at 0x12131b | `_msgClear 0x125ea0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12133c | `_strncpy 0x17e1b0` | `?` | `[sp+8]` | `0x4` | `?` | `?` | `?` |
| `call` at 0x121344 | `_elscUpdateSaveInfo 0x121540` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12135b | `_sprintf 0x171680` | `?` | `0x121237` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121270 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x12127a | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `je` at 0x121287 | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) == 0x0` | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) != 0x0` |
| `je` at 0x121292 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x1212d3 | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` |
| `jne` at 0x1212fb | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x121314 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121376 | `eax` | `?` |

## `_elscSetup`

- `_elscSetup`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1213f0 to 0x1214c0.
  Established. The first `_elscDoAdminCommand` site has leading candidates `[[sp+4]+12]`,
  `0x12138b`, and `sp-88` (`call` at 0x121425); its result branch is taken when
  `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` (`je` at 0x12142f). Four-byte
  stores through `[sp+4] +0x0` retain unknown values (`mov` at 0x12147a, `mov` at 0x1214ac). A later
  call uses `[[sp+4]+12]`, `0x1213e0`, and `0x0` (`call` at 0x1214a3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1213f8 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x1213fb | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x121404 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121421 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x121431 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121465 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121476 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x12147a | store | 4 | `[sp+4] +0x0` | `?` |
| `mov` at 0x121490 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x12149f | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1214a8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1214ac | store | 4 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121410 | `_ffscMsg 0x11ebd0` | `0x121377` | `?` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x121425 | `_elscDoAdminCommand 0x1202e0` | `[[sp+4]+12]` | `0x12138b` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x12143d | `_ffscMsg 0x11ebd0` | `0x121391` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121458 | `_strncmp 0x1749b8` | `sp-88` | `0x120e0d` | `0x2` | `?` | `?` | `?` |
| `call` at 0x121471 | `_ffscMsg 0x11ebd0` | `0x1213bc` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x1214a3 | `_elscDoAdminCommand 0x1202e0` | `[[sp+4]+12]` | `0x1213e0` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121402 | `(([0x1da698]:1 & 0xff) & 0x2) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x12142f | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x121462 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x121496 | `[[sp+4]+4] != 0x1` | `[[sp+4]+4] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1214b8 | `eax` | `?` |

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120e60 to 0x120fe0.
  Established. The initial four-byte store through `[sp+8] +0x0` writes `0xffffffff` (`mov` at
  0x120e73). The `_elscDoAdminCommand` site has leading candidates `[sp+4]`, `0x120dde`, and `sp-88`
  (`call` at 0x120e83). The `_strtol` call presents `sp-85`, `sp-92`, and `0x10` (`call` at
  0x120f0a). A later four-byte store through `[sp+8] +0x0` retains an unknown value (`mov` at
  0x120f5d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120e69 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120e6c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120e73 | store | 4 | `[sp+8] +0x0` | `0xffffffff` |
| `or` at 0x120ea0 | load | 1 | `(unknown)` | `-` |
| `or` at 0x120ea0 | store | 1 | `(unknown)` | `?` |
| `movsx` at 0x120edc | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120f14 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120f17 | load | 1 | `[sp-92] +0x0` | `-` |
| `movsx` at 0x120f2d | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120f5d | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120e83 | `_elscDoAdminCommand 0x1202e0` | `[sp+4]` | `0x120dde` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x120e94 | `_ffscMsg 0x11ebd0` | `0x120de2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120ec8 | `_strncmp 0x1749b8` | `sp-88` | `0x120e0d` | `0x2` | `?` | `?` | `?` |
| `call` at 0x120eea | `_ffscMsg 0x11ebd0` | `0x120e10` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x120f0a | `_strtol 0x17d9f8` | `sp-85` | `sp-92` | `0x10` | `?` | `?` | `?` |
| `call` at 0x120f3b | `_ffscMsg 0x11ebd0` | `0x120e32` | `?` | `sp-85` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120e71 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x120e8d | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x120ed2 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x120f1b | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) != 0x0` | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x120f23 | `ret(_strtol) <= 0xff (unsigned, 32-bit)` | `ret(_strtol) > 0xff (unsigned, 32-bit)` |
| `jne` at 0x120f52 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x120f5b | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120f6a | `eax` | `?` |

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1214c0 to 0x121540.
  Established. Both explicit four-byte stores have unknown targets and values (`rep movsd` at
  0x1214d6, `mov` at 0x1214e8). The loop branch is taken when `? <= 0x1 (unsigned, 32-bit)` (`jbe`
  at 0x1214f3). The `_addrdUpdateLocalIdentity` site presents candidate zero `sp-32` (`call` at
  0x1214f9). Neither an identity layout nor its update semantics follows from that name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x1214d6 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1214d6 | load | 4 | `abs 0x1ee558` | `-` |
| `mov` at 0x1214e4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1214e8 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1214f9 | `_addrdUpdateLocalIdentity 0x10db70` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1214f3 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121506 | `eax` | `ret(_addrdUpdateLocalIdentity)` |

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121540 to 0x1215f0.
  Established. The `_bzero` site has leading candidates `sp-68` and `0x40` (`call` at 0x12154f). The
  `_strncpy` site retains unknown candidates zero and one, followed by `0x4` (`call` at 0x121594).
  The `_nvramWrite` call presents `0x5` and `sp-68` (`call` at 0x1215b2); its result branch is taken
  when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` (`jne` at 0x1215bc). No record size or write
  outcome is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121559 | store | 4 | `sp -0x48` | `sp-68` |
| `mov` at 0x12155c | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x12155f | store | 4 | `sp -0x54` | `0x1daade` |
| `mov` at 0x12156d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121573 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121575 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12157b | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x12157e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121582 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x12158c | load | 4 | `sp -0x54` | `-` |
| `add` at 0x12159f | load | 4 | `sp -0x54` | `-` |
| `add` at 0x12159f | store | 4 | `sp -0x54` | `[sp-84]+60` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12154f | `_bzero 0x170f20` | `sp-68` | `0x40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121594 | `_strncpy 0x17e1b0` | `?` | `?` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1215b2 | `_nvramWrite 0x126e80` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x1215d5 | `_ffscMsg 0x11ebd0` | `0x121507` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1215aa | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x1215bc | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1215e8 | `eax` | `?` |

## `_elscInit`

- `_elscInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1204c0 to 0x120790.
  Established. The `_nvramRead` site presents leading candidates `0x5` and `sp-68` (`call` at
  0x1204d6); its result branch is taken when `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` (`je` at
  0x1204e0). The later `_nvramWrite` site uses the same leading candidate values (`call` at
  0x120537). Four-byte stores at unknown effective addresses retain the printed result expression
  `ret(_msgNew)` (`mov` at 0x12065d, `mov` at 0x120694, `mov` at 0x1206cb). Separate call results do
  not establish equal pointers.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1204c9 | store | 4 | `sp -0x54` | `0x0` |
| `mov` at 0x1204f1 | store | 4 | `sp -0x74` | `sp-68` |
| `mov` at 0x1204f9 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1204ff | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x120502 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12051d | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x120520 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x120549 | store | 4 | `sp -0x58` | `sp-68` |
| `mov` at 0x12054f | store | 4 | `sp -0x5c` | `sp-76` |
| `mov` at 0x120554 | store | 4 | `sp -0x60` | `0x0` |
| `mov` at 0x12055b | store | 4 | `sp -0x64` | `0x1daade` |
| `mov` at 0x120562 | store | 4 | `sp -0x68` | `0x0` |
| `mov` at 0x120569 | store | 4 | `sp -0x6c` | `0x1daad4` |
| `mov` at 0x120570 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12057a | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x120582 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120589 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12058d | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120591 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120599 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x12059c | load | 4 | `sp -0x58` | `-` |
| `mov` at 0x12059f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1205a2 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1205a6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1205ac | store | 1 | `(unknown)` | `?` |
| `mov` at 0x1205b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1205b7 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x1205bd | load | 4 | `sp -0x6c` | `-` |
| `mov` at 0x1205c9 | load | 4 | `sp -0x58` | `-` |
| `mov` at 0x1205cc | load | 4 | `sp -0x68` | `-` |
| `test` at 0x1205cf | load | 1 | `([sp-104] + [sp-88]) +0x1c` | `-` |
| `mov` at 0x1205d6 | load | 4 | `([sp-104] + [sp-88]) +0x4` | `-` |
| `mov` at 0x1205da | store | 4 | `(unknown)` | `[([sp-104] + [sp-88])+4]` |
| `mov` at 0x1205e0 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1205e8 | load | 4 | `sp -0x58` | `-` |
| `mov` at 0x1205eb | load | 4 | `sp -0x68` | `-` |
| `test` at 0x1205ee | load | 1 | `([sp-104] + [sp-88]) +0x1c` | `-` |
| `mov` at 0x1205fc | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120608 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x120615 | load | 4 | `sp -0x64` | `-` |
| `cmp` at 0x120621 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120628 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x120636 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120642 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120648 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12065d | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x120665 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120670 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x12067c | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120682 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120694 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x12069c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1206a7 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1206b3 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1206b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1206cb | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `mov` at 0x120704 | load | 4 | `sp -0x50` | `-` |
| `cmp` at 0x120707 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12070d | store | 4 | `(unknown)` | `[sp-80]` |
| `mov` at 0x120711 | store | 4 | `sp -0x54` | `0x1` |
| `add` at 0x12071b | load | 4 | `sp -0x60` | `-` |
| `add` at 0x12071b | store | 4 | `sp -0x60` | `[sp-96]+16` |
| `add` at 0x12071f | load | 4 | `sp -0x64` | `-` |
| `add` at 0x12071f | store | 4 | `sp -0x64` | `[sp-100]+60` |
| `add` at 0x120723 | load | 4 | `sp -0x68` | `-` |
| `add` at 0x120723 | store | 4 | `sp -0x68` | `[sp-104]+32` |
| `add` at 0x120727 | load | 4 | `sp -0x6c` | `-` |
| `add` at 0x120727 | store | 4 | `sp -0x6c` | `[sp-108]+60` |
| `cmp` at 0x120735 | load | 4 | `sp -0x54` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1204d6 | `_nvramRead 0x126cf0` | `0x5` | `sp-68` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x1204e5 | `_bzero 0x170f20` | `sp-68` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x120515 | `_strncpy 0x17e1b0` | `?` | `0x1204a1` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120537 | `_nvramWrite 0x126e80` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x1205c1 | `_bzero 0x170f20` | `[sp-108]` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x120600 | `_strncpy 0x17e1b0` | `[sp-100]` | `([sp-104] + [sp-88])+8` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120619 | `_strcpy 0x17dfdc` | `[sp-100]` | `0x1204a1` | `?` | `?` | `?` | `?` |
| `call` at 0x12063a | `_sprintf 0x171680` | `[sp-92]` | `0x1204a6` | `?` | `?` | `?` | `?` |
| `call` at 0x120655 | `_msgNew 0x125f50` | `0xf0` | `?` | `0x1` | `[sp-92]` | `?` | `?` |
| `call` at 0x120674 | `_sprintf 0x171680` | `[sp-92]` | `0x1204ad` | `?` | `?` | `?` | `?` |
| `call` at 0x12068c | `_msgNew 0x125f50` | `0x50` | `?` | `0x1` | `[sp-92]` | `?` | `?` |
| `call` at 0x1206ab | `_sprintf 0x171680` | `[sp-92]` | `0x1204b4` | `?` | `?` | `?` | `?` |
| `call` at 0x1206c3 | `_msgNew 0x125f50` | `0x50` | `?` | `0x1` | `[sp-92]` | `?` | `?` |
| `call` at 0x1206e7 | `_elscSetup 0x1213f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1206f8 | `_elscQueryModuleNum 0x120e60` | `?` | `sp-80` | `?` | `?` | `?` | `?` |
| `call` at 0x12073b | `_elscUpdateModuleNums 0x1214c0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1204e0 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x12052f | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x1205d4 | `(([([sp-104] + [sp-88])+28]:1 & 0xff) & 0x1) == 0x0` | `(([([sp-104] + [sp-88])+28]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x1205f3 | `(([([sp-104] + [sp-88])+28]:1 & 0xff) & 0x2) == 0x0` | `(([([sp-104] + [sp-88])+28]:1 & 0xff) & 0x2) != 0x0` |
| `jl` at 0x120626 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x120663 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `je` at 0x12069a | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `jne` at 0x1206d1 | `(ret(_msgNew) & ret(_msgNew)) != 0x0` | `(ret(_msgNew) & ret(_msgNew)) == 0x0` |
| `jne` at 0x1206f1 | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x120702 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x12070b | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x12072f | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120739 | `[sp-84] == 0x0` | `[sp-84] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12074b | `eax` | `?` |

## `_elscProcessInput`

- `_elscProcessInput`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120c40 to 0x120e60.
  Established. The four-byte store through `[[sp+4]+48] +0x1c` writes `0x4` (`mov` at 0x120c7f). The
  `_msgRead` site presents `[[sp+4]+48]` and `[0x1da138]` (`call` at 0x120c90). A byte store through
  `[[[sp+4]+48]+12] +0x0` writes `0x0` (`mov` at 0x120d30). The later `_elscProcessCommand` site has
  candidate zero `[sp+4]` and unknown remaining slots (`call` at 0x120dae). Buffer and protocol
  roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120c49 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120c4c | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x120c54 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x120c70 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120c7c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120c7f | store | 4 | `[[sp+4]+48] +0x1c` | `0x4` |
| `mov` at 0x120c86 | load | 4 | `abs 0x1da138` | `-` |
| `mov` at 0x120c8c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120cb0 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120cb3 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x120cbb | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120cbf | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120cd0 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x120cd8 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x120cde | store | 4 | `[[sp+4]+48] +0xc` | `[[[sp+4]+48]+12]-2` |
| `mov` at 0x120cf0 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x120cf8 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x120cfb | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120d02 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120d09 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x120d20 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x120d24 | store | 4 | `[[sp+4]+48] +0xc` | `?` |
| `mov` at 0x120d27 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x120d2a | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120d2d | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x120d30 | store | 1 | `[[[sp+4]+48]+12] +0x0` | `0x0` |
| `mov` at 0x120d33 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120d36 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120d39 | load | 1 | `[[[sp+4]+48]+8] +0x0` | `-` |
| `mov` at 0x120d3f | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x120d42 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120d49 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120d50 | load | 4 | `abs 0x1ee564` | `-` |
| `test` at 0x120d67 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x120d70 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120d73 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120d86 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120d89 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120da0 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x120da8 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120dc0 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120dc4 | load | 4 | `[sp+4] +0x30` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120c5d | `_ffscMsg 0x11ebd0` | `0x120b8f` | `[[sp+4]+4]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120c74 | `_msgClear 0x125ea0` | `[[sp+4]+48]` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x120c90 | `_msgRead 0x126160` | `[[sp+4]+48]` | `[0x1da138]` | `?` | `?` | `?` | `?` |
| `call` at 0x120d5f | `_sprintf 0x171680` | `sp-16` | `0x120bec` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x120d7e | `_ffscMsg 0x11ebd0` | `0x120bf3` | `sp-16` | `?` | `[[[sp+4]+48]+8]` | `?` | `?` |
| `call` at 0x120d8f | `_elscPrintMsg 0x120790` | `[sp+4]` | `sp-16` | `[[[sp+4]+48]+8]` | `?` | `?` | `?` |
| `call` at 0x120dae | `_elscProcessCommand 0x120970` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120dcd | `_ffscMsg 0x11ebd0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120c52 | `[[sp+4]+4] == 0x1` | `[[sp+4]+4] != 0x1` |
| `je` at 0x120c9a | `(ret(_msgRead) & ret(_msgRead)) == 0x0` | `(ret(_msgRead) & ret(_msgRead)) != 0x0` |
| `jne` at 0x120cb9 | `[[[sp+4]+48]+28] != 0xa` | `[[[sp+4]+48]+28] == 0xa` |
| `jne` at 0x120cd6 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |
| `je` at 0x120cf6 | `[[[sp+4]+48]+28] == 0x17` | `[[[sp+4]+48]+28] != 0x17` |
| `jne` at 0x120d00 | `[[[sp+4]+48]+12] != [[[sp+4]+48]+8]` | `[[[sp+4]+48]+12] == [[[sp+4]+48]+8]` |
| `je` at 0x120d3d | `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14` | `([[[[sp+4]+48]+8]]:1 & 0xff) != 0x14` |
| `je` at 0x120d6e | `(([0x1da698]:1 & 0xff) & 0x10) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x120da6 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120ddd | `eax` | `?` |

## `_elscProcessCommand`

- `_elscProcessCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120970 to 0x120c40.
  Established. The `_strtok_r` site has leading candidates `[sp+8]`, `0x1208c4`, and `sp-8` (`call`
  at 0x1209b1); its result branch is taken when `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` (`jne` at
  0x1209bd). The `_strncpy` site presents `[sp+4]+28`, `[sp-8]`, and `0xa` (`call` at 0x120a16). A
  four-byte store through `[sp+4] +0x10` writes `[sp-92]` (`mov` at 0x120aec). These candidates do
  not establish command grammar.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12097c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12097f | load | 4 | `sp +0x8` | `-` |
| `test` at 0x120982 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x12098c | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1209a0 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x1209bf | load | 1 | `[sp+4] +0x2b` | `-` |
| `cmp` at 0x120a09 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x120a11 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x120a1e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120a29 | load | 1 | `[sp+4] +0x24` | `-` |
| `mov` at 0x120a43 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120a88 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120a8f | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120aa8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120ac9 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x120add | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x120ae0 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120ae9 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120aec | store | 4 | `[sp+4] +0x10` | `[sp-92]` |
| `mov` at 0x120b12 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120b19 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x120b30 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x120b41 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120b5c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120b64 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120998 | `_ffscMsg 0x11ebd0` | `0x1208aa` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` |
| `call` at 0x1209b1 | `_strtok_r 0x17e358` | `[sp+8]` | `0x1208c4` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x1209cb | `_ffscMsg 0x11ebd0` | `0x1208c7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1209e1 | `_ffscConvertToUpperCase 0x12543c` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1209ef | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x1208e1` | `?` | `?` | `?` | `?` |
| `call` at 0x120a01 | `_bzero 0x170f20` | `[sp+4]+28` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x120a16 | `_strncpy 0x17e1b0` | `[sp+4]+28` | `[sp-8]` | `0xa` | `?` | `?` | `?` |
| `call` at 0x120a3a | `_sprintf 0x171680` | `sp-88` | `0x1208e5` | `[sp+4]+28` | `?` | `?` | `?` |
| `call` at 0x120a47 | `_logWriteLine 0x124ff0` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x120a68 | `_strncmp 0x1749b8` | `ret(_strtok_r)` | `0x120910` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120a7c | `_strncmp 0x1749b8` | `ret(_strtok_r)` | `0x120915` | `0x3` | `?` | `?` | `?` |
| `call` at 0x120a9f | `_sprintf 0x171680` | `sp-88` | `0x120919` | `?` | `?` | `?` | `?` |
| `call` at 0x120aac | `_logWriteLine 0x124ff0` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x120ab5 | `_elscSetup 0x1213f0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120acd | `_elscQueryModuleNum 0x120e60` | `[[sp+4]+12]` | `sp-92` | `?` | `?` | `?` | `?` |
| `call` at 0x120aef | `_elscUpdateModuleNums 0x1214c0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120b06 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x120932` | `?` | `?` | `?` | `?` |
| `call` at 0x120b28 | `_sprintf 0x171680` | `sp-88` | `0x120936` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x120b36 | `_elscPrintMsg 0x120790` | `[sp+4]` | `sp-88` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x120b54 | `_sprintf 0x171680` | `sp-348` | `0x12093d` | `?` | `[sp+8]` | `?` | `?` |
| `call` at 0x120b68 | `_logWriteLine 0x124ff0` | `[[sp+4]+44]` | `sp-348` | `?` | `?` | `?` | `?` |
| `call` at 0x120b71 | `_ffscMsg 0x11ebd0` | `sp-348` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120989 | `(([0x1da698]:1 & 0xff) & 0x10) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x1209bd | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x1209f9 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x120a0d | `[sp-8] == 0x0` | `[sp-8] != 0x0` |
| `je` at 0x120a23 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `je` at 0x120a72 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x120a86 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x120a8d | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x120abf | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x120ad7 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x120ae3 | `[sp-92] == [[sp+4]+16]` | `[sp-92] != [[sp+4]+16]` |
| `jne` at 0x120b10 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x120b61 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120b8e | `eax` | `?` |

## `_routeDoAdminCommand`

- `_routeDoAdminCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x129274 to 0x129294.
  Established. The four-byte loads read `sp +0x4` (`mov` at 0x129277) and absolute `0x1da97c` (`mov`
  at 0x12927a). The `_ProcessCommand` site presents leading candidates `[sp+4]` and `[0x1da97c]`
  (`call` at 0x129281). The exit reports `ret(_ProcessCommand)` in `eax` (`ret` at 0x129289).
  Post-return decoded bytes and the raw tail receive no executable effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x129277 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12927a | load | 4 | `abs 0x1da97c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x129281 | `_ProcessCommand 0x1296c4` | `[sp+4]` | `[0x1da97c]` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x129289 | `eax` | `ret(_ProcessCommand)` |

## `_routeProcessELSCCommand`

- `_routeProcessELSCCommand`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x129294 to 0x129384.
  Established. The byte loads include target `([0x1ee564] + [sp+12]) +0x4` (`mov` at 0x1292a2, `mov`
  at 0x1292b6, `mov` at 0x1292e4). The `_elscSendPassword` call has unknown candidate zero and
  candidate one `[[sp+8]+32]` (`call` at 0x1292cc). The `_elscSendCommand` call has unknown
  candidate zero and candidate one `[sp+4]` (`call` at 0x1292d9). A four-byte store at an unresolved
  effective address writes `0xb` (`mov` at 0x129309). Selection and message meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12929a | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12929d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1292a2 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x1292b0 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1292b6 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x1292c4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1292c7 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1292d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1292e4 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x1292f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1292f7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129303 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129309 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1292cc | `_elscSendPassword 0x121180` | `?` | `[[sp+8]+32]` | `?` | `?` | `?` | `?` |
| `call` at 0x1292d9 | `_elscSendCommand 0x120fe0` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x1292fb | `_sprintf 0x171680` | `?` | `0x12928a` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1292a8 | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jae` at 0x1292c2 | `C == 0` | `not (C == 0)` |
| `jge` at 0x1292ea | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jbe` at 0x129317 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x129322 | `eax` | `?` |

## `_cmdELSC`

- `_cmdELSC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115df0 to 0x116250.
  Established. The `_routeProcessELSCCommand` site presents `[[sp+4]]`, `[sp+8]`, and `[sp+12]` as
  its leading candidates (`call` at 0x115e23). A four-byte store through `[[sp+8]+132] +0x14` writes
  `0xffffffff` (`mov` at 0x115ff6). The subsequent byte store has an unknown effective address and
  value `0x0` (`mov` at 0x11600c). Another four-byte store writes `0x1` at absolute `0x1eebc0`
  (`mov` at 0x11614f). These stores do not identify field roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x115dfc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x115dff | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x115e02 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x115e05 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x115e19 | load | 1 | `(ret(_strspn) + [[sp+4]]) +0x0` | `-` |
| `mov` at 0x115e40 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x115e47 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x115e4e | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115e56 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115eb0 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115ebc | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x115ebf | load | 4 | `abs 0x1ee564` | `-` |
| `test` at 0x115ee0 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x115ee9 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x115eef | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x115f00 | load | 4 | `sp -0xc` | `-` |
| `sub` at 0x115f06 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x115f11 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f18 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f2a | load | 4 | `sp -0xc` | `-` |
| `sub` at 0x115f30 | load | 4 | `sp -0xc` | `-` |
| `movsx` at 0x115f33 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x115f39 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115f44 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f4b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f54 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115f59 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115f60 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x115fa2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x115fa8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x115fac | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x115fc4 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115fc9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115fd0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115ff0 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x115ff6 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x115ffd | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116002 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116009 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11600c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x116020 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x116023 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11602f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116049 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11604e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116055 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116084 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116087 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11608e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116094 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11609d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1160a2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160a9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160cd | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1160d2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1160e0 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x1160f2 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1160f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x116105 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11610c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116115 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11611a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116121 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x11613d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116144 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11614f | store | 4 | `abs 0x1eebc0` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x115e11 | `_strspn 0x17e264` | `[[sp+4]]` | `0x115930` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x115e23 | `_routeProcessELSCCommand 0x129294` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x115e64 | `_ffscMsg 0x11ebd0` | `0x115c81` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x115e89 | `_destNumBays 0x11eeec` | `[sp+12]` | `sp-8` | `sp-12` | `?` | `?` | `?` |
| `call` at 0x115e9c | `_ffscMsg 0x11ebd0` | `0x115ca1` | `ret(_destNumBays)` | `?` | `?` | `?` | `?` |
| `call` at 0x115ecc | `_ffscMsg 0x11ebd0` | `0x115cd3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115ef8 | `_ffscMsg 0x11ebd0` | `0x115d05` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x115f22 | `_ffscMsg 0x11ebd0` | `0x115d31` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115f4f | `_sprintf 0x171680` | `?` | `0x115d4f` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x115f81 | `_userAcquireRemote 0x12a978` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115fbc | `_sprintf 0x171680` | `sp-256` | `0x115d62` | `[sp-12]` | `[[[sp+8]+132]+20]` | `0x3` | `?` |
| `call` at 0x115fd6 | `_userSendCommand 0x12b7d8` | `ret(_userAcquireRemote)` | `sp-256` | `?` | `?` | `?` | `?` |
| `call` at 0x115fe3 | `_userReleaseRemote 0x12b738` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11603c | `_ffscMsg 0x11ebd0` | `0x115d77` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116059 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116078 | `_remStartConnectCommand 0x128568` | `[sp+8]` | `0x115dc1` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1160ad | `_sprintf 0x171680` | `?` | `0x115dc6` | `?` | `?` | `?` | `?` |
| `call` at 0x116110 | `_sprintf 0x171680` | `?` | `0x115d4f` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x116148 | `_sprintf 0x171680` | `?` | `?` | `0x8000` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x115e09 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x115e1e | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) == 0x0` | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) != 0x0` |
| `je` at 0x115e54 | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x115e5c | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x115e94 | `ret(_destNumBays) == 0x1` | `ret(_destNumBays) != 0x1` |
| `jne` at 0x115eb6 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x115ec5 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `je` at 0x115ee7 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x115f16 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x115f8d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x115fe0 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x116029 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `jne` at 0x116035 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116082 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x1160f7 | `? == 0x1` | `? != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116165 | `eax` | `?` |

## `_cmdPWR`

- `_cmdPWR`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117da0 to 0x117fc0.
  Established. The `_destNumRacks` result branch is taken when `ret(_destNumRacks) > 0x1 (signed,
  32-bit)` (`jg` at 0x117dfd). The four-byte store through `[[sp+8]+136] +0x1c` writes `0x1` (`mov`
  at 0x117e6d). The `_nanosleep` site presents leading candidates `sp-256` and `0x0` (`call` at
  0x117f50). The `_routeProcessELSCCommand` call has `sp-248`, `[sp+8]`, and `[sp+12]` as its first
  three candidates (`call` at 0x117f66). Neither a power action nor a delay duration is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117dac | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x117daf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x117dec | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117e25 | load | 4 | `[sp+8] +0x10` | `-` |
| `imul` at 0x117e34 | load | 4 | `abs 0x1da1a0` | `-` |
| `fld` at 0x117e3c | load | 8 | `abs 0x117d98` | `-` |
| `fidivr` at 0x117e43 | load | 4 | `sp -0x118` | `-` |
| `fstp` at 0x117e4c | store | 8 | `sp -0x11c` | `?` |
| `mov` at 0x117e55 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x117e5b | load | 4 | `[[sp+8]+136] +0x8` | `-` |
| `mov` at 0x117e67 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x117e6d | store | 4 | `[[sp+8]+136] +0x1c` | `0x1` |
| `mov` at 0x117e74 | load | 4 | `abs 0x1da158` | `-` |
| `mov` at 0x117e7a | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x117e8d | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x117e93 | load | 4 | `[[sp+8]+136] +0x1c` | `-` |
| `mov` at 0x117ead | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117eb2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117eb9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117ed0 | load | 4 | `[sp+8] +0x88` | `-` |
| `mov` at 0x117edf | load | 4 | `abs 0x1da1a0` | `-` |
| `mov` at 0x117ef8 | store | 4 | `sp -0x100` | `?` |
| `mov` at 0x117f25 | store | 4 | `sp -0xfc` | `?` |
| `mov` at 0x117f38 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117f3b | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x117f58 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117f60 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117f70 | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117dba | `_strtok_r 0x17e358` | `0x0` | `0x116c80` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x117dca | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x117d1d` | `?` | `?` | `?` | `?` |
| `call` at 0x117dde | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x117d1f` | `?` | `?` | `?` | `?` |
| `call` at 0x117df0 | `_destNumRacks 0x11ef3c` | `[sp+12]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x117e1d | `_sprintf 0x171680` | `sp-248` | `0x117d21` | `ret(_strtok_r)` | `?` | `?` | `?` |
| `call` at 0x117e5f | `_sprintf 0x171680` | `[[[sp+8]+136]+8]` | `0x117d28` | `?` | `?` | `?` | `?` |
| `call` at 0x117e81 | `_msgWrite 0x126680` | `[[sp+8]+136]` | `[0x1da158]` | `?` | `?` | `?` | `?` |
| `call` at 0x117ea0 | `_ffscMsg 0x11ebd0` | `0x117d66` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117ebd | `_sprintf 0x171680` | `?` | `0x117d8c` | `?` | `?` | `?` | `?` |
| `call` at 0x117ed7 | `_msgClear 0x125ea0` | `[[sp+8]+136]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117f50 | `_nanosleep 0x179570` | `sp-256` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x117f66 | `_routeProcessELSCCommand 0x129294` | `sp-248` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x117f77 | `_remStartRemoteCommand 0x128918` | `[sp+8]` | `sp-248` | `?` | `[sp+12]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x117dd6 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x117de8 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `jg` at 0x117dfd | `ret(_destNumRacks) > 0x1 (signed, 32-bit)` | `ret(_destNumRacks) <= 0x1 (signed, 32-bit)` |
| `je` at 0x117e2b | `[[sp+8]+16] == 0x3` | `[[sp+8]+16] != 0x3` |
| `jne` at 0x117e8b | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` |
| `je` at 0x117e99 | `[[[sp+8]+136]+28] == 0xb` | `[[[sp+8]+136]+28] != 0xb` |
| `je` at 0x117f41 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x117f45 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x117f5e | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x117f88 | `? <= 0x8 (unsigned, 32-bit)` | `? > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x117f9b | `eax` | `?` |
