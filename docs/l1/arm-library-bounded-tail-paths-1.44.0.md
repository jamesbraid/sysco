# ARM library calls and undecoded tails

These entries identify shared-library instances in
`6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4`, version `1.44.0`. ELF
symbol values and sizes supply the selected analysis spans. The full original symbol
spans include explicitly retained trailing regions. Bounded contexts and local control
flow support the analysis without proving complete code-only function extents. Hardware
roles and pointee meanings remain unknown.

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

Undecodable bytes alone do not establish whether a local path reaches them. An empty
unresolved-transfer list also does not settle that question. Each entry records the
explicit unconditional return boundary and separately retains decoded unreached runs and
undecoded ranges. Calls remain opaque. Local nonreachability does not establish runtime
or global nonexecution.

The [complete facts](disasm/arm-1.44.0-libraries/bounded-tail-paths-facts.txt) retain every access, relocation, constant and warning, including omitted frame saves, restores and initial input spill stores. Meaningful entry-stack, reloaded input, local return-value and outgoing accesses are retained.

- `L1DirectRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xa7b0 to 0xa86c (exclusive), 188 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `d291e80eb7fd99aff1e62ac6ba0a1b9da34ac5feb594db4688ac63d8c14d9a55`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0xa7d4).
  Grouped accesses: 2 4-byte load records with target `sp +0x4`, including `ldr` at 0xa7d8, `ldr` at 0xa7e4.
  Branch (`beq` at 0xa7e0): taken to 0xa7f4 when `[sp+4] == 0x0`, falls through to 0xa7e4 when `[sp+4] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0xa7e8, `ldr` at 0xa80c.
  Two-byte load: target `r0 +0x6` (`ldrh` at 0xa7ec).
  Four-byte store: target `[sp+4] +0x0`, value `[r0+6]:2` (`str` at 0xa7f0).
  Four-byte store: target `sp -0x38`, value `0x2` (`str` at 0xa7f8).
  Four-byte load: target `sp -0x20` (`ldr` at 0xa7fc).
  Four-byte store: target `sp -0x34`, value `unknown` (`str` at 0xa800).
  Four-byte load: target `sp +0x0` (`ldr` at 0xa804).
  Four-byte store: target `sp -0x30`, value `[sp]` (`str` at 0xa808).
  Four-byte load: target `sp -0x18` (`ldr` at 0xa810).
  Four-byte load: target `sp -0x1c` (`ldr` at 0xa814).
  Call (`bl` at 0xa81c): (unknown callee), target 0x35f8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35f8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x35f8)` (`str` at 0xa824).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, including `ldr` at 0xa828, `ldr` at 0xa844.
  Branch (`bne` at 0xa830): taken to 0xa840 when `ret(linked ELF 0x35f8) != 0x0`, falls through to 0xa834 when `ret(linked ELF 0x35f8) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xa864`, linked address 0xa864, basis `ELF load-bias-relative` (`ldr` at 0xa834).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0xa838).
  Jump (`b` at 0xa83c): target 0xa858.
  Four-byte load: target `ELF load bias (unknown) +0xa868`, linked address 0xa868, basis `ELF load-bias-relative` (`ldr` at 0xa840).
  ELF literal warning: linked 0xa868, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xa840.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0xa848).
  Call (`bl` at 0xa84c): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, r0=`ret(linked ELF 0x35f8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x34f0)` (`str` at 0xa854).
  Four-byte load: target `sp -0x2c` (`ldr` at 0xa858).
  At the return (`ldm` at 0xa860), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0xa860): the modeled local edges stop before the trailing region 0xa864 to 0xa86c (exclusive). Calls remain opaque.
  Undecoded range: 0xa864 to 0xa868 (exclusive), 4 bytes `a4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0xa868 to 0xa868 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `L1DirectEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xa9d0 to 0xaa70 (exclusive), 160 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `0110df8337401431a6cd891aa684a6d956fe925ca5247fce06b7d03882cd07d8`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0xa9f4).
  Four-byte store: target `sp -0x38`, value `0x2` (`str` at 0xa9fc).
  Four-byte load: target `sp -0x1c` (`ldr` at 0xaa00).
  Four-byte store: target `sp -0x34`, value `r2` (`str` at 0xaa04).
  Four-byte load: target `sp -0x20` (`ldr` at 0xaa08).
  Four-byte store: target `sp -0x30`, value `r3` (`str` at 0xaa0c).
  Four-byte load: target `sp -0x14` (`ldr` at 0xaa10).
  Four-byte load: target `sp -0x18` (`ldr` at 0xaa18).
  Call (`bl` at 0xaa20): (unknown callee), target 0x35f8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35f8, r0=`r0`, r1=`0x0`, r2=`r1`, r3=`0x2`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x35f8)` (`str` at 0xaa28).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, including `ldr` at 0xaa2c, `ldr` at 0xaa48.
  Branch (`bne` at 0xaa34): taken to 0xaa44 when `ret(linked ELF 0x35f8) != 0x0`, falls through to 0xaa38 when `ret(linked ELF 0x35f8) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xaa68`, linked address 0xaa68, basis `ELF load-bias-relative` (`ldr` at 0xaa38).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0xaa3c).
  Jump (`b` at 0xaa40): target 0xaa5c.
  Four-byte load: target `ELF load bias (unknown) +0xaa6c`, linked address 0xaa6c, basis `ELF load-bias-relative` (`ldr` at 0xaa44).
  ELF literal warning: linked 0xaa6c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaa44.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0xaa4c).
  Call (`bl` at 0xaa50): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, r0=`ret(linked ELF 0x35f8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x34f0)` (`str` at 0xaa58).
  Four-byte load: target `sp -0x2c` (`ldr` at 0xaa5c).
  The exit (`ldm` at 0xaa64) reports r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0xaa64): the modeled local edges stop before the trailing region 0xaa68 to 0xaa70 (exclusive). Calls remain opaque.
  Undecoded range: 0xaa68 to 0xaa6c (exclusive), 4 bytes `a4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0xaa6c to 0xaa6c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `L1DirectResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xa86c to 0xa9d0 (exclusive), 356 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `0f242aea1ae05f6bde56d70c918f528a438c9022cb88d409c8686d46c3f5f90d`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0xa890).
  Grouped accesses: 3 4-byte load records with target `sp -0x14`, including `ldr` at 0xa894, `ldr` at 0xa89c, `ldr` at 0xa8d8.
  Four-byte store: target `sp -0x30`, value `r0` (`str` at 0xa898).
  Branch (`bne` at 0xa8a4): taken to 0xa8b4 when `r0 != 0x0`, falls through to 0xa8a8 when `r0 == 0x0`.
  Grouped accesses: 7 4-byte load records with target `sp -0x1c`, including `ldr` at 0xa8a8, `ldr` at 0xa8b4, `ldr` at 0xa8bc.
  Four-byte store: target `sp -0x30`, value `r2+24` (`str` at 0xa8b0).
  Two-byte load: target `r2 +0x22` (`ldrh` at 0xa8b8).
  Byte load: target `r2 +0x12` (`ldrb` at 0xa8c0).
  Four-byte store: target `sp -0x3c`, value `[r2+18]:1` (`str` at 0xa8c4).
  Four-byte load: target `sp -0x20` (`ldr` at 0xa8c8).
  Four-byte store: target `sp -0x38`, value `r3` (`str` at 0xa8cc).
  Four-byte load: target `sp +0x0` (`ldr` at 0xa8d0).
  Four-byte store: target `sp -0x34`, value `[sp]` (`str` at 0xa8d4).
  Four-byte load: target `sp -0x30` (`ldr` at 0xa8dc).
  Call (`bl` at 0xa8e4): (unknown callee), target 0x35f8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35f8, r0=`r0`, r1=`unknown`, r2=`[r2+34]:2`, r3=`0x1`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x35f8)` (`str` at 0xa8ec).
  Grouped accesses: 5 4-byte load records with target `sp -0x28`, including `ldr` at 0xa8f0, `ldr` at 0xa908, `ldr` at 0xa94c.
  Branch (`bne` at 0xa8f8): taken to 0xa908 when `ret(linked ELF 0x35f8) != 0x0`, falls through to 0xa8fc when `ret(linked ELF 0x35f8) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xa9c4`, linked address 0xa9c4, basis `ELF load-bias-relative` (`ldr` at 0xa8fc).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0xa900).
  Jump (`b` at 0xa904): target 0xa9b8.
  Four-byte load: target `sp -0x18` (`ldr` at 0xa90c).
  The analyzer invalidates unsupported effects (`rsb` at 0xa914).
  Four-byte load: target `ELF load bias (unknown) +0xa9c8`, linked address 0xa9c8, basis `ELF load-bias-relative` (`ldr` at 0xa918).
  The analyzer invalidates unsupported effects (`smull` at 0xa91c).
  The analyzer invalidates unsupported effects (`rsb` at 0xa928).
  The analyzer invalidates unsupported effects (`rsb` at 0xa944).
  Byte store: target `ret(linked ELF 0x35f8) +0x13`, value `unknown` (`strb` at 0xa948).
  Grouped accesses: 2 1-byte load records with target `[sp-28] +0x21`, including `ldrb` at 0xa958, `ldrb` at 0xa98c.
  Byte load: target `[sp-28] +0x14` (`ldrb` at 0xa960).
  The analyzer invalidates unsupported effects (`mvn` at 0xa968).
  Byte store: target `[sp-40] +0x14`, value `unknown` (`strb` at 0xa978).
  Byte load: target `[sp-28] +0x14` (`ldrb` at 0xa984).
  Byte store: target `[sp-40] +0x21`, value `unknown` (`strb` at 0xa99c).
  Four-byte load: target `ELF load bias (unknown) +0xa9cc`, linked address 0xa9cc, basis `ELF load-bias-relative` (`ldr` at 0xa9a0).
  ELF literal warning: linked 0xa9cc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xa9a0.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0xa9a8).
  Call (`bl` at 0xa9ac): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, r0=`[sp-40]`, r1=`unknown`, r2=`[[sp-28]+20]:1`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x34f0)` (`str` at 0xa9b4).
  Four-byte load: target `sp -0x2c` (`ldr` at 0xa9b8).
  Recorded r0 at the return is `unknown` (`ldm` at 0xa9c0).
  Unconditional return boundary (`ldm` at 0xa9c0): the modeled local edges stop before the trailing region 0xa9c4 to 0xa9d0 (exclusive). Calls remain opaque.
  Undecoded range: 0xa9c4 to 0xa9c8 (exclusive), 4 bytes `a4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0xa9c8 to 0xa9cc are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterRequestL3`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x4ba4 to 0x4c80 (exclusive), 220 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `08809f9000d0d33be81e03effcd5f5b540d30edff5586ff550cc61111836f29a`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x4bc8).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0x4bcc, `ldr` at 0x4c54.
  Call (`bl` at 0x4bd0): (unknown callee), target 0x1e28, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e28, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x4bdc): taken to 0x4bec when `ret(linked ELF 0x1e28) != 0x0`, falls through to 0x4be0 when `ret(linked ELF 0x1e28) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4c78`, linked address 0x4c78, basis `ELF load-bias-relative` (`ldr` at 0x4be0).
  Four-byte store: target `sp -0x2c`, value `0xfffffed1` (`str` at 0x4be4).
  Jump (`b` at 0x4be8): target 0x4c6c.
  Grouped accesses: 2 4-byte load records with target `sp +0x8`, including `ldr` at 0x4bec, `ldr` at 0x4bf8.
  Branch (`beq` at 0x4bf4): taken to 0x4c08 when `[sp+8] == 0x0`, falls through to 0x4bf8 when `[sp+8] != 0x0`.
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, including `ldr` at 0x4bfc, `ldr` at 0x4c20, `ldr` at 0x4c58.
  Two-byte load: target `[sp-24] +0x6` (`ldrh` at 0x4c00).
  Four-byte store: target `[sp+8] +0x0`, value `[[sp-24]+6]:2` (`str` at 0x4c04).
  Four-byte store: target `sp -0x38`, value `0x2` (`str` at 0x4c0c).
  Four-byte load: target `sp +0x0` (`ldr` at 0x4c10).
  Four-byte store: target `sp -0x34`, value `[sp]` (`str` at 0x4c14).
  Four-byte load: target `sp +0x4` (`ldr` at 0x4c18).
  Four-byte store: target `sp -0x30`, value `[sp+4]` (`str` at 0x4c1c).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x4c24).
  Four-byte load: target `sp -0x20` (`ldr` at 0x4c28).
  Call (`bl` at 0x4c30): (unknown callee), target 0x1d74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d74, r0=`[sp-24]`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`0x0`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x1d74)` (`str` at 0x4c38).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, including `ldr` at 0x4c3c, `ldr` at 0x4c5c.
  Branch (`bne` at 0x4c44): taken to 0x4c54 when `ret(linked ELF 0x1d74) != 0x0`, falls through to 0x4c48 when `ret(linked ELF 0x1d74) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4c7c`, linked address 0x4c7c, basis `ELF load-bias-relative` (`ldr` at 0x4c48).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0x4c4c).
  Jump (`b` at 0x4c50): target 0x4c6c.
  Call (`bl` at 0x4c60): (unknown callee), target 0x4b50, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x4b50, r0=`[sp-20]`, r1=`[sp-24]`, r2=`ret(linked ELF 0x1d74)`, r3=`ret(linked ELF 0x1d74)`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x4b50)` (`str` at 0x4c68).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x4c6c).
  `ldm` at 0x4c74 has recorded r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x4c74): the modeled local edges stop before the trailing region 0x4c78 to 0x4c80 (exclusive). Calls remain opaque.
  Undecoded range: 0x4c78 to 0x4c80 (exclusive), 8 bytes `d1feffffa4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.

- `IRouterEventL3`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x4de8 to 0x4ea8 (exclusive), 192 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `113e7680e0ca32fd55ef58216f427f5c7b8289d239f73b26a326e36b65f9a8cb`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x4e0c).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0x4e10, `ldr` at 0x4e7c.
  Call (`bl` at 0x4e14): (unknown callee), target 0x1e28, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e28, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x4e20): taken to 0x4e30 when `ret(linked ELF 0x1e28) != 0x0`, falls through to 0x4e24 when `ret(linked ELF 0x1e28) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4ea0`, linked address 0x4ea0, basis `ELF load-bias-relative` (`ldr` at 0x4e24).
  Four-byte store: target `sp -0x2c`, value `0xfffffed1` (`str` at 0x4e28).
  Jump (`b` at 0x4e2c): target 0x4e94.
  Four-byte store: target `sp -0x38`, value `0x2` (`str` at 0x4e34).
  Four-byte load: target `sp -0x20` (`ldr` at 0x4e38).
  Four-byte store: target `sp -0x34`, value `[sp-32]` (`str` at 0x4e3c).
  Four-byte load: target `sp +0x0` (`ldr` at 0x4e40).
  Four-byte store: target `sp -0x30`, value `[sp]` (`str` at 0x4e44).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x4e48, `ldr` at 0x4e80.
  Four-byte load: target `sp -0x1c` (`ldr` at 0x4e50).
  Call (`bl` at 0x4e58): (unknown callee), target 0x1d74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d74, r0=`[sp-24]`, r1=`0x0`, r2=`[sp-28]`, r3=`0x2`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x1d74)` (`str` at 0x4e60).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, including `ldr` at 0x4e64, `ldr` at 0x4e84.
  Branch (`bne` at 0x4e6c): taken to 0x4e7c when `ret(linked ELF 0x1d74) != 0x0`, falls through to 0x4e70 when `ret(linked ELF 0x1d74) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4ea4`, linked address 0x4ea4, basis `ELF load-bias-relative` (`ldr` at 0x4e70).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0x4e74).
  Jump (`b` at 0x4e78): target 0x4e94.
  Call (`bl` at 0x4e88): (unknown callee), target 0x4b50, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x4b50, r0=`[sp-20]`, r1=`[sp-24]`, r2=`ret(linked ELF 0x1d74)`, r3=`ret(linked ELF 0x1d74)`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x4b50)` (`str` at 0x4e90).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x4e94).
  At the return (`ldm` at 0x4e9c), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0x4e9c): the modeled local edges stop before the trailing region 0x4ea0 to 0x4ea8 (exclusive). Calls remain opaque.
  Undecoded range: 0x4ea0 to 0x4ea8 (exclusive), 8 bytes `d1feffffa4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.

