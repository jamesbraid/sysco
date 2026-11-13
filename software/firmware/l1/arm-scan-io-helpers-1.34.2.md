# ARM scan I/O helpers in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates.
`jmstr_move_to_idle` reports one unresolved computed transfer on known edges.
The other fourteen routines report none. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/scan-io-helpers-facts.txt) retain all tool sections.

## `JTAG_Init`

- `JTAG_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2a6c to 0xc2c3c.
  Established. The call to `0xbc2cc` has first candidates `0xc5e54` and `0xc57c8`, with third and
  fourth unknown (`bl` at 0xc2ac8). A word store puts `ret(0x1889c)` at absolute `0x284340`, and a
  later word store writes `[0x284340]+16` at absolute `0x2842a0` (`str` at 0xc2b3c, `str` at
  0xc2b90). Four halfword stores at `[0x2842a0] +0x0` write `0x800`, `0x1000`, `0x800`, `0x800` in
  that order (`strh` at 0xc2ba0, `strh` at 0xc2bb0, `strh` at 0xc2bc0, `strh` at 0xc2bd0). The last
  explicit word store sets absolute `0x27eb94` to `0x1` (`str` at 0xc2bf0). The halfwords’ physical
  signal meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc2a7c | load | 4 | `abs 0xc2bf8` | `-` |
| `ldr` at 0xc2a80 | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc2a8c | load | 4 | `abs 0xc2bfc` | `-` |
| `str` at 0xc2a90 | store | 4 | `sp -0x24` | `0x109dc0` |
| `ldr` at 0xc2a98 | load | 4 | `abs 0xc2c00` | `-` |
| `ldr` at 0xc2a9c | load | 4 | `abs 0xc2c04` | `-` |
| `ldr` at 0xc2aa8 | load | 4 | `abs 0xc2c08` | `-` |
| `ldr` at 0xc2aac | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc2ab8 | load | 4 | `abs 0xc2c0c` | `-` |
| `ldr` at 0xc2ac0 | load | 4 | `abs 0xc2c10` | `-` |
| `ldr` at 0xc2ac4 | load | 4 | `abs 0xc2c14` | `-` |
| `ldr` at 0xc2acc | load | 4 | `abs 0xc2c18` | `-` |
| `ldr` at 0xc2ad0 | load | 4 | `abs 0xc2c1c` | `-` |
| `str` at 0xc2adc | store | 4 | `sp -0x1c` | `ret(0x18b54)` |
| `ldr` at 0xc2ae0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2af8 | load | 4 | `abs 0xc2c20` | `-` |
| `ldr` at 0xc2afc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0xc2b00 | load | 4 | `abs 0xc2c24` | `-` |
| `ldr` at 0xc2b04 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xc2b14 | load | 4 | `abs 0xc2c28` | `-` |
| `ldr` at 0xc2b18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc2b1c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc2b24 | store | 4 | `sp -0x20` | `0x10000000` |
| `str` at 0xc2b3c | store | 4 | `abs 0x284340` | `ret(0x1889c)` |
| `ldr` at 0xc2b40 | load | 4 | `abs 0x284340` | `-` |
| `ldr` at 0xc2b58 | load | 4 | `abs 0xc2c20` | `-` |
| `ldr` at 0xc2b5c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0xc2b60 | load | 4 | `abs 0xc2c2c` | `-` |
| `ldr` at 0xc2b64 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xc2b80 | load | 4 | `abs 0xc2c30` | `-` |
| `ldr` at 0xc2b84 | load | 4 | `abs 0xc2c28` | `-` |
| `ldr` at 0xc2b88 | load | 4 | `abs 0x284340` | `-` |
| `str` at 0xc2b90 | store | 4 | `abs 0x2842a0` | `[0x284340]+16` |
| `ldr` at 0xc2b94 | load | 4 | `abs 0xc2c30` | `-` |
| `ldr` at 0xc2b98 | load | 4 | `abs 0x2842a0` | `-` |
| `strh` at 0xc2ba0 | store | 2 | `[0x2842a0] +0x0` | `0x800` |
| `ldr` at 0xc2ba4 | load | 4 | `abs 0xc2c30` | `-` |
| `ldr` at 0xc2ba8 | load | 4 | `abs 0x2842a0` | `-` |
| `strh` at 0xc2bb0 | store | 2 | `[0x2842a0] +0x0` | `0x1000` |
| `ldr` at 0xc2bb4 | load | 4 | `abs 0xc2c30` | `-` |
| `ldr` at 0xc2bb8 | load | 4 | `abs 0x2842a0` | `-` |
| `strh` at 0xc2bc0 | store | 2 | `[0x2842a0] +0x0` | `0x800` |
| `ldr` at 0xc2bc4 | load | 4 | `abs 0xc2c30` | `-` |
| `ldr` at 0xc2bc8 | load | 4 | `abs 0x2842a0` | `-` |
| `strh` at 0xc2bd0 | store | 2 | `[0x2842a0] +0x0` | `0x800` |
| `ldr` at 0xc2bd4 | load | 4 | `abs 0xc2c34` | `-` |
| `ldr` at 0xc2bd8 | load | 4 | `abs 0xc2c38` | `-` |
| `ldr` at 0xc2be8 | load | 4 | `abs 0xc2bf8` | `-` |
| `str` at 0xc2bf0 | store | 4 | `abs 0x27eb94` | `0x1` |
| `ldmdb` at 0xc2bf4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc2bf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2bf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2bf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc2aa4 | `(unknown callee) 0x1eed0` | `0x9` | `0x109dd8` | `0x102` | `0x0` |
| `bl` at 0xc2abc | `(unknown callee) 0x20f14` | `0x109df0` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xc2ac8 | `(unknown callee) 0xbc2cc` | `0xc5e54` | `0xc57c8` | `?` | `?` |
| `bl` at 0xc2ad4 | `(unknown callee) 0x18b54` | `0x109e24` | `0x1002` | `?` | `?` |
| `bl` at 0xc2aec | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0xc2b08 | `(unknown callee) 0x21010` | `[0x116928]` | `0x109e30` | `[ret(0x188b4)]` | `0x116928` |
| `bl` at 0xc2b10 | `(unknown callee) 0x18b84` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xc2b38 | `(unknown callee) 0x1889c` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xc2b4c | `(unknown callee) 0x188b4` | `ret(0x1889c)` | `?` | `?` | `[0x284340]` |
| `bl` at 0xc2b68 | `(unknown callee) 0x21010` | `[0x116928]` | `0x109e50` | `[ret(0x188b4)]` | `0x116928` |
| `bl` at 0xc2b70 | `(unknown callee) 0x18b84` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xc2b7c | `(unknown callee) 0xc2c3c` | `0x1` | `0x0` | `?` | `?` |
| `bl` at 0xc2be4 | `(unknown callee) 0x1f9e8` | `0x284380` | `0x109e7c` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc2a88 | `[0x27eb94] != 0x1` | `[0x27eb94] == 0x1` |
| `beq` at 0xc2ab4 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bge` at 0xc2ae8 | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `bne` at 0xc2b48 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2bf4 | `r0` | `ret(0x1f9e8)` |

