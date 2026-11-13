# ColdFire L1 pointer paths in 1.4.1 and 1.6.0

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

The [full facts](disasm/coldfire-variants/pointer-paths-1.4.1-1.6.0-facts.txt) retain all instructions, constants and warnings.

- `0xffe6f0c0`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6f0c0 to 0xffe6f278 (exclusive), 440 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `0d92413fa34296c556550cf171ae32df1aba29d8ca37c6292e49049352344bce`.
  Four-byte load, pattern 1: target `sp +0x14`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe6f0cc.
  Four-byte store, pattern 2: target `sp -0x1c`, raw base `a7`, value `0xffffffff`. Count 1 static access, including `pea.l` at 0xffe6f0d0.
  Four-byte store, pattern 3: target `sp -0x20`, raw base `a7`, value `0x3000260c`. Count 1 static access, including `pea.l` at 0xffe6f0d4.
  Four-byte store, pattern 4: target `sp -0x1c`, raw base `a7`, value `ret(0xffe488d4)`. Count 1 static access, including `move.l` at 0xffe6f0e8.
  Four-byte store, pattern 5: target `sp -0x20`, raw base `a7`, value `0x1`. Count 3 static accesses, including `pea.l` at 0xffe6f0ea, 0xffe6f146, 0xffe6f190.
  Four-byte store, pattern 6: target `sp -0x24`, raw base `a7`, value `0xe6`. Count 1 static access, including `pea.l` at 0xffe6f0ee.
  Four-byte store, pattern 7: target `sp -0x28`, raw base `a7`, value `0xffe70cc0`. Count 3 static accesses, including `pea.l` at 0xffe6f0f2, 0xffe6f14e, 0xffe6f198.
  Four-byte load, pattern 8: target `abs 0x30000058`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe6f106.
  Four-byte load, pattern 9: target `unknown`, raw base `a4`. Count 1 static access, including `cmpa.l` at 0xffe6f110.
  Four-byte load, pattern 10: target `unknown`, raw base `a4`. Count 1 static access, including `movea.l` at 0xffe6f11c.
  Four-byte load, pattern 11: target `sp +0xc`, raw base `a6`. Count 2 static accesses, including `move.l` at 0xffe6f128, 0xffe6f1d6.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `a7`, value `0x3000260c`. Count 2 static accesses, including `pea.l` at 0xffe6f130, 0xffe6f17a.
  Four-byte store, pattern 13: target `sp -0x1c`, raw base `a7`, value `ret(0xffe4892c)`. Count 2 static accesses, including `move.l` at 0xffe6f144, 0xffe6f18e.
  Four-byte store, pattern 14: target `sp -0x24`, raw base `a7`, value `0xf6`. Count 1 static access, including `pea.l` at 0xffe6f14a.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `a7`, value `0x1c`. Count 2 static accesses, including `pea.l` at 0xffe6f168, 0xffe6f1b2.
  Four-byte store, pattern 16: target `sp -0x24`, raw base `a7`, value `0xfc`. Count 1 static access, including `pea.l` at 0xffe6f194.
  Four-byte store, pattern 17: target `sp -0x20`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe6f1b6.
  Four-byte store, pattern 18: target `sp -0x24`, raw base `a7`, value `ret(0xffe58d24)`. Count 1 static access, including `move.l` at 0xffe6f1b8.
  Four-byte load, pattern 19: target `sp +0x10`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe6f1c4.
  Two-byte store, pattern 20: target `ret(0xffe58d24) +0x8`, raw base `a4`, value `([sp+16] & 0xffff)`. Count 1 static access, including `move.w` at 0xffe6f1c8.
  Four-byte store, pattern 21: target `ret(0xffe58d24) +0x4`, raw base `a4`, value `[sp+20]`. Count 1 static access, including `move.l` at 0xffe6f1cc.
  Four-byte load, pattern 22: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe6f1d0.
  Four-byte load, pattern 23: target `[sp+4] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe6f1d4.
  Four-byte store, pattern 24: target `ret(0xffe58d24) +0x0`, raw base `a4`, value `[[sp+4]]`. Count 1 static access, including `move.l` at 0xffe6f1d4.
  Four-byte load, pattern 25: target `sp +0x8`, raw base `a6`. Count 2 static accesses, including `move.l` at 0xffe6f1de, 0xffe6f1ea.
  Four-byte load, pattern 26: target `unknown`, raw base `a4`. Count 1 static access, including `and.l` at 0xffe6f1e4.
  Four-byte store, pattern 27: target `unknown`, raw base `a4`, value `unknown`. Count 1 static access, including `and.l` at 0xffe6f1e4.
  Four-byte load, pattern 28: target `unknown`, raw base `a4`. Count 1 static access, including `or.l` at 0xffe6f1ee.
  Four-byte store, pattern 29: target `unknown`, raw base `a4`, value `unknown`. Count 1 static access, including `or.l` at 0xffe6f1ee.
  Four-byte load, pattern 30: target `unknown`, raw base `a4`. Count 2 static accesses, including `tst.l` at 0xffe6f1f6, 0xffe6f228.
  Four-byte load, pattern 31: target `abs 0x30000058`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe6f200.
  Four-byte load, pattern 32: target `[0x30000058] +0x18`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe6f206.
  Four-byte store, pattern 33: target `abs 0x30000058`, raw base `absolute`, value `[[0x30000058]+24]`. Count 1 static access, including `move.l` at 0xffe6f20a.
  Four-byte load, pattern 34: target `unknown`, raw base `a4`. Count 1 static access, including `move.l` at 0xffe6f212.
  Four-byte store, pattern 35: target `unknown`, raw base `a3`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6f212.
  Four-byte store, pattern 36: target `unknown`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6f218.
  Four-byte load, pattern 37: target `abs 0x30000058`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe6f22e.
  Four-byte store, pattern 38: target `unknown`, raw base `a4`, value `[0x30000058]`. Count 1 static access, including `move.l` at 0xffe6f234.
  Four-byte store, pattern 39: target `abs 0x30000058`, raw base `absolute`, value `unknown`. Count 1 static access, including `move.l` at 0xffe6f238.
  Four-byte store, pattern 40: target `unknown`, raw base `a7`, value `0x3000260c`. Count 1 static access, including `pea.l` at 0xffe6f23e.
  Four-byte store, pattern 41: target `unknown`, raw base `a7`, value `ret(0xffe4892c)`. Count 1 static access, including `move.l` at 0xffe6f252.
  Four-byte store, pattern 42: target `unknown`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe6f254.
  Four-byte store, pattern 43: target `unknown`, raw base `a7`, value `0x11f`. Count 1 static access, including `pea.l` at 0xffe6f258.
  Four-byte store, pattern 44: target `unknown`, raw base `a7`, value `0xffe70cc0`. Count 1 static access, including `pea.l` at 0xffe6f25c.
  Call (`jsr` at 0xffe6f0da): (unknown callee), target 0xffe488d4. Candidate slots: stack arg0=`0x3000260c`, stack arg1=`0xffffffff`, stack arg2=`d6`, stack arg3=`d7`, stack arg4=`a3`, stack arg5=`a4`.
  Call (`jsr` at 0xffe6f0f8): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`0xffe70cc0`, stack arg1=`0xe6`, stack arg2=`0x1`, stack arg3=`ret(0xffe488d4)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f136): (unknown callee), target 0xffe4892c. Candidate slots: stack arg0=`0x3000260c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f154): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`0xffe70cc0`, stack arg1=`0xf6`, stack arg2=`0x1`, stack arg3=`ret(0xffe4892c)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f16c): (unknown callee), target 0xffe58d24. Candidate slots: stack arg0=`0x1c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f180): (unknown callee), target 0xffe4892c. Candidate slots: stack arg0=`0x3000260c`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f19e): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`0xffe70cc0`, stack arg1=`0xfc`, stack arg2=`0x1`, stack arg3=`ret(0xffe4892c)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f1ba): (unknown callee), target 0xffe73bb4. Candidate slots: stack arg0=`ret(0xffe58d24)`, stack arg1=`0x0`, stack arg2=`0x1c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f21a): (unknown callee), target 0xffe58df0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f244): (unknown callee), target 0xffe4892c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f262): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6f0e6): target 0xffe6f102. Taken when `ret(0xffe488d4) == 0x0`. Falls through when `ret(0xffe488d4) != 0x0`.
  Branch (`beq.b` at 0xffe6f10e): target 0xffe6f122. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`bne.b` at 0xffe6f114): target 0xffe6f11a. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe6f118): target 0xffe6f122. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe6f120): target 0xffe6f10c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.w` at 0xffe6f124): target 0xffe6f1d6. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`bne.b` at 0xffe6f12e): target 0xffe6f168. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`beq.b` at 0xffe6f142): target 0xffe6f15e. Taken when `ret(0xffe4892c) == 0x0`. Falls through when `ret(0xffe4892c) != 0x0`.
  Jump (`bra.w` at 0xffe6f164): target 0xffe6f26e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe6f178): target 0xffe6f1b2. Taken when `ret(0xffe58d24) != 0x0`. Falls through when `ret(0xffe58d24) == 0x0`.
  Branch (`beq.b` at 0xffe6f18c): target 0xffe6f1a8. Taken when `ret(0xffe4892c) == 0x0`. Falls through when `ret(0xffe4892c) != 0x0`.
  Jump (`bra.w` at 0xffe6f1ae): target 0xffe6f26e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe6f1dc): target 0xffe6f1ea. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe6f1e8): target 0xffe6f1f2. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6f1f4): target 0xffe6f224. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`bne.b` at 0xffe6f1fa): target 0xffe6f224. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`bne.b` at 0xffe6f1fe): target 0xffe6f212. Taken when `? != 0x0`. Falls through when `? == 0x0`.
  Jump (`bra.b` at 0xffe6f210): target 0xffe6f218. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe6f222): target 0xffe6f23e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe6f226): target 0xffe6f23e. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`beq.b` at 0xffe6f22c): target 0xffe6f23e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe6f250): target 0xffe6f26c. Taken when `ret(0xffe4892c) == 0x0`. Falls through when `ret(0xffe4892c) != 0x0`.
  At the exit (`rts` at 0xffe6f276), d0=`unknown`.
  Effect warning (`not.l` at 0xffe6f1e2): unsupported effects invalidated.

