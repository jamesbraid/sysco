# ARM expander device reset and interrupt paths in 1.44.0

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
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/expander-device-reset-facts.txt) retain all tool sections.

## `mic74_Reset`

- `mic74_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdb5c to 0xcde28.
  Established. The unnamed `0xbb498` call at `0xcdbb8` carries unknown, `[sp-26]:1`, `0x4`, `0x0`
  (`bl` at 0xcdbb8); its later call at `0xcdcd8` carries unknown, `[sp-26]:1`, `0x5`,
  `[[sp-24]+6]:1` (`bl` at 0xcdcd8). An included word assignment at `sp -0x30` writes `0x1` (`str`
  at 0xcdc14). Device reset ordering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdb6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdb70 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcdb74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdb78 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0xcdb7c | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0xcdb80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdb84 | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0xcdb88 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0xcdb8c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcdb90 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdba4 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xcdbc0 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcdbc4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcdbc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdbdc | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xcdbe0 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xcdbe4 | store | 4 | `sp -0x2c` | `[sp-26]:1` |
| `str` at 0xcdbec | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcdbf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdbf4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcdc00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdc04 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xcdc14 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xcdc18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcdc1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcdc24 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xcdc30 | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcdc34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdc40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdc4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdc50 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcdc54 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcdc58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdc6c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xcdc70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdc74 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0xcdc88 | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcdc8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdc98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdca4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdca8 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcdcac | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcdcb0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdcc4 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xcdcc8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdccc | load | 1 | `[sp-24] +0x6` | `-` |
| `str` at 0xcdce0 | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcdce4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdcf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdcfc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdd00 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcdd04 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcdd08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdd1c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xcdd20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdd24 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xcdd40 | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcdd44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdd50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdd5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdd60 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcdd64 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcdd68 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdd7c | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xcdd98 | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcdd9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdda8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcddb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcddb8 | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcddbc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcddc0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcddd4 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xcddf4 | store | 4 | `sp -0x24` | `ret(0xbb5c4)` |
| `ldr` at 0xcddf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcde04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcde10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcde14 | store | 4 | `sp -0x20` | `ret(0xbb5c4)` |
| `ldr` at 0xcde18 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xcde24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcde24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcde24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdb98 | `(unknown callee) 0xbac78` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0xcdbb8 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x4` | `0x0` |
| `bl` at 0xcdbd0 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcdc28 | `(unknown callee) 0xbb498` | `[sp-40]` | `[sp-26]:1` | `0x0` | `?` |
| `bl` at 0xcdc60 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcdc80 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0xcdcb8 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcdcd8 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x5` | `[[sp-24]+6]:1` |
| `bl` at 0xcdd10 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcdd38 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x1` | `?` |
| `bl` at 0xcdd70 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcdd90 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x6` | `0x0` |
| `bl` at 0xcddc8 | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcddec | `(unknown callee) 0xbb5c4` | `?` | `[sp-26]:1` | `0x3` | `sp-27` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcdbfc | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |
| `beq` at 0xcdc0c | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |
| `beq` at 0xcdc3c | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcdc48 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xcdc94 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcdca0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xcdcec | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcdcf8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xcdd4c | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcdd58 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xcdda4 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcddb0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xcde00 | `ret(0xbb5c4) == 0x0` | `ret(0xbb5c4) != 0x0` |
| `bne` at 0xcde0c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcde24 | `r0` | `[sp-32]` |

## `mic74_PreReset`

- `mic74_PreReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcde28 to 0xcdf00.
  Established. The unnamed `0xbb498` call at `0xcde84` carries unknown, `[sp-26]:1`, `0x2`,
  `[[sp-24]+5]:1` (`bl` at 0xcde84); its later call carries unknown, `[sp-26]:1`, `0x1`, unknown
  (`bl` at 0xcdec4). A zero later result takes `0xcdef0`, while nonzero falls through (`beq` at 0xcded8). Reset preparation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcde38 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcde3c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcde40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcde44 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0xcde48 | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0xcde4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcde50 | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0xcde54 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0xcde58 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcde5c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcde70 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xcde74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcde78 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0xcde8c | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldrb` at 0xcde90 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcde94 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xcdea8 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xcdeac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdeb0 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xcdecc | store | 4 | `sp -0x24` | `ret(0xbb498)` |
| `ldr` at 0xcded0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdedc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdee8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdeec | store | 4 | `sp -0x20` | `ret(0xbb498)` |
| `ldr` at 0xcdef0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xcdefc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcdefc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcdefc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcde64 | `(unknown callee) 0xbac78` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0xcde84 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0xcde9c | `(unknown callee) 0xbac78` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xcdec4 | `(unknown callee) 0xbb498` | `?` | `[sp-26]:1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcded8 | `ret(0xbb498) == 0x0` | `ret(0xbb498) != 0x0` |
| `bne` at 0xcdee4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcdefc | `r0` | `[sp-32]` |

