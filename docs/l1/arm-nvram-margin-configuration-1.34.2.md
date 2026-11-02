# ARM NVRAM, margin and configuration entry paths in 1.34.2

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
Computed transfers leave unresolved edges in several routines, identified below. Undecodable byte ranges are retained as warnings in the fact sheets. Tables cover known control-flow edges only.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/nvram-margin-configuration-facts.txt) retain all tool sections.

## `nvramInit`

- `nvramInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x194a8 to 0x1a360.
  Established. A word at `0x1169b0` receives `[0x1169b0]+1` (`str` at 0x194fc), and another at
  `0x1169d4` receives `ret(0x18b54)` (`str` at 0x19530). The call to `0x18cc8` has candidates
  `[0x1169d4]`, `0x27eed0`, `0x2000` and `0x1169d4` (`bl` at 0x1959c); its nonnegative signed result
  takes the branch, while a negative result falls through (`bge` at 0x195a8). The computed transfer
  has an unsigned comparison of an unknown value with `0x9` (`ldrls` at 0x196cc), and its taken
  edges remain unresolved. Device initialization and persistence semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x194bc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x194c0 | load | 4 | `abs 0x1a2f0` | `-` |
| `ldr` at 0x194c4 | load | 4 | `abs 0x1169b0` | `-` |
| `ldr` at 0x194d0 | load | 4 | `abs 0x1a2f4` | `-` |
| `str` at 0x194d4 | store | 4 | `sp -0x68` | `0xd6cdc` |
| `ldr` at 0x194dc | load | 4 | `abs 0x1a2f8` | `-` |
| `ldr` at 0x194ec | load | 4 | `abs 0x1a2f0` | `-` |
| `ldr` at 0x194f0 | load | 4 | `abs 0x1a2f0` | `-` |
| `ldr` at 0x194f4 | load | 4 | `abs 0x1169b0` | `-` |
| `str` at 0x194fc | store | 4 | `abs 0x1169b0` | `[0x1169b0]+1` |
| `ldr` at 0x19500 | load | 4 | `abs 0x1a2fc` | `-` |
| `ldr` at 0x19510 | load | 4 | `abs 0x1a300` | `-` |
| `str` at 0x19514 | store | 4 | `sp -0x24` | `0xd6cfc` |
| `ldr` at 0x19518 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1951c | load | 4 | `abs 0x1a304` | `-` |
| `ldr` at 0x19520 | load | 4 | `abs 0x1a308` | `-` |
| `ldr` at 0x1952c | load | 4 | `abs 0x1a30c` | `-` |
| `str` at 0x19530 | store | 4 | `abs 0x1169d4` | `ret(0x18b54)` |
| `ldr` at 0x19534 | load | 4 | `abs 0x1169d4` | `-` |
| `ldr` at 0x19548 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x19554 | load | 4 | `abs 0x1a310` | `-` |
| `str` at 0x19558 | store | 4 | `sp -0x68` | `0xd6d0c` |
| `ldr` at 0x1955c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x19560 | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x19564 | store | 4 | `sp -0x60` | `ret(0x1e71c)` |
| `ldr` at 0x1956c | load | 4 | `abs 0x1a2f8` | `-` |
| `ldr` at 0x1957c | load | 4 | `abs 0x1a314` | `-` |
| `ldr` at 0x1958c | load | 4 | `abs 0x1a30c` | `-` |
| `ldr` at 0x19590 | load | 4 | `abs 0x1169d4` | `-` |
| `ldr` at 0x19594 | load | 4 | `abs 0x1a314` | `-` |
| `ldr` at 0x195b4 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x195c0 | load | 4 | `abs 0x1a318` | `-` |
| `str` at 0x195c4 | store | 4 | `sp -0x68` | `0xd6d30` |
| `ldr` at 0x195c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x195cc | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x195d0 | store | 4 | `sp -0x60` | `ret(0x1e71c)` |
| `ldr` at 0x195d8 | load | 4 | `abs 0x1a2f8` | `-` |
| `ldr` at 0x195e8 | load | 4 | `abs 0x1a31c` | `-` |
| `ldr` at 0x195ec | load | 4 | `abs 0x1a320` | `-` |
| `ldr` at 0x195fc | load | 4 | `abs 0x1a324` | `-` |
| `ldr` at 0x19600 | load | 4 | `abs 0x1a314` | `-` |
| `str` at 0x19604 | store | 4 | `abs 0x116944` | `0x27eed0` |
| `ldr` at 0x19608 | load | 4 | `abs 0x1a324` | `-` |
| `ldr` at 0x1960c | load | 4 | `abs 0x116944` | `-` |
| `ldr` at 0x1962c | load | 4 | `abs 0x1a324` | `-` |
| `ldr` at 0x19630 | load | 4 | `abs 0x116944` | `-` |
| `ldrb` at 0x19634 | load | 1 | `[0x116944] +0x0` | `-` |
| `ldr` at 0x19648 | load | 4 | `abs 0x1a324` | `-` |
| `ldr` at 0x1964c | load | 4 | `abs 0x116944` | `-` |
| `ldrb` at 0x19650 | load | 1 | `[0x116944] +0x1` | `-` |
| `ldr` at 0x19660 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19664 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19670 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19674 | load | 4 | `abs 0x1a324` | `-` |
| `ldr` at 0x19678 | load | 4 | `abs 0x116944` | `-` |
| `ldrb` at 0x1967c | load | 1 | `[0x116944] +0x1` | `-` |
| `ldr` at 0x19684 | load | 4 | `abs 0x1a314` | `-` |
| `str` at 0x1968c | store | 4 | `abs 0x11694c` | `?` |
| `ldr` at 0x19690 | load | 4 | `abs 0x1a32c` | `-` |
| `ldr` at 0x19694 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19698 | load | 4 | `abs 0x11694c` | `-` |
| `str` at 0x196a0 | store | 4 | `abs 0x116950` | `[0x11694c]+2352` |
| `ldr` at 0x196a4 | load | 4 | `abs 0x1a330` | `-` |
| `ldr` at 0x196a8 | load | 4 | `abs 0x1a334` | `-` |
| `str` at 0x196ac | store | 4 | `abs 0x116954` | `0x280ec8` |
| `ldr` at 0x196b0 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x196b4 | load | 4 | `abs 0x11694c` | `-` |
| `ldrb` at 0x196b8 | load | 1 | `[0x11694c] +0x0` | `-` |
| `ldrls` at 0x196cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x19d6c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19d70 | load | 4 | `abs 0x11694c` | `-` |
| `str` at 0x19d90 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x19d94 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19d98 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19d9c | load | 4 | `abs 0x1a344` | `-` |
| `ldrb` at 0x19da0 | load | 1 | `[0x11694c] +0x7c8` | `-` |
| `ldr` at 0x19db0 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19db4 | load | 4 | `abs 0x11694c` | `-` |
| `str` at 0x19de0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x19de8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19dec | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19df0 | load | 4 | `abs 0x1a348` | `-` |
| `strb` at 0x19df8 | store | 1 | `[0x11694c] +0x7cc` | `0x0` |
| `ldr` at 0x19dfc | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e00 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19e04 | load | 4 | `[0x11694c] +0x7cc` | `-` |
| `str` at 0x19e08 | store | 4 | `sp -0x5c` | `[[0x11694c]+1996]` |
| `ldr` at 0x19e0c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x19e14 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x19e18 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x19e1c | store | 4 | `[0x11694c] +0x7cc` | `[sp-92]` |
| `ldr` at 0x19e20 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e24 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19e3c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e40 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19e58 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e5c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19e60 | load | 4 | `abs 0x1a344` | `-` |
| `strb` at 0x19e68 | store | 1 | `[0x11694c] +0x7c8` | `0x3` |
| `ldr` at 0x19e6c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e70 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19e74 | load | 4 | `abs 0x1a33c` | `-` |
| `ldr` at 0x19e78 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e7c | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x19e88 | store | 1 | `[0x11694c] +0x3bf` | `(ret(0x1eddc) & 0xff)` |
| `ldr` at 0x19e8c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e90 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19e94 | load | 4 | `abs 0x1a34c` | `-` |
| `ldr` at 0x19e98 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19e9c | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x19eb4 | store | 1 | `[0x11694c] +0x87b` | `(ret(0x1eddc) & 0xff)` |
| `ldr` at 0x19eb8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19ebc | load | 4 | `abs 0x11694c` | `-` |
| `ldrb` at 0x19ec4 | load | 1 | `[0x11694c] +0x780` | `-` |
| `ldr` at 0x19ed4 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19ed8 | load | 4 | `abs 0x11694c` | `-` |
| `str` at 0x19f00 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x19f08 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f0c | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x19f18 | store | 1 | `[0x11694c] +0x780` | `0x1` |
| `ldr` at 0x19f1c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f20 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19f24 | load | 4 | `abs 0x1a350` | `-` |
| `ldr` at 0x19f28 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f2c | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x19f40 | store | 1 | `[0x11694c] +0x7a3` | `(ret(0x1eddc) & 0xff)` |
| `ldr` at 0x19f44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x19f58 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f5c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19f64 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f6c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19f78 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f7c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19f88 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19f8c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19fa8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19fac | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x19fb8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x19fbc | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a014 | load | 4 | `abs 0x1a330` | `-` |
| `ldr` at 0x1a018 | load | 4 | `abs 0x1a32c` | `-` |
| `ldr` at 0x1a01c | load | 4 | `abs 0x116954` | `-` |
| `ldr` at 0x1a020 | load | 4 | `abs 0x116950` | `-` |
| `ldr` at 0x1a028 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a02c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a030 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a034 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a038 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1a03c | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1a04c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a050 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a054 | load | 4 | `abs 0x1a32c` | `-` |
| `ldr` at 0x1a058 | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1a05c | load | 4 | `abs 0x116950` | `-` |
| `ldr` at 0x1a068 | load | 4 | `abs 0x1a32c` | `-` |
| `ldr` at 0x1a06c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a070 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a074 | load | 4 | `abs 0x116950` | `-` |
| `ldr` at 0x1a078 | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1a080 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x1a084 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a088 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a08c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a090 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a094 | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1a098 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1a0a0 | store | 4 | `[0x11694c] +0x784` | `([[0x11694c]+1924] + [sp-36])` |
| `ldr` at 0x1a0a4 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a0a8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a0ac | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a0b0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a0b4 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1a0b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1a0c0 | store | 4 | `[0x11694c] +0x788` | `([[0x11694c]+1928] + [sp-36])` |
| `ldr` at 0x1a0c4 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a0c8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a0cc | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a0d0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a0d4 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1a0d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1a0e0 | store | 4 | `[0x11694c] +0x78c` | `([[0x11694c]+1932] + [sp-36])` |
| `ldr` at 0x1a0e4 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a0e8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a0ec | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a0f0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a0f4 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1a0f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1a100 | store | 4 | `[0x11694c] +0x790` | `([[0x11694c]+1936] + [sp-36])` |
| `ldr` at 0x1a104 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a108 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a10c | load | 4 | `abs 0x1a350` | `-` |
| `ldr` at 0x1a110 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a114 | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x1a128 | store | 1 | `[0x11694c] +0x7a3` | `(ret(0x1eddc) & 0xff)` |
| `ldr` at 0x1a12c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a130 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a13c | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a140 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a158 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a15c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a160 | load | 4 | `abs 0x1a32c` | `-` |
| `ldr` at 0x1a164 | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1a168 | load | 4 | `abs 0x116950` | `-` |
| `ldr` at 0x1a174 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a178 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a17c | load | 4 | `abs 0x1a330` | `-` |
| `ldr` at 0x1a180 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1a184 | load | 4 | `abs 0x116954` | `-` |
| `ldr` at 0x1a190 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a194 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a198 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a19c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a1a0 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1a1a4 | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1a1b0 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a1b4 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a1b8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a1bc | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a1c0 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1a1c4 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1a1d0 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a1d4 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a1d8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a1dc | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a1e0 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1a1e4 | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1a1f0 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a1f4 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a1f8 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a1fc | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a200 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1a204 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1a214 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a218 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a21c | load | 4 | `abs 0x1a32c` | `-` |
| `ldr` at 0x1a220 | load | 4 | `abs 0x116950` | `-` |
| `str` at 0x1a224 | store | 4 | `[0x11694c] +0x784` | `[0x116950]` |
| `ldr` at 0x1a228 | load | 4 | `abs 0x1a328` | `-` |
| `ldr` at 0x1a22c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1a230 | load | 4 | `abs 0x1a330` | `-` |
| `ldr` at 0x1a234 | load | 4 | `abs 0x116954` | `-` |
| `str` at 0x1a238 | store | 4 | `[0x11694c] +0x788` | `[0x116954]` |
| `ldr` at 0x1a240 | load | 4 | `abs 0x1a2fc` | `-` |
| `ldr` at 0x1a244 | load | 4 | `abs 0x1a354` | `-` |
| `ldrb` at 0x1a248 | load | 1 | `abs 0x116958` | `-` |
| `strb` at 0x1a250 | store | 1 | `abs 0x116958` | `([0x116958]:1+1 & 0xff)` |
| `ldr` at 0x1a258 | load | 4 | `abs 0x1a358` | `-` |
| `str` at 0x1a25c | store | 4 | `(unknown)` | `0xd6d68` |
| `ldr` at 0x1a260 | load | 4 | `abs 0x1a354` | `-` |
| `ldrb` at 0x1a264 | load | 1 | `abs 0x116958` | `-` |
| `ldr` at 0x1a270 | load | 4 | `abs 0x1a354` | `-` |
| `strb` at 0x1a278 | store | 1 | `abs 0x116958` | `0x0` |
| `ldr` at 0x1a27c | load | 4 | `abs 0x1a2fc` | `-` |
| `ldr` at 0x1a280 | load | 4 | `abs 0x1a354` | `-` |
| `ldrb` at 0x1a284 | load | 1 | `abs 0x116958` | `-` |
| `ldr` at 0x1a28c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1a29c | load | 4 | `abs 0x1a2fc` | `-` |
| `ldr` at 0x1a2a0 | load | 4 | `abs 0x1a354` | `-` |
| `ldrb` at 0x1a2a4 | load | 1 | `abs 0x116958` | `-` |
| `ldr` at 0x1a2ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1a2b4 | load | 4 | `abs 0x1a354` | `-` |
| `ldr` at 0x1a2b8 | load | 4 | `abs 0x1a354` | `-` |
| `ldrb` at 0x1a2bc | load | 1 | `abs 0x116958` | `-` |
| `strb` at 0x1a2c4 | store | 1 | `abs 0x116958` | `([0x116958]:1+1 & 0xff)` |
| `ldr` at 0x1a2dc | load | 4 | `abs 0x1a35c` | `-` |
| `ldmdb` at 0x1a2ec | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1a2ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1a2ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1a2ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1a2ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x194e8 | `(unknown callee) 0x1eed0` | `0x9` | `0xd6c64` | `0x71` | `0x0` |
| `bl` at 0x1950c | `(unknown callee) 0x18c20` | `0x1169b4` | `0x0` | `0x20` | `[0x1169b0]+1` |
| `bl` at 0x19524 | `(unknown callee) 0x18b54` | `0xd6cfc` | `0x1042` | `0x1b6` | `0xd6cfc` |
| `bl` at 0x19540 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `ret(0x18b54)` | `[0x1169d4]` |
| `bl` at 0x1954c | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0x19578 | `(unknown callee) 0x1eed0` | `0x9` | `0xd6c64` | `0x82` | `0x0` |
| `bl` at 0x19588 | `(unknown callee) 0x18c20` | `0x27eed0` | `0x0` | `0x2000` | `?` |
| `bl` at 0x1959c | `(unknown callee) 0x18cc8` | `[0x1169d4]` | `0x27eed0` | `0x2000` | `0x1169d4` |
| `bl` at 0x195ac | `(unknown callee) 0x188b4` | `ret(0x18cc8)` | `?` | `?` | `ret(0x18cc8)` |
| `bl` at 0x195b8 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0x195e4 | `(unknown callee) 0x1eed0` | `0x9` | `0xd6c64` | `0x88` | `0x0` |
| `bl` at 0x195f8 | `(unknown callee) 0x1f9e8` | `0x11695c` | `0xd6d5c` | `0x1` | `0x9` |
| `bl` at 0x19614 | `(unknown callee) 0x1ed68` | `[0x116944]` | `0x8` | `0x116944` | `0x116944` |
| `bl` at 0x19628 | `(unknown callee) 0x1aacc` | `ret(0x1ed68)` | `?` | `?` | `?` |
| `bl` at 0x1965c | `(unknown callee) 0x1aacc` | `?` | `?` | `?` | `?` |
| `bl` at 0x19d78 | `(unknown callee) 0x1ed68` | `[0x11694c]` | `0x3c0` | `?` | `0x11694c` |
| `bl` at 0x19dc8 | `(unknown callee) 0x1ed68` | `[0x11694c]+1992` | `0xb4` | `[0x11694c]` | `[0x11694c]+1992` |
| `bl` at 0x19e38 | `(unknown callee) 0x18c20` | `[0x11694c]+2000` | `0x0` | `0xa8` | `[0x11694c]+2000` |
| `bl` at 0x19e54 | `(unknown callee) 0x18c20` | `[0x11694c]+952` | `0x0` | `0x7` | `[0x11694c]+952` |
| `bl` at 0x19e84 | `(unknown callee) 0x1eddc` | `[0x11694c]` | `0x3c0` | `0x7c8` | `0x11694c` |
| `bl` at 0x19eb0 | `(unknown callee) 0x1eddc` | `[0x11694c]+1992` | `0xb4` | `?` | `[0x11694c]+1992` |
| `bl` at 0x19ee8 | `(unknown callee) 0x1ed68` | `[0x11694c]+1920` | `0x24` | `[0x11694c]` | `[0x11694c]+1920` |
| `bl` at 0x19f3c | `(unknown callee) 0x1eddc` | `[0x11694c]+1920` | `0x24` | `0x780` | `[0x11694c]+1920` |
| `bl` at 0x19f50 | `(unknown callee) 0x1a360` | `?` | `?` | `?` | `[sp-32]` |
| `bl` at 0x19f74 | `(unknown callee) 0x18ae8` | `[0x11694c]+960` | `[0x11694c]` | `0x3c0` | `[0x11694c]+960` |
| `bl` at 0x19fa4 | `(unknown callee) 0x18ae8` | `[0x11694c]+2172` | `[0x11694c]+1992` | `0xb4` | `[0x11694c]+1992` |
| `bl` at 0x19fd0 | `(unknown callee) 0x18ae8` | `[0x11694c]+1956` | `[0x11694c]+1920` | `0x24` | `[0x11694c]+1920` |
| `bl` at 0x19fd8 | `(unknown callee) 0x1c79c` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x19ff0 | `(unknown callee) 0x1abd0` | `0x11` | `0x2` | `?` | `ret(0x1c79c)` |
| `bl` at 0x19ff8 | `(unknown callee) 0x1c79c` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x1a010 | `(unknown callee) 0x1abd0` | `0x3d` | `0x2` | `?` | `ret(0x1c79c)` |
| `bl` at 0x1a124 | `(unknown callee) 0x1eddc` | `[0x11694c]+1920` | `0x24` | `[[0x11694c]+1936]` | `[0x11694c]+1920` |
| `bl` at 0x1a154 | `(unknown callee) 0x18ae8` | `[0x11694c]+1956` | `[0x11694c]+1920` | `0x24` | `[0x11694c]+1920` |
| `bl` at 0x1a23c | `(unknown callee) 0x1d638` | `?` | `?` | `[0x11694c]` | `[0x116954]` |
| `bl` at 0x1a2b0 | `(unknown callee) 0x1d6d8` | `?` | `?` | `0x1169b4` | `?` |
| `bl` at 0x1a2cc | `(unknown callee) 0x19390` | `?` | `?` | `?` | `?` |
| `bl` at 0x1a2d4 | `(unknown callee) 0x1c79c` | `0x46` | `?` | `?` | `?` |
| `bl` at 0x1a2e8 | `(unknown callee) 0x18884` | `0xd6d8c` | `ret(0x1c79c)` | `0x1` | `ret(0x1c79c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x194cc | `[0x1169b0] == 0x0` | `[0x1169b0] != 0x0` |
| `bge` at 0x1953c | `[0x1169d4] >= 0x0 (signed, 32-bit)` | `[0x1169d4] < 0x0 (signed, 32-bit)` |
| `bge` at 0x195a8 | `ret(0x18cc8) >= 0x0 (signed, 32-bit)` | `ret(0x18cc8) < 0x0 (signed, 32-bit)` |
| `beq` at 0x19624 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x19644 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x19658 | `[[0x116944]+1]:1 != 0x0` | `[[0x116944]+1]:1 == 0x0` |
| `bne` at 0x1966c | `[0x11694c] != 0x0` | `[0x11694c] == 0x0` |
| `ldrls` at 0x196cc | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |
| `beq` at 0x19d88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x19da8 | `[[0x11694c]+1992]:1 == 0x3` | `[[0x11694c]+1992]:1 != 0x3` |
| `beq` at 0x19dd8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x19ecc | `[[0x11694c]+1920]:1 == 0x1` | `[[0x11694c]+1920]:1 != 0x1` |
| `beq` at 0x19ef8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x19f4c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bhi` at 0x19fe4 | `ret(0x1c79c) > 0x1 (unsigned, 32-bit)` | `ret(0x1c79c) <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x1a004 | `ret(0x1c79c) != 0x0` | `ret(0x1c79c) == 0x0` |
| `bne` at 0x1a048 | `? != ?` | `? == ?` |
| `beq` at 0x1a064 | `[[0x11694c]+1924] == [0x116950]` | `[[0x11694c]+1924] != [0x116950]` |
| `bne` at 0x1a170 | `[[0x11694c]+1924] != [0x116950]` | `[[0x11694c]+1924] == [0x116950]` |
| `bne` at 0x1a18c | `[[0x11694c]+1928] != [0x116954]` | `[[0x11694c]+1928] == [0x116954]` |
| `blo` at 0x1a1ac | `[[0x11694c]+1932] < [[0x11694c]+1924] (unsigned, 32-bit)` | `[[0x11694c]+1932] >= [[0x11694c]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1a1cc | `[[0x11694c]+1932] >= [[0x11694c]+1928] (unsigned, 32-bit)` | `[[0x11694c]+1932] < [[0x11694c]+1928] (unsigned, 32-bit)` |
| `blo` at 0x1a1ec | `[[0x11694c]+1936] < [[0x11694c]+1924] (unsigned, 32-bit)` | `[[0x11694c]+1936] >= [[0x11694c]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1a20c | `[[0x11694c]+1936] >= [[0x11694c]+1928] (unsigned, 32-bit)` | `[[0x11694c]+1936] < [[0x11694c]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1a26c | `[0x116958]:1 == 0x0` | `[0x116958]:1 != 0x0` |
| `bne` at 0x1a294 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1a2ec | `r0` | `ret(0x18884)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x196cc).

## `nvramGet`

- `nvramGet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1c79c to 0x1d638.
  Established. The computed transfer is taken when unsigned `r0-1 <= 0x9f`, and falls through when
  `r0-1 > 0x9f` (`ldrls` at 0x1c7bc). The known fall-through path stores a zero word at `sp -0x18`
  (`str` at 0x1d624) and reports zero in `r0` at the exit (`ldmdb` at 0x1d62c). No call is recorded
  on known edges. The taken targets and setting meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1c7ac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1c7b0 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x1c7bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x1d624 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x1d628 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1d62c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1d62c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1d62c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1c7bc | `r0-1 <= 0x9f (unsigned, 32-bit)` | `r0-1 > 0x9f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1d62c | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1c7bc).