## `JTAG_Lock`

- `JTAG_Lock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2c9c to 0xc2d08.
  Established. The opening call to `0x1fb00` has candidates `0x284380`, unknown, entry r2 and entry
  r3 (`bl` at 0xc2cb4). A zero result takes the edge past the two later calls, while nonzero falls
  through to `0x1e71c` and `0x1eed0` (`beq` at 0xc2cc8, `bl` at 0xc2cd0, `bl` at 0xc2cf4). A
  stack word stores `ret(0x1fb00)` at `sp -0x14` (`str` at 0xc2cbc). Mutual-exclusion and timeout
  semantics of the numeric callee are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc2cac | load | 4 | `abs 0xc2cfc` | `-` |
| `str` at 0xc2cbc | store | 4 | `sp -0x14` | `ret(0x1fb00)` |
| `ldr` at 0xc2cc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2cd8 | load | 4 | `abs 0xc2d00` | `-` |
| `str` at 0xc2cdc | store | 4 | `sp -0x1c` | `0x109e88` |
| `str` at 0xc2ce0 | store | 4 | `sp -0x18` | `ret(0x1e71c)` |
| `ldr` at 0xc2ce8 | load | 4 | `abs 0xc2d04` | `-` |
| `ldmdb` at 0xc2cf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2cf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2cf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc2cb4 | `(unknown callee) 0x1fb00` | `0x284380` | `?` | `r2` | `r3` |
| `bl` at 0xc2cd0 | `(unknown callee) 0x1e71c` | `ret(0x1fb00)` | `?` | `?` | `ret(0x1fb00)` |
| `bl` at 0xc2cf4 | `(unknown callee) 0x1eed0` | `0x9` | `0x109dd8` | `0x170` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc2cc8 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2cf8 | `r0` | `?` |

## `JTAG_Unlock`

- `JTAG_Unlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2d08 to 0xc2d70.
  Established. The call to `0x1fe64` has candidates `0x284380`, entry r1, entry r2 and entry r3
  (`bl` at 0xc2d1c); its zero result takes the edge to the return, while a nonzero result falls
  through into calls to `0x1e71c` and `0x1eed0` (`beq` at 0xc2d30, `bl` at 0xc2d38, `bl` at
  0xc2d5c). The return value is unknown after the join (`ldmdb` at 0xc2d60). The callee’s effect
  is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc2d18 | load | 4 | `abs 0xc2d64` | `-` |
