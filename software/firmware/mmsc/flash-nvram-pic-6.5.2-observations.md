# MMSC flash, NVRAM and interrupt observations

The symbols below label software windows in one shipped carrier.
Hardware roles, types, pointee meanings and runtime placement remain unknown.
The x86-32 a.out load is 0x108000. Each next-symbol bound is inferred.
Local CFG reachability assumes ordinary call fallthrough. It does not establish runtime feasibility.
Call slots use the assumed cdecl32 convention. Actual arity and custom ABI remain unknown.
All six modeled slots remain literal, including saved-register and opaque call-result expressions.
Only native resolved memory targets are named. Unknown targets retain a raw base without a displacement.
Memory patterns group identical encoded instructions and identical native access fields within one entry.
Their counts are static records, with the first three actual sites listed.
These groups establish neither dynamic counts nor equal runtime pointees.
Proved entry register saves and paired terminal restores are omitted.
Incoming, local, outgoing and ambiguous stack accesses remain selected.
The exclusions describe encoded frame structure, without proving a runtime calling convention.
Known values remain literal. Only the tool value ? becomes unknown.
All decoded unreached runs are separate. They confer no effects or execution claims.
Context slices admit instructions only when their ends fit the requested bounds.
Crossers and early decoder-stop remainders remain raw. Context alignment is not generally proved.

The [full facts](disasm/flash-nvram-pic-6.5.2-observations-facts.txt) retain every access, instruction and warning.

