# ARM library NVRAM and router paths

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

Groups count static access records with identical native semantic fields: access kind,
width, base, raw displacement, reported value, implicit status, condition and any
resolved-origin or linked-coordinate metadata. They do not imply one path through
different branch arms or equal values across executions. Raw base registers identify
accesses whose effective address is unknown. Their raw displacements are not promoted to
resolved offsets.

Undecodable bytes alone do not establish whether a local path reaches them. An empty
unresolved-transfer list also does not settle that question. Entries with trailing
ranges record the explicit unconditional return boundary and separately retain decoded
unreached runs and undecoded ranges. Calls remain opaque. Local nonreachability does not
establish runtime feasibility or global nonexecution.

The full spans of `nvramSet` and `nvramGet` remain excluded because their reached
indexed PC transfers have unresolved destinations. Their unknown control flow is not
repaired by visible default-path returns.

The [complete facts](disasm/arm-1.44.0-libraries/nvram-router-paths-facts.txt) retain every access, relocation, constant and warning, including omitted paired frame saves and restores. All initial spills, unmatched frame-like records, reloaded input, local return-value and outgoing accesses are retained.

- `nvramInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x108c4 to 0x10c80 (exclusive), 956 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `1838685a68e76e2de88694dda10ebbd371a3207c974455964f0b473d65a74ab1`.
  The 15 unapplied ELF relocation records at linked places 0x10c44, 0x10c48, 0x10c4c, 0x10c50, 0x10c54, 0x10c6c, 0x10c70, 0x10c74, 0x10c78, 0x10c7c, 0x10c40, 0x10c58, 0x10c5c, 0x10c64, 0x10c68 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x108c8).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x108d4).
  Four-byte store: target `sp -0x1c`, value `0x0`, base register `r11`, raw displacement `-0x18` (`str` at 0x108dc).
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0x108e4).
  Four-byte store: target `sp -0x24`, value `0x0`, base register `r11`, raw displacement `-0x20` (`str` at 0x108ec).
  Grouped accesses: 3 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x108f0, `ldr` at 0x10bbc, `ldr` at 0x10bd8.
  Branch (`beq` at 0x108f8): taken to 0x1094c when `r0 == 0x1`, falls through to 0x108fc when `r0 != 0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x10c40`, base register `pc`, raw displacement `+0x33c`, linked address 0x10c40, basis `ELF load-bias-relative` (`ldr` at 0x108fc).
  ELF literal warning: linked 0x10c40, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x108fc, `ldr` at 0x10934, `ldr` at 0x10938.
  Grouped accesses: 23 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x10900, `ldr` at 0x1093c, `ldr` at 0x10950.
  Branch (`beq` at 0x10908): taken to 0x10934 when `? == 0x0`, falls through to 0x1090c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x10c44`, base register `pc`, raw displacement `+0x330`, linked address 0x10c44, basis `ELF load-bias-relative` (`ldr` at 0x1090c).
  ELF literal warning: linked 0x10c44, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x1090c.
  Four-byte store: target `sp -0x30`, value `unknown`, base register `sp`, raw displacement `+0x0` (`str` at 0x10910).
  Four-byte load: target `ELF load bias (unknown) +0x10c48`, base register `pc`, raw displacement `+0x328`, linked address 0x10c48, basis `ELF load-bias-relative` (`ldr` at 0x10918).
  ELF literal warning: linked 0x10c48, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10918.
  Call (`bl` at 0x10924): (unknown callee), target 0x3550, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3550, r0=`0x4`, r1=`unknown`, r2=`0x53`, r3=`0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x28`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x24`, including `str` at 0x1092c, `str` at 0x10960.
  Jump (`b` at 0x10930): target 0x10c34.
  Four-byte load: target `ELF load bias (unknown) +0x10c40`, base register `pc`, raw displacement `+0x304`, linked address 0x10c40, basis `ELF load-bias-relative` (`ldr` at 0x10934).
  Four-byte load: target `ELF load bias (unknown) +0x10c40`, base register `pc`, raw displacement `+0x300`, linked address 0x10c40, basis `ELF load-bias-relative` (`ldr` at 0x10938).
  Grouped accesses: 4 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r2`, raw displacement `+0x0`, including `str` at 0x10944, `str` at 0x109a0, `str` at 0x10a6c.
  Jump (`b` at 0x10948): target 0x10968.
  Four-byte load: target `ELF load bias (unknown) +0x10c4c`, base register `pc`, raw displacement `+0x2f8`, linked address 0x10c4c, basis `ELF load-bias-relative` (`ldr` at 0x1094c).
  ELF literal warning: linked 0x10c4c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x1094c, `ldr` at 0x10bd4.
  Branch (`beq` at 0x10958): taken to 0x10968 when `? == 0x0`, falls through to 0x1095c when `? != 0x0`.
  Jump (`b` at 0x10964): target 0x10c34.
  Four-byte load: target `ELF load bias (unknown) +0x10c50`, base register `pc`, raw displacement `+0x2e0`, linked address 0x10c50, basis `ELF load-bias-relative` (`ldr` at 0x10968).
  ELF literal warning: linked 0x10c50, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x10968, `ldr` at 0x1099c, `ldr` at 0x10a20.
  Call (`bl` at 0x10970): (unknown callee), target 0x11ea4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x11ea4, r0=`unknown`, r1=`0x4000`, r2=`unknown`, r3=`unknown`.
  Branch (`bge` at 0x1097c): taken to 0x1098c when `ret(linked ELF 0x11ea4) >= 0x0 (signed, 32-bit)`, falls through to 0x10980 when `ret(linked ELF 0x11ea4) < 0x0 (signed, 32-bit)`.
  The analyzer invalidates unsupported effects (`mvn` at 0x10980).
  Four-byte store: target `sp -0x28`, value `unknown`, base register `r11`, raw displacement `-0x24` (`str` at 0x10984).
  Jump (`b` at 0x10988): target 0x10c34.
  Four-byte load: target `ELF load bias (unknown) +0x10c54`, base register `pc`, raw displacement `+0x2c0`, linked address 0x10c54, basis `ELF load-bias-relative` (`ldr` at 0x1098c).
  ELF literal warning: linked 0x10c54, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x1098c, `ldr` at 0x10bb0.
  Four-byte store: target `(unknown effective address)`, value `0x1`, base register `r2`, raw displacement `+0x0` (`str` at 0x10994).
  Four-byte load: target `ELF load bias (unknown) +0x10c58`, base register `pc`, raw displacement `+0x2b8`, linked address 0x10c58, basis `ELF load-bias-relative` (`ldr` at 0x10998).
  ELF literal warning: linked 0x10c58, value unknown (relocation, writable or unmapped bytes), 5 read records, including `ldr` at 0x10998, `ldr` at 0x109a4, `ldr` at 0x109d0.
  Four-byte load: target `ELF load bias (unknown) +0x10c50`, base register `pc`, raw displacement `+0x2ac`, linked address 0x10c50, basis `ELF load-bias-relative` (`ldr` at 0x1099c).
  Four-byte load: target `ELF load bias (unknown) +0x10c58`, base register `pc`, raw displacement `+0x2ac`, linked address 0x10c58, basis `ELF load-bias-relative` (`ldr` at 0x109a4).
  Call (`bl` at 0x109b0): (unknown callee), target 0x3364, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3364, r0=`unknown`, r1=`0x8`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x109c0): taken to 0x109d0 when `? == 0x0`, falls through to 0x109c4 when `? != 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x1c`, reported value `0x1` in every record, base register `r11`, raw displacement `-0x18`, including `str` at 0x109c8, `str` at 0x10a04.
  Call (`bl` at 0x109cc): (unknown callee), target 0x10d5c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x10d5c, r0=`ret(linked ELF 0x3364)`, r1=`unknown`, r2=`unknown`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x10c58`, base register `pc`, raw displacement `+0x280`, linked address 0x10c58, basis `ELF load-bias-relative` (`ldr` at 0x109d0).
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0x109d8).
  Branch (`beq` at 0x109e8): taken to 0x10a00 when `? == 0x0`, falls through to 0x109ec when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x10c58`, base register `pc`, raw displacement `+0x264`, linked address 0x10c58, basis `ELF load-bias-relative` (`ldr` at 0x109ec).
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x1`, including `ldrb` at 0x109f4, `ldrb` at 0x10a18.
  Branch (`bne` at 0x109fc): taken to 0x10a0c when `? != 0x0`, falls through to 0x10a00 when `? == 0x0`.
  Call (`bl` at 0x10a08): (unknown callee), target 0x10d5c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x10d5c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x248`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10a0c).
  ELF literal warning: linked 0x10c5c, value unknown (relocation, writable or unmapped bytes), 15 read records, including `ldr` at 0x10a0c, `ldr` at 0x10a2c, `ldr` at 0x10a5c.
  Four-byte load: target `ELF load bias (unknown) +0x10c58`, base register `pc`, raw displacement `+0x240`, linked address 0x10c58, basis `ELF load-bias-relative` (`ldr` at 0x10a10).
  Four-byte load: target `ELF load bias (unknown) +0x10c50`, base register `pc`, raw displacement `+0x228`, linked address 0x10c50, basis `ELF load-bias-relative` (`ldr` at 0x10a20).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r1`, raw displacement `+0x0` (`str` at 0x10a28).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x228`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10a2c).
  Four-byte load: target `ELF load bias (unknown) +0x10c60`, base register `pc`, raw displacement `+0x224`, linked address 0x10c60, basis `ELF load-bias-relative` (`ldr` at 0x10a34).
  Call (`bl` at 0x10a38): (unknown callee), target 0x3364, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3364, r0=`unknown`, r1=`0x82c`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x10a48): taken to 0x10a58 when `? == 0x0`, falls through to 0x10a4c when `? != 0x0`.
  Four-byte store: target `sp -0x20`, value `0x1`, base register `r11`, raw displacement `-0x1c` (`str` at 0x10a50).
  Call (`bl` at 0x10a54): (unknown callee), target 0x10cc0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x10cc0, r0=`ret(linked ELF 0x3364)`, r1=`unknown`, r2=`unknown`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x10c64`, base register `pc`, raw displacement `+0x204`, linked address 0x10c64, basis `ELF load-bias-relative` (`ldr` at 0x10a58).
  ELF literal warning: linked 0x10c64, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x10a58, `ldr` at 0x10a84, `ldr` at 0x10b48.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x1f8`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10a5c).
  Four-byte load: target `ELF load bias (unknown) +0x10c68`, base register `pc`, raw displacement `+0x1f0`, linked address 0x10c68, basis `ELF load-bias-relative` (`ldr` at 0x10a70).
  ELF literal warning: linked 0x10c68, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x10a70, `ldr` at 0x10aa0, `ldr` at 0x10b5c.
  Four-byte load: target `ELF load bias (unknown) +0x10c6c`, base register `pc`, raw displacement `+0x1f0`, linked address 0x10c6c, basis `ELF load-bias-relative` (`ldr` at 0x10a74).
  ELF literal warning: linked 0x10c6c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10a74.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x1d8`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10a7c).
  Four-byte load: target `ELF load bias (unknown) +0x10c64`, base register `pc`, raw displacement `+0x1d8`, linked address 0x10c64, basis `ELF load-bias-relative` (`ldr` at 0x10a84).
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x818`, including `ldr` at 0x10a88, `ldr` at 0x10ac8, `ldr` at 0x10b08.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, including `ldr` at 0x10a8c, `ldr` at 0x10aa8.
  Branch (`bne` at 0x10a94): taken to 0x10b38 when `? != ?`, falls through to 0x10a98 when `? == ?`.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x1bc`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10a98).
  Four-byte load: target `ELF load bias (unknown) +0x10c68`, base register `pc`, raw displacement `+0x1c0`, linked address 0x10c68, basis `ELF load-bias-relative` (`ldr` at 0x10aa0).
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x81c`, including `ldr` at 0x10aa4, `ldr` at 0x10ae8, `ldr` at 0x10b28.
  Branch (`bne` at 0x10ab0): taken to 0x10b38 when `? != ?`, falls through to 0x10ab4 when `? == ?`.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x1a0`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10ab4).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x198`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10abc).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x820`, including `ldr` at 0x10ac4, `ldr` at 0x10ae4.
  Branch (`blo` at 0x10ad0): taken to 0x10b38 when `? < ? (unsigned, 32-bit)`, falls through to 0x10ad4 when `? >= ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x180`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10ad4).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x178`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10adc).
  Branch (`bhs` at 0x10af0): taken to 0x10b38 when `? >= ? (unsigned, 32-bit)`, falls through to 0x10af4 when `? < ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x160`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10af4).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x158`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10afc).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x824`, including `ldr` at 0x10b04, `ldr` at 0x10b24.
  Branch (`blo` at 0x10b10): taken to 0x10b38 when `? < ? (unsigned, 32-bit)`, falls through to 0x10b14 when `? >= ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x140`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10b14).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x138`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10b1c).
  Branch (`bhs` at 0x10b30): taken to 0x10b38 when `? >= ? (unsigned, 32-bit)`, falls through to 0x10b34 when `? < ? (unsigned, 32-bit)`.
  Jump (`b` at 0x10b34): target 0x10b6c.
  Four-byte store: target `sp -0x24`, value `0x1`, base register `r11`, raw displacement `-0x20` (`str` at 0x10b3c).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x114`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10b40).
  Four-byte load: target `ELF load bias (unknown) +0x10c64`, base register `pc`, raw displacement `+0x114`, linked address 0x10c64, basis `ELF load-bias-relative` (`ldr` at 0x10b48).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x818` (`str` at 0x10b50).
  Four-byte load: target `ELF load bias (unknown) +0x10c5c`, base register `pc`, raw displacement `+0x100`, linked address 0x10c5c, basis `ELF load-bias-relative` (`ldr` at 0x10b54).
  Four-byte load: target `ELF load bias (unknown) +0x10c68`, base register `pc`, raw displacement `+0x104`, linked address 0x10c68, basis `ELF load-bias-relative` (`ldr` at 0x10b5c).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x81c` (`str` at 0x10b64).
  Call (`bl` at 0x10b68): (unknown callee), target 0x3784, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3784, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x10b6c, `ldr` at 0x10bfc.
  Branch (`beq` at 0x10b74): taken to 0x10b84 when `[sp-28] == 0x0`, falls through to 0x10b78 when `[sp-28] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x10c70`, base register `pc`, raw displacement `+0xf0`, linked address 0x10c70, basis `ELF load-bias-relative` (`ldr` at 0x10b78).
  ELF literal warning: linked 0x10c70, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10b78.
  Call (`bl` at 0x10b7c): (unknown callee), target 0x3574, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3574, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-28]`.
  Jump (`b` at 0x10b80): target 0x10bb0.
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x10b84, `ldr` at 0x10c00.
  Branch (`beq` at 0x10b8c): taken to 0x10b9c when `[sp-32] == 0x0`, falls through to 0x10b90 when `[sp-32] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x10c74`, base register `pc`, raw displacement `+0xdc`, linked address 0x10c74, basis `ELF load-bias-relative` (`ldr` at 0x10b90).
  ELF literal warning: linked 0x10c74, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10b90.
  Call (`bl` at 0x10b94): (unknown callee), target 0x3574, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3574, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]`.
  Jump (`b` at 0x10b98): target 0x10bb0.
  Grouped accesses: 2 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x10b9c, `ldr` at 0x10c08.
  Branch (`beq` at 0x10ba4): taken to 0x10bb0 when `[sp-36] == 0x0`, falls through to 0x10ba8 when `[sp-36] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x10c78`, base register `pc`, raw displacement `+0xc8`, linked address 0x10c78, basis `ELF load-bias-relative` (`ldr` at 0x10ba8).
  ELF literal warning: linked 0x10c78, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10ba8.
  Call (`bl` at 0x10bac): (unknown callee), target 0x3574, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3574, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Four-byte load: target `ELF load bias (unknown) +0x10c54`, base register `pc`, raw displacement `+0x9c`, linked address 0x10c54, basis `ELF load-bias-relative` (`ldr` at 0x10bb0).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r3`, raw displacement `+0x0` (`str` at 0x10bb8).
  Branch (`bne` at 0x10bc4): taken to 0x10bd4 when `[sp-20] != 0x3`, falls through to 0x10bc8 when `[sp-20] == 0x3`.
  Four-byte load: target `ELF load bias (unknown) +0x10c50`, base register `pc`, raw displacement `+0x80`, linked address 0x10c50, basis `ELF load-bias-relative` (`ldr` at 0x10bc8).
  Call (`bl` at 0x10bd0): (unknown callee), target 0x12124, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x12124, r0=`unknown`, r1=`0x4000`, r2=`0x0`, r3=`[sp-20]`.
  Four-byte load: target `ELF load bias (unknown) +0x10c4c`, base register `pc`, raw displacement `+0x70`, linked address 0x10c4c, basis `ELF load-bias-relative` (`ldr` at 0x10bd4).
  Four-byte store: target `(unknown effective address)`, value `[sp-20]`, base register `r2`, raw displacement `+0x0` (`str` at 0x10bdc).
  Call (`bl` at 0x10be4): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`0x11`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Four-byte load: target `ELF load bias (unknown) +0x10c7c`, base register `pc`, raw displacement `+0x88`, linked address 0x10c7c, basis `ELF load-bias-relative` (`ldr` at 0x10bec).
  ELF literal warning: linked 0x10c7c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x10bec.
  Call (`bl` at 0x10bf8): (unknown callee), target 0x349c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x349c, r0=`unknown`, r1=`ret(linked ELF 0x37cc)`, r2=`0x1`, r3=`ret(linked ELF 0x37cc)`.
  Branch (`beq` at 0x10c14): taken to 0x10c24 when `? == 0x0`, falls through to 0x10c18 when `? != 0x0`.
  The analyzer invalidates unsupported effects (`mvn` at 0x10c18).
  Four-byte store: target `sp -0x2c`, value `unknown`, base register `r11`, raw displacement `-0x28` (`str` at 0x10c1c).
  Jump (`b` at 0x10c20): target 0x10c2c.
  Four-byte store: target `sp -0x2c`, value `0x0`, base register `r11`, raw displacement `-0x28` (`str` at 0x10c28).
  Four-byte load: target `sp -0x2c`, base register `r11`, raw displacement `-0x28` (`ldr` at 0x10c2c).
  Four-byte store: target `sp -0x28`, value `[sp-44]`, base register `r11`, raw displacement `-0x24` (`str` at 0x10c30).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x10c34).
  At the return (`ldm` at 0x10c3c), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0x10c3c): the modeled local edges stop before the trailing region 0x10c40 to 0x10c80 (exclusive).
  The decoded instructions from 0x10c40 to 0x10c7c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `nvramLogInsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x11604 to 0x119e8 (exclusive), 996 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `320b6a1a10d28ce03d8a6c730a78f97f1f31f7bd02b55d0a7681d535ecfcd187`.
  The 3 unapplied ELF relocation records at linked places 0x119e0, 0x119e4, 0x119d4 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x14`, implicit access (`push` at 0x11608).
  Four-byte store: target `sp -0x1c`, value `r0`, base register `r11`, raw displacement `-0x18` (`str` at 0x11614).
  Call (`bl` at 0x1161c): (unknown callee), target 0x3598, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3598, r0=`0x0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x3598)`, base register `r11`, raw displacement `-0x20` (`str` at 0x11624).
  Four-byte store: target `sp -0x28`, value `sp-36`, base register `r11`, raw displacement `-0x24` (`str` at 0x1162c).
  Four-byte store: target `sp -0x2c`, value `0x5`, base register `r11`, raw displacement `-0x28` (`str` at 0x11634).
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x11638, `ldr` at 0x11674.
  Call (`bl` at 0x1163c): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`0x5`.
  Four-byte store: target `sp -0x30`, value `ret(linked ELF 0x35d4)`, base register `r11`, raw displacement `-0x2c` (`str` at 0x11644).
  Byte store: target `sp -0x31`, value `0x0`, base register `r11`, raw displacement `-0x2d` (`strb` at 0x1164c).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x37c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11650).
  ELF literal warning: linked 0x119d4, value unknown (relocation, writable or unmapped bytes), 35 read records, including `ldr` at 0x11650, `ldr` at 0x11658, `ldr` at 0x11698.
  Grouped accesses: 36 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x11654, `ldr` at 0x1165c, `ldr` at 0x1169c.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x374`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11658).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x81c`, including `ldr` at 0x11660, `ldr` at 0x116e4.
  Grouped accesses: 6 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x818`, including `ldr` at 0x11664, `ldr` at 0x116cc, `ldr` at 0x117c0.
  The analyzer invalidates unsupported effects (`rsb` at 0x11668).
  Grouped accesses: 3 4-byte store records with target `sp -0x38`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x34`, including `str` at 0x1166c, `str` at 0x116f4, `str` at 0x11738.
  The analyzer invalidates unsupported effects (`mvn` at 0x11670).
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x11678, `ldr` at 0x1173c, `ldr` at 0x1189c.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldrb` at 0x11684, `ldrb` at 0x1176c.
  Branch (`beq` at 0x1168c): taken to 0x11698 when `? == 0xa`, falls through to 0x11690 when `? != 0xa`.
  Byte store: target `sp -0x31`, value `0x1`, base register `r11`, raw displacement `-0x2d` (`strb` at 0x11694).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x334`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11698).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x32c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x116a0).
  Grouped accesses: 5 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x824`, including `ldr` at 0x116a8, `ldr` at 0x1170c, `ldr` at 0x11870.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x820`, including `ldr` at 0x116ac, `ldr` at 0x11710.
  Branch (`bls` at 0x116b4): taken to 0x116fc when `? <= ? (unsigned, 32-bit)`, falls through to 0x116b8 when `? > ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x314`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x116b8).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x30c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x116c0).
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x820`, including `ldr` at 0x116c8, `ldr` at 0x1172c, `ldr` at 0x117a0.
  The analyzer invalidates unsupported effects (`rsb` at 0x116d0).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x2f8`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x116d4).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x2f0`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x116dc).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x824`, including `ldr` at 0x116e8, `ldr` at 0x11730.
  The analyzer invalidates unsupported effects (`rsb` at 0x116ec).
  Jump (`b` at 0x116f8): target 0x1173c.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x2d0`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x116fc).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x2c8`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11704).
  Branch (`bhs` at 0x11718): taken to 0x1173c when `? >= ? (unsigned, 32-bit)`, falls through to 0x1171c when `? < ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x2b0`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x1171c).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x2a8`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11724).
  The analyzer invalidates unsupported effects (`rsb` at 0x11734).
  Grouped accesses: 3 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x11740, `ldr` at 0x117dc, `ldr` at 0x117f0.
  Grouped accesses: 2 1-byte load records with target `sp -0x31`, base register `r11`, raw displacement `-0x2d`, including `ldrsb` at 0x11748, `ldrsb` at 0x11924.
  Grouped accesses: 3 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0x11750, `ldr` at 0x11784, `ldr` at 0x117cc.
  Branch (`blt` at 0x11758): taken to 0x117dc when `? < [sp-56] (signed, 32-bit)`, falls through to 0x1175c when `? >= [sp-56] (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x270`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x1175c).
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x0` (`ldr` at 0x11768).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0` (`str` at 0x11774).
  Branch (`beq` at 0x11780): taken to 0x117cc when `? == 0xa`, falls through to 0x11784 when `? != 0xa`.
  Grouped accesses: 2 4-byte store records with target `sp -0x38`, reported value `[sp-56]+1` in every record, base register `r11`, raw displacement `-0x34`, including `str` at 0x1178c, `str` at 0x117d4.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x23c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11790).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x234`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11798).
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x81c`, including `ldr` at 0x117a4, `ldr` at 0x11874, `ldr` at 0x118fc.
  Branch (`blo` at 0x117ac): taken to 0x1175c when `? < ? (unsigned, 32-bit)`, falls through to 0x117b0 when `? >= ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x21c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x117b0).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x214`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x117b8).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x820` (`str` at 0x117c4).
  Jump (`b` at 0x117c8): target 0x1175c.
  Jump (`b` at 0x117d8): target 0x1173c.
  Four-byte store: target `sp -0x2c`, value `[sp-44]-1`, base register `r11`, raw displacement `-0x28` (`str` at 0x117e4).
  The analyzer invalidates unsupported effects (`cmn` at 0x117e8).
  Branch (`beq` at 0x117ec): taken to 0x1189c when `Z == 1`, falls through to 0x117f0 when `not (Z == 1)`.
  Branch (`bne` at 0x117f8): taken to 0x11828 when `[sp-44] != 0x4`, falls through to 0x117fc when `[sp-44] == 0x4`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x1d0`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x117fc).
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, including `ldr` at 0x1180c, `ldr` at 0x11838, `ldr` at 0x118c0.
  The analyzer invalidates unsupported effects (`mvn` at 0x11814).
  Byte store: target `(unknown effective address)`, value `unknown`, base register `r0`, raw displacement `+0x0` (`strb` at 0x11818).
  Grouped accesses: 4 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r1`, raw displacement `+0x0`, including `str` at 0x11820, `str` at 0x1185c, `str` at 0x118e4.
  Jump (`b` at 0x11824): target 0x11860.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x1a4`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11828).
  Four-byte load: target `sp -0x28`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x11840).
  Byte load: target `[sp-40] +0x0`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0x11844).
  Four-byte store: target `sp -0x28`, value `[sp-40]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0x1184c).
  Byte store: target `(unknown effective address)`, value `([[sp-40]]:1 & 0xff)`, base register `r3`, raw displacement `+0x0` (`strb` at 0x11854).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x16c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11860).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x164`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11868).
  Branch (`blo` at 0x1187c): taken to 0x117dc when `? < ? (unsigned, 32-bit)`, falls through to 0x11880 when `? >= ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x14c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11880).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x144`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11888).
  Grouped accesses: 3 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r2`, raw displacement `+0x824`, including `str` at 0x11894, `str` at 0x1191c, `str` at 0x1198c.
  Jump (`b` at 0x11898): target 0x117dc.
  Four-byte store: target `sp -0x30`, value `[sp-48]-1`, base register `r11`, raw displacement `-0x2c` (`str` at 0x118a4).
  The analyzer invalidates unsupported effects (`cmn` at 0x118a8).
  Branch (`beq` at 0x118ac): taken to 0x11924 when `Z == 1`, falls through to 0x118b0 when `not (Z == 1)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x11c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x118b0).
  Four-byte load: target `sp -0x1c`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x118c8).
  Byte load: target `[sp-28] +0x0`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0x118cc).
  Four-byte store: target `sp -0x1c`, value `[sp-28]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0x118d4).
  Byte store: target `(unknown effective address)`, value `([[sp-28]]:1 & 0xff)`, base register `r3`, raw displacement `+0x0` (`strb` at 0x118dc).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0xe4`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x118e8).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0xdc`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x118f0).
  Branch (`blo` at 0x11904): taken to 0x1189c when `? < ? (unsigned, 32-bit)`, falls through to 0x11908 when `? >= ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0xc4`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11908).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0xbc`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11910).
  Jump (`b` at 0x11920): target 0x1189c.
  Branch (`beq` at 0x1192c): taken to 0x11990 when `? == 0x0`, falls through to 0x11930 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x9c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11930).
  Byte store: target `(unknown effective address)`, value `0xa`, base register `r0`, raw displacement `+0x0` (`strb` at 0x1194c).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x74`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11958).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x6c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11960).
  Branch (`blo` at 0x11974): taken to 0x11990 when `? < ? (unsigned, 32-bit)`, falls through to 0x11978 when `? >= ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x54`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11978).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x4c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11980).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x3c`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x11990).
  Four-byte load: target `ELF load bias (unknown) +0x119d8`, base register `pc`, raw displacement `+0x38`, linked address 0x119d8, basis `ELF load-bias-relative` (`ldr` at 0x11998).
  Four-byte load: target `ELF load bias (unknown) +0x119d4`, base register `pc`, raw displacement `+0x30`, linked address 0x119d4, basis `ELF load-bias-relative` (`ldr` at 0x1199c).
  Four-byte load: target `ELF load bias (unknown) +0x119dc`, base register `pc`, raw displacement `+0x30`, linked address 0x119dc, basis `ELF load-bias-relative` (`ldr` at 0x119a4).
  Call (`bl` at 0x119a8): (unknown callee), target 0x33c4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x33c4, r0=`unknown`, r1=`0x82c`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(linked ELF 0x33c4) & 0xff)`, base register `r5`, raw displacement `+0x0` (`strb` at 0x119ac).
  Four-byte load: target `ELF load bias (unknown) +0x119e0`, base register `pc`, raw displacement `+0x28`, linked address 0x119e0, basis `ELF load-bias-relative` (`ldr` at 0x119b0).
  ELF literal warning: linked 0x119e0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x119b0.
  Branch (`bne` at 0x119bc): taken to 0x119cc when `? != 0x3`, falls through to 0x119c0 when `? == 0x3`.
  Four-byte load: target `ELF load bias (unknown) +0x119e4`, base register `pc`, raw displacement `+0x1c`, linked address 0x119e4, basis `ELF load-bias-relative` (`ldr` at 0x119c0).
  ELF literal warning: linked 0x119e4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x119c0.
  Call (`bl` at 0x119c8): (unknown callee), target 0x12124, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x12124, r0=`unknown`, r1=`0x4000`, r2=`unknown`, r3=`unknown`.
  The exit (`ldm` at 0x119d0) reports r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x119d0): the modeled local edges stop before the trailing region 0x119d4 to 0x119e8 (exclusive).
  The decoded instructions from 0x119d4 to 0x119e4 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `nvramLogFetch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libmain.armlinux.so`.
  Inferred analysis span: 0x11a30 to 0x11dd8 (exclusive), 936 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 86208 bytes, SHA256 `6a870b62130b1c7c5db86dce8ce7ddbb54692609860e74fb17840f2adb28e6e2`.
  Read window: SHA256 `3bea9b0570cc5187d94ce6b3c12309b66b4c407d93836f1e4e3777f7b79555a6`.
  The 2 unapplied ELF relocation records at linked places 0x11dd4, 0x11dd0 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x11a34).
  Four-byte store: target `sp -0x44`, value `0x1`, base register `r11`, raw displacement `-0x40` (`str` at 0x11a44).
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x380`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11a48).
  ELF literal warning: linked 0x11dd0, value unknown (relocation, writable or unmapped bytes), 10 read records, including `ldr` at 0x11a48, `ldr` at 0x11a50, `ldr` at 0x11a68.
  Grouped accesses: 10 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x11a4c, `ldr` at 0x11a54, `ldr` at 0x11a6c.
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x378`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11a50).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x824`, including `ldr` at 0x11a58, `ldr` at 0x11aac.
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x820`, including `ldr` at 0x11a5c, `ldr` at 0x11a70, `ldr` at 0x11ab0.
  Branch (`bls` at 0x11a64): taken to 0x11a9c when `? <= ? (unsigned, 32-bit)`, falls through to 0x11a68 when `? > ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x360`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11a68).
  Grouped accesses: 2 4-byte store records with target `sp -0x18`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x14`, including `str` at 0x11a74, `str` at 0x11ac8.
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x350`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11a78).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x824`, including `ldr` at 0x11a80, `ldr` at 0x11af4.
  Grouped accesses: 2 4-byte store records with target `sp -0x1c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x18`, including `str` at 0x11a84, `str` at 0x11ad8.
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0x11a8c).
  Four-byte store: target `sp -0x24`, value `0x0`, base register `r11`, raw displacement `-0x20` (`str` at 0x11a94).
  Jump (`b` at 0x11a98): target 0x11b3c.
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x32c`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11a9c).
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x324`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11aa4).
  Branch (`bls` at 0x11ab8): taken to 0x11b00 when `? <= ? (unsigned, 32-bit)`, falls through to 0x11abc when `? > ? (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x30c`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11abc).
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x2fc`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11acc).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x81c` (`ldr` at 0x11ad4).
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x2ec`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11adc).
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x818` (`ldr` at 0x11ae4).
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0x11ae8).
  Four-byte load: target `ELF load bias (unknown) +0x11dd0`, base register `pc`, raw displacement `+0x2dc`, linked address 0x11dd0, basis `ELF load-bias-relative` (`ldr` at 0x11aec).
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0x11af8).
  Jump (`b` at 0x11afc): target 0x11b3c.
  Call (`bl` at 0x11b04): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, r0=`0x1`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x14`, reported value `ret(linked ELF 0x3514)` in every record, base register `r11`, raw displacement `-0x10`, including `str` at 0x11b08, `str` at 0x11c30.
  Grouped accesses: 9 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x11b0c, `ldr` at 0x11b24, `ldr` at 0x11b30.
  Branch (`bne` at 0x11b14): taken to 0x11b24 when `ret(linked ELF 0x3514) != 0x0`, falls through to 0x11b18 when `ret(linked ELF 0x3514) == 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x48`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x44`, including `str` at 0x11b1c, `str` at 0x11c44.
  Jump (`b` at 0x11b20): target 0x11dc4.
  Byte store: target `ret(linked ELF 0x3514) +0x0`, value `0x0`, base register `r2`, raw displacement `+0x0` (`strb` at 0x11b2c).
  Grouped accesses: 2 4-byte store records with target `sp -0x48`, reported value `[sp-20]` in every record, base register `r11`, raw displacement `-0x44`, including `str` at 0x11b34, `str` at 0x11dc0.
  Jump (`b` at 0x11b38): target 0x11dc4.
  Grouped accesses: 2 4-byte store records with target `sp -0x34`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x30`, including `str` at 0x11b40, `str` at 0x11c50.
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x11b44, `ldr` at 0x11c5c.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `[sp-24]` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x11b48, `str` at 0x11c60.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x11b4c, `ldr` at 0x11c64.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[sp-28]` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x11b50, `str` at 0x11c68.
  Grouped accesses: 4 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x11b54, `ldr` at 0x11b60, `ldr` at 0x11c6c.
  Branch (`beq` at 0x11b5c): taken to 0x11bfc when `Z == 1`, falls through to 0x11b60 when `not (Z == 1)`.
  Grouped accesses: 4 4-byte store records with target `sp -0x28`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x24`, including `str` at 0x11b64, `str` at 0x11bd0, `str` at 0x11c7c.
  Grouped accesses: 8 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0x11b68, `ldr` at 0x11b94, `ldr` at 0x11bc8.
  Grouped accesses: 2 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x11b6c, `ldr` at 0x11c84.
  Branch (`bhs` at 0x11b74): taken to 0x11bd8 when `C == 1`, falls through to 0x11b78 when `not (C == 1)`.
  Grouped accesses: 4 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x11b78, `ldr` at 0x11b84, `ldr` at 0x11c90.
  Branch (`beq` at 0x11b80): taken to 0x11b94 when `Z == 1`, falls through to 0x11b84 when `not (Z == 1)`.
  Four-byte store: target `sp -0x34`, value `unknown`, base register `r11`, raw displacement `-0x30` (`str` at 0x11b8c).
  Jump (`b` at 0x11b90): target 0x11bc8.
  Grouped accesses: 4 1-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldrb` at 0x11b98, `ldrb` at 0x11cac, `ldrb` at 0x11d2c.
  Branch (`bne` at 0x11ba0): taken to 0x11bbc when `Z == 0`, falls through to 0x11ba4 when `not (Z == 0)`.
  Grouped accesses: 8 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0x11ba4, `ldr` at 0x11bbc, `ldr` at 0x11bfc.
  Grouped accesses: 2 4-byte store records with target `sp -0x44`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x40`, including `str` at 0x11bac, `str` at 0x11bc4.
  Grouped accesses: 2 4-byte store records with target `sp -0x34`, reported value `0x4` in every record, base register `r11`, raw displacement `-0x30`, including `str` at 0x11bb4, `str` at 0x11d44.
  Jump (`b` at 0x11bb8): target 0x11bc8.
  Jump (`b` at 0x11bd4): target 0x11b68.
  Grouped accesses: 4 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x11bd8, `ldr` at 0x11be8, `ldr` at 0x11d84.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x11bdc, `str` at 0x11bec.
  Grouped accesses: 4 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x11be0, `ldr` at 0x11bf0, `ldr` at 0x11d8c.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x11be4, `str` at 0x11bf4.
  Jump (`b` at 0x11bf8): target 0x11b54.
  Branch (`ble` at 0x11c04): taken to 0x11c18 when `le flags condition (operands unknown)`, falls through to 0x11c08 when `not (le flags condition (operands unknown))`.
  Four-byte store: target `sp -0x4c`, value `unknown`, base register `r11`, raw displacement `-0x48` (`str` at 0x11c10).
  Jump (`b` at 0x11c14): target 0x11c20.
  Four-byte store: target `sp -0x4c`, value `0x64`, base register `r11`, raw displacement `-0x48` (`str` at 0x11c1c).
  Four-byte load: target `sp -0x4c`, base register `r11`, raw displacement `-0x48` (`ldr` at 0x11c20).
  Four-byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`str` at 0x11c24).
  Four-byte load: target `sp -0x40`, base register `r11`, raw displacement `-0x3c` (`ldr` at 0x11c28).
  Call (`bl` at 0x11c2c): (unknown callee), target 0x3514, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3514, r0=`unknown`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x11c3c): taken to 0x11c4c when `ret(linked ELF 0x3514) != 0x0`, falls through to 0x11c40 when `ret(linked ELF 0x3514) == 0x0`.
  Jump (`b` at 0x11c48): target 0x11dc4.
  Four-byte store: target `sp -0x44`, value `0x0`, base register `r11`, raw displacement `-0x40` (`str` at 0x11c58).
  Branch (`beq` at 0x11c74): taken to 0x11da8 when `Z == 1`, falls through to 0x11c78 when `not (Z == 1)`.
  Branch (`bhs` at 0x11c8c): taken to 0x11d84 when `C == 1`, falls through to 0x11c90 when `not (C == 1)`.
  Branch (`beq` at 0x11c98): taken to 0x11d28 when `Z == 1`, falls through to 0x11c9c when `not (Z == 1)`.
  Four-byte load: target `sp -0x3c`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x11ca0).
  Byte store: target `[sp-60] +0x0`, value `unknown`, base register `r1`, raw displacement `+0x0` (`strb` at 0x11cb0).
  Four-byte store: target `sp -0x3c`, value `[sp-60]+1`, base register `r0`, raw displacement `+0x0` (`str` at 0x11cb8).
  Four-byte store: target `sp -0x34`, value `[sp-52]-1`, base register `r11`, raw displacement `-0x30` (`str` at 0x11cc4).
  Branch (`bne` at 0x11ccc): taken to 0x11d74 when `[sp-52]-1 != 0x0`, falls through to 0x11cd0 when `[sp-52]-1 == 0x0`.
  Call (`bl` at 0x11cd8): (unknown callee), target 0x340c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x340c, r0=`sp-56`, r1=`[sp-60]`, r2=`[sp-60]+1`, r3=`sp-56`.
  Four-byte load: target `ELF load bias (unknown) +0x11dd4`, base register `pc`, raw displacement `+0xd8`, linked address 0x11dd4, basis `ELF load-bias-relative` (`ldr` at 0x11cf4).
  ELF literal warning: linked 0x11dd4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x11cf4.
  Call (`bl` at 0x11cfc): (unknown callee), target 0x3520, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3520, r0=`([sp-20] + [sp-68])`, r1=`0x13`, r2=`unknown`, r3=`ret(linked ELF 0x340c)`.
  Call (`bl` at 0x11d10): (unknown callee), target 0x35d4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x35d4, r0=`([sp-20] + [sp-68])`, r1=`unknown`, r2=`[sp-20]`, r3=`([sp-20] + [sp-68])`.
  Four-byte store: target `sp -0x44`, value `([sp-68] + ret(linked ELF 0x35d4))`, base register `r11`, raw displacement `-0x40` (`str` at 0x11d20).
  Jump (`b` at 0x11d24): target 0x11d74.
  Branch (`bne` at 0x11d34): taken to 0x11d4c when `Z == 0`, falls through to 0x11d38 when `not (Z == 0)`.
  Four-byte store: target `sp -0x3c`, value `sp-56`, base register `r11`, raw displacement `-0x38` (`str` at 0x11d3c).
  Jump (`b` at 0x11d48): target 0x11d74.
  Four-byte load: target `sp -0x44`, base register `r0`, raw displacement `+0x0` (`ldr` at 0x11d50).
  Byte store: target `(unknown effective address)`, value `unknown`, base register `r1`, raw displacement `+0x0` (`strb` at 0x11d68).
  Four-byte store: target `sp -0x44`, value `unknown`, base register `r0`, raw displacement `+0x0` (`str` at 0x11d70).
  Jump (`b` at 0x11d80): target 0x11c80.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `[sp-32]` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x11d88, `str` at 0x11d98.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[sp-36]` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x11d90, `str` at 0x11da0.
  Jump (`b` at 0x11da4): target 0x11c6c.
  Byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x0` (`strb` at 0x11db8).
  Four-byte load: target `sp -0x48`, base register `r11`, raw displacement `-0x44` (`ldr` at 0x11dc4).
  Recorded r0 at the return is `unknown` (`ldm` at 0x11dcc).
  Unconditional return boundary (`ldm` at 0x11dcc): the modeled local edges stop before the trailing region 0x11dd0 to 0x11dd8 (exclusive).
  The decoded instructions from 0x11dd0 to 0x11dd4 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterForwardPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x43a4 to 0x468c (exclusive), 744 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `b1ade82738b795f84a1bd4d35f75e83a451001de99bc64aa21c88126eae99e7b`.
  The 2 unapplied ELF relocation records at linked places 0x4684, 0x4688 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x43a8).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x43b4).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0x43b8).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0x43bc).
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0x43c4).
  Grouped accesses: 6 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x43c8, `ldr` at 0x43d8, `ldr` at 0x445c.
  Byte load: target `r2 +0x16`, base register `r3`, raw displacement `+0x16` (`ldrb` at 0x43cc).
  Branch (`bne` at 0x43d4): taken to 0x4454 when `[r2+22]:1 != 0x2`, falls through to 0x43d8 when `[r2+22]:1 == 0x2`.
  Four-byte load: target `r2 +0x1c`, base register `r3`, raw displacement `+0x1c` (`ldr` at 0x43dc).
  Four-byte store: target `sp -0x38`, value `[r2+28]`, base register `r11`, raw displacement `-0x34` (`str` at 0x43e0).
  Grouped accesses: 6 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0x43e4, `ldr` at 0x43f0, `ldr` at 0x4404.
  Branch (`blt` at 0x43ec): taken to 0x4448 when `[r2+28] < 0x0 (signed, 32-bit)`, falls through to 0x43f0 when `[r2+28] >= 0x0 (signed, 32-bit)`.
  Branch (`bne` at 0x4400): taken to 0x4448 when `? != 0x0`, falls through to 0x4404 when `? == 0x0`.
  Branch (`bge` at 0x440c): taken to 0x4428 when `[r2+28] >= 0x0 (signed, 32-bit)`, falls through to 0x4410 when `[r2+28] < 0x0 (signed, 32-bit)`.
  Branch (`beq` at 0x4420): taken to 0x443c when `? == 0x5`, falls through to 0x4424 when `? != 0x5`.
  Jump (`b` at 0x4424): target 0x4448.
  Branch (`beq` at 0x4434): taken to 0x443c when `? == 0x5`, falls through to 0x4438 when `? != 0x5`.
  Jump (`b` at 0x4438): target 0x4448.
  Byte store: target `sp -0x31`, value `0xa`, base register `r11`, raw displacement `-0x2d` (`strb` at 0x4440).
  Jump (`b` at 0x4444): target 0x445c.
  Byte store: target `sp -0x31`, value `0xb`, base register `r11`, raw displacement `-0x2d` (`strb` at 0x444c).
  Jump (`b` at 0x4450): target 0x445c.
  Byte store: target `sp -0x31`, value `0x14`, base register `r11`, raw displacement `-0x2d` (`strb` at 0x4458).
  Byte load: target `r2 +0x15`, base register `r3`, raw displacement `+0x15` (`ldrb` at 0x4460).
  Four-byte store: target `sp -0x28`, value `unknown`, base register `r11`, raw displacement `-0x24` (`str` at 0x446c).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x4470).
  Call (`bl` at 0x447c): (unknown callee), target 0x1d68, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d68, r0=`unknown`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x1d68)`, base register `r11`, raw displacement `-0x20` (`str` at 0x4480).
  Grouped accesses: 11 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x4484, `ldr` at 0x449c, `ldr` at 0x44c0.
  Branch (`bne` at 0x448c): taken to 0x449c when `ret(linked ELF 0x1d68) != 0x0`, falls through to 0x4490 when `ret(linked ELF 0x1d68) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4680`, base register `pc`, raw displacement `+0x1e8`, linked address 0x4680, basis `ELF load-bias-relative` (`ldr` at 0x4490).
  Four-byte store: target `sp -0x3c`, value `0xfffffda4`, base register `r11`, raw displacement `-0x38` (`str` at 0x4494).
  Jump (`b` at 0x4498): target 0x4674.
  Byte load: target `ret(linked ELF 0x1d68) +0x0`, base register `r2`, raw displacement `+0x0` (`ldrb` at 0x44a0).
  Byte store: target `sp -0x40`, value `([ret(linked ELF 0x1d68)]:1 & 0xff)`, base register `r11`, raw displacement `-0x3c` (`strb` at 0x44a4).
  Grouped accesses: 2 1-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldrb` at 0x44a8, `ldrb` at 0x44b8.
  Byte store: target `sp -0x40`, value `unknown`, base register `r11`, raw displacement `-0x3c` (`strb` at 0x44b4).
  Byte store: target `ret(linked ELF 0x1d68) +0x0`, value `([sp-64]:1 & 0xff)`, base register `r2`, raw displacement `+0x0` (`strb` at 0x44bc).
  Four-byte load: target `ELF load bias (unknown) +0x4684`, base register `pc`, raw displacement `+0x1b8`, linked address 0x4684, basis `ELF load-bias-relative` (`ldr` at 0x44c4).
  ELF literal warning: linked 0x4684, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x44c4.
  Four-byte store: target `[sp-36] +0x4`, value `unknown`, base register `r2`, raw displacement `+0x4` (`str` at 0x44c8).
  Byte load: target `[sp-36] +0x0`, base register `r2`, raw displacement `+0x0` (`ldrb` at 0x44d0).
  Byte store: target `[sp-36] +0x0`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0x44d8).
  Four-byte load: target `[sp-36] +0x0`, base register `r2`, raw displacement `+0x0` (`ldr` at 0x44e0).
  Four-byte store: target `sp -0x44`, value `[[sp-36]]`, base register `r11`, raw displacement `-0x40` (`str` at 0x44e4).
  Grouped accesses: 2 4-byte load records with target `sp -0x44`, base register `r11`, raw displacement `-0x40`, including `ldr` at 0x44e8, `ldr` at 0x44fc.
  Four-byte store: target `sp -0x44`, value `unknown`, base register `r11`, raw displacement `-0x40` (`str` at 0x44f8).
  Four-byte store: target `[sp-36] +0x0`, value `[sp-68]`, base register `r2`, raw displacement `+0x0` (`str` at 0x4500).
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[sp-28]` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x4508, `str` at 0x4620.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `0x1` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x4510, `str` at 0x4628.
  Grouped accesses: 8 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x4514, `ldr` at 0x4520, `ldr` at 0x452c.
  Branch (`beq` at 0x451c): taken to 0x45f8 when `Z == 1`, falls through to 0x4520 when `not (Z == 1)`.
  Two-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x10` (`ldrh` at 0x4524).
  Four-byte store: target `sp -0x38`, value `unknown`, base register `r11`, raw displacement `-0x34` (`str` at 0x4528).
  Call (`bl` at 0x4534): (unknown callee), target 0x1cd8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cd8, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 7 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x4538, `ldr` at 0x4568, `ldr` at 0x4580.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, including `ldrb` at 0x4548, `ldrb` at 0x4590.
  Grouped accesses: 2 1-byte store records with target `sp -0x48`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x44`, including `strb` at 0x454c, `strb` at 0x455c.
  Grouped accesses: 2 1-byte load records with target `sp -0x48`, base register `r11`, raw displacement `-0x44`, including `ldrb` at 0x4550, `ldrb` at 0x4560.
  Byte store: target `(unknown effective address)`, value `([sp-72]:1 & 0xff)`, base register `r2`, raw displacement `+0x0` (`strb` at 0x4564).
  Four-byte store: target `(unknown effective address)`, value `[sp-48]`, base register `r2`, raw displacement `+0x4` (`str` at 0x457c).
  Byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0` (`strb` at 0x4598).
  Four-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x0` (`ldr` at 0x45b4).
  Grouped accesses: 2 4-byte store records with target `sp -0x4c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x48`, including `str` at 0x45b8, `str` at 0x45d0.
  Grouped accesses: 2 4-byte load records with target `sp -0x4c`, base register `r11`, raw displacement `-0x48`, including `ldr` at 0x45c0, `ldr` at 0x45d4.
  Four-byte store: target `(unknown effective address)`, value `[sp-76]`, base register `r1`, raw displacement `+0x0` (`str` at 0x45d8).
  Grouped accesses: 2 4-byte load records with target `[sp-48] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x45e0, `ldr` at 0x464c.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[[sp-48]]` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x45e4, `str` at 0x4650.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `[sp-44]+1` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x45f0, `str` at 0x465c.
  Jump (`b` at 0x45f4): target 0x4514.
  Byte load: target `sp -0x31`, base register `r11`, raw displacement `-0x2d` (`ldrb` at 0x45f8).
  Four-byte store: target `sp -0x50`, value `[sp-49]:1`, base register `sp`, raw displacement `+0x0` (`str` at 0x45fc).
  Four-byte load: target `sp -0x14`, base register `r11`, raw displacement `-0x10` (`ldr` at 0x4600).
  Call (`bl` at 0x4610): (unknown callee), target 0x1e88, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e88, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0x4618): (unknown callee), target 0x1e4c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e4c, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x4634): taken to 0x4664 when `Z == 1`, falls through to 0x4638 when `not (Z == 1)`.
  Four-byte load: target `ELF load bias (unknown) +0x4688`, base register `pc`, raw displacement `+0x48`, linked address 0x4688, basis `ELF load-bias-relative` (`ldr` at 0x4638).
  ELF literal warning: linked 0x4688, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4638.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x4640).
  Call (`bl` at 0x4644): (unknown callee), target 0x1cd8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cd8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0x4660): target 0x462c.
  Call (`bl` at 0x4668): (unknown callee), target 0x1c90, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c90, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0x466c).
  Four-byte store: target `sp -0x3c`, value `[sp-32]`, base register `r11`, raw displacement `-0x38` (`str` at 0x4670).
  Four-byte load: target `sp -0x3c`, base register `r11`, raw displacement `-0x38` (`ldr` at 0x4674).
  `ldm` at 0x467c has recorded r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x467c): the modeled local edges stop before the trailing region 0x4680 to 0x468c (exclusive).
  Undecoded range: 0x4680 to 0x4684 (exclusive), 4 bytes `a4fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x4684 to 0x4688 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `pkt_timeout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x5864 to 0x58b4 (exclusive), 80 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `070a0eb61ea3c25ffa44762e85f96c8999c265e4c960e9fba462b318d011ddb2`.
  The 3 unapplied ELF relocation records at linked places 0x58ac, 0x58a8, 0x58a4 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x5868).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x5874).
  Four-byte load: target `ELF load bias (unknown) +0x58a4`, base register `pc`, raw displacement `+0x24`, linked address 0x58a4, basis `ELF load-bias-relative` (`ldr` at 0x5878).
  ELF literal warning: linked 0x58a4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5878.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x0`, including `str` at 0x5880, `str` at 0x588c.
  Four-byte load: target `ELF load bias (unknown) +0x58a8`, base register `pc`, raw displacement `+0x1c`, linked address 0x58a8, basis `ELF load-bias-relative` (`ldr` at 0x5884).
  ELF literal warning: linked 0x58a8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5884.
  Four-byte load: target `ELF load bias (unknown) +0x58ac`, base register `pc`, raw displacement `+0x14`, linked address 0x58ac, basis `ELF load-bias-relative` (`ldr` at 0x5890).
  ELF literal warning: linked 0x58ac, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x5890.
  Four-byte load: target `ELF load bias (unknown) +0x58b0`, base register `pc`, raw displacement `+0x14`, linked address 0x58b0, basis `ELF load-bias-relative` (`ldr` at 0x5894).
  Four-byte store: target `(unknown effective address)`, value `0xfffffd94`, base register `r2`, raw displacement `+0x0` (`str` at 0x5898).
  Call (`bl` at 0x589c): (unknown callee), target 0x1c54, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c54, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`0xfffffd94`.
  Four-byte load: target `sp -0x14`, base register `sp`, raw displacement `+0x0` (`ldm` at 0x58a0).
  At the return (`ldm` at 0x58a0), r0 is `ret(linked ELF 0x1c54)`.
  Unconditional return boundary (`ldm` at 0x58a0): the modeled local edges stop before the trailing region 0x58a4 to 0x58b4 (exclusive).
  The decoded instructions from 0x58a4 to 0x58ac are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x58b0 to 0x58b4 (exclusive), 4 bytes `94fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.

