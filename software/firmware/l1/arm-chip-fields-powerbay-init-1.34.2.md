# ARM chip fields and power-bay initialization in 1.34.2

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
PBaySetState reports one unresolved computed transfer; its unknown destinations are excluded from the summaries. Other routines report no unresolved computed transfers. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/chip-fields-powerbay-init-facts.txt) retain all tool sections.

## `jchip_dump_io`

- `jchip_dump_io`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca8c0 to 0xca99c.
  Established. The first call to `0x18c74` carries `sp-72`, `0x10ac38`, entry r0 and entry r1 (`bl`
  at 0xca8f4). The later call to that target uses `sp-72`, `0x10ac44`, `[sp-20]`, `[sp-24]` (`bl` at
  0xca94c). Calls to `0xd2e88` have candidates `0x0`, `sp-72`, unknown, `[sp-32]` and later `0x0`,
  `sp-72`, unknown, `[sp]` (`bl` at 0xca934, `bl` at 0xca98c). No conditional branches are reported.
  The final return reports `ret(0xd2e88)`, with call purposes unknown (`ldmdb` at 0xca990).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca8d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca8d4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca8d8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xca8dc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xca8e8 | load | 4 | `abs 0xca994` | `-` |
| `ldr` at 0xca8ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca8f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca8fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca930 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca940 | load | 4 | `abs 0xca998` | `-` |
| `ldr` at 0xca944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca948 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca968 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca988 | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0xca990 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca990 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca990 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca8f4 | `(unknown callee) 0x18c74` | `sp-72` | `0x10ac38` | `r0` | `r1` |
| `bl` at 0xca934 | `(unknown callee) 0xd2e88` | `0x0` | `sp-72` | `?` | `[sp-32]` |
| `bl` at 0xca94c | `(unknown callee) 0x18c74` | `sp-72` | `0x10ac44` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xca98c | `(unknown callee) 0xd2e88` | `0x0` | `sp-72` | `?` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca990 | `r0` | `ret(0xd2e88)` |

## `JChip_protect_field`

