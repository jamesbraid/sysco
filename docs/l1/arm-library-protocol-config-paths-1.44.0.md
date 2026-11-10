# ARM library protocol and configuration paths

These entries read selected spans in `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package
`6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`. Symbol
metadata supplies the full inferred analysis spans. Bounded contexts and the local
control model support the reading, without proving complete code-only function extents
or runtime feasibility. Hardware roles remain unknown. Names beginning with L2 do not
identify the separate PowerPC L2 firmware.

The inferred entries describe bounded software observations under the native local
model, with calls opaque. `L2CfgMSetConsole` remains unknown because its indexed PC load
has no known taken destination. Its access, call and return observations cover known
local edges only. No missing destination or subsequent effect is supplied from raw words
or relocation records. A null target is not address zero.

Instruction and direct call coordinates are linked ELF virtual addresses, with lowest
linked load `0x0`. Runtime load bias remains unknown. Data references marked `ELF load
bias (unknown)` retain that coordinate uncertainty. Native runtime-constant coordinates
do not identify a physical address or pointee. Relocations are not applied, and
relocation-sensitive or writable literal values stay unknown.

Calls retain their native callee, target coordinate basis, execution state, canonical
instruction target, conditional status and four candidate AAPCS core-register slots.
Actual arity and custom conventions remain unknown. Bare register names denote entry
values, and `sp` denotes the entry stack pointer. A lone `?` becomes `unknown`. Compound
native expressions, including opaque call results, remain unchanged. Unsupported effects
are not repaired from raw operands.

Access groups retain the same native operation, width, value, implicit status, condition
and target metadata. Ordinary and unknown-address groups also retain the raw base,
displacement and addressing form. Simple PC-relative literal reads may share a group
when their native linked target and coordinate basis match, despite different
instruction-relative displacements. Every member's raw encoding remains in the full
facts. Counts describe static records, without merging branch arms, establishing equal
dynamic values or identifying one pointee.

Only independently paired frame saves and restores are omitted from summaries. Initial
spills, incoming reloads, outgoing slots, local return state, unmatched frame-like
records and ambiguous accesses remain. The full facts retain every access and warning.
Every deciding branch keeps both native predicates, including opaque conditions. The
prose includes selected unconditional jumps: loopbacks, transfers across retained raw
regions and known jumps in the unknown entry. Any out-of-span transfer is retained.
Other forward transfers remain in the complete facts.

Decoded records outside known local edges and undecodable regions remain in the full
spans. They are not established as dead, data or globally nonexecuting. A known return
does not prove a complete function boundary. The interior undecodable region in
`L2CfgMQuery` remains between known code regions, with its native jump retained. No
first-return crop or padding is used.

The selected full symbol spans, native facts, raw words, bounded contexts and ELF
symbol, segment and relocation metadata were read. These bounded routes do not claim
that every possible source was exhausted.

`L2CfgMSetConsole` is resolved in the last section of this note.

The [complete facts](disasm/arm-1.44.0-libraries/protocol-config-paths-facts.txt) preserve the full native listings for all selected spans.

- `PBayFlashFileOpen`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xedd4 to 0xef94 (exclusive), 448 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `c10994ff2580c5b9e20b86f387da3b4b2fb9c9c9eb873d0f8215df03dc4783e0`.
  The native facts retain 5 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xedd8).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xede4).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xede8).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xedec).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0xedf0).
  Grouped accesses: 5 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xedf4, `ldr` at 0xee20, `ldr` at 0xee70.
  Call (`bl` at 0xedfc): (unknown callee), target 0x36d0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36d0, conditional `false`, r0=`r0`, r1=`0x0`, r2=`r2`, r3=`r3`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x36d0)`, base register `r11`, raw displacement `-0x24` (`str` at 0xee04).
  Grouped accesses: 4 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xee08, `ldr` at 0xee44, `ldr` at 0xeef8.
  Branch (`bge` at 0xee10): taken to 0xee40 when `ret(linked ELF 0x36d0) >= 0x0 (signed, 32-bit)`, falls through to 0xee14 when `ret(linked ELF 0x36d0) < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0xef80`, base register `pc`, raw displacement `+0x15c`, linked address 0xef80, basis `ELF load-bias-relative` (`ldr` at 0xee1c).
  Native warning (`ldr` at 0xee1c): `ELF literal at linked 0xef80 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xee24): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`sp-212`, r1=`unknown`, r2=`[sp-20]`, r3=`sp-212`.
  Call (`bl` at 0xee30): (unknown callee), target 0x343c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x343c, conditional `false`, r0=`sp-212`, r1=`unknown`, r2=`unknown`, r3=`sp-212`.
  Warning class: 4 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0xee34, `mvn` at 0xee84, `mvn` at 0xeeec.
  Grouped accesses: 4 4-byte store records with target `sp -0xd8`, reported value `unknown` in every record, base register `r11`, raw displacement `-0xd4`, including `str` at 0xee38, `str` at 0xee88, `str` at 0xeef0.
  Call (`bl` at 0xee4c): (unknown callee), target 0x123cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x123cc, conditional `false`, r0=`ret(linked ELF 0x36d0)`, r1=`sp-128`, r2=`unknown`, r3=`sp-128`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x123cc)`, base register `r11`, raw displacement `-0x20` (`str` at 0xee54).
  Grouped accesses: 3 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xee58, `ldr` at 0xef10, `ldr` at 0xef40.
  Branch (`bge` at 0xee60): taken to 0xee90 when `ret(linked ELF 0x123cc) >= 0x0 (signed, 32-bit)`, falls through to 0xee64 when `ret(linked ELF 0x123cc) < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0xef84`, base register `pc`, raw displacement `+0x110`, linked address 0xef84, basis `ELF load-bias-relative` (`ldr` at 0xee6c).
  Native warning (`ldr` at 0xee6c): `ELF literal at linked 0xef84 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xee74): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`sp-212`, r1=`unknown`, r2=`[sp-20]`, r3=`sp-212`.
  Call (`bl` at 0xee80): (unknown callee), target 0x343c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x343c, conditional `false`, r0=`sp-212`, r1=`unknown`, r2=`unknown`, r3=`sp-212`.
  Grouped accesses: 5 4-byte load records with target `sp -0x54`, base register `r11`, raw displacement `-0x50`, including `ldr` at 0xee90, `ldr` at 0xeec4, `ldr` at 0xef00.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xee94, `ldr` at 0xeec8.
  Call (`bl` at 0xeea0): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`([sp-84] + [sp-24])`, r1=`unknown`, r2=`[sp-84]`, r3=`([sp-84] + [sp-24])`.
  Four-byte store: target `sp -0x84`, value `ret(linked ELF 0x3514)`, base register `r11`, raw displacement `-0x80` (`str` at 0xeea4).
  Grouped accesses: 3 4-byte load records with target `sp -0x84`, base register `r11`, raw displacement `-0x80`, including `ldr` at 0xeea8, `ldr` at 0xeefc, `ldr` at 0xef58.
  Branch (`bne` at 0xeeb0): taken to 0xeef8 when `ret(linked ELF 0x3514) != 0x0`, falls through to 0xeeb4 when `ret(linked ELF 0x3514) == 0x0`.
  Call (`bl` at 0xeeb4): (unknown callee), target 0x34d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34d8, conditional `false`, r0=`ret(linked ELF 0x3514)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3514)`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xef88`, linked address 0xef88, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xeec0, `ldr` at 0xef28.
  Warning class: 2 native records report `ELF literal at linked 0xef88 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xeec0, `ldr` at 0xef28.
  Four-byte load: target `ret(linked ELF 0x34d8) +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xeed0).
  Grouped accesses: 2 4-byte store records with target `sp -0xdc`, reported value `[ret(linked ELF 0x34d8)]` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0xeed4, `str` at 0xef30.
  Four-byte load: target `(unknown effective address)`, base register `r0`, raw displacement `+0x0`, raw addressing form `[r0]` (`ldr` at 0xeed8).
  Four-byte load: target `ELF load bias (unknown) +0xef8c`, base register `pc`, raw displacement `+0xa8`, linked address 0xef8c, basis `ELF load-bias-relative` (`ldr` at 0xeedc).
  Native warning (`ldr` at 0xeedc): `ELF literal at linked 0xef8c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xeee8): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-20]`, r3=`([sp-84] + [sp-24])`.
  Call (`bl` at 0xef04): (unknown callee), target 0x37c0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37c0, conditional `false`, r0=`[sp-40]`, r1=`ret(linked ELF 0x3514)`, r2=`[sp-84]`, r3=`ret(linked ELF 0x3514)`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x37c0)`, base register `r11`, raw displacement `-0x20` (`str` at 0xef0c).
  Branch (`beq` at 0xef1c): taken to 0xef54 when `ret(linked ELF 0x37c0) == [sp-84]`, falls through to 0xef20 when `ret(linked ELF 0x37c0) != [sp-84]`.
  Call (`bl` at 0xef20): (unknown callee), target 0x34d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34d8, conditional `false`, r0=`ret(linked ELF 0x37c0)`, r1=`unknown`, r2=`ret(linked ELF 0x37c0)`, r3=`[sp-84]`.
  Four-byte load: target `ret(linked ELF 0x34d8) +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0xef2c).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldr` at 0xef34).
  Four-byte load: target `ELF load bias (unknown) +0xef90`, base register `pc`, raw displacement `+0x50`, linked address 0xef90, basis `ELF load-bias-relative` (`ldr` at 0xef38).
  Native warning (`ldr` at 0xef38): `ELF literal at linked 0xef90 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xef44): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-20]`, r3=`[sp-36]`.
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xef54).
  Four-byte store: target `[sp-28] +0x0`, value `[sp-132]`, base register `r2`, raw displacement `+0x0` (`str` at 0xef5c).
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xef60).
  Four-byte store: target `[sp-32] +0x0`, value `[sp-84]`, base register `r2`, raw displacement `+0x0` (`str` at 0xef68).
  Four-byte store: target `sp -0xd8`, value `[sp-40]`, base register `r11`, raw displacement `-0xd4` (`str` at 0xef70).
  Four-byte load: target `sp -0xd8`, base register `r11`, raw displacement `-0xd4` (`ldr` at 0xef74).
  Known-edge return (`ldm` at 0xef7c): recorded r0 is `[sp-216]`.
  Known return boundary (`ldm` at 0xef7c): the modeled local edges end before the retained trailing region 0xef80 to 0xef94 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xef80 to 0xef90 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `PBayFlashValidateImage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xefbc to 0xf110 (exclusive), 340 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `1e2a368b03a28cd6983b4e4b89882273f9bd7ed61ac8e503b9b2aebaa12b6de7`.
  The native facts retain 1 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xefc0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xefcc).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xefd0).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xefd4).
  Grouped accesses: 10 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xefdc, `ldr` at 0xeff4, `ldr` at 0xf00c.
  Byte load: target `r0 +0x4`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xefe4).
  Branch (`bne` at 0xefec): taken to 0xf0e8 when `[r0+4]:1 != 0x3c`, falls through to 0xeff0 when `[r0+4]:1 == 0x3c`.
  Byte load: target `r0 +0x5`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xeffc).
  Branch (`bne` at 0xf004): taken to 0xf0e8 when `[r0+5]:1 != 0x3c`, falls through to 0xf008 when `[r0+5]:1 == 0x3c`.
  Byte load: target `r0 +0x6`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf014).
  Branch (`bne` at 0xf01c): taken to 0xf0e8 when `[r0+6]:1 != 0x20`, falls through to 0xf020 when `[r0+6]:1 == 0x20`.
  Byte load: target `r0 +0x7`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf02c).
  Branch (`bne` at 0xf034): taken to 0xf0e8 when `[r0+7]:1 != 0x41`, falls through to 0xf038 when `[r0+7]:1 == 0x41`.
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xf038).
  Branch (`beq` at 0xf040): taken to 0xf0dc when `r2 == 0x0`, falls through to 0xf044 when `r2 != 0x0`.
  Byte load: target `r0 +0x1e`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf050).
  Byte load: target `r0 +0x1f`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf064).
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0xf06c).
  Four-byte load: target `ELF load bias (unknown) +0xf0fc`, base register `pc`, raw displacement `+0x84`, linked address 0xf0fc, basis `ELF load-bias-relative` (`ldr` at 0xf070).
  Byte load: target `r0 +0x11ce`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf07c).
  Four-byte load: target `ELF load bias (unknown) +0xf100`, base register `pc`, raw displacement `+0x74`, linked address 0xf100, basis `ELF load-bias-relative` (`ldr` at 0xf084).
  Byte load: target `r0 +0x11c9`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf090).
  Four-byte load: target `ELF load bias (unknown) +0xf104`, base register `pc`, raw displacement `+0x60`, linked address 0xf104, basis `ELF load-bias-relative` (`ldr` at 0xf09c).
  Byte load: target `r0 +0x11c4`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf0a8).
  Four-byte load: target `ELF load bias (unknown) +0xf108`, base register `pc`, raw displacement `+0x4c`, linked address 0xf108, basis `ELF load-bias-relative` (`ldr` at 0xf0b4).
  Byte load: target `r0 +0x11bf`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xf0c0).
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0xf0c8).
  Four-byte load: target `ELF load bias (unknown) +0xf10c`, base register `pc`, raw displacement `+0x38`, linked address 0xf10c, basis `ELF load-bias-relative` (`ldr` at 0xf0cc).
  Native warning (`ldr` at 0xf0cc): `ELF literal at linked 0xf10c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xf0d0).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xf0d4).
  Call (`bl` at 0xf0d8): (unknown callee), target 0xed40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xed40, conditional `false`, r0=`unknown`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Four-byte store: target `sp -0x28`, value `0x0`, base register `r11`, raw displacement `-0x24` (`str` at 0xf0e0).
  Native warning (`mvn` at 0xf0e8): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x28`, value `unknown`, base register `r11`, raw displacement `-0x24` (`str` at 0xf0ec).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0xf0f0).
  At the known-edge return (`ldm` at 0xf0f8), r0 is `[sp-40]`.
  Known return boundary (`ldm` at 0xf0f8): the modeled local edges end before the retained trailing region 0xf0fc to 0xf110 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xf0fc to 0xf10c are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `PBayFlashUpdateRemote`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xf19c to 0xfbac (exclusive), 2576 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `a9b500bd5e46c59194a04ca1474a681b3e39e5f9bfcebd4cbde972e4817ca23b`.
  The native facts retain 19 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0xf1a0).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0xf1ac).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0xf1b0).
  Four-byte store: target `sp -0x20`, value `r2`, base register `r11`, raw displacement `-0x1c` (`str` at 0xf1b4).
  Four-byte store: target `sp -0x24`, value `r3`, base register `r11`, raw displacement `-0x20` (`str` at 0xf1b8).
  Four-byte load: target `ELF load bias (unknown) +0xfb50`, base register `pc`, raw displacement `+0x98c`, linked address 0xfb50, basis `ELF load-bias-relative` (`ldr` at 0xf1bc).
  Native warning (`ldr` at 0xf1bc): `ELF literal at linked 0xfb50 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `r3, {r0, r1}` (`ldm` at 0xf1c4).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `r3, {r0, r1}` (`ldm` at 0xf1c4).
  Four-byte store: target `sp -0x4c`, value `unknown`, base register `r2`, raw displacement `+0x0` (`str` at 0xf1c8).
  Two-byte store: target `sp -0x48`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strh` at 0xf1d0).
  Four-byte load: target `ELF load bias (unknown) +0xfb54`, base register `pc`, raw displacement `+0x978`, linked address 0xfb54, basis `ELF load-bias-relative` (`ldr` at 0xf1d4).
  Native warning (`ldr` at 0xf1d4): `ELF literal at linked 0xfb54 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf1d8): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`sp-72`, r3=`unknown`.
  Grouped accesses: 5 4-byte store records with target `sp -0xac`, reported value `sp-56` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0xf1e4, `str` at 0xf410, `str` at 0xf624.
  Grouped accesses: 5 4-byte store records with target `sp -0xa8`, reported value `0x1e` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0xf1ec, `str` at 0xf418, `str` at 0xf62c.
  Call (`bl` at 0xf200): (unknown callee), target 0x3508, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3508, conditional `false`, r0=`sp-32`, r1=`0x92`, r2=`0x0`, r3=`0x0`.
  Grouped accesses: 5 4-byte store records with target `sp -0x64`, reported value `ret(linked ELF 0x3508)` in every record, base register `r11`, raw displacement `-0x60`, including `str` at 0xf208, `str` at 0xf434, `str` at 0xf648.
  Grouped accesses: 15 4-byte load records with target `sp -0x64`, base register `r11`, raw displacement `-0x60`, including `ldr` at 0xf20c, `ldr` at 0xf264, `ldr` at 0xf290.
  Branch (`bne` at 0xf214): taken to 0xf264 when `ret(linked ELF 0x3508) != 0x0`, falls through to 0xf218 when `ret(linked ELF 0x3508) == 0x0`.
  Grouped accesses: 45 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0xf218, `ldr` at 0xf228, `ldr` at 0xf29c.
  Grouped accesses: 10 1-byte load records with target `[sp-56] +0x13`, base register `r3`, raw displacement `+0x13`, including `ldrb` at 0xf21c, `ldrb` at 0xf22c, `ldrb` at 0xf448.
  Branch (`beq` at 0xf224): taken to 0xf248 when `[[sp-56]+19]:1 == 0x0`, falls through to 0xf228 when `[[sp-56]+19]:1 != 0x0`.
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0xfb58`, linked address 0xfb58, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xf230, `ldr` at 0xf45c, `ldr` at 0xf670.
  Four-byte store: target `sp -0x6c`, value `0xfffffda8`, base register `r11`, raw displacement `-0x68` (`str` at 0xf234).
  Grouped accesses: 2 4-byte load records with target `sp -0x6c`, base register `r11`, raw displacement `-0x68`, including `ldr` at 0xf238, `ldr` at 0xf250.
  Warning class: 6 native records report `rsb: unsupported effects invalidated`, including `rsb` at 0xf23c, `rsb` at 0xf468, `rsb` at 0xf590.
  Four-byte store: target `sp -0x6c`, value `unknown`, base register `r11`, raw displacement `-0x68` (`str` at 0xf240).
  Four-byte store: target `sp -0x6c`, value `0x0`, base register `r11`, raw displacement `-0x68` (`str` at 0xf24c).
  Grouped accesses: 5 4-byte store records with target `sp -0x64`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x60`, including `str` at 0xf254, `str` at 0xf480, `str` at 0xf694.
  Branch (`bne` at 0xf25c): taken to 0xf264 when `Z == 0`, falls through to 0xf260 when `not (Z == 0)`.
  Call (`bl` at 0xf268): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0xfb5c`, linked address 0xfb5c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xf270, `ldr` at 0xf49c, `ldr` at 0xf6b0.
  Warning class: 5 native records report `ELF literal at linked 0xfb5c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xf270, `ldr` at 0xf49c, `ldr` at 0xf6b0.
  Grouped accesses: 5 4-byte store records with target `sp -0xac`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0xf274, `str` at 0xf4a0, `str` at 0xf6b4.
  Grouped accesses: 5 4-byte store records with target `sp -0xa8`, reported value `ret(linked ELF 0x36a0)` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0xf278, `str` at 0xf4a4, `str` at 0xf6b8.
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0xfb60`, linked address 0xfb60, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xf280, `ldr` at 0xf4ac, `ldr` at 0xf6c0.
  Warning class: 5 native records report `ELF literal at linked 0xfb60 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xf280, `ldr` at 0xf4ac, `ldr` at 0xf6c0.
  Four-byte load: target `ELF load bias (unknown) +0xfb64`, base register `pc`, raw displacement `+0x8d8`, linked address 0xfb64, basis `ELF load-bias-relative` (`ldr` at 0xf284).
  Call (`bl` at 0xf28c): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x103`, r3=`0x0`.
  Grouped accesses: 5 4-byte store records with target `sp -0x68`, reported value `[sp-100]` in every record, base register `r11`, raw displacement `-0x64`, including `str` at 0xf294, `str` at 0xf4c0, `str` at 0xf6d4.
  Grouped accesses: 5 1-byte load records with target `[sp-56] +0x24`, base register `r3`, raw displacement `+0x24`, including `ldrb` at 0xf2a0, `ldrb` at 0xf4cc, `ldrb` at 0xf6e0.
  Branch (`bne` at 0xf2ac): taken to 0xf2c0 when `? != 0x0`, falls through to 0xf2b0 when `? == 0x0`.
  Grouped accesses: 5 4-byte load records with target `[sp-56] +0x28`, base register `r3`, raw displacement `+0x28`, including `ldr` at 0xf2b4, `ldr` at 0xf4e0, `ldr` at 0xf6f4.
  Four-byte store: target `sp -0x70`, value `[[sp-56]+40]`, base register `r11`, raw displacement `-0x6c` (`str` at 0xf2b8).
  Call (`bl` at 0xf2c8): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x70`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x6c` (`str` at 0xf2cc).
  Four-byte load: target `sp -0x70`, base register `r11`, raw displacement `-0x6c` (`ldr` at 0xf2d0).
  Four-byte store: target `sp -0x54`, value `[sp-112]`, base register `r11`, raw displacement `-0x50` (`str` at 0xf2d4).
  Grouped accesses: 2 1-byte load records with target `[sp-56] +0x2c`, base register `r3`, raw displacement `+0x2c`, including `ldrb` at 0xf2dc, `ldrb` at 0xfa84.
  Branch (`bne` at 0xf2e8): taken to 0xf2fc when `? != 0x0`, falls through to 0xf2ec when `? == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp-56] +0x30`, base register `r3`, raw displacement `+0x30`, including `ldr` at 0xf2f0, `ldr` at 0xfa98.
  Four-byte store: target `sp -0x74`, value `[[sp-56]+48]`, base register `r11`, raw displacement `-0x70` (`str` at 0xf2f4).
  Call (`bl` at 0xf304): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x74`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x70` (`str` at 0xf308).
  Four-byte load: target `sp -0x74`, base register `r11`, raw displacement `-0x70` (`ldr` at 0xf30c).
  Four-byte store: target `sp -0x58`, value `[sp-116]`, base register `r11`, raw displacement `-0x54` (`str` at 0xf310).
  Grouped accesses: 2 1-byte load records with target `[sp-56] +0x34`, base register `r3`, raw displacement `+0x34`, including `ldrb` at 0xf318, `ldrb` at 0xfac0.
  Branch (`bne` at 0xf324): taken to 0xf338 when `? != 0x0`, falls through to 0xf328 when `? == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp-56] +0x38`, base register `r3`, raw displacement `+0x38`, including `ldr` at 0xf32c, `ldr` at 0xfad4.
  Four-byte store: target `sp -0x78`, value `[[sp-56]+56]`, base register `r11`, raw displacement `-0x74` (`str` at 0xf330).
  Call (`bl` at 0xf340): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x78`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x74` (`str` at 0xf344).
  Four-byte load: target `sp -0x78`, base register `r11`, raw displacement `-0x74` (`ldr` at 0xf348).
  Four-byte store: target `sp -0x5c`, value `[sp-120]`, base register `r11`, raw displacement `-0x58` (`str` at 0xf34c).
  Byte load: target `[sp-56] +0x3c`, base register `r3`, raw displacement `+0x3c` (`ldrb` at 0xf354).
  Branch (`bne` at 0xf360): taken to 0xf374 when `? != 0x0`, falls through to 0xf364 when `? == 0x0`.
  Four-byte load: target `[sp-56] +0x40`, base register `r3`, raw displacement `+0x40` (`ldr` at 0xf368).
  Four-byte store: target `sp -0x7c`, value `[[sp-56]+64]`, base register `r11`, raw displacement `-0x78` (`str` at 0xf36c).
  Call (`bl` at 0xf37c): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x3`, r2=`[sp-120]`, r3=`unknown`.
  Four-byte store: target `sp -0x7c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x78` (`str` at 0xf380).
  Four-byte load: target `sp -0x7c`, base register `r11`, raw displacement `-0x78` (`ldr` at 0xf384).
  Four-byte store: target `sp -0x60`, value `[sp-124]`, base register `r11`, raw displacement `-0x5c` (`str` at 0xf388).
  Call (`bl` at 0xf390): (unknown callee), target 0x33dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`[sp-124]`.
  Grouped accesses: 10 4-byte load records with target `sp -0x54`, base register `r11`, raw displacement `-0x50`, including `ldr` at 0xf394, `ldr` at 0xf3ac, `ldr` at 0xf50c.
  Branch (`beq` at 0xf39c): taken to 0xf3c8 when `[sp-84] == 0x0`, falls through to 0xf3a0 when `[sp-84] != 0x0`.
  PC-relative loads: 8 4-byte records report target expression `ELF load bias (unknown) +0xfb68`, linked address 0xfb68, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xf3a0, `ldr` at 0xf3b4, `ldr` at 0xf3e4.
  Warning class: 8 native records report `ELF literal at linked 0xfb68 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xf3a0, `ldr` at 0xf3b4, `ldr` at 0xf3e4.
  Grouped accesses: 9 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xf3a4, `ldr` at 0xf3b8, `ldr` at 0xf3e8.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xfb6c`, linked address 0xfb6c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xf3a8, `ldr` at 0xfb14.
  Warning class: 2 native records report `ELF literal at linked 0xfb6c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xf3a8, `ldr` at 0xfb14.
  Call (`bl` at 0xf3b0): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-84]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfb70`, base register `pc`, raw displacement `+0x7ac`, linked address 0xfb70, basis `ELF load-bias-relative` (`ldr` at 0xf3bc).
  Native warning (`ldr` at 0xf3bc): `ELF literal at linked 0xfb70 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf3c0): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfb74`, base register `pc`, raw displacement `+0x7a4`, linked address 0xfb74, basis `ELF load-bias-relative` (`ldr` at 0xf3c8).
  Native warning (`ldr` at 0xf3c8): `ELF literal at linked 0xfb74 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte load records with target `sp -0x58`, base register `r11`, raw displacement `-0x54`, including `ldr` at 0xf3cc, `ldr` at 0xfb30.
  Grouped accesses: 2 4-byte load records with target `sp -0x5c`, base register `r11`, raw displacement `-0x58`, including `ldr` at 0xf3d0, `ldr` at 0xfb34.
  Call (`bl` at 0xf3d4): (unknown callee), target 0xed40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xed40, conditional `false`, r0=`unknown`, r1=`[sp-88]`, r2=`[sp-92]`, r3=`[sp-84]`.
  Four-byte load: target `sp +0x0`, base register `r11`, raw displacement `+0x4` (`ldr` at 0xf3d8).
  Branch (`bne` at 0xf3e0): taken to 0xf400 when `[sp] != 0x0`, falls through to 0xf3e4 when `[sp] == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xfb78`, base register `pc`, raw displacement `+0x784`, linked address 0xfb78, basis `ELF load-bias-relative` (`ldr` at 0xf3ec).
  Native warning (`ldr` at 0xf3ec): `ELF literal at linked 0xfb78 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf3f0): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x68`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x64`, including `str` at 0xf3f8, `str` at 0xfb40.
  Four-byte load: target `ELF load bias (unknown) +0xfb7c`, base register `pc`, raw displacement `+0x774`, linked address 0xfb7c, basis `ELF load-bias-relative` (`ldr` at 0xf400).
  Native warning (`ldr` at 0xf400): `ELF literal at linked 0xfb7c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf404): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf42c): (unknown callee), target 0x3508, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3508, conditional `false`, r0=`sp-32`, r1=`0x94`, r2=`0x0`, r3=`0x0`.
  Branch (`bne` at 0xf440): taken to 0xf490 when `ret(linked ELF 0x3508) != 0x0`, falls through to 0xf444 when `ret(linked ELF 0x3508) == 0x0`.
  Branch (`beq` at 0xf450): taken to 0xf474 when `[[sp-56]+19]:1 == 0x0`, falls through to 0xf454 when `[[sp-56]+19]:1 != 0x0`.
  Four-byte store: target `sp -0x80`, value `0xfffffda8`, base register `r11`, raw displacement `-0x7c` (`str` at 0xf460).
  Grouped accesses: 2 4-byte load records with target `sp -0x80`, base register `r11`, raw displacement `-0x7c`, including `ldr` at 0xf464, `ldr` at 0xf47c.
  Four-byte store: target `sp -0x80`, value `unknown`, base register `r11`, raw displacement `-0x7c` (`str` at 0xf46c).
  Four-byte store: target `sp -0x80`, value `0x0`, base register `r11`, raw displacement `-0x7c` (`str` at 0xf478).
  Branch (`bne` at 0xf488): taken to 0xf490 when `Z == 0`, falls through to 0xf48c when `not (Z == 0)`.
  Call (`bl` at 0xf494): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf4b8): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x11c`, r3=`0x0`.
  Branch (`bne` at 0xf4d8): taken to 0xf4ec when `? != 0x0`, falls through to 0xf4dc when `? == 0x0`.
  Four-byte store: target `sp -0x84`, value `[[sp-56]+40]`, base register `r11`, raw displacement `-0x80` (`str` at 0xf4e4).
  Call (`bl` at 0xf4f4): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x84`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x80` (`str` at 0xf4f8).
  Four-byte load: target `sp -0x84`, base register `r11`, raw displacement `-0x80` (`ldr` at 0xf4fc).
  Four-byte store: target `sp -0x54`, value `[sp-132]`, base register `r11`, raw displacement `-0x50` (`str` at 0xf500).
  Call (`bl` at 0xf508): (unknown callee), target 0x33dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`[sp-132]`.
  Branch (`beq` at 0xf514): taken to 0xf52c when `[sp-84] == 0x0`, falls through to 0xf518 when `[sp-84] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xfb80`, base register `pc`, raw displacement `+0x658`, linked address 0xfb80, basis `ELF load-bias-relative` (`ldr` at 0xf520).
  Native warning (`ldr` at 0xf520): `ELF literal at linked 0xfb80 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf528): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-84]`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x50`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x4c`, including `str` at 0xf530, `str` at 0xf7e8.
  Four-byte load: target `ELF load bias (unknown) +0xfb84`, base register `pc`, raw displacement `+0x648`, linked address 0xfb84, basis `ELF load-bias-relative` (`ldr` at 0xf534).
  Native warning (`ldr` at 0xf534): `ELF literal at linked 0xfb84 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf538): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Four-byte load: target `sp -0x60`, base register `r11`, raw displacement `-0x5c` (`ldr` at 0xf53c).
  Four-byte store: target `sp -0x3c`, value `[sp-96]`, base register `r11`, raw displacement `-0x38` (`str` at 0xf540).
  Grouped accesses: 5 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0xf544, `ldr` at 0xf574, `ldr` at 0xf5a0.
  Branch (`ble` at 0xf54c): taken to 0xf558 when `[sp-96] <= 0x384 (signed, 32-bit)`, falls through to 0xf550 when `[sp-96] > 0x384 (signed, 32-bit)`.
  Four-byte store: target `sp -0x3c`, value `0x384`, base register `r11`, raw displacement `-0x38` (`str` at 0xf554).
  Four-byte store: target `sp -0x40`, value `0x0`, base register `r11`, raw displacement `-0x3c` (`str` at 0xf55c).
  Grouped accesses: 7 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0xf560, `ldr` at 0xf570, `ldr` at 0xf58c.
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xf564, `ldr` at 0xf57c, `ldr` at 0xf588.
  Branch (`bhs` at 0xf56c): taken to 0xf800 when `C == 1`, falls through to 0xf570 when `not (C == 1)`.
  Branch (`blo` at 0xf584): taken to 0xf598 when `C == 0`, falls through to 0xf588 when `not (C == 0)`.
  Four-byte store: target `sp -0x3c`, value `unknown`, base register `r11`, raw displacement `-0x38` (`str` at 0xf594).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xf598).
  Call (`bl` at 0xf5a4): (unknown callee), target 0x36f4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36f4, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x44`, value `ret(linked ELF 0x36f4)`, base register `r11`, raw displacement `-0x40` (`str` at 0xf5ac).
  Grouped accesses: 3 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0xf5b0, `ldr` at 0xf608, `ldr` at 0xf830.
  Branch (`bne` at 0xf5b8): taken to 0xf5d8 when `ret(linked ELF 0x36f4) != 0x0`, falls through to 0xf5bc when `ret(linked ELF 0x36f4) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xfb88`, base register `pc`, raw displacement `+0x5bc`, linked address 0xfb88, basis `ELF load-bias-relative` (`ldr` at 0xf5c4).
  Native warning (`ldr` at 0xf5c4): `ELF literal at linked 0xfb88 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf5c8): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Warning class: 5 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0xf5cc, `mvn` at 0xf740, `mvn` at 0xf754.
  Grouped accesses: 4 4-byte store records with target `sp -0x68`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x64`, including `str` at 0xf5d0, `str` at 0xf744, `str` at 0xf96c.
  Grouped accesses: 2 1-byte store records with target `sp -0x34`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x30`, including `strb` at 0xf5dc, `strb` at 0xf804.
  Grouped accesses: 2 1-byte store records with target `sp -0x33`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x2f`, including `strb` at 0xf5e4, `strb` at 0xf80c.
  Four-byte store: target `sp -0x30`, value `[sp-64]`, base register `r11`, raw displacement `-0x2c` (`str` at 0xf5ec).
  Grouped accesses: 2 2-byte store records with target `sp -0x32`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x2e`, including `strh` at 0xf5f4, `strh` at 0xf81c.
  Grouped accesses: 2 1-byte store records with target `sp -0x2c`, reported value `0x9` in every record, base register `r11`, raw displacement `-0x28`, including `strb` at 0xf5fc, `strb` at 0xf824.
  Grouped accesses: 2 1-byte store records with target `sp -0x2b`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x27`, including `strb` at 0xf604, `strb` at 0xf82c.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x36f4)`, base register `r11`, raw displacement `-0x24` (`str` at 0xf60c).
  Two-byte store: target `sp -0x2a`, value `([sp-60] & 0xffff)`, base register `r11`, raw displacement `-0x26` (`strh` at 0xf614).
  Call (`bl` at 0xf640): (unknown callee), target 0x3508, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3508, conditional `false`, r0=`sp-32`, r1=`0x93`, r2=`0x2`, r3=`sp-52`.
  Branch (`bne` at 0xf654): taken to 0xf6a4 when `ret(linked ELF 0x3508) != 0x0`, falls through to 0xf658 when `ret(linked ELF 0x3508) == 0x0`.
  Branch (`beq` at 0xf664): taken to 0xf688 when `[[sp-56]+19]:1 == 0x0`, falls through to 0xf668 when `[[sp-56]+19]:1 != 0x0`.
  Four-byte store: target `sp -0x88`, value `0xfffffda8`, base register `r11`, raw displacement `-0x84` (`str` at 0xf674).
  Grouped accesses: 2 4-byte load records with target `sp -0x88`, base register `r11`, raw displacement `-0x84`, including `ldr` at 0xf678, `ldr` at 0xf690.
  Four-byte store: target `sp -0x88`, value `unknown`, base register `r11`, raw displacement `-0x84` (`str` at 0xf680).
  Four-byte store: target `sp -0x88`, value `0x0`, base register `r11`, raw displacement `-0x84` (`str` at 0xf68c).
  Branch (`bne` at 0xf69c): taken to 0xf6a4 when `Z == 0`, falls through to 0xf6a0 when `not (Z == 0)`.
  Call (`bl` at 0xf6a8): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf6cc): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x148`, r3=`0x0`.
  Branch (`bne` at 0xf6ec): taken to 0xf700 when `? != 0x0`, falls through to 0xf6f0 when `? == 0x0`.
  Four-byte store: target `sp -0x8c`, value `[[sp-56]+40]`, base register `r11`, raw displacement `-0x88` (`str` at 0xf6f8).
  Call (`bl` at 0xf708): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x8c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x88` (`str` at 0xf70c).
  Four-byte load: target `sp -0x8c`, base register `r11`, raw displacement `-0x88` (`ldr` at 0xf710).
  Four-byte store: target `sp -0x54`, value `[sp-140]`, base register `r11`, raw displacement `-0x50` (`str` at 0xf714).
  Call (`bl` at 0xf71c): (unknown callee), target 0x33dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`[sp-140]`.
  Branch (`beq` at 0xf728): taken to 0xf74c when `[sp-84] == 0x0`, falls through to 0xf72c when `[sp-84] != 0x0`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xfb8c`, linked address 0xfb8c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xf734, `ldr` at 0xf95c.
  Warning class: 2 native records report `ELF literal at linked 0xfb8c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xf734, `ldr` at 0xf95c.
  Call (`bl` at 0xf73c): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-84]`, r3=`unknown`.
  Four-byte load: target `sp -0x50`, base register `r0`, raw displacement `+0x0` (`ldr` at 0xf750).
  Byte store: target `(unknown effective address)`, value `0x2e`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`strb` at 0xf76c).
  Four-byte store: target `sp -0x50`, value `[sp-80]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0xf774).
  Call (`bl` at 0xf7a0): (unknown callee), target 0x122b8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x122b8, conditional `false`, r0=`unknown`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfb90`, base register `pc`, raw displacement `+0x3e0`, linked address 0xfb90, basis `ELF load-bias-relative` (`ldr` at 0xf7a8).
  Native warning (`ldr` at 0xf7a8): `ELF literal at linked 0xfb90 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf7b4): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`unknown`, r1=`sp-76`, r2=`ret(linked ELF 0x122b8)`, r3=`ret(linked ELF 0x122b8)`.
  Four-byte load: target `ELF load bias (unknown) +0xfb94`, base register `pc`, raw displacement `+0x3d4`, linked address 0xfb94, basis `ELF load-bias-relative` (`ldr` at 0xf7b8).
  Native warning (`ldr` at 0xf7b8): `ELF literal at linked 0xfb94 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf7c0): (unknown callee), target 0x3460, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3460, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x50`, base register `r11`, raw displacement `-0x4c` (`ldr` at 0xf7c4).
  Branch (`bls` at 0xf7cc): taken to 0xf7ec when `[sp-80] <= 0x4 (unsigned, 32-bit)`, falls through to 0xf7d0 when `[sp-80] > 0x4 (unsigned, 32-bit)`.
  Call (`bl` at 0xf7e0): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`sp-76`, r1=`0x20`, r2=`0x5`, r3=`sp-76`.
  Four-byte store: target `sp -0x40`, value `([sp-64] + [sp-60])`, base register `r11`, raw displacement `-0x3c` (`str` at 0xf7f8).
  Jump (`b` at 0xf7fc): target 0xf560.
  Four-byte store: target `sp -0x30`, value `0x0`, base register `r11`, raw displacement `-0x2c` (`str` at 0xf814).
  Four-byte store: target `sp -0x28`, value `[sp-68]`, base register `r11`, raw displacement `-0x24` (`str` at 0xf834).
  Two-byte store: target `sp -0x2a`, value `0x0`, base register `r11`, raw displacement `-0x26` (`strh` at 0xf83c).
  Call (`bl` at 0xf868): (unknown callee), target 0x3508, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3508, conditional `false`, r0=`sp-32`, r1=`0x93`, r2=`0x2`, r3=`sp-52`.
  Branch (`bne` at 0xf87c): taken to 0xf8cc when `ret(linked ELF 0x3508) != 0x0`, falls through to 0xf880 when `ret(linked ELF 0x3508) == 0x0`.
  Branch (`beq` at 0xf88c): taken to 0xf8b0 when `[[sp-56]+19]:1 == 0x0`, falls through to 0xf890 when `[[sp-56]+19]:1 != 0x0`.
  Four-byte store: target `sp -0x90`, value `0xfffffda8`, base register `r11`, raw displacement `-0x8c` (`str` at 0xf89c).
  Grouped accesses: 2 4-byte load records with target `sp -0x90`, base register `r11`, raw displacement `-0x8c`, including `ldr` at 0xf8a0, `ldr` at 0xf8b8.
  Four-byte store: target `sp -0x90`, value `unknown`, base register `r11`, raw displacement `-0x8c` (`str` at 0xf8a8).
  Four-byte store: target `sp -0x90`, value `0x0`, base register `r11`, raw displacement `-0x8c` (`str` at 0xf8b4).
  Branch (`bne` at 0xf8c4): taken to 0xf8cc when `Z == 0`, falls through to 0xf8c8 when `not (Z == 0)`.
  Call (`bl` at 0xf8d0): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfb98`, base register `pc`, raw displacement `+0x2a4`, linked address 0xfb98, basis `ELF load-bias-relative` (`ldr` at 0xf8ec).
  Call (`bl` at 0xf8f4): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x166`, r3=`0x0`.
  Branch (`bne` at 0xf914): taken to 0xf928 when `? != 0x0`, falls through to 0xf918 when `? == 0x0`.
  Four-byte store: target `sp -0x94`, value `[[sp-56]+40]`, base register `r11`, raw displacement `-0x90` (`str` at 0xf920).
  Call (`bl` at 0xf930): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x94`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x90` (`str` at 0xf934).
  Four-byte load: target `sp -0x94`, base register `r11`, raw displacement `-0x90` (`ldr` at 0xf938).
  Four-byte store: target `sp -0x54`, value `[sp-148]`, base register `r11`, raw displacement `-0x50` (`str` at 0xf93c).
  Call (`bl` at 0xf944): (unknown callee), target 0x33dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`[sp-148]`.
  Branch (`beq` at 0xf950): taken to 0xf974 when `[sp-84] == 0x0`, falls through to 0xf954 when `[sp-84] != 0x0`.
  Call (`bl` at 0xf964): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-84]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfb9c`, base register `pc`, raw displacement `+0x220`, linked address 0xfb9c, basis `ELF load-bias-relative` (`ldr` at 0xf974).
  Native warning (`ldr` at 0xf974): `ELF literal at linked 0xfb9c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf978): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-84]`.
  Four-byte load: target `ELF load bias (unknown) +0xfba0`, base register `pc`, raw displacement `+0x21c`, linked address 0xfba0, basis `ELF load-bias-relative` (`ldr` at 0xf97c).
  Native warning (`ldr` at 0xf97c): `ELF literal at linked 0xfba0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xf980): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9a8): (unknown callee), target 0x3508, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3508, conditional `false`, r0=`sp-32`, r1=`0x92`, r2=`0x0`, r3=`0x0`.
  Branch (`bne` at 0xf9bc): taken to 0xfa0c when `ret(linked ELF 0x3508) != 0x0`, falls through to 0xf9c0 when `ret(linked ELF 0x3508) == 0x0`.
  Branch (`beq` at 0xf9cc): taken to 0xf9f0 when `[[sp-56]+19]:1 == 0x0`, falls through to 0xf9d0 when `[[sp-56]+19]:1 != 0x0`.
  Four-byte store: target `sp -0x98`, value `0xfffffda8`, base register `r11`, raw displacement `-0x94` (`str` at 0xf9dc).
  Grouped accesses: 2 4-byte load records with target `sp -0x98`, base register `r11`, raw displacement `-0x94`, including `ldr` at 0xf9e0, `ldr` at 0xf9f8.
  Four-byte store: target `sp -0x98`, value `unknown`, base register `r11`, raw displacement `-0x94` (`str` at 0xf9e8).
  Four-byte store: target `sp -0x98`, value `0x0`, base register `r11`, raw displacement `-0x94` (`str` at 0xf9f4).
  Branch (`bne` at 0xfa04): taken to 0xfa0c when `Z == 0`, falls through to 0xfa08 when `not (Z == 0)`.
  Call (`bl` at 0xfa10): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfba4`, base register `pc`, raw displacement `+0x170`, linked address 0xfba4, basis `ELF load-bias-relative` (`ldr` at 0xfa2c).
  Call (`bl` at 0xfa34): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x177`, r3=`0x0`.
  Branch (`bne` at 0xfa54): taken to 0xfa68 when `? != 0x0`, falls through to 0xfa58 when `? == 0x0`.
  Four-byte store: target `sp -0x9c`, value `[[sp-56]+40]`, base register `r11`, raw displacement `-0x98` (`str` at 0xfa60).
  Call (`bl` at 0xfa70): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x9c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x98` (`str` at 0xfa74).
  Four-byte load: target `sp -0x9c`, base register `r11`, raw displacement `-0x98` (`ldr` at 0xfa78).
  Four-byte store: target `sp -0x54`, value `[sp-156]`, base register `r11`, raw displacement `-0x50` (`str` at 0xfa7c).
  Branch (`bne` at 0xfa90): taken to 0xfaa4 when `? != 0x0`, falls through to 0xfa94 when `? == 0x0`.
  Four-byte store: target `sp -0xa0`, value `[[sp-56]+48]`, base register `r11`, raw displacement `-0x9c` (`str` at 0xfa9c).
  Call (`bl` at 0xfaac): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0xa0`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x9c` (`str` at 0xfab0).
  Four-byte load: target `sp -0xa0`, base register `r11`, raw displacement `-0x9c` (`ldr` at 0xfab4).
  Four-byte store: target `sp -0x58`, value `[sp-160]`, base register `r11`, raw displacement `-0x54` (`str` at 0xfab8).
  Branch (`bne` at 0xfacc): taken to 0xfae0 when `? != 0x0`, falls through to 0xfad0 when `? == 0x0`.
  Four-byte store: target `sp -0xa4`, value `[[sp-56]+56]`, base register `r11`, raw displacement `-0xa0` (`str` at 0xfad8).
  Call (`bl` at 0xfae8): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-56]`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0xa4`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0xa0` (`str` at 0xfaec).
  Four-byte load: target `sp -0xa4`, base register `r11`, raw displacement `-0xa0` (`ldr` at 0xfaf0).
  Four-byte store: target `sp -0x5c`, value `[sp-164]`, base register `r11`, raw displacement `-0x58` (`str` at 0xfaf4).
  Call (`bl` at 0xfafc): (unknown callee), target 0x33dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`[sp-164]`, r3=`unknown`.
  Branch (`beq` at 0xfb08): taken to 0xfb2c when `[sp-84] == 0x0`, falls through to 0xfb0c when `[sp-84] != 0x0`.
  Call (`bl` at 0xfb1c): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-84]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfba8`, base register `pc`, raw displacement `+0x74`, linked address 0xfba8, basis `ELF load-bias-relative` (`ldr` at 0xfb2c).
  Native warning (`ldr` at 0xfb2c): `ELF literal at linked 0xfba8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xfb38): (unknown callee), target 0xed40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xed40, conditional `false`, r0=`unknown`, r1=`[sp-88]`, r2=`[sp-92]`, r3=`[sp-84]`.
  Four-byte load: target `sp -0x68`, base register `r11`, raw displacement `-0x64` (`ldr` at 0xfb44).
  `ldm` at 0xfb4c reports r0 = `unknown` on the modeled known edges.
  Known return boundary (`ldm` at 0xfb4c): the modeled local edges end before the retained trailing region 0xfb50 to 0xfbac (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xfb50 to 0xfb54 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.
  Undecoded range: 0xfb58 to 0xfb5c (exclusive), 4 bytes `a8fdffff`. Native warning: `undecodable bytes at 0xfb58-0xfb5c; fall-through stops at this range`. This raw region remains in the full selected span and supplies no decoded effects.
  The decoded records from 0xfb5c to 0xfba8 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `L2CfgMQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xc804 to 0xdad0 (exclusive), 4812 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `5c09fbc167699f4eb54737aeaff7b1f424d6fefe8789a309483cb6d10223cc8a`.
  The native facts retain 56 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0xc808).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0xc814).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0xc818).
  Four-byte store: target `sp -0x20`, value `r2`, base register `r11`, raw displacement `-0x1c` (`str` at 0xc81c).
  Four-byte store: target `sp -0x24`, value `0x2000`, base register `r11`, raw displacement `-0x20` (`str` at 0xc824).
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0xd75c`, linked address 0xd75c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xc828, `ldr` at 0xc83c, `ldr` at 0xc8b0.
  Warning class: 5 native records report `ELF literal at linked 0xd75c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xc828, `ldr` at 0xc83c, `ldr` at 0xc8b0.
  Grouped accesses: 83 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xc82c, `ldr` at 0xc860, `ldr` at 0xc86c.
  Branch (`bne` at 0xc834): taken to 0xc858 when `? != 0x0`, falls through to 0xc838 when `? == 0x0`.
  PC-relative loads: 7 4-byte records report target expression `ELF load bias (unknown) +0xd760`, linked address 0xd760, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xc838, `ldr` at 0xc85c, `ldr` at 0xc894.
  Warning class: 7 native records report `ELF literal at linked 0xd760 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xc838, `ldr` at 0xc85c, `ldr` at 0xc894.
  Grouped accesses: 3 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xc840, `ldr` at 0xc8a8, `ldr` at 0xc8d4.
  Four-byte store: target `(unknown effective address)`, value `0x2000`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xc844).
  Call (`bl` at 0xc84c): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`0x2000`, r1=`r1`, r2=`unknown`, r3=`0x2000`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x3514)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]` (`str` at 0xc854).
  PC-relative loads: 108 4-byte records report target expression `ELF load bias (unknown) +0xd764`, linked address 0xd764, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xc858, `ldr` at 0xc868, `ldr` at 0xc8f4.
  Warning class: 108 native records report `ELF literal at linked 0xd764 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xc858, `ldr` at 0xc868, `ldr` at 0xc8f4.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xc864).
  Byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`strb` at 0xc874).
  Four-byte store: target `sp -0x30`, value `0x0`, base register `r11`, raw displacement `-0x2c` (`str` at 0xc87c).
  Grouped accesses: 5 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xc880, `ldr` at 0xc970, `ldr` at 0xd9f4.
  Four-byte store: target `sp -0x28`, value `[sp-24]`, base register `r11`, raw displacement `-0x24` (`str` at 0xc884).
  Grouped accesses: 18 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xc888, `ldr` at 0xc928, `ldr` at 0xc94c.
  Branch (`beq` at 0xc890): taken to 0xdabc when `Z == 1`, falls through to 0xc894 when `not (Z == 1)`.
  Call (`bl` at 0xc89c): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0xc8bc): taken to 0xc91c when `(ret(linked ELF 0x35d4) + [sp-36]) <= ? (unsigned, 32-bit)`, falls through to 0xc8c0 when `(ret(linked ELF 0x35d4) + [sp-36]) > ? (unsigned, 32-bit)`.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r0`, raw displacement `+0x0`, raw addressing form `[r0]` (`str` at 0xc8dc).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]`, including `ldr` at 0xc8e0, `ldr` at 0xcce0.
  Call (`bl` at 0xc8e8): (unknown callee), target 0x364c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x364c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x364c)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]` (`str` at 0xc8f0).
  Call (`bl` at 0xc900): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 31 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]`, including `str` at 0xc918, `str` at 0xc9b0, `str` at 0xca00.
  Grouped accesses: 2 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0xc91c, `ldr` at 0xdaac.
  Branch (`beq` at 0xc924): taken to 0xc93c when `[sp-48] == 0x0`, falls through to 0xc928 when `[sp-48] != 0x0`.
  Four-byte load: target `[sp-40] +0x8`, base register `r3`, raw displacement `+0x8` (`ldr` at 0xc92c).
  Branch (`bne` at 0xc934): taken to 0xc93c when `[[sp-40]+8] != 0x0`, falls through to 0xc938 when `[[sp-40]+8] == 0x0`.
  Jump (`b` at 0xc938): target 0xdaa0.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xc93c, `ldr` at 0xca04.
  Branch (`bne` at 0xc948): taken to 0xc960 when `? != 0x0`, falls through to 0xc94c when `? == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd768`, base register `pc`, raw displacement `+0xe10`, linked address 0xd768, basis `ELF load-bias-relative` (`ldr` at 0xc950).
  Native warning (`ldr` at 0xc950): `ELF literal at linked 0xd768 has relocation, writable or unmapped bytes; value unknown`.
  Branch (`beq` at 0xc958): taken to 0xc960 when `[sp-40] == ?`, falls through to 0xc95c when `[sp-40] != ?`.
  Jump (`b` at 0xc95c): target 0xdaa0.
  Grouped accesses: 3 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xc960, `ldr` at 0xcd50, `ldr` at 0xcf50.
  Branch (`beq` at 0xc968): taken to 0xc9b4 when `[sp-32] == 0x0`, falls through to 0xc96c when `[sp-32] != 0x0`.
  Branch (`beq` at 0xc978): taken to 0xc9b4 when `[sp-40] == [sp-24]`, falls through to 0xc97c when `[sp-40] != [sp-24]`.
  Four-byte load: target `ELF load bias (unknown) +0xd76c`, base register `pc`, raw displacement `+0xde0`, linked address 0xd76c, basis `ELF load-bias-relative` (`ldr` at 0xc984).
  Native warning (`ldr` at 0xc984): `ELF literal at linked 0xd76c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xc988): (unknown callee), target 0x3658, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3658, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0xc998): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `[sp-40] +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0xc9b8).
  Call (`bl` at 0xc9bc): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, conditional `false`, r0=`[[sp-40]]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  PC-relative loads: 9 4-byte records report target expression `ELF load bias (unknown) +0xd82c`, linked address 0xd82c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xc9cc, `ldr` at 0xca70, `ldr` at 0xcb74.
  Warning class: 9 native records report `ELF literal at linked 0xd82c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xc9cc, `ldr` at 0xca70, `ldr` at 0xcb74.
  Four-byte load: target `ELF load bias (unknown) +0xd770`, base register `pc`, raw displacement `+0xd98`, linked address 0xd770, basis `ELF load-bias-relative` (`ldr` at 0xc9d0).
  Native warning (`ldr` at 0xc9d0): `ELF literal at linked 0xd770 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xc9d8): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x34f0)`.
  Call (`bl` at 0xc9e8): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xca10): taken to 0xcbb0 when `? == 0x0`, falls through to 0xca14 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd774`, base register `pc`, raw displacement `+0xd4c`, linked address 0xd774, basis `ELF load-bias-relative` (`ldr` at 0xca20).
  Native warning (`ldr` at 0xca20): `ELF literal at linked 0xd774 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xd778`, base register `pc`, raw displacement `+0xd4c`, linked address 0xd778, basis `ELF load-bias-relative` (`ldr` at 0xca24).
  Native warning (`ldr` at 0xca24): `ELF literal at linked 0xd778 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `[sp-40] +0xc`, base register `r12`, raw displacement `+0xc` (`ldr` at 0xca28).
  Call (`bl` at 0xca2c): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[[sp-40]+12]`.
  Call (`bl` at 0xca3c): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-40] +0x10`, base register `r3`, raw displacement `+0x10` (`ldrb` at 0xca5c).
  Branch (`bne` at 0xca64): taken to 0xcaac when `[[sp-40]+16]:1 != 0xff`, falls through to 0xca68 when `[[sp-40]+16]:1 == 0xff`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xd77c`, linked address 0xd77c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xca74, `ldr` at 0xcb78.
  Warning class: 2 native records report `ELF literal at linked 0xd77c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xca74, `ldr` at 0xcb78.
  Four-byte load: target `ELF load bias (unknown) +0xd780`, base register `pc`, raw displacement `+0xd00`, linked address 0xd780, basis `ELF load-bias-relative` (`ldr` at 0xca78).
  Native warning (`ldr` at 0xca78): `ELF literal at linked 0xd780 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xca7c): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xca8c): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0xcaa8): target 0xdaa0.
  Four-byte load: target `ELF load bias (unknown) +0xd784`, base register `pc`, raw displacement `+0xcd0`, linked address 0xd784, basis `ELF load-bias-relative` (`ldr` at 0xcaac).
  Byte store: target `sp -0x13c`, value `0x0`, base register `r1`, raw displacement `+0x0` (`strb` at 0xcab8).
  Grouped accesses: 2 4-byte store records with target `sp -0x3c`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x38`, including `str` at 0xcac0, `str` at 0xda04.
  Grouped accesses: 38 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0xcac4, `ldr` at 0xcad8, `ldr` at 0xcb38.
  Branch (`bgt` at 0xcacc): taken to 0xcb50 when `gt flags condition (operands unknown)`, falls through to 0xcad0 when `not (gt flags condition (operands unknown))`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]` (`ldrb` at 0xcae4).
  Branch (`beq` at 0xcaec): taken to 0xcb40 when `Z == 1`, falls through to 0xcaf0 when `not (Z == 1)`.
  Call (`bl` at 0xcaf8): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`sp-316`, r1=`0x10`, r2=`[sp-40]`, r3=`sp-316`.
  Branch (`beq` at 0xcb04): taken to 0xcb18 when `ret(linked ELF 0x35d4) == 0x0`, falls through to 0xcb08 when `ret(linked ELF 0x35d4) != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd788`, base register `pc`, raw displacement `+0xc70`, linked address 0xd788, basis `ELF load-bias-relative` (`ldr` at 0xcb10).
  Native warning (`ldr` at 0xcb10): `ELF literal at linked 0xd788 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcb14): (unknown callee), target 0x3658, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3658, conditional `false`, r0=`sp-316`, r1=`unknown`, r2=`unknown`, r3=`sp-316`.
  Call (`bl` at 0xcb24): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`sp-316`, r1=`unknown`, r2=`unknown`, r3=`sp-316`.
  Four-byte load: target `ELF load bias (unknown) +0xd78c`, base register `pc`, raw displacement `+0xc50`, linked address 0xd78c, basis `ELF load-bias-relative` (`ldr` at 0xcb34).
  Native warning (`ldr` at 0xcb34): `ELF literal at linked 0xd78c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcb3c): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`(sp-316 + ret(linked ELF 0x35d4))`, r1=`unknown`, r2=`[sp-60]`, r3=`(sp-316 + ret(linked ELF 0x35d4))`.
  Grouped accesses: 2 4-byte store records with target `sp -0x3c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x38`, including `str` at 0xcb48, `str` at 0xcd4c.
  Jump (`b` at 0xcb4c): target 0xcac4.
  Call (`bl` at 0xcb58): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`sp-316`, r1=`unknown`, r2=`unknown`, r3=`sp-316`.
  Branch (`beq` at 0xcb64): taken to 0xdaa0 when `ret(linked ELF 0x35d4) == 0x0`, falls through to 0xcb68 when `ret(linked ELF 0x35d4) != 0x0`.
  Call (`bl` at 0xcb80): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`sp-316`.
  Call (`bl` at 0xcb90): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0xcbac): target 0xdaa0.
  PC-relative loads: 16 4-byte records report target expression `ELF load bias (unknown) +0xd818`, linked address 0xd818, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xcbbc, `ldr` at 0xcce4, `ldr` at 0xcd9c.
  Warning class: 16 native records report `ELF literal at linked 0xd818 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xcbbc, `ldr` at 0xcce4, `ldr` at 0xcd9c.
  Four-byte load: target `ELF load bias (unknown) +0xd790`, base register `pc`, raw displacement `+0xbc8`, linked address 0xd790, basis `ELF load-bias-relative` (`ldr` at 0xcbc0).
  Native warning (`ldr` at 0xcbc0): `ELF literal at linked 0xd790 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `[sp-40] +0x118`, base register `r12`, raw displacement `+0x118` (`ldr` at 0xcbc4).
  Call (`bl` at 0xcbc8): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[[sp-40]+280]`.
  Call (`bl` at 0xcbd8): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-40] +0x11c`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xcc00).
  Branch (`beq` at 0xcc08): taken to 0xcc2c when `[[sp-40]+284]:1 == 0x0`, falls through to 0xcc0c when `[[sp-40]+284]:1 != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd794`, base register `pc`, raw displacement `+0xb6c`, linked address 0xd794, basis `ELF load-bias-relative` (`ldr` at 0xcc20).
  Native warning (`ldr` at 0xcc20): `ELF literal at linked 0xd794 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcc28): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]+284`.
  Call (`bl` at 0xcc38): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-40] +0x130`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xcc60).
  Branch (`beq` at 0xcc68): taken to 0xcc8c when `[[sp-40]+304]:1 == 0x0`, falls through to 0xcc6c when `[[sp-40]+304]:1 != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd798`, base register `pc`, raw displacement `+0xb10`, linked address 0xd798, basis `ELF load-bias-relative` (`ldr` at 0xcc80).
  Native warning (`ldr` at 0xcc80): `ELF literal at linked 0xd798 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcc88): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]+304`.
  Call (`bl` at 0xcc98): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `[sp-40] +0x144`, base register `r2`, raw displacement `+0x0`, including `ldrb` at 0xccbc, `ldrb` at 0xccd8.
  Branch (`beq` at 0xccc8): taken to 0xccf0 when `? == 0x0`, falls through to 0xcccc when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd79c`, base register `pc`, raw displacement `+0xaac`, linked address 0xd79c, basis `ELF load-bias-relative` (`ldr` at 0xcce8).
  Native warning (`ldr` at 0xcce8): `ELF literal at linked 0xd79c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xccec): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xccfc): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x34`, value `0x0`, base register `r11`, raw displacement `-0x30` (`str` at 0xcd1c).
  Grouped accesses: 4 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0xcd20, `ldr` at 0xcd2c, `ldr` at 0xcda4.
  Branch (`bgt` at 0xcd28): taken to 0xd9f0 when `gt flags condition (operands unknown)`, falls through to 0xcd2c when `not (gt flags condition (operands unknown))`.
  Native warning (`rsb` at 0xcd38): `rsb: unsupported effects invalidated`.
  Branch (`beq` at 0xcd58): taken to 0xcd94 when `[sp-32] == 0x0`, falls through to 0xcd5c when `[sp-32] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7a0`, base register `pc`, raw displacement `+0xa34`, linked address 0xd7a0, basis `ELF load-bias-relative` (`ldr` at 0xcd64).
  Native warning (`ldr` at 0xcd64): `ELF literal at linked 0xd7a0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcd68): (unknown callee), target 0x3658, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3658, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcd78): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xd7a4`, base register `pc`, raw displacement `+0x9fc`, linked address 0xd7a4, basis `ELF load-bias-relative` (`ldr` at 0xcda0).
  Native warning (`ldr` at 0xcda0): `ELF literal at linked 0xd7a4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcda8): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-52]`.
  Call (`bl` at 0xcdb8): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte load records with target `[sp-60] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0xcdd8, `ldr` at 0xceb8.
  Branch (`bne` at 0xcde0): taken to 0xcde8 when `[[sp-60]] != 0x0`, falls through to 0xcde4 when `[[sp-60]] == 0x0`.
  Jump (`b` at 0xcde4): target 0xd9e0.
  Four-byte load: target `ELF load bias (unknown) +0xd7a8`, base register `pc`, raw displacement `+0x9a8`, linked address 0xd7a8, basis `ELF load-bias-relative` (`ldr` at 0xcdf8).
  Native warning (`ldr` at 0xcdf8): `ELF literal at linked 0xd7a8 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `[sp-60] +0x0`, base register `r12`, raw displacement `+0x0` (`ldr` at 0xcdfc).
  Call (`bl` at 0xce00): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[[sp-60]]`.
  Call (`bl` at 0xce10): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xd7ac`, base register `pc`, raw displacement `+0x968`, linked address 0xd7ac, basis `ELF load-bias-relative` (`ldr` at 0xce3c).
  Native warning (`ldr` at 0xce3c): `ELF literal at linked 0xd7ac has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `[sp-60] +0xc`, base register `r12`, raw displacement `+0xc` (`ldr` at 0xce40).
  Call (`bl` at 0xce44): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[[sp-60]+12]`.
  Call (`bl` at 0xce54): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xd7b0`, base register `pc`, raw displacement `+0x928`, linked address 0xd7b0, basis `ELF load-bias-relative` (`ldr` at 0xce80).
  Native warning (`ldr` at 0xce80): `ELF literal at linked 0xd7b0 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `[sp-60] +0x10`, base register `r12`, raw displacement `+0x10` (`ldr` at 0xce84).
  Call (`bl` at 0xce88): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[[sp-60]+16]`.
  Call (`bl` at 0xce98): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xcec0): taken to 0xcec8 when `[[sp-60]] == 0x2`, falls through to 0xcec4 when `[[sp-60]] != 0x2`.
  Jump (`b` at 0xcec4): target 0xd9e0.
  Four-byte store: target `sp -0x38`, value `0x0`, base register `r11`, raw displacement `-0x34` (`str` at 0xcecc).
  Four-byte load: target `[sp-60] +0x14`, base register `r3`, raw displacement `+0x14` (`ldr` at 0xced4).
  Grouped accesses: 40 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0xced8, `ldr` at 0xcee8, `ldr` at 0xcf1c.
  Branch (`ble` at 0xcee0): taken to 0xd9e0 when `le flags condition (operands unknown)`, falls through to 0xcee4 when `not (le flags condition (operands unknown))`.
  Grouped accesses: 23 4-byte load records with target `[sp-60] +0x18`, base register `r1`, raw displacement `+0x18`, including `ldr` at 0xcf04, `ldr` at 0xcf38, `ldr` at 0xd044.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x1`, raw addressing form `[r3, #1]` (`ldrb` at 0xcf0c).
  Branch (`bne` at 0xcf14): taken to 0xcf50 when `Z == 0`, falls through to 0xcf18 when `not (Z == 0)`.
  Grouped accesses: 3 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0xc`, raw addressing form `[r3, #0xc]`, including `ldrb` at 0xcf40, `ldrb` at 0xd66c, `ldrb` at 0xd6c8.
  Branch (`bne` at 0xcf48): taken to 0xcf50 when `Z == 0`, falls through to 0xcf4c when `not (Z == 0)`.
  Jump (`b` at 0xcf4c): target 0xd9d0.
  Branch (`beq` at 0xcf58): taken to 0xcf94 when `[sp-32] == 0x0`, falls through to 0xcf5c when `[sp-32] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7b4`, base register `pc`, raw displacement `+0x848`, linked address 0xd7b4, basis `ELF load-bias-relative` (`ldr` at 0xcf64).
  Native warning (`ldr` at 0xcf64): `ELF literal at linked 0xd7b4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xcf68): (unknown callee), target 0x3658, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3658, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcf78): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x140`, value `unknown`, base register `r11`, raw displacement `-0x13c` (`str` at 0xcf98).
  Branch (`beq` at 0xcfa4): taken to 0xcfb4 when `Z == 1`, falls through to 0xcfa8 when `not (Z == 1)`.
  Four-byte load: target `ELF load bias (unknown) +0xd7b8`, base register `pc`, raw displacement `+0x808`, linked address 0xd7b8, basis `ELF load-bias-relative` (`ldr` at 0xcfa8).
  Native warning (`ldr` at 0xcfa8): `ELF literal at linked 0xd7b8 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x144`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x140`, including `str` at 0xcfac, `str` at 0xcfb8.
  Four-byte load: target `ELF load bias (unknown) +0xd7bc`, base register `pc`, raw displacement `+0x800`, linked address 0xd7bc, basis `ELF load-bias-relative` (`ldr` at 0xcfb4).
  Native warning (`ldr` at 0xcfb4): `ELF literal at linked 0xd7bc has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x140`, base register `r11`, raw displacement `-0x13c` (`ldr` at 0xcfbc).
  Four-byte load: target `[sp-320] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xcfc0).
  Four-byte load: target `sp -0x144`, base register `r11`, raw displacement `-0x140` (`ldr` at 0xcfc8).
  Call (`bl` at 0xcfd0): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-320]]`, r1=`unknown`, r2=`[sp-324]`, r3=`unknown`.
  Call (`bl` at 0xcfe0): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x148`, value `unknown`, base register `r11`, raw displacement `-0x144` (`str` at 0xd000).
  Branch (`beq` at 0xd00c): taken to 0xd01c when `[sp-56] == 0x0`, falls through to 0xd010 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7c0`, base register `pc`, raw displacement `+0x7a8`, linked address 0xd7c0, basis `ELF load-bias-relative` (`ldr` at 0xd010).
  Native warning (`ldr` at 0xd010): `ELF literal at linked 0xd7c0 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x14c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x148`, including `str` at 0xd014, `str` at 0xd020.
  Four-byte load: target `ELF load bias (unknown) +0xd7c4`, base register `pc`, raw displacement `+0x7a0`, linked address 0xd7c4, basis `ELF load-bias-relative` (`ldr` at 0xd01c).
  Native warning (`ldr` at 0xd01c): `ELF literal at linked 0xd7c4 has relocation, writable or unmapped bytes; value unknown`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x3`, raw addressing form `[r3, #3]` (`ldrb` at 0xd04c).
  Four-byte load: target `sp -0x148`, base register `r11`, raw displacement `-0x144` (`ldr` at 0xd050).
  Four-byte load: target `[sp-328] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd054).
  Four-byte load: target `sp -0x14c`, base register `r11`, raw displacement `-0x148` (`ldr` at 0xd05c).
  Call (`bl` at 0xd060): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-328]]`, r1=`unknown`, r2=`[sp-332]`, r3=`unknown`.
  Call (`bl` at 0xd070): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x150`, value `unknown`, base register `r11`, raw displacement `-0x14c` (`str` at 0xd090).
  Branch (`beq` at 0xd09c): taken to 0xd0ac when `[sp-56] == 0x0`, falls through to 0xd0a0 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7c8`, base register `pc`, raw displacement `+0x720`, linked address 0xd7c8, basis `ELF load-bias-relative` (`ldr` at 0xd0a0).
  Native warning (`ldr` at 0xd0a0): `ELF literal at linked 0xd7c8 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x154`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x150`, including `str` at 0xd0a4, `str` at 0xd0b0.
  Four-byte load: target `ELF load bias (unknown) +0xd7cc`, base register `pc`, raw displacement `+0x718`, linked address 0xd7cc, basis `ELF load-bias-relative` (`ldr` at 0xd0ac).
  Native warning (`ldr` at 0xd0ac): `ELF literal at linked 0xd7cc has relocation, writable or unmapped bytes; value unknown`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]` (`ldrb` at 0xd0dc).
  Four-byte load: target `sp -0x150`, base register `r11`, raw displacement `-0x14c` (`ldr` at 0xd0e0).
  Four-byte load: target `[sp-336] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xd0e4).
  Four-byte load: target `sp -0x154`, base register `r11`, raw displacement `-0x150` (`ldr` at 0xd0ec).
  Call (`bl` at 0xd0f0): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-336]]`, r1=`unknown`, r2=`[sp-340]`, r3=`unknown`.
  Call (`bl` at 0xd100): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x158`, value `unknown`, base register `r11`, raw displacement `-0x154` (`str` at 0xd120).
  Branch (`beq` at 0xd12c): taken to 0xd13c when `[sp-56] == 0x0`, falls through to 0xd130 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7d0`, base register `pc`, raw displacement `+0x698`, linked address 0xd7d0, basis `ELF load-bias-relative` (`ldr` at 0xd130).
  Native warning (`ldr` at 0xd130): `ELF literal at linked 0xd7d0 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x15c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x158`, including `str` at 0xd134, `str` at 0xd140.
  Four-byte load: target `ELF load bias (unknown) +0xd7d4`, base register `pc`, raw displacement `+0x690`, linked address 0xd7d4, basis `ELF load-bias-relative` (`ldr` at 0xd13c).
  Native warning (`ldr` at 0xd13c): `ELF literal at linked 0xd7d4 has relocation, writable or unmapped bytes; value unknown`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x5`, raw addressing form `[r3, #5]` (`ldrb` at 0xd16c).
  Four-byte load: target `sp -0x158`, base register `r11`, raw displacement `-0x154` (`ldr` at 0xd170).
  Four-byte load: target `[sp-344] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd174).
  Four-byte load: target `sp -0x15c`, base register `r11`, raw displacement `-0x158` (`ldr` at 0xd17c).
  Call (`bl` at 0xd180): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-344]]`, r1=`unknown`, r2=`[sp-348]`, r3=`unknown`.
  Call (`bl` at 0xd190): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x8`, raw addressing form `[r3, #8]`, including `ldrb` at 0xd1d4, `ldrb` at 0xd230.
  Branch (`beq` at 0xd1dc): taken to 0xd248 when `? == 0x0`, falls through to 0xd1e0 when `? != 0x0`.
  Four-byte store: target `sp -0x160`, value `unknown`, base register `r11`, raw displacement `-0x15c` (`str` at 0xd1e4).
  Branch (`beq` at 0xd1f0): taken to 0xd200 when `[sp-56] == 0x0`, falls through to 0xd1f4 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7d8`, base register `pc`, raw displacement `+0x5dc`, linked address 0xd7d8, basis `ELF load-bias-relative` (`ldr` at 0xd1f4).
  Native warning (`ldr` at 0xd1f4): `ELF literal at linked 0xd7d8 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x164`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x160`, including `str` at 0xd1f8, `str` at 0xd204.
  Four-byte load: target `ELF load bias (unknown) +0xd7dc`, base register `pc`, raw displacement `+0x5d4`, linked address 0xd7dc, basis `ELF load-bias-relative` (`ldr` at 0xd200).
  Native warning (`ldr` at 0xd200): `ELF literal at linked 0xd7dc has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x160`, base register `r11`, raw displacement `-0x15c` (`ldr` at 0xd234).
  Four-byte load: target `[sp-352] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xd238).
  Four-byte load: target `sp -0x164`, base register `r11`, raw displacement `-0x160` (`ldr` at 0xd240).
  Call (`bl` at 0xd244): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-352]]`, r1=`unknown`, r2=`[sp-356]`, r3=`unknown`.
  Call (`bl` at 0xd254): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x7`, raw addressing form `[r3, #7]`, including `ldrb` at 0xd298, `ldrb` at 0xd2f4.
  Branch (`beq` at 0xd2a0): taken to 0xd30c when `? == 0x0`, falls through to 0xd2a4 when `? != 0x0`.
  Four-byte store: target `sp -0x168`, value `unknown`, base register `r11`, raw displacement `-0x164` (`str` at 0xd2a8).
  Branch (`beq` at 0xd2b4): taken to 0xd2c4 when `[sp-56] == 0x0`, falls through to 0xd2b8 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7e0`, base register `pc`, raw displacement `+0x520`, linked address 0xd7e0, basis `ELF load-bias-relative` (`ldr` at 0xd2b8).
  Native warning (`ldr` at 0xd2b8): `ELF literal at linked 0xd7e0 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x16c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x168`, including `str` at 0xd2bc, `str` at 0xd2c8.
  Four-byte load: target `ELF load bias (unknown) +0xd7e4`, base register `pc`, raw displacement `+0x518`, linked address 0xd7e4, basis `ELF load-bias-relative` (`ldr` at 0xd2c4).
  Native warning (`ldr` at 0xd2c4): `ELF literal at linked 0xd7e4 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x168`, base register `r11`, raw displacement `-0x164` (`ldr` at 0xd2f8).
  Four-byte load: target `[sp-360] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd2fc).
  Four-byte load: target `sp -0x16c`, base register `r11`, raw displacement `-0x168` (`ldr` at 0xd304).
  Call (`bl` at 0xd308): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-360]]`, r1=`unknown`, r2=`[sp-364]`, r3=`unknown`.
  Call (`bl` at 0xd318): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x9`, raw addressing form `[r3, #9]`, including `ldrb` at 0xd35c, `ldrb` at 0xd3b8.
  Branch (`beq` at 0xd364): taken to 0xd3d0 when `? == 0x0`, falls through to 0xd368 when `? != 0x0`.
  Four-byte store: target `sp -0x170`, value `unknown`, base register `r11`, raw displacement `-0x16c` (`str` at 0xd36c).
  Branch (`beq` at 0xd378): taken to 0xd388 when `[sp-56] == 0x0`, falls through to 0xd37c when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7e8`, base register `pc`, raw displacement `+0x464`, linked address 0xd7e8, basis `ELF load-bias-relative` (`ldr` at 0xd37c).
  Native warning (`ldr` at 0xd37c): `ELF literal at linked 0xd7e8 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x174`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x170`, including `str` at 0xd380, `str` at 0xd38c.
  Four-byte load: target `ELF load bias (unknown) +0xd7ec`, base register `pc`, raw displacement `+0x45c`, linked address 0xd7ec, basis `ELF load-bias-relative` (`ldr` at 0xd388).
  Native warning (`ldr` at 0xd388): `ELF literal at linked 0xd7ec has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x170`, base register `r11`, raw displacement `-0x16c` (`ldr` at 0xd3bc).
  Four-byte load: target `[sp-368] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xd3c0).
  Four-byte load: target `sp -0x174`, base register `r11`, raw displacement `-0x170` (`ldr` at 0xd3c8).
  Call (`bl` at 0xd3cc): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-368]]`, r1=`unknown`, r2=`[sp-372]`, r3=`unknown`.
  Call (`bl` at 0xd3dc): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0xa`, raw addressing form `[r3, #0xa]`, including `ldrb` at 0xd420, `ldrb` at 0xd47c.
  Branch (`beq` at 0xd428): taken to 0xd494 when `? == 0x0`, falls through to 0xd42c when `? != 0x0`.
  Four-byte store: target `sp -0x178`, value `unknown`, base register `r11`, raw displacement `-0x174` (`str` at 0xd430).
  Branch (`beq` at 0xd43c): taken to 0xd44c when `[sp-56] == 0x0`, falls through to 0xd440 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7f0`, base register `pc`, raw displacement `+0x3a8`, linked address 0xd7f0, basis `ELF load-bias-relative` (`ldr` at 0xd440).
  Native warning (`ldr` at 0xd440): `ELF literal at linked 0xd7f0 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x17c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x178`, including `str` at 0xd444, `str` at 0xd450.
  Four-byte load: target `ELF load bias (unknown) +0xd7f4`, base register `pc`, raw displacement `+0x3a0`, linked address 0xd7f4, basis `ELF load-bias-relative` (`ldr` at 0xd44c).
  Native warning (`ldr` at 0xd44c): `ELF literal at linked 0xd7f4 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x178`, base register `r11`, raw displacement `-0x174` (`ldr` at 0xd480).
  Four-byte load: target `[sp-376] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd484).
  Four-byte load: target `sp -0x17c`, base register `r11`, raw displacement `-0x178` (`ldr` at 0xd48c).
  Call (`bl` at 0xd490): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-376]]`, r1=`unknown`, r2=`[sp-380]`, r3=`unknown`.
  Call (`bl` at 0xd4a0): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0xb`, raw addressing form `[r3, #0xb]`, including `ldrb` at 0xd4e4, `ldrb` at 0xd540.
  Branch (`beq` at 0xd4ec): taken to 0xd558 when `? == 0x0`, falls through to 0xd4f0 when `? != 0x0`.
  Four-byte store: target `sp -0x180`, value `unknown`, base register `r11`, raw displacement `-0x17c` (`str` at 0xd4f4).
  Branch (`beq` at 0xd500): taken to 0xd510 when `[sp-56] == 0x0`, falls through to 0xd504 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd7f8`, base register `pc`, raw displacement `+0x2ec`, linked address 0xd7f8, basis `ELF load-bias-relative` (`ldr` at 0xd504).
  Native warning (`ldr` at 0xd504): `ELF literal at linked 0xd7f8 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x184`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x180`, including `str` at 0xd508, `str` at 0xd514.
  Four-byte load: target `ELF load bias (unknown) +0xd7fc`, base register `pc`, raw displacement `+0x2e4`, linked address 0xd7fc, basis `ELF load-bias-relative` (`ldr` at 0xd510).
  Native warning (`ldr` at 0xd510): `ELF literal at linked 0xd7fc has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x180`, base register `r11`, raw displacement `-0x17c` (`ldr` at 0xd544).
  Four-byte load: target `[sp-384] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xd548).
  Four-byte load: target `sp -0x184`, base register `r11`, raw displacement `-0x180` (`ldr` at 0xd550).
  Call (`bl` at 0xd554): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-384]]`, r1=`unknown`, r2=`[sp-388]`, r3=`unknown`.
  Call (`bl` at 0xd564): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x10`, raw addressing form `[r3, #0x10]`, including `ldr` at 0xd5a8, `ldr` at 0xd614.
  Branch (`beq` at 0xd5b0): taken to 0xd61c when `? == 0x0`, falls through to 0xd5b4 when `? != 0x0`.
  Four-byte store: target `sp -0x188`, value `unknown`, base register `r11`, raw displacement `-0x184` (`str` at 0xd5b8).
  Branch (`beq` at 0xd5c4): taken to 0xd5d4 when `[sp-56] == 0x0`, falls through to 0xd5c8 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd800`, base register `pc`, raw displacement `+0x230`, linked address 0xd800, basis `ELF load-bias-relative` (`ldr` at 0xd5c8).
  Native warning (`ldr` at 0xd5c8): `ELF literal at linked 0xd800 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x18c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x188`, including `str` at 0xd5cc, `str` at 0xd5d8.
  Four-byte load: target `ELF load bias (unknown) +0xd804`, base register `pc`, raw displacement `+0x228`, linked address 0xd804, basis `ELF load-bias-relative` (`ldr` at 0xd5d4).
  Native warning (`ldr` at 0xd5d4): `ELF literal at linked 0xd804 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x188`, base register `r11`, raw displacement `-0x184` (`ldr` at 0xd604).
  Four-byte load: target `[sp-392] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd608).
  Four-byte load: target `sp -0x18c`, base register `r11`, raw displacement `-0x188` (`ldr` at 0xd610).
  Call (`bl` at 0xd618): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-392]]`, r1=`unknown`, r2=`[sp-396]`, r3=`unknown`.
  Call (`bl` at 0xd628): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xd674): taken to 0xd6e0 when `? == 0x0`, falls through to 0xd678 when `? != 0x0`.
  Four-byte store: target `sp -0x190`, value `unknown`, base register `r11`, raw displacement `-0x18c` (`str` at 0xd67c).
  Branch (`beq` at 0xd688): taken to 0xd698 when `[sp-56] == 0x0`, falls through to 0xd68c when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd808`, base register `pc`, raw displacement `+0x174`, linked address 0xd808, basis `ELF load-bias-relative` (`ldr` at 0xd68c).
  Native warning (`ldr` at 0xd68c): `ELF literal at linked 0xd808 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x194`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x190`, including `str` at 0xd690, `str` at 0xd69c.
  Four-byte load: target `ELF load bias (unknown) +0xd80c`, base register `pc`, raw displacement `+0x16c`, linked address 0xd80c, basis `ELF load-bias-relative` (`ldr` at 0xd698).
  Native warning (`ldr` at 0xd698): `ELF literal at linked 0xd80c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x190`, base register `r11`, raw displacement `-0x18c` (`ldr` at 0xd6cc).
  Four-byte load: target `[sp-400] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xd6d0).
  Four-byte load: target `sp -0x194`, base register `r11`, raw displacement `-0x190` (`ldr` at 0xd6d8).
  Call (`bl` at 0xd6dc): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-400]]`, r1=`unknown`, r2=`[sp-404]`, r3=`unknown`.
  Call (`bl` at 0xd6ec): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x6`, raw addressing form `[r3, #6]`, including `ldrb` at 0xd730, `ldrb` at 0xd870.
  Branch (`beq` at 0xd738): taken to 0xd888 when `? == 0x0`, falls through to 0xd73c when `? != 0x0`.
  Four-byte store: target `sp -0x198`, value `unknown`, base register `r11`, raw displacement `-0x194` (`str` at 0xd740).
  Branch (`beq` at 0xd74c): taken to 0xd840 when `[sp-56] == 0x0`, falls through to 0xd750 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd810`, base register `pc`, raw displacement `+0xb8`, linked address 0xd810, basis `ELF load-bias-relative` (`ldr` at 0xd750).
  Native warning (`ldr` at 0xd750): `ELF literal at linked 0xd810 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x19c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x198`, including `str` at 0xd754, `str` at 0xd844.
  Jump (`b` at 0xd758): target 0xd848.
  The decoded records from 0xd75c to 0xd780 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.
  Undecoded range: 0xd784 to 0xd788 (exclusive), 4 bytes `d8feffff`. Native warning: `undecodable bytes at 0xd784-0xd788; fall-through stops at this range`. This raw region remains in the full selected span and supplies no decoded effects.
  Interior gap coverage (`b` at 0xd758): the native jump goes to 0xd848, beyond the retained undecoded range. The verified known local edges enter no gap, without classifying it as data or proving global nonexecution.
  The decoded records from 0xd788 to 0xd83c are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.
  Four-byte load: target `ELF load bias (unknown) +0xd814`, base register `pc`, raw displacement `-0x34`, linked address 0xd814, basis `ELF load-bias-relative` (`ldr` at 0xd840).
  Native warning (`ldr` at 0xd840): `ELF literal at linked 0xd814 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x198`, base register `r11`, raw displacement `-0x194` (`ldr` at 0xd874).
  Four-byte load: target `[sp-408] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd878).
  Four-byte load: target `sp -0x19c`, base register `r11`, raw displacement `-0x198` (`ldr` at 0xd880).
  Call (`bl` at 0xd884): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-408]]`, r1=`unknown`, r2=`[sp-412]`, r3=`unknown`.
  PC-relative loads: 15 4-byte records report target expression `ELF load bias (unknown) +0xd838`, linked address 0xd838, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xd888, `ldr` at 0xd88c, `ldr` at 0xd8a0.
  Warning class: 15 native records report `ELF literal at linked 0xd838 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xd888, `ldr` at 0xd88c, `ldr` at 0xd8a0.
  Call (`bl` at 0xd894): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x1a0`, value `unknown`, base register `r11`, raw displacement `-0x19c` (`str` at 0xd8b4).
  Branch (`beq` at 0xd8c0): taken to 0xd8d0 when `[sp-56] == 0x0`, falls through to 0xd8c4 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd81c`, base register `pc`, raw displacement `-0xb0`, linked address 0xd81c, basis `ELF load-bias-relative` (`ldr` at 0xd8c4).
  Native warning (`ldr` at 0xd8c4): `ELF literal at linked 0xd81c has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x1a4`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x1a0`, including `str` at 0xd8c8, `str` at 0xd8d4.
  Four-byte load: target `ELF load bias (unknown) +0xd820`, base register `pc`, raw displacement `-0xb8`, linked address 0xd820, basis `ELF load-bias-relative` (`ldr` at 0xd8d0).
  Native warning (`ldr` at 0xd8d0): `ELF literal at linked 0xd820 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x1a0`, base register `r11`, raw displacement `-0x19c` (`ldr` at 0xd904).
  Four-byte load: target `[sp-416] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xd908).
  Four-byte load: target `sp -0x1a4`, base register `r11`, raw displacement `-0x1a0` (`ldr` at 0xd910).
  Call (`bl` at 0xd914): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-416]]`, r1=`unknown`, r2=`[sp-420]`, r3=`unknown`.
  Call (`bl` at 0xd924): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x1a8`, value `unknown`, base register `r11`, raw displacement `-0x1a4` (`str` at 0xd944).
  Branch (`beq` at 0xd950): taken to 0xd960 when `[sp-56] == 0x0`, falls through to 0xd954 when `[sp-56] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd824`, base register `pc`, raw displacement `-0x138`, linked address 0xd824, basis `ELF load-bias-relative` (`ldr` at 0xd954).
  Native warning (`ldr` at 0xd954): `ELF literal at linked 0xd824 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x1ac`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x1a8`, including `str` at 0xd958, `str` at 0xd964.
  Four-byte load: target `ELF load bias (unknown) +0xd828`, base register `pc`, raw displacement `-0x140`, linked address 0xd828, basis `ELF load-bias-relative` (`ldr` at 0xd960).
  Native warning (`ldr` at 0xd960): `ELF literal at linked 0xd828 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `sp -0x1a8`, base register `r11`, raw displacement `-0x1a4` (`ldr` at 0xd994).
  Four-byte load: target `[sp-424] +0x0`, base register `r1`, raw displacement `+0x0` (`ldr` at 0xd998).
  Four-byte load: target `sp -0x1ac`, base register `r11`, raw displacement `-0x1a8` (`ldr` at 0xd9a0).
  Call (`bl` at 0xd9a4): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`[[sp-424]]`, r1=`unknown`, r2=`[sp-428]`, r3=`unknown`.
  Call (`bl` at 0xd9b4): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x38`, value `unknown`, base register `r11`, raw displacement `-0x34` (`str` at 0xd9d8).
  Jump (`b` at 0xd9dc): target 0xced0.
  Four-byte store: target `sp -0x34`, value `[sp-52]+1`, base register `r11`, raw displacement `-0x30` (`str` at 0xd9e8).
  Jump (`b` at 0xd9ec): target 0xcd20.
  Branch (`bne` at 0xd9fc): taken to 0xdaa0 when `[sp-40] != [sp-24]`, falls through to 0xda00 when `[sp-40] == [sp-24]`.
  Branch (`bgt` at 0xda10): taken to 0xdaa0 when `gt flags condition (operands unknown)`, falls through to 0xda14 when `not (gt flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `[sp-24] +0x5b4`, base register `r1`, raw displacement `+0x5b4`, including `ldr` at 0xda20, `ldr` at 0xda44.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]`, including `ldr` at 0xda28, `ldr` at 0xda4c.
  Branch (`beq` at 0xda30): taken to 0xda68 when `Z == 1`, falls through to 0xda34 when `not (Z == 1)`.
  Four-byte store: target `sp -0x1b0`, value `unknown`, base register `sp`, raw displacement `+0x0` (`str` at 0xda50).
  Four-byte load: target `(unknown effective address)`, base register `r0`, raw displacement `+0x0`, raw addressing form `[r0]` (`ldr` at 0xda54).
  Four-byte load: target `ELF load bias (unknown) +0xd830`, base register `pc`, raw displacement `-0x230`, linked address 0xd830, basis `ELF load-bias-relative` (`ldr` at 0xda58).
  Native warning (`ldr` at 0xda58): `ELF literal at linked 0xd830 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xd834`, base register `pc`, raw displacement `-0x230`, linked address 0xd834, basis `ELF load-bias-relative` (`ldr` at 0xda5c).
  Native warning (`ldr` at 0xda5c): `ELF literal at linked 0xd834 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xda64): (unknown callee), target 0x379c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x379c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xda74): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x3c`, value `[sp-60]+1`, base register `r11`, raw displacement `-0x38` (`str` at 0xda98).
  Jump (`b` at 0xda9c): target 0xda08.
  Four-byte load: target `[sp-40] +0x114`, base register `r3`, raw displacement `+0x114` (`ldr` at 0xdaa4).
  Four-byte store: target `sp -0x28`, value `[[sp-40]+276]`, base register `r11`, raw displacement `-0x24` (`str` at 0xdaa8).
  Four-byte store: target `sp -0x30`, value `[sp-48]+1`, base register `r11`, raw displacement `-0x2c` (`str` at 0xdab4).
  Jump (`b` at 0xdab8): target 0xc888.
  Four-byte load: target `ELF load bias (unknown) +0xd83c`, base register `pc`, raw displacement `-0x288`, linked address 0xd83c, basis `ELF load-bias-relative` (`ldr` at 0xdabc).
  Native warning (`ldr` at 0xdabc): `ELF literal at linked 0xd83c has relocation, writable or unmapped bytes; value unknown`.
  The native return (`ldm` at 0xdacc) retains r0 = `unknown`.

- `L2CfgMCfgL2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xa2ac to 0xb818 (exclusive), 5484 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `45b5d2db484d4a0449fe9d82ed99d802efc727ae9285caccca49477095eec2b1`.
  The native facts retain 37 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0xa2b0).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0xa2bc).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0xa2c0).
  Grouped accesses: 5 4-byte store records with target `sp -0x20`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x1c`, including `str` at 0xa2c8, `str` at 0xa558, `str` at 0xa570.
  Four-byte store: target `sp -0x24`, value `0x0`, base register `r11`, raw displacement `-0x20` (`str` at 0xa2d0).
  Grouped accesses: 8 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xa2d4, `ldr` at 0xa2e0, `ldr` at 0xa2f4.
  Byte load: target `r1 +0x20`, base register `r3`, raw displacement `+0x20` (`ldrb` at 0xa2d8).
  Four-byte store: target `sp -0x28`, value `[r1+32]:1`, base register `r11`, raw displacement `-0x24` (`str` at 0xa2dc).
  Byte load: target `r1 +0x2c`, base register `r3`, raw displacement `+0x2c` (`ldrb` at 0xa2e4).
  Branch (`bne` at 0xa2f0): taken to 0xa304 when `? != 0x0`, falls through to 0xa2f4 when `? == 0x0`.
  Four-byte load: target `r1 +0x30`, base register `r3`, raw displacement `+0x30` (`ldr` at 0xa2f8).
  Four-byte store: target `sp -0x68`, value `[r1+48]`, base register `r11`, raw displacement `-0x64` (`str` at 0xa2fc).
  Call (`bl` at 0xa30c): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`r1`, r1=`0x1`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x68`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x64` (`str` at 0xa310).
  Four-byte load: target `sp -0x68`, base register `r11`, raw displacement `-0x64` (`ldr` at 0xa314).
  Four-byte store: target `sp -0x2c`, value `[sp-104]`, base register `r11`, raw displacement `-0x28` (`str` at 0xa318).
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0xa31c).
  Branch (`bne` at 0xa324): taken to 0xa358 when `[sp-104] != 0x1`, falls through to 0xa328 when `[sp-104] == 0x1`.
  Four-byte load: target `ELF load bias (unknown) +0xb28c`, base register `pc`, raw displacement `+0xf5c`, linked address 0xb28c, basis `ELF load-bias-relative` (`ldr` at 0xa328).
  Native warning (`ldr` at 0xa328): `ELF literal at linked 0xb28c has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 15 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xa32c, `ldr` at 0xa33c, `ldr` at 0xa3b0.
  Branch (`beq` at 0xa334): taken to 0xa348 when `? == 0x0`, falls through to 0xa338 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xb290`, base register `pc`, raw displacement `+0xf50`, linked address 0xb290, basis `ELF load-bias-relative` (`ldr` at 0xa338).
  Native warning (`ldr` at 0xa338): `ELF literal at linked 0xb290 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xb294`, base register `pc`, raw displacement `+0xf4c`, linked address 0xb294, basis `ELF load-bias-relative` (`ldr` at 0xa340).
  Native warning (`ldr` at 0xa340): `ELF literal at linked 0xb294 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa344): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 6 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xa348, `ldr` at 0xa4e8, `ldr` at 0xa538.
  Call (`bl` at 0xa350): (unknown callee), target 0x346c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x346c, conditional `false`, r0=`[sp-24]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0xa354): target 0xb810.
  Four-byte store: target `sp -0x30`, value `0x2`, base register `r11`, raw displacement `-0x2c` (`str` at 0xa35c).
  Grouped accesses: 5 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0xa360, `ldr` at 0xa374, `ldr` at 0xa39c.
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0xa364).
  Branch (`bge` at 0xa36c): taken to 0xb804 when `ge flags condition (operands unknown)`, falls through to 0xa370 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldrb` at 0xa388, `ldrb` at 0xa428.
  Branch (`bne` at 0xa394): taken to 0xa3bc when `? != 0x0`, falls through to 0xa398 when `? == 0x0`.
  Four-byte store: target `sp -0x6c`, value `unknown`, base register `r11`, raw displacement `-0x68` (`str` at 0xa3b4).
  Call (`bl` at 0xa3c4): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Four-byte store: target `sp -0x6c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x68` (`str` at 0xa3c8).
  Four-byte load: target `sp -0x6c`, base register `r11`, raw displacement `-0x68` (`ldr` at 0xa3cc).
  Four-byte store: target `sp -0x5c`, value `unknown`, base register `r11`, raw displacement `-0x58` (`str` at 0xa3d0).
  Branch (`bne` at 0xa3d8): taken to 0xa3e0 when `Z == 0`, falls through to 0xa3dc when `not (Z == 0)`.
  Jump (`b` at 0xa3dc): target 0xb7f4.
  Grouped accesses: 7 4-byte load records with target `sp -0x5c`, base register `r11`, raw displacement `-0x58`, including `ldr` at 0xa3e0, `ldr` at 0xa400, `ldr` at 0xa410.
  Call (`bl` at 0xa3e4): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xa3f4): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`ret(linked ELF 0x35d4)+1`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x35d4)+1`.
  Call (`bl` at 0xa404): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`ret(linked ELF 0x3514)`, r1=`[sp-92]`, r2=`unknown`, r3=`ret(linked ELF 0x3514)`.
  Four-byte store: target `sp -0x5c`, value `ret(linked ELF 0x37d8)`, base register `r11`, raw displacement `-0x58` (`str` at 0xa40c).
  Four-byte store: target `sp -0x60`, value `ret(linked ELF 0x37d8)`, base register `r11`, raw displacement `-0x5c` (`str` at 0xa414).
  Branch (`beq` at 0xa420): taken to 0xb7ec when `Z == 1`, falls through to 0xa424 when `not (Z == 1)`.
  Branch (`beq` at 0xa430): taken to 0xb7ec when `Z == 1`, falls through to 0xa434 when `not (Z == 1)`.
  Four-byte store: target `sp -0x34`, value `unknown`, base register `r11`, raw displacement `-0x30` (`str` at 0xa438).
  Grouped accesses: 2 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0xa43c, `ldr` at 0xa484.
  Call (`bl` at 0xa444): (unknown callee), target 0x3394, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3394, conditional `false`, r0=`unknown`, r1=`0x2c`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x5c`, value `ret(linked ELF 0x3394)`, base register `r11`, raw displacement `-0x58` (`str` at 0xa44c).
  Branch (`bne` at 0xa458): taken to 0xa468 when `ret(linked ELF 0x3394) != 0x0`, falls through to 0xa45c when `ret(linked ELF 0x3394) == 0x0`.
  Four-byte store: target `sp -0x5c`, value `0x0`, base register `r11`, raw displacement `-0x58` (`str` at 0xa460).
  Four-byte load: target `sp -0x5c`, base register `r0`, raw displacement `+0x0` (`ldr` at 0xa46c).
  Byte store: target `ret(linked ELF 0x3394) +0x0`, value `0x0`, base register `r1`, raw displacement `+0x0` (`strb` at 0xa478).
  Four-byte store: target `sp -0x5c`, value `ret(linked ELF 0x3394)+1`, base register `r0`, raw displacement `+0x0` (`str` at 0xa480).
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xb298`, linked address 0xb298, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xa488, `ldr` at 0xa49c.
  Warning class: 2 native records report `ELF literal at linked 0xb298 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xa488, `ldr` at 0xa49c.
  Call (`bl` at 0xa48c): (unknown callee), target 0x376c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x376c, conditional `false`, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Four-byte store: target `sp -0x38`, value `ret(linked ELF 0x376c)`, base register `r11`, raw displacement `-0x34` (`str` at 0xa494).
  Call (`bl` at 0xa4a0): (unknown callee), target 0x376c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x376c, conditional `false`, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x376c)`.
  Four-byte store: target `sp -0x3c`, value `ret(linked ELF 0x376c)`, base register `r11`, raw displacement `-0x38` (`str` at 0xa4a8).
  Grouped accesses: 41 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0xa4ac, `ldr` at 0xa4d8, `ldr` at 0xa604.
  Branch (`bne` at 0xa4b4): taken to 0xa4c0 when `ret(linked ELF 0x376c) != 0x0`, falls through to 0xa4b8 when `ret(linked ELF 0x376c) == 0x0`.
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0xb314`, linked address 0xb314, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xa4b8, `ldr` at 0xab80, `ldr` at 0xac6c.
  Warning class: 5 native records report `ELF literal at linked 0xb314 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xa4b8, `ldr` at 0xab80, `ldr` at 0xac6c.
  Four-byte store: target `sp -0x3c`, value `unknown`, base register `r11`, raw displacement `-0x38` (`str` at 0xa4bc).
  Grouped accesses: 27 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0xa4c0, `ldr` at 0xa5ac, `ldr` at 0xa5e0.
  Four-byte load: target `ELF load bias (unknown) +0xb29c`, base register `pc`, raw displacement `+0xdd0`, linked address 0xb29c, basis `ELF load-bias-relative` (`ldr` at 0xa4c4).
  Native warning (`ldr` at 0xa4c4): `ELF literal at linked 0xb29c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa4c8): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0xa4d4): taken to 0xa5ac when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa4d8 when `ret(linked ELF 0x3418) == 0x0`.
  Call (`bl` at 0xa4dc): (unknown callee), target 0x3604, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3604, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Four-byte store: target `sp -0x40`, value `ret(linked ELF 0x3604)`, base register `r11`, raw displacement `-0x3c` (`str` at 0xa4e4).
  Grouped accesses: 2 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0xa4ec, `ldr` at 0xa53c.
  Call (`bl` at 0xa4f8): (unknown callee), target 0x33a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33a0, conditional `false`, r0=`[sp-24]`, r1=`ret(linked ELF 0x3604)`, r2=`0x0`, r3=`0x0`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x33a0)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa500).
  Grouped accesses: 12 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xa504, `ldr` at 0xa55c, `ldr` at 0xa560.
  Branch (`bne` at 0xa50c): taken to 0xa578 when `ret(linked ELF 0x33a0) != 0x0`, falls through to 0xa510 when `ret(linked ELF 0x33a0) == 0x0`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xb2a0`, linked address 0xb2a0, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xa510, `ldr` at 0xa578.
  Warning class: 2 native records report `ELF literal at linked 0xb2a0 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xa510, `ldr` at 0xa578.
  Branch (`bne` at 0xa51c): taken to 0xa56c when `? != 0x2`, falls through to 0xa520 when `? == 0x2`.
  Four-byte load: target `ELF load bias (unknown) +0xb2a4`, base register `pc`, raw displacement `+0xd7c`, linked address 0xb2a4, basis `ELF load-bias-relative` (`ldr` at 0xa520).
  Native warning (`ldr` at 0xa520): `ELF literal at linked 0xb2a4 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x9c`, value `sp-32`, base register `sp`, raw displacement `+0x0` (`str` at 0xa534).
  Call (`bl` at 0xa544): (unknown callee), target 0x3448, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3448, conditional `false`, r0=`[sp-24]`, r1=`[sp-64]`, r2=`unknown`, r3=`0x0`.
  Branch (`bge` at 0xa550): taken to 0xa55c when `ret(linked ELF 0x3448) >= 0x0 (signed, 32-bit)`, falls through to 0xa554 when `ret(linked ELF 0x3448) < 0x0 (signed, 32-bit)`.
  Four-byte store: target `[sp-32] +0x8`, value `[sp-32]`, base register `r2`, raw displacement `+0x8` (`str` at 0xa564).
  Branch (`beq` at 0xa584): taken to 0xa5a0 when `? == 0x2`, falls through to 0xa588 when `? != 0x2`.
  Four-byte load: target `ELF load bias (unknown) +0xb2a8`, base register `pc`, raw displacement `+0xd14`, linked address 0xb2a8, basis `ELF load-bias-relative` (`ldr` at 0xa58c).
  Native warning (`ldr` at 0xa58c): `ELF literal at linked 0xb2a8 has relocation, writable or unmapped bytes; value unknown`.
  Branch (`bne` at 0xa594): taken to 0xa5a0 when `ret(linked ELF 0x33a0) != ?`, falls through to 0xa598 when `ret(linked ELF 0x33a0) == ?`.
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0xa5a4).
  Jump (`b` at 0xa5a8): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2ac`, base register `pc`, raw displacement `+0xcf4`, linked address 0xb2ac, basis `ELF load-bias-relative` (`ldr` at 0xa5b0).
  Native warning (`ldr` at 0xa5b0): `ELF literal at linked 0xb2ac has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa5b4): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa5c0): taken to 0xa5e0 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa5c4 when `ret(linked ELF 0x3418) == 0x0`.
  Four-byte load: target `[sp-32] +0x8`, base register `r3`, raw displacement `+0x8` (`ldr` at 0xa5cc).
  Call (`bl` at 0xa5d0): (unknown callee), target 0x35a4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35a4, conditional `false`, r0=`[sp-24]`, r1=`[[sp-32]+8]`, r2=`unknown`, r3=`[sp-32]`.
  Jump (`b` at 0xa5dc): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2b0`, base register `pc`, raw displacement `+0xcc4`, linked address 0xb2b0, basis `ELF load-bias-relative` (`ldr` at 0xa5e4).
  Native warning (`ldr` at 0xa5e4): `ELF literal at linked 0xb2b0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa5e8): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa5f4): taken to 0xa644 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa5f8 when `ret(linked ELF 0x3418) == 0x0`.
  Grouped accesses: 73 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xa5f8, `ldr` at 0xa660, `ldr` at 0xa734.
  Branch (`beq` at 0xa600): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xa604 when `[sp-36] != 0x0`.
  Branch (`beq` at 0xa60c): taken to 0xa418 when `[sp-60] == 0x0`, falls through to 0xa610 when `[sp-60] != 0x0`.
  Call (`bl` at 0xa620): (unknown callee), target 0x3730, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3730, conditional `false`, r0=`[sp-60]`, r1=`0x0`, r2=`0x10`, r3=`[sp-60]`.
  Four-byte store: target `[sp-32] +0xc`, value `ret(linked ELF 0x3730)`, base register `r4`, raw displacement `+0xc` (`str` at 0xa624).
  Call (`bl` at 0xa63c): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`[sp-32]+16`, r1=`0x0`, r2=`0x100`, r3=`[sp-32]+16`.
  Jump (`b` at 0xa640): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2b4`, base register `pc`, raw displacement `+0xc64`, linked address 0xb2b4, basis `ELF load-bias-relative` (`ldr` at 0xa648).
  Native warning (`ldr` at 0xa648): `ELF literal at linked 0xb2b4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa650): (unknown callee), target 0x35f8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35f8, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`0x6`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa65c): taken to 0xa71c when `ret(linked ELF 0x35f8) != 0x0`, falls through to 0xa660 when `ret(linked ELF 0x35f8) == 0x0`.
  Branch (`beq` at 0xa668): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xa66c when `[sp-36] != 0x0`.
  Branch (`beq` at 0xa674): taken to 0xa418 when `[sp-60] == 0x0`, falls through to 0xa678 when `[sp-60] != 0x0`.
  Byte load: target `[sp-60] +0x0`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0xa67c).
  Branch (`bne` at 0xa684): taken to 0xa6a4 when `[[sp-60]]:1 != 0x2a`, falls through to 0xa688 when `[[sp-60]]:1 == 0x2a`.
  Call (`bl` at 0xa69c): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`[sp-32]+16`, r1=`0x1`, r2=`0x100`, r3=`[sp-32]+16`.
  Jump (`b` at 0xa6a0): target 0xa418.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xb2b8`, linked address 0xb2b8, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xa6a8, `ldr` at 0xa708.
  Warning class: 2 native records report `ELF literal at linked 0xb2b8 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xa6a8, `ldr` at 0xa708.
  Call (`bl` at 0xa6ac): (unknown callee), target 0x376c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x376c, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`[[sp-60]]:1`.
  Grouped accesses: 2 4-byte store records with target `sp -0x64`, reported value `ret(linked ELF 0x376c)` in every record, base register `r11`, raw displacement `-0x60`, including `str` at 0xa6b4, `str` at 0xa714.
  Grouped accesses: 7 4-byte load records with target `sp -0x64`, base register `r11`, raw displacement `-0x60`, including `ldr` at 0xa6b8, `ldr` at 0xa6c4, `ldr` at 0xab34.
  Branch (`beq` at 0xa6c0): taken to 0xa418 when `ret(linked ELF 0x376c) == 0x0`, falls through to 0xa6c4 when `ret(linked ELF 0x376c) != 0x0`.
  Call (`bl` at 0xa6d0): (unknown callee), target 0x3730, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3730, conditional `false`, r0=`ret(linked ELF 0x376c)`, r1=`0x0`, r2=`0xa`, r3=`ret(linked ELF 0x376c)`.
  Four-byte store: target `sp -0x4c`, value `ret(linked ELF 0x3730)`, base register `r11`, raw displacement `-0x48` (`str` at 0xa6d8).
  Grouped accesses: 4 4-byte load records with target `sp -0x4c`, base register `r11`, raw displacement `-0x48`, including `ldr` at 0xa6dc, `ldr` at 0xa6f0, `ldr` at 0xb7c0.
  Branch (`bgt` at 0xa6e4): taken to 0xa704 when `ret(linked ELF 0x3730) > 0xff (signed, 32-bit)`, falls through to 0xa6e8 when `ret(linked ELF 0x3730) <= 0xff (signed, 32-bit)`.
  Byte store: target `([sp-32] + ret(linked ELF 0x3730)) +0x10`, value `0x1`, base register `r2`, raw displacement `+0x0` (`strb` at 0xa700).
  Call (`bl` at 0xa70c): (unknown callee), target 0x376c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x376c, conditional `false`, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0xa718): target 0xa6b8.
  Four-byte load: target `ELF load bias (unknown) +0xb2bc`, base register `pc`, raw displacement `+0xb94`, linked address 0xb2bc, basis `ELF load-bias-relative` (`ldr` at 0xa720).
  Native warning (`ldr` at 0xa720): `ELF literal at linked 0xb2bc has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa724): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x35f8)`.
  Branch (`bne` at 0xa730): taken to 0xa76c when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa734 when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xa73c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xa740 when `[sp-36] != 0x0`.
  Call (`bl` at 0xa754): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`[sp-36]+280`, r1=`0x0`, r2=`0x30`, r3=`[sp-36]+280`.
  Call (`bl` at 0xa760): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `[sp-36] +0x118`, value `ret(linked ELF 0x3700)`, base register `r4`, raw displacement `+0x118` (`str` at 0xa764).
  Jump (`b` at 0xa768): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2c0`, base register `pc`, raw displacement `+0xb48`, linked address 0xb2c0, basis `ELF load-bias-relative` (`ldr` at 0xa770).
  Native warning (`ldr` at 0xa770): `ELF literal at linked 0xb2c0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa774): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa780): taken to 0xa7b4 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa784 when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xa78c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xa790 when `[sp-36] != 0x0`.
  Branch (`beq` at 0xa798): taken to 0xa418 when `[sp-60] == 0x0`, falls through to 0xa79c when `[sp-60] != 0x0`.
  Call (`bl` at 0xa7ac): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[sp-36]+284`, r1=`[sp-60]`, r2=`unknown`, r3=`[sp-36]+284`.
  Jump (`b` at 0xa7b0): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2c4`, base register `pc`, raw displacement `+0xb04`, linked address 0xb2c4, basis `ELF load-bias-relative` (`ldr` at 0xa7b8).
  Native warning (`ldr` at 0xa7b8): `ELF literal at linked 0xb2c4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa7bc): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa7c8): taken to 0xa7fc when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa7cc when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xa7d4): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xa7d8 when `[sp-36] != 0x0`.
  Branch (`beq` at 0xa7e0): taken to 0xa418 when `[sp-60] == 0x0`, falls through to 0xa7e4 when `[sp-60] != 0x0`.
  Call (`bl` at 0xa7f4): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[sp-36]+304`, r1=`[sp-60]`, r2=`unknown`, r3=`[sp-36]+304`.
  Jump (`b` at 0xa7f8): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2c8`, base register `pc`, raw displacement `+0xac0`, linked address 0xb2c8, basis `ELF load-bias-relative` (`ldr` at 0xa800).
  Native warning (`ldr` at 0xa800): `ELF literal at linked 0xb2c8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa804): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa810): taken to 0xa868 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa814 when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xa81c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xa820 when `[sp-36] != 0x0`.
  Branch (`beq` at 0xa828): taken to 0xa418 when `[sp-60] == 0x0`, falls through to 0xa82c when `[sp-60] != 0x0`.
  Call (`bl` at 0xa834): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`[sp-60]`.
  Byte load: target `[sp-36] +0x144`, base register `r4`, raw displacement `+0x0` (`ldrb` at 0xa840).
  Byte store: target `sp -0x70`, value `([[sp-36]+324]:1 & 0xff)`, base register `r11`, raw displacement `-0x6c` (`strb` at 0xa844).
  Grouped accesses: 2 1-byte load records with target `sp -0x70`, base register `r11`, raw displacement `-0x6c`, including `ldrb` at 0xa84c, `ldrb` at 0xa85c.
  Byte store: target `sp -0x70`, value `unknown`, base register `r11`, raw displacement `-0x6c` (`strb` at 0xa858).
  Byte store: target `[sp-36] +0x144`, value `([sp-112]:1 & 0xff)`, base register `r4`, raw displacement `+0x0` (`strb` at 0xa860).
  Jump (`b` at 0xa864): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2cc`, base register `pc`, raw displacement `+0xa58`, linked address 0xb2cc, basis `ELF load-bias-relative` (`ldr` at 0xa86c).
  Native warning (`ldr` at 0xa86c): `ELF literal at linked 0xb2cc has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xa870): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xa87c): taken to 0xaa58 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xa880 when `ret(linked ELF 0x3418) == 0x0`.
  Call (`bl` at 0xa884): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Four-byte store: target `sp -0x44`, value `ret(linked ELF 0x3700)`, base register `r11`, raw displacement `-0x40` (`str` at 0xa888).
  Branch (`beq` at 0xa894): taken to 0xaa34 when `[sp-36] == 0x0`, falls through to 0xa898 when `[sp-36] != 0x0`.
  Grouped accesses: 41 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0xa898, `ldr` at 0xa8ac, `ldr` at 0xa8bc.
  Branch (`bne` at 0xa8a0): taken to 0xaa34 when `ret(linked ELF 0x3700) != 0x0`, falls through to 0xa8a4 when `ret(linked ELF 0x3700) == 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x44`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x40`, including `str` at 0xa8a8, `str` at 0xaa0c.
  Branch (`bhi` at 0xa8b4): taken to 0xaa08 when `hi flags condition (operands unknown)`, falls through to 0xa8b8 when `not (hi flags condition (operands unknown))`.
  Warning class: 38 native records report `rsb: unsupported effects invalidated`, including `rsb` at 0xa8cc, `rsb` at 0xa8fc, `rsb` at 0xa928.
  Branch (`beq` at 0xa8e4): taken to 0xa914 when `? == 0x0`, falls through to 0xa8e8 when `? != 0x0`.
  Call (`bl` at 0xa910): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`unknown`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Branch (`beq` at 0xa940): taken to 0xa970 when `? == 0x0`, falls through to 0xa944 when `? != 0x0`.
  Call (`bl` at 0xa96c): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`unknown`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Branch (`beq` at 0xa99c): taken to 0xa9cc when `? == 0x0`, falls through to 0xa9a0 when `? != 0x0`.
  Call (`bl` at 0xa9c8): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`unknown`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xa9f4).
  Four-byte store: target `sp -0x44`, value `[sp-68]+1`, base register `r11`, raw displacement `-0x40` (`str` at 0xaa00).
  Jump (`b` at 0xaa04): target 0xa8ac.
  Call (`bl` at 0xaa24): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`[sp-36]+332`, r1=`0x0`, r2=`0x460`, r3=`[sp-36]+332`.
  Four-byte store: target `[sp-36] +0x148`, value `0x0`, base register `r2`, raw displacement `+0x148` (`str` at 0xaa30).
  Branch (`beq` at 0xaa3c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xaa40 when `[sp-36] != 0x0`.
  Four-byte load: target `[sp-36] +0x148`, base register `r3`, raw displacement `+0x148` (`ldr` at 0xaa48).
  Four-byte store: target `[sp-36] +0x148`, value `[[sp-36]+328]+1`, base register `r2`, raw displacement `+0x148` (`str` at 0xaa50).
  Jump (`b` at 0xaa54): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2d0`, base register `pc`, raw displacement `+0x86c`, linked address 0xb2d0, basis `ELF load-bias-relative` (`ldr` at 0xaa5c).
  Native warning (`ldr` at 0xaa5c): `ELF literal at linked 0xb2d0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xaa60): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xaa6c): taken to 0xaab0 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xaa70 when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xaa78): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xaa7c when `[sp-36] != 0x0`.
  Call (`bl` at 0xaaa4): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x3700)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]` (`str` at 0xaaa8).
  Jump (`b` at 0xaaac): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2d4`, base register `pc`, raw displacement `+0x818`, linked address 0xb2d4, basis `ELF load-bias-relative` (`ldr` at 0xaab4).
  Native warning (`ldr` at 0xaab4): `ELF literal at linked 0xb2d4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xaab8): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xaac4): taken to 0xab9c when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xaac8 when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xaad0): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xaad4 when `[sp-36] != 0x0`.
  Branch (`beq` at 0xaadc): taken to 0xab00 when `[sp-60] == 0x0`, falls through to 0xaae0 when `[sp-60] != 0x0`.
  Call (`bl` at 0xaae4): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`[sp-60]`.
  Four-byte store: target `sp -0x74`, value `ret(linked ELF 0x35d4)`, base register `r11`, raw displacement `-0x70` (`str` at 0xaaec).
  Grouped accesses: 2 4-byte load records with target `sp -0x74`, base register `r11`, raw displacement `-0x70`, including `ldr` at 0xaaf0, `ldr` at 0xab08.
  Four-byte store: target `sp -0x74`, value `ret(linked ELF 0x35d4)+1`, base register `r11`, raw displacement `-0x70` (`str` at 0xaaf8).
  Four-byte store: target `sp -0x74`, value `0x1`, base register `r11`, raw displacement `-0x70` (`str` at 0xab04).
  Grouped accesses: 3 4-byte store records with target `sp -0x64`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x60`, including `str` at 0xab0c, `str` at 0xabf8, `str` at 0xad00.
  Call (`bl` at 0xab38): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`[sp-100]`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Grouped accesses: 3 4-byte store records with target `(unknown effective address)`, reported value `ret(linked ELF 0x3514)` in every record, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]`, including `str` at 0xab40, `str` at 0xac2c, `str` at 0xad3c.
  Four-byte store: target `sp -0x78`, value `unknown`, base register `r11`, raw displacement `-0x74` (`str` at 0xab68).
  Four-byte store: target `sp -0x7c`, value `[sp-60]`, base register `r11`, raw displacement `-0x78` (`str` at 0xab70).
  Branch (`bne` at 0xab7c): taken to 0xab88 when `[sp-60] != 0x0`, falls through to 0xab80 when `[sp-60] == 0x0`.
  Four-byte store: target `sp -0x7c`, value `unknown`, base register `r11`, raw displacement `-0x78` (`str` at 0xab84).
  Four-byte load: target `sp -0x78`, base register `r11`, raw displacement `-0x74` (`ldr` at 0xab88).
  Four-byte load: target `[sp-120] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xab8c).
  Four-byte load: target `sp -0x7c`, base register `r11`, raw displacement `-0x78` (`ldr` at 0xab90).
  Call (`bl` at 0xab94): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[[sp-120]]`, r1=`unknown`, r2=`[sp-120]`, r3=`[sp-60]`.
  Jump (`b` at 0xab98): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2d8`, base register `pc`, raw displacement `+0x730`, linked address 0xb2d8, basis `ELF load-bias-relative` (`ldr` at 0xaba0).
  Native warning (`ldr` at 0xaba0): `ELF literal at linked 0xb2d8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xaba4): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xabb0): taken to 0xac88 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xabb4 when `ret(linked ELF 0x3418) == 0x0`.
  Branch (`beq` at 0xabbc): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xabc0 when `[sp-36] != 0x0`.
  Branch (`beq` at 0xabc8): taken to 0xabec when `[sp-60] == 0x0`, falls through to 0xabcc when `[sp-60] != 0x0`.
  Call (`bl` at 0xabd0): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`[sp-60]`.
  Four-byte store: target `sp -0x80`, value `ret(linked ELF 0x35d4)`, base register `r11`, raw displacement `-0x7c` (`str` at 0xabd8).
  Grouped accesses: 2 4-byte load records with target `sp -0x80`, base register `r11`, raw displacement `-0x7c`, including `ldr` at 0xabdc, `ldr` at 0xabf4.
  Four-byte store: target `sp -0x80`, value `ret(linked ELF 0x35d4)+1`, base register `r11`, raw displacement `-0x7c` (`str` at 0xabe4).
  Four-byte store: target `sp -0x80`, value `0x1`, base register `r11`, raw displacement `-0x7c` (`str` at 0xabf0).
  Call (`bl` at 0xac24): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`[sp-100]`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Four-byte store: target `sp -0x84`, value `unknown`, base register `r11`, raw displacement `-0x80` (`str` at 0xac54).
  Four-byte store: target `sp -0x88`, value `[sp-60]`, base register `r11`, raw displacement `-0x84` (`str` at 0xac5c).
  Branch (`bne` at 0xac68): taken to 0xac74 when `[sp-60] != 0x0`, falls through to 0xac6c when `[sp-60] == 0x0`.
  Four-byte store: target `sp -0x88`, value `unknown`, base register `r11`, raw displacement `-0x84` (`str` at 0xac70).
  Four-byte load: target `sp -0x84`, base register `r11`, raw displacement `-0x80` (`ldr` at 0xac74).
  Four-byte load: target `[sp-132] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0xac78).
  Four-byte load: target `sp -0x88`, base register `r11`, raw displacement `-0x84` (`ldr` at 0xac7c).
  Call (`bl` at 0xac80): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[[sp-132]]`, r1=`unknown`, r2=`[sp-132]`, r3=`[sp-60]`.
  Jump (`b` at 0xac84): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2dc`, base register `pc`, raw displacement `+0x648`, linked address 0xb2dc, basis `ELF load-bias-relative` (`ldr` at 0xac8c).
  Native warning (`ldr` at 0xac8c): `ELF literal at linked 0xb2dc has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xac90): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3418)`.
  Branch (`bne` at 0xac9c): taken to 0xaf8c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xaca0 when `ret(linked ELF 0x36dc) == 0x0`.
  Call (`bl` at 0xaca4): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Four-byte store: target `sp -0x48`, value `ret(linked ELF 0x3700)`, base register `r11`, raw displacement `-0x44` (`str` at 0xaca8).
  Branch (`bne` at 0xacb4): taken to 0xaccc when `[sp-36] != 0x0`, falls through to 0xacb8 when `[sp-36] == 0x0`.
  Branch (`beq` at 0xacc0): taken to 0xaccc when `[sp-32] == 0x0`, falls through to 0xacc4 when `[sp-32] != 0x0`.
  Four-byte store: target `sp -0x24`, value `[sp-32]`, base register `r11`, raw displacement `-0x20` (`str` at 0xacc8).
  Branch (`beq` at 0xacd4): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xacd8 when `[sp-36] != 0x0`.
  Branch (`bne` at 0xad08): taken to 0xad44 when `? != 0x0`, falls through to 0xad0c when `? == 0x0`.
  Call (`bl` at 0xad34): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`0x2c`, r1=`[sp-36]`, r2=`[sp-68]`, r3=`unknown`.
  Grouped accesses: 20 4-byte load records with target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]`, including `ldr` at 0xadac, `ldr` at 0xae00, `ldr` at 0xae58.
  Call (`bl` at 0xadb4): (unknown callee), target 0x364c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x364c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-100]`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x364c)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]` (`str` at 0xadbc).
  Call (`bl` at 0xae14): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`0x0`, r2=`0x2c`, r3=`unknown`.
  Grouped accesses: 18 4-byte load records with target `sp -0x48`, base register `r11`, raw displacement `-0x44`, including `ldr` at 0xae60, `ldr` at 0xaf0c, `ldr` at 0xaf5c.
  Byte store: target `(unknown effective address)`, value `([sp-72] & 0xff)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`strb` at 0xae64).
  Four-byte store: target `sp -0x48`, value `unknown`, base register `r11`, raw displacement `-0x44` (`str` at 0xae90).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r12`, raw displacement `+0x0`, raw addressing form `[ip]` (`str` at 0xaee4).
  Byte store: target `(unknown effective address)`, value `0x1`, base register `r2`, raw displacement `+0x1`, raw addressing form `[r2, #1]` (`strb` at 0xaf34).
  Byte store: target `(unknown effective address)`, value `0x2`, base register `r2`, raw displacement `+0xc`, raw addressing form `[r2, #0xc]` (`strb` at 0xaf84).
  Jump (`b` at 0xaf88): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2e0`, base register `pc`, raw displacement `+0x348`, linked address 0xb2e0, basis `ELF load-bias-relative` (`ldr` at 0xaf90).
  Native warning (`ldr` at 0xaf90): `ELF literal at linked 0xb2e0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xaf94): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xafa0): taken to 0xb00c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xafa4 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xafac): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xafb0 when `[sp-36] != 0x0`.
  Call (`bl` at 0xaffc): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x3`, raw addressing form `[r4, #3]` (`strb` at 0xb004).
  Jump (`b` at 0xb008): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2e4`, base register `pc`, raw displacement `+0x2cc`, linked address 0xb2e4, basis `ELF load-bias-relative` (`ldr` at 0xb010).
  Native warning (`ldr` at 0xb010): `ELF literal at linked 0xb2e4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb014): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb020): taken to 0xb08c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb024 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb02c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb030 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb07c): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x4`, raw addressing form `[r4, #4]` (`strb` at 0xb084).
  Jump (`b` at 0xb088): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2e8`, base register `pc`, raw displacement `+0x250`, linked address 0xb2e8, basis `ELF load-bias-relative` (`ldr` at 0xb090).
  Native warning (`ldr` at 0xb090): `ELF literal at linked 0xb2e8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb094): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb0a0): taken to 0xb10c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb0a4 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb0ac): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb0b0 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb0fc): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x8`, raw addressing form `[r4, #8]` (`strb` at 0xb104).
  Jump (`b` at 0xb108): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2ec`, base register `pc`, raw displacement `+0x1d4`, linked address 0xb2ec, basis `ELF load-bias-relative` (`ldr` at 0xb110).
  Native warning (`ldr` at 0xb110): `ELF literal at linked 0xb2ec has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb114): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb120): taken to 0xb18c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb124 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb12c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb130 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb17c): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x7`, raw addressing form `[r4, #7]` (`strb` at 0xb184).
  Jump (`b` at 0xb188): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2f0`, base register `pc`, raw displacement `+0x158`, linked address 0xb2f0, basis `ELF load-bias-relative` (`ldr` at 0xb190).
  Native warning (`ldr` at 0xb190): `ELF literal at linked 0xb2f0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb194): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb1a0): taken to 0xb20c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb1a4 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb1ac): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb1b0 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb1fc): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x5`, raw addressing form `[r4, #5]` (`strb` at 0xb204).
  Jump (`b` at 0xb208): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2f4`, base register `pc`, raw displacement `+0xdc`, linked address 0xb2f4, basis `ELF load-bias-relative` (`ldr` at 0xb210).
  Native warning (`ldr` at 0xb210): `ELF literal at linked 0xb2f4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb214): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb220): taken to 0xb320 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb224 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb22c): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb230 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb27c): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x9`, raw addressing form `[r4, #9]` (`strb` at 0xb284).
  Jump (`b` at 0xb288): target 0xa418.
  The decoded records from 0xb28c to 0xb31c are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.
  Four-byte load: target `ELF load bias (unknown) +0xb2f8`, base register `pc`, raw displacement `-0x34`, linked address 0xb2f8, basis `ELF load-bias-relative` (`ldr` at 0xb324).
  Native warning (`ldr` at 0xb324): `ELF literal at linked 0xb2f8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb328): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb334): taken to 0xb3a0 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb338 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb340): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb344 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb390): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0xa`, raw addressing form `[r4, #0xa]` (`strb` at 0xb398).
  Jump (`b` at 0xb39c): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb2fc`, base register `pc`, raw displacement `-0xb0`, linked address 0xb2fc, basis `ELF load-bias-relative` (`ldr` at 0xb3a4).
  Native warning (`ldr` at 0xb3a4): `ELF literal at linked 0xb2fc has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb3a8): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb3b4): taken to 0xb420 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb3b8 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb3c0): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb3c4 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb410): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0xb`, raw addressing form `[r4, #0xb]` (`strb` at 0xb418).
  Jump (`b` at 0xb41c): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb300`, base register `pc`, raw displacement `-0x12c`, linked address 0xb300, basis `ELF load-bias-relative` (`ldr` at 0xb424).
  Native warning (`ldr` at 0xb424): `ELF literal at linked 0xb300 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb428): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb434): taken to 0xb49c when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb438 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb440): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb444 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb490): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x3700)`, base register `r4`, raw displacement `+0x10`, raw addressing form `[r4, #0x10]` (`str` at 0xb494).
  Jump (`b` at 0xb498): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb304`, base register `pc`, raw displacement `-0x1a4`, linked address 0xb304, basis `ELF load-bias-relative` (`ldr` at 0xb4a0).
  Native warning (`ldr` at 0xb4a0): `ELF literal at linked 0xb304 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb4a4): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb4b0): taken to 0xb5c0 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb4b4 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb4bc): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb4c0 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb50c): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0xc`, raw addressing form `[r4, #0xc]` (`strb` at 0xb514).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0xc`, raw addressing form `[r3, #0xc]` (`ldrb` at 0xb560).
  Branch (`bne` at 0xb568): taken to 0xa418 when `? != 0x1`, falls through to 0xb56c when `? == 0x1`.
  Byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x1`, raw addressing form `[r2, #1]` (`strb` at 0xb5b8).
  Jump (`b` at 0xb5bc): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb308`, base register `pc`, raw displacement `-0x2c4`, linked address 0xb308, basis `ELF load-bias-relative` (`ldr` at 0xb5c4).
  Native warning (`ldr` at 0xb5c4): `ELF literal at linked 0xb308 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb5c8): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb5d4): taken to 0xb640 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb5d8 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb5e0): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb5e4 when `[sp-36] != 0x0`.
  Call (`bl` at 0xb630): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x3700) & 0xff)`, base register `r4`, raw displacement `+0x6`, raw addressing form `[r4, #6]` (`strb` at 0xb638).
  Jump (`b` at 0xb63c): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb30c`, base register `pc`, raw displacement `-0x340`, linked address 0xb30c, basis `ELF load-bias-relative` (`ldr` at 0xb644).
  Native warning (`ldr` at 0xb644): `ELF literal at linked 0xb30c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb648): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb654): taken to 0xb6e0 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb658 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb660): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb664 when `[sp-36] != 0x0`.
  Four-byte store: target `sp -0x8c`, value `unknown`, base register `r11`, raw displacement `-0x88` (`str` at 0xb6b0).
  Four-byte store: target `sp -0x90`, value `[sp-60]`, base register `r11`, raw displacement `-0x8c` (`str` at 0xb6b8).
  Branch (`bne` at 0xb6c4): taken to 0xb6d0 when `[sp-60] != 0x0`, falls through to 0xb6c8 when `[sp-60] == 0x0`.
  Four-byte store: target `sp -0x90`, value `unknown`, base register `r11`, raw displacement `-0x8c` (`str` at 0xb6cc).
  Four-byte load: target `sp -0x8c`, base register `r11`, raw displacement `-0x88` (`ldr` at 0xb6d0).
  Four-byte load: target `sp -0x90`, base register `r11`, raw displacement `-0x8c` (`ldr` at 0xb6d4).
  Call (`bl` at 0xb6d8): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[sp-140]`, r1=`unknown`, r2=`unknown`, r3=`[sp-60]`.
  Jump (`b` at 0xb6dc): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb310`, base register `pc`, raw displacement `-0x3dc`, linked address 0xb310, basis `ELF load-bias-relative` (`ldr` at 0xb6e4).
  Native warning (`ldr` at 0xb6e4): `ELF literal at linked 0xb310 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb6e8): (unknown callee), target 0x36dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36dc, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb6f4): taken to 0xb780 when `ret(linked ELF 0x36dc) != 0x0`, falls through to 0xb6f8 when `ret(linked ELF 0x36dc) == 0x0`.
  Branch (`beq` at 0xb700): taken to 0xa418 when `[sp-36] == 0x0`, falls through to 0xb704 when `[sp-36] != 0x0`.
  Four-byte store: target `sp -0x94`, value `unknown`, base register `r11`, raw displacement `-0x90` (`str` at 0xb750).
  Four-byte store: target `sp -0x98`, value `[sp-60]`, base register `r11`, raw displacement `-0x94` (`str` at 0xb758).
  Branch (`bne` at 0xb764): taken to 0xb770 when `[sp-60] != 0x0`, falls through to 0xb768 when `[sp-60] == 0x0`.
  Four-byte store: target `sp -0x98`, value `unknown`, base register `r11`, raw displacement `-0x94` (`str` at 0xb76c).
  Four-byte load: target `sp -0x94`, base register `r11`, raw displacement `-0x90` (`ldr` at 0xb770).
  Four-byte load: target `sp -0x98`, base register `r11`, raw displacement `-0x94` (`ldr` at 0xb774).
  Call (`bl` at 0xb778): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[sp-148]`, r1=`unknown`, r2=`[sp-60]`, r3=`unknown`.
  Jump (`b` at 0xb77c): target 0xa418.
  Four-byte load: target `ELF load bias (unknown) +0xb318`, base register `pc`, raw displacement `-0x474`, linked address 0xb318, basis `ELF load-bias-relative` (`ldr` at 0xb784).
  Native warning (`ldr` at 0xb784): `ELF literal at linked 0xb318 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb788): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x36dc)`.
  Branch (`bne` at 0xb794): taken to 0xa418 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0xb798 when `ret(linked ELF 0x3418) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xb31c`, base register `pc`, raw displacement `-0x490`, linked address 0xb31c, basis `ELF load-bias-relative` (`ldr` at 0xb7a4).
  Native warning (`ldr` at 0xb7a4): `ELF literal at linked 0xb31c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xb7b0): (unknown callee), target 0x3718, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3718, conditional `false`, r0=`[sp-60]`, r1=`unknown`, r2=`sp-76`, r3=`sp-80`.
  Grouped accesses: 2 4-byte load records with target `sp -0x50`, base register `r11`, raw displacement `-0x4c`, including `ldr` at 0xb7b4, `ldr` at 0xb7e0.
  Branch (`beq` at 0xb7bc): taken to 0xa418 when `[sp-80] == 0x0`, falls through to 0xb7c0 when `[sp-80] != 0x0`.
  Branch (`bgt` at 0xb7c8): taken to 0xa418 when `[sp-76] > 0xff (signed, 32-bit)`, falls through to 0xb7cc when `[sp-76] <= 0xff (signed, 32-bit)`.
  Four-byte load: target `[sp-24] +0x5b4`, base register `r1`, raw displacement `+0x5b4` (`ldr` at 0xb7d8).
  Four-byte store: target `(unknown effective address)`, value `[sp-80]`, base register `r2`, raw displacement `+0x4`, raw addressing form `[r2, #4]` (`str` at 0xb7e4).
  Jump (`b` at 0xb7e8): target 0xa418.
  Four-byte load: target `sp -0x60`, base register `r11`, raw displacement `-0x5c` (`ldr` at 0xb7ec).
  Call (`bl` at 0xb7f0): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x30`, value `[sp-48]+1`, base register `r11`, raw displacement `-0x2c` (`str` at 0xb7fc).
  Jump (`b` at 0xb800): target 0xa360.
  Native warning (`mvn` at 0xb808): `mvn: unsupported effects invalidated`.
  Call (`bl` at 0xb80c): (unknown callee), target 0x35ec, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35ec, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Known-edge return (`ldm` at 0xb814): recorded r0 is `unknown`.

- `L2CfgMCfgL2_rev1`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x9bdc to 0xa2ac (exclusive), 1744 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `712f2e8a57169835b1b7accc0325f12913b557acdba36edba04b4288c650d27b`.
  The native facts retain 1 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0x9be0).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0x9bec).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0x9bf0).
  Grouped accesses: 15 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x9bf4, `ldr` at 0x9c08, `ldr` at 0x9c18.
  Byte load: target `r1 +0x24`, base register `r3`, raw displacement `+0x24` (`ldrb` at 0x9bf8).
  Branch (`bne` at 0x9c04): taken to 0x9c18 when `? != 0x0`, falls through to 0x9c08 when `? == 0x0`.
  Four-byte load: target `r1 +0x28`, base register `r3`, raw displacement `+0x28` (`ldr` at 0x9c0c).
  Four-byte store: target `sp -0x4c`, value `[r1+40]`, base register `r11`, raw displacement `-0x48` (`str` at 0x9c10).
  Call (`bl` at 0x9c20): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`r1`, r1=`0x0`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x4c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x48` (`str` at 0x9c24).
  Four-byte load: target `sp -0x4c`, base register `r11`, raw displacement `-0x48` (`ldr` at 0x9c28).
  Four-byte store: target `sp -0x20`, value `[sp-76]`, base register `r11`, raw displacement `-0x1c` (`str` at 0x9c2c).
  Byte load: target `[sp-28] +0x2c`, base register `r3`, raw displacement `+0x2c` (`ldrb` at 0x9c34).
  Branch (`bne` at 0x9c40): taken to 0x9c54 when `? != 0x0`, falls through to 0x9c44 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x30`, base register `r3`, raw displacement `+0x30` (`ldr` at 0x9c48).
  Four-byte store: target `sp -0x50`, value `[[sp-28]+48]`, base register `r11`, raw displacement `-0x4c` (`str` at 0x9c4c).
  Call (`bl` at 0x9c5c): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x50`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x4c` (`str` at 0x9c60).
  Four-byte load: target `sp -0x50`, base register `r11`, raw displacement `-0x4c` (`ldr` at 0x9c64).
  Four-byte store: target `sp -0x24`, value `[sp-80]`, base register `r11`, raw displacement `-0x20` (`str` at 0x9c68).
  Byte load: target `[sp-28] +0x34`, base register `r3`, raw displacement `+0x34` (`ldrb` at 0x9c70).
  Branch (`bne` at 0x9c7c): taken to 0x9c90 when `? != 0x0`, falls through to 0x9c80 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x38`, base register `r3`, raw displacement `+0x38` (`ldr` at 0x9c84).
  Four-byte store: target `sp -0x54`, value `[[sp-28]+56]`, base register `r11`, raw displacement `-0x50` (`str` at 0x9c88).
  Call (`bl` at 0x9c98): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x54`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x50` (`str` at 0x9c9c).
  Four-byte load: target `sp -0x54`, base register `r11`, raw displacement `-0x50` (`ldr` at 0x9ca0).
  Four-byte store: target `sp -0x28`, value `[sp-84]`, base register `r11`, raw displacement `-0x24` (`str` at 0x9ca4).
  Four-byte store: target `sp -0x2c`, value `0x0`, base register `r11`, raw displacement `-0x28` (`str` at 0x9cac).
  Grouped accesses: 8 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x9cb0, `ldr` at 0x9cc4, `ldr` at 0x9cec.
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x9cb4).
  Branch (`bge` at 0x9cbc): taken to 0xa2a0 when `ge flags condition (operands unknown)`, falls through to 0x9cc0 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldrb` at 0x9cd8, `ldrb` at 0x9d6c.
  Branch (`bne` at 0x9ce4): taken to 0x9d0c when `? != 0x0`, falls through to 0x9ce8 when `? == 0x0`.
  Grouped accesses: 12 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0x9d00, `ldr` at 0x9d94, `ldr` at 0x9e2c.
  Four-byte store: target `sp -0x58`, value `unknown`, base register `r11`, raw displacement `-0x54` (`str` at 0x9d04).
  Call (`bl` at 0x9d20): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Four-byte store: target `sp -0x58`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x54` (`str` at 0x9d24).
  Four-byte load: target `sp -0x58`, base register `r11`, raw displacement `-0x54` (`ldr` at 0x9d28).
  Four-byte store: target `sp -0x30`, value `unknown`, base register `r11`, raw displacement `-0x2c` (`str` at 0x9d2c).
  Four-byte load: target `ELF load bias (unknown) +0xa2a8`, base register `pc`, raw displacement `+0x570`, linked address 0xa2a8, basis `ELF load-bias-relative` (`ldr` at 0x9d30).
  Native warning (`ldr` at 0x9d30): `ELF literal at linked 0xa2a8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0x9d48): (unknown callee), target 0x3694, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3694, conditional `false`, r0=`sp-56`, r1=`unknown`, r2=`0x4`, r3=`unknown`.
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0x9d4c).
  Four-byte store: target `sp -0x3c`, value `[sp-24]`, base register `r11`, raw displacement `-0x38` (`str` at 0x9d50).
  Branch (`bne` at 0x9d78): taken to 0x9da0 when `? != 0x0`, falls through to 0x9d7c when `? == 0x0`.
  Four-byte store: target `sp -0x5c`, value `unknown`, base register `r11`, raw displacement `-0x58` (`str` at 0x9d98).
  Call (`bl` at 0x9db4): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Four-byte store: target `sp -0x5c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x58` (`str` at 0x9db8).
  Four-byte load: target `sp -0x5c`, base register `r11`, raw displacement `-0x58` (`ldr` at 0x9dbc).
  Four-byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`str` at 0x9dc0).
  Four-byte store: target `sp -0x34`, value `0x0`, base register `r11`, raw displacement `-0x30` (`str` at 0x9dc8).
  Grouped accesses: 19 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x9dcc, `ldr` at 0x9de0, `ldr` at 0x9e04.
  Four-byte load: target `sp -0x30`, base register `r11`, raw displacement `-0x2c` (`ldr` at 0x9dd0).
  Branch (`bge` at 0x9dd8): taken to 0xa290 when `ge flags condition (operands unknown)`, falls through to 0x9ddc when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 9 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0x9ddc, `ldr` at 0x9e3c, `ldr` at 0x9e70.
  Warning class: 9 native records report `rsb: unsupported effects invalidated`, including `rsb` at 0x9df0, `rsb` at 0x9e50, `rsb` at 0x9e84.
  Grouped accesses: 8 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0x9e00, `ldr` at 0x9fc4, `ldr` at 0xa074.
  Call (`bl` at 0x9e30): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-64]`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x337c)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]` (`str` at 0x9e38).
  Branch (`beq` at 0x9e68): taken to 0x9e70 when `? == 0x2`, falls through to 0x9e6c when `? != 0x2`.
  Branch (`beq` at 0x9e9c): taken to 0x9ecc when `? == 0x0`, falls through to 0x9ea0 when `? != 0x0`.
  Call (`bl` at 0x9ec8): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`unknown`, r1=`[sp-60]`, r2=`[sp-52]`, r3=`unknown`.
  Call (`bl` at 0x9ef4): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`0xb0`, r1=`[sp-60]`, r2=`[sp-52]`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x3514)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4]` (`str` at 0x9efc).
  Call (`bl` at 0x9f30): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`0x0`, r2=`0xb0`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `0x4`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0x9f5c).
  Four-byte store: target `sp -0x44`, value `0x0`, base register `r11`, raw displacement `-0x40` (`str` at 0x9f64).
  Grouped accesses: 11 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0x9f68, `ldr` at 0x9f98, `ldr` at 0x9fc8.
  Branch (`bgt` at 0x9f70): taken to 0xa280 when `gt flags condition (operands unknown)`, falls through to 0x9f74 when `not (gt flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]`, including `ldr` at 0x9fb4, `ldr` at 0xa058.
  Four-byte store: target `sp -0x48`, value `unknown`, base register `r11`, raw displacement `-0x44` (`str` at 0x9fbc).
  Grouped accesses: 7 4-byte load records with target `sp -0x48`, base register `r11`, raw displacement `-0x44`, including `ldr` at 0x9fc0, `ldr` at 0xa070, `ldr` at 0xa0c8.
  Call (`bl` at 0xa00c): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-72] +0x1`, value `(ret(linked ELF 0x337c) & 0xff)`, base register `r4`, raw displacement `+0x1` (`strb` at 0xa014).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x1`, raw addressing form `[r3, #1]` (`ldrb` at 0xa060).
  Branch (`bne` at 0xa068): taken to 0xa070 when `? != 0x0`, falls through to 0xa06c when `? == 0x0`.
  Call (`bl` at 0xa0bc): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-72] +0x3`, value `(ret(linked ELF 0x337c) & 0xff)`, base register `r4`, raw displacement `+0x3` (`strb` at 0xa0c4).
  Call (`bl` at 0xa114): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-72] +0x4`, value `(ret(linked ELF 0x337c) & 0xff)`, base register `r4`, raw displacement `+0x4` (`strb` at 0xa11c).
  Call (`bl` at 0xa16c): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-72] +0x5`, value `(ret(linked ELF 0x337c) & 0xff)`, base register `r4`, raw displacement `+0x5` (`strb` at 0xa174).
  Call (`bl` at 0xa1c4): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-72] +0x9`, value `(ret(linked ELF 0x337c) & 0xff)`, base register `r4`, raw displacement `+0x9` (`strb` at 0xa1cc).
  Call (`bl` at 0xa21c): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[sp-72]+20`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xa26c): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`[sp-72]+32`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x44`, value `[sp-68]+1`, base register `r11`, raw displacement `-0x40` (`str` at 0xa278).
  Jump (`b` at 0xa27c): target 0x9f68.
  Four-byte store: target `sp -0x34`, value `[sp-52]+1`, base register `r11`, raw displacement `-0x30` (`str` at 0xa288).
  Jump (`b` at 0xa28c): target 0x9dcc.
  Four-byte store: target `sp -0x2c`, value `[sp-44]+1`, base register `r11`, raw displacement `-0x28` (`str` at 0xa298).
  Jump (`b` at 0xa29c): target 0x9cb0.
  At the known-edge return (`ldm` at 0xa2a4), r0 is `unknown`.
  Known return boundary (`ldm` at 0xa2a4): the modeled local edges end before the retained trailing region 0xa2a8 to 0xa2ac (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xa2a8 to 0xa2a8 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `L2CfgMCfgPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xbb24 to 0xbffc (exclusive), 1240 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `02e7f476aadd445110309fc40bd43bf3ac1c0f6693fb6c54802aef4e5d90fdf2`.
  The native facts retain 2 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xbb28).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xbb34).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xbb38).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xbb3c).
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0xbb44).
  Four-byte store: target `sp -0x24`, value `0x0`, base register `r11`, raw displacement `-0x20` (`str` at 0xbb4c).
  Four-byte store: target `sp -0x28`, value `0x0`, base register `r11`, raw displacement `-0x24` (`str` at 0xbb54).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xbb58, `ldr` at 0xbfd8.
  Four-byte store: target `sp -0x20`, value `r0`, base register `r11`, raw displacement `-0x1c` (`str` at 0xbb5c).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xbb60).
  Native warning (`rsb` at 0xbb6c): `rsb: unsupported effects invalidated`.
  Four-byte load: target `ELF load bias (unknown) +0xbff4`, base register `pc`, raw displacement `+0x478`, linked address 0xbff4, basis `ELF load-bias-relative` (`ldr` at 0xbb74).
  Native warning (`ldr` at 0xbb74): `ELF literal at linked 0xbff4 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0xbb7c).
  Grouped accesses: 23 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xbb80, `ldr` at 0xbb8c, `ldr` at 0xbba0.
  Branch (`beq` at 0xbb88): taken to 0xbf00 when `[sp-28] == 0x0`, falls through to 0xbb8c when `[sp-28] != 0x0`.
  Byte load: target `[sp-28] +0x24`, base register `r3`, raw displacement `+0x24` (`ldrb` at 0xbb90).
  Branch (`bne` at 0xbb9c): taken to 0xbbb0 when `? != 0x0`, falls through to 0xbba0 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x28`, base register `r3`, raw displacement `+0x28` (`ldr` at 0xbba4).
  Four-byte store: target `sp -0x50`, value `[[sp-28]+40]`, base register `r11`, raw displacement `-0x4c` (`str` at 0xbba8).
  Call (`bl` at 0xbbb8): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x50`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x4c` (`str` at 0xbbbc).
  Four-byte load: target `sp -0x50`, base register `r11`, raw displacement `-0x4c` (`ldr` at 0xbbc0).
  Four-byte store: target `sp -0x38`, value `[sp-80]`, base register `r11`, raw displacement `-0x34` (`str` at 0xbbc4).
  Grouped accesses: 21 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xbbc8, `ldr` at 0xbbd4, `ldr` at 0xbbe0.
  Grouped accesses: 2 4-byte load records with target `[sp-36] +0x14`, base register `r3`, raw displacement `+0x14`, including `ldr` at 0xbbcc, `ldr` at 0xbe88.
  Four-byte store: target `sp -0x2c`, value `[[sp-36]+20]`, base register `r11`, raw displacement `-0x28` (`str` at 0xbbd0).
  Grouped accesses: 3 4-byte load records with target `[sp-36] +0x18`, base register `r3`, raw displacement `+0x18`, including `ldr` at 0xbbd8, `ldr` at 0xbf78, `ldr` at 0xbf88.
  Four-byte store: target `sp -0x30`, value `[[sp-36]+24]`, base register `r11`, raw displacement `-0x2c` (`str` at 0xbbdc).
  Grouped accesses: 2 4-byte store records with target `[sp-36] +0x18`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x18`, including `str` at 0xbbe8, `str` at 0xbfa4.
  Grouped accesses: 2 4-byte store records with target `[sp-36] +0x14`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x14`, including `str` at 0xbbf4, `str` at 0xbf98.
  Four-byte store: target `[sp-36] +0x0`, value `0x2`, base register `r2`, raw displacement `+0x0` (`str` at 0xbc00).
  Grouped accesses: 2 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0xbc04, `ldr` at 0xbce4.
  Branch (`bne` at 0xbc0c): taken to 0xbce4 when `[sp-56] != 0x6`, falls through to 0xbc10 when `[sp-56] == 0x6`.
  Grouped accesses: 2 4-byte store records with target `sp -0x34`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x30`, including `str` at 0xbc14, `str` at 0xbe20.
  Grouped accesses: 13 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0xbc18, `ldr` at 0xbc28, `ldr` at 0xbc48.
  Branch (`bgt` at 0xbc20): taken to 0xbe84 when `gt flags condition (operands unknown)`, falls through to 0xbc24 when `not (gt flags condition (operands unknown))`.
  Two-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x2`, raw addressing form `[r3, #2]` (`ldrh` at 0xbc3c).
  Four-byte store: target `sp -0x3c`, value `unknown`, base register `r11`, raw displacement `-0x38` (`str` at 0xbc40).
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldrb` at 0xbc5c, `ldrb` at 0xbe40.
  Branch (`bne` at 0xbc68): taken to 0xbc90 when `? != 0x0`, falls through to 0xbc6c when `? == 0x0`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]` (`ldr` at 0xbc84).
  Four-byte store: target `sp -0x54`, value `unknown`, base register `r11`, raw displacement `-0x50` (`str` at 0xbc88).
  Call (`bl` at 0xbca0): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Four-byte store: target `sp -0x54`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x50` (`str` at 0xbca4).
  Four-byte load: target `sp -0x54`, base register `r11`, raw displacement `-0x50` (`ldr` at 0xbca8).
  Four-byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`str` at 0xbcac).
  Four-byte load: target `ELF load bias (unknown) +0xbff8`, base register `pc`, raw displacement `+0x340`, linked address 0xbff8, basis `ELF load-bias-relative` (`ldr` at 0xbcb0).
  Native warning (`ldr` at 0xbcb0): `ELF literal at linked 0xbff8 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0xbcb8, `ldr` at 0xbe28.
  Four-byte store: target `sp -0x6c`, value `[sp-60]`, base register `sp`, raw displacement `+0x0` (`str` at 0xbcbc).
  Four-byte load: target `(unknown effective address)`, base register `r12`, raw displacement `+0x0`, raw addressing form `[ip, r2, lsl #2]` (`ldr` at 0xbcc8).
  Four-byte load: target `sp -0x40`, base register `r11`, raw displacement `-0x3c` (`ldr` at 0xbccc).
  Call (`bl` at 0xbcd0): (unknown callee), target 0xb818, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xb818, conditional `false`, r0=`[sp-36]`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x34`, reported value `[sp-52]+1` in every record, base register `r11`, raw displacement `-0x30`, including `str` at 0xbcdc, `str` at 0xbe7c.
  Jump (`b` at 0xbce0): target 0xbc18.
  Branch (`bne` at 0xbcec): taken to 0xbe84 when `[sp-56] != 0x7`, falls through to 0xbcf0 when `[sp-56] == 0x7`.
  Byte load: target `[sp-28] +0x2c`, base register `r3`, raw displacement `+0x2c` (`ldrb` at 0xbcf4).
  Branch (`bne` at 0xbd00): taken to 0xbd14 when `? != 0x0`, falls through to 0xbd04 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x30`, base register `r3`, raw displacement `+0x30` (`ldr` at 0xbd08).
  Four-byte store: target `sp -0x58`, value `[[sp-28]+48]`, base register `r11`, raw displacement `-0x54` (`str` at 0xbd0c).
  Call (`bl` at 0xbd1c): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x1`, r2=`[sp-36]`, r3=`unknown`.
  Four-byte store: target `sp -0x58`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x54` (`str` at 0xbd20).
  Four-byte load: target `sp -0x58`, base register `r11`, raw displacement `-0x54` (`ldr` at 0xbd24).
  Four-byte store: target `sp -0x40`, value `[sp-88]`, base register `r11`, raw displacement `-0x3c` (`str` at 0xbd28).
  Byte load: target `[sp-28] +0x34`, base register `r3`, raw displacement `+0x34` (`ldrb` at 0xbd30).
  Branch (`bne` at 0xbd3c): taken to 0xbd50 when `? != 0x0`, falls through to 0xbd40 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x38`, base register `r3`, raw displacement `+0x38` (`ldr` at 0xbd44).
  Four-byte store: target `sp -0x5c`, value `[[sp-28]+56]`, base register `r11`, raw displacement `-0x58` (`str` at 0xbd48).
  Call (`bl` at 0xbd58): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x5c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x58` (`str` at 0xbd5c).
  Four-byte load: target `sp -0x5c`, base register `r11`, raw displacement `-0x58` (`ldr` at 0xbd60).
  Four-byte store: target `sp -0x3c`, value `[sp-92]`, base register `r11`, raw displacement `-0x38` (`str` at 0xbd64).
  Byte load: target `[sp-28] +0x3c`, base register `r3`, raw displacement `+0x3c` (`ldrb` at 0xbd6c).
  Branch (`bne` at 0xbd78): taken to 0xbd8c when `? != 0x0`, falls through to 0xbd7c when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x40`, base register `r3`, raw displacement `+0x40` (`ldr` at 0xbd80).
  Four-byte store: target `sp -0x60`, value `[[sp-28]+64]`, base register `r11`, raw displacement `-0x5c` (`str` at 0xbd84).
  Call (`bl` at 0xbd94): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x3`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x60`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x5c` (`str` at 0xbd98).
  Four-byte load: target `sp -0x60`, base register `r11`, raw displacement `-0x5c` (`ldr` at 0xbd9c).
  Four-byte store: target `sp -0x44`, value `[sp-96]`, base register `r11`, raw displacement `-0x40` (`str` at 0xbda0).
  Byte load: target `[sp-28] +0x44`, base register `r3`, raw displacement `+0x44` (`ldrb` at 0xbda8).
  Branch (`bne` at 0xbdb4): taken to 0xbdc8 when `? != 0x0`, falls through to 0xbdb8 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x48`, base register `r3`, raw displacement `+0x48` (`ldr` at 0xbdbc).
  Four-byte store: target `sp -0x64`, value `[[sp-28]+72]`, base register `r11`, raw displacement `-0x60` (`str` at 0xbdc0).
  Call (`bl` at 0xbdd0): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x4`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x64`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x60` (`str` at 0xbdd4).
  Four-byte load: target `sp -0x64`, base register `r11`, raw displacement `-0x60` (`ldr` at 0xbdd8).
  Four-byte store: target `sp -0x48`, value `[sp-100]`, base register `r11`, raw displacement `-0x44` (`str` at 0xbddc).
  Byte load: target `[sp-28] +0x4c`, base register `r3`, raw displacement `+0x4c` (`ldrb` at 0xbde4).
  Branch (`bne` at 0xbdf0): taken to 0xbe04 when `? != 0x0`, falls through to 0xbdf4 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x50`, base register `r3`, raw displacement `+0x50` (`ldr` at 0xbdf8).
  Four-byte store: target `sp -0x68`, value `[[sp-28]+80]`, base register `r11`, raw displacement `-0x64` (`str` at 0xbdfc).
  Call (`bl` at 0xbe0c): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x5`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x68`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x64` (`str` at 0xbe10).
  Four-byte load: target `sp -0x68`, base register `r11`, raw displacement `-0x64` (`ldr` at 0xbe14).
  Four-byte store: target `sp -0x4c`, value `[sp-104]`, base register `r11`, raw displacement `-0x48` (`str` at 0xbe18).
  Branch (`bge` at 0xbe30): taken to 0xbe84 when `ge flags condition (operands unknown)`, falls through to 0xbe34 when `not (ge flags condition (operands unknown))`.
  Four-byte load: target `sp -0x48`, base register `r11`, raw displacement `-0x44` (`ldr` at 0xbe34).
  Grouped accesses: 2 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0xbe48, `ldr` at 0xbe58.
  Native warning (`mul` at 0xbe4c): `mul: unsupported effects invalidated`.
  Four-byte load: target `sp -0x4c`, base register `r11`, raw displacement `-0x48` (`ldr` at 0xbe50).
  Four-byte store: target `sp -0x6c`, value `[sp-68]`, base register `sp`, raw displacement `+0x0` (`str` at 0xbe5c).
  Call (`bl` at 0xbe70): (unknown callee), target 0xb818, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xb818, conditional `false`, r0=`[sp-36]`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0xbe80): target 0xbe24.
  Grouped accesses: 2 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0xbe8c, `ldr` at 0xbe9c.
  Branch (`bne` at 0xbe94): taken to 0xbee0 when `[[sp-36]+20] != [sp-44]`, falls through to 0xbe98 when `[[sp-36]+20] == [sp-44]`.
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0xbeb8, `ldr` at 0xbee8, `ldr` at 0xbef4.
  Four-byte load: target `[sp-36] +0x18`, base register `r1`, raw displacement `+0x18` (`ldr` at 0xbebc).
  Call (`bl` at 0xbec4): (unknown callee), target 0x3610, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3610, conditional `false`, r0=`[sp-48]`, r1=`[[sp-36]+24]`, r2=`unknown`, r3=`unknown`.
  Warning class: 2 native records report `predicated effects are merged with the unexecuted path`, including `moveq` at 0xbed0, `movne` at 0xbed4.
  Four-byte store: target `sp -0x28`, value `unknown`, base register `r11`, raw displacement `-0x24` (`str` at 0xbed8).
  Grouped accesses: 2 4-byte store records with target `sp -0x28`, reported value `0x1` in every record, base register `r11`, raw displacement `-0x24`, including `str` at 0xbee4, `str` at 0xbf14.
  Branch (`beq` at 0xbef0): taken to 0xbfcc when `[sp-48] == 0x0`, falls through to 0xbef4 when `[sp-48] != 0x0`.
  Call (`bl` at 0xbef8): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]`.
  Four-byte load: target `[sp-36] +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0xbf04).
  Branch (`beq` at 0xbf0c): taken to 0xbf18 when `[[sp-36]] == 0x0`, falls through to 0xbf10 when `[[sp-36]] != 0x0`.
  Four-byte store: target `[sp-36] +0x0`, value `0x0`, base register `r2`, raw displacement `+0x0` (`str` at 0xbf20).
  Grouped accesses: 2 4-byte load records with target `[sp-36] +0xc`, base register `r3`, raw displacement `+0xc`, including `ldr` at 0xbf28, `ldr` at 0xbf38.
  Branch (`beq` at 0xbf30): taken to 0xbf40 when `[[sp-36]+12] == 0x0`, falls through to 0xbf34 when `[[sp-36]+12] != 0x0`.
  Call (`bl` at 0xbf3c): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-36]+12]`, r1=`r1`, r2=`[sp-36]`, r3=`[sp-36]`.
  Four-byte store: target `[sp-36] +0xc`, value `0x0`, base register `r2`, raw displacement `+0xc` (`str` at 0xbf48).
  Grouped accesses: 2 4-byte load records with target `[sp-36] +0x10`, base register `r3`, raw displacement `+0x10`, including `ldr` at 0xbf50, `ldr` at 0xbf60.
  Branch (`beq` at 0xbf58): taken to 0xbf68 when `[[sp-36]+16] == 0x0`, falls through to 0xbf5c when `[[sp-36]+16] != 0x0`.
  Call (`bl` at 0xbf64): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-36]+16]`, r1=`unknown`, r2=`[sp-36]`, r3=`[sp-36]`.
  Four-byte store: target `[sp-36] +0x10`, value `0x0`, base register `r2`, raw displacement `+0x10` (`str` at 0xbf70).
  Branch (`beq` at 0xbf80): taken to 0xbf90 when `[[sp-36]+24] == 0x0`, falls through to 0xbf84 when `[[sp-36]+24] != 0x0`.
  Call (`bl` at 0xbf8c): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-36]+24]`, r1=`unknown`, r2=`[sp-36]`, r3=`[sp-36]`.
  Grouped accesses: 3 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xbfa8, `ldr` at 0xbfb8, `ldr` at 0xbfbc.
  Grouped accesses: 2 4-byte load records with target `[sp-32] +0x148`, base register `r3`, raw displacement `+0x148`, including `ldr` at 0xbfac, `ldr` at 0xbfc0.
  Branch (`beq` at 0xbfb4): taken to 0xbfcc when `[[sp-32]+328] == 0x0`, falls through to 0xbfb8 when `[[sp-32]+328] != 0x0`.
  Four-byte store: target `[sp-32] +0x148`, value `[[sp-32]+328]-1`, base register `r2`, raw displacement `+0x148` (`str` at 0xbfc8).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xbfcc, `ldr` at 0xbfe4.
  Branch (`beq` at 0xbfd4): taken to 0xbfe4 when `[sp-40] == 0x0`, falls through to 0xbfd8 when `[sp-40] != 0x0`.
  Native warning (`mvn` at 0xbfdc): `mvn: unsupported effects invalidated`.
  Call (`bl` at 0xbfe0): (unknown callee), target 0x35ec, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35ec, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  `ldm` at 0xbff0 reports r0 = `[sp-40]` on the modeled known edges.
  Known return boundary (`ldm` at 0xbff0): the modeled local edges end before the retained trailing region 0xbff4 to 0xbffc (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xbff4 to 0xbff8 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `L2CfgMAddL2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x4a2c to 0x4da8 (exclusive), 892 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `a29c35caff595100dbafe94e6af6a15207c021e5a9a8bbea9934ef9baee5a787`.
  The native facts retain 5 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0x4a30).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0x4a3c).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0x4a40).
  Four-byte store: target `sp -0x20`, value `r2`, base register `r11`, raw displacement `-0x1c` (`str` at 0x4a44).
  Four-byte store: target `sp -0x24`, value `r3`, base register `r11`, raw displacement `-0x20` (`str` at 0x4a48).
  Four-byte store: target `sp -0x28`, value `0x0`, base register `r11`, raw displacement `-0x24` (`str` at 0x4a50).
  Four-byte store: target `sp -0x2c`, value `0x0`, base register `r11`, raw displacement `-0x28` (`str` at 0x4a58).
  Four-byte store: target `sp -0x30`, value `0x0`, base register `r11`, raw displacement `-0x2c` (`str` at 0x4a60).
  Four-byte load: target `ELF load bias (unknown) +0x4d94`, base register `pc`, raw displacement `+0x328`, linked address 0x4d94, basis `ELF load-bias-relative` (`ldr` at 0x4a64).
  Native warning (`ldr` at 0x4a64): `ELF literal at linked 0x4d94 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 7 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0x4a68, `ldr` at 0x4af0, `ldr` at 0x4b3c.
  Branch (`beq` at 0x4a70): taken to 0x4a94 when `? == 0x0`, falls through to 0x4a74 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4d98`, base register `pc`, raw displacement `+0x31c`, linked address 0x4d98, basis `ELF load-bias-relative` (`ldr` at 0x4a74).
  Native warning (`ldr` at 0x4a74): `ELF literal at linked 0x4d98 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 7 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x4a78, `ldr` at 0x4a94, `ldr` at 0x4aa0.
  Four-byte store: target `sp -0x48`, value `r3`, base register `sp`, raw displacement `+0x0` (`str` at 0x4a7c).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldr` at 0x4a80).
  Four-byte load: target `ELF load bias (unknown) +0x4d9c`, base register `pc`, raw displacement `+0x310`, linked address 0x4d9c, basis `ELF load-bias-relative` (`ldr` at 0x4a84).
  Native warning (`ldr` at 0x4a84): `ELF literal at linked 0x4d9c has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 7 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x4a88, `ldr` at 0x4b08, `ldr` at 0x4c90.
  Grouped accesses: 5 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x4a8c, `ldr` at 0x4adc, `ldr` at 0x4b7c.
  Call (`bl` at 0x4a90): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Branch (`beq` at 0x4a9c): taken to 0x4ab0 when `Z == 1`, falls through to 0x4aa0 when `not (Z == 1)`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x8`, raw addressing form `[r3, #8]` (`ldr` at 0x4aa4).
  Grouped accesses: 2 4-byte store records with target `sp -0x38`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x34`, including `str` at 0x4aa8, `str` at 0x4ab4.
  Grouped accesses: 5 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x4ab0, `ldr` at 0x4afc, `ldr` at 0x4b48.
  Four-byte load: target `sp -0x38`, base register `r11`, raw displacement `-0x34` (`ldr` at 0x4ab8).
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0x4abc).
  Branch (`beq` at 0x4ac8): taken to 0x4adc when `Z == 1`, falls through to 0x4acc when `not (Z == 1)`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]`, including `ldr` at 0x4ad0, `ldr` at 0x4b0c.
  Grouped accesses: 2 4-byte store records with target `sp -0x3c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x38`, including `str` at 0x4ad4, `str` at 0x4ae0.
  Four-byte load: target `sp -0x3c`, base register `r11`, raw displacement `-0x38` (`ldr` at 0x4ae4).
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0x4ae8).
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0x4da0`, linked address 0x4da0, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0x4aec, `ldr` at 0x4b38, `ldr` at 0x4be4.
  Warning class: 4 native records report `ELF literal at linked 0x4da0 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0x4aec, `ldr` at 0x4b38, `ldr` at 0x4be4.
  Branch (`bne` at 0x4b04): taken to 0x4b38 when `Z == 0`, falls through to 0x4b08 when `not (Z == 0)`.
  Grouped accesses: 5 4-byte store records with target `sp -0x30`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x4b10, `str` at 0x4b30, `str` at 0x4b60.
  Grouped accesses: 8 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x4b14, `ldr` at 0x4b54, `ldr` at 0x4b74.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0xc`, raw addressing form `[r3, #0xc]` (`ldr` at 0x4b20).
  Branch (`bne` at 0x4b50): taken to 0x4bc0 when `Z == 0`, falls through to 0x4b54 when `not (Z == 0)`.
  Four-byte load: target `ELF load bias (unknown) +0x4da4`, base register `pc`, raw displacement `+0x238`, linked address 0x4da4, basis `ELF load-bias-relative` (`ldr` at 0x4b64).
  Native warning (`ldr` at 0x4b64): `ELF literal at linked 0x4da4 has relocation, writable or unmapped bytes; value unknown`.
  Branch (`bne` at 0x4b70): taken to 0x4b94 when `? != 0x1`, falls through to 0x4b74 when `? == 0x1`.
  Grouped accesses: 2 4-byte store records with target `sp -0x40`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x3c`, including `str` at 0x4b8c, `str` at 0x4bb4.
  Call (`bl` at 0x4ba0): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x40`, base register `r11`, raw displacement `-0x3c` (`ldr` at 0x4bb8).
  Four-byte store: target `sp -0x30`, value `[sp-64]`, base register `r11`, raw displacement `-0x2c` (`str` at 0x4bbc).
  Four-byte store: target `sp -0x48`, value `unknown`, base register `sp`, raw displacement `+0x0` (`str` at 0x4bc4).
  Call (`bl` at 0x4bd8): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, conditional `false`, r0=`0x0`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x3580)`, base register `r11`, raw displacement `-0x24` (`str` at 0x4be0).
  Branch (`bne` at 0x4bfc): taken to 0x4c88 when `? != [sp-32]`, falls through to 0x4c00 when `? == [sp-32]`.
  Branch (`bge` at 0x4c08): taken to 0x4c28 when `[sp-48] >= 0x0 (signed, 32-bit)`, falls through to 0x4c0c when `[sp-48] < 0x0 (signed, 32-bit)`.
  Grouped accesses: 3 4-byte store records with target `sp -0x44`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x40`, including `str` at 0x4c14, `str` at 0x4c20, `str` at 0x4c30.
  Grouped accesses: 5 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0x4c18, `ldr` at 0x4c34, `ldr` at 0x4c40.
  Branch (`beq` at 0x4c3c): taken to 0x4c6c when `[sp-68] == 0x8`, falls through to 0x4c40 when `[sp-68] != 0x8`.
  Branch (`bhi` at 0x4c48): taken to 0x4c5c when `[sp-68] > 0x8 (unsigned, 32-bit)`, falls through to 0x4c4c when `[sp-68] <= 0x8 (unsigned, 32-bit)`.
  Branch (`beq` at 0x4c54): taken to 0x4c7c when `[sp-68] == 0x6`, falls through to 0x4c58 when `[sp-68] != 0x6`.
  Branch (`beq` at 0x4c64): taken to 0x4c6c when `[sp-68] == 0xe`, falls through to 0x4c68 when `[sp-68] != 0xe`.
  Grouped accesses: 12 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0x4c6c, `ldr` at 0x4c7c, `ldr` at 0x4c88.
  Four-byte store: target `ret(linked ELF 0x3580) +0xc`, value `0x0`, base register `r2`, raw displacement `+0xc` (`str` at 0x4c74).
  Four-byte store: target `ret(linked ELF 0x3580) +0xc`, value `0x16`, base register `r2`, raw displacement `+0xc` (`str` at 0x4c84).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x4`, raw addressing form `[r2, #4]` (`ldr` at 0x4c98).
  Call (`bl` at 0x4ca0): (unknown callee), target 0x33b8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33b8, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x33b8)`, base register `r11`, raw displacement `-0x28` (`str` at 0x4ca8).
  Grouped accesses: 4 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x4cac, `ldr` at 0x4d2c, `ldr` at 0x4d50.
  Branch (`bne` at 0x4cb4): taken to 0x4d2c when `ret(linked ELF 0x33b8) != 0x0`, falls through to 0x4cb8 when `ret(linked ELF 0x33b8) == 0x0`.
  Branch (`bne` at 0x4cd0): taken to 0x4cf4 when `? != [sp-32]`, falls through to 0x4cd4 when `? == [sp-32]`.
  Four-byte load: target `[sp-24] +0x5b0`, base register `r3`, raw displacement `+0x5b0` (`ldr` at 0x4cdc).
  Four-byte store: target `[sp-40] +0x114`, value `[[sp-24]+1456]`, base register `r2`, raw displacement `+0x114` (`str` at 0x4ce0).
  Four-byte store: target `[sp-24] +0x5b0`, value `[sp-40]`, base register `r2`, raw displacement `+0x5b0` (`str` at 0x4cec).
  Four-byte load: target `[sp-24] +0x114`, base register `r3`, raw displacement `+0x114` (`ldr` at 0x4cfc).
  Four-byte store: target `[sp-40] +0x114`, value `[[sp-24]+276]`, base register `r2`, raw displacement `+0x114` (`str` at 0x4d00).
  Four-byte store: target `[sp-24] +0x114`, value `[sp-40]`, base register `r2`, raw displacement `+0x114` (`str` at 0x4d0c).
  Grouped accesses: 4 4-byte load records with target `sp +0x0`, base register `r11`, raw displacement `+0x4`, including `ldr` at 0x4d10, `ldr` at 0x4d1c, `ldr` at 0x4d60.
  Branch (`beq` at 0x4d18): taken to 0x4d80 when `[sp] == 0x0`, falls through to 0x4d1c when `[sp] != 0x0`.
  Four-byte store: target `[sp] +0x0`, value `[sp-40]`, base register `r2`, raw displacement `+0x0` (`str` at 0x4d24).
  Four-byte load: target `ret(linked ELF 0x33b8) +0x8`, base register `r3`, raw displacement `+0x8` (`ldr` at 0x4d30).
  Branch (`beq` at 0x4d38): taken to 0x4d50 when `[ret(linked ELF 0x33b8)+8] == 0x0`, falls through to 0x4d3c when `[ret(linked ELF 0x33b8)+8] != 0x0`.
  Call (`bl` at 0x4d40): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`[ret(linked ELF 0x33b8)+8]`.
  Native warning (`mvn` at 0x4d44): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x34`, value `unknown`, base register `r11`, raw displacement `-0x30` (`str` at 0x4d48).
  Four-byte load: target `[sp-40] +0x8`, base register `r3`, raw displacement `+0x8` (`ldr` at 0x4d58).
  Four-byte store: target `ret(linked ELF 0x33b8) +0x8`, value `[[sp-40]+8]`, base register `r2`, raw displacement `+0x8` (`str` at 0x4d5c).
  Branch (`beq` at 0x4d68): taken to 0x4d78 when `[sp] == 0x0`, falls through to 0x4d6c when `[sp] != 0x0`.
  Four-byte store: target `[sp] +0x0`, value `[sp-44]`, base register `r2`, raw displacement `+0x0` (`str` at 0x4d74).
  Call (`bl` at 0x4d7c): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x34`, value `0x0`, base register `r11`, raw displacement `-0x30` (`str` at 0x4d84).
  Four-byte load: target `sp -0x34`, base register `r11`, raw displacement `-0x30` (`ldr` at 0x4d88).
  The native return (`ldm` at 0x4d90) retains r0 = `[sp-52]`.
  Known return boundary (`ldm` at 0x4d90): the modeled local edges end before the retained trailing region 0x4d94 to 0x4da8 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0x4d94 to 0x4da4 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `L2CfgMDeleteL2Peer`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x5ecc to 0x6000 (exclusive), 308 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `3620006d2180772e8b2ac1f7f01a35a0439582aec778ec080e2cfde20bdf2049`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x5ed0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x5edc).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0x5ee0).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0x5ee4).
  Four-byte store: target `sp -0x1c`, value `r0`, base register `r11`, raw displacement `-0x18` (`str` at 0x5ee8).
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x5eec, `ldr` at 0x5f14.
  Four-byte load: target `r0 +0x114`, base register `r3`, raw displacement `+0x114` (`ldr` at 0x5ef0).
  Four-byte store: target `sp -0x20`, value `[r0+276]`, base register `r11`, raw displacement `-0x1c` (`str` at 0x5ef4).
  Grouped accesses: 5 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x5ef8, `ldr` at 0x5f04, `ldr` at 0x5f18.
  Branch (`beq` at 0x5f00): taken to 0x5fec when `Z == 1`, falls through to 0x5f04 when `not (Z == 1)`.
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x5f08, `ldr` at 0x5f4c, `ldr` at 0x5fc0.
  Branch (`bne` at 0x5f10): taken to 0x5fd4 when `Z == 0`, falls through to 0x5f14 when `not (Z == 0)`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x114`, raw addressing form `[r3, #0x114]`, including `ldr` at 0x5f1c, `ldr` at 0x5fe0.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x114`, raw addressing form `[r2, #0x114]` (`str` at 0x5f20).
  Four-byte store: target `sp -0x28`, value `0x0`, base register `r11`, raw displacement `-0x24` (`str` at 0x5f28).
  Grouped accesses: 3 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0x5f2c, `ldr` at 0x5f38, `ldr` at 0x5fb0.
  Branch (`bgt` at 0x5f34): taken to 0x5fc0 when `gt flags condition (operands unknown)`, falls through to 0x5f38 when `not (gt flags condition (operands unknown))`.
  Native warning (`rsb` at 0x5f44): `rsb: unsupported effects invalidated`.
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0x5f58).
  Grouped accesses: 6 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x5f5c, `ldr` at 0x5f6c, `ldr` at 0x5f78.
  Grouped accesses: 2 4-byte load records with target `[sp-36] +0xc`, base register `r3`, raw displacement `+0xc`, including `ldr` at 0x5f60, `ldr` at 0x5f70.
  Branch (`beq` at 0x5f68): taken to 0x5f78 when `[[sp-36]+12] == 0x0`, falls through to 0x5f6c when `[[sp-36]+12] != 0x0`.
  Call (`bl` at 0x5f74): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-36]+12]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Grouped accesses: 2 4-byte load records with target `[sp-36] +0x10`, base register `r3`, raw displacement `+0x10`, including `ldr` at 0x5f7c, `ldr` at 0x5f8c.
  Branch (`beq` at 0x5f84): taken to 0x5f94 when `[[sp-36]+16] == 0x0`, falls through to 0x5f88 when `[[sp-36]+16] != 0x0`.
  Call (`bl` at 0x5f90): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-36]+16]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Grouped accesses: 2 4-byte load records with target `[sp-36] +0x18`, base register `r3`, raw displacement `+0x18`, including `ldr` at 0x5f98, `ldr` at 0x5fa8.
  Branch (`beq` at 0x5fa0): taken to 0x5fb0 when `[[sp-36]+24] == 0x0`, falls through to 0x5fa4 when `[[sp-36]+24] != 0x0`.
  Call (`bl` at 0x5fac): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-36]+24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Four-byte store: target `sp -0x28`, value `[sp-40]+1`, base register `r11`, raw displacement `-0x24` (`str` at 0x5fb8).
  Jump (`b` at 0x5fbc): target 0x5f2c.
  Call (`bl` at 0x5fc4): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x5fcc, `str` at 0x5ff0.
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0x5fd8).
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0x5fe4).
  Jump (`b` at 0x5fe8): target 0x5ef8.
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0x5ff4).
  Known-edge return (`ldm` at 0x5ffc): recorded r0 is `0x0`.