- `_initFlash`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10cc14`.
  Inferred analysis window: 0x10cc14 to 0x10cc84 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 471, body file offset 0x4c34, window SHA256 `bd312adaab114f6cc018389f8c9490de5bf381f629627f277a4891d423ee24d3`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10cc17.
  Four-byte store, pattern 2: target `[sp+4] +0x0`, raw base `eax`, value `0x550`. Count 1 static access. Sites: `mov` at 0x10cc1a.
  Four-byte store, pattern 3: target `[sp+4] +0x4`, raw base `eax`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10cc20.
  Four-byte store, pattern 4: target `[sp+4] +0x8`, raw base `eax`, value `0x2`. Count 1 static access. Sites: `mov` at 0x10cc27.
  Four-byte store, pattern 5: target `[sp+4] +0xc`, raw base `eax`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10cc2e.
  Four-byte store, pattern 6: target `[sp+4] +0x10`, raw base `eax`, value `0x8000`. Count 1 static access. Sites: `mov` at 0x10cc35.
  Four-byte store, pattern 7: target `[sp+4] +0x14`, raw base `eax`, value `0x1`. Count 1 static access. Sites: `mov` at 0x10cc3c.
  Four-byte store, pattern 8: target `[sp+4] +0x18`, raw base `eax`, value `0x760`. Count 1 static access. Sites: `mov` at 0x10cc43.
  Four-byte store, pattern 9: target `[sp+4] +0x1c`, raw base `eax`, value `0x540`. Count 1 static access. Sites: `mov` at 0x10cc4a.
  Four-byte store, pattern 10: target `[sp+4] +0x20`, raw base `eax`, value `0x548`. Count 1 static access. Sites: `mov` at 0x10cc51.
  Four-byte store, pattern 11: target `[sp+4] +0x24`, raw base `eax`, value `0xa0000`. Count 1 static access. Sites: `mov` at 0x10cc58.
  Four-byte store, pattern 12: target `[sp+4] +0x28`, raw base `eax`, value `0x800`. Count 1 static access. Sites: `mov` at 0x10cc5f.
  Four-byte store, pattern 13: target `[sp+4] +0x2c`, raw base `eax`, value `0x1ac9a4`. Count 1 static access. Sites: `mov` at 0x10cc66.
  Four-byte store, pattern 14: target `[sp+4] +0x30`, raw base `eax`, value `0x1ac9e4`. Count 1 static access. Sites: `mov` at 0x10cc6d.
  At the exit (`ret` at 0x10cc77), eax=`[sp+4]`.
  Local unreached instructions: 0x10cc78 to 0x10cc83, ending at 0x10cc84 (exclusive). Count 12.

- `_openFlashWindow`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10cc84`.
  Inferred analysis window: 0x10cc84 to 0x10cd04 (exclusive), 128 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 472, body file offset 0x4ca4, window SHA256 `5fffbe93f3df221a080226b621a91526ccc00748184335437b25bf64ce3ece30`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10cc8a.
  Four-byte load, pattern 2: target `[sp+4] +0x10`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10cc8d.
  Four-byte load, pattern 3: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10cc90.
  Four-byte load, pattern 4: target `[sp+4] +0x28`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10cc96.
  Four-byte load, pattern 5: target `[sp+4] +0x24`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10ccaf.
  Four-byte store, pattern 6: target `sp -0x14`, raw base `esp`, value `0xffffffff`. Count 1 static access. Sites: `push` at 0x10ccb5.
  Four-byte load, pattern 7: target `[sp+4] +0x20`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10ccb7.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `[[sp+4]+32]`. Count 1 static access. Sites: `push` at 0x10ccba.
  Four-byte load, pattern 9: target `[sp+4] +0x2c`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10ccc3.
  Four-byte load, pattern 10: target `unknown`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x10ccc6.
  Four-byte load, pattern 11: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x10ccc9.
  Four-byte load, pattern 12: target `[sp+4] +0x30`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10cccf.
  Four-byte load, pattern 13: target `unknown`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x10ccd2.
  Four-byte store, pattern 14: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x10ccda.
  Four-byte load, pattern 15: target `[sp+4] +0x18`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10ccdb.
  Four-byte store, pattern 16: target `sp -0x18`, raw base `esp`, value `[[sp+4]+24]`. Count 1 static access. Sites: `push` at 0x10ccde.
  Four-byte store, pattern 17: target `sp -0x14`, raw base `esp`, value `0x0`. Count 1 static access. Sites: `push` at 0x10cce7.
  Four-byte store, pattern 18: target `sp -0x18`, raw base `esp`, value `0x1ac994`. Count 1 static access. Sites: `push` at 0x10cce9.
  Call (`call` at 0x10ccbb): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`[[sp+4]+32]`, stack arg1=`0xffffffff`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10ccdf): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`[[sp+4]+24]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10ccee): _nanosleep, target 0x17ba20. Candidate slots: stack arg0=`0x1ac994`, stack arg1=`0x0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jbe` at 0x10cc9e): target 0x10cca4. Taken when `? <= ? (unsigned, 32-bit)`. Falls through when `? > ? (unsigned, 32-bit)`.
  Jump (`jmp` at 0x10cca2): target 0x10cce7. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x10cccd): target 0x10ccd7. Taken when `[sp+12] != 0x40`. Falls through when `[sp+12] == 0x40`.
  At the exit (`ret` at 0x10ccfe), eax=`unknown`.
  Register fact (`xor` at 0x10cca0): edi=`0x0`.
  Tool warning: `0x10cc93 imul: unsupported effects invalidated`.
  Local unreached instructions: 0x10ccff to 0x10cd03, ending at 0x10cd04 (exclusive). Count 5.

- `_closeFlashWindow`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10cd04`.
  Inferred analysis window: 0x10cd04 to 0x10cdb4 (exclusive), 176 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 473, body file offset 0x4d24, window SHA256 `3ec43cdc303679a78d35974b4806b7f7edc66f58cc8b583ef426dd968bb404bc`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10cd08.
  Four-byte load, pattern 2: target `[sp+4] +0x2c`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x10cd0b, `mov` at 0x10cd20.
  Four-byte load, pattern 3: target `[[sp+4]+44] +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x10cd0e.
  Four-byte store, pattern 4: target `sp -0xc`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x10cd13, `push` at 0x10cd29.
  Four-byte load, pattern 5: target `[sp+4] +0x18`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x10cd14, `mov` at 0x10cd2a.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `[[sp+4]+24]`. Count 2 static accesses. Sites: `push` at 0x10cd17, `push` at 0x10cd2d.
  Four-byte load, pattern 7: target `[[sp+4]+44] +0x20`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x10cd23.
  Four-byte store, pattern 8: target `sp -0xc`, raw base `esp`, value `0xffffffff`. Count 1 static access. Sites: `push` at 0x10cd36.
  Four-byte load, pattern 9: target `[sp+4] +0x1c`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10cd38.
  Four-byte store, pattern 10: target `sp -0x10`, raw base `esp`, value `[[sp+4]+28]`. Count 1 static access. Sites: `push` at 0x10cd3b.
  Four-byte store, pattern 11: target `sp -0xc`, raw base `esp`, value `0x0`. Count 1 static access. Sites: `push` at 0x10cd44.
  Four-byte store, pattern 12: target `sp -0x10`, raw base `esp`, value `0x1ac994`. Count 1 static access. Sites: `push` at 0x10cd46.
  Call (`call` at 0x10cd18): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`[[sp+4]+24]`, stack arg1=`unknown`, stack arg2=`ebx`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10cd2e): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`[[sp+4]+24]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10cd3c): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`[[sp+4]+28]`, stack arg1=`0xffffffff`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10cd4b): _nanosleep, target 0x17ba20. Candidate slots: stack arg0=`0x1ac994`, stack arg1=`0x0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`ret` at 0x10cd56), eax=`ret(_nanosleep)`.
  Local unreached instructions: 0x10cd57 to 0x10cdb3, ending at 0x10cdb4 (exclusive). Count 36.

- `_closeFlash`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10d144`.
  Inferred analysis window: 0x10d144 to 0x10d164 (exclusive), 32 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 480, body file offset 0x5164, window SHA256 `bfa45193dbfb6a8cc05949bc25ae00ca3c35edbb9757b621f8f9a462249259f9`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10d148.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x10d14b.
  Four-byte store, pattern 3: target `[sp+4] +0x4`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10d151.
  Call (`call` at 0x10d14c): _closeFlashWindow, target 0x10cd04. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`ebx`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`ret` at 0x10d160), eax=`0x0`.
  Register fact (`xor` at 0x10d158): eax=`0x0`.
  Local unreached instructions: 0x10d161 to 0x10d163, ending at 0x10d164 (exclusive). Count 3.

