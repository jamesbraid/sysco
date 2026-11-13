# ARM TDR scan and link transport in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/tdr-scan-link-transport-facts.txt) retain all tool sections.

## `JChip_ir_load`

- `JChip_ir_load`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0104 to 0xe0540.
  Established. A word assignment clears `0x2973a8` (`str` at 0xe0384).
  The unsigned comparison `[0x2973a8] >= [sp-32]` takes `0xe03d0`, while a smaller value falls through (`bhs` at 0xe0398).
  The unnamed `0xd5e9c` call carries `0x0`, `[sp-60]`, `[sp-36]`, `[sp-56]` (`bl` at 0xe0408).
  Instruction-register scan framing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe0114 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe0118 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe011c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe0124 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xe0128 | load | 4 | `abs 0xe04f8` | `-` |
| `ldr` at 0xe012c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0138 | load | 4 | `abs 0xe04fc` | `-` |
| `ldr` at 0xe013c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0140 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0144 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe014c | load | 4 | `abs 0xe0500` | `-` |
| `str` at 0xe0150 | store | 4 | `sp -0x38` | `0x2973ac` |
| `ldr` at 0xe0154 | load | 4 | `abs 0xe0504` | `-` |
| `str` at 0xe0158 | store | 4 | `sp -0x3c` | `0x29742c` |
| `ldr` at 0xe015c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe0168 | store | 4 | `sp -0x30` | `ret(0xdefac)` |
| `ldr` at 0xe016c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0178 | load | 4 | `abs 0xe0508` | `-` |
| `ldr` at 0xe017c | load | 4 | `abs 0xe050c` | `-` |
| `ldr` at 0xe0180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0188 | load | 4 | `abs 0xe0508` | `-` |
| `str` at 0xe018c | store | 4 | `sp -0x58` | `0x29c6f8` |
| `ldr` at 0xe0194 | load | 4 | `abs 0xe0510` | `-` |
| `ldr` at 0xe0198 | load | 4 | `abs 0xe0514` | `-` |
| `ldr` at 0xe01a4 | load | 4 | `abs 0xe0518` | `-` |
| `str` at 0xe01a8 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xe01b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe01c4 | load | 4 | `abs 0xe051c` | `-` |
| `str` at 0xe01c8 | store | 4 | `sp -0x48` | `0xfffff7c5` |
| `ldr` at 0xe01d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe01d4 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrh` at 0xe01d8 | load | 2 | `[[sp-48]] +0xe` | `-` |
| `str` at 0xe01ec | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0xe01f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe01f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe01fc | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrb` at 0xe0200 | load | 1 | `[[sp-48]] +0xc` | `-` |
| `str` at 0xe0204 | store | 4 | `sp -0x28` | `[[[sp-48]]+12]:1` |
| `ldr` at 0xe0208 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe020c | store | 4 | `sp -0x2c` | `[[[sp-48]]+12]:1` |
| `ldr` at 0xe0214 | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0xe0218 | load | 1 | `[sp-48] +0x21` | `-` |
| `str` at 0xe021c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe0220 | load | 4 | `abs 0xe0520` | `-` |
| `ldr` at 0xe0224 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xe0228 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xe0248 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe0250 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe0258 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe025c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe0260 | load | 4 | `abs 0xe0520` | `-` |
| `ldr` at 0xe0264 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xe0268 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xe0288 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe0290 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe0298 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe029c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe02a0 | load | 4 | `abs 0xe0520` | `-` |
| `ldr` at 0xe02a4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xe02a8 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xe02c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe02cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe02d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe02d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe02dc | load | 4 | `[sp-48] +0x28` | `-` |
| `ldr` at 0xe02e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe02e8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe02ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe02fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0300 | load | 4 | `abs 0xe0508` | `-` |
| `ldr` at 0xe0314 | load | 4 | `abs 0xe0508` | `-` |
| `str` at 0xe0318 | store | 4 | `sp -0x58` | `0x29c6f8` |
| `ldr` at 0xe0320 | load | 4 | `abs 0xe0510` | `-` |
| `ldr` at 0xe0324 | load | 4 | `abs 0xe0524` | `-` |
| `ldr` at 0xe0330 | load | 4 | `abs 0xe0528` | `-` |
| `str` at 0xe0334 | store | 4 | `sp -0x48` | `0xfffff7ca` |
| `ldr` at 0xe033c | load | 4 | `abs 0xe04f8` | `-` |
| `ldr` at 0xe0340 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe034c | load | 4 | `abs 0xe052c` | `-` |
| `ldr` at 0xe0350 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0358 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe036c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0378 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe037c | load | 4 | `abs 0xe0530` | `-` |
| `str` at 0xe0384 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xe0388 | load | 4 | `abs 0xe0530` | `-` |
| `ldr` at 0xe038c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0390 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe039c | load | 4 | `abs 0xe0530` | `-` |
| `ldr` at 0xe03a0 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe03a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe03b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe03b8 | load | 4 | `abs 0xe0530` | `-` |
| `ldr` at 0xe03bc | load | 4 | `abs 0xe0530` | `-` |
| `ldr` at 0xe03c0 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe03c8 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xe03d0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe03d4 | load | 4 | `sp -0x34` | `-` |
| `ldrh` at 0xe03d8 | load | 2 | `[sp-52] +0x4` | `-` |
| `str` at 0xe03dc | store | 4 | `[sp-60] +0x0` | `[[sp-52]+4]:2` |
| `ldr` at 0xe03e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe03f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe03f4 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xe03fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0400 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe0404 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe040c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0410 | store | 4 | `sp -0x58` | `[sp-44]` |
| `ldr` at 0xe0414 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe0418 | store | 4 | `sp -0x54` | `[sp-56]` |
| `ldr` at 0xe041c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0420 | store | 4 | `sp -0x50` | `[sp-60]` |
| `ldr` at 0xe0424 | load | 4 | `abs 0xe0508` | `-` |
| `str` at 0xe0428 | store | 4 | `sp -0x4c` | `0x29c6f8` |
| `ldr` at 0xe0438 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe044c | load | 4 | `abs 0xe0508` | `-` |
| `str` at 0xe0450 | store | 4 | `sp -0x58` | `0x29c6f8` |
| `ldr` at 0xe0458 | load | 4 | `abs 0xe0510` | `-` |
| `ldr` at 0xe0468 | load | 4 | `abs 0xe0528` | `-` |
| `str` at 0xe046c | store | 4 | `sp -0x48` | `0xfffff7ca` |
| `ldr` at 0xe0474 | load | 4 | `abs 0xe04f8` | `-` |
| `ldr` at 0xe0478 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0484 | load | 4 | `abs 0xe0520` | `-` |
| `ldr` at 0xe0488 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xe048c | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xe04ac | load | 4 | `abs 0xe0534` | `-` |
| `ldr` at 0xe04b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe04b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe04c0 | load | 4 | `abs 0xe0538` | `-` |
| `ldr` at 0xe04c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe04c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe04d4 | load | 4 | `abs 0xe053c` | `-` |
| `ldr` at 0xe04d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe04dc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe04e8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe04ec | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xe04f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe04f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe04f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe0148 | `(unknown callee) 0x221f0` | `0x122744` | `r0` | `r1` | `r2` |
| `bl` at 0xe0160 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `0x29742c` |
| `bl` at 0xe0184 | `(unknown callee) 0x197a8` | `0x29c6f8` | `0x122778` | `[sp-20]` | `ret(0xdefac)` |
| `bl` at 0xe01a0 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x52b` | `0x0` |
| `bl` at 0xe01b4 | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |
| `bl` at 0xe0304 | `(unknown callee) 0xd0814` | `0x1` | `?` | `0x29c6f8` | `[sp-24]` |
| `bl` at 0xe032c | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x54c` | `0x0` |
| `bl` at 0xe0354 | `(unknown callee) 0x221f0` | `0x1227a8` | `[sp-64]` | `?` | `[0x29737c]` |
| `bl` at 0xe03ec | `(unknown callee) 0xd6084` | `0x0` | `0x20` | `[sp-60]` | `[[sp-52]+4]:2` |
| `bl` at 0xe0408 | `(unknown callee) 0xd5e9c` | `0x0` | `[sp-60]` | `[sp-36]` | `[sp-56]` |
| `bl` at 0xe043c | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-64]` |
| `bl` at 0xe0464 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x560` | `0x0` |
| `bl` at 0xe04b8 | `(unknown callee) 0x221f0` | `0x1227e0` | `[sp-64]` | `?` | `?` |
| `bl` at 0xe04cc | `(unknown callee) 0xe8e78` | `0x0` | `0x12283c` | `[sp-32]` | `[sp-56]` |
| `bl` at 0xe04e0 | `(unknown callee) 0xe8e78` | `0x0` | `0x122848` | `[sp-32]` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe0134 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe0174 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xe01c0 | `ret(0xdcd98) == 0x0` | `ret(0xdcd98) != 0x0` |
| `beq` at 0xe01e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe02f4 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0xe0310 | `ret(0xd0814) != 0x0` | `ret(0xd0814) == 0x0` |
| `bls` at 0xe0348 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xe0398 | `[0x2973a8] >= [sp-32] (unsigned, 32-bit)` | `[0x2973a8] < [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xe0448 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe0480 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe04f4 | `r0` | `?` |

## `JChip_tdr_load`

- `JChip_tdr_load`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0540 to 0xe0eb0.
  Established. The unnamed `0xe0104` call carries `[sp-20]`, `0x0`, `[sp-28]`, unknown (`bl` at 0xe0728).
  A signed nonnegative `ret(0xe0104)` takes `0xe0768`, while a negative value falls through (`bge` at 0xe073c).
  Included word assignments write `0x0` and `0x1` to `[sp-32] +0x0` (`str` at 0xe0db8, `str` at 0xe0e08).
  TDR comparison status meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe0550 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe0554 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe0558 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe055c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe0564 | store | 4 | `sp -0xd0` | `0x0` |
| `ldr` at 0xe0568 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe056c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0578 | load | 4 | `abs 0xe0e34` | `-` |
| `ldr` at 0xe057c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0580 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0584 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe058c | load | 4 | `abs 0xe0e38` | `-` |
| `str` at 0xe0590 | store | 4 | `sp -0x48` | `0x2973ac` |
| `ldr` at 0xe0594 | load | 4 | `abs 0xe0e3c` | `-` |
| `str` at 0xe0598 | store | 4 | `sp -0x4c` | `0x29742c` |
| `ldr` at 0xe059c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe05a8 | store | 4 | `sp -0x3c` | `ret(0xdefac)` |
| `ldr` at 0xe05ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe05b8 | load | 4 | `abs 0xe0e40` | `-` |
| `ldr` at 0xe05bc | load | 4 | `abs 0xe0e44` | `-` |
| `ldr` at 0xe05c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe05c8 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe05cc | store | 4 | `sp -0xe8` | `0x29c6f8` |
| `ldr` at 0xe05d4 | load | 4 | `abs 0xe0e48` | `-` |
| `ldr` at 0xe05d8 | load | 4 | `abs 0xe0e4c` | `-` |
| `ldr` at 0xe05e4 | load | 4 | `abs 0xe0e50` | `-` |
| `str` at 0xe05e8 | store | 4 | `sp -0xd4` | `0xfffff7c6` |
| `ldr` at 0xe05f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0604 | load | 4 | `abs 0xe0e54` | `-` |
| `str` at 0xe0608 | store | 4 | `sp -0xd4` | `0xfffff7c5` |
| `ldr` at 0xe0610 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0614 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrh` at 0xe0618 | load | 2 | `[[sp-60]] +0xe` | `-` |
| `str` at 0xe062c | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0xe0634 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xe063c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xe0644 | load | 4 | `sp -0x3c` | `-` |
| `ldrsb` at 0xe0648 | load | 1 | `[sp-60] +0x21` | `-` |
| `str` at 0xe064c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe0650 | load | 4 | `abs 0xe0e58` | `-` |
| `ldr` at 0xe0654 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xe0658 | load | 1 | `[sp-60] +0x19` | `-` |
| `ldr` at 0xe0678 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe067c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe0680 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0684 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe068c | load | 4 | `[sp-60] +0x28` | `-` |
| `ldr` at 0xe0694 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0698 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe069c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe06b0 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe06b4 | load | 4 | `abs 0xe0e40` | `-` |
| `ldr` at 0xe06c8 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe06cc | store | 4 | `sp -0xe8` | `0x29c6f8` |
| `ldr` at 0xe06d4 | load | 4 | `abs 0xe0e48` | `-` |
| `ldr` at 0xe06d8 | load | 4 | `abs 0xe0e5c` | `-` |
| `ldr` at 0xe06e4 | load | 4 | `abs 0xe0e60` | `-` |
| `str` at 0xe06e8 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xe06f0 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe06f4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0700 | load | 4 | `abs 0xe0e64` | `-` |
| `ldr` at 0xe0704 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe070c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe071c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0724 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe0730 | store | 4 | `sp -0x44` | `ret(0xe0104)` |
| `ldr` at 0xe0734 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe0740 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe0744 | store | 4 | `sp -0xe8` | `0x29c6f8` |
| `ldr` at 0xe074c | load | 4 | `abs 0xe0e48` | `-` |
| `ldr` at 0xe0750 | load | 4 | `abs 0xe0e68` | `-` |
| `ldr` at 0xe075c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0760 | store | 4 | `sp -0xd4` | `[sp-68]` |
| `ldr` at 0xe0768 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe076c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe077c | load | 4 | `abs 0xe0e6c` | `-` |
| `ldr` at 0xe0780 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe078c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0790 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xe0794 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `ldr` at 0xe0798 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe07a4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe07a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe07bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe07c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe07cc | load | 4 | `abs 0xe0e70` | `-` |
| `str` at 0xe07d4 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xe07d8 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe07dc | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe07e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe07ec | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe07f0 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe07f8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe0804 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0808 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe080c | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0810 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe0818 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xe0820 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0824 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0828 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xe082c | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xe0830 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xe0838 | load | 4 | `[sp-64] +0x10` | `-` |
| `ldr` at 0xe083c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0840 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe0848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0858 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe085c | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xe0860 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe0864 | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xe0868 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe086c | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xe0870 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe0874 | store | 4 | `sp -0xdc` | `0x29c6f8` |
| `ldr` at 0xe0884 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe0898 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe089c | store | 4 | `sp -0xe8` | `0x29c6f8` |
| `ldr` at 0xe08a4 | load | 4 | `abs 0xe0e48` | `-` |
| `ldr` at 0xe08a8 | load | 4 | `abs 0xe0e74` | `-` |
| `ldr` at 0xe08b4 | load | 4 | `abs 0xe0e60` | `-` |
| `str` at 0xe08b8 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xe08c0 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe08c4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe08d0 | load | 4 | `abs 0xe0e78` | `-` |
| `ldr` at 0xe08d4 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe08d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe08e4 | load | 4 | `abs 0xe0e7c` | `-` |
| `ldr` at 0xe08e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe08ec | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe08f8 | load | 4 | `abs 0xe0e80` | `-` |
| `ldr` at 0xe08fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0900 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0908 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe090c | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xe0910 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xe0914 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xe0918 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe091c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0920 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0924 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe092c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0930 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0934 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe093c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe094c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0950 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0954 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xe0958 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xe095c | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xe0960 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0964 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe096c | load | 4 | `[sp-64] +0x10` | `-` |
| `str` at 0xe0978 | store | 4 | `sp -0x28` | `0x6` |
| `str` at 0xe0984 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xe0988 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe098c | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xe0990 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe0994 | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xe0998 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe099c | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xe09a0 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe09a4 | store | 4 | `sp -0xdc` | `0x29c6f8` |
| `ldr` at 0xe09ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe09b4 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe09c8 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe09cc | store | 4 | `sp -0xe8` | `0x29c6f8` |
| `ldr` at 0xe09d4 | load | 4 | `abs 0xe0e48` | `-` |
| `ldr` at 0xe09d8 | load | 4 | `abs 0xe0e84` | `-` |
| `ldr` at 0xe09e4 | load | 4 | `abs 0xe0e60` | `-` |
| `str` at 0xe09e8 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xe09f0 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe09f4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0a00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0a0c | load | 4 | `abs 0xe0e88` | `-` |
| `str` at 0xe0a10 | store | 4 | `sp -0xd8` | `0x122930` |
| `ldr` at 0xe0a18 | load | 4 | `abs 0xe0e8c` | `-` |
| `str` at 0xe0a1c | store | 4 | `sp -0xd8` | `0x122940` |
| `ldr` at 0xe0a20 | load | 4 | `abs 0xe0e90` | `-` |
| `ldr` at 0xe0a24 | load | 4 | `sp -0xd8` | `-` |
| `ldr` at 0xe0a28 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe0a2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe0a38 | load | 4 | `abs 0xe0e94` | `-` |
| `ldr` at 0xe0a3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0a40 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe0a4c | load | 4 | `abs 0xe0e98` | `-` |
| `ldr` at 0xe0a50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0a54 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0a5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0a70 | load | 4 | `abs 0xe0e9c` | `-` |
| `ldr` at 0xe0a74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0a80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0a90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0a94 | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xe0a98 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe0a9c | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xe0aa0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe0aa4 | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xe0aa8 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe0aac | store | 4 | `sp -0xdc` | `0x29c6f8` |
| `ldr` at 0xe0abc | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe0ad0 | load | 4 | `abs 0xe0e40` | `-` |
| `str` at 0xe0ad4 | store | 4 | `sp -0xe8` | `0x29c6f8` |
| `ldr` at 0xe0adc | load | 4 | `abs 0xe0e48` | `-` |
| `ldr` at 0xe0ae0 | load | 4 | `abs 0xe0ea0` | `-` |
| `ldr` at 0xe0aec | load | 4 | `abs 0xe0e60` | `-` |
| `str` at 0xe0af0 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xe0af8 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe0afc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0b08 | load | 4 | `abs 0xe0ea4` | `-` |
| `ldr` at 0xe0b0c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xe0b10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe0b1c | load | 4 | `abs 0xe0e94` | `-` |
| `ldr` at 0xe0b20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b24 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe0b30 | load | 4 | `abs 0xe0e98` | `-` |
| `ldr` at 0xe0b34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b38 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0b40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0b4c | load | 4 | `abs 0xe0e70` | `-` |
| `str` at 0xe0b54 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xe0b58 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0b5c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0b60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b6c | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0b70 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe0b8c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0b90 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0b94 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0b98 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe0ba0 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xe0bac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0bb4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0bbc | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe0bc0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0bd4 | load | 4 | `abs 0xe0ea8` | `-` |
| `ldr` at 0xe0bd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0be0 | load | 4 | `abs 0xe0e70` | `-` |
| `str` at 0xe0be8 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xe0bec | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0bf0 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0bf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0c00 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c04 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0c0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe0c14 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c18 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0c20 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe0c28 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c2c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0c44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0c48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0c50 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0c54 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c58 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c5c | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe0c64 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xe0c6c | load | 4 | `abs 0xe0e70` | `-` |
| `str` at 0xe0c74 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xe0c78 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c7c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0c80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0c8c | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0c90 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0c98 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0ca0 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0ca4 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0cac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0cb4 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0cb8 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0cd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0cd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0cdc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0ce0 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0ce4 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0ce8 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe0cf0 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xe0cf8 | load | 4 | `abs 0xe0e70` | `-` |
| `str` at 0xe0d00 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xe0d04 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0d08 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0d0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0d18 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0d1c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0d34 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0d38 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0d40 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe0d48 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0d4c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xe0d54 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe0d5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0d60 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0d68 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0d6c | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0d70 | load | 4 | `abs 0xe0e70` | `-` |
| `ldr` at 0xe0d74 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xe0d7c | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xe0d84 | load | 4 | `abs 0xe0e30` | `-` |
| `ldr` at 0xe0d88 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe0d9c | load | 4 | `abs 0xe0eac` | `-` |
| `ldr` at 0xe0da0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0dac | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe0db0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0db8 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xe0dbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0dc0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0dcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0dd0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xe0ddc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0df4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0e00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0e08 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0xe0e0c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe0e14 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xe0e20 | store | 4 | `sp -0xd4` | `0x0` |
| `ldr` at 0xe0e24 | load | 4 | `sp -0xd4` | `-` |
| `ldm` at 0xe0e2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe0e2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe0e2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe0588 | `(unknown callee) 0x221f0` | `0x122854` | `r0` | `r1` | `r2` |
| `bl` at 0xe05a0 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `0x29742c` |
| `bl` at 0xe05c4 | `(unknown callee) 0x197a8` | `0x29c6f8` | `0x122778` | `[sp-20]` | `ret(0xdefac)` |
| `bl` at 0xe05e0 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x58c` | `0x0` |
| `bl` at 0xe05f4 | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |
| `bl` at 0xe06b8 | `(unknown callee) 0xd0814` | `0x1` | `?` | `0x29c6f8` | `?` |
| `bl` at 0xe06e0 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x5a8` | `0x0` |
| `bl` at 0xe0708 | `(unknown callee) 0x221f0` | `0x1227a8` | `[sp-208]` | `?` | `[0x29737c]` |
| `bl` at 0xe0728 | `(unknown callee) 0xe0104` | `[sp-20]` | `0x0` | `[sp-28]` | `?` |
| `bl` at 0xe0758 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x5b4` | `0x0` |
| `bl` at 0xe0788 | `(unknown callee) 0xe4750` | `0x0` | `0x122890` | `[sp-64]` | `0x4` |
| `bl` at 0xe0844 | `(unknown callee) 0xd5e9c` | `0x0` | `[[sp-64]+16]` | `[sp-52]` | `[sp-72]` |
| `bl` at 0xe0888 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x6` | `[sp-208]` |
| `bl` at 0xe08b0 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x5c7` | `0x0` |
| `bl` at 0xe08dc | `(unknown callee) 0x221f0` | `0x1228ac` | `[sp-208]` | `[sp-36]` | `[0x29737c]` |
| `bl` at 0xe08f0 | `(unknown callee) 0xe8e78` | `0x0` | `0x122908` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xe0904 | `(unknown callee) 0xe8e78` | `0x0` | `0x12291c` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xe0928 | `(unknown callee) 0xd5e9c` | `[sp-52]` | `[sp-76]` | `[sp-52]` | `[sp-72]` |
| `bl` at 0xe0938 | `(unknown callee) 0xe5470` | `[sp-64]` | `[sp-52]` | `[sp-72]` | `?` |
| `bl` at 0xe0970 | `(unknown callee) 0xd5e9c` | `[sp-52]` | `[sp-72]` | `0x0` | `[[sp-64]+16]` |
| `bl` at 0xe09b8 | `(unknown callee) 0xd0d10` | `0x1` | `?` | `0x1` | `[sp-208]` |
| `bl` at 0xe09e0 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x5e3` | `0x0` |
| `bl` at 0xe0a30 | `(unknown callee) 0x221f0` | `0x12294c` | `[sp-216]` | `[sp-208]` | `[sp-36]` |
| `bl` at 0xe0a44 | `(unknown callee) 0xe8e78` | `0x0` | `0x1229a0` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xe0a58 | `(unknown callee) 0xe8e78` | `0x0` | `0x1229ac` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xe0a7c | `(unknown callee) 0xe4750` | `0x0` | `0x1229b8` | `[sp-64]` | `0x4` |
| `bl` at 0xe0ac0 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-208]` |
| `bl` at 0xe0ae8 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x5f6` | `0x0` |
| `bl` at 0xe0b14 | `(unknown callee) 0x221f0` | `0x1229d4` | `[sp-208]` | `[sp-36]` | `[0x29737c]` |
| `bl` at 0xe0b28 | `(unknown callee) 0xe8e78` | `0x0` | `0x1229a0` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xe0b3c | `(unknown callee) 0xe8e78` | `0x0` | `0x1229ac` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xe0bb8 | `(unknown callee) 0xe557c` | `[sp-64]` | `0x4` | `[sp-52]` | `sp-204` |
| `bl` at 0xe0bdc | `(unknown callee) 0xe8e78` | `0x0` | `0x122a30` | `[sp-44]` | `sp-204` |
| `bl` at 0xe0da4 | `(unknown callee) 0xe8e78` | `0x0` | `0x122a40` | `[sp-44]` | `sp-204` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe0574 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe05b4 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xe0600 | `ret(0xdcd98) == 0x0` | `ret(0xdcd98) != 0x0` |
| `beq` at 0xe0624 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe06a8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe06c4 | `ret(0xd0814) != 0x0` | `ret(0xd0814) == 0x0` |
| `bls` at 0xe06fc | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xe0718 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xe073c | `ret(0xe0104) >= 0x0 (signed, 32-bit)` | `ret(0xe0104) < 0x0 (signed, 32-bit)` |
| `bls` at 0xe0774 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhs` at 0xe07e8 | `[0x2973a8] >= [sp-44] (unsigned, 32-bit)` | `[0x2973a8] < [sp-44] (unsigned, 32-bit)` |
| `beq` at 0xe0854 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe0894 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe08cc | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe0948 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe09c4 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe09fc | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe0a08 | `[sp-40] != 0x6` | `[sp-40] == 0x6` |
| `beq` at 0xe0a68 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe0a8c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe0acc | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe0b04 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe0b48 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bhs` at 0xe0b68 | `[0x2973a8] >= [sp-44] (unsigned, 32-bit)` | `[0x2973a8] < [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xe0bc8 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xe0bfc | `[0x2973a8] >= [sp-44] (unsigned, 32-bit)` | `[0x2973a8] < [sp-44] (unsigned, 32-bit)` |
| `bhs` at 0xe0c88 | `[0x2973a8] >= [sp-44] (unsigned, 32-bit)` | `[0x2973a8] < [sp-44] (unsigned, 32-bit)` |
| `bhs` at 0xe0d14 | `[0x2973a8] >= [sp-44] (unsigned, 32-bit)` | `[0x2973a8] < [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xe0d90 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xe0dc8 | `[sp-48] >= [sp-44] (unsigned, 32-bit)` | `[sp-48] < [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xe0dd8 | `[[sp-32]] != 0x0` | `[[sp-32]] == 0x0` |
| `beq` at 0xe0dfc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe0e2c | `r0` | `?` |

## `JChip_TDR_Raw`

- `JChip_TDR_Raw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xde978 to 0xdef00.
  Established. The signed comparison `[sp] <= 0x400` takes `0xdeda4`, while a larger value falls through (`ble` at 0xded34).
  The unnamed `0xdf374` call carries `[sp-64]`, `[sp-60]`, `[sp]`, `[sp-28]` (`bl` at 0xdedc4).
  A later unnamed `0xd5e9c` call carries `[sp-60]`, `0x29742c`, `0x0`, `[sp-32]` (`bl` at 0xdee90).
  Raw TDR exchange semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xde988 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xde98c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xde990 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xde994 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xde998 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde9a4 | store | 4 | `sp -0x28` | `ret(0xdefac)` |
| `str` at 0xde9ac | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xde9b0 | load | 4 | `abs 0xdeea8` | `-` |
| `str` at 0xde9b4 | store | 4 | `sp -0x44` | `0x2973ac` |
| `ldr` at 0xde9b8 | load | 4 | `abs 0xdeeac` | `-` |
| `str` at 0xde9bc | store | 4 | `sp -0x48` | `0x29742c` |
| `ldr` at 0xde9c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde9cc | load | 4 | `abs 0xdeeb0` | `-` |
| `str` at 0xde9d0 | store | 4 | `sp -0x4c` | `0xfffff7c6` |
| `ldr` at 0xde9d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde9ec | load | 4 | `abs 0xdeeb4` | `-` |
| `str` at 0xde9f0 | store | 4 | `sp -0x4c` | `0xfffff7c5` |
| `ldr` at 0xde9f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xde9fc | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrh` at 0xdea00 | load | 2 | `[[sp-40]] +0xe` | `-` |
| `str` at 0xdea14 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xdea1c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdea20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdea24 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0xdea28 | load | 1 | `[[sp-40]] +0xc` | `-` |
| `str` at 0xdea2c | store | 4 | `sp -0x34` | `[[[sp-40]]+12]:1` |
| `ldr` at 0xdea30 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdea34 | store | 4 | `sp -0x38` | `[[[sp-40]]+12]:1` |
| `str` at 0xdea3c | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xdea44 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xdea4c | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xdea50 | load | 1 | `[sp-40] +0x21` | `-` |
| `str` at 0xdea54 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xdea58 | load | 4 | `abs 0xdeeb8` | `-` |
| `ldr` at 0xdea5c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdea60 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xdea80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdea88 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xdea90 | load | 2 | `(unknown)` | `-` |
| `str` at 0xdea94 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdea98 | load | 4 | `abs 0xdeeb8` | `-` |
| `ldr` at 0xdea9c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdeaa0 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xdeac0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdeac8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdead0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xdead4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdead8 | load | 4 | `abs 0xdeeb8` | `-` |
| `ldr` at 0xdeadc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdeae0 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xdeb00 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdeb04 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xdeb08 | load | 4 | `abs 0xdeeb8` | `-` |
| `ldr` at 0xdeb0c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xdeb10 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xdeb30 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdeb34 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xdeb38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdeb44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdeb48 | load | 4 | `abs 0xdeebc` | `-` |
| `ldr` at 0xdeb54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdeb5c | load | 4 | `abs 0xdeebc` | `-` |
| `str` at 0xdeb68 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xdeb6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdeb70 | load | 4 | `abs 0xdeec0` | `-` |
| `ldr` at 0xdeb7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdeb84 | load | 4 | `abs 0xdeec0` | `-` |
| `str` at 0xdeb90 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xdeb94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdeb98 | load | 4 | `abs 0xdeec4` | `-` |
| `ldr` at 0xdeba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdebac | load | 4 | `abs 0xdeec4` | `-` |
| `str` at 0xdebb8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xdebbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdebcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdebe0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xdebe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdebf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdec08 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xdec10 | load | 4 | `abs 0xdeea8` | `-` |
| `str` at 0xdec14 | store | 4 | `sp -0x5c` | `0x2973ac` |
| `ldr` at 0xdec18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdec1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdec20 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdec30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdec34 | load | 4 | `abs 0xdeec8` | `-` |
| `ldr` at 0xdec3c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdec40 | store | 4 | `sp -0x5c` | `[sp-56]` |
| `ldr` at 0xdec44 | load | 4 | `abs 0xdeea8` | `-` |
| `str` at 0xdec48 | store | 4 | `sp -0x58` | `0x2973ac` |
| `ldr` at 0xdec4c | load | 4 | `abs 0xdeeac` | `-` |
| `str` at 0xdec50 | store | 4 | `sp -0x54` | `0x29742c` |
| `ldr` at 0xdec54 | load | 4 | `abs 0xdeec8` | `-` |
| `str` at 0xdec58 | store | 4 | `sp -0x50` | `0x29c6f8` |
| `ldr` at 0xdec68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdec7c | load | 4 | `abs 0xdeecc` | `-` |
| `str` at 0xdec80 | store | 4 | `sp -0x5c` | `0x1222f4` |
| `ldr` at 0xdec88 | load | 4 | `abs 0xdeed0` | `-` |
| `ldr` at 0xdec8c | load | 4 | `abs 0xdeed4` | `-` |
| `ldr` at 0xdec98 | load | 4 | `abs 0xdeed8` | `-` |
| `str` at 0xdec9c | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xdeca4 | load | 4 | `abs 0xdeedc` | `-` |
| `ldr` at 0xdeca8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdecb4 | load | 4 | `abs 0xdeeac` | `-` |
| `str` at 0xdecb8 | store | 4 | `sp -0x5c` | `0x29742c` |
| `ldr` at 0xdecbc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xdecc0 | load | 4 | `abs 0xdeee0` | `-` |
| `ldr` at 0xdecc4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdecc8 | load | 4 | `abs 0xdeea8` | `-` |
| `ldr` at 0xdecd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdecdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdecec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdecf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdecfc | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xded00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xded10 | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0xded14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xded20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xded24 | store | 4 | `sp -0x48` | `[sp-32]` |
| `ldr` at 0xded2c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xded38 | load | 4 | `abs 0xdeee4` | `-` |
| `str` at 0xded3c | store | 4 | `sp -0x5c` | `0x122320` |
| `ldr` at 0xded40 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xded44 | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0xded4c | load | 4 | `abs 0xdeed0` | `-` |
| `ldr` at 0xded50 | load | 4 | `abs 0xdeee8` | `-` |
| `ldr` at 0xded5c | load | 4 | `abs 0xdeed8` | `-` |
| `str` at 0xded60 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xded68 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xded74 | load | 4 | `abs 0xdeeec` | `-` |
| `str` at 0xded78 | store | 4 | `sp -0x5c` | `0x122370` |
| `ldr` at 0xded7c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xded80 | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0xded88 | load | 4 | `abs 0xdeed0` | `-` |
| `ldr` at 0xded8c | load | 4 | `abs 0xdeef0` | `-` |
| `ldr` at 0xded98 | load | 4 | `abs 0xdeed8` | `-` |
| `str` at 0xded9c | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `str` at 0xdeda8 | store | 4 | `sp -0x5c` | `sp-36` |
| `ldr` at 0xdedac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdedb0 | store | 4 | `sp -0x58` | `[sp-68]` |
| `ldr` at 0xdedb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdedb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdedbc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xdedc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdedc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdedcc | store | 4 | `sp -0x5c` | `[sp-36]` |
| `ldr` at 0xdedd0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdedd4 | store | 4 | `sp -0x58` | `[sp-68]` |
| `ldr` at 0xdedd8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xdeddc | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0xdede0 | load | 4 | `abs 0xdeec8` | `-` |
| `str` at 0xdede4 | store | 4 | `sp -0x50` | `0x29c6f8` |
| `ldr` at 0xdedf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdee08 | load | 4 | `abs 0xdeef4` | `-` |
| `str` at 0xdee0c | store | 4 | `sp -0x5c` | `0x1223d0` |
| `ldr` at 0xdee14 | load | 4 | `abs 0xdeed0` | `-` |
| `ldr` at 0xdee18 | load | 4 | `abs 0xdeef8` | `-` |
| `ldr` at 0xdee24 | load | 4 | `abs 0xdeed8` | `-` |
| `str` at 0xdee28 | store | 4 | `sp -0x4c` | `0xfffff7ca` |
| `ldr` at 0xdee30 | load | 4 | `abs 0xdeedc` | `-` |
| `ldr` at 0xdee34 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdee40 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xdee44 | store | 4 | `sp -0x5c` | `[sp-72]` |
| `ldr` at 0xdee48 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xdee4c | load | 4 | `abs 0xdeefc` | `-` |
| `ldr` at 0xdee50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdee54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdee68 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xdee6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdee78 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xdee7c | store | 4 | `sp -0x5c` | `[sp]` |
| `ldr` at 0xdee80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdee84 | load | 4 | `abs 0xdeeac` | `-` |
| `ldr` at 0xdee8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdee98 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xdee9c | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0xdeea4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdeea4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdeea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xde99c | `(unknown callee) 0xdefac` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xde9dc | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |
| `bl` at 0xdec28 | `(unknown callee) 0xdf0f0` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `sp-24` |
| `bl` at 0xdec38 | `(unknown callee) 0xd0754` | `0x1` | `[sp-44]` | `0x29c6f8` | `?` |
| `bl` at 0xdec6c | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `[sp-44]` |
| `bl` at 0xdec94 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x339` | `0x0` |
| `bl` at 0xdeccc | `(unknown callee) 0xdf464` | `[sp+4]` | `0x122318` | `[sp-56]` | `0x2973ac` |
| `bl` at 0xded58 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x362` | `0x0` |
| `bl` at 0xded94 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x36a` | `0x0` |
| `bl` at 0xdedc4 | `(unknown callee) 0xdf374` | `[sp-64]` | `[sp-60]` | `[sp]` | `[sp-28]` |
| `bl` at 0xdedf8 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-44]` |
| `bl` at 0xdee20 | `(unknown callee) 0x200a0` | `0x3` | `0x12208c` | `0x373` | `0x0` |
| `bl` at 0xdee58 | `(unknown callee) 0xdf464` | `[sp+4]` | `0x1223f4` | `[sp-36]` | `[sp-68]` |
| `bl` at 0xdee90 | `(unknown callee) 0xd5e9c` | `[sp-60]` | `0x29742c` | `0x0` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xde9c8 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0xde9e8 | `ret(0xdcd98) == 0x0` | `ret(0xdcd98) != 0x0` |
| `beq` at 0xdea0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdeb40 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xdec78 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xdecb0 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xdecd8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xdece4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xdecf4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xded08 | `[sp-64] != 0x1` | `[sp-64] == 0x1` |
| `beq` at 0xded1c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `ble` at 0xded34 | `[sp] <= 0x400 (signed, 32-bit)` | `[sp] > 0x400 (signed, 32-bit)` |
| `ble` at 0xded70 | `[sp] <= 0x400 (signed, 32-bit)` | `[sp] > 0x400 (signed, 32-bit)` |
| `bne` at 0xdee04 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xdee3c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xdee64 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xdee74 | `[sp-72] == [sp-32]` | `[sp-72] != [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdeea4 | `r0` | `?` |

## `setup_tdrdata_shub`

- `setup_tdrdata_shub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5678 to 0xe5c40.
  Established. The unnamed `0x1943c` call carries `0x3c` followed by three unknown arguments (`bl` at 0xe56a4).
  A word store places `ret(0x1943c)` at `[sp-24] +0x28` (`str` at 0xe56a8).
  A nonzero `[[sp-24]+40]` takes `0xe56c4`, while zero falls through (`bne` at 0xe56b4).
  SHUB TDR descriptor layout beyond these accesses remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5688 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe5690 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe5694 | load | 4 | `abs 0xe5bd4` | `-` |
| `ldr` at 0xe569c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe56a8 | store | 4 | `[sp-24] +0x28` | `ret(0x1943c)` |
| `ldr` at 0xe56ac | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe56bc | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe56c8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe56cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe56d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe56dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe56e4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe56f0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe56f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe56fc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe5704 | load | 4 | `abs 0xe5bd8` | `-` |
| `ldr` at 0xe570c | load | 4 | `abs 0xe5bdc` | `-` |
| `ldr` at 0xe5714 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe571c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5728 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5730 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5738 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe573c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe574c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5754 | load | 4 | `abs 0xe5be0` | `-` |
| `ldr` at 0xe575c | load | 4 | `abs 0xe5be4` | `-` |
| `ldr` at 0xe5764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe576c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5778 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5780 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5788 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe578c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe579c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe57a4 | load | 4 | `abs 0xe5be8` | `-` |
| `ldr` at 0xe57ac | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe57b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe57bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe57c8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe57d0 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe57d8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe57dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe57ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe57f4 | load | 4 | `abs 0xe5bf0` | `-` |
| `ldr` at 0xe57fc | load | 4 | `abs 0xe5bf4` | `-` |
| `ldr` at 0xe5804 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe580c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5818 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5820 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5828 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe582c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe583c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5844 | load | 4 | `abs 0xe5bf8` | `-` |
| `ldr` at 0xe584c | load | 4 | `abs 0xe5bfc` | `-` |
| `ldr` at 0xe5854 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe585c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5868 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5870 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5878 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe587c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe588c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5894 | load | 4 | `abs 0xe5c00` | `-` |
| `ldr` at 0xe589c | load | 4 | `abs 0xe5c04` | `-` |
| `ldr` at 0xe58a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe58ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe58b8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe58c0 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe58c8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe58cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe58dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe58e4 | load | 4 | `abs 0xe5c08` | `-` |
| `ldr` at 0xe58ec | load | 4 | `abs 0xe5c0c` | `-` |
| `ldr` at 0xe58f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe58fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5908 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5910 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5918 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe591c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe592c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5934 | load | 4 | `abs 0xe5c10` | `-` |
| `ldr` at 0xe593c | load | 4 | `abs 0xe5c14` | `-` |
| `ldr` at 0xe5944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe594c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5958 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5960 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5968 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe596c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe597c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5984 | load | 4 | `abs 0xe5c18` | `-` |
| `ldr` at 0xe598c | load | 4 | `abs 0xe5c14` | `-` |
| `ldr` at 0xe5994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe599c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe59a8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe59b0 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe59b8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe59bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe59cc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe59d4 | load | 4 | `abs 0xe5c1c` | `-` |
| `ldr` at 0xe59dc | load | 4 | `abs 0xe5c20` | `-` |
| `ldr` at 0xe59e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe59ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe59f8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5a00 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5a08 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5a0c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5a1c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5a24 | load | 4 | `abs 0xe5c24` | `-` |
| `ldr` at 0xe5a2c | load | 4 | `abs 0xe5c20` | `-` |
| `ldr` at 0xe5a34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5a3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5a48 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5a50 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5a58 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5a5c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5a6c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5a74 | load | 4 | `abs 0xe5c28` | `-` |
| `ldr` at 0xe5a7c | load | 4 | `abs 0xe5c2c` | `-` |
| `ldr` at 0xe5a84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5a8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5a98 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5aa0 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5aa8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5aac | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5abc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5ac4 | load | 4 | `abs 0xe5c30` | `-` |
| `ldr` at 0xe5acc | load | 4 | `abs 0xe5c2c` | `-` |
| `ldr` at 0xe5ad4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5adc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5ae8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5af0 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5af8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5afc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5b0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5b14 | load | 4 | `abs 0xe5c34` | `-` |
| `ldr` at 0xe5b1c | load | 4 | `abs 0xe5c38` | `-` |
| `ldr` at 0xe5b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5b2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5b38 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5b40 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5b48 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5b4c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5b5c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5b64 | load | 4 | `abs 0xe5c3c` | `-` |
| `ldr` at 0xe5b6c | load | 4 | `abs 0xe5c38` | `-` |
| `ldr` at 0xe5b74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5b7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5b88 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5b90 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5b98 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5b9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5bac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5bb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5bb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe5bbc | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xe5bc4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe5bc8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xe5bd0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe5bd0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe5bd0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe5bd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5698 | `(unknown callee) 0x221f0` | `0x123478` | `r1` | `r2` | `0x0` |
| `bl` at 0xe56a4 | `(unknown callee) 0x1943c` | `0x3c` | `?` | `?` | `?` |
| `bl` at 0xe5710 | `(unknown callee) 0xe4524` | `0x123490` | `0x7c` | `0x12ec70` | `?` |
| `bl` at 0xe5760 | `(unknown callee) 0xe4524` | `0x12349c` | `0x7d` | `0x12ec88` | `?` |
| `bl` at 0xe57b0 | `(unknown callee) 0xe4524` | `0x1234a8` | `0x7f` | `0x12eca0` | `?` |
| `bl` at 0xe5800 | `(unknown callee) 0xe4524` | `0x1234b0` | `0x7e` | `0x12db50` | `?` |
| `bl` at 0xe5850 | `(unknown callee) 0xe4524` | `0x1234b8` | `0x7` | `0x12dbe0` | `?` |
| `bl` at 0xe58a0 | `(unknown callee) 0xe4524` | `0x1234c0` | `0x5` | `0x12dc1c` | `?` |
| `bl` at 0xe58f0 | `(unknown callee) 0xe4524` | `0x1234cc` | `0x6` | `0x12ddd8` | `?` |
| `bl` at 0xe5940 | `(unknown callee) 0xe4524` | `0x1234d4` | `0xa` | `0x12de80` | `?` |
| `bl` at 0xe5990 | `(unknown callee) 0xe4524` | `0x1234e0` | `0xb` | `0x12de80` | `?` |
| `bl` at 0xe59e0 | `(unknown callee) 0xe4524` | `0x1234ec` | `0x1a` | `0x12edbc` | `?` |
| `bl` at 0xe5a30 | `(unknown callee) 0xe4524` | `0x1234fc` | `0x1b` | `0x12edbc` | `?` |
| `bl` at 0xe5a80 | `(unknown callee) 0xe4524` | `0x12350c` | `0x60` | `0x12def8` | `?` |
| `bl` at 0xe5ad0 | `(unknown callee) 0xe4524` | `0x12351c` | `0x62` | `0x12def8` | `?` |
| `bl` at 0xe5b20 | `(unknown callee) 0xe4524` | `0x12352c` | `0x61` | `0x12ee34` | `?` |
| `bl` at 0xe5b70 | `(unknown callee) 0xe4524` | `0x12353c` | `0x63` | `0x12ee34` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe56b4 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bhi` at 0xe56d4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe5744 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5794 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe57e4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5834 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5884 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe58d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5924 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5974 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe59c4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5a14 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5a64 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5ab4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5b04 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5b54 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5ba4 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe5bd0 | `r0` | `[sp-36]` |

## `setup_tdrdata_tio`

- `setup_tdrdata_tio`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5c40 to 0xe62b8.
  Established. The unnamed `0x1943c` call receives `0x44` and three unknown argument candidates (`bl` at 0xe5c6c).
  Word stores write `ret(0x1943c)` to `[sp-24] +0x28` and `[sp-32]+1` to `[sp-24] +0x24` (`str` at 0xe5c70, `str` at 0xe6224).
  TIO descriptor ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5c50 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe5c58 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe5c5c | load | 4 | `abs 0xe623c` | `-` |
| `ldr` at 0xe5c64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5c70 | store | 4 | `[sp-24] +0x28` | `ret(0x1943c)` |
| `ldr` at 0xe5c74 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5c84 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe5c90 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe5c94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5ca0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5cac | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5cb8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe5cbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe5cc4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe5ccc | load | 4 | `abs 0xe6240` | `-` |
| `ldr` at 0xe5cd4 | load | 4 | `abs 0xe6244` | `-` |
| `ldr` at 0xe5cdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5ce4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5cf0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5cf8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5d00 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5d04 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5d14 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5d1c | load | 4 | `abs 0xe6248` | `-` |
| `ldr` at 0xe5d24 | load | 4 | `abs 0xe624c` | `-` |
| `ldr` at 0xe5d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5d34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5d40 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5d48 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5d50 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5d54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5d64 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5d6c | load | 4 | `abs 0xe6250` | `-` |
| `ldr` at 0xe5d74 | load | 4 | `abs 0xe6254` | `-` |
| `ldr` at 0xe5d7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5d84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5d90 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5d98 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5da0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5da4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5db4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5dbc | load | 4 | `abs 0xe6258` | `-` |
| `ldr` at 0xe5dc4 | load | 4 | `abs 0xe625c` | `-` |
| `ldr` at 0xe5dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5dd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5de0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5de8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5df0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5df4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5e04 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5e0c | load | 4 | `abs 0xe6260` | `-` |
| `ldr` at 0xe5e14 | load | 4 | `abs 0xe6264` | `-` |
| `ldr` at 0xe5e1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5e24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5e30 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5e38 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5e40 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5e44 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5e54 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5e5c | load | 4 | `abs 0xe6268` | `-` |
| `ldr` at 0xe5e64 | load | 4 | `abs 0xe626c` | `-` |
| `ldr` at 0xe5e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5e74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5e80 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5e88 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5e90 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5e94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5ea4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5eac | load | 4 | `abs 0xe6270` | `-` |
| `ldr` at 0xe5eb4 | load | 4 | `abs 0xe6274` | `-` |
| `ldr` at 0xe5ebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5ec4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5ed0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5ed8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5ee0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5ee4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5ef4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5efc | load | 4 | `abs 0xe6278` | `-` |
| `ldr` at 0xe5f04 | load | 4 | `abs 0xe627c` | `-` |
| `ldr` at 0xe5f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5f14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5f20 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5f28 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5f30 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5f34 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5f44 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5f4c | load | 4 | `abs 0xe6280` | `-` |
| `ldr` at 0xe5f54 | load | 4 | `abs 0xe6284` | `-` |
| `ldr` at 0xe5f5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5f64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5f70 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5f78 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5f80 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5f84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5f94 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5f9c | load | 4 | `abs 0xe6288` | `-` |
| `ldr` at 0xe5fa4 | load | 4 | `abs 0xe628c` | `-` |
| `ldr` at 0xe5fac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5fb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5fc0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe5fc8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe5fd0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe5fd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5fe4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5fec | load | 4 | `abs 0xe6290` | `-` |
| `ldr` at 0xe5ff4 | load | 4 | `abs 0xe628c` | `-` |
| `ldr` at 0xe5ffc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6004 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6010 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6018 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6020 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6024 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6034 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe603c | load | 4 | `abs 0xe6294` | `-` |
| `ldr` at 0xe6044 | load | 4 | `abs 0xe6298` | `-` |
| `ldr` at 0xe604c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6054 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6060 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6068 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6070 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6074 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6084 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe608c | load | 4 | `abs 0xe629c` | `-` |
| `ldr` at 0xe6094 | load | 4 | `abs 0xe6298` | `-` |
| `ldr` at 0xe609c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe60a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe60b0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe60b8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe60c0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe60c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe60d4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe60dc | load | 4 | `abs 0xe62a0` | `-` |
| `ldr` at 0xe60e4 | load | 4 | `abs 0xe62a4` | `-` |
| `ldr` at 0xe60ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe60f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6100 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6108 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6110 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6114 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6124 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe612c | load | 4 | `abs 0xe62a8` | `-` |
| `ldr` at 0xe6134 | load | 4 | `abs 0xe62a4` | `-` |
| `ldr` at 0xe613c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6144 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6150 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6158 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6160 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6164 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6174 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe617c | load | 4 | `abs 0xe62ac` | `-` |
| `ldr` at 0xe6184 | load | 4 | `abs 0xe62b0` | `-` |
| `ldr` at 0xe618c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6194 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe61a0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe61a8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe61b0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe61b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe61c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe61cc | load | 4 | `abs 0xe62b4` | `-` |
| `ldr` at 0xe61d4 | load | 4 | `abs 0xe62b0` | `-` |
| `ldr` at 0xe61dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe61e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe61f0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe61f8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6200 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6204 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6214 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe621c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6220 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe6224 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xe622c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe6230 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xe6238 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe6238 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe6238 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe6238 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5c60 | `(unknown callee) 0x221f0` | `0x123b3c` | `r1` | `r2` | `0x0` |
| `bl` at 0xe5c6c | `(unknown callee) 0x1943c` | `0x44` | `?` | `?` | `?` |
| `bl` at 0xe5cd8 | `(unknown callee) 0xe4524` | `0x123b54` | `0x7c` | `0x12ec70` | `?` |
| `bl` at 0xe5d28 | `(unknown callee) 0xe4524` | `0x123b60` | `0x7d` | `0x12ec88` | `?` |
| `bl` at 0xe5d78 | `(unknown callee) 0xe4524` | `0x123b6c` | `0x7f` | `0x12eca0` | `?` |
| `bl` at 0xe5dc8 | `(unknown callee) 0xe4524` | `0x123b74` | `0x7e` | `0x12dfbc` | `?` |
| `bl` at 0xe5e18 | `(unknown callee) 0xe4524` | `0x123b7c` | `0x7` | `0x12e088` | `?` |
| `bl` at 0xe5e68 | `(unknown callee) 0xe4524` | `0x123b84` | `0x5` | `0x12e0c4` | `?` |
| `bl` at 0xe5eb8 | `(unknown callee) 0xe4524` | `0x123b90` | `0x6` | `0x12e2a8` | `?` |
| `bl` at 0xe5f08 | `(unknown callee) 0xe4524` | `0x123b98` | `0x12` | `0x12ecb8` | `?` |
| `bl` at 0xe5f58 | `(unknown callee) 0xe4524` | `0x123ba8` | `0x11` | `0x12ecd0` | `?` |
| `bl` at 0xe5fa8 | `(unknown callee) 0xe4524` | `0x123bb8` | `0xa` | `0x12e388` | `?` |
| `bl` at 0xe5ff8 | `(unknown callee) 0xe4524` | `0x123bc4` | `0xb` | `0x12e388` | `?` |
| `bl` at 0xe6048 | `(unknown callee) 0xe4524` | `0x123bd0` | `0x1a` | `0x12edbc` | `?` |
| `bl` at 0xe6098 | `(unknown callee) 0xe4524` | `0x123be0` | `0x1b` | `0x12edbc` | `?` |
| `bl` at 0xe60e8 | `(unknown callee) 0xe4524` | `0x123bf0` | `0x60` | `0x12e428` | `?` |
| `bl` at 0xe6138 | `(unknown callee) 0xe4524` | `0x123c00` | `0x62` | `0x12e428` | `?` |
| `bl` at 0xe6188 | `(unknown callee) 0xe4524` | `0x123c10` | `0x61` | `0x12ee34` | `?` |
| `bl` at 0xe61d8 | `(unknown callee) 0xe4524` | `0x123c20` | `0x63` | `0x12ee34` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe5c7c | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bhi` at 0xe5c9c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe5d0c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5d5c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5dac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5dfc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5e4c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5e9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5eec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5f3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5f8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe5fdc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe602c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe607c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe60cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe611c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe616c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe61bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe620c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe6238 | `r0` | `[sp-36]` |

## `setup_tdrdata_nl4r`

- `setup_tdrdata_nl4r`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe62b8 to 0xe69f4.
  Established. The unnamed `0x1943c` call presents `0xa8` followed by three unknown candidates (`bl` at 0xe62ec);
  its result is written as a word to `[sp-24] +0x28` (`str` at 0xe62f0).
  NL4R scan descriptor interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe62c8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe62d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe62d4 | load | 4 | `abs 0xe697c` | `-` |
| `str` at 0xe62d8 | store | 4 | `sp -0x44` | `0x1246c8` |
| `ldr` at 0xe62dc | load | 4 | `abs 0xe6980` | `-` |
| `ldr` at 0xe62e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe62f0 | store | 4 | `[sp-24] +0x28` | `ret(0x1943c)` |
| `ldr` at 0xe62f4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6304 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xe6310 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe6314 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe632c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6338 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe633c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6344 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe634c | load | 4 | `abs 0xe6984` | `-` |
| `ldr` at 0xe6354 | load | 4 | `abs 0xe6988` | `-` |
| `ldr` at 0xe635c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6364 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6370 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6378 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6380 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6384 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6394 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe639c | load | 4 | `abs 0xe698c` | `-` |
| `ldr` at 0xe63a4 | load | 4 | `abs 0xe6990` | `-` |
| `ldr` at 0xe63ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe63b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe63c0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe63c8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe63d0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe63d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe63e4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe63ec | load | 4 | `abs 0xe6994` | `-` |
| `ldr` at 0xe63f4 | load | 4 | `abs 0xe6998` | `-` |
| `ldr` at 0xe63fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6404 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6410 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6418 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6420 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6424 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6434 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe643c | load | 4 | `abs 0xe699c` | `-` |
| `ldr` at 0xe6444 | load | 4 | `abs 0xe69a0` | `-` |
| `ldr` at 0xe644c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6454 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6460 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6468 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6470 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6474 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6484 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe648c | load | 4 | `abs 0xe69a4` | `-` |
| `ldr` at 0xe6494 | load | 4 | `abs 0xe69a8` | `-` |
| `ldr` at 0xe649c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe64a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe64b0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe64b8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe64c0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe64c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe64d4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe64dc | load | 4 | `abs 0xe69ac` | `-` |
| `ldr` at 0xe64e4 | load | 4 | `abs 0xe69b0` | `-` |
| `ldr` at 0xe64ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe64f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6500 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6508 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6510 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6514 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6524 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe652c | load | 4 | `abs 0xe69b4` | `-` |
| `ldr` at 0xe6534 | load | 4 | `abs 0xe69b8` | `-` |
| `ldr` at 0xe653c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6544 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6550 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6558 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6560 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6564 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6574 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe657c | load | 4 | `abs 0xe69bc` | `-` |
| `ldr` at 0xe6584 | load | 4 | `abs 0xe69c0` | `-` |
| `ldr` at 0xe658c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe65a0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe65a8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe65b0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe65b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe65c4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe65cc | load | 4 | `abs 0xe69c4` | `-` |
| `ldr` at 0xe65d4 | load | 4 | `abs 0xe69c8` | `-` |
| `ldr` at 0xe65dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe65e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe65f0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe65f8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6600 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6604 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6614 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe661c | load | 4 | `abs 0xe69cc` | `-` |
| `ldr` at 0xe6624 | load | 4 | `abs 0xe69d0` | `-` |
| `ldr` at 0xe662c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6634 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6640 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6648 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6650 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6654 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6664 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xe6670 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe6678 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xe667c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe668c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6690 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe6698 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe66a0 | load | 4 | `abs 0xe69d4` | `-` |
| `ldr` at 0xe66b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe66c4 | load | 4 | `abs 0xe69d8` | `-` |
| `ldr` at 0xe66cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe66d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe66e0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe66e8 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe66f0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe66f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6704 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe670c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6714 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe6718 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6720 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xe672c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe6734 | store | 4 | `sp -0x24` | `0x11` |
| `ldr` at 0xe6738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6748 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe674c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe6754 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe675c | load | 4 | `abs 0xe69dc` | `-` |
| `ldr` at 0xe676c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6780 | load | 4 | `abs 0xe69e0` | `-` |
| `ldr` at 0xe6788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6790 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe679c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe67a4 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe67ac | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe67b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe67c0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe67c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe67d0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe67d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe67dc | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xe67e8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe67f0 | store | 4 | `sp -0x24` | `0x60` |
| `ldr` at 0xe67f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6804 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe6808 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe6810 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe6818 | load | 4 | `abs 0xe69e4` | `-` |
| `ldr` at 0xe6828 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe683c | load | 4 | `abs 0xe69e8` | `-` |
| `ldr` at 0xe6844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe684c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6858 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe6860 | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6868 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe686c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe687c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe6884 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe688c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe6890 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6898 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xe68a4 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe68ac | store | 4 | `sp -0x24` | `0x61` |
| `ldr` at 0xe68b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe68c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe68c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe68cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe68d4 | load | 4 | `abs 0xe69ec` | `-` |
| `ldr` at 0xe68e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe68f8 | load | 4 | `abs 0xe69f0` | `-` |
| `ldr` at 0xe6900 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6914 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe691c | store | 4 | `(unknown)` | `ret(0xe4524)` |
| `str` at 0xe6924 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe6928 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6938 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe6940 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6948 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe694c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6954 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0xe695c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6960 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe6964 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xe696c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe6970 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xe6978 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe6978 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe6978 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe6978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe62e0 | `(unknown callee) 0x221f0` | `0x1246d4` | `r1` | `r2` | `0x1246c8` |
| `bl` at 0xe62ec | `(unknown callee) 0x1943c` | `0xa8` | `?` | `?` | `?` |
| `bl` at 0xe6358 | `(unknown callee) 0xe4524` | `0x1246ec` | `0x7c` | `0x12ec70` | `?` |
| `bl` at 0xe63a8 | `(unknown callee) 0xe4524` | `0x1246f8` | `0x7d` | `0x12ec88` | `?` |
| `bl` at 0xe63f8 | `(unknown callee) 0xe4524` | `0x124704` | `0x7f` | `0x12eca0` | `?` |
| `bl` at 0xe6448 | `(unknown callee) 0xe4524` | `0x12470c` | `0x7e` | `0x12e44c` | `?` |
| `bl` at 0xe6498 | `(unknown callee) 0xe4524` | `0x124714` | `0x7` | `0x12e4b8` | `?` |
| `bl` at 0xe64e8 | `(unknown callee) 0xe4524` | `0x12471c` | `0x5` | `0x12e4f4` | `?` |
| `bl` at 0xe6538 | `(unknown callee) 0xe4524` | `0x124728` | `0x6` | `0x12e980` | `?` |
| `bl` at 0xe6588 | `(unknown callee) 0xe4524` | `0x124730` | `0xd` | `0x12ecb8` | `?` |
| `bl` at 0xe65d8 | `(unknown callee) 0xe4524` | `0x124740` | `0xc` | `0x12ecd0` | `?` |
| `bl` at 0xe6628 | `(unknown callee) 0xe4524` | `0x124750` | `0x4b` | `0x12ee50` | `?` |
| `bl` at 0xe66a8 | `(unknown callee) 0x197a8` | `sp-64` | `0x12475c` | `?` | `?` |
| `bl` at 0xe66c8 | `(unknown callee) 0xe4524` | `sp-64` | `?` | `0x12eb7c` | `?` |
| `bl` at 0xe6764 | `(unknown callee) 0x197a8` | `sp-64` | `0x124764` | `?` | `?` |
| `bl` at 0xe6784 | `(unknown callee) 0xe4524` | `sp-64` | `?` | `0x12edbc` | `?` |
| `bl` at 0xe6820 | `(unknown callee) 0x197a8` | `sp-64` | `0x124770` | `?` | `?` |
| `bl` at 0xe6840 | `(unknown callee) 0xe4524` | `sp-64` | `?` | `0x12ec58` | `?` |
| `bl` at 0xe68dc | `(unknown callee) 0x197a8` | `sp-64` | `0x124780` | `?` | `?` |
| `bl` at 0xe68fc | `(unknown callee) 0xe4524` | `sp-64` | `?` | `0x12ee34` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe62fc | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bhi` at 0xe631c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe638c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe63dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe642c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe647c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe64cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe651c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe656c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe65bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe660c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe665c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe6684 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe66fc | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe6740 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe67b8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe67fc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe6874 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe68b8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe6930 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe6978 | `r0` | `[sp-72]` |

## `llpcmd_from_tdr`

- `llpcmd_from_tdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe83b4 to 0xe8938.
  Established. The unnamed `0xdf6d8` call carries entry `r0`, entry `r1`, `0x0`, `sp-36` (`bl` at 0xe83e0).
  Included word stores write an unknown value to `[sp-28] +0x4` and `[sp-40]` to `[sp-28] +0xc` (`str` at 0xe8420, `str` at 0xe84b4).
  Link-command field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe83c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe83c8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe83cc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe83d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe83d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe83e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe83fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe840c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8414 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe8420 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xe8428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe842c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8444 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8450 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8458 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8460 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8478 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe8484 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xe8494 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe84a0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe84a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe84b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe84b4 | store | 4 | `[sp-28] +0xc` | `[sp-40]` |
| `ldr` at 0xe84bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe84c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe84d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe84e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe84f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe84fc | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe8508 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xe8510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8514 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8538 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe854c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe8558 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xe8560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8588 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85a0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe85ac | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xe85b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe85b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe85c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe85e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85f0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe85fc | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xe8604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8618 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe862c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe863c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8644 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe8650 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xe8658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe865c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe866c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8680 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8698 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe86a4 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xe86ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe86b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe86c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe86d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe86e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe86e8 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe86f4 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xe86fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8724 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8728 | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0xe8730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8734 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8744 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8770 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe877c | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xe8784 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe87bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87c4 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe87d0 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xe87d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe87dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe87ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8800 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xe8818 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe882c | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xe8834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe885c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xe8874 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8884 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xe888c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe88a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe88b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe88b8 | store | 4 | `[sp-28] +0x1c` | `?` |
| `ldr` at 0xe88c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe88d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe88d8 | store | 4 | `[sp-28] +0x20` | `?` |
| `ldr` at 0xe88e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe88e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe88f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8908 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe890c | store | 4 | `[sp-28] +0x24` | `?` |
| `ldr` at 0xe8914 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8928 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe892c | store | 4 | `[sp-28] +0x28` | `?` |
| `ldm` at 0xe8934 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe8934 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe8934 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe83e0 | `(unknown callee) 0xdf6d8` | `r0` | `r1` | `0x0` | `sp-36` |
| `bl` at 0xe8434 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xe84c8 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xe851c | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xe856c | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xe85c0 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xe8610 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xe8664 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xe86b8 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xe8708 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xe873c | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xe8790 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xe87e4 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xe8840 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xe8898 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xe88ec | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe8934 | `r0` | `sp-16` |

## `llpcmd_to_tdr`

- `llpcmd_to_tdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7f58 to 0xe83b4.
  Established. Included unnamed `0xdfc34` calls carry `[sp-20]`, `[sp-24]`, `0x0`, `sp-36` and `[sp-20]`, `[sp-24]`, `0xc`, `sp-36` (`bl` at 0xe7fac, `bl` at 0xe83a8).
  A word store clears an unknown effective target (`str` at 0xe7fc0).
  TDR command packing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7f68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7f6c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe7f70 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe7f84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7f8c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe7f98 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7fa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7fa4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7fc0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe7fd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7fdc | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe7fe8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8004 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe801c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xe802c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8038 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe805c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe8068 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8070 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8090 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8098 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe80a0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe80a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe80ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe80c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe80d0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe80dc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe80e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe80e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe810c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xe8114 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe811c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe813c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8144 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe8150 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe815c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8178 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8180 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe818c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe81b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe81bc | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe81c4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe81cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe81d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe81ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe81f4 | load | 4 | `[sp-28] +0x14` | `-` |
| `str` at 0xe81f8 | store | 4 | `(unknown)` | `[[sp-28]+20]` |
| `ldr` at 0xe8200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8228 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe8234 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe823c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe825c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8264 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xe8270 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe827c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8298 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe82a0 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xe82b0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe82b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe82bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe82d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe82e0 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xe82ec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe82f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe82f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8314 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe831c | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xe8320 | store | 4 | `(unknown)` | `[[sp-28]+28]` |
| `ldr` at 0xe8334 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe833c | load | 4 | `[sp-28] +0x20` | `-` |
| `str` at 0xe8340 | store | 4 | `(unknown)` | `[[sp-28]+32]` |
| `ldr` at 0xe8348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe834c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8368 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8370 | load | 4 | `[sp-28] +0x24` | `-` |
| `str` at 0xe8374 | store | 4 | `(unknown)` | `[[sp-28]+36]` |
| `ldr` at 0xe8388 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8390 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0xe8394 | store | 4 | `(unknown)` | `[[sp-28]+40]` |
| `ldr` at 0xe839c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe83a0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xe83b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe83b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe83b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7fac | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x0` | `sp-36` |
| `bl` at 0xe8040 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xe807c | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xe80b4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xe80f0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xe8128 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xe8164 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xe81a0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xe81d8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xe820c | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xe8248 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xe8284 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xe82c4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xe8300 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xe8354 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xe83a8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe83b0 | `r0` | `ret(0xdfc34)` |

## `JChip_read_link_state`

- `JChip_read_link_state`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6cc8 to 0xe7044.
  Established. The unnamed `0xe0540` call carries `[sp-20]`, `0x3`, `[sp-36]`, `0x0` (`bl` at 0xe6eb4).
  A signed nonnegative `ret(0xe0540)` takes `0xe6ed8`, while a negative value falls through (`bge` at 0xe6ec8).
  Word stores copy `[sp-44]` to `[sp-32] +0x0` and `[sp] +0x0` (`str` at 0xe6f4c, `str` at 0xe6f78).
  Link-state bit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6cd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe6cdc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe6ce0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe6ce4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe6ce8 | load | 4 | `abs 0xe7028` | `-` |
| `ldr` at 0xe6cec | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe6cf8 | load | 4 | `abs 0xe702c` | `-` |
| `ldr` at 0xe6cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6d00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6d04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6d0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6d18 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe6d1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe6d28 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe6d34 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe6d44 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe6d54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6d60 | load | 4 | `abs 0xe7030` | `-` |
| `str` at 0xe6d64 | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0xe6d6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6d74 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6d78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6d84 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe6d8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6d98 | load | 4 | `abs 0xe7030` | `-` |
| `str` at 0xe6d9c | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0xe6da4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6dac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6db0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6dbc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe6dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6dd0 | load | 4 | `abs 0xe7030` | `-` |
| `str` at 0xe6dd4 | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0xe6ddc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6de4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6de8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6df0 | load | 4 | `abs 0xe7034` | `-` |
| `str` at 0xe6df8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe6e00 | load | 4 | `abs 0xe7030` | `-` |
| `str` at 0xe6e04 | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0xe6e0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6e18 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe6e28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6e34 | store | 4 | `sp -0x30` | `ret(0xdefac)` |
| `ldr` at 0xe6e38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe6e44 | load | 4 | `abs 0xe7038` | `-` |
| `str` at 0xe6e48 | store | 4 | `sp -0x44` | `0xfffff7c6` |
| `ldr` at 0xe6e50 | load | 4 | `abs 0xe7028` | `-` |
| `ldr` at 0xe6e54 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe6e60 | load | 4 | `abs 0xe703c` | `-` |
| `ldr` at 0xe6e68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe6e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6e74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6e7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6e84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6e8c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6e98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6ea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6eac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6ebc | store | 4 | `sp -0x34` | `ret(0xe0540)` |
| `ldr` at 0xe6ec0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe6ecc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe6ed0 | store | 4 | `sp -0x44` | `ret(0xe0540)` |
| `ldr` at 0xe6ed8 | load | 4 | `abs 0xe7028` | `-` |
| `ldr` at 0xe6edc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe6ee8 | load | 4 | `abs 0xe7040` | `-` |
| `ldr` at 0xe6ef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6ef8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6f08 | store | 4 | `sp -0x34` | `ret(0xe0540)` |
| `ldr` at 0xe6f0c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe6f18 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe6f1c | store | 4 | `sp -0x44` | `ret(0xe0540)` |
| `ldr` at 0xe6f28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6f2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6f38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6f44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6f48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe6f4c | store | 4 | `[sp-32] +0x0` | `[sp-44]` |
| `ldr` at 0xe6f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6f58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6f64 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe6f70 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe6f74 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe6f78 | store | 4 | `[sp] +0x0` | `[sp-44]` |
| `ldr` at 0xe6f7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe6f80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6f88 | load | 4 | `[sp-48] +0x28` | `-` |
| `ldr` at 0xe6f90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6f98 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe6fac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6fb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6fc4 | store | 4 | `sp -0x38` | `ret(0xde16c)` |
| `ldr` at 0xe6fc8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6fd4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe6fe0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe6fe8 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xe6fec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe6ff0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe6ff8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe7004 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe7008 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe7010 | store | 4 | `[sp+4] +0x0` | `?` |
| `str` at 0xe7018 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xe701c | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0xe7024 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe7024 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe7024 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6d08 | `(unknown callee) 0x221f0` | `0x124790` | `r0` | `r1` | `r2` |
| `bl` at 0xe6e2c | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xe6e64 | `(unknown callee) 0x221f0` | `0x1247c8` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xe6e80 | `(unknown callee) 0xe5150` | `?` | `?` | `?` | `?` |
| `bl` at 0xe6ea0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-36]` | `0x1` | `sp-44` |
| `bl` at 0xe6eb4 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x3` | `[sp-36]` | `0x0` |
| `bl` at 0xe6eec | `(unknown callee) 0x221f0` | `0x1247ec` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xe6f00 | `(unknown callee) 0xe0540` | `[sp-20]` | `0xc` | `[sp-36]` | `0x0` |
| `bl` at 0xe6f34 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-36]` | `0x3` | `sp-44` |
| `bl` at 0xe6f60 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-36]` | `0x4` | `sp-44` |
| `bl` at 0xe6f94 | `(unknown callee) 0xe5150` | `?` | `[sp-48]` | `?` | `?` |
| `bl` at 0xe6fbc | `(unknown callee) 0xde16c` | `?` | `[sp-40]` | `sp-60` | `sp-64` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6cf4 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xe6d24 | `[sp-72] == 0x4` | `[sp-72] != 0x4` |
| `bgt` at 0xe6d30 | `[sp-72] > 0x4 (signed, 32-bit)` | `[sp-72] <= 0x4 (signed, 32-bit)` |
| `beq` at 0xe6d3c | `[sp-72] == 0x2` | `[sp-72] != 0x2` |
| `beq` at 0xe6d4c | `[sp-72] == 0x6` | `[sp-72] != 0x6` |
| `bls` at 0xe6d5c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xe6d94 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xe6dcc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe6e14 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe6e20 | `[sp] != 0x0` | `[sp] == 0x0` |
| `bne` at 0xe6e40 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bls` at 0xe6e5c | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0xe6ec8 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bls` at 0xe6ee4 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0xe6f14 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe6f40 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xe6f6c | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0xe6fa0 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `beq` at 0xe6fd0 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0xe6fdc | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `beq` at 0xe7000 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe7024 | `r0` | `?` |

## `JChip_read_dmin_dmax`

- `JChip_read_dmin_dmax`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6a34 to 0xe6cc8.
  Established. The unnamed `0xdfc34` call carries `[sp-20]`, `[sp-36]`, `0x1`, `sp-40` (`bl` at 0xe6bbc).
  A signed nonnegative `ret(0xe0540)` takes `0xe6bf4`, while a negative value falls through (`bge` at 0xe6be4).
  Included word stores copy `[sp-40]` to `[sp-32] +0x0` and `[sp] +0x0` (`str` at 0xe6c5c, `str` at 0xe6c7c).
  Delay-range encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6a44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe6a48 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe6a4c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe6a50 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe6a54 | load | 4 | `abs 0xe6cb0` | `-` |
| `ldr` at 0xe6a58 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe6a64 | load | 4 | `abs 0xe6cb4` | `-` |
| `ldr` at 0xe6a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6a6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6a70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6a78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6a84 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe6a88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6a94 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6aa0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6ab0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6ac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6acc | load | 4 | `abs 0xe6cb8` | `-` |
| `str` at 0xe6ad0 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xe6ad8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6ae0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6af4 | load | 4 | `abs 0xe6cb8` | `-` |
| `str` at 0xe6af8 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xe6b00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6b08 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6b10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe6b1c | load | 4 | `abs 0xe6cb8` | `-` |
| `str` at 0xe6b20 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xe6b28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6b30 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe6b38 | load | 4 | `abs 0xe6cb8` | `-` |
| `str` at 0xe6b3c | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xe6b44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6b50 | store | 4 | `sp -0x2c` | `ret(0xdefac)` |
| `ldr` at 0xe6b54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe6b60 | load | 4 | `abs 0xe6cbc` | `-` |
| `str` at 0xe6b64 | store | 4 | `sp -0x34` | `0xfffff7c6` |
| `ldr` at 0xe6b6c | load | 4 | `abs 0xe6cb0` | `-` |
| `ldr` at 0xe6b70 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe6b7c | load | 4 | `abs 0xe6cc0` | `-` |
| `ldr` at 0xe6b84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe6b88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6b90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe6ba0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6ba8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe6bb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6bc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6bc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6bd8 | store | 4 | `sp -0x30` | `ret(0xe0540)` |
| `ldr` at 0xe6bdc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe6be8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe6bec | store | 4 | `sp -0x34` | `ret(0xe0540)` |
| `ldr` at 0xe6bf4 | load | 4 | `abs 0xe6cb0` | `-` |
| `ldr` at 0xe6bf8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe6c04 | load | 4 | `abs 0xe6cc4` | `-` |
| `ldr` at 0xe6c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6c14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe6c24 | store | 4 | `sp -0x30` | `ret(0xe0540)` |
| `ldr` at 0xe6c28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe6c34 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe6c38 | store | 4 | `sp -0x34` | `ret(0xe0540)` |
| `ldr` at 0xe6c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6c48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6c54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe6c58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6c5c | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `ldr` at 0xe6c64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6c68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6c74 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe6c78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6c7c | store | 4 | `[sp] +0x0` | `[sp-40]` |
| `ldr` at 0xe6c80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe6c84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe6c8c | load | 4 | `[sp-44] +0x28` | `-` |
| `ldr` at 0xe6c94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe6ca0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe6ca4 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xe6cac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe6cac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe6cac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6a74 | `(unknown callee) 0x221f0` | `0x124790` | `r0` | `r1` | `r2` |
| `bl` at 0xe6b48 | `(unknown callee) 0xdefac` | `?` | `?` | `?` | `?` |
| `bl` at 0xe6b80 | `(unknown callee) 0x221f0` | `0x1247c8` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xe6b9c | `(unknown callee) 0xe5150` | `?` | `?` | `?` | `?` |
| `bl` at 0xe6bbc | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-36]` | `0x1` | `sp-40` |
| `bl` at 0xe6bd0 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x3` | `[sp-36]` | `0x0` |
| `bl` at 0xe6c08 | `(unknown callee) 0x221f0` | `0x1247ec` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xe6c1c | `(unknown callee) 0xe0540` | `[sp-20]` | `0xc` | `[sp-36]` | `0x0` |
| `bl` at 0xe6c50 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-36]` | `0x3` | `sp-40` |
| `bl` at 0xe6c70 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-36]` | `0x4` | `sp-40` |
| `bl` at 0xe6c98 | `(unknown callee) 0xe5150` | `?` | `[sp-44]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe6a60 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xe6a90 | `[sp-56] == 0x4` | `[sp-56] != 0x4` |
| `bgt` at 0xe6a9c | `[sp-56] > 0x4 (signed, 32-bit)` | `[sp-56] <= 0x4 (signed, 32-bit)` |
| `beq` at 0xe6aa8 | `[sp-56] == 0x2` | `[sp-56] != 0x2` |
| `beq` at 0xe6ab8 | `[sp-56] == 0x6` | `[sp-56] != 0x6` |
| `bls` at 0xe6ac8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xe6af0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xe6b18 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe6b5c | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bls` at 0xe6b78 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0xe6be4 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bls` at 0xe6c00 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0xe6c30 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe6cac | `r0` | `?` |

