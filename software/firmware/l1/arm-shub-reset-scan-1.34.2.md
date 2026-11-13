# ARM SHUB reset and scan paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates. No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/shub-reset-scan-facts.txt) retain all tool sections.

## `SHUB_Reset_MCI`

- `SHUB_Reset_MCI`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24ae0 to 0x24b00.
  Established. A word store writes `0x0` at absolute `0x119a44` (`str` at 0x24af4). The return
  retains entry r0 (`ldmdb` at 0x24af8). No call or conditional branch is reported. The cleared
  word’s MCI-reset meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24aec | load | 4 | `abs 0x24afc` | `-` |
| `str` at 0x24af4 | store | 4 | `abs 0x119a44` | `0x0` |
| `ldmdb` at 0x24af8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24af8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24af8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24af8 | `r0` | `r0` |

## `SHUB_Reset_FSBCfg`

- `SHUB_Reset_FSBCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24f50 to 0x24f70.
  Established. The explicit store clears the word at absolute `0x119a4c` (`str` at 0x24f64), after
  which entry r0 remains the reported return value (`ldmdb` at 0x24f68). This span has no call or
  conditional branch. The absolute word’s configuration role is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24f5c | load | 4 | `abs 0x24f6c` | `-` |
| `str` at 0x24f64 | store | 4 | `abs 0x119a4c` | `0x0` |
| `ldmdb` at 0x24f68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24f68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24f68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24f68 | `r0` | `r0` |

## `SHUB_Reset_WRT90`

- `SHUB_Reset_WRT90`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24be8 to 0x24c08.
  Established. Absolute address `0x119a48` receives word `0x0` (`str` at 0x24bfc). Entry r0 reaches
  the return unchanged (`ldmdb` at 0x24c00). No calls or conditional branches appear in the reported
  facts. Whether the cleared word controls reset timing is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24bf4 | load | 4 | `abs 0x24c04` | `-` |
| `str` at 0x24bfc | store | 4 | `abs 0x119a48` | `0x0` |
| `ldmdb` at 0x24c00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24c00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24c00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24c00 | `r0` | `r0` |

## `SHUB_Reset_Hold`

- `SHUB_Reset_Hold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29ed4 to 0x29ef8.
  Established. The routine saves `(r0 & 0xff)` as a byte at `sp -0x11` (`strb` at 0x29ee8). It then
  sets r0 to `0x0` and returns that value (`mov` at 0x29ef0, `ldmdb` at 0x29ef4). No call or
  conditional branch is reported, and no electrical hold action is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x29ee8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x29ef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29ef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29ef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29ef4 | `r0` | `0x0` |

## `SHUB_Allow_Resets`

- `SHUB_Allow_Resets`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24f70 to 0x251e8.
  Established. Two calls to `0x24790` have second candidates `0x50000010` and `0x50002010`, each
  with third `0x0` and fourth `[sp-40]` (`bl` at 0x24fdc, `bl` at 0x2500c). Their first candidates
  are unknown and `[sp-17]:1`, respectively (`bl` at 0x24fdc, `bl` at 0x2500c). Two calls to
  `0x247f4` use `[sp-17]:1`, those same respective second candidates, `sp-44`, `sp-48` (`bl` at
  0x2504c, `bl` at 0x25090). A value unequal to `0x1` at `[sp-48]` takes each continuation branch,
  while equality falls through to another iteration (`bne` at 0x25070, `bne` at 0x250b4). The
  reset-enable semantics and exact iteration bound remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24f84 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x24f88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x24f90 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x24f98 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x24f9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x24fac | store | 4 | `sp -0x28` | `?` |
| `str` at 0x24fb4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x24fb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24fc8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x24fd0 | load | 4 | `abs 0x251c4` | `-` |
| `ldr` at 0x24fd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24fec | load | 4 | `abs 0x251c8` | `-` |
| `str` at 0x24ff0 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldrb` at 0x24ff8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x25000 | load | 4 | `abs 0x251cc` | `-` |
| `ldr` at 0x25008 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2501c | load | 4 | `abs 0x251c8` | `-` |
| `str` at 0x25020 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x25028 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x25034 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x25044 | load | 4 | `abs 0x251c4` | `-` |
| `ldr` at 0x2505c | load | 4 | `abs 0x251c8` | `-` |
| `str` at 0x25060 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x25068 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x25078 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x25088 | load | 4 | `abs 0x251cc` | `-` |
| `ldr` at 0x250a0 | load | 4 | `abs 0x251c8` | `-` |
| `str` at 0x250a4 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x250ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x250c0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x250c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x250d0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x250d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x250e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x250f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x250fc | load | 4 | `abs 0x251d0` | `-` |
| `str` at 0x25100 | store | 4 | `sp -0x38` | `0xda608` |
| `ldr` at 0x25108 | load | 4 | `abs 0x251d4` | `-` |
| `str` at 0x2510c | store | 4 | `sp -0x38` | `0xda614` |
| `ldr` at 0x25110 | load | 4 | `abs 0x251d8` | `-` |
| `ldr` at 0x25114 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x25118 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25120 | load | 4 | `abs 0x251dc` | `-` |
| `ldr` at 0x25124 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x25130 | load | 4 | `abs 0x251d8` | `-` |
| `str` at 0x25134 | store | 4 | `sp -0x4c` | `0xda61c` |
| `ldr` at 0x25138 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x25144 | load | 4 | `abs 0x251d0` | `-` |
| `str` at 0x25148 | store | 4 | `sp -0x3c` | `0xda608` |
| `ldr` at 0x25150 | load | 4 | `abs 0x251d4` | `-` |
| `str` at 0x25154 | store | 4 | `sp -0x3c` | `0xda614` |
| `ldr` at 0x25158 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x2515c | store | 4 | `sp -0x48` | `[sp-60]` |
| `ldr` at 0x25160 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25164 | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0x2516c | load | 4 | `abs 0x251e0` | `-` |
| `ldr` at 0x25170 | load | 4 | `abs 0x251e4` | `-` |
| `ldr` at 0x2517c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2518c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x25194 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x251a0 | load | 4 | `abs 0x251c8` | `-` |
| `str` at 0x251a4 | store | 4 | `sp -0x40` | `0xfffff893` |
| `str` at 0x251b0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x251b4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x251b8 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x251bc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x251c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x251c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x251c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24fdc | `(unknown callee) 0x24790` | `?` | `0x50000010` | `0x0` | `[sp-40]` |
| `bl` at 0x2500c | `(unknown callee) 0x24790` | `[sp-17]:1` | `0x50002010` | `0x0` | `[sp-40]` |
| `bl` at 0x2504c | `(unknown callee) 0x247f4` | `[sp-17]:1` | `0x50000010` | `sp-44` | `sp-48` |
| `bl` at 0x25090 | `(unknown callee) 0x247f4` | `[sp-17]:1` | `0x50002010` | `sp-44` | `sp-48` |
| `bl` at 0x2511c | `(unknown callee) 0x1dba4` | `0xda61c` | `[sp-56]` | `?` | `?` |
| `bl` at 0x25178 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x1a2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x24fc0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x24fe8 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `bne` at 0x25018 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `bne` at 0x25030 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x25058 | `ret(0x247f4) != 0x0` | `ret(0x247f4) == 0x0` |
| `bne` at 0x25070 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `bne` at 0x2509c | `ret(0x247f4) != 0x0` | `ret(0x247f4) == 0x0` |
| `bne` at 0x250b4 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `ble` at 0x250e0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x250ec | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x250f8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x2512c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x25140 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x25184 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x2519c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x251c0 | `r0` | `?` |

## `SHUB_Reset_PreRelease`

- `SHUB_Reset_PreRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29ef8 to 0x2a428.
  Established. The opening calls to `0x2896c` and `0x28d68` use first candidates `(r0 & 0xff)` and
  `[sp-17]:1`, respectively (`bl` at 0x29f2c, `bl` at 0x29f58). A zero result of each takes the edge
  to further processing, while nonzero falls through to saving that result at `sp -0x74`
  (`beq` at 0x29f40, `str` at 0x29f48, `beq` at 0x29f6c, `str` at 0x29f74). Later calls to `0xc9f88`
  include candidates `[sp-112]`, `0x60`, `sp-28`, `0x0` and `[sp-112]`, `0x62`, `sp-28`, `0x0` (`bl`
  at 0x2a120, `bl` at 0x2a180). Another call uses `[sp-112]`, `0x6`, `[0x119a40]`, `0x0` (`bl` at
  0x2a210). Their scan and reset-release meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x29f0c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x29f14 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x29f18 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29f20 | store | 4 | `sp -0x70` | `?` |
