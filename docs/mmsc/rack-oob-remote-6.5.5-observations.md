# MMSC rack, out-of-band and remote observations

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

The [full facts](disasm/rack-oob-remote-6.5.5-observations-facts.txt) retain every access, instruction and warning.

- `_identClearTable`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1250f0`.
  Inferred analysis window: 0x1250f0 to 0x125140 (exclusive), 80 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 940, body file offset 0x1d110, window SHA256 `aa61bcdf727039721424b7d04154fbee583145d2b1039cab41b556b6287459e7`.
  Four-byte store, pattern 1: target `sp -0xc`, raw base `esp`, value `ret(_lstFirst)`. Count 1 static access. Sites: `push` at 0x125100.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `esp`, value `0x1d8c78`. Count 1 static access. Sites: `push` at 0x125101.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `ebp`, value `ret(_lstFirst)`. Count 1 static access. Sites: `mov` at 0x125106.
  Four-byte load, pattern 4: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x125111.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `[sp-8]`. Count 1 static access. Sites: `push` at 0x125114.
  Four-byte store, pattern 6: target `sp -0xc`, raw base `esp`, value `0x1d8c78`. Count 1 static access. Sites: `push` at 0x12511d.
  Call (`call` at 0x125109): _lstDelete, target 0x194080. Candidate slots: stack arg0=`0x1d8c78`, stack arg1=`ret(_lstFirst)`, stack arg2=`ret(_lstFirst)`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x125115): _free, target 0x182aa0. Candidate slots: stack arg0=`[sp-8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x125122): _lstFirst, target 0x194160. Candidate slots: stack arg0=`0x1d8c78`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x1250f6): target 0x12511d. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x12512c): target 0x125100. Taken when `(ret(_lstFirst) & ret(_lstFirst)) != 0x0`. Falls through when `(ret(_lstFirst) & ret(_lstFirst)) == 0x0`.
  At the exit (`ret` at 0x125133), eax=`0x0`.
  Register fact (`xor` at 0x12512e): eax=`0x0`.
  Local unreached instructions: 0x1250f8 to 0x1250ff, ending at 0x125100 (exclusive). Count 8.
  Local unreached instructions: 0x125134 to 0x12513f, ending at 0x125140 (exclusive). Count 12.