- `JChip_protect_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca99c to 0xcab20.
  Established. The call to `0xca498` has four unknown candidates (`bl` at 0xca9e4); its nonzero
  result takes the continuation and zero falls through to word `0xfffff7c6` at `sp -0x28` (`bne`
  at 0xca9f8, `str` at 0xcaa00). The later call to `0xcf7a4` carries unknown, `[sp-28]`, `[sp-32]`,
  unknown (`bl` at 0xcaaa0). A final byte store has unknown effective target and value (`strb` at
  0xcaaf4). Field protection semantics and the merged return remain unknown (`ldmdb` at 0xcab04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca9ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca9b0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca9b4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xca9b8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xca9bc | load | 4 | `abs 0xcab08` | `-` |
| `ldr` at 0xca9c0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xca9cc | load | 4 | `abs 0xcab0c` | `-` |
| `ldr` at 0xca9d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca9d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca9d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca9e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca9ec | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xca9f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca9fc | load | 4 | `abs 0xcab10` | `-` |
| `str` at 0xcaa00 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xcaa08 | load | 4 | `abs 0xcab08` | `-` |
| `ldr` at 0xcaa0c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcaa18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaa1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaa24 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcaa2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaa30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaa34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaa3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaa44 | load | 4 | `abs 0xcab14` | `-` |
| `ldr` at 0xcaa48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaa50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcaa5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcaa60 | load | 4 | `abs 0xcab18` | `-` |
| `ldr` at 0xcaa64 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcaa6c | load | 4 | `abs 0xcab1c` | `-` |
| `ldr` at 0xcaa74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcaa80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaa84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaa8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaa94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaa98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaa9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcaaa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaaa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaab0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaab8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaabc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaac8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaacc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaad4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaadc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaae0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaae4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcaaec | load | 1 | `(unknown)` | `-` |
| `strb` at 0xcaaf4 | store | 1 | `(unknown)` | `?` |
| `str` at 0xcaafc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcab00 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcab04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcab04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcab04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca9dc | `(unknown callee) 0x20f14` | `0x10ac50` | `r0` | `r1` | `r2` |
| `bl` at 0xca9e4 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcaa4c | `(unknown callee) 0x20f14` | `0x10ac88` | `?` | `?` | `?` |
| `bl` at 0xcaa68 | `(unknown callee) 0x20f14` | `0x10aca8` | `[[sp-32]]` | `?` | `[sp-32]` |
| `bl` at 0xcaa70 | `(unknown callee) 0x20f14` | `0x10acbc` | `?` | `?` | `?` |
| `bl` at 0xcaaa0 | `(unknown callee) 0xcf7a4` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xca9c8 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xca9f8 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `bls` at 0xcaa14 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xcaa58 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xcaa7c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcab04 | `r0` | `?` |

## `JChip_get_field`

- `JChip_get_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcab20 to 0xcac40.
  Established. A stack word stores `[sp-32]` at `sp -0x2c` before the last call (`str` at 0xcac08).
  That call targets `0xce614` with candidates unknown, `[sp-28]`, `0x0`, unknown (`bl` at 0xcac1c).
  The earlier unsigned test takes its bypass for `[0x27eb98] <= 0x3` and falls through above that
  bound (`bls` at 0xcab98). The result slot `sp -0x28` can receive `0x0` (`str` at 0xcac24). The
  field’s device meaning and merged return are unknown (`ldmdb` at 0xcac2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcab30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcab34 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcab38 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcab3c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcab40 | load | 4 | `abs 0xcac30` | `-` |
| `ldr` at 0xcab44 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcab50 | load | 4 | `abs 0xcac34` | `-` |
| `ldr` at 0xcab54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcab58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcab5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcab64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcab70 | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xcab74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcab80 | load | 4 | `abs 0xcac38` | `-` |
| `str` at 0xcab84 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xcab8c | load | 4 | `abs 0xcac30` | `-` |
| `ldr` at 0xcab90 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcab9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaba0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaba8 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcabb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcabb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcabb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcabc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcabc8 | load | 4 | `abs 0xcac3c` | `-` |
| `ldr` at 0xcabcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcabd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcabd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcabe0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcabe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcabec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcabf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcabf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcac00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcac04 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcac08 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xcac0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcac10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcac18 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcac24 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcac28 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcac2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcac2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcac2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcab60 | `(unknown callee) 0x20f14` | `0x10acc0` | `r0` | `r1` | `r2` |
| `bl` at 0xcab68 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcabd0 | `(unknown callee) 0x20f14` | `0x10acf4` | `?` | `?` | `?` |
| `bl` at 0xcac1c | `(unknown callee) 0xce614` | `?` | `[sp-28]` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcab4c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcab7c | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `bls` at 0xcab98 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcac2c | `r0` | `?` |

## `JChip_get_field_by_name`

- `JChip_get_field_by_name`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcac40 to 0xcadb4.
  Established. The call to `0xce8a4` uses `[sp-44]`, `[sp-28]`, unknown, unknown (`bl` at 0xcacd4).
  The comparison takes its edge for `ret(0xce8a4) < [[sp-44]+2]:2` unsigned and falls through for
  greater-or-equal (`blo` at 0xcacf0). The fall-through path stores `0xfffff7c6` at `sp -0x30`
  (`str` at 0xcacf8). The final call targets `0xce614` with unknown, unknown, `0x0`, unknown (`bl`
  at 0xcad90). Name matching and the field’s protocol meaning are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcac50 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcac54 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcac58 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcac5c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcac60 | load | 4 | `abs 0xcada4` | `-` |
| `ldr` at 0xcac64 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcac70 | load | 4 | `abs 0xcada8` | `-` |
| `ldr` at 0xcac74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcac7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcac84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcac90 | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xcac94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaca0 | load | 4 | `abs 0xcadac` | `-` |
| `str` at 0xcaca4 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xcacac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcacb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcacb8 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcacc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcacc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcacc8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcaccc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcacd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcacdc | store | 4 | `sp -0x28` | `ret(0xce8a4)` |
| `ldr` at 0xcace0 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcace4 | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xcace8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcacf4 | load | 4 | `abs 0xcadac` | `-` |
| `str` at 0xcacf8 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xcad00 | load | 4 | `abs 0xcada4` | `-` |
| `ldr` at 0xcad04 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcad10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcad14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcad1c | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcad24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcad34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad3c | load | 4 | `abs 0xcadb0` | `-` |
| `ldr` at 0xcad40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcad4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcad54 | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcad5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcad64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcad6c | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcad74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcad7c | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0xcad80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcad84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcad8c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcad98 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcad9c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcada0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcada0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcada0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcac80 | `(unknown callee) 0x20f14` | `0x10ad14` | `r0` | `r1` | `r2` |
| `bl` at 0xcac88 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcacd4 | `(unknown callee) 0xce8a4` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xcad44 | `(unknown callee) 0x20f14` | `0x10acf4` | `?` | `?` | `?` |
| `bl` at 0xcad90 | `(unknown callee) 0xce614` | `?` | `?` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcac6c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcac9c | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `blo` at 0xcacf0 | `ret(0xce8a4) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xce8a4) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `bls` at 0xcad0c | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcada0 | `r0` | `?` |

## `JChip_get_field_str`

- `JChip_get_field_str`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcadb4 to 0xcaf74.
  Established. The call to `0x18920` has four unknown candidates (`bl` at 0xcaec8); its nonzero
  result takes the continuation and zero falls through to word `0xfffff7cc` at `sp -0x30` (`bne`
  at 0xcaedc, `str` at 0xcaee4). The later `0xcf198` call has candidates `[sp-40]`, `[sp-44]`,
  `[sp-32]`, unknown (`bl` at 0xcaf44). A following call targets `0x18be4` with `[sp-44]` and three
  unknown candidates (`bl` at 0xcaf4c). Output encoding and the joined return remain unknown
  (`ldmdb` at 0xcaf5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcadc4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcadc8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcadcc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcadd0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcadd4 | load | 4 | `abs 0xcaf60` | `-` |
| `ldr` at 0xcadd8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcade4 | load | 4 | `abs 0xcaf64` | `-` |
| `ldr` at 0xcade8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcadec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcadf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcadf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcae04 | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xcae08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcae14 | load | 4 | `abs 0xcaf68` | `-` |
| `str` at 0xcae18 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xcae20 | load | 4 | `abs 0xcaf60` | `-` |
| `ldr` at 0xcae24 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcae30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcae34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcae3c | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcae44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcae54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae5c | load | 4 | `abs 0xcaf6c` | `-` |
| `ldr` at 0xcae60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcae6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcae74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcae84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcae8c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcae94 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcae98 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcae9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcaeb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcaed0 | store | 4 | `sp -0x2c` | `ret(0x18920)` |
| `ldr` at 0xcaed4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcaee0 | load | 4 | `abs 0xcaf70` | `-` |
| `str` at 0xcaee4 | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xcaeec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaef8 | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcaf00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaf04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcaf08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaf10 | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcaf18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaf1c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcaf20 | store | 4 | `sp -0x34` | `ret(0x18920)` |
| `ldr` at 0xcaf24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaf28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaf30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaf38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcaf3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcaf40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcaf48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcaf54 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcaf58 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcaf5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcaf5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcaf5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcadf4 | `(unknown callee) 0x20f14` | `0x10ad50` | `r0` | `r1` | `r2` |
| `bl` at 0xcadfc | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcae64 | `(unknown callee) 0x20f14` | `0x10ad88` | `?` | `?` | `?` |
| `bl` at 0xcaec8 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xcaf34 | `(unknown callee) 0xce614` | `?` | `[sp-28]` | `0x0` | `?` |
| `bl` at 0xcaf44 | `(unknown callee) 0xcf198` | `[sp-40]` | `[sp-44]` | `[sp-32]` | `?` |
| `bl` at 0xcaf4c | `(unknown callee) 0x18be4` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcade0 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcae10 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `bls` at 0xcae2c | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xcaedc | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcaf5c | `r0` | `?` |

## `JChip_can_override_field`

- `JChip_can_override_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcaf74 to 0xcb020.
  Established. The unsigned test takes its bypass for `[0x27eb98] <= 0x2` and falls through above it
  (`bls` at 0xcaf9c). The call to `0xca498` has four unknown candidates (`bl` at 0xcafb8). A nonzero
  result takes the longer path, while zero falls through to word `0x0` at `sp -0x24` (`bne` at
  0xcafcc, `str` at 0xcafd4). The longer path stores an unknown word in the same stack slot (`str`
  at 0xcb00c). The permission-like interpretation suggested by the symbol and the return value are
  unknown (`ldmdb` at 0xcb014).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcaf84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcaf88 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcaf8c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcaf90 | load | 4 | `abs 0xcb018` | `-` |
| `ldr` at 0xcaf94 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcafa0 | load | 4 | `abs 0xcb01c` | `-` |
| `ldr` at 0xcafa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcafa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcafac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcafb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcafc0 | store | 4 | `sp -0x20` | `ret(0xca498)` |
| `ldr` at 0xcafc4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcafd4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcafdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcafe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcafe8 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcaff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaff4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaff8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcb000 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcb00c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcb010 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xcb014 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb014 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcafb0 | `(unknown callee) 0x20f14` | `0x10adac` | `r0` | `r1` | `r2` |
| `bl` at 0xcafb8 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcaf9c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcafcc | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb014 | `r0` | `?` |

## `JChip_override_field`

- `JChip_override_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb020 to 0xcb16c.
  Established. A word store saves `[r3]` at `sp -0x2c` before the first call (`str` at 0xcb058). The
  later call to `0xcf7a4` carries unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at 0xcb148). The test
  prior that region takes its edge for `? == 0x0` and falls through for inequality (`beq` at
  0xcb0c8). The inequality path stores `0xfffff7c6` at `sp -0x28` (`str` at 0xcb0d0). Override
  behavior and the merged return remain unproven (`ldmdb` at 0xcb158).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb030 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb034 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb038 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcb03c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcb040 | load | 4 | `abs 0xcb15c` | `-` |
| `ldr` at 0xcb044 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb050 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb054 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xcb058 | store | 4 | `sp -0x2c` | `[r3]` |
| `ldr` at 0xcb05c | load | 4 | `abs 0xcb160` | `-` |
| `ldr` at 0xcb060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb07c | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xcb080 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb08c | load | 4 | `abs 0xcb164` | `-` |
| `str` at 0xcb090 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xcb098 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb09c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb0a4 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcb0ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb0b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb0b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcb0bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcb0cc | load | 4 | `abs 0xcb164` | `-` |
| `str` at 0xcb0d0 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xcb0d8 | load | 4 | `abs 0xcb15c` | `-` |
| `ldr` at 0xcb0dc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb0e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb0ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb0f4 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcb0fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb100 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb10c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb114 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb118 | load | 4 | `abs 0xcb168` | `-` |
| `ldr` at 0xcb11c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb120 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcb128 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb12c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb134 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb13c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb140 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb144 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcb150 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcb154 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcb158 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb158 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb158 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb06c | `(unknown callee) 0x20f14` | `0x10ade8` | `r0` | `r1` | `r2` |
| `bl` at 0xcb074 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb124 | `(unknown callee) 0x20f14` | `0x10ae30` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xcb148 | `(unknown callee) 0xcf7a4` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcb04c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcb088 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `beq` at 0xcb0c8 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xcb0e4 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb158 | `r0` | `?` |

## `JChip_override_field_by_name`

- `JChip_override_field_by_name`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb16c to 0xcb30c.
  Established. The call to `0xce8a4` has candidates `[sp-44]`, `[sp-28]`, unknown, unknown (`bl` at
  0xcb20c); its unsigned bound test takes the continuation for `ret(0xce8a4) < [[sp-44]+2]:2` and
  falls through otherwise (`blo` at 0xcb228). The final call to `0xcf7a4` carries unknown, unknown,
  `[sp-32]`, unknown (`bl` at 0xcb2e8). The later result-slot store writes `0x0` at `sp -0x30`
  (`str` at 0xcb2f0). The selected field’s physical effect and joined return are unknown (`ldmdb` at
  0xcb2f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb17c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb180 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb184 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcb188 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcb18c | load | 4 | `abs 0xcb2fc` | `-` |
| `ldr` at 0xcb190 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb19c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb1a0 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xcb1a4 | store | 4 | `sp -0x34` | `[r3]` |
| `ldr` at 0xcb1a8 | load | 4 | `abs 0xcb300` | `-` |
| `ldr` at 0xcb1ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb1b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb1b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb1bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb1c8 | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xcb1cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb1d8 | load | 4 | `abs 0xcb304` | `-` |
| `str` at 0xcb1dc | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xcb1e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb1e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb1f0 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xcb1f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb1fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb200 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcb204 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcb208 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb214 | store | 4 | `sp -0x28` | `ret(0xce8a4)` |
| `ldr` at 0xcb218 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcb21c | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xcb220 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb22c | load | 4 | `abs 0xcb304` | `-` |
| `str` at 0xcb230 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xcb238 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb23c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb244 | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcb24c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb250 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb254 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb25c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcb26c | load | 4 | `abs 0xcb304` | `-` |
| `str` at 0xcb270 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xcb278 | load | 4 | `abs 0xcb2fc` | `-` |
| `ldr` at 0xcb27c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb288 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb28c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb294 | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcb29c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb2a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb2a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb2ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb2b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb2b8 | load | 4 | `abs 0xcb308` | `-` |
| `ldr` at 0xcb2bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb2c0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcb2c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb2cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb2d4 | load | 4 | `[sp-36] +0x1c` | `-` |
| `ldr` at 0xcb2dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb2e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb2e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcb2f0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcb2f4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcb2f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb2f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb2f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb1b8 | `(unknown callee) 0x20f14` | `0x10ae64` | `r0` | `r1` | `r2` |
| `bl` at 0xcb1c0 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb20c | `(unknown callee) 0xce8a4` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xcb2c4 | `(unknown callee) 0x20f14` | `0x10ae30` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xcb2e8 | `(unknown callee) 0xcf7a4` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcb198 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcb1d4 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `blo` at 0xcb228 | `ret(0xce8a4) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xce8a4) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xcb268 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xcb284 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb2f8 | `r0` | `?` |

