# MMSC serial flow and buffer-path observations

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

The [full facts](disasm/serial-flow-buffer-paths-6.5.6-observations-facts.txt) retain every access, instruction and warning.

- `_comSetFlowCntl`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e1a8`.
  Inferred analysis window: 0x12e1a8 to 0x12e2f8 (exclusive), 336 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 702, body file offset 0x261c8, window SHA256 `1dabc665e57a020574727cc75b14dacbac1bfbdb7cf7db63c86f7cb4320b88db`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e1b0.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e1b3.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `esp`, value `sp-8`. Count 1 static access. Sites: `push` at 0x12e1b9.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `esp`, value `0x1006`. Count 1 static access. Sites: `push` at 0x12e1ba.
  Four-byte load, pattern 5: target `unknown`, raw base `unknown`. Count 6 static accesses. Sites: `mov` at 0x12e1bf, `mov` at 0x12e1d3, `mov` at 0x12e22a.
  Four-byte store, pattern 6: target `sp -0x1c`, raw base `esp`, value `unknown`. Count 4 static accesses. Sites: `push` at 0x12e1c6, `push` at 0x12e231, `push` at 0x12e249.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `unknown`. Count 3 static accesses. Sites: `push` at 0x12e1da, `push` at 0x12e26f, `push` at 0x12e29f.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `0x12e14a`. Count 1 static access. Sites: `push` at 0x12e1db.
  Four-byte load, pattern 9: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `and` at 0x12e1ec.
  Four-byte store, pattern 10: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `and` at 0x12e1ec.
  Byte load, pattern 11: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `or` at 0x12e1f0.
  Byte store, pattern 12: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `or` at 0x12e1f0.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `esp`, value `0x800`. Count 2 static accesses. Sites: `push` at 0x12e1f4, `push` at 0x12e210.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `esp`, value `[sp+4]+1`. Count 2 static accesses. Sites: `push` at 0x12e1fc, `push` at 0x12e218.
  Byte load, pattern 15: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `or` at 0x12e208.
  Byte store, pattern 16: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `or` at 0x12e208.
  Four-byte load, pattern 17: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `and` at 0x12e20c.
  Four-byte store, pattern 18: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `and` at 0x12e20c.
  Four-byte load, pattern 19: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e221.
  Four-byte store, pattern 20: target `sp -0x14`, raw base `esp`, value `[sp-8]`. Count 1 static access. Sites: `push` at 0x12e224.
  Four-byte store, pattern 21: target `sp -0x18`, raw base `esp`, value `0x1005`. Count 1 static access. Sites: `push` at 0x12e225.
  Four-byte store, pattern 22: target `sp -0x14`, raw base `esp`, value `0x0`. Count 1 static access. Sites: `push` at 0x12e23e.
  Four-byte store, pattern 23: target `sp -0x18`, raw base `esp`, value `0x13`. Count 1 static access. Sites: `push` at 0x12e240.
  Four-byte store, pattern 24: target `sp -0x18`, raw base `esp`, value `0x3`. Count 1 static access. Sites: `push` at 0x12e270.
  Four-byte store, pattern 25: target `sp -0x18`, raw base `esp`, value `0x12e176`. Count 1 static access. Sites: `push` at 0x12e2a0.
  Call (`call` at 0x12e1c7): _ioctl, target 0x18d1a8. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1006`, stack arg2=`sp-8`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`unknown`.
  Call (`call` at 0x12e1fd): _comSetFlag, target 0x12e0e8. Candidate slots: stack arg0=`[sp+4]+1`, stack arg1=`0x800`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e219): _comResetFlag, target 0x12e078. Candidate slots: stack arg0=`[sp+4]+1`, stack arg1=`0x800`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e232): _ioctl, target 0x18d1a8. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1005`, stack arg2=`[sp-8]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e24a): _ioctl, target 0x18d1a8. Candidate slots: stack arg0=`unknown`, stack arg1=`0x13`, stack arg2=`0x0`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e27a): _ioctl, target 0x18d1a8. Candidate slots: stack arg0=`unknown`, stack arg1=`0x3`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e2a5): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x12e1d1): target 0x12e1e8. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Jump (`jmp` at 0x12e1e0): target 0x12e2a5. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12e1ea): target 0x12e208. Taken when `([sp+8] & [sp+8]) == 0x0`. Falls through when `([sp+8] & [sp+8]) != 0x0`.
  Jump (`jmp` at 0x12e202): target 0x12e21e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jl` at 0x12e23c): target 0x12e298. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Branch (`je` at 0x12e254): target 0x12e268. Taken when `([sp+8] & [sp+8]) == 0x0`. Falls through when `([sp+8] & [sp+8]) != 0x0`.
  Jump (`jmp` at 0x12e258): target 0x12e26a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jl` at 0x12e284): target 0x12e298. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Jump (`jmp` at 0x12e288): target 0x12e2af. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x12e2b7), eax=`unknown`.
  Register fact (`xor` at 0x12e286): eax=`0x0`.
  Register fact (`mov` at 0x12e2aa): eax=`0xffffffff`.
  Tool warning: `decoder stopped at 0x12e2f4; remaining bytes are not instructions`.
  Local unreached instructions: 0x12e1e5 to 0x12e1e7, ending at 0x12e1e8 (exclusive). Count 3.
  Local unreached instructions: 0x12e204 to 0x12e207, ending at 0x12e208 (exclusive). Count 4.
  Local unreached instructions: 0x12e25a to 0x12e267, ending at 0x12e268 (exclusive). Count 14.
  Local unreached instructions: 0x12e28a to 0x12e297, ending at 0x12e298 (exclusive). Count 14.
  Local unreached instructions: 0x12e2b8 to 0x12e2f3, ending at 0x12e2f4 (exclusive). Count 28.
  Raw symbol tail: 0x12e2f4 to 0x12e2f8 (exclusive), bytes `00909090`, with no instruction or effect credit.

