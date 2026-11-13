# ColdFire branching pointer-memory windows

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/branching-pointer-memory-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xffeca5f0`, `root/6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeca5f0 to 0xffeca6b2 (exclusive), 194 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `736b340cc121c6246894b4dbe7eb41e0fb03ab118a4bbded097339bb62443e37`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffeca5f8).
  Branch (`beq.b` at 0xffeca604): taken to 0xffeca63a when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffeca606 when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffeca616): taken to 0xffeca61c when `Z == 1`, falls through to 0xffeca618 when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffeca618).
  Jump (`bra.b` at 0xffeca61a): target 0xffeca612.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffeca622).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffeca62a).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffeca634).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffeca63a).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffeca640).
  Grouped accesses: 3 4-byte load records with target `abs 0x300002ac`, including `movea.l` at 0xffeca646, `movea.l` at 0xffeca662, `movea.l` at 0xffeca668.
  Byte load: target `[0x300002ac] +0x40` (`move.b` at 0xffeca64c).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffeca65c).
  Byte load: target `[0x300002ac] +0x40` (`move.b` at 0xffeca66e).
  Byte store: target `[0x300002ac] +0x40`, value `unknown` (`move.b` at 0xffeca672).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffeca676).
  Branch (`beq.b` at 0xffeca67c): taken to 0xffeca6a8 when `[0x301019ac] == 0x0`, falls through to 0xffeca67e when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffeca67e).
  Branch (`bne.b` at 0xffeca684): taken to 0xffeca6a8 when `[0x301019b4] != 0x0`, falls through to 0xffeca686 when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffeca686, `move.l` at 0xffeca694.
  Branch (`beq.b` at 0xffeca68c): taken to 0xffeca6a8 when `[0x30101998] == 0x0`, falls through to 0xffeca68e when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffeca68e).
  Call (`bsr.w` at 0xffeca6a0): (unknown callee), target 0xffeca958, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffeca6b0), d0 is `unknown`.

- `0xffe6cb2c`, `root/6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe6cb2c to 0xffe6cbee (exclusive), 194 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `883b0d9fc8151a237b08eac23ffb5ab324fcaf5fb031df50b677bd2381675d02`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe6cb34).
  Grouped accesses: 2 4-byte load records with target `abs 0x3000074c`, including `movea.l` at 0xffe6cb3e, `movea.l` at 0xffe6cb64.
  Byte load: target `[0x3000074c] +0x1c9` (`move.b` at 0xffe6cb42).
  Branch (`beq.b` at 0xffe6cb50): taken to 0xffe6cb6a when `unknown == unknown`, falls through to 0xffe6cb52 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe6cb54): taken to 0xffe6cb5e when `unknown == 0x0`, falls through to 0xffe6cb56 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe6cb5c): target 0xffe6cb64.
  Byte store: target `[0x3000074c] +0x1c9`, value `unknown` (`move.b` at 0xffe6cb66).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe6cb6e).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe6cb74).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe6cb7a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe6cb98).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe6cb9e).
  Grouped accesses: 3 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe6cbb2, `move.b` at 0xffe6cbbc, `move.b` at 0xffe6cbce.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe6cbb4).
  Branch (`bge.b` at 0xffe6cbd2): taken to 0xffe6cba4 when `ge flags condition (operands unknown)`, falls through to 0xffe6cbd4 when `not (ge flags condition (operands unknown))`.
  Four-byte store: target `(unknown effective address through a1)`, value `unknown` (`move.l` at 0xffe6cbd4).
  Branch (`bgt.b` at 0xffe6cbd8): taken to 0xffe6cb9e when `gt flags condition (operands unknown)`, falls through to 0xffe6cbda when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xffe6cbec) reports d0 = `0x1`.

- `0xffedcb2c`, `root/6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffedcb2c to 0xffedcbee (exclusive), 194 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `883b0d9fc8151a237b08eac23ffb5ab324fcaf5fb031df50b677bd2381675d02`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffedcb34).
  Grouped accesses: 2 4-byte load records with target `abs 0x3000074c`, including `movea.l` at 0xffedcb3e, `movea.l` at 0xffedcb64.
  Byte load: target `[0x3000074c] +0x1c9` (`move.b` at 0xffedcb42).
  Branch (`beq.b` at 0xffedcb50): taken to 0xffedcb6a when `unknown == unknown`, falls through to 0xffedcb52 when `unknown != unknown`.
  Branch (`beq.b` at 0xffedcb54): taken to 0xffedcb5e when `unknown == 0x0`, falls through to 0xffedcb56 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffedcb5c): target 0xffedcb64.
  Byte store: target `[0x3000074c] +0x1c9`, value `unknown` (`move.b` at 0xffedcb66).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffedcb6e).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffedcb74).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffedcb7a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffedcb98).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffedcb9e).
  Grouped accesses: 3 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffedcbb2, `move.b` at 0xffedcbbc, `move.b` at 0xffedcbce.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffedcbb4).
  Branch (`bge.b` at 0xffedcbd2): taken to 0xffedcba4 when `ge flags condition (operands unknown)`, falls through to 0xffedcbd4 when `not (ge flags condition (operands unknown))`.
  Four-byte store: target `(unknown effective address through a1)`, value `unknown` (`move.l` at 0xffedcbd4).
  Branch (`bgt.b` at 0xffedcbd8): taken to 0xffedcb9e when `gt flags condition (operands unknown)`, falls through to 0xffedcbda when `not (gt flags condition (operands unknown))`.
  `rts` at 0xffedcbec has recorded d0 = `0x1`.

