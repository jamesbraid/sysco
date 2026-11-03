# ARM power-bay requests and status in 1.40.5

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
`PBaySetState` and `PBayCmd` have unresolved computed transfers. Their access, call and return
lists cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/power-bay-requests-status-facts.txt) retain all tool sections.

## `PBayInit`

- `PBayInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa02c to 0xaa12c.
  Established. The absolute word assignment at `0x14a74c` writes `ret(0x1b1b4)` (`str` at 0xaa054).
  The halfword assignment at `0x14a744` writes `0x0` (`strh` at 0xaa084). The unnamed `0x22480` call
  carries `[0x14a74c]+4`, `0x127a44`, `0xaad38`, `0x1` (`bl` at 0xaa0e0). Setup roles remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaa044 | load | 4 | `abs 0xaa11c` | `-` |
| `str` at 0xaa054 | store | 4 | `abs 0x14a74c` | `ret(0x1b1b4)` |
| `ldr` at 0xaa058 | load | 4 | `abs 0xaa11c` | `-` |
| `ldr` at 0xaa05c | load | 4 | `abs 0x14a74c` | `-` |
| `ldr` at 0xaa06c | load | 4 | `abs 0xaa120` | `-` |
| `ldr` at 0xaa07c | load | 4 | `abs 0xaa120` | `-` |
| `strh` at 0xaa084 | store | 2 | `abs 0x14a744` | `0x0` |
| `ldr` at 0xaa088 | load | 4 | `abs 0xaa11c` | `-` |
| `ldr` at 0xaa08c | load | 4 | `abs 0x14a74c` | `-` |
| `ldr` at 0xaa090 | load | 4 | `abs 0xaa120` | `-` |
| `str` at 0xaa094 | store | 4 | `[0x14a74c] +0x8` | `0x14a73c` |
| `ldr` at 0xaa098 | load | 4 | `abs 0xaa11c` | `-` |
| `ldr` at 0xaa09c | load | 4 | `abs 0x14a74c` | `-` |
| `ldr` at 0xaa0a4 | load | 4 | `abs 0xaa11c` | `-` |
| `ldr` at 0xaa0a8 | load | 4 | `abs 0x14a74c` | `-` |
| `str` at 0xaa0ac | store | 4 | `sp -0x2c` | `[0x14a74c]` |
| `str` at 0xaa0b4 | store | 4 | `sp -0x28` | `0xa00` |
| `str` at 0xaa0bc | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xaa0c4 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0xaa0cc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xaa0d4 | load | 4 | `abs 0xaa124` | `-` |
| `ldr` at 0xaa0d8 | load | 4 | `abs 0xaa128` | `-` |
| `str` at 0xaa0e8 | store | 4 | `sp -0x18` | `ret(0x22480)` |
| `ldr` at 0xaa0ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa10c | load | 4 | `abs 0xaa11c` | `-` |
| `ldr` at 0xaa110 | load | 4 | `abs 0x14a74c` | `-` |
| `ldmdb` at 0xaa118 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaa118 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa118 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa118 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa040 | `(unknown callee) 0xaa12c` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xaa04c | `(unknown callee) 0x1b1b4` | `0x104` | `?` | `?` | `?` |
| `bl` at 0xaa068 | `(unknown callee) 0x1b4cc` | `[0x14a74c]` | `0x0` | `0x104` | `0x14a74c` |
| `bl` at 0xaa078 | `(unknown callee) 0x1b4cc` | `0x14a73c` | `0x0` | `0x10` | `?` |
| `bl` at 0xaa0e0 | `(unknown callee) 0x22480` | `[0x14a74c]+4` | `0x127a44` | `0xaad38` | `0x1` |
| `bl` at 0xaa100 | `(unknown callee) 0xaa7cc` | `?` | `0x7` | `?` | `ret(0x22480)` |
| `bl` at 0xaa108 | `(unknown callee) 0xaa12c` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xaa114 | `(unknown callee) 0x1b490` | `[0x14a74c]` | `?` | `?` | `0x14a74c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa0f4 | `ret(0x22480) == 0x0` | `ret(0x22480) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa118 | `r0` | `?` |

## `PBaySetState`

- `PBaySetState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa12c to 0xaa208.
  Established. The absolute four-byte assignment at `0x14a738` writes `r0` (`str` at 0xaa148). The
  selector takes unsigned `[0x14a738] <= 0x5` to an unresolved destination, and falls through above
  that bound (`ldrls` at 0xaa158). The known fallback writes word `0x127ab8` at `0x143614` (`str` at
  0xaa1e0). State-label interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa13c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaa140 | load | 4 | `abs 0xaa1e8` | `-` |
| `ldr` at 0xaa144 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaa148 | store | 4 | `abs 0x14a738` | `r0` |
| `ldr` at 0xaa14c | load | 4 | `abs 0xaa1e8` | `-` |
| `ldr` at 0xaa150 | load | 4 | `abs 0x14a738` | `-` |
| `ldrls` at 0xaa158 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaa1d8 | load | 4 | `abs 0xaa1ec` | `-` |
| `ldr` at 0xaa1dc | load | 4 | `abs 0xaa204` | `-` |
| `str` at 0xaa1e0 | store | 4 | `abs 0x143614` | `0x127ab8` |
| `ldmdb` at 0xaa1e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa1e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa1e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xaa158 | `[0x14a738] <= 0x5 (unsigned, 32-bit)` | `[0x14a738] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa1e4 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xaa158).

## `PBayGetState`

- `PBayGetState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa208 to 0xaa228.
  Established. The absolute word at `0x14a738` is read (`ldr` at 0xaa218). The return reports
  `[0x14a738]` (`ldmdb` at 0xaa220). Power-bay state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaa214 | load | 4 | `abs 0xaa224` | `-` |
| `ldr` at 0xaa218 | load | 4 | `abs 0x14a738` | `-` |
| `ldmdb` at 0xaa220 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa220 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa220 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa220 | `r0` | `[0x14a738]` |

## `PBayCmd`

- `PBayCmd`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa228 to 0xaa7cc.
  Established. The initial word assignment through `r1 +0x0` writes `0x0` (`str` at 0xaa250). The
  unnamed `0x8dbc0` call carries `[[[sp-28]+112]]`, `[sp-32]`, `r1`, `[[sp-28]+112]` (`bl` at
  0xaa264); its zero result takes `0xaa280`, while nonzero falls through (`beq` at 0xaa270).
  Dispatch effects beyond known edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa238 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xaa23c | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xaa244 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xaa248 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa250 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xaa254 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa258 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xaa25c | load | 4 | `[[sp-28]+112] +0x0` | `-` |