| `str` at 0xc2d24 | store | 4 | `sp -0x14` | `ret(0x1fe64)` |
| `ldr` at 0xc2d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2d34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2d40 | load | 4 | `abs 0xc2d68` | `-` |
| `str` at 0xc2d44 | store | 4 | `sp -0x1c` | `0x109ea8` |
| `str` at 0xc2d48 | store | 4 | `sp -0x18` | `ret(0x1e71c)` |
| `ldr` at 0xc2d50 | load | 4 | `abs 0xc2d6c` | `-` |
| `ldmdb` at 0xc2d60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2d60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2d60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc2d1c | `(unknown callee) 0x1fe64` | `0x284380` | `r1` | `r2` | `r3` |
| `bl` at 0xc2d38 | `(unknown callee) 0x1e71c` | `ret(0x1fe64)` | `?` | `?` | `ret(0x1fe64)` |
| `bl` at 0xc2d5c | `(unknown callee) 0x1eed0` | `0x9` | `0x109dd8` | `0x180` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc2d30 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2d60 | `r0` | `?` |

## `jmstr_set_functions`

- `jmstr_set_functions`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc2cc to 0xbc308.
  Established. Two word stores write entry r0 at absolute `0x27eb84` and `[sp-24]` at absolute
  `0x27eb88` (`str` at 0xbc2ec, `str` at 0xbc2f8). Entry r1 was saved at `sp -0x18` (`str` at
  0xbc2e0). The return retains entry r0 (`ldmdb` at 0xbc2fc). No call is reported, and the stored
  values’ function-pointer interpretation is unknown within this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc2dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbc2e0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbc2e4 | load | 4 | `abs 0xbc300` | `-` |
| `ldr` at 0xbc2e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbc2ec | store | 4 | `abs 0x27eb84` | `r0` |
| `ldr` at 0xbc2f0 | load | 4 | `abs 0xbc304` | `-` |
| `ldr` at 0xbc2f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbc2f8 | store | 4 | `abs 0x27eb88` | `[sp-24]` |
| `ldmdb` at 0xbc2fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc2fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc2fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc2fc | `r0` | `r0` |

## `jmstr_run_tck`

- `jmstr_run_tck`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc76c to 0xbc810.
  Established. A zero entry r1 takes the bypass edge, while nonzero falls through to an
  explicit-link call (`beq` at 0xbc7b0, `mov` at 0xbc7d0). That call has unknown target and
  candidates unknown, entry r1, `0x0`, `0x0` (`mov` at 0xbc7d0). A nonzero result takes the edge
  to a stack word store of `0x1`, while zero falls through to `0x18c74` and a stack word
  store of `0x0` at `sp -0x24` (`bne` at 0xbc7dc, `str` at 0xbc7fc, `bl` at 0xbc7e8, `str` at
  0xbc7f0). The number or timing of clock transitions is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc77c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbc780 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbc784 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbc78c | store | 4 | `sp -0x20` | `0x10` |
| `ldr` at 0xbc790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc79c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbc7a4 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0xbc7a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc7b4 | load | 4 | `abs 0xbc808` | `-` |
| `ldr` at 0xbc7b8 | load | 4 | `abs 0x27eb88` | `-` |
| `ldr` at 0xbc7bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc7c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc7e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc7e4 | load | 4 | `abs 0xbc80c` | `-` |
| `str` at 0xbc7f0 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xbc7fc | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xbc800 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xbc804 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc804 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xbc7d0 | `(unknown callee) (unknown target)` | `?` | `r1` | `0x0` | `0x0` |
| `bl` at 0xbc7e8 | `(unknown callee) 0x18c74` | `[sp-28]` | `0x109b14` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbc798 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xbc7b0 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xbc7dc | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc804 | `r0` | `?` |

## `jmstr_custom_seq`

- `jmstr_custom_seq`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc810 to 0xbc8b4.
  Established. The explicit-link call has unknown target and candidates unknown, entry r1, entry r2
  and entry r3 (`mov` at 0xbc874). Before it, a word store puts `[sp]` at `sp -0x2c` (`str` at
  0xbc858). A nonzero result takes the edge to word `0x1` at `sp -0x28`, while zero falls
  through to a call to `0x18c74` and word `0x0` there (`bne` at 0xbc880, `str` at 0xbc8a0, `bl`
  at 0xbc88c, `str` at 0xbc894). The custom sequence’s encoding and physical actions remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc820 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbc824 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbc828 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbc82c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbc834 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xbc838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc844 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc84c | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xbc850 | load | 4 | `abs 0xbc8ac` | `-` |
| `ldr` at 0xbc854 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbc858 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbc85c | load | 4 | `abs 0x27eb84` | `-` |
| `ldr` at 0xbc860 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc868 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc86c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc884 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xbc888 | load | 4 | `abs 0xbc8b0` | `-` |
| `str` at 0xbc894 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xbc8a0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xbc8a4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xbc8a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc8a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc8a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xbc874 | `(unknown callee) (unknown target)` | `?` | `r1` | `r2` | `r3` |
| `bl` at 0xbc88c | `(unknown callee) 0x18c74` | `[sp+4]` | `0x109b14` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbc840 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xbc880 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc8a8 | `r0` | `?` |

## `jmstr_move_to_idle`