## `JChip_link_summary_f`

- `JChip_link_summary_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7044 to 0xe7540.
  Established. A byte store clears entry `r3 +0x0` (`strb` at 0xe7094).
  The unnamed `0xe6cc8` call carries `[sp-20]`, `[sp-24]`, `0x0`, unknown (`bl` at 0xe7110).
  Link summary field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7054 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7058 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe705c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe7060 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe7068 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe7070 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xe7078 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe707c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7080 | store | 4 | `sp -0x64` | `r3` |
| `ldr` at 0xe7084 | load | 4 | `abs 0xe7510` | `-` |
| `str` at 0xe7088 | store | 4 | `sp -0x68` | `0x124814` |
| `ldr` at 0xe708c | load | 4 | `sp -0x64` | `-` |
| `strb` at 0xe7094 | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0xe7098 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe70a8 | store | 4 | `sp -0x74` | `0x0` |
| `str` at 0xe70b8 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe70bc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe70cc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe70dc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe70e8 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xe70f0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe70fc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe7100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7104 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7118 | store | 4 | `sp -0x4c` | `ret(0xe6cc8)` |
| `ldr` at 0xe711c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe7128 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe712c | load | 4 | `abs 0xe7514` | `-` |
| `ldr` at 0xe7134 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe7138 | store | 4 | `sp -0x74` | `[sp-76]` |
| `ldr` at 0xe7140 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe7148 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xe7154 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe7160 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xe7164 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe7170 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe717c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xe718c | load | 4 | `sp -0x78` | `-` |
| `str` at 0xe71a0 | store | 4 | `sp -0x54` | `0x9` |
| `str` at 0xe71ac | store | 4 | `sp -0x54` | `0xe` |
| `str` at 0xe71b4 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe71b8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe71c4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe71dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe71e0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe71ec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe7204 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7208 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe721c | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xe7220 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe7224 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0xe7228 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe722c | load | 4 | `abs 0xe7518` | `-` |
| `ldr` at 0xe7230 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe7234 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe723c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe724c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe7254 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0xe7258 | load | 4 | `abs 0xe751c` | `-` |
| `str` at 0xe725c | store | 4 | `sp -0x68` | `0x124864` |
| `ldr` at 0xe7264 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe726c | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xe7274 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe7280 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe728c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe7290 | load | 4 | `abs 0xe7520` | `-` |
| `ldr` at 0xe7298 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe72a8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe72b0 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0xe72b4 | load | 4 | `abs 0xe751c` | `-` |
| `str` at 0xe72b8 | store | 4 | `sp -0x68` | `0x124864` |
| `str` at 0xe72c0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe72c4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe72d0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe72e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe72ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe72fc | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xe7300 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe7318 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe731c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe732c | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xe7330 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe7338 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xe7340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe734c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe7358 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe7364 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xe7374 | load | 4 | `abs 0xe7510` | `-` |
| `str` at 0xe7378 | store | 4 | `sp -0x6c` | `0x124814` |
| `ldr` at 0xe737c | load | 4 | `abs 0xe7510` | `-` |
| `str` at 0xe7380 | store | 4 | `sp -0x70` | `0x124814` |
| `ldr` at 0xe7384 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe7390 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xe739c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe73a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe73b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe73c0 | load | 4 | `abs 0xe7524` | `-` |
| `str` at 0xe73c4 | store | 4 | `sp -0x6c` | `0x124878` |
| `ldr` at 0xe73c8 | load | 4 | `abs 0xe7528` | `-` |
| `str` at 0xe73cc | store | 4 | `sp -0x70` | `0x124888` |
| `ldr` at 0xe73d0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe73dc | load | 4 | `abs 0xe752c` | `-` |
| `str` at 0xe73e0 | store | 4 | `sp -0x7c` | `0x12488c` |
| `ldr` at 0xe73e8 | load | 4 | `abs 0xe7530` | `-` |
| `str` at 0xe73ec | store | 4 | `sp -0x7c` | `0x1248ac` |
| `ldr` at 0xe73f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe73f4 | store | 4 | `sp -0x8c` | `[sp-44]` |
| `ldr` at 0xe73f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe73fc | store | 4 | `sp -0x88` | `[sp-40]` |
| `ldr` at 0xe7400 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7404 | store | 4 | `sp -0x84` | `[sp-36]` |
| `ldr` at 0xe7408 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe740c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xe7410 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe7414 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0xe741c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe742c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe7434 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0xe7438 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xe7444 | load | 4 | `abs 0xe7534` | `-` |
| `str` at 0xe7448 | store | 4 | `sp -0x80` | `0x1248c0` |
| `ldr` at 0xe7450 | load | 4 | `abs 0xe7538` | `-` |
| `str` at 0xe7454 | store | 4 | `sp -0x80` | `0x1248dc` |
| `ldr` at 0xe7458 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe745c | store | 4 | `sp -0x8c` | `[sp-48]` |
| `ldr` at 0xe7460 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe7464 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xe7468 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe746c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe7474 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe7484 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe748c | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0xe7490 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe7494 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xe749c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe74ac | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe74b4 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0xe74bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe74c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe74d4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe74d8 | load | 4 | `abs 0xe753c` | `-` |
| `ldr` at 0xe74e0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe74f0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe74f8 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `str` at 0xe7500 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0xe7504 | load | 4 | `sp -0x74` | `-` |
| `ldm` at 0xe750c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe750c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe750c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe70b0 | `(unknown callee) 0xd708c` | `r0` | `r1` | `r3` | `[sp-20]` |
| `bl` at 0xe7110 | `(unknown callee) 0xe6cc8` | `[sp-20]` | `[sp-24]` | `0x0` | `?` |
| `bl` at 0xe7130 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x124818` | `?` | `ret(0xe6cc8)` |
| `bl` at 0xe7150 | `(unknown callee) 0xd70fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xe7238 | `(unknown callee) 0x197a8` | `[sp-100]` | `0x12483c` | `[sp-68]` | `[sp-64]` |
| `bl` at 0xe7240 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xe7294 | `(unknown callee) 0x197a8` | `[sp-100]` | `0x124868` | `?` | `[sp-28]` |
| `bl` at 0xe729c | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xe7418 | `(unknown callee) 0x197a8` | `[sp-100]` | `[sp-124]` | `[sp-104]` | `?` |
| `bl` at 0xe7420 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xe7470 | `(unknown callee) 0x197a8` | `([sp-100] + ret(0x19538))` | `[sp-128]` | `[sp-56]` | `[sp-52]` |
| `bl` at 0xe7478 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xe7498 | `(unknown callee) 0x195e0` | `([sp-100] + ret(0x19538))` | `[sp-112]` | `ret(0x19538)` | `([sp-100] + ret(0x19538))` |
| `bl` at 0xe74a0 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xe74dc | `(unknown callee) 0x197a8` | `[sp-100]` | `0x1248e8` | `?` | `[sp-56]` |
| `bl` at 0xe74e4 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe70a0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bgt` at 0xe70c4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0xe7124 | `ret(0xe6cc8) >= 0x0 (signed, 32-bit)` | `ret(0xe6cc8) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe716c | `[sp-120] == 0x4` | `[sp-120] != 0x4` |
| `bgt` at 0xe7178 | `[sp-120] > 0x4 (signed, 32-bit)` | `[sp-120] <= 0x4 (signed, 32-bit)` |
| `beq` at 0xe7184 | `[sp-120] == 0x2` | `[sp-120] != 0x2` |
| `beq` at 0xe7194 | `[sp-120] == 0x6` | `[sp-120] != 0x6` |
| `bgt` at 0xe71c0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xe71e8 | `? != [sp-84]` | `? == [sp-84]` |
| `bne` at 0xe7210 | `? != [sp-68]` | `? == [sp-68]` |
| `bne` at 0xe727c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe7288 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `bgt` at 0xe72cc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xe72f4 | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xe7324 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xe7348 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xe7354 | `[sp-88] != 0x0` | `[sp-88] == 0x0` |
| `bne` at 0xe7360 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `bne` at 0xe736c | `[sp-96] != 0x0` | `[sp-96] == 0x0` |
| `bne` at 0xe738c | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `bne` at 0xe7398 | `[sp-96] != 0x0` | `[sp-96] == 0x0` |
| `bne` at 0xe73a4 | `[sp-88] != 0x0` | `[sp-88] == 0x0` |
| `bne` at 0xe73b0 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `bne` at 0xe73bc | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0xe73d8 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0xe7440 | `[sp-96] == 0x0` | `[sp-96] != 0x0` |
| `bne` at 0xe74c4 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `bne` at 0xe74d0 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe750c | `r0` | `?` |

## `JChip_link_summary_stub`

- `JChip_link_summary_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb900 to 0xdb954.
  Established. A byte assignment clears `[sp-32] +0x0` (`strb` at 0xdb930).
  The unnamed `0x1e4ac` calls carry `0x121a88`, entry `r1`, entry `r2`, entry `r3` and `0x121a8c`, unknown, `[sp-32]`, `0x0` (`bl` at 0xdb924, `bl` at 0xdb938).
  Link summary stub output meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb910 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb914 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdb918 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdb91c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdb920 | load | 4 | `abs 0xdb94c` | `-` |
