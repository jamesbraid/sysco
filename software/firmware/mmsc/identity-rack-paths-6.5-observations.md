# MMSC identity and rack-path observations

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

The [full facts](disasm/identity-rack-paths-6.5-observations-facts.txt) retain every access, instruction and warning.

- `_identScanForConflicts`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124798`.
  Inferred analysis window: 0x124798 to 0x124a28 (exclusive), 656 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 572, body file offset 0x1c7b8, window SHA256 `dadc9fe6ca8c19e92b50aa97ee523c295add6010bc76e14cccd6db57596110d7`.
  Byte load, pattern 1: target `abs 0x1df500`, raw base `unknown`. Count 3 static accesses. Sites: `test` at 0x12479e, `test` at 0x124884, `test` at 0x124963.
  Four-byte store, pattern 2: target `sp -0x14`, raw base `esp`, value `0x124729`. Count 1 static access. Sites: `push` at 0x1247a7.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `esp`, value `0x1d75a0`. Count 2 static accesses. Sites: `push` at 0x1247b6, `push` at 0x12489e.
  Four-byte load, pattern 4: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1247d0.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `unknown`. Count 3 static accesses. Sites: `push` at 0x1247d3, `push` at 0x1247e4, `push` at 0x1248c3.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `esp`, value `unknown`. Count 4 static accesses. Sites: `push` at 0x1247e5, `push` at 0x12483a, `push` at 0x1248c4.
  Four-byte load, pattern 7: target `sp +0x4`, raw base `ebp`. Count 2 static accesses. Sites: `mov` at 0x1247e6, `mov` at 0x12483b.
  Four-byte store, pattern 8: target `sp -0x14`, raw base `esp`, value `unknown`. Count 4 static accesses. Sites: `push` at 0x124808, `push` at 0x124865, `push` at 0x1248e8.
  Four-byte load, pattern 9: target `ret(_lstNext) +0x18`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x124818, `mov` at 0x12482c.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `esp`, value `[ret(_lstNext)+24]`. Count 1 static access. Sites: `push` at 0x12481b.
  Four-byte load, pattern 11: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x124829.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `esp`, value `ret(_lstNext)+16`. Count 2 static accesses. Sites: `push` at 0x124836, `push` at 0x124905.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `esp`, value `ret(_lstNext)`. Count 2 static accesses. Sites: `push` at 0x124852, `push` at 0x124931.
  Four-byte store, pattern 14: target `sp -0x14`, raw base `esp`, value `0x124753`. Count 1 static access. Sites: `push` at 0x12488d.
  Four-byte load, pattern 15: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1248b8.
  Four-byte load, pattern 16: target `sp +0x8`, raw base `ebp`. Count 2 static accesses. Sites: `mov` at 0x1248c5, `mov` at 0x12490a.
  Four-byte load, pattern 17: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1248f8.
  Four-byte load, pattern 18: target `ret(_lstNext) +0x1c`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x1248fb.
  Four-byte store, pattern 19: target `sp -0x14`, raw base `esp`, value `0x124779`. Count 1 static access. Sites: `push` at 0x12496c.
  Call (`call` at 0x1247ac): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124729`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x1247bb): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1247d4): _inet_lnaof, target 0x18b128. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1247e9): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12481c): _inet_lnaof, target 0x18b128. Candidate slots: stack arg0=`[ret(_lstNext)+24]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12483e): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`ret(_lstNext)+16`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124853): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124866): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124892): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124753`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1248a3): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1248c8): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12490d): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`ret(_lstNext)+16`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124932): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124945): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124971): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124779`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x1247a5): target 0x1247b4. Taken when `(([0x1df500]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df500]:1 & 0xff) & 0x1) != 0x0`.
  Branch (`je` at 0x1247c7): target 0x12487c. Taken when `(ret(_lstFirst) & ret(_lstFirst)) == 0x0`. Falls through when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0`.
  Branch (`jg` at 0x1247df): target 0x124808. Taken when `ret(_inet_lnaof) > 0xf (signed, 32-bit)`. Falls through when `ret(_inet_lnaof) <= 0xf (signed, 32-bit)`.
  Branch (`jl` at 0x1247f0): target 0x124916. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Branch (`jne` at 0x1247f6): target 0x124865. Taken when `(ret(unknown) & ret(unknown)) != 0x0`. Falls through when `(ret(unknown) & ret(unknown)) == 0x0`.
  Jump (`jmp` at 0x1247fd): target 0x124865. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x124809): target 0x124853. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jle` at 0x124827): target 0x124852. Taken when `ret(_inet_lnaof) <= 0xf (signed, 32-bit)`. Falls through when `ret(_inet_lnaof) > 0xf (signed, 32-bit)`.
  Branch (`jne` at 0x124831): target 0x124852. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`jl` at 0x124845): target 0x124916. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Branch (`jne` at 0x12484b): target 0x124852. Taken when `(ret(unknown) & ret(unknown)) != 0x0`. Falls through when `(ret(unknown) & ret(unknown)) == 0x0`.
  Branch (`je` at 0x12485f): target 0x124865. Taken when `(ret(_lstNext) & ret(_lstNext)) == 0x0`. Falls through when `(ret(_lstNext) & ret(_lstNext)) != 0x0`.
  Branch (`je` at 0x124863): target 0x124818. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`je` at 0x124872): target 0x12487c. Taken when `(ret(_lstNext) & ret(_lstNext)) == 0x0`. Falls through when `(ret(_lstNext) & ret(_lstNext)) != 0x0`.
  Branch (`je` at 0x124876): target 0x1247d0. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jne` at 0x12487e): target 0x1247b4. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`je` at 0x12488b): target 0x12489c. Taken when `(([0x1df500]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df500]:1 & 0xff) & 0x1) != 0x0`.
  Branch (`je` at 0x1248af): target 0x12495b. Taken when `(ret(_lstFirst) & ret(_lstFirst)) == 0x0`. Falls through when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0`.
  Branch (`jne` at 0x1248be): target 0x1248e8. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`jl` at 0x1248cf): target 0x124916. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Branch (`jne` at 0x1248d1): target 0x124944. Taken when `(ret(unknown) & ret(unknown)) != 0x0`. Falls through when `(ret(unknown) & ret(unknown)) == 0x0`.
  Jump (`jmp` at 0x1248d8): target 0x124944. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x1248e9): target 0x124932. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x124900): target 0x124931. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`jge` at 0x124914): target 0x124928. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Jump (`jmp` at 0x12491b): target 0x124978. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x12492a): target 0x124931. Taken when `(ret(unknown) & ret(unknown)) != 0x0`. Falls through when `(ret(unknown) & ret(unknown)) == 0x0`.
  Branch (`je` at 0x12493e): target 0x124944. Taken when `(ret(_lstNext) & ret(_lstNext)) == 0x0`. Falls through when `(ret(_lstNext) & ret(_lstNext)) != 0x0`.
  Branch (`je` at 0x124942): target 0x1248f8. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`je` at 0x124951): target 0x12495b. Taken when `(ret(_lstNext) & ret(_lstNext)) == 0x0`. Falls through when `(ret(_lstNext) & ret(_lstNext)) != 0x0`.
  Branch (`je` at 0x124955): target 0x1248b8. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jne` at 0x12495d): target 0x12489c. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`je` at 0x12496a): target 0x124976. Taken when `(([0x1df500]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df500]:1 & 0xff) & 0x1) != 0x0`.
  At the exit (`ret` at 0x124981), eax=`unknown`.
  Register fact (`xor` at 0x1247b4): edi=`0x0`.
  Register fact (`mov` at 0x1247f8): edi=`0x1`.
  Register fact (`mov` at 0x12484d): edi=`0x1`.
  Register fact (`xor` at 0x12489c): edi=`0x0`.
  Register fact (`mov` at 0x1248d3): edi=`0x1`.
  Register fact (`mov` at 0x124916): eax=`0xffffffff`.
  Register fact (`mov` at 0x12492c): edi=`0x1`.
  Register fact (`xor` at 0x124976): eax=`0x0`.
  Local unreached instructions: 0x1247ff to 0x124807, ending at 0x124808 (exclusive). Count 9.
  Local unreached instructions: 0x12480b to 0x124817, ending at 0x124818 (exclusive). Count 13.
  Local unreached instructions: 0x1248da to 0x1248e7, ending at 0x1248e8 (exclusive). Count 14.
  Local unreached instructions: 0x1248eb to 0x1248f7, ending at 0x1248f8 (exclusive). Count 13.
  Local unreached instructions: 0x12491d to 0x124927, ending at 0x124928 (exclusive). Count 11.
  Local unreached instructions: 0x124982 to 0x124a27, ending at 0x124a28 (exclusive). Count 76.

