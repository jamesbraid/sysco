# ARM display state and initialisation in 1.38.4

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
Accesses and return values cover known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/display-state-initialization-facts.txt) retain all tool sections.

## `displayProbe`

- `displayProbe`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51298 to 0x5144c.
  Established. The initial byte read uses absolute `0x1427a0` (`ldrb` at 0x512b4). Equality with
  `0xff` takes the branch and inequality falls through (`beq` at 0x512bc). The opening call to
  `0x21e28` has `0x9`, `0x10af4c`, `0x71` and `0x0` as candidates (`bl` at 0x512d8). Probe success
  and the final r0 value remain unknown (`ldmdb` at 0x51428).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x512ac | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x512b0 | load | 4 | `abs 0x5142c` | `-` |
| `ldrb` at 0x512b4 | load | 1 | `abs 0x1427a0` | `-` |
| `ldr` at 0x512c0 | load | 4 | `abs 0x51430` | `-` |
| `str` at 0x512c4 | store | 4 | `sp -0x28` | `0x10af34` |
| `ldr` at 0x512cc | load | 4 | `abs 0x51434` | `-` |
| `ldrb` at 0x512dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x512f4 | store | 4 | `sp -0x1c` | `ret(0xd6b9c)` |
| `ldr` at 0x512f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51310 | load | 4 | `abs 0x51438` | `-` |
| `str` at 0x51314 | store | 4 | `sp -0x28` | `0x10af64` |
| `str` at 0x51318 | store | 4 | `sp -0x24` | `ret(0x219a8)` |
| `ldr` at 0x51320 | load | 4 | `abs 0x51434` | `-` |
| `ldrb` at 0x51330 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x51344 | load | 4 | `abs 0x5142c` | `-` |
| `ldrb` at 0x51348 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5134c | store | 1 | `abs 0x1427a0` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x51350 | load | 4 | `abs 0x5143c` | `-` |
| `strb` at 0x51358 | store | 1 | `abs 0x1482a1` | `0x1` |
| `ldr` at 0x5135c | load | 4 | `abs 0x5142c` | `-` |
| `ldrb` at 0x51360 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51374 | store | 4 | `sp -0x18` | `ret(0xd75c0)` |
| `ldr` at 0x51378 | load | 4 | `abs 0x51440` | `-` |
| `ldr` at 0x5137c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x51380 | load | 1 | `ret(0xd75c0) +0x1` | `-` |
| `strb` at 0x51384 | store | 1 | `abs 0x1482a8` | `([ret(0xd75c0)+1]:1 & 0xff)` |
| `ldr` at 0x51388 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5138c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x51398 | load | 4 | `abs 0x51440` | `-` |
| `strb` at 0x513a0 | store | 1 | `abs 0x1482a9` | `0x76` |
| `ldr` at 0x513a4 | load | 4 | `abs 0x51444` | `-` |
| `ldr` at 0x513a8 | load | 4 | `abs 0x1429cc` | `-` |
| `ldr` at 0x513ac | load | 4 | `abs 0x51440` | `-` |
| `ldr` at 0x513b8 | load | 4 | `abs 0x51440` | `-` |
| `ldr` at 0x513bc | load | 4 | `abs 0x51440` | `-` |
| `ldrb` at 0x513c0 | load | 1 | `abs 0x1482ac` | `-` |
| `strb` at 0x513c8 | store | 1 | `abs 0x1482ac` | `?` |
| `ldr` at 0x513d0 | load | 4 | `abs 0x51440` | `-` |
| `strb` at 0x513d8 | store | 1 | `abs 0x1482a9` | `?` |
| `ldr` at 0x513dc | load | 4 | `abs 0x51440` | `-` |
| `ldr` at 0x513e4 | load | 4 | `abs 0x51440` | `-` |
| `ldrb` at 0x513e8 | load | 1 | `abs 0x1482aa` | `-` |
| `ldr` at 0x513f4 | load | 4 | `abs 0x51448` | `-` |
| `str` at 0x513f8 | store | 4 | `sp -0x28` | `0x10af90` |
| `ldr` at 0x513fc | load | 4 | `abs 0x51440` | `-` |
| `ldrb` at 0x51400 | load | 1 | `abs 0x1482aa` | `-` |
| `str` at 0x51404 | store | 4 | `sp -0x24` | `[0x1482aa]:1` |
| `ldr` at 0x5140c | load | 4 | `abs 0x51434` | `-` |
| `ldr` at 0x5141c | load | 4 | `abs 0x51440` | `-` |
| `strb` at 0x51424 | store | 1 | `abs 0x1482aa` | `0x10` |
| `ldmdb` at 0x51428 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51428 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51428 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x512d8 | `(unknown callee) 0x21e28` | `0x9` | `0x10af4c` | `0x71` | `0x0` |
| `bl` at 0x512ec | `(unknown callee) 0xd6b9c` | `0x0` | `?` | `sp-29` | `?` |
| `bl` at 0x51308 | `(unknown callee) 0x219a8` | `ret(0xd6b9c)` | `?` | `?` | `ret(0xd6b9c)` |
| `bl` at 0x5132c | `(unknown callee) 0x21e28` | `0x1` | `0x10af4c` | `0x77` | `0x0` |
| `bl` at 0x5133c | `(unknown callee) 0xd5f28` | `0x0` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0x5136c | `(unknown callee) 0xd75c0` | `0x0` | `[0x1427a0]:1` | `0x1482a1` | `[0x1427a0]:1` |
| `mov` at 0x513b4 | `(unknown callee) (unknown target)` | `0x1482a8` | `?` | `0x1482a8` | `[0x1429cc]` |
| `bl` at 0x513e0 | `(unknown callee) 0xdb12c` | `0x1482a8` | `?` | `?` | `0x1482a8` |
| `bl` at 0x51418 | `(unknown callee) 0x21e28` | `0x1` | `0x10af4c` | `0x94` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x512bc | `[0x1427a0]:1 == 0xff` | `[0x1427a0]:1 != 0xff` |
| `beq` at 0x51300 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `bne` at 0x51394 | `[[sp-24]]:1 != 0x2` | `[[sp-24]]:1 == 0x2` |
| `bls` at 0x513f0 | `[0x1482aa]:1 <= 0x10 (unsigned, 32-bit)` | `[0x1482aa]:1 > 0x10 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51428 | `r0` | `?` |

## `displayInit`

- `displayInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5144c to 0x51560.
  Established. A byte read accesses `0x1427a0` (`ldrb` at 0x51460); its value unequal to `0xff`
  takes the first branch and equality falls through (`bne` at 0x51468). The first recorded call to
  `0x21e28` has candidates `0x9`, `0x10af4c`, `0xab` and `0x0` (`bl` at 0x51498). Display initialisation
  semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5145c | load | 4 | `abs 0x51544` | `-` |
