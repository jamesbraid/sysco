# ARM library calls and memory accesses

These entries identify shared-library instances in
`6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4`, version `1.44.0`. ELF
symbol values and sizes supply the selected analysis spans. Whole-span decoding and
bounded contexts support those inferred spans, without independently proving complete
function extents. Hardware roles and pointee meanings remain unknown.

Instructions and direct targets use linked ELF virtual coordinates. The lowest linked
load coordinate is `0x0`. Runtime ELF load bias remains unknown. Data references marked
`ELF load bias (unknown)` retain that uncertainty. A linked address is not a physical or
runtime address. Values read through relocation-dependent literals remain unknown.

Calls report four candidate AAPCS core-register slots. Actual arity and custom
conventions remain unknown. Target coordinate basis, execution state and canonical
instruction target are stated separately for every call. Unknown indirect-call metadata
stays unknown. Bare register names in value expressions denote entry values, and `sp`
denotes the entry stack pointer. A lone `?` is written as `unknown`. Compound
expressions and predicates retain their printed text, including opaque `ret(unknown)`
expressions.

Groups count static access records with the same width, base, displacement, resolved
target and reported store value. They do not imply one path through different branch
arms or equal values across executions. Raw base registers identify accesses whose
effective address is unknown. Their raw displacements are not promoted to resolved
offsets.

The [complete facts](disasm/arm-1.44.0-libraries/calls-memory-paths-facts.txt) retain every access, relocation, constant and warning, including omitted frame saves, restores and input spills.

- `L1DirectSendPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xaa70 to 0xaaec (exclusive), 124 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `b0ac32041084c736f2a92fd57e8481af41889961d4324d65956d7025c017c3bb`.
  Four-byte load: target `ELF load bias (unknown) +0xaadc`, linked address 0xaadc, basis `ELF load-bias-relative` (`ldr` at 0xaa88).
  ELF literal warning: linked 0xaadc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaa88.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xaa8c, `ldr` at 0xaab4.
  Branch (`beq` at 0xaa94): taken to 0xaac0 when `? == 0x0`, falls through to 0xaa98 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xaae0`, linked address 0xaae0, basis `ELF load-bias-relative` (`ldr` at 0xaa9c).
  ELF literal warning: linked 0xaae0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaa9c.
  Call (`bl` at 0xaaa8): (unknown callee), target 0x3694, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3694, r0=`unknown`, r1=`r0`, r2=`sp-104`, r3=`sp-104`.
  Four-byte load: target `ELF load bias (unknown) +0xaae4`, linked address 0xaae4, basis `ELF load-bias-relative` (`ldr` at 0xaaac).
  ELF literal warning: linked 0xaae4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaaac.
  Four-byte load: target `ELF load bias (unknown) +0xaae8`, linked address 0xaae8, basis `ELF load-bias-relative` (`ldr` at 0xaab8).
  ELF literal warning: linked 0xaae8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaab8.
  Call (`bl` at 0xaabc): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`sp-104`, r3=`unknown`.
  Call (`bl` at 0xaac8): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  At the return (`ldm` at 0xaad8), r0 is `0x0`.
  The instructions from 0xaadc to 0xaae8 are not reached by any path from the entry, and their branches are not listed.

