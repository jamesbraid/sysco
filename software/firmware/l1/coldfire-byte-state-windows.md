# ColdFire L1 byte and state windows

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

The [full facts](disasm/coldfire-variants/byte-state-windows-facts.txt) retain all instructions, constants and warnings.

- `0xffe6bbc0`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6bbc0 to 0xffe6bd1c (exclusive), 348 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `ab5578c9e21f5483a86c766b8f2fd905536b005e0c6dc9f45b221d50c6f8eebc`.
  Four-byte load, pattern 1: target `sp +0xc`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe6bbcc.
  Four-byte load, pattern 2: target `abs 0x30000f84`, raw base `absolute`. Count 1 static access, including `tst.l` at 0xffe6bbd2.
  Four-byte store, pattern 3: target `sp -0x34`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe6bbda.
  Four-byte store, pattern 4: target `sp -0x34`, raw base `a7`, value `0x700`. Count 1 static access, including `pea.l` at 0xffe6bbee.
  Four-byte store, pattern 5: target `sp -0x8`, raw base `a6`, value `ret(0xffe4457c)`. Count 1 static access, including `move.l` at 0xffe6bbfa.
  Four-byte load, pattern 6: target `abs 0x30000f8c`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe6bbfe.
  Four-byte load, pattern 7: target `sp +0x4`, raw base `a6`. Count 1 static access, including `add.l` at 0xffe6bc04.
  Four-byte store, pattern 8: target `sp +0x4`, raw base `a6`, value `([sp+4] + [0x30000f8c])`. Count 1 static access, including `add.l` at 0xffe6bc04.
  Four-byte load, pattern 9: target `sp +0x4`, raw base `a6`. Count 1 static access, including `cmp.l` at 0xffe6bc0e.
  Four-byte load, pattern 10: target `sp +0x8`, raw base `a6`. Count 2 static accesses, including `move.l` at 0xffe6bc14, 0xffe6bc42.
  Four-byte load, pattern 11: target `sp +0x4`, raw base `a6`. Count 1 static access, including `add.l` at 0xffe6bc18.
  Four-byte load, pattern 12: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe6bc28.
  Four-byte store, pattern 13: target `[sp+12] +0x0`, raw base `a5`, value `([sp+4] + [0x30000f8c])`. Count 1 static access, including `move.l` at 0xffe6bc28.
  Four-byte load, pattern 14: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe6bc36.
  Four-byte load, pattern 15: target `abs 0x30000f88`, raw base `absolute`. Count 3 static accesses, including `movea.l` at 0xffe6bc4e, 0xffe6bce8, 0xffe6bcf6.
  Four-byte load, pattern 16: target `[0x30000f88] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe6bc54.
  Four-byte load, pattern 17: target `abs 0x30000f90`, raw base `absolute`. Count 1 static access, including `cmp.l` at 0xffe6bc56.
  Four-byte load, pattern 18: target `abs 0x300002ac`, raw base `absolute`. Count 2 static accesses, including `movea.l` at 0xffe6bc66, 0xffe6bc72.
  Byte store, pattern 19: target `[0x300002ac] +0x43`, raw base `a0`, value `0x55`. Count 1 static access, including `move.b` at 0xffe6bc6e.
  Byte store, pattern 20: target `[0x300002ac] +0x43`, raw base `a0`, value `0xaa`. Count 1 static access, including `move.b` at 0xffe6bc7a.
  Four-byte load, pattern 21: target `abs 0x30000f80`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe6bc96.
  Four-byte load, pattern 22: target `unknown`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe6bc9e.
  Four-byte store, pattern 23: target `sp -0x34`, raw base `a7`, value `sp-8`. Count 1 static access, including `pea.l` at 0xffe6bcac.
  Four-byte load, pattern 24: target `abs 0x30000f80`, raw base `absolute`. Count 1 static access, including `add.l` at 0xffe6bcb2.
  Four-byte store, pattern 25: target `sp -0x38`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6bcb8.
  Four-byte store, pattern 26: target `[sp+12] +0x0`, raw base `a5`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6bcd2.
  Four-byte load, pattern 27: target `sp -0x8`, raw base `a6`. Count 2 static accesses, including `move.l` at 0xffe6bcd4, 0xffe6bd04.
  Four-byte store, pattern 28: target `sp -0x34`, raw base `a7`, value `[sp-8]`. Count 2 static accesses, including `move.l` at 0xffe6bcd4, 0xffe6bd04.
  Four-byte load, pattern 29: target `unknown`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe6bcee.
  Four-byte load, pattern 30: target `unknown`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe6bcfc.
  Call (`bsr.w` at 0xffe6bbdc): (unknown callee), target 0xffe6bb24. Candidate slots: stack arg0=`0x0`, stack arg1=`d2`, stack arg2=`d3`, stack arg3=`d4`, stack arg4=`d5`, stack arg5=`d6`.
  Call (`jsr` at 0xffe6bbf2): (unknown callee), target 0xffe4457c. Candidate slots: stack arg0=`0x700`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6bcba): (unknown callee), target 0x30000e50. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6bcd8): (unknown callee), target 0xffe4457c. Candidate slots: stack arg0=`[sp-8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6bd08): (unknown callee), target 0xffe4457c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe6bbd8): target 0xffe6bbee. Taken when `[0x30000f84] != 0x0`. Falls through when `[0x30000f84] == 0x0`.
  Branch (`bpl.b` at 0xffe6bbe6): target 0xffe6bbee. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.w` at 0xffe6bbea): target 0xffe6bd12. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bhi.b` at 0xffe6bc12): target 0xffe6bc24. Taken when `0xffe00000 > ([sp+4] + [0x30000f8c]) (unsigned, 32-bit)`. Falls through when `0xffe00000 <= ([sp+4] + [0x30000f8c]) (unsigned, 32-bit)`.
  Branch (`bls.b` at 0xffe6bc22): target 0xffe6bc36. Taken when `([sp+8] + ([sp+4] + [0x30000f8c])) <= 0xfff00000 (unsigned, 32-bit)`. Falls through when `([sp+8] + ([sp+4] + [0x30000f8c])) > 0xfff00000 (unsigned, 32-bit)`.
  Branch (`beq.b` at 0xffe6bc26): target 0xffe6bc2c. Taken when `[sp+12] == 0x0`. Falls through when `[sp+12] != 0x0`.
  Jump (`bra.w` at 0xffe6bc32): target 0xffe6bd12. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bge.w` at 0xffe6bc5c): target 0xffe6bd04. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`bcs.w` at 0xffe6bc62): target 0xffe6bd04. Taken when `C == 1`. Falls through when `not (C == 1)`.
  Branch (`bne.b` at 0xffe6bc80): target 0xffe6bc8e. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`bcs.w` at 0xffe6bc84): target 0xffe6bce8. Taken when `C == 1`. Falls through when `not (C == 1)`.
  Branch (`bhi.w` at 0xffe6bc8a): target 0xffe6bce8. Taken when `hi flags condition (operands unknown)`. Falls through when `not (hi flags condition (operands unknown))`.
  Branch (`bcc.b` at 0xffe6bc94): target 0xffe6bca8. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Branch (`bne.b` at 0xffe6bca2): target 0xffe6bca8. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe6bca6): target 0xffe6bc92. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe6bcaa): target 0xffe6bce8. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Branch (`bpl.b` at 0xffe6bcc4): target 0xffe6bce8. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Branch (`beq.b` at 0xffe6bcc8): target 0xffe6bcd4. Taken when `[sp+12] == 0x0`. Falls through when `[sp+12] != 0x0`.
  Jump (`bra.b` at 0xffe6bce6): target 0xffe6bd12. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe6bd00): target 0xffe6bc56. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6bd1a), d0=`unknown`.

- `0xffe597a8`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe597a8 to 0xffe5990e (exclusive), 358 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `b59d2cc9dac97a051f386a59ed8b36c55fb9e2165f39e99c219cfb943ad17713`.
  Four-byte load, pattern 1: target `abs 0x300001e8`, raw base `absolute`. Count 18 static accesses, including `movea.l` at 0xffe597b6, 0xffe597cc, 0xffe597d6.
  Four-byte load, pattern 2: target `[0x300001e8] +0xf0`, raw base `a0`. Count 5 static accesses, including `movea.l` at 0xffe597bc, 0xffe597e8, 0xffe59842.
  Four-byte load, pattern 3: target `abs 0x300001e8`, raw base `absolute`. Count 5 static accesses, including `movea.l` at 0xffe597c0, 0xffe597ec, 0xffe59846.
  Four-byte load, pattern 4: target `[0x300001e8] +0xec`, raw base `a1`. Count 5 static accesses, including `cmpa.l` at 0xffe597c6, 0xffe597f2, 0xffe5984c.
  Four-byte load, pattern 5: target `[0x300001e8] +0xf0`, raw base `a0`. Count 2 static accesses, including `move.l` at 0xffe597d2, 0xffe597fe.
  Four-byte load, pattern 6: target `[0x300001e8] +0xec`, raw base `a0`. Count 2 static accesses, including `sub.l` at 0xffe597dc, 0xffe598c4.
  Four-byte load, pattern 7: target `[0x300001e8] +0xe4`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe59808.
  Four-byte load, pattern 8: target `[0x300001e8] +0xe8`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe59812.
  Four-byte load, pattern 9: target `[0x300001e8] +0xec`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe5981c.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `a7`, value `unknown`. Count 1 static access, including `pea.l` at 0xffe59824.
  Byte store, pattern 11: target `ret(0xffe59c58) +0x0`, raw base `a5`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe59852.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe5986e, 0xffe598c8.
  Four-byte load, pattern 13: target `[0x300001e8] +0xec`, raw base `a0`. Count 2 static accesses, including `move.l` at 0xffe59876, 0xffe598d0.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `a7`, value `[[0x300001e8]+236]`. Count 2 static accesses, including `move.l` at 0xffe59876, 0xffe598d0.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `a7`, value `ret(0xffe59c58)`. Count 2 static accesses, including `move.l` at 0xffe5987a, 0xffe598d4.
  Four-byte load, pattern 16: target `[0x300001e8] +0xf0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe598a6.
  Four-byte load, pattern 17: target `[0x300001e8] +0xe4`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe598b0.
  Four-byte load, pattern 18: target `[0x300001e8] +0xe8`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe598ba.
  Four-byte store, pattern 19: target `sp -0x14`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe598e0.
  Four-byte load, pattern 20: target `[0x300001e8] +0xe4`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe598e8.
  Four-byte store, pattern 21: target `sp -0x18`, raw base `a7`, value `[[0x300001e8]+228]`. Count 1 static access, including `move.l` at 0xffe598e8.
  Four-byte store, pattern 22: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe598f0.
  Byte store, pattern 23: target `unknown`, raw base `a5`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe598fe.
  Call (`jsr` at 0xffe59828): (unknown callee), target 0xffe59c58. Candidate slots: stack arg0=`unknown`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a5`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5987c): (unknown callee), target 0xffe75f3c. Candidate slots: stack arg0=`ret(0xffe59c58)`, stack arg1=`[[0x300001e8]+236]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe598d6): (unknown callee), target 0xffe75f3c. Candidate slots: stack arg0=`ret(0xffe59c58)`, stack arg1=`[[0x300001e8]+236]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe598f2): (unknown callee), target 0xffe75f3c. Candidate slots: stack arg0=`unknown`, stack arg1=`[[0x300001e8]+228]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bls.b` at 0xffe597ca): target 0xffe597e2. Taken when `[[0x300001e8]+240] <= [[0x300001e8]+236] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] > [[0x300001e8]+236] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe597e0): target 0xffe59822. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe597f6): target 0xffe59822. Taken when `[[0x300001e8]+240] >= [[0x300001e8]+236] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] < [[0x300001e8]+236] (unsigned, 32-bit)`.
  Branch (`bne.b` at 0xffe59834): target 0xffe5983c. Taken when `ret(0xffe59c58) != 0x0`. Falls through when `ret(0xffe59c58) == 0x0`.
  Jump (`bra.w` at 0xffe59838): target 0xffe59904. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe59850): target 0xffe59858. Taken when `[[0x300001e8]+240] != [[0x300001e8]+236]`. Falls through when `[[0x300001e8]+240] == [[0x300001e8]+236]`.
  Jump (`bra.w` at 0xffe59854): target 0xffe598fe. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bls.b` at 0xffe5986c): target 0xffe5988a. Taken when `[[0x300001e8]+240] <= [[0x300001e8]+236] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] > [[0x300001e8]+236] (unsigned, 32-bit)`.
  Jump (`bra.w` at 0xffe59886): target 0xffe598fe. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe5989e): target 0xffe598fe. Taken when `[[0x300001e8]+240] >= [[0x300001e8]+236] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] < [[0x300001e8]+236] (unsigned, 32-bit)`.
  At the exit (`rts` at 0xffe5990c), d0=`unknown`.

- `0xffe724a8`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe724a8 to 0xffe72632 (exclusive), 394 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `05edeca7118b0595929b49b7b3edad171ee8f640b6400e7c18055c1570cf2b84`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe724b4.
  Four-byte load, pattern 2: target `abs 0x30000058`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe724b8.
  Four-byte load, pattern 3: target `unknown`, raw base `a4`. Count 1 static access, including `move.l` at 0xffe724c4.
  Four-byte load, pattern 4: target `[sp+4] +0x0`, raw base `a5`. Count 1 static access, including `cmp.l` at 0xffe724c6.
  Four-byte load, pattern 5: target `[sp+4] +0x0`, raw base `a5`. Count 1 static access, including `move.l` at 0xffe724cc.
  Four-byte load, pattern 6: target `unknown`, raw base `a4`. Count 1 static access, including `cmp.l` at 0xffe724d4.
  Four-byte store, pattern 7: target `sp -0x3c`, raw base `a7`, value `0xffffffff`. Count 1 static access, including `pea.l` at 0xffe724da.
  Four-byte store, pattern 8: target `sp -0x40`, raw base `a7`, value `0x30002700`. Count 1 static access, including `pea.l` at 0xffe724de.
  Four-byte store, pattern 9: target `sp -0x3c`, raw base `a7`, value `ret(0xffe46bb4)`. Count 1 static access, including `move.l` at 0xffe724f2.
  Four-byte store, pattern 10: target `sp -0x40`, raw base `a7`, value `0x1`. Count 2 static accesses, including `pea.l` at 0xffe724f4, 0xffe72530.
  Four-byte store, pattern 11: target `sp -0x44`, raw base `a7`, value `0x511`. Count 1 static access, including `pea.l` at 0xffe724f8.
  Four-byte store, pattern 12: target `sp -0x48`, raw base `a7`, value `0xffe73088`. Count 2 static accesses, including `pea.l` at 0xffe724fc, 0xffe72538.
  Four-byte load, pattern 13: target `sp +0x8`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7250c.
  Two-byte load, pattern 14: target `[sp+8] +0x4`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe72510.
  Two-byte store, pattern 15: target `[sp+8] +0x4`, raw base `a0`, value `unknown`. Count 1 static access, including `move.w` at 0xffe72516.
  Four-byte store, pattern 16: target `sp -0x3c`, raw base `a7`, value `0x30002700`. Count 1 static access, including `pea.l` at 0xffe7251a.
  Four-byte store, pattern 17: target `sp -0x3c`, raw base `a7`, value `ret(0xffe46c0c)`. Count 1 static access, including `move.l` at 0xffe7252e.
  Four-byte store, pattern 18: target `sp -0x44`, raw base `a7`, value `0x513`. Count 1 static access, including `pea.l` at 0xffe72534.
  Two-byte load, pattern 19: target `unknown`, raw base `a4`. Count 1 static access, including `move.w` at 0xffe7254a.
  Two-byte load, pattern 20: target `unknown`, raw base `a4`. Count 1 static access, including `move.w` at 0xffe72556.
  Two-byte load, pattern 21: target `unknown`, raw base `a4`. Count 4 static accesses, including `move.w` at 0xffe72560, 0xffe7256c, 0xffe725f0.
  Two-byte store, pattern 22: target `unknown`, raw base `a4`, value `unknown`. Count 2 static accesses, including `move.w` at 0xffe72572, 0xffe72602.
  Four-byte load, pattern 23: target `sp +0x8`, raw base `a6`. Count 3 static accesses, including `move.l` at 0xffe72576, 0xffe72606, 0xffe72616.
  Four-byte store, pattern 24: target `sp -0x3c`, raw base `a7`, value `[sp+8]`. Count 3 static accesses, including `move.l` at 0xffe72576, 0xffe72606, 0xffe72616.
  Four-byte store, pattern 25: target `sp -0x3c`, raw base `a7`, value `0x1f`. Count 1 static access, including `pea.l` at 0xffe72584.
  Four-byte store, pattern 26: target `sp -0x40`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe72588.
  Four-byte store, pattern 27: target `sp -0x44`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7258a.
  Four-byte store, pattern 28: target `sp -0x48`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7258c.
  Four-byte load, pattern 29: target `abs 0x30000074`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7258e.
  Four-byte load, pattern 30: target `[0x30000074] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe72594.
  Four-byte store, pattern 31: target `sp -0x4c`, raw base `a7`, value `[[0x30000074]]`. Count 1 static access, including `move.l` at 0xffe72594.
  Four-byte store, pattern 32: target `sp -0x50`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe72596.
  Four-byte store, pattern 33: target `sp -0x54`, raw base `a7`, value `sp+8`. Count 1 static access, including `pea.l` at 0xffe7259a.
  Four-byte load, pattern 34: target `unknown`, raw base `a4`. Count 1 static access, including `move.l` at 0xffe7259e.
  Four-byte store, pattern 35: target `sp -0x58`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7259e.
  Two-byte store, pattern 36: target `unknown`, raw base `a4`, value `0x1`. Count 1 static access, including `move.w` at 0xffe725c2.
  Four-byte load, pattern 37: target `unknown`, raw base `a4`. Count 2 static accesses, including `movea.l` at 0xffe725c6, 0xffe72612.
  Four-byte store, pattern 38: target `sp -0x3c`, raw base `a7`, value `unknown`. Count 1 static access, including `pea.l` at 0xffe725ca.
  Four-byte store, pattern 39: target `sp -0x40`, raw base `a7`, value `0xffe73154`. Count 1 static access, including `pea.l` at 0xffe725ce.
  Four-byte store, pattern 40: target `sp -0x44`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe725d4.
  Four-byte store, pattern 41: target `sp -0x3c`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe725e2.
  Four-byte load, pattern 42: target `unknown`, raw base `a4`. Count 1 static access, including `movea.l` at 0xffe7261e.
  Call (`jsr` at 0xffe724e4): (unknown callee), target 0xffe46bb4. Candidate slots: stack arg0=`0x30002700`, stack arg1=`0xffffffff`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe72502): (unknown callee), target 0xffe5a1b8. Candidate slots: stack arg0=`0xffe73088`, stack arg1=`0x511`, stack arg2=`0x1`, stack arg3=`ret(0xffe46bb4)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe72520): (unknown callee), target 0xffe46c0c. Candidate slots: stack arg0=`0x30002700`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7253e): (unknown callee), target 0xffe5a1b8. Candidate slots: stack arg0=`0xffe73088`, stack arg1=`0x513`, stack arg2=`0x1`, stack arg3=`ret(0xffe46c0c)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7257a): (unknown callee), target 0xffe71f8c. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe725a2): (unknown callee), target 0xffe460e0. Candidate slots: stack arg0=`unknown`, stack arg1=`sp+8`, stack arg2=`0x1`, stack arg3=`[[0x30000074]]`, stack arg4=`0x0`, stack arg5=`0x0`.
  Call (`jsr` at 0xffe725d8): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`sp-44`, stack arg1=`0xffe73154`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe725e6): (unknown callee), target 0xffe59778. Candidate slots: stack arg0=`sp-44`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7260a): (unknown callee), target 0xffe71f8c. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7261a): (unknown callee), target unknown. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.w` at 0xffe724c0): target 0xffe72626. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.w` at 0xffe724c8): target 0xffe7261e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.w` at 0xffe724d6): target 0xffe7261e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe724f0): target 0xffe7250c. Taken when `ret(0xffe46bb4) == 0x0`. Falls through when `ret(0xffe46bb4) != 0x0`.
  Branch (`beq.b` at 0xffe7252c): target 0xffe72548. Taken when `ret(0xffe46c0c) == 0x0`. Falls through when `ret(0xffe46c0c) != 0x0`.
  Branch (`bne.w` at 0xffe72550): target 0xffe72612. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`beq.b` at 0xffe7255c): target 0xffe72584. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bge.b` at 0xffe7256a): target 0xffe72576. Taken when `? >= 0xffff (signed, 32-bit)`. Falls through when `? < 0xffff (signed, 32-bit)`.
  Jump (`bra.w` at 0xffe72580): target 0xffe7261e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.w` at 0xffe725b0): target 0xffe7261e. Taken when `ret(0xffe460e0) == 0x0`. Falls through when `ret(0xffe460e0) != 0x0`.
  Branch (`beq.b` at 0xffe725b8): target 0xffe725c0. Taken when `0xffffffce == ret(0xffe460e0)`. Falls through when `0xffffffce != ret(0xffe460e0)`.
  Branch (`bne.b` at 0xffe725be): target 0xffe72606. Taken when `0xffffffd2 != ret(0xffe460e0)`. Falls through when `0xffffffd2 == ret(0xffe460e0)`.
  Branch (`bge.b` at 0xffe725fa): target 0xffe72606. Taken when `? >= 0xffff (signed, 32-bit)`. Falls through when `? < 0xffff (signed, 32-bit)`.
  Jump (`bra.b` at 0xffe72610): target 0xffe7261e. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe72622): target 0xffe724be. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe72630), d0=`0x0`.

- `0xffe59544`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe59544 to 0xffe59776 (exclusive), 562 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `201d705248f62f433504e27dc08a16d273d5c9f03550cfce464837289954c3b5`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe59550.
  Four-byte store, pattern 2: target `sp -0x1c`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe59554.
  Four-byte load, pattern 3: target `abs 0x300001e8`, raw base `absolute`. Count 25 static accesses, including `movea.l` at 0xffe59562, 0xffe5956c, 0xffe5959e.
  Four-byte load, pattern 4: target `[0x300001e8] +0xe8`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe59568.
  Four-byte load, pattern 5: target `[0x300001e8] +0xe4`, raw base `a0`. Count 2 static accesses, including `sub.l` at 0xffe59572, 0xffe595c4.
  Byte load, pattern 6: target `([sp+4] + ret(0xffe76154)) -0x1`, raw base `a5`. Count 1 static access, including `move.b` at 0xffe59576.
  Four-byte store, pattern 7: target `unknown`, raw base `a7`, value `0xffffffff`. Count 1 static access, including `pea.l` at 0xffe59584.
  Four-byte store, pattern 8: target `unknown`, raw base `a7`, value `0x30002d60`. Count 2 static accesses, including `pea.l` at 0xffe59588, 0xffe5973e.
  Four-byte load, pattern 9: target `[0x300001e8] +0xf0`, raw base `a0`. Count 6 static accesses, including `movea.l` at 0xffe595a4, 0xffe595e6, 0xffe5967a.
  Four-byte load, pattern 10: target `abs 0x300001e8`, raw base `absolute`. Count 10 static accesses, including `movea.l` at 0xffe595a8, 0xffe595ea, 0xffe5963e.
  Four-byte load, pattern 11: target `[0x300001e8] +0xec`, raw base `a1`. Count 2 static accesses, including `cmpa.l` at 0xffe595ae, 0xffe595f0.
  Four-byte load, pattern 12: target `[0x300001e8] +0xec`, raw base `a0`. Count 2 static accesses, including `move.l` at 0xffe595ba, 0xffe595fc.
  Four-byte load, pattern 13: target `[0x300001e8] +0xe8`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe595ce.
  Four-byte load, pattern 14: target `[0x300001e8] +0xf0`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe595d8.
  Four-byte load, pattern 15: target `[0x300001e8] +0xf0`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe59606.
  Four-byte load, pattern 16: target `[0x300001e8] +0xec`, raw base `a0`. Count 2 static accesses, including `movea.l` at 0xffe5961a, 0xffe5963a.
  Byte load, pattern 17: target `[[0x300001e8]+236] +0x0`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe5961e.
  Four-byte load, pattern 18: target `[0x300001e8] +0xec`, raw base `a0`. Count 2 static accesses, including `addq.l` at 0xffe5962e, 0xffe59666.
  Four-byte store, pattern 19: target `[0x300001e8] +0xec`, raw base `a0`, value `[[0x300001e8]+236]+1`. Count 2 static accesses, including `addq.l` at 0xffe5962e, 0xffe59666.
  Four-byte load, pattern 20: target `[0x300001e8] +0xe8`, raw base `a1`. Count 3 static accesses, including `cmpa.l` at 0xffe59644, 0xffe5969a, 0xffe596e4.
  Four-byte load, pattern 21: target `[0x300001e8] +0xe4`, raw base `a1`. Count 3 static accesses, including `move.l` at 0xffe59656, 0xffe596ac, 0xffe596f6.
  Four-byte store, pattern 22: target `[0x300001e8] +0xec`, raw base `a0`, value `[[0x300001e8]+228]`. Count 1 static access, including `move.l` at 0xffe5965a.
  Byte load, pattern 23: target `unknown`, raw base `a5`. Count 1 static access, including `move.b` at 0xffe5967e.
  Byte store, pattern 24: target `[[0x300001e8]+240] +0x0`, raw base `a0`, value `unknown`. Count 1 static access, including `move.b` at 0xffe5967e.
  Four-byte load, pattern 25: target `[0x300001e8] +0xf0`, raw base `a0`. Count 2 static accesses, including `addq.l` at 0xffe59686, 0xffe596d0.
  Four-byte store, pattern 26: target `[0x300001e8] +0xf0`, raw base `a0`, value `[[0x300001e8]+240]+1`. Count 2 static accesses, including `addq.l` at 0xffe59686, 0xffe596d0.
  Four-byte store, pattern 27: target `[0x300001e8] +0xf0`, raw base `a0`, value `[[0x300001e8]+228]`. Count 2 static accesses, including `move.l` at 0xffe596b0, 0xffe596fa.
  Byte store, pattern 28: target `[[0x300001e8]+240] +0x0`, raw base `a0`, value `0xa`. Count 1 static access, including `move.b` at 0xffe596c8.
  Four-byte store, pattern 29: target `unknown`, raw base `a7`, value `0x24`. Count 1 static access, including `pea.l` at 0xffe596fe.
  Four-byte store, pattern 30: target `unknown`, raw base `a7`, value `[0x300001e8]+224`. Count 1 static access, including `pea.l` at 0xffe59708.
  Byte store, pattern 31: target `[0x300001e8] +0x103`, raw base `a0`, value `(ret(0xffe5e8d8) & 0xff)`. Count 1 static access, including `move.b` at 0xffe5971a.
  Four-byte load, pattern 32: target `unknown`, raw base `a0`. Count 3 static accesses, including `move.l` at 0xffe59734, 0xffe59736, 0xffe5973c.
  Four-byte store, pattern 33: target `unknown`, raw base `a1`, value `unknown`. Count 3 static accesses, including `move.l` at 0xffe59734, 0xffe59736, 0xffe5973c.
  Four-byte store, pattern 34: target `unknown`, raw base `a7`, value `ret(0xffe46c0c)`. Count 1 static access, including `move.l` at 0xffe59752.
  Four-byte store, pattern 35: target `unknown`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe59754.
  Four-byte store, pattern 36: target `unknown`, raw base `a7`, value `0x478`. Count 1 static access, including `pea.l` at 0xffe59758.
  Four-byte store, pattern 37: target `unknown`, raw base `a7`, value `0xffe599f4`. Count 1 static access, including `pea.l` at 0xffe5975c.
  Call (`jsr` at 0xffe59556): (unknown callee), target 0xffe76154. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`d4`, stack arg2=`d5`, stack arg3=`d6`, stack arg4=`d7`, stack arg5=`a5`.
  Call (`jsr` at 0xffe5958e): (unknown callee), target 0xffe46bb4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5970c): (unknown callee), target 0xffe5e8d8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe59744): (unknown callee), target 0xffe46c0c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe59762): (unknown callee), target 0xffe5a1b8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe59580): target 0xffe59584. Taken when `0xa == ?`. Falls through when `0xa != ?`.
  Branch (`bne.w` at 0xffe5959a): target 0xffe5976c. Taken when `ret(0xffe46bb4) != 0x0`. Falls through when `ret(0xffe46bb4) == 0x0`.
  Branch (`bls.b` at 0xffe595b2): target 0xffe595e0. Taken when `[[0x300001e8]+240] <= [[0x300001e8]+236] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] > [[0x300001e8]+236] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe595de): target 0xffe5960a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe595f4): target 0xffe5960a. Taken when `[[0x300001e8]+240] >= [[0x300001e8]+236] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] < [[0x300001e8]+236] (unsigned, 32-bit)`.
  Branch (`bgt.b` at 0xffe59612): target 0xffe5966e. Taken when `? > ? (signed, 32-bit)`. Falls through when `? <= ? (signed, 32-bit)`.
  Branch (`beq.b` at 0xffe59626): target 0xffe59660. Taken when `0xa == ?`. Falls through when `0xa != ?`.
  Branch (`bcs.b` at 0xffe59648): target 0xffe59614. Taken when `[[0x300001e8]+236] < [[0x300001e8]+232] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+236] >= [[0x300001e8]+232] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe5965e): target 0xffe59614. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe5966c): target 0xffe5960a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe59670): target 0xffe596b6. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bcs.b` at 0xffe5969e): target 0xffe5966e. Taken when `[[0x300001e8]+240] < [[0x300001e8]+232] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] >= [[0x300001e8]+232] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe596b4): target 0xffe5966e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe596ba): target 0xffe596fe. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bcs.b` at 0xffe596e8): target 0xffe596fe. Taken when `[[0x300001e8]+240] < [[0x300001e8]+232] (unsigned, 32-bit)`. Falls through when `[[0x300001e8]+240] >= [[0x300001e8]+232] (unsigned, 32-bit)`.
  Branch (`bgt.b` at 0xffe5973a): target 0xffe59734. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe59750): target 0xffe5976c. Taken when `ret(0xffe46c0c) == 0x0`. Falls through when `ret(0xffe46c0c) != 0x0`.
  At the exit (`rts` at 0xffe59774), d0=`unknown`.
  Effect warning (`extb.l` at 0xffe5957a): unsupported effects invalidated.
  Effect warning (`extb.l` at 0xffe5960c): unsupported effects invalidated.
  Effect warning (`extb.l` at 0xffe59620): unsupported effects invalidated.

