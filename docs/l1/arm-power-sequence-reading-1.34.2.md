# ARM L1 power sequence and reading entry paths in 1.34.2

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
An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/power-sequence-reading-facts.txt) retain all tool sections.

## `powerUp`

- `powerUp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39a3c to 0x3a458.
  Established. The initial known paths include word clears at 0x119dd4 and 0x119dd8, followed by a
  word store of 0x1 at 0x119de0 (`str` at 0x39bf8, `str` at 0x39c04, `str` at 0x39c24). Later word
  stores to 0x119de0 write 0x0 at three separate sites (`str` at 0x39ce4, `str` at 0x39fa4, `str` at
  0x3a0bc). Another known path writes 0x1 at 0x119ddc (`str` at 0x3a248). Five computed transfers
  remain unresolved (`mov` at 0x39aa0, `ldrls` at 0x39b00, `mov` at 0x3a1dc, `ldrls` at 0x3a264,
  `mov` at 0x3a408). One later `beq` takes the zero-word path from 0x27ee58 past the final computed
  transfer, then a word store clears 0x119de0 (`beq` at 0x3a3f8, `mov` at 0x3a408, `str` at
  0x3a414). The known call list includes 0x3ed00 with first candidates 0xff and 0x3, and 0x5edf8
  with first candidate 0x1 (`bl` at 0x3a210, `bl` at 0x3a33c). The complete physical power-up
  sequence, global meanings and additional transfer effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39a4c | store | 4 | `sp -0x28` | `r0` |
| `str` at 0x39a50 | store | 4 | `sp -0x2c` | `r1` |
| `str` at 0x39a58 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x39a60 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x39a64 | load | 4 | `abs 0x3a158` | `-` |
| `ldm` at 0x39a6c | load | 4 | `abs 0xe162c` | `-` |
| `ldm` at 0x39a6c | load | 4 | `abs 0xe1630` | `-` |
| `ldm` at 0x39a6c | load | 4 | `abs 0xe1634` | `-` |
| `ldm` at 0x39a6c | load | 4 | `abs 0xe1638` | `-` |
| `stm` at 0x39a70 | store | 4 | `sp -0x58` | `[0xe162c]` |
| `stm` at 0x39a70 | store | 4 | `sp -0x54` | `[0xe1630]` |
| `stm` at 0x39a70 | store | 4 | `sp -0x50` | `[0xe1634]` |
| `stm` at 0x39a70 | store | 4 | `sp -0x4c` | `[0xe1638]` |
| `strb` at 0x39a7c | store | 1 | `sp -0x5d` | `(ret(0x2b31c) & 0xff)` |
| `ldr` at 0x39a84 | load | 4 | `abs 0x3a1b0` | `-` |
| `ldr` at 0x39a88 | load | 4 | `abs 0x27eec0` | `-` |
| `ldr` at 0x39a94 | load | 4 | `abs 0x3a1b0` | `-` |
| `ldr` at 0x39a98 | load | 4 | `abs 0x27eec0` | `-` |
| `str` at 0x39ae0 | store | 4 | `sp -0x3c` | `ret(0x446bc)` |
| `ldr` at 0x39ae4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x39af0 | load | 4 | `sp -0x3c` | `-` |
| `ldrls` at 0x39b00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39b60 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x39b6c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x39b70 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x39b78 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39b80 | store | 4 | `sp -0x78` | `ret(0x437dc)` |
| `ldr` at 0x39b84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39b90 | load | 4 | `abs 0x3a17c` | `-` |
| `str` at 0x39b94 | store | 4 | `sp -0x7c` | `0xe16f4` |
| `ldr` at 0x39b9c | load | 4 | `abs 0x3a180` | `-` |
| `str` at 0x39ba0 | store | 4 | `sp -0x7c` | `0xe16f8` |
| `ldr` at 0x39ba4 | load | 4 | `abs 0x3a184` | `-` |
| `ldr` at 0x39ba8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x39bac | load | 4 | `sp -0x78` | `-` |
| `str` at 0x39bc0 | store | 4 | `sp -0x38` | `ret(0x2c4a4)` |
| `str` at 0x39bd4 | store | 4 | `sp -0x38` | `ret(0x2d164)` |
| `ldr` at 0x39bdc | load | 4 | `abs 0x3a188` | `-` |
| `str` at 0x39bec | store | 4 | `sp -0x38` | `ret(0x2bfac)` |
| `ldr` at 0x39bf0 | load | 4 | `abs 0x3a18c` | `-` |
| `str` at 0x39bf8 | store | 4 | `abs 0x119dd4` | `0x0` |
| `ldr` at 0x39bfc | load | 4 | `abs 0x3a190` | `-` |
| `str` at 0x39c04 | store | 4 | `abs 0x119dd8` | `0x0` |
| `str` at 0x39c14 | store | 4 | `sp -0x34` | `ret(0x433b0)` |
| `ldr` at 0x39c1c | load | 4 | `abs 0x3a1b4` | `-` |
| `str` at 0x39c24 | store | 4 | `abs 0x119de0` | `0x1` |
| `ldrb` at 0x39c28 | load | 1 | `sp -0x5d` | `-` |
| `str` at 0x39c38 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x39c3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39c4c | load | 4 | `abs 0x3a194` | `-` |
| `ldr` at 0x39c50 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x39c54 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x39c5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39c68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39c7c | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x39c80 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x39c88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39c94 | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x39c98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39cac | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x39cc0 | store | 4 | `sp -0x38` | `ret(0x3dbec)` |
| `ldr` at 0x39cc4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39cdc | load | 4 | `abs 0x3a1b4` | `-` |
| `str` at 0x39ce4 | store | 4 | `abs 0x119de0` | `0x0` |
| `ldr` at 0x39ce8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x39cec | store | 4 | `sp -0x74` | `[sp-56]` |
| `ldr` at 0x39cfc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x39d04 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x39d10 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x39d4c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x39d50 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x39d60 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x39d78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x39d84 | store | 4 | `sp -0x68` | `ret(0x3d538)` |
| `ldr` at 0x39d88 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x39da4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39db0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x39dc0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x39dd0 | load | 4 | `sp -0x68` | `-` |
| `ldm` at 0x39dec | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x39dec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39e3c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x39e4c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x39e5c | load | 4 | `sp -0x68` | `-` |
| `ldm` at 0x39e78 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x39e78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39ecc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x39ed4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x39edc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x39ee0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x39ee8 | store | 4 | `sp -0x5c` | `[sp-92]+1` |
| `ldr` at 0x39ef4 | load | 4 | `abs 0x3a19c` | `-` |
| `ldr` at 0x39ef8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39efc | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x39f0c | load | 4 | `abs 0x3a194` | `-` |
| `ldr` at 0x39f10 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x39f14 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x39f1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39f28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39f3c | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x39f40 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x39f48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39f54 | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x39f58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39f6c | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x39f80 | store | 4 | `sp -0x38` | `ret(0x3dbec)` |
| `ldr` at 0x39f84 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39f9c | load | 4 | `abs 0x3a1b4` | `-` |
| `str` at 0x39fa4 | store | 4 | `abs 0x119de0` | `0x0` |
| `ldr` at 0x39fa8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x39fac | store | 4 | `sp -0x74` | `[sp-56]` |
| `ldr` at 0x39fb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x39fc8 | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x39fcc | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x39fd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39ff4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x39ffc | store | 4 | `sp -0x40` | `?` |
| `str` at 0x3a008 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x3a00c | load | 4 | `abs 0x3a19c` | `-` |
| `ldr` at 0x3a010 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3a014 | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3a024 | load | 4 | `abs 0x3a194` | `-` |
| `ldr` at 0x3a028 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3a02c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3a034 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a040 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3a054 | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x3a058 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3a060 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a06c | load | 4 | `abs 0x3a198` | `-` |
| `ldr` at 0x3a070 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3a084 | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x3a098 | store | 4 | `sp -0x38` | `ret(0x3dbec)` |
| `ldr` at 0x3a09c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3a0b4 | load | 4 | `abs 0x3a1b4` | `-` |
| `str` at 0x3a0bc | store | 4 | `abs 0x119de0` | `0x0` |
| `ldr` at 0x3a0c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3a0c4 | store | 4 | `sp -0x74` | `[sp-56]` |
| `ldrb` at 0x3a0cc | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x3a0d8 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3a0f0 | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x3a0fc | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3a114 | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x3a120 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3a140 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3a148 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3a1c0 | load | 4 | `abs 0x3a1a0` | `-` |
| `ldr` at 0x3a1c4 | load | 4 | `abs 0x281140` | `-` |
| `ldr` at 0x3a1d0 | load | 4 | `abs 0x3a1a0` | `-` |
| `ldr` at 0x3a1d4 | load | 4 | `abs 0x281140` | `-` |
| `str` at 0x3a218 | store | 4 | `sp -0x38` | `ret(0x3ed00)` |
| `ldr` at 0x3a21c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3a228 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a234 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3a238 | store | 4 | `sp -0x34` | `ret(0x3ed00)` |
| `ldr` at 0x3a240 | load | 4 | `abs 0x3a1a4` | `-` |
| `str` at 0x3a248 | store | 4 | `abs 0x119ddc` | `0x1` |
| `ldr` at 0x3a24c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3a258 | load | 1 | `sp -0x5d` | `-` |
| `ldrls` at 0x3a264 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3a308 | store | 4 | `sp -0x38` | `ret(0x43298)` |
| `ldr` at 0x3a30c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3a318 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a324 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3a328 | store | 4 | `sp -0x34` | `ret(0x43298)` |
| `ldr` at 0x3a32c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a344 | load | 4 | `abs 0x3a1a8` | `-` |
| `str` at 0x3a354 | store | 4 | `sp -0x38` | `ret(0x2bfac)` |
| `ldr` at 0x3a360 | load | 4 | `abs 0x3a1ac` | `-` |
| `str` at 0x3a370 | store | 4 | `sp -0x38` | `ret(0x2bfac)` |
| `ldrb` at 0x3a380 | load | 1 | `sp -0x5d` | `-` |
| `str` at 0x3a384 | store | 4 | `sp -0x80` | `[sp-93]:1` |
| `ldr` at 0x3a388 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x3a394 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x3a3a0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x3a3b0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x3a3cc | store | 4 | `sp -0x38` | `ret(0x43298)` |
| `ldr` at 0x3a3d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3a3e8 | store | 4 | `sp -0x38` | `ret(0x433b0)` |
| `ldr` at 0x3a3ec | load | 4 | `abs 0x3a1b0` | `-` |
| `ldr` at 0x3a3f0 | load | 4 | `abs 0x27ee58` | `-` |
| `ldr` at 0x3a3fc | load | 4 | `abs 0x3a1b0` | `-` |
| `ldr` at 0x3a400 | load | 4 | `abs 0x27ee58` | `-` |
| `ldr` at 0x3a40c | load | 4 | `abs 0x3a1b4` | `-` |
| `str` at 0x3a414 | store | 4 | `abs 0x119de0` | `0x0` |
| `ldr` at 0x3a418 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3a42c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3a430 | store | 4 | `sp -0x84` | `[sp-52]` |
| `ldr` at 0x3a434 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a440 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3a444 | store | 4 | `sp -0x84` | `[sp-60]` |
| `ldr` at 0x3a448 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x3a44c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x3a450 | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3a454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39a74 | `(unknown callee) 0x2b31c` | `[0xe162c]` | `[0xe1630]` | `[0xe1634]` | `[0xe1638]` |
| `bl` at 0x39a80 | `(unknown callee) 0x2c470` | `ret(0x2b31c)` | `?` | `?` | `ret(0x2b31c)` |
| `bl` at 0x39ad8 | `(unknown callee) 0x446bc` | `ret(0x2c470)` | `?` | `?` | `[0x27eec0]` |
| `bl` at 0x39b58 | `(unknown callee) 0x2c4a4` | `0x2` | `?` | `?` | `ret(0x446bc)+1704` |
| `bl` at 0x39b68 | `(unknown callee) 0x2bfac` | `0x32` | `[sp-112]` | `0x2` | `?` |
| `bl` at 0x39b7c | `(unknown callee) 0x437dc` | `[sp-44]` | `?` | `?` | `ret(0x446bc)` |
| `bl` at 0x39bb0 | `(unknown callee) 0x1dba4` | `0xe1700` | `[sp-124]` | `ret(0x437dc)` | `?` |
| `bl` at 0x39bb8 | `(unknown callee) 0x2c4a4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x39bcc | `(unknown callee) 0x2d164` | `0x1` | `0x0` | `?` | `ret(0x2c4a4)` |
| `bl` at 0x39be4 | `(unknown callee) 0x2bfac` | `0x32` | `0xe1710` | `0x1` | `ret(0x2d164)` |
| `bl` at 0x39c0c | `(unknown callee) 0x433b0` | `0x5` | `?` | `0x119dd8` | `0x0` |
| `bl` at 0x39c18 | `(unknown callee) 0x6b9b8` | `ret(0x433b0)` | `?` | `?` | `ret(0x433b0)` |
| `bl` at 0x39cb8 | `(unknown callee) 0x3dbec` | `?` | `0x281120` | `?` | `?` |
| `bl` at 0x39cd4 | `(unknown callee) 0x3a458` | `0x5` | `?` | `?` | `ret(0x3dbec)` |
| `bl` at 0x39cd8 | `(unknown callee) 0x6b9d0` | `ret(0x3a458)` | `?` | `?` | `?` |
| `bl` at 0x39cf8 | `(unknown callee) 0x3e294` | `0x32` | `?` | `?` | `ret(0x3dbec)` |
| `bl` at 0x39d28 | `(unknown callee) 0xb7a78` | `0x4` | `0x40` | `?` | `?` |
| `bl` at 0x39d40 | `(unknown callee) 0x3e294` | `0x14` | `?` | `?` | `ret(0xb7a78)` |
| `bl` at 0x39d7c | `(unknown callee) 0x3d538` | `?` | `sp-36` | `?` | `?` |
| `bl` at 0x39dac | `(unknown callee) 0x3cd7c` | `?` | `0x0` | `sp-100` | `?` |
| `bl` at 0x39db4 | `(unknown callee) 0xd5c38` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x39dc4 | `(unknown callee) 0xd5c38` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x39dd4 | `(unknown callee) 0xd5c38` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x39df0 | `(unknown callee) 0xd5c84` | `ret(0xd5c38)` | `?` | `?` | `?` |
| `bl` at 0x39e0c | `(unknown callee) 0xd58fc` | `ret(0xd5c38)` | `?` | `ret(0xd5c84)` | `?` |
| `bl` at 0x39e28 | `(unknown callee) 0xd6148` | `ret(0xd5c38)` | `?` | `ret(0xd58fc)` | `?` |
| `bl` at 0x39e40 | `(unknown callee) 0xd5c38` | `[sp-100]` | `?` | `?` | `ret(0xd6148)` |
| `bl` at 0x39e50 | `(unknown callee) 0xd5c38` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x39e60 | `(unknown callee) 0xd5c38` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x39e7c | `(unknown callee) 0xd5c84` | `ret(0xd5c38)` | `?` | `?` | `?` |
| `bl` at 0x39e98 | `(unknown callee) 0xd5900` | `ret(0xd5c38)` | `?` | `ret(0xd5c84)` | `?` |
| `bl` at 0x39eb4 | `(unknown callee) 0xd6150` | `ret(0xd5c38)` | `?` | `ret(0xd5900)` | `?` |
| `bl` at 0x39f78 | `(unknown callee) 0x3dbec` | `?` | `0x281120` | `?` | `?` |
| `bl` at 0x39f94 | `(unknown callee) 0x3a458` | `0x5` | `?` | `?` | `ret(0x3dbec)` |
| `bl` at 0x39f98 | `(unknown callee) 0x6b9d0` | `ret(0x3a458)` | `?` | `?` | `?` |
| `bl` at 0x39fe8 | `(unknown callee) 0xb790c` | `0x1` | `0x80` | `?` | `?` |
| `bl` at 0x39ff0 | `(unknown callee) 0x3e294` | `0x32` | `?` | `?` | `?` |
| `bl` at 0x3a090 | `(unknown callee) 0x3dbec` | `?` | `0x281120` | `?` | `?` |
| `bl` at 0x3a0ac | `(unknown callee) 0x3a458` | `0x5` | `?` | `?` | `ret(0x3dbec)` |
| `bl` at 0x3a0b0 | `(unknown callee) 0x6b9d0` | `ret(0x3a458)` | `?` | `?` | `?` |
| `bl` at 0x3a0e8 | `(unknown callee) 0x3e294` | `0xfa` | `?` | `?` | `[sp-64]` |
| `bl` at 0x3a10c | `(unknown callee) 0x3e294` | `0x64` | `?` | `?` | `[sp-64]` |
| `bl` at 0x3a130 | `(unknown callee) 0x3e294` | `0x32` | `?` | `?` | `[sp-64]` |
| `bl` at 0x3a13c | `(unknown callee) 0x3e294` | `0x19` | `?` | `?` | `?` |
| `bl` at 0x3a1bc | `(unknown callee) 0x3e294` | `0x64` | `?` | `?` | `[0x281100]` |
| `bl` at 0x3a210 | `(unknown callee) 0x3ed00` | `0xff` | `0x3` | `?` | `[0x281140]` |
| `bl` at 0x3a23c | `(unknown callee) 0x6b9d0` | `ret(0x3ed00)` | `?` | `?` | `?` |
| `bl` at 0x3a300 | `(unknown callee) 0x43298` | `0x5` | `?` | `0x1` | `[sp-93]:1-48` |
| `bl` at 0x3a33c | `(unknown callee) 0x5edf8` | `0x1` | `?` | `?` | `[sp-52]` |
| `bl` at 0x3a34c | `(unknown callee) 0x2bfac` | `0x32` | `0xe171c` | `0x0` | `?` |
| `bl` at 0x3a368 | `(unknown callee) 0x2bfac` | `0x32` | `0xe1728` | `0x2` | `[sp-52]` |
| `bl` at 0x3a374 | `(unknown callee) 0x2e328` | `ret(0x2bfac)` | `?` | `?` | `ret(0x2bfac)` |
| `bl` at 0x3a378 | `(unknown callee) 0x2e40c` | `ret(0x2e328)` | `?` | `?` | `?` |
| `bl` at 0x3a37c | `(unknown callee) 0x2dcf4` | `ret(0x2e40c)` | `?` | `?` | `?` |
| `bl` at 0x3a3c4 | `(unknown callee) 0x43298` | `0x5` | `[sp-93]:1` | `?` | `[sp-93]:1` |
| `bl` at 0x3a3e0 | `(unknown callee) 0x433b0` | `0x5` | `?` | `?` | `[sp-40]` |
| `bl` at 0x3a428 | `(unknown callee) 0x6bab8` | `0x0` | `?` | `0x0` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x39a90 | `[0x27eec0] == 0x0` | `[0x27eec0] != 0x0` |
| `beq` at 0x39aec | `ret(0x446bc) == 0x0` | `ret(0x446bc) != 0x0` |
| `ldrls` at 0x39b00 | `ret(0x446bc)+1704 <= 0x4 (unsigned, 32-bit)` | `ret(0x446bc)+1704 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x39b8c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x39c30 | `[sp-93]:1 != 0x34` | `[sp-93]:1 == 0x34` |
| `ble` at 0x39c44 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x39c64 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x39c90 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x39ccc | `ret(0x3dbec) == 0x0` | `ret(0x3dbec) != 0x0` |
| `bne` at 0x39d18 | `[sp-72] != 0xf` | `[sp-72] == 0xf` |
| `beq` at 0x39d34 | `ret(0xb7a78) == 0x0` | `ret(0xb7a78) != 0x0` |
| `ble` at 0x39d58 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0x39e34 | `ret(0xd6148) > 0x0 (signed, 32-bit)` | `ret(0xd6148) <= 0x0 (signed, 32-bit)` |
| `blt` at 0x39ec0 | `ret(0xd6150) < 0x0 (signed, 32-bit)` | `ret(0xd6150) >= 0x0 (signed, 32-bit)` |
| `blt` at 0x39f04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x39f24 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x39f50 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x39f8c | `ret(0x3dbec) == 0x0` | `ret(0x3dbec) != 0x0` |
| `bne` at 0x39fdc | `? != 0xb` | `? == 0xb` |
| `blt` at 0x3a01c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3a03c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3a068 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3a0a4 | `ret(0x3dbec) == 0x0` | `ret(0x3dbec) != 0x0` |
| `bne` at 0x3a0d4 | `[sp-93]:1 != 0x41` | `[sp-93]:1 == 0x41` |
| `bne` at 0x3a0e0 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0x3a0f8 | `[sp-93]:1 != 0x36` | `[sp-93]:1 == 0x36` |
| `bne` at 0x3a104 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0x3a11c | `[sp-93]:1 != 0x36` | `[sp-93]:1 == 0x36` |
| `bne` at 0x3a128 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `beq` at 0x3a1cc | `[0x281140] == 0x0` | `[0x281140] != 0x0` |
| `beq` at 0x3a224 | `ret(0x3ed00) == 0x0` | `ret(0x3ed00) != 0x0` |
| `bne` at 0x3a230 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x3a254 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `ldrls` at 0x3a264 | `[sp-93]:1-48 <= 0x21 (unsigned, 32-bit)` | `[sp-93]:1-48 > 0x21 (unsigned, 32-bit)` |
| `beq` at 0x3a314 | `ret(0x43298) == 0x0` | `ret(0x43298) != 0x0` |
| `bne` at 0x3a320 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `bne` at 0x3a334 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x3a390 | `[sp-93]:1 == 0x3d` | `[sp-93]:1 != 0x3d` |
| `bgt` at 0x3a39c | `[sp-93]:1 > 0x3d (signed, 32-bit)` | `[sp-93]:1 <= 0x3d (signed, 32-bit)` |
| `beq` at 0x3a3a8 | `[sp-93]:1 == 0x23` | `[sp-93]:1 != 0x23` |
| `beq` at 0x3a3b8 | `[sp-93]:1 == 0x41` | `[sp-93]:1 != 0x41` |
| `bne` at 0x3a3d8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x3a3f8 | `[0x27ee58] == 0x0` | `[0x27ee58] != 0x0` |
| `beq` at 0x3a420 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x3a43c | `[sp-52] != 0x0` | `[sp-52] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3a454 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x39aa0).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x39b00).

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3a1dc).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3a264).

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3a408).

