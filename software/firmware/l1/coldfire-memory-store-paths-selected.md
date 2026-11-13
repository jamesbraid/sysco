# ColdFire memory-store paths in selected images

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/memory-store-paths-selected-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xffed31bc`, `root/6.5.15/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.15:l1#0`, version `1.10.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffed31bc to 0xffed3318 (exclusive), 348 bytes.
  Carrier: 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Package: carrier offset 0x0, 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Payload: carrier offset 0x2c, 972800 bytes, SHA256 `18b09029d3c94d99ff6ddec9215755a6fc4933d212d53830db005768793dea73`.
  Read window: SHA256 `03f164e915e69550af4b870d1b10b68ace16f5b0303bc1798c7e53ddbe82039b`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffed31c8).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffed31cc).
  Byte load: target `sp +0x13` (`move.b` at 0xffed31d0).
  Branch (`beq.b` at 0xffed31d8): taken to 0xffed31e0 when `Z == 1`, falls through to 0xffed31da when `not (Z == 1)`.
  Branch (`beq.b` at 0xffed31dc): taken to 0xffed31f8 when `Z == 1`, falls through to 0xffed31de when `not (Z == 1)`.
  Jump (`bra.b` at 0xffed31de): target 0xffed3210.
  Grouped accesses: 4 4-byte load records with target `abs 0x300007d8`, including `movea.l` at 0xffed31e6, `movea.l` at 0xffed31fe, `movea.l` at 0xffed32e4.
  Jump (`bra.b` at 0xffed31f6): target 0xffed3232.
  Jump (`bra.b` at 0xffed320e): target 0xffed3232.
  Call (`jsr` at 0xffed3228): (unknown callee), target 0xffec83ac, stack arg0=`0x9`, stack arg1=`0xffed39f4`, stack arg2=`0x7c`, stack arg3=`0x0`, stack arg4=`0xffed3a01`, stack arg5=`[sp+4]`.
  Four-byte load: target `(unknown effective address through a5)` (`tst.l` at 0xffed3232).
  Branch (`beq.b` at 0xffed3234): taken to 0xffed3258 when `Z == 1`, falls through to 0xffed3236 when `not (Z == 1)`.
  Call (`jsr` at 0xffed324e): (unknown callee), target 0xffec83ac, stack arg0=`0x9`, stack arg1=`0xffed39f4`, stack arg2=`0x80`, stack arg3=`0x0`, stack arg4=`0xffed3a30`, stack arg5=`[sp+4]`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffed325a).
  Call (`jsr` at 0xffed326e): (unknown callee), target 0xffed5658, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffed3470`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed3286): (unknown callee), target 0xffeb4ba4, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffed3294): taken to 0xffed32b8 when `ret(0xffeb4ba4) == 0x0`, falls through to 0xffed3296 when `ret(0xffeb4ba4) != 0x0`.
  Call (`jsr` at 0xffed32ae): (unknown callee), target 0xffec83ac, stack arg0=`0x9`, stack arg1=`0xffed39f4`, stack arg2=`0x8e`, stack arg3=`0x0`, stack arg4=`0xffed3a67`, stack arg5=`ret(0xffeb4ba4)`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffed5658)` (`move.l` at 0xffed32b8).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffed32ba).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffed32be).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffed32c2).
  Call (`jsr` at 0xffed32c8): (unknown callee), target 0xffed549c, stack arg0=`ret(0xffed5658)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed32d6): (unknown callee), target 0xffed560c, stack arg0=`ret(0xffed5658)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffed32e2): taken to 0xffed32fa when `0x1 != [sp+4]`, falls through to 0xffed32e4 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x300007d8] +0x36`, including `move.w` at 0xffed32ea, `move.w` at 0xffed3300.
  Grouped accesses: 2 2-byte store records with target `[0x300007d8] +0x36`, reported value `unknown`, including `move.w` at 0xffed32f4, `move.w` at 0xffed330a.
  Jump (`bra.b` at 0xffed32f8): target 0xffed330e.
  At the return (`rts` at 0xffed3316), d0 is `unknown`.

- `0xffe636a4`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#0`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe636a4 to 0xffe63800 (exclusive), 348 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0x0, 974628 bytes, SHA256 `c091d32e310b7104130380d6c7f6c1e0a48f9cc4c79f13928860c1be37d52419`.
  Payload: carrier offset 0x40, 974564 bytes, SHA256 `a0a6cfec2692b7267ff56d52e46f1c75a39a623048dcd942835c816e4dde8ce0`.
  Read window: SHA256 `6b8561a2fab4c79b4700d007f0b3d92471654d093d3812e175feb0ce8a50aef5`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe636b0).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe636b4).
  Byte load: target `sp +0x13` (`move.b` at 0xffe636b8).
  Branch (`beq.b` at 0xffe636c0): taken to 0xffe636c8 when `Z == 1`, falls through to 0xffe636c2 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe636c4): taken to 0xffe636e0 when `Z == 1`, falls through to 0xffe636c6 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe636c6): target 0xffe636f8.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000630`, including `movea.l` at 0xffe636ce, `movea.l` at 0xffe636e6, `movea.l` at 0xffe637cc.
  Jump (`bra.b` at 0xffe636de): target 0xffe6371a.
  Jump (`bra.b` at 0xffe636f6): target 0xffe6371a.
  Call (`jsr` at 0xffe63710): (unknown callee), target 0xffe5e128, stack arg0=`0x9`, stack arg1=`0xffe63edc`, stack arg2=`0x7c`, stack arg3=`0x0`, stack arg4=`0xffe63ee9`, stack arg5=`[sp+4]`.
  Four-byte load: target `(unknown effective address through a5)` (`tst.l` at 0xffe6371a).
  Branch (`beq.b` at 0xffe6371c): taken to 0xffe63740 when `Z == 1`, falls through to 0xffe6371e when `not (Z == 1)`.
  Call (`jsr` at 0xffe63736): (unknown callee), target 0xffe5e128, stack arg0=`0x9`, stack arg1=`0xffe63edc`, stack arg2=`0x80`, stack arg3=`0x0`, stack arg4=`0xffe63f18`, stack arg5=`[sp+4]`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe63742).
  Call (`jsr` at 0xffe63756): (unknown callee), target 0xffe64e60, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe63958`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6376e): (unknown callee), target 0xffe4574c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6377c): taken to 0xffe637a0 when `ret(0xffe4574c) == 0x0`, falls through to 0xffe6377e when `ret(0xffe4574c) != 0x0`.
  Call (`jsr` at 0xffe63796): (unknown callee), target 0xffe5e128, stack arg0=`0x9`, stack arg1=`0xffe63edc`, stack arg2=`0x8e`, stack arg3=`0x0`, stack arg4=`0xffe63f4f`, stack arg5=`ret(0xffe4574c)`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe64e60)` (`move.l` at 0xffe637a0).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe637a2).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffe637a6).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe637aa).
  Call (`jsr` at 0xffe637b0): (unknown callee), target 0xffe64ca4, stack arg0=`ret(0xffe64e60)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe637be): (unknown callee), target 0xffe64e14, stack arg0=`ret(0xffe64e60)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe637ca): taken to 0xffe637e2 when `0x1 != [sp+4]`, falls through to 0xffe637cc when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000630] +0x36`, including `move.w` at 0xffe637d2, `move.w` at 0xffe637e8.
  Grouped accesses: 2 2-byte store records with target `[0x30000630] +0x36`, reported value `unknown`, including `move.w` at 0xffe637dc, `move.w` at 0xffe637f2.
  Jump (`bra.b` at 0xffe637e0): target 0xffe637f6.
  The exit (`rts` at 0xffe637fe) reports d0 = `unknown`.