| `ldrb` at 0x29f24 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29f34 | store | 4 | `sp -0x18` | `ret(0x2896c)` |
| `ldr` at 0x29f38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29f44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29f48 | store | 4 | `sp -0x74` | `ret(0x2896c)` |
| `ldrb` at 0x29f50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29f60 | store | 4 | `sp -0x18` | `ret(0x28d68)` |
| `ldr` at 0x29f64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29f70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29f74 | store | 4 | `sp -0x74` | `ret(0x28d68)` |
| `ldr` at 0x29f9c | load | 4 | `abs 0x2a3bc` | `-` |
| `str` at 0x29fa0 | store | 4 | `sp -0x74` | `0xfffff883` |
| `ldrb` at 0x29fa8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29fbc | store | 4 | `sp -0x18` | `ret(0x27f00)` |
| `ldr` at 0x29fc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29fcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29fd0 | store | 4 | `sp -0x74` | `ret(0x27f00)` |
| `ldrb` at 0x29fd8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29fe8 | store | 4 | `sp -0x18` | `ret(0x25f58)` |
| `ldr` at 0x29fec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29ff8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29ffc | store | 4 | `sp -0x74` | `ret(0x25f58)` |
| `ldr` at 0x2a004 | load | 4 | `abs 0x2a3c0` | `-` |
| `ldr` at 0x2a008 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a014 | load | 4 | `abs 0x2a3c4` | `-` |
| `str` at 0x2a018 | store | 4 | `sp -0x7c` | `0xdb3c8` |
| `ldr` at 0x2a020 | load | 4 | `abs 0x2a3c8` | `-` |
| `str` at 0x2a034 | store | 4 | `sp -0x1c` | `0x10` |
| `ldr` at 0x2a038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a03c | load | 4 | `abs 0x2a3cc` | `-` |
| `ldr` at 0x2a048 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a050 | load | 4 | `abs 0x2a3cc` | `-` |
| `str` at 0x2a05c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a064 | load | 4 | `abs 0x2a3d0` | `-` |
| `ldr` at 0x2a070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a078 | load | 4 | `abs 0x2a3d0` | `-` |
| `str` at 0x2a084 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a088 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a08c | load | 4 | `abs 0x2a3d4` | `-` |
| `ldr` at 0x2a098 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a0a0 | load | 4 | `abs 0x2a3d4` | `-` |
| `str` at 0x2a0ac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a0b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a0c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a0d4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a0d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a0e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a0fc | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x2a108 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x2a10c | load | 4 | `abs 0x2a3d8` | `-` |
| `str` at 0x2a110 | store | 4 | `sp -0x78` | `0xdb3dc` |
| `ldr` at 0x2a114 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x2a130 | load | 4 | `abs 0x2a3dc` | `-` |
| `str` at 0x2a134 | store | 4 | `sp -0x7c` | `0xdb3ec` |
| `ldrb` at 0x2a138 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a13c | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x2a144 | load | 4 | `abs 0x2a3c8` | `-` |
| `ldr` at 0x2a148 | load | 4 | `abs 0x2a3e0` | `-` |
| `ldr` at 0x2a154 | load | 4 | `abs 0x2a3e4` | `-` |
| `str` at 0x2a158 | store | 4 | `sp -0x74` | `0xfffff894` |
| `str` at 0x2a168 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x2a16c | load | 4 | `abs 0x2a3e8` | `-` |
| `str` at 0x2a170 | store | 4 | `sp -0x78` | `0xdb41c` |
| `ldr` at 0x2a174 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x2a190 | load | 4 | `abs 0x2a3ec` | `-` |
| `str` at 0x2a194 | store | 4 | `sp -0x7c` | `0xdb42c` |
| `ldrb` at 0x2a198 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a19c | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x2a1a4 | load | 4 | `abs 0x2a3c8` | `-` |
| `ldr` at 0x2a1a8 | load | 4 | `abs 0x2a3f0` | `-` |
| `ldr` at 0x2a1b4 | load | 4 | `abs 0x2a3e4` | `-` |
| `str` at 0x2a1b8 | store | 4 | `sp -0x74` | `0xfffff894` |
| `ldr` at 0x2a1c0 | load | 4 | `abs 0x2a3c0` | `-` |
| `ldr` at 0x2a1c4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a1d0 | load | 4 | `abs 0x2a3f4` | `-` |
| `str` at 0x2a1d4 | store | 4 | `sp -0x7c` | `0xdb45c` |
| `ldr` at 0x2a1dc | load | 4 | `abs 0x2a3c8` | `-` |
| `ldr` at 0x2a1e0 | load | 4 | `abs 0x2a3f8` | `-` |
| `ldr` at 0x2a1ec | load | 4 | `abs 0x2a3fc` | `-` |
| `str` at 0x2a1f4 | store | 4 | `sp -0x7c` | `0x7e` |
| `ldr` at 0x2a1f8 | load | 4 | `abs 0x2a400` | `-` |
| `str` at 0x2a1fc | store | 4 | `sp -0x78` | `0xdb468` |
| `ldr` at 0x2a200 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x2a208 | load | 4 | `abs 0x119a40` | `-` |
| `ldr` at 0x2a220 | load | 4 | `abs 0x2a404` | `-` |
| `str` at 0x2a224 | store | 4 | `sp -0x7c` | `0xdb470` |
| `ldrb` at 0x2a228 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a22c | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x2a234 | load | 4 | `abs 0x2a3c8` | `-` |
| `ldr` at 0x2a238 | load | 4 | `abs 0x2a408` | `-` |
| `ldr` at 0x2a244 | load | 4 | `abs 0x2a40c` | `-` |
| `str` at 0x2a248 | store | 4 | `sp -0x74` | `0xfffff884` |
| `ldr` at 0x2a250 | load | 4 | `abs 0x2a3c0` | `-` |
| `ldr` at 0x2a254 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a260 | load | 4 | `abs 0x2a410` | `-` |
| `str` at 0x2a264 | store | 4 | `sp -0x7c` | `0xdb498` |
| `ldr` at 0x2a26c | load | 4 | `abs 0x2a3c8` | `-` |
| `ldr` at 0x2a270 | load | 4 | `abs 0x2a414` | `-` |
| `str` at 0x2a280 | store | 4 | `sp -0x1c` | `0xff` |
| `ldr` at 0x2a284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a288 | load | 4 | `abs 0x2a3cc` | `-` |
| `ldr` at 0x2a294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a29c | load | 4 | `abs 0x2a3cc` | `-` |
| `str` at 0x2a2a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a2ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a2b0 | load | 4 | `abs 0x2a3d0` | `-` |
| `ldr` at 0x2a2bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a2c4 | load | 4 | `abs 0x2a3d0` | `-` |
| `str` at 0x2a2d0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a2d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a2d8 | load | 4 | `abs 0x2a3d4` | `-` |
| `ldr` at 0x2a2e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a2ec | load | 4 | `abs 0x2a3d4` | `-` |
| `str` at 0x2a2f8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a2fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a30c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a320 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2a324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a334 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a348 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x2a354 | store | 4 | `sp -0x7c` | `0x8` |
| `ldr` at 0x2a358 | load | 4 | `abs 0x2a418` | `-` |
| `str` at 0x2a35c | store | 4 | `sp -0x78` | `0xdb4a8` |
| `ldr` at 0x2a360 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x2a37c | load | 4 | `abs 0x2a41c` | `-` |
| `str` at 0x2a380 | store | 4 | `sp -0x7c` | `0xdb4b4` |
| `ldrb` at 0x2a384 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a388 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x2a390 | load | 4 | `abs 0x2a3c8` | `-` |
| `ldr` at 0x2a394 | load | 4 | `abs 0x2a420` | `-` |
| `ldr` at 0x2a3a0 | load | 4 | `abs 0x2a424` | `-` |
| `str` at 0x2a3a4 | store | 4 | `sp -0x74` | `0xfffff892` |
| `str` at 0x2a3b0 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x2a3b4 | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0x2a3b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a3b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a3b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29f2c | `(unknown callee) 0x2896c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x29f58 | `(unknown callee) 0x28d68` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x29f8c | `(unknown callee) 0xbc3c4` | `0x1` | `0x0` | `sp-108` | `sp-108` |
| `bl` at 0x29fb4 | `(unknown callee) 0x27f00` | `[sp-17]:1` | `0x1` | `?` | `[sp-17]:1` |
| `bl` at 0x29fe0 | `(unknown callee) 0x25f58` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x2a02c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x750` | `0x0` |
| `bl` at 0x2a120 | `(unknown callee) 0xc9f88` | `[sp-112]` | `0x60` | `sp-28` | `0x0` |
| `bl` at 0x2a150 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x756` | `0x0` |
| `bl` at 0x2a180 | `(unknown callee) 0xc9f88` | `[sp-112]` | `0x62` | `sp-28` | `0x0` |
| `bl` at 0x2a1b0 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x75b` | `0x0` |
| `bl` at 0x2a1e8 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x763` | `0x0` |
| `bl` at 0x2a210 | `(unknown callee) 0xc9f88` | `[sp-112]` | `0x6` | `[0x119a40]` | `0x0` |
| `bl` at 0x2a240 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x767` | `0x0` |
| `bl` at 0x2a278 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x76f` | `0x0` |
| `bl` at 0x2a36c | `(unknown callee) 0xc9f88` | `[sp-112]` | `0x9` | `sp-28` | `0x0` |
| `bl` at 0x2a39c | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x775` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x29f40 | `ret(0x2896c) == 0x0` | `ret(0x2896c) != 0x0` |
| `beq` at 0x29f6c | `ret(0x28d68) == 0x0` | `ret(0x28d68) != 0x0` |
| `bne` at 0x29f98 | `ret(0xbc3c4) != 0x0` | `ret(0xbc3c4) == 0x0` |
| `beq` at 0x29fc8 | `ret(0x27f00) == 0x0` | `ret(0x27f00) != 0x0` |
| `beq` at 0x29ff4 | `ret(0x25f58) == 0x0` | `ret(0x25f58) != 0x0` |
| `beq` at 0x2a010 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x2a12c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x2a18c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x2a1cc | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x2a21c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x2a25c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x2a378 | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a3b8 | `r0` | `?` |

## `SHUB_Reset_PostRelease`

- `SHUB_Reset_PostRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a428 to 0x2a5d4.
  Established. The body includes two calls to `0x24790` with candidates `[sp-17]:1`, `0x500303a0`,
  `0x7`, `0x0` and `[sp-17]:1`, `0x500305a0`, `0x7`, `0x0` (`bl` at 0x2a4c8, `bl` at 0x2a52c). A
  nonzero result from the first takes the edge to further processing, while zero falls through
  to an unknown-value word store at `sp -0x18` (`bne` at 0x2a4d4, `str` at 0x2a4dc). A later
  call targets `0x251e8` with first and fourth candidates `[sp-17]:1`, second and third unknown
  (`bl` at 0x2a57c). The return reports `[sp-24]` (`ldmdb` at 0x2a5a0). The numeric values’ external
  reset effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2a43c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x2a444 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2a450 | load | 4 | `abs 0x2a5a4` | `-` |
| `ldrb` at 0x2a454 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x2a474 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2a480 | load | 4 | `abs 0x2a5a8` | `-` |
| `ldr` at 0x2a484 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a490 | load | 4 | `abs 0x2a5ac` | `-` |
| `str` at 0x2a494 | store | 4 | `sp -0x20` | `0xdb4e0` |
| `ldrb` at 0x2a498 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a49c | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x2a4a4 | load | 4 | `abs 0x2a5b0` | `-` |
| `ldr` at 0x2a4a8 | load | 4 | `abs 0x2a5b4` | `-` |
| `ldrb` at 0x2a4b4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2a4bc | load | 4 | `abs 0x2a5b8` | `-` |
| `str` at 0x2a4dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2a4e4 | load | 4 | `abs 0x2a5a8` | `-` |
| `ldr` at 0x2a4e8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a4f4 | load | 4 | `abs 0x2a5bc` | `-` |
| `str` at 0x2a4f8 | store | 4 | `sp -0x20` | `0xdb500` |
| `ldrb` at 0x2a4fc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a500 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x2a508 | load | 4 | `abs 0x2a5b0` | `-` |
| `ldr` at 0x2a50c | load | 4 | `abs 0x2a5c0` | `-` |
| `ldrb` at 0x2a518 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2a520 | load | 4 | `abs 0x2a5c4` | `-` |
| `str` at 0x2a540 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2a548 | load | 4 | `abs 0x2a5a8` | `-` |
| `ldr` at 0x2a54c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a558 | load | 4 | `abs 0x2a5c8` | `-` |
| `str` at 0x2a55c | store | 4 | `sp -0x20` | `0xdb520` |
| `ldr` at 0x2a564 | load | 4 | `abs 0x2a5b0` | `-` |
| `ldr` at 0x2a568 | load | 4 | `abs 0x2a5cc` | `-` |
| `ldrb` at 0x2a574 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2a584 | store | 4 | `sp -0x18` | `ret(0x251e8)` |
| `ldrb` at 0x2a588 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2a58c | load | 4 | `abs 0x2a5d0` | `-` |
| `ldr` at 0x2a598 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2a5a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a5a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a5a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2a44c | `(unknown callee) 0x2ad48` | `0x1` | `r1` | `r2` | `0x0` |
| `bl` at 0x2a4b0 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x79e` | `0x0` |
| `bl` at 0x2a4c8 | `(unknown callee) 0x24790` | `[sp-17]:1` | `0x500303a0` | `0x7` | `0x0` |
| `bl` at 0x2a514 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x7a3` | `0x0` |
| `bl` at 0x2a52c | `(unknown callee) 0x24790` | `[sp-17]:1` | `0x500305a0` | `0x7` | `0x0` |
| `bl` at 0x2a570 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x7a9` | `0x0` |
| `bl` at 0x2a57c | `(unknown callee) 0x251e8` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x2a594 | `(unknown callee) 0x1dba4` | `0xdb560` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2a47c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x2a48c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bne` at 0x2a4d4 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `beq` at 0x2a4f0 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bne` at 0x2a538 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `beq` at 0x2a554 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a5a0 | `r0` | `[sp-24]` |