## `powerDown`

- `powerDown`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a458 to 0x3a838.
  Established. The first call to 0x6eb6c is followed by `beq`, whose taken zero-result path skips an
  earlier conditional dispatch (`bl` at 0x3a480, `beq` at 0x3a48c, `ldrls` at 0x3a504). That `ldrls`
  has taken condition `[sp-20]-1 <= 0x4 (unsigned, 32-bit)` and an unresolved taken edge (`ldrls` at
  0x3a504). A later word load from 0x281100 supplies the save `[0x281100]-1` at sp -0x20 before a
  loop whose `bge` has unknown operands (`ldr` at 0x3a654, `str` at 0x3a65c, `bge` at 0x3a668).
  Known calls include 0x3efcc with first candidates 0xff and 0x2, and 0x5edf8 with first candidate
  0x0 (`bl` at 0x3a5dc, `bl` at 0x3a7d0). Two other computed transfers remain unresolved (`mov` at
  0x3a624, `mov` at 0x3a7f8). The known return retains `[sp-24]`. The physical power-down sequence
  and additional transfer effects are unknown (`ldmdb` at 0x3a808).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a468 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3a470 | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x3a47c | store | 1 | `sp -0x71` | `(ret(0x2b31c) & 0xff)` |
| `ldrb` at 0x3a490 | load | 1 | `sp -0x71` | `-` |
| `str` at 0x3a494 | store | 4 | `sp -0x7c` | `[sp-113]:1` |
| `ldr` at 0x3a498 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3a4a4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3a4b0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3a4c0 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x3a4e4 | store | 4 | `sp -0x1c` | `ret(0x2d224)` |
| `ldr` at 0x3a4f0 | load | 4 | `abs 0x3a80c` | `-` |
| `ldr` at 0x3a4f8 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x3a504 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3a570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a57c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a588 | load | 4 | `abs 0x3a814` | `-` |
| `str` at 0x3a5a0 | store | 4 | `sp -0x1c` | `ret(0x2c5e4)` |
| `ldr` at 0x3a5a8 | load | 4 | `abs 0x3a818` | `-` |
| `str` at 0x3a5b8 | store | 4 | `sp -0x1c` | `ret(0x2bfac)` |
| `str` at 0x3a5cc | store | 4 | `sp -0x18` | `ret(0x433b0)` |
| `str` at 0x3a5e4 | store | 4 | `sp -0x1c` | `ret(0x3efcc)` |
| `ldr` at 0x3a5e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a5f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a600 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a604 | store | 4 | `sp -0x18` | `ret(0x3efcc)` |
| `ldr` at 0x3a608 | load | 4 | `abs 0x3a81c` | `-` |
| `ldr` at 0x3a60c | load | 4 | `abs 0x281160` | `-` |
| `ldr` at 0x3a618 | load | 4 | `abs 0x3a81c` | `-` |
| `ldr` at 0x3a61c | load | 4 | `abs 0x281160` | `-` |
| `ldr` at 0x3a650 | load | 4 | `abs 0x3a820` | `-` |
| `ldr` at 0x3a654 | load | 4 | `abs 0x281100` | `-` |
| `str` at 0x3a65c | store | 4 | `sp -0x20` | `[0x281100]-1` |
| `ldr` at 0x3a660 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3a670 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3a684 | load | 4 | `abs 0x3a824` | `-` |
| `ldr` at 0x3a688 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3a690 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a69c | load | 4 | `abs 0x3a828` | `-` |
| `ldr` at 0x3a6a0 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3a6a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3a6ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a6b8 | load | 4 | `abs 0x3a828` | `-` |
| `ldr` at 0x3a6bc | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3a6c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3a6c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a6d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3a6e8 | load | 4 | `abs 0x3a824` | `-` |
| `ldr` at 0x3a6ec | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3a6f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a710 | load | 4 | `abs 0x3a824` | `-` |
| `ldr` at 0x3a714 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3a728 | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x3a73c | store | 4 | `sp -0x1c` | `ret(0x3dde4)` |
| `ldr` at 0x3a740 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a74c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a758 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a75c | store | 4 | `sp -0x18` | `ret(0x3dde4)` |
| `ldr` at 0x3a760 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3a768 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x3a770 | load | 1 | `sp -0x71` | `-` |
| `ldr` at 0x3a788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a798 | load | 4 | `abs 0x3a82c` | `-` |
| `str` at 0x3a7a8 | store | 4 | `sp -0x1c` | `ret(0x2bfac)` |
| `ldr` at 0x3a7b4 | load | 4 | `abs 0x3a830` | `-` |
| `str` at 0x3a7c4 | store | 4 | `sp -0x1c` | `ret(0x2bfac)` |
| `ldr` at 0x3a7d4 | load | 4 | `abs 0x3a834` | `-` |
| `ldr` at 0x3a7d8 | load | 4 | `abs 0x27ee58` | `-` |
| `ldr` at 0x3a7ec | load | 4 | `abs 0x3a834` | `-` |
| `ldr` at 0x3a7f0 | load | 4 | `abs 0x27ee58` | `-` |
| `ldr` at 0x3a7fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3a800 | store | 4 | `sp -0x78` | `[sp-24]` |
| `ldr` at 0x3a804 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x3a808 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3a808 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3a808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a474 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3a480 | `(unknown callee) 0x6eb6c` | `ret(0x2b31c)` | `?` | `?` | `ret(0x2b31c)` |
| `bl` at 0x3a4dc | `(unknown callee) 0x2d224` | `0x2` | `0x0` | `0x1` | `[sp-113]:1` |
| `bl` at 0x3a4f4 | `(unknown callee) 0x18c74` | `sp-112` | `0xe1738` | `?` | `sp-112` |
| `bl` at 0x3a580 | `(unknown callee) 0x437dc` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x3a590 | `(unknown callee) 0x1dba4` | `0xe1774` | `ret(0x437dc)` | `?` | `ret(0x437dc)` |
| `bl` at 0x3a598 | `(unknown callee) 0x2c5e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3a5b0 | `(unknown callee) 0x2bfac` | `0x32` | `0xe1784` | `0x1` | `ret(0x2c5e4)` |
| `bl` at 0x3a5bc | `(unknown callee) 0x6ba1c` | `ret(0x2bfac)` | `?` | `?` | `ret(0x2bfac)` |
| `bl` at 0x3a5c4 | `(unknown callee) 0x433b0` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x3a5d0 | `(unknown callee) 0x2de10` | `ret(0x433b0)` | `?` | `?` | `ret(0x433b0)` |
| `bl` at 0x3a5dc | `(unknown callee) 0x3efcc` | `0xff` | `0x2` | `?` | `?` |
| `bl` at 0x3a6fc | `(unknown callee) 0xb78b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x3a734 | `(unknown callee) 0x3dde4` | `?` | `0x281120` | `?` | `?` |
| `bl` at 0x3a784 | `(unknown callee) 0xb7958` | `0x1` | `0x80` | `?` | `[sp-113]:1` |
| `bl` at 0x3a7a0 | `(unknown callee) 0x2bfac` | `0x32` | `0xe1790` | `0x0` | `[sp-24]` |
| `bl` at 0x3a7bc | `(unknown callee) 0x2bfac` | `0x32` | `0xe17a0` | `0x2` | `[sp-24]` |
| `bl` at 0x3a7c8 | `(unknown callee) 0x6ba34` | `ret(0x2bfac)` | `?` | `?` | `ret(0x2bfac)` |
| `bl` at 0x3a7d0 | `(unknown callee) 0x5edf8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3a7e8 | `(unknown callee) 0x3e294` | `0x28` | `?` | `?` | `[0x27ee58]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3a48c | `ret(0x6eb6c) == 0x0` | `ret(0x6eb6c) != 0x0` |
| `beq` at 0x3a4a0 | `[sp-113]:1 == 0x41` | `[sp-113]:1 != 0x41` |
| `bgt` at 0x3a4ac | `[sp-113]:1 > 0x41 (signed, 32-bit)` | `[sp-113]:1 <= 0x41 (signed, 32-bit)` |
| `beq` at 0x3a4b8 | `[sp-113]:1 == 0x39` | `[sp-113]:1 != 0x39` |
| `beq` at 0x3a4c8 | `[sp-113]:1 == 0x51` | `[sp-113]:1 != 0x51` |
| `ldrls` at 0x3a504 | `[sp-20]-1 <= 0x4 (unsigned, 32-bit)` | `[sp-20]-1 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x3a578 | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `beq` at 0x3a5f0 | `ret(0x3efcc) == 0x0` | `ret(0x3efcc) != 0x0` |
| `bne` at 0x3a5fc | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3a614 | `[0x281160] == 0x0` | `[0x281160] != 0x0` |
| `bge` at 0x3a668 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3a698 | `? == 0xff` | `? != 0xff` |
| `bne` at 0x3a6b4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3a6d0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3a708 | `ret(0xb78b4) != 0x0` | `ret(0xb78b4) == 0x0` |
| `beq` at 0x3a748 | `ret(0x3dde4) == 0x0` | `ret(0x3dde4) != 0x0` |
| `bne` at 0x3a754 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x3a778 | `[sp-113]:1 != 0x34` | `[sp-113]:1 == 0x34` |
| `bne` at 0x3a790 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3a7e0 | `[0x27ee58] == 0x0` | `[0x27ee58] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3a808 | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3a504).

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3a624).

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3a7f8).