- `IRouterResponseL3`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x4c80 to 0x4de8 (exclusive), 360 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `3f7fb2d77838035183a1f3815b37861cf7ac6566be0ffd5911bc44f97bebb6c3`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x4ca4).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0x4ca8, `ldr` at 0x4db8.
  Call (`bl` at 0x4cac): (unknown callee), target 0x1e28, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e28, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Branch (`bne` at 0x4cb8): taken to 0x4cc8 when `ret(linked ELF 0x1e28) != 0x0`, falls through to 0x4cbc when `ret(linked ELF 0x1e28) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4ddc`, linked address 0x4ddc, basis `ELF load-bias-relative` (`ldr` at 0x4cbc).
  Four-byte store: target `sp -0x2c`, value `0xfffffed1` (`str` at 0x4cc0).
  Jump (`b` at 0x4cc4): target 0x4dd0.
  Grouped accesses: 7 4-byte load records with target `sp -0x20`, including `ldr` at 0x4cc8, `ldr` at 0x4cd0, `ldr` at 0x4cd8.
  Two-byte load: target `[sp-32] +0x22` (`ldrh` at 0x4cd4).
  Byte load: target `[sp-32] +0x12` (`ldrb` at 0x4cdc).
  Four-byte store: target `sp -0x38`, value `[[sp-32]+18]:1` (`str` at 0x4ce0).
  Four-byte load: target `sp +0x0` (`ldr` at 0x4ce4).
  Four-byte store: target `sp -0x34`, value `[sp]` (`str` at 0x4ce8).
  Four-byte load: target `sp +0x4` (`ldr` at 0x4cec).
  Four-byte store: target `sp -0x30`, value `[sp+4]` (`str` at 0x4cf0).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x4cf4, `ldr` at 0x4dbc.
  Call (`bl` at 0x4cfc): (unknown callee), target 0x1d74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d74, r0=`[sp-24]`, r1=`[sp-32]+28`, r2=`[[sp-32]+34]:2`, r3=`0x1`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x1d74)` (`str` at 0x4d04).
  Grouped accesses: 5 4-byte load records with target `sp -0x28`, including `ldr` at 0x4d08, `ldr` at 0x4d20, `ldr` at 0x4d64.
  Branch (`bne` at 0x4d10): taken to 0x4d20 when `ret(linked ELF 0x1d74) != 0x0`, falls through to 0x4d14 when `ret(linked ELF 0x1d74) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4de0`, linked address 0x4de0, basis `ELF load-bias-relative` (`ldr` at 0x4d14).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0x4d18).
  Jump (`b` at 0x4d1c): target 0x4dd0.
  Four-byte load: target `sp -0x1c` (`ldr` at 0x4d24).
  The analyzer invalidates unsupported effects (`rsb` at 0x4d2c).
  Four-byte load: target `ELF load bias (unknown) +0x4de4`, linked address 0x4de4, basis `ELF load-bias-relative` (`ldr` at 0x4d30).
  The analyzer invalidates unsupported effects (`smull` at 0x4d34).
  The analyzer invalidates unsupported effects (`rsb` at 0x4d40).
  The analyzer invalidates unsupported effects (`rsb` at 0x4d5c).
  Byte store: target `ret(linked ELF 0x1d74) +0x13`, value `unknown` (`strb` at 0x4d60).
  Grouped accesses: 2 1-byte load records with target `[sp-32] +0x21`, including `ldrb` at 0x4d70, `ldrb` at 0x4da4.
  Byte load: target `[sp-32] +0x14` (`ldrb` at 0x4d78).
  The analyzer invalidates unsupported effects (`mvn` at 0x4d80).
  Byte store: target `[sp-40] +0x14`, value `unknown` (`strb` at 0x4d90).
  Byte load: target `[sp-32] +0x14` (`ldrb` at 0x4d9c).
  Byte store: target `[sp-40] +0x21`, value `unknown` (`strb` at 0x4db4).
  Call (`bl` at 0x4dc4): (unknown callee), target 0x4b50, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x4b50, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-40]`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x4b50)` (`str` at 0x4dcc).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x4dd0).
  The exit (`ldm` at 0x4dd8) reports r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x4dd8): the modeled local edges stop before the trailing region 0x4ddc to 0x4de8 (exclusive). Calls remain opaque.
  Undecoded range: 0x4ddc to 0x4de4 (exclusive), 8 bytes `d1feffffa4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x4de4 to 0x4de4 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `sn1ScanInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x3ab8 to 0x3c10 (exclusive), 344 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `03f21d326fd3ceac2945d4258ccbf92cdac6210036e8afcaba0513efd2abe74a`.
  Four-byte load: target `ELF load bias (unknown) +0x3bf8`, linked address 0x3bf8, basis `ELF load-bias-relative` (`ldr` at 0x3acc).
  ELF literal warning: linked 0x3bf8, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x3acc, `ldr` at 0x3bd0, `ldr` at 0x3bd4.
  Grouped accesses: 7 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x3ad0, `ldr` at 0x3b28, `ldr` at 0x3b48.
  Branch (`ble` at 0x3ad8): taken to 0x3b04 when `? <= 0x0 (signed, 32-bit)`, falls through to 0x3adc when `? > 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x3bfc`, linked address 0x3bfc, basis `ELF load-bias-relative` (`ldr` at 0x3adc).
  ELF literal warning: linked 0x3bfc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3adc.
  Four-byte store: target `sp -0x28`, value `unknown` (`str` at 0x3ae0).
  Four-byte load: target `ELF load bias (unknown) +0x3c00`, linked address 0x3c00, basis `ELF load-bias-relative` (`ldr` at 0x3ae8).
  ELF literal warning: linked 0x3c00, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3ae8.
  Call (`bl` at 0x3af4): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x4`, r1=`unknown`, r2=`0x3a`, r3=`0x0`.
  Four-byte store: target `sp -0x1c`, value `0x0` (`str` at 0x3afc).
  Jump (`b` at 0x3b00): target 0x3bec.
  Four-byte load: target `ELF load bias (unknown) +0x3c04`, linked address 0x3c04, basis `ELF load-bias-relative` (`ldr` at 0x3b04).
  Call (`bl` at 0x3b08): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, r0=`0xffff`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b0c).
  ELF literal warning: linked 0x3c08, value unknown (relocation, writable or unmapped bytes), 11 read records, including `ldr` at 0x3b0c, `ldr` at 0x3b1c, `ldr` at 0x3b20.
  Call (`bl` at 0x3b18): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, r0=`unknown`, r1=`0x0`, r2=`0x0`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b1c).
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b20).
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b24).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r1` (`str` at 0x3b34).
  Grouped accesses: 4 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r2`, including `str` at 0x3b38, `str` at 0x3b68, `str` at 0x3b88.
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b3c).
  Four-byte store: target `sp -0x20`, value `unknown` (`str` at 0x3b40).
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b44).
  Branch (`bge` at 0x3b50): taken to 0x3b74 when `? >= 0x0 (signed, 32-bit)`, falls through to 0x3b54 when `? < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b54).
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b58).
  Grouped accesses: 2 4-byte store records with target `sp -0x24`, reported value `unknown` in every record, including `str` at 0x3b6c, `str` at 0x3b8c.
  Jump (`b` at 0x3b70): target 0x3b90.
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b74).
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b78).
  Four-byte load: target `sp -0x24` (`ldr` at 0x3b90).
  Four-byte load: target `sp -0x20` (`ldr` at 0x3b94).
  Four-byte store: target `[sp-32] +0x0`, value `[sp-36]` (`str` at 0x3b98).
  Four-byte load: target `ELF load bias (unknown) +0x3c08`, linked address 0x3c08, basis `ELF load-bias-relative` (`ldr` at 0x3b9c).
  Four-byte load: target `ELF load bias (unknown) +0x3c0c`, linked address 0x3c0c, basis `ELF load-bias-relative` (`ldr` at 0x3ba0).
  ELF literal warning: linked 0x3c0c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3ba0.
  Four-byte load: target `sp -0x14` (`ldr` at 0x3ba4).
  Call (`bl` at 0x3bac): (unknown callee), target 0x364c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x364c, r0=`unknown`, r1=`[sp-20]`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x18`, value `ret(linked ELF 0x364c)` (`str` at 0x3bb4).
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, including `ldr` at 0x3bb8, `ldr` at 0x3bc4, `ldr` at 0x3be4.
  Branch (`bge` at 0x3bc0): taken to 0x3bd0 when `ret(linked ELF 0x364c) >= 0x0 (signed, 32-bit)`, falls through to 0x3bc4 when `ret(linked ELF 0x364c) < 0x0 (signed, 32-bit)`.
  Four-byte store: target `sp -0x1c`, value `ret(linked ELF 0x364c)` (`str` at 0x3bc8).
  Jump (`b` at 0x3bcc): target 0x3bec.
  Four-byte load: target `ELF load bias (unknown) +0x3bf8`, linked address 0x3bf8, basis `ELF load-bias-relative` (`ldr` at 0x3bd0).
  Four-byte load: target `ELF load bias (unknown) +0x3bf8`, linked address 0x3bf8, basis `ELF load-bias-relative` (`ldr` at 0x3bd4).
  Four-byte store: target `sp -0x1c`, value `[sp-24]` (`str` at 0x3be8).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x3bec).
  Recorded r0 at the return is `unknown` (`ldm` at 0x3bf4).
  Unconditional return boundary (`ldm` at 0x3bf4): the modeled local edges stop before the trailing region 0x3bf8 to 0x3c10 (exclusive). Calls remain opaque.
  The decoded instructions from 0x3bf8 to 0x3c00 are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x3c04 to 0x3c08 (exclusive), 4 bytes `ffff0000`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x3c08 to 0x3c0c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterConnectOS`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x5a4c to 0x5afc (exclusive), 176 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `2e3a6f9b121432d6fbc331fb8859d0f2ee859f8c55f7f1d7414bd29cf30530ca`.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x5a6c, `ldr` at 0x5aa4.
  Call (`bl` at 0x5a74): (unknown callee), target 0x1e10, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e10, r0=`r1`, r1=`0x2`, r2=`r2`, r3=`r3`.
  Four-byte load: target `ELF load bias (unknown) +0x5aec`, linked address 0x5aec, basis `ELF load-bias-relative` (`ldr` at 0x5a7c).
  ELF literal warning: linked 0x5aec, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5a7c.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x1e10)`, base register `r3` (`str` at 0x5a80).
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x5a84).
  The analyzer invalidates unsupported effects (`cmn` at 0x5a88).
  Branch (`bne` at 0x5a8c): taken to 0x5ad0 when `Z == 0`, falls through to 0x5a90 when `not (Z == 0)`.
  Call (`bl` at 0x5a90): (unknown callee), target 0x1d38, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d38, r0=`ret(linked ELF 0x1e10)`, r1=`unknown`, r2=`ret(linked ELF 0x1e10)`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x5af0`, linked address 0x5af0, basis `ELF load-bias-relative` (`ldr` at 0x5a9c).
  ELF literal warning: linked 0x5af0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5a9c.
  Four-byte store: target `sp -0x2c`, value `unknown` (`str` at 0x5aa0).
  Four-byte store: target `sp -0x28`, value `[sp-24]` (`str` at 0x5aa8).
  Four-byte load: target `ret(linked ELF 0x1d38) +0x0` (`ldr` at 0x5aac).
  Four-byte store: target `sp -0x24`, value `[ret(linked ELF 0x1d38)]` (`str` at 0x5ab0).
  Four-byte load: target `ELF load bias (unknown) +0x5af4`, linked address 0x5af4, basis `ELF load-bias-relative` (`ldr` at 0x5ab8).
  ELF literal warning: linked 0x5af4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5ab8.
  Call (`bl` at 0x5ac4): (unknown callee), target 0x1d80, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d80, r0=`0x3`, r1=`unknown`, r2=`0x59`, r3=`0x0`.
  The analyzer invalidates unsupported effects (`mvn` at 0x5ac8).
  Call (`bl` at 0x5acc): (unknown callee), target 0x1e40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e40, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x20` (`ldr` at 0x5ad0).
  Four-byte load: target `ELF load bias (unknown) +0x5af8`, linked address 0x5af8, basis `ELF load-bias-relative` (`ldr` at 0x5ad4).
  ELF literal warning: linked 0x5af8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5ad4.
  Four-byte store: target `[sp-32] +0x0`, value `unknown` (`str` at 0x5ad8).
  `ldm` at 0x5ae8 has recorded r0 = `0x0`.
  Unconditional return boundary (`ldm` at 0x5ae8): the modeled local edges stop before the trailing region 0x5aec to 0x5afc (exclusive). Calls remain opaque.
  The decoded instructions from 0x5aec to 0x5af4 are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x5af8 to 0x5afc (exclusive), 4 bytes `dcf20000`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.