- `0xffe6caa0`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6caa0 to 0xffe6cb2a (exclusive), 138 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `55c962fd72b07f86d3dad60b1a1f0e09477433686b2e50afc52c4ba2b3ec8db5`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe6caac.
  Four-byte load, pattern 2: target `abs 0x3000074c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe6cab0.
  Byte load, pattern 3: target `[0x3000074c] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe6cab8.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6cb02.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6cb04.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6cb06.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6cb08.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6cb0a.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6cb0c.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe6ce94`. Count 1 static access, including `pea.l` at 0xffe6cb0e.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe6cb14.
  Call (`jsr` at 0xffe6cb16): (unknown callee), target 0xffe72d64. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe6ce94`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6cac0): target 0xffe6cac6. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe6cac4): target 0xffe6cac8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6cacc): target 0xffe6cad2. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe6cad0): target 0xffe6cad4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6cad8): target 0xffe6cade. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe6cadc): target 0xffe6cae0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6cae4): target 0xffe6caea. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe6cae8): target 0xffe6caec. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe6caee): target 0xffe6caf4. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe6caf2): target 0xffe6caf6. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6cafa): target 0xffe6cb00. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe6cafe): target 0xffe6cb02. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6cb28), d0=`ret(0xffe72d64)`.

- `0xffe70fe8`, `6.5.15/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.15:l1#0`, version `1.10.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe70fe8 to 0xffe71072 (exclusive), 138 bytes.
  Carrier: 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Package: carrier offset 0x0, 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Payload: carrier offset 0x2c, 972800 bytes, SHA256 `18b09029d3c94d99ff6ddec9215755a6fc4933d212d53830db005768793dea73`.
  Read window: SHA256 `fd28ea74bb20598a143944cbc060ed372103b1df7ecbf2fb053a57aa564432c5`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe70ff4.
  Four-byte load, pattern 2: target `abs 0x300007d8`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe70ff8.
  Byte load, pattern 3: target `[0x300007d8] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe71000.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7104a.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7104c.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7104e.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71050.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71052.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71054.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe713dc`. Count 1 static access, including `pea.l` at 0xffe71056.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe7105c.
  Call (`jsr` at 0xffe7105e): (unknown callee), target 0xffe77528. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe713dc`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe71008): target 0xffe7100e. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe7100c): target 0xffe71010. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71014): target 0xffe7101a. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe71018): target 0xffe7101c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71020): target 0xffe71026. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe71024): target 0xffe71028. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe7102c): target 0xffe71032. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe71030): target 0xffe71034. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe71036): target 0xffe7103c. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe7103a): target 0xffe7103e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71042): target 0xffe71048. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe71046): target 0xffe7104a. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe71070), d0=`ret(0xffe77528)`.

