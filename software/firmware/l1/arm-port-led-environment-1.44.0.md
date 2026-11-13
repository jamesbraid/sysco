# ARM port, LED and environment paths in 1.44.0

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
Computed transfers leave unresolved edges in the event routine, identified below. Tables cover known control-flow edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/port-led-environment-facts.txt) retain all tool sections.

## `portInt`

- `portInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c914 to 0x3cdd8.
  Established. Equality `ret(0x20cd0) == 0x0` takes `0x3c988`, while inequality falls through (`beq` at 0x3c960).
  An included unnamed `0x3cf90` call carries `[sp-24]`, `[sp-32]` and two unknown candidates (`bl` at 0x3cb84).
  The unnamed `0x2105c` call carries `0x1321b4` followed by three unknown candidates (`bl` at 0x3cd50).
  Port interrupt handling beyond these calls remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c924 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0x3c930 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0x3c938 | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `strb` at 0x3c940 | store | 1 | `sp -0x1b` | `(r3 & 0xff)` |
| `ldr` at 0x3c944 | load | 4 | `abs 0x3cd94` | `-` |
| `str` at 0x3c954 | store | 4 | `sp -0x2c` | `ret(0x20cd0)` |
| `ldr` at 0x3c958 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c964 | load | 4 | `abs 0x3cd98` | `-` |
| `str` at 0x3c968 | store | 4 | `sp -0x44` | `0xf61c0` |
| `ldr` at 0x3c96c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3c970 | store | 4 | `sp -0x40` | `ret(0x20cd0)` |
| `ldr` at 0x3c978 | load | 4 | `abs 0x3cd9c` | `-` |
| `ldr` at 0x3c97c | load | 4 | `abs 0x3cda0` | `-` |
| `ldrb` at 0x3c988 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3c98c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c99c | store | 4 | `sp -0x20` | `ret(0x3d930)` |
| `ldrb` at 0x3c9a0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3c9a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c9b4 | store | 4 | `sp -0x24` | `ret(0x3da1c)` |
| `ldr` at 0x3c9b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c9c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c9d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c9d8 | load | 4 | `abs 0x3cda4` | `-` |
| `ldr` at 0x3c9dc | load | 4 | `abs 0x3cda8` | `-` |
| `str` at 0x3ca18 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3ca1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ca24 | load | 4 | `abs 0x3cda4` | `-` |
| `ldr` at 0x3ca28 | load | 4 | `abs 0x3cda8` | `-` |
| `ldr` at 0x3ca34 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3ca44 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3ca48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ca58 | load | 4 | `abs 0x3cdac` | `-` |
| `str` at 0x3ca5c | store | 4 | `(unknown)` | `0xf61e4` |
| `str` at 0x3ca60 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x3ca64 | store | 4 | `(unknown)` | `ret(0xcb12c)` |
| `ldrb` at 0x3ca68 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x3ca6c | store | 4 | `(unknown)` | `[sp-26]:1` |
| `ldr` at 0x3ca74 | load | 4 | `abs 0x3cd9c` | `-` |
| `ldrb` at 0x3ca84 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x3ca88 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x3ca8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ca98 | load | 4 | `abs 0x3cdb0` | `-` |
| `ldr` at 0x3ca9c | load | 4 | `abs 0x132630` | `-` |
| `ldr` at 0x3caa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cab0 | load | 4 | `abs 0x3cdb4` | `-` |
| `ldr` at 0x3cab4 | load | 4 | `abs 0x3cdb8` | `-` |
| `str` at 0x3caf0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3caf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cafc | load | 4 | `abs 0x3cdb4` | `-` |
| `ldr` at 0x3cb00 | load | 4 | `abs 0x3cdb8` | `-` |
| `ldr` at 0x3cb0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3cb1c | load | 4 | `abs 0x3cdbc` | `-` |
| `str` at 0x3cb20 | store | 4 | `(unknown)` | `0xf622c` |
| `str` at 0x3cb24 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x3cb28 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x3cb2c | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldrb` at 0x3cb30 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x3cb34 | store | 4 | `(unknown)` | `[sp-26]:1` |
| `ldrb` at 0x3cb38 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x3cb3c | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldr` at 0x3cb44 | load | 4 | `abs 0x3cd9c` | `-` |
| `ldr` at 0x3cb48 | load | 4 | `abs 0x3cdc0` | `-` |
| `ldr` at 0x3cb54 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3cb60 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x3cb68 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x3cb74 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x3cb78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cb7c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3cb8c | store | 4 | `sp -0x28` | `ret(0x3cf90)` |
| `ldr` at 0x3cb90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cb9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cba4 | load | 4 | `abs 0x3cda4` | `-` |
| `ldr` at 0x3cba8 | load | 4 | `abs 0x3cda8` | `-` |
| `str` at 0x3cbe4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3cbe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cbf0 | load | 4 | `abs 0x3cda4` | `-` |
| `ldr` at 0x3cbf4 | load | 4 | `abs 0x3cda8` | `-` |
| `ldr` at 0x3cc00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3cc10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cc1c | load | 4 | `abs 0x3cdc4` | `-` |
| `str` at 0x3cc20 | store | 4 | `(unknown)` | `0xf625c` |
| `ldr` at 0x3cc24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3cc28 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0x3cc2c | store | 4 | `(unknown)` | `[[sp-32]+8]` |
| `str` at 0x3cc30 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x3cc34 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x3cc3c | load | 4 | `abs 0x3cd9c` | `-` |
| `ldr` at 0x3cc40 | load | 4 | `abs 0x3cdc8` | `-` |
| `ldr` at 0x3cc4c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3cc58 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x3cc64 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0x3cc74 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x3cc78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cc7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cc8c | store | 4 | `sp -0x28` | `ret(0x3cf90)` |
| `ldr` at 0x3cc90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cc9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cca4 | load | 4 | `abs 0x3cda4` | `-` |
| `ldr` at 0x3cca8 | load | 4 | `abs 0x3cda8` | `-` |
| `str` at 0x3cce4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3cce8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ccf0 | load | 4 | `abs 0x3cda4` | `-` |
| `ldr` at 0x3ccf4 | load | 4 | `abs 0x3cda8` | `-` |
| `ldr` at 0x3cd00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3cd10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cd1c | load | 4 | `abs 0x3cdc4` | `-` |
| `str` at 0x3cd20 | store | 4 | `(unknown)` | `0xf625c` |
| `ldr` at 0x3cd24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cd28 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x3cd2c | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `str` at 0x3cd30 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0x3cd34 | store | 4 | `(unknown)` | `ret(0x1fc1c)` |
| `ldr` at 0x3cd3c | load | 4 | `abs 0x3cd9c` | `-` |
| `ldr` at 0x3cd40 | load | 4 | `abs 0x3cdcc` | `-` |
| `ldr` at 0x3cd4c | load | 4 | `abs 0x3cd94` | `-` |
| `str` at 0x3cd58 | store | 4 | `sp -0x2c` | `ret(0x2105c)` |
| `ldr` at 0x3cd5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cd68 | load | 4 | `abs 0x3cdd0` | `-` |
| `str` at 0x3cd6c | store | 4 | `(unknown)` | `0xf6278` |
| `ldr` at 0x3cd70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3cd74 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3cd7c | load | 4 | `abs 0x3cd9c` | `-` |
| `ldr` at 0x3cd80 | load | 4 | `abs 0x3cdd4` | `-` |
| `ldm` at 0x3cd90 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x3cd90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3cd90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3cd90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c94c | `(unknown callee) 0x20cd0` | `0x1321b4` | `?` | `r2` | `r3` |
| `bl` at 0x3c984 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0x2ef` | `0x0` |
| `bl` at 0x3c994 | `(unknown callee) 0x3d930` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x3c9ac | `(unknown callee) 0x3da1c` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x3c9e0 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3c9ec | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3ca2c | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3ca3c | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3ca50 | `(unknown callee) 0xcb12c` | `[sp-24]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x3ca80 | `(unknown callee) 0x200a0` | `0x1` | `0xf5e64` | `0x2f8` | `0x0` |
| `bl` at 0x3ca94 | `(unknown callee) 0xca878` | `[sp-24]` | `[sp-25]:1` | `[sp-26]:1` | `[sp-25]:1` |
| `bl` at 0x3cab8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf6220` | `0xf6228` |
| `bl` at 0x3cac4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3cb04 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf6220` | `0xf6228` |
| `bl` at 0x3cb14 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3cb50 | `(unknown callee) 0x200a0` | `0x6` | `0xf5e64` | `0x2ff` | `0x0` |
| `bl` at 0x3cb84 | `(unknown callee) 0x3cf90` | `[sp-24]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3cbac | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3cbb8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3cbf8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3cc08 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3cc14 | `(unknown callee) 0x1fc1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x3cc48 | `(unknown callee) 0x200a0` | `0x3` | `0xf5e64` | `0x307` | `0x0` |
| `bl` at 0x3cc84 | `(unknown callee) 0x3cf90` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x3ccac | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3ccb8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3ccf8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xf5eb0` | `0xf5eb8` |
| `bl` at 0x3cd08 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x3cd14 | `(unknown callee) 0x1fc1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x3cd48 | `(unknown callee) 0x200a0` | `0x3` | `0xf5e64` | `0x30d` | `0x0` |
| `bl` at 0x3cd50 | `(unknown callee) 0x2105c` | `0x1321b4` | `?` | `?` | `?` |
| `bl` at 0x3cd88 | `(unknown callee) 0x200a0` | `0x9` | `0xf5e64` | `0x315` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c960 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `bne` at 0x3c9c0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x3c9cc | `ret(0x3da1c) != 0x0` | `ret(0x3da1c) == 0x0` |
| `beq` at 0x3caa4 | `[0x132630] == 0x0` | `[0x132630] != 0x0` |
| `beq` at 0x3cb5c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x3cb98 | `ret(0x3cf90) == 0x0` | `ret(0x3cf90) != 0x0` |
| `beq` at 0x3cc54 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3cc98 | `ret(0x3cf90) == 0x0` | `ret(0x3cf90) != 0x0` |
| `beq` at 0x3cd64 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3cd90 | `r0` | `?` |

## `portStatus`

- `portStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3bf68 to 0x3c3f8.
  Established. The unnamed `0xca70c` call carries `[sp-20]`, `[[sp-32]]:1`, `[sp-28]`, `[[sp-32]]:1` (`bl` at 0x3c004).
  A zero `ret(0xca70c)` takes `0x3c028`, while nonzero falls through (`beq` at 0x3c018).
  Included byte and word stores write `([sp-57]:1 & 0xff)` and `[sp-64]` to `[sp-28] +0x0` (`strb` at 0x3c080, `str` at 0x3c1a0).
  Port status bit definitions remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3bf78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3bf80 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x3bf84 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x3bf8c | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0x3bf90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3bf94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3bfa4 | store | 4 | `sp -0x20` | `ret(0x3d824)` |