## `mic74_IntSet`

- `mic74_IntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdf00 to 0xcdf6c.
  Established. The unnamed `0xbb498` call carries unknown, `[[sp-24]+2]:1`, `0x4`, `[sp-25]:1` (`bl`
  at 0xcdf58). The byte through entry `r1 +0x1` is read (`ldrb` at 0xcdf24). The included return
  record carries `ret(0xbb498)` in `r0` (`ldm` at 0xcdf68). Device interrupt configuration remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdf10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdf14 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xcdf1c | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xcdf20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdf24 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xcdf28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdf3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdf40 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xcdf44 | load | 1 | `sp -0x19` | `-` |
| `ldm` at 0xcdf68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcdf68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcdf68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdf30 | `(unknown callee) 0xbac78` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xcdf58 | `(unknown callee) 0xbb498` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcdf68 | `r0` | `ret(0xbb498)` |

## `mic74_IntGet`

- `mic74_IntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdf6c to 0xcdfd0.
  Established. The unnamed `0xbb5c4` call carries unknown, `[[sp-24]+2]:1`, `0x4`, `[sp-28]` (`bl`
  at 0xcdfbc). The byte through entry `r1 +0x1` is read (`ldrb` at 0xcdf8c). The included return
  record gives `ret(0xbb5c4)` in `r0` (`ldm` at 0xcdfcc). Interrupt register interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdf7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdf80 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcdf84 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcdf88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdf8c | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xcdf90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdfa4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcdfa8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xcdfb8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xcdfcc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcdfcc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcdfcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdf98 | `(unknown callee) 0xbac78` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xcdfbc | `(unknown callee) 0xbb5c4` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcdfcc | `r0` | `ret(0xbb5c4)` |

## `mic74_Int`