| `ldrb` at 0x51460 | load | 1 | `abs 0x1427a0` | `-` |
| `ldr` at 0x51470 | load | 4 | `abs 0x51548` | `-` |
| `ldrb` at 0x51474 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x51480 | load | 4 | `abs 0x5154c` | `-` |
| `str` at 0x51484 | store | 4 | `sp -0x1c` | `0x10afa8` |
| `ldr` at 0x5148c | load | 4 | `abs 0x51550` | `-` |
| `ldr` at 0x5149c | load | 4 | `abs 0x51548` | `-` |
| `strb` at 0x514a4 | store | 1 | `abs 0x1482a0` | `0x1` |
| `str` at 0x514b4 | store | 4 | `sp -0x14` | `ret(0x51db0)` |
| `ldr` at 0x514b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x514c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x514d0 | load | 4 | `abs 0x51554` | `-` |
| `str` at 0x514d4 | store | 4 | `sp -0x1c` | `0x10afc4` |
| `str` at 0x514d8 | store | 4 | `sp -0x18` | `ret(0x219a8)` |
| `ldr` at 0x514e0 | load | 4 | `abs 0x51550` | `-` |
| `ldr` at 0x514f4 | load | 4 | `abs 0x51558` | `-` |
| `str` at 0x51504 | store | 4 | `sp -0x14` | `ret(0x51580)` |
| `ldr` at 0x51508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51514 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51520 | load | 4 | `abs 0x5155c` | `-` |
| `str` at 0x51524 | store | 4 | `sp -0x1c` | `0x10afe8` |
| `str` at 0x51528 | store | 4 | `sp -0x18` | `ret(0x219a8)` |
| `ldr` at 0x51530 | load | 4 | `abs 0x51550` | `-` |
| `ldmdb` at 0x51540 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51540 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51540 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51498 | `(unknown callee) 0x21e28` | `0x9` | `0x10af4c` | `0xab` | `0x0` |
| `bl` at 0x514ac | `(unknown callee) 0x51db0` | `0x1` | `?` | `0x1482a0` | `0x1` |
| `bl` at 0x514c8 | `(unknown callee) 0x219a8` | `ret(0x51db0)` | `?` | `?` | `ret(0x51db0)` |
| `bl` at 0x514ec | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0xc6` | `0x0` |
| `bl` at 0x514fc | `(unknown callee) 0x51580` | `0x32` | `0x10afdc` | `0x0` | `?` |
| `bl` at 0x51518 | `(unknown callee) 0x219a8` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x5153c | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0xd6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51468 | `[0x1427a0]:1 != 0xff` | `[0x1427a0]:1 == 0xff` |
| `beq` at 0x5147c | `[0x1482a0]:1 == 0x0` | `[0x1482a0]:1 != 0x0` |
| `beq` at 0x514c0 | `ret(0x51db0) == 0x0` | `ret(0x51db0) != 0x0` |
| `beq` at 0x51510 | `ret(0x51580) == 0x0` | `ret(0x51580) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51540 | `r0` | `?` |

## `displayPresent`