- `L2CfgInitLocal`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x4698 to 0x48b4 (exclusive), 540 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `6d56025066a7af8df110b3270e7a590d51faf7ec3a7146aaf205f8d38f97f28a`.
  The native facts retain 10 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x14`, implicit access (`push` at 0x469c).
  Four-byte store: target `sp -0x1c`, value `r0`, base register `r11`, raw displacement `-0x18` (`str` at 0x46a8).
  Four-byte store: target `sp -0x20`, value `r1`, base register `r11`, raw displacement `-0x1c` (`str` at 0x46ac).
  Four-byte store: target `sp -0x24`, value `0x0`, base register `r11`, raw displacement `-0x20` (`str` at 0x46b4).
  Four-byte load: target `ELF load bias (unknown) +0x488c`, base register `pc`, raw displacement `+0x1cc`, linked address 0x488c, basis `ELF load-bias-relative` (`ldr` at 0x46b8).
  Native warning (`ldr` at 0x46b8): `ELF literal at linked 0x488c has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0x46bc, `ldr` at 0x46cc, `ldr` at 0x46e4.
  Branch (`beq` at 0x46c4): taken to 0x46e0 when `? == 0x0`, falls through to 0x46c8 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4890`, base register `pc`, raw displacement `+0x1c0`, linked address 0x4890, basis `ELF load-bias-relative` (`ldr` at 0x46c8).
  Native warning (`ldr` at 0x46c8): `ELF literal at linked 0x4890 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x4894`, base register `pc`, raw displacement `+0x1bc`, linked address 0x4894, basis `ELF load-bias-relative` (`ldr` at 0x46d0).
  Native warning (`ldr` at 0x46d0): `ELF literal at linked 0x4894 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x46d4, `ldr` at 0x474c, `ldr` at 0x476c.
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x46d8, `ldr` at 0x4790.
  Call (`bl` at 0x46dc): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0x4898`, linked address 0x4898, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0x46e0, `ldr` at 0x4718.
  Warning class: 2 native records report `ELF literal at linked 0x4898 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0x46e0, `ldr` at 0x4718.
  Branch (`beq` at 0x46ec): taken to 0x4718 when `? == 0x0`, falls through to 0x46f0 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x489c`, base register `pc`, raw displacement `+0x1a4`, linked address 0x489c, basis `ELF load-bias-relative` (`ldr` at 0x46f0).
  Native warning (`ldr` at 0x46f0): `ELF literal at linked 0x489c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x38`, value `unknown`, base register `sp`, raw displacement `+0x0` (`str` at 0x46f4).
  Four-byte load: target `ELF load bias (unknown) +0x48a0`, base register `pc`, raw displacement `+0x19c`, linked address 0x48a0, basis `ELF load-bias-relative` (`ldr` at 0x46fc).
  Native warning (`ldr` at 0x46fc): `ELF literal at linked 0x48a0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0x4708): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x67`, r3=`0x0`.
  Native warning (`mvn` at 0x470c): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x2c`, value `unknown`, base register `r11`, raw displacement `-0x28` (`str` at 0x4710).
  Four-byte store: target `(unknown effective address)`, value `0x1`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0x4720).
  Grouped accesses: 4 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x4724, `ldr` at 0x4744, `ldr` at 0x4764.
  Grouped accesses: 3 4-byte store records with target `sp -0x24`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x20`, including `str` at 0x4730, `str` at 0x4758, `str` at 0x4780.
  Four-byte load: target `ELF load bias (unknown) +0x48a4`, base register `pc`, raw displacement `+0x168`, linked address 0x48a4, basis `ELF load-bias-relative` (`ldr` at 0x4734).
  Native warning (`ldr` at 0x4734): `ELF literal at linked 0x48a4 has relocation, writable or unmapped bytes; value unknown`.
  Branch (`bne` at 0x4740): taken to 0x4764 when `? != 0x1`, falls through to 0x4744 when `? == 0x1`.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x475c, `str` at 0x4784.
  Call (`bl` at 0x4770): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Four-byte load: target `sp -0x30`, base register `r11`, raw displacement `-0x2c` (`ldr` at 0x4788).
  Four-byte store: target `sp -0x24`, value `[sp-48]`, base register `r11`, raw displacement `-0x20` (`str` at 0x478c).
  Four-byte store: target `sp -0x38`, value `[sp-32]`, base register `sp`, raw displacement `+0x0` (`str` at 0x4794).
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0x48a8`, linked address 0x48a8, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0x4798, `ldr` at 0x47ac, `ldr` at 0x4818.
  Warning class: 5 native records report `ELF literal at linked 0x48a8 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0x4798, `ldr` at 0x47ac, `ldr` at 0x4818.
  Call (`bl` at 0x47a8): (unknown callee), target 0x3580, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3580, conditional `false`, r0=`unknown`, r1=`0x1`, r2=`[sp-28]`, r3=`[sp-48]`.
  Call (`bl` at 0x47b4): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, conditional `false`, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x47d8): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, conditional `false`, r0=`0x16`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r5`, raw displacement `+0x118`, raw addressing form `[r5, #0x118]` (`str` at 0x47e4).
  Call (`bl` at 0x47ec): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x48ac`, base register `pc`, raw displacement `+0xb0`, linked address 0x48ac, basis `ELF load-bias-relative` (`ldr` at 0x47f4).
  Native warning (`ldr` at 0x47f4): `ELF literal at linked 0x48ac has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0x47fc): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`unknown`, r1=`ret(linked ELF 0x37cc)`, r2=`unknown`, r3=`ret(linked ELF 0x37cc)`.
  Call (`bl` at 0x4804): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, conditional `false`, r0=`0x4`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x48b0`, base register `pc`, raw displacement `+0x9c`, linked address 0x48b0, basis `ELF load-bias-relative` (`ldr` at 0x480c).
  Native warning (`ldr` at 0x480c): `ELF literal at linked 0x48b0 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0x4814): (unknown callee), target 0x37d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37d8, conditional `false`, r0=`unknown`, r1=`ret(linked ELF 0x37cc)`, r2=`unknown`, r3=`ret(linked ELF 0x37cc)`.
  Call (`bl` at 0x4820): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, conditional `false`, r0=`0x9`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `(unknown effective address)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4, r1]` (`ldrb` at 0x482c).
  Grouped accesses: 2 1-byte store records with target `sp -0x34`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x30`, including `strb` at 0x4830, `strb` at 0x4844.
  Grouped accesses: 2 1-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldrb` at 0x4838, `ldrb` at 0x4848.
  Byte store: target `(unknown effective address)`, value `([sp-52]:1 & 0xff)`, base register `r4`, raw displacement `+0x0`, raw addressing form `[r4, r1]` (`strb` at 0x484c).
  Call (`bl` at 0x4858): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`0x800`, r1=`0x144`, r2=`[sp-52]:1`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x3514)`, base register `r4`, raw displacement `+0x5b4`, raw addressing form `[r4, #0x5b4]` (`str` at 0x4860).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x5b4`, raw addressing form `[r3, #0x5b4]` (`ldr` at 0x4868).
  Call (`bl` at 0x4874): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`0x0`, r2=`0x800`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `0x0`, base register `r11`, raw displacement `-0x28` (`str` at 0x487c).
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0x4880).
  At the known-edge return (`ldm` at 0x4888), r0 is `[sp-44]`.
  Known return boundary (`ldm` at 0x4888): the modeled local edges end before the retained trailing region 0x488c to 0x48b4 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0x488c to 0x48b0 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `L2CfgMSetConsole`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xeab4 to 0xed40 (exclusive), 652 bytes. Grade: unknown. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `dbe388481d7f4d2763df0a87f084b141077dc4dc55c89b3432cdc44d8f81c5e9`.
  The native facts retain 77 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xeab8).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xeac4).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xeac8).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xeacc).
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xead0, `ldr` at 0xeae4.
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0xeae0).
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0xeaf0).
  Grouped accesses: 4 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xeaf4, `ldr` at 0xeb08, `ldr` at 0xeb18.
  Native warning (`cmn` at 0xeaf8): `cmn: unsupported effects invalidated`.
  Branch (`bne` at 0xeafc): taken to 0xeb4c when `Z == 0`, falls through to 0xeb00 when `not (Z == 0)`.
  Four-byte store: target `sp -0x18`, value `0x0`, base register `r11`, raw displacement `-0x14` (`str` at 0xeb04).
  Branch (`bgt` at 0xeb10): taken to 0xeb40 when `gt flags condition (operands unknown)`, falls through to 0xeb14 when `not (gt flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xeb14, `ldr` at 0xeb4c.
  Four-byte load: target `[sp-20] +0x5b4`, base register `r1`, raw displacement `+0x5b4` (`ldr` at 0xeb20).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x4`, raw addressing form `[r2, #4]` (`str` at 0xeb2c).
  Four-byte store: target `sp -0x18`, value `[sp-24]+1`, base register `r11`, raw displacement `-0x14` (`str` at 0xeb38).
  Jump (`b` at 0xeb3c): target 0xeb08.
  Grouped accesses: 3 4-byte store records with target `sp -0x2c`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0xeb44, `str` at 0xeb78, `str` at 0xecd0.
  Jump (`b` at 0xeb48): target 0xed34.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xeb50).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xeb54).
  Call (`bl` at 0xeb5c): (unknown callee), target 0x34fc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34fc, conditional `false`, r0=`[sp-20]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`0x0`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x34fc)`, base register `r11`, raw displacement `-0x24` (`str` at 0xeb64).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xeb68, `ldr` at 0xeb80.
  Branch (`bne` at 0xeb70): taken to 0xeb80 when `ret(linked ELF 0x34fc) != 0x0`, falls through to 0xeb74 when `ret(linked ELF 0x34fc) == 0x0`.
  Jump (`b` at 0xeb7c): target 0xed34.
  Byte load: target `ret(linked ELF 0x34fc) +0x5`, base register `r3`, raw displacement `+0x5` (`ldrb` at 0xeb84).
  Four-byte load: target `(unknown effective address)`, base register `pc`, raw displacement `+0x0`, condition `[ret(linked ELF 0x34fc)+5]:1-48 <= 0x4c (unsigned, 32-bit)`, raw addressing form `[pc, r3, lsl #2]` (`ldrls` at 0xeb90).
  Branch (`ldrls` at 0xeb90): taken destination unknown when `[ret(linked ELF 0x34fc)+5]:1-48 <= 0x4c (unsigned, 32-bit)`, falls through to 0xeb94 when `[ret(linked ELF 0x34fc)+5]:1-48 > 0x4c (unsigned, 32-bit)`.
  Native warning (`ldrls` at 0xeb90): `unresolved indirect transfers: access, call and return lists cover known edges only`.
  Native warning (`ldrls` at 0xeb90): `predicated effects are merged with the unexecuted path`.
  Jump (`b` at 0xeb94): target 0xeccc.
  The decoded records from 0xeb98 to 0xecc8 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.
  Jump (`b` at 0xecd4): target 0xed34.
  The decoded records from 0xecd8 to 0xed30 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0xed34).
  `ldm` at 0xed3c reports r0 = `0x0` on the modeled known edges.

