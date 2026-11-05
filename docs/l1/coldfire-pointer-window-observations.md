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
Crossing instructions and four early decoder-stop tails remain raw and open.
Those tails establish no caller arguments or control flow. Pre-entry slices have unproved boundaries.
Aligned absolute-JSR byte matches remain hints. They do not prove that each occurrence executes.

The [full facts](disasm/coldfire-variants/pointer-window-observations-facts.txt) retain every instruction and warning.

- `0xffe4c56c`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4c56c to 0xffe4c59a (exclusive), 46 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `f070fc9128ecb7118558e6dd5125011cc98d7aff665ce4263f82ff2d06623abd`.
  Four-byte load, pattern 1: target `abs 0x30000490`, raw base `absolute`. Count 2 static accesses. Sites: `movea.l` at 0xffe4c570, `movea.l` at 0xffe4c57c.
  Byte store, pattern 2: target `[0x30000490] +0x43`, raw base `a0`, value `0x55`. Count 1 static access. Sites: `move.b` at 0xffe4c578.
  Byte store, pattern 3: target `[0x30000490] +0x43`, raw base `a0`, value `0xaa`. Count 1 static access. Sites: `move.b` at 0xffe4c584.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `a7`, value `0x14`. Count 1 static access. Sites: `pea.l` at 0xffe4c588.
  Call (`jsr` at 0xffe4c58c): (unknown callee), target 0xffe3eaf8. Candidate slots: stack arg0=`0x14`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffe4c594): target 0xffe4c570. The local CFG lists a taken edge and no fallthrough edge.
  No reached exit is modeled.
  Register fact (`moveq` at 0xffe4c576): d0=`0x55`.
  Register fact (`moveq` at 0xffe4c582): d0=`0xffffffaa`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.
  Local unreached instructions: 0xffe4c596 to 0xffe4c598, ending at 0xffe4c59a (exclusive). Count 2.

