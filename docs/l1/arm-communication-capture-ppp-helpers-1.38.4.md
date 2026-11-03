# ARM communication capture and PPP helpers in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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
IRFmtAddr has an unresolved computed selector. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/communication-capture-ppp-helpers-facts.txt) retain all tool sections.

## `SysComLogInit_2m`

- `SysComLogInit_2m`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf134 to 0xaf268.
  Established. The first branch takes `[0x2b1730] != 0x0` and falls through on zero (`bne` at
  0xaf150). The fall-through word stores clear absolute `0x14971c` and `0x149720` (`str` at 0xaf15c,
  `str` at 0xaf168). The call to `0x1b124` has unknown, entry r1, entry r2 and unknown as candidates
  (`bl` at 0xaf190). Logging geometry and final r0 value remain unknown (`ldmdb` at 0xaf248).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaf144 | load | 4 | `abs 0xaf24c` | `-` |
| `ldr` at 0xaf148 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xaf154 | load | 4 | `abs 0xaf250` | `-` |
| `str` at 0xaf15c | store | 4 | `abs 0x14971c` | `0x0` |
| `ldr` at 0xaf160 | load | 4 | `abs 0xaf254` | `-` |
| `str` at 0xaf168 | store | 4 | `abs 0x149720` | `0x0` |
| `ldr` at 0xaf16c | load | 4 | `abs 0xaf258` | `-` |
| `str` at 0xaf174 | store | 4 | `abs 0x149724` | `0x0` |
| `ldr` at 0xaf17c | load | 4 | `abs 0xaf25c` | `-` |
| `ldr` at 0xaf180 | load | 4 | `abs 0xaf24c` | `-` |
| `ldr` at 0xaf184 | load | 4 | `abs 0x2b1730` | `-` |
| `str` at 0xaf198 | store | 4 | `abs 0x149a48` | `ret(0x1b124)` |
| `ldr` at 0xaf19c | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf1a8 | load | 4 | `abs 0xaf260` | `-` |
| `ldr` at 0xaf1b0 | load | 4 | `abs 0xaf250` | `-` |
| `str` at 0xaf1b8 | store | 4 | `abs 0x14971c` | `0x0` |
| `ldr` at 0xaf1bc | load | 4 | `abs 0xaf254` | `-` |
| `str` at 0xaf1c4 | store | 4 | `abs 0x149720` | `0x0` |
| `ldr` at 0xaf1c8 | load | 4 | `abs 0xaf258` | `-` |
| `str` at 0xaf1d0 | store | 4 | `abs 0x149724` | `0x0` |
| `str` at 0xaf1dc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xaf1e0 | load | 4 | `abs 0xaf24c` | `-` |
| `ldr` at 0xaf1e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf1e8 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xaf208 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf210 | load | 4 | `abs 0xaf25c` | `-` |
| `ldr` at 0xaf214 | load | 4 | `abs 0x149a48` | `-` |
| `str` at 0xaf21c | store | 4 | `(unknown)` | `ret(0xb97ac)` |
| `ldr` at 0xaf220 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf22c | load | 4 | `abs 0xaf264` | `-` |
| `ldr` at 0xaf230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf238 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaf240 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xaf248 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaf248 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf248 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf248 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf190 | `(unknown callee) 0x1b124` | `?` | `r1` | `r2` | `?` |
| `bl` at 0xaf1ac | `(unknown callee) 0x2070c` | `0x1274bc` | `?` | `?` | `[0x149a48]` |
| `bl` at 0xaf200 | `(unknown callee) 0xb97ac` | `0x1000` | `0x0` | `?` | `[0x2b1730]` |
| `bl` at 0xaf234 | `(unknown callee) 0x2070c` | `0x1274e4` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf150 | `[0x2b1730] != 0x0` | `[0x2b1730] == 0x0` |
| `bne` at 0xaf1a4 | `[0x149a48] != 0x0` | `[0x149a48] == 0x0` |
| `blt` at 0xaf1f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xaf228 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf248 | `r0` | `?` |

## `SysComLogInsert_2m`

- `SysComLogInsert_2m`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf268 to 0xaf2c4.
  Established. The global guard takes `[0x149a48] != 0x0` and falls through on zero (`bne` at
  0xaf290). The call to `0xb98d4` receives unknown, entry r1, entry r2 and unknown as candidates
  (`bl` at 0xaf2b8). The recorded exit expression is unknown (`ldmdb` at 0xaf2bc). Log insertion
  ownership remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf278 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaf27c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaf280 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xaf284 | load | 4 | `abs 0xaf2c0` | `-` |
| `ldr` at 0xaf288 | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf2a0 | load | 4 | `abs 0xaf2c0` | `-` |
| `ldr` at 0xaf2a4 | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf2ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf2b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf2b4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xaf2bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf2bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf2bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf2b8 | `(unknown callee) 0xb98d4` | `?` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf290 | `[0x149a48] != 0x0` | `[0x149a48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf2bc | `r0` | `?` |

## `SysComLogFetch_2m`

- `SysComLogFetch_2m`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf2c4 to 0xaf518.
  Established. The initial branch takes `Z == 0` and falls through on `not (Z == 0)` (`bne` at
  0xaf2e0). The opening call to `0x74908` carries `0x0`, entry r1, entry r2 and entry r0 as
  candidates (`bl` at 0xaf2e8). The merged r0 result and fetched log format remain unknown (`ldmdb`
  at 0xaf508).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf2d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf2d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf2f0 | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `ldr` at 0xaf2f4 | load | 4 | `abs 0xaf50c` | `-` |
| `ldr` at 0xaf2f8 | load | 4 | `abs 0x2b1730` | `-` |
| `str` at 0xaf30c | store | 4 | `sp -0x28` | `ret(0x1b124)` |
| `str` at 0xaf314 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xaf318 | load | 4 | `abs 0xaf50c` | `-` |
| `ldr` at 0xaf31c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf320 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xaf330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf338 | load | 4 | `abs 0xaf510` | `-` |
| `ldr` at 0xaf33c | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf344 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf35c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaf364 | store | 4 | `(unknown)` | `ret(0xb9d60)` |
| `ldr` at 0xaf368 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaf370 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0xaf37c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xaf384 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xaf388 | load | 4 | `abs 0xaf50c` | `-` |
| `ldr` at 0xaf38c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf390 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xaf3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf3a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf3b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf3c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaf3cc | store | 4 | `sp -0x20` | `(ret(0x1b22c) + [sp-32])+80` |
| `ldr` at 0xaf3d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaf3d8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xaf3e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaf3ec | store | 4 | `sp -0x24` | `ret(0x1b124)` |
| `ldr` at 0xaf3f0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xaf3f8 | store | 1 | `ret(0x1b124) +0x0` | `0x0` |
| `str` at 0xaf400 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xaf408 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xaf40c | load | 4 | `abs 0xaf50c` | `-` |
| `ldr` at 0xaf410 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf414 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xaf424 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf434 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf444 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf44c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaf450 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xaf454 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xaf45c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf46c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf478 | load | 4 | `abs 0xaf514` | `-` |
| `ldr` at 0xaf47c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf48c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf494 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf498 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf4a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf4a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf4b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf4b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaf4c0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xaf4c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf4d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaf4d4 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xaf4dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf4e4 | load | 4 | `abs 0xaf510` | `-` |
| `ldr` at 0xaf4e8 | load | 4 | `abs 0x149a48` | `-` |
| `ldr` at 0xaf4f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaf500 | store | 4 | `sp -0x2c` | `ret(0xb9d60)` |
| `ldr` at 0xaf504 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xaf508 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf508 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf508 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf2e8 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r0` |
| `bl` at 0xaf304 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xaf34c | `(unknown callee) 0xb9d60` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xaf3b4 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0xaf3e4 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `[0x2b1730]` |
| `bl` at 0xaf428 | `(unknown callee) 0x1b22c` | `[sp-36]` | `?` | `?` | `[0x2b1730]` |
| `bl` at 0xaf458 | `(unknown callee) 0x69d3c` | `(ret(0x1b22c) + [sp-36])` | `[[sp-24]+8]` | `[[sp-24]+16]` | `[[sp-24]+12]` |
| `bl` at 0xaf460 | `(unknown callee) 0x1b22c` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xaf480 | `(unknown callee) 0x1b49c` | `(ret(0x1b22c) + [sp-36])` | `0x12750c` | `[sp-28]` | `(ret(0x1b22c) + [sp-36])` |
| `bl` at 0xaf49c | `(unknown callee) 0x1b2d4` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xaf4b4 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0xaf4cc | `(unknown callee) 0x1b400` | `[sp-40]` | `?` | `?` | `[0x2b1730]` |
| `bl` at 0xaf4f8 | `(unknown callee) 0xb9d60` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf2e0 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xaf328 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xaf398 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xaf41c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf508 | `r0` | `?` |

