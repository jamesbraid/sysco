# ARM JTAG and scan-reset control in 1.38.4

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
One routine has unresolved computed-transfer destinations; its tables and return cover only known edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/jtag-scan-reset-control-facts.txt) retain all tool sections.

## `JTAG_Init`

- `JTAG_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe312c to 0xe32fc.
  Established. The word store writes `[0x2b3840]+16` at `0x2b37a0` (`str` at 0xe3250). Four halfword
  stores at `[0x2b37a0] +0x0` carry `0x800`, `0x1000`, `0x800`, `0x800`, respectively (`strh` at
  0xe3260, `strh` at 0xe3270, `strh` at 0xe3280, `strh` at 0xe3290). A later word store writes `0x1`
  at `0x2ad258` (`str` at 0xe32b0). The exit propagates `ret(0x22920)` (`ldmdb` at 0xe32b4).
  Physical TAP signal meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe313c | load | 4 | `abs 0xe32b8` | `-` |
| `ldr` at 0xe3140 | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe314c | load | 4 | `abs 0xe32bc` | `-` |
| `str` at 0xe3150 | store | 4 | `sp -0x24` | `0x1355c8` |
| `ldr` at 0xe3158 | load | 4 | `abs 0xe32c0` | `-` |
| `ldr` at 0xe315c | load | 4 | `abs 0xe32c4` | `-` |
| `ldr` at 0xe3168 | load | 4 | `abs 0xe32c8` | `-` |
| `ldr` at 0xe316c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe3178 | load | 4 | `abs 0xe32cc` | `-` |
| `ldr` at 0xe3180 | load | 4 | `abs 0xe32d0` | `-` |
| `ldr` at 0xe3184 | load | 4 | `abs 0xe32d4` | `-` |
| `ldr` at 0xe318c | load | 4 | `abs 0xe32d8` | `-` |
| `ldr` at 0xe3190 | load | 4 | `abs 0xe32dc` | `-` |
| `str` at 0xe319c | store | 4 | `sp -0x1c` | `ret(0x1b370)` |
| `ldr` at 0xe31a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe31b8 | load | 4 | `abs 0xe32e0` | `-` |
| `ldr` at 0xe31bc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xe31c0 | load | 4 | `abs 0xe32e4` | `-` |
| `ldr` at 0xe31c4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xe31d4 | load | 4 | `abs 0xe32e8` | `-` |
| `ldr` at 0xe31d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe31dc | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe31e4 | store | 4 | `sp -0x20` | `0x10000000` |
| `str` at 0xe31fc | store | 4 | `abs 0x2b3840` | `ret(0x1b0a0)` |
| `ldr` at 0xe3200 | load | 4 | `abs 0x2b3840` | `-` |
| `ldr` at 0xe3218 | load | 4 | `abs 0xe32e0` | `-` |
| `ldr` at 0xe321c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xe3220 | load | 4 | `abs 0xe32ec` | `-` |
| `ldr` at 0xe3224 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xe3240 | load | 4 | `abs 0xe32f0` | `-` |
| `ldr` at 0xe3244 | load | 4 | `abs 0xe32e8` | `-` |
| `ldr` at 0xe3248 | load | 4 | `abs 0x2b3840` | `-` |
| `str` at 0xe3250 | store | 4 | `abs 0x2b37a0` | `[0x2b3840]+16` |
| `ldr` at 0xe3254 | load | 4 | `abs 0xe32f0` | `-` |
| `ldr` at 0xe3258 | load | 4 | `abs 0x2b37a0` | `-` |
| `strh` at 0xe3260 | store | 2 | `[0x2b37a0] +0x0` | `0x800` |
| `ldr` at 0xe3264 | load | 4 | `abs 0xe32f0` | `-` |
| `ldr` at 0xe3268 | load | 4 | `abs 0x2b37a0` | `-` |
| `strh` at 0xe3270 | store | 2 | `[0x2b37a0] +0x0` | `0x1000` |
| `ldr` at 0xe3274 | load | 4 | `abs 0xe32f0` | `-` |
| `ldr` at 0xe3278 | load | 4 | `abs 0x2b37a0` | `-` |
| `strh` at 0xe3280 | store | 2 | `[0x2b37a0] +0x0` | `0x800` |
| `ldr` at 0xe3284 | load | 4 | `abs 0xe32f0` | `-` |
| `ldr` at 0xe3288 | load | 4 | `abs 0x2b37a0` | `-` |
| `strh` at 0xe3290 | store | 2 | `[0x2b37a0] +0x0` | `0x800` |
| `ldr` at 0xe3294 | load | 4 | `abs 0xe32f4` | `-` |
| `ldr` at 0xe3298 | load | 4 | `abs 0xe32f8` | `-` |
| `ldr` at 0xe32a8 | load | 4 | `abs 0xe32b8` | `-` |
| `str` at 0xe32b0 | store | 4 | `abs 0x2ad258` | `0x1` |
| `ldmdb` at 0xe32b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe32b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe32b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe32b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3164 | `(unknown callee) 0x21e28` | `0x9` | `0x1355e0` | `0x102` | `0x0` |
| `bl` at 0xe317c | `(unknown callee) 0x23f2c` | `0x1355f8` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0xe3188 | `(unknown callee) 0xdc96c` | `0xe5ef0` | `0xe5c8c` | `?` | `?` |
| `bl` at 0xe3194 | `(unknown callee) 0x1b370` | `0x13562c` | `0x1002` | `?` | `?` |
| `bl` at 0xe31ac | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xe31c8 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x135638` | `[ret(0x1b0b8)]` | `0x144dc8` |
| `bl` at 0xe31d0 | `(unknown callee) 0x1b3ac` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xe31f8 | `(unknown callee) 0x1b0a0` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xe320c | `(unknown callee) 0x1b0b8` | `ret(0x1b0a0)` | `?` | `?` | `[0x2b3840]` |
| `bl` at 0xe3228 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x135658` | `[ret(0x1b0b8)]` | `0x144dc8` |
| `bl` at 0xe3230 | `(unknown callee) 0x1b3ac` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xe323c | `(unknown callee) 0xe32fc` | `0x1` | `0x0` | `?` | `?` |
| `bl` at 0xe32a4 | `(unknown callee) 0x22920` | `0x2b3860` | `0x135684` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3148 | `[0x2ad258] != 0x1` | `[0x2ad258] == 0x1` |
| `beq` at 0xe3174 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bge` at 0xe31a8 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0xe3208 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe32b4 | `r0` | `ret(0x22920)` |