- `_identFindIdentityNodeByUniqueID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x125fa0`.
  Inferred analysis window: 0x125fa0 to 0x125ffc (exclusive), 92 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 941, body file offset 0x1dfc0, window SHA256 `ddec88c1bdccf3ce545ee84e55cfc115e79bc3e6889f5c6e62e3d9c5d7e8bfc0`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x125fa5.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `esp`, value `0x1d8c78`. Count 1 static access. Sites: `push` at 0x125fa8.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x125fc0.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x125fc4.
  Four-byte store, pattern 5: target `sp -0x10`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x125fe0.
  Call (`call` at 0x125fad): _lstFirst, target 0x194160. Candidate slots: stack arg0=`0x1d8c78`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x125fc5): _ffscCompareUniqueIDs, target 0x127f40. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x125fe1): _lstNext, target 0x194210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x125fb2): target 0x125fe6. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x125fcf): target 0x125fe0. Taken when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) != 0x0`. Falls through when `(ret(_ffscCompareUniqueIDs) & ret(_ffscCompareUniqueIDs)) == 0x0`.
  Jump (`jmp` at 0x125fd3): target 0x125ff1. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x125fed): target 0x125fc0. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  At the exit (`ret` at 0x125ff9), eax=`unknown`.
  Register fact (`xor` at 0x125fef): eax=`0x0`.
  Local unreached instructions: 0x125fb4 to 0x125fbf, ending at 0x125fc0 (exclusive). Count 12.
  Local unreached instructions: 0x125fd5 to 0x125fdf, ending at 0x125fe0 (exclusive). Count 11.
  Local unreached instructions: 0x125ffa to 0x125ffb, ending at 0x125ffc (exclusive). Count 2.

- `_ffscParseRackID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x128340`.
  Inferred analysis window: 0x128340 to 0x1283e0 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 704, body file offset 0x20360, window SHA256 `8426463652b3bc933cffdc62ef9216d280af3352f56a121c72cd6b85646b2f10`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x128348.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12834b.
  Four-byte store, pattern 3: target `sp -0x18`, raw base `esp`, value `sp-9`. Count 1 static access. Sites: `push` at 0x128351.
  Four-byte store, pattern 4: target `sp -0x1c`, raw base `esp`, value `sp-8`. Count 1 static access. Sites: `push` at 0x128355.
  Four-byte store, pattern 5: target `sp -0x20`, raw base `esp`, value `0x128324`. Count 1 static access. Sites: `push` at 0x128356.
  Four-byte store, pattern 6: target `sp -0x24`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12835b.
  Four-byte load, pattern 7: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x128369.
  Four-byte load, pattern 8: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x12836f.
  Four-byte store, pattern 9: target `sp -0x18`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x128375.
  Four-byte store, pattern 10: target `sp -0x1c`, raw base `esp`, value `0x128329`. Count 1 static access. Sites: `push` at 0x128376.
  Four-byte load, pattern 11: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x128394.
  Four-byte store, pattern 12: target `[sp+8] +0x0`, raw base `esi`, value `[sp-8]`. Count 1 static access. Sites: `mov` at 0x128397.
  Call (`call` at 0x12835c): _sscanf, target 0x1779e0. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0x128324`, stack arg2=`sp-8`, stack arg3=`sp-9`, stack arg4=`ebx`, stack arg5=`esi`.
  Call (`call` at 0x12837b): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x128329`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x128367): target 0x128375. Taken when `ret(_sscanf) != 0x1`. Falls through when `ret(_sscanf) == 0x1`.
  Branch (`ja` at 0x12836d): target 0x128375. Taken when `[sp-8] > 0x14 (unsigned, 32-bit)`. Falls through when `[sp-8] <= 0x14 (unsigned, 32-bit)`.
  Branch (`jne` at 0x128373): target 0x128390. Taken when `[sp-8] != 0x0`. Falls through when `[sp-8] == 0x0`.
  Jump (`jmp` at 0x128385): target 0x12839b. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x128392): target 0x128399. Taken when `([sp+8] & [sp+8]) == 0x0`. Falls through when `([sp+8] & [sp+8]) != 0x0`.
  At the exit (`ret` at 0x1283a3), eax=`unknown`.
  Register fact (`mov` at 0x128380): eax=`0xffffffff`.
  Register fact (`xor` at 0x128399): eax=`0x0`.
  Local unreached instructions: 0x128387 to 0x12838f, ending at 0x128390 (exclusive). Count 9.
  Local unreached instructions: 0x1283a4 to 0x1283df, ending at 0x1283e0 (exclusive). Count 21.

- `_ffscRackIDToString`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1284d0`.
  Inferred analysis window: 0x1284d0 to 0x128550 (exclusive), 128 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 703, body file offset 0x204f0, window SHA256 `df97ab6929b105d5460d39ec2e44c3b030497ed4c3ba9a2700dd1d64e24d41e1`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1284d4.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1284d7.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `esp`, value `0x12813e`. Count 1 static access. Sites: `push` at 0x1284e8.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x1284ed.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x128500.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `0x1284be`. Count 1 static access. Sites: `push` at 0x128501.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x128506.
  Call (`call` at 0x1284ee): _strcpy, target 0x1841e4. Candidate slots: stack arg0=`unknown`, stack arg1=`0x12813e`, stack arg2=`ebx`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x128507): _sprintf, target 0x176c30. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1284be`, stack arg2=`[sp+4]`, stack arg3=`ebx`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`jne` at 0x1284dc): target 0x1284e3. Taken when `([sp+8] & [sp+8]) != 0x0`. Falls through when `([sp+8] & [sp+8]) == 0x0`.
  Branch (`jne` at 0x1284e6): target 0x128500. Taken when `[sp+4] != 0xffffffff`. Falls through when `[sp+4] == 0xffffffff`.
  Jump (`jmp` at 0x1284f3): target 0x12850c. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x128514), eax=`unknown`.
  Register fact (`mov` at 0x1284de): ebx=`0x1d8c94`.
  Local unreached instructions: 0x1284f5 to 0x1284ff, ending at 0x128500 (exclusive). Count 11.
  Local unreached instructions: 0x128515 to 0x12854f, ending at 0x128550 (exclusive). Count 35.

- `_destNumRacks`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x121938`.
  Inferred analysis window: 0x121938 to 0x1219e8 (exclusive), 176 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 674, body file offset 0x19958, window SHA256 `0925744fb6c141a69eb12e4eee84984284355538d46e3f769ba8a2de3464fc48`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12193d.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x121940.
  Byte load, pattern 3: target `unknown`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x121948.
  Four-byte store, pattern 4: target `[sp+8] +0x0`, raw base `ebx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x121955.
  Branch (`je` at 0x12194e): target 0x121957. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`je` at 0x121953): target 0x121957. Taken when `([sp+8] & [sp+8]) == 0x0`. Falls through when `([sp+8] & [sp+8]) != 0x0`.
  Branch (`jle` at 0x12195b): target 0x121948. Taken when `? <= 0x14 (signed, 32-bit)`. Falls through when `? > 0x14 (signed, 32-bit)`.
  At the exit (`ret` at 0x121967), eax=`unknown`.
  Register fact (`xor` at 0x121943): ecx=`0x0`.
  Register fact (`xor` at 0x121945): edx=`0x0`.
  Tool warning: `0x121950 inc: unsupported effects invalidated`.
  Tool warning: `0x121957 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x121968 to 0x1219e7, ending at 0x1219e8 (exclusive). Count 62.