- `0xffed36a4`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#0`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffed36a4 to 0xffed3800 (exclusive), 348 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0x0, 974628 bytes, SHA256 `c091d32e310b7104130380d6c7f6c1e0a48f9cc4c79f13928860c1be37d52419`.
  Payload: carrier offset 0x40, 974564 bytes, SHA256 `a0a6cfec2692b7267ff56d52e46f1c75a39a623048dcd942835c816e4dde8ce0`.
  Read window: SHA256 `415f25274a7537f87341cf3baef398c0d2b6af724bd2a8a230e23dea9c2075ad`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffed36b0).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffed36b4).
  Byte load: target `sp +0x13` (`move.b` at 0xffed36b8).
  Branch (`beq.b` at 0xffed36c0): taken to 0xffed36c8 when `Z == 1`, falls through to 0xffed36c2 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffed36c4): taken to 0xffed36e0 when `Z == 1`, falls through to 0xffed36c6 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffed36c6): target 0xffed36f8.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000630`, including `movea.l` at 0xffed36ce, `movea.l` at 0xffed36e6, `movea.l` at 0xffed37cc.
  Jump (`bra.b` at 0xffed36de): target 0xffed371a.
  Jump (`bra.b` at 0xffed36f6): target 0xffed371a.
  Call (`jsr` at 0xffed3710): (unknown callee), target 0xffece128, stack arg0=`0x9`, stack arg1=`0xffed3edc`, stack arg2=`0x7c`, stack arg3=`0x0`, stack arg4=`0xffed3ee9`, stack arg5=`[sp+4]`.
  Four-byte load: target `(unknown effective address through a5)` (`tst.l` at 0xffed371a).
  Branch (`beq.b` at 0xffed371c): taken to 0xffed3740 when `Z == 1`, falls through to 0xffed371e when `not (Z == 1)`.
  Call (`jsr` at 0xffed3736): (unknown callee), target 0xffece128, stack arg0=`0x9`, stack arg1=`0xffed3edc`, stack arg2=`0x80`, stack arg3=`0x0`, stack arg4=`0xffed3f18`, stack arg5=`[sp+4]`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffed3742).
  Call (`jsr` at 0xffed3756): (unknown callee), target 0xffed4e60, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffed3958`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed376e): (unknown callee), target 0xffeb574c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffed377c): taken to 0xffed37a0 when `ret(0xffeb574c) == 0x0`, falls through to 0xffed377e when `ret(0xffeb574c) != 0x0`.
  Call (`jsr` at 0xffed3796): (unknown callee), target 0xffece128, stack arg0=`0x9`, stack arg1=`0xffed3edc`, stack arg2=`0x8e`, stack arg3=`0x0`, stack arg4=`0xffed3f4f`, stack arg5=`ret(0xffeb574c)`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffed4e60)` (`move.l` at 0xffed37a0).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffed37a2).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffed37a6).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffed37aa).
  Call (`jsr` at 0xffed37b0): (unknown callee), target 0xffed4ca4, stack arg0=`ret(0xffed4e60)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed37be): (unknown callee), target 0xffed4e14, stack arg0=`ret(0xffed4e60)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffed37ca): taken to 0xffed37e2 when `0x1 != [sp+4]`, falls through to 0xffed37cc when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000630] +0x36`, including `move.w` at 0xffed37d2, `move.w` at 0xffed37e8.
  Grouped accesses: 2 2-byte store records with target `[0x30000630] +0x36`, reported value `unknown`, including `move.w` at 0xffed37dc, `move.w` at 0xffed37f2.
  Jump (`bra.b` at 0xffed37e0): target 0xffed37f6.
  `rts` at 0xffed37fe has recorded d0 = `unknown`.

