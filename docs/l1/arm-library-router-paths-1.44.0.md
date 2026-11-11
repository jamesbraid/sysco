# ARM router library windows

These entries read selected full symbol spans in `6.5.30/usr/cpu/firmware/sysco/l1.bin`,
package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
Symbol metadata supplies inferred analysis spans, without proving complete code-only
function extents. Names supply no hardware role. The scope is this ARM L1 library, not
PowerPC L2 firmware or another version.

The observations use the native intraprocedural known-local model, with every callee
opaque. `RouterInput` includes a recognized opaque call whose target, canonical target,
execution state and coordinate basis remain unknown. An empty native unresolved-transfer
list does not resolve that callee or prove its runtime behavior. The native
`ret(unknown)` expression remains distinct from scalar unknown.

Instruction and direct call coordinates are linked ELF virtual addresses, with lowest
linked load `0x0`. Runtime load bias is unknown. Data references retain their native
coordinate domain. No runtime constant or linked address identifies a physical address
or pointee. Relocations are not applied, and relocation-sensitive or writable literal
values remain unknown.

Calls retain native callee and target metadata, conditional status and every one of the
four candidate AAPCS core-register slots. Actual arity and custom conventions remain
unknown. Bare register names denote entry values, and `sp` denotes the entry stack
pointer. A lone `?` is written as `unknown`. Compound native expressions remain
unchanged, and unsupported effects are not repaired from raw operands.

Repeated access groups require the same full native fields and encoded instruction form,
including operation, width, base, raw displacement, value, condition, coordinate
metadata, instruction bytes and operands. The count is static. It does not assert equal
dynamic values, combine branch arms or identify a single pointee behind unknown-address
records. Raw addressing forms are not resolved offsets. Only proved paired frame saves
and restores are omitted. All entry spills, incoming reloads, outgoing and local state,
unmatched frame-like records and ambiguous accesses remain.

Every deciding branch keeps both native predicates. The prose includes selected
unconditional jumps at loopbacks, retained raw-region crossings and known boundaries.
Any out-of-span transfer is retained. Other ordinary forward transfers remain in the
full facts. Known returns do not establish complete routine extents or runtime
feasibility.

All raw gaps and decoded records outside known local edges remain in the original full
spans. They are not classified as padding, dead data or globally nonexecuting. A word
that fails instruction decoding can still have a native read-only scalar value, which
remains in native stores, arguments or constants when emitted. No first-return crop is
used.

Routes tried for the opaque call: selected-span native analysis, full raw words and
bounded contexts, and ELF symbol, segment and relocation metadata. These bounded routes
do not claim that every possible source was exhausted.

The callee is still unresolved. The writers of the field named above are the next routines to read.

The [complete facts](disasm/arm-1.44.0-libraries/router-paths-facts.txt) retain every emitted native field, instruction, raw gap, warning, relocation and constant for the full selected spans.

