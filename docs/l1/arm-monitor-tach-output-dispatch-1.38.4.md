# ARM monitor tachometer and output dispatch in 1.38.4

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
monitorInt has an unresolved computed selector. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/monitor-tach-output-dispatch-facts.txt) retain all tool sections.

## `monitorInit`

- `monitorInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbfec to 0xcc230.
  Established. A word read uses absolute `0x2ad200` (`ldr` at 0xcc00c). Zero takes the
  branch and nonzero falls through (`beq` at 0xcc014). The call to `0x1b3c4` carries `[sp-44]`,
  `0x1`, `0x2ad204` and unknown (`bl` at 0xcc090). The monitor allocation geometry and joined r0
  value remain unknown (`ldmdb` at 0xcc200).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbffc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcc000 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcc004 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xcc008 | load | 4 | `abs 0xcc204` | `-` |
| `ldr` at 0xcc00c | load | 4 | `abs 0x2ad200` | `-` |
| `ldr` at 0xcc018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc024 | load | 4 | `abs 0xcc208` | `-` |
| `str` at 0xcc028 | store | 4 | `sp -0x38` | `0x133d4c` |
| `ldr` at 0xcc02c | load | 4 | `abs 0xcc204` | `-` |
| `ldr` at 0xcc030 | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc034 | store | 4 | `sp -0x34` | `[0x2ad200]` |
| `ldr` at 0xcc038 | load | 4 | `abs 0xcc20c` | `-` |
| `ldr` at 0xcc03c | load | 4 | `abs 0x2ad204` | `-` |
| `str` at 0xcc040 | store | 4 | `sp -0x30` | `[0x2ad204]` |
| `ldr` at 0xcc048 | load | 4 | `abs 0xcc210` | `-` |
| `ldr` at 0xcc04c | load | 4 | `abs 0xcc214` | `-` |
| `ldr` at 0xcc058 | load | 4 | `abs 0xcc20c` | `-` |
| `ldr` at 0xcc05c | load | 4 | `abs 0x2ad204` | `-` |
| `ldr` at 0xcc068 | load | 4 | `abs 0xcc20c` | `-` |
| `str` at 0xcc070 | store | 4 | `abs 0x2ad204` | `0x1` |
| `ldr` at 0xcc074 | load | 4 | `abs 0xcc20c` | `-` |
| `ldr` at 0xcc078 | load | 4 | `abs 0x2ad204` | `-` |
| `str` at 0xcc080 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcc084 | load | 4 | `abs 0xcc204` | `-` |
| `ldr` at 0xcc088 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcc098 | store | 4 | `abs 0x2ad200` | `ret(0x1b3c4)` |
| `ldr` at 0xcc09c | load | 4 | `abs 0x2ad200` | `-` |
| `ldr` at 0xcc0a8 | load | 4 | `abs 0xcc218` | `-` |
| `str` at 0xcc0ac | store | 4 | `sp -0x38` | `0x133d98` |
| `ldr` at 0xcc0b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcc0b4 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xcc0bc | load | 4 | `abs 0xcc210` | `-` |
| `ldr` at 0xcc0c0 | load | 4 | `abs 0xcc21c` | `-` |
| `ldr` at 0xcc0cc | load | 4 | `abs 0xcc204` | `-` |
| `ldr` at 0xcc0d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc0d8 | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc0e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcc0e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc0e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcc0ec | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xcc0f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc0f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcc0f8 | store | 4 | `[sp-36] +0x8` | `[sp-32]` |
| `ldr` at 0xcc0fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc100 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc104 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xcc114 | store | 4 | `[sp-36] +0x4` | `ret(0x1b3c4)` |
| `ldr` at 0xcc118 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xcc124 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc12c | load | 4 | `abs 0xcc220` | `-` |
| `ldr` at 0xcc130 | load | 4 | `abs 0xcc224` | `-` |
| `str` at 0xcc15c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcc160 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc168 | load | 4 | `abs 0xcc220` | `-` |
| `ldr` at 0xcc16c | load | 4 | `abs 0xcc224` | `-` |
| `ldr` at 0xcc178 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc188 | load | 4 | `abs 0xcc228` | `-` |
| `str` at 0xcc18c | store | 4 | `(unknown)` | `0x133de0` |
| `str` at 0xcc190 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xcc194 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc198 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xcc19c | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `ldr` at 0xcc1a4 | load | 4 | `abs 0xcc210` | `-` |
| `ldr` at 0xcc1a8 | load | 4 | `abs 0xcc22c` | `-` |
| `str` at 0xcc1b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcc1bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc1c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc1c4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xcc1d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc1d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc1e0 | load | 4 | `[sp-36] +0x4` | `-` |
| `strb` at 0xcc1ec | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xcc1f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcc1f8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0xcc200 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcc200 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc200 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc200 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc054 | `(unknown callee) 0x21e28` | `0x9` | `0x133d7c` | `0x10f` | `0x0` |
| `bl` at 0xcc090 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `0x2ad204` | `?` |
| `bl` at 0xcc0c8 | `(unknown callee) 0x21e28` | `0x9` | `0x133d7c` | `0x11b` | `0x0` |
| `bl` at 0xcc10c | `(unknown callee) 0x1b3c4` | `[[sp-36]+8]` | `0x8` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xcc134 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc140 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc170 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc180 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc1b0 | `(unknown callee) 0x21e28` | `0x9` | `0x133d7c` | `0x126` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcc014 | `[0x2ad200] == 0x0` | `[0x2ad200] != 0x0` |
| `bne` at 0xcc020 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xcc064 | `[0x2ad204] != 0x0` | `[0x2ad204] == 0x0` |
| `bne` at 0xcc0a4 | `[0x2ad200] != 0x0` | `[0x2ad200] == 0x0` |
| `bne` at 0xcc120 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `blt` at 0xcc1cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc200 | `r0` | `?` |

## `monitorSetFlags`

- `monitorSetFlags`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc230 to 0xcc278.
  Established. A byte store writes `((r1 & 0xff) & 0xff)` at `[sp-28] +0xc` (`strb` at 0xcc26c). The
  word read feeding the record calculation uses absolute `0x2ad200` (`ldr` at 0xcc258). Entry r0
  remains the recorded exit value (`ldmdb` at 0xcc270). The flag definitions and record geometry are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc240 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc248 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcc24c | load | 4 | `abs 0xcc274` | `-` |
| `ldr` at 0xcc250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc258 | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc260 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcc264 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc268 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xcc26c | store | 1 | `[sp-28] +0xc` | `((r1 & 0xff) & 0xff)` |
| `ldmdb` at 0xcc270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc270 | `r0` | `r0` |

## `monitorNotPresent`

- `monitorNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc278 to 0xcc384.
  Established. The deciding branch takes `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` and falls
  through below that bound (`bge` at 0xcc2bc). A byte store on fall-through clears an unknown
  effective target (`strb` at 0xcc2d8). The other arm includes a call to `0x4f564` with entry r0,
  `0x0`, `0x133dd4` and `0x133ddc` (`bl` at 0xcc2f0). The physical presence policy is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc288 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc290 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcc294 | load | 4 | `abs 0xcc370` | `-` |