- `0xffe6b7d8`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#1`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe6b7d8 to 0xffe6b934 (exclusive), 348 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0xedf24, 1589988 bytes, SHA256 `5df6abeffec6c64b0fecbc68970e31a7dd41c2444993af801114d27a944d4799`.
  Payload: carrier offset 0xedf50, 1589944 bytes, SHA256 `64ad5503da721ec1dd85f37e372b6918629a1fd90ab9d8f990ef870fd86121e7`.
  Read window: SHA256 `663facd789f87610ecdb9bf13112733f7de1992461b3062eda2bfecbfe80f437`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe6b7e4).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe6b7e8).
  Byte load: target `sp +0x13` (`move.b` at 0xffe6b7ec).
  Branch (`beq.b` at 0xffe6b7f4): taken to 0xffe6b7fc when `Z == 1`, falls through to 0xffe6b7f6 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe6b7f8): taken to 0xffe6b814 when `Z == 1`, falls through to 0xffe6b7fa when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe6b7fa): target 0xffe6b82c.
  Grouped accesses: 4 4-byte load records with target `abs 0x300006a0`, including `movea.l` at 0xffe6b802, `movea.l` at 0xffe6b81a, `movea.l` at 0xffe6b900.
  Jump (`bra.b` at 0xffe6b812): target 0xffe6b84e.
  Jump (`bra.b` at 0xffe6b82a): target 0xffe6b84e.
  Call (`jsr` at 0xffe6b844): (unknown callee), target 0xffe6625c, stack arg0=`0x9`, stack arg1=`0xffe6c010`, stack arg2=`0x7c`, stack arg3=`0x0`, stack arg4=`0xffe6c01d`, stack arg5=`[sp+4]`.
  Four-byte load: target `(unknown effective address through a5)` (`tst.l` at 0xffe6b84e).
  Branch (`beq.b` at 0xffe6b850): taken to 0xffe6b874 when `Z == 1`, falls through to 0xffe6b852 when `not (Z == 1)`.
  Call (`jsr` at 0xffe6b86a): (unknown callee), target 0xffe6625c, stack arg0=`0x9`, stack arg1=`0xffe6c010`, stack arg2=`0x80`, stack arg3=`0x0`, stack arg4=`0xffe6c04c`, stack arg5=`[sp+4]`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe6b876).
  Call (`jsr` at 0xffe6b88a): (unknown callee), target 0xffe6dca0, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe6ba8c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6b8a2): (unknown callee), target 0xffe4951c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6b8b0): taken to 0xffe6b8d4 when `ret(0xffe4951c) == 0x0`, falls through to 0xffe6b8b2 when `ret(0xffe4951c) != 0x0`.
  Call (`jsr` at 0xffe6b8ca): (unknown callee), target 0xffe6625c, stack arg0=`0x9`, stack arg1=`0xffe6c010`, stack arg2=`0x8e`, stack arg3=`0x0`, stack arg4=`0xffe6c083`, stack arg5=`ret(0xffe4951c)`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe6dca0)` (`move.l` at 0xffe6b8d4).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe6b8d6).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffe6b8da).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe6b8de).
  Call (`jsr` at 0xffe6b8e4): (unknown callee), target 0xffe6dae4, stack arg0=`ret(0xffe6dca0)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe6b8f2): (unknown callee), target 0xffe6dc54, stack arg0=`ret(0xffe6dca0)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe6b8fe): taken to 0xffe6b916 when `0x1 != [sp+4]`, falls through to 0xffe6b900 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x300006a0] +0x36`, including `move.w` at 0xffe6b906, `move.w` at 0xffe6b91c.
  Grouped accesses: 2 2-byte store records with target `[0x300006a0] +0x36`, reported value `unknown`, including `move.w` at 0xffe6b910, `move.w` at 0xffe6b926.
  Jump (`bra.b` at 0xffe6b914): target 0xffe6b92a.
  Recorded d0 at the return is `unknown` (`rts` at 0xffe6b932).