- `_identSetIPAddr`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124d48`.
  Inferred analysis window: 0x124d48 to 0x124e38 (exclusive), 240 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 526, body file offset 0x1cd68, window SHA256 `e42228109fe41e4ebaee09c424f26eccfd3c8838475ff30fb180e25b244a94fb`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124d51.
  Four-byte load, pattern 2: target `[sp+4] +0x0`, raw base `esi`. Count 4 static accesses. Sites: `mov` at 0x124d54, `mov` at 0x124d62, `mov` at 0x124d88.
  Four-byte store, pattern 3: target `sp -0x2c`, raw base `esp`, value `[[sp+4]]`. Count 2 static accesses. Sites: `push` at 0x124d56, `push` at 0x124d64.
  Four-byte store, pattern 4: target `sp -0x1c`, raw base `ebp`, value `ret(_inet_lnaof)`. Count 1 static access. Sites: `mov` at 0x124d5f.
  Four-byte load, pattern 5: target `abs 0x1dfa4c`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x124d6f.
  Four-byte store, pattern 6: target `sp -0x2c`, raw base `esp`, value `0x124c8d`. Count 1 static access. Sites: `push` at 0x124d77.
  Four-byte store, pattern 7: target `sp -0x2c`, raw base `esp`, value `sp-24`. Count 4 static accesses. Sites: `push` at 0x124d87, `push` at 0x124d93, `push` at 0x124da5.
  Four-byte store, pattern 8: target `sp -0x30`, raw base `esp`, value `[[sp+4]]`. Count 1 static access. Sites: `push` at 0x124d8a.
  Four-byte store, pattern 9: target `sp -0x30`, raw base `esp`, value `0x124cb3`. Count 1 static access. Sites: `push` at 0x124d94.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `esp`, value `0x124cb8`. Count 1 static access. Sites: `push` at 0x124da6.
  Four-byte store, pattern 11: target `abs 0x1f3450`, raw base `unknown`, value `[[sp+4]]`. Count 1 static access. Sites: `mov` at 0x124dba.
  Four-byte store, pattern 12: target `abs 0x1dfa4c`, raw base `unknown`, value `ret(_inet_netof)`. Count 1 static access. Sites: `mov` at 0x124dbf.
  Four-byte load, pattern 13: target `sp -0x1c`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124dc5.
  Four-byte store, pattern 14: target `abs 0x1dfa48`, raw base `unknown`, value `[sp-28]`. Count 1 static access. Sites: `mov` at 0x124dc8.
  Four-byte store, pattern 15: target `sp -0x2c`, raw base `esp`, value `0x1dfa48`. Count 1 static access. Sites: `push` at 0x124dce.
  Four-byte store, pattern 16: target `sp -0x30`, raw base `esp`, value `0x1`. Count 1 static access. Sites: `push` at 0x124dd3.
  Four-byte store, pattern 17: target `sp -0x2c`, raw base `esp`, value `0x124ce2`. Count 1 static access. Sites: `push` at 0x124de1.
  Byte load, pattern 18: target `abs 0x1df501`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x124dee.
  Four-byte store, pattern 19: target `sp -0x30`, raw base `esp`, value `0x124d20`. Count 1 static access. Sites: `push` at 0x124df8.
  Call (`call` at 0x124d57): _inet_lnaof, target 0x18b128. Candidate slots: stack arg0=`[[sp+4]]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124d65): _inet_netof, target 0x18b268. Candidate slots: stack arg0=`[[sp+4]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`ret(_inet_lnaof)`, stack arg5=`unknown`.
  Call (`call` at 0x124d7c): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124c8d`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124d8b): _inet_ntoa_b, target 0x18b458. Candidate slots: stack arg0=`[[sp+4]]`, stack arg1=`sp-24`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124d99): _ifAddrSet, target 0x18c0a8. Candidate slots: stack arg0=`0x124cb3`, stack arg1=`sp-24`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124dab): _ffscMsgS, target 0x120634. Candidate slots: stack arg0=`0x124cb8`, stack arg1=`sp-24`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124dd5): _nvramWrite, target 0x128b18. Candidate slots: stack arg0=`0x1`, stack arg1=`0x1dfa48`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124de6): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124ce2`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124dfd): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124d20`, stack arg1=`sp-24`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x124d75): target 0x124d84. Taken when `[0x1dfa4c] == ret(_inet_netof)`. Falls through when `[0x1dfa4c] != ret(_inet_netof)`.
  Branch (`je` at 0x124da3): target 0x124db8. Taken when `(ret(_ifAddrSet) & ret(_ifAddrSet)) == 0x0`. Falls through when `(ret(_ifAddrSet) & ret(_ifAddrSet)) != 0x0`.
  Jump (`jmp` at 0x124db5): target 0x124e04. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x124ddf): target 0x124dee. Taken when `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0`. Falls through when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0`.
  Branch (`je` at 0x124df5): target 0x124e02. Taken when `(([0x1df501]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df501]:1 & 0xff) & 0x1) != 0x0`.
  At the exit (`ret` at 0x124e0d), eax=`unknown`.
  Register fact (`mov` at 0x124db0): eax=`0xffffffff`.
  Register fact (`xor` at 0x124e02): eax=`0x0`.
  Local unreached instructions: 0x124db7 to 0x124db7, ending at 0x124db8 (exclusive). Count 1.
  Local unreached instructions: 0x124e0e to 0x124e37, ending at 0x124e38 (exclusive). Count 21.

- `_identSetCapabilities`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124c68`.
  Inferred analysis window: 0x124c68 to 0x124d48 (exclusive), 224 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 527, body file offset 0x1cc88, window SHA256 `d773ed96701c63ede1b268d21bdd6f59de479f602a4354ddfd7db3d4bf965b95`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124c6b.
  Four-byte store, pattern 2: target `abs 0x1f3458`, raw base `unknown`, value `[sp+4]`. Count 1 static access. Sites: `mov` at 0x124c6e.
  Byte load, pattern 3: target `abs 0x1df501`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x124c73.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x124c7c.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x124c3d`. Count 1 static access. Sites: `push` at 0x124c7d.
  Call (`call` at 0x124c82): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124c3d`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x124c7a): target 0x124c87. Taken when `(([0x1df501]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df501]:1 & 0xff) & 0x1) != 0x0`.
  At the exit (`ret` at 0x124c8c), eax=`0x0`.
  Register fact (`xor` at 0x124c87): eax=`0x0`.
  Local unreached instructions: 0x124c8d to 0x124d47, ending at 0x124d48 (exclusive). Count 91.

