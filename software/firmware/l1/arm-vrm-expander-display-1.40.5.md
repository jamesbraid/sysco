# ARM regulator, expander and display paths in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
The selected spans have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/vrm-expander-display-facts.txt) retain all tool sections.

## `vrmInit`

- `vrmInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cb50 to 0x5cd6c.
  Established. The absolute word at `0x149110` receives `0x1` (`str` at 0x5cbd4). Another absolute
  word at `0x14910c` receives `ret(0x1b454)` (`str` at 0x5cc08). A byte assignment at `[sp-36] +0x8`
  retains `([sp-32]:1 & 0xff)` (`strb` at 0x5cc74). The final unnamed `0x1b4cc` call carries
  `[[sp-36]+4]`, `0x1`, `[[sp-36]+8]:1`, `[sp-36]` (`bl` at 0x5cd44). Initialization effects on
  hardware remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cb60 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5cb64 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5cb68 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x5cb6c | load | 4 | `abs 0x5cd4c` | `-` |
| `ldr` at 0x5cb70 | load | 4 | `abs 0x14910c` | `-` |
| `ldr` at 0x5cb7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5cb88 | load | 4 | `abs 0x5cd50` | `-` |
| `str` at 0x5cb8c | store | 4 | `sp -0x34` | `0x10ca04` |
| `ldr` at 0x5cb90 | load | 4 | `abs 0x5cd4c` | `-` |
| `ldr` at 0x5cb94 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5cb98 | store | 4 | `sp -0x30` | `[0x14910c]` |
| `ldr` at 0x5cb9c | load | 4 | `abs 0x5cd54` | `-` |
| `ldr` at 0x5cba0 | load | 4 | `abs 0x149110` | `-` |
| `str` at 0x5cba4 | store | 4 | `sp -0x2c` | `[0x149110]` |
| `ldr` at 0x5cbac | load | 4 | `abs 0x5cd58` | `-` |
| `ldr` at 0x5cbbc | load | 4 | `abs 0x5cd54` | `-` |
| `ldr` at 0x5cbc0 | load | 4 | `abs 0x149110` | `-` |
| `ldr` at 0x5cbcc | load | 4 | `abs 0x5cd54` | `-` |
| `str` at 0x5cbd4 | store | 4 | `abs 0x149110` | `0x1` |
| `ldr` at 0x5cbd8 | load | 4 | `abs 0x5cd54` | `-` |
| `ldr` at 0x5cbdc | load | 4 | `abs 0x149110` | `-` |
| `str` at 0x5cbf0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5cbf4 | load | 4 | `abs 0x5cd4c` | `-` |
| `ldr` at 0x5cbf8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5cc08 | store | 4 | `abs 0x14910c` | `ret(0x1b454)` |
| `ldr` at 0x5cc0c | load | 4 | `abs 0x14910c` | `-` |
| `ldr` at 0x5cc18 | load | 4 | `abs 0x5cd5c` | `-` |
| `str` at 0x5cc1c | store | 4 | `sp -0x34` | `0x10ca48` |
| `ldr` at 0x5cc20 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5cc24 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x5cc2c | load | 4 | `abs 0x5cd58` | `-` |
| `ldr` at 0x5cc3c | load | 4 | `abs 0x5cd4c` | `-` |
| `ldr` at 0x5cc40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5cc54 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5cc5c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5cc60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cc64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5cc68 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x5cc6c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cc70 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x5cc74 | store | 1 | `[sp-36] +0x8` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0x5cc78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cc7c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cc80 | load | 1 | `[sp-36] +0x8` | `-` |
| `str` at 0x5cc94 | store | 4 | `[sp-36] +0x4` | `ret(0x1b454)` |
| `ldr` at 0x5cc98 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x5cca4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ccac | load | 4 | `abs 0x5cd60` | `-` |
| `ldr` at 0x5ccb0 | load | 4 | `abs 0x5cd64` | `-` |
| `str` at 0x5ccdc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5cce0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5cce8 | load | 4 | `abs 0x5cd60` | `-` |
| `ldr` at 0x5ccec | load | 4 | `abs 0x5cd64` | `-` |
| `ldr` at 0x5ccf8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5cd08 | load | 4 | `abs 0x5cd68` | `-` |
| `str` at 0x5cd0c | store | 4 | `(unknown)` | `0x10ca8c` |
| `str` at 0x5cd10 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x5cd14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5cd18 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x5cd20 | load | 4 | `abs 0x5cd58` | `-` |
| `ldr` at 0x5cd30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cd34 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cd38 | load | 1 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x5cd3c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldmdb` at 0x5cd48 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5cd48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cd48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cd48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cbb8 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ca34` | `0x36` | `0x0` |
| `bl` at 0x5cc00 | `(unknown callee) 0x1b454` | `[sp-40]` | `0x1` | `[0x149110]` | `?` |
| `bl` at 0x5cc38 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ca34` | `0x42` | `0x0` |
| `bl` at 0x5cc8c | `(unknown callee) 0x1b454` | `[[sp-36]+8]:1` | `0x1` | `[sp-36]` | `[[sp-36]+8]:1` |
| `bl` at 0x5ccb4 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5ccc0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5ccf0 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5cd00 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5cd2c | `(unknown callee) 0x21eb8` | `0x9` | `0x10ca34` | `0x4d` | `0x0` |
| `bl` at 0x5cd44 | `(unknown callee) 0x1b4cc` | `[[sp-36]+4]` | `0x1` | `[[sp-36]+8]:1` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5cb78 | `[0x14910c] == 0x0` | `[0x14910c] != 0x0` |
| `bne` at 0x5cb84 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x5cbc8 | `[0x149110] != 0x0` | `[0x149110] == 0x0` |
| `bne` at 0x5cc14 | `[0x14910c] != 0x0` | `[0x14910c] == 0x0` |
| `bne` at 0x5cca0 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cd48 | `r0` | `ret(0x1b4cc)` |