- `0xfff5b7d8`, `root/6.5.16/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.16:l1#1`, version `1.12.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff5b7d8 to 0xfff5b934 (exclusive), 348 bytes.
  Carrier: 2564616 bytes, SHA256 `7579eea4f23f1d62181cccb8086c045fc1a63d7690ffeeedc8e7fc0885061c4c`.
  Package: carrier offset 0xedf24, 1589988 bytes, SHA256 `5df6abeffec6c64b0fecbc68970e31a7dd41c2444993af801114d27a944d4799`.
  Payload: carrier offset 0xedf50, 1589944 bytes, SHA256 `64ad5503da721ec1dd85f37e372b6918629a1fd90ab9d8f990ef870fd86121e7`.
  Read window: SHA256 `53992a7dbed61cd6e567a900840cad202e9ddcf00b3e8d85cf33743216c32d05`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff5b7e4).
  Four-byte load: target `sp +0x8` (`move.l` at 0xfff5b7e8).
  Byte load: target `sp +0x13` (`move.b` at 0xfff5b7ec).
  Branch (`beq.b` at 0xfff5b7f4): taken to 0xfff5b7fc when `Z == 1`, falls through to 0xfff5b7f6 when `not (Z == 1)`.
  Branch (`beq.b` at 0xfff5b7f8): taken to 0xfff5b814 when `Z == 1`, falls through to 0xfff5b7fa when `not (Z == 1)`.
  Jump (`bra.b` at 0xfff5b7fa): target 0xfff5b82c.
  Grouped accesses: 4 4-byte load records with target `abs 0x300006a0`, including `movea.l` at 0xfff5b802, `movea.l` at 0xfff5b81a, `movea.l` at 0xfff5b900.
  Jump (`bra.b` at 0xfff5b812): target 0xfff5b84e.
  Jump (`bra.b` at 0xfff5b82a): target 0xfff5b84e.
  Call (`jsr` at 0xfff5b844): (unknown callee), target 0xfff5625c, stack arg0=`0x9`, stack arg1=`0xfff5c010`, stack arg2=`0x7c`, stack arg3=`0x0`, stack arg4=`0xfff5c01d`, stack arg5=`[sp+4]`.
  Four-byte load: target `(unknown effective address through a5)` (`tst.l` at 0xfff5b84e).
  Branch (`beq.b` at 0xfff5b850): taken to 0xfff5b874 when `Z == 1`, falls through to 0xfff5b852 when `not (Z == 1)`.
  Call (`jsr` at 0xfff5b86a): (unknown callee), target 0xfff5625c, stack arg0=`0x9`, stack arg1=`0xfff5c010`, stack arg2=`0x80`, stack arg3=`0x0`, stack arg4=`0xfff5c04c`, stack arg5=`[sp+4]`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xfff5b876).
  Call (`jsr` at 0xfff5b88a): (unknown callee), target 0xfff5dca0, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xfff5ba8c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff5b8a2): (unknown callee), target 0xfff3951c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xfff5b8b0): taken to 0xfff5b8d4 when `ret(0xfff3951c) == 0x0`, falls through to 0xfff5b8b2 when `ret(0xfff3951c) != 0x0`.
  Call (`jsr` at 0xfff5b8ca): (unknown callee), target 0xfff5625c, stack arg0=`0x9`, stack arg1=`0xfff5c010`, stack arg2=`0x8e`, stack arg3=`0x0`, stack arg4=`0xfff5c083`, stack arg5=`ret(0xfff3951c)`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xfff5dca0)` (`move.l` at 0xfff5b8d4).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xfff5b8d6).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xfff5b8da).
  Four-byte load: target `sp +0x14` (`move.l` at 0xfff5b8de).
  Call (`jsr` at 0xfff5b8e4): (unknown callee), target 0xfff5dae4, stack arg0=`ret(0xfff5dca0)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xfff5b8f2): (unknown callee), target 0xfff5dc54, stack arg0=`ret(0xfff5dca0)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xfff5b8fe): taken to 0xfff5b916 when `0x1 != [sp+4]`, falls through to 0xfff5b900 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x300006a0] +0x36`, including `move.w` at 0xfff5b906, `move.w` at 0xfff5b91c.
  Grouped accesses: 2 2-byte store records with target `[0x300006a0] +0x36`, reported value `unknown`, including `move.w` at 0xfff5b910, `move.w` at 0xfff5b926.
  Jump (`bra.b` at 0xfff5b914): target 0xfff5b92a.
  At the return (`rts` at 0xfff5b932), d0 is `unknown`.

- `0xffe4eb2c`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe4eb2c to 0xffe4ec6e (exclusive), 322 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `5eb1edc756559dc8b93e033ef6985aed4161d6be9cb282e783edce4cb56fe271`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe4eb38).
  Branch (`beq.b` at 0xffe4eb40): taken to 0xffe4eb48 when `Z == 1`, falls through to 0xffe4eb42 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe4eb44): taken to 0xffe4eb60 when `Z == 1`, falls through to 0xffe4eb46 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe4eb46): target 0xffe4eb78.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000480`, including `movea.l` at 0xffe4eb4e, `movea.l` at 0xffe4eb66, `movea.l` at 0xffe4ec3a.
  Jump (`bra.b` at 0xffe4eb5e): target 0xffe4eb92.
  Jump (`bra.b` at 0xffe4eb76): target 0xffe4eb92.
  Call (`jsr` at 0xffe4eb88): (unknown callee), target 0xffe44544, stack arg0=`0xffe4f088`, stack arg1=`0x66`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffe4eb92, `move.l` at 0xffe4eb96.
  Branch (`beq.b` at 0xffe4eb94): taken to 0xffe4ebb0 when `Z == 1`, falls through to 0xffe4eb96 when `not (Z == 1)`.
  Call (`jsr` at 0xffe4eba6): (unknown callee), target 0xffe44544, stack arg0=`0xffe4f088`, stack arg1=`0x6a`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe4ebb2).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe4ebc2).
  Call (`jsr` at 0xffe4ebc8): (unknown callee), target 0xffe4faac, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe4ec70`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffe4ebdc, `move.b` at 0xffe4ec12.
  Call (`jsr` at 0xffe4ebe2): (unknown callee), target 0xffe39cc8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe4ebf0): taken to 0xffe4ec0c when `ret(0xffe39cc8) == 0x0`, falls through to 0xffe4ebf2 when `ret(0xffe39cc8) != 0x0`.
  Call (`jsr` at 0xffe4ec02): (unknown callee), target 0xffe44544, stack arg0=`0xffe4f088`, stack arg1=`0x78`, stack arg2=`0x1`, stack arg3=`ret(0xffe39cc8)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe4faac)` (`move.l` at 0xffe4ec0c).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe4ec0e).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffe4ec12).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe4ec18).
  Call (`jsr` at 0xffe4ec1e): (unknown callee), target 0xffe50160, stack arg0=`ret(0xffe4faac)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe4ec2c): (unknown callee), target 0xffe50420, stack arg0=`ret(0xffe4faac)`, stack arg1=`0x100`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe4ec38): taken to 0xffe4ec50 when `0x1 != [sp+4]`, falls through to 0xffe4ec3a when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000480] +0x36`, including `move.w` at 0xffe4ec40, `move.w` at 0xffe4ec56.
  Grouped accesses: 2 2-byte store records with target `[0x30000480] +0x36`, reported value `unknown`, including `move.w` at 0xffe4ec4a, `move.w` at 0xffe4ec60.
  Jump (`bra.b` at 0xffe4ec4e): target 0xffe4ec64.
  The exit (`rts` at 0xffe4ec6c) reports d0 = `unknown`.

- `0xffe71ca0`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71ca0 to 0xffe71dd4 (exclusive), 308 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `712917bed9aec89eb6ce6a98da0a2fc454b3b6790ad3f60b9647832ebabc59a2`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71ca8).
  Call (`jsr` at 0xffe71cac): (unknown callee), target 0xffe43ac6, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300002d4`, including `movea.l` at 0xffe71cb8, `movea.l` at 0xffe71cde.
  Byte load: target `[0x300002d4] +0x1c9` (`move.b` at 0xffe71cbc).
  Branch (`beq.b` at 0xffe71cca): taken to 0xffe71ce4 when `unknown == unknown`, falls through to 0xffe71ccc when `unknown != unknown`.
  Branch (`beq.b` at 0xffe71cce): taken to 0xffe71cd8 when `unknown == 0x0`, falls through to 0xffe71cd0 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe71cd6): target 0xffe71cde.
  Byte store: target `[0x300002d4] +0x1c9`, value `unknown` (`move.b` at 0xffe71ce0).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe71ce8).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe71cee).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe71cf4).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe71cfa).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe71d18).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe71d22).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffe71d24).
  Branch (`beq.b` at 0xffe71d2c): taken to 0xffe71d72 when `unknown == 0x0`, falls through to 0xffe71d2e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe71d4a, `move.b` at 0xffe71d54, `move.b` at 0xffe71d66.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffe71d4c, `move.b` at 0xffe71d9c.
  Branch (`bge.b` at 0xffe71d6a): taken to 0xffe71d2e when `ge flags condition (operands unknown)`, falls through to 0xffe71d6c when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffe71d6c, `move.l` at 0xffe71dbe.
  Branch (`bgt.b` at 0xffe71d70): taken to 0xffe71d22 when `gt flags condition (operands unknown)`, falls through to 0xffe71d72 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe71d7a): taken to 0xffe71dc0 when `Z == 1`, falls through to 0xffe71d7c when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe71dbc): taken to 0xffe71d7e when `gt flags condition (operands unknown)`, falls through to 0xffe71dbe when `not (gt flags condition (operands unknown))`.
  `rts` at 0xffe71dd2 has recorded d0 = `0x1`.

