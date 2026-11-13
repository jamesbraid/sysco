# ARM packet, scan and image helpers in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/packet-scan-image-helpers-facts.txt) retain all tool sections.

## `IP57_Int`

- `IP57_Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57690 to 0x578d0.
  Established. The unnamed `0x25914` call carries `0x0`, `[sp-24]:1`, `[sp-25]:1`, `[sp-26]:1` (`bl` at 0x5770c).
  A zero `ret(0xca70c)` takes `0x57794`, while nonzero falls through (`beq` at 0x57748).
  The unnamed `0x2457c` call carries `0x0`, `[sp-23]:1`, `sp-36`, `[sp-23]:1` (`bl` at 0x577dc).
  IP57 interrupt source meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x576a4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x576a8 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x576b0 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0x576b8 | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0x576c0 | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `strb` at 0x576c8 | store | 1 | `sp -0x18` | `(r3 & 0xff)` |
| `strb` at 0x576d0 | store | 1 | `sp -0x19` | `([sp] & 0xff)` |
| `strb` at 0x576d8 | store | 1 | `sp -0x1a` | `([sp+4] & 0xff)` |
| `ldrb` at 0x576dc | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x576e8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x576f0 | store | 1 | `sp -0x1b` | `?` |
| `ldrb` at 0x576f4 | load | 1 | `sp -0x18` | `-` |
| `ldrb` at 0x576f8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x576fc | load | 1 | `sp -0x1a` | `-` |
| `ldrsb` at 0x57710 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x57724 | load | 1 | `sp -0x18` | `-` |
| `str` at 0x5773c | store | 4 | `sp -0x20` | `ret(0xca70c)` |
| `ldr` at 0x57740 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5774c | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x57760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5776c | load | 4 | `abs 0x578b0` | `-` |
| `str` at 0x57770 | store | 4 | `sp -0x38` | `0xf9310` |
| `str` at 0x57774 | store | 4 | `sp -0x34` | `ret(0xcb12c)` |
| `str` at 0x57778 | store | 4 | `sp -0x30` | `ret(0x1fc1c)` |
| `ldr` at 0x57780 | load | 4 | `abs 0x578b4` | `-` |
| `ldr` at 0x57784 | load | 4 | `abs 0x578b8` | `-` |
| `ldrb` at 0x57794 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x577a8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x577b0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x577b8 | load | 4 | `abs 0x578bc` | `-` |
| `ldrb` at 0x577cc | load | 1 | `sp -0x17` | `-` |
| `str` at 0x577e4 | store | 4 | `sp -0x20` | `ret(0x2457c)` |
| `ldr` at 0x577e8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x577f4 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x57808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57814 | load | 4 | `abs 0x578c0` | `-` |
| `str` at 0x57818 | store | 4 | `sp -0x38` | `0xf9378` |
| `str` at 0x5781c | store | 4 | `sp -0x34` | `ret(0x24628)` |
| `str` at 0x57820 | store | 4 | `sp -0x30` | `ret(0x1fc1c)` |
| `ldr` at 0x57828 | load | 4 | `abs 0x578b4` | `-` |
| `ldr` at 0x5782c | load | 4 | `abs 0x578c4` | `-` |
| `ldr` at 0x5783c | load | 4 | `abs 0x578c8` | `-` |
| `str` at 0x57840 | store | 4 | `sp -0x38` | `0xf939c` |
| `ldrb` at 0x57844 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5784c | store | 4 | `sp -0x34` | `[sp-21]:1+48` |
| `ldrb` at 0x57850 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x57858 | store | 4 | `sp -0x30` | `[sp-22]:1+65` |
| `ldr` at 0x5785c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57860 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x57864 | load | 4 | `sp -0x24` | `-` |
| `ldc` at 0x5786c | load | unknown | `abs 0x578a0` | `-` |
| `ldc` at 0x57874 | load | unknown | `abs 0x578a8` | `-` |
| `str` at 0x57880 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x57888 | load | 4 | `abs 0x578b4` | `-` |
| `ldr` at 0x5788c | load | 4 | `abs 0x578cc` | `-` |
| `ldm` at 0x5789c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5789c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5789c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5789c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5770c | `(unknown callee) 0x25914` | `0x0` | `[sp-24]:1` | `[sp-25]:1` | `[sp-26]:1` |
| `bl` at 0x57720 | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x57734 | `(unknown callee) 0xca70c` | `0x0` | `[sp-24]:1` | `sp-26` | `[sp-24]:1` |
| `bl` at 0x57758 | `(unknown callee) 0xcb12c` | `0x0` | `[sp-24]:1` | `?` | `[sp-24]:1` |
| `bl` at 0x57764 | `(unknown callee) 0x1fc1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5778c | `(unknown callee) 0x200a0` | `0x3` | `0xf9204` | `0x494` | `0x0` |
| `bl` at 0x577c4 | `(unknown callee) 0x1e958` | `0xf934c` | `[sp-21]:1+48` | `[sp-22]:1+65` | `[sp-22]:1+65` |
| `bl` at 0x577dc | `(unknown callee) 0x2457c` | `0x0` | `[sp-23]:1` | `sp-36` | `[sp-23]:1` |
| `bl` at 0x57800 | `(unknown callee) 0x24628` | `0x0` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x5780c | `(unknown callee) 0x1fc1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x57834 | `(unknown callee) 0x200a0` | `0x3` | `0xf9204` | `0x49e` | `0x0` |
| `bl` at 0x57894 | `(unknown callee) 0x200a0` | `0x1` | `0xf9204` | `0x4a1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x57718 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `beq` at 0x57748 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `bne` at 0x577a4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x577f0 | `ret(0x2457c) == 0x0` | `ret(0x2457c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5789c | `r0` | `?` |

## `build_ir_access_chip`

- `build_ir_access_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf0f0 to 0xdf1b0.
  Established. Included word assignments write `0x0` and `[0x2973a8]+1` to `0x2973a8` (`str` at 0xdf13c, `str` at 0xdf180).
  The unsigned comparison `[0x2973a8] >= [sp-36]` takes `0xdf188`, while lower falls through (`bhs` at 0xdf150).
  The unnamed `0xd5e9c` call carries `0x0`, `[sp-32]`, `[sp-24]`, `[sp]` (`bl` at 0xdf1a0).
  Instruction-register bit ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf100 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf104 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf108 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf10c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf124 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf130 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdf134 | load | 4 | `abs 0xdf1ac` | `-` |
