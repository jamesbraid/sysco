# ColdFire L1 entry windows reached by decoded calls

Each entry applies to its exact shipped build, package, payload and load address.
The windows are inferred. Additional function extent and function uniqueness remain unknown.
Roles, types, physical pointees, runtime banks and path feasibility remain unknown.
Local path statements assume ordinary call fallthrough.
Calls show six candidate stack locations. Actual arity and custom ABI remain unknown.
Resolved target expressions and known values stay as printed. Unknown effective addresses show only a raw base.
Counts describe static access records, not dynamic counts or equal runtime pointees.
A pattern groups identical encoded instructions and native access fields within one entry.
Different patterns stay separate even when their displayed unresolved targets agree.
Proved frame linkage and paired entry saves and terminal restores are omitted from the summaries.
Meaningful or ambiguous incoming, local, outgoing and return state is retained.
The common-subset decoder does not establish silicon identity. Unsupported effects remain unknown.
Each selected body is complete through its terminal return within the stated bounds.
Raw padding, unproved pre-entry starts and excluded context tails establish no instructions or caller arguments.
Decoded caller reachability does not prove runtime execution.

The [full facts](disasm/coldfire-variants/called-entry-windows-facts.txt) retain all instructions, constants and warnings.

- `0xffe74410`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe74410 to 0xffe74504 (exclusive), 244 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `25845d7e8c405547b0ac4cb8b276c1436b137058e6cb01f3a2661721719df7af`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7441c.
  Four-byte load, pattern 2: target `abs 0x300012ac`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe7442e.
  Four-byte store, pattern 3: target `sp -0x20`, raw base `a7`, value `0x300012a0`. Count 1 static access, including `pea.l` at 0xffe74436.
  Four-byte store, pattern 4: target `abs 0x3000129c`, raw base `a5`, value `ret(0xffe7c1fc)`. Count 1 static access, including `move.l` at 0xffe74442.
  Four-byte store, pattern 5: target `sp -0x20`, raw base `a7`, value `0x300012b0`. Count 1 static access, including `pea.l` at 0xffe74446.
  Four-byte store, pattern 6: target `abs 0x300012a4`, raw base `a5`, value `ret(0xffe7a70c)`. Count 1 static access, including `move.l` at 0xffe74452.
  Four-byte load, pattern 7: target `abs 0x30000b00`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe74456.
  Four-byte store, pattern 8: target `sp -0x20`, raw base `a7`, value `0x300012b4`. Count 1 static access, including `pea.l` at 0xffe7445c.
  Four-byte store, pattern 9: target `abs 0x300012a8`, raw base `a5`, value `ret(unknown)`. Count 1 static access, including `move.l` at 0xffe74464.
  Four-byte store, pattern 10: target `abs 0x30001298`, raw base `a5`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe74468.
  Four-byte load, pattern 11: target `abs 0x3000129c`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe7446c.
  Four-byte load, pattern 12: target `abs 0x300012a0`, raw base `a5`. Count 1 static access, including `movea.l` at 0xffe74474.
  Byte load, pattern 13: target `unknown`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe74478.
  Four-byte load, pattern 14: target `abs 0x30001298`, raw base `a5`. Count 7 static accesses, including `move.l` at 0xffe74484, 0xffe7448c, 0xffe744b0.
  Four-byte store, pattern 15: target `unknown`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7448a.
  Four-byte store, pattern 16: target `unknown`, raw base `a3`, value `unknown`. Count 1 static access, including `move.l` at 0xffe74490.
  Four-byte load, pattern 17: target `abs 0x30001298`, raw base `a5`. Count 3 static accesses, including `addq.l` at 0xffe74494, 0xffe744c0, 0xffe744e6.
  Four-byte store, pattern 18: target `abs 0x30001298`, raw base `a5`, value `[0x30001298]+1`. Count 3 static accesses, including `addq.l` at 0xffe74494, 0xffe744c0, 0xffe744e6.
  Four-byte load, pattern 19: target `abs 0x3000129c`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe7449c.
  Four-byte load, pattern 20: target `abs 0x300012a4`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe744a4.
  Four-byte store, pattern 21: target `unknown`, raw base `a0`, value `0x1`. Count 1 static access, including `move.l` at 0xffe744b6.
  Four-byte store, pattern 22: target `unknown`, raw base `a1`, value `unknown`. Count 1 static access, including `move.l` at 0xffe744bc.
  Four-byte load, pattern 23: target `abs 0x300012a4`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe744c4.
  Four-byte load, pattern 24: target `abs 0x300012a8`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe744cc.
  Four-byte store, pattern 25: target `unknown`, raw base `a0`, value `0x2`. Count 1 static access, including `move.l` at 0xffe744dc.
  Four-byte store, pattern 26: target `unknown`, raw base `a4`, value `unknown`. Count 1 static access, including `move.l` at 0xffe744e2.
  Four-byte load, pattern 27: target `abs 0x300012a8`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe744ea.
  Four-byte store, pattern 28: target `abs 0x300012ac`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe744f2.
  Four-byte store, pattern 29: target `[sp+4] +0x0`, raw base `a2`, value `0x3000f48c`. Count 1 static access, including `move.l` at 0xffe744f6.
  Call (`jsr` at 0xffe7443a): (unknown callee), target 0xffe7c1fc. Candidate slots: stack arg0=`0x300012a0`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a2`, stack arg4=`a3`, stack arg5=`a4`.
  Call (`jsr` at 0xffe7444a): (unknown callee), target 0xffe7a70c. Candidate slots: stack arg0=`0x300012b0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe74460): (unknown callee), target unknown. Candidate slots: stack arg0=`0x300012b4`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.w` at 0xffe74432): target 0xffe744f6. Taken when `[0x300012ac] != 0x0`. Falls through when `[0x300012ac] == 0x0`.
  Branch (`ble.b` at 0xffe74470): target 0xffe744a2. Taken when `[0x3000129c] <= 0x0 (signed, 32-bit)`. Falls through when `[0x3000129c] > 0x0 (signed, 32-bit)`.
  Branch (`beq.b` at 0xffe74480): target 0xffe74496. Taken when `(? & 0x4) == 0x0`. Falls through when `(? & 0x4) != 0x0`.
  Branch (`blt.b` at 0xffe744a0): target 0xffe74474. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`ble.b` at 0xffe744a8): target 0xffe744ca. Taken when `[0x300012a4] <= 0x0 (signed, 32-bit)`. Falls through when `[0x300012a4] > 0x0 (signed, 32-bit)`.
  Branch (`blt.b` at 0xffe744c8): target 0xffe744ae. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`ble.b` at 0xffe744d0): target 0xffe744f0. Taken when `[0x300012a8] <= 0x0 (signed, 32-bit)`. Falls through when `[0x300012a8] > 0x0 (signed, 32-bit)`.
  Branch (`blt.b` at 0xffe744ee): target 0xffe744d4. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffe74502), d0=`[0x30001298]`.

- `0xffe63984`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe63984 to 0xffe63a78 (exclusive), 244 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `ac7690d205a5260999151b9221ba3a988ac1a80aea595ce0097af2cff3d33255`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe63990.
  Four-byte load, pattern 2: target `abs 0x300011f8`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe639a2.
  Four-byte store, pattern 3: target `sp -0x20`, raw base `a7`, value `0x300011ec`. Count 1 static access, including `pea.l` at 0xffe639aa.
  Four-byte store, pattern 4: target `abs 0x300011e8`, raw base `a5`, value `ret(0xffe69638)`. Count 1 static access, including `move.l` at 0xffe639b6.
  Four-byte store, pattern 5: target `sp -0x20`, raw base `a7`, value `0x300011fc`. Count 1 static access, including `pea.l` at 0xffe639ba.
  Four-byte store, pattern 6: target `abs 0x300011f0`, raw base `a5`, value `ret(0xffe67fdc)`. Count 1 static access, including `move.l` at 0xffe639c6.
  Four-byte load, pattern 7: target `abs 0x30000a28`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe639ca.
  Four-byte store, pattern 8: target `sp -0x20`, raw base `a7`, value `0x30001200`. Count 1 static access, including `pea.l` at 0xffe639d0.
  Four-byte store, pattern 9: target `abs 0x300011f4`, raw base `a5`, value `ret(unknown)`. Count 1 static access, including `move.l` at 0xffe639d8.
  Four-byte store, pattern 10: target `abs 0x300011e4`, raw base `a5`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe639dc.
  Four-byte load, pattern 11: target `abs 0x300011e8`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe639e0.
  Four-byte load, pattern 12: target `abs 0x300011ec`, raw base `a5`. Count 1 static access, including `movea.l` at 0xffe639e8.
  Byte load, pattern 13: target `unknown`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe639ec.
  Four-byte load, pattern 14: target `abs 0x300011e4`, raw base `a5`. Count 7 static accesses, including `move.l` at 0xffe639f8, 0xffe63a00, 0xffe63a24.
  Four-byte store, pattern 15: target `unknown`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe639fe.
  Four-byte store, pattern 16: target `unknown`, raw base `a3`, value `unknown`. Count 1 static access, including `move.l` at 0xffe63a04.
  Four-byte load, pattern 17: target `abs 0x300011e4`, raw base `a5`. Count 3 static accesses, including `addq.l` at 0xffe63a08, 0xffe63a34, 0xffe63a5a.
  Four-byte store, pattern 18: target `abs 0x300011e4`, raw base `a5`, value `[0x300011e4]+1`. Count 3 static accesses, including `addq.l` at 0xffe63a08, 0xffe63a34, 0xffe63a5a.
  Four-byte load, pattern 19: target `abs 0x300011e8`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe63a10.
  Four-byte load, pattern 20: target `abs 0x300011f0`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe63a18.
  Four-byte store, pattern 21: target `unknown`, raw base `a0`, value `0x1`. Count 1 static access, including `move.l` at 0xffe63a2a.
  Four-byte store, pattern 22: target `unknown`, raw base `a1`, value `unknown`. Count 1 static access, including `move.l` at 0xffe63a30.
  Four-byte load, pattern 23: target `abs 0x300011f0`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe63a38.
  Four-byte load, pattern 24: target `abs 0x300011f4`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe63a40.
  Four-byte store, pattern 25: target `unknown`, raw base `a0`, value `0x2`. Count 1 static access, including `move.l` at 0xffe63a50.
  Four-byte store, pattern 26: target `unknown`, raw base `a4`, value `unknown`. Count 1 static access, including `move.l` at 0xffe63a56.
  Four-byte load, pattern 27: target `abs 0x300011f4`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe63a5e.
  Four-byte store, pattern 28: target `abs 0x300011f8`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe63a66.
  Four-byte store, pattern 29: target `[sp+4] +0x0`, raw base `a2`, value `0x3000bb70`. Count 1 static access, including `move.l` at 0xffe63a6a.
  Call (`jsr` at 0xffe639ae): (unknown callee), target 0xffe69638. Candidate slots: stack arg0=`0x300011ec`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a2`, stack arg4=`a3`, stack arg5=`a4`.
  Call (`jsr` at 0xffe639be): (unknown callee), target 0xffe67fdc. Candidate slots: stack arg0=`0x300011fc`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe639d4): (unknown callee), target unknown. Candidate slots: stack arg0=`0x30001200`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.w` at 0xffe639a6): target 0xffe63a6a. Taken when `[0x300011f8] != 0x0`. Falls through when `[0x300011f8] == 0x0`.
  Branch (`ble.b` at 0xffe639e4): target 0xffe63a16. Taken when `[0x300011e8] <= 0x0 (signed, 32-bit)`. Falls through when `[0x300011e8] > 0x0 (signed, 32-bit)`.
  Branch (`beq.b` at 0xffe639f4): target 0xffe63a0a. Taken when `(? & 0x4) == 0x0`. Falls through when `(? & 0x4) != 0x0`.
  Branch (`blt.b` at 0xffe63a14): target 0xffe639e8. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`ble.b` at 0xffe63a1c): target 0xffe63a3e. Taken when `[0x300011f0] <= 0x0 (signed, 32-bit)`. Falls through when `[0x300011f0] > 0x0 (signed, 32-bit)`.
  Branch (`blt.b` at 0xffe63a3c): target 0xffe63a22. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`ble.b` at 0xffe63a44): target 0xffe63a64. Taken when `[0x300011f4] <= 0x0 (signed, 32-bit)`. Falls through when `[0x300011f4] > 0x0 (signed, 32-bit)`.
  Branch (`blt.b` at 0xffe63a62): target 0xffe63a48. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffe63a76), d0=`[0x300011e4]`.

- `0xffe7bca8`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7bca8 to 0xffe7bd9c (exclusive), 244 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `a6234ee4674ee509b88a8f4e739f8fc0bbff9bba8e8f9821d6d60d99f31f2267`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7bcb4.
  Four-byte load, pattern 2: target `abs 0x30001358`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe7bcc6.
  Four-byte store, pattern 3: target `sp -0x20`, raw base `a7`, value `0x3000134c`. Count 1 static access, including `pea.l` at 0xffe7bcce.
  Four-byte store, pattern 4: target `abs 0x30001348`, raw base `a5`, value `ret(0xffe82c34)`. Count 1 static access, including `move.l` at 0xffe7bcda.
  Four-byte store, pattern 5: target `sp -0x20`, raw base `a7`, value `0x3000135c`. Count 1 static access, including `pea.l` at 0xffe7bcde.
  Four-byte store, pattern 6: target `abs 0x30001350`, raw base `a5`, value `ret(0xffe8101c)`. Count 1 static access, including `move.l` at 0xffe7bcea.
  Four-byte load, pattern 7: target `abs 0x30000b64`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7bcee.
  Four-byte store, pattern 8: target `sp -0x20`, raw base `a7`, value `0x30001360`. Count 1 static access, including `pea.l` at 0xffe7bcf4.
  Four-byte store, pattern 9: target `abs 0x30001354`, raw base `a5`, value `ret(unknown)`. Count 1 static access, including `move.l` at 0xffe7bcfc.
  Four-byte store, pattern 10: target `abs 0x30001344`, raw base `a5`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7bd00.
  Four-byte load, pattern 11: target `abs 0x30001348`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe7bd04.
  Four-byte load, pattern 12: target `abs 0x3000134c`, raw base `a5`. Count 1 static access, including `movea.l` at 0xffe7bd0c.
  Byte load, pattern 13: target `unknown`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe7bd10.
  Four-byte load, pattern 14: target `abs 0x30001344`, raw base `a5`. Count 7 static accesses, including `move.l` at 0xffe7bd1c, 0xffe7bd24, 0xffe7bd48.
  Four-byte store, pattern 15: target `unknown`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7bd22.
  Four-byte store, pattern 16: target `unknown`, raw base `a3`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7bd28.
  Four-byte load, pattern 17: target `abs 0x30001344`, raw base `a5`. Count 3 static accesses, including `addq.l` at 0xffe7bd2c, 0xffe7bd58, 0xffe7bd7e.
  Four-byte store, pattern 18: target `abs 0x30001344`, raw base `a5`, value `[0x30001344]+1`. Count 3 static accesses, including `addq.l` at 0xffe7bd2c, 0xffe7bd58, 0xffe7bd7e.
  Four-byte load, pattern 19: target `abs 0x30001348`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe7bd34.
  Four-byte load, pattern 20: target `abs 0x30001350`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe7bd3c.
  Four-byte store, pattern 21: target `unknown`, raw base `a0`, value `0x1`. Count 1 static access, including `move.l` at 0xffe7bd4e.
  Four-byte store, pattern 22: target `unknown`, raw base `a1`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7bd54.
  Four-byte load, pattern 23: target `abs 0x30001350`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe7bd5c.
  Four-byte load, pattern 24: target `abs 0x30001354`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe7bd64.
  Four-byte store, pattern 25: target `unknown`, raw base `a0`, value `0x2`. Count 1 static access, including `move.l` at 0xffe7bd74.
  Four-byte store, pattern 26: target `unknown`, raw base `a4`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7bd7a.
  Four-byte load, pattern 27: target `abs 0x30001354`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe7bd82.
  Four-byte store, pattern 28: target `abs 0x30001358`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe7bd8a.
  Four-byte store, pattern 29: target `[sp+4] +0x0`, raw base `a2`, value `0x3000bff4`. Count 1 static access, including `move.l` at 0xffe7bd8e.
  Call (`jsr` at 0xffe7bcd2): (unknown callee), target 0xffe82c34. Candidate slots: stack arg0=`0x3000134c`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a2`, stack arg4=`a3`, stack arg5=`a4`.
  Call (`jsr` at 0xffe7bce2): (unknown callee), target 0xffe8101c. Candidate slots: stack arg0=`0x3000135c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7bcf8): (unknown callee), target unknown. Candidate slots: stack arg0=`0x30001360`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.w` at 0xffe7bcca): target 0xffe7bd8e. Taken when `[0x30001358] != 0x0`. Falls through when `[0x30001358] == 0x0`.
  Branch (`ble.b` at 0xffe7bd08): target 0xffe7bd3a. Taken when `[0x30001348] <= 0x0 (signed, 32-bit)`. Falls through when `[0x30001348] > 0x0 (signed, 32-bit)`.
  Branch (`beq.b` at 0xffe7bd18): target 0xffe7bd2e. Taken when `(? & 0x4) == 0x0`. Falls through when `(? & 0x4) != 0x0`.
  Branch (`blt.b` at 0xffe7bd38): target 0xffe7bd0c. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`ble.b` at 0xffe7bd40): target 0xffe7bd62. Taken when `[0x30001350] <= 0x0 (signed, 32-bit)`. Falls through when `[0x30001350] > 0x0 (signed, 32-bit)`.
  Branch (`blt.b` at 0xffe7bd60): target 0xffe7bd46. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  Branch (`ble.b` at 0xffe7bd68): target 0xffe7bd88. Taken when `[0x30001354] <= 0x0 (signed, 32-bit)`. Falls through when `[0x30001354] > 0x0 (signed, 32-bit)`.
  Branch (`blt.b` at 0xffe7bd86): target 0xffe7bd6c. Taken when `lt flags condition (operands unknown)`. Falls through when `not (lt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffe7bd9a), d0=`[0x30001344]`.

- `0xffe63b2c`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe63b2c to 0xffe63bc6 (exclusive), 154 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `e2d293922980e53ae2cfcb0c777b27a82d9737aab61e5e084506da17bc08242c`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe63b34.
  Four-byte store, pattern 2: target `sp -0x18`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe63b34.
  Four-byte load, pattern 3: target `ret(0xffe63a78) +0x0`, raw base `a5`. Count 1 static access, including `move.l` at 0xffe63b40.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `sp-5`. Count 3 static accesses, including `pea.l` at 0xffe63b50, 0xffe63b70, 0xffe63b94.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `0x27`. Count 1 static access, including `move.l` at 0xffe63b56.
  Four-byte load, pattern 6: target `ret(0xffe63a78) +0x4`, raw base `a5`. Count 3 static accesses, including `move.l` at 0xffe63b58, 0xffe63b76, 0xffe63b9c.
  Four-byte store, pattern 7: target `sp -0x20`, raw base `a7`, value `unknown`. Count 3 static accesses, including `move.l` at 0xffe63b60, 0xffe63b7e, 0xffe63ba4.
  Four-byte store, pattern 8: target `sp -0x1c`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe63b74.
  Four-byte load, pattern 9: target `abs 0x30000a38`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe63b8e.
  Four-byte store, pattern 10: target `sp -0x1c`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe63b9a.
  Four-byte load, pattern 11: target `sp +0x8`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe63bae.
  Byte load, pattern 12: target `sp -0x5`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe63bb2.
  Four-byte store, pattern 13: target `unknown`, raw base `a0`, value `unknown`. Count 1 static access, including `move.l` at 0xffe63bb8.
  Call (`bsr.w` at 0xffe63b38): (unknown callee), target 0xffe63a78. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`d7`, stack arg2=`a5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`a6`.
  Call (`jsr` at 0xffe63b62): (unknown callee), target 0xffe6980c. Candidate slots: stack arg0=`unknown`, stack arg1=`0x27`, stack arg2=`sp-5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe63b80): (unknown callee), target 0xffe681c8. Candidate slots: stack arg0=`unknown`, stack arg1=`0x0`, stack arg2=`sp-5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe63ba6): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1`, stack arg2=`sp-5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe63b44): target 0xffe63b50. Taken when `[ret(0xffe63a78)] == 0x0`. Falls through when `[ret(0xffe63a78)] != 0x0`.
  Branch (`beq.b` at 0xffe63b48): target 0xffe63b70. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe63b4c): target 0xffe63b8e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe63b4e): target 0xffe63bae. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe63b6e): target 0xffe63bae. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe63b8c): target 0xffe63bae. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe63bc4), d0=`unknown`.
  Effect warning (`extb.l` at 0xffe63bb6): unsupported effects invalidated.