## `JTAG_Lock`

- `JTAG_Lock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe335c to 0xe33c8.
  Established. The opening call to `0x22a38` has `0x2b3860`, unknown, entry r2, entry r3 (`bl` at
  0xe3374); its zero result takes the bypass and nonzero falls through (`beq` at 0xe3388). The final
  call to `0x21e28` carries `0x9`, `0x1355e0`, `0x170`, `0x0` (`bl` at 0xe33b4). Lock semantics and
  the common r0 result remain unproved (`ldmdb` at 0xe33b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe336c | load | 4 | `abs 0xe33bc` | `-` |
| `str` at 0xe337c | store | 4 | `sp -0x14` | `ret(0x22a38)` |
| `ldr` at 0xe3380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe338c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3398 | load | 4 | `abs 0xe33c0` | `-` |
| `str` at 0xe339c | store | 4 | `sp -0x1c` | `0x135690` |
| `str` at 0xe33a0 | store | 4 | `sp -0x18` | `ret(0x21674)` |
| `ldr` at 0xe33a8 | load | 4 | `abs 0xe33c4` | `-` |
| `ldmdb` at 0xe33b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe33b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe33b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3374 | `(unknown callee) 0x22a38` | `0x2b3860` | `?` | `r2` | `r3` |
| `bl` at 0xe3390 | `(unknown callee) 0x21674` | `ret(0x22a38)` | `?` | `?` | `ret(0x22a38)` |
| `bl` at 0xe33b4 | `(unknown callee) 0x21e28` | `0x9` | `0x1355e0` | `0x170` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe3388 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe33b8 | `r0` | `?` |

## `JTAG_Unlock`

- `JTAG_Unlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe33c8 to 0xe3430.
  Established. The call to `0x22dc0` receives `0x2b3860`, entry r1, entry r2, entry r3 (`bl` at
  0xe33dc); its zero result takes the bypass and nonzero falls through (`beq` at 0xe33f0). The last
  call to `0x21e28` has `0x9`, `0x1355e0`, `0x180`, `0x0` (`bl` at 0xe341c). Ownership semantics and r0
  at exit remain unresolved (`ldmdb` at 0xe3420).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe33d8 | load | 4 | `abs 0xe3424` | `-` |
| `str` at 0xe33e4 | store | 4 | `sp -0x14` | `ret(0x22dc0)` |
| `ldr` at 0xe33e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe33f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3400 | load | 4 | `abs 0xe3428` | `-` |
| `str` at 0xe3404 | store | 4 | `sp -0x1c` | `0x1356b0` |
| `str` at 0xe3408 | store | 4 | `sp -0x18` | `ret(0x21674)` |
| `ldr` at 0xe3410 | load | 4 | `abs 0xe342c` | `-` |
| `ldmdb` at 0xe3420 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3420 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3420 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe33dc | `(unknown callee) 0x22dc0` | `0x2b3860` | `r1` | `r2` | `r3` |
| `bl` at 0xe33f8 | `(unknown callee) 0x21674` | `ret(0x22dc0)` | `?` | `?` | `ret(0x22dc0)` |
| `bl` at 0xe341c | `(unknown callee) 0x21e28` | `0x9` | `0x1355e0` | `0x180` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe33f0 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3420 | `r0` | `?` |

## `scancmd_create`