- `_identSetModuleNums`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124e38`.
  Inferred analysis window: 0x124e38 to 0x124f18 (exclusive), 224 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 528, body file offset 0x1ce58, window SHA256 `52285a6fbd301b59fcf2e45fa4bfdcf71d7702918639192e3edb585010f988ef`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124e41.
  Four-byte load, pattern 2: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x124e50.
  Four-byte store, pattern 3: target `unknown`, raw base `unknown`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124e53.
  Byte load, pattern 4: target `abs 0x1df501`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x124e62.
  Four-byte store, pattern 5: target `sp -0x20`, raw base `esp`, value `0x124e0e`. Count 1 static access. Sites: `push` at 0x124e6b.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `esp`, value `sp-16`. Count 2 static accesses. Sites: `push` at 0x124e84, `push` at 0x124e95.
  Four-byte load, pattern 7: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x124e85.
  Four-byte store, pattern 8: target `sp -0x24`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x124e8c.
  Four-byte store, pattern 9: target `sp -0x24`, raw base `esp`, value `0x124e29`. Count 1 static access. Sites: `push` at 0x124e96.
  Four-byte store, pattern 10: target `sp -0x20`, raw base `esp`, value `0x124afa`. Count 1 static access. Sites: `push` at 0x124eaf.
  Call (`call` at 0x124e70): _ffscMsgN, target 0x1205a4. Candidate slots: stack arg0=`0x124e0e`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124e8d): _ffscModuleNumToString, target 0x127138. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-16`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124e9b): _ffscMsgN, target 0x1205a4. Candidate slots: stack arg0=`0x124e29`, stack arg1=`sp-16`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124eb4): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124afa`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jbe` at 0x124e60): target 0x124e50. Taken when `be flags condition (operands unknown)`. Falls through when `not (be flags condition (operands unknown))`.
  Branch (`je` at 0x124e69): target 0x124eb9. Taken when `(([0x1df501]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df501]:1 & 0xff) & 0x1) != 0x0`.
  Branch (`jbe` at 0x124ead): target 0x124e84. Taken when `be flags condition (operands unknown)`. Falls through when `not (be flags condition (operands unknown))`.
  At the exit (`ret` at 0x124ec4), eax=`0x0`.
  Register fact (`xor` at 0x124e44): ebx=`0x0`.
  Register fact (`mov` at 0x124e46): edx=`0x1f345c`.
  Register fact (`mov` at 0x124e4b): ecx=`0x1f3460`.
  Register fact (`xor` at 0x124e78): ebx=`0x0`.
  Register fact (`mov` at 0x124e7d): esi=`0x1f345c`.
  Register fact (`xor` at 0x124eb9): eax=`0x0`.
  Tool warning: `0x124e5d inc: unsupported effects invalidated`.
  Tool warning: `0x124ea6 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x124ec5 to 0x124f12, ending at 0x124f18 (exclusive). Count 42.

- `_identFindByUniqueID`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1243b8`.
  Inferred analysis window: 0x1243b8 to 0x1243d8 (exclusive), 32 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 545, body file offset 0x1c3d8, window SHA256 `1e3f9bef70c15c0c0fa7a12381037d722ac144168dd5bd4b887817a10babde83`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1243bb.
  Four-byte store, pattern 2: target `sp -0x8`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x1243be.
  Call (`call` at 0x1243bf): _identFindIdentityNodeByUniqueID, target 0x124f88. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x1243c8): target 0x1243cd. Taken when `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0`. Falls through when `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0`.
  At the exit (`ret` at 0x1243d2), eax=`unknown`.
  Register fact (`xor` at 0x1243c4): edx=`0x0`.
  Local unreached instructions: 0x1243d3 to 0x1243d7, ending at 0x1243d8 (exclusive). Count 5.