- `0xffe7be50`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7be50 to 0xffe7beea (exclusive), 154 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `78fa90c1b2c5513287f129d9df7fb100c6f7efeec47430ea80a0c328d95c286a`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe7be58.
  Four-byte store, pattern 2: target `sp -0x18`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe7be58.
  Four-byte load, pattern 3: target `ret(0xffe7bd9c) +0x0`, raw base `a5`. Count 1 static access, including `move.l` at 0xffe7be64.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `sp-5`. Count 3 static accesses, including `pea.l` at 0xffe7be74, 0xffe7be94, 0xffe7beb8.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `0x27`. Count 1 static access, including `move.l` at 0xffe7be7a.
  Four-byte load, pattern 6: target `ret(0xffe7bd9c) +0x4`, raw base `a5`. Count 3 static accesses, including `move.l` at 0xffe7be7c, 0xffe7be9a, 0xffe7bec0.
  Four-byte store, pattern 7: target `sp -0x20`, raw base `a7`, value `unknown`. Count 3 static accesses, including `move.l` at 0xffe7be84, 0xffe7bea2, 0xffe7bec8.
  Four-byte store, pattern 8: target `sp -0x1c`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7be98.
  Four-byte load, pattern 9: target `abs 0x30000b74`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7beb2.
  Four-byte store, pattern 10: target `sp -0x1c`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe7bebe.
  Four-byte load, pattern 11: target `sp +0x8`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7bed2.
  Byte load, pattern 12: target `sp -0x5`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe7bed6.
  Four-byte store, pattern 13: target `unknown`, raw base `a0`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7bedc.
  Call (`bsr.w` at 0xffe7be5c): (unknown callee), target 0xffe7bd9c. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`d7`, stack arg2=`a5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`a6`.
  Call (`jsr` at 0xffe7be86): (unknown callee), target 0xffe82e08. Candidate slots: stack arg0=`unknown`, stack arg1=`0x27`, stack arg2=`sp-5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7bea4): (unknown callee), target 0xffe81208. Candidate slots: stack arg0=`unknown`, stack arg1=`0x0`, stack arg2=`sp-5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7beca): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`0x1`, stack arg2=`sp-5`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe7be68): target 0xffe7be74. Taken when `[ret(0xffe7bd9c)] == 0x0`. Falls through when `[ret(0xffe7bd9c)] != 0x0`.
  Branch (`beq.b` at 0xffe7be6c): target 0xffe7be94. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe7be70): target 0xffe7beb2. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe7be72): target 0xffe7bed2. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7be92): target 0xffe7bed2. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7beb0): target 0xffe7bed2. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe7bee8), d0=`unknown`.
  Effect warning (`extb.l` at 0xffe7beda): unsupported effects invalidated.

- `0xffe7d854`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7d854 to 0xffe7d91c (exclusive), 200 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `cbd3ee6eed03c633b23b099670949c5935a90aeef242eebc44ef9deccf3486f3`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe7d860.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7d864.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe7d868, 0xffe7d8bc.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `a7`, value `0x68`. Count 1 static access, including `move.l` at 0xffe7d874.
  Four-byte store, pattern 5: target `sp -0x18`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7d876.
  Four-byte store, pattern 6: target `sp -0x1c`, raw base `a7`, value `[sp+8]`. Count 1 static access, including `move.l` at 0xffe7d878.
  Byte load, pattern 7: target `ret(0xffe7ce00) +0x4`, raw base `a4`. Count 1 static access, including `move.b` at 0xffe7d886.
  Four-byte store, pattern 8: target `[sp+8] +0x0`, raw base `a5`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7d88a.
  Four-byte load, pattern 9: target `[sp+8] +0x0`, raw base `a5`. Count 1 static access, including `move.l` at 0xffe7d8d0.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `a7`, value `[sp+8]`. Count 3 static accesses, including `move.l` at 0xffe7d8e0, 0xffe7d8ec, 0xffe7d8fe.
  Four-byte store, pattern 11: target `sp -0x18`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe7d8e2, 0xffe7d8ee.
  Four-byte load, pattern 12: target `abs 0x30000b18`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7d8f8.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7d904.
  Call (`bsr.w` at 0xffe7d86a): (unknown callee), target 0xffe7ce00. Candidate slots: stack arg0=`unknown`, stack arg1=`d7`, stack arg2=`a4`, stack arg3=`a5`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7d87a): (unknown callee), target 0xffe913b0. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`0x0`, stack arg2=`0x68`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7d8be): (unknown callee), target 0xffe7cd80. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7d8e4): (unknown callee), target 0xffe7d384. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7d8f0): (unknown callee), target 0xffe7d090. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7d906): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe7d896): target 0xffe7d8d0. Taken when `? == 0xff`. Falls through when `? != 0xff`.
  Branch (`beq.b` at 0xffe7d8a2): target 0xffe7d8d0. Taken when `? == 0xfe`. Falls through when `? != 0xfe`.
  Branch (`beq.b` at 0xffe7d8ae): target 0xffe7d8d0. Taken when `? == 0xfd`. Falls through when `? != 0xfd`.
  Branch (`beq.b` at 0xffe7d8ba): target 0xffe7d8d0. Taken when `? == 0xfc`. Falls through when `? != 0xfc`.
  Branch (`bne.b` at 0xffe7d8c6): target 0xffe7d8d0. Taken when `ret(0xffe7cd80) != 0x0`. Falls through when `ret(0xffe7cd80) == 0x0`.
  Jump (`bra.b` at 0xffe7d8ce): target 0xffe7d912. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe7d8d4): target 0xffe7d8e0. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe7d8d8): target 0xffe7d8ec. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe7d8dc): target 0xffe7d8f8. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe7d8de): target 0xffe7d90c. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7d8ea): target 0xffe7d912. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7d8f6): target 0xffe7d912. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7d90a): target 0xffe7d912. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe7d91a), d0=`unknown`.

- `0xffe6aca0`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6aca0 to 0xffe6ad68 (exclusive), 200 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `797c5832f703293ce77f6ed958959e512a86fed7b9d9d06cfad52f5cd9329ce0`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe6acac.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe6acb0.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe6acb4, 0xffe6ad08.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `a7`, value `0x68`. Count 1 static access, including `move.l` at 0xffe6acc0.
  Four-byte store, pattern 5: target `sp -0x18`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe6acc2.
  Four-byte store, pattern 6: target `sp -0x1c`, raw base `a7`, value `[sp+8]`. Count 1 static access, including `move.l` at 0xffe6acc4.
  Byte load, pattern 7: target `ret(0xffe6a244) +0x4`, raw base `a4`. Count 1 static access, including `move.b` at 0xffe6acd2.
  Four-byte store, pattern 8: target `[sp+8] +0x0`, raw base `a5`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6acd6.
  Four-byte load, pattern 9: target `[sp+8] +0x0`, raw base `a5`. Count 1 static access, including `move.l` at 0xffe6ad1c.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `a7`, value `[sp+8]`. Count 3 static accesses, including `move.l` at 0xffe6ad2c, 0xffe6ad38, 0xffe6ad4a.
  Four-byte store, pattern 11: target `sp -0x18`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe6ad2e, 0xffe6ad3a.
  Four-byte load, pattern 12: target `abs 0x30000a40`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe6ad44.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6ad50.
  Call (`bsr.w` at 0xffe6acb6): (unknown callee), target 0xffe6a244. Candidate slots: stack arg0=`unknown`, stack arg1=`d7`, stack arg2=`a4`, stack arg3=`a5`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6acc6): (unknown callee), target 0xffe7a990. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`0x0`, stack arg2=`0x68`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6ad0a): (unknown callee), target 0xffe6a1c4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6ad30): (unknown callee), target 0xffe6a7d0. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6ad3c): (unknown callee), target 0xffe6a4dc. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6ad52): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6ace2): target 0xffe6ad1c. Taken when `? == 0xff`. Falls through when `? != 0xff`.
  Branch (`beq.b` at 0xffe6acee): target 0xffe6ad1c. Taken when `? == 0xfe`. Falls through when `? != 0xfe`.
  Branch (`beq.b` at 0xffe6acfa): target 0xffe6ad1c. Taken when `? == 0xfd`. Falls through when `? != 0xfd`.
  Branch (`beq.b` at 0xffe6ad06): target 0xffe6ad1c. Taken when `? == 0xfc`. Falls through when `? != 0xfc`.
  Branch (`bne.b` at 0xffe6ad12): target 0xffe6ad1c. Taken when `ret(0xffe6a1c4) != 0x0`. Falls through when `ret(0xffe6a1c4) == 0x0`.
  Jump (`bra.b` at 0xffe6ad1a): target 0xffe6ad5e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6ad20): target 0xffe6ad2c. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe6ad24): target 0xffe6ad38. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe6ad28): target 0xffe6ad44. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe6ad2a): target 0xffe6ad58. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe6ad36): target 0xffe6ad5e. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe6ad42): target 0xffe6ad5e. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe6ad56): target 0xffe6ad5e. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6ad66), d0=`unknown`.

- `0xffe8429c`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8429c to 0xffe84364 (exclusive), 200 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `c08bd2aa6896fa1e43e88f0ef3f4d62d091760f45d1d015ed465198c9412e729`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe842a8.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe842ac.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe842b0, 0xffe84304.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `a7`, value `0x68`. Count 1 static access, including `move.l` at 0xffe842bc.
  Four-byte store, pattern 5: target `sp -0x18`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe842be.
  Four-byte store, pattern 6: target `sp -0x1c`, raw base `a7`, value `[sp+8]`. Count 1 static access, including `move.l` at 0xffe842c0.
  Byte load, pattern 7: target `ret(0xffe83840) +0x4`, raw base `a4`. Count 1 static access, including `move.b` at 0xffe842ce.
  Four-byte store, pattern 8: target `[sp+8] +0x0`, raw base `a5`, value `unknown`. Count 1 static access, including `move.l` at 0xffe842d2.
  Four-byte load, pattern 9: target `[sp+8] +0x0`, raw base `a5`. Count 1 static access, including `move.l` at 0xffe84318.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `a7`, value `[sp+8]`. Count 3 static accesses, including `move.l` at 0xffe84328, 0xffe84334, 0xffe84346.
  Four-byte store, pattern 11: target `sp -0x18`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe8432a, 0xffe84336.
  Four-byte load, pattern 12: target `abs 0x30000b7c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe84340.
  Four-byte store, pattern 13: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8434c.
  Call (`bsr.w` at 0xffe842b2): (unknown callee), target 0xffe83840. Candidate slots: stack arg0=`unknown`, stack arg1=`d7`, stack arg2=`a4`, stack arg3=`a5`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe842c2): (unknown callee), target 0xffe9865c. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`0x0`, stack arg2=`0x68`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe84306): (unknown callee), target 0xffe837c0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe8432c): (unknown callee), target 0xffe83dcc. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe84338): (unknown callee), target 0xffe83ad8. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8434e): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe842de): target 0xffe84318. Taken when `? == 0xff`. Falls through when `? != 0xff`.
  Branch (`beq.b` at 0xffe842ea): target 0xffe84318. Taken when `? == 0xfe`. Falls through when `? != 0xfe`.
  Branch (`beq.b` at 0xffe842f6): target 0xffe84318. Taken when `? == 0xfd`. Falls through when `? != 0xfd`.
  Branch (`beq.b` at 0xffe84302): target 0xffe84318. Taken when `? == 0xfc`. Falls through when `? != 0xfc`.
  Branch (`bne.b` at 0xffe8430e): target 0xffe84318. Taken when `ret(0xffe837c0) != 0x0`. Falls through when `ret(0xffe837c0) == 0x0`.
  Jump (`bra.b` at 0xffe84316): target 0xffe8435a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8431c): target 0xffe84328. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe84320): target 0xffe84334. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe84324): target 0xffe84340. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe84326): target 0xffe84354. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe84332): target 0xffe8435a. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe8433e): target 0xffe8435a. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe84352): target 0xffe8435a. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe84362), d0=`unknown`.