## `nvramSet`

- `nvramSet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1abd0 to 0x1c79c.
  Established. The two entry words are saved at `sp -0x1c` (`str` at 0x1abe0) and `sp -0x20` (`str`
  at 0x1abe4). The computed transfer splits on unsigned `r0-1 <= 0x9f`, following that arm and falling
  through when greater (`ldrls` at 0x1abf4); its fall-through reaches the recorded return with entry
  `r0` preserved (`ldmdb` at 0x1c798). The taken targets remain unresolved, so the known-edge table
  establishes no setting write or callee. Effects of the setting-specific bodies are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1abe0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1abe4 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x1abe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x1abf4 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x1c798 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1c798 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1c798 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1c798 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1c798 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1abf4 | `r0-1 <= 0x9f (unsigned, 32-bit)` | `r0-1 > 0x9f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1c798 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1abf4).

## `power_margin_multi_init`

- `power_margin_multi_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b9cc to 0x3c2c8.
  Established. The first explicit word clear targets `[sp] +0x0` (`str` at 0x3b9f4). Later
  word stores write `0x4` at `[sp-24] +0x10` (`str` at 0x3ba2c) and `0x4` at `[sp-24] +0xc`
  (`str` at 0x3ba34). Byte stores write `0x2` at `[sp-24] +0x24` (`strb` at 0x3ba70) and `0x6`
  at `[sp-24] +0x25` (`strb` at 0x3ba7c). The computed transfer is taken when unsigned
  `[[[sp-24]+8]+8]-1 <= 0x4`, with greater values falling through (`ldrls` at 0x3bb50); its taken
  edges and the electrical meanings of these stored values remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b9dc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3b9e0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3b9e4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x3b9e8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x3b9ec | load | 4 | `sp +0x0` | `-` |
| `str` at 0x3b9f4 | store | 4 | `[sp] +0x0` | `0x0` |
| `ldr` at 0x3b9f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ba08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ba0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ba10 | store | 4 | `[sp-24] +0x0` | `[sp-28]` |
| `ldr` at 0x3ba14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ba18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ba1c | store | 4 | `[sp-24] +0x4` | `[sp-32]` |
| `ldr` at 0x3ba20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ba24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ba2c | store | 4 | `[sp-24] +0x10` | `0x4` |
| `str` at 0x3ba34 | store | 4 | `[sp-24] +0xc` | `0x4` |
| `ldr` at 0x3ba38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ba3c | load | 4 | `abs 0x3c2a8` | `-` |
| `str` at 0x3ba40 | store | 4 | `[sp-24] +0x14` | `0x0` |
| `ldr` at 0x3ba44 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ba4c | store | 1 | `[sp-24] +0x1d` | `0x0` |
| `ldr` at 0x3ba50 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ba58 | store | 1 | `[sp-24] +0x1f` | `0x0` |
| `ldr` at 0x3ba5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ba64 | store | 4 | `[sp-24] +0x28` | `0x1` |
| `ldr` at 0x3ba68 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ba70 | store | 1 | `[sp-24] +0x24` | `0x2` |
| `ldr` at 0x3ba74 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ba7c | store | 1 | `[sp-24] +0x25` | `0x6` |
| `ldr` at 0x3ba80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ba84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ba8c | store | 4 | `[sp-24] +0x8` | `ret(0x3dae8)` |
| `ldr` at 0x3ba90 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x3baa4 | store | 4 | `sp -0x3c` | `ret(0x3db60)` |
| `ldr` at 0x3baac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3bab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3babc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3bac0 | load | 4 | `[[sp-24]+8] +0x8` | `-` |
| `ldr` at 0x3bacc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bad0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3bad4 | load | 4 | `[[sp-24]+8] +0x8` | `-` |
| `ldr` at 0x3bae0 | load | 4 | `abs 0x3c2ac` | `-` |
| `str` at 0x3bae4 | store | 4 | `sp -0x3c` | `0xfffff9b8` |
| `ldr` at 0x3baec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3baf0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x3baf4 | load | 1 | `[[sp-24]+8] +0x10` | `-` |
| `str` at 0x3bb08 | store | 4 | `sp -0x30` | `ret(0x1c79c)` |
| `ldr` at 0x3bb0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bb10 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x3bb14 | load | 1 | `[[sp-24]+8] +0x10` | `-` |
| `ldr` at 0x3bb28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bb2c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x3bb30 | load | 1 | `[[sp-24]+8] +0x6` | `-` |
| `ldr` at 0x3bb3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bb40 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3bb44 | load | 4 | `[[sp-24]+8] +0x8` | `-` |
| `ldrls` at 0x3bb50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3bee8 | load | 4 | `abs 0x3c2b4` | `-` |
| `str` at 0x3beec | store | 4 | `sp -0x48` | `0xe17b4` |
| `ldr` at 0x3bef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bef4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3bef8 | load | 4 | `[[sp-24]+8] +0x8` | `-` |
| `str` at 0x3befc | store | 4 | `sp -0x44` | `[[[sp-24]+8]+8]` |
| `ldr` at 0x3bf04 | load | 4 | `abs 0x3c2b8` | `-` |
| `ldr` at 0x3bf08 | load | 4 | `abs 0x3c2bc` | `-` |
| `ldr` at 0x3bf18 | load | 4 | `abs 0x3c2c0` | `-` |
| `str` at 0x3bf1c | store | 4 | `sp -0x3c` | `0xfffff9be` |
| `ldr` at 0x3bf24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3bf30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bf34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bf38 | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0x3bf40 | store | 1 | `[sp-24] +0x1e` | `(ret(0x3d8ac) & 0xff)` |
| `ldr` at 0x3bf44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bf48 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x3bf4c | load | 1 | `[[sp-24]+8] +0x10` | `-` |
| `ldr` at 0x3bf60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bf64 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x3bf68 | load | 1 | `[[sp-24]+8] +0x10` | `-` |
| `ldr` at 0x3bf70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3bf74 | load | 1 | `[sp-24] +0x1e` | `-` |
| `ldr` at 0x3bf84 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x3bf8c | store | 4 | `[sp] +0x0` | `0x1` |
| `ldr` at 0x3bf90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bf94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3bf98 | load | 1 | `[sp-24] +0x1e` | `-` |
| `ldr` at 0x3bf9c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x3bfac | store | 4 | `sp -0x3c` | `ret(0x3dea4)` |
| `ldr` at 0x3bfb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3bfc0 | store | 4 | `sp -0x2c` | `ret(0x3aa60)` |
| `ldr` at 0x3bfc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3bfd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3bfd4 | load | 4 | `abs 0x3c2ac` | `-` |
| `ldr` at 0x3bfe0 | load | 4 | `abs 0x3c2c4` | `-` |
| `str` at 0x3bfe4 | store | 4 | `sp -0x3c` | `0xfffff9bd` |
| `ldr` at 0x3bfec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bff0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3bff8 | store | 4 | `[sp-24] +0x18` | `ret(0x3d538)` |
| `ldr` at 0x3bffc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c000 | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0x3c004 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3c010 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3c01c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3c02c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3c03c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c044 | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0x3c04c | store | 1 | `[sp-24] +0x1e` | `(ret(0x3d7d8) & 0xff)` |
| `ldr` at 0x3c050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c054 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c05c | store | 4 | `[sp-24] +0x18` | `ret(0x3d2e0)` |
| `ldr` at 0x3c064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c06c | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0x3c074 | store | 1 | `[sp-24] +0x1e` | `(ret(0x3d8ac) & 0xff)` |
| `ldr` at 0x3c07c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c084 | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0x3c08c | store | 1 | `[sp-24] +0x1e` | `(ret(0x3d980) & 0xff)` |
| `ldr` at 0x3c090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c094 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c09c | store | 4 | `[sp-24] +0x18` | `ret(0x3d40c)` |
| `ldr` at 0x3c0a4 | load | 4 | `abs 0x3c2b0` | `-` |
| `str` at 0x3c0a8 | store | 4 | `sp -0x3c` | `0xfffff9bc` |
| `ldr` at 0x3c0b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c0b4 | load | 1 | `[sp-24] +0x1e` | `-` |
| `ldr` at 0x3c0c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c0c4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3c0c8 | load | 4 | `[[sp-24]+8] +0x8` | `-` |
| `ldr` at 0x3c0d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c0d8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3c0dc | load | 4 | `[[sp-24]+8] +0x8` | `-` |
| `ldr` at 0x3c0ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c0f0 | load | 1 | `[sp-24] +0x1e` | `-` |
| `ldr` at 0x3c0f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c0f8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x3c110 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3c114 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c118 | load | 1 | `[sp-24] +0x1e` | `-` |
| `ldr` at 0x3c11c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c120 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x3c138 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3c13c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c140 | load | 4 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x3c14c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c150 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c154 | load | 1 | `[sp-24] +0x1d` | `-` |
| `ldr` at 0x3c158 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x3c164 | store | 1 | `[sp-24] +0x21` | `?` |
| `ldr` at 0x3c168 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c16c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c170 | load | 1 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x3c174 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3c180 | store | 1 | `[sp-24] +0x20` | `?` |
| `ldr` at 0x3c188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c18c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c190 | load | 1 | `[sp-24] +0x1d` | `-` |
| `ldr` at 0x3c194 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x3c1a0 | store | 1 | `[sp-24] +0x21` | `?` |
| `ldr` at 0x3c1a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c1a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c1ac | load | 1 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x3c1b0 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3c1bc | store | 1 | `[sp-24] +0x20` | `?` |
| `ldr` at 0x3c1c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c1c8 | load | 1 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x3c1cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c1d0 | load | 1 | `[sp-24] +0x1d` | `-` |
| `str` at 0x3c1d8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3c1dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c1e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c1e4 | load | 1 | `[sp-24] +0x1d` | `-` |
| `strb` at 0x3c1e8 | store | 1 | `[sp-24] +0x21` | `([[sp-24]+29]:1 & 0xff)` |
| `ldr` at 0x3c1ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c1f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c1f4 | load | 1 | `[sp-24] +0x1c` | `-` |
| `strb` at 0x3c1f8 | store | 1 | `[sp-24] +0x20` | `([[sp-24]+28]:1 & 0xff)` |
| `ldr` at 0x3c1fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c204 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c208 | load | 1 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x3c20c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c210 | load | 1 | `[sp-24] +0x1d` | `-` |
| `ldrb` at 0x3c228 | load | 1 | `[sp-24] +0x1d` | `-` |
| `strb` at 0x3c230 | store | 1 | `[sp-24] +0x1e` | `?` |
| `ldr` at 0x3c234 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3c23c | store | 1 | `[sp-24] +0x25` | `0x2` |
| `ldr` at 0x3c240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c244 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c248 | load | 1 | `[sp-24] +0x21` | `-` |
| `strb` at 0x3c24c | store | 1 | `[sp-24] +0x23` | `([[sp-24]+33]:1 & 0xff)` |
| `ldr` at 0x3c250 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c254 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c258 | load | 1 | `[sp-24] +0x20` | `-` |
| `strb` at 0x3c25c | store | 1 | `[sp-24] +0x22` | `([[sp-24]+32]:1 & 0xff)` |
| `ldr` at 0x3c260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c264 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c268 | load | 1 | `[sp-24] +0x1e` | `-` |
| `ldr` at 0x3c26c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x3c27c | store | 4 | `sp -0x2c` | `ret(0x3dea4)` |
| `ldr` at 0x3c280 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c28c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3c290 | store | 4 | `sp -0x3c` | `ret(0x3dea4)` |
| `str` at 0x3c29c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3c2a0 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x3c2a4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3c2a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c2a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c2a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ba04 | `(unknown callee) 0x18c20` | `[sp-24]` | `0x0` | `0x4` | `0x0` |
| `bl` at 0x3ba88 | `(unknown callee) 0x3dae8` | `[sp-28]` | `[sp-24]` | `[sp-24]` | `0x6` |
| `bl` at 0x3ba9c | `(unknown callee) 0x3db60` | `ret(0x3dae8)` | `?` | `?` | `[[sp-24]+8]` |
| `bl` at 0x3bb00 | `(unknown callee) 0x1c79c` | `[[[sp-24]+8]+16]:1+20` | `?` | `?` | `[[[sp-24]+8]+16]:1+20` |
| `bl` at 0x3bb24 | `(unknown callee) 0x1abd0` | `[[[sp-24]+8]+16]:1+20` | `0x5` | `?` | `[[[sp-24]+8]+16]:1+20` |
| `bl` at 0x3bf10 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x665` | `0x0` |
| `bl` at 0x3bf3c | `(unknown callee) 0x3d8ac` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x3bf5c | `(unknown callee) 0x1abd0` | `[[[sp-24]+8]+16]:1+20` | `0x0` | `?` | `[[[sp-24]+8]+16]:1+20` |
| `bl` at 0x3bf80 | `(unknown callee) 0x1abd0` | `[[[sp-24]+8]+16]:1+130` | `[[sp-24]+30]:1` | `[[[sp-24]+8]+16]:1+130` | `[[sp-24]+30]:1` |
| `bl` at 0x3bfa4 | `(unknown callee) 0x3dea4` | `[[sp-24]+8]` | `[[sp-24]+30]:1` | `[sp-24]` | `[[sp-24]+30]:1` |
| `bl` at 0x3bfb8 | `(unknown callee) 0x3aa60` | `[sp-28]` | `?` | `?` | `[sp-32]` |
| `bl` at 0x3bff4 | `(unknown callee) 0x3d538` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3c048 | `(unknown callee) 0x3d7d8` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x3c058 | `(unknown callee) 0x3d2e0` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3c070 | `(unknown callee) 0x3d8ac` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x3c088 | `(unknown callee) 0x3d980` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x3c098 | `(unknown callee) 0x3d40c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3c274 | `(unknown callee) 0x3dea4` | `[[sp-24]+8]` | `[[sp-24]+30]:1` | `[sp-24]` | `[[sp-24]+30]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3ba98 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `bne` at 0x3bab4 | `[sp-32] != 0x6` | `[sp-32] == 0x6` |
| `beq` at 0x3bac8 | `[[[sp-24]+8]+8] == 0x1` | `[[[sp-24]+8]+8] != 0x1` |
| `beq` at 0x3badc | `[[[sp-24]+8]+8] == 0x2` | `[[[sp-24]+8]+8] != 0x2` |
| `beq` at 0x3bb38 | `[[[sp-24]+8]+6]:1 == 0xff` | `[[[sp-24]+8]+6]:1 != 0xff` |
| `ldrls` at 0x3bb50 | `[[[sp-24]+8]+8]-1 <= 0x4 (unsigned, 32-bit)` | `[[[sp-24]+8]+8]-1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0x3bf2c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3bfcc | `ret(0x3aa60) == 0x1` | `ret(0x3aa60) != 0x1` |
| `beq` at 0x3bfdc | `ret(0x3aa60) == 0xfffff9b8` | `ret(0x3aa60) != 0xfffff9b8` |
| `beq` at 0x3c00c | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `bgt` at 0x3c018 | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x3c024 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `beq` at 0x3c034 | `[sp-32] == 0x3` | `[sp-32] != 0x3` |
| `beq` at 0x3c0bc | `[[sp-24]+30]:1 == 0x0` | `[[sp-24]+30]:1 != 0x0` |
| `beq` at 0x3c0d0 | `[[[sp-24]+8]+8] == 0x1` | `[[[sp-24]+8]+8] != 0x1` |
| `beq` at 0x3c0e4 | `[[[sp-24]+8]+8] == 0x2` | `[[[sp-24]+8]+8] != 0x2` |
| `ble` at 0x3c148 | `[[sp-24]+40] <= 0x0 (signed, 32-bit)` | `[[sp-24]+40] > 0x0 (signed, 32-bit)` |
| `beq` at 0x3c288 | `ret(0x3dea4) == 0x0` | `ret(0x3dea4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c2a4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3bb50).