- `_writeFlashBytes`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10d2a4`.
  Inferred analysis window: 0x10d2a4 to 0x10d394 (exclusive), 240 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 482, body file offset 0x52c4, window SHA256 `f6466211747bce317678aebc31f596185fb5437d7d963fc31b0de4af2e76e77e`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10d2ad.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `ebp`, value `0x64`. Count 1 static access. Sites: `mov` at 0x10d2b2.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `ebp`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10d2bb.
  Four-byte load, pattern 4: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x10d2c2.
  Byte store, pattern 5: target `[sp+4] +0x0`, raw base `ecx`, value `0x50`. Count 4 static accesses. Sites: `mov` at 0x10d2cc, `mov` at 0x10d2d4, `mov` at 0x10d2f4.
  Byte store, pattern 6: target `[sp+4] +0x0`, raw base `ecx`, value `0x70`. Count 2 static accesses. Sites: `mov` at 0x10d2d7, `mov` at 0x10d30b.
  Byte load, pattern 7: target `[sp+4] +0x0`, raw base `ecx`. Count 2 static accesses. Sites: `mov` at 0x10d2da, `mov` at 0x10d30e.
  Byte store, pattern 8: target `unknown`, raw base `ebx`, value `0x40`. Count 1 static access. Sites: `mov` at 0x10d2f7.
  Four-byte load, pattern 9: target `sp +0x8`, raw base `ebp`. Count 2 static accesses. Sites: `mov` at 0x10d2fa, `mov` at 0x10d349.
  Byte load, pattern 10: target `[sp+8] +0x0`, raw base `edi`. Count 2 static accesses. Sites: `mov` at 0x10d2fd, `mov` at 0x10d34c.
  Byte store, pattern 11: target `unknown`, raw base `ebx`, value `[[sp+8]]:1`. Count 1 static access. Sites: `mov` at 0x10d2ff.
  Byte store, pattern 12: target `[sp+4] +0x0`, raw base `ecx`, value `0xff`. Count 1 static access. Sites: `mov` at 0x10d344.
  Byte load, pattern 13: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x10d347.
  Four-byte load, pattern 14: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `dec` at 0x10d357.
  Four-byte store, pattern 15: target `sp -0xc`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `dec` at 0x10d357.
  Four-byte store, pattern 16: target `sp -0xc`, raw base `ebp`, value `0x2710`. Count 1 static access. Sites: `mov` at 0x10d364.
  Four-byte load, pattern 17: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `inc` at 0x10d36b.
  Four-byte store, pattern 18: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `inc` at 0x10d36b.
  Four-byte load, pattern 19: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `inc` at 0x10d36f.
  Four-byte store, pattern 20: target `sp +0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `inc` at 0x10d36f.
  Four-byte load, pattern 21: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10d372.
  Four-byte load, pattern 22: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x10d375.
  Branch (`jbe` at 0x10d2c5): target 0x10d37e. Taken when `[sp+12] <= 0x0 (unsigned, 32-bit)`. Falls through when `[sp+12] > 0x0 (unsigned, 32-bit)`.
  Branch (`jl` at 0x10d2de): target 0x10d2e6. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Branch (`jne` at 0x10d2e4): target 0x10d2d4. Taken when `? != 0xffffffff`. Falls through when `? == 0xffffffff`.
  Branch (`jne` at 0x10d2e8): target 0x10d2f4. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`jmp` at 0x10d2ef): target 0x10d37e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jl` at 0x10d312): target 0x10d31a. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Branch (`jne` at 0x10d318): target 0x10d308. Taken when `? != 0xffffffff`. Falls through when `? == 0xffffffff`.
  Branch (`jne` at 0x10d31c): target 0x10d334. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`jmp` at 0x10d323): target 0x10d37e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x10d337): target 0x10d344. Taken when `(([[sp+4]]:1 & 0xff) & 0x10) == 0x0`. Falls through when `(([[sp+4]]:1 & 0xff) & 0x10) != 0x0`.
  Jump (`jmp` at 0x10d33e): target 0x10d372. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x10d350): target 0x10d364. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`je` at 0x10d35a): target 0x10d37e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`jmp` at 0x10d35c): target 0x10d372. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jb` at 0x10d378): target 0x10d2cc. Taken when `[sp-8] < [sp+12] (unsigned, 32-bit)`. Falls through when `[sp-8] >= [sp+12] (unsigned, 32-bit)`.
  At the exit (`ret` at 0x10d389), eax=`unknown`.
  Register fact (`xor` at 0x10d2b0): esi=`0x0`.
  Register fact (`mov` at 0x10d2cf): eax=`0x270f`.
  Register fact (`mov` at 0x10d2ea): esi=`0xfffffff6`.
  Register fact (`mov` at 0x10d301): eax=`0x270f`.
  Register fact (`mov` at 0x10d31e): esi=`0xfffffff5`.
  Register fact (`mov` at 0x10d339): esi=`0xfffffff4`.
  Register fact (`mov` at 0x10d352): esi=`0xfffffff3`.
  Tool warning: `0x10d2e0 dec: unsupported effects invalidated`.
  Tool warning: `0x10d314 dec: unsupported effects invalidated`.
  Tool warning: `0x10d357 dec: unsupported effects invalidated`.
  Tool warning: `0x10d36b inc: unsupported effects invalidated`.
  Tool warning: `0x10d36e inc: unsupported effects invalidated`.
  Tool warning: `0x10d36f inc: unsupported effects invalidated`.
  Local unreached instructions: 0x10d325 to 0x10d333, ending at 0x10d334 (exclusive). Count 15.
  Local unreached instructions: 0x10d340 to 0x10d343, ending at 0x10d344 (exclusive). Count 4.
  Local unreached instructions: 0x10d35e to 0x10d363, ending at 0x10d364 (exclusive). Count 6.
  Local unreached instructions: 0x10d38a to 0x10d393, ending at 0x10d394 (exclusive). Count 10.

