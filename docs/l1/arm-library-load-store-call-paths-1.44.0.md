# ARM library loads, stores and calls

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
unresolved-transfer list also does not settle that question. Entries with trailing
ranges record the explicit unconditional return boundary and separately retain decoded
unreached runs and undecoded ranges. Calls remain opaque. Local nonreachability does not
establish runtime feasibility or global nonexecution.

The [complete facts](disasm/arm-1.44.0-libraries/load-store-call-paths-facts.txt) retain every access, relocation, constant and warning, including omitted frame saves, paired restores and paired initial core-register spill stores. Meaningful entry-stack, reloaded input, local return-value and outgoing accesses are retained.

- `l1d_read_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xb78c to 0xb880 (exclusive), 244 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `9034cea72ff313e1a2b1c900ea88fbcc7133b1dee4c154082637f6b103525607`.
  The 5 unapplied ELF relocation records at linked places 0xb86c, 0xb874, 0xb878, 0xb87c, 0xb870 retain unknown values.
  Four-byte load: target `ELF load bias (unknown) +0xb86c`, linked address 0xb86c, basis `ELF load-bias-relative` (`ldr` at 0xb7a8).
  ELF literal warning: linked 0xb86c, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xb7a8, `ldr` at 0xb804, `ldr` at 0xb838.
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xb7ac, `ldr` at 0xb808, `ldr` at 0xb824.
  Branch (`beq` at 0xb7b4): taken to 0xb7d8 when `? == 0x0`, falls through to 0xb7b8 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xb870`, linked address 0xb870, basis `ELF load-bias-relative` (`ldr` at 0xb7b8).
  ELF literal warning: linked 0xb870, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xb7b8, `ldr` at 0xb820.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, including `ldr` at 0xb7bc, `ldr` at 0xb7f4.
  Four-byte store: target `sp -0x24`, value `r2` (`str` at 0xb7c0).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0xb7c4).
  Four-byte load: target `ELF load bias (unknown) +0xb874`, linked address 0xb874, basis `ELF load-bias-relative` (`ldr` at 0xb7c8).
  ELF literal warning: linked 0xb874, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb7c8.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0xb7cc, `ldr` at 0xb7ec.
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, including `ldr` at 0xb7d0, `ldr` at 0xb7f0, `ldr` at 0xb84c.
  Call (`bl` at 0xb7d4): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Call (`bl` at 0xb7d8): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `ret(linked ELF 0x3580) +0x0`, value `0x0` (`str` at 0xb7e8).
  Call (`bl` at 0xb7f8): (unknown callee), target 0x3730, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3730, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`0x0`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x3730)` (`str` at 0xb800).
  Four-byte load: target `ELF load bias (unknown) +0xb86c`, linked address 0xb86c, basis `ELF load-bias-relative` (`ldr` at 0xb804).
  Branch (`beq` at 0xb810): taken to 0xb838 when `? == 0x0`, falls through to 0xb814 when `? != 0x0`.
  Call (`bl` at 0xb814): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x3730)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb870`, linked address 0xb870, basis `ELF load-bias-relative` (`ldr` at 0xb820).
  Four-byte load: target `ELF load bias (unknown) +0xb878`, linked address 0xb878, basis `ELF load-bias-relative` (`ldr` at 0xb828).
  ELF literal warning: linked 0xb878, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb828.
  Grouped accesses: 3 4-byte load records with target `sp -0x20`, including `ldr` at 0xb82c, `ldr` at 0xb850, `ldr` at 0xb85c.
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xb830).
  Call (`bl` at 0xb834): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-32]`, r3=`[ret(linked ELF 0x3580)]`.
  Four-byte load: target `ELF load bias (unknown) +0xb86c`, linked address 0xb86c, basis `ELF load-bias-relative` (`ldr` at 0xb838).
  Branch (`beq` at 0xb844): taken to 0xb85c when `? == 0x0`, falls through to 0xb848 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xb87c`, linked address 0xb87c, basis `ELF load-bias-relative` (`ldr` at 0xb848).
  ELF literal warning: linked 0xb87c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb848.
  Call (`bl` at 0xb858): (unknown callee), target 0xbfbc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xbfbc, r0=`unknown`, r1=`[sp-24]`, r2=`unknown`, r3=`0x1`.
  At the return (`ldm` at 0xb868), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0xb868): the modeled local edges stop before the trailing region 0xb86c to 0xb880 (exclusive).
  The decoded instructions from 0xb86c to 0xb87c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `l1d_write_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xbb64 to 0xbc5c (exclusive), 248 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `f7386a63bff4bb8547daec9ef61429dccf8efb159bd9b8ddbe092ba1e661df44`.
  The 6 unapplied ELF relocation records at linked places 0xbc44, 0xbc4c, 0xbc50, 0xbc54, 0xbc58, 0xbc48 retain unknown values.
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, including `ldr` at 0xbb80, `ldr` at 0xbb9c, `ldr` at 0xbbc8.
  Four-byte store: target `sp -0x20`, value `r2` (`str` at 0xbb84).
  Four-byte load: target `ELF load bias (unknown) +0xbc44`, linked address 0xbc44, basis `ELF load-bias-relative` (`ldr` at 0xbb88).
  ELF literal warning: linked 0xbc44, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xbb88, `ldr` at 0xbbdc, `ldr` at 0xbc00.
  Grouped accesses: 5 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xbb8c, `ldr` at 0xbbbc, `ldr` at 0xbbe0.
  Branch (`beq` at 0xbb94): taken to 0xbbb8 when `? == 0x0`, falls through to 0xbb98 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbc48`, linked address 0xbc48, basis `ELF load-bias-relative` (`ldr` at 0xbb98).
  ELF literal warning: linked 0xbc48, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xbb98, `ldr` at 0xbc1c.
  Four-byte store: target `sp -0x24`, value `r2` (`str` at 0xbba0).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0xbba4).
  Four-byte load: target `ELF load bias (unknown) +0xbc4c`, linked address 0xbc4c, basis `ELF load-bias-relative` (`ldr` at 0xbba8).
  ELF literal warning: linked 0xbc4c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbba8.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0xbbac, `ldr` at 0xbbc0.
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, including `ldr` at 0xbbb0, `ldr` at 0xbbc4, `ldr` at 0xbbf0.
  Call (`bl` at 0xbbb4): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Four-byte load: target `ELF load bias (unknown) +0xbc50`, linked address 0xbc50, basis `ELF load-bias-relative` (`ldr` at 0xbbb8).
  ELF literal warning: linked 0xbc50, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbbb8.
  Call (`mov` at 0xbbd0): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `ret(unknown)` (`str` at 0xbbd8).
  Four-byte load: target `ELF load bias (unknown) +0xbc44`, linked address 0xbc44, basis `ELF load-bias-relative` (`ldr` at 0xbbdc).
  Branch (`beq` at 0xbbe8): taken to 0xbc00 when `? == 0x0`, falls through to 0xbbec when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbc54`, linked address 0xbc54, basis `ELF load-bias-relative` (`ldr` at 0xbbec).
  ELF literal warning: linked 0xbc54, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbbec.
  Call (`bl` at 0xbbfc): (unknown callee), target 0xbfbc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xbfbc, r0=`unknown`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0xbc44`, linked address 0xbc44, basis `ELF load-bias-relative` (`ldr` at 0xbc00).
  Branch (`beq` at 0xbc0c): taken to 0xbc34 when `? == 0x0`, falls through to 0xbc10 when `? != 0x0`.
  Call (`bl` at 0xbc10): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xbc48`, linked address 0xbc48, basis `ELF load-bias-relative` (`ldr` at 0xbc1c).
  Four-byte load: target `ELF load bias (unknown) +0xbc58`, linked address 0xbc58, basis `ELF load-bias-relative` (`ldr` at 0xbc24).
  ELF literal warning: linked 0xbc58, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbc24.
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, including `ldr` at 0xbc28, `ldr` at 0xbc34.
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xbc2c).
  Call (`bl` at 0xbc30): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-32]`, r3=`[ret(linked ELF 0x3580)]`.
  The exit (`ldm` at 0xbc40) reports r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0xbc40): the modeled local edges stop before the trailing region 0xbc44 to 0xbc5c (exclusive).
  The decoded instructions from 0xbc44 to 0xbc58 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `l1d_write_packet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xbd14 to 0xbdfc (exclusive), 232 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `c847ca7e219c3e339009cdf484cbf9268c7f2a7ee45df1c1537ba7a8402aaac0`.
  The 7 unapplied ELF relocation records at linked places 0xbde0, 0xbde8, 0xbdec, 0xbdf4, 0xbdf8, 0xbde4, 0xbdf0 retain unknown values.
  Four-byte load: target `ELF load bias (unknown) +0xbde0`, linked address 0xbde0, basis `ELF load-bias-relative` (`ldr` at 0xbd2c).
  ELF literal warning: linked 0xbde0, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xbd2c, `ldr` at 0xbdb8.
  Grouped accesses: 5 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xbd30, `ldr` at 0xbd40, `ldr` at 0xbda4.
  Branch (`beq` at 0xbd38): taken to 0xbd54 when `? == 0x0`, falls through to 0xbd3c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbde4`, linked address 0xbde4, basis `ELF load-bias-relative` (`ldr` at 0xbd3c).
  ELF literal warning: linked 0xbde4, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xbd3c, `ldr` at 0xbda0, `ldr` at 0xbdc8.
  Four-byte load: target `ELF load bias (unknown) +0xbde8`, linked address 0xbde8, basis `ELF load-bias-relative` (`ldr` at 0xbd44).
  ELF literal warning: linked 0xbde8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbd44.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0xbd48, `ldr` at 0xbd68.
  Grouped accesses: 3 4-byte load records with target `sp -0x1c`, including `ldr` at 0xbd4c, `ldr` at 0xbd54, `ldr` at 0xbd6c.
  Call (`bl` at 0xbd50): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0xbd58).
  Four-byte store: target `sp -0x20`, value `unknown` (`str` at 0xbd5c).
  Four-byte load: target `ELF load bias (unknown) +0xbdec`, linked address 0xbdec, basis `ELF load-bias-relative` (`ldr` at 0xbd60).
  ELF literal warning: linked 0xbdec, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbd60.
  Four-byte load: target `ELF load bias (unknown) +0xbdf0`, linked address 0xbdf0, basis `ELF load-bias-relative` (`ldr` at 0xbd64).
  ELF literal warning: linked 0xbdf0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbd64.
  Call (`bl` at 0xbd70): (unknown callee), target 0x3718, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3718, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x3718)` (`str` at 0xbd78).
  Grouped accesses: 2 4-byte load records with target `sp -0x24`, including `ldr` at 0xbd7c, `ldr` at 0xbd94.
  Branch (`beq` at 0xbd84): taken to 0xbdb8 when `ret(linked ELF 0x3718) == 0x0`, falls through to 0xbd88 when `ret(linked ELF 0x3718) != 0x0`.
  Four-byte load: target `sp -0x20` (`ldr` at 0xbd88).
  Call (`bl` at 0xbd8c): (unknown callee), target 0xbe74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xbe74, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3718)`.
  Call (`bl` at 0xbd98): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xbde4`, linked address 0xbde4, basis `ELF load-bias-relative` (`ldr` at 0xbda0).
  Four-byte load: target `ELF load bias (unknown) +0xbdf4`, linked address 0xbdf4, basis `ELF load-bias-relative` (`ldr` at 0xbda8).
  ELF literal warning: linked 0xbdf4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbda8.
  Call (`bl` at 0xbdb4): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0xbe74)`, r3=`ret(linked ELF 0x37cc)`.
  Four-byte load: target `ELF load bias (unknown) +0xbde0`, linked address 0xbde0, basis `ELF load-bias-relative` (`ldr` at 0xbdb8).
  Branch (`beq` at 0xbdc4): taken to 0xbdd8 when `? == 0x0`, falls through to 0xbdc8 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbde4`, linked address 0xbde4, basis `ELF load-bias-relative` (`ldr` at 0xbdc8).
  Four-byte load: target `ELF load bias (unknown) +0xbdf8`, linked address 0xbdf8, basis `ELF load-bias-relative` (`ldr` at 0xbdd0).
  ELF literal warning: linked 0xbdf8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbdd0.
  Call (`bl` at 0xbdd4): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 at the return is `unknown` (`ldm` at 0xbddc).
  Unconditional return boundary (`ldm` at 0xbddc): the modeled local edges stop before the trailing region 0xbde0 to 0xbdfc (exclusive).
  The decoded instructions from 0xbde0 to 0xbdf8 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `l1d_read_ppp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xb880 to 0xb938 (exclusive), 184 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `b77113e82cd08cd769fd707db4dbde0d1dbd1190d43990d97477a1e4e6e1ee91`.
  The 6 unapplied ELF relocation records at linked places 0xb920, 0xb928, 0xb934, 0xb924, 0xb92c, 0xb930 retain unknown values.
  Four-byte load: target `ELF load bias (unknown) +0xb920`, linked address 0xb920, basis `ELF load-bias-relative` (`ldr` at 0xb89c).
  ELF literal warning: linked 0xb920, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xb89c, `ldr` at 0xb8f0.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xb8a0, `ldr` at 0xb8f4, `ldr` at 0xb904.
  Branch (`beq` at 0xb8a8): taken to 0xb8cc when `? == 0x0`, falls through to 0xb8ac when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xb924`, linked address 0xb924, basis `ELF load-bias-relative` (`ldr` at 0xb8ac).
  ELF literal warning: linked 0xb924, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xb8ac, `ldr` at 0xb900.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, including `ldr` at 0xb8b0, `ldr` at 0xb8cc.
  Four-byte store: target `sp -0x24`, value `r2` (`str` at 0xb8b4).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0xb8b8).
  Four-byte load: target `ELF load bias (unknown) +0xb928`, linked address 0xb928, basis `ELF load-bias-relative` (`ldr` at 0xb8bc).
  ELF literal warning: linked 0xb928, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb8bc.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0xb8c0, `ldr` at 0xb8dc.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0xb8c4, `ldr` at 0xb8e0.
  Call (`bl` at 0xb8c8): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Four-byte store: target `sp -0x24`, value `unknown` (`str` at 0xb8d0).
  Four-byte load: target `ELF load bias (unknown) +0xb92c`, linked address 0xb92c, basis `ELF load-bias-relative` (`ldr` at 0xb8d4).
  ELF literal warning: linked 0xb92c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb8d4.
  Four-byte load: target `ELF load bias (unknown) +0xb930`, linked address 0xb930, basis `ELF load-bias-relative` (`ldr` at 0xb8d8).
  ELF literal warning: linked 0xb930, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb8d8.
  Call (`bl` at 0xb8e4): (unknown callee), target 0x346c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x346c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x346c)` (`str` at 0xb8ec).
  Four-byte load: target `ELF load bias (unknown) +0xb920`, linked address 0xb920, basis `ELF load-bias-relative` (`ldr` at 0xb8f0).
  Branch (`beq` at 0xb8fc): taken to 0xb910 when `? == 0x0`, falls through to 0xb900 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xb924`, linked address 0xb924, basis `ELF load-bias-relative` (`ldr` at 0xb900).
  Four-byte load: target `ELF load bias (unknown) +0xb934`, linked address 0xb934, basis `ELF load-bias-relative` (`ldr` at 0xb908).
  ELF literal warning: linked 0xb934, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb908.
  Call (`bl` at 0xb90c): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x20` (`ldr` at 0xb910).
  `ldm` at 0xb91c has recorded r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0xb91c): the modeled local edges stop before the trailing region 0xb920 to 0xb938 (exclusive).
  The decoded instructions from 0xb920 to 0xb934 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `l1d_write_ppp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xbc5c to 0xbd14 (exclusive), 184 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `10cde306c9e73e35be96c4315ff445614b66c82734735c2a3cbab5e47f4cecc9`.
  The 6 unapplied ELF relocation records at linked places 0xbcfc, 0xbd04, 0xbd10, 0xbd00, 0xbd08, 0xbd0c retain unknown values.
  Four-byte load: target `ELF load bias (unknown) +0xbcfc`, linked address 0xbcfc, basis `ELF load-bias-relative` (`ldr` at 0xbc78).
  ELF literal warning: linked 0xbcfc, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xbc78, `ldr` at 0xbccc.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xbc7c, `ldr` at 0xbcd0, `ldr` at 0xbce0.
  Branch (`beq` at 0xbc84): taken to 0xbca8 when `? == 0x0`, falls through to 0xbc88 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbd00`, linked address 0xbd00, basis `ELF load-bias-relative` (`ldr` at 0xbc88).
  ELF literal warning: linked 0xbd00, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xbc88, `ldr` at 0xbcdc.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, including `ldr` at 0xbc8c, `ldr` at 0xbca8.
  Four-byte store: target `sp -0x24`, value `r2` (`str` at 0xbc90).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0xbc94).
  Four-byte load: target `ELF load bias (unknown) +0xbd04`, linked address 0xbd04, basis `ELF load-bias-relative` (`ldr` at 0xbc98).
  ELF literal warning: linked 0xbd04, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbc98.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, including `ldr` at 0xbc9c, `ldr` at 0xbcb8.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0xbca0, `ldr` at 0xbcbc.
  Call (`bl` at 0xbca4): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Four-byte store: target `sp -0x24`, value `unknown` (`str` at 0xbcac).
  Four-byte load: target `ELF load bias (unknown) +0xbd08`, linked address 0xbd08, basis `ELF load-bias-relative` (`ldr` at 0xbcb0).
  ELF literal warning: linked 0xbd08, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbcb0.
  Four-byte load: target `ELF load bias (unknown) +0xbd0c`, linked address 0xbd0c, basis `ELF load-bias-relative` (`ldr` at 0xbcb4).
  ELF literal warning: linked 0xbd0c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbcb4.
  Call (`bl` at 0xbcc0): (unknown callee), target 0x36e8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36e8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x36e8)` (`str` at 0xbcc8).
  Four-byte load: target `ELF load bias (unknown) +0xbcfc`, linked address 0xbcfc, basis `ELF load-bias-relative` (`ldr` at 0xbccc).
  Branch (`beq` at 0xbcd8): taken to 0xbcec when `? == 0x0`, falls through to 0xbcdc when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbd00`, linked address 0xbd00, basis `ELF load-bias-relative` (`ldr` at 0xbcdc).
  Four-byte load: target `ELF load bias (unknown) +0xbd10`, linked address 0xbd10, basis `ELF load-bias-relative` (`ldr` at 0xbce4).
  ELF literal warning: linked 0xbd10, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbce4.
  Call (`bl` at 0xbce8): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x20` (`ldr` at 0xbcec).
  At the return (`ldm` at 0xbcf8), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0xbcf8): the modeled local edges stop before the trailing region 0xbcfc to 0xbd14 (exclusive).
  The decoded instructions from 0xbcfc to 0xbd10 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `L1DirectGetPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xaaec to 0xac7c (exclusive), 400 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `50e2ba524bb7b59f7e5009576e3264d6b411050f8a72eade9ddb5a3afb9924ef`.
  The 4 unapplied ELF relocation records at linked places 0xac6c, 0xac70, 0xac74, 0xac78 retain unknown values.
  Four-byte store: target `sp -0x20`, value `0x0` (`str` at 0xab0c).
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0xab14).
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, including `ldr` at 0xab18, `ldr` at 0xab58, `ldr` at 0xab74.
  Branch (`bne` at 0xab20): taken to 0xab58 when `r1 != 0x0`, falls through to 0xab24 when `r1 == 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x20`, reported value `0x1` in every record, including `str` at 0xab28, `str` at 0xab68.
  Four-byte load: target `ELF load bias (unknown) +0xac6c`, linked address 0xac6c, basis `ELF load-bias-relative` (`ldr` at 0xab2c).
  ELF literal warning: linked 0xac6c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xab2c, `ldr` at 0xab6c.
  Grouped accesses: 2 4-byte store records with target `sp -0x28`, reported value `unknown` in every record, including `str` at 0xab30, `str` at 0xab70.
  Four-byte load: target `ELF load bias (unknown) +0xac70`, linked address 0xac70, basis `ELF load-bias-relative` (`ldr` at 0xab3c).
  ELF literal warning: linked 0xac70, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xab3c, `ldr` at 0xab7c.
  Call (`bl` at 0xab44): (unknown callee), target 0x3664, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3664, r0=`0x0`, r1=`0x3e8`, r2=`unknown`, r3=`0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xac74`, linked address 0xac74, basis `ELF load-bias-relative` (`ldr` at 0xab4c).
  ELF literal warning: linked 0xac74, value unknown (relocation, writable or unmapped bytes), 6 read records, including `ldr` at 0xab4c, `ldr` at 0xab8c, `ldr` at 0xaba0.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `ret(linked ELF 0x3664)` in every record, base register `r3`, including `str` at 0xab50, `str` at 0xab90.
  Jump (`b` at 0xab54): target 0xab94.
  The analyzer invalidates unsupported effects (`cmn` at 0xab5c).
  Branch (`beq` at 0xab60): taken to 0xab94 when `Z == 1`, falls through to 0xab64 when `not (Z == 1)`.
  Four-byte load: target `ELF load bias (unknown) +0xac6c`, linked address 0xac6c, basis `ELF load-bias-relative` (`ldr` at 0xab6c).
  Four-byte load: target `ELF load bias (unknown) +0xac70`, linked address 0xac70, basis `ELF load-bias-relative` (`ldr` at 0xab7c).
  Call (`bl` at 0xab84): (unknown callee), target 0x3664, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3664, r0=`[sp-24]`, r1=`0x0`, r2=`unknown`, r3=`0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xac74`, linked address 0xac74, basis `ELF load-bias-relative` (`ldr` at 0xab8c).
  Four-byte load: target `sp -0x20` (`ldr` at 0xab94).
  Branch (`beq` at 0xab9c): taken to 0xabc8 when `[sp-32] == 0x0`, falls through to 0xaba0 when `[sp-32] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xac74`, linked address 0xac74, basis `ELF load-bias-relative` (`ldr` at 0xaba0).
  Grouped accesses: 5 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xaba4, `ldr` at 0xabd0, `ldr` at 0xabfc.
  Branch (`bne` at 0xabac): taken to 0xabc8 when `? != 0x0`, falls through to 0xabb0 when `? == 0x0`.
  Grouped accesses: 4 4-byte load records with target `sp -0x14`, including `ldr` at 0xabb0, `ldr` at 0xabdc, `ldr` at 0xabf4.
  Grouped accesses: 2 4-byte store records with target `[sp-20] +0x0`, reported value `0x0` in every record, including `str` at 0xabb8, `str` at 0xabe4.
  The analyzer invalidates unsupported effects (`mvn` at 0xabbc).
  Grouped accesses: 2 4-byte store records with target `sp -0x24`, reported value `unknown` in every record, including `str` at 0xabc0, `str` at 0xabec.
  Jump (`b` at 0xabc4): target 0xac34.
  Call (`bl` at 0xabc8): (unknown callee), target 0x3838, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3838, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xac78`, linked address 0xac78, basis `ELF load-bias-relative` (`ldr` at 0xabcc).
  ELF literal warning: linked 0xac78, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xabcc, `ldr` at 0xabf8, `ldr` at 0xac04.
  Branch (`bne` at 0xabd8): taken to 0xabf4 when `? != 0x0`, falls through to 0xabdc when `? == 0x0`.
  The analyzer invalidates unsupported effects (`mvn` at 0xabe8).
  Jump (`b` at 0xabf0): target 0xac34.
  Four-byte load: target `ELF load bias (unknown) +0xac78`, linked address 0xac78, basis `ELF load-bias-relative` (`ldr` at 0xabf8).
  Four-byte store: target `[sp-20] +0x0`, value `unknown` (`str` at 0xac00).
  Four-byte load: target `ELF load bias (unknown) +0xac78`, linked address 0xac78, basis `ELF load-bias-relative` (`ldr` at 0xac04).
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r2`, including `str` at 0xac0c, `str` at 0xac58.
  Four-byte load: target `sp -0x1c` (`ldr` at 0xac10).
  Branch (`beq` at 0xac18): taken to 0xac34 when `[sp-28] == 0x0`, falls through to 0xac1c when `[sp-28] != 0x0`.
  Four-byte load: target `[sp-20] +0x0` (`ldr` at 0xac20).
  Byte load: target `[[sp-20]] +0x16` (`ldrb` at 0xac24).
  Branch (`beq` at 0xac2c): taken to 0xac34 when `[[[sp-20]]+22]:1 == 0x1`, falls through to 0xac30 when `[[[sp-20]]+22]:1 != 0x1`.
  Jump (`b` at 0xac30): target 0xab94.
  Four-byte load: target `ELF load bias (unknown) +0xac74`, linked address 0xac74, basis `ELF load-bias-relative` (`ldr` at 0xac34).
  Branch (`beq` at 0xac40): taken to 0xac5c when `? == 0x0`, falls through to 0xac44 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xac74`, linked address 0xac74, basis `ELF load-bias-relative` (`ldr` at 0xac44).
  Call (`bl` at 0xac4c): (unknown callee), target 0x34fc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xac74`, linked address 0xac74, basis `ELF load-bias-relative` (`ldr` at 0xac50).
  Four-byte load: target `sp -0x24` (`ldr` at 0xac5c).
  The exit (`ldm` at 0xac68) reports r0 = `[sp-36]`.
  Unconditional return boundary (`ldm` at 0xac68): the modeled local edges stop before the trailing region 0xac6c to 0xac7c (exclusive).
  The decoded instructions from 0xac6c to 0xac78 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterRequestOS`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x5b24 to 0x5c0c (exclusive), 232 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `e2090a58017c97694a691eb0e4f1778ba916b03b2ac27e69d0c87d867f10df08`.
  The 2 unapplied ELF relocation records at linked places 0x5bfc, 0x5c08 retain unknown values.
  Four-byte store: target `sp -0x14`, value `r0` (`str` at 0x5b34).
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x5b48).
  Four-byte load: target `ELF load bias (unknown) +0x5bfc`, linked address 0x5bfc, basis `ELF load-bias-relative` (`ldr` at 0x5b4c).
  ELF literal warning: linked 0x5bfc, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x5b4c, `ldr` at 0x5bd0.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x5b50, `ldr` at 0x5bdc.
  The analyzer invalidates unsupported effects (`cmn` at 0x5b54).
  Branch (`bne` at 0x5b58): taken to 0x5b68 when `Z == 0`, falls through to 0x5b5c when `not (Z == 0)`.
  Four-byte load: target `ELF load bias (unknown) +0x5c00`, linked address 0x5c00, basis `ELF load-bias-relative` (`ldr` at 0x5b5c).
  Four-byte store: target `sp -0x2c`, value `0xfffffed1` (`str` at 0x5b60).
  Jump (`b` at 0x5b64): target 0x5bf0.
  Grouped accesses: 2 4-byte load records with target `sp +0x8`, including `ldr` at 0x5b68, `ldr` at 0x5b74.
  Branch (`beq` at 0x5b70): taken to 0x5b84 when `[sp+8] == 0x0`, falls through to 0x5b74 when `[sp+8] != 0x0`.
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, including `ldr` at 0x5b78, `ldr` at 0x5b9c, `ldr` at 0x5bd4.
  Two-byte load: target `[sp-24] +0x6` (`ldrh` at 0x5b7c).
  Four-byte store: target `[sp+8] +0x0`, value `[[sp-24]+6]:2` (`str` at 0x5b80).
  Four-byte store: target `sp -0x38`, value `0x2` (`str` at 0x5b88).
  Four-byte load: target `sp +0x0` (`ldr` at 0x5b8c).
  Four-byte store: target `sp -0x34`, value `[sp]` (`str` at 0x5b90).
  Four-byte load: target `sp +0x4` (`ldr` at 0x5b94).
  Four-byte store: target `sp -0x30`, value `[sp+4]` (`str` at 0x5b98).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x5ba0).
  Four-byte load: target `sp -0x20` (`ldr` at 0x5ba4).
  Call (`bl` at 0x5bac): (unknown callee), target 0x1d74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d74, r0=`[sp-24]`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`0x0`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x1d74)` (`str` at 0x5bb4).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, including `ldr` at 0x5bb8, `ldr` at 0x5be0.
  Branch (`bne` at 0x5bc0): taken to 0x5bd0 when `ret(linked ELF 0x1d74) != 0x0`, falls through to 0x5bc4 when `ret(linked ELF 0x1d74) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5c04`, linked address 0x5c04, basis `ELF load-bias-relative` (`ldr` at 0x5bc4).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0x5bc8).
  Jump (`b` at 0x5bcc): target 0x5bf0.
  Four-byte load: target `ELF load bias (unknown) +0x5bfc`, linked address 0x5bfc, basis `ELF load-bias-relative` (`ldr` at 0x5bd0).
  Four-byte load: target `ELF load bias (unknown) +0x5c08`, linked address 0x5c08, basis `ELF load-bias-relative` (`ldr` at 0x5bd8).
  ELF literal warning: linked 0x5c08, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5bd8.
  Call (`bl` at 0x5be4): (unknown callee), target 0x1de0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1de0, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x1d74)`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x1de0)` (`str` at 0x5bec).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x5bf0).
  Recorded r0 at the return is `unknown` (`ldm` at 0x5bf8).
  Unconditional return boundary (`ldm` at 0x5bf8): the modeled local edges stop before the trailing region 0x5bfc to 0x5c0c (exclusive).
  The decoded instructions from 0x5bfc to 0x5bfc are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x5c00 to 0x5c08 (exclusive), 8 bytes `d1feffffa4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x5c08 to 0x5c08 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterResponseOS`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x5c0c to 0x5d80 (exclusive), 372 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `b503e04914b9b17fe525341b72e57122797110405a2ea45c914b7131123825be`.
  The 2 unapplied ELF relocation records at linked places 0x5d6c, 0x5d7c retain unknown values.
  Four-byte store: target `sp -0x14`, value `r0` (`str` at 0x5c1c).
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x5c30).
  Four-byte load: target `ELF load bias (unknown) +0x5d6c`, linked address 0x5d6c, basis `ELF load-bias-relative` (`ldr` at 0x5c34).
  ELF literal warning: linked 0x5d6c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x5c34, `ldr` at 0x5d40.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x5c38, `ldr` at 0x5d4c.
  The analyzer invalidates unsupported effects (`cmn` at 0x5c3c).
  Branch (`bne` at 0x5c40): taken to 0x5c50 when `Z == 0`, falls through to 0x5c44 when `not (Z == 0)`.
  Four-byte load: target `ELF load bias (unknown) +0x5d70`, linked address 0x5d70, basis `ELF load-bias-relative` (`ldr` at 0x5c44).
  Four-byte store: target `sp -0x2c`, value `0xfffffed1` (`str` at 0x5c48).
  Jump (`b` at 0x5c4c): target 0x5d60.
  Grouped accesses: 7 4-byte load records with target `sp -0x20`, including `ldr` at 0x5c50, `ldr` at 0x5c58, `ldr` at 0x5c60.
  Two-byte load: target `[sp-32] +0x22` (`ldrh` at 0x5c5c).
  Byte load: target `[sp-32] +0x12` (`ldrb` at 0x5c64).
  Four-byte store: target `sp -0x38`, value `[[sp-32]+18]:1` (`str` at 0x5c68).
  Four-byte load: target `sp +0x0` (`ldr` at 0x5c6c).
  Four-byte store: target `sp -0x34`, value `[sp]` (`str` at 0x5c70).
  Four-byte load: target `sp +0x4` (`ldr` at 0x5c74).
  Four-byte store: target `sp -0x30`, value `[sp+4]` (`str` at 0x5c78).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x5c7c, `ldr` at 0x5d44.
  Call (`bl` at 0x5c84): (unknown callee), target 0x1d74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d74, r0=`[sp-24]`, r1=`[sp-32]+28`, r2=`[[sp-32]+34]:2`, r3=`0x1`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x1d74)` (`str` at 0x5c8c).
  Grouped accesses: 5 4-byte load records with target `sp -0x28`, including `ldr` at 0x5c90, `ldr` at 0x5ca8, `ldr` at 0x5cec.
  Branch (`bne` at 0x5c98): taken to 0x5ca8 when `ret(linked ELF 0x1d74) != 0x0`, falls through to 0x5c9c when `ret(linked ELF 0x1d74) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5d74`, linked address 0x5d74, basis `ELF load-bias-relative` (`ldr` at 0x5c9c).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0x5ca0).
  Jump (`b` at 0x5ca4): target 0x5d60.
  Four-byte load: target `sp -0x1c` (`ldr` at 0x5cac).
  The analyzer invalidates unsupported effects (`rsb` at 0x5cb4).
  Four-byte load: target `ELF load bias (unknown) +0x5d78`, linked address 0x5d78, basis `ELF load-bias-relative` (`ldr` at 0x5cb8).
  The analyzer invalidates unsupported effects (`smull` at 0x5cbc).
  The analyzer invalidates unsupported effects (`rsb` at 0x5cc8).
  The analyzer invalidates unsupported effects (`rsb` at 0x5ce4).
  Byte store: target `ret(linked ELF 0x1d74) +0x13`, value `unknown` (`strb` at 0x5ce8).
  Grouped accesses: 2 1-byte load records with target `[sp-32] +0x21`, including `ldrb` at 0x5cf8, `ldrb` at 0x5d2c.
  Byte load: target `[sp-32] +0x14` (`ldrb` at 0x5d00).
  The analyzer invalidates unsupported effects (`mvn` at 0x5d08).
  Byte store: target `[sp-40] +0x14`, value `unknown` (`strb` at 0x5d18).
  Byte load: target `[sp-32] +0x14` (`ldrb` at 0x5d24).
  Byte store: target `[sp-40] +0x21`, value `unknown` (`strb` at 0x5d3c).
  Four-byte load: target `ELF load bias (unknown) +0x5d6c`, linked address 0x5d6c, basis `ELF load-bias-relative` (`ldr` at 0x5d40).
  Four-byte load: target `ELF load bias (unknown) +0x5d7c`, linked address 0x5d7c, basis `ELF load-bias-relative` (`ldr` at 0x5d48).
  ELF literal warning: linked 0x5d7c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5d48.
  Call (`bl` at 0x5d54): (unknown callee), target 0x1de0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1de0, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x1de0)` (`str` at 0x5d5c).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x5d60).
  `ldm` at 0x5d68 has recorded r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x5d68): the modeled local edges stop before the trailing region 0x5d6c to 0x5d80 (exclusive).
  The decoded instructions from 0x5d6c to 0x5d6c are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x5d70 to 0x5d78 (exclusive), 8 bytes `d1feffffa4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x5d78 to 0x5d7c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterEventOS`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x5d80 to 0x5e4c (exclusive), 204 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `566c666c1cb22cd4d8fd26accbef98c497a6aed22fce642d772e636d6956b133`.
  The 2 unapplied ELF relocation records at linked places 0x5e3c, 0x5e48 retain unknown values.
  Four-byte store: target `sp -0x14`, value `r0` (`str` at 0x5d90).
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x5da4).
  Four-byte load: target `ELF load bias (unknown) +0x5e3c`, linked address 0x5e3c, basis `ELF load-bias-relative` (`ldr` at 0x5da8).
  ELF literal warning: linked 0x5e3c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x5da8, `ldr` at 0x5e10.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x5dac, `ldr` at 0x5e1c.
  The analyzer invalidates unsupported effects (`cmn` at 0x5db0).
  Branch (`bne` at 0x5db4): taken to 0x5dc4 when `Z == 0`, falls through to 0x5db8 when `not (Z == 0)`.
  Four-byte load: target `ELF load bias (unknown) +0x5e40`, linked address 0x5e40, basis `ELF load-bias-relative` (`ldr` at 0x5db8).
  Four-byte store: target `sp -0x2c`, value `0xfffffed1` (`str` at 0x5dbc).
  Jump (`b` at 0x5dc0): target 0x5e30.
  Four-byte store: target `sp -0x38`, value `0x2` (`str` at 0x5dc8).
  Four-byte load: target `sp -0x20` (`ldr` at 0x5dcc).
  Four-byte store: target `sp -0x34`, value `[sp-32]` (`str` at 0x5dd0).
  Four-byte load: target `sp +0x0` (`ldr` at 0x5dd4).
  Four-byte store: target `sp -0x30`, value `[sp]` (`str` at 0x5dd8).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x5ddc, `ldr` at 0x5e14.
  Four-byte load: target `sp -0x1c` (`ldr` at 0x5de4).
  Call (`bl` at 0x5dec): (unknown callee), target 0x1d74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d74, r0=`[sp-24]`, r1=`0x0`, r2=`[sp-28]`, r3=`0x2`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x1d74)` (`str` at 0x5df4).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, including `ldr` at 0x5df8, `ldr` at 0x5e20.
  Branch (`bne` at 0x5e00): taken to 0x5e10 when `ret(linked ELF 0x1d74) != 0x0`, falls through to 0x5e04 when `ret(linked ELF 0x1d74) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5e44`, linked address 0x5e44, basis `ELF load-bias-relative` (`ldr` at 0x5e04).
  Four-byte store: target `sp -0x2c`, value `0xfffffda4` (`str` at 0x5e08).
  Jump (`b` at 0x5e0c): target 0x5e30.
  Four-byte load: target `ELF load bias (unknown) +0x5e3c`, linked address 0x5e3c, basis `ELF load-bias-relative` (`ldr` at 0x5e10).
  Four-byte load: target `ELF load bias (unknown) +0x5e48`, linked address 0x5e48, basis `ELF load-bias-relative` (`ldr` at 0x5e18).
  ELF literal warning: linked 0x5e48, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5e18.
  Call (`bl` at 0x5e24): (unknown callee), target 0x1de0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1de0, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x1d74)`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x1de0)` (`str` at 0x5e2c).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x5e30).
  At the return (`ldm` at 0x5e38), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0x5e38): the modeled local edges stop before the trailing region 0x5e3c to 0x5e4c (exclusive).
  The decoded instructions from 0x5e3c to 0x5e3c are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x5e40 to 0x5e48 (exclusive), 8 bytes `d1feffffa4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x5e48 to 0x5e48 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterWritePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x360c to 0x3770 (exclusive), 356 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `52d1a6976d36b2083724bb79b0b67993473e46f35f93d8a79807b10ca6b8c553`.
  The analyzer retains no ELF relocation records for this span.
  Four-byte store: target `sp -0x14`, value `r0` (`str` at 0x361c).
  Grouped accesses: 4 4-byte load records with target `sp -0x20`, including `ldr` at 0x362c, `ldr` at 0x3688, `ldr` at 0x3700.
  Four-byte store: target `sp -0x24`, value `r3` (`str` at 0x3630).
  Grouped accesses: 5 4-byte load records with target `sp -0x24`, including `ldr` at 0x3634, `ldr` at 0x3640, `ldr` at 0x364c.
  Branch (`beq` at 0x363c): taken to 0x374c when `Z == 1`, falls through to 0x3640 when `not (Z == 1)`.
  Two-byte load: target `(unknown effective address)`, base register `r3` (`ldrh` at 0x3644).
  Four-byte store: target `sp -0x28`, value `unknown` (`str` at 0x3648).
  Byte load: target `(unknown effective address)`, base register `r3` (`ldrb` at 0x3650).
  Four-byte store: target `sp -0x2c`, value `unknown` (`str` at 0x3660).
  Grouped accesses: 4 4-byte load records with target `sp -0x28`, including `ldr` at 0x3664, `ldr` at 0x369c, `ldr` at 0x36d4.
  Call (`bl` at 0x3670): (unknown callee), target 0x1d68, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d68, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x34`, value `ret(linked ELF 0x1d68)` (`str` at 0x3678).
  Grouped accesses: 6 4-byte load records with target `sp -0x34`, including `ldr` at 0x367c, `ldr` at 0x36a4, `ldr` at 0x36b4.
  Branch (`bne` at 0x3684): taken to 0x369c when `ret(linked ELF 0x1d68) != 0x0`, falls through to 0x3688 when `ret(linked ELF 0x1d68) == 0x0`.
  Call (`bl` at 0x368c): (unknown callee), target 0x1c90, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c90, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x1d68)`.
  Four-byte load: target `ELF load bias (unknown) +0x3768`, linked address 0x3768, basis `ELF load-bias-relative` (`ldr` at 0x3690).
  Four-byte store: target `sp -0x38`, value `0xfffffda4` (`str` at 0x3694).
  Jump (`b` at 0x3698): target 0x375c.
  Call (`bl` at 0x36b0): (unknown callee), target 0x1df8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1df8, r0=`ret(linked ELF 0x1d68)`, r1=`[sp-36]`, r2=`[sp-40]+16`, r3=`[sp-40]+16`.
  Call (`bl` at 0x36bc): (unknown callee), target 0x1cd8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cd8, r0=`[sp-52]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x18` (`ldr` at 0x36c8).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x36cc).
  Call (`mov` at 0x36dc): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`[sp-28]`, r1=`[sp-52]+16`, r2=`[sp-40]`, r3=`[sp-52]+16`.
  Four-byte store: target `sp -0x30`, value `ret(unknown)` (`str` at 0x36e4).
  Grouped accesses: 2 4-byte load records with target `sp -0x30`, including `ldr` at 0x36e8, `ldr` at 0x3708.
  Branch (`beq` at 0x36f4): taken to 0x3734 when `ret(unknown) == [sp-40]`, falls through to 0x36f8 when `ret(unknown) != [sp-40]`.
  Call (`bl` at 0x36fc): (unknown callee), target 0x1e4c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e4c, r0=`[sp-52]`, r1=`unknown`, r2=`ret(unknown)`, r3=`[sp-40]`.
  Call (`bl` at 0x3704): (unknown callee), target 0x1c90, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c90, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x3710): taken to 0x3720 when `[sp-48] != 0x0`, falls through to 0x3714 when `[sp-48] == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x376c`, linked address 0x376c, basis `ELF load-bias-relative` (`ldr` at 0x3714).
  Four-byte store: target `sp -0x3c`, value `0xfffffd92` (`str` at 0x3718).
  Jump (`b` at 0x371c): target 0x3728.
  The analyzer invalidates unsupported effects (`mvn` at 0x3720).
  Four-byte store: target `sp -0x3c`, value `unknown` (`str` at 0x3724).
  Four-byte load: target `sp -0x3c` (`ldr` at 0x3728).
  Four-byte store: target `sp -0x38`, value `[sp-60]` (`str` at 0x372c).
  Jump (`b` at 0x3730): target 0x375c.
  Call (`bl` at 0x3738): (unknown callee), target 0x1e4c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e4c, r0=`[sp-52]`, r1=`unknown`, r2=`ret(unknown)`, r3=`[sp-40]`.
  Four-byte load: target `[sp-36] +0x0` (`ldr` at 0x3740).
  Four-byte store: target `sp -0x24`, value `[[sp-36]]` (`str` at 0x3744).
  Jump (`b` at 0x3748): target 0x3634.
  Call (`bl` at 0x3750): (unknown callee), target 0x1c90, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c90, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x38`, value `0x0` (`str` at 0x3758).
  Four-byte load: target `sp -0x38` (`ldr` at 0x375c).
  The exit (`ldm` at 0x3764) reports r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x3764): the modeled local edges stop before the trailing region 0x3768 to 0x3770 (exclusive).
  Undecoded range: 0x3768 to 0x3770 (exclusive), 8 bytes `a4fdffff92fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.