## `power_margin_multi`

- `power_margin_multi`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c2c8 to 0x3cce8.
  Established. The initial zero test takes its branch when entry `r0 == 0`, and falls through
  otherwise (`beq` at 0x3c2e8). After the call to `0x3cd7c` (`bl` at 0x3c33c), the branch is taken
  when its result is zero (`beq` at 0x3c350). That arm clears a word at `[sp-32] +0x0` (`str` at
  0x3c368). Later byte updates include `([[sp-28]+32]:1 & 0xff)` at `[sp-28] +0x1e` (`strb` at 0x3c5f8) and `0x2` at
  `[sp-28] +0x25` (`strb` at 0x3cc30). The final call to `0x3dea4` has candidates `[[sp-28]+8]`,
  `[[sp-28]+30]:1`, `[sp-28]` and `[[sp-28]+30]:1` (`bl` at 0x3cc48). Margin units and physical
  transitions are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c2d8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3c2dc | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x3c2e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c2ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c2f0 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0x3c2fc | load | 4 | `abs 0x3cca8` | `-` |
| `str` at 0x3c300 | store | 4 | `sp -0x34` | `0xe193c` |
| `ldr` at 0x3c308 | load | 4 | `abs 0x3ccac` | `-` |
| `ldr` at 0x3c30c | load | 4 | `abs 0x3ccb0` | `-` |
| `ldr` at 0x3c318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c31c | load | 4 | `(unknown)` | `-` |
| `str` at 0x3c320 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3c324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c328 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c330 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3c344 | store | 4 | `sp -0x24` | `ret(0x3cd7c)` |
| `ldr` at 0x3c348 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c354 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c358 | store | 4 | `sp -0x2c` | `ret(0x3cd7c)` |
| `ldr` at 0x3c360 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c368 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x3c36c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c374 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3c378 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c390 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c394 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c3a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c3b0 | load | 4 | `abs 0x3ccb8` | `-` |
| `ldr` at 0x3c3bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c3c4 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3c3c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c3cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c3d0 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3c3d4 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c3ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c3f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3c3f4 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c40c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c410 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3c424 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c428 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c434 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c438 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c444 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3c454 | load | 4 | `abs 0x3ccbc` | `-` |
| `ldr` at 0x3c468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c46c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c470 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c474 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldrb` at 0x3c478 | load | 1 | `[sp-28] +0x10` | `-` |
| `strb` at 0x3c480 | store | 1 | `[sp-28] +0x1e` | `?` |
| `ldr` at 0x3c484 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c48c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3c490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c498 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3c49c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c4b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4b8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3c4cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4d4 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3c4d8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3c4e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4f0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3c4f4 | load | 4 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3c508 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c50c | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c518 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c51c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c528 | load | 4 | `abs 0x3ccc0` | `-` |
| `ldr` at 0x3c534 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c53c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3c540 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c548 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3c54c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c568 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3c56c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c584 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c588 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c58c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c590 | load | 4 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3c598 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c59c | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3c5a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c5ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c5b0 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldrb` at 0x3c5b4 | load | 1 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3c5c0 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c5c4 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c5d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c5d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c5e0 | load | 4 | `abs 0x3ccc4` | `-` |
| `ldr` at 0x3c5ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c5f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c5f4 | load | 1 | `[sp-28] +0x20` | `-` |
| `strb` at 0x3c5f8 | store | 1 | `[sp-28] +0x1e` | `([[sp-28]+32]:1 & 0xff)` |
| `ldr` at 0x3c5fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c604 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3c60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c610 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3c614 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c62c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c630 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c638 | load | 4 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3c640 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c644 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0x3c650 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c654 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c658 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldrb` at 0x3c65c | load | 1 | `[sp-28] +0x1d` | `-` |
| `ldr` at 0x3c668 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c66c | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c678 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c67c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c688 | load | 4 | `abs 0x3ccc8` | `-` |
| `ldr` at 0x3c694 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c69c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3c6a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c6a4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x3c6b0 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c6b4 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c6c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c6c4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c6d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c6d4 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c6d8 | load | 4 | `abs 0x3cccc` | `-` |
| `ldr` at 0x3c6e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c6ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c6f0 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c6f4 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x3c704 | store | 4 | `sp -0x24` | `ret(0x3dea4)` |
| `ldr` at 0x3c708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c714 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c71c | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x3c720 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c724 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x3c72c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c730 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x3c734 | load | 1 | `[[sp-28]+8] +0x10` | `-` |
| `ldr` at 0x3c73c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c744 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3c74c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c750 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x3c754 | load | 1 | `[[sp-28]+8] +0x10` | `-` |
| `ldr` at 0x3c75c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c760 | load | 1 | `[sp-28] +0x1e` | `-` |
| `str` at 0x3c774 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3c77c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c780 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c784 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3c788 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3c798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c79c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3c7b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c7b4 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3c7c8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3c7d0 | store | 1 | `[sp-28] +0x24` | `0x1` |
| `ldr` at 0x3c7d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c7d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c7dc | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x3c7e0 | store | 4 | `[sp-28] +0xc` | `[[sp-28]+16]` |
| `ldr` at 0x3c7e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c7e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c7ec | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3c7f0 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3c808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c80c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c810 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldrb` at 0x3c814 | load | 1 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3c820 | load | 4 | `abs 0x3ccd0` | `-` |
| `str` at 0x3c824 | store | 4 | `sp -0x2c` | `0xfffff9ba` |
| `ldr` at 0x3c82c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c830 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c834 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldrb` at 0x3c838 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3c844 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c848 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c84c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c850 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3c854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c858 | load | 4 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3c86c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c870 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c874 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldrb` at 0x3c878 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0x3c884 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c888 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c894 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c898 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c8a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c8a8 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c8ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c8b0 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3c8b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c8b8 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0x3c8bc | store | 4 | `sp -0x34` | `[[sp-28]+40]` |
| `ldr` at 0x3c8c0 | load | 4 | `abs 0x3ccd4` | `-` |
| `ldr` at 0x3c8d4 | load | 4 | `abs 0x3ccd8` | `-` |
| `str` at 0x3c8d8 | store | 4 | `sp -0x2c` | `0xfffff9b7` |
| `ldr` at 0x3c8e0 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3c8e4 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3c8f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c8f4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3c900 | load | 4 | `abs 0x3ccdc` | `-` |
| `ldr` at 0x3c90c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c910 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x3c914 | load | 1 | `[[sp-28]+8] +0x10` | `-` |
| `ldr` at 0x3c91c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c924 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3c92c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c930 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x3c934 | load | 1 | `[[sp-28]+8] +0x10` | `-` |
| `ldr` at 0x3c93c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c940 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3c950 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c958 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `str` at 0x3c960 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3c968 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c96c | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3c980 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c984 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c988 | load | 1 | `[sp-28] +0x1e` | `-` |
| `strb` at 0x3c98c | store | 1 | `[sp-28] +0x21` | `([[sp-28]+30]:1 & 0xff)` |
| `ldr` at 0x3c990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c994 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c998 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3c99c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c9a0 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0x3c9b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c9b4 | load | 1 | `[sp-28] +0x25` | `-` |
| `str` at 0x3c9d0 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0x3c9d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c9dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c9e0 | load | 1 | `[sp-28] +0x24` | `-` |
| `str` at 0x3c9e4 | store | 4 | `[sp-28] +0x10` | `[[sp-28]+36]:1` |
| `ldr` at 0x3c9ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c9f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c9f4 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldrb` at 0x3c9f8 | load | 1 | `[sp-28] +0x1d` | `-` |
| `ldr` at 0x3ca04 | load | 4 | `abs 0x3ccd0` | `-` |
| `str` at 0x3ca08 | store | 4 | `sp -0x2c` | `0xfffff9ba` |
| `ldr` at 0x3ca10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ca14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ca18 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldrb` at 0x3ca1c | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0x3ca28 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ca2c | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3ca30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ca34 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0x3ca38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ca3c | load | 4 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3ca50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ca54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ca58 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldrb` at 0x3ca5c | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0x3ca68 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3ca6c | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3ca78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ca7c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3ca88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ca8c | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3ca90 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ca94 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0x3ca98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ca9c | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0x3caa0 | store | 4 | `sp -0x34` | `[[sp-28]+40]` |
| `ldr` at 0x3caa4 | load | 4 | `abs 0x3cce0` | `-` |
| `ldr` at 0x3cab8 | load | 4 | `abs 0x3ccd8` | `-` |
| `str` at 0x3cabc | store | 4 | `sp -0x2c` | `0xfffff9b7` |
| `ldr` at 0x3cac4 | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3cac8 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3cad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cad8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3cae4 | load | 4 | `abs 0x3ccdc` | `-` |
| `ldr` at 0x3caf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3caf4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x3caf8 | load | 1 | `[[sp-28]+8] +0x10` | `-` |
| `ldr` at 0x3cb00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cb08 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3cb10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cb14 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x3cb18 | load | 1 | `[[sp-28]+8] +0x10` | `-` |
| `ldr` at 0x3cb20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cb24 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3cb34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3cb3c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `str` at 0x3cb44 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3cb4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cb50 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3cb64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cb68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cb6c | load | 1 | `[sp-28] +0x1e` | `-` |
| `strb` at 0x3cb70 | store | 1 | `[sp-28] +0x20` | `([[sp-28]+30]:1 & 0xff)` |
| `ldr` at 0x3cb74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cb78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cb7c | load | 1 | `[sp-28] +0x20` | `-` |
| `str` at 0x3cb80 | store | 4 | `sp -0x30` | `[[sp-28]+32]:1` |
| `ldr` at 0x3cb84 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cb88 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0x3cb8c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3cb94 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3cb98 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3cba4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3cba8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cbac | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0x3cbb0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3cbc8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3cbcc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3cbd4 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0x3cbdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cbe0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cbe4 | load | 1 | `[sp-28] +0x24` | `-` |
| `str` at 0x3cbec | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0x3cbf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cbf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cbf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cbfc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3cc00 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0x3cc08 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0x3cc0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cc10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cc14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cc18 | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldrb` at 0x3cc1c | load | 1 | `[sp-28] +0x10` | `-` |
| `strb` at 0x3cc24 | store | 1 | `[sp-28] +0x1e` | `(([[sp-28]+30]:1 + [[sp-28]+16]:1) & 0xff)` |
| `ldr` at 0x3cc28 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3cc30 | store | 1 | `[sp-28] +0x25` | `0x2` |
| `ldr` at 0x3cc34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cc38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3cc3c | load | 1 | `[sp-28] +0x1e` | `-` |
| `ldr` at 0x3cc40 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x3cc50 | store | 4 | `sp -0x24` | `ret(0x3dea4)` |
| `ldr` at 0x3cc54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cc60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cc64 | store | 4 | `sp -0x2c` | `ret(0x3dea4)` |
| `ldr` at 0x3cc6c | load | 4 | `abs 0x3ccb4` | `-` |
| `ldr` at 0x3cc70 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3cc7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cc80 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3cc8c | load | 4 | `abs 0x3cce4` | `-` |
| `str` at 0x3cc9c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3cca0 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3cca4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3cca4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3cca4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3cca4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3cca4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c314 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x6cc` | `0x0` |
| `bl` at 0x3c33c | `(unknown callee) 0x3cd7c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x3c37c | `(unknown callee) 0xd6930` | `[[sp-28]+20]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c3a8 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3c3b8 | `(unknown callee) 0x20f14` | `0xe1978` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3c3d8 | `(unknown callee) 0xd6928` | `[[sp-28]+20]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c3f8 | `(unknown callee) 0xd6920` | `[sp-40]` | `[[sp-28]+24]` | `?` | `[sp-28]` |
| `bl` at 0x3c43c | `(unknown callee) 0x3d108` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3c448 | `(unknown callee) 0xd5c38` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x3c464 | `(unknown callee) 0x20f14` | `0xe1998` | `ret(0x3d108)` | `ret(0xd5c38)` | `?` |
| `bl` at 0x3c4a0 | `(unknown callee) 0xd6920` | `[[sp-28]+20]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c520 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c530 | `(unknown callee) 0x20f14` | `0xe19d0` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3c550 | `(unknown callee) 0xd6928` | `[[sp-28]+20]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c570 | `(unknown callee) 0xd6928` | `[sp-40]` | `[[sp-28]+24]` | `?` | `[sp-28]` |
| `bl` at 0x3c5d8 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+32]:1` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c5e8 | `(unknown callee) 0x20f14` | `0xe1a08` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3c618 | `(unknown callee) 0xd6920` | `[sp-40]` | `[[sp-28]+24]` | `?` | `[sp-28]` |
| `bl` at 0x3c680 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+33]:1` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c690 | `(unknown callee) 0x20f14` | `0xe1a34` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3c6c8 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3c6e4 | `(unknown callee) 0x20f14` | `0xe1a70` | `ret(0x3d108)` | `[[sp-28]+30]:1` | `[[sp-28]+30]:1` |
| `bl` at 0x3c6fc | `(unknown callee) 0x3dea4` | `[[sp-28]+8]` | `[[sp-28]+30]:1` | `[sp-28]` | `[[sp-28]+30]:1` |
| `bl` at 0x3c748 | `(unknown callee) 0x1abd0` | `[[[sp-28]+8]+16]:1+20` | `[[sp-28]+4]` | `[sp-28]` | `[[[sp-28]+8]+16]:1+20` |
| `bl` at 0x3c76c | `(unknown callee) 0x1abd0` | `[[[sp-28]+8]+16]:1+130` | `[[sp-28]+30]:1` | `[[[sp-28]+8]+16]:1+130` | `[[sp-28]+30]:1` |
| `bl` at 0x3c7f4 | `(unknown callee) 0xd6928` | `[[sp-28]+20]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c89c | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+32]:1` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c8d0 | `(unknown callee) 0x20f14` | `0xe1a90` | `ret(0x3d108)` | `[[sp-28]+30]:1` | `[[sp-28]+32]:1` |
| `bl` at 0x3c8f8 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+32]:1` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3c908 | `(unknown callee) 0x20f14` | `0xe1ac0` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3c928 | `(unknown callee) 0x1abd0` | `[[[sp-28]+8]+16]:1+20` | `[[sp-28]+4]` | `[sp-28]` | `[[[sp-28]+8]+16]:1+20` |
| `bl` at 0x3c94c | `(unknown callee) 0x1abd0` | `[[[sp-28]+8]+16]:1+130` | `[[sp-28]+30]:1` | `[[[sp-28]+8]+16]:1+130` | `[[sp-28]+30]:1` |
| `bl` at 0x3c9c0 | `(unknown callee) 0xd569c` | `?` | `[[sp-28]+37]:1` | `?` | `[[sp-28]+37]:1` |
| `bl` at 0x3ca80 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+33]:1` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3cab4 | `(unknown callee) 0x20f14` | `0xe1ae8` | `ret(0x3d108)` | `[[sp-28]+30]:1` | `[[sp-28]+33]:1` |
| `bl` at 0x3cadc | `(unknown callee) 0x3d108` | `[[sp-28]]` | `[[sp-28]+33]:1` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3caec | `(unknown callee) 0x20f14` | `0xe1ac0` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3cb0c | `(unknown callee) 0x1abd0` | `[[[sp-28]+8]+16]:1+20` | `[[sp-28]+4]` | `[sp-28]` | `[[[sp-28]+8]+16]:1+20` |
| `bl` at 0x3cb30 | `(unknown callee) 0x1abd0` | `[[[sp-28]+8]+16]:1+130` | `[[sp-28]+30]:1` | `[[[sp-28]+8]+16]:1+130` | `[[sp-28]+30]:1` |
| `bl` at 0x3cbb8 | `(unknown callee) 0xd569c` | `[sp-48]` | `[[sp-28]+37]:1` | `?` | `[[sp-28]+37]:1` |
| `bl` at 0x3cc48 | `(unknown callee) 0x3dea4` | `[[sp-28]+8]` | `[[sp-28]+30]:1` | `[sp-28]` | `[[sp-28]+30]:1` |
| `bl` at 0x3cc84 | `(unknown callee) 0x3d108` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3cc94 | `(unknown callee) 0x20f14` | `0xe1b18` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c2e8 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x3c2f8 | `[r0+8] != 0x0` | `[r0+8] == 0x0` |
| `beq` at 0x3c350 | `ret(0x3cd7c) == 0x0` | `ret(0x3cd7c) != 0x0` |
| `beq` at 0x3c388 | `ret(0xd6930) == 0x0` | `ret(0xd6930) != 0x0` |
| `beq` at 0x3c39c | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `blt` at 0x3c3e4 | `ret(0xd6928) < 0x0 (signed, 32-bit)` | `ret(0xd6928) >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3c404 | `ret(0xd6920) >= 0x0 (signed, 32-bit)` | `ret(0xd6920) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3c420 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x3c430 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `bge` at 0x3c4ac | `ret(0xd6920) >= 0x0 (signed, 32-bit)` | `ret(0xd6920) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3c4c8 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x3c4e4 | `([[sp-28]+16] + [[sp-28]+12]) != 0x0` | `([[sp-28]+16] + [[sp-28]+12]) == 0x0` |
| `bgt` at 0x3c500 | `? > 0x0 (signed, 32-bit)` | `? <= 0x0 (signed, 32-bit)` |
| `beq` at 0x3c514 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `blt` at 0x3c55c | `ret(0xd6928) < 0x0 (signed, 32-bit)` | `ret(0xd6928) >= 0x0 (signed, 32-bit)` |
| `blt` at 0x3c57c | `ret(0xd6928) < 0x0 (signed, 32-bit)` | `ret(0xd6928) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x3c5a4 | `([[sp-28]+30]:1 + [[sp-28]+40]) != [[sp-28]+32]:1` | `([[sp-28]+30]:1 + [[sp-28]+40]) == [[sp-28]+32]:1` |
| `beq` at 0x3c5bc | `[[sp-28]+32]:1 == [[sp-28]+28]:1` | `[[sp-28]+32]:1 != [[sp-28]+28]:1` |
| `beq` at 0x3c5cc | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `bgt` at 0x3c624 | `ret(0xd6920) > 0x0 (signed, 32-bit)` | `ret(0xd6920) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x3c64c | `? != [[sp-28]+33]:1` | `? == [[sp-28]+33]:1` |
| `beq` at 0x3c664 | `[[sp-28]+33]:1 == [[sp-28]+29]:1` | `[[sp-28]+33]:1 != [[sp-28]+29]:1` |
| `beq` at 0x3c674 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `beq` at 0x3c6ac | `[[sp-32]] == 0x0` | `[[sp-32]] != 0x0` |
| `beq` at 0x3c6bc | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `beq` at 0x3c710 | `ret(0x3dea4) == 0x0` | `ret(0x3dea4) != 0x0` |
| `bge` at 0x3c794 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bne` at 0x3c7ac | `? != 0x2` | `? == 0x2` |
| `bne` at 0x3c7c4 | `? != 0x2` | `? == 0x2` |
| `blt` at 0x3c800 | `ret(0xd6928) < 0x0 (signed, 32-bit)` | `ret(0xd6928) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x3c81c | `[[sp-28]+30]:1 != [[sp-28]+28]:1` | `[[sp-28]+30]:1 == [[sp-28]+28]:1` |
| `beq` at 0x3c840 | `[[sp-28]+30]:1 == [[sp-28]+32]:1` | `[[sp-28]+30]:1 != [[sp-28]+32]:1` |
| `beq` at 0x3c864 | `[[sp-28]+30]:1 == ([[sp-28]+32]:1 + [[sp-28]+40])` | `[[sp-28]+30]:1 != ([[sp-28]+32]:1 + [[sp-28]+40])` |
| `bne` at 0x3c880 | `[[sp-28]+32]:1 != [[sp-28]+34]:1` | `[[sp-28]+32]:1 == [[sp-28]+34]:1` |
| `beq` at 0x3c890 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `beq` at 0x3c8ec | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `ble` at 0x3c97c | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `bne` at 0x3ca00 | `[[sp-28]+30]:1 != [[sp-28]+29]:1` | `[[sp-28]+30]:1 == [[sp-28]+29]:1` |
| `beq` at 0x3ca24 | `[[sp-28]+30]:1 == [[sp-28]+33]:1` | `[[sp-28]+30]:1 != [[sp-28]+33]:1` |
| `beq` at 0x3ca48 | `[[sp-28]+30]:1 == ?` | `[[sp-28]+30]:1 != ?` |
| `bne` at 0x3ca64 | `[[sp-28]+33]:1 != [[sp-28]+35]:1` | `[[sp-28]+33]:1 == [[sp-28]+35]:1` |
| `beq` at 0x3ca74 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `beq` at 0x3cad0 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `ble` at 0x3cb60 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `beq` at 0x3cc5c | `ret(0x3dea4) == 0x0` | `ret(0x3dea4) != 0x0` |
| `beq` at 0x3cc78 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3cca4 | `r0` | `?` |

## `L1CfgOptimize`

- `L1CfgOptimize`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57930 to 0x58ba4.
  Established. The first equality branch is taken when `[0x11a0f4] == 0`, and falls through
  otherwise (`beq` at 0x5794c). Word updates include `[0x11a0f0]+5` at `0x11a0f4` (`str` at
  0x579f0), `ret(0x189c8)` at `0x11a0f0` (`str` at 0x57a0c) and `ret(0x18920)` at `0x11a32c` (`str`
  at 0x57aa8). Later stores clear the word at `0x11a414` (`str` at 0x57fa0) and decrement the word
  at `0x11a438` (`str` at 0x58a84). A call to `0x56b54` uses `[sp-128]` as its first candidate (`bl`
  at 0x58b8c). Election policy and the meanings of the stored words remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x57940 | load | 4 | `abs 0x588bc` | `-` |
| `ldr` at 0x57944 | load | 4 | `abs 0x11a0f4` | `-` |
| `ldr` at 0x57950 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x57954 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57964 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x57968 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5796c | load | 4 | `abs 0x588c8` | `-` |
| `ldr` at 0x57970 | load | 4 | `abs 0x588cc` | `-` |
| `ldr` at 0x57980 | load | 4 | `abs 0x588d0` | `-` |
| `ldr` at 0x57984 | load | 4 | `abs 0x11a0f0` | `-` |
| `ldr` at 0x5799c | load | 4 | `abs 0x588d0` | `-` |
| `ldr` at 0x579a0 | load | 4 | `abs 0x11a0f0` | `-` |
| `str` at 0x579a8 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x579ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x579b8 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x579bc | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x579cc | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x579d0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x579d4 | load | 4 | `abs 0x588d4` | `-` |
| `ldr` at 0x579d8 | load | 4 | `abs 0x588cc` | `-` |
| `ldr` at 0x579e0 | load | 4 | `abs 0x588bc` | `-` |
| `ldr` at 0x579e4 | load | 4 | `abs 0x588d0` | `-` |
| `ldr` at 0x579e8 | load | 4 | `abs 0x11a0f0` | `-` |
| `str` at 0x579f0 | store | 4 | `abs 0x11a0f4` | `[0x11a0f0]+5` |
| `ldr` at 0x57a08 | load | 4 | `abs 0x588d0` | `-` |
| `str` at 0x57a0c | store | 4 | `abs 0x11a0f0` | `ret(0x189c8)` |
| `ldr` at 0x57a10 | load | 4 | `abs 0x588d8` | `-` |
| `ldr` at 0x57a14 | load | 4 | `abs 0x11a330` | `-` |
| `ldr` at 0x57a20 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57a24 | load | 4 | `abs 0x588d8` | `-` |
| `ldr` at 0x57a28 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x57a2c | load | 4 | `abs 0x11a330` | `-` |
| `ldr` at 0x57a3c | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x57a40 | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x57a4c | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x57a50 | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x57a58 | load | 4 | `abs 0x588d8` | `-` |
| `ldr` at 0x57a5c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57a60 | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x57a7c | store | 4 | `abs 0x11a330` | `?` |
| `ldr` at 0x57a80 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x57a84 | load | 4 | `abs 0x588d8` | `-` |
| `ldr` at 0x57a88 | load | 4 | `abs 0x11a330` | `-` |
| `str` at 0x57aa8 | store | 4 | `abs 0x11a32c` | `ret(0x18920)` |
| `ldr` at 0x57aac | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x57ab0 | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x57ac0 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x57ac4 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57ac8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57acc | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x57ae0 | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x57ae4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57af0 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57af4 | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x57af8 | store | 4 | `sp -0x7c` | `[0x11a42c]` |
| `ldr` at 0x57afc | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x57b00 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57b10 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x57b14 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x57b18 | load | 4 | `abs 0x588e0` | `-` |
| `str` at 0x57b28 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x57b2c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57b30 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57b34 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x57b44 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57b48 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57b64 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57b6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57b78 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x57b7c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57b8c | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x57b90 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x57b94 | load | 4 | `abs 0x588e4` | `-` |
| `ldr` at 0x57b98 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57ba4 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x57ba8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57bb8 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x57bbc | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57bc0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57bdc | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57be4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x57be8 | load | 4 | `abs 0x588e8` | `-` |
| `ldr` at 0x57bec | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57bf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x57bfc | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x57c00 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57c04 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57c20 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57c28 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57c2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57c3c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57c40 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57c5c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57c64 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57c78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x57c80 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x57c84 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57c88 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57c8c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57c90 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57c94 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57c98 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x57c9c | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x57ca0 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x57cac | store | 4 | `sp -0x9c` | `ret(0x59b44)` |
| `ldr` at 0x57cb0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57cbc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57cc0 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x57cc8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57ccc | load | 4 | `ret(0x59b44) +0x14` | `-` |
| `ldr` at 0x57cd8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57cdc | load | 4 | `sp -0x80` | `-` |
| `str` at 0x57ce0 | store | 4 | `ret(0x59b44) +0x10` | `[sp-128]` |
| `ldr` at 0x57ce4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57ce8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57cec | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x57cf4 | store | 4 | `[sp-156] +0x14` | `[[sp-152]+20]+1` |
| `ldr` at 0x57cf8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57cfc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57d00 | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x57d04 | store | 4 | `[sp-156] +0xc` | `[[sp-152]+12]` |
| `ldr` at 0x57d08 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x57d0c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57d1c | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x57d20 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57d24 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57d28 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57d2c | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x57d34 | store | 4 | `sp -0xc0` | `[[sp-156]+16]+64` |
| `ldr` at 0x57d38 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57d3c | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x57d40 | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x57d44 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x57d48 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x57d4c | store | 4 | `sp -0xb8` | `[[sp-156]+20]` |
| `ldr` at 0x57d50 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x57d54 | load | 4 | `abs 0x588ec` | `-` |
| `ldr` at 0x57d58 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x57d5c | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x57d68 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57d6c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57d70 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57d8c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57d94 | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x57d98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57da4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57da8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57dac | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x57db0 | store | 4 | `sp -0xc0` | `[[sp-152]+4]` |
| `ldr` at 0x57db4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57db8 | load | 4 | `[sp-152] +0x8` | `-` |
| `str` at 0x57dbc | store | 4 | `sp -0xbc` | `[[sp-152]+8]` |
| `ldr` at 0x57dc0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57dc4 | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x57dc8 | store | 4 | `sp -0xb8` | `[[sp-152]+12]` |
| `ldr` at 0x57dcc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57dd0 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x57dd8 | store | 4 | `sp -0xb4` | `[[sp-152]+20]+1` |
| `ldr` at 0x57ddc | load | 4 | `sp -0x80` | `-` |
| `str` at 0x57de0 | store | 4 | `sp -0xb0` | `[sp-128]` |
| `ldr` at 0x57de4 | load | 4 | `abs 0x588f0` | `-` |
| `ldr` at 0x57de8 | load | 4 | `abs 0x588f4` | `-` |
| `ldr` at 0x57dec | load | 4 | `abs 0x588f8` | `-` |
| `ldr` at 0x57df0 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x57dfc | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x57e00 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x57e10 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x57e14 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57e18 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57e1c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x57e24 | store | 4 | `sp -0xc0` | `[sp-128]+64` |
| `ldr` at 0x57e28 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57e2c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x57e48 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x57e50 | load | 4 | `(unknown)` | `-` |
| `str` at 0x57e58 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x57e5c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x57e60 | load | 4 | `[sp-152] +0x10` | `-` |
| `str` at 0x57e68 | store | 4 | `sp -0xb8` | `[[sp-152]+16]+64` |
| `ldr` at 0x57e6c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x57e70 | load | 4 | `abs 0x588fc` | `-` |
| `ldr` at 0x57e74 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x57e78 | load | 4 | `[sp-152] +0x4` | `-` |
| `ldr` at 0x57e80 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x57e88 | store | 4 | `sp -0x84` | `[sp-132]+1` |
| `ldr` at 0x57e90 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x57e98 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `str` at 0x57ea4 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x57ea8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57eac | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57eb0 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x57ec0 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57ec4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57ed8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57ee0 | load | 4 | `abs 0x58900` | `-` |
| `ldr` at 0x57ee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57ee8 | load | 4 | `abs 0x1155f0` | `-` |
| `ldr` at 0x57ef4 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57ef8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57efc | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57f00 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57f14 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57f1c | load | 4 | `(unknown)` | `-` |
| `str` at 0x57f20 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x57f24 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57f28 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57f3c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57f44 | load | 4 | `(unknown)` | `-` |
| `str` at 0x57f48 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x57f4c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x57f50 | load | 4 | `abs 0x588f4` | `-` |
| `ldr` at 0x57f54 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x57f58 | load | 4 | `abs 0x58904` | `-` |
| `ldr` at 0x57f60 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x57f68 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x57f70 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57f74 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57f78 | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x57f7c | load | 4 | `abs 0x11a3ac` | `-` |
| `str` at 0x57f88 | store | 4 | `sp -0x90` | `ret(0x5e764)` |
| `ldr` at 0x57f8c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57f90 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x57f98 | store | 4 | `ret(0x5e764) +0xc` | `0x0` |
| `str` at 0x57fa0 | store | 4 | `abs 0x11a414` | `0x0` |
| `ldr` at 0x57fa4 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57fa8 | load | 4 | `abs 0x11a3f8` | `-` |
| `ldr` at 0x57fb4 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57fb8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x57fc0 | store | 4 | `[sp-144] +0xc` | `?` |
| `str` at 0x57fc8 | store | 4 | `abs 0x11a414` | `?` |
| `ldr` at 0x57fcc | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57fd0 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57fd4 | load | 4 | `abs 0x11a3a8` | `-` |
| `str` at 0x57fd8 | store | 4 | `abs 0x11a418` | `[0x11a3a8]` |
| `ldr` at 0x57fdc | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57fe0 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x57fe4 | load | 4 | `abs 0x11a3ac` | `-` |
| `str` at 0x57fe8 | store | 4 | `abs 0x11a41c` | `[0x11a3ac]` |
| `str` at 0x57ff8 | store | 4 | `sp -0x94` | `ret(0x59db8)` |
| `ldr` at 0x57ffc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x58008 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x5800c | load | 4 | `ret(0x59db8) +0xc` | `-` |
| `ldr` at 0x58018 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x5801c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x58020 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x58024 | store | 4 | `[sp-144] +0xc` | `[[sp-148]+16]` |
| `ldr` at 0x58028 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5802c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x58030 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x58034 | store | 4 | `abs 0x11a414` | `[[sp-148]+16]` |
| `ldr` at 0x58038 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5803c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x58040 | load | 4 | `[sp-148] +0x0` | `-` |
| `str` at 0x58044 | store | 4 | `abs 0x11a418` | `[[sp-148]]` |
| `ldr` at 0x58048 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5804c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x58050 | load | 4 | `[sp-148] +0x4` | `-` |
| `str` at 0x58054 | store | 4 | `abs 0x11a41c` | `[[sp-148]+4]` |
| `ldr` at 0x58058 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5805c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x58060 | load | 4 | `[sp-148] +0x14` | `-` |
| `str` at 0x58064 | store | 4 | `abs 0x11a420` | `[[sp-148]+20]` |
| `ldr` at 0x58068 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x5806c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5807c | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x58080 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58084 | load | 4 | `abs 0x58908` | `-` |
| `str` at 0x58090 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x58094 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58098 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x5809c | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x580a8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x580ac | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x580b0 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x580b4 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x580c8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x580cc | load | 4 | `abs 0x11a32c` | `-` |
| `str` at 0x580ec | store | 4 | `sp -0xa4` | `0x1` |
| `ldr` at 0x580f0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x580f4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x580f8 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x580fc | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5810c | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x58110 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58114 | load | 4 | `abs 0x5890c` | `-` |
| `ldr` at 0x58118 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x58120 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5812c | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x58130 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58140 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x58144 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58148 | load | 4 | `abs 0x58910` | `-` |
| `str` at 0x58154 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x58158 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5815c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58160 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5816c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x58174 | store | 4 | `sp -0x8c` | `0x1` |
| `ldr` at 0x5817c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58184 | load | 4 | `abs 0x58914` | `-` |
| `ldr` at 0x58188 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5818c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x581a0 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x581a4 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x581b4 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x581bc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x581c0 | load | 4 | `abs 0x58918` | `-` |
| `str` at 0x581cc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x581d0 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x581d4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x581d8 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x581e8 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x581ec | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x581f0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58204 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5820c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58210 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58224 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5822c | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x58230 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58234 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58238 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58244 | store | 4 | `sp -0x9c` | `ret(0x59b44)` |
| `ldr` at 0x58248 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58254 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x5825c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x58264 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58268 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x5827c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x58284 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58288 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x5829c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x582a4 | load | 4 | `abs 0x5891c` | `-` |
| `ldr` at 0x582a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x582ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x582c0 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x582c4 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x582d4 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x582dc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x582e0 | load | 4 | `abs 0x58918` | `-` |
| `ldr` at 0x582e8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x582f0 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x582fc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x58300 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58304 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58314 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58318 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5831c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58330 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x58334 | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x5833c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58350 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x58354 | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x5835c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x58360 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x58364 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58368 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58374 | store | 4 | `sp -0x9c` | `ret(0x59b44)` |
| `ldr` at 0x58378 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58384 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x5838c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x58394 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x583a8 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x583ac | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x583b4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x583c8 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x583cc | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x583d8 | load | 4 | `abs 0x58920` | `-` |
| `ldr` at 0x583dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x583e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x583f4 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x583f8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58408 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x58410 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58414 | load | 4 | `abs 0x58918` | `-` |
| `ldr` at 0x5841c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x58424 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x58430 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x58434 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58438 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x5843c | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5844c | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58450 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58464 | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x5846c | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x58470 | load | 4 | `abs 0x588dc` | `-` |
| `ldr` at 0x58474 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58478 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x5847c | load | 4 | `abs 0x11a32c` | `-` |
| `ldr` at 0x58480 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58484 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58488 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58494 | store | 4 | `sp -0x98` | `ret(0x59b44)` |
| `ldr` at 0x58498 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x584a4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x584a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x584ac | load | 4 | `[sp-156] +0x14` | `-` |
| `ldr` at 0x584b0 | load | 4 | `ret(0x59b44) +0x14` | `-` |
| `ldr` at 0x584bc | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x584c4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x584cc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x584d0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x584d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x584d8 | load | 4 | `ret(0x59b44) +0x14` | `-` |
| `str` at 0x584dc | store | 4 | `sp -0xc0` | `[ret(0x59b44)+20]` |
| `ldr` at 0x584e0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x584e4 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x584e8 | store | 4 | `sp -0xbc` | `[[sp-156]+20]` |
| `ldr` at 0x584f0 | load | 4 | `abs 0x58924` | `-` |
| `ldr` at 0x584f4 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x584f8 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x5850c | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x58510 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58520 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x58528 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5852c | load | 4 | `abs 0x58918` | `-` |
| `ldr` at 0x58534 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58540 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58544 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58548 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x5854c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58558 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x58560 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x58568 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x5856c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58570 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58574 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58578 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x5857c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58580 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x58584 | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x5858c | load | 4 | `abs 0x58928` | `-` |
| `ldr` at 0x58590 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x58594 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x585a8 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x585ac | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x585bc | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x585c4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x585c8 | load | 4 | `abs 0x58918` | `-` |
| `ldr` at 0x585d0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x585dc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x585e0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x585e4 | load | 4 | `[sp-156] +0xc` | `-` |
| `ldr` at 0x585e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x585f4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x585fc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x58604 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58608 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x5860c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58610 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58614 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x58618 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x5861c | load | 4 | `[sp-156] +0xc` | `-` |
| `str` at 0x58620 | store | 4 | `sp -0xbc` | `[[sp-156]+12]` |
| `ldr` at 0x58628 | load | 4 | `abs 0x5892c` | `-` |
| `ldr` at 0x5862c | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x58630 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x58644 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x58648 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58658 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x58660 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58664 | load | 4 | `abs 0x58918` | `-` |
| `ldr` at 0x5866c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x58674 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x58684 | load | 4 | `abs 0x58930` | `-` |
| `ldr` at 0x58688 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5869c | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x586a0 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x586b0 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x586b8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x586bc | load | 4 | `abs 0x58918` | `-` |
| `ldr` at 0x586c4 | load | 4 | `abs 0x588c4` | `-` |
| `ldr` at 0x586c8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x586cc | load | 4 | `abs 0x58910` | `-` |
| `str` at 0x586d8 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x586dc | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x586e0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x586e4 | load | 4 | `abs 0x11a438` | `-` |
| `str` at 0x586f8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x586fc | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58700 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58704 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58708 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5871c | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x58724 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58728 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5873c | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x58744 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x58748 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5874c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58750 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5875c | store | 4 | `sp -0x9c` | `ret(0x59b44)` |
| `ldr` at 0x58760 | load | 4 | `abs 0x588c0` | `-` |
| `ldr` at 0x58764 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58774 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58784 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x5878c | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x58790 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x5879c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x587a0 | store | 4 | `sp -0xa8` | `[sp-160]` |
| `ldr` at 0x587a4 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x587a8 | load | 4 | `abs 0x11a3f8` | `-` |
| `ldr` at 0x587b4 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x587b8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x587bc | load | 4 | `ret(0x59b44) +0x10` | `-` |
| `ldr` at 0x587d8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x587e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x587ec | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x587f4 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x587f8 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x587fc | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x58800 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x58804 | store | 4 | `sp -0xac` | `[sp-160]` |
| `ldr` at 0x58808 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x58818 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5881c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58820 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5883c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x58844 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58850 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x58858 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x5885c | load | 4 | `sp -0xac` | `-` |
| `str` at 0x58860 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x58864 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58870 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58874 | load | 4 | `abs 0x11a3f8` | `-` |
| `ldr` at 0x58880 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58884 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58888 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x588a4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x588ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58948 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x58958 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x5895c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x58960 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5897c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x58984 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58990 | load | 4 | `abs 0x58938` | `-` |
| `ldr` at 0x58994 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x589a4 | load | 4 | `abs 0x5893c` | `-` |
| `ldr` at 0x589a8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x589ac | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x589c0 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x589c8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x589cc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x589e0 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x589e8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x589ec | store | 4 | `sp -0xc0` | `[sp-160]` |
| `ldr` at 0x589f0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x589f4 | load | 4 | `abs 0x58934` | `-` |
| `ldr` at 0x589f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x589fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58a04 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58a08 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x58a1c | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x58a24 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58a28 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x58a3c | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x58a48 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58a4c | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x58a50 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x58a74 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58a78 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58a7c | load | 4 | `abs 0x11a438` | `-` |
| `str` at 0x58a84 | store | 4 | `abs 0x11a438` | `[0x11a438]-1` |
| `ldr` at 0x58a88 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x58a90 | store | 4 | `sp -0x88` | `[sp-136]-1` |
| `ldr` at 0x58a94 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x58a9c | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x58aa8 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58aac | load | 4 | `abs 0x11a410` | `-` |
| `ldr` at 0x58abc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x58ac8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x58ad4 | load | 4 | `abs 0x58938` | `-` |
| `ldr` at 0x58ad8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x58ae8 | load | 4 | `abs 0x5893c` | `-` |
| `ldr` at 0x58aec | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x58af0 | load | 4 | `abs 0x58940` | `-` |
| `str` at 0x58afc | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x58b00 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58b04 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x58b08 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x58b18 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58b1c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x58b38 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x58b40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58b50 | load | 4 | `abs 0x58944` | `-` |
| `ldr` at 0x58b54 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x58b70 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x58b78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58b88 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x58b90 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x58b98 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldmdb` at 0x58ba0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x58ba0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58ba0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58ba0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57974 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6444` | `0x281ef8` | `0x116928` |
| `bl` at 0x57978 | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `?` |
| `bl` at 0x57994 | `(unknown callee) 0x189c8` | `0x0` | `r1` | `r2` | `[0x11a0f0]` |
| `bl` at 0x579dc | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6464` | `0x281ef8` | `0x116928` |
| `bl` at 0x579f4 | `(unknown callee) 0x5659c` | `?` | `?` | `0x11a0f4` | `[0x11a0f0]+5` |
| `bl` at 0x57a00 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x57a54 | `(unknown callee) 0x18be4` | `[0x11a32c]` | `?` | `?` | `0x11a32c` |
| `bl` at 0x57aa0 | `(unknown callee) 0x18920` | `?` | `0x11a330` | `[0x11a330]` | `?` |
| `bl` at 0x57abc | `(unknown callee) 0x18c20` | `[0x11a32c]` | `0x0` | `0x4` | `0x11a32c` |
| `bl` at 0x57aec | `(unknown callee) 0x18ae8` | `[0x11a32c]` | `[0x11a434]` | `?` | `?` |
| `bl` at 0x57b1c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6480` | `?` | `0x116928` |
| `bl` at 0x57b20 | `(unknown callee) 0x55b6c` | `?` | `?` | `?` | `?` |
| `bl` at 0x57b9c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6494` | `[sp-128]` | `0x116928` |
| `bl` at 0x57bf4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe64b4` | `[sp-128]` | `?` |
| `bl` at 0x57ca4 | `(unknown callee) 0x59b44` | `[0x11a434]` | `[0x11a42c]` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x57d60 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe64d4` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x57df4 | `(unknown callee) 0x5abf4` | `0x11a434` | `0x11a42c` | `0x11a430` | `[[sp-152]]` |
| `bl` at 0x57e7c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6504` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x57f5c | `(unknown callee) 0x5b2bc` | `[0x11a434]` | `0x11a42c` | `[0x11a440]` | `0x11a438` |
| `bl` at 0x57f80 | `(unknown callee) 0x5e764` | `[0x11a3a8]` | `[0x11a3ac]` | `0x11a3a0` | `0x11a3a0` |
| `bl` at 0x57ff0 | `(unknown callee) 0x59db8` | `ret(0x5e764)` | `0x11a3a0` | `ret(0x5e764)` | `[0x11a3f8]` |
| `bl` at 0x58088 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe652c` | `?` | `0x116928` |
| `bl` at 0x580d4 | `(unknown callee) 0x18a58` | `[0x11a434]` | `[0x11a32c]` | `?` | `?` |
| `bl` at 0x5811c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6540` | `?` | `0x116928` |
| `bl` at 0x5814c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6558` | `?` | `0x116928` |
| `bl` at 0x58190 | `(unknown callee) 0x18c74` | `sp-120` | `0xe657c` | `[0x11a42c]` | `[sp-124]` |
| `bl` at 0x5819c | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x581c4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x5823c | `(unknown callee) 0x59b44` | `[0x11a32c]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x582b0 | `(unknown callee) 0x18c74` | `sp-120` | `0xe65a8` | `?` | `?` |
| `bl` at 0x582bc | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x582e4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x5836c | `(unknown callee) 0x59b44` | `[0x11a434]` | `[0x11a42c]` | `?` | `?` |
| `bl` at 0x583e4 | `(unknown callee) 0x18c74` | `sp-120` | `0xe65b8` | `?` | `?` |
| `bl` at 0x583f0 | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x58418 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x5848c | `(unknown callee) 0x59b44` | `[0x11a32c]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x584fc | `(unknown callee) 0x18c74` | `sp-120` | `0xe65cc` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x58508 | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x58530 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x58598 | `(unknown callee) 0x18c74` | `sp-120` | `0xe65e4` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x585a4 | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x585cc | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x58634 | `(unknown callee) 0x18c74` | `sp-120` | `0xe6600` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x58640 | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x58668 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x5868c | `(unknown callee) 0x18c74` | `sp-120` | `0xe661c` | `?` | `sp-120` |
| `bl` at 0x58698 | `(unknown callee) 0x57654` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x586c0 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe65a0` | `sp-120` | `0x116928` |
| `bl` at 0x586d0 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6558` | `?` | `0x116928` |
| `bl` at 0x58754 | `(unknown callee) 0x59b44` | `[0x11a434]` | `[0x11a42c]` | `?` | `?` |
| `bl` at 0x58a00 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe662c` | `?` | `?` |
| `bl` at 0x58a70 | `(unknown callee) 0x18ae8` | `?` | `?` | `?` | `?` |
| `bl` at 0x58aa4 | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `[0x11a438]` |
| `bl` at 0x58af4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe664c` | `?` | `0x116928` |
| `bl` at 0x58b8c | `(unknown callee) 0x56b54` | `[sp-128]` | `0x11a3a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5794c | `[0x11a0f4] == 0x0` | `[0x11a0f4] != 0x0` |
| `beq` at 0x57960 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5798c | `[0x11a0f0] == 0x0` | `[0x11a0f0] != 0x0` |
| `bgt` at 0x579b4 | `[sp-152] > 0x4 (signed, 32-bit)` | `[sp-152] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x579c8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x57a1c | `[0x11a330] == 0x0` | `[0x11a330] != 0x0` |
| `bgt` at 0x57a34 | `[0x11a42c] > [0x11a330] (signed, 32-bit)` | `[0x11a42c] <= [0x11a330] (signed, 32-bit)` |
| `beq` at 0x57a48 | `[0x11a32c] == 0x0` | `[0x11a32c] != 0x0` |
| `beq` at 0x57b0c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x57b3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x57b74 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x57b88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x57bb4 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x57c34 | `[sp-132] < ? (signed, 32-bit)` | `[sp-132] >= ? (signed, 32-bit)` |
| `beq` at 0x57cb8 | `ret(0x59b44) == 0x0` | `ret(0x59b44) != 0x0` |
| `bhs` at 0x57cd4 | `[[sp-152]+20]+1 >= [ret(0x59b44)+20] (unsigned, 32-bit)` | `[[sp-152]+20]+1 < [ret(0x59b44)+20] (unsigned, 32-bit)` |
| `beq` at 0x57d18 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x57da0 | `[[sp-152]+16] == ?` | `[[sp-152]+16] != ?` |
| `beq` at 0x57e0c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x57eb8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x57ef0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x57fb0 | `[0x11a3f8] == 0x0` | `[0x11a3f8] != 0x0` |
| `beq` at 0x58004 | `ret(0x59db8) == 0x0` | `ret(0x59db8) != 0x0` |
| `bne` at 0x58014 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x58078 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x580a4 | `[sp-124] != [0x11a42c]` | `[sp-124] == [0x11a42c]` |
| `bne` at 0x580e0 | `ret(0x18a58) != 0x0` | `ret(0x18a58) == 0x0` |
| `beq` at 0x58108 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58128 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5813c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58168 | `[sp-124] == [0x11a42c]` | `[sp-124] != [0x11a42c]` |
| `beq` at 0x581b0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x581e0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x58250 | `ret(0x59b44) != 0x0` | `ret(0x59b44) == 0x0` |
| `beq` at 0x582d0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x5830c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x58380 | `ret(0x59b44) != 0x0` | `ret(0x59b44) == 0x0` |
| `beq` at 0x58404 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x58444 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x584a0 | `ret(0x59b44) == 0x0` | `ret(0x59b44) != 0x0` |
| `beq` at 0x584b8 | `[[sp-156]+20] == [ret(0x59b44)+20]` | `[[sp-156]+20] != [ret(0x59b44)+20]` |
| `beq` at 0x5851c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5853c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x58554 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x585b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x585d8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x585f0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x58654 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x586ac | `? == 0x0` | `? != 0x0` |
| `blt` at 0x586ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58770 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x58798 | `[sp-160] != 0x0` | `[sp-160] == 0x0` |
| `bne` at 0x587b0 | `[0x11a3f8] != 0x0` | `[0x11a3f8] == 0x0` |
| `bne` at 0x587e8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x58814 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5884c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5886c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5887c | `[0x11a3f8] != 0x0` | `[0x11a3f8] == 0x0` |
| `bne` at 0x588b4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x58954 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5898c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x589a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58ab4 | `[0x11a410] == 0x0` | `[0x11a410] != 0x0` |
| `beq` at 0x58ac4 | `[sp-140] == 0x0` | `[sp-140] != 0x0` |
| `beq` at 0x58ad0 | `[sp-140] == 0x2` | `[sp-140] != 0x2` |
| `beq` at 0x58ae4 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x58b10 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x58b48 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x58b80 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58ba0 | `r0` | `?` |