- `_nvramWrite`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127288`.
  Inferred analysis window: 0x127288 to 0x127348 (exclusive), 192 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 684, body file offset 0x1f2a8, window SHA256 `1cb59c365893eef61e29722564405dd00a31787b3a2144a8d57f0d834793ac0d`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12728d.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x127290.
  Four-byte load, pattern 3: target `unknown`, raw base `ecx`. Count 1 static access. Sites: `cmp` at 0x1272a4.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `esp`, value `[sp+4]`. Count 3 static accesses. Sites: `push` at 0x1272ab, `push` at 0x127308, `push` at 0x127315.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `0x127201`. Count 1 static access. Sites: `push` at 0x1272ac.
  Four-byte load, pattern 6: target `abs 0x1dd058`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x1272b8.
  Four-byte store, pattern 7: target `sp -0x10`, raw base `esp`, value `0x127229`. Count 1 static access. Sites: `push` at 0x1272c4.
  Four-byte load, pattern 8: target `unknown`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x1272d8.
  Four-byte store, pattern 9: target `sp -0x10`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x1272de.
  Four-byte load, pattern 10: target `unknown`, raw base `ecx`. Count 1 static access. Sites: `mov` at 0x1272df.
  Four-byte store, pattern 11: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x1272e3.
  Four-byte store, pattern 12: target `sp -0x18`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x1272e4.
  Four-byte store, pattern 13: target `sp -0x10`, raw base `esp`, value `ret(_sram_write)`. Count 1 static access. Sites: `push` at 0x1272f2.
  Four-byte store, pattern 14: target `sp -0x14`, raw base `esp`, value `0x127259`. Count 1 static access. Sites: `push` at 0x1272f3.
  Call (`call` at 0x1272c9): _ffscMsg, target 0x11f0d0. Candidate slots: stack arg0=`0x127229`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1272e5): _sram_write, target 0x10ca78. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`ebp`.
  Call (`call` at 0x1272f8): _ffscMsg, target 0x11f0d0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x127309): _nvramIsValid, target 0x127368. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x127316): _nvramValidate, target 0x127798. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12731e): _nvramWriteHeader, target 0x1277f8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`ja` at 0x127296): target 0x1272ab. Taken when `[sp+4] > 0xc (unsigned, 32-bit)`. Falls through when `[sp+4] <= 0xc (unsigned, 32-bit)`.
  Branch (`jne` at 0x1272a9): target 0x1272b8. Taken when `? != 0x0`. Falls through when `? == 0x0`.
  Jump (`jmp` at 0x1272b1): target 0x1272f8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x1272c2): target 0x1272d8. Taken when `[0x1dd058] == 0xff5cff5c`. Falls through when `[0x1dd058] != 0xff5cff5c`.
  Jump (`jmp` at 0x1272d3): target 0x12733a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x1272f0): target 0x127308. Taken when `ret(_sram_write) == 0x1`. Falls through when `ret(_sram_write) != 0x1`.
  Jump (`jmp` at 0x127302): target 0x12733a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x127313): target 0x127338. Taken when `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0`. Falls through when `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0`.
  Branch (`je` at 0x127325): target 0x127338. Taken when `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) == 0x0`. Falls through when `(ret(_nvramWriteHeader) & ret(_nvramWriteHeader)) != 0x0`.
  Jump (`jmp` at 0x12732c): target 0x12733a. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x127342), eax=`unknown`.
  Register fact (`mov` at 0x12729f): ecx=`0x1dc750`.
  Register fact (`mov` at 0x1272ce): eax=`0xffffffff`.
  Register fact (`mov` at 0x1272fd): eax=`0xffffffff`.
  Register fact (`mov` at 0x127327): eax=`0xffffffff`.
  Register fact (`xor` at 0x127338): eax=`0x0`.
  Local unreached instructions: 0x1272b3 to 0x1272b7, ending at 0x1272b8 (exclusive). Count 5.
  Local unreached instructions: 0x1272d5 to 0x1272d7, ending at 0x1272d8 (exclusive). Count 3.
  Local unreached instructions: 0x127304 to 0x127307, ending at 0x127308 (exclusive). Count 4.
  Local unreached instructions: 0x12732e to 0x127337, ending at 0x127338 (exclusive). Count 10.
  Local unreached instructions: 0x127343 to 0x127347, ending at 0x127348 (exclusive). Count 5.

- `_nvramRead`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1270f8`.
  Inferred analysis window: 0x1270f8 to 0x127198 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 809, body file offset 0x1f118, window SHA256 `600ea8bb337c55f915f5645e23667b281e1b6f065096960ea463f2cdb9894eba`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1270fe.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x127101.
  Four-byte load, pattern 3: target `unknown`, raw base `esi`. Count 1 static access. Sites: `cmp` at 0x127115.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 2 static accesses. Sites: `push` at 0x12711c, `push` at 0x127148.
  Four-byte store, pattern 5: target `sp -0x18`, raw base `esp`, value `0x127076`. Count 1 static access. Sites: `push` at 0x12711d.
  Four-byte load, pattern 6: target `abs 0x1dd058`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x127128.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `0x12709d`. Count 1 static access. Sites: `push` at 0x127134.
  Four-byte load, pattern 8: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x127155.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12715b.
  Four-byte load, pattern 10: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12715c.
  Four-byte store, pattern 11: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x127160.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x127161.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `esp`, value `ret(_sram_read)`. Count 1 static access. Sites: `push` at 0x127178.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `esp`, value `0x1270cc`. Count 1 static access. Sites: `push` at 0x127179.
  Call (`call` at 0x127139): _ffscMsg, target 0x11f0d0. Candidate slots: stack arg0=`0x12709d`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x127149): _nvramIsValid, target 0x127368. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x127162): _sram_read, target 0x10c998. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12717e): _ffscMsg, target 0x11f0d0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`ja` at 0x127107): target 0x12711c. Taken when `[sp+4] > 0xc (unsigned, 32-bit)`. Falls through when `[sp+4] <= 0xc (unsigned, 32-bit)`.
  Branch (`jne` at 0x12711a): target 0x127128. Taken when `? != 0x0`. Falls through when `? == 0x0`.
  Jump (`jmp` at 0x127122): target 0x12717e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x127132): target 0x127148. Taken when `[0x1dd058] == 0xff5cff5c`. Falls through when `[0x1dd058] != 0xff5cff5c`.
  Jump (`jmp` at 0x12713e): target 0x127183. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x127153): target 0x127183. Taken when `(ret(_nvramIsValid) & ret(_nvramIsValid)) == 0x0`. Falls through when `(ret(_nvramIsValid) & ret(_nvramIsValid)) != 0x0`.
  Branch (`jne` at 0x12716d): target 0x127178. Taken when `ret(_sram_read) != 0x1`. Falls through when `ret(_sram_read) == 0x1`.
  Jump (`jmp` at 0x127171): target 0x127188. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x127191), eax=`unknown`.
  Register fact (`mov` at 0x127110): esi=`0x1dc750`.
  Register fact (`xor` at 0x12716f): eax=`0x0`.
  Register fact (`mov` at 0x127183): eax=`0xffffffff`.
  Local unreached instructions: 0x127124 to 0x127127, ending at 0x127128 (exclusive). Count 4.
  Local unreached instructions: 0x127140 to 0x127147, ending at 0x127148 (exclusive). Count 8.
  Local unreached instructions: 0x127173 to 0x127177, ending at 0x127178 (exclusive). Count 5.
  Local unreached instructions: 0x127192 to 0x127197, ending at 0x127198 (exclusive). Count 6.