| `ldr` at 0xcc298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2a0 | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc2a8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcc2ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc2b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc2b4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcc2c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc2c4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc2cc | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xcc2d8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xcc2e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2e8 | load | 4 | `abs 0xcc374` | `-` |
| `ldr` at 0xcc2ec | load | 4 | `abs 0xcc378` | `-` |
| `str` at 0xcc318 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc31c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc324 | load | 4 | `abs 0xcc374` | `-` |
| `ldr` at 0xcc328 | load | 4 | `abs 0xcc378` | `-` |
| `ldr` at 0xcc334 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc344 | load | 4 | `abs 0xcc37c` | `-` |
| `str` at 0xcc348 | store | 4 | `(unknown)` | `0x133e18` |
| `str` at 0xcc34c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xcc350 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcc354 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcc35c | load | 4 | `abs 0xcc380` | `-` |
| `ldmdb` at 0xcc36c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc36c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc36c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc2f0 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc2fc | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc32c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc33c | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc368 | `(unknown callee) 0x21e28` | `0x6` | `0x133d7c` | `0x140` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcc2bc | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc36c | `r0` | `?` |

## `monitorPresent`

- `monitorPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc384 to 0xcc494.
  Established. The range branch takes `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` and falls
  through below the bound (`bge` at 0xcc3c8). A fall-through byte store writes `0x1` to an unknown
  effective target (`strb` at 0xcc3e4). The call to `0x4f564` on the other path receives entry r0,
  `0x0`, `0x133dd4` and `0x133ddc` (`bl` at 0xcc3fc). The presence encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc394 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc39c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xcc3a0 | load | 4 | `abs 0xcc47c` | `-` |
| `ldr` at 0xcc3a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc3ac | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc3b4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xcc3b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc3bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc3c0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcc3cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc3d0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc3d8 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xcc3e4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xcc3ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc3f4 | load | 4 | `abs 0xcc480` | `-` |
| `ldr` at 0xcc3f8 | load | 4 | `abs 0xcc484` | `-` |
| `str` at 0xcc424 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc430 | load | 4 | `abs 0xcc480` | `-` |
| `ldr` at 0xcc434 | load | 4 | `abs 0xcc484` | `-` |
| `ldr` at 0xcc440 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc450 | load | 4 | `abs 0xcc488` | `-` |
| `str` at 0xcc454 | store | 4 | `(unknown)` | `0x133e40` |
| `str` at 0xcc458 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xcc45c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xcc460 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcc468 | load | 4 | `abs 0xcc48c` | `-` |
| `ldr` at 0xcc46c | load | 4 | `abs 0xcc490` | `-` |
| `ldmdb` at 0xcc478 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc478 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc3fc | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc408 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc438 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc448 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc474 | `(unknown callee) 0x21e28` | `0x6` | `0x133d7c` | `0x14f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcc3c8 | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc478 | `r0` | `?` |