- `displayPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51560 to 0x51580.
  Established. The absolute byte read fetches `0x1482a1` (`ldrb` at 0x51570); its value reaches the
  return as `[0x1482a1]:1` (`ldmdb` at 0x51578). The byte's meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5156c | load | 4 | `abs 0x5157c` | `-` |
| `ldrb` at 0x51570 | load | 1 | `abs 0x1482a1` | `-` |
| `ldmdb` at 0x51578 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51578 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51578 | `r0` | `[0x1482a1]:1` |

## `displayString`

- `displayString`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51580 to 0x517fc.
  Established. The first decision takes `[0x1482a0]:1 != 0x0` and falls through on zero (`bne` at
  0x515a8). The call to `0x1b508` has candidates `0x1427a4`, entry r1, entry r2 and entry r2 (`bl` at
  0x51600). The pointee meanings and joined r0 result remain unknown (`ldmdb` at 0x517d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51590 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x51594 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x51598 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x5159c | load | 4 | `abs 0x517dc` | `-` |
| `ldrb` at 0x515a0 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x515ac | load | 4 | `abs 0x517e0` | `-` |
| `str` at 0x515b0 | store | 4 | `sp -0x30` | `0xfffff8f7` |
| `ldr` at 0x515b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x515c4 | load | 4 | `abs 0x517e4` | `-` |
| `str` at 0x515c8 | store | 4 | `sp -0x20` | `0x1482c8` |
| `ldr` at 0x515cc | load | 4 | `abs 0x517e8` | `-` |
| `ldrb` at 0x515d0 | load | 1 | `abs 0x1482aa` | `-` |
| `str` at 0x515d4 | store | 4 | `sp -0x24` | `[0x1482aa]:1` |
| `ldr` at 0x515dc | load | 4 | `abs 0x517ec` | `-` |
| `ldrb` at 0x515e0 | load | 1 | `abs 0x1482a2` | `-` |
| `ldr` at 0x515ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x515f8 | load | 4 | `abs 0x517f0` | `-` |
| `ldr` at 0x515fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51608 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x51610 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51620 | load | 4 | `abs 0x517f4` | `-` |
| `str` at 0x51624 | store | 4 | `sp -0x20` | `0x1482d9` |
| `ldr` at 0x51628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x51638 | load | 4 | `abs 0x517e8` | `-` |
| `ldrb` at 0x5163c | load | 1 | `abs 0x1482aa` | `-` |
| `str` at 0x5164c | store | 4 | `sp -0x24` | `0x48` |
| `ldr` at 0x51654 | load | 4 | `abs 0x517e8` | `-` |
| `ldrb` at 0x51658 | load | 1 | `abs 0x1482aa` | `-` |
| `str` at 0x5165c | store | 4 | `sp -0x24` | `[0x1482aa]:1` |
| `ldr` at 0x51660 | load | 4 | `abs 0x517f0` | `-` |
| `ldr` at 0x51664 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51670 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x51674 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x51678 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51688 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5168c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51698 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5169c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x516a4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x516a8 | load | 4 | `abs 0x517e8` | `-` |
| `ldrb` at 0x516ac | load | 1 | `abs 0x1482aa` | `-` |
| `ldr` at 0x516b0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x516c0 | store | 1 | `sp -0x35` | `0x20` |
| `strb` at 0x516cc | store | 1 | `sp -0x35` | `0x0` |
| `ldrb` at 0x516d0 | load | 1 | `sp -0x35` | `-` |
| `ldr` at 0x516d4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x516d8 | store | 1 | `(unknown)` | `([sp-53]:1 & 0xff)` |
| `ldr` at 0x516e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x516e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x516ec | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x516fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x51700 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x51708 | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x51710 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51720 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x51724 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x51728 | store | 1 | `sp -0x3d` | `([[sp-24]]:1 & 0xff)` |
| `strb` at 0x51734 | store | 1 | `sp -0x3d` | `0x3f` |
| `ldrb` at 0x51738 | load | 1 | `sp -0x3d` | `-` |
| `ldr` at 0x5173c | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x51740 | store | 1 | `[sp-60] +0x0` | `?` |
| `ldr` at 0x51744 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5174c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x51750 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x51758 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x51760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51764 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x51770 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x51774 | load | 4 | `abs 0x517e8` | `-` |
| `ldr` at 0x51778 | load | 4 | `abs 0x517e8` | `-` |
| `ldr` at 0x5177c | load | 4 | `abs 0x1482c0` | `-` |
| `ldr` at 0x51780 | load | 4 | `abs 0x1482b0` | `-` |
| `ldr` at 0x51784 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51788 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51794 | load | 4 | `abs 0x517f8` | `-` |
| `ldrb` at 0x51798 | load | 1 | `abs 0x148322` | `-` |
| `ldr` at 0x517a4 | load | 4 | `abs 0x517e8` | `-` |
| `ldr` at 0x517a8 | load | 4 | `abs 0x1482b8` | `-` |
| `ldr` at 0x517ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x517b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x517bc | store | 4 | `sp -0x44` | `ret(unknown)` |
| `str` at 0x517c8 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x517cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x517d0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x517d4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x517d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x517d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x517d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51600 | `(unknown callee) 0x1b508` | `0x1427a4` | `r1` | `r2` | `r2` |
| `bl` at 0x5161c | `(unknown callee) 0x51a4c` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x5162c | `(unknown callee) 0x1b22c` | `[sp-24]` | `?` | `?` | `0x1482d9` |
| `bl` at 0x51668 | `(unknown callee) 0x1b508` | `0x1427a4` | `[sp-24]` | `ret(0x1b22c)` | `?` |
| `bl` at 0x516f0 | `(unknown callee) 0x1b4cc` | `?` | `?` | `?` | `?` |
| `mov` at 0x51790 | `(unknown callee) (unknown target)` | `[0x1482b0]` | `[sp-32]` | `[sp-36]` | `[0x1482c0]` |
| `mov` at 0x517b8 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-36]` | `?` | `[0x1482b8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x515a8 | `[0x1482a0]:1 != 0x0` | `[0x1482a0]:1 == 0x0` |
| `bne` at 0x515c0 | `r0 != 0x31` | `r0 == 0x31` |
| `beq` at 0x515e8 | `[0x1482a2]:1 == 0x0` | `[0x1482a2]:1 != 0x0` |
| `bne` at 0x515f4 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0x51618 | `r2 != 0x2` | `r2 == 0x2` |
| `bls` at 0x51644 | `ret(0x1b22c) <= [0x1482aa]:1 (unsigned, 32-bit)` | `ret(0x1b22c) > [0x1482aa]:1 (unsigned, 32-bit)` |
| `blt` at 0x51680 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x51694 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x516b8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x5171c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x517a0 | `[0x148322]:1 != 0x0` | `[0x148322]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x517d8 | `r0` | `?` |

## `displayChar`

- `displayChar`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x517fc to 0x51968.
  Established. Two byte saves retain `(r1 & 0xff)` and `(r2 & 0xff)` at `sp -0x15` and `sp -0x16`
  (`strb` at 0x51814, `strb` at 0x5181c). The branch takes `[0x1482a0]:1 != 0x0` and falls through
  on zero (`bne` at 0x5182c). The call to `0x1b4cc` has candidates entry r0, entry r1, `(r1 & 0xff)` and
  `[0x1482aa]:1` (`bl` at 0x51878). The byte encoding is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5180c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51814 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x5181c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x51820 | load | 4 | `abs 0x51950` | `-` |