- `IRouterFreePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x2bac to 0x2e70 (exclusive), 708 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `459755b3224bdca6e3468507d7c0c08737639c3d0bf5cb32ad663782a4354da1`.
  The 9 unapplied ELF relocation records at linked places 0x2e50, 0x2e54, 0x2e5c, 0x2e64, 0x2e6c, 0x2e68, 0x2e48, 0x2e4c, 0x2e58 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x2bb0).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x2bbc).
  Four-byte load: target `ELF load bias (unknown) +0x2e48`, base register `pc`, raw displacement `+0x280`, linked address 0x2e48, basis `ELF load-bias-relative` (`ldr` at 0x2bc0).
  ELF literal warning: linked 0x2e48, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x2bc0, `ldr` at 0x2e0c.
  Grouped accesses: 13 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x2bc4, `ldr` at 0x2c00, `ldr` at 0x2c50.
  Branch (`beq` at 0x2bcc): taken to 0x2c08 when `? == 0x0`, falls through to 0x2bd0 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x2e4c`, base register `pc`, raw displacement `+0x274`, linked address 0x2e4c, basis `ELF load-bias-relative` (`ldr` at 0x2bd0).
  ELF literal warning: linked 0x2e4c, value unknown (relocation, writable or unmapped bytes), 5 read records, including `ldr` at 0x2bd0, `ldr` at 0x2bfc, `ldr` at 0x2c70.
  Grouped accesses: 18 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x2bd4, `ldr` at 0x2bf4, `ldr` at 0x2c08.
  Two-byte load: target `r0 +0x4`, base register `r3`, raw displacement `+0x4` (`ldrh` at 0x2bd8).
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, including `ldr` at 0x2bec, `ldr` at 0x2c8c, `ldr` at 0x2e2c.
  Four-byte load: target `ELF load bias (unknown) +0x2e50`, base register `pc`, raw displacement `+0x258`, linked address 0x2e50, basis `ELF load-bias-relative` (`ldr` at 0x2bf0).
  ELF literal warning: linked 0x2e50, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2bf0.
  Call (`bl` at 0x2bf8): (unknown callee), target 0x1ce4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1ce4, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x2e4c`, base register `pc`, raw displacement `+0x248`, linked address 0x2e4c, basis `ELF load-bias-relative` (`ldr` at 0x2bfc).
  Call (`bl` at 0x2c04): (unknown callee), target 0x1cf0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cf0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 3 2-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x4`, including `ldrh` at 0x2c0c, `ldrh` at 0x2c78, `ldrh` at 0x2ca4.
  Branch (`bne` at 0x2c20): taken to 0x2c4c when `? != 0x0`, falls through to 0x2c24 when `? == 0x0`.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x16` (`ldrb` at 0x2c28).
  Four-byte load: target `ELF load bias (unknown) +0x2e54`, base register `pc`, raw displacement `+0x21c`, linked address 0x2e54, basis `ELF load-bias-relative` (`ldr` at 0x2c30).
  ELF literal warning: linked 0x2e54, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2c30.
  Four-byte load: target `(unknown effective address)`, base register `r2`, raw displacement `+0x1c` (`ldr` at 0x2c38).
  Call (`bl` at 0x2c3c): (unknown callee), target 0x1dec, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dec, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 4-byte store records with target `sp -0x24`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x20`, including `str` at 0x2c44, `str` at 0x2e38.
  Jump (`b` at 0x2c48): target 0x2e3c.
  Four-byte load: target `ELF load bias (unknown) +0x2e58`, base register `pc`, raw displacement `+0x204`, linked address 0x2e58, basis `ELF load-bias-relative` (`ldr` at 0x2c4c).
  ELF literal warning: linked 0x2e58, value unknown (relocation, writable or unmapped bytes), 5 read records, including `ldr` at 0x2c4c, `ldr` at 0x2c5c, `ldr` at 0x2db4.
  Branch (`beq` at 0x2c58): taken to 0x2c9c when `? == 0x0`, falls through to 0x2c5c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x2e58`, base register `pc`, raw displacement `+0x1f4`, linked address 0x2e58, basis `ELF load-bias-relative` (`ldr` at 0x2c5c).
  Branch (`bne` at 0x2c6c): taken to 0x2c9c when `Z == 0`, falls through to 0x2c70 when `not (Z == 0)`.
  Four-byte load: target `ELF load bias (unknown) +0x2e4c`, base register `pc`, raw displacement `+0x1d4`, linked address 0x2e4c, basis `ELF load-bias-relative` (`ldr` at 0x2c70).
  Four-byte load: target `ELF load bias (unknown) +0x2e5c`, base register `pc`, raw displacement `+0x1c4`, linked address 0x2e5c, basis `ELF load-bias-relative` (`ldr` at 0x2c90).
  ELF literal warning: linked 0x2e5c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2c90.
  Call (`bl` at 0x2c98): (unknown callee), target 0x1ce4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1ce4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Two-byte load: target `(unknown effective address)`, base register `r1`, raw displacement `+0x4` (`ldrh` at 0x2cb0).
  Grouped accesses: 2 2-byte store records with target `sp -0x28`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x24`, including `strh` at 0x2cb4, `strh` at 0x2cd4.
  Four-byte load: target `ELF load bias (unknown) +0x2e60`, base register `pc`, raw displacement `+0x1a0`, linked address 0x2e60, basis `ELF load-bias-relative` (`ldr` at 0x2cb8).
  Grouped accesses: 2 2-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldrh` at 0x2cc4, `ldrh` at 0x2cd8.
  Two-byte store: target `(unknown effective address)`, value `([sp-40]:2 & 0xffff)`, base register `r1`, raw displacement `+0x4` (`strh` at 0x2cdc).
  Branch (`bne` at 0x2cf8): taken to 0x2e34 when `? != 0x0`, falls through to 0x2cfc when `? == 0x0`.
  Grouped accesses: 2 4-byte load records with target `[sp-20] +0x8`, base register `r3`, raw displacement `+0x8`, including `ldr` at 0x2d00, `ldr` at 0x2d78.
  Branch (`beq` at 0x2d08): taken to 0x2d80 when `[[sp-20]+8] == 0x0`, falls through to 0x2d0c when `[[sp-20]+8] != 0x0`.
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0x2d10).
  Byte load: target `[sp-20] +0x20`, base register `r3`, raw displacement `+0x20` (`ldrb` at 0x2d18).
  Grouped accesses: 4 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x2d1c, `ldr` at 0x2d2c, `ldr` at 0x2d4c.
  Branch (`ble` at 0x2d24): taken to 0x2d74 when `le flags condition (operands unknown)`, falls through to 0x2d28 when `not (le flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `[sp-20] +0x8`, base register `r1`, raw displacement `+0x8`, including `ldr` at 0x2d34, `ldr` at 0x2d54.
  Branch (`beq` at 0x2d44): taken to 0x2d64 when `Z == 1`, falls through to 0x2d48 when `not (Z == 1)`.
  Call (`bl` at 0x2d60): (unknown callee), target 0x1e4c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e4c, r0=`unknown`, r1=`[sp-20]`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `sp -0x20`, value `unknown`, base register `r11`, raw displacement `-0x1c` (`str` at 0x2d6c).
  Jump (`b` at 0x2d70): target 0x2d14.
  Call (`bl` at 0x2d7c): (unknown callee), target 0x1e4c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e4c, r0=`[[sp-20]+8]`, r1=`unknown`, r2=`[[sp-20]+32]:1`, r3=`[sp-20]`.
  Four-byte store: target `sp -0x1c`, value `[sp-20]`, base register `r11`, raw displacement `-0x18` (`str` at 0x2d84).
  Grouped accesses: 3 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x2d88, `ldr` at 0x2d94, `ldr` at 0x2d9c.
  Branch (`beq` at 0x2d90): taken to 0x2db4 when `Z == 1`, falls through to 0x2d94 when `not (Z == 1)`.
  Four-byte store: target `sp -0x18`, value `unknown`, base register `r11`, raw displacement `-0x14` (`str` at 0x2d98).
  Four-byte store: target `sp -0x1c`, value `unknown`, base register `r11`, raw displacement `-0x18` (`str` at 0x2da4).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0x2da8).
  Call (`bl` at 0x2dac): (unknown callee), target 0x1e4c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e4c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0x2db0): target 0x2d88.
  Four-byte load: target `ELF load bias (unknown) +0x2e58`, base register `pc`, raw displacement `+0x9c`, linked address 0x2e58, basis `ELF load-bias-relative` (`ldr` at 0x2db4).
  Branch (`beq` at 0x2dc0): taken to 0x2df8 when `? == 0x0`, falls through to 0x2dc4 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x2e58`, base register `pc`, raw displacement `+0x8c`, linked address 0x2e58, basis `ELF load-bias-relative` (`ldr` at 0x2dc4).
  Branch (`bne` at 0x2dd4): taken to 0x2df8 when `[sp-20] != ?`, falls through to 0x2dd8 when `[sp-20] == ?`.
  Four-byte load: target `ELF load bias (unknown) +0x2e58`, base register `pc`, raw displacement `+0x78`, linked address 0x2e58, basis `ELF load-bias-relative` (`ldr` at 0x2dd8).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r3`, raw displacement `+0x0` (`str` at 0x2de0).
  Four-byte load: target `ELF load bias (unknown) +0x2e4c`, base register `pc`, raw displacement `+0x60`, linked address 0x2e4c, basis `ELF load-bias-relative` (`ldr` at 0x2de4).
  Four-byte load: target `ELF load bias (unknown) +0x2e64`, base register `pc`, raw displacement `+0x70`, linked address 0x2e64, basis `ELF load-bias-relative` (`ldr` at 0x2dec).
  ELF literal warning: linked 0x2e64, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2dec.
  Call (`bl` at 0x2df4): (unknown callee), target 0x1ce4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1ce4, r0=`unknown`, r1=`unknown`, r2=`[sp-20]`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x2e68`, base register `pc`, raw displacement `+0x68`, linked address 0x2e68, basis `ELF load-bias-relative` (`ldr` at 0x2df8).
  ELF literal warning: linked 0x2e68, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x2df8, `ldr` at 0x2dfc, `ldr` at 0x2e20.
  Four-byte load: target `ELF load bias (unknown) +0x2e68`, base register `pc`, raw displacement `+0x64`, linked address 0x2e68, basis `ELF load-bias-relative` (`ldr` at 0x2dfc).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0` (`str` at 0x2e08).
  Four-byte load: target `ELF load bias (unknown) +0x2e48`, base register `pc`, raw displacement `+0x34`, linked address 0x2e48, basis `ELF load-bias-relative` (`ldr` at 0x2e0c).
  Branch (`beq` at 0x2e18): taken to 0x2e34 when `? == 0x0`, falls through to 0x2e1c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x2e4c`, base register `pc`, raw displacement `+0x28`, linked address 0x2e4c, basis `ELF load-bias-relative` (`ldr` at 0x2e1c).
  Four-byte load: target `ELF load bias (unknown) +0x2e68`, base register `pc`, raw displacement `+0x40`, linked address 0x2e68, basis `ELF load-bias-relative` (`ldr` at 0x2e20).
  Four-byte load: target `ELF load bias (unknown) +0x2e6c`, base register `pc`, raw displacement `+0x3c`, linked address 0x2e6c, basis `ELF load-bias-relative` (`ldr` at 0x2e28).
  ELF literal warning: linked 0x2e6c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2e28.
  Call (`bl` at 0x2e30): (unknown callee), target 0x1ce4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1ce4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0x2e3c).
  The exit (`ldm` at 0x2e44) reports r0 = `0x0`.
  Unconditional return boundary (`ldm` at 0x2e44): the modeled local edges stop before the trailing region 0x2e48 to 0x2e70 (exclusive).
  The decoded instructions from 0x2e48 to 0x2e6c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterMakePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x2ef8 to 0x30fc (exclusive), 516 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `1b13925af17460990b8a1f590a4eb3713d434576d78d6ee27bb9ddbe48ee5d4c`.
  The analyzer retains no ELF relocation records for this span.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x2efc).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x2f08).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0x2f0c).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0x2f10).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0x2f14).
  Grouped accesses: 2 4-byte store records with target `sp -0x28`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x24`, including `str` at 0x2f1c, `str` at 0x30e4.
  Grouped accesses: 5 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x2f20, `ldr` at 0x2f44, `ldr` at 0x2f54.
  Branch (`beq` at 0x2f28): taken to 0x3054 when `r1 == 0x0`, falls through to 0x2f2c when `r1 != 0x0`.
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x2f2c, `ldr` at 0x30bc.
  Branch (`bne` at 0x2f34): taken to 0x3054 when `r3 != 0x1`, falls through to 0x2f38 when `r3 == 0x1`.
  Four-byte load: target `sp +0x0`, base register `r11`, raw displacement `+0x4` (`ldr` at 0x2f38).
  Branch (`ble` at 0x2f40): taken to 0x3054 when `[sp] <= 0x1 (signed, 32-bit)`, falls through to 0x2f44 when `[sp] > 0x1 (signed, 32-bit)`.
  Grouped accesses: 3 4-byte load records with target `r1 +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x2f48, `ldr` at 0x2f58, `ldr` at 0x2f84.
  Branch (`bge` at 0x2f50): taken to 0x2f80 when `[r1] >= 0x0 (signed, 32-bit)`, falls through to 0x2f54 when `[r1] < 0x0 (signed, 32-bit)`.
  Branch (`bne` at 0x2f64): taken to 0x2f74 when `? != 0x2`, falls through to 0x2f68 when `? == 0x2`.
  Four-byte store: target `sp -0x30`, value `0xe`, base register `r11`, raw displacement `-0x2c` (`str` at 0x2f6c).
  Jump (`b` at 0x2f70): target 0x2f9c.
  Four-byte store: target `sp -0x30`, value `0xf`, base register `r11`, raw displacement `-0x2c` (`str` at 0x2f78).
  Jump (`b` at 0x2f7c): target 0x2f9c.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x2f8c, `str` at 0x2f98.
  Grouped accesses: 3 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x2f90, `ldr` at 0x2f9c, `ldr` at 0x2fac.
  Branch (`bhi` at 0x2fa4): taken to 0x3054 when `[sp-48] > 0xf (unsigned, 32-bit)`, falls through to 0x2fa8 when `[sp-48] <= 0xf (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x30f8`, base register `pc`, raw displacement `+0x13c`, linked address 0x30f8, basis `ELF load-bias-relative` (`ldr` at 0x2fb4).
  Branch (`bne` at 0x2fc0): taken to 0x2fc8 when `? != 0x0`, falls through to 0x2fc4 when `? == 0x0`.
  Jump (`b` at 0x2fc4): target 0x3054.
  Grouped accesses: 6 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x2fc8, `ldr` at 0x2fd8, `ldr` at 0x3004.
  Grouped accesses: 3 4-byte load records with target `r0 +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x2fcc, `ldr` at 0x2fdc, `ldr` at 0x3008.
  Branch (`bge` at 0x2fd4): taken to 0x3004 when `[r0] >= 0x0 (signed, 32-bit)`, falls through to 0x2fd8 when `[r0] < 0x0 (signed, 32-bit)`.
  Branch (`bne` at 0x2fe8): taken to 0x2ff8 when `? != 0x2`, falls through to 0x2fec when `? == 0x2`.
  Four-byte store: target `sp -0x2c`, value `0xe`, base register `r11`, raw displacement `-0x28` (`str` at 0x2ff0).
  Jump (`b` at 0x2ff4): target 0x3020.
  Four-byte store: target `sp -0x2c`, value `0xf`, base register `r11`, raw displacement `-0x28` (`str` at 0x2ffc).
  Jump (`b` at 0x3000): target 0x3020.
  Grouped accesses: 2 4-byte store records with target `sp -0x2c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x28`, including `str` at 0x3010, `str` at 0x301c.
  Grouped accesses: 3 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x3014, `ldr` at 0x3020, `ldr` at 0x3030.
  Branch (`bhi` at 0x3028): taken to 0x3054 when `[sp-44] > 0xf (unsigned, 32-bit)`, falls through to 0x302c when `[sp-44] <= 0xf (unsigned, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x30f8`, base register `pc`, raw displacement `+0xb8`, linked address 0x30f8, basis `ELF load-bias-relative` (`ldr` at 0x3038).
  Branch (`bne` at 0x3044): taken to 0x304c when `? != 0x0`, falls through to 0x3048 when `? == 0x0`.
  Jump (`b` at 0x3048): target 0x3054.
  Four-byte store: target `sp -0x28`, value `0x1`, base register `r11`, raw displacement `-0x24` (`str` at 0x3050).
  Byte load: target `r0 +0x4`, base register `r3`, raw displacement `+0x4` (`ldrb` at 0x305c).
  Four-byte load: target `sp -0x1c`, base register `r11`, raw displacement `-0x18` (`ldr` at 0x3060).
  Four-byte store: target `sp -0x54`, value `r2`, base register `sp`, raw displacement `+0x0` (`str` at 0x3064).
  Four-byte store: target `sp -0x50`, value `0x0`, base register `sp`, raw displacement `+0x4` (`str` at 0x306c).
  Two-byte load: target `r0 +0x6`, base register `r2`, raw displacement `+0x6` (`ldrh` at 0x3074).
  Two-byte store: target `r0 +0x6`, value `([r0+6]:2+1 & 0xffff)`, base register `r2`, raw displacement `+0x6` (`strh` at 0x307c).
  Four-byte store: target `sp -0x4c`, value `unknown`, base register `sp`, raw displacement `+0x8` (`str` at 0x3088).
  Four-byte store: target `sp -0x48`, value `[sp-24]`, base register `sp`, raw displacement `+0xc` (`str` at 0x3090).
  Four-byte store: target `sp -0x44`, value `[sp-20]`, base register `sp`, raw displacement `+0x10` (`str` at 0x3098).
  Four-byte load: target `sp +0x4`, base register `r11`, raw displacement `+0x8` (`ldr` at 0x309c).
  Four-byte store: target `sp -0x40`, value `[sp+4]`, base register `sp`, raw displacement `+0x14` (`str` at 0x30a0).
  Four-byte load: target `sp +0x8`, base register `r11`, raw displacement `+0xc` (`ldr` at 0x30a4).
  Four-byte store: target `sp -0x3c`, value `[sp+8]`, base register `sp`, raw displacement `+0x18` (`str` at 0x30a8).
  Four-byte store: target `sp -0x38`, value `0x0`, base register `sp`, raw displacement `+0x1c` (`str` at 0x30b0).
  Four-byte store: target `sp -0x34`, value `0x0`, base register `sp`, raw displacement `+0x20` (`str` at 0x30b8).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x30c0).
  Call (`bl` at 0x30c8): (unknown callee), target 0x21b4, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x21b4, r0=`sp-36`, r1=`[sp-32]`, r2=`[sp-40]`, r3=`[r0+4]:1`.
  Branch (`beq` at 0x30d4): taken to 0x30e0 when `ret(linked ELF 0x21b4) == 0x0`, falls through to 0x30d8 when `ret(linked ELF 0x21b4) != 0x0`.
  Four-byte store: target `sp -0x24`, value `0x0`, base register `r11`, raw displacement `-0x20` (`str` at 0x30dc).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0x30e8).
  Recorded r0 at the return is `[sp-36]` (`ldm` at 0x30f4).
  Unconditional return boundary (`ldm` at 0x30f4): the modeled local edges stop before the trailing region 0x30f8 to 0x30fc (exclusive).
  The decoded instructions from 0x30f8 to 0x30f8 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterConnectL3`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x4858 to 0x4b30 (exclusive), 728 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `9e12d4c135a25f64e3d34a1dee3d45d2602bf39801b0f6a8599038a03da8a766`.
  The 15 unapplied ELF relocation records at linked places 0x4af0, 0x4af4, 0x4af8, 0x4afc, 0x4b10, 0x4b24, 0x4b00, 0x4b04, 0x4b08, 0x4b0c, 0x4b14, 0x4b18, 0x4b1c, 0x4b20, 0x4b2c retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0x485c).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0x4868).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0x486c).
  Four-byte store: target `sp -0x20`, value `r2`, base register `r11`, raw displacement `-0x1c` (`str` at 0x4870).
  Four-byte store: target `sp -0x24`, value `r3`, base register `r11`, raw displacement `-0x20` (`str` at 0x4874).
  Four-byte store: target `sp -0x28`, value `0x12c`, base register `r11`, raw displacement `-0x24` (`str` at 0x487c).
  Four-byte store: target `sp -0x2c`, value `0x100`, base register `r11`, raw displacement `-0x28` (`str` at 0x4884).
  Four-byte store: target `sp -0x34`, value `0x2`, base register `r11`, raw displacement `-0x30` (`str` at 0x488c).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x4890, `ldr` at 0x4898.
  Four-byte store: target `sp -0x90`, value `r0`, base register `r11`, raw displacement `-0x8c` (`str` at 0x4894).
  Branch (`bne` at 0x48a0): taken to 0x48ac when `r0 != 0x0`, falls through to 0x48a4 when `r0 == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4af0`, base register `pc`, raw displacement `+0x244`, linked address 0x4af0, basis `ELF load-bias-relative` (`ldr` at 0x48a4).
  ELF literal warning: linked 0x4af0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x48a4.
  Four-byte store: target `sp -0x90`, value `unknown`, base register `r11`, raw displacement `-0x8c` (`str` at 0x48a8).
  Grouped accesses: 2 4-byte store records with target `sp -0xa4`, reported value `0x0` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0x48b0, `str` at 0x4a34.
  Grouped accesses: 2 4-byte load records with target `sp -0x2c`, base register `r11`, raw displacement `-0x28`, including `ldr` at 0x48b4, `ldr` at 0x4934.
  Four-byte load: target `sp -0x90`, base register `r11`, raw displacement `-0x8c` (`ldr` at 0x48bc).
  Four-byte load: target `sp +0x0`, base register `r11`, raw displacement `+0x4` (`ldr` at 0x48c0).
  Call (`bl` at 0x48c4): (unknown callee), target 0x1d2c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d2c, r0=`0x100`, r1=`0x1`, r2=`unknown`, r3=`[sp]`.
  Branch (`bge` at 0x48d0): taken to 0x48f8 when `ret(linked ELF 0x1d2c) >= 0x0 (signed, 32-bit)`, falls through to 0x48d4 when `ret(linked ELF 0x1d2c) < 0x0 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0x4af4`, base register `pc`, raw displacement `+0x218`, linked address 0x4af4, basis `ELF load-bias-relative` (`ldr` at 0x48d4).
  ELF literal warning: linked 0x4af4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x48d4.
  Grouped accesses: 3 4-byte store records with target `sp -0xa4`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0x48d8, `str` at 0x48fc, `str` at 0x4984.
  Four-byte load: target `ELF load bias (unknown) +0x4af8`, base register `pc`, raw displacement `+0x210`, linked address 0x4af8, basis `ELF load-bias-relative` (`ldr` at 0x48e0).
  ELF literal warning: linked 0x4af8, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x48e0, `ldr` at 0x4914, `ldr` at 0x498c.
  Call (`bl` at 0x48ec): (unknown callee), target 0x1d80, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d80, r0=`0x6`, r1=`unknown`, r2=`0x50`, r3=`0x0`.
  The analyzer invalidates unsupported effects (`mvn` at 0x48f0).
  Call (`bl` at 0x48f4): (unknown callee), target 0x1e40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e40, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x4afc`, base register `pc`, raw displacement `+0x1fc`, linked address 0x4afc, basis `ELF load-bias-relative` (`ldr` at 0x48f8).
  ELF literal warning: linked 0x4afc, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x48f8.
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x4900, `ldr` at 0x495c.
  Four-byte store: target `sp -0xa0`, value `[sp-28]`, base register `sp`, raw displacement `+0x4` (`str` at 0x4904).
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x4908, `ldr` at 0x4960.
  Four-byte store: target `sp -0x9c`, value `[sp-32]`, base register `sp`, raw displacement `+0x8` (`str` at 0x490c).
  Four-byte load: target `ELF load bias (unknown) +0x4af8`, base register `pc`, raw displacement `+0x1dc`, linked address 0x4af8, basis `ELF load-bias-relative` (`ldr` at 0x4914).
  Call (`bl` at 0x4920): (unknown callee), target 0x1d80, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d80, r0=`0x6`, r1=`unknown`, r2=`0x54`, r3=`0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4b00`, base register `pc`, raw displacement `+0x1d4`, linked address 0x4b00, basis `ELF load-bias-relative` (`ldr` at 0x4924).
  ELF literal warning: linked 0x4b00, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4924.
  Four-byte load: target `sp +0x4`, base register `r11`, raw displacement `+0x8` (`ldr` at 0x4928).
  Four-byte store: target `(unknown effective address)`, value `[sp+4]`, base register `r2`, raw displacement `+0x0` (`str` at 0x492c).
  Grouped accesses: 5 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x4930, `ldr` at 0x49fc, `ldr` at 0x4a7c.
  Four-byte store: target `sp -0xa4`, value `[sp-44]`, base register `sp`, raw displacement `+0x0` (`str` at 0x4938).
  Four-byte load: target `ELF load bias (unknown) +0x4b04`, base register `pc`, raw displacement `+0x1c0`, linked address 0x4b04, basis `ELF load-bias-relative` (`ldr` at 0x493c).
  ELF literal warning: linked 0x4b04, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x493c.
  Grouped accesses: 2 4-byte store records with target `sp -0xa0`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0x4940, `str` at 0x4a3c.
  Four-byte store: target `sp -0x9c`, value `0x0`, base register `sp`, raw displacement `+0x8` (`str` at 0x4948).
  Four-byte load: target `ELF load bias (unknown) +0x4b08`, base register `pc`, raw displacement `+0x1b4`, linked address 0x4b08, basis `ELF load-bias-relative` (`ldr` at 0x494c).
  ELF literal warning: linked 0x4b08, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x494c.
  Four-byte store: target `sp -0x98`, value `unknown`, base register `sp`, raw displacement `+0xc` (`str` at 0x4950).
  Four-byte load: target `ELF load bias (unknown) +0x4b0c`, base register `pc`, raw displacement `+0x1b0`, linked address 0x4b0c, basis `ELF load-bias-relative` (`ldr` at 0x4954).
  ELF literal warning: linked 0x4b0c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4954.
  Four-byte store: target `sp -0x94`, value `unknown`, base register `sp`, raw displacement `+0x10` (`str` at 0x4958).
  Four-byte load: target `sp -0x34`, base register `r11`, raw displacement `-0x30` (`ldr` at 0x4964).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x4968).
  Call (`bl` at 0x496c): (unknown callee), target 0x1c6c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c6c, r0=`[sp-28]`, r1=`[sp-32]`, r2=`[sp-52]`, r3=`[sp-40]`.
  Four-byte store: target `[sp-36] +0x0`, value `ret(linked ELF 0x1c6c)`, base register `r4`, raw displacement `+0x0` (`str` at 0x4970).
  Four-byte load: target `[sp-36] +0x0`, base register `r4`, raw displacement `+0x0` (`ldr` at 0x4974).
  Branch (`bne` at 0x497c): taken to 0x49a4 when `[[sp-36]] != 0x0`, falls through to 0x4980 when `[[sp-36]] == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4b10`, base register `pc`, raw displacement `+0x188`, linked address 0x4b10, basis `ELF load-bias-relative` (`ldr` at 0x4980).
  ELF literal warning: linked 0x4b10, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4980.
  Four-byte load: target `ELF load bias (unknown) +0x4af8`, base register `pc`, raw displacement `+0x164`, linked address 0x4af8, basis `ELF load-bias-relative` (`ldr` at 0x498c).
  Call (`bl` at 0x4998): (unknown callee), target 0x1d80, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d80, r0=`0x6`, r1=`unknown`, r2=`0x60`, r3=`0x0`.
  The analyzer invalidates unsupported effects (`mvn` at 0x499c).
  Call (`bl` at 0x49a0): (unknown callee), target 0x1e40, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e40, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x4b14`, base register `pc`, raw displacement `+0x168`, linked address 0x4b14, basis `ELF load-bias-relative` (`ldr` at 0x49a4).
  ELF literal warning: linked 0x4b14, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x49a4, `ldr` at 0x49b0, `ldr` at 0x49f0.
  Grouped accesses: 7 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x49a8, `ldr` at 0x4a04, `ldr` at 0x4a10.
  Four-byte store: target `sp -0x30`, value `unknown`, base register `r11`, raw displacement `-0x2c` (`str` at 0x49ac).
  Four-byte load: target `ELF load bias (unknown) +0x4b14`, base register `pc`, raw displacement `+0x15c`, linked address 0x4b14, basis `ELF load-bias-relative` (`ldr` at 0x49b0).
  Grouped accesses: 5 4-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x0`, including `str` at 0x49b8, `str` at 0x49c4, `str` at 0x49d0.
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0x154`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x49bc).
  ELF literal warning: linked 0x4b18, value unknown (relocation, writable or unmapped bytes), 7 read records, including `ldr` at 0x49bc, `ldr` at 0x4a0c, `ldr` at 0x4a1c.
  Four-byte load: target `ELF load bias (unknown) +0x4b1c`, base register `pc`, raw displacement `+0x14c`, linked address 0x4b1c, basis `ELF load-bias-relative` (`ldr` at 0x49c8).
  ELF literal warning: linked 0x4b1c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x49c8, `ldr` at 0x4a00.
  Four-byte load: target `ELF load bias (unknown) +0x4b20`, base register `pc`, raw displacement `+0x144`, linked address 0x4b20, basis `ELF load-bias-relative` (`ldr` at 0x49d4).
  ELF literal warning: linked 0x4b20, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x49d4, `ldr` at 0x49e4.
  Four-byte store: target `(unknown effective address)`, value `0x1`, base register `r2`, raw displacement `+0x0` (`str` at 0x49dc).
  Call (`bl` at 0x49e0): (unknown callee), target 0x1e34, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e34, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x4b20`, base register `pc`, raw displacement `+0x134`, linked address 0x4b20, basis `ELF load-bias-relative` (`ldr` at 0x49e4).
  Four-byte load: target `ELF load bias (unknown) +0x4b14`, base register `pc`, raw displacement `+0x11c`, linked address 0x4b14, basis `ELF load-bias-relative` (`ldr` at 0x49f0).
  Four-byte load: target `sp -0x30`, base register `r11`, raw displacement `-0x2c` (`ldr` at 0x49f4).
  Four-byte store: target `(unknown effective address)`, value `[sp-48]`, base register `r2`, raw displacement `+0x0` (`str` at 0x49f8).
  Four-byte load: target `ELF load bias (unknown) +0x4b1c`, base register `pc`, raw displacement `+0x114`, linked address 0x4b1c, basis `ELF load-bias-relative` (`ldr` at 0x4a00).
  Four-byte store: target `[sp-36] +0x0`, value `unknown`, base register `r2`, raw displacement `+0x0` (`str` at 0x4a08).
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0x104`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x4a0c).
  Branch (`beq` at 0x4a18): taken to 0x4a2c when `? == 0x0`, falls through to 0x4a1c when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0xf4`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x4a1c).
  Grouped accesses: 2 4-byte store records with target `sp -0x8c`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x88`, including `str` at 0x4a24, `str` at 0x4ae0.
  Jump (`b` at 0x4a28): target 0x4ae4.
  Four-byte load: target `ELF load bias (unknown) +0x4b24`, base register `pc`, raw displacement `+0xe4`, linked address 0x4b24, basis `ELF load-bias-relative` (`ldr` at 0x4a38).
  ELF literal warning: linked 0x4b24, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4a38.
  Four-byte load: target `ELF load bias (unknown) +0x4b28`, base register `pc`, raw displacement `+0xd8`, linked address 0x4b28, basis `ELF load-bias-relative` (`ldr` at 0x4a48).
  Four-byte load: target `ELF load bias (unknown) +0x4b2c`, base register `pc`, raw displacement `+0xd8`, linked address 0x4b2c, basis `ELF load-bias-relative` (`ldr` at 0x4a4c).
  ELF literal warning: linked 0x4b2c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4a4c.
  Call (`bl` at 0x4a50): (unknown callee), target 0x1cc0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cc0, r0=`sp-136`, r1=`0x0`, r2=`0x7a120`, r3=`unknown`.
  Call (`bl` at 0x4a68): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`0x0`, r1=`0x0`, r2=`sp-136`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0xa4`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x4a6c).
  The analyzer invalidates unsupported effects (`cmn` at 0x4a74).
  Branch (`bne` at 0x4a78): taken to 0x4ac4 when `Z == 0`, falls through to 0x4a7c when `not (Z == 0)`.
  Grouped accesses: 3 4-byte load records with target `[sp-36] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x4a80, `ldr` at 0x4a90, `ldr` at 0x4aa4.
  Branch (`beq` at 0x4a88): taken to 0x4ac4 when `[[sp-36]] == 0x0`, falls through to 0x4a8c when `[[sp-36]] != 0x0`.
  Four-byte load: target `[[sp-36]] +0xfc`, base register `r3`, raw displacement `+0xfc` (`ldr` at 0x4a94).
  Branch (`beq` at 0x4a9c): taken to 0x4ac4 when `[[[sp-36]]+252] == 0x0`, falls through to 0x4aa0 when `[[[sp-36]]+252] != 0x0`.
  Four-byte load: target `[[sp-36]] +0x158`, base register `r3`, raw displacement `+0x158` (`ldr` at 0x4aa8).
  Branch (`beq` at 0x4ab0): taken to 0x4ac4 when `[[[sp-36]]+344] == 0x0`, falls through to 0x4ab4 when `[[[sp-36]]+344] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0x5c`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x4ab4).
  Jump (`b` at 0x4ac0): target 0x4ad8.
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0x4c`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x4ac4).
  The analyzer invalidates unsupported effects (`cmn` at 0x4acc).
  Branch (`bne` at 0x4ad0): taken to 0x4ad8 when `Z == 0`, falls through to 0x4ad4 when `not (Z == 0)`.
  Jump (`b` at 0x4ad4): target 0x4a2c.
  Four-byte load: target `ELF load bias (unknown) +0x4b18`, base register `pc`, raw displacement `+0x38`, linked address 0x4b18, basis `ELF load-bias-relative` (`ldr` at 0x4ad8).
  Four-byte load: target `sp -0x8c`, base register `r11`, raw displacement `-0x88` (`ldr` at 0x4ae4).
  `ldm` at 0x4aec has recorded r0 = `[sp-140]`.
  Unconditional return boundary (`ldm` at 0x4aec): the modeled local edges stop before the trailing region 0x4af0 to 0x4b30 (exclusive).
  The decoded instructions from 0x4af0 to 0x4b2c are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x29e8 to 0x2afc (exclusive), 276 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `c0d784cbe51e42944bce30d918df570f24a31cccac29712508ccabed680fa951`.
  The 3 unapplied ELF relocation records at linked places 0x2af0, 0x2af4, 0x2af8 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x14`, implicit access (`push` at 0x29ec).
  Four-byte store: target `sp -0x1c`, value `r0`, base register `r11`, raw displacement `-0x18` (`str` at 0x29f8).
  Four-byte store: target `sp -0x20`, value `0x1`, base register `r11`, raw displacement `-0x1c` (`str` at 0x2a00).
  Byte load: target `sp -0x1d`, base register `r11`, raw displacement `-0x19` (`ldrb` at 0x2a04).
  Branch (`beq` at 0x2a0c): taken to 0x2a20 when `[sp-29]:1 == 0x0`, falls through to 0x2a10 when `[sp-29]:1 != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x2af0`, base register `pc`, raw displacement `+0xd8`, linked address 0x2af0, basis `ELF load-bias-relative` (`ldr` at 0x2a10).
  ELF literal warning: linked 0x2af0, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x2a10, `ldr` at 0x2a20, `ldr` at 0x2a60.
  Four-byte store: target `(unknown effective address)`, value `0x1`, base register `r2`, raw displacement `+0x0` (`str` at 0x2a18).
  Jump (`b` at 0x2a1c): target 0x2a2c.
  Four-byte load: target `ELF load bias (unknown) +0x2af0`, base register `pc`, raw displacement `+0xc8`, linked address 0x2af0, basis `ELF load-bias-relative` (`ldr` at 0x2a20).
  Four-byte store: target `(unknown effective address)`, value `0x2`, base register `r2`, raw displacement `+0x0` (`str` at 0x2a28).
  Four-byte store: target `sp -0x20`, value `0x0`, base register `r11`, raw displacement `-0x1c` (`str` at 0x2a30).
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0xb8`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a34).
  ELF literal warning: linked 0x2af4, value unknown (relocation, writable or unmapped bytes), 8 read records, including `ldr` at 0x2a34, `ldr` at 0x2a40, `ldr` at 0x2a44.
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x0` (`str` at 0x2a3c).
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0xac`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a40).
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0xa8`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a44).
  Grouped accesses: 4 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x2a48, `ldr` at 0x2a64, `ldr` at 0x2a78.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r2`, raw displacement `+0x0` (`str` at 0x2a54).
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0x94`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a58).
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0x2a5c).
  Four-byte load: target `ELF load bias (unknown) +0x2af0`, base register `pc`, raw displacement `+0x88`, linked address 0x2af0, basis `ELF load-bias-relative` (`ldr` at 0x2a60).
  Branch (`bne` at 0x2a6c): taken to 0x2a98 when `? != 0x1`, falls through to 0x2a70 when `? == 0x1`.
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0x7c`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a70).
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0x78`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a74).
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x2a80, `ldr` at 0x2aa8.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r1`, raw displacement `+0x0` (`str` at 0x2a8c).
  Grouped accesses: 2 4-byte store records with target `sp -0x28`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x24`, including `str` at 0x2a90, `str` at 0x2ac0.
  Jump (`b` at 0x2a94): target 0x2ac4.
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0x54`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a98).
  Four-byte load: target `ELF load bias (unknown) +0x2af4`, base register `pc`, raw displacement `+0x50`, linked address 0x2af4, basis `ELF load-bias-relative` (`ldr` at 0x2a9c).
  Call (`bl` at 0x2aac): (unknown callee), target 0x1c48, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c48, r0=`[sp-28]`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r5`, raw displacement `+0x0` (`str` at 0x2abc).
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x2ac4).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0x2ac8).
  Four-byte store: target `[sp-36] +0x0`, value `[sp-40]`, base register `r3`, raw displacement `+0x0` (`str` at 0x2acc).
  Four-byte load: target `ELF load bias (unknown) +0x2af8`, base register `pc`, raw displacement `+0x20`, linked address 0x2af8, basis `ELF load-bias-relative` (`ldr` at 0x2ad0).
  ELF literal warning: linked 0x2af8, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x2ad0.
  Call (`bl` at 0x2adc): (unknown callee), target 0x1c60, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1c60, r0=`unknown`, r1=`0x1`, r2=`0x3`, r3=`[sp-36]`.
  Four-byte load: target `sp -0x20`, base register `r11`, raw displacement `-0x1c` (`ldr` at 0x2ae0).
  At the return (`ldm` at 0x2aec), r0 is `[sp-32]`.
  Unconditional return boundary (`ldm` at 0x2aec): the modeled local edges stop before the trailing region 0x2af0 to 0x2afc (exclusive).
  The decoded instructions from 0x2af0 to 0x2af8 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `IRouterGetPacketL3`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libIRouter.armlinux.so`.
  Inferred analysis span: 0x4ea8 to 0x53a8 (exclusive), 1280 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 32496 bytes, SHA256 `57c3edeb6354e82cdac736f9c6dab2cd191207dff107b4313dca1e8cfd2ad8c6`.
  Read window: SHA256 `f63b26dab5e0645a62e6db1809134fe1dac17faed363bea09f3867dddd988b7e`.
  The 13 unapplied ELF relocation records at linked places 0x536c, 0x5370, 0x53a4, 0x5384, 0x5388, 0x538c, 0x5390, 0x5378, 0x537c, 0x5394, 0x5398, 0x539c, 0x53a0 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x4eac).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x4eb8).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0x4ebc).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0x4ec0).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0x4ec4).
  Four-byte store: target `sp -0x7c`, value `0x0`, base register `r11`, raw displacement `-0x78` (`str` at 0x4ecc).
  Four-byte store: target `sp -0x80`, value `0x0`, base register `r11`, raw displacement `-0x7c` (`str` at 0x4ed4).
  Grouped accesses: 17 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x4ed8, `ldr` at 0x4ee4, `ldr` at 0x4ef4.
  Branch (`beq` at 0x4ee0): taken to 0x4fbc when `r2 == 0x0`, falls through to 0x4ee4 when `r2 != 0x0`.
  Grouped accesses: 2 4-byte load records with target `r2 +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x4ee8, `ldr` at 0x4ef8.
  Branch (`beq` at 0x4ef0): taken to 0x4fbc when `[r2] == 0x0`, falls through to 0x4ef4 when `[r2] != 0x0`.
  Call (`bl` at 0x4efc): (unknown callee), target 0x1e28, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e28, r0=`[r2]`, r1=`r1`, r2=`r2`, r3=`r2`.
  Branch (`bne` at 0x4f08): taken to 0x4f18 when `ret(linked ELF 0x1e28) != 0x0`, falls through to 0x4f0c when `ret(linked ELF 0x1e28) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5368`, base register `pc`, raw displacement `+0x454`, linked address 0x5368, basis `ELF load-bias-relative` (`ldr` at 0x4f0c).
  Grouped accesses: 2 4-byte store records with target `sp -0x114`, reported value `0xfffffed1` in every record, base register `r11`, raw displacement `-0x110`, including `str` at 0x4f10, `str` at 0x4fd0.
  Jump (`b` at 0x4f14): target 0x535c.
  Four-byte store: target `sp -0x110`, value `sp-256`, base register `r11`, raw displacement `-0x10c` (`str` at 0x4f1c).
  Four-byte store: target `sp -0x10c`, value `0x0`, base register `r11`, raw displacement `-0x108` (`str` at 0x4f24).
  Grouped accesses: 3 4-byte load records with target `sp -0x10c`, base register `r11`, raw displacement `-0x108`, including `ldr` at 0x4f28, `ldr` at 0x4f38, `ldr` at 0x4f44.
  Branch (`bhi` at 0x4f30): taken to 0x4f54 when `hi flags condition (operands unknown)`, falls through to 0x4f34 when `not (hi flags condition (operands unknown))`.
  Four-byte load: target `sp -0x110`, base register `r11`, raw displacement `-0x10c` (`ldr` at 0x4f34).
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r1`, raw displacement `+0x0`, including `str` at 0x4f40, `str` at 0x5210.
  Four-byte store: target `sp -0x10c`, value `unknown`, base register `r11`, raw displacement `-0x108` (`str` at 0x4f4c).
  Jump (`b` at 0x4f50): target 0x4f28.
  Grouped accesses: 7 4-byte load records with target `[sp-28] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x4f58, `ldr` at 0x4f98, `ldr` at 0x5004.
  Grouped accesses: 2 4-byte load records with target `[[sp-28]] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x4f5c, `ldr` at 0x4f9c.
  The analyzer invalidates unsupported effects (`mvn` at 0x4f64).
  The analyzer invalidates unsupported effects (`mvn` at 0x4f7c).
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r1`, raw displacement `+0x0`, including `ldr` at 0x4fac, `ldr` at 0x51cc.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r0`, raw displacement `+0x0`, including `str` at 0x4fb4, `str` at 0x51e0.
  Jump (`b` at 0x4fb8): target 0x4fd8.
  Call (`bl` at 0x4fbc): (unknown callee), target 0x1d44, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d44, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Branch (`bne` at 0x4fc8): taken to 0x4fd8 when `ret(linked ELF 0x1d44) != 0x0`, falls through to 0x4fcc when `ret(linked ELF 0x1d44) == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5368`, base register `pc`, raw displacement `+0x394`, linked address 0x5368, basis `ELF load-bias-relative` (`ldr` at 0x4fcc).
  Jump (`b` at 0x4fd4): target 0x535c.
  Four-byte load: target `ELF load bias (unknown) +0x536c`, base register `pc`, raw displacement `+0x38c`, linked address 0x536c, basis `ELF load-bias-relative` (`ldr` at 0x4fd8).
  ELF literal warning: linked 0x536c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x4fd8.
  Call (`bl` at 0x4fdc): (unknown callee), target 0x1d50, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d50, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 4 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x4fe0, `ldr` at 0x5068, `ldr` at 0x50b0.
  Branch (`bne` at 0x4fe8): taken to 0x5068 when `[sp-32] != 0x0`, falls through to 0x4fec when `[sp-32] == 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x7c`, reported value `0x1` in every record, base register `r11`, raw displacement `-0x78`, including `str` at 0x4ff0, `str` at 0x5078.
  Branch (`beq` at 0x4ffc): taken to 0x503c when `[sp-28] == 0x0`, falls through to 0x5000 when `[sp-28] != 0x0`.
  Branch (`beq` at 0x500c): taken to 0x503c when `[[sp-28]] == 0x0`, falls through to 0x5010 when `[[sp-28]] != 0x0`.
  Grouped accesses: 2 4-byte store records with target `sp -0x11c`, reported value `0x0` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0x5018, `str` at 0x50a0.
  Four-byte load: target `ELF load bias (unknown) +0x5370`, base register `pc`, raw displacement `+0x34c`, linked address 0x5370, basis `ELF load-bias-relative` (`ldr` at 0x501c).
  ELF literal warning: linked 0x5370, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x501c, `ldr` at 0x503c, `ldr` at 0x50a4.
  Grouped accesses: 2 4-byte store records with target `sp -0x118`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x4`, including `str` at 0x5020, `str` at 0x50a8.
  Four-byte load: target `ELF load bias (unknown) +0x5374`, base register `pc`, raw displacement `+0x340`, linked address 0x5374, basis `ELF load-bias-relative` (`ldr` at 0x502c).
  Four-byte load: target `ELF load bias (unknown) +0x5378`, base register `pc`, raw displacement `+0x340`, linked address 0x5378, basis `ELF load-bias-relative` (`ldr` at 0x5030).
  ELF literal warning: linked 0x5378, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x5030, `ldr` at 0x504c, `ldr` at 0x50b8.
  Call (`bl` at 0x5034): (unknown callee), target 0x1cc0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cc0, r0=`sp-120`, r1=`0x0`, r2=`0x2710`, r3=`unknown`.
  Jump (`b` at 0x5038): target 0x50ec.
  Four-byte load: target `ELF load bias (unknown) +0x5370`, base register `pc`, raw displacement `+0x32c`, linked address 0x5370, basis `ELF load-bias-relative` (`ldr` at 0x503c).
  Grouped accesses: 2 4-byte store records with target `sp -0x11c`, reported value `unknown` in every record, base register `sp`, raw displacement `+0x0`, including `str` at 0x5040, `str` at 0x50c8.
  Four-byte load: target `ELF load bias (unknown) +0x5374`, base register `pc`, raw displacement `+0x324`, linked address 0x5374, basis `ELF load-bias-relative` (`ldr` at 0x5048).
  Four-byte load: target `ELF load bias (unknown) +0x5378`, base register `pc`, raw displacement `+0x324`, linked address 0x5378, basis `ELF load-bias-relative` (`ldr` at 0x504c).
  Call (`bl` at 0x5054): (unknown callee), target 0x1d8c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d8c, r0=`0x0`, r1=`0x2710`, r2=`unknown`, r3=`0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x537c`, base register `pc`, raw displacement `+0x318`, linked address 0x537c, basis `ELF load-bias-relative` (`ldr` at 0x505c).
  ELF literal warning: linked 0x537c, value unknown (relocation, writable or unmapped bytes), 6 read records, including `ldr` at 0x505c, `ldr` at 0x50e4, `ldr` at 0x5114.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `ret(linked ELF 0x1d8c)` in every record, base register `r3`, raw displacement `+0x0`, including `str` at 0x5060, `str` at 0x50e8.
  Jump (`b` at 0x5064): target 0x50ec.
  The analyzer invalidates unsupported effects (`cmn` at 0x506c).
  Branch (`beq` at 0x5070): taken to 0x50ec when `Z == 1`, falls through to 0x5074 when `not (Z == 1)`.
  Branch (`beq` at 0x5084): taken to 0x50c4 when `[sp-28] == 0x0`, falls through to 0x5088 when `[sp-28] != 0x0`.
  Branch (`beq` at 0x5094): taken to 0x50c4 when `[[sp-28]] == 0x0`, falls through to 0x5098 when `[[sp-28]] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5370`, base register `pc`, raw displacement `+0x2c4`, linked address 0x5370, basis `ELF load-bias-relative` (`ldr` at 0x50a4).
  Four-byte load: target `ELF load bias (unknown) +0x5378`, base register `pc`, raw displacement `+0x2b8`, linked address 0x5378, basis `ELF load-bias-relative` (`ldr` at 0x50b8).
  Call (`bl` at 0x50bc): (unknown callee), target 0x1cc0, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1cc0, r0=`sp-120`, r1=`[sp-32]`, r2=`0x0`, r3=`unknown`.
  Jump (`b` at 0x50c0): target 0x50ec.
  Four-byte load: target `ELF load bias (unknown) +0x5370`, base register `pc`, raw displacement `+0x2a4`, linked address 0x5370, basis `ELF load-bias-relative` (`ldr` at 0x50c4).
  Four-byte load: target `ELF load bias (unknown) +0x5378`, base register `pc`, raw displacement `+0x29c`, linked address 0x5378, basis `ELF load-bias-relative` (`ldr` at 0x50d4).
  Call (`bl` at 0x50dc): (unknown callee), target 0x1d8c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d8c, r0=`[sp-32]`, r1=`0x0`, r2=`unknown`, r3=`0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x537c`, base register `pc`, raw displacement `+0x290`, linked address 0x537c, basis `ELF load-bias-relative` (`ldr` at 0x50e4).
  Four-byte load: target `sp -0x7c`, base register `r11`, raw displacement `-0x78` (`ldr` at 0x50ec).
  Branch (`beq` at 0x50f4): taken to 0x516c when `[sp-124] == 0x0`, falls through to 0x50f8 when `[sp-124] != 0x0`.
  Branch (`beq` at 0x5100): taken to 0x5114 when `[sp-28] == 0x0`, falls through to 0x5104 when `[sp-28] != 0x0`.
  Branch (`bne` at 0x5110): taken to 0x5128 when `[[sp-28]] != 0x0`, falls through to 0x5114 when `[[sp-28]] == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x537c`, base register `pc`, raw displacement `+0x260`, linked address 0x537c, basis `ELF load-bias-relative` (`ldr` at 0x5114).
  Grouped accesses: 10 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0x5118, `ldr` at 0x5170, `ldr` at 0x51b0.
  Branch (`bne` at 0x5120): taken to 0x5128 when `? != 0x0`, falls through to 0x5124 when `? == 0x0`.
  Jump (`b` at 0x5124): target 0x5154.
  Branch (`beq` at 0x5130): taken to 0x516c when `[sp-28] == 0x0`, falls through to 0x5134 when `[sp-28] != 0x0`.
  Branch (`beq` at 0x5140): taken to 0x516c when `[[sp-28]] == 0x0`, falls through to 0x5144 when `[[sp-28]] != 0x0`.
  Four-byte load: target `sp -0x28`, base register `r11`, raw displacement `-0x24` (`ldr` at 0x5144).
  The analyzer invalidates unsupported effects (`cmn` at 0x5148).
  Branch (`bne` at 0x514c): taken to 0x5154 when `Z == 0`, falls through to 0x5150 when `not (Z == 0)`.
  Jump (`b` at 0x5150): target 0x516c.
  Grouped accesses: 5 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x5154, `ldr` at 0x5290, `ldr` at 0x52b0.
  Grouped accesses: 2 4-byte store records with target `[sp-24] +0x0`, reported value `0x0` in every record, base register `r3`, raw displacement `+0x0`, including `str` at 0x515c, `str` at 0x52b8.
  Four-byte load: target `ELF load bias (unknown) +0x5380`, base register `pc`, raw displacement `+0x218`, linked address 0x5380, basis `ELF load-bias-relative` (`ldr` at 0x5160).
  Grouped accesses: 2 4-byte store records with target `sp -0x80`, reported value `0xfffffd94` in every record, base register `r11`, raw displacement `-0x7c`, including `str` at 0x5164, `str` at 0x52c0.
  Jump (`b` at 0x5168): target 0x5324.
  Four-byte load: target `ELF load bias (unknown) +0x5384`, base register `pc`, raw displacement `+0x210`, linked address 0x5384, basis `ELF load-bias-relative` (`ldr` at 0x516c).
  ELF literal warning: linked 0x5384, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x516c, `ldr` at 0x5178, `ldr` at 0x5268.
  Four-byte store: target `sp -0x24`, value `unknown`, base register `r11`, raw displacement `-0x20` (`str` at 0x5174).
  Four-byte load: target `ELF load bias (unknown) +0x5384`, base register `pc`, raw displacement `+0x204`, linked address 0x5384, basis `ELF load-bias-relative` (`ldr` at 0x5178).
  Grouped accesses: 7 4-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x0`, including `str` at 0x5180, `str` at 0x518c, `str` at 0x5198.
  Four-byte load: target `ELF load bias (unknown) +0x5388`, base register `pc`, raw displacement `+0x1fc`, linked address 0x5388, basis `ELF load-bias-relative` (`ldr` at 0x5184).
  ELF literal warning: linked 0x5388, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x5184, `ldr` at 0x5274, `ldr` at 0x5284.
  Four-byte load: target `ELF load bias (unknown) +0x538c`, base register `pc`, raw displacement `+0x1f4`, linked address 0x538c, basis `ELF load-bias-relative` (`ldr` at 0x5190).
  ELF literal warning: linked 0x538c, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x5190, `ldr` at 0x52f4.
  Four-byte load: target `ELF load bias (unknown) +0x5390`, base register `pc`, raw displacement `+0x1ec`, linked address 0x5390, basis `ELF load-bias-relative` (`ldr` at 0x519c).
  ELF literal warning: linked 0x5390, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0x519c, `ldr` at 0x525c.
  Four-byte store: target `(unknown effective address)`, value `0x2`, base register `r2`, raw displacement `+0x0` (`str` at 0x51a4).
  Four-byte load: target `ELF load bias (unknown) +0x5394`, base register `pc`, raw displacement `+0x1e4`, linked address 0x5394, basis `ELF load-bias-relative` (`ldr` at 0x51a8).
  ELF literal warning: linked 0x5394, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x51a8, `ldr` at 0x51c8, `ldr` at 0x51e4.
  Four-byte load: target `ELF load bias (unknown) +0x5398`, base register `pc`, raw displacement `+0x1e4`, linked address 0x5398, basis `ELF load-bias-relative` (`ldr` at 0x51ac).
  ELF literal warning: linked 0x5398, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0x51ac, `ldr` at 0x51e8, `ldr` at 0x5200.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, including `ldr` at 0x51b4, `ldr` at 0x51f0, `ldr` at 0x52f8.
  Branch (`beq` at 0x51bc): taken to 0x5218 when `? == ?`, falls through to 0x51c0 when `? != ?`.
  Four-byte load: target `ELF load bias (unknown) +0x539c`, base register `pc`, raw displacement `+0x1d4`, linked address 0x539c, basis `ELF load-bias-relative` (`ldr` at 0x51c0).
  ELF literal warning: linked 0x539c, value unknown (relocation, writable or unmapped bytes), 4 read records, including `ldr` at 0x51c0, `ldr` at 0x52a0, `ldr` at 0x52cc.
  Four-byte load: target `ELF load bias (unknown) +0x53a0`, base register `pc`, raw displacement `+0x1d4`, linked address 0x53a0, basis `ELF load-bias-relative` (`ldr` at 0x51c4).
  ELF literal warning: linked 0x53a0, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x51c4.
  Four-byte load: target `ELF load bias (unknown) +0x5394`, base register `pc`, raw displacement `+0x1c4`, linked address 0x5394, basis `ELF load-bias-relative` (`ldr` at 0x51c8).
  Four-byte store: target `(unknown effective address)`, value `unknown`, base register `r1`, raw displacement `+0x0` (`str` at 0x51d8).
  Four-byte load: target `(unknown effective address)`, base register `r12`, raw displacement `+0x0` (`ldr` at 0x51dc).
  Four-byte load: target `ELF load bias (unknown) +0x5394`, base register `pc`, raw displacement `+0x1a8`, linked address 0x5394, basis `ELF load-bias-relative` (`ldr` at 0x51e4).
  Four-byte load: target `ELF load bias (unknown) +0x5398`, base register `pc`, raw displacement `+0x1a8`, linked address 0x5398, basis `ELF load-bias-relative` (`ldr` at 0x51e8).
  Branch (`bne` at 0x51f8): taken to 0x525c when `? != ?`, falls through to 0x51fc when `? == ?`.
  Four-byte load: target `ELF load bias (unknown) +0x5394`, base register `pc`, raw displacement `+0x190`, linked address 0x5394, basis `ELF load-bias-relative` (`ldr` at 0x51fc).
  Four-byte load: target `ELF load bias (unknown) +0x5398`, base register `pc`, raw displacement `+0x190`, linked address 0x5398, basis `ELF load-bias-relative` (`ldr` at 0x5200).
  Jump (`b` at 0x5214): target 0x525c.
  Branch (`beq` at 0x5220): taken to 0x5258 when `[sp-28] == 0x0`, falls through to 0x5224 when `[sp-28] != 0x0`.
  Branch (`beq` at 0x5230): taken to 0x5258 when `[[sp-28]] == 0x0`, falls through to 0x5234 when `[[sp-28]] != 0x0`.
  Call (`bl` at 0x5250): (unknown callee), target 0x1dc8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1dc8, r0=`sp-256`, r1=`sp-256`, r2=`sp-120`, r3=`0x1`.
  Jump (`b` at 0x5254): target 0x525c.
  Call (`bl` at 0x5258): (unknown callee), target 0x1e34, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1e34, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x5390`, base register `pc`, raw displacement `+0x12c`, linked address 0x5390, basis `ELF load-bias-relative` (`ldr` at 0x525c).
  Four-byte load: target `ELF load bias (unknown) +0x5384`, base register `pc`, raw displacement `+0x114`, linked address 0x5384, basis `ELF load-bias-relative` (`ldr` at 0x5268).
  Four-byte load: target `sp -0x24`, base register `r11`, raw displacement `-0x20` (`ldr` at 0x526c).
  Four-byte store: target `(unknown effective address)`, value `[sp-36]`, base register `r2`, raw displacement `+0x0` (`str` at 0x5270).
  Four-byte load: target `ELF load bias (unknown) +0x5388`, base register `pc`, raw displacement `+0x10c`, linked address 0x5388, basis `ELF load-bias-relative` (`ldr` at 0x5274).
  Branch (`beq` at 0x5280): taken to 0x52a0 when `? == 0x0`, falls through to 0x5284 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5388`, base register `pc`, raw displacement `+0xfc`, linked address 0x5388, basis `ELF load-bias-relative` (`ldr` at 0x5284).
  Four-byte store: target `sp -0x80`, value `unknown`, base register `r11`, raw displacement `-0x7c` (`str` at 0x528c).
  Four-byte store: target `[sp-24] +0x0`, value `0x0`, base register `r2`, raw displacement `+0x0` (`str` at 0x5298).
  Jump (`b` at 0x529c): target 0x5324.
  Four-byte load: target `ELF load bias (unknown) +0x539c`, base register `pc`, raw displacement `+0xf4`, linked address 0x539c, basis `ELF load-bias-relative` (`ldr` at 0x52a0).
  Branch (`bne` at 0x52ac): taken to 0x52c8 when `? != 0x0`, falls through to 0x52b0 when `? == 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x5380`, base register `pc`, raw displacement `+0xbc`, linked address 0x5380, basis `ELF load-bias-relative` (`ldr` at 0x52bc).
  Jump (`b` at 0x52c4): target 0x5324.
  Four-byte load: target `ELF load bias (unknown) +0x539c`, base register `pc`, raw displacement `+0xc8`, linked address 0x539c, basis `ELF load-bias-relative` (`ldr` at 0x52cc).
  Four-byte store: target `[sp-24] +0x0`, value `unknown`, base register `r2`, raw displacement `+0x0` (`str` at 0x52d4).
  Four-byte load: target `ELF load bias (unknown) +0x539c`, base register `pc`, raw displacement `+0xbc`, linked address 0x539c, basis `ELF load-bias-relative` (`ldr` at 0x52d8).
  Branch (`beq` at 0x52ec): taken to 0x5300 when `[sp-28] == 0x0`, falls through to 0x52f0 when `[sp-28] != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x538c`, base register `pc`, raw displacement `+0x90`, linked address 0x538c, basis `ELF load-bias-relative` (`ldr` at 0x52f4).
  Four-byte store: target `[sp-28] +0x0`, value `unknown`, base register `r3`, raw displacement `+0x0` (`str` at 0x52fc).
  Four-byte load: target `sp +0x0`, base register `r11`, raw displacement `+0x4` (`ldr` at 0x5300).
  Branch (`beq` at 0x5308): taken to 0x5324 when `[sp] == 0x0`, falls through to 0x530c when `[sp] != 0x0`.
  Four-byte load: target `[sp-24] +0x0`, base register `r3`, raw displacement `+0x0` (`ldr` at 0x5310).
  Byte load: target `[[sp-24]] +0x16`, base register `r3`, raw displacement `+0x16` (`ldrb` at 0x5314).
  Branch (`beq` at 0x531c): taken to 0x5324 when `[[[sp-24]]+22]:1 == 0x1`, falls through to 0x5320 when `[[[sp-24]]+22]:1 != 0x1`.
  Jump (`b` at 0x5320): target 0x50ec.
  Four-byte load: target `ELF load bias (unknown) +0x537c`, base register `pc`, raw displacement `+0x50`, linked address 0x537c, basis `ELF load-bias-relative` (`ldr` at 0x5324).
  Branch (`beq` at 0x5330): taken to 0x534c when `? == 0x0`, falls through to 0x5334 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0x537c`, base register `pc`, raw displacement `+0x40`, linked address 0x537c, basis `ELF load-bias-relative` (`ldr` at 0x5334).
  Call (`bl` at 0x533c): (unknown callee), target 0x1d08, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d08, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0x537c`, base register `pc`, raw displacement `+0x34`, linked address 0x537c, basis `ELF load-bias-relative` (`ldr` at 0x5340).
  Four-byte load: target `ELF load bias (unknown) +0x53a4`, base register `pc`, raw displacement `+0x50`, linked address 0x53a4, basis `ELF load-bias-relative` (`ldr` at 0x534c).
  ELF literal warning: linked 0x53a4, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0x534c.
  Call (`bl` at 0x5350): (unknown callee), target 0x1d50, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x1d50, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `sp -0x80`, base register `r11`, raw displacement `-0x7c` (`ldr` at 0x5354).
  Four-byte store: target `sp -0x114`, value `[sp-128]`, base register `r11`, raw displacement `-0x110` (`str` at 0x5358).
  Four-byte load: target `sp -0x114`, base register `r11`, raw displacement `-0x110` (`ldr` at 0x535c).
  The exit (`ldm` at 0x5364) reports r0 = `unknown`.
  Unconditional return boundary (`ldm` at 0x5364): the modeled local edges stop before the trailing region 0x5368 to 0x53a8 (exclusive).
  Undecoded range: 0x5368 to 0x536c (exclusive), 4 bytes `d1feffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x536c to 0x537c are not reached by the modeled local edges from the entry, and their effects are not listed.
  Undecoded range: 0x5380 to 0x5384 (exclusive), 4 bytes `94fdffff`. It lies in the trailing region after the modeled return boundary and supplies no decoded effects.
  The decoded instructions from 0x5384 to 0x53a4 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `l1d_ppperr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xbdfc to 0xbe48 (exclusive), 76 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `9edc70e8ad325776078c64242ec5dd2751d8152dbe1027816601b02b8647c65d`.
  The 2 unapplied ELF relocation records at linked places 0xbe44, 0xbe40 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0xbe00).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0xbe0c).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0xbe10).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0xbe14).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0xbe18).
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xbe1c).
  Call (`bl` at 0xbe20): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`r1`, r1=`r1`, r2=`r2`, r3=`r3`.
  Four-byte load: target `ELF load bias (unknown) +0xbe40`, base register `pc`, raw displacement `+0x10`, linked address 0xbe40, basis `ELF load-bias-relative` (`ldr` at 0xbe28).
  ELF literal warning: linked 0xbe40, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbe28.
  Four-byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0` (`ldr` at 0xbe2c).
  Four-byte load: target `ELF load bias (unknown) +0xbe44`, base register `pc`, raw displacement `+0xc`, linked address 0xbe44, basis `ELF load-bias-relative` (`ldr` at 0xbe30).
  ELF literal warning: linked 0xbe44, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbe30.
  Call (`bl` at 0xbe34): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0x37cc)`, r3=`unknown`.
  Recorded r0 at the return is `ret(linked ELF 0x34cc)` (`ldm` at 0xbe3c).
  Unconditional return boundary (`ldm` at 0xbe3c): the modeled local edges stop before the trailing region 0xbe40 to 0xbe48 (exclusive).
  The decoded instructions from 0xbe40 to 0xbe44 are not reached by the modeled local edges from the entry, and their effects are not listed.

