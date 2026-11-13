# ARM board reset and I/O paths in 1.34.2

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
`l1ioInit` reports one unresolved computed transfer on known edges. The other fourteen routines
report none. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/board-reset-io-facts.txt) retain all tool sections.

## `IP41_ResetHold`

- `IP41_ResetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21f10 to 0x2222c.
  Established. A nonzero byte at absolute `0x119a0c` takes the entry branch into the longer path,
  while zero falls through to word `0xfffffa24` at `sp -0x20` (`ldrb` at 0x21f28, `bne` at
  0x21f30, `str` at 0x21f38). The longer path clears byte `0x119a04` (`strb` at 0x21f48); its calls
  include `0xb79a0` with candidates `0xa`, `0x3`, `0x2`, unknown and later with `0xa`, `0x88`,
  `0x0`, unknown (`bl` at 0x21fe4, `bl` at 0x22138). Another pair of calls to `0xb7958` uses first
  candidates `0xa` and `0xc`, each with second candidate `0x4` (`bl` at 0x221b8, `bl` at 0x221e4).
  The external reset-line meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21f20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x21f24 | load | 4 | `abs 0x22218` | `-` |
| `ldrb` at 0x21f28 | load | 1 | `abs 0x119a0c` | `-` |
| `ldr` at 0x21f34 | load | 4 | `abs 0x2221c` | `-` |
| `str` at 0x21f38 | store | 4 | `sp -0x20` | `0xfffffa24` |
| `ldr` at 0x21f40 | load | 4 | `abs 0x22220` | `-` |
| `strb` at 0x21f48 | store | 1 | `abs 0x119a04` | `0x0` |
| `ldr` at 0x21f9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x21fa0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldrb` at 0x21fa8 | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0x21fac | load | 4 | `abs 0x22224` | `-` |
| `str` at 0x21fbc | store | 4 | `sp -0x1c` | `ret(0x2f208)` |
| `ldr` at 0x21fc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x21fcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x21fd0 | store | 4 | `sp -0x20` | `ret(0x2f208)` |
| `str` at 0x21fec | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x21ff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x21ffc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22000 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `str` at 0x2201c | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x22020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2202c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22030 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `str` at 0x22048 | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x2204c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22058 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2205c | store | 4 | `sp -0x20` | `ret(0xb790c)` |
| `str` at 0x22074 | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x22078 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22084 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22088 | store | 4 | `sp -0x20` | `ret(0xb790c)` |
| `ldr` at 0x22090 | load | 4 | `abs 0x22228` | `-` |
| `ldr` at 0x22094 | load | 4 | `abs 0x119a00` | `-` |
| `strb` at 0x220b8 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x220bc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x220cc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x220e4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x22100 | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x22104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22110 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22114 | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `ldrb` at 0x2211c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x22124 | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `str` at 0x22140 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x22144 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22150 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22154 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `str` at 0x22170 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x22174 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22180 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22184 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `ldr` at 0x2218c | load | 4 | `abs 0x22228` | `-` |
| `ldr` at 0x22190 | load | 4 | `abs 0x119a00` | `-` |
| `str` at 0x221c0 | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x221c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x221d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x221d4 | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `str` at 0x221ec | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x221f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x221fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22200 | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `str` at 0x2220c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x22210 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x22214 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x22214 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x22214 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x21f50 | `(unknown callee) 0x2a880` | `0x0` | `r1` | `0x119a04` | `0x0` |
| `bl` at 0x21f58 | `(unknown callee) 0x2a8a8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x21f60 | `(unknown callee) 0x24228` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x21f64 | `(unknown callee) 0x3a8c0` | `ret(0x24228)` | `?` | `?` | `?` |
| `bl` at 0x21f74 | `(unknown callee) 0x2b31c` | `ret(0x3a8c0)` | `?` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x21f8c | `(unknown callee) 0xc478c` | `?` | `sp-28` | `sp-28` | `?` |
| `bl` at 0x21fb4 | `(unknown callee) 0x2f208` | `0x29ed4` | `[sp-20]:1` | `?` | `[sp-20]:1` |
| `bl` at 0x21fe4 | `(unknown callee) 0xb79a0` | `0xa` | `0x3` | `0x2` | `?` |
| `bl` at 0x22014 | `(unknown callee) 0xb79a0` | `0xc` | `0x3` | `0x2` | `ret(0xb79a0)` |
| `bl` at 0x22040 | `(unknown callee) 0xb790c` | `0xb` | `0x1` | `?` | `ret(0xb79a0)` |
| `bl` at 0x2206c | `(unknown callee) 0xb790c` | `0xd` | `0x1` | `?` | `ret(0xb790c)` |
| `bl` at 0x220a0 | `(unknown callee) 0x3ddc4` | `ret(0xb790c)` | `?` | `?` | `[0x119a00]` |
| `bl` at 0x220d4 | `(unknown callee) 0x2e8fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x220f8 | `(unknown callee) 0xb7958` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x22138 | `(unknown callee) 0xb79a0` | `0xa` | `0x88` | `0x0` | `?` |
| `bl` at 0x22168 | `(unknown callee) 0xb79a0` | `0xc` | `0x88` | `0x0` | `ret(0xb79a0)` |
| `bl` at 0x2219c | `(unknown callee) 0x3ddc4` | `ret(0xb79a0)` | `?` | `?` | `[0x119a00]` |
| `bl` at 0x221b8 | `(unknown callee) 0xb7958` | `0xa` | `0x4` | `?` | `?` |
| `bl` at 0x221e4 | `(unknown callee) 0xb7958` | `0xc` | `0x4` | `?` | `ret(0xb7958)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x21f30 | `[0x119a0c]:1 != 0x0` | `[0x119a0c]:1 == 0x0` |
| `beq` at 0x21f70 | `ret(0x3a8c0) == 0x0` | `ret(0x3a8c0) != 0x0` |
| `bne` at 0x21f98 | `ret(0xc478c) != 0x0` | `ret(0xc478c) == 0x0` |
| `beq` at 0x21fc8 | `ret(0x2f208) == 0x0` | `ret(0x2f208) != 0x0` |
| `beq` at 0x21ff8 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x22028 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x22054 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x22080 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `bne` at 0x2209c | `[0x119a00] != 0x0` | `[0x119a00] == 0x0` |
| `bne` at 0x220ac | `ret(0x3ddc4) != 0x0` | `ret(0x3ddc4) == 0x0` |
| `bls` at 0x220c4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x220e0 | `ret(0x2e8fc) == 0x0` | `ret(0x2e8fc) != 0x0` |
| `beq` at 0x2210c | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x2214c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x2217c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x22198 | `[0x119a00] != 0x0` | `[0x119a00] == 0x0` |
| `bne` at 0x221a8 | `ret(0x3ddc4) != 0x0` | `ret(0x3ddc4) == 0x0` |
| `beq` at 0x221cc | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x221f8 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x22214 | `r0` | `?` |

## `IP41_ResetRelease`

- `IP41_ResetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2222c to 0x222b4.
  Established. The byte at absolute `0x119a04` receives `0x0` and later `0x1` on the longer entry
  path (`strb` at 0x22264, `strb` at 0x22288). Between those stores, the body calls `0xc2c9c`,
  `0x222b4` and `0xc2d08` (`bl` at 0x22268, `bl` at 0x22270, `bl` at 0x2227c). The call to `0x222b4`
  has first candidate `[sp-20]` and the rest unknown (`bl` at 0x22270). The last call targets
  `0x2a790` with candidates `0x1`, `0x1`, `0x119a04`, `0x1` (`bl` at 0x22294). The byte’s control
  role and merged return value are unknown (`ldmdb` at 0x222a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2223c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x22240 | load | 4 | `abs 0x222a8` | `-` |
| `ldrb` at 0x22244 | load | 1 | `abs 0x119a0c` | `-` |
| `ldr` at 0x22250 | load | 4 | `abs 0x222ac` | `-` |
| `str` at 0x22254 | store | 4 | `sp -0x20` | `0xfffffa24` |
| `ldr` at 0x2225c | load | 4 | `abs 0x222b0` | `-` |
| `strb` at 0x22264 | store | 1 | `abs 0x119a04` | `0x0` |
| `ldr` at 0x2226c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22278 | store | 4 | `sp -0x18` | `ret(0x222b4)` |
| `ldr` at 0x22280 | load | 4 | `abs 0x222b0` | `-` |
| `strb` at 0x22288 | store | 1 | `abs 0x119a04` | `0x1` |
| `ldr` at 0x22298 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2229c | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x222a0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x222a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x222a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x222a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22268 | `(unknown callee) 0xc2c9c` | `r0` | `r1` | `0x119a04` | `0x0` |
| `bl` at 0x22270 | `(unknown callee) 0x222b4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x2227c | `(unknown callee) 0xc2d08` | `ret(0x222b4)` | `?` | `?` | `ret(0x222b4)` |
| `bl` at 0x22294 | `(unknown callee) 0x2a790` | `0x1` | `0x1` | `0x119a04` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2224c | `[0x119a0c]:1 != 0x0` | `[0x119a0c]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x222a4 | `r0` | `?` |

## `IP41_reset_release`

- `IP41_reset_release`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x222b4 to 0x22710.
  Established. The opening call to `0x2f208` has candidates `0x29ef8`, `[sp-20]:1`, entry r2 and
  `[sp-20]:1` (`bl` at 0x222d4). A zero result takes the edge into further processing, while nonzero
  falls through to a word store of the result at `sp -0x74` (`beq` at 0x222e8, `str` at
  0x222f0). Later calls to `0x2f208` use first candidates `0x29a68` and `0x2a428`, both with second
  candidate `[sp-20]:1` (`bl` at 0x22448, `bl` at 0x2261c). The two calls to `0xbc308` share
  candidates `0x1`, `0x0`, `sp-112`, `sp-112` (`bl` at 0x2247c, `bl` at 0x226c0). The forwarded
  numeric addresses’ roles and reset-release effects are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x222c4 | store | 4 | `sp -0x14` | `r0` |
| `ldrb` at 0x222c8 | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0x222cc | load | 4 | `abs 0x226e4` | `-` |
| `str` at 0x222dc | store | 4 | `sp -0x18` | `ret(0x2f208)` |
| `ldr` at 0x222e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x222ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x222f0 | store | 4 | `sp -0x74` | `ret(0x2f208)` |
| `ldr` at 0x222f8 | load | 4 | `abs 0x226e8` | `-` |
| `ldr` at 0x222fc | load | 4 | `abs 0x119a00` | `-` |
| `str` at 0x2232c | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x22330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2233c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22340 | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `str` at 0x22358 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x2235c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22368 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2236c | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `str` at 0x2238c | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x22390 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2239c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x223a0 | store | 4 | `sp -0x74` | `ret(0xb7958)` |
| `str` at 0x223b8 | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x223bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x223c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x223cc | store | 4 | `sp -0x74` | `ret(0xb7958)` |
| `str` at 0x223ec | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x223f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x223fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22400 | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `str` at 0x22418 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x2241c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22428 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2242c | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `ldrb` at 0x2243c | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0x22440 | load | 4 | `abs 0x226ec` | `-` |
| `str` at 0x22450 | store | 4 | `sp -0x18` | `ret(0x2f208)` |
| `ldr` at 0x22454 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22460 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22464 | store | 4 | `sp -0x74` | `ret(0x2f208)` |
| `str` at 0x22494 | store | 4 | `sp -0x18` | `ret(0xb79a0)` |
| `ldr` at 0x22498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x224a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x224a8 | store | 4 | `sp -0x74` | `ret(0xb79a0)` |
| `str` at 0x224c4 | store | 4 | `sp -0x18` | `ret(0xb79a0)` |
| `ldr` at 0x224c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x224d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x224d8 | store | 4 | `sp -0x74` | `ret(0xb79a0)` |
| `ldr` at 0x224e0 | load | 4 | `abs 0x226e8` | `-` |
| `ldr` at 0x224e4 | load | 4 | `abs 0x119a00` | `-` |
| `strb` at 0x22510 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x22514 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x22524 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x2253c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x22558 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x2255c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22568 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2256c | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `ldrb` at 0x22574 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x2257c | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0x2259c | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x225a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x225ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x225b0 | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `str` at 0x225d0 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x225d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x225e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x225e4 | store | 4 | `sp -0x74` | `ret(0xb790c)` |
| `ldr` at 0x225ec | load | 4 | `abs 0x226f0` | `-` |
| `ldr` at 0x225f0 | load | 4 | `abs 0x115454` | `-` |
| `str` at 0x225fc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x22600 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x22610 | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0x22614 | load | 4 | `abs 0x226f4` | `-` |
| `str` at 0x22624 | store | 4 | `sp -0x18` | `ret(0x2f208)` |
| `ldr` at 0x22628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22640 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22648 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x22650 | load | 4 | `abs 0x226f8` | `-` |
| `ldr` at 0x22654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22658 | load | 4 | `abs 0x1199fc` | `-` |
| `ldr` at 0x22664 | load | 4 | `abs 0x226fc` | `-` |
| `str` at 0x22668 | store | 4 | `sp -0x84` | `0xd971c` |
| `ldr` at 0x2266c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22678 | load | 4 | `abs 0x22700` | `-` |
| `str` at 0x2267c | store | 4 | `sp -0x78` | `0xd974c` |
| `ldr` at 0x22684 | load | 4 | `abs 0x22704` | `-` |
| `str` at 0x22688 | store | 4 | `sp -0x78` | `0xd9754` |
| `ldr` at 0x2268c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x22690 | store | 4 | `sp -0x80` | `[sp-120]` |
| `ldr` at 0x22694 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22698 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x226a0 | load | 4 | `abs 0x22708` | `-` |
| `ldr` at 0x226a4 | load | 4 | `abs 0x2270c` | `-` |
| `ldr` at 0x226d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x226d8 | store | 4 | `sp -0x74` | `[sp-24]` |
| `ldr` at 0x226dc | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0x226e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x226e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x226e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x222d4 | `(unknown callee) 0x2f208` | `0x29ef8` | `[sp-20]:1` | `r2` | `[sp-20]:1` |
| `bl` at 0x22308 | `(unknown callee) 0x3ddc4` | `ret(0x2f208)` | `?` | `?` | `[0x119a00]` |
| `bl` at 0x22324 | `(unknown callee) 0xb790c` | `0xa` | `0x4` | `?` | `?` |
| `bl` at 0x22350 | `(unknown callee) 0xb790c` | `0xc` | `0x4` | `?` | `ret(0xb790c)` |
| `bl` at 0x22378 | `(unknown callee) 0x24228` | `0xa` | `?` | `?` | `ret(0xb790c)` |
| `bl` at 0x22384 | `(unknown callee) 0xb7958` | `0xa` | `0x4` | `?` | `?` |
| `bl` at 0x223b0 | `(unknown callee) 0xb7958` | `0xc` | `0x4` | `?` | `ret(0xb7958)` |
| `bl` at 0x223d8 | `(unknown callee) 0x24228` | `0xa` | `?` | `?` | `ret(0xb7958)` |
| `bl` at 0x223e4 | `(unknown callee) 0xb790c` | `0xa` | `0x4` | `?` | `?` |
| `bl` at 0x22410 | `(unknown callee) 0xb790c` | `0xc` | `0x4` | `?` | `ret(0xb790c)` |
| `bl` at 0x22438 | `(unknown callee) 0x24228` | `0x15` | `?` | `?` | `ret(0xb790c)` |
| `bl` at 0x22448 | `(unknown callee) 0x2f208` | `0x29a68` | `[sp-20]:1` | `?` | `[sp-20]:1` |
| `bl` at 0x2247c | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x2248c | `(unknown callee) 0xb79a0` | `0xa` | `0x88` | `0x88` | `?` |
| `bl` at 0x224bc | `(unknown callee) 0xb79a0` | `0xc` | `0x88` | `0x88` | `ret(0xb79a0)` |
| `bl` at 0x224f0 | `(unknown callee) 0x3ddc4` | `ret(0xb79a0)` | `?` | `?` | `[0x119a00]` |
| `bl` at 0x22508 | `(unknown callee) 0x24228` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x2252c | `(unknown callee) 0x2e8fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x22550 | `(unknown callee) 0xb790c` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x22588 | `(unknown callee) 0x24228` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x22594 | `(unknown callee) 0xb790c` | `0xa` | `0x1` | `?` | `?` |
| `bl` at 0x225bc | `(unknown callee) 0x24228` | `0x1` | `?` | `?` | `ret(0xb790c)` |
| `bl` at 0x225c8 | `(unknown callee) 0xb790c` | `0xc` | `0x1` | `?` | `?` |
| `bl` at 0x225f4 | `(unknown callee) 0x24228` | `[0x115454]` | `?` | `?` | `0x115454` |
| `bl` at 0x2261c | `(unknown callee) 0x2f208` | `0x2a428` | `[sp-20]:1` | `?` | `[sp-20]:1` |
| `bl` at 0x2263c | `(unknown callee) 0x24228` | `0xa` | `?` | `?` | `ret(0x2f208)` |
| `bl` at 0x226ac | `(unknown callee) 0x1eed0` | `0x4` | `0xd9624` | `0x2da` | `0x0` |
| `bl` at 0x226c0 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x226c8 | `(unknown callee) 0x2a880` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x226d0 | `(unknown callee) 0x2a8a8` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x222e8 | `ret(0x2f208) == 0x0` | `ret(0x2f208) != 0x0` |
| `bne` at 0x22304 | `[0x119a00] != 0x0` | `[0x119a00] == 0x0` |
| `bne` at 0x22314 | `ret(0x3ddc4) != 0x0` | `ret(0x3ddc4) == 0x0` |
| `beq` at 0x22338 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x22364 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x22398 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x223c4 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x223f8 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x22424 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x2245c | `ret(0x2f208) == 0x0` | `ret(0x2f208) != 0x0` |
| `beq` at 0x224a0 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x224d0 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x224ec | `[0x119a00] != 0x0` | `[0x119a00] == 0x0` |
| `bne` at 0x224fc | `ret(0x3ddc4) != 0x0` | `ret(0x3ddc4) == 0x0` |
| `bls` at 0x2251c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x22538 | `ret(0x2e8fc) == 0x0` | `ret(0x2e8fc) != 0x0` |
| `beq` at 0x22564 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x225a8 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x225dc | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `ble` at 0x22608 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x22630 | `ret(0x2f208) != 0x0` | `ret(0x2f208) == 0x0` |
| `ble` at 0x22660 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x22674 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x226e0 | `r0` | `?` |

## `IP41_SoftReset`

- `IP41_SoftReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22710 to 0x22880.
  Established. An entry value equal to `0xff` takes the path to a byte store of `0x3` at `sp -0x21`,
  while other values fall through to an unknown-value byte store there (`beq` at 0x2272c, `strb`
  at 0x22748, `strb` at 0x2273c). Calls to `0xb7958` use candidates `0xa`, `0x2`, unknown, unknown
  and `0xc`, `0x2`, unknown, unknown (`bl` at 0x22788, `bl` at 0x227c4). Later calls to `0xb790c`
  have those same respective first two candidates (`bl` at 0x22808, `bl` at 0x22844). The branch
  predicates selecting the first pair retain only flags, and the later pair’s tested values are
  unknown (`beq` at 0x2277c, `beq` at 0x227b8, `beq` at 0x227fc, `beq` at 0x22838). No physical
  reset timing or mask interpretation is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x22720 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x22724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22734 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2273c | store | 1 | `sp -0x21` | `?` |
| `strb` at 0x22748 | store | 1 | `sp -0x21` | `0x3` |
| `ldrb` at 0x2274c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x22750 | store | 1 | `sp -0x19` | `?` |
| `ldr` at 0x22754 | load | 4 | `abs 0x22878` | `-` |
| `ldrb` at 0x22758 | load | 1 | `abs 0x119a0c` | `-` |
| `ldr` at 0x22764 | load | 4 | `abs 0x2287c` | `-` |
| `str` at 0x22768 | store | 4 | `sp -0x20` | `0xfffffa24` |
| `ldrb` at 0x22770 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x22790 | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x22794 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x227a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x227a4 | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `ldrb` at 0x227ac | load | 1 | `sp -0x19` | `-` |
| `str` at 0x227cc | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x227d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x227dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x227e0 | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `ldrb` at 0x227f0 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x22810 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x22814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22820 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22824 | store | 4 | `sp -0x20` | `ret(0xb790c)` |
| `ldrb` at 0x2282c | load | 1 | `sp -0x19` | `-` |
| `str` at 0x2284c | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x22850 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2285c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22860 | store | 4 | `sp -0x20` | `ret(0xb790c)` |
| `str` at 0x2286c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x22870 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x22874 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x22874 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x22874 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22788 | `(unknown callee) 0xb7958` | `0xa` | `0x2` | `?` | `?` |
| `bl` at 0x227c4 | `(unknown callee) 0xb7958` | `0xc` | `0x2` | `?` | `?` |
| `bl` at 0x227ec | `(unknown callee) 0x24228` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x22808 | `(unknown callee) 0xb790c` | `0xa` | `0x2` | `?` | `?` |
| `bl` at 0x22844 | `(unknown callee) 0xb790c` | `0xc` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2272c | `r0 == 0xff` | `r0 != 0xff` |
| `bne` at 0x22760 | `[0x119a0c]:1 != 0x0` | `[0x119a0c]:1 == 0x0` |
| `beq` at 0x2277c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x2279c | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x227b8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x227d8 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x227fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2281c | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x22838 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x22858 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x22874 | `r0` | `?` |

## `IP57_ResetHold`

- `IP57_ResetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d770 to 0x4d9f0.
  Established. The opening call to `0x4e270` has candidates `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 &
  0xff)` (`bl` at 0x4d798). A zero result takes the edge into a byte store of `([[sp-24]+2]:1+2 &
  0xff)` at `sp -0x1d`, while nonzero falls through to saving the result at `sp -0x24` (`beq` at
  0x4d7ac, `strb` at 0x4d7c8, `str` at 0x4d7b4). An explicit-link call has unknown target and
  candidates unknown, `[[sp-24]+12]:1`, `[0x11584c]`, `[[sp-24]+12]:1` (`mov` at 0x4d7f8). Later
  calls to `0xb79a0` use `[sp-29]:1`, `0xc`, `0x8`, `[sp-29]:1` and `[sp-29]:1`, `0xc0`, `0x0`,
  `[sp-29]:1` (`bl` at 0x4d88c, `bl` at 0x4d988). The selector and electrical hold semantics are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d784 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4d788 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d7a0 | store | 4 | `sp -0x1c` | `ret(0x4e270)` |
| `ldr` at 0x4d7a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d7b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d7b4 | store | 4 | `sp -0x24` | `ret(0x4e270)` |
| `ldr` at 0x4d7bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4d7c0 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x4d7c8 | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x4d7cc | load | 4 | `abs 0x4d9ec` | `-` |
| `ldr` at 0x4d7d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4d7d4 | load | 1 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x4d7e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4d7e4 | load | 1 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x4d7e8 | load | 4 | `abs 0x11584c` | `-` |
| `ldr` at 0x4d844 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d848 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldrb` at 0x4d850 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d860 | store | 4 | `sp -0x1c` | `ret(0x29ed4)` |
| `ldr` at 0x4d864 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d870 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d874 | store | 4 | `sp -0x24` | `ret(0x29ed4)` |
| `ldrb` at 0x4d87c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4d894 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x4d898 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d8a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d8a8 | store | 4 | `sp -0x24` | `ret(0xb79a0)` |
| `ldrb` at 0x4d8b0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4d8c4 | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x4d8c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d8d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d8d8 | store | 4 | `sp -0x24` | `ret(0xb790c)` |
| `strb` at 0x4d8e4 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x4d8e8 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x4d8f8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d904 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x4d924 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4d928 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x4d92c | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x4d94c | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x4d950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d95c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d960 | store | 4 | `sp -0x24` | `ret(0xb7958)` |
| `ldrb` at 0x4d968 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x4d970 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x4d978 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4d990 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x4d994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d9a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d9a4 | store | 4 | `sp -0x24` | `ret(0xb79a0)` |
| `ldrb` at 0x4d9ac | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4d9c0 | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x4d9c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4d9d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4d9d4 | store | 4 | `sp -0x24` | `ret(0xb7958)` |
| `str` at 0x4d9e0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4d9e4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4d9e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d9e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d9e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d798 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x4d7f8 | `(unknown callee) (unknown target)` | `?` | `[[sp-24]+12]:1` | `[0x11584c]` | `[[sp-24]+12]:1` |
| `bl` at 0x4d800 | `(unknown callee) 0x2a880` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x4d808 | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x4d80c | `(unknown callee) 0x3a8c0` | `ret(0x4e310)` | `?` | `?` | `?` |
| `bl` at 0x4d81c | `(unknown callee) 0x2b31c` | `ret(0x3a8c0)` | `?` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x4d834 | `(unknown callee) 0xc478c` | `?` | `sp-28` | `sp-28` | `?` |
| `bl` at 0x4d858 | `(unknown callee) 0x29ed4` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4d88c | `(unknown callee) 0xb79a0` | `[sp-29]:1` | `0xc` | `0x8` | `[sp-29]:1` |
| `bl` at 0x4d8bc | `(unknown callee) 0xb790c` | `[sp-29]:1` | `0x10` | `?` | `[sp-29]:1` |
| `bl` at 0x4d914 | `(unknown callee) 0x2e8fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x4d944 | `(unknown callee) 0xb7958` | `?` | `0x4` | `[[sp-24]+2]:1` | `?` |
| `bl` at 0x4d988 | `(unknown callee) 0xb79a0` | `[sp-29]:1` | `0xc0` | `0x0` | `[sp-29]:1` |
| `bl` at 0x4d9b8 | `(unknown callee) 0xb7958` | `[sp-29]:1` | `0x2` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d7ac | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `beq` at 0x4d818 | `ret(0x3a8c0) == 0x0` | `ret(0x3a8c0) != 0x0` |
| `bne` at 0x4d840 | `ret(0xc478c) != 0x0` | `ret(0xc478c) == 0x0` |
| `beq` at 0x4d86c | `ret(0x29ed4) == 0x0` | `ret(0x29ed4) != 0x0` |
| `beq` at 0x4d8a0 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x4d8d0 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `bls` at 0x4d8f0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x4d920 | `ret(0x2e8fc) == 0x0` | `ret(0x2e8fc) != 0x0` |
| `beq` at 0x4d958 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x4d99c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x4d9cc | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d9e8 | `r0` | `?` |

## `IP57_ResetRelease`

- `IP57_ResetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d9f0 to 0x4ddcc.
  Established. A call to `0x4e270` uses candidates `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x4da18). The longer path includes an explicit-link call with unknown target and
  candidates unknown, `[[sp-24]+12]:1`, `[0x11584c]`, `[[sp-24]+12]:1` (`mov` at 0x4da78). Calls to
  `0x4e310` include first candidates `0xa`, `0x15`, `0x3` and `0x1`, each with remaining values
  retained in the table (`bl` at 0x4dadc, `bl` at 0x4db4c, `bl` at 0x4dbc4, `bl` at 0x4dc64). The
  call to `0xbc308` appears twice with `0x1`, `0x0`, `sp-112`, `sp-112` (`bl` at 0x4db8c, `bl` at
  0x4dd90). A later branch takes its edge for `ret(0x2a428) != 0x0` and falls through for equality
  (`bne` at 0x4dcd8). Release operations and candidate units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4da04 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4da08 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4da20 | store | 4 | `sp -0x1c` | `ret(0x4e270)` |
| `ldr` at 0x4da24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4da30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4da34 | store | 4 | `sp -0x78` | `ret(0x4e270)` |
| `ldr` at 0x4da3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4da40 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x4da48 | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x4da4c | load | 4 | `abs 0x4ddac` | `-` |
| `ldr` at 0x4da50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4da54 | load | 1 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x4da60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4da64 | load | 1 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x4da68 | load | 4 | `abs 0x11584c` | `-` |
| `ldrb` at 0x4da7c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4da8c | store | 4 | `sp -0x1c` | `ret(0x29ef8)` |
| `ldr` at 0x4da90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4da9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4daa0 | store | 4 | `sp -0x78` | `ret(0x29ef8)` |
| `ldrb` at 0x4daa8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4dabc | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x4dac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dacc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dad0 | store | 4 | `sp -0x78` | `ret(0xb790c)` |
| `ldrb` at 0x4dae0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4daf4 | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x4daf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4db08 | store | 4 | `sp -0x78` | `ret(0xb7958)` |
| `ldrb` at 0x4db18 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4db2c | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x4db30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4db40 | store | 4 | `sp -0x78` | `ret(0xb790c)` |
| `ldrb` at 0x4db50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4db60 | store | 4 | `sp -0x1c` | `ret(0x29a68)` |
| `ldr` at 0x4db64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4db70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4db74 | store | 4 | `sp -0x78` | `ret(0x29a68)` |
| `ldrb` at 0x4db90 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4dba4 | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x4dba8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dbb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dbb8 | store | 4 | `sp -0x78` | `ret(0xb790c)` |
| `strb` at 0x4dbcc | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x4dbd0 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x4dbe0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4dbec | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x4dc0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4dc10 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x4dc14 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x4dc34 | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x4dc38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dc44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dc48 | store | 4 | `sp -0x78` | `ret(0xb790c)` |
| `ldrb` at 0x4dc50 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x4dc58 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x4dc68 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x4dc7c | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0x4dc80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dc8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dc90 | store | 4 | `sp -0x78` | `ret(0xb790c)` |
| `ldr` at 0x4dc98 | load | 4 | `abs 0x4ddb0` | `-` |
| `ldr` at 0x4dc9c | load | 4 | `abs 0x115454` | `-` |
| `str` at 0x4dca8 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x4dcac | load | 4 | `sp -0x74` | `-` |
| `ldrb` at 0x4dcbc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4dccc | store | 4 | `sp -0x1c` | `ret(0x2a428)` |
| `ldr` at 0x4dcd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dce8 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x4dcf0 | store | 4 | `sp -0x74` | `[sp-116]+1` |
| `ldr` at 0x4dcf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dd04 | load | 4 | `abs 0x4ddb4` | `-` |
| `ldr` at 0x4dd08 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x4dd0c | load | 4 | `abs 0x1199fc` | `-` |
| `ldr` at 0x4dd1c | load | 4 | `abs 0x4ddb8` | `-` |
| `str` at 0x4dd20 | store | 4 | `sp -0x88` | `0xe39d8` |
| `ldr` at 0x4dd24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dd30 | load | 4 | `abs 0x4ddbc` | `-` |
| `str` at 0x4dd34 | store | 4 | `sp -0x7c` | `0xe3a08` |
| `ldr` at 0x4dd3c | load | 4 | `abs 0x4ddc0` | `-` |
| `str` at 0x4dd40 | store | 4 | `sp -0x7c` | `0xe3a10` |
| `ldr` at 0x4dd44 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x4dd48 | store | 4 | `sp -0x84` | `[sp-124]` |
| `ldr` at 0x4dd4c | load | 4 | `sp -0x74` | `-` |
| `str` at 0x4dd50 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0x4dd58 | load | 4 | `abs 0x4ddc4` | `-` |
| `ldr` at 0x4dd5c | load | 4 | `abs 0x4ddc8` | `-` |
| `ldr` at 0x4dd68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dd74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dd78 | store | 4 | `sp -0x78` | `[sp-28]` |
| `str` at 0x4dda0 | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x4dda4 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x4dda8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4dda8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4dda8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4da18 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x4da78 | `(unknown callee) (unknown target)` | `?` | `[[sp-24]+12]:1` | `[0x11584c]` | `[[sp-24]+12]:1` |
| `bl` at 0x4da84 | `(unknown callee) 0x29ef8` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4dab4 | `(unknown callee) 0xb790c` | `[sp-29]:1` | `0x2` | `?` | `[sp-29]:1` |
| `bl` at 0x4dadc | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `ret(0xb790c)` |
| `bl` at 0x4daec | `(unknown callee) 0xb7958` | `[sp-29]:1` | `0x2` | `?` | `[sp-29]:1` |
| `bl` at 0x4db14 | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `ret(0xb7958)` |
| `bl` at 0x4db24 | `(unknown callee) 0xb790c` | `[sp-29]:1` | `0x2` | `?` | `[sp-29]:1` |
| `bl` at 0x4db4c | `(unknown callee) 0x4e310` | `0x15` | `?` | `?` | `ret(0xb790c)` |
| `bl` at 0x4db58 | `(unknown callee) 0x29a68` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4db8c | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x4db9c | `(unknown callee) 0xb790c` | `[sp-29]:1` | `0xc0` | `?` | `[sp-29]:1` |
| `bl` at 0x4dbc4 | `(unknown callee) 0x4e310` | `0x3` | `?` | `?` | `ret(0xb790c)` |
| `bl` at 0x4dbfc | `(unknown callee) 0x2e8fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x4dc2c | `(unknown callee) 0xb790c` | `?` | `0x4` | `[[sp-24]+2]:1` | `?` |
| `bl` at 0x4dc64 | `(unknown callee) 0x4e310` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x4dc74 | `(unknown callee) 0xb790c` | `[sp-29]:1` | `0x4` | `?` | `[sp-29]:1` |
| `bl` at 0x4dca0 | `(unknown callee) 0x4e310` | `[0x115454]` | `?` | `?` | `0x115454` |
| `bl` at 0x4dcc4 | `(unknown callee) 0x2a428` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4dce4 | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `ret(0x2a428)` |
| `bl` at 0x4dd64 | `(unknown callee) 0x1eed0` | `0x4` | `0xe3930` | `0x3b6` | `0x0` |
| `bl` at 0x4dd90 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x4dd98 | `(unknown callee) 0x2a880` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4da2c | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `beq` at 0x4da98 | `ret(0x29ef8) == 0x0` | `ret(0x29ef8) != 0x0` |
| `beq` at 0x4dac8 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x4db00 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x4db38 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x4db6c | `ret(0x29a68) == 0x0` | `ret(0x29a68) != 0x0` |
| `beq` at 0x4dbb0 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `bls` at 0x4dbd8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x4dc08 | `ret(0x2e8fc) == 0x0` | `ret(0x2e8fc) != 0x0` |
| `beq` at 0x4dc40 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `beq` at 0x4dc88 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `ble` at 0x4dcb4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4dcd8 | `ret(0x2a428) != 0x0` | `ret(0x2a428) == 0x0` |
| `bne` at 0x4dd00 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bgt` at 0x4dd14 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x4dd2c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x4dd70 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4dda8 | `r0` | `?` |

## `IP57_SoftReset`

- `IP57_SoftReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ddcc to 0x4dea0.
  Established. The body stores byte `([[sp-24]+2]:1+2 & 0xff)` at `sp -0x19` (`strb` at 0x4de24).
  The subsequent call to `0xb7958` has that expression as first and fourth candidates, second `0x8`
  and third unknown (`bl` at 0x4de34). A zero result takes the edge to a call to `0x4e310` with
  first candidate `0xa`, while nonzero falls through to storing the result at `sp -0x24` (`beq`
  at 0x4de48, `bl` at 0x4de5c, `str` at 0x4de50). A later call to `0xb790c` has candidates
  `[sp-25]:1`, `0x8`, unknown, `[sp-25]:1` (`bl` at 0x4de6c). The numeric selector’s soft-reset
  action is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4dde0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4dde4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4ddfc | store | 4 | `sp -0x20` | `ret(0x4e270)` |