## `L1CfgQuery`

- `L1CfgQuery`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ca74 to 0x5e520.
  Established. The initial branch is taken when entry `r1 == 0`, and falls through otherwise (`beq`
  at 0x5ca94). Word stores of `ret(0x18920)` include the target `[sp-24] +0x0` (`str` at 0x5cae0,
  `str` at 0x5d1a8, `str` at 0x5d534). The computed transfer is taken when unsigned `[[sp-60]+56] <=
  0x5`, and falls through when greater (`ldrls` at 0x5cf04); its taken targets remain unresolved. A
  separate branch falls through when entry `r1 == 0x1010`, and is taken otherwise (`bne` at
  0x5d7d0). That fall-through writes words `ret(0x189c8)` at `0x11a0e4` (`str` at 0x5d7e0) and zero
  at `0x11a0ec` (`str` at 0x5d7ec). Query formats, time units and field roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ca84 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5ca88 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x5ca8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ca98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5caac | load | 4 | `abs 0x5da14` | `-` |
| `ldr` at 0x5cab4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5cab8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cabc | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x5cae0 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5cae4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5cae8 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x5caf0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5caf4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5caf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cafc | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5cb0c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cb10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cb24 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cb2c | load | 4 | `(unknown)` | `-` |
| `str` at 0x5cb30 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x5cb34 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cb38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cb4c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cb54 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5cb58 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x5cb5c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cb60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cb74 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cb7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cb88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5cb9c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cba0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cbb4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cbbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cbc8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cbcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cbe0 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cbe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cbec | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5cbf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cbfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cc00 | load | 4 | `abs 0x5d92c` | `-` |
| `ldr` at 0x5cc08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cc18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5cc20 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5cc24 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cc28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cc3c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cc44 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cc48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cc5c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cc64 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cc68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cc7c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cc84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cc88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cc8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cc90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cc98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cca8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5ccb0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5ccb4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5ccb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cccc | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5ccd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cce0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cce4 | load | 4 | `abs 0x5d930` | `-` |
| `ldr` at 0x5ccf0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5ccf4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5ccf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cd0c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cd14 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5cd18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cd24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cd28 | load | 4 | `abs 0x5d934` | `-` |
| `ldr` at 0x5cd34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cd44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5cd4c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5cd50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5cd60 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cd64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cd78 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cd80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cd84 | load | 4 | `abs 0x5d938` | `-` |
| `ldr` at 0x5cd88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cd90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cda0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5cda8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5cdac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5cdbc | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cdc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cdd4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cddc | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5cde0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cdf4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5cdfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ce00 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5ce0c | store | 4 | `sp -0x3c` | `ret(0x5e5a8)` |
| `ldr` at 0x5ce10 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5ce14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5ce28 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5ce30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ce3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ce40 | load | 4 | `abs 0x5d93c` | `-` |
| `ldr` at 0x5ce4c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5ce50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5ce64 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5ce6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ce78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ce7c | load | 4 | `abs 0x5d940` | `-` |
| `ldr` at 0x5ce88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ce8c | load | 4 | `abs 0x5d944` | `-` |
| `ldr` at 0x5ce94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cea4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5ceac | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5ceb0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5cebc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cec0 | load | 4 | `abs 0x5d948` | `-` |
| `ldr` at 0x5cecc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5ced0 | load | 4 | `[sp-60] +0x34` | `-` |
| `ldr` at 0x5cedc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cee0 | load | 4 | `abs 0x5d94c` | `-` |
| `ldr` at 0x5ceec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cef0 | load | 4 | `abs 0x5d950` | `-` |
| `ldr` at 0x5cef8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5cefc | load | 4 | `[sp-60] +0x38` | `-` |
| `ldrls` at 0x5cf04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5cf84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cf88 | load | 4 | `abs 0x5d96c` | `-` |
| `ldr` at 0x5cf90 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5cf94 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x5cfa4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5cfa8 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x5cfb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cfbc | load | 4 | `abs 0x5d970` | `-` |
| `ldr` at 0x5cfc8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5cfcc | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x5cfdc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5cfe0 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x5cff0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cff4 | load | 4 | `abs 0x5d974` | `-` |
| `ldr` at 0x5d000 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5d004 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x5d014 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d018 | load | 4 | `abs 0x5d978` | `-` |
| `ldr` at 0x5d024 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5d028 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x5d038 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d03c | load | 4 | `abs 0x5d97c` | `-` |
| `ldr` at 0x5d048 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d04c | load | 4 | `abs 0x5d980` | `-` |
| `ldr` at 0x5d054 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d058 | load | 4 | `abs 0x5da00` | `-` |
| `ldr` at 0x5d060 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d070 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d078 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d07c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d084 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5d08c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d09c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d0a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d0a4 | load | 4 | `abs 0x5d984` | `-` |
| `ldr` at 0x5d0a8 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5d0b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d0c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d0c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d0cc | load | 4 | `abs 0x5d988` | `-` |
| `ldr` at 0x5d0d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d0d4 | load | 4 | `abs 0x5d98c` | `-` |
| `ldr` at 0x5d0d8 | load | 4 | `abs 0x11a0d0` | `-` |
| `ldr` at 0x5d0e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d0f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d0f8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d0fc | load | 4 | `abs 0x5d990` | `-` |
| `ldr` at 0x5d100 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d104 | load | 4 | `abs 0x5d994` | `-` |
| `ldr` at 0x5d108 | load | 4 | `abs 0x11a0d4` | `-` |
| `ldr` at 0x5d110 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d120 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d128 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d12c | load | 4 | `abs 0x5d998` | `-` |
| `ldr` at 0x5d130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d134 | load | 4 | `abs 0x5d99c` | `-` |
| `ldr` at 0x5d138 | load | 4 | `abs 0x11a0d8` | `-` |
| `ldr` at 0x5d140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d150 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d158 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d174 | load | 4 | `abs 0x5da14` | `-` |
| `ldr` at 0x5d17c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d180 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d184 | load | 4 | `abs 0x11a424` | `-` |
| `str` at 0x5d1a8 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5d1ac | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5d1b0 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x5d1b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d1b8 | load | 4 | `abs 0x5d9a0` | `-` |
| `ldr` at 0x5d1c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d1d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d1d8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d1dc | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d1e0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d1e4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d1e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d1ec | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x5d1f0 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5d1f4 | load | 4 | `abs 0x11a3ac` | `-` |
| `ldr` at 0x5d1fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d200 | load | 4 | `abs 0x5da00` | `-` |
| `ldr` at 0x5d208 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d218 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d220 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `str` at 0x5d228 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x5d22c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d230 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d234 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x5d244 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d248 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5d24c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d25c | load | 4 | `abs 0x5d9a4` | `-` |
| `ldr` at 0x5d260 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d268 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d280 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d284 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d288 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d2a4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d2ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d2b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d2bc | load | 4 | `abs 0x5d9a8` | `-` |
| `ldr` at 0x5d2c8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d2cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d2e8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d2f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d2fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d300 | load | 4 | `abs 0x5d9ac` | `-` |
| `ldr` at 0x5d30c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d310 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d32c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d334 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d338 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d354 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d35c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d360 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d37c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d384 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d38c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d390 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d398 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d39c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d3b8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d3c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d3cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d3d0 | load | 4 | `abs 0x5d9b0` | `-` |
| `ldr` at 0x5d3dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d3e0 | load | 4 | `abs 0x5d9b4` | `-` |
| `ldr` at 0x5d3e8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d3ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d408 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d410 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d414 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d430 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d438 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d43c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d448 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d454 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d458 | load | 4 | `abs 0x5d9b4` | `-` |
| `ldr` at 0x5d460 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d470 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d478 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d47c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d480 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d49c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5d4a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d4a8 | load | 4 | `abs 0x5d9b8` | `-` |
| `ldr` at 0x5d4ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d4b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d4b8 | load | 4 | `abs 0x5da00` | `-` |
| `ldr` at 0x5d4c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d4d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d4d8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d4dc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5d4e4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5d4f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d504 | load | 4 | `abs 0x5da14` | `-` |
| `ldr` at 0x5d50c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d510 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d514 | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x5d534 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5d538 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5d53c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x5d540 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d544 | load | 4 | `abs 0x5d9a0` | `-` |
| `ldr` at 0x5d54c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d55c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d564 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d568 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d56c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d570 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d574 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d578 | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x5d57c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5d580 | load | 4 | `abs 0x11a3ac` | `-` |
| `ldr` at 0x5d588 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d58c | load | 4 | `abs 0x5da00` | `-` |
| `ldr` at 0x5d594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d5a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d5ac | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `str` at 0x5d5b4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x5d5b8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d5bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d5c0 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x5d5d0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d5d4 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5d5d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d5e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d5e8 | load | 4 | `abs 0x5da18` | `-` |
| `ldr` at 0x5d5ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d5f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d604 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d60c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `str` at 0x5d614 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x5d618 | load | 4 | `abs 0x5d9bc` | `-` |
| `ldr` at 0x5d61c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5d620 | load | 4 | `abs 0x1155f0` | `-` |
| `str` at 0x5d634 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5d638 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d63c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d640 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5d650 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d654 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d668 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5d670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d674 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5d684 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d688 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d69c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5d6a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d6a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5d6b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5d6c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d6c4 | load | 4 | `abs 0x5d9c0` | `-` |
| `ldr` at 0x5d6cc | load | 4 | `abs 0x5d9c4` | `-` |
| `ldr` at 0x5d6d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d6e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d6ec | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d6f0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d6f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d708 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5d710 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d714 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d728 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5d730 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d734 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d748 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5d750 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d754 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d75c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5d764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d768 | load | 4 | `abs 0x5da00` | `-` |
| `ldr` at 0x5d770 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d780 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d788 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d78c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d794 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5d79c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5d7a4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5d7ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5d7b4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5d7c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d7c8 | load | 4 | `abs 0x5d9c8` | `-` |
| `ldr` at 0x5d7d4 | load | 4 | `abs 0x5d9dc` | `-` |
| `str` at 0x5d7e0 | store | 4 | `abs 0x11a0e4` | `ret(0x189c8)` |
| `ldr` at 0x5d7e4 | load | 4 | `abs 0x5d9e4` | `-` |
| `str` at 0x5d7ec | store | 4 | `abs 0x11a0ec` | `0x0` |
| `ldr` at 0x5d7f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d800 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5d810 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x5d814 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5d818 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d828 | load | 4 | `abs 0x5d9d8` | `-` |
| `ldr` at 0x5d82c | load | 4 | `abs 0x11a0dc` | `-` |
| `str` at 0x5d834 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x5d838 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5d83c | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5d854 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5d858 | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5d88c | load | 4 | `abs 0x5d9ec` | `-` |
| `ldr` at 0x5d8a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5d8a8 | load | 4 | `abs 0x5d9ec` | `-` |
| `str` at 0x5d8d4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x5d8d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d8dc | load | 4 | `abs 0x5d9cc` | `-` |
| `ldr` at 0x5d8ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d8fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d904 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5d908 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d90c | load | 4 | `abs 0x11a410` | `-` |
| `ldr` at 0x5d918 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d91c | load | 4 | `abs 0x5d9d0` | `-` |
| `ldr` at 0x5da48 | load | 4 | `abs 0x5d9d4` | `-` |
| `ldr` at 0x5da4c | load | 4 | `abs 0x5d9d8` | `-` |
| `ldr` at 0x5da50 | load | 4 | `abs 0x11a0e0` | `-` |
| `ldr` at 0x5da54 | load | 4 | `abs 0x11a0dc` | `-` |
| `str` at 0x5da5c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x5da6c | load | 4 | `abs 0x5d9dc` | `-` |
| `ldr` at 0x5da70 | load | 4 | `abs 0x11a0e4` | `-` |
| `str` at 0x5da78 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x5da7c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5da80 | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5da98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5da9c | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5dad0 | load | 4 | `abs 0x5d9ec` | `-` |
| `ldr` at 0x5dae8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5daec | load | 4 | `abs 0x5d9ec` | `-` |
| `str` at 0x5db18 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x5db1c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x5db20 | load | 4 | `abs 0x5d9e8` | `-` |
| `str` at 0x5db38 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x5db3c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x5db40 | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5db74 | load | 4 | `abs 0x5d9ec` | `-` |
| `str` at 0x5db8c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x5db90 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x5db94 | load | 4 | `abs 0x5d9ec` | `-` |
| `str` at 0x5dbc0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x5dbc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dbc8 | load | 4 | `abs 0x5d9e0` | `-` |
| `ldr` at 0x5dbd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dbe8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5dbf0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5dc08 | load | 4 | `abs 0x5d9e4` | `-` |
| `ldr` at 0x5dc0c | load | 4 | `abs 0x11a0ec` | `-` |
| `ldr` at 0x5dc24 | load | 4 | `abs 0x5d9e4` | `-` |
| `ldr` at 0x5dc28 | load | 4 | `abs 0x11a0ec` | `-` |
| `str` at 0x5dc30 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x5dc34 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5dc38 | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5dc50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5dc54 | load | 4 | `abs 0x5d9e8` | `-` |
| `ldr` at 0x5dc88 | load | 4 | `abs 0x5d9ec` | `-` |
| `ldr` at 0x5dca0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5dca4 | load | 4 | `abs 0x5d9ec` | `-` |
| `str` at 0x5dcd0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x5dcd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dcd8 | load | 4 | `abs 0x5d9f0` | `-` |
| `ldr` at 0x5dcec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dcf0 | load | 4 | `abs 0x5d9f4` | `-` |
| `ldr` at 0x5dcfc | load | 4 | `abs 0x5d9f8` | `-` |
| `ldr` at 0x5dd00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dd04 | load | 4 | `abs 0x5d9fc` | `-` |
| `ldr` at 0x5dd08 | load | 4 | `abs 0x1155f8` | `-` |
| `ldr` at 0x5dd10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dd14 | load | 4 | `abs 0x5da00` | `-` |
| `ldr` at 0x5dd20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dd30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5dd40 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x5dd44 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5dd48 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5dd4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dd50 | load | 4 | `abs 0x5da04` | `-` |
| `ldr` at 0x5dd54 | load | 4 | `abs 0x5da08` | `-` |
| `ldr` at 0x5dd60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dd70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dd80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5dd8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dd9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5dda0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5dda4 | load | 4 | `abs 0x11a438` | `-` |
| `str` at 0x5ddc4 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5ddc8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5ddcc | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x5ddd4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5ddd8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5dddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5dde0 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5ddf0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5ddf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5de08 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5de10 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5de14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5de28 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5de30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5de34 | load | 4 | `abs 0x5da0c` | `-` |
| `ldr` at 0x5de38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5de3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5de44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5de54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5de5c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5de60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5de68 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x5de70 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5de74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5de78 | load | 4 | `abs 0x5da10` | `-` |
| `ldr` at 0x5de7c | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5de84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5de94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5de9c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5dea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dec4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dee4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5def8 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x5df00 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x5df04 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5df08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5df0c | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x5df1c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5df20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5df3c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5df44 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5df48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5df50 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x5df54 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5df5c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5df64 | load | 4 | `abs 0x5da14` | `-` |
| `ldr` at 0x5df6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5df70 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x5df90 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5df94 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5df98 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x5dfa0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x5dfa4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5dfa8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5dfac | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x5dfbc | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5dfc0 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5dfc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5dfd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dfd4 | load | 4 | `abs 0x5da18` | `-` |
| `ldr` at 0x5dfd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5dfe0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5dff0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5dff8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `str` at 0x5e000 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5e004 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e008 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e024 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e02c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e030 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e044 | load | 4 | `abs 0x5da1c` | `-` |
| `ldr` at 0x5e04c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e05c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e064 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5e068 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e06c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e088 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e090 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e0a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e0ac | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e0b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e0cc | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e0d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e0e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e0f0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e0f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e110 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e12c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e134 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e138 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e13c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e140 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e158 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e160 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5e164 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e168 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e184 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e18c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e1a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e1a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e1b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e1b8 | load | 4 | `abs 0x5da20` | `-` |
| `ldr` at 0x5e1c4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e1c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e1e4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e1ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e200 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e208 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e214 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e218 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e234 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e23c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e250 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e25c | load | 4 | `abs 0x5da24` | `-` |
| `ldr` at 0x5e260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e26c | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e270 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e28c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e294 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e298 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e2b4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e2bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e2d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e2d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e2dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e2e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e2ec | load | 4 | `abs 0x5da28` | `-` |
| `ldr` at 0x5e2f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e308 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e310 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5e314 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e318 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e334 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e33c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e364 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e368 | load | 4 | `abs 0x5da2c` | `-` |
| `ldr` at 0x5e374 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e378 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e394 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e39c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e3b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e3b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e3c4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e3c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5e3e4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5e3ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5e400 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e408 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e40c | load | 4 | `abs 0x5da30` | `-` |
| `ldr` at 0x5e410 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e41c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e420 | load | 4 | `abs 0x5da34` | `-` |
| `ldr` at 0x5e428 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e438 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e440 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x18a1c))` |
| `ldr` at 0x5e444 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5e44c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x5e454 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5e45c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5e46c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e47c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e48c | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5e490 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5e494 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x5e498 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e49c | load | 4 | `abs 0x11a414` | `-` |
| `ldr` at 0x5e4a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e4ac | load | 4 | `abs 0x5da38` | `-` |
| `ldr` at 0x5e4b8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e4bc | load | 4 | `abs 0x11a414` | `-` |
| `ldr` at 0x5e4c8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e4cc | load | 4 | `abs 0x11a418` | `-` |
| `ldr` at 0x5e4e8 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e4ec | load | 4 | `abs 0x11a41c` | `-` |
| `ldr` at 0x5e4f4 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5e4f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e4fc | load | 4 | `abs 0x5da40` | `-` |
| `ldr` at 0x5e504 | load | 4 | `abs 0x11a420` | `-` |
| `ldr` at 0x5e510 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e514 | load | 4 | `abs 0x5da44` | `-` |
| `ldmdb` at 0x5e51c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5e51c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e51c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e51c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cab0 | `(unknown callee) 0x56518` | `0xe6c70` | `r1` | `r2` | `?` |
| `bl` at 0x5cad8 | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a42c]` | `?` |
| `bl` at 0x5cbf4 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6a1c` | `?` | `?` |
| `bl` at 0x5cc04 | `(unknown callee) 0x18cd4` | `[sp-32]` | `0xe6a24` | `?` | `?` |
| `bl` at 0x5cc0c | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5cc94 | `(unknown callee) 0x54dd8` | `([sp-32] + ret(0x18a1c))` | `?` | `?` | `?` |
| `bl` at 0x5cc9c | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5cce8 | `(unknown callee) 0x18cd4` | `([sp-32] + ret(0x18a1c))` | `0xe6a2c` | `?` | `?` |
| `bl` at 0x5cd1c | `(unknown callee) 0x5c834` | `[0x11a3b0]` | `?` | `?` | `?` |
| `bl` at 0x5cd30 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6a34` | `ret(0x5c834)` | `ret(0x5c834)` |
| `bl` at 0x5cd38 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5cd8c | `(unknown callee) 0x18c74` | `([sp-32] + ret(0x18a1c))` | `0xe6a38` | `?` | `?` |
| `bl` at 0x5cd94 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5ce04 | `(unknown callee) 0x5e5a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x5ce44 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6a3c` | `?` | `?` |
| `bl` at 0x5ce80 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6a44` | `?` | `?` |
| `bl` at 0x5ce90 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6a4c` | `?` | `?` |
| `bl` at 0x5ce98 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5cec4 | `(unknown callee) 0x18ab8` | `([sp-32] + ret(0x18a1c))` | `0xe6a54` | `ret(0x18a1c)` | `[sp-60]` |
| `bl` at 0x5cee4 | `(unknown callee) 0x18ab8` | `([sp-32] + ret(0x18a1c))` | `0xe6a60` | `ret(0x18a1c)` | `[[sp-60]+52]` |
| `bl` at 0x5cef4 | `(unknown callee) 0x18ab8` | `([sp-32] + ret(0x18a1c))` | `0xe6a68` | `ret(0x18a1c)` | `[[sp-60]+52]` |
| `bl` at 0x5cf8c | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6aa0` | `?` | `[[sp-60]+56]` |
| `bl` at 0x5cfc0 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6aa8` | `?` | `?` |
| `bl` at 0x5cff8 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6ab4` | `?` | `?` |
| `bl` at 0x5d01c | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6ac0` | `?` | `?` |
| `bl` at 0x5d040 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6acc` | `?` | `?` |
| `bl` at 0x5d050 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6ad8` | `?` | `?` |
| `bl` at 0x5d05c | `(unknown callee) 0x18ab8` | `?` | `0xe6338` | `?` | `?` |
| `bl` at 0x5d064 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d0ac | `(unknown callee) 0x18c74` | `?` | `0xe6ae4` | `[0x11a42c]` | `0x11a3a0` |
| `bl` at 0x5d0b4 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d0dc | `(unknown callee) 0x18c74` | `([sp-32] + ret(0x18a1c))` | `0xe6af0` | `[0x11a0d0]` | `0x11a0d0` |
| `bl` at 0x5d0e4 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d10c | `(unknown callee) 0x18c74` | `([sp-32] + ret(0x18a1c))` | `0xe6b08` | `[0x11a0d4]` | `0x11a0d4` |
| `bl` at 0x5d114 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d13c | `(unknown callee) 0x18c74` | `([sp-32] + ret(0x18a1c))` | `0xe6b20` | `[0x11a0d8]` | `0x11a0d8` |
| `bl` at 0x5d144 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d15c | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `?` |
| `bl` at 0x5d178 | `(unknown callee) 0x56518` | `0xe6c70` | `r1` | `r2` | `?` |
| `bl` at 0x5d1a0 | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a424]` | `?` |
| `bl` at 0x5d1bc | `(unknown callee) 0x18c74` | `[[sp-24]]` | `0xe6b38` | `?` | `[[sp-24]]` |
| `bl` at 0x5d1c4 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d1f8 | `(unknown callee) 0x54dd8` | `([sp-32] + ret(0x18a1c))` | `[0x11a3a8]` | `[0x11a3b0]` | `[0x11a3ac]` |
| `bl` at 0x5d204 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6338` | `?` | `?` |
| `bl` at 0x5d20c | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d250 | `(unknown callee) 0x5c834` | `[0x11a3b0]` | `?` | `?` | `0x11a3a0` |
| `bl` at 0x5d264 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6b44` | `[sp-48]` | `ret(0x5c834)` |
| `bl` at 0x5d26c | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d2c0 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6b50` | `?` | `?` |
| `bl` at 0x5d304 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6b60` | `?` | `?` |
| `bl` at 0x5d394 | `(unknown callee) 0x54dd8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d3d4 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6b7c` | `?` | `?` |
| `bl` at 0x5d3e4 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6b84` | `?` | `?` |
| `bl` at 0x5d440 | `(unknown callee) 0x5c834` | `?` | `?` | `?` | `?` |
| `bl` at 0x5d450 | `(unknown callee) 0x18ab8` | `[sp-32]` | `ret(0x5c834)` | `?` | `ret(0x5c834)` |
| `bl` at 0x5d45c | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6b84` | `?` | `?` |
| `bl` at 0x5d464 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d4b0 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6b88` | `?` | `?` |
| `bl` at 0x5d4bc | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6338` | `?` | `?` |
| `bl` at 0x5d4c4 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d4ec | `(unknown callee) 0x5659c` | `ret(0x18a1c)` | `?` | `?` | `[0x11a424]` |
| `bl` at 0x5d508 | `(unknown callee) 0x56518` | `0xe6c70` | `r1` | `r2` | `?` |
| `bl` at 0x5d52c | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a42c]` | `?` |
| `bl` at 0x5d548 | `(unknown callee) 0x18c74` | `[[sp-24]]` | `0xe6b38` | `?` | `[[sp-24]]` |
| `bl` at 0x5d550 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d584 | `(unknown callee) 0x54dd8` | `([sp-32] + ret(0x18a1c))` | `[0x11a3a8]` | `[0x11a3b0]` | `[0x11a3ac]` |
| `bl` at 0x5d590 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6338` | `?` | `?` |
| `bl` at 0x5d598 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d5dc | `(unknown callee) 0x5c834` | `[0x11a3b0]` | `?` | `?` | `0x11a3a0` |
| `bl` at 0x5d5f0 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6c7c` | `[sp-48]` | `ret(0x5c834)` |
| `bl` at 0x5d5f8 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d6d0 | `(unknown callee) 0x18c74` | `([sp-32] + ret(0x18a1c))` | `0xe6b94` | `?` | `0xe6b98` |
| `bl` at 0x5d6d8 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d760 | `(unknown callee) 0x54dd8` | `([sp-32] + ret(0x18a1c))` | `?` | `?` | `?` |
| `bl` at 0x5d76c | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6338` | `?` | `?` |
| `bl` at 0x5d774 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d7bc | `(unknown callee) 0x5659c` | `ret(0x18a1c)` | `?` | `?` | `[0x11a424]` |
| `bl` at 0x5d7dc | `(unknown callee) 0x189c8` | `0x0` | `r1` | `r1` | `0x1010` |
| `bl` at 0x5d808 | `(unknown callee) 0x18920` | `0xc8` | `?` | `?` | `?` |
| `bl` at 0x5d820 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5d8e8 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6b9c` | `?` | `?` |
| `bl` at 0x5d8f0 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5d920 | `(unknown callee) 0x18cd4` | `([sp-32] + ret(0x18a1c))` | `0xe6bb4` | `ret(0x18a1c)` | `[0x11a410]` |
| `bl` at 0x5da64 | `(unknown callee) 0x189c8` | `0x0` | `[0x11a0e0]` | `0x11a0dc` | `?` |
| `bl` at 0x5dbd4 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6bcc` | `?` | `?` |
| `bl` at 0x5dbdc | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5dbf8 | `(unknown callee) 0x1c79c` | `0x9a` | `?` | `ret(0x18a1c)` | `([sp-32] + ret(0x18a1c))` |
| `bl` at 0x5dc1c | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[0x11a0ec]` |
| `bl` at 0x5dce4 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6bf8` | `?` | `?` |
| `bl` at 0x5dcf4 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6c10` | `?` | `[0x11a0ec]` |
| `bl` at 0x5dd0c | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6c24` | `[0x1155f8]` | `0x1155f8` |
| `bl` at 0x5dd18 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xe6338` | `?` | `?` |
| `bl` at 0x5dd38 | `(unknown callee) 0x18920` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x5dd58 | `(unknown callee) 0x18c74` | `?` | `0xe6c40` | `0x281ef8` | `?` |
| `bl` at 0x5dd6c | `(unknown callee) 0x57704` | `?` | `?` | `?` | `?` |
| `bl` at 0x5dd84 | `(unknown callee) 0x57788` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5ddbc | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a438]` | `?` |
| `bl` at 0x5de40 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6c58` | `?` | `?` |
| `bl` at 0x5de48 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5de80 | `(unknown callee) 0x18c74` | `?` | `0xe6c64` | `[0x11a438]` | `0x11a3a0` |
| `bl` at 0x5de88 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5debc | `(unknown callee) 0x1abd0` | `0x9a` | `0x0` | `?` | `?` |
| `bl` at 0x5dedc | `(unknown callee) 0x1abd0` | `0x9a` | `0x1` | `?` | `?` |
| `bl` at 0x5df68 | `(unknown callee) 0x56518` | `0xe6c70` | `?` | `?` | `[0x11a424]` |
| `bl` at 0x5df88 | `(unknown callee) 0x18920` | `?` | `?` | `[sp-60]` | `?` |
| `bl` at 0x5dfc8 | `(unknown callee) 0x5c834` | `[0x11a3b0]` | `?` | `?` | `0x11a3a0` |
| `bl` at 0x5dfdc | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6c7c` | `[sp-48]` | `ret(0x5c834)` |
| `bl` at 0x5dfe4 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5e048 | `(unknown callee) 0x18cd4` | `[sp-32]` | `0xe6c88` | `[sp-36]` | `?` |
| `bl` at 0x5e050 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5e144 | `(unknown callee) 0x54dd8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5e14c | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5e1bc | `(unknown callee) 0x18cd4` | `[sp-32]` | `0xe6c90` | `?` | `?` |
| `bl` at 0x5e264 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6c98` | `?` | `?` |
| `bl` at 0x5e2e0 | `(unknown callee) 0x5c834` | `?` | `?` | `?` | `?` |
| `bl` at 0x5e2f4 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6ca4` | `ret(0x5c834)` | `ret(0x5c834)` |
| `bl` at 0x5e2fc | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5e36c | `(unknown callee) 0x18cd4` | `[sp-32]` | `0xe6cb0` | `?` | `?` |
| `bl` at 0x5e414 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6cb8` | `?` | `?` |
| `bl` at 0x5e424 | `(unknown callee) 0x18cd4` | `[sp-32]` | `0xe6cc0` | `?` | `?` |
| `bl` at 0x5e42c | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x5e464 | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `[0x11a424]` |
| `bl` at 0x5e484 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x5e4b0 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6ccc` | `?` | `[0x11a414]` |
| `bl` at 0x5e508 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6cd8` | `?` | `[0x11a420]` |
| `bl` at 0x5e518 | `(unknown callee) 0x18c74` | `[sp-32]` | `0xe6cec` | `?` | `[0x11a414]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ca94 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x5caa4 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x5cb04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x5cb84 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x5cb94 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5cbc4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5ccdc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5cd5c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5cdb8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5ce38 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5ce74 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5ceb8 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `beq` at 0x5ced8 | `[[sp-60]+52] == 0x0` | `[[sp-60]+52] != 0x0` |
| `ldrls` at 0x5cf04 | `[[sp-60]+56] <= 0x5 (unsigned, 32-bit)` | `[[sp-60]+56] > 0x5 (unsigned, 32-bit)` |
| `beq` at 0x5cfa0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5cfb4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5cfd8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5cfec | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5d010 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d034 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d098 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d170 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x5d23c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5d2b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x5d2f8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x5d3c8 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x5d500 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x5d5c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5d628 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5d648 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5d67c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5d6b0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5d7d0 | `r1 != 0x1010` | `r1 == 0x1010` |
| `beq` at 0x5d7fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d914 | `[0x11a410] == 0x0` | `[0x11a410] != 0x0` |
| `beq` at 0x5dc04 | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `beq` at 0x5dc14 | `[0x11a0ec] == 0x0` | `[0x11a0ec] != 0x0` |
| `beq` at 0x5dd2c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5dd68 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5dd7c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5dd98 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x5dde8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5deb0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5ded0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5def0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x5df14 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5dfb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5e038 | `[sp-36] < ? (signed, 32-bit)` | `[sp-36] >= ? (signed, 32-bit)` |
| `bne` at 0x5e1b0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5e210 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5e360 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5e3c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5e478 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5e4a4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5e4c4 | `[0x11a414] == 0x0` | `[0x11a414] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e51c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5cf04).

## `L1CfgGetIndexBrick`

- `L1CfgGetIndexBrick`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e6f0 to 0x5e764.
  Established. The first comparison takes its branch when entry `r0` is signed negative (`blt` at
  0x5e70c). If it is nonnegative, the next comparison takes its branch when `r0 >= [0x11a42c]`
  signed, and falls through when less (`bge` at 0x5e720). The common taken path writes a zero word
  at `sp -0x18` (`str` at 0x5e72c). The other path loads a word at `0x11a434` (`ldr` at 0x5e74c),
  but the resulting arithmetic value remains unknown at the return (`ldmdb` at 0x5e75c). The
  identity of that value is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e700 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5e704 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e710 | load | 4 | `abs 0x5e760` | `-` |
| `ldr` at 0x5e714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e718 | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x5e72c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e734 | load | 4 | `abs 0x5e760` | `-` |
| `ldr` at 0x5e738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e74c | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x5e754 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5e758 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e75c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e75c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e75c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5e70c | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x5e720 | `r0 >= [0x11a42c] (signed, 32-bit)` | `r0 < [0x11a42c] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e75c | `r0` | `?` |