| `str` at 0xdf13c | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xdf140 | load | 4 | `abs 0xdf1ac` | `-` |
| `ldr` at 0xdf144 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xdf148 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf154 | load | 4 | `abs 0xdf1ac` | `-` |
| `ldr` at 0xdf158 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xdf160 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xdf16c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf170 | load | 4 | `abs 0xdf1ac` | `-` |
| `ldr` at 0xdf174 | load | 4 | `abs 0xdf1ac` | `-` |
| `ldr` at 0xdf178 | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xdf180 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xdf188 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf18c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xdf194 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf19c | load | 4 | `sp +0x0` | `-` |
| `ldm` at 0xdf1a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf1a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf1a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf1a0 | `(unknown callee) 0xd5e9c` | `0x0` | `[sp-32]` | `[sp-24]` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xdf150 | `[0x2973a8] >= [sp-36] (unsigned, 32-bit)` | `[0x2973a8] < [sp-36] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf1a8 | `r0` | `ret(0xd5e9c)` |

## `build_dr_access_chip`

- `build_dr_access_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf374 to 0xdf464.
  Established. A word assignment writes `(r0 + r2)-1` to `[sp] +0x0` (`str` at 0xdf3a8).
  Equality `[sp-32] == [sp+4]` takes `0xdf458`, while inequality falls through (`beq` at 0xdf3b8).
  The unnamed `0xd5e9c` call carries `0x0`, `[sp-32]`, `[sp-24]`, `[sp+4]` (`bl` at 0xdf454).
  Data-register vector meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf384 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf388 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf38c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf390 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf394 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xdf398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf39c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf3a8 | store | 4 | `[sp] +0x0` | `(r0 + r2)-1` |