| `ldr` at 0x4de00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4de0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4de10 | store | 4 | `sp -0x24` | `ret(0x4e270)` |
| `ldr` at 0x4de18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4de1c | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x4de24 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x4de28 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x4de3c | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `ldr` at 0x4de40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4de4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4de50 | store | 4 | `sp -0x24` | `ret(0xb7958)` |
| `ldrb` at 0x4de60 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x4de74 | store | 4 | `sp -0x20` | `ret(0xb790c)` |
| `ldr` at 0x4de78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4de84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4de88 | store | 4 | `sp -0x24` | `ret(0xb790c)` |
| `str` at 0x4de94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4de98 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4de9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4de9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4de9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4ddf4 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4de34 | `(unknown callee) 0xb7958` | `([[sp-24]+2]:1+2 & 0xff)` | `0x8` | `?` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x4de5c | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `ret(0xb7958)` |
| `bl` at 0x4de6c | `(unknown callee) 0xb790c` | `[sp-25]:1` | `0x8` | `?` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4de08 | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `beq` at 0x4de48 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x4de80 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4de9c | `r0` | `?` |

## `NL4R_ResetHold`

- `NL4R_ResetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32eec to 0x3301c.
  Established. A byte store clears absolute `0x119d79` on the longer entry path (`strb` at 0x32f2c).
  The selector comparison takes the edge for `[sp-20] != 0xff` and falls through for equality (`bne`
  at 0x32f38). On the taken path, a signed comparison takes the edge for `[sp-20] < [0x119d80]` and
  falls through otherwise (`blt` at 0x32fcc). Both numeric calls target `0x33510` with first
  candidate unknown, second `0x119d7c`, and third and fourth unknown (`bl` at 0x32f74, `bl` at
  0x32ff4). The selector-to-device mapping and hold operation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x32efc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x32f04 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x32f08 | load | 4 | `abs 0x33008` | `-` |
| `ldrb` at 0x32f0c | load | 1 | `abs 0x119d78` | `-` |
| `ldr` at 0x32f18 | load | 4 | `abs 0x3300c` | `-` |
| `str` at 0x32f1c | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x32f24 | load | 4 | `abs 0x33010` | `-` |
| `strb` at 0x32f2c | store | 1 | `abs 0x119d79` | `0x0` |
| `ldr` at 0x32f30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x32f40 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x32f44 | load | 4 | `abs 0x33014` | `-` |
| `ldr` at 0x32f48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x32f4c | load | 4 | `abs 0x119d80` | `-` |
| `ldr` at 0x32f5c | load | 4 | `abs 0x33018` | `-` |
| `ldr` at 0x32f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x32f68 | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x32f7c | store | 4 | `sp -0x1c` | `ret(0x33510)` |
| `ldr` at 0x32f80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x32f98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x32f9c | store | 4 | `sp -0x18` | `ret(0x33510)` |
| `ldr` at 0x32fa0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x32fa8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x32fb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x32fb4 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x32fbc | load | 4 | `abs 0x33014` | `-` |
| `ldr` at 0x32fc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x32fc4 | load | 4 | `abs 0x119d80` | `-` |
| `str` at 0x32fd4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x32fdc | load | 4 | `abs 0x33018` | `-` |
| `ldr` at 0x32fe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x32fe8 | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x32ffc | store | 4 | `sp -0x20` | `ret(0x33510)` |
| `ldr` at 0x33000 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x33004 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x33004 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x33004 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x32f74 | `(unknown callee) 0x33510` | `?` | `0x119d7c` | `?` | `?` |
| `bl` at 0x32ff4 | `(unknown callee) 0x33510` | `?` | `0x119d7c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x32f14 | `[0x119d78]:1 != 0x0` | `[0x119d78]:1 == 0x0` |
| `bne` at 0x32f38 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x32f54 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x32f88 | `ret(0x33510) == 0x0` | `ret(0x33510) != 0x0` |
| `bne` at 0x32f94 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x32fcc | `[sp-20] < [0x119d80] (signed, 32-bit)` | `[sp-20] >= [0x119d80] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x33004 | `r0` | `?` |

## `NL4R_ResetRelease`

- `NL4R_ResetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3301c to 0x33190.
  Established. The longer path clears byte `0x119d79`, calls `0xc2c9c`, and later calls `0xc2d08`
  before setting that byte to `0x1` (`strb` at 0x3305c, `bl` at 0x33060, `bl` at 0x33140, `strb` at
  0x3314c). Two intervening calls to `0x3368c` have all candidates unknown (`bl` at 0x330b4, `bl` at
  0x33134). The final conditional branch takes its bypass edge for `? == 0x40` and falls through
  to a call to `0x332b0` for inequality (`beq` at 0x33160, `bl` at 0x33168). That call has first
  candidate `[sp-20]`, with the others unknown (`bl` at 0x33168). The absolute byte’s policy and
  electrical release sequence are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3302c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x33034 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x33038 | load | 4 | `abs 0x3317c` | `-` |