## `L1CfgGetBrick`

- `L1CfgGetBrick`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e764 to 0x5e7a8.
  Established. A word at `0x11a434` supplies the first call candidate (`ldr` at 0x5e784), while a
  word at `0x11a42c` supplies the second (`ldr` at 0x5e788). The call itself has candidates
  `([0x11a434], [0x11a42c], entry r0, entry r1)` (`bl` at 0x5e794); its result reaches the return
  (`ldmdb` at 0x5e7a0). No conditional branch is reported, and the object identified by the result
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e774 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5e778 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5e77c | load | 4 | `abs 0x5e7a4` | `-` |
| `ldr` at 0x5e780 | load | 4 | `abs 0x5e7a4` | `-` |
| `ldr` at 0x5e784 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5e788 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5e78c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e790 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e7a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e7a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e7a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e794 | `(unknown callee) 0x59b44` | `[0x11a434]` | `[0x11a42c]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e7a0 | `r0` | `ret(0x59b44)` |

## `L1CfgGetCfg`

- `L1CfgGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e7a8 to 0x5e958.
  Established. The computed transfer is taken when unsigned entry `r0 <= 0x28`, and falls through
  when greater (`ldrls` at 0x5e7c4). The fall-through stores the word `0x11a544` at `sp -0x28`
  (`str` at 0x5e910) before returning that value (`ldmdb` at 0x5e94c). No call is recorded on known
  edges. The unresolved taken targets prevent a complete configuration mapping.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e7b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5e7bc | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x5e7c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e90c | load | 4 | `abs 0x5e954` | `-` |