- `0xffe76c3c`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#0`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe76c3c to 0xffe76d58 (exclusive), 284 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0x0, 974628 bytes, SHA256 `c091d32e310b7104130380d6c7f6c1e0a48f9cc4c79f13928860c1be37d52419`.
  Payload: carrier offset 0x40, 974564 bytes, SHA256 `a0a6cfec2692b7267ff56d52e46f1c75a39a623048dcd942835c816e4dde8ce0`.
  Read window: SHA256 `cc7f38d142c8bccce6e1e4c8f670f12acaba23391c13d14d378ce2235f7ed0cb`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe76c44.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 2 static accesses, including `movea.l` at 0xffe76c48, 0xffe76cb6.
  Byte load, pattern 3: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe76c4e.
  Four-byte load, pattern 4: target `abs 0x300000a4`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe76c56.
  Four-byte load, pattern 5: target `[0x300000a4] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe76c5c.
  Four-byte load, pattern 6: target `abs 0x300000a0`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe76c60.
  Four-byte load, pattern 7: target `[0x300000a0] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe76c66.
  Four-byte store, pattern 8: target `sp -0x34`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe76c68.
  Four-byte store, pattern 9: target `sp -0x30`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe76c6c.
  Four-byte store, pattern 10: target `sp -0x40`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe76c72.
  Four-byte store, pattern 11: target `sp -0x44`, raw base `a7`, value `sp-52`. Count 1 static access, including `pea.l` at 0xffe76c74.
  Four-byte store, pattern 12: target `sp -0x48`, raw base `a7`, value `0xffe76790`. Count 1 static access, including `pea.l` at 0xffe76c78.
  Four-byte store, pattern 13: target `sp -0x4c`, raw base `a7`, value `0xffe7670c`. Count 1 static access, including `pea.l` at 0xffe76c7e.
  Four-byte store, pattern 14: target `sp -0x50`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe76c84.
  Four-byte store, pattern 15: target `sp -0x54`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe76c88.
  Four-byte store, pattern 16: target `sp -0x58`, raw base `a7`, value `sp+8`. Count 1 static access, including `pea.l` at 0xffe76c8a.
  Four-byte store, pattern 17: target `sp -0x5c`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe76c8e.
  Byte load, pattern 18: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `tst.b` at 0xffe76cba.
  Four-byte store, pattern 19: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe76cce.
  Four-byte store, pattern 20: target `sp -0x44`, raw base `a7`, value `[sp+4]+16`. Count 1 static access, including `pea.l` at 0xffe76cd0.
  Four-byte store, pattern 21: target `sp -0x48`, raw base `a7`, value `0xffe778be`. Count 1 static access, including `pea.l` at 0xffe76cd4.
  Four-byte store, pattern 22: target `sp -0x4c`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe76cda.
  Four-byte store, pattern 23: target `sp -0x40`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe76ce8.
  Four-byte load, pattern 24: target `sp -0x30`, raw base `a6`. Count 1 static access, including `tst.l` at 0xffe76cfc.
  Four-byte load, pattern 25: target `sp -0x30`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe76d02.
  Byte load, pattern 26: target `[sp-48] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe76d08.
  Four-byte store, pattern 27: target `sp -0x40`, raw base `a7`, value `0xffe778fc`. Count 1 static access, including `pea.l` at 0xffe76d22.
  Four-byte store, pattern 28: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe76d38.
  Four-byte load, pattern 29: target `sp -0x30`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe76d42.
  Four-byte store, pattern 30: target `sp -0x40`, raw base `a7`, value `[sp-48]`. Count 1 static access, including `move.l` at 0xffe76d42.
  Call (`jsr` at 0xffe76c90): (unknown callee), target 0xffe429dc. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`sp+8`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`0xffe7670c`, stack arg5=`0xffe76790`.
  Call (`jsr` at 0xffe76cde): (unknown callee), target 0xffe78288. Candidate slots: stack arg0=`sp-44`, stack arg1=`0xffe778be`, stack arg2=`[sp+4]+16`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe76cec): (unknown callee), target 0xffe587c0. Candidate slots: stack arg0=`sp-44`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe76d28): (unknown callee), target 0xffe587c0. Candidate slots: stack arg0=`0xffe778fc`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe76d3a): (unknown callee), target 0xffe587c0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe76d46): (unknown callee), target 0xffe76550. Candidate slots: stack arg0=`[sp-48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe76c54): target 0xffe76c60. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe76c5e): target 0xffe76c68. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.w` at 0xffe76c9e): target 0xffe76cfc. Taken when `ret(0xffe429dc) == 0x0`. Falls through when `ret(0xffe429dc) != 0x0`.
  Branch (`beq.b` at 0xffe76ca8): target 0xffe76cb0. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe76cac): target 0xffe76cb0. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe76cae): target 0xffe76cf6. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe76cbe): target 0xffe76cc8. Taken when `([[sp+8]+22]:1 & 0xff) != 0x0`. Falls through when `([[sp+8]+22]:1 & 0xff) == 0x0`.
  Jump (`bra.b` at 0xffe76cc6): target 0xffe76cce. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe76cf4): target 0xffe76cfc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe76d00): target 0xffe76d4c. Taken when `[sp-48] == 0x0`. Falls through when `[sp-48] != 0x0`.
  Branch (`beq.b` at 0xffe76d0e): target 0xffe76d1a. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`beq.b` at 0xffe76d12): target 0xffe76d22. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe76d16): target 0xffe76d32. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe76d18): target 0xffe76d42. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe76d20): target 0xffe76d38. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe76d30): target 0xffe76d42. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe76d56), d0=`unknown`.

- `0xffe71754`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#0`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe71754 to 0xffe71878 (exclusive), 292 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0x0, 974628 bytes, SHA256 `c091d32e310b7104130380d6c7f6c1e0a48f9cc4c79f13928860c1be37d52419`.
  Payload: carrier offset 0x40, 974564 bytes, SHA256 `a0a6cfec2692b7267ff56d52e46f1c75a39a623048dcd942835c816e4dde8ce0`.
  Read window: SHA256 `e6509054ba938a2d15dbb9524350a1b85a7548c07d861d28040417b880605ada`.
  Four-byte store, pattern 1: target `abs 0x3000272c`, raw base `a4`, value `0x0`. Count 2 static accesses, including `clr.l` at 0xffe71780, 0xffe717d4.
  Four-byte store, pattern 2: target `abs 0x30002730`, raw base `a4`, value `0x3`. Count 1 static access, including `move.l` at 0xffe71784.
  Four-byte load, pattern 3: target `abs 0x30002730`, raw base `a4`. Count 2 static accesses, including `move.l` at 0xffe7178e, 0xffe717e2.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe71794, 0xffe717e8.
  Four-byte store, pattern 5: target `abs 0x3000007c`, raw base `a5`, value `ret(0xffe61154)`. Count 2 static accesses, including `move.l` at 0xffe7179e, 0xffe717f2.
  Four-byte load, pattern 6: target `abs 0x3000007c`, raw base `a5`. Count 16 static accesses, including `movea.l` at 0xffe717a8, 0xffe717b0, 0xffe717b6.
  Byte store, pattern 7: target `[0x3000007c] +0x2`, raw base `a0`, value `0x5`. Count 1 static access, including `move.b` at 0xffe717aa.
  Byte store, pattern 8: target `[0x3000007c] +0x3`, raw base `a0`, value `0x1`. Count 1 static access, including `move.b` at 0xffe717b2.
  Byte store, pattern 9: target `[0x3000007c] +0x6`, raw base `a0`, value `0x5`. Count 1 static access, including `move.b` at 0xffe717b8.
  Byte store, pattern 10: target `[0x3000007c] +0x7`, raw base `a0`, value `0x1`. Count 1 static access, including `move.b` at 0xffe717be.
  Byte store, pattern 11: target `[0x3000007c] +0xa`, raw base `a0`, value `0x5`. Count 1 static access, including `move.b` at 0xffe717c4.
  Four-byte store, pattern 12: target `abs 0x30002730`, raw base `a4`, value `0x6`. Count 1 static access, including `move.l` at 0xffe717d8.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `a7`, value `0xffe71979`. Count 1 static access, including `pea.l` at 0xffe717f8.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe717fe.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71800.
  Four-byte store, pattern 16: target `sp -0x20`, raw base `a7`, value `0xffe7196e`. Count 1 static access, including `pea.l` at 0xffe71802.
  Four-byte store, pattern 17: target `sp -0x24`, raw base `a7`, value `0x3`. Count 1 static access, including `move.l` at 0xffe7180a.
  Byte store, pattern 18: target `[0x3000007c] +0x2`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe7181e.
  Byte store, pattern 19: target `[0x3000007c] +0x3`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe71824.
  Byte store, pattern 20: target `[0x3000007c] +0x6`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe7182a.
  Byte store, pattern 21: target `[0x3000007c] +0x7`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe71830.
  Byte store, pattern 22: target `[0x3000007c] +0xa`, raw base `a0`, value `0x7`. Count 1 static access, including `move.b` at 0xffe71838.
  Byte store, pattern 23: target `[0x3000007c] +0xb`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7183e.
  Byte store, pattern 24: target `[0x3000007c] +0xe`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe71844.
  Byte store, pattern 25: target `[0x3000007c] +0xf`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7184a.
  Byte store, pattern 26: target `[0x3000007c] +0x12`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe71850.
  Byte store, pattern 27: target `[0x3000007c] +0x13`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe71856.
  Byte store, pattern 28: target `[0x3000007c] +0x16`, raw base `a0`, value `0x7`. Count 1 static access, including `move.b` at 0xffe7185c.
  Four-byte load, pattern 29: target `abs 0x3000007c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe71862.
  Byte store, pattern 30: target `unknown`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7186a.
  Call (`jsr` at 0xffe71760): (unknown callee), target 0xffe46d28. Candidate slots: stack arg0=`d7`, stack arg1=`a4`, stack arg2=`a5`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe71796): (unknown callee), target 0xffe61154. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe717ea): (unknown callee), target 0xffe61154. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7180c): (unknown callee), target 0xffe5e128. Candidate slots: stack arg0=`0x3`, stack arg1=`0xffe7196e`, stack arg2=`unknown`, stack arg3=`0x0`, stack arg4=`0xffe71979`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe71770): target 0xffe7177a. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe71774): target 0xffe717ce. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe71776): target 0xffe7186c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe717a0): target 0xffe717a6. Taken when `ret(0xffe61154) != 0x0`. Falls through when `ret(0xffe61154) == 0x0`.
  Jump (`bra.b` at 0xffe717a4): target 0xffe717f8. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe717ca): target 0xffe71862. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe717f4): target 0xffe7181a. Taken when `ret(0xffe61154) != 0x0`. Falls through when `ret(0xffe61154) == 0x0`.
  Jump (`bra.b` at 0xffe71818): target 0xffe7186e. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe71876), d0=`unknown`.

- `0xffe7fa7c`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#1`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7fa7c to 0xffe7fb98 (exclusive), 284 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0xedf24, 1589988 bytes, SHA256 `5df6abeffec6c64b0fecbc68970e31a7dd41c2444993af801114d27a944d4799`.
  Payload: carrier offset 0xedf50, 1589944 bytes, SHA256 `64ad5503da721ec1dd85f37e372b6918629a1fd90ab9d8f990ef870fd86121e7`.
  Read window: SHA256 `77c3a03bf035d22227f3791f0f79ef5fb52d211483ef411ae7122ffcda619ffe`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7fa84.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 2 static accesses, including `movea.l` at 0xffe7fa88, 0xffe7faf6.
  Byte load, pattern 3: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe7fa8e.
  Four-byte load, pattern 4: target `abs 0x300000a4`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7fa96.
  Four-byte load, pattern 5: target `[0x300000a4] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe7fa9c.
  Four-byte load, pattern 6: target `abs 0x300000a0`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7faa0.
  Four-byte load, pattern 7: target `[0x300000a0] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe7faa6.
  Four-byte store, pattern 8: target `sp -0x34`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7faa8.
  Four-byte store, pattern 9: target `sp -0x30`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7faac.
  Four-byte store, pattern 10: target `sp -0x40`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe7fab2.
  Four-byte store, pattern 11: target `sp -0x44`, raw base `a7`, value `sp-52`. Count 1 static access, including `pea.l` at 0xffe7fab4.
  Four-byte store, pattern 12: target `sp -0x48`, raw base `a7`, value `0xffe7f5d0`. Count 1 static access, including `pea.l` at 0xffe7fab8.
  Four-byte store, pattern 13: target `sp -0x4c`, raw base `a7`, value `0xffe7f54c`. Count 1 static access, including `pea.l` at 0xffe7fabe.
  Four-byte store, pattern 14: target `sp -0x50`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7fac4.
  Four-byte store, pattern 15: target `sp -0x54`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe7fac8.
  Four-byte store, pattern 16: target `sp -0x58`, raw base `a7`, value `sp+8`. Count 1 static access, including `pea.l` at 0xffe7faca.
  Four-byte store, pattern 17: target `sp -0x5c`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe7face.
  Byte load, pattern 18: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `tst.b` at 0xffe7fafa.
  Four-byte store, pattern 19: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7fb0e.
  Four-byte store, pattern 20: target `sp -0x44`, raw base `a7`, value `[sp+4]+16`. Count 1 static access, including `pea.l` at 0xffe7fb10.
  Four-byte store, pattern 21: target `sp -0x48`, raw base `a7`, value `0xffe806fe`. Count 1 static access, including `pea.l` at 0xffe7fb14.
  Four-byte store, pattern 22: target `sp -0x4c`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe7fb1a.
  Four-byte store, pattern 23: target `sp -0x40`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe7fb28.
  Four-byte load, pattern 24: target `sp -0x30`, raw base `a6`. Count 1 static access, including `tst.l` at 0xffe7fb3c.
  Four-byte load, pattern 25: target `sp -0x30`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe7fb42.
  Byte load, pattern 26: target `[sp-48] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe7fb48.
  Four-byte store, pattern 27: target `sp -0x40`, raw base `a7`, value `0xffe8073c`. Count 1 static access, including `pea.l` at 0xffe7fb62.
  Four-byte store, pattern 28: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7fb78.
  Four-byte load, pattern 29: target `sp -0x30`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe7fb82.
  Four-byte store, pattern 30: target `sp -0x40`, raw base `a7`, value `[sp-48]`. Count 1 static access, including `move.l` at 0xffe7fb82.
  Call (`jsr` at 0xffe7fad0): (unknown callee), target 0xffe467ac. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`sp+8`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`0xffe7f54c`, stack arg5=`0xffe7f5d0`.
  Call (`jsr` at 0xffe7fb1e): (unknown callee), target 0xffe810e0. Candidate slots: stack arg0=`sp-44`, stack arg1=`0xffe806fe`, stack arg2=`[sp+4]+16`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7fb2c): (unknown callee), target 0xffe608f4. Candidate slots: stack arg0=`sp-44`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7fb68): (unknown callee), target 0xffe608f4. Candidate slots: stack arg0=`0xffe8073c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7fb7a): (unknown callee), target 0xffe608f4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7fb86): (unknown callee), target 0xffe7f390. Candidate slots: stack arg0=`[sp-48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe7fa94): target 0xffe7faa0. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe7fa9e): target 0xffe7faa8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.w` at 0xffe7fade): target 0xffe7fb3c. Taken when `ret(0xffe467ac) == 0x0`. Falls through when `ret(0xffe467ac) != 0x0`.
  Branch (`beq.b` at 0xffe7fae8): target 0xffe7faf0. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe7faec): target 0xffe7faf0. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe7faee): target 0xffe7fb36. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe7fafe): target 0xffe7fb08. Taken when `([[sp+8]+22]:1 & 0xff) != 0x0`. Falls through when `([[sp+8]+22]:1 & 0xff) == 0x0`.
  Jump (`bra.b` at 0xffe7fb06): target 0xffe7fb0e. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7fb34): target 0xffe7fb3c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe7fb40): target 0xffe7fb8c. Taken when `[sp-48] == 0x0`. Falls through when `[sp-48] != 0x0`.
  Branch (`beq.b` at 0xffe7fb4e): target 0xffe7fb5a. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`beq.b` at 0xffe7fb52): target 0xffe7fb62. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe7fb56): target 0xffe7fb72. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe7fb58): target 0xffe7fb82. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7fb60): target 0xffe7fb78. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe7fb70): target 0xffe7fb82. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe7fb96), d0=`unknown`.