- `jmstr_move_to_idle`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc4e8 to 0xbc76c.
  Established. The conditional computed transfer retains a less-or-equal flags predicate with
  unknown operands for its unresolved taken edge (`ldrls` at 0xbc5cc). The known fall-through path
  stores words `0xff000000` and `0x5` at `sp -0x2c` and `sp -0x38` (`str` at 0xbc660, `str` at
  0xbc668). A later explicit-link call has unknown target and candidates unknown, `0x5`, `sp-60`,
  `sp-44` (`mov` at 0xbc6a4). Another reached explicit-link call has all candidates unknown (`mov`
  at 0xbc708). The excluded destinations’ operations and any idle-state transition are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc4f8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbc4fc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbc500 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xbc504 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xbc50c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xbc514 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xbc518 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc524 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc538 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbc550 | store | 4 | `sp -0x30` | `ret(0x18920)` |
| `ldr` at 0xbc554 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbc560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc564 | load | 4 | `abs 0xbc75c` | `-` |
| `str` at 0xbc570 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xbc578 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc58c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc598 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbc5a8 | store | 4 | `sp -0x28` | `0x10` |
| `ldr` at 0xbc5ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc5b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbc5c0 | store | 4 | `sp -0x28` | `0x18` |
| `ldr` at 0xbc5c4 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0xbc5cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc660 | store | 4 | `sp -0x2c` | `0xff000000` |
| `str` at 0xbc668 | store | 4 | `sp -0x38` | `0x5` |
| `ldr` at 0xbc66c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbc678 | load | 4 | `abs 0xbc760` | `-` |
| `str` at 0xbc688 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0xbc68c | load | 4 | `abs 0x27eb84` | `-` |
| `ldr` at 0xbc690 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbc694 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbc6b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc6b8 | load | 4 | `abs 0xbc764` | `-` |
| `ldr` at 0xbc6c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbc6cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbc6d8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xbc6e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc6ec | load | 4 | `abs 0xbc768` | `-` |
| `ldr` at 0xbc6f0 | load | 4 | `abs 0x27eb88` | `-` |
| `ldr` at 0xbc6f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbc6f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc6fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbc700 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbc718 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc71c | load | 4 | `abs 0xbc764` | `-` |
| `ldr` at 0xbc724 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbc730 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xbc738 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc744 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbc750 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xbc754 | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0xbc758 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbc758 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc758 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc758 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbc548 | `(unknown callee) 0x18920` | `?` | `r1` | `?` | `?` |
| `bl` at 0xbc568 | `(unknown callee) 0x18c74` | `[sp-36]` | `0x109b48` | `?` | `ret(0x18920)` |
| `bl` at 0xbc5a0 | `(unknown callee) 0x18c20` | `ret(0x18920)` | `0x0` | `?` | `?` |
| `mov` at 0xbc6a4 | `(unknown callee) (unknown target)` | `?` | `0x5` | `sp-60` | `sp-44` |
| `bl` at 0xbc6bc | `(unknown callee) 0x18c74` | `[sp-36]` | `0x109b14` | `?` | `ret(unknown)` |
| `bl` at 0xbc6d0 | `(unknown callee) 0x18be4` | `[sp-48]` | `?` | `?` | `[sp-48]` |
| `mov` at 0xbc708 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` |
| `bl` at 0xbc720 | `(unknown callee) 0x18c74` | `[sp-36]` | `0x109b14` | `?` | `ret(unknown)` |
| `bl` at 0xbc728 | `(unknown callee) 0x18be4` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xbc748 | `(unknown callee) 0x18be4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbc520 | `r2 == 0x0` | `r2 != 0x0` |
| `bne` at 0xbc55c | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `beq` at 0xbc5b4 | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xbc5cc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xbc674 | `0x5 == 0x0` | `0x5 != 0x0` |
| `bne` at 0xbc6b0 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xbc6c8 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xbc6e8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbc714 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xbc740 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc758 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xbc5cc).

## `sn1sc_set_tap_val`

- `sn1sc_set_tap_val`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc526c to 0xc5304.
  Established. The body saves entry r0 at `sp -0x14` (`str` at 0xc527c) and issues a halfword store
  at `[0x2842a0] +0x0` with unknown value (`strh` at 0xc52f8). Entry r0 remains the reported return
  value (`ldmdb` at 0xc52fc). No calls or conditional branches are reported. The input-to-signal bit
  mapping is unknown in these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc527c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5280 | load | 4 | `abs 0xc5300` | `-` |
| `ldr` at 0xc5284 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5288 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc5294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52e4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc52f8 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `ldmdb` at 0xc52fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc52fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc52fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc52fc | `r0` | `r0` |

## `sn1sc_get_tap_val`

- `sn1sc_get_tap_val`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4f18 to 0xc503c.
  Established. Six halfword loads address `[0x2842a0] +0x0` (`ldrh` at 0xc4f30, `ldrh` at 0xc4f54,
  `ldrh` at 0xc4f80, `ldrh` at 0xc4fac, `ldrh` at 0xc4fd8, `ldrh` at 0xc5004). Their subsequent
  calculation is stored as an unknown word at `sp -0x14` (`str` at 0xc5028). The return reports
  `[sp-20]` in r0 (`ldmdb` at 0xc5034). No call is reported, and no stable snapshot or signal naming
  is established by the separate reads.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc4f28 | load | 4 | `abs 0xc5038` | `-` |
