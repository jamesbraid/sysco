# ARM scan selection and configuration in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/scan-select-config-facts.txt) retain all tool sections.

## `jmstr_sel_sic`

- `jmstr_sel_sic`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbce44 to 0xbecec.
  Established. The first call targets `0x18920` with candidates `0x60`, entry r1, entry r2 and entry
  r3 (`bl` at 0xbce68); its nonzero result takes the continuation edge, while zero falls through
  to `0x18c74` (`bne` at 0xbce7c, `bl` at 0xbce88). Word stores include `0x4` at `[sp-64] +0x0`
  and `0x0` at `[sp-68] +0x0` (`str` at 0xbd090, `str` at 0xbd1b8). The merged r0 return is unknown
  (`ldmdb` at 0xbece8). Selection semantics and physical actions of the numeric callees remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbce54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbce58 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbce5c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbce60 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbce70 | store | 4 | `sp -0x40` | `ret(0x18920)` |
| `ldr` at 0xbce74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbce80 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbce84 | load | 4 | `abs 0xbde1c` | `-` |
| `str` at 0xbce90 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbcea4 | store | 4 | `sp -0x44` | `ret(0x18920)` |
| `ldr` at 0xbcea8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbceb4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbceb8 | load | 4 | `abs 0xbde1c` | `-` |
| `ldr` at 0xbcec0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbcecc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbced4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbcee4 | load | 4 | `abs 0xbde20` | `-` |
| `str` at 0xbcee8 | store | 4 | `sp -0x28` | `0xfff00000` |
| `str` at 0xbcef0 | store | 4 | `sp -0x2c` | `0x80000000` |
| `ldr` at 0xbcef8 | load | 4 | `abs 0xbde20` | `-` |
| `str` at 0xbcefc | store | 4 | `sp -0x28` | `0xfff00000` |
| `ldr` at 0xbcf00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbcf04 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0xbcf08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf0c | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbcf18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf20 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbcf2c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbcf30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf34 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbcf40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf48 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbcf54 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbcf58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf5c | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbcf68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf70 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbcf7c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbcf80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcf90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbcfa4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbcfa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcfb8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbcfcc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbcfd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbcfdc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbcfe4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xbcfec | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbcff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd004 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd008 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbd01c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd024 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd030 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd03c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd040 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbd054 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd05c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd068 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbd084 | store | 4 | `sp -0x24` | `0x5` |
| `ldr` at 0xbd088 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd090 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `str` at 0xbd09c | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xbd0a0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd0a8 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `ldr` at 0xbd0ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd0b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd0b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd0b8 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbd0c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd0c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd0d0 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbd0dc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd0e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd0e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd0e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd0ec | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbd0f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd0fc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd104 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbd110 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd114 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd118 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd11c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd120 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbd12c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd130 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd138 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbd144 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd148 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd14c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd150 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd160 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd164 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd178 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd17c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd180 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd184 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd194 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd198 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd1ac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd1b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd1b8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbd1bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd1c0 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbd1c4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd1c8 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbd1cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd1d0 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbd1d4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbd1d8 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbd1e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd1fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd204 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd210 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd218 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd21c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd220 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbd22c | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xbd234 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xbd238 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd23c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbd240 | store | 4 | `[sp-64] +0x0` | `[sp-24]` |
| `ldr` at 0xbd244 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd24c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbd250 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd254 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd258 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd25c | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbd268 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd26c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd274 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbd280 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd284 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd288 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd28c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd290 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbd29c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd2a0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd2a8 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbd2b4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd2b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd2bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd2c0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd2c4 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbd2d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd2d4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd2dc | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbd2e8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd2ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd2f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd2f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd304 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd308 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd31c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd320 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd324 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd328 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd338 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd33c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd350 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd354 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd358 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbd35c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd360 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbd364 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd368 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbd36c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbd370 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbd380 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd394 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd39c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd3a8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd3b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd3b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd3b8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbd3c0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbd3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd3d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbd3e8 | store | 4 | `sp -0x4c` | `0x5` |
| `str` at 0xbd3f4 | store | 4 | `sp -0x4c` | `0xc` |
| `ldr` at 0xbd3f8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbd3fc | store | 4 | `sp -0x24` | `[sp-76]` |
| `ldr` at 0xbd400 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd408 | store | 4 | `[sp-64] +0x0` | `0xe` |
| `ldr` at 0xbd40c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd414 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbd418 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd41c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd420 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd424 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbd430 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd434 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd43c | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbd448 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd44c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd450 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd454 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd458 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbd464 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd468 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd470 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbd47c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd480 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd484 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd488 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd48c | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbd498 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd49c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd4a4 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbd4b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd4b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd4b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd4bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd4cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd4d0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd4e4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd4e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd4ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd4f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd500 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd504 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd518 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd51c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd520 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbd524 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd528 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbd52c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd530 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbd534 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbd538 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbd548 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd55c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd564 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd570 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd578 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd57c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd580 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbd58c | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xbd594 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xbd598 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd59c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd5a0 | store | 4 | `[sp-64] +0x0` | `[sp]` |
| `ldr` at 0xbd5a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd5a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd5ac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd5b0 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbd5bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd5c0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd5c8 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbd5d4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd5d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd5dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd5e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd5e4 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbd5f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd5f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd5fc | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbd608 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd60c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd610 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd614 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd618 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbd624 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd628 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd630 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbd63c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd640 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd644 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd648 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd658 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd65c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd670 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd674 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd678 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd67c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd68c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd690 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd6a4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd6ac | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd6b8 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xbd6bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd6c4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbd6cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd6d8 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xbd6dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd6e0 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbd6e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd6e8 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbd6ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd6f0 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbd6f4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbd6f8 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbd708 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd71c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd724 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd730 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbd744 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd748 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd74c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbd754 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbd75c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd768 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xbd774 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd780 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd788 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xbd794 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xbd798 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbd7ac | store | 4 | `sp -0x50` | `0x5` |
| `str` at 0xbd7b8 | store | 4 | `sp -0x50` | `0xc` |
| `ldr` at 0xbd7bc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xbd7c0 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0xbd7c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd7c8 | load | 4 | `abs 0xbde24` | `-` |
| `str` at 0xbd7cc | store | 4 | `[sp-64] +0x0` | `0xfff` |
| `ldr` at 0xbd7d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd7d8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbd7dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd7e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd7e4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd7e8 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbd7f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd7f8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd800 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbd80c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd810 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd814 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd818 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd81c | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbd828 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd82c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd834 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbd840 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd844 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd848 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd84c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd850 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbd85c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd860 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd868 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbd874 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd878 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd87c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd880 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd890 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd894 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd8a8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd8ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd8b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd8b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbd8c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd8c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbd8dc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbd8e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd8e4 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbd8e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd8ec | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbd8f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd8f4 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbd8f8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbd8fc | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbd90c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbd920 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd928 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd934 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbd93c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd940 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd944 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbd950 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbd954 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd958 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd95c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbd960 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbd968 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbd96c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd970 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbd974 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbd980 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbd984 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbd988 | load | 4 | `abs 0xbde34` | `-` |
| `ldr` at 0xbd98c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbd994 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd99c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd9a8 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbd9b4 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xbd9b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbd9bc | load | 4 | `abs 0xbde28` | `-` |
| `str` at 0xbd9c0 | store | 4 | `[sp-64] +0x0` | `0xa800ffea` |
| `ldr` at 0xbd9c4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd9cc | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbd9d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbd9d4 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbd9d8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbd9dc | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbd9e0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbd9e4 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbd9e8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbd9ec | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbd9fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbda10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbda18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbda24 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbda2c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbda30 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbda34 | load | 4 | `abs 0xbde2c` | `-` |
| `ldr` at 0xbda40 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbda44 | load | 4 | `abs 0xbde30` | `-` |
| `ldr` at 0xbda4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbda54 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbda60 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbda68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbda7c | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xbda80 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbda88 | store | 4 | `[sp-64] +0x0` | `0x9` |
| `ldr` at 0xbda8c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbda94 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbda98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbda9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdaa0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdaa4 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbdab0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdab4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdabc | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbdac8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdacc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdad0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdad4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdad8 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbdae4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdae8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdaf0 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbdafc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdb00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb08 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdb0c | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbdb18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb1c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdb24 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbdb30 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdb34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb3c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdb4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb50 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbdb64 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdb68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb70 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdb80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdb84 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbdb98 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdb9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbdba0 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbdba4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbdba8 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbdbac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdbb0 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbdbb4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbdbb8 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbdbc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbdbdc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdbe4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdbf0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbdbf8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdbfc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdc00 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbdc0c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbdc10 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdc14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdc18 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbdc1c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbdc24 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbdc28 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdc2c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbdc30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbdc3c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdc40 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbdc44 | load | 4 | `abs 0xbde34` | `-` |
| `ldr` at 0xbdc48 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbdc50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdc58 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdc64 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbdc70 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xbdc74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdc78 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbdc7c | store | 4 | `[sp-64] +0x0` | `[sp+4]` |
| `ldr` at 0xbdc80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdc84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdc88 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdc8c | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbdc98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdc9c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdca4 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbdcb0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdcb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdcb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdcbc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdcc0 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbdccc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdcd0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdcd8 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbdce4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdce8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdcec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdcf0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdcf4 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbdd00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd04 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdd0c | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbdd18 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdd1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd24 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdd34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd38 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbdd4c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdd50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd58 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdd68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdd6c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbdd80 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdd84 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdd8c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbdd90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbdd94 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbdd98 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbdd9c | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbdda0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdda4 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbdda8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbddac | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbddbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbddd0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbddd8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdde4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbddec | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbddf0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbddf4 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbddfc | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbde00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbde14 | store | 4 | `sp -0x54` | `0x5` |
| `str` at 0xbde54 | store | 4 | `sp -0x54` | `0xc` |
| `ldr` at 0xbde58 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xbde5c | store | 4 | `sp -0x24` | `[sp-84]` |
| `ldr` at 0xbde60 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbde68 | store | 4 | `[sp-64] +0x0` | `0x8` |
| `ldr` at 0xbde6c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbde74 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbde78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbde7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbde80 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbde84 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbde90 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbde94 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbde9c | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbdea8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdeac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdeb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdeb4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdeb8 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbdec4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdec8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbded0 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbdedc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdee0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdee4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdee8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdeec | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbdef8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdefc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdf04 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbdf10 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdf14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdf18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdf1c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdf2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdf30 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbdf44 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdf48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdf4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdf50 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbdf60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdf64 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbdf78 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbdf7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbdf80 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbdf84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbdf88 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbdf8c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdf90 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbdf94 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbdf98 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbdfa8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbdfbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbdfc4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbdfd0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbdfd8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdfdc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdfe0 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbdfec | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbdff0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdff4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbdff8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbdffc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbe004 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe008 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe00c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe010 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbe01c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe020 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbe024 | load | 4 | `abs 0xbde34` | `-` |
| `ldr` at 0xbe028 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe030 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe038 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe044 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe04c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe060 | store | 4 | `sp -0x24` | `0xa` |
| `ldr` at 0xbe064 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe06c | load | 4 | `abs 0xbde38` | `-` |
| `str` at 0xbe074 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe078 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe07c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe080 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe084 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbe090 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe094 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe09c | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbe0a8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe0ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe0b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe0b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe0b8 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbe0c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe0c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe0d0 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbe0dc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe0e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe0e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe0e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe0ec | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbe0f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe0fc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe104 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbe110 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe114 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe118 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe11c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe12c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe130 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe144 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe148 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe14c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe150 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe160 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe164 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe178 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe180 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe18c | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xbe190 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe198 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbe1a0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe1ac | store | 4 | `[sp-68] +0x4` | `0x0` |
| `str` at 0xbe1b8 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xbe1bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe1cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe1d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1d8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe1e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe1e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe1f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1fc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe204 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe210 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xbe214 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe21c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbe224 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe230 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xbe234 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe238 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe23c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe240 | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbe24c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe250 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe258 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbe264 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe268 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe26c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe270 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe274 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbe280 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe284 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe28c | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbe298 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe29c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe2a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe2a4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe2a8 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbe2b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe2b8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe2c0 | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbe2cc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe2d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe2d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe2d8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe2e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe2ec | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe300 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe304 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe308 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe30c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe31c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe320 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe334 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe338 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbe33c | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbe340 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe344 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbe348 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe34c | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbe350 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbe354 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbe364 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbe378 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe380 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe38c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe394 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe3a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe3a4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe3a8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbe3b0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe3bc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe3c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe3d0 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xbe3dc | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xbe3e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbe3f4 | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xbe3f8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe400 | store | 4 | `[sp-64] +0x0` | `0x1` |
| `ldr` at 0xbe404 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe40c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbe410 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe414 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe418 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe41c | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbe428 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe42c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe434 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbe440 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe444 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe448 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe44c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe450 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbe45c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe460 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe468 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbe474 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe478 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe47c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe480 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe484 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbe490 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe494 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe49c | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbe4a8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe4ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe4b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe4b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe4c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe4c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe4dc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe4e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe4e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe4e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe4f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe4fc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe510 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe514 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbe518 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbe51c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe520 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbe524 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe528 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbe52c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbe530 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbe540 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbe554 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe55c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe568 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe570 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe574 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe578 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbe57c | store | 4 | `[sp-68] +0x0` | `[[sp-68]]` |
| `str` at 0xbe584 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xbe588 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe590 | store | 4 | `[sp-64] +0x0` | `0x0` |
| `ldr` at 0xbe594 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe59c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xbe5a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe5a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe5a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe5ac | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbe5b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe5bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe5c4 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbe5d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe5d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe5d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe5dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe5e0 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbe5ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe5f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe5f8 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbe604 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe608 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe60c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe610 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe614 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbe620 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe624 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe62c | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbe638 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe63c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe640 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe644 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe654 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe658 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe66c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe670 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe674 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe678 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbe688 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe68c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xbe6a0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbe6a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbe6a8 | store | 4 | `sp -0x64` | `[sp-36]` |
| `ldr` at 0xbe6ac | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe6b0 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbe6b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe6b8 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbe6bc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbe6c0 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbe6d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbe6e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe6ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe6f8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe700 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe704 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe708 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe70c | load | 4 | `abs 0xbde3c` | `-` |
| `ldr` at 0xbe718 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe71c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe724 | load | 4 | `abs 0xbde3c` | `-` |
| `str` at 0xbe730 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe734 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe738 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe73c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe740 | load | 4 | `abs 0xbde40` | `-` |
| `ldr` at 0xbe74c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe750 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe758 | load | 4 | `abs 0xbde40` | `-` |
| `str` at 0xbe764 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe768 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe76c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe770 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe774 | load | 4 | `abs 0xbde44` | `-` |
| `ldr` at 0xbe780 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe784 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe78c | load | 4 | `abs 0xbde44` | `-` |
| `str` at 0xbe798 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe79c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe7a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe7a4 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe7b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe7b8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbe7cc | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe7d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe7d4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe7d8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xbe7e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe7ec | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbe800 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xbe804 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xbe808 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbe80c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xbe810 | store | 4 | `[sp+8] +0x0` | `[[sp-68]]` |
| `str` at 0xbe818 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xbe81c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe830 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbe834 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbe848 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe850 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe85c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe864 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe89c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbe8a0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbe8b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe8bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe8c8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe8e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe8f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbe900 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbe914 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe91c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe928 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbe930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe940 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbe948 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbe954 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xbe960 | store | 4 | `sp -0x38` | `0x100` |
| `ldr` at 0xbe964 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbe974 | store | 4 | `sp -0x3c` | `0x300` |
| `ldr` at 0xbe978 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbe98c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbe998 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xbe9a0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xbe9a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbe9a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbe9b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbe9c0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbe9cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbe9d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbe9d8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xbe9e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbe9ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbe9fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbea04 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbea10 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbea14 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbea1c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xbea28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbea38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbea48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbea50 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbea5c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbea60 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbea68 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xbea70 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbea74 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xbea78 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbea7c | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbea80 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbea84 | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbea88 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbea8c | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbea9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbeab0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbeab8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbeac4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbeacc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbead0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbead4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbead8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xbeaec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbeaf0 | load | 4 | `abs 0xbde48` | `-` |
| `ldr` at 0xbeaf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbeb00 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbeb0c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbeb18 | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0xbeb1c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbeb2c | store | 4 | `sp -0x3c` | `0xc0` |
| `ldr` at 0xbeb30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbeb44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbeb50 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xbeb58 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xbeb5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbeb60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbeb70 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbeb78 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbeb84 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbeb88 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbeb90 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xbeb9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbeba4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbebb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbebbc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbebc8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbebcc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbebd4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xbebe0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbebf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbec00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbec08 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbec14 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbec18 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbec20 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xbec28 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbec2c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xbec30 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbec34 | store | 4 | `sp -0x60` | `[sp-64]` |
| `ldr` at 0xbec38 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbec3c | store | 4 | `sp -0x5c` | `[sp-68]` |
| `ldr` at 0xbec40 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbec44 | store | 4 | `sp -0x58` | `[sp+16]` |
| `ldr` at 0xbec54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbec68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbec70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbec7c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbec84 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbec88 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbec8c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbec90 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xbeca4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xbeca8 | load | 4 | `abs 0xbde4c` | `-` |
| `ldr` at 0xbecb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbecb8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbecc4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbeccc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbecd4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbece0 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xbece4 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xbece8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbece8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbece8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbce68 | `(unknown callee) 0x18920` | `0x60` | `r1` | `r2` | `r3` |
| `bl` at 0xbce88 | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109be0` | `?` | `ret(0x18920)` |
| `bl` at 0xbce9c | `(unknown callee) 0x18920` | `0x60` | `?` | `?` | `ret(0x18920)` |
| `bl` at 0xbcebc | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109be0` | `?` | `ret(0x18920)` |
| `bl` at 0xbcec4 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbd00c | `(unknown callee) 0xbc308` | `0x0` | `0x1` | `[sp+16]` | `?` |
| `bl` at 0xbd020 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xbd028 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd044 | `(unknown callee) 0xbc308` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xbd058 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xbd060 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd1ec | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xbd200 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbd208 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd384 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbd398 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbd3a0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd54c | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbd560 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbd568 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd70c | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbd720 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbd728 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd910 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbd924 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbd92c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbd990 | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109c3c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xbd998 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbd9a0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbda00 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbda14 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbda1c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbda48 | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109c0c` | `[[sp-68]]` | `0x54007ff5` |
| `bl` at 0xbda50 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbda58 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbdbcc | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbdbe0 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbdbe8 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbdc4c | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109c3c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xbdc54 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbdc5c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbddc0 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbddd4 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbdddc | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbdfac | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbdfc0 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbdfc8 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe02c | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109c3c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xbe034 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbe03c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe368 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbe37c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbe384 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe544 | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xbe558 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbe560 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe6d4 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbe6e8 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbe6f0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe838 | `(unknown callee) 0xbc308` | `0x0` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xbe84c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xbe854 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe8a4 | `(unknown callee) 0xbc308` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xbe8b8 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xbe8c0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe904 | `(unknown callee) 0xbc4e8` | `0x0` | `[sp-52]` | `0x0` | `[sp+16]` |
| `bl` at 0xbe918 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc4e8)` |
| `bl` at 0xbe920 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbe944 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbe94c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbeaa0 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbeab4 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbeabc | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbeadc | `(unknown callee) 0xc05a8` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xbeaf4 | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109c68` | `?` | `ret(0xc05a8)` |
| `bl` at 0xbeafc | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbeb04 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbec58 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xbec6c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbec74 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbec94 | `(unknown callee) 0xc05a8` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+12]` |
| `bl` at 0xbecac | `(unknown callee) 0x18c74` | `[sp+16]` | `0x109c90` | `?` | `ret(0xc05a8)` |
| `bl` at 0xbecb4 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbecbc | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbecd0 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `ret(0xc05a8)` |
| `bl` at 0xbecd8 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbce7c | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xbceb0 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `beq` at 0xbcee0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xbcffc | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbd018 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xbd050 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `beq` at 0xbd07c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbd1f8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbd390 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbd3d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xbd3e0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbd558 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbd718 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbd740 | `[sp-36] != 0x18` | `[sp-36] == 0x18` |
| `beq` at 0xbd7a4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbd91c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbd97c | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xbda0c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbda3c | `[[sp-68]] == 0x54007ff5` | `[[sp-68]] != 0x54007ff5` |
| `beq` at 0xbda74 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbdbd8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbdc38 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xbddcc | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbde0c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbdfb8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbe018 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `beq` at 0xbe058 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xbe1c8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbe374 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbe39c | `[sp-36] != 0x8` | `[sp-36] == 0x8` |
| `beq` at 0xbe3ec | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbe550 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbe6e0 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbe828 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbe844 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xbe870 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbe880 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbe890 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbe8b0 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xbe8dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbe8ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbe910 | `ret(0xbc4e8) != 0x0` | `ret(0xbc4e8) == 0x0` |
| `bne` at 0xbe93c | `? != 0x0` | `? == 0x0` |
| `blo` at 0xbe9b0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbe9f4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbea40 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbeaac | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbeae8 | `ret(0xc05a8) != 0x0` | `ret(0xc05a8) == 0x0` |
| `blo` at 0xbeb68 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbebac | `C == 0` | `not (C == 0)` |
| `blo` at 0xbebf8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbec64 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbeca0 | `ret(0xc05a8) != 0x0` | `ret(0xc05a8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbece8 | `r0` | `?` |

## `jmstr_sel_sic_multi`

- `jmstr_sel_sic_multi`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbecec to 0xc05a8.
  Established. A word read from entry `r0 +0x10` feeds two initial tests (`ldr` at 0xbed18). Zero
  takes the first bypass and an unsigned value above `0x8` takes the second, while their respective
  fall-through predicates are nonzero and at most `0x8` (`beq` at 0xbed28, `bhi` at 0xbed34). The
  first call to `0x18920` carries candidates `0x60`, entry r1, `0x10`, `[r0+16]` (`bl` at 0xbed5c).
  Later word stores include `0x800ffe0` at an unknown target and `0x0` at another unknown target
  (`str` at 0xbf904, `str` at 0xbf91c). The routine returns an unknown r0 after its paths join
  (`ldmdb` at 0xc05a4). The count’s device meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbecfc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbed00 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbed04 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbed08 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbed10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbed18 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xbed1c | store | 4 | `sp -0x68` | `[r0+16]` |
| `ldr` at 0xbed20 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbed2c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbed3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed40 | load | 4 | `abs 0xbfc80` | `-` |
| `ldr` at 0xbed44 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xbed50 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbed64 | store | 4 | `sp -0x3c` | `ret(0x18920)` |
| `ldr` at 0xbed68 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbed74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed78 | load | 4 | `abs 0xbfc84` | `-` |
| `str` at 0xbed84 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbed98 | store | 4 | `sp -0x40` | `ret(0x18920)` |
| `ldr` at 0xbed9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbeda8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbedac | load | 4 | `abs 0xbfc84` | `-` |
| `ldr` at 0xbedb4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbedc0 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbedd4 | store | 4 | `sp -0x44` | `ret(0x18920)` |
| `ldr` at 0xbedd8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xbede4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbede8 | load | 4 | `abs 0xbfc84` | `-` |
| `ldr` at 0xbedf0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbedf8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbee04 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbee10 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbee14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbee18 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbee28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbee3c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xbee40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbee58 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xbee64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbee70 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbee74 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbee78 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbee80 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbee8c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbee90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbee94 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbeea4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbeeb8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xbeebc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbeed4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xbeee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbeeec | load | 4 | `(unknown)` | `-` |
| `str` at 0xbef00 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbef04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbef0c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbef20 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbef24 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbef30 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xbef34 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xbef48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbef4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbef60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbef68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbef70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbef7c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbef88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbef8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbefa0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbefa8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbefb0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbefbc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbefc4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xbefcc | store | 4 | `sp -0x94` | `[sp-104]-1` |
| `ldr` at 0xbefd0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbefe4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbefe8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbefec | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbeffc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf004 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf010 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf014 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf01c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf028 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf038 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf040 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf048 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xbf04c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf050 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf064 | load | 4 | `abs 0xbfc88` | `-` |
| `ldr` at 0xbf06c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf074 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf07c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf084 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf088 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xbf08c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf094 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xbf0a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf0a4 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xbf0a8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf0ac | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf0b0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf0b4 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf0b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf0bc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf0cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf0e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf0e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf0f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf0fc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbf104 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf108 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xbf10c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf110 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf114 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf118 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf11c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf120 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf130 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf144 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf14c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf154 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf160 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbf16c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf170 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf174 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf184 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf18c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf198 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf19c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf1a4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf1b0 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf1b4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf1c8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xbf1cc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xbf1d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf1e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf1e8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbf1f0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbf1f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf200 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf208 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xbf20c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf210 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf22c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf234 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf23c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf244 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf248 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xbf24c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf250 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf25c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf260 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf264 | store | 4 | `sp -0xb0` | `[sp-36]` |
| `ldr` at 0xbf268 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf26c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf270 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf274 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf27c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf28c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf2a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf2a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf2b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf2bc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbf2c4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xbf2dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf2f0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf2f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf2f8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf308 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf310 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf31c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf320 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf328 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf334 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf344 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf34c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf354 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xbf358 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf35c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf370 | load | 4 | `abs 0xbfc88` | `-` |
| `ldr` at 0xbf378 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf380 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf388 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf390 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf394 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xbf398 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf39c | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xbf3a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf3a4 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf3a8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf3ac | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf3b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf3b4 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf3c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf3d8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf3e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf3e8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf3f4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbf400 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf404 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf408 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf418 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf420 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf42c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf430 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf438 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf444 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf448 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf45c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xbf460 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xbf46c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf478 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf47c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbf484 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbf48c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf494 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf49c | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xbf4a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf4a4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf4c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf4c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf4d0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf4d8 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf4dc | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xbf4e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf4e4 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xbf4e8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf4ec | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf4f0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf4f4 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf4f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf4fc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf50c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf520 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf528 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf530 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf53c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbf544 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xbf55c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf570 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf574 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf578 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf588 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf590 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf59c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf5a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf5a8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf5b4 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf5c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf5cc | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf5d4 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xbf5d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf5dc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf5f0 | load | 4 | `abs 0xbfc88` | `-` |
| `ldr` at 0xbf5f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf600 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf608 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf610 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf614 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xbf618 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf61c | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xbf620 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf624 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf628 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf62c | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf630 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf634 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf644 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf658 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf660 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf668 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf674 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbf680 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf684 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf688 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf698 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf6a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf6ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf6b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf6b8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf6c4 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf6c8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf6dc | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xbf6e0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xbf6ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf6f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf6fc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbf704 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbf70c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf714 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf71c | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xbf720 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf724 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf740 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf748 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf750 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf758 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf75c | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xbf760 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf764 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xbf768 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf76c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf770 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf774 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf778 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf77c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf78c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf7a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf7a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf7b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf7bc | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbf7c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf7cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf7d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf7e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf7e8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf7f4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf7f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf800 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbf80c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xbf810 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf814 | store | 4 | `sp -0x2c` | `[sp-148]` |
| `ldr` at 0xbf818 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf81c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf830 | load | 4 | `abs 0xbfc88` | `-` |
| `ldr` at 0xbf838 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf840 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf848 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf850 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xbf854 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf858 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf85c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xbf860 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf864 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf868 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf86c | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf870 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf874 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf884 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf898 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf8a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf8a8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf8b4 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbf8bc | load | 4 | `sp -0x94` | `-` |
| `str` at 0xbf8c4 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `str` at 0xbf8d0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbf8d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf8d8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf8e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf900 | load | 4 | `abs 0xbfc8c` | `-` |
| `str` at 0xbf904 | store | 4 | `(unknown)` | `0x800ffe0` |
| `ldr` at 0xbf908 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf910 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf91c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf920 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf928 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf934 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbf938 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf940 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbf954 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbf958 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf960 | store | 4 | `sp -0x24` | `ret(0xc0778)` |
| `ldr` at 0xbf964 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf968 | store | 4 | `sp -0xb0` | `ret(0xc0778)` |
| `ldr` at 0xbf96c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbf970 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbf974 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbf978 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbf97c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbf980 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbf990 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf9a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbf9ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbf9b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbf9c0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbf9c8 | load | 4 | `abs 0xbfc8c` | `-` |
| `str` at 0xbf9cc | store | 4 | `sp -0x88` | `0x800ffe0` |
| `ldr` at 0xbf9d0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbf9d4 | load | 4 | `abs 0xbfc94` | `-` |
| `ldr` at 0xbf9e0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbf9e8 | load | 4 | `abs 0xbfc94` | `-` |
| `str` at 0xbf9f4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbf9f8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbf9fc | load | 4 | `abs 0xbfc98` | `-` |
| `ldr` at 0xbfa08 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfa10 | load | 4 | `abs 0xbfc98` | `-` |
| `str` at 0xbfa1c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbfa20 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfa24 | load | 4 | `abs 0xbfc9c` | `-` |
| `ldr` at 0xbfa30 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfa38 | load | 4 | `abs 0xbfc9c` | `-` |
| `str` at 0xbfa44 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbfa48 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfa58 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xbfa6c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbfa70 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfa80 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xbfa94 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xbfa9c | load | 4 | `sp -0x68` | `-` |
| `str` at 0xbfaa4 | store | 4 | `sp -0x9c` | `?` |
| `str` at 0xbfaac | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xbfab0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfab4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfac4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfadc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfae0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfae8 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0xbfaf0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbfaf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfb0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfb24 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfb2c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfb34 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xbfb40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfb48 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbfb4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfb54 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbfb5c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xbfb60 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xbfb68 | store | 4 | `sp -0x88` | `?` |
| `str` at 0xbfb70 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbfb74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfb78 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfb88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfb90 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbfb98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfbb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfbb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbfbc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfbc4 | load | 4 | `abs 0xbfc90` | `-` |
| `ldr` at 0xbfbcc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbfbd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbfbdc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbfbe8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbfbf0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfbf8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xbfc04 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xbfc08 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbfc0c | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xbfc10 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbfc14 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xbfc18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbfc1c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbfc20 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfc40 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbfc48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbfc50 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbfc5c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbfc68 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbfc6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfc70 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfca8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfcbc | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xbfcc0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xbfccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfcd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbfcdc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xbfce0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfcf8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xbfd04 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xbfd10 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbfd14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfd1c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbfd28 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbfd2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfd34 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbfd40 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbfd44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfd4c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbfd60 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfd64 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbfd6c | store | 4 | `sp -0x24` | `ret(0xc0778)` |
| `ldr` at 0xbfd70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbfd74 | store | 4 | `sp -0xb0` | `ret(0xc0778)` |
| `ldr` at 0xbfd78 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbfd7c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbfd80 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbfd84 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbfd88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbfd8c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbfd9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbfdb0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbfdb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbfdc0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbfdcc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbfdd4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xbfde8 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xbfdec | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbfdf0 | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xbfdf4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbfdf8 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xbfdfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbfe00 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbfe04 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfe24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbfe2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbfe34 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbfe40 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xbfe4c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xbfe50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfe54 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfe64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfe80 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbfe84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfe8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbfe98 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbfe9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfea4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbfeb0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xbfeb4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfebc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xbfed0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbfed4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbfedc | store | 4 | `sp -0x24` | `ret(0xc0778)` |
| `ldr` at 0xbfee0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbfee4 | store | 4 | `sp -0xb0` | `ret(0xc0778)` |
| `ldr` at 0xbfee8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbfeec | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xbfef0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbfef4 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xbfef8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbfefc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xbff0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbff20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbff28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff30 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbff3c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xbff44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff4c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbff50 | load | 4 | `abs 0xbfc94` | `-` |
| `ldr` at 0xbff5c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff60 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbff68 | load | 4 | `abs 0xbfc94` | `-` |
| `str` at 0xbff74 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbff78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff80 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbff84 | load | 4 | `abs 0xbfc98` | `-` |
| `ldr` at 0xbff90 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbff94 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbff9c | load | 4 | `abs 0xbfc98` | `-` |
| `str` at 0xbffa8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbffac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbffb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbffb4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbffb8 | load | 4 | `abs 0xbfc9c` | `-` |
| `ldr` at 0xbffc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbffc8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbffd0 | load | 4 | `abs 0xbfc9c` | `-` |
| `str` at 0xbffdc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xbffe0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbffe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbffe8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xbfff8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbfffc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xc0010 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xc0014 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0018 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc001c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xc002c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0030 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xc0044 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xc0048 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc004c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0050 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xc0054 | store | 4 | `[sp-24] +0x0` | `[[sp-64]]` |
| `str` at 0xc005c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc0060 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc0074 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc008c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc0094 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc009c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc00a8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc00b0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc00c0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc00d0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc00e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc00ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0100 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc0108 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0110 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc011c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc0124 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc0134 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc014c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0154 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0168 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc0170 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0178 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc0184 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc018c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc019c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc01a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc01ac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc01b8 | store | 4 | `sp -0xa0` | `0x1` |
| `str` at 0xc01c4 | store | 4 | `sp -0x34` | `0x100` |
| `ldr` at 0xc01c8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc01d8 | store | 4 | `sp -0x38` | `0x300` |
| `ldr` at 0xc01dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc01f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc01fc | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc0204 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc0208 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc020c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc021c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0224 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc0230 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc0234 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc023c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc0248 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0250 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0260 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0268 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc0274 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc0278 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0280 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc028c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc029c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc02ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc02b4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc02c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc02c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc02cc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc02d4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc02d8 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xc02dc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc02e0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xc02e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc02e8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xc02ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc02f0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xc0300 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0314 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc031c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0324 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc0330 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc033c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0340 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc0344 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0358 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc035c | load | 4 | `abs 0xbfca0` | `-` |
| `ldr` at 0xc0364 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc036c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0374 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc0380 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc0388 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0398 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc039c | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `str` at 0xc03a4 | store | 4 | `sp -0x34` | `0x40` |
| `ldr` at 0xc03a8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc03b8 | store | 4 | `sp -0x38` | `0xc0` |
| `ldr` at 0xc03bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc03d0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc03dc | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc03e4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc03e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc03ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc03fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0404 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc0410 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc0414 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc041c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc0428 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0430 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0440 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0448 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc0454 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc0458 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0460 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc046c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc047c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc048c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0494 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc04a0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc04a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc04ac | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc04b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc04b8 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xc04bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc04c0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xc04c4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc04c8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xc04cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc04d0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xc04e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc04f4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc04fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0504 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc0510 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc051c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0520 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc0524 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0538 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc053c | load | 4 | `abs 0xbfca4` | `-` |
| `ldr` at 0xc0544 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc054c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0554 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc0560 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xc0568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0578 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc057c | store | 4 | `[sp-28] +0x0` | `[sp-40]` |
| `ldr` at 0xc0580 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc0588 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc0590 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc059c | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0xc05a0 | load | 4 | `sp -0xa0` | `-` |
| `ldmdb` at 0xc05a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc05a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc05a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbed48 | `(unknown callee) 0x18c74` | `r3` | `0x109cc0` | `[r0+16]` | `[r0+16]` |
| `bl` at 0xbed5c | `(unknown callee) 0x18920` | `0x60` | `r1` | `0x10` | `[r0+16]` |
| `bl` at 0xbed7c | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109be0` | `?` | `ret(0x18920)` |
| `bl` at 0xbed90 | `(unknown callee) 0x18920` | `0x60` | `?` | `?` | `ret(0x18920)` |
| `bl` at 0xbedb0 | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109be0` | `?` | `ret(0x18920)` |
| `bl` at 0xbedb8 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbedcc | `(unknown callee) 0x18920` | `0x60` | `?` | `?` | `ret(0x18920)` |
| `bl` at 0xbedec | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109be0` | `?` | `ret(0x18920)` |
| `bl` at 0xbedf4 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbedfc | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbef28 | `(unknown callee) 0xc0778` | `sp-136` | `0xc` | `[sp-104]` | `?` |
| `bl` at 0xbef50 | `(unknown callee) 0xbc308` | `0x0` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0xbef64 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xbef6c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbef74 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbef90 | `(unknown callee) 0xbc308` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xbefa4 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xbefac | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbefb4 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf03c | `(unknown callee) 0xc09d4` | `0x4` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf070 | `(unknown callee) 0xc09d4` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf0d0 | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `[sp-48]` |
| `bl` at 0xbf0e4 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf0ec | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf0f4 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf134 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf148 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf150 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf158 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf1fc | `(unknown callee) 0xc09d4` | `[sp-136]` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf230 | `(unknown callee) 0xc09d4` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf290 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf2a4 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf2ac | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf2b4 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf348 | `(unknown callee) 0xc09d4` | `0xe` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf37c | `(unknown callee) 0xc09d4` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf3c8 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf3dc | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf3e4 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf3ec | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf490 | `(unknown callee) 0xc09d4` | `[sp-136]` | `0x18` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf4c4 | `(unknown callee) 0xc09d4` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf510 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf524 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf52c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf534 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf5c8 | `(unknown callee) 0xc09d4` | `0x9` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf5fc | `(unknown callee) 0xc09d4` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf648 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf65c | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf664 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf66c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf710 | `(unknown callee) 0xc09d4` | `[sp-136]` | `0x8` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf744 | `(unknown callee) 0xc09d4` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf790 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf7a4 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf7ac | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf7b4 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf83c | `(unknown callee) 0xc09d4` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xbf888 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf89c | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf8a4 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf8ac | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbf95c | `(unknown callee) 0xc0778` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xbf994 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbf9a8 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbf9b0 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbf9b8 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbfbc8 | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109c0c` | `?` | `?` |
| `bl` at 0xbfbd0 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbfbd8 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbfbe0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbfc30 | `(unknown callee) 0xc0c28` | `[sp-104]` | `0x8` | `0xc` | `0x1` |
| `bl` at 0xbfc44 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xc0c28)` |
| `bl` at 0xbfc4c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbfc54 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbfd68 | `(unknown callee) 0xc0778` | `sp-136` | `0x8` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xbfda0 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbfdb4 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbfdbc | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbfdc4 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbfe14 | `(unknown callee) 0xc0c28` | `[sp-104]` | `0x1` | `0xc` | `0x1` |
| `bl` at 0xbfe28 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xc0c28)` |
| `bl` at 0xbfe30 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbfe38 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xbfed8 | `(unknown callee) 0xc0778` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xbff10 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xbff24 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xbff2c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xbff34 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc007c | `(unknown callee) 0xbc308` | `0x0` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0xc0090 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xc0098 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc00a0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc00f0 | `(unknown callee) 0xbc308` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xc0104 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc308)` |
| `bl` at 0xc010c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc0114 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc0158 | `(unknown callee) 0xbc4e8` | `0x0` | `[sp-48]` | `0x0` | `[sp-32]` |
| `bl` at 0xc016c | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc4e8)` |
| `bl` at 0xc0174 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc017c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc01a0 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xc01a8 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc01b0 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc0304 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xc0318 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xc0320 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc0328 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc0348 | `(unknown callee) 0xc05a8` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xc0360 | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109c68` | `?` | `ret(0xc05a8)` |
| `bl` at 0xc0368 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xc0370 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc0378 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc04e4 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xc04f8 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xc0500 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc0508 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc0528 | `(unknown callee) 0xc05a8` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xc0540 | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109c90` | `?` | `ret(0xc05a8)` |
| `bl` at 0xc0548 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xc0550 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc0558 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc0584 | `(unknown callee) 0x18be4` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xc058c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc0594 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbed28 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bhi` at 0xbed34 | `[r0+16] > 0x8 (unsigned, 32-bit)` | `[r0+16] <= 0x8 (unsigned, 32-bit)` |
| `bne` at 0xbed70 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xbeda4 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xbede0 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `blo` at 0xbee20 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbee9c | `C == 0` | `not (C == 0)` |
| `beq` at 0xbef40 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbef5c | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xbef9c | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bge` at 0xbefd8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xbeff4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf058 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf09c | `[sp-148] != [sp-104]-1` | `[sp-148] == [sp-104]-1` |
| `bne` at 0xbf0dc | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xbf140 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xbf17c | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf218 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf29c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbf2e8 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xbf300 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf364 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf3d4 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xbf410 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf4ac | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf51c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbf568 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xbf580 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf5e4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf654 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xbf690 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf72c | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf79c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xbf7d8 | `C == 0` | `not (C == 0)` |
| `blo` at 0xbf824 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf894 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xbf8e0 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbf9a0 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xbfabc | `C == 0` | `not (C == 0)` |
| `blo` at 0xbfb80 | `C == 0` | `not (C == 0)` |
| `beq` at 0xbfbbc | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbfc3c | `ret(0xc0c28) != 0x0` | `ret(0xc0c28) == 0x0` |
| `blo` at 0xbfc78 | `C == 0` | `not (C == 0)` |
| `bne` at 0xbfdac | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xbfde0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbfe20 | `ret(0xc0c28) != 0x0` | `ret(0xc0c28) == 0x0` |
| `blo` at 0xbfe5c | `C == 0` | `not (C == 0)` |
| `bne` at 0xbff1c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xc006c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc0088 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xc00bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc00cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc00dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc00fc | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xc0130 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc0140 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc0164 | `ret(0xbc4e8) != 0x0` | `ret(0xbc4e8) == 0x0` |
| `bne` at 0xc0198 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xc0214 | `C == 0` | `not (C == 0)` |
| `blo` at 0xc0258 | `C == 0` | `not (C == 0)` |
| `blo` at 0xc02a4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc0310 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xc0354 | `ret(0xc05a8) != 0x0` | `ret(0xc05a8) == 0x0` |
| `beq` at 0xc0390 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blo` at 0xc03f4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xc0438 | `C == 0` | `not (C == 0)` |
| `blo` at 0xc0484 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc04f0 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xc0534 | `ret(0xc05a8) != 0x0` | `ret(0xc05a8) == 0x0` |
| `beq` at 0xc0570 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc05a4 | `r0` | `?` |