| `str` at 0x5e910 | store | 4 | `sp -0x28` | `0x11a544` |
| `ldr` at 0x5e948 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5e94c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e94c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e94c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x5e7c4 | `r0 <= 0x28 (unsigned, 32-bit)` | `r0 > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e94c | `r0` | `0x11a544` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5e7c4).

## `L1CfgGetPort`

- `L1CfgGetPort`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e9d0 to 0x5ea18.
  Established. The entry value is saved as a word at `sp -0x14` (`str` at 0x5e9e0). A later word
  load reads `0x11a428` (`ldr` at 0x5ea04). The arithmetic result remains unknown in `r0` at the
  return (`ldmdb` at 0x5ea10). No conditional branch or call record appears. The returned value
  cannot be assigned a physical port role from these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e9e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5e9e4 | load | 4 | `abs 0x5ea14` | `-` |
| `ldr` at 0x5e9e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ea04 | load | 4 | `abs 0x11a428` | `-` |
| `ldmdb` at 0x5ea10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ea10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ea10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ea10 | `r0` | `?` |

## `L1CfgBrickCount`

- `L1CfgBrickCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ea18 to 0x5ea38.
  Established. The word read at `0x11a42c` (`ldr` at 0x5ea28) reaches `r0` at the exit (`ldmdb` at
  0x5ea30). The body records no call or conditional branch. What that word counts remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5ea24 | load | 4 | `abs 0x5ea34` | `-` |