- `IRouterPacketFromRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x46c8 to 0x4858 (exclusive), 400 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `27f83096d10277dcd85811823443595b53c3c30222450030a1cb87839aa2846f`.
  The 2 unapplied ELF relocation records at linked places 0x4850, 0x4854 retain unknown values.
  Grouped accesses: 6 4-byte load records with target `sp -0x18`, including `ldr` at 0x46e0, `ldr` at 0x4730, `ldr` at 0x4798.
  Four-byte load: target `ELF load bias (unknown) +0x4850`, linked address 0x4850, basis `ELF load-bias-relative` (`ldr` at 0x46e4).
  ELF literal warning: linked 0x4850, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x46e4.
  Four-byte load: target `r1 +0x4` (`ldr` at 0x46e8).
  Call (`bl` at 0x46ec): (unknown callee), target 0x1ca8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1ca8, r0=`unknown`, r1=`[r1+4]`, r2=`r2`, r3=`r1`.
  Branch (`beq` at 0x46f8): taken to 0x4708 when `ret(linked ELF 0x1ca8) == 0x0`, falls through to 0x46fc when `ret(linked ELF 0x1ca8) != 0x0`.
  Four-byte store: target `sp -0x2c`, value `0x0` (`str` at 0x4700).
  Jump (`b` at 0x4704): target 0x4844.
  Four-byte load: target `sp -0x14` (`ldr` at 0x4708).
  Four-byte store: target `sp -0x20`, value `[sp-20]-1` (`str` at 0x4710).
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x4718).
  Grouped accesses: 7 4-byte load records with target `sp -0x20`, including `ldr` at 0x471c, `ldr` at 0x4728, `ldr` at 0x4790.
  Branch (`ble` at 0x4724): taken to 0x4834 when `le flags condition (operands unknown)`, falls through to 0x4728 when `not (le flags condition (operands unknown))`.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x4738).
  Four-byte store: target `sp -0x28`, value `unknown` (`str` at 0x473c).
  Four-byte load: target `ELF load bias (unknown) +0x4854`, linked address 0x4854, basis `ELF load-bias-relative` (`ldr` at 0x4740).
  ELF literal warning: linked 0x4854, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4740.
  Grouped accesses: 5 4-byte load records with target `sp -0x28`, including `ldr` at 0x4744, `ldr` at 0x4750, `ldr` at 0x475c.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x4748).
  Call (`bl` at 0x474c): (unknown callee), target 0x1cd8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cd8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte load records with target `sp -0x24`, including `ldr` at 0x4754, `ldr` at 0x483c.
  Four-byte store: target `[sp-40] +0x0`, value `[sp-36]` (`str` at 0x4758).
  Two-byte load: target `[sp-40] +0x4` (`ldrh` at 0x4760).
  Two-byte store: target `sp -0x30`, value `([[sp-40]+4]:2 & 0xffff)` (`strh` at 0x4764).
  Grouped accesses: 2 2-byte load records with target `sp -0x30`, including `ldrh` at 0x4768, `ldrh` at 0x477c.
  Two-byte store: target `sp -0x30`, value `unknown` (`strh` at 0x4778).
  Two-byte store: target `[sp-40] +0x4`, value `([sp-48]:2 & 0xffff)` (`strh` at 0x4780).
  Four-byte store: target `[sp-40] +0x8`, value `0x0` (`str` at 0x478c).
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r2`, including `ldrb` at 0x47a0, `ldrb` at 0x47e4.
  Grouped accesses: 2 1-byte store records with target `sp -0x34`, reported value `unknown` in every record, including `strb` at 0x47a4, `strb` at 0x47b0.
  Grouped accesses: 2 1-byte load records with target `sp -0x34`, including `ldrb` at 0x47a8, `ldrb` at 0x47b4.
  Byte store: target `(unknown effective address)`, value `([sp-52]:1 & 0xff)`, base register `r2` (`strb` at 0x47b8).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2` (`str` at 0x47d0).
  Byte store: target `(unknown effective address)`, value `unknown`, base register `r2` (`strb` at 0x47ec).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0x4800).
  Grouped accesses: 2 4-byte store records with target `sp -0x38`, reported value `unknown` in every record, including `str` at 0x4804, `str` at 0x4810.
  Grouped accesses: 2 4-byte load records with target `sp -0x38`, including `ldr` at 0x4808, `ldr` at 0x4814.
  Four-byte store: target `(unknown effective address)`, value `[sp-56]`, base register `r2` (`str` at 0x4818).
  Four-byte store: target `sp -0x24`, value `[sp-40]` (`str` at 0x4820).
  Four-byte store: target `sp -0x20`, value `[sp-32]-1` (`str` at 0x482c).
  Jump (`b` at 0x4830): target 0x471c.
  Call (`bl` at 0x4838): (unknown callee), target 0x1eac, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1eac, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `[sp-36]` (`str` at 0x4840).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x4844).
  Recorded r0 at the return is `unknown` (`ldm` at 0x484c).
  Unconditional return boundary (`ldm` at 0x484c): the modeled local edges stop before the trailing region 0x4850 to 0x4858 (exclusive).
  The decoded instructions from 0x4850 to 0x4854 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `sn1ScanReq`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x3c58 to 0x3dd8 (exclusive), 384 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `6cd96810cccf4fbd94e9e81213b8f6321177a611be28a8f6f58e9e8d94a4a009`.
  The 6 unapplied ELF relocation records at linked places 0x3dbc, 0x3dc0, 0x3dc4, 0x3dcc, 0x3dd4, 0x3dc8 retain unknown values.
  Four-byte load: target `ELF load bias (unknown) +0x3dbc`, linked address 0x3dbc, basis `ELF load-bias-relative` (`ldr` at 0x3c78).
  ELF literal warning: linked 0x3dbc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3c78.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x3c7c).
  Branch (`bne` at 0x3c84): taken to 0x3ccc when `? != 0x0`, falls through to 0x3c88 when `? == 0x0`.
  Call (`bl` at 0x3c8c): (unknown callee), target 0x3814, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3814, r0=`0x0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x3814)` (`str` at 0x3c94).
  Grouped accesses: 5 4-byte load records with target `sp -0x24`, including `ldr` at 0x3c98, `ldr` at 0x3cc0, `ldr` at 0x3d64.
  Branch (`bge` at 0x3ca0): taken to 0x3ccc when `ret(linked ELF 0x3814) >= 0x0 (signed, 32-bit)`, falls through to 0x3ca4 when `ret(linked ELF 0x3814) < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x3dc0`, linked address 0x3dc0, basis `ELF load-bias-relative` (`ldr` at 0x3ca4).
  ELF literal warning: linked 0x3dc0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3ca4.
  Grouped accesses: 2 4-byte store records with target `sp -0x3c`, reported value `unknown` in every record, including `str` at 0x3ca8, `str` at 0x3d80.
  Four-byte load: target `ELF load bias (unknown) +0x3dc4`, linked address 0x3dc4, basis `ELF load-bias-relative` (`ldr` at 0x3cb0).
  ELF literal warning: linked 0x3dc4, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x3cb0, `ldr` at 0x3d8c.
  Call (`bl` at 0x3cbc): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x3`, r1=`unknown`, r2=`0x6c`, r3=`0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[sp-36]` in every record, including `str` at 0x3cc4, `str` at 0x3da0.
  Jump (`b` at 0x3cc8): target 0x3db0.
  The analyzer invalidates unsupported effects (`mvn` at 0x3ccc).
  Byte store: target `(unknown effective address)`, value `0x1`, base register `r2` (`strb` at 0x3cdc).
  The analyzer invalidates unsupported effects (`mvn` at 0x3ce0).
  Byte store: target `(unknown effective address)`, value `0x0`, base register `r2` (`strb` at 0x3cf0).
  The analyzer invalidates unsupported effects (`mvn` at 0x3cf4).
  Four-byte load: target `sp -0x18` (`ldr` at 0x3d00).
  Four-byte store: target `(unknown effective address)`, value `[sp-24]`, base register `r2` (`str` at 0x3d04).
  The analyzer invalidates unsupported effects (`mvn` at 0x3d08).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x3d14).
  Two-byte store: target `(unknown effective address)`, value `([sp-28] & 0xffff)`, base register `r2` (`strh` at 0x3d18).
  Call (`bl` at 0x3d1c): (unknown callee), target 0x37b4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37b4, r0=`unknown`, r1=`sp-16`, r2=`unknown`, r3=`[sp-28]`.
  Four-byte load: target `ELF load bias (unknown) +0x3dc8`, linked address 0x3dc8, basis `ELF load-bias-relative` (`ldr` at 0x3d24).
  ELF literal warning: linked 0x3dc8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3d24.
  Four-byte store: target `sp -0x3c`, value `0x1` (`str` at 0x3d2c).
  Four-byte store: target `sp -0x38`, value `sp-44` (`str` at 0x3d34).
  Four-byte load: target `sp -0x20` (`ldr` at 0x3d38).
  Four-byte store: target `sp -0x34`, value `[sp-32]` (`str` at 0x3d3c).
  Four-byte load: target `(unknown effective address)`, base register `r1` (`ldr` at 0x3d40).
  Four-byte load: target `ELF load bias (unknown) +0x3dcc`, linked address 0x3dcc, basis `ELF load-bias-relative` (`ldr` at 0x3d48).
  ELF literal warning: linked 0x3dcc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3d48.
  Four-byte load: target `sp -0x14` (`ldr` at 0x3d4c).
  Four-byte load: target `ELF load bias (unknown) +0x3dd0`, linked address 0x3dd0, basis `ELF load-bias-relative` (`ldr` at 0x3d50).
  Call (`mov` at 0x3d58): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`ret(linked ELF 0x37b4)`, r1=`unknown`, r2=`[sp-20]`, r3=`0xabcd`.
  Four-byte store: target `sp -0x24`, value `ret(unknown)` (`str` at 0x3d60).
  Branch (`beq` at 0x3d6c): taken to 0x3da8 when `ret(unknown) == 0x0`, falls through to 0x3d70 when `ret(unknown) != 0x0`.
  Call (`bl` at 0x3d74): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Four-byte load: target `ELF load bias (unknown) +0x3dd4`, linked address 0x3dd4, basis `ELF load-bias-relative` (`ldr` at 0x3d7c).
  ELF literal warning: linked 0x3dd4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3d7c.
  Four-byte store: target `sp -0x38`, value `ret(linked ELF 0x37cc)` (`str` at 0x3d84).
  Four-byte load: target `ELF load bias (unknown) +0x3dc4`, linked address 0x3dc4, basis `ELF load-bias-relative` (`ldr` at 0x3d8c).
  Call (`bl` at 0x3d98): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x3`, r1=`unknown`, r2=`0x7e`, r3=`0x0`.
  Jump (`b` at 0x3da4): target 0x3db0.
  Four-byte store: target `sp -0x30`, value `0x0` (`str` at 0x3dac).
  Four-byte load: target `sp -0x30` (`ldr` at 0x3db0).
  `ldm` at 0x3db8 has recorded r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x3db8): the modeled local edges stop before the trailing region 0x3dbc to 0x3dd8 (exclusive).
  The decoded instructions from 0x3dbc to 0x3dd4 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `sn1ScanRsp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x3dd8 to 0x4038 (exclusive), 608 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `fff6337d2662c92a8921d5d44142e131789cdd36aa63fec173bd6762a971b631`.
  The 8 unapplied ELF relocation records at linked places 0x4014, 0x4018, 0x401c, 0x4024, 0x4028, 0x4030, 0x4034, 0x4020 retain unknown values.
  Four-byte load: target `ELF load bias (unknown) +0x4014`, linked address 0x4014, basis `ELF load-bias-relative` (`ldr` at 0x3df8).
  ELF literal warning: linked 0x4014, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3df8.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x3dfc).
  Branch (`bne` at 0x3e04): taken to 0x3e30 when `? != 0x0`, falls through to 0x3e08 when `? == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4018`, linked address 0x4018, basis `ELF load-bias-relative` (`ldr` at 0x3e08).
  ELF literal warning: linked 0x4018, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3e08.
  Grouped accesses: 4 4-byte store records with target `sp -0x40`, reported value `unknown` in every record, including `str` at 0x3e0c, `str` at 0x3e7c, `str` at 0x3ebc.
  Four-byte load: target `ELF load bias (unknown) +0x401c`, linked address 0x401c, basis `ELF load-bias-relative` (`ldr` at 0x3e14).
  ELF literal warning: linked 0x401c, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x3e14, `ldr` at 0x3e88, `ldr` at 0x3ed0.
  Call (`bl` at 0x3e20): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x4`, r1=`unknown`, r2=`0x94`, r3=`0x0`.
  The analyzer invalidates unsupported effects (`mvn` at 0x3e24).
  Grouped accesses: 4 4-byte store records with target `sp -0x34`, reported value `unknown` in every record, including `str` at 0x3e28, `str` at 0x3eec, `str` at 0x3f38.
  Jump (`b` at 0x3e2c): target 0x4008.
  Four-byte load: target `ELF load bias (unknown) +0x4020`, linked address 0x4020, basis `ELF load-bias-relative` (`ldr` at 0x3e30).
  ELF literal warning: linked 0x4020, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3e30.
  Four-byte store: target `sp -0x40`, value `0x1` (`str` at 0x3e3c).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0x3e40).
  Four-byte load: target `ELF load bias (unknown) +0x4024`, linked address 0x4024, basis `ELF load-bias-relative` (`ldr` at 0x3e44).
  ELF literal warning: linked 0x4024, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3e44.
  Four-byte load: target `sp -0x24` (`ldr` at 0x3e4c).
  Call (`mov` at 0x3e54): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`sp-44`, r2=`0x0`, r3=`r3`.
  Four-byte store: target `sp -0x28`, value `ret(unknown)` (`str` at 0x3e5c).
  Grouped accesses: 3 4-byte load records with target `sp -0x28`, including `ldr` at 0x3e60, `ldr` at 0x3e6c, `ldr` at 0x3e98.
  Branch (`beq` at 0x3e68): taken to 0x3ea4 when `ret(unknown) == 0x0`, falls through to 0x3e6c when `ret(unknown) != 0x0`.
  Call (`bl` at 0x3e70): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Four-byte load: target `ELF load bias (unknown) +0x4028`, linked address 0x4028, basis `ELF load-bias-relative` (`ldr` at 0x3e78).
  ELF literal warning: linked 0x4028, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3e78.
  Four-byte store: target `sp -0x3c`, value `ret(linked ELF 0x37cc)` (`str` at 0x3e80).
  Four-byte load: target `ELF load bias (unknown) +0x401c`, linked address 0x401c, basis `ELF load-bias-relative` (`ldr` at 0x3e88).
  Call (`bl` at 0x3e94): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x3`, r1=`unknown`, r2=`0x9d`, r3=`0x0`.
  Four-byte store: target `sp -0x34`, value `[sp-40]` (`str` at 0x3e9c).
  Jump (`b` at 0x3ea0): target 0x4008.
  Grouped accesses: 14 4-byte load records with target `sp -0x2c`, including `ldr` at 0x3ea4, `ldr` at 0x3ec0, `ldr` at 0x3ee0.
  Grouped accesses: 2 2-byte load records with target `[sp-44] +0x22`, including `ldrh` at 0x3ea8, `ldrh` at 0x3ec4.
  Four-byte load: target `ELF load bias (unknown) +0x402c`, linked address 0x402c, basis `ELF load-bias-relative` (`ldr` at 0x3eac).
  Branch (`beq` at 0x3eb4): taken to 0x3ef4 when `[[sp-44]+34]:2 == 0xabcd`, falls through to 0x3eb8 when `[[sp-44]+34]:2 != 0xabcd`.
  Four-byte load: target `ELF load bias (unknown) +0x4030`, linked address 0x4030, basis `ELF load-bias-relative` (`ldr` at 0x3eb8).
  ELF literal warning: linked 0x4030, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3eb8.
  Four-byte store: target `sp -0x3c`, value `[[sp-44]+34]:2` (`str` at 0x3ec8).
  Four-byte load: target `ELF load bias (unknown) +0x401c`, linked address 0x401c, basis `ELF load-bias-relative` (`ldr` at 0x3ed0).
  Call (`bl` at 0x3edc): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x3`, r1=`unknown`, r2=`0xa6`, r3=`0x0`.
  Call (`bl` at 0x3ee4): (unknown callee), target 0x3478, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3478, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  The analyzer invalidates unsupported effects (`mvn` at 0x3ee8).
  Jump (`b` at 0x3ef0): target 0x4008.
  Grouped accesses: 2 1-byte load records with target `[sp-44] +0x20`, including `ldrb` at 0x3ef8, `ldrb` at 0x3f10.
  Branch (`beq` at 0x3f00): taken to 0x3f40 when `[[sp-44]+32]:1 == 0x1`, falls through to 0x3f04 when `[[sp-44]+32]:1 != 0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x4034`, linked address 0x4034, basis `ELF load-bias-relative` (`ldr` at 0x3f04).
  ELF literal warning: linked 0x4034, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3f04.
  Four-byte store: target `sp -0x3c`, value `[[sp-44]+32]:1` (`str` at 0x3f14).
  Four-byte load: target `ELF load bias (unknown) +0x401c`, linked address 0x401c, basis `ELF load-bias-relative` (`ldr` at 0x3f1c).
  Call (`bl` at 0x3f28): (unknown callee), target 0x3634, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3634, r0=`0x3`, r1=`unknown`, r2=`0xab`, r3=`0x0`.
  Call (`bl` at 0x3f30): (unknown callee), target 0x3478, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3478, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  The analyzer invalidates unsupported effects (`mvn` at 0x3f34).
  Jump (`b` at 0x3f3c): target 0x4008.
  Four-byte load: target `sp -0x20` (`ldr` at 0x3f40).
  Byte load: target `[sp-44] +0x14` (`ldrb` at 0x3f48).
  Byte load: target `[sp-44] +0x21` (`ldrb` at 0x3f50).
  Four-byte store: target `[sp-32] +0x0`, value `unknown` (`str` at 0x3f5c).
  Grouped accesses: 3 4-byte load records with target `sp -0x1c`, including `ldr` at 0x3f60, `ldr` at 0x3f74, `ldr` at 0x3fe4.
  Two-byte load: target `[sp-44] +0x26` (`ldrh` at 0x3f68).
  Four-byte store: target `[sp-28] +0x0`, value `[[sp-44]+38]:2` (`str` at 0x3f6c).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x3f70, `ldr` at 0x3fe0.
  Four-byte load: target `[sp-28] +0x0` (`ldr` at 0x3f78).
  Call (`bl` at 0x3f7c): (unknown callee), target 0x35ec, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35ec, r0=`[[sp-28]]`, r1=`[sp-32]`, r2=`[sp-28]`, r3=`[sp-28]`.
  Four-byte store: target `[sp-24] +0x0`, value `ret(linked ELF 0x35ec)` (`str` at 0x3f80).
  Four-byte load: target `[sp-24] +0x0` (`ldr` at 0x3f84).
  Branch (`bne` at 0x3f8c): taken to 0x3fa4 when `[[sp-24]] != 0x0`, falls through to 0x3f90 when `[[sp-24]] == 0x0`.
  Call (`bl` at 0x3f94): (unknown callee), target 0x3478, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3478, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]]`.
  The analyzer invalidates unsupported effects (`mvn` at 0x3f98).
  Jump (`b` at 0x3fa0): target 0x4008.
  Byte load: target `[sp-44] +0x24` (`ldrb` at 0x3fa8).
  Branch (`bne` at 0x3fb4): taken to 0x3fc8 when `? != 0x0`, falls through to 0x3fb8 when `? == 0x0`.
  Four-byte load: target `[sp-44] +0x28` (`ldr` at 0x3fbc).
  Four-byte store: target `sp -0x38`, value `[[sp-44]+40]` (`str` at 0x3fc0).
  Jump (`b` at 0x3fc4): target 0x3fd8.
  Call (`bl` at 0x3fd0): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, r0=`[sp-44]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x38`, value `ret(linked ELF 0x367c)` (`str` at 0x3fd4).
  Four-byte load: target `sp -0x38` (`ldr` at 0x3fd8).
  Four-byte store: target `sp -0x30`, value `[sp-56]` (`str` at 0x3fdc).
  Four-byte load: target `[sp-24] +0x0` (`ldr` at 0x3fe8).
  Four-byte load: target `sp -0x30` (`ldr` at 0x3fec).
  Four-byte load: target `[sp-28] +0x0` (`ldr` at 0x3ff0).
  Call (`bl` at 0x3ff4): (unknown callee), target 0x37c0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37c0, r0=`[[sp-24]]`, r1=`[sp-56]`, r2=`[[sp-28]]`, r3=`[sp-24]`.
  Call (`bl` at 0x3ffc): (unknown callee), target 0x3478, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3478, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x34`, value `0x0` (`str` at 0x4004).
  Four-byte load: target `sp -0x34` (`ldr` at 0x4008).
  At the return (`ldm` at 0x4010), r0 is `[sp-52]`.
  Unconditional return boundary (`ldm` at 0x4010): the modeled local edges stop before the trailing region 0x4014 to 0x4038 (exclusive).
  The decoded instructions from 0x4014 to 0x4034 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `PPPReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libsys.armlinux.so`.
  Inferred analysis span: 0x7b08 to 0x7b6c (exclusive), 100 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 143436 bytes, SHA256 `60385315240617716defcd16c609c6cf44914e6c56d778cce3c831d5ebff379b`.
  Read window: SHA256 `6cce4089ce85d46f3a14a2e8ee075c7d6afece322789c737865ab547902847f3`.
  The analyzer retains no ELF relocation records for this span.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, including `ldr` at 0x7b20, `ldr` at 0x7b48.
  Branch (`beq` at 0x7b2c): taken to 0x7b48 when `? == 0x0`, falls through to 0x7b30 when `? != 0x0`.
  Grouped accesses: 3 4-byte load records with target `sp -0x14`, including `ldr` at 0x7b30, `ldr` at 0x7b3c, `ldr` at 0x7b58.
  Four-byte store: target `r0 +0xc`, value `0x0` (`str` at 0x7b38).
  Four-byte store: target `[sp-20] +0x30`, value `0x0` (`str` at 0x7b44).
  Branch (`beq` at 0x7b54): taken to 0x7b64 when `? == 0x0`, falls through to 0x7b58 when `? != 0x0`.
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2` (`str` at 0x7b60).
  The exit (`ldm` at 0x7b68) reports r0 = `r0`.