- `_nvramWriteHeader`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1277f8`.
  Inferred analysis window: 0x1277f8 to 0x12782c (exclusive), 52 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 991, body file offset 0x1f818, window SHA256 `1f3e6f8d6ff5468321c1aee808cb98dc7f8d5b810e2aa089edb522692fb4f966`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `esp`, value `0x0`. Count 1 static access. Sites: `push` at 0x1277fb.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x40`. Count 1 static access. Sites: `push` at 0x1277fd.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `0x1dd058`. Count 1 static access. Sites: `push` at 0x1277ff.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `ret(_sram_write)`. Count 1 static access. Sites: `push` at 0x127818.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x1277ca`. Count 1 static access. Sites: `push` at 0x127819.
  Call (`call` at 0x127804): _sram_write, target 0x10ca78. Candidate slots: stack arg0=`0x1dd058`, stack arg1=`0x40`, stack arg2=`0x0`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12781e): _ffscMsg, target 0x11f0d0. Candidate slots: stack arg0=`0x1277ca`, stack arg1=`ret(_sram_write)`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x12780f): target 0x127818. Taken when `ret(_sram_write) != 0x1`. Falls through when `ret(_sram_write) == 0x1`.
  At the exit (`ret` at 0x127816), eax=`0x0`.
  At the exit (`ret` at 0x12782b), eax=`0xffffffff`.
  Register fact (`xor` at 0x127811): eax=`0x0`.
  Register fact (`mov` at 0x127823): eax=`0xffffffff`.
  Local unreached instructions: 0x127817 to 0x127817, ending at 0x127818 (exclusive). Count 1.

- `_nvramZap`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127348`.
  Inferred analysis window: 0x127348 to 0x127368 (exclusive), 32 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 923, body file offset 0x1f368, window SHA256 `2054ef39dcc1fa7d7ad06ed483d8658bf465927d0b6799044c398d36ac905253`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `esp`, value `0x40`. Count 1 static access. Sites: `push` at 0x12734b.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x1dd058`. Count 1 static access. Sites: `push` at 0x12734d.
  Call (`call` at 0x127352): _bzero, target 0x172778. Candidate slots: stack arg0=`0x1dd058`, stack arg1=`0x40`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12735a): _nvramWriteHeader, target 0x1277f8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`ret` at 0x127362), eax=`ret(_nvramWriteHeader)`.
  Local unreached instructions: 0x127363 to 0x127367, ending at 0x127368 (exclusive). Count 5.

- `_nvramIsValid`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127368`.
  Inferred analysis window: 0x127368 to 0x127398 (exclusive), 48 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 993, body file offset 0x1f388, window SHA256 `f0e57f389074b931949d63b3f7e452265d5f9757a75d85e11cd6fd2f7f4c629d`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12736b.
  Four-byte load, pattern 2: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x127376.
  At the exit (`ret` at 0x12738b), eax=`unknown`.
  Tool warning: `0x12737d bt: unsupported effects invalidated`.
  Tool warning: `0x127380 setb: unsupported effects invalidated`.
  Local unreached instructions: 0x12738c to 0x127397, ending at 0x127398 (exclusive). Count 12.