- `mic74_Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdfd0 to 0xce260.
  Established. The unnamed `0xbb5c4` call carries unknown, `[[sp-28]+2]:1`, `0x3`, `sp-34` (`bl` at 0xce044). A zero result takes `0xce118`, while nonzero falls through (`beq` at 0xce058). The
  unnamed `0xcad04` call carries `[sp-24]`, `[sp-28]`, `[sp-32]`, `[sp-33]:1` (`bl` at 0xce150).
  Device interrupt processing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdfe0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcdfe4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcdfe8 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xcdfec | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `str` at 0xcdff4 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xcdffc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xce000 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce00c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce010 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xce014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce028 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce02c | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0xce04c | store | 4 | `sp -0x2c` | `ret(0xbb5c4)` |
| `ldr` at 0xce050 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce05c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce064 | load | 4 | `abs 0xce24c` | `-` |
| `ldr` at 0xce068 | load | 4 | `abs 0xce250` | `-` |
| `str` at 0xce0a4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xce0a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce0b0 | load | 4 | `abs 0xce24c` | `-` |
| `ldr` at 0xce0b4 | load | 4 | `abs 0xce250` | `-` |
| `ldr` at 0xce0c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce0d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce0dc | load | 4 | `abs 0xce254` | `-` |
| `str` at 0xce0e0 | store | 4 | `(unknown)` | `0x120b00` |
| `str` at 0xce0e4 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xce0e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce0ec | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xce0f0 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xce0f4 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0xce0fc | load | 4 | `abs 0xce258` | `-` |
| `ldr` at 0xce10c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xce110 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xce118 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce11c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xce120 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xce134 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xce138 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xce13c | store | 4 | `sp -0x4c` | `[sp-34]:1` |
| `ldr` at 0xce140 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce144 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce158 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xce160 | store | 4 | `sp -0x28` | `([sp-40] + ret(0xcad04))` |
| `ldr` at 0xce168 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xce16c | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xce174 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xce17c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xce184 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce18c | load | 4 | `abs 0xce24c` | `-` |
| `ldr` at 0xce190 | load | 4 | `abs 0xce250` | `-` |
| `str` at 0xce1cc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xce1d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce1d8 | load | 4 | `abs 0xce24c` | `-` |
| `ldr` at 0xce1dc | load | 4 | `abs 0xce250` | `-` |
| `ldr` at 0xce1e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce1f8 | load | 4 | `abs 0xce25c` | `-` |
| `str` at 0xce1fc | store | 4 | `(unknown)` | `0x120b44` |
| `str` at 0xce200 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xce204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce208 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xce20c | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0xce210 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xce214 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0xce218 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce21c | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0xce220 | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0xce228 | load | 4 | `abs 0xce258` | `-` |
| `ldr` at 0xce238 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xce23c | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xce240 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xce248 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xce248 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xce248 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xce248 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce01c | `(unknown callee) 0xbac78` | `?` | `?` | `?` | `?` |
| `bl` at 0xce044 | `(unknown callee) 0xbb5c4` | `?` | `[[sp-28]+2]:1` | `0x3` | `sp-34` |
| `bl` at 0xce06c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120af4` | `0x120afc` |
| `bl` at 0xce078 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xce0b8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120af4` | `0x120afc` |
| `bl` at 0xce0c8 | `(unknown callee) 0x19808` | `[sp-52]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xce0d4 | `(unknown callee) 0x1fc1c` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xce108 | `(unknown callee) 0x200a0` | `0x3` | `0x120b28` | `0x87` | `0x0` |
| `bl` at 0xce150 | `(unknown callee) 0xcad04` | `[sp-24]` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0xce194 | `(unknown callee) 0x48da8` | `?` | `0x0` | `0x120af4` | `0x120afc` |
| `bl` at 0xce1a0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xce1e0 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120af4` | `0x120afc` |
| `bl` at 0xce1f0 | `(unknown callee) 0x19808` | `[sp-52]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xce234 | `(unknown callee) 0x200a0` | `0x3` | `0x120b28` | `0x92` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xce008 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xce058 | `ret(0xbb5c4) == 0x0` | `ret(0xbb5c4) != 0x0` |
| `beq` at 0xce130 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xce248 | `r0` | `[sp-40]` |

## `psicInit`

