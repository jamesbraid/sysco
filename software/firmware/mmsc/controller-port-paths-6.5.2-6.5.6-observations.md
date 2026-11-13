# MMSC controller and port-path observations

The symbols below label software windows in three distinct shipped carriers.
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

The [full facts](disasm/controller-port-paths-6.5.2-6.5.6-observations-facts.txt) retain every access, instruction and warning.

- `_sysIntInitPIC`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x1082f8`.
  Inferred analysis window: 0x1082f8 to 0x108398 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 64, body file offset 0x318, window SHA256 `ea51ab7515c0f5951240e9fe82a02cec43e3e21419839168146f987b4c306cbf`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `esp`, value `0x11`. Count 2 static accesses. Sites: `push` at 0x1082fb, `push` at 0x108331.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x20`. Count 1 static access. Sites: `push` at 0x1082fd.
  Byte load, pattern 3: target `abs 0x1afe10`, raw base `unknown`. Count 1 static access. Sites: `movsx` at 0x108307.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x10830e, `push` at 0x10834b.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x21`. Count 4 static accesses. Sites: `push` at 0x10830f, `push` at 0x10831b, `push` at 0x108327.
  Four-byte store, pattern 6: target `sp -0x8`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x108319.
  Four-byte store, pattern 7: target `sp -0x8`, raw base `esp`, value `0x1`. Count 2 static accesses. Sites: `push` at 0x108325, `push` at 0x108368.
  Four-byte store, pattern 8: target `sp -0xc`, raw base `esp`, value `0xa0`. Count 1 static access. Sites: `push` at 0x108333.
  Byte load, pattern 9: target `abs 0x1afe10`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x108340.
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

- `_sysIntDisablePIC`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x1083b8`.
  Inferred analysis window: 0x1083b8 to 0x108428 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
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

- `_sysIntEnablePIC`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x108428`.
  Inferred analysis window: 0x108428 to 0x108498 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
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

- `_sysSerialHwInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10b618`.
  Inferred analysis window: 0x10b618 to 0x10b758 (exclusive), 320 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 195, body file offset 0x3638, window SHA256 `0007798bde3c9727fbe22c43c305f2dd0ade57a2b2a1ba46413f9f938bdcc515`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `ebp`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10b621.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `ebp`, value `0x1d4be0`. Count 1 static access. Sites: `mov` at 0x10b632.
  Two-byte store, pattern 3: target `unknown`, raw base `ecx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10b640.
  Four-byte store, pattern 4: target `unknown`, raw base `ecx`, value `0x1c2000`. Count 1 static access. Sites: `mov` at 0x10b647.
  Four-byte load, pattern 5: target `unknown`, raw base `edi`. Count 3 static accesses. Sites: `mov` at 0x10b64f, `mov` at 0x10b669, `mov` at 0x10b6f4.
  Four-byte store, pattern 6: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b653.
  Byte load, pattern 7: target `unknown`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x10b657.
  Byte store, pattern 8: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b65b.
  Two-byte load, pattern 9: target `unknown`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x10b65f.
  Two-byte store, pattern 10: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b664.
  Four-byte store, pattern 11: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b66d.
  Two-byte load, pattern 12: target `unknown`, raw base `edi`. Count 8 static accesses. Sites: `movzx` at 0x10b671, `movzx` at 0x10b67e, `movzx` at 0x10b68d.
  Four-byte load, pattern 13: target `unknown`, raw base `edi`. Count 9 static accesses. Sites: `add` at 0x10b676, `add` at 0x10b685, `add` at 0x10b694.
  Four-byte store, pattern 14: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b67a.
  Four-byte store, pattern 15: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b689.
  Four-byte store, pattern 16: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b698.
  Four-byte store, pattern 17: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6a8.
  Four-byte store, pattern 18: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6b8.
  Four-byte store, pattern 19: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6c8.
  Four-byte store, pattern 20: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6da.
  Two-byte load, pattern 21: target `unknown`, raw base `edi`. Count 1 static access. Sites: `movzx` at 0x10b6de.
  Four-byte store, pattern 22: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6f0.
  Four-byte store, pattern 23: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6f8.
  Four-byte store, pattern 24: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b705.
  Four-byte store, pattern 25: target `unknown`, raw base `ecx`, value `0x1081d0`. Count 1 static access. Sites: `mov` at 0x10b709.
  Four-byte store, pattern 26: target `unknown`, raw base `ecx`, value `0x1081b0`. Count 1 static access. Sites: `mov` at 0x10b711.
  Four-byte load, pattern 27: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10b719.
  Four-byte store, pattern 28: target `sp -0x24`, raw base `esp`, value `[sp-12]`. Count 1 static access. Sites: `push` at 0x10b71c.
  Four-byte store, pattern 29: target `sp -0x10`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b71d.
  Four-byte load, pattern 30: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `add` at 0x10b728.
  Four-byte store, pattern 31: target `sp -0xc`, raw base `ebp`, value `[sp-12]+108`. Count 1 static access. Sites: `add` at 0x10b728.
  Four-byte load, pattern 32: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `inc` at 0x10b732.
  Four-byte store, pattern 33: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `inc` at 0x10b732.
  Four-byte load, pattern 34: target `sp -0x10`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10b735.
  Four-byte load, pattern 35: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x10b738.
  Call (`call` at 0x10b720): _ns16550DevInit, target 0x175394. Candidate slots: stack arg0=`[sp-12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x10b73c): target 0x10b640. Taken when `[sp-8] <= 0x5 (signed, 32-bit)`. Falls through when `[sp-8] > 0x5 (signed, 32-bit)`.
  At the exit (`ret` at 0x10b74b), eax=`ret(_ns16550DevInit)`.
  Register fact (`mov` at 0x10b628): ecx=`0x1d4be0`.
  Register fact (`mov` at 0x10b62d): edi=`0x1ac6dc`.
  Register fact (`xor` at 0x10b639): ebx=`0x0`.
  Register fact (`xor` at 0x10b63b): esi=`0x0`.
  Tool warning: `0x10b732 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x10b74c to 0x10b757, ending at 0x10b758 (exclusive). Count 12.
  Encoding (`mov` at 0x10b640): bytes `66c744315c0000`, raw prefix `0x66`.
  Encoding (`mov` at 0x10b65f): bytes `668b441f08`, raw prefix `0x66`.
  Encoding (`mov` at 0x10b664): bytes `668944315e`, raw prefix `0x66`.