- `0xffee1ca0`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#0`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee1ca0 to 0xffee1dd4 (exclusive), 308 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0x0, 977032 bytes, SHA256 `6283673b886c81233a9f84f215a7d1f630470063e05692b31a5c1832b6673ea0`.
  Payload: carrier offset 0x40, 976968 bytes, SHA256 `7c501ba738cbbe28fbfb791df5b3c676a647ff2c02d9c94f9b29c1d2dfe36818`.
  Read window: SHA256 `0640803883929ce5b892c5f1910e2647359cb3015e0fb818640283ce34f339a3`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee1ca8).
  Call (`jsr` at 0xffee1cac): (unknown callee), target 0xffeb3ac6, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300002d4`, including `movea.l` at 0xffee1cb8, `movea.l` at 0xffee1cde.
  Byte load: target `[0x300002d4] +0x1c9` (`move.b` at 0xffee1cbc).
  Branch (`beq.b` at 0xffee1cca): taken to 0xffee1ce4 when `unknown == unknown`, falls through to 0xffee1ccc when `unknown != unknown`.
  Branch (`beq.b` at 0xffee1cce): taken to 0xffee1cd8 when `unknown == 0x0`, falls through to 0xffee1cd0 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffee1cd6): target 0xffee1cde.
  Byte store: target `[0x300002d4] +0x1c9`, value `unknown` (`move.b` at 0xffee1ce0).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffee1ce8).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffee1cee).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffee1cf4).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffee1cfa).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffee1d18).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffee1d22).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffee1d24).
  Branch (`beq.b` at 0xffee1d2c): taken to 0xffee1d72 when `unknown == 0x0`, falls through to 0xffee1d2e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffee1d4a, `move.b` at 0xffee1d54, `move.b` at 0xffee1d66.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffee1d4c, `move.b` at 0xffee1d9c.
  Branch (`bge.b` at 0xffee1d6a): taken to 0xffee1d2e when `ge flags condition (operands unknown)`, falls through to 0xffee1d6c when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffee1d6c, `move.l` at 0xffee1dbe.
  Branch (`bgt.b` at 0xffee1d70): taken to 0xffee1d22 when `gt flags condition (operands unknown)`, falls through to 0xffee1d72 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffee1d7a): taken to 0xffee1dc0 when `Z == 1`, falls through to 0xffee1d7c when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffee1dbc): taken to 0xffee1d7e when `gt flags condition (operands unknown)`, falls through to 0xffee1dbe when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffee1dd2).

- `0xffe830e8`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe830e8 to 0xffe8321c (exclusive), 308 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `6f6c3047f7152f9f3233763efabb97bb6498668fbadc7c241285f75496715832`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe830f0).
  Call (`jsr` at 0xffe830f4): (unknown callee), target 0xffe4acaa, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xffe83100, `movea.l` at 0xffe83126.
  Byte load: target `[0x30000370] +0x1c9` (`move.b` at 0xffe83104).
  Branch (`beq.b` at 0xffe83112): taken to 0xffe8312c when `unknown == unknown`, falls through to 0xffe83114 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe83116): taken to 0xffe83120 when `unknown == 0x0`, falls through to 0xffe83118 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe8311e): target 0xffe83126.
  Byte store: target `[0x30000370] +0x1c9`, value `unknown` (`move.b` at 0xffe83128).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe83130).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe83136).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe8313c).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe83142).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe83160).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe8316a).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffe8316c).
  Branch (`beq.b` at 0xffe83174): taken to 0xffe831ba when `unknown == 0x0`, falls through to 0xffe83176 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe83192, `move.b` at 0xffe8319c, `move.b` at 0xffe831ae.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffe83194, `move.b` at 0xffe831e4.
  Branch (`bge.b` at 0xffe831b2): taken to 0xffe83176 when `ge flags condition (operands unknown)`, falls through to 0xffe831b4 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffe831b4, `move.l` at 0xffe83206.
  Branch (`bgt.b` at 0xffe831b8): taken to 0xffe8316a when `gt flags condition (operands unknown)`, falls through to 0xffe831ba when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe831c2): taken to 0xffe83208 when `Z == 1`, falls through to 0xffe831c4 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe83204): taken to 0xffe831c6 when `gt flags condition (operands unknown)`, falls through to 0xffe83206 when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffe8321a), d0 is `0x1`.

- `0xfff730e8`, `root/6.5.17/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.17:l1#1`, version `1.14.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff730e8 to 0xfff7321c (exclusive), 308 bytes.
  Carrier: 2618232 bytes, SHA256 `bb4b3fc7b3949115ff8d764b7619402b5d1cb4b1edf8427501d9f22099cd9d46`.
  Package: carrier offset 0xee888, 1641200 bytes, SHA256 `0ac30a07afa1c10751d4846e3ba6dc54eed90a97bb9fcc41c64423738eed32e7`.
  Payload: carrier offset 0xee8b4, 1641156 bytes, SHA256 `1b12fda1772b53ee08207fb82e7bbfecf2cb35f468904ae89962ca3474c562d2`.
  Read window: SHA256 `2dab31cbd6b747e0d1c5b95579d99b57b302482ca8562bec161a8b25eedb195d`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff730f0).
  Call (`jsr` at 0xfff730f4): (unknown callee), target 0xfff3acaa, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xfff73100, `movea.l` at 0xfff73126.
  Byte load: target `[0x30000370] +0x1c9` (`move.b` at 0xfff73104).
  Branch (`beq.b` at 0xfff73112): taken to 0xfff7312c when `unknown == unknown`, falls through to 0xfff73114 when `unknown != unknown`.
  Branch (`beq.b` at 0xfff73116): taken to 0xfff73120 when `unknown == 0x0`, falls through to 0xfff73118 when `unknown != 0x0`.
  Jump (`bra.b` at 0xfff7311e): target 0xfff73126.
  Byte store: target `[0x30000370] +0x1c9`, value `unknown` (`move.b` at 0xfff73128).
  Four-byte load: target `sp +0x8` (`move.l` at 0xfff73130).
  Four-byte load: target `sp +0xc` (`move.l` at 0xfff73136).
  Four-byte load: target `sp +0x14` (`move.l` at 0xfff7313c).
  Four-byte load: target `sp +0x10` (`move.l` at 0xfff73142).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xfff73160).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xfff7316a).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xfff7316c).
  Branch (`beq.b` at 0xfff73174): taken to 0xfff731ba when `unknown == 0x0`, falls through to 0xfff73176 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xfff73192, `move.b` at 0xfff7319c, `move.b` at 0xfff731ae.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xfff73194, `move.b` at 0xfff731e4.
  Branch (`bge.b` at 0xfff731b2): taken to 0xfff73176 when `ge flags condition (operands unknown)`, falls through to 0xfff731b4 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xfff731b4, `move.l` at 0xfff73206.
  Branch (`bgt.b` at 0xfff731b8): taken to 0xfff7316a when `gt flags condition (operands unknown)`, falls through to 0xfff731ba when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xfff731c2): taken to 0xfff73208 when `Z == 1`, falls through to 0xfff731c4 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xfff73204): taken to 0xfff731c6 when `gt flags condition (operands unknown)`, falls through to 0xfff73206 when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xfff7321a) reports d0 = `0x1`.