| `ldrb` at 0x3303c | load | 1 | `abs 0x119d78` | `-` |
| `ldr` at 0x33048 | load | 4 | `abs 0x33180` | `-` |
| `str` at 0x3304c | store | 4 | `sp -0x24` | `0xfffffed4` |
| `ldr` at 0x33054 | load | 4 | `abs 0x33184` | `-` |
| `strb` at 0x3305c | store | 1 | `abs 0x119d79` | `0x0` |
| `ldr` at 0x33064 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33074 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x33078 | load | 4 | `abs 0x33188` | `-` |
| `ldr` at 0x3307c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33080 | load | 4 | `abs 0x119d80` | `-` |
| `ldr` at 0x33090 | load | 4 | `abs 0x3318c` | `-` |
| `ldr` at 0x33094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3309c | load | 4 | `abs 0x119d7c` | `-` |
| `ldr` at 0x330a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x330bc | store | 4 | `sp -0x20` | `ret(0x3368c)` |
| `ldr` at 0x330c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x330cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x330d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x330dc | store | 4 | `sp -0x1c` | `ret(0x3368c)` |
| `ldr` at 0x330e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x330e8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x330f0 | load | 4 | `abs 0x33188` | `-` |
| `ldr` at 0x330f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x330f8 | load | 4 | `abs 0x119d80` | `-` |
| `str` at 0x33108 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x33110 | load | 4 | `abs 0x3318c` | `-` |
| `ldr` at 0x33114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3311c | load | 4 | `abs 0x119d7c` | `-` |
| `ldr` at 0x33124 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3313c | store | 4 | `sp -0x1c` | `ret(0x3368c)` |
| `ldr` at 0x33144 | load | 4 | `abs 0x33184` | `-` |
| `strb` at 0x3314c | store | 1 | `abs 0x119d79` | `0x1` |
| `ldr` at 0x33164 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3316c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x33170 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x33174 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x33178 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x33178 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x33178 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x33060 | `(unknown callee) 0xc2c9c` | `r0` | `r1` | `0x119d79` | `0x0` |
| `bl` at 0x330b4 | `(unknown callee) 0x3368c` | `?` | `?` | `?` | `?` |
| `bl` at 0x33134 | `(unknown callee) 0x3368c` | `?` | `?` | `?` | `?` |
| `bl` at 0x33140 | `(unknown callee) 0xc2d08` | `?` | `?` | `?` | `?` |
| `bl` at 0x33150 | `(unknown callee) 0x2b31c` | `ret(0xc2d08)` | `?` | `0x119d79` | `0x1` |
| `bl` at 0x33168 | `(unknown callee) 0x332b0` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x33044 | `[0x119d78]:1 != 0x0` | `[0x119d78]:1 == 0x0` |
| `bne` at 0x3306c | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x33088 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x330c8 | `ret(0x3368c) == 0x0` | `ret(0x3368c) != 0x0` |
| `bne` at 0x330d4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x33100 | `[sp-20] < [0x119d80] (signed, 32-bit)` | `[sp-20] >= [0x119d80] (signed, 32-bit)` |
| `beq` at 0x33160 | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x33178 | `r0` | `?` |