## `powerGetMargin`

- `powerGetMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ae7c to 0x3b134.
  Established. The first `bne` takes the nonzero-result path after the call to 0x3dae8, while zero
  fall-through calls 0x3db60 (`bl` at 0x3ae98, `bne` at 0x3aeac, `bl` at 0x3aeb0). A later `beq`
  takes the byte-equal-to-0xff path from `[ret(0x3dae8)+6]:1` to a save of 0xfffff9be at sp -0x2c
  (`beq` at 0x3aecc, `str` at 0x3b10c). The conditional computed transfer has taken condition
  `[ret(0x3dae8)+8]-1 <= 0x4 (unsigned, 32-bit)` and an unresolved taken edge (`ldrls` at 0x3aee0).
  Known fall-through reaches 0x1eed0, followed by a word save of 0x0 at sp -0x2c (`bl` at 0x3b100,
  `str` at 0x3b118). Margin encoding and further dispatch effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ae8c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3ae90 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x3ae94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3aea0 | store | 4 | `sp -0x24` | `ret(0x3dae8)` |
| `ldr` at 0x3aea4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3aeb8 | store | 4 | `sp -0x2c` | `ret(0x3db60)` |
| `ldr` at 0x3aec0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3aec4 | load | 1 | `ret(0x3dae8) +0x6` | `-` |
| `ldr` at 0x3aed0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3aed4 | load | 4 | `ret(0x3dae8) +0x8` | `-` |
| `ldrls` at 0x3aee0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3b0dc | load | 4 | `abs 0x3b124` | `-` |
| `str` at 0x3b0e0 | store | 4 | `sp -0x34` | `0xe17b4` |
| `ldr` at 0x3b0e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b0e8 | load | 4 | `ret(0x3dae8) +0x8` | `-` |
| `str` at 0x3b0ec | store | 4 | `sp -0x30` | `[ret(0x3dae8)+8]` |
| `ldr` at 0x3b0f4 | load | 4 | `abs 0x3b128` | `-` |
| `ldr` at 0x3b0f8 | load | 4 | `abs 0x3b12c` | `-` |
| `ldr` at 0x3b108 | load | 4 | `abs 0x3b130` | `-` |
| `str` at 0x3b10c | store | 4 | `sp -0x2c` | `0xfffff9be` |
| `str` at 0x3b118 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3b11c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3b120 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3b120 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3b120 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b120 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b120 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ae98 | `(unknown callee) 0x3dae8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3aeb0 | `(unknown callee) 0x3db60` | `ret(0x3dae8)` | `?` | `?` | `ret(0x3dae8)` |
| `bl` at 0x3b100 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x4e7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3aeac | `ret(0x3dae8) != 0x0` | `ret(0x3dae8) == 0x0` |
| `beq` at 0x3aecc | `[ret(0x3dae8)+6]:1 == 0xff` | `[ret(0x3dae8)+6]:1 != 0xff` |
| `ldrls` at 0x3aee0 | `[ret(0x3dae8)+8]-1 <= 0x4 (unsigned, 32-bit)` | `[ret(0x3dae8)+8]-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b120 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3aee0).

## `powerGetReading`

- `powerGetReading`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3cd7c to 0x3d108.
  Established. The call to 0xb0614 has candidates `[ret(0x3dae8)+4]:1`, `[ret(0x3dae8)+5]:1`, sp-42
  and `[ret(0x3dae8)+5]:1` (`bl` at 0x3cde0). After its zero-result path is taken by `beq`, another
  `beq` skips the halfword store at `[sp-32] +0x0` when `[sp-32] == 0x0` (`beq` at 0x3cdf4, `beq` at
  0x3ce0c, `strh` at 0x3ce18). A later word store writes `ret(0xd65f0)` at `[sp-56] +0x0` (`str` at
  0x3ce94). The `ldrls` taken condition is `? <= 0x6 (unsigned, 32-bit)`, with an unresolved taken
  edge (`ldrls` at 0x3ced0). The measurement units, conversion and additional transfer effects
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3cd8c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3cd90 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x3cd94 | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x3cd98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3cda4 | store | 4 | `sp -0x28` | `ret(0x3dae8)` |
| `ldr` at 0x3cda8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3cdbc | store | 4 | `sp -0x34` | `ret(0x3db60)` |
| `ldr` at 0x3cdc4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3cdc8 | load | 1 | `ret(0x3dae8) +0x4` | `-` |
| `ldr` at 0x3cdcc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3cdd0 | load | 1 | `ret(0x3dae8) +0x5` | `-` |
| `str` at 0x3cde8 | store | 4 | `sp -0x30` | `ret(0xb0614)` |
| `ldr` at 0x3cdec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3cdf8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3cdfc | store | 4 | `sp -0x34` | `ret(0xb0614)` |
| `ldr` at 0x3ce04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ce10 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3ce14 | load | 2 | `sp -0x2a` | `-` |
| `strh` at 0x3ce18 | store | 2 | `[sp-32] +0x0` | `([sp-42]:2 & 0xffff)` |
| `ldr` at 0x3ce1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ce28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ce2c | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldrsh` at 0x3ce30 | load | 2 | `sp -0x2a` | `-` |
| `str` at 0x3ce3c | store | 4 | `sp -0x3c` | `ret(0xd65a4)` |
| `ldrsh` at 0x3ce40 | load | 2 | `sp -0x2a` | `-` |
| `ldr` at 0x3ce4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3ce50 | load | 4 | `abs 0x3d0f0` | `-` |
| `str` at 0x3ce5c | store | 4 | `sp -0x3c` | `ret(0xd63dc)` |
| `ldr` at 0x3ce60 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3ce64 | load | 1 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x3ce68 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3ce6c | load | 1 | `[sp-40] +0x5` | `-` |
| `ldr` at 0x3ce80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3ce90 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3ce94 | store | 4 | `[sp-56] +0x0` | `ret(0xd65f0)` |
| `ldr` at 0x3ce98 | load | 4 | `abs 0x3d0f4` | `-` |
| `ldr` at 0x3ce9c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3cea0 | load | 1 | `[sp-40] +0x0` | `-` |
| `ldrh` at 0x3cec0 | load | 2 | `(unknown)` | `-` |
| `ldrls` at 0x3ced0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3d0c4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3d0c8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3d0cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d0cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3d0cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d0cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d0cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3cd9c | `(unknown callee) 0x3dae8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3cdb4 | `(unknown callee) 0x3db60` | `ret(0x3dae8)` | `?` | `?` | `ret(0x3dae8)` |
| `bl` at 0x3cde0 | `(unknown callee) 0xb0614` | `[ret(0x3dae8)+4]:1` | `[ret(0x3dae8)+5]:1` | `sp-42` | `[ret(0x3dae8)+5]:1` |
| `bl` at 0x3ce38 | `(unknown callee) 0xd65a4` | `?` | `?` | `[sp-36]` | `?` |
| `bl` at 0x3ce54 | `(unknown callee) 0xd63dc` | `ret(0xd65a4)` | `0x47800000` | `?` | `?` |
| `bl` at 0x3ce78 | `(unknown callee) 0xb0f24` | `[[sp-40]+4]:1` | `[[sp-40]+5]:1` | `[[sp-40]+4]:1` | `[[sp-40]+5]:1` |
| `bl` at 0x3ce88 | `(unknown callee) 0xd65f0` | `[sp-60]` | `ret(0xb0f24)` | `?` | `ret(0xb0f24)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3cdb0 | `ret(0x3dae8) != 0x0` | `ret(0x3dae8) == 0x0` |
| `beq` at 0x3cdf4 | `ret(0xb0614) == 0x0` | `ret(0xb0614) != 0x0` |
| `beq` at 0x3ce0c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x3ce24 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bge` at 0x3ce48 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `ldrls` at 0x3ced0 | `? <= 0x6 (unsigned, 32-bit)` | `? > 0x6 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d0cc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3ced0).