- `IRouterSetQueueSuspendTime`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x2e70 to 0x2eb0 (exclusive), 64 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `ab0538fab12ea7cd8ff1197c1a86339fc8feaa41286937872b71a67364ea1440`.
  Four-byte load: target `ELF load bias (unknown) +0x2ea8`, linked address 0x2ea8, basis `ELF load-bias-relative` (`ldr` at 0x2e88).
  ELF literal warning: linked 0x2ea8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2e88.
  Four-byte load: target `sp -0x14` (`ldr` at 0x2e8c).
  Four-byte store: target `(unknown effective address)`, value `r0`, base register `r2` (`str` at 0x2e90).
  Four-byte load: target `ELF load bias (unknown) +0x2eac`, linked address 0x2eac, basis `ELF load-bias-relative` (`ldr` at 0x2e94).
  ELF literal warning: linked 0x2eac, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2e94.
  Four-byte load: target `sp -0x18` (`ldr` at 0x2e98).
  Four-byte store: target `(unknown effective address)`, value `[sp-24]`, base register `r2` (`str` at 0x2e9c).
  At the return (`ldm` at 0x2ea4), r0 is `r0`.
  Unconditional return boundary (`ldm` at 0x2ea4): the modeled local edges stop before the trailing region 0x2ea8 to 0x2eb0 (exclusive). Calls remain opaque.
  Undecoded range: 0x2ea8 to 0x2eb0 (exclusive), 8 bytes `f8f10000fcf10000`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