## `vrmDisable`

- `vrmDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cd6c to 0x5ce80.
  Established. The unsigned branch takes `(r1 & 0xff) >= [[sp-28]+8]:1` to `0x5cddc`, and falls
  through below the bound (`bhs` at 0x5cdbc). The fall-through byte assignment clears `((r1 & 0xff) + [[sp-28]+4]) +0x0` (`strb` at 0x5cdd4). The unnamed `0x21eb8` call carries `0x6`, `0x10ca34`,
  `0x5c`, `0x0` (`bl` at 0x5ce64). Disable effects on hardware remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cd7c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5cd84 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5cd88 | load | 4 | `abs 0x5ce6c` | `-` |
| `ldr` at 0x5cd8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cda0 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5cda8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5cdac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cdb0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5cdb4 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5cdc0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cdc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5cdc8 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x5cdd4 | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x0` |
| `ldr` at 0x5cddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cde4 | load | 4 | `abs 0x5ce70` | `-` |
| `ldr` at 0x5cde8 | load | 4 | `abs 0x5ce74` | `-` |
| `str` at 0x5ce14 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5ce18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ce20 | load | 4 | `abs 0x5ce70` | `-` |
| `ldr` at 0x5ce24 | load | 4 | `abs 0x5ce74` | `-` |
| `ldr` at 0x5ce30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ce40 | load | 4 | `abs 0x5ce78` | `-` |
| `str` at 0x5ce44 | store | 4 | `(unknown)` | `0x10cac8` |
| `str` at 0x5ce48 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x5ce4c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5ce50 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x5ce58 | load | 4 | `abs 0x5ce7c` | `-` |
| `ldmdb` at 0x5ce68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ce68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ce68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cdec | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5cdf8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5ce28 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5ce38 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5ce64 | `(unknown callee) 0x21eb8` | `0x6` | `0x10ca34` | `0x5c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5cdbc | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ce68 | `r0` | `?` |

## `vrmEnable`

- `vrmEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ce80 to 0x5cf94.
  Established. The byte assignment at `((r1 & 0xff) + [[sp-28]+4]) +0x0` writes `0x1` (`strb` at
  0x5cee8). The unsigned branch reaches that fall-through path below `[[sp-28]+8]:1`, and takes
  `0x5cef0` at or above that bound (`bhs` at 0x5ced0). The unnamed `0x4f9a4` call carries `r0`,
  `0x0`, `0x10ca80`, `0x10ca88` (`bl` at 0x5cf00). Hardware enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ce90 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5ce98 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5ce9c | load | 4 | `abs 0x5cf80` | `-` |
| `ldr` at 0x5cea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ceb4 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5cebc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5cec0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cec4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5cec8 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5ced4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5ced8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5cedc | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x5cee8 | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x1` |
| `ldr` at 0x5cef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cef8 | load | 4 | `abs 0x5cf84` | `-` |
| `ldr` at 0x5cefc | load | 4 | `abs 0x5cf88` | `-` |
| `str` at 0x5cf28 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5cf2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cf34 | load | 4 | `abs 0x5cf84` | `-` |
| `ldr` at 0x5cf38 | load | 4 | `abs 0x5cf88` | `-` |
| `ldr` at 0x5cf44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cf54 | load | 4 | `abs 0x5cf8c` | `-` |
| `str` at 0x5cf58 | store | 4 | `(unknown)` | `0x10cae4` |
| `str` at 0x5cf5c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x5cf60 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5cf64 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x5cf6c | load | 4 | `abs 0x5cf90` | `-` |
| `ldmdb` at 0x5cf7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cf7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cf7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cf00 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5cf0c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5cf3c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5cf4c | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5cf78 | `(unknown callee) 0x21eb8` | `0x6` | `0x10ca34` | `0x6a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5ced0 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cf7c | `r0` | `?` |

## `vrmGetCfg`

- `vrmGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cfd4 to 0x5d034.
  Established. A four-byte read uses absolute `0x14910c` (`ldr` at 0x5d004). The word at `[sp-28] +0x0` is read (`ldr` at 0x5d018), and `[[sp-28]]` is written at `r1 +0x0` (`str` at 0x5d01c). The
  return reports `[[sp-28]+8]:1` (`ldmdb` at 0x5d02c). Configuration field meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cfe4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5cfe8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5cfec | load | 4 | `abs 0x5d030` | `-` |
| `ldr` at 0x5cff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d004 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d00c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5d010 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d018 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5d01c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x5d020 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d024 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0x5d02c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d02c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d02c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d02c | `r0` | `[[sp-28]+8]:1` |

## `vrmIntEnable`

- `vrmIntEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d034 to 0x5d1e8.
  Established. The byte assignment at `[sp-32] +0x9` writes `0x1` (`strb` at 0x5d074). The unnamed
  `0xd76d4` call has `[sp-28]` and three unknown candidates (`bl` at 0x5d0f8); its zero result takes
  `0x5d1c0`, and nonzero falls through (`beq` at 0x5d10c). Included byte reads in the loop have
  unknown effective addresses (`ldrb` at 0x5d0a8, `ldrb` at 0x5d0d0, `ldrb` at 0x5d0dc, `ldrb` at
  0x5d0e0). Interrupt-enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d044 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5d048 | load | 4 | `abs 0x5d1d4` | `-` |