| `ldr` at 0xc4f2c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4f30 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4f4c | load | 4 | `abs 0xc5038` | `-` |
| `ldr` at 0xc4f50 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4f54 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4f78 | load | 4 | `abs 0xc5038` | `-` |
| `ldr` at 0xc4f7c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4f80 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4fa4 | load | 4 | `abs 0xc5038` | `-` |
| `ldr` at 0xc4fa8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4fac | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4fd0 | load | 4 | `abs 0xc5038` | `-` |
| `ldr` at 0xc4fd4 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4fd8 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4ffc | load | 4 | `abs 0xc5038` | `-` |
| `ldr` at 0xc5000 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5004 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5028 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc502c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc5034 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5034 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5034 | `r0` | `[sp-20]` |

## `sn1sc_get_tap_text`

- `sn1sc_get_tap_text`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc503c to 0xc526c.
  Established. The halfword reads include `[0x2842a0] +0x0` (`ldrh` at 0xc5058, `ldrh` at 0xc507c,
  `ldrh` at 0xc50a8, `ldrh` at 0xc50d4, `ldrh` at 0xc5100, `ldrh` at 0xc512c). Separate paths store
  word `0x31` or `0x30` at `sp -0x1c` (`str` at 0xc5168, `str` at 0xc5174). Their deciding branch
  tests an unknown value (`beq` at 0xc5160). The call to `0x18c74` has candidates entry r0,
  `0x10a018`, `[sp-28]`, `[sp-32]` (`bl` at 0xc525c). The return reports `ret(0x18c74)` (`ldmdb` at
  0xc5260). The displayed field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc504c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5050 | load | 4 | `abs 0xc5264` | `-` |
| `ldr` at 0xc5054 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5058 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5074 | load | 4 | `abs 0xc5264` | `-` |
| `ldr` at 0xc5078 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc507c | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc50a0 | load | 4 | `abs 0xc5264` | `-` |
| `ldr` at 0xc50a4 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc50a8 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc50cc | load | 4 | `abs 0xc5264` | `-` |
| `ldr` at 0xc50d0 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc50d4 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc50f8 | load | 4 | `abs 0xc5264` | `-` |
| `ldr` at 0xc50fc | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5100 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5124 | load | 4 | `abs 0xc5264` | `-` |
| `ldr` at 0xc5128 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc512c | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5150 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc5154 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5168 | store | 4 | `sp -0x1c` | `0x31` |
| `str` at 0xc5174 | store | 4 | `sp -0x1c` | `0x30` |
| `ldr` at 0xc5178 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc518c | store | 4 | `sp -0x20` | `0x31` |
| `str` at 0xc5198 | store | 4 | `sp -0x20` | `0x30` |
| `ldr` at 0xc519c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc51b0 | store | 4 | `sp -0x24` | `0x31` |
| `str` at 0xc51bc | store | 4 | `sp -0x24` | `0x30` |
| `ldr` at 0xc51c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc51c4 | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0xc51c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc51dc | store | 4 | `sp -0x28` | `0x31` |
| `str` at 0xc51e8 | store | 4 | `sp -0x28` | `0x30` |
| `ldr` at 0xc51ec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc51f0 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0xc51f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5208 | store | 4 | `sp -0x2c` | `0x31` |
| `str` at 0xc5214 | store | 4 | `sp -0x2c` | `0x30` |
| `ldr` at 0xc5218 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc521c | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0xc5220 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5234 | store | 4 | `sp -0x30` | `0x31` |
| `str` at 0xc5240 | store | 4 | `sp -0x30` | `0x30` |
| `ldr` at 0xc5244 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc5248 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xc524c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5250 | load | 4 | `abs 0xc5268` | `-` |
| `ldr` at 0xc5254 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5258 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc5260 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5260 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5260 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc525c | `(unknown callee) 0x18c74` | `r0` | `0x10a018` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5160 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc5184 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc51a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc51d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc5200 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc522c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5260 | `r0` | `ret(0x18c74)` |

## `sn1sc_asm_drive_tck`

- `sn1sc_asm_drive_tck`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5698 to 0xc57c8.
  Established. The two halfword stores address `[0x2842a0] +0x0`, each with value `([sp-40]:2 &
  0xffff)` at its respective site (`strh` at 0xc5790, `strh` at 0xc57ac). The intervening word store
  at `sp -0x28` has unknown value (`str` at 0xc579c). The loop branch takes its edge for `C == 0`
  and falls through for `not (C == 0)` (`blo` at 0xc5774). The return retains entry r0 (`ldmdb` at
  0xc57c0). Clock period and output signal interpretation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc56a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc56ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc56b0 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xc56b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc56c0 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc56c4 | store | 4 | `sp -0x20` | `[r0]` |
