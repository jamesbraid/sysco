# ARM library state and indirect-call paths

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

The [complete facts](disasm/arm-1.44.0-libraries/state-indirect-call-paths-facts.txt) retain every access, relocation, constant and warning, including omitted frame saves, restores and input spills.


- `nvramReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x10c80 to 0x10cc0 (exclusive), 64 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `42e57bce824d0ebce73e98a453e967dd244dc41c6012f0dc8e6182e004d7bdfa`.
  Call (`bl` at 0x10c8c): (unknown callee), target 0x10d5c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x10d5c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Four-byte load: target `ELF load bias (unknown) +0x10cb8`, linked address 0x10cb8, basis `ELF load-bias-relative` (`ldr` at 0x10c90).
  ELF literal warning: linked 0x10cb8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10c90.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x10c94).
  Branch (`bne` at 0x10c9c): taken to 0x10cac when `? != 0x3`, falls through to 0x10ca0 when `? == 0x3`.
  Four-byte load: target `ELF load bias (unknown) +0x10cbc`, linked address 0x10cbc, basis `ELF load-bias-relative` (`ldr` at 0x10ca0).
  ELF literal warning: linked 0x10cbc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10ca0.
  Call (`bl` at 0x10ca8): (unknown callee), target 0x12124, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x12124, r0=`unknown`, r1=`0x4000`, r2=`unknown`, r3=`unknown`.
  At the return (`ldm` at 0x10cb4), r0 is `0x0`.
  The instructions from 0x10cb8 to 0x10cbc are not reached by any path from the entry, and their branches are not listed.

- `nvramLogReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x11574 to 0x11604 (exclusive), 144 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `90768c953b2efa46e738d141ab997616acee4b80d4274711cc56fc864d8c13b7`.
  Four-byte load: target `ELF load bias (unknown) +0x115f0`, linked address 0x115f0, basis `ELF load-bias-relative` (`ldr` at 0x11580).
  ELF literal warning: linked 0x115f0, value unknown (relocation, writable or unmapped bytes), 6 read records, including `ldr` at 0x11580, `ldr` at 0x11588, `ldr` at 0x11598.
  Grouped accesses: 7 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x11584, `ldr` at 0x1158c, `ldr` at 0x1159c.
  Four-byte load: target `ELF load bias (unknown) +0x115f0`, linked address 0x115f0, basis `ELF load-bias-relative` (`ldr` at 0x11588).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0x11590, `ldr` at 0x115a8.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2` (`str` at 0x11594).
  Four-byte load: target `ELF load bias (unknown) +0x115f0`, linked address 0x115f0, basis `ELF load-bias-relative` (`ldr` at 0x11598).
  Four-byte load: target `ELF load bias (unknown) +0x115f0`, linked address 0x115f0, basis `ELF load-bias-relative` (`ldr` at 0x115a0).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2` (`str` at 0x115ac).
  Four-byte load: target `ELF load bias (unknown) +0x115f0`, linked address 0x115f0, basis `ELF load-bias-relative` (`ldr` at 0x115b0).
  Four-byte load: target `ELF load bias (unknown) +0x115f4`, linked address 0x115f4, basis `ELF load-bias-relative` (`ldr` at 0x115b8).
  Four-byte load: target `ELF load bias (unknown) +0x115f0`, linked address 0x115f0, basis `ELF load-bias-relative` (`ldr` at 0x115bc).
  Four-byte load: target `ELF load bias (unknown) +0x115f8`, linked address 0x115f8, basis `ELF load-bias-relative` (`ldr` at 0x115c4).
  Call (`bl` at 0x115c8): (unknown callee), target 0x33c4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33c4, r0=`unknown`, r1=`0x82c`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x33c4) & 0xff)`, base register `r5` (`strb` at 0x115cc).
  Four-byte load: target `ELF load bias (unknown) +0x115fc`, linked address 0x115fc, basis `ELF load-bias-relative` (`ldr` at 0x115d0).
  ELF literal warning: linked 0x115fc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x115d0.
  Branch (`bne` at 0x115dc): taken to 0x115ec when `? != 0x3`, falls through to 0x115e0 when `? == 0x3`.
  Four-byte load: target `ELF load bias (unknown) +0x11600`, linked address 0x11600, basis `ELF load-bias-relative` (`ldr` at 0x115e0).
  ELF literal warning: linked 0x11600, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x115e0.
  Call (`bl` at 0x115e8): (unknown callee), target 0x12124, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x12124, r0=`unknown`, r1=`0x4000`, r2=`unknown`, r3=`unknown`.
  The exit (`ldm` at 0x115ec) reports r0 = `unknown`.
  The instructions from 0x115f0 to 0x11600 are not reached by any path from the entry, and their branches are not listed.

- `L2CfgMEventRack`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x60b8 to 0x6118 (exclusive), 96 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `c010df83d4d50424ca20f085c79dadabaa60eccaadf2cd69c97b1aa9225ed5b2`.
  Call (`bl` at 0x60d0): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`0x1`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x60dc): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`0x16`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `[sp-20] +0x118`, value `unknown` (`str` at 0x610c).
  Recorded r0 at the return is `ret(linked ELF 0x37cc)` (`ldm` at 0x6114).

- `l1d_tty_debug`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xb4a4 to 0xb5ec (exclusive), 328 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `555b660643feb511495b683dad480b6ee9d386907011b2849f7f4fbcd14cba0f`.
  Call (`bl` at 0xb4c4): (unknown callee), target 0x3958, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3958, r0=`r0`, r1=`sp-84`, r2=`r2`, r3=`sp-84`.
  Effects remain unknown for `cmn` at 0xb4cc. unsupported effects invalidated.
  Branch (`bne` at 0xb4d0): taken to 0xb4f8 when `Z == 0`, falls through to 0xb4d4 when `not (Z == 0)`.
  Call (`bl` at 0xb4d4): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x3958)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3958)`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb4e0).
  ELF literal warning: linked 0xb5c8, value unknown (relocation, writable or unmapped bytes), 8 read records, including `ldr` at 0xb4e0, `ldr` at 0xb4f8, `ldr` at 0xb50c.
  Grouped accesses: 7 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xb4e4, `ldr` at 0xb4fc, `ldr` at 0xb510.
  Four-byte load: target `ELF load bias (unknown) +0xb5cc`, linked address 0xb5cc, basis `ELF load-bias-relative` (`ldr` at 0xb4e8).
  ELF literal warning: linked 0xb5cc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb4e8.
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xb4ec).
  Call (`bl` at 0xb4f0): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[ret(linked ELF 0x3580)]`, r3=`unknown`.
  Jump (`b` at 0xb4f4): target 0xb5c0.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb4f8).
  Four-byte load: target `ELF load bias (unknown) +0xb5d0`, linked address 0xb5d0, basis `ELF load-bias-relative` (`ldr` at 0xb500).
  ELF literal warning: linked 0xb5d0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb500.
  Call (`bl` at 0xb508): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-84]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb50c).
  Four-byte load: target `ELF load bias (unknown) +0xb5d4`, linked address 0xb5d4, basis `ELF load-bias-relative` (`ldr` at 0xb514).
  ELF literal warning: linked 0xb5d4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb514.
  Call (`bl` at 0xb51c): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-80]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb520).
  Four-byte load: target `ELF load bias (unknown) +0xb5d8`, linked address 0xb5d8, basis `ELF load-bias-relative` (`ldr` at 0xb528).
  ELF literal warning: linked 0xb5d8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb528.
  Call (`bl` at 0xb530): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-76]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb534).
  Four-byte load: target `ELF load bias (unknown) +0xb5dc`, linked address 0xb5dc, basis `ELF load-bias-relative` (`ldr` at 0xb53c).
  ELF literal warning: linked 0xb5dc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb53c.
  Call (`bl` at 0xb544): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-72]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb548).
  Four-byte load: target `ELF load bias (unknown) +0xb5e0`, linked address 0xb5e0, basis `ELF load-bias-relative` (`ldr` at 0xb550).
  ELF literal warning: linked 0xb5e0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb550.
  Call (`bl` at 0xb558): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb55c).
  Four-byte load: target `ELF load bias (unknown) +0xb5e4`, linked address 0xb5e4, basis `ELF load-bias-relative` (`ldr` at 0xb564).
  ELF literal warning: linked 0xb5e4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb564.
  Call (`bl` at 0xb56c): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Four-byte store: target `sp -0x18`, value `0x0` (`str` at 0xb574).
  Grouped accesses: 4 4-byte load records with target `sp -0x18`, including `ldr` at 0xb578, `ldr` at 0xb58c, `ldr` at 0xb5a8.
  Branch (`bgt` at 0xb580): taken to 0xb5c0 when `gt flags condition (operands unknown)`, falls through to 0xb584 when `not (gt flags condition (operands unknown))`.
  Four-byte load: target `ELF load bias (unknown) +0xb5c8`, linked address 0xb5c8, basis `ELF load-bias-relative` (`ldr` at 0xb584).
  Effects remain unknown for `mvn` at 0xb588. unsupported effects invalidated.
  Byte load: target `(unknown effective address)`, base register `r3` (`ldrb` at 0xb59c).
  Four-byte load: target `(unknown effective address)`, base register `r1` (`ldr` at 0xb5a0).
  Four-byte load: target `ELF load bias (unknown) +0xb5e8`, linked address 0xb5e8, basis `ELF load-bias-relative` (`ldr` at 0xb5a4).
  ELF literal warning: linked 0xb5e8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb5a4.
  Call (`bl` at 0xb5ac): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-24]`, r3=`unknown`.
  Four-byte store: target `sp -0x18`, value `[sp-24]+1` (`str` at 0xb5b8).
  Jump (`b` at 0xb5bc): target 0xb578.
  `ldm` at 0xb5c4 has recorded r0 = `ret(linked ELF 0x34cc)`.
  The instructions from 0xb5c8 to 0xb5e8 are not reached by any path from the entry, and their branches are not listed.