- `0xffe57fa8`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe57fa8 to 0xffe581da (exclusive), 562 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `befb4feaa4288ff313d82f10e95bc7709e8d286333f19510a923d25a4a63ee12`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe57fb4.
  Four-byte store, pattern 2: target `sp -0x1c`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe57fb8.
  Four-byte load, pattern 3: target `abs 0x3000023c`, raw base `absolute`. Count 25 static accesses, including `movea.l` at 0xffe57fc6, 0xffe57fd0, 0xffe58002.
  Four-byte load, pattern 4: target `[0x3000023c] +0xe8`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe57fcc.
  Four-byte load, pattern 5: target `[0x3000023c] +0xe4`, raw base `a0`. Count 2 static accesses, including `sub.l` at 0xffe57fd6, 0xffe58028.
  Byte load, pattern 6: target `([sp+4] + ret(0xffe73d4c)) -0x1`, raw base `a5`. Count 1 static access, including `move.b` at 0xffe57fda.
  Four-byte store, pattern 7: target `unknown`, raw base `a7`, value `0xffffffff`. Count 1 static access, including `pea.l` at 0xffe57fe8.
  Four-byte store, pattern 8: target `unknown`, raw base `a7`, value `0x30002c38`. Count 2 static accesses, including `pea.l` at 0xffe57fec, 0xffe581a2.
  Four-byte load, pattern 9: target `[0x3000023c] +0xf0`, raw base `a0`. Count 6 static accesses, including `movea.l` at 0xffe58008, 0xffe5804a, 0xffe580de.
  Four-byte load, pattern 10: target `abs 0x3000023c`, raw base `absolute`. Count 10 static accesses, including `movea.l` at 0xffe5800c, 0xffe5804e, 0xffe580a2.
  Four-byte load, pattern 11: target `[0x3000023c] +0xec`, raw base `a1`. Count 2 static accesses, including `cmpa.l` at 0xffe58012, 0xffe58054.
  Four-byte load, pattern 12: target `[0x3000023c] +0xec`, raw base `a0`. Count 2 static accesses, including `move.l` at 0xffe5801e, 0xffe58060.
  Four-byte load, pattern 13: target `[0x3000023c] +0xe8`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe58032.
  Four-byte load, pattern 14: target `[0x3000023c] +0xf0`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe5803c.
  Four-byte load, pattern 15: target `[0x3000023c] +0xf0`, raw base `a0`. Count 1 static access, including `sub.l` at 0xffe5806a.
  Four-byte load, pattern 16: target `[0x3000023c] +0xec`, raw base `a0`. Count 2 static accesses, including `movea.l` at 0xffe5807e, 0xffe5809e.
  Byte load, pattern 17: target `[[0x3000023c]+236] +0x0`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe58082.
  Four-byte load, pattern 18: target `[0x3000023c] +0xec`, raw base `a0`. Count 2 static accesses, including `addq.l` at 0xffe58092, 0xffe580ca.
  Four-byte store, pattern 19: target `[0x3000023c] +0xec`, raw base `a0`, value `[[0x3000023c]+236]+1`. Count 2 static accesses, including `addq.l` at 0xffe58092, 0xffe580ca.
  Four-byte load, pattern 20: target `[0x3000023c] +0xe8`, raw base `a1`. Count 3 static accesses, including `cmpa.l` at 0xffe580a8, 0xffe580fe, 0xffe58148.
  Four-byte load, pattern 21: target `[0x3000023c] +0xe4`, raw base `a1`. Count 3 static accesses, including `move.l` at 0xffe580ba, 0xffe58110, 0xffe5815a.
  Four-byte store, pattern 22: target `[0x3000023c] +0xec`, raw base `a0`, value `[[0x3000023c]+228]`. Count 1 static access, including `move.l` at 0xffe580be.
  Byte load, pattern 23: target `unknown`, raw base `a5`. Count 1 static access, including `move.b` at 0xffe580e2.
  Byte store, pattern 24: target `[[0x3000023c]+240] +0x0`, raw base `a0`, value `unknown`. Count 1 static access, including `move.b` at 0xffe580e2.
  Four-byte load, pattern 25: target `[0x3000023c] +0xf0`, raw base `a0`. Count 2 static accesses, including `addq.l` at 0xffe580ea, 0xffe58134.
  Four-byte store, pattern 26: target `[0x3000023c] +0xf0`, raw base `a0`, value `[[0x3000023c]+240]+1`. Count 2 static accesses, including `addq.l` at 0xffe580ea, 0xffe58134.
  Four-byte store, pattern 27: target `[0x3000023c] +0xf0`, raw base `a0`, value `[[0x3000023c]+228]`. Count 2 static accesses, including `move.l` at 0xffe58114, 0xffe5815e.
  Byte store, pattern 28: target `[[0x3000023c]+240] +0x0`, raw base `a0`, value `0xa`. Count 1 static access, including `move.b` at 0xffe5812c.
  Four-byte store, pattern 29: target `unknown`, raw base `a7`, value `0x24`. Count 1 static access, including `pea.l` at 0xffe58162.
  Four-byte store, pattern 30: target `unknown`, raw base `a7`, value `[0x3000023c]+224`. Count 1 static access, including `pea.l` at 0xffe5816c.
  Byte store, pattern 31: target `[0x3000023c] +0x103`, raw base `a0`, value `(ret(0xffe58690) & 0xff)`. Count 1 static access, including `move.b` at 0xffe5817e.
  Four-byte load, pattern 32: target `unknown`, raw base `a0`. Count 3 static accesses, including `move.l` at 0xffe58198, 0xffe5819a, 0xffe581a0.
  Four-byte store, pattern 33: target `unknown`, raw base `a1`, value `unknown`. Count 3 static accesses, including `move.l` at 0xffe58198, 0xffe5819a, 0xffe581a0.
  Four-byte store, pattern 34: target `unknown`, raw base `a7`, value `ret(0xffe4892c)`. Count 1 static access, including `move.l` at 0xffe581b6.
  Four-byte store, pattern 35: target `unknown`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe581b8.
  Four-byte store, pattern 36: target `unknown`, raw base `a7`, value `0x42e`. Count 1 static access, including `pea.l` at 0xffe581bc.
  Four-byte store, pattern 37: target `unknown`, raw base `a7`, value `0xffe58458`. Count 1 static access, including `pea.l` at 0xffe581c0.
  Call (`jsr` at 0xffe57fba): (unknown callee), target 0xffe73d4c. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`d4`, stack arg2=`d5`, stack arg3=`d6`, stack arg4=`d7`, stack arg5=`a5`.
  Call (`jsr` at 0xffe57ff2): (unknown callee), target 0xffe488d4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe58170): (unknown callee), target 0xffe58690. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe581a8): (unknown callee), target 0xffe4892c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe581c6): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe57fe4): target 0xffe57fe8. Taken when `0xa == ?`. Falls through when `0xa != ?`.
  Branch (`bne.w` at 0xffe57ffe): target 0xffe581d0. Taken when `ret(0xffe488d4) != 0x0`. Falls through when `ret(0xffe488d4) == 0x0`.
  Branch (`bls.b` at 0xffe58016): target 0xffe58044. Taken when `[[0x3000023c]+240] <= [[0x3000023c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000023c]+240] > [[0x3000023c]+236] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe58042): target 0xffe5806e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe58058): target 0xffe5806e. Taken when `[[0x3000023c]+240] >= [[0x3000023c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000023c]+240] < [[0x3000023c]+236] (unsigned, 32-bit)`.
  Branch (`bgt.b` at 0xffe58076): target 0xffe580d2. Taken when `? > ? (signed, 32-bit)`. Falls through when `? <= ? (signed, 32-bit)`.
  Branch (`beq.b` at 0xffe5808a): target 0xffe580c4. Taken when `0xa == ?`. Falls through when `0xa != ?`.
  Branch (`bcs.b` at 0xffe580ac): target 0xffe58078. Taken when `[[0x3000023c]+236] < [[0x3000023c]+232] (unsigned, 32-bit)`. Falls through when `[[0x3000023c]+236] >= [[0x3000023c]+232] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe580c2): target 0xffe58078. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe580d0): target 0xffe5806e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe580d4): target 0xffe5811a. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bcs.b` at 0xffe58102): target 0xffe580d2. Taken when `[[0x3000023c]+240] < [[0x3000023c]+232] (unsigned, 32-bit)`. Falls through when `[[0x3000023c]+240] >= [[0x3000023c]+232] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe58118): target 0xffe580d2. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5811e): target 0xffe58162. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bcs.b` at 0xffe5814c): target 0xffe58162. Taken when `[[0x3000023c]+240] < [[0x3000023c]+232] (unsigned, 32-bit)`. Falls through when `[[0x3000023c]+240] >= [[0x3000023c]+232] (unsigned, 32-bit)`.
  Branch (`bgt.b` at 0xffe5819e): target 0xffe58198. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe581b4): target 0xffe581d0. Taken when `ret(0xffe4892c) == 0x0`. Falls through when `ret(0xffe4892c) != 0x0`.
  At the exit (`rts` at 0xffe581d8), d0=`unknown`.
  Effect warning (`extb.l` at 0xffe57fde): unsupported effects invalidated.
  Effect warning (`extb.l` at 0xffe58070): unsupported effects invalidated.
  Effect warning (`extb.l` at 0xffe58084): unsupported effects invalidated.

- `0xffe51e24`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe51e24 to 0xffe51e4e (exclusive), 42 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `eca0c95911b2e10cd1853a3e3b7742a142beb41f3a6c380ba700e97d493de0f8`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 2 static accesses, including `movea.l` at 0xffe51e28, 0xffe51e32.
  Four-byte load, pattern 2: target `abs 0x300000e4`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe51e2c.
  Four-byte store, pattern 3: target `[sp+4] +0x0`, raw base `a0`, value `[0x300000e4]`. Count 1 static access, including `move.l` at 0xffe51e2c.
  Four-byte load, pattern 4: target `[sp+4] +0x0`, raw base `a0`. Count 1 static access, including `tst.l` at 0xffe51e36.
  Four-byte load, pattern 5: target `abs 0x300000e4`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe51e3e.
  Byte load, pattern 6: target `[0x300000e4] +0x5`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe51e46.
  Branch (`bne.b` at 0xffe51e38): target 0xffe51e3e. Taken when `[[sp+4]] != 0x0`. Falls through when `[[sp+4]] == 0x0`.
  Jump (`bra.b` at 0xffe51e3c): target 0xffe51e4a. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe51e4c), d0=`unknown`.

- `0xffe676dc`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe676dc to 0xffe67730 (exclusive), 84 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `c5442f0899b31bc7878fdb28e967b468f5b622fdd5077b7650571e66c28823c1`.
  Four-byte load, pattern 1: target `abs 0x300002ac`, raw base `absolute`. Count 3 static accesses, including `movea.l` at 0xffe676ee, 0xffe676f8, 0xffe67704.
  Byte store, pattern 2: target `[0x300002ac] +0x1ec`, raw base `a0`, value `0x0`. Count 1 static access, including `clr.b` at 0xffe676f4.
  Byte store, pattern 3: target `[0x300002ac] +0x1e8`, raw base `a0`, value `0xb0`. Count 1 static access, including `move.b` at 0xffe67700.
  Byte load, pattern 4: target `sp +0x7`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe6770a.
  Byte store, pattern 5: target `[0x300002ac] +0x1f0`, raw base `a0`, value `unknown`. Count 1 static access, including `move.b` at 0xffe6770e.
  Four-byte store, pattern 6: target `sp -0xc`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe67712.
  Call (`bsr.b` at 0xffe676e2): (unknown callee), target 0xffe676a4. Candidate slots: stack arg0=`d7`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe67716): (unknown callee), target 0xffe675ac. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe676e8): target 0xffe676ee. Taken when `ret(0xffe676a4) == 0x0`. Falls through when `ret(0xffe676a4) != 0x0`.
  Jump (`bra.b` at 0xffe676ec): target 0xffe67728. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe67720): target 0xffe67726. Taken when `ret(0xffe675ac) == 0x0`. Falls through when `ret(0xffe675ac) != 0x0`.
  Jump (`bra.b` at 0xffe67724): target 0xffe67728. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6772e), d0=`unknown`.

- `0xffe50d18`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe50d18 to 0xffe50d82 (exclusive), 106 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `a22177b2f249ff0b23fe15f8ff1704b1b93814464cb87bf6794948e2fa65b4c8`.
  Four-byte load, pattern 1: target `abs 0x300000c4`, raw base `absolute`. Count 1 static access, including `tst.l` at 0xffe50d1e.
  Byte load, pattern 2: target `abs 0x300000c0`, raw base `absolute`. Count 1 static access, including `tst.b` at 0xffe50d2c.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `a7`, value `0x20`. Count 1 static access, including `pea.l` at 0xffe50d34.
  Four-byte load, pattern 4: target `abs 0x300000c4`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe50d38.
  Byte load, pattern 5: target `[0x300000c4] +0x0`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe50d40.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe50d42.
  Four-byte store, pattern 7: target `sp -0xc`, raw base `a7`, value `0x2c`. Count 1 static access, including `pea.l` at 0xffe50d62.
  Call (`jsr` at 0xffe50d44): (unknown callee), target 0xffe63ac0. Candidate slots: stack arg0=`unknown`, stack arg1=`0x20`, stack arg2=`d7`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe50d66): (unknown callee), target 0xffe58ee8. Candidate slots: stack arg0=`0x2c`, stack arg1=`d7`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe50d24): target 0xffe50d2c. Taken when `[0x300000c4] != 0x0`. Falls through when `[0x300000c4] == 0x0`.
  Jump (`bra.w` at 0xffe50d28): target 0xffe50d7a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe50d32): target 0xffe50d62. Taken when `([0x300000c0]:1 & 0xff) == 0x0`. Falls through when `([0x300000c0]:1 & 0xff) != 0x0`.
  Branch (`beq.b` at 0xffe50d4e): target 0xffe50d56. Taken when `ret(0xffe63ac0) == 0x0`. Falls through when `ret(0xffe63ac0) != 0x0`.
  Branch (`beq.b` at 0xffe50d52): target 0xffe50d5a. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe50d54): target 0xffe50d5e. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe50d58): target 0xffe50d7a. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe50d5c): target 0xffe50d7a. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe50d60): target 0xffe50d7a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe50d70): target 0xffe50d76. Taken when `ret(0xffe58ee8) == 0x0`. Falls through when `ret(0xffe58ee8) != 0x0`.
  Jump (`bra.b` at 0xffe50d74): target 0xffe50d78. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe50d80), d0=`unknown`.

- `0xffe594d4`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe594d4 to 0xffe59544 (exclusive), 112 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `df9464646ff1b68c056e32e8cd434c601840179d92b34390b20ab8bc97d23414`.
  Four-byte load, pattern 1: target `abs 0x300001e8`, raw base `absolute`. Count 4 static accesses, including `movea.l` at 0xffe594d8, 0xffe594ec, 0xffe59516.
  Four-byte load, pattern 2: target `abs 0x300001e8`, raw base `absolute`. Count 4 static accesses, including `movea.l` at 0xffe594de, 0xffe594f2, 0xffe59504.
  Four-byte load, pattern 3: target `[0x300001e8] +0xe4`, raw base `a1`. Count 2 static accesses, including `move.l` at 0xffe594e4, 0xffe594f8.
  Four-byte store, pattern 4: target `[0x300001e8] +0xec`, raw base `a0`, value `[[0x300001e8]+228]`. Count 1 static access, including `move.l` at 0xffe594e8.
  Four-byte store, pattern 5: target `[0x300001e8] +0xf0`, raw base `a0`, value `[[0x300001e8]+228]`. Count 1 static access, including `move.l` at 0xffe594fc.
  Four-byte store, pattern 6: target `sp -0x8`, raw base `a7`, value `0x24`. Count 1 static access, including `pea.l` at 0xffe59500.
  Four-byte store, pattern 7: target `sp -0xc`, raw base `a7`, value `[0x300001e8]+224`. Count 1 static access, including `pea.l` at 0xffe5950a.
  Byte store, pattern 8: target `[0x300001e8] +0x103`, raw base `a0`, value `(ret(0xffe5e8d8) & 0xff)`. Count 1 static access, including `move.b` at 0xffe5951c.
  Four-byte load, pattern 9: target `unknown`, raw base `a0`. Count 3 static accesses, including `move.l` at 0xffe59536, 0xffe59538, 0xffe5953e.
  Four-byte store, pattern 10: target `unknown`, raw base `a1`, value `unknown`. Count 3 static accesses, including `move.l` at 0xffe59536, 0xffe59538, 0xffe5953e.
  Call (`jsr` at 0xffe5950e): (unknown callee), target 0xffe5e8d8. Candidate slots: stack arg0=`[0x300001e8]+224`, stack arg1=`0x24`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bgt.b` at 0xffe5953c): target 0xffe59536. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffe59542), d0=`unknown`.

