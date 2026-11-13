# ARM scan register data helpers in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges
remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/scan-register-data-facts.txt) retain all tool sections.

## `JChip_TDR_Raw`

- `JChip_TDR_Raw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9f88 to 0xca3ec.
  Established. The first call targets `0xca498` with all four entry register candidates (`bl` at
  0xc9fac). Nonzero takes its continuation edge, while zero falls through to a stack word store
  of `0xfffff7c6` at `sp -0x34` (`bne` at 0xc9fc0, `str` at 0xc9fc8). Calls to `0xbc8b4` include
  candidates `0x0`, `0x0`, `0x1`, `0x0` and later `0x1`, `0x1`, `0x1`, `0x0` (`bl` at 0xca1cc, `bl`
  at 0xca308). The final call targets `0xc1ab4` with `[sp-36]`, `0x27ec38`, `0x0`, `[sp-32]` (`bl`
  at 0xca390). Register meanings and the merged return are unknown (`ldmdb` at 0xca3a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9f98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9f9c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc9fa0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc9fa4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc9fa8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9fb4 | store | 4 | `sp -0x30` | `ret(0xca498)` |
| `ldr` at 0xc9fb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9fc4 | load | 4 | `abs 0xca3a4` | `-` |
| `str` at 0xc9fc8 | store | 4 | `sp -0x34` | `0xfffff7c6` |
| `ldr` at 0xc9fd0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9fe4 | load | 4 | `abs 0xca3a8` | `-` |
| `str` at 0xc9fe8 | store | 4 | `sp -0x34` | `0xfffff7c5` |
| `ldr` at 0xc9ff0 | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0xc9ff4 | load | 1 | `[sp-48] +0x15` | `-` |
| `str` at 0xc9ff8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc9ffc | load | 4 | `abs 0xca3ac` | `-` |
| `ldr` at 0xca000 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xca004 | load | 1 | `[sp-48] +0xd` | `-` |
| `ldr` at 0xca024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca02c | load | 4 | `(unknown)` | `-` |
| `str` at 0xca034 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xca038 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca044 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca048 | load | 4 | `abs 0xca3b0` | `-` |
| `ldr` at 0xca054 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca05c | load | 4 | `abs 0xca3b0` | `-` |
| `str` at 0xca068 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xca06c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca070 | load | 4 | `abs 0xca3b4` | `-` |
| `ldr` at 0xca07c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca084 | load | 4 | `abs 0xca3b4` | `-` |
| `str` at 0xca090 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xca094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca098 | load | 4 | `abs 0xca3b8` | `-` |
| `ldr` at 0xca0a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca0ac | load | 4 | `abs 0xca3b8` | `-` |
| `str` at 0xca0b8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xca0bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca0cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca0e0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xca0e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca0f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca108 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xca10c | load | 4 | `abs 0xca3ac` | `-` |
| `ldr` at 0xca110 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xca114 | load | 1 | `[sp-48] +0xd` | `-` |
| `ldr` at 0xca134 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xca138 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xca13c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca140 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrb` at 0xca144 | load | 1 | `[[sp-48]] +0x8` | `-` |
| `ldr` at 0xca14c | load | 4 | `abs 0xca3bc` | `-` |
| `str` at 0xca150 | store | 4 | `sp -0x44` | `0x27ebb8` |
| `ldr` at 0xca154 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca16c | load | 4 | `abs 0xca3c0` | `-` |
| `ldr` at 0xca174 | load | 4 | `abs 0xca3ac` | `-` |
| `ldr` at 0xca178 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xca17c | load | 1 | `[sp-48] +0xd` | `-` |
| `ldr` at 0xca19c | load | 4 | `(unknown)` | `-` |
| `str` at 0xca1a0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xca1a4 | load | 4 | `abs 0xca3bc` | `-` |
| `str` at 0xca1a8 | store | 4 | `sp -0x40` | `0x27ebb8` |
| `ldr` at 0xca1ac | load | 4 | `abs 0xca3c4` | `-` |
| `str` at 0xca1b0 | store | 4 | `sp -0x3c` | `0x27ec38` |
| `ldr` at 0xca1b4 | load | 4 | `abs 0xca3c0` | `-` |
| `str` at 0xca1b8 | store | 4 | `sp -0x38` | `0x2845a8` |
| `ldr` at 0xca1dc | load | 4 | `abs 0xca3c8` | `-` |
| `str` at 0xca1e0 | store | 4 | `sp -0x44` | `0x10ab7c` |
| `ldr` at 0xca1e8 | load | 4 | `abs 0xca3cc` | `-` |
| `ldr` at 0xca1ec | load | 4 | `abs 0xca3d0` | `-` |
| `ldr` at 0xca1f8 | load | 4 | `abs 0xca3d4` | `-` |
| `str` at 0xca1fc | store | 4 | `sp -0x34` | `0xfffff7ca` |
| `ldr` at 0xca204 | load | 4 | `abs 0xca3d8` | `-` |
| `ldr` at 0xca208 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xca214 | load | 4 | `abs 0xca3ac` | `-` |
| `ldr` at 0xca218 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xca21c | load | 1 | `[sp-48] +0xd` | `-` |
| `ldr` at 0xca23c | load | 4 | `abs 0xca3c4` | `-` |
| `str` at 0xca240 | store | 4 | `sp -0x44` | `0x27ec38` |
| `ldr` at 0xca244 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xca248 | load | 4 | `abs 0xca3dc` | `-` |
| `ldr` at 0xca24c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca250 | load | 4 | `abs 0xca3bc` | `-` |
| `ldr` at 0xca258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca264 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca274 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca284 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca288 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xca28c | load | 4 | `abs 0xca3ac` | `-` |
| `ldr` at 0xca290 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xca294 | load | 1 | `[sp-48] +0xd` | `-` |
| `str` at 0xca2b8 | store | 4 | `sp -0x44` | `sp-44` |
| `ldr` at 0xca2bc | load | 4 | `abs 0xca3bc` | `-` |
| `str` at 0xca2c0 | store | 4 | `sp -0x40` | `0x27ebb8` |
| `ldr` at 0xca2c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca2c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca2cc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xca2d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca2d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xca2dc | store | 4 | `sp -0x44` | `[sp-44]` |
| `ldr` at 0xca2e0 | load | 4 | `abs 0xca3bc` | `-` |
| `str` at 0xca2e4 | store | 4 | `sp -0x40` | `0x27ebb8` |
| `ldr` at 0xca2e8 | load | 4 | `abs 0xca3c4` | `-` |
| `str` at 0xca2ec | store | 4 | `sp -0x3c` | `0x27ec38` |
| `ldr` at 0xca2f0 | load | 4 | `abs 0xca3c0` | `-` |
| `str` at 0xca2f4 | store | 4 | `sp -0x38` | `0x2845a8` |
| `ldr` at 0xca318 | load | 4 | `abs 0xca3e0` | `-` |
| `str` at 0xca31c | store | 4 | `sp -0x44` | `0x10aba8` |
| `ldr` at 0xca324 | load | 4 | `abs 0xca3cc` | `-` |
| `ldr` at 0xca328 | load | 4 | `abs 0xca3e4` | `-` |
| `ldr` at 0xca334 | load | 4 | `abs 0xca3d4` | `-` |
| `str` at 0xca338 | store | 4 | `sp -0x34` | `0xfffff7ca` |
| `ldr` at 0xca340 | load | 4 | `abs 0xca3d8` | `-` |
| `ldr` at 0xca344 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xca350 | load | 4 | `abs 0xca3c4` | `-` |
| `str` at 0xca354 | store | 4 | `sp -0x44` | `0x27ec38` |
| `ldr` at 0xca358 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xca35c | load | 4 | `abs 0xca3e8` | `-` |
| `ldr` at 0xca360 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca364 | load | 4 | `abs 0xca3bc` | `-` |
| `ldr` at 0xca36c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca378 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xca37c | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0xca380 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca384 | load | 4 | `abs 0xca3c4` | `-` |
| `ldr` at 0xca38c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca398 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xca39c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xca3a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca3a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca3a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9fac | `(unknown callee) 0xca498` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc9fd4 | `(unknown callee) 0xc8d88` | `ret(0xca498)` | `?` | `?` | `ret(0xca498)` |
| `bl` at 0xca160 | `(unknown callee) 0xca57c` | `?` | `[[sp-40]+2]:2` | `[[[sp-48]]+8]:1` | `sp-24` |
| `bl` at 0xca170 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `0x2845a8` | `?` |
| `bl` at 0xca1cc | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xca1f4 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x23b` | `0x0` |
| `bl` at 0xca254 | `(unknown callee) 0xca8c0` | `[sp+4]` | `0x10aba0` | `?` | `0x27ebb8` |
| `bl` at 0xca2d4 | `(unknown callee) 0xca7e0` | `?` | `[sp-36]` | `[sp]` | `[sp-28]` |
| `bl` at 0xca308 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xca330 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x252` | `0x0` |
| `bl` at 0xca368 | `(unknown callee) 0xca8c0` | `[sp+4]` | `0x10abcc` | `[sp-44]` | `0x27ebb8` |
| `bl` at 0xca390 | `(unknown callee) 0xc1ab4` | `[sp-36]` | `0x27ec38` | `0x0` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc9fc0 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `beq` at 0xc9fe0 | `ret(0xc8d88) == 0x0` | `ret(0xc8d88) != 0x0` |
| `beq` at 0xca040 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xca1d8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xca210 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xca260 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xca26c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xca27c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xca314 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xca34c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xca374 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca3a0 | `r0` | `?` |

