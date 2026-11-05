# ARM library NVRAM: unresolved dispatch

These are bounded unknown findings for `nvramSet` and `nvramGet` in
`6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4`, version `1.44.0`, inner
`/stand/sysco/lib/libmain.armlinux.so`. The full ELF symbol spans were read, including
every decoded record and bounded neighboring context. Symbol metadata supplies inferred
analysis spans, without proving complete code-only routine extents. Grade and hardware
role remain unknown.

Each routine reaches an indexed load into PC whose taken destination is unresolved. The
effects below cover known local edges only. They are not complete access, call or return
lists for all paths. Records outside those edges remain in the full facts and are not
established as dead, data or globally nonexecuting. A visible return on a known path
does not settle the missing taken paths. No destination is inferred from nearby words or
a null target.

Instructions and direct call targets use linked ELF virtual coordinates, with lowest
linked load `0x0`. Runtime ELF load bias is unknown. Relocations are not applied, and
relocation-dependent or writable literal values stay unknown. Raw base registers and
displacements remain distinct from resolved effective addresses. A null target does not
mean address zero, and a raw displacement of zero does not resolve the indexed address.

Calls retain four candidate AAPCS register slots, with actual arity unknown. Bare
register names denote entry values, and `sp` is the entry stack pointer. A lone `?` is
written as `unknown`. Compound native expressions remain unchanged. Unsupported effects
are not repaired from raw instruction operands. Conditions retain both native
predicates, including opaque flag expressions.

Access summaries group only identical native semantic fields, including width, base, raw
displacement, value, condition, implicit status and origin or coordinate metadata.
Counts refer to static records, without merging branch arms or asserting equal dynamic
values. Only independently paired frame saves and restores are omitted. Initial spills,
unmatched frame-like records, incoming reloads and local or ambiguous state accesses
remain.

Routes tried: complete native analysis of both ARM ELF spans, full raw instruction and
context reads, symbol and relocation metadata, and examination of the instruction
analysis limitations. The taken destinations and their subsequent effects remain
unresolved. These bounded attempts make no claim that every source was exhausted.

The [open question](../open-questions.md#arm-library-nvram-dispatch) records the remaining software route.

The [complete facts](disasm/arm-1.44.0-libraries/nvram-unresolved-dispatch-facts.txt) retain every emitted field, decoded record, relocation and warning for both full symbol spans.

- `nvramSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x10e08 to 0x11288 (exclusive), 1152 bytes. Grade: unknown. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `0a176eaa49a109f221aeb56b0cd97fa78953a85f7ba5b5ab70d34b4affed9dcf`.
  The native facts retain 28 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x14`, implicit access (`push` at 0x10e0c).
  Four-byte store: target `sp -0x1c`, value `r0`, base register `r11`, raw displacement `-0x18` (`str` at 0x10e18).
  Four-byte store: target `sp -0x20`, value `r1`, base register `r11`, raw displacement `-0x1c` (`str` at 0x10e1c).
  Four-byte load: target `ELF load bias (unknown) +0x11274`, base register `pc`, raw displacement `+0x44c`, linked address 0x11274, basis `ELF load-bias-relative` (`ldr` at 0x10e20).
  Native warning (`ldr` at 0x10e20): `ELF literal at linked 0x11274 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x10e24).
  Branch (`bne` at 0x10e30): taken to 0x10e38 when `? != 0x0`, falls through to 0x10e34 when `? == 0x0`.
  Jump (`b` at 0x10e34): target 0x1126c.
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0x10e38).
  Four-byte load: target `(unknown effective address)`, base register `pc`, raw displacement `+0x0`, condition `r0-1 <= 0x18 (unsigned, 32-bit)` (`ldrls` at 0x10e44).
  Branch (`ldrls` at 0x10e44): taken destination unknown when `r0-1 <= 0x18 (unsigned, 32-bit)`, falls through to 0x10e48 when `r0-1 > 0x18 (unsigned, 32-bit)`.
  Native warning (`ldrls` at 0x10e44): `predicated effects are merged with the unexecuted path`.
  Native warning (`ldrls` at 0x10e44): `unresolved indirect transfers: access, call and return lists cover known edges only`.
  Jump (`b` at 0x10e48): target 0x1126c.
  The decoded records from 0x10e4c to 0x11268 are outside the modeled known local edges from the entry. Their effects are not listed, and the unresolved transfer prevents an all-path conclusion.
  Known-edge return (`ldm` at 0x11270): recorded r0 is `r0`. This does not settle the unresolved taken destination.
  The decoded records from 0x11274 to 0x11284 are outside the modeled known local edges from the entry. Their effects are not listed, and the unresolved transfer prevents an all-path conclusion.

- `nvramGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x11288 to 0x11574 (exclusive), 748 bytes. Grade: unknown. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `8ca9e69b0d726c02680c79c8189b06e7d063cf4d50a77a128d4ff4bceb43a7ea`.
  The native facts retain 27 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x1128c).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x11298).
  Four-byte load: target `ELF load bias (unknown) +0x1156c`, base register `pc`, raw displacement `+0x2c8`, linked address 0x1156c, basis `ELF load-bias-relative` (`ldr` at 0x1129c).
  Native warning (`ldr` at 0x1129c): `ELF literal at linked 0x1156c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x112a0).
  Branch (`bne` at 0x112ac): taken to 0x112b8 when `? != 0x0`, falls through to 0x112b0 when `? == 0x0`.
  Call (`bl` at 0x112b4): (unknown callee), target 0x355c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x355c, r0=`0x1`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0x112b8).
  Four-byte load: target `(unknown effective address)`, base register `pc`, raw displacement `+0x0`, condition `ls flags condition (operands unknown)` (`ldrls` at 0x112c4).
  Branch (`ldrls` at 0x112c4): taken destination unknown when `ls flags condition (operands unknown)`, falls through to 0x112c8 when `not (ls flags condition (operands unknown))`.
  Native warning (`ldrls` at 0x112c4): `predicated effects are merged with the unexecuted path`.
  Native warning (`ldrls` at 0x112c4): `unresolved indirect transfers: access, call and return lists cover known edges only`.
  Jump (`b` at 0x112c8): target 0x11558.
  The decoded records from 0x112cc to 0x11554 are outside the modeled known local edges from the entry. Their effects are not listed, and the unresolved transfer prevents an all-path conclusion.
  Native warning (`mvn` at 0x11558): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x18`, value `unknown`, base register `r11`, raw displacement `-0x14` (`str` at 0x1155c).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0x11560).
  Known-edge return (`ldm` at 0x11568): recorded r0 is `[sp-24]`. This does not settle the unresolved taken destination.
  The decoded records from 0x1156c to 0x11570 are outside the modeled known local edges from the entry. Their effects are not listed, and the unresolved transfer prevents an all-path conclusion.