- `0xffe71074`, `root/6.5.15/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.15:l1#0`, version `1.10.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71074 to 0xffe71136 (exclusive), 194 bytes.
  Carrier: 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Package: carrier offset 0x0, 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Payload: carrier offset 0x2c, 972800 bytes, SHA256 `18b09029d3c94d99ff6ddec9215755a6fc4933d212d53830db005768793dea73`.
  Read window: SHA256 `c6d02a6fdb1fdd17fcf16b5199d3523f08529e270e9c44b3ea88afc71a2721bf`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7107c).
  Grouped accesses: 2 4-byte load records with target `abs 0x300007d8`, including `movea.l` at 0xffe71086, `movea.l` at 0xffe710ac.
  Byte load: target `[0x300007d8] +0x1c9` (`move.b` at 0xffe7108a).
  Branch (`beq.b` at 0xffe71098): taken to 0xffe710b2 when `unknown == unknown`, falls through to 0xffe7109a when `unknown != unknown`.
  Branch (`beq.b` at 0xffe7109c): taken to 0xffe710a6 when `unknown == 0x0`, falls through to 0xffe7109e when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe710a4): target 0xffe710ac.
  Byte store: target `[0x300007d8] +0x1c9`, value `unknown` (`move.b` at 0xffe710ae).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe710b6).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe710bc).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe710c2).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe710e0).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe710e6).
  Grouped accesses: 3 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe710fa, `move.b` at 0xffe71104, `move.b` at 0xffe71116.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe710fc).
  Branch (`bge.b` at 0xffe7111a): taken to 0xffe710ec when `ge flags condition (operands unknown)`, falls through to 0xffe7111c when `not (ge flags condition (operands unknown))`.
  Four-byte store: target `(unknown effective address through a1)`, value `unknown` (`move.l` at 0xffe7111c).
  Branch (`bgt.b` at 0xffe71120): taken to 0xffe710e6 when `gt flags condition (operands unknown)`, falls through to 0xffe71122 when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffe71134).

- `0xffee1074`, `root/6.5.15/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.15:l1#0`, version `1.10.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee1074 to 0xffee1136 (exclusive), 194 bytes.
  Carrier: 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Package: carrier offset 0x0, 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Payload: carrier offset 0x2c, 972800 bytes, SHA256 `18b09029d3c94d99ff6ddec9215755a6fc4933d212d53830db005768793dea73`.
  Read window: SHA256 `c6d02a6fdb1fdd17fcf16b5199d3523f08529e270e9c44b3ea88afc71a2721bf`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee107c).
  Grouped accesses: 2 4-byte load records with target `abs 0x300007d8`, including `movea.l` at 0xffee1086, `movea.l` at 0xffee10ac.
  Byte load: target `[0x300007d8] +0x1c9` (`move.b` at 0xffee108a).
  Branch (`beq.b` at 0xffee1098): taken to 0xffee10b2 when `unknown == unknown`, falls through to 0xffee109a when `unknown != unknown`.
  Branch (`beq.b` at 0xffee109c): taken to 0xffee10a6 when `unknown == 0x0`, falls through to 0xffee109e when `unknown != 0x0`.
  Jump (`bra.b` at 0xffee10a4): target 0xffee10ac.
  Byte store: target `[0x300007d8] +0x1c9`, value `unknown` (`move.b` at 0xffee10ae).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffee10b6).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffee10bc).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffee10c2).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffee10e0).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffee10e6).
  Grouped accesses: 3 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffee10fa, `move.b` at 0xffee1104, `move.b` at 0xffee1116.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffee10fc).
  Branch (`bge.b` at 0xffee111a): taken to 0xffee10ec when `ge flags condition (operands unknown)`, falls through to 0xffee111c when `not (ge flags condition (operands unknown))`.
  Four-byte store: target `(unknown effective address through a1)`, value `unknown` (`move.l` at 0xffee111c).
  Branch (`bgt.b` at 0xffee1120): taken to 0xffee10e6 when `gt flags condition (operands unknown)`, falls through to 0xffee1122 when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffee1134), d0 is `0x1`.