- `0xffe5a36c`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5a36c to 0xffe5a3a4 (exclusive), 56 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `dcbff06188de1ca11719fc7534b173ff6bb9b2abdf08d802751a7b606501d533`.
  Four-byte load, pattern 1: target `abs 0x30000490`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe5a378.
  Byte load, pattern 2: target `[0x30000490] +0x1ec`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe5a37e.
  Call (`bsr.w` at 0xffe5a38e): (unknown callee), target 0xffe59f18. Candidate slots: stack arg0=`d7`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5a386): target 0xffe5a39a. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Branch (`bne.b` at 0xffe5a38c): target 0xffe5a378. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe5a398): target 0xffe5a39c. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe5a3a2), d0=`unknown`.
  Register fact (`move.l` at 0xffe5a372): d7=`0x3e8`.
  Register fact (`move.l` at 0xffe5a392): d0=`0xffffff33`.
  Register fact (`moveq` at 0xffe5a39a): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe4c528`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4c528 to 0xffe4c56c (exclusive), 68 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `4cb2b1500541a80091c2154dee32cbee22b6e9d2b8c63c8cd4d220c8dcb31445`.
  Four-byte load, pattern 1: target `abs 0x30000550`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe4c52c.
  Four-byte store, pattern 2: target `abs 0x30000550`, raw base `absolute`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe4c534.
  Four-byte load, pattern 3: target `abs 0x30000490`, raw base `absolute`. Count 4 static accesses. Sites: `movea.l` at 0xffe4c53a, `movea.l` at 0xffe4c546, `movea.l` at 0xffe4c550.
  Byte store, pattern 4: target `[0x30000490] +0x41`, raw base `a0`, value `0xfe`. Count 1 static access. Sites: `move.b` at 0xffe4c542.
  Byte store, pattern 5: target `[0x30000490] +0x42`, raw base `a0`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe4c54c.
  Byte store, pattern 6: target `[0x30000490] +0x43`, raw base `a0`, value `0x55`. Count 1 static access. Sites: `move.b` at 0xffe4c558.
  Byte store, pattern 7: target `[0x30000490] +0x43`, raw base `a0`, value `0xaa`. Count 1 static access. Sites: `move.b` at 0xffe4c564.
  Branch (`beq.b` at 0xffe4c532): target 0xffe4c568. Taken when `[0x30000550] == 0x0`. Falls through when `[0x30000550] != 0x0`.
  At the exit (`rts` at 0xffe4c56a), d0=`unknown`.
  Register fact (`moveq` at 0xffe4c540): d0=`0xfffffffe`.
  Register fact (`moveq` at 0xffe4c556): d0=`0x55`.
  Register fact (`moveq` at 0xffe4c562): d0=`0xffffffaa`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe5adec`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5adec to 0xffe5ae4a (exclusive), 94 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `c4c275caed5fbe3b8b6a17c51f6b5b4e1447ba89a428f7fe8fb38021e5468e7a`.
  Four-byte load, pattern 1: target `abs 0x30000490`, raw base `absolute`. Count 1 static access. Sites: `movea.l` at 0xffe5adf2.
  Two-byte load, pattern 2: target `abs 0x3000750c`, raw base `absolute`. Count 1 static access. Sites: `move.w` at 0xffe5adf8.
  Two-byte load, pattern 3: target `[0x30000490] +0x36`, raw base `a0`. Count 1 static access. Sites: `move.w` at 0xffe5adfe.
  Two-byte store, pattern 4: target `[0x30000490] +0x36`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.w` at 0xffe5ae04.
  Four-byte load, pattern 5: target `abs 0x30007c28`, raw base `absolute`. Count 1 static access. Sites: `addq.l` at 0xffe5ae08.
  Four-byte store, pattern 6: target `abs 0x30007c28`, raw base `absolute`, value `[0x30007c28]+1`. Count 1 static access. Sites: `addq.l` at 0xffe5ae08.
  Four-byte store, pattern 7: target `sp -0xc`, raw base `a7`, value `0x30007bbc`. Count 2 static accesses. Sites: `move.l` at 0xffe5ae0e, `move.l` at 0xffe5ae22.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `a7`, value `ret(0xffe3eb64)`. Count 1 static access. Sites: `move.l` at 0xffe5ae28.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `a7`, value `0x2`. Count 1 static access. Sites: `pea.l` at 0xffe5ae2a.
  Four-byte store, pattern 10: target `sp -0x18`, raw base `a7`, value `0x1d3`. Count 1 static access. Sites: `pea.l` at 0xffe5ae2e.
  Four-byte store, pattern 11: target `sp -0x1c`, raw base `a7`, value `0xffe5aef0`. Count 1 static access. Sites: `pea.l` at 0xffe5ae32.
  Call (`jsr` at 0xffe5ae14): (unknown callee), target 0xffe3eb64. Candidate slots: stack arg0=`0x30007bbc`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5ae38): (unknown callee), target 0xffe4c250. Candidate slots: stack arg0=`0xffe5aef0`, stack arg1=`0x1d3`, stack arg2=`0x2`, stack arg3=`ret(0xffe3eb64)`, stack arg4=`0x30007bbc`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5ae20): target 0xffe5ae42. Taken when `ret(0xffe3eb64) == 0x0`. Falls through when `ret(0xffe3eb64) != 0x0`.
  At the exit (`rts` at 0xffe5ae48), d0=`unknown`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe53afc`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe53afc to 0xffe53b66 (exclusive), 106 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `afd0d854667ffc259fbe56ce78f9ba4064a1451598adf277c904a15fcc1b4bb3`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 2 static accesses. Sites: `movea.l` at 0xffe53b06, `movea.l` at 0xffe53b5a.
  Four-byte store, pattern 2: target `[sp+4] +0x0`, raw base `a0`, value `ret(0xffe3d98e)`. Count 1 static access. Sites: `move.l` at 0xffe53b0a.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `a7`, value `0x0`. Count 1 static access. Sites: `move.l` at 0xffe53b0c.
  Four-byte store, pattern 4: target `sp -0xc`, raw base `a7`, value `0x40f86a00`. Count 1 static access. Sites: `move.l` at 0xffe53b12.
  Four-byte load, pattern 5: target `abs 0x30000490`, raw base `absolute`. Count 2 static accesses. Sites: `movea.l` at 0xffe53b18, `movea.l` at 0xffe53b2c.
  Two-byte load, pattern 6: target `[0x30000490] +0x104`, raw base `a1`. Count 1 static access. Sites: `move.w` at 0xffe53b20.
  Four-byte store, pattern 7: target `sp -0x10`, raw base `a7`, value `ret(0xffe6d252)`. Count 1 static access. Sites: `move.l` at 0xffe53b2a.
  Two-byte load, pattern 8: target `[0x30000490] +0x10c`, raw base `a1`. Count 1 static access. Sites: `move.w` at 0xffe53b34.
  Four-byte store, pattern 9: target `[sp+4] +0x4`, raw base `a0`, value `ret(0xffe6ce0e)`. Count 1 static access. Sites: `move.l` at 0xffe53b5e.
  Call (`jsr` at 0xffe53b00): (unknown callee), target 0xffe3d98e. Candidate slots: stack arg0=`a6`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe53b24): (unknown callee), target 0xffe6d252. Candidate slots: stack arg0=`0x40f86a00`, stack arg1=`0x0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe53b38): (unknown callee), target 0xffe6d252. Candidate slots: stack arg0=`ret(0xffe6d252)`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe53b3e): (unknown callee), target 0xffe6d1d2. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe53b46): (unknown callee), target 0xffe6d878. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe53b4c): (unknown callee), target 0xffe6d300. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe53b54): (unknown callee), target 0xffe6ce0e. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the exit (`rts` at 0xffe53b64), d0=`ret(0xffe6ce0e)`.
  Register fact (`moveq` at 0xffe53b1e): d0=`0x0`.
  Register fact (`moveq` at 0xffe53b32): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe4bc14`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4bc14 to 0xffe4bc84 (exclusive), 112 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `a0970db4237005538e36d04762839925fb1e15f2eaf36f909bb4db4e24179d0b`.
  Four-byte load, pattern 1: target `abs 0x3000052c`, raw base `absolute`. Count 4 static accesses. Sites: `movea.l` at 0xffe4bc18, `movea.l` at 0xffe4bc2c, `movea.l` at 0xffe4bc56.
  Four-byte load, pattern 2: target `abs 0x3000052c`, raw base `absolute`. Count 4 static accesses. Sites: `movea.l` at 0xffe4bc1e, `movea.l` at 0xffe4bc32, `movea.l` at 0xffe4bc44.
  Four-byte load, pattern 3: target `[0x3000052c] +0xe4`, raw base `a1`. Count 2 static accesses. Sites: `move.l` at 0xffe4bc24, `move.l` at 0xffe4bc38.
  Four-byte store, pattern 4: target `[0x3000052c] +0xec`, raw base `a0`, value `[[0x3000052c]+228]`. Count 1 static access. Sites: `move.l` at 0xffe4bc28.
  Four-byte store, pattern 5: target `[0x3000052c] +0xf0`, raw base `a0`, value `[[0x3000052c]+228]`. Count 1 static access. Sites: `move.l` at 0xffe4bc3c.
  Four-byte store, pattern 6: target `sp -0x8`, raw base `a7`, value `0x24`. Count 1 static access. Sites: `pea.l` at 0xffe4bc40.
  Four-byte store, pattern 7: target `sp -0xc`, raw base `a7`, value `[0x3000052c]+224`. Count 1 static access. Sites: `pea.l` at 0xffe4bc4a.
  Byte store, pattern 8: target `[0x3000052c] +0x103`, raw base `a0`, value `(ret(0xffe52a6c) & 0xff)`. Count 1 static access. Sites: `move.b` at 0xffe4bc5c.
  Four-byte load, pattern 9: target `unknown`, raw base `a0`. Count 3 static accesses. Sites: `move.l` at 0xffe4bc76, `move.l` at 0xffe4bc78, `move.l` at 0xffe4bc7e.
  Four-byte store, pattern 10: target `unknown`, raw base `a1`, value `unknown`. Count 3 static accesses. Sites: `move.l` at 0xffe4bc76, `move.l` at 0xffe4bc78, `move.l` at 0xffe4bc7e.
  Call (`jsr` at 0xffe4bc4e): (unknown callee), target 0xffe52a6c. Candidate slots: stack arg0=`[0x3000052c]+224`, stack arg1=`0x24`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bgt.b` at 0xffe4bc7c): target 0xffe4bc76. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  At the exit (`rts` at 0xffe4bc82), d0=`unknown`.
  Register fact (`moveq` at 0xffe4bc74): d0=`0x4`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe6fc44`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6fc44 to 0xffe6fcd6 (exclusive), 146 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `ac1edf52064f40ef3fa7098ec2b8f0490790c6772c6887297d8e573c4525b8b1`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe6fc50.
  Four-byte load, pattern 2: target `abs 0x30000490`, raw base `absolute`. Count 2 static accesses. Sites: `movea.l` at 0xffe6fc56, `movea.l` at 0xffe6fc62.
  Byte store, pattern 3: target `[0x30000490] +0x43`, raw base `a0`, value `0x55`. Count 1 static access. Sites: `move.b` at 0xffe6fc5e.
  Byte store, pattern 4: target `[0x30000490] +0x43`, raw base `a0`, value `0xaa`. Count 1 static access. Sites: `move.b` at 0xffe6fc6a.
  Four-byte load, pattern 5: target `abs 0x3000124c`, raw base `absolute`. Count 2 static accesses. Sites: `movea.l` at 0xffe6fc6e, `movea.l` at 0xffe6fc76.
  Two-byte load, pattern 6: target `[0x3000124c] +0x0`, raw base `a0`. Count 1 static access. Sites: `move.w` at 0xffe6fc74.
  Two-byte load, pattern 7: target `[0x3000124c] +0x0`, raw base `a0`. Count 1 static access. Sites: `move.w` at 0xffe6fc7c.
  Four-byte store, pattern 8: target `sp -0x18`, raw base `a7`, value `0x3`. Count 1 static access. Sites: `pea.l` at 0xffe6fc9e.
  Four-byte store, pattern 9: target `sp -0x1c`, raw base `a7`, value `0xffe5ed64`. Count 1 static access. Sites: `pea.l` at 0xffe6fca2.
  Call (`bsr.w` at 0xffe6fca8): (unknown callee), target 0xffe6fc04. Candidate slots: stack arg0=`0xffe5ed64`, stack arg1=`0x3`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe6fc94): target 0xffe6fc9a. Taken when `? != ?`. Falls through when `? == ?`.
  Jump (`bra.b` at 0xffe6fc98): target 0xffe6fccc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe6fc9c): target 0xffe6fcb2. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe6fcb0): target 0xffe6fccc. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe6fcc0): target 0xffe6fcc4. Taken when `0x20 != ?`. Falls through when `0x20 == ?`.
  Branch (`bne.w` at 0xffe6fcc6): target 0xffe6fc56. Taken when `[sp+4] != 0x0`. Falls through when `[sp+4] == 0x0`.
  At the exit (`rts` at 0xffe6fcd4), d0=`unknown`.
  Register fact (`moveq` at 0xffe6fc54): d4=`0x0`.
  Register fact (`moveq` at 0xffe6fc5c): d0=`0x55`.
  Register fact (`moveq` at 0xffe6fc68): d0=`0xffffffaa`.
  Register fact (`moveq` at 0xffe6fc7e): d0=`0x0`.
  Register fact (`moveq` at 0xffe6fc88): d1=`0x0`.
  Register fact (`moveq` at 0xffe6fc96): d0=`0x0`.
  Register fact (`moveq` at 0xffe6fcae): d0=`0xffffffff`.
  Register fact (`moveq` at 0xffe6fcb2): d0=`0x0`.
  Register fact (`moveq` at 0xffe6fcbc): d1=`0x20`.
  Register fact (`moveq` at 0xffe6fcc2): d4=`0x1`.
  Register fact (`moveq` at 0xffe6fcca): d0=`0x1`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe5f4cc`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5f4cc to 0xffe5f576 (exclusive), 170 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `dd280c90a7251e62e49251bb87ff3cde7cf42a2a17d2674ca635f45896fb7677`.
  Four-byte load, pattern 1: target `abs 0x3000003c`, raw base `absolute`. Count 1 static access. Sites: `tst.l` at 0xffe5f4d0.
  Four-byte store, pattern 2: target `sp -0x8`, raw base `a7`, value `0xffe5fe70`. Count 1 static access. Sites: `pea.l` at 0xffe5f4d8.
  Four-byte store, pattern 3: target `sp -0x8`, raw base `a7`, value `0xffe5fbdc`. Count 1 static access. Sites: `pea.l` at 0xffe5f4e6.
  Four-byte store, pattern 4: target `sp -0xc`, raw base `a7`, value `0xffe5fca8`. Count 1 static access. Sites: `pea.l` at 0xffe5f4ec.
  Four-byte load, pattern 5: target `abs 0x30000490`, raw base `absolute`. Count 7 static accesses. Sites: `movea.l` at 0xffe5f4fa, `movea.l` at 0xffe5f50e, `movea.l` at 0xffe5f51a.
  Two-byte load, pattern 6: target `[0x30000490] +0xca`, raw base `a0`. Count 1 static access. Sites: `move.w` at 0xffe5f500.
  Two-byte store, pattern 7: target `[0x30000490] +0xca`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.w` at 0xffe5f50a.
  Byte store, pattern 8: target `[0x30000490] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access. Sites: `move.b` at 0xffe5f516.
  Byte store, pattern 9: target `[0x30000490] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access. Sites: `move.b` at 0xffe5f522.
  Byte store, pattern 10: target `[0x30000490] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses. Sites: `move.b` at 0xffe5f52e, `move.b` at 0xffe5f546.
  Byte store, pattern 11: target `[0x30000490] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access. Sites: `move.b` at 0xffe5f53a.
  Byte store, pattern 12: target `[0x30000490] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access. Sites: `move.b` at 0xffe5f552.
  Four-byte load, pattern 13: target `abs 0x30000490`, raw base `absolute`. Count 1 static access. Sites: `add.l` at 0xffe5f55c.
  Four-byte store, pattern 14: target `abs 0x30000044`, raw base `absolute`, value `[0x30000490]+457`. Count 1 static access. Sites: `move.l` at 0xffe5f562.
  Four-byte store, pattern 15: target `abs 0x30000040`, raw base `absolute`, value `0x1`. Count 1 static access. Sites: `move.l` at 0xffe5f56a.
  Call (`jsr` at 0xffe5f4de): (unknown callee), target 0xffe69384. Candidate slots: stack arg0=`0xffe5fe70`, stack arg1=`a6`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5f4f2): (unknown callee), target 0xffe642e4. Candidate slots: stack arg0=`0xffe5fca8`, stack arg1=`0xffe5fbdc`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5f4d6): target 0xffe5f4e6. Taken when `[0x3000003c] == 0x0`. Falls through when `[0x3000003c] != 0x0`.
  At the exit (`rts` at 0xffe5f574), d0=`0x1`.
  Register fact (`moveq` at 0xffe5f514): d0=`0x31`.
  Register fact (`moveq` at 0xffe5f520): d0=`0xfffffffe`.
  Register fact (`moveq` at 0xffe5f52c): d0=`0x38`.
  Register fact (`moveq` at 0xffe5f538): d0=`0x28`.
  Register fact (`moveq` at 0xffe5f544): d0=`0x38`.
  Register fact (`moveq` at 0xffe5f550): d0=`0x30`.
  Register fact (`move.l` at 0xffe5f556): d0=`0x1c9`.
  Register fact (`moveq` at 0xffe5f568): d0=`0x1`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe62e54`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe62e54 to 0xffe62f02 (exclusive), 174 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `6661c7ac9c04530dd114dd58c38bee613a18920eac9f3de7ec31a33b01412d41`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe62e5a.
  Four-byte load, pattern 2: target `abs 0x30000490`, raw base `absolute`. Count 3 static accesses. Sites: `movea.l` at 0xffe62e64, `movea.l` at 0xffe62e7e, `movea.l` at 0xffe62ea6.
  Byte load, pattern 3: target `[0x30000490] +0x1c9`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe62e72.
  Byte store, pattern 4: target `[0x30000490] +0x1c9`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.b` at 0xffe62e78.
  Byte load, pattern 5: target `unknown`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe62e8e.
  Byte store, pattern 6: target `unknown`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.b` at 0xffe62e94.
  Byte load, pattern 7: target `[0x30000490] +0x1c9`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe62eae.
  Byte load, pattern 8: target `abs 0x30000389`, raw base `absolute`. Count 2 static accesses. Sites: `move.b` at 0xffe62ec0, `move.b` at 0xffe62ed4.
  Byte store, pattern 9: target `unknown`, raw base `a0`, value `0x0`. Count 2 static accesses. Sites: `clr.b` at 0xffe62ec8, `clr.b` at 0xffe62ee8.
  Byte load, pattern 10: target `abs 0x3000038a`, raw base `absolute`. Count 1 static access. Sites: `move.b` at 0xffe62ee0.
  Byte load, pattern 11: target `abs 0x30000388`, raw base `absolute`. Count 1 static access. Sites: `tst.b` at 0xffe62eea.
  Byte store, pattern 12: target `abs 0x30000388`, raw base `absolute`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe62ef2.
  Branch (`beq.b` at 0xffe62e62): target 0xffe62e7e. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe62e7c): target 0xffe62e98. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe62ea4): target 0xffe62eea. Taken when `0x10 != ?`. Falls through when `0x10 == ?`.
  Branch (`beq.b` at 0xffe62eb6): target 0xffe62ecc. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe62eca): target 0xffe62eea. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe62ef0): target 0xffe62ef8. Taken when `([0x30000388]:1 & 0xff) != 0x0`. Falls through when `([0x30000388]:1 & 0xff) == 0x0`.
  At the exit (`rts` at 0xffe62f00), d0=`0x0`.
  Register fact (`moveq` at 0xffe62ea0): d1=`0x10`.
  Register fact (`moveq` at 0xffe62eac): d0=`0x0`.
  Register fact (`movea.l` at 0xffe62eb8): a0=`0x30002600`.
  Register fact (`moveq` at 0xffe62ebe): d0=`0x0`.
  Register fact (`movea.l` at 0xffe62ecc): a0=`0x30002700`.
  Register fact (`moveq` at 0xffe62ed2): d0=`0x0`.
  Register fact (`moveq` at 0xffe62ede): d0=`0x0`.
  Register fact (`moveq` at 0xffe62ef8): d0=`0x0`.
  Tool warning: `0xffe62e8c not.l: unsupported effects invalidated`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe5a40c`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5a40c to 0xffe5a4c2 (exclusive), 182 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `752e38e1cf60d17a26480bcb5faeb51f6df200b8fcea28210b22420b6429e2a6`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `movea.l` at 0xffe5a418.
  Two-byte load, pattern 2: target `sp +0xa`, raw base `a6`. Count 1 static access. Sites: `move.w` at 0xffe5a41c.
  Four-byte load, pattern 3: target `abs 0x30000490`, raw base `absolute`. Count 4 static accesses. Sites: `movea.l` at 0xffe5a420, `movea.l` at 0xffe5a42c, `movea.l` at 0xffe5a472.
  Byte store, pattern 4: target `[0x30000490] +0x1e8`, raw base `a0`, value `0xa0`. Count 1 static access. Sites: `move.b` at 0xffe5a428.
  Byte load, pattern 5: target `[0x30000490] +0x1f0`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe5a432.
  Byte store, pattern 6: target `sp -0x5`, raw base `a6`, value `unknown`. Count 1 static access. Sites: `move.b` at 0xffe5a436.
  Four-byte store, pattern 7: target `sp -0x1c`, raw base `a7`, value `0x0`. Count 2 static accesses. Sites: `clr.l` at 0xffe5a43a, `clr.l` at 0xffe5a488.
  Four-byte store, pattern 8: target `sp -0x1c`, raw base `a7`, value `0xffe5a738`. Count 1 static access. Sites: `pea.l` at 0xffe5a448.
  Four-byte store, pattern 9: target `sp -0x20`, raw base `a7`, value `0x30006d78`. Count 2 static accesses. Sites: `pea.l` at 0xffe5a44e, `pea.l` at 0xffe5a49c.
  Byte store, pattern 10: target `[0x30000490] +0x1e8`, raw base `a0`, value `0xa8`. Count 1 static access. Sites: `move.b` at 0xffe5a47a.
  Byte load, pattern 11: target `[0x30000490] +0x1f0`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe5a484.
  Byte store, pattern 12: target `unknown`, raw base `a5`, value `([[0x30000490]+496]:1 & 0xff)`. Count 1 static access. Sites: `move.b` at 0xffe5a484.
  Four-byte store, pattern 13: target `sp -0x1c`, raw base `a7`, value `0xffe5a768`. Count 1 static access. Sites: `pea.l` at 0xffe5a496.
  Call (`bsr.w` at 0xffe5a43c): (unknown callee), target 0xffe5a2d0. Candidate slots: stack arg0=`0x0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5a454): (unknown callee), target 0xffe69a28. Candidate slots: stack arg0=`0x30006d78`, stack arg1=`0xffe5a738`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a48a): (unknown callee), target 0xffe5a2d0. Candidate slots: stack arg0=`0x0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5a4a2): (unknown callee), target 0xffe69a28. Candidate slots: stack arg0=`0x30006d78`, stack arg1=`0xffe5a768`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5a446): target 0xffe5a460. Taken when `ret(0xffe5a2d0) == 0x0`. Falls through when `ret(0xffe5a2d0) != 0x0`.
  Jump (`bra.b` at 0xffe5a45e): target 0xffe5a4b8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5a468): target 0xffe5a4b6. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bne.b` at 0xffe5a470): target 0xffe5a47e. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Branch (`beq.b` at 0xffe5a494): target 0xffe5a4ae. Taken when `ret(0xffe5a2d0) == 0x0`. Falls through when `ret(0xffe5a2d0) != 0x0`.
  Jump (`bra.b` at 0xffe5a4ac): target 0xffe5a4b8. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe5a4b4): target 0xffe5a460. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe5a4c0), d0=`unknown`.
  Register fact (`moveq` at 0xffe5a426): d0=`0xffffffa0`.
  Register fact (`moveq` at 0xffe5a46a): d0=`0x0`.
  Register fact (`moveq` at 0xffe5a478): d0=`0xffffffa8`.
  Register fact (`moveq` at 0xffe5a4b6): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe59f18`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe59f18 to 0xffe59fe2 (exclusive), 202 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `676368ac04fef6fbef4297e5f44d08c6e2fc0c5d2c0349e18fcd8ab3eca0b168`.
  Four-byte load, pattern 1: target `abs 0x30000490`, raw base `absolute`. Count 11 static accesses. Sites: `movea.l` at 0xffe59f30, `movea.l` at 0xffe59f3a, `movea.l` at 0xffe59f46.
  Byte store, pattern 2: target `[0x30000490] +0x1e8`, raw base `a0`, value `0x0`. Count 3 static accesses. Sites: `clr.b` at 0xffe59f36, `clr.b` at 0xffe59f72, `clr.b` at 0xffe59fa6.
  Byte store, pattern 3: target `[0x30000490] +0x1e4`, raw base `a0`, value `0x13`. Count 1 static access. Sites: `move.b` at 0xffe59f42.
  Byte store, pattern 4: target `[0x30000490] +0x1e0`, raw base `a0`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe59f4c.
  Byte store, pattern 5: target `[0x30000490] +0x1e8`, raw base `a0`, value `0x80`. Count 1 static access. Sites: `move.b` at 0xffe59f58.
  Byte load, pattern 6: target `[0x30000490] +0x1ec`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe59f62.
  Byte store, pattern 7: target `[0x30000490] +0x1e8`, raw base `a0`, value `0xa0`. Count 1 static access. Sites: `move.b` at 0xffe59f7e.
  Byte load, pattern 8: target `[0x30000490] +0x1f0`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe59f88.
  Byte load, pattern 9: target `[0x30000490] +0x1ec`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe59f92.
  Byte store, pattern 10: target `[0x30000490] +0x1ec`, raw base `a0`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe59f9c.
  Four-byte store, pattern 11: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe59fb6.
  Four-byte store, pattern 12: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe59fbe.
  Four-byte store, pattern 13: target `sp -0x20`, raw base `a7`, value `0x2`. Count 1 static access. Sites: `pea.l` at 0xffe59fc0.
  Four-byte store, pattern 14: target `sp -0x24`, raw base `a7`, value `0xe9`. Count 1 static access. Sites: `pea.l` at 0xffe59fc4.
  Four-byte store, pattern 15: target `sp -0x28`, raw base `a7`, value `0xffe5a534`. Count 1 static access. Sites: `pea.l` at 0xffe59fc8.
  Call (`jsr` at 0xffe59fce): (unknown callee), target 0xffe4c250. Candidate slots: stack arg0=`0xffe5a534`, stack arg1=`0xe9`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bge.w` at 0xffe59f2c): target 0xffe59fb0. Taken when `ge flags condition (operands unknown)`. Falls through when `not (ge flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe59f6a): target 0xffe59fd8. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.w` at 0xffe59fac): target 0xffe59f26. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe59fe0), d0=`unknown`.
  Register fact (`moveq` at 0xffe59f24): d4=`0x0`.
  Register fact (`moveq` at 0xffe59f40): d0=`0x13`.
  Register fact (`moveq` at 0xffe59f56): d0=`0xffffff80`.
  Register fact (`moveq` at 0xffe59f7c): d0=`0xffffffa0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe5a150`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe5a150 to 0xffe5a252 (exclusive), 258 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `4505af2ea5b8078cf6e936b82f98c88a5174389b4790f2f25230ce08221ebcfd`.
  Byte load, pattern 1: target `sp +0x7`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe5a158.
  Four-byte store, pattern 2: target `sp -0x10`, raw base `a7`, value `unknown`. Count 4 static accesses. Sites: `move.l` at 0xffe5a162, `move.l` at 0xffe5a176, `move.l` at 0xffe5a1b8.
  Four-byte store, pattern 3: target `sp -0x14`, raw base `a7`, value `0xffe5a610`. Count 1 static access. Sites: `pea.l` at 0xffe5a178.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `0x30006d78`. Count 3 static accesses. Sites: `pea.l` at 0xffe5a17e, `pea.l` at 0xffe5a1c0, `pea.l` at 0xffe5a216.
  Four-byte load, pattern 5: target `abs 0x30000490`, raw base `absolute`. Count 3 static accesses. Sites: `movea.l` at 0xffe5a194, `movea.l` at 0xffe5a1da, `movea.l` at 0xffe5a1e6.
  Byte load, pattern 6: target `sp +0xb`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe5a19a.
  Byte store, pattern 7: target `[0x30000490] +0x1f0`, raw base `a0`, value `unknown`. Count 2 static accesses. Sites: `move.b` at 0xffe5a19e, `move.b` at 0xffe5a1f4.
  Four-byte store, pattern 8: target `sp -0x10`, raw base `a7`, value `0x1`. Count 2 static accesses. Sites: `pea.l` at 0xffe5a1a2, `pea.l` at 0xffe5a1f8.
  Four-byte store, pattern 9: target `sp -0x14`, raw base `a7`, value `0xffe5a644`. Count 1 static access. Sites: `pea.l` at 0xffe5a1ba.
  Byte store, pattern 10: target `[0x30000490] +0x1e8`, raw base `a0`, value `0xb4`. Count 1 static access. Sites: `move.b` at 0xffe5a1e2.
  Four-byte store, pattern 11: target `sp -0x14`, raw base `a7`, value `0xffe5a680`. Count 1 static access. Sites: `pea.l` at 0xffe5a210.
  Two-byte load, pattern 12: target `sp +0x12`, raw base `a6`. Count 1 static access. Sites: `move.w` at 0xffe5a230.
  Four-byte store, pattern 13: target `sp -0x10`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe5a234.
  Four-byte load, pattern 14: target `sp +0xc`, raw base `a6`. Count 1 static access. Sites: `move.l` at 0xffe5a236.
  Four-byte store, pattern 15: target `sp -0x14`, raw base `a7`, value `[sp+12]`. Count 1 static access. Sites: `move.l` at 0xffe5a236.
  Call (`bsr.w` at 0xffe5a164): (unknown callee), target 0xffe5a3a4. Candidate slots: stack arg0=`unknown`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5a184): (unknown callee), target 0xffe69a28. Candidate slots: stack arg0=`0x30006d78`, stack arg1=`0xffe5a610`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a1a6): (unknown callee), target 0xffe5a2d0. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5a1c6): (unknown callee), target 0xffe69a28. Candidate slots: stack arg0=`0x30006d78`, stack arg1=`0xffe5a644`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a1d0): (unknown callee), target 0xffe5a3f8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a1fc): (unknown callee), target 0xffe5a2d0. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5a21c): (unknown callee), target 0xffe69a28. Candidate slots: stack arg0=`0x30006d78`, stack arg1=`0xffe5a680`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a226): (unknown callee), target 0xffe5a3f8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a23a): (unknown callee), target 0xffe5a40c. Candidate slots: stack arg0=`[sp+12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe5a242): (unknown callee), target 0xffe5a3f8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5a16e): target 0xffe5a194. Taken when `ret(0xffe5a3a4) == 0x0`. Falls through when `ret(0xffe5a3a4) != 0x0`.
  Jump (`bra.w` at 0xffe5a190): target 0xffe5a248. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5a1b0): target 0xffe5a1da. Taken when `ret(0xffe5a2d0) == 0x0`. Falls through when `ret(0xffe5a2d0) != 0x0`.
  Jump (`bra.w` at 0xffe5a1d6): target 0xffe5a248. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe5a206): target 0xffe5a22e. Taken when `ret(0xffe5a2d0) == 0x0`. Falls through when `ret(0xffe5a2d0) != 0x0`.
  Jump (`bra.b` at 0xffe5a22c): target 0xffe5a248. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe5a250), d0=`unknown`.
  Register fact (`moveq` at 0xffe5a1e0): d0=`0xffffffb4`.
  Register fact (`moveq` at 0xffe5a22e): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe4bee8`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4bee8 to 0xffe4c04e (exclusive), 358 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `cf168f5c371997cb8c3f42833d58b42d4bdb7236932788cbcdee274323b02253`.
  Four-byte load, pattern 1: target `abs 0x3000052c`, raw base `absolute`. Count 18 static accesses. Sites: `movea.l` at 0xffe4bef6, `movea.l` at 0xffe4bf0c, `movea.l` at 0xffe4bf16.
  Four-byte load, pattern 2: target `[0x3000052c] +0xf0`, raw base `a0`. Count 5 static accesses. Sites: `movea.l` at 0xffe4befc, `movea.l` at 0xffe4bf28, `movea.l` at 0xffe4bf82.
  Four-byte load, pattern 3: target `abs 0x3000052c`, raw base `absolute`. Count 5 static accesses. Sites: `movea.l` at 0xffe4bf00, `movea.l` at 0xffe4bf2c, `movea.l` at 0xffe4bf86.
  Four-byte load, pattern 4: target `[0x3000052c] +0xec`, raw base `a1`. Count 5 static accesses. Sites: `cmpa.l` at 0xffe4bf06, `cmpa.l` at 0xffe4bf32, `cmpa.l` at 0xffe4bf8c.
  Four-byte load, pattern 5: target `[0x3000052c] +0xf0`, raw base `a0`. Count 2 static accesses. Sites: `move.l` at 0xffe4bf12, `move.l` at 0xffe4bf3e.
  Four-byte load, pattern 6: target `[0x3000052c] +0xec`, raw base `a0`. Count 2 static accesses. Sites: `sub.l` at 0xffe4bf1c, `sub.l` at 0xffe4c004.
  Four-byte load, pattern 7: target `[0x3000052c] +0xe4`, raw base `a0`. Count 1 static access. Sites: `sub.l` at 0xffe4bf48.
  Four-byte load, pattern 8: target `[0x3000052c] +0xe8`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe4bf52.
  Four-byte load, pattern 9: target `[0x3000052c] +0xec`, raw base `a0`. Count 1 static access. Sites: `sub.l` at 0xffe4bf5c.
  Four-byte store, pattern 10: target `sp -0x14`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `pea.l` at 0xffe4bf64.
  Byte store, pattern 11: target `ret(0xffe4ea20) +0x0`, raw base `a5`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe4bf92.
  Four-byte store, pattern 12: target `sp -0x14`, raw base `a7`, value `unknown`. Count 2 static accesses. Sites: `move.l` at 0xffe4bfae, `move.l` at 0xffe4c008.
  Four-byte load, pattern 13: target `[0x3000052c] +0xec`, raw base `a0`. Count 2 static accesses. Sites: `move.l` at 0xffe4bfb6, `move.l` at 0xffe4c010.
  Four-byte store, pattern 14: target `sp -0x18`, raw base `a7`, value `[[0x3000052c]+236]`. Count 2 static accesses. Sites: `move.l` at 0xffe4bfb6, `move.l` at 0xffe4c010.
  Four-byte store, pattern 15: target `sp -0x1c`, raw base `a7`, value `ret(0xffe4ea20)`. Count 2 static accesses. Sites: `move.l` at 0xffe4bfba, `move.l` at 0xffe4c014.
  Four-byte load, pattern 16: target `[0x3000052c] +0xf0`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe4bfe6.
  Four-byte load, pattern 17: target `[0x3000052c] +0xe4`, raw base `a0`. Count 1 static access. Sites: `sub.l` at 0xffe4bff0.
  Four-byte load, pattern 18: target `[0x3000052c] +0xe8`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe4bffa.
  Four-byte store, pattern 19: target `sp -0x14`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe4c020.
  Four-byte load, pattern 20: target `[0x3000052c] +0xe4`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe4c028.
  Four-byte store, pattern 21: target `sp -0x18`, raw base `a7`, value `[[0x3000052c]+228]`. Count 1 static access. Sites: `move.l` at 0xffe4c028.
  Four-byte store, pattern 22: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe4c030.
  Byte store, pattern 23: target `unknown`, raw base `a5`, value `0x0`. Count 1 static access. Sites: `clr.b` at 0xffe4c03e.
  Call (`jsr` at 0xffe4bf68): (unknown callee), target 0xffe4ea20. Candidate slots: stack arg0=`unknown`, stack arg1=`d6`, stack arg2=`d7`, stack arg3=`a5`, stack arg4=`a6`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4bfbc): (unknown callee), target 0xffe6c074. Candidate slots: stack arg0=`ret(0xffe4ea20)`, stack arg1=`[[0x3000052c]+236]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4c016): (unknown callee), target 0xffe6c074. Candidate slots: stack arg0=`ret(0xffe4ea20)`, stack arg1=`[[0x3000052c]+236]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4c032): (unknown callee), target 0xffe6c074. Candidate slots: stack arg0=`unknown`, stack arg1=`[[0x3000052c]+228]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bls.b` at 0xffe4bf0a): target 0xffe4bf22. Taken when `[[0x3000052c]+240] <= [[0x3000052c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] > [[0x3000052c]+236] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe4bf20): target 0xffe4bf62. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe4bf36): target 0xffe4bf62. Taken when `[[0x3000052c]+240] >= [[0x3000052c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] < [[0x3000052c]+236] (unsigned, 32-bit)`.
  Branch (`bne.b` at 0xffe4bf74): target 0xffe4bf7c. Taken when `ret(0xffe4ea20) != 0x0`. Falls through when `ret(0xffe4ea20) == 0x0`.
  Jump (`bra.w` at 0xffe4bf78): target 0xffe4c044. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe4bf90): target 0xffe4bf98. Taken when `[[0x3000052c]+240] != [[0x3000052c]+236]`. Falls through when `[[0x3000052c]+240] == [[0x3000052c]+236]`.
  Jump (`bra.w` at 0xffe4bf94): target 0xffe4c03e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bls.b` at 0xffe4bfac): target 0xffe4bfca. Taken when `[[0x3000052c]+240] <= [[0x3000052c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] > [[0x3000052c]+236] (unsigned, 32-bit)`.
  Jump (`bra.w` at 0xffe4bfc6): target 0xffe4c03e. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe4bfde): target 0xffe4c03e. Taken when `[[0x3000052c]+240] >= [[0x3000052c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] < [[0x3000052c]+236] (unsigned, 32-bit)`.
  At the exit (`rts` at 0xffe4c04c), d0=`unknown`.
  Register fact (`moveq` at 0xffe4bef4): d7=`0x1`.
  Register fact (`moveq` at 0xffe4bf76): d0=`0x0`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.