## `jchip_get_target`

- `jchip_get_target`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca63c to 0xca7e0.
  Established. The entry r0 and r1 bytes are saved at `sp -0x11` and `sp -0x12` as `(r0 & 0xff)` and
  `(r1 & 0xff)` (`strb` at 0xca654, `strb` at 0xca65c). A later branch takes its edge for `? != (r1
  & 0xff)` and falls through on equality (`bne` at 0xca6bc). The only call targets `0x1eed0` with
  `0x9`, `0x10a974`, `0x2ad`, `0x0` (`bl` at 0xca780). The result slot `sp -0x28` receives unknown
  values on its separate paths, and the return reports `[sp-40]` (`str` at 0xca7a0, `str` at
  0xca7bc, `ldmdb` at 0xca7c4). The target encoding’s device meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xca654 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xca65c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0xca664 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xca668 | load | 4 | `abs 0xca7c8` | `-` |
| `ldr` at 0xca66c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca670 | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xca680 | load | 4 | `abs 0xca7cc` | `-` |
| `ldr` at 0xca684 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca68c | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xca694 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xca698 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xca69c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xca6a0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xca6ac | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xca6b0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xca6b4 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xca6c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca6c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca6c8 | load | 4 | `abs 0xca7d0` | `-` |
| `ldrb` at 0xca6cc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xca6ec | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xca6f4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xca6fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xca700 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca70c | load | 4 | `abs 0xca7d4` | `-` |
| `str` at 0xca710 | store | 4 | `sp -0x3c` | `0x10abd4` |
| `ldr` at 0xca714 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca718 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca71c | load | 4 | `(unknown)` | `-` |
| `str` at 0xca720 | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0xca724 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xca728 | store | 4 | `sp -0x34` | `(r0 & 0xff)` |
| `ldrb` at 0xca72c | load | 1 | `sp -0x12` | `-` |
| `str` at 0xca730 | store | 4 | `sp -0x30` | `(r1 & 0xff)` |
| `ldr` at 0xca734 | load | 4 | `abs 0xca7d0` | `-` |
| `ldrb` at 0xca738 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xca758 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xca760 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xca768 | load | 1 | `(unknown)` | `-` |
| `str` at 0xca76c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xca774 | load | 4 | `abs 0xca7d8` | `-` |
| `ldr` at 0xca778 | load | 4 | `abs 0xca7dc` | `-` |
| `ldr` at 0xca784 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca788 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca78c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca794 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xca798 | load | 1 | `(unknown)` | `-` |
| `str` at 0xca7a0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xca7a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca7b0 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xca7bc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xca7c0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xca7c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca7c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca7c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca780 | `(unknown callee) 0x1eed0` | `0x9` | `0x10a974` | `0x2ad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xca678 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xca6a8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xca6bc | `? != (r1 & 0xff)` | `? == (r1 & 0xff)` |
| `beq` at 0xca708 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca7c4 | `r0` | `[sp-40]` |

## `PBayGetState`

- `PBayGetState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f9b8 to 0x8f9d8.
  Established. A word load reads absolute `0x11b318` (`ldr` at 0x8f9c8). That value moves to r0 and
  reaches the return as `[0x11b318]` (`mov` at 0x8f9cc, `ldmdb` at 0x8f9d0). No calls, explicit
  stores or conditional branches are reported. The word’s state encoding is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8f9c4 | load | 4 | `abs 0x8f9d4` | `-` |