- `0xffe74ccc`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#0`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe74ccc to 0xffe74d88 (exclusive), 188 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0x0, 974628 bytes, SHA256 `c091d32e310b7104130380d6c7f6c1e0a48f9cc4c79f13928860c1be37d52419`.
  Payload: carrier offset 0x40, 974564 bytes, SHA256 `a0a6cfec2692b7267ff56d52e46f1c75a39a623048dcd942835c816e4dde8ce0`.
  Read window: SHA256 `ced27fe55caf738fc4564ae1413347677ba0267349af65bfae41e892e63bb608`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffe74cd4).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffe74cd8).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffe74cdc, `movea.l` at 0xffe74ce4, `movea.l` at 0xffe74d30.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffe74ce0, `tst.l` at 0xffe74d3c.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe74ce8, `tst.l` at 0xffe74d34.
  Call (`jsr` at 0xffe74cf0): (unknown callee), target 0xffe77fa0, stack arg0=`0xffe75522`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffe74cfe): (unknown callee), target 0xffe61154, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffe61154)` (`move.l` at 0xffe74d06).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffe74d08).
  Branch (`bne.b` at 0xffe74d0e): taken to 0xffe74d30 when `[0x30000044] != 0x0`, falls through to 0xffe74d10 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffe74d10, `move.l` at 0xffe74d24.
  Call (`jsr` at 0xffe74d12): (unknown callee), target 0xffe6f398, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe74d1c): taken to 0xffe74d30 when `ret(0xffe6f398) != 0x0`, falls through to 0xffe74d1e when `ret(0xffe6f398) == 0x0`.
  Call (`jsr` at 0xffe74d26): (unknown callee), target 0xffe7a200, stack arg0=`[[sp+8]]`, stack arg1=`0xffe75180`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffe74d2e): target 0xffe74d7c.
  Branch (`beq.b` at 0xffe74d36): taken to 0xffe74d72 when `[[[sp+4]+112]] == 0x0`, falls through to 0xffe74d38 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffe74d40): taken to 0xffe74d5a when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffe74d42 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000630`, including `movea.l` at 0xffe74d42, `movea.l` at 0xffe74d5a.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe74d4c, `move.l` at 0xffe74d64.
  Byte load: target `[0x30000630] +0x1c9` (`move.b` at 0xffe74d4e).
  Byte store: target `[0x30000630] +0x1c9`, value `unknown` (`move.b` at 0xffe74d54).
  Jump (`bra.b` at 0xffe74d58): target 0xffe74d72.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe74d68).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffe74d6e).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffe74d72).
  Call (`jsr` at 0xffe74d74): (unknown callee), target 0xffe6ff10, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe74d86) reports d0 = `0x0`.