| `ldr` at 0xdf3ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf3b0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xdf3bc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xdf3c0 | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xdf3d4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xdf3d8 | load | 4 | `[sp] +0x0` | `-` |
| `str` at 0xdf3e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdf3e8 | load | 4 | `abs 0xdf460` | `-` |
| `str` at 0xdf3f0 | store | 4 | `abs 0x2973a8` | `0x0` |
| `ldr` at 0xdf3f4 | load | 4 | `abs 0xdf460` | `-` |
| `ldr` at 0xdf3f8 | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xdf3fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf408 | load | 4 | `abs 0xdf460` | `-` |
| `ldr` at 0xdf40c | load | 4 | `abs 0x2973a8` | `-` |
| `ldr` at 0xdf414 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xdf420 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdf424 | load | 4 | `abs 0xdf460` | `-` |
| `ldr` at 0xdf428 | load | 4 | `abs 0xdf460` | `-` |
| `ldr` at 0xdf42c | load | 4 | `abs 0x2973a8` | `-` |
| `str` at 0xdf434 | store | 4 | `abs 0x2973a8` | `[0x2973a8]+1` |
| `ldr` at 0xdf43c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf440 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xdf448 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf44c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdf450 | load | 4 | `sp +0x4` | `-` |
| `ldm` at 0xdf45c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdf45c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdf45c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf454 | `(unknown callee) 0xd5e9c` | `0x0` | `[sp-32]` | `[sp-24]` | `[sp+4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdf3b8 | `[sp-32] == [sp+4]` | `[sp-32] != [sp+4]` |
| `bhs` at 0xdf404 | `[0x2973a8] >= [sp-36] (unsigned, 32-bit)` | `[0x2973a8] < [sp-36] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdf45c | `r0` | `?` |

## `idcode_fill`

- `idcode_fill`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc584 to 0xdc69c.
  Established. Included word assignments write `[sp-28]` to `r1 +0x0`, `[sp-32]` to `[sp-24] +0x0` and `[sp-40]` to `[sp-24] +0x0` (`str` at 0xdc5cc, `str` at 0xdc610, `str` at 0xdc688).
  The integer return candidate is `[[sp-24]]` (`ldm` at 0xdc690).
  Identification record field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc594 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc598 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdc59c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc5a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc5ac | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xdc5b0 | store | 4 | `sp -0x1c` | `[r1]` |
| `ldr` at 0xdc5b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc5c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdc5c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc5cc | store | 4 | `r1 +0x0` | `[sp-28]` |
| `ldr` at 0xdc5d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc5d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc5e4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xdc5e8 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0xdc5ec | load | 4 | `abs 0xdc694` | `-` |
| `ldr` at 0xdc5f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc608 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc60c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc610 | store | 4 | `[sp-24] +0x0` | `[sp-32]` |
| `ldr` at 0xdc614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc628 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xdc62c | store | 4 | `sp -0x24` | `[[sp-24]]` |
| `ldr` at 0xdc630 | load | 4 | `abs 0xdc698` | `-` |
| `ldr` at 0xdc63c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc64c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc650 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc654 | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `ldr` at 0xdc658 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc664 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xdc668 | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0xdc670 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdc680 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdc684 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdc688 | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `ldm` at 0xdc690 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc690 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc690 | `r0` | `[[sp-24]]` |

## `vector_write_set_bits`