| `ldr` at 0xaa260 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa278 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xaa280 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa288 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xaa290 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xaa2ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa2bc | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xaa2c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaa2c4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xaa2c8 | load | 4 | `abs 0xaa7a4` | `-` |
| `ldr` at 0xaa2d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa2d4 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xaa2dc | load | 4 | `abs 0xaa7a8` | `-` |
| `ldr` at 0xaa2e0 | load | 4 | `abs 0x143614` | `-` |
| `ldr` at 0xaa2ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa2fc | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xaa300 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaa304 | load | 4 | `abs 0xaa7a8` | `-` |
| `ldr` at 0xaa308 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xaa30c | load | 4 | `abs 0x143614` | `-` |
| `str` at 0xaa318 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xaa320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa328 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xaa330 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xaa33c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa348 | store | 4 | `sp -0x24` | `ret(0xaacc8)` |
| `ldr` at 0xaa34c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa350 | store | 4 | `sp -0x34` | `ret(0xaacc8)` |
| `ldr` at 0xaa358 | load | 4 | `abs 0xaa7ac` | `-` |
| `ldr` at 0xaa35c | load | 4 | `abs 0x14a74c` | `-` |
| `str` at 0xaa374 | store | 4 | `sp -0x2c` | `ret(0x22ac8)` |
| `ldr` at 0xaa378 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaa384 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa388 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xaa398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa3a0 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xaa3a8 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldrls` at 0xaa3b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaa758 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xaa75c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa764 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xaa76c | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xaa780 | load | 4 | `abs 0xaa7ac` | `-` |
| `ldr` at 0xaa784 | load | 4 | `abs 0x14a74c` | `-` |
| `ldr` at 0xaa794 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa798 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xaa79c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xaa7a0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaa7a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaa7a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa7a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa7a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa264 | `(unknown callee) 0x8dbc0` | `[[[sp-28]+112]]` | `[sp-32]` | `r1` | `[[sp-28]+112]` |
| `bl` at 0xaa29c | `(unknown callee) 0xaa208` | `ret(0x8dbc0)` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xaa2b4 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `ret(0xaa208)` |
| `bl` at 0xaa2cc | `(unknown callee) 0x1b598` | `[[sp-32]]` | `0x127acc` | `?` | `[sp-32]` |
| `bl` at 0xaa2f4 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `0x4` | `[0x143614]` |
| `bl` at 0xaa310 | `(unknown callee) 0x1b598` | `[[sp-32]]` | `[0x143614]` | `0x143614` | `[sp-32]` |
| `bl` at 0xaa340 | `(unknown callee) 0xaacc8` | `[sp-32]` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xaa36c | `(unknown callee) 0x22ac8` | `[0x14a74c]+164` | `0x0` | `0x4` | `[0x14a74c]+164` |
| `bl` at 0xaa394 | `(unknown callee) 0xaa12c` | `0x3` | `?` | `?` | `ret(0x22ac8)` |
| `bl` at 0xaa77c | `(unknown callee) 0xaa12c` | `0x2` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xaa790 | `(unknown callee) 0x22e50` | `[0x14a74c]+164` | `?` | `?` | `[0x14a74c]+164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa270 | `ret(0x8dbc0) == 0x0` | `ret(0x8dbc0) != 0x0` |
| `bne` at 0xaa298 | `[[[sp-28]+112]+4] != 0xf` | `[[[sp-28]+112]+4] == 0xf` |
| `beq` at 0xaa2a8 | `ret(0xaa208) == 0x5` | `ret(0xaa208) != 0x5` |
| `beq` at 0xaa2e8 | `[0x143614] == 0x0` | `[0x143614] != 0x0` |
| `bne` at 0xaa338 | `[[[sp-28]+112]+4] != 0x10` | `[[[sp-28]+112]+4] == 0x10` |
| `beq` at 0xaa380 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `ldrls` at 0xaa3b4 | `[[[sp-28]+112]+4]-1 <= 0xf (unsigned, 32-bit)` | `[[[sp-28]+112]+4]-1 > 0xf (unsigned, 32-bit)` |
| `beq` at 0xaa774 | `[[[sp-28]+112]+4] == 0xf` | `[[[sp-28]+112]+4] != 0xf` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa7a0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xaa3b4).

## `PBayError`

- `PBayError`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa7cc to 0xaa808.
  Established. The absolute four-byte read uses `0x145ae8` (`ldr` at 0xaa7e8). The unnamed `0x240b8`
  call carries `[0x145ae8]`, `0x127b44`, `r0`, `r1` (`bl` at 0xaa7f8); its result is reported at the
  return (`ldmdb` at 0xaa7fc). Error interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa7dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaa7e0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xaa7e4 | load | 4 | `abs 0xaa800` | `-` |