## `monitorIsPresent`

- `monitorIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc494 to 0xcc4d8.
  Established. The call to `0xce238` receives entry r0, `(r1 & 0xff)`, entry r2 and `(r1 & 0xff)`
  (`bl` at 0xcc4bc); its result is saved as a word at `sp -0x1c` (`str` at 0xcc4c4). The recorded
  exit carries `[ret(0xce238)]:1` (`ldmdb` at 0xcc4d4). The byte's hardware meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc4a4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcc4ac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xcc4b0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc4b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc4c4 | store | 4 | `sp -0x1c` | `ret(0xce238)` |
| `ldr` at 0xcc4c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc4cc | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldmdb` at 0xcc4d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc4d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc4d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc4bc | `(unknown callee) 0xce238` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc4d4 | `r0` | `[ret(0xce238)]:1` |

## `monitorGetCfg`

- `monitorGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc7c8 to 0xcc81c.
  Established. The word read at absolute `0x2ad200` feeds a record calculation (`ldr` at 0xcc7ec). A
  word store writes `[[sp-28]]` at `r1 +0x0` (`str` at 0xcc804), while the exit carries
  `[[sp-28]+8]` (`ldmdb` at 0xcc814). The configuration record layout remains unknown beyond the
  explicit accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc7d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcc7dc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcc7e0 | load | 4 | `abs 0xcc818` | `-` |
| `ldr` at 0xcc7e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc7ec | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc7f4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcc7f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc7fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc800 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcc804 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xcc808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc80c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xcc814 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc814 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc814 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc814 | `r0` | `[[sp-28]+8]` |

## `monitorCount`

- `monitorCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc81c to 0xcc85c.
  Established. A word load reads absolute `0x2ad200` (`ldr` at 0xcc83c). The body stores an unknown
  calculated value at `sp -0x18` (`str` at 0xcc844). The return expression is `[[sp-24]+8]` (`ldmdb`
  at 0xcc854); its interpretation as a count remains unknown here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc82c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcc830 | load | 4 | `abs 0xcc858` | `-` |
| `ldr` at 0xcc834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc83c | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc844 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xcc848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc84c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0xcc854 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc854 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc854 | `r0` | `[[sp-24]+8]` |

## `monitorSetTachLimit`