## `JChip_find_tdr`

- `JChip_find_tdr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb30c to 0xcb3f4.
  Established. The call to `0xca498` has four unknown candidates (`bl` at 0xcb348); its nonzero
  result takes the continuation, while zero falls through to word `0xfffff7c6` at `sp -0x24`
  (`bne` at 0xcb35c, `str` at 0xcb364). The call to `0x187e8` carries `[sp-24]` followed
  by three unknown candidates (`bl` at 0xcb3ac). Nonzero takes the next-iteration path and zero
  falls through to storing `[sp-32]` at `sp -0x24` (`bne` at 0xcb3b8, `str` at 0xcb3c0). The
  matching rule and merged return remain unknown (`ldmdb` at 0xcb3e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb31c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb320 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcb324 | load | 4 | `abs 0xcb3e8` | `-` |
| `ldr` at 0xcb328 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb334 | load | 4 | `abs 0xcb3ec` | `-` |
| `ldr` at 0xcb338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb33c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb344 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb350 | store | 4 | `sp -0x1c` | `ret(0xca498)` |
| `ldr` at 0xcb354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb360 | load | 4 | `abs 0xcb3f0` | `-` |
| `str` at 0xcb364 | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `str` at 0xcb370 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcb374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb378 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb37c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb38c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb390 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb398 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb3a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb3a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb3a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb3bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcb3c0 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xcb3c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcb3d0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcb3d8 | load | 4 | `abs 0xcb3f0` | `-` |
| `str` at 0xcb3dc | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `ldr` at 0xcb3e0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xcb3e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb3e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb3e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb340 | `(unknown callee) 0x20f14` | `0x10aeb8` | `r0` | `r1` | `[0x27eb98]` |
| `bl` at 0xcb348 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb3ac | `(unknown callee) 0x187e8` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcb330 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcb35c | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `blo` at 0xcb384 | `C == 0` | `not (C == 0)` |
| `bne` at 0xcb3b8 | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb3e4 | `r0` | `?` |

## `JChip_find_tdr_field`

- `JChip_find_tdr_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb3f4 to 0xcb4cc.
  Established. A call to `0xce8a4` uses candidates `[sp-40]`, `[sp-28]`, unknown, unknown (`bl` at
  0xcb484); its result is compared unsigned against `[[sp-40]+2]:2`, following the edge when smaller
  and falling through when at least that value (`blo` at 0xcb4a0). The fall-through path stores word
  `0xfffff7c6` at `sp -0x2c` (`str` at 0xcb4a8). The joined r0 return is unknown (`ldmdb` at
  0xcb4bc). The selected field’s hardware role is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb404 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb408 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb40c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcb410 | load | 4 | `abs 0xcb4c0` | `-` |
| `ldr` at 0xcb414 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb420 | load | 4 | `abs 0xcb4c4` | `-` |
| `ldr` at 0xcb424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb42c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb434 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb440 | store | 4 | `sp -0x20` | `ret(0xca498)` |
| `ldr` at 0xcb444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb450 | load | 4 | `abs 0xcb4c8` | `-` |
| `str` at 0xcb454 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xcb45c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb468 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcb470 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb474 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb478 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcb47c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb480 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb48c | store | 4 | `sp -0x24` | `ret(0xce8a4)` |
| `ldr` at 0xcb490 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xcb494 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xcb498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb4a4 | load | 4 | `abs 0xcb4c8` | `-` |
| `str` at 0xcb4a8 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xcb4b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcb4b4 | store | 4 | `sp -0x2c` | `ret(0xce8a4)` |
| `ldr` at 0xcb4b8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcb4bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb4bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb4bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb430 | `(unknown callee) 0x20f14` | `0x10aee4` | `r0` | `r1` | `r2` |
| `bl` at 0xcb438 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb484 | `(unknown callee) 0xce8a4` | `[sp-40]` | `[sp-28]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcb41c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcb44c | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `blo` at 0xcb4a0 | `ret(0xce8a4) < [[sp-40]+2]:2 (unsigned, 32-bit)` | `ret(0xce8a4) >= [[sp-40]+2]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb4bc | `r0` | `?` |

## `JChip_tdr_load`