## `SysComCaptureIn`

- `SysComCaptureIn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf9d8 to 0xb0044.
  Established. The signed comparison takes `[sp-24] < ? (signed, 32-bit)` and falls through on
  `[sp-24] >= ? (signed, 32-bit)` (`blt` at 0xafb10). The call to `0x1b304` has unknown, `[sp-28]`,
  `[sp-32]` and unknown as candidates (`bl` at 0xafd70). The exit records `0x2b1720` (`ldmdb` at
  0xb002c). Capture-record geometry remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf9e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaf9ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaf9f0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaf9f4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xaf9f8 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xaf9fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafa20 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafa28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafa54 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xafa5c | load | 1 | `(unknown)` | `-` |
| `str` at 0xafa60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xafa64 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafa68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafa8c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafa94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafac0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafac8 | load | 4 | `abs 0xb0034` | `-` |
| `ldrh` at 0xafacc | load | 2 | `(unknown)` | `-` |
| `str` at 0xafad0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xafad4 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafafc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafb04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafb08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafb18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xafb24 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xafb28 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafb2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafb50 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafb58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafb84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafb8c | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafb90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafbb4 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafbbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafbe8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xafbf0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xafc10 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xafc14 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafc18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafc3c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafc44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafc70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafc78 | load | 4 | `abs 0xb0034` | `-` |
| `ldr` at 0xafc7c | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafc80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafca4 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafcac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafcd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafce0 | load | 4 | `abs 0xb0034` | `-` |
| `ldrh` at 0xafce4 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xafcec | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xafcf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafd1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafd28 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafd2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafd50 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafd58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafd68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafd6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xafd74 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafd78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafd9c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafda4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafdd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafddc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xafde8 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xafdec | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xafdf0 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafdf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafe18 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafe20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafe4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafe54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafe58 | load | 4 | `abs 0xb0038` | `-` |
| `ldrh` at 0xafe68 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xafe6c | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xafe70 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafe74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafe98 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xafea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafecc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafed4 | load | 4 | `abs 0xb003c` | `-` |
| `ldr` at 0xafed8 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xafedc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff00 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xaff08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaff34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaff3c | load | 4 | `abs 0xb003c` | `-` |
| `ldr` at 0xaff40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaff48 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xaff4c | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xaff50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff74 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xaff7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaffa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaffb0 | load | 4 | `abs 0xb0040` | `-` |
| `ldr` at 0xaffb4 | load | 4 | `abs 0xb0030` | `-` |
| `ldr` at 0xaffb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaffdc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xaffe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0010 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0018 | load | 4 | `abs 0xb0040` | `-` |
| `ldr` at 0xb001c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0020 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0028 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xb002c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb002c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb002c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafd70 | `(unknown callee) 0x1b304` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xafb10 | `[sp-24] < ? (signed, 32-bit)` | `[sp-24] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb002c | `r0` | `0x2b1720` |

## `SysComCaptureOut`

- `SysComCaptureOut`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0044 to 0xb04ec.
  Established. The first branch takes `[sp-24] < ? (signed, 32-bit)` and falls through on `[sp-24]
  >= ? (signed, 32-bit)` (`blt` at 0xb017c). The call to `0x1b304` receives unknown, `[sp-28]`,
  `[sp-32]` and unknown as candidates (`bl` at 0xb03dc). The return records `0x2b1720` (`ldmdb` at
  0xb04dc). Outbound capture fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0054 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb0058 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb005c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb0060 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb0064 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb0068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb008c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb00c0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb00c8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb00cc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb00d0 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb00d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb00f8 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb012c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0134 | load | 4 | `abs 0xb04e4` | `-` |
| `ldrh` at 0xb0138 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb013c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb0140 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb0144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0168 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0170 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0174 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0184 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0190 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb0194 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb0198 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb01bc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb01c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb01f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb01f8 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb01fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0220 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0254 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb025c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xb027c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb0280 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb0284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb02a8 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb02b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb02dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb02e4 | load | 4 | `abs 0xb04e4` | `-` |
| `ldr` at 0xb02e8 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb02ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0310 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0344 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb034c | load | 4 | `abs 0xb04e4` | `-` |
| `ldrh` at 0xb0350 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb0358 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xb035c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0388 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0394 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb0398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb03bc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb03c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb03d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb03d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb03e0 | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb03e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0408 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb043c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0448 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb0454 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xb0458 | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xb045c | load | 4 | `abs 0xb04e0` | `-` |
| `ldr` at 0xb0460 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0484 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb048c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb04b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb04c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb04c4 | load | 4 | `abs 0xb04e8` | `-` |
| `ldrh` at 0xb04d4 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb04d8 | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldmdb` at 0xb04dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb04dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb04dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb03dc | `(unknown callee) 0x1b304` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb017c | `[sp-24] < ? (signed, 32-bit)` | `[sp-24] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb04dc | `r0` | `0x2b1720` |

## `SysComCaptureDump`