## `sn1sc_dr_load`

- `sn1sc_dr_load`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc44c0 to 0xc478c.
  Established. The word at entry `r0 +0x10` is tested for zero, following the bypass on equality and
  falling through otherwise (`ldr` at 0xc44e0, `beq` at 0xc44f0). The initial call to `0xbc8b4` has
  candidates `0x0`, unknown, `0x1`, unknown (`bl` at 0xc455c). Stack words carry `r0+24` at `sp
  -0x40` and `sp -0x3c` before that call (`str` at 0xc453c, `str` at 0xc4544). A later call to the
  same target uses `0x1`, `0x6`, `0x1`, unknown (`bl` at 0xc4748). The reported return is unknown,
  as is the physical load operation (`ldmdb` at 0xc4784).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc44d0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc44d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc44e0 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xc44e4 | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0xc44e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc44f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc44fc | store | 4 | `sp -0x28` | `r0+24` |
| `ldr` at 0xc4504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc450c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xc451c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4524 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xc4530 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc4534 | store | 4 | `sp -0x44` | `[r0+16]` |
| `ldr` at 0xc4538 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc453c | store | 4 | `sp -0x40` | `r0+24` |
| `ldr` at 0xc4540 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4544 | store | 4 | `sp -0x3c` | `r0+24` |
| `ldr` at 0xc4548 | load | 4 | `abs 0xc4788` | `-` |
| `str` at 0xc454c | store | 4 | `sp -0x38` | `0x2842a8` |
| `str` at 0xc4564 | store | 4 | `sp -0x18` | `ret(0xbc8b4)` |
| `ldr` at 0xc4568 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc4578 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc4584 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc458c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc4590 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc4594 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc45a4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xc45ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc45b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc45c8 | store | 4 | `sp -0x1c` | `0x6` |
| `str` at 0xc45d4 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xc45d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc45e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc45f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc45fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc4600 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc4608 | store | 4 | `sp -0x34` | `[sp-52]+24` |
| `ldr` at 0xc4610 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc461c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc4624 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc4628 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc4630 | store | 4 | `sp -0x34` | `[sp-52]+28` |
| `ldr` at 0xc4634 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc4638 | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0xc4640 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4648 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4654 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc4658 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc465c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4660 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0xc4664 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4668 | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xc466c | load | 4 | `abs 0xc4788` | `-` |
| `str` at 0xc4670 | store | 4 | `sp -0x38` | `0x2842a8` |
| `ldr` at 0xc467c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc468c | store | 4 | `sp -0x18` | `ret(0xbc8b4)` |
| `ldr` at 0xc4690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4694 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc46ac | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xc46b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc46cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc46e8 | store | 4 | `sp -0x2c` | `ret(0x18920)` |
| `ldr` at 0xc46ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc46f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc46fc | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xc4704 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc470c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xc4718 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc471c | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xc4720 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4724 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0xc4728 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc472c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc4730 | load | 4 | `abs 0xc4788` | `-` |
| `str` at 0xc4734 | store | 4 | `sp -0x38` | `0x2842a8` |
| `str` at 0xc4750 | store | 4 | `sp -0x18` | `ret(0xbc8b4)` |
| `ldr` at 0xc4754 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4758 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4768 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc4770 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4778 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc477c | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0xc4780 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xc4784 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4784 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc455c | `(unknown callee) 0xbc8b4` | `0x0` | `?` | `0x1` | `?` |
| `bl` at 0xc4684 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `[sp-28]` | `?` |
| `bl` at 0xc46e0 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xc4748 | `(unknown callee) 0xbc8b4` | `0x1` | `0x6` | `0x1` | `?` |
| `bl` at 0xc4774 | `(unknown callee) 0x18be4` | `[sp-44]` | `?` | `[sp-44]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc44f0 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bne` at 0xc4570 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xc459c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc45c0 | `? != 0x1` | `? == 0x1` |
| `bne` at 0xc45e4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc46a4 | `? == 0x1` | `? != 0x1` |
| `bne` at 0xc46f4 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xc4760 | `[sp-44] != [sp-40]` | `[sp-44] == [sp-40]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4784 | `r0` | `?` |

## `build_sic_tdi`

- `build_sic_tdi`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0778 to 0xc09d4.
  Established. One word store clears entry `r3 +0x0` (`str` at 0xc07fc). Two other word stores have
  unknown targets and values (`str` at 0xc0938, `str` at 0xc099c). The deciding unsigned comparison
  takes its edge when `([sp-36] + [sp-24]) <= 0x1f` and falls through above that bound (`bls` at
  0xc0954). No calls are reported. The r0 return and the packed data’s meaning remain unknown
  (`ldmdb` at 0xc09c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0788 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc078c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc0790 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc0794 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc079c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xc07a4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc07a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc07ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc07c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc07c8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc07d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc07d4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc07dc | store | 4 | `sp -0x34` | `?` |
| `str` at 0xc07e8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc07f0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc07f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc07fc | store | 4 | `r3 +0x0` | `0x0` |
| `str` at 0xc0804 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc0808 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc080c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc081c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0824 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc082c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0830 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc0838 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc083c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0840 | load | 4 | `abs 0xc09c8` | `-` |
| `ldr` at 0xc084c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0854 | load | 4 | `abs 0xc09c8` | `-` |
| `str` at 0xc0860 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0864 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0868 | load | 4 | `abs 0xc09cc` | `-` |
| `ldr` at 0xc0874 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc087c | load | 4 | `abs 0xc09cc` | `-` |
| `str` at 0xc0888 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc088c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0890 | load | 4 | `abs 0xc09d0` | `-` |
| `ldr` at 0xc089c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc08a4 | load | 4 | `abs 0xc09d0` | `-` |
| `str` at 0xc08b0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc08b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc08c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc08d8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc08dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc08ec | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc0900 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0904 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc090c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0914 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc091c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0924 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0928 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0930 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0938 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc093c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0940 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0948 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])` |
| `ldr` at 0xc094c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0958 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0960 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])-32` |
| `ldr` at 0xc0964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0968 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0970 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0974 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc097c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xc0980 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0988 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0990 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0994 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc099c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc09a0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc09a8 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0xc09b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc09b8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc09c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc09c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc09c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc07b4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xc0814 | `C == 0` | `not (C == 0)` |
| `bls` at 0xc0954 | `([sp-36] + [sp-24]) <= 0x1f (unsigned, 32-bit)` | `([sp-36] + [sp-24]) > 0x1f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc09c4 | `r0` | `?` |