- `0xffee4ccc`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#0`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee4ccc to 0xffee4d88 (exclusive), 188 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0x0, 974628 bytes, SHA256 `c091d32e310b7104130380d6c7f6c1e0a48f9cc4c79f13928860c1be37d52419`.
  Payload: carrier offset 0x40, 974564 bytes, SHA256 `a0a6cfec2692b7267ff56d52e46f1c75a39a623048dcd942835c816e4dde8ce0`.
  Read window: SHA256 `e659156a96cb6ed98db2c82f65ae9bb1dd9b32707b9a0e110375dc1f27592a2b`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffee4cd4).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffee4cd8).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffee4cdc, `movea.l` at 0xffee4ce4, `movea.l` at 0xffee4d30.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffee4ce0, `tst.l` at 0xffee4d3c.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffee4ce8, `tst.l` at 0xffee4d34.
  Call (`jsr` at 0xffee4cf0): (unknown callee), target 0xffee7fa0, stack arg0=`0xffee5522`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffee4cfe): (unknown callee), target 0xffed1154, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffed1154)` (`move.l` at 0xffee4d06).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffee4d08).
  Branch (`bne.b` at 0xffee4d0e): taken to 0xffee4d30 when `[0x30000044] != 0x0`, falls through to 0xffee4d10 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffee4d10, `move.l` at 0xffee4d24.
  Call (`jsr` at 0xffee4d12): (unknown callee), target 0xffedf398, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffee4d1c): taken to 0xffee4d30 when `ret(0xffedf398) != 0x0`, falls through to 0xffee4d1e when `ret(0xffedf398) == 0x0`.
  Call (`jsr` at 0xffee4d26): (unknown callee), target 0xffeea200, stack arg0=`[[sp+8]]`, stack arg1=`0xffee5180`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffee4d2e): target 0xffee4d7c.
  Branch (`beq.b` at 0xffee4d36): taken to 0xffee4d72 when `[[[sp+4]+112]] == 0x0`, falls through to 0xffee4d38 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffee4d40): taken to 0xffee4d5a when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffee4d42 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000630`, including `movea.l` at 0xffee4d42, `movea.l` at 0xffee4d5a.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffee4d4c, `move.l` at 0xffee4d64.
  Byte load: target `[0x30000630] +0x1c9` (`move.b` at 0xffee4d4e).
  Byte store: target `[0x30000630] +0x1c9`, value `unknown` (`move.b` at 0xffee4d54).
  Jump (`bra.b` at 0xffee4d58): target 0xffee4d72.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffee4d68).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffee4d6e).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffee4d72).
  Call (`jsr` at 0xffee4d74): (unknown callee), target 0xffedff10, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffee4d86 has recorded d0 = `0x0`.

- `0xffe7db0c`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#1`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7db0c to 0xffe7dbc8 (exclusive), 188 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0xedf24, 1589988 bytes, SHA256 `5df6abeffec6c64b0fecbc68970e31a7dd41c2444993af801114d27a944d4799`.
  Payload: carrier offset 0xedf50, 1589944 bytes, SHA256 `64ad5503da721ec1dd85f37e372b6918629a1fd90ab9d8f990ef870fd86121e7`.
  Read window: SHA256 `03bf49455d1dd7868eae42c4b08dfc1289b7f806737455f26f136f33a75f804f`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffe7db14).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffe7db18).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffe7db1c, `movea.l` at 0xffe7db24, `movea.l` at 0xffe7db70.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffe7db20, `tst.l` at 0xffe7db7c.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe7db28, `tst.l` at 0xffe7db74.
  Call (`jsr` at 0xffe7db30): (unknown callee), target 0xffe80df8, stack arg0=`0xffe7e362`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffe7db3e): (unknown callee), target 0xffe69288, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffe69288)` (`move.l` at 0xffe7db46).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffe7db48).
  Branch (`bne.b` at 0xffe7db4e): taken to 0xffe7db70 when `[0x30000044] != 0x0`, falls through to 0xffe7db50 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffe7db50, `move.l` at 0xffe7db64.
  Call (`jsr` at 0xffe7db52): (unknown callee), target 0xffe781d8, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe7db5c): taken to 0xffe7db70 when `ret(0xffe781d8) != 0x0`, falls through to 0xffe7db5e when `ret(0xffe781d8) == 0x0`.
  Call (`jsr` at 0xffe7db66): (unknown callee), target 0xffe83058, stack arg0=`[[sp+8]]`, stack arg1=`0xffe7dfc0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffe7db6e): target 0xffe7dbbc.
  Branch (`beq.b` at 0xffe7db76): taken to 0xffe7dbb2 when `[[[sp+4]+112]] == 0x0`, falls through to 0xffe7db78 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffe7db80): taken to 0xffe7db9a when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffe7db82 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300006a0`, including `movea.l` at 0xffe7db82, `movea.l` at 0xffe7db9a.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe7db8c, `move.l` at 0xffe7dba4.
  Byte load: target `[0x300006a0] +0x1c9` (`move.b` at 0xffe7db8e).
  Byte store: target `[0x300006a0] +0x1c9`, value `unknown` (`move.b` at 0xffe7db94).
  Jump (`bra.b` at 0xffe7db98): target 0xffe7dbb2.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe7dba8).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffe7dbae).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffe7dbb2).
  Call (`jsr` at 0xffe7dbb4): (unknown callee), target 0xffe78d50, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `0x0` (`rts` at 0xffe7dbc6).

- `0xfff6db0c`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#1`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff6db0c to 0xfff6dbc8 (exclusive), 188 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0xedf24, 1589988 bytes, SHA256 `5df6abeffec6c64b0fecbc68970e31a7dd41c2444993af801114d27a944d4799`.
  Payload: carrier offset 0xedf50, 1589944 bytes, SHA256 `64ad5503da721ec1dd85f37e372b6918629a1fd90ab9d8f990ef870fd86121e7`.
  Read window: SHA256 `d320e56d408171b8ee3648078f355d412c4d764333a4141a6fdc631b3fca2c38`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xfff6db14).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xfff6db18).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xfff6db1c, `movea.l` at 0xfff6db24, `movea.l` at 0xfff6db70.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xfff6db20, `tst.l` at 0xfff6db7c.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xfff6db28, `tst.l` at 0xfff6db74.
  Call (`jsr` at 0xfff6db30): (unknown callee), target 0xfff70df8, stack arg0=`0xfff6e362`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xfff6db3e): (unknown callee), target 0xfff59288, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xfff59288)` (`move.l` at 0xfff6db46).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xfff6db48).
  Branch (`bne.b` at 0xfff6db4e): taken to 0xfff6db70 when `[0x30000044] != 0x0`, falls through to 0xfff6db50 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xfff6db50, `move.l` at 0xfff6db64.
  Call (`jsr` at 0xfff6db52): (unknown callee), target 0xfff681d8, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff6db5c): taken to 0xfff6db70 when `ret(0xfff681d8) != 0x0`, falls through to 0xfff6db5e when `ret(0xfff681d8) == 0x0`.
  Call (`jsr` at 0xfff6db66): (unknown callee), target 0xfff73058, stack arg0=`[[sp+8]]`, stack arg1=`0xfff6dfc0`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xfff6db6e): target 0xfff6dbbc.
  Branch (`beq.b` at 0xfff6db76): taken to 0xfff6dbb2 when `[[[sp+4]+112]] == 0x0`, falls through to 0xfff6db78 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xfff6db80): taken to 0xfff6db9a when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xfff6db82 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300006a0`, including `movea.l` at 0xfff6db82, `movea.l` at 0xfff6db9a.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xfff6db8c, `move.l` at 0xfff6dba4.
  Byte load: target `[0x300006a0] +0x1c9` (`move.b` at 0xfff6db8e).
  Byte store: target `[0x300006a0] +0x1c9`, value `unknown` (`move.b` at 0xfff6db94).
  Jump (`bra.b` at 0xfff6db98): target 0xfff6dbb2.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xfff6dba8).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xfff6dbae).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xfff6dbb2).
  Call (`jsr` at 0xfff6dbb4): (unknown callee), target 0xfff68d50, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xfff6dbc6), d0 is `0x0`.