- `0xffe7354c`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7354c to 0xffe735bc (exclusive), 112 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `9fbd6e2ca8f45e7dc23ee13721c825cb3b900b17c002da10dec1d8778b67dc9e`.
  Four-byte load, pattern 1: target `abs 0x30000024`, raw base `absolute`. Count 1 static access, including `adda.l` at 0xffe7355e.
  Four-byte load, pattern 2: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe73564.
  Byte store, pattern 3: target `[0x30000024] +0x30001868`, raw base `a0`, value `([sp+4] & 0xff)`. Count 1 static access, including `move.b` at 0xffe73568.
  Four-byte load, pattern 4: target `abs 0x30000024`, raw base `absolute`. Count 1 static access, including `addq.l` at 0xffe7356a.
  Four-byte store, pattern 5: target `abs 0x30000024`, raw base `absolute`, value `[0x30000024]+1`. Count 1 static access, including `addq.l` at 0xffe7356a.
  Four-byte load, pattern 6: target `sp +0x8`, raw base `a6`. Count 1 static access, including `tst.l` at 0xffe73570.
  Four-byte load, pattern 7: target `abs 0x30000024`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe73576.
  Four-byte load, pattern 8: target `abs 0x30000024`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe73584.
  Four-byte store, pattern 9: target `abs 0x30000024`, raw base `absolute`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7358a.
  Four-byte load, pattern 10: target `abs 0x30000030`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe73590.
  Four-byte store, pattern 11: target `sp -0xc`, raw base `a7`, value `[0x30000024]`. Count 1 static access, including `move.l` at 0xffe73596.
  Four-byte store, pattern 12: target `sp -0x10`, raw base `a7`, value `0x30001868`. Count 1 static access, including `pea.l` at 0xffe73598.
  Four-byte load, pattern 13: target `abs 0x30000020`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe7359e.
  Four-byte store, pattern 14: target `sp -0x14`, raw base `a7`, value `[0x30000020]`. Count 1 static access, including `move.l` at 0xffe7359e.
  Call (`jsr` at 0xffe73552): (unknown callee), target 0xffe446a0. Candidate slots: stack arg0=`d7`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe735a4): (unknown callee), target unknown. Candidate slots: stack arg0=`[0x30000020]`, stack arg1=`0x30001868`, stack arg2=`[0x30000024]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe73574): target 0xffe73584. Taken when `[sp+8] != 0x0`. Falls through when `[sp+8] == 0x0`.
  Branch (`bcs.b` at 0xffe73582): target 0xffe735b2. Taken when `[0x30000024] < 0xc4 (unsigned, 32-bit)`. Falls through when `[0x30000024] >= 0xc4 (unsigned, 32-bit)`.
  Branch (`beq.b` at 0xffe735ac): target 0xffe735b2. Taken when `[0x30000024] == ret(unknown)`. Falls through when `[0x30000024] != ret(unknown)`.
  Jump (`bra.b` at 0xffe735b0): target 0xffe735b4. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe735ba), d0=`unknown`.

- `0xffe79880`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe79880 to 0xffe79910 (exclusive), 144 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `cc1612c3b747b72e2014526d607c39d54ddd4b0f0cf1954a9a1febe213c7fd02`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe7988c.
  Four-byte load, pattern 2: target `abs 0x30000f80`, raw base `absolute`. Count 1 static access, including `move.l` at 0xffe79890.
  Four-byte load, pattern 3: target `abs 0x300002ac`, raw base `absolute`. Count 2 static accesses, including `movea.l` at 0xffe79898, 0xffe798a4.
  Byte store, pattern 4: target `[0x300002ac] +0x43`, raw base `a0`, value `0x55`. Count 1 static access, including `move.b` at 0xffe798a0.
  Byte store, pattern 5: target `[0x300002ac] +0x43`, raw base `a0`, value `0xaa`. Count 1 static access, including `move.b` at 0xffe798ac.
  Two-byte load, pattern 6: target `[0x30000f80] +0x0`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe798b2.
  Two-byte load, pattern 7: target `[0x30000f80] +0x0`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe798b6.
  Four-byte store, pattern 8: target `sp -0x1c`, raw base `a7`, value `0x3`. Count 1 static access, including `pea.l` at 0xffe798d8.
  Four-byte store, pattern 9: target `sp -0x20`, raw base `a7`, value `0xffe6bf38`. Count 1 static access, including `pea.l` at 0xffe798dc.
  Call (`bsr.w` at 0xffe798e2): (unknown callee), target 0xffe79840. Candidate slots: stack arg0=`0xffe6bf38`, stack arg1=`0x3`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe798ce): target 0xffe798d4. Taken when `? != ?`. Falls through when `? == ?`.
  Jump (`bra.b` at 0xffe798d2): target 0xffe79906. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe798d6): target 0xffe798ec. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe798ea): target 0xffe79906. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe798fa): target 0xffe798fe. Taken when `0x20 != ?`. Falls through when `0x20 == ?`.
  Branch (`bne.w` at 0xffe79900): target 0xffe79898. Taken when `[sp+4] != 0x0`. Falls through when `[sp+4] == 0x0`.
  At the exit (`rts` at 0xffe7990e), d0=`unknown`.

- `0xffe6c594`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6c594 to 0xffe6c63e (exclusive), 170 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `26e643affc93f8793e0e683aa740732593f4622472956260b5c6181c123529c1`.
  Four-byte load, pattern 1: target `abs 0x30000040`, raw base `absolute`. Count 1 static access, including `tst.l` at 0xffe6c598.
  Four-byte store, pattern 2: target `sp -0x8`, raw base `a7`, value `0xffe6d1c4`. Count 1 static access, including `pea.l` at 0xffe6c5a0.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `a7`, value `0xffe6cf30`. Count 1 static access, including `pea.l` at 0xffe6c5ae.
  Four-byte store, pattern 4: target `sp -0xc`, raw base `a7`, value `0xffe6cffc`. Count 1 static access, including `pea.l` at 0xffe6c5b4.
  Four-byte load, pattern 5: target `abs 0x300002ac`, raw base `absolute`. Count 7 static accesses, including `movea.l` at 0xffe6c5c2, 0xffe6c5d6, 0xffe6c5e2.
  Two-byte load, pattern 6: target `[0x300002ac] +0xca`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe6c5c8.
  Two-byte store, pattern 7: target `[0x300002ac] +0xca`, raw base `a0`, value `unknown`. Count 1 static access, including `move.w` at 0xffe6c5d2.
  Byte store, pattern 8: target `[0x300002ac] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access, including `move.b` at 0xffe6c5de.
  Byte store, pattern 9: target `[0x300002ac] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access, including `move.b` at 0xffe6c5ea.
  Byte store, pattern 10: target `[0x300002ac] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses, including `move.b` at 0xffe6c5f6, 0xffe6c60e.
  Byte store, pattern 11: target `[0x300002ac] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access, including `move.b` at 0xffe6c602.
  Byte store, pattern 12: target `[0x300002ac] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access, including `move.b` at 0xffe6c61a.
  Four-byte load, pattern 13: target `abs 0x300002ac`, raw base `absolute`. Count 1 static access, including `add.l` at 0xffe6c624.
  Four-byte store, pattern 14: target `abs 0x30000048`, raw base `absolute`, value `[0x300002ac]+457`. Count 1 static access, including `move.l` at 0xffe6c62a.
  Four-byte store, pattern 15: target `abs 0x30000044`, raw base `absolute`, value `0x1`. Count 1 static access, including `move.l` at 0xffe6c632.
  Call (`jsr` at 0xffe6c5a6): (unknown callee), target 0xffe73438. Candidate slots: stack arg0=`0xffe6d1c4`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6c5ba): (unknown callee), target 0xffe6d33c. Candidate slots: stack arg0=`0xffe6cffc`, stack arg1=`0xffe6cf30`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6c59e): target 0xffe6c5ae. Taken when `[0x30000040] == 0x0`. Falls through when `[0x30000040] != 0x0`.
  At the exit (`rts` at 0xffe6c63c), d0=`0x1`.