| `ldr` at 0xdb928 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xdb930 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xdb934 | load | 4 | `abs 0xdb950` | `-` |
| `ldm` at 0xdb948 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb948 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb948 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb924 | `(unknown callee) 0x1e4ac` | `0x121a88` | `r1` | `r2` | `r3` |
| `bl` at 0xdb938 | `(unknown callee) 0x1e4ac` | `0x121a8c` | `?` | `[sp-32]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb948 | `r0` | `0x0` |

## `readsection`

- `readsection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd57b4 to 0xd583c.
  Established. The unnamed `0x19478` call carries entry `r0`, `0x1`, entry `r1`, entry `r2` (`bl` at 0xd57e0).
  Inequality `ret(0x19478) != [sp-24]` takes `0xd581c`, while equality falls through (`bne` at 0xd57f4).
  A byte read has an unknown effective target (`ldrb` at 0xd580c).
  Section terminator interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd57c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd57c8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd57cc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd57d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd57d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd57dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd57e4 | store | 4 | `sp -0x20` | `ret(0x19478)` |
| `ldr` at 0xd57e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd57ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd57fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5800 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd580c | load | 1 | `(unknown)` | `-` |
| `str` at 0xd5820 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xd582c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd5830 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xd5838 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd5838 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd5838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd57e0 | `(unknown callee) 0x19478` | `r0` | `0x1` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd57f4 | `ret(0x19478) != [sp-24]` | `ret(0x19478) == [sp-24]` |
| `bne` at 0xd5814 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd5838 | `r0` | `[sp-36]` |