- `sn1ScanConvertName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0x4038 to 0x428c (exclusive), 596 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `d1a6ac14b744c9788d91e55b411a452f7d845357bf6a2f2d3d4a2904893058fd`.
  The analyzer retains no ELF relocation records for this span.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0xc`, implicit access (`push` at 0x403c).
  Four-byte store: target `sp -0x14`, value `r0`, base register `r11`, raw displacement `-0x10` (`str` at 0x4048).
  Four-byte store: target `sp -0x18`, value `r1`, base register `r11`, raw displacement `-0x14` (`str` at 0x404c).
  Four-byte store: target `sp -0x1c`, value `r2`, base register `r11`, raw displacement `-0x18` (`str` at 0x4050).
  Four-byte store: target `sp -0x20`, value `r3`, base register `r11`, raw displacement `-0x1c` (`str` at 0x4054).
  Grouped accesses: 2 4-byte load records with target `sp -0x14`, base register `r11`, raw displacement `-0x10`, including `ldr` at 0x4058, `ldr` at 0x4080.
  Call (`bl` at 0x405c): (unknown callee), target 0x36ac, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x36ac, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x4068): taken to 0x4078 when `ret(linked ELF 0x36ac) == 0x6`, falls through to 0x406c when `ret(linked ELF 0x36ac) != 0x6`.
  Grouped accesses: 3 4-byte store records with target `sp -0x3c`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x38`, including `str` at 0x4070, `str` at 0x40b4, `str` at 0x4198.
  Jump (`b` at 0x4074): target 0x4280.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `0x0` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x407c, `str` at 0x4168.
  Four-byte store: target `sp -0x24`, value `[sp-20]`, base register `r11`, raw displacement `-0x20` (`str` at 0x4084).
  Grouped accesses: 8 4-byte load records with target `sp -0x30`, base register `r11`, raw displacement `-0x2c`, including `ldr` at 0x4088, `ldr` at 0x40c0, `ldr` at 0x40dc.
  Branch (`bhi` at 0x4090): taken to 0x40f8 when `hi flags condition (operands unknown)`, falls through to 0x4094 when `not (hi flags condition (operands unknown))`.
  Grouped accesses: 8 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0x4094, `ldr` at 0x40d0, `ldr` at 0x40e8.
  Byte load: target `(unknown effective address)`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0x4098).
  Call (`bl` at 0x40a0): (unknown callee), target 0x37fc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x40ac): taken to 0x40bc when `ret(linked ELF 0x37fc) != 0x0`, falls through to 0x40b0 when `ret(linked ELF 0x37fc) == 0x0`.
  Jump (`b` at 0x40b8): target 0x4280.
  The analyzer invalidates unsupported effects (`mvn` at 0x40bc).
  Grouped accesses: 2 1-byte load records with target `[sp-36] +0x0`, base register `r3`, raw displacement `+0x0`, including `ldrb` at 0x40d4, `ldrb` at 0x41b8.
  Grouped accesses: 2 1-byte store records with target `(unknown effective address)`, reported value `([[sp-36]]:1 & 0xff)` in every record, base register `r2`, raw displacement `+0x0`, including `strb` at 0x40d8, `strb` at 0x41bc.
  Grouped accesses: 2 4-byte store records with target `sp -0x30`, reported value `[sp-48]+1` in every record, base register `r11`, raw displacement `-0x2c`, including `str` at 0x40e4, `str` at 0x41c8.
  Grouped accesses: 3 4-byte store records with target `sp -0x24`, reported value `[sp-36]+1` in every record, base register `r11`, raw displacement `-0x20`, including `str` at 0x40f0, `str` at 0x4160, `str` at 0x41d4.
  Jump (`b` at 0x40f4): target 0x4088.
  The analyzer invalidates unsupported effects (`mvn` at 0x40f8).
  Grouped accesses: 2 1-byte store records with target `(unknown effective address)`, reported value `0x0` in every record, base register `r2`, raw displacement `+0x0`, including `strb` at 0x4110, `strb` at 0x41f4.
  Call (`bl` at 0x411c): (unknown callee), target 0x385c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x385c, r0=`sp-44`, r1=`sp-16`, r2=`unknown`, r3=`sp-44`.
  Four-byte store: target `sp -0x34`, value `ret(linked ELF 0x385c)`, base register `r11`, raw displacement `-0x30` (`str` at 0x4120).
  Grouped accesses: 2 4-byte load records with target `sp -0x18`, base register `r11`, raw displacement `-0x14`, including `ldr` at 0x4124, `ldr` at 0x4130.
  Branch (`beq` at 0x412c): taken to 0x413c when `[sp-24] == 0x0`, falls through to 0x4130 when `[sp-24] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `sp -0x34`, base register `r11`, raw displacement `-0x30`, including `ldr` at 0x4134, `ldr` at 0x4220.
  Four-byte store: target `[sp-24] +0x0`, value `ret(linked ELF 0x385c)`, base register `r2`, raw displacement `+0x0` (`str` at 0x4138).
  Grouped accesses: 2 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0x413c, `ldr` at 0x4148.
  Branch (`beq` at 0x4144): taken to 0x4158 when `[sp-32] == 0x0`, falls through to 0x4148 when `[sp-32] != 0x0`.
  Byte load: target `[sp-36] +0x0`, base register `r2`, raw displacement `+0x0` (`ldrb` at 0x4150).
  Byte store: target `[sp-32] +0x0`, value `([[sp-36]]:1 & 0xff)`, base register `r3`, raw displacement `+0x0` (`strb` at 0x4154).
  Branch (`bhi` at 0x4174): taken to 0x41dc when `hi flags condition (operands unknown)`, falls through to 0x4178 when `not (hi flags condition (operands unknown))`.
  Byte load: target `[sp-36] +0x1`, base register `r3`, raw displacement `+0x0` (`ldrb` at 0x417c).
  Call (`bl` at 0x4184): (unknown callee), target 0x37fc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37fc, r0=`[[sp-36]+1]:1`, r1=`unknown`, r2=`unknown`, r3=`[[sp-36]+1]:1`.
  Branch (`bne` at 0x4190): taken to 0x41a0 when `ret(linked ELF 0x37fc) != 0x0`, falls through to 0x4194 when `ret(linked ELF 0x37fc) == 0x0`.
  Jump (`b` at 0x419c): target 0x4280.
  The analyzer invalidates unsupported effects (`mvn` at 0x41a0).
  Jump (`b` at 0x41d8): target 0x416c.
  The analyzer invalidates unsupported effects (`mvn` at 0x41dc).
  Call (`bl` at 0x4200): (unknown callee), target 0x385c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x385c, r0=`sp-44`, r1=`sp-16`, r2=`unknown`, r3=`sp-44`.
  Four-byte store: target `sp -0x38`, value `ret(linked ELF 0x385c)`, base register `r11`, raw displacement `-0x34` (`str` at 0x4204).
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0x4208, `ldr` at 0x4214.
  Branch (`beq` at 0x4210): taken to 0x4220 when `[sp-28] == 0x0`, falls through to 0x4214 when `[sp-28] != 0x0`.
  Grouped accesses: 2 4-byte load records with target `sp -0x38`, base register `r11`, raw displacement `-0x34`, including `ldr` at 0x4218, `ldr` at 0x4244.
  Four-byte store: target `[sp-28] +0x0`, value `ret(linked ELF 0x385c)`, base register `r2`, raw displacement `+0x0` (`str` at 0x421c).
  Grouped accesses: 6 4-byte store records with target `sp -0x40`, reported value `unknown` in every record, base register `r11`, raw displacement `-0x3c`, including `str` at 0x4228, `str` at 0x4234, `str` at 0x4240.
  Grouped accesses: 6 4-byte load records with target `sp -0x40`, base register `r11`, raw displacement `-0x3c`, including `ldr` at 0x422c, `ldr` at 0x4238, `ldr` at 0x424c.
  Four-byte store: target `sp -0x3c`, value `[sp-64]`, base register `r11`, raw displacement `-0x38` (`str` at 0x427c).
  Four-byte load: target `sp -0x3c`, base register `r11`, raw displacement `-0x38` (`ldr` at 0x4280).
  `ldm` at 0x4288 has recorded r0 = `unknown`.