| `ldr` at 0x5ea28 | load | 4 | `abs 0x11a42c` | `-` |
| `ldmdb` at 0x5ea30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ea30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ea30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ea30 | `r0` | `[0x11a42c]` |

## `L1CfgTypeNonRoutered`

- `L1CfgTypeNonRoutered`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ea38 to 0x5eaa8.
  Established. Equality branches for entry `r0` values `0x44` (`beq` at 0x5ea5c), `0x40` (`beq` at
  0x5ea74) and `0x53` (`beq` at 0x5ea84) reach the zero-word store at `sp -0x18` (`str` at 0x5ea90).
  Other paths write one to that stack word (`str` at 0x5ea9c). The merged return value is unknown
  (`ldmdb` at 0x5eaa4). Routing and type semantics are not established by those comparisons.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ea48 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5ea4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5ea50 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5ea54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ea60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ea6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ea7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ea90 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x5ea9c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x5eaa0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5eaa4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5eaa4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5eaa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ea5c | `r0 == 0x44` | `r0 != 0x44` |
| `bgt` at 0x5ea68 | `r0 > 0x44 (signed, 32-bit)` | `r0 <= 0x44 (signed, 32-bit)` |
| `beq` at 0x5ea74 | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0x5ea84 | `r0 == 0x53` | `r0 != 0x53` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5eaa4 | `r0` | `?` |