- `0xffe74e24`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe74e24 to 0xffe74ee0 (exclusive), 188 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `c7192ec4ff40d0a182b29c4cb26798d4f85936599e31fbc44b877228921300ec`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffe74e2c).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffe74e30).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffe74e34, `movea.l` at 0xffe74e3c, `movea.l` at 0xffe74e88.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffe74e38, `tst.l` at 0xffe74e94.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe74e40, `tst.l` at 0xffe74e8c.
  Call (`jsr` at 0xffe74e48): (unknown callee), target 0xffe78864, stack arg0=`0xffe7567a`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffe74e56): (unknown callee), target 0xffe5bc0c, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffe5bc0c)` (`move.l` at 0xffe74e5e).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffe74e60).
  Branch (`bne.b` at 0xffe74e66): taken to 0xffe74e88 when `[0x30000044] != 0x0`, falls through to 0xffe74e68 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffe74e68, `move.l` at 0xffe74e7c.
  Call (`jsr` at 0xffe74e6a): (unknown callee), target 0xffe70de0, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe74e74): taken to 0xffe74e88 when `ret(0xffe70de0) != 0x0`, falls through to 0xffe74e76 when `ret(0xffe70de0) == 0x0`.
  Call (`jsr` at 0xffe74e7e): (unknown callee), target 0xffe7aac4, stack arg0=`[[sp+8]]`, stack arg1=`0xffe752d8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffe74e86): target 0xffe74ed4.
  Branch (`beq.b` at 0xffe74e8e): taken to 0xffe74eca when `[[[sp+4]+112]] == 0x0`, falls through to 0xffe74e90 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffe74e98): taken to 0xffe74eb2 when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffe74e9a when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300002d4`, including `movea.l` at 0xffe74e9a, `movea.l` at 0xffe74eb2.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe74ea4, `move.l` at 0xffe74ebc.
  Byte load: target `[0x300002d4] +0x1c9` (`move.b` at 0xffe74ea6).
  Byte store: target `[0x300002d4] +0x1c9`, value `unknown` (`move.b` at 0xffe74eac).
  Jump (`bra.b` at 0xffe74eb0): target 0xffe74eca.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe74ec0).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffe74ec6).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffe74eca).
  Call (`jsr` at 0xffe74ecc): (unknown callee), target 0xffe71afc, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe74ede) reports d0 = `0x0`.

- `0xffee4e24`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee4e24 to 0xffee4ee0 (exclusive), 188 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `cf822856f1fc478d0d0bbc158be32b5fc5f865a7399c736e7edd513967691f2c`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffee4e2c).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffee4e30).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffee4e34, `movea.l` at 0xffee4e3c, `movea.l` at 0xffee4e88.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffee4e38, `tst.l` at 0xffee4e94.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffee4e40, `tst.l` at 0xffee4e8c.
  Call (`jsr` at 0xffee4e48): (unknown callee), target 0xffee8864, stack arg0=`0xffee567a`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffee4e56): (unknown callee), target 0xffecbc0c, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffecbc0c)` (`move.l` at 0xffee4e5e).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffee4e60).
  Branch (`bne.b` at 0xffee4e66): taken to 0xffee4e88 when `[0x30000044] != 0x0`, falls through to 0xffee4e68 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffee4e68, `move.l` at 0xffee4e7c.
  Call (`jsr` at 0xffee4e6a): (unknown callee), target 0xffee0de0, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffee4e74): taken to 0xffee4e88 when `ret(0xffee0de0) != 0x0`, falls through to 0xffee4e76 when `ret(0xffee0de0) == 0x0`.
  Call (`jsr` at 0xffee4e7e): (unknown callee), target 0xffeeaac4, stack arg0=`[[sp+8]]`, stack arg1=`0xffee52d8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffee4e86): target 0xffee4ed4.
  Branch (`beq.b` at 0xffee4e8e): taken to 0xffee4eca when `[[[sp+4]+112]] == 0x0`, falls through to 0xffee4e90 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffee4e98): taken to 0xffee4eb2 when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffee4e9a when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300002d4`, including `movea.l` at 0xffee4e9a, `movea.l` at 0xffee4eb2.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffee4ea4, `move.l` at 0xffee4ebc.
  Byte load: target `[0x300002d4] +0x1c9` (`move.b` at 0xffee4ea6).
  Byte store: target `[0x300002d4] +0x1c9`, value `unknown` (`move.b` at 0xffee4eac).
  Jump (`bra.b` at 0xffee4eb0): target 0xffee4eca.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffee4ec0).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffee4ec6).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffee4eca).
  Call (`jsr` at 0xffee4ecc): (unknown callee), target 0xffee1afc, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffee4ede has recorded d0 = `0x0`.