- `RouterInput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0x9ab0 to 0xa064 (exclusive), 1460 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `3f991e7dfad8dce784c6cec3305ebcb133c76edc75c0d50713fef772c24cc918`.
  The native facts retain 2 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0x9ab4).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0x9ac0).
  Grouped accesses: 24 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x9ac4, `ldr` at 0x9ae4, `ldr` at 0x9b08.
  Four-byte load: target `r0 +0xe4`, base register `r3`, raw displacement `+0xe4` (`ldr` at 0x9ac8).
  Branch (`bne` at 0x9ad0): taken to 0x9b18 when `[r0+228] != 0x0`, falls through to 0x9ad4 when `[r0+228] == 0x0`.
  Grouped accesses: 16 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x9ad4, `ldr` at 0x9b0c, `ldr` at 0x9b2c.
  Four-byte store: target `r0 +0xf0`, value `0x8`, base register `r2`, raw displacement `+0xf0` (`str` at 0x9adc).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x9ae0, `ldr` at 0x9f14.
  Four-byte load: target `[sp-24] +0xf0`, base register `r3`, raw displacement `+0xf0` (`ldr` at 0x9ae8).
  Call (`bl` at 0x9aec): (unknown callee), target 0x35a4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35a4, conditional `false`, r0=`[[sp-24]+240]`, r1=`r1`, r2=`r0`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0xe4`, value `ret(linked ELF 0x35a4)`, base register `r4`, raw displacement `+0xe4` (`str` at 0x9af4).
  Grouped accesses: 2 4-byte load records with target `[sp-24] +0xe4`, base register `r4`, raw displacement `+0xe4`, including `ldr` at 0x9af8, `ldr` at 0x9f30.
  Branch (`bne` at 0x9b00): taken to 0x9b08 when `[[sp-24]+228] != 0x0`, falls through to 0x9b04 when `[[sp-24]+228] == 0x0`.
  Call (`bl` at 0x9b04): (unknown callee), target 0x3910, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3910, conditional `false`, r0=`ret(linked ELF 0x35a4)`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+228]`.
  Four-byte load: target `[sp-24] +0xe4`, base register `r2`, raw displacement `+0xe4` (`ldr` at 0x9b10).
  Four-byte store: target `[sp-24] +0xe8`, value `[[sp-24]+228]`, base register `r3`, raw displacement `+0xe8` (`str` at 0x9b14).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0xec`, raw addressing form `[r3, #0xec]`, including `ldr` at 0x9b1c, `ldr` at 0x9c60.
  Branch (`ble` at 0x9b24): taken to 0x9c5c when `le flags condition (operands unknown)`, falls through to 0x9b28 when `not (le flags condition (operands unknown))`.
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0x9b30).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]` (`ldr` at 0x9b34).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0xe8`, raw addressing form `[r2, #0xe8]` (`ldr` at 0x9b38).
  Four-byte load: target `(unknown effective address)`, base register `r12`, raw displacement `+0xec`, raw addressing form `[ip, #0xec]` (`ldr` at 0x9b3c).
  Call (`bl` at 0x9b40): (unknown callee), target 0x3904, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3904, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x1c`, value `ret(linked ELF 0x3904)`, base register `r11`, raw displacement `-0x18` (`str` at 0x9b48).
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x9b4c, `ldr` at 0x9c14, `ldr` at 0x9c38.
  Branch (`bge` at 0x9b54): taken to 0x9c14 when `ret(linked ELF 0x3904) >= 0x0 (signed, 32-bit)`, falls through to 0x9b58 when `ret(linked ELF 0x3904) < 0x0 (signed, 32-bit)`.
  Call (`bl` at 0x9b58): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, conditional `false`, r0=`ret(linked ELF 0x3904)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3904)`.
  Four-byte load: target `ret(linked ELF 0x34f0) +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x9b60).
  Four-byte store: target `sp -0x34`, value `[ret(linked ELF 0x34f0)]`, base register `r11`, raw displacement `-0x30` (`str` at 0x9b64).
  Grouped accesses: 3 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x9b68, `ldr` at 0x9b8c, `ldr` at 0x9bb4.
  Branch (`beq` at 0x9b70): taken to 0x9bd4 when `[ret(linked ELF 0x34f0)] == 0x6b`, falls through to 0x9b74 when `[ret(linked ELF 0x34f0)] != 0x6b`.
  Grouped accesses: 2 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x9b74, `ldr` at 0x9b98.
  Branch (`bgt` at 0x9b7c): taken to 0x9bc4 when `[ret(linked ELF 0x34f0)] > 0x6b (signed, 32-bit)`, falls through to 0x9b80 when `[ret(linked ELF 0x34f0)] <= 0x6b (signed, 32-bit)`.
  Grouped accesses: 3 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x9b80, `ldr` at 0x9ba4, `ldr` at 0x9bc4.
  Branch (`beq` at 0x9b88): taken to 0x9bd4 when `[ret(linked ELF 0x34f0)] == 0x65`, falls through to 0x9b8c when `[ret(linked ELF 0x34f0)] != 0x65`.
  Branch (`bgt` at 0x9b94): taken to 0x9bb4 when `[ret(linked ELF 0x34f0)] > 0x65 (signed, 32-bit)`, falls through to 0x9b98 when `[ret(linked ELF 0x34f0)] <= 0x65 (signed, 32-bit)`.
  Branch (`beq` at 0x9ba0): taken to 0xa054 when `[ret(linked ELF 0x34f0)] == 0xb`, falls through to 0x9ba4 when `[ret(linked ELF 0x34f0)] != 0xb`.
  Branch (`beq` at 0x9bac): taken to 0x9bd4 when `[ret(linked ELF 0x34f0)] == 0x20`, falls through to 0x9bb0 when `[ret(linked ELF 0x34f0)] != 0x20`.
  Branch (`beq` at 0x9bbc): taken to 0x9bd4 when `[ret(linked ELF 0x34f0)] == 0x68`, falls through to 0x9bc0 when `[ret(linked ELF 0x34f0)] != 0x68`.
  Branch (`bhi` at 0x9bd0): taken to 0x9be0 when `[ret(linked ELF 0x34f0)]-110 > 0x1 (unsigned, 32-bit)`, falls through to 0x9bd4 when `[ret(linked ELF 0x34f0)]-110 <= 0x1 (unsigned, 32-bit)`.
  Grouped accesses: 6 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x9bd4, `ldr` at 0x9c08, `ldr` at 0x9c20.
  Call (`bl` at 0x9bd8): (unknown callee), target 0x340c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x340c, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`[ret(linked ELF 0x34f0)]`, r3=`unknown`.
  Call (`bl` at 0x9be0): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, conditional `false`, r0=`ret(linked ELF 0x34f0)`, r1=`unknown`, r2=`[ret(linked ELF 0x34f0)]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xa05c`, base register `pc`, raw displacement `+0x468`, linked address 0xa05c, basis `ELF load-bias-relative` (`ldr` at 0x9bec).
  Native warning (`ldr` at 0x9bec): `ELF literal at linked 0xa05c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x44`, value `unknown`, base register `sp`, raw displacement `+0x0` (`str` at 0x9bf0).
  Warning class: 12 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0x9bf4, `mvn` at 0x9c88, `mvn` at 0x9cb4.
  Four-byte load: target `ELF load bias (unknown) +0xa060`, base register `pc`, raw displacement `+0x460`, linked address 0xa060, basis `ELF load-bias-relative` (`ldr` at 0x9bf8).
  Native warning (`ldr` at 0x9bf8): `ELF literal at linked 0xa060 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `ret(linked ELF 0x34f0) +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0x9bfc).
  Call (`bl` at 0x9c04): (unknown callee), target 0x38f8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x38f8, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[ret(linked ELF 0x34f0)]`, r3=`[sp-24]`.
  Call (`bl` at 0x9c0c): (unknown callee), target 0x34d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34d8, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x9c1c): taken to 0x9c2c when `ret(linked ELF 0x3904) != 0x0`, falls through to 0x9c20 when `ret(linked ELF 0x3904) == 0x0`.
  Call (`bl` at 0x9c24): (unknown callee), target 0x340c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x340c, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3904)`.
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x9c2c, `ldr` at 0x9c44, `ldr` at 0xa044.
  Four-byte load: target `[sp-24] +0xe8`, base register `r3`, raw displacement `+0xe8` (`ldr` at 0x9c34).
  Four-byte store: target `[sp-24] +0xe8`, value `([[sp-24]+232] + ret(linked ELF 0x3904))`, base register `r1`, raw displacement `+0xe8` (`str` at 0x9c40).
  Four-byte load: target `[sp-24] +0xec`, base register `r3`, raw displacement `+0xec` (`ldr` at 0x9c4c).
  Warning class: 2 native records report `rsb: unsupported effects invalidated`, including `rsb` at 0x9c54, `rsb` at 0x9eb0.
  Four-byte store: target `[sp-24] +0xec`, value `unknown`, base register `r1`, raw displacement `+0xec` (`str` at 0x9c58).
  Branch (`bgt` at 0x9c68): taken to 0xa054 when `gt flags condition (operands unknown)`, falls through to 0x9c6c when `not (gt flags condition (operands unknown))`.
  Two-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0xf6`, raw addressing form `[r3, #0xf6]` (`ldrsh` at 0x9c70).
  Branch (`bne` at 0x9c78): taken to 0x9ea0 when `? != 0x0`, falls through to 0x9c7c when `? == 0x0`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0xe4`, raw addressing form `[r3, #0xe4]` (`ldr` at 0x9c80).
  Four-byte store: target `sp -0x30`, value `unknown`, base register `r11`, raw displacement `-0x2c` (`str` at 0x9c84).
  Four-byte load: target `sp -0x30`, base register `r1`, raw displacement `+0x0` (`ldr` at 0x9c98).
  Grouped accesses: 5 1-byte load records with target `[sp-48] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldrb` at 0x9ca0, `ldrb` at 0x9cd8, `ldrb` at 0x9d78.
  Four-byte store: target `sp -0x30`, value `[sp-48]+1`, base register `r1`, raw displacement `+0x0` (`str` at 0x9cac).
  Two-byte store: target `(unknown effective address)`, value `unknown`, base register `r0`, raw displacement `+0x0`, raw addressing form `[r0]` (`strh` at 0x9cb0).
  Four-byte load: target `sp -0x30`, base register `r12`, raw displacement `+0x0` (`ldr` at 0x9cd0).
  Two-byte load: target `(unknown effective address)`, base register `r0`, raw displacement `+0x0`, raw addressing form `[r0]` (`ldrh` at 0x9ce0).
  Four-byte store: target `sp -0x30`, value `[sp-48]+1`, base register `r12`, raw displacement `+0x0` (`str` at 0x9cec).
  Two-byte store: target `(unknown effective address)`, value `unknown`, base register `lr`, raw displacement `+0x0`, raw addressing form `[lr]` (`strh` at 0x9cf0).
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r2`, raw displacement `+0x0`, including `ldr` at 0x9d04, `ldr` at 0x9d28, `ldr` at 0x9d4c.
  Grouped accesses: 3 1-byte load records with target `[sp-48] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldrb` at 0x9d08, `ldrb` at 0x9d2c, `ldrb` at 0x9d50.
  Grouped accesses: 3 4-byte store records with target `sp -0x30`, reported value `[sp-48]+1` in every record, base register `r2`, raw displacement `+0x0`, including `str` at 0x9d10, `str` at 0x9d34, `str` at 0x9d58.
  Byte store: target `(unknown effective address)`, value `([[sp-48]]:1 & 0xff)`, base register `r0`, raw displacement `+0x2`, raw addressing form `[r0, #2]` (`strb` at 0x9d14).
  Byte store: target `(unknown effective address)`, value `([[sp-48]]:1 & 0xff)`, base register `r0`, raw displacement `+0x3`, raw addressing form `[r0, #3]` (`strb` at 0x9d38).
  Byte store: target `(unknown effective address)`, value `([[sp-48]]:1 & 0xff)`, base register `r0`, raw displacement `+0x4`, raw addressing form `[r0, #4]` (`strb` at 0x9d5c).
  Grouped accesses: 2 4-byte load records with target `sp -0x30`, base register `r12`, raw displacement `+0x0`, including `ldr` at 0x9d70, `ldr` at 0x9e3c.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r0`, raw displacement `+0x4`, raw addressing form `[r0, #4]`, including `ldr` at 0x9d80, `ldr` at 0x9e4c.
  Four-byte store: target `sp -0x38`, value `unknown`, base register `r11`, raw displacement `-0x34` (`str` at 0x9d84).
  Four-byte load: target `sp -0x38`, base register `r11`, raw displacement `-0x34` (`ldr` at 0x9d8c).
  Four-byte store: target `sp -0x38`, value `unknown`, base register `r11`, raw displacement `-0x34` (`str` at 0x9d9c).
  Four-byte load: target `sp -0x38`, base register `r11`, raw displacement `-0x34` (`ldr` at 0x9da0).
  Four-byte store: target `(unknown effective address)`, value `[sp-56]`, base register `r0`, raw displacement `+0x4`, raw addressing form `[r0, #4]` (`str` at 0x9da4).
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[sp-48]+1` in every record, base register `r12`, raw displacement `+0x0`, including `str` at 0x9dac, `str` at 0x9e78.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]`, including `ldr` at 0x9dc8, `ldr` at 0x9e30.
  Four-byte load: target `sp -0x30`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x9dd4).
  Four-byte load: target `(unknown effective address)`, base register `r12`, raw displacement `+0x4`, raw addressing form `[ip, #4]` (`ldr` at 0x9de8).
  Four-byte store: target `sp -0x3c`, value `unknown`, base register `r11`, raw displacement `-0x38` (`str` at 0x9dec).
  Four-byte load: target `sp -0x3c`, base register `r11`, raw displacement `-0x38` (`ldr` at 0x9df4).
  Four-byte store: target `sp -0x3c`, value `unknown`, base register `r11`, raw displacement `-0x38` (`str` at 0x9e04).
  Four-byte load: target `sp -0x3c`, base register `r11`, raw displacement `-0x38` (`ldr` at 0x9e08).
  Four-byte store: target `(unknown effective address)`, value `[sp-60]`, base register `r12`, raw displacement `+0x4`, raw addressing form `[ip, #4]` (`str` at 0x9e0c).
  Four-byte store: target `sp -0x30`, value `[sp-48]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0x9e14).
  Four-byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`str` at 0x9e50).
  Four-byte load: target `sp -0x40`, base register `r11`, raw displacement `-0x3c` (`ldr` at 0x9e58).
  Four-byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`str` at 0x9e68).
  Four-byte load: target `sp -0x40`, base register `r11`, raw displacement `-0x3c` (`ldr` at 0x9e6c).
  Four-byte store: target `(unknown effective address)`, value `[sp-64]`, base register `r0`, raw displacement `+0x4`, raw addressing form `[r0, #4]` (`str` at 0x9e70).
  Grouped accesses: 3 4-byte load records with target `[sp-24] +0xe4`, base register `r3`, raw displacement `+0xe4`, including `ldr` at 0x9e84, `ldr` at 0x9f1c, `ldr` at 0x9fb8.
  Call (`bl` at 0x9e90): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`[[sp-24]+228]`, r1=`sp-44`, r2=`0x8`, r3=`[sp-24]`.
  Two-byte store: target `[sp-24] +0xf6`, value `0x1`, base register `r2`, raw displacement `+0xf6` (`strh` at 0x9e9c).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0xe8`, raw addressing form `[r3, #0xe8]` (`ldr` at 0x9ea8).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0xe4`, raw addressing form `[r2, #0xe4]` (`ldr` at 0x9eac).
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0x9eb4).
  Grouped accesses: 3 4-byte load records with target `[sp-24] +0xe4`, base register `r3`, raw displacement `+0xe4`, including `ldr` at 0x9ebc, `ldr` at 0x9ee0, `ldr` at 0x9f48.
  Grouped accesses: 2 4-byte load records with target `[[sp-24]+228] +0x4`, base register `r3`, raw displacement `+0x4`, including `ldr` at 0x9ec0, `ldr` at 0x9ee4.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0x9ecc).
  Branch (`bls` at 0x9ed4): taken to 0x9f64 when `? <= [sp-32] (unsigned, 32-bit)`, falls through to 0x9ed8 when `? > [sp-32] (unsigned, 32-bit)`.
  Four-byte store: target `[sp-24] +0xec`, value `unknown`, base register `r2`, raw displacement `+0xec` (`str` at 0x9eec).
  Four-byte load: target `[sp-24] +0xec`, base register `r3`, raw displacement `+0xec` (`ldr` at 0x9ef4).
  Four-byte store: target `sp -0x24`, value `[[sp-24]+236]+8`, base register `r11`, raw displacement `-0x20` (`str` at 0x9efc).
  Four-byte load: target `[sp-24] +0xf0`, base register `r3`, raw displacement `+0xf0` (`ldr` at 0x9f04).
  Grouped accesses: 2 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x9f08, `ldr` at 0x9f58.
  Branch (`bge` at 0x9f10): taken to 0xa054 when `[[sp-24]+240] >= [[sp-24]+236]+8 (signed, 32-bit)`, falls through to 0x9f14 when `[[sp-24]+240] < [[sp-24]+236]+8 (signed, 32-bit)`.
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0x9f20).
  Call (`bl` at 0x9f24): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[[sp-24]+228]`, r1=`[[sp-24]+236]+8`, r2=`[[sp-24]+240]`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0xe4`, value `ret(linked ELF 0x3700)`, base register `r4`, raw displacement `+0xe4` (`str` at 0x9f2c).
  Branch (`bne` at 0x9f38): taken to 0x9f40 when `[[sp-24]+228] != 0x0`, falls through to 0x9f3c when `[[sp-24]+228] == 0x0`.
  Call (`bl` at 0x9f3c): (unknown callee), target 0x3910, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3910, conditional `false`, r0=`ret(linked ELF 0x3700)`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+228]`.
  Four-byte store: target `[sp-24] +0xe8`, value `[[sp-24]+228]+8`, base register `r2`, raw displacement `+0xe8` (`str` at 0x9f50).
  Four-byte store: target `[sp-24] +0xf0`, value `[sp-36]`, base register `r2`, raw displacement `+0xf0` (`str` at 0x9f5c).
  Four-byte load: target `[sp-24] +0x128`, base register `r3`, raw displacement `+0x128` (`ldr` at 0x9f6c).
  Four-byte store: target `[sp-24] +0x128`, value `[[sp-24]+296]+1`, base register `r2`, raw displacement `+0x128` (`str` at 0x9f74).
  Four-byte load: target `[sp-24] +0xf8`, base register `r3`, raw displacement `+0xf8` (`ldr` at 0x9f7c).
  Call (`mov` at 0x9f88): (unknown callee), target (unknown target), basis `unknown`, execution state `unknown`, canonical instruction target (unknown target), conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`[sp-24]`, r3=`[[sp-24]+248]`.
  Branch (`beq` at 0x9f94): taken to 0x9f9c when `ret(unknown) == 0x0`, falls through to 0x9f98 when `ret(unknown) != 0x0`.
  Call (`bl` at 0x9fa0): (unknown callee), target 0x37c0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37c0, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Branch (`bne` at 0x9fac): taken to 0x9fb4 when `ret(linked ELF 0x37c0) != 0x0`, falls through to 0x9fb0 when `ret(linked ELF 0x37c0) == 0x0`.
  Call (`bl` at 0x9fbc): (unknown callee), target 0x3808, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3808, conditional `false`, r0=`[[sp-24]+228]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Four-byte store: target `[sp-24] +0xe4`, value `0x0`, base register `r2`, raw displacement `+0xe4` (`str` at 0x9fc8).
  Four-byte store: target `[sp-24] +0xf0`, value `0x0`, base register `r2`, raw displacement `+0xf0` (`str` at 0x9fd4).
  Four-byte store: target `[sp-24] +0xe8`, value `0x0`, base register `r2`, raw displacement `+0xe8` (`str` at 0x9fe0).
  Four-byte store: target `[sp-24] +0xec`, value `0x8`, base register `r2`, raw displacement `+0xec` (`str` at 0x9fec).
  Two-byte store: target `[sp-24] +0xf4`, value `0x0`, base register `r2`, raw displacement `+0xf4` (`strh` at 0x9ff8).
  Two-byte store: target `[sp-24] +0xf6`, value `0x0`, base register `r2`, raw displacement `+0xf6` (`strh` at 0xa004).
  Byte load: target `[sp-24] +0x154`, base register `r2`, raw displacement `+0x0` (`ldrb` at 0xa010).
  Branch (`beq` at 0xa018): taken to 0xa054 when `[[sp-24]+340]:1 == 0x0`, falls through to 0xa01c when `[[sp-24]+340]:1 != 0x0`.
  Four-byte load: target `[sp-24] +0x14c`, base register `r3`, raw displacement `+0x14c` (`ldr` at 0xa020).
  Branch (`beq` at 0xa028): taken to 0xa054 when `[[sp-24]+332] == 0x0`, falls through to 0xa02c when `[[sp-24]+332] != 0x0`.
  Four-byte load: target `[sp-24] +0xd4`, base register `r3`, raw displacement `+0xd4` (`ldr` at 0xa030).
  Branch (`bgt` at 0xa038): taken to 0xa054 when `[[sp-24]+212] > 0xe6 (signed, 32-bit)`, falls through to 0xa03c when `[[sp-24]+212] <= 0xe6 (signed, 32-bit)`.
  Call (`bl` at 0xa040): (unknown callee), target 0xd1e4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xd1e4, conditional `false`, r0=`[sp-24]`, r1=`unknown`, r2=`[sp-24]`, r3=`[[sp-24]+212]`.
  Byte store: target `[sp-24] +0x154`, value `0x0`, base register `r1`, raw displacement `+0x0` (`strb` at 0xa050).
  Known-edge return (`ldm` at 0xa058): recorded r0 is `unknown`.
  Known return boundary (`ldm` at 0xa058): the modeled local edges end before the retained trailing region 0xa05c to 0xa064 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xa05c to 0xa060 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterUnpack`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xbc9c to 0xc34c (exclusive), 1712 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `129687d0e27d8a3b681b96cedbc6878bb954dc5e5fa9c53a27a0020d0e492663`.
  The native facts retain 3 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0xbca0).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0xbcac).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0xbcb0).
  Four-byte store: target `sp -0x34`, value `0x0`, base register `r11`, raw displacement `-0x30` (`str` at 0xbcb8).
  Four-byte store: target `sp -0x38`, value `0x0`, base register `r11`, raw displacement `-0x34` (`str` at 0xbcc0).
  Grouped accesses: 11 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xbcc4, `ldr` at 0xbcd4, `ldr` at 0xbde4.
  Grouped accesses: 2 4-byte load records with target `r0 +0xfc`, base register `r3`, raw displacement `+0xfc`, including `ldr` at 0xbcc8, `ldr` at 0xbcd8.
  Branch (`beq` at 0xbcd0): taken to 0xbcf0 when `[r0+252] == 0x0`, falls through to 0xbcd4 when `[r0+252] != 0x0`.
  Four-byte load: target `[r0+252] +0x4`, base register `r3`, raw displacement `+0x4` (`ldr` at 0xbcdc).
  Branch (`bge` at 0xbce4): taken to 0xbcf0 when `[[r0+252]+4] >= 0x0 (signed, 32-bit)`, falls through to 0xbce8 when `[[r0+252]+4] < 0x0 (signed, 32-bit)`.
  Four-byte store: target `sp -0x34`, value `0x1`, base register `r11`, raw displacement `-0x30` (`str` at 0xbcec).
  Grouped accesses: 14 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xbcf0, `ldr` at 0xbd04, `ldr` at 0xbd10.
  Grouped accesses: 4 4-byte load records with target `r1 +0x4`, base register `r3`, raw displacement `+0x4`, including `ldr` at 0xbcf4, `ldr` at 0xbd14, `ldr` at 0xbd4c.
  Four-byte store: target `sp -0x2c`, value `unknown`, base register `r11`, raw displacement `-0x28` (`str` at 0xbd00).
  Grouped accesses: 2 4-byte store records with target `sp -0x20`, reported value `r1+8` in every record, base register `r11`, raw displacement `-0x1c`, including `str` at 0xbd0c, `str` at 0xbe38.
  Grouped accesses: 7 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xbd1c, `ldr` at 0xbe48, `ldr` at 0xbf34.
  Grouped accesses: 2 4-byte store records with target `sp -0x28`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x24`, including `str` at 0xbd24, `str` at 0xbe50.
  Grouped accesses: 3 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0xbd28, `ldr` at 0xbe54, `ldr` at 0xc308.
  Branch (`beq` at 0xbd30): taken to 0xbe30 when `Z == 1`, falls through to 0xbd34 when `not (Z == 1)`.
  Byte load: target `r1 +0x4`, base register `r3`, raw displacement `+0x4` (`ldrb` at 0xbd38).
  Branch (`beq` at 0xbd44): taken to 0xbe30 when `? == 0x0`, falls through to 0xbd48 when `? != 0x0`.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xbd5c).
  Call (`bl` at 0xbd68): (unknown callee), target 0x358c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x358c, conditional `false`, r0=`r1+8`, r1=`unknown`, r2=`sp-36`, r3=`sp-48`.
  Native warning (`cmn` at 0xbd70): `cmn: unsupported effects invalidated`.
  Branch (`bne` at 0xbd74): taken to 0xbd7c when `Z == 0`, falls through to 0xbd78 when `not (Z == 0)`.
  Call (`bl` at 0xbd78): (unknown callee), target 0x3910, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3910, conditional `false`, r0=`ret(linked ELF 0x358c)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x358c)`.
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xbd7c).
  Four-byte store: target `sp -0x20`, value `[sp-36]`, base register `r11`, raw displacement `-0x1c` (`str` at 0xbd80).
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xbd84).
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0xbd88, `ldr` at 0xbdd4, `ldr` at 0xbe20.
  Four-byte store: target `sp -0x28`, value `([sp-36] + [sp-48])`, base register `r11`, raw displacement `-0x24` (`str` at 0xbd90).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x5a4`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbd94).
  Warning class: 14 native records report `ELF literal at linked 0xc340 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xbd94, `ldr` at 0xbd98, `ldr` at 0xbda8.
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x5a0`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbd98).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0xc`, raw addressing form `[r3, #0xc]` (`ldr` at 0xbd9c).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0xc`, raw addressing form `[r2, #0xc]` (`str` at 0xbda4).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x590`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbda8).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x58c`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbdac).
  Grouped accesses: 6 4-byte load records with target `[sp-28] +0x4`, base register `r3`, raw displacement `+0x4`, including `ldr` at 0xbdb4, `ldr` at 0xbe00, `ldr` at 0xbe80.
  Four-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x20`, raw addressing form `[r1, #0x20]` (`ldr` at 0xbdbc).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r0`, raw displacement `+0x20`, raw addressing form `[r0, #0x20]` (`str` at 0xbdc4).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x570`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbdc8).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x56c`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbdcc).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x24`, raw addressing form `[r3, #0x24]` (`ldr` at 0xbdd0).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r1`, raw displacement `+0x24`, raw addressing form `[r1, #0x24]` (`str` at 0xbddc).
  Grouped accesses: 3 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xbde0, `ldr` at 0xbe94, `ldr` at 0xbf00.
  Four-byte load: target `[sp-24] +0x10c`, base register `r3`, raw displacement `+0x10c` (`ldr` at 0xbde8).
  Four-byte store: target `[sp-24] +0x10c`, value `[[sp-24]+268]+1`, base register `r2`, raw displacement `+0x10c` (`str` at 0xbdf0).
  Grouped accesses: 5 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xbdf4, `ldr` at 0xbea8, `ldr` at 0xbf14.
  Grouped accesses: 20 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xbdf8, `ldr` at 0xbe14, `ldr` at 0xbeac.
  Four-byte load: target `[sp-24] +0x120`, base register `r1`, raw displacement `+0x120` (`ldr` at 0xbe08).
  Four-byte store: target `[sp-24] +0x120`, value `unknown`, base register `r0`, raw displacement `+0x120` (`str` at 0xbe10).
  Four-byte load: target `[sp-24] +0x124`, base register `r3`, raw displacement `+0x124` (`ldr` at 0xbe1c).
  Four-byte store: target `[sp-24] +0x124`, value `([[sp-24]+292] + [sp-48])`, base register `r1`, raw displacement `+0x124` (`str` at 0xbe28).
  Branch (`bne` at 0xbe5c): taken to 0xbecc when `Z == 0`, falls through to 0xbe60 when `not (Z == 0)`.
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x4d8`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbe60).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x4d4`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbe64).
  Grouped accesses: 5 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xbe68, `ldr` at 0xc1e8, `ldr` at 0xc230.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xbe70).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x4c4`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbe74).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x4c0`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbe78).
  Four-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x10`, raw addressing form `[r1, #0x10]` (`ldr` at 0xbe88).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r0`, raw displacement `+0x10`, raw addressing form `[r0, #0x10]` (`str` at 0xbe90).
  Four-byte load: target `[sp-24] +0x100`, base register `r3`, raw displacement `+0x100` (`ldr` at 0xbe9c).
  Four-byte store: target `[sp-24] +0x100`, value `[[sp-24]+256]+1`, base register `r2`, raw displacement `+0x100` (`str` at 0xbea4).
  Four-byte load: target `[sp-24] +0x110`, base register `r1`, raw displacement `+0x110` (`ldr` at 0xbebc).
  Four-byte store: target `[sp-24] +0x110`, value `unknown`, base register `r0`, raw displacement `+0x110` (`str` at 0xbec4).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x46c`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbecc).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x468`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbed0).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x8`, raw addressing form `[r3, #8]` (`ldr` at 0xbed4).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x8`, raw addressing form `[r2, #8]` (`str` at 0xbedc).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x458`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbee0).
  Four-byte load: target `ELF load bias (unknown) +0xc340`, base register `pc`, raw displacement `+0x454`, linked address 0xc340, basis `ELF load-bias-relative` (`ldr` at 0xbee4).
  Four-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x18`, raw addressing form `[r1, #0x18]` (`ldr` at 0xbef4).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r0`, raw displacement `+0x18`, raw addressing form `[r0, #0x18]` (`str` at 0xbefc).
  Four-byte load: target `[sp-24] +0x108`, base register `r3`, raw displacement `+0x108` (`ldr` at 0xbf08).
  Four-byte store: target `[sp-24] +0x108`, value `[[sp-24]+264]+1`, base register `r2`, raw displacement `+0x108` (`str` at 0xbf10).
  Four-byte load: target `[sp-24] +0x118`, base register `r1`, raw displacement `+0x118` (`ldr` at 0xbf28).
  Four-byte store: target `[sp-24] +0x118`, value `unknown`, base register `r0`, raw displacement `+0x118` (`str` at 0xbf30).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xbf3c, `ldr` at 0xc1f8.
  Branch (`bhs` at 0xbf44): taken to 0xc308 when `[sp-32]+4 >= [sp-40] (unsigned, 32-bit)`, falls through to 0xbf48 when `[sp-32]+4 < [sp-40] (unsigned, 32-bit)`.
  Grouped accesses: 2 4-byte load records with target `[sp-24] +0x188`, base register `r3`, raw displacement `+0x188`, including `ldr` at 0xbf4c, `ldr` at 0xbfc8.
  Grouped accesses: 16 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0xbf50, `ldr` at 0xbfbc, `ldr` at 0xc0b4.
  Branch (`bgt` at 0xbf58): taken to 0xc084 when `[[sp-24]+392] > [sp-56] (signed, 32-bit)`, falls through to 0xbf5c when `[[sp-24]+392] <= [sp-56] (signed, 32-bit)`.
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xbf5c).
  Four-byte load: target `ELF load bias (unknown) +0xc344`, base register `pc`, raw displacement `+0x3d4`, linked address 0xc344, basis `ELF load-bias-relative` (`ldr` at 0xbf68).
  Warning class: 2 native records report `ELF literal at linked 0xc344 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xbf68, `ldr` at 0xbfa8.
  Grouped accesses: 2 4-byte load records with target `[sp-24] +0x188`, base register `r3`, raw displacement `+0x188`, including `ldr` at 0xbf6c, `ldr` at 0xbfac.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]`, including `ldr` at 0xbf70, `ldr` at 0xbfb0, `ldr` at 0xc174.
  Four-byte load: target `[sp-24] +0x18c`, base register `r0`, raw displacement `+0x18c` (`ldr` at 0xbf7c).
  Call (`bl` at 0xbf84): (unknown callee), target 0x3700, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3700, conditional `false`, r0=`[[sp-24]+396]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `[sp-24] +0x18c`, value `ret(linked ELF 0x3700)`, base register `r4`, raw displacement `+0x18c` (`str` at 0xbf8c).
  Four-byte load: target `[sp-24] +0x18c`, base register `r4`, raw displacement `+0x18c` (`ldr` at 0xbf90).
  Branch (`bne` at 0xbf98): taken to 0xbfa0 when `[[sp-24]+396] != 0x0`, falls through to 0xbf9c when `[[sp-24]+396] == 0x0`.
  Call (`bl` at 0xbf9c): (unknown callee), target 0x3910, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3910, conditional `false`, r0=`ret(linked ELF 0x3700)`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+396]`.
  Four-byte load: target `ELF load bias (unknown) +0xc344`, base register `pc`, raw displacement `+0x394`, linked address 0xc344, basis `ELF load-bias-relative` (`ldr` at 0xbfa8).
  Four-byte store: target `[sp-24] +0x188`, value `unknown`, base register `r0`, raw displacement `+0x188` (`str` at 0xbfb8).
  Four-byte store: target `sp -0x3c`, value `[sp-56]`, base register `r11`, raw displacement `-0x38` (`str` at 0xbfc0).
  Grouped accesses: 6 4-byte load records with target `sp -0x3c`, base register `r11`, raw displacement `-0x38`, including `ldr` at 0xbfcc, `ldr` at 0xbfdc, `ldr` at 0xc00c.
  Branch (`ble` at 0xbfd4): taken to 0xc084 when `le flags condition (operands unknown)`, falls through to 0xbfd8 when `not (le flags condition (operands unknown))`.
  Grouped accesses: 16 4-byte load records with target `[sp-24] +0x18c`, base register `r1`, raw displacement `+0x18c`, including `ldr` at 0xbfe4, `ldr` at 0xc014, `ldr` at 0xc030.
  Byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldrb` at 0xbfec).
  Byte store: target `sp -0x44`, value `unknown`, base register `r11`, raw displacement `-0x40` (`strb` at 0xbff0).
  Byte load: target `sp -0x44`, base register `r11`, raw displacement `-0x40` (`ldrb` at 0xbff4).
  Byte store: target `sp -0x44`, value `unknown`, base register `r11`, raw displacement `-0x40` (`strb` at 0xbffc).
  Byte load: target `sp -0x44`, base register `r11`, raw displacement `-0x40` (`ldrb` at 0xc000).
  Byte store: target `(unknown effective address)`, value `([sp-68]:1 & 0xff)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`strb` at 0xc004).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x4`, raw addressing form `[r2, #4]` (`str` at 0xc020).
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]`, including `ldrb` at 0xc038, `ldrb` at 0xc2ec.
  Grouped accesses: 2 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]`, including `strb` at 0xc040, `strb` at 0xc2f4.
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldr` at 0xc058).
  Four-byte store: target `sp -0x48`, value `unknown`, base register `r11`, raw displacement `-0x44` (`str` at 0xc05c).
  Four-byte load: target `sp -0x48`, base register `r11`, raw displacement `-0x44` (`ldr` at 0xc060).
  Four-byte store: target `sp -0x48`, value `unknown`, base register `r11`, raw displacement `-0x44` (`str` at 0xc068).
  Four-byte load: target `sp -0x48`, base register `r11`, raw displacement `-0x44` (`ldr` at 0xc06c).
  Four-byte store: target `(unknown effective address)`, value `[sp-72]`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xc070).
  Four-byte store: target `sp -0x3c`, value `[sp-60]+1`, base register `r11`, raw displacement `-0x38` (`str` at 0xc07c).
  Jump (`b` at 0xc080): target 0xbfc4.
  Four-byte load: target `ELF load bias (unknown) +0xc348`, base register `pc`, raw displacement `+0x2bc`, linked address 0xc348, basis `ELF load-bias-relative` (`ldr` at 0xc084).
  Warning class: 4 native records report `ELF literal at linked 0xc348 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xc084, `ldr` at 0xc0a0, `ldr` at 0xc190.
  Grouped accesses: 3 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xc088, `ldr` at 0xc194, `ldr` at 0xc29c.
  Call (`bl` at 0xc090): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`[sp-32]`, r2=`0x4`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x20`, reported value `[sp-32]+4` in every record, base register `r11`, raw displacement `-0x1c`, including `str` at 0xc09c, `str` at 0xc1a8.
  Four-byte load: target `ELF load bias (unknown) +0xc348`, base register `pc`, raw displacement `+0x2a0`, linked address 0xc348, basis `ELF load-bias-relative` (`ldr` at 0xc0a0).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xc0a4, `ldr` at 0xc1c4.
  Call (`bl` at 0xc0a8): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x40`, value `ret(linked ELF 0x337c)`, base register `r11`, raw displacement `-0x3c` (`str` at 0xc0ac).
  Grouped accesses: 2 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0xc0c4, `ldr` at 0xc108.
  Byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`ldrb` at 0xc0cc).
  Byte store: target `sp -0x4c`, value `unknown`, base register `r11`, raw displacement `-0x48` (`strb` at 0xc0d0).
  Byte load: target `sp -0x4c`, base register `r11`, raw displacement `-0x48` (`ldrb` at 0xc0d8).
  Byte store: target `sp -0x4c`, value `unknown`, base register `r11`, raw displacement `-0x48` (`strb` at 0xc0e8).
  Byte load: target `sp -0x4c`, base register `r11`, raw displacement `-0x48` (`ldrb` at 0xc0ec).
  Byte store: target `(unknown effective address)`, value `([sp-76]:1 & 0xff)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`strb` at 0xc0f0).
  Four-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`ldr` at 0xc110).
  Four-byte store: target `sp -0x50`, value `unknown`, base register `r11`, raw displacement `-0x4c` (`str` at 0xc114).
  Four-byte load: target `sp -0x50`, base register `r11`, raw displacement `-0x4c` (`ldr` at 0xc11c).
  Four-byte store: target `sp -0x50`, value `unknown`, base register `r11`, raw displacement `-0x4c` (`str` at 0xc12c).
  Four-byte load: target `sp -0x50`, base register `r11`, raw displacement `-0x4c` (`ldr` at 0xc130).
  Four-byte store: target `(unknown effective address)`, value `[sp-80]`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`str` at 0xc134).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]` (`ldrb` at 0xc14c).
  Branch (`bne` at 0xc15c): taken to 0xc1d4 when `? != 0x0`, falls through to 0xc160 when `? == 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x54`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x50`, including `str` at 0xc178, `str` at 0xc184.
  Four-byte load: target `sp -0x54`, base register `r11`, raw displacement `-0x50` (`ldr` at 0xc17c).
  Four-byte load: target `sp -0x54`, base register `r11`, raw displacement `-0x50` (`ldr` at 0xc188).
  Four-byte store: target `(unknown effective address)`, value `[sp-84]`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`str` at 0xc18c).
  Four-byte load: target `ELF load bias (unknown) +0xc348`, base register `pc`, raw displacement `+0x1b0`, linked address 0xc348, basis `ELF load-bias-relative` (`ldr` at 0xc190).
  Call (`bl` at 0xc19c): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`[sp-32]`, r2=`0x4`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xc348`, base register `pc`, raw displacement `+0x180`, linked address 0xc348, basis `ELF load-bias-relative` (`ldr` at 0xc1c0).
  Call (`bl` at 0xc1c8): (unknown callee), target 0x337c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x337c, conditional `false`, r0=`unknown`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x337c)`, base register `r4`, raw displacement `+0x4`, raw addressing form `[r4, #4]` (`str` at 0xc1cc).
  Branch (`bls` at 0xc200): taken to 0xc208 when `? <= [sp-40] (unsigned, 32-bit)`, falls through to 0xc204 when `? > [sp-40] (unsigned, 32-bit)`.
  Call (`bl` at 0xc23c): (unknown callee), target 0x35a4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35a4, conditional `false`, r0=`unknown`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `ret(linked ELF 0x35a4)`, base register `r4`, raw displacement `+0x4`, raw addressing form `[r4, #4]` (`str` at 0xc244).
  Four-byte load: target `(unknown effective address)`, base register `r4`, raw displacement `+0x4`, raw addressing form `[r4, #4]` (`ldr` at 0xc248).
  Branch (`bne` at 0xc250): taken to 0xc268 when `? != 0x0`, falls through to 0xc254 when `? == 0x0`.
  Four-byte load: target `sp -0x38`, base register `r11`, raw displacement `-0x34` (`ldr` at 0xc258).
  Four-byte load: target `[sp-24] +0x18c`, base register `r3`, raw displacement `+0x18c` (`ldr` at 0xc25c).
  Call (`bl` at 0xc260): (unknown callee), target 0x3598, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3598, conditional `false`, r0=`[sp-56]`, r1=`[[sp-24]+396]`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0xc264): (unknown callee), target 0x3910, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3910, conditional `false`, r0=`ret(linked ELF 0x3598)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `(unknown effective address)`, base register `r0`, raw displacement `+0x4`, raw addressing form `[r0, #4]` (`ldr` at 0xc298).
  Call (`bl` at 0xc2a4): (unknown callee), target 0x373c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x373c, conditional `false`, r0=`unknown`, r1=`[sp-32]`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0xc2d4).
  Four-byte store: target `sp -0x38`, value `[sp-56]+1`, base register `r11`, raw displacement `-0x34` (`str` at 0xc300).
  Jump (`b` at 0xc304): target 0xbf34.
  Branch (`beq` at 0xc310): taken to 0xc330 when `[sp-52] == 0x0`, falls through to 0xc314 when `[sp-52] != 0x0`.
  Byte load: target `[sp-28] +0x4`, base register `r3`, raw displacement `+0x4` (`ldrb` at 0xc318).
  Branch (`beq` at 0xc324): taken to 0xc330 when `? == 0x0`, falls through to 0xc328 when `? != 0x0`.
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xc328).
  Call (`bl` at 0xc32c): (unknown callee), target 0x352c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x352c, conditional `false`, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  At the known-edge return (`ldm` at 0xc33c), r0 is `[sp-56]`.
  Known return boundary (`ldm` at 0xc33c): the modeled local edges end before the retained trailing region 0xc340 to 0xc34c (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xc340 to 0xc348 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterProcessPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xa064 to 0xa250 (exclusive), 492 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `64774f397a137c7540d8da17fa1d9e6051d6a91c38dc82e30fd6982b626561c3`.
  The native facts retain 1 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xa068).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xa074).
  Four-byte load: target `ELF load bias (unknown) +0xa24c`, base register `pc`, raw displacement `+0x1cc`, linked address 0xa24c, basis `ELF load-bias-relative` (`ldr` at 0xa078).
  Native warning (`ldr` at 0xa078): `ELF literal at linked 0xa24c has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x18`, value `unknown`, base register `r11`, raw displacement `-0x14` (`str` at 0xa07c).
  Grouped accesses: 3 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xa080, `ldr` at 0xa098, `ldr` at 0xa22c.
  Grouped accesses: 10 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xa084, `ldr` at 0xa154, `ldr` at 0xa16c.
  Four-byte load: target `r0 +0xe4`, base register `r3`, raw displacement `+0xe4` (`ldr` at 0xa088).
  Call (`bl` at 0xa08c): (unknown callee), target 0x34e4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34e4, conditional `false`, r0=`r0`, r1=`[r0+228]`, r2=`r2`, r3=`r0`.
  Four-byte store: target `sp -0x1c`, value `ret(linked ELF 0x34e4)`, base register `r11`, raw displacement `-0x18` (`str` at 0xa094).
  Four-byte load: target `[sp-20] +0xe4`, base register `r3`, raw displacement `+0xe4` (`ldr` at 0xa09c).
  Byte load: target `[[sp-20]+228] +0x2`, base register `r3`, raw displacement `+0x2` (`ldrb` at 0xa0a0).
  Four-byte store: target `sp -0x24`, value `[[[sp-20]+228]+2]:1`, base register `r11`, raw displacement `-0x20` (`str` at 0xa0a4).
  Grouped accesses: 13 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xa0a8, `ldr` at 0xa0b4, `ldr` at 0xa0c0.
  Branch (`beq` at 0xa0b0): taken to 0xa1dc when `[[[sp-20]+228]+2]:1 == 0x18`, falls through to 0xa0b4 when `[[[sp-20]+228]+2]:1 != 0x18`.
  Branch (`bgt` at 0xa0bc): taken to 0xa104 when `[[[sp-20]+228]+2]:1 > 0x18 (signed, 32-bit)`, falls through to 0xa0c0 when `[[[sp-20]+228]+2]:1 <= 0x18 (signed, 32-bit)`.
  Branch (`beq` at 0xa0c8): taken to 0xa19c when `[[[sp-20]+228]+2]:1 == 0x11`, falls through to 0xa0cc when `[[[sp-20]+228]+2]:1 != 0x11`.
  Branch (`bgt` at 0xa0d4): taken to 0xa0e8 when `[[[sp-20]+228]+2]:1 > 0x11 (signed, 32-bit)`, falls through to 0xa0d8 when `[[[sp-20]+228]+2]:1 <= 0x11 (signed, 32-bit)`.
  Branch (`beq` at 0xa0e0): taken to 0xa154 when `[[[sp-20]+228]+2]:1 == 0x10`, falls through to 0xa0e4 when `[[[sp-20]+228]+2]:1 != 0x10`.
  Branch (`beq` at 0xa0f0): taken to 0xa1b0 when `[[[sp-20]+228]+2]:1 == 0x12`, falls through to 0xa0f4 when `[[[sp-20]+228]+2]:1 != 0x12`.
  Branch (`beq` at 0xa0fc): taken to 0xa1c4 when `[[[sp-20]+228]+2]:1 == 0x14`, falls through to 0xa100 when `[[[sp-20]+228]+2]:1 != 0x14`.
  Branch (`beq` at 0xa10c): taken to 0xa20c when `[[[sp-20]+228]+2]:1 == 0x24`, falls through to 0xa110 when `[[[sp-20]+228]+2]:1 != 0x24`.
  Branch (`bgt` at 0xa118): taken to 0xa138 when `[[[sp-20]+228]+2]:1 > 0x24 (signed, 32-bit)`, falls through to 0xa11c when `[[[sp-20]+228]+2]:1 <= 0x24 (signed, 32-bit)`.
  Branch (`beq` at 0xa124): taken to 0xa154 when `[[[sp-20]+228]+2]:1 == 0x20`, falls through to 0xa128 when `[[[sp-20]+228]+2]:1 != 0x20`.
  Branch (`beq` at 0xa130): taken to 0xa1f4 when `[[[sp-20]+228]+2]:1 == 0x21`, falls through to 0xa134 when `[[[sp-20]+228]+2]:1 != 0x21`.
  Branch (`beq` at 0xa140): taken to 0xa16c when `[[[sp-20]+228]+2]:1 == 0x40`, falls through to 0xa144 when `[[[sp-20]+228]+2]:1 != 0x40`.
  Branch (`beq` at 0xa14c): taken to 0xa184 when `[[[sp-20]+228]+2]:1 == 0x80`, falls through to 0xa150 when `[[[sp-20]+228]+2]:1 != 0x80`.
  Grouped accesses: 6 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xa158, `ldr` at 0xa170, `ldr` at 0xa188.
  Call (`bl` at 0xa15c): (unknown callee), target 0xa250, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa250, conditional `false`, r0=`[sp-20]`, r1=`ret(linked ELF 0x34e4)`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa250)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa164).
  Call (`bl` at 0xa174): (unknown callee), target 0xa344, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa344, conditional `false`, r0=`[sp-20]`, r1=`ret(linked ELF 0x34e4)`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa344)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa17c).
  Call (`bl` at 0xa18c): (unknown callee), target 0xa5a8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa5a8, conditional `false`, r0=`[sp-20]`, r1=`ret(linked ELF 0x34e4)`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa5a8)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa194).
  Call (`bl` at 0xa1a0): (unknown callee), target 0xa610, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa610, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa610)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa1a8).
  Call (`bl` at 0xa1b4): (unknown callee), target 0xa6ac, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa6ac, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa6ac)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa1bc).
  Call (`bl` at 0xa1cc): (unknown callee), target 0xa71c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa71c, conditional `false`, r0=`[sp-20]`, r1=`ret(linked ELF 0x34e4)`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa71c)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa1d4).
  Call (`bl` at 0xa1e4): (unknown callee), target 0xa99c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xa99c, conditional `false`, r0=`[sp-20]`, r1=`ret(linked ELF 0x34e4)`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xa99c)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa1ec).
  Call (`bl` at 0xa1fc): (unknown callee), target 0xac40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xac40, conditional `false`, r0=`[sp-20]`, r1=`ret(linked ELF 0x34e4)`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0xac40)`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa204).
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa210).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xa21c).
  Call (`bl` at 0xa220): (unknown callee), target 0x385c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x385c, conditional `false`, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`[[[sp-20]+228]+2]:1`.
  Native warning (`mvn` at 0xa224): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0xa228).
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xa230).
  Four-byte load: target `[sp-20] +0x18c`, base register `r3`, raw displacement `+0x18c` (`ldr` at 0xa234).
  Call (`bl` at 0xa238): (unknown callee), target 0x3598, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3598, conditional `false`, r0=`[sp-28]`, r1=`[[sp-20]+396]`, r2=`unknown`, r3=`[sp-20]`.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xa23c).
  `ldm` at 0xa248 reports r0 = `[sp-32]` on the modeled known edges.
  Known return boundary (`ldm` at 0xa248): the modeled local edges end before the retained trailing region 0xa24c to 0xa250 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xa24c to 0xa24c are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterFlushOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0x8f58 to 0x9188 (exclusive), 560 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `e27553d07cc275f64f540792275835e00823da8707464b4c3a831423ff8760ab`.
  The native facts retain 2 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x8f5c).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x8f68).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0x8f6c).
  Four-byte store: target `sp -0x1c`, value `0x0`, base register `r11`, raw displacement `-0x18` (`str` at 0x8f74).
  Grouped accesses: 10 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x8f78, `ldr` at 0x8f90, `ldr` at 0x8fa0.
  Four-byte load: target `r0 +0x14c`, base register `r3`, raw displacement `+0x14c` (`ldr` at 0x8f7c).
  Branch (`bgt` at 0x8f84): taken to 0x8f90 when `[r0+332] > 0x0 (signed, 32-bit)`, falls through to 0x8f88 when `[r0+332] <= 0x0 (signed, 32-bit)`.
  Grouped accesses: 5 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x8f88, `ldr` at 0x907c, `ldr` at 0x90b0.
  Call (`bl` at 0x8f8c): (unknown callee), target 0xb374, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xb374, conditional `false`, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[r0+332]`.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x14c`, raw addressing form `[r3, #0x14c]` (`ldr` at 0x8f94).
  Branch (`ble` at 0x8f9c): taken to 0x9178 when `le flags condition (operands unknown)`, falls through to 0x8fa0 when `not (le flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x8fa4, `ldr` at 0x9124.
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0x8fa8).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]` (`ldr` at 0x8fac).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x148`, raw addressing form `[r2, #0x148]` (`ldr` at 0x8fb0).
  Four-byte load: target `(unknown effective address)`, base register `r12`, raw displacement `+0x14c`, raw addressing form `[ip, #0x14c]` (`ldr` at 0x8fb4).
  Call (`bl` at 0x8fb8): (unknown callee), target 0x33dc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33dc, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `ret(linked ELF 0x33dc)`, base register `r11`, raw displacement `-0x1c` (`str` at 0x8fc0).
  Grouped accesses: 3 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x8fc4, `ldr` at 0x90d0, `ldr` at 0x9168.
  Branch (`bge` at 0x8fcc): taken to 0x90c4 when `ret(linked ELF 0x33dc) >= 0x0 (signed, 32-bit)`, falls through to 0x8fd0 when `ret(linked ELF 0x33dc) < 0x0 (signed, 32-bit)`.
  Call (`bl` at 0x8fd0): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, conditional `false`, r0=`ret(linked ELF 0x33dc)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x33dc)`.
  Four-byte load: target `ret(linked ELF 0x34f0) +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x8fd8).
  Four-byte store: target `sp -0x24`, value `[ret(linked ELF 0x34f0)]`, base register `r11`, raw displacement `-0x20` (`str` at 0x8fdc).
  Grouped accesses: 5 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x8fe0, `ldr` at 0x8ff8, `ldr` at 0x9010.
  Branch (`beq` at 0x8fe8): taken to 0x907c when `[ret(linked ELF 0x34f0)] == 0x6b`, falls through to 0x8fec when `[ret(linked ELF 0x34f0)] != 0x6b`.
  Grouped accesses: 3 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x8fec, `ldr` at 0x9004, `ldr` at 0x901c.
  Branch (`bgt` at 0x8ff4): taken to 0x903c when `[ret(linked ELF 0x34f0)] > 0x6b (signed, 32-bit)`, falls through to 0x8ff8 when `[ret(linked ELF 0x34f0)] <= 0x6b (signed, 32-bit)`.
  Branch (`beq` at 0x9000): taken to 0x907c when `[ret(linked ELF 0x34f0)] == 0x65`, falls through to 0x9004 when `[ret(linked ELF 0x34f0)] != 0x65`.
  Branch (`bgt` at 0x900c): taken to 0x902c when `[ret(linked ELF 0x34f0)] > 0x65 (signed, 32-bit)`, falls through to 0x9010 when `[ret(linked ELF 0x34f0)] <= 0x65 (signed, 32-bit)`.
  Branch (`beq` at 0x9018): taken to 0x9050 when `[ret(linked ELF 0x34f0)] == 0xb`, falls through to 0x901c when `[ret(linked ELF 0x34f0)] != 0xb`.
  Branch (`beq` at 0x9024): taken to 0x907c when `[ret(linked ELF 0x34f0)] == 0x20`, falls through to 0x9028 when `[ret(linked ELF 0x34f0)] != 0x20`.
  Branch (`beq` at 0x9034): taken to 0x907c when `[ret(linked ELF 0x34f0)] == 0x68`, falls through to 0x9038 when `[ret(linked ELF 0x34f0)] != 0x68`.
  Branch (`bhi` at 0x9048): taken to 0x9088 when `[ret(linked ELF 0x34f0)]-110 > 0x1 (unsigned, 32-bit)`, falls through to 0x904c when `[ret(linked ELF 0x34f0)]-110 <= 0x1 (unsigned, 32-bit)`.
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0x9050).
  Four-byte store: target `sp -0x1c`, value `[sp-28]+1`, base register `r11`, raw displacement `-0x18` (`str` at 0x905c).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0x9060).
  Branch (`ble` at 0x9068): taken to 0x9070 when `[sp-28]+1 <= [sp-24] (signed, 32-bit)`, falls through to 0x906c when `[sp-28]+1 > [sp-24] (signed, 32-bit)`.
  Call (`bl` at 0x9074): (unknown callee), target 0x3628, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3628, conditional `false`, r0=`0x1`, r1=`unknown`, r2=`[sp-28]+1`, r3=`[sp-24]`.
  Call (`bl` at 0x9080): (unknown callee), target 0x340c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x340c, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`[ret(linked ELF 0x34f0)]`, r3=`unknown`.
  Call (`bl` at 0x9088): (unknown callee), target 0x34f0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34f0, conditional `false`, r0=`ret(linked ELF 0x34f0)`, r1=`unknown`, r2=`[ret(linked ELF 0x34f0)]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x9180`, base register `pc`, raw displacement `+0xe4`, linked address 0x9180, basis `ELF load-bias-relative` (`ldr` at 0x9094).
  Native warning (`ldr` at 0x9094): `ELF literal at linked 0x9180 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x2c`, value `unknown`, base register `sp`, raw displacement `+0x0` (`str` at 0x9098).
  Native warning (`mvn` at 0x909c): `mvn: unsupported effects invalidated`.
  Four-byte load: target `ELF load bias (unknown) +0x9184`, base register `pc`, raw displacement `+0xdc`, linked address 0x9184, basis `ELF load-bias-relative` (`ldr` at 0x90a0).
  Native warning (`ldr` at 0x90a0): `ELF literal at linked 0x9184 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte load: target `ret(linked ELF 0x34f0) +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0x90a4).
  Call (`bl` at 0x90ac): (unknown callee), target 0x38f8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x38f8, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`[ret(linked ELF 0x34f0)]`, r3=`[sp-20]`.
  Call (`bl` at 0x90b4): (unknown callee), target 0x34d8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34d8, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0x90c0).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x90c4, `ldr` at 0x915c.
  Four-byte load: target `[sp-20] +0x14c`, base register `r3`, raw displacement `+0x14c` (`ldr` at 0x90cc).
  Native warning (`rsb` at 0x90d4): `rsb: unsupported effects invalidated`.
  Four-byte store: target `[sp-20] +0x14c`, value `unknown`, base register `r1`, raw displacement `+0x14c` (`str` at 0x90d8).
  Branch (`bgt` at 0x90e0): taken to 0x915c when `? > 0x0 (signed, 32-bit)`, falls through to 0x90e4 when `? <= 0x0 (signed, 32-bit)`.
  Four-byte load: target `[sp-20] +0x144`, base register `r3`, raw displacement `+0x144` (`ldr` at 0x90e8).
  Byte load: target `[[sp-20]+324] +0x16`, base register `r3`, raw displacement `+0x16` (`ldrb` at 0x90ec).
  Four-byte store: target `sp -0x28`, value `[[[sp-20]+324]+22]:1`, base register `r11`, raw displacement `-0x24` (`str` at 0x90f0).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x90f4).
  Branch (`beq` at 0x90fc): taken to 0x9110 when `[[[sp-20]+324]+22]:1 == 0x10`, falls through to 0x9100 when `[[[sp-20]+324]+22]:1 != 0x10`.
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x9100).
  Branch (`beq` at 0x9108): taken to 0x911c when `[[[sp-20]+324]+22]:1 == 0x20`, falls through to 0x910c when `[[[sp-20]+324]+22]:1 != 0x20`.
  Four-byte load: target `[sp-20] +0x144`, base register `r3`, raw displacement `+0x144` (`ldr` at 0x9114).
  Call (`bl` at 0x9118): (unknown callee), target 0xd134, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xd134, conditional `false`, r0=`[[sp-20]+324]`, r1=`[sp-20]`, r2=`[[sp-20]+332]`, r3=`[sp-20]`.
  Four-byte load: target `[sp-20] +0x144`, base register `r2`, raw displacement `+0x144` (`ldr` at 0x912c).
  Call (`bl` at 0x9134): (unknown callee), target 0xc954, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xc954, conditional `false`, r0=`[sp-20]+204`, r1=`[[sp-20]+324]`, r2=`0x0`, r3=`[sp-20]+204`.
  Four-byte load: target `[sp-20] +0x144`, base register `r3`, raw displacement `+0x144` (`ldr` at 0x9144).
  Call (`bl` at 0x914c): (unknown callee), target 0xd064, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xd064, conditional `false`, r0=`[sp-20]`, r1=`[[sp-20]+324]`, r2=`0x0`, r3=`[sp-20]`.
  Call (`bl` at 0x9154): (unknown callee), target 0xb374, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xb374, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0x9158): target 0x8f90.
  Four-byte load: target `[sp-20] +0x148`, base register `r3`, raw displacement `+0x148` (`ldr` at 0x9164).
  Four-byte store: target `[sp-20] +0x148`, value `([[sp-20]+328] + [sp-32])`, base register `r1`, raw displacement `+0x148` (`str` at 0x9170).
  Jump (`b` at 0x9174): target 0x8f90.
  The native return (`ldm` at 0x917c) retains r0 = `unknown`.
  Known return boundary (`ldm` at 0x917c): the modeled local edges end before the retained trailing region 0x9180 to 0x9188 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0x9180 to 0x9184 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterDoOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xae20 to 0xaed8 (exclusive), 184 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `1298d36864f30c082a51296a731e237c235d40eba408137eb42a498ba5c2d060`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xae24).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xae30).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xae34).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xae38).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0xae3c).
  Four-byte load: target `sp +0x14`, base register `r11`, raw displacement `+0x18` (`ldr` at 0xae40).
  Byte store: target `sp -0x21`, value `([sp+20] & 0xff)`, base register `r11`, raw displacement `-0x1d` (`strb` at 0xae44).
  Four-byte load: target `sp +0x0`, base register `r11`, raw displacement `+0x4` (`ldr` at 0xae48).
  Four-byte store: target `sp -0x44`, value `[sp]`, base register `sp`, raw displacement `+0x0` (`str` at 0xae4c).
  Four-byte load: target `sp +0x4`, base register `r11`, raw displacement `+0x8` (`ldr` at 0xae50).
  Four-byte store: target `sp -0x40`, value `[sp+4]`, base register `sp`, raw displacement `+0x4` (`str` at 0xae54).
  Four-byte load: target `sp +0x8`, base register `r11`, raw displacement `+0xc` (`ldr` at 0xae58).
  Four-byte store: target `sp -0x3c`, value `[sp+8]`, base register `sp`, raw displacement `+0x8` (`str` at 0xae5c).
  Four-byte load: target `sp +0xc`, base register `r11`, raw displacement `+0x10` (`ldr` at 0xae60).
  Four-byte store: target `sp -0x38`, value `[sp+12]`, base register `sp`, raw displacement `+0xc` (`str` at 0xae64).
  Four-byte load: target `sp +0x10`, base register `r11`, raw displacement `+0x14` (`ldr` at 0xae68).
  Four-byte store: target `sp -0x34`, value `[sp+16]`, base register `sp`, raw displacement `+0x10` (`str` at 0xae6c).
  Byte load: target `sp -0x21`, base register `r11`, raw displacement `-0x1d` (`ldrb` at 0xae70).
  Four-byte store: target `sp -0x30`, value `([sp+20] & 0xff)`, base register `sp`, raw displacement `+0x14` (`str` at 0xae74).
  Four-byte store: target `sp -0x2c`, value `sp-40`, base register `sp`, raw displacement `+0x18` (`str` at 0xae7c).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xae80, `ldr` at 0xaec4.
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xae84).
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xae88).
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xae8c).
  Call (`bl` at 0xae90): (unknown callee), target 0x367c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x367c, conditional `false`, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xae94, `ldr` at 0xaea4.
  Byte load: target `[sp-40] +0x16`, base register `r3`, raw displacement `+0x16` (`ldrb` at 0xae98).
  Branch (`bne` at 0xaea0): taken to 0xaec4 when `[[sp-40]+22]:1 != 0x40`, falls through to 0xaea4 when `[[sp-40]+22]:1 == 0x40`.
  Four-byte load: target `[sp-40] +0x1c`, base register `r3`, raw displacement `+0x1c` (`ldr` at 0xaea8).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0xaeac).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xaeb0).
  Two-byte load: target `[sp-20] +0xf4`, base register `r3`, raw displacement `+0xf4` (`ldrh` at 0xaeb4).
  Byte store: target `[sp-40] +0x17`, value `([[sp-20]+244]:2 & 0xff)`, base register `r2`, raw displacement `+0x17` (`strb` at 0xaeb8).
  Byte load: target `[sp-40] +0x17`, base register `r2`, raw displacement `+0x17` (`ldrb` at 0xaebc).
  Byte store: target `[[sp-40]+28] +0x3`, value `([[sp-40]+23]:1 & 0xff)`, base register `r1`, raw displacement `+0x3` (`strb` at 0xaec0).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0xaec8).
  Call (`bl` at 0xaecc): (unknown callee), target 0xaed8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xaed8, conditional `false`, r0=`[sp-20]`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Known-edge return (`ldm` at 0xaed4): recorded r0 is `ret(linked ELF 0xaed8)`.

- `RouterConfigure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xfee0 to 0x100d0 (exclusive), 496 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `69e6b3bf0b35f5c59805c62a6f14555b051afa65aa2ce23134dec7afa4626d93`.
  The native facts retain 1 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xfee4).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xfef0).
  Four-byte store: target `sp -0x68`, value `sp-100`, base register `r11`, raw displacement `-0x64` (`str` at 0xfef8).
  Grouped accesses: 2 4-byte load records with target `sp -0x68`, base register `r11`, raw displacement `-0x64`, including `ldr` at 0xfefc, `ldr` at 0xff6c.
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x1c4`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xff00).
  Warning class: 8 native records report `ELF literal at linked 0x100cc has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xff00, `ldr` at 0xff1c, `ldr` at 0xff38.
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xff04, `ldr` at 0xff20, `ldr` at 0xff3c.
  Byte store: target `sp -0x64`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xff0c).
  Grouped accesses: 9 4-byte load records with target `sp -0x68`, base register `r11`, raw displacement `-0x64`, including `ldr` at 0xff14, `ldr` at 0xff30, `ldr` at 0xff4c.
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x1a8`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xff1c).
  Byte store: target `sp -0x63`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xff28).
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x18c`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xff38).
  Byte store: target `sp -0x62`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xff44).
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x170`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xff54).
  Byte store: target `sp -0x61`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xff5c).
  Four-byte store: target `sp -0x68`, value `sp-96`, base register `r11`, raw displacement `-0x64` (`str` at 0xff68).
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x154`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xff70).
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, raw addressing form `[r3, #4]`, including `ldr` at 0xff74, `ldr` at 0xff90, `ldr` at 0xffac.
  Byte store: target `sp -0x60`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xff7c).
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x138`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xff8c).
  Byte store: target `sp -0x5f`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xff98).
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x11c`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xffa8).
  Byte store: target `sp -0x5e`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xffb4).
  Four-byte load: target `ELF load bias (unknown) +0x100cc`, base register `pc`, raw displacement `+0x100`, linked address 0x100cc, basis `ELF load-bias-relative` (`ldr` at 0xffc4).
  Byte store: target `sp -0x5d`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0xffcc).
  Four-byte store: target `sp -0x68`, value `sp-92`, base register `r11`, raw displacement `-0x64` (`str` at 0xffd8).
  Byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldrb` at 0xffe8).
  Byte store: target `sp -0x6c`, value `unknown`, base register `r11`, raw displacement `-0x68` (`strb` at 0xffec).
  Byte load: target `sp -0x6c`, base register `r11`, raw displacement `-0x68` (`ldrb` at 0xfff0).
  Byte store: target `sp -0x6c`, value `unknown`, base register `r11`, raw displacement `-0x68` (`strb` at 0xfffc).
  Byte load: target `sp -0x6c`, base register `r11`, raw displacement `-0x68` (`ldrb` at 0x10000).
  Byte store: target `(unknown effective address)`, value `([sp-108]:1 & 0xff)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`strb` at 0x10004).
  Warning class: 4 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0x10008, `mvn` at 0x1001c, `mvn` at 0x10034.
  Four-byte store: target `(unknown effective address)`, value `sp-100`, base register `r2`, raw displacement `+0x4`, raw addressing form `[r2, #4]` (`str` at 0x10018).
  Byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`ldrb` at 0x10028).
  Byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0`, raw addressing form `[r2]` (`strb` at 0x10030).
  Native warning (`rsb` at 0x10048): `rsb: unsupported effects invalidated`.
  Four-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`ldr` at 0x1004c).
  Four-byte store: target `sp -0x70`, value `unknown`, base register `r11`, raw displacement `-0x6c` (`str` at 0x10050).
  Four-byte load: target `sp -0x70`, base register `r11`, raw displacement `-0x6c` (`ldr` at 0x10058).
  Four-byte store: target `sp -0x70`, value `unknown`, base register `r11`, raw displacement `-0x6c` (`str` at 0x10068).
  Four-byte load: target `sp -0x70`, base register `r11`, raw displacement `-0x6c` (`ldr` at 0x1006c).
  Four-byte store: target `(unknown effective address)`, value `[sp-112]`, base register `r1`, raw displacement `+0x0`, raw addressing form `[r1]` (`str` at 0x10070).
  Four-byte store: target `sp -0x88`, value `sp-28`, base register `sp`, raw displacement `+0x0` (`str` at 0x10078).
  Four-byte store: target `sp -0x84`, value `0x0`, base register `sp`, raw displacement `+0x4` (`str` at 0x10080).
  Four-byte store: target `sp -0x80`, value `0x0`, base register `sp`, raw displacement `+0x8` (`str` at 0x10088).
  Four-byte store: target `sp -0x7c`, value `0x0`, base register `sp`, raw displacement `+0xc` (`str` at 0x10090).
  Four-byte store: target `sp -0x78`, value `0x0`, base register `sp`, raw displacement `+0x10` (`str` at 0x10098).
  Four-byte store: target `sp -0x74`, value `0x0`, base register `sp`, raw displacement `+0x14` (`str` at 0x100a0).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x100a4, `ldr` at 0x100b8.
  Call (`bl` at 0x100b4): (unknown callee), target 0x3484, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3484, conditional `false`, r0=`[sp-20]`, r1=`0x0`, r2=`0x14`, r3=`0x1`.
  Call (`bl` at 0x100c0): (unknown callee), target 0x33c4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33c4, conditional `false`, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  At the known-edge return (`ldm` at 0x100c8), r0 is `ret(linked ELF 0x33c4)`.
  Known return boundary (`ldm` at 0x100c8): the modeled local edges end before the retained trailing region 0x100cc to 0x100d0 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0x100cc to 0x100cc are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterStartServer`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xe7bc to 0xe970 (exclusive), 436 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `b10dcea9febc05c1ab30b1ede3667b6d3776aba265d02bdd302a3d7d44ef160f`.
  The native facts retain 3 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xe7c0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xe7cc).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xe7d0).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xe7d4).
  Four-byte load: target `ELF load bias (unknown) +0xe964`, base register `pc`, raw displacement `+0x184`, linked address 0xe964, basis `ELF load-bias-relative` (`ldr` at 0xe7d8).
  Native warning (`ldr` at 0xe7d8): `ELF literal at linked 0xe964 has relocation, writable or unmapped bytes; value unknown`.
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0xe7dc).
  Warning class: 4 native records report `mvn: unsupported effects invalidated`, including `mvn` at 0xe7e0, `mvn` at 0xe7e8, `mvn` at 0xe7fc.
  Four-byte store: target `sp -0x28`, value `unknown`, base register `r11`, raw displacement `-0x24` (`str` at 0xe7e4).
  Four-byte store: target `sp -0x2c`, value `unknown`, base register `r11`, raw displacement `-0x28` (`str` at 0xe7ec).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0xe7f0, `ldr` at 0xe918.
  Branch (`bne` at 0xe7f8): taken to 0xe81c when `[sp-24] != 0x0`, falls through to 0xe7fc when `[sp-24] == 0x0`.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xe800).
  Four-byte load: target `ELF load bias (unknown) +0xe968`, base register `pc`, raw displacement `+0x158`, linked address 0xe968, basis `ELF load-bias-relative` (`ldr` at 0xe808).
  Warning class: 3 native records report `ELF literal at linked 0xe968 has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe808, `ldr` at 0xe86c, `ldr` at 0xe878.
  Call (`bl` at 0xe80c): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`unknown`, r1=`[sp-32]`, r2=`0x0`, r3=`unknown`.
  Grouped accesses: 5 4-byte store records with target `sp -0x34`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x30`, including `str` at 0xe814, `str` at 0xe838, `str` at 0xe894.
  Call (`bl` at 0xe824): (unknown callee), target 0x382c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x382c, conditional `false`, r0=`sp-36`, r1=`r1`, r2=`r2`, r3=`sp-36`.
  Branch (`beq` at 0xe830): taken to 0xe840 when `ret(linked ELF 0x382c) == 0x0`, falls through to 0xe834 when `ret(linked ELF 0x382c) != 0x0`.
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xe840).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xe844).
  Call (`bl` at 0xe84c): (unknown callee), target 0x3370, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3370, conditional `false`, r0=`[sp-36]`, r1=`[sp-20]`, r2=`0x0`, r3=`ret(linked ELF 0x382c)`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x3370)`, base register `r11`, raw displacement `-0x24` (`str` at 0xe854).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xe858, `ldr` at 0xe8ac.
  Branch (`blt` at 0xe860): taken to 0xe89c when `ret(linked ELF 0x3370) < 0x0 (signed, 32-bit)`, falls through to 0xe864 when `ret(linked ELF 0x3370) >= 0x0 (signed, 32-bit)`.
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xe864).
  Four-byte store: target `sp -0x3c`, value `[sp-20]`, base register `sp`, raw displacement `+0x0` (`str` at 0xe868).
  Four-byte load: target `ELF load bias (unknown) +0xe968`, base register `pc`, raw displacement `+0xf4`, linked address 0xe968, basis `ELF load-bias-relative` (`ldr` at 0xe86c).
  Four-byte store: target `sp -0x38`, value `unknown`, base register `sp`, raw displacement `+0x4` (`str` at 0xe870).
  Four-byte load: target `ELF load bias (unknown) +0xe968`, base register `pc`, raw displacement `+0xe8`, linked address 0xe968, basis `ELF load-bias-relative` (`ldr` at 0xe878).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xe880).
  Call (`bl` at 0xe884): (unknown callee), target 0x376c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x376c, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`0x0`, r3=`[sp-36]`.
  Grouped accesses: 3 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xe888, `ldr` at 0xe8f0, `ldr` at 0xe90c.
  Call (`bl` at 0xe88c): (unknown callee), target 0x33b8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33b8, conditional `false`, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xe89c, `ldr` at 0xe8d4.
  Call (`bl` at 0xe8a0): (unknown callee), target 0x37a8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37a8, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3370)`.
  Four-byte store: target `sp -0x28`, value `ret(linked ELF 0x37a8)`, base register `r11`, raw displacement `-0x24` (`str` at 0xe8a8).
  Branch (`bge` at 0xe8b4): taken to 0xe8c4 when `ret(linked ELF 0x37a8) >= 0x0 (signed, 32-bit)`, falls through to 0xe8b8 when `ret(linked ELF 0x37a8) < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0xe96c`, base register `pc`, raw displacement `+0xa0`, linked address 0xe96c, basis `ELF load-bias-relative` (`ldr` at 0xe8c4).
  Warning class: 2 native records report `ELF literal at linked 0xe96c has relocation, writable or unmapped bytes; value unknown`, including `ldr` at 0xe8c4, `ldr` at 0xe938.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, raw addressing form `[r3]`, including `ldr` at 0xe8c8, `ldr` at 0xe93c.
  Branch (`beq` at 0xe8d0): taken to 0xe904 when `? == 0x0`, falls through to 0xe8d4 when `? != 0x0`.
  Call (`bl` at 0xe8d8): (unknown callee), target 0x3664, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3664, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x2c`, value `ret(linked ELF 0x3664)`, base register `r11`, raw displacement `-0x28` (`str` at 0xe8e0).
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0xe8e4).
  Branch (`bge` at 0xe8ec): taken to 0xe904 when `ret(linked ELF 0x3664) >= 0x0 (signed, 32-bit)`, falls through to 0xe8f0 when `ret(linked ELF 0x3664) < 0x0 (signed, 32-bit)`.
  Call (`bl` at 0xe8f4): (unknown callee), target 0x33b8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33b8, conditional `false`, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0x3664)`.
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xe904).
  Four-byte store: target `sp -0x3c`, value `[sp-28]`, base register `sp`, raw displacement `+0x0` (`str` at 0xe908).
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0xe910).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xe914).
  Call (`bl` at 0xe91c): (unknown callee), target 0xed00, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xed00, conditional `false`, r0=`unknown`, r1=`[sp-44]`, r2=`[sp-20]`, r3=`[sp-24]`.
  Four-byte store: target `sp -0x30`, value `ret(linked ELF 0xed00)`, base register `r11`, raw displacement `-0x2c` (`str` at 0xe924).
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0xe928, `ldr` at 0xe930, `ldr` at 0xe948.
  Call (`bl` at 0xe92c): (unknown callee), target 0xedbc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xedbc, conditional `false`, r0=`ret(linked ELF 0xed00)`, r1=`unknown`, r2=`unknown`, r3=`ret(linked ELF 0xed00)`.
  Call (`bl` at 0xe934): (unknown callee), target 0x100d0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x100d0, conditional `false`, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xe96c`, base register `pc`, raw displacement `+0x2c`, linked address 0xe96c, basis `ELF load-bias-relative` (`ldr` at 0xe938).
  Branch (`beq` at 0xe944): taken to 0xe950 when `? == 0x0`, falls through to 0xe948 when `? != 0x0`.
  Call (`bl` at 0xe94c): (unknown callee), target 0x10134, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x10134, conditional `false`, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x30`, base register `r11`, raw displacement `-0x2c` (`ldr` at 0xe950).
  Four-byte store: target `sp -0x34`, value `[sp-48]`, base register `r11`, raw displacement `-0x30` (`str` at 0xe954).
  Four-byte load: target `sp -0x34`, base register `r11`, raw displacement `-0x30` (`ldr` at 0xe958).
  `ldm` at 0xe960 reports r0 = `unknown` on the modeled known edges.
  Known return boundary (`ldm` at 0xe960): the modeled local edges end before the retained trailing region 0xe964 to 0xe970 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xe964 to 0xe96c are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterMakeConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xd894 to 0xd924 (exclusive), 144 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `6af3ac4b85461870d71356b973cff38cc47ee9ec135f81b902264be4b36b628c`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xd898).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xd8a4).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xd8a8).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xd8ac).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0xd8b0).
  Four-byte load: target `ELF load bias (unknown) +0xd920`, base register `pc`, raw displacement `+0x64`, linked address 0xd920, basis `ELF load-bias-relative` (`ldr` at 0xd8b4).
  Four-byte store: target `sp -0x24`, value `0xc241`, base register `r11`, raw displacement `-0x20` (`str` at 0xd8b8).
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0xd8bc).
  Four-byte store: target `sp -0x44`, value `r3`, base register `sp`, raw displacement `+0x0` (`str` at 0xd8c0).
  Four-byte load: target `sp +0x0`, base register `r11`, raw displacement `+0x4` (`ldr` at 0xd8c4).
  Four-byte store: target `sp -0x40`, value `[sp]`, base register `sp`, raw displacement `+0x4` (`str` at 0xd8c8).
  Four-byte store: target `sp -0x3c`, value `0x0`, base register `sp`, raw displacement `+0x8` (`str` at 0xd8d0).
  Four-byte load: target `sp +0x4`, base register `r11`, raw displacement `+0x8` (`ldr` at 0xd8d4).
  Four-byte store: target `sp -0x38`, value `[sp+4]`, base register `sp`, raw displacement `+0xc` (`str` at 0xd8d8).
  Four-byte load: target `sp +0x8`, base register `r11`, raw displacement `+0xc` (`ldr` at 0xd8dc).
  Four-byte store: target `sp -0x34`, value `[sp+8]`, base register `sp`, raw displacement `+0x10` (`str` at 0xd8e0).
  Four-byte load: target `sp +0xc`, base register `r11`, raw displacement `+0x10` (`ldr` at 0xd8e4).
  Four-byte store: target `sp -0x30`, value `[sp+12]`, base register `sp`, raw displacement `+0x14` (`str` at 0xd8e8).
  Four-byte load: target `sp +0x10`, base register `r11`, raw displacement `+0x14` (`ldr` at 0xd8ec).
  Four-byte store: target `sp -0x2c`, value `[sp+16]`, base register `sp`, raw displacement `+0x18` (`str` at 0xd8f0).
  Four-byte store: target `sp -0x28`, value `0x0`, base register `sp`, raw displacement `+0x1c` (`str` at 0xd8f8).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xd8fc).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xd900).
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xd904).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0xd908).
  Call (`bl` at 0xd90c): (unknown callee), target 0xdbb8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xdbb8, conditional `false`, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0xc241`.
  The native return (`ldm` at 0xd91c) retains r0 = `ret(linked ELF 0xdbb8)`.
  Known return boundary (`ldm` at 0xd91c): the modeled local edges end before the retained trailing region 0xd920 to 0xd924 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xd920 to 0xd920 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterRetryConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xda44 to 0xdb08 (exclusive), 196 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `cdafa545076419d9520bea0b27a41790ad8cb67c24c2a795414133bf8ec2d665`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xda48).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xda54).
  Grouped accesses: 9 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xda58, `ldr` at 0xda70, `ldr` at 0xda80.
  Four-byte load: target `r0 +0x18`, base register `r3`, raw displacement `+0x18` (`ldr` at 0xda5c).
  Four-byte store: target `sp -0x18`, value `unknown`, base register `r11`, raw displacement `-0x14` (`str` at 0xda6c).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xda78).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xda7c).
  Four-byte load: target `r0 +0x20`, base register `r3`, raw displacement `+0x20` (`ldr` at 0xda84).
  Four-byte store: target `sp -0x38`, value `[r0+32]`, base register `sp`, raw displacement `+0x0` (`str` at 0xda88).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xda8c).
  Four-byte load: target `ELF load bias (unknown) +0xdb04`, base register `pc`, raw displacement `+0x6c`, linked address 0xdb04, basis `ELF load-bias-relative` (`ldr` at 0xda90).
  Two-byte load: target `r0 +0x156`, base register `r2`, raw displacement `+0x0` (`ldrsh` at 0xda94).
  Four-byte store: target `sp -0x34`, value `unknown`, base register `sp`, raw displacement `+0x4` (`str` at 0xda98).
  Four-byte store: target `sp -0x30`, value `r0+16`, base register `sp`, raw displacement `+0x8` (`str` at 0xdaa4).
  Four-byte load: target `r0 +0xb0`, base register `r3`, raw displacement `+0xb0` (`ldr` at 0xdaac).
  Four-byte store: target `sp -0x2c`, value `[r0+176]`, base register `sp`, raw displacement `+0xc` (`str` at 0xdab0).
  Four-byte load: target `r0 +0xb4`, base register `r3`, raw displacement `+0xb4` (`ldr` at 0xdab8).
  Four-byte store: target `sp -0x28`, value `[r0+180]`, base register `sp`, raw displacement `+0x10` (`str` at 0xdabc).
  Four-byte load: target `r0 +0xb8`, base register `r3`, raw displacement `+0xb8` (`ldr` at 0xdac4).
  Four-byte store: target `sp -0x24`, value `[r0+184]`, base register `sp`, raw displacement `+0x14` (`str` at 0xdac8).
  Four-byte load: target `r0 +0xbc`, base register `r3`, raw displacement `+0xbc` (`ldr` at 0xdad0).
  Four-byte store: target `sp -0x20`, value `[r0+188]`, base register `sp`, raw displacement `+0x18` (`str` at 0xdad4).
  Four-byte load: target `r0 +0x24`, base register `r3`, raw displacement `+0x24` (`ldr` at 0xdadc).
  Four-byte store: target `sp -0x1c`, value `[r0+36]+1`, base register `sp`, raw displacement `+0x1c` (`str` at 0xdae4).
  Four-byte load: target `r0 +0x8`, base register `r12`, raw displacement `+0x8` (`ldr` at 0xdaec).
  Four-byte load: target `r0 +0xc`, base register `lr`, raw displacement `+0xc` (`ldr` at 0xdaf0).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xdaf4).
  Call (`bl` at 0xdaf8): (unknown callee), target 0xdbb8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xdbb8, conditional `false`, r0=`r0+44`, r1=`[r0+8]`, r2=`[r0+12]`, r3=`[sp-24]`.
  Known-edge return (`ldm` at 0xdb00): recorded r0 is `ret(linked ELF 0xdbb8)`.
  Known return boundary (`ldm` at 0xdb00): the modeled local edges end before the retained trailing region 0xdb04 to 0xdb08 (exclusive). This does not prove complete code-only function extent.
  The decoded records from 0xdb04 to 0xdb04 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterConnectionError`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xd550 to 0xd5e8 (exclusive), 152 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `43aa34456d43a91263d00bd76d9301ef02956ec0d6caf088030130f9d58c002c`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xd554).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xd560).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xd564).
  Four-byte load: target `r0 +0x18`, base register `r3`, raw displacement `+0x18` (`ldr` at 0xd568).
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0xd570).
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xd574, `ldr` at 0xd580, `ldr` at 0xd58c.
  Branch (`beq` at 0xd57c): taken to 0xd5b8 when `[sp-28] == 0x8`, falls through to 0xd580 when `[sp-28] != 0x8`.
  Branch (`bgt` at 0xd588): taken to 0xd59c when `[sp-28] > 0x8 (signed, 32-bit)`, falls through to 0xd58c when `[sp-28] <= 0x8 (signed, 32-bit)`.
  Branch (`beq` at 0xd594): taken to 0xd5ac when `[sp-28] == 0x1`, falls through to 0xd598 when `[sp-28] != 0x1`.
  Branch (`beq` at 0xd5a4): taken to 0xd5b8 when `[sp-28] == 0x10`, falls through to 0xd5a8 when `[sp-28] != 0x10`.
  Four-byte load: target `ELF load bias (unknown) +0xd5e0`, base register `pc`, raw displacement `+0x2c`, linked address 0xd5e0, basis `ELF load-bias-relative` (`ldr` at 0xd5ac).
  Four-byte store: target `sp -0x18`, value `0xfffffed2`, base register `r11`, raw displacement `-0x14` (`str` at 0xd5b0).
  Four-byte load: target `ELF load bias (unknown) +0xd5e4`, base register `pc`, raw displacement `+0x24`, linked address 0xd5e4, basis `ELF load-bias-relative` (`ldr` at 0xd5b8).
  Four-byte store: target `sp -0x18`, value `0xfffffecc`, base register `r11`, raw displacement `-0x14` (`str` at 0xd5bc).
  Native warning (`mvn` at 0xd5c4): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x18`, value `unknown`, base register `r11`, raw displacement `-0x14` (`str` at 0xd5c8).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xd5cc).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xd5d0).
  Call (`bl` at 0xd5d4): (unknown callee), target 0x3604, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3604, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`r2`, r3=`unknown`.
  At the known-edge return (`ldm` at 0xd5dc), r0 is `ret(linked ELF 0x3604)`.
  Known return boundary (`ldm` at 0xd5dc): the modeled local edges end before the retained trailing region 0xd5e0 to 0xd5e8 (exclusive). This does not prove complete code-only function extent.
  Undecoded range: 0xd5e0 to 0xd5e8 (exclusive), 8 bytes `d2feffffccfeffff`. Native warning: `undecodable bytes at 0xd5e0-0xd5e8; fall-through stops at this range`. This raw region remains in the full selected span and supplies no decoded effects.

- `RouterConnectionLost`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xd3bc to 0xd488 (exclusive), 204 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `5a79814ee39e120c2c837c707dd5fc42d2f12eec476c397f17b8651eddc82605`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xd3c0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xd3cc).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xd3d0, `ldr` at 0xd418.
  Grouped accesses: 2 4-byte load records with target `r0 +0x18`, base register `r3`, raw displacement `+0x18`, including `ldr` at 0xd3d4, `ldr` at 0xd41c.
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0xd3dc).
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xd3e0, `ldr` at 0xd3ec, `ldr` at 0xd3f8.
  Branch (`beq` at 0xd3e8): taken to 0xd444 when `[sp-28] == 0x8`, falls through to 0xd3ec when `[sp-28] != 0x8`.
  Branch (`bgt` at 0xd3f4): taken to 0xd408 when `[sp-28] > 0x8 (signed, 32-bit)`, falls through to 0xd3f8 when `[sp-28] <= 0x8 (signed, 32-bit)`.
  Branch (`beq` at 0xd400): taken to 0xd418 when `[sp-28] == 0x1`, falls through to 0xd404 when `[sp-28] != 0x1`.
  Branch (`beq` at 0xd410): taken to 0xd450 when `[sp-28] == 0x10`, falls through to 0xd414 when `[sp-28] != 0x10`.
  Branch (`bne` at 0xd428): taken to 0xd438 when `? != 0x80`, falls through to 0xd42c when `? == 0x80`.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xd42c, `ldr` at 0xd464.
  Call (`bl` at 0xd430): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xd478`, base register `pc`, raw displacement `+0x38`, linked address 0xd478, basis `ELF load-bias-relative` (`ldr` at 0xd438).
  Four-byte store: target `sp -0x18`, value `0xfffffed4`, base register `r11`, raw displacement `-0x14` (`str` at 0xd43c).
  Four-byte load: target `ELF load bias (unknown) +0xd47c`, base register `pc`, raw displacement `+0x30`, linked address 0xd47c, basis `ELF load-bias-relative` (`ldr` at 0xd444).
  Four-byte store: target `sp -0x18`, value `0xfffffece`, base register `r11`, raw displacement `-0x14` (`str` at 0xd448).
  Four-byte load: target `ELF load bias (unknown) +0xd480`, base register `pc`, raw displacement `+0x28`, linked address 0xd480, basis `ELF load-bias-relative` (`ldr` at 0xd450).
  Four-byte store: target `sp -0x18`, value `0xfffffecd`, base register `r11`, raw displacement `-0x14` (`str` at 0xd454).
  Four-byte load: target `ELF load bias (unknown) +0xd484`, base register `pc`, raw displacement `+0x20`, linked address 0xd484, basis `ELF load-bias-relative` (`ldr` at 0xd45c).
  Four-byte store: target `sp -0x18`, value `0xfffffed1`, base register `r11`, raw displacement `-0x14` (`str` at 0xd460).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xd468).
  Call (`bl` at 0xd46c): (unknown callee), target 0x3604, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3604, conditional `false`, r0=`r0`, r1=`unknown`, r2=`r2`, r3=`unknown`.
  `ldm` at 0xd474 reports r0 = `unknown` on the modeled known edges.
  Known return boundary (`ldm` at 0xd474): the modeled local edges end before the retained trailing region 0xd478 to 0xd488 (exclusive). This does not prove complete code-only function extent.
  Undecoded range: 0xd478 to 0xd488 (exclusive), 16 bytes `d4feffffcefeffffcdfeffffd1feffff`. Native warning: `undecodable bytes at 0xd478-0xd488; fall-through stops at this range`. This raw region remains in the full selected span and supplies no decoded effects.