| `ldr` at 0x8f9c8 | load | 4 | `abs 0x11b318` | `-` |
| `ldmdb` at 0x8f9d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f9d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f9d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f9d0 | `r0` | `[0x11b318]` |

## `PBaySetState`

- `PBaySetState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f8dc to 0x8f9b8.
  Established. An absolute word store writes entry r0 at absolute `0x11b318` (`str` at
  0x8f8f8). The computed transfer takes an unresolved edge for `[0x11b318] <= 0x5` unsigned and
  falls through above that bound (`ldrls` at 0x8f908). The known fall-through path stores word
  `0xfbd88` at absolute `0x11564c` (`str` at 0x8f990); its return retains entry r0 (`ldmdb` at
  0x8f994). The excluded destinations’ stores and state meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f8ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8f8f0 | load | 4 | `abs 0x8f998` | `-` |
| `ldr` at 0x8f8f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8f8f8 | store | 4 | `abs 0x11b318` | `r0` |
| `ldr` at 0x8f8fc | load | 4 | `abs 0x8f998` | `-` |
| `ldr` at 0x8f900 | load | 4 | `abs 0x11b318` | `-` |
| `ldrls` at 0x8f908 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f988 | load | 4 | `abs 0x8f99c` | `-` |
| `ldr` at 0x8f98c | load | 4 | `abs 0x8f9b4` | `-` |
| `str` at 0x8f990 | store | 4 | `abs 0x11564c` | `0xfbd88` |
| `ldmdb` at 0x8f994 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f994 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x8f908 | `[0x11b318] <= 0x5 (unsigned, 32-bit)` | `[0x11b318] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f994 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8f908).