- `0xffe8626c`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe8626c to 0xffe86328 (exclusive), 188 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `445ccdd209da9dede51f7caefc0727d7a081b918c64041524233d743c9a1b9fa`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffe86274).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffe86278).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffe8627c, `movea.l` at 0xffe86284, `movea.l` at 0xffe862d0.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffe86280, `tst.l` at 0xffe862dc.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe86288, `tst.l` at 0xffe862d4.
  Call (`jsr` at 0xffe86290): (unknown callee), target 0xffe8c0ac, stack arg0=`0xffe86ac2`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffe8629e): (unknown callee), target 0xffe6ba84, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffe6ba84)` (`move.l` at 0xffe862a6).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffe862a8).
  Branch (`bne.b` at 0xffe862ae): taken to 0xffe862d0 when `[0x30000044] != 0x0`, falls through to 0xffe862b0 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffe862b0, `move.l` at 0xffe862c4.
  Call (`jsr` at 0xffe862b2): (unknown callee), target 0xffe82228, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe862bc): taken to 0xffe862d0 when `ret(0xffe82228) != 0x0`, falls through to 0xffe862be when `ret(0xffe82228) == 0x0`.
  Call (`jsr` at 0xffe862c6): (unknown callee), target 0xffe8e30c, stack arg0=`[[sp+8]]`, stack arg1=`0xffe86720`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffe862ce): target 0xffe8631c.
  Branch (`beq.b` at 0xffe862d6): taken to 0xffe86312 when `[[[sp+4]+112]] == 0x0`, falls through to 0xffe862d8 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffe862e0): taken to 0xffe862fa when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffe862e2 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xffe862e2, `movea.l` at 0xffe862fa.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe862ec, `move.l` at 0xffe86304.
  Byte load: target `[0x30000370] +0x1c9` (`move.b` at 0xffe862ee).
  Byte store: target `[0x30000370] +0x1c9`, value `unknown` (`move.b` at 0xffe862f4).
  Jump (`bra.b` at 0xffe862f8): target 0xffe86312.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe86308).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffe8630e).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffe86312).
  Call (`jsr` at 0xffe86314): (unknown callee), target 0xffe82f44, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `0x0` (`rts` at 0xffe86326).

- `0xfff7626c`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff7626c to 0xfff76328 (exclusive), 188 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `38f17c85cdf6cd489072fc1924d00c14c8b73977aaaff2f715017f7ee8a3f98f`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xfff76274).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xfff76278).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xfff7627c, `movea.l` at 0xfff76284, `movea.l` at 0xfff762d0.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xfff76280, `tst.l` at 0xfff762dc.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xfff76288, `tst.l` at 0xfff762d4.
  Call (`jsr` at 0xfff76290): (unknown callee), target 0xfff7c0ac, stack arg0=`0xfff76ac2`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xfff7629e): (unknown callee), target 0xfff5ba84, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xfff5ba84)` (`move.l` at 0xfff762a6).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xfff762a8).
  Branch (`bne.b` at 0xfff762ae): taken to 0xfff762d0 when `[0x30000044] != 0x0`, falls through to 0xfff762b0 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xfff762b0, `move.l` at 0xfff762c4.
  Call (`jsr` at 0xfff762b2): (unknown callee), target 0xfff72228, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff762bc): taken to 0xfff762d0 when `ret(0xfff72228) != 0x0`, falls through to 0xfff762be when `ret(0xfff72228) == 0x0`.
  Call (`jsr` at 0xfff762c6): (unknown callee), target 0xfff7e30c, stack arg0=`[[sp+8]]`, stack arg1=`0xfff76720`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xfff762ce): target 0xfff7631c.
  Branch (`beq.b` at 0xfff762d6): taken to 0xfff76312 when `[[[sp+4]+112]] == 0x0`, falls through to 0xfff762d8 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xfff762e0): taken to 0xfff762fa when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xfff762e2 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xfff762e2, `movea.l` at 0xfff762fa.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xfff762ec, `move.l` at 0xfff76304.
  Byte load: target `[0x30000370] +0x1c9` (`move.b` at 0xfff762ee).
  Byte store: target `[0x30000370] +0x1c9`, value `unknown` (`move.b` at 0xfff762f4).
  Jump (`bra.b` at 0xfff762f8): target 0xfff76312.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xfff76308).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xfff7630e).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xfff76312).
  Call (`jsr` at 0xfff76314): (unknown callee), target 0xfff72f44, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xfff76326), d0 is `0x0`.