- `_sysSerialHwInit2`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10b758`.
  Inferred analysis window: 0x10b758 to 0x10b7c8 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 198, body file offset 0x3778, window SHA256 `cafa3304bde7bcda9a8b370432c4fad40b772901d8fe9c01b80e0617bb1500d8`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `ebp`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10b761.
  Two-byte load, pattern 2: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `cmp` at 0x10b774.
  Four-byte store, pattern 3: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x10b77a.
  Four-byte store, pattern 4: target `sp -0x1c`, raw base `esp`, value `0x1760b4`. Count 1 static access. Sites: `push` at 0x10b77b.
  Two-byte load, pattern 5: target `unknown`, raw base `ebx`. Count 1 static access. Sites: `movzx` at 0x10b780.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x10b786.
  Two-byte load, pattern 7: target `unknown`, raw base `esi`. Count 1 static access. Sites: `movzx` at 0x10b78f.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `unknown`. Count 1 static access. Sites: `push` at 0x10b796.
  Four-byte load, pattern 9: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `inc` at 0x10b7a8.
  Four-byte store, pattern 10: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `inc` at 0x10b7a8.
  Four-byte load, pattern 11: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x10b7ab.
  Call (`call` at 0x10b787): _intConnect, target 0x176498. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1760b4`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x10b797): _sysIntEnablePIC, target 0x108428. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x10b778): target 0x10b79f. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`jle` at 0x10b7af): target 0x10b774. Taken when `[sp-8] <= 0x5 (signed, 32-bit)`. Falls through when `[sp-8] > 0x5 (signed, 32-bit)`.
  At the exit (`ret` at 0x10b7ba), eax=`unknown`.
  Register fact (`mov` at 0x10b768): ebx=`0x1ac6dc`.
  Register fact (`mov` at 0x10b76d): edi=`0x1d4be0`.
  Register fact (`xor` at 0x10b772): esi=`0x0`.
  Tool warning: `0x10b7a8 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x10b7bb to 0x10b7c7, ending at 0x10b7c8 (exclusive). Count 13.
  Encoding (`cmp` at 0x10b774): bytes `66833b00`, raw prefix `0x66`.

- `_sysSerialHwInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x10b618`.
  Inferred analysis window: 0x10b618 to 0x10b758 (exclusive), 320 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 195, body file offset 0x3638, window SHA256 `dd1bf05fc29f040d55c1c101adf8e8ae16340d0f6dbf7ffc9cd4509e39cd7332`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `ebp`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10b621.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `ebp`, value `0x1d89f0`. Count 1 static access. Sites: `mov` at 0x10b632.
  Two-byte store, pattern 3: target `unknown`, raw base `ecx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x10b640.
  Four-byte store, pattern 4: target `unknown`, raw base `ecx`, value `0x1c2000`. Count 1 static access. Sites: `mov` at 0x10b647.
  Four-byte load, pattern 5: target `unknown`, raw base `edi`. Count 3 static accesses. Sites: `mov` at 0x10b64f, `mov` at 0x10b669, `mov` at 0x10b6f4.
  Four-byte store, pattern 6: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b653.
  Byte load, pattern 7: target `unknown`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x10b657.
  Byte store, pattern 8: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b65b.
  Two-byte load, pattern 9: target `unknown`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x10b65f.
  Two-byte store, pattern 10: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b664.
  Four-byte store, pattern 11: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b66d.
  Two-byte load, pattern 12: target `unknown`, raw base `edi`. Count 8 static accesses. Sites: `movzx` at 0x10b671, `movzx` at 0x10b67e, `movzx` at 0x10b68d.
  Four-byte load, pattern 13: target `unknown`, raw base `edi`. Count 9 static accesses. Sites: `add` at 0x10b676, `add` at 0x10b685, `add` at 0x10b694.
  Four-byte store, pattern 14: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b67a.
  Four-byte store, pattern 15: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b689.
  Four-byte store, pattern 16: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b698.
  Four-byte store, pattern 17: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6a8.
  Four-byte store, pattern 18: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6b8.
  Four-byte store, pattern 19: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6c8.
  Four-byte store, pattern 20: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6da.
  Two-byte load, pattern 21: target `unknown`, raw base `edi`. Count 1 static access. Sites: `movzx` at 0x10b6de.
  Four-byte store, pattern 22: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6f0.
  Four-byte store, pattern 23: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b6f8.
  Four-byte store, pattern 24: target `unknown`, raw base `ecx`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b705.
  Four-byte store, pattern 25: target `unknown`, raw base `ecx`, value `0x1081d0`. Count 1 static access. Sites: `mov` at 0x10b709.
  Four-byte store, pattern 26: target `unknown`, raw base `ecx`, value `0x1081b0`. Count 1 static access. Sites: `mov` at 0x10b711.
  Four-byte load, pattern 27: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10b719.
  Four-byte store, pattern 28: target `sp -0x24`, raw base `esp`, value `[sp-12]`. Count 1 static access. Sites: `push` at 0x10b71c.
  Four-byte store, pattern 29: target `sp -0x10`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `mov` at 0x10b71d.
  Four-byte load, pattern 30: target `sp -0xc`, raw base `ebp`. Count 1 static access. Sites: `add` at 0x10b728.
  Four-byte store, pattern 31: target `sp -0xc`, raw base `ebp`, value `[sp-12]+108`. Count 1 static access. Sites: `add` at 0x10b728.
  Four-byte load, pattern 32: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `inc` at 0x10b732.
  Four-byte store, pattern 33: target `sp -0x8`, raw base `ebp`, value `unknown`. Count 1 static access. Sites: `inc` at 0x10b732.
  Four-byte load, pattern 34: target `sp -0x10`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x10b735.
  Four-byte load, pattern 35: target `sp -0x8`, raw base `ebp`. Count 1 static access. Sites: `cmp` at 0x10b738.
  Call (`call` at 0x10b720): _ns16550DevInit, target 0x1790ec. Candidate slots: stack arg0=`[sp-12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x10b73c): target 0x10b640. Taken when `[sp-8] <= 0x5 (signed, 32-bit)`. Falls through when `[sp-8] > 0x5 (signed, 32-bit)`.
  At the exit (`ret` at 0x10b74b), eax=`ret(_ns16550DevInit)`.
  Register fact (`mov` at 0x10b628): ecx=`0x1d89f0`.
  Register fact (`mov` at 0x10b62d): edi=`0x1b0434`.
  Register fact (`xor` at 0x10b639): ebx=`0x0`.
  Register fact (`xor` at 0x10b63b): esi=`0x0`.
  Tool warning: `0x10b732 inc: unsupported effects invalidated`.
  Local unreached instructions: 0x10b74c to 0x10b757, ending at 0x10b758 (exclusive). Count 12.
  Encoding (`mov` at 0x10b640): bytes `66c744315c0000`, raw prefix `0x66`.
  Encoding (`mov` at 0x10b65f): bytes `668b441f08`, raw prefix `0x66`.
  Encoding (`mov` at 0x10b664): bytes `668944315e`, raw prefix `0x66`.

- `_sysIntInitPIC`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1082f8`.
  Inferred analysis window: 0x1082f8 to 0x108398 (exclusive), 160 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 64, body file offset 0x318, window SHA256 `2c27b91a9b404179d65855080621ef8eb00bbcb7da8f0f44dc10c0d3fb354e7f`.
  Four-byte store, pattern 1: target `sp -0x8`, raw base `esp`, value `0x11`. Count 2 static accesses. Sites: `push` at 0x1082fb, `push` at 0x108331.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `esp`, value `0x20`. Count 1 static access. Sites: `push` at 0x1082fd.
  Byte load, pattern 3: target `abs 0x1afe00`, raw base `unknown`. Count 1 static access. Sites: `movsx` at 0x108307.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x10830e, `push` at 0x10834b.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `0x21`. Count 4 static accesses. Sites: `push` at 0x10830f, `push` at 0x10831b, `push` at 0x108327.
  Four-byte store, pattern 6: target `sp -0x8`, raw base `esp`, value `0x4`. Count 1 static access. Sites: `push` at 0x108319.
  Four-byte store, pattern 7: target `sp -0x8`, raw base `esp`, value `0x1`. Count 2 static accesses. Sites: `push` at 0x108325, `push` at 0x108368.
  Four-byte store, pattern 8: target `sp -0xc`, raw base `esp`, value `0xa0`. Count 1 static access. Sites: `push` at 0x108333.
  Byte load, pattern 9: target `abs 0x1afe00`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x108340.
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