## `NL4R_SoftReset`

- `NL4R_SoftReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x33190 to 0x332b0.
  Established. A nonzero byte at absolute `0x119d78` takes the edge into selection, while zero falls
  through to word `0xfffffed4` at `sp -0x20` (`ldrb` at 0x331b0, `bne` at 0x331b8, `str` at
  0x331c0). The next inequality branch takes the single-selection path for `r0 != 0xff`, while
  equality falls through into the loop (`bne` at 0x331d0). Both paths call `0x339a0` with first
  candidate unknown, second `0x119d7c` and the rest unknown (`bl` at 0x3320c, `bl` at 0x3328c). The
  signed single-selection comparison takes the edge for `r0 < [0x119d80]` and falls through
  otherwise (`blt` at 0x33264). The loop’s exact bound and the callee’s physical effect remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x331a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x331a8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x331ac | load | 4 | `abs 0x332a0` | `-` |
| `ldrb` at 0x331b0 | load | 1 | `abs 0x119d78` | `-` |
| `ldr` at 0x331bc | load | 4 | `abs 0x332a4` | `-` |
| `str` at 0x331c0 | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x331c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x331d8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x331dc | load | 4 | `abs 0x332a8` | `-` |
| `ldr` at 0x331e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x331e4 | load | 4 | `abs 0x119d80` | `-` |
| `ldr` at 0x331f4 | load | 4 | `abs 0x332ac` | `-` |
| `ldr` at 0x331f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33200 | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x33214 | store | 4 | `sp -0x1c` | `ret(0x339a0)` |
| `ldr` at 0x33218 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x33224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33230 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x33234 | store | 4 | `sp -0x18` | `ret(0x339a0)` |
| `ldr` at 0x33238 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33240 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x33248 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3324c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x33254 | load | 4 | `abs 0x332a8` | `-` |
| `ldr` at 0x33258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3325c | load | 4 | `abs 0x119d80` | `-` |
| `str` at 0x3326c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x33274 | load | 4 | `abs 0x332ac` | `-` |
| `ldr` at 0x33278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33280 | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x33294 | store | 4 | `sp -0x20` | `ret(0x339a0)` |
| `ldr` at 0x33298 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3329c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3329c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3329c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3320c | `(unknown callee) 0x339a0` | `?` | `0x119d7c` | `?` | `?` |
| `bl` at 0x3328c | `(unknown callee) 0x339a0` | `?` | `0x119d7c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x331b8 | `[0x119d78]:1 != 0x0` | `[0x119d78]:1 == 0x0` |
| `bne` at 0x331d0 | `r0 != 0xff` | `r0 == 0xff` |
| `blt` at 0x331ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x33220 | `ret(0x339a0) == 0x0` | `ret(0x339a0) != 0x0` |
| `bne` at 0x3322c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x33264 | `r0 < [0x119d80] (signed, 32-bit)` | `r0 >= [0x119d80] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3329c | `r0` | `?` |

## `TIOResetHold`

- `TIOResetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x457c8 to 0x45994.
  Established. The longer path clears byte `0x119e7c` (`strb` at 0x45808); its calls include
  `0xb79a0` with candidates `[[sp-24]]:1`, `0x60`, `0x0`, `[[sp-24]]:1`, followed by `0xb7958` with
  `[[sp-24]]:1`, `0x4`, unknown, `[[sp-24]]:1` (`bl` at 0x458c8, `bl` at 0x45904). After the loop,
  `0x49c84` is called with candidates `0xff`, `0xff`, `[0x2811d8]:1`, unknown (`bl` at 0x45948). A
  nonzero result falls through its equality branch to further testing, while zero takes the
  bypass (`beq` at 0x4595c). The absolute flag and the callees’ reset effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x457d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x457e0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x457e4 | load | 4 | `abs 0x45984` | `-` |
