# ColdFire byte stores in IRIX 6.5.27 and later selected images

Each entry identifies an exact carrier, package, payload and byte position. Addresses use the package payload load plus byte offset. The analysis spans are inferred from entry, return and bounded context evidence. Runtime mapping, hardware roles and pointee meanings remain unknown.

Decoding uses the big-endian M68K 68040 common subset. ColdFire classification comes from package metadata and does not identify fitted silicon independently. Separate positions do not establish additional unique functions or physical flash banks.

`sp` names the stack pointer at the analyzed entry. Stored bytes and the full `d0` return expressions are reported separately. The [complete facts](disasm/coldfire-variants/byte-stores-6.5.27-patches-facts.txt) retain frame effects omitted from these summaries. Range ends are exclusive, and caller byte matches do not prove predecessor flow.

- `0xffe72020`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#1`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72020 to 0xffe72036 (exclusive), 22 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0xefb48, 981764 bytes, SHA256 `28f6fdb9ba631c1fc226bf9d30b1812e4d5df62e06959929711f7340d3ed1c1d`.
  Payload: carrier offset 0xefb74, 981720 bytes, SHA256 `38a4b37992f18dfaaf632b9051ebd921251d6776f17c90b93ab353f59589af77`.
  Read window: SHA256 `2413d3cc2dc6c8cd32d551a3b35c7e4fb5eb3c85c4fcbc48a1baeb50ad638afa`.
  Four-byte load: target `abs 0x3000043c` (`movea.l` at 0xffe72024).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7202a).
  Byte store: target `[0x3000043c] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe7202e).
  At the return (`rts` at 0xffe72034), d0 is `[sp+4]`.

- `0xffeb5b94`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#2`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeb5b94 to 0xffeb5baa (exclusive), 22 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0x1df64c, 1906128 bytes, SHA256 `594a95e42bcabd09a02f4606b39227845ec66070af182a311caa4a0489d87398`.
  Payload: carrier offset 0x1df678, 1906084 bytes, SHA256 `c98c5c63dfded2699bcda8f99352e918b27d3a323447ee576e36ab89fe2e31cf`.
  Read window: SHA256 `97a754c1e4e8c0c97f3a64de6533f4c5207eeaf0dc8402c3022394065a28bcaf`.
  Four-byte load: target `abs 0x30000a8c` (`movea.l` at 0xffeb5b98).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffeb5b9e).
  Byte store: target `[0x30000a8c] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffeb5ba2).
  The exit (`rts` at 0xffeb5ba8) reports d0 = `[sp+4]`.

- `0xffe72408`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#0`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72408 to 0xffe7241e (exclusive), 22 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x0, 982424 bytes, SHA256 `1eb7cb1ad9e96ce311f598424e9b1a836c9c04f585798d659a4e1ddefe362d4b`.
  Payload: carrier offset 0x58, 982336 bytes, SHA256 `56e24f817b86ba5d2a0b48c7a322d287e7481a6a1173db19136547872804bf26`.
  Read window: SHA256 `d8cac666ab097c98176536d75bfba92436ccf66293a8c73be2d7f61379c40418`.
  Four-byte load: target `abs 0x300002bc` (`movea.l` at 0xffe7240c).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe72412).
  Byte store: target `[0x300002bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe72416).
  `rts` at 0xffe7241c has recorded d0 = `[sp+4]`.

- `0xffe722c0`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#1`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe722c0 to 0xffe722d6 (exclusive), 22 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0xefd98, 982380 bytes, SHA256 `ea2b4e11ada0feac76aefd64e259195de58fca3acf9ec4184bf2253c4b8b4d08`.
  Payload: carrier offset 0xefdc4, 982336 bytes, SHA256 `ac159ab4f345a13eb60f0a0cca6c1d5860a166315e65af27e019685d97a1f8e0`.
  Read window: SHA256 `3c3fe93a41c4b77e9abc896118377b9d5af483e2cf22089808add3b82a28d301`.
  Four-byte load: target `abs 0x300002e8` (`movea.l` at 0xffe722c4).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe722ca).
  Byte store: target `[0x300002e8] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe722ce).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe722d4).

- `0xffe95218`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#2`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe95218 to 0xffe9522e (exclusive), 22 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x1dfb04, 1754852 bytes, SHA256 `ea36aeea83c9a9aa85c4bc761d80c960851e77a88d69967aa33d58e96f5294f3`.
  Payload: carrier offset 0x1dfb30, 1754808 bytes, SHA256 `4530117d97540ee5a199d8cb8480cd1799ca11b9bf3538d46217ddb13743008a`.
  Read window: SHA256 `89904c2743603d571eeeed97a16ee904f6b71294f88fb23b18913021c2eb7660`.
  Four-byte load: target `abs 0x300008bc` (`movea.l` at 0xffe9521c).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe95222).
  Byte store: target `[0x300008bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe95226).
  At the return (`rts` at 0xffe9522c), d0 is `[sp+4]`.

- `0xffea0df4`, `root/6.5.28/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.28:l1#3`, version `1.38.4`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffea0df4 to 0xffea0e0a (exclusive), 22 bytes.
  Carrier: 9293972 bytes, SHA256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
  Package: carrier offset 0x38c1e8, 1772984 bytes, SHA256 `2094d418e7129ff920233c3f5385fc7bb6bb6e65cbd203164e10250c539b2e1c`.
  Payload: carrier offset 0x38c214, 1772940 bytes, SHA256 `c09284761067f8587cbe2029bca484448b73c1e2c43870af5fd5cb6fa5d0f358`.
  Read window: SHA256 `812066de143c06745575f9265e006f68876cdd35024904ee2608cda34baf2574`.
  Four-byte load: target `abs 0x30000ba8` (`movea.l` at 0xffea0df8).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffea0dfe).
  Byte store: target `[0x30000ba8] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffea0e02).
  The exit (`rts` at 0xffea0e08) reports d0 = `[sp+4]`.

- `0xffe72418`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#0`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72418 to 0xffe7242e (exclusive), 22 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x0, 982440 bytes, SHA256 `d1724d58c910a9bd1510b531e99c15d94ed1b429687131cd78214bc3876a0037`.
  Payload: carrier offset 0x58, 982352 bytes, SHA256 `d36178e16f6b36f7c1f5940320acbd041cd1ebcae18e5fec7b10734086e98a0e`.
  Read window: SHA256 `c85f68eb1ea42612eef3299ed7981f4d2bc2da54e2d10447d878a9efb18d636c`.
  Four-byte load: target `abs 0x3000031c` (`movea.l` at 0xffe7241c).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe72422).
  Byte store: target `[0x3000031c] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe72426).
  `rts` at 0xffe7242c has recorded d0 = `[sp+4]`.

- `0xffe722d0`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#1`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe722d0 to 0xffe722e6 (exclusive), 22 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0xefda8, 982396 bytes, SHA256 `67295cdae5ff370410852805127e6f410676e912b07eab1881c2caca7f70e5c5`.
  Payload: carrier offset 0xefdd4, 982352 bytes, SHA256 `1ba25f46035369eeb52170440fd7a6e8dd4b3ef74024f3e9bbaded05b899f91a`.
  Read window: SHA256 `eb56adc05a7a5c7cfcb224a0cbbb14142744890cdc97770b09a8a332d9cb1911`.
  Four-byte load: target `abs 0x30000348` (`movea.l` at 0xffe722d4).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe722da).
  Byte store: target `[0x30000348] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe722de).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe722e4).

- `0xffe95228`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#2`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe95228 to 0xffe9523e (exclusive), 22 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x1dfb24, 1754868 bytes, SHA256 `728c3b39b0aaebd3f8e0ee49e7c42f0d3877d4b8cff52cb06fc0ea4606612802`.
  Payload: carrier offset 0x1dfb50, 1754824 bytes, SHA256 `6d104cefdb289169aa434cde1e6641c316128c7d81149d27fbe448b172d00459`.
  Read window: SHA256 `c3b13a906c2d36705335dc7cae2ac0885170ef1053dd8d8817f96d8331223517`.
  Four-byte load: target `abs 0x3000092c` (`movea.l` at 0xffe9522c).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe95232).
  Byte store: target `[0x3000092c] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe95236).
  At the return (`rts` at 0xffe9523c), d0 is `[sp+4]`.

- `0xffe9c77c`, `root/6.5.29/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.29:l1#3`, version `1.40.5`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9c77c to 0xffe9c792 (exclusive), 22 bytes.
  Carrier: 9295164 bytes, SHA256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
  Package: carrier offset 0x38c218, 1774128 bytes, SHA256 `17388cb36d1c3b9ac35e33e5964e0641334b6b24dd1f787d7d81d0fe3c494a2c`.
  Payload: carrier offset 0x38c244, 1774084 bytes, SHA256 `2028cd13279e10c398b6e1b86e9db5d8fb7802cdddb4de5912588c8da736ecc0`.
  Read window: SHA256 `812066de143c06745575f9265e006f68876cdd35024904ee2608cda34baf2574`.
  Four-byte load: target `abs 0x30000ba8` (`movea.l` at 0xffe9c780).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9c786).
  Byte store: target `[0x30000ba8] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9c78a).
  The exit (`rts` at 0xffe9c790) reports d0 = `[sp+4]`.

- `0xffe69f44`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#0`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe69f44 to 0xffe69f5a (exclusive), 22 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0x0, 948616 bytes, SHA256 `e8f384de115f495ea87f86c045ab40c27a27d9b7c957bd5d39627c49881eaa59`.
  Payload: carrier offset 0x58, 948528 bytes, SHA256 `0f9d1cee2a1c4c7000fd322ccbb4f09f897551825463f7e123c35239905fbad8`.
  Read window: SHA256 `3d29c6abd4b11ddf48a8648bf717e1c31a418b60a8b34be3c00427529d8c2f5c`.
  Four-byte load: target `abs 0x30000380` (`movea.l` at 0xffe69f48).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe69f4e).
  Byte store: target `[0x30000380] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe69f52).
  `rts` at 0xffe69f58 has recorded d0 = `[sp+4]`.

- `0xffe9c220`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#1`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9c220 to 0xffe9c236 (exclusive), 22 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0xe7988, 1772496 bytes, SHA256 `45223c158690b9ea637f65a9710f4af32dd05bc0bd83dd3fc54216042ee0fd2d`.
  Payload: carrier offset 0xe79b4, 1772452 bytes, SHA256 `25561f492def55deebc646f821a1cd5d9b94277e69988c393dfa3df7f8622d7b`.
  Read window: SHA256 `a859bfb44794dea2e9480057f91910830c01d1eb18002570e0e9763ef2ee89c1`.
  Four-byte load: target `abs 0x30000b50` (`movea.l` at 0xffe9c224).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9c22a).
  Byte store: target `[0x30000b50] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9c22e).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe9c234).

- `0xffe98bdc`, `root/cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `cd-ist-3.24:l1#3`, version `1.62.1`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe98bdc to 0xffe98bf2 (exclusive), 22 bytes.
  Carrier: 13258744 bytes, SHA256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`.
  Package: carrier offset 0x63c654, 1771696 bytes, SHA256 `14b7122d6b6c072d9b4d2425f07ad68041807d535084e8c3501b512e017aa66e`.
  Payload: carrier offset 0x63c680, 1771652 bytes, SHA256 `fdb1a5444e0e11bd6d24bb080b3e5795a2dd340a2146cc1ced74419597d8f339`.
  Read window: SHA256 `f296062bc1c203972a63483de8ebf268b3abcb856e2f8dd17023fde9e10c5e46`.
  Four-byte load: target `abs 0x30000bd4` (`movea.l` at 0xffe98be0).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe98be6).
  Byte store: target `[0x30000bd4] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe98bea).
  At the return (`rts` at 0xffe98bf0), d0 is `[sp+4]`.