## `PBayError`

- `PBayError`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8ff7c to 0x8ffb8.
  Established. Entry r0 and r1 are saved as words at `sp -0x14` and `sp -0x18` (`str` at 0x8ff8c,
  `str` at 0x8ff90). The only call targets `0x21010` with candidates `[0x116928]`, `0xfbe14`, entry
  r0 and entry r1 (`bl` at 0x8ffa8); its result reaches the return (`ldmdb` at 0x8ffac). No
  conditional branches are reported, and the callee’s output contract is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8ff8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8ff90 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8ff94 | load | 4 | `abs 0x8ffb0` | `-` |
| `ldr` at 0x8ff98 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x8ff9c | load | 4 | `abs 0x8ffb4` | `-` |
| `ldr` at 0x8ffa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ffa4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8ffac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8ffac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8ffac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8ffa8 | `(unknown callee) 0x21010` | `[0x116928]` | `0xfbe14` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8ffac | `r0` | `ret(0x21010)` |

## `PBayInit`

- `PBayInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f7c8 to 0x8f8dc.
  Established. The call to `0x18920` uses `0x100` followed by three unknown candidates, and its
  result is stored as a word at absolute `0x11b32c` (`bl` at 0x8f7fc, `str` at 0x8f804). A halfword
  store clears absolute `0x11b324` (`strh` at 0x8f834). A word store writes `0x11b31c` at
  `[0x11b32c] +0x8` (`str` at 0x8f844). The call to `0x1f498` carries `[0x11b32c]+4`, `0xfbd14`,
  `0x904e8`, `0x1` (`bl` at 0x8f890). The numeric candidates’ task and hardware meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8f7f4 | load | 4 | `abs 0x8f8cc` | `-` |
| `str` at 0x8f804 | store | 4 | `abs 0x11b32c` | `ret(0x18920)` |
| `ldr` at 0x8f808 | load | 4 | `abs 0x8f8cc` | `-` |
| `ldr` at 0x8f80c | load | 4 | `abs 0x11b32c` | `-` |
| `ldr` at 0x8f81c | load | 4 | `abs 0x8f8d0` | `-` |
| `ldr` at 0x8f82c | load | 4 | `abs 0x8f8d0` | `-` |
| `strh` at 0x8f834 | store | 2 | `abs 0x11b324` | `0x0` |
| `ldr` at 0x8f838 | load | 4 | `abs 0x8f8cc` | `-` |
| `ldr` at 0x8f83c | load | 4 | `abs 0x11b32c` | `-` |
| `ldr` at 0x8f840 | load | 4 | `abs 0x8f8d0` | `-` |
| `str` at 0x8f844 | store | 4 | `[0x11b32c] +0x8` | `0x11b31c` |
| `ldr` at 0x8f848 | load | 4 | `abs 0x8f8cc` | `-` |
| `ldr` at 0x8f84c | load | 4 | `abs 0x11b32c` | `-` |
| `ldr` at 0x8f854 | load | 4 | `abs 0x8f8cc` | `-` |
| `ldr` at 0x8f858 | load | 4 | `abs 0x11b32c` | `-` |
| `str` at 0x8f85c | store | 4 | `sp -0x2c` | `[0x11b32c]` |
| `str` at 0x8f864 | store | 4 | `sp -0x28` | `0xa00` |
| `str` at 0x8f86c | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0x8f874 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0x8f87c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f884 | load | 4 | `abs 0x8f8d4` | `-` |
| `ldr` at 0x8f888 | load | 4 | `abs 0x8f8d8` | `-` |
| `str` at 0x8f898 | store | 4 | `sp -0x18` | `ret(0x1f498)` |
| `ldr` at 0x8f89c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f8bc | load | 4 | `abs 0x8f8cc` | `-` |
| `ldr` at 0x8f8c0 | load | 4 | `abs 0x11b32c` | `-` |
| `ldmdb` at 0x8f8c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8f8c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f8c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f8c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8f7d8 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x8f7f0 | `(unknown callee) 0x8f8dc` | `0x1` | `?` | `?` | `ret(0x2b31c)` |
| `bl` at 0x8f7fc | `(unknown callee) 0x18920` | `0x100` | `?` | `?` | `?` |
| `bl` at 0x8f818 | `(unknown callee) 0x18c20` | `[0x11b32c]` | `0x0` | `0x100` | `0x11b32c` |
| `bl` at 0x8f828 | `(unknown callee) 0x18c20` | `0x11b31c` | `0x0` | `0x10` | `?` |
| `bl` at 0x8f890 | `(unknown callee) 0x1f498` | `[0x11b32c]+4` | `0xfbd14` | `0x904e8` | `0x1` |
| `bl` at 0x8f8b0 | `(unknown callee) 0x8ff7c` | `?` | `0x7` | `?` | `ret(0x1f498)` |
| `bl` at 0x8f8b8 | `(unknown callee) 0x8f8dc` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x8f8c4 | `(unknown callee) 0x18be4` | `[0x11b32c]` | `?` | `?` | `0x11b32c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8f7e4 | `ret(0x2b31c) != 0x32` | `ret(0x2b31c) == 0x32` |
| `beq` at 0x8f8a4 | `ret(0x1f498) == 0x0` | `ret(0x1f498) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f8c8 | `r0` | `?` |

## `PBayInitFunc`

- `PBayInitFunc`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x904e8 to 0x905a8.
  Established. The first call targets `0x1f9e8` with candidates entry `r1+160`, `0xfbe48`, `0x0`,
  unknown (`bl` at 0x90520); its zero result takes the continuation, while nonzero falls through
  to `0x8ff7c` and `0x8f8dc` (`beq` at 0x9052c, `bl` at 0x90538, `bl` at 0x90540). A halfword
  store at `[[sp-36]+8] +0x4` has unknown value (`strh` at 0x90588). The last call targets `0x8f8dc`
  with `0x5` followed by three unknown candidates (`bl` at 0x9059c); its result reaches the return,
  with initialization semantics unproven (`ldmdb` at 0x905a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x904f8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x904fc | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x90500 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x90504 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x90508 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x90514 | load | 4 | `abs 0x905a4` | `-` |
| `ldr` at 0x90550 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x90554 | load | 4 | `[sp-36] +0x8` | `-` |
| `strh` at 0x90588 | store | 2 | `[[sp-36]+8] +0x4` | `?` |
| `ldmdb` at 0x905a0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x905a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x905a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x905a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x905a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x90520 | `(unknown callee) 0x1f9e8` | `r1+160` | `0xfbe48` | `0x0` | `?` |
| `bl` at 0x90538 | `(unknown callee) 0x8ff7c` | `?` | `0x11` | `?` | `ret(0x1f9e8)` |
| `bl` at 0x90540 | `(unknown callee) 0x8f8dc` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x9054c | `(unknown callee) 0x8f8dc` | `0x1` | `?` | `?` | `ret(0x1f9e8)` |
| `bl` at 0x9055c | `(unknown callee) 0x1c79c` | `0x1` | `?` | `?` | `[sp-36]` |
| `bl` at 0x90570 | `(unknown callee) 0x1c79c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x90594 | `(unknown callee) 0x8ff7c` | `?` | `0x2` | `?` | `?` |
| `bl` at 0x9059c | `(unknown callee) 0x8f8dc` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9052c | `ret(0x1f9e8) == 0x0` | `ret(0x1f9e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x905a0 | `r0` | `ret(0x8f8dc)` |

## `PBayInitIcmb`

- `PBayInitIcmb`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x905a8 to 0x90884.
  Established. Halfword stores at `[[sp-24]+8] +0x6` include `0x0`, `(ret(0x94714) & 0xffff)`,
  `0x4400` on separate paths (`strh` at 0x905d0, `strh` at 0x90678, `strh` at 0x906e8). Two stack
  halfword stores at `sp -0x1b4` write `0x0` and `([[[sp-24]+8]+6]:2 & 0xffff)` (`strh` at 0x905e0,
  `strh` at 0x906a0). Calls to `0x90884` include `[[sp-24]+8]`, `sp-436`, `sp-436`, `[sp-24]` (`bl`
  at 0x9061c, `bl` at 0x90650, `bl` at 0x906bc, `bl` at 0x90728, `bl` at 0x907bc). The final call
  targets `0x8f8dc` with `0x2` and three unknown candidates (`bl` at 0x90878). The physical
  interconnect operation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x905b8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x905c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x905c8 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0x905d0 | store | 2 | `[[sp-24]+8] +0x6` | `0x0` |
| `ldr` at 0x905d4 | load | 4 | `abs 0x90880` | `-` |
| `strh` at 0x905e0 | store | 2 | `sp -0x1b4` | `0x0` |
| `str` at 0x905e8 | store | 4 | `sp -0x1b0` | `0x10` |
| `str` at 0x905f0 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0x905f8 | store | 4 | `sp -0x118` | `0xff` |
| `str` at 0x90600 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x90608 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9060c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90614 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x90624 | store | 4 | `sp -0x1b0` | `0x2` |
| `str` at 0x9062c | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x90634 | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0x9063c | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x90640 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90648 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x90658 | store | 4 | `sp -0x1c` | `ret(0x90884)` |
| `ldr` at 0x9065c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90668 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9066c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x90670 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0x90678 | store | 2 | `[[sp-24]+8] +0x6` | `(ret(0x94714) & 0xffff)` |
| `str` at 0x90680 | store | 4 | `sp -0x1b0` | `0x12` |
| `str` at 0x90688 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x9068c | load | 4 | `abs 0x90880` | `-` |
| `ldr` at 0x90690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90694 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0x90698 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `strh` at 0x906a0 | store | 2 | `sp -0x1b4` | `([[[sp-24]+8]+6]:2 & 0xffff)` |
| `str` at 0x906a8 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x906ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x906b4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x906c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x906c4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0x906c8 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `ldr` at 0x906dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x906e0 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0x906e8 | store | 2 | `[[sp-24]+8] +0x6` | `0x4400` |
| `ldr` at 0x906f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x906f4 | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0x906fc | store | 1 | `[[sp-24]+8] +0x0` | `0x0` |
| `str` at 0x90704 | store | 4 | `sp -0x1b0` | `0x13` |
| `str` at 0x9070c | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0x90714 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x90718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90720 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x90730 | store | 4 | `sp -0x1c` | `ret(0x90884)` |
| `ldr` at 0x90734 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9074c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90750 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x90754 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x90758 | load | 1 | `[sp-44] +0x0` | `-` |
| `strb` at 0x9075c | store | 1 | `[[sp-24]+8] +0x0` | `([[sp-44]]:1 & 0xff)` |
| `str` at 0x90764 | store | 4 | `sp -0x1b0` | `0x20` |
| `str` at 0x9076c | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0x90774 | store | 4 | `sp -0x118` | `0x0` |
| `ldr` at 0x9077c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90780 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x90784 | load | 1 | `[[sp-24]+8] +0x0` | `-` |
| `strb` at 0x9078c | store | 1 | `(unknown)` | `([[[sp-24]+8]]:1 & 0xff)` |
| `str` at 0x90794 | store | 4 | `sp -0x110` | `0x0` |
| `strb` at 0x907a0 | store | 1 | `sp -0x10c` | `0x0` |
| `str` at 0x907a8 | store | 4 | `sp -0xc8` | `0x0` |
| `ldr` at 0x907ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x907b4 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x907c4 | store | 4 | `sp -0x1c` | `ret(0x90884)` |
| `ldr` at 0x907c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x907d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x907dc | store | 4 | `sp -0x1b8` | `ret(0x94484)` |
| `ldr` at 0x907e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x907e4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x907e8 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x907ec | load | 1 | `ret(0x94484) +0x1` | `-` |
| `strb` at 0x907f0 | store | 1 | `[[sp-24]+8] +0x1` | `([ret(0x94484)+1]:1 & 0xff)` |
| `ldr` at 0x907f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x907f8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x907fc | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x90800 | load | 1 | `[sp-440] +0x2` | `-` |
| `strb` at 0x90804 | store | 1 | `[[sp-24]+8] +0x2` | `([[sp-440]+2]:1 & 0xff)` |
| `ldr` at 0x90808 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9080c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x90810 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x90814 | load | 1 | `[sp-440] +0x3` | `-` |
| `strb` at 0x90818 | store | 1 | `[[sp-24]+8] +0x3` | `([[sp-440]+3]:1 & 0xff)` |
| `ldr` at 0x9081c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90820 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x90824 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0x90830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90834 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x90838 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0x90844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90848 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x9084c | load | 1 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0x90858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9085c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x90864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90868 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0x9087c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x9087c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9087c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9087c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x905c0 | `(unknown callee) 0x8f8dc` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0x9061c | `(unknown callee) 0x90884` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x90650 | `(unknown callee) 0x90884` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x90674 | `(unknown callee) 0x94714` | `[sp-44]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x906bc | `(unknown callee) 0x90884` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x906d8 | `(unknown callee) 0x8f8dc` | `0x5` | `?` | `?` | `[[[sp-24]+8]+6]:2` |
| `bl` at 0x90728 | `(unknown callee) 0x90884` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x90744 | `(unknown callee) 0x8f8dc` | `0x5` | `?` | `?` | `ret(0x90884)` |
| `bl` at 0x907bc | `(unknown callee) 0x90884` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0x907d8 | `(unknown callee) 0x94484` | `[sp-44]` | `?` | `?` | `ret(0x90884)` |
| `bl` at 0x90840 | `(unknown callee) 0x93e60` | `[[[sp-24]+8]+1]:1` | `?` | `[[sp-24]+8]` | `[[[sp-24]+8]+1]:1` |
| `bl` at 0x90860 | `(unknown callee) 0x90d14` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x90870 | `(unknown callee) 0x92b44` | `[[sp-24]+8]` | `0x0` | `?` | `[sp-24]` |
| `bl` at 0x90878 | `(unknown callee) 0x8f8dc` | `0x2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x90664 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |
| `bne` at 0x906d0 | `[[[sp-24]+8]+6]:2 != 0x0` | `[[[sp-24]+8]+6]:2 == 0x0` |
| `bne` at 0x9073c | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |
| `beq` at 0x907d0 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |
| `beq` at 0x9082c | `[[[sp-24]+8]+1]:1 == 0x0` | `[[[sp-24]+8]+1]:1 != 0x0` |
| `beq` at 0x90854 | `[[[sp-24]+8]+2]:1 == 0x0` | `[[[sp-24]+8]+2]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9087c | `r0` | `ret(0x8f8dc)` |