- `l1d_read_packet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#4`, version `1.44.0`, inner `/stand/sysco/lib/libapi.armlinux.so`.
  Inferred analysis span: 0xb938 to 0xbb64 (exclusive), 556 bytes. Role: unknown.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x53d154, 3817724 bytes, SHA256 `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
  Cramfs: package offset 0xbb728. Inner ELF: 63600 bytes, SHA256 `b9a9b7b143ef01b81962825f2bff5aafff3657213d0b3048c72c4f111a83e40d`.
  Read window: SHA256 `2b50b7ad508a62e3ec6298efb623fcb804fcd6d96b0aa5106afe599356026bd6`.
  The 15 unapplied ELF relocation records at linked places 0xbb28, 0xbb30, 0xbb34, 0xbb3c, 0xbb40, 0xbb44, 0xbb48, 0xbb4c, 0xbb50, 0xbb54, 0xbb58, 0xbb5c, 0xbb60, 0xbb2c, 0xbb38 retain unknown values.
  Four-byte store: target `sp -0x4`, value `unknown`, base register `sp`, raw displacement `+0x10`, implicit access (`push` at 0xb93c).
  Four-byte store: target `sp -0x18`, value `r0`, base register `r11`, raw displacement `-0x14` (`str` at 0xb948).
  Four-byte store: target `sp -0x1c`, value `r1`, base register `r11`, raw displacement `-0x18` (`str` at 0xb94c).
  Four-byte load: target `ELF load bias (unknown) +0xbb28`, base register `pc`, raw displacement `+0x1d0`, linked address 0xbb28, basis `ELF load-bias-relative` (`ldr` at 0xb950).
  ELF literal warning: linked 0xbb28, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xb950, `ldr` at 0xbb00.
  Grouped accesses: 14 4-byte load records with target `(unknown effective address)`, base register `r3`, raw displacement `+0x0`, including `ldr` at 0xb954, `ldr` at 0xb964, `ldr` at 0xb9a8.
  Branch (`beq` at 0xb95c): taken to 0xb978 when `? == 0x0`, falls through to 0xb960 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0x1c4`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xb960).
  ELF literal warning: linked 0xbb2c, value unknown (relocation, writable or unmapped bytes), 7 read records, including `ldr` at 0xb960, `ldr` at 0xb9dc, `ldr` at 0xba18.
  Four-byte load: target `ELF load bias (unknown) +0xbb30`, base register `pc`, raw displacement `+0x1c0`, linked address 0xbb30, basis `ELF load-bias-relative` (`ldr` at 0xb968).
  ELF literal warning: linked 0xbb30, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb968.
  Four-byte load: target `sp -0x18`, base register `r11`, raw displacement `-0x14` (`ldr` at 0xb96c).
  Grouped accesses: 2 4-byte load records with target `sp -0x1c`, base register `r11`, raw displacement `-0x18`, including `ldr` at 0xb970, `ldr` at 0xb984.
  Call (`bl` at 0xb974): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`r0`, r3=`r1`.
  Four-byte load: target `ELF load bias (unknown) +0xbb34`, base register `pc`, raw displacement `+0x1b0`, linked address 0xbb34, basis `ELF load-bias-relative` (`ldr` at 0xb97c).
  ELF literal warning: linked 0xbb34, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb97c.
  Four-byte load: target `ELF load bias (unknown) +0xbb38`, base register `pc`, raw displacement `+0x1b0`, linked address 0xbb38, basis `ELF load-bias-relative` (`ldr` at 0xb980).
  ELF literal warning: linked 0xbb38, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xb980.
  Call (`bl` at 0xb988): (unknown callee), target 0x38c8, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x38c8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`sp-32`.
  Four-byte store: target `sp -0x24`, value `ret(linked ELF 0x38c8)`, base register `r11`, raw displacement `-0x20` (`str` at 0xb990).
  Grouped accesses: 4 4-byte load records with target `sp -0x24`, base register `r11`, raw displacement `-0x20`, including `ldr` at 0xb994, `ldr` at 0xb9a4, `ldr` at 0xb9f8.
  Branch (`beq` at 0xb99c): taken to 0xba2c when `ret(linked ELF 0x38c8) == 0x0`, falls through to 0xb9a0 when `ret(linked ELF 0x38c8) != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbb3c`, base register `pc`, raw displacement `+0x194`, linked address 0xbb3c, basis `ELF load-bias-relative` (`ldr` at 0xb9a0).
  ELF literal warning: linked 0xbb3c, value unknown (relocation, writable or unmapped bytes), 3 read records, including `ldr` at 0xb9a0, `ldr` at 0xb9f4, `ldr` at 0xba54.
  Branch (`bne` at 0xb9b0): taken to 0xb9cc when `ret(linked ELF 0x38c8) != ?`, falls through to 0xb9b4 when `ret(linked ELF 0x38c8) == ?`.
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0x184`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xb9b4).
  ELF literal warning: linked 0xbb40, value unknown (relocation, writable or unmapped bytes), 8 read records, including `ldr` at 0xb9b4, `ldr` at 0xb9b8, `ldr` at 0xb9cc.
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0x180`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xb9b8).
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `r2`, raw displacement `+0x0`, including `str` at 0xb9c4, `str` at 0xb9fc.
  Jump (`b` at 0xb9c8): target 0xbb00.
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0x16c`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xb9cc).
  Branch (`ble` at 0xb9d8): taken to 0xb9f4 when `? <= 0x1 (signed, 32-bit)`, falls through to 0xb9dc when `? > 0x1 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0x148`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xb9dc).
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0x158`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xb9e0).
  Four-byte load: target `ELF load bias (unknown) +0xbb44`, base register `pc`, raw displacement `+0x154`, linked address 0xbb44, basis `ELF load-bias-relative` (`ldr` at 0xb9e8).
  ELF literal warning: linked 0xbb44, value unknown (relocation, writable or unmapped bytes), 2 read records, including `ldr` at 0xb9e8, `ldr` at 0xba48.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `r2`, raw displacement `+0x0`, including `ldr` at 0xb9ec, `ldr` at 0xba4c.
  Call (`bl` at 0xb9f0): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xbb3c`, base register `pc`, raw displacement `+0x140`, linked address 0xbb3c, basis `ELF load-bias-relative` (`ldr` at 0xb9f4).
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0x138`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xba00).
  Four-byte store: target `(unknown effective address)`, value `0x1`, base register `r2`, raw displacement `+0x0` (`str` at 0xba08).
  Call (`bl` at 0xba10): (unknown callee), target 0x37cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x37cc, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`0x1`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0x10c`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xba18).
  Four-byte load: target `ELF load bias (unknown) +0xbb48`, base register `pc`, raw displacement `+0x120`, linked address 0xbb48, basis `ELF load-bias-relative` (`ldr` at 0xba20).
  ELF literal warning: linked 0xbb48, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xba20.
  Call (`bl` at 0xba24): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0x37cc)`, r3=`unknown`.
  Jump (`b` at 0xba28): target 0xbb00.
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0x10c`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xba2c).
  Branch (`ble` at 0xba38): taken to 0xba54 when `? <= 0x1 (signed, 32-bit)`, falls through to 0xba3c when `? > 0x1 (signed, 32-bit)`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0xe8`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xba3c).
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0xf8`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xba40).
  Four-byte load: target `ELF load bias (unknown) +0xbb44`, base register `pc`, raw displacement `+0xf4`, linked address 0xbb44, basis `ELF load-bias-relative` (`ldr` at 0xba48).
  Call (`bl` at 0xba50): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xbb3c`, base register `pc`, raw displacement `+0xe0`, linked address 0xbb3c, basis `ELF load-bias-relative` (`ldr` at 0xba54).
  Four-byte load: target `ELF load bias (unknown) +0xbb40`, base register `pc`, raw displacement `+0xe0`, linked address 0xbb40, basis `ELF load-bias-relative` (`ldr` at 0xba58).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r2`, raw displacement `+0x0` (`str` at 0xba60).
  Four-byte store: target `(unknown effective address)`, value `0x0`, base register `r1`, raw displacement `+0x0` (`str` at 0xba68).
  Grouped accesses: 4 4-byte load records with target `sp -0x20`, base register `r11`, raw displacement `-0x1c`, including `ldr` at 0xba6c, `ldr` at 0xba84, `ldr` at 0xbad0.
  Byte load: target `[sp-32] +0x13`, base register `r3`, raw displacement `+0x13` (`ldrb` at 0xba70).
  Four-byte store: target `sp -0x28`, value `[[sp-32]+19]:1`, base register `r11`, raw displacement `-0x24` (`str` at 0xba74).
  Grouped accesses: 2 4-byte load records with target `sp -0x28`, base register `r11`, raw displacement `-0x24`, including `ldr` at 0xba78, `ldr` at 0xba94.
  Branch (`beq` at 0xba80): taken to 0xbab8 when `[[sp-32]+19]:1 == 0x0`, falls through to 0xba84 when `[[sp-32]+19]:1 != 0x0`.
  Four-byte load: target `[sp-32] +0x1c`, base register `r3`, raw displacement `+0x1c` (`ldr` at 0xba88).
  Call (`bl` at 0xba8c): (unknown callee), target 0xbe74, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0xbe74, r0=`[[sp-32]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]`.
  Call (`bl` at 0xba98): (unknown callee), target 0x3790, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3790, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0x84`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xbaa0).
  Four-byte load: target `ELF load bias (unknown) +0xbb4c`, base register `pc`, raw displacement `+0x9c`, linked address 0xbb4c, basis `ELF load-bias-relative` (`ldr` at 0xbaa8).
  ELF literal warning: linked 0xbb4c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbaa8.
  Call (`bl` at 0xbab4): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`ret(linked ELF 0xbe74)`, r3=`ret(linked ELF 0x3790)`.
  Four-byte load: target `ELF load bias (unknown) +0xbb50`, base register `pc`, raw displacement `+0x90`, linked address 0xbb50, basis `ELF load-bias-relative` (`ldr` at 0xbab8).
  ELF literal warning: linked 0xbb50, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbab8.
  Branch (`beq` at 0xbac4): taken to 0xbaf0 when `? == 0x0`, falls through to 0xbac8 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbb54`, base register `pc`, raw displacement `+0x80`, linked address 0xbb54, basis `ELF load-bias-relative` (`ldr` at 0xbacc).
  ELF literal warning: linked 0xbb54, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbacc.
  Call (`bl` at 0xbad8): (unknown callee), target 0x3694, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x3694, r0=`unknown`, r1=`[sp-32]`, r2=`sp-120`, r3=`sp-120`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0x48`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xbadc).
  Four-byte load: target `ELF load bias (unknown) +0xbb58`, base register `pc`, raw displacement `+0x68`, linked address 0xbb58, basis `ELF load-bias-relative` (`ldr` at 0xbae8).
  ELF literal warning: linked 0xbb58, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbae8.
  Call (`bl` at 0xbaec): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`sp-120`, r3=`unknown`.
  Four-byte load: target `ELF load bias (unknown) +0xbb5c`, base register `pc`, raw displacement `+0x64`, linked address 0xbb5c, basis `ELF load-bias-relative` (`ldr` at 0xbaf0).
  ELF literal warning: linked 0xbb5c, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbaf0.
  Four-byte store: target `(unknown effective address)`, value `[sp-32]`, base register `r2`, raw displacement `+0x0` (`str` at 0xbaf8).
  Call (`bl` at 0xbafc): (unknown callee), target 0x340c, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x340c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]`.
  Four-byte load: target `ELF load bias (unknown) +0xbb28`, base register `pc`, raw displacement `+0x20`, linked address 0xbb28, basis `ELF load-bias-relative` (`ldr` at 0xbb00).
  Branch (`beq` at 0xbb0c): taken to 0xbb20 when `? == 0x0`, falls through to 0xbb10 when `? != 0x0`.
  Four-byte load: target `ELF load bias (unknown) +0xbb2c`, base register `pc`, raw displacement `+0x14`, linked address 0xbb2c, basis `ELF load-bias-relative` (`ldr` at 0xbb10).
  Four-byte load: target `ELF load bias (unknown) +0xbb60`, base register `pc`, raw displacement `+0x40`, linked address 0xbb60, basis `ELF load-bias-relative` (`ldr` at 0xbb18).
  ELF literal warning: linked 0xbb60, value unknown (relocation, writable or unmapped bytes), 1 read record, including `ldr` at 0xbb18.
  Call (`bl` at 0xbb1c): (unknown callee), target 0x34cc, basis `linked ELF virtual address`, execution state `arm`, canonical instruction target 0x34cc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  At the return (`ldm` at 0xbb24), r0 is `unknown`.
  Unconditional return boundary (`ldm` at 0xbb24): the modeled local edges stop before the trailing region 0xbb28 to 0xbb64 (exclusive).
  The decoded instructions from 0xbb28 to 0xbb60 are not reached by the modeled local edges from the entry, and their effects are not listed.
