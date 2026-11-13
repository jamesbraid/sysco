# ARM shared-library call and pointer paths

These entries concern three shared libraries extracted from package `6.5.30:l1#4`,
version `1.44.0`, in `6.5.30/usr/cpu/firmware/sysco/l1.bin`. Each names its inner file
and hashes. ELF symbols supply the selected names and sizes. Whole bodies and bounded
neighbouring decodes support the inferred analysis spans. They do not independently
prove function boundaries or runtime placement. Role: unknown.

Instruction addresses and direct call targets use linked ELF virtual coordinates. A load
coordinate of `0x0` does not establish runtime address zero: the runtime ELF load bias
remains unknown. References tagged `linked ELF` remain in that coordinate domain. All
reported call targets enter ARM state at the displayed instruction target. Names are
unknown where no callee name is resolved.

The four core-register slots are candidates under an assumed ARM AAPCS convention.
Actual arity is unknown. Bare register names in expressions denote their values at
entry. `sp` denotes the entry stack pointer. `unknown` preserves an unresolved value or
address. Static grouped records do not establish identical values across executions or
simultaneous execution of branch arms.

The [complete facts](disasm/arm-1.44.0-libraries/call-pointer-paths-facts.txt) retain frame accesses, constants and warnings omitted from these summaries.

- `IRouterSetAuthority`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x2b78 to 0x2bac (exclusive), 52 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `621905f5f5717718e0c16a77987af496a93406c365d710ff5fda5e05aa5d49dc`.
  Byte store: target `r0 +0x4`, value `(r1 & 0xff)` (`strb` at 0x2b98).
  At the return (`ldm` at 0x2ba8), r0 is `0x0`.

- `IRouterSetFrameByteOrder`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x3770 to 0x3a3c (exclusive), 716 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `23c38f847567e23e4ef2293a42022d0af6b2b3ec912971e88d23d2cf60fff0b6`.
  Byte load: target `r0 +0x6` (`ldrb` at 0x378c).
  Branch (`bne` at 0x37a4): taken to 0x37ac when `unknown != r1`, falls through to 0x37a8 when `unknown == r1`.
  Jump (`b` at 0x37a8): target 0x3a34.
  Branch (`bne` at 0x37b4): taken to 0x38f8 when `r1 != 0x2`, falls through to 0x37b8 when `r1 == 0x2`.
  Grouped accesses: 2 2-byte load records with target `r0 +0x10`, including `ldrh` at 0x37c0, `ldrh` at 0x3900.
  Call (`bl` at 0x37c8): (unknown callee), linked ELF target 0x1da4, ARM instruction target 0x1da4, r0=`[r0+16]:2`, r1=`r1`, r2=`unknown`, r3=`[r0+16]:2`.
  Two-byte store: target `r0 +0x10`, value `(ret(linked ELF 0x1da4) & 0xffff)` (`strh` at 0x37d0).
  Grouped accesses: 2 4-byte load records with target `[sp-24] +0x18`, including `ldr` at 0x37dc, `ldr` at 0x391c.
  Call (`bl` at 0x37e0): (unknown callee), linked ELF target 0x1c48, ARM instruction target 0x1c48, r0=`[[sp-24]+24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0x18`, value `ret(linked ELF 0x1c48)` (`str` at 0x37e8).
  Grouped accesses: 2 4-byte load records with target `[sp-24] +0x1c`, including `ldr` at 0x37f4, `ldr` at 0x3934.
  Call (`bl` at 0x37f8): (unknown callee), linked ELF target 0x1c48, ARM instruction target 0x1c48, r0=`[[sp-24]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0x1c`, value `ret(linked ELF 0x1c48)` (`str` at 0x3800).
  Grouped accesses: 2 2-byte load records with target `[sp-24] +0x22`, including `ldrh` at 0x380c, `ldrh` at 0x394c.
  Call (`bl` at 0x3814): (unknown callee), linked ELF target 0x1da4, ARM instruction target 0x1da4, r0=`[[sp-24]+34]:2`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+34]:2`.
  Two-byte store: target `[sp-24] +0x22`, value `(ret(linked ELF 0x1da4) & 0xffff)` (`strh` at 0x381c).
  Grouped accesses: 2 1-byte load records with target `[sp-24] +0x20`, including `ldrb` at 0x382c, `ldrb` at 0x396c.
  Branch (`ble` at 0x3838): taken to 0x38d0 when `le flags condition (operands unknown)`, falls through to 0x383c when `not (le flags condition (operands unknown))`.
  Grouped accesses: 2 2-byte load records with target `(unknown effective address)`, base register `r3`, including `ldrh` at 0x386c, `ldrh` at 0x39ac.
  Call (`bl` at 0x3874): (unknown callee), linked ELF target 0x1da4, ARM instruction target 0x1da4, r0=`unknown`, r1=`0x24`, r2=`[sp-24]`, r3=`unknown`.
  Two-byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x1da4) & 0xffff)`, base register `r4` (`strh` at 0x387c).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x38b0, `ldr` at 0x39f0.
  Call (`bl` at 0x38b4): (unknown callee), linked ELF target 0x1c48, ARM instruction target 0x1c48, r0=`unknown`, r1=`0x28`, r2=`[sp-24]`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x1c48)`, base register `r4` (`str` at 0x38bc).
  Jump (`b` at 0x38cc): target 0x3828.
  Grouped accesses: 2 1-byte load records with target `[sp-24] +0x6`, including `ldrb` at 0x38d4, `ldrb` at 0x3a14.
  Byte store: target `[sp-24] +0x6`, value `([sp-36]:1 & 0xff)` (`strb` at 0x38f0).
  Jump (`b` at 0x38f4): target 0x3a34.
  Call (`bl` at 0x3908): (unknown callee), linked ELF target 0x1e58, ARM instruction target 0x1e58, r0=`[r0+16]:2`, r1=`r1`, r2=`unknown`, r3=`[r0+16]:2`.
  Two-byte store: target `r0 +0x10`, value `(ret(linked ELF 0x1e58) & 0xffff)` (`strh` at 0x3910).
  Call (`bl` at 0x3920): (unknown callee), linked ELF target 0x1d20, ARM instruction target 0x1d20, r0=`[[sp-24]+24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0x18`, value `ret(linked ELF 0x1d20)` (`str` at 0x3928).
  Call (`bl` at 0x3938): (unknown callee), linked ELF target 0x1d20, ARM instruction target 0x1d20, r0=`[[sp-24]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0x1c`, value `ret(linked ELF 0x1d20)` (`str` at 0x3940).
  Call (`bl` at 0x3954): (unknown callee), linked ELF target 0x1e58, ARM instruction target 0x1e58, r0=`[[sp-24]+34]:2`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+34]:2`.
  Two-byte store: target `[sp-24] +0x22`, value `(ret(linked ELF 0x1e58) & 0xffff)` (`strh` at 0x395c).
  Branch (`ble` at 0x3978): taken to 0x3a10 when `le flags condition (operands unknown)`, falls through to 0x397c when `not (le flags condition (operands unknown))`.
  Call (`bl` at 0x39b4): (unknown callee), linked ELF target 0x1e58, ARM instruction target 0x1e58, r0=`unknown`, r1=`0x24`, r2=`[sp-24]`, r3=`unknown`.
  Two-byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x1e58) & 0xffff)`, base register `r4` (`strh` at 0x39bc).
  Call (`bl` at 0x39f4): (unknown callee), linked ELF target 0x1d20, ARM instruction target 0x1d20, r0=`unknown`, r1=`0x28`, r2=`[sp-24]`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x1d20)`, base register `r4` (`str` at 0x39fc).
  Jump (`b` at 0x3a0c): target 0x3968.
  Byte store: target `[sp-24] +0x6`, value `([sp-40]:1 & 0xff)` (`strb` at 0x3a30).
  The exit (`ldm` at 0x3a38) reports r0 = `unknown`.