- `0xffe10510`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe10510 to 0xffe106d0 (exclusive), 448 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `d8a37fb3c8b578fb248338cb731898de005863fc7df92af53208fdf82199b8cf`.
  Four-byte load, pattern 1: target `abs 0xffe106d0`, raw base `a1`. Count 1 static access. Sites: `move.l` at 0xffe10526.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `a0`, value `[0xffe106d0]`. Count 1 static access. Sites: `move.l` at 0xffe10526.
  Byte load, pattern 3: target `abs 0xffe106d4`, raw base `a1`. Count 1 static access. Sites: `move.b` at 0xffe10528.
  Byte store, pattern 4: target `sp -0x8`, raw base `a0`, value `([0xffe106d4]:1 & 0xff)`. Count 1 static access. Sites: `move.b` at 0xffe10528.
  Four-byte store, pattern 5: target `sp -0x24`, raw base `a7`, value `0xffe106ec`. Count 1 static access. Sites: `pea.l` at 0xffe10536.
  Four-byte store, pattern 6: target `sp -0x24`, raw base `a7`, value `0x1`. Count 1 static access. Sites: `pea.l` at 0xffe1055c.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `0x2`. Count 1 static access. Sites: `pea.l` at 0xffe10572.
  Four-byte store, pattern 8: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe10584.
  Four-byte store, pattern 9: target `sp -0x24`, raw base `a7`, value `0xffe106fc`. Count 1 static access. Sites: `pea.l` at 0xffe105fe.
  Four-byte store, pattern 10: target `sp -0x28`, raw base `a7`, value `0x31`. Count 1 static access. Sites: `pea.l` at 0xffe10604.
  Four-byte store, pattern 11: target `sp -0x24`, raw base `a7`, value `0xffe1070c`. Count 1 static access. Sites: `pea.l` at 0xffe10610.
  Four-byte store, pattern 12: target `sp -0x28`, raw base `a7`, value `0x32`. Count 2 static accesses. Sites: `pea.l` at 0xffe10616, `pea.l` at 0xffe10644.
  Four-byte store, pattern 13: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe10642.
  Four-byte store, pattern 14: target `abs 0x301019a0`, raw base `absolute`, value `0x0`. Count 1 static access. Sites: `clr.l` at 0xffe10650.
  Four-byte load, pattern 15: target `abs 0x30000490`, raw base `absolute`. Count 4 static accesses. Sites: `movea.l` at 0xffe1065c, `movea.l` at 0xffe10668, `movea.l` at 0xffe10686.
  Byte store, pattern 16: target `[0x30000490] +0x43`, raw base `a0`, value `0x55`. Count 2 static accesses. Sites: `move.b` at 0xffe10664, `move.b` at 0xffe1068e.
  Byte store, pattern 17: target `[0x30000490] +0x43`, raw base `a0`, value `0xaa`. Count 2 static accesses. Sites: `move.b` at 0xffe10670, `move.b` at 0xffe1069a.
  Four-byte store, pattern 18: target `unknown`, raw base `a7`, value `0x14`. Count 1 static access. Sites: `pea.l` at 0xffe10674.
  Byte load, pattern 19: target `unknown`, raw base `a6`. Count 1 static access. Sites: `move.b` at 0xffe106aa.
  Four-byte store, pattern 20: target `unknown`, raw base `a7`, value `unknown`. Count 1 static access. Sites: `move.l` at 0xffe106b0.
  Four-byte store, pattern 21: target `unknown`, raw base `a7`, value `0xb`. Count 1 static access. Sites: `pea.l` at 0xffe106b2.
  Four-byte store, pattern 22: target `unknown`, raw base `a7`, value `0x31`. Count 1 static access. Sites: `pea.l` at 0xffe106b6.
  Call (`jsr` at 0xffe10530): (unknown callee), target 0xffe4a71c. Candidate slots: stack arg0=`d4`, stack arg1=`d5`, stack arg2=`d6`, stack arg3=`d7`, stack arg4=`a5`, stack arg5=`[0xffe106d0]`.
  Call (`jsr` at 0xffe1053c): (unknown callee), target 0xffe4bc84. Candidate slots: stack arg0=`0xffe106ec`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10544): (unknown callee), target 0xffe4de40. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe1054a): (unknown callee), target 0xffe49b3c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10550): (unknown callee), target 0xffe42134. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10556): (unknown callee), target 0xffe4c744. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10560): (unknown callee), target 0xffe4b7bc. Candidate slots: stack arg0=`0x1`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10576): (unknown callee), target 0xffe4b7bc. Candidate slots: stack arg0=`0x2`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10586): (unknown callee), target 0xffe67ee4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe1058e): (unknown callee), target 0xffe27bc0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10594): (unknown callee), target 0xffe16834. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe1059a): (unknown callee), target 0xffe39a70. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105a0): (unknown callee), target 0xffe393c4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105a6): (unknown callee), target 0xffe2b85c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105ac): (unknown callee), target 0xffe34654. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105b2): (unknown callee), target 0xffe2f53c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105b8): (unknown callee), target 0xffe29238. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105be): (unknown callee), target 0xffe38228. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105c4): (unknown callee), target 0xffe375f8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105ca): (unknown callee), target 0xffe2a9bc. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105d0): (unknown callee), target 0xffe38c90. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105d6): (unknown callee), target 0xffe36dec. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105dc): (unknown callee), target 0xffe5ef64. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105e2): (unknown callee), target 0xffe5a7c0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105e8): (unknown callee), target 0xffe26dcc. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe105ee): (unknown callee), target 0xffe4e1c4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10608): (unknown callee), target 0xffe4618c. Candidate slots: stack arg0=`0x31`, stack arg1=`0xffe106fc`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe1061a): (unknown callee), target 0xffe4618c. Candidate slots: stack arg0=`0x32`, stack arg1=`0xffe1070c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10624): (unknown callee), target 0xffe3694c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe1062a): (unknown callee), target 0xffe4c668. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10648): (unknown callee), target 0xffe4618c. Candidate slots: stack arg0=`0x32`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10656): (unknown callee), target 0xffe4c528. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe10678): (unknown callee), target 0xffe3eaf8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe106ba): (unknown callee), target 0xffe461a8. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe105fc): target 0xffe10624. Taken when `0x3f != ?`. Falls through when `0x3f == ?`.
  Jump (`bra.b` at 0xffe10622): target 0xffe10650. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe10632): target 0xffe1063c. Taken when `ret(0xffe4c668) == 0x0`. Falls through when `ret(0xffe4c668) != 0x0`.
  Jump (`bra.b` at 0xffe1063a): target 0xffe10642. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bgt.b` at 0xffe10684): target 0xffe1065c. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Jump (`bra.b` at 0xffe106c4): target 0xffe1065c. The local CFG lists a taken edge and no fallthrough edge.
  No reached exit is modeled.
  Register fact (`movea.l` at 0xffe10520): a1=`0xffe106d0`.
  Register fact (`move.l` at 0xffe10526): a1=`0xffe106d4`.
  Register fact (`move.b` at 0xffe10528): a1=`0xffe106d5`.
  Register fact (`moveq` at 0xffe1052a): d7=`0x4`.
  Register fact (`moveq` at 0xffe1052c): d6=`0x0`.
  Register fact (`moveq` at 0xffe1052e): d5=`0x2`.
  Register fact (`moveq` at 0xffe1057e): d1=`0x3f`.
  Register fact (`moveq` at 0xffe105f4): d1=`0x0`.
  Register fact (`moveq` at 0xffe105f8): d0=`0x3f`.
  Register fact (`movea.l` at 0xffe10634): a5=`0xffe1071c`.
  Register fact (`movea.l` at 0xffe1063c): a5=`0xffe10728`.
  Register fact (`moveq` at 0xffe10662): d0=`0x55`.
  Register fact (`moveq` at 0xffe1066e): d0=`0xffffffaa`.
  Register fact (`moveq` at 0xffe1068c): d0=`0x55`.
  Register fact (`moveq` at 0xffe10698): d0=`0xffffffaa`.
  Register fact (`moveq` at 0xffe1069e): d5=`0x2`.
  Tool warning: `0xffe106a6 divs.l: unsupported effects invalidated`.
  Tool warning: `0xffe106ae extb.l: unsupported effects invalidated`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.
  Local unreached instructions: 0xffe106c6 to 0xffe106ce, ending at 0xffe106d0 (exclusive). Count 3.