- `_comSetSpeed`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e3f8`.
  Inferred analysis window: 0x12e3f8 to 0x12e468 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 699, body file offset 0x26418, window SHA256 `19b550228b67ce74a6e14c179d73cacec634c9b2c4f545ee7786b69f1ac6ea62`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e3fe.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e401.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x12e407.
  Four-byte load, pattern 4: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12e408.
  Four-byte store, pattern 5: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12e40f.
  Four-byte store, pattern 6: target `unknown`, raw base `eax`, value `[sp+8]`. Count 1 static access. Sites: `mov` at 0x12e421.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `0x1e1048`. Count 1 static access. Sites: `push` at 0x12e427.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x12e42c.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12e448.
  Four-byte store, pattern 10: target `sp -0x18`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x12e449.
  Four-byte store, pattern 11: target `sp -0x1c`, raw base `esp`, value `0x12e3d1`. Count 1 static access. Sites: `push` at 0x12e44a.
  Call (`call` at 0x12e410): _ttySetSpeed, target 0x12e938. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`ebx`, stack arg3=`esi`, stack arg4=`edi`, stack arg5=`ebp`.
  Call (`call` at 0x12e42e): _nvramWrite, target 0x129b50. Candidate slots: stack arg0=`0x4`, stack arg1=`0x1e1048`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e44f): _ffscMsgS, target 0x12167c. Candidate slots: stack arg0=`0x12e3d1`, stack arg1=`[sp+8]`, stack arg2=`[sp+4]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x12e41a): target 0x12e448. Taken when `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) != 0x0`. Falls through when `(ret(_ttySetSpeed) & ret(_ttySetSpeed)) == 0x0`.
  Branch (`jne` at 0x12e435): target 0x12e454. Taken when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0`. Falls through when `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0`.
  Jump (`jmp` at 0x12e439): target 0x12e459. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x12e462), eax=`unknown`.
  Register fact (`xor` at 0x12e437): eax=`0x0`.
  Register fact (`mov` at 0x12e454): eax=`0xffffffff`.
  Local unreached instructions: 0x12e43b to 0x12e447, ending at 0x12e448 (exclusive). Count 13.
  Local unreached instructions: 0x12e463 to 0x12e467, ending at 0x12e468 (exclusive). Count 5.

- `_comSetFunction`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e2f8`.
  Inferred analysis window: 0x12e2f8 to 0x12e398 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 698, body file offset 0x26318, window SHA256 `e1631ffdfc672bacbe283506e1dcdf335f50273832de359a882cdba92ff6b3e2`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e301.
  Two-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e304.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `ebp`, value `[sp+4]-1`. Count 1 static access. Sites: `mov` at 0x12e30b.
  Two-byte store, pattern 4: target `unknown`, raw base `eax`, value `[sp+8]:2`. Count 1 static access. Sites: `mov` at 0x12e313.
  Byte load, pattern 5: target `abs 0x1e0bf0`, raw base `unknown`. Count 2 static accesses. Sites: `test` at 0x12e31a, `test` at 0x12e354.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x12e326, `push` at 0x12e360.
  Four-byte store, pattern 7: target `sp -0x1c`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12e327.
  Four-byte store, pattern 8: target `sp -0x20`, raw base `esp`, value `0x12e2b8`. Count 1 static access. Sites: `push` at 0x12e328.
  Two-byte load, pattern 9: target `unknown`, raw base `edi`. Count 1 static access. Sites: `cmp` at 0x12e341.
  Four-byte load, pattern 10: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x12e348.
  Two-byte store, pattern 11: target `unknown`, raw base `edi`, value `0xffff`. Count 1 static access. Sites: `mov` at 0x12e34d.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `esp`, value `0x12e2d4`. Count 1 static access. Sites: `push` at 0x12e361.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `esp`, value `0x1e1048`. Count 1 static access. Sites: `push` at 0x12e374.
  Four-byte store, pattern 14: target `sp -0x1c`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x12e379.
  Call (`call` at 0x12e32d): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x12e2b8`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e366): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x12e2d4`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e37b): _nvramWrite, target 0x129b50. Candidate slots: stack arg0=`0x4`, stack arg1=`0x1e1048`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12e321): target 0x12e335. Taken when `(([0x1e0bf0]:1 & 0xff) & 0x4) == 0x0`. Falls through when `(([0x1e0bf0]:1 & 0xff) & 0x4) != 0x0`.
  Branch (`jne` at 0x12e346): target 0x12e36e. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`je` at 0x12e34b): target 0x12e36e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`je` at 0x12e35b): target 0x12e36e. Taken when `(([0x1e0bf0]:1 & 0xff) & 0x4) == 0x0`. Falls through when `(([0x1e0bf0]:1 & 0xff) & 0x4) != 0x0`.
  Branch (`jle` at 0x12e372): target 0x12e33c. Taken when `? <= 0x5 (signed, 32-bit)`. Falls through when `? > 0x5 (signed, 32-bit)`.
  At the exit (`ret` at 0x12e38e), eax=`0x1`.
  Register fact (`xor` at 0x12e335): ebx=`0x0`.
  Register fact (`mov` at 0x12e337): edi=`0x1e1048`.
  Register fact (`mov` at 0x12e380): eax=`0x1`.
  Tool warning: `0x12e36e inc: unsupported effects invalidated`.
  Local unreached instructions: 0x12e38f to 0x12e397, ending at 0x12e398 (exclusive). Count 9.
  Encoding (`mov` at 0x12e304): bytes `668b750c`, raw prefix `0x66`.
  Encoding (`mov` at 0x12e313): bytes `6689b04e101e00`, raw prefix `0x66`.
  Encoding (`cmp` at 0x12e341): bytes `6639740706`, raw prefix `0x66`.
  Encoding (`mov` at 0x12e34d): bytes `66c7440706ffff`, raw prefix `0x66`.