| `str` at 0xc56cc | store | 4 | `sp -0x1c` | `r0+4` |
| `ldr` at 0xc56d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc56dc | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xc56e0 | store | 4 | `sp -0x24` | `[r0+4]` |
| `str` at 0xc56e8 | store | 4 | `sp -0x1c` | `r0+8` |
| `ldr` at 0xc56ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc56f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc570c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5720 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5734 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5748 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc575c | store | 4 | `sp -0x20` | `?` |
| `str` at 0xc5764 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc5768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc576c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc577c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5780 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0xc5784 | load | 4 | `abs 0xc57c4` | `-` |
| `ldr` at 0xc5788 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc578c | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xc5790 | store | 2 | `[0x2842a0] +0x0` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xc5794 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc579c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc57a0 | load | 4 | `abs 0xc57c4` | `-` |
| `ldr` at 0xc57a4 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc57a8 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xc57ac | store | 2 | `[0x2842a0] +0x0` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xc57b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc57b8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xc57c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc57c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc57c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc5774 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc57c0 | `r0` | `r0` |

## `sn1sc_asm_drive_tap_fixed_tms`

- `sn1sc_asm_drive_tap_fixed_tms`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5304 to 0xc5698.
  Established. The first three halfword stores address `[0x2842a0] +0x0`, each reporting `([sp-60]:2
  & 0xffff)` (`strh` at 0xc5494, `strh` at 0xc54d4, `strh` at 0xc5510). The later path’s
  stores use that same target and expression (`strh` at 0xc55e4, `strh` at 0xc5624, `strh` at
  0xc5660). A word store also writes an unknown value at `[sp-44] +0x0` (`str` at 0xc5684). The
  branch before the later path takes its bypass for `? == 0x0` and falls through for `? != 0x0`
  (`beq` at 0xc555c). No calls are reported, and the input/output bits’ scan roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5314 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5318 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc531c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xc5324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc532c | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc5330 | store | 4 | `sp -0x20` | `[r0]` |
