# ColdFire memory updates in selected images

Each entry describes its exact carrier, package, payload and selected byte position. Addresses are analysis coordinates formed from the stated payload load plus byte offset. Later positions in one payload do not establish additional unique functions, physical flash banks, runtime mapping or image selection. No byte or semantic equivalence between positions is assumed.

The spans are inferred from entry, return and bounded context evidence. They are analyzed windows, not proved function sizes. Hardware roles, pointee meanings, types, fitted silicon and runtime path feasibility remain unknown. ColdFire classification comes from package metadata and decoding uses the 68040 common subset.

`sp` is the stack pointer at the analyzed entry. The six stack slots at each call are candidate argument locations under an assumed m68k-sysv convention. Actual arity and custom conventions remain unknown. A `ret(F)` expression does not identify its producing instruction. Repeated access groups count static shapes and do not establish equal values from separate loads or one unconditional path through different branch arms. Unknown values preserve tool question marks.

The [complete facts](disasm/coldfire-variants/memory-updates-selected-images-facts.txt) retain the complete analyzed windows. Range ends are exclusive. Bounded contexts with clipped instructions or uncertain initial alignment are not proof of predecessor flow or full function extent.

- `0xffe44974`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe44974 to 0xffe44a36 (exclusive), 194 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `344c7bf1d476c62c2776b07c6cd2a70ca46dc43cccbb259a0d3628d4675e2298`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffe4497c).
  Branch (`beq.b` at 0xffe44988): taken to 0xffe449be when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffe4498a when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffe4499a): taken to 0xffe449a0 when `Z == 1`, falls through to 0xffe4499c when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffe4499c).
  Jump (`bra.b` at 0xffe4499e): target 0xffe44996.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffe449a6).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffe449ae).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffe449b8).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffe449be).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffe449c4).
  Grouped accesses: 3 4-byte load records with target `abs 0x30000480`, including `movea.l` at 0xffe449ca, `movea.l` at 0xffe449e6, `movea.l` at 0xffe449ec.
  Byte load: target `[0x30000480] +0x40` (`move.b` at 0xffe449d0).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffe449e0).
  Byte load: target `[0x30000480] +0x40` (`move.b` at 0xffe449f2).
  Byte store: target `[0x30000480] +0x40`, value `unknown` (`move.b` at 0xffe449f6).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffe449fa).
  Branch (`beq.b` at 0xffe44a00): taken to 0xffe44a2c when `[0x301019ac] == 0x0`, falls through to 0xffe44a02 when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffe44a02).
  Branch (`bne.b` at 0xffe44a08): taken to 0xffe44a2c when `[0x301019b4] != 0x0`, falls through to 0xffe44a0a when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffe44a0a, `move.l` at 0xffe44a18.
  Branch (`beq.b` at 0xffe44a10): taken to 0xffe44a2c when `[0x30101998] == 0x0`, falls through to 0xffe44a12 when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffe44a12).
  Call (`bsr.w` at 0xffe44a24): (unknown callee), target 0xffe44cdc, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffe44a34), d0 is `unknown`.