- `psicInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf410 to 0xcf644.
  Established. The absolute word at `0x12da38` receives `0x1` (`str` at 0xcf498). The word at
  `0x12da34` receives `ret(0x196d0)` (`str` at 0xcf4cc). The unnamed `0x196d0` call at `0xcf54c`
  carries `[[sp-36]+4]`, `0x1`, `[sp-36]`, `[sp-36]` (`bl` at 0xcf54c). Controller initialization
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf420 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcf424 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcf428 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xcf42c | load | 4 | `abs 0xcf620` | `-` |
| `ldr` at 0xcf430 | load | 4 | `abs 0x12da34` | `-` |
| `ldr` at 0xcf434 | load | 4 | `abs 0xcf624` | `-` |
| `ldr` at 0xcf440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf44c | load | 4 | `abs 0xcf628` | `-` |
| `str` at 0xcf450 | store | 4 | `sp -0x34` | `0x120ce0` |
| `ldr` at 0xcf454 | load | 4 | `abs 0xcf620` | `-` |
| `ldr` at 0xcf458 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf45c | store | 4 | `sp -0x30` | `[0x12da34]` |
| `ldr` at 0xcf460 | load | 4 | `abs 0xcf62c` | `-` |
| `ldr` at 0xcf464 | load | 4 | `abs 0x12da38` | `-` |
| `str` at 0xcf468 | store | 4 | `sp -0x2c` | `[0x12da38]` |
| `ldr` at 0xcf470 | load | 4 | `abs 0xcf630` | `-` |
| `ldr` at 0xcf480 | load | 4 | `abs 0xcf62c` | `-` |
| `ldr` at 0xcf484 | load | 4 | `abs 0x12da38` | `-` |
| `ldr` at 0xcf490 | load | 4 | `abs 0xcf62c` | `-` |
| `str` at 0xcf498 | store | 4 | `abs 0x12da38` | `0x1` |
| `ldr` at 0xcf49c | load | 4 | `abs 0xcf62c` | `-` |
| `ldr` at 0xcf4a0 | load | 4 | `abs 0x12da38` | `-` |
| `str` at 0xcf4b4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcf4b8 | load | 4 | `abs 0xcf620` | `-` |
| `ldr` at 0xcf4bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcf4cc | store | 4 | `abs 0x12da34` | `ret(0x196d0)` |
| `ldr` at 0xcf4d0 | load | 4 | `abs 0x12da34` | `-` |
| `ldr` at 0xcf4dc | load | 4 | `abs 0xcf634` | `-` |
| `str` at 0xcf4e0 | store | 4 | `sp -0x34` | `0x120d18` |
| `ldr` at 0xcf4e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcf4e8 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xcf4f0 | load | 4 | `abs 0xcf630` | `-` |
| `ldr` at 0xcf500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf514 | load | 4 | `abs 0xcf620` | `-` |
| `ldr` at 0xcf518 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf520 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcf524 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf528 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcf52c | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xcf530 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf534 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf538 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xcf53c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf540 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf544 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xcf554 | store | 4 | `[sp-36] +0x8` | `ret(0x196d0)` |
| `ldr` at 0xcf558 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xcf564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf56c | load | 4 | `abs 0xcf638` | `-` |
| `ldr` at 0xcf570 | load | 4 | `abs 0xcf63c` | `-` |
| `str` at 0xcf5ac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcf5b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf5b8 | load | 4 | `abs 0xcf638` | `-` |
| `ldr` at 0xcf5bc | load | 4 | `abs 0xcf63c` | `-` |
| `ldr` at 0xcf5c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcf5d8 | load | 4 | `abs 0xcf640` | `-` |
| `str` at 0xcf5dc | store | 4 | `(unknown)` | `0x120d5c` |
| `str` at 0xcf5e0 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcf5e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf5e8 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xcf5f0 | load | 4 | `abs 0xcf630` | `-` |
| `ldr` at 0xcf600 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf604 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf608 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xcf610 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldm` at 0xcf61c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcf61c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcf61c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcf61c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf47c | `(unknown callee) 0x200a0` | `0x9` | `0x120d10` | `0x58` | `0x0` |
| `bl` at 0xcf4c4 | `(unknown callee) 0x196d0` | `[sp-40]` | `0x1` | `[0x12da38]` | `?` |
| `bl` at 0xcf4fc | `(unknown callee) 0x200a0` | `0x9` | `0x120d10` | `0x64` | `0x0` |
| `bl` at 0xcf54c | `(unknown callee) 0x196d0` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xcf574 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120d50` | `0x120d58` |
| `bl` at 0xcf580 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcf5c0 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120d50` | `0x120d58` |
| `bl` at 0xcf5d0 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcf5fc | `(unknown callee) 0x200a0` | `0x9` | `0x120d10` | `0x70` | `0x0` |
| `bl` at 0xcf614 | `(unknown callee) 0x19748` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcf43c | `[0x12da34] == 0x29735c` | `[0x12da34] != 0x29735c` |
| `bne` at 0xcf448 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xcf48c | `[0x12da38] != 0x0` | `[0x12da38] == 0x0` |
| `bne` at 0xcf4d8 | `[0x12da34] != 0x0` | `[0x12da34] == 0x0` |
| `bne` at 0xcf560 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcf61c | `r0` | `ret(0x19748)` |

## `psicReset`