## `SHUB_ScanResetNI`

- `SHUB_ScanResetNI`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x257b0 to 0x25950.
  Established. The two calls to `0x25388` have candidates `[sp-28]`, `0x0`, `0x1`, unknown and
  `[sp-28]`, `0x1`, `0x1`, unknown (`bl` at 0x2585c, `bl` at 0x258f0). The loop increments the word
  at `sp -0x1c` to `[sp-28]+1` (`str` at 0x25934); its repeat branch retains only a signed less-than
  flags predicate with unknown operands (`blt` at 0x257e8). The tested values before the two calls
  are unknown (`bne` at 0x25820, `bne` at 0x258ac). The affected interface and reset mechanism
  remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x257c4 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x257cc | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x257d4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x257d8 | load | 4 | `abs 0x25948` | `-` |
| `ldrb` at 0x257dc | load | 1 | `abs 0x119a24` | `-` |
| `ldr` at 0x257e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x257f0 | load | 4 | `abs 0x2594c` | `-` |
| `ldr` at 0x257f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x25814 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x25824 | load | 4 | `abs 0x2594c` | `-` |
| `ldr` at 0x25828 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x25848 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x25850 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25864 | store | 4 | `sp -0x18` | `ret(0x25388)` |
| `ldr` at 0x25868 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25874 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x25878 | store | 4 | `sp -0x14` | `ret(0x25388)` |
| `ldr` at 0x2587c | load | 4 | `abs 0x2594c` | `-` |
| `ldr` at 0x25880 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x258a0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x258b4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x258b8 | load | 4 | `abs 0x2594c` | `-` |
| `ldr` at 0x258bc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x258dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x258e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25900 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25910 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x25914 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x25918 | store | 4 | `sp -0x18` | `[sp-32]` |
| `ldr` at 0x25924 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x25928 | store | 4 | `sp -0x14` | `[sp-32]` |
| `ldr` at 0x2592c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25934 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x2593c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x25944 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25944 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2585c | `(unknown callee) 0x25388` | `[sp-28]` | `0x0` | `0x1` | `?` |
| `bl` at 0x258f0 | `(unknown callee) 0x25388` | `[sp-28]` | `0x1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x257e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x25820 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x25870 | `ret(0x25388) == 0x0` | `ret(0x25388) != 0x0` |
| `bne` at 0x258ac | `? != 0x0` | `? == 0x0` |
| `beq` at 0x258fc | `ret(0x25388) == 0x0` | `ret(0x25388) != 0x0` |
| `bne` at 0x25908 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x25920 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25944 | `r0` | `?` |

## `SHUB_ScanResetLLP`

- `SHUB_ScanResetLLP`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25950 to 0x25da4.
  Established. Calls to `0x24790` include second candidates `0x500303b0` and `0x500305b0`, both with
  third `0x1` and fourth unknown (`bl` at 0x259bc, `bl` at 0x259e8). Other calls to that target use
  `0x50000008` and `0x50002008` as second candidates, both with third `0x0` and fourth `0x1` (`bl`
  at 0x25b6c, `bl` at 0x25b98). The body also calls `0x2ad48` with first candidate `0x1c2` (`bl` at
  0x25a18). Two later equality branches take their edges for `[sp-24] == 0x9` and fall through for
  inequality (`beq` at 0x25c28, `beq` at 0x25cd0). Candidate units and physical link-reset semantics
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x25960 | load | 4 | `abs 0x25d5c` | `-` |
| `ldr` at 0x25964 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x25970 | load | 4 | `abs 0x25d60` | `-` |
| `str` at 0x25974 | store | 4 | `sp -0x2c` | `0xda7d4` |
| `ldr` at 0x2597c | load | 4 | `abs 0x25d64` | `-` |
| `ldr` at 0x25980 | load | 4 | `abs 0x25d68` | `-` |
| `str` at 0x25990 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x25994 | load | 4 | `abs 0x25d6c` | `-` |
| `ldrb` at 0x25998 | load | 1 | `abs 0x119a24` | `-` |
| `ldr` at 0x2599c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x259ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x259b0 | load | 4 | `abs 0x25d70` | `-` |
| `ldr` at 0x259cc | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x259d0 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x259d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x259dc | load | 4 | `abs 0x25d78` | `-` |
| `ldr` at 0x259f8 | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x259fc | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25a04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25a0c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x25a14 | load | 4 | `abs 0x25d7c` | `-` |
| `str` at 0x25a20 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x25a24 | load | 4 | `abs 0x25d6c` | `-` |
| `ldrb` at 0x25a28 | load | 1 | `abs 0x119a24` | `-` |
| `ldr` at 0x25a2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25a44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25a48 | load | 4 | `abs 0x25d70` | `-` |
| `ldr` at 0x25a64 | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x25a68 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25a70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25a80 | load | 4 | `abs 0x25d80` | `-` |
| `str` at 0x25a84 | store | 4 | `sp -0x2c` | `0xda7e8` |
| `ldr` at 0x25a88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25a8c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x25a90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25a94 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x25a9c | load | 4 | `abs 0x25d64` | `-` |
| `ldr` at 0x25aa0 | load | 4 | `abs 0x25d84` | `-` |
| `ldr` at 0x25abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25ac0 | load | 4 | `abs 0x25d78` | `-` |
| `ldr` at 0x25adc | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x25ae0 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25af8 | load | 4 | `abs 0x25d88` | `-` |
| `str` at 0x25afc | store | 4 | `sp -0x2c` | `0xda818` |
| `ldr` at 0x25b00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25b04 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x25b08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25b0c | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x25b14 | load | 4 | `abs 0x25d64` | `-` |
| `ldr` at 0x25b2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25b34 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x25b40 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x25b44 | load | 4 | `abs 0x25d6c` | `-` |
| `ldrb` at 0x25b48 | load | 1 | `abs 0x119a24` | `-` |
| `ldr` at 0x25b4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25b5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25b7c | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x25b80 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25b88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25b8c | load | 4 | `abs 0x25d8c` | `-` |
| `ldr` at 0x25ba8 | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x25bac | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25bb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25bbc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x25bd0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x25bd4 | load | 4 | `abs 0x25d6c` | `-` |
| `ldrb` at 0x25bd8 | load | 1 | `abs 0x119a24` | `-` |
| `ldr` at 0x25bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25c14 | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x25c18 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25c20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25c2c | load | 4 | `abs 0x25d90` | `-` |
| `ldr` at 0x25c30 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x25c50 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x25c60 | load | 4 | `abs 0x25d94` | `-` |
| `str` at 0x25c64 | store | 4 | `sp -0x2c` | `0xda848` |
| `ldr` at 0x25c68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25c6c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x25c70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x25c74 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x25c7c | load | 4 | `abs 0x25d64` | `-` |
| `ldr` at 0x25c80 | load | 4 | `abs 0x25d98` | `-` |
| `ldr` at 0x25c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25ca0 | load | 4 | `abs 0x25d9c` | `-` |
| `ldr` at 0x25cbc | load | 4 | `abs 0x25d74` | `-` |
| `str` at 0x25cc0 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x25cc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25cd4 | load | 4 | `abs 0x25d90` | `-` |
| `ldr` at 0x25cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x25cf8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x25d08 | load | 4 | `abs 0x25da0` | `-` |
| `str` at 0x25d0c | store | 4 | `sp -0x2c` | `0xda878` |
| `ldr` at 0x25d10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25d14 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x25d18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x25d1c | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x25d24 | load | 4 | `abs 0x25d64` | `-` |
| `ldr` at 0x25d3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x25d44 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x25d50 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x25d54 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x25d58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25d58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25d58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25988 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x27b` | `0x0` |
| `bl` at 0x259bc | `(unknown callee) 0x24790` | `?` | `0x500303b0` | `0x1` | `?` |
| `bl` at 0x259e8 | `(unknown callee) 0x24790` | `[sp-28]` | `0x500305b0` | `0x1` | `?` |
| `bl` at 0x25a18 | `(unknown callee) 0x2ad48` | `0x1c2` | `?` | `[0x119a24]:1` | `?` |
| `bl` at 0x25a54 | `(unknown callee) 0x247f4` | `?` | `0x500303b0` | `sp-20` | `sp-24` |
| `bl` at 0x25aa8 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x28d` | `0x0` |
| `bl` at 0x25ab0 | `(unknown callee) 0x2ad48` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x25acc | `(unknown callee) 0x247f4` | `[sp-28]` | `0x500305b0` | `sp-20` | `sp-24` |
| `bl` at 0x25b20 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x294` | `0x0` |
| `bl` at 0x25b28 | `(unknown callee) 0x2ad48` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x25b6c | `(unknown callee) 0x24790` | `?` | `0x50000008` | `0x0` | `0x1` |
| `bl` at 0x25b98 | `(unknown callee) 0x24790` | `[sp-28]` | `0x50002008` | `0x0` | `0x1` |
| `bl` at 0x25bc8 | `(unknown callee) 0x2ad48` | `0xa` | `?` | `[0x119a24]:1` | `?` |
| `bl` at 0x25c04 | `(unknown callee) 0x247f4` | `?` | `0x50000000` | `sp-20` | `sp-24` |
| `bl` at 0x25c88 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x2ad` | `0x0` |
| `bl` at 0x25c90 | `(unknown callee) 0x2ad48` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x25cac | `(unknown callee) 0x247f4` | `[sp-28]` | `0x50002000` | `sp-20` | `sp-24` |
| `bl` at 0x25d30 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x2b4` | `0x0` |
| `bl` at 0x25d38 | `(unknown callee) 0x2ad48` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2596c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `blt` at 0x259a4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x259c8 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `bne` at 0x259f4 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `blt` at 0x25a34 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x25a60 | `ret(0x247f4) != 0x0` | `ret(0x247f4) == 0x0` |
| `bne` at 0x25a7c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x25ad8 | `ret(0x247f4) != 0x0` | `ret(0x247f4) == 0x0` |
| `bne` at 0x25af4 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x25b54 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x25b78 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `bne` at 0x25ba4 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `blt` at 0x25be4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x25c10 | `ret(0x247f4) != 0x0` | `ret(0x247f4) == 0x0` |
| `beq` at 0x25c28 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x25c5c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x25cb8 | `ret(0x247f4) != 0x0` | `ret(0x247f4) == 0x0` |
| `beq` at 0x25cd0 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x25d04 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25d58 | `r0` | `?` |