- `SysComCaptureDump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0870 to 0xb0eb8.
  Established. The signed guard takes `r0 < [0x2b1730] (signed, 32-bit)` and falls through at or
  above the bound (`blt` at 0xb089c). The call to `0x1b124` has `0x50`, entry r1, entry r0 and
  `[0x2b1730]` as candidates (`bl` at 0xb08a8). The exit carries `[sp-76]` (`ldmdb` at 0xb0e84). The
  capture dump's output ownership remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0880 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb0884 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb0888 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xb088c | load | 4 | `abs 0xb0e88` | `-` |
| `ldr` at 0xb0890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0894 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0xb08a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb08b0 | store | 4 | `r2 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb08b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb08b8 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb08bc | load | 4 | `abs 0xb0e8c` | `-` |
| `ldr` at 0xb08c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb08cc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb08d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb08e0 | load | 4 | `abs 0xb0e88` | `-` |
| `ldr` at 0xb08e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0908 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0910 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0914 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb091c | load | 4 | `abs 0xb0e88` | `-` |
| `ldr` at 0xb0920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0944 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb094c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0950 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb0954 | load | 4 | `abs 0xb0e88` | `-` |
| `ldr` at 0xb0958 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb097c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0xb0984 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0988 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb098c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0998 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb09a8 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb09ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb09b0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb09b4 | load | 4 | `abs 0xb0e90` | `-` |
| `str` at 0xb09c0 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0xb09cc | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xb09d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb09d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb09dc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb09f0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb09f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0a04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0a3c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb0a48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0a58 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb0a64 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb0a68 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb0a70 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb0a78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb0a80 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb0a88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0a8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb0aa0 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb0aa4 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xb0aa8 | store | 4 | `sp -0x40` | `[[sp-32]]` |
| `str` at 0xb0ab0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb0ab4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0ab8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb0ac8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0af4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0afc | load | 4 | `abs 0xb0e94` | `-` |
| `ldr` at 0xb0b00 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0b04 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xb0b08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0b34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0b3c | load | 4 | `abs 0xb0e98` | `-` |
| `ldr` at 0xb0b40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0b44 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xb0b48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0b4c | load | 4 | `abs 0xb0e9c` | `-` |
| `ldr` at 0xb0b50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0b54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0b5c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0b6c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb0b74 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x1b22c))` |
| `ldr` at 0xb0b78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0ba4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb0bac | load | 1 | `(unknown)` | `-` |
| `str` at 0xb0bb4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xb0bbc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb0bc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0bd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0bfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0c08 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb0c14 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb0c18 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb0c1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb0c28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0c54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0c5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0c60 | load | 4 | `abs 0xb0ea0` | `-` |
| `ldrh` at 0xb0c70 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb0c74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0c78 | load | 4 | `abs 0xb0ea4` | `-` |
| `ldr` at 0xb0c84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0c94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb0c9c | store | 4 | `sp -0x40` | `([sp-64] + ret(0x1b22c))` |
| `str` at 0xb0ca4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xb0ca8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb0cac | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb0cbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0ce8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0cf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0cfc | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xb0d04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0d1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0d48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0d50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0d5c | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xb0d64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0d68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0d6c | load | 4 | `abs 0xb0ea8` | `-` |
| `ldr` at 0xb0d7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0da8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0db0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0dbc | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xb0dc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0dc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0dcc | load | 4 | `abs 0xb0eac` | `-` |
| `ldr` at 0xb0dd8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb0de0 | store | 4 | `sp -0x40` | `[sp-64]+3` |
| `ldr` at 0xb0de4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb0dec | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xb0df4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0df8 | load | 4 | `abs 0xb0eb0` | `-` |
| `ldr` at 0xb0e00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0e10 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb0e18 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x1b22c))` |
| `ldr` at 0xb0e1c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb0e3c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb0e40 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb0e48 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xb0e50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb0e58 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xb0e60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0e64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0e68 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb0e74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb0e78 | load | 4 | `abs 0xb0eb4` | `-` |
| `ldr` at 0xb0e80 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0xb0e84 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb0e84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0e84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0e84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb08a8 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r0` | `[0x2b1730]` |
| `bl` at 0xb08c4 | `(unknown callee) 0x1b49c` | `[[sp-32]]` | `0x1275ec` | `[sp-24]` | `[sp-32]` |
| `bl` at 0xb09a0 | `(unknown callee) 0x1b124` | `0x50` | `[sp-24]` | `?` | `[sp-36]` |
| `bl` at 0xb09b8 | `(unknown callee) 0x1b508` | `[[sp-32]]` | `0x12760c` | `?` | `[sp-32]` |
| `bl` at 0xb0a98 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xb0b58 | `(unknown callee) 0x1b49c` | `?` | `0x127640` | `[sp-24]` | `?` |
| `bl` at 0xb0b60 | `(unknown callee) 0x1b22c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xb0c80 | `(unknown callee) 0x1b49c` | `?` | `0x127668` | `?` | `?` |
| `bl` at 0xb0c88 | `(unknown callee) 0x1b22c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xb0d0c | `(unknown callee) 0x1afbc` | `?` | `?` | `?` | `?` |
| `bl` at 0xb0d74 | `(unknown callee) 0x1b49c` | `[sp-64]` | `0x127670` | `?` | `?` |
| `bl` at 0xb0dd4 | `(unknown callee) 0x1b49c` | `[sp-64]` | `0x127678` | `?` | `?` |
| `bl` at 0xb0dfc | `(unknown callee) 0x1b508` | `?` | `0x127680` | `?` | `[sp-68]` |
| `bl` at 0xb0e04 | `(unknown callee) 0x1b22c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xb0e7c | `(unknown callee) 0x1b2d4` | `?` | `0x127684` | `?` | `[[sp-32]]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb089c | `r0 < [0x2b1730] (signed, 32-bit)` | `r0 >= [0x2b1730] (signed, 32-bit)` |
| `bne` at 0xb08dc | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xb0994 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `blt` at 0xb09e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xb09fc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0xb0ac0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xb0bc8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb0c24 | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `blt` at 0xb0cb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb0d18 | `ret(0x1afbc) == 0x0` | `ret(0x1afbc) != 0x0` |
| `bne` at 0xb0e70 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0e84 | `r0` | `[sp-76]` |

## `IRArg`

- `IRArg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7944 to 0xb7cc4.
  Established. The signed bound branch takes `r1 < [r0+32]:1 (signed, 32-bit)` and falls through at
  or above the bound (`blt` at 0xb796c). The first call to `0x1b124` carries unknown, entry r0,
  `[sp-52]` and unknown as candidates (`bl` at 0xb7ae4). The final r0 expression and argument record
  geometry remain unknown (`ldmdb` at 0xb7cc0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7954 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb7958 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb795c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7960 | load | 1 | `r0 +0x20` | `-` |
| `ldr` at 0xb7964 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7974 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb797c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7980 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb7994 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb79a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb79bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xb79c0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb79c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb79e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb79e4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb79e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb79ec | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xb79f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb79fc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb7a04 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0xb7a14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7a18 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7a1c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb7a24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7a34 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb7a3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7a40 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb7a54 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb7a58 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb7a5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7a60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7a74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7a78 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7a80 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb7a84 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb7a88 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb7a8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7a94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7aa0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb7aa4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7aa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7ab4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7ab8 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xb7ac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7ac4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7ad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7ad4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7ad8 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0xb7aec | store | 4 | `[sp-24] +0x8` | `ret(0x1b124)` |
| `ldr` at 0xb7af0 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xb7b00 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb7b08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7b0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7b10 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xb7b18 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7b28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7b2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7b34 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7b3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7b48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7b4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7b54 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7b5c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7b60 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb7b68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7b6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7b74 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7b7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7b80 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb7b94 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb7ba4 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xb7ba8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7bb8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb7bc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7bc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7bcc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7bd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7bd8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb7bdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7be0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7bf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7bf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7c00 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb7c04 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb7c08 | load | 2 | `[sp-32] +0x10` | `-` |
| `ldr` at 0xb7c0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7c14 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7c20 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb7c24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7c28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7c2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7c34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7c38 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7c40 | store | 4 | `sp -0x24` | `([sp-36] + [sp-44])` |
| `ldr` at 0xb7c44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7c48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7c50 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb7c54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7c60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7c64 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xb7c68 | store | 4 | `sp -0x20` | `[[sp-32]]` |
| `ldr` at 0xb7c6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7c74 | store | 4 | `sp -0x28` | `[[sp-32]]+36` |
| `ldr` at 0xb7c78 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb7c7c | load | 2 | `[[sp-32]] +0x10` | `-` |
| `ldr` at 0xb7c84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb7c90 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb7c94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7ca0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7cac | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7cb4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7cb8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb7cbc | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xb7cc0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb7cc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7cc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7cc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7ae4 | `(unknown callee) 0x1b124` | `?` | `r0` | `[sp-52]` | `?` |
| `bl` at 0xb7b24 | `(unknown callee) 0x1b43c` | `[[sp-24]+8]` | `0x0` | `?` | `?` |
| `bl` at 0xb7b9c | `(unknown callee) 0x1b124` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0xb7c30 | `(unknown callee) 0x1b304` | `[sp-36]` | `[sp-40]` | `[sp-44]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb796c | `r1 < [r0+32]:1 (signed, 32-bit)` | `r1 >= [r0+32]:1 (signed, 32-bit)` |
| `bne` at 0xb79a0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb79f8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb7a0c | `[sp-48]-1 != 0x0` | `[sp-48]-1 == 0x0` |
| `bne` at 0xb7a2c | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb7ab0 | `[sp-52] > [sp-44] (unsigned, 32-bit)` | `[sp-52] <= [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xb7acc | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `bne` at 0xb7af8 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `beq` at 0xb7b44 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb7bb0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb7c5c | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `bne` at 0xb7c9c | `[sp-52] != 0x0` | `[sp-52] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7cc0 | `r0` | `?` |

## `IRFmtClas`

- `IRFmtClas`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7cc4 to 0xb7d5c.
  Established. The first comparison takes `r0 == 0x1` and falls through on inequality (`beq` at
  0xb7ce8). The recorded return expression is unknown (`ldmdb` at 0xb7d48). Packet-class label
  meanings remain unproved beyond the branch table.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7cd4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb7cd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7cdc | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xb7ce0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7cec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7cf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7d18 | load | 4 | `abs 0xb7d4c` | `-` |
| `str` at 0xb7d1c | store | 4 | `sp -0x18` | `0x128318` |
| `ldr` at 0xb7d24 | load | 4 | `abs 0xb7d50` | `-` |
| `str` at 0xb7d28 | store | 4 | `sp -0x18` | `0x12831c` |
| `ldr` at 0xb7d30 | load | 4 | `abs 0xb7d54` | `-` |
| `str` at 0xb7d34 | store | 4 | `sp -0x18` | `0x128320` |
| `ldr` at 0xb7d3c | load | 4 | `abs 0xb7d58` | `-` |
| `str` at 0xb7d40 | store | 4 | `sp -0x18` | `0x128324` |
| `ldr` at 0xb7d44 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb7d48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7d48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7d48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7ce8 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0xb7cf4 | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xb7d00 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xb7d10 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7d48 | `r0` | `?` |

## `IRFmtAddr`

- `IRFmtAddr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7d5c to 0xb8030.
  Established. The opening branch takes `r0 >= 0x0 (signed, 32-bit)` and falls through on negative
  values (`bge` at 0xb7d84). The later selector remains unresolved (`ldrls` at 0xb7e70). The
  known-edge call to `0x1b49c` receives `[sp-24]`, `0x128358`, `[sp-32]` and `[sp-32]` as candidates
  (`bl` at 0xb7f9c). Address-format mappings on selected edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7d6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7d70 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb7d74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7d78 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb7d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7d88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7d9c | store | 4 | `sp -0x38` | `0xe` |
| `str` at 0xb7da8 | store | 4 | `sp -0x38` | `0xf` |
| `ldr` at 0xb7db0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7db8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb7dbc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb7dc4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb7dc8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb7dcc | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0xb7dd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7de0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb7de4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7df0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb7df4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7e08 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb7e0c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7e14 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xb7e20 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb7e24 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7e28 | store | 4 | `sp -0x2c` | `[sp-60]` |
| `ldr` at 0xb7e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7e40 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb7e44 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb7e4c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb7e54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7e5c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb7e60 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb7e64 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xb7e68 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xb7e70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7f90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7f94 | load | 4 | `abs 0xb8028` | `-` |
| `ldr` at 0xb7f98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7fb4 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x1b22c))` |
| `ldr` at 0xb7fb8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7fbc | store | 4 | `sp -0x4c` | `[sp-44]` |
| `ldr` at 0xb7fc0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb7fc4 | store | 4 | `sp -0x48` | `[sp-48]` |
| `ldr` at 0xb7fc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fcc | load | 4 | `abs 0xb802c` | `-` |
| `ldr` at 0xb7fd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7fd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7fdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fe8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7ff0 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x1b22c))` |
| `ldr` at 0xb7ff4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7ff8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb7ffc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xb8000 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8000 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8000 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7f9c | `(unknown callee) 0x1b49c` | `[sp-24]` | `0x128358` | `[sp-32]` | `[sp-32]` |
| `bl` at 0xb7fa4 | `(unknown callee) 0x1b22c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xb7fd8 | `(unknown callee) 0x1b49c` | `([sp-24] + ret(0x1b22c))` | `0x128360` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xb7fe0 | `(unknown callee) 0x1b22c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb7d84 | `r0 >= 0x0 (signed, 32-bit)` | `r0 < 0x0 (signed, 32-bit)` |
| `bne` at 0xb7d94 | `? != 0x2` | `? == 0x2` |
| `blt` at 0xb7dfc | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0xb7e34 | `[sp-20] >= 0x0 (signed, 32-bit)` | `[sp-20] < 0x0 (signed, 32-bit)` |
| `ldrls` at 0xb7e70 | `[sp-32] <= 0xf (unsigned, 32-bit)` | `[sp-32] > 0xf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8000 | `r0` | `[sp-28]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb7e70).