- `0xffe71b78`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#0`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71b78 to 0xffe71b8e (exclusive), 22 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0x0, 981788 bytes, SHA256 `cfe50f185db4cbb8da444dd49efedbd662c463ae161559ec6d24a8acbff2c7cb`.
  Payload: carrier offset 0x50, 981708 bytes, SHA256 `330ef3074bdfdbbabcffd95fb2c26ea942118cd33e4029e0277d136d63e11e0f`.
  Read window: SHA256 `51d9d8716a3caab1ddc83186abca88c70a752d748e57750453fbe0f9d908c897`.
  Four-byte load: target `abs 0x300003bc` (`movea.l` at 0xffe71b7c).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71b82).
  Byte store: target `[0x300003bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe71b86).
  The exit (`rts` at 0xffe71b8c) reports d0 = `[sp+4]`.

- `0xffe71a14`, `root/patches/SG0005762/usr/cpu/firmware/sysco/l1.bin`.
  Package `patches/SG0005762:l1#1`, version `1.32.15`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71a14 to 0xffe71a2a (exclusive), 22 bytes.
  Carrier: 7406448 bytes, SHA256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`.
  Package: carrier offset 0xefb1c, 981728 bytes, SHA256 `fdc8eb9f405f0cc5a8eb72dd9e3e579e28e1f72b574c0508639269d722830922`.
  Payload: carrier offset 0xefb48, 981684 bytes, SHA256 `1aa8d9f5f1ebd4a38b084a4f783f6b3c402fa4ec14b3e08051b8bf913c36dcbc`.
  Read window: SHA256 `c92a1f8bdcb8083d78aaf378322606496216b2b880ae318537010e5916f7c0f4`.
  Four-byte load: target `abs 0x300003e8` (`movea.l` at 0xffe71a18).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71a1e).
  Byte store: target `[0x300003e8] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe71a22).
  `rts` at 0xffe71a28 has recorded d0 = `[sp+4]`.