- `0xffe735dc`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#0`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe735dc to 0xffe73698 (exclusive), 188 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0x0, 969844 bytes, SHA256 `f54e47c2935cd6cf448928e373ec7a24d3da1265b5f73adec5c3f26f426d278b`.
  Payload: carrier offset 0x40, 969780 bytes, SHA256 `703576f15e0bbcd5be107d70d238c5ded01c115e270ca1be2460b1531e9e24ae`.
  Read window: SHA256 `0d8caa380df615f8a6059731b7101f04ac6d7c9beb5f318b679da231323ba9a0`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffe735e4).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffe735e8).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffe735ec, `movea.l` at 0xffe735f4, `movea.l` at 0xffe73640.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffe735f0, `tst.l` at 0xffe7364c.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe735f8, `tst.l` at 0xffe73644.
  Call (`jsr` at 0xffe73600): (unknown callee), target 0xffe76bf0, stack arg0=`0xffe73e32`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffe7360e): (unknown callee), target 0xffe5b4a0, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffe5b4a0)` (`move.l` at 0xffe73616).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffe73618).
  Branch (`bne.b` at 0xffe7361e): taken to 0xffe73640 when `[0x30000044] != 0x0`, falls through to 0xffe73620 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffe73620, `move.l` at 0xffe73634.
  Call (`jsr` at 0xffe73622): (unknown callee), target 0xffe6f748, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe7362c): taken to 0xffe73640 when `ret(0xffe6f748) != 0x0`, falls through to 0xffe7362e when `ret(0xffe6f748) == 0x0`.
  Call (`jsr` at 0xffe73636): (unknown callee), target 0xffe78e50, stack arg0=`[[sp+8]]`, stack arg1=`0xffe73a90`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffe7363e): target 0xffe7368c.
  Branch (`beq.b` at 0xffe73646): taken to 0xffe73682 when `[[[sp+4]+112]] == 0x0`, falls through to 0xffe73648 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffe73650): taken to 0xffe7366a when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffe73652 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000029c`, including `movea.l` at 0xffe73652, `movea.l` at 0xffe7366a.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffe7365c, `move.l` at 0xffe73674.
  Byte load: target `[0x3000029c] +0x1c9` (`move.b` at 0xffe7365e).
  Byte store: target `[0x3000029c] +0x1c9`, value `unknown` (`move.b` at 0xffe73664).
  Jump (`bra.b` at 0xffe73668): target 0xffe73682.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffe73678).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffe7367e).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffe73682).
  Call (`jsr` at 0xffe73684): (unknown callee), target 0xffe7035c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffe73696) reports d0 = `0x0`.