- `JChip_tdr_load`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb8f4 to 0xcc21c.
  Established. Stack words retain `0x27ebb8` and `0x27ec38` at `sp -0x44` and `sp -0x48` (`str` at
  0xcb93c, `str` at 0xcb944). Absolute word stores include zero and `[0x27ebb4]+1` at `0x27ebb4`
  (`str` at 0xcbb40, `str` at 0xcbb88). The call to `0xcf814` uses `[sp-60]`, `[sp-52]`, `[sp-68]`,
  unknown (`bl` at 0xcbca4). A later call to `0xcf8fc` carries `[sp-60]`, `0x4`, `[sp-52]`, `sp-200`
  (`bl` at 0xcbf20). Two stores write `0x0` and `0x1` at `[sp-32] +0x0` on their respective paths
  (`str` at 0xcc12c, `str` at 0xcc17c). The physical loading effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb904 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb908 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb90c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcb910 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcb914 | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcb918 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb924 | load | 4 | `abs 0xcc1a4` | `-` |
| `ldr` at 0xcb928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb92c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb930 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb938 | load | 4 | `abs 0xcc1a8` | `-` |
| `str` at 0xcb93c | store | 4 | `sp -0x44` | `0x27ebb8` |
| `ldr` at 0xcb940 | load | 4 | `abs 0xcc1ac` | `-` |
| `str` at 0xcb944 | store | 4 | `sp -0x48` | `0x27ec38` |
| `ldr` at 0xcb948 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb954 | store | 4 | `sp -0x38` | `ret(0xca498)` |
| `ldr` at 0xcb958 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcb964 | load | 4 | `abs 0xcc1b0` | `-` |
| `ldr` at 0xcb968 | load | 4 | `abs 0xcc1b4` | `-` |
| `ldr` at 0xcb96c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb974 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcb978 | store | 4 | `sp -0xe0` | `0x2845a8` |
| `ldr` at 0xcb980 | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcb984 | load | 4 | `abs 0xcc1bc` | `-` |
| `ldr` at 0xcb990 | load | 4 | `abs 0xcc1c0` | `-` |
| `str` at 0xcb994 | store | 4 | `sp -0xcc` | `0xfffff7c6` |
| `ldr` at 0xcb99c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcb9b0 | load | 4 | `abs 0xcc1c4` | `-` |
| `str` at 0xcb9b4 | store | 4 | `sp -0xcc` | `0xfffff7c5` |
| `ldr` at 0xcb9bc | load | 4 | `sp -0x38` | `-` |
| `ldrsb` at 0xcb9c0 | load | 1 | `[sp-56] +0x15` | `-` |
| `str` at 0xcb9c4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xcb9c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcb9cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb9d4 | load | 4 | `[sp-56] +0x1c` | `-` |
| `ldr` at 0xcb9dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb9e0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xcb9e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb9fc | load | 4 | `abs 0xcc1b0` | `-` |
| `ldr` at 0xcba10 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcba14 | store | 4 | `sp -0xe0` | `0x2845a8` |
| `ldr` at 0xcba1c | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcba20 | load | 4 | `abs 0xcc1c8` | `-` |
| `ldr` at 0xcba2c | load | 4 | `abs 0xcc1cc` | `-` |
| `str` at 0xcba30 | store | 4 | `sp -0xcc` | `0xfffff7ca` |
| `ldr` at 0xcba38 | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcba3c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcba48 | load | 4 | `abs 0xcc1d0` | `-` |
| `ldr` at 0xcba50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcba60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcba68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcba74 | store | 4 | `sp -0x40` | `ret(0xcb4cc)` |
| `ldr` at 0xcba78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcba84 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcba88 | store | 4 | `sp -0xe0` | `0x2845a8` |
| `ldr` at 0xcba90 | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcba94 | load | 4 | `abs 0xcc1d4` | `-` |
| `ldr` at 0xcbaa0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcbaa4 | store | 4 | `sp -0xcc` | `[sp-64]` |
| `ldr` at 0xcbaac | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcbab0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcbac0 | load | 4 | `abs 0xcc1d8` | `-` |
| `ldr` at 0xcbac4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbad0 | load | 4 | `abs 0xcc1dc` | `-` |
| `ldr` at 0xcbad4 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xcbad8 | load | 1 | `[sp-56] +0xd` | `-` |
| `ldr` at 0xcbaf8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbafc | load | 4 | `[sp-60] +0x8` | `-` |
| `ldrh` at 0xcbb00 | load | 2 | `[[sp-60]+8] +0x0` | `-` |
| `ldr` at 0xcbb04 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcbb10 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcbb14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcbb28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcbb34 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcbb38 | load | 4 | `abs 0xcc1e0` | `-` |
| `str` at 0xcbb40 | store | 4 | `abs 0x27ebb4` | `0x0` |
| `ldr` at 0xcbb44 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbb48 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbb4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbb5c | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbb60 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbb68 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcbb74 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcbb78 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbb7c | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbb80 | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcbb88 | store | 4 | `abs 0x27ebb4` | `[0x27ebb4]+1` |
| `ldr` at 0xcbb90 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbb94 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbb98 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldrh` at 0xcbb9c | load | 2 | `[[sp-60]+8] +0x0` | `-` |
| `str` at 0xcbba0 | store | 4 | `sp -0xe0` | `[[[sp-60]+8]]:2` |
| `ldr` at 0xcbba8 | load | 4 | `[sp-60] +0x10` | `-` |
| `ldr` at 0xcbbac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbbb0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbbb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbbc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcbbcc | store | 4 | `sp -0xe0` | `[sp-36]` |
| `ldr` at 0xcbbd0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcbbd4 | store | 4 | `sp -0xdc` | `[sp-68]` |
| `ldr` at 0xcbbd8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xcbbdc | store | 4 | `sp -0xd8` | `[sp-72]` |
| `ldr` at 0xcbbe0 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcbbe4 | store | 4 | `sp -0xd4` | `0x2845a8` |
| `ldr` at 0xcbc08 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcbc0c | store | 4 | `sp -0xe0` | `0x2845a8` |
| `ldr` at 0xcbc14 | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcbc24 | load | 4 | `abs 0xcc1cc` | `-` |
| `str` at 0xcbc28 | store | 4 | `sp -0xcc` | `0xfffff7ca` |
| `ldr` at 0xcbc30 | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcbc34 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcbc40 | load | 4 | `abs 0xcc1e4` | `-` |
| `ldr` at 0xcbc44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcbc50 | load | 4 | `abs 0xcc1e8` | `-` |
| `ldr` at 0xcbc54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbc58 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbc64 | load | 4 | `abs 0xcc1ec` | `-` |
| `ldr` at 0xcbc68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbc6c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcbc74 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbc78 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldrh` at 0xcbc7c | load | 2 | `[[sp-60]+8] +0x0` | `-` |
| `str` at 0xcbc80 | store | 4 | `sp -0xe0` | `[[[sp-60]+8]]:2` |
| `ldr` at 0xcbc84 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbc88 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcbc8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbc90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbc98 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbc9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbca0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbca8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbcb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbcbc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbcc0 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldrh` at 0xcbcc4 | load | 2 | `[[sp-60]+8] +0x0` | `-` |
| `str` at 0xcbcc8 | store | 4 | `sp -0xe0` | `[[[sp-60]+8]]:2` |
| `ldr` at 0xcbccc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbcd0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbcd8 | load | 4 | `[sp-60] +0x10` | `-` |
| `str` at 0xcbce4 | store | 4 | `sp -0x28` | `0x6` |
| `str` at 0xcbcf0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xcbcf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcbcf8 | store | 4 | `sp -0xe0` | `[sp-36]` |
| `ldr` at 0xcbcfc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcbd00 | store | 4 | `sp -0xdc` | `[sp-68]` |
| `ldr` at 0xcbd04 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xcbd08 | store | 4 | `sp -0xd8` | `[sp-72]` |
| `ldr` at 0xcbd0c | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcbd10 | store | 4 | `sp -0xd4` | `0x2845a8` |
| `ldr` at 0xcbd18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcbd34 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcbd38 | store | 4 | `sp -0xe0` | `0x2845a8` |
| `ldr` at 0xcbd40 | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcbd44 | load | 4 | `abs 0xcc1f0` | `-` |
| `ldr` at 0xcbd50 | load | 4 | `abs 0xcc1cc` | `-` |
| `str` at 0xcbd54 | store | 4 | `sp -0xcc` | `0xfffff7ca` |
| `ldr` at 0xcbd5c | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcbd60 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcbd6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcbd78 | load | 4 | `abs 0xcc1f4` | `-` |
| `str` at 0xcbd7c | store | 4 | `sp -0xd0` | `0x10b0f8` |
| `ldr` at 0xcbd84 | load | 4 | `abs 0xcc1f8` | `-` |
| `str` at 0xcbd88 | store | 4 | `sp -0xd0` | `0x10b108` |
| `ldr` at 0xcbd8c | load | 4 | `abs 0xcc1fc` | `-` |
| `ldr` at 0xcbd90 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xcbd94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcbda0 | load | 4 | `abs 0xcc200` | `-` |
| `ldr` at 0xcbda4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbda8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbdb4 | load | 4 | `abs 0xcc204` | `-` |
| `ldr` at 0xcbdb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbdbc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcbdc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbdd8 | load | 4 | `abs 0xcc208` | `-` |
| `ldr` at 0xcbddc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbde8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbdf8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcbdfc | store | 4 | `sp -0xe0` | `[sp-36]` |
| `ldr` at 0xcbe00 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcbe04 | store | 4 | `sp -0xdc` | `[sp-68]` |
| `ldr` at 0xcbe08 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xcbe0c | store | 4 | `sp -0xd8` | `[sp-72]` |
| `ldr` at 0xcbe10 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcbe14 | store | 4 | `sp -0xd4` | `0x2845a8` |
| `ldr` at 0xcbe38 | load | 4 | `abs 0xcc1b0` | `-` |
| `str` at 0xcbe3c | store | 4 | `sp -0xe0` | `0x2845a8` |
| `ldr` at 0xcbe44 | load | 4 | `abs 0xcc1b8` | `-` |
| `ldr` at 0xcbe48 | load | 4 | `abs 0xcc20c` | `-` |
| `ldr` at 0xcbe54 | load | 4 | `abs 0xcc1cc` | `-` |
| `str` at 0xcbe58 | store | 4 | `sp -0xcc` | `0xfffff7ca` |
| `ldr` at 0xcbe60 | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcbe64 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcbe70 | load | 4 | `abs 0xcc210` | `-` |
| `ldr` at 0xcbe74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcbe80 | load | 4 | `abs 0xcc200` | `-` |
| `ldr` at 0xcbe84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbe88 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbe94 | load | 4 | `abs 0xcc204` | `-` |
| `ldr` at 0xcbe98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbe9c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcbea4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcbeb0 | load | 4 | `abs 0xcc1e0` | `-` |
| `str` at 0xcbeb8 | store | 4 | `abs 0x27ebb4` | `0x0` |
| `ldr` at 0xcbebc | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbec0 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbec4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbed4 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbed8 | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcbef4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcbef8 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbefc | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbf00 | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcbf08 | store | 4 | `abs 0x27ebb4` | `[0x27ebb4]+1` |
| `ldr` at 0xcbf14 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcbf1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbf24 | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcbf28 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcbf3c | load | 4 | `abs 0xcc214` | `-` |
| `ldr` at 0xcbf40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbf48 | load | 4 | `abs 0xcc1e0` | `-` |
| `str` at 0xcbf50 | store | 4 | `abs 0x27ebb4` | `0x0` |
| `ldr` at 0xcbf54 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbf58 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbf5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbf6c | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbf70 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbf78 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbf80 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbf84 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbf8c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbf94 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbf98 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbfb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcbfb4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcbfbc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcbfc0 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbfc4 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbfc8 | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcbfd0 | store | 4 | `abs 0x27ebb4` | `[0x27ebb4]+1` |
| `ldr` at 0xcbfd8 | load | 4 | `abs 0xcc1e0` | `-` |
| `str` at 0xcbfe0 | store | 4 | `abs 0x27ebb4` | `0x0` |
| `ldr` at 0xcbfe4 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcbfe8 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcbfec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcbffc | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc000 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc008 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcc010 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc014 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc01c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcc024 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc028 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc040 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcc044 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc04c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcc050 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc054 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc058 | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcc060 | store | 4 | `abs 0x27ebb4` | `[0x27ebb4]+1` |
| `ldr` at 0xcc068 | load | 4 | `abs 0xcc1e0` | `-` |
| `str` at 0xcc070 | store | 4 | `abs 0x27ebb4` | `0x0` |
| `ldr` at 0xcc074 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc078 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc07c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc08c | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc090 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc0a8 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc0ac | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc0b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcc0bc | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc0c0 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcc0c8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcc0d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcc0d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc0dc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcc0e0 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc0e4 | load | 4 | `abs 0xcc1e0` | `-` |
| `ldr` at 0xcc0e8 | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcc0f0 | store | 4 | `abs 0x27ebb4` | `[0x27ebb4]+1` |
| `ldr` at 0xcc0f8 | load | 4 | `abs 0xcc1a0` | `-` |
| `ldr` at 0xcc0fc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc110 | load | 4 | `abs 0xcc218` | `-` |
| `ldr` at 0xcc114 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcc120 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcc124 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcc12c | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xcc130 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc134 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc144 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcc150 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc168 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcc174 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcc17c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0xcc180 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xcc188 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xcc194 | store | 4 | `sp -0xcc` | `0x0` |
| `ldr` at 0xcc198 | load | 4 | `sp -0xcc` | `-` |
| `ldmdb` at 0xcc19c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc19c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc19c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb934 | `(unknown callee) 0x20f14` | `0x10b020` | `r0` | `r1` | `r2` |
| `bl` at 0xcb94c | `(unknown callee) 0xca498` | `?` | `?` | `?` | `0x27ec38` |
| `bl` at 0xcb970 | `(unknown callee) 0x18c74` | `0x2845a8` | `0x10af50` | `[sp-20]` | `ret(0xca498)` |
| `bl` at 0xcb98c | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x451` | `0x0` |
| `bl` at 0xcb9a0 | `(unknown callee) 0xc8d88` | `ret(0xca498)` | `?` | `?` | `ret(0xca498)` |
| `bl` at 0xcba00 | `(unknown callee) 0xbc3c4` | `0x1` | `0x0` | `0x2845a8` | `?` |
| `bl` at 0xcba28 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x461` | `0x0` |
| `bl` at 0xcba4c | `(unknown callee) 0x20f14` | `0x10af80` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xcba6c | `(unknown callee) 0xcb4cc` | `[sp-20]` | `0x0` | `[sp-28]` | `?` |
| `bl` at 0xcba9c | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x46d` | `0x0` |
| `bl` at 0xcbacc | `(unknown callee) 0xceb64` | `0x0` | `0x10b05c` | `[sp-60]` | `0x4` |
| `bl` at 0xcbbb4 | `(unknown callee) 0xc1ab4` | `0x0` | `[[sp-60]+16]` | `[sp-52]` | `[sp-68]` |
| `bl` at 0xcbbf8 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x6` | `0x0` |
| `bl` at 0xcbc20 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x480` | `0x0` |
| `bl` at 0xcbc48 | `(unknown callee) 0x20f14` | `0x10b078` | `[sp-36]` | `?` | `[0x27eb98]` |
| `bl` at 0xcbc5c | `(unknown callee) 0xd2e88` | `0x0` | `0x10b0d0` | `[sp-44]` | `[sp-68]` |
| `bl` at 0xcbc70 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b0e4` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xcbc94 | `(unknown callee) 0xc1ab4` | `[sp-52]` | `[sp-72]` | `[sp-52]` | `[sp-68]` |
| `bl` at 0xcbca4 | `(unknown callee) 0xcf814` | `[sp-60]` | `[sp-52]` | `[sp-68]` | `?` |
| `bl` at 0xcbcdc | `(unknown callee) 0xc1ab4` | `[sp-52]` | `[sp-68]` | `0x0` | `[[sp-60]+16]` |
| `bl` at 0xcbd24 | `(unknown callee) 0xbc8b4` | `0x1` | `?` | `0x1` | `0x0` |
| `bl` at 0xcbd4c | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x49c` | `0x0` |
| `bl` at 0xcbd98 | `(unknown callee) 0x20f14` | `0x10b114` | `[sp-208]` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xcbdac | `(unknown callee) 0xd2e88` | `0x0` | `0x10b164` | `[sp-44]` | `[sp-68]` |
| `bl` at 0xcbdc0 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b170` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xcbde4 | `(unknown callee) 0xceb64` | `0x0` | `0x10b17c` | `[sp-60]` | `0x4` |
| `bl` at 0xcbe28 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcbe50 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x4af` | `0x0` |
| `bl` at 0xcbe78 | `(unknown callee) 0x20f14` | `0x10b198` | `[sp-36]` | `?` | `[0x27eb98]` |
| `bl` at 0xcbe8c | `(unknown callee) 0xd2e88` | `0x0` | `0x10b164` | `[sp-44]` | `[sp-68]` |
| `bl` at 0xcbea0 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b170` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xcbf20 | `(unknown callee) 0xcf8fc` | `[sp-60]` | `0x4` | `[sp-52]` | `sp-200` |
| `bl` at 0xcbf44 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b1ec` | `[sp-44]` | `sp-200` |
| `bl` at 0xcc118 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b1fc` | `[sp-44]` | `sp-200` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcb920 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcb960 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `beq` at 0xcb9ac | `ret(0xc8d88) == 0x0` | `ret(0xc8d88) != 0x0` |
| `beq` at 0xcb9f0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xcba0c | `ret(0xbc3c4) != 0x0` | `ret(0xbc3c4) == 0x0` |
| `bls` at 0xcba44 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xcba5c | `? == 0x0` | `? != 0x0` |
| `bge` at 0xcba80 | `ret(0xcb4cc) >= 0x0 (signed, 32-bit)` | `ret(0xcb4cc) < 0x0 (signed, 32-bit)` |
| `bls` at 0xcbab8 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `blo` at 0xcbb54 | `[0x27ebb4] < [sp-44] (unsigned, 32-bit)` | `[0x27ebb4] >= [sp-44] (unsigned, 32-bit)` |
| `beq` at 0xcbbc4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xcbc04 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcbc3c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xcbcb4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xcbd30 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcbd68 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcbd74 | `[sp-40] != 0x6` | `[sp-40] == 0x6` |
| `beq` at 0xcbdd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcbdf4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xcbe34 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcbe6c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xcbeac | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `blo` at 0xcbecc | `[0x27ebb4] < [sp-44] (unsigned, 32-bit)` | `[0x27ebb4] >= [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xcbf30 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |
| `blo` at 0xcbf64 | `[0x27ebb4] < [sp-44] (unsigned, 32-bit)` | `[0x27ebb4] >= [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xcbff4 | `[0x27ebb4] < [sp-44] (unsigned, 32-bit)` | `[0x27ebb4] >= [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xcc084 | `[0x27ebb4] < [sp-44] (unsigned, 32-bit)` | `[0x27ebb4] >= [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xcc104 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xcc13c | `[sp-48] >= [sp-44] (unsigned, 32-bit)` | `[sp-48] < [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xcc14c | `[[sp-32]] != 0x0` | `[[sp-32]] == 0x0` |
| `beq` at 0xcc170 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc19c | `r0` | `?` |

## `tdrfld_create`

- `tdrfld_create`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdecc to 0xcdf44.
  Established. The opening call to `0x18920` carries `0x8`, entry r1, entry r2 and entry r3 (`bl` at
  0xcdee4). A nonzero result takes the continuation and zero falls through to a word store of
  `0x0` at `sp -0x1c` (`bne` at 0xcdef8, `str` at 0xcdf00). The subsequent call targets `0xcdf44`
  with `ret(0x18920)`, `[sp-20]`, unknown, `ret(0x18920)` (`bl` at 0xcdf10). The final r0 value is
  unknown after paths merge (`ldmdb` at 0xcdf40). The numeric callees’ allocation contract is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdedc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdeec | store | 4 | `sp -0x18` | `ret(0x18920)` |
| `ldr` at 0xcdef0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcdf00 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcdf08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdf0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdf20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcdf2c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcdf34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcdf38 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xcdf3c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xcdf40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdf40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdf40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdee4 | `(unknown callee) 0x18920` | `0x8` | `r1` | `r2` | `r3` |
| `bl` at 0xcdf10 | `(unknown callee) 0xcdf44` | `ret(0x18920)` | `[sp-20]` | `?` | `ret(0x18920)` |
| `bl` at 0xcdf24 | `(unknown callee) 0x18be4` | `[sp-24]` | `?` | `?` | `ret(0xcdf44)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcdef8 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xcdf1c | `ret(0xcdf44) != 0x0` | `ret(0xcdf44) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdf40 | `r0` | `?` |

## `tdrfld_init`

- `tdrfld_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdf44 to 0xcdfd4.
  Established. Zero entry r1 takes the bypass and nonzero falls through to the only call (`beq`
  at 0xcdf64, `bl` at 0xcdf70). That call targets `0x18788` with candidates entry r1, entry r1,
  entry r2, entry r1 (`bl` at 0xcdf70). The stores include a zero halfword at `[sp-24] +0x4` and
  zero bytes at `[sp-24] +0x6` and `[sp-24] +0x7` (`strh` at 0xcdfa8, `strb` at 0xcdfb4, `strb` at
  0xcdfc0). The merged return and field interpretation remain unknown (`ldmdb` at 0xcdfd0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdf54 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcdf58 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xcdf5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcdf68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdf6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcdf74 | store | 4 | `r0 +0x0` | `ret(0x18788)` |
| `ldr` at 0xcdf78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdf7c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xcdf8c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcdf94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcdf9c | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xcdfa0 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xcdfa8 | store | 2 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xcdfac | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xcdfb4 | store | 1 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xcdfb8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xcdfc0 | store | 1 | `[sp-24] +0x7` | `0x0` |
| `str` at 0xcdfc8 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xcdfcc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcdfd0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcdfd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdfd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdfd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdf70 | `(unknown callee) 0x18788` | `r1` | `r1` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcdf64 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xcdf84 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdfd0 | `r0` | `?` |

## `tdrfld_dump`

- `tdrfld_dump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdfd4 to 0xce0e8.
  Established. Entry r2 being nonzero takes the longer body, while zero falls through to the
  return (`bne` at 0xcdff8). Calls to `0x21010` include candidate pairs `[sp-20]`, `0x10b53c`
  followed by two `[[sp-28]+4]:2` values (`bl` at 0xce08c). Later calls use `[sp-20]`, `0x10b548`
  with two `[[sp-28]+6]:1` values, then `[sp-20]`, `0x10b554` with two `[[sp-28]+7]:1` values (`bl`
  at 0xce0a4, `bl` at 0xce0c0). These are argument candidates with unknown arity. No hardware action
  is established by the calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdfe4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdfe8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcdfec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcdff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce000 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce00c | load | 4 | `abs 0xce0c8` | `-` |
| `ldr` at 0xce010 | load | 4 | `abs 0x116924` | `-` |
| `str` at 0xce014 | store | 4 | `sp -0x14` | `[0x116924]` |
| `ldr` at 0xce018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce028 | load | 4 | `abs 0xce0cc` | `-` |
| `ldr` at 0xce034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce038 | load | 4 | `abs 0xce0d0` | `-` |
| `ldr` at 0xce03c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce048 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xce04c | store | 4 | `sp -0x20` | `[[sp-28]]` |
| `ldr` at 0xce050 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce054 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xce060 | load | 4 | `abs 0xce0d4` | `-` |
| `str` at 0xce064 | store | 4 | `sp -0x20` | `0x10b528` |
| `ldr` at 0xce068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce06c | load | 4 | `abs 0xce0d8` | `-` |
| `ldr` at 0xce070 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce078 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce07c | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xce080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce084 | load | 4 | `abs 0xce0dc` | `-` |
| `ldr` at 0xce090 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce094 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xce098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce09c | load | 4 | `abs 0xce0e0` | `-` |
| `ldr` at 0xce0a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce0ac | load | 1 | `[sp-28] +0x7` | `-` |
| `ldr` at 0xce0b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce0b4 | load | 1 | `[sp-28] +0x7` | `-` |
| `ldr` at 0xce0b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce0bc | load | 4 | `abs 0xce0e4` | `-` |
| `ldmdb` at 0xce0c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce0c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce0c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce02c | `(unknown callee) 0x21010` | `?` | `0x10b504` | `r2` | `r1` |
| `bl` at 0xce040 | `(unknown callee) 0x21010` | `?` | `0x10b524` | `r1` | `r1` |
| `bl` at 0xce074 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b530` | `?` | `?` |
| `bl` at 0xce08c | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b53c` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xce0a4 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b548` | `[[sp-28]+6]:1` | `[[sp-28]+6]:1` |
| `bl` at 0xce0c0 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b554` | `[[sp-28]+7]:1` | `[[sp-28]+7]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcdff8 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xce008 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xce020 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xce05c | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce0c4 | `r0` | `?` |

## `tdrdef_create`

- `tdrdef_create`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce0e8 to 0xce168.
  Established. The call to `0x18920` has candidates `0xc`, entry r1, entry r2 and entry r3 (`bl` at
  0xce104); its nonzero result takes the continuation, while zero falls through to a stack word
  of `0x0` at `sp -0x20` (`bne` at 0xce118, `str` at 0xce120). The next call to `0xce168` uses
  `ret(0x18920)`, `[sp-20]`, `[sp-24]`, `ret(0x18920)` (`bl` at 0xce134). The two explicit stores to
  the result slot on later paths write `0x0` and `[sp-28]`, respectively (`str` at 0xce150, `str` at
  0xce15c). The merged return and record’s device meaning are unknown (`ldmdb` at 0xce164).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce0f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce0fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xce10c | store | 4 | `sp -0x1c` | `ret(0x18920)` |
| `ldr` at 0xce110 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce120 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xce128 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce12c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce144 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce150 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xce158 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce15c | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xce160 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xce164 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce164 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce104 | `(unknown callee) 0x18920` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xce134 | `(unknown callee) 0xce168` | `ret(0x18920)` | `[sp-20]` | `[sp-24]` | `ret(0x18920)` |
| `bl` at 0xce148 | `(unknown callee) 0x18be4` | `[sp-28]` | `?` | `?` | `ret(0xce168)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce118 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xce140 | `ret(0xce168) != 0x0` | `ret(0xce168) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce164 | `r0` | `?` |

## `tdrdef_init`

- `tdrdef_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce168 to 0xce270.
  Established. A word store puts `ret(0x18920)` at entry `r0 +0x4` (`str` at 0xce1a0). Later stores
  place `([sp-32]:2 & 0xffff)` at `[sp-24] +0x2`, `ret(0x18920)` at `[sp-24] +0x8`, and `([sp-28]:2
  & 0xffff)` at `[sp-24] +0x0` (`strh` at 0xce1d8, `str` at 0xce218, `strh` at 0xce25c). The final
  call targets `0x18c20` with `[[sp-24]+8]`, `0x0`, `[sp-36]`, `[sp-24]` (`bl` at 0xce250). The
  element layout’s hardware interpretation and the return value remain unknown (`ldmdb` at 0xce26c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce178 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xce17c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xce180 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xce184 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce18c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xce190 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce194 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xce1a0 | store | 4 | `r0 +0x4` | `ret(0x18920)` |
| `ldr` at 0xce1a4 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xce1b4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xce1bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce1c0 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xce1c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce1d0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xce1d4 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xce1d8 | store | 2 | `[sp-24] +0x2` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xce1dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce1f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce204 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xce208 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce20c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xce218 | store | 4 | `[sp-24] +0x8` | `ret(0x18920)` |
| `ldr` at 0xce21c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xce228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce22c | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xce238 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xce240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce244 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xce24c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce254 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xce258 | load | 2 | `sp -0x1c` | `-` |
| `strh` at 0xce25c | store | 2 | `[sp-24] +0x0` | `([sp-28]:2 & 0xffff)` |
| `str` at 0xce264 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xce268 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xce26c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xce26c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce26c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce26c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce198 | `(unknown callee) 0x18920` | `[sp-36]` | `r1` | `r2` | `?` |
| `bl` at 0xce1cc | `(unknown callee) 0x18c20` | `[[sp-24]+4]` | `0x0` | `[sp-36]` | `[sp-24]` |
| `bl` at 0xce210 | `(unknown callee) 0x18920` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xce230 | `(unknown callee) 0x18be4` | `[[sp-24]+4]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xce250 | `(unknown callee) 0x18c20` | `[[sp-24]+8]` | `0x0` | `[sp-36]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce1ac | `[r0+4] != 0x0` | `[r0+4] == 0x0` |
| `bne` at 0xce224 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce26c | `r0` | `?` |

## `tdrdef_dump`

- `tdrdef_dump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce270 to 0xce410.
  Established. The longer path is taken for nonzero entry r2, while zero falls through to the
  return (`bne` at 0xce294). A call to `0xcdfd4` carries `[sp-20]`, `sp-72`, unknown, unknown (`bl`
  at 0xce368). The final call targets `0xd2e88` with candidates `[sp-20]`, `0x10b5b0`, `[sp-76]`,
  `[[sp-28]+8]` (`bl` at 0xce3ec). Stack word `sp -0x4c` can receive `[sp-76]+1` (`str` at 0xce3d4).
  The calls’ purposes remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce280 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce284 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xce288 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xce28c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce29c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2a8 | load | 4 | `abs 0xce3f4` | `-` |
| `ldr` at 0xce2ac | load | 4 | `abs 0x116924` | `-` |
| `str` at 0xce2b0 | store | 4 | `sp -0x14` | `[0x116924]` |
| `ldr` at 0xce2b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce2c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2c4 | load | 4 | `abs 0xce3f8` | `-` |
| `ldr` at 0xce2d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2d4 | load | 4 | `abs 0xce3fc` | `-` |
| `ldr` at 0xce2d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce2e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce2e4 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xce2e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce2ec | load | 4 | `abs 0xce400` | `-` |
| `ldr` at 0xce2f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce2fc | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xce300 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce304 | load | 4 | `abs 0xce404` | `-` |
| `str` at 0xce314 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xce318 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce31c | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xce320 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce338 | load | 4 | `abs 0xce408` | `-` |
| `ldr` at 0xce33c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce348 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce34c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce354 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xce35c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce36c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce374 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xce37c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce380 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xce398 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce39c | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xce3ac | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xce3b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xce3b8 | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xce3c8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xce3cc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xce3d4 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xce3d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce3dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce3e0 | load | 4 | `abs 0xce40c` | `-` |
| `ldr` at 0xce3e4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xce3e8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xce3f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce3f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce3f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce2c8 | `(unknown callee) 0x21010` | `?` | `0x10b56c` | `r2` | `r1` |
| `bl` at 0xce2dc | `(unknown callee) 0x21010` | `?` | `0x10b524` | `r1` | `r1` |
| `bl` at 0xce2f4 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b548` | `[[sp-28]]:2` | `[[sp-28]]:2` |
| `bl` at 0xce30c | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b594` | `[[sp-28]+2]:2` | `[[sp-28]+2]:2` |
| `bl` at 0xce340 | `(unknown callee) 0x18c74` | `sp-72` | `0x10b5a0` | `?` | `sp-72` |
| `bl` at 0xce368 | `(unknown callee) 0xcdfd4` | `[sp-20]` | `sp-72` | `?` | `?` |
| `bl` at 0xce3ec | `(unknown callee) 0xd2e88` | `[sp-20]` | `0x10b5b0` | `[sp-76]` | `[[sp-28]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce294 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xce2a4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xce2bc | `r1 != 0x0` | `r1 == 0x0` |
| `blo` at 0xce328 | `C == 0` | `not (C == 0)` |
| `bne` at 0xce394 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce3f0 | `r0` | `?` |

## `tdrdef_get_field`

- `tdrdef_get_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce614 to 0xce8a4.
  Established. An explicit word store clears an unknown target (`str` at 0xce760). Later word stores
  at unknown targets have unknown values (`str` at 0xce7f4, `str` at 0xce82c). The branch on
  `[sp-56]` takes its edge when unequal to `0x80000000` and falls through on equality (`bne` at
  0xce838). The following branch takes its edge for `[sp-52] != 0x1` and falls through on
  equality (`bne` at 0xce868). No calls are reported. The extracted field’s meaning and r0 return
  remain unknown (`ldmdb` at 0xce8a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce624 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce628 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xce62c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xce630 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xce634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce640 | load | 4 | `r0 +0x4` | `-` |
| `ldrb` at 0xce648 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xce65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce660 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce668 | load | 4 | `r0 +0x4` | `-` |
| `ldrb` at 0xce670 | load | 1 | `(unknown)` | `-` |
| `str` at 0xce67c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xce684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce688 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce690 | load | 4 | `r0 +0x4` | `-` |
| `ldrb` at 0xce698 | load | 1 | `(unknown)` | `-` |
| `str` at 0xce6a4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xce6a8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xce6b0 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xce6b4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xce6b8 | store | 4 | `sp -0x28` | `[sp-60]` |
| `ldr` at 0xce6bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xce6c4 | store | 4 | `sp -0x30` | `[sp-60]-1` |
| `str` at 0xce6cc | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xce6d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce6d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce6dc | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xce6e4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xce6e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce6f4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xce6fc | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xce700 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce70c | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xce714 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xce718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce724 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xce72c | store | 4 | `sp -0x34` | `?` |
| `str` at 0xce734 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xce738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce73c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xce74c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce754 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xce760 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xce764 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xce76c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xce778 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xce77c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce788 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xce790 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xce794 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce7a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce7ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce7b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce7b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce7c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce7d0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce7d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce7e0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce7e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce7ec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xce7f4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xce7fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce804 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce80c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce814 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce81c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xce824 | load | 4 | `(unknown)` | `-` |
| `str` at 0xce82c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xce830 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xce83c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xce844 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `str` at 0xce84c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xce854 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xce85c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xce860 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce86c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xce874 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xce87c | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xce884 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xce88c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xce890 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xce898 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xce8a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce8a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce8a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce658 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xce744 | `C == 0` | `not (C == 0)` |
| `blo` at 0xce79c | `C == 0` | `not (C == 0)` |
| `beq` at 0xce7c4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xce838 | `[sp-56] != 0x80000000` | `[sp-56] == 0x80000000` |
| `bne` at 0xce868 | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce8a0 | `r0` | `?` |

## `tdrdef_find_field`

- `tdrdef_find_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce8a4 to 0xce934.
  Established. The only call targets `0x187e8` with four unknown candidates (`bl` at 0xce8f8); its
  nonzero result takes the next-iteration path and zero falls through to a stack word store of
  `[sp-28]` at `sp -0x20` (`bne` at 0xce904, `str` at 0xce90c). The loop branch retains the
  flags-only predicate `C == 0`, following its edge for that predicate and falling through otherwise
  (`blo` at 0xce8d4). The return is unknown after the join (`ldmdb` at 0xce930). The callee’s
  matching semantics remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce8b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce8b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xce8c0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xce8c4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xce8c8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xce8cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce8dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce8e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce8e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce8f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce8f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce908 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce90c | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xce914 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce91c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xce924 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce928 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xce92c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xce930 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce930 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce930 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce8f8 | `(unknown callee) 0x187e8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xce8d4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xce904 | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce930 | `r0` | `?` |

## `tdrdef_set_field`

- `tdrdef_set_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce410 to 0xce614.
  Established. Two word stores have unknown targets and values (`str` at
  0xce564, `str` at 0xce59c). Their selecting branch takes its edge for `? == 0x0` and falls through
  for `? != 0x0` (`beq` at 0xce534). A later branch takes its edge for `[sp-48] != 0x80000000` and
  falls through on equality (`bne` at 0xce5a8). The equality path writes word `0x1` at `sp -0x30`
  (`str` at 0xce5bc). No calls are reported, and the destination bits’ hardware meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce420 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce424 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xce428 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xce42c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xce430 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce43c | load | 4 | `r0 +0x4` | `-` |
| `ldrb` at 0xce444 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xce45c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce468 | load | 4 | `r0 +0x4` | `-` |
| `ldrb` at 0xce470 | load | 1 | `(unknown)` | `-` |
| `str` at 0xce480 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xce488 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xce48c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce498 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xce4a0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xce4a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce4b0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xce4b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce4b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce4c0 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xce4c8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xce4cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce4e0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xce4e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xce4ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce4f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce4f8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrb` at 0xce500 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xce504 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce514 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xce51c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce524 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce528 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce538 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce540 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce548 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce550 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce558 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce55c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xce564 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xce56c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce574 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce57c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce584 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xce58c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce594 | load | 4 | `(unknown)` | `-` |
| `str` at 0xce59c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xce5a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce5ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xce5b4 | store | 4 | `sp -0x28` | `[sp-40]-1` |
| `str` at 0xce5bc | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xce5c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xce5cc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xce5d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce5dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xce5e4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xce5ec | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xce5f4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xce5fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xce600 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xce608 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xce610 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce610 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce610 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xce50c | `C == 0` | `not (C == 0)` |
| `beq` at 0xce534 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xce5a8 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xce5d8 | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce610 | `r0` | `?` |

## `tdrdata_add_overrides`

- `tdrdata_add_overrides`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf814 to 0xcf8fc.
  Established. The stack saves entry r0, r1 and r2 at `sp -0x14`, `sp -0x18` and `sp -0x1c` (`str`
  at 0xcf824, `str` at 0xcf828, `str` at 0xcf82c). The test before the call takes its bypass for `?
  == 0x0` and falls through for inequality (`beq` at 0xcf870). The only call targets `0xc1ab4`, with
  all four candidates unknown (`bl` at 0xcf8e4); its prior stack word store at `sp -0x24` also
  has unknown value (`str` at 0xcf8d4). No external override operation or known return is
  established (`ldmdb` at 0xcf8f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf824 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf828 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf82c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcf834 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcf838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf83c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf840 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf844 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf858 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf85c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcf864 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf878 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf87c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf884 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf88c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf890 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf89c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf8a4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf8ac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf8b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf8b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf8bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf8c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf8c8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcf8d0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcf8d4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcf8d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf8e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf8e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf8f0 | store | 4 | `sp -0x20` | `?` |
| `ldmdb` at 0xcf8f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf8f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf8f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf8e4 | `(unknown callee) 0xc1ab4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xcf84c | `C == 0` | `not (C == 0)` |
| `beq` at 0xcf870 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf8f8 | `r0` | `?` |

## `tdrdata_build_mask`

- `tdrdata_build_mask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf8fc to 0xcf9e0.
  Established. The first call targets `0xc1880` with candidates entry r2, `[[r0+8]]:2`, entry r3,
  `[[r0+8]]:2` (`bl` at 0xcf934). The later call to `0xc19a0` has unknown, unknown, `[sp-32]`,
  unknown (`bl` at 0xcf9c8). The branch before that call takes its bypass for `? == 0x0` and falls
  through otherwise (`beq` at 0xcf97c). Stack word `sp -0x24` starts at zero (`str` at 0xcf93c). The
  resulting mask’s physical use and the merged return are unknown (`ldmdb` at 0xcf9dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf90c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf910 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf914 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcf918 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcf91c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf920 | load | 4 | `r0 +0x8` | `-` |
| `ldrh` at 0xcf924 | load | 2 | `[r0+8] +0x0` | `-` |
| `ldr` at 0xcf928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf930 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf93c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcf940 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf944 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrh` at 0xcf948 | load | 2 | `[[sp-20]+8] +0x2` | `-` |
| `ldr` at 0xcf94c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf95c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf960 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xcf964 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcf96c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf980 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf984 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xcf988 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf990 | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xcf998 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf99c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf9a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf9a8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xcf9ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf9b4 | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrb` at 0xcf9bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf9c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf9cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcf9d4 | store | 4 | `sp -0x24` | `?` |
| `ldmdb` at 0xcf9dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf9dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf9dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf934 | `(unknown callee) 0xc1880` | `r2` | `[[r0+8]]:2` | `r3` | `[[r0+8]]:2` |
| `bl` at 0xcf9c8 | `(unknown callee) 0xc19a0` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xcf954 | `C == 0` | `not (C == 0)` |
| `beq` at 0xcf97c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf9dc | `r0` | `?` |