- `_sysIntDisablePIC`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1083b8`.
  Inferred analysis window: 0x1083b8 to 0x108428 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
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

- `_sysIntEnablePIC`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x108428`.
  Inferred analysis window: 0x108428 to 0x108498 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
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

- `_csSysInEcho`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x11f968`.
  Inferred analysis window: 0x11f968 to 0x11f9e8 (exclusive), 128 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 856, body file offset 0x17988, window SHA256 `68667716c59652639c9991524e74b5c5dae2c2b6971418b8f7d5a9bf8197f82c`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11f96b.
  Four-byte load, pattern 2: target `[sp+4] +0x78`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x11f96e.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `esp`, value `[[sp+4]+120]`. Count 1 static access. Sites: `push` at 0x11f971.
  Four-byte load, pattern 4: target `[sp+4] +0x68`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x11f972, `mov` at 0x11f979.
  Four-byte load, pattern 5: target `[[sp+4]+104] +0x10`, raw base `eax`. Count 1 static access. Sites: `mov` at 0x11f975.
  Four-byte store, pattern 6: target `sp -0xc`, raw base `esp`, value `[[[sp+4]+104]+16]`. Count 1 static access. Sites: `push` at 0x11f978.
  Four-byte store, pattern 7: target `sp -0x10`, raw base `esp`, value `[[sp+4]+104]`. Count 1 static access. Sites: `push` at 0x11f97c.
  Four-byte load, pattern 8: target `[sp+4] +0x74`, raw base `edx`. Count 1 static access. Sites: `mov` at 0x11f97d.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `esp`, value `[[sp+4]+116]`. Count 1 static access. Sites: `push` at 0x11f980.
  Call (`call` at 0x11f981): _bufAppendAddMsgs, target 0x10f118. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`[[sp+4]+104]`, stack arg2=`[[[sp+4]+104]+16]`, stack arg3=`[[sp+4]+120]`, stack arg4=`ebp`, stack arg5=`unknown`.
  At the exit (`ret` at 0x11f989), eax=`ret(_bufAppendAddMsgs)`.
  Local unreached instructions: 0x11f98a to 0x11f9e7, ending at 0x11f9e8 (exclusive). Count 49.

- `_csSysInToUserOut`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x120088`.
  Inferred analysis window: 0x120088 to 0x1200f8 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 859, body file offset 0x180a8, window SHA256 `26f6e588b4f9f433d2d35fc70a0c7634e620b6cf4f275434e7572a4fe913aa44`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x12008d.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x120090.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x120093.
  Four-byte load, pattern 4: target `[sp+4] +0x68`, raw base `ebx`. Count 4 static accesses. Sites: `mov` at 0x120094, `mov` at 0x1200b6, `mov` at 0x1200d8.
  Four-byte load, pattern 5: target `[[sp+4]+104] +0x4`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x120097, `mov` at 0x1200db.
  Four-byte store, pattern 6: target `sp -0x14`, raw base `esp`, value `[[[sp+4]+104]+4]`. Count 1 static access. Sites: `push` at 0x12009a.
  Four-byte load, pattern 7: target `abs 0x1e1154`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x12009b.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `[0x1e1154]`. Count 1 static access. Sites: `push` at 0x1200a0.
  Four-byte load, pattern 9: target `[sp+4] +0x18`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x1200a9.
  Four-byte load, pattern 10: target `[sp+4] +0x78`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x1200b1.
  Four-byte store, pattern 11: target `sp -0x10`, raw base `esp`, value `[[sp+4]+120]`. Count 1 static access. Sites: `push` at 0x1200b4.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x1200b5.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `esp`, value `[[sp+4]+104]`. Count 1 static access. Sites: `push` at 0x1200b9.
  Four-byte load, pattern 14: target `[sp+4] +0x74`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x1200ba.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `esp`, value `[[sp+4]+116]`. Count 1 static access. Sites: `push` at 0x1200bd.
  Four-byte store, pattern 16: target `sp -0x10`, raw base `esp`, value `([[[sp+4]+104]+4] + [sp+8])`. Count 1 static access. Sites: `push` at 0x1200e0.
  Four-byte store, pattern 17: target `sp -0x14`, raw base `esp`, value `[[sp+4]+104]`. Count 1 static access. Sites: `push` at 0x1200e4.
  Call (`call` at 0x1200a1): _logWrite, target 0x127974. Candidate slots: stack arg0=`[0x1e1154]`, stack arg1=`[[[sp+4]+104]+4]`, stack arg2=`[sp+8]`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`ebp`.
  Call (`call` at 0x1200be): _bufAppendAddMsgs, target 0x10f118. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`[[sp+4]+104]`, stack arg2=`[sp+8]`, stack arg3=`[[sp+4]+120]`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1200e5): _bufLeftAlign, target 0x10fb08. Candidate slots: stack arg0=`[[sp+4]+104]`, stack arg1=`([[[sp+4]+104]+4] + [sp+8])`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x1200af): target 0x1200d8. Taken when `[[sp+4]+24] == 0x9`. Falls through when `[[sp+4]+24] != 0x9`.
  Branch (`je` at 0x1200c5): target 0x1200ea. Taken when `(ret(_bufAppendAddMsgs) & ret(_bufAppendAddMsgs)) == 0x0`. Falls through when `(ret(_bufAppendAddMsgs) & ret(_bufAppendAddMsgs)) != 0x0`.
  Jump (`jmp` at 0x1200cc): target 0x1200ec. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x1200f4), eax=`unknown`.
  Register fact (`mov` at 0x1200c7): eax=`0xffffffff`.
  Register fact (`xor` at 0x1200ea): eax=`0x0`.
  Local unreached instructions: 0x1200ce to 0x1200d7, ending at 0x1200d8 (exclusive). Count 10.
  Local unreached instructions: 0x1200f5 to 0x1200f7, ending at 0x1200f8 (exclusive). Count 3.