- `0xffeb4974`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeb4974 to 0xffeb4a36 (exclusive), 194 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `344c7bf1d476c62c2776b07c6cd2a70ca46dc43cccbb259a0d3628d4675e2298`.
  Four-byte load: target `abs 0x30101990` (`move.l` at 0xffeb497c).
  Branch (`beq.b` at 0xffeb4988): taken to 0xffeb49be when `[0x30101990] == 0xa1b2c3d4`, falls through to 0xffeb498a when `[0x30101990] != 0xa1b2c3d4`.
  Branch (`beq.b` at 0xffeb499a): taken to 0xffeb49a0 when `Z == 1`, falls through to 0xffeb499c when `not (Z == 1)`.
  Byte store: target `(unknown effective address through a5)`, value `0x0` (`clr.b` at 0xffeb499c).
  Jump (`bra.b` at 0xffeb499e): target 0xffeb4996.
  Four-byte store: target `abs 0x30101990`, value `0xa1b2c3d4` (`move.l` at 0xffeb49a6).
  Two-byte store: target `abs 0x30101994`, value `0x1` (`move.w` at 0xffeb49ae).
  Two-byte store: target `abs 0x30101996`, value `0x4d8` (`move.w` at 0xffeb49b8).
  Four-byte load: target `abs 0x30101800` (`move.l` at 0xffeb49be).
  Four-byte store: target `abs 0x301019b4`, value `[0x30101800]` (`move.l` at 0xffeb49c4).
  Grouped accesses: 3 4-byte load records with target `abs 0x30000480`, including `movea.l` at 0xffeb49ca, `movea.l` at 0xffeb49e6, `movea.l` at 0xffeb49ec.
  Byte load: target `[0x30000480] +0x40` (`move.b` at 0xffeb49d0).
  Four-byte store: target `abs 0x301019ac`, value `unknown` (`move.l` at 0xffeb49e0).
  Byte load: target `[0x30000480] +0x40` (`move.b` at 0xffeb49f2).
  Byte store: target `[0x30000480] +0x40`, value `unknown` (`move.b` at 0xffeb49f6).
  Four-byte load: target `abs 0x301019ac` (`tst.l` at 0xffeb49fa).
  Branch (`beq.b` at 0xffeb4a00): taken to 0xffeb4a2c when `[0x301019ac] == 0x0`, falls through to 0xffeb4a02 when `[0x301019ac] != 0x0`.
  Four-byte load: target `abs 0x301019b4` (`tst.l` at 0xffeb4a02).
  Branch (`bne.b` at 0xffeb4a08): taken to 0xffeb4a2c when `[0x301019b4] != 0x0`, falls through to 0xffeb4a0a when `[0x301019b4] == 0x0`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30101998`, including `tst.l` at 0xffeb4a0a, `move.l` at 0xffeb4a18.
  Branch (`beq.b` at 0xffeb4a10): taken to 0xffeb4a2c when `[0x30101998] == 0x0`, falls through to 0xffeb4a12 when `[0x30101998] != 0x0`.
  Four-byte load: target `abs 0x3010199c` (`move.l` at 0xffeb4a12).
  Call (`bsr.w` at 0xffeb4a24): (unknown callee), target 0xffeb4cdc, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffeb4a34) reports d0 = `unknown`.

- `0xffe4afc0`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe4afc0 to 0xffe4b084 (exclusive), 196 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `3fcaf021c52c01b59eee1fa8944c42176eb64c1b2431d474b861f7ad75d46ddc`.
  Grouped accesses: 6 4-byte load records with target `abs 0x30000530`, including `move.l` at 0xffe4afca, `movea.l` at 0xffe4afda, `movea.l` at 0xffe4aff0.
  Call (`jsr` at 0xffe4afd0): (unknown callee), target 0xffe6c0f4, stack arg0=`[0x30000530]`, stack arg1=`0x0`, stack arg2=`0x8`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `[0x30000530] +0x0`, including `and.l` at 0xffe4afe6, `or.l` at 0xffe4afee.
  Grouped accesses: 2 4-byte store records with target `[0x30000530] +0x0`, reported value `unknown`, including `and.l` at 0xffe4afe6, `or.l` at 0xffe4afee.
  Byte store: target `[0x30000530] +0x1`, value `0x1` (`move.b` at 0xffe4aff8).
  Call (`jsr` at 0xffe4b006): (unknown callee), target 0xffe52a6c, stack arg0=`[0x30000530]`, stack arg1=`0x8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `[0x30000530] +0x7`, value `unknown` (`move.b` at 0xffe4b018).
  Grouped accesses: 2 1-byte load records with target `abs 0x30000528`, including `move.b` at 0xffe4b022, `move.b` at 0xffe4b036.
  Four-byte store: target `(unknown effective address through a0)`, value `0xffe4c224` (`move.l` at 0xffe4b034).
  Byte store: target `abs 0x30000528`, value `unknown` (`move.b` at 0xffe4b03e).
  Byte load: target `[0x30000530] +0x1` (`move.b` at 0xffe4b04c).
  Four-byte store: target `abs 0x3000052c`, value `unknown` (`move.l` at 0xffe4b058).
  Four-byte load: target `abs 0x3000052c` (`add.l` at 0xffe4b064).
  Four-byte store: target `abs 0x30000534`, value `[0x3000052c]+656` (`move.l` at 0xffe4b06a).
  Four-byte store: target `abs 0x30000538`, value `0x200007f8` (`move.l` at 0xffe4b076).
  Call (`bsr.w` at 0xffe4b07c): (unknown callee), target 0xffe4ac84, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `rts` at 0xffe4b082 has recorded d0 = `ret(0xffe4ac84)`.

- `0xffe5713c`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5713c to 0xffe57200 (exclusive), 196 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `507bdad6e8d51751865c10a4a5369a9653706130518ab333a3d5809c2dc287b6`.
  Grouped accesses: 6 4-byte load records with target `abs 0x30000240`, including `move.l` at 0xffe57146, `movea.l` at 0xffe57156, `movea.l` at 0xffe5716c.
  Call (`jsr` at 0xffe5714c): (unknown callee), target 0xffe73bb4, stack arg0=`[0x30000240]`, stack arg1=`0x0`, stack arg2=`0x8`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `[0x30000240] +0x0`, including `and.l` at 0xffe57162, `or.l` at 0xffe5716a.
  Grouped accesses: 2 4-byte store records with target `[0x30000240] +0x0`, reported value `unknown`, including `and.l` at 0xffe57162, `or.l` at 0xffe5716a.
  Byte store: target `[0x30000240] +0x1`, value `0x1` (`move.b` at 0xffe57174).
  Call (`jsr` at 0xffe57182): (unknown callee), target 0xffe58690, stack arg0=`[0x30000240]`, stack arg1=`0x8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `[0x30000240] +0x7`, value `unknown` (`move.b` at 0xffe57194).
  Grouped accesses: 2 1-byte load records with target `abs 0x30000238`, including `move.b` at 0xffe5719e, `move.b` at 0xffe571b2.
  Four-byte store: target `(unknown effective address through a0)`, value `0xffe58630` (`move.l` at 0xffe571b0).
  Byte store: target `abs 0x30000238`, value `unknown` (`move.b` at 0xffe571ba).
  Byte load: target `[0x30000240] +0x1` (`move.b` at 0xffe571c8).
  Four-byte store: target `abs 0x3000023c`, value `unknown` (`move.l` at 0xffe571d4).
  Four-byte load: target `abs 0x3000023c` (`add.l` at 0xffe571e0).
  Four-byte store: target `abs 0x30000248`, value `[0x3000023c]+656` (`move.l` at 0xffe571e6).
  Four-byte store: target `abs 0x3000024c`, value `0x200007f8` (`move.l` at 0xffe571f2).
  Call (`bsr.w` at 0xffe571f8): (unknown callee), target 0xffe56b3c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded d0 at the return is `ret(0xffe56b3c)` (`rts` at 0xffe571fe).

- `0xffebafc0`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffebafc0 to 0xffebb084 (exclusive), 196 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `7c5fbbed1ed1b1324a342c9de26bf64b916b414f4cc272d7dd909d3e901f0e45`.
  Grouped accesses: 6 4-byte load records with target `abs 0x30000530`, including `move.l` at 0xffebafca, `movea.l` at 0xffebafda, `movea.l` at 0xffebaff0.
  Call (`jsr` at 0xffebafd0): (unknown callee), target 0xffedc0f4, stack arg0=`[0x30000530]`, stack arg1=`0x0`, stack arg2=`0x8`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `[0x30000530] +0x0`, including `and.l` at 0xffebafe6, `or.l` at 0xffebafee.
  Grouped accesses: 2 4-byte store records with target `[0x30000530] +0x0`, reported value `unknown`, including `and.l` at 0xffebafe6, `or.l` at 0xffebafee.
  Byte store: target `[0x30000530] +0x1`, value `0x1` (`move.b` at 0xffebaff8).
  Call (`jsr` at 0xffebb006): (unknown callee), target 0xffec2a6c, stack arg0=`[0x30000530]`, stack arg1=`0x8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `[0x30000530] +0x7`, value `unknown` (`move.b` at 0xffebb018).
  Grouped accesses: 2 1-byte load records with target `abs 0x30000528`, including `move.b` at 0xffebb022, `move.b` at 0xffebb036.
  Four-byte store: target `(unknown effective address through a0)`, value `0xffebc224` (`move.l` at 0xffebb034).
  Byte store: target `abs 0x30000528`, value `unknown` (`move.b` at 0xffebb03e).
  Byte load: target `[0x30000530] +0x1` (`move.b` at 0xffebb04c).
  Four-byte store: target `abs 0x3000052c`, value `unknown` (`move.l` at 0xffebb058).
  Four-byte load: target `abs 0x3000052c` (`add.l` at 0xffebb064).
  Four-byte store: target `abs 0x30000534`, value `[0x3000052c]+656` (`move.l` at 0xffebb06a).
  Four-byte store: target `abs 0x30000538`, value `0x200007f8` (`move.l` at 0xffebb076).
  Call (`bsr.w` at 0xffebb07c): (unknown callee), target 0xffebac84, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`rts` at 0xffebb082), d0 is `ret(0xffebac84)`.