| `ldr` at 0x3bfa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3bfb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3bfc0 | store | 4 | `sp -0x34` | `ret(0x3d8bc)` |
| `ldrb` at 0x3bfc8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3bfcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3bfe4 | load | 4 | `abs 0x3c3f0` | `-` |
| `str` at 0x3bfe8 | store | 4 | `sp -0x34` | `0xfffffb4e` |
| `ldr` at 0x3bff0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3bff4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x3bff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c000 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c00c | store | 4 | `sp -0x24` | `ret(0xca70c)` |
| `ldr` at 0x3c010 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c01c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c020 | store | 4 | `sp -0x34` | `ret(0xca70c)` |
| `ldr` at 0x3c028 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3c02c | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3c03c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c040 | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x3c044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c048 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3c04c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3c050 | load | 1 | `[sp-32] +0x1` | `-` |
| `strb` at 0x3c068 | store | 1 | `sp -0x39` | `0x4` |
| `strb` at 0x3c074 | store | 1 | `sp -0x39` | `0x0` |
| `ldrb` at 0x3c078 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x3c07c | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3c080 | store | 1 | `[sp-28] +0x0` | `([sp-57]:1 & 0xff)` |
| `ldr` at 0x3c088 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3c08c | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x3c098 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c09c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c0a0 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x3c0a8 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x3c0ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c0b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c0b4 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x3c0bc | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x3c0c0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3c0c4 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3c0d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c0d8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3c0e4 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3c0ec | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3c0f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c0f4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3c100 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3c108 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3c10c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c110 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3c11c | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3c124 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3c128 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c12c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3c138 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3c140 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3c144 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c148 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3c14c | store | 1 | `[sp-28] +0x0` | `([sp-37]:1 & 0xff)` |
| `ldr` at 0x3c150 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3c154 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3c164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c168 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c16c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c17c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c180 | store | 4 | `sp -0x40` | `[[sp-28]]` |
| `ldr` at 0x3c188 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3c198 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3c19c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3c1a0 | store | 4 | `[sp-28] +0x0` | `[sp-64]` |
| `ldr` at 0x3c1a4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3c1a8 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3c1b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c1bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c1c0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c1d0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c1d4 | store | 4 | `sp -0x44` | `[[sp-28]]` |
| `ldr` at 0x3c1dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3c1ec | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x3c1f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3c1f4 | store | 4 | `[sp-28] +0x0` | `[sp-68]` |
| `ldr` at 0x3c1f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3c1fc | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x3c204 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c214 | store | 4 | `sp -0x24` | `ret(0xca70c)` |
| `ldr` at 0x3c218 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c224 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c228 | store | 4 | `sp -0x34` | `ret(0xca70c)` |
| `ldr` at 0x3c230 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3c234 | load | 1 | `sp -0x25` | `-` |
| `ldrb` at 0x3c238 | load | 1 | `[sp-32] +0x3` | `-` |
| `strb` at 0x3c240 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3c244 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3c248 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3c258 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c25c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x3c26c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c270 | store | 4 | `sp -0x48` | `[[sp-28]]` |
| `ldr` at 0x3c278 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3c288 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x3c28c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3c290 | store | 4 | `[sp-28] +0x0` | `[sp-72]` |
| `ldr` at 0x3c298 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c29c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x3c2ac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c2b0 | store | 4 | `sp -0x4c` | `[[sp-28]]` |
| `ldr` at 0x3c2b8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3c2c8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x3c2cc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3c2d0 | store | 4 | `[sp-28] +0x0` | `[sp-76]` |
| `ldr` at 0x3c2d4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3c2d8 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3c2e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c2ec | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c300 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c304 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c318 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c31c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c33c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3c340 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3c34c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3c358 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3c368 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x3c37c | store | 4 | `sp -0x30` | `0x2` |
| `str` at 0x3c388 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3c390 | load | 4 | `abs 0x3c3f4` | `-` |
| `ldr` at 0x3c3a0 | load | 4 | `sp -0x30` | `-` |
| `ldc` at 0x3c3ac | load | unknown | `sp -0x2c` | `-` |
| `ldc` at 0x3c3b4 | load | unknown | `abs 0x3c3e8` | `-` |
| `ldr` at 0x3c3c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c3c8 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x3c3d0 | store | 1 | `[sp-28] +0x0` | `?` |
| `str` at 0x3c3d8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3c3dc | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x3c3e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3c3e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3c3e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3bf9c | `(unknown callee) 0x3d824` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x3bfb8 | `(unknown callee) 0x3d8bc` | `[sp-20]` | `?` | `?` | `ret(0x3d824)` |
| `bl` at 0x3bfd4 | `(unknown callee) 0x3b628` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x3c004 | `(unknown callee) 0xca70c` | `[sp-20]` | `[[sp-32]]:1` | `[sp-28]` | `[[sp-32]]:1` |
| `bl` at 0x3c20c | `(unknown callee) 0xca70c` | `[sp-20]` | `[[sp-32]+2]:1` | `sp-37` | `[[sp-32]+2]:1` |
| `bl` at 0x3c330 | `(unknown callee) 0x488cc` | `[[sp-28]]` | `[sp-28]` | `?` | `?` |
| `bl` at 0x3c394 | `(unknown callee) 0x1e4ac` | `0xf6010` | `?` | `?` | `[sp-80]` |
| `bl` at 0x3c3a8 | `(unknown callee) 0x398cc` | `0x0` | `?` | `0x0` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3bfb0 | `ret(0x3d824) != 0x0` | `ret(0x3d824) == 0x0` |
| `bne` at 0x3bfe0 | `ret(0x3b628) != 0x0` | `ret(0x3b628) == 0x0` |
| `beq` at 0x3c018 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `beq` at 0x3c038 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c060 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c094 | `[[sp-32]+1]:1 == 0x0` | `[[sp-32]+1]:1 != 0x0` |
| `beq` at 0x3c0d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c160 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c1b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c220 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `beq` at 0x3c254 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c2e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c2fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3c314 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x3c32c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3c348 | `[sp-80] == 0x3d` | `[sp-80] != 0x3d` |
| `bgt` at 0x3c354 | `[sp-80] > 0x3d (signed, 32-bit)` | `[sp-80] <= 0x3d (signed, 32-bit)` |
| `beq` at 0x3c360 | `[sp-80] == 0x23` | `[sp-80] != 0x23` |
| `beq` at 0x3c370 | `[sp-80] == 0x4f` | `[sp-80] != 0x4f` |
| `bmi` at 0x3c3bc | `N == 1` | `not (N == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3c3e4 | `r0` | `?` |

## `pcixLedOn`

- `pcixLedOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x500b4 to 0x50318.
  Established. Inequality `[sp-21]:1 != 0xff` takes `0x501d0`, while equality falls through (`bne` at 0x50148).
  An included unnamed `0x53f84` call carries `[sp-20]`, `ret(0x53100)`, `0x1`, `ret(0x53100)` (`bl` at 0x502f4).
  PCI-X LED-on command interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x500c4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x500cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x500d4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x500dc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x500e0 | load | 4 | `abs 0x5030c` | `-` |