| `str` at 0xc5338 | store | 4 | `sp -0x1c` | `r0+4` |
| `ldr` at 0xc5340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5348 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xc534c | store | 4 | `sp -0x24` | `[r0+4]` |
| `str` at 0xc5354 | store | 4 | `sp -0x1c` | `r0+8` |
| `ldr` at 0xc535c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5364 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xc5368 | store | 4 | `sp -0x28` | `[r0+8]` |
| `str` at 0xc5370 | store | 4 | `sp -0x1c` | `r0+12` |
| `ldr` at 0xc5378 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5380 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xc5384 | store | 4 | `sp -0x2c` | `[r0+12]` |
| `str` at 0xc538c | store | 4 | `sp -0x1c` | `r0+16` |
| `ldr` at 0xc5390 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc539c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc53b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc53c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc53d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc53ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5400 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xc5408 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc540c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5414 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5428 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xc5430 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5438 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc543c | store | 4 | `sp -0x34` | `?` |
| `str` at 0xc5444 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc544c | store | 4 | `sp -0x30` | `0x1f` |
| `ldr` at 0xc5450 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc5460 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc5464 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc5474 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5478 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc547c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5484 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5488 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc548c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5490 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc5494 | store | 2 | `[0x2842a0] +0x0` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc5498 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc549c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc54a0 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc54b8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc54bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc54c4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc54c8 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc54cc | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc54d0 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc54d4 | store | 2 | `[0x2842a0] +0x0` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc54d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc54dc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc54e4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc54e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc54ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc54f4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc54f8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc5500 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5504 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc5508 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc550c | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc5510 | store | 2 | `[0x2842a0] +0x0` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc5514 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc551c | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0xc5528 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5530 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc5534 | store | 4 | `(unknown)` | `?` |
| `str` at 0xc553c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc5540 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5548 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc5550 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5560 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc556c | store | 4 | `sp -0x44` | `?` |
| `str` at 0xc5574 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xc557c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5584 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xc5588 | store | 4 | `sp -0x34` | `[[sp-40]]` |
| `str` at 0xc5590 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xc5598 | store | 4 | `sp -0x30` | `0x1f` |
| `ldr` at 0xc559c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc55a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc55b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc55b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc55c4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc55c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc55cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc55d4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc55d8 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc55dc | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc55e0 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc55e4 | store | 2 | `[0x2842a0] +0x0` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc55e8 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc55ec | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc55f0 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5608 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc560c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc5614 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5618 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc561c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5620 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc5624 | store | 2 | `[0x2842a0] +0x0` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc5628 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc562c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc5634 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc5638 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc563c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc5644 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5648 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc5650 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5654 | load | 4 | `abs 0xc5694` | `-` |
| `ldr` at 0xc5658 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc565c | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc5660 | store | 2 | `[0x2842a0] +0x0` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc5664 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc566c | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0xc5678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5680 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc5684 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xc568c | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldmdb` at 0xc5690 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5690 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc541c | `C == 0` | `not (C == 0)` |
| `bge` at 0xc5458 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc555c | `? == 0x0` | `? != 0x0` |
| `bge` at 0xc55a8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5690 | `r0` | `?` |

## `sn1sc_drive_tap_fixed_tms`

- `sn1sc_drive_tap_fixed_tms`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc57c8 to 0xc5a2c.
  Established. A nonzero entry r1 takes the edge into the longer path, while zero falls through
  to a stack word store of `0x1` (`bne` at 0xc57f0, `str` at 0xc57f8). The longer path includes
  a halfword store with unknown value at `[0x2842a0] +0x0` (`strh` at 0xc59c0). A later flags-only
  branch selects a taken path to `0xc5698` for `Z == 1` and falls through to `0xc5304` otherwise
  (`beq` at 0xc59f8, `bl` at 0xc5a14, `bl` at 0xc5a04). Both calls use candidates `sp-52`, unknown,
  unknown, `sp-52` (`bl` at 0xc5a14, `bl` at 0xc5a04). The return reports `0x1` (`ldmdb` at
  0xc5a24). A fixed-TMS electrical behavior is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc57d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc57dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc57e0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc57e4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc57e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc57f8 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xc5800 | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc5804 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5808 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5824 | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc5828 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc582c | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5850 | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc5854 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5858 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc587c | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc5880 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5884 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc58a8 | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc58ac | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc58b0 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc58d4 | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc58d8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc58dc | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5900 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5904 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc590c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc591c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc592c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc5934 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc593c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc5944 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5948 | load | 4 | `abs 0xc5a28` | `-` |
| `ldr` at 0xc594c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5950 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xc595c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5970 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5984 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5998 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc59ac | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xc59c0 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `ldr` at 0xc59c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc59c8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc59cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc59d0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc59d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc59e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc59e4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc59e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc59ec | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc59f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5a1c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xc5a20 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xc5a24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5a24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5a24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5a04 | `(unknown callee) 0xc5304` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xc5a14 | `(unknown callee) 0xc5698` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc57f0 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0xc5918 | `? == ?` | `? != ?` |
| `beq` at 0xc5928 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc59dc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc59f8 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5a24 | `r0` | `0x1` |

## `sn1sc_asm_drive_tap`

- `sn1sc_asm_drive_tap`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5a2c to 0xc5e54.
  Established. The six halfword stores address `[0x2842a0] +0x0`, each reporting `([sp-68]:2 &
  0xffff)` at its respective site (`strh` at 0xc5c14, `strh` at 0xc5c54, `strh` at 0xc5c90, `strh`
  at 0xc5da0, `strh` at 0xc5de0, `strh` at 0xc5e1c). A later word store has unknown value at
  `[sp-44] +0x0` (`str` at 0xc5e40). The branch selecting the later path takes its bypass for `? ==
  0x0` and falls through for inequality (`beq` at 0xc5cdc). The loop comparisons retain flags-only
  predicates with unknown operands (`blo` at 0xc5b60, `bge` at 0xc5bb8, `bge` at 0xc5d44). The
  physical scan sequence and merged return value remain unknown (`ldmdb` at 0xc5e4c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5a3c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5a40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5a44 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xc5a4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a54 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc5a58 | store | 4 | `sp -0x20` | `[r0]` |
| `str` at 0xc5a60 | store | 4 | `sp -0x1c` | `r0+4` |
| `ldr` at 0xc5a68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a70 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xc5a74 | store | 4 | `sp -0x24` | `[r0+4]` |
| `str` at 0xc5a7c | store | 4 | `sp -0x1c` | `r0+8` |
| `ldr` at 0xc5a84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a8c | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xc5a90 | store | 4 | `sp -0x28` | `[r0+8]` |
| `str` at 0xc5a98 | store | 4 | `sp -0x1c` | `r0+12` |
| `ldr` at 0xc5aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5aa8 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xc5aac | store | 4 | `sp -0x2c` | `[r0+12]` |
| `str` at 0xc5ab4 | store | 4 | `sp -0x1c` | `r0+16` |
| `ldr` at 0xc5abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5ac4 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xc5ac8 | store | 4 | `sp -0x30` | `[r0+16]` |
| `str` at 0xc5ad0 | store | 4 | `sp -0x1c` | `r0+20` |
| `ldr` at 0xc5ad4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5ae0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5af4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5b08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5b1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5b30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5b44 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xc5b4c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc5b50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5b58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5b6c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xc5b74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5b7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc5b80 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xc5b88 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc5b90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc5b98 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc5b9c | store | 4 | `sp -0x40` | `?` |
| `str` at 0xc5ba4 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xc5bac | store | 4 | `sp -0x34` | `0x1f` |
| `ldr` at 0xc5bb0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc5bc0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5bc4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc5bd4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5bd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc5bdc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc5bec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc5bf4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5bf8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc5bfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5c04 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5c08 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5c0c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5c10 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0xc5c14 | store | 2 | `[0x2842a0] +0x0` | `([sp-68]:2 & 0xffff)` |
| `ldr` at 0xc5c18 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5c1c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5c20 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5c38 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xc5c3c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc5c44 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5c48 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5c4c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5c50 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0xc5c54 | store | 2 | `[0x2842a0] +0x0` | `([sp-68]:2 & 0xffff)` |
| `ldr` at 0xc5c58 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc5c5c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc5c64 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xc5c68 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc5c6c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc5c74 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5c78 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc5c80 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5c84 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5c88 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5c8c | load | 2 | `sp -0x44` | `-` |
| `strh` at 0xc5c90 | store | 2 | `[0x2842a0] +0x0` | `([sp-68]:2 & 0xffff)` |
| `ldr` at 0xc5c94 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc5c9c | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0xc5ca8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5cb0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc5cb4 | store | 4 | `(unknown)` | `?` |
| `str` at 0xc5cbc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc5cc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5cc8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc5cd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5ce0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc5cec | store | 4 | `sp -0x4c` | `?` |
| `str` at 0xc5cf4 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xc5cfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5d04 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xc5d08 | store | 4 | `sp -0x38` | `[[sp-40]]` |
| `str` at 0xc5d10 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc5d18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc5d20 | load | 4 | `[sp-48] +0x0` | `-` |
| `str` at 0xc5d24 | store | 4 | `sp -0x40` | `[[sp-48]]` |
| `str` at 0xc5d2c | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `str` at 0xc5d34 | store | 4 | `sp -0x34` | `0x1f` |
| `ldr` at 0xc5d38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc5d3c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc5d4c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5d50 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc5d60 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5d64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc5d68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc5d78 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc5d80 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5d84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc5d88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5d90 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5d94 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5d98 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5d9c | load | 2 | `sp -0x44` | `-` |
| `strh` at 0xc5da0 | store | 2 | `[0x2842a0] +0x0` | `([sp-68]:2 & 0xffff)` |
| `ldr` at 0xc5da4 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5da8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5dac | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5dc4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xc5dc8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc5dd0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5dd4 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5dd8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5ddc | load | 2 | `sp -0x44` | `-` |
| `strh` at 0xc5de0 | store | 2 | `[0x2842a0] +0x0` | `([sp-68]:2 & 0xffff)` |
| `ldr` at 0xc5de4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc5de8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc5df0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xc5df4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc5df8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc5e00 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc5e04 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc5e0c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc5e10 | load | 4 | `abs 0xc5e50` | `-` |
| `ldr` at 0xc5e14 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5e18 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0xc5e1c | store | 2 | `[0x2842a0] +0x0` | `([sp-68]:2 & 0xffff)` |
| `ldr` at 0xc5e20 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc5e28 | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0xc5e34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5e3c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc5e40 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xc5e48 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldmdb` at 0xc5e4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5e4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5e4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc5b60 | `C == 0` | `not (C == 0)` |
| `bge` at 0xc5bb8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc5cdc | `? == 0x0` | `? != 0x0` |
| `bge` at 0xc5d44 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5e4c | `r0` | `?` |