## `IRFmtPkt`

- `IRFmtPkt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8030 to 0xb80fc.
  Established. The opening call to `0xb7d5c` has `[r1+24]`, `sp-60`, `sp-60` and entry r1 as
  candidates (`bl` at 0xb805c). The return carries `[sp-28]` (`ldmdb` at 0xb80f4). The packet field
  names and formatting contract remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8040 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8044 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8048 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb804c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8054 | load | 4 | `r1 +0x18` | `-` |
| `ldr` at 0xb8060 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8068 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xb8074 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8078 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xb8088 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb808c | load | 1 | `[sp-24] +0x14` | `-` |
| `ldr` at 0xb8090 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8094 | load | 1 | `[sp-24] +0x21` | `-` |
| `str` at 0xb80a0 | store | 4 | `sp -0x74` | `ret(0xb7cc4)` |
| `ldr` at 0xb80a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb80a8 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0xb80ac | store | 4 | `sp -0x70` | `[[sp-24]+32]:1` |
| `ldr` at 0xb80b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb80b4 | load | 1 | `[sp-24] +0x13` | `-` |
| `str` at 0xb80b8 | store | 4 | `sp -0x6c` | `[[sp-24]+19]:1` |
| `ldr` at 0xb80bc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb80c0 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0xb80c4 | store | 4 | `sp -0x68` | `[[sp-24]+34]:2` |
| `str` at 0xb80cc | store | 4 | `sp -0x64` | `sp-60` |
| `str` at 0xb80d4 | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0xb80d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb80dc | load | 4 | `abs 0xb80f8` | `-` |
| `ldr` at 0xb80e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb80ec | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb80f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb80f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb80f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb805c | `(unknown callee) 0xb7d5c` | `[r1+24]` | `sp-60` | `sp-60` | `r1` |
| `bl` at 0xb8070 | `(unknown callee) 0xb7d5c` | `[[sp-24]+28]` | `sp-92` | `sp-92` | `[sp-24]` |
| `bl` at 0xb8080 | `(unknown callee) 0xb7cc4` | `[[sp-24]+22]:1` | `?` | `?` | `[[sp-24]+22]:1` |
| `bl` at 0xb80e8 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x128374` | `[sp-20]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb80f4 | `r0` | `[sp-28]` |

## `IRLogFrame`

- `IRLogFrame`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb80fc to 0xb820c.
  Established. The entry-r1 guard takes nonzero and falls through on zero (`bne` at 0xb811c). The
  first call to `0x20240` receives `0x12839c`, entry r1, entry r2 and entry r1 as candidates (`bl`
  at 0xb8124). The return records `ret(0x20240)` (`ldmdb` at 0xb81fc). Frame logging destination and
  field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb810c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8110 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8120 | load | 4 | `abs 0xb8200` | `-` |
| `ldr` at 0xb8128 | load | 4 | `abs 0xb8204` | `-` |
| `ldr` at 0xb8138 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb813c | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xb8140 | store | 4 | `sp -0xcc` | `[r1]` |
| `ldr` at 0xb8144 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8148 | load | 2 | `r1 +0x10` | `-` |
| `str` at 0xb814c | store | 4 | `sp -0xc8` | `[r1+16]:2` |
| `ldr` at 0xb8150 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8154 | load | 1 | `r1 +0x12` | `-` |
| `str` at 0xb8158 | store | 4 | `sp -0xc4` | `[r1+18]:1` |
| `ldr` at 0xb815c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8160 | load | 1 | `r1 +0x13` | `-` |
| `str` at 0xb8164 | store | 4 | `sp -0xc0` | `[r1+19]:1` |
| `ldr` at 0xb8168 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb816c | load | 1 | `r1 +0x14` | `-` |
| `str` at 0xb8170 | store | 4 | `sp -0xbc` | `[r1+20]:1` |
| `ldr` at 0xb8174 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0xb8178 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb8180 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb8184 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8188 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb8190 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xb8194 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8198 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xb819c | store | 4 | `sp -0xb0` | `[r1+22]:1` |
| `ldr` at 0xb81a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb81a4 | load | 4 | `r1 +0x18` | `-` |
| `str` at 0xb81a8 | store | 4 | `sp -0xac` | `[r1+24]` |
| `ldr` at 0xb81ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb81b0 | load | 4 | `r1 +0x1c` | `-` |
| `str` at 0xb81b4 | store | 4 | `sp -0xa8` | `[r1+28]` |
| `ldr` at 0xb81b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb81bc | load | 1 | `r1 +0x20` | `-` |
| `str` at 0xb81c0 | store | 4 | `sp -0xa4` | `[r1+32]:1` |
| `ldr` at 0xb81c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb81c8 | load | 1 | `r1 +0x21` | `-` |
| `str` at 0xb81cc | store | 4 | `sp -0xa0` | `[r1+33]:1` |
| `ldr` at 0xb81d0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb81d4 | load | 2 | `r1 +0x22` | `-` |
| `str` at 0xb81d8 | store | 4 | `sp -0x9c` | `[r1+34]:2` |
| `ldr` at 0xb81e0 | load | 4 | `abs 0xb8208` | `-` |
| `ldr` at 0xb81e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb81e8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb81fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb81fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb81fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8124 | `(unknown callee) 0x20240` | `0x12839c` | `r1` | `r2` | `r1` |
| `bl` at 0xb812c | `(unknown callee) 0x20240` | `0x1283ec` | `?` | `?` | `?` |
| `bl` at 0xb81ec | `(unknown callee) 0x1b49c` | `sp-152` | `0x12843c` | `r0` | `r1` |
| `bl` at 0xb81f8 | `(unknown callee) 0x20240` | `sp-152` | `?` | `?` | `sp-152` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb811c | `r1 != 0x0` | `r1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb81fc | `r0` | `ret(0x20240)` |

