# ColdFire L1 pointer update windows

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

The [full facts](disasm/coldfire-variants/pointer-update-windows-facts.txt) retain all instructions, constants and warnings.

- `0xffe70de0`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe70de0 to 0xffe70e7a (exclusive), 154 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `ea9e18759ce37a87b8937f6eb273e3736b9e4566501f8e9132b25ac3d7491cd6`.
  Four-byte load, pattern 1: target `abs 0x30000040`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe70dec.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `a7`, value `0xffe71ea2`. Count 1 static access, including `pea.l` at 0xffe70df0.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `a7`, value `0xffe71b88`. Count 1 static access, including `pea.l` at 0xffe70dfe.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `a7`, value `0xffe71ca0`. Count 1 static access, including `pea.l` at 0xffe70e04.
  Four-byte load, pattern 5: target `abs 0x300002d4`, raw base `a1`. Count 7 static accesses, including `movea.l` at 0xffe70e18, 0xffe70e28, 0xffe70e30.
  Two-byte load, pattern 6: target `[0x300002d4] +0xca`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe70e1a.
  Two-byte store, pattern 7: target `[0x300002d4] +0xca`, raw base `a0`, value `unknown`. Count 1 static access, including `move.w` at 0xffe70e24.
  Byte store, pattern 8: target `[0x300002d4] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access, including `move.b` at 0xffe70e2c.
  Byte store, pattern 9: target `[0x300002d4] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access, including `move.b` at 0xffe70e34.
  Byte store, pattern 10: target `[0x300002d4] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses, including `move.b` at 0xffe70e3c, 0xffe70e4a.
  Byte store, pattern 11: target `[0x300002d4] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access, including `move.b` at 0xffe70e44.
  Byte store, pattern 12: target `[0x300002d4] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access, including `move.b` at 0xffe70e52.
  Four-byte load, pattern 13: target `abs 0x300002d4`, raw base `a1`. Count 1 static access, including `add.l` at 0xffe70e5c.
  Four-byte store, pattern 14: target `abs 0x30000048`, raw base `a5`, value `[0x300002d4]+457`. Count 1 static access, including `move.l` at 0xffe70e5e.
  Four-byte load, pattern 15: target `abs 0x3000004c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe70e62.
  Four-byte store, pattern 16: target `abs 0x30000044`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe70e6c.
  Call (`jsr` at 0xffe70df6): (unknown callee), target 0xffe78864. Candidate slots: stack arg0=`0xffe71ea2`, stack arg1=`a5`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe70e0a): (unknown callee), target 0xffe74558. Candidate slots: stack arg0=`0xffe71ca0`, stack arg1=`0xffe71b88`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe70e68): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe70dee): target 0xffe70dfe. Taken when `[0x30000040] == 0x0`. Falls through when `[0x30000040] != 0x0`.
  At the exit (`rts` at 0xffe70e78), d0=`0x1`.