## `powerLowVoltage`

- `powerLowVoltage`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d2e0 to 0x3d40c.
  Established. The first call targets 0x3d538 (`bl` at 0x3d2f8). Three `beq` sites take paths for
  `[sp-40]` equal to 0x49, 0x25 or 0x50 (`beq` at 0x3d31c, `beq` at 0x3d334, `beq` at 0x3d344). A
  later `bne` tests an unknown expression against 0xb. Equal fall-through saves 0x40200000 at sp
  -0x24 (`bne` at 0x3d374, `str` at 0x3d37c). Subsequent calls include 0x3d6dc and 0xd62e0, whose
  result is retained in r0 at return (`bl` at 0x3d394, `bl` at 0x3d3ec, `ldmdb` at 0x3d3f8). The
  low-voltage arithmetic and units are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d2f0 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x3d2f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d300 | store | 4 | `sp -0x24` | `ret(0x3d538)` |
| `str` at 0x3d310 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3d314 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d32c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d33c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d34c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d360 | load | 4 | `abs 0x3d404` | `-` |
| `ldr` at 0x3d364 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d36c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3d378 | load | 4 | `abs 0x3d408` | `-` |
| `str` at 0x3d37c | store | 4 | `sp -0x24` | `0x40200000` |
| `ldr` at 0x3d380 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d390 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x3d3b0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3d3b0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x3d3f8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d3f8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d3f8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3d3f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d3f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d3f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d2f8 | `(unknown callee) 0x3d538` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3d304 | `(unknown callee) 0x2b31c` | `ret(0x3d538)` | `?` | `?` | `ret(0x3d538)` |
| `bl` at 0x3d384 | `(unknown callee) 0xd5c38` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x3d394 | `(unknown callee) 0x3d6dc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3d3a0 | `(unknown callee) 0xd5c38` | `ret(0x3d6dc)` | `?` | `?` | `ret(0x3d6dc)` |
| `bl` at 0x3d3bc | `(unknown callee) 0xd58fc` | `?` | `?` | `ret(0xd5c38)` | `?` |
| `bl` at 0x3d3d8 | `(unknown callee) 0xd5c84` | `ret(0xd5c38)` | `?` | `ret(0xd58fc)` | `?` |
| `bl` at 0x3d3ec | `(unknown callee) 0xd62e0` | `ret(0xd5c84)` | `?` | `?` | `ret(0xd5c84)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d31c | `[sp-40] == 0x49` | `[sp-40] != 0x49` |
| `bgt` at 0x3d328 | `[sp-40] > 0x49 (signed, 32-bit)` | `[sp-40] <= 0x49 (signed, 32-bit)` |
| `beq` at 0x3d334 | `[sp-40] == 0x25` | `[sp-40] != 0x25` |
| `beq` at 0x3d344 | `[sp-40] == 0x50` | `[sp-40] != 0x50` |
| `bne` at 0x3d374 | `? != 0xb` | `? == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d3f8 | `r0` | `ret(0xd62e0)` |

## `powerHighVoltage`

- `powerHighVoltage`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d40c to 0x3d538.
  Established. Calls first target 0x3d538 and 0x2b31c (`bl` at 0x3d424, `bl` at 0x3d430). The later
  `beq` paths for `[sp-40]` equal to 0x49, 0x25 or 0x50 lead to a `bne` test of an unknown
  expression against 0xb (`beq` at 0x3d448, `beq` at 0x3d460, `beq` at 0x3d470, `bne` at 0x3d4a0);
  its equal fall-through saves 0x40200000 at sp -0x24 (`str` at 0x3d4a8). The last call targets
  0xd62e0 and supplies the return's r0 expression (`bl` at 0x3d518, `ldmdb` at 0x3d524). The
  high-voltage interpretation and units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d41c | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x3d420 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d42c | store | 4 | `sp -0x24` | `ret(0x3d538)` |
| `str` at 0x3d43c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3d440 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d44c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d458 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d468 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d48c | load | 4 | `abs 0x3d530` | `-` |
| `ldr` at 0x3d490 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d498 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3d4a4 | load | 4 | `abs 0x3d534` | `-` |
| `str` at 0x3d4a8 | store | 4 | `sp -0x24` | `0x40200000` |
| `ldr` at 0x3d4ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d4bc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x3d4e4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3d4e4 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x3d524 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d524 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d524 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3d524 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d524 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d524 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d424 | `(unknown callee) 0x3d538` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3d430 | `(unknown callee) 0x2b31c` | `ret(0x3d538)` | `?` | `?` | `ret(0x3d538)` |
| `bl` at 0x3d4b0 | `(unknown callee) 0xd5c38` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x3d4c0 | `(unknown callee) 0x3d6dc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3d4cc | `(unknown callee) 0xd5c38` | `ret(0x3d6dc)` | `?` | `?` | `ret(0x3d6dc)` |
| `bl` at 0x3d4e8 | `(unknown callee) 0xd5900` | `ret(0xd5c38)` | `?` | `?` | `?` |
| `bl` at 0x3d504 | `(unknown callee) 0xd5c84` | `ret(0xd5c38)` | `?` | `ret(0xd5900)` | `?` |
| `bl` at 0x3d518 | `(unknown callee) 0xd62e0` | `ret(0xd5c84)` | `?` | `?` | `ret(0xd5c84)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d448 | `[sp-40] == 0x49` | `[sp-40] != 0x49` |
| `bgt` at 0x3d454 | `[sp-40] > 0x49 (signed, 32-bit)` | `[sp-40] <= 0x49 (signed, 32-bit)` |
| `beq` at 0x3d460 | `[sp-40] == 0x25` | `[sp-40] != 0x25` |
| `beq` at 0x3d470 | `[sp-40] == 0x50` | `[sp-40] != 0x50` |
| `bne` at 0x3d4a0 | `? != 0xb` | `? == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d524 | `r0` | `ret(0xd62e0)` |