## `add_sic_tdi`

- `add_sic_tdi`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc09d4 to 0xc0c28.
  Established. The final explicit word store writes `([[sp-32]] + [sp-24])` at `[sp-32] +0x0` (`str`
  at 0xc0c08). Earlier word stores at unknown targets also have unknown values (`str` at 0xc0b8c,
  `str` at 0xc0bf0). The branch takes its edge for the unsigned comparison `([sp-40] + [sp-24]) <=
  0x20` and falls through above it (`bls` at 0xc0ba8). The return reports `[[sp-32]]` in r0 (`ldmdb`
  at 0xc0c18). No call is reported, and the appended data’s protocol role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc09e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc09e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc09ec | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc09f0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc09f8 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xc0a00 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc0a04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0a08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0a1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc0a24 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc0a2c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc0a30 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc0a38 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc0a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0a44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc0a4c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0a50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0a54 | load | 4 | `abs 0xc0c1c` | `-` |
| `ldr` at 0xc0a60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0a68 | load | 4 | `abs 0xc0c1c` | `-` |
| `str` at 0xc0a74 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0a78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0a7c | load | 4 | `abs 0xc0c20` | `-` |
| `ldr` at 0xc0a88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0a90 | load | 4 | `abs 0xc0c20` | `-` |
| `str` at 0xc0a9c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0aa0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0aa4 | load | 4 | `abs 0xc0c24` | `-` |
| `ldr` at 0xc0ab0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0ab8 | load | 4 | `abs 0xc0c24` | `-` |
| `str` at 0xc0ac4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0ac8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0ad8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc0aec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0af0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0b00 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc0b14 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc0b18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0b1c | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xc0b20 | store | 4 | `sp -0x28` | `[r3]` |
| `str` at 0xc0b28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0b2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0b3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0b44 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc0b48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0b50 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0b58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0b60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0b68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0b70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0b78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0b7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0b84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0b8c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc0b90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0b94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0b9c | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])` |
| `ldr` at 0xc0ba0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0bac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0bb4 | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])-32` |
| `ldr` at 0xc0bb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0bbc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0bc4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0bc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0bd0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc0bd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0be4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc0be8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0bf0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc0bf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0bf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0bfc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xc0c00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0c08 | store | 4 | `[sp-32] +0x0` | `([[sp-32]] + [sp-24])` |
| `ldr` at 0xc0c0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0c10 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldmdb` at 0xc0c18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0c18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0c18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc0a10 | `C == 0` | `not (C == 0)` |
| `bhi` at 0xc0b34 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0xc0ba8 | `([sp-40] + [sp-24]) <= 0x20 (unsigned, 32-bit)` | `([sp-40] + [sp-24]) > 0x20 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0c18 | `r0` | `[[sp-32]]` |