- `_nvramValidate`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127798`.
  Inferred analysis window: 0x127798 to 0x1277f8 (exclusive), 96 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 994, body file offset 0x1f7b8, window SHA256 `e0e4bfcb706a877ffea815e35f95addc7237d1a19e178eaa403260b05c09adcd`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12779f.
  Four-byte load, pattern 2: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x1277b3.
  Four-byte store, pattern 3: target `unknown`, raw base `unknown`, value `unknown`. Count 1 static access. Sites: `mov` at 0x1277bc.
  At the exit (`ret` at 0x1277c9), eax=`unknown`.
  Register fact (`mov` at 0x1277ac): eax=`0x1`.
  Local unreached instructions: 0x1277ca to 0x1277f7, ending at 0x1277f8 (exclusive). Count 19.

- `_sysIntInitPIC`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1082f8`.
  Inferred analysis window: 0x1082f8 to 0x108398 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 64, body file offset 0x318, window SHA256 `3cadbad89ff853f66cbb2c79bf8c69c3aff433ff4debc52097550d2177be64b3`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `esp`, value `0x11`. Count 2 static accesses. Sites: `push` at 0x1082fb, `push` at 0x108331.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x20`. Count 1 static access. Sites: `push` at 0x1082fd.
  Byte load, pattern 3: target `abs 0x1ac0a8`, raw base `unknown`. Count 1 static access. Sites: `movsx` at 0x108307.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x10830e, `push` at 0x10834b.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x21`. Count 4 static accesses. Sites: `push` at 0x10830f, `push` at 0x10831b, `push` at 0x108327.
  Four-byte store, pattern 6: target `sp -0x8`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x108319.
  Four-byte store, pattern 7: target `sp -0x8`, raw base `esp`, value `0x1`. Count 2 static accesses. Sites: `push` at 0x108325, `push` at 0x108368.
  Four-byte store, pattern 8: target `sp -0xc`, raw base `esp`, value `0xa0`. Count 1 static access. Sites: `push` at 0x108333.
  Byte load, pattern 9: target `abs 0x1ac0a8`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x108340.
  Four-byte store, pattern 10: target `sp -0xc`, raw base `esp`, value `0xa1`. Count 4 static accesses. Sites: `push` at 0x10834c, `push` at 0x10835b, `push` at 0x10836a.
  Four-byte store, pattern 11: target `sp -0x8`, raw base `esp`, value `0x2`. Count 1 static access. Sites: `push` at 0x108359.
  Four-byte store, pattern 12: target `sp -0x8`, raw base `esp`, value `0xfffffffb`. Count 1 static access. Sites: `push` at 0x108377.
  Four-byte store, pattern 13: target `sp -0x8`, raw base `esp`, value `0xffffffff`. Count 1 static access. Sites: `push` at 0x108383.
  Call (`call` at 0x1082ff): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0x20`, stack arg1=`0x11`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x108311): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0x21`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10831d): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0x21`, stack arg1=`0x4`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x108329): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0x21`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x108338): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0xa0`, stack arg1=`0x11`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x108351): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0xa1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x108360): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0xa1`, stack arg1=`0x2`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10836f): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0xa1`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10837b): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0x21`, stack arg1=`0xfffffffb`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10838a): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`0xa1`, stack arg1=`0xffffffff`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`ret` at 0x108392), eax=`ret(_sysOutByte)`.
  Local unreached instructions: 0x108393 to 0x108397, ending at 0x108398 (exclusive). Count 5.
  Predecessor context at 0x1082d0 stopped decoding early. Bytes 0x1082d8 to 0x1082f8 (exclusive) remain raw and open.