| `ldr` at 0xaa7e8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xaa7ec | load | 4 | `abs 0xaa804` | `-` |
| `ldr` at 0xaa7f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaa7f4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaa7fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa7fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa7fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa7f8 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x127b44` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa7fc | `r0` | `ret(0x240b8)` |

## `PBayInitFunc`

- `PBayInitFunc`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaad38 to 0xaadf8.
  Established. The unnamed `0x229b0` call carries `r1+164`, `0x127b78`, `0x0`, unknown (`bl` at
  0xaad70); its zero result takes `0xaad98`, while nonzero falls through (`beq` at 0xaad7c). The
  halfword assignment at `[[sp-36]+8] +0x4` has an unknown value (`strh` at 0xaadd8). Power-bay
  setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaad48 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xaad4c | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0xaad50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaad54 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0xaad58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaad64 | load | 4 | `abs 0xaadf4` | `-` |
| `ldr` at 0xaada0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaada4 | load | 4 | `[sp-36] +0x8` | `-` |
| `strh` at 0xaadd8 | store | 2 | `[[sp-36]+8] +0x4` | `?` |
| `ldmdb` at 0xaadf0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaadf0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaadf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaadf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaadf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaad70 | `(unknown callee) 0x229b0` | `r1+164` | `0x127b78` | `0x0` | `?` |
| `bl` at 0xaad88 | `(unknown callee) 0xaa7cc` | `?` | `0x11` | `?` | `ret(0x229b0)` |
| `bl` at 0xaad90 | `(unknown callee) 0xaa12c` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xaad9c | `(unknown callee) 0xaa12c` | `0x1` | `?` | `?` | `ret(0x229b0)` |
| `bl` at 0xaadac | `(unknown callee) 0x1f26c` | `0x1` | `?` | `?` | `[sp-36]` |
| `bl` at 0xaadc0 | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xaade4 | `(unknown callee) 0xaa7cc` | `?` | `0x2` | `?` | `?` |
| `bl` at 0xaadec | `(unknown callee) 0xaa12c` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaad7c | `ret(0x229b0) == 0x0` | `ret(0x229b0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaadf0 | `r0` | `ret(0xaa12c)` |

## `PBayInitIcmb`

- `PBayInitIcmb`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaadf8 to 0xab0d4.
  Established. The halfword assignment at `[[sp-24]+8] +0x6` first writes `0x0` (`strh` at 0xaae20).
  A later assignment at that target writes `0x4400` (`strh` at 0xaaf38). The unnamed `0xab0d4` call
  at `0xaae6c` carries `[[sp-24]+8]`, `sp-436`, `sp-436`, `[sp-24]` (`bl` at 0xaae6c). ICMB request
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaae08 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xaae14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaae18 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0xaae20 | store | 2 | `[[sp-24]+8] +0x6` | `0x0` |
| `ldr` at 0xaae24 | load | 4 | `abs 0xab0d0` | `-` |
| `strh` at 0xaae30 | store | 2 | `sp -0x1b4` | `0x0` |
| `str` at 0xaae38 | store | 4 | `sp -0x1b0` | `0x10` |
| `str` at 0xaae40 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xaae48 | store | 4 | `sp -0x118` | `0xff` |
| `str` at 0xaae50 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0xaae58 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xaae5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaae64 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaae74 | store | 4 | `sp -0x1b0` | `0x2` |
| `str` at 0xaae7c | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0xaae84 | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0xaae8c | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xaae90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaae98 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaaea8 | store | 4 | `sp -0x1c` | `ret(0xab0d4)` |
| `ldr` at 0xaaeac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaeb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaebc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaaec0 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xaaec8 | store | 2 | `[[sp-24]+8] +0x6` | `(ret(0xaef64) & 0xffff)` |
| `str` at 0xaaed0 | store | 4 | `sp -0x1b0` | `0x12` |
| `str` at 0xaaed8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xaaedc | load | 4 | `abs 0xab0d0` | `-` |
| `ldr` at 0xaaee0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaee4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xaaee8 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `strh` at 0xaaef0 | store | 2 | `sp -0x1b4` | `([[[sp-24]+8]+6]:2 & 0xffff)` |
| `str` at 0xaaef8 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xaaefc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf04 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaaf10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf14 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xaaf18 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `ldr` at 0xaaf2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf30 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0xaaf38 | store | 2 | `[[sp-24]+8] +0x6` | `0x4400` |
| `ldr` at 0xaaf40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf44 | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0xaaf4c | store | 1 | `[[sp-24]+8] +0x0` | `0x0` |
| `str` at 0xaaf54 | store | 4 | `sp -0x1b0` | `0x13` |
| `str` at 0xaaf5c | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0xaaf64 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xaaf68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf70 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaaf80 | store | 4 | `sp -0x1c` | `ret(0xab0d4)` |
| `ldr` at 0xaaf84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaf9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaafa0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaafa4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaafa8 | load | 1 | `[sp-44] +0x0` | `-` |
| `strb` at 0xaafac | store | 1 | `[[sp-24]+8] +0x0` | `([[sp-44]]:1 & 0xff)` |
| `str` at 0xaafb4 | store | 4 | `sp -0x1b0` | `0x20` |
| `str` at 0xaafbc | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xaafc4 | store | 4 | `sp -0x118` | `0x0` |
| `ldr` at 0xaafcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaafd0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xaafd4 | load | 1 | `[[sp-24]+8] +0x0` | `-` |
| `strb` at 0xaafdc | store | 1 | `(unknown)` | `([[[sp-24]+8]]:1 & 0xff)` |
| `str` at 0xaafe4 | store | 4 | `sp -0x110` | `0x0` |
| `strb` at 0xaaff0 | store | 1 | `sp -0x10c` | `0x0` |
| `str` at 0xaaff8 | store | 4 | `sp -0xc8` | `0x0` |
| `ldr` at 0xaaffc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab004 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xab014 | store | 4 | `sp -0x1c` | `ret(0xab0d4)` |
| `ldr` at 0xab018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab024 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab02c | store | 4 | `sp -0x1b8` | `ret(0xaecd4)` |
| `ldr` at 0xab030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab034 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab038 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xab03c | load | 1 | `ret(0xaecd4) +0x1` | `-` |
| `strb` at 0xab040 | store | 1 | `[[sp-24]+8] +0x1` | `([ret(0xaecd4)+1]:1 & 0xff)` |
| `ldr` at 0xab044 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab048 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab04c | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xab050 | load | 1 | `[sp-440] +0x2` | `-` |
| `strb` at 0xab054 | store | 1 | `[[sp-24]+8] +0x2` | `([[sp-440]+2]:1 & 0xff)` |
| `ldr` at 0xab058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab05c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab060 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xab064 | load | 1 | `[sp-440] +0x3` | `-` |
| `strb` at 0xab068 | store | 1 | `[[sp-24]+8] +0x3` | `([[sp-440]+3]:1 & 0xff)` |
| `ldr` at 0xab06c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab070 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xab074 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0xab080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab084 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xab088 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0xab094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab098 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xab09c | load | 1 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xab0a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab0ac | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab0b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab0b8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0xab0cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xab0cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab0cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab0cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaae10 | `(unknown callee) 0xaa12c` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xaae6c | `(unknown callee) 0xab0d4` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaaea0 | `(unknown callee) 0xab0d4` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaaec4 | `(unknown callee) 0xaef64` | `[sp-44]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaaf0c | `(unknown callee) 0xab0d4` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaaf28 | `(unknown callee) 0xaa12c` | `0x5` | `?` | `?` | `[[[sp-24]+8]+6]:2` |
| `bl` at 0xaaf78 | `(unknown callee) 0xab0d4` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaaf94 | `(unknown callee) 0xaa12c` | `0x5` | `?` | `?` | `ret(0xab0d4)` |
| `bl` at 0xab00c | `(unknown callee) 0xab0d4` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xab028 | `(unknown callee) 0xaecd4` | `[sp-44]` | `?` | `?` | `ret(0xab0d4)` |
| `bl` at 0xab090 | `(unknown callee) 0xae6b0` | `[[[sp-24]+8]+1]:1` | `?` | `[[sp-24]+8]` | `[[[sp-24]+8]+1]:1` |
| `bl` at 0xab0b0 | `(unknown callee) 0xab564` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xab0c0 | `(unknown callee) 0xad394` | `[[sp-24]+8]` | `0x0` | `?` | `[sp-24]` |
| `bl` at 0xab0c8 | `(unknown callee) 0xaa12c` | `0x2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaaeb4 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |
| `bne` at 0xaaf20 | `[[[sp-24]+8]+6]:2 != 0x0` | `[[[sp-24]+8]+6]:2 == 0x0` |
| `bne` at 0xaaf8c | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |
| `beq` at 0xab020 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |
| `beq` at 0xab07c | `[[[sp-24]+8]+1]:1 == 0x0` | `[[[sp-24]+8]+1]:1 != 0x0` |
| `beq` at 0xab0a4 | `[[[sp-24]+8]+2]:1 == 0x0` | `[[[sp-24]+8]+2]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab0cc | `r0` | `ret(0xaa12c)` |

## `PBayPollHwStatus`

- `PBayPollHwStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabaac to 0xac140.
  Established. The unnamed `0x1f26c` call carries `0x55`, `r1`, `r2`, `0x0` (`bl` at 0xabad4); its
  nonzero result takes `0xabaec`, while zero falls through (`bne` at 0xabae4). The unnamed `0x7aad8`
  call at `0xac0d0` carries `0x0`, `[sp-44]+29440`, `[sp-44]+445`, `sp-168` (`bl` at 0xac0d0).
  Hardware status meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xababc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabac4 | store | 4 | `sp -0x4c` | `0x1` |
| `str` at 0xabacc | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xabad8 | store | 4 | `sp -0x18` | `ret(0x1f26c)` |
| `ldr` at 0xabadc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabb00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabb30 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xabb34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabb44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabb4c | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xabb5c | store | 4 | `sp -0x24` | `ret(0xae988)` |
| `ldr` at 0xabb60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xabb7c | store | 4 | `sp -0x20` | `?` |
| `str` at 0xabb84 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xabb8c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xabb90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabba4 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0xabbac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabbb4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xabbbc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xabbcc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xabbdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabbe0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xabbe8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xabbec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabbf0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabc00 | load | 4 | `abs 0xac118` | `-` |
| `str` at 0xabc04 | store | 4 | `sp -0xac` | `0x127bcc` |
| `ldr` at 0xabc08 | load | 4 | `abs 0xac11c` | `-` |
| `str` at 0xabc0c | store | 4 | `sp -0x44` | `0xbbf` |
| `str` at 0xabc14 | store | 4 | `sp -0x48` | `0x7500` |
| `ldr` at 0xabc1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabc20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabc30 | load | 4 | `abs 0xac120` | `-` |
| `str` at 0xabc34 | store | 4 | `sp -0xac` | `0x127bd8` |
| `str` at 0xabc3c | store | 4 | `sp -0x44` | `0xbc0` |
| `str` at 0xabc44 | store | 4 | `sp -0x48` | `0x7400` |
| `ldr` at 0xabc48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabc54 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xabc68 | load | 4 | `abs 0xac124` | `-` |
| `ldr` at 0xabc6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xabc70 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xabc7c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xabc80 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xabc84 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xabc88 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xabc90 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xabc94 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xabca0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabca8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xabcb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabcb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabcc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabcd8 | store | 4 | `sp -0x58` | `0x3` |
| `str` at 0xabce0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xabce4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xabcf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xabd00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabd08 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xabd18 | store | 4 | `sp -0x24` | `ret(0xae988)` |
| `ldr` at 0xabd1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xabd30 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xabd40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xabd50 | load | 4 | `abs 0xac128` | `-` |
| `ldr` at 0xabd54 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xabd60 | store | 1 | `[sp-44] +0x14a754` | `0x0` |
| `str` at 0xabd68 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xabd6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabd74 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xabd80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xabd90 | load | 4 | `abs 0xac128` | `-` |
| `ldr` at 0xabd94 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xabd9c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xabda0 | load | 1 | `[sp-44] +0x14a754` | `-` |
| `strb` at 0xabda8 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xabdac | load | 1 | `sp -0x39` | `-` |
| `str` at 0xabdbc | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xabdc0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabdc8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xabdd4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xabde4 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0xabde8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xabdf0 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xabdf4 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xabe04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabe18 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xabe24 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0xabe34 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xabe3c | load | 4 | `abs 0xac12c` | `-` |
| `str` at 0xabe40 | store | 4 | `sp -0xac` | `0x127bf4` |
| `ldr` at 0xabe44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabe50 | store | 4 | `sp -0x44` | `[sp-44]+427` |
| `ldr` at 0xabe54 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabe5c | store | 4 | `sp -0x48` | `[sp-44]+28672` |
| `ldrb` at 0xabe64 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xabe74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabe84 | load | 4 | `abs 0xac130` | `-` |
| `str` at 0xabe88 | store | 4 | `sp -0xac` | `0x127c10` |
| `ldr` at 0xabe8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabe98 | store | 4 | `sp -0x44` | `[sp-44]+433` |
| `ldr` at 0xabe9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabea4 | store | 4 | `sp -0x48` | `[sp-44]+28928` |
| `ldrb` at 0xabeac | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xabebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabecc | load | 4 | `abs 0xac134` | `-` |
| `str` at 0xabed0 | store | 4 | `sp -0xac` | `0x127c28` |
| `ldr` at 0xabed4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabee0 | store | 4 | `sp -0x44` | `[sp-44]+439` |
| `ldr` at 0xabee4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabeec | store | 4 | `sp -0x48` | `[sp-44]+29184` |
| `str` at 0xabef8 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0xabefc | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xabf10 | load | 4 | `abs 0xac124` | `-` |
| `ldr` at 0xabf14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xabf18 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xabf24 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xabf28 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xabf2c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xabf30 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xabf38 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xabf3c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xabf48 | load | 4 | `abs 0xac128` | `-` |
| `ldr` at 0xabf4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xabf58 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xabf68 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xabf6c | store | 1 | `[sp-44] +0x14a754` | `?` |
| `str` at 0xabf74 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xabf78 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabf80 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xabf88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabf9c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xabfa0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabfb4 | store | 4 | `sp -0x58` | `0x2` |
| `str` at 0xabfbc | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0xabfc0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xabfd4 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xabfd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabfdc | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xabfe0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xabfec | store | 4 | `sp -0x40` | `ret(0xac460)` |
| `ldr` at 0xabff0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xabffc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xac00c | load | 4 | `abs 0xac138` | `-` |
| `ldr` at 0xac010 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac018 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0xac01c | load | 1 | `[sp-44] +0x14a75b` | `-` |
| `strb` at 0xac028 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xac02c | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xac03c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xac050 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xac05c | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0xac06c | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0xac084 | load | 4 | `abs 0xac13c` | `-` |
| `ldr` at 0xac088 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xac090 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac09c | store | 4 | `sp -0x44` | `[sp-44]+445` |
| `ldr` at 0xac0a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac0a8 | store | 4 | `sp -0x48` | `[sp-44]+29440` |
| `ldr` at 0xac0b0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xac0b4 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xac0b8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xac0bc | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xac0c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xac0c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xac0d4 | load | 4 | `abs 0xac138` | `-` |
| `ldr` at 0xac0d8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac0e0 | load | 1 | `sp -0x40` | `-` |
| `strb` at 0xac0e4 | store | 1 | `[sp-44] +0x14a75b` | `([sp-64]:1 & 0xff)` |
| `ldr` at 0xac0ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac0f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xac104 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xac114 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac114 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac114 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabad4 | `(unknown callee) 0x1f26c` | `0x55` | `r1` | `r2` | `0x0` |
| `bl` at 0xabaec | `(unknown callee) 0xaa208` | `ret(0x1f26c)` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0xabb10 | `(unknown callee) 0x22ac8` | `[sp-20]+164` | `0x0` | `?` | `[sp-20]+164` |
| `bl` at 0xabb28 | `(unknown callee) 0xaa12c` | `0x3` | `?` | `?` | `ret(0x22ac8)` |
| `bl` at 0xabb54 | `(unknown callee) 0xae988` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0xabb70 | `(unknown callee) 0x1f26c` | `0x69` | `?` | `?` | `ret(0xae988)` |
| `bl` at 0xabc74 | `(unknown callee) 0x1b52c` | `sp-168` | `0x127be0` | `[sp-44]` | `[sp-172]` |
| `bl` at 0xabc9c | `(unknown callee) 0x7aad8` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0xabcd0 | `(unknown callee) 0x1d41c` | `0x69` | `?` | `[sp-32]` | `?` |
| `bl` at 0xabd10 | `(unknown callee) 0xae988` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0xabf1c | `(unknown callee) 0x1b52c` | `sp-168` | `0x127be0` | `[sp-44]` | `?` |
| `bl` at 0xabf44 | `(unknown callee) 0x7aad8` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0xabfe4 | `(unknown callee) 0xac460` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xac074 | `(unknown callee) 0xac140` | `0x64` | `[sp-88]` | `[sp-88]-1` | `0x1` |
| `bl` at 0xac08c | `(unknown callee) 0x1b52c` | `sp-168` | `0x127c38` | `[sp-44]` | `sp-168` |
| `bl` at 0xac0d0 | `(unknown callee) 0x7aad8` | `0x0` | `[sp-44]+29440` | `[sp-44]+445` | `sp-168` |
| `bl` at 0xac100 | `(unknown callee) 0xaa12c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xac110 | `(unknown callee) 0x22e50` | `[sp-20]+164` | `?` | `?` | `[sp-20]+164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xabae4 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `beq` at 0xabaf8 | `ret(0xaa208) == 0x2` | `ret(0xaa208) != 0x2` |
| `beq` at 0xabb1c | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0xabb40 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabb68 | `ret(0xae988) == 0x0` | `ret(0xae988) != 0x0` |
| `ble` at 0xabb98 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xabbd8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xabbfc | `? != 0x0` | `? == 0x0` |
| `beq` at 0xabc2c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabc50 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xabc5c | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0xabcbc | `Z == 1` | `not (Z == 1)` |
| `ble` at 0xabcec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xabcfc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xabd24 | `ret(0xae988) != 0x0` | `ret(0xae988) == 0x0` |
| `beq` at 0xabd4c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xabdb4 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0xabe00 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabe10 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xabe2c | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |
| `beq` at 0xabe70 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabe80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabeb8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabec8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabf04 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xabf94 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xabfa8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xabfc8 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xabff8 | `ret(0xac460) < 0x0 (signed, 32-bit)` | `ret(0xac460) >= 0x0 (signed, 32-bit)` |
| `bne` at 0xac008 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xac034 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0xac048 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xac064 | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac114 | `r0` | `?` |

## `PBaySdrGetMain`

- `PBaySdrGetMain`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac68c to 0xac8c0.
  Established. The absolute word at `0x14a764` is read (`ldr` at 0xac6b4); its nonzero value takes
  `0xac6cc`, while zero falls through (`bne` at 0xac6bc). The byte assignment at `ret(0xaecd4) +0x3`
  writes `([sp-500]:1 & 0xff)` (`strb` at 0xac834). Main record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac69c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac6a0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xac6a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac6ac | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xac6b0 | load | 4 | `abs 0xac8a0` | `-` |
| `ldr` at 0xac6b4 | load | 4 | `abs 0x14a764` | `-` |
| `str` at 0xac6c4 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0xac6d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac6e0 | store | 4 | `sp -0x1f4` | `ret(0xae988)` |
| `ldr` at 0xac6e4 | load | 4 | `sp -0x1f4` | `-` |
| `str` at 0xac6f4 | store | 4 | `sp -0x200` | `0x0` |
| `str` at 0xac708 | store | 4 | `sp -0x20` | `ret(0x1b1b4)` |
| `ldr` at 0xac70c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac710 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac714 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xac718 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xac720 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `str` at 0xac728 | store | 4 | `sp -0x1f0` | `0x0` |
| `ldr` at 0xac72c | load | 4 | `abs 0xac8a0` | `-` |
| `ldr` at 0xac730 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xac734 | load | 4 | `abs 0x14a764` | `-` |
| `ldr` at 0xac748 | load | 4 | `abs 0xac8a4` | `-` |
| `ldr` at 0xac74c | load | 4 | `sp -0x1f0` | `-` |
| `ldrb` at 0xac764 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac77c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac78c | store | 4 | `sp -0x1ec` | `ret(0xab0d4)` |
| `ldr` at 0xac790 | load | 4 | `sp -0x1ec` | `-` |
| `ldr` at 0xac79c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac7a0 | load | 4 | `abs 0xac8a8` | `-` |
| `str` at 0xac7ac | store | 4 | `sp -0x200` | `?` |
| `ldr` at 0xac7b4 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xac7c8 | load | 4 | `abs 0xac8ac` | `-` |
| `str` at 0xac7dc | store | 4 | `sp -0x24` | `ret(0xae29c)` |
| `ldr` at 0xac7e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac7e4 | load | 1 | `ret(0xae29c) +0x0` | `-` |
| `ldr` at 0xac7f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac7f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac800 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac804 | load | 4 | `abs 0xac8b0` | `-` |
| `ldr` at 0xac80c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac810 | load | 4 | `abs 0xac8b4` | `-` |
| `ldr` at 0xac818 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xac820 | store | 4 | `sp -0x1c` | `ret(0xaecd4)` |
| `ldr` at 0xac824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac828 | load | 4 | `abs 0xac8b8` | `-` |
| `ldrb` at 0xac830 | load | 1 | `sp -0x1f4` | `-` |
| `strb` at 0xac834 | store | 1 | `ret(0xaecd4) +0x3` | `([sp-500]:1 & 0xff)` |
| `ldr` at 0xac838 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xac84c | load | 4 | `abs 0xac8ac` | `-` |
| `ldr` at 0xac858 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac868 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac874 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac878 | load | 4 | `abs 0xac8bc` | `-` |
| `ldr` at 0xac880 | load | 4 | `sp -0x1f0` | `-` |
| `str` at 0xac888 | store | 4 | `sp -0x1f0` | `[sp-496]+1` |
| `str` at 0xac894 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0xac898 | load | 4 | `sp -0x200` | `-` |
| `ldmdb` at 0xac89c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac89c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac89c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac6d8 | `(unknown callee) 0xae988` | `[sp-20]` | `sp-508` | `r1` | `sp-508` |
| `bl` at 0xac700 | `(unknown callee) 0x1b1b4` | `0x100` | `?` | `?` | `ret(0xae988)` |
| `bl` at 0xac774 | `(unknown callee) 0xaddec` | `[sp-20]` | `sp-488` | `?` | `?` |
| `bl` at 0xac784 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-488` | `?` | `sp-488` |
| `bl` at 0xac7a4 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x127cb4` | `?` | `ret(0xab0d4)` |
| `bl` at 0xac7d4 | `(unknown callee) 0xae29c` | `?` | `?` | `?` | `?` |
| `bl` at 0xac7f8 | `(unknown callee) 0x1b364` | `[sp-32]` | `ret(0xae29c)` | `?` | `[ret(0xae29c)]:1` |
| `bl` at 0xac808 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x127d74` | `?` | `[ret(0xae29c)]:1` |
| `bl` at 0xac814 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x127d84` | `?` | `?` |
| `bl` at 0xac81c | `(unknown callee) 0xaecd4` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xac860 | `(unknown callee) 0xae170` | `[sp-28]` | `?` | `sp-76` | `?` |
| `bl` at 0xac870 | `(unknown callee) 0x1b364` | `[sp-32]` | `sp-76` | `?` | `sp-76` |
| `bl` at 0xac87c | `(unknown callee) 0x1b364` | `[sp-32]` | `0x127d20` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xac6bc | `[0x14a764] != 0x0` | `[0x14a764] == 0x0` |
| `bne` at 0xac6ec | `ret(0xae988) != 0x0` | `ret(0xae988) == 0x0` |
| `blt` at 0xac73c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xac798 | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |
| `beq` at 0xac7ec | `[ret(0xae29c)]:1 == 0x0` | `[ret(0xae29c)]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac89c | `r0` | `?` |

## `PBaySdrGetDpsStateHdr`

- `PBaySdrGetDpsStateHdr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac168 to 0xac1b4.
  Established. The unnamed `0x1b1b4` call carries `0x50`, `r1`, `r2`, `r3` (`bl` at 0xac17c). The
  unnamed `0x1b598` call uses `ret(0x1b1b4)`, `0x127c68`, unknown, `ret(0x1b1b4)` (`bl` at 0xac190).
  The return reports `[sp-20]` (`ldmdb` at 0xac1a8). Header contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac184 | store | 4 | `sp -0x14` | `ret(0x1b1b4)` |
| `ldr` at 0xac188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac18c | load | 4 | `abs 0xac1ac` | `-` |
| `ldr` at 0xac194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac198 | load | 4 | `abs 0xac1b0` | `-` |
| `ldr` at 0xac1a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xac1a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac1a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac1a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac17c | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xac190 | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `0x127c68` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xac19c | `(unknown callee) 0x1b364` | `[sp-20]` | `0x127c88` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac1a8 | `r0` | `[sp-20]` |

## `PBaySdrGetDpsState`

- `PBaySdrGetDpsState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac1b4 to 0xac414.
  Established. The signed `[sp-24] <= 0x0` condition takes `0xac1f8`, and positive values fall
  through (`ble` at 0xac1e4). The byte assignment at `[sp-40] +0x1` writes `([sp-24]:1+48 & 0xff)`
  (`strb` at 0xac270). The unnamed `0xab0d4` call carries `[sp-20]`, `sp-456`, unknown, `sp-456`
  (`bl` at 0xac29c). Supply-state bit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac1c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac1c8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac1cc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xac1d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac1d8 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0xac1dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac1e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac1fc | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0xac204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac20c | load | 4 | `abs 0xac3e0` | `-` |
| `str` at 0xac214 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xac218 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac21c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xac22c | store | 4 | `sp -0x1d0` | `0x0` |
| `str` at 0xac240 | store | 4 | `sp -0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xac244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac248 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xac24c | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xac250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac254 | load | 4 | `abs 0xac3e4` | `-` |
| `ldr` at 0xac260 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xac268 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xac270 | store | 1 | `[sp-40] +0x1` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0xac278 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac27c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xac280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac294 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac2a4 | store | 4 | `sp -0x1cc` | `ret(0xab0d4)` |
| `ldr` at 0xac2a8 | load | 4 | `sp -0x1cc` | `-` |
| `ldr` at 0xac2b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac2b8 | load | 4 | `abs 0xac3e8` | `-` |
| `str` at 0xac2c4 | store | 4 | `sp -0x1d0` | `?` |
| `ldr` at 0xac2cc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xac2d4 | store | 4 | `sp -0x24` | `ret(0xaecd4)` |
| `ldr` at 0xac2d8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac2dc | load | 1 | `ret(0xaecd4) +0x2` | `-` |
| `strb` at 0xac2e0 | store | 1 | `sp -0x2d` | `([ret(0xaecd4)+2]:1 & 0xff)` |
| `ldrb` at 0xac2e4 | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0xac2f4 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xac304 | load | 4 | `abs 0xac3ec` | `-` |
| `str` at 0xac308 | store | 4 | `sp -0x2c` | `0x127cc4` |
| `ldr` at 0xac310 | load | 4 | `abs 0xac3f0` | `-` |
| `str` at 0xac314 | store | 4 | `sp -0x2c` | `0x127ccc` |
| `ldr` at 0xac318 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac31c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac324 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xac334 | load | 4 | `abs 0xac3f4` | `-` |
| `str` at 0xac338 | store | 4 | `sp -0x2c` | `0x127cd4` |
| `ldr` at 0xac340 | load | 4 | `abs 0xac3f8` | `-` |
| `str` at 0xac344 | store | 4 | `sp -0x2c` | `0x127ce0` |
| `ldr` at 0xac348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac34c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac354 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xac364 | load | 4 | `abs 0xac3fc` | `-` |
| `str` at 0xac368 | store | 4 | `sp -0x2c` | `0x127cec` |
| `ldr` at 0xac370 | load | 4 | `abs 0xac400` | `-` |
| `str` at 0xac374 | store | 4 | `sp -0x2c` | `0x127cf8` |
| `ldr` at 0xac378 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac37c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac384 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xac394 | load | 4 | `abs 0xac404` | `-` |
| `str` at 0xac398 | store | 4 | `sp -0x2c` | `0x127d04` |
| `ldr` at 0xac3a0 | load | 4 | `abs 0xac408` | `-` |
| `str` at 0xac3a4 | store | 4 | `sp -0x2c` | `0x127d0c` |
| `ldr` at 0xac3a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac3ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xac3b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac3bc | load | 4 | `abs 0xac40c` | `-` |
| `ldr` at 0xac3c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac3c8 | load | 4 | `abs 0xac410` | `-` |
| `str` at 0xac3d4 | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0xac3d8 | load | 4 | `sp -0x1d0` | `-` |
| `ldmdb` at 0xac3dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac3dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac3dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac238 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xac258 | `(unknown callee) 0x1b598` | `[sp-40]` | `0x127cac` | `[sp-28]` | `ret(0x1b1b4)` |
| `bl` at 0xac28c | `(unknown callee) 0xaddec` | `[sp-20]` | `sp-456` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0xac29c | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-456` | `?` | `sp-456` |
| `bl` at 0xac2bc | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `?` | `ret(0xab0d4)` |
| `bl` at 0xac2d0 | `(unknown callee) 0xaecd4` | `[sp-64]` | `?` | `?` | `ret(0xab0d4)` |
| `bl` at 0xac320 | `(unknown callee) 0x1b364` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xac350 | `(unknown callee) 0x1b364` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xac380 | `(unknown callee) 0x1b364` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xac3b0 | `(unknown callee) 0x1b364` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xac3c0 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127d14` | `?` | `?` |
| `bl` at 0xac3cc | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127d20` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xac1e4 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bgt` at 0xac1f0 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `bne` at 0xac224 | `[[sp-32]]:1 != 0xff` | `[[sp-32]]:1 == 0xff` |
| `bne` at 0xac2b0 | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |
| `bne` at 0xac2f0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xac300 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xac330 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xac360 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xac390 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac3dc | `r0` | `?` |

## `PBaySdrGetFruHdr`

- `PBaySdrGetFruHdr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac8c0 to 0xac90c.
  Established. The unnamed `0x1b1b4` call carries `0xa0`, `r1`, `r2`, `r3` (`bl` at 0xac8d4); its
  result is assigned as a word at `sp -0x14` (`str` at 0xac8dc). The unnamed `0x1b364` call carries
  `[sp-20]`, `0x127dcc`, unknown, unknown (`bl` at 0xac8f4). FRU header meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac8dc | store | 4 | `sp -0x14` | `ret(0x1b1b4)` |
| `ldr` at 0xac8e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac8e4 | load | 4 | `abs 0xac904` | `-` |
| `ldr` at 0xac8ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac8f0 | load | 4 | `abs 0xac908` | `-` |
| `ldr` at 0xac8f8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xac900 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac900 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac900 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac8d4 | `(unknown callee) 0x1b1b4` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xac8e8 | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `0x127d88` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xac8f4 | `(unknown callee) 0x1b364` | `[sp-20]` | `0x127dcc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac900 | `r0` | `[sp-20]` |

## `PBaySdrGetFru`

- `PBaySdrGetFru`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac90c to 0xad394.
  Established. The initial word assignment through `r3 +0x0` writes `0x0` (`str` at 0xac934). The
  unnamed `0x1b1b4` call carries `0x400`, unknown, unknown, unknown (`bl` at 0xaca84). The byte
  assignment at `sp -0x2d3` writes `0x8` (`strb` at 0xacb14). FRU data interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac91c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac920 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac924 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xac928 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xac92c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac934 | store | 4 | `r3 +0x0` | `0x0` |
| `ldr` at 0xac938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac944 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac958 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xac960 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac96c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac974 | load | 4 | `abs 0xad34c` | `-` |
| `str` at 0xac97c | store | 4 | `sp -0x130` | `?` |
| `ldr` at 0xac984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac990 | load | 4 | `abs 0xad350` | `-` |
| `str` at 0xac994 | store | 4 | `sp -0x130` | `0x143678` |
| `ldr` at 0xac99c | load | 4 | `abs 0xad34c` | `-` |
| `str` at 0xac9a0 | store | 4 | `sp -0x130` | `0x14365c` |
| `ldr` at 0xac9a4 | load | 4 | `sp -0x130` | `-` |
| `ldrb` at 0xac9a8 | load | 1 | `[sp-304] +0x0` | `-` |
| `str` at 0xac9b8 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xac9c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac9cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac9d4 | load | 4 | `abs 0xad354` | `-` |
| `str` at 0xac9dc | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0xac9e0 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0xac9e4 | load | 1 | `[sp-740] +0x0` | `-` |
| `str` at 0xac9f4 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xaca00 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0xaca04 | load | 1 | `[sp-740] +0x0` | `-` |
| `ldr` at 0xaca08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaca1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaca2c | store | 4 | `sp -0x2cc` | `ret(0xab0d4)` |
| `ldr` at 0xaca30 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xaca3c | load | 4 | `sp -0x140` | `-` |
| `str` at 0xaca44 | store | 4 | `sp -0x2e8` | `ret(0xaecd4)` |
| `ldr` at 0xaca4c | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xaca50 | load | 1 | `[sp-744] +0x2` | `-` |
| `strb` at 0xaca58 | store | 1 | `(unknown)` | `([[sp-744]+2]:1 & 0xff)` |
| `ldrb` at 0xaca64 | load | 1 | `(unknown)` | `-` |
| `str` at 0xaca78 | store | 4 | `sp -0x300` | `0x0` |
| `str` at 0xaca8c | store | 4 | `sp -0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xaca90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaca94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaca98 | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xaca9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacaa8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacaac | load | 4 | `abs 0xad358` | `-` |
| `ldr` at 0xacab8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xacac0 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xacac8 | store | 1 | `[sp-40] +0x1` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0xacad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacadc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacae0 | load | 4 | `abs 0xad35c` | `-` |
| `ldr` at 0xacaec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacaf0 | load | 4 | `abs 0xad360` | `-` |
| `ldr` at 0xacaf8 | load | 4 | `abs 0xad364` | `-` |
| `strh` at 0xacb04 | store | 2 | `sp -0x2d2` | `0x0` |
| `ldr` at 0xacb08 | load | 4 | `abs 0xad368` | `-` |
| `strb` at 0xacb14 | store | 1 | `sp -0x2d3` | `0x8` |
| `ldr` at 0xacb1c | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xacb24 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xacb28 | load | 4 | `abs 0xad368` | `-` |
| `ldrb` at 0xacb30 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xacb34 | store | 4 | `sp -0x308` | `0x8` |
| `ldr` at 0xacb38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacb40 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xacb50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacb60 | store | 4 | `sp -0x2cc` | `ret(0xab0d4)` |
| `ldr` at 0xacb64 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xacb70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacb74 | load | 4 | `abs 0xad36c` | `-` |
| `str` at 0xacb80 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xacb88 | load | 4 | `sp -0x140` | `-` |
| `str` at 0xacb90 | store | 4 | `sp -0x2ec` | `ret(0xaecd4)` |
| `ldr` at 0xacb98 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xacbb0 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xacbbc | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0xacbc8 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xacbd4 | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0xacbe0 | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xacbe4 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xacbf0 | load | 1 | `sp -0x2f0` | `-` |
| `strh` at 0xacbfc | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0xacc00 | load | 4 | `abs 0xad368` | `-` |
| `strb` at 0xacc0c | store | 1 | `sp -0x2d3` | `0x3` |
| `ldr` at 0xacc14 | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xacc1c | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xacc20 | load | 4 | `abs 0xad368` | `-` |
| `ldrb` at 0xacc28 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xacc2c | store | 4 | `sp -0x308` | `0x3` |
| `str` at 0xacc34 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xacc38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacc40 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xacc50 | load | 4 | `abs 0xad374` | `-` |
| `strb` at 0xacc58 | store | 1 | `sp -0x2d4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xacc5c | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xacc64 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xacc70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacc74 | load | 4 | `abs 0xad36c` | `-` |
| `str` at 0xacc80 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xacc88 | load | 4 | `abs 0xad368` | `-` |
| `ldr` at 0xacc8c | load | 4 | `abs 0xad378` | `-` |
| `ldrb` at 0xacc94 | load | 1 | `sp -0x127` | `-` |
| `strb` at 0xacca0 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0xacca4 | load | 4 | `abs 0xad368` | `-` |
| `ldr` at 0xacca8 | load | 4 | `abs 0xad368` | `-` |
| `ldr` at 0xaccac | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xaccb4 | load | 1 | `sp -0x2d3` | `-` |
| `ldrb` at 0xaccbc | load | 1 | `sp -0x2d4` | `-` |
| `strb` at 0xaccc8 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0xacccc | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xaccd0 | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xaccd8 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xacce4 | store | 2 | `sp -0x2d2` | `([sp-722]:2+3 & 0xffff)` |
| `ldr` at 0xaccec | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xaccf4 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xaccf8 | load | 4 | `abs 0xad368` | `-` |
| `ldrb` at 0xacd00 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xacd04 | store | 4 | `sp -0x308` | `[sp-723]:1` |
| `str` at 0xacd0c | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xacd10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacd18 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xacd28 | load | 4 | `abs 0xad374` | `-` |
| `strb` at 0xacd30 | store | 1 | `sp -0x2d4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xacd34 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xacd3c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xacd48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacd4c | load | 4 | `abs 0xad36c` | `-` |
| `str` at 0xacd58 | store | 4 | `sp -0x300` | `?` |
| `str` at 0xacd64 | store | 4 | `sp -0x12c` | `0x0` |
| `str` at 0xacd6c | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0xacd70 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xacd80 | load | 4 | `abs 0xad37c` | `-` |
| `ldr` at 0xacd84 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xacd94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xacda8 | load | 4 | `abs 0xad37c` | `-` |
| `ldr` at 0xacdac | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xacdbc | load | 1 | `(sp-16 + [sp-300]) -0x118` | `-` |
| `strb` at 0xacdc8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xacdcc | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xacdd4 | store | 4 | `sp -0x12c` | `[sp-300]+1` |
| `ldr` at 0xacdd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacde8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xacdf0 | store | 4 | `sp -0x24` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0xacdf8 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xace08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xace0c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xace24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xace28 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xace30 | store | 4 | `sp -0x12c` | `?` |
| `ldrb` at 0xace3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xace40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xace48 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xace50 | load | 4 | `abs 0xad380` | `-` |
| `ldr` at 0xace54 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xace60 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xace64 | load | 1 | `[sp-720] +0x1436ac` | `-` |
| `ldr` at 0xace70 | load | 4 | `abs 0xad380` | `-` |
| `ldr` at 0xace74 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xace7c | load | 1 | `[sp-720] +0x1436ac` | `-` |
| `ldrb` at 0xace88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xace90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xacea0 | load | 4 | `abs 0xad380` | `-` |
| `ldr` at 0xacea4 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xaceac | load | 1 | `[sp-720] +0x1436ac` | `-` |
| `ldrb` at 0xaceb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xacec0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xacec8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xaced0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xacedc | store | 1 | `[sp-36] +0x0` | `0x20` |
| `str` at 0xacee4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xacee8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xacef0 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `ldr` at 0xacef4 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0xacefc | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0xacf04 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xacf10 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0xacf1c | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xacf28 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0xacf34 | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xacf38 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xacf44 | load | 1 | `sp -0x2f1` | `-` |
| `strh` at 0xacf50 | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0xacf54 | load | 4 | `abs 0xad368` | `-` |
| `strb` at 0xacf60 | store | 1 | `sp -0x2d3` | `0x6` |
| `ldr` at 0xacf68 | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xacf70 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xacf74 | load | 4 | `abs 0xad368` | `-` |
| `ldrb` at 0xacf7c | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xacf80 | store | 4 | `sp -0x308` | `0x6` |
| `str` at 0xacf88 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xacf8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacf94 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xacfa4 | load | 4 | `abs 0xad374` | `-` |
| `strb` at 0xacfac | store | 1 | `sp -0x2d4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xacfb0 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xacfb8 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xacfc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacfc8 | load | 4 | `abs 0xad36c` | `-` |
| `str` at 0xacfd4 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xacfdc | load | 4 | `abs 0xad384` | `-` |
| `ldrb` at 0xacfe4 | load | 1 | `sp -0x123` | `-` |
| `str` at 0xacfe8 | store | 4 | `sp -0x2dc` | `[sp-291]:1` |
| `ldr` at 0xacfec | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xacff4 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xacff8 | load | 4 | `abs 0xad388` | `-` |
| `ldrb` at 0xad000 | load | 1 | `sp -0x124` | `-` |
| `ldr` at 0xad004 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xad00c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xad010 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xad018 | store | 4 | `sp -0x2dc` | `?` |
| `ldrb` at 0xad024 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xad028 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xad030 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xad034 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xad040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad04c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xad058 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xad064 | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0xad070 | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xad07c | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0xad088 | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xad08c | load | 4 | `abs 0xad370` | `-` |
| `ldrb` at 0xad098 | load | 1 | `sp -0x2ef` | `-` |
| `strh` at 0xad0a4 | store | 2 | `sp -0x2d2` | `?` |
| `str` at 0xad0ac | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0xad0b4 | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xad0bc | load | 2 | `sp -0x2d2` | `-` |
| `str` at 0xad0c4 | store | 4 | `sp -0x308` | `0x8` |
| `str` at 0xad0cc | store | 4 | `sp -0x304` | `sp-764` |
| `ldr` at 0xad0d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad0d8 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xad0e8 | load | 4 | `abs 0xad374` | `-` |
| `strb` at 0xad0f0 | store | 1 | `sp -0x2d4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xad0f4 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xad0fc | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xad108 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad10c | load | 4 | `abs 0xad36c` | `-` |
| `str` at 0xad118 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xad140 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad144 | load | 4 | `abs 0xad38c` | `-` |
| `str` at 0xad150 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xad158 | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xad15c | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xad160 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xad168 | load | 1 | `sp -0x2d4` | `-` |
| `ldrh` at 0xad174 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xad180 | store | 2 | `sp -0x2d2` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `ldr` at 0xad188 | load | 4 | `abs 0xad364` | `-` |
| `ldrh` at 0xad190 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xad194 | load | 4 | `abs 0xad390` | `-` |
| `ldrb` at 0xad1a0 | load | 1 | `sp -0x2fa` | `-` |
| `str` at 0xad1a4 | store | 4 | `sp -0x308` | `[sp-762]:1` |
| `str` at 0xad1ac | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xad1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad1b8 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xad1c8 | load | 4 | `abs 0xad374` | `-` |
| `strb` at 0xad1d0 | store | 1 | `sp -0x2d4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xad1d4 | load | 4 | `abs 0xad374` | `-` |
| `ldr` at 0xad1d8 | load | 4 | `abs 0xad390` | `-` |
| `ldrb` at 0xad1e8 | load | 1 | `sp -0x2d4` | `-` |
| `ldrb` at 0xad1ec | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0xad1f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad1fc | load | 4 | `abs 0xad36c` | `-` |
| `str` at 0xad208 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xad210 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xad218 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xad220 | load | 4 | `abs 0xad37c` | `-` |
| `ldr` at 0xad230 | load | 4 | `abs 0xad390` | `-` |
| `ldrb` at 0xad23c | load | 1 | `sp -0x2f9` | `-` |
| `strb` at 0xad240 | store | 1 | `(unknown)` | `([sp-761]:1 & 0xff)` |
| `ldr` at 0xad248 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xad250 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xad274 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad278 | load | 4 | `abs 0xad38c` | `-` |
| `str` at 0xad284 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xad28c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad29c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xad2a4 | store | 4 | `sp -0x24` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0xad2a8 | load | 4 | `abs 0xad390` | `-` |
| `ldrb` at 0xad2b4 | load | 1 | `sp -0x2fc` | `-` |
| `ldr` at 0xad2b8 | load | 4 | `abs 0xad390` | `-` |
| `ldrb` at 0xad2c4 | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0xad2d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad2dc | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xad2e0 | load | 4 | `abs 0xad364` | `-` |
| `ldr` at 0xad2e4 | load | 4 | `abs 0xad374` | `-` |
| `ldrb` at 0xad2ec | load | 1 | `sp -0x2d4` | `-` |
| `ldrh` at 0xad2f8 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xad304 | store | 2 | `sp -0x2d2` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `ldr` at 0xad308 | load | 4 | `abs 0xad390` | `-` |
| `ldrsb` at 0xad314 | load | 1 | `sp -0x2fb` | `-` |
| `ldr` at 0xad324 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0xad330 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `str` at 0xad340 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xad344 | load | 4 | `sp -0x300` | `-` |
| `ldmdb` at 0xad348 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad348 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad348 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaca14 | `(unknown callee) 0xaddec` | `[sp-20]` | `sp-712` | `[[sp-740]]:1` | `[[sp-740]]:1` |
| `bl` at 0xaca24 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0xaca40 | `(unknown callee) 0xaecd4` | `[sp-320]` | `?` | `?` | `ret(0xab0d4)` |
| `bl` at 0xaca84 | `(unknown callee) 0x1b1b4` | `0x400` | `?` | `?` | `?` |
| `bl` at 0xacab0 | `(unknown callee) 0x1b598` | `[sp-40]` | `0x127e10` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xacae4 | `(unknown callee) 0x1b598` | `[sp-40]` | `0x127e18` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xacaf4 | `(unknown callee) 0x1b598` | `[sp-40]` | `0x127e20` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xacb48 | `(unknown callee) 0xadf0c` | `[sp-20]` | `sp-712` | `[sp-304]` | `0x0` |
| `bl` at 0xacb58 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0xacb78 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `?` | `ret(0xab0d4)` |
| `bl` at 0xacb8c | `(unknown callee) 0xaecd4` | `[sp-320]` | `?` | `?` | `ret(0xab0d4)` |
| `bl` at 0xacbac | `(unknown callee) 0x1b394` | `sp-756` | `ret(0xaecd4)+1` | `0x8` | `ret(0xaecd4)+1` |
| `bl` at 0xacc48 | `(unknown callee) 0xae028` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xacc78 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `ret(0xae028)` | `(ret(0xae028) & 0xff)` |
| `bl` at 0xacd20 | `(unknown callee) 0xae028` | `[sp-20]` | `sp-712` | `[sp-304]` | `([sp-722]:2+3 & 0xffff)` |
| `bl` at 0xacd50 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `sp-16` | `(ret(0xae028) & 0xff)` |
| `bl` at 0xacddc | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `sp-16` | `[sp-300]+1` |
| `bl` at 0xace18 | `(unknown callee) 0x1b394` | `[sp-36]` | `(sp-296 + [sp-300])` | `?` | `?` |
| `bl` at 0xace9c | `(unknown callee) 0x1b4cc` | `[sp-36]` | `0x20` | `?` | `?` |
| `bl` at 0xacf9c | `(unknown callee) 0xae028` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xacfcc | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `ret(0xae028)` | `(ret(0xae028) & 0xff)` |
| `bl` at 0xad038 | `(unknown callee) 0xae5e4` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0xad048 | `(unknown callee) 0x1b364` | `[sp-40]` | `ret(0xae5e4)` | `?` | `ret(0xae5e4)` |
| `bl` at 0xad0e0 | `(unknown callee) 0xae028` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xad110 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `ret(0xae028)` | `(ret(0xae028) & 0xff)` |
| `bl` at 0xad12c | `(unknown callee) 0xae63c` | `sp-764` | `0x8` | `ret(0xae028)` | `sp-764` |
| `bl` at 0xad148 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127e28` | `?` | `?` |
| `bl` at 0xad1c0 | `(unknown callee) 0xae028` | `[sp-20]` | `sp-712` | `[sp-304]` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `bl` at 0xad200 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `(ret(0xae028) & 0xff)` | `[sp-762]:1` |
| `bl` at 0xad260 | `(unknown callee) 0xae63c` | `sp-296` | `[sp-724]:1+1` | `sp-296` | `[sp-724]:1+1` |
| `bl` at 0xad27c | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127e28` | `?` | `?` |
| `bl` at 0xad290 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xad2d8 | `(unknown callee) 0xb3b60` | `[sp-764]:1` | `[sp-762]:1` | `sp-296` | `([sp-40] + ret(0x1b2bc))` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xac940 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0xac94c | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `ble` at 0xac968 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0xac98c | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xac9b0 | `[[sp-304]]:1 != 0xff` | `[[sp-304]]:1 == 0xff` |
| `ble` at 0xac9c8 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bne` at 0xac9ec | `[[sp-740]]:1 != 0xff` | `[[sp-740]]:1 == 0xff` |
| `beq` at 0xaca38 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |
| `beq` at 0xaca70 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xacaa4 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0xacad8 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xacb6c | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |
| `beq` at 0xacbc4 | `[sp-752]:1 == 0x0` | `[sp-752]:1 != 0x0` |
| `beq` at 0xacbdc | `[sp-752]:1 == 0xff` | `[sp-752]:1 != 0xff` |
| `bne` at 0xacc6c | `(ret(0xae028) & 0xff) != 0x0` | `(ret(0xae028) & 0xff) == 0x0` |
| `bne` at 0xacd44 | `(ret(0xae028) & 0xff) != 0x0` | `(ret(0xae028) & 0xff) == 0x0` |
| `ble` at 0xacd78 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xacd9c | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xace6c | `? >= [[sp-720]+1324716]:1 (unsigned, 32-bit)` | `? < [[sp-720]+1324716]:1 (unsigned, 32-bit)` |
| `beq` at 0xacf18 | `[sp-753]:1 == 0x0` | `[sp-753]:1 != 0x0` |
| `beq` at 0xacf30 | `[sp-753]:1 == 0xff` | `[sp-753]:1 != 0xff` |
| `bne` at 0xacfc0 | `(ret(0xae028) & 0xff) != 0x0` | `(ret(0xae028) & 0xff) == 0x0` |
| `beq` at 0xad054 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xad06c | `[sp-751]:1 == 0x0` | `[sp-751]:1 != 0x0` |
| `beq` at 0xad084 | `[sp-751]:1 == 0xff` | `[sp-751]:1 != 0xff` |
| `beq` at 0xad104 | `(ret(0xae028) & 0xff) == 0x8` | `(ret(0xae028) & 0xff) != 0x8` |
| `beq` at 0xad13c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xad1f4 | `(ret(0xae028) & 0xff) == [sp-762]:1` | `(ret(0xae028) & 0xff) != [sp-762]:1` |
| `beq` at 0xad270 | `? == 0x0` | `? != 0x0` |
| `blt` at 0xad31c | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `ble` at 0xad338 | `[sp-720] <= 0x4 (signed, 32-bit)` | `[sp-720] > 0x4 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad348 | `r0` | `?` |

## `PBaySdrGetConnector`

- `PBaySdrGetConnector`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae4b8 to 0xae5e4.
  Established. The halfword assignment at `sp -0x1ac` writes `([[sp-20]+6]:2 & 0xffff)` (`strh` at
  0xae4ec). The unnamed `0xab0d4` call carries `[sp-20]`, `sp-428`, `0xfffffe6c`, `sp-428` (`bl` at
  0xae534); its zero result takes `0xae5c8`, and nonzero falls through (`beq` at 0xae548). Connector
  record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae4c8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xae4d8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xae4dc | load | 4 | `abs 0xae5dc` | `-` |
| `ldr` at 0xae4e0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xae4e4 | load | 2 | `[sp-20] +0x6` | `-` |
| `strh` at 0xae4ec | store | 2 | `sp -0x1ac` | `([[sp-20]+6]:2 & 0xffff)` |
| `str` at 0xae4f4 | store | 4 | `sp -0x1a8` | `0xa` |
| `str` at 0xae4fc | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xae504 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xae50c | store | 4 | `sp -0x114` | `0x1` |
| `ldr` at 0xae510 | load | 4 | `abs 0xae5e0` | `-` |
| `strb` at 0xae51c | store | 1 | `sp -0x1a4` | `0x0` |
| `str` at 0xae524 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xae52c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae53c | store | 4 | `sp -0x1b0` | `ret(0xab0d4)` |
| `ldr` at 0xae540 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xae54c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xae550 | store | 4 | `sp -0x1b4` | `[sp-36]` |
| `ldr` at 0xae554 | load | 4 | `sp -0x1b4` | `-` |
| `ldrb` at 0xae558 | load | 1 | `[sp-36] +0x0` | `-` |
| `str` at 0xae56c | store | 4 | `sp -0x1a8` | `0xb` |
| `str` at 0xae574 | store | 4 | `sp -0x114` | `0x0` |
| `ldr` at 0xae57c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae58c | store | 4 | `sp -0x1b0` | `ret(0xab0d4)` |
| `ldr` at 0xae590 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xae59c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xae5a0 | store | 4 | `sp -0x1b8` | `[sp-36]` |
| `ldr` at 0xae5a4 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xae5a8 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae5b8 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xae5bc | load | 1 | `[sp-440] +0x0` | `-` |
| `strb` at 0xae5c4 | store | 1 | `(unknown)` | `([[sp-440]]:1 & 0xff)` |
| `ldrb` at 0xae5d0 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0xae5d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae5d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae5d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae534 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-428` | `0xfffffe6c` | `sp-428` |
| `bl` at 0xae584 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-428` | `?` | `sp-428` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae548 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |
| `beq` at 0xae564 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xae598 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |
| `beq` at 0xae5b0 | `[[sp-36]]:1 == 0x0` | `[[sp-36]]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae5d8 | `r0` | `?` |

## `PBaySdrGetNbrDps`

- `PBaySdrGetNbrDps`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae988 to 0xaeab4.
  Established. The unnamed `0xab0d4` call carries `[sp-20]`, `sp-452`, unknown, `sp-452` (`bl` at
  0xaea24); its nonzero result takes `0xaea48`, while zero falls through (`bne` at 0xaea38). A byte
  assignment at `([sp-24] + [sp-32]) +0x0` writes `(([ret(0xaecd4)+2]:1 & 0xff) & 0xff)` (`strb` at
  0xaea70). Supply count encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae998 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xae99c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xae9a4 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xae9ac | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xae9b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xae9c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xae9c8 | load | 4 | `abs 0xaeab0` | `-` |
| `str` at 0xae9d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xae9d4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xae9d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xae9e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae9e8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xae9f4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xaea00 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xaea04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xaea08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaea1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaea2c | store | 4 | `sp -0x1c8` | `ret(0xab0d4)` |
| `ldr` at 0xaea30 | load | 4 | `sp -0x1c8` | `-` |
| `str` at 0xaea40 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xaea48 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xaea50 | store | 4 | `sp -0x28` | `ret(0xaecd4)` |
| `ldr` at 0xaea54 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xaea58 | load | 1 | `ret(0xaecd4) +0x2` | `-` |
| `strb` at 0xaea5c | store | 1 | `sp -0x29` | `([ret(0xaecd4)+2]:1 & 0xff)` |
| `ldr` at 0xaea60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaea64 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xaea6c | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xaea70 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `(([ret(0xaecd4)+2]:1 & 0xff) & 0xff)` |
| `ldrb` at 0xaea74 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xaea84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaea8c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xaea90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaea98 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xaeaa0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaeaa4 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xaeaa8 | load | 4 | `sp -0x1cc` | `-` |
| `ldmdb` at 0xaeaac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaeaac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaeaac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaea14 | `(unknown callee) 0xaddec` | `?` | `sp-452` | `?` | `?` |
| `bl` at 0xaea24 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-452` | `?` | `sp-452` |
| `bl` at 0xaea4c | `(unknown callee) 0xaecd4` | `[sp-60]` | `?` | `?` | `ret(0xab0d4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xae9b8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xae9e0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xaea38 | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |
| `bne` at 0xaea80 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaeaac | `r0` | `?` |