- `0xffe61a28`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe61a28 to 0xffe61afc (exclusive), 212 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `ad3a84a1a34c4f465ce4ee0a6606b10eba42b7b73dacda08adfb4fd24990e0cc`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe61a30.
  Byte load, pattern 2: target `sp +0xb`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe61a34.
  Four-byte load, pattern 3: target `[sp+4] +0x4`, raw base `a5`. Count 1 static access, including `and.l` at 0xffe61a4a.
  Four-byte store, pattern 4: target `[sp+4] +0x4`, raw base `a5`, value `unknown`. Count 1 static access, including `move.l` at 0xffe61a50.
  Four-byte load, pattern 5: target `abs 0x30000058`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe61a54.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe61a5a.
  Byte load, pattern 7: target `unknown`, raw base `a0`. Count 2 static accesses, including `move.b` at 0xffe61a76, 0xffe61a84.
  Byte store, pattern 8: target `unknown`, raw base `a0`, value `unknown`. Count 1 static access, including `move.b` at 0xffe61a8c.
  Four-byte store, pattern 9: target `sp -0x10`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe61a96.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `a7`, value `0xffe6360d`. Count 1 static access, including `pea.l` at 0xffe61a98.
  Four-byte store, pattern 11: target `sp -0x18`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe61a9e.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `a7`, value `0xe6c`. Count 1 static access, including `pea.l` at 0xffe61aa0.
  Four-byte store, pattern 13: target `sp -0x20`, raw base `a7`, value `0xffe626be`. Count 1 static access, including `pea.l` at 0xffe61aa4.
  Four-byte store, pattern 14: target `sp -0x24`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe61aac.
  Byte store, pattern 15: target `unknown`, raw base `a5`, value `0xa`. Count 1 static access, including `move.b` at 0xffe61aba.
  Four-byte store, pattern 16: target `sp -0x10`, raw base `a7`, value `0x3`. Count 1 static access, including `move.l` at 0xffe61ac0.
  Four-byte store, pattern 17: target `sp -0x10`, raw base `a7`, value `0xffe6363c`. Count 1 static access, including `pea.l` at 0xffe61acc.
  Four-byte store, pattern 18: target `sp -0x14`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe61ad2.
  Four-byte store, pattern 19: target `sp -0x18`, raw base `a7`, value `0xe75`. Count 1 static access, including `pea.l` at 0xffe61ad4.
  Four-byte store, pattern 20: target `sp -0x1c`, raw base `a7`, value `0xffe626be`. Count 1 static access, including `pea.l` at 0xffe61ad8.
  Four-byte store, pattern 21: target `sp -0x20`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe61ae0.
  Byte store, pattern 22: target `abs 0x30000281`, raw base `absolute`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe61aec.
  Call (`jsr` at 0xffe61a5c): (unknown callee), target unknown. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe61aae): (unknown callee), target 0xffe6a884. Candidate slots: stack arg0=`0x1`, stack arg1=`0xffe626be`, stack arg2=`0xe6c`, stack arg3=`0x0`, stack arg4=`0xffe6360d`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe61ac2): (unknown callee), target 0xffe5d1f4. Candidate slots: stack arg0=`0x3`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe61ae2): (unknown callee), target 0xffe6a884. Candidate slots: stack arg0=`0x1`, stack arg1=`0xffe626be`, stack arg2=`0xe75`, stack arg3=`0x0`, stack arg4=`0xffe6363c`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe61a3a): target 0xffe61a40. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Jump (`bra.b` at 0xffe61a3e): target 0xffe61a42. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.w` at 0xffe61a62): target 0xffe61af2. Taken when `ret(unknown) != 0x0`. Falls through when `ret(unknown) == 0x0`.
  Branch (`ble.b` at 0xffe61a7a): target 0xffe61a90. Taken when `? <= 0x0 (signed, 32-bit)`. Falls through when `? > 0x0 (signed, 32-bit)`.
  Jump (`bra.b` at 0xffe61a8e): target 0xffe61abe. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe61aca): target 0xffe61af2. Taken when `ret(0xffe5d1f4) == 0x0`. Falls through when `ret(0xffe5d1f4) != 0x0`.
  At the exit (`rts` at 0xffe61afa), d0=`unknown`.

- `0xffe712b0`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe712b0 to 0xffe7133a (exclusive), 138 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `d9e8b78fd3454c44fd494dd600b6f4cd98b3d60d74ffb19fdb6982673c3048b9`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe712bc.
  Four-byte load, pattern 2: target `abs 0x30000370`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe712c0.
  Byte load, pattern 3: target `[0x30000370] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe712c8.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71312.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71314.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71316.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71318.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7131a.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7131c.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe72344`. Count 1 static access, including `pea.l` at 0xffe7131e.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe71324.
  Call (`jsr` at 0xffe71326): (unknown callee), target 0xffe78b90. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe72344`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe712d0): target 0xffe712d6. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe712d4): target 0xffe712d8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe712dc): target 0xffe712e2. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe712e0): target 0xffe712e4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe712e8): target 0xffe712ee. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe712ec): target 0xffe712f0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe712f4): target 0xffe712fa. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe712f8): target 0xffe712fc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe712fe): target 0xffe71304. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe71302): target 0xffe71306. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe7130a): target 0xffe71310. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe7130e): target 0xffe71312. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe71338), d0=`ret(0xffe78b90)`.