## `SHUB_ScanInitLLP`

- `SHUB_ScanInitLLP`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25f58 to 0x264ec.
  Established. An explicit-link call has unknown target and candidates `[sp-20]`, unknown, unknown,
  `[0x119a38]` (`mov` at 0x260a0); its equality branches select taken paths for `ret(unknown) ==
  0x1`, `0x0` and `0x2` (`beq` at 0x260bc, `beq` at 0x260d4, `beq` at 0x260e4). Halfword stores
  include `0x5` and `0x0` at unresolved effective targets (`strh` at 0x26120, `strh` at 0x2615c,
  `strh` at 0x261cc). Later calls to `0xc9f88` use candidates `[sp-116]`, `0xa`, `sp-32`, `0x0` and
  `[sp-116]`, `0xb`, `sp-32`, `0x0` (`bl` at 0x262f4, `bl` at 0x26448). The stored halfwords’ state
  meaning and the callees’ scan operation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25f68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x25f70 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x25f74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25f7c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x25f80 | load | 4 | `abs 0x26498` | `-` |
| `ldr` at 0x25f84 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x25f90 | load | 4 | `abs 0x2649c` | `-` |
| `str` at 0x25f94 | store | 4 | `sp -0x90` | `0xda904` |
| `ldr` at 0x25f98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25f9c | store | 4 | `sp -0x8c` | `r0` |
| `ldr` at 0x25fa4 | load | 4 | `abs 0x264a0` | `-` |
| `ldr` at 0x25fa8 | load | 4 | `abs 0x264a4` | `-` |
| `str` at 0x25fe0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x25fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25ff4 | load | 4 | `abs 0x264a8` | `-` |
| `ldr` at 0x25ff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x26008 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x26014 | load | 4 | `abs 0x26498` | `-` |
| `ldr` at 0x26018 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x26024 | load | 4 | `abs 0x264ac` | `-` |
| `str` at 0x26028 | store | 4 | `sp -0x90` | `0xda918` |
| `ldr` at 0x2602c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x26030 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x26034 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x26038 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x2603c | load | 4 | `abs 0x264b0` | `-` |
| `ldr` at 0x26040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26054 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x2606c | load | 2 | `(unknown)` | `-` |
| `str` at 0x26070 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x26078 | load | 4 | `abs 0x264a0` | `-` |
| `ldr` at 0x2607c | load | 4 | `abs 0x264b4` | `-` |
| `ldr` at 0x2608c | load | 4 | `abs 0x264b8` | `-` |
| `ldr` at 0x26090 | load | 4 | `abs 0x119a38` | `-` |
| `ldr` at 0x26094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26098 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x260a8 | store | 4 | `sp -0x78` | `ret(unknown)` |
| `ldr` at 0x260ac | load | 4 | `sp -0x78` | `-` |
| `str` at 0x260b0 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x260b4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x260c0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x260cc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x260dc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x260ec | load | 4 | `abs 0x264b0` | `-` |
| `ldr` at 0x260f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26104 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x26120 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x26128 | load | 4 | `abs 0x264b0` | `-` |
| `ldr` at 0x2612c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x26140 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x2615c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x26164 | load | 4 | `abs 0x26498` | `-` |
| `ldr` at 0x26168 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x26174 | load | 4 | `abs 0x264bc` | `-` |
| `str` at 0x26178 | store | 4 | `sp -0x90` | `0xda954` |
| `ldr` at 0x2617c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x26180 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x26188 | load | 4 | `abs 0x264a0` | `-` |
| `ldr` at 0x2618c | load | 4 | `abs 0x264c0` | `-` |
| `ldr` at 0x26198 | load | 4 | `abs 0x264b0` | `-` |
| `ldr` at 0x2619c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x261b0 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x261cc | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x261d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x261d8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x261e0 | load | 4 | `abs 0x264b0` | `-` |
| `ldr` at 0x261e4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x26204 | load | 2 | `(unknown)` | `-` |
| `str` at 0x26208 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2620c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26210 | load | 4 | `abs 0x264c4` | `-` |
| `ldr` at 0x2621c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26224 | load | 4 | `abs 0x264c4` | `-` |
| `str` at 0x26230 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x26234 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26238 | load | 4 | `abs 0x264c8` | `-` |
| `ldr` at 0x26244 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2624c | load | 4 | `abs 0x264c8` | `-` |
| `str` at 0x26258 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2625c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26260 | load | 4 | `abs 0x264cc` | `-` |
| `ldr` at 0x2626c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26274 | load | 4 | `abs 0x264cc` | `-` |
| `str` at 0x26280 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x26284 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26294 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x262a8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x262ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x262bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x262d0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x262dc | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x262e0 | load | 4 | `abs 0x264d0` | `-` |
| `str` at 0x262e4 | store | 4 | `sp -0x8c` | `0xda984` |
| `ldr` at 0x262e8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x26304 | load | 4 | `abs 0x264d4` | `-` |
| `str` at 0x26308 | store | 4 | `sp -0x90` | `0xda990` |
| `ldr` at 0x2630c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x26310 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x26318 | load | 4 | `abs 0x264a0` | `-` |
| `ldr` at 0x2631c | load | 4 | `abs 0x264d8` | `-` |
| `ldr` at 0x26328 | load | 4 | `abs 0x264dc` | `-` |
| `str` at 0x2632c | store | 4 | `sp -0x7c` | `0xfffff888` |
| `ldr` at 0x26334 | load | 4 | `abs 0x264b0` | `-` |
| `ldr` at 0x26338 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x26358 | load | 2 | `(unknown)` | `-` |
| `str` at 0x2635c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x26360 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26364 | load | 4 | `abs 0x264c4` | `-` |
| `ldr` at 0x26370 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26378 | load | 4 | `abs 0x264c4` | `-` |
| `str` at 0x26384 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x26388 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2638c | load | 4 | `abs 0x264c8` | `-` |
| `ldr` at 0x26398 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x263a0 | load | 4 | `abs 0x264c8` | `-` |
| `str` at 0x263ac | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x263b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x263b4 | load | 4 | `abs 0x264cc` | `-` |
| `ldr` at 0x263c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x263c8 | load | 4 | `abs 0x264cc` | `-` |
| `str` at 0x263d4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x263d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x263e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x263fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x26400 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x26410 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x26424 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x26430 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x26434 | load | 4 | `abs 0x264e0` | `-` |
| `str` at 0x26438 | store | 4 | `sp -0x8c` | `0xda9bc` |
| `ldr` at 0x2643c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x26458 | load | 4 | `abs 0x264e4` | `-` |
| `str` at 0x2645c | store | 4 | `sp -0x90` | `0xda9c8` |
| `ldr` at 0x26460 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x26464 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x2646c | load | 4 | `abs 0x264a0` | `-` |
| `ldr` at 0x26470 | load | 4 | `abs 0x264e8` | `-` |
| `ldr` at 0x2647c | load | 4 | `abs 0x264dc` | `-` |
| `str` at 0x26480 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `str` at 0x2648c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x26490 | load | 4 | `sp -0x7c` | `-` |
| `ldmdb` at 0x26494 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x26494 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x26494 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25fb0 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x2fd` | `0x0` |
| `bl` at 0x25fc4 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x25fd8 | `(unknown callee) 0xbc4e8` | `0x0` | `0x0` | `0x20` | `sp-112` |
| `bl` at 0x26084 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x30b` | `0x0` |
| `mov` at 0x260a0 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[0x119a38]` |
| `bl` at 0x26194 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x31a` | `0x0` |
| `bl` at 0x262f4 | `(unknown callee) 0xc9f88` | `[sp-116]` | `0xa` | `sp-32` | `0x0` |
| `bl` at 0x26324 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x329` | `0x0` |
| `bl` at 0x26448 | `(unknown callee) 0xc9f88` | `[sp-116]` | `0xb` | `sp-32` | `0x0` |
| `bl` at 0x26478 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x331` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25f8c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `ble` at 0x25fec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x26010 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x26020 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x260bc | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |
| `bgt` at 0x260c8 | `ret(unknown) > 0x1 (signed, 32-bit)` | `ret(unknown) <= 0x1 (signed, 32-bit)` |
| `beq` at 0x260d4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x260e4 | `ret(unknown) == 0x2` | `ret(unknown) != 0x2` |
| `beq` at 0x26170 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x26300 | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x26454 | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x26494 | `r0` | `?` |

## `SHUB_ScanInit_LLP_TEST_CTL`

- `SHUB_ScanInit_LLP_TEST_CTL`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25da4 to 0x25f58.
  Established. Separate paths store words `0xa00300` and `0xa00000` at `sp -0x18` (`str` at 0x25e38,
  `str` at 0x25e44). Their deciding branch retains an unknown tested value (`bne` at 0x25e30). Calls
  to `0x24790` have candidates `[sp-20]`, `0x50000028`, `[sp-24]`, `0xa5fffff` and `[sp-20]`,
  `0x50002028`, `[sp-24]`, `0xa5fffff` (`bl` at 0x25e58, `bl` at 0x25ed8). Each nonzero result takes
  the continuation edge, while zero falls through to additional calls and a stack result store
  (`bne` at 0x25e64, `bne` at 0x25ee4). The numeric candidates’ test-control meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x25db4 | load | 4 | `abs 0x25f20` | `-` |
