# ARM serial keys and log helpers in 1.44.0

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
The computed transfers in `SerNumLearn` (`ldrls` at 0xa1f34) and `inv_manip` (`ldrls` at 0x49b2c)
leave unresolved selector edges. Access and call tables cover only known edges.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/serial-key-log-helpers-facts.txt) retain all tool sections.

## `sernumGetKey`

- `sernumGetKey`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48edc to 0x49050.
  Established. Included word reads address `[sp-28] +0x6c`, `[sp-28] +0x70` and `[sp-28] +0x74` (`ldr` at 0x48f68, `ldr` at 0x48f5c, `ldr` at 0x48f74).
  The unsigned condition `[sp-60] > 0xea60` takes `0x48ff8`, while lower or equal falls through (`bhi` at 0x48fb4).
  The unnamed `0xc5738` call carries `0x0`, `sp-28`, unknown, `sp-28` (`bl` at 0x48f10).
  Serial key record validation meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48eec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x48ef0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x48f00 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x48f28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48f30 | store | 4 | `sp -0x1c` | `[sp-28]+480` |
| `ldr` at 0x48f34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f38 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x48f44 | load | 4 | `abs 0x49040` | `-` |
| `str` at 0x48f50 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x48f58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f5c | load | 4 | `[sp-28] +0x70` | `-` |
| `str` at 0x48f60 | store | 4 | `sp -0x3c` | `[[sp-28]+112]` |
| `ldr` at 0x48f64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f68 | load | 4 | `[sp-28] +0x6c` | `-` |
| `str` at 0x48f6c | store | 4 | `sp -0x38` | `[[sp-28]+108]` |
| `ldr` at 0x48f70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f74 | load | 4 | `[sp-28] +0x74` | `-` |
| `str` at 0x48f78 | store | 4 | `sp -0x40` | `[[sp-28]+116]` |
| `ldr` at 0x48f7c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x48f80 | store | 4 | `sp -0x2c` | `[[sp-28]+108]` |
| `ldr` at 0x48f84 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x48f88 | store | 4 | `sp -0x28` | `[[sp-28]+112]` |
| `ldr` at 0x48f8c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x48f90 | store | 4 | `sp -0x20` | `[[sp-28]+116]` |
| `ldr` at 0x48f94 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x48fa0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x48fa0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x48fa0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x48fa0 | load | 4 | `sp -0x20` | `-` |
| `stm` at 0x48fa4 | store | 4 | `[sp-20] +0x0` | `[[sp-28]+108]` |
| `stm` at 0x48fa4 | store | 4 | `[sp-20] +0x4` | `[[sp-28]+112]` |
| `stm` at 0x48fa4 | store | 4 | `[sp-20] +0x8` | `[sp-36]` |
| `stm` at 0x48fa4 | store | 4 | `[sp-20] +0xc` | `[[sp-28]+116]` |
| `ldr` at 0x48fa8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x48fac | load | 4 | `abs 0x49044` | `-` |
| `ldr` at 0x48fb8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x48fbc | load | 4 | `abs 0x49044` | `-` |
| `ldr` at 0x48fc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x48fcc | load | 4 | `abs 0x49044` | `-` |
| `ldr` at 0x48fd8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x48fe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x48fe8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x48ff8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49004 | load | 4 | `abs 0x49048` | `-` |
| `ldr` at 0x4900c | load | 4 | `abs 0x4904c` | `-` |
| `ldr` at 0x49010 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x49014 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x49018 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x49024 | store | 4 | `sp -0x50` | `0x2` |
| `str` at 0x49030 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0x49034 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x4903c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4903c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4903c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48ef4 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x48f10 | `(unknown callee) 0xc5738` | `0x0` | `sp-28` | `?` | `sp-28` |
| `bl` at 0x48f14 | `(unknown callee) 0x488cc` | `ret(0xc5738)` | `?` | `?` | `?` |
| `bl` at 0x48f48 | `(unknown callee) 0x1e4ac` | `0xf7e80` | `?` | `?` | `[[sp-28]+28]` |
| `bl` at 0x49008 | `(unknown callee) 0x221f0` | `0xf7eb4` | `[[sp-28]+112]` | `?` | `[sp-24]` |
| `bl` at 0x4901c | `(unknown callee) 0x1e958` | `0xf7ee4` | `[sp-56]` | `[sp-60]` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x48f24 | `? != 0x5e` | `? == 0x5e` |
| `beq` at 0x48f40 | `[[sp-28]+28] == 0x0` | `[[sp-28]+28] != 0x0` |
| `bhi` at 0x48fb4 | `[sp-60] > 0xea60 (unsigned, 32-bit)` | `[sp-60] <= 0xea60 (unsigned, 32-bit)` |
| `bhi` at 0x48fc4 | `[sp-56] > 0xea60 (unsigned, 32-bit)` | `[sp-56] <= 0xea60 (unsigned, 32-bit)` |
| `bhi` at 0x48fd4 | `[sp-64] > 0xea60 (unsigned, 32-bit)` | `[sp-64] <= 0xea60 (unsigned, 32-bit)` |
| `bne` at 0x48fe0 | `[sp-56] != 0x2` | `[sp-56] == 0x2` |
| `bhi` at 0x48ff0 | `[sp-64] > [sp-60] (unsigned, 32-bit)` | `[sp-64] <= [sp-60] (unsigned, 32-bit)` |
| `beq` at 0x49000 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4903c | `r0` | `?` |

## `sernumCodedStringToInt`