| `ldr` at 0x5d04c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d060 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d068 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d06c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x5d074 | store | 1 | `[sp-32] +0x9` | `0x1` |
| `str` at 0x5d07c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5d080 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5d084 | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x5d088 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d098 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d09c | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5d0a0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x5d0a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d0b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d0b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d0c0 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5d0c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5d0cc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d0d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d0d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5d0d8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d0dc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5d0e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d0ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d100 | store | 4 | `sp -0x24` | `ret(0xd76d4)` |
| `ldr` at 0x5d104 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d110 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d118 | load | 4 | `abs 0x5d1d8` | `-` |
| `ldr` at 0x5d11c | load | 4 | `abs 0x5d1dc` | `-` |
| `str` at 0x5d148 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x5d14c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d154 | load | 4 | `abs 0x5d1d8` | `-` |
| `ldr` at 0x5d158 | load | 4 | `abs 0x5d1dc` | `-` |
| `ldr` at 0x5d164 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d174 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d178 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x5d17c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d18c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d198 | load | 4 | `abs 0x5d1e0` | `-` |
| `str` at 0x5d19c | store | 4 | `(unknown)` | `0x10cafc` |
| `str` at 0x5d1a0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x5d1a4 | store | 4 | `(unknown)` | `ret(0xd7eec)` |
| `str` at 0x5d1a8 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x5d1b0 | load | 4 | `abs 0x5d1e4` | `-` |
| `ldr` at 0x5d1c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5d1c8 | store | 4 | `sp -0x28` | `?` |
| `ldmdb` at 0x5d1d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5d1d0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5d1d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d1d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d1d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d0f8 | `(unknown callee) 0xd76d4` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5d120 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5d12c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5d15c | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5d16c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5d184 | `(unknown callee) 0xd7eec` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x5d190 | `(unknown callee) 0x21a38` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x5d1bc | `(unknown callee) 0x21eb8` | `0x3` | `0x10ca34` | `0xa3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5d090 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5d0b0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5d10c | `ret(0xd76d4) == 0x0` | `ret(0xd76d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d1d0 | `r0` | `?` |

## `vrmIntDisable`

- `vrmIntDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d1e8 to 0x5d3dc.
  Established. The byte at `[sp-32] +0x9` is cleared (`strb` at 0x5d228). The unnamed `0xd72bc` call
  carries `[sp-28]`, unknown, `[[sp-32]+4]`, unknown (`bl` at 0x5d2ac); its nonzero result takes
  `0x5d2c0`, while zero falls through (`bne` at 0x5d2b8). The later unnamed `0xd768c` call has
  `[sp-28]` followed by three unknown candidates (`bl` at 0x5d2ec). Hardware interrupt-disable
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d1f8 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5d1fc | load | 4 | `abs 0x5d3c8` | `-` |
| `ldr` at 0x5d200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d214 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d21c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d220 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x5d228 | store | 1 | `[sp-32] +0x9` | `0x0` |
| `str` at 0x5d230 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5d234 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5d238 | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x5d23c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d24c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d258 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5d260 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5d264 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d268 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5d26c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x5d274 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d284 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5d288 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x5d290 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d29c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d2a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d2a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d2c0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d2c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d2c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5d2cc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d2d0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5d2d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d2e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d2f4 | store | 4 | `sp -0x24` | `ret(0xd768c)` |
| `ldr` at 0x5d2f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d30c | load | 4 | `abs 0x5d3cc` | `-` |
| `ldr` at 0x5d310 | load | 4 | `abs 0x5d3d0` | `-` |
| `str` at 0x5d33c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x5d340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d348 | load | 4 | `abs 0x5d3cc` | `-` |
| `ldr` at 0x5d34c | load | 4 | `abs 0x5d3d0` | `-` |
| `ldr` at 0x5d358 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d368 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5d36c | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x5d370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d380 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d38c | load | 4 | `abs 0x5d3d4` | `-` |
| `str` at 0x5d390 | store | 4 | `(unknown)` | `0x10cb34` |
| `str` at 0x5d394 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x5d398 | store | 4 | `(unknown)` | `ret(0xd7eec)` |
| `str` at 0x5d39c | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x5d3a4 | load | 4 | `abs 0x5d3d8` | `-` |
| `ldr` at 0x5d3b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5d3bc | store | 4 | `sp -0x28` | `?` |
| `ldmdb` at 0x5d3c4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5d3c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5d3c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d3c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d3c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d2ac | `(unknown callee) 0xd72bc` | `[sp-28]` | `?` | `[[sp-32]+4]` | `?` |
| `bl` at 0x5d2ec | `(unknown callee) 0xd768c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5d314 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5d320 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5d350 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5d360 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5d378 | `(unknown callee) 0xd7eec` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x5d384 | `(unknown callee) 0x21a38` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x5d3b0 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ca34` | `0xbc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5d244 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5d27c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5d298 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5d2b8 | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |
| `beq` at 0x5d300 | `ret(0xd768c) == 0x0` | `ret(0xd768c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d3c4 | `r0` | `?` |

## `vrmEnabled`

- `vrmEnabled`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d3dc to 0x5d428.
  Established. The byte at `[sp-24] +0x9` is read (`ldrb` at 0x5d418). The return reports
  `[[sp-24]+9]:1` (`ldmdb` at 0x5d420). Enabled-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d3ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5d3f0 | load | 4 | `abs 0x5d424` | `-` |
| `ldr` at 0x5d3f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d408 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d410 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5d414 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5d418 | load | 1 | `[sp-24] +0x9` | `-` |
| `ldmdb` at 0x5d420 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d420 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d420 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d420 | `r0` | `[[sp-24]+9]:1` |

## `vrmPresent`

- `vrmPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d530 to 0x5d634.
  Established. The byte at `((r1 & 0xff) + [[sp-28]+4]) +0x0` is read (`ldrb` at 0x5d594). The
  subsequent zero test takes `0x5d620`, and nonzero falls through (`beq` at 0x5d59c). The unnamed
  `0xd7530` call carries `r0`, `[[sp-36]]:1`, `sp-29`, `[[sp-36]]:1` (`bl` at 0x5d5e8). Presence
  encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d540 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5d548 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5d54c | load | 4 | `abs 0x5d630` | `-` |
| `ldr` at 0x5d550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d564 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d56c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5d570 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d574 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5d578 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5d584 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d588 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5d58c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x5d594 | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldr` at 0x5d5a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d5a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5d5ac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5d5b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5d5b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d5bc | load | 1 | `[sp-36] +0x3` | `-` |
| `str` at 0x5d5cc | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x5d5d4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d5d8 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x5d5e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d5fc | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x5d604 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5d608 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d618 | store | 4 | `sp -0x28` | `ret(0x5d4ac)` |
| `str` at 0x5d624 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5d628 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5d62c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d62c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d62c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d5e8 | `(unknown callee) 0xd7530` | `r0` | `[[sp-36]]:1` | `sp-29` | `[[sp-36]]:1` |
| `bl` at 0x5d610 | `(unknown callee) 0x5d4ac` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5d580 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |
| `beq` at 0x5d59c | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x5d5c4 | `[[sp-36]+3]:1 != 0x0` | `[[sp-36]+3]:1 == 0x0` |
| `beq` at 0x5d5f4 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d62c | `r0` | `[sp-40]` |

## `vrmOK`

- `vrmOK`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d634 to 0x5d810.
  Established. The unnamed `0x5d530` call carries `r0` and three `(r1 & 0xff)` candidates (`bl` at
  0x5d6b0); its nonzero result takes `0x5d6cc`, while zero falls through (`bne` at 0x5d6bc). The
  unnamed `0xfc33c` call carries `ret(0x40754)`, `0x42400000`, unknown, `ret(0x40754)` (`bl` at
  0x5d734). The state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d644 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5d64c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5d650 | load | 4 | `abs 0x5d808` | `-` |
| `ldr` at 0x5d654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d668 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d670 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5d674 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d678 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5d67c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5d688 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d68c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5d690 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x5d698 | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldrb` at 0x5d6a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5d6a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d6c4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5d6cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d6d0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5d6d8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5d6e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5d6e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d6e8 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x5d6f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d70c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5d714 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d718 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x5d71c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d730 | load | 4 | `abs 0x5d80c` | `-` |
| `ldrb` at 0x5d748 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5d750 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0x5d754 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d758 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x5d768 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d76c | load | 1 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x5d778 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5d77c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d798 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d79c | load | 1 | `[sp-36] +0x3` | `-` |
| `ldrb` at 0x5d7a8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5d7ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d7c8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x5d7d4 | store | 4 | `sp -0x28` | `0x1` |
| `ldrb` at 0x5d7dc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5d7e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d7f0 | store | 4 | `sp -0x28` | `ret(0x5d428)` |
| `str` at 0x5d7fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5d800 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5d804 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d804 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d6b0 | `(unknown callee) 0x5d530` | `r0` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x5d6f8 | `(unknown callee) 0xd7530` | `[sp-20]` | `[[sp-36]]:1` | `sp-29` | `[[sp-36]]:1` |
| `bl` at 0x5d724 | `(unknown callee) 0x40754` | `[sp-20]` | `[[sp-36]+1]:1` | `?` | `[[sp-36]+1]:1` |
| `bl` at 0x5d734 | `(unknown callee) 0xfc33c` | `ret(0x40754)` | `0x42400000` | `?` | `ret(0x40754)` |
| `bl` at 0x5d784 | `(unknown callee) 0x5d428` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x5d7b4 | `(unknown callee) 0x5d4ac` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x5d7e8 | `(unknown callee) 0x5d428` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5d684 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |
| `beq` at 0x5d6a0 | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x5d6bc | `ret(0x5d530) != 0x0` | `ret(0x5d530) == 0x0` |
| `beq` at 0x5d704 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `beq` at 0x5d740 | `ret(0xfc33c) == 0x0` | `ret(0xfc33c) != 0x0` |
| `beq` at 0x5d764 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d774 | `[[sp-36]+4]:1 == 0x0` | `[[sp-36]+4]:1 != 0x0` |
| `bne` at 0x5d790 | `ret(0x5d428) != 0x0` | `ret(0x5d428) == 0x0` |
| `beq` at 0x5d7a4 | `[[sp-36]+3]:1 == 0x0` | `[[sp-36]+3]:1 != 0x0` |
| `bne` at 0x5d7c0 | `ret(0x5d4ac) != 0x0` | `ret(0x5d4ac) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d804 | `r0` | `[sp-40]` |

## `vrmAllPresent`

- `vrmAllPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d810 to 0x5d994.
  Established. The unnamed `0x5d530` call has unknown, `[sp-36]:1`, `[[sp-28]+8]:1`, `[sp-36]:1`
  (`bl` at 0x5d87c); its nonzero result takes `0x5d960`, and zero falls through (`bne` at 0x5d888).
  The later word assignment at `sp -0x20` clears that stack location (`str` at 0x5d95c). Presence
  aggregation semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d820 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5d824 | load | 4 | `abs 0x5d97c` | `-` |
| `ldr` at 0x5d828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d83c | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d844 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5d84c | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x5d854 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5d858 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d85c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5d860 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d870 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x5d874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d88c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d894 | load | 4 | `abs 0x5d980` | `-` |
| `ldr` at 0x5d898 | load | 4 | `abs 0x5d984` | `-` |
| `str` at 0x5d8c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5d8c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d8d0 | load | 4 | `abs 0x5d980` | `-` |
| `ldr` at 0x5d8d4 | load | 4 | `abs 0x5d984` | `-` |
| `ldr` at 0x5d8e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d8f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d8f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d8fc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d904 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d918 | load | 4 | `abs 0x5d988` | `-` |
| `str` at 0x5d91c | store | 4 | `(unknown)` | `0x10cb6c` |
| `str` at 0x5d920 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x5d924 | store | 4 | `(unknown)` | `ret(0x403f4)` |
| `ldr` at 0x5d928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d92c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d934 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d93c | load | 1 | `(unknown)` | `-` |
| `str` at 0x5d940 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x5d948 | load | 4 | `abs 0x5d98c` | `-` |
| `ldr` at 0x5d94c | load | 4 | `abs 0x5d990` | `-` |
| `str` at 0x5d95c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5d960 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d968 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x5d970 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5d978 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5d978 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d978 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d87c | `(unknown callee) 0x5d530` | `?` | `[sp-36]:1` | `[[sp-28]+8]:1` | `[sp-36]:1` |
| `bl` at 0x5d89c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5d8a8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5d8d8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5d8e8 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5d910 | `(unknown callee) 0x403f4` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5d954 | `(unknown callee) 0x21eb8` | `0x1` | `0x10ca34` | `0x12b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5d868 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5d888 | `ret(0x5d530) != 0x0` | `ret(0x5d530) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d978 | `r0` | `?` |

## `vrmAllOK`

- `vrmAllOK`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d994 to 0x5db5c.
  Established. The unnamed `0x5d634` call carries unknown, `[sp-36]:1`, `[[sp-28]+8]:1`, `[sp-36]:1`
  (`bl` at 0x5da00); its nonzero result takes `0x5db28`, and zero falls through (`bne` at 0x5da0c).
  A separate word assignment clears `sp -0x20` (`str` at 0x5db24). Overall status meaning remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d9a4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5d9a8 | load | 4 | `abs 0x5db44` | `-` |
| `ldr` at 0x5d9ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d9c0 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5d9c8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5d9d0 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x5d9d8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5d9dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d9e0 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5d9e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d9f4 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x5d9f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5da10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5da14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5da1c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5da24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5da28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5da3c | load | 4 | `abs 0x5db48` | `-` |
| `ldr` at 0x5da54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5da5c | load | 4 | `abs 0x5db4c` | `-` |
| `ldr` at 0x5da60 | load | 4 | `abs 0x5db50` | `-` |
| `str` at 0x5da8c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5da90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5da98 | load | 4 | `abs 0x5db4c` | `-` |
| `ldr` at 0x5da9c | load | 4 | `abs 0x5db50` | `-` |
| `ldr` at 0x5daa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5dab8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dabc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5dac4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5dacc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5dad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5dae0 | load | 4 | `abs 0x5db54` | `-` |
| `str` at 0x5dae4 | store | 4 | `(unknown)` | `0x10cb94` |
| `str` at 0x5dae8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x5daec | store | 4 | `(unknown)` | `ret(0x403f4)` |
| `ldr` at 0x5daf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5daf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5dafc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5db04 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5db08 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x5db10 | load | 4 | `abs 0x5db58` | `-` |
| `str` at 0x5db24 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5db28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5db30 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x5db38 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5db40 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5db40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5db40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5db40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5da00 | `(unknown callee) 0x5d634` | `?` | `[sp-36]:1` | `[[sp-28]+8]:1` | `[sp-36]:1` |
| `bl` at 0x5da30 | `(unknown callee) 0x40754` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5da40 | `(unknown callee) 0xfc33c` | `ret(0x40754)` | `0x42400000` | `?` | `ret(0x40754)` |
| `bl` at 0x5da64 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5da70 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5daa0 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10ca80` | `0x10ca88` |
| `bl` at 0x5dab0 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5dad8 | `(unknown callee) 0x403f4` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5db1c | `(unknown callee) 0x21eb8` | `0x1` | `0x10ca34` | `0x148` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5d9ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5da0c | `ret(0x5d634) != 0x0` | `ret(0x5d634) == 0x0` |
| `bne` at 0x5da4c | `ret(0xfc33c) != 0x0` | `ret(0xfc33c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5db40 | `r0` | `?` |

## `vrmInt`

- `vrmInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5db5c to 0x5e0a8.
  Established. Included byte stack assignments retain `(r1 & 0xff)` at `sp -0x15` (`strb` at
  0x5db78), `(r2 & 0xff)` at `sp -0x16` (`strb` at 0x5db80), and `(r3 & 0xff)` at `sp -0x17` (`strb`
  at 0x5db88). The unnamed `0x7aad8` call carries `0x0`, `0x3402`, `0x1a3`, `sp-92` (`bl` at
  0x5dd84). A later call to that same numeric target carries `0x0`, `0x3500`, `0x1a4`, `0x10ccdc`
  (`bl` at 0x5de98). Event and hardware meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5db6c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5db78 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x5db80 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x5db88 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x5db8c | load | 4 | `abs 0x5e058` | `-` |
| `ldr` at 0x5db90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dba4 | load | 4 | `abs 0x14910c` | `-` |
| `str` at 0x5dbac | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5dbb4 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x5dbb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5dbbc | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5dbc0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x5dbd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dbd4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x5dbdc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5dbe4 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x5dbe8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dbec | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5dbf0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5dbfc | load | 4 | `abs 0x5e05c` | `-` |
| `ldr` at 0x5dc00 | load | 4 | `abs 0x149b70` | `-` |
| `ldrb` at 0x5dc10 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x5dc28 | store | 1 | `sp -0x62` | `(ret(0xd7530) & 0xff)` |
| `ldrb` at 0x5dc2c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5dc30 | load | 1 | `sp -0x61` | `-` |
| `ldrb` at 0x5dc3c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5dc40 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5dc58 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5dc5c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5dc60 | load | 1 | `sp -0x61` | `-` |
| `ldr` at 0x5dc64 | load | 4 | `abs 0x5e060` | `-` |
| `ldrb` at 0x5dc78 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5dc7c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5dc80 | load | 1 | `sp -0x61` | `-` |
| `ldr` at 0x5dc84 | load | 4 | `abs 0x5e064` | `-` |
| `ldr` at 0x5dc94 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dc98 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5dc9c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5dcb0 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5dcb4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x5dccc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dcd0 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x5dce0 | load | 4 | `abs 0x5e068` | `-` |
| `ldr` at 0x5dce4 | load | 4 | `abs 0x149110` | `-` |
| `ldr` at 0x5dcf0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dcf4 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5dcf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dd0c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dd10 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x5dd14 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x5dd1c | load | 4 | `abs 0x5e06c` | `-` |
| `ldr` at 0x5dd20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dd30 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dd34 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5dd38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dd4c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dd50 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5dd58 | load | 4 | `abs 0x5e070` | `-` |
| `str` at 0x5dd6c | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5dd74 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5dd7c | load | 4 | `abs 0x5e074` | `-` |
| `ldr` at 0x5dd80 | load | 4 | `abs 0x5e078` | `-` |
| `ldr` at 0x5dd8c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dd90 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5dd94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dda8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5ddac | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5ddb4 | load | 4 | `abs 0x5e07c` | `-` |
| `str` at 0x5ddc8 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5ddd0 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5ddd8 | load | 4 | `abs 0x5e080` | `-` |
| `ldr` at 0x5dddc | load | 4 | `abs 0x5e084` | `-` |
| `ldr` at 0x5ddec | load | 4 | `abs 0x5e05c` | `-` |
| `ldr` at 0x5ddf0 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x5de00 | load | 4 | `abs 0x5e088` | `-` |
| `ldr` at 0x5de0c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5de10 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5de14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5de28 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5de2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5de30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5de44 | load | 4 | `abs 0x5e08c` | `-` |
| `ldrb` at 0x5de5c | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5de64 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5de68 | load | 1 | `[sp-104] +0x4` | `-` |
| `str` at 0x5de7c | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5de84 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5de94 | load | 4 | `abs 0x5e090` | `-` |
| `ldrb` at 0x5dea0 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5dea4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x5debc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dec0 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x5ded0 | load | 4 | `abs 0x5e068` | `-` |
| `ldr` at 0x5ded4 | load | 4 | `abs 0x149110` | `-` |
| `ldr` at 0x5dee0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dee4 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5dee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5defc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5df00 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x5df04 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x5df0c | load | 4 | `abs 0x5e094` | `-` |
| `ldr` at 0x5df10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5df20 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5df24 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5df28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5df3c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5df40 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5df48 | load | 4 | `abs 0x5e098` | `-` |
| `str` at 0x5df5c | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5df64 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5df6c | load | 4 | `abs 0x5e074` | `-` |
| `ldr` at 0x5df70 | load | 4 | `abs 0x5e078` | `-` |
| `ldr` at 0x5df7c | load | 4 | `abs 0x5e068` | `-` |
| `ldr` at 0x5df80 | load | 4 | `abs 0x149110` | `-` |
| `ldr` at 0x5df8c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5df90 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5df94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dfa8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dfac | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x5dfb0 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x5dfb8 | load | 4 | `abs 0x5e09c` | `-` |
| `ldr` at 0x5dfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dfcc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dfd0 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5dfd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5dfe8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5dfec | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5dff4 | load | 4 | `abs 0x5e0a0` | `-` |
| `str` at 0x5e008 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5e010 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5e018 | load | 4 | `abs 0x5e074` | `-` |
| `ldr` at 0x5e01c | load | 4 | `abs 0x5e078` | `-` |
| `ldr` at 0x5e028 | load | 4 | `abs 0x5e05c` | `-` |
| `ldr` at 0x5e02c | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x5e03c | load | 4 | `abs 0x5e0a4` | `-` |
| `ldr` at 0x5e044 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x5e04c | store | 4 | `sp -0x60` | `?` |
| `ldmdb` at 0x5e054 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e054 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e054 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5dc20 | `(unknown callee) 0xd7530` | `0x0` | `?` | `sp-97` | `?` |
| `bl` at 0x5dc70 | `(unknown callee) 0x2079c` | `0x10cbb8` | `[sp-22]:1` | `[sp-23]:1` | `[sp-97]:1` |
| `bl` at 0x5dc90 | `(unknown callee) 0x2079c` | `0x10cbf4` | `[sp-22]:1` | `[sp-23]:1` | `[sp-97]:1` |
| `bl` at 0x5dcbc | `(unknown callee) 0x5d4ac` | `?` | `?` | `?` | `?` |
| `bl` at 0x5dd00 | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5dd28 | `(unknown callee) 0x1b52c` | `sp-92` | `0x10cc30` | `[sp-20]` | `ret(0x403f4)` |
| `bl` at 0x5dd40 | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5dd60 | `(unknown callee) 0x1b52c` | `sp-92` | `0x10cc60` | `ret(0x403f4)` | `[[sp-104]+2]:1` |
| `bl` at 0x5dd84 | `(unknown callee) 0x7aad8` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x5dd9c | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5ddbc | `(unknown callee) 0x1b52c` | `sp-92` | `0x10cc88` | `ret(0x403f4)` | `[[sp-104]+2]:1` |
| `bl` at 0x5dde4 | `(unknown callee) 0x7aad8` | `0x0` | `0x3401` | `0x1a2` | `sp-92` |
| `bl` at 0x5de04 | `(unknown callee) 0x2079c` | `0x10ccac` | `?` | `?` | `?` |
| `bl` at 0x5de38 | `(unknown callee) 0x40754` | `?` | `?` | `?` | `?` |
| `bl` at 0x5de48 | `(unknown callee) 0xfc33c` | `ret(0x40754)` | `0x42400000` | `?` | `ret(0x40754)` |
| `bl` at 0x5de98 | `(unknown callee) 0x7aad8` | `0x0` | `0x3500` | `0x1a4` | `0x10ccdc` |
| `bl` at 0x5deac | `(unknown callee) 0x5d428` | `[sp-104]` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x5def0 | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5df18 | `(unknown callee) 0x1b52c` | `sp-92` | `0x10ccf4` | `[sp-20]` | `ret(0x403f4)` |
| `bl` at 0x5df30 | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5df50 | `(unknown callee) 0x1b52c` | `sp-92` | `0x10cd24` | `ret(0x403f4)` | `[[sp-104]+2]:1` |
| `bl` at 0x5df74 | `(unknown callee) 0x7aad8` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x5df9c | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5dfc4 | `(unknown callee) 0x1b52c` | `sp-92` | `0x10cd4c` | `[sp-20]` | `ret(0x403f4)` |
| `bl` at 0x5dfdc | `(unknown callee) 0x403f4` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5dffc | `(unknown callee) 0x1b52c` | `sp-92` | `0x10cd74` | `ret(0x403f4)` | `[[sp-104]+2]:1` |
| `bl` at 0x5e020 | `(unknown callee) 0x7aad8` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x5e040 | `(unknown callee) 0x2079c` | `0x10cd94` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5dbc8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5dbf8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5dc0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5dc38 | `[sp-22]:1 == [sp-97]:1` | `[sp-22]:1 != [sp-97]:1` |
| `bne` at 0x5dc50 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5dcac | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5dcc8 | `ret(0x5d4ac) != 0x0` | `ret(0x5d4ac) == 0x0` |
| `beq` at 0x5dcdc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x5dcec | `[0x149110] <= 0x1 (signed, 32-bit)` | `[0x149110] > 0x1 (signed, 32-bit)` |
| `beq` at 0x5ddfc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5de24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5de54 | `ret(0xfc33c) == 0x0` | `ret(0xfc33c) != 0x0` |
| `beq` at 0x5de74 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5deb8 | `ret(0x5d428) != 0x0` | `ret(0x5d428) == 0x0` |
| `beq` at 0x5decc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x5dedc | `[0x149110] <= 0x1 (signed, 32-bit)` | `[0x149110] > 0x1 (signed, 32-bit)` |
| `ble` at 0x5df88 | `[0x149110] <= 0x1 (signed, 32-bit)` | `[0x149110] > 0x1 (signed, 32-bit)` |
| `beq` at 0x5e038 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e054 | `r0` | `?` |

## `mic74_Int`

- `mic74_Int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdac6c to 0xdaedc.
  Established. The unnamed `0xc8998` call carries unknown, `[[sp-28]+2]:1`, `0x3`, `sp-34` (`bl` at
  0xdace4); its zero result takes `0xdada8`, while nonzero falls through (`beq` at 0xdacf8). The
  unnamed `0xd7afc` call carries `[sp-24]`, `[sp-28]`, `[sp-32]`, `[sp-33]:1` (`bl` at 0xdade0); its
  result contributes to a word assignment at `sp -0x28` with value `([sp-40] + ret(0xd7afc))` (`str`
  at 0xdadf0). Interrupt effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdac7c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdac80 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdac84 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xdac88 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `str` at 0xdac90 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdac98 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdac9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdacac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdacb0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdacb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdacc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdaccc | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0xdacec | store | 4 | `sp -0x2c` | `ret(0xc8998)` |
| `ldr` at 0xdacf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdacfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdad04 | load | 4 | `abs 0xdaec8` | `-` |
| `ldr` at 0xdad08 | load | 4 | `abs 0xdaecc` | `-` |
| `str` at 0xdad34 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdad38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdad40 | load | 4 | `abs 0xdaec8` | `-` |
| `ldr` at 0xdad44 | load | 4 | `abs 0xdaecc` | `-` |
| `ldr` at 0xdad50 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdad60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdad6c | load | 4 | `abs 0xdaed0` | `-` |
| `str` at 0xdad70 | store | 4 | `(unknown)` | `0x1358e0` |
| `str` at 0xdad74 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xdad78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdad7c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xdad80 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xdad84 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0xdad8c | load | 4 | `abs 0xdaed4` | `-` |
| `ldr` at 0xdad9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdada0 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xdada8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xdadac | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xdadb0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xdadc4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xdadc8 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xdadcc | store | 4 | `sp -0x4c` | `[sp-34]:1` |
| `ldr` at 0xdadd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdadd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdadd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdade8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdadf0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0xd7afc))` |
| `ldr` at 0xdadf8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdadfc | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xdae04 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdae0c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdae14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdae1c | load | 4 | `abs 0xdaec8` | `-` |
| `ldr` at 0xdae20 | load | 4 | `abs 0xdaecc` | `-` |
| `str` at 0xdae4c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdae50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdae58 | load | 4 | `abs 0xdaec8` | `-` |
| `ldr` at 0xdae5c | load | 4 | `abs 0xdaecc` | `-` |
| `ldr` at 0xdae68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdae78 | load | 4 | `abs 0xdaed8` | `-` |
| `str` at 0xdae7c | store | 4 | `(unknown)` | `0x135924` |
| `str` at 0xdae80 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xdae84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdae88 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xdae8c | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0xdae90 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xdae94 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0xdae98 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xdae9c | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0xdaea0 | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0xdaea8 | load | 4 | `abs 0xdaed4` | `-` |
| `ldr` at 0xdaeb8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdaebc | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xdaec0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xdaec4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdaec4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaec4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaec4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdacbc | `(unknown callee) 0xc807c` | `?` | `?` | `?` | `?` |
| `bl` at 0xdace4 | `(unknown callee) 0xc8998` | `?` | `[[sp-28]+2]:1` | `0x3` | `sp-34` |
| `bl` at 0xdad0c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1358d4` | `0x1358dc` |
| `bl` at 0xdad18 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdad48 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1358d4` | `0x1358dc` |
| `bl` at 0xdad58 | `(unknown callee) 0x1b598` | `[sp-52]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdad64 | `(unknown callee) 0x21a38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xdad98 | `(unknown callee) 0x21eb8` | `0x3` | `0x135908` | `0x87` | `0x0` |
| `bl` at 0xdade0 | `(unknown callee) 0xd7afc` | `[sp-24]` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0xdae24 | `(unknown callee) 0x4f9a4` | `?` | `0x0` | `0x1358d4` | `0x1358dc` |
| `bl` at 0xdae30 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdae60 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1358d4` | `0x1358dc` |
| `bl` at 0xdae70 | `(unknown callee) 0x1b598` | `[sp-52]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xdaeb4 | `(unknown callee) 0x21eb8` | `0x3` | `0x135908` | `0x92` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xdaca4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xdacf8 | `ret(0xc8998) == 0x0` | `ret(0xc8998) != 0x0` |
| `beq` at 0xdadc0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaec4 | `r0` | `[sp-40]` |

## `pcf2119_Init_f`

- `pcf2119_Init_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaedc to 0xdaf84.
  Established. Included byte assignments write `0x10` at `r0 +0x2` (`strb` at 0xdaef8), `0x30` at
  `[sp-20] +0x3` (`strb` at 0xdaf04), and `0x1` at `[sp-20] +0x4` (`strb` at 0xdaf10). The byte
  write at `[sp-20] +0x5` has an unknown value (`strb` at 0xdaf1c). The absolute word at `0x2b4460`
  receives `[sp-20]` (`str` at 0xdaf64). Record roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdaeec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdaef0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdaef8 | store | 1 | `r0 +0x2` | `0x10` |