- `_csSysInToUserOut`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x11dc20`.
  Inferred analysis window: 0x11dc20 to 0x11dc90 (exclusive), 112 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 826, body file offset 0x15c40, window SHA256 `34f94424bddcc22d0d84a88b139dd760306aca148a12b9d1b0f2e24367f9ced1`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11dc25.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11dc28.
  Four-byte store, pattern 3: target `sp -0x10`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x11dc2b.
  Four-byte load, pattern 4: target `[sp+4] +0x68`, raw base `ebx`. Count 4 static accesses. Sites: `mov` at 0x11dc2c, `mov` at 0x11dc4e, `mov` at 0x11dc70.
  Four-byte load, pattern 5: target `[[sp+4]+104] +0x4`, raw base `eax`. Count 2 static accesses. Sites: `mov` at 0x11dc2f, `mov` at 0x11dc73.
  Four-byte store, pattern 6: target `sp -0x14`, raw base `esp`, value `[[[sp+4]+104]+4]`. Count 1 static access. Sites: `push` at 0x11dc32.
  Four-byte load, pattern 7: target `abs 0x1dd2f4`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x11dc33.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `esp`, value `[0x1dd2f4]`. Count 1 static access. Sites: `push` at 0x11dc38.
  Four-byte load, pattern 9: target `[sp+4] +0x18`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11dc41.
  Four-byte load, pattern 10: target `[sp+4] +0x78`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11dc49.
  Four-byte store, pattern 11: target `sp -0x10`, raw base `esp`, value `[[sp+4]+120]`. Count 1 static access. Sites: `push` at 0x11dc4c.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `esp`, value `[sp+8]`. Count 1 static access. Sites: `push` at 0x11dc4d.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `esp`, value `[[sp+4]+104]`. Count 1 static access. Sites: `push` at 0x11dc51.
  Four-byte load, pattern 14: target `[sp+4] +0x74`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x11dc52.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `esp`, value `[[sp+4]+116]`. Count 1 static access. Sites: `push` at 0x11dc55.
  Four-byte store, pattern 16: target `sp -0x10`, raw base `esp`, value `([[[sp+4]+104]+4] + [sp+8])`. Count 1 static access. Sites: `push` at 0x11dc78.
  Four-byte store, pattern 17: target `sp -0x14`, raw base `esp`, value `[[sp+4]+104]`. Count 1 static access. Sites: `push` at 0x11dc7c.
  Call (`call` at 0x11dc39): _logWrite, target 0x1250bc. Candidate slots: stack arg0=`[0x1dd2f4]`, stack arg1=`[[[sp+4]+104]+4]`, stack arg2=`[sp+8]`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`ebp`.
  Call (`call` at 0x11dc56): _bufAppendAddMsgs, target 0x10ef28. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`[[sp+4]+104]`, stack arg2=`[sp+8]`, stack arg3=`[[sp+4]+120]`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11dc7d): _bufLeftAlign, target 0x10f908. Candidate slots: stack arg0=`[[sp+4]+104]`, stack arg1=`([[[sp+4]+104]+4] + [sp+8])`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x11dc47): target 0x11dc70. Taken when `[[sp+4]+24] == 0x9`. Falls through when `[[sp+4]+24] != 0x9`.
  Branch (`je` at 0x11dc5d): target 0x11dc82. Taken when `(ret(_bufAppendAddMsgs) & ret(_bufAppendAddMsgs)) == 0x0`. Falls through when `(ret(_bufAppendAddMsgs) & ret(_bufAppendAddMsgs)) != 0x0`.
  Jump (`jmp` at 0x11dc64): target 0x11dc84. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`ret` at 0x11dc8c), eax=`unknown`.
  Register fact (`mov` at 0x11dc5f): eax=`0xffffffff`.
  Register fact (`xor` at 0x11dc82): eax=`0x0`.
  Local unreached instructions: 0x11dc66 to 0x11dc6f, ending at 0x11dc70 (exclusive). Count 10.
  Local unreached instructions: 0x11dc8d to 0x11dc8f, ending at 0x11dc90 (exclusive). Count 3.

- `_crRouterIn`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x11e4b8`.
  Inferred analysis window: 0x11e4b8 to 0x11e5c8 (exclusive), 272 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 831, body file offset 0x164d8, window SHA256 `5f21d9b981e0b33ef51ecb6d35103fa442de781753347551786f2b3f96af8f3e`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11e4c4.
  Four-byte load, pattern 2: target `abs 0x1e0670`, raw base `unknown`. Count 1 static access. Sites: `mov` at 0x11e4c7.
  Four-byte store, pattern 3: target `sp -0x108`, raw base `esp`, value `[0x1e0670]`. Count 1 static access. Sites: `push` at 0x11e4cc.
  Four-byte store, pattern 4: target `sp -0x10c`, raw base `esp`, value `0xf0`. Count 1 static access. Sites: `push` at 0x11e4cd.
  Four-byte store, pattern 5: target `sp -0x110`, raw base `esp`, value `sp-248`. Count 1 static access. Sites: `push` at 0x11e4d8.
  Four-byte load, pattern 6: target `[sp+4] +0x5c`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x11e4d9.
  Four-byte store, pattern 7: target `sp -0x114`, raw base `esp`, value `[[sp+4]+92]`. Count 1 static access. Sites: `push` at 0x11e4dc.
  Four-byte store, pattern 8: target `sp -0x108`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x11e4eb.
  Four-byte store, pattern 9: target `sp -0x10c`, raw base `esp`, value `0x11e458`. Count 1 static access. Sites: `push` at 0x11e4ec.
  Four-byte store, pattern 10: target `sp -0x108`, raw base `esp`, value `0xd`. Count 1 static access. Sites: `push` at 0x11e518.
  Four-byte store, pattern 11: target `sp -0x10c`, raw base `esp`, value `sp-248`. Count 1 static access. Sites: `push` at 0x11e51a.
  Byte store, pattern 12: target `ret(_strrchr) +0x0`, raw base `eax`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11e527.
  Byte store, pattern 13: target `(ret(_timeoutRead) + sp-4) -0xf4`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x11e52a.
  Byte load, pattern 14: target `abs 0x1e0bf0`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x11e532.
  Four-byte store, pattern 15: target `sp -0x108`, raw base `esp`, value `sp-248`. Count 2 static accesses. Sites: `push` at 0x11e53b, `push` at 0x11e54a.
  Four-byte store, pattern 16: target `sp -0x10c`, raw base `esp`, value `[sp+4]`. Count 2 static accesses. Sites: `push` at 0x11e53c, `push` at 0x11e54b.
  Four-byte store, pattern 17: target `sp -0x110`, raw base `esp`, value `0x11e481`. Count 1 static access. Sites: `push` at 0x11e53d.
  Call (`call` at 0x11e4dd): _timeoutRead, target 0x12dc68. Candidate slots: stack arg0=`[[sp+4]+92]`, stack arg1=`sp-248`, stack arg2=`0xf0`, stack arg3=`[0x1e0670]`, stack arg4=`ebx`, stack arg5=`esi`.
  Call (`call` at 0x11e4f1): _ffscMsgS, target 0x12167c. Candidate slots: stack arg0=`0x11e458`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11e51b): _strrchr, target 0x184458. Candidate slots: stack arg0=`sp-248`, stack arg1=`0xd`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11e542): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x11e481`, stack arg1=`[sp+4]`, stack arg2=`sp-248`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11e54c): _crProcessCmd, target 0x11e5c8. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`sp-248`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x11e4e9): target 0x11e508. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Jump (`jmp` at 0x11e4fb): target 0x11e551. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x11e50a): target 0x11e518. Taken when `(ret(_timeoutRead) & ret(_timeoutRead)) != 0x0`. Falls through when `(ret(_timeoutRead) & ret(_timeoutRead)) == 0x0`.
  Jump (`jmp` at 0x11e511): target 0x11e551. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x11e525): target 0x11e52a. Taken when `(ret(_strrchr) & ret(_strrchr)) == 0x0`. Falls through when `(ret(_strrchr) & ret(_strrchr)) != 0x0`.
  Branch (`je` at 0x11e539): target 0x11e54a. Taken when `(([0x1e0bf0]:1 & 0xff) & 0x4) == 0x0`. Falls through when `(([0x1e0bf0]:1 & 0xff) & 0x4) != 0x0`.
  At the exit (`ret` at 0x11e55d), eax=`unknown`.
  Register fact (`mov` at 0x11e4f6): eax=`0xffffffff`.
  Register fact (`mov` at 0x11e50c): eax=`0xffffffff`.
  Local unreached instructions: 0x11e4fd to 0x11e507, ending at 0x11e508 (exclusive). Count 11.
  Local unreached instructions: 0x11e513 to 0x11e517, ending at 0x11e518 (exclusive). Count 5.
  Local unreached instructions: 0x11e55e to 0x11e5c7, ending at 0x11e5c8 (exclusive). Count 56.

- `_cuUserInELSC`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x120938`.
  Inferred analysis window: 0x120938 to 0x120a78 (exclusive), 320 bytes. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 867, body file offset 0x18958, window SHA256 `ca538a90cb8e40959e61f81ae51661d5537b1c0742e1cad8f259733fe956977f`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x120944.
  Four-byte load, pattern 2: target `[sp+4] +0x70`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x120947.
  Four-byte load, pattern 3: target `[sp+4] +0x24`, raw base `edi`. Count 2 static accesses. Sites: `mov` at 0x12094a, `mov` at 0x120952.
  Four-byte store, pattern 4: target `sp -0x1744`, raw base `esp`, value `[[sp+4]+36]`. Count 1 static access. Sites: `push` at 0x120955.
  Four-byte store, pattern 5: target `sp -0x1748`, raw base `esp`, value `[sp+4]`. Count 2 static accesses. Sites: `push` at 0x120956, `push` at 0x120a05.
  Four-byte store, pattern 6: target `sp -0x174c`, raw base `esp`, value `0x1208d8`. Count 1 static access. Sites: `push` at 0x120957.
  Four-byte load, pattern 7: target `[[sp+4]+112] +0x4`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x120978, `mov` at 0x12099e.
  Four-byte load, pattern 8: target `[[sp+4]+112] +0x10`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x12097b, `mov` at 0x1209a1.
  Byte store, pattern 9: target `([[[sp+4]+112]+16] + [[[sp+4]+112]+4]) +0x0`, raw base `eax`, value `0x0`. Count 1 static access. Sites: `mov` at 0x12097e.
  Byte load, pattern 10: target `[sp+4] +0x90`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x120982.
  Four-byte store, pattern 11: target `sp -0x1744`, raw base `esp`, value `unknown`. Count 2 static accesses. Sites: `push` at 0x12098b, `push` at 0x1209b4.
  Four-byte load, pattern 12: target `[[sp+4]+112] +0x4`, raw base `ebx`. Count 3 static accesses. Sites: `mov` at 0x12098c, `mov` at 0x1209a7, `mov` at 0x1209b5.
  Four-byte store, pattern 13: target `sp -0x1748`, raw base `esp`, value `[[[sp+4]+112]+4]`. Count 2 static accesses. Sites: `push` at 0x12098f, `push` at 0x1209b8.
  Four-byte load, pattern 14: target `[sp+4] +0x6c`, raw base `edi`. Count 1 static access. Sites: `mov` at 0x1209b9.
  Four-byte store, pattern 15: target `sp -0x174c`, raw base `esp`, value `[[sp+4]+108]`. Count 1 static access. Sites: `push` at 0x1209bc.
  Byte load, pattern 16: target `unknown`, raw base `esi`. Count 1 static access. Sites: `mov` at 0x1209c5.
  Four-byte store, pattern 17: target `sp -0x1744`, raw base `esp`, value `[[sp+4]+112]`. Count 1 static access. Sites: `push` at 0x1209cb.
  Byte load, pattern 18: target `abs 0x1e0bf0`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x1209d8.
  Four-byte store, pattern 19: target `sp -0x1744`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x1209e1.
  Four-byte store, pattern 20: target `sp -0x1748`, raw base `esp`, value `0x120905`. Count 1 static access. Sites: `push` at 0x1209e2.
  Four-byte store, pattern 21: target `sp -0x1744`, raw base `esp`, value `sp-5940`. Count 2 static accesses. Sites: `push` at 0x1209f5, `push` at 0x120a04.
  Four-byte store, pattern 22: target `sp -0x1748`, raw base `esp`, value `0x12092c`. Count 1 static access. Sites: `push` at 0x1209f6.
  Four-byte store, pattern 23: target `sp -0x174c`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x1209fb.
  Call (`call` at 0x12095c): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x1208d8`, stack arg1=`[sp+4]`, stack arg2=`[[sp+4]+36]`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`edi`.
  Call (`call` at 0x120990): _strchr, target 0x184070. Candidate slots: stack arg0=`[[[sp+4]+112]+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1209bd): _bufWrite, target 0x10fe28. Candidate slots: stack arg0=`[[sp+4]+108]`, stack arg1=`[[[sp+4]+112]+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1209cc): _bufClear, target 0x10f6a8. Candidate slots: stack arg0=`[[sp+4]+112]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1209e7): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x120905`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1209fc): _cmSendFFSCCmd, target 0x11c738. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0x12092c`, stack arg2=`sp-5940`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x120a06): _cmPrintResponse, target 0x11c5c8. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`sp-5940`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x120950): target 0x120978. Taken when `[[sp+4]+36] == 0x1`. Falls through when `[[sp+4]+36] != 0x1`.
  Jump (`jmp` at 0x120966): target 0x120a0d. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`jne` at 0x12099c): target 0x1209a7. Taken when `(ret(_strchr) & ret(_strchr)) != 0x0`. Falls through when `(ret(_strchr) & ret(_strchr)) == 0x0`.
  Branch (`jle` at 0x1209b2): target 0x1209c5. Taken when `le flags condition (operands unknown)`. Falls through when `not (le flags condition (operands unknown))`.
  Branch (`jne` at 0x1209c9): target 0x1209d8. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`jmp` at 0x1209d1): target 0x120a0b. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`je` at 0x1209df): target 0x1209ef. Taken when `(([0x1e0bf0]:1 & 0xff) & 0x4) == 0x0`. Falls through when `(([0x1e0bf0]:1 & 0xff) & 0x4) != 0x0`.
  At the exit (`ret` at 0x120a19), eax=`unknown`.
  Register fact (`mov` at 0x120961): eax=`0xffffffff`.
  Register fact (`xor` at 0x120a0b): eax=`0x0`.
  Local unreached instructions: 0x12096b to 0x120977, ending at 0x120978 (exclusive). Count 13.
  Local unreached instructions: 0x1209d3 to 0x1209d7, ending at 0x1209d8 (exclusive). Count 5.
  Local unreached instructions: 0x120a1a to 0x120a77, ending at 0x120a78 (exclusive). Count 35.