- `L2CfgMGetConsole`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xe068 to 0xe9ac (exclusive), 2372 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `91449942065bdca9e35dcfaa86ab246f00e802cb94075fa15d55bfb5c25a1b29`.
  The native facts retain 18 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x14`, implicit access (`push` at 0xe06c).
  Four-byte store: target `sp -0x1c`, value `r0`, base register `r11`, raw displacement `-0x18` (`str` at 0xe078).
  Four-byte store: target `sp -0x20`, value `r1`, base register `r11`, raw displacement `-0x1c` (`str` at 0xe07c).
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0xe964`, linked address 0xe964, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe080, `ldr` at 0xe0c0, `ldr` at 0xe14c.
  Warning class: 4 native records report `ELF literal at linked 0xe964 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe080, `ldr` at 0xe0c0, `ldr` at 0xe14c.
  Grouped accesses: 9 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xe084, `ldr` at 0xe094, `ldr` at 0xe0c4.
  Branch (`beq` at 0xe08c): taken to 0xe0a8 when `? == 0x0`, falls through to 0xe090 when `? != 0x0`.
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0xe968`, linked address 0xe968, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe090, `ldr` at 0xe0d0, `ldr` at 0xe17c.
  Warning class: 4 native records report `ELF literal at linked 0xe968 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe090, `ldr` at 0xe0d0, `ldr` at 0xe17c.
  Four-byte load: target `ELF load bias (unknown) +0xe96c`, base register `pc`, raw displacement `+0x8cc`, linked address 0xe96c, basis `ELF load-bias-relative` (`ldr` at 0xe098).
  Native warning (`ldr` at 0xe098): `ELF literal at linked 0xe96c has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 6 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xe09c, `ldr` at 0xe0f8, `ldr` at 0xe114.
  Grouped accesses: 10 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xe0a0, `ldr` at 0xe0a8, `ldr` at 0xe0b4.
  Call (`bl` at 0xe0a4): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Warning class: 4 native records report `cmn: unsupported effects invalidated`, including `cmn` at 0xe0ac, `cmn` at 0xe0f0, `cmn` at 0xe50c.
  Branch (`beq` at 0xe0b0): taken to 0xe0ec when `Z == 1`, falls through to 0xe0b4 when `not (Z == 1)`.
  Branch (`ble` at 0xe0bc): taken to 0xe0ec when `[sp-32] <= 0xff (signed, 32-bit)`, falls through to 0xe0c0 when `[sp-32] > 0xff (signed, 32-bit)`.
  Branch (`beq` at 0xe0cc): taken to 0xe0e0 when `? == 0x0`, falls through to 0xe0d0 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xe970`, base register `pc`, raw displacement `+0x890`, linked address 0xe970, basis `ELF load-bias-relative` (`ldr` at 0xe0d8).
  Native warning (`ldr` at 0xe0d8): `ELF literal at linked 0xe970 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xe0dc): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x34`, value `0x0`, base register `r11`, raw displacement `-0x30` (`str` at 0xe0e4).
  Branch (`beq` at 0xe0f4): taken to 0xe1bc when `Z == 1`, falls through to 0xe0f8 when `not (Z == 1)`.
  Grouped accesses: 4 4-byte load records with target `[sp-28] +0x5b4`, base register `r1`, raw displacement `+0x5b4`, including `ldr` at 0xe104, `ldr` at 0xe120, `ldr` at 0xe7e4.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]` (`ldr` at 0xe10c).
  Four-byte store: target `sp -0x2c`, value `unknown`, base register `r11`, raw displacement `-0x28` (`str` at 0xe110).
  Four-byte store: target `sp -0x28`, value `unknown`, base register `r11`, raw displacement `-0x24` (`str` at 0xe12c).
  Grouped accesses: 4 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0xe130, `ldr` at 0xe160, `ldr` at 0xe194.
  Branch (`bne` at 0xe138): taken to 0xe14c when `[sp-44] != 0x0`, falls through to 0xe13c when `[sp-44] == 0x0`.
  Grouped accesses: 3 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xe13c, `ldr` at 0xe170, `ldr` at 0xe1a8.
  Branch (`bne` at 0xe144): taken to 0xe14c when `[sp-40] != 0x0`, falls through to 0xe148 when `[sp-40] == 0x0`.
  Branch (`beq` at 0xe158): taken to 0xe194 when `? == 0x0`, falls through to 0xe15c when `? != 0x0`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xe974`, linked address 0xe974, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe15c, `ldr` at 0xe8a8.
  Warning class: 2 native records report `ELF literal at linked 0xe974 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe15c, `ldr` at 0xe8a8.
  Call (`bl` at 0xe164): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`[sp-44]`, r2=`unknown`, r3=`unknown`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xe978`, linked address 0xe978, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe16c, `ldr` at 0xe8c0.
  Warning class: 2 native records report `ELF literal at linked 0xe978 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe16c, `ldr` at 0xe8c0.
  Call (`bl` at 0xe174): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xe97c`, base register `pc`, raw displacement `+0x7f0`, linked address 0xe97c, basis `ELF load-bias-relative` (`ldr` at 0xe184).
  Native warning (`ldr` at 0xe184): `ELF literal at linked 0xe97c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xe190): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0x373c)`, r3=`ret(linked ELF 0x373c)`.
  Four-byte store: target `sp -0x44`, value `[sp-44]`, base register `r11`, raw displacement `-0x40` (`str` at 0xe198).
  Branch (`bne` at 0xe1a4): taken to 0xe1b0 when `[sp-44] != 0x0`, falls through to 0xe1a8 when `[sp-44] == 0x0`.
  Four-byte store: target `sp -0x44`, value `[sp-40]`, base register `r11`, raw displacement `-0x40` (`str` at 0xe1ac).
  Four-byte load: target `sp -0x44`, base register `r11`, raw displacement `-0x40` (`ldr` at 0xe1b0).
  Grouped accesses: 2 4-byte store records with target `sp -0x34`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x30`, including `str` at 0xe1b4, `str` at 0xe954.
  PC-relative loads: 8 4-byte records report target expression `ELF load bias (unknown) +0xe980`, linked address 0xe980, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe1bc, `ldr` at 0xe584, `ldr` at 0xe60c.
  Warning class: 8 native records report `ELF literal at linked 0xe980 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe1bc, `ldr` at 0xe584, `ldr` at 0xe60c.
  Call (`bl` at 0xe1c8): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`0x0`, r2=`0x400`, r3=`unknown`.
  PC-relative loads: 5 4-byte records report target expression `ELF load bias (unknown) +0xe984`, linked address 0xe984, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe1cc, `ldr` at 0xe814, `ldr` at 0xe824.
  Warning class: 5 native records report `ELF literal at linked 0xe984 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe1cc, `ldr` at 0xe814, `ldr` at 0xe824.
  Call (`bl` at 0xe1d8): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`0x0`, r2=`0x400`, r3=`unknown`.
  PC-relative loads: 7 4-byte records report target expression `ELF load bias (unknown) +0xe988`, linked address 0xe988, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe1dc, `ldr` at 0xe6b4, `ldr` at 0xe73c.
  Warning class: 7 native records report `ELF literal at linked 0xe988 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe1dc, `ldr` at 0xe6b4, `ldr` at 0xe73c.
  Call (`bl` at 0xe1e8): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`0x0`, r2=`0x400`, r3=`unknown`.
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0xe98c`, linked address 0xe98c, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe1ec, `ldr` at 0xe528, `ldr` at 0xe564.
  Warning class: 4 native records report `ELF literal at linked 0xe98c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe1ec, `ldr` at 0xe528, `ldr` at 0xe564.
  Warning class: 6 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0xe1f0, `mvn` at 0xe200, `mvn` at 0xe210.
  Call (`bl` at 0xe1f8): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x400`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xe990`, base register `pc`, raw displacement `+0x78c`, linked address 0xe990, basis `ELF load-bias-relative` (`ldr` at 0xe1fc).
  Native warning (`ldr` at 0xe1fc): `ELF literal at linked 0xe990 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xe208): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x400`, r3=`unknown`.
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0xe994`, linked address 0xe994, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe20c, `ldr` at 0xe664, `ldr` at 0xe694.
  Warning class: 4 native records report `ELF literal at linked 0xe994 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe20c, `ldr` at 0xe664, `ldr` at 0xe694.
  Call (`bl` at 0xe218): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x400`, r3=`unknown`.
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0xe998`, linked address 0xe998, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe21c, `ldr` at 0xe540, `ldr` at 0xe574.
  Warning class: 4 native records report `ELF literal at linked 0xe998 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe21c, `ldr` at 0xe540, `ldr` at 0xe574.
  Call (`bl` at 0xe228): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x400`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xe99c`, base register `pc`, raw displacement `+0x768`, linked address 0xe99c, basis `ELF load-bias-relative` (`ldr` at 0xe22c).
  Native warning (`ldr` at 0xe22c): `ELF literal at linked 0xe99c has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xe238): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x400`, r3=`unknown`.
  PC-relative loads: 4 4-byte records report target expression `ELF load bias (unknown) +0xe9a0`, linked address 0xe9a0, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xe23c, `ldr` at 0xe67c, `ldr` at 0xe6a4.
  Warning class: 4 native records report `ELF literal at linked 0xe9a0 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe23c, `ldr` at 0xe67c, `ldr` at 0xe6a4.
  Call (`bl` at 0xe248): (unknown callee), target 0x3748, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3748, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x400`, r3=`unknown`.
  Four-byte store: target `sp -0x24`, value `[sp-28]`, base register `r11`, raw displacement `-0x20` (`str` at 0xe250).
  Grouped accesses: 3 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xe254, `ldr` at 0xe288, `ldr` at 0xe7b4.
  Branch (`beq` at 0xe25c): taken to 0xe7c4 when `Z == 1`, falls through to 0xe260 when `not (Z == 1)`.
  Four-byte store: target `sp -0x38`, value `0x0`, base register `r11`, raw displacement `-0x34` (`str` at 0xe264).
  Grouped accesses: 4 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0xe268, `ldr` at 0xe274, `ldr` at 0xe27c.
  Branch (`bgt` at 0xe270): taken to 0xe7b4 when `gt flags condition (operands unknown)`, falls through to 0xe274 when `not (gt flags condition (operands unknown))`.
  Native warning (`rsb` at 0xe280): `rsb: unsupported effects invalidated`.
  Four-byte store: target `sp -0x30`, value `unknown`, base register `r11`, raw displacement `-0x2c` (`str` at 0xe294).
  Grouped accesses: 12 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0xe298, `ldr` at 0xe2b4, `ldr` at 0xe2c8.
  Four-byte load: target `[sp-48] +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0xe29c).
  Branch (`beq` at 0xe2a4): taken to 0xe2ac when `[[sp-48]] == 0x2`, falls through to 0xe2a8 when `[[sp-48]] != 0x2`.
  Four-byte store: target `sp -0x3c`, value `0x0`, base register `r11`, raw displacement `-0x38` (`str` at 0xe2b0).
  Four-byte load: target `[sp-48] +0x14`, base register `r3`, raw displacement `+0x14` (`ldr` at 0xe2b8).
  Grouped accesses: 32 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0xe2bc, `ldr` at 0xe2cc, `ldr` at 0xe2d4.
  Branch (`ble` at 0xe2c4): taken to 0xe7a4 when `le flags condition (operands unknown)`, falls through to 0xe2c8 when `not (le flags condition (operands unknown))`.
  Grouped accesses: 10 4-byte load records with target `[sp-48] +0x18`, base register `r1`, raw displacement `+0x18`, including `ldr` at 0xe2ec, `ldr` at 0xe324, `ldr` at 0xe358.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x1`, raw addressing form `[r3, #1]` (`ldrb` at 0xe2f4).
  Branch (`beq` at 0xe2fc): taken to 0xe794 when `Z == 1`, falls through to 0xe300 when `not (Z == 1)`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x3`, raw addressing form `[r3, #3]` (`ldrb` at 0xe32c).
  Four-byte store: target `sp -0x48`, value `unknown`, base register `r11`, raw displacement `-0x44` (`str` at 0xe330).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]` (`ldrb` at 0xe360).
  Four-byte store: target `sp -0x4c`, value `unknown`, base register `r11`, raw displacement `-0x48` (`str` at 0xe364).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x5`, raw addressing form `[r3, #5]` (`ldrb` at 0xe394).
  Four-byte store: target `sp -0x50`, value `unknown`, base register `r11`, raw displacement `-0x4c` (`str` at 0xe398).
  Grouped accesses: 5 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x7`, raw addressing form `[r3, #7]`, including `ldrb` at 0xe3c8, `ldrb` at 0xe5ec, `ldrb` at 0xe64c.
  Grouped accesses: 19 4-byte load records with target `sp -0x50`, base register `r11`, raw displacement `-0x4c`, including `ldr` at 0xe3cc, `ldr` at 0xe42c, `ldr` at 0xe438.
  Call (`bl` at 0xe3d4): (unknown callee), target 0xe9ac, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xe9ac, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x54`, value `ret(linked ELF 0xe9ac)`, base register `r11`, raw displacement `-0x50` (`str` at 0xe3d8).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x6`, raw addressing form `[r3, #6]` (`ldrb` at 0xe408).
  Four-byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`str` at 0xe40c).
  Grouped accesses: 5 4-byte load records with target `sp -0x48`, base register `r11`, raw displacement `-0x44`, including `ldr` at 0xe410, `ldr` at 0xe534, `ldr` at 0xe568.
  Branch (`beq` at 0xe418): taken to 0xe794 when `[sp-72] == 0x0`, falls through to 0xe41c when `[sp-72] != 0x0`.
  Grouped accesses: 5 4-byte load records with target `sp -0x4c`, base register `r11`, raw displacement `-0x48`, including `ldr` at 0xe41c, `ldr` at 0xe54c, `ldr` at 0xe578.
  Branch (`bne` at 0xe424): taken to 0xe42c when `[sp-76] != 0x0`, falls through to 0xe428 when `[sp-76] == 0x0`.
  Branch (`beq` at 0xe434): taken to 0xe508 when `[sp-80] == 0x43`, falls through to 0xe438 when `[sp-80] != 0x43`.
  Branch (`beq` at 0xe440): taken to 0xe508 when `[sp-80] == 0x33`, falls through to 0xe444 when `[sp-80] != 0x33`.
  Branch (`beq` at 0xe44c): taken to 0xe508 when `[sp-80] == 0x35`, falls through to 0xe450 when `[sp-80] != 0x35`.
  Branch (`beq` at 0xe458): taken to 0xe508 when `[sp-80] == 0x34`, falls through to 0xe45c when `[sp-80] != 0x34`.
  Branch (`beq` at 0xe464): taken to 0xe508 when `[sp-80] == 0x41`, falls through to 0xe468 when `[sp-80] != 0x41`.
  Branch (`beq` at 0xe470): taken to 0xe508 when `[sp-80] == 0x36`, falls through to 0xe474 when `[sp-80] != 0x36`.
  Branch (`beq` at 0xe47c): taken to 0xe508 when `[sp-80] == 0x39`, falls through to 0xe480 when `[sp-80] != 0x39`.
  Branch (`beq` at 0xe488): taken to 0xe508 when `[sp-80] == 0x37`, falls through to 0xe48c when `[sp-80] != 0x37`.
  Branch (`beq` at 0xe494): taken to 0xe508 when `[sp-80] == 0x38`, falls through to 0xe498 when `[sp-80] != 0x38`.
  Branch (`beq` at 0xe4a0): taken to 0xe508 when `[sp-80] == 0x30`, falls through to 0xe4a4 when `[sp-80] != 0x30`.
  Branch (`beq` at 0xe4ac): taken to 0xe508 when `[sp-80] == 0x51`, falls through to 0xe4b0 when `[sp-80] != 0x51`.
  Branch (`beq` at 0xe4b8): taken to 0xe508 when `[sp-80] == 0x40`, falls through to 0xe4bc when `[sp-80] != 0x40`.
  Branch (`beq` at 0xe4c4): taken to 0xe508 when `[sp-80] == 0x4f`, falls through to 0xe4c8 when `[sp-80] != 0x4f`.
  Branch (`beq` at 0xe4d0): taken to 0xe508 when `[sp-80] == 0x42`, falls through to 0xe4d4 when `[sp-80] != 0x42`.
  Branch (`beq` at 0xe4dc): taken to 0xe508 when `[sp-80] == 0x46`, falls through to 0xe4e0 when `[sp-80] != 0x46`.
  Branch (`beq` at 0xe4e8): taken to 0xe508 when `[sp-80] == 0x4c`, falls through to 0xe4ec when `[sp-80] != 0x4c`.
  Branch (`beq` at 0xe4f4): taken to 0xe508 when `[sp-80] == 0x4d`, falls through to 0xe4f8 when `[sp-80] != 0x4d`.
  Branch (`beq` at 0xe500): taken to 0xe508 when `[sp-80] == 0x7c`, falls through to 0xe504 when `[sp-80] != 0x7c`.
  Branch (`beq` at 0xe510): taken to 0xe528 when `Z == 1`, falls through to 0xe514 when `not (Z == 1)`.
  Grouped accesses: 34 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0xe518, `ldr` at 0xe52c, `ldr` at 0xe544.
  Branch (`beq` at 0xe520): taken to 0xe528 when `[sp-64] == [sp-32]`, falls through to 0xe524 when `[sp-64] != [sp-32]`.
  Grouped accesses: 10 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3, r1, lsl #2]`, including `ldr` at 0xe530, `ldr` at 0xe548, `ldr` at 0xe590.
  Branch (`bhi` at 0xe53c): taken to 0xe664 when `[sp-72] > ? (unsigned, 32-bit)`, falls through to 0xe540 when `[sp-72] <= ? (unsigned, 32-bit)`.
  Branch (`bhi` at 0xe554): taken to 0xe664 when `[sp-76] > ? (unsigned, 32-bit)`, falls through to 0xe558 when `[sp-76] <= ? (unsigned, 32-bit)`.
  Four-byte load: target `sp -0x54`, base register `r11`, raw displacement `-0x50` (`ldr` at 0xe558).
  Branch (`bne` at 0xe560): taken to 0xe664 when `[sp-84] != 0x4`, falls through to 0xe564 when `[sp-84] == 0x4`.
  Four-byte store: target `(unknown effective address)`, value `[sp-72]`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3, r1, lsl #2]` (`str` at 0xe570).
  Four-byte store: target `(unknown effective address)`, value `[sp-76]`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3, r1, lsl #2]` (`str` at 0xe580).
  Grouped accesses: 4 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r0`, raw displacement `+0x0`, raw addressing form `[r0, r1, lsl #2]`, including `str` at 0xe608, `str` at 0xe660, `str` at 0xe738.
  Grouped accesses: 11 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3, r2, lsl #2]`, including `ldr` at 0xe618, `ldr` at 0xe66c, `ldr` at 0xe684.
  Branch (`bhi` at 0xe678): taken to 0xe794 when `[sp-72] > ? (unsigned, 32-bit)`, falls through to 0xe67c when `[sp-72] <= ? (unsigned, 32-bit)`.
  Branch (`bhi` at 0xe690): taken to 0xe794 when `[sp-76] > ? (unsigned, 32-bit)`, falls through to 0xe694 when `[sp-76] <= ? (unsigned, 32-bit)`.
  Four-byte store: target `(unknown effective address)`, value `[sp-72]`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3, r2, lsl #2]` (`str` at 0xe6a0).
  Four-byte store: target `(unknown effective address)`, value `[sp-76]`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3, r2, lsl #2]` (`str` at 0xe6b0).
  Four-byte store: target `sp -0x3c`, value `unknown`, base register `r11`, raw displacement `-0x38` (`str` at 0xe79c).
  Jump (`b` at 0xe7a0): target 0xe2b4.
  Four-byte store: target `sp -0x38`, value `[sp-56]+1`, base register `r11`, raw displacement `-0x34` (`str` at 0xe7ac).
  Jump (`b` at 0xe7b0): target 0xe268.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x114`, raw addressing form `[r3, #0x114]` (`ldr` at 0xe7b8).
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0xe7bc).
  Jump (`b` at 0xe7c0): target 0xe254.
  Four-byte store: target `sp -0x40`, value `0x0`, base register `r11`, raw displacement `-0x3c` (`str` at 0xe7c8).
  Branch (`bgt` at 0xe7d4): taken to 0xe91c when `gt flags condition (operands unknown)`, falls through to 0xe7d8 when `not (gt flags condition (operands unknown))`.
  Four-byte store: target `sp -0x58`, value `unknown`, base register `r11`, raw displacement `-0x54` (`str` at 0xe7ec).
  Grouped accesses: 3 4-byte store records with target `sp -0x5c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x58`, including `str` at 0xe7fc, `str` at 0xe820, `str` at 0xe844.
  Branch (`bne` at 0xe810): taken to 0xe848 when `? != 0x0`, falls through to 0xe814 when `? == 0x0`.
  Branch (`bne` at 0xe834): taken to 0xe848 when `? != 0x0`, falls through to 0xe838 when `? == 0x0`.
  Four-byte load: target `sp -0x5c`, base register `r11`, raw displacement `-0x58` (`ldr` at 0xe848).
  Four-byte load: target `sp -0x58`, base register `r11`, raw displacement `-0x54` (`ldr` at 0xe84c).
  Four-byte store: target `(unknown effective address)`, value `[sp-92]`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xe850).
  Branch (`beq` at 0xe860): taken to 0xe90c when `? == 0x0`, falls through to 0xe864 when `? != 0x0`.
  Branch (`bne` at 0xe874): taken to 0xe8a4 when `? != 0x0`, falls through to 0xe878 when `? == 0x0`.
  Branch (`bne` at 0xe888): taken to 0xe8a4 when `? != 0x0`, falls through to 0xe88c when `? == 0x0`.
  Branch (`bne` at 0xe89c): taken to 0xe8a4 when `? != 0x0`, falls through to 0xe8a0 when `? == 0x0`.
  Call (`bl` at 0xe8b4): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-64]`, r3=`unknown`.
  Call (`bl` at 0xe8cc): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-64]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xe9a4`, base register `pc`, raw displacement `+0xc4`, linked address 0xe9a4, basis `ELF load-bias-relative` (`ldr` at 0xe8d8).
  Native warning (`ldr` at 0xe8d8): `ELF literal at linked 0xe9a4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xe8e4): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-64]`, r3=`unknown`.
  Four-byte store: target `sp -0x68`, value `ret(linked ELF 0x373c)`, base register `sp`, raw displacement `+0x0` (`str` at 0xe8f0).
  Four-byte store: target `sp -0x64`, value `ret(linked ELF 0x373c)`, base register `sp`, raw displacement `+0x4` (`str` at 0xe8f4).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldr` at 0xe8f8).
  Four-byte load: target `ELF load bias (unknown) +0xe9a8`, base register `pc`, raw displacement `+0xa4`, linked address 0xe9a8, basis `ELF load-bias-relative` (`ldr` at 0xe8fc).
  Native warning (`ldr` at 0xe8fc): `ELF literal at linked 0xe9a8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xe908): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[sp-64]`, r3=`ret(linked ELF 0x373c)`.
  Four-byte store: target `sp -0x40`, value `[sp-64]+1`, base register `r11`, raw displacement `-0x3c` (`str` at 0xe914).
  Jump (`b` at 0xe918): target 0xe7cc.
  Branch (`beq` at 0xe924): taken to 0xe948 when `Z == 1`, falls through to 0xe928 when `not (Z == 1)`.
  Four-byte store: target `sp -0x60`, value `unknown`, base register `r11`, raw displacement `-0x5c` (`str` at 0xe940).
  Four-byte store: target `sp -0x60`, value `0x0`, base register `r11`, raw displacement `-0x5c` (`str` at 0xe94c).
  Four-byte load: target `sp -0x60`, base register `r11`, raw displacement `-0x5c` (`ldr` at 0xe950).
  Four-byte load: target `sp -0x34`, base register `r11`, raw displacement `-0x30` (`ldr` at 0xe958).
  The native return (`ldm` at 0xe960) retains r0 = `unknown`.
  Known return boundary (`ldm` at 0xe960): the modeled local edges end before the retained trailing region 0xe964 to 0xe9ac (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xe964 to 0xe9a8 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `SysCfgConnInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xfbac to 0xfdb8 (exclusive), 524 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `9950c73c79376b9593182658b67ff810ab6b84a63be89afca17cbf8bef3d9a08`.
  The native facts retain 5 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xfbb0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xfbbc).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xfbc0).
  Grouped accesses: 9 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xfbc4, `ldr` at 0xfbe4, `ldr` at 0xfbf0.
  Branch (`bne` at 0xfbcc): taken to 0xfbe4 when `r0 != 0x0`, falls through to 0xfbd0 when `r0 == 0x0`.
  Call (`bl` at 0xfbd0): (unknown callee), target 0x3688, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3688, conditional `false`, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Branch (`bne` at 0xfbdc): taken to 0xfbe4 when `ret(linked ELF 0x3688) != 0x0`, falls through to 0xfbe0 when `ret(linked ELF 0x3688) == 0x0`.
  Branch (`beq` at 0xfbec): taken to 0xfc2c when `Z == 1`, falls through to 0xfbf0 when `not (Z == 1)`.
  Call (`bl` at 0xfbf4): (unknown callee), target 0x3760, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3760, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0xfc00): taken to 0xfc2c when `ret(linked ELF 0x3760) != 0x0`, falls through to 0xfc04 when `ret(linked ELF 0x3760) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xfda0`, base register `pc`, raw displacement `+0x194`, linked address 0xfda0, basis `ELF load-bias-relative` (`ldr` at 0xfc04).
  Native warning (`ldr` at 0xfc04): `ELF literal at linked 0xfda0 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 3 4-byte store records with target `sp -0x4c`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0xfc08, `str` at 0xfd30, `str` at 0xfd78.
  Grouped accesses: 4 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xfc0c, `ldr` at 0xfd34, `ldr` at 0xfd5c.
  Grouped accesses: 3 4-byte store records with target `sp -0x48`, reported value `[sp-24]` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0xfc10, `str` at 0xfd38, `str` at 0xfd80.
  PC-relative loads: 3 4-byte records report target expression `ELF load bias (unknown) +0xfda4`, linked address 0xfda4, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xfc18, `ldr` at 0xfd44, `ldr` at 0xfd88.
  Warning class: 3 native records report `ELF literal at linked 0xfda4 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xfc18, `ldr` at 0xfd44, `ldr` at 0xfd88.
  Call (`bl` at 0xfc24): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x26`, r3=`0x0`.
  Branch (`beq` at 0xfc34): taken to 0xfc5c when `Z == 1`, falls through to 0xfc38 when `not (Z == 1)`.
  Call (`bl` at 0xfc3c): (unknown callee), target 0x3760, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3760, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfc54): (unknown callee), target 0x34b4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34b4, conditional `false`, r0=`[sp-20]+148`, r1=`ret(linked ELF 0x3760)`, r2=`[sp-20]+148`, r3=`ret(linked ELF 0x3760)`.
  Call (`bl` at 0xfc5c): (unknown callee), target 0x3688, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3688, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xfda8`, base register `pc`, raw displacement `+0x13c`, linked address 0xfda8, basis `ELF load-bias-relative` (`ldr` at 0xfc64).
  Native warning (`ldr` at 0xfc64): `ELF literal at linked 0xfda8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xfc6c): (unknown callee), target 0x34b4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34b4, conditional `false`, r0=`unknown`, r1=`ret(linked ELF 0x3688)`, r2=`unknown`, r3=`ret(linked ELF 0x3688)`.
  Four-byte load: target `ELF load bias (unknown) +0xfdac`, base register `pc`, raw displacement `+0x134`, linked address 0xfdac, basis `ELF load-bias-relative` (`ldr` at 0xfc70).
  Four-byte store: target `sp -0x3c`, value `0x1ffff004`, base register `r11`, raw displacement `-0x38` (`str` at 0xfc74).
  Byte store: target `sp -0x38`, value `0x0`, base register `r11`, raw displacement `-0x34` (`strb` at 0xfc7c).
  Byte store: target `sp -0x37`, value `0x0`, base register `r11`, raw displacement `-0x33` (`strb` at 0xfc84).
  Four-byte store: target `sp -0x34`, value `0x0`, base register `r11`, raw displacement `-0x30` (`str` at 0xfc8c).
  Two-byte store: target `sp -0x36`, value `0x0`, base register `r11`, raw displacement `-0x32` (`strh` at 0xfc94).
  Branch (`bne` at 0xfca0): taken to 0xfcdc when `[sp-20] != 0x0`, falls through to 0xfca4 when `[sp-20] == 0x0`.
  Four-byte store: target `sp -0x4c`, value `sp-64`, base register `sp`, raw displacement `+0x0` (`str` at 0xfcb0).
  Four-byte store: target `sp -0x48`, value `0x1e`, base register `sp`, raw displacement `+0x4` (`str` at 0xfcb8).
  Call (`bl` at 0xfccc): (unknown callee), target 0x3508, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3508, conditional `false`, r0=`sp-60`, r1=`0x0`, r2=`0x1`, r3=`sp-56`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x3508)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xfcd4).
  Four-byte store: target `sp -0x4c`, value `sp-56`, base register `sp`, raw displacement `+0x0` (`str` at 0xfce4).
  Four-byte store: target `sp -0x48`, value `sp-64`, base register `sp`, raw displacement `+0x4` (`str` at 0xfcec).
  Four-byte store: target `sp -0x44`, value `0x1e`, base register `sp`, raw displacement `+0x8` (`str` at 0xfcf4).
  Call (`bl` at 0xfd08): (unknown callee), target 0x3640, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3640, conditional `false`, r0=`[sp-20]`, r1=`sp-60`, r2=`0x0`, r3=`0x1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x3640)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xfd10).
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xfd14, `ldr` at 0xfd20.
  Branch (`beq` at 0xfd1c): taken to 0xfd58 when `[sp-32] == 0x0`, falls through to 0xfd20 when `[sp-32] != 0x0`.
  Call (`bl` at 0xfd24): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]`.
  Four-byte load: target `ELF load bias (unknown) +0xfdb0`, base register `pc`, raw displacement `+0x7c`, linked address 0xfdb0, basis `ELF load-bias-relative` (`ldr` at 0xfd2c).
  Native warning (`ldr` at 0xfd2c): `ELF literal at linked 0xfdb0 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x44`, value `ret(linked ELF 0x36a0)`, base register `sp`, raw displacement `+0x8` (`str` at 0xfd3c).
  Call (`bl` at 0xfd50): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x3a`, r3=`0x0`.
  Four-byte load: target `sp -0x40`, base register `r11`, raw displacement `-0x3c` (`ldr` at 0xfd60).
  Call (`bl` at 0xfd64): (unknown callee), target 0x3790, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3790, conditional `false`, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-64]`, r3=`[sp-32]`.
  Branch (`bge` at 0xfd70): taken to 0xfd98 when `ret(linked ELF 0x3790) >= 0x0 (signed, 32-bit)`, falls through to 0xfd74 when `ret(linked ELF 0x3790) < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0xfdb4`, base register `pc`, raw displacement `+0x38`, linked address 0xfdb4, basis `ELF load-bias-relative` (`ldr` at 0xfd74).
  Native warning (`ldr` at 0xfd74): `ELF literal at linked 0xfdb4 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xfd94): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x59`, r3=`0x0`.
  Known-edge return (`ldm` at 0xfd9c): recorded r0 is `unknown`.
  Known return boundary (`ldm` at 0xfd9c): the modeled local edges end before the retained trailing region 0xfda0 to 0xfdb8 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xfda0 to 0xfdb4 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `SysCfgConnRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xfdb8 to 0xff04 (exclusive), 332 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `5c8d100f5c8f8d16615680f3b1c3f5b8467aa7480a4a7a777fef6bfe7e07c595`.
  The native facts retain 2 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xfdbc).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xfdc8).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xfdcc).
  Four-byte load: target `ELF load bias (unknown) +0xfef8`, base register `pc`, raw displacement `+0x120`, linked address 0xfef8, basis `ELF load-bias-relative` (`ldr` at 0xfdd0).
  Four-byte store: target `sp -0x30`, value `0x1ffff004`, base register `r11`, raw displacement `-0x2c` (`str` at 0xfdd4).
  Byte store: target `sp -0x2c`, value `0x0`, base register `r11`, raw displacement `-0x28` (`strb` at 0xfddc).
  Byte store: target `sp -0x2b`, value `0x0`, base register `r11`, raw displacement `-0x27` (`strb` at 0xfde4).
  Four-byte store: target `sp -0x28`, value `0x0`, base register `r11`, raw displacement `-0x24` (`str` at 0xfdec).
  Two-byte store: target `sp -0x2a`, value `0x0`, base register `r11`, raw displacement `-0x26` (`strh` at 0xfdf4).
  Grouped accesses: 3 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xfdf8, `ldr` at 0xfe74, `ldr` at 0xfe94.
  Branch (`bne` at 0xfe00): taken to 0xfe74 when `r0 != 0x0`, falls through to 0xfe04 when `r0 == 0x0`.
  Four-byte store: target `sp -0x3c`, value `0x0`, base register `sp`, raw displacement `+0x0` (`str` at 0xfe10).
  Call (`bl` at 0xfe24): (unknown callee), target 0x3478, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3478, conditional `false`, r0=`sp-48`, r1=`0x0`, r2=`0x1`, r3=`sp-44`.
  Four-byte store: target `sp -0x1c`, value `ret(linked ELF 0x3478)`, base register `r11`, raw displacement `-0x18` (`str` at 0xfe2c).
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xfe30, `ldr` at 0xfe3c, `ldr` at 0xfeb0.
  Branch (`beq` at 0xfe38): taken to 0xfe74 when `ret(linked ELF 0x3478) == 0x0`, falls through to 0xfe3c when `ret(linked ELF 0x3478) != 0x0`.
  Call (`bl` at 0xfe40): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`ret(linked ELF 0x3478)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3478)`.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xfefc`, linked address 0xfefc, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xfe48, `ldr` at 0xfec8.
  Warning class: 2 native records report `ELF literal at linked 0xfefc has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xfe48, `ldr` at 0xfec8.
  Grouped accesses: 2 4-byte store records with target `sp -0x3c`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0xfe4c, `str` at 0xfecc.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xfe50, `ldr` at 0xfed0.
  Grouped accesses: 2 4-byte store records with target `sp -0x38`, reported value `[sp-24]` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0xfe54, `str` at 0xfed4.
  Grouped accesses: 2 4-byte store records with target `sp -0x34`, reported value `ret(linked ELF 0x36a0)` in every record, base register `sp`, raw displacement `+0x8`, including `str` at 0xfe58, `str` at 0xfed8.
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0xff00`, linked address 0xff00, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0xfe60, `ldr` at 0xfee0.
  Warning class: 2 native records report `ELF literal at linked 0xff00 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xfe60, `ldr` at 0xfee0.
  Call (`bl` at 0xfe6c): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x68`, r3=`0x0`.
  Branch (`beq` at 0xfe7c): taken to 0xfef0 when `Z == 1`, falls through to 0xfe80 when `not (Z == 1)`.
  Four-byte store: target `sp -0x3c`, value `sp-44`, base register `sp`, raw displacement `+0x0` (`str` at 0xfe88).
  Four-byte store: target `sp -0x38`, value `0x0`, base register `sp`, raw displacement `+0x4` (`str` at 0xfe90).
  Call (`bl` at 0xfea4): (unknown callee), target 0x37b4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37b4, conditional `false`, r0=`unknown`, r1=`sp-48`, r2=`0x0`, r3=`0x1`.
  Four-byte store: target `sp -0x1c`, value `ret(linked ELF 0x37b4)`, base register `r11`, raw displacement `-0x18` (`str` at 0xfeac).
  Branch (`beq` at 0xfeb8): taken to 0xfef0 when `ret(linked ELF 0x37b4) == 0x0`, falls through to 0xfebc when `ret(linked ELF 0x37b4) != 0x0`.
  Call (`bl` at 0xfec0): (unknown callee), target 0x36a0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36a0, conditional `false`, r0=`ret(linked ELF 0x37b4)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x37b4)`.
  Call (`bl` at 0xfeec): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x6b`, r3=`0x0`.
  At the known-edge return (`ldm` at 0xfef4), r0 is `unknown`.
  Known return boundary (`ldm` at 0xfef4): the modeled local edges end before the retained trailing region 0xfef8 to 0xff04 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xfef8 to 0xff00 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `SysCfgConnProcessEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0xff04 to 0x1055c (exclusive), 1624 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `d7e4c51f36e3a092c87878b2cd9536a2e2f145c9913d8bdded32d9ec2730e8f7`.
  The native facts retain 9 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xff08).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xff14).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xff18).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xff1c).
  Grouped accesses: 12 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xff20, `ldr` at 0xff2c, `ldr` at 0xff58.
  Branch (`beq` at 0xff28): taken to 0xff48 when `r0 == 0x0`, falls through to 0xff2c when `r0 != 0x0`.
  Four-byte store: target `sp -0x60`, value `r0+3920`, base register `r11`, raw displacement `-0x5c` (`str` at 0xff34).
  Grouped accesses: 2 4-byte load records with target `sp -0x60`, base register `r11`, raw displacement `-0x5c`, including `ldr` at 0xff38, `ldr` at 0xff50.
  Four-byte store: target `sp -0x60`, value `r0+3924`, base register `r11`, raw displacement `-0x5c` (`str` at 0xff40).
  Four-byte load: target `ELF load bias (unknown) +0x10538`, base register `pc`, raw displacement `+0x5e8`, linked address 0x10538, basis `ELF load-bias-relative` (`ldr` at 0xff48).
  Native warning (`ldr` at 0xff48): `ELF literal at linked 0x10538 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x60`, value `unknown`, base register `r11`, raw displacement `-0x5c` (`str` at 0xff4c).
  Four-byte store: target `sp -0x34`, value `[sp-96]`, base register `r11`, raw displacement `-0x30` (`str` at 0xff54).
  Branch (`beq` at 0xff60): taken to 0xff74 when `r0 == 0x0`, falls through to 0xff64 when `r0 != 0x0`.
  Four-byte store: target `sp -0x64`, value `r0+148`, base register `r11`, raw displacement `-0x60` (`str` at 0xff6c).
  Four-byte load: target `ELF load bias (unknown) +0x1053c`, base register `pc`, raw displacement `+0x5c0`, linked address 0x1053c, basis `ELF load-bias-relative` (`ldr` at 0xff74).
  Native warning (`ldr` at 0xff74): `ELF literal at linked 0x1053c has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 7 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xff78, `ldr` at 0x102b4, `ldr` at 0x102e8.
  Four-byte store: target `sp -0x64`, value `unknown`, base register `r11`, raw displacement `-0x60` (`str` at 0xff7c).
  Four-byte load: target `sp -0x64`, base register `r11`, raw displacement `-0x60` (`ldr` at 0xff80).
  Four-byte store: target `sp -0x38`, value `[sp-100]`, base register `r11`, raw displacement `-0x34` (`str` at 0xff84).
  Branch (`beq` at 0xff90): taken to 0xffb0 when `r0 == 0x0`, falls through to 0xff94 when `r0 != 0x0`.
  Four-byte store: target `sp -0x68`, value `r0+1600`, base register `r11`, raw displacement `-0x64` (`str` at 0xff9c).
  Grouped accesses: 2 4-byte load records with target `sp -0x68`, base register `r11`, raw displacement `-0x64`, including `ldr` at 0xffa0, `ldr` at 0xffb8.
  Four-byte store: target `sp -0x68`, value `r0+1612`, base register `r11`, raw displacement `-0x64` (`str` at 0xffa8).
  Four-byte load: target `ELF load bias (unknown) +0x10540`, base register `pc`, raw displacement `+0x588`, linked address 0x10540, basis `ELF load-bias-relative` (`ldr` at 0xffb0).
  Native warning (`ldr` at 0xffb0): `ELF literal at linked 0x10540 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x68`, value `unknown`, base register `r11`, raw displacement `-0x64` (`str` at 0xffb4).
  Four-byte store: target `sp -0x3c`, value `[sp-104]`, base register `r11`, raw displacement `-0x38` (`str` at 0xffbc).
  Branch (`beq` at 0xffc8): taken to 0xffdc when `r0 == 0x0`, falls through to 0xffcc when `r0 != 0x0`.
  Four-byte store: target `sp -0x6c`, value `r0+3664`, base register `r11`, raw displacement `-0x68` (`str` at 0xffd4).
  Four-byte load: target `ELF load bias (unknown) +0x10544`, base register `pc`, raw displacement `+0x560`, linked address 0x10544, basis `ELF load-bias-relative` (`ldr` at 0xffdc).
  Native warning (`ldr` at 0xffdc): `ELF literal at linked 0x10544 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x6c`, value `unknown`, base register `r11`, raw displacement `-0x68` (`str` at 0xffe0).
  Four-byte load: target `sp -0x6c`, base register `r11`, raw displacement `-0x68` (`ldr` at 0xffe4).
  Four-byte store: target `sp -0x40`, value `[sp-108]`, base register `r11`, raw displacement `-0x3c` (`str` at 0xffe8).
  Branch (`beq` at 0xfff4): taken to 0x10008 when `r0 == 0x0`, falls through to 0xfff8 when `r0 != 0x0`.
  Four-byte store: target `sp -0x70`, value `r0+1616`, base register `r11`, raw displacement `-0x6c` (`str` at 0x10000).
  Four-byte load: target `ELF load bias (unknown) +0x10548`, base register `pc`, raw displacement `+0x538`, linked address 0x10548, basis `ELF load-bias-relative` (`ldr` at 0x10008).
  Native warning (`ldr` at 0x10008): `ELF literal at linked 0x10548 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x70`, value `unknown`, base register `r11`, raw displacement `-0x6c` (`str` at 0x1000c).
  Four-byte load: target `sp -0x70`, base register `r11`, raw displacement `-0x6c` (`ldr` at 0x10010).
  Four-byte store: target `sp -0x44`, value `[sp-112]`, base register `r11`, raw displacement `-0x40` (`str` at 0x10014).
  Branch (`beq` at 0x10020): taken to 0x10040 when `r0 == 0x0`, falls through to 0x10024 when `r0 != 0x0`.
  Four-byte store: target `sp -0x74`, value `r0+3664`, base register `r11`, raw displacement `-0x70` (`str` at 0x1002c).
  Grouped accesses: 2 4-byte load records with target `sp -0x74`, base register `r11`, raw displacement `-0x70`, including `ldr` at 0x10030, `ldr` at 0x10048.
  Four-byte store: target `sp -0x74`, value `r0+3668`, base register `r11`, raw displacement `-0x70` (`str` at 0x10038).
  Four-byte load: target `ELF load bias (unknown) +0x1054c`, base register `pc`, raw displacement `+0x504`, linked address 0x1054c, basis `ELF load-bias-relative` (`ldr` at 0x10040).
  Native warning (`ldr` at 0x10040): `ELF literal at linked 0x1054c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x74`, value `unknown`, base register `r11`, raw displacement `-0x70` (`str` at 0x10044).
  Four-byte store: target `sp -0x48`, value `[sp-116]`, base register `r11`, raw displacement `-0x44` (`str` at 0x1004c).
  Grouped accesses: 11 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x10050, `ldr` at 0x100a8, `ldr` at 0x100bc.
  Byte load: target `r2 +0x13`, base register `r3`, raw displacement `+0x13` (`ldrb` at 0x10054).
  Four-byte store: target `sp -0x2c`, value `[r2+19]:1`, base register `r11`, raw displacement `-0x28` (`str` at 0x10058).
  Grouped accesses: 2 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x1005c, `ldr` at 0x10068.
  Branch (`beq` at 0x10064): taken to 0x100a8 when `[r2+19]:1 == 0x0`, falls through to 0x10068 when `[r2+19]:1 != 0x0`.
  Call (`bl` at 0x1006c): (unknown callee), target 0x3664, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3664, conditional `false`, r0=`[r2+19]:1`, r1=`r1`, r2=`r2`, r3=`[r2+19]:1`.
  Four-byte load: target `ELF load bias (unknown) +0x10550`, base register `pc`, raw displacement `+0x4d4`, linked address 0x10550, basis `ELF load-bias-relative` (`ldr` at 0x10074).
  Native warning (`ldr` at 0x10074): `ELF literal at linked 0x10550 has relocation, writable or unmapped bytes; value unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x8c`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0x10078, `str` at 0x1017c.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x1007c, `ldr` at 0x10180.
  Grouped accesses: 2 4-byte store records with target `sp -0x88`, reported value `[sp-24]` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0x10080, `str` at 0x10184.
  Four-byte store: target `sp -0x84`, value `ret(linked ELF 0x3664)`, base register `sp`, raw displacement `+0x8` (`str` at 0x10084).
  PC-relative loads: 2 4-byte records report target expression `ELF load bias (unknown) +0x10554`, linked address 0x10554, basis `ELF load-bias-relative`, operation `ldr`. Raw displacements differ. Locators include `ldr` at 0x1008c, `ldr` at 0x1018c.
  Warning class: 2 native records report `ELF literal at linked 0x10554 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0x1008c, `ldr` at 0x1018c.
  Call (`bl` at 0x10098): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x83`, r3=`0x0`.
  Warning class: 2 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0x1009c, `mvn` at 0x1019c.
  Grouped accesses: 2 4-byte store records with target `sp -0x5c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x58`, including `str` at 0x100a0, `str` at 0x101a0.
  Byte load: target `r2 +0x24`, base register `r3`, raw displacement `+0x24` (`ldrb` at 0x100ac).
  Branch (`bne` at 0x100b8): taken to 0x100cc when `? != 0x0`, falls through to 0x100bc when `? == 0x0`.
  Four-byte load: target `r2 +0x28`, base register `r3`, raw displacement `+0x28` (`ldr` at 0x100c0).
  Four-byte store: target `sp -0x78`, value `[r2+40]`, base register `r11`, raw displacement `-0x74` (`str` at 0x100c4).
  Call (`bl` at 0x100d4): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`r2`, r1=`0x0`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x78`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x74` (`str` at 0x100d8).
  Four-byte load: target `sp -0x78`, base register `r11`, raw displacement `-0x74` (`ldr` at 0x100dc).
  Four-byte store: target `sp -0x24`, value `[sp-120]`, base register `r11`, raw displacement `-0x20` (`str` at 0x100e0).
  Byte load: target `[sp-28] +0x2c`, base register `r3`, raw displacement `+0x2c` (`ldrb` at 0x100e8).
  Branch (`bne` at 0x100f4): taken to 0x10108 when `? != 0x0`, falls through to 0x100f8 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x30`, base register `r3`, raw displacement `+0x30` (`ldr` at 0x100fc).
  Four-byte store: target `sp -0x7c`, value `[[sp-28]+48]`, base register `r11`, raw displacement `-0x78` (`str` at 0x10100).
  Call (`bl` at 0x10110): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x7c`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x78` (`str` at 0x10114).
  Four-byte load: target `sp -0x7c`, base register `r11`, raw displacement `-0x78` (`ldr` at 0x10118).
  Four-byte store: target `sp -0x28`, value `[sp-124]`, base register `r11`, raw displacement `-0x24` (`str` at 0x1011c).
  Byte load: target `[sp-28] +0x34`, base register `r3`, raw displacement `+0x34` (`ldrb` at 0x10124).
  Branch (`bne` at 0x10130): taken to 0x10144 when `? != 0x0`, falls through to 0x10134 when `? == 0x0`.
  Four-byte load: target `[sp-28] +0x38`, base register `r3`, raw displacement `+0x38` (`ldr` at 0x10138).
  Four-byte store: target `sp -0x80`, value `[[sp-28]+56]`, base register `r11`, raw displacement `-0x7c` (`str` at 0x1013c).
  Call (`bl` at 0x1014c): (unknown callee), target 0x35b0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35b0, conditional `false`, r0=`[sp-28]`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x80`, value `ret(linked ELF 0x35b0)`, base register `r11`, raw displacement `-0x7c` (`str` at 0x10150).
  Four-byte load: target `sp -0x80`, base register `r11`, raw displacement `-0x7c` (`ldr` at 0x10154).
  Four-byte store: target `sp -0x30`, value `[sp-128]`, base register `r11`, raw displacement `-0x2c` (`str` at 0x10158).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0x1015c, `ldr` at 0x1016c.
  Branch (`bgt` at 0x10164): taken to 0x1016c when `[sp-40] > 0x1 (signed, 32-bit)`, falls through to 0x10168 when `[sp-40] <= 0x1 (signed, 32-bit)`.
  Branch (`ble` at 0x10174): taken to 0x101a8 when `[sp-40] <= 0x2 (signed, 32-bit)`, falls through to 0x10178 when `[sp-40] > 0x2 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x10558`, base register `pc`, raw displacement `+0x3d8`, linked address 0x10558, basis `ELF load-bias-relative` (`ldr` at 0x10178).
  Native warning (`ldr` at 0x10178): `ELF literal at linked 0x10558 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0x10198): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, conditional `false`, r0=`0x3`, r1=`unknown`, r2=`0x9e`, r3=`0x0`.
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x101a8, `ldr` at 0x101d8, `ldr` at 0x10204.
  Branch (`bne` at 0x101b0): taken to 0x101c0 when `[sp-128] != 0x0`, falls through to 0x101b4 when `[sp-128] == 0x0`.
  Grouped accesses: 3 4-byte store records with target `sp -0x5c`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x58`, including `str` at 0x101b8, `str` at 0x101f0, `str` at 0x10528.
  Grouped accesses: 4 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x101c0, `ldr` at 0x101d0, `ldr` at 0x101f8.
  Grouped accesses: 3 4-byte load records with target `[sp-52] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x101c4, `ldr` at 0x101d4, `ldr` at 0x101fc.
  Branch (`beq` at 0x101cc): taken to 0x10204 when `[[sp-52]] == 0x0`, falls through to 0x101d0 when `[[sp-52]] != 0x0`.
  Call (`bl` at 0x101dc): (unknown callee), target 0x3418, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3418, conditional `false`, r0=`[[sp-52]]`, r1=`[sp-128]`, r2=`unknown`, r3=`[sp-52]`.
  Branch (`bne` at 0x101e8): taken to 0x101f8 when `ret(linked ELF 0x3418) != 0x0`, falls through to 0x101ec when `ret(linked ELF 0x3418) == 0x0`.
  Call (`bl` at 0x10200): (unknown callee), target 0x3724, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3724, conditional `false`, r0=`[[sp-52]]`, r1=`unknown`, r2=`unknown`, r3=`[sp-52]`.
  Call (`bl` at 0x10208): (unknown callee), target 0x33d0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33d0, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `[sp-52] +0x0`, value `ret(linked ELF 0x33d0)`, base register `r3`, raw displacement `+0x0` (`str` at 0x10218).
  Grouped accesses: 2 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0x1021c, `ldr` at 0x10240.
  Call (`bl` at 0x10224): (unknown callee), target 0x36b8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36b8, conditional `false`, r0=`[sp-56]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 5 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0x10228, `ldr` at 0x103dc, `ldr` at 0x1047c.
  Four-byte store: target `[sp-60] +0x0`, value `0x0`, base register `r2`, raw displacement `+0x0` (`str` at 0x10230).
  Grouped accesses: 3 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0x10234, `ldr` at 0x10254, `ldr` at 0x10508.
  Four-byte store: target `[sp-64] +0x0`, value `0x0`, base register `r2`, raw displacement `+0x0` (`str` at 0x1023c).
  Four-byte store: target `sp -0x20`, value `[sp-56]`, base register `r11`, raw displacement `-0x1c` (`str` at 0x10244).
  Grouped accesses: 8 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x10248, `ldr` at 0x1026c, `ldr` at 0x10290.
  Branch (`beq` at 0x10250): taken to 0x104ec when `Z == 1`, falls through to 0x10254 when `not (Z == 1)`.
  Four-byte load: target `[sp-64] +0x0`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x10258).
  Grouped accesses: 2 4-byte load records with target `sp -0x48`, base register `r11`, raw displacement `-0x44`, including `ldr` at 0x10264, `ldr` at 0x10514.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`str` at 0x10270).
  Four-byte store: target `[sp-64] +0x0`, value `[[sp-64]]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0x10278).
  Four-byte store: target `sp -0x4c`, value `0x0`, base register `r11`, raw displacement `-0x48` (`str` at 0x10280).
  Grouped accesses: 7 4-byte load records with target `sp -0x4c`, base register `r11`, raw displacement `-0x48`, including `ldr` at 0x10284, `ldr` at 0x10294, `ldr` at 0x102c8.
  Branch (`bgt` at 0x1028c): taken to 0x104dc when `gt flags condition (operands unknown)`, falls through to 0x10290 when `not (gt flags condition (operands unknown))`.
  Warning class: 5 native records report `rsb: unsupported effects invalidated`, including `rsb` at 0x102a4, `rsb` at 0x102d8, `rsb` at 0x10314.
  Branch (`bne` at 0x102bc): taken to 0x102c4 when `? != 0x0`, falls through to 0x102c0 when `? == 0x0`.
  Branch (`beq` at 0x102f0): taken to 0x102f8 when `? == 0x2`, falls through to 0x102f4 when `? != 0x2`.
  Four-byte store: target `sp -0x50`, value `0x0`, base register `r11`, raw displacement `-0x4c` (`str` at 0x102fc).
  Grouped accesses: 4 4-byte load records with target `sp -0x50`, base register `r11`, raw displacement `-0x4c`, including `ldr` at 0x10328, `ldr` at 0x10358, `ldr` at 0x103ac.
  Branch (`ble` at 0x10330): taken to 0x104cc when `? <= [sp-80] (signed, 32-bit)`, falls through to 0x10334 when `? > [sp-80] (signed, 32-bit)`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]`, including `ldr` at 0x10374, `ldr` at 0x103c8.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x1`, raw addressing form `[r3, #1]` (`ldrb` at 0x1037c).
  Branch (`beq` at 0x10384): taken to 0x104bc when `? == 0x0`, falls through to 0x10388 when `? != 0x0`.
  Four-byte store: target `sp -0x58`, value `unknown`, base register `r11`, raw displacement `-0x54` (`str` at 0x103d0).
  Four-byte store: target `sp -0x54`, value `0x0`, base register `r11`, raw displacement `-0x50` (`str` at 0x103d8).
  Grouped accesses: 3 4-byte load records with target `[sp-60] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x103e0, `ldr` at 0x10480, `ldr` at 0x104f0.
  Grouped accesses: 6 4-byte load records with target `sp -0x54`, base register `r11`, raw displacement `-0x50`, including `ldr` at 0x103e4, `ldr` at 0x103f0, `ldr` at 0x10418.
  Branch (`ble` at 0x103ec): taken to 0x1047c when `le flags condition (operands unknown)`, falls through to 0x103f0 when `not (le flags condition (operands unknown))`.
  Grouped accesses: 5 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0x103f8, `ldr` at 0x10420, `ldr` at 0x10448.
  Grouped accesses: 4 4-byte load records with target `sp -0x58`, base register `r11`, raw displacement `-0x54`, including `ldr` at 0x10404, `ldr` at 0x1042c, `ldr` at 0x10454.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x3`, raw addressing form `[r3, #3]` (`ldrb` at 0x10408).
  Byte load: target `[sp-88] +0x3`, base register `r2`, raw displacement `+0x3` (`ldrb` at 0x1040c).
  Branch (`bne` at 0x10414): taken to 0x1046c when `Z == 0`, falls through to 0x10418 when `not (Z == 0)`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]` (`ldrb` at 0x10430).
  Byte load: target `[sp-88] +0x4`, base register `r2`, raw displacement `+0x4` (`ldrb` at 0x10434).
  Branch (`bne` at 0x1043c): taken to 0x1046c when `Z == 0`, falls through to 0x10440 when `not (Z == 0)`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x5`, raw addressing form `[r3, #5]` (`ldrb` at 0x10458).
  Byte load: target `[sp-88] +0x5`, base register `r2`, raw displacement `+0x5` (`ldrb` at 0x1045c).
  Branch (`bne` at 0x10464): taken to 0x1046c when `Z == 0`, falls through to 0x10468 when `not (Z == 0)`.
  Four-byte store: target `sp -0x54`, value `unknown`, base register `r11`, raw displacement `-0x50` (`str` at 0x10474).
  Jump (`b` at 0x10478): target 0x103dc.
  Branch (`ble` at 0x1048c): taken to 0x10494 when `le flags condition (operands unknown)`, falls through to 0x10490 when `not (le flags condition (operands unknown))`.
  Four-byte load: target `[sp-60] +0x0`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x10498).
  Four-byte store: target `(unknown effective address)`, value `[sp-88]`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`str` at 0x104b0).
  Four-byte store: target `[sp-60] +0x0`, value `[[sp-60]]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0x104b8).
  Four-byte store: target `sp -0x50`, value `[sp-80]+1`, base register `r11`, raw displacement `-0x4c` (`str` at 0x104c4).
  Jump (`b` at 0x104c8): target 0x10300.
  Four-byte store: target `sp -0x4c`, value `[sp-76]+1`, base register `r11`, raw displacement `-0x48` (`str` at 0x104d4).
  Jump (`b` at 0x104d8): target 0x10284.
  Four-byte load: target `[sp-32] +0x114`, base register `r3`, raw displacement `+0x114` (`ldr` at 0x104e0).
  Four-byte store: target `sp -0x20`, value `[[sp-32]+276]`, base register `r11`, raw displacement `-0x1c` (`str` at 0x104e4).
  Jump (`b` at 0x104e8): target 0x10248.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]`, including `str` at 0x10504, `str` at 0x10520.
  Four-byte load: target `[sp-64] +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x1050c).
  Four-byte load: target `sp -0x5c`, base register `r11`, raw displacement `-0x58` (`ldr` at 0x1052c).
  `ldm` at 0x10534 reports r0 = `[sp-92]` on the modeled known edges.
  Known return boundary (`ldm` at 0x10534): the modeled local edges end before the retained trailing region 0x10538 to 0x1055c (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0x10538 to 0x10558 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

## `L2CfgMSetConsole`: the console routing table

Read in `/stand/sysco/lib/libmain.armlinux.so` of package `6.5.30:l1#4` (1.44.0, ARM), `0xeab4` to `0xed40`, with the jump table at `0xeb98` read from the file bytes (the 77 relocation records of the span are the table's words, which already hold the linked targets). Established.