| `ldr` at 0x25db8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x25dc4 | load | 4 | `abs 0x25f24` | `-` |
| `str` at 0x25dc8 | store | 4 | `sp -0x20` | `0xda8a8` |
| `ldr` at 0x25dd0 | load | 4 | `abs 0x25f28` | `-` |
| `ldr` at 0x25dd4 | load | 4 | `abs 0x25f2c` | `-` |
| `str` at 0x25de4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x25de8 | load | 4 | `abs 0x25f30` | `-` |
| `ldrb` at 0x25dec | load | 1 | `abs 0x119a24` | `-` |
| `ldr` at 0x25df0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e00 | load | 4 | `abs 0x25f34` | `-` |
| `ldr` at 0x25e04 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x25e24 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x25e34 | load | 4 | `abs 0x25f38` | `-` |
| `str` at 0x25e38 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x25e44 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x25e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e4c | load | 4 | `abs 0x25f3c` | `-` |
| `ldr` at 0x25e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25e54 | load | 4 | `abs 0x25f40` | `-` |
| `ldr` at 0x25e68 | load | 4 | `abs 0x25f44` | `-` |
| `ldr` at 0x25e6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e74 | load | 4 | `abs 0x25f48` | `-` |
| `str` at 0x25e78 | store | 4 | `sp -0x1c` | `0xfffff88a` |
| `ldr` at 0x25e80 | load | 4 | `abs 0x25f34` | `-` |
| `ldr` at 0x25e84 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x25ea4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x25eb4 | load | 4 | `abs 0x25f38` | `-` |
| `str` at 0x25eb8 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x25ec4 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x25ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ecc | load | 4 | `abs 0x25f4c` | `-` |
| `ldr` at 0x25ed0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25ed4 | load | 4 | `abs 0x25f40` | `-` |
| `ldr` at 0x25ee8 | load | 4 | `abs 0x25f50` | `-` |
| `ldr` at 0x25eec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ef4 | load | 4 | `abs 0x25f54` | `-` |
| `str` at 0x25ef8 | store | 4 | `sp -0x1c` | `0xfffff889` |
| `ldr` at 0x25f00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25f08 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x25f14 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x25f18 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x25f1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25f1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25f1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25ddc | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x2ca` | `0x0` |
| `bl` at 0x25e58 | `(unknown callee) 0x24790` | `[sp-20]` | `0x50000028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x25e70 | `(unknown callee) 0x1dba4` | `0xda8c4` | `[sp-20]` | `?` | `ret(0x24790)` |
| `bl` at 0x25ed8 | `(unknown callee) 0x24790` | `[sp-20]` | `0x50002028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x25ef0 | `(unknown callee) 0x1dba4` | `0xda8e4` | `[sp-20]` | `?` | `ret(0x24790)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25dc0 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `blt` at 0x25df8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x25e30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x25e64 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |
| `bne` at 0x25eb0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x25ee4 | `ret(0x24790) != 0x0` | `ret(0x24790) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25f1c | `r0` | `?` |

## `SHUB_ScanInitConfig`

- `SHUB_ScanInitConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27f00 to 0x2896c.
  Established. A zero halfword at absolute `0x280f60` takes the edge to a halfword store of
  `0x320` there, while nonzero falls through to further testing (`beq` at 0x28034, `strh` at
  0x2807c). The body includes calls to `0xc9f88` with second candidates `0x7c`, `0x7e`, `0x1a`,
  `0x1b` and `0x7d` (`bl` at 0x27fe8, `bl` at 0x285c0, `bl` at 0x286d8, `bl` at 0x287b0, `bl` at
  0x28810). A nonzero `[sp-24]` takes the branch into the later call paths, while zero falls through
  into calls to `0xbc308` and `0xbc4e8` (`bne` at 0x28608, `bl` at 0x2861c, `bl` at 0x28630). The
  earlier conditional paths include halfword stores with unresolved targets, carrying `([[sp-172]+8]:2 &
  0xffff)` and `([[sp-172]+10]:2 & 0xffff)` (`strh` at 0x28408, `strh` at 0x2852c). Their hardware
  configuration meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27f10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27f14 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x27f20 | store | 4 | `sp -0xa4` | `0x0` |
