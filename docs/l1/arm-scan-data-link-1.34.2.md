# ARM scan data and link configuration in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/scan-data-link-facts.txt) retain all tool sections.

## `JTAG_Cfg_display_f`

- `JTAG_Cfg_display_f`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7014 to 0xc72d0.
  Established. Calls to `0x20f14` include candidates `0x10a25c`, `[0x115844]`, unknown, `0x115844`,
  and later `0x10a26c`, `[0x27eba4]`, unknown, `0x27eba4` (`bl` at 0xc7064, `bl` at 0xc7074). The
  first unsigned comparison takes its edge for `[0x27eb98] <= 0x2` and falls through above that
  value (`bls` at 0xc7030). A later halfword load has unknown target (`ldrh` at 0xc71e4). The return
  reports `ret(0x20f14)` (`ldmdb` at 0xc7298). Output-field meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc7024 | load | 4 | `abs 0xc729c` | `-` |
| `ldr` at 0xc7028 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc7034 | load | 4 | `abs 0xc72a0` | `-` |
| `str` at 0xc7038 | store | 4 | `sp -0x20` | `0x10a230` |
| `ldr` at 0xc7040 | load | 4 | `abs 0xc72a4` | `-` |
| `ldr` at 0xc7044 | load | 4 | `abs 0xc72a8` | `-` |
| `ldr` at 0xc7050 | load | 4 | `abs 0xc72ac` | `-` |
| `ldr` at 0xc7058 | load | 4 | `abs 0xc72b0` | `-` |
| `ldr` at 0xc705c | load | 4 | `abs 0xc72b4` | `-` |
| `ldr` at 0xc7060 | load | 4 | `abs 0x115844` | `-` |
| `ldr` at 0xc7068 | load | 4 | `abs 0xc72b8` | `-` |
| `ldr` at 0xc706c | load | 4 | `abs 0xc72bc` | `-` |
| `ldr` at 0xc7070 | load | 4 | `abs 0x27eba4` | `-` |
| `str` at 0xc707c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xc7080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7090 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc7094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc70b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc70c4 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc70c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc70e0 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc70e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7104 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc7108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7128 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc712c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc7130 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc7134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7154 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc7158 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc715c | load | 4 | `abs 0xc72c4` | `-` |
| `ldr` at 0xc7160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7164 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc7168 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc7174 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc7178 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc717c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc719c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc71a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc71b0 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc71b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc71d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc71dc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc71e4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc71e8 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc71ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc720c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7214 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc721c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc7220 | load | 4 | `abs 0xc72c0` | `-` |
| `ldr` at 0xc7224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7244 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc724c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc7254 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc7258 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc725c | load | 4 | `abs 0xc72c8` | `-` |
| `ldr` at 0xc7260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7270 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc7278 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc7280 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc7288 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc7290 | load | 4 | `abs 0xc72cc` | `-` |
| `ldmdb` at 0xc7298 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7298 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7298 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc704c | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x1a9` | `0x0` |
| `bl` at 0xc7054 | `(unknown callee) 0x20f14` | `0x10a244` | `?` | `?` | `?` |
| `bl` at 0xc7064 | `(unknown callee) 0x20f14` | `0x10a25c` | `[0x115844]` | `?` | `0x115844` |
| `bl` at 0xc7074 | `(unknown callee) 0x20f14` | `0x10a26c` | `[0x27eba4]` | `?` | `0x27eba4` |
| `bl` at 0xc716c | `(unknown callee) 0x20f14` | `0x10a27c` | `?` | `?` | `?` |
| `bl` at 0xc726c | `(unknown callee) 0x20f14` | `0x10a2b8` | `?` | `?` | `?` |
| `bl` at 0xc7294 | `(unknown callee) 0x20f14` | `0x10a2ec` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc7030 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xc7088 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc70bc | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xc71a8 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7298 | `r0` | `ret(0x20f14)` |

## `JTAG_Cfg_display_stub`

- `JTAG_Cfg_display_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7ac4 to 0xc7ad4.
  Established. The prologue saves registers and the epilogue restores them (`push` at 0xc7ac8,
  `ldmdb` at 0xc7ad0). Entry r0 remains the reported return value (`ldmdb` at 0xc7ad0). No calls or
  conditional branches are reported. The display effect suggested by the symbol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0xc7ad0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7ad0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7ad0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7ad0 | `r0` | `r0` |

## `JTAG_LLP_Setup`

- `JTAG_LLP_Setup`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0e10 to 0xd0e3c.
  Established. A single explicit word store writes `0xd10c8` at absolute `0x115858` (`str` at
  0xd0e24). The stored code address is not called in this body. r0 receives `0x0` and the return
  reports that value (`mov` at 0xd0e28, `mov` at 0xd0e2c, `ldmdb` at 0xd0e30). No calls or
  conditional branches are reported. The consumer of the absolute slot is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd0e1c | load | 4 | `abs 0xd0e34` | `-` |
| `ldr` at 0xd0e20 | load | 4 | `abs 0xd0e38` | `-` |
| `str` at 0xd0e24 | store | 4 | `abs 0x115858` | `0xd10c8` |
| `ldmdb` at 0xd0e30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd0e30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd0e30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd0e30 | `r0` | `0x0` |

## `JTAG_LLP_SC_stub`