- `_sysIntDisablePIC`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1083b8`.
  Inferred analysis window: 0x1083b8 to 0x108428 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 66, body file offset 0x3d8, window SHA256 `69b2626f718d4a1cebdf502e70569eff1c784da7bc8f1e1235a324d5346e9041`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1083bc.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x21`. Count 1 static access. Sites: `push` at 0x1083c4.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x1083de, `push` at 0x108406.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `esp`, value `0x21`. Count 1 static access. Sites: `push` at 0x1083df.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0xa1`. Count 1 static access. Sites: `push` at 0x1083e8.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `0xa1`. Count 1 static access. Sites: `push` at 0x108407.
  Call (`call` at 0x1083c6): _sysInByte, target 0x1081b0. Candidate slots: stack arg0=`0x21`, stack arg1=`ebx`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1083ed): _sysInByte, target 0x1081b0. Candidate slots: stack arg0=`0xa1`, stack arg1=`ebx`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10840c): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jg` at 0x1083c2): target 0x1083e8. Taken when `[sp+4] > 0x7 (signed, 32-bit)`. Falls through when `[sp+4] <= 0x7 (signed, 32-bit)`.
  Jump (`jmp` at 0x1083e1): target 0x10840c. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x108419), eax=`0x0`.
  Register fact (`mov` at 0x1083d0): eax=`0x1`.
  Register fact (`mov` at 0x1083fa): eax=`0x1`.
  Register fact (`xor` at 0x108411): eax=`0x0`.
  Local unreached instructions: 0x1083e3 to 0x1083e7, ending at 0x1083e8 (exclusive). Count 5.
  Local unreached instructions: 0x10841a to 0x108427, ending at 0x108428 (exclusive). Count 14.

- `_sysIntEnablePIC`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x108428`.
  Inferred analysis window: 0x108428 to 0x108498 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 67, body file offset 0x448, window SHA256 `d7af03b5f5ab6d63c74d69a4c6c0389fa74b98a6b69c1a7b1ceeaaded0cdb76a`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10842c.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x21`. Count 1 static access. Sites: `push` at 0x108434.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x10844e, `push` at 0x108476.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `esp`, value `0x21`. Count 1 static access. Sites: `push` at 0x10844f.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0xa1`. Count 1 static access. Sites: `push` at 0x108458.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `0xa1`. Count 1 static access. Sites: `push` at 0x108477.
  Call (`call` at 0x108436): _sysInByte, target 0x1081b0. Candidate slots: stack arg0=`0x21`, stack arg1=`ebx`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10845d): _sysInByte, target 0x1081b0. Candidate slots: stack arg0=`0xa1`, stack arg1=`ebx`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10847c): _sysOutByte, target 0x1081d0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jg` at 0x108432): target 0x108458. Taken when `[sp+4] > 0x7 (signed, 32-bit)`. Falls through when `[sp+4] <= 0x7 (signed, 32-bit)`.
  Jump (`jmp` at 0x108451): target 0x10847c. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x108489), eax=`0x0`.
  Register fact (`mov` at 0x108440): eax=`0xfffffffe`.
  Register fact (`mov` at 0x10846a): eax=`0xfffffffe`.
  Register fact (`xor` at 0x108481): eax=`0x0`.
  Tool warning: `0x108447 rol: unsupported effects invalidated`.
  Tool warning: `0x10846f rol: unsupported effects invalidated`.
  Local unreached instructions: 0x108453 to 0x108457, ending at 0x108458 (exclusive). Count 5.
  Local unreached instructions: 0x10848a to 0x108497, ending at 0x108498 (exclusive). Count 14.