## `powerNominalVoltageI`

- `powerNominalVoltageI`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d538 to 0x3d6dc.
  Established. A word read from 0x27ee98 decides `beq`, whose taken zero path reaches the call to
  0x3d108 (`ldr` at 0x3d5a0, `beq` at 0x3d5a8, `bl` at 0x3d5d0). Nonzero fall-through instead loads
  a word from 0x27ee94 and reaches an unresolved computed transfer (`ldr` at 0x3d5b0, `mov` at
  0x3d5bc). Other known paths save 0x4023e76d or an unknown word at sp -0x18 (`str` at 0x3d66c,
  `str` at 0x3d6b4). The return value, voltage units and additional transfer effects remain unknown
  (`ldmdb` at 0x3d6bc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d548 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d54c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d560 | load | 4 | `abs 0x3d6c0` | `-` |
| `ldr` at 0x3d564 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d56c | load | 1 | `(unknown)` | `-` |
| `str` at 0x3d570 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3d574 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d58c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d59c | load | 4 | `abs 0x3d6c4` | `-` |
| `ldr` at 0x3d5a0 | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d5ac | load | 4 | `abs 0x3d6c4` | `-` |
| `ldr` at 0x3d5b0 | load | 4 | `abs 0x27ee94` | `-` |
| `ldr` at 0x3d5b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d5cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d5d8 | load | 4 | `abs 0x3d6c8` | `-` |
| `str` at 0x3d5dc | store | 4 | `sp -0x2c` | `0xe1b6c` |
| `ldr` at 0x3d5e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d5f4 | load | 4 | `abs 0x3d6c0` | `-` |
| `ldr` at 0x3d5f8 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d600 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3d604 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x3d608 | store | 4 | `sp -0x24` | `ret(0x3d108)` |
| `ldr` at 0x3d610 | load | 4 | `abs 0x3d6cc` | `-` |
| `ldr` at 0x3d614 | load | 4 | `abs 0x3d6d0` | `-` |
| `str` at 0x3d62c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3d630 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d63c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d668 | load | 4 | `abs 0x3d6d4` | `-` |
| `str` at 0x3d66c | store | 4 | `sp -0x18` | `0x4023e76d` |
| `ldr` at 0x3d674 | load | 4 | `abs 0x3d6d8` | `-` |
| `ldr` at 0x3d678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d68c | load | 4 | `abs 0x3d6c0` | `-` |
| `ldr` at 0x3d690 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d698 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3d6b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3d6b4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3d6b8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d6bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d6bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d6bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d5d0 | `(unknown callee) 0x3d108` | `r0` | `r1` | `[sp-32]` | `[0x27ee98]` |
| `bl` at 0x3d61c | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x8d1` | `0x0` |
| `bl` at 0x3d620 | `(unknown callee) 0x2b31c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d57c | `[sp-32] == 0xb` | `[sp-32] != 0xb` |
| `blt` at 0x3d588 | `[sp-32] < 0xb (signed, 32-bit)` | `[sp-32] >= 0xb (signed, 32-bit)` |
| `bhi` at 0x3d598 | `[sp-32]-16 > 0x1 (unsigned, 32-bit)` | `[sp-32]-16 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x3d5a8 | `[0x27ee98] == 0x0` | `[0x27ee98] != 0x0` |
| `beq` at 0x3d638 | `[sp-28] == 0x49` | `[sp-28] != 0x49` |
| `bgt` at 0x3d644 | `[sp-28] > 0x49 (signed, 32-bit)` | `[sp-28] <= 0x49 (signed, 32-bit)` |
| `beq` at 0x3d650 | `[sp-28] == 0x25` | `[sp-28] != 0x25` |
| `beq` at 0x3d660 | `[sp-28] == 0x50` | `[sp-28] != 0x50` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d6bc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3d5bc).

## `powerMarginPercent`

- `powerMarginPercent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d6dc to 0x3d7d8.
  Established. The `beq` paths for `[sp-28]` equal to 0xa and 0x10 reach different arms (`beq` at
  0x3d720, `beq` at 0x3d72c). The first arm saves 0x3cdd67ba at sp -0x18 (`str` at 0x3d7a0). In the
  second arm, a `beq` takes the zero-word path from 0x27ee9c to 0x3d108. Nonzero fall-through
  reloads that global and reaches an unresolved computed transfer (`beq` at 0x3d740, `ldr` at
  0x3d748, `mov` at 0x3d754, `bl` at 0x3d768). Another known path saves 0x3d4ccccd at sp -0x18
  (`str` at 0x3d7ac). The percent encoding and further transfer effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d6ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d6f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d704 | load | 4 | `abs 0x3d7bc` | `-` |
| `ldr` at 0x3d708 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d710 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3d714 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3d718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d724 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d734 | load | 4 | `abs 0x3d7c0` | `-` |
| `ldr` at 0x3d738 | load | 4 | `abs 0x27ee9c` | `-` |
| `ldr` at 0x3d744 | load | 4 | `abs 0x3d7c0` | `-` |
| `ldr` at 0x3d748 | load | 4 | `abs 0x27ee9c` | `-` |
| `ldr` at 0x3d74c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d770 | load | 4 | `abs 0x3d7c4` | `-` |
| `str` at 0x3d774 | store | 4 | `sp -0x28` | `0xe1ba8` |
| `ldr` at 0x3d778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d77c | store | 4 | `sp -0x24` | `[sp-20]` |
| `str` at 0x3d780 | store | 4 | `sp -0x20` | `ret(0x3d108)` |
| `ldr` at 0x3d788 | load | 4 | `abs 0x3d7c8` | `-` |
| `ldr` at 0x3d78c | load | 4 | `abs 0x3d7cc` | `-` |
| `ldr` at 0x3d79c | load | 4 | `abs 0x3d7d0` | `-` |
| `str` at 0x3d7a0 | store | 4 | `sp -0x18` | `0x3cdd67ba` |
| `ldr` at 0x3d7a8 | load | 4 | `abs 0x3d7d4` | `-` |
| `str` at 0x3d7ac | store | 4 | `sp -0x18` | `0x3d4ccccd` |
| `ldr` at 0x3d7b4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d7b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d7b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d7b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d768 | `(unknown callee) 0x3d108` | `r0` | `r1` | `?` | `[0x27ee9c]` |
| `bl` at 0x3d794 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x8ea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d720 | `[sp-28] == 0xa` | `[sp-28] != 0xa` |
| `beq` at 0x3d72c | `[sp-28] == 0x10` | `[sp-28] != 0x10` |
| `beq` at 0x3d740 | `[0x27ee9c] == 0x0` | `[0x27ee9c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d7b8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3d754).

## `powerLowMargin`

- `powerLowMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d7d8 to 0x3d8ac.
  Established. `bhi` takes the path past the global test when `[r0]:1-16 > 0x1` unsigned (`bhi` at
  0x3d7fc). If it falls through, `beq` takes the zero-word path from 0x27ee98 to 0x3da54 and
  0x3d108 (`beq` at 0x3d80c, `bl` at 0x3d848, `bl` at 0x3d854). The nonzero path instead reloads
  0x27ee98 and reaches an unresolved computed transfer (`ldr` at 0x3d820, `mov` at 0x3d830). The
  later byte load has an unknown effective target and is saved as an unknown word at sp -0x18
  (`ldrb` at 0x3d88c, `str` at 0x3d890). Physical margin selection and additional effects are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d7e8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d7ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d7f0 | load | 1 | `r0 +0x0` | `-` |
| `ldr` at 0x3d800 | load | 4 | `abs 0x3d89c` | `-` |
| `ldr` at 0x3d804 | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d81c | load | 4 | `abs 0x3d89c` | `-` |
| `ldr` at 0x3d820 | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d85c | load | 4 | `abs 0x3d8a0` | `-` |
| `str` at 0x3d860 | store | 4 | `sp -0x24` | `0xe1b6c` |
| `ldr` at 0x3d864 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d868 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0x3d86c | store | 4 | `sp -0x20` | `[[sp-20]]:1` |
| `str` at 0x3d870 | store | 4 | `sp -0x1c` | `ret(0x3d108)` |
| `ldr` at 0x3d878 | load | 4 | `abs 0x3d8a4` | `-` |
| `ldr` at 0x3d87c | load | 4 | `abs 0x3d8a8` | `-` |
| `ldr` at 0x3d888 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d88c | load | 1 | `(unknown)` | `-` |
| `str` at 0x3d890 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3d894 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d898 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d898 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d814 | `(unknown callee) 0x3da54` | `r0` | `r1` | `r2` | `[0x27ee98]` |
| `bl` at 0x3d848 | `(unknown callee) 0x3da54` | `r0` | `r1` | `r2` | `[0x27ee98]` |
| `bl` at 0x3d854 | `(unknown callee) 0x3d108` | `ret(0x3da54)` | `?` | `?` | `ret(0x3da54)` |
| `bl` at 0x3d884 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x901` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x3d7fc | `[r0]:1-16 > 0x1 (unsigned, 32-bit)` | `[r0]:1-16 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x3d80c | `[0x27ee98] == 0x0` | `[0x27ee98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d898 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3d830).

## `powerNormalMargin`

- `powerNormalMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d8ac to 0x3d980.
  Established. The initial `bhi` takes the path past the global test for `[r0]:1-16 > 0x1` unsigned
  (`bhi` at 0x3d8d0). The later `beq` takes the zero-word path from 0x27ee98 to two calls to
  0x3da54 and 0x3d108 (`beq` at 0x3d8e0, `bl` at 0x3d91c, `bl` at 0x3d928). Nonzero fall-through
  reaches an unresolved computed transfer after reloading 0x27ee98 (`ldr` at 0x3d8f4, `mov` at
  0x3d904). An unknown-valued word save at sp -0x18 follows a byte load with unknown target on a
  known path (`ldrb` at 0x3d960, `str` at 0x3d964). Normal-margin semantics and further effects
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d8bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d8c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d8c4 | load | 1 | `r0 +0x0` | `-` |
| `ldr` at 0x3d8d4 | load | 4 | `abs 0x3d970` | `-` |
| `ldr` at 0x3d8d8 | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d8e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d8f0 | load | 4 | `abs 0x3d970` | `-` |
| `ldr` at 0x3d8f4 | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d918 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d930 | load | 4 | `abs 0x3d974` | `-` |
| `str` at 0x3d934 | store | 4 | `sp -0x24` | `0xe1b6c` |
| `ldr` at 0x3d938 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d93c | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0x3d940 | store | 4 | `sp -0x20` | `[[sp-20]]:1` |
| `str` at 0x3d944 | store | 4 | `sp -0x1c` | `ret(0x3d108)` |
| `ldr` at 0x3d94c | load | 4 | `abs 0x3d978` | `-` |
| `ldr` at 0x3d950 | load | 4 | `abs 0x3d97c` | `-` |
| `ldr` at 0x3d95c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d960 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3d964 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3d968 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d96c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d96c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d96c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d8e8 | `(unknown callee) 0x3da54` | `r0` | `r1` | `r2` | `[0x27ee98]` |
| `bl` at 0x3d91c | `(unknown callee) 0x3da54` | `r0` | `r1` | `r2` | `[0x27ee98]` |
| `bl` at 0x3d928 | `(unknown callee) 0x3d108` | `ret(0x3da54)` | `?` | `?` | `ret(0x3da54)` |
| `bl` at 0x3d958 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x918` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x3d8d0 | `[r0]:1-16 > 0x1 (unsigned, 32-bit)` | `[r0]:1-16 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x3d8e0 | `[0x27ee98] == 0x0` | `[0x27ee98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d96c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3d904).

## `powerHighMargin`

- `powerHighMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d980 to 0x3da54.
  Established. The conditional `bhi` takes the `[r0]:1-16 > 0x1` unsigned path past the global test
  (`bhi` at 0x3d9a4). The global-zero path taken by `beq` reaches 0x3da54 and 0x3d108 (`beq` at
  0x3d9b4, `bl` at 0x3d9f0, `bl` at 0x3d9fc). The other path reloads 0x27ee98 and reaches an
  unresolved computed transfer (`ldr` at 0x3d9c8, `mov` at 0x3d9d8). The later byte load and its
  word save at sp -0x18 have unknown target and value respectively (`ldrb` at 0x3da34, `str` at
  0x3da38). High-margin semantics and additional transfer effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d990 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d994 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d998 | load | 1 | `r0 +0x0` | `-` |
| `ldr` at 0x3d9a8 | load | 4 | `abs 0x3da44` | `-` |
| `ldr` at 0x3d9ac | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d9b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d9c4 | load | 4 | `abs 0x3da44` | `-` |
| `ldr` at 0x3d9c8 | load | 4 | `abs 0x27ee98` | `-` |
| `ldr` at 0x3d9ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3da04 | load | 4 | `abs 0x3da48` | `-` |
| `str` at 0x3da08 | store | 4 | `sp -0x24` | `0xe1b6c` |
| `ldr` at 0x3da0c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3da10 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0x3da14 | store | 4 | `sp -0x20` | `[[sp-20]]:1` |
| `str` at 0x3da18 | store | 4 | `sp -0x1c` | `ret(0x3d108)` |
| `ldr` at 0x3da20 | load | 4 | `abs 0x3da4c` | `-` |
| `ldr` at 0x3da24 | load | 4 | `abs 0x3da50` | `-` |
| `ldr` at 0x3da30 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3da34 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3da38 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3da3c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3da40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3da40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3da40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d9bc | `(unknown callee) 0x3da54` | `r0` | `r1` | `r2` | `[0x27ee98]` |
| `bl` at 0x3d9f0 | `(unknown callee) 0x3da54` | `r0` | `r1` | `r2` | `[0x27ee98]` |
| `bl` at 0x3d9fc | `(unknown callee) 0x3d108` | `ret(0x3da54)` | `?` | `?` | `ret(0x3da54)` |
| `bl` at 0x3da2c | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x92e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x3d9a4 | `[r0]:1-16 > 0x1 (unsigned, 32-bit)` | `[r0]:1-16 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x3d9b4 | `[0x27ee98] == 0x0` | `[0x27ee98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3da40 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3d9d8).

## `powerGetIndexOfConfig`

- `powerGetIndexOfConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3da54 to 0x3dae8.
  Established. The word at sp -0x18 begins at zero (`str` at 0x3da6c). The loop's `blt` has unknown
  operands, and its `bne` retains only Z == 0 (`blt` at 0x3da80, `bne` at 0x3dab0). That `bne` taken
  path skips the unknown-valued result save at sp -0x1c (`bne` at 0x3dab0, `str` at 0x3dab8).
  Another result save to that same slot is also unknown (`str` at 0x3dad4). The return retains
  `[sp-28]`, while the index interpretation and loop count remain unknown (`ldmdb` at 0x3dadc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3da64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3da6c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3da70 | load | 4 | `abs 0x3dae0` | `-` |
| `ldr` at 0x3da74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3da78 | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3da88 | load | 4 | `abs 0x3dae4` | `-` |
| `ldr` at 0x3da8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3daa0 | load | 4 | `abs 0x281120` | `-` |
| `ldr` at 0x3daa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dab4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dab8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3dac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dac8 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x3dad4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3dad8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3dadc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dadc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dadc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3da80 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3dab0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dadc | `r0` | `[sp-28]` |

## `IP57_PowerUp`

- `IP57_PowerUp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d594 to 0x4d698.
  Established. The call to 0x4e270 has first candidate `(r0 & 0xff)` and second candidate sp-24
  (`bl` at 0x4d5c4). `beq` takes its zero-result path to a byte initialization at sp -0x21 before a
  loop whose `bls` predicate has unknown operands (`beq` at 0x4d5d8, `strb` at 0x4d5ec, `bls` at
  0x4d5f8). Within that loop, another `beq` skips 0xb7c74 when `ret(0x2e8fc) == 0x0` (`beq` at
  0x4d628, `bl` at 0x4d64c). The call to 0xb7c74 has first candidate unknown and second candidate
  0x80 (`bl` at 0x4d64c). The board's physical power-up effect and iteration count are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d5a8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4d5b0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x4d5b4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d5cc | store | 4 | `sp -0x20` | `ret(0x4e270)` |
| `ldr` at 0x4d5d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d5dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d5e0 | store | 4 | `sp -0x28` | `ret(0x4e270)` |
| `strb` at 0x4d5ec | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x4d5f0 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x4d600 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d60c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4d62c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4d630 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x4d634 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x4d654 | store | 4 | `sp -0x20` | `ret(0xb7c74)` |
| `ldr` at 0x4d658 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d664 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d670 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d674 | store | 4 | `sp -0x1c` | `ret(0xb7c74)` |
| `ldrb` at 0x4d678 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x4d680 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x4d688 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d68c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x4d690 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4d694 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d694 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d5c4 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4d61c | `(unknown callee) 0x2e8fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x4d64c | `(unknown callee) 0xb7c74` | `?` | `0x80` | `[[sp-24]+2]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d5d8 | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `bls` at 0x4d5f8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x4d628 | `ret(0x2e8fc) == 0x0` | `ret(0x2e8fc) != 0x0` |
| `beq` at 0x4d660 | `ret(0xb7c74) == 0x0` | `ret(0xb7c74) != 0x0` |
| `bne` at 0x4d66c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d694 | `r0` | `?` |

## `IP57_PowerDown`

- `IP57_PowerDown`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d698 to 0x4d770.
  Established. The first call to 0x4e270 uses candidates `(r0 & 0xff)`, sp-24, sp-24 and `(r0 &
  0xff)` (`bl` at 0x4d6c8). `beq` takes its zero-result path to the byte initialization at sp -0x21
  (`beq` at 0x4d6dc, `strb` at 0x4d6f0). The loop's call to 0xb7c2c has first candidate unknown and
  second candidate 0x80 (`bl` at 0x4d724); its following `beq` takes the zero-result path past the
  result save at sp -0x1c (`beq` at 0x4d738, `str` at 0x4d74c). The physical power-down effect
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d6ac | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4d6b4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x4d6b8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d6d0 | store | 4 | `sp -0x20` | `ret(0x4e270)` |
| `ldr` at 0x4d6d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d6e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d6e4 | store | 4 | `sp -0x28` | `ret(0x4e270)` |
| `strb` at 0x4d6f0 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x4d6f4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4d704 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4d708 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x4d70c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x4d72c | store | 4 | `sp -0x20` | `ret(0xb7c2c)` |
| `ldr` at 0x4d730 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4d73c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d748 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4d74c | store | 4 | `sp -0x1c` | `ret(0xb7c2c)` |
| `ldrb` at 0x4d750 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x4d758 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x4d760 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d764 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x4d768 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4d76c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d76c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d76c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d6c8 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4d724 | `(unknown callee) 0xb7c2c` | `?` | `0x80` | `[[sp-24]+2]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d6dc | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `bls` at 0x4d6fc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x4d738 | `ret(0xb7c2c) == 0x0` | `ret(0xb7c2c) != 0x0` |
| `bne` at 0x4d744 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d76c | `r0` | `?` |

## `EnvPower`

- `EnvPower`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b68c to 0x6b6b4.
  Established. Entry r0 is saved as a word at sp -0x14 (`str` at 0x6b69c). The first call targets
  0x1abd0 with r0 candidate 0x7d and r1 candidate entry r0 (`bl` at 0x6b6a8). A second call targets
  0x6b748, whose result reaches r0 at return (`bl` at 0x6b6ac, `ldmdb` at 0x6b6b0). The
  power-request meaning of 0x7d is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b69c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6b6a4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b6b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b6b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b6b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b6a8 | `(unknown callee) 0x1abd0` | `0x7d` | `r0` | `r2` | `r3` |
| `bl` at 0x6b6ac | `(unknown callee) 0x6b748` | `ret(0x1abd0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b6b0 | `r0` | `ret(0x6b748)` |