- `l1d_tty_open`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xb5ec to 0xb744 (exclusive), 344 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `918607ff7792da6c405d5a862698b8e32b26d3341250e3cf6fee3d1fe98c7e38`.
  Effects remain unknown for `mvn` at 0xb600. unsupported effects invalidated.
  Call (`bl` at 0xb610): (unknown callee), target 0x3808, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3808, r0=`[sp-20]`, r1=`0x2`, r2=`r2`, r3=`unknown`.
  Effects remain unknown for `cmn` at 0xb620. unsupported effects invalidated.
  Branch (`bne` at 0xb624): taken to 0xb658 when `Z == 0`, falls through to 0xb628 when `not (Z == 0)`.
  Call (`bl` at 0xb628): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x3808)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3808)`.
  Four-byte load: target `ELF load bias (unknown) +0xb730`, linked address 0xb730, basis `ELF load-bias-relative` (`ldr` at 0xb634).
  ELF literal warning: linked 0xb730, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xb634, `ldr` at 0xb680, `ldr` at 0xb6fc.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xb638, `ldr` at 0xb684, `ldr` at 0xb700.
  Four-byte load: target `ELF load bias (unknown) +0xb734`, linked address 0xb734, basis `ELF load-bias-relative` (`ldr` at 0xb63c).
  ELF literal warning: linked 0xb734, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb63c.
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xb644).
  Call (`bl` at 0xb648): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[sp-20]`, r3=`[ret(linked ELF 0x3580)]`.
  Effects remain unknown for `mvn` at 0xb64c. unsupported effects invalidated.
  Grouped accesses: 3 4-byte store records with target `sp -0x58`, reported value `unknown`, including `str` at 0xb650, `str` at 0xb698, `str` at 0xb714.
  Jump (`b` at 0xb654): target 0xb724.
  Call (`bl` at 0xb664): (unknown callee), target 0x3958, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3958, r0=`[sp-24]`, r1=`sp-84`, r2=`unknown`, r3=`sp-84`.
  Effects remain unknown for `cmn` at 0xb66c. unsupported effects invalidated.
  Branch (`bne` at 0xb670): taken to 0xb6a0 when `Z == 0`, falls through to 0xb674 when `not (Z == 0)`.
  Call (`bl` at 0xb674): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x3958)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3958)`.
  Four-byte load: target `ELF load bias (unknown) +0xb730`, linked address 0xb730, basis `ELF load-bias-relative` (`ldr` at 0xb680).
  Four-byte load: target `ELF load bias (unknown) +0xb738`, linked address 0xb738, basis `ELF load-bias-relative` (`ldr` at 0xb688).
  ELF literal warning: linked 0xb738, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb688.
  Grouped accesses: 2 4-byte load records with target `ret(linked ELF 0x3580) +0x0`, including `ldr` at 0xb68c, `ldr` at 0xb708.
  Call (`bl` at 0xb690): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[ret(linked ELF 0x3580)]`, r3=`unknown`.
  Effects remain unknown for `mvn` at 0xb694. unsupported effects invalidated.
  Jump (`b` at 0xb69c): target 0xb724.
  Four-byte store: target `sp -0x54`, value `0x5` (`str` at 0xb6a4).
  Four-byte store: target `sp -0x50`, value `0x0` (`str` at 0xb6ac).
  Four-byte load: target `ELF load bias (unknown) +0xb73c`, linked address 0xb73c, basis `ELF load-bias-relative` (`ldr` at 0xb6b0).
  Four-byte store: target `sp -0x4c`, value `0x800008bf` (`str` at 0xb6b4).
  Four-byte store: target `sp -0x48`, value `0x0` (`str` at 0xb6bc).
  Byte store: target `sp -0x3d`, value `0x1` (`strb` at 0xb6c4).
  Byte store: target `sp -0x3e`, value `0x0` (`strb` at 0xb6cc).
  Call (`bl` at 0xb6e0): (unknown callee), target 0x3568, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3568, r0=`[sp-24]`, r1=`0x0`, r2=`sp-84`, r3=`sp-84`.
  Effects remain unknown for `cmn` at 0xb6e8. unsupported effects invalidated.
  Branch (`bne` at 0xb6ec): taken to 0xb71c when `Z == 0`, falls through to 0xb6f0 when `not (Z == 0)`.
  Call (`bl` at 0xb6f0): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x3568)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3568)`.
  Four-byte load: target `ELF load bias (unknown) +0xb730`, linked address 0xb730, basis `ELF load-bias-relative` (`ldr` at 0xb6fc).
  Four-byte load: target `ELF load bias (unknown) +0xb740`, linked address 0xb740, basis `ELF load-bias-relative` (`ldr` at 0xb704).
  ELF literal warning: linked 0xb740, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb704.
  Call (`bl` at 0xb70c): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`[ret(linked ELF 0x3580)]`, r3=`unknown`.
  Effects remain unknown for `mvn` at 0xb710. unsupported effects invalidated.
  Jump (`b` at 0xb718): target 0xb724.
  Four-byte store: target `sp -0x58`, value `[sp-24]` (`str` at 0xb720).
  Four-byte load: target `sp -0x58` (`ldr` at 0xb724).
  At the return (`ldm` at 0xb72c), r0 is `[sp-88]`.
  The instructions from 0xb730 to 0xb740 are not reached by any path from the entry, and their branches are not listed.

- `l1d_close_serial_direct`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xad48 to 0xadc0 (exclusive), 120 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `be28eac55dddaec5de95208637f7141351b02d93b3ec73cb9526b35100843987`.
  Four-byte load: target `ELF load bias (unknown) +0xadb0`, linked address 0xadb0, basis `ELF load-bias-relative` (`ldr` at 0xad58).
  ELF literal warning: linked 0xadb0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xad58.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xad5c, `ldr` at 0xad80.
  Call (`bl` at 0xad60): (unknown callee), target 0x38bc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x38bc, r0=`unknown`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Branch (`beq` at 0xad6c): taken to 0xada0 when `ret(linked ELF 0x38bc) == 0x0`, falls through to 0xad70 when `ret(linked ELF 0x38bc) != 0x0`.
  Call (`bl` at 0xad70): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x38bc)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x38bc)`.
  Four-byte load: target `ELF load bias (unknown) +0xadb4`, linked address 0xadb4, basis `ELF load-bias-relative` (`ldr` at 0xad7c).
  ELF literal warning: linked 0xadb4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xad7c.
  Four-byte load: target `ELF load bias (unknown) +0xadb8`, linked address 0xadb8, basis `ELF load-bias-relative` (`ldr` at 0xad84).
  ELF literal warning: linked 0xadb8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xad84.
  Four-byte load: target `ELF load bias (unknown) +0xadbc`, linked address 0xadbc, basis `ELF load-bias-relative` (`ldr` at 0xad88).
  ELF literal warning: linked 0xadbc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xad88.
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xad8c).
  Call (`bl` at 0xad90): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[ret(linked ELF 0x3580)]`.
  Effects remain unknown for `mvn` at 0xad94. unsupported effects invalidated.
  Four-byte store: target `sp -0x14`, value `unknown` (`str` at 0xad98).
  Jump (`b` at 0xad9c): target 0xada8.
  Four-byte store: target `sp -0x14`, value `0x0` (`str` at 0xada4).
  Four-byte load: target `sp -0x14` (`ldr` at 0xada8).
  The exit (`ldm` at 0xadac) reports r0 = `unknown`.
  The instructions from 0xadb0 to 0xadbc are not reached by any path from the entry, and their branches are not listed.

- `l1d_close_serial_enet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xaea8 to 0xaf30 (exclusive), 136 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `d4bbe4a85f740dbc3fc94551697f90aca8aa9fec0255aa1fb73d174cd7d255c0`.
  Four-byte load: target `ELF load bias (unknown) +0xaf1c`, linked address 0xaf1c, basis `ELF load-bias-relative` (`ldr` at 0xaeb8).
  ELF literal warning: linked 0xaf1c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaeb8.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0xaebc).
  Call (`bl` at 0xaec0): (unknown callee), target 0x343c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x343c, r0=`unknown`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Branch (`beq` at 0xaecc): taken to 0xaf08 when `ret(linked ELF 0x343c) == 0x0`, falls through to 0xaed0 when `ret(linked ELF 0x343c) != 0x0`.
  Call (`bl` at 0xaed0): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x343c)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x343c)`.
  Four-byte load: target `ELF load bias (unknown) +0xaf20`, linked address 0xaf20, basis `ELF load-bias-relative` (`ldr` at 0xaed8).
  ELF literal warning: linked 0xaf20, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaed8.
  Four-byte load: target `ELF load bias (unknown) +0xaf24`, linked address 0xaf24, basis `ELF load-bias-relative` (`ldr` at 0xaedc).
  ELF literal warning: linked 0xaf24, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaedc.
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xaee0).
  Four-byte store: target `sp -0x18`, value `[ret(linked ELF 0x3580)]` (`str` at 0xaee4).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0xaee8).
  Four-byte load: target `ELF load bias (unknown) +0xaf28`, linked address 0xaf28, basis `ELF load-bias-relative` (`ldr` at 0xaeec).
  ELF literal warning: linked 0xaf28, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaeec.
  Four-byte load: target `ELF load bias (unknown) +0xaf2c`, linked address 0xaf2c, basis `ELF load-bias-relative` (`ldr` at 0xaef0).
  ELF literal warning: linked 0xaf2c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaef0.
  Four-byte load: target `(unknown effective address)`, base register `r12` (`ldr` at 0xaef4).
  Call (`bl` at 0xaef8): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Effects remain unknown for `mvn` at 0xaefc. unsupported effects invalidated.
  Four-byte store: target `sp -0x14`, value `unknown` (`str` at 0xaf00).
  Jump (`b` at 0xaf04): target 0xaf10.
  Four-byte store: target `sp -0x14`, value `0x0` (`str` at 0xaf0c).
  Four-byte load: target `sp -0x14` (`ldr` at 0xaf10).
  Recorded r0 at the return is `unknown` (`ldm` at 0xaf18).
  The instructions from 0xaf1c to 0xaf2c are not reached by any path from the entry, and their branches are not listed.

- `l1d_open_serial_direct`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xac7c to 0xad48 (exclusive), 204 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `d36d12e8eded846fcd81478930918ec7b6bddffec5ba0b8dc004232d2216d572`.
  Four-byte load: target `ELF load bias (unknown) +0xad2c`, linked address 0xad2c, basis `ELF load-bias-relative` (`ldr` at 0xac8c).
  ELF literal warning: linked 0xad2c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xac8c, `ldr` at 0xacfc.
  Grouped accesses: 6 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xac90, `ldr` at 0xaca0, `ldr` at 0xacc0.
  Branch (`beq` at 0xac98): taken to 0xacac when `? == 0x0`, falls through to 0xac9c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xad30`, linked address 0xad30, basis `ELF load-bias-relative` (`ldr` at 0xac9c).
  ELF literal warning: linked 0xad30, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xac9c, `ldr` at 0xacd8, `ldr` at 0xad0c.
  Four-byte load: target `ELF load bias (unknown) +0xad34`, linked address 0xad34, basis `ELF load-bias-relative` (`ldr` at 0xaca4).
  ELF literal warning: linked 0xad34, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xaca4.
  Call (`bl` at 0xaca8): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r2`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xad38`, linked address 0xad38, basis `ELF load-bias-relative` (`ldr` at 0xacac).
  ELF literal warning: linked 0xad38, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xacac, `ldr` at 0xace4.
  Call (`bl` at 0xacb0): (unknown callee), target 0x36c4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36c4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xad3c`, linked address 0xad3c, basis `ELF load-bias-relative` (`ldr` at 0xacb8).
  ELF literal warning: linked 0xad3c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xacb8.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x36c4)`, base register `r3` (`str` at 0xacbc).
  Branch (`bge` at 0xacc8): taken to 0xacfc when `? >= 0x0 (signed, 32-bit)`, falls through to 0xaccc when `? < 0x0 (signed, 32-bit)`.
  Call (`bl` at 0xaccc): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x36c4)`, r1=`unknown`, r2=`ret(linked ELF 0x36c4)`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xad30`, linked address 0xad30, basis `ELF load-bias-relative` (`ldr` at 0xacd8).
  Four-byte load: target `ELF load bias (unknown) +0xad40`, linked address 0xad40, basis `ELF load-bias-relative` (`ldr` at 0xace0).
  ELF literal warning: linked 0xad40, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xace0.
  Four-byte load: target `ELF load bias (unknown) +0xad38`, linked address 0xad38, basis `ELF load-bias-relative` (`ldr` at 0xace4).
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xace8).
  Call (`bl` at 0xacec): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[ret(linked ELF 0x3580)]`.
  Effects remain unknown for `mvn` at 0xacf0. unsupported effects invalidated.
  Four-byte store: target `sp -0x14`, value `unknown` (`str` at 0xacf4).
  Jump (`b` at 0xacf8): target 0xad24.
  Four-byte load: target `ELF load bias (unknown) +0xad2c`, linked address 0xad2c, basis `ELF load-bias-relative` (`ldr` at 0xacfc).
  Branch (`beq` at 0xad08): taken to 0xad1c when `? == 0x0`, falls through to 0xad0c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xad30`, linked address 0xad30, basis `ELF load-bias-relative` (`ldr` at 0xad0c).
  Four-byte load: target `ELF load bias (unknown) +0xad44`, linked address 0xad44, basis `ELF load-bias-relative` (`ldr` at 0xad14).
  ELF literal warning: linked 0xad44, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xad14.
  Call (`bl` at 0xad18): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0x36c4)`, r3=`unknown`.
  Four-byte store: target `sp -0x14`, value `0x0` (`str` at 0xad20).
  Four-byte load: target `sp -0x14` (`ldr` at 0xad24).
  `ldm` at 0xad28 has recorded r0 = `unknown`.
  The instructions from 0xad2c to 0xad44 are not reached by any path from the entry, and their branches are not listed.

- `l1d_open_serial_enet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xadc0 to 0xaea8 (exclusive), 232 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `75270f9fd84468055e1b71077ff8f0cafb49f39ae3856ca0e529fe3bfeb94bbd`.
  Four-byte load: target `ELF load bias (unknown) +0xae88`, linked address 0xae88, basis `ELF load-bias-relative` (`ldr` at 0xadd0).
  ELF literal warning: linked 0xae88, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xadd0, `ldr` at 0xae54.
  Grouped accesses: 6 4-byte load records with target `(unknown effective address)`, base register `r3`, including `ldr` at 0xadd4, `ldr` at 0xade4, `ldr` at 0xadf8.
  Branch (`beq` at 0xaddc): taken to 0xadf0 when `? == 0x0`, falls through to 0xade0 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xae8c`, linked address 0xae8c, basis `ELF load-bias-relative` (`ldr` at 0xade0).
  ELF literal warning: linked 0xae8c, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xade0, `ldr` at 0xae24, `ldr` at 0xae64.
  Four-byte load: target `ELF load bias (unknown) +0xae90`, linked address 0xae90, basis `ELF load-bias-relative` (`ldr` at 0xade8).
  ELF literal warning: linked 0xae90, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xade8.
  Call (`bl` at 0xadec): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r2`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xae94`, linked address 0xae94, basis `ELF load-bias-relative` (`ldr` at 0xadf0).
  ELF literal warning: linked 0xae94, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xadf0, `ldr` at 0xae28.
  Four-byte load: target `ELF load bias (unknown) +0xae98`, linked address 0xae98, basis `ELF load-bias-relative` (`ldr` at 0xadf4).
  ELF literal warning: linked 0xae98, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xadf4, `ldr` at 0xae3c.
  Call (`bl` at 0xae00): (unknown callee), target 0x33e8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33e8, r0=`unknown`, r1=`unknown`, r2=`0x0`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xae9c`, linked address 0xae9c, basis `ELF load-bias-relative` (`ldr` at 0xae08).
  ELF literal warning: linked 0xae9c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xae08.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x33e8)`, base register `r3` (`str` at 0xae0c).
  Branch (`bge` at 0xae18): taken to 0xae54 when `? >= 0x0 (signed, 32-bit)`, falls through to 0xae1c when `? < 0x0 (signed, 32-bit)`.
  Call (`bl` at 0xae1c): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, r0=`ret(linked ELF 0x33e8)`, r1=`unknown`, r2=`ret(linked ELF 0x33e8)`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xae8c`, linked address 0xae8c, basis `ELF load-bias-relative` (`ldr` at 0xae24).
  Four-byte load: target `ELF load bias (unknown) +0xae94`, linked address 0xae94, basis `ELF load-bias-relative` (`ldr` at 0xae28).
  Four-byte load: target `ret(linked ELF 0x3580) +0x0` (`ldr` at 0xae2c).
  Four-byte store: target `sp -0x18`, value `[ret(linked ELF 0x3580)]` (`str` at 0xae30).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0xae34).
  Four-byte load: target `ELF load bias (unknown) +0xaea0`, linked address 0xaea0, basis `ELF load-bias-relative` (`ldr` at 0xae38).
  ELF literal warning: linked 0xaea0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xae38.
  Four-byte load: target `ELF load bias (unknown) +0xae98`, linked address 0xae98, basis `ELF load-bias-relative` (`ldr` at 0xae3c).
  Four-byte load: target `(unknown effective address)`, base register `r12` (`ldr` at 0xae40).
  Call (`bl` at 0xae44): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Effects remain unknown for `mvn` at 0xae48. unsupported effects invalidated.
  Four-byte store: target `sp -0x14`, value `unknown` (`str` at 0xae4c).
  Jump (`b` at 0xae50): target 0xae7c.
  Four-byte load: target `ELF load bias (unknown) +0xae88`, linked address 0xae88, basis `ELF load-bias-relative` (`ldr` at 0xae54).
  Branch (`beq` at 0xae60): taken to 0xae74 when `? == 0x0`, falls through to 0xae64 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xae8c`, linked address 0xae8c, basis `ELF load-bias-relative` (`ldr` at 0xae64).
  Four-byte load: target `ELF load bias (unknown) +0xaea4`, linked address 0xaea4, basis `ELF load-bias-relative` (`ldr` at 0xae6c).
  ELF literal warning: linked 0xaea4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xae6c.
  Call (`bl` at 0xae70): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0x33e8)`, r3=`unknown`.
  Four-byte store: target `sp -0x14`, value `0x0` (`str` at 0xae78).
  Four-byte load: target `sp -0x14` (`ldr` at 0xae7c).
  At the return (`ldm` at 0xae84), r0 is `unknown`.
  The instructions from 0xae88 to 0xaea4 are not reached by any path from the entry, and their branches are not listed.