## `IRPrintFrame`

- `IRPrintFrame`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb820c to 0xb8324.
  Established. Nonzero entry r1 takes the initial branch and zero falls through (`bne` at 0xb822c).
  The first call to `0x24028` carries `[0x144dc8]`, `0x12848c`, entry r2 and `0x144dc8` as
  candidates (`bl` at 0xb823c). The exit records `ret(0x24028)` (`ldmdb` at 0xb8310). Frame-output
  fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb821c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8220 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8230 | load | 4 | `abs 0xb8314` | `-` |
| `ldr` at 0xb8234 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb8238 | load | 4 | `abs 0xb8318` | `-` |
| `ldr` at 0xb8240 | load | 4 | `abs 0xb8314` | `-` |
| `ldr` at 0xb8244 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb8248 | load | 4 | `abs 0xb831c` | `-` |
| `ldr` at 0xb8254 | load | 4 | `abs 0xb8314` | `-` |
| `ldr` at 0xb8258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb825c | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xb8260 | store | 4 | `sp -0xcc` | `[r1]` |
| `ldr` at 0xb8264 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8268 | load | 2 | `r1 +0x10` | `-` |
| `str` at 0xb826c | store | 4 | `sp -0xc8` | `[r1+16]:2` |
| `ldr` at 0xb8270 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8274 | load | 1 | `r1 +0x12` | `-` |
| `str` at 0xb8278 | store | 4 | `sp -0xc4` | `[r1+18]:1` |
| `ldr` at 0xb827c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8280 | load | 1 | `r1 +0x13` | `-` |
| `str` at 0xb8284 | store | 4 | `sp -0xc0` | `[r1+19]:1` |
| `ldr` at 0xb8288 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb828c | load | 1 | `r1 +0x14` | `-` |
| `str` at 0xb8290 | store | 4 | `sp -0xbc` | `[r1+20]:1` |
| `ldr` at 0xb8294 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0xb8298 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb82a0 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb82a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb82a8 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb82b0 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xb82b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb82b8 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xb82bc | store | 4 | `sp -0xb0` | `[r1+22]:1` |
| `ldr` at 0xb82c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb82c4 | load | 4 | `r1 +0x18` | `-` |
| `str` at 0xb82c8 | store | 4 | `sp -0xac` | `[r1+24]` |
| `ldr` at 0xb82cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb82d0 | load | 4 | `r1 +0x1c` | `-` |
| `str` at 0xb82d4 | store | 4 | `sp -0xa8` | `[r1+28]` |
| `ldr` at 0xb82d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb82dc | load | 1 | `r1 +0x20` | `-` |
| `str` at 0xb82e0 | store | 4 | `sp -0xa4` | `[r1+32]:1` |
| `ldr` at 0xb82e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb82e8 | load | 1 | `r1 +0x21` | `-` |
| `str` at 0xb82ec | store | 4 | `sp -0xa0` | `[r1+33]:1` |
| `ldr` at 0xb82f0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb82f4 | load | 2 | `r1 +0x22` | `-` |
| `str` at 0xb82f8 | store | 4 | `sp -0x9c` | `[r1+34]:2` |
| `ldr` at 0xb82fc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb8300 | load | 4 | `abs 0xb8320` | `-` |
| `ldr` at 0xb8304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8308 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8310 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8310 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8310 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb823c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x12848c` | `r2` | `0x144dc8` |
| `bl` at 0xb824c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1284dc` | `?` | `0x144dc8` |
| `bl` at 0xb830c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x12852c` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb822c | `r1 != 0x0` | `r1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8310 | `r0` | `ret(0x24028)` |