- `RouterConnectionTimeout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xd488 to 0xd550 (exclusive), 200 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `7a77d647edef531e996b6cffd51aed30ccd18435e200656a421cf4dd7279fe92`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xd48c).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xd498).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xd49c, `ldr` at 0xd4e4.
  Grouped accesses: 2 4-byte load records with target `r0 +0x18`, base register `r3`, raw displacement `+0x18`, including `ldr` at 0xd4a0, `ldr` at 0xd4e8.
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0xd4a8).
  Grouped accesses: 4 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xd4ac, `ldr` at 0xd4b8, `ldr` at 0xd4c4.
  Branch (`beq` at 0xd4b4): taken to 0xd510 when `[sp-28] == 0x8`, falls through to 0xd4b8 when `[sp-28] != 0x8`.
  Branch (`bgt` at 0xd4c0): taken to 0xd4d4 when `[sp-28] > 0x8 (signed, 32-bit)`, falls through to 0xd4c4 when `[sp-28] <= 0x8 (signed, 32-bit)`.
  Branch (`beq` at 0xd4cc): taken to 0xd4e4 when `[sp-28] == 0x1`, falls through to 0xd4d0 when `[sp-28] != 0x1`.
  Branch (`beq` at 0xd4dc): taken to 0xd51c when `[sp-28] == 0x10`, falls through to 0xd4e0 when `[sp-28] != 0x10`.
  Branch (`bne` at 0xd4f4): taken to 0xd504 when `? != 0x80`, falls through to 0xd4f8 when `? == 0x80`.
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0xd4f8, `ldr` at 0xd530.
  Call (`bl` at 0xd4fc): (unknown callee), target 0x3454, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3454, conditional `false`, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Native warning (`mvn` at 0xd504): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x18`, value `unknown`, base register `r11`, raw displacement `-0x14` (`str` at 0xd508).
  Four-byte load: target `ELF load bias (unknown) +0xd544`, base register `pc`, raw displacement `+0x2c`, linked address 0xd544, basis `ELF load-bias-relative` (`ldr` at 0xd510).
  Four-byte store: target `sp -0x18`, value `0xfffffece`, base register `r11`, raw displacement `-0x14` (`str` at 0xd514).
  Four-byte load: target `ELF load bias (unknown) +0xd548`, base register `pc`, raw displacement `+0x24`, linked address 0xd548, basis `ELF load-bias-relative` (`ldr` at 0xd51c).
  Four-byte store: target `sp -0x18`, value `0xfffffecd`, base register `r11`, raw displacement `-0x14` (`str` at 0xd520).
  Four-byte load: target `ELF load bias (unknown) +0xd54c`, base register `pc`, raw displacement `+0x1c`, linked address 0xd54c, basis `ELF load-bias-relative` (`ldr` at 0xd528).
  Four-byte store: target `sp -0x18`, value `0xfffffed0`, base register `r11`, raw displacement `-0x14` (`str` at 0xd52c).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xd534).
  Call (`bl` at 0xd538): (unknown callee), target 0x3604, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3604, conditional `false`, r0=`unknown`, r1=`unknown`, r2=`r2`, r3=`unknown`.
  The native return (`ldm` at 0xd540) retains r0 = `unknown`.
  Known return boundary (`ldm` at 0xd540): the modeled local edges end before the retained trailing region 0xd544 to 0xd550 (exclusive). This does not prove complete code-only function extent.
  Undecoded range: 0xd544 to 0xd550 (exclusive), 12 bytes `cefeffffcdfeffffd0feffff`. Native warning: `undecodable bytes at 0xd544-0xd550; fall-through stops at this range`. This raw region remains in the full selected span and supplies no decoded effects.