- `0xffe8ad24`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8ad24 to 0xffe8adae (exclusive), 138 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `9b79a3058f5ac96c48fac81838535b9eda8004d0ba05c28eb2ecfc7b3e9de9a2`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe8ad30.
  Four-byte load, pattern 2: target `abs 0x3000048c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe8ad34.
  Byte load, pattern 3: target `[0x3000048c] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe8ad3c.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad86.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad88.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad8a.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad8c.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad8e.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad90.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe8bdb8`. Count 1 static access, including `pea.l` at 0xffe8ad92.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe8ad98.
  Call (`jsr` at 0xffe8ad9a): (unknown callee), target 0xffe9685c. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe8bdb8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe8ad44): target 0xffe8ad4a. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe8ad48): target 0xffe8ad4c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8ad50): target 0xffe8ad56. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe8ad54): target 0xffe8ad58. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8ad5c): target 0xffe8ad62. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe8ad60): target 0xffe8ad64. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8ad68): target 0xffe8ad6e. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe8ad6c): target 0xffe8ad70. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe8ad72): target 0xffe8ad78. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe8ad76): target 0xffe8ad7a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8ad7e): target 0xffe8ad84. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe8ad82): target 0xffe8ad86. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe8adac), d0=`ret(0xffe9685c)`.

- `0xffe4f330`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4f330 to 0xffe4f3ae (exclusive), 126 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `ce2b973102c14a27d4b4ee2c5072ceda742942ae3b5b036397a63a3b59bcf786`.
  Four-byte store, pattern 1: target `sp -0x18`, raw base `a7`, value `0xffe4f52c`. Count 1 static access, including `pea.l` at 0xffe4f342.
  Four-byte load, pattern 2: target `abs 0x30000370`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe4f360.
  Byte load, pattern 3: target `[0x30000370] +0x41`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe4f366.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `sp-8`. Count 1 static access, including `pea.l` at 0xffe4f370.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe4f374.
  Four-byte load, pattern 6: target `sp -0x8`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe4f382.
  Four-byte load, pattern 7: target `abs 0x30002700`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe4f38e.
  Four-byte load, pattern 8: target `abs 0x30002700`, raw base `a5`. Count 1 static access, including `movea.l` at 0xffe4f394.
  Four-byte store, pattern 9: target `sp -0x18`, raw base `a7`, value `0xff`. Count 1 static access, including `pea.l` at 0xffe4f398.
  Call (`jsr` at 0xffe4f338): (unknown callee), target 0xffe4c4e8. Candidate slots: stack arg0=`d7`, stack arg1=`a5`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4f348): (unknown callee), target 0xffe54d2c. Candidate slots: stack arg0=`0xffe4f52c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4f350): (unknown callee), target 0xffe4c4e8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4f376): (unknown callee), target 0xffe70e84. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4f39c): (unknown callee), target unknown. Candidate slots: stack arg0=`0xff`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe4f35e): target 0xffe4f388. Taken when `0x35 != ?`. Falls through when `0x35 == ?`.
  Branch (`beq.b` at 0xffe4f36e): target 0xffe4f388. Taken when `(? & 0x80) == 0x0`. Falls through when `(? & 0x80) != 0x0`.
  Branch (`bne.b` at 0xffe4f380): target 0xffe4f388. Taken when `ret(0xffe70e84) != 0x0`. Falls through when `ret(0xffe70e84) == 0x0`.
  Jump (`bra.b` at 0xffe4f386): target 0xffe4f3a4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe4f392): target 0xffe4f3a2. Taken when `[0x30002700] == 0x0`. Falls through when `[0x30002700] != 0x0`.
  Jump (`bra.b` at 0xffe4f3a0): target 0xffe4f3a4. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe4f3ac), d0=`unknown`.