- `0xffe70500`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#0`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe70500 to 0xffe70634 (exclusive), 308 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0x0, 969844 bytes, SHA256 `f54e47c2935cd6cf448928e373ec7a24d3da1265b5f73adec5c3f26f426d278b`.
  Payload: carrier offset 0x40, 969780 bytes, SHA256 `703576f15e0bbcd5be107d70d238c5ded01c115e270ca1be2460b1531e9e24ae`.
  Read window: SHA256 `e9a1dea95d1d7d3376a2fe1ad59bf27bc9a281279e0144fe946733bbc0d8e806`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe70508).
  Call (`jsr` at 0xffe7050c): (unknown callee), target 0xffe40d0e, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000029c`, including `movea.l` at 0xffe70518, `movea.l` at 0xffe7053e.
  Byte load: target `[0x3000029c] +0x1c9` (`move.b` at 0xffe7051c).
  Branch (`beq.b` at 0xffe7052a): taken to 0xffe70544 when `unknown == unknown`, falls through to 0xffe7052c when `unknown != unknown`.
  Branch (`beq.b` at 0xffe7052e): taken to 0xffe70538 when `unknown == 0x0`, falls through to 0xffe70530 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe70536): target 0xffe7053e.
  Byte store: target `[0x3000029c] +0x1c9`, value `unknown` (`move.b` at 0xffe70540).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe70548).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe7054e).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe70554).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe7055a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe70578).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe70582).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffe70584).
  Branch (`beq.b` at 0xffe7058c): taken to 0xffe705d2 when `unknown == 0x0`, falls through to 0xffe7058e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe705aa, `move.b` at 0xffe705b4, `move.b` at 0xffe705c6.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffe705ac, `move.b` at 0xffe705fc.
  Branch (`bge.b` at 0xffe705ca): taken to 0xffe7058e when `ge flags condition (operands unknown)`, falls through to 0xffe705cc when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffe705cc, `move.l` at 0xffe7061e.
  Branch (`bgt.b` at 0xffe705d0): taken to 0xffe70582 when `gt flags condition (operands unknown)`, falls through to 0xffe705d2 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe705da): taken to 0xffe70620 when `Z == 1`, falls through to 0xffe705dc when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe7061c): taken to 0xffe705de when `gt flags condition (operands unknown)`, falls through to 0xffe7061e when `not (gt flags condition (operands unknown))`.
  `rts` at 0xffe70632 has recorded d0 = `0x1`.

- `0xffee0500`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#0`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee0500 to 0xffee0634 (exclusive), 308 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0x0, 969844 bytes, SHA256 `f54e47c2935cd6cf448928e373ec7a24d3da1265b5f73adec5c3f26f426d278b`.
  Payload: carrier offset 0x40, 969780 bytes, SHA256 `703576f15e0bbcd5be107d70d238c5ded01c115e270ca1be2460b1531e9e24ae`.
  Read window: SHA256 `a2a5d6f1ca19a1a56ff81a642de35b8b2425204212a2fa709c13eabcfe8117e2`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee0508).
  Call (`jsr` at 0xffee050c): (unknown callee), target 0xffeb0d0e, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000029c`, including `movea.l` at 0xffee0518, `movea.l` at 0xffee053e.
  Byte load: target `[0x3000029c] +0x1c9` (`move.b` at 0xffee051c).
  Branch (`beq.b` at 0xffee052a): taken to 0xffee0544 when `unknown == unknown`, falls through to 0xffee052c when `unknown != unknown`.
  Branch (`beq.b` at 0xffee052e): taken to 0xffee0538 when `unknown == 0x0`, falls through to 0xffee0530 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffee0536): target 0xffee053e.
  Byte store: target `[0x3000029c] +0x1c9`, value `unknown` (`move.b` at 0xffee0540).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffee0548).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffee054e).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffee0554).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffee055a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffee0578).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffee0582).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffee0584).
  Branch (`beq.b` at 0xffee058c): taken to 0xffee05d2 when `unknown == 0x0`, falls through to 0xffee058e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffee05aa, `move.b` at 0xffee05b4, `move.b` at 0xffee05c6.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffee05ac, `move.b` at 0xffee05fc.
  Branch (`bge.b` at 0xffee05ca): taken to 0xffee058e when `ge flags condition (operands unknown)`, falls through to 0xffee05cc when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffee05cc, `move.l` at 0xffee061e.
  Branch (`bgt.b` at 0xffee05d0): taken to 0xffee0582 when `gt flags condition (operands unknown)`, falls through to 0xffee05d2 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffee05da): taken to 0xffee0620 when `Z == 1`, falls through to 0xffee05dc when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffee061c): taken to 0xffee05de when `gt flags condition (operands unknown)`, falls through to 0xffee061e when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffee0632).

- `0xffe849a4`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe849a4 to 0xffe84ad8 (exclusive), 308 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `d3e3f55cab1a039a93aba412cc81c1d6adb4e96dd5a059ed7c5610a20ef4f990`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe849ac).
  Call (`jsr` at 0xffe849b0): (unknown callee), target 0xffe486c6, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000340`, including `movea.l` at 0xffe849bc, `movea.l` at 0xffe849e2.
  Byte load: target `[0x30000340] +0x1c9` (`move.b` at 0xffe849c0).
  Branch (`beq.b` at 0xffe849ce): taken to 0xffe849e8 when `unknown == unknown`, falls through to 0xffe849d0 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe849d2): taken to 0xffe849dc when `unknown == 0x0`, falls through to 0xffe849d4 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe849da): target 0xffe849e2.
  Byte store: target `[0x30000340] +0x1c9`, value `unknown` (`move.b` at 0xffe849e4).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe849ec).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe849f2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe849f8).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe849fe).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe84a1c).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe84a26).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffe84a28).
  Branch (`beq.b` at 0xffe84a30): taken to 0xffe84a76 when `unknown == 0x0`, falls through to 0xffe84a32 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe84a4e, `move.b` at 0xffe84a58, `move.b` at 0xffe84a6a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffe84a50, `move.b` at 0xffe84aa0.
  Branch (`bge.b` at 0xffe84a6e): taken to 0xffe84a32 when `ge flags condition (operands unknown)`, falls through to 0xffe84a70 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffe84a70, `move.l` at 0xffe84ac2.
  Branch (`bgt.b` at 0xffe84a74): taken to 0xffe84a26 when `gt flags condition (operands unknown)`, falls through to 0xffe84a76 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe84a7e): taken to 0xffe84ac4 when `Z == 1`, falls through to 0xffe84a80 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe84ac0): taken to 0xffe84a82 when `gt flags condition (operands unknown)`, falls through to 0xffe84ac2 when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffe84ad6), d0 is `0x1`.