- `JTAG_LLP_SC_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7af4 to 0xc7b14.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0xc7b04). The routine selects
  `0x0` for r0 and returns it (`mov` at 0xc7b08, `mov` at 0xc7b0c, `ldmdb` at 0xc7b10). No calls or
  conditional branches are reported. The zero value’s external status meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7b04 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xc7b10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7b10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7b10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7b10 | `r0` | `0x0` |

## `JTAG_LLP_SC_f`

- `JTAG_LLP_SC_f`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd10c8 to 0xd1aec.
  Established. The first call targets `0xca63c` with candidates unknown, unknown, `0x4`, unknown
  (`bl` at 0xd1114). A later call to `0xcb4cc` carries `[sp-24]`, `0x0`, `[sp-32]`, unknown (`bl` at
  0xd1480); its signed nonnegative result takes the continuation edge, while a negative result falls
  through to the later cleanup path (`bge` at 0xd1494). Word stores at `[sp-44] +0x0` include
  three unknown values (`str` at 0xd1628, `str` at 0xd1944, `str` at 0xd1978). The link-level action
  and merged return remain unknown (`ldmdb` at 0xd1ac8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd10d8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd10e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd10e8 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xd10f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1100 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xd111c | store | 4 | `sp -0x18` | `ret(0xca63c)` |
| `ldr` at 0xd1120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd112c | load | 4 | `abs 0xd1acc` | `-` |
| `str` at 0xd1130 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xd1138 | load | 4 | `abs 0xd1ad0` | `-` |
| `ldr` at 0xd113c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xd1148 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1154 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1158 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd1164 | load | 4 | `abs 0xd1ad4` | `-` |
| `ldr` at 0xd116c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1174 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd117c | store | 4 | `sp -0x2c` | `[sp-20]+16` |
| `ldr` at 0xd1180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1184 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xd1190 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd1194 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1198 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xd11a8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd11ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd11b4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xd11b8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xd11c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xd11d0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xd11e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xd11f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd11fc | load | 4 | `abs 0xd1ad8` | `-` |
| `str` at 0xd1200 | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xd1208 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1210 | store | 4 | `sp -0x20` | `[sp-32]+9` |
| `ldr` at 0xd1218 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1224 | load | 4 | `abs 0xd1ad8` | `-` |
| `str` at 0xd1228 | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xd1230 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1238 | store | 4 | `sp -0x20` | `[sp-32]+11` |
| `ldr` at 0xd1240 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd124c | load | 4 | `abs 0xd1ad8` | `-` |
| `str` at 0xd1250 | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xd1258 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1260 | store | 4 | `sp -0x20` | `[sp-32]+18` |
| `ldr` at 0xd1268 | load | 4 | `abs 0xd1ad8` | `-` |
| `str` at 0xd126c | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xd1274 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd1280 | store | 4 | `sp -0x34` | `ret(0xca498)` |
| `ldr` at 0xd1284 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1290 | load | 4 | `abs 0xd1adc` | `-` |
| `ldr` at 0xd1294 | load | 4 | `abs 0xd1ae0` | `-` |
| `ldr` at 0xd1298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd12a0 | load | 4 | `abs 0xd1adc` | `-` |
| `str` at 0xd12a4 | store | 4 | `sp -0x60` | `0x2845a8` |
| `ldr` at 0xd12ac | load | 4 | `abs 0xd1ae4` | `-` |
| `ldr` at 0xd12bc | load | 4 | `abs 0xd1acc` | `-` |
| `str` at 0xd12c0 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xd12c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd12cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd12d4 | load | 4 | `ret(0xca498) +0x1c` | `-` |
| `ldr` at 0xd12dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xd12e0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd12e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd12e8 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd12ec | load | 2 | `[[sp-56]+8] +0x2` | `-` |
| `str` at 0xd12f0 | store | 4 | `sp -0x28` | `[[[sp-56]+8]+2]:2` |
| `ldr` at 0xd12f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1300 | store | 4 | `sp -0x44` | `ret(0x18920)` |
| `ldr` at 0xd1304 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1310 | load | 4 | `abs 0xd1ae8` | `-` |
| `str` at 0xd1314 | store | 4 | `sp -0x48` | `0xfffff7cc` |
| `str` at 0xd1320 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd1324 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1328 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1338 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd133c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1344 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1348 | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0xd134c | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xd1354 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xd1358 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xd135c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd1364 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xd136c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1370 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd1374 | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `ldr` at 0xd138c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1390 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd1394 | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xd13a4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xd13ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd13b0 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd13b4 | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xd13c4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xd13c8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xd13d0 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xd13d4 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xd13d8 | store | 4 | `sp -0x28` | `[sp-80]` |
| `ldr` at 0xd13dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd13f0 | store | 4 | `sp -0x40` | `ret(0x18920)` |
| `ldr` at 0xd13f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1400 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1408 | load | 4 | `abs 0xd1ae8` | `-` |
| `str` at 0xd140c | store | 4 | `sp -0x48` | `0xfffff7cc` |
| `str` at 0xd1418 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd141c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1420 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1430 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1438 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1440 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1444 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd144c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xd1454 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd1458 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd145c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd1464 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xd146c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1474 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd147c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1488 | store | 4 | `sp -0x1c` | `ret(0xcb4cc)` |
| `ldr` at 0xd148c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd149c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd14a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd14ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd14b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd14b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd14c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd14d0 | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd14d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd14e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd14f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd14fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1504 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1514 | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd1518 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd152c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1530 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1540 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd1544 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd1550 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1568 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1578 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1588 | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd158c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd159c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd15a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd15ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd15bc | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd15c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd15d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd15d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd15e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd15ec | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xd15f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd1600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1604 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1608 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1610 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1614 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1618 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xd1628 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xd1630 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd1638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd163c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1640 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1648 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1658 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd166c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1684 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1690 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd16a0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd16a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd16ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd16c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd16c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd16d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd16e0 | store | 4 | `sp -0x54` | `0x3` |
| `str` at 0xd16ec | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xd16f0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xd16f4 | store | 4 | `sp -0x30` | `[sp-84]` |
| `ldr` at 0xd16fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd170c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1714 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1724 | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd1728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1748 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd1758 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd1760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1774 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd1784 | store | 4 | `sp -0x58` | `0x3` |
| `str` at 0xd1790 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xd1794 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xd1798 | store | 4 | `sp -0x30` | `[sp-88]` |
| `ldr` at 0xd17a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd17a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd17b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd17b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd17c8 | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd17cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd17dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd17ec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd17f8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd1800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1804 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1818 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1824 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd182c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd183c | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd1840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1854 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1858 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1868 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd186c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd1878 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1888 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd1894 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd189c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd18a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd18b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd18b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd18c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd18c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd18d8 | store | 4 | `sp -0x1c` | `ret(0xcb8f4)` |
| `ldr` at 0xd18dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd18f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd18f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1900 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1908 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xd1910 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd191c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1920 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1924 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd192c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1930 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1934 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xd1944 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xd194c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd1954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1958 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd195c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1964 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1968 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd196c | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xd1978 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xd1980 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd1984 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1988 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd198c | load | 2 | `[[sp-56]+8] +0x2` | `-` |
| `str` at 0xd1990 | store | 4 | `sp -0x28` | `[[[sp-56]+8]+2]:2` |
| `str` at 0xd1998 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd199c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd19a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd19b0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd19b4 | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0xd19b8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd19c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd19c4 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xd19cc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xd19d0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xd19d4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd19dc | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xd19e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd19e8 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd19ec | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `ldr` at 0xd1a04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1a08 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd1a0c | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xd1a1c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xd1a24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1a28 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xd1a2c | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xd1a3c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xd1a40 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xd1a48 | store | 4 | `sp -0x5c` | `[sp-92]+1` |
| `ldr` at 0xd1a4c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xd1a50 | store | 4 | `sp -0x28` | `[sp-92]` |
| `str` at 0xd1a58 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd1a5c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1a60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1a70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd1a74 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1a7c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xd1a84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd1a8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd1a98 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1a9c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd1aa4 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xd1aac | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1ab4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1abc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd1ac0 | store | 4 | `sp -0x48` | `[sp-28]` |
| `ldr` at 0xd1ac4 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xd1ac8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1ac8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1ac8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1114 | `(unknown callee) 0xca63c` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xd114c | `(unknown callee) 0xc960c` | `[sp-24]` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xd1170 | `(unknown callee) 0x20f14` | `0x10cfd0` | `ret(0xc960c)` | `[sp-24]` | `?` |
| `bl` at 0xd1278 | `(unknown callee) 0xca498` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xd129c | `(unknown callee) 0x18c74` | `0x2845a8` | `0x10cffc` | `[sp-24]` | `ret(0xca498)` |
| `bl` at 0xd12b8 | `(unknown callee) 0x1eed0` | `0x3` | `0x10d02c` | `0xc0` | `0x0` |
| `bl` at 0xd12f8 | `(unknown callee) 0x18920` | `[[[sp-56]+8]+2]:2` | `ret(0xca498)` | `?` | `[[[sp-56]+8]+2]:2` |
| `bl` at 0xd13e8 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xd1404 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `ret(0x18920)` |
| `bl` at 0xd1470 | `(unknown callee) 0xcf520` | `[sp-56]` | `?` | `?` | `[sp-40]` |
| `bl` at 0xd1480 | `(unknown callee) 0xcb4cc` | `[sp-24]` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0xd14b4 | `(unknown callee) 0xd1aec` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-36]` |
| `bl` at 0xd14c8 | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xd14f8 | `(unknown callee) 0xcf520` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xd150c | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xd1538 | `(unknown callee) 0xcab20` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xd156c | `(unknown callee) 0xd1aec` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `?` |
| `bl` at 0xd1580 | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xd15a0 | `(unknown callee) 0xcf520` | `[sp-56]` | `?` | `?` | `ret(0xcb8f4)` |
| `bl` at 0xd15b4 | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xd15e0 | `(unknown callee) 0xcab20` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xd160c | `(unknown callee) 0xd1f44` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-48]` |
| `bl` at 0xd1644 | `(unknown callee) 0xd1aec` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-48]` |
| `bl` at 0xd168c | `(unknown callee) 0xcab20` | `[sp-24]` | `[sp-32]` | `0x0` | `sp-48` |
| `bl` at 0xd16b4 | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x0` | `sp-48` |
| `bl` at 0xd16cc | `(unknown callee) 0xcab20` | `[sp-24]` | `[sp-32]` | `0x2` | `sp-48` |
| `bl` at 0xd1708 | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x2` | `sp-48` |
| `bl` at 0xd171c | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xd176c | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x0` | `sp-48` |
| `bl` at 0xd17ac | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x2` | `sp-48` |
| `bl` at 0xd17c0 | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xd17f0 | `(unknown callee) 0xcf520` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xd180c | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x6` | `sp-48` |
| `bl` at 0xd1820 | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x7` | `sp-48` |
| `bl` at 0xd1834 | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xd1860 | `(unknown callee) 0xcab20` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xd188c | `(unknown callee) 0xcf520` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xd18a8 | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x6` | `sp-48` |
| `bl` at 0xd18bc | `(unknown callee) 0xcb020` | `[sp-24]` | `[sp-32]` | `0x7` | `sp-48` |
| `bl` at 0xd18d0 | `(unknown callee) 0xcb8f4` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xd18fc | `(unknown callee) 0xcab20` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xd1928 | `(unknown callee) 0xd1f44` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-48]` |
| `bl` at 0xd1960 | `(unknown callee) 0xd1f44` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `?` |
| `bl` at 0xd1ab0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1ab8 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd1128 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xd1144 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xd11c0 | `[sp-76] == 0x4` | `[sp-76] != 0x4` |
| `bgt` at 0xd11cc | `[sp-76] > 0x4 (signed, 32-bit)` | `[sp-76] <= 0x4 (signed, 32-bit)` |
| `beq` at 0xd11d8 | `[sp-76] == 0x2` | `[sp-76] != 0x2` |
| `beq` at 0xd11e8 | `[sp-76] == 0x6` | `[sp-76] != 0x6` |
| `bls` at 0xd11f8 | `[sp-32] <= 0x1 (unsigned, 32-bit)` | `[sp-32] > 0x1 (unsigned, 32-bit)` |
| `bls` at 0xd1220 | `[sp-32] <= 0x1 (unsigned, 32-bit)` | `[sp-32] > 0x1 (unsigned, 32-bit)` |
| `bls` at 0xd1248 | `[sp-32] <= 0x7 (unsigned, 32-bit)` | `[sp-32] > 0x7 (unsigned, 32-bit)` |
| `bne` at 0xd128c | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `bne` at 0xd130c | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `blo` at 0xd1330 | `C == 0` | `not (C == 0)` |
| `bne` at 0xd1388 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd13fc | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `blo` at 0xd1428 | `C == 0` | `not (C == 0)` |
| `bge` at 0xd1494 | `ret(0xcb4cc) >= 0x0 (signed, 32-bit)` | `ret(0xcb4cc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xd14a4 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bge` at 0xd14dc | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xd14f0 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xd1520 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xd154c | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bls` at 0xd1558 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bge` at 0xd1594 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bge` at 0xd15c8 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xd15fc | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bne` at 0xd1654 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd1664 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xd1678 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd16d8 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bge` at 0xd1730 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xd1744 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd177c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bge` at 0xd17d4 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xd17e8 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xd1848 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bne` at 0xd1874 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bls` at 0xd1880 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bge` at 0xd18e4 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bne` at 0xd1918 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `blo` at 0xd19a8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xd1a00 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xd1a68 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1ac8 | `r0` | `?` |