## `tdrdata_get_data`

- `tdrdata_get_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4fac to 0xe5150.
  Established. Zero entry `r1` takes `0xe5090`, while nonzero falls through (`beq` at 0xe4fd0).
  Included word stores write unknown values to unknown effective targets (`str` at 0xe507c, `str` at 0xe5134).
  A word assignment increments `sp -0x20` with `[sp-32]+1` (`str` at 0xe5140).
  TDR data extraction bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4fbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe4fc0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe4fc4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe4fc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe4fd8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe4fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4fe0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe4fe4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4ffc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5000 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe5004 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe5014 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5028 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe502c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe5040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5050 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5064 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5068 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5070 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5078 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe507c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe5080 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe5088 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe5094 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe5098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe509c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe50a0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe50b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe50bc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe50c0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe50d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe50e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe50e4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe50e8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe50fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe510c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5114 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe511c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5120 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5130 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5134 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe5138 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe5140 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0xe514c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe514c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe514c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe4fd0 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xe4ff8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe501c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xe5048 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe50b4 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe50d8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xe5104 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe514c | `r0` | `?` |

## `tdrdata_set_data`

- `tdrdata_set_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4ed4 to 0xe4fac.
  Established. An included word store writes an unknown value to an unknown effective target (`str` at 0xe4f90).
  A word assignment increments `sp -0x1c` with `[sp-28]+1` (`str` at 0xe4f9c).
  The integer return candidate preserves entry `r0` (`ldm` at 0xe4fa8).
  TDR data replacement length remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4ee4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe4ee8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe4ef0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe4ef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4ef8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe4efc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4f14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4f18 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe4f1c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4f40 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe4f44 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4f58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4f6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4f74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4f7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4f84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4f8c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe4f90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe4f94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe4f9c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0xe4fa8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4fa8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4fa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe4f10 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe4f34 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xe4f60 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4fa8 | `r0` | `r0` |
