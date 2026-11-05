# ColdFire L1 reached instruction observations

Each entry applies to its own shipped carrier, package and extracted payload.
Window bounds, additional function extent, function uniqueness and runtime mapping remain unknown or inferred.
Path and reachability statements refer to the current local CFG with ordinary call fallthrough.
Unknown and indirect callees remain opaque. Arity and custom ABI remain unknown.
Modeled absence of a reached exit does not establish actual runtime feasibility.
Only reached accesses, calls and branches are summarized. Exact unreached runs are stated separately.
All widths are in bytes. A raw base register does not supply an unknown effective address.
Each memory summary covers one selected encoded pattern, identified by its cited instructions. Static groups do not establish equal runtime pointees, dynamic counts or one unconditional sequence.
Stack summaries include incoming loads and the final three windows' local copies and reloads.
Outgoing stack writes are retained in the full facts.
Calls list the modeled candidate slots, whose unknown values do not erase known writes in the full facts.
Known expressions and stored literals remain as printed. Unknown values remain unknown.

The [full facts](disasm/coldfire-variants/reached-window-observations-facts.txt) retain the complete windows and tool warnings.

- `0xffe8e0e0`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#2`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8e0e0 to 0xffe8e31a (exclusive), 570 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`.
  Read window: SHA256 `9667b10adda8ba7bbee5066a5c4eb1f4376d904aa5a09a5278db5348a1d523db`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8e0ec.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e0f0.
  Four-byte load: target `abs 0x30001188`. Selected encoded pattern: 1 static access, including `tst.l` at 0xffe8e126.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e192.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8e1c4, `tst.l` at 0xffe8e206.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xffe8e1fc, `clr.b` at 0xffe8e282, `clr.b` at 0xffe8e2d0.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8e246.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8e256, `tst.l` at 0xffe8e2a4.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xffe8e286.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8e296.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8e2d6.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xffe8e2da.
  Four-byte load: target `abs 0x300008bc`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e2f6.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e2fc.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e302.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e308.
  Call (`jsr` at 0xffe8e112): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`0x9`, stack arg1=`0xffe8e48b`, stack arg2=`0x119`, stack arg3=`0x0`, stack arg4=`0xffe8e50c`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xffe8e13c): (unknown callee), target 0xffe496fc. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e176): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e1a4): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e1f2): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e22e): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e278): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e2c6): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e2e8): (unknown callee), target 0xffe75138. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe8e0f8): target 0xffe8e11c. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xffe8e1b6): target 0xffe8e192. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xffe8e1d2): target 0xffe8e1fc. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xffe8e252): target 0xffe8e2f0. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xffe8e2a2): target 0xffe8e2f0. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xffe8e2f2): target 0xffe8e126. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe8e30c): target 0xffe8e11c. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xffe8e310 to 0xffe8e318 are not reached by any path from the entry, and their branches are not listed.

- `0xffe66d6c`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe66d6c to 0xffe66fa6 (exclusive), 570 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `386a9ebf97348ed6138751a0d704e8eb661440dbb410b75851b1bbea2c02fa0b`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe66d78.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe66d7c.
  Four-byte load: target `abs 0x30000f9c`. Selected encoded pattern: 1 static access, including `tst.l` at 0xffe66db2.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe66e1e.
  Four-byte load: target `abs 0x30000fa0`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe66e50, `tst.l` at 0xffe66e92.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xffe66e88, `clr.b` at 0xffe66f0e, `clr.b` at 0xffe66f5c.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe66ed2.
  Four-byte load: target `abs 0x30000fa0`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe66ee2, `tst.l` at 0xffe66f30.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xffe66f12.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe66f22.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe66f62.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xffe66f66.
  Four-byte load: target `abs 0x3000074c`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe66f82.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe66f88.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe66f8e.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe66f94.
  Call (`jsr` at 0xffe66d9e): (unknown callee), target 0xffe579f4. Candidate slots: stack arg0=`0x9`, stack arg1=`0xffe67121`, stack arg2=`0x122`, stack arg3=`0x0`, stack arg4=`0xffe671d9`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xffe66dc8): (unknown callee), target 0xffe43610. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66e02): (unknown callee), target 0xffe579f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66e30): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66e7e): (unknown callee), target 0xffe579f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66eba): (unknown callee), target 0xffe579f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66f04): (unknown callee), target 0xffe579f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66f52): (unknown callee), target 0xffe579f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe66f74): (unknown callee), target 0xffe5c788. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe66d84): target 0xffe66da8. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xffe66e42): target 0xffe66e1e. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xffe66e5e): target 0xffe66e88. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xffe66ede): target 0xffe66f7c. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xffe66f2e): target 0xffe66f7c. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xffe66f7e): target 0xffe66db2. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe66f98): target 0xffe66da8. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xffe66f9c to 0xffe66fa4 are not reached by any path from the entry, and their branches are not listed.