- `_setOOBStatusStart`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1190f8`.
  Inferred analysis window: 0x1190f8 to 0x119148 (exclusive), 80 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 762, body file offset 0x11118, window SHA256 `7b9957db73cbc6efb5645666b1ff0c9fabd2d8bd4436018e1b04beb101934e45`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x1190fb.
  Four-byte load, pattern 2: target `[sp+4] +0xa0`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x1190fe, `mov` at 0x11911b.
  Four-byte load, pattern 3: target `abs 0x1f1cb4`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x119104.
  Four-byte load, pattern 4: target `abs 0x1e1380`, raw base `unknown`. Count 2 static accesses. Sites: `mov` at 0x11910c, `mov` at 0x119129.
  Four-byte store, pattern 5: target `abs 0x1e1380`, raw base `unknown`, value `unknown`. Count 2 static accesses. Sites: `mov` at 0x119116, `mov` at 0x119133.
  Four-byte load, pattern 6: target `abs 0x1f1cc0`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x119121.
  Branch (`jne` at 0x11910a): target 0x11911b. Taken when `[0x1f1cb4] != [[sp+4]+160]`. Falls through when `[0x1f1cb4] == [[sp+4]+160]`.
  Branch (`jne` at 0x119127): target 0x119138. Taken when `[0x1f1cc0] != [[sp+4]+160]`. Falls through when `[0x1f1cc0] == [[sp+4]+160]`.
  At the exit (`ret` at 0x11913b), eax=`unknown`.
  Local unreached instructions: 0x11913c to 0x119147, ending at 0x119148 (exclusive). Count 12.

- `_setOOBStatusFinish`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x119148`.
  Inferred analysis window: 0x119148 to 0x119198 (exclusive), 80 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 764, body file offset 0x11168, window SHA256 `09524cf875318667969f7c74f5e7b820d28cb9bee59c7344e909c9b7a2300a19`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11914b.
  Four-byte load, pattern 2: target `[sp+4] +0xa0`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x11914e, `mov` at 0x11916b.
  Four-byte load, pattern 3: target `abs 0x1f1cb4`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x119154.
  Four-byte load, pattern 4: target `abs 0x1e1380`, raw base `unknown`. Count 2 static accesses. Sites: `mov` at 0x11915c, `mov` at 0x119179.
  Four-byte store, pattern 5: target `abs 0x1e1380`, raw base `unknown`, value `unknown`. Count 2 static accesses. Sites: `mov` at 0x119166, `mov` at 0x119183.
  Four-byte load, pattern 6: target `abs 0x1f1cc0`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x119171.
  Branch (`jne` at 0x11915a): target 0x11916b. Taken when `[0x1f1cb4] != [[sp+4]+160]`. Falls through when `[0x1f1cb4] == [[sp+4]+160]`.
  Branch (`jne` at 0x119177): target 0x119188. Taken when `[0x1f1cc0] != [[sp+4]+160]`. Falls through when `[0x1f1cc0] == [[sp+4]+160]`.
  At the exit (`ret` at 0x11918b), eax=`unknown`.
  Local unreached instructions: 0x11918c to 0x119197, ending at 0x119198 (exclusive). Count 12.

- `_oobChecksumMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12a184`.
  Inferred analysis window: 0x12a184 to 0x12a294 (exclusive), 272 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 770, body file offset 0x221a4, window SHA256 `1e010456386865e784506394472d93122e638b61b4d340ad3e908973099b5571`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12a189.
  Byte load, pattern 2: target `[sp+4] +0x2`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12a18c.
  Byte load, pattern 3: target `[sp+4] +0x3`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12a194.
  Byte load, pattern 4: target `unknown`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x12a1b0.
  Branch (`jge` at 0x12a1ad): target 0x12a1bf. Taken when `0x1 >= ? (signed, 32-bit)`. Falls through when `0x1 < ? (signed, 32-bit)`.
  Branch (`jl` at 0x12a1bd): target 0x12a1b0. Taken when `? < ? (signed, 32-bit)`. Falls through when `? >= ? (signed, 32-bit)`.
  At the exit (`ret` at 0x12a1ca), eax=`unknown`.
  Register fact (`xor` at 0x12a1a4): ecx=`0x0`.
  Register fact (`mov` at 0x12a1a6): edx=`0x1`.
  Tool warning: `0x12a1ba inc: unsupported effects invalidated`.
  Local unreached instructions: 0x12a1cb to 0x12a293, ending at 0x12a294 (exclusive). Count 77.