- `monitorSetTachLimit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcce10 to 0xccf4c.
  Established. Two word saves record `ret(0xce118)` at `sp -0x20` and `ret(0xce238)` at `sp -0x24`
  (`str` at 0xcce4c, `str` at 0xcce64). The branch takes `[ret(0xce238)]:1 != 0x0` and falls through
  on zero (`bne` at 0xcce74). The fall-through word store writes `0xfffffc11` at `sp -0x28` (`str`
  at 0xcce7c). Tachometer units and limit semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcce20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcce24 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcce2c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcce34 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xcce38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcce3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcce4c | store | 4 | `sp -0x20` | `ret(0xce118)` |
| `ldrb` at 0xcce50 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcce54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcce64 | store | 4 | `sp -0x24` | `ret(0xce238)` |
| `ldr` at 0xcce68 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcce6c | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcce78 | load | 4 | `abs 0xccf44` | `-` |
| `str` at 0xcce7c | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xcce84 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcce88 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcce8c | store | 4 | `sp -0x2c` | `[[sp-32]]:1` |
| `ldr` at 0xcce90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcce9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xccea8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcceb8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xccec8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xccecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcced0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcced8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xccee4 | store | 4 | `sp -0x28` | `ret(0xcf0f4)` |
| `ldrb` at 0xcceec | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xccef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccef4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccefc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xccf08 | store | 4 | `sp -0x28` | `ret(0xcfd10)` |
| `ldrb` at 0xccf10 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xccf14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccf18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccf20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xccf2c | store | 4 | `sp -0x28` | `ret(0xd127c)` |
| `ldr` at 0xccf34 | load | 4 | `abs 0xccf48` | `-` |
| `str` at 0xccf38 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xccf3c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xccf40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xccf40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xccf40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcce44 | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcce5c | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xccedc | `(unknown callee) 0xcf0f4` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xccf00 | `(unknown callee) 0xcfd10` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xccf24 | `(unknown callee) 0xd127c` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcce74 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `beq` at 0xcce98 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xccea4 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcceb0 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xccec0 | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xccf40 | `r0` | `?` |

## `monitorReadTach`

- `monitorReadTach`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd334 to 0xcd47c.
  Established. Before the first call, a word store clears `r3 +0x0` (`str` at 0xcd364). The call to `0xce118` has
  candidates `[sp-20]`, `[sp-21]:1`, entry r3 and `[sp-21]:1` (`bl` at 0xcd374). The next
  result-byte test takes nonzero and falls through on zero (`bne` at 0xcd3a4); its returned r0 value
  and tachometer measurement units remain unknown (`ldmdb` at 0xcd470).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd344 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd348 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcd350 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd358 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xcd35c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd364 | store | 4 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xcd368 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd36c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd37c | store | 4 | `sp -0x20` | `ret(0xce118)` |
| `ldrb` at 0xcd380 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd384 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd394 | store | 4 | `sp -0x24` | `ret(0xce238)` |
| `ldr` at 0xcd398 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcd39c | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcd3a8 | load | 4 | `abs 0xcd474` | `-` |
| `str` at 0xcd3ac | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xcd3b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd3b8 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcd3bc | store | 4 | `sp -0x2c` | `[[sp-32]]:1` |
| `ldr` at 0xcd3c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd3cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd3d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd3e8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xcd3f8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd3fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd400 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd408 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd414 | store | 4 | `sp -0x28` | `ret(0xcefc8)` |
| `ldrb` at 0xcd41c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd420 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd424 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd42c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd438 | store | 4 | `sp -0x28` | `ret(0xcfb6c)` |
| `ldrb` at 0xcd440 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd448 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd450 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd45c | store | 4 | `sp -0x28` | `ret(0xd1188)` |
| `ldr` at 0xcd464 | load | 4 | `abs 0xcd478` | `-` |
| `str` at 0xcd468 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xcd46c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcd470 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd470 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd470 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd374 | `(unknown callee) 0xce118` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xcd38c | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcd40c | `(unknown callee) 0xcefc8` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xcd430 | `(unknown callee) 0xcfb6c` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xcd454 | `(unknown callee) 0xd1188` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd3a4 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `beq` at 0xcd3c8 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xcd3d4 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcd3e0 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xcd3f0 | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd470 | `r0` | `?` |

## `monitorWriteOutput`

- `monitorWriteOutput`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd47c to 0xcd71c.
  Established. The opening byte saves retain `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at `sp
  -0x15`, `sp -0x16` and `sp -0x17` (`strb` at 0xcd498, `strb` at 0xcd4a0, `strb` at 0xcd4a8). The
  first result-byte branch takes `[ret(0xce238)]:1 != 0x0` and falls through on zero (`bne` at
  0xcd4e8). A later word store places `0xfffffc11` at `sp -0x2c` (`str` at 0xcd4f0). Output device,
  scale and polarity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd48c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcd498 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd4a0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcd4a8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xcd4ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd4b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd4c0 | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldrb` at 0xcd4c4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd4c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd4d8 | store | 4 | `sp -0x20` | `ret(0xce238)` |
| `ldr` at 0xcd4dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd4e0 | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcd4ec | load | 4 | `abs 0xcd710` | `-` |
| `str` at 0xcd4f0 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcd4f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd4fc | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xcd500 | store | 4 | `sp -0x34` | `[[sp-28]]:1` |
| `ldr` at 0xcd504 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd510 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd51c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd52c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xcd53c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd548 | load | 4 | `abs 0xcd714` | `-` |
| `str` at 0xcd54c | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `strb` at 0xcd558 | store | 1 | `sp -0x21` | `0x19` |
| `ldrb` at 0xcd560 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xcd564 | store | 4 | `sp -0x30` | `[sp-22]:1` |
| `ldr` at 0xcd568 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcd574 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcd580 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcd590 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xcd5a4 | store | 1 | `sp -0x21` | `0x30` |
| `strb` at 0xcd5ac | store | 1 | `sp -0x22` | `0x5c` |
| `strb` at 0xcd5b8 | store | 1 | `sp -0x21` | `0x31` |
| `strb` at 0xcd5c0 | store | 1 | `sp -0x22` | `0x5d` |
| `strb` at 0xcd5cc | store | 1 | `sp -0x21` | `0x32` |
| `strb` at 0xcd5d4 | store | 1 | `sp -0x22` | `0x5e` |
| `ldr` at 0xcd5dc | load | 4 | `abs 0xcd714` | `-` |
| `str` at 0xcd5e0 | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `ldr` at 0xcd5e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd5ec | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xcd5f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd604 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd608 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xcd60c | load | 1 | `sp -0x22` | `-` |
| `str` at 0xcd624 | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xcd628 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcd634 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcd638 | store | 4 | `sp -0x2c` | `ret(0xc7f7c)` |
| `ldrb` at 0xcd648 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xcd650 | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xcd654 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd658 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xcd65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd670 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd674 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xcd678 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xcd694 | store | 4 | `sp -0x28` | `ret(0xc80a0)` |
| `ldrb` at 0xcd698 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0xcd6a0 | store | 1 | `sp -0x23` | `?` |
| `ldrb` at 0xcd6a4 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xcd6a8 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0xcd6b0 | store | 1 | `sp -0x17` | `?` |
| `ldr` at 0xcd6b8 | load | 4 | `abs 0xcd718` | `-` |
| `str` at 0xcd6bc | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xcd6c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd6c8 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xcd6cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd6e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd6e4 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xcd6e8 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xcd6ec | load | 1 | `sp -0x17` | `-` |
| `str` at 0xcd704 | store | 4 | `sp -0x2c` | `ret(0xc7f7c)` |
| `ldr` at 0xcd708 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcd70c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd70c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd70c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd4b8 | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcd4d0 | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcd5f8 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xcd61c | `(unknown callee) 0xc7f7c` | `?` | `[[sp-28]+2]:1` | `[sp-34]:1` | `0xe2` |
| `bl` at 0xcd664 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xcd68c | `(unknown callee) 0xc80a0` | `?` | `[[sp-28]+2]:1` | `[sp-33]:1` | `sp-35` |
| `bl` at 0xcd6d4 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xcd6fc | `(unknown callee) 0xc7f7c` | `?` | `[[sp-28]+2]:1` | `[sp-33]:1` | `[sp-23]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd4e8 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `beq` at 0xcd50c | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0xcd518 | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcd524 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xcd534 | `[[sp-28]]:1 == 0x6` | `[[sp-28]]:1 != 0x6` |
| `beq` at 0xcd544 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xcd570 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0xcd57c | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xcd588 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xcd598 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0xcd630 | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd70c | `r0` | `?` |