| `ldrb` at 0x51824 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x51830 | load | 4 | `abs 0x51954` | `-` |
| `str` at 0x51834 | store | 4 | `sp -0x1c` | `0xfffff8f7` |
| `ldr` at 0x5183c | load | 4 | `abs 0x51958` | `-` |
| `ldr` at 0x51840 | load | 4 | `abs 0x1482bc` | `-` |
| `str` at 0x51850 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x51858 | load | 4 | `abs 0x51958` | `-` |
| `ldrb` at 0x5185c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51860 | load | 1 | `abs 0x1482aa` | `-` |
| `str` at 0x51870 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x51884 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5188c | load | 4 | `ret(0x1b4cc) +0x0` | `-` |
| `ldrh` at 0x51894 | load | 2 | `(unknown)` | `-` |
| `strb` at 0x518a8 | store | 1 | `sp -0x16` | `0x3f` |
| `ldr` at 0x518ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x518b8 | load | 4 | `abs 0x5195c` | `-` |
| `ldrb` at 0x518bc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x518c0 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x518c4 | store | 1 | `[sp-21]:1 +0x1482c8` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x518cc | load | 4 | `abs 0x51960` | `-` |
| `ldrb` at 0x518d0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x518d4 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x518d8 | store | 1 | `[sp-21]:1 +0x1482d9` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x518dc | load | 4 | `abs 0x51958` | `-` |
| `ldr` at 0x518e8 | load | 4 | `abs 0x1482c0` | `-` |
| `ldr` at 0x518fc | load | 4 | `abs 0x51964` | `-` |
| `ldrb` at 0x51900 | load | 1 | `abs 0x148322` | `-` |
| `ldr` at 0x5190c | load | 4 | `abs 0x51958` | `-` |
| `ldrb` at 0x51910 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51914 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x51918 | load | 4 | `abs 0x1482bc` | `-` |
| `ldr` at 0x5191c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51930 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `str` at 0x5193c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x51940 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x51944 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51948 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x5194c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5194c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5194c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51878 | `(unknown callee) 0x1b4cc` | `r0` | `r1` | `(r1 & 0xff)` | `[0x1482aa]:1` |
| `mov` at 0x518f8 | `(unknown callee) (unknown target)` | `sp-23` | `sp-22` | `0x1` | `[0x1482c0]` |
| `mov` at 0x5192c | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-21]:1` | `[sp-23]:1` | `[0x1482bc]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5182c | `[0x1482a0]:1 != 0x0` | `[0x1482a0]:1 == 0x0` |
| `bne` at 0x51848 | `[0x1482bc] != 0x0` | `[0x1482bc] == 0x0` |
| `blo` at 0x51868 | `(r1 & 0xff) < [0x1482aa]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x1482aa]:1 (unsigned, 32-bit)` |
| `bne` at 0x518a0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x518b4 | `[sp-20] != 0x31` | `[sp-20] == 0x31` |
| `bne` at 0x51908 | `[0x148322]:1 != 0x0` | `[0x148322]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5194c | `r0` | `?` |

## `displayHeatbeat`

- `displayHeatbeat`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51968 to 0x51a4c.
  Established. A word load reads absolute `0x1482bc` (`ldr` at 0x5197c); its zero value takes the
  branch and nonzero falls through (`beq` at 0x51984). The indirect call candidates are `0x31`
  followed by three unknowns (`mov` at 0x51a20). The update interval, animation and hardware effect
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x51978 | load | 4 | `abs 0x51a34` | `-` |
| `ldr` at 0x5197c | load | 4 | `abs 0x1482bc` | `-` |
| `ldr` at 0x51988 | load | 4 | `abs 0x51a38` | `-` |
| `ldrb` at 0x5198c | load | 1 | `abs 0x148322` | `-` |
| `str` at 0x519a0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x519a8 | load | 4 | `abs 0x51a3c` | `-` |
| `ldr` at 0x519ac | load | 4 | `abs 0x51a34` | `-` |
| `ldrb` at 0x519b0 | load | 1 | `abs 0x1482aa` | `-` |
| `ldr` at 0x519c0 | load | 4 | `abs 0x51a40` | `-` |
| `ldr` at 0x519c4 | load | 4 | `abs 0x51a44` | `-` |
| `ldrb` at 0x519c8 | load | 1 | `abs 0x1482a3` | `-` |
| `ldrb` at 0x519d0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x519d4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x519d8 | load | 4 | `abs 0x51a34` | `-` |
| `ldr` at 0x519dc | load | 4 | `abs 0x51a34` | `-` |
| `ldrb` at 0x519e0 | load | 1 | `abs 0x1482aa` | `-` |
| `ldr` at 0x519ec | load | 4 | `abs 0x51a48` | `-` |
| `ldr` at 0x519f0 | load | 4 | `abs 0x51a44` | `-` |
| `ldrb` at 0x519f4 | load | 1 | `abs 0x1482a3` | `-` |
| `strb` at 0x519fc | store | 1 | `abs 0x1482a3` | `([0x1482a3]:1+1 & 0xff)` |
| `ldrb` at 0x51a08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x51a0c | load | 4 | `abs 0x1482bc` | `-` |
| `str` at 0x51a28 | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0x51a2c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x51a30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51a30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51a30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x51a20 | `(unknown callee) (unknown target)` | `0x31` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x51984 | `[0x1482bc] == 0x0` | `[0x1482bc] != 0x0` |
| `bne` at 0x51994 | `[0x148322]:1 != 0x0` | `[0x148322]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51a30 | `r0` | `?` |

## `displayLock`