- `_comSetFlag`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e0e8`.
  Inferred analysis window: 0x12e0e8 to 0x12e1a8 (exclusive), 192 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 696, body file offset 0x26108, window SHA256 `f73e0581d0001647e43a870fdcbf973af38913315c69120e98b55d30eaf5b3b3`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e0ee.
  Two-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e0f5.
  Two-byte load, pattern 3: target `unknown`, raw base `eax`. Count 1 static access. Sites: `or` at 0x12e0f9.
  Two-byte store, pattern 4: target `unknown`, raw base `eax`, value `unknown`. Count 1 static access. Sites: `or` at 0x12e0f9.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `0x1e1048`. Count 1 static access. Sites: `push` at 0x12e100.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x12e105.
  Four-byte load, pattern 7: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12e118.
  Four-byte store, pattern 8: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12e11f.
  Call (`call` at 0x12e107): _nvramWrite, target 0x129b50. Candidate slots: stack arg0=`0x4`, stack arg1=`0x1e1048`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e120): _cmCheckTTYFlags, target 0x11c0a8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x12e134): target 0x12e118. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  At the exit (`ret` at 0x12e149), eax=`unknown`.
  Register fact (`xor` at 0x12e10f): edi=`0x0`.
  Register fact (`xor` at 0x12e111): esi=`0x0`.
  Register fact (`mov` at 0x12e113): ebx=`0x1f1cc8`.
  Tool warning: `0x12e0f1 dec: unsupported effects invalidated`.
  Tool warning: `0x12e12d inc: unsupported effects invalidated`.
  Tool warning: `0x12e138 setle: unsupported effects invalidated`.
  Local unreached instructions: 0x12e14a to 0x12e1a7, ending at 0x12e1a8 (exclusive). Count 42.
  Encoding (`mov` at 0x12e0f5): bytes `668b550c`, raw prefix `0x66`.
  Encoding (`or` at 0x12e0f9): bytes `6609904c101e00`, raw prefix `0x66`.

- `_comResetFlag`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e078`.
  Inferred analysis window: 0x12e078 to 0x12e0e8 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 697, body file offset 0x26098, window SHA256 `e47345b5f2c433447e84170ce2d2def7321830d833d49496f0f8ecaf543a50e0`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e07e.
  Two-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e085.
  Two-byte load, pattern 3: target `unknown`, raw base `eax`. Count 1 static access. Sites: `and` at 0x12e08c.
  Two-byte store, pattern 4: target `unknown`, raw base `eax`, value `unknown`. Count 1 static access. Sites: `and` at 0x12e08c.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `0x1e1048`. Count 1 static access. Sites: `push` at 0x12e093.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x12e098.
  Four-byte load, pattern 7: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12e0ac.
  Four-byte store, pattern 8: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12e0b3.
  Call (`call` at 0x12e09a): _nvramWrite, target 0x129b50. Candidate slots: stack arg0=`0x4`, stack arg1=`0x1e1048`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e0b4): _cmCheckTTYFlags, target 0x11c0a8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x12e0c8): target 0x12e0ac. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  At the exit (`ret` at 0x12e0dd), eax=`unknown`.
  Register fact (`xor` at 0x12e0a2): edi=`0x0`.
  Register fact (`xor` at 0x12e0a4): esi=`0x0`.
  Register fact (`mov` at 0x12e0a6): ebx=`0x1f1cc8`.
  Tool warning: `0x12e081 dec: unsupported effects invalidated`.
  Tool warning: `0x12e089 not: unsupported effects invalidated`.
  Tool warning: `0x12e0c1 inc: unsupported effects invalidated`.
  Tool warning: `0x12e0cc setle: unsupported effects invalidated`.
  Local unreached instructions: 0x12e0de to 0x12e0e7, ending at 0x12e0e8 (exclusive). Count 10.
  Encoding (`mov` at 0x12e085): bytes `668b550c`, raw prefix `0x66`.
  Encoding (`not` at 0x12e089): bytes `66f7d2`, raw prefix `0x66`.
  Encoding (`and` at 0x12e08c): bytes `6621904c101e00`, raw prefix `0x66`.