- `_identFindByIPAddr`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1242a8`.
  Inferred analysis window: 0x1242a8 to 0x1242f8 (exclusive), 80 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 555, body file offset 0x1c2c8, window SHA256 `2eb9d265b99b5086cc185b401ba739583b7568f09dbe25707f7687155e5cb126`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1242ac.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x1242af.
  Four-byte load, pattern 3: target `unknown`, raw base `ecx`. Count 1 static access. Sites: `mov` at 0x1242c8.
  Four-byte load, pattern 4: target `[sp+4] +0x0`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x1242cb.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x1242d8.
  Call (`call` at 0x1242b4): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1242d9): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x1242b9): target 0x1242de. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x1242cf): target 0x1242d8. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`jmp` at 0x1242d4): target 0x1242e9. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x1242e5): target 0x1242c8. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  At the exit (`ret` at 0x1242ef), eax=`unknown`.
  Register fact (`xor` at 0x1242e7): eax=`0x0`.
  Local unreached instructions: 0x1242bb to 0x1242c7, ending at 0x1242c8 (exclusive). Count 13.
  Local unreached instructions: 0x1242d6 to 0x1242d7, ending at 0x1242d8 (exclusive). Count 2.
  Local unreached instructions: 0x1242f0 to 0x1242f7, ending at 0x1242f8 (exclusive). Count 8.

- `_identCleanOldEntries`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124048`.
  Inferred analysis window: 0x124048 to 0x1240e8 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 559, body file offset 0x1c068, window SHA256 `54b97973cc9968ab6b782005e88262973a7f7b92bcdeb4a7dbe3af714fcf8337`.
  Four-byte store, pattern 1: target `sp -0x14`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x12404e.
  Four-byte store, pattern 2: target `sp -0x14`, raw base `esp`, value `unknown`. Count 3 static accesses. Sites: `push` at 0x124064, `push` at 0x1240bf, `push` at 0x1240cd.
  Four-byte load, pattern 3: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12406f.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x124072.
  Four-byte store, pattern 5: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x124076.
  Four-byte store, pattern 6: target `sp -0x14`, raw base `esp`, value `0x1f3448`. Count 1 static access. Sites: `push` at 0x124083.
  Four-byte store, pattern 7: target `sp -0x18`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x12408b, `push` at 0x12409e.
  Four-byte load, pattern 8: target `abs 0x1f05c0`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x124098.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `esp`, value `[0x1f05c0]`. Count 1 static access. Sites: `push` at 0x12409d.
  Four-byte store, pattern 10: target `abs 0x1f05c0`, raw base `unknown`, value `0x0`. Count 1 static access. Sites: `mov` at 0x1240ab.
  Four-byte store, pattern 11: target `abs 0x1f398c`, raw base `unknown`, value `0x0`. Count 1 static access. Sites: `mov` at 0x1240b5.
  Four-byte store, pattern 12: target `sp -0x18`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x1240c0.
  Call (`call` at 0x124053): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x124065): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124077): _ffscCompareTimestamps, target 0x126ed8. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12408c): _ffscCompareUniqueIDs, target 0x126f28. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1f3448`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12409f): _ffscCompareUniqueIDs, target 0x126f28. Candidate slots: stack arg0=`unknown`, stack arg1=`[0x1f05c0]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1240c5): _lstDelete, target 0x1929f0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1240ce): _free, target 0x181410. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12405f): target 0x1240dc. Taken when `(ret(_lstFirst) & ret(_lstFirst)) == 0x0`. Falls through when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0`.
  Branch (`jge` at 0x124081): target 0x1240d6. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`je` at 0x124096): target 0x1240d6. Taken when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0`. Falls through when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0`.
  Branch (`jne` at 0x1240a9): target 0x1240bf. Taken when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0`. Falls through when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0`.
  Branch (`jne` at 0x1240da): target 0x124064. Taken when `(ret(_lstNext) & ret(_lstNext)) != 0x0`. Falls through when `(ret(_lstNext) & ret(_lstNext)) == 0x0`.
  At the exit (`ret` at 0x1240e7), eax=`0x0`.
  Register fact (`xor` at 0x1240dc): eax=`0x0`.

- `_identEnumerateRacks`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1241f8`.
  Inferred analysis window: 0x1241f8 to 0x1242a8 (exclusive), 176 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 897, body file offset 0x1c218, window SHA256 `568c8f41da6f1d7480b9f4f7ff35fe9173b915c598a0b307ceb24e899ec5f97f`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124201.
  Four-byte store, pattern 2: target `sp -0x20`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x124206.
  Four-byte load, pattern 3: target `unknown`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x12421c, `mov` at 0x124224.
  Four-byte load, pattern 4: target `sp +0x4`, raw base `ebp`. Count 4 static accesses. Sites: `mov` at 0x124227, `mov` at 0x124253, `mov` at 0x12425c.
  Four-byte store, pattern 5: target `unknown`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x12422a.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12422e.
  Four-byte store, pattern 7: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124246.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124259.
  Four-byte store, pattern 9: target `sp -0xc`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124262.
  Four-byte load, pattern 10: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12426b.
  Four-byte load, pattern 11: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12426e.
  Four-byte load, pattern 12: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x124275.
  Four-byte load, pattern 13: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x124278.
  Four-byte store, pattern 14: target `unknown`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x12427b.
  Four-byte store, pattern 15: target `unknown`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x12427e.
  Four-byte load, pattern 16: target `sp -0x10`, raw base `ebp`. Count 1 static access. Sites: `add` at 0x124281.
  Four-byte store, pattern 17: target `sp -0x10`, raw base `ebp`, value `[sp-16]+4`. Count 1 static access. Sites: `add` at 0x124281.
  Four-byte load, pattern 18: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124286.
  Four-byte load, pattern 19: target `sp -0x10`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x124289.
  Four-byte load, pattern 20: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x12428f.
  Call (`call` at 0x12420b): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12422f): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x124210): target 0x124234. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12421a): target 0x12423d. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`je` at 0x124222): target 0x12422e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jle` at 0x12423b): target 0x124218. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  Branch (`jge` at 0x124244): target 0x124294. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`jge` at 0x124251): target 0x12428e. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`jae` at 0x124273): target 0x124281. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Branch (`jl` at 0x12428c): target 0x124268. Taken when `[sp-16] < [sp-12] (signed, 32-bit)`. Falls through when `[sp-16] >= [sp-12] (signed, 32-bit)`.
  Branch (`jg` at 0x124292): target 0x12424c. Taken when `[sp-8] > ? (signed, 32-bit)`. Falls through when `[sp-8] <= ? (signed, 32-bit)`.
  At the exit (`ret` at 0x12429f), eax=`unknown`.
  Register fact (`xor` at 0x124204): edi=`0x0`.
  Register fact (`xor` at 0x12423d): ebx=`0x0`.
  Tool warning: `0x12422d inc: unsupported effects invalidated`.
  Tool warning: `0x124285 inc: unsupported effects invalidated`.
  Tool warning: `0x12428e inc: unsupported effects invalidated`.
  Local unreached instructions: 0x124212 to 0x124217, ending at 0x124218 (exclusive). Count 6.
  Local unreached instructions: 0x1242a0 to 0x1242a7, ending at 0x1242a8 (exclusive). Count 8.

- `_identEnumerateModules`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124138`.
  Inferred analysis window: 0x124138 to 0x1241f8 (exclusive), 192 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 904, body file offset 0x1c158, window SHA256 `12bdc0118aacde388334c5485d83e49618dff6ef8cf32d2b64ef8ba45268de0f`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124141.
  Four-byte store, pattern 2: target `sp -0x20`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x124146.
  Four-byte load, pattern 3: target `unknown`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x124164, `mov` at 0x12416d.
  Four-byte load, pattern 4: target `sp +0x4`, raw base `ebp`. Count 4 static accesses. Sites: `mov` at 0x124171, `mov` at 0x1241a3, `mov` at 0x1241ac.
  Four-byte store, pattern 5: target `unknown`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124174.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12417e.
  Four-byte store, pattern 7: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124196.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x1241a9.
  Four-byte store, pattern 9: target `sp -0xc`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x1241b2.
  Four-byte load, pattern 10: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1241bb.
  Four-byte load, pattern 11: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1241be.
  Four-byte load, pattern 12: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1241c5.
  Four-byte load, pattern 13: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1241c8.
  Four-byte store, pattern 14: target `unknown`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x1241cb.
  Four-byte store, pattern 15: target `unknown`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x1241ce.
  Four-byte load, pattern 16: target `sp -0x10`, raw base `ebp`. Count 1 static access. Sites: `add` at 0x1241d1.
  Four-byte store, pattern 17: target `sp -0x10`, raw base `ebp`, value `[sp-16]+4`. Count 1 static access. Sites: `add` at 0x1241d1.
  Four-byte load, pattern 18: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1241d6.
  Four-byte load, pattern 19: target `sp -0x10`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x1241d9.
  Four-byte load, pattern 20: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x1241df.
  Call (`call` at 0x12414b): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12417f): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x124150): target 0x124184. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12415a): target 0x12418d. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jg` at 0x124162): target 0x12417e. Taken when `g flags condition (operands unknown)`. Falls through when `not (g flags condition (operands unknown))`.
  Branch (`je` at 0x12416b): target 0x124178. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jle` at 0x12417c): target 0x124160. Taken when `? <= 0x1 (signed, 32-bit)`. Falls through when `? > 0x1 (signed, 32-bit)`.
  Branch (`jle` at 0x12418b): target 0x124158. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  Branch (`jge` at 0x124194): target 0x1241e4. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`jge` at 0x1241a1): target 0x1241de. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`jae` at 0x1241c3): target 0x1241d1. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Branch (`jl` at 0x1241dc): target 0x1241b8. Taken when `[sp-16] < [sp-12] (signed, 32-bit)`. Falls through when `[sp-16] >= [sp-12] (signed, 32-bit)`.
  Branch (`jg` at 0x1241e2): target 0x12419c. Taken when `[sp-8] > ? (signed, 32-bit)`. Falls through when `[sp-8] <= ? (signed, 32-bit)`.
  At the exit (`ret` at 0x1241ef), eax=`unknown`.
  Register fact (`xor` at 0x124144): edi=`0x0`.
  Register fact (`xor` at 0x12415c): ecx=`0x0`.
  Register fact (`xor` at 0x12418d): ecx=`0x0`.
  Tool warning: `0x124177 inc: unsupported effects invalidated`.
  Tool warning: `0x124178 inc: unsupported effects invalidated`.
  Tool warning: `0x1241d5 inc: unsupported effects invalidated`.
  Tool warning: `0x1241de inc: unsupported effects invalidated`.
  Local unreached instructions: 0x124152 to 0x124157, ending at 0x124158 (exclusive). Count 6.
  Local unreached instructions: 0x1241f0 to 0x1241f7, ending at 0x1241f8 (exclusive). Count 8.