| `ldr` at 0x500e4 | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x500f0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50100 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x50108 | load | 4 | `abs 0x50310` | `-` |
| `str` at 0x5010c | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x50110 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x50114 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x5011c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50130 | load | 4 | `abs 0x50314` | `-` |
| `ldr` at 0x50134 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x5013c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x50140 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50150 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x50154 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50158 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x5015c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50168 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5016c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50174 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x5017c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50190 | store | 4 | `sp -0x2c` | `ret(0x53f84)` |
| `ldr` at 0x50194 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x501a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x501ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x501b0 | store | 4 | `sp -0x28` | `ret(0x53f84)` |
| `ldr` at 0x501b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x501bc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x501c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x501c8 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x501d0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x501dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x501e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x501f0 | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x501f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50200 | load | 4 | `abs 0x50310` | `-` |
| `str` at 0x50204 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x5020c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50210 | load | 1 | `ret(0x53024) +0x1` | `-` |
| `str` at 0x50214 | store | 4 | `sp -0x30` | `[ret(0x53024)+1]:1` |
| `ldr` at 0x50218 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5021c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x50220 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50224 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5022c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50238 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5023c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50244 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x5024c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50260 | store | 4 | `sp -0x2c` | `ret(0x53f84)` |
| `ldr` at 0x50264 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50270 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5027c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50280 | store | 4 | `sp -0x28` | `ret(0x53f84)` |
| `ldr` at 0x50284 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5028c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x50294 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50298 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x502a0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x502a4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x502a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x502b8 | store | 4 | `sp -0x24` | `ret(0x53100)` |
| `ldr` at 0x502bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x502c8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x502cc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x502d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x502e0 | store | 4 | `sp -0x34` | `ret(0x532e0)` |
| `ldr` at 0x502e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x502ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x502fc | store | 4 | `sp -0x34` | `ret(0x53f84)` |
| `ldr` at 0x50300 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x50308 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x50308 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x50308 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50188 | `(unknown callee) 0x53f84` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x501e8 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x50258 | `(unknown callee) 0x53f84` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x502b0 | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x502d8 | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x502f4 | `(unknown callee) 0x53f84` | `[sp-20]` | `ret(0x53100)` | `0x1` | `ret(0x53100)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x500ec | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x500f8 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x50148 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x50164 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x5019c | `ret(0x53f84) == 0x0` | `ret(0x53f84) != 0x0` |
| `bne` at 0x501a8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x501d8 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x501fc | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `ble` at 0x50234 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x5026c | `ret(0x53f84) == 0x0` | `ret(0x53f84) != 0x0` |
| `bne` at 0x50278 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x502c4 | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x50308 | `r0` | `?` |

## `pcixLedOff`

- `pcixLedOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50318 to 0x5057c.
  Established. Included unnamed `0x53f84` calls carry `[sp-20]`, unknown, `0x0`, unknown and `[sp-20]`, unknown, `0x1`, unknown (`bl` at 0x503ec, `bl` at 0x504bc).
  A later unnamed call to that target carries `[sp-20]`, `ret(0x53100)`, `0x0`, `ret(0x53100)` (`bl` at 0x50558).
  PCI-X LED-off polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50328 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x50330 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x50338 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x50340 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x50344 | load | 4 | `abs 0x50570` | `-` |
