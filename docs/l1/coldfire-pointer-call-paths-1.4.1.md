# ColdFire L1 pointer and call observations

Each entry applies to its own shipped carrier, package and extracted payload.
The analysis windows are inferred. Whole-function extent, uniqueness and runtime banks remain unknown.
Roles, types and physical pointee meanings remain unknown.
Path and reachability statements use the local CFG with ordinary call fallthrough.
Calls retain all six candidate stack locations. Actual arity and custom ABI remain unknown.
Only native resolved targets are named. Unknown addresses retain a raw base without a displacement.
Memory patterns group identical encoded instructions and identical native access fields within one entry.
Distinct pattern numbers keep different encoded effects separate, even when the displayed fields agree.
Counts describe static access records. They do not establish dynamic counts or equal runtime pointees.
Proved frame linkage and paired entry saves and terminal restores are omitted from the summaries.
Meaningful incoming, local and outgoing stack state is retained. Ambiguous records stay selected.
The exclusions classify encoded frame structure, without proving a runtime calling convention.
Known literals and opaque expressions remain as printed by the tool. Unknown values remain unknown.
A missing modeled exit establishes no runtime never-return claim.
Context views admit only instructions ending within the requested bounds.
Ten crossing instructions are excluded. Their requested tails remain raw and open.
Raw tails establish no caller arguments or control flow. Pre-entry slices have unproved boundaries.
Entry evidence uses reconstructed caller windows. Local reachability does not prove runtime execution.

The [full facts](disasm/coldfire-variants/pointer-call-paths-1.4.1-facts.txt) retain every instruction and warning.

