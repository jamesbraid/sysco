# ColdFire byte stores in selected IRIX 6.5.9 through 6.5.30 images

Each entry identifies an exact carrier, package, payload and byte position. Addresses use the package payload load plus byte offset. The analysis spans are inferred from entry, return and bounded context evidence. Runtime mapping, hardware roles and pointee meanings remain unknown.

Decoding uses the big-endian M68K 68040 common subset. ColdFire classification comes from package metadata and does not identify fitted silicon independently. Separate positions do not establish additional unique functions or physical flash banks.

`sp` names the stack pointer at the analyzed entry. Stored bytes and the full `d0` return expressions are reported separately. The [complete facts](disasm/coldfire-variants/byte-stores-6.5.9-6.5.30-facts.txt) retain frame effects omitted from these summaries. Range ends are exclusive, and caller byte matches do not prove predecessor flow.

- `0xffe536a4`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe536a4 to 0xffe536b8 (exclusive), 20 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `6bb4290fc536a896cdd5527a72eed62c93ddfc57dcefe89e83dbf7e5f21882bf`.
  Four-byte load: target `abs 0x30000480` (`movea.l` at 0xffe536a8).
  Byte store: target `[0x30000480] +0x1e8`, value `0x80` (`move.b` at 0xffe536b0).
  At the return (`rts` at 0xffe536b6), d0 is `0xffffff80`.

- `0xffec36a4`, `root/6.5.9/usr/cpu/firmware/l1.bin`.
  Package `6.5.9:l1#0`, version `1.0.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffec36a4 to 0xffec36b8 (exclusive), 20 bytes.
  Carrier: 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Package: carrier offset 0x0, 927304 bytes, SHA256 `346ebde8121eda2b390df35b37bca75a28c5bbfc44dd5f329b91d56455b575e8`.
  Payload: carrier offset 0x20, 927272 bytes, SHA256 `8a177e55cfdd58f8d233ea3c6e5f8b1a5a9bcaf10a20e95cbde444bd34da6545`.
  Read window: SHA256 `6bb4290fc536a896cdd5527a72eed62c93ddfc57dcefe89e83dbf7e5f21882bf`.
  Four-byte load: target `abs 0x30000480` (`movea.l` at 0xffec36a8).
  Byte store: target `[0x30000480] +0x1e8`, value `0x80` (`move.b` at 0xffec36b0).
  The exit (`rts` at 0xffec36b6) reports d0 = `0xffffff80`.

- `0xffe72430`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#0`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72430 to 0xffe72446 (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x0, 982480 bytes, SHA256 `c391b1c9925c25fa5040d54e9adc51c8cb2ed9b5626a8734938bbc19267e4f08`.
  Payload: carrier offset 0x68, 982376 bytes, SHA256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`.
  Read window: SHA256 `d8cac666ab097c98176536d75bfba92436ccf66293a8c73be2d7f61379c40418`.
  Four-byte load: target `abs 0x300002bc` (`movea.l` at 0xffe72434).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7243a).
  Byte store: target `[0x300002bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe7243e).
  `rts` at 0xffe72444 has recorded d0 = `[sp+4]`.

- `0xffe9548c`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#2`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9548c to 0xffe954a2 (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`.
  Read window: SHA256 `89904c2743603d571eeeed97a16ee904f6b71294f88fb23b18913021c2eb7660`.
  Four-byte load: target `abs 0x300008bc` (`movea.l` at 0xffe95490).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe95496).
  Byte store: target `[0x300008bc] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9549a).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe954a0).

- `0xffe9c294`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#3`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9c294 to 0xffe9c2aa (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x38c508, 1772620 bytes, SHA256 `40d905ba620ebba0c8259b12410dfe42d21d18cad06477b2873f201059af2fe4`.
  Payload: carrier offset 0x38c534, 1772576 bytes, SHA256 `d13e4959222bad3308e71ba9c854efae01770afb942a9e562212eaeb19fff6fa`.
  Read window: SHA256 `57a07135f45c035a9584ce1719a6f889c72b559cd8ca39bbe6ce53a2ffa6a430`.
  Four-byte load: target `abs 0x30000a50` (`movea.l` at 0xffe9c298).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9c29e).
  Byte store: target `[0x30000a50] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9c2a2).
  At the return (`rts` at 0xffe9c2a8), d0 is `[sp+4]`.

- `0xffe9adbc`, `root/6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#5`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9adbc to 0xffe9add2 (exclusive), 22 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x8e1250, 1778776 bytes, SHA256 `562aa31da2e8d4dfbb94274fb383b078c78c25d84b5c84c747b11ed9d481a68f`.
  Payload: carrier offset 0x8e127c, 1778732 bytes, SHA256 `5d4521de0baec59a9592cee0aa871cc67cd309c85f4dd303a290e937f49dbf8c`.
  Read window: SHA256 `f296062bc1c203972a63483de8ebf268b3abcb856e2f8dd17023fde9e10c5e46`.
  Four-byte load: target `abs 0x30000bd4` (`movea.l` at 0xffe9adc0).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9adc6).
  Byte store: target `[0x30000bd4] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe9adca).
  The exit (`rts` at 0xffe9add0) reports d0 = `[sp+4]`.

- `0xffe711dc`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#1`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe711dc to 0xffe711f2 (exclusive), 22 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0xef4a4, 979096 bytes, SHA256 `1f3ca14b06175e39d38af6315bef38a6ab31ff2178a5cfd05943593bf37f64a7`.
  Payload: carrier offset 0xef4d0, 979052 bytes, SHA256 `72ad6ba3005bd3a464774c90dcfe60e333e18e06d10812724c72c2ddd62a0351`.
  Read window: SHA256 `2e82c108cb971a797514f8120b9ea90d0561481bf2e251fb36ab4b71c67fa0d2`.
  Four-byte load: target `abs 0x300008b0` (`movea.l` at 0xffe711e0).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe711e6).
  Byte store: target `[0x300008b0] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe711ea).
  `rts` at 0xffe711f0 has recorded d0 = `[sp+4]`.

- `0xffea1f60`, `root/6.5.24/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.24:l1#2`, version `1.28.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffea1f60 to 0xffea1f76 (exclusive), 22 bytes.
  Carrier: 3759416 bytes, SHA256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`.
  Package: carrier offset 0x1de53c, 1800188 bytes, SHA256 `391a349a2940f18047f404a69cbf7641e25c67c2d7739e3739ec54e473b9d6a7`.
  Payload: carrier offset 0x1de568, 1800144 bytes, SHA256 `1f088a070cf4398029a936d96860ad27d7e2bf2ed2ad35b871f07d84398be8c5`.
  Read window: SHA256 `abfec6b667fa6a480a13de2856280b61a5555dd31cc946ad73f8bce69281a4d4`.
  Four-byte load: target `abs 0x30000b18` (`movea.l` at 0xffea1f64).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffea1f6a).
  Byte store: target `[0x30000b18] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffea1f6e).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffea1f74).