- `L2CfgMEventSystemName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x6150 to 0x6188 (exclusive), 56 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `1a4b13922fa097f127ed52f8fa6260e2531930f7d9dd26a742f7eb020973bdf4`.
  Call (`bl` at 0x6168): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`0x4`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x6180): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, r0=`[sp-20]+304`, r1=`ret(linked ELF 0x37cc)`, r2=`[sp-20]+304`, r3=`ret(linked ELF 0x37cc)`.
  The exit (`ldm` at 0x6184) reports r0 = `ret(linked ELF 0x37d8)`.

- `L2CfgMEventSystemSN`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x6118 to 0x6150 (exclusive), 56 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `397a0771012bf547113b6c6c70bd2896cb20111b61c61454f607d3dc90d20f57`.
  Call (`bl` at 0x6130): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`0x3`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x6148): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, r0=`[sp-20]+284`, r1=`ret(linked ELF 0x37cc)`, r2=`[sp-20]+284`, r3=`ret(linked ELF 0x37cc)`.
  Recorded r0 at the return is `ret(linked ELF 0x37d8)` (`ldm` at 0x614c).

- `L2CfgMEventDisplayDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x6188 to 0x61dc (exclusive), 84 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `ede032f495e7081ad9e2cc4b5e0fb83acc9367e9332a710c1a3e2ff2049ab703`.
  Call (`bl` at 0x61a4): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`0x9`, r1=`r1`, r2=`r2`, r3=`r3`.
  Byte load: target `r0 +0x144` (`ldrb` at 0x61b0).
  Byte store: target `r0 +0x144`, value `([sp-28]:1 & 0xff)` (`strb` at 0x61d0).
  `ldm` at 0x61d8 has recorded r0 = `([r0+324]:1 & 0xff)`.

- `sn1ScanDisconnect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x3c10 to 0x3c58 (exclusive), 72 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `94300d72e2cad8b2e9769d44688c5ff8bf003609e0325cf7d991a3af707500f9`.
  Call (`bl` at 0x3c1c): (unknown callee), target 0x37b4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37b4, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Four-byte load: target `ELF load bias (unknown) +0x3c50`, linked address 0x3c50, basis `ELF load-bias-relative` (`ldr` at 0x3c24).
  ELF literal warning: linked 0x3c50, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3c24.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x3c28, `ldr` at 0x3c40.
  Call (`mov` at 0x3c34): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`ret(linked ELF 0x37b4)`, r1=`unknown`, r2=`ret(linked ELF 0x37b4)`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x3c54`, linked address 0x3c54, basis `ELF load-bias-relative` (`ldr` at 0x3c38).
  ELF literal warning: linked 0x3c54, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x3c38, `ldr` at 0x3c3c.
  Four-byte load: target `ELF load bias (unknown) +0x3c54`, linked address 0x3c54, basis `ELF load-bias-relative` (`ldr` at 0x3c3c).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2` (`str` at 0x3c48).
  At the return (`ldm` at 0x3c4c), r0 is `ret(unknown)`.
  The instructions from 0x3c50 to 0x3c54 are not reached by any path from the entry, and their branches are not listed.

- `jmstr_reset_and_idle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x3968 to 0x3a90 (exclusive), 296 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `8deb9c825653f8f05ad84a4d345e8f79127807b03be47d33240a74768bfc0594`.
  Four-byte store: target `sp -0x28`, value `0x0` (`str` at 0x3988).
  Four-byte store: target `sp -0x24`, value `0x20` (`str` at 0x3990).
  Branch (`beq` at 0x399c): taken to 0x39ac when `r1 == 0x0`, falls through to 0x39a0 when `r1 != 0x0`.
  Grouped accesses: 5 4-byte load records with target `sp -0x24`, including `ldr` at 0x39a0, `ldr` at 0x39b8, `ldr` at 0x39d4.
  Four-byte store: target `sp -0x24`, value `0x28` (`str` at 0x39a8).
  Branch (`bne` at 0x39b4): taken to 0x39c4 when `r0 != 0x1`, falls through to 0x39b8 when `r0 == 0x1`.
  Four-byte store: target `sp -0x24`, value `unknown` (`str` at 0x39c0).
  Four-byte load: target `ELF load bias (unknown) +0x3a88`, linked address 0x3a88, basis `ELF load-bias-relative` (`ldr` at 0x39c4).
  ELF literal warning: linked 0x3a88, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x39c4, `ldr` at 0x3a2c.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x39d0, `ldr` at 0x3a38.
  Call (`mov` at 0x39e4): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`0x20`, r2=`sp-40`, r3=`sp-40`.
  Branch (`bne` at 0x39f0): taken to 0x3a0c when `ret(unknown) != 0x0`, falls through to 0x39f4 when `ret(unknown) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x3a8c`, linked address 0x3a8c, basis `ELF load-bias-relative` (`ldr` at 0x39f8).
  ELF literal warning: linked 0x3a8c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x39f8, `ldr` at 0x3a60.
  Call (`bl` at 0x39fc): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `0x0`, including `str` at 0x3a04, `str` at 0x3a6c.
  Jump (`b` at 0x3a08): target 0x3a7c.
  Four-byte store: target `sp -0x24`, value `0x10` (`str` at 0x3a10).
  Branch (`beq` at 0x3a1c): taken to 0x3a2c when `[sp-28] == 0x0`, falls through to 0x3a20 when `[sp-28] != 0x0`.
  Four-byte store: target `sp -0x24`, value `0x18` (`str` at 0x3a28).
  Four-byte load: target `ELF load bias (unknown) +0x3a88`, linked address 0x3a88, basis `ELF load-bias-relative` (`ldr` at 0x3a2c).
  Call (`mov` at 0x3a4c): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`0x20`, r2=`sp-40`, r3=`sp-40`.
  Branch (`bne` at 0x3a58): taken to 0x3a74 when `ret(unknown) != 0x0`, falls through to 0x3a5c when `ret(unknown) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x3a8c`, linked address 0x3a8c, basis `ELF load-bias-relative` (`ldr` at 0x3a60).
  Call (`bl` at 0x3a64): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Jump (`b` at 0x3a70): target 0x3a7c.
  Four-byte store: target `sp -0x2c`, value `0x1` (`str` at 0x3a78).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x3a7c).
  The exit (`ldm` at 0x3a84) reports r0 = `unknown`.
  The instructions from 0x3a88 to 0x3a8c are not reached by any path from the entry, and their branches are not listed.