| `ldr` at 0x50348 | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x50354 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x50364 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x5036c | load | 4 | `abs 0x50574` | `-` |
| `str` at 0x50370 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x50374 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x50378 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x50380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50394 | load | 4 | `abs 0x50578` | `-` |
| `ldr` at 0x50398 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x503a0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x503a4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x503b4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x503b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x503bc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x503c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x503cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x503d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x503d8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x503e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x503f4 | store | 4 | `sp -0x2c` | `ret(0x53f84)` |
| `ldr` at 0x503f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50404 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50410 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50414 | store | 4 | `sp -0x28` | `ret(0x53f84)` |
| `ldr` at 0x50418 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x50420 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x50428 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5042c | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x50434 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x50440 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x50444 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50454 | store | 4 | `sp -0x20` | `ret(0x53024)` |
| `ldr` at 0x50458 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50464 | load | 4 | `abs 0x50574` | `-` |
| `str` at 0x50468 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x50470 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50474 | load | 1 | `ret(0x53024) +0x1` | `-` |
| `str` at 0x50478 | store | 4 | `sp -0x30` | `[ret(0x53024)+1]:1` |
| `ldr` at 0x5047c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50480 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x50484 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x50488 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x50490 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5049c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x504a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x504a8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x504b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x504c4 | store | 4 | `sp -0x2c` | `ret(0x53f84)` |
| `ldr` at 0x504c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x504d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x504e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x504e4 | store | 4 | `sp -0x28` | `ret(0x53f84)` |
| `ldr` at 0x504e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x504f0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x504f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x504fc | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x50504 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50508 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5050c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5051c | store | 4 | `sp -0x24` | `ret(0x53100)` |
| `ldr` at 0x50520 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5052c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50530 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50534 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50544 | store | 4 | `sp -0x34` | `ret(0x532e0)` |
| `ldr` at 0x5054c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50550 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x50560 | store | 4 | `sp -0x34` | `ret(0x53f84)` |
| `ldr` at 0x50564 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x5056c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5056c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5056c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x503ec | `(unknown callee) 0x53f84` | `[sp-20]` | `?` | `0x0` | `?` |
| `bl` at 0x5044c | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x504bc | `(unknown callee) 0x53f84` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x50514 | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x5053c | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x50558 | `(unknown callee) 0x53f84` | `[sp-20]` | `ret(0x53100)` | `0x0` | `ret(0x53100)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x50350 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x5035c | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x503ac | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x503c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x50400 | `ret(0x53f84) == 0x0` | `ret(0x53f84) != 0x0` |
| `bne` at 0x5040c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x5043c | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x50460 | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |
| `ble` at 0x50498 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x504d0 | `ret(0x53f84) == 0x0` | `ret(0x53f84) != 0x0` |
| `bne` at 0x504dc | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x50528 | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5056c | `r0` | `?` |

## `pwrStatusLEDInit`

- `pwrStatusLEDInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55640 to 0x556d0.
  Established. Included word stores write an unknown value to `0x13254c` and `[sp-24]` to `0x132550` (`str` at 0x556a4, `str` at 0x556b0).
  A zero `[0x13254c]` takes `0x5569c`, while nonzero falls through (`beq` at 0x55664).
  LED initialization callback meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55650 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x55654 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x55658 | load | 4 | `abs 0x556c0` | `-` |