- `RouterStartInput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0x9a38 to 0x9ab0 (exclusive), 120 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `83c29dd6bfbd976d066b2e2e345250b5cae14500a6b63170b7ed69775bb275a5`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x9a3c).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x9a48).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x9a4c, `ldr` at 0x9a5c.
  Four-byte load: target `r0 +0xe4`, base register `r3`, raw displacement `+0xe4` (`ldr` at 0x9a50).
  Branch (`beq` at 0x9a58): taken to 0x9a68 when `[r0+228] == 0x0`, falls through to 0x9a5c when `[r0+228] != 0x0`.
  Four-byte load: target `r0 +0xe4`, base register `r3`, raw displacement `+0xe4` (`ldr` at 0x9a60).
  Call (`bl` at 0x9a64): (unknown callee), target 0x3808, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3808, conditional `false`, r0=`[r0+228]`, r1=`r1`, r2=`r2`, r3=`r0`.
  Grouped accesses: 5 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x9a68, `ldr` at 0x9a74, `ldr` at 0x9a80.
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0xe8`, raw addressing form `[r2, #0xe8]` (`str` at 0x9a70).
  Four-byte store: target `[sp-20] +0xe4`, value `0x0`, base register `r2`, raw displacement `+0xe4` (`str` at 0x9a7c).
  Four-byte store: target `[sp-20] +0xf0`, value `0x0`, base register `r2`, raw displacement `+0xf0` (`str` at 0x9a88).
  Four-byte store: target `[sp-20] +0xec`, value `0x8`, base register `r2`, raw displacement `+0xec` (`str` at 0x9a94).
  Two-byte store: target `[sp-20] +0xf6`, value `0x0`, base register `r2`, raw displacement `+0xf6` (`strh` at 0x9aa0).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0x9aa4).
  Call (`bl` at 0x9aa8): (unknown callee), target 0xd180, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xd180, conditional `false`, r0=`[sp-20]`, r1=`unknown`, r2=`[sp-20]`, r3=`0x0`.
  Four-byte load: target `sp -0x14`, base register `sp`, raw displacement `+0x0` (`ldm` at 0x9aac).
  Known-edge return (`ldm` at 0x9aac): recorded r0 is `ret(linked ELF 0xd180)`.