- `_identFindByModuleNum`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1242f8`.
  Inferred analysis window: 0x1242f8 to 0x124368 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 906, body file offset 0x1c318, window SHA256 `5aa84dafe545a60607a1f62efbdafaae9d8ffddff00de2c12566a85a16cea835`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1242fe.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124301.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x124309.
  Four-byte load, pattern 4: target `unknown`, raw base `ecx`. Count 1 static access. Sites: `mov` at 0x124320.
  Four-byte store, pattern 5: target `[sp+8] +0x0`, raw base `esi`, value `unknown`. Count 1 static access. Sites: `mov` at 0x12432c.
  Four-byte store, pattern 6: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12433e.
  Call (`call` at 0x12430e): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x12433f): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x124307): target 0x12434d. Taken when `[sp+4] == 0xffffffff`. Falls through when `[sp+4] != 0xffffffff`.
  Jump (`jmp` at 0x124313): target 0x124344. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x124326): target 0x124338. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`je` at 0x12432a): target 0x12432e. Taken when `([sp+8] & [sp+8]) == 0x0`. Falls through when `([sp+8] & [sp+8]) != 0x0`.
  Jump (`jmp` at 0x124330): target 0x12434f. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jle` at 0x12433c): target 0x124320. Taken when `? <= 0x1 (signed, 32-bit)`. Falls through when `? > 0x1 (signed, 32-bit)`.
  Branch (`jne` at 0x12434b): target 0x124318. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  At the exit (`ret` at 0x124358), eax=`unknown`.
  Register fact (`xor` at 0x124318): edx=`0x0`.
  Register fact (`xor` at 0x12434d): eax=`0x0`.
  Tool warning: `0x124338 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x124315 to 0x124317, ending at 0x124318 (exclusive). Count 3.
  Local unreached instructions: 0x124332 to 0x124337, ending at 0x124338 (exclusive). Count 6.
  Local unreached instructions: 0x124359 to 0x124367, ending at 0x124368 (exclusive). Count 15.

- `_identFindConflictingIPAddr`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1243d8`.
  Inferred analysis window: 0x1243d8 to 0x124448 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 932, body file offset 0x1c3f8, window SHA256 `11ddcc0d00f972240da615ce78afaaf303d57f91ed36025216f080d60ddb0cf5`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1243de.
  Four-byte store, pattern 2: target `sp -0x14`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x1243f8.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x124408.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12440c.
  Four-byte load, pattern 5: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x124419.
  Four-byte load, pattern 6: target `[sp+4] +0x8`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x12441c.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x124423.
  Call (`call` at 0x1243fd): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`edi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x12440d): _ffscCompareUniqueIDs, target 0x126f28. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124424): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x1243e3): target 0x1243f8. Taken when `([sp+4] & [sp+4]) != 0x0`. Falls through when `([sp+4] & [sp+4]) == 0x0`.
  Jump (`jmp` at 0x1243e5): target 0x124432. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x1243ea): target 0x124434. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x124402): target 0x124429. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x124417): target 0x124423. Taken when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0`. Falls through when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0`.
  Branch (`je` at 0x124421): target 0x1243e8. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jne` at 0x124430): target 0x124408. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  At the exit (`ret` at 0x12443d), eax=`unknown`.
  Register fact (`xor` at 0x124432): eax=`0x0`.
  Local unreached instructions: 0x1243e7 to 0x1243e7, ending at 0x1243e8 (exclusive). Count 1.
  Local unreached instructions: 0x1243ec to 0x1243f7, ending at 0x1243f8 (exclusive). Count 12.
  Local unreached instructions: 0x124404 to 0x124407, ending at 0x124408 (exclusive). Count 4.
  Local unreached instructions: 0x12443e to 0x124447, ending at 0x124448 (exclusive). Count 10.

- `_identUpdate`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124a28`.
  Inferred analysis window: 0x124a28 to 0x124b58 (exclusive), 304 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 520, body file offset 0x1ca48, window SHA256 `76138d46e7438845c27dc2e398432f43072ba4a767a5d469f0968abf8587f7ba`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124a2d.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x124a30.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `ret(_identFindIdentityNodeByUniqueID)+8`. Count 1 static access. Sites: `push` at 0x124a42.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `0x0`. Count 2 static accesses. Sites: `push` at 0x124a43, `push` at 0x124a9c.
  Four-byte store, pattern 5: target `sp -0x10`, raw base `esp`, value `0x124982`. Count 1 static access. Sites: `push` at 0x124a51.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `0x1c`. Count 2 static accesses. Sites: `push` at 0x124a5b, `push` at 0x124acb.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 2 static accesses. Sites: `push` at 0x124a5d, `push` at 0x124acd.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `ret(_identFindIdentityNodeByUniqueID)+16`. Count 1 static access. Sites: `push` at 0x124a5e.
  Four-byte store, pattern 9: target `sp -0x10`, raw base `esp`, value `0x2c`. Count 1 static access. Sites: `push` at 0x124a68.
  Four-byte store, pattern 10: target `sp -0x10`, raw base `esp`, value `0x1249b5`. Count 1 static access. Sites: `push` at 0x124a78.
  Four-byte store, pattern 11: target `sp -0x10`, raw base `esp`, value `ret(_malloc)+8`. Count 1 static access. Sites: `push` at 0x124a9b.
  Four-byte store, pattern 12: target `sp -0x10`, raw base `esp`, value `0x1249e8`. Count 1 static access. Sites: `push` at 0x124aaa.
  Four-byte store, pattern 13: target `sp -0x10`, raw base `esp`, value `ret(_malloc)`. Count 2 static accesses. Sites: `push` at 0x124ab7, `push` at 0x124ad7.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `esp`, value `ret(_malloc)+16`. Count 1 static access. Sites: `push` at 0x124ace.
  Four-byte store, pattern 15: target `sp -0x14`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x124ad8.
  Call (`call` at 0x124a31): _identFindIdentityNodeByUniqueID, target 0x124f88. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124a45): _clock_gettime, target 0x17e394. Candidate slots: stack arg0=`0x0`, stack arg1=`ret(_identFindIdentityNodeByUniqueID)+8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124a5f): _memcpy, target 0x182918. Candidate slots: stack arg0=`ret(_identFindIdentityNodeByUniqueID)+16`, stack arg1=`[sp+4]`, stack arg2=`0x1c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124a6a): _malloc, target 0x1813f0. Candidate slots: stack arg0=`0x2c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124a7d): _ffscMsgS, target 0x120634. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124a9e): _clock_gettime, target 0x17e394. Candidate slots: stack arg0=`0x0`, stack arg1=`ret(_malloc)+8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124aaf): _ffscMsgS, target 0x120634. Candidate slots: stack arg0=`0x1249e8`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124ab8): _free, target 0x181410. Candidate slots: stack arg0=`ret(_malloc)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124acf): _memcpy, target 0x182918. Candidate slots: stack arg0=`ret(_malloc)+16`, stack arg1=`[sp+4]`, stack arg2=`0x1c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124add): _lstAdd, target 0x192960. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ret(_malloc)`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x124a3d): target 0x124a68. Taken when `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) == 0x0`. Falls through when `(ret(_identFindIdentityNodeByUniqueID) & ret(_identFindIdentityNodeByUniqueID)) != 0x0`.
  Branch (`je` at 0x124a4f): target 0x124a58. Taken when `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0`. Falls through when `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0`.
  Jump (`jmp` at 0x124a56): target 0x124a7d. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x124a66): target 0x124ae7. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x124a76): target 0x124a98. Taken when `(ret(_malloc) & ret(_malloc)) != 0x0`. Falls through when `(ret(_malloc) & ret(_malloc)) == 0x0`.
  Jump (`jmp` at 0x124a87): target 0x124ae7. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x124aa8): target 0x124ac8. Taken when `(ret(_clock_gettime) & ret(_clock_gettime)) == 0x0`. Falls through when `(ret(_clock_gettime) & ret(_clock_gettime)) != 0x0`.
  Jump (`jmp` at 0x124ac2): target 0x124ae7. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x124aef), eax=`unknown`.
  Register fact (`xor` at 0x124a64): eax=`0x0`.
  Register fact (`mov` at 0x124a82): eax=`0xffffffff`.
  Register fact (`mov` at 0x124abd): eax=`0xffffffff`.
  Register fact (`mov` at 0x124ae2): eax=`0x1`.
  Local unreached instructions: 0x124a89 to 0x124a97, ending at 0x124a98 (exclusive). Count 15.
  Local unreached instructions: 0x124ac4 to 0x124ac7, ending at 0x124ac8 (exclusive). Count 4.
  Local unreached instructions: 0x124af0 to 0x124b57, ending at 0x124b58 (exclusive). Count 50.