## `monitorReadOutput`

- `monitorReadOutput`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd71c to 0xcd93c.
  Established. The call to `0xce118` carries entry r0, `(r1 & 0xff)`, entry r2 and `(r1 & 0xff)`
  (`bl` at 0xcd750). The branch takes `[ret(0xce238)]:1 != 0x0` and falls through on zero (`bne` at
  0xcd780). That fall-through path stores `0xfffffc11` at `sp -0x30` (`str` at 0xcd788). The output
  representation and final value are unknown (`ldmdb` at 0xcd92c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd72c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd730 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcd738 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd740 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xcd744 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd748 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd758 | store | 4 | `sp -0x20` | `ret(0xce118)` |
| `ldrb` at 0xcd75c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd760 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd770 | store | 4 | `sp -0x24` | `ret(0xce238)` |
| `ldr` at 0xcd774 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcd778 | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcd784 | load | 4 | `abs 0xcd930` | `-` |
| `str` at 0xcd788 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xcd790 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd794 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcd798 | store | 4 | `sp -0x38` | `[[sp-32]]:1` |
| `ldr` at 0xcd79c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcd7a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcd7b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcd7c4 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xcd7d4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd7e0 | load | 4 | `abs 0xcd934` | `-` |
| `str` at 0xcd7e4 | store | 4 | `sp -0x30` | `0xfffff82e` |
| `strb` at 0xcd7f0 | store | 1 | `sp -0x25` | `0x19` |
| `ldrb` at 0xcd7f8 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xcd7fc | store | 4 | `sp -0x34` | `[sp-22]:1` |
| `ldr` at 0xcd800 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd80c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd818 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd828 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xcd83c | store | 1 | `sp -0x25` | `0x30` |
| `strb` at 0xcd848 | store | 1 | `sp -0x25` | `0x31` |
| `strb` at 0xcd854 | store | 1 | `sp -0x25` | `0x32` |
| `ldr` at 0xcd85c | load | 4 | `abs 0xcd934` | `-` |
| `str` at 0xcd860 | store | 4 | `sp -0x30` | `0xfffff82e` |
| `ldrb` at 0xcd870 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xcd878 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0xcd87c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd880 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0xcd884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd898 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd89c | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xcd8a0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcd8ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd8b8 | store | 4 | `sp -0x2c` | `ret(0xc80a0)` |
| `ldr` at 0xcd8bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd8c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd8c4 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0xcd8cc | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xcd8d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcd8d4 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xcd8dc | load | 4 | `abs 0xcd938` | `-` |
| `str` at 0xcd8e0 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0xcd8e8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd8ec | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0xcd8f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd904 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd908 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xcd90c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcd918 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd924 | store | 4 | `sp -0x30` | `ret(0xc80a0)` |
| `ldr` at 0xcd928 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcd92c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd92c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd92c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd750 | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcd768 | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcd88c | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0xcd8b0 | `(unknown callee) 0xc80a0` | `?` | `[[sp-32]+2]:1` | `[sp-37]:1` | `[sp-28]` |
| `bl` at 0xcd8f8 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0xcd91c | `(unknown callee) 0xc80a0` | `?` | `[[sp-32]+2]:1` | `[sp-37]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd780 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `beq` at 0xcd7a4 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xcd7b0 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcd7bc | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xcd7cc | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |
| `beq` at 0xcd7dc | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xcd808 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0xcd814 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xcd820 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xcd830 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd92c | `r0` | `?` |

## `monitorIntCount`

- `monitorIntCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdb7c to 0xcdbc0.
  Established. The call to `0xce238` passes entry r0, `(r1 & 0xff)`, entry r2 and `(r1 & 0xff)`
  (`bl` at 0xcdba4). The return carries the word expression `[ret(0xce238)+4]` (`ldmdb` at 0xcdbbc);
  its use as an interrupt count is unknown from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdb8c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdb94 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xcdb98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdb9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdbac | store | 4 | `sp -0x1c` | `ret(0xce238)` |
| `ldr` at 0xcdbb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcdbb4 | load | 4 | `ret(0xce238) +0x4` | `-` |
| `ldmdb` at 0xcdbbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdbbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdbbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdba4 | `(unknown callee) 0xce238` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdbbc | `r0` | `[ret(0xce238)+4]` |

## `monitorIntDisable`

- `monitorIntDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdbc0 to 0xcdc08.
  Established. The only call targets `0xce830` with entry r0, `(r1 & 0xff)`, `(r2 & 0xff)` and `0x0`
  (`bl` at 0xcdbf8); its result reaches the return as `ret(0xce830)` (`ldmdb` at 0xcdc04). The
  interrupt-disable effect and selector meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdbd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdbd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcdbe0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xcdbe4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xcdbe8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcdbec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcdc04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdc04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdc04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdbf8 | `(unknown callee) 0xce830` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdc04 | `r0` | `ret(0xce830)` |

## `monitorIntEnable`

- `monitorIntEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdc08 to 0xcdc50.
  Established. The wrapper sends entry r0, `(r1 & 0xff)`, `(r2 & 0xff)` and `0x1` to `0xce830` (`bl`
  at 0xcdc40). The recorded return is `ret(0xce830)` (`ldmdb` at 0xcdc4c). Hardware enable polarity
  cannot be established from this call alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdc18 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdc20 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcdc28 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xcdc2c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xcdc30 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcdc34 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcdc4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdc4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdc4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdc40 | `(unknown callee) 0xce830` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdc4c | `r0` | `ret(0xce830)` |

## `monitorInt`

- `monitorInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdd1c to 0xcdf64.
  Established. A word store clears `sp -0x28` (`str` at 0xcdd44). The opening loop decision has
  unknown operands (`bls` at 0xcdd5c). The call to `0xce238` carries `[sp-20]`, `[sp-33]:1`, unknown
  and `[sp-33]:1` (`bl` at 0xcdd88). The later computed selector remains unresolved (`ldrls` at
  0xcddb4). Interrupt dispatch and accumulated results are unknown beyond known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdd2c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdd34 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcdd3c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0xcdd44 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xcdd48 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xcdd4c | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0xcdd50 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xcdd54 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xcdd64 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcdd68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdd78 | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldrb` at 0xcdd7c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xcdd80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdd90 | store | 4 | `sp -0x20` | `ret(0xce238)` |
| `ldr` at 0xcdd94 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcdd98 | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcdda4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcdda8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xcddb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcde74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcde7c | load | 4 | `abs 0xcdf50` | `-` |
| `ldr` at 0xcde80 | load | 4 | `abs 0xcdf54` | `-` |
| `str` at 0xcdeac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcdeb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdeb8 | load | 4 | `abs 0xcdf50` | `-` |
| `ldr` at 0xcdebc | load | 4 | `abs 0xcdf54` | `-` |
| `ldr` at 0xcdec8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcded8 | load | 4 | `abs 0xcdf58` | `-` |
| `str` at 0xcdedc | store | 4 | `(unknown)` | `0x133e68` |
| `str` at 0xcdee0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xcdee4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcdee8 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xcdeec | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0xcdef4 | load | 4 | `abs 0xcdf5c` | `-` |
| `ldr` at 0xcdef8 | load | 4 | `abs 0xcdf60` | `-` |
| `ldr` at 0xcdf04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcdf10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcdf14 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcdf1c | store | 4 | `sp -0x28` | `([sp-40] + [sp-44])` |
| `ldr` at 0xcdf20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdf24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdf28 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xcdf30 | store | 4 | `[sp-32] +0x4` | `[[sp-32]+4]+1` |
| `ldrb` at 0xcdf34 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xcdf3c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xcdf44 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcdf4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdf4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdf4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdd70 | `(unknown callee) 0xce118` | `?` | `?` | `?` | `?` |
| `bl` at 0xcdd88 | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xcde84 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcde90 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xcdec0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcded0 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xcdf00 | `(unknown callee) 0x21e28` | `0x9` | `0x133d7c` | `0x531` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcdd5c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xcdda0 | `[ret(0xce238)]:1 == 0x0` | `[ret(0xce238)]:1 != 0x0` |
| `ldrls` at 0xcddb4 | `[[sp-28]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xcdf0c | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdf4c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcddb4).