- `0xffe6731c`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6731c to 0xffe673c8 (exclusive), 172 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `a90068723444309d7e3f103b203af1fd7e8793090a5419225e39c029a403ba42`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe67324.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `a7`, value `unknown`. Count 3 static accesses, including `move.l` at 0xffe6732e, 0xffe67342, 0xffe67384.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `0xffe67a18`. Count 1 static access, including `pea.l` at 0xffe67344.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `0x3000496c`. Count 2 static accesses, including `pea.l` at 0xffe6734a, 0xffe6738c.
  Four-byte load, pattern 5: target `abs 0x300002ac`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe67360.
  Byte load, pattern 6: target `sp +0xb`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe67366.
  Byte store, pattern 7: target `[0x300002ac] +0x1f0`, raw base `a0`, value `unknown`. Count 1 static access, including `move.b` at 0xffe6736a.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe6736e.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `a7`, value `0xffe67a50`. Count 1 static access, including `pea.l` at 0xffe67386.
  Two-byte load, pattern 10: target `sp +0x12`, raw base `a6`. Count 1 static access, including `move.w` at 0xffe673a6.
  Four-byte store, pattern 11: target `sp -0x10`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe673aa.
  Four-byte load, pattern 12: target `sp +0xc`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe673ac.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `a7`, value `[sp+12]`. Count 1 static access, including `move.l` at 0xffe673ac.
  Call (`bsr.w` at 0xffe67330): (unknown callee), target 0xffe676dc. Candidate slots: stack arg0=`unknown`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe67350): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67a18`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe67372): (unknown callee), target 0xffe675ac. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe67392): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67a50`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6739c): (unknown callee), target 0xffe67730. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe673b0): (unknown callee), target 0xffe677fc. Candidate slots: stack arg0=`[sp+12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe673b8): (unknown callee), target 0xffe67730. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6733a): target 0xffe67360. Taken when `ret(0xffe676dc) == 0x0`. Falls through when `ret(0xffe676dc) != 0x0`.
  Jump (`bra.w` at 0xffe6735c): target 0xffe673be. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6737c): target 0xffe673a4. Taken when `ret(0xffe675ac) == 0x0`. Falls through when `ret(0xffe675ac) != 0x0`.
  Jump (`bra.b` at 0xffe673a2): target 0xffe673be. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe673c6), d0=`unknown`.

- `0xffe67744`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe67744 to 0xffe677fa (exclusive), 182 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `5392b86d1f39f241f0f139204b8bee342b0a8fbdc2c63b99c9741ab6caed0467`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe67750.
  Two-byte load, pattern 2: target `sp +0xa`, raw base `a6`. Count 1 static access, including `move.w` at 0xffe67754.
  Four-byte load, pattern 3: target `abs 0x300002ac`, raw base `absolute`. Count 4 static accesses, including `movea.l` at 0xffe67758, 0xffe67764, 0xffe677aa.
  Byte store, pattern 4: target `[0x300002ac] +0x1e8`, raw base `a0`, value `0xa0`. Count 1 static access, including `move.b` at 0xffe67760.
  Byte load, pattern 5: target `[0x300002ac] +0x1f0`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe6776a.
  Byte store, pattern 6: target `sp -0x5`, raw base `a6`, value `unknown`. Count 1 static access, including `move.b` at 0xffe6776e.
  Four-byte store, pattern 7: target `sp -0x1c`, raw base `a7`, value `0x0`. Count 2 static accesses, including `clr.l` at 0xffe67772, 0xffe677c0.
  Four-byte store, pattern 8: target `sp -0x1c`, raw base `a7`, value `0xffe67bfc`. Count 1 static access, including `pea.l` at 0xffe67780.
  Four-byte store, pattern 9: target `sp -0x20`, raw base `a7`, value `0x3000496c`. Count 2 static accesses, including `pea.l` at 0xffe67786, 0xffe677d4.
  Byte store, pattern 10: target `[0x300002ac] +0x1e8`, raw base `a0`, value `0xa8`. Count 1 static access, including `move.b` at 0xffe677b2.
  Byte load, pattern 11: target `[0x300002ac] +0x1f0`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe677bc.
  Byte store, pattern 12: target `unknown`, raw base `a5`, value `([[0x300002ac]+496]:1 & 0xff)`. Count 1 static access, including `move.b` at 0xffe677bc.
  Four-byte store, pattern 13: target `sp -0x1c`, raw base `a7`, value `0xffe67c2c`. Count 1 static access, including `pea.l` at 0xffe677ce.
  Call (`bsr.w` at 0xffe67774): (unknown callee), target 0xffe675ac. Candidate slots: stack arg0=`0x0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6778c): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67bfc`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe677c2): (unknown callee), target 0xffe675ac. Candidate slots: stack arg0=`0x0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe677da): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67c2c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6777e): target 0xffe67798. Taken when `ret(0xffe675ac) == 0x0`. Falls through when `ret(0xffe675ac) != 0x0`.
  Jump (`bra.b` at 0xffe67796): target 0xffe677f0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe677a0): target 0xffe677ee. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bne.b` at 0xffe677a8): target 0xffe677b6. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`beq.b` at 0xffe677cc): target 0xffe677e6. Taken when `ret(0xffe675ac) == 0x0`. Falls through when `ret(0xffe675ac) != 0x0`.
  Jump (`bra.b` at 0xffe677e4): target 0xffe677f0. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe677ec): target 0xffe67798. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe677f8), d0=`unknown`.

- `0xffe55ffc`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe55ffc to 0xffe560ca (exclusive), 206 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `3d0ef7329fa6b5ed7db19a71e901e6f503aae1890098bdf76570b6d2eefb6bd2`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe56008.
  Four-byte load, pattern 2: target `abs 0x30000138`, raw base `absolute`. Count 1 static access, including `cmp.l` at 0xffe5600c.
  Four-byte load, pattern 3: target `abs 0x30000120`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe56014.
  Four-byte load, pattern 4: target `[0x30000120] +0x28`, raw base `a5`. Count 2 static accesses, including `move.l` at 0xffe5601a, 0xffe56028.
  Four-byte load, pattern 5: target `abs 0x30000128`, raw base `absolute`. Count 1 static access, including `cmp.l` at 0xffe56020.
  Four-byte store, pattern 6: target `abs 0x30000128`, raw base `absolute`, value `[[0x30000120]+40]+1`. Count 1 static access, including `move.l` at 0xffe5602e.
  Four-byte load, pattern 7: target `abs 0x30000124`, raw base `absolute`. Count 1 static access, including `cmp.l` at 0xffe56036.
  Four-byte load, pattern 8: target `abs 0x3000011c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe5603e.
  Four-byte load, pattern 9: target `[0x3000011c] +0x28`, raw base `a5`. Count 2 static accesses, including `move.l` at 0xffe56044, 0xffe56052.
  Four-byte load, pattern 10: target `abs 0x3000012c`, raw base `absolute`. Count 1 static access, including `cmp.l` at 0xffe5604a.
  Four-byte store, pattern 11: target `abs 0x3000012c`, raw base `absolute`, value `[[0x3000011c]+40]+1`. Count 1 static access, including `move.l` at 0xffe56058.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `a7`, value `0x64`. Count 1 static access, including `pea.l` at 0xffe56064.
  Four-byte store, pattern 13: target `sp -0x20`, raw base `a7`, value `sp-8`. Count 1 static access, including `pea.l` at 0xffe56068.
  Four-byte store, pattern 14: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe5606c.
  Four-byte load, pattern 15: target `abs 0x30000134`, raw base `absolute`. Count 1 static access, including `tst.l` at 0xffe5607e.
  Four-byte store, pattern 16: target `abs 0x30000134`, raw base `absolute`, value `0x1`. Count 1 static access, including `move.l` at 0xffe56088.
  Four-byte store, pattern 17: target `sp -0x1c`, raw base `a7`, value `ret(0xffe45ba8)`. Count 1 static access, including `move.l` at 0xffe5608e.
  Four-byte store, pattern 18: target `sp -0x20`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe56090.
  Four-byte store, pattern 19: target `sp -0x24`, raw base `a7`, value `0xffe563c8`. Count 1 static access, including `pea.l` at 0xffe56092.
  Four-byte store, pattern 20: target `sp -0x28`, raw base `a7`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe56098.
  Four-byte store, pattern 21: target `sp -0x2c`, raw base `a7`, value `0x83`. Count 1 static access, including `pea.l` at 0xffe5609a.
  Four-byte store, pattern 22: target `sp -0x30`, raw base `a7`, value `0xffe563ac`. Count 1 static access, including `pea.l` at 0xffe5609e.
  Four-byte store, pattern 23: target `sp -0x34`, raw base `a7`, value `0x1`. Count 1 static access, including `pea.l` at 0xffe560a4.
  Four-byte store, pattern 24: target `abs 0x30000134`, raw base `absolute`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe560b2.
  Four-byte load, pattern 25: target `sp -0x8`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe560bc.
  Call (`jsr` at 0xffe5606e): (unknown callee), target 0xffe45ba8. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`0x64`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe560a8): (unknown callee), target 0xffe565d8. Candidate slots: stack arg0=`0x1`, stack arg1=`0xffe563ac`, stack arg2=`0x83`, stack arg3=`0x0`, stack arg4=`0xffe563c8`, stack arg5=`[sp+4]`.
  Branch (`bhi.b` at 0xffe56012): target 0xffe56036. Taken when `[sp+4] > [0x30000138] (unsigned, 32-bit)`. Falls through when `[sp+4] <= [0x30000138] (unsigned, 32-bit)`.
  Branch (`bls.b` at 0xffe56026): target 0xffe56064. Taken when `[[0x30000120]+40]+1 <= [0x30000128] (unsigned, 32-bit)`. Falls through when `[[0x30000120]+40]+1 > [0x30000128] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe56034): target 0xffe56064. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bhi.b` at 0xffe5603c): target 0xffe56060. Taken when `[sp+4] > [0x30000124] (unsigned, 32-bit)`. Falls through when `[sp+4] <= [0x30000124] (unsigned, 32-bit)`.
  Branch (`bls.b` at 0xffe56050): target 0xffe56064. Taken when `[[0x3000011c]+40]+1 <= [0x3000012c] (unsigned, 32-bit)`. Falls through when `[[0x3000011c]+40]+1 > [0x3000012c] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe5605e): target 0xffe56064. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe56062): target 0xffe560c0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5607c): target 0xffe560bc. Taken when `ret(0xffe45ba8) == 0x0`. Falls through when `ret(0xffe45ba8) != 0x0`.
  Branch (`bne.b` at 0xffe56084): target 0xffe560b8. Taken when `[0x30000134] != 0x0`. Falls through when `[0x30000134] == 0x0`.
  Jump (`bra.b` at 0xffe560ba): target 0xffe560c0. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe560c8), d0=`unknown`.

- `0xffe6742c`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6742c to 0xffe6752e (exclusive), 258 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `914a2893ba37dd93f0eec388369e2fbdac3d4768f9d2caad7cc7853a6703acd1`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe67434.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `a7`, value `unknown`. Count 4 static accesses, including `move.l` at 0xffe6743e, 0xffe67452, 0xffe67494.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `0xffe67abc`. Count 1 static access, including `pea.l` at 0xffe67454.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `0x3000496c`. Count 3 static accesses, including `pea.l` at 0xffe6745a, 0xffe6749c, 0xffe674f2.
  Four-byte load, pattern 5: target `abs 0x300002ac`, raw base `absolute`. Count 3 static accesses, including `movea.l` at 0xffe67470, 0xffe674b6, 0xffe674c2.
  Byte load, pattern 6: target `sp +0xb`, raw base `a6`. Count 1 static access, including `move.b` at 0xffe67476.
  Byte store, pattern 7: target `[0x300002ac] +0x1f0`, raw base `a0`, value `unknown`. Count 2 static accesses, including `move.b` at 0xffe6747a, 0xffe674d0.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `a7`, value `0x1`. Count 2 static accesses, including `pea.l` at 0xffe6747e, 0xffe674d4.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `a7`, value `0xffe67af0`. Count 1 static access, including `pea.l` at 0xffe67496.
  Byte store, pattern 10: target `[0x300002ac] +0x1e8`, raw base `a0`, value `0xb4`. Count 1 static access, including `move.b` at 0xffe674be.
  Four-byte store, pattern 11: target `sp -0x14`, raw base `a7`, value `0xffe67b2c`. Count 1 static access, including `pea.l` at 0xffe674ec.
  Two-byte load, pattern 12: target `sp +0x12`, raw base `a6`. Count 1 static access, including `move.w` at 0xffe6750c.
  Four-byte store, pattern 13: target `sp -0x10`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe67510.
  Four-byte load, pattern 14: target `sp +0xc`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe67512.
  Four-byte store, pattern 15: target `sp -0x14`, raw base `a7`, value `[sp+12]`. Count 1 static access, including `move.l` at 0xffe67512.
  Call (`bsr.w` at 0xffe67440): (unknown callee), target 0xffe676dc. Candidate slots: stack arg0=`unknown`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe67460): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67abc`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe67482): (unknown callee), target 0xffe675ac. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe674a2): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67af0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe674ac): (unknown callee), target 0xffe67730. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe674d8): (unknown callee), target 0xffe675ac. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe674f8): (unknown callee), target 0xffe73ae0. Candidate slots: stack arg0=`0x3000496c`, stack arg1=`0xffe67b2c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe67502): (unknown callee), target 0xffe67730. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe67516): (unknown callee), target 0xffe67744. Candidate slots: stack arg0=`[sp+12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6751e): (unknown callee), target 0xffe67730. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6744a): target 0xffe67470. Taken when `ret(0xffe676dc) == 0x0`. Falls through when `ret(0xffe676dc) != 0x0`.
  Jump (`bra.w` at 0xffe6746c): target 0xffe67524. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6748c): target 0xffe674b6. Taken when `ret(0xffe675ac) == 0x0`. Falls through when `ret(0xffe675ac) != 0x0`.
  Jump (`bra.w` at 0xffe674b2): target 0xffe67524. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe674e2): target 0xffe6750a. Taken when `ret(0xffe675ac) == 0x0`. Falls through when `ret(0xffe675ac) != 0x0`.
  Jump (`bra.b` at 0xffe67508): target 0xffe67524. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6752c), d0=`unknown`.