- `psicReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf644 to 0xcf744.
  Established. The unnamed `0xcfc78` call at `0xcf6c8` carries `[[sp-32]]`, `[[sp-24]+3]:1`, `0xa`,
  `[[sp-24]+6]:1` (`bl` at 0xcf6c8); its later call carries `[[sp-32]]`, `[[sp-24]+3]:1`, `0xc`,
  `0x0` (`bl` at 0xcf704). A zero later result takes `0xcf730`, while nonzero falls through (`beq`
  at 0xcf718). Controller reset protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf654 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf658 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcf65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf670 | load | 4 | `abs 0xcf740` | `-` |
| `ldr` at 0xcf674 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf67c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcf680 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf684 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf688 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xcf690 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcf698 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xcf6a0 | store | 4 | `sp -0x28` | `0x0` |
| `strb` at 0xcf6a8 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xcf6ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf6b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf6b4 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xcf6b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf6bc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xcf6c0 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xcf6d0 | store | 4 | `sp -0x24` | `ret(0xcfc78)` |
| `ldr` at 0xcf6d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf6e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcf6e4 | store | 4 | `sp -0x28` | `ret(0xcfc78)` |
| `ldr` at 0xcf6e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf6ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf6f0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xcf6f4 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xcf70c | store | 4 | `sp -0x24` | `ret(0xcfc78)` |
| `ldr` at 0xcf710 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf71c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcf728 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcf72c | store | 4 | `sp -0x28` | `ret(0xcfc78)` |
| `ldr` at 0xcf730 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xcf73c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcf73c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcf73c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf6c8 | `(unknown callee) 0xcfc78` | `[[sp-32]]` | `[[sp-24]+3]:1` | `0xa` | `[[sp-24]+6]:1` |
| `bl` at 0xcf704 | `(unknown callee) 0xcfc78` | `[[sp-32]]` | `[[sp-24]+3]:1` | `0xc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcf6dc | `ret(0xcfc78) == 0x0` | `ret(0xcfc78) != 0x0` |
| `beq` at 0xcf718 | `ret(0xcfc78) == 0x0` | `ret(0xcfc78) != 0x0` |
| `bne` at 0xcf724 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcf73c | `r0` | `[sp-40]` |

## `psicResetAll`

- `psicResetAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf744 to 0xcf810.
  Established. The unnamed `0xca3d0` call carries entry `r0`, `sp-24`, entry `r2`, `sp-24` (`bl` at 0xcf76c). A zero `ret(0xcf644)` takes `0xcf7f0`, while nonzero falls through (`beq` at 0xcf7d8).
  An included word assignment at `sp -0x24` writes `ret(0xcf644)` (`str` at 0xcf7ec). Controller
  reset traversal remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf754 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf75c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcf764 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcf774 | store | 4 | `sp -0x20` | `ret(0xca3d0)` |
| `str` at 0xcf77c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcf780 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf784 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf798 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf7a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf7ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf7b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf7bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcf7cc | store | 4 | `sp -0x28` | `ret(0xcf644)` |
| `ldr` at 0xcf7d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcf7dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf7e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcf7ec | store | 4 | `sp -0x24` | `ret(0xcf644)` |
| `ldr` at 0xcf7f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcf7f8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcf800 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xcf80c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcf80c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcf80c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf76c | `(unknown callee) 0xca3d0` | `r0` | `sp-24` | `r2` | `sp-24` |
| `bl` at 0xcf7c4 | `(unknown callee) 0xcf644` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcf78c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xcf7a8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xcf7d8 | `ret(0xcf644) == 0x0` | `ret(0xcf644) != 0x0` |
| `bne` at 0xcf7e4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcf80c | `r0` | `[sp-36]` |

## `psicRead`

- `psicRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf8a0 to 0xcf928.
  Established. The absolute word at `0x12da34` is read (`ldr` at 0xcf8d4). The unnamed `0xcfe44`
  call carries unknown, `[r1+3]:1`, `0xa`, entry `r2` (`bl` at 0xcf910). The included return record
  gives `ret(0xcfe44)` in `r0` (`ldm` at 0xcf920). Device read semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf8b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf8b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf8b8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcf8bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf8d0 | load | 4 | `abs 0xcf924` | `-` |
| `ldr` at 0xcf8d4 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf8dc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcf8e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf8e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf8e8 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xcf8f0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcf8f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf8fc | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xcf900 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf90c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xcf920 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcf920 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcf920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf910 | `(unknown callee) 0xcfe44` | `?` | `[r1+3]:1` | `0xa` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcf920 | `r0` | `ret(0xcfe44)` |

## `psicWrite`