- `scancmd_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x2c70 to 0x2d10 (exclusive), 160 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `04b50a6f72d42026089a4c545470fb79ad054a86c1d0d19df1127c11dc096615`.
  Branch (`bne` at 0x2c90): taken to 0x2ca0 when `r1 != 0x0`, falls through to 0x2c94 when `r1 == 0x0`.
  Four-byte store: target `sp -0x20`, value `0x0` (`str` at 0x2c98).
  Jump (`b` at 0x2c9c): target 0x2d04.
  Branch (`bne` at 0x2cac): taken to 0x2cd0 when `? != 0x0`, falls through to 0x2cb0 when `? == 0x0`.
  Grouped accesses: 4 4-byte store records with target `sp -0x24`, reported value `unknown`, including `str` at 0x2cb8, `str` at 0x2cc8, `str` at 0x2cd8.
  Grouped accesses: 3 4-byte load records with target `sp -0x24`, including `ldr` at 0x2cbc, `ldr` at 0x2cdc, `ldr` at 0x2cf4.
  Jump (`b` at 0x2ccc): target 0x2cec.
  Call (`bl` at 0x2cf8): (unknown callee), target 0x1d98, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d98, r0=`r1`, r1=`0x0`, r2=`[sp-36]`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `0x1` (`str` at 0x2d00).
  Four-byte load: target `sp -0x20` (`ldr` at 0x2d04).
  Recorded r0 at the return is `unknown` (`ldm` at 0x2d0c).

- `scancmd_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x2bac to 0x2c70 (exclusive), 196 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `ac33762b202b011fa86b2a98d5436fec20eb5940d3764d3f781800a5ede7c3b4`.
  Branch (`bne` at 0x2bcc): taken to 0x2bf0 when `? != 0x0`, falls through to 0x2bd0 when `? == 0x0`.
  Grouped accesses: 4 4-byte store records with target `sp -0x20`, reported value `unknown`, including `str` at 0x2bd8, `str` at 0x2be8, `str` at 0x2bf8.
  Grouped accesses: 3 4-byte load records with target `sp -0x20`, including `ldr` at 0x2bdc, `ldr` at 0x2bfc, `ldr` at 0x2c0c.
  Jump (`b` at 0x2bec): target 0x2c0c.
  Call (`bl` at 0x2c10): (unknown callee), target 0x1c84, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c84, r0=`[sp-32]`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x18`, value `ret(linked ELF 0x1c84)` (`str` at 0x2c14).
  Grouped accesses: 4 4-byte load records with target `sp -0x18`, including `ldr` at 0x2c18, `ldr` at 0x2c34, `ldr` at 0x2c48.
  Branch (`bne` at 0x2c20): taken to 0x2c30 when `ret(linked ELF 0x1c84) != 0x0`, falls through to 0x2c24 when `ret(linked ELF 0x1c84) == 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x1c`, reported value `0x0`, including `str` at 0x2c28, `str` at 0x2c54.
  Jump (`b` at 0x2c2c): target 0x2c64.
  Call (`bl` at 0x2c38): (unknown callee), target 0x1d80, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d80, r0=`[sp-20]`, r1=`ret(linked ELF 0x1c84)`, r2=`unknown`, r3=`ret(linked ELF 0x1c84)`.
  Branch (`bne` at 0x2c44): taken to 0x2c5c when `ret(linked ELF 0x1d80) != 0x0`, falls through to 0x2c48 when `ret(linked ELF 0x1d80) == 0x0`.
  Call (`bl` at 0x2c4c): (unknown callee), target 0x1d68, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d68, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x1d80)`.
  Jump (`b` at 0x2c58): target 0x2c64.
  Four-byte store: target `sp -0x1c`, value `[sp-24]` (`str` at 0x2c60).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x2c64).
  `ldm` at 0x2c6c has recorded r0 = `unknown`.