- `0xffe82228`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe82228 to 0xffe822c2 (exclusive), 154 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `fd5d2cda83b580ff5ab145f02017cdf15754706e046ead480d983cdaebbbf3b7`.
  Four-byte load, pattern 1: target `abs 0x30000040`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe82234.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `a7`, value `0xffe832ea`. Count 1 static access, including `pea.l` at 0xffe82238.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `a7`, value `0xffe82fd0`. Count 1 static access, including `pea.l` at 0xffe82246.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `a7`, value `0xffe830e8`. Count 1 static access, including `pea.l` at 0xffe8224c.
  Four-byte load, pattern 5: target `abs 0x30000370`, raw base `a1`. Count 7 static accesses, including `movea.l` at 0xffe82260, 0xffe82270, 0xffe82278.
  Two-byte load, pattern 6: target `[0x30000370] +0xca`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe82262.
  Two-byte store, pattern 7: target `[0x30000370] +0xca`, raw base `a0`, value `unknown`. Count 1 static access, including `move.w` at 0xffe8226c.
  Byte store, pattern 8: target `[0x30000370] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access, including `move.b` at 0xffe82274.
  Byte store, pattern 9: target `[0x30000370] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access, including `move.b` at 0xffe8227c.
  Byte store, pattern 10: target `[0x30000370] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses, including `move.b` at 0xffe82284, 0xffe82292.
  Byte store, pattern 11: target `[0x30000370] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access, including `move.b` at 0xffe8228c.
  Byte store, pattern 12: target `[0x30000370] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access, including `move.b` at 0xffe8229a.
  Four-byte load, pattern 13: target `abs 0x30000370`, raw base `a1`. Count 1 static access, including `add.l` at 0xffe822a4.
  Four-byte store, pattern 14: target `abs 0x30000048`, raw base `a5`, value `[0x30000370]+457`. Count 1 static access, including `move.l` at 0xffe822a6.
  Four-byte load, pattern 15: target `abs 0x3000004c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe822aa.
  Four-byte store, pattern 16: target `abs 0x30000044`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe822b4.
  Call (`jsr` at 0xffe8223e): (unknown callee), target 0xffe8c0ac. Candidate slots: stack arg0=`0xffe832ea`, stack arg1=`a5`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe82252): (unknown callee), target 0xffe859a0. Candidate slots: stack arg0=`0xffe830e8`, stack arg1=`0xffe82fd0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe822b0): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe82236): target 0xffe82246. Taken when `[0x30000040] == 0x0`. Falls through when `[0x30000040] != 0x0`.
  At the exit (`rts` at 0xffe822c0), d0=`0x1`.

- `0xffe6f748`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#0`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe6f748 to 0xffe6f7e2 (exclusive), 154 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0x0, 969844 bytes, SHA256 `f54e47c2935cd6cf448928e373ec7a24d3da1265b5f73adec5c3f26f426d278b`.
  Payload: carrier offset 0x40, 969780 bytes, SHA256 `703576f15e0bbcd5be107d70d238c5ded01c115e270ca1be2460b1531e9e24ae`.
  Read window: SHA256 `765b7674c6c28b1ceecb9319e4bb347c399485775bb6a5a430d3a8b969c6a057`.
  Four-byte load, pattern 1: target `abs 0x30000040`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe6f754.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `a7`, value `0xffe70702`. Count 1 static access, including `pea.l` at 0xffe6f758.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `a7`, value `0xffe703e8`. Count 1 static access, including `pea.l` at 0xffe6f766.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `a7`, value `0xffe70500`. Count 1 static access, including `pea.l` at 0xffe6f76c.
  Four-byte load, pattern 5: target `abs 0x3000029c`, raw base `a1`. Count 7 static accesses, including `movea.l` at 0xffe6f780, 0xffe6f790, 0xffe6f798.
  Two-byte load, pattern 6: target `[0x3000029c] +0xca`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe6f782.
  Two-byte store, pattern 7: target `[0x3000029c] +0xca`, raw base `a0`, value `unknown`. Count 1 static access, including `move.w` at 0xffe6f78c.
  Byte store, pattern 8: target `[0x3000029c] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access, including `move.b` at 0xffe6f794.
  Byte store, pattern 9: target `[0x3000029c] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access, including `move.b` at 0xffe6f79c.
  Byte store, pattern 10: target `[0x3000029c] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses, including `move.b` at 0xffe6f7a4, 0xffe6f7b2.
  Byte store, pattern 11: target `[0x3000029c] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access, including `move.b` at 0xffe6f7ac.
  Byte store, pattern 12: target `[0x3000029c] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access, including `move.b` at 0xffe6f7ba.
  Four-byte load, pattern 13: target `abs 0x3000029c`, raw base `a1`. Count 1 static access, including `add.l` at 0xffe6f7c4.
  Four-byte store, pattern 14: target `abs 0x30000048`, raw base `a5`, value `[0x3000029c]+457`. Count 1 static access, including `move.l` at 0xffe6f7c6.
  Four-byte load, pattern 15: target `abs 0x3000004c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe6f7ca.
  Four-byte store, pattern 16: target `abs 0x30000044`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe6f7d4.
  Call (`jsr` at 0xffe6f75e): (unknown callee), target 0xffe76bf0. Candidate slots: stack arg0=`0xffe70702`, stack arg1=`a5`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f772): (unknown callee), target 0xffe72d10. Candidate slots: stack arg0=`0xffe70500`, stack arg1=`0xffe703e8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6f7d0): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6f756): target 0xffe6f766. Taken when `[0x30000040] == 0x0`. Falls through when `[0x30000040] != 0x0`.
  At the exit (`rts` at 0xffe6f7e0), d0=`0x1`.

- `0xffe83bec`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe83bec to 0xffe83c86 (exclusive), 154 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `5327051010df33148a45f5f9e1599beb9bf51c36b49fd33cda9caf16c66317ce`.
  Four-byte load, pattern 1: target `abs 0x30000040`, raw base `a5`. Count 1 static access, including `tst.l` at 0xffe83bf8.
  Four-byte store, pattern 2: target `sp -0xc`, raw base `a7`, value `0xffe84ba6`. Count 1 static access, including `pea.l` at 0xffe83bfc.
  Four-byte store, pattern 3: target `sp -0xc`, raw base `a7`, value `0xffe8488c`. Count 1 static access, including `pea.l` at 0xffe83c0a.
  Four-byte store, pattern 4: target `sp -0x10`, raw base `a7`, value `0xffe849a4`. Count 1 static access, including `pea.l` at 0xffe83c10.
  Four-byte load, pattern 5: target `abs 0x30000340`, raw base `a1`. Count 7 static accesses, including `movea.l` at 0xffe83c24, 0xffe83c34, 0xffe83c3c.
  Two-byte load, pattern 6: target `[0x30000340] +0xca`, raw base `a0`. Count 1 static access, including `move.w` at 0xffe83c26.
  Two-byte store, pattern 7: target `[0x30000340] +0xca`, raw base `a0`, value `unknown`. Count 1 static access, including `move.w` at 0xffe83c30.
  Byte store, pattern 8: target `[0x30000340] +0x1c9`, raw base `a0`, value `0x31`. Count 1 static access, including `move.b` at 0xffe83c38.
  Byte store, pattern 9: target `[0x30000340] +0x1c5`, raw base `a0`, value `0xfe`. Count 1 static access, including `move.b` at 0xffe83c40.
  Byte store, pattern 10: target `[0x30000340] +0x1c9`, raw base `a0`, value `0x38`. Count 2 static accesses, including `move.b` at 0xffe83c48, 0xffe83c56.
  Byte store, pattern 11: target `[0x30000340] +0x1c9`, raw base `a0`, value `0x28`. Count 1 static access, including `move.b` at 0xffe83c50.
  Byte store, pattern 12: target `[0x30000340] +0x1c9`, raw base `a0`, value `0x30`. Count 1 static access, including `move.b` at 0xffe83c5e.
  Four-byte load, pattern 13: target `abs 0x30000340`, raw base `a1`. Count 1 static access, including `add.l` at 0xffe83c68.
  Four-byte store, pattern 14: target `abs 0x30000048`, raw base `a5`, value `[0x30000340]+457`. Count 1 static access, including `move.l` at 0xffe83c6a.
  Four-byte load, pattern 15: target `abs 0x3000004c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe83c6e.
  Four-byte store, pattern 16: target `abs 0x30000044`, raw base `a5`, value `0x1`. Count 1 static access, including `move.l` at 0xffe83c78.
  Call (`jsr` at 0xffe83c02): (unknown callee), target 0xffe8f1d0. Candidate slots: stack arg0=`0xffe84ba6`, stack arg1=`a5`, stack arg2=`a6`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe83c16): (unknown callee), target 0xffe871b4. Candidate slots: stack arg0=`0xffe849a4`, stack arg1=`0xffe8488c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe83c74): (unknown callee), target unknown. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe83bfa): target 0xffe83c0a. Taken when `[0x30000040] == 0x0`. Falls through when `[0x30000040] != 0x0`.
  At the exit (`rts` at 0xffe83c84), d0=`0x1`.

- `0xffe77500`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe77500 to 0xffe7761c (exclusive), 284 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `18ad1e1beaed29c6f57b3ae85acf9796a6af4286b40d140eb7ba996ec1dc0e44`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe77508.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 2 static accesses, including `movea.l` at 0xffe7750c, 0xffe7757a.
  Byte load, pattern 3: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe77512.
  Four-byte load, pattern 4: target `abs 0x30000080`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7751a.
  Four-byte load, pattern 5: target `[0x30000080] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe77520.
  Four-byte load, pattern 6: target `abs 0x3000007c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe77524.
  Four-byte load, pattern 7: target `[0x3000007c] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe7752a.
  Four-byte store, pattern 8: target `sp -0x34`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe7752c.
  Four-byte store, pattern 9: target `sp -0x30`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe77530.
  Four-byte store, pattern 10: target `sp -0x40`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe77536.
  Four-byte store, pattern 11: target `sp -0x44`, raw base `a7`, value `sp-52`. Count 1 static access, including `pea.l` at 0xffe77538.
  Four-byte store, pattern 12: target `sp -0x48`, raw base `a7`, value `0xffe77048`. Count 1 static access, including `pea.l` at 0xffe7753c.
  Four-byte store, pattern 13: target `sp -0x4c`, raw base `a7`, value `0xffe76fc4`. Count 1 static access, including `pea.l` at 0xffe77542.
  Four-byte store, pattern 14: target `sp -0x50`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe77548.
  Four-byte store, pattern 15: target `sp -0x54`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe7754c.
  Four-byte store, pattern 16: target `sp -0x58`, raw base `a7`, value `sp+8`. Count 1 static access, including `pea.l` at 0xffe7754e.
  Four-byte store, pattern 17: target `sp -0x5c`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe77552.
  Byte load, pattern 18: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `tst.b` at 0xffe7757e.
  Four-byte store, pattern 19: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe77592.
  Four-byte store, pattern 20: target `sp -0x44`, raw base `a7`, value `[sp+4]+16`. Count 1 static access, including `pea.l` at 0xffe77594.
  Four-byte store, pattern 21: target `sp -0x48`, raw base `a7`, value `0xffe78182`. Count 1 static access, including `pea.l` at 0xffe77598.
  Four-byte store, pattern 22: target `sp -0x4c`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe7759e.
  Four-byte store, pattern 23: target `sp -0x40`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe775ac.
  Four-byte load, pattern 24: target `sp -0x30`, raw base `a6`. Count 1 static access, including `tst.l` at 0xffe775c0.
  Four-byte load, pattern 25: target `sp -0x30`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe775c6.
  Byte load, pattern 26: target `[sp-48] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe775cc.
  Four-byte store, pattern 27: target `sp -0x40`, raw base `a7`, value `0xffe781c0`. Count 1 static access, including `pea.l` at 0xffe775e6.
  Four-byte store, pattern 28: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe775fc.
  Four-byte load, pattern 29: target `sp -0x30`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe77606.
  Four-byte store, pattern 30: target `sp -0x40`, raw base `a7`, value `[sp-48]`. Count 1 static access, including `move.l` at 0xffe77606.
  Call (`jsr` at 0xffe77554): (unknown callee), target 0xffe4544c. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`sp+8`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`0xffe76fc4`, stack arg5=`0xffe77048`.
  Call (`jsr` at 0xffe775a2): (unknown callee), target 0xffe78c44. Candidate slots: stack arg0=`sp-44`, stack arg1=`0xffe78182`, stack arg2=`[sp+4]+16`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe775b0): (unknown callee), target 0xffe57118. Candidate slots: stack arg0=`sp-44`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe775ec): (unknown callee), target 0xffe57118. Candidate slots: stack arg0=`0xffe781c0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe775fe): (unknown callee), target 0xffe57118. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe7760a): (unknown callee), target 0xffe76e08. Candidate slots: stack arg0=`[sp-48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe77518): target 0xffe77524. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe77522): target 0xffe7752c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.w` at 0xffe77562): target 0xffe775c0. Taken when `ret(0xffe4544c) == 0x0`. Falls through when `ret(0xffe4544c) != 0x0`.
  Branch (`beq.b` at 0xffe7756c): target 0xffe77574. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe77570): target 0xffe77574. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe77572): target 0xffe775ba. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe77582): target 0xffe7758c. Taken when `([[sp+8]+22]:1 & 0xff) != 0x0`. Falls through when `([[sp+8]+22]:1 & 0xff) == 0x0`.
  Jump (`bra.b` at 0xffe7758a): target 0xffe77592. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe775b8): target 0xffe775c0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe775c4): target 0xffe77610. Taken when `[sp-48] == 0x0`. Falls through when `[sp-48] != 0x0`.
  Branch (`beq.b` at 0xffe775d2): target 0xffe775de. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`beq.b` at 0xffe775d6): target 0xffe775e6. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe775da): target 0xffe775f6. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe775dc): target 0xffe77606. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe775e4): target 0xffe775fc. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe775f4): target 0xffe77606. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe7761a), d0=`unknown`.