- `0xfff749a4`, `root/6.5.18/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.18:l1#1`, version `1.16.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff749a4 to 0xfff74ad8 (exclusive), 308 bytes.
  Carrier: 2627188 bytes, SHA256 `496d9a1ec82d7e5d676032cd68630d928119bbcfd111936ed3c960287823f930`.
  Package: carrier offset 0xecc74, 1657344 bytes, SHA256 `a1c8be76d07641312949343ecca5280c8b8ba1203ca1ad4be703d9d8da70a290`.
  Payload: carrier offset 0xecca0, 1657300 bytes, SHA256 `ea5a7a1053b90e7691c97e22f77d65015389a15d2da4c1c7f4ac03f919a52abd`.
  Read window: SHA256 `e7bd55076631c3b2f330efd6f0622af2b705f5f0b5951e08c0025dbf6b7c9565`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff749ac).
  Call (`jsr` at 0xfff749b0): (unknown callee), target 0xfff386c6, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000340`, including `movea.l` at 0xfff749bc, `movea.l` at 0xfff749e2.
  Byte load: target `[0x30000340] +0x1c9` (`move.b` at 0xfff749c0).
  Branch (`beq.b` at 0xfff749ce): taken to 0xfff749e8 when `unknown == unknown`, falls through to 0xfff749d0 when `unknown != unknown`.
  Branch (`beq.b` at 0xfff749d2): taken to 0xfff749dc when `unknown == 0x0`, falls through to 0xfff749d4 when `unknown != 0x0`.
  Jump (`bra.b` at 0xfff749da): target 0xfff749e2.
  Byte store: target `[0x30000340] +0x1c9`, value `unknown` (`move.b` at 0xfff749e4).
  Four-byte load: target `sp +0x8` (`move.l` at 0xfff749ec).
  Four-byte load: target `sp +0xc` (`move.l` at 0xfff749f2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xfff749f8).
  Four-byte load: target `sp +0x10` (`move.l` at 0xfff749fe).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xfff74a1c).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xfff74a26).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xfff74a28).
  Branch (`beq.b` at 0xfff74a30): taken to 0xfff74a76 when `unknown == 0x0`, falls through to 0xfff74a32 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xfff74a4e, `move.b` at 0xfff74a58, `move.b` at 0xfff74a6a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xfff74a50, `move.b` at 0xfff74aa0.
  Branch (`bge.b` at 0xfff74a6e): taken to 0xfff74a32 when `ge flags condition (operands unknown)`, falls through to 0xfff74a70 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xfff74a70, `move.l` at 0xfff74ac2.
  Branch (`bgt.b` at 0xfff74a74): taken to 0xfff74a26 when `gt flags condition (operands unknown)`, falls through to 0xfff74a76 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xfff74a7e): taken to 0xfff74ac4 when `Z == 1`, falls through to 0xfff74a80 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xfff74ac0): taken to 0xfff74a82 when `gt flags condition (operands unknown)`, falls through to 0xfff74ac2 when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xfff74ad6) reports d0 = `0x1`.