## `L1CfgPortNonRoutered`

- `L1CfgPortNonRoutered`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5eaa8 to 0x5eb18.
  Established. The signed comparison takes its branch for `r0 >= [0x11a424]`, and falls through when
  less (`bge` at 0x5eacc). The taken path stores a zero word at `sp -0x18` (`str` at 0x5eb08). The
  other path includes a word load at `0x11a428` (`ldr` at 0x5eaf0) and a further load with unknown
  effective target (`ldr` at 0x5eaf8). The merged result remains unknown at the exit (`ldmdb` at
  0x5eb10); its relationship to routing is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5eab8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5eabc | load | 4 | `abs 0x5eb14` | `-` |
| `ldr` at 0x5eac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5eac4 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x5ead0 | load | 4 | `abs 0x5eb14` | `-` |
| `ldr` at 0x5ead4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5eaf0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5eaf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5eafc | store | 4 | `sp -0x18` | `?` |
| `str` at 0x5eb08 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5eb0c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5eb10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5eb10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5eb10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x5eacc | `r0 >= [0x11a424] (signed, 32-bit)` | `r0 < [0x11a424] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5eb10 | `r0` | `?` |

## `EnvPowerUpStart`

- `EnvPowerUpStart`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b9b8 to 0x6b9d0.
  Established. The sole call carries candidates `(0x2, entry r1, entry r2, entry r3)` (`bl` at
  0x6b9c8). The exit preserves `ret(0x69e48)` in `r0` (`ldmdb` at 0x6b9cc). There are no conditional
  branch records. The call alone does not establish a power-up transition.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x6b9cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b9cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b9cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b9c8 | `(unknown callee) 0x69e48` | `0x2` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b9cc | `r0` | `ret(0x69e48)` |