- `IRouterDisconnectOS`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x5afc to 0x5b24 (exclusive), 40 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `8256e274377e7c1b3ec18580c0bc10831a2332cd1bfb83e632c9b8c21110dd90`.
  Four-byte load: target `ELF load bias (unknown) +0x5b20`, linked address 0x5b20, basis `ELF load-bias-relative` (`ldr` at 0x5b10).
  ELF literal warning: linked 0x5b20, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5b10.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x5b14).
  Call (`bl` at 0x5b18): (unknown callee), target 0x1ccc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1ccc, r0=`unknown`, r1=`r1`, r2=`r2`, r3=`unknown`.
  At the return (`ldm` at 0x5b1c), r0 is `ret(linked ELF 0x1ccc)`.
  The instructions from 0x5b20 to 0x5b20 are not reached by any path from the entry, and their branches are not listed.

- `IRouterInitID`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x2afc to 0x2b78 (exclusive), 124 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `9a663672d24b8bdb9ca2ebc712a91671b3b29881747cc8afc025f1f3889a1a69`.
  Four-byte load: target `ELF load bias (unknown) +0x2b74`, linked address 0x2b74, basis `ELF load-bias-relative` (`ldr` at 0x2b1c).
  ELF literal warning: linked 0x2b74, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2b1c.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x2b20).
  Four-byte store: target `r0 +0x0`, value `unknown` (`str` at 0x2b24).
  Four-byte load: target `[sp-20] +0x0` (`ldr` at 0x2b30).
  Four-byte store: target `[sp-20] +0x0`, value `unknown` (`str` at 0x2b48).
  Byte store: target `[sp-20] +0x4`, value `([sp-28] & 0xff)` (`strb` at 0x2b54).
  Two-byte store: target `[sp-20] +0x6`, value `0x0` (`strh` at 0x2b60).
  The exit (`ldm` at 0x2b70) reports r0 = `0x0`.
  The instructions from 0x2b74 to 0x2b74 are not reached by any path from the entry, and their branches are not listed.