| `ldr` at 0xdaefc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdaf04 | store | 1 | `[sp-20] +0x3` | `0x30` |
| `ldr` at 0xdaf08 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdaf10 | store | 1 | `[sp-20] +0x4` | `0x1` |
| `ldr` at 0xdaf14 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdaf1c | store | 1 | `[sp-20] +0x5` | `?` |
| `ldr` at 0xdaf20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf24 | load | 4 | `abs 0xdaf6c` | `-` |
| `str` at 0xdaf28 | store | 4 | `[sp-20] +0x8` | `0x2b446b` |
| `ldr` at 0xdaf2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf30 | load | 4 | `abs 0xdaf70` | `-` |
| `str` at 0xdaf34 | store | 4 | `[sp-20] +0xc` | `0xdafa8` |
| `ldr` at 0xdaf38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf3c | load | 4 | `abs 0xdaf74` | `-` |
| `str` at 0xdaf40 | store | 4 | `[sp-20] +0x10` | `0xdb1a8` |
| `ldr` at 0xdaf44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf48 | load | 4 | `abs 0xdaf78` | `-` |
| `str` at 0xdaf4c | store | 4 | `[sp-20] +0x14` | `0xdb228` |
| `ldr` at 0xdaf50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf54 | load | 4 | `abs 0xdaf7c` | `-` |
| `str` at 0xdaf58 | store | 4 | `[sp-20] +0x18` | `0xdb2c4` |
| `ldr` at 0xdaf5c | load | 4 | `abs 0xdaf80` | `-` |
| `ldr` at 0xdaf60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdaf64 | store | 4 | `abs 0x2b4460` | `[sp-20]` |
| `ldmdb` at 0xdaf68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaf68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaf68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaf68 | `r0` | `r0` |

## `pcf2119_Setup`

- `pcf2119_Setup`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaf84 to 0xdafa8.
  Established. The absolute four-byte assignment at `0x1436d8` writes `0xdaedc` (`str` at 0xdaf98).
  The return preserves the entry `r0` (`ldmdb` at 0xdaf9c). The stored pointer consumer remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdaf90 | load | 4 | `abs 0xdafa0` | `-` |
| `ldr` at 0xdaf94 | load | 4 | `abs 0xdafa4` | `-` |
| `str` at 0xdaf98 | store | 4 | `abs 0x1436d8` | `0xdaedc` |
| `ldmdb` at 0xdaf9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaf9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaf9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaf9c | `r0` | `r0` |