- `0xffec713c`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec713c to 0xffec7200 (exclusive), 196 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `2af9ec36c50c579ff33002d2eb110cd33f0c8f4b43ddb9d4a11112b3e180e537`.
  Grouped accesses: 6 4-byte load records with target `abs 0x30000240`, including `move.l` at 0xffec7146, `movea.l` at 0xffec7156, `movea.l` at 0xffec716c.
  Call (`jsr` at 0xffec714c): (unknown callee), target 0xffee3bb4, stack arg0=`[0x30000240]`, stack arg1=`0x0`, stack arg2=`0x8`, stack arg3=`a6`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `[0x30000240] +0x0`, including `and.l` at 0xffec7162, `or.l` at 0xffec716a.
  Grouped accesses: 2 4-byte store records with target `[0x30000240] +0x0`, reported value `unknown`, including `and.l` at 0xffec7162, `or.l` at 0xffec716a.
  Byte store: target `[0x30000240] +0x1`, value `0x1` (`move.b` at 0xffec7174).
  Call (`jsr` at 0xffec7182): (unknown callee), target 0xffec8690, stack arg0=`[0x30000240]`, stack arg1=`0x8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte store: target `[0x30000240] +0x7`, value `unknown` (`move.b` at 0xffec7194).
  Grouped accesses: 2 1-byte load records with target `abs 0x30000238`, including `move.b` at 0xffec719e, `move.b` at 0xffec71b2.
  Four-byte store: target `(unknown effective address through a0)`, value `0xffec8630` (`move.l` at 0xffec71b0).
  Byte store: target `abs 0x30000238`, value `unknown` (`move.b` at 0xffec71ba).
  Byte load: target `[0x30000240] +0x1` (`move.b` at 0xffec71c8).
  Four-byte store: target `abs 0x3000023c`, value `unknown` (`move.l` at 0xffec71d4).
  Four-byte load: target `abs 0x3000023c` (`add.l` at 0xffec71e0).
  Four-byte store: target `abs 0x30000248`, value `[0x3000023c]+656` (`move.l` at 0xffec71e6).
  Four-byte store: target `abs 0x3000024c`, value `0x200007f8` (`move.l` at 0xffec71f2).
  Call (`bsr.w` at 0xffec71f8): (unknown callee), target 0xffec6b3c, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`rts` at 0xffec71fe) reports d0 = `ret(0xffec6b3c)`.

- `0xffe55120`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe55120 to 0xffe55262 (exclusive), 322 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `1e4221b8f3fc5e25d41b453613968990db30875d40190701ba8c88f060130fc2`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe5512c).
  Branch (`beq.b` at 0xffe55134): taken to 0xffe5513c when `Z == 1`, falls through to 0xffe55136 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe55138): taken to 0xffe55154 when `Z == 1`, falls through to 0xffe5513a when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe5513a): target 0xffe5516c.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000490`, including `movea.l` at 0xffe55142, `movea.l` at 0xffe5515a, `movea.l` at 0xffe5522e.
  Jump (`bra.b` at 0xffe55152): target 0xffe55186.
  Jump (`bra.b` at 0xffe5516a): target 0xffe55186.
  Call (`jsr` at 0xffe5517c): (unknown callee), target 0xffe4c250, stack arg0=`0xffe559d8`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffe55186, `move.l` at 0xffe5518a.
  Branch (`beq.b` at 0xffe55188): taken to 0xffe551a4 when `Z == 1`, falls through to 0xffe5518a when `not (Z == 1)`.
  Call (`jsr` at 0xffe5519a): (unknown callee), target 0xffe4c250, stack arg0=`0xffe559d8`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe551a6).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe551b6).
  Call (`jsr` at 0xffe551bc): (unknown callee), target 0xffe563fc, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe55264`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffe551d0, `move.b` at 0xffe55206.
  Call (`jsr` at 0xffe551d6): (unknown callee), target 0xffe401e4, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe551e4): taken to 0xffe55200 when `ret(0xffe401e4) == 0x0`, falls through to 0xffe551e6 when `ret(0xffe401e4) != 0x0`.
  Call (`jsr` at 0xffe551f6): (unknown callee), target 0xffe4c250, stack arg0=`0xffe559d8`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffe401e4)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe563fc)` (`move.l` at 0xffe55200).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe55202).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffe55206).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe5520c).
  Call (`jsr` at 0xffe55212): (unknown callee), target 0xffe56abc, stack arg0=`ret(0xffe563fc)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe55220): (unknown callee), target 0xffe56d80, stack arg0=`ret(0xffe563fc)`, stack arg1=`0x100`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5522c): taken to 0xffe55244 when `0x1 != [sp+4]`, falls through to 0xffe5522e when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000490] +0x36`, including `move.w` at 0xffe55234, `move.w` at 0xffe5524a.
  Grouped accesses: 2 2-byte store records with target `[0x30000490] +0x36`, reported value `unknown`, including `move.w` at 0xffe5523e, `move.w` at 0xffe55254.
  Jump (`bra.b` at 0xffe55242): target 0xffe55258.
  `rts` at 0xffe55260 has recorded d0 = `unknown`.