| `str` at 0x27f2c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x27f38 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x27f44 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x27f50 | store | 4 | `sp -0x94` | `0x0` |
| `str` at 0x27f5c | store | 4 | `sp -0x90` | `0x0` |
| `str` at 0x27f68 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x27f74 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x27f78 | load | 4 | `abs 0x288b4` | `-` |
| `str` at 0x27f7c | store | 4 | `sp -0xa4` | `0x7ffffd28` |
| `ldr` at 0x27f80 | load | 4 | `abs 0x288b8` | `-` |
| `str` at 0x27f84 | store | 4 | `sp -0xa0` | `0x602800` |
| `ldr` at 0x27f88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27f90 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x27f94 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x27f98 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x27fa4 | load | 4 | `abs 0x288c0` | `-` |
| `str` at 0x27fa8 | store | 4 | `sp -0xc4` | `0xdad70` |
| `ldr` at 0x27fac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27fb0 | store | 4 | `sp -0xc0` | `r0` |
| `ldr` at 0x27fb8 | load | 4 | `abs 0x288c4` | `-` |
| `str` at 0x27fcc | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x27fd0 | load | 4 | `abs 0x288c8` | `-` |
| `str` at 0x27fd4 | store | 4 | `sp -0xc0` | `0xdad8c` |
| `ldr` at 0x27fd8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x27ff8 | load | 4 | `abs 0x288cc` | `-` |
| `str` at 0x27ffc | store | 4 | `sp -0xc4` | `0xdad98` |
| `ldr` at 0x28000 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28004 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x2800c | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28010 | load | 4 | `abs 0x288d0` | `-` |
| `ldr` at 0x2801c | load | 4 | `abs 0x288d4` | `-` |
| `str` at 0x28020 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x28028 | load | 4 | `abs 0x288d8` | `-` |
| `ldrh` at 0x2802c | load | 2 | `abs 0x280f60` | `-` |
| `ldr` at 0x28038 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x2803c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28048 | load | 4 | `abs 0x288dc` | `-` |
| `str` at 0x2804c | store | 4 | `sp -0xc4` | `0xdadc8` |
| `ldr` at 0x28050 | load | 4 | `abs 0x288d8` | `-` |
| `ldrh` at 0x28054 | load | 2 | `abs 0x280f60` | `-` |
| `str` at 0x28058 | store | 4 | `sp -0xc0` | `[0x280f60]:2` |
| `ldr` at 0x28060 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28064 | load | 4 | `abs 0x288e0` | `-` |
| `ldr` at 0x28074 | load | 4 | `abs 0x288d8` | `-` |
| `strh` at 0x2807c | store | 2 | `abs 0x280f60` | `0x320` |
| `ldr` at 0x28080 | load | 4 | `abs 0x288e4` | `-` |
| `ldrb` at 0x28084 | load | 1 | `abs 0x119a2c` | `-` |
| `str` at 0x28094 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0x28098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x280a0 | load | 4 | `abs 0x288e8` | `-` |
| `ldr` at 0x280a4 | load | 4 | `abs 0x119a28` | `-` |
| `ldrb` at 0x280ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x280b0 | load | 4 | `abs 0x288e4` | `-` |
| `ldrb` at 0x280b4 | load | 1 | `abs 0x119a2c` | `-` |
| `ldr` at 0x280bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x280d0 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x280d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x280f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x28100 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28104 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x28124 | load | 1 | `(unknown)` | `-` |
| `str` at 0x28128 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x2812c | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x28130 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2813c | load | 4 | `abs 0x288f0` | `-` |
| `str` at 0x28140 | store | 4 | `sp -0xc4` | `0xdade0` |
| `ldr` at 0x28144 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28148 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x2814c | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x28150 | store | 4 | `sp -0xbc` | `[sp-176]` |
| `ldr` at 0x28158 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x2815c | load | 4 | `abs 0x288f4` | `-` |
| `ldr` at 0x2816c | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x28170 | load | 1 | `[sp-172] +0x6` | `-` |
| `str` at 0x28174 | store | 4 | `sp -0xb0` | `[[sp-172]+6]:1` |
| `ldr` at 0x28178 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x2817c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28188 | load | 4 | `abs 0x288f8` | `-` |
| `str` at 0x2818c | store | 4 | `sp -0xc4` | `0xdae04` |
| `ldr` at 0x28190 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28194 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28198 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x2819c | store | 4 | `sp -0xbc` | `[[sp-172]+6]:1` |
| `ldr` at 0x281a4 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x281b4 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x281b8 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x281bc | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x281c8 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x281d4 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x281e4 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x281e8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x28208 | load | 1 | `sp -0xb0` | `-` |
| `strb` at 0x2820c | store | 1 | `(unknown)` | `([sp-176]:1 & 0xff)` |
| `ldr` at 0x28214 | load | 4 | `abs 0x288fc` | `-` |
| `str` at 0x28218 | store | 4 | `sp -0xc4` | `0xdae1c` |
| `ldr` at 0x2821c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28220 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28228 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x2822c | load | 4 | `abs 0x28900` | `-` |
| `ldr` at 0x28238 | load | 4 | `abs 0x288d4` | `-` |
| `str` at 0x2823c | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x28244 | load | 4 | `abs 0x28904` | `-` |
| `ldr` at 0x28248 | load | 4 | `abs 0x119a44` | `-` |
| `ldr` at 0x28254 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x28258 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28264 | load | 4 | `abs 0x28908` | `-` |
| `str` at 0x28268 | store | 4 | `sp -0xc4` | `0xdae70` |
| `ldr` at 0x2826c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28270 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28274 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28278 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x28298 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2829c | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x282a4 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x282a8 | load | 4 | `abs 0x2890c` | `-` |
| `ldr` at 0x282b8 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x282bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x282dc | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x282e0 | load | 1 | `[sp-172] +0xc` | `-` |
| `strb` at 0x282e4 | store | 1 | `(unknown)` | `([[sp-172]+12]:1 & 0xff)` |
| `ldr` at 0x282e8 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x282ec | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x282f8 | load | 4 | `abs 0x28910` | `-` |
| `str` at 0x282fc | store | 4 | `sp -0xc4` | `0xdae94` |
| `ldr` at 0x28300 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28304 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28308 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x2830c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x2832c | load | 1 | `(unknown)` | `-` |
| `str` at 0x28330 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x28338 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x2833c | load | 4 | `abs 0x28914` | `-` |
| `ldr` at 0x28348 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x2834c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x2836c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x28378 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x2837c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28388 | load | 4 | `abs 0x28918` | `-` |
| `str` at 0x2838c | store | 4 | `sp -0xc4` | `0xdaeb8` |
| `ldr` at 0x28390 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28394 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28398 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x2839c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x283bc | load | 2 | `(unknown)` | `-` |
| `str` at 0x283c0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x283c8 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x283cc | load | 4 | `abs 0x2891c` | `-` |
| `ldr` at 0x283dc | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x283e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28400 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x28404 | load | 2 | `[sp-172] +0x8` | `-` |
| `strh` at 0x28408 | store | 2 | `(unknown)` | `([[sp-172]+8]:2 & 0xffff)` |
| `ldr` at 0x2840c | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x28410 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2841c | load | 4 | `abs 0x28920` | `-` |
| `str` at 0x28420 | store | 4 | `sp -0xc4` | `0xdaee4` |
| `ldr` at 0x28424 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28428 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x2842c | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28430 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x28450 | load | 2 | `(unknown)` | `-` |
| `str` at 0x28454 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x2845c | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28460 | load | 4 | `abs 0x28924` | `-` |
| `ldr` at 0x2846c | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28470 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x28490 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x2849c | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x284a0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x284ac | load | 4 | `abs 0x28928` | `-` |
| `str` at 0x284b0 | store | 4 | `sp -0xc4` | `0xdaf04` |
| `ldr` at 0x284b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x284b8 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x284bc | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x284c0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x284e0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x284e4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x284ec | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x284f0 | load | 4 | `abs 0x2892c` | `-` |
| `ldr` at 0x28500 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28524 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x28528 | load | 2 | `[sp-172] +0xa` | `-` |
| `strh` at 0x2852c | store | 2 | `(unknown)` | `([[sp-172]+10]:2 & 0xffff)` |
| `ldr` at 0x28530 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x28534 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28540 | load | 4 | `abs 0x28930` | `-` |
| `str` at 0x28544 | store | 4 | `sp -0xc4` | `0xdaf30` |
| `ldr` at 0x28548 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2854c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28550 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28554 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x28574 | load | 2 | `(unknown)` | `-` |
| `str` at 0x28578 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x28580 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28584 | load | 4 | `abs 0x28934` | `-` |
| `ldr` at 0x28594 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x285a8 | store | 4 | `sp -0xc4` | `0x38` |
| `ldr` at 0x285ac | load | 4 | `abs 0x28938` | `-` |
| `str` at 0x285b0 | store | 4 | `sp -0xc0` | `0xdaf50` |
| `ldr` at 0x285b4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x285d0 | load | 4 | `abs 0x2893c` | `-` |
| `str` at 0x285d4 | store | 4 | `sp -0xc4` | `0xdaf58` |
| `ldr` at 0x285d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x285dc | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x285e4 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x285f4 | load | 4 | `abs 0x288d4` | `-` |
| `str` at 0x285f8 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x28600 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28638 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x28640 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x28644 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x28664 | load | 2 | `(unknown)` | `-` |
| `str` at 0x286c0 | store | 4 | `sp -0xc4` | `0xf2` |
| `ldr` at 0x286c4 | load | 4 | `abs 0x28940` | `-` |
| `str` at 0x286c8 | store | 4 | `sp -0xc0` | `0xdaf80` |
| `ldr` at 0x286cc | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x286e8 | load | 4 | `abs 0x28944` | `-` |
| `str` at 0x286ec | store | 4 | `sp -0xc4` | `0xdaf90` |
| `ldr` at 0x286f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x286f4 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x286fc | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28700 | load | 4 | `abs 0x28948` | `-` |
| `ldr` at 0x2870c | load | 4 | `abs 0x288d4` | `-` |
| `str` at 0x28710 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x28718 | load | 4 | `abs 0x288ec` | `-` |
| `ldr` at 0x2871c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x2873c | load | 2 | `(unknown)` | `-` |
| `str` at 0x28798 | store | 4 | `sp -0xc4` | `0xf2` |
| `ldr` at 0x2879c | load | 4 | `abs 0x2894c` | `-` |
| `str` at 0x287a0 | store | 4 | `sp -0xc0` | `0xdafc0` |
| `ldr` at 0x287a4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x287c0 | load | 4 | `abs 0x28950` | `-` |
| `str` at 0x287c4 | store | 4 | `sp -0xc4` | `0xdafd0` |
| `ldr` at 0x287c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x287cc | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x287d4 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x287d8 | load | 4 | `abs 0x28954` | `-` |
| `ldr` at 0x287e4 | load | 4 | `abs 0x288d4` | `-` |
| `str` at 0x287e8 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `str` at 0x287f4 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x287f8 | load | 4 | `abs 0x28958` | `-` |
| `str` at 0x287fc | store | 4 | `sp -0xc0` | `0xdb000` |
| `ldr` at 0x28800 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x28820 | load | 4 | `abs 0x2895c` | `-` |
| `str` at 0x28824 | store | 4 | `sp -0xc4` | `0xdb00c` |
| `ldr` at 0x28828 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2882c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x28834 | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28838 | load | 4 | `abs 0x28960` | `-` |
| `ldr` at 0x28844 | load | 4 | `abs 0x288d4` | `-` |
| `str` at 0x28848 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x28850 | load | 4 | `abs 0x288bc` | `-` |
| `ldr` at 0x28854 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28860 | load | 4 | `abs 0x28964` | `-` |
| `str` at 0x28864 | store | 4 | `sp -0xc4` | `0xdb038` |
| `ldr` at 0x2886c | load | 4 | `abs 0x288c4` | `-` |
| `ldr` at 0x28870 | load | 4 | `abs 0x28968` | `-` |
| `str` at 0x288a8 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x288ac | load | 4 | `sp -0xb4` | `-` |
| `ldmdb` at 0x288b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x288b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x288b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27fc4 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x4b0` | `0x0` |
| `bl` at 0x27fe8 | `(unknown callee) 0xc9f88` | `[sp-168]` | `0x7c` | `0x0` | `0x0` |
| `bl` at 0x28018 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x4b8` | `0x0` |
| `bl` at 0x2806c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x4bf` | `0x0` |
| `bl` at 0x280cc | `(unknown callee) 0x264ec` | `[sp-20]` | `?` | `[0x119a2c]:1` | `sp-172` |
| `bl` at 0x28164 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x4dd` | `0x0` |
| `bl` at 0x281b0 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x4e0` | `0x0` |
| `bl` at 0x28234 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x4ec` | `0x0` |
| `bl` at 0x282b0 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x4f6` | `0x0` |
| `bl` at 0x28344 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x4f9` | `0x0` |
| `bl` at 0x283d4 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x502` | `0x0` |
| `bl` at 0x28468 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x506` | `0x0` |
| `bl` at 0x284f8 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x50f` | `0x0` |
| `bl` at 0x2858c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x513` | `0x0` |
| `bl` at 0x2859c | `(unknown callee) 0x2786c` | `[sp-20]` | `sp-132` | `?` | `sp-132` |
| `bl` at 0x285c0 | `(unknown callee) 0xc9f88` | `[sp-168]` | `0x7e` | `sp-132` | `0x0` |
| `bl` at 0x285f0 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x520` | `0x0` |
| `bl` at 0x2861c | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x28630 | `(unknown callee) 0xbc4e8` | `0x0` | `0x0` | `0x20` | `sp-124` |
| `bl` at 0x28680 | `(unknown callee) 0xc1540` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x28690 | `(unknown callee) 0xc1540` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x286a4 | `(unknown callee) 0xc15b4` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x286b4 | `(unknown callee) 0xc15b4` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x286d8 | `(unknown callee) 0xc9f88` | `[sp-168]` | `0x1a` | `sp-164` | `0x0` |
| `bl` at 0x28708 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x53f` | `0x0` |
| `bl` at 0x28758 | `(unknown callee) 0xc1540` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x28768 | `(unknown callee) 0xc1540` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x2877c | `(unknown callee) 0xc15b4` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x2878c | `(unknown callee) 0xc15b4` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x287b0 | `(unknown callee) 0xc9f88` | `[sp-168]` | `0x1b` | `sp-164` | `0x0` |
| `bl` at 0x287e0 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x54c` | `0x0` |
| `bl` at 0x28810 | `(unknown callee) 0xc9f88` | `[sp-168]` | `0x7d` | `0x0` | `0x0` |
| `bl` at 0x28840 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x556` | `0x0` |
| `bl` at 0x28878 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x559` | `0x0` |
| `bl` at 0x2888c | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x288a0 | `(unknown callee) 0xbc4e8` | `0x0` | `0x0` | `0x20` | `sp-124` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x27fa0 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x27ff4 | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x28034 | `[0x280f60]:2 == 0x0` | `[0x280f60]:2 != 0x0` |
| `beq` at 0x28044 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x2808c | `[0x119a2c]:1 == 0x0` | `[0x119a2c]:1 != 0x0` |
| `beq` at 0x280fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x28138 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x28184 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `blt` at 0x281c4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x281d0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x281dc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x28250 | `[0x119a44] == 0x0` | `[0x119a44] != 0x0` |
| `beq` at 0x28260 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x282f4 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x28374 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x28384 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x28418 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x28498 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x284a8 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x2853c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x285cc | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `bne` at 0x28608 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x28670 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x286e4 | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x28748 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x287bc | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x2881c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x2885c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x288b0 | `r0` | `?` |

## `SHUB_Scan_IDCODE`

- `SHUB_Scan_IDCODE`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2896c to 0x28d34.
  Established. The first explicit-link call has unknown target with candidates unknown, unknown,
  `[0x11584c]`, unknown (`mov` at 0x289d4). A later call to `0xc9f88` has candidates unknown, `0x3`,
  `0x0`, unknown (`bl` at 0x28a58). Five word stores have unresolved effective targets and unknown
  values (`str` at 0x28ad4, `str` at 0x28b14, `str` at 0x28b54, `str` at 0x28b94, `str` at 0x28bd4).
  An equality branch takes its edge for `? == 0x4363049`, falling through for inequality (`beq` at
  0x28bf0). A signed-greater-than branch later takes its edge when `ret(0x28d34) > 0x1` and falls
  through otherwise (`bgt` at 0x28ca0). Neither a particular IDCODE interpretation nor device
  revision is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2897c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x28984 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28988 | load | 4 | `abs 0x28cd8` | `-` |
| `ldr` at 0x2898c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28994 | load | 4 | `abs 0x28cdc` | `-` |
| `ldr` at 0x28998 | load | 4 | `abs 0x119a28` | `-` |
| `ldrb` at 0x289a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x289ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x289b4 | load | 4 | `abs 0x28cdc` | `-` |
| `ldr` at 0x289b8 | load | 4 | `abs 0x119a28` | `-` |
| `ldrb` at 0x289c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x289c4 | load | 4 | `abs 0x11584c` | `-` |
| `ldr` at 0x289f8 | load | 4 | `abs 0x28ce0` | `-` |
| `str` at 0x289fc | store | 4 | `sp -0x78` | `0xdb064` |
| `ldr` at 0x28a04 | load | 4 | `abs 0x28ce4` | `-` |
| `ldr` at 0x28a08 | load | 4 | `abs 0x28ce8` | `-` |
| `ldr` at 0x28a14 | load | 4 | `abs 0x28cec` | `-` |
| `str` at 0x28a18 | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x28a20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28a30 | load | 4 | `abs 0x28cf0` | `-` |
| `str` at 0x28a3c | store | 4 | `sp -0x78` | `0x20` |
| `ldr` at 0x28a40 | load | 4 | `abs 0x28cf4` | `-` |
| `str` at 0x28a44 | store | 4 | `sp -0x74` | `0xdb07c` |
| `ldr` at 0x28a68 | load | 4 | `abs 0x28cf8` | `-` |
| `str` at 0x28a6c | store | 4 | `sp -0x78` | `0xdb08c` |
| `ldr` at 0x28a70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28a74 | store | 4 | `sp -0x74` | `[sp-20]` |
| `ldr` at 0x28a7c | load | 4 | `abs 0x28ce4` | `-` |
| `ldr` at 0x28a80 | load | 4 | `abs 0x28cfc` | `-` |
| `ldr` at 0x28a8c | load | 4 | `abs 0x28cec` | `-` |
| `str` at 0x28a90 | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x28a98 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28a9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28aa0 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28aa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28aac | load | 4 | `abs 0x28d00` | `-` |
| `ldr` at 0x28ab8 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28abc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ac0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28ac8 | load | 4 | `abs 0x28d00` | `-` |
| `str` at 0x28ad4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x28ad8 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ae0 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28ae4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ae8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28aec | load | 4 | `abs 0x28d04` | `-` |
| `ldr` at 0x28af8 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28b08 | load | 4 | `abs 0x28d04` | `-` |
| `str` at 0x28b14 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x28b18 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b20 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28b2c | load | 4 | `abs 0x28d08` | `-` |
| `ldr` at 0x28b38 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28b48 | load | 4 | `abs 0x28d08` | `-` |
| `str` at 0x28b54 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x28b58 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b60 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28b78 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b80 | load | 4 | `(unknown)` | `-` |
| `str` at 0x28b94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x28b98 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ba0 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28ba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ba8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28bb8 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28bc0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x28bd4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x28bd8 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28bdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28be0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28be8 | load | 4 | `abs 0x28d0c` | `-` |
| `ldr` at 0x28bf4 | load | 4 | `abs 0x28d10` | `-` |
| `str` at 0x28bf8 | store | 4 | `sp -0x78` | `0xdb0bc` |
| `ldr` at 0x28c00 | load | 4 | `abs 0x28ce4` | `-` |
| `ldr` at 0x28c04 | load | 4 | `abs 0x28d14` | `-` |
| `ldr` at 0x28c10 | load | 4 | `abs 0x28d18` | `-` |
| `str` at 0x28c14 | store | 4 | `sp -0x78` | `0xdb0f8` |
| `ldr` at 0x28c18 | load | 4 | `abs 0x28cf0` | `-` |
| `ldr` at 0x28c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28c20 | load | 4 | `(unknown)` | `-` |
| `str` at 0x28c28 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x28c30 | load | 4 | `abs 0x28ce4` | `-` |
| `ldr` at 0x28c34 | load | 4 | `abs 0x28d1c` | `-` |
| `ldr` at 0x28c40 | load | 4 | `abs 0x28cec` | `-` |
| `str` at 0x28c44 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x28c4c | load | 4 | `abs 0x28d20` | `-` |
| `ldr` at 0x28c50 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28c5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28c68 | load | 4 | `abs 0x28d24` | `-` |
| `str` at 0x28c6c | store | 4 | `sp -0x78` | `0xdb128` |
| `ldr` at 0x28c70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28c74 | store | 4 | `sp -0x74` | `[sp-20]` |
| `str` at 0x28c78 | store | 4 | `sp -0x70` | `ret(0x28d34)` |
| `ldr` at 0x28c80 | load | 4 | `abs 0x28ce4` | `-` |
| `ldr` at 0x28c84 | load | 4 | `abs 0x28d28` | `-` |
| `ldr` at 0x28c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28ca4 | load | 4 | `abs 0x28d2c` | `-` |
| `str` at 0x28ca8 | store | 4 | `sp -0x78` | `0xdb138` |
| `ldr` at 0x28cb0 | load | 4 | `abs 0x28ce4` | `-` |
| `ldr` at 0x28cb4 | load | 4 | `abs 0x28d30` | `-` |
| `ldr` at 0x28cc0 | load | 4 | `abs 0x28cec` | `-` |
| `str` at 0x28cc4 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x28cc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28ccc | store | 4 | `sp -0x6c` | `[sp-24]` |
| `ldr` at 0x28cd0 | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0x28cd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28cd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28cd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x289d4 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x11584c]` | `?` |
| `bl` at 0x289e8 | `(unknown callee) 0xbc3c4` | `0x0` | `0x0` | `sp-104` | `sp-104` |
| `bl` at 0x28a10 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x573` | `0x0` |
| `bl` at 0x28a58 | `(unknown callee) 0xc9f88` | `?` | `0x3` | `0x0` | `?` |
| `bl` at 0x28a88 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x579` | `0x0` |
| `bl` at 0x28c0c | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x583` | `0x0` |
| `bl` at 0x28c3c | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x585` | `0x0` |
| `bl` at 0x28c60 | `(unknown callee) 0x28d34` | `[sp-20]` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0x28c8c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x589` | `0x0` |
| `bl` at 0x28c94 | `(unknown callee) 0x28d34` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x28cbc | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x58e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x289f4 | `ret(0xbc3c4) != 0x0` | `ret(0xbc3c4) == 0x0` |
| `beq` at 0x28a64 | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x28bf0 | `? == 0x4363049` | `? != 0x4363049` |
| `beq` at 0x28c58 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bgt` at 0x28ca0 | `ret(0x28d34) > 0x1 (signed, 32-bit)` | `ret(0x28d34) <= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28cd4 | `r0` | `?` |

## `SHUB_ScanReadMCILockStatus`

- `SHUB_ScanReadMCILockStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29a68 to 0x29ed4.
  Established. A call to `0xc9f88` has candidates `[sp-72]`, `0x7`, `sp-48`, `sp-64` (`bl` at
  0x29b00). A zero result takes the continuation edge, while nonzero falls through to a stack
  word store of `0xfffff890` (`beq` at 0x29b0c, `str` at 0x29b38). The second call to the same
  target has an unknown second candidate, with the other candidates `[sp-72]`, `sp-48`, `sp-64`
  (`bl` at 0x29b80). Later branches compare unknown values to `0x1ff`, following their equality edges
  (`beq` at 0x29c50, `beq` at 0x29d50). Lock-state bit meanings and merged return value remain
  unknown (`ldmdb` at 0x29e84).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x29a7c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x29a84 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x29a88 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29a90 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x29a94 | load | 4 | `abs 0x29e88` | `-` |