- `0xffe7183c`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#0`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7183c to 0xffe71852 (exclusive), 22 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0x0, 979900 bytes, SHA256 `bc2358995619da07a0363c20e1ed39794146b9ad478b1b3e5068aca530c3dfc9`.
  Payload: carrier offset 0x48, 979828 bytes, SHA256 `a13d2aa10e7903bf53ab9b8395c23d4ebcf47eb3c09bb126cabfdf3f423e8b98`.
  Read window: SHA256 `161bcacdf707238e84b0329fa8fb83921311a7f8838b276f01151bbf0498e49b`.
  Four-byte load: target `abs 0x30000320` (`movea.l` at 0xffe71840).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71846).
  Byte store: target `[0x30000320] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe7184a).
  At the return (`rts` at 0xffe71850), d0 is `[sp+4]`.

- `0xffe71854`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#1`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71854 to 0xffe7186a (exclusive), 22 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0xef3bc, 979984 bytes, SHA256 `0c31235ae9a306523cbb55de14016bc052fa2f4ae889defc8b6de3099e19c112`.
  Payload: carrier offset 0xef3e8, 979940 bytes, SHA256 `d592fe64ddd586ff8ffef2cea25d5e55e38c41114174b22d106a211524055ef0`.
  Read window: SHA256 `90dd858b68e2993c5e0753d989db7b63b241ef16d021cbea5dc65482c284150b`.
  Four-byte load: target `abs 0x30000354` (`movea.l` at 0xffe71858).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7185e).
  Byte store: target `[0x30000354] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe71862).
  The exit (`rts` at 0xffe71868) reports d0 = `[sp+4]`.

- `0xffeb22cc`, `root/6.5.25/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.25:l1#2`, version `1.30.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeb22cc to 0xffeb22e2 (exclusive), 22 bytes.
  Carrier: 3817584 bytes, SHA256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`.
  Package: carrier offset 0x1de7cc, 1857700 bytes, SHA256 `718b73d44aa415958aa1899cc03977a0fc5c1f659184012a4c2c33771cd500e3`.
  Payload: carrier offset 0x1de7f8, 1857656 bytes, SHA256 `01ddeae2fbe3704ee85ad90d827a6325a0ada649e91d7dfc593a1a14f80e07b5`.
  Read window: SHA256 `f602a47c77d422039b992e43f0c29c8d79dd5f5e54ff61a1b3d12358ab31e3d5`.
  Four-byte load: target `abs 0x300004b0` (`movea.l` at 0xffeb22d0).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffeb22d6).
  Byte store: target `[0x300004b0] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffeb22da).
  `rts` at 0xffeb22e0 has recorded d0 = `[sp+4]`.