- `IRouterPacketCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x200c to 0x2088 (exclusive), 124 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `aba01fd19ffb649eee242614f3168a8be9adb9bdbf4594a0e2431ad3dc59a5b0`.
  Four-byte load: target `ELF load bias (unknown) +0x207c`, linked address 0x207c, basis `ELF load-bias-relative` (`ldr` at 0x2020).
  ELF literal warning: linked 0x207c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x2020, `ldr` at 0x2024.
  Four-byte load: target `ELF load bias (unknown) +0x207c`, linked address 0x207c, basis `ELF load-bias-relative` (`ldr` at 0x2024).
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x2028, `ldr` at 0x2040, `ldr` at 0x2058.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown`, base register `r1`, including `str` at 0x2034, `str` at 0x204c.
  Four-byte load: target `ELF load bias (unknown) +0x2080`, linked address 0x2080, basis `ELF load-bias-relative` (`ldr` at 0x2038).
  ELF literal warning: linked 0x2080, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x2038, `ldr` at 0x203c, `ldr` at 0x2050.
  Four-byte load: target `ELF load bias (unknown) +0x2080`, linked address 0x2080, basis `ELF load-bias-relative` (`ldr` at 0x203c).
  Four-byte load: target `ELF load bias (unknown) +0x2080`, linked address 0x2080, basis `ELF load-bias-relative` (`ldr` at 0x2050).
  Four-byte load: target `ELF load bias (unknown) +0x2084`, linked address 0x2084, basis `ELF load-bias-relative` (`ldr` at 0x2054).
  ELF literal warning: linked 0x2084, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x2054, `ldr` at 0x2068.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, including `ldr` at 0x205c, `ldr` at 0x2070.
  Branch (`ble` at 0x2064): taken to 0x2078 when `? <= ? (signed, 32-bit)`, falls through to 0x2068 when `? > ? (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x2084`, linked address 0x2084, basis `ELF load-bias-relative` (`ldr` at 0x2068).
  Four-byte load: target `ELF load bias (unknown) +0x2080`, linked address 0x2080, basis `ELF load-bias-relative` (`ldr` at 0x206c).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r3` (`str` at 0x2074).
  Recorded r0 at the return is `r0` (`ldm` at 0x2078).
  The instructions from 0x207c to 0x2084 are not reached by any path from the entry, and their branches are not listed.

- `sn1SCConnSocket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x9370 to 0x93bc (exclusive), 76 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `d90f0877c98659cb8f5f2f96ecee57fb26a49e8196af60bc17c2efe4909b4381`.
  Grouped accesses: 2 4-byte load records with target `r0 +0x84`, including `ldr` at 0x9388, `ldr` at 0x9398.
  Branch (`beq` at 0x9390): taken to 0x93a8 when `[r0+132] == 0x0`, falls through to 0x9394 when `[r0+132] != 0x0`.
  Four-byte load: target `[r0+132] +0x0` (`ldr` at 0x939c).
  Four-byte store: target `sp -0x18`, value `[[r0+132]]` (`str` at 0x93a0).
  Jump (`b` at 0x93a4): target 0x93b0.
  Effects remain unknown for `mvn` at 0x93a8 (unsupported effects invalidated).
  Four-byte store: target `sp -0x18`, value `unknown` (`str` at 0x93ac).
  Four-byte load: target `sp -0x18` (`ldr` at 0x93b0).
  `ldm` at 0x93b8 has recorded r0 = `[sp-24]`.

- `sn1SCConnDisconnect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x9330 to 0x9370 (exclusive), 64 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `4e44a0f437a2cae214c7882ea53fa800a8c99e6f1202a61c00a5965a53abcb78`.
  Four-byte load: target `ELF load bias (unknown) +0x936c`, linked address 0x936c, basis `ELF load-bias-relative` (`ldr` at 0x9344).
  ELF literal warning: linked 0x936c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x9344.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x934c).
  Four-byte load: target `r0 +0x84` (`ldr` at 0x9350).
  Call (`mov` at 0x9358): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`[r0+132]`, r1=`r1`, r2=`r0`, r3=`unknown`.
  Four-byte store: target `[sp-20] +0x84`, value `0x0` (`str` at 0x9364).
  At the return (`ldm` at 0x9368), r0 is `ret(unknown)`.
  The instructions from 0x936c to 0x936c are not reached by any path from the entry, and their branches are not listed.