- `displayLock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51a4c to 0x51a6c.
  Established. The byte store sets absolute `0x1482a2` to `0x1` (`strb` at 0x51a60). The return
  preserves entry r0 (`ldmdb` at 0x51a64). The byte's meaning and external synchronisation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x51a58 | load | 4 | `abs 0x51a68` | `-` |
| `strb` at 0x51a60 | store | 1 | `abs 0x1482a2` | `0x1` |
| `ldmdb` at 0x51a64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51a64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51a64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51a64 | `r0` | `r0` |

## `displayUnlock`

- `displayUnlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51a6c to 0x51aa0.
  Established. The byte store clears absolute `0x1482a2` (`strb` at 0x51a80). The call to `0x51580`
  has candidates `0x32`, `0x1427a4`, `0x0` and `0x1482a2` (`bl` at 0x51a90); its result reaches the
  recorded exit (`ldmdb` at 0x51a94). The store's and call's external effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x51a78 | load | 4 | `abs 0x51a98` | `-` |
| `strb` at 0x51a80 | store | 1 | `abs 0x1482a2` | `0x0` |
| `ldr` at 0x51a88 | load | 4 | `abs 0x51a9c` | `-` |
| `ldmdb` at 0x51a94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51a94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51a94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51a90 | `(unknown callee) 0x51580` | `0x32` | `0x1427a4` | `0x0` | `0x1482a2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51a94 | `r0` | `ret(0x51580)` |

## `displayStatus`

- `displayStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51fbc to 0x520b8.
  Established. The first branch takes `[0x1482a0]:1 != 0x0` and falls through on zero (`bne` at
  0x51fdc). The fall-through word store writes `0xfffff8f7` at `sp -0x18` (`str` at 0x51fe4). The
  call to `0xd6b9c` has candidates `0x0`, `[0x1427a0]:1`, `[sp-20]+16` and `[sp-20]+16` (`bl` at 0x52088).
  The status record meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51fcc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x51fd0 | load | 4 | `abs 0x5209c` | `-` |