- `IRouterPacketToRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x468c to 0x46c8 (exclusive), 60 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `fd0577432d247e6ea4e2b824d73fa647e02862a38ff5de78ca00afb27263a039`.
  Call (`bl` at 0x46b4): (unknown callee), linked ELF target 0x1c3c, ARM instruction target 0x1c3c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Recorded r0 at the return is `ret(linked ELF 0x1c3c)` (`ldm` at 0x46c4).

- `IRouterDisconnectL3`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x4b30 to 0x4b50 (exclusive), 32 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `943f34611ebf2d0776eb9524993354a6a0913bb7411ed922053bca3b3a90116f`.
  Call (`bl` at 0x4b48): (unknown callee), linked ELF target 0x1cfc, ARM instruction target 0x1cfc, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  `ldm` at 0x4b4c has recorded r0 = `ret(linked ELF 0x1cfc)`.

- `sn1SCConnClient`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x9400 to 0x9458 (exclusive), 88 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `53548471594c4eadd36bb53d77efad790b5ed5b32d433c0237f381e055c47b6b`.
  Grouped accesses: 2 4-byte load records with target `r0 +0x84`, including `ldr` at 0x9418, `ldr` at 0x9428.
  Branch (`beq` at 0x9420): taken to 0x9448 when `[r0+132] == 0x0`, falls through to 0x9424 when `[r0+132] != 0x0`.
  Call (`bl` at 0x942c): (unknown callee), linked ELF target 0x382c, ARM instruction target 0x382c, r0=`[r0+132]`, r1=`r1`, r2=`r2`, r3=`r0`.
  Branch (`bne` at 0x9438): taken to 0x9448 when `ret(linked ELF 0x382c) != 0x0`, falls through to 0x943c when `ret(linked ELF 0x382c) == 0x0`.
  Four-byte store: target `[sp-20] +0x84`, value `0x0` (`str` at 0x9444).
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x944c).
  At the return (`ldm` at 0x9454), r0 is `unknown`.

- `L1DirectConnect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xa754 to 0xa774 (exclusive), 32 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `c47cd71845ed27e58e904d863c028ad599e20082afe9079315fc6aad96d5d981`.
  Call (`bl` at 0xa764): (unknown callee), linked ELF target 0x3970, ARM instruction target 0x3970, r0=`0x1`, r1=`r1`, r2=`r2`, r3=`r3`.
  The exit (`ldm` at 0xa770) reports r0 = `ret(linked ELF 0x3970)`.