- `0xffe8de6c`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#2`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8de6c to 0xffe8e0a6 (exclusive), 570 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x1dfb04, 1754852 bytes, SHA256 `ea36aeea83c9a9aa85c4bc761d80c960851e77a88d69967aa33d58e96f5294f3`.
  Payload: carrier offset 0x1dfb30, 1754808 bytes, SHA256 `4530117d97540ee5a199d8cb8480cd1799ca11b9bf3538d46217ddb13743008a`.
  Read window: SHA256 `95b5740f5392f4636e572c9763877519b55447f8f3600b91f826652d1075c6c1`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8de78.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8de7c.
  Four-byte load: target `abs 0x30001188`. Selected encoded pattern: 1 static access, including `tst.l` at 0xffe8deb2.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8df1e.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8df50, `tst.l` at 0xffe8df92.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xffe8df88, `clr.b` at 0xffe8e00e, `clr.b` at 0xffe8e05c.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8dfd2.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8dfe2, `tst.l` at 0xffe8e030.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xffe8e012.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8e022.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8e062.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xffe8e066.
  Four-byte load: target `abs 0x300008bc`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e082.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e088.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e08e.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e094.
  Call (`jsr` at 0xffe8de9e): (unknown callee), target 0xffe78fa8. Candidate slots: stack arg0=`0x9`, stack arg1=`0xffe8e217`, stack arg2=`0x119`, stack arg3=`0x0`, stack arg4=`0xffe8e298`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xffe8dec8): (unknown callee), target 0xffe494b4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8df02): (unknown callee), target 0xffe78fa8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8df30): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8df7e): (unknown callee), target 0xffe78fa8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8dfba): (unknown callee), target 0xffe78fa8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e004): (unknown callee), target 0xffe78fa8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e052): (unknown callee), target 0xffe78fa8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e074): (unknown callee), target 0xffe74ee0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe8de84): target 0xffe8dea8. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xffe8df42): target 0xffe8df1e. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xffe8df5e): target 0xffe8df88. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xffe8dfde): target 0xffe8e07c. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xffe8e02e): target 0xffe8e07c. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xffe8e07e): target 0xffe8deb2. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe8e098): target 0xffe8dea8. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xffe8e09c to 0xffe8e0a4 are not reached by any path from the entry, and their branches are not listed.

- `0xffe8de7c`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#2`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8de7c to 0xffe8e0b6 (exclusive), 570 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x1dfb24, 1754868 bytes, SHA256 `728c3b39b0aaebd3f8e0ee49e7c42f0d3877d4b8cff52cb06fc0ea4606612802`.
  Payload: carrier offset 0x1dfb50, 1754824 bytes, SHA256 `6d104cefdb289169aa434cde1e6641c316128c7d81149d27fbe448b172d00459`.
  Read window: SHA256 `b0991bfc8a52c60c7db0544cbd91c798eb324ac094df81ff6f69d93139d185e2`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8de88.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8de8c.
  Four-byte load: target `abs 0x30001188`. Selected encoded pattern: 1 static access, including `tst.l` at 0xffe8dec2.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8df2e.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8df60, `tst.l` at 0xffe8dfa2.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xffe8df98, `clr.b` at 0xffe8e01e, `clr.b` at 0xffe8e06c.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8dfe2.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8dff2, `tst.l` at 0xffe8e040.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xffe8e022.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8e032.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8e072.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xffe8e076.
  Four-byte load: target `abs 0x3000092c`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e092.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e098.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e09e.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e0a4.
  Call (`jsr` at 0xffe8deae): (unknown callee), target 0xffe79df8. Candidate slots: stack arg0=`0x9`, stack arg1=`0xffe8e227`, stack arg2=`0x119`, stack arg3=`0x0`, stack arg4=`0xffe8e2a8`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xffe8ded8): (unknown callee), target 0xffe494b4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8df12): (unknown callee), target 0xffe79df8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8df40): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8df8e): (unknown callee), target 0xffe79df8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8dfca): (unknown callee), target 0xffe79df8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e014): (unknown callee), target 0xffe79df8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e062): (unknown callee), target 0xffe79df8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e084): (unknown callee), target 0xffe78654. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe8de94): target 0xffe8deb8. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xffe8df52): target 0xffe8df2e. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xffe8df6e): target 0xffe8df98. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xffe8dfee): target 0xffe8e08c. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xffe8e03e): target 0xffe8e08c. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xffe8e08e): target 0xffe8dec2. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe8e0a8): target 0xffe8deb8. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xffe8e0ac to 0xffe8e0b4 are not reached by any path from the entry, and their branches are not listed.