- `sernumCodedStringToInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49a34 to 0x49b08.
  Established. A byte read has unknown effective target (`ldrb` at 0x49a8c).
  The unnamed `0x499cc` call carries `0xa`, `[sp-40]`, unknown, unknown (`bl` at 0x49aa4).
  A word assignment increments `sp -0x30` with `[sp-48]+1` (`str` at 0x49af0).
  Coded serial string digit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49a44 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x49a48 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x49a4c | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x49a50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49a54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49a5c | store | 4 | `sp -0x28` | `?` |
| `str` at 0x49a64 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x49a68 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49a6c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x49a70 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x49a74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49a84 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x49a8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x49aa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49aac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x49ab0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49adc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x49ae4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x49ae8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x49af0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x49af8 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x49b04 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x49b04 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x49b04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x49b04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x49b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49a94 | `(unknown callee) 0x498dc` | `?` | `?` | `[sp-28]` | `?` |
| `bl` at 0x49aa4 | `(unknown callee) 0x499cc` | `0xa` | `[sp-40]` | `?` | `?` |
| `bl` at 0x49ac0 | `(unknown callee) 0x499cc` | `0xa` | `?` | `[sp-48]` | `?` |
| `bl` at 0x49ad0 | `(unknown callee) 0xeb174` | `ret(0x499cc)` | `ret(0x499cc)` | `?` | `ret(0x499cc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x49a7c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x49b04 | `r0` | `?` |

## `sernumUnscrambleExtra`

- `sernumUnscrambleExtra`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49798 to 0x498dc.
  Established. The unnamed `0x49a34` call carries entry `r0`, `0x0`, `0x0`, `0x0` (`bl` at 0x497c8).
  A nonzero `[sp-36]` takes `0x49850`, while zero falls through (`bne` at 0x49808).
  Included byte assignments write `(ret(0x498dc) & 0xff)` and `0x0` to unknown effective targets (`strb` at 0x49898, `strb` at 0x498c8).
  Extra serial payload encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x497a8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x497ac | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x497b0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x497b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x497bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x497d0 | store | 4 | `sp -0x24` | `ret(0x49a34)` |
| `ldr` at 0x497d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x497e8 | store | 4 | `sp -0x30` | `ret(0x49a34)` |
| `ldr` at 0x497ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x497fc | store | 4 | `sp -0x2c` | `ret(0x19538)-1` |
| `ldr` at 0x49800 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4980c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49814 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x49820 | store | 4 | `sp -0x28` | `ret(0x49a34)` |
| `ldr` at 0x49828 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x49834 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49848 | store | 4 | `sp -0x28` | `ret(0xeb368)` |
| `str` at 0x49854 | store | 4 | `sp -0x34` | `0x2` |
| `str` at 0x4985c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x49860 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x49864 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x49870 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49874 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4987c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49880 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x49888 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x49898 | store | 1 | `(unknown)` | `(ret(0x498dc) & 0xff)` |
| `ldr` at 0x4989c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x498a4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x498a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x498b0 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x498b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x498bc | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x498c8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x498cc | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x498d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x498d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x498d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x498d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x497c8 | `(unknown callee) 0x49a34` | `r0` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x497e0 | `(unknown callee) 0x49a34` | `[sp-24]` | `0x1` | `0x1` | `ret(0x49a34)` |
| `bl` at 0x497f0 | `(unknown callee) 0x19538` | `[sp-24]` | `?` | `?` | `ret(0x49a34)` |
| `bl` at 0x49818 | `(unknown callee) 0x49a34` | `[sp-24]` | `0x2` | `ret(0x19538)-1` | `[sp-36]` |
| `bl` at 0x4982c | `(unknown callee) 0x499cc` | `0xa` | `[sp-48]` | `?` | `ret(0x49a34)` |
| `bl` at 0x49840 | `(unknown callee) 0xeb368` | `[sp-40]` | `ret(0x499cc)` | `ret(0x499cc)` | `[sp-40]` |
| `bl` at 0x49890 | `(unknown callee) 0x498dc` | `?` | `?` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x49808 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bgt` at 0x4986c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x498d8 | `r0` | `[sp-40]` |

## `sernumBootCheck`

- `sernumBootCheck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49c2c to 0x4a1e8.
  Established. An included byte read addresses `0x29758c` (`ldrb` at 0x49d54).
  A zero byte at that address takes `0x49d84`, while nonzero falls through (`beq` at 0x49d5c).
  Separate word assignments write `0xfffff95c` and `0xfffff95a` to `sp -0xc4` (`str` at 0x49cf0, `str` at 0x49d48).
  The hardware significance of these checks and result codes remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49c40 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x49c4c | store | 4 | `sp -0x18` | `ret(0x1d444)` |
| `str` at 0x49c58 | store | 4 | `sp -0x20` | `ret(0x1d444)` |
| `str` at 0x49c74 | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0x49c80 | store | 4 | `sp -0x1c` | `ret(0x48b30)` |
| `ldr` at 0x49c84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49c90 | load | 4 | `abs 0x4a16c` | `-` |
| `str` at 0x49c94 | store | 4 | `sp -0xd0` | `0xf7fa8` |
| `ldr` at 0x49c9c | load | 4 | `abs 0x4a170` | `-` |
| `str` at 0x49cb0 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x49cb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49cbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49cd4 | load | 4 | `abs 0x4a174` | `-` |
| `ldr` at 0x49cdc | load | 4 | `abs 0x4a178` | `-` |
| `ldr` at 0x49ce0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49cec | load | 4 | `abs 0x4a17c` | `-` |
| `str` at 0x49cf0 | store | 4 | `sp -0xc4` | `0xfffff95c` |
| `ldr` at 0x49cf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49d04 | load | 4 | `abs 0x4a180` | `-` |
| `ldr` at 0x49d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49d24 | load | 4 | `abs 0x4a184` | `-` |
| `ldr` at 0x49d38 | load | 4 | `abs 0x4a188` | `-` |
| `ldr` at 0x49d3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49d44 | load | 4 | `abs 0x4a18c` | `-` |
| `str` at 0x49d48 | store | 4 | `sp -0xc4` | `0xfffff95a` |
| `ldr` at 0x49d50 | load | 4 | `abs 0x4a190` | `-` |
| `ldrb` at 0x49d54 | load | 1 | `abs 0x29758c` | `-` |
| `ldr` at 0x49d60 | load | 4 | `abs 0x4a190` | `-` |
| `ldr` at 0x49d64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49d68 | load | 1 | `abs 0x29758c` | `-` |
| `ldrb` at 0x49d6c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x49d7c | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x49d84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49d88 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x49d94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49d98 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x49da4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49da8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x49db4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49db8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x49dc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49dc8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x49dd4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49dd8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x49de4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49de8 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x49df8 | store | 4 | `sp -0x14` | `0xa` |
| `ldr` at 0x49e00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49e08 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x49e18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49e20 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x49e3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49e44 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x49e4c | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x49e54 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49e74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49e7c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x49e84 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x49e8c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49eac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49eb4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x49ebc | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x49ec4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49ee4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49eec | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x49ef4 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x49efc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49f1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49f24 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x49f2c | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x49f34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49f54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49f5c | load | 1 | `[sp-24] +0x7` | `-` |
| `ldr` at 0x49f64 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x49f6c | load | 2 | `(unknown)` | `-` |
| `str` at 0x49f80 | store | 4 | `sp -0x14` | `0xb` |
| `ldr` at 0x49f84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49f90 | load | 4 | `abs 0x4a194` | `-` |
| `ldr` at 0x49f98 | load | 4 | `abs 0x4a198` | `-` |
| `ldr` at 0x49f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49fa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49fa8 | store | 4 | `sp -0xc8` | `[sp-20]` |
| `ldr` at 0x49fac | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x49fb8 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x49fc4 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x49fd4 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x49fe4 | load | 4 | `abs 0x4a190` | `-` |
| `ldrb` at 0x49fe8 | load | 1 | `abs 0x29758c` | `-` |
| `ldr` at 0x49fec | load | 4 | `abs 0x4a19c` | `-` |
| `ldr` at 0x49ffc | load | 4 | `abs 0x4a1a0` | `-` |
| `ldr` at 0x4a000 | load | 4 | `abs 0x4a1a4` | `-` |
| `ldr` at 0x4a00c | load | 4 | `abs 0x4a1a8` | `-` |
| `ldr` at 0x4a018 | load | 4 | `abs 0x4a1ac` | `-` |
| `ldr` at 0x4a020 | load | 4 | `abs 0x4a1b0` | `-` |
| `str` at 0x4a024 | store | 4 | `sp -0xc4` | `0xfffff959` |
| `ldr` at 0x4a02c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a030 | load | 4 | `abs 0x4a1b4` | `-` |
| `ldr` at 0x4a044 | load | 4 | `abs 0x4a1b8` | `-` |
| `ldr` at 0x4a054 | load | 4 | `abs 0x4a1bc` | `-` |
| `ldr` at 0x4a058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a05c | load | 4 | `abs 0x4a1b4` | `-` |
| `ldr` at 0x4a064 | load | 4 | `abs 0x4a1c0` | `-` |
| `ldr` at 0x4a068 | load | 4 | `abs 0x133b64` | `-` |
| `str` at 0x4a070 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x4a074 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x4a080 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x4a08c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x4a09c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x4a0b4 | load | 4 | `abs 0x4a1c4` | `-` |
| `ldr` at 0x4a0c8 | load | 4 | `abs 0x4a1c8` | `-` |
| `ldr` at 0x4a0dc | load | 4 | `abs 0x4a1cc` | `-` |
| `ldr` at 0x4a0f0 | load | 4 | `abs 0x4a1d0` | `-` |
| `str` at 0x4a0f4 | store | 4 | `sp -0xc4` | `0xfffff95b` |
| `ldr` at 0x4a0fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a108 | load | 4 | `abs 0x4a1d4` | `-` |
| `ldr` at 0x4a11c | load | 4 | `abs 0x4a1d8` | `-` |
| `ldr` at 0x4a120 | load | 4 | `abs 0x4a1d4` | `-` |
| `ldr` at 0x4a134 | load | 4 | `abs 0x4a1dc` | `-` |
| `ldr` at 0x4a13c | load | 4 | `abs 0x4a1e0` | `-` |
| `ldr` at 0x4a140 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a144 | load | 4 | `abs 0x4a1b4` | `-` |
| `ldr` at 0x4a14c | load | 4 | `abs 0x4a1e4` | `-` |
| `str` at 0x4a150 | store | 4 | `sp -0xc4` | `0xfffff958` |
| `str` at 0x4a15c | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x4a160 | load | 4 | `sp -0xc4` | `-` |
| `ldm` at 0x4a168 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4a168 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4a168 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49c48 | `(unknown callee) 0x1d444` | `0xd` | `r1` | `r2` | `0x0` |
| `bl` at 0x49c54 | `(unknown callee) 0x1d444` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x49c60 | `(unknown callee) 0x1d444` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x49c7c | `(unknown callee) 0x48b30` | `ret(0x1d444)` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x49ca8 | `(unknown callee) 0x200a0` | `0x1` | `0xf7fd4` | `0x238` | `0x0` |
| `bl` at 0x49cc4 | `(unknown callee) 0x1955c` | `[sp-32]` | `ret(0x48b30)` | `0x6` | `ret(0x48b30)` |
| `bl` at 0x49cd8 | `(unknown callee) 0x1e4ac` | `0xf7fec` | `?` | `?` | `ret(0x1955c)` |
| `bl` at 0x49ce8 | `(unknown callee) 0x1e958` | `0xf800c` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0x49d08 | `(unknown callee) 0x19310` | `[sp-24]+1` | `0xf8030` | `?` | `[sp-24]+1` |
| `bl` at 0x49d28 | `(unknown callee) 0x19310` | `[sp-24]+1` | `0xf8038` | `?` | `[sp-24]+1` |
| `bl` at 0x49d40 | `(unknown callee) 0x1e958` | `0xf8040` | `[sp-24]` | `?` | `ret(0x19310)` |
| `bl` at 0x49e2c | `(unknown callee) 0x197d8` | `ret(0x19310)` | `?` | `0x1` | `[[sp-24]+1]:1` |
| `bl` at 0x49e64 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x49e9c | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x49ed4 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x49f0c | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x49f44 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x49f94 | `(unknown callee) 0x1e4ac` | `0xf8060` | `?` | `?` | `?` |
| `bl` at 0x49fa0 | `(unknown callee) 0x1e958` | `0xf8078` | `[sp-24]` | `?` | `?` |
| `bl` at 0x49ff4 | `(unknown callee) 0x1e958` | `0xf8084` | `[0x29758c]:1` | `?` | `[0x29758c]:1` |
| `bl` at 0x4a004 | `(unknown callee) 0x1e958` | `0xf80b0` | `0xf80f0` | `?` | `[sp-20]` |
| `bl` at 0x4a010 | `(unknown callee) 0x1e958` | `0xf8100` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4a01c | `(unknown callee) 0x1e958` | `0xf8140` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4a034 | `(unknown callee) 0x19310` | `[sp-24]` | `0x29a978` | `?` | `?` |
| `bl` at 0x4a048 | `(unknown callee) 0x1e4ac` | `0xf8168` | `?` | `?` | `ret(0x19310)` |
| `bl` at 0x4a060 | `(unknown callee) 0x197a8` | `sp-112` | `0xf8188` | `[sp-24]` | `0x29a978` |
| `bl` at 0x4a0b8 | `(unknown callee) 0x195e0` | `sp-112` | `0xf81b0` | `?` | `sp-112` |
| `bl` at 0x4a0cc | `(unknown callee) 0x195e0` | `sp-112` | `0xf81c4` | `?` | `sp-112` |
| `bl` at 0x4a0e0 | `(unknown callee) 0x195e0` | `sp-112` | `0xf81d4` | `?` | `sp-112` |
| `bl` at 0x4a0ec | `(unknown callee) 0x1e4ac` | `sp-112` | `?` | `?` | `sp-112` |
| `bl` at 0x4a10c | `(unknown callee) 0x19310` | `[sp-24]+1` | `0xf81e4` | `?` | `[sp-24]+1` |
| `bl` at 0x4a124 | `(unknown callee) 0x19310` | `0x29a979` | `0xf81e4` | `?` | `ret(0x19310)` |
| `bl` at 0x4a138 | `(unknown callee) 0x1e4ac` | `0xf81ec` | `?` | `?` | `ret(0x19310)` |
| `bl` at 0x4a148 | `(unknown callee) 0x1e958` | `0xf8200` | `[sp-24]` | `0x29a978` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x49c6c | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `bne` at 0x49c8c | `ret(0x48b30) != 0x0` | `ret(0x48b30) == 0x0` |
| `beq` at 0x49cd0 | `ret(0x1955c) == 0x0` | `ret(0x1955c) != 0x0` |
| `beq` at 0x49d14 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `bne` at 0x49d34 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `beq` at 0x49d5c | `[0x29758c]:1 == 0x0` | `[0x29758c]:1 != 0x0` |
| `beq` at 0x49d74 | `[0x29758c]:1 == [[sp-24]]:1` | `[0x29758c]:1 != [[sp-24]]:1` |
| `beq` at 0x49d90 | `[[sp-24]]:1 == 0x4c` | `[[sp-24]]:1 != 0x4c` |
| `beq` at 0x49da0 | `[[sp-24]]:1 == 0x4d` | `[[sp-24]]:1 != 0x4d` |
| `beq` at 0x49db0 | `[[sp-24]]:1 == 0x4e` | `[[sp-24]]:1 != 0x4e` |
| `beq` at 0x49dc0 | `[[sp-24]]:1 == 0x50` | `[[sp-24]]:1 != 0x50` |
| `beq` at 0x49dd0 | `[[sp-24]]:1 == 0x52` | `[[sp-24]]:1 != 0x52` |
| `beq` at 0x49de0 | `[[sp-24]]:1 == 0x54` | `[[sp-24]]:1 != 0x54` |
| `beq` at 0x49df0 | `[[sp-24]]:1 == 0x55` | `[[sp-24]]:1 != 0x55` |
| `bls` at 0x49e10 | `[[sp-24]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-24]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0x49e28 | `[[sp-24]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-24]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0x49e60 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x49e98 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x49ed0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x49f08 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x49f40 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x49f78 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x49f8c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x49fb4 | `[sp-20] == 0xa` | `[sp-20] != 0xa` |
| `bgt` at 0x49fc0 | `[sp-20] > 0xa (signed, 32-bit)` | `[sp-20] <= 0xa (signed, 32-bit)` |
| `beq` at 0x49fcc | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0x49fdc | `[sp-20] == 0xb` | `[sp-20] != 0xb` |
| `beq` at 0x4a040 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0x4a07c | `[sp-204] == 0x2000` | `[sp-204] != 0x2000` |
| `bgt` at 0x4a088 | `[sp-204] > 0x2000 (signed, 32-bit)` | `[sp-204] <= 0x2000 (signed, 32-bit)` |
| `beq` at 0x4a094 | `[sp-204] == 0x1000` | `[sp-204] != 0x1000` |
| `beq` at 0x4a0a4 | `[sp-204] == 0x4000` | `[sp-204] != 0x4000` |
| `bne` at 0x4a118 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0x4a130 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4a168 | `r0` | `?` |

## `sernumInit`

- `sernumInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4a5c4 to 0x4ab64.
  Established. A word read addresses `0x297688` (`ldr` at 0x4a614).
  Zero at that address takes `0x4a634`, while nonzero falls through (`beq` at 0x4a61c).
  The unknown-target call carries `ret(0x1d444)`, unknown, unknown and `[0x297688]` (`mov` at 0x4a62c).
  A later byte assignment writes `([0x29758c]:1 & 0xff)` to `sp -0xac` (`strb` at 0x4a8d4).
  The callback ownership and hardware role remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4a5d8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x4a5e8 | store | 4 | `sp -0x14` | `?` |
| `str` at 0x4a5f4 | store | 4 | `sp -0x18` | `ret(0x1d444)` |
| `str` at 0x4a600 | store | 4 | `sp -0x20` | `ret(0x1d444)` |
| `str` at 0x4a60c | store | 4 | `sp -0x24` | `ret(0x1d444)` |
| `ldr` at 0x4a610 | load | 4 | `abs 0x4ab44` | `-` |
| `ldr` at 0x4a614 | load | 4 | `abs 0x297688` | `-` |
| `ldr` at 0x4a620 | load | 4 | `abs 0x4ab44` | `-` |
| `ldr` at 0x4a624 | load | 4 | `abs 0x297688` | `-` |
| `str` at 0x4a644 | store | 4 | `sp -0x28` | `ret(0x48b30)` |
| `ldr` at 0x4a648 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4a654 | load | 4 | `abs 0x4ab48` | `-` |
| `str` at 0x4a658 | store | 4 | `sp -0xb0` | `0xf8228` |
| `ldr` at 0x4a660 | load | 4 | `abs 0x4ab4c` | `-` |
| `ldr` at 0x4a664 | load | 4 | `abs 0x4ab50` | `-` |
| `str` at 0x4a674 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x4a678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a69c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a6fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a72c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a744 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a75c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a78c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a7a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a7b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a7bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a7c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a7d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a7e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a81c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a85c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a890 | load | 4 | `abs 0x4ab44` | `-` |
| `ldrb` at 0x4a894 | load | 1 | `abs 0x29758c` | `-` |
| `ldr` at 0x4a8a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4a8a4 | load | 4 | `abs 0x4ab44` | `-` |
| `ldrb` at 0x4a8a8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x4a8ac | load | 1 | `abs 0x29758c` | `-` |
| `ldr` at 0x4a8b8 | load | 4 | `abs 0x4ab54` | `-` |
| `ldm` at 0x4a8c0 | load | 4 | `abs 0xf8254` | `-` |
| `ldm` at 0x4a8c0 | load | 4 | `abs 0xf8258` | `-` |
| `ldm` at 0x4a8c0 | load | 4 | `abs 0xf825c` | `-` |
| `stm` at 0x4a8c4 | store | 4 | `sp -0xac` | `[0xf8254]` |
| `stm` at 0x4a8c4 | store | 4 | `sp -0xa8` | `[0xf8258]` |
| `strb` at 0x4a8c8 | store | 1 | `sp -0xa4` | `([0xf825c] & 0xff)` |
| `ldr` at 0x4a8cc | load | 4 | `abs 0x4ab44` | `-` |
| `ldrb` at 0x4a8d0 | load | 1 | `abs 0x29758c` | `-` |
| `strb` at 0x4a8d4 | store | 1 | `sp -0xac` | `([0x29758c]:1 & 0xff)` |
| `ldr` at 0x4a8e8 | load | 4 | `abs 0x4ab58` | `-` |
| `ldr` at 0x4a8f0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a8f4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a900 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a904 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a910 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a914 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a920 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a924 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a930 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a934 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a940 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a944 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a950 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a954 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x4a964 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a96c | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x4a97c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a984 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x4a9a0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a9a8 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x4a9b0 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x4a9b8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4a9d8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4a9e0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x4a9e8 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x4a9f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4aa10 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4aa18 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x4aa20 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x4aa28 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4aa48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4aa50 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x4aa58 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x4aa60 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4aa80 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4aa88 | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x4aa90 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x4aa98 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4aab8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4aac0 | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x4aac8 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x4aad0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4aae4 | load | 4 | `abs 0x4ab5c` | `-` |
| `ldr` at 0x4aaec | load | 4 | `abs 0x4ab58` | `-` |
| `ldr` at 0x4aaf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4ab00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4ab04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4ab1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4ab2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4ab34 | load | 4 | `abs 0x4ab60` | `-` |
| `ldm` at 0x4ab40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4ab40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4ab40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4a5dc | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x4a5f0 | `(unknown callee) 0x1d444` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x4a5fc | `(unknown callee) 0x1d444` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x4a608 | `(unknown callee) 0x1d444` | `0xf` | `?` | `?` | `?` |
| `mov` at 0x4a62c | `(unknown callee) (unknown target)` | `ret(0x1d444)` | `?` | `?` | `[0x297688]` |
| `bl` at 0x4a63c | `(unknown callee) 0x1b5f0` | `0x54` | `0x1` | `?` | `[0x297688]` |
| `bl` at 0x4a640 | `(unknown callee) 0x48b30` | `ret(0x1b5f0)` | `?` | `?` | `?` |
| `bl` at 0x4a66c | `(unknown callee) 0x200a0` | `0x1` | `0xf7fd4` | `0x2ff` | `0x0` |
| `bl` at 0x4a6f8 | `(unknown callee) 0x1b5f0` | `0x50` | `0x0` | `?` | `[sp-20]` |
| `bl` at 0x4a7f4 | `(unknown callee) 0x1d444` | `0x50` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4a87c | `(unknown callee) 0x1d444` | `0x3e` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4a8e4 | `(unknown callee) 0x1b5f0` | `0xd` | `sp-172` | `[0xf825c]` | `sp-172` |
| `bl` at 0x4a8ec | `(unknown callee) 0x1e4ac` | `0xf8260` | `?` | `?` | `?` |
| `bl` at 0x4a990 | `(unknown callee) 0x197d8` | `?` | `?` | `0x1` | `[[sp-32]+1]:1` |
| `bl` at 0x4a9c8 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x4aa00 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x4aa38 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x4aa70 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x4aaa8 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0x4aae8 | `(unknown callee) 0x1b5f0` | `0xd` | `0xf827c` | `?` | `?` |
| `bl` at 0x4aaf0 | `(unknown callee) 0x1e4ac` | `0xf8260` | `?` | `?` | `?` |
| `bl` at 0x4ab0c | `(unknown callee) 0x1955c` | `[sp-36]` | `[sp-40]` | `0x6` | `[sp-40]` |
| `bl` at 0x4ab30 | `(unknown callee) 0x1b5f0` | `0xf` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x4ab38 | `(unknown callee) 0x1e4ac` | `0xf8288` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4a61c | `[0x297688] == 0x0` | `[0x297688] != 0x0` |
| `bne` at 0x4a650 | `ret(0x48b30) != 0x0` | `ret(0x48b30) == 0x0` |
| `beq` at 0x4a680 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x4a68c | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x4a698 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x4a6a4 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x4a6b0 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x4a6bc | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x4a6c8 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x4a6d4 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x4a6e0 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0x4a6ec | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0x4a704 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x4a710 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x4a71c | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x4a728 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x4a734 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x4a740 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x4a74c | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x4a758 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x4a764 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `bne` at 0x4a770 | `[sp-20] != 0x4d` | `[sp-20] == 0x4d` |
| `beq` at 0x4a77c | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x4a788 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x4a794 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x4a7a0 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x4a7ac | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x4a7b8 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x4a7c4 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x4a7d0 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x4a7dc | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0x4a7e8 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `bne` at 0x4a800 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `beq` at 0x4a80c | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x4a818 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x4a824 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x4a830 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x4a840 | `[sp-24] == 0x1` | `[sp-24] != 0x1` |
| `beq` at 0x4a84c | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x4a858 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x4a864 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x4a870 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `bne` at 0x4a888 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `beq` at 0x4a89c | `[0x29758c]:1 == 0x0` | `[0x29758c]:1 != 0x0` |
| `beq` at 0x4a8b4 | `[[sp-32]]:1 == [0x29758c]:1` | `[[sp-32]]:1 != [0x29758c]:1` |
| `beq` at 0x4a8fc | `[[sp-32]]:1 == 0x4c` | `[[sp-32]]:1 != 0x4c` |
| `beq` at 0x4a90c | `[[sp-32]]:1 == 0x4d` | `[[sp-32]]:1 != 0x4d` |
| `beq` at 0x4a91c | `[[sp-32]]:1 == 0x4e` | `[[sp-32]]:1 != 0x4e` |
| `beq` at 0x4a92c | `[[sp-32]]:1 == 0x50` | `[[sp-32]]:1 != 0x50` |
| `beq` at 0x4a93c | `[[sp-32]]:1 == 0x52` | `[[sp-32]]:1 != 0x52` |
| `beq` at 0x4a94c | `[[sp-32]]:1 == 0x54` | `[[sp-32]]:1 != 0x54` |
| `bne` at 0x4a95c | `[[sp-32]]:1 != 0x55` | `[[sp-32]]:1 == 0x55` |
| `bls` at 0x4a974 | `[[sp-32]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-32]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0x4a98c | `[[sp-32]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-32]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0x4a9c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4a9fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4aa34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4aa6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4aaa4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x4aadc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4aafc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x4ab18 | `ret(0x1955c) == 0x0` | `ret(0x1955c) != 0x0` |
| `bne` at 0x4ab24 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4ab40 | `r0` | `?` |

## `SerNumLearn`

- `SerNumLearn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1de8 to 0xa2324.
  Established. Included word assignments clear `0x133b64` to zero and write `[sp-24]` to the same address on a separate path (`str` at 0xa1ef4, `str` at 0xa1f18).
  The signed condition `[sp-24] > 0x3fff` takes `0xa1eb0`, while less than or equal falls through (`bgt` at 0xa1ea0).
  The computed transfer remains unresolved, so access, call and return records cover known edges only (`ldrls` at 0xa1f34).
  Serial-number learning policy and hardware responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1df8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa1dfc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa1e08 | store | 4 | `sp -0x1c` | `ret(0x1d444)` |
| `str` at 0xa1e18 | store | 4 | `sp -0x8c` | `?` |
| `str` at 0xa1e24 | store | 4 | `sp -0x90` | `ret(0x1d444)` |
| `str` at 0xa1e30 | store | 4 | `sp -0x94` | `ret(0x1d444)` |
| `ldr` at 0xa1e34 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1e40 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1e4c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1e58 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1e68 | load | 4 | `abs 0xa22e8` | `-` |
| `ldr` at 0xa1e6c | load | 4 | `abs 0x13246c` | `-` |
| `str` at 0xa1e7c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xa1e84 | load | 4 | `abs 0xa22e8` | `-` |
| `ldr` at 0xa1e88 | load | 4 | `abs 0x13246c` | `-` |
| `ldr` at 0xa1e94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1e98 | load | 4 | `abs 0xa22ec` | `-` |
| `str` at 0xa1ea8 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xa1eb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1eb8 | load | 4 | `abs 0xa22f0` | `-` |
| `ldr` at 0xa1ebc | load | 4 | `abs 0x133b64` | `-` |
| `ldr` at 0xa1ecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1ed8 | load | 4 | `abs 0xa22f4` | `-` |
| `ldr` at 0xa1eec | load | 4 | `abs 0xa22f0` | `-` |
| `str` at 0xa1ef4 | store | 4 | `abs 0x133b64` | `0x0` |
| `str` at 0xa1efc | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xa1f04 | load | 4 | `abs 0xa22f8` | `-` |
| `ldr` at 0xa1f08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1f10 | load | 4 | `abs 0xa22f0` | `-` |
| `ldr` at 0xa1f14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa1f18 | store | 4 | `abs 0x133b64` | `[sp-24]` |
| `ldr` at 0xa1f20 | load | 4 | `abs 0xa22f8` | `-` |
| `str` at 0xa1f24 | store | 4 | `sp -0x14` | `0x29a978` |
| `ldr` at 0xa1f28 | load | 4 | `sp -0x8c` | `-` |
| `ldrls` at 0xa1f34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa216c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2170 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa2184 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2188 | load | 4 | `abs 0xa2304` | `-` |
| `ldr` at 0xa219c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa21a0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa21ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa21c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa21d4 | load | 4 | `abs 0xa2308` | `-` |
| `ldr` at 0xa21d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa21dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa21e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa21ec | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xa21f0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xa21fc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xa2208 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xa2218 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xa2224 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xa223c | load | 4 | `abs 0xa230c` | `-` |
| `ldr` at 0xa2250 | load | 4 | `abs 0xa2310` | `-` |
| `ldr` at 0xa2264 | load | 4 | `abs 0xa2314` | `-` |
| `ldr` at 0xa2278 | load | 4 | `abs 0xa2318` | `-` |
| `ldr` at 0xa2290 | load | 4 | `abs 0xa231c` | `-` |
| `ldr` at 0xa229c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa22a0 | load | 4 | `abs 0xa22ec` | `-` |
| `str` at 0xa22bc | store | 4 | `sp -0x98` | `0x1` |
| `ldr` at 0xa22c4 | load | 4 | `abs 0xa2320` | `-` |
| `ldr` at 0xa22cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa22d8 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xa22dc | load | 4 | `sp -0x98` | `-` |
| `ldm` at 0xa22e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa22e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa22e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1e04 | `(unknown callee) 0x1d444` | `0xd` | `r1` | `r2` | `r3` |
| `bl` at 0xa1e0c | `(unknown callee) 0x488cc` | `ret(0x1d444)` | `?` | `?` | `?` |
| `bl` at 0xa1e20 | `(unknown callee) 0x1d444` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xa1e2c | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xa1edc | `(unknown callee) 0x19310` | `[sp-20]+1` | `0x1130d4` | `?` | `[sp-20]+1` |
| `bl` at 0xa1f0c | `(unknown callee) 0x19808` | `0x29a978` | `[sp-20]` | `?` | `ret(0x19310)` |
| `bl` at 0xa2174 | `(unknown callee) 0x19310` | `[sp-28]` | `?` | `?` | `[sp-140]-48` |
| `bl` at 0xa218c | `(unknown callee) 0x19310` | `[sp-28]` | `0x11318c` | `?` | `ret(0x19310)` |
| `bl` at 0xa21b0 | `(unknown callee) 0xa2324` | `[sp-20]` | `?` | `?` | `[[sp-28]]:1` |
| `bl` at 0xa21c8 | `(unknown callee) 0x1b5f0` | `0xd` | `[sp-20]` | `?` | `ret(0xa2324)` |
| `bl` at 0xa21e0 | `(unknown callee) 0x197a8` | `sp-128` | `0x113198` | `[sp-24]` | `[sp-20]` |
| `bl` at 0xa2240 | `(unknown callee) 0x195e0` | `sp-128` | `0x1131d8` | `?` | `sp-128` |
| `bl` at 0xa2254 | `(unknown callee) 0x195e0` | `sp-128` | `0x1131e0` | `?` | `sp-128` |
| `bl` at 0xa2268 | `(unknown callee) 0x195e0` | `sp-128` | `0x1131ec` | `?` | `sp-128` |
| `bl` at 0xa227c | `(unknown callee) 0x195e0` | `sp-128` | `0x1131f8` | `?` | `sp-128` |
| `bl` at 0xa2288 | `(unknown callee) 0x1e4ac` | `sp-128` | `?` | `?` | `sp-128` |
| `bl` at 0xa2298 | `(unknown callee) 0x221f0` | `0x113204` | `sp-128` | `?` | `sp-128` |
| `bl` at 0xa22b4 | `(unknown callee) 0x6a9b0` | `?` | `?` | `[sp-24]` | `?` |
| `bl` at 0xa22c8 | `(unknown callee) 0x1e4ac` | `0x113208` | `?` | `?` | `ret(0xa2324)` |
| `bl` at 0xa22d0 | `(unknown callee) 0x1e4ac` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa1e3c | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0xa1e48 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0xa1e54 | `[sp-140] == 0x36` | `[sp-140] != 0x36` |
| `beq` at 0xa1e60 | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `bne` at 0xa1e74 | `[0x13246c] != 0x2` | `[0x13246c] == 0x2` |
| `bne` at 0xa1e90 | `[0x13246c] != 0x1` | `[0x13246c] == 0x1` |
| `bgt` at 0xa1ea0 | `[sp-24] > 0x3fff (signed, 32-bit)` | `[sp-24] <= 0x3fff (signed, 32-bit)` |
| `blt` at 0xa1ec8 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `bne` at 0xa1ee8 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `ldrls` at 0xa1f34 | `[sp-140]-48 <= 0x4c (unsigned, 32-bit)` | `[sp-140]-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0xa2180 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xa2198 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `beq` at 0xa21a8 | `[[sp-28]]:1 == 0x0` | `[[sp-28]]:1 != 0x0` |
| `beq` at 0xa21bc | `ret(0xa2324) == 0x0` | `ret(0xa2324) != 0x0` |
| `beq` at 0xa21f8 | `[sp-156] == 0x2000` | `[sp-156] != 0x2000` |
| `bgt` at 0xa2204 | `[sp-156] > 0x2000 (signed, 32-bit)` | `[sp-156] <= 0x2000 (signed, 32-bit)` |
| `beq` at 0xa2210 | `[sp-156] == 0x1000` | `[sp-156] != 0x1000` |
| `beq` at 0xa2220 | `[sp-156] == 0x4000` | `[sp-156] != 0x4000` |
| `beq` at 0xa222c | `[sp-156] == 0x8000` | `[sp-156] != 0x8000` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa22e4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa1f34).

## `SerNumValid`

- `SerNumValid`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa2324 to 0xa25a8.
  Established. The byte read at `0x29758c` precedes a branch that takes `0xa2354` for nonzero and falls through for zero (`ldrb` at 0xa233c, `bne` at 0xa2344).
  Equality `ret(0x19538) == 0x8` takes `0xa2374`, while inequality falls through (`beq` at 0xa2364).
  The unnamed `0x192a4` call carries `sp-28`, `[[sp-20]]:1`, `sp-28` and `[[sp-20]]:1` (`bl` at 0xa2394).
  The hardware meaning of the accepted format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa2334 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa2338 | load | 4 | `abs 0xa259c` | `-` |
| `ldrb` at 0xa233c | load | 1 | `abs 0x29758c` | `-` |
| `str` at 0xa234c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xa2354 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa236c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa2374 | load | 4 | `abs 0xa259c` | `-` |
| `ldrb` at 0xa2378 | load | 1 | `abs 0x29758c` | `-` |
| `strb` at 0xa237c | store | 1 | `sp -0x1c` | `([0x29758c]:1 & 0xff)` |
| `ldr` at 0xa2384 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa2388 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xa23a8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa23b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa23bc | load | 4 | `abs 0xa25a0` | `-` |
| `ldr` at 0xa23d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa23dc | load | 4 | `abs 0xa25a4` | `-` |
| `str` at 0xa23f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa2400 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa2408 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xa2418 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa2420 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xa243c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa2444 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xa244c | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0xa2454 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa2474 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa247c | load | 1 | `[sp-20] +0x3` | `-` |
| `ldr` at 0xa2484 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0xa248c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa24ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa24b4 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa24bc | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0xa24c4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa24e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa24ec | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa24f4 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0xa24fc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa251c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa2524 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa252c | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0xa2534 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa2554 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa255c | load | 1 | `[sp-20] +0x7` | `-` |
| `ldr` at 0xa2564 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0xa256c | load | 2 | `(unknown)` | `-` |
| `str` at 0xa2580 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa258c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xa2590 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xa2598 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa2598 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa2598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa2358 | `(unknown callee) 0x19538` | `r0` | `r1` | `r2` | `[0x29758c]:1` |
| `bl` at 0xa2394 | `(unknown callee) 0x192a4` | `sp-28` | `[[sp-20]]:1` | `sp-28` | `[[sp-20]]:1` |
| `bl` at 0xa23c0 | `(unknown callee) 0x19310` | `[sp-20]+1` | `0x113230` | `?` | `[sp-20]+1` |
| `bl` at 0xa23e0 | `(unknown callee) 0x19310` | `[sp-20]+1` | `0x1130dc` | `?` | `[sp-20]+1` |
| `bl` at 0xa242c | `(unknown callee) 0x197d8` | `ret(0x19310)` | `?` | `0x1` | `[[sp-20]+1]:1` |
| `bl` at 0xa2464 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0xa249c | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0xa24d4 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0xa250c | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |
| `bl` at 0xa2544 | `(unknown callee) 0x197d8` | `ret(0x197d8)` | `ret(0x197d8)` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa2344 | `[0x29758c]:1 != 0x0` | `[0x29758c]:1 == 0x0` |
| `beq` at 0xa2364 | `ret(0x19538) == 0x8` | `ret(0x19538) != 0x8` |
| `bne` at 0xa23a0 | `ret(0x192a4) != 0x0` | `ret(0x192a4) == 0x0` |
| `beq` at 0xa23cc | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `bne` at 0xa23ec | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bls` at 0xa2410 | `[[sp-20]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-20]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0xa2428 | `[[sp-20]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-20]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0xa2460 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa2498 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa24d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa2508 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa2540 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa2578 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa2598 | `r0` | `?` |

## `KeyInLog`

- `KeyInLog`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4a3ec to 0x4a5c4.
  Established. The unnamed `0x1d444` call carries `0x52`, entry `r1`, entry `r2` and `sp-28` (`bl` at 0x4a408).
  Included byte reads address `([sp-44] + [sp-36]) +0x0` and `([sp-44] + [sp-36]) +0x6` (`ldrb` at 0x4a450, `ldrb` at 0x4a550).
  Inequality `[sp-20] != [sp-56]` takes `0x4a5a0`, while equality falls through (`bne` at 0x4a56c).
  Key-log record ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stm` at 0x4a400 | store | 4 | `sp -0x1c` | `r0` |
| `stm` at 0x4a400 | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x4a400 | store | 4 | `sp -0x14` | `r2` |
| `str` at 0x4a40c | store | 4 | `sp -0x20` | `ret(0x1d444)` |
| `str` at 0x4a418 | store | 4 | `sp -0x2c` | `ret(0x1d444)` |
| `str` at 0x4a420 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4a424 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4a430 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4a440 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4a444 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a448 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a450 | load | 1 | `([sp-44] + [sp-36]) +0x0` | `-` |
| `str` at 0x4a45c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4a464 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a468 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a474 | load | 1 | `([sp-44] + [sp-36]) +0x1` | `-` |
| `ldr` at 0x4a478 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x4a480 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4a484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4a488 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4a498 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a49c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a4a8 | load | 1 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0x4a4b4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x4a4bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a4c0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a4cc | load | 1 | `([sp-44] + [sp-36]) +0x3` | `-` |
| `ldr` at 0x4a4d0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x4a4d8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x4a4dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a4e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4a4f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a4f4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a500 | load | 1 | `([sp-44] + [sp-36]) +0x4` | `-` |
| `str` at 0x4a50c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x4a514 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a518 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a524 | load | 1 | `([sp-44] + [sp-36]) +0x5` | `-` |
| `ldr` at 0x4a530 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4a538 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x4a540 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a544 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4a550 | load | 1 | `([sp-44] + [sp-36]) +0x6` | `-` |
| `ldr` at 0x4a554 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4a55c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x4a560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a564 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4a570 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4a574 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4a58c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x4a598 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x4a5a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4a5a8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x4a5b4 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x4a5b8 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x4a5c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4a5c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4a5c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4a408 | `(unknown callee) 0x1d444` | `0x52` | `r1` | `r2` | `sp-28` |
| `bl` at 0x4a414 | `(unknown callee) 0x1d444` | `0x53` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x4a42c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x4a490 | `[sp-28] != [sp-48]` | `[sp-28] == [sp-48]` |
| `bne` at 0x4a4e8 | `[sp-24] != [sp-52]` | `[sp-24] == [sp-52]` |
| `bne` at 0x4a56c | `[sp-20] != [sp-56]` | `[sp-20] == [sp-56]` |
| `beq` at 0x4a584 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4a5c0 | `r0` | `?` |

## `KeyLogInsert`

- `KeyLogInsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4a1e8 to 0x4a3ec.
  Established. Included byte assignments write unknown values to `[sp-36] +0x0`, `[sp-36] +0x3` and `[sp-36] +0x6` (`strb` at 0x4a280, `strb` at 0x4a2fc, `strb` at 0x4a37c).
  The unnamed `0x1b5f0` call carries `0x51`, `[sp-29]:1`, unknown and `[sp-29]:1` (`bl` at 0x4a3c8).
  The return candidate is zero (`ldm` at 0x4a3e4).
  Persistent storage ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stm` at 0x4a1fc | store | 4 | `sp -0x1c` | `r0` |
| `stm` at 0x4a1fc | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x4a1fc | store | 4 | `sp -0x14` | `r2` |
| `strb` at 0x4a20c | store | 1 | `sp -0x1d` | `(ret(0x1d444) & 0xff)` |
| `str` at 0x4a218 | store | 4 | `sp -0x28` | `ret(0x1d444)` |
| `str` at 0x4a224 | store | 4 | `sp -0x24` | `ret(0x1d444)` |
| `ldrb` at 0x4a228 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4a238 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4a240 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x4a244 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4a254 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4a25c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4a264 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a268 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x4a280 | store | 1 | `[sp-36] +0x0` | `?` |
| `str` at 0x4a288 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x4a290 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a294 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x4a2a8 | store | 1 | `[sp-36] +0x1` | `?` |
| `str` at 0x4a2b0 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0x4a2b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a2bc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x4a2d4 | store | 1 | `[sp-36] +0x2` | `?` |
| `str` at 0x4a2dc | store | 4 | `sp -0x24` | `[sp-36]+3` |
| `ldr` at 0x4a2e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a2e8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x4a2fc | store | 1 | `[sp-36] +0x3` | `?` |
| `str` at 0x4a304 | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0x4a30c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a310 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4a328 | store | 1 | `[sp-36] +0x4` | `?` |
| `str` at 0x4a330 | store | 4 | `sp -0x24` | `[sp-36]+5` |
| `ldr` at 0x4a338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a33c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4a354 | store | 1 | `[sp-36] +0x5` | `?` |
| `str` at 0x4a35c | store | 4 | `sp -0x24` | `[sp-36]+6` |
| `ldr` at 0x4a364 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a368 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4a37c | store | 1 | `[sp-36] +0x6` | `?` |
| `str` at 0x4a384 | store | 4 | `sp -0x24` | `[sp-36]+7` |
| `ldrb` at 0x4a388 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4a390 | load | 4 | `abs 0x4a3e8` | `-` |
| `strb` at 0x4a3b8 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0x4a3bc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4a3d0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x4a3e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4a3e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4a3e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4a204 | `(unknown callee) 0x1d444` | `0x51` | `r1` | `r2` | `sp-28` |
| `bl` at 0x4a214 | `(unknown callee) 0x1d444` | `0x52` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x4a220 | `(unknown callee) 0x1d444` | `0x53` | `?` | `?` | `?` |
| `bl` at 0x4a3c8 | `(unknown callee) 0x1b5f0` | `0x51` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x4a3d4 | `(unknown callee) 0x1b5f0` | `0x52` | `[sp-40]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4a3e4 | `r0` | `0x0` |

## `decode`

- `decode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x498dc to 0x499cc.
  Established. A byte read addresses `0x299840` (`ldrb` at 0x49928).
  The comparison branch takes `0x49990` when an unknown value is not equal to `[sp-17]:1`, while equality falls through (`bne` at 0x49968).
  An included word assignment writes `[sp-24]` to `sp -0x20` (`str` at 0x49988).
  Decoding table entries remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x498f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x498f8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x498fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4990c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4991c | load | 4 | `abs 0x499c4` | `-` |
| `ldr` at 0x49920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49924 | load | 4 | `abs 0x499c8` | `-` |
| `ldrb` at 0x49928 | load | 1 | `abs 0x299840` | `-` |
| `ldr` at 0x49954 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4995c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x49960 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4996c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4997c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49984 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x49988 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x49990 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49998 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x499a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x499a8 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x499b4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x499b8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x499c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x499c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x499c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x49904 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0x49918 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x49968 | `? != [sp-17]:1` | `? == [sp-17]:1` |
| `ble` at 0x49974 | `[sp-24] <= 0x9 (signed, 32-bit)` | `[sp-24] > 0x9 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x499c0 | `r0` | `[sp-32]` |

## `make_bin`

- `make_bin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49050 to 0x4914c.
  Established. Included byte assignments write `0x0` to `r1 +0x0`, `0x1` to `([sp-24] + [sp-36]) +0x0` and `0x0` to that same printed expression (`strb` at 0x49090, `strb` at 0x490c0, `strb` at 0x49110).
  The signed condition `[sp-36] < 0x0` takes `0x4913c`, while nonnegative falls through (`blt` at 0x4909c).
  Binary digit record conventions remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49060 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x49064 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4906c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x49074 | store | 4 | `sp -0x20` | `0x40000000` |
| `str` at 0x4907c | store | 4 | `sp -0x24` | `0x1e` |
| `str` at 0x49084 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x49088 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x49090 | store | 1 | `r1 +0x0` | `0x0` |
| `ldr` at 0x49094 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x490a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x490a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x490b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x490b4 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x490c0 | store | 1 | `([sp-24] + [sp-36]) +0x0` | `0x1` |
| `ldr` at 0x490c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x490d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x490d4 | store | 4 | `sp -0x28` | `[sp-36]` |
| `str` at 0x490dc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x490e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x490e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x490ec | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x490f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49104 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x49110 | store | 1 | `([sp-24] + [sp-36]) +0x0` | `0x0` |
| `ldr` at 0x49114 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4911c | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldr` at 0x49120 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49134 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4913c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x49148 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x49148 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x49148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x4909c | `[sp-36] < 0x0 (signed, 32-bit)` | `[sp-36] >= 0x0 (signed, 32-bit)` |
| `blt` at 0x490ac | `[sp-20] < [sp-32] (signed, 32-bit)` | `[sp-20] >= [sp-32] (signed, 32-bit)` |
| `bne` at 0x490cc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x490fc | `[sp-28] != 0x1` | `[sp-28] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x49148 | `r0` | `[sp-40]` |

## `inv_manip`

- `inv_manip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49b08 to 0x49c2c.
  Established. The unsigned condition `r1-1 <= 0x6` selects an unresolved computed transfer, while greater falls through (`ldrls` at 0x49b2c).
  On the known fallback, included word assignments write `r0-9` to `sp -0x14` and `sp -0x1c` (`str` at 0x49c14, `str` at 0x49c1c).
  Selector operations behind the unresolved edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49b18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x49b1c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x49b20 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x49b2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x49c0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49c14 | store | 4 | `sp -0x14` | `r0-9` |
| `ldr` at 0x49c18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49c1c | store | 4 | `sp -0x1c` | `r0-9` |
| `ldr` at 0x49c20 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x49c28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x49c28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x49c28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x49b2c | `r1-1 <= 0x6 (unsigned, 32-bit)` | `r1-1 > 0x6 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x49c28 | `r0` | `r0-9` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x49b2c).

## `descramble`

- `descramble`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49608 to 0x49798.
  Established. The unnamed `0x197a8` call carries `sp-48`, `0xf7f9c`, entry `r1`, entry `r2` (`bl` at 0x49664).
  Equality `[sp+4] == ret(0x498dc)` takes `0x496ec`, while inequality falls through (`beq` at 0x496d4).
  An included byte assignment writes an unknown value to `0x299840` (`strb` at 0x496a0).
  Descrambled serial field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49618 | store | 4 | `sp -0x54` | `r0` |
| `str` at 0x4961c | store | 4 | `sp -0x14` | `r1` |
| `str` at 0x49620 | store | 4 | `sp -0x18` | `r2` |
| `str` at 0x49624 | store | 4 | `sp -0x1c` | `r3` |
| `str` at 0x4962c | store | 4 | `sp -0x50` | `0x1` |
| `str` at 0x49634 | store | 4 | `sp -0x4c` | `0x1` |
| `str` at 0x4963c | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0x49644 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49648 | store | 4 | `sp -0x5c` | `r3` |
| `ldr` at 0x4964c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x49650 | store | 4 | `sp -0x58` | `[sp]` |
| `ldr` at 0x49658 | load | 4 | `abs 0x4978c` | `-` |
| `ldr` at 0x4965c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49660 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4966c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x49670 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x4967c | load | 4 | `abs 0x49790` | `-` |
| `ldr` at 0x49680 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x49688 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4968c | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0x49698 | load | 4 | `abs 0x49794` | `-` |
| `ldr` at 0x4969c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x496a0 | store | 1 | `abs 0x299840` | `?` |
| `ldr` at 0x496a4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x496ac | store | 4 | `sp -0x40` | `?` |
| `ldrb` at 0x496b4 | load | 1 | `sp -0x2f` | `-` |
| `str` at 0x496c4 | store | 4 | `sp -0x44` | `ret(0x498dc)` |
| `ldr` at 0x496c8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x496cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x496d8 | load | 4 | `sp -0x54` | `-` |
| `ldm` at 0x496e0 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x496e0 | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0x496e0 | load | 4 | `sp -0x48` | `-` |
| `stm` at 0x496e4 | store | 4 | `[sp-84] +0x0` | `[sp-80]` |
| `stm` at 0x496e4 | store | 4 | `[sp-84] +0x4` | `[sp-76]` |
| `stm` at 0x496e4 | store | 4 | `[sp-84] +0x8` | `[sp-72]` |
| `str` at 0x49704 | store | 4 | `sp -0x34` | `ret(0x49a34)` |
| `str` at 0x49720 | store | 4 | `sp -0x38` | `ret(0x49a34)` |
| `str` at 0x4973c | store | 4 | `sp -0x3c` | `ret(0x49a34)` |
| `ldr` at 0x49740 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x49744 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x4974c | store | 4 | `sp -0x50` | `ret(0x49b08)` |
| `ldr` at 0x49750 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x49754 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x4975c | store | 4 | `sp -0x4c` | `ret(0x49b08)` |
| `ldr` at 0x49760 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x49764 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x4976c | store | 4 | `sp -0x48` | `ret(0x49b08)` |
| `ldr` at 0x49770 | load | 4 | `sp -0x54` | `-` |
| `ldm` at 0x49778 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x49778 | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0x49778 | load | 4 | `sp -0x48` | `-` |
| `stm` at 0x4977c | store | 4 | `[sp-84] +0x0` | `[sp-80]` |
| `stm` at 0x4977c | store | 4 | `[sp-84] +0x4` | `[sp-76]` |
| `stm` at 0x4977c | store | 4 | `[sp-84] +0x8` | `ret(0x49b08)` |
| `ldr` at 0x49780 | load | 4 | `sp -0x54` | `-` |
| `ldm` at 0x49788 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x49788 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x49788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49664 | `(unknown callee) 0x197a8` | `sp-48` | `0xf7f9c` | `r1` | `r2` |
| `bl` at 0x496bc | `(unknown callee) 0x498dc` | `[sp-47]:1` | `?` | `?` | `[sp-47]:1` |
| `bl` at 0x496fc | `(unknown callee) 0x49a34` | `sp-48` | `0x2` | `0x6` | `sp-48` |
| `bl` at 0x49718 | `(unknown callee) 0x49a34` | `sp-48` | `0x7` | `0xb` | `sp-48` |
| `bl` at 0x49734 | `(unknown callee) 0x49a34` | `sp-48` | `0xc` | `0x11` | `sp-48` |
| `bl` at 0x49748 | `(unknown callee) 0x49b08` | `[sp-52]` | `[sp-68]` | `?` | `ret(0x49a34)` |
| `bl` at 0x49758 | `(unknown callee) 0x49b08` | `[sp-56]` | `[sp-68]` | `?` | `?` |
| `bl` at 0x49768 | `(unknown callee) 0x49b08` | `[sp-60]` | `[sp-68]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x49678 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x49694 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x496d4 | `[sp+4] == ret(0x498dc)` | `[sp+4] != ret(0x498dc)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x49788 | `r0` | `[sp-84]` |

## `rsam`

- `rsam`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4914c to 0x4925c.
  Established. The unnamed `0x49050` call carries entry `r1`, `sp-80`, entry `r2`, `sp-80` (`bl` at 0x49194).
  A predicate `[sp-48] != 0x1` takes `0x4923c`, while equality falls through (`bne` at 0x49204).
  An included unnamed `0xeb368` call carries `[sp-44]`, `[sp-28]`, `[sp-36]`, `[sp-44]` (`bl` at 0x49230).
  Serial arithmetic parameter meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4915c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x49160 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x49164 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x4916c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x49174 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x4917c | store | 4 | `sp -0x28` | `0x1` |
| `str` at 0x49184 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x4918c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4919c | store | 4 | `sp -0x54` | `ret(0x49050)` |
| `ldr` at 0x491a0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x491ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x491b4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x491b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x491bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x491c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x491c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x491d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x491dc | store | 4 | `sp -0x24` | `ret(0xeb368)` |
| `ldr` at 0x491e4 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x491f4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x491f8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x491fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x49208 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49210 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x49214 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49218 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49220 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x49224 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4922c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49238 | store | 4 | `sp -0x24` | `ret(0xeb368)` |
| `ldr` at 0x4923c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x49244 | store | 4 | `sp -0x54` | `[sp-84]-1` |
| `ldr` at 0x4924c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x49258 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x49258 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x49258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49194 | `(unknown callee) 0x49050` | `r1` | `sp-80` | `r2` | `sp-80` |
| `bl` at 0x491d4 | `(unknown callee) 0xeb368` | `[sp-40]` | `[sp-28]` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x49230 | `(unknown callee) 0xeb368` | `[sp-44]` | `[sp-28]` | `[sp-36]` | `[sp-44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x491a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x49204 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x49258 | `r0` | `[sp-36]` |

## `ramLogInit`

- `ramLogInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacd00 to 0xacde4.
  Established. The unnamed `0x1943c` call carries `0x18`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0xacd1c).
  A nonzero result takes `0xacd40`, while zero falls through (`bne` at 0xacd30).
  Included word stores write `[sp-28]` to `[sp-32] +0x0` and `([[sp-32]+4] + [sp-24])` to `[sp-32] +0xc` (`str` at 0xacd84, `str` at 0xacdac).
  Log record field ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacd10 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xacd14 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xacd24 | store | 4 | `sp -0x20` | `ret(0x1943c)` |
| `ldr` at 0xacd28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xacd38 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xacd40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacd44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacd50 | store | 4 | `ret(0x1943c) +0x4` | `ret(0x1943c)` |
| `ldr` at 0xacd54 | load | 4 | `ret(0x1943c) +0x4` | `-` |
| `ldr` at 0xacd60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xacd6c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xacd74 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xacd7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacd80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xacd84 | store | 4 | `[sp-32] +0x0` | `[sp-28]` |
| `ldr` at 0xacd88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacd8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacd90 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xacd94 | store | 4 | `[sp-32] +0x8` | `[[sp-32]+4]` |
| `ldr` at 0xacd98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacd9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacda0 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xacda4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacdac | store | 4 | `[sp-32] +0xc` | `([[sp-32]+4] + [sp-24])` |
| `ldr` at 0xacdb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacdb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacdb8 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xacdbc | store | 4 | `[sp-32] +0x10` | `[[sp-32]+4]` |
| `ldr` at 0xacdc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacdc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacdc8 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xacdcc | store | 4 | `[sp-32] +0x14` | `[[sp-32]+4]` |
| `ldr` at 0xacdd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xacdd4 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xacdd8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xacde0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xacde0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xacde0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xacde0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacd1c | `(unknown callee) 0x1943c` | `0x18` | `r1` | `r2` | `r3` |
| `bl` at 0xacd48 | `(unknown callee) 0x1943c` | `[sp-24]` | `?` | `?` | `ret(0x1943c)` |
| `bl` at 0xacd64 | `(unknown callee) 0x1970c` | `[sp-32]` | `?` | `?` | `[ret(0x1943c)+4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xacd30 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xacd5c | `[ret(0x1943c)+4] != 0x0` | `[ret(0x1943c)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xacde0 | `r0` | `?` |