## `IRFmtError`

- `IRFmtError`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8324 to 0xb837c.
  Established. The first branch takes `r0 == 0x0` and falls through on nonzero (`beq` at 0xb8340).
  The call to `0x21674` has unknown, entry r1, unknown and entry r0 as candidates (`bl` at 0xb8354).
  The final r0 value and error-code conversion remain unknown (`ldmdb` at 0xb8370).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8334 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb8338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8348 | load | 4 | `abs 0xb8374` | `-` |
| `str` at 0xb835c | store | 4 | `sp -0x18` | `ret(0x21674)` |
| `ldr` at 0xb8364 | load | 4 | `abs 0xb8378` | `-` |
| `str` at 0xb8368 | store | 4 | `sp -0x18` | `0x12857c` |
| `ldr` at 0xb836c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8370 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8370 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8370 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8354 | `(unknown callee) 0x21674` | `?` | `r1` | `?` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8340 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8370 | `r0` | `?` |

## `PPPInit`

- `PPPInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba79c to 0xba86c.
  Established. The call to `0x1b43c` receives entry r0, `0x0`, `0x3c` and entry r3 as candidates
  (`bl` at 0xba7c8). The exit carries `ret(0xba86c)` (`ldmdb` at 0xba868). PPP record geometry and
  flag meanings remain unknown beyond the explicit accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba7ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba7b0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xba7b4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xba7b8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xba7bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba7cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba7d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xba7d4 | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xba7d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba7dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba7e0 | store | 4 | `[sp-20] +0x4` | `[sp-28]` |
| `ldr` at 0xba7e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba7e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba7ec | store | 4 | `[sp-20] +0x10` | `[sp-32]` |
| `ldr` at 0xba7f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba7f4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xba7f8 | store | 4 | `[sp-20] +0x14` | `[sp]` |
| `ldr` at 0xba7fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba800 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xba810 | store | 4 | `[sp-20] +0x20` | `?` |
| `ldr` at 0xba814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba818 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xba828 | store | 4 | `[sp-20] +0x24` | `?` |
| `ldr` at 0xba82c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba830 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xba834 | store | 4 | `[sp-20] +0x28` | `[sp+12]` |
| `ldr` at 0xba838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba83c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xba840 | store | 4 | `[sp-20] +0x2c` | `[sp+16]` |
| `ldr` at 0xba844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba848 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xba84c | store | 4 | `[sp-20] +0x34` | `[sp+20]` |
| `ldr` at 0xba850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba854 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xba858 | store | 4 | `[sp-20] +0x38` | `[sp+24]` |
| `ldr` at 0xba85c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xba868 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba868 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba868 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba7c8 | `(unknown callee) 0x1b43c` | `r0` | `0x0` | `0x3c` | `r3` |
| `bl` at 0xba864 | `(unknown callee) 0xba86c` | `[sp-20]` | `0x3` | `[sp-20]` | `[sp+24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba868 | `r0` | `ret(0xba86c)` |

## `PPPRead`

- `PPPRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbade0 to 0xbb764.
  Established. The signed entry-stack guard takes `[sp] >= 0x0 (signed, 32-bit)` and falls through
  on negative values (`bge` at 0xbae80). The first call to `0x1b2c8` has four unknown candidates
  (`bl` at 0xbaf2c). The final r0 expression is unknown (`ldmdb` at 0xbb740). Receive-buffer
  ownership and framing validity remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbadf0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbadf4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbadf8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xbadfc | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xbae04 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xbae0c | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xbae14 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xbae1c | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xbae24 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0xbae2c | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0xbae34 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbae3c | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbae44 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbae4c | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xbae54 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xbae5c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xbae60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbae64 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xbae68 | store | 4 | `sp -0x60` | `[r0+40]` |
| `ldr` at 0xbae6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbae70 | load | 4 | `r0 +0x2c` | `-` |
| `str` at 0xbae74 | store | 4 | `sp -0x64` | `[r0+44]` |
| `ldr` at 0xbae78 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbae88 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xbae8c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbae94 | store | 4 | `sp +0x0` | `?` |
| `ldr` at 0xbae98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbae9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbaea8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbaeac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbaeb0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbaeb8 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xbaec0 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbaec8 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbaed0 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xbaed8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbaee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaee8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbaef8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaefc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbaf00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbaf0c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbaf18 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbaf20 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xbaf24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbaf28 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbaf34 | store | 4 | `sp -0x38` | `ret(0x1b2c8)` |
| `ldr` at 0xbaf38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaf40 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xbaf48 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbaf4c | store | 1 | `sp -0x2d` | `?` |
| `str` at 0xbaf54 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbaf58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaf5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaf60 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbaf68 | store | 4 | `[sp-24] +0xc` | `[[sp-24]+12]-1` |
| `ldr` at 0xbaf70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbaf78 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xbaf80 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbaf84 | store | 1 | `([sp-64] + [sp-56]) +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xbaf8c | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xbaf90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaf94 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbafa0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbafa4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbafb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbafb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbafbc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xbafc0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbafc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbafcc | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbafd0 | store | 4 | `sp -0x44` | `[[sp-24]+12]` |
| `str` at 0xbafdc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbafe0 | load | 4 | `sp -0x5c` | `-` |
| `ldrb` at 0xbafec | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0xbaffc | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xbb000 | store | 4 | `sp -0x70` | `[sp-45]:1` |
| `ldr` at 0xbb004 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xbb010 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xbb01c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xbb02c | load | 4 | `sp -0x70` | `-` |
| `str` at 0xbb040 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xbb044 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb048 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0xbb054 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xbb060 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb064 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb068 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb074 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb078 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb07c | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb080 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb084 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb08c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbb090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb094 | load | 4 | `abs 0xbb744` | `-` |
| `ldr` at 0xbb09c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb0a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb0b4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb0c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb0c4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb0c8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb0d4 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb0d8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb0dc | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb0e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb0e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb0ec | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbb0f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb0f4 | load | 4 | `abs 0xbb748` | `-` |
| `ldr` at 0xbb0fc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb108 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb114 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xbb120 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xbb128 | store | 4 | `sp -0x48` | `[sp-72]-1` |
| `ldrb` at 0xbb12c | load | 1 | `[sp-72] -0x1` | `-` |
| `strb` at 0xbb130 | store | 1 | `sp -0x33` | `([[sp-72]-1]:1 & 0xff)` |
| `ldr` at 0xbb134 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xbb13c | store | 4 | `sp -0x48` | `[sp-72]-2` |
| `ldrb` at 0xbb140 | load | 1 | `[sp-72] -0x2` | `-` |
| `strb` at 0xbb144 | store | 1 | `sp -0x34` | `([[sp-72]-2]:1 & 0xff)` |
| `ldr` at 0xbb148 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbb150 | store | 4 | `sp -0x4c` | `[sp-76]-2` |
| `ldr` at 0xbb158 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0xbb164 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xbb168 | store | 1 | `sp -0x33` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xbb16c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xbb174 | store | 4 | `sp -0x48` | `[sp-72]-1` |
| `ldrb` at 0xbb178 | load | 1 | `[sp-72] -0x1` | `-` |
| `strb` at 0xbb17c | store | 1 | `sp -0x34` | `([[sp-72]-1]:1 & 0xff)` |
| `ldr` at 0xbb180 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbb188 | store | 4 | `sp -0x4c` | `[sp-76]-1` |
| `ldr` at 0xbb18c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb190 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb194 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xbb1a0 | load | 4 | `abs 0xbb74c` | `-` |
| `ldr` at 0xbb1a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb1c0 | load | 4 | `abs 0xbb750` | `-` |
| `str` at 0xbb1d0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xbb1d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb1dc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb1e0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb1ec | store | 4 | `sp -0x7c` | `sp-52` |
| `ldr` at 0xbb1f0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb1f4 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb1f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb1fc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb204 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbb208 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb20c | load | 4 | `abs 0xbb754` | `-` |
| `ldr` at 0xbb214 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb21c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbb228 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb230 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0xbb234 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb238 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb23c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb244 | store | 4 | `sp -0x48` | `([[sp-36]] + [sp-96])` |
| `str` at 0xbb24c | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb254 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbb25c | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xbb264 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xbb26c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb270 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xbb280 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xbb288 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb28c | load | 4 | `[sp-24] +0x24` | `-` |
| `ldr` at 0xbb298 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb2a0 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `ldr` at 0xbb2a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb2a8 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb2ac | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb2b8 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb2bc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb2c0 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb2c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb2c8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb2d0 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbb2d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb2d8 | load | 4 | `abs 0xbb758` | `-` |
| `ldr` at 0xbb2e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb2e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb2f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb2fc | load | 4 | `abs 0xbb758` | `-` |
| `str` at 0xbb300 | store | 4 | `sp -0x6c` | `0xfffffd40` |
| `ldr` at 0xbb308 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb30c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0xbb318 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xbb32c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb330 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb334 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb338 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb33c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb344 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbb348 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb360 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xbb368 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0xbb374 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbb37c | store | 1 | `sp -0x2d` | `?` |
| `str` at 0xbb384 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xbb388 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb38c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbb398 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xbb3a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb3a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb3ac | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb3b0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xbb3bc | store | 4 | `[sp-36] +0x0` | `ret(0x1b2c8)` |
| `ldr` at 0xbb3c0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb3cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb3d4 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `str` at 0xbb3dc | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb3e0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb3e4 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb3e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb3ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb3f4 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbb3f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb3fc | load | 4 | `abs 0xbb75c` | `-` |
| `ldr` at 0xbb40c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb418 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb420 | load | 4 | `abs 0xbb75c` | `-` |
| `str` at 0xbb424 | store | 4 | `sp -0x6c` | `0xfffffd3e` |
| `ldr` at 0xbb42c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb430 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb434 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbb43c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb444 | store | 4 | `sp -0x48` | `(([[sp-36]] + [sp]) + [sp-96])` |
| `ldr` at 0xbb448 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xbb44c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb454 | store | 4 | `sp +0x0` | `?` |
| `str` at 0xbb45c | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xbb464 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xbb470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb474 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb478 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb484 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb488 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb48c | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb490 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb494 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb49c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbb4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb4ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb4b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb4bc | store | 4 | `[sp-24] +0x30` | `0x0` |
| `str` at 0xbb4c4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbb4cc | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb4d4 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbb4dc | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xbb4e4 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xbb4ec | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xbb4f8 | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0xbb500 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbb504 | store | 1 | `[sp-80] +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xbb50c | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xbb510 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xbb518 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xbb524 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xbb52c | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbb530 | store | 1 | `[sp-72] +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xbb538 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xbb53c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbb544 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xbb54c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbb558 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbb564 | load | 4 | `abs 0xbb760` | `-` |
| `ldr` at 0xbb568 | load | 4 | `abs 0x149dc4` | `-` |
| `ldr` at 0xbb574 | load | 4 | `abs 0xbb760` | `-` |
| `ldr` at 0xbb578 | load | 4 | `abs 0x149dc4` | `-` |
| `ldr` at 0xbb580 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb584 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbb588 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xbb58c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb590 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb594 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb598 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbb59c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbb5a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb5a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbb5a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbb5b0 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xbb5bc | store | 4 | `(unknown)` | `ret(unknown)` |
| `ldr` at 0xbb5c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbb5cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb5d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb5d4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb5d8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb5e4 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb5e8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb5ec | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb5f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb5f4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb5fc | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbb600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb604 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbb60c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb614 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb61c | store | 4 | `[sp-24] +0x30` | `0x0` |
| `ldr` at 0xbb620 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb62c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb638 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbb63c | store | 4 | `sp -0x6c` | `[[sp-24]+12]` |
| `ldr` at 0xbb644 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb64c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb658 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbb660 | store | 4 | `sp -0x3c` | `[sp-60]+256` |
| `ldr` at 0xbb664 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb668 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbb674 | store | 4 | `sp -0x38` | `ret(0x1b2c8)` |
| `ldr` at 0xbb67c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb684 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xbb690 | store | 1 | `([sp-64] + [sp-56]) +0x0` | `0x55` |
| `str` at 0xbb698 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xbb6a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb6a8 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xbb6b4 | store | 1 | `([sp-64]+1 + [sp-56]) +0x0` | `?` |
| `str` at 0xbb6bc | store | 4 | `sp -0x40` | `[sp-64]+2` |
| `str` at 0xbb6c4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbb6c8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbb6d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb6d8 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xbb6e8 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbb6f0 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb6f8 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbb700 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb708 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xbb710 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xbb714 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbb720 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb72c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb734 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbb738 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xbb73c | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0xbb740 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbb740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbb740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbb740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbaf2c | `(unknown callee) 0x1b2c8` | `?` | `?` | `?` | `?` |
| `bl` at 0xbafc4 | `(unknown callee) 0x1b304` | `([sp-56] + [sp-64]+1)+1` | `[[sp-24]+8]` | `[[sp-24]+12]` | `([sp-56] + [sp-64]+1)+1` |
| `bl` at 0xbb0a0 | `(unknown callee) 0xba71c` | `[sp-24]` | `0xfffffd41` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb100 | `(unknown callee) 0xba71c` | `[sp-24]` | `0xfffffd42` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb1b0 | `(unknown callee) 0xba5a4` | `0xffff` | `([[sp-36]] + [sp-96])` | `[sp-76]` | `sp-52` |
| `bl` at 0xbb218 | `(unknown callee) 0xba71c` | `[sp-24]` | `0xfffffd44` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb2e4 | `(unknown callee) 0xba71c` | `[sp-24]` | `0xfffffd40` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb2f8 | `(unknown callee) 0x1b400` | `[sp-56]` | `?` | `?` | `[sp-56]` |
| `bl` at 0xbb358 | `(unknown callee) 0xba71c` | `[sp-24]` | `?` | `sp-45` | `0x1` |
| `bl` at 0xbb3b4 | `(unknown callee) 0x1b2c8` | `[[sp-36]]` | `[sp-100]` | `[sp-76]` | `[sp-36]` |
| `bl` at 0xbb408 | `(unknown callee) 0xba71c` | `[sp-24]` | `0xfffffd3e` | `0x0` | `0x0` |
| `bl` at 0xbb41c | `(unknown callee) 0x1b400` | `[sp-56]` | `?` | `?` | `[sp-56]` |
| `bl` at 0xbb4b0 | `(unknown callee) 0xba71c` | `[sp-24]` | `?` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb57c | `(unknown callee) 0x226f0` | `[0x149dc4]` | `?` | `?` | `0x149dc4` |
| `mov` at 0xbb5b8 | `(unknown callee) (unknown target)` | `[sp-32]` | `?` | `[[sp-24]+4]` | `?` |
| `bl` at 0xbb610 | `(unknown callee) 0xba71c` | `[sp-24]` | `[[sp-24]+12]` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb630 | `(unknown callee) 0x1b400` | `[sp-56]` | `?` | `[sp-24]` | `[sp-56]` |
| `bl` at 0xbb66c | `(unknown callee) 0x1b2c8` | `[sp-56]` | `[sp-60]+256` | `[sp-64]+3` | `[sp-60]+256` |
| `bl` at 0xbb730 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbae80 | `[sp] >= 0x0 (signed, 32-bit)` | `[sp] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbaea4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbaee0 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0xbaef0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `blo` at 0xbaf14 | `C == 0` | `not (C == 0)` |
| `beq` at 0xbaf9c | `[[sp-24]+12] == 0x0` | `[[sp-24]+12] != 0x0` |
| `beq` at 0xbafe8 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0xbaff4 | `[sp-45]:1 == 0x7e` | `[sp-45]:1 != 0x7e` |
| `beq` at 0xbb00c | `[sp-45]:1 == 0x7d` | `[sp-45]:1 != 0x7d` |
| `bgt` at 0xbb018 | `[sp-45]:1 > 0x7d (signed, 32-bit)` | `[sp-45]:1 <= 0x7d (signed, 32-bit)` |
| `beq` at 0xbb024 | `[sp-45]:1 == 0xd` | `[sp-45]:1 != 0xd` |
| `beq` at 0xbb034 | `[sp-45]:1 == 0x7e` | `[sp-45]:1 != 0x7e` |
| `beq` at 0xbb050 | `[[sp-24]+48] == 0x0` | `[[sp-24]+48] != 0x0` |
| `beq` at 0xbb05c | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `beq` at 0xbb0b0 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bhi` at 0xbb0bc | `[sp-76] > 0x2 (unsigned, 32-bit)` | `[sp-76] <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xbb110 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bne` at 0xbb11c | `[sp-84] != 0x0` | `[sp-84] == 0x0` |
| `bne` at 0xbb160 | `[sp-84] != 0x1` | `[sp-84] == 0x1` |
| `bne` at 0xbb1c8 | `? != 0xf0b8` | `? == 0xf0b8` |
| `bne` at 0xbb224 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xbb278 | `[[sp-24]+48] == 0x0` | `[[sp-24]+48] != 0x0` |
| `beq` at 0xbb294 | `[[sp-24]+36] == 0x0` | `[[sp-24]+36] != 0x0` |
| `beq` at 0xbb2f0 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bne` at 0xbb314 | `[[sp-24]+48] != 0x0` | `[[sp-24]+48] == 0x0` |
| `bne` at 0xbb320 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `beq` at 0xbb370 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `blo` at 0xbb394 | `[sp-76] < [sp] (unsigned, 32-bit)` | `[sp-76] >= [sp] (unsigned, 32-bit)` |
| `beq` at 0xbb3a0 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `bne` at 0xbb3c8 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `beq` at 0xbb414 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bls` at 0xbb46c | `[sp-84] <= 0x1 (unsigned, 32-bit)` | `[sp-84] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xbb554 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xbb560 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbb570 | `[0x149dc4] == 0x0` | `[0x149dc4] != 0x0` |
| `bge` at 0xbb5c8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xbb628 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `blo` at 0xbb654 | `[sp-64]+3 < [sp-60] (unsigned, 32-bit)` | `[sp-64]+3 >= [sp-60] (unsigned, 32-bit)` |
| `beq` at 0xbb6d0 | `[sp-104] == 0x0` | `[sp-104] != 0x0` |
| `bne` at 0xbb6e0 | `[[sp-24]+48] != 0x0` | `[[sp-24]+48] == 0x0` |
| `beq` at 0xbb71c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbb728 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbb740 | `r0` | `?` |