| `ldrb` at 0x51fd4 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x51fe0 | load | 4 | `abs 0x520a0` | `-` |
| `str` at 0x51fe4 | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x51fec | load | 4 | `abs 0x520a4` | `-` |
| `ldr` at 0x51ff0 | load | 4 | `abs 0x520a8` | `-` |
| `ldrb` at 0x51ff4 | load | 1 | `abs 0x1482ab` | `-` |
| `strb` at 0x52004 | store | 1 | `abs 0x1482a4` | `?` |
| `ldr` at 0x52008 | load | 4 | `abs 0x520a4` | `-` |
| `strb` at 0x52010 | store | 1 | `abs 0x1482a5` | `0x2e` |
| `ldr` at 0x52014 | load | 4 | `abs 0x520a4` | `-` |
| `ldr` at 0x52018 | load | 4 | `abs 0x520a8` | `-` |
| `ldrb` at 0x5201c | load | 1 | `abs 0x1482ab` | `-` |
| `strb` at 0x52028 | store | 1 | `abs 0x1482a6` | `?` |
| `ldr` at 0x5202c | load | 4 | `abs 0x520a4` | `-` |
| `strb` at 0x52034 | store | 1 | `abs 0x1482a7` | `0x0` |
| `ldr` at 0x52038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5203c | load | 4 | `abs 0x520ac` | `-` |
| `str` at 0x52040 | store | 4 | `[sp-20] +0x0` | `0x1482c8` |
| `ldr` at 0x52044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52048 | load | 4 | `abs 0x520b0` | `-` |
| `str` at 0x5204c | store | 4 | `[sp-20] +0x4` | `0x1482d9` |
| `ldr` at 0x52050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52054 | load | 4 | `abs 0x520a4` | `-` |
| `str` at 0x52058 | store | 4 | `[sp-20] +0x8` | `0x1482a4` |
| `ldr` at 0x5205c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52060 | load | 4 | `abs 0x520a8` | `-` |
| `ldrb` at 0x52064 | load | 1 | `abs 0x1482ab` | `-` |
| `strb` at 0x52068 | store | 1 | `[sp-20] +0xc` | `([0x1482ab]:1 & 0xff)` |
| `ldr` at 0x5206c | load | 4 | `abs 0x520b4` | `-` |
| `ldrb` at 0x52070 | load | 1 | `abs 0x1427a0` | `-` |
| `ldr` at 0x52074 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52090 | store | 4 | `sp -0x18` | `ret(0xd6b9c)` |
| `ldr` at 0x52094 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52098 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52098 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52098 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52088 | `(unknown callee) 0xd6b9c` | `0x0` | `[0x1427a0]:1` | `[sp-20]+16` | `[sp-20]+16` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51fdc | `[0x1482a0]:1 != 0x0` | `[0x1482a0]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52098 | `r0` | `?` |

## `displayInt`

- `displayInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x520b8 to 0x526e8.
  Established. The opening byte stores save `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at `sp
  -0x15`, `sp -0x16` and `sp -0x17` (`strb` at 0x520d4, `strb` at 0x520dc, `strb` at 0x520e4). The
  branch takes `(r1 & 0xff) == [0x1427a0]:1` and falls through on inequality (`beq` at 0x520f8). The
  opening call to `0x21e28` has candidates `0x9`, `0x10af4c`, `0x29e` and `0x0` (`bl` at 0x5211c). The input bytes'
  meanings and hardware actions remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x520c8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x520d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x520dc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x520e4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x520e8 | load | 4 | `abs 0x5266c` | `-` |
| `ldrb` at 0x520ec | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x520f0 | load | 1 | `abs 0x1427a0` | `-` |
| `ldr` at 0x520fc | load | 4 | `abs 0x52670` | `-` |
| `str` at 0x52100 | store | 4 | `sp -0x24` | `0x10b030` |
| `ldrb` at 0x52104 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x52108 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0x52110 | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52114 | load | 4 | `abs 0x52678` | `-` |
| `ldr` at 0x52120 | load | 4 | `abs 0x5267c` | `-` |
| `ldrb` at 0x52124 | load | 1 | `abs 0x1482ac` | `-` |
| `ldrb` at 0x52134 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x5213c | store | 1 | `sp -0x17` | `?` |
| `ldrb` at 0x52140 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x52150 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x52184 | load | 4 | `abs 0x52680` | `-` |
| `str` at 0x52188 | store | 4 | `sp -0x24` | `0x10b058` |
| `ldr` at 0x52190 | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52194 | load | 4 | `abs 0x52684` | `-` |
| `ldr` at 0x521a4 | load | 4 | `abs 0x52688` | `-` |
| `str` at 0x521c0 | store | 4 | `sp -0x1c` | `ret(0x3d48c)` |
| `ldr` at 0x521c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x521d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x521dc | load | 4 | `abs 0x5268c` | `-` |
| `str` at 0x521e0 | store | 4 | `sp -0x24` | `0x10b0a8` |
| `str` at 0x521e4 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x521ec | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x521f0 | load | 4 | `abs 0x52690` | `-` |
| `str` at 0x52210 | store | 4 | `sp -0x1c` | `ret(0x3cc04)` |
| `ldr` at 0x52214 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5222c | load | 4 | `abs 0x52694` | `-` |
| `str` at 0x52230 | store | 4 | `sp -0x24` | `0x10b0c0` |
| `str` at 0x52234 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x5223c | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52240 | load | 4 | `abs 0x52698` | `-` |
| `ldrb` at 0x5224c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5225c | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x52400 | load | 4 | `abs 0x5269c` | `-` |
| `str` at 0x52404 | store | 4 | `sp -0x24` | `0x10b0d4` |
| `ldr` at 0x5240c | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52410 | load | 4 | `abs 0x526a0` | `-` |
| `ldr` at 0x52420 | load | 4 | `abs 0x526a4` | `-` |
| `ldr` at 0x52434 | load | 4 | `abs 0x526a8` | `-` |
| `str` at 0x52444 | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x52448 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52454 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52460 | load | 4 | `abs 0x526ac` | `-` |
| `str` at 0x52464 | store | 4 | `sp -0x24` | `0x10afe8` |
| `str` at 0x52468 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x52470 | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52474 | load | 4 | `abs 0x526b0` | `-` |
| `str` at 0x52490 | store | 4 | `sp -0x1c` | `ret(0x4e4d8)` |
| `ldr` at 0x52494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x524a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x524ac | load | 4 | `abs 0x526b4` | `-` |
| `str` at 0x524b0 | store | 4 | `sp -0x24` | `0x10b124` |
| `str` at 0x524b4 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x524bc | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x524c0 | load | 4 | `abs 0x526b8` | `-` |
| `ldr` at 0x524d0 | load | 4 | `abs 0x526bc` | `-` |
| `str` at 0x524e0 | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x524e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x524f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x524fc | load | 4 | `abs 0x526ac` | `-` |
| `str` at 0x52500 | store | 4 | `sp -0x24` | `0x10afe8` |
| `str` at 0x52504 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x5250c | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52510 | load | 4 | `abs 0x526c0` | `-` |
| `ldrb` at 0x5251c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5252c | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x52550 | load | 4 | `abs 0x526c4` | `-` |
| `str` at 0x52554 | store | 4 | `sp -0x24` | `0x10b140` |
| `ldr` at 0x5255c | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x52560 | load | 4 | `abs 0x526c8` | `-` |
| `ldr` at 0x52570 | load | 4 | `abs 0x526cc` | `-` |
| `ldr` at 0x52584 | load | 4 | `abs 0x526d0` | `-` |
| `str` at 0x52594 | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x52598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x525a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x525b0 | load | 4 | `abs 0x526ac` | `-` |
| `str` at 0x525b4 | store | 4 | `sp -0x24` | `0x10afe8` |
| `str` at 0x525b8 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x525c0 | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x525c4 | load | 4 | `abs 0x526d4` | `-` |
| `str` at 0x525dc | store | 4 | `sp -0x1c` | `ret(0x4e10c)` |
| `ldr` at 0x525e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x525ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x525f8 | load | 4 | `abs 0x526d8` | `-` |
| `str` at 0x525fc | store | 4 | `sp -0x24` | `0x10b198` |
| `str` at 0x52600 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x52608 | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x5260c | load | 4 | `abs 0x526dc` | `-` |
| `ldr` at 0x5261c | load | 4 | `abs 0x526e0` | `-` |
| `str` at 0x5262c | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x52630 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5263c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52648 | load | 4 | `abs 0x526ac` | `-` |
| `str` at 0x5264c | store | 4 | `sp -0x24` | `0x10afe8` |
| `str` at 0x52650 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x52658 | load | 4 | `abs 0x52674` | `-` |
| `ldr` at 0x5265c | load | 4 | `abs 0x526e4` | `-` |
| `ldmdb` at 0x52668 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52668 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52668 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5211c | `(unknown callee) 0x21e28` | `0x9` | `0x10af4c` | `0x29e` | `0x0` |
| `bl` at 0x52160 | `(unknown callee) 0x3d8e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x52174 | `(unknown callee) 0x1f1dc` | `0x77` | `?` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x5219c | `(unknown callee) 0x21e28` | `0x6` | `0x10af4c` | `0x2be` | `0x0` |
| `bl` at 0x521ac | `(unknown callee) 0x51580` | `0x32` | `0x10b094` | `0x1` | `?` |
| `bl` at 0x521b8 | `(unknown callee) 0x3d48c` | `0x2` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x521d4 | `(unknown callee) 0x219a8` | `ret(0x3d48c)` | `?` | `?` | `ret(0x3d48c)` |
| `bl` at 0x521f8 | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2c3` | `0x0` |
| `bl` at 0x52208 | `(unknown callee) 0x3cc04` | `0x1` | `0x2` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x52224 | `(unknown callee) 0x219a8` | `ret(0x3cc04)` | `?` | `?` | `ret(0x3cc04)` |
| `bl` at 0x52248 | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2c7` | `0x0` |
| `bl` at 0x5226c | `(unknown callee) 0x4f090` | `?` | `?` | `?` | `?` |
| `bl` at 0x52280 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52294 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x522a8 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x522bc | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x522d0 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x522e4 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x522f8 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x5230c | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52320 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52334 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52348 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x5235c | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52370 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52384 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x52398 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x523ac | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x523c0 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x523d4 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x523f0 | `(unknown callee) 0x1f1dc` | `0x78` | `?` | `?` | `?` |
| `bl` at 0x52418 | `(unknown callee) 0x21e28` | `0x6` | `0x10af4c` | `0x2d3` | `0x0` |
| `bl` at 0x52428 | `(unknown callee) 0x51580` | `0x32` | `0x10b108` | `0x1` | `?` |
| `bl` at 0x5243c | `(unknown callee) 0x51580` | `0x32` | `0x10b11c` | `0x1` | `ret(0x1f1dc)` |
| `bl` at 0x52458 | `(unknown callee) 0x219a8` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x5247c | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2d7` | `0x0` |
| `bl` at 0x52488 | `(unknown callee) 0x4e4d8` | `0x2` | `0xff` | `?` | `?` |
| `bl` at 0x524a4 | `(unknown callee) 0x219a8` | `ret(0x4e4d8)` | `?` | `?` | `ret(0x4e4d8)` |
| `bl` at 0x524c8 | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2db` | `0x0` |
| `bl` at 0x524d8 | `(unknown callee) 0x51580` | `0x32` | `0x10b134` | `0x1` | `?` |
| `bl` at 0x524f4 | `(unknown callee) 0x219a8` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x52518 | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2de` | `0x0` |
| `bl` at 0x52540 | `(unknown callee) 0x1f1dc` | `0x79` | `?` | `?` | `?` |
| `bl` at 0x52568 | `(unknown callee) 0x21e28` | `0x6` | `0x10af4c` | `0x2e9` | `0x0` |
| `bl` at 0x52578 | `(unknown callee) 0x51580` | `0x32` | `0x10b178` | `0x1` | `?` |
| `bl` at 0x5258c | `(unknown callee) 0x51580` | `0x32` | `0x10b18c` | `0x1` | `ret(0x1f1dc)` |
| `bl` at 0x525a8 | `(unknown callee) 0x219a8` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x525cc | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2ee` | `0x0` |
| `bl` at 0x525d4 | `(unknown callee) 0x4e10c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x525f0 | `(unknown callee) 0x219a8` | `ret(0x4e10c)` | `?` | `?` | `ret(0x4e10c)` |
| `bl` at 0x52614 | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2f2` | `0x0` |
| `bl` at 0x52624 | `(unknown callee) 0x51580` | `0x32` | `0x10b1a8` | `0x1` | `?` |
| `bl` at 0x52640 | `(unknown callee) 0x219a8` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x52664 | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x2f5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x520f8 | `(r1 & 0xff) == [0x1427a0]:1` | `(r1 & 0xff) != [0x1427a0]:1` |
| `beq` at 0x52130 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5214c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5215c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5216c | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `beq` at 0x52180 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x521cc | `ret(0x3d48c) == 0x0` | `ret(0x3d48c) != 0x0` |
| `beq` at 0x5221c | `ret(0x3cc04) == 0x0` | `ret(0x3cc04) != 0x0` |
| `beq` at 0x52258 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52268 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5227c | `? == 0x43` | `? != 0x43` |
| `beq` at 0x52290 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x522a4 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x522b8 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x522cc | `? == 0x41` | `? != 0x41` |
| `beq` at 0x522e0 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x522f4 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x52308 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x5231c | `? == 0x38` | `? != 0x38` |
| `beq` at 0x52330 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x52344 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x52358 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x5236c | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x52380 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x52394 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x523a8 | `? == 0x48` | `? != 0x48` |
| `beq` at 0x523bc | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x523d0 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x523e4 | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x523fc | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x52450 | `ret(0x51580) == 0x0` | `ret(0x51580) != 0x0` |
| `beq` at 0x5249c | `ret(0x4e4d8) == 0x0` | `ret(0x4e4d8) != 0x0` |
| `beq` at 0x524ec | `ret(0x51580) == 0x0` | `ret(0x51580) != 0x0` |
| `beq` at 0x52528 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52538 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5254c | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x525a0 | `ret(0x51580) == 0x0` | `ret(0x51580) != 0x0` |
| `beq` at 0x525e8 | `ret(0x4e10c) == 0x0` | `ret(0x4e10c) != 0x0` |
| `beq` at 0x52638 | `ret(0x51580) == 0x0` | `ret(0x51580) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52668 | `r0` | `?` |

## `pcf2119_Init_stub`

- `pcf2119_Init_stub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc84e0 to 0xc84f8.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0xc84f0). The recorded return
  preserves entry r0 (`ldmdb` at 0xc84f4). The stack save does not establish a
  display-initialisation operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc84f0 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xc84f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc84f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc84f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc84f4 | `r0` | `r0` |