| `ldr` at 0x29a98 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x29aa4 | load | 4 | `abs 0x29e8c` | `-` |
| `str` at 0x29aa8 | store | 4 | `sp -0x5c` | `0xdb29c` |
| `ldr` at 0x29ab0 | load | 4 | `abs 0x29e90` | `-` |
| `ldr` at 0x29ab4 | load | 4 | `abs 0x29e94` | `-` |
| `ldrb` at 0x29ac8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29ae8 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x29aec | load | 4 | `abs 0x29e98` | `-` |
| `str` at 0x29af0 | store | 4 | `sp -0x58` | `0xdb2bc` |
| `ldr` at 0x29af4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x29b10 | load | 4 | `abs 0x29e9c` | `-` |
| `str` at 0x29b14 | store | 4 | `sp -0x5c` | `0xdb2c4` |
| `ldrb` at 0x29b18 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29b1c | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x29b24 | load | 4 | `abs 0x29e90` | `-` |
| `ldr` at 0x29b28 | load | 4 | `abs 0x29ea0` | `-` |
| `ldr` at 0x29b34 | load | 4 | `abs 0x29ea4` | `-` |
| `str` at 0x29b38 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x29b40 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29b68 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x29b6c | load | 4 | `abs 0x29e98` | `-` |
| `str` at 0x29b70 | store | 4 | `sp -0x58` | `0xdb2bc` |
| `ldr` at 0x29b74 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x29b90 | load | 4 | `abs 0x29e9c` | `-` |
| `str` at 0x29b94 | store | 4 | `sp -0x5c` | `0xdb2c4` |
| `ldrb` at 0x29b98 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29b9c | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x29ba4 | load | 4 | `abs 0x29e90` | `-` |
| `ldr` at 0x29ba8 | load | 4 | `abs 0x29ea8` | `-` |
| `ldr` at 0x29bb4 | load | 4 | `abs 0x29ea4` | `-` |
| `str` at 0x29bb8 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x29bc0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x29bd4 | load | 4 | `abs 0x29eac` | `-` |
| `ldrb` at 0x29bd8 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x29bf0 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x29bf4 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x29bf8 | load | 4 | `abs 0x29eb0` | `-` |
| `ldrb` at 0x29bfc | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x29c14 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x29c18 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x29c1c | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x29c20 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x29c28 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x29c2c | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x29c38 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x29c3c | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x29c48 | load | 4 | `abs 0x29eb4` | `-` |
| `ldrb` at 0x29c54 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x29c58 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x29c5c | load | 4 | `abs 0x29eb8` | `-` |
| `str` at 0x29c6c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x29c70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29c80 | load | 4 | `abs 0x29e88` | `-` |
| `ldr` at 0x29c84 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x29c90 | load | 4 | `abs 0x29ebc` | `-` |
| `str` at 0x29c94 | store | 4 | `sp -0x5c` | `0xdb32c` |
| `ldrb` at 0x29c98 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29c9c | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x29ca0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29ca4 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x29ca8 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x29cb0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x29cb8 | load | 4 | `abs 0x29e90` | `-` |
| `ldr` at 0x29cbc | load | 4 | `abs 0x29ec0` | `-` |
| `ldrh` at 0x29cc8 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x29cd0 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x29cd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29cdc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x29ce4 | load | 4 | `abs 0x29eac` | `-` |
| `ldrb` at 0x29ce8 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x29d00 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x29d04 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x29d08 | load | 4 | `abs 0x29eb0` | `-` |
| `ldrb` at 0x29d0c | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x29d24 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x29d28 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x29d2c | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x29d30 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x29d38 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x29d3c | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x29d48 | load | 4 | `abs 0x29eb4` | `-` |
| `ldrb` at 0x29d54 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x29d58 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x29d5c | load | 4 | `abs 0x29ec4` | `-` |
| `str` at 0x29d6c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x29d70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29d80 | load | 4 | `abs 0x29e88` | `-` |
| `ldr` at 0x29d84 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x29d90 | load | 4 | `abs 0x29ec8` | `-` |
| `str` at 0x29d94 | store | 4 | `sp -0x5c` | `0xdb384` |
| `ldrb` at 0x29d98 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x29d9c | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x29da0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29da4 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x29da8 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x29db0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x29db8 | load | 4 | `abs 0x29e90` | `-` |
| `ldr` at 0x29dbc | load | 4 | `abs 0x29ecc` | `-` |
| `ldrh` at 0x29dc8 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x29dd0 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x29dd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29ddc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x29de4 | load | 4 | `abs 0x29eac` | `-` |
| `ldrb` at 0x29de8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x29e08 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x29e0c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x29e10 | load | 4 | `abs 0x29eb0` | `-` |
| `ldrb` at 0x29e14 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x29e34 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x29e38 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x29e3c | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x29e40 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x29e48 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x29e4c | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x29e58 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x29e5c | load | 2 | `sp -0x42` | `-` |
| `ldrb` at 0x29e68 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x29e6c | load | 4 | `abs 0x29ed0` | `-` |
| `str` at 0x29e7c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x29e80 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x29e84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29e84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29e84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29abc | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x6a7` | `0x0` |
| `bl` at 0x29ac4 | `(unknown callee) 0x2ad48` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x29ad8 | `(unknown callee) 0x291b8` | `[sp-17]:1` | `sp-48` | `sp-48` | `[sp-17]:1` |
| `bl` at 0x29b00 | `(unknown callee) 0xc9f88` | `[sp-72]` | `0x7` | `sp-48` | `sp-64` |
| `bl` at 0x29b30 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x6b4` | `0x0` |
| `bl` at 0x29b50 | `(unknown callee) 0x295c0` | `[sp-17]:1` | `0x1` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x29b58 | `(unknown callee) 0x2ad48` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x29b80 | `(unknown callee) 0xc9f88` | `[sp-72]` | `?` | `sp-48` | `sp-64` |
| `bl` at 0x29bb0 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x6c7` | `0x0` |
| `bl` at 0x29bd0 | `(unknown callee) 0x295c0` | `[sp-17]:1` | `0x2` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x29c64 | `(unknown callee) 0x1dba4` | `0xdb2f0` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x29cc4 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x6db` | `0x0` |
| `bl` at 0x29d64 | `(unknown callee) 0x1dba4` | `0xdb348` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x29dc4 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x6e5` | `0x0` |
| `bl` at 0x29e74 | `(unknown callee) 0x1dba4` | `0xdb3a0` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x29aa0 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x29b0c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x29b8c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `beq` at 0x29c50 | `? == 0x1ff` | `? != 0x1ff` |
| `ble` at 0x29c78 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x29c8c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x29d50 | `? == 0x1ff` | `? != 0x1ff` |
| `ble` at 0x29d78 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x29d8c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x29e64 | `[sp-66]:2 == 0x1` | `[sp-66]:2 != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29e84 | `r0` | `?` |