- `psicWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf810 to 0xcf8a0.
  Established. The unnamed `0xcfc78` call carries unknown, `[r1+3]:1`, `0xa`, `(r2 & 0xff)` (`bl` at 0xcf888). The byte through entry `r1 +0x2` is read (`ldrb` at 0xcf85c). The byte at `sp -0x19`
  receives `(r2 & 0xff)` (`strb` at 0xcf82c). Device write protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf820 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf824 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xcf82c | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xcf830 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf844 | load | 4 | `abs 0xcf89c` | `-` |
| `ldr` at 0xcf848 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf850 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcf854 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf858 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf85c | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xcf864 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcf86c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf870 | load | 1 | `r1 +0x3` | `-` |
| `ldrb` at 0xcf874 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcf878 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcf898 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcf898 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcf898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf888 | `(unknown callee) 0xcfc78` | `?` | `[r1+3]:1` | `0xa` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcf898 | `r0` | `ret(0xcfc78)` |

## `psicIntSet`

- `psicIntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf928 to 0xcf9b8.
  Established. The unnamed `0xcfc78` call carries unknown, `[r1+3]:1`, `0xc`, `(r2 & 0xff)` (`bl` at 0xcf9a0). The absolute word at `0x12da34` is read (`ldr` at 0xcf960). The included return record
  carries `ret(0xcfc78)` in `r0` (`ldm` at 0xcf9b0). Interrupt setting effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf938 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf93c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xcf944 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xcf948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf95c | load | 4 | `abs 0xcf9b4` | `-` |
| `ldr` at 0xcf960 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf968 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcf96c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf970 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf974 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xcf97c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcf984 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf988 | load | 1 | `r1 +0x3` | `-` |
| `ldrb` at 0xcf98c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcf990 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcf9b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcf9b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcf9b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf9a0 | `(unknown callee) 0xcfc78` | `?` | `[r1+3]:1` | `0xc` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcf9b0 | `r0` | `ret(0xcfc78)` |

## `psicIntGet`

- `psicIntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf9b8 to 0xcfa40.
  Established. The unnamed `0xcfe44` call carries unknown, `[r1+3]:1`, `0xc`, entry `r2` (`bl` at 0xcfa28). The byte through entry `r1 +0x3` is read (`ldrb` at 0xcfa14). The included return record
  carries `ret(0xcfe44)` in `r0` (`ldm` at 0xcfa38). Interrupt read format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf9c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf9cc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf9d0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcf9d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf9e8 | load | 4 | `abs 0xcfa3c` | `-` |
| `ldr` at 0xcf9ec | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcf9f4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcf9f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf9fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcfa00 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xcfa08 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcfa10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcfa14 | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xcfa18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcfa24 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xcfa38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcfa38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcfa38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcfa28 | `(unknown callee) 0xcfe44` | `?` | `[r1+3]:1` | `0xc` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcfa38 | `r0` | `ret(0xcfe44)` |

## `psicInt`

- `psicInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcfa40 to 0xcfc78.
  Established. The unnamed `0xcfe44` call carries `[sp-44]`, unknown, `0xd`, `sp-57` (`bl` at 0xcfadc); its zero result takes `0xcfbb0`, while nonzero falls through (`beq` at 0xcfaf0). The
  unnamed `0xcad04` call carries `0x0`, `[sp-28]`, `[sp-32]`, `[sp-33]:1` (`bl` at 0xcfbe8).
  Controller interrupt policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcfa50 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcfa54 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcfa58 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xcfa5c | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xcfa60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfa74 | load | 4 | `abs 0xcfc60` | `-` |