| `ldr` at 0x5565c | load | 4 | `abs 0x13254c` | `-` |
| `ldr` at 0x55668 | load | 4 | `abs 0x556c4` | `-` |
| `str` at 0x5566c | store | 4 | `sp -0x24` | `0xf90a8` |
| `ldr` at 0x55670 | load | 4 | `abs 0x556c0` | `-` |
| `ldr` at 0x55674 | load | 4 | `abs 0x13254c` | `-` |
| `str` at 0x55678 | store | 4 | `sp -0x20` | `[0x13254c]` |
| `ldr` at 0x5567c | load | 4 | `abs 0x556c8` | `-` |
| `ldr` at 0x55680 | load | 4 | `abs 0x132550` | `-` |
| `str` at 0x55684 | store | 4 | `sp -0x1c` | `[0x132550]` |
| `ldr` at 0x5568c | load | 4 | `abs 0x556cc` | `-` |
| `ldr` at 0x5569c | load | 4 | `abs 0x556c0` | `-` |
| `ldr` at 0x556a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x556a4 | store | 4 | `abs 0x13254c` | `?` |
| `ldr` at 0x556a8 | load | 4 | `abs 0x556c8` | `-` |
| `ldr` at 0x556ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x556b0 | store | 4 | `abs 0x132550` | `[sp-24]` |
| `ldm` at 0x556bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x556bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x556bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55698 | `(unknown callee) 0x200a0` | `0x9` | `0xf90d0` | `0x2c` | `0x0` |
| `bl` at 0x556b4 | `(unknown callee) 0x556d0` | `?` | `?` | `0x132550` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55664 | `[0x13254c] == 0x0` | `[0x13254c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x556bc | `r0` | `ret(0x556d0)` |

## `pwrStatusLEDUpdate`

- `pwrStatusLEDUpdate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x556d0 to 0x5590c.
  Established. Equality `ret(0x37960) == 0x2` takes `0x55788`, while inequality falls through (`beq` at 0x55768).
  Included unnamed `0xca4e8` and `0xca538` calls carry `0x0` followed by three unknown candidates (`bl` at 0x557fc, `bl` at 0x5583c).
  LED polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x556e4 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x556ec | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x556f0 | load | 4 | `abs 0x55900` | `-` |
| `ldr` at 0x556f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x556f8 | load | 4 | `abs 0x132550` | `-` |
| `str` at 0x55708 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5570c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55714 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55718 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x55720 | load | 1 | `(unknown)` | `-` |
| `str` at 0x55724 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x55728 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55730 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55734 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x5573c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x55740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55750 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5575c | store | 4 | `sp -0x20` | `ret(0x37960)` |
| `ldr` at 0x55760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5576c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55770 | load | 4 | `abs 0x55908` | `-` |
| `ldr` at 0x5577c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55784 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x55788 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55790 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x55798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x557a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x557ac | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x557b0 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x557b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x557c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x557cc | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x557d0 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x557d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x557dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x557e4 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x557e8 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x557f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x55804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5580c | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55810 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x55818 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5581c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55824 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55828 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x55830 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x55840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5584c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55854 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55858 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x55860 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5586c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55874 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55878 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x55880 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x55884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5588c | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x55890 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x55898 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x558ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x558b4 | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x558b8 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x558c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x558c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x558cc | load | 4 | `abs 0x55904` | `-` |
| `ldr` at 0x558d0 | load | 4 | `abs 0x13254c` | `-` |
| `ldrb` at 0x558d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x558e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x558f0 | store | 4 | `sp -0x14` | `?` |
| `ldm` at 0x558fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x558fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x558fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55754 | `(unknown callee) 0x37960` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x557fc | `(unknown callee) 0xca4e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5583c | `(unknown callee) 0xca538` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x558a4 | `(unknown callee) 0xca538` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x558e4 | `(unknown callee) 0xca4e8` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x55700 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blt` at 0x55748 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x55768 | `ret(0x37960) == 0x2` | `ret(0x37960) != 0x2` |
| `beq` at 0x55778 | `ret(0x37960) == 0xfffff9b9` | `ret(0x37960) != 0xfffff9b9` |
| `bne` at 0x557a0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x557c0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x55848 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x55868 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x558fc | `r0` | `?` |

## `nl4r_init_ports`

- `nl4r_init_ports`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a22c to 0x2a6d0.
  Established. The unnamed `0xe0540` call carries `[sp-20]`, `0x2`, `[sp-24]+10`, `0x0` (`bl` at 0x2a454).
  A signed nonnegative `ret(0xe0540)` takes `0x2a478`, while negative falls through (`bge` at 0x2a468).
  NL4R link activation sequence beyond these calls remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a23c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2a240 | load | 4 | `abs 0x2a6ac` | `-` |