- `0xffe5c9fc`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5c9fc to 0xffe5ca7a (exclusive), 126 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `8e118136c6a4e141dfd8bb94f648ef25c56fc65dc074ebace58b81c904f87e3d`.
  Four-byte store, pattern 1: target `sp -0x18`, raw base `a7`, value `0xffe5cbf8`. Count 1 static access, including `pea.l` at 0xffe5ca0e.
  Four-byte load, pattern 2: target `abs 0x3000048c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe5ca2c.
  Byte load, pattern 3: target `[0x3000048c] +0x41`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe5ca32.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `sp-8`. Count 1 static access, including `pea.l` at 0xffe5ca3c.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe5ca40.
  Four-byte load, pattern 6: target `sp -0x8`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe5ca4e.
  Four-byte load, pattern 7: target `abs 0x30002864`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe5ca5a.
  Four-byte load, pattern 8: target `abs 0x30002864`, raw base `a5`. Count 1 static access, including `movea.l` at 0xffe5ca60.
  Four-byte store, pattern 9: target `sp -0x18`, raw base `a7`, value `0xff`. Count 1 static access, including `pea.l` at 0xffe5ca64.
  Call (`jsr` at 0xffe5ca04): (unknown callee), target 0xffe57f7c. Candidate slots: stack arg0=`d7`, stack arg1=`a5`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5ca14): (unknown callee), target 0xffe6d050. Candidate slots: stack arg0=`0xffe5cbf8`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5ca1c): (unknown callee), target 0xffe57f7c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5ca42): (unknown callee), target 0xffe8a8f8. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5ca68): (unknown callee), target unknown. Candidate slots: stack arg0=`0xff`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5ca2a): target 0xffe5ca54. Taken when `0x35 != ?`. Falls through when `0x35 == ?`.
  Branch (`beq.b` at 0xffe5ca3a): target 0xffe5ca54. Taken when `(? & 0x80) == 0x0`. Falls through when `(? & 0x80) != 0x0`.
  Branch (`bne.b` at 0xffe5ca4c): target 0xffe5ca54. Taken when `ret(0xffe8a8f8) != 0x0`. Falls through when `ret(0xffe8a8f8) == 0x0`.
  Jump (`bra.b` at 0xffe5ca52): target 0xffe5ca70. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5ca5e): target 0xffe5ca6e. Taken when `[0x30002864] == 0x0`. Falls through when `[0x30002864] != 0x0`.
  Jump (`bra.b` at 0xffe5ca6c): target 0xffe5ca70. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe5ca78), d0=`unknown`.