- `0xffe4bc84`, `6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe4bc84 to 0xffe4beb6 (exclusive), 562 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `21f5295c7c584dcaa962d86185cb5e0cb71dcc63905f35353e43b6045bb9fc4e`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access. Sites: `movea.l` at 0xffe4bc90.
  Four-byte store, pattern 2: target `sp -0x1c`, raw base `a7`, value `[sp+4]`. Count 1 static access. Sites: `move.l` at 0xffe4bc94.
  Four-byte load, pattern 3: target `abs 0x3000052c`, raw base `absolute`. Count 25 static accesses. Sites: `movea.l` at 0xffe4bca2, `movea.l` at 0xffe4bcac, `movea.l` at 0xffe4bcde.
  Four-byte load, pattern 4: target `[0x3000052c] +0xe8`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe4bca8.
  Four-byte load, pattern 5: target `[0x3000052c] +0xe4`, raw base `a0`. Count 2 static accesses. Sites: `sub.l` at 0xffe4bcb2, `sub.l` at 0xffe4bd04.
  Byte load, pattern 6: target `([sp+4] + ret(0xffe6c28c)) -0x1`, raw base `a5`. Count 1 static access. Sites: `move.b` at 0xffe4bcb6.
  Four-byte store, pattern 7: target `unknown`, raw base `a7`, value `0xffffffff`. Count 1 static access. Sites: `pea.l` at 0xffe4bcc4.
  Four-byte store, pattern 8: target `unknown`, raw base `a7`, value `0x30006070`. Count 2 static accesses. Sites: `pea.l` at 0xffe4bcc8, `pea.l` at 0xffe4be7e.
  Four-byte load, pattern 9: target `[0x3000052c] +0xf0`, raw base `a0`. Count 6 static accesses. Sites: `movea.l` at 0xffe4bce4, `movea.l` at 0xffe4bd26, `movea.l` at 0xffe4bdba.
  Four-byte load, pattern 10: target `abs 0x3000052c`, raw base `absolute`. Count 10 static accesses. Sites: `movea.l` at 0xffe4bce8, `movea.l` at 0xffe4bd2a, `movea.l` at 0xffe4bd7e.
  Four-byte load, pattern 11: target `[0x3000052c] +0xec`, raw base `a1`. Count 2 static accesses. Sites: `cmpa.l` at 0xffe4bcee, `cmpa.l` at 0xffe4bd30.
  Four-byte load, pattern 12: target `[0x3000052c] +0xec`, raw base `a0`. Count 2 static accesses. Sites: `move.l` at 0xffe4bcfa, `move.l` at 0xffe4bd3c.
  Four-byte load, pattern 13: target `[0x3000052c] +0xe8`, raw base `a0`. Count 1 static access. Sites: `move.l` at 0xffe4bd0e.
  Four-byte load, pattern 14: target `[0x3000052c] +0xf0`, raw base `a0`. Count 1 static access. Sites: `sub.l` at 0xffe4bd18.
  Four-byte load, pattern 15: target `[0x3000052c] +0xf0`, raw base `a0`. Count 1 static access. Sites: `sub.l` at 0xffe4bd46.
  Four-byte load, pattern 16: target `[0x3000052c] +0xec`, raw base `a0`. Count 2 static accesses. Sites: `movea.l` at 0xffe4bd5a, `movea.l` at 0xffe4bd7a.
  Byte load, pattern 17: target `[[0x3000052c]+236] +0x0`, raw base `a0`. Count 1 static access. Sites: `move.b` at 0xffe4bd5e.
  Four-byte load, pattern 18: target `[0x3000052c] +0xec`, raw base `a0`. Count 2 static accesses. Sites: `addq.l` at 0xffe4bd6e, `addq.l` at 0xffe4bda6.
  Four-byte store, pattern 19: target `[0x3000052c] +0xec`, raw base `a0`, value `[[0x3000052c]+236]+1`. Count 2 static accesses. Sites: `addq.l` at 0xffe4bd6e, `addq.l` at 0xffe4bda6.
  Four-byte load, pattern 20: target `[0x3000052c] +0xe8`, raw base `a1`. Count 3 static accesses. Sites: `cmpa.l` at 0xffe4bd84, `cmpa.l` at 0xffe4bdda, `cmpa.l` at 0xffe4be24.
  Four-byte load, pattern 21: target `[0x3000052c] +0xe4`, raw base `a1`. Count 3 static accesses. Sites: `move.l` at 0xffe4bd96, `move.l` at 0xffe4bdec, `move.l` at 0xffe4be36.
  Four-byte store, pattern 22: target `[0x3000052c] +0xec`, raw base `a0`, value `[[0x3000052c]+228]`. Count 1 static access. Sites: `move.l` at 0xffe4bd9a.
  Byte load, pattern 23: target `unknown`, raw base `a5`. Count 1 static access. Sites: `move.b` at 0xffe4bdbe.
  Byte store, pattern 24: target `[[0x3000052c]+240] +0x0`, raw base `a0`, value `unknown`. Count 1 static access. Sites: `move.b` at 0xffe4bdbe.
  Four-byte load, pattern 25: target `[0x3000052c] +0xf0`, raw base `a0`. Count 2 static accesses. Sites: `addq.l` at 0xffe4bdc6, `addq.l` at 0xffe4be10.
  Four-byte store, pattern 26: target `[0x3000052c] +0xf0`, raw base `a0`, value `[[0x3000052c]+240]+1`. Count 2 static accesses. Sites: `addq.l` at 0xffe4bdc6, `addq.l` at 0xffe4be10.
  Four-byte store, pattern 27: target `[0x3000052c] +0xf0`, raw base `a0`, value `[[0x3000052c]+228]`. Count 2 static accesses. Sites: `move.l` at 0xffe4bdf0, `move.l` at 0xffe4be3a.
  Byte store, pattern 28: target `[[0x3000052c]+240] +0x0`, raw base `a0`, value `0xa`. Count 1 static access. Sites: `move.b` at 0xffe4be08.
  Four-byte store, pattern 29: target `unknown`, raw base `a7`, value `0x24`. Count 1 static access. Sites: `pea.l` at 0xffe4be3e.
  Four-byte store, pattern 30: target `unknown`, raw base `a7`, value `[0x3000052c]+224`. Count 1 static access. Sites: `pea.l` at 0xffe4be48.
  Byte store, pattern 31: target `[0x3000052c] +0x103`, raw base `a0`, value `(ret(0xffe52a6c) & 0xff)`. Count 1 static access. Sites: `move.b` at 0xffe4be5a.
  Four-byte load, pattern 32: target `unknown`, raw base `a0`. Count 3 static accesses. Sites: `move.l` at 0xffe4be74, `move.l` at 0xffe4be76, `move.l` at 0xffe4be7c.
  Four-byte store, pattern 33: target `unknown`, raw base `a1`, value `unknown`. Count 3 static accesses. Sites: `move.l` at 0xffe4be74, `move.l` at 0xffe4be76, `move.l` at 0xffe4be7c.
  Four-byte store, pattern 34: target `unknown`, raw base `a7`, value `ret(0xffe41448)`. Count 1 static access. Sites: `move.l` at 0xffe4be92.
  Four-byte store, pattern 35: target `unknown`, raw base `a7`, value `0x1`. Count 1 static access. Sites: `pea.l` at 0xffe4be94.
  Four-byte store, pattern 36: target `unknown`, raw base `a7`, value `0x367`. Count 1 static access. Sites: `pea.l` at 0xffe4be98.
  Four-byte store, pattern 37: target `unknown`, raw base `a7`, value `0xffe4c104`. Count 1 static access. Sites: `pea.l` at 0xffe4be9c.
  Call (`jsr` at 0xffe4bc96): (unknown callee), target 0xffe6c28c. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`d4`, stack arg2=`d5`, stack arg3=`d6`, stack arg4=`d7`, stack arg5=`a5`.
  Call (`jsr` at 0xffe4bcce): (unknown callee), target 0xffe413f0. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4be4c): (unknown callee), target 0xffe52a6c. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4be84): (unknown callee), target 0xffe41448. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4bea2): (unknown callee), target 0xffe4c250. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe4bcc0): target 0xffe4bcc4. Taken when `0xa == ?`. Falls through when `0xa != ?`.
  Branch (`bne.w` at 0xffe4bcda): target 0xffe4beac. Taken when `ret(0xffe413f0) != 0x0`. Falls through when `ret(0xffe413f0) == 0x0`.
  Branch (`bls.b` at 0xffe4bcf2): target 0xffe4bd20. Taken when `[[0x3000052c]+240] <= [[0x3000052c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] > [[0x3000052c]+236] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe4bd1e): target 0xffe4bd4a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bcc.b` at 0xffe4bd34): target 0xffe4bd4a. Taken when `[[0x3000052c]+240] >= [[0x3000052c]+236] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] < [[0x3000052c]+236] (unsigned, 32-bit)`.
  Branch (`bgt.b` at 0xffe4bd52): target 0xffe4bdae. Taken when `? > ? (signed, 32-bit)`. Falls through when `? <= ? (signed, 32-bit)`.
  Branch (`beq.b` at 0xffe4bd66): target 0xffe4bda0. Taken when `0xa == ?`. Falls through when `0xa != ?`.
  Branch (`bcs.b` at 0xffe4bd88): target 0xffe4bd54. Taken when `[[0x3000052c]+236] < [[0x3000052c]+232] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+236] >= [[0x3000052c]+232] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe4bd9e): target 0xffe4bd54. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe4bdac): target 0xffe4bd4a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe4bdb0): target 0xffe4bdf6. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bcs.b` at 0xffe4bdde): target 0xffe4bdae. Taken when `[[0x3000052c]+240] < [[0x3000052c]+232] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] >= [[0x3000052c]+232] (unsigned, 32-bit)`.
  Jump (`bra.b` at 0xffe4bdf4): target 0xffe4bdae. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe4bdfa): target 0xffe4be3e. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`bcs.b` at 0xffe4be28): target 0xffe4be3e. Taken when `[[0x3000052c]+240] < [[0x3000052c]+232] (unsigned, 32-bit)`. Falls through when `[[0x3000052c]+240] >= [[0x3000052c]+232] (unsigned, 32-bit)`.
  Branch (`bgt.b` at 0xffe4be7a): target 0xffe4be74. Taken when `gt flags condition (operands unknown)`. Falls through when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe4be90): target 0xffe4beac. Taken when `ret(0xffe41448) == 0x0`. Falls through when `ret(0xffe41448) != 0x0`.
  At the exit (`rts` at 0xffe4beb4), d0=`unknown`.
  Register fact (`moveq` at 0xffe4bca0): d5=`0x0`.
  Register fact (`moveq` at 0xffe4bcbc): d1=`0xa`.
  Register fact (`moveq` at 0xffe4bcc2): d5=`0x1`.
  Register fact (`moveq` at 0xffe4bd62): d1=`0xa`.
  Register fact (`moveq` at 0xffe4be06): d0=`0xa`.
  Register fact (`moveq` at 0xffe4be72): d0=`0x4`.
  Tool warning: `0xffe4bcba extb.l: unsupported effects invalidated`.
  Tool warning: `0xffe4bd4c extb.l: unsupported effects invalidated`.
  Tool warning: `0xffe4bd60 extb.l: unsupported effects invalidated`.
  Tool warning: `ColdFire uses the 68040 decoder for the common ISA subset; unsupported opcodes are unknown`.
  Caller-hint context at 0xffe2b7a6 stopped decoding early. Bytes 0xffe2b7b4 to 0xffe2b7be (exclusive) remain raw and open.
  Caller-hint context at 0xffe2df16 stopped decoding early. Bytes 0xffe2df2c to 0xffe2df2e (exclusive) remain raw and open.
  Caller-hint context at 0xffe38166 stopped decoding early. Bytes 0xffe38174 to 0xffe3817e (exclusive) remain raw and open.
  Caller-hint context at 0xffe38c0a stopped decoding early. Bytes 0xffe38c18 to 0xffe38c22 (exclusive) remain raw and open.