| `str` at 0x2a244 | store | 4 | `sp -0x28` | `0xf13d8` |
| `ldr` at 0x2a248 | load | 4 | `abs 0x2a6b0` | `-` |
| `ldr` at 0x2a24c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a258 | load | 4 | `abs 0x2a6b4` | `-` |
| `ldr` at 0x2a25c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a268 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2a270 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x2a278 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2a27c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a288 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a28c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a2a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a2ac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x2a2b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2a2b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x2a2bc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x2a2c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2a2c8 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x2a2cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x2a2d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2a2d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a2dc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x2a2e4 | load | 4 | `abs 0x2a6b8` | `-` |
| `ldr` at 0x2a2e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2a2f4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2a2f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a308 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2a30c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a32c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a330 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a348 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2a34c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a36c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a370 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a388 | store | 4 | `sp -0x20` | `0x24` |
| `ldr` at 0x2a38c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a3ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a3b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a3cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a3d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a3ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a3f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a40c | load | 4 | `abs 0x2a6b0` | `-` |
| `ldr` at 0x2a410 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a41c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2a420 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2a428 | load | 1 | `([sp-40] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x2a42c | load | 4 | `abs 0x2a6bc` | `-` |
| `ldr` at 0x2a434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2a43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a444 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a45c | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2a460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2a46c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2a470 | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `ldr` at 0x2a478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a498 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2a4a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a4b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a4b8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a4bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a4c4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x2a4d0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2a4d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a4e0 | load | 4 | `abs 0x2a6b0` | `-` |
| `ldr` at 0x2a4e4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a4f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2a4f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2a4fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2a500 | load | 4 | `abs 0x2a6c0` | `-` |
| `ldr` at 0x2a50c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a514 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a52c | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2a530 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2a53c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2a540 | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `ldr` at 0x2a548 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a550 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x2a55c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2a560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a56c | load | 4 | `abs 0x2a6b0` | `-` |
| `ldr` at 0x2a570 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a57c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2a580 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2a588 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2a58c | load | 4 | `abs 0x2a6c4` | `-` |
| `ldr` at 0x2a598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a5a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a5b8 | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2a5bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2a5c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2a5cc | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `ldr` at 0x2a5d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a5dc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x2a5e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a5e8 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x2a5f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a600 | load | 4 | `abs 0x2a6b0` | `-` |
| `ldr` at 0x2a604 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a610 | load | 4 | `abs 0x2a6c8` | `-` |
| `ldr` at 0x2a618 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a630 | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2a634 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2a640 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2a644 | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `ldr` at 0x2a64c | load | 4 | `abs 0x2a6b0` | `-` |
| `ldr` at 0x2a650 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a65c | load | 4 | `abs 0x2a6cc` | `-` |
| `ldr` at 0x2a664 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a67c | store | 4 | `sp -0x24` | `ret(0xe0540)` |
| `ldr` at 0x2a680 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2a68c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2a690 | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `str` at 0x2a69c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x2a6a0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x2a6a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2a6a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2a6a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2a260 | `(unknown callee) 0x221f0` | `0xf13e4` | `r0` | `r2` | `[0x29737c]` |
| `bl` at 0x2a290 | `(unknown callee) 0x2adc4` | `?` | `?` | `?` | `?` |
| `bl` at 0x2a2ec | `(unknown callee) 0x1e958` | `0xf1408` | `?` | `?` | `?` |
| `bl` at 0x2a328 | `(unknown callee) 0xdfc34` | `[sp-20]` | `?` | `0x1` | `sp-32` |
| `bl` at 0x2a334 | `(unknown callee) 0x2adc4` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x2a368 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x2a374 | `(unknown callee) 0x2ae94` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x2a3a8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x2a3c8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x2a3e8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x2a408 | `(unknown callee) 0xdfc34` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x2a438 | `(unknown callee) 0x221f0` | `0xf1414` | `[([sp-40] + [sp-24])]:1` | `[sp-32]` | `[([sp-40] + [sp-24])]:1` |
| `bl` at 0x2a454 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `[sp-24]+10` | `0x0` |
| `bl` at 0x2a494 | `(unknown callee) 0xdf6d8` | `[sp-20]` | `[sp-24]+10` | `0x0` | `sp-32` |
| `bl` at 0x2a508 | `(unknown callee) 0x221f0` | `0xf1440` | `?` | `[sp-40]` | `?` |
| `bl` at 0x2a524 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x2a594 | `(unknown callee) 0x221f0` | `0xf1464` | `?` | `[sp-40]` | `?` |
| `bl` at 0x2a5b0 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x2a5fc | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x5` | `0x0` | `sp-32` |
| `bl` at 0x2a614 | `(unknown callee) 0x221f0` | `0xf1488` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x2a628 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `0x5` | `0x0` |
| `bl` at 0x2a660 | `(unknown callee) 0x221f0` | `0xf14a8` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x2a674 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `0x9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2a254 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bhi` at 0x2a284 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x2a29c | `ret(0x2adc4) == 0x0` | `ret(0x2adc4) != 0x0` |
| `bhi` at 0x2a300 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x2a380 | `ret(0x2ae94) == 0x0` | `ret(0x2ae94) != 0x0` |
| `bls` at 0x2a418 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a468 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bne` at 0x2a4a0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bhi` at 0x2a4dc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0x2a4ec | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a538 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bhi` at 0x2a568 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0x2a578 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a5c4 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bls` at 0x2a60c | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a63c | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bls` at 0x2a658 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a688 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2a6a8 | `r0` | `?` |

## `L1FRUNL4RInit`

- `L1FRUNL4RInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c97c to 0x9cb7c.
  Established. A nonzero `[0x29a95c]` takes `0x9c9b0`, while zero falls through (`bne` at 0x9c9a8).
  Included word stores write `0x2` to an unknown effective target and `[0x29a958]+1` to `0x29a958` (`str` at 0x9c9ec, `str` at 0x9cb40).
  NL4R capture-record ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c98c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x9c990 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9c994 | load | 4 | `abs 0x29a958` | `-` |