- `0xffe70fdc`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#0`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe70fdc to 0xffe70ff2 (exclusive), 22 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0x0, 978796 bytes, SHA256 `69539d85355ecdbbf84627ba00e20cfbc7662fbbebb071c8ac525a43150aaa0d`.
  Payload: carrier offset 0x48, 978724 bytes, SHA256 `92a0694b91b363f898d9d22a3072fc2f9932830b683e2b071f3df45f7329f07e`.
  Read window: SHA256 `f3390943ff1b1cff4a0f94640cd687134898cea4fc12ca28b1f686559215f976`.
  Four-byte load: target `abs 0x300003f8` (`movea.l` at 0xffe70fe0).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe70fe6).
  Byte store: target `[0x300003f8] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe70fea).
  Recorded d0 at the return is `[sp+4]` (`rts` at 0xffe70ff0).

- `0xffe71b7c`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#1`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71b7c to 0xffe71b92 (exclusive), 22 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0xeef6c, 982112 bytes, SHA256 `81744f9b80dc588fc8c607763d38aa7fe926adf437202b23befbd9932ec14368`.
  Payload: carrier offset 0xeef98, 982068 bytes, SHA256 `fe1b451b03bf7c29e2e09011317c999cba37b2ea2c8bd89f316497d756cc56a9`.
  Read window: SHA256 `086b8bb033875e8be0c9b689f94f90ae156eae660329900b2af4e869818f8374`.
  Four-byte load: target `abs 0x30000434` (`movea.l` at 0xffe71b80).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71b86).
  Byte store: target `[0x30000434] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe71b8a).
  At the return (`rts` at 0xffe71b90), d0 is `[sp+4]`.

- `0xffeb6288`, `root/6.5.26/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.26:l1#2`, version `1.32.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffeb6288 to 0xffeb629e (exclusive), 22 bytes.
  Carrier: 3846136 bytes, SHA256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`.
  Package: carrier offset 0x1debcc, 1885228 bytes, SHA256 `84466adaef25b1382edb5db0569655ab340dea79b2adde07cf9499f400ca4837`.
  Payload: carrier offset 0x1debf8, 1885184 bytes, SHA256 `9793fe0aac389db6d98cbd2fe0c6d96aa70dcc526d0cfe5810d554316931d981`.
  Read window: SHA256 `b34974bf8cce4e6d03415e4b6f2b44467719e5c307f3620596a4af9af698ced7`.
  Four-byte load: target `abs 0x30000a7c` (`movea.l` at 0xffeb628c).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffeb6292).
  Byte store: target `[0x30000a7c] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffeb6296).
  The exit (`rts` at 0xffeb629c) reports d0 = `[sp+4]`.

- `0xffe72184`, `root/6.5.27/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.27:l1#0`, version `1.34.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72184 to 0xffe7219a (exclusive), 22 bytes.
  Carrier: 7523600 bytes, SHA256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
  Package: carrier offset 0x0, 981832 bytes, SHA256 `d3968aabd51125c60c759766ceed4c9fc66321e66e4e562d235bc47b16efb14d`.
  Payload: carrier offset 0x50, 981752 bytes, SHA256 `117f4126e840ee43eac0d96eb2aede9ff454f78b7c32a3aa42b68fcbe034e91b`.
  Read window: SHA256 `84d763abe39b57fa861aec62c3a758dfbdcc112eba6697f83cf8174dfd8f8ef0`.
  Four-byte load: target `abs 0x30000410` (`movea.l` at 0xffe72188).
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7218e).
  Byte store: target `[0x30000410] +0x1c9`, value `([sp+4] & 0xff)` (`move.b` at 0xffe72192).
  `rts` at 0xffe72198 has recorded d0 = `[sp+4]`.