## `JTAG_Cfg_create`

- `JTAG_Cfg_create`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc607c to 0xc6274.
  Established. Two absolute word stores clear `0x27eba4` and set `0x115844` to `0x1` (`str` at
  0xc6150, `str` at 0xc615c). The earlier call to `0x1eed0` carries candidates `0x6`, `0x10a068`,
  `0x79`, `0x0` (`bl` at 0xc60c0). Four later word stores write zero at unknown targets (`str` at
  0xc61b4, `str` at 0xc61e0, `str` at 0xc620c, `str` at 0xc6238). Entry r0 being nonzero takes the
  longer path, while zero falls through to the return (`bne` at 0xc6168). The r0 return is `0x1`
  (`ldmdb` at 0xc6258). The cleared slots’ hardware significance is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc608c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc6090 | load | 4 | `abs 0xc625c` | `-` |
| `ldr` at 0xc6094 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc60a0 | load | 4 | `abs 0xc6260` | `-` |
| `str` at 0xc60a4 | store | 4 | `sp -0x24` | `0x10a050` |
| `ldr` at 0xc60a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc60ac | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xc60b4 | load | 4 | `abs 0xc6264` | `-` |
| `str` at 0xc60c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc60cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc60dc | load | 4 | `abs 0xc6268` | `-` |
| `ldr` at 0xc60e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6100 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc610c | load | 4 | `abs 0xc6268` | `-` |
| `ldr` at 0xc6110 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6130 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6138 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6140 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc6148 | load | 4 | `abs 0xc626c` | `-` |
| `str` at 0xc6150 | store | 4 | `abs 0x27eba4` | `0x0` |
| `ldr` at 0xc6154 | load | 4 | `abs 0xc6270` | `-` |
| `str` at 0xc615c | store | 4 | `abs 0x115844` | `0x1` |
| `ldr` at 0xc6160 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6170 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xc617c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc6180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6184 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6194 | load | 4 | `abs 0xc6268` | `-` |
| `ldr` at 0xc6198 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc61b4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc61b8 | load | 4 | `abs 0xc6268` | `-` |
| `ldr` at 0xc61bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc61e0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc61e4 | load | 4 | `abs 0xc6268` | `-` |
| `ldr` at 0xc61e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc620c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc6210 | load | 4 | `abs 0xc6268` | `-` |
| `ldr` at 0xc6214 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6238 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc623c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6244 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xc6250 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xc6254 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc6258 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6258 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc60c0 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x79` | `0x0` |
| `bl` at 0xc6134 | `(unknown callee) 0x18be4` | `?` | `0xc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc609c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bls` at 0xc60d4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xc6108 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc6168 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `blo` at 0xc618c | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6258 | `r0` | `0x1` |

## `JTAG_Cfg_info_init`

- `JTAG_Cfg_info_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6274 to 0xc65e8.
  Established. The word read at absolute `0x27eb98` controls an unsigned comparison: at most `0x2`
  takes the edge and larger values fall through (`ldr` at 0xc6290, `bls` at 0xc6298). A later call
  targets `0x18920` with candidates unknown, `0xc`, unknown, unknown (`bl` at 0xc6428); its result
  is stored as a word at an unknown target (`str` at 0xc6430). The initialization stores include a
  zero halfword and two zero bytes, each at an unknown target (`strh` at 0xc64bc, `strb` at 0xc64f8,
  `strb` at 0xc6534). The merged return and record layout remain unknown (`ldmdb` at 0xc65d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc6284 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc6288 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc628c | load | 4 | `abs 0xc65d4` | `-` |
| `ldr` at 0xc6290 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc629c | load | 4 | `abs 0xc65d8` | `-` |
| `str` at 0xc62a0 | store | 4 | `sp -0x30` | `0x10a084` |
| `ldr` at 0xc62a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc62a8 | store | 4 | `sp -0x2c` | `r0` |
| `ldr` at 0xc62ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc62b0 | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0xc62b8 | load | 4 | `abs 0xc65dc` | `-` |
| `ldr` at 0xc62c8 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc62cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc62ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc62f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc62fc | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc6300 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6324 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc6328 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc632c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc634c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6358 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc635c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc637c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6384 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc6388 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc63ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc63b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc63c0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xc63c8 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc63cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc63ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc63f8 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc63fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc641c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6430 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0xc6434 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6440 | load | 4 | `abs 0xc65e4` | `-` |
| `str` at 0xc6444 | store | 4 | `sp -0x30` | `0x10a0ac` |
| `ldr` at 0xc644c | load | 4 | `abs 0xc65dc` | `-` |
| `str` at 0xc6460 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc646c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc6470 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6474 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6484 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc6488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc64a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc64b0 | load | 4 | `(unknown)` | `-` |
| `strh` at 0xc64bc | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xc64c0 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc64c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc64e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc64ec | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc64f8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xc64fc | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc6500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6520 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6528 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc6534 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xc6538 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6540 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xc6548 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc654c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6568 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc656c | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc6570 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6594 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc6598 | load | 4 | `abs 0xc65e0` | `-` |
| `ldr` at 0xc659c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc65bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc65c0 | store | 4 | `(unknown)` | `[sp-28]` |
| `str` at 0xc65c8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xc65cc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc65d0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc65d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc65d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc65d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc62c4 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0xa1` | `0x0` |
| `bl` at 0xc6380 | `(unknown callee) 0x18be4` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xc6428 | `(unknown callee) 0x18920` | `?` | `0xc` | `?` | `?` |
| `bl` at 0xc6458 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a068` | `0xb5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc6298 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xc62f8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc6354 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc63b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc63f4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc643c | `? != 0x0` | `? == 0x0` |
| `blo` at 0xc647c | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc65d0 | `r0` | `?` |

## `JTAG_Cfg_calc_ir_vals`

- `JTAG_Cfg_calc_ir_vals`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc69e8 to 0xc6b70.
  Established. A halfword store writes `([sp-28]:2 & 0xffff)` at an unknown target (`strh` at
  0xc6ab0). A later word store has unknown target and value (`str` at 0xc6b30). The final flags test
  takes its bypass for `[0x27eb98] <= 0x3` unsigned and falls through above that value (`bls` at
  0xc6b40). The reached explicit-link call has unknown target with candidates `0x2843e0`, `0x4`,
  unknown, `[0x115850]` (`mov` at 0xc6b50). Neither the merged return nor the halfword’s device
  interpretation is established (`ldmdb` at 0xc6b54).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc69f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc69fc | load | 4 | `abs 0xc6b58` | `-` |
| `ldr` at 0xc6a00 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc6a0c | load | 4 | `abs 0xc6b5c` | `-` |
| `str` at 0xc6a10 | store | 4 | `sp -0x24` | `0x10a130` |
| `ldr` at 0xc6a14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6a18 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xc6a20 | load | 4 | `abs 0xc6b60` | `-` |
| `ldr` at 0xc6a24 | load | 4 | `abs 0xc6b64` | `-` |
| `str` at 0xc6a34 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xc6a3c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc6a40 | load | 4 | `abs 0xc6b68` | `-` |
| `ldr` at 0xc6a44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6a64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6a68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6a78 | load | 4 | `abs 0xc6b68` | `-` |
| `ldr` at 0xc6a7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6a9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6aa4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xc6aac | load | 2 | `sp -0x1c` | `-` |
| `strh` at 0xc6ab0 | store | 2 | `(unknown)` | `([sp-28]:2 & 0xffff)` |
| `ldr` at 0xc6ab4 | load | 4 | `abs 0xc6b68` | `-` |
| `ldr` at 0xc6ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6ad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6ae0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc6ae8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc6aec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6af4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc6af8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6b00 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc6b08 | load | 4 | `abs 0xc6b68` | `-` |
| `ldr` at 0xc6b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6b2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6b30 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc6b34 | load | 4 | `abs 0xc6b58` | `-` |
| `ldr` at 0xc6b38 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc6b44 | load | 4 | `abs 0xc6b6c` | `-` |
| `ldr` at 0xc6b48 | load | 4 | `abs 0x115850` | `-` |
| `ldmdb` at 0xc6b54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6b54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6b54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc6a2c | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x111` | `0x0` |
| `mov` at 0xc6b50 | `(unknown callee) (unknown target)` | `0x2843e0` | `0x4` | `?` | `[0x115850]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc6a08 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `blo` at 0xc6a70 | `C == 0` | `not (C == 0)` |
| `bls` at 0xc6b40 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6b54 | `r0` | `?` |

## `JTAG_Cfg_find_chip`

- `JTAG_Cfg_find_chip`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6d3c to 0xc6fd4.
  Established. The only call targets `0x1eed0` with candidates `0x6`, `0x10a068`, `0x173`, `0x0`
  (`bl` at 0xc6da4). Before it, entry r3 is saved as a word at `sp -0x20` (`str` at 0xc6d58). A
  later branch retains a signed less-than flags predicate with unknown operands, following its edge
  when that predicate holds and falling through otherwise (`blt` at 0xc6df8). The return reports
  `[sp-44]` in r0 (`ldmdb` at 0xc6fbc). Chip identification and matching criteria remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc6d4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc6d50 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc6d54 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc6d58 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc6d5c | load | 4 | `abs 0xc6fc0` | `-` |
| `ldr` at 0xc6d60 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc6d6c | load | 4 | `abs 0xc6fc4` | `-` |
| `str` at 0xc6d70 | store | 4 | `sp -0x40` | `0x10a1f8` |
| `ldr` at 0xc6d74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6d78 | store | 4 | `sp -0x3c` | `r0` |
| `ldr` at 0xc6d7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6d80 | store | 4 | `sp -0x38` | `r1` |
| `ldr` at 0xc6d84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6d88 | store | 4 | `sp -0x34` | `r2` |
| `ldr` at 0xc6d8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6d90 | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0xc6d98 | load | 4 | `abs 0xc6fc8` | `-` |
| `ldr` at 0xc6d9c | load | 4 | `abs 0xc6fcc` | `-` |
| `ldr` at 0xc6da8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6db4 | load | 4 | `abs 0xc6fd0` | `-` |
| `ldr` at 0xc6db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6dd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc6de8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6dfc | load | 4 | `abs 0xc6fd0` | `-` |
| `ldr` at 0xc6e00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6e20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6e24 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc6e38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6e4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6e50 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc6e54 | load | 4 | `abs 0xc6fd0` | `-` |
| `ldr` at 0xc6e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6e78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6e7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6e8c | load | 4 | `abs 0xc6fd0` | `-` |
| `ldr` at 0xc6e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6eb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6eb8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc6ec0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc6ec4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6ed0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc6ed4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6edc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc6ee4 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc6ef0 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc6ef8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc6efc | load | 4 | `abs 0xc6fd0` | `-` |
| `ldr` at 0xc6f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6f20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6f24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6f34 | load | 4 | `abs 0xc6fd0` | `-` |
| `ldr` at 0xc6f38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6f58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6f60 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc6f68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc6f6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6f78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc6f7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6f88 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc6f8c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6f94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc6f9c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc6fa0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc6fa8 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc6fb4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6fb8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xc6fbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6fbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6fbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc6da4 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x173` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc6d68 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xc6db0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xc6de0 | `? != 0x0` | `? == 0x0` |
| `blt` at 0xc6df8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0xc6e2c | `C == 1` | `not (C == 1)` |
| `bne` at 0xc6e48 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xc6e84 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc6ecc | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xc6f2c | `C == 0` | `not (C == 0)` |
| `bne` at 0xc6f74 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc6f84 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6fbc | `r0` | `[sp-44]` |

## `JTAG_Comm_read`

- `JTAG_Comm_read`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7774 to 0xc78fc.
  Established. The first call targets `0xc9f88` with candidates entry r0, `[sp-48]`, `sp-44`,
  `sp-44` (`bl` at 0xc77d8). A zero result takes the edge past the two intervening calls and nonzero
  falls through to `0x1e71c` and `0x18c74` (`beq` at 0xc77ec, `bl` at 0xc77f4, `bl` at 0xc7808).
  The later call to `0xc1c9c` uses candidates `0x0`, `0x40`, `sp-44`, `sp-44` (`bl` at 0xc7828). The
  byte stores include an unknown value at `([sp-24] + [sp-32]) +0x0` (`strb` at 0xc78ac). Transfer
  direction at a physical interface remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7784 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc7788 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc778c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc7790 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc77a4 | store | 4 | `sp -0x30` | `0xc` |
| `str` at 0xc77b0 | store | 4 | `sp -0x30` | `0x11` |
| `str` at 0xc77c0 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xc77c4 | load | 4 | `abs 0xc78f4` | `-` |
| `str` at 0xc77c8 | store | 4 | `sp -0x38` | `0x10a378` |
| `ldr` at 0xc77cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc77d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc77e0 | store | 4 | `sp -0x24` | `ret(0xc9f88)` |
| `ldr` at 0xc77e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc77f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc77fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7800 | load | 4 | `abs 0xc78f8` | `-` |
| `ldr` at 0xc780c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc7810 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0xc7830 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc7834 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc784c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc7864 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc7868 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc7884 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc788c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc7890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7894 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc78a8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc78ac | store | 1 | `([sp-24] + [sp-32]) +0x0` | `?` |
| `ldr` at 0xc78c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc78d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc78d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc78dc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xc78e8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc78ec | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xc78f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc78f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc78f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc77d8 | `(unknown callee) 0xc9f88` | `r0` | `[sp-48]` | `sp-44` | `sp-44` |
| `bl` at 0xc77f4 | `(unknown callee) 0x1e71c` | `ret(0xc9f88)` | `?` | `?` | `ret(0xc9f88)` |
| `bl` at 0xc7808 | `(unknown callee) 0x18c74` | `[sp-28]` | `0x10a388` | `ret(0x1e71c)` | `ret(0x1e71c)` |
| `bl` at 0xc7828 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc779c | `? != 0x6` | `? == 0x6` |
| `beq` at 0xc77ec | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |
| `ble` at 0xc783c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc78f0 | `r0` | `?` |

## `JTAG_Comm_write`

- `JTAG_Comm_write`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc78fc to 0xc7a80.
  Established. Two word stores from one multiple-store instruction write zero at unknown targets
  (`stm` at 0xc792c). The call to `0xc1c9c` carries candidates `0x0`, `0x40`, `sp-44`, `sp-44` (`bl`
  at 0xc7a04). The later call to `0xc9f88` has `[sp-20]`, `[sp-48]`, `sp-44`, `0x0` (`bl` at
  0xc7a28); its zero result takes the bypass and nonzero falls through to the following calls
  (`beq` at 0xc7a3c). The return value and command meanings remain unknown (`ldmdb` at 0xc7a74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc790c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc7910 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc7914 | store | 4 | `sp -0x1c` | `r2` |
| `stm` at 0xc792c | store | 4 | `(unknown)` | `0x0` |
| `stm` at 0xc792c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc7930 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc7944 | store | 4 | `sp -0x30` | `0xd` |
| `str` at 0xc7950 | store | 4 | `sp -0x30` | `0x12` |
| `str` at 0xc7958 | store | 4 | `sp -0x20` | `0x3` |
| `ldr` at 0xc795c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7984 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc7990 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7994 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc79a0 | load | 1 | `([sp-24] + [sp-32]) +0x4` | `-` |
| `str` at 0xc79a8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc79c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc79cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc79d0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc79d8 | load | 1 | `([sp-24] + [sp-32]) +0x0` | `-` |
| `str` at 0xc79e0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc79e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc79ec | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `str` at 0xc7a10 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xc7a14 | load | 4 | `abs 0xc7a78` | `-` |
| `str` at 0xc7a18 | store | 4 | `sp -0x38` | `0x10a3b4` |
| `ldr` at 0xc7a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7a20 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc7a30 | store | 4 | `sp -0x24` | `ret(0xc9f88)` |
| `ldr` at 0xc7a34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc7a40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc7a4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7a50 | load | 4 | `abs 0xc7a7c` | `-` |
| `ldr` at 0xc7a5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc7a60 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0xc7a6c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc7a70 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xc7a74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7a74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7a74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7a04 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-44` | `sp-44` |
| `bl` at 0xc7a28 | `(unknown callee) 0xc9f88` | `[sp-20]` | `[sp-48]` | `sp-44` | `0x0` |
| `bl` at 0xc7a44 | `(unknown callee) 0x1e71c` | `ret(0xc9f88)` | `?` | `?` | `ret(0xc9f88)` |
| `bl` at 0xc7a58 | `(unknown callee) 0x18c74` | `[sp-28]` | `0x10a3c4` | `ret(0x1e71c)` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc793c | `? != 0x6` | `? == 0x6` |
| `bge` at 0xc7964 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc7a3c | `ret(0xc9f88) == 0x0` | `ret(0xc9f88) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7a74 | `r0` | `?` |

## `JTAG_Cfg_Setup`

- `JTAG_Cfg_Setup`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7720 to 0xc7774.
  Established. Three word stores write `0xc72d0`, `0xc6b70`, `0xc7014` at absolute `0x115848`,
  `0x11584c`, `0x115850`, respectively (`str` at 0xc7734, `str` at 0xc7740, `str` at 0xc774c). These
  addresses are stored values rather than call targets in this body. No calls or conditional
  branches are reported. r0 is set to `0x0` and returned (`mov` at 0xc7754, `ldmdb` at 0xc7758). The
  slots’ consumer is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc772c | load | 4 | `abs 0xc775c` | `-` |
| `ldr` at 0xc7730 | load | 4 | `abs 0xc7760` | `-` |
| `str` at 0xc7734 | store | 4 | `abs 0x115848` | `0xc72d0` |
| `ldr` at 0xc7738 | load | 4 | `abs 0xc7764` | `-` |
| `ldr` at 0xc773c | load | 4 | `abs 0xc7768` | `-` |
| `str` at 0xc7740 | store | 4 | `abs 0x11584c` | `0xc6b70` |
| `ldr` at 0xc7744 | load | 4 | `abs 0xc776c` | `-` |
| `ldr` at 0xc7748 | load | 4 | `abs 0xc7770` | `-` |
| `str` at 0xc774c | store | 4 | `abs 0x115850` | `0xc7014` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7758 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7758 | `r0` | `0x0` |

## `JTAG_Cfg_L3SetCfg_f`

- `JTAG_Cfg_L3SetCfg_f`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc72d0 to 0xc7720.
  Established. The calls include `0xc6274` and `0xc69e8`, each with candidates `[0x27eba4]`,
  unknown, unknown, `0x27eba4` (`bl` at 0xc7478, `bl` at 0xc7684). A separate call to `0xca3ec` has
  candidates `0x3`, unknown, unknown, unknown (`bl` at 0xc761c). A byte store writes `0x3` at an
  unknown target (`strb` at 0xc7668). The last call targets `0xc91a0` with candidates `0x2843e0`,
  `0x14`, unknown, `0x0` (`bl` at 0xc76e8). The physical configuration change and merged return
  remain unknown (`ldmdb` at 0xc76f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc72e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc72e4 | load | 4 | `abs 0xc76fc` | `-` |
| `ldr` at 0xc72e8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc72f4 | load | 4 | `abs 0xc7700` | `-` |
| `str` at 0xc72f8 | store | 4 | `sp -0x3c` | `0x10a2f0` |
| `ldr` at 0xc7300 | load | 4 | `abs 0xc7704` | `-` |
| `ldr` at 0xc7314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc731c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc7320 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc7328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7330 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc7334 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc7338 | load | 4 | `abs 0xc76fc` | `-` |
| `ldr` at 0xc733c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc7348 | load | 4 | `abs 0xc7708` | `-` |
| `str` at 0xc734c | store | 4 | `sp -0x3c` | `0x10a304` |
| `ldr` at 0xc7350 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7354 | load | 4 | `abs 0x27eba4` | `-` |
| `str` at 0xc7358 | store | 4 | `sp -0x38` | `[0x27eba4]` |
| `ldr` at 0xc735c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc7360 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc7364 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7368 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc7370 | load | 4 | `abs 0xc7704` | `-` |
| `ldr` at 0xc7374 | load | 4 | `abs 0xc7710` | `-` |
| `ldr` at 0xc7380 | load | 4 | `abs 0xc76fc` | `-` |
| `ldr` at 0xc7384 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc7390 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc7394 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7398 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc73b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc73b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc73c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc73cc | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc73d0 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc73d4 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc73f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc73f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc7404 | load | 4 | `abs 0xc7718` | `-` |
| `str` at 0xc7408 | store | 4 | `sp -0x3c` | `0x10a334` |
| `ldr` at 0xc740c | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7410 | load | 4 | `abs 0x27eba4` | `-` |
| `str` at 0xc7414 | store | 4 | `sp -0x38` | `[0x27eba4]` |
| `ldr` at 0xc7418 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc741c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc7420 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7424 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc7428 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc742c | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7430 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc7450 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc7454 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc745c | load | 4 | `abs 0xc7704` | `-` |
| `ldr` at 0xc7460 | load | 4 | `abs 0xc771c` | `-` |
| `ldr` at 0xc746c | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7470 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc7474 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7488 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc748c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7490 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc7498 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xc74a0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc74a8 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc74ac | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc74b0 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc74c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc74cc | store | 4 | `(unknown)` | `[sp-24]` |
| `strb` at 0xc74d4 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0xc74d8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc74dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc74ec | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc74f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7504 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc7508 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc750c | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc7510 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7514 | load | 4 | `abs 0x27eba4` | `-` |
| `ldrb` at 0xc7534 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc753c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc7544 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc754c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc7550 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc7554 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7558 | load | 4 | `abs 0x27eba4` | `-` |
| `ldrb` at 0xc7578 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc7580 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc7588 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc758c | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc7590 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc7594 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7598 | load | 4 | `abs 0x27eba4` | `-` |
| `ldrb` at 0xc75b8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc75c0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc75c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc75d4 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc75d8 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc75dc | load | 4 | `abs 0x27eba4` | `-` |
| `ldrb` at 0xc75fc | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc7604 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc760c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc762c | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc7630 | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7634 | load | 4 | `abs 0x27eba4` | `-` |
| `ldrb` at 0xc7654 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc765c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc7668 | store | 1 | `(unknown)` | `0x3` |
| `ldrb` at 0xc766c | load | 1 | `sp -0x22` | `-` |
| `strb` at 0xc7674 | store | 1 | `sp -0x22` | `([sp-34]:1+1 & 0xff)` |
| `ldr` at 0xc767c | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7680 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc7688 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc768c | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc7690 | load | 4 | `abs 0x27eba4` | `-` |
| `strb` at 0xc76b4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc76b8 | load | 4 | `abs 0xc7714` | `-` |
| `ldr` at 0xc76bc | load | 4 | `abs 0xc770c` | `-` |
| `ldr` at 0xc76c0 | load | 4 | `abs 0x27eba4` | `-` |
| `str` at 0xc76e4 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xc76f0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xc76f4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xc76f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc76f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc76f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc730c | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x1cc` | `0x0` |
| `bl` at 0xc737c | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x1d5` | `0x0` |
| `bl` at 0xc7468 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x1da` | `0x0` |
| `bl` at 0xc7478 | `(unknown callee) 0xc6274` | `[0x27eba4]` | `?` | `?` | `0x27eba4` |
| `bl` at 0xc761c | `(unknown callee) 0xca3ec` | `0x3` | `?` | `?` | `?` |
| `bl` at 0xc7684 | `(unknown callee) 0xc69e8` | `[0x27eba4]` | `?` | `?` | `0x27eba4` |
| `bl` at 0xc76e8 | `(unknown callee) 0xc91a0` | `0x2843e0` | `0x14` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc72f0 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0xc7344 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0xc738c | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0xc73bc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc73c8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc7400 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc7484 | `ret(0xc6274) != 0x0` | `ret(0xc6274) == 0x0` |
| `blo` at 0xc74e4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc75d0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc7614 | `? != 0x8` | `? == 0x8` |
| `beq` at 0xc7628 | `ret(0xca3ec) == 0x0` | `ret(0xca3ec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc76f8 | `r0` | `?` |

## `JTAG_Cfg_L3SetCfg_stub`

- `JTAG_Cfg_L3SetCfg_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7a80 to 0xc7aa0.
  Established. Entry r0 is saved as a word at `sp -0x14` (`str` at 0xc7a90). The body sets r0 to
  `0x1` before returning it (`mov` at 0xc7a98, `ldmdb` at 0xc7a9c). No calls or conditional branches
  are reported. The returned value’s external interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7a90 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xc7a9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7a9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7a9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7a9c | `r0` | `0x1` |

## `JTAG_Cfg_pid`

- `JTAG_Cfg_pid`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6fd4 to 0xc7014.
  Established. The entry r0 word is saved at `sp -0x14` (`str` at 0xc6fe4). The later word load has
  an unknown effective target (`ldr` at 0xc7004); its value is moved to r0, whose reported return is
  unknown (`mov` at 0xc7008, `ldmdb` at 0xc700c). No calls are reported. A process-identifier role
  is not established by the symbol name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc6fe4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc6fe8 | load | 4 | `abs 0xc7010` | `-` |
| `ldr` at 0xc6fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7004 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xc700c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc700c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc700c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc700c | `r0` | `?` |