| `ldr` at 0x457e8 | load | 4 | `abs 0x119e78` | `-` |
| `ldr` at 0x457f4 | load | 4 | `abs 0x45988` | `-` |
| `str` at 0x457f8 | store | 4 | `sp -0x2c` | `0xfffff768` |
| `ldr` at 0x45800 | load | 4 | `abs 0x4598c` | `-` |
| `strb` at 0x45808 | store | 1 | `abs 0x119e7c` | `0x0` |
| `str` at 0x45810 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x45814 | load | 4 | `abs 0x45990` | `-` |
| `ldrb` at 0x45818 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x4581c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4582c | load | 4 | `abs 0x45984` | `-` |
| `ldr` at 0x45830 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45838 | load | 4 | `abs 0x119e78` | `-` |
| `str` at 0x45840 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x45844 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x45848 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4585c | store | 4 | `sp -0x20` | `ret(0xb7c2c)` |
| `ldr` at 0x45860 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4586c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45878 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4587c | store | 4 | `sp -0x1c` | `ret(0xb7c2c)` |
| `ldr` at 0x45880 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45890 | store | 4 | `sp -0x20` | `ret(0x47a08)` |
| `ldr` at 0x45894 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x458a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x458ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x458b0 | store | 4 | `sp -0x1c` | `ret(0x47a08)` |
| `ldr` at 0x458b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x458b8 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x458d0 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `ldr` at 0x458d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x458e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x458ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x458f0 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x458f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x458f8 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x4590c | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `ldr` at 0x45910 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4591c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45928 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4592c | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0x45930 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x45938 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x45950 | store | 4 | `sp -0x20` | `ret(0x49c84)` |
| `ldr` at 0x45954 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x45960 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4596c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x45970 | store | 4 | `sp -0x1c` | `ret(0x49c84)` |
| `ldr` at 0x45974 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45978 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x4597c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x45980 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45980 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45980 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45854 | `(unknown callee) 0xb7c2c` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x45888 | `(unknown callee) 0x47a08` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x458c8 | `(unknown callee) 0xb79a0` | `[[sp-24]]:1` | `0x60` | `0x0` | `[[sp-24]]:1` |
| `bl` at 0x45904 | `(unknown callee) 0xb7958` | `[[sp-24]]:1` | `0x4` | `?` | `[[sp-24]]:1` |
| `bl` at 0x45948 | `(unknown callee) 0x49c84` | `0xff` | `0xff` | `[0x2811d8]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x457f0 | `[0x119e78] != 0x0` | `[0x119e78] == 0x0` |
| `blt` at 0x45824 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x45868 | `ret(0xb7c2c) == 0x0` | `ret(0xb7c2c) != 0x0` |
| `bne` at 0x45874 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4589c | `ret(0x47a08) == 0x0` | `ret(0x47a08) != 0x0` |
| `bne` at 0x458a8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x458dc | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x458e8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x45918 | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `bne` at 0x45924 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4595c | `ret(0x49c84) == 0x0` | `ret(0x49c84) != 0x0` |
| `bne` at 0x45968 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45980 | `r0` | `?` |

## `TIOResetRelease`

- `TIOResetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45994 to 0x45bc0.
  Established. A nonzero result from `0x3a8c0` takes the edge into further calls, while zero falls
  through to a word store of `0x0` at `sp -0x20` (`bne` at 0x459d0, `str` at 0x459d8). The body
  includes calls to `0x46894` and `0x46e0c`, each with first and fourth candidates `[sp-28]:1`,
  second and third unknown (`bl` at 0x45a30, `bl` at 0x45ab8). A later call to `0x49ef4` uses
  `0xff`, `0xff`, `[0x2811d8]:1`, unknown (`bl` at 0x45af8). On its zero-result path, byte `0x1` is
  stored at absolute `0x119e7c` (`beq` at 0x45b0c, `strb` at 0x45b24). A later call to `0xb7cbc` has
  candidates unknown, `0x2`, `0x2`, unknown (`bl` at 0x45b6c). The reset-release meaning of these
  operations remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x459a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x459a8 | load | 4 | `abs 0x45bb0` | `-` |