- `_ttyDevCreate`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1937d4`.
  Inferred analysis window: 0x1937d4 to 0x1938a4 (exclusive), 208 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 1070, body file offset 0x8b7f4, window SHA256 `c291bb42ccccd807c3ee243374788003013c078d22d536e63f5dd745b528dcc1`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1937da.
  Four-byte load, pattern 2: target `sp +0x10`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1937dd.
  Four-byte load, pattern 3: target `abs 0x1dbd84`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x1937e0.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `0xc0001`. Count 1 static access. Sites: `push` at 0x1937e9.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `0xe4`. Count 1 static access. Sites: `push` at 0x193804.
  Four-byte store, pattern 6: target `sp -0x14`, raw base `esp`, value `0x193904`. Count 1 static access. Sites: `push` at 0x193817.
  Four-byte store, pattern 7: target `sp -0x18`, raw base `esp`, value `[sp+16]`. Count 1 static access. Sites: `push` at 0x19381c.
  Four-byte load, pattern 8: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x19381d.
  Four-byte store, pattern 9: target `sp -0x1c`, raw base `esp`, value `[sp+12]`. Count 1 static access. Sites: `push` at 0x193820.
  Four-byte store, pattern 10: target `sp -0x20`, raw base `esp`, value `ret(_malloc)`. Count 1 static access. Sites: `push` at 0x193821.
  Four-byte store, pattern 11: target `ret(_malloc) +0xe0`, raw base `ebx`, value `[sp+8]`. Count 1 static access. Sites: `mov` at 0x19382e.
  Four-byte load, pattern 12: target `[sp+8] +0x0`, raw base `esi`. Count 3 static accesses. Sites: `mov` at 0x193834, `mov` at 0x193847, `mov` at 0x19385a.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `esp`, value `ret(_malloc)`. Count 3 static accesses. Sites: `push` at 0x193836, `push` at 0x193849, `push` at 0x193884.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `esp`, value `0x16fe48`. Count 1 static access. Sites: `push` at 0x193837.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `esp`, value `0x1`. Count 1 static access. Sites: `push` at 0x19383c.
  Four-byte store, pattern 16: target `sp -0x20`, raw base `esp`, value `[sp+8]`. Count 2 static accesses. Sites: `push` at 0x19383e, `push` at 0x193851.
  Four-byte load, pattern 17: target `[[sp+8]] +0x8`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x19383f, `mov` at 0x193852.
  Four-byte store, pattern 18: target `sp -0x18`, raw base `esp`, value `0x16ff88`. Count 1 static access. Sites: `push` at 0x19384a.
  Four-byte store, pattern 19: target `sp -0x1c`, raw base `esp`, value `0x2`. Count 1 static access. Sites: `push` at 0x19384f.
  Four-byte store, pattern 20: target `sp -0x14`, raw base `esp`, value `0x2`. Count 1 static access. Sites: `push` at 0x19385c.
  Four-byte store, pattern 21: target `sp -0x18`, raw base `esp`, value `0x1007`. Count 1 static access. Sites: `push` at 0x19385e.
  Four-byte store, pattern 22: target `sp -0x1c`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x193863.
  Four-byte load, pattern 23: target `[[sp+8]] +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x193864.
  Four-byte load, pattern 24: target `abs 0x1dbd84`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x19386b.
  Four-byte store, pattern 25: target `sp -0x14`, raw base `esp`, value `[0x1dbd84]`. Count 1 static access. Sites: `push` at 0x193870.
  Four-byte load, pattern 26: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x193871.
  Four-byte store, pattern 27: target `sp -0x18`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x193874.
  Four-byte store, pattern 28: target `sp -0x1c`, raw base `esp`, value `ret(_malloc)`. Count 1 static access. Sites: `push` at 0x193875.
  Call (`call` at 0x1937ee): _errnoSet, target 0x171340. Candidate slots: stack arg0=`0xc0001`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x193809): _malloc, target 0x17ed28. Candidate slots: stack arg0=`0xe4`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x193822): _tyDevInit, target 0x16f5f8. Candidate slots: stack arg0=`ret(_malloc)`, stack arg1=`[sp+12]`, stack arg2=`[sp+16]`, stack arg3=`0x193904`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x193842): (unknown callee), target unknown. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`0x1`, stack arg2=`0x16fe48`, stack arg3=`ret(_malloc)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x193855): (unknown callee), target unknown. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`0x2`, stack arg2=`0x16ff88`, stack arg3=`ret(_malloc)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x193866): (unknown callee), target unknown. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`0x1007`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x193876): _iosDevAdd, target 0x170a68. Candidate slots: stack arg0=`ret(_malloc)`, stack arg1=`[sp+4]`, stack arg2=`[0x1dbd84]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x193885): _free, target 0x17ed48. Candidate slots: stack arg0=`ret(_malloc)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jg` at 0x1937e7): target 0x193804. Taken when `[0x1dbd84] > 0x0 (signed, 32-bit)`. Falls through when `[0x1dbd84] <= 0x0 (signed, 32-bit)`.
  Jump (`jmp` at 0x1937f3): target 0x19388a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x193815): target 0x19388a. Taken when `(ret(_malloc) & ret(_malloc)) == 0x0`. Falls through when `(ret(_malloc) & ret(_malloc)) != 0x0`.
  Branch (`jne` at 0x19382c): target 0x193884. Taken when `(ret(_tyDevInit) & ret(_tyDevInit)) != 0x0`. Falls through when `(ret(_tyDevInit) & ret(_tyDevInit)) == 0x0`.
  Jump (`jmp` at 0x19387b): target 0x19388f. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x193898), eax=`unknown`.
  Register fact (`mov` at 0x19388a): eax=`0xffffffff`.
  Local unreached instructions: 0x1937f8 to 0x193803, ending at 0x193804 (exclusive). Count 12.
  Local unreached instructions: 0x19387d to 0x193883, ending at 0x193884 (exclusive). Count 7.
  Local unreached instructions: 0x193899 to 0x1938a3, ending at 0x1938a4 (exclusive). Count 11.