## `pcf2119_Init_f`

- `pcf2119_Init_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda548 to 0xda5f0.
  Established. Byte stores write `0x10` at `r0 +0x2`, `0x30` at `[sp-20] +0x3` and `0x1` at
  `[sp-20] +0x4` (`strb` at 0xda564, `strb` at 0xda570, `strb` at 0xda57c). Another byte store has unknown
  value at `[sp-20] +0x5` (`strb` at 0xda588). The byte-field meanings and controller configuration
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda558 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xda55c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xda564 | store | 1 | `r0 +0x2` | `0x10` |
| `ldr` at 0xda568 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xda570 | store | 1 | `[sp-20] +0x3` | `0x30` |
| `ldr` at 0xda574 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xda57c | store | 1 | `[sp-20] +0x4` | `0x1` |
| `ldr` at 0xda580 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xda588 | store | 1 | `[sp-20] +0x5` | `?` |
| `ldr` at 0xda58c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda590 | load | 4 | `abs 0xda5d8` | `-` |
| `str` at 0xda594 | store | 4 | `[sp-20] +0x8` | `0x2b36eb` |
| `ldr` at 0xda598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda59c | load | 4 | `abs 0xda5dc` | `-` |
| `str` at 0xda5a0 | store | 4 | `[sp-20] +0xc` | `0xda614` |
| `ldr` at 0xda5a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda5a8 | load | 4 | `abs 0xda5e0` | `-` |
| `str` at 0xda5ac | store | 4 | `[sp-20] +0x10` | `0xda814` |
| `ldr` at 0xda5b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda5b4 | load | 4 | `abs 0xda5e4` | `-` |
| `str` at 0xda5b8 | store | 4 | `[sp-20] +0x14` | `0xda894` |
| `ldr` at 0xda5bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda5c0 | load | 4 | `abs 0xda5e8` | `-` |
| `str` at 0xda5c4 | store | 4 | `[sp-20] +0x18` | `0xda930` |
| `ldr` at 0xda5c8 | load | 4 | `abs 0xda5ec` | `-` |
| `ldr` at 0xda5cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda5d0 | store | 4 | `abs 0x2b36e0` | `[sp-20]` |
| `ldmdb` at 0xda5d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda5d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda5d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda5d4 | `r0` | `r0` |

## `pcf2119_Setup`

- `pcf2119_Setup`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda5f0 to 0xda614.
  Established. One word store installs `0xda548` at absolute `0x1429cc` (`str` at 0xda604). Entry r0
  survives to the exit (`ldmdb` at 0xda608). This body does not establish the installed
  value's role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xda5fc | load | 4 | `abs 0xda60c` | `-` |
| `ldr` at 0xda600 | load | 4 | `abs 0xda610` | `-` |
| `str` at 0xda604 | store | 4 | `abs 0x1429cc` | `0xda548` |
| `ldmdb` at 0xda608 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda608 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda608 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda608 | `r0` | `r0` |

## `t1_displayInit`

- `t1_displayInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb12c to 0xdb1d0.
  Established. Byte stores supply `0xc` at `r0 +0x2`, `0x0` at `[sp-20] +0x3`, `0x0` at
  `[sp-20] +0x4` and `0x20` at `[sp-20] +0x5` (`strb` at 0xdb148, `strb` at 0xdb154, `strb` at 0xdb160,
  `strb` at 0xdb16c). The return carries entry r0 (`ldmdb` at 0xdb1b8). Their display-controller
  interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb13c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdb140 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdb148 | store | 1 | `r0 +0x2` | `0xc` |