- `0xffec5120`, `root/6.5.10/usr/cpu/firmware/l1.bin`.
  Package `6.5.10:l1#0`, version `1.1.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec5120 to 0xffec5262 (exclusive), 322 bytes.
  Carrier: 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Package: carrier offset 0x0, 961604 bytes, SHA256 `75cbe6e87e2e84fca517ce27a11391c362057ea1a8b3b3e994ce069994720a9a`.
  Payload: carrier offset 0x28, 961564 bytes, SHA256 `5b9d6771ee2405c5d556304b713f1a6b1deaa9bfd49009cd21856cd266eb3ef9`.
  Read window: SHA256 `8b0e5f0224783a227fc25fa135ba6387ab845e4791173ab97a09935fffac784a`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffec512c).
  Branch (`beq.b` at 0xffec5134): taken to 0xffec513c when `Z == 1`, falls through to 0xffec5136 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffec5138): taken to 0xffec5154 when `Z == 1`, falls through to 0xffec513a when `not (Z == 1)`.
  Jump (`bra.b` at 0xffec513a): target 0xffec516c.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000490`, including `movea.l` at 0xffec5142, `movea.l` at 0xffec515a, `movea.l` at 0xffec522e.
  Jump (`bra.b` at 0xffec5152): target 0xffec5186.
  Jump (`bra.b` at 0xffec516a): target 0xffec5186.
  Call (`jsr` at 0xffec517c): (unknown callee), target 0xffebc250, stack arg0=`0xffec59d8`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffec5186, `move.l` at 0xffec518a.
  Branch (`beq.b` at 0xffec5188): taken to 0xffec51a4 when `Z == 1`, falls through to 0xffec518a when `not (Z == 1)`.
  Call (`jsr` at 0xffec519a): (unknown callee), target 0xffebc250, stack arg0=`0xffec59d8`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffec51a6).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffec51b6).
  Call (`jsr` at 0xffec51bc): (unknown callee), target 0xffec63fc, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffec5264`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffec51d0, `move.b` at 0xffec5206.
  Call (`jsr` at 0xffec51d6): (unknown callee), target 0xffeb01e4, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffec51e4): taken to 0xffec5200 when `ret(0xffeb01e4) == 0x0`, falls through to 0xffec51e6 when `ret(0xffeb01e4) != 0x0`.
  Call (`jsr` at 0xffec51f6): (unknown callee), target 0xffebc250, stack arg0=`0xffec59d8`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffeb01e4)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffec63fc)` (`move.l` at 0xffec5200).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffec5202).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffec5206).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffec520c).
  Call (`jsr` at 0xffec5212): (unknown callee), target 0xffec6abc, stack arg0=`ret(0xffec63fc)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffec5220): (unknown callee), target 0xffec6d80, stack arg0=`ret(0xffec63fc)`, stack arg1=`0x100`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffec522c): taken to 0xffec5244 when `0x1 != [sp+4]`, falls through to 0xffec522e when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000490] +0x36`, including `move.w` at 0xffec5234, `move.w` at 0xffec524a.
  Grouped accesses: 2 2-byte store records with target `[0x30000490] +0x36`, reported value `unknown`, including `move.w` at 0xffec523e, `move.w` at 0xffec5254.
  Jump (`bra.b` at 0xffec5242): target 0xffec5258.
  Recorded d0 at the return is `unknown` (`rts` at 0xffec5260).