- `0xffe8ad10`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe8ad10 to 0xffe8ae2c (exclusive), 284 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `41215eea34bd2bb7470a793e8be85009c273042b1ce9244d76b9a472889db570`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe8ad18.
  Four-byte load, pattern 2: target `sp +0x8`, raw base `a6`. Count 2 static accesses, including `movea.l` at 0xffe8ad1c, 0xffe8ad8a.
  Byte load, pattern 3: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe8ad22.
  Four-byte load, pattern 4: target `abs 0x30000080`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe8ad2a.
  Four-byte load, pattern 5: target `[0x30000080] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe8ad30.
  Four-byte load, pattern 6: target `abs 0x3000007c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe8ad34.
  Four-byte load, pattern 7: target `[0x3000007c] +0x0`, raw base `a0`. Count 1 static access, including `move.l` at 0xffe8ad3a.
  Four-byte store, pattern 8: target `sp -0x34`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe8ad3c.
  Four-byte store, pattern 9: target `sp -0x30`, raw base `a6`, value `0x0`. Count 1 static access, including `clr.l` at 0xffe8ad40.
  Four-byte store, pattern 10: target `sp -0x40`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe8ad46.
  Four-byte store, pattern 11: target `sp -0x44`, raw base `a7`, value `sp-52`. Count 1 static access, including `pea.l` at 0xffe8ad48.
  Four-byte store, pattern 12: target `sp -0x48`, raw base `a7`, value `0xffe8a858`. Count 1 static access, including `pea.l` at 0xffe8ad4c.
  Four-byte store, pattern 13: target `sp -0x4c`, raw base `a7`, value `0xffe8a7d4`. Count 1 static access, including `pea.l` at 0xffe8ad52.
  Four-byte store, pattern 14: target `sp -0x50`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ad58.
  Four-byte store, pattern 15: target `sp -0x54`, raw base `a7`, value `0x1`. Count 1 static access, including `move.l` at 0xffe8ad5c.
  Four-byte store, pattern 16: target `sp -0x58`, raw base `a7`, value `sp+8`. Count 1 static access, including `pea.l` at 0xffe8ad5e.
  Four-byte store, pattern 17: target `sp -0x5c`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe8ad62.
  Byte load, pattern 18: target `[sp+8] +0x16`, raw base `a0`. Count 1 static access, including `tst.b` at 0xffe8ad8e.
  Four-byte store, pattern 19: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ada2.
  Four-byte store, pattern 20: target `sp -0x44`, raw base `a7`, value `[sp+4]+16`. Count 1 static access, including `pea.l` at 0xffe8ada4.
  Four-byte store, pattern 21: target `sp -0x48`, raw base `a7`, value `0xffe8b992`. Count 1 static access, including `pea.l` at 0xffe8ada8.
  Four-byte store, pattern 22: target `sp -0x4c`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe8adae.
  Four-byte store, pattern 23: target `sp -0x40`, raw base `a7`, value `sp-44`. Count 1 static access, including `pea.l` at 0xffe8adbc.
  Four-byte load, pattern 24: target `sp -0x30`, raw base `a6`. Count 1 static access, including `tst.l` at 0xffe8add0.
  Four-byte load, pattern 25: target `sp -0x30`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe8add6.
  Byte load, pattern 26: target `[sp-48] +0x16`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe8addc.
  Four-byte store, pattern 27: target `sp -0x40`, raw base `a7`, value `0xffe8b9d0`. Count 1 static access, including `pea.l` at 0xffe8adf6.
  Four-byte store, pattern 28: target `sp -0x40`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8ae0c.
  Four-byte load, pattern 29: target `sp -0x30`, raw base `a6`. Count 1 static access, including `move.l` at 0xffe8ae16.
  Four-byte store, pattern 30: target `sp -0x40`, raw base `a7`, value `[sp-48]`. Count 1 static access, including `move.l` at 0xffe8ae16.
  Call (`jsr` at 0xffe8ad64): (unknown callee), target 0xffe4c630. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`sp+8`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`0xffe8a7d4`, stack arg5=`0xffe8a858`.
  Call (`jsr` at 0xffe8adb2): (unknown callee), target 0xffe8c48c. Candidate slots: stack arg0=`sp-44`, stack arg1=`0xffe8b992`, stack arg2=`[sp+4]+16`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8adc0): (unknown callee), target 0xffe63cb4. Candidate slots: stack arg0=`sp-44`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8adfc): (unknown callee), target 0xffe63cb4. Candidate slots: stack arg0=`0xffe8b9d0`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe8ae0e): (unknown callee), target 0xffe63cb4. Candidate slots: stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`bsr.w` at 0xffe8ae1a): (unknown callee), target 0xffe8a618. Candidate slots: stack arg0=`[sp-48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe8ad28): target 0xffe8ad34. Taken when `Z == 0`. Falls through when `not (Z == 0)`.
  Jump (`bra.b` at 0xffe8ad32): target 0xffe8ad3c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.w` at 0xffe8ad72): target 0xffe8add0. Taken when `ret(0xffe4c630) == 0x0`. Falls through when `ret(0xffe4c630) != 0x0`.
  Branch (`beq.b` at 0xffe8ad7c): target 0xffe8ad84. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe8ad80): target 0xffe8ad84. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe8ad82): target 0xffe8adca. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bne.b` at 0xffe8ad92): target 0xffe8ad9c. Taken when `([[sp+8]+22]:1 & 0xff) != 0x0`. Falls through when `([[sp+8]+22]:1 & 0xff) == 0x0`.
  Jump (`bra.b` at 0xffe8ad9a): target 0xffe8ada2. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe8adc8): target 0xffe8add0. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8add4): target 0xffe8ae20. Taken when `[sp-48] == 0x0`. Falls through when `[sp-48] != 0x0`.
  Branch (`beq.b` at 0xffe8ade2): target 0xffe8adee. Taken when `? == 0x0`. Falls through when `? != 0x0`.
  Branch (`beq.b` at 0xffe8ade6): target 0xffe8adf6. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe8adea): target 0xffe8ae06. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe8adec): target 0xffe8ae16. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe8adf4): target 0xffe8ae0c. The local CFG lists a taken edge and no fallthrough edge.
  Jump (`bra.b` at 0xffe8ae04): target 0xffe8ae16. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe8ae2a), d0=`unknown`.

- `0xffe71afc`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe71afc to 0xffe71b86 (exclusive), 138 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `8e724864b55a5d64e0effbea4dce9620642d572d382bc4e3ff9b41efde7f1224`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe71b08.
  Four-byte load, pattern 2: target `abs 0x300002d4`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe71b0c.
  Byte load, pattern 3: target `[0x300002d4] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe71b14.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71b5e.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71b60.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71b62.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71b64.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71b66.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe71b68.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe71fca`. Count 1 static access, including `pea.l` at 0xffe71b6a.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe71b70.
  Call (`jsr` at 0xffe71b72): (unknown callee), target 0xffe78c44. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe71fca`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe71b1c): target 0xffe71b22. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe71b20): target 0xffe71b24. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71b28): target 0xffe71b2e. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe71b2c): target 0xffe71b30. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71b34): target 0xffe71b3a. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe71b38): target 0xffe71b3c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71b40): target 0xffe71b46. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe71b44): target 0xffe71b48. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe71b4a): target 0xffe71b50. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe71b4e): target 0xffe71b52. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe71b56): target 0xffe71b5c. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe71b5a): target 0xffe71b5e. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe71b84), d0=`ret(0xffe78c44)`.