## `sn1sc_drive_tap`

- `sn1sc_drive_tap`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5e54 to 0xc607c.
  Established. A halfword store writes unknown value at `[0x2842a0] +0x0` on a path selected by
  comparisons with unknown values (`strh` at 0xc6034, `beq` at 0xc5f8c, `beq` at 0xc5f9c). A stack
  word receives `[sp]` at `sp -0x28` (`str` at 0xc6054). The only call targets `0xc5a2c` with
  candidates `sp-52`, unknown, unknown, `sp-52` (`bl` at 0xc6068). The routine sets r0 to `0x1` and
  returns that value (`mov` at 0xc6070, `ldmdb` at 0xc6074). The scan-line mapping and callee’s
  physical action are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5e64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5e68 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc5e6c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc5e70 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc5e74 | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5e78 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5e7c | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5e98 | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5e9c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5ea0 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5ec4 | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5ec8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5ecc | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5ef0 | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5ef4 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5ef8 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5f1c | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5f20 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5f24 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc5f48 | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5f4c | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5f50 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc5f74 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5f78 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5f80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5f90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5fa0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc5fa8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5fb0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc5fb8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5fbc | load | 4 | `abs 0xc6078` | `-` |
| `ldr` at 0xc5fc0 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc5fc4 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xc5fd0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5fe4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc5ff8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc600c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc6020 | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xc6034 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `ldr` at 0xc6038 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc603c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc6040 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6044 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc6048 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc604c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6050 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc6054 | store | 4 | `sp -0x28` | `[sp]` |
| `ldr` at 0xc6058 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc605c | store | 4 | `sp -0x24` | `?` |
| `ldmdb` at 0xc6074 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6074 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6074 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc6068 | `(unknown callee) 0xc5a2c` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5f8c | `? == ?` | `? != ?` |
| `beq` at 0xc5f9c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6074 | `r0` | `0x1` |