- `_oobProcessMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12a294`.
  Inferred analysis window: 0x12a294 to 0x12a424 (exclusive), 400 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 861, body file offset 0x222b4, window SHA256 `3fdce0f4a3684c8cfcd29099140b371ed6d92541e88c72efae7343da7b617289`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12a299.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12a29c.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12a2a3.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `esp`, value `0x12a1cb`. Count 1 static access. Sites: `push` at 0x12a2a4.
  Byte load, pattern 5: target `[sp+8] +0x0`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x12a2b4, `mov` at 0x12a2ba.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `unknown`. Count 3 static accesses. Sites: `push` at 0x12a2c1, `push` at 0x12a301, `push` at 0x12a355.
  Four-byte store, pattern 7: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 4 static accesses. Sites: `push` at 0x12a2c2, `push` at 0x12a356, `push` at 0x12a3a5.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `0x12a1f5`. Count 1 static access. Sites: `push` at 0x12a2c3.
  Four-byte store, pattern 9: target `sp -0x10`, raw base `esp`, value `[sp+8]`. Count 2 static accesses. Sites: `push` at 0x12a2d4, `push` at 0x12a3a4.
  Byte load, pattern 10: target `[sp+8] +0x2`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x12a2df, `mov` at 0x12a302.
  Byte load, pattern 11: target `[sp+8] +0x3`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x12a2e7, `mov` at 0x12a30a.
  Byte load, pattern 12: target `unknown`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x12a2f7, `mov` at 0x12a31a.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12a322.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12a323.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `esp`, value `0x12a22f`. Count 1 static access. Sites: `push` at 0x12a324.
  Four-byte store, pattern 16: target `sp -0x10`, raw base `esp`, value `0x1b1010`. Count 1 static access. Sites: `push` at 0x12a331.
  Byte load, pattern 17: target `abs 0x1e0be1`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x12a344.
  Byte load, pattern 18: target `[sp+8] +0x1`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x12a34d, `mov` at 0x12a364.
  Four-byte store, pattern 19: target `sp -0x18`, raw base `esp`, value `0x12a26e`. Count 1 static access. Sites: `push` at 0x12a357.
  Four-byte store, pattern 20: target `sp -0x10`, raw base `esp`, value `0x1b101a`. Count 1 static access. Sites: `push` at 0x12a394.
  Four-byte load, pattern 21: target `[sp+4] +0x3c`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12a3b4.
  Four-byte store, pattern 22: target `sp -0x10`, raw base `esp`, value `[sp+8]+4`. Count 1 static access. Sites: `push` at 0x12a3c1.
  Four-byte store, pattern 23: target `sp -0x10`, raw base `esp`, value `0x1b101f`. Count 1 static access. Sites: `push` at 0x12a3d4.
  Four-byte store, pattern 24: target `sp -0x10`, raw base `esp`, value `0x1b1015`. Count 1 static access. Sites: `push` at 0x12a3e4.
  Four-byte load, pattern 25: target `[sp+4] +0x6c`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x12a3e9.
  Four-byte load, pattern 26: target `[[sp+4]+108] +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x12a3ec.
  Four-byte store, pattern 27: target `sp -0x14`, raw base `esp`, value `[[[sp+4]+108]]`. Count 1 static access. Sites: `push` at 0x12a3ee.
  Call (`call` at 0x12a2a9): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12a1cb`, stack arg1=`[sp+4]`, stack arg2=`ebx`, stack arg3=`esi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Call (`call` at 0x12a2c8): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12a1f5`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`ebp`.
  Call (`call` at 0x12a2d5): _oobChecksumMsg, target 0x12a184. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a329): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12a22f`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a35c): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12a26e`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a3a6): _oobSendGraphicsCmd, target 0x12a544. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a3c3): _oobSendFFSCCmd, target 0x12a424. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`[sp+8]+4`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a3ef): _oobSendMsg, target 0x12a694. Candidate slots: stack arg0=`[[[sp+4]+108]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x12a2a1): target 0x12a2b4. Taken when `([sp+8] & [sp+8]) != 0x0`. Falls through when `([sp+8] & [sp+8]) == 0x0`.
  Jump (`jmp` at 0x12a2ae): target 0x12a3f4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12a2b8): target 0x12a2d4. Taken when `([[sp+8]]:1 & 0xff) == 0xa0`. Falls through when `([[sp+8]]:1 & 0xff) != 0xa0`.
  Jump (`jmp` at 0x12a2cd): target 0x12a3f4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12a2fc): target 0x12a344. Taken when `? == ?`. Falls through when `? != ?`.
  Jump (`jmp` at 0x12a336): target 0x12a3e9. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12a34b): target 0x12a364. Taken when `(([0x1e0be1]:1 & 0xff) & 0x8) == 0x0`. Falls through when `(([0x1e0be1]:1 & 0xff) & 0x8) != 0x0`.
  Branch (`jg` at 0x12a36f): target 0x12a384. Taken when `? > 0xb (signed, 32-bit)`. Falls through when `? <= 0xb (signed, 32-bit)`.
  Branch (`jge` at 0x12a374): target 0x12a3a4. Taken when `? >= 0x1 (signed, 32-bit)`. Falls through when `? < 0x1 (signed, 32-bit)`.
  Branch (`je` at 0x12a378): target 0x12a394. Taken when `(? & ?) == 0x0`. Falls through when `(? & ?) != 0x0`.
  Jump (`jmp` at 0x12a37a): target 0x12a3e4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x12a387): target 0x12a3b4. Taken when `? == 0x3d`. Falls through when `? != 0x3d`.
  Branch (`je` at 0x12a38c): target 0x12a3d4. Taken when `? == 0x3e`. Falls through when `? != 0x3e`.
  Jump (`jmp` at 0x12a38e): target 0x12a3e4. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x12a399): target 0x12a3e9. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x12a3ab): target 0x12a3f4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x12a3bc): target 0x12a3e4. Taken when `([[sp+4]+60] & 0x40000) != 0x0`. Falls through when `([[sp+4]+60] & 0x40000) == 0x0`.
  Jump (`jmp` at 0x12a3c8): target 0x12a3f4. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x12a3d9): target 0x12a3e9. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x12a3fc), eax=`unknown`.
  Local unreached instructions: 0x12a2b3 to 0x12a2b3, ending at 0x12a2b4 (exclusive). Count 1.
  Local unreached instructions: 0x12a2d2 to 0x12a2d3, ending at 0x12a2d4 (exclusive). Count 2.
  Local unreached instructions: 0x12a33b to 0x12a343, ending at 0x12a344 (exclusive). Count 9.
  Local unreached instructions: 0x12a37c to 0x12a383, ending at 0x12a384 (exclusive). Count 8.
  Local unreached instructions: 0x12a390 to 0x12a393, ending at 0x12a394 (exclusive). Count 4.
  Local unreached instructions: 0x12a39b to 0x12a3a3, ending at 0x12a3a4 (exclusive). Count 9.
  Local unreached instructions: 0x12a3ad to 0x12a3b3, ending at 0x12a3b4 (exclusive). Count 7.
  Local unreached instructions: 0x12a3ca to 0x12a3d3, ending at 0x12a3d4 (exclusive). Count 10.
  Local unreached instructions: 0x12a3db to 0x12a3e3, ending at 0x12a3e4 (exclusive). Count 9.
  Local unreached instructions: 0x12a3fd to 0x12a423, ending at 0x12a424 (exclusive). Count 20.

- `_oobSendMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12a694`.
  Inferred analysis window: 0x12a694 to 0x12a728 (exclusive), 148 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 1041, body file offset 0x226b4, window SHA256 `b4d84c0b2aef9e67f0279bfb86b298b8a357c8f3a60c18e30c672392f7d3ff92`.
  Four-byte load, pattern 1: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12a69a.
  Byte load, pattern 2: target `[sp+8] +0x2`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x12a69d.
  Byte load, pattern 3: target `[sp+8] +0x3`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x12a6a5.
  Four-byte load, pattern 4: target `abs 0x1e06d0`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12a6bc.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `[0x1e06d0]`. Count 1 static access. Sites: `push` at 0x12a6c1.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12a6c2.
  Four-byte store, pattern 7: target `sp -0x1c`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12a6c6.
  Four-byte load, pattern 8: target `sp +0x4`, raw base `ebp`. Count 2 static accesses. Sites: `mov` at 0x12a6c7, `mov` at 0x12a6f4.
  Four-byte store, pattern 9: target `sp -0x20`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12a6ca.
  Four-byte load, pattern 10: target `ret(___errno) +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x12a6dc.
  Four-byte store, pattern 11: target `sp -0x14`, raw base `esp`, value `0x12a645`. Count 1 static access. Sites: `push` at 0x12a6e3.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12a6f7.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `esp`, value `0x12a663`. Count 1 static access. Sites: `push` at 0x12a6f8.
  Call (`call` at 0x12a6cb): _timeoutWrite, target 0x12ded8. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`[0x1e06d0]`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a6d7): ___errno, target 0x175028. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a6e8): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12a645`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12a6fd): _ffscMsgS, target 0x12166c. Candidate slots: stack arg0=`0x12a663`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x12a6b9): target 0x12a71c. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  Branch (`jge` at 0x12a6d5): target 0x12a714. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`jne` at 0x12a6e1): target 0x12a6f4. Taken when `[ret(___errno)] != 0x4`. Falls through when `[ret(___errno)] == 0x4`.
  Jump (`jmp` at 0x12a6ed): target 0x12a702. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`jmp` at 0x12a707): target 0x12a71e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jg` at 0x12a71a): target 0x12a6bc. Taken when `g flags condition (operands unknown)`. Falls through when `not (g flags condition (operands unknown))`.
  At the exit (`ret` at 0x12a727), eax=`unknown`.
  Register fact (`xor` at 0x12a6b5): esi=`0x0`.
  Register fact (`mov` at 0x12a702): eax=`0xffffffff`.
  Register fact (`xor` at 0x12a71c): eax=`0x0`.
  Local unreached instructions: 0x12a6ef to 0x12a6f3, ending at 0x12a6f4 (exclusive). Count 5.
  Local unreached instructions: 0x12a709 to 0x12a713, ending at 0x12a714 (exclusive). Count 11.