- `vector_write_set_bits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8d24 to 0xe8e78.
  Established. A nonzero entry `r0` takes `0xe8d64`, while zero falls through (`bne` at 0xe8d54).
  An included word read addresses `0x12eeb4` (`ldr` at 0xe8d5c).
  The unsigned condition `ret(0xeab90) >= [sp-28]` takes `0xe8e18`, while lower falls through (`bhs` at 0xe8dcc).
  Set-bit formatting destination semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8d34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe8d38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe8d3c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe8d40 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe8d44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe8d48 | store | 4 | `sp -0x2c` | `r0` |
| `ldr` at 0xe8d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8d58 | load | 4 | `abs 0xe8e60` | `-` |
| `ldr` at 0xe8d5c | load | 4 | `abs 0x12eeb4` | `-` |
| `str` at 0xe8d60 | store | 4 | `sp -0x2c` | `[0x12eeb4]` |
| `ldr` at 0xe8d64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8d68 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe8d6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8d7c | load | 4 | `abs 0xe8e64` | `-` |
| `ldr` at 0xe8d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8d8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8d90 | load | 4 | `abs 0xe8e68` | `-` |
| `str` at 0xe8d9c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe8da4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe8da8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe8dac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8db0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8dbc | store | 4 | `sp -0x24` | `ret(0xeab90)` |
| `ldr` at 0xe8dc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe8dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8dd4 | load | 4 | `abs 0xe8e6c` | `-` |
| `ldr` at 0xe8dd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe8de0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8de8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe8dec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8df4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe8df8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8e0c | load | 4 | `abs 0xe8e70` | `-` |
| `ldr` at 0xe8e18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8e24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8e28 | load | 4 | `abs 0xe8e74` | `-` |
| `ldr` at 0xe8e34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8e44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8e48 | load | 4 | `abs 0xe8e70` | `-` |
| `ldr` at 0xe8e50 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xe8e5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe8e5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe8e5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8d84 | `(unknown callee) 0x222f0` | `?` | `0x1249b8` | `r1` | `r1` |
| `bl` at 0xe8d94 | `(unknown callee) 0x222f0` | `?` | `0x1249dc` | `r2` | `r1` |
| `bl` at 0xe8db4 | `(unknown callee) 0xeab90` | `?` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0xe8ddc | `(unknown callee) 0x222f0` | `[sp-20]` | `0x1249fc` | `ret(0xeab90)` | `[sp-28]` |
| `bl` at 0xe8e10 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x124a04` | `?` | `?` |
| `bl` at 0xe8e2c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x124a08` | `ret(0xeab90)` | `[sp-40]` |
| `bl` at 0xe8e4c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x124a04` | `ret(0xeab90)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8d54 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xe8d74 | `r1 == 0x0` | `r1 != 0x0` |
| `bhs` at 0xe8dcc | `ret(0xeab90) >= [sp-28] (unsigned, 32-bit)` | `ret(0xeab90) < [sp-28] (unsigned, 32-bit)` |
| `bne` at 0xe8e04 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe8e20 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xe8e40 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe8e5c | `r0` | `[sp-40]` |

## `readhead`

- `readhead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5468 to 0xd5660.
  Established. A zero `ret(0xd5660)` takes `0xd54a8`, while nonzero falls through (`beq` at 0xd5498).
  Included word assignments write `ret(0x1943c)` to `[sp-24] +0x0` and `[sp-24] +0xc` (`str` at 0xd54e4, `str` at 0xd55f8).
  A later word assignment writes `[sp-32]` to `[sp-24] +0x10` (`str` at 0xd5648).
  Image header record ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5478 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd547c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xd5480 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd548c | store | 4 | `sp -0x20` | `ret(0xd5660)` |