- `jmstr_run_tck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x3d14 to 0x3dbc (exclusive), 168 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `607ce2d114369cba72eb2cad50bb685ab92d1bd2061632056432912cdf73c21d`.
  Four-byte store: target `sp -0x20`, value `0x10` (`str` at 0x3d34).
  Branch (`beq` at 0x3d40): taken to 0x3d50 when `r0 == 0x0`, falls through to 0x3d44 when `r0 != 0x0`.
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, including `ldr` at 0x3d44, `ldr` at 0x3d64.
  Four-byte store: target `sp -0x20`, value `0x18` (`str` at 0x3d4c).
  Branch (`beq` at 0x3d58): taken to 0x3da0 when `r1 == 0x0`, falls through to 0x3d5c when `r1 != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x3db4`, linked address 0x3db4, basis `ELF load-bias-relative` (`ldr` at 0x3d5c).
  ELF literal warning: linked 0x3db4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3d5c.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x3d60).
  Call (`mov` at 0x3d78): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`r1`, r2=`0x0`, r3=`0x0`.
  Branch (`bne` at 0x3d84): taken to 0x3da0 when `ret(unknown) != 0x0`, falls through to 0x3d88 when `ret(unknown) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x3db8`, linked address 0x3db8, basis `ELF load-bias-relative` (`ldr` at 0x3d8c).
  ELF literal warning: linked 0x3db8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3d8c.
  Call (`bl` at 0x3d90): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Four-byte store: target `sp -0x24`, value `0x0` (`str` at 0x3d98).
  Jump (`b` at 0x3d9c): target 0x3da8.
  Four-byte store: target `sp -0x24`, value `0x1` (`str` at 0x3da4).
  Four-byte load: target `sp -0x24` (`ldr` at 0x3da8).
  The exit (`ldm` at 0x3db0) reports r0 = `unknown`.
  The instructions from 0x3db4 to 0x3db8 are not reached by any path from the entry, and their branches are not listed.

- `jmstr_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x38a8 to 0x3968 (exclusive), 192 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `0b6797b1a5bcf47dd8831c1f0123b1e5890351dc8b86cf9f812f6b8747458fa3`.
  Four-byte store: target `sp -0x28`, value `0x0` (`str` at 0x38c8).
  Four-byte store: target `sp -0x24`, value `0x20` (`str` at 0x38d0).
  Branch (`beq` at 0x38dc): taken to 0x38ec when `r1 == 0x0`, falls through to 0x38e0 when `r1 != 0x0`.
  Grouped accesses: 3 4-byte load records with target `sp -0x24`, including `ldr` at 0x38e0, `ldr` at 0x38f8, `ldr` at 0x3914.
  Four-byte store: target `sp -0x24`, value `0x28` (`str` at 0x38e8).
  Branch (`bne` at 0x38f4): taken to 0x3904 when `r0 != 0x1`, falls through to 0x38f8 when `r0 == 0x1`.
  Four-byte store: target `sp -0x24`, value `unknown` (`str` at 0x3900).
  Four-byte load: target `ELF load bias (unknown) +0x3960`, linked address 0x3960, basis `ELF load-bias-relative` (`ldr` at 0x3904).
  ELF literal warning: linked 0x3960, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3904.
  Four-byte load: target `(unknown effective address)`, base register `r3` (`ldr` at 0x3910).
  Call (`mov` at 0x3924): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`0x20`, r2=`sp-40`, r3=`sp-40`.
  Branch (`bne` at 0x3930): taken to 0x394c when `ret(unknown) != 0x0`, falls through to 0x3934 when `ret(unknown) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x3964`, linked address 0x3964, basis `ELF load-bias-relative` (`ldr` at 0x3938).
  ELF literal warning: linked 0x3964, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3938.
  Call (`bl` at 0x393c): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Four-byte store: target `sp -0x2c`, value `0x0` (`str` at 0x3944).
  Jump (`b` at 0x3948): target 0x3954.
  Four-byte store: target `sp -0x2c`, value `0x1` (`str` at 0x3950).
  Four-byte load: target `sp -0x2c` (`ldr` at 0x3954).
  Recorded r0 at the return is `unknown` (`ldm` at 0x395c).
  The instructions from 0x3960 to 0x3964 are not reached by any path from the entry, and their branches are not listed.