- `0xffe5e2ec`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5e2ec to 0xffe5e42e (exclusive), 322 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `1983c70eeee506d669d3e2873bcb07ad46c627e659db6a7d385f6d7f9c158dfc`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe5e2f8).
  Branch (`beq.b` at 0xffe5e300): taken to 0xffe5e308 when `Z == 1`, falls through to 0xffe5e302 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe5e304): taken to 0xffe5e320 when `Z == 1`, falls through to 0xffe5e306 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe5e306): target 0xffe5e338.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000198`, including `movea.l` at 0xffe5e30e, `movea.l` at 0xffe5e326, `movea.l` at 0xffe5e3fa.
  Jump (`bra.b` at 0xffe5e31e): target 0xffe5e352.
  Jump (`bra.b` at 0xffe5e336): target 0xffe5e352.
  Call (`jsr` at 0xffe5e348): (unknown callee), target 0xffe53984, stack arg0=`0xffe5eba4`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffe5e352, `move.l` at 0xffe5e356.
  Branch (`beq.b` at 0xffe5e354): taken to 0xffe5e370 when `Z == 1`, falls through to 0xffe5e356 when `not (Z == 1)`.
  Call (`jsr` at 0xffe5e366): (unknown callee), target 0xffe53984, stack arg0=`0xffe5eba4`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe5e372).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe5e382).
  Call (`jsr` at 0xffe5e388): (unknown callee), target 0xffe60510, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe5e430`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffe5e39c, `move.b` at 0xffe5e3d2.
  Call (`jsr` at 0xffe5e3a2): (unknown callee), target 0xffe47644, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5e3b0): taken to 0xffe5e3cc when `ret(0xffe47644) == 0x0`, falls through to 0xffe5e3b2 when `ret(0xffe47644) != 0x0`.
  Call (`jsr` at 0xffe5e3c2): (unknown callee), target 0xffe53984, stack arg0=`0xffe5eba4`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffe47644)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe60510)` (`move.l` at 0xffe5e3cc).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe5e3ce).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffe5e3d2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe5e3d8).
  Call (`jsr` at 0xffe5e3de): (unknown callee), target 0xffe60a7c, stack arg0=`ret(0xffe60510)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5e3ec): (unknown callee), target 0xffe60d18, stack arg0=`ret(0xffe60510)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5e3f8): taken to 0xffe5e410 when `0x1 != [sp+4]`, falls through to 0xffe5e3fa when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000198] +0x36`, including `move.w` at 0xffe5e400, `move.w` at 0xffe5e416.
  Grouped accesses: 2 2-byte store records with target `[0x30000198] +0x36`, reported value `unknown`, including `move.w` at 0xffe5e40a, `move.w` at 0xffe5e420.
  Jump (`bra.b` at 0xffe5e40e): target 0xffe5e424.
  At the return (`rts` at 0xffe5e42c), d0 is `unknown`.

- `0xffece2ec`, `root/6.5.12/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.12:l1#0`, version `1.4.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffece2ec to 0xffece42e (exclusive), 322 bytes.
  Carrier: 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Package: carrier offset 0x0, 947836 bytes, SHA256 `8a6b608cdd4b28ab4758020afbcda5eea94429ffcc92d12fa401a6db41254add`.
  Payload: carrier offset 0x28, 947796 bytes, SHA256 `7084835eb61932052dc28531fe420d397a42df805f59b49d47bec8a95f6dcc93`.
  Read window: SHA256 `5adb97ec306f1d829671870174c42abccb735c6de7316505debeffa0c6e0eec8`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffece2f8).
  Branch (`beq.b` at 0xffece300): taken to 0xffece308 when `Z == 1`, falls through to 0xffece302 when `not (Z == 1)`.
  Branch (`beq.b` at 0xffece304): taken to 0xffece320 when `Z == 1`, falls through to 0xffece306 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffece306): target 0xffece338.
  Grouped accesses: 4 4-byte load records with target `abs 0x30000198`, including `movea.l` at 0xffece30e, `movea.l` at 0xffece326, `movea.l` at 0xffece3fa.
  Jump (`bra.b` at 0xffece31e): target 0xffece352.
  Jump (`bra.b` at 0xffece336): target 0xffece352.
  Call (`jsr` at 0xffece348): (unknown callee), target 0xffec3984, stack arg0=`0xffeceba4`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffece352, `move.l` at 0xffece356.
  Branch (`beq.b` at 0xffece354): taken to 0xffece370 when `Z == 1`, falls through to 0xffece356 when `not (Z == 1)`.
  Call (`jsr` at 0xffece366): (unknown callee), target 0xffec3984, stack arg0=`0xffeceba4`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffece372).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffece382).
  Call (`jsr` at 0xffece388): (unknown callee), target 0xffed0510, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffece430`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffece39c, `move.b` at 0xffece3d2.
  Call (`jsr` at 0xffece3a2): (unknown callee), target 0xffeb7644, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffece3b0): taken to 0xffece3cc when `ret(0xffeb7644) == 0x0`, falls through to 0xffece3b2 when `ret(0xffeb7644) != 0x0`.
  Call (`jsr` at 0xffece3c2): (unknown callee), target 0xffec3984, stack arg0=`0xffeceba4`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffeb7644)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffed0510)` (`move.l` at 0xffece3cc).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffece3ce).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffece3d2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffece3d8).
  Call (`jsr` at 0xffece3de): (unknown callee), target 0xffed0a7c, stack arg0=`ret(0xffed0510)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffece3ec): (unknown callee), target 0xffed0d18, stack arg0=`ret(0xffed0510)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffece3f8): taken to 0xffece410 when `0x1 != [sp+4]`, falls through to 0xffece3fa when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x30000198] +0x36`, including `move.w` at 0xffece400, `move.w` at 0xffece416.
  Grouped accesses: 2 2-byte store records with target `[0x30000198] +0x36`, reported value `unknown`, including `move.w` at 0xffece40a, `move.w` at 0xffece420.
  Jump (`bra.b` at 0xffece40e): target 0xffece424.
  The exit (`rts` at 0xffece42c) reports d0 = `unknown`.