- `RouterCheckOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0xd354 to 0xd3bc (exclusive), 104 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `636791606005c8facba08fe23b96d17c76bd24651c8e06657ca9c14ac2ab5ea9`.
  The native facts retain 1 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xd358).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xd364).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xd368).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0xd36c).
  Four-byte load: target `r0 +0x18`, base register `r3`, raw displacement `+0x18` (`ldr` at 0xd370).
  Branch (`bne` at 0xd37c): taken to 0xd3a0 when `? != 0x0`, falls through to 0xd380 when `? == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xd3b4`, base register `pc`, raw displacement `+0x2c`, linked address 0xd3b4, basis `ELF load-bias-relative` (`ldr` at 0xd380).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xd384).
  Four-byte load: target `ELF load bias (unknown) +0xd3b8`, base register `pc`, raw displacement `+0x24`, linked address 0xd3b8, basis `ELF load-bias-relative` (`ldr` at 0xd38c).
  Native warning (`ldr` at 0xd38c): `ELF literal at linked 0xd3b8 has relocation, writable or unmapped bytes; value unknown`.
  Call (`bl` at 0xd390): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, conditional `false`, r0=`0xfffffeca`, r1=`r1`, r2=`0x0`, r3=`unknown`.
  Native warning (`mvn` at 0xd394): `mvn: unsupported effects invalidated`.
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0xd398).
  Four-byte store: target `sp -0x1c`, value `0x0`, base register `r11`, raw displacement `-0x18` (`str` at 0xd3a4).
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0xd3a8).
  At the known-edge return (`ldm` at 0xd3b0), r0 is `unknown`.
  Known return boundary (`ldm` at 0xd3b0): the modeled local edges end before the retained trailing region 0xd3b4 to 0xd3bc (exclusive). This does not prove complete code-only function extent.
  Undecoded range: 0xd3b4 to 0xd3b8 (exclusive), 4 bytes `cafeffff`. Native warning: `undecodable bytes at 0xd3b4-0xd3b8; fall-through stops at this range`. This raw region remains in the full selected span and supplies no decoded effects.
  The decoded records from 0xd3b8 to 0xd3b8 are outside the modeled known local edges from the entry. Their effects are not listed, without establishing runtime or global nonexecution.

- `RouterPing`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libRouter.armlinux.so`.
  Inferred analysis span: 0x939c to 0x9408 (exclusive), 108 bytes. Grade: inferred. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 72332 bytes, SHA256 `64e901d4e78f6aefaa60d02d569c7611a28e1c77b2a93fb156d4efd0f0780e04`.
  Read window: SHA256 `3c8586bc4708509d5b0ba7ef845dded20c30c07556f780da4c147f8907b0f437`.
  The native facts retain 0 unapplied ELF relocation records for this span. Relocation-dependent values remain unknown.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x93a0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x93ac).
  Four-byte store: target `sp -0x2c`, value `0x0`, base register `sp`, raw displacement `+0x0` (`str` at 0x93b4).
  Four-byte store: target `sp -0x28`, value `0x0`, base register `sp`, raw displacement `+0x4` (`str` at 0x93bc).
  Four-byte store: target `sp -0x24`, value `0x0`, base register `sp`, raw displacement `+0x8` (`str` at 0x93c4).
  Four-byte store: target `sp -0x20`, value `0x0`, base register `sp`, raw displacement `+0xc` (`str` at 0x93cc).
  Four-byte store: target `sp -0x1c`, value `0x0`, base register `sp`, raw displacement `+0x10` (`str` at 0x93d4).
  Four-byte store: target `sp -0x18`, value `0xff`, base register `sp`, raw displacement `+0x14` (`str` at 0x93dc).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x93e0, `ldr` at 0x93f4.
  Call (`bl` at 0x93f0): (unknown callee), target 0x3484, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3484, conditional `false`, r0=`r0`, r1=`0x0`, r2=`0x24`, r3=`0x0`.
  Call (`bl` at 0x93fc): (unknown callee), target 0x33c4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33c4, conditional `false`, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  `ldm` at 0x9404 reports r0 = `ret(linked ELF 0x33c4)` on the modeled known edges.