- `_ttyGetInfo`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12e498`.
  Inferred analysis window: 0x12e498 to 0x12e4d8 (exclusive), 64 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 816, body file offset 0x264b8, window SHA256 `ee21687998e7d6ca3562aaa3a91d4d52699d0afe6d25cda624df920a1ece1e06`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e49b.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12e49e.
  Four-byte load, pattern 3: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12e4bc.
  Four-byte store, pattern 4: target `[sp+8] +0x0`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x12e4c3.
  Branch (`jbe` at 0x12e4a4): target 0x12e4b8. Taken when `[sp+4] <= 0x5 (unsigned, 32-bit)`. Falls through when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  Branch (`je` at 0x12e4ba): target 0x12e4c5. Taken when `([sp+8] & [sp+8]) == 0x0`. Falls through when `([sp+8] & [sp+8]) != 0x0`.
  At the exit (`ret` at 0x12e4ab), eax=`0x0`.
  At the exit (`ret` at 0x12e4d2), eax=`unknown`.
  Register fact (`xor` at 0x12e4a6): eax=`0x0`.
  Local unreached instructions: 0x12e4ac to 0x12e4b7, ending at 0x12e4b8 (exclusive). Count 12.
  Local unreached instructions: 0x12e4d3 to 0x12e4d7, ending at 0x12e4d8 (exclusive). Count 5.

- `_userReleaseRemote`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12f878`.
  Inferred analysis window: 0x12f878 to 0x12f928 (exclusive), 176 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 749, body file offset 0x27898, window SHA256 `c8b44b21c12b2839d241128a7ae6c31c69bb9b6e1cf6d27d64da56e2bc5b8f48`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12f87b.
  Four-byte load, pattern 2: target `abs 0x1e0ec8`, raw base `unknown`. Count 1 static access. Sites: `cmp` at 0x12f87e.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12f886.
  Four-byte store, pattern 4: target `sp -0xc`, raw base `esp`, value `0x12f845`. Count 1 static access. Sites: `push` at 0x12f887.
  Four-byte load, pattern 5: target `[sp+4] +0x14`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x12f898, `mov` at 0x12f8a0.
  Four-byte store, pattern 6: target `[sp+4] +0x14`, raw base `edx`, value `unknown`. Count 2 static accesses. Sites: `mov` at 0x12f89d, `mov` at 0x12f8a5.
  Four-byte store, pattern 7: target `abs 0x1f224c`, raw base `unknown`, value `0x1`. Count 1 static access. Sites: `mov` at 0x12f8a8.
  Four-byte store, pattern 8: target `[sp+4] +0x1c`, raw base `edx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x12f8b2.
  Four-byte store, pattern 9: target `[sp+4] +0x20`, raw base `edx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x12f8b9.
  Four-byte store, pattern 10: target `sp -0x8`, raw base `esp`, value `[sp+4]+36`. Count 1 static access. Sites: `push` at 0x12f8c3.
  Call (`call` at 0x12f88c): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12f845`, stack arg1=`[sp+4]`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12f8c4): _destClear, target 0x1218c8. Candidate slots: stack arg0=`[sp+4]+36`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12f884): target 0x12f898. Taken when `[0x1e0ec8] == [sp+4]`. Falls through when `[0x1e0ec8] != [sp+4]`.
  At the exit (`ret` at 0x12f894), eax=`ret(_ffscMsg)`.
  At the exit (`ret` at 0x12f8cc), eax=`ret(_destClear)`.
  Local unreached instructions: 0x12f895 to 0x12f897, ending at 0x12f898 (exclusive). Count 3.
  Local unreached instructions: 0x12f8cd to 0x12f927, ending at 0x12f928 (exclusive). Count 50.

- `_ttyToggleHwflow`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12e958`.
  Inferred analysis window: 0x12e958 to 0x12ea08 (exclusive), 176 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 1111, body file offset 0x26978, window SHA256 `f31c10c9d347cac14e5fc7ae497b81b8a9252f1add4607e719790f2bf840eac2`.
  Two-byte load, pattern 1: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `movsx` at 0x12e96d.
  Four-byte load, pattern 2: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12e974.
  Four-byte load, pattern 3: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x12e97b.
  Byte load, pattern 4: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `test` at 0x12e980.
  Four-byte store, pattern 5: target `sp -0x14`, raw base `esp`, value `0x0`. Count 1 static access. Sites: `push` at 0x12e989.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x12e98b, `push` at 0x12e9aa.
  Two-byte load, pattern 7: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `and` at 0x12e994.
  Two-byte store, pattern 8: target `unknown`, raw base `ebx`, value `unknown`. Count 1 static access. Sites: `and` at 0x12e994.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `esp`, value `0x1`. Count 1 static access. Sites: `push` at 0x12e9a8.
  Two-byte load, pattern 10: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `or` at 0x12e9b3.
  Two-byte store, pattern 11: target `unknown`, raw base `ebx`, value `unknown`. Count 1 static access. Sites: `or` at 0x12e9b3.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x12e9bc, `push` at 0x12e9c7.
  Four-byte load, pattern 13: target `ret(_sysSerialChanGet) +0x0`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x12e9c5.
  Four-byte store, pattern 14: target `sp -0x14`, raw base `esp`, value `ret(_sysSerialChanGet)`. Count 1 static access. Sites: `push` at 0x12e9d0.
  Four-byte load, pattern 15: target `[ret(_sysSerialChanGet)] +0x4`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x12e9d1.
  Call (`call` at 0x12e98c): _comSetFlowCntl, target 0x12e198. Candidate slots: stack arg0=`unknown`, stack arg1=`0x0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e9ab): _comSetFlowCntl, target 0x12e198. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e9bd): _sysSerialChanGet, target 0x10b7c8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e9c8): _sysSerialChanGet, target 0x10b7c8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12e9d4): (unknown callee), target unknown. Candidate slots: stack arg0=`ret(_sysSerialChanGet)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x12e97e): target 0x12e9db. Taken when `[sp+4] != ?`. Falls through when `[sp+4] == ?`.
  Branch (`je` at 0x12e987): target 0x12e9a8. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`jmp` at 0x12e99d): target 0x12e9bc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jle` at 0x12e9df): target 0x12e968. Taken when `? <= 0x5 (signed, 32-bit)`. Falls through when `? > 0x5 (signed, 32-bit)`.
  Branch (`jl` at 0x12e9e3): target 0x12e9f8. Taken when `l flags condition (operands unknown)`. Falls through when `not (l flags condition (operands unknown))`.
  Jump (`jmp` at 0x12e9e7): target 0x12e9fd. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x12ea06), eax=`unknown`.
  Register fact (`mov` at 0x12e95e): edx=`0xffffffff`.
  Register fact (`xor` at 0x12e963): edi=`0x0`.
  Register fact (`xor` at 0x12e9d9): edx=`0x0`.
  Register fact (`xor` at 0x12e9e5): eax=`0x0`.
  Register fact (`mov` at 0x12e9f8): eax=`0xffffffff`.
  Tool warning: `0x12e9db inc: unsupported effects invalidated`.
  Local unreached instructions: 0x12e99f to 0x12e9a7, ending at 0x12e9a8 (exclusive). Count 9.
  Local unreached instructions: 0x12e9e9 to 0x12e9f7, ending at 0x12e9f8 (exclusive). Count 15.
  Local unreached instructions: 0x12ea07 to 0x12ea07, ending at 0x12ea08 (exclusive). Count 1.
  Successor context at 0x12ea08 stopped decoding early. Bytes 0x12ea0c to 0x12ea28 (exclusive) remain raw and open.

- `_routeProcessELSCCommand`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12c4d4`.
  Inferred analysis window: 0x12c4d4 to 0x12c5c4 (exclusive), 240 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 747, body file offset 0x244f4, window SHA256 `e664a7d7a657500cc5c87d480066134404491e2f398bddc2ff6a5b1081c54623`.
  Four-byte load, pattern 1: target `sp +0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12c4da.
  Four-byte load, pattern 2: target `abs 0x1f4b64`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12c4dd.
  Byte load, pattern 3: target `([0x1f4b64] + [sp+12]) +0x4`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x12c4e2.
  Four-byte load, pattern 4: target `abs 0x1f4b64`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12c4f0.
  Byte load, pattern 5: target `([0x1f4b64] + [sp+12]) +0x4`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x12c4f6, `mov` at 0x12c524.
  Four-byte load, pattern 6: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12c504.
  Four-byte load, pattern 7: target `[sp+8] +0x20`, raw base `ecx`. Count 1 static access. Sites: `mov` at 0x12c507.
  Four-byte store, pattern 8: target `sp -0x14`, raw base `esp`, value `[[sp+8]+32]`. Count 1 static access. Sites: `push` at 0x12c50a.
  Four-byte store, pattern 9: target `sp -0x18`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x12c50b, `push` at 0x12c518.
  Four-byte load, pattern 10: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12c514.
  Four-byte store, pattern 11: target `sp -0x14`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x12c517.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `esp`, value `0x12c4ca`. Count 1 static access. Sites: `push` at 0x12c52c.
  Four-byte load, pattern 13: target `unknown`, raw base `esi`. Count 2 static accesses. Sites: `mov` at 0x12c531, `mov` at 0x12c543.
  Four-byte load, pattern 14: target `unknown`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x12c537.
  Four-byte store, pattern 15: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x12c53a.
  Four-byte store, pattern 16: target `unknown`, raw base `eax`, value `0xb`. Count 1 static access. Sites: `mov` at 0x12c549.
  Call (`call` at 0x12c50c): _elscSendPassword, target 0x1240dc. Candidate slots: stack arg0=`unknown`, stack arg1=`[[sp+8]+32]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12c519): _elscSendCommand, target 0x123f3c. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12c53b): _sprintf, target 0x176c30. Candidate slots: stack arg0=`unknown`, stack arg1=`0x12c4ca`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12c4e8): target 0x12c559. Taken when `(([([0x1f4b64] + [sp+12])+4]:1 & 0xff) & ([([0x1f4b64] + [sp+12])+4]:1 & 0xff)) == 0x0`. Falls through when `(([([0x1f4b64] + [sp+12])+4]:1 & 0xff) & ([([0x1f4b64] + [sp+12])+4]:1 & 0xff)) != 0x0`.
  Branch (`jae` at 0x12c502): target 0x12c524. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Jump (`jmp` at 0x12c521): target 0x12c550. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jge` at 0x12c52a): target 0x12c550. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`jbe` at 0x12c557): target 0x12c4f0. Taken when `? <= 0x1 (unsigned, 32-bit)`. Falls through when `? > 0x1 (unsigned, 32-bit)`.
  At the exit (`ret` at 0x12c562), eax=`unknown`.
  Register fact (`xor` at 0x12c4ea): ebx=`0x0`.
  Register fact (`xor` at 0x12c4ec): esi=`0x0`.
  Tool warning: `0x12c4ff bt: unsupported effects invalidated`.
  Tool warning: `0x12c553 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x12c523 to 0x12c523, ending at 0x12c524 (exclusive). Count 1.
  Local unreached instructions: 0x12c563 to 0x12c5c3, ending at 0x12c5c4 (exclusive). Count 42.