- `_comSetRxBuf`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e398`.
  Inferred analysis window: 0x12e398 to 0x12e3f8 (exclusive), 96 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 700, body file offset 0x263b8, window SHA256 `c4e87aaa61ff7fc81c1d19c16af7738090ce1761bd467f51e2214cdc5173d410`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e39b.
  Four-byte load, pattern 2: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e39e.
  Four-byte store, pattern 3: target `unknown`, raw base `eax`, value `[sp+8]`. Count 1 static access. Sites: `mov` at 0x12e3a5.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `0x1e1048`. Count 1 static access. Sites: `push` at 0x12e3ab.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x12e3b0.
  Call (`call` at 0x12e3b2): _nvramWrite, target 0x129b50. Candidate slots: stack arg0=`0x4`, stack arg1=`0x1e1048`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x12e3b9): target 0x12e3c8. Taken when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0`. Falls through when `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0`.
  At the exit (`ret` at 0x12e3c0), eax=`0x0`.
  At the exit (`ret` at 0x12e3d0), eax=`0xffffffff`.
  Register fact (`xor` at 0x12e3bb): eax=`0x0`.
  Register fact (`mov` at 0x12e3c8): eax=`0xffffffff`.
  Tool warning: `0x12e3a1 dec: unsupported effects invalidated`.
  Tool warning: `decoder stopped at 0x12e3f5; remaining bytes are not instructions`.
  Local unreached instructions: 0x12e3c1 to 0x12e3c7, ending at 0x12e3c8 (exclusive). Count 7.
  Local unreached instructions: 0x12e3d1 to 0x12e3f4, ending at 0x12e3f5 (exclusive). Count 14.
  Raw symbol tail: 0x12e3f5 to 0x12e3f8 (exclusive), bytes `256400`, with no instruction or effect credit.

- `_comSetTxBuf`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e468`.
  Inferred analysis window: 0x12e468 to 0x12e4a8 (exclusive), 64 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 701, body file offset 0x26488, window SHA256 `78c8c4e54bcb60c1e1d8b81bb70a405806f3d2d0c770f4569ee72934bb552f45`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e46b.
  Four-byte load, pattern 2: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e46e.
  Four-byte store, pattern 3: target `unknown`, raw base `eax`, value `[sp+8]`. Count 1 static access. Sites: `mov` at 0x12e475.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `0x1e1048`. Count 1 static access. Sites: `push` at 0x12e47b.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x12e480.
  Call (`call` at 0x12e482): _nvramWrite, target 0x129b50. Candidate slots: stack arg0=`0x4`, stack arg1=`0x1e1048`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x12e489): target 0x12e498. Taken when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0`. Falls through when `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0`.
  At the exit (`ret` at 0x12e490), eax=`0x0`.
  At the exit (`ret` at 0x12e4a0), eax=`0xffffffff`.
  Register fact (`xor` at 0x12e48b): eax=`0x0`.
  Register fact (`mov` at 0x12e498): eax=`0xffffffff`.
  Tool warning: `0x12e471 dec: unsupported effects invalidated`.
  Local unreached instructions: 0x12e491 to 0x12e497, ending at 0x12e498 (exclusive). Count 7.
  Local unreached instructions: 0x12e4a1 to 0x12e4a7, ending at 0x12e4a8 (exclusive). Count 7.