| `str` at 0x9c998 | store | 4 | `sp -0x24` | `[0x29a958]` |
| `ldr` at 0x9c99c | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9c9a0 | load | 4 | `abs 0x29a95c` | `-` |
| `str` at 0x9c9b4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9c9b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c9bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c9c8 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9c9cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c9e0 | load | 4 | `abs 0x29a95c` | `-` |
| `str` at 0x9c9ec | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x9c9f0 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9c9f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9ca08 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9ca10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ca14 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x9ca18 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9ca1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9ca30 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9ca38 | load | 4 | `abs 0x9cb6c` | `-` |
| `ldr` at 0x9ca3c | load | 4 | `abs 0x12d8d4` | `-` |
| `str` at 0x9ca40 | store | 4 | `(unknown)` | `[0x12d8d4]` |
| `ldr` at 0x9ca44 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9ca48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9ca5c | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9ca64 | load | 4 | `abs 0x9cb70` | `-` |
| `str` at 0x9ca68 | store | 4 | `(unknown)` | `0x11cb74` |
| `ldr` at 0x9ca6c | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9ca70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9ca84 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9ca8c | load | 4 | `abs 0x9cb74` | `-` |
| `ldr` at 0x9ca90 | load | 4 | `abs 0x12d8d8` | `-` |
| `str` at 0x9ca94 | store | 4 | `(unknown)` | `[0x12d8d8]` |
| `ldr` at 0x9ca98 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9ca9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9cab0 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9cab8 | load | 4 | `abs 0x9cb78` | `-` |
| `str` at 0x9cabc | store | 4 | `(unknown)` | `0x11d71c` |
| `ldr` at 0x9cac0 | load | 4 | `abs 0x9cb6c` | `-` |
| `ldr` at 0x9cac4 | load | 4 | `abs 0x12d8d4` | `-` |
| `str` at 0x9cacc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9cad0 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9cad4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9cae8 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9caf0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9cafc | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x9cb00 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9cb04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9cb18 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9cb20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cb28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9cb30 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9cb34 | load | 4 | `abs 0x9cb68` | `-` |
| `ldr` at 0x9cb38 | load | 4 | `abs 0x29a958` | `-` |
| `str` at 0x9cb40 | store | 4 | `abs 0x29a958` | `[0x29a958]+1` |
| `ldr` at 0x9cb44 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9cb4c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x9cb50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9cb58 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0x9cb64 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9cb64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9cb64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9cb64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9caf4 | `(unknown callee) 0x1943c` | `[sp-28]` | `0x29a944` | `?` | `[0x29a95c]` |
| `bl` at 0x9cb2c | `(unknown callee) 0x19748` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9c9a8 | `[0x29a95c] != 0x0` | `[0x29a95c] == 0x0` |
| `bge` at 0x9c9c4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9cb64 | `r0` | `?` |

## `L1FRUNL4RCapture`

- `L1FRUNL4RCapture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c704 to 0x9c97c.
  Established. Included word stores write `0x1` and `0x4` to `0x29a94c` (`str` at 0x9c730, `str` at 0x9c930).
  The unnamed `0xde16c` call carries `ret(0x291a4)`, `[sp-40]`, `sp-48`, `sp-52` (`bl` at 0x9c808).
  A nonzero `[0x29a94c]` takes `0x9c968`, while zero falls through (`bne` at 0x9c770).
  Capture status codes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c714 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9c718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c71c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x9c728 | load | 4 | `abs 0x9c974` | `-` |
| `str` at 0x9c730 | store | 4 | `abs 0x29a94c` | `0x1` |
| `ldr` at 0x9c73c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c740 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c744 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9c74c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x9c760 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x9c764 | load | 4 | `abs 0x9c974` | `-` |
| `ldr` at 0x9c768 | load | 4 | `abs 0x29a94c` | `-` |
| `ldr` at 0x9c774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c778 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9c77c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9c78c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x9c790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c794 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c7a8 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0x9c7b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c7b4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x9c7b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c7d0 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0x9c7d8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9c7e0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9c7e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c7e8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x9c800 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c81c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9c820 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c82c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c838 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c840 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9c848 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9c84c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c850 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c858 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c860 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9c868 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9c86c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c870 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c878 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c880 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9c888 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9c88c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c898 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c8a0 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x9c8a4 | store | 1 | `(unknown)` | `([sp-48] & 0xff)` |
| `ldr` at 0x9c8a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c8ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c8b4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c8bc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9c8c4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9c8c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c8cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c8d4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c8dc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9c8e4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9c8e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c8ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c8f4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c8fc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9c904 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9c908 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c90c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c914 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9c91c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x9c920 | store | 1 | `(unknown)` | `([sp-52] & 0xff)` |
| `ldr` at 0x9c928 | load | 4 | `abs 0x9c974` | `-` |
| `str` at 0x9c930 | store | 4 | `abs 0x29a94c` | `0x4` |
| `ldr` at 0x9c934 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9c938 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x9c93c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9c940 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x9c944 | load | 4 | `abs 0x9c978` | `-` |
| `ldr` at 0x9c948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c94c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c950 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c958 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9c960 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x9c970 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9c970 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9c970 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c738 | `(unknown callee) 0xd708c` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0x9c758 | `(unknown callee) 0x19748` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0x9c7ec | `(unknown callee) 0x291a4` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0x9c808 | `(unknown callee) 0xde16c` | `ret(0x291a4)` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0x9c954 | `(unknown callee) 0x1e958` | `0x112c34` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0x9c968 | `(unknown callee) 0xd70fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9c724 | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0x9c770 | `[0x29a94c] != 0x0` | `[0x29a94c] == 0x0` |
| `ble` at 0x9c784 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9c828 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9c970 | `r0` | `?` |

## `sn1sc_select_port`