- `0xffe601d8`, `root/6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe601d8 to 0xffe6031a (exclusive), 322 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `3cc4182eeb814243e74fc3bebb25b7778e5fd9c73b550fd52799cc8778c67479`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe601e4).
  Branch (`beq.b` at 0xffe601ec): taken to 0xffe601f4 when `Z == 1`, falls through to 0xffe601ee when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe601f0): taken to 0xffe6020c when `Z == 1`, falls through to 0xffe601f2 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe601f2): target 0xffe60224.
  Grouped accesses: 4 4-byte load records with target `abs 0x300002ac`, including `movea.l` at 0xffe601fa, `movea.l` at 0xffe60212, `movea.l` at 0xffe602e6.
  Jump (`bra.b` at 0xffe6020a): target 0xffe6023e.
  Jump (`bra.b` at 0xffe60222): target 0xffe6023e.
  Call (`jsr` at 0xffe60234): (unknown callee), target 0xffe5a1b8, stack arg0=`0xffe60a90`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffe6023e, `move.l` at 0xffe60242.
  Branch (`beq.b` at 0xffe60240): taken to 0xffe6025c when `Z == 1`, falls through to 0xffe60242 when `not (Z == 1)`.
  Call (`jsr` at 0xffe60252): (unknown callee), target 0xffe5a1b8, stack arg0=`0xffe60a90`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe6025e).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe6026e).
  Call (`jsr` at 0xffe60274): (unknown callee), target 0xffe624ec, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe6031c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffe60288, `move.b` at 0xffe602be.
  Call (`jsr` at 0xffe6028e): (unknown callee), target 0xffe45924, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe6029c): taken to 0xffe602b8 when `ret(0xffe45924) == 0x0`, falls through to 0xffe6029e when `ret(0xffe45924) != 0x0`.
  Call (`jsr` at 0xffe602ae): (unknown callee), target 0xffe5a1b8, stack arg0=`0xffe60a90`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffe45924)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe624ec)` (`move.l` at 0xffe602b8).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe602ba).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffe602be).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe602c4).
  Call (`jsr` at 0xffe602ca): (unknown callee), target 0xffe62a54, stack arg0=`ret(0xffe624ec)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe602d8): (unknown callee), target 0xffe62cf8, stack arg0=`ret(0xffe624ec)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe602e4): taken to 0xffe602fc when `0x1 != [sp+4]`, falls through to 0xffe602e6 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x300002ac] +0x36`, including `move.w` at 0xffe602ec, `move.w` at 0xffe60302.
  Grouped accesses: 2 2-byte store records with target `[0x300002ac] +0x36`, reported value `unknown`, including `move.w` at 0xffe602f6, `move.w` at 0xffe6030c.
  Jump (`bra.b` at 0xffe602fa): target 0xffe60310.
  `rts` at 0xffe60318 has recorded d0 = `unknown`.

- `0xffed01d8`, `root/6.5.13/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.13:l1#0`, version `1.6.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffed01d8 to 0xffed031a (exclusive), 322 bytes.
  Carrier: 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Package: carrier offset 0x0, 957104 bytes, SHA256 `b69671ddfc246ad55cf3162af0b9b86652ce74eba10cf9325a3fef8ee1d96fdd`.
  Payload: carrier offset 0x28, 957064 bytes, SHA256 `6e560e68a673766d56d716333f39ae79dca4b8779b59c7c97ef02741c822611d`.
  Read window: SHA256 `b36b8c5008fcf735a1d702d49a7296ccfa1f2906bc4ffa0931238ea7d6a5d753`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffed01e4).
  Branch (`beq.b` at 0xffed01ec): taken to 0xffed01f4 when `Z == 1`, falls through to 0xffed01ee when `not (Z == 1)`.
  Branch (`beq.b` at 0xffed01f0): taken to 0xffed020c when `Z == 1`, falls through to 0xffed01f2 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffed01f2): target 0xffed0224.
  Grouped accesses: 4 4-byte load records with target `abs 0x300002ac`, including `movea.l` at 0xffed01fa, `movea.l` at 0xffed0212, `movea.l` at 0xffed02e6.
  Jump (`bra.b` at 0xffed020a): target 0xffed023e.
  Jump (`bra.b` at 0xffed0222): target 0xffed023e.
  Call (`jsr` at 0xffed0234): (unknown callee), target 0xffeca1b8, stack arg0=`0xffed0a90`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d6`, stack arg5=`d7`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffed023e, `move.l` at 0xffed0242.
  Branch (`beq.b` at 0xffed0240): taken to 0xffed025c when `Z == 1`, falls through to 0xffed0242 when `not (Z == 1)`.
  Call (`jsr` at 0xffed0252): (unknown callee), target 0xffeca1b8, stack arg0=`0xffed0a90`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffed025e).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffed026e).
  Call (`jsr` at 0xffed0274): (unknown callee), target 0xffed24ec, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffed031c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `sp +0x13`, including `move.b` at 0xffed0288, `move.b` at 0xffed02be.
  Call (`jsr` at 0xffed028e): (unknown callee), target 0xffeb5924, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffed029c): taken to 0xffed02b8 when `ret(0xffeb5924) == 0x0`, falls through to 0xffed029e when `ret(0xffeb5924) != 0x0`.
  Call (`jsr` at 0xffed02ae): (unknown callee), target 0xffeca1b8, stack arg0=`0xffed0a90`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffeb5924)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffed24ec)` (`move.l` at 0xffed02b8).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffed02ba).
  Byte store: target `(unknown effective address through a5)`, value `([sp+19]:1 & 0xff)` (`move.b` at 0xffed02be).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffed02c4).
  Call (`jsr` at 0xffed02ca): (unknown callee), target 0xffed2a54, stack arg0=`ret(0xffed24ec)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffed02d8): (unknown callee), target 0xffed2cf8, stack arg0=`ret(0xffed24ec)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffed02e4): taken to 0xffed02fc when `0x1 != [sp+4]`, falls through to 0xffed02e6 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x300002ac] +0x36`, including `move.w` at 0xffed02ec, `move.w` at 0xffed0302.
  Grouped accesses: 2 2-byte store records with target `[0x300002ac] +0x36`, reported value `unknown`, including `move.w` at 0xffed02f6, `move.w` at 0xffed030c.
  Jump (`bra.b` at 0xffed02fa): target 0xffed0310.
  Recorded d0 at the return is `unknown` (`rts` at 0xffed0318).

- `0xffe5f400`, `root/6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe5f400 to 0xffe5f544 (exclusive), 324 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `96a16dea2ff6944ec413b5b07bf3bea4de7424a024cd489fff4d8c8b9971c052`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe5f40c).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe5f410).
  Byte load: target `sp +0x13` (`move.b` at 0xffe5f414).
  Branch (`beq.b` at 0xffe5f41c): taken to 0xffe5f424 when `Z == 1`, falls through to 0xffe5f41e when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe5f420): taken to 0xffe5f43c when `Z == 1`, falls through to 0xffe5f422 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe5f422): target 0xffe5f454.
  Grouped accesses: 4 4-byte load records with target `abs 0x3000074c`, including `movea.l` at 0xffe5f42a, `movea.l` at 0xffe5f442, `movea.l` at 0xffe5f510.
  Jump (`bra.b` at 0xffe5f43a): target 0xffe5f46e.
  Jump (`bra.b` at 0xffe5f452): target 0xffe5f46e.
  Call (`jsr` at 0xffe5f464): (unknown callee), target 0xffe5b618, stack arg0=`0xffe5fc20`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d5`, stack arg5=`d6`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffe5f46e, `move.l` at 0xffe5f472.
  Branch (`beq.b` at 0xffe5f470): taken to 0xffe5f48c when `Z == 1`, falls through to 0xffe5f472 when `not (Z == 1)`.
  Call (`jsr` at 0xffe5f482): (unknown callee), target 0xffe5b618, stack arg0=`0xffe5fc20`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe5f48e).
  Call (`jsr` at 0xffe5f4a2): (unknown callee), target 0xffe617e4, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe5f69c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5f4ba): (unknown callee), target 0xffe423d0, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe5f4c8): taken to 0xffe5f4e4 when `ret(0xffe423d0) == 0x0`, falls through to 0xffe5f4ca when `ret(0xffe423d0) != 0x0`.
  Call (`jsr` at 0xffe5f4da): (unknown callee), target 0xffe5b618, stack arg0=`0xffe5fc20`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffe423d0)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe617e4)` (`move.l` at 0xffe5f4e4).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe5f4e6).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffe5f4ea).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe5f4ee).
  Call (`jsr` at 0xffe5f4f4): (unknown callee), target 0xffe61640, stack arg0=`ret(0xffe617e4)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe5f502): (unknown callee), target 0xffe61798, stack arg0=`ret(0xffe617e4)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe5f50e): taken to 0xffe5f526 when `0x1 != [sp+4]`, falls through to 0xffe5f510 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x3000074c] +0x36`, including `move.w` at 0xffe5f516, `move.w` at 0xffe5f52c.
  Grouped accesses: 2 2-byte store records with target `[0x3000074c] +0x36`, reported value `unknown`, including `move.w` at 0xffe5f520, `move.w` at 0xffe5f536.
  Jump (`bra.b` at 0xffe5f524): target 0xffe5f53a.
  At the return (`rts` at 0xffe5f542), d0 is `unknown`.

- `0xffecf400`, `root/6.5.14/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.14:l1#0`, version `1.8.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffecf400 to 0xffecf544 (exclusive), 324 bytes.
  Carrier: 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Package: carrier offset 0x0, 954016 bytes, SHA256 `e94bee926c0a769369e3079b4f1ba44e2815a4d45075636cf4cd5c9b2c86d0ed`.
  Payload: carrier offset 0x2c, 953972 bytes, SHA256 `9120c5175bdb5ffde86516d75920c95fc9abe5afa5dbabd8a6e38f15595fe4fd`.
  Read window: SHA256 `2385fd721377990b4e4bd225162df380da90cba450b167eee706d2d1d8f9c92e`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffecf40c).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffecf410).
  Byte load: target `sp +0x13` (`move.b` at 0xffecf414).
  Branch (`beq.b` at 0xffecf41c): taken to 0xffecf424 when `Z == 1`, falls through to 0xffecf41e when `not (Z == 1)`.
  Branch (`beq.b` at 0xffecf420): taken to 0xffecf43c when `Z == 1`, falls through to 0xffecf422 when `not (Z == 1)`.
  Jump (`bra.b` at 0xffecf422): target 0xffecf454.
  Grouped accesses: 4 4-byte load records with target `abs 0x3000074c`, including `movea.l` at 0xffecf42a, `movea.l` at 0xffecf442, `movea.l` at 0xffecf510.
  Jump (`bra.b` at 0xffecf43a): target 0xffecf46e.
  Jump (`bra.b` at 0xffecf452): target 0xffecf46e.
  Call (`jsr` at 0xffecf464): (unknown callee), target 0xffecb618, stack arg0=`0xffecfc20`, stack arg1=`0x7c`, stack arg2=`0x1`, stack arg3=`[sp+4]`, stack arg4=`d5`, stack arg5=`d6`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address through a5)`, including `tst.l` at 0xffecf46e, `move.l` at 0xffecf472.
  Branch (`beq.b` at 0xffecf470): taken to 0xffecf48c when `Z == 1`, falls through to 0xffecf472 when `not (Z == 1)`.
  Call (`jsr` at 0xffecf482): (unknown callee), target 0xffecb618, stack arg0=`0xffecfc20`, stack arg1=`0x80`, stack arg2=`0x1`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffecf48e).
  Call (`jsr` at 0xffecf4a2): (unknown callee), target 0xffed17e4, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffecf69c`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffecf4ba): (unknown callee), target 0xffeb23d0, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffecf4c8): taken to 0xffecf4e4 when `ret(0xffeb23d0) == 0x0`, falls through to 0xffecf4ca when `ret(0xffeb23d0) != 0x0`.
  Call (`jsr` at 0xffecf4da): (unknown callee), target 0xffecb618, stack arg0=`0xffecfc20`, stack arg1=`0x8e`, stack arg2=`0x1`, stack arg3=`ret(0xffeb23d0)`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffed17e4)` (`move.l` at 0xffecf4e4).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffecf4e6).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffecf4ea).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffecf4ee).
  Call (`jsr` at 0xffecf4f4): (unknown callee), target 0xffed1640, stack arg0=`ret(0xffed17e4)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffecf502): (unknown callee), target 0xffed1798, stack arg0=`ret(0xffed17e4)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffecf50e): taken to 0xffecf526 when `0x1 != [sp+4]`, falls through to 0xffecf510 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x3000074c] +0x36`, including `move.w` at 0xffecf516, `move.w` at 0xffecf52c.
  Grouped accesses: 2 2-byte store records with target `[0x3000074c] +0x36`, reported value `unknown`, including `move.w` at 0xffecf520, `move.w` at 0xffecf536.
  Jump (`bra.b` at 0xffecf524): target 0xffecf53a.
  The exit (`rts` at 0xffecf542) reports d0 = `unknown`.