- `0xffe71454`, `root/6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71454 to 0xffe71588 (exclusive), 308 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `c11adcb5f200ba32c9c223bed044ef54aa4f7e4f44cda73d71941361c553b279`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7145c).
  Call (`jsr` at 0xffe71460): (unknown callee), target 0xffe41ba2, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xffe7146c, `movea.l` at 0xffe71492.
  Byte load: target `[0x30000370] +0x1c9` (`move.b` at 0xffe71470).
  Branch (`beq.b` at 0xffe7147e): taken to 0xffe71498 when `unknown == unknown`, falls through to 0xffe71480 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe71482): taken to 0xffe7148c when `unknown == 0x0`, falls through to 0xffe71484 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe7148a): target 0xffe71492.
  Byte store: target `[0x30000370] +0x1c9`, value `unknown` (`move.b` at 0xffe71494).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe7149c).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe714a2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe714a8).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe714ae).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe714cc).
  Four-byte load: target `(unknown effective address through a5)` (`move.l` at 0xffe714d6).
  Four-byte load: target `(unknown effective address through a2)` (`move.l` at 0xffe714d8).
  Branch (`beq.b` at 0xffe714e0): taken to 0xffe71526 when `unknown == 0x0`, falls through to 0xffe714e2 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address through a0)`, reported value `unknown`, including `move.b` at 0xffe714fe, `move.b` at 0xffe71508, `move.b` at 0xffe7151a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address through a0)`, including `move.b` at 0xffe71500, `move.b` at 0xffe71550.
  Branch (`bge.b` at 0xffe7151e): taken to 0xffe714e2 when `ge flags condition (operands unknown)`, falls through to 0xffe71520 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address through a1)`, reported value `unknown`, including `move.l` at 0xffe71520, `move.l` at 0xffe71572.
  Branch (`bgt.b` at 0xffe71524): taken to 0xffe714d6 when `gt flags condition (operands unknown)`, falls through to 0xffe71526 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe7152e): taken to 0xffe71574 when `Z == 1`, falls through to 0xffe71530 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe71570): taken to 0xffe71532 when `gt flags condition (operands unknown)`, falls through to 0xffe71572 when `not (gt flags condition (operands unknown))`.
  `rts` at 0xffe71586 has recorded d0 = `0x1`.