## `SHUB_ScanSetBusratio`

- `SHUB_ScanSetBusratio`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28d68 to 0x29098.
  Established. A zero word at absolute `0x119a4c` takes the edge into the longer path, while nonzero
  falls through to a word store of `0x0` at `sp -0x28` (`ldr` at 0x28d88, `beq` at 0x28d90,
  `str` at 0x28d98). The longer path calls `0xb45d0` with candidates `sp-32`, entry r1, entry r2,
  `sp-32` (`bl` at 0x28da8). An unsigned comparison takes its edge for `[sp-34]:2 <= 0x8fc` and
  falls through for a larger value (`bls` at 0x28f98). The body later stores byte `([sp-35]:1 &
  0xff)` at an unresolved effective target (`strb` at 0x29048). The return reports `0x0` (`ldmdb` at
  0x29058). Frequency units and the programmed bus ratio remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28d78 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0x28d80 | store | 2 | `sp -0x22` | `0x0` |
| `ldr` at 0x28d84 | load | 4 | `abs 0x2905c` | `-` |
| `ldr` at 0x28d88 | load | 4 | `abs 0x119a4c` | `-` |
| `str` at 0x28d98 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x28db0 | store | 4 | `sp -0x1c` | `ret(0xb45d0)` |
| `ldr` at 0x28db4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x28dc0 | load | 4 | `abs 0x29060` | `-` |
| `str` at 0x28dc4 | store | 4 | `sp -0x38` | `0xdb178` |
| `ldr` at 0x28dcc | load | 4 | `abs 0x29064` | `-` |
| `ldr` at 0x28dd0 | load | 4 | `abs 0x29068` | `-` |
| `str` at 0x28de0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x28de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28df0 | load | 4 | `abs 0x2906c` | `-` |
| `ldr` at 0x28df4 | load | 4 | `abs 0x119a28` | `-` |
| `ldrb` at 0x28dfc | load | 1 | `(unknown)` | `-` |
| `str` at 0x28e0c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x28e18 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28e24 | load | 4 | `abs 0x2906c` | `-` |
| `ldr` at 0x28e28 | load | 4 | `abs 0x119a28` | `-` |
| `ldrb` at 0x28e30 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x28e44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28e4c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x28e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28e74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28e7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28e88 | load | 4 | `abs 0x29070` | `-` |
| `ldr` at 0x28e8c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28e98 | load | 4 | `abs 0x29074` | `-` |
| `str` at 0x28e9c | store | 4 | `sp -0x38` | `0xdb194` |
| `ldr` at 0x28ea0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28ea4 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x28ea8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28eac | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x28eb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28eb4 | load | 4 | `[sp-32] +0x28` | `-` |
| `str` at 0x28eb8 | store | 4 | `sp -0x2c` | `[[sp-32]+40]` |
| `ldr` at 0x28ec0 | load | 4 | `abs 0x29064` | `-` |
| `ldrh` at 0x28ed0 | load | 2 | `sp -0x22` | `-` |
| `ldrh` at 0x28edc | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x28ee0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ef4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28efc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x28f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28f20 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x28f28 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x28f2c | store | 2 | `sp -0x22` | `?` |
| `ldr` at 0x28f30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28f38 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrh` at 0x28f40 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x28f4c | load | 4 | `abs 0x29070` | `-` |
| `ldr` at 0x28f50 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28f5c | load | 4 | `abs 0x29078` | `-` |
| `str` at 0x28f60 | store | 4 | `sp -0x38` | `0xdb1b0` |
| `ldr` at 0x28f64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28f68 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x28f70 | load | 4 | `abs 0x29064` | `-` |
| `ldr` at 0x28f74 | load | 4 | `abs 0x2907c` | `-` |
| `str` at 0x28f84 | store | 4 | `sp -0x28` | `0x0` |
| `ldrh` at 0x28f8c | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x28f90 | load | 4 | `abs 0x29080` | `-` |
| `ldr` at 0x28f9c | load | 4 | `abs 0x29084` | `-` |
| `str` at 0x28fa0 | store | 4 | `sp -0x38` | `0xdb1e4` |
| `ldr` at 0x28fa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28fa8 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x28fb0 | load | 4 | `abs 0x29064` | `-` |
| `ldrh` at 0x28fc0 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x28fc8 | load | 4 | `abs 0x29088` | `-` |
| `strb` at 0x28fe0 | store | 1 | `sp -0x23` | `?` |
| `ldr` at 0x28fe4 | load | 4 | `abs 0x29070` | `-` |
| `ldr` at 0x28fe8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x28ff4 | load | 4 | `abs 0x2908c` | `-` |
| `str` at 0x28ff8 | store | 4 | `sp -0x38` | `0xdb224` |
| `ldr` at 0x28ffc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29000 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldrb` at 0x29004 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x29008 | store | 4 | `sp -0x30` | `[sp-35]:1` |
| `ldr` at 0x29010 | load | 4 | `abs 0x29064` | `-` |
| `ldr` at 0x29014 | load | 4 | `abs 0x29090` | `-` |
| `ldr` at 0x29020 | load | 4 | `abs 0x29094` | `-` |
| `ldr` at 0x29024 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x29044 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0x29048 | store | 1 | `(unknown)` | `([sp-35]:1 & 0xff)` |
| `str` at 0x29050 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x29054 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x29058 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29058 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x28da8 | `(unknown callee) 0xb45d0` | `sp-32` | `r1` | `r2` | `sp-32` |
| `bl` at 0x28dd8 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x5b5` | `0x0` |
| `bl` at 0x28ecc | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x5c0` | `0x0` |
| `bl` at 0x28f7c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x5ca` | `0x0` |
| `bl` at 0x28fbc | `(unknown callee) 0x1eed0` | `0x4` | `0xda5f4` | `0x5d0` | `0x0` |
| `bl` at 0x2901c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x5d3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28d90 | `[0x119a4c] == 0x0` | `[0x119a4c] != 0x0` |
| `bne` at 0x28dbc | `ret(0xb45d0) != 0x0` | `ret(0xb45d0) == 0x0` |
| `bne` at 0x28e04 | `? != 0xff` | `? == 0xff` |
| `ble` at 0x28e58 | `[sp-24] <= 0x1 (signed, 32-bit)` | `[sp-24] > 0x1 (signed, 32-bit)` |
| `bne` at 0x28e84 | `? != 0x3` | `? == 0x3` |
| `beq` at 0x28e94 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x28ed8 | `[sp-34]:2 == 0x0` | `[sp-34]:2 != 0x0` |
| `bge` at 0x28f04 | `[sp-34]:2 >= ? (signed, 32-bit)` | `[sp-34]:2 < ? (signed, 32-bit)` |
| `bne` at 0x28f48 | `[sp-34]:2 != 0x0` | `[sp-34]:2 == 0x0` |
| `beq` at 0x28f58 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bls` at 0x28f98 | `[sp-34]:2 <= 0x8fc (unsigned, 32-bit)` | `[sp-34]:2 > 0x8fc (unsigned, 32-bit)` |
| `beq` at 0x28ff0 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29058 | `r0` | `0x0` |