- `_comGetInfo`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e018`.
  Inferred analysis window: 0x12e018 to 0x12e038 (exclusive), 32 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 692, body file offset 0x26038, window SHA256 `7e4209456c2cfef6f37e890fef254fc3f5b465d8f8beac810810747cb71ddeba`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e01b.
  At the exit (`ret` at 0x12e029), eax=`unknown`.
  Local unreached instructions: 0x12e02a to 0x12e037, ending at 0x12e038 (exclusive). Count 14.

- `_comGetRxBuf`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e038`.
  Inferred analysis window: 0x12e038 to 0x12e058 (exclusive), 32 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 694, body file offset 0x26058, window SHA256 `d0cadf64b2b96c04c898d3fe68bce650be49e5c38fe743d383383e818692e0f1`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e03b.
  Four-byte store, pattern 2: target `sp -0x8`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12e03f.
  Call (`call` at 0x12e040): _ttyGetRxBuf, target 0x12e4e8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`ret` at 0x12e048), eax=`ret(_ttyGetRxBuf)`.
  Tool warning: `0x12e03e dec: unsupported effects invalidated`.
  Local unreached instructions: 0x12e049 to 0x12e057, ending at 0x12e058 (exclusive). Count 15.

- `_comGetTxBuf`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e058`.
  Inferred analysis window: 0x12e058 to 0x12e078 (exclusive), 32 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 693, body file offset 0x26078, window SHA256 `5f99184a83cdd2067836c4342cda044f4763a116afd566c75e46641a1519b649`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e05b.
  Four-byte store, pattern 2: target `sp -0x8`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12e05f.
  Call (`call` at 0x12e060): _ttyGetTxBuf, target 0x12e538. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`ret` at 0x12e068), eax=`ret(_ttyGetTxBuf)`.
  Tool warning: `0x12e05e dec: unsupported effects invalidated`.
  Local unreached instructions: 0x12e069 to 0x12e077, ending at 0x12e078 (exclusive). Count 15.

- `_ttyGetRxBuf`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e4e8`.
  Inferred analysis window: 0x12e4e8 to 0x12e538 (exclusive), 80 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 1106, body file offset 0x26508, window SHA256 `907bce0cdf235c593336fa7e0ca82e21af78bfd2458f1046cc7625d3b67878ed`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e4eb.
  Four-byte load, pattern 2: target `unknown`, raw base `eax`. Count 1 static access. Sites: `cmp` at 0x12e512.
  Four-byte load, pattern 3: target `unknown`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x12e519, `mov` at 0x12e52d.
  Branch (`jbe` at 0x12e4f1): target 0x12e508. Taken when `[sp+4] <= 0x5 (unsigned, 32-bit)`. Falls through when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  Branch (`je` at 0x12e517): target 0x12e528. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  At the exit (`ret` at 0x12e4fb), eax=`0xffffffff`.
  At the exit (`ret` at 0x12e520), eax=`unknown`.
  At the exit (`ret` at 0x12e534), eax=`unknown`.
  Register fact (`mov` at 0x12e4f3): eax=`0xffffffff`.
  Register fact (`mov` at 0x12e50d): eax=`0x1e1048`.
  Register fact (`mov` at 0x12e528): eax=`0x12dfb8`.
  Local unreached instructions: 0x12e4fc to 0x12e507, ending at 0x12e508 (exclusive). Count 12.
  Local unreached instructions: 0x12e521 to 0x12e527, ending at 0x12e528 (exclusive). Count 7.
  Local unreached instructions: 0x12e535 to 0x12e537, ending at 0x12e538 (exclusive). Count 3.