- `0xffe8e0e0`, `patches/SG0007149/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0007149:l1#2`, version `1.48.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8e0e0 to 0xffe8e31a (exclusive), 570 bytes.
  Carrier: 16027376 bytes, SHA256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `0fb3794227c1ae743f7f0cdd5993df2469b76c71cbf485789deac09838a7cc16`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `13f01e4aae803cd12f181fb1e6ad28e6dc72ae477013e30d1f7b5aff6aa49a81`.
  Read window: SHA256 `9667b10adda8ba7bbee5066a5c4eb1f4376d904aa5a09a5278db5348a1d523db`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8e0ec.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e0f0.
  Four-byte load: target `abs 0x30001188`. Selected encoded pattern: 1 static access, including `tst.l` at 0xffe8e126.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e192.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8e1c4, `tst.l` at 0xffe8e206.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xffe8e1fc, `clr.b` at 0xffe8e282, `clr.b` at 0xffe8e2d0.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8e246.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffe8e256, `tst.l` at 0xffe8e2a4.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xffe8e286.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffe8e296.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xffe8e2d6.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xffe8e2da.
  Four-byte load: target `abs 0x300008bc`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffe8e2f6.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e2fc.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e302.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xffe8e308.
  Call (`jsr` at 0xffe8e112): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`0x9`, stack arg1=`0xffe8e48b`, stack arg2=`0x119`, stack arg3=`0x0`, stack arg4=`0xffe8e50c`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xffe8e13c): (unknown callee), target 0xffe496fc. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e176): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e1a4): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e1f2): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e22e): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e278): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e2c6): (unknown callee), target 0xffe79210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8e2e8): (unknown callee), target 0xffe75138. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe8e0f8): target 0xffe8e11c. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xffe8e1b6): target 0xffe8e192. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xffe8e1d2): target 0xffe8e1fc. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xffe8e252): target 0xffe8e2f0. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xffe8e2a2): target 0xffe8e2f0. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xffe8e2f2): target 0xffe8e126. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe8e30c): target 0xffe8e11c. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xffe8e310 to 0xffe8e318 are not reached by any path from the entry, and their branches are not listed.

- `0xfff7e0e0`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#2`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xfff7e0e0 to 0xfff7e31a (exclusive), 570 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`.
  Read window: SHA256 `c948331f51e8308fa87904119f039215aa7cd06a7ad83ef76f75614288d8b87d`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff7e0ec.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff7e0f0.
  Four-byte load: target `abs 0x30001188`. Selected encoded pattern: 1 static access, including `tst.l` at 0xfff7e126.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff7e192.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xfff7e1c4, `tst.l` at 0xfff7e206.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xfff7e1fc, `clr.b` at 0xfff7e282, `clr.b` at 0xfff7e2d0.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xfff7e246.
  Four-byte load: target `abs 0x3000118c`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xfff7e256, `tst.l` at 0xfff7e2a4.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xfff7e286.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xfff7e296.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff7e2d6.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xfff7e2da.
  Four-byte load: target `abs 0x300008bc`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff7e2f6.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xfff7e2fc.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xfff7e302.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xfff7e308.
  Call (`jsr` at 0xfff7e112): (unknown callee), target 0xfff69210. Candidate slots: stack arg0=`0x9`, stack arg1=`0xfff7e48b`, stack arg2=`0x119`, stack arg3=`0x0`, stack arg4=`0xfff7e50c`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xfff7e13c): (unknown callee), target 0xfff396fc. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e176): (unknown callee), target 0xfff69210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e1a4): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e1f2): (unknown callee), target 0xfff69210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e22e): (unknown callee), target 0xfff69210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e278): (unknown callee), target 0xfff69210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e2c6): (unknown callee), target 0xfff69210. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff7e2e8): (unknown callee), target 0xfff65138. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xfff7e0f8): target 0xfff7e11c. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xfff7e1b6): target 0xfff7e192. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xfff7e1d2): target 0xfff7e1fc. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xfff7e252): target 0xfff7e2f0. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xfff7e2a2): target 0xfff7e2f0. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xfff7e2f2): target 0xfff7e126. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xfff7e30c): target 0xfff7e11c. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xfff7e310 to 0xfff7e318 are not reached by any path from the entry, and their branches are not listed.

- `0xffed6d6c`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffed6d6c to 0xffed6fa6 (exclusive), 570 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `6ddd9e8bca27c8de858aff29858f393f7e0d4800c194a3fd5df4832a0ad898cb`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffed6d78.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffed6d7c.
  Four-byte load: target `abs 0x30000f9c`. Selected encoded pattern: 1 static access, including `tst.l` at 0xffed6db2.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffed6e1e.
  Four-byte load: target `abs 0x30000fa0`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffed6e50, `tst.l` at 0xffed6e92.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 3 static accesses, including `clr.b` at 0xffed6e88, `clr.b` at 0xffed6f0e, `clr.b` at 0xffed6f5c.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffed6ed2.
  Four-byte load: target `abs 0x30000fa0`. Selected encoded pattern: 2 static accesses, including `tst.l` at 0xffed6ee2, `tst.l` at 0xffed6f30.
  Four-byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.l` at 0xffed6f12.
  Byte store: target `unknown`, raw base `a4`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffed6f22.
  Four-byte store: target `unknown`, raw base `a4`, value `0x1`. Selected encoded pattern: 1 static access, including `move.l` at 0xffed6f62.
  Byte store: target `unknown`, raw base `a4`, value `0x0`. Selected encoded pattern: 1 static access, including `clr.b` at 0xffed6f66.
  Four-byte load: target `abs 0x3000074c`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffed6f82.
  Two-byte load: target `unknown`, raw base `a4`. Selected encoded pattern: 1 static access, including `move.w` at 0xffed6f88.
  Two-byte load: target `unknown`, raw base `a0`. Selected encoded pattern: 1 static access, including `move.w` at 0xffed6f8e.
  Two-byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 1 static access, including `move.w` at 0xffed6f94.
  Call (`jsr` at 0xffed6d9e): (unknown callee), target 0xffec79f4. Candidate slots: stack arg0=`0x9`, stack arg1=`0xffed7121`, stack arg2=`0x122`, stack arg3=`0x0`, stack arg4=`0xffed71d9`, stack arg5=`[sp+4]`.
  Call (`jsr` at 0xffed6dc8): (unknown callee), target 0xffeb3610. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6e02): (unknown callee), target 0xffec79f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6e30): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6e7e): (unknown callee), target 0xffec79f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6eba): (unknown callee), target 0xffec79f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6f04): (unknown callee), target 0xffec79f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6f52): (unknown callee), target 0xffec79f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed6f74): (unknown callee), target 0xffecc788. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffed6d84): target 0xffed6da8. Taken when `0x1 == [sp+4]`. Falls through when `0x1 != [sp+4]`.
  Branch (`blt.b` at 0xffed6e42): target 0xffed6e1e. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`bge.b` at 0xffed6e5e): target 0xffed6e88. Taken when `0xa >= unknown (signed, 32-bit)`. Falls through when `0xa < unknown (signed, 32-bit)`.
  Branch (`bgt.w` at 0xffed6ede): target 0xffed6f7c. Taken when `0x64 > unknown (signed, 32-bit)`. Falls through when `0x64 <= unknown (signed, 32-bit)`.
  Branch (`bgt.b` at 0xffed6f2e): target 0xffed6f7c. Taken when `0xa > unknown (signed, 32-bit)`. Falls through when `0xa <= unknown (signed, 32-bit)`.
  Branch (`beq.w` at 0xffed6f7e): target 0xffed6db2. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffed6f98): target 0xffed6da8. The local CFG lists the taken edge and no fall-through edge.
  The instructions from 0xffed6f9c to 0xffed6fa4 are not reached by any path from the entry, and their branches are not listed.