## `tdrdata_create`

- `tdrdata_create`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce934 to 0xce9c4.
  Established. The opening call to `0x18920` uses candidates `0x14`, entry r1, entry r2 and entry r1
  (`bl` at 0xce958). A nonzero result takes the continuation and zero falls through to a stack
  word store of `0x0` (`bne` at 0xce96c, `str` at 0xce974). The next call targets `0xce9c4` with
  candidates `ret(0x18920)`, `[sp-20]`, `[sp-22]:2`, `[sp-28]` (`bl` at 0xce990). Entry r1 is saved
  as halfword `(r1 & 0xffff)` at `sp -0x16` (`strh` at 0xce950). The return is unknown after the
  join (`ldmdb` at 0xce9c0). Allocation semantics of the numeric callee are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce944 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce94c | store | 4 | `sp -0x1c` | `r2` |
| `strh` at 0xce950 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `str` at 0xce960 | store | 4 | `sp -0x20` | `ret(0x18920)` |
| `ldr` at 0xce964 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce974 | store | 4 | `sp -0x24` | `0x0` |
| `ldrh` at 0xce97c | load | 2 | `sp -0x16` | `-` |
| `ldr` at 0xce980 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce98c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce9a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce9ac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xce9b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xce9b8 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xce9bc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xce9c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce9c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce9c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce958 | `(unknown callee) 0x18920` | `0x14` | `r1` | `r2` | `r1` |
| `bl` at 0xce990 | `(unknown callee) 0xce9c4` | `ret(0x18920)` | `[sp-20]` | `[sp-22]:2` | `[sp-28]` |
| `bl` at 0xce9a4 | `(unknown callee) 0x18be4` | `[sp-32]` | `?` | `?` | `ret(0xce9c4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce96c | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xce99c | `ret(0xce9c4) != 0x0` | `ret(0xce9c4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce9c0 | `r0` | `?` |

## `tdrdata_init`

- `tdrdata_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce9c4 to 0xceb64.
  Established. The first test takes its bypass for zero entry r1 and falls through otherwise (`beq`
  at 0xce9ec). Halfword stores at `[sp-24] +0x4` and `[sp-24] +0x6` write `([sp-30]:2 & 0xffff)` and
  `0x0`, respectively (`strh` at 0xcea30, `strh` at 0xcea3c). Separate word stores put
  `ret(0x18920)` at `[sp-24] +0x10` and `[sp-24] +0xc` (`str` at 0xceaa0, `str` at 0xceaf0). The
  last call targets `0xcf520` with first candidate `[sp-24]` and the other three unknown (`bl` at
  0xceb50). Record semantics and the merged return remain unknown (`ldmdb` at 0xceb60).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce9d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xce9d8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xce9dc | store | 4 | `sp -0x24` | `r3` |
| `strh` at 0xce9e0 | store | 2 | `sp -0x1e` | `(r2 & 0xffff)` |
| `ldr` at 0xce9e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce9f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xce9f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce9fc | store | 4 | `r0 +0x0` | `ret(0x18788)` |
| `ldr` at 0xcea00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcea04 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xcea14 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcea1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcea24 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xcea28 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xcea2c | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xcea30 | store | 2 | `[sp-24] +0x4` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xcea34 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xcea3c | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xcea40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcea44 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcea48 | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0xcea4c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xcea50 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xcea70 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xcea74 | load | 2 | `[sp-36] +0x0` | `-` |
| `str` at 0xcea8c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcea90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcea94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xceaa0 | store | 4 | `[sp-24] +0x10` | `ret(0x18920)` |
| `ldr` at 0xceaa4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xceab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceab4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xceac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceac4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xcead0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcead8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceadc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xceae0 | load | 2 | `[sp-36] +0x2` | `-` |
| `str` at 0xceaf0 | store | 4 | `[sp-24] +0xc` | `ret(0x18920)` |
| `ldr` at 0xceaf4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xceb00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceb04 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xceb10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceb14 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xceb1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceb20 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xceb2c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xceb34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceb38 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xceb3c | load | 2 | `[sp-36] +0x2` | `-` |
| `ldr` at 0xceb40 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xceb4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xceb58 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xceb5c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xceb60 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xceb60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xceb60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xceb60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce9f8 | `(unknown callee) 0x18788` | `r1` | `r1` | `r2` | `r1` |
| `bl` at 0xcea98 | `(unknown callee) 0x18920` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xceac8 | `(unknown callee) 0x18be4` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xceae8 | `(unknown callee) 0x18920` | `[[sp-36]+2]:2` | `?` | `?` | `[[sp-36]+2]:2` |
| `bl` at 0xceb18 | `(unknown callee) 0x18be4` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xceb24 | `(unknown callee) 0x18be4` | `[[sp-24]+16]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xceb48 | `(unknown callee) 0x18c20` | `[[sp-24]+12]` | `0x0` | `[[sp-36]+2]:2` | `[sp-24]` |
| `bl` at 0xceb50 | `(unknown callee) 0xcf520` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xce9ec | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xcea0c | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0xceaac | `[[sp-24]+16] != 0x0` | `[[sp-24]+16] == 0x0` |
| `beq` at 0xceabc | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |
| `bne` at 0xceafc | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `beq` at 0xceb0c | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xceb60 | `r0` | `?` |

## `tdrdata_dump`

- `tdrdata_dump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xceb64 to 0xceea4.
  Established. A nonzero entry r2 takes the edge into the longer body, while zero falls through
  to the return (`bne` at 0xceb8c). With entry r0 zero, the fall-through path reads absolute
  `0x116924` and saves `[0x116924]` at `sp -0x14` (`bne` at 0xceb9c, `ldr` at 0xceba4, `str` at
  0xceba8). A later call to `0xce270` has candidates `[sp-20]`, `0x10b634`, `[[sp-28]+8]`, `[sp-28]`
  (`bl` at 0xcec74). The calls also include `0xceea4` with `[sp-20]`, `[sp-28]`, `0x1`, unknown
  (`bl` at 0xcee6c). A hardware transfer is not established by these calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xceb74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xceb78 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xceb7c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xceb80 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xceb84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xceb94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceba0 | load | 4 | `abs 0xcee74` | `-` |
| `ldr` at 0xceba4 | load | 4 | `abs 0x116924` | `-` |
| `str` at 0xceba8 | store | 4 | `sp -0x14` | `[0x116924]` |
| `ldr` at 0xcebac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcebb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcebbc | load | 4 | `abs 0xcee78` | `-` |
| `ldr` at 0xcebc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcebcc | load | 4 | `abs 0xcee7c` | `-` |
| `ldr` at 0xcebd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcebd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcebdc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcebe0 | store | 4 | `sp -0x34` | `[[sp-28]]` |
| `ldr` at 0xcebe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcebe8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xcebf4 | load | 4 | `abs 0xcee80` | `-` |
| `str` at 0xcebf8 | store | 4 | `sp -0x34` | `0x10b5f0` |
| `ldr` at 0xcebfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcec00 | load | 4 | `abs 0xcee84` | `-` |
| `ldr` at 0xcec04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcec0c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xcec10 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcec14 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xcec18 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xcec1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcec20 | load | 4 | `abs 0xcee88` | `-` |
| `ldr` at 0xcec28 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xcec2c | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xcec30 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xcec34 | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xcec38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcec3c | load | 4 | `abs 0xcee8c` | `-` |
| `ldr` at 0xcec44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcec54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcec58 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcec64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcec68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcec6c | load | 4 | `abs 0xcee90` | `-` |
| `ldr` at 0xcec70 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xcec78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcec88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcec8c | load | 4 | `abs 0xcee94` | `-` |
| `ldr` at 0xcec94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcec98 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xcec9c | load | 2 | `[[sp-28]+8] +0x2` | `-` |
| `str` at 0xcecac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcecb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcecb4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xcecb8 | load | 2 | `[[sp-28]+8] +0x2` | `-` |
| `str` at 0xcecc8 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xcecd0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcecd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcecd8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcecec | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcecf0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xced00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xced04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xced0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xced14 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0xced1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xced20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xced24 | load | 4 | `abs 0xcee98` | `-` |
| `ldr` at 0xced30 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xced38 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xced40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xced44 | load | 4 | `abs 0xcee9c` | `-` |
| `ldr` at 0xced4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xced54 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xced5c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xced6c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xced70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xced74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xced84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xced88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xced90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xced98 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0xceda0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xceda4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceda8 | load | 4 | `abs 0xcee98` | `-` |
| `ldr` at 0xcedb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcedbc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xcedc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcedc8 | load | 4 | `abs 0xcee9c` | `-` |
| `ldr` at 0xcedd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcedd4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xcedd8 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `ldr` at 0xcedf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcedf4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xcedf8 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `str` at 0xcee08 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcee10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcee14 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xcee18 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `str` at 0xcee28 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcee2c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcee34 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xcee38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcee3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcee40 | load | 4 | `abs 0xceea0` | `-` |
| `ldr` at 0xcee44 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcee48 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xcee50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcee60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcee64 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xcee70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcee70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcee70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcebc0 | `(unknown callee) 0x21010` | `?` | `0x10b5cc` | `r2` | `r1` |
| `bl` at 0xcebd4 | `(unknown callee) 0x21010` | `?` | `0x10b5ec` | `r1` | `r1` |
| `bl` at 0xcec08 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b5f8` | `?` | `?` |
| `bl` at 0xcec24 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b604` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xcec40 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b61c` | `[[sp-28]+6]:2` | `[[sp-28]+6]:2` |
| `bl` at 0xcec74 | `(unknown callee) 0xce270` | `[sp-20]` | `0x10b634` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0xcec90 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b650` | `?` | `?` |
| `bl` at 0xced2c | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b66c` | `?` | `?` |
| `bl` at 0xced48 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b674` | `?` | `?` |
| `bl` at 0xcedb0 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b66c` | `?` | `?` |
| `bl` at 0xcedcc | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b674` | `?` | `[sp-48]` |
| `bl` at 0xcee4c | `(unknown callee) 0xd2e88` | `[sp-20]` | `0x10b678` | `[sp-56]` | `[[sp-28]+16]` |
| `bl` at 0xcee6c | `(unknown callee) 0xceea4` | `[sp-20]` | `[sp-28]` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xceb8c | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xceb9c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xcebb4 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xcebf0 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `beq` at 0xcec50 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcec60 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xcec84 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xcece0 | `C == 0` | `not (C == 0)` |
| `bls` at 0xcecf8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xced64 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `blo` at 0xced7c | `C == 0` | `not (C == 0)` |
| `bne` at 0xcedec | `? != 0x0` | `? == 0x0` |
| `beq` at 0xcee5c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcee70 | `r0` | `?` |

## `tdrdata_dump_fields`

- `tdrdata_dump_fields`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xceea4 to 0xcf198.
  Established. Word stores clear absolute `0x27ecc0` and later write `[0x27ecc0]+1` there (`str` at
  0xcf000, `str` at 0xcf0d0). The corresponding unsigned comparisons take their edges for
  `[0x27ecc0] < ?` and fall through for `[0x27ecc0] >= ?`, with the other operand unknown (`blo` at
  0xcf060, `blo` at 0xcf09c). The call to `0xce614` uses `[[sp-24]+8]`, `[sp-32]`, `0x0`,
  `[[sp-24]+16]` (`bl` at 0xcf0f8). The later `0xcf198` call carries unknown, `[sp-36]`, `sp-116`,
  unknown (`bl` at 0xcf124). The absolute word’s device role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xceeb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xceeb8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xceebc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xceec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceecc | load | 4 | `abs 0xcf188` | `-` |
| `ldr` at 0xceed0 | load | 4 | `abs 0x116924` | `-` |
| `str` at 0xceed4 | store | 4 | `sp -0x14` | `[0x116924]` |
| `ldr` at 0xceed8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceedc | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xceeec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceef0 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xceef4 | load | 2 | `[r1+8] +0x0` | `-` |
| `ldr` at 0xcef0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcef10 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xcef14 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xcef24 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xcef28 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcef30 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xcef38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcef3c | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xcef40 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xcef50 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xcef54 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcef60 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xcef64 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcef70 | store | 4 | `sp -0x24` | `ret(0x18920)` |
| `ldr` at 0xcef74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcef84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcef88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcef8c | load | 4 | `abs 0xcf18c` | `-` |
| `ldr` at 0xcef90 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xcef9c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcefa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcefa4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xcefa8 | load | 2 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xcefac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcefbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcefc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcefd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcefd8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xcefdc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcefe4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xceff8 | load | 4 | `abs 0xcf190` | `-` |
| `str` at 0xcf000 | store | 4 | `abs 0x27ecc0` | `0x0` |
| `ldr` at 0xcf004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf008 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcf00c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf014 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrb` at 0xcf01c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf030 | load | 4 | `abs 0xcf190` | `-` |
| `ldr` at 0xcf034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf038 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcf03c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf044 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrb` at 0xcf04c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf058 | load | 4 | `abs 0x27ecc0` | `-` |
| `ldr` at 0xcf068 | load | 4 | `abs 0xcf190` | `-` |
| `ldr` at 0xcf06c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf070 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcf074 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf07c | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrb` at 0xcf084 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf094 | load | 4 | `abs 0x27ecc0` | `-` |
| `ldr` at 0xcf0a4 | load | 4 | `abs 0xcf190` | `-` |
| `ldr` at 0xcf0a8 | load | 4 | `abs 0x27ecc0` | `-` |
| `ldr` at 0xcf0b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcf0bc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcf0c0 | load | 4 | `abs 0xcf190` | `-` |
| `ldr` at 0xcf0c4 | load | 4 | `abs 0xcf190` | `-` |
| `ldr` at 0xcf0c8 | load | 4 | `abs 0x27ecc0` | `-` |
| `str` at 0xcf0d0 | store | 4 | `abs 0x27ecc0` | `[0x27ecc0]+1` |
| `ldr` at 0xcf0d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf0dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf0e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcf0e4 | store | 4 | `sp -0x7c` | `[sp-36]` |
| `ldr` at 0xcf0e8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcf0ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf0f4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xcf0fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf100 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcf104 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf10c | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrb` at 0xcf114 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcf120 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf128 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf12c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xcf130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf138 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldr` at 0xcf140 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf144 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xcf148 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcf150 | load | 1 | `([[sp-24]+12] + [sp-32]) +0x0` | `-` |
| `ldr` at 0xcf158 | load | 4 | `abs 0xcf194` | `-` |
| `ldr` at 0xcf15c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf16c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf174 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcf17c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xcf184 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf184 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf184 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcef68 | `(unknown callee) 0x18920` | `[sp-120]` | `r1` | `r2` | `?` |
| `bl` at 0xcef94 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10b68c` | `[[sp-24]]` | `[sp-24]` |
| `bl` at 0xcf0f8 | `(unknown callee) 0xce614` | `[[sp-24]+8]` | `[sp-32]` | `0x0` | `[[sp-24]+16]` |
| `bl` at 0xcf124 | `(unknown callee) 0xcf198` | `?` | `[sp-36]` | `sp-116` | `?` |
| `bl` at 0xcf168 | `(unknown callee) 0x20f14` | `0x10b6ac` | `?` | `[([[sp-24]+12] + [sp-32])]:1` | `sp-116` |
| `bl` at 0xcf180 | `(unknown callee) 0x18be4` | `[sp-36]` | `?` | `[[[sp-24]+8]+2]:2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xceec8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xceee4 | `[r1+8] != 0x0` | `[r1+8] == 0x0` |
| `bne` at 0xcef08 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcef7c | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `blo` at 0xcefb4 | `C == 0` | `not (C == 0)` |
| `beq` at 0xcefc4 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xcefd0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xceff0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcf02c | `? != 0x0` | `? == 0x0` |
| `blo` at 0xcf060 | `[0x27ecc0] < ? (unsigned, 32-bit)` | `[0x27ecc0] >= ? (unsigned, 32-bit)` |
| `blo` at 0xcf09c | `[0x27ecc0] < ? (unsigned, 32-bit)` | `[0x27ecc0] >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf184 | `r0` | `?` |

## `tdrdata_create_field_str`

- `tdrdata_create_field_str`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf198 to 0xcf2ac.
  Established. The output stores include an unknown byte and a zero byte, each at an unknown target
  (`strb` at 0xcf248, `strb` at 0xcf2a0). Stack word `sp -0x2c` receives `0x1c` on the zero
  fall-through of the test on `[sp-44]`, while nonzero takes the decrement path (`str` at 0xcf268,
  `bne` at 0xcf260, `str` at 0xcf284). Entry r0 remains the return value (`ldmdb` at 0xcf2a4). No
  calls are reported. The resulting text’s encoding is unknown in these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf1a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf1ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf1b0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcf1b4 | load | 4 | `abs 0xcf2a8` | `-` |
| `str` at 0xcf1b8 | store | 4 | `sp -0x34` | `0x10b6cc` |
| `str` at 0xcf1c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcf1c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf1d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcf1e4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xcf1e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcf1f8 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xcf200 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcf204 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf208 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcf218 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf21c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcf224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf22c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf230 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcf23c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xcf244 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xcf248 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xcf24c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcf254 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xcf258 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcf268 | store | 4 | `sp -0x2c` | `0x1c` |
| `ldr` at 0xcf26c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcf274 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xcf27c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcf284 | store | 4 | `sp -0x2c` | `[sp-44]-4` |
| `ldr` at 0xcf288 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf290 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcf298 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcf2a0 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0xcf2a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf2a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf2a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xcf210 | `C == 0` | `not (C == 0)` |
| `bne` at 0xcf260 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf2a4 | `r0` | `r0` |

## `tdrdata_set_data`

- `tdrdata_set_data`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf2ac to 0xcf380.
  Established. Entry r0 and r1 are saved as words at `sp -0x14` and `sp -0x18` (`str` at 0xcf2bc,
  `str` at 0xcf2c0). The later word store has unknown value and effective target (`str` at 0xcf368).
  Two loop branches take their edges for `C == 0` and fall through otherwise, with comparison
  operands unknown (`blo` at 0xcf30c, `blo` at 0xcf338). The return retains entry r0 (`ldmdb` at
  0xcf37c). No calls are reported, and the copied words’ physical destination is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf2bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf2c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf2c8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcf2cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf2d0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf2d4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf2ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf2f0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf2f4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf318 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf31c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf344 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf34c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf35c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf364 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcf368 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcf36c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcf374 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0xcf37c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf37c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf37c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcf2e8 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xcf30c | `C == 0` | `not (C == 0)` |
| `blo` at 0xcf338 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf37c | `r0` | `r0` |

## `tdrdata_get_data`

- `tdrdata_get_data`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf380 to 0xcf520.
  Established. Zero entry r1 takes the alternate path and nonzero falls through (`beq` at 0xcf3a4).
  Each path includes a word store with unknown target and value (`str` at 0xcf450, `str` at
  0xcf508). Stack word `sp -0x20` is initialized to zero independently on both paths (`str` at
  0xcf3ac, `str` at 0xcf468). The joined r0 return is unknown (`ldmdb` at 0xcf51c). No calls are
  reported. The selector’s device meaning is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf390 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf394 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf398 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcf39c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcf3ac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcf3b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf3b4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf3b8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf3d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf3d4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf3d8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf3e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf3f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf3fc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf400 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf414 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf424 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf42c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf438 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf43c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf444 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf44c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcf450 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcf454 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf45c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xcf468 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcf46c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf470 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf474 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf48c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf490 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf494 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf4a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf4b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf4b8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf4bc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf4d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf4e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf4e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf4f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf4f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcf4fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf504 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcf508 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcf50c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcf514 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xcf51c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf51c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf51c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcf3a4 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xcf3cc | `? != 0x0` | `? == 0x0` |
| `blo` at 0xcf3f0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xcf41c | `C == 0` | `not (C == 0)` |
| `bne` at 0xcf488 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xcf4ac | `C == 0` | `not (C == 0)` |
| `blo` at 0xcf4d8 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf51c | `r0` | `?` |

## `tdrdata_reset`

- `tdrdata_reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf520 to 0xcf6fc.
  Established. A word load at entry `r0 +0x8` feeds a zero test that takes the alternate path on
  equality and falls through otherwise (`ldr` at 0xcf538, `beq` at 0xcf540). Stores include an
  unknown word, a zero word and a zero byte, all at unknown targets (`str` at 0xcf5f0, `str` at
  0xcf698, `strb` at 0xcf6e4). The final loop branch takes its edge for `C == 0` and falls through
  otherwise, retaining unknown operands (`blo` at 0xcf6c8). No calls are reported. Neither a
  hardware reset nor the merged return value is established (`ldmdb` at 0xcf6f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf530 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcf534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf538 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xcf548 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcf54c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf550 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf554 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf56c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf570 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf574 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf598 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf59c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf5b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf5c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf5c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf5cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf5d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf5d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf5dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf5e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf5ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xcf5f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcf5f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcf5fc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xcf608 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcf60c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf610 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf614 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf62c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf630 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf634 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf644 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf658 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf65c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf670 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf68c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcf698 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcf69c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcf6a4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xcf6b0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcf6b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf6b8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcf6bc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf6c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf6d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf6d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcf6d8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xcf6e4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xcf6e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcf6f0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xcf6f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf6f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf6f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcf540 | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `bne` at 0xcf568 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xcf58c | `C == 0` | `not (C == 0)` |
| `blo` at 0xcf5b8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xcf628 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xcf64c | `C == 0` | `not (C == 0)` |
| `blo` at 0xcf678 | `C == 0` | `not (C == 0)` |
| `blo` at 0xcf6c8 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf6f8 | `r0` | `?` |

## `tdrdata_reset_field`

- `tdrdata_reset_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf6fc to 0xcf7a4.
  Established. The only call targets `0xc1ab4` with candidates unknown, `[[r0+8]+8]`, unknown,
  `[r0+16]` (`bl` at 0xcf784). After it, a byte store clears `([[sp-24]+12] + [sp-28]) +0x0` (`strb`
  at 0xcf79c). The return reports `ret(0xc1ab4)` in r0 (`ldmdb` at 0xcf7a0). No conditional branches
  are reported. The reset field’s protocol interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf70c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcf710 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xcf714 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf718 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xcf71c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf724 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xcf72c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf730 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf734 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xcf738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf73c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xcf740 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf748 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xcf750 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcf754 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf75c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xcf760 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcf768 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrb` at 0xcf770 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcf774 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcf778 | load | 4 | `[r0+8] +0x8` | `-` |
| `ldr` at 0xcf780 | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xcf788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf78c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xcf790 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcf79c | store | 1 | `([[sp-24]+12] + [sp-28]) +0x0` | `0x0` |
| `ldmdb` at 0xcf7a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcf7a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf7a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf7a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf784 | `(unknown callee) 0xc1ab4` | `?` | `[[r0+8]+8]` | `?` | `[r0+16]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf7a0 | `r0` | `ret(0xc1ab4)` |

## `tdrdata_override_field`

- `tdrdata_override_field`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf7a4 to 0xcf814.
  Established. A byte store at `([r0+12] + r1) +0x0` has unknown value (`strb` at 0xcf7e8). A stack
  word carries `[[sp-20]+16]` at `sp -0x20` (`str` at 0xcf7f8). The only call targets `0xce410` with
  candidates `[[sp-20]+8]`, `[sp-24]`, `[sp-28]`, `0x0` (`bl` at 0xcf80c); its result reaches the
  return (`ldmdb` at 0xcf810). No conditional branches are reported, and the override’s external
  effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf7b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcf7b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcf7bc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcf7c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf7c4 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xcf7c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf7d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf7d4 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xcf7d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcf7e0 | load | 1 | `([r0+12] + r1) +0x0` | `-` |
| `strb` at 0xcf7e8 | store | 1 | `([r0+12] + r1) +0x0` | `?` |
| `ldr` at 0xcf7ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf7f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcf7f4 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xcf7f8 | store | 4 | `sp -0x20` | `[[sp-20]+16]` |
| `ldr` at 0xcf7fc | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xcf800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcf804 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xcf810 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcf810 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcf810 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcf80c | `(unknown callee) 0xce410` | `[[sp-20]+8]` | `[sp-24]` | `[sp-28]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcf810 | `r0` | `ret(0xce410)` |