- `sn1sc_select_port`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd702c to 0xd708c.
  Established. A halfword store writes an unknown value to `[0x29c4e8] +0xe` (`strh` at 0xd7078).
  Byte assignments preserve `(r0 & 0xff)` and `(r1 & 0xff)` at `sp -0x11` and `sp -0x12` (`strb` at 0xd7044, `strb` at 0xd704c).
  The integer return candidate is `0x1` (`ldm` at 0xd7084).
  Scan port selector encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xd7044 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xd704c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xd7054 | load | 4 | `abs 0xd7088` | `-` |
| `ldr` at 0xd7058 | load | 4 | `abs 0x29c4e8` | `-` |
| `ldrb` at 0xd7060 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xd7068 | load | 1 | `sp -0x11` | `-` |
| `strh` at 0xd7078 | store | 2 | `[0x29c4e8] +0xe` | `?` |
| `ldm` at 0xd7084 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd7084 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd7084 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd7084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd7084 | `r0` | `0x1` |

## `cpuEnabled`

- `cpuEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x542cc to 0x5435c.
  Established. The signed comparison `(r0 & 0xff) >= [0x297608]` takes `0x54344`, while less falls through (`bge` at 0x542f4).
  The unnamed `0x1d444` call carries `[sp-17]:1+88`, unknown, unknown, `[sp-17]:1+88` (`bl` at 0x5431c).
  CPU enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x542e0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x542e4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x542e8 | load | 4 | `abs 0x54358` | `-` |
| `ldr` at 0x542ec | load | 4 | `abs 0x297608` | `-` |
| `ldrb` at 0x542f8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x54310 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x54320 | store | 4 | `sp -0x1c` | `ret(0x1d444)` |
| `ldr` at 0x54324 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54334 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x54338 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5433c | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x54348 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5434c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x54354 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54354 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54354 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54300 | `(unknown callee) 0x54244` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x5431c | `(unknown callee) 0x1d444` | `[sp-17]:1+88` | `?` | `?` | `[sp-17]:1+88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x542f4 | `(r0 & 0xff) >= [0x297608] (signed, 32-bit)` | `(r0 & 0xff) < [0x297608] (signed, 32-bit)` |
| `beq` at 0x5430c | `ret(0x54244) == 0x0` | `ret(0x54244) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54354 | `r0` | `?` |

## `nodeEnabled`

- `nodeEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5496c to 0x54a0c.
  Established. An included unnamed `0x1d444` call carries `0x70`, unknown, unknown, `ret(0x548e4)` (`bl` at 0x549b4).
  The signed comparison `(r0 & 0xff) >= [0x297634]` takes `0x549f4`, while less falls through (`bge` at 0x54994).
  Node-enable bit interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54980 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x54984 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x54988 | load | 4 | `abs 0x54a08` | `-` |
| `ldr` at 0x5498c | load | 4 | `abs 0x297634` | `-` |
| `ldrb` at 0x54998 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x549b8 | store | 4 | `sp -0x1c` | `ret(0x1d444)` |
| `ldrb` at 0x549bc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x549c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x549d0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x549d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x549e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x549e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x549ec | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x549f8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x549fc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x54a04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54a04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54a04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x549a0 | `(unknown callee) 0x548e4` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x549b4 | `(unknown callee) 0x1d444` | `0x70` | `?` | `?` | `ret(0x548e4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x54994 | `(r0 & 0xff) >= [0x297634] (signed, 32-bit)` | `(r0 & 0xff) < [0x297634] (signed, 32-bit)` |
| `beq` at 0x549ac | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54a04 | `r0` | `?` |

## `EnvAuto`

- `EnvAuto`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76c10 to 0x76c7c.
  Established. A zero entry `r0` takes `0x76c68`, while nonzero falls through (`beq` at 0x76c2c).
  An included unnamed `0x1b5f0` call carries `0x6` and three unknown candidates (`bl` at 0x76c70).
  The integer return candidate is `ret(0x76ca4)` (`ldm` at 0x76c78).
  Automatic environment mode semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76c20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76c4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76c6c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76c78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76c78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76c78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76c78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76c48 | `(unknown callee) 0x1b5f0` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x76c64 | `(unknown callee) 0x1b5f0` | `0x7d` | `?` | `?` | `?` |
| `bl` at 0x76c70 | `(unknown callee) 0x1b5f0` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x76c74 | `(unknown callee) 0x76ca4` | `ret(0x1b5f0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x76c2c | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76c78 | `r0` | `ret(0x76ca4)` |

## `EnvGetStatus`

- `EnvGetStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76fb4 to 0x76ff0.
  Established. A word store copies `[0x12d79c]` to entry `r0 +0x0` (`str` at 0x76fd4).
  A word read addresses `0x133220` (`ldr` at 0x76fdc).
  The integer return candidate is `[0x133220]` (`ldm` at 0x76fe4).
  Environment status bit definitions remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76fc4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76fc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76fcc | load | 4 | `abs 0x76fe8` | `-` |
| `ldr` at 0x76fd0 | load | 4 | `abs 0x12d79c` | `-` |
| `str` at 0x76fd4 | store | 4 | `r0 +0x0` | `[0x12d79c]` |
| `ldr` at 0x76fd8 | load | 4 | `abs 0x76fec` | `-` |
| `ldr` at 0x76fdc | load | 4 | `abs 0x133220` | `-` |
| `ldm` at 0x76fe4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76fe4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76fe4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76fe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76fe4 | `r0` | `[0x133220]` |

## `EnvTestEvent`

- `EnvTestEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76d68 to 0x76e90.
  Established. The unsigned condition entry `r0 <= 0x4` takes an unresolved target, while greater falls through (`ldrls` at 0x76d84).
  A word read at that transfer has an unknown effective target (`ldrls` at 0x76d84).
  The integer return candidate on known edges retains entry `r0` (`ldm` at 0x76e70).
  Event dispatch effects beyond known edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76d78 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x76d7c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x76d84 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x76e70 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76e70 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76e70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x76d84 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76e70 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x76d84).