- `0xfff8df94`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#1`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xfff8df94 to 0xfff8e0c8 (exclusive), 308 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0xef9d0, 1784816 bytes, SHA256 `d597012cce4081272d0987ef729afeb15d40a0de6590702331370b07e76dcfae`.
  Payload: carrier offset 0xef9fc, 1784772 bytes, SHA256 `c960b88c14cba5aaca4d5efed9325e5830948f33cb623bf8dbc9887a4b1a8c26`.
  Read window: SHA256 `89f5c5b71a87c1173c04bcea5d8d4b9c3d81f41dae347f880635bd862f60b698`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8df9c.
  Four-byte store: target `sp -0x18`, value `[sp+4]`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfd8.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfdc.
  Four-byte store: target `sp -0x14`, value `[sp+8]`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfdc.
  Four-byte load: target `sp +0xc`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfe2.
  Four-byte store: target `sp -0x10`, value `[sp+12]`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfe2.
  Four-byte load: target `sp +0x14`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfe8.
  Four-byte store: target `sp -0xc`, value `[sp+20]`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfe8.
  Four-byte load: target `sp +0x10`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfee.
  Four-byte store: target `sp -0x8`, value `[sp+16]`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8dfee.
  Four-byte load: target `sp +0xffffffe8`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8e002.
  Four-byte load: target `sp +0xffffffec`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8e004.
  Four-byte load: target `sp +0xfffffff0`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff8e006.
  Four-byte load: target `sp +0xfffffff4`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff8e008.
  Four-byte load: target `sp +0xfffffff8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff8e00a.
  Four-byte load: target `abs 0x300004f8`. Selected encoded pattern: 2 static accesses, including `movea.l` at 0xfff8dfac, `movea.l` at 0xfff8dfd2.
  Byte store: target `[0x300004f8] +0x1c9`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xfff8dfd4.
  Four-byte load: target `abs 0x30000048`. Selected encoded pattern: 1 static access, including `movea.l` at 0xfff8e00c.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `move.l` at 0xfff8e016.
  Byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 6 static accesses, including `move.b` at 0xfff8e03e, `move.b` at 0xfff8e048, `move.b` at 0xfff8e05a.
  Four-byte store: target `unknown`, raw base `a1`, value `unknown`. Selected encoded pattern: 2 static accesses, including `move.l` at 0xfff8e060, `move.l` at 0xfff8e0b2.
  Call (`jsr` at 0xfff8dfa0): (unknown callee), target 0xfff3d2ba. Candidate slots: stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xfff8dfbe): target 0xfff8dfd8. Taken when `unknown == unknown`. Falls through when `unknown != unknown`.
  Branch (`bge.b` at 0xfff8e05e): target 0xfff8e022. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`bgt.b` at 0xfff8e064): target 0xfff8e016. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Branch (`bgt.b` at 0xfff8e0b0): target 0xfff8e072. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  At the exit (`rts` at 0xfff8e0c6), d0=`0x1`.

- `0xffee1bdc`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#0`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffee1bdc to 0xffee1d10 (exclusive), 308 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x0, 977968 bytes, SHA256 `5660c129f64680c9639f50d5f3d7c61428912f2270dde66185a7acccdcf64755`.
  Payload: carrier offset 0x48, 977896 bytes, SHA256 `0fe5e128ec92fb73681f4b9c7e148b1a68de5cf5199c150251c4703beef4e13f`.
  Read window: SHA256 `e142361a6f748a2ba318147cf2971f1c33b551cccdfb4b0045696c3148ad8d9c`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1be4.
  Four-byte store: target `sp -0x18`, value `[sp+4]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c20.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c24.
  Four-byte store: target `sp -0x14`, value `[sp+8]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c24.
  Four-byte load: target `sp +0xc`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c2a.
  Four-byte store: target `sp -0x10`, value `[sp+12]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c2a.
  Four-byte load: target `sp +0x14`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c30.
  Four-byte store: target `sp -0xc`, value `[sp+20]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c30.
  Four-byte load: target `sp +0x10`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c36.
  Four-byte store: target `sp -0x8`, value `[sp+16]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c36.
  Four-byte load: target `sp +0xffffffe8`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c4a.
  Four-byte load: target `sp +0xffffffec`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c4c.
  Four-byte load: target `sp +0xfffffff0`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee1c4e.
  Four-byte load: target `sp +0xfffffff4`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee1c50.
  Four-byte load: target `sp +0xfffffff8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee1c52.
  Four-byte load: target `abs 0x300008b4`. Selected encoded pattern: 2 static accesses, including `movea.l` at 0xffee1bf4, `movea.l` at 0xffee1c1a.
  Byte store: target `[0x300008b4] +0x1c9`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffee1c1c.
  Four-byte load: target `abs 0x30000048`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee1c54.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee1c5e.
  Byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 6 static accesses, including `move.b` at 0xffee1c86, `move.b` at 0xffee1c90, `move.b` at 0xffee1ca2.
  Four-byte store: target `unknown`, raw base `a1`, value `unknown`. Selected encoded pattern: 2 static accesses, including `move.l` at 0xffee1ca8, `move.l` at 0xffee1cfa.
  Call (`jsr` at 0xffee1be8): (unknown callee), target 0xffeb289a. Candidate slots: stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffee1c06): target 0xffee1c20. Taken when `unknown == unknown`. Falls through when `unknown != unknown`.
  Branch (`bge.b` at 0xffee1ca6): target 0xffee1c6a. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`bgt.b` at 0xffee1cac): target 0xffee1c5e. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Branch (`bgt.b` at 0xffee1cf8): target 0xffee1cba. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffee1d0e), d0=`0x1`.