- `_identInit`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124558`.
  Inferred analysis window: 0x124558 to 0x1246d8 (exclusive), 384 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 933, body file offset 0x1c578, window SHA256 `c611d78c75671d988d5a92d8fb1471c53f4ffba0abad939644f3f87809aa1567`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `esp`, value `0x1c`. Count 1 static access. Sites: `push` at 0x12455b.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x1f3448`. Count 1 static access. Sites: `push` at 0x12455d.
  Four-byte store, pattern 3: target `abs 0x1f05c0`, raw base `unknown`, value `0x0`. Count 1 static access. Sites: `mov` at 0x12456a.
  Four-byte store, pattern 4: target `abs 0x1f398c`, raw base `unknown`, value `0x0`. Count 1 static access. Sites: `mov` at 0x124574.
  Byte store, pattern 5: target `abs 0x1f3448`, raw base `unknown`, value `0x0`. Count 1 static access. Sites: `mov` at 0x12457e.
  Byte store, pattern 6: target `abs 0x1f3449`, raw base `unknown`, value `0x0`. Count 1 static access. Sites: `mov` at 0x124585.
  Byte load, pattern 7: target `unknown`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x124590.
  Byte store, pattern 8: target `unknown`, raw base `edx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124596.
  Four-byte store, pattern 9: target `unknown`, raw base `unknown`, value `0xffffffff`. Count 1 static access. Sites: `mov` at 0x1245b0.
  Four-byte store, pattern 10: target `abs 0x1f3458`, raw base `unknown`, value `0x1`. Count 1 static access. Sites: `mov` at 0x1245c3.
  Four-byte store, pattern 11: target `sp -0x8`, raw base `esp`, value `0x1dfa48`. Count 2 static accesses. Sites: `push` at 0x1245cd, `push` at 0x124645.
  Four-byte store, pattern 12: target `sp -0xc`, raw base `esp`, value `0x1`. Count 2 static accesses. Sites: `push` at 0x1245d2, `push` at 0x12464a.
  Byte load, pattern 13: target `abs 0x1df501`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x1245e0.
  Four-byte store, pattern 14: target `sp -0x8`, raw base `esp`, value `0x124529`. Count 1 static access. Sites: `push` at 0x1245e9.
  Byte load, pattern 15: target `abs 0x1f344d`, raw base `unknown`. Count 1 static access. Sites: `movzx` at 0x1245f6.
  Byte load, pattern 16: target `abs 0x1f344e`, raw base `unknown`. Count 1 static access. Sites: `movzx` at 0x124600.
  Byte load, pattern 17: target `abs 0x1f344f`, raw base `unknown`. Count 1 static access. Sites: `movzx` at 0x12460c.
  Four-byte store, pattern 18: target `abs 0x1dfa48`, raw base `unknown`, value `unknown`. Count 1 static access. Sites: `mov` at 0x124615.
  Four-byte store, pattern 19: target `abs 0x1dfa48`, raw base `unknown`, value `0xf`. Count 1 static access. Sites: `mov` at 0x124627.
  Four-byte store, pattern 20: target `abs 0x1dfa4c`, raw base `unknown`, value `0x64`. Count 1 static access. Sites: `mov` at 0x124631.
  Four-byte store, pattern 21: target `abs 0x1dfa50`, raw base `unknown`, value `0xff000000`. Count 1 static access. Sites: `mov` at 0x12463b.
  Four-byte store, pattern 22: target `sp -0x8`, raw base `esp`, value `0x1f3450`. Count 1 static access. Sites: `push` at 0x124654.
  Four-byte load, pattern 23: target `abs 0x1dfa48`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x124659.
  Four-byte store, pattern 24: target `sp -0xc`, raw base `esp`, value `[0x1dfa48]`. Count 1 static access. Sites: `push` at 0x12465e.
  Four-byte load, pattern 25: target `abs 0x1dfa4c`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12465f.
  Four-byte store, pattern 26: target `sp -0x10`, raw base `esp`, value `[0x1dfa4c]`. Count 1 static access. Sites: `push` at 0x124664.
  Four-byte store, pattern 27: target `sp -0x8`, raw base `esp`, value `0x1f3454`. Count 1 static access. Sites: `push` at 0x12466d.
  Four-byte store, pattern 28: target `sp -0xc`, raw base `esp`, value `0x2`. Count 1 static access. Sites: `push` at 0x124672.
  Four-byte store, pattern 29: target `abs 0x1f3454`, raw base `unknown`, value `0xffffffff`. Count 1 static access. Sites: `mov` at 0x124680.
  Four-byte store, pattern 30: target `sp -0x8`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x12468a.
  Four-byte store, pattern 31: target `sp -0x8`, raw base `esp`, value `0x1f3448`. Count 1 static access. Sites: `push` at 0x124697.
  Call (`call` at 0x124562): _bzero, target 0x174e40. Candidate slots: stack arg0=`0x1f3448`, stack arg1=`0x1c`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1245d4): _nvramRead, target 0x128988. Candidate slots: stack arg0=`0x1`, stack arg1=`0x1dfa48`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1245ee): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x124529`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12464c): _nvramWrite, target 0x128b18. Candidate slots: stack arg0=`0x1`, stack arg1=`0x1dfa48`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124665): _inet_makeaddr_b, target 0x18b198. Candidate slots: stack arg0=`[0x1dfa4c]`, stack arg1=`[0x1dfa48]`, stack arg2=`0x1f3450`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124674): _nvramRead, target 0x128988. Candidate slots: stack arg0=`0x2`, stack arg1=`0x1f3454`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12468f): _lstInit, target 0x192940. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12469c): _identUpdate, target 0x124a28. Candidate slots: stack arg0=`0x1f3448`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x1245a0): target 0x124590. Taken when `? <= 0x5 (signed, 32-bit)`. Falls through when `? > 0x5 (signed, 32-bit)`.
  Branch (`jle` at 0x1245c1): target 0x1245b0. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  Branch (`je` at 0x1245de): target 0x124654. Taken when `(ret(_nvramRead) & ret(_nvramRead)) == 0x0`. Falls through when `(ret(_nvramRead) & ret(_nvramRead)) != 0x0`.
  Branch (`je` at 0x1245e7): target 0x1245f6. Taken when `(([0x1df501]:1 & 0xff) & 0x1) == 0x0`. Falls through when `(([0x1df501]:1 & 0xff) & 0x1) != 0x0`.
  Branch (`jbe` at 0x124625): target 0x124631. Taken when `? <= 0xffffee (unsigned, 32-bit)`. Falls through when `? > 0xffffee (unsigned, 32-bit)`.
  Branch (`je` at 0x12467e): target 0x12468a. Taken when `(ret(_nvramRead) & ret(_nvramRead)) == 0x0`. Falls through when `(ret(_nvramRead) & ret(_nvramRead)) != 0x0`.
  Branch (`jl` at 0x1246a3): target 0x1246b8. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  At the exit (`ret` at 0x1246aa), eax=`0x0`.
  At the exit (`ret` at 0x1246c0), eax=`0xffffffff`.
  Register fact (`xor` at 0x12458c): edx=`0x0`.
  Register fact (`xor` at 0x1245a2): edx=`0x0`.
  Register fact (`mov` at 0x1245a4): eax=`0x1f345c`.
  Register fact (`mov` at 0x1245a9): ecx=`0x1f3460`.
  Register fact (`xor` at 0x1246a5): eax=`0x0`.
  Register fact (`mov` at 0x1246b8): eax=`0xffffffff`.
  Tool warning: `0x12459c inc: unsupported effects invalidated`.
  Tool warning: `0x1245be inc: unsupported effects invalidated`.
  Local unreached instructions: 0x1246ab to 0x1246b7, ending at 0x1246b8 (exclusive). Count 13.
  Local unreached instructions: 0x1246c1 to 0x1246d7, ending at 0x1246d8 (exclusive). Count 11.