| `ldr` at 0x459ac | load | 4 | `abs 0x119e78` | `-` |
| `ldr` at 0x459b8 | load | 4 | `abs 0x45bb4` | `-` |
| `str` at 0x459bc | store | 4 | `sp -0x20` | `0xfffff768` |
| `str` at 0x459d8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x459e8 | store | 4 | `sp -0x18` | `ret(0x4a384)` |
| `ldr` at 0x459ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x459f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x459fc | store | 4 | `sp -0x20` | `ret(0x4a384)` |
| `str` at 0x45a08 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x45a0c | load | 4 | `abs 0x45bb8` | `-` |
| `ldrb` at 0x45a10 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45a14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45a28 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x45a38 | store | 4 | `sp -0x18` | `ret(0x46894)` |
| `ldr` at 0x45a40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45a4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45a50 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x45a58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45a60 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x45a70 | store | 4 | `sp -0x18` | `ret(0x4a430)` |
| `ldr` at 0x45a74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45a80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45a84 | store | 4 | `sp -0x20` | `ret(0x4a430)` |
| `str` at 0x45a90 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x45a94 | load | 4 | `abs 0x45bb8` | `-` |
| `ldrb` at 0x45a98 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45a9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45ab0 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x45ac0 | store | 4 | `sp -0x18` | `ret(0x46e0c)` |
| `ldr` at 0x45ac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45ad4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45ad8 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x45ae0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45ae8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x45b00 | store | 4 | `sp -0x18` | `ret(0x49ef4)` |
| `ldr` at 0x45b04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45b10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45b14 | store | 4 | `sp -0x20` | `ret(0x49ef4)` |
| `ldr` at 0x45b1c | load | 4 | `abs 0x45bbc` | `-` |
| `strb` at 0x45b24 | store | 1 | `abs 0x119e7c` | `0x1` |
| `str` at 0x45b2c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x45b30 | load | 4 | `abs 0x45bb8` | `-` |
| `ldrb` at 0x45b34 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45b38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b50 | load | 4 | `abs 0x45bb0` | `-` |
| `ldr` at 0x45b54 | load | 4 | `abs 0x119e78` | `-` |
| `ldrb` at 0x45b5c | load | 1 | `(unknown)` | `-` |
| `str` at 0x45b74 | store | 4 | `sp -0x18` | `ret(0xb7cbc)` |
| `ldr` at 0x45b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45b84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45b88 | store | 4 | `sp -0x20` | `ret(0xb7cbc)` |
| `ldr` at 0x45b90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45b98 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x45ba4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x45ba8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x45bac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45bac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45bac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x459c4 | `(unknown callee) 0x3a8c0` | `r0` | `r1` | `r2` | `[0x119e78]` |
| `bl` at 0x459e0 | `(unknown callee) 0x4a384` | `ret(0x3a8c0)` | `?` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x45a24 | `(unknown callee) 0xc2c9c` | `?` | `?` | `[0x2811d8]:1` | `?` |
| `bl` at 0x45a30 | `(unknown callee) 0x46894` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x45a3c | `(unknown callee) 0xc2d08` | `ret(0x46894)` | `?` | `?` | `ret(0x46894)` |
| `bl` at 0x45a68 | `(unknown callee) 0x4a430` | `?` | `?` | `[0x2811d8]:1` | `?` |
| `bl` at 0x45aac | `(unknown callee) 0xc2c9c` | `?` | `?` | `[0x2811d8]:1` | `?` |
| `bl` at 0x45ab8 | `(unknown callee) 0x46e0c` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x45ac4 | `(unknown callee) 0xc2d08` | `ret(0x46e0c)` | `?` | `?` | `ret(0x46e0c)` |
| `bl` at 0x45af8 | `(unknown callee) 0x49ef4` | `0xff` | `0xff` | `[0x2811d8]:1` | `?` |
| `bl` at 0x45b6c | `(unknown callee) 0xb7cbc` | `?` | `0x2` | `0x2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x459b4 | `[0x119e78] != 0x0` | `[0x119e78] == 0x0` |
| `bne` at 0x459d0 | `ret(0x3a8c0) != 0x0` | `ret(0x3a8c0) == 0x0` |
| `beq` at 0x459f4 | `ret(0x4a384) == 0x0` | `ret(0x4a384) != 0x0` |
| `blt` at 0x45a1c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x45a48 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x45a7c | `ret(0x4a430) == 0x0` | `ret(0x4a430) != 0x0` |
| `blt` at 0x45aa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x45ad0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x45b0c | `ret(0x49ef4) == 0x0` | `ret(0x49ef4) != 0x0` |
| `blt` at 0x45b40 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x45b80 | `ret(0xb7cbc) == 0x0` | `ret(0xb7cbc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45bac | `r0` | `?` |

## `TIOPciResetInt`

- `TIOPciResetInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45f30 to 0x46030.
  Established. Entry r0, r1 and r2 are truncated into bytes at `sp -0x11`, `sp -0x12`, `sp -0x13`
  (`strb` at 0x45f44, `strb` at 0x45f4c, `strb` at 0x45f54). The search loop’s backward branch takes
  its edge for `Z == 0`, with compared values unknown (`bne` at 0x45fd4). Two later branches take
  their bypasses for `? == 0x0` and fall through for `? != 0x0` (`beq` at 0x45fe4, `beq` at
  0x46004). Each fall-through path includes a call to `0x47a90` with all candidates unknown (`bl` at
  0x45ff4, `bl` at 0x46014). The interrupt mask and acknowledgement semantics remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x45f44 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x45f4c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x45f54 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `str` at 0x45f5c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x45f60 | load | 4 | `abs 0x4601c` | `-` |
| `ldrb` at 0x45f64 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45f74 | load | 4 | `abs 0x46020` | `-` |
| `str` at 0x45f78 | store | 4 | `sp -0x24` | `0xe2f58` |
| `ldrb` at 0x45f7c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45f80 | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldr` at 0x45f88 | load | 4 | `abs 0x46024` | `-` |
| `ldr` at 0x45f8c | load | 4 | `abs 0x46028` | `-` |
| `ldr` at 0x45f9c | load | 4 | `abs 0x4602c` | `-` |
| `ldr` at 0x45fa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45fb0 | load | 4 | `abs 0x119e78` | `-` |
| `str` at 0x45fb8 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x45fc0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x45fc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x45fc8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x45fcc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x45fd8 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x45fe8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x45fec | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x45ff8 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x46008 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4600c | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x46018 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46018 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45f94 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2e3c` | `0x265` | `0x0` |
| `bl` at 0x45ff4 | `(unknown callee) 0x47a90` | `?` | `?` | `?` | `?` |
| `bl` at 0x46014 | `(unknown callee) 0x47a90` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x45f70 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x45fd4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x45fe4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x46004 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46018 | `r0` | `?` |

## `tio_test_pci_reset_int`

- `tio_test_pci_reset_int`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37428 to 0x37484.
  Established. The first branch takes its bypass for `(r0 & 0xff) != 0xc` and falls through for
  equality (`bne` at 0x37458). The next branch takes the bypass for `? == 0x0` and falls through for
  `? != 0x0` (`beq` at 0x37468). The fall-through call targets `0x45f30` with candidates `(r0 &
  0xff)`, `0x2`, `(r2 & 0xff)`, `(r0 & 0xff)` (`bl` at 0x3747c). The call’s physical PCI reset
  effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3743c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x37444 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x3744c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x37450 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x3745c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x3746c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x37470 | load | 1 | `sp -0x13` | `-` |
| `ldmdb` at 0x37480 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x37480 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x37480 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3747c | `(unknown callee) 0x45f30` | `(r0 & 0xff)` | `0x2` | `(r2 & 0xff)` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x37458 | `(r0 & 0xff) != 0xc` | `(r0 & 0xff) == 0xc` |
| `beq` at 0x37468 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x37480 | `r0` | `?` |

## `l1ioInit`

- `l1ioInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4fca0 to 0x504a0.
  Established. The opening branch takes its edge for `[sp-24] != 0x0` and falls through for zero
  (`bne` at 0x4fd04). The zero-entry path includes a call to `0x18b54` with candidates `0xe40a0`,
  `0x802`, unknown, `ret(0x1e0f8)` (`bl` at 0x4fd20); its signed-nonnegative result takes the edge
  to a call to `0x18800`, while a negative result falls through to further testing (`bge` at
  0x4fd34, `bl` at 0x4fd64). The later computed transfer has taken condition `? <= 0x29` unsigned
  and falls through for a greater value, with computed destinations unresolved (`ldrls` at 0x4fd88).
  The reported return of `0x0` covers only known edges (`ldmdb` at 0x50424). The memory accesses
  decoded beyond that unresolved edge are not established as selected paths.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4fcb0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4fcb8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x4fcc0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x4fcc8 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x4fcd0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x4fcd8 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x4fce0 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x4fce8 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x4fcf0 | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x4fcf8 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x4fcfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4fd18 | load | 4 | `abs 0x50428` | `-` |
| `ldr` at 0x4fd1c | load | 4 | `abs 0x5042c` | `-` |
| `str` at 0x4fd28 | store | 4 | `sp -0x6c` | `ret(0x18b54)` |
| `ldr` at 0x4fd2c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x4fd40 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x4fd4c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x4fd54 | load | 4 | `abs 0x50430` | `-` |
| `ldr` at 0x4fd60 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x4fd6c | store | 4 | `sp -0x70` | `0x0` |
| `ldrls` at 0x4fd88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4fe8c | load | 4 | `abs 0x50450` | `-` |
| `str` at 0x4fe90 | store | 4 | `sp -0x80` | `0xe41dc` |
| `ldr` at 0x4fe94 | load | 4 | `abs 0x50454` | `-` |
| `str` at 0x4fe98 | store | 4 | `sp -0x7c` | `0xe4214` |
| `ldr` at 0x4fea0 | load | 4 | `abs 0x50458` | `-` |
| `str` at 0x4feb4 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x50420 | load | 4 | `sp -0x70` | `-` |
| `ldmdb` at 0x50424 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x50424 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x50424 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x50424 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4fd08 | `(unknown callee) 0x1e0f8` | `r0` | `r1` | `r2` | `[sp-24]` |
| `bl` at 0x4fd20 | `(unknown callee) 0x18b54` | `0xe40a0` | `0x802` | `?` | `ret(0x1e0f8)` |
| `bl` at 0x4fd38 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0x4fd50 | `(unknown callee) 0x18800` | `[sp-108]` | `?` | `?` | `[ret(0x188b4)]` |
| `bl` at 0x4fd58 | `(unknown callee) 0x1dba4` | `0xe40b0` | `?` | `?` | `?` |
| `bl` at 0x4fd64 | `(unknown callee) 0x18800` | `?` | `?` | `?` | `?` |
| `bl` at 0x4fd74 | `(unknown callee) 0x2b31c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4feac | `(unknown callee) 0x1eed0` | `0x4` | `0xe4220` | `0x86` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4fd04 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x4fd14 | `ret(0x1e0f8) == 0x0` | `ret(0x1e0f8) != 0x0` |
| `bge` at 0x4fd34 | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `bne` at 0x4fd48 | `[ret(0x188b4)] != 0x13` | `[ret(0x188b4)] == 0x13` |
| `ldrls` at 0x4fd88 | `? <= 0x29 (unsigned, 32-bit)` | `? > 0x29 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x50424 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4fd88).