- `0xffee097c`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#1`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffee097c to 0xffee0ab0 (exclusive), 308 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0xeec30, 973412 bytes, SHA256 `8ce4c2cf2d93012d81b4ba6bc7a1deb66de83a32c5cb3052715b3a5d96fb7971`.
  Payload: carrier offset 0xeec5c, 973368 bytes, SHA256 `1640737f85acb71067aab42164e36704e6f1510cd8346bbdca7526e8a63c6b12`.
  Read window: SHA256 `c87a3d2080871e662956572e6c3a5f6a84e5d43588680bccd4d36cffe15c7a08`.
  Four-byte load: target `sp +0x4`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee0984.
  Four-byte store: target `sp -0x18`, value `[sp+4]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09c0.
  Four-byte load: target `sp +0x8`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09c4.
  Four-byte store: target `sp -0x14`, value `[sp+8]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09c4.
  Four-byte load: target `sp +0xc`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09ca.
  Four-byte store: target `sp -0x10`, value `[sp+12]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09ca.
  Four-byte load: target `sp +0x14`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09d0.
  Four-byte store: target `sp -0xc`, value `[sp+20]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09d0.
  Four-byte load: target `sp +0x10`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09d6.
  Four-byte store: target `sp -0x8`, value `[sp+16]`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09d6.
  Four-byte load: target `sp +0xffffffe8`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09ea.
  Four-byte load: target `sp +0xffffffec`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09ec.
  Four-byte load: target `sp +0xfffffff0`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee09ee.
  Four-byte load: target `sp +0xfffffff4`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee09f0.
  Four-byte load: target `sp +0xfffffff8`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee09f2.
  Four-byte load: target `abs 0x3000094c`. Selected encoded pattern: 2 static accesses, including `movea.l` at 0xffee0994, `movea.l` at 0xffee09ba.
  Byte store: target `[0x3000094c] +0x1c9`, value `unknown`. Selected encoded pattern: 1 static access, including `move.b` at 0xffee09bc.
  Four-byte load: target `abs 0x30000048`. Selected encoded pattern: 1 static access, including `movea.l` at 0xffee09f4.
  Four-byte load: target `unknown`, raw base `a5`. Selected encoded pattern: 1 static access, including `move.l` at 0xffee09fe.
  Byte store: target `unknown`, raw base `a0`, value `unknown`. Selected encoded pattern: 6 static accesses, including `move.b` at 0xffee0a26, `move.b` at 0xffee0a30, `move.b` at 0xffee0a42.
  Four-byte store: target `unknown`, raw base `a1`, value `unknown`. Selected encoded pattern: 2 static accesses, including `move.l` at 0xffee0a48, `move.l` at 0xffee0a9a.
  Call (`jsr` at 0xffee0988): (unknown callee), target 0xffeb289a. Candidate slots: stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffee09a6): target 0xffee09c0. Taken when `unknown == unknown`. Falls through when `unknown != unknown`.
  Branch (`bge.b` at 0xffee0a46): target 0xffee0a0a. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`bgt.b` at 0xffee0a4c): target 0xffee09fe. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Branch (`bgt.b` at 0xffee0a98): target 0xffee0a5a. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffee0aae), d0=`0x1`.