| `ldr` at 0xd5490 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd549c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd54a0 | store | 4 | `sp -0x24` | `ret(0xd5660)` |
| `ldr` at 0xd54ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd54b8 | store | 4 | `sp -0x20` | `ret(0xd56f4)` |
| `ldr` at 0xd54bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd54c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd54cc | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xd54d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd54e4 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xd54e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54ec | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xd54f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd54f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5500 | store | 4 | `sp -0x20` | `ret(0xd57b4)` |
| `ldr` at 0xd5508 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5514 | store | 4 | `sp -0x20` | `ret(0xd56f4)` |
| `ldr` at 0xd5518 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5524 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5528 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xd5530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5534 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5540 | store | 4 | `[sp-24] +0x4` | `ret(0x1943c)` |
| `ldr` at 0xd5544 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5548 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd554c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5550 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd555c | store | 4 | `sp -0x20` | `ret(0xd57b4)` |
| `ldr` at 0xd5564 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5570 | store | 4 | `sp -0x20` | `ret(0xd56f4)` |
| `ldr` at 0xd5574 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5580 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5584 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xd558c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5590 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd559c | store | 4 | `[sp-24] +0x8` | `ret(0x1943c)` |
| `ldr` at 0xd55a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd55a4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xd55a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd55ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd55b8 | store | 4 | `sp -0x20` | `ret(0xd57b4)` |
| `ldr` at 0xd55c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd55cc | store | 4 | `sp -0x20` | `ret(0xd56f4)` |
| `ldr` at 0xd55d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd55dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd55e0 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xd55e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd55ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd55f8 | store | 4 | `[sp-24] +0xc` | `ret(0x1943c)` |
| `ldr` at 0xd55fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5600 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xd5604 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5608 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5614 | store | 4 | `sp -0x20` | `ret(0xd57b4)` |
| `ldr` at 0xd5618 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5624 | store | 4 | `sp -0x20` | `ret(0xd583c)` |
| `ldr` at 0xd5628 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5634 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5638 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xd5640 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5644 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5648 | store | 4 | `[sp-24] +0x10` | `[sp-32]` |
| `str` at 0xd5650 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd5654 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xd565c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd565c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd565c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd565c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5484 | `(unknown callee) 0xd5660` | `r1` | `r1` | `r2` | `r3` |
| `bl` at 0xd54b0 | `(unknown callee) 0xd56f4` | `0x0` | `[sp-28]` | `?` | `ret(0xd5660)` |
| `bl` at 0xd54dc | `(unknown callee) 0x1943c` | `[sp-32]` | `?` | `?` | `ret(0xd56f4)` |
| `bl` at 0xd54f8 | `(unknown callee) 0xd57b4` | `[[sp-24]]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xd550c | `(unknown callee) 0xd56f4` | `0x0` | `[sp-28]` | `?` | `ret(0xd57b4)` |
| `bl` at 0xd5538 | `(unknown callee) 0x1943c` | `[sp-32]` | `?` | `?` | `ret(0xd56f4)` |
| `bl` at 0xd5554 | `(unknown callee) 0xd57b4` | `[[sp-24]+4]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xd5568 | `(unknown callee) 0xd56f4` | `0x0` | `[sp-28]` | `?` | `ret(0xd57b4)` |
| `bl` at 0xd5594 | `(unknown callee) 0x1943c` | `[sp-32]` | `?` | `?` | `ret(0xd56f4)` |
| `bl` at 0xd55b0 | `(unknown callee) 0xd57b4` | `[[sp-24]+8]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xd55c4 | `(unknown callee) 0xd56f4` | `0x0` | `[sp-28]` | `?` | `ret(0xd57b4)` |
| `bl` at 0xd55f0 | `(unknown callee) 0x1943c` | `[sp-32]` | `?` | `?` | `ret(0xd56f4)` |
| `bl` at 0xd560c | `(unknown callee) 0xd57b4` | `[[sp-24]+12]` | `[sp-32]` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xd561c | `(unknown callee) 0xd583c` | `[sp-28]` | `?` | `?` | `ret(0xd57b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd5498 | `ret(0xd5660) == 0x0` | `ret(0xd5660) != 0x0` |
| `bne` at 0xd54c4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd5520 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd557c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd55d8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd5630 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd565c | `r0` | `?` |

## `readhead13`

- `readhead13`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5660 to 0xd56f4.
  Established. The unnamed `0x19478` call carries `sp-40`, `0x1`, `0xd`, entry `r0` (`bl` at 0xd5688).
  Equality `ret(0x19478) == 0xd` takes `0xd56a8`, while inequality falls through (`beq` at 0xd5698).
  Another unnamed `0x1955c` call carries `sp-40`, `0x12da54`, `0xd`, `sp-40` (`bl` at 0xd56b8).
  Header signature meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5670 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd5684 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd568c | store | 4 | `sp -0x18` | `ret(0x19478)` |
| `ldr` at 0xd5690 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd56a0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd56b0 | load | 4 | `abs 0xd56f0` | `-` |
| `str` at 0xd56c0 | store | 4 | `sp -0x18` | `ret(0x1955c)` |
| `ldr` at 0xd56c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd56d4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xd56e0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd56e4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xd56ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd56ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd56ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5688 | `(unknown callee) 0x19478` | `sp-40` | `0x1` | `0xd` | `r0` |
| `bl` at 0xd56b8 | `(unknown callee) 0x1955c` | `sp-40` | `0x12da54` | `0xd` | `sp-40` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd5698 | `ret(0x19478) == 0xd` | `ret(0x19478) != 0xd` |
| `beq` at 0xd56cc | `ret(0x1955c) == 0x0` | `ret(0x1955c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd56ec | `r0` | `[sp-44]` |

## `readsecthead`

- `readsecthead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd56f4 to 0xd57b4.
  Established. A byte read addresses `[sp-20] +0x1` (`ldrb` at 0xd5750);
  its nonzero value takes `0xd5760`, while zero falls through (`bne` at 0xd5758).
  The unnamed `0x19478` call carries `sp-33`, `0x1`, `0x1`, `[sp-24]` (`bl` at 0xd5780).
  Section tag encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5704 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd5708 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd570c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd571c | store | 4 | `sp -0x14` | `sp-32` |
| `ldr` at 0xd5720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd572c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5734 | store | 4 | `sp -0x1c` | `ret(0x19478)` |
| `ldr` at 0xd5738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5748 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd5750 | load | 1 | `[sp-20] +0x1` | `-` |
| `str` at 0xd5764 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd577c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5784 | store | 4 | `sp -0x1c` | `ret(0x19478)` |
| `ldr` at 0xd5788 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5798 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xd57a0 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xd57a4 | store | 4 | `sp -0x28` | `[sp-33]:1` |
| `ldr` at 0xd57a8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xd57b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd57b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd57b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5730 | `(unknown callee) 0x19478` | `?` | `0x1` | `0x2` | `r1` |
| `bl` at 0xd5780 | `(unknown callee) 0x19478` | `sp-33` | `0x1` | `0x1` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5714 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd5740 | `ret(0x19478) != 0x2` | `ret(0x19478) == 0x2` |
| `bne` at 0xd5758 | `[[sp-20]+1]:1 != 0x0` | `[[sp-20]+1]:1 == 0x0` |
| `beq` at 0xd5790 | `ret(0x19478) == 0x1` | `ret(0x19478) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd57b0 | `r0` | `[sp-40]` |

## `readlength`

- `readlength`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd583c to 0xd5914.
  Established. The unnamed `0x19478` call carries `sp-24`, `0x1`, `0x2`, entry `r0` (`bl` at 0xd5864).
  A return unequal to `0x2` takes `0xd589c`, while equality falls through (`bne` at 0xd5874).
  Included byte reads address `sp -0x1c`, `sp -0x1b` and `sp -0x1a` (`ldrb` at 0xd58dc, `ldrb` at 0xd58e4, `ldrb` at 0xd58f0).
  Encoded length units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd584c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd5860 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5868 | store | 4 | `sp -0x24` | `ret(0x19478)` |
| `ldr` at 0xd586c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5880 | load | 4 | `abs 0xd5910` | `-` |
| `str` at 0xd58a0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd58b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd58c0 | store | 4 | `sp -0x24` | `ret(0x19478)` |
| `ldr` at 0xd58c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd58d4 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xd58dc | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xd58e4 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd58f0 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd58f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd58fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5900 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0xd5904 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xd590c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd590c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd590c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5864 | `(unknown callee) 0x19478` | `sp-24` | `0x1` | `0x2` | `r0` |
| `bl` at 0xd5888 | `(unknown callee) 0x1955c` | `sp-24` | `0x12132c` | `0x2` | `sp-24` |
| `bl` at 0xd58bc | `(unknown callee) 0x19478` | `sp-28` | `0x1` | `0x3` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5874 | `ret(0x19478) != 0x2` | `ret(0x19478) == 0x2` |
| `bne` at 0xd5894 | `ret(0x1955c) != 0x0` | `ret(0x1955c) == 0x0` |
| `beq` at 0xd58cc | `ret(0x19478) == 0x3` | `ret(0x19478) != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd590c | `r0` | `[sp-40]` |

## `verify_length_data`

- `verify_length_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd497c to 0xd4b48.
  Established. A word assignment clears `r3 +0x0` to `0x0` (`str` at 0xd49a4).
  A later word assignment writes an unknown value to `[sp-32] +0x0` (`str` at 0xd4b30).
  The predicate `[sp-48] != 0x80000000` takes `0xd4a54`, while equality falls through (`bne` at 0xd4a38).
  Scan length validation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd498c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd4990 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd4994 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd4998 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd499c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd49a4 | store | 4 | `r3 +0x0` | `0x0` |
| `str` at 0xd49ac | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd49b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd49c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd49cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd49ec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd49f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd49f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4a00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd4a04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4a14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4a18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4a24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4a2c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd4a30 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4a40 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd4a44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4a4c | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0xd4a54 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4a5c | store | 4 | `sp -0x30` | `?` |
| `str` at 0xd4a68 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd4a6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4a7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd4a80 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4a90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4a94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4aa0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4aa8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd4aac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4abc | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd4ac0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4ac8 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0xd4ad0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4ad8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4ae0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4ae4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4af0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4b00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd4b04 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd4b18 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd4b20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4b28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4b30 | store | 4 | `[sp-32] +0x0` | `?` |
| `str` at 0xd4b38 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xd4b3c | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xd4b44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd4b44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd4b44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd4a10 | `? == 0x0` | `? != 0x0` |
| `bhs` at 0xd4a20 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd4a38 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xd4a8c | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xd4a9c | `C == 1` | `not (C == 1)` |
| `bne` at 0xd4ab4 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xd4aec | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd4b10 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd4b44 | `r0` | `?` |

## `PktGenRead`

- `PktGenRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a344 to 0x5a414.
  Established. The unnamed `0x58b64` call carries `0x0`, `[sp-20]`, `[sp-24]`, `[sp-28]` (`bl` at 0x5a3bc).
  A nonzero result takes `0x5a404`, while zero falls through (`bne` at 0x5a3d0).
  Included word stores write unknown values to `[sp+8] +0x0` and `[sp+12] +0x0` (`str` at 0x5a3e8, `str` at 0x5a400).
  Packet read result encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a354 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5a358 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5a35c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x5a360 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x5a370 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x5a374 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x5a384 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x5a388 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x5a38c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5a390 | store | 4 | `sp -0x44` | `[sp-32]` |
| `str` at 0x5a398 | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0x5a3a0 | store | 4 | `sp -0x3c` | `sp-44` |
| `str` at 0x5a3a8 | store | 4 | `sp -0x38` | `sp-52` |
| `ldr` at 0x5a3b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a3b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a3b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5a3c4 | store | 4 | `sp -0x24` | `ret(0x58b64)` |
| `ldr` at 0x5a3c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5a3d4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x5a3e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5a3e8 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0x5a3ec | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x5a3fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x5a400 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0x5a404 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x5a410 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5a410 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5a410 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5a3bc | `(unknown callee) 0x58b64` | `0x0` | `[sp-20]` | `[sp-24]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a3d0 | `ret(0x58b64) != 0x0` | `ret(0x58b64) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5a410 | `r0` | `?` |

## `PktGenWrite`

- `PktGenWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a414 to 0x5a4d0.
  Established. The unnamed `0x58b64` call carries `0x1`, `[sp-20]`, `[sp-24]`, `[sp-28]` (`bl` at 0x5a4b4).
  Included word assignments write `0x1` to `sp -0x40`, `sp-44` to `sp -0x3c` and `sp-52` to `sp -0x38` (`str` at 0x5a490, `str` at 0x5a498, `str` at 0x5a4a0).
  Packet write parameter ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a424 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5a428 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5a42c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x5a430 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x5a440 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x5a444 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x5a454 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x5a458 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x5a468 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x5a46c | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x5a47c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x5a480 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x5a484 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5a488 | store | 4 | `sp -0x44` | `[sp-32]` |
| `str` at 0x5a490 | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0x5a498 | store | 4 | `sp -0x3c` | `sp-44` |
| `str` at 0x5a4a0 | store | 4 | `sp -0x38` | `sp-52` |
| `ldr` at 0x5a4a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a4ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a4b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5a4bc | store | 4 | `sp -0x24` | `ret(0x58b64)` |
| `ldr` at 0x5a4c0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x5a4cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5a4cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5a4cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5a4b4 | `(unknown callee) 0x58b64` | `0x1` | `[sp-20]` | `[sp-24]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5a4cc | `r0` | `ret(0x58b64)` |

## `initbh`

- `initbh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd53c4 to 0xd5418.
  Established. Included word assignments clear `r0 +0x0`, `[sp-20] +0x4`, `[sp-20] +0x8`, `[sp-20] +0xc` and `[sp-20] +0x10` to `0x0` (`str` at 0xd53e0, `str` at 0xd53ec, `str` at 0xd53f8, `str` at 0xd5404, `str` at 0xd5410).
  The integer return record preserves entry `r0` (`ldm` at 0xd5414).
  Buffer-header field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd53d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd53d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd53e0 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xd53e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd53ec | store | 4 | `[sp-20] +0x4` | `0x0` |
| `ldr` at 0xd53f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd53f8 | store | 4 | `[sp-20] +0x8` | `0x0` |
| `ldr` at 0xd53fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5404 | store | 4 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0xd5408 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5410 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldm` at 0xd5414 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd5414 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd5414 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd5414 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd5414 | `r0` | `r0` |

## `freebh`

- `freebh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5418 to 0xd5468.
  Established. Included unnamed `0x1970c` calls carry `[r0]`, entry `r1`, entry `r2`, entry `r0` and `[[sp-20]+12]`, unknown, unknown, `[sp-20]` (`bl` at 0xd5434, `bl` at 0xd5458).
  The unnamed `0xd53c4` call carries `[sp-20]` followed by three unknown candidates (`bl` at 0xd5460).
  Referenced allocation ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5428 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd542c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5430 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xd5438 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd543c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xd5444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5448 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xd5450 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5454 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xd545c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd5464 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd5464 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd5464 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd5464 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5434 | `(unknown callee) 0x1970c` | `[r0]` | `r1` | `r2` | `r0` |
| `bl` at 0xd5440 | `(unknown callee) 0x1970c` | `[[sp-20]+4]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xd544c | `(unknown callee) 0x1970c` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xd5458 | `(unknown callee) 0x1970c` | `[[sp-20]+12]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xd5460 | `(unknown callee) 0xd53c4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd5464 | `r0` | `ret(0xd53c4)` |

## `ipmiInit`

- `ipmiInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1ff2c to 0x1ff3c.
  Established. The return record preserves entry `r0` (`ldm` at 0x1ff38).
  IPMI initialization has Unknown device responsibilities.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x1ff38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1ff38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1ff38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1ff38 | `r0` | `r0` |