- `PPPInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libsys.armlinux.so`.
  Inferred analysis span: 0x7a34 to 0x7b08 (exclusive), 212 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 143436 bytes, SHA256 `60385315240617716defcd16c609c6cf44914e6c56d778cce3c831d5ebff379b`.
  Read window: SHA256 `23febeba3aa7f788809b6115b2adbbcb0842a66b5bc99ad2231a9d339856f15b`.
  The analyzer retains no ELF relocation records for this span.
  Grouped accesses: 12 4-byte load records with target `sp -0x14`, including `ldr` at 0x7a54, `ldr` at 0x7a64, `ldr` at 0x7a70.
  Call (`bl` at 0x7a60): (unknown callee), target 0x6908, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x6908, r0=`r0`, r1=`0x0`, r2=`0x3c`, r3=`r3`.
  Four-byte load: target `sp -0x18` (`ldr` at 0x7a68).
  Four-byte store: target `[sp-20] +0x0`, value `[sp-24]` (`str` at 0x7a6c).
  Four-byte load: target `sp -0x1c` (`ldr` at 0x7a74).
  Four-byte store: target `[sp-20] +0x4`, value `[sp-28]` (`str` at 0x7a78).
  Four-byte load: target `sp -0x20` (`ldr` at 0x7a80).
  Four-byte store: target `[sp-20] +0x10`, value `[sp-32]` (`str` at 0x7a84).
  Four-byte load: target `sp +0x0` (`ldr` at 0x7a8c).
  Four-byte store: target `[sp-20] +0x14`, value `[sp]` (`str` at 0x7a90).
  Four-byte load: target `sp +0x4` (`ldr` at 0x7a98).
  Predicated effects are merged with the unexecuted path (`moveq` at 0x7aa0).
  Predicated effects are merged with the unexecuted path (`movne` at 0x7aa4).
  Four-byte store: target `[sp-20] +0x20`, value `unknown` (`str` at 0x7aa8).
  Four-byte load: target `sp +0x8` (`ldr` at 0x7ab0).
  Predicated effects are merged with the unexecuted path (`moveq` at 0x7ab8).
  Predicated effects are merged with the unexecuted path (`movne` at 0x7abc).
  Four-byte store: target `[sp-20] +0x24`, value `unknown` (`str` at 0x7ac0).
  Four-byte load: target `sp +0xc` (`ldr` at 0x7ac8).
  Four-byte store: target `[sp-20] +0x28`, value `[sp+12]` (`str` at 0x7acc).
  Four-byte load: target `sp +0x10` (`ldr` at 0x7ad4).
  Four-byte store: target `[sp-20] +0x2c`, value `[sp+16]` (`str` at 0x7ad8).
  Four-byte load: target `sp +0x14` (`ldr` at 0x7ae0).
  Four-byte store: target `[sp-20] +0x34`, value `[sp+20]` (`str` at 0x7ae4).
  Four-byte load: target `sp +0x18` (`ldr` at 0x7aec).
  Four-byte store: target `[sp-20] +0x38`, value `[sp+24]` (`str` at 0x7af0).
  Call (`bl` at 0x7afc): (unknown callee), target 0x683c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x683c, r0=`[sp-20]`, r1=`0x3`, r2=`[sp-20]`, r3=`[sp+24]`.
  Recorded r0 at the return is `ret(linked ELF 0x683c)` (`ldm` at 0x7b04).