- `0xffe4db20`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4db20 to 0xffe4db5a (exclusive), 58 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `c3d0b0388acbfee57d6a8f036ae07c1a710d493607509fa412aa23870c182c81`.
  Four-byte load, pattern 1: target `abs 0x30000094`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe4db24.
  Four-byte load, pattern 2: target `abs 0x30000094`, raw base `absolute`. Count 2 static accesses. Sites: `movea.l` at 0xffe4db34, `movea.l` at 0xffe4db42.
  Byte load, pattern 3: target `[0x30000094] +0x1`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4db3c.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe4db40.
  Byte load, pattern 5: target `[0x30000094] +0x0`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4db4a.
  Four-byte store, pattern 6: target `sp -0xc`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe4db4c.
  Call (`jsr` at 0xffe4db4e): (unknown callee), target 0xffe6198c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe4db2a): target 0xffe4db34. Taken when `[0x30000094] != 0x0`. Falls through when `[0x30000094] == 0x0`.
  Jump (`bra.b` at 0xffe4db32): target 0xffe4db56. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe4db58), d0=`unknown`.
  Register fact (`move.l` at 0xffe4db2c): d0=`0xfffffe0c`.
  Register fact (`moveq` at 0xffe4db3a): d0=`0x0`.
  Register fact (`moveq` at 0xffe4db48): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe6ab30`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6ab30 to 0xffe6ab9c (exclusive), 108 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `2bca1ea97963f238b6bcb92189b00d978f0534f492ae2ddf0f46705ccd0da11d`.
  Four-byte load, pattern 1: target `abs 0x30000044`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe6ab36.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `a7`, value `0x30002238`. Count 1 static access. Sites: `pea.l` at 0xffe6ab3e.
  Four-byte load, pattern 3: target `abs 0x30000198`, raw base `absolute`. Count 5 static accesses. Sites: `movea.l` at 0xffe6ab52, `movea.l` at 0xffe6ab64, `movea.l` at 0xffe6ab70.
  Byte load, pattern 4: target `[0x30000198] +0x1c9`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe6ab5a.
  Byte store, pattern 5: target `[0x30000198] +0x1c9`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.b` at 0xffe6ab6a.
  Byte store, pattern 6: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access. Sites: `move.b` at 0xffe6ab76.
  Byte store, pattern 7: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x38`. Count 1 static access. Sites: `move.b` at 0xffe6ab82.
  Byte store, pattern 8: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access. Sites: `move.b` at 0xffe6ab8e.
  Call (`bsr.w` at 0xffe6ab44): (unknown callee), target 0xffe6a310. Candidate slots: stack arg0=`0x30002238`, stack arg1=`d7`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe6ab3c): target 0xffe6ab52. Taken when `[0x30000044] != 0x0`. Falls through when `[0x30000044] == 0x0`.
  Branch (`bne.b` at 0xffe6ab4c): target 0xffe6ab52. Taken when `ret(0xffe6a310) != 0x0`. Falls through when `ret(0xffe6a310) == 0x0`.
  Jump (`bra.b` at 0xffe6ab50): target 0xffe6ab94. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6ab9a), d0=`unknown`.
  Register fact (`moveq` at 0xffe6ab4e): d0=`0x0`.
  Register fact (`moveq` at 0xffe6ab58): d7=`0x0`.
  Register fact (`moveq` at 0xffe6ab6e): d7=`0x28`.
  Register fact (`moveq` at 0xffe6ab7a): d7=`0x38`.
  Register fact (`moveq` at 0xffe6ab86): d7=`0x30`.
  Register fact (`moveq` at 0xffe6ab92): d0=`0x1`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe71148`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe71148 to 0xffe711c4 (exclusive), 124 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `732433b545593789f555a4ef80ddb6908c1e22b4110d074384484061ac616666`.
  Four-byte load, pattern 1: target `abs 0x30000024`, raw base `absolute`. Count 1 static access. Sites: `adda.l` at 0xffe7115a.
  Four-byte load, pattern 2: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe71160.
  Byte store, pattern 3: target `[0x30000024] +0x30001854`, raw base `a0`, value `([sp+4] & 0xff)`. Count 1 static access. Sites: `move.b` at 0xffe71164.
  Four-byte load, pattern 4: target `abs 0x30000024`, raw base `absolute`. Count 1 static access. Sites: `addq.l` at 0xffe71166.
  Four-byte store, pattern 5: target `abs 0x30000024`, raw base `absolute`, value `[0x30000024]+1`. Count 1 static access. Sites: `addq.l` at 0xffe71166.
  Four-byte load, pattern 6: target `sp +0x8`, raw base `a6`. Count 1 static access. Sites: `tst.l` at 0xffe7116c.
  Four-byte load, pattern 7: target `abs 0x30000024`, raw base `absolute`. Count 1 static access. Sites: `move.l` at 0xffe71172.
  Four-byte load, pattern 8: target `abs 0x30000024`, raw base `absolute`. Count 1 static access. Sites: `move.l` at 0xffe71180.
  Four-byte store, pattern 9: target `abs 0x30000024`, raw base `absolute`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe71186.
  Four-byte load, pattern 10: target `abs 0x30000020`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe7118c.
  Four-byte load, pattern 11: target `abs 0x30000030`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe71198.
  Four-byte store, pattern 12: target `sp -0xc`, raw base `a7`, value `[0x30000024]`. Count 1 static access. Sites: `move.l` at 0xffe7119e.
  Four-byte store, pattern 13: target `sp -0x10`, raw base `a7`, value `0x30001854`. Count 1 static access. Sites: `pea.l` at 0xffe711a0.
  Four-byte load, pattern 14: target `abs 0x30000020`, raw base `absolute`. Count 1 static access. Sites: `move.l` at 0xffe711a6.
  Four-byte store, pattern 15: target `sp -0x14`, raw base `a7`, value `[0x30000020]`. Count 1 static access. Sites: `move.l` at 0xffe711a6.
  Call (`jsr` at 0xffe7114e): (unknown callee), target 0xffe463c0. Candidate slots: stack arg0=`d7`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe711ac): (unknown callee), target unknown. Candidate slots: stack arg0=`[0x30000020]`, stack arg1=`0x30001854`, stack arg2=`[0x30000024]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe71170): target 0xffe71180. Taken when `[sp+8] != 0x0`. Falls through when `[sp+8] == 0x0`.
  Branch (`bcs.b` at 0xffe7117e): target 0xffe711ba. Taken when `[0x30000024] < 0xc4 (unsigned, 32-bit)`. Falls through when `[0x30000024] >= 0xc4 (unsigned, 32-bit)`.
  Branch (`bne.b` at 0xffe71192): target 0xffe71198. Taken when `[0x30000020] != 0x0`. Falls through when `[0x30000020] == 0x0`.
  Jump (`bra.b` at 0xffe71196): target 0xffe711bc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe711b4): target 0xffe711ba. Taken when `[0x30000024] == ret(unknown)`. Falls through when `[0x30000024] != ret(unknown)`.
  Jump (`bra.b` at 0xffe711b8): target 0xffe711bc. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe711c2), d0=`unknown`.
  Register fact (`movea.l` at 0xffe71154): a0=`0x30001854`.
  Register fact (`moveq` at 0xffe71194): d0=`0xffffffff`.
  Register fact (`moveq` at 0xffe711b6): d0=`0xffffffff`.
  Register fact (`moveq` at 0xffe711ba): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe4dc9c`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4dc9c to 0xffe4dd1c (exclusive), 128 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `b7acfb5b8fb0c7c10ed60190db4c6b35c029e743f99d616a02d84613e76cea00`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe4dca2.
  Four-byte load, pattern 2: target `abs 0x30000094`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe4dca6.
  Four-byte load, pattern 3: target `abs 0x30000094`, raw base `absolute`. Count 4 static accesses. Sites: `movea.l` at 0xffe4dcb8, `movea.l` at 0xffe4dcea, `movea.l` at 0xffe4dcf6.
  Byte load, pattern 4: target `[0x30000094] +0x2`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4dcc0.
  Byte load, pattern 5: target `[0x30000094] +0x6`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4dcf0.
  Byte load, pattern 6: target `[0x30000094] +0x7`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4dcfc.
  Byte load, pattern 7: target `[0x30000094] +0x8`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4dd08.
  Four-byte store, pattern 8: target `sp -0xc`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe4dd0c.
  Call (`bsr.w` at 0xffe4dd0e): (unknown callee), target 0xffe4dbf0. Candidate slots: stack arg0=`unknown`, stack arg1=`d7`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe4dcac): target 0xffe4dcb8. Taken when `[0x30000094] != 0x0`. Falls through when `[0x30000094] == 0x0`.
  Jump (`bra.w` at 0xffe4dcb4): target 0xffe4dd14. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe4dcca): target 0xffe4dcd4. Taken when `? != 0xff`. Falls through when `? == 0xff`.
  Jump (`bra.b` at 0xffe4dcd2): target 0xffe4dd14. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe4dcde): target 0xffe4dcea. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`beq.b` at 0xffe4dce2): target 0xffe4dcf6. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe4dce6): target 0xffe4dd02. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe4dce8): target 0xffe4dd0c. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe4dcf4): target 0xffe4dd0c. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe4dd00): target 0xffe4dd0c. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe4dd1a), d0=`unknown`.
  Register fact (`move.l` at 0xffe4dcae): d0=`0xfffffe0c`.
  Register fact (`moveq` at 0xffe4dcbe): d0=`0x0`.
  Register fact (`move.l` at 0xffe4dccc): d0=`0xfffffe0b`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe6abb4`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6abb4 to 0xffe6ac40 (exclusive), 140 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `2dfc9245017e8067a61a8305bd33136e77f36f4a457ed586cff974a81601d901`.
  Four-byte load, pattern 1: target `abs 0x30000198`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe6abc0.
  Byte load, pattern 2: target `[0x30000198] +0x1c9`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe6abc8.
  Four-byte store, pattern 3: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe6ac16.
  Four-byte store, pattern 4: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe6ac18.
  Four-byte store, pattern 5: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe6ac1a.
  Four-byte store, pattern 6: target `sp -0x30`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe6ac1c.
  Four-byte store, pattern 7: target `sp -0x34`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe6ac1e.
  Four-byte store, pattern 8: target `sp -0x38`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe6ac20.
  Four-byte store, pattern 9: target `sp -0x3c`, raw base `a7`, value `0xffe6afc8`. Count 1 static access. Sites: `pea.l` at 0xffe6ac22.
  Four-byte load, pattern 10: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe6ac28.
  Four-byte store, pattern 11: target `sp -0x40`, raw base `a7`, value `[sp+4]`. Count 1 static access. Sites: `move.l` at 0xffe6ac28.
  Call (`jsr` at 0xffe6ac2c): (unknown callee), target 0xffe716d8. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe6afc8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6abd0): target 0xffe6abd6. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe6abd4): target 0xffe6abd8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6abdc): target 0xffe6abe2. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe6abe0): target 0xffe6abe4. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6abe8): target 0xffe6abee. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe6abec): target 0xffe6abf0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6abf4): target 0xffe6abfa. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe6abf8): target 0xffe6abfc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe6abfe): target 0xffe6ac04. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe6ac02): target 0xffe6ac06. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6ac0a): target 0xffe6ac12. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe6ac10): target 0xffe6ac16. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe6ac3e), d0=`ret(0xffe716d8)`.
  Register fact (`moveq` at 0xffe6abc6): d7=`0x0`.
  Register fact (`moveq` at 0xffe6abd2): d6=`0x31`.
  Register fact (`moveq` at 0xffe6abd6): d6=`0x30`.
  Register fact (`moveq` at 0xffe6abde): d5=`0x31`.
  Register fact (`moveq` at 0xffe6abe2): d5=`0x30`.
  Register fact (`moveq` at 0xffe6abea): d4=`0x31`.
  Register fact (`moveq` at 0xffe6abee): d4=`0x30`.
  Register fact (`moveq` at 0xffe6abf6): d3=`0x31`.
  Register fact (`moveq` at 0xffe6abfa): d3=`0x30`.
  Register fact (`moveq` at 0xffe6ac00): d2=`0x31`.
  Register fact (`moveq` at 0xffe6ac04): d2=`0x30`.
  Register fact (`movea.w` at 0xffe6ac0c): a5=`0x31`.
  Register fact (`movea.w` at 0xffe6ac12): a5=`0x30`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe5f39c`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5f39c to 0xffe5f440 (exclusive), 164 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `757929a6b0cc069fa42c6b5e339bbc145b4cea1c541cc0b7fcc814e48f7b499f`.
  Four-byte load, pattern 1: target `abs 0x3000057c`, raw base `absolute`. Count 3 static accesses. Sites: `movea.l` at 0xffe5f3aa, `movea.l` at 0xffe5f3dc, `movea.l` at 0xffe5f418.
  Byte load, pattern 2: target `[0x3000057c] +0x80`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe5f3b0.
  Four-byte load, pattern 3: target `abs 0x30000580`, raw base `absolute`. Count 1 static access. Sites: `cmp.l` at 0xffe5f3b6.
  Four-byte load, pattern 4: target `unknown`, raw base `a0`. Count 1 static access. Sites: `tst.l` at 0xffe5f3d8.
  Byte load, pattern 5: target `unknown`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe5f3e8.
  Four-byte store, pattern 6: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe5f400.
  Four-byte store, pattern 7: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe5f40e.
  Byte store, pattern 8: target `unknown`, raw base `a0`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe5f424.
  Call (`bsr.b` at 0xffe5f410): (unknown callee), target 0xffe5f470. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bge.w` at 0xffe5f3bc): target 0xffe5f436. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`beq.w` at 0xffe5f3c2): target 0xffe5f436. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`beq.b` at 0xffe5f3ca): target 0xffe5f428. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Branch (`beq.b` at 0xffe5f3da): target 0xffe5f418. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe5f3f8): target 0xffe5f428. Taken when `? == 0xc1`. Falls through when `? != 0xc1`.
  Jump (`bra.b` at 0xffe5f416): target 0xffe5f428. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe5f432): target 0xffe5f3b6. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe5f43e), d0=`unknown`.
  Register fact (`moveq` at 0xffe5f3a8): d5=`0x0`.
  Register fact (`moveq` at 0xffe5f3b4): d4=`0x0`.
  Register fact (`movea.l` at 0xffe5f3cc): a0=`0x300031d0`.
  Register fact (`move.l` at 0xffe5f402): d0=`0x300031d0`.
  Register fact (`moveq` at 0xffe5f428): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe6a310`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6a310 to 0xffe6a3ba (exclusive), 170 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `99d5937db1a02de84e5528ebed1ef4d9e0282692846997145ce91c437758f9b0`.
  Four-byte load, pattern 1: target `abs 0x30000040`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe6a314.
  Four-byte store, pattern 2: target `sp -0x8`, raw base `a7`, value `0xffe6aed4`. Count 1 static access. Sites: `pea.l` at 0xffe6a31c.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `a7`, value `0xffe6ac40`. Count 1 static access. Sites: `pea.l` at 0xffe6a32a.
  Four-byte store, pattern 4: target `sp -0xc`, raw base `a7`, value `0xffe6ad0c`. Count 1 static access. Sites: `pea.l` at 0xffe6a330.
  Four-byte load, pattern 5: target `abs 0x30000198`, raw base `absolute`. Count 7 static accesses. Sites: `movea.l` at 0xffe6a33e, `movea.l` at 0xffe6a352, `movea.l` at 0xffe6a35e.
  Two-byte load, pattern 6: target `[0x30000198] +0xca`, raw base `a0`. Count 1 static access. Sites: `move.w` at 0xffe6a344.
  Two-byte store, pattern 7: target `[0x30000198] +0xca`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.w` at 0xffe6a34e.
  Byte store, pattern 8: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access. Sites: `move.b` at 0xffe6a35a.
  Byte store, pattern 9: target `[0x30000198] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access. Sites: `move.b` at 0xffe6a366.
  Byte store, pattern 10: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses. Sites: `move.b` at 0xffe6a372, `move.b` at 0xffe6a38a.
  Byte store, pattern 11: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access. Sites: `move.b` at 0xffe6a37e.
  Byte store, pattern 12: target `[0x30000198] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access. Sites: `move.b` at 0xffe6a396.
  Four-byte load, pattern 13: target `abs 0x30000198`, raw base `absolute`. Count 1 static access. Sites: `add.l` at 0xffe6a3a0.
  Four-byte store, pattern 14: target `abs 0x30000048`, raw base `absolute`, value `[0x30000198]+457`. Count 1 static access. Sites: `move.l` at 0xffe6a3a6.
  Four-byte store, pattern 15: target `abs 0x30000044`, raw base `absolute`, value `0x1`. Count 1 static access. Sites: `move.l` at 0xffe6a3ae.
  Call (`jsr` at 0xffe6a322): (unknown callee), target 0xffe71034. Candidate slots: stack arg0=`0xffe6aed4`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6a336): (unknown callee), target 0xffe6b04c. Candidate slots: stack arg0=`0xffe6ad0c`, stack arg1=`0xffe6ac40`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6a31a): target 0xffe6a32a. Taken when `[0x30000040] == 0x0`. Falls through when `[0x30000040] != 0x0`.
  At the exit (`rts` at 0xffe6a3b8), d0=`0x1`.
  Register fact (`moveq` at 0xffe6a358): d0=`0x31`.
  Register fact (`moveq` at 0xffe6a364): d0=`0xfffffffe`.
  Register fact (`moveq` at 0xffe6a370): d0=`0x38`.
  Register fact (`moveq` at 0xffe6a37c): d0=`0x28`.
  Register fact (`moveq` at 0xffe6a388): d0=`0x38`.
  Register fact (`moveq` at 0xffe6a394): d0=`0x30`.
  Register fact (`move.l` at 0xffe6a39a): d0=`0x1c9`.
  Register fact (`moveq` at 0xffe6a3ac): d0=`0x1`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe65098`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe65098 to 0xffe65144 (exclusive), 172 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `a1d99c31d1c7d86cee5eef149f6ad13475f6b85c533e035cec98d5d06df34d5d`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe650a0.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `a7`, value `unknown`. Count 3 static accesses. Sites: `move.l` at 0xffe650aa, `move.l` at 0xffe650be, `move.l` at 0xffe65100.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `0xffe65794`. Count 1 static access. Sites: `pea.l` at 0xffe650c0.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `0x30004660`. Count 2 static accesses. Sites: `pea.l` at 0xffe650c6, `pea.l` at 0xffe65108.
  Four-byte load, pattern 5: target `abs 0x30000198`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe650dc.
  Byte load, pattern 6: target `sp +0xb`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe650e2.
  Byte store, pattern 7: target `[0x30000198] +0x1f0`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.b` at 0xffe650e6.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `a7`, value `0x1`. Count 1 static access. Sites: `pea.l` at 0xffe650ea.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `a7`, value `0xffe657cc`. Count 1 static access. Sites: `pea.l` at 0xffe65102.
  Two-byte load, pattern 10: target `sp +0x12`, raw base `a6`. Count 1 static access. Sites: `move.w` at 0xffe65122.
  Four-byte store, pattern 11: target `sp -0x10`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe65126.
  Four-byte load, pattern 12: target `sp +0xc`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe65128.
  Four-byte store, pattern 13: target `sp -0x14`, raw base `a7`, value `[sp+12]`. Count 1 static access. Sites: `move.l` at 0xffe65128.
  Call (`bsr.w` at 0xffe650ac): (unknown callee), target 0xffe65458. Candidate slots: stack arg0=`unknown`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe650cc): (unknown callee), target 0xffe716d8. Candidate slots: stack arg0=`0x30004660`, stack arg1=`0xffe65794`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe650ee): (unknown callee), target 0xffe65328. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6510e): (unknown callee), target 0xffe716d8. Candidate slots: stack arg0=`0x30004660`, stack arg1=`0xffe657cc`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe65118): (unknown callee), target 0xffe654ac. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe6512c): (unknown callee), target 0xffe65578. Candidate slots: stack arg0=`[sp+12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe65134): (unknown callee), target 0xffe654ac. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe650b6): target 0xffe650dc. Taken when `ret(0xffe65458) == 0x0`. Falls through when `ret(0xffe65458) != 0x0`.
  Jump (`bra.w` at 0xffe650d8): target 0xffe6513a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe650f8): target 0xffe65120. Taken when `ret(0xffe65328) == 0x0`. Falls through when `ret(0xffe65328) != 0x0`.
  Jump (`bra.b` at 0xffe6511e): target 0xffe6513a. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe65142), d0=`unknown`.
  Register fact (`moveq` at 0xffe65120): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe5aeac`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5aeac to 0xffe5af7a (exclusive), 206 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `34155b39c9c750760d1db9afe940a0f2fb0a66d7c08280e11659d267cc42f55f`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe5aeb8.
  Four-byte load, pattern 2: target `abs 0x300002c8`, raw base `absolute`. Count 1 static access. Sites: `cmp.l` at 0xffe5aebc.
  Four-byte load, pattern 3: target `abs 0x300002b0`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe5aec4.
  Four-byte load, pattern 4: target `[0x300002b0] +0x28`, raw base `a5`. Count 2 static accesses. Sites: `move.l` at 0xffe5aeca, `move.l` at 0xffe5aed8.
  Four-byte load, pattern 5: target `abs 0x300002b8`, raw base `absolute`. Count 1 static access. Sites: `cmp.l` at 0xffe5aed0.
  Four-byte store, pattern 6: target `abs 0x300002b8`, raw base `absolute`, value `[[0x300002b0]+40]+1`. Count 1 static access. Sites: `move.l` at 0xffe5aede.
  Four-byte load, pattern 7: target `abs 0x300002b4`, raw base `absolute`. Count 1 static access. Sites: `cmp.l` at 0xffe5aee6.
  Four-byte load, pattern 8: target `abs 0x300002ac`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe5aeee.
  Four-byte load, pattern 9: target `[0x300002ac] +0x28`, raw base `a5`. Count 2 static accesses. Sites: `move.l` at 0xffe5aef4, `move.l` at 0xffe5af02.
  Four-byte load, pattern 10: target `abs 0x300002bc`, raw base `absolute`. Count 1 static access. Sites: `cmp.l` at 0xffe5aefa.
  Four-byte store, pattern 11: target `abs 0x300002bc`, raw base `absolute`, value `[[0x300002ac]+40]+1`. Count 1 static access. Sites: `move.l` at 0xffe5af08.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `a7`, value `0x64`. Count 1 static access. Sites: `pea.l` at 0xffe5af14.
  Four-byte store, pattern 13: target `sp -0x20`, raw base `a7`, value `sp-8`. Count 1 static access. Sites: `pea.l` at 0xffe5af18.
  Four-byte store, pattern 14: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe5af1c.
  Four-byte load, pattern 15: target `abs 0x300002c4`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe5af2e.
  Four-byte store, pattern 16: target `abs 0x300002c4`, raw base `absolute`, value `0x1`. Count 1 static access. Sites: `move.l` at 0xffe5af38.
  Four-byte store, pattern 17: target `sp -0x1c`, raw base `a7`, value `ret(0xffe478c8)`. Count 1 static access. Sites: `move.l` at 0xffe5af3e.
  Four-byte store, pattern 18: target `sp -0x20`, raw base `a7`, value `[sp+4]`. Count 1 static access. Sites: `move.l` at 0xffe5af40.
  Four-byte store, pattern 19: target `sp -0x24`, raw base `a7`, value `0xffe5b278`. Count 1 static access. Sites: `pea.l` at 0xffe5af42.
  Four-byte store, pattern 20: target `sp -0x28`, raw base `a7`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe5af48.
  Four-byte store, pattern 21: target `sp -0x2c`, raw base `a7`, value `0x83`. Count 1 static access. Sites: `pea.l` at 0xffe5af4a.
  Four-byte store, pattern 22: target `sp -0x30`, raw base `a7`, value `0xffe5b25c`. Count 1 static access. Sites: `pea.l` at 0xffe5af4e.
  Four-byte store, pattern 23: target `sp -0x34`, raw base `a7`, value `0x1`. Count 1 static access. Sites: `pea.l` at 0xffe5af54.
  Four-byte store, pattern 24: target `abs 0x300002c4`, raw base `absolute`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe5af62.
  Four-byte load, pattern 25: target `sp -0x8`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe5af6c.
  Call (`jsr` at 0xffe5af1e): (unknown callee), target 0xffe478c8. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`0x64`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5af58): (unknown callee), target 0xffe55c04. Candidate slots: stack arg0=`0x1`, stack arg1=`0xffe5b25c`, stack arg2=`0x83`, stack arg3=`0x0`, stack arg4=`0xffe5b278`, stack arg5=`[sp+4]`.
  Branch (`bhi.b` at 0xffe5aec2): target 0xffe5aee6. Taken when `[sp+4] > [0x300002c8] (unsigned, 32-bit)`. Falls through when `[sp+4] <= [0x300002c8] (unsigned, 32-bit)`.
  Branch (`bls.b` at 0xffe5aed6): target 0xffe5af14. Taken when `[[0x300002b0]+40]+1 <= [0x300002b8] (unsigned, 32-bit)`. Falls through when `[[0x300002b0]+40]+1 > [0x300002b8] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe5aee4): target 0xffe5af14. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bhi.b` at 0xffe5aeec): target 0xffe5af10. Taken when `[sp+4] > [0x300002b4] (unsigned, 32-bit)`. Falls through when `[sp+4] <= [0x300002b4] (unsigned, 32-bit)`.
  Branch (`bls.b` at 0xffe5af00): target 0xffe5af14. Taken when `[[0x300002ac]+40]+1 <= [0x300002bc] (unsigned, 32-bit)`. Falls through when `[[0x300002ac]+40]+1 > [0x300002bc] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe5af0e): target 0xffe5af14. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe5af12): target 0xffe5af70. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5af2c): target 0xffe5af6c. Taken when `ret(0xffe478c8) == 0x0`. Falls through when `ret(0xffe478c8) != 0x0`.
  Branch (`bne.b` at 0xffe5af34): target 0xffe5af68. Taken when `[0x300002c4] != 0x0`. Falls through when `[0x300002c4] == 0x0`.
  Jump (`bra.b` at 0xffe5af6a): target 0xffe5af70. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe5af78), d0=`unknown`.
  Register fact (`moveq` at 0xffe5af10): d0=`0x0`.
  Register fact (`moveq` at 0xffe5af36): d0=`0x1`.
  Register fact (`moveq` at 0xffe5af68): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe6993c`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6993c to 0xffe69a98 (exclusive), 348 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `61c217d9fe9696b91cfb423e52e6273328215ba4ceb2a2946e6ec0ccf17b4ecd`.
  Four-byte load, pattern 1: target `sp +0xc`, raw base `a6`. Count 1 static access. Sites: `movea.l` at 0xffe69948.
  Four-byte load, pattern 2: target `abs 0x30000f70`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe6994e.
  Four-byte store, pattern 3: target `sp -0x34`, raw base `a7`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe69956.
  Four-byte store, pattern 4: target `sp -0x34`, raw base `a7`, value `0x700`. Count 1 static access. Sites: `pea.l` at 0xffe6996a.
  Four-byte store, pattern 5: target `sp -0x8`, raw base `a6`, value `ret(0xffe4629c)`. Count 1 static access. Sites: `move.l` at 0xffe69976.
  Four-byte load, pattern 6: target `abs 0x30000f78`, raw base `absolute`. Count 1 static access. Sites: `move.l` at 0xffe6997a.
  Four-byte load, pattern 7: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `add.l` at 0xffe69980.
  Four-byte store, pattern 8: target `sp +0x4`, raw base `a6`, value `([sp+4] + [0x30000f78])`. Count 1 static access. Sites: `add.l` at 0xffe69980.
  Four-byte load, pattern 9: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `cmp.l` at 0xffe6998a.
  Four-byte load, pattern 10: target `sp +0x8`, raw base `a6`. Count 2 static accesses. Sites: `move.l` at 0xffe69990, `move.l` at 0xffe699be.
  Four-byte load, pattern 11: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `add.l` at 0xffe69994.
  Four-byte load, pattern 12: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe699a4.
  Four-byte store, pattern 13: target `[sp+12] +0x0`, raw base `a5`, value `([sp+4] + [0x30000f78])`. Count 1 static access. Sites: `move.l` at 0xffe699a4.
  Four-byte load, pattern 14: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe699b2.
  Four-byte load, pattern 15: target `abs 0x30000f74`, raw base `absolute`. Count 3 static accesses. Sites: `movea.l` at 0xffe699ca, `movea.l` at 0xffe69a64, `movea.l` at 0xffe69a72.
  Four-byte load, pattern 16: target `[0x30000f74] +0x0`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe699d0.
  Four-byte load, pattern 17: target `abs 0x30000f7c`, raw base `absolute`. Count 1 static access. Sites: `cmp.l` at 0xffe699d2.
  Four-byte load, pattern 18: target `abs 0x30000198`, raw base `absolute`. Count 2 static accesses. Sites: `movea.l` at 0xffe699e2, `movea.l` at 0xffe699ee.
  Byte store, pattern 19: target `[0x30000198] +0x43`, raw base `a0`, value `0x55`. Count 1 static access. Sites: `move.b` at 0xffe699ea.
  Byte store, pattern 20: target `[0x30000198] +0x43`, raw base `a0`, value `0xaa`. Count 1 static access. Sites: `move.b` at 0xffe699f6.
  Four-byte load, pattern 21: target `abs 0x30000f6c`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe69a12.
  Four-byte load, pattern 22: target `unknown`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe69a1a.
  Four-byte store, pattern 23: target `sp -0x34`, raw base `a7`, value `sp-8`. Count 1 static access. Sites: `pea.l` at 0xffe69a28.
  Four-byte load, pattern 24: target `abs 0x30000f6c`, raw base `absolute`. Count 1 static access. Sites: `add.l` at 0xffe69a2e.
  Four-byte store, pattern 25: target `sp -0x38`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe69a34.
  Four-byte store, pattern 26: target `[sp+12] +0x0`, raw base `a5`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe69a4e.
  Four-byte load, pattern 27: target `sp -0x8`, raw base `a6`. Count 2 static accesses. Sites: `move.l` at 0xffe69a50, `move.l` at 0xffe69a80.
  Four-byte store, pattern 28: target `sp -0x34`, raw base `a7`, value `[sp-8]`. Count 2 static accesses. Sites: `move.l` at 0xffe69a50, `move.l` at 0xffe69a80.
  Four-byte load, pattern 29: target `unknown`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe69a6a.
  Four-byte load, pattern 30: target `unknown`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe69a78.
  Call (`bsr.w` at 0xffe69958): (unknown callee), target 0xffe698a0. Candidate slots: stack arg0=`0x0`, stack arg1=`d2`, stack arg2=`d3`, stack arg3=`d4`, stack arg4=`d5`, stack arg5=`d6`.
  Call (`jsr` at 0xffe6996e): (unknown callee), target 0xffe4629c. Candidate slots: stack arg0=`0x700`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe69a36): (unknown callee), target 0x30000e44. Candidate slots: stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe69a54): (unknown callee), target 0xffe4629c. Candidate slots: stack arg0=`[sp-8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe69a84): (unknown callee), target 0xffe4629c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe69954): target 0xffe6996a. Taken when `[0x30000f70] != 0x0`. Falls through when `[0x30000f70] == 0x0`.
  Branch (`bpl.b` at 0xffe69962): target 0xffe6996a. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.w` at 0xffe69966): target 0xffe69a8e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bhi.b` at 0xffe6998e): target 0xffe699a0. Taken when `0xffe00000 > ([sp+4] + [0x30000f78]) (unsigned, 32-bit)`. Falls through when `0xffe00000 <= ([sp+4] + [0x30000f78]) (unsigned, 32-bit)`.
  Branch (`bls.b` at 0xffe6999e): target 0xffe699b2. Taken when `([sp+8] + ([sp+4] + [0x30000f78])) <= 0xfff00000 (unsigned, 32-bit)`. Falls through when `([sp+8] + ([sp+4] + [0x30000f78])) > 0xfff00000 (unsigned, 32-bit)`.
  Branch (`beq.b` at 0xffe699a2): target 0xffe699a8. Taken when `[sp+12] == 0x0`. Falls through when `[sp+12] != 0x0`.
  Jump (`bra.w` at 0xffe699ae): target 0xffe69a8e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bge.w` at 0xffe699d8): target 0xffe69a80. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`bcs.w` at 0xffe699de): target 0xffe69a80. Taken when `C == 1`. Falls through when `not (C == 1)`.
  Branch (`bne.b` at 0xffe699fc): target 0xffe69a0a. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`bcs.w` at 0xffe69a00): target 0xffe69a64. Taken when `C == 1`. Falls through when `not (C == 1)`.
  Branch (`bhi.w` at 0xffe69a06): target 0xffe69a64. Taken when `hi flags condition (operands unknown)`. Falls through when `not (hi flags condition (operands unknown))`.
  Branch (`bcc.b` at 0xffe69a10): target 0xffe69a24. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Branch (`bne.b` at 0xffe69a1e): target 0xffe69a24. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe69a22): target 0xffe69a0e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe69a26): target 0xffe69a64. Taken when `C == 0`. Falls through when `not (C == 0)`.
  Branch (`bpl.b` at 0xffe69a40): target 0xffe69a64. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Branch (`beq.b` at 0xffe69a44): target 0xffe69a50. Taken when `[sp+12] == 0x0`. Falls through when `[sp+12] != 0x0`.
  Jump (`bra.b` at 0xffe69a62): target 0xffe69a8e. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe69a7c): target 0xffe699d2. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe69a96), d0=`unknown`.
  Register fact (`moveq` at 0xffe6994c): d4=`0x0`.
  Register fact (`move.l` at 0xffe69984): d0=`0xffe00000`.
  Register fact (`move.l` at 0xffe699a8): d0=`0xfffffbb4`.
  Register fact (`moveq` at 0xffe699c6): d6=`0x0`.
  Register fact (`moveq` at 0xffe699c8): d3=`0x0`.
  Register fact (`moveq` at 0xffe699e8): d0=`0x55`.
  Register fact (`moveq` at 0xffe699f4): d0=`0xffffffaa`.
  Register fact (`moveq` at 0xffe69a0a): d4=`0x1`.
  Register fact (`move.l` at 0xffe69a46): d0=`0xffe00000`.
  Register fact (`move.l` at 0xffe69a5c): d0=`0xfffffbb1`.
  Register fact (`moveq` at 0xffe69a8c): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe70110`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe70110 to 0xffe7029a (exclusive), 394 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `8d86a7c2f399f062dd9252f498abf199eb316254d9a81ec22c09de707ebe7ecb`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `movea.l` at 0xffe7011c.
  Four-byte load, pattern 2: target `abs 0x30000058`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe70120.
  Four-byte load, pattern 3: target `unknown`, raw base `a4`. Count 1 static access. Sites: `move.l` at 0xffe7012c.
  Four-byte load, pattern 4: target `[sp+4] +0x0`, raw base `a5`. Count 1 static access. Sites: `cmp.l` at 0xffe7012e.
  Four-byte load, pattern 5: target `[sp+4] +0x0`, raw base `a5`. Count 1 static access. Sites: `move.l` at 0xffe70134.
  Four-byte load, pattern 6: target `unknown`, raw base `a4`. Count 1 static access. Sites: `cmp.l` at 0xffe7013c.
  Four-byte store, pattern 7: target `sp -0x3c`, raw base `a7`, value `0xffffffff`. Count 1 static access. Sites: `pea.l` at 0xffe70142.
  Four-byte store, pattern 8: target `sp -0x40`, raw base `a7`, value `0x30002634`. Count 1 static access. Sites: `pea.l` at 0xffe70146.
  Four-byte store, pattern 9: target `sp -0x3c`, raw base `a7`, value `ret(0xffe488d4)`. Count 1 static access. Sites: `move.l` at 0xffe7015a.
  Four-byte store, pattern 10: target `sp -0x40`, raw base `a7`, value `0x1`. Count 2 static accesses. Sites: `pea.l` at 0xffe7015c, `pea.l` at 0xffe70198.
  Four-byte store, pattern 11: target `sp -0x44`, raw base `a7`, value `0x4ea`. Count 1 static access. Sites: `pea.l` at 0xffe70160.
  Four-byte store, pattern 12: target `sp -0x48`, raw base `a7`, value `0xffe70cc0`. Count 2 static accesses. Sites: `pea.l` at 0xffe70164, `pea.l` at 0xffe701a0.
  Four-byte load, pattern 13: target `sp +0x8`, raw base `a6`. Count 1 static access. Sites: `movea.l` at 0xffe70174.
  Two-byte load, pattern 14: target `[sp+8] +0x4`, raw base `a0`. Count 1 static access. Sites: `move.w` at 0xffe70178.
  Two-byte store, pattern 15: target `[sp+8] +0x4`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.w` at 0xffe7017e.
  Four-byte store, pattern 16: target `sp -0x3c`, raw base `a7`, value `0x30002634`. Count 1 static access. Sites: `pea.l` at 0xffe70182.
  Four-byte store, pattern 17: target `sp -0x3c`, raw base `a7`, value `ret(0xffe4892c)`. Count 1 static access. Sites: `move.l` at 0xffe70196.
  Four-byte store, pattern 18: target `sp -0x44`, raw base `a7`, value `0x4ec`. Count 1 static access. Sites: `pea.l` at 0xffe7019c.
  Two-byte load, pattern 19: target `unknown`, raw base `a4`. Count 1 static access. Sites: `move.w` at 0xffe701b2.
  Two-byte load, pattern 20: target `unknown`, raw base `a4`. Count 1 static access. Sites: `move.w` at 0xffe701be.
  Two-byte load, pattern 21: target `unknown`, raw base `a4`. Count 4 static accesses. Sites: `move.w` at 0xffe701c8, `move.w` at 0xffe701d4, `move.w` at 0xffe70258.
  Two-byte store, pattern 22: target `unknown`, raw base `a4`, value `unknown`. Count 2 static accesses. Sites: `move.w` at 0xffe701da, `move.w` at 0xffe7026a.
  Four-byte load, pattern 23: target `sp +0x8`, raw base `a6`. Count 3 static accesses. Sites: `move.l` at 0xffe701de, `move.l` at 0xffe7026e, `move.l` at 0xffe7027e.
  Four-byte store, pattern 24: target `sp -0x3c`, raw base `a7`, value `[sp+8]`. Count 3 static accesses. Sites: `move.l` at 0xffe701de, `move.l` at 0xffe7026e, `move.l` at 0xffe7027e.
  Four-byte store, pattern 25: target `sp -0x3c`, raw base `a7`, value `0x1f`. Count 1 static access. Sites: `pea.l` at 0xffe701ec.
  Four-byte store, pattern 26: target `sp -0x40`, raw base `a7`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe701f0.
  Four-byte store, pattern 27: target `sp -0x44`, raw base `a7`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe701f2.
  Four-byte store, pattern 28: target `sp -0x48`, raw base `a7`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe701f4.
  Four-byte load, pattern 29: target `abs 0x30000074`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe701f6.
  Four-byte load, pattern 30: target `[0x30000074] +0x0`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe701fc.
  Four-byte store, pattern 31: target `sp -0x4c`, raw base `a7`, value `[[0x30000074]]`. Count 1 static access. Sites: `move.l` at 0xffe701fc.
  Four-byte store, pattern 32: target `sp -0x50`, raw base `a7`, value `0x1`. Count 1 static access. Sites: `pea.l` at 0xffe701fe.
  Four-byte store, pattern 33: target `sp -0x54`, raw base `a7`, value `sp+8`. Count 1 static access. Sites: `pea.l` at 0xffe70202.
  Four-byte load, pattern 34: target `unknown`, raw base `a4`. Count 1 static access. Sites: `move.l` at 0xffe70206.
  Four-byte store, pattern 35: target `sp -0x58`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe70206.
  Two-byte store, pattern 36: target `unknown`, raw base `a4`, value `0x1`. Count 1 static access. Sites: `move.w` at 0xffe7022a.
  Four-byte load, pattern 37: target `unknown`, raw base `a4`. Count 2 static accesses. Sites: `movea.l` at 0xffe7022e, `movea.l` at 0xffe7027a.
  Four-byte store, pattern 38: target `sp -0x3c`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `pea.l` at 0xffe70232.
  Four-byte store, pattern 39: target `sp -0x40`, raw base `a7`, value `0xffe70d8c`. Count 1 static access. Sites: `pea.l` at 0xffe70236.
  Four-byte store, pattern 40: target `sp -0x44`, raw base `a7`, value `sp-44`. Count 1 static access. Sites: `pea.l` at 0xffe7023c.
  Four-byte store, pattern 41: target `sp -0x3c`, raw base `a7`, value `sp-44`. Count 1 static access. Sites: `pea.l` at 0xffe7024a.
  Four-byte load, pattern 42: target `unknown`, raw base `a4`. Count 1 static access. Sites: `movea.l` at 0xffe70286.
  Call (`jsr` at 0xffe7014c): (unknown callee), target 0xffe488d4. Candidate slots: stack arg0=`0x30002634`, stack arg1=`0xffffffff`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7016a): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`0xffe70cc0`, stack arg1=`0x4ea`, stack arg2=`0x1`, stack arg3=`ret(0xffe488d4)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe70188): (unknown callee), target 0xffe4892c. Candidate slots: stack arg0=`0x30002634`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe701a6): (unknown callee), target 0xffe53984. Candidate slots: stack arg0=`0xffe70cc0`, stack arg1=`0x4ec`, stack arg2=`0x1`, stack arg3=`ret(0xffe4892c)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe701e2): (unknown callee), target 0xffe6fc10. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7020a): (unknown callee), target 0xffe47e00. Candidate slots: stack arg0=`unknown`, stack arg1=`sp+8`, stack arg2=`0x1`, stack arg3=`[[0x30000074]]`, stack arg4=`0x0`, stack arg5=`0x0`.
  Call (`jsr` at 0xffe70240): (unknown callee), target 0xffe716d8. Candidate slots: stack arg0=`sp-44`, stack arg1=`0xffe70d8c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe7024e): (unknown callee), target 0xffe581dc. Candidate slots: stack arg0=`sp-44`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe70272): (unknown callee), target 0xffe6fc10. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe70282): (unknown callee), target unknown. Candidate slots: stack arg0=`[sp+8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.w` at 0xffe70128): target 0xffe7028e. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.w` at 0xffe70130): target 0xffe70286. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.w` at 0xffe7013e): target 0xffe70286. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe70158): target 0xffe70174. Taken when `ret(0xffe488d4) == 0x0`. Falls through when `ret(0xffe488d4) != 0x0`.
  Branch (`beq.b` at 0xffe70194): target 0xffe701b0. Taken when `ret(0xffe4892c) == 0x0`. Falls through when `ret(0xffe4892c) != 0x0`.
  Branch (`bne.w` at 0xffe701b8): target 0xffe7027a. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`beq.b` at 0xffe701c4): target 0xffe701ec. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bge.b` at 0xffe701d2): target 0xffe701de. Taken when `? >= 0xffff (signed, 32-bit)`. Falls through when `? < 0xffff (signed, 32-bit)`.
  Jump (`bra.w` at 0xffe701e8): target 0xffe70286. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.w` at 0xffe70218): target 0xffe70286. Taken when `ret(0xffe47e00) == 0x0`. Falls through when `ret(0xffe47e00) != 0x0`.
  Branch (`beq.b` at 0xffe70220): target 0xffe70228. Taken when `0xffffffce == ret(0xffe47e00)`. Falls through when `0xffffffce != ret(0xffe47e00)`.
  Branch (`bne.b` at 0xffe70226): target 0xffe7026e. Taken when `0xffffffd2 != ret(0xffe47e00)`. Falls through when `0xffffffd2 == ret(0xffe47e00)`.
  Branch (`bge.b` at 0xffe70262): target 0xffe7026e. Taken when `? >= 0xffff (signed, 32-bit)`. Falls through when `? < 0xffff (signed, 32-bit)`.
  Jump (`bra.b` at 0xffe70278): target 0xffe70286. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.w` at 0xffe7028a): target 0xffe70126. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe70298), d0=`0x0`.
  Register fact (`moveq` at 0xffe701b0): d0=`0x0`.
  Register fact (`moveq` at 0xffe701bc): d0=`0x0`.
  Register fact (`moveq` at 0xffe701c6): d0=`0x0`.
  Register fact (`moveq` at 0xffe7021c): d0=`0xffffffce`.
  Register fact (`moveq` at 0xffe70222): d0=`0xffffffd2`.
  Register fact (`moveq` at 0xffe70228): d0=`0x1`.
  Register fact (`moveq` at 0xffe70256): d0=`0x0`.
  Register fact (`moveq` at 0xffe7028e): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.