- **Arguments.** `cfg` (the configuration object), `console` (a number, or -1), and `addr` (a brick address word). In `addr`, bits 27:18 are the rack, bits 17:12 the slot (the same packing the L2 power queue uses, [../l2/power-queue.md](../l2/power-queue.md)).
- **Console -1.** Clears the second word of each of 256 eight-byte entries of the table whose base pointer is at `cfg + 0x5b4`, and returns 0.
- **Otherwise.** It looks the brick up with `L2CfgMFindBrickCfg(cfg, rack, slot, 0)`; no such brick returns 0. It then dispatches on the brick's type character (byte 5 of the brick record, minus `0x30`, a 77-entry table, `0xeb90`). Two targets exist. For the type characters `0 3 4 5 6 7 8 9 @ A B C F L M O Q |` the routine requires byte 6 of the brick record to equal `console` (else it returns 0), stores `addr` in the second word of entry `console mod 256` and returns `addr`. For every other character in the range (`1 2 : ; < = > ? D E G H I J K N P R S T U V W X Y Z [ \ ] ^ _` and the lowercase letters and `{`), it returns 0 and changes nothing. Characters outside the table's range (below `0` or above `|`) also return 0 through the bounds test at `0xeb8c`.
- **Reading.** The table maps a console number to the address of the compute brick that owns it, and only compute-class bricks (the same type characters the L2 sorts into its compute class) can own one; the brick must itself report that console number. The first word of each entry is not touched here.

The earlier "unknown" status came from the indexed `pc` load of the type dispatch having no taken destination in the native model; its two destinations are `0xecd8` (store) and `0xeccc` (return 0).