- `0xffe631bc`, `root/6.5.15/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.15:l1#0`, version `1.10.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe631bc to 0xffe63318 (exclusive), 348 bytes.
  Carrier: 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Package: carrier offset 0x0, 972844 bytes, SHA256 `5929d259064c8a49a3515ea814b79e20c7134f67bf50860957e143a4974b4101`.
  Payload: carrier offset 0x2c, 972800 bytes, SHA256 `18b09029d3c94d99ff6ddec9215755a6fc4933d212d53830db005768793dea73`.
  Read window: SHA256 `c4cfa7eb0b40b61b846f4d18edcc21c921d62907ed8c283933d408fe74d1f8bb`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe631c8).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe631cc).
  Byte load: target `sp +0x13` (`move.b` at 0xffe631d0).
  Branch (`beq.b` at 0xffe631d8): taken to 0xffe631e0 when `Z == 1`, falls through to 0xffe631da when `not (Z == 1)`.
  Branch (`beq.b` at 0xffe631dc): taken to 0xffe631f8 when `Z == 1`, falls through to 0xffe631de when `not (Z == 1)`.
  Jump (`bra.b` at 0xffe631de): target 0xffe63210.
  Grouped accesses: 4 4-byte load records with target `abs 0x300007d8`, including `movea.l` at 0xffe631e6, `movea.l` at 0xffe631fe, `movea.l` at 0xffe632e4.
  Jump (`bra.b` at 0xffe631f6): target 0xffe63232.
  Jump (`bra.b` at 0xffe6320e): target 0xffe63232.
  Call (`jsr` at 0xffe63228): (unknown callee), target 0xffe583ac, stack arg0=`0x9`, stack arg1=`0xffe639f4`, stack arg2=`0x7c`, stack arg3=`0x0`, stack arg4=`0xffe63a01`, stack arg5=`[sp+4]`.
  Four-byte load: target `(unknown effective address through a5)` (`tst.l` at 0xffe63232).
  Branch (`beq.b` at 0xffe63234): taken to 0xffe63258 when `Z == 1`, falls through to 0xffe63236 when `not (Z == 1)`.
  Call (`jsr` at 0xffe6324e): (unknown callee), target 0xffe583ac, stack arg0=`0x9`, stack arg1=`0xffe639f4`, stack arg2=`0x80`, stack arg3=`0x0`, stack arg4=`0xffe63a30`, stack arg5=`[sp+4]`.
  Four-byte load: target `sp +0xc` (`and.l` at 0xffe6325a).
  Call (`jsr` at 0xffe6326e): (unknown callee), target 0xffe65658, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`0xffe63470`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe63286): (unknown callee), target 0xffe44ba4, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`sp-8`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`beq.b` at 0xffe63294): taken to 0xffe632b8 when `ret(0xffe44ba4) == 0x0`, falls through to 0xffe63296 when `ret(0xffe44ba4) != 0x0`.
  Call (`jsr` at 0xffe632ae): (unknown callee), target 0xffe583ac, stack arg0=`0x9`, stack arg1=`0xffe639f4`, stack arg2=`0x8e`, stack arg3=`0x0`, stack arg4=`0xffe63a67`, stack arg5=`ret(0xffe44ba4)`.
  Four-byte store: target `(unknown effective address through a5)`, value `ret(0xffe65658)` (`move.l` at 0xffe632b8).
  Four-byte store: target `(unknown effective address through a5)`, value `unknown` (`move.l` at 0xffe632ba).
  Byte store: target `(unknown effective address through a5)`, value `unknown` (`move.b` at 0xffe632be).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe632c2).
  Call (`jsr` at 0xffe632c8): (unknown callee), target 0xffe6549c, stack arg0=`ret(0xffe65658)`, stack arg1=`[sp+20]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`jsr` at 0xffe632d6): (unknown callee), target 0xffe6560c, stack arg0=`ret(0xffe65658)`, stack arg1=`0x1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`bne.b` at 0xffe632e2): taken to 0xffe632fa when `0x1 != [sp+4]`, falls through to 0xffe632e4 when `0x1 == [sp+4]`.
  Grouped accesses: 2 2-byte load records with target `[0x300007d8] +0x36`, including `move.w` at 0xffe632ea, `move.w` at 0xffe63300.
  Grouped accesses: 2 2-byte store records with target `[0x300007d8] +0x36`, reported value `unknown`, including `move.w` at 0xffe632f4, `move.w` at 0xffe6330a.
  Jump (`bra.b` at 0xffe632f8): target 0xffe6330e.
  `rts` at 0xffe63316 has recorded d0 = `unknown`.