- `0xffee35dc`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#0`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee35dc to 0xffee3698 (exclusive), 188 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0x0, 969844 bytes, SHA256 `f54e47c2935cd6cf448928e373ec7a24d3da1265b5f73adec5c3f26f426d278b`.
  Payload: carrier offset 0x40, 969780 bytes, SHA256 `703576f15e0bbcd5be107d70d238c5ded01c115e270ca1be2460b1531e9e24ae`.
  Read window: SHA256 `d5f533d52ace357ff690b4694ab885f2889af2164997892ed98f2455eeb4ea80`.
  Four-byte load: target `sp +0x4` (`movea.l` at 0xffee35e4).
  Four-byte load: target `sp +0x8` (`movea.l` at 0xffee35e8).
  Grouped accesses: 6 4-byte load records with target `[sp+4] +0x70`, including `movea.l` at 0xffee35ec, `movea.l` at 0xffee35f4, `movea.l` at 0xffee3640.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x4`, including `move.l` at 0xffee35f0, `tst.l` at 0xffee364c.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffee35f8, `tst.l` at 0xffee3644.
  Call (`jsr` at 0xffee3600): (unknown callee), target 0xffee6bf0, stack arg0=`0xffee3e32`, stack arg1=`[[[sp+4]+112]]`, stack arg2=`[[[sp+4]+112]+4]`, stack arg3=`a4`, stack arg4=`a5`, stack arg5=`a6`.
  Call (`jsr` at 0xffee360e): (unknown callee), target 0xffecb4a0, stack arg0=`0x50`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `[sp+8] +0x0`, value `ret(0xffecb4a0)` (`move.l` at 0xffee3616).
  Four-byte load: target `abs 0x30000044` (`tst.l` at 0xffee3618).
  Branch (`bne.b` at 0xffee361e): taken to 0xffee3640 when `[0x30000044] != 0x0`, falls through to 0xffee3620 when `[0x30000044] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp+8] +0x0`, including `move.l` at 0xffee3620, `move.l` at 0xffee3634.
  Call (`jsr` at 0xffee3622): (unknown callee), target 0xffedf748, stack arg0=`[[sp+8]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffee362c): taken to 0xffee3640 when `ret(0xffedf748) != 0x0`, falls through to 0xffee362e when `ret(0xffedf748) == 0x0`.
  Call (`jsr` at 0xffee3636): (unknown callee), target 0xffee8e50, stack arg0=`[[sp+8]]`, stack arg1=`0xffee3a90`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`bra.b` at 0xffee363e): target 0xffee368c.
  Branch (`beq.b` at 0xffee3646): taken to 0xffee3682 when `[[[sp+4]+112]] == 0x0`, falls through to 0xffee3648 when `[[[sp+4]+112]] != 0x0`.
  Branch (`beq.b` at 0xffee3650): taken to 0xffee366a when `[[[sp+4]+112]+4] == 0x0`, falls through to 0xffee3652 when `[[[sp+4]+112]+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000029c`, including `movea.l` at 0xffee3652, `movea.l` at 0xffee366a.
  Grouped accesses: 2 4-byte load records with target `[[sp+4]+112] +0x0`, including `move.l` at 0xffee365c, `move.l` at 0xffee3674.
  Byte load: target `[0x3000029c] +0x1c9` (`move.b` at 0xffee365e).
  Byte store: target `[0x3000029c] +0x1c9`, value `unknown` (`move.b` at 0xffee3664).
  Jump (`bra.b` at 0xffee3668): target 0xffee3682.
  Byte load: target `(unknown effective address through a0)` (`move.b` at 0xffee3678).
  Byte store: target `(unknown effective address through a0)`, value `unknown` (`move.b` at 0xffee367e).
  Four-byte load: target `(unknown effective address through a4)` (`move.l` at 0xffee3682).
  Call (`jsr` at 0xffee3684): (unknown callee), target 0xffee035c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffee3696 has recorded d0 = `0x0`.