- `L1DirectOpen`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xa774 to 0xa794 (exclusive), 32 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `2fb1ae13785b6b99bdbb0ebce2ea7d8747d1b175ae01cf0b430353e4a2457958`.
  Call (`bl` at 0xa784): (unknown callee), linked ELF target 0x3970, ARM instruction target 0x3970, r0=`0x0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Recorded r0 at the return is `ret(linked ELF 0x3970)` (`ldm` at 0xa790).

- `L1DirectClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xa794 to 0xa7b0 (exclusive), 28 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `ef71c281efcc69c11c9bbcd371da098f9a4fce0e0a6e65f6aa42fea13872bc3f`.
  Call (`bl` at 0xa7a0): (unknown callee), linked ELF target 0x35c8, ARM instruction target 0x35c8, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  `ldm` at 0xa7ac has recorded r0 = `ret(linked ELF 0x35c8)`.

- `l1d_tty_close`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xb744 to 0xb78c (exclusive), 72 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `c1932861ec4cdc6dd665a8029f31ef87e8ba92aaa2f43f2431aba5c9ec914fce`.
  Call (`bl` at 0xb75c): (unknown callee), linked ELF target 0x34b4, ARM instruction target 0x34b4, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0xb768): taken to 0xb778 when `ret(linked ELF 0x34b4) == 0x0`, falls through to 0xb76c when `ret(linked ELF 0x34b4) != 0x0`.
  Effects remain unknown for `mvn` at 0xb76c. unsupported effects invalidated.
  Four-byte store: target `sp -0x54`, value `unknown` (`str` at 0xb770).
  Jump (`b` at 0xb774): target 0xb780.
  Four-byte store: target `sp -0x54`, value `0x0` (`str` at 0xb77c).
  Four-byte load: target `sp -0x54` (`ldr` at 0xb780).
  At the return (`ldm` at 0xb788), r0 is `[sp-84]`.

- `PBayFlashFileClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xef94 to 0xefbc (exclusive), 40 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `5017656a914ae24c432a9beb67af6c58198c0d1f3b5d83e4309710b034cca8d7`.
  Call (`bl` at 0xefac): (unknown callee), linked ELF target 0x3430, ARM instruction target 0x3430, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  The exit (`ldm` at 0xefb8) reports r0 = `0x0`.

- `PBayFlashImageData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xf110 to 0xf19c (exclusive), 140 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `09f625993d056524202e26f33f1ce476123e449b28a16222ac3557aeee5950a7`.
  Branch (`beq` at 0xf138): taken to 0xf148 when `unknown == 0x0`, falls through to 0xf13c when `unknown != 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x24`, reported value `0x0`, including `str` at 0xf140, `str` at 0xf164.
  Jump (`b` at 0xf144): target 0xf190.
  Call (`bl` at 0xf14c): (unknown callee), linked ELF target 0x3514, ARM instruction target 0x3514, r0=`r2`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x3514)` (`str` at 0xf150).
  Branch (`bne` at 0xf15c): taken to 0xf16c when `ret(linked ELF 0x3514) != 0x0`, falls through to 0xf160 when `ret(linked ELF 0x3514) == 0x0`.
  Jump (`b` at 0xf168): target 0xf190.
  Call (`bl` at 0xf184): (unknown callee), linked ELF target 0x3694, ARM instruction target 0x3694, r0=`ret(linked ELF 0x3514)`, r1=`([sp-20] + [sp-24])`, r2=`[sp-28]`, r3=`([sp-20] + [sp-24])`.
  Four-byte store: target `sp -0x24`, value `[sp-32]` (`str` at 0xf18c).
  Four-byte load: target `sp -0x24` (`ldr` at 0xf190).
  Recorded r0 at the return is `unknown` (`ldm` at 0xf198).

- `nvramLogFinsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x119e8 to 0x11a30 (exclusive), 72 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `0ba89c6b84601271d26b268423eda579e19082a9937a666556d7cc25b490ad6d`.
  Four-byte store: target `sp -0x24`, value `sp-12` (`str` at 0x11a00).
  Four-byte load: target `sp -0x10` (`ldr` at 0x11a10).
  Four-byte load: target `sp -0x24` (`ldr` at 0x11a14).
  Call (`bl` at 0x11a18): (unknown callee), linked ELF target 0x36c4, ARM instruction target 0x36c4, r0=`sp-196`, r1=`0xa0`, r2=`r0`, r3=`sp-12`.
  Call (`bl` at 0x11a24): (unknown callee), linked ELF target 0x3574, ARM instruction target 0x3574, r0=`sp-196`, r1=`unknown`, r2=`unknown`, r3=`sp-196`.
  `ldm` at 0x11a2c has recorded r0 = `ret(linked ELF 0x3574)`.