| `ldr` at 0xcfa78 | load | 4 | `abs 0x12da34` | `-` |
| `str` at 0xcfa80 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcfa84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcfa88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcfa8c | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xcfa94 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xcfa9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfaa0 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xcfaa8 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xcfab0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xcfab4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcfac0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcfac4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcfacc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcfae4 | store | 4 | `sp -0x30` | `ret(0xcfe44)` |
| `ldr` at 0xcfae8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcfaf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfafc | load | 4 | `abs 0xcfc64` | `-` |
| `ldr` at 0xcfb00 | load | 4 | `abs 0xcfc68` | `-` |
| `str` at 0xcfb3c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xcfb40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfb48 | load | 4 | `abs 0xcfc64` | `-` |
| `ldr` at 0xcfb4c | load | 4 | `abs 0xcfc68` | `-` |
| `ldr` at 0xcfb58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcfb68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcfb74 | load | 4 | `abs 0xcfc6c` | `-` |
| `str` at 0xcfb78 | store | 4 | `(unknown)` | `0x120d98` |
| `str` at 0xcfb7c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xcfb80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcfb84 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xcfb88 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xcfb8c | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0xcfb94 | load | 4 | `abs 0xcfc70` | `-` |
| `ldr` at 0xcfba4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcfba8 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xcfbb0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcfbb4 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xcfbb8 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0xcfbcc | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xcfbd0 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xcfbd4 | store | 4 | `sp -0x54` | `[sp-57]:1` |
| `ldr` at 0xcfbdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcfbe0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfbf0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcfbf8 | store | 4 | `sp -0x34` | `([sp-52] + ret(0xcad04))` |
| `ldr` at 0xcfc00 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcfc04 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xcfc0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcfc14 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xcfc1c | load | 4 | `abs 0xcfc74` | `-` |
| `str` at 0xcfc20 | store | 4 | `sp -0x54` | `0x120dc0` |
| `ldr` at 0xcfc24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcfc28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfc2c | store | 4 | `sp -0x50` | `?` |
| `ldrb` at 0xcfc30 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xcfc34 | store | 4 | `sp -0x4c` | `[sp-57]:1` |
| `ldr` at 0xcfc3c | load | 4 | `abs 0xcfc70` | `-` |
| `ldr` at 0xcfc4c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcfc50 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xcfc54 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0xcfc5c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcfc5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcfc5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcfc5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcfadc | `(unknown callee) 0xcfe44` | `[sp-44]` | `?` | `0xd` | `sp-57` |
| `bl` at 0xcfb04 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120d50` | `0x120d58` |
| `bl` at 0xcfb10 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xcfb50 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x120d50` | `0x120d58` |
| `bl` at 0xcfb60 | `(unknown callee) 0x19808` | `[sp-64]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xcfb6c | `(unknown callee) 0x1fc1c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xcfba0 | `(unknown callee) 0x200a0` | `0x3` | `0x120d10` | `0xe0` | `0x0` |
| `bl` at 0xcfbe8 | `(unknown callee) 0xcad04` | `0x0` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0xcfc48 | `(unknown callee) 0x200a0` | `0x3` | `0x120d10` | `0xeb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xcfabc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xcfaf0 | `ret(0xcfe44) == 0x0` | `ret(0xcfe44) != 0x0` |
| `beq` at 0xcfbc8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcfc5c | `r0` | `[sp-52]` |

## `vsc055_Count`

- `vsc055_Count`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcce64 to 0xcceb4.
  Established. The absolute word at `0x12da28` is read (`ldr` at 0xcce90). The word through `[sp-24] +0x4` is read (`ldr` at 0xccea0). The included return record gives `[[sp-24]+4]` in `r0` (`ldm` at 0xcceac). Device count representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcce74 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcce78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcce8c | load | 4 | `abs 0xcceb0` | `-` |
| `ldr` at 0xcce90 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcce98 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xcce9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccea0 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldm` at 0xcceac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcceac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcceac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcceac | `r0` | `[[sp-24]+4]` |

## `vsc055_GetCfg`

- `vsc055_GetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccdf4 to 0xcce64.
  Established. A zero entry `r1` takes `0xcce4c`, while nonzero falls through (`beq` at 0xcce38).
  The included word assignment through entry `r1 +0x0` writes `[[sp-28]]` (`str` at 0xcce48). The
  absolute word at `0x12da28` is read (`ldr` at 0xcce24). Device configuration accessor meaning
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcce04 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcce08 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcce0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcce20 | load | 4 | `abs 0xcce60` | `-` |
| `ldr` at 0xcce24 | load | 4 | `abs 0x12da28` | `-` |
| `str` at 0xcce2c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcce30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcce3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcce40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcce44 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcce48 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xcce4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcce50 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0xcce5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcce5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcce5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcce38 | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcce5c | `r0` | `[[sp-28]+4]` |