- `0xffe82f44`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe82f44 to 0xffe82fce (exclusive), 138 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `1d6b3b198ccee34bf5d0d7999e6aa31f1c1b77fc98b50480fb1d25849c48c4aa`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe82f50.
  Four-byte load, pattern 2: target `abs 0x30000370`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe82f54.
  Byte load, pattern 3: target `[0x30000370] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe82f5c.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe82fa6.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe82fa8.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe82faa.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe82fac.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe82fae.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe82fb0.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe83412`. Count 1 static access, including `pea.l` at 0xffe82fb2.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe82fb8.
  Call (`jsr` at 0xffe82fba): (unknown callee), target 0xffe8c48c. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe83412`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe82f64): target 0xffe82f6a. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe82f68): target 0xffe82f6c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe82f70): target 0xffe82f76. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe82f74): target 0xffe82f78. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe82f7c): target 0xffe82f82. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe82f80): target 0xffe82f84. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe82f88): target 0xffe82f8e. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe82f8c): target 0xffe82f90. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe82f92): target 0xffe82f98. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe82f96): target 0xffe82f9a. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe82f9e): target 0xffe82fa4. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe82fa2): target 0xffe82fa6. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe82fcc), d0=`ret(0xffe8c48c)`.

- `0xffe7035c`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#0`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe7035c to 0xffe703e6 (exclusive), 138 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0x0, 969844 bytes, SHA256 `f54e47c2935cd6cf448928e373ec7a24d3da1265b5f73adec5c3f26f426d278b`.
  Payload: carrier offset 0x40, 969780 bytes, SHA256 `703576f15e0bbcd5be107d70d238c5ded01c115e270ca1be2460b1531e9e24ae`.
  Read window: SHA256 `29bb6362a0aa66542b75c95278c12b7844455bac029a4193f1e0d74eccdc14c4`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe70368.
  Four-byte load, pattern 2: target `abs 0x3000029c`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe7036c.
  Byte load, pattern 3: target `[0x3000029c] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe70374.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe703be.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe703c0.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe703c2.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe703c4.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe703c6.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe703c8.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe70816`. Count 1 static access, including `pea.l` at 0xffe703ca.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe703d0.
  Call (`jsr` at 0xffe703d2): (unknown callee), target 0xffe76fd0. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe70816`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe7037c): target 0xffe70382. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe70380): target 0xffe70384. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe70388): target 0xffe7038e. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe7038c): target 0xffe70390. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe70394): target 0xffe7039a. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe70398): target 0xffe7039c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe703a0): target 0xffe703a6. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe703a4): target 0xffe703a8. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe703aa): target 0xffe703b0. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe703ae): target 0xffe703b2. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe703b6): target 0xffe703bc. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe703ba): target 0xffe703be. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe703e4), d0=`ret(0xffe76fd0)`.

- `0xffe84800`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis window: 0xffe84800 to 0xffe8488a (exclusive), 138 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `c236fa76ab561cea9abac7234e5d3bf958e81e2ee298bef0bdb310966661503a`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `a6`. Count 1 static access, including `movea.l` at 0xffe8480c.
  Four-byte load, pattern 2: target `abs 0x30000340`, raw base `absolute`. Count 1 static access, including `movea.l` at 0xffe84810.
  Byte load, pattern 3: target `[0x30000340] +0x1c9`, raw base `a0`. Count 1 static access, including `move.b` at 0xffe84818.
  Four-byte store, pattern 4: target `sp -0x18`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe84862.
  Four-byte store, pattern 5: target `sp -0x1c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe84864.
  Four-byte store, pattern 6: target `sp -0x20`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe84866.
  Four-byte store, pattern 7: target `sp -0x24`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe84868.
  Four-byte store, pattern 8: target `sp -0x28`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8486a.
  Four-byte store, pattern 9: target `sp -0x2c`, raw base `a7`, value `unknown`. Count 1 static access, including `move.l` at 0xffe8486c.
  Four-byte store, pattern 10: target `sp -0x30`, raw base `a7`, value `0xffe84cba`. Count 1 static access, including `pea.l` at 0xffe8486e.
  Four-byte store, pattern 11: target `sp -0x34`, raw base `a7`, value `[sp+4]`. Count 1 static access, including `move.l` at 0xffe84874.
  Call (`jsr` at 0xffe84876): (unknown callee), target 0xffe8f5b0. Candidate slots: stack arg0=`[sp+4]`, stack arg1=`0xffe84cba`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe84820): target 0xffe84826. Taken when `(? & 0x10) == 0x0`. Falls through when `(? & 0x10) != 0x0`.
  Jump (`bra.b` at 0xffe84824): target 0xffe84828. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8482c): target 0xffe84832. Taken when `(? & 0x8) == 0x0`. Falls through when `(? & 0x8) != 0x0`.
  Jump (`bra.b` at 0xffe84830): target 0xffe84834. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe84838): target 0xffe8483e. Taken when `(? & 0x40) == 0x0`. Falls through when `(? & 0x40) != 0x0`.
  Jump (`bra.b` at 0xffe8483c): target 0xffe84840. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe84844): target 0xffe8484a. Taken when `(? & 0x20) == 0x0`. Falls through when `(? & 0x20) != 0x0`.
  Jump (`bra.b` at 0xffe84848): target 0xffe8484c. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`bpl.b` at 0xffe8484e): target 0xffe84854. Taken when `N == 0`. Falls through when `not (N == 0)`.
  Jump (`bra.b` at 0xffe84852): target 0xffe84856. The local CFG lists a taken edge and no fallthrough edge.
  Branch (`beq.b` at 0xffe8485a): target 0xffe84860. Taken when `(? & 0x1) == 0x0`. Falls through when `(? & 0x1) != 0x0`.
  Jump (`bra.b` at 0xffe8485e): target 0xffe84862. The local CFG lists a taken edge and no fallthrough edge.
  At the exit (`rts` at 0xffe84888), d0=`ret(0xffe8f5b0)`.