- `_userAcquireRemote`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12eab8`.
  Inferred analysis window: 0x12eab8 to 0x12eb88 (exclusive), 208 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 748, body file offset 0x26ad8, window SHA256 `f29548df72e2140050d13cb75f4ae9744a36c4b08988e750d390a766c1538993`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12eabb.
  Four-byte load, pattern 2: target `abs 0x1e0ec8`, raw base `unknown`. Count 4 static accesses. Sites: `mov` at 0x12eabe, `mov` at 0x12ead3, `mov` at 0x12eaeb.
  Four-byte load, pattern 3: target `[0x1e0ec8] +0x14`, raw base `edx`. Count 3 static accesses. Sites: `mov` at 0x12eac4, `mov` at 0x12eacb, `mov` at 0x12ead9.
  Four-byte store, pattern 4: target `[0x1e0ec8] +0x14`, raw base `edx`, value `unknown`. Count 2 static accesses. Sites: `mov` at 0x12ead0, `mov` at 0x12eade.
  Four-byte store, pattern 5: target `abs 0x1f224c`, raw base `unknown`, value `0x1`. Count 1 static access. Sites: `mov` at 0x12eae1.
  Four-byte load, pattern 6: target `[sp+4] +0x1c`, raw base `ecx`. Count 1 static access. Sites: `mov` at 0x12eaf1.
  Four-byte store, pattern 7: target `[0x1e0ec8] +0x1c`, raw base `edx`, value `[[sp+4]+28]`. Count 1 static access. Sites: `mov` at 0x12eaf4.
  Four-byte load, pattern 8: target `[sp+4] +0x20`, raw base `ecx`. Count 1 static access. Sites: `mov` at 0x12eafd.
  Four-byte store, pattern 9: target `[0x1e0ec8] +0x20`, raw base `edx`, value `[[sp+4]+32]`. Count 1 static access. Sites: `mov` at 0x12eb00.
  Four-byte store, pattern 10: target `sp -0x8`, raw base `esp`, value `[sp+4]+61`. Count 1 static access. Sites: `push` at 0x12eb06.
  Four-byte load, pattern 11: target `abs 0x1e0ec8`, raw base `unknown`. Count 2 static accesses. Sites: `mov` at 0x12eb07, `mov` at 0x12eb12.
  Four-byte store, pattern 12: target `sp -0xc`, raw base `esp`, value `[0x1e0ec8]`. Count 1 static access. Sites: `push` at 0x12eb0c.
  Four-byte store, pattern 13: target `sp -0x8`, raw base `esp`, value `0x12ea78`. Count 1 static access. Sites: `push` at 0x12eb28.
  Call (`call` at 0x12eb0d): _userInitDfltDest, target 0x12fa88. Candidate slots: stack arg0=`[0x1e0ec8]`, stack arg1=`[sp+4]+61`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x12eb2d): _ffscMsg, target 0x12153c. Candidate slots: stack arg0=`0x12ea78`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12eac9): target 0x12eb28. Taken when `(? & 0x2) == 0x0`. Falls through when `(? & 0x2) != 0x0`.
  At the exit (`ret` at 0x12eb1a), eax=`[0x1e0ec8]`.
  At the exit (`ret` at 0x12eb37), eax=`0x0`.
  Register fact (`xor` at 0x12eb32): eax=`0x0`.
  Local unreached instructions: 0x12eb1b to 0x12eb27, ending at 0x12eb28 (exclusive). Count 13.
  Local unreached instructions: 0x12eb38 to 0x12eb87, ending at 0x12eb88 (exclusive). Count 30.
  Predecessor context at 0x12ea08 stopped decoding early. Bytes 0x12ea0c to 0x12eab8 (exclusive) remain raw and open.