| `ldr` at 0xdb14c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdb154 | store | 1 | `[sp-20] +0x3` | `0x0` |
| `ldr` at 0xdb158 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdb160 | store | 1 | `[sp-20] +0x4` | `0x0` |
| `ldr` at 0xdb164 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdb16c | store | 1 | `[sp-20] +0x5` | `0x20` |
| `ldr` at 0xdb170 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb174 | load | 4 | `abs 0xdb1bc` | `-` |
| `str` at 0xdb178 | store | 4 | `[sp-20] +0x8` | `0x2b3749` |
| `ldr` at 0xdb17c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb180 | load | 4 | `abs 0xdb1c0` | `-` |
| `str` at 0xdb184 | store | 4 | `[sp-20] +0xc` | `0xdb1d0` |
| `ldr` at 0xdb188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb18c | load | 4 | `abs 0xdb1c4` | `-` |
| `str` at 0xdb190 | store | 4 | `[sp-20] +0x10` | `0xdb34c` |
| `ldr` at 0xdb194 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb19c | store | 4 | `[sp-20] +0x14` | `0x0` |
| `ldr` at 0xdb1a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb1a4 | load | 4 | `abs 0xdb1c8` | `-` |
| `str` at 0xdb1a8 | store | 4 | `[sp-20] +0x18` | `0xdb508` |
| `ldr` at 0xdb1ac | load | 4 | `abs 0xdb1cc` | `-` |
| `ldr` at 0xdb1b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb1b4 | store | 4 | `abs 0x2ad238` | `[sp-20]` |
| `ldmdb` at 0xdb1b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb1b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb1b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb1b8 | `r0` | `r0` |

## `IP65_Init`

- `IP65_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63db8 to 0x63e38.
  Established. The opening call to `0x5dec8` has candidates `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0x63de0); its zero result takes the branch and nonzero falls through (`beq` at
  0x63dec). The two result stores write `0x0` and `0x1` at `sp -0x28` on their respective paths
  (`str` at 0x63e20, `str` at 0x63e2c). The board-initialisation role and merged return remain
  unknown (`ldmdb` at 0x63e34).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x63dcc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x63dd4 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x63dd8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x63df0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x63e08 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x63e20 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x63e2c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x63e30 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x63e34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63e34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63e34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63de0 | `(unknown callee) 0x5dec8` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x63df8 | `(unknown callee) 0x5df4c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x63e18 | `(unknown callee) 0x64054` | `[sp-17]:1` | `0x0` | `0x1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x63dec | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `bne` at 0x63e04 | `ret(0x5df4c) != 0x0` | `ret(0x5df4c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63e34 | `r0` | `?` |