- `_ttyGetTxBuf`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e538`.
  Inferred analysis window: 0x12e538 to 0x12e628 (exclusive), 240 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 1107, body file offset 0x26558, window SHA256 `321c3758032acba5a941de17cd455b797fd21027ac83eeac8d1db87ff6551948`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e53b.
  Four-byte load, pattern 2: target `unknown`, raw base `eax`. Count 1 static access. Sites: `cmp` at 0x12e562.
  Four-byte load, pattern 3: target `unknown`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x12e569, `mov` at 0x12e57d.
  Branch (`jbe` at 0x12e541): target 0x12e558. Taken when `[sp+4] <= 0x5 (unsigned, 32-bit)`. Falls through when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  Branch (`je` at 0x12e567): target 0x12e578. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  At the exit (`ret` at 0x12e54b), eax=`0xffffffff`.
  At the exit (`ret` at 0x12e570), eax=`unknown`.
  At the exit (`ret` at 0x12e584), eax=`unknown`.
  Register fact (`mov` at 0x12e543): eax=`0xffffffff`.
  Register fact (`mov` at 0x12e55d): eax=`0x1e1048`.
  Register fact (`mov` at 0x12e578): eax=`0x12dfb8`.
  Tool warning: `decoder stopped at 0x12e626; remaining bytes are not instructions`.
  Local unreached instructions: 0x12e54c to 0x12e557, ending at 0x12e558 (exclusive). Count 12.
  Local unreached instructions: 0x12e571 to 0x12e577, ending at 0x12e578 (exclusive). Count 7.
  Local unreached instructions: 0x12e585 to 0x12e624, ending at 0x12e626 (exclusive). Count 60.
  Raw symbol tail: 0x12e626 to 0x12e628 (exclusive), bytes `0090`, with no instruction or effect credit.

- `_cmdTTY_REINIT`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x11a458`.
  Inferred analysis window: 0x11a458 to 0x11a520 (exclusive), 200 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 646, body file offset 0x12478, window SHA256 `6dbe78f8421b3111a67503fe43d716dc340d3b6007d8b5415bf11355cc95bd59`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11a45b.
  Four-byte load, pattern 2: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11a45e.
  Four-byte load, pattern 3: target `[sp+8] +0x20`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x11a461.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `0x1188e8`. Count 1 static access. Sites: `push` at 0x11a46b.
  Four-byte load, pattern 5: target `abs 0x1f4b7c`, raw base `unknown`. Count 4 static accesses. Sites: `mov` at 0x11a470, `mov` at 0x11a498, `mov` at 0x11a4e2.
  Four-byte load, pattern 6: target `unknown`, raw base `unknown`. Count 3 static accesses. Sites: `mov` at 0x11a475, `mov` at 0x11a4ed, `mov` at 0x11a502.
  Four-byte load, pattern 7: target `unknown`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x11a47c, `mov` at 0x11a4f4.
  Four-byte store, pattern 8: target `sp -0xc`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x11a47f.
  Byte load, pattern 9: target `([0x1f4b7c] + [sp+12]) +0x4`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x11a49d.
  Four-byte store, pattern 10: target `sp -0x8`, raw base `esp`, value `0x3c`. Count 1 static access. Sites: `push` at 0x11a4b8.
  Four-byte store, pattern 11: target `sp -0x8`, raw base `esp`, value `0x11a423`. Count 1 static access. Sites: `push` at 0x11a4d0.
  Four-byte store, pattern 12: target `sp -0x8`, raw base `esp`, value `[0x1f4b7c]`. Count 1 static access. Sites: `push` at 0x11a4e7.
  Four-byte store, pattern 13: target `sp -0xc`, raw base `esp`, value `0x118a61`. Count 1 static access. Sites: `push` at 0x11a4e8.
  Four-byte store, pattern 14: target `sp -0x10`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x11a4f7.
  Four-byte store, pattern 15: target `unknown`, raw base `eax`, value `0xb`. Count 1 static access. Sites: `mov` at 0x11a509.
  Call (`call` at 0x11a480): _sprintf, target 0x176c40. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1188e8`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11a4ba): _taskDelay, target 0x176140. Candidate slots: stack arg0=`0x3c`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11a4c2): _taskLock, target 0x176010. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11a4c7): _ttyReInit, target 0x12e8d8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11a4d5): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x11a423`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11a4dd): _taskUnlock, target 0x176040. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11a4f8): _sprintf, target 0x176c40. Candidate slots: stack arg0=`unknown`, stack arg1=`0x118a61`, stack arg2=`[0x1f4b7c]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jg` at 0x11a469): target 0x11a498. Taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)`. Falls through when `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)`.
  Branch (`jne` at 0x11a4a3): target 0x11a4b8. Taken when `(([([0x1f4b7c] + [sp+12])+4]:1 & 0xff) & ([([0x1f4b7c] + [sp+12])+4]:1 & 0xff)) != 0x0`. Falls through when `(([([0x1f4b7c] + [sp+12])+4]:1 & 0xff) & ([([0x1f4b7c] + [sp+12])+4]:1 & 0xff)) == 0x0`.
  Branch (`je` at 0x11a4ce): target 0x11a4dd. Taken when `(ret(_ttyReInit) & ret(_ttyReInit)) == 0x0`. Falls through when `(ret(_ttyReInit) & ret(_ttyReInit)) != 0x0`.
  At the exit (`ret` at 0x11a48a), eax=`0x0`.
  At the exit (`ret` at 0x11a4ad), eax=`0x1`.
  At the exit (`ret` at 0x11a518), eax=`0x1`.
  Register fact (`xor` at 0x11a485): eax=`0x0`.
  Register fact (`mov` at 0x11a4a5): eax=`0x1`.
  Register fact (`mov` at 0x11a510): eax=`0x1`.
  Local unreached instructions: 0x11a48b to 0x11a497, ending at 0x11a498 (exclusive). Count 13.
  Local unreached instructions: 0x11a4ae to 0x11a4b7, ending at 0x11a4b8 (exclusive). Count 10.
  Local unreached instructions: 0x11a519 to 0x11a51e, ending at 0x11a520 (exclusive). Count 5.

- `_cmCheckTTYFlags`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x11c0a8`.
  Inferred analysis window: 0x11c0a8 to 0x11c258 (exclusive), 432 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 794, body file offset 0x140c8, window SHA256 `e6efae4bc620b6dfeb4e9f6fb644d2826fe915ac33a747869dbfaa9f5feb279c`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11c0b1.
  Four-byte load, pattern 2: target `[sp+4] +0x3c`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11c0b4.
  Four-byte store, pattern 3: target `sp -0x18`, raw base `esp`, value `sp-8`. Count 1 static access. Sites: `push` at 0x11c0bf.
  Four-byte store, pattern 4: target `sp -0x1c`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x11c0c0.
  Four-byte load, pattern 5: target `[sp+4] +0x70`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11c0d3.
  Four-byte load, pattern 6: target `[[sp+4]+112] +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x11c0d6.
  Four-byte load, pattern 7: target `sp -0x8`, raw base `ebp`. Count 2 static accesses. Sites: `cmp` at 0x11c0d8, `cmp` at 0x11c138.
  Two-byte load, pattern 8: target `ret(_ttyGetInfo) +0x4`, raw base `edx`. Count 4 static accesses. Sites: `mov` at 0x11c0dd, `mov` at 0x11c103, `mov` at 0x11c13d.
  Four-byte load, pattern 9: target `[sp+4] +0x3c`, raw base `ebx`. Count 9 static accesses. Sites: `mov` at 0x11c0e6, `mov` at 0x11c0f8, `mov` at 0x11c115.
  Four-byte store, pattern 10: target `[sp+4] +0x3c`, raw base `ebx`, value `unknown`. Count 4 static accesses. Sites: `mov` at 0x11c100, `mov` at 0x11c130, `mov` at 0x11c160.
  Byte load, pattern 11: target `abs 0x1e0bf3`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x11c10c.
  Four-byte load, pattern 12: target `[sp+4] +0x68`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11c133.
  Four-byte load, pattern 13: target `[[sp+4]+104] +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x11c136.
  Byte load, pattern 14: target `abs 0x1e0bf3`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x11c16c.
  Call (`call` at 0x11c0c1): _ttyGetInfo, target 0x12e4a8. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x11c0cd): target 0x11c193. Taken when `(ret(_ttyGetInfo) & ret(_ttyGetInfo)) == 0x0`. Falls through when `(ret(_ttyGetInfo) & ret(_ttyGetInfo)) != 0x0`.
  Branch (`jne` at 0x11c0db): target 0x11c133. Taken when `[sp-8] != [[[sp+4]+112]]`. Falls through when `[sp-8] == [[[sp+4]+112]]`.
  Branch (`je` at 0x11c0e4): target 0x11c0f8. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`jmp` at 0x11c0ee): target 0x11c100. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x11c10a): target 0x11c128. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Branch (`jne` at 0x11c113): target 0x11c128. Taken when `(([0x1e0bf3]:1 & 0xff) & 0x10) != 0x0`. Falls through when `(([0x1e0bf3]:1 & 0xff) & 0x10) == 0x0`.
  Jump (`jmp` at 0x11c11d): target 0x11c130. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x11c13b): target 0x11c193. Taken when `[sp-8] != [[[sp+4]+104]]`. Falls through when `[sp-8] == [[[sp+4]+104]]`.
  Branch (`je` at 0x11c144): target 0x11c158. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`jmp` at 0x11c14e): target 0x11c160. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x11c16a): target 0x11c188. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Branch (`jne` at 0x11c173): target 0x11c188. Taken when `(([0x1e0bf3]:1 & 0xff) & 0x20) != 0x0`. Falls through when `(([0x1e0bf3]:1 & 0xff) & 0x20) == 0x0`.
  Jump (`jmp` at 0x11c17d): target 0x11c190. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jle` at 0x11c197): target 0x11c0bc. Taken when `? <= 0x5 (signed, 32-bit)`. Falls through when `? > 0x5 (signed, 32-bit)`.
  At the exit (`ret` at 0x11c1b3), eax=`unknown`.
  Register fact (`xor` at 0x11c0b7): esi=`0x0`.
  Tool warning: `0x11c193 inc: unsupported effects invalidated`.
  Tool warning: `0x11c1a2 setne: unsupported effects invalidated`.
  Local unreached instructions: 0x11c0f0 to 0x11c0f7, ending at 0x11c0f8 (exclusive). Count 8.
  Local unreached instructions: 0x11c11f to 0x11c127, ending at 0x11c128 (exclusive). Count 9.
  Local unreached instructions: 0x11c150 to 0x11c157, ending at 0x11c158 (exclusive). Count 8.
  Local unreached instructions: 0x11c17f to 0x11c187, ending at 0x11c188 (exclusive). Count 9.
  Local unreached instructions: 0x11c1b4 to 0x11c257, ending at 0x11c258 (exclusive). Count 68.
  Encoding (`mov` at 0x11c0dd): bytes `668b4204`, raw prefix `0x66`.
  Encoding (`mov` at 0x11c103): bytes `668b4204`, raw prefix `0x66`.
  Encoding (`mov` at 0x11c13d): bytes `668b4204`, raw prefix `0x66`.
  Encoding (`mov` at 0x11c163): bytes `668b4204`, raw prefix `0x66`.

- `_clLeaveELSCMode`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x11b9f0`.
  Inferred analysis window: 0x11b9f0 to 0x11baf0 (exclusive), 256 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 803, body file offset 0x13a10, window SHA256 `baea775e309578df36091c6233d102929a5963a390d3a73dc3ab5432f9cc6406`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11b9fa.
  Four-byte load, pattern 2: target `[sp+4] +0x68`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x11b9fd, `mov` at 0x11ba09.
  Four-byte store, pattern 3: target `sp -0x173c`, raw base `esp`, value `[[sp+4]+104]`. Count 2 static accesses. Sites: `push` at 0x11ba00, `push` at 0x11ba0c.
  Four-byte load, pattern 4: target `[sp+4] +0x6c`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11ba15.
  Four-byte store, pattern 5: target `sp -0x173c`, raw base `esp`, value `[[sp+4]+108]`. Count 1 static access. Sites: `push` at 0x11ba18.
  Four-byte load, pattern 6: target `[sp+4] +0x70`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11ba21.
  Four-byte store, pattern 7: target `sp -0x173c`, raw base `esp`, value `[[sp+4]+112]`. Count 1 static access. Sites: `push` at 0x11ba24.
  Four-byte load, pattern 8: target `[sp+4] +0x74`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x11ba2d, `mov` at 0x11ba39.
  Four-byte store, pattern 9: target `sp -0x173c`, raw base `esp`, value `[[sp+4]+116]`. Count 2 static accesses. Sites: `push` at 0x11ba30, `push` at 0x11ba3c.
  Four-byte store, pattern 10: target `[sp+4] +0x18`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11ba45.
  Four-byte store, pattern 11: target `[sp+4] +0x1c`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11ba4c.
  Four-byte store, pattern 12: target `[sp+4] +0x20`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11ba53.
  Four-byte store, pattern 13: target `[sp+4] +0x24`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11ba5a.
  Four-byte store, pattern 14: target `[sp+4] +0x28`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11ba61.
  Four-byte load, pattern 15: target `[sp+4] +0x7c`, raw base `ebx`. Count 3 static accesses. Sites: `mov` at 0x11ba68, `mov` at 0x11ba6f, `mov` at 0x11ba79.
  Four-byte store, pattern 16: target `[[sp+4]+124] +0x4`, raw base `eax`, value `0x1`. Count 1 static access. Sites: `mov` at 0x11ba72.
  Four-byte store, pattern 17: target `sp -0x173c`, raw base `esp`, value `[[sp+4]+124]`. Count 1 static access. Sites: `push` at 0x11ba7c.
  Four-byte store, pattern 18: target `abs 0x1f2264`, raw base `unknown`, value `0x1`. Count 1 static access. Sites: `mov` at 0x11ba85.
  Four-byte store, pattern 19: target `sp -0x173c`, raw base `esp`, value `sp-5940`. Count 1 static access. Sites: `push` at 0x11ba95.
  Four-byte store, pattern 20: target `sp -0x1740`, raw base `esp`, value `0x11b9cc`. Count 1 static access. Sites: `push` at 0x11ba96.
  Four-byte store, pattern 21: target `sp -0x1744`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x11ba9b.
  Four-byte store, pattern 22: target `[sp+4] +0x7c`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11baa4.
  Byte load, pattern 23: target `abs 0x1e0bf0`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x11baab.
  Four-byte store, pattern 24: target `sp -0x173c`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x11bab4.
  Four-byte store, pattern 25: target `sp -0x1740`, raw base `esp`, value `0x11b9d4`. Count 1 static access. Sites: `push` at 0x11bab5.
  Call (`call` at 0x11ba01): _bufHold, target 0x10f948. Candidate slots: stack arg0=`[[sp+4]+104]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba0d): _bufDetachFD, target 0x10f6e8. Candidate slots: stack arg0=`[[sp+4]+104]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba19): _bufDetachFD, target 0x10f6e8. Candidate slots: stack arg0=`[[sp+4]+108]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba25): _bufDetachFD, target 0x10f6e8. Candidate slots: stack arg0=`[[sp+4]+112]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba31): _bufDetachFD, target 0x10f6e8. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba3d): _bufDetachLog, target 0x10f758. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba7d): _elscSetup, target 0x12434c. Candidate slots: stack arg0=`[[sp+4]+124]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11ba9c): _cmSendFFSCCmd, target 0x11c738. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0x11b9cc`, stack arg2=`sp-5940`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11baba): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x11b9d4`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x11ba6d): target 0x11baab. Taken when `([[sp+4]+124] & [[sp+4]+124]) == 0x0`. Falls through when `([[sp+4]+124] & [[sp+4]+124]) != 0x0`.
  Branch (`je` at 0x11bab2): target 0x11babf. Taken when `(([0x1e0bf0]:1 & 0xff) & 0x4) == 0x0`. Falls through when `(([0x1e0bf0]:1 & 0xff) & 0x4) != 0x0`.
  At the exit (`ret` at 0x11baca), eax=`0x0`.
  Register fact (`xor` at 0x11babf): eax=`0x0`.
  Local unreached instructions: 0x11bacb to 0x11baef, ending at 0x11baf0 (exclusive). Count 17.