- `jmstr_custom_seq`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x3dbc to 0x3e64 (exclusive), 168 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `891e4b1737ed91a99476e6f87d732e70a607e0870eaa93f4b0cb5a482466ba2b`.
  Four-byte store: target `sp -0x24`, value `0x10` (`str` at 0x3de0).
  Branch (`beq` at 0x3dec): taken to 0x3dfc when `r0 == 0x0`, falls through to 0x3df0 when `r0 != 0x0`.
  Grouped accesses: 2 4-byte load records with target `sp -0x24`, including `ldr` at 0x3df0, `ldr` at 0x3e0c.
  Four-byte store: target `sp -0x24`, value `0x18` (`str` at 0x3df8).
  Four-byte load: target `ELF load bias (unknown) +0x3e5c`, linked address 0x3e5c, basis `ELF load-bias-relative` (`ldr` at 0x3dfc).
  ELF literal warning: linked 0x3e5c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3dfc.
  Four-byte load: target `sp +0x0` (`ldr` at 0x3e00).
  Four-byte store: target `sp -0x2c`, value `[sp]` (`str` at 0x3e04).
  Four-byte load: target `(unknown effective address)`, base register `r2` (`ldr` at 0x3e08).
  Call (`mov` at 0x3e20): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), r0=`unknown`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x3e2c): taken to 0x3e48 when `ret(unknown) != 0x0`, falls through to 0x3e30 when `ret(unknown) == 0x0`.
  Four-byte load: target `sp +0x4` (`ldr` at 0x3e30).
  Four-byte load: target `ELF load bias (unknown) +0x3e60`, linked address 0x3e60, basis `ELF load-bias-relative` (`ldr` at 0x3e34).
  ELF literal warning: linked 0x3e60, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3e34.
  Call (`bl` at 0x3e38): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`[sp+4]`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Four-byte store: target `sp -0x28`, value `0x0` (`str` at 0x3e40).
  Jump (`b` at 0x3e44): target 0x3e50.
  Four-byte store: target `sp -0x28`, value `0x1` (`str` at 0x3e4c).
  Four-byte load: target `sp -0x28` (`ldr` at 0x3e50).
  `ldm` at 0x3e58 has recorded r0 = `unknown`.
  The instructions from 0x3e5c to 0x3e60 are not reached by any path from the entry, and their branches are not listed.