## Where the `RouterInput` indirect call goes

Read in `/stand/sysco/lib/libRouter.armlinux.so` of the same package (1.44.0, ARM, linked addresses, relocation records applied by reading the library's `.rel.dyn`). Established unless marked.

- The call at `0x9f88` (`mov lr, pc` / `mov pc, r3`) jumps to a function pointer held at offset `0xf8` of the router client object that `RouterInput` is working on (`0x9f7c`). The call passes only the object itself in `r0`. Just before it, the routine increments a counter at offset `0x128` of the same object (`0x9f64`-`0x9f74`). A nonzero return makes the routine leave through its exit path (`0x9f98`); zero continues with the next step.
- The field at `0xf8` has three writers in the whole library. `RouterMakeClient` (`0x9534`, 500 bytes) stores its eighth argument there (`0x95ec`; the first stack argument after four register arguments is at `[fp+4]` in this library's frame, so `[fp+0x10]` is the eighth), so a client of the library chooses the handler when it creates the client object. `RouterSetGroups` (`0xe608`) and `RouterFindServer` (`0xf7a4`) each store the address of the library's own `RouterProcessPacket` (`0xa064`, 492 bytes), reached through a data relocation at `0xe678` and `0xf7d0` (a plain absolute relocation against the dynamic symbol), after sending their request. So the handler called for incoming packets is the client-supplied one until one of those two requests is made, and `RouterProcessPacket` afterwards (inferred as the library's default handler by its name and by its use as the reset value).
- No program or library of this package imports or calls `RouterMakeClient`: the string appears only in `libRouter` itself, and the library has no internal call to `0x9534` (searched the linear disassembly of the library and the dynamic-symbol strings of `l1` and the other five libraries). So in package 4 of 1.44.0 the only writers of the field that can run are `RouterSetGroups` and `RouterFindServer`, and the handler `RouterInput` calls is `RouterProcessPacket` once either has run. The value of the field before that depends on how the client object is allocated and cleared, which was not read; a null field would fault the call. Test to settle it: find the code that allocates the client object and see whether it clears offset `0xf8` (the library's own allocation routines are the candidates).