- `_clLeaveELSCMode`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1199f8`.
  Inferred analysis window: 0x1199f8 to 0x119af8 (exclusive), 256 bytes. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 769, body file offset 0x11a18, window SHA256 `92c643ce97655b6abe4bd08d2aa5df4b5cf89a448fef86be09b4a1583edd0d59`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x119a02.
  Four-byte load, pattern 2: target `[sp+4] +0x68`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x119a05, `mov` at 0x119a11.
  Four-byte store, pattern 3: target `sp -0x704`, raw base `esp`, value `[[sp+4]+104]`. Count 2 static accesses. Sites: `push` at 0x119a08, `push` at 0x119a14.
  Four-byte load, pattern 4: target `[sp+4] +0x6c`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x119a1d.
  Four-byte store, pattern 5: target `sp -0x704`, raw base `esp`, value `[[sp+4]+108]`. Count 1 static access. Sites: `push` at 0x119a20.
  Four-byte load, pattern 6: target `[sp+4] +0x70`, raw base `ebx`. Count 1 static access. Sites: `mov` at 0x119a29.
  Four-byte store, pattern 7: target `sp -0x704`, raw base `esp`, value `[[sp+4]+112]`. Count 1 static access. Sites: `push` at 0x119a2c.
  Four-byte load, pattern 8: target `[sp+4] +0x74`, raw base `ebx`. Count 2 static accesses. Sites: `mov` at 0x119a35, `mov` at 0x119a41.
  Four-byte store, pattern 9: target `sp -0x704`, raw base `esp`, value `[[sp+4]+116]`. Count 2 static accesses. Sites: `push` at 0x119a38, `push` at 0x119a44.
  Four-byte store, pattern 10: target `[sp+4] +0x18`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x119a4d.
  Four-byte store, pattern 11: target `[sp+4] +0x1c`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x119a54.
  Four-byte store, pattern 12: target `[sp+4] +0x20`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x119a5b.
  Four-byte store, pattern 13: target `[sp+4] +0x24`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x119a62.
  Four-byte store, pattern 14: target `[sp+4] +0x28`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x119a69.
  Four-byte load, pattern 15: target `[sp+4] +0x7c`, raw base `ebx`. Count 3 static accesses. Sites: `mov` at 0x119a70, `mov` at 0x119a77, `mov` at 0x119a81.
  Four-byte store, pattern 16: target `[[sp+4]+124] +0x4`, raw base `eax`, value `0x1`. Count 1 static access. Sites: `mov` at 0x119a7a.
  Four-byte store, pattern 17: target `sp -0x704`, raw base `esp`, value `[[sp+4]+124]`. Count 1 static access. Sites: `push` at 0x119a84.
  Four-byte store, pattern 18: target `abs 0x1ee3f4`, raw base `unknown`, value `0x1`. Count 1 static access. Sites: `mov` at 0x119a8d.
  Four-byte store, pattern 19: target `sp -0x704`, raw base `esp`, value `sp-1788`. Count 1 static access. Sites: `push` at 0x119a9d.
  Four-byte store, pattern 20: target `sp -0x708`, raw base `esp`, value `0x1199c6`. Count 1 static access. Sites: `push` at 0x119a9e.
  Four-byte store, pattern 21: target `sp -0x70c`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x119aa3.
  Four-byte store, pattern 22: target `[sp+4] +0x7c`, raw base `ebx`, value `0x0`. Count 1 static access. Sites: `mov` at 0x119aac.
  Byte load, pattern 23: target `abs 0x1dcdc8`, raw base `unknown`. Count 1 static access. Sites: `test` at 0x119ab3.
  Four-byte store, pattern 24: target `sp -0x704`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x119abc.
  Four-byte store, pattern 25: target `sp -0x708`, raw base `esp`, value `0x1199ce`. Count 1 static access. Sites: `push` at 0x119abd.
  Call (`call` at 0x119a09): _bufHold, target 0x10f748. Candidate slots: stack arg0=`[[sp+4]+104]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119a15): _bufDetachFD, target 0x10f4f8. Candidate slots: stack arg0=`[[sp+4]+104]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119a21): _bufDetachFD, target 0x10f4f8. Candidate slots: stack arg0=`[[sp+4]+108]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119a2d): _bufDetachFD, target 0x10f4f8. Candidate slots: stack arg0=`[[sp+4]+112]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119a39): _bufDetachFD, target 0x10f4f8. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119a45): _bufDetachLog, target 0x10f568. Candidate slots: stack arg0=`[[sp+4]+116]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119a85): _elscSetup, target 0x121a90. Candidate slots: stack arg0=`[[sp+4]+124]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119aa4): _cmSendFFSCCmd, target 0x11a6f0. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0x1199c6`, stack arg2=`sp-1788`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x119ac2): _ffscMsg, target 0x11f0d0. Candidate slots: stack arg0=`0x1199ce`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x119a75): target 0x119ab3. Taken when `([[sp+4]+124] & [[sp+4]+124]) == 0x0`. Falls through when `([[sp+4]+124] & [[sp+4]+124]) != 0x0`.
  Branch (`je` at 0x119aba): target 0x119ac7. Taken when `(([0x1dcdc8]:1 & 0xff) & 0x4) == 0x0`. Falls through when `(([0x1dcdc8]:1 & 0xff) & 0x4) != 0x0`.
  At the exit (`ret` at 0x119ad2), eax=`0x0`.
  Register fact (`xor` at 0x119ac7): eax=`0x0`.
  Local unreached instructions: 0x119ad3 to 0x119af7, ending at 0x119af8 (exclusive). Count 17.