- `scancmd_create`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc22c to 0xdc2f0.
  Established. The call to `0xdc2f0` uses `[sp-20]`, `ret(0x1b124)`, unknown, `ret(0x1b124)` (`bl`
  at 0xdc2bc); its nonzero result takes the continuation and zero falls through (`bne` at 0xdc2c8).
  The zero arm calls `0x1b400` with `[sp-24]`, unknown, unknown, `ret(0xdc2f0)` (`bl` at 0xdc2d0).
  Scan-command record ownership and the common r0 value remain unknown (`ldmdb` at 0xdc2ec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc23c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdc240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc250 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc258 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc25c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc268 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc270 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc278 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc27c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc288 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdc28c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc298 | store | 4 | `sp -0x18` | `ret(0x1b124)` |
| `ldr` at 0xdc29c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdc2ac | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdc2b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc2b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc2cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdc2d8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdc2e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdc2e4 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xdc2e8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdc2ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc2ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc2ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc290 | `(unknown callee) 0x1b124` | `[sp-32]` | `r1` | `r2` | `?` |
| `bl` at 0xdc2bc | `(unknown callee) 0xdc2f0` | `[sp-20]` | `ret(0x1b124)` | `?` | `ret(0x1b124)` |
| `bl` at 0xdc2d0 | `(unknown callee) 0x1b400` | `[sp-24]` | `?` | `?` | `ret(0xdc2f0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdc24c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdc2a4 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xdc2c8 | `ret(0xdc2f0) != 0x0` | `ret(0xdc2f0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc2ec | `r0` | `?` |

## `scancmd_init`

- `scancmd_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc2f0 to 0xdc38c.
  Established. The entry test takes its arm for `r1 != 0x0` and falls through on zero (`bne` at
  0xdc310). The sole call to `0x1b43c` has entry r1, `0x0`, `[sp-36]`, unknown (`bl` at 0xdc378). A
  later word store writes `0x1` at `sp -0x20` (`str` at 0xdc380). Command-record dimensions and the
  final r0 remain unidentified (`ldmdb` at 0xdc388).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc300 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc304 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdc308 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdc318 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdc320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc330 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc338 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc33c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc348 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc350 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc358 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc35c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc368 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc36c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc374 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc380 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xdc384 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdc388 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc388 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc388 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc378 | `(unknown callee) 0x1b43c` | `r1` | `0x0` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdc310 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xdc32c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc388 | `r0` | `?` |

## `scancmd_destroy`

- `scancmd_destroy`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc38c to 0xdc3bc.
  Established. The guard takes the call arm for entry `r0 != 0x0` and falls through on zero (`bne`
  at 0xdc3a8). The call to `0x1b400` has entry r0, entry r1, entry r2, entry r0 (`bl` at 0xdc3b4).
  The exit reports an unknown r0 (`ldmdb` at 0xdc3b8). Destruction and ownership semantics are not
  established solely by that callee address.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc39c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdc3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc3b0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdc3b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc3b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc3b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc3b4 | `(unknown callee) 0x1b400` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdc3a8 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc3b8 | `r0` | `?` |

## `jmstr_reset`

- `jmstr_reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc9a8 to 0xdca64.
  Established. The callback target is unknown, with candidates unknown, `0x20`, `sp-40`, `sp-40`
  (`mov` at 0xdca24); its nonzero result takes the continuation and zero falls through (`bne` at
  0xdca30). The other call reaches `0x1b49c` with `[sp-32]`, `0x13531c`, unknown, `ret(unknown)`
  (`bl` at 0xdca3c). TAP reset encoding and r0 at exit remain unknown (`ldmdb` at 0xdca58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc9b8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc9bc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdc9c0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xdc9c8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdc9d0 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xdc9d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc9e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc9e8 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xdc9ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc9f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdca00 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdca04 | load | 4 | `abs 0xdca5c` | `-` |
| `ldr` at 0xdca10 | load | 4 | `abs 0x2ad24c` | `-` |
| `ldr` at 0xdca14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdca34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdca38 | load | 4 | `abs 0xdca60` | `-` |
| `str` at 0xdca44 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdca50 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xdca54 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xdca58 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdca58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdca58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdca58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdca24 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xdca3c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x13531c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdc9dc | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xdc9f4 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xdca30 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdca58 | `r0` | `?` |

## `jmstr_reset_and_idle`

- `jmstr_reset_and_idle`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdca64 to 0xdcb88.
  Established. Both unknown-target callbacks carry unknown, `0x20`, `sp-40`, `sp-40` (`mov` at
  0xdcae0, `mov` at 0xdcb48). Each result test takes its continuation on nonzero and falls through
  on zero (`bne` at 0xdcaec, `bne` at 0xdcb54). Between the callbacks, a word store writes `0x10` at
  `sp -0x24` (`str` at 0xdcb0c). Reset-to-idle signaling and the merged r0 remain unproved (`ldmdb`
  at 0xdcb7c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdca74 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdca78 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdca7c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xdca84 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdca8c | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xdca90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdca9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdcaa4 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xdcaa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcab4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdcabc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdcac0 | load | 4 | `abs 0xdcb80` | `-` |
| `ldr` at 0xdcacc | load | 4 | `abs 0x2ad24c` | `-` |
| `ldr` at 0xdcad0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcaf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcaf4 | load | 4 | `abs 0xdcb84` | `-` |
| `str` at 0xdcb00 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdcb0c | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xdcb10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdcb1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdcb24 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xdcb28 | load | 4 | `abs 0xdcb80` | `-` |
| `ldr` at 0xdcb34 | load | 4 | `abs 0x2ad24c` | `-` |
| `ldr` at 0xdcb38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdcb58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcb5c | load | 4 | `abs 0xdcb84` | `-` |
| `str` at 0xdcb68 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdcb74 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xdcb78 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xdcb7c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdcb7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcb7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcb7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdcae0 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xdcaf8 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x13531c` | `?` | `ret(unknown)` |
| `mov` at 0xdcb48 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xdcb60 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x13531c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdca98 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xdcab0 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xdcaec | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xdcb18 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xdcb54 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcb7c | `r0` | `?` |

## `jmstr_scan`

- `jmstr_scan`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcf54 to 0xdd4e4.
  Established. The computed selector takes its arm for unsigned `r1 <= 0xd` and falls through for
  greater (`ldrls` at 0xdcfbc); its taken destinations are unresolved and excluded from the fact
  sections (`ldrls` at 0xdcfbc). The reported known-edge call to `0x1b508` has `[sp+12]`,
  `0x135374`, entry r2, entry r1 (`bl` at 0xdd11c). The known-edge exit leaves r0 unknown (`ldmdb`
  at 0xdd4d0). Scan bit ordering and callback arguments on excluded paths are unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcf64 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xdcf68 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xdcf6c | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xdcf70 | store | 4 | `sp -0x28` | `r3` |
| `str` at 0xdcf78 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdcf7c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xdcf8c | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xdcf98 | store | 4 | `sp -0x34` | `0x10` |
| `ldr` at 0xdcf9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdcfa8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdcfb0 | store | 4 | `sp -0x34` | `0x18` |
| `ldr` at 0xdcfb4 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xdcfbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd114 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xdd118 | load | 4 | `abs 0xdd4d4` | `-` |
| `str` at 0xdd124 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdd4cc | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xdd4d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xdd4d0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdd4d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd4d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd4d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdd11c | `(unknown callee) 0x1b508` | `[sp+12]` | `0x135374` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcf84 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0xdcfa4 | `r3 == 0x0` | `r3 != 0x0` |
| `ldrls` at 0xdcfbc | `r1 <= 0xd (unsigned, 32-bit)` | `r1 > 0xd (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd4d0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xdcfbc).

## `SHUB_ScanResetNI`

- `SHUB_ScanResetNI`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48b10 to 0x48cb0.
  Established. The two calls to `0x486e4` have `[sp-28]`, `0x0`, `0x1`, unknown and `[sp-28]`,
  `0x1`, `0x1`, unknown, respectively (`bl` at 0x48bbc, `bl` at 0x48c50). The first result selects
  its taken continuation on zero and falls through on nonzero (`beq` at 0x48bd0). A byte read
  addresses `0x148054` (`ldrb` at 0x48b3c). Network-interface reset behavior and the common r0
  result remain unknown (`ldmdb` at 0x48ca4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48b24 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x48b2c | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x48b34 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48b38 | load | 4 | `abs 0x48ca8` | `-` |
| `ldrb` at 0x48b3c | load | 1 | `abs 0x148054` | `-` |
| `ldr` at 0x48b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48b50 | load | 4 | `abs 0x48cac` | `-` |
| `ldr` at 0x48b54 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48b74 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x48b84 | load | 4 | `abs 0x48cac` | `-` |
| `ldr` at 0x48b88 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48ba8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x48bb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48bc4 | store | 4 | `sp -0x18` | `ret(0x486e4)` |
| `ldr` at 0x48bc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48bd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48bd8 | store | 4 | `sp -0x14` | `ret(0x486e4)` |
| `ldr` at 0x48bdc | load | 4 | `abs 0x48cac` | `-` |
| `ldr` at 0x48be0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48c00 | load | 2 | `(unknown)` | `-` |
| `str` at 0x48c14 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x48c18 | load | 4 | `abs 0x48cac` | `-` |
| `ldr` at 0x48c1c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48c3c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x48c44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48c60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x48c70 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x48c74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x48c78 | store | 4 | `sp -0x18` | `[sp-32]` |
| `ldr` at 0x48c84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48c88 | store | 4 | `sp -0x14` | `[sp-32]` |
| `ldr` at 0x48c8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48c94 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x48c9c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x48ca4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48ca4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48ca4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48bbc | `(unknown callee) 0x486e4` | `[sp-28]` | `0x0` | `0x1` | `?` |
| `bl` at 0x48c50 | `(unknown callee) 0x486e4` | `[sp-28]` | `0x1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x48b48 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x48b80 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x48bd0 | `ret(0x486e4) == 0x0` | `ret(0x486e4) != 0x0` |
| `bne` at 0x48c0c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x48c5c | `ret(0x486e4) == 0x0` | `ret(0x486e4) != 0x0` |
| `bne` at 0x48c68 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x48c80 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48ca4 | `r0` | `?` |

## `SHUB_ScanResetLLP`

- `SHUB_ScanResetLLP`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48cb0 to 0x4910c.
  Established. The call to `0x47a98` includes `[sp-28]`, `0x500305b0`, `0x1`, unknown (`bl` at
  0x48d48); its nonzero result takes the continuation and zero falls through (`bne` at 0x48d54).
  Later calls to `0x47b0c` include `[sp-28]`, `0x50002000`, `sp-20`, `sp-24` (`bl` at 0x4900c). Link
  reset register meanings and r0 at exit are unresolved (`ldmdb` at 0x490b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48cc0 | load | 4 | `abs 0x490bc` | `-` |
| `ldr` at 0x48cc4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x48cd0 | load | 4 | `abs 0x490c0` | `-` |
| `str` at 0x48cd4 | store | 4 | `sp -0x2c` | `0x109660` |
| `ldr` at 0x48cdc | load | 4 | `abs 0x490c4` | `-` |
| `ldr` at 0x48ce0 | load | 4 | `abs 0x490c8` | `-` |
| `str` at 0x48cf0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48cf4 | load | 4 | `abs 0x490cc` | `-` |
| `ldrb` at 0x48cf8 | load | 1 | `abs 0x148054` | `-` |
| `ldr` at 0x48cfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48d0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48d10 | load | 4 | `abs 0x490d0` | `-` |
| `ldr` at 0x48d2c | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48d30 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48d38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48d3c | load | 4 | `abs 0x490d8` | `-` |
| `ldr` at 0x48d58 | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48d5c | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48d64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48d6c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x48d74 | load | 4 | `abs 0x490dc` | `-` |
| `str` at 0x48d80 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48d84 | load | 4 | `abs 0x490cc` | `-` |
| `ldrb` at 0x48d88 | load | 1 | `abs 0x148054` | `-` |
| `ldr` at 0x48d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48da4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48da8 | load | 4 | `abs 0x490d0` | `-` |
| `ldr` at 0x48dc4 | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48dc8 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48de0 | load | 4 | `abs 0x490e0` | `-` |
| `str` at 0x48de4 | store | 4 | `sp -0x2c` | `0x109674` |
| `ldr` at 0x48de8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48dec | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x48df0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x48df4 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x48dfc | load | 4 | `abs 0x490c4` | `-` |
| `ldr` at 0x48e00 | load | 4 | `abs 0x490e4` | `-` |
| `ldr` at 0x48e1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48e20 | load | 4 | `abs 0x490d8` | `-` |
| `ldr` at 0x48e3c | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48e40 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48e58 | load | 4 | `abs 0x490e8` | `-` |
| `str` at 0x48e5c | store | 4 | `sp -0x2c` | `0x1096a4` |
| `ldr` at 0x48e60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48e64 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x48e68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x48e6c | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x48e74 | load | 4 | `abs 0x490c4` | `-` |
| `ldr` at 0x48e78 | load | 4 | `abs 0x490ec` | `-` |
| `ldr` at 0x48e8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48e94 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x48ea0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48ea4 | load | 4 | `abs 0x490cc` | `-` |
| `ldrb` at 0x48ea8 | load | 1 | `abs 0x148054` | `-` |
| `ldr` at 0x48eac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48ebc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48edc | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48ee0 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48ee8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48eec | load | 4 | `abs 0x490f0` | `-` |
| `ldr` at 0x48f08 | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48f0c | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48f14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48f1c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x48f30 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x48f34 | load | 4 | `abs 0x490cc` | `-` |
| `ldrb` at 0x48f38 | load | 1 | `abs 0x148054` | `-` |
| `ldr` at 0x48f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48f74 | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x48f78 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x48f80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48f8c | load | 4 | `abs 0x490f4` | `-` |
| `ldr` at 0x48f90 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x48fb0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x48fc0 | load | 4 | `abs 0x490f8` | `-` |
| `str` at 0x48fc4 | store | 4 | `sp -0x2c` | `0x1096d4` |
| `ldr` at 0x48fc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48fcc | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x48fd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48fd4 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x48fdc | load | 4 | `abs 0x490c4` | `-` |
| `ldr` at 0x48fe0 | load | 4 | `abs 0x490fc` | `-` |
| `ldr` at 0x48ffc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49000 | load | 4 | `abs 0x49100` | `-` |
| `ldr` at 0x4901c | load | 4 | `abs 0x490d4` | `-` |
| `str` at 0x49020 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x49028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49034 | load | 4 | `abs 0x490f4` | `-` |
| `ldr` at 0x49038 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x49058 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49068 | load | 4 | `abs 0x49104` | `-` |
| `str` at 0x4906c | store | 4 | `sp -0x2c` | `0x109704` |
| `ldr` at 0x49070 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49074 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x49078 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4907c | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x49084 | load | 4 | `abs 0x490c4` | `-` |
| `ldr` at 0x49088 | load | 4 | `abs 0x49108` | `-` |
| `ldr` at 0x4909c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x490a4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x490b0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x490b4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x490b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x490b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x490b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48ce8 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x289` | `0x0` |
| `bl` at 0x48d1c | `(unknown callee) 0x47a98` | `?` | `0x500303b0` | `0x1` | `?` |
| `bl` at 0x48d48 | `(unknown callee) 0x47a98` | `[sp-28]` | `0x500305b0` | `0x1` | `?` |
| `bl` at 0x48d78 | `(unknown callee) 0x4e0e4` | `0x1c2` | `?` | `[0x148054]:1` | `?` |
| `bl` at 0x48db4 | `(unknown callee) 0x47b0c` | `?` | `0x500303b0` | `sp-20` | `sp-24` |
| `bl` at 0x48e08 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x29b` | `0x0` |
| `bl` at 0x48e10 | `(unknown callee) 0x4e0e4` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x48e2c | `(unknown callee) 0x47b0c` | `[sp-28]` | `0x500305b0` | `sp-20` | `sp-24` |
| `bl` at 0x48e80 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x2a2` | `0x0` |
| `bl` at 0x48e88 | `(unknown callee) 0x4e0e4` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x48ecc | `(unknown callee) 0x47a98` | `?` | `0x50000008` | `0x0` | `0x1` |
| `bl` at 0x48ef8 | `(unknown callee) 0x47a98` | `[sp-28]` | `0x50002008` | `0x0` | `0x1` |
| `bl` at 0x48f28 | `(unknown callee) 0x4e0e4` | `0xa` | `?` | `[0x148054]:1` | `?` |
| `bl` at 0x48f64 | `(unknown callee) 0x47b0c` | `?` | `0x50000000` | `sp-20` | `sp-24` |
| `bl` at 0x48fe8 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x2bb` | `0x0` |
| `bl` at 0x48ff0 | `(unknown callee) 0x4e0e4` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x4900c | `(unknown callee) 0x47b0c` | `[sp-28]` | `0x50002000` | `sp-20` | `sp-24` |
| `bl` at 0x49090 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x2c2` | `0x0` |
| `bl` at 0x49098 | `(unknown callee) 0x4e0e4` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x48ccc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `blt` at 0x48d04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x48d28 | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `bne` at 0x48d54 | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `blt` at 0x48d94 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x48dc0 | `ret(0x47b0c) != 0x0` | `ret(0x47b0c) == 0x0` |
| `bne` at 0x48ddc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x48e38 | `ret(0x47b0c) != 0x0` | `ret(0x47b0c) == 0x0` |
| `bne` at 0x48e54 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x48eb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x48ed8 | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `bne` at 0x48f04 | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `blt` at 0x48f44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x48f70 | `ret(0x47b0c) != 0x0` | `ret(0x47b0c) == 0x0` |
| `beq` at 0x48f88 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x48fbc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x49018 | `ret(0x47b0c) != 0x0` | `ret(0x47b0c) == 0x0` |
| `beq` at 0x49030 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x49064 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x490b8 | `r0` | `?` |

## `SHUB_ScanInit_LLP_TEST_CTL`

- `SHUB_ScanInit_LLP_TEST_CTL`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4910c to 0x492bc.
  Established. Two calls to `0x47a98` have `[sp-20]`, `0x50000028`, `[sp-24]`, `0xa5fffff` and
  `[sp-20]`, `0x50002028`, `[sp-24]`, `0xa5fffff`, respectively (`bl` at 0x491c0, `bl` at 0x49240).
  The first result takes the continuation on nonzero and falls through on zero (`bne` at 0x491cc).
  The exit reports an unknown r0 (`ldmdb` at 0x49284). Test-control bit meanings remain
  unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4911c | load | 4 | `abs 0x49288` | `-` |
| `ldr` at 0x49120 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4912c | load | 4 | `abs 0x4928c` | `-` |
| `str` at 0x49130 | store | 4 | `sp -0x20` | `0x109734` |
| `ldr` at 0x49138 | load | 4 | `abs 0x49290` | `-` |
| `str` at 0x4914c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x49150 | load | 4 | `abs 0x49294` | `-` |
| `ldrb` at 0x49154 | load | 1 | `abs 0x148054` | `-` |
| `ldr` at 0x49158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49168 | load | 4 | `abs 0x49298` | `-` |
| `ldr` at 0x4916c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4918c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4919c | load | 4 | `abs 0x4929c` | `-` |
| `str` at 0x491a0 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x491ac | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x491b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x491b4 | load | 4 | `abs 0x492a0` | `-` |
| `ldr` at 0x491b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x491bc | load | 4 | `abs 0x492a4` | `-` |
| `ldr` at 0x491d0 | load | 4 | `abs 0x492a8` | `-` |
| `ldr` at 0x491d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x491dc | load | 4 | `abs 0x492ac` | `-` |
| `str` at 0x491e0 | store | 4 | `sp -0x1c` | `0xfffff88a` |
| `ldr` at 0x491e8 | load | 4 | `abs 0x49298` | `-` |
| `ldr` at 0x491ec | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4920c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4921c | load | 4 | `abs 0x4929c` | `-` |
| `str` at 0x49220 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x4922c | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x49230 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49234 | load | 4 | `abs 0x492b0` | `-` |
| `ldr` at 0x49238 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4923c | load | 4 | `abs 0x492a4` | `-` |
| `ldr` at 0x49250 | load | 4 | `abs 0x492b4` | `-` |
| `ldr` at 0x49254 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4925c | load | 4 | `abs 0x492b8` | `-` |
| `str` at 0x49260 | store | 4 | `sp -0x1c` | `0xfffff889` |
| `ldr` at 0x49268 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49270 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x4927c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49280 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x49284 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49284 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49284 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49144 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x2d8` | `0x0` |
| `bl` at 0x491c0 | `(unknown callee) 0x47a98` | `[sp-20]` | `0x50000028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x491d8 | `(unknown callee) 0x2070c` | `0x109750` | `[sp-20]` | `?` | `ret(0x47a98)` |
| `bl` at 0x49240 | `(unknown callee) 0x47a98` | `[sp-20]` | `0x50002028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x49258 | `(unknown callee) 0x2070c` | `0x109770` | `[sp-20]` | `?` | `ret(0x47a98)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x49128 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `blt` at 0x49160 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x49198 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x491cc | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `bne` at 0x49218 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x4924c | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49284 | `r0` | `?` |

## `SHUB_ScanInitLLP`

- `SHUB_ScanInitLLP`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x492bc to 0x49860.
  Established. The unknown-target callback carries `[sp-20]`, unknown, unknown, `[0x148068]` (`mov`
  at 0x49414). Three halfword stores have unknown targets and carry `0x5`, `0x0`, `0x5`,
  respectively (`strh` at 0x49494, `strh` at 0x494d0, `strh` at 0x49540). Later calls to `0xeac50`
  use `[sp-116]`, `0xa`, `sp-32`, `0x0` and `[sp-116]`, `0xb`, `sp-32`, `0x0` (`bl` at 0x49668, `bl`
  at 0x497bc). Link initialization meaning and the common r0 result remain unknown (`ldmdb` at
  0x49808).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x492cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x492d4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x492d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x492e0 | load | 4 | `abs 0x4980c` | `-` |
| `ldr` at 0x492e4 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x492ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x492f0 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x492f4 | load | 4 | `abs 0x49810` | `-` |
| `ldr` at 0x492f8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x49304 | load | 4 | `abs 0x49814` | `-` |
| `str` at 0x49308 | store | 4 | `sp -0x90` | `0x109790` |
| `ldr` at 0x4930c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49310 | store | 4 | `sp -0x8c` | `r0` |
| `ldr` at 0x49318 | load | 4 | `abs 0x49818` | `-` |
| `ldr` at 0x4931c | load | 4 | `abs 0x4981c` | `-` |
| `str` at 0x49354 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x49358 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49368 | load | 4 | `abs 0x49820` | `-` |
| `ldr` at 0x4936c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4937c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x49388 | load | 4 | `abs 0x49810` | `-` |
| `ldr` at 0x4938c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x49398 | load | 4 | `abs 0x49824` | `-` |
| `str` at 0x4939c | store | 4 | `sp -0x90` | `0x1097a4` |
| `ldr` at 0x493a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x493a4 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x493a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x493ac | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x493b0 | load | 4 | `abs 0x49828` | `-` |
| `ldr` at 0x493b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x493c8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x493e0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x493e4 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x493ec | load | 4 | `abs 0x49818` | `-` |
| `ldr` at 0x493f0 | load | 4 | `abs 0x4982c` | `-` |
| `ldr` at 0x49400 | load | 4 | `abs 0x49830` | `-` |
| `ldr` at 0x49404 | load | 4 | `abs 0x148068` | `-` |
| `ldr` at 0x49408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4940c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4941c | store | 4 | `sp -0x78` | `ret(unknown)` |
| `ldr` at 0x49420 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x49424 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x49428 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49434 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49440 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49450 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49460 | load | 4 | `abs 0x49828` | `-` |
| `ldr` at 0x49464 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49478 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x49494 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x4949c | load | 4 | `abs 0x49828` | `-` |
| `ldr` at 0x494a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x494b4 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x494d0 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x494d8 | load | 4 | `abs 0x49810` | `-` |
| `ldr` at 0x494dc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x494e8 | load | 4 | `abs 0x49834` | `-` |
| `str` at 0x494ec | store | 4 | `sp -0x90` | `0x1097e0` |
| `ldr` at 0x494f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x494f4 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x494fc | load | 4 | `abs 0x49818` | `-` |
| `ldr` at 0x4950c | load | 4 | `abs 0x49828` | `-` |
| `ldr` at 0x49510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49524 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x49540 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x49544 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4954c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x49554 | load | 4 | `abs 0x49828` | `-` |
| `ldr` at 0x49558 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x49578 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4957c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49584 | load | 4 | `abs 0x49838` | `-` |
| `ldr` at 0x49590 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49598 | load | 4 | `abs 0x49838` | `-` |
| `str` at 0x495a4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x495a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x495ac | load | 4 | `abs 0x4983c` | `-` |
| `ldr` at 0x495b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x495c0 | load | 4 | `abs 0x4983c` | `-` |
| `str` at 0x495cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x495d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x495d4 | load | 4 | `abs 0x49840` | `-` |
| `ldr` at 0x495e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x495e8 | load | 4 | `abs 0x49840` | `-` |
| `str` at 0x495f4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x495f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49608 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4961c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49630 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49644 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x49650 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x49654 | load | 4 | `abs 0x49844` | `-` |
| `str` at 0x49658 | store | 4 | `sp -0x8c` | `0x109810` |
| `ldr` at 0x4965c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x49678 | load | 4 | `abs 0x49848` | `-` |
| `str` at 0x4967c | store | 4 | `sp -0x90` | `0x10981c` |
| `ldr` at 0x49680 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49684 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x4968c | load | 4 | `abs 0x49818` | `-` |
| `ldr` at 0x49690 | load | 4 | `abs 0x4984c` | `-` |
| `ldr` at 0x4969c | load | 4 | `abs 0x49850` | `-` |
| `str` at 0x496a0 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `ldr` at 0x496a8 | load | 4 | `abs 0x49828` | `-` |
| `ldr` at 0x496ac | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x496cc | load | 2 | `(unknown)` | `-` |
| `str` at 0x496d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x496d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x496d8 | load | 4 | `abs 0x49838` | `-` |
| `ldr` at 0x496e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x496ec | load | 4 | `abs 0x49838` | `-` |
| `str` at 0x496f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x496fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49700 | load | 4 | `abs 0x4983c` | `-` |
| `ldr` at 0x4970c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49714 | load | 4 | `abs 0x4983c` | `-` |
| `str` at 0x49720 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49724 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49728 | load | 4 | `abs 0x49840` | `-` |
| `ldr` at 0x49734 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4973c | load | 4 | `abs 0x49840` | `-` |
| `str` at 0x49748 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4974c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4975c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49770 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49774 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49784 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49798 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x497a4 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x497a8 | load | 4 | `abs 0x49854` | `-` |
| `str` at 0x497ac | store | 4 | `sp -0x8c` | `0x109848` |
| `ldr` at 0x497b0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x497cc | load | 4 | `abs 0x49858` | `-` |
| `str` at 0x497d0 | store | 4 | `sp -0x90` | `0x109854` |
| `ldr` at 0x497d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x497d8 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x497e0 | load | 4 | `abs 0x49818` | `-` |
| `ldr` at 0x497e4 | load | 4 | `abs 0x4985c` | `-` |
| `ldr` at 0x497f0 | load | 4 | `abs 0x49850` | `-` |
| `str` at 0x497f4 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `str` at 0x49800 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x49804 | load | 4 | `sp -0x7c` | `-` |
| `ldmdb` at 0x49808 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49808 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49324 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x30b` | `0x0` |
| `bl` at 0x49338 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x4934c | `(unknown callee) 0xdcb88` | `0x0` | `0x0` | `0x20` | `sp-112` |
| `bl` at 0x493f8 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x319` | `0x0` |
| `mov` at 0x49414 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[0x148068]` |
| `bl` at 0x49508 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x328` | `0x0` |
| `bl` at 0x49668 | `(unknown callee) 0xeac50` | `[sp-116]` | `0xa` | `sp-32` | `0x0` |
| `bl` at 0x49698 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x336` | `0x0` |
| `bl` at 0x497bc | `(unknown callee) 0xeac50` | `[sp-116]` | `0xb` | `sp-32` | `0x0` |
| `bl` at 0x497ec | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x33e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x49300 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `ble` at 0x49360 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x49384 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x49394 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x49430 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |
| `bgt` at 0x4943c | `ret(unknown) > 0x1 (signed, 32-bit)` | `ret(unknown) <= 0x1 (signed, 32-bit)` |
| `beq` at 0x49448 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x49458 | `ret(unknown) == 0x2` | `ret(unknown) != 0x2` |
| `beq` at 0x494e4 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x49674 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x497c8 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49808 | `r0` | `?` |

## `SHUB_Reset_Hold`

- `SHUB_Reset_Hold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d334 to 0x4d358.
  Established. The byte save retains `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4d348); its return
  supplies r0 `0x0` (`ldmdb` at 0x4d354). The facts contain no calls or conditional branches.
  Physical SHUB reset assertion is not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d348 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x4d354 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d354 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d354 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d354 | `r0` | `0x0` |

## `SHUB_Reset_PostRelease`

- `SHUB_Reset_PostRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d8a0 to 0x4da4c.
  Established. The two calls to `0x47a98` use `[sp-17]:1`, `0x500303a0`, `0x7`, `0x0` and
  `[sp-17]:1`, `0x500305a0`, `0x7`, `0x0`, respectively (`bl` at 0x4d940, `bl` at 0x4d9a4). The
  first nonzero result takes its continuation and zero falls through (`bne` at 0x4d94c). The return
  reports r0 `[sp-24]` (`ldmdb` at 0x4da18). Post-release register meanings and physical readiness
  are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d8b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4d8bc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4d8c8 | load | 4 | `abs 0x4da1c` | `-` |
| `ldrb` at 0x4d8cc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d8ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4d8f8 | load | 4 | `abs 0x4da20` | `-` |
| `ldr` at 0x4d8fc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d908 | load | 4 | `abs 0x4da24` | `-` |
| `str` at 0x4d90c | store | 4 | `sp -0x20` | `0x10a36c` |
| `ldrb` at 0x4d910 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d914 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x4d91c | load | 4 | `abs 0x4da28` | `-` |
| `ldr` at 0x4d920 | load | 4 | `abs 0x4da2c` | `-` |
| `ldrb` at 0x4d92c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d934 | load | 4 | `abs 0x4da30` | `-` |
| `str` at 0x4d954 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d95c | load | 4 | `abs 0x4da20` | `-` |
| `ldr` at 0x4d960 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d96c | load | 4 | `abs 0x4da34` | `-` |
| `str` at 0x4d970 | store | 4 | `sp -0x20` | `0x10a38c` |
| `ldrb` at 0x4d974 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d978 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x4d980 | load | 4 | `abs 0x4da28` | `-` |
| `ldr` at 0x4d984 | load | 4 | `abs 0x4da38` | `-` |
| `ldrb` at 0x4d990 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d998 | load | 4 | `abs 0x4da3c` | `-` |
| `str` at 0x4d9b8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d9c0 | load | 4 | `abs 0x4da20` | `-` |
| `ldr` at 0x4d9c4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4d9d0 | load | 4 | `abs 0x4da40` | `-` |
| `str` at 0x4d9d4 | store | 4 | `sp -0x20` | `0x10a3ac` |
| `ldr` at 0x4d9dc | load | 4 | `abs 0x4da28` | `-` |
| `ldr` at 0x4d9e0 | load | 4 | `abs 0x4da44` | `-` |
| `ldrb` at 0x4d9ec | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d9fc | store | 4 | `sp -0x18` | `ret(0x48548)` |
| `ldrb` at 0x4da00 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4da04 | load | 4 | `abs 0x4da48` | `-` |
| `ldr` at 0x4da10 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4da18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4da18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4da18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d8c4 | `(unknown callee) 0x4e0e4` | `0x1` | `r1` | `r2` | `0x0` |
| `bl` at 0x4d928 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x7b1` | `0x0` |
| `bl` at 0x4d940 | `(unknown callee) 0x47a98` | `[sp-17]:1` | `0x500303a0` | `0x7` | `0x0` |
| `bl` at 0x4d98c | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x7b6` | `0x0` |
| `bl` at 0x4d9a4 | `(unknown callee) 0x47a98` | `[sp-17]:1` | `0x500305a0` | `0x7` | `0x0` |
| `bl` at 0x4d9e8 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x7bc` | `0x0` |
| `bl` at 0x4d9f4 | `(unknown callee) 0x48548` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4da0c | `(unknown callee) 0x2070c` | `0x10a3ec` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4d8f4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4d904 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bne` at 0x4d94c | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `beq` at 0x4d968 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bne` at 0x4d9b0 | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `beq` at 0x4d9cc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4da18 | `r0` | `[sp-24]` |