- `_identFindIdentityNodeByUniqueID`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x124f88`.
  Inferred analysis window: 0x124f88 to 0x124fe4 (exclusive), 92 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 931, body file offset 0x1cfa8, window SHA256 `2eb08063b2240c748e0ecf7e9da50cc608660ce38d9ff26b27f56a7c04350921`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x124f8d.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `esp`, value `0x1d75a0`. Count 1 static access. Sites: `push` at 0x124f90.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x124fa8.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x124fac.
  Four-byte store, pattern 5: target `sp -0x10`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x124fc8.
  Call (`call` at 0x124f95): _lstFirst, target 0x192ad0. Candidate slots: stack arg0=`0x1d75a0`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124fad): _ffscCompareUniqueIDs, target 0x126f28. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x124fc9): _lstNext, target 0x192b80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x124f9a): target 0x124fce. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x124fb7): target 0x124fc8. Taken when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0`. Falls through when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0`.
  Jump (`jmp` at 0x124fbb): target 0x124fd9. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x124fd5): target 0x124fa8. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  At the exit (`ret` at 0x124fe1), eax=`unknown`.
  Register fact (`xor` at 0x124fd7): eax=`0x0`.
  Local unreached instructions: 0x124f9c to 0x124fa7, ending at 0x124fa8 (exclusive). Count 12.
  Local unreached instructions: 0x124fbd to 0x124fc7, ending at 0x124fc8 (exclusive). Count 11.
  Local unreached instructions: 0x124fe2 to 0x124fe3, ending at 0x124fe4 (exclusive). Count 2.

- `_cmdRACKID`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1141a0`.
  Inferred analysis window: 0x1141a0 to 0x114350 (exclusive), 432 bytes. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 629, body file offset 0xc1c0, window SHA256 `5a04f5eceee3bd02015d7934b80381defd6c9ed8da7a84a4fbc313feda28adc3`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1141a9.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1141ac.
  Four-byte load, pattern 3: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1141af.
  Four-byte store, pattern 4: target `sp -0x40`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x1141b2.
  Four-byte store, pattern 5: target `sp -0x44`, raw base `esp`, value `0x11347f`. Count 1 static access. Sites: `push` at 0x1141b3.
  Four-byte store, pattern 6: target `sp -0x48`, raw base `esp`, value `0x0`. Count 1 static access. Sites: `push` at 0x1141b8.
  Four-byte load, pattern 7: target `abs 0x1f3454`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x1141c8.
  Byte load, pattern 8: target `([0x1f3454] + [sp+12]) +0x4`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x1141ce.
  Four-byte store, pattern 9: target `sp -0x40`, raw base `esp`, value `sp-44`. Count 2 static accesses. Sites: `push` at 0x1141dd, `push` at 0x1141e7.
  Four-byte store, pattern 10: target `sp -0x44`, raw base `esp`, value `[0x1f3454]`. Count 1 static access. Sites: `push` at 0x1141de.
  Four-byte load, pattern 11: target `abs 0x1f3454`, raw base `unknown`. Count 5 static accesses. Sites: `mov` at 0x1141e8, `mov` at 0x114203, `mov` at 0x114270.
  Four-byte store, pattern 12: target `sp -0x44`, raw base `esp`, value `[0x1f3454]`. Count 1 static access. Sites: `push` at 0x1141ed.
  Four-byte store, pattern 13: target `sp -0x48`, raw base `esp`, value `0x1140e5`. Count 1 static access. Sites: `push` at 0x1141ee.
  Four-byte load, pattern 14: target `unknown`, raw base `unknown`. Count 4 static accesses. Sites: `mov` at 0x1141f3, `mov` at 0x114208, `mov` at 0x114307.
  Four-byte load, pattern 15: target `unknown`, raw base `eax`. Count 3 static accesses. Sites: `mov` at 0x1141fa, `mov` at 0x11430e, `mov` at 0x114332.
  Four-byte store, pattern 16: target `sp -0x4c`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x1141fd.
  Four-byte store, pattern 17: target `unknown`, raw base `eax`, value `0xb`. Count 1 static access. Sites: `mov` at 0x11420f.
  Four-byte load, pattern 18: target `[sp+8] +0x20`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x114220.
  Four-byte store, pattern 19: target `sp -0x40`, raw base `esp`, value `0x113503`. Count 1 static access. Sites: `push` at 0x11422a.
  Four-byte store, pattern 20: target `sp -0x40`, raw base `esp`, value `0x0`. Count 2 static accesses. Sites: `push` at 0x114240, `push` at 0x1142a0.
  Four-byte store, pattern 21: target `sp -0x44`, raw base `esp`, value `[sp+12]`. Count 1 static access. Sites: `push` at 0x114242.
  Four-byte store, pattern 22: target `sp -0x40`, raw base `esp`, value `ret(_destNumRacks)`. Count 1 static access. Sites: `push` at 0x114250.
  Four-byte store, pattern 23: target `sp -0x44`, raw base `esp`, value `0x1140f0`. Count 1 static access. Sites: `push` at 0x114251.
  Four-byte store, pattern 24: target `sp -0x40`, raw base `esp`, value `0x114121`. Count 1 static access. Sites: `push` at 0x11425e.
  Byte load, pattern 25: target `([0x1f3454] + [sp+12]) +0x4`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x114275.
  Four-byte store, pattern 26: target `unknown`, raw base `edi`, value `unknown`. Count 1 static access. Sites: `rep movsd` at 0x11429e.
  Four-byte load, pattern 27: target `abs 0x1f3448`, raw base `esi`. Count 1 static access. Sites: `rep movsd` at 0x11429e.
  Four-byte store, pattern 28: target `sp -0x44`, raw base `esp`, value `sp-48`. Count 1 static access. Sites: `push` at 0x1142a5.
  Four-byte store, pattern 29: target `sp -0x48`, raw base `esp`, value `ret(_strtok_r)`. Count 1 static access. Sites: `push` at 0x1142a6.
  Four-byte store, pattern 30: target `sp -0x14`, raw base `ebp`, value `ret(_strtoul)`. Count 1 static access. Sites: `mov` at 0x1142af.
  Four-byte load, pattern 31: target `sp -0x30`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1142b2.
  Byte load, pattern 32: target `[sp-48] +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x1142b5.
  Four-byte load, pattern 33: target `sp -0x14`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x1142bb.
  Four-byte store, pattern 34: target `sp -0x40`, raw base `esp`, value `ret(_strtok_r)`. Count 1 static access. Sites: `push` at 0x1142c1.
  Four-byte store, pattern 35: target `sp -0x44`, raw base `esp`, value `0x11412c`. Count 1 static access. Sites: `push` at 0x1142c2.
  Four-byte store, pattern 36: target `sp -0x40`, raw base `esp`, value `0x113498`. Count 1 static access. Sites: `push` at 0x1142cf.
  Four-byte store, pattern 37: target `sp -0x40`, raw base `esp`, value `sp-32`. Count 1 static access. Sites: `push` at 0x1142e3.
  Four-byte store, pattern 38: target `sp -0x40`, raw base `esp`, value `0x114141`. Count 1 static access. Sites: `push` at 0x1142f0.
  Four-byte store, pattern 39: target `sp -0x40`, raw base `esp`, value `0x11417b`. Count 1 static access. Sites: `push` at 0x1142fd.
  Four-byte store, pattern 40: target `sp -0x44`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x114311.
  Four-byte store, pattern 41: target `sp -0x40`, raw base `esp`, value `[0x1f3454]`. Count 1 static access. Sites: `push` at 0x114325.
  Four-byte store, pattern 42: target `sp -0x44`, raw base `esp`, value `0x11418a`. Count 1 static access. Sites: `push` at 0x114326.
  Four-byte store, pattern 43: target `sp -0x48`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x114335.
  Call (`call` at 0x1141ba): _strtok_r, target 0x182ed0. Candidate slots: stack arg0=`0x0`, stack arg1=`0x11347f`, stack arg2=`[sp+4]`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`edi`.
  Call (`call` at 0x1141df): _ffscRackIDToString, target 0x1274b8. Candidate slots: stack arg0=`[0x1f3454]`, stack arg1=`sp-44`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1141fe): _sprintf, target 0x1755a0. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1140e5`, stack arg2=`[0x1f3454]`, stack arg3=`sp-44`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x114243): _destNumRacks, target 0x120900. Candidate slots: stack arg0=`[sp+12]`, stack arg1=`0x0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x114256): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x1140f0`, stack arg1=`ret(_destNumRacks)`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1142a7): _strtoul, target 0x182714. Candidate slots: stack arg0=`ret(_strtok_r)`, stack arg1=`sp-48`, stack arg2=`0x0`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1142c7): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x11412c`, stack arg1=`ret(_strtok_r)`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1142e4): _addrdRequestRackID, target 0x10dd20. Candidate slots: stack arg0=`sp-32`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1142f5): _ffscMsg, target 0x120504. Candidate slots: stack arg0=`0x114141`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x114312): _sprintf, target 0x1755a0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x114336): _sprintf, target 0x1755a0. Candidate slots: stack arg0=`unknown`, stack arg1=`0x11418a`, stack arg2=`[0x1f3454]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x1141c6): target 0x114220. Taken when `(ret(_strtok_r) & ret(_strtok_r)) != 0x0`. Falls through when `(ret(_strtok_r) & ret(_strtok_r)) == 0x0`.
  Branch (`je` at 0x1141d4): target 0x11427d. Taken when `(([([0x1f3454] + [sp+12])+4]:1 & 0xff) & ([([0x1f3454] + [sp+12])+4]:1 & 0xff)) == 0x0`. Falls through when `(([([0x1f3454] + [sp+12])+4]:1 & 0xff) & ([([0x1f3454] + [sp+12])+4]:1 & 0xff)) != 0x0`.
  Jump (`jmp` at 0x114216): target 0x11427d. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jg` at 0x114228): target 0x114240. Taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)`. Falls through when `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)`.
  Jump (`jmp` at 0x11422f): target 0x114302. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jle` at 0x11424e): target 0x114270. Taken when `ret(_destNumRacks) <= 0x1 (signed, 32-bit)`. Falls through when `ret(_destNumRacks) > 0x1 (signed, 32-bit)`.
  Jump (`jmp` at 0x114263): target 0x114302. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x11427b): target 0x114290. Taken when `(([([0x1f3454] + [sp+12])+4]:1 & 0xff) & ([([0x1f3454] + [sp+12])+4]:1 & 0xff)) != 0x0`. Falls through when `(([([0x1f3454] + [sp+12])+4]:1 & 0xff) & ([([0x1f3454] + [sp+12])+4]:1 & 0xff)) == 0x0`.
  Jump (`jmp` at 0x114282): target 0x11433d. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x1142b9): target 0x1142c1. Taken when `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) != 0x0`. Falls through when `(([[sp-48]]:1 & 0xff) & ([[sp-48]]:1 & 0xff)) == 0x0`.
  Branch (`jbe` at 0x1142bf): target 0x1142e0. Taken when `ret(_strtoul) <= 0xa (unsigned, 32-bit)`. Falls through when `ret(_strtoul) > 0xa (unsigned, 32-bit)`.
  Jump (`jmp` at 0x1142d4): target 0x114302. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x1142ee): target 0x114320. Taken when `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) == 0x0`. Falls through when `(ret(_addrdRequestRackID) & ret(_addrdRequestRackID)) != 0x0`.
  Jump (`jmp` at 0x114317): target 0x11433b. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x114346), eax=`unknown`.
  Register fact (`mov` at 0x11427d): eax=`0x1`.
  Register fact (`mov` at 0x114293): esi=`0x1f3448`.
  Register fact (`mov` at 0x114299): ecx=`0x7`.
  Register fact (`xor` at 0x11433b): eax=`0x0`.
  Tool warning: `0x11429e rep movsd: unsupported effects invalidated`.
  Local unreached instructions: 0x114218 to 0x11421f, ending at 0x114220 (exclusive). Count 8.
  Local unreached instructions: 0x114234 to 0x11423f, ending at 0x114240 (exclusive). Count 12.
  Local unreached instructions: 0x114268 to 0x11426f, ending at 0x114270 (exclusive). Count 8.
  Local unreached instructions: 0x114287 to 0x11428f, ending at 0x114290 (exclusive). Count 9.
  Local unreached instructions: 0x1142d6 to 0x1142df, ending at 0x1142e0 (exclusive). Count 10.
  Local unreached instructions: 0x114319 to 0x11431f, ending at 0x114320 (exclusive). Count 7.
  Local unreached instructions: 0x114347 to 0x11434f, ending at 0x114350 (exclusive). Count 4.
  Encoding (`rep movsd` at 0x11429e): bytes `f3a5`, raw prefix `0xf3`, raw operands `dword ptr es:[edi], dword ptr [esi]`.
  REP limits (`rep movsd` at 0x11429e): 2 static modeled operand records with unsupported effects and unknown completed runtime accesses, repetition count, iteration, direction, aggregate transfer, segment bases, physical address space and hardware effects.