- `jmstr_set_functions`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0x32ec to 0x332c (exclusive), 64 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `fed263aa3dec0a577e1eac152a12e82259b261a7fccf509fb410c438df19db05`.
  Four-byte load: target `ELF load bias (unknown) +0x3324`, linked address 0x3324, basis `ELF load-bias-relative` (`ldr` at 0x3304).
  ELF literal warning: linked 0x3324, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3304.
  Four-byte store: target `(unknown effective address)`, value `r0`, base register `r2` (`str` at 0x330c).
  Four-byte load: target `ELF load bias (unknown) +0x3328`, linked address 0x3328, basis `ELF load-bias-relative` (`ldr` at 0x3310).
  ELF literal warning: linked 0x3328, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x3310.
  Four-byte store: target `(unknown effective address)`, value `[sp-24]`, base register `r2` (`str` at 0x3318).
  At the return (`ldm` at 0x3320), r0 is `r0`.
  The instructions from 0x3324 to 0x3328 are not reached by any path from the entry, and their branches are not listed.

- `llpcmd_set_drvr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0xc228 to 0xc2e8 (exclusive), 192 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `015debbd54b6e039fb4feb2b0fe03d6e589360bc1fbc4d4ea7685251e4c8a773`.
  Branch (`beq` at 0xc260): taken to 0xc26c when `(r0 & 0xff) == 0x0`, falls through to 0xc264 when `(r0 & 0xff) != 0x0`.
  Grouped accesses: 6 4-byte load records with target `sp +0x0`, including `ldr` at 0xc264, `ldr` at 0xc270, `ldr` at 0xc284.
  Call (`bl` at 0xc268): (unknown callee), target 0x1c6c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c6c, r0=`[sp]`, r1=`r3`, r2=`r2`, r3=`(r0 & 0xff)`.
  Four-byte store: target `[sp] +0xc`, value `unknown` (`str` at 0xc27c).
  Four-byte load: target `[sp] +0x10` (`ldr` at 0xc2a0).
  Four-byte store: target `[sp] +0x10`, value `unknown` (`str` at 0xc2ac).
  Four-byte load: target `[sp] +0x4` (`ldr` at 0xc2d0).
  Four-byte store: target `[sp] +0x4`, value `unknown` (`str` at 0xc2dc).
  The exit (`ldm` at 0xc2e4) reports r0 = `unknown`.

- `llpcmd_set_rcvr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libscan.armlinux.so`.
  Inferred analysis span: 0xc2e8 to 0xc3c4 (exclusive), 220 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 58312 bytes, SHA256 `a267e99f77f6dd127250abcbf83a0775e40207c48e77fe85260a4269d91f777f`.
  Read window: SHA256 `54023672fafe6510831098b35d05012ad5e1fbf73ea8820469a90e6e681501e0`.
  Branch (`beq` at 0xc324): taken to 0xc330 when `(r0 & 0xff) == 0x0`, falls through to 0xc328 when `(r0 & 0xff) != 0x0`.
  Grouped accesses: 7 4-byte load records with target `sp +0x0`, including `ldr` at 0xc328, `ldr` at 0xc334, `ldr` at 0xc348.
  Call (`bl` at 0xc32c): (unknown callee), target 0x1c6c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c6c, r0=`[sp]`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Grouped accesses: 3 4-byte load records with target `[sp] +0x8`, including `ldr` at 0xc350, `ldr` at 0xc380, `ldr` at 0xc3b0.
  Grouped accesses: 3 4-byte store records with target `[sp] +0x8`, reported value `unknown`, including `str` at 0xc35c, `str` at 0xc38c, `str` at 0xc3bc.
  Recorded r0 at the return is `unknown` (`ldm` at 0xc3c0).