- `0xffe7a594`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#1`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7a594 to 0xffe7a6b8 (exclusive), 292 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0xedf24, 1589988 bytes, SHA256 `5df6abeffec6c64b0fecbc68970e31a7dd41c2444993af801114d27a944d4799`.
  Payload: carrier offset 0xedf50, 1589944 bytes, SHA256 `64ad5503da721ec1dd85f37e372b6918629a1fd90ab9d8f990ef870fd86121e7`.
  Read window: SHA256 `bf2b7bd6a6ff43b85b72fcd97e1546f66c39495e131c75a27f71fe040887346f`.
  Four-byte store, pattern 1: target `abs 0x300027b0`, raw base `a4`, value `0x0`. Count 2 static accesses, including `clr.l` at 0xffe7a5c0, 0xffe7a614.
  Four-byte store, pattern 2: target `abs 0x300027b4`, raw base `a4`, value `0x3`. Count 1 static access, including `move.l` at 0xffe7a5c4.
  Four-byte load, pattern 3: target `abs 0x300027b4`, raw base `a4`. Count 2 static accesses, including `move.l` at 0xffe7a5ce, 0xffe7a622.
  Four-byte store, pattern 4: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses, including `move.l` at 0xffe7a5d4, 0xffe7a628.
  Four-byte store, pattern 5: target `abs 0x3000007c`, raw base `a5`, value `ret(0xffe69288)`. Count 2 static accesses, including `move.l` at 0xffe7a5de, 0xffe7a632.
  Four-byte load, pattern 6: target `abs 0x3000007c`, raw base `a5`. Count 16 static accesses, including `movea.l` at 0xffe7a5e8, 0xffe7a5f0, 0xffe7a5f6.
  Byte store, pattern 7: target `[0x3000007c] +0x2`, raw base `a0`, value `0x5`. Count 1 static access, including `move.b` at 0xffe7a5ea.
  Byte store, pattern 8: target `[0x3000007c] +0x3`, raw base `a0`, value `0x1`. Count 1 static access, including `move.b` at 0xffe7a5f2.
  Byte store, pattern 9: target `[0x3000007c] +0x6`, raw base `a0`, value `0x5`. Count 1 static access, including `move.b` at 0xffe7a5f8.
  Byte store, pattern 10: target `[0x3000007c] +0x7`, raw base `a0`, value `0x1`. Count 1 static access, including `move.b` at 0xffe7a5fe.
  Byte store, pattern 11: target `[0x3000007c] +0xa`, raw base `a0`, value `0x5`. Count 1 static access, including `move.b` at 0xffe7a604.
  Four-byte store, pattern 12: target `abs 0x300027b4`, raw base `a4`, value `0x6`. Count 1 static access, including `move.l` at 0xffe7a618.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `a7`, value `0xffe7a7b9`. Count 1 static access, including `pea.l` at 0xffe7a638.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7a63e.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe7a640.
  Four-byte store, pattern 16: target `sp -0x20`, raw base `a7`, value `0xffe7a7ae`. Count 1 static access, including `pea.l` at 0xffe7a642.
  Four-byte store, pattern 17: target `sp -0x24`, raw base `a7`, value `0x3`. Count 1 static access, including `move.l` at 0xffe7a64a.
  Byte store, pattern 18: target `[0x3000007c] +0x2`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe7a65e.
  Byte store, pattern 19: target `[0x3000007c] +0x3`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7a664.
  Byte store, pattern 20: target `[0x3000007c] +0x6`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe7a66a.
  Byte store, pattern 21: target `[0x3000007c] +0x7`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7a670.
  Byte store, pattern 22: target `[0x3000007c] +0xa`, raw base `a0`, value `0x7`. Count 1 static access, including `move.b` at 0xffe7a678.
  Byte store, pattern 23: target `[0x3000007c] +0xb`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7a67e.
  Byte store, pattern 24: target `[0x3000007c] +0xe`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe7a684.
  Byte store, pattern 25: target `[0x3000007c] +0xf`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7a68a.
  Byte store, pattern 26: target `[0x3000007c] +0x12`, raw base `a0`, value `0x8`. Count 1 static access, including `move.b` at 0xffe7a690.
  Byte store, pattern 27: target `[0x3000007c] +0x13`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7a696.
  Byte store, pattern 28: target `[0x3000007c] +0x16`, raw base `a0`, value `0x7`. Count 1 static access, including `move.b` at 0xffe7a69c.
  Four-byte load, pattern 29: target `abs 0x3000007c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7a6a2.
  Byte store, pattern 30: target `unknown`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe7a6aa.
  Call (`jsr` at 0xffe7a5a0): (unknown callee), target 0xffe4aaf8. Candidate slots: stack arg0=`d7`, stack arg1=`a4`, stack arg2=`a5`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7a5d6): (unknown callee), target 0xffe69288. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7a62a): (unknown callee), target 0xffe69288. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7a64c): (unknown callee), target 0xffe6625c. Candidate slots: stack arg0=`0x3`, stack arg1=`0xffe7a7ae`, stack arg2=`unknown`, stack arg3=`0x0`, stack arg4=`0xffe7a7b9`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe7a5b0): target 0xffe7a5ba. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe7a5b4): target 0xffe7a60e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.w` at 0xffe7a5b6): target 0xffe7a6ac. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe7a5e0): target 0xffe7a5e6. Taken when `ret(0xffe69288) != 0x0`. Falls through when `ret(0xffe69288) == 0x0`.
  Jump (`bra.b` at 0xffe7a5e4): target 0xffe7a638. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe7a60a): target 0xffe7a6a2. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe7a634): target 0xffe7a65a. Taken when `ret(0xffe69288) != 0x0`. Falls through when `ret(0xffe69288) == 0x0`.
  Jump (`bra.b` at 0xffe7a658): target 0xffe7a6ae. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe7a6b6), d0=`unknown`.
