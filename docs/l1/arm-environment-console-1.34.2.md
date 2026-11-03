# ARM environment and console state paths in 1.34.2

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
EnvQuery and EnvTestEvent each report one unresolved computed transfer.
The other thirteen routines report none.
Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/environment-console-facts.txt) retain all tool sections.

## `EnvEvent`

- `EnvEvent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x636f8 to 0x63db8.
  Established. Two word stores address absolute `0x11a768`, with unknown values on their respective
  paths (`str` at 0x638ec, `str` at 0x639c4). A separate load addresses `ret(0x5e7a8) +0x5c`, and
  its value is stored at `sp -0xf4` (`ldr` at 0x63ad0, `str` at 0x63ad4). The later branch tree
  includes equality tests of `[sp-248]` against `0x3300`, `0x2000` and `0x1200`, each selecting the
  taken edge for equality (`beq` at 0x63aec, `beq` at 0x63b04, `beq` at 0x63b1c). A final call to
  `0x5f0a4` has candidates `[sp-244]`, unknown, unknown and unknown (`bl` at 0x63d78); its result
  reaches the return (`ldmdb` at 0x63d7c). Event encodings and the absolute word’s physical effect
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63708 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x6370c | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x63710 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x63714 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x63718 | load | 4 | `abs 0x63d80` | `-` |
| `str` at 0x63728 | store | 4 | `sp -0xf0` | `ret(0x1fb00)` |
| `ldr` at 0x6372c | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x63738 | load | 4 | `abs 0x63d84` | `-` |
| `str` at 0x6373c | store | 4 | `sp -0x100` | `0xe7644` |
| `ldr` at 0x63740 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x63744 | store | 4 | `sp -0xfc` | `ret(0x1fb00)` |
| `ldr` at 0x6374c | load | 4 | `abs 0x63d88` | `-` |
| `ldr` at 0x6375c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x63768 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63778 | load | 4 | `abs 0x63d8c` | `-` |
| `ldr` at 0x637b0 | load | 4 | `abs 0x63d90` | `-` |
| `ldr` at 0x637bc | load | 4 | `abs 0x11af18` | `-` |
| `ldr` at 0x637d0 | load | 4 | `abs 0x63d94` | `-` |
| `ldr` at 0x637e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x637f0 | load | 4 | `abs 0x63d8c` | `-` |
| `ldr` at 0x637f8 | load | 4 | `abs 0x63d98` | `-` |
| `ldr` at 0x637fc | load | 4 | `abs 0x11a76c` | `-` |
| `ldr` at 0x63808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x63840 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6386c | store | 4 | `sp -0x100` | `0x1` |
| `ldr` at 0x63884 | load | 4 | `abs 0x63d9c` | `-` |
| `ldr` at 0x6388c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x63898 | store | 4 | `sp -0xf0` | `ret(0x9bf04)` |
| `ldr` at 0x6389c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x638b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x638c8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x638d8 | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x638dc | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x638e0 | load | 4 | `abs 0x11a768` | `-` |
| `ldr` at 0x638e4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x638ec | store | 4 | `abs 0x11a768` | `?` |
| `ldr` at 0x638f0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6390c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63918 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x63928 | load | 4 | `abs 0x63da4` | `-` |
| `ldr` at 0x6392c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63938 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x63948 | load | 4 | `abs 0x63da8` | `-` |
| `ldr` at 0x6394c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63958 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x63968 | load | 4 | `abs 0x63dac` | `-` |
| `ldr` at 0x6396c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63978 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x63988 | load | 4 | `abs 0x63db0` | `-` |
| `ldr` at 0x6398c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63998 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x639a8 | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x639ac | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x639b0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x639bc | load | 4 | `abs 0x11a768` | `-` |
| `str` at 0x639c4 | store | 4 | `abs 0x11a768` | `?` |
| `ldr` at 0x639cc | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x639d0 | load | 4 | `abs 0x11a768` | `-` |
| `ldr` at 0x639e4 | load | 4 | `abs 0x63da4` | `-` |
| `ldr` at 0x639f4 | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x639f8 | load | 4 | `abs 0x11a768` | `-` |
| `ldr` at 0x63a0c | load | 4 | `abs 0x63da8` | `-` |
| `ldr` at 0x63a1c | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x63a20 | load | 4 | `abs 0x11a768` | `-` |
| `ldr` at 0x63a34 | load | 4 | `abs 0x63dac` | `-` |
| `ldr` at 0x63a44 | load | 4 | `abs 0x63da0` | `-` |
| `ldr` at 0x63a48 | load | 4 | `abs 0x11a768` | `-` |
| `ldr` at 0x63a5c | load | 4 | `abs 0x63db0` | `-` |
| `ldr` at 0x63a78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63a84 | load | 4 | `abs 0x63d80` | `-` |
| `str` at 0x63a90 | store | 4 | `sp -0xf0` | `ret(0x1fe64)` |
| `ldr` at 0x63a94 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x63aa0 | load | 4 | `abs 0x63db4` | `-` |
| `str` at 0x63aa4 | store | 4 | `sp -0x100` | `0xe7684` |
| `ldr` at 0x63aa8 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x63aac | store | 4 | `sp -0xfc` | `ret(0x1fe64)` |
| `ldr` at 0x63ab4 | load | 4 | `abs 0x63d88` | `-` |
| `ldr` at 0x63ad0 | load | 4 | `ret(0x5e7a8) +0x5c` | `-` |
| `str` at 0x63ad4 | store | 4 | `sp -0xf4` | `[ret(0x5e7a8)+92]` |
| `ldr` at 0x63ad8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63ae0 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0x63ae4 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63af0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63afc | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b08 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b14 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b20 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b2c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b38 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b48 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b54 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b64 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b70 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b7c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b88 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63b98 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63ba4 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63bb4 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63bc0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63bcc | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63bd8 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63be4 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63bf0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c00 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c0c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c1c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c28 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c34 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c40 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c50 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c5c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c68 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c78 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x63c88 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63c90 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63c98 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63ca0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63ca8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63cb0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63cb8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63cc0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63cc8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63cd0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63cd8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63ce0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63ce8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63cf0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63cf8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d00 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d08 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d10 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d18 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d20 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d28 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d30 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d38 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d40 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d48 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d50 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d58 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d60 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d68 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x63d70 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x63d74 | load | 4 | `sp -0xf4` | `-` |
| `ldmdb` at 0x63d7c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x63d7c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x63d7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63d7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63d7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63720 | `(unknown callee) 0x1fb00` | `0x11a998` | `?` | `r2` | `r3` |
| `bl` at 0x63758 | `(unknown callee) 0x1eed0` | `0x9` | `0xe7664` | `0xb0` | `0x0` |
| `bl` at 0x6376c | `(unknown callee) 0x1d6d8` | `[sp-40]` | `?` | `?` | `[sp]` |
| `bl` at 0x6377c | `(unknown callee) 0x18cd4` | `sp-236` | `0xe7678` | `?` | `sp-236` |
| `bl` at 0x6378c | `(unknown callee) 0x18a1c` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x6379c | `(unknown callee) 0x1c79c` | `0x1` | `?` | `?` | `ret(0x18a1c)` |
| `bl` at 0x637a8 | `(unknown callee) 0x1c79c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x637c4 | `(unknown callee) 0x54dd8` | `(sp-236 + ret(0x18a1c))` | `ret(0x1c79c)` | `[0x11af18]` | `ret(0x1c79c)` |
| `bl` at 0x637d4 | `(unknown callee) 0x18ab8` | `sp-236` | `0xe767c` | `?` | `sp-236` |
| `bl` at 0x637e4 | `(unknown callee) 0x18ab8` | `sp-236` | `[sp-40]` | `?` | `sp-236` |
| `bl` at 0x637f4 | `(unknown callee) 0x18ab8` | `sp-236` | `0xe7678` | `?` | `sp-236` |
| `bl` at 0x6382c | `(unknown callee) 0x541d4` | `sp-84` | `sp-80` | `[sp-32]` | `sp-84` |
| `bl` at 0x63844 | `(unknown callee) 0x54230` | `sp-84` | `sp-80` | `[sp-36]` | `sp-84` |
| `bl` at 0x63850 | `(unknown callee) 0x18a1c` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x6387c | `(unknown callee) 0x54450` | `sp-84` | `sp-80` | `sp-236` | `ret(0x18a1c)` |
| `bl` at 0x63890 | `(unknown callee) 0x9bf04` | `0x11a798` | `0x0` | `[sp-84]` | `sp-80` |
| `bl` at 0x638b0 | `(unknown callee) 0x2c4a4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x638c0 | `(unknown callee) 0x2bfac` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x63904 | `(unknown callee) 0x2c4a4` | `0x1` | `0x11a768` | `[0x11a768]` | `?` |
| `bl` at 0x63914 | `(unknown callee) 0x2bfac` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x63930 | `(unknown callee) 0x18cd4` | `0x11af1c` | `[sp-40]` | `?` | `?` |
| `bl` at 0x63950 | `(unknown callee) 0x18cd4` | `0x11af6c` | `[sp-40]` | `?` | `?` |
| `bl` at 0x63970 | `(unknown callee) 0x18cd4` | `0x11afbc` | `[sp-40]` | `?` | `?` |
| `bl` at 0x63990 | `(unknown callee) 0x18cd4` | `0x11b00c` | `[sp-40]` | `?` | `?` |
| `bl` at 0x639c8 | `(unknown callee) 0x2c470` | `0x11a768` | `0x11a768` | `?` | `?` |
| `bl` at 0x639ec | `(unknown callee) 0x2bfac` | `0x32` | `0x11af1c` | `0x2` | `?` |
| `bl` at 0x63a14 | `(unknown callee) 0x2bfac` | `0x32` | `0x11af6c` | `0x2` | `?` |
| `bl` at 0x63a3c | `(unknown callee) 0x2bfac` | `0x32` | `0x11afbc` | `0x2` | `?` |
| `bl` at 0x63a64 | `(unknown callee) 0x2bfac` | `0x32` | `0x11b00c` | `0x2` | `?` |
| `bl` at 0x63a70 | `(unknown callee) 0x2c5e4` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x63a80 | `(unknown callee) 0x2bfac` | `0x32` | `[sp-40]` | `0x1` | `?` |
| `bl` at 0x63a88 | `(unknown callee) 0x1fe64` | `0x11a998` | `?` | `?` | `?` |
| `bl` at 0x63ac0 | `(unknown callee) 0x1eed0` | `0x9` | `0xe7664` | `0x118` | `0x0` |
| `bl` at 0x63ac8 | `(unknown callee) 0x5e7a8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x63d78 | `(unknown callee) 0x5f0a4` | `[sp-244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x63734 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x63764 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x63804 | `[0x11a76c] == 0x0` | `[0x11a76c] != 0x0` |
| `bgt` at 0x63810 | `[sp-28] > 0x1 (signed, 32-bit)` | `[sp-28] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x638a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x638d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x638fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63924 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63944 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63964 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63984 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x639a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x639dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63a04 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63a2c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63a54 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63a9c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |
| `beq` at 0x63aec | `[sp-248] == 0x3300` | `[sp-248] != 0x3300` |
| `bhi` at 0x63af8 | `[sp-248] > 0x3300 (unsigned, 32-bit)` | `[sp-248] <= 0x3300 (unsigned, 32-bit)` |
| `beq` at 0x63b04 | `[sp-248] == 0x2000` | `[sp-248] != 0x2000` |
| `bhi` at 0x63b10 | `[sp-248] > 0x2000 (unsigned, 32-bit)` | `[sp-248] <= 0x2000 (unsigned, 32-bit)` |
| `beq` at 0x63b1c | `[sp-248] == 0x1200` | `[sp-248] != 0x1200` |
| `bhi` at 0x63b28 | `[sp-248] > 0x1200 (unsigned, 32-bit)` | `[sp-248] <= 0x1200 (unsigned, 32-bit)` |
| `beq` at 0x63b34 | `[sp-248] == 0x1000` | `[sp-248] != 0x1000` |
| `beq` at 0x63b40 | `[sp-248] == 0x1100` | `[sp-248] != 0x1100` |
| `beq` at 0x63b50 | `[sp-248] == 0x1300` | `[sp-248] != 0x1300` |
| `beq` at 0x63b5c | `[sp-248] == 0x1400` | `[sp-248] != 0x1400` |
| `beq` at 0x63b6c | `[sp-248] == 0x3000` | `[sp-248] != 0x3000` |
| `bhi` at 0x63b78 | `[sp-248] > 0x3000 (unsigned, 32-bit)` | `[sp-248] <= 0x3000 (unsigned, 32-bit)` |
| `beq` at 0x63b84 | `[sp-248] == 0x2100` | `[sp-248] != 0x2100` |
| `beq` at 0x63b90 | `[sp-248] == 0x2200` | `[sp-248] != 0x2200` |
| `beq` at 0x63ba0 | `[sp-248] == 0x3100` | `[sp-248] != 0x3100` |
| `beq` at 0x63bac | `[sp-248] == 0x3200` | `[sp-248] != 0x3200` |
| `beq` at 0x63bbc | `[sp-248] == 0x6000` | `[sp-248] != 0x6000` |
| `bhi` at 0x63bc8 | `[sp-248] > 0x6000 (unsigned, 32-bit)` | `[sp-248] <= 0x6000 (unsigned, 32-bit)` |
| `beq` at 0x63bd4 | `[sp-248] == 0x4000` | `[sp-248] != 0x4000` |
| `bhi` at 0x63be0 | `[sp-248] > 0x4000 (unsigned, 32-bit)` | `[sp-248] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0x63bec | `[sp-248] == 0x3400` | `[sp-248] != 0x3400` |
| `beq` at 0x63bf8 | `[sp-248] == 0x3500` | `[sp-248] != 0x3500` |
| `beq` at 0x63c08 | `[sp-248] == 0x4100` | `[sp-248] != 0x4100` |
| `beq` at 0x63c14 | `[sp-248] == 0x5000` | `[sp-248] != 0x5000` |
| `beq` at 0x63c24 | `[sp-248] == 0x7200` | `[sp-248] != 0x7200` |
| `bhi` at 0x63c30 | `[sp-248] > 0x7200 (unsigned, 32-bit)` | `[sp-248] <= 0x7200 (unsigned, 32-bit)` |
| `beq` at 0x63c3c | `[sp-248] == 0x7000` | `[sp-248] != 0x7000` |
| `beq` at 0x63c48 | `[sp-248] == 0x7100` | `[sp-248] != 0x7100` |
| `beq` at 0x63c58 | `[sp-248] == 0x7400` | `[sp-248] != 0x7400` |
| `bhi` at 0x63c64 | `[sp-248] > 0x7400 (unsigned, 32-bit)` | `[sp-248] <= 0x7400 (unsigned, 32-bit)` |
| `beq` at 0x63c70 | `[sp-248] == 0x7300` | `[sp-248] != 0x7300` |
| `beq` at 0x63c80 | `[sp-248] == 0x7500` | `[sp-248] != 0x7500` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63d7c | `r0` | `ret(0x5f0a4)` |

## `EnvInit`

- `EnvInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63db8 to 0x64324.
  Established. The two opening calls to `0x6bbd4` each feed an unsigned comparison whose taken edge
  is `ret(0x6bbd4) <= 0x7ffff` (`bl` at 0x63dd0, `bls` at 0x63de0, `bl` at 0x63e00, `bls` at
  0x63e10). A word store increments absolute `0x11a760` (`str` at 0x63e6c). Later stores put
  `ret(0x39994)`, `ret(0x4220c)`, `ret(0x2d6d4)` and `ret(0xaff10)` at absolute `0x11aa68`,
  `0x11aa6c`, `0x11aa70` and `0x11aa80`, respectively (`str` at 0x64110, `str` at 0x6416c, `str` at
  0x641d4, `str` at 0x64230). The explicit-link call has unknown target with candidates `[sp-32]`,
  `0x11aa58`, unknown and `[0x27eeb8]` (`mov` at 0x640f0). The meanings and units of the stored
  quantities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63dcc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x63dd8 | load | 4 | `abs 0x6423c` | `-` |
| `str` at 0x63de8 | store | 4 | `sp -0x2c` | `0x960` |
| `ldr` at 0x63df0 | load | 4 | `abs 0x64240` | `-` |
| `str` at 0x63df4 | store | 4 | `sp -0x2c` | `0x5dc` |
| `ldr` at 0x63df8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x63dfc | store | 4 | `sp -0x24` | `[sp-44]` |
| `ldr` at 0x63e08 | load | 4 | `abs 0x6423c` | `-` |
| `str` at 0x63e18 | store | 4 | `sp -0x30` | `0x960` |
| `ldr` at 0x63e20 | load | 4 | `abs 0x64240` | `-` |
| `str` at 0x63e24 | store | 4 | `sp -0x30` | `0x5dc` |
| `ldr` at 0x63e28 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x63e2c | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0x63e30 | load | 4 | `abs 0x64244` | `-` |
| `ldr` at 0x63e34 | load | 4 | `abs 0x11a760` | `-` |
| `ldr` at 0x63e40 | load | 4 | `abs 0x64248` | `-` |
| `str` at 0x63e44 | store | 4 | `sp -0x44` | `0xe76a8` |
| `ldr` at 0x63e4c | load | 4 | `abs 0x6424c` | `-` |
| `ldr` at 0x63e50 | load | 4 | `abs 0x64250` | `-` |
| `ldr` at 0x63e5c | load | 4 | `abs 0x64244` | `-` |
| `ldr` at 0x63e60 | load | 4 | `abs 0x64244` | `-` |
| `ldr` at 0x63e64 | load | 4 | `abs 0x11a760` | `-` |
| `str` at 0x63e6c | store | 4 | `abs 0x11a760` | `[0x11a760]+1` |
| `ldr` at 0x63e70 | load | 4 | `abs 0x64254` | `-` |
| `str` at 0x63e80 | store | 4 | `abs 0x11af18` | `?` |
| `ldr` at 0x63e84 | load | 4 | `abs 0x11af18` | `-` |
| `ldr` at 0x63e94 | load | 4 | `abs 0x64254` | `-` |
| `ldr` at 0x63e98 | load | 4 | `abs 0x11af18` | `-` |
| `ldr` at 0x63ea8 | load | 4 | `abs 0x64258` | `-` |
| `str` at 0x63eb0 | store | 4 | `abs 0x11a778` | `0x0` |
| `ldr` at 0x63eb4 | load | 4 | `abs 0x6425c` | `-` |
| `ldr` at 0x63ec4 | load | 4 | `abs 0x64260` | `-` |
| `ldr` at 0x63ec8 | load | 4 | `abs 0x64264` | `-` |
| `ldr` at 0x63ed8 | load | 4 | `abs 0x64268` | `-` |
| `ldr` at 0x63edc | load | 4 | `abs 0x6426c` | `-` |
| `ldr` at 0x63eec | load | 4 | `abs 0x64270` | `-` |
| `ldr` at 0x63ef0 | load | 4 | `abs 0x64274` | `-` |
| `ldr` at 0x63f00 | load | 4 | `abs 0x64278` | `-` |
| `ldr` at 0x63f04 | load | 4 | `abs 0x6427c` | `-` |
| `ldr` at 0x63f14 | load | 4 | `abs 0x64280` | `-` |
| `ldr` at 0x63f18 | load | 4 | `abs 0x64284` | `-` |
| `ldr` at 0x63f28 | load | 4 | `abs 0x64288` | `-` |
| `ldr` at 0x63f2c | load | 4 | `abs 0x6428c` | `-` |
| `ldr` at 0x63f3c | load | 4 | `abs 0x64290` | `-` |
| `ldr` at 0x63f40 | load | 4 | `abs 0x64294` | `-` |
| `ldr` at 0x63f50 | load | 4 | `abs 0x64298` | `-` |
| `ldr` at 0x63f54 | load | 4 | `abs 0x6429c` | `-` |
| `str` at 0x63f68 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x63f6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63f70 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x63f78 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x63f80 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x63f88 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x63f8c | load | 4 | `abs 0x642a0` | `-` |
| `ldr` at 0x63f90 | load | 4 | `abs 0x642a4` | `-` |
| `ldr` at 0x63f94 | load | 4 | `abs 0x642a8` | `-` |
| `ldr` at 0x63f98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63fa4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x63fa8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63fac | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x63fb4 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x63fbc | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x63fc4 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x63fc8 | load | 4 | `abs 0x642ac` | `-` |
| `ldr` at 0x63fcc | load | 4 | `abs 0x642b0` | `-` |
| `ldr` at 0x63fd0 | load | 4 | `abs 0x642b4` | `-` |
| `ldr` at 0x63fd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63fe0 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x63fe4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x63fe8 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x63ff0 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x63ff8 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x64000 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x64004 | load | 4 | `abs 0x642b8` | `-` |
| `ldr` at 0x64008 | load | 4 | `abs 0x642bc` | `-` |
| `ldr` at 0x6400c | load | 4 | `abs 0x642c0` | `-` |
| `ldr` at 0x64010 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6401c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x64020 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x64024 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x6402c | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x64034 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x6403c | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x64040 | load | 4 | `abs 0x642c4` | `-` |
| `ldr` at 0x64044 | load | 4 | `abs 0x642c8` | `-` |
| `ldr` at 0x64048 | load | 4 | `abs 0x642cc` | `-` |
| `ldr` at 0x6404c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64058 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x6405c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x64060 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x64068 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x64070 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x64078 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x6407c | load | 4 | `abs 0x642d0` | `-` |
| `ldr` at 0x64080 | load | 4 | `abs 0x642d4` | `-` |
| `ldr` at 0x64084 | load | 4 | `abs 0x642d8` | `-` |
| `ldr` at 0x64088 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64094 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x64098 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6409c | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x640a4 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x640ac | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x640b4 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x640b8 | load | 4 | `abs 0x642dc` | `-` |
| `ldr` at 0x640bc | load | 4 | `abs 0x642e0` | `-` |
| `ldr` at 0x640c0 | load | 4 | `abs 0x642e4` | `-` |
| `ldr` at 0x640c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x640cc | load | 4 | `abs 0x642e8` | `-` |
| `ldr` at 0x640d0 | load | 4 | `abs 0x27eeb8` | `-` |
| `ldr` at 0x640dc | load | 4 | `abs 0x642e8` | `-` |
| `ldr` at 0x640e0 | load | 4 | `abs 0x27eeb8` | `-` |
| `ldr` at 0x640e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x640e8 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x640f4 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64104 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64108 | load | 4 | `abs 0x642f0` | `-` |
| `str` at 0x64110 | store | 4 | `abs 0x11aa68` | `ret(0x39994)` |
| `ldr` at 0x64114 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64118 | load | 4 | `abs 0x11aa68` | `-` |
| `ldr` at 0x64124 | load | 4 | `abs 0x642f4` | `-` |
| `str` at 0x64128 | store | 4 | `sp -0x44` | `0xe776c` |
| `ldr` at 0x64130 | load | 4 | `abs 0x6424c` | `-` |
| `ldr` at 0x64134 | load | 4 | `abs 0x642f8` | `-` |
| `ldr` at 0x64144 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64148 | load | 4 | `abs 0x11aa68` | `-` |
| `ldr` at 0x64150 | load | 4 | `abs 0x642fc` | `-` |
| `ldr` at 0x64160 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64164 | load | 4 | `abs 0x64300` | `-` |
| `str` at 0x6416c | store | 4 | `abs 0x11aa6c` | `ret(0x4220c)` |
| `ldr` at 0x64170 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64174 | load | 4 | `abs 0x11aa6c` | `-` |
| `ldr` at 0x64180 | load | 4 | `abs 0x64304` | `-` |
| `str` at 0x64184 | store | 4 | `sp -0x44` | `0xe7790` |
| `ldr` at 0x6418c | load | 4 | `abs 0x6424c` | `-` |
| `ldr` at 0x64190 | load | 4 | `abs 0x64308` | `-` |
| `ldr` at 0x641a0 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x641a4 | load | 4 | `abs 0x11aa6c` | `-` |
| `ldr` at 0x641b8 | load | 4 | `abs 0x6430c` | `-` |
| `ldr` at 0x641c8 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x641cc | load | 4 | `abs 0x64310` | `-` |
| `str` at 0x641d4 | store | 4 | `abs 0x11aa70` | `ret(0x2d6d4)` |
| `ldr` at 0x641d8 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x641dc | load | 4 | `abs 0x11aa70` | `-` |
| `ldr` at 0x641e8 | load | 4 | `abs 0x64314` | `-` |
| `str` at 0x641ec | store | 4 | `sp -0x44` | `0xe77ac` |
| `ldr` at 0x641f4 | load | 4 | `abs 0x6424c` | `-` |
| `ldr` at 0x641f8 | load | 4 | `abs 0x64318` | `-` |
| `ldr` at 0x64208 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x6420c | load | 4 | `abs 0x11aa70` | `-` |
| `ldr` at 0x64214 | load | 4 | `abs 0x6431c` | `-` |
| `ldr` at 0x64224 | load | 4 | `abs 0x642ec` | `-` |
| `ldr` at 0x64228 | load | 4 | `abs 0x64320` | `-` |
| `str` at 0x64230 | store | 4 | `abs 0x11aa80` | `ret(0xaff10)` |
| `ldmdb` at 0x64238 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x64238 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64238 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64238 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63dd0 | `(unknown callee) 0x6bbd4` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x63e00 | `(unknown callee) 0x6bbd4` | `ret(0x6bbd4)` | `?` | `ret(0x6bbd4)` | `[sp-44]` |
| `bl` at 0x63e58 | `(unknown callee) 0x1eed0` | `0x9` | `0xe7664` | `0x163` | `0x0` |
| `bl` at 0x63e74 | `(unknown callee) 0x2b31c` | `?` | `?` | `0x11a760` | `[0x11a760]+1` |
| `bl` at 0x63ec0 | `(unknown callee) 0x9a7e0` | `0x11a798` | `0x4` | `0x0` | `0x0` |
| `bl` at 0x63ed4 | `(unknown callee) 0x1f9e8` | `0x11a7a0` | `0xe76c4` | `0x0` | `0x9` |
| `bl` at 0x63ee8 | `(unknown callee) 0x1f9e8` | `0x11a7f4` | `0xe76d0` | `0x0` | `0x9` |
| `bl` at 0x63efc | `(unknown callee) 0x1f9e8` | `0x11a848` | `0xe76dc` | `0x0` | `0x9` |
| `bl` at 0x63f10 | `(unknown callee) 0x1f9e8` | `0x11a89c` | `0xe76e8` | `0x0` | `0x9` |
| `bl` at 0x63f24 | `(unknown callee) 0x1f9e8` | `0x11a8f0` | `0xe76f4` | `0x0` | `0x9` |
| `bl` at 0x63f38 | `(unknown callee) 0x1f9e8` | `0x11a944` | `0xe7700` | `0x0` | `0x9` |
| `bl` at 0x63f4c | `(unknown callee) 0x1f9e8` | `0x11a998` | `0xe770c` | `0x1` | `0x9` |
| `bl` at 0x63f60 | `(unknown callee) 0x1f9e8` | `0x11a9ec` | `0xe7718` | `0x1` | `0x9` |
| `bl` at 0x63f9c | `(unknown callee) 0x1f498` | `0x11aa40` | `0xe7724` | `0x677f8` | `[sp-32]` |
| `bl` at 0x63fd8 | `(unknown callee) 0x1f498` | `0x11aa44` | `0xe7730` | `0x68068` | `[sp-32]` |
| `bl` at 0x64014 | `(unknown callee) 0x1f498` | `0x11aa48` | `0xe773c` | `0x68618` | `[sp-32]` |
| `bl` at 0x64050 | `(unknown callee) 0x1f498` | `0x11aa4c` | `0xe7748` | `0x68ec4` | `[sp-32]` |
| `bl` at 0x6408c | `(unknown callee) 0x1f498` | `0x11aa50` | `0xe7754` | `0x691c0` | `[sp-32]` |
| `bl` at 0x640c8 | `(unknown callee) 0x1f498` | `0x11aa54` | `0xe7760` | `0x694fc` | `[sp-32]` |
| `mov` at 0x640f0 | `(unknown callee) (unknown target)` | `[sp-32]` | `0x11aa58` | `?` | `[0x27eeb8]` |
| `bl` at 0x64100 | `(unknown callee) 0x18c20` | `0x11aa58` | `0x0` | `0x4c0` | `?` |
| `bl` at 0x6410c | `(unknown callee) 0x39994` | `0x11aa74` | `?` | `?` | `?` |
| `bl` at 0x6413c | `(unknown callee) 0x1eed0` | `0x9` | `0xe7664` | `0x1a7` | `0x0` |
| `bl` at 0x6415c | `(unknown callee) 0x18c20` | `0x11aa88` | `0x0` | `?` | `?` |
| `bl` at 0x64168 | `(unknown callee) 0x4220c` | `0x11aa78` | `?` | `?` | `?` |
| `bl` at 0x64198 | `(unknown callee) 0x1eed0` | `0x9` | `0xe7664` | `0x1b2` | `0x0` |
| `bl` at 0x641c4 | `(unknown callee) 0x18c20` | `0x11abf4` | `0x0` | `?` | `?` |
| `bl` at 0x641d0 | `(unknown callee) 0x2d6d4` | `0x11aa7c` | `?` | `?` | `?` |
| `bl` at 0x64200 | `(unknown callee) 0x1eed0` | `0x9` | `0xe7664` | `0x1bd` | `0x0` |
| `bl` at 0x64220 | `(unknown callee) 0x18c20` | `0x11aec8` | `0x0` | `?` | `?` |
| `bl` at 0x6422c | `(unknown callee) 0xaff10` | `0x11aa84` | `?` | `?` | `?` |
| `bl` at 0x64234 | `(unknown callee) 0x6b748` | `ret(0xaff10)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x63de0 | `ret(0x6bbd4) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x6bbd4) > 0x7ffff (unsigned, 32-bit)` |
| `bls` at 0x63e10 | `ret(0x6bbd4) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x6bbd4) > 0x7ffff (unsigned, 32-bit)` |
| `beq` at 0x63e3c | `[0x11a760] == 0x0` | `[0x11a760] != 0x0` |
| `bne` at 0x63e8c | `[0x11af18] != 0x54` | `[0x11af18] == 0x54` |
| `bne` at 0x63ea0 | `[0x11af18] != 0x32` | `[0x11af18] == 0x32` |
| `beq` at 0x640d8 | `[0x27eeb8] == 0x0` | `[0x27eeb8] != 0x0` |
| `ble` at 0x64120 | `[0x11aa68] <= 0x2a (signed, 32-bit)` | `[0x11aa68] > 0x2a (signed, 32-bit)` |
| `ble` at 0x6417c | `[0x11aa6c] <= 0x14 (signed, 32-bit)` | `[0x11aa6c] > 0x14 (signed, 32-bit)` |
| `ble` at 0x641e4 | `[0x11aa70] <= 0x14 (signed, 32-bit)` | `[0x11aa70] > 0x14 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64238 | `r0` | `?` |

## `EnvQuery`

- `EnvQuery`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b2a0 to 0x6b654.
  Established. A call to `0x18920` has an unknown first candidate, with second candidate
  `[0x11aa68]` and third `(([0x11aa68] + [0x11aa6c]) + [0x11aa70])` (`bl` at 0x6b32c); its result is
  written as a word at `[sp-24] +0x0` (`str` at 0x6b334). A zero value loaded from that target falls
  through the branch to a call to `0x18788` and a replacement word store of that result (`bne`
  at 0x6b340, `bl` at 0x6b348, `str` at 0x6b358). The nonzero path includes a byte store of `0x0` at
  `[[sp-24]] +0x0` (`strb` at 0x6b374). A later computed transfer has taken condition `[0x11aa64] <=
  0x3` unsigned and falls through for a larger value, but its computed destinations remain
  unresolved (`ldrls` at 0x6b444). The output’s format and complete path coverage are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b2b0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x6b2b4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x6b2d0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x6b2ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6b2f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6b2f4 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b2f8 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b2fc | load | 4 | `abs 0x11aa68` | `-` |
| `ldr` at 0x6b300 | load | 4 | `abs 0x11aa6c` | `-` |
| `ldr` at 0x6b308 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b30c | load | 4 | `abs 0x11aa70` | `-` |
| `str` at 0x6b334 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x6b338 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x6b344 | load | 4 | `abs 0x6b614` | `-` |
| `ldr` at 0x6b354 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6b358 | store | 4 | `[sp-24] +0x0` | `ret(0x18788)` |
| `ldr` at 0x6b360 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6b364 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x6b368 | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0x6b36c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x6b374 | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x6b378 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6b384 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b388 | load | 4 | `abs 0x6b618` | `-` |
| `ldr` at 0x6b390 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b394 | load | 4 | `abs 0x6b61c` | `-` |
| `ldr` at 0x6b39c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b3a0 | load | 4 | `abs 0x6b620` | `-` |
| `ldr` at 0x6b3a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6b3b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b3b8 | load | 4 | `abs 0x6b624` | `-` |
| `ldr` at 0x6b3c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b3c4 | load | 4 | `abs 0x6b628` | `-` |
| `ldr` at 0x6b3cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b3d0 | load | 4 | `abs 0x6b62c` | `-` |
| `ldr` at 0x6b3d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b3dc | load | 4 | `abs 0x6b630` | `-` |
| `ldr` at 0x6b3e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b3ec | load | 4 | `abs 0x6b634` | `-` |
| `ldr` at 0x6b3f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b404 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b40c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x6b410 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b414 | load | 4 | `abs 0x11aa58` | `-` |
| `str` at 0x6b418 | store | 4 | `sp -0x2c` | `[0x11aa58]` |
| `ldr` at 0x6b41c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6b428 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6b438 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b43c | load | 4 | `abs 0x11aa64` | `-` |
| `ldrls` at 0x6b444 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6b49c | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b4a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b4a4 | load | 4 | `abs 0x6b648` | `-` |
| `ldr` at 0x6b4a8 | load | 4 | `abs 0x11aa64` | `-` |
| `ldr` at 0x6b4b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b4b8 | load | 4 | `abs 0x6b64c` | `-` |
| `ldr` at 0x6b4c4 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b4c8 | load | 4 | `abs 0x6b610` | `-` |
| `ldr` at 0x6b4cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b4d0 | load | 4 | `abs 0x6b650` | `-` |
| `ldr` at 0x6b4d4 | load | 4 | `abs 0x11aa58` | `-` |
| `ldr` at 0x6b4d8 | load | 4 | `abs 0x11aa64` | `-` |
| `ldr` at 0x6b4e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b4f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b4f8 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x6b4fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b50c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b510 | load | 4 | `abs 0x6b638` | `-` |
| `ldr` at 0x6b518 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b528 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b530 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x6b534 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b53c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b54c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b554 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x6b558 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b560 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b570 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b578 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x6b57c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b584 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b594 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b59c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x6b5a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6b5ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b5b0 | load | 4 | `abs 0x6b618` | `-` |
| `ldr` at 0x6b5b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b5bc | load | 4 | `abs 0x6b61c` | `-` |
| `ldr` at 0x6b5c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b5c8 | load | 4 | `abs 0x6b620` | `-` |
| `ldr` at 0x6b5d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6b5dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b5e0 | load | 4 | `abs 0x6b624` | `-` |
| `ldr` at 0x6b5e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b5ec | load | 4 | `abs 0x6b628` | `-` |
| `ldr` at 0x6b5f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b5f8 | load | 4 | `abs 0x6b62c` | `-` |
| `ldr` at 0x6b600 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b604 | load | 4 | `abs 0x6b630` | `-` |
| `ldmdb` at 0x6b60c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b60c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b60c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b60c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b2bc | `(unknown callee) 0x1c79c` | `0x6` | `r1` | `r2` | `r3` |
| `bl` at 0x6b2d8 | `(unknown callee) 0x1c79c` | `0x7` | `?` | `?` | `?` |
| `bl` at 0x6b32c | `(unknown callee) 0x18920` | `?` | `[0x11aa68]` | `(([0x11aa68] + [0x11aa6c]) + [0x11aa70])` | `?` |
| `bl` at 0x6b348 | `(unknown callee) 0x18788` | `0xe833c` | `?` | `?` | `[[sp-24]]` |
| `bl` at 0x6b38c | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe836c` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x6b398 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe83a0` | `?` | `?` |
| `bl` at 0x6b3a4 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe83d4` | `?` | `?` |
| `bl` at 0x6b3bc | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe8408` | `?` | `[sp-36]` |
| `bl` at 0x6b3c8 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe843c` | `?` | `?` |
| `bl` at 0x6b3d4 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe8470` | `?` | `?` |
| `bl` at 0x6b3e0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe84a0` | `?` | `?` |
| `bl` at 0x6b3f0 | `(unknown callee) 0x18cd4` | `[sp-40]` | `0xe84d4` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x6b3f8 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x6b4ac | `(unknown callee) 0x18c74` | `([sp-40] + ret(0x18a1c))` | `0xe8578` | `[0x11aa64]` | `0x11aa58` |
| `bl` at 0x6b4bc | `(unknown callee) 0x18cd4` | `([sp-40] + ret(0x18a1c))` | `0xe85a8` | `ret(0x18a1c)` | `[0x11aa58]` |
| `bl` at 0x6b4dc | `(unknown callee) 0x18c74` | `([sp-40] + ret(0x18a1c))` | `0xe85b8` | `[0x11aa58]` | `[0x11aa64]` |
| `bl` at 0x6b4e4 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x6b514 | `(unknown callee) 0x18ab8` | `([sp-40] + ret(0x18a1c))` | `0xe8058` | `ret(0x18a1c)` | `[sp-28]` |
| `bl` at 0x6b51c | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x6b538 | `(unknown callee) 0x69f30` | `([sp-40] + ret(0x18a1c))` | `?` | `ret(0x18a1c)` | `([sp-40] + ret(0x18a1c))` |
| `bl` at 0x6b540 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x6b55c | `(unknown callee) 0x6a408` | `([sp-40] + ret(0x18a1c))` | `?` | `ret(0x18a1c)` | `([sp-40] + ret(0x18a1c))` |
| `bl` at 0x6b564 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x6b580 | `(unknown callee) 0x6a6b4` | `([sp-40] + ret(0x18a1c))` | `?` | `ret(0x18a1c)` | `([sp-40] + ret(0x18a1c))` |
| `bl` at 0x6b588 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x6b5b4 | `(unknown callee) 0x18ab8` | `([sp-40] + ret(0x18a1c))` | `0xe836c` | `ret(0x18a1c)` | `[sp-32]` |
| `bl` at 0x6b5c0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe83a0` | `?` | `?` |
| `bl` at 0x6b5cc | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe83d4` | `?` | `?` |
| `bl` at 0x6b5e4 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe8408` | `?` | `[sp-36]` |
| `bl` at 0x6b5f0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe843c` | `?` | `?` |
| `bl` at 0x6b5fc | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe8470` | `?` | `?` |
| `bl` at 0x6b608 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xe84a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6b340 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0x6b380 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x6b3b0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x6b424 | `[0x11aa58] == 0x0` | `[0x11aa58] != 0x0` |
| `beq` at 0x6b430 | `[0x11aa58] == 0x1` | `[0x11aa58] != 0x1` |
| `ldrls` at 0x6b444 | `[0x11aa64] <= 0x3 (unsigned, 32-bit)` | `[0x11aa64] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x6b504 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x6b5a8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x6b5d8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b60c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6b444).

## `EnvReset`

- `EnvReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b748 to 0x6b80c.
  Established. A nonzero word at absolute `0x11a760` takes the branch into the body, while zero
  falls through to the return (`ldr` at 0x6b75c, `bne` at 0x6b764). The body includes a call to
  `0x1c79c` with first candidate `0x6`, followed by a word store of unknown value at absolute
  `0x11aa58` (`bl` at 0x6b778, `str` at 0x6b78c). An optional explicit-link call has unknown target
  and candidates `ret(0x65110)`, unknown, unknown, `[0x27ee58]` (`mov` at 0x6b7b0). Later a zero
  word at absolute `0x11aa64` takes the edge to a word store of `0x2` at `sp -0x18`, whereas
  nonzero falls through to a store of `0x6` there (`ldr` at 0x6b7c8, `beq` at 0x6b7d0, `str` at
  0x6b7e4, `str` at 0x6b7d8). The reset behavior behind the numeric callees is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6b758 | load | 4 | `abs 0x6b800` | `-` |
| `ldr` at 0x6b75c | load | 4 | `abs 0x11a760` | `-` |
| `ldr` at 0x6b770 | load | 4 | `abs 0x6b804` | `-` |
| `str` at 0x6b78c | store | 4 | `abs 0x11aa58` | `?` |
| `ldr` at 0x6b794 | load | 4 | `abs 0x6b808` | `-` |
| `ldr` at 0x6b798 | load | 4 | `abs 0x27ee58` | `-` |
| `ldr` at 0x6b7a4 | load | 4 | `abs 0x6b808` | `-` |
| `ldr` at 0x6b7a8 | load | 4 | `abs 0x27ee58` | `-` |
| `ldr` at 0x6b7b4 | load | 4 | `abs 0x6b804` | `-` |
| `ldr` at 0x6b7b8 | load | 4 | `abs 0x11aa58` | `-` |
| `ldr` at 0x6b7c4 | load | 4 | `abs 0x6b804` | `-` |
| `ldr` at 0x6b7c8 | load | 4 | `abs 0x11aa64` | `-` |
| `str` at 0x6b7d8 | store | 4 | `sp -0x18` | `0x6` |
| `str` at 0x6b7e4 | store | 4 | `sp -0x18` | `0x2` |
| `str` at 0x6b7f0 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x6b7f4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6b7fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b7fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b7fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b7fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b76c | `(unknown callee) 0x64324` | `r0` | `r1` | `r2` | `[0x11a760]` |
| `bl` at 0x6b778 | `(unknown callee) 0x1c79c` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x6b790 | `(unknown callee) 0x65110` | `ret(0x1c79c)` | `?` | `?` | `?` |
| `mov` at 0x6b7b0 | `(unknown callee) (unknown target)` | `ret(0x65110)` | `?` | `?` | `[0x27ee58]` |
| `bl` at 0x6b7f8 | `(unknown callee) 0x5f0a4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6b764 | `[0x11a760] != 0x0` | `[0x11a760] == 0x0` |
| `beq` at 0x6b7a0 | `[0x27ee58] == 0x0` | `[0x27ee58] != 0x0` |
| `bne` at 0x6b7c0 | `[0x11aa58] != 0x0` | `[0x11aa58] == 0x0` |
| `beq` at 0x6b7d0 | `[0x11aa64] == 0x0` | `[0x11aa64] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b7fc | `r0` | `?` |

## `EnvMessages`

- `EnvMessages`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bb0c to 0x6bbd4.
  Established. Entry r0 equal to `0x1` takes the edge to a word store of `0x1` at absolute
  `0x11a76c` (`beq` at 0x6bb30, `str` at 0x6bb90). Entry r0 equal to `0x2` takes a separate edge
  to word `0x0` at that same address (`beq` at 0x6bb58, `str` at 0x6bba8). The r0-zero branch
  leads to a test of `[0x11a76c]`, where zero takes the edge to a stack store of `0xe86b8` and
  nonzero falls through to `0xe8694` (`beq` at 0x6bb48, `beq` at 0x6bb6c, `str` at 0x6bb80,
  `str` at 0x6bb74). The return reports `[sp-24]` in r0 (`ldmdb` at 0x6bbbc). The constant
  addresses’ pointee types and the absolute flag’s message policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6bb1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6bb20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6bb24 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x6bb28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bb34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bb40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bb50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bb60 | load | 4 | `abs 0x6bbc0` | `-` |
| `ldr` at 0x6bb64 | load | 4 | `abs 0x11a76c` | `-` |
| `ldr` at 0x6bb70 | load | 4 | `abs 0x6bbc4` | `-` |
| `str` at 0x6bb74 | store | 4 | `sp -0x18` | `0xe8694` |
| `ldr` at 0x6bb7c | load | 4 | `abs 0x6bbc8` | `-` |
| `str` at 0x6bb80 | store | 4 | `sp -0x18` | `0xe86b8` |
| `ldr` at 0x6bb88 | load | 4 | `abs 0x6bbc0` | `-` |
| `str` at 0x6bb90 | store | 4 | `abs 0x11a76c` | `0x1` |
| `ldr` at 0x6bb94 | load | 4 | `abs 0x6bbcc` | `-` |
| `str` at 0x6bb98 | store | 4 | `sp -0x18` | `0xe86dc` |
| `ldr` at 0x6bba0 | load | 4 | `abs 0x6bbc0` | `-` |
| `str` at 0x6bba8 | store | 4 | `abs 0x11a76c` | `0x0` |
| `ldr` at 0x6bbac | load | 4 | `abs 0x6bbd0` | `-` |
| `str` at 0x6bbb0 | store | 4 | `sp -0x18` | `0xe8708` |
| `ldr` at 0x6bbb8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6bbbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6bbbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6bbbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6bb30 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0x6bb3c | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x6bb48 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x6bb58 | `r0 == 0x2` | `r0 != 0x2` |
| `beq` at 0x6bb6c | `[0x11a76c] == 0x0` | `[0x11a76c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6bbbc | `r0` | `[sp-24]` |

## `EnvAuto`

- `EnvAuto`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b6b4 to 0x6b720.
  Established. A zero entry r0 takes the edge past the first two calls, while nonzero falls through
  (`beq` at 0x6b6d0). The bypassed calls target `0x1abd0` with first candidates `0x7` and `0x7d`,
  each with an unknown second candidate (`bl` at 0x6b6ec, `bl` at 0x6b708). The joined path calls
  `0x1abd0` with first candidate `0x6` and the rest unknown (`bl` at 0x6b714), followed by a call to
  `0x6b748` whose result reaches the return (`bl` at 0x6b718, `ldmdb` at 0x6b71c). These selectors’
  control meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b6c4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6b6c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6b6d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6b6f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6b710 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b71c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b71c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b71c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b6ec | `(unknown callee) 0x1abd0` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x6b708 | `(unknown callee) 0x1abd0` | `0x7d` | `?` | `?` | `?` |
| `bl` at 0x6b714 | `(unknown callee) 0x1abd0` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x6b718 | `(unknown callee) 0x6b748` | `ret(0x1abd0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6b6d0 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b71c | `r0` | `ret(0x6b748)` |

## `EnvAltitude`

- `EnvAltitude`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b720 to 0x6b748.
  Established. The first call targets `0x1abd0` with candidates `0xc`, entry r0, entry r2 and entry
  r3 (`bl` at 0x6b73c). It is followed by a call to `0x6b748` with first candidate `ret(0x1abd0)`
  and the rest unknown (`bl` at 0x6b740). The return reports `ret(0x6b748)` (`ldmdb` at 0x6b744).
  Altitude units and any hardware adjustment remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b730 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6b738 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b744 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b744 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b73c | `(unknown callee) 0x1abd0` | `0xc` | `r0` | `r2` | `r3` |
| `bl` at 0x6b740 | `(unknown callee) 0x6b748` | `ret(0x1abd0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b744 | `r0` | `ret(0x6b748)` |

## `EnvTemp`

- `EnvTemp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b654 to 0x6b68c.
  Established. The body saves entry r0 as a word at `sp -0x14` (`str` at 0x6b664); its call to
  `0x1abd0` has candidates `0x7`, unknown, entry r2 and unknown (`bl` at 0x6b680). The subsequent
  call to `0x6b748` uses first candidate `ret(0x1abd0)`, with the others unknown (`bl` at 0x6b684).
  That result reaches the return (`ldmdb` at 0x6b688). Neither a temperature unit nor a
  temperature-control operation follows from these calls alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b664 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6b668 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b688 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b688 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b688 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b680 | `(unknown callee) 0x1abd0` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x6b684 | `(unknown callee) 0x6b748` | `ret(0x1abd0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b688 | `r0` | `ret(0x6b748)` |

## `EnvTestTemp`

- `EnvTestTemp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b930 to 0x6b9b8.
  Established. Six absolute word stores write `0x1` to `0x11a778`, `[sp-20]` to `0x11a77c`,
  `[sp-24]` to `0x11a780`, `[sp-28]` to `0x11a784`, `[sp-32]` to `0x11a788` and `[sp]` to `0x11a78c`
  (`str` at 0x6b958, `str` at 0x6b964, `str` at 0x6b970, `str` at 0x6b97c, `str` at 0x6b988, `str`
  at 0x6b994). A following call to `0x6b748` has candidates entry r0, entry r1, `0x11a78c`, `[sp]`
  (`bl` at 0x6b998). The return reports that call’s result (`ldmdb` at 0x6b99c). The stored test
  values’ interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b940 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6b944 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6b948 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6b94c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x6b950 | load | 4 | `abs 0x6b9a0` | `-` |
| `str` at 0x6b958 | store | 4 | `abs 0x11a778` | `0x1` |
| `ldr` at 0x6b95c | load | 4 | `abs 0x6b9a4` | `-` |
| `ldr` at 0x6b960 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6b964 | store | 4 | `abs 0x11a77c` | `[sp-20]` |
| `ldr` at 0x6b968 | load | 4 | `abs 0x6b9a8` | `-` |
| `ldr` at 0x6b96c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6b970 | store | 4 | `abs 0x11a780` | `[sp-24]` |
| `ldr` at 0x6b974 | load | 4 | `abs 0x6b9ac` | `-` |
| `ldr` at 0x6b978 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6b97c | store | 4 | `abs 0x11a784` | `[sp-28]` |
| `ldr` at 0x6b980 | load | 4 | `abs 0x6b9b0` | `-` |
| `ldr` at 0x6b984 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6b988 | store | 4 | `abs 0x11a788` | `[sp-32]` |
| `ldr` at 0x6b98c | load | 4 | `abs 0x6b9b4` | `-` |
| `ldr` at 0x6b990 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x6b994 | store | 4 | `abs 0x11a78c` | `[sp]` |
| `ldmdb` at 0x6b99c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b99c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b99c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b998 | `(unknown callee) 0x6b748` | `r0` | `r1` | `0x11a78c` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b99c | `r0` | `ret(0x6b748)` |

## `EnvTestEvent`

- `EnvTestEvent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b80c to 0x6b930.
  Established. The early conditional computed transfer takes its unresolved edge for `r0 <= 0x4`
  unsigned and falls through for larger entry values (`ldrls` at 0x6b828). On the known fall-through
  path, the return reports entry r0 (`ldmdb` at 0x6b910). Calls decoded beyond the unresolved edge
  have no entry-reachable call facts, so their arguments and selection mapping remain unknown. The
  retained listing does not establish which event body executes.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b81c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6b820 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x6b828 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x6b910 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b910 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x6b828 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b910 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6b828).

## `SMPSystemConsole`

- `SMPSystemConsole`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53094 to 0x531c4.
  Established. A zero entry r0 takes the first edge to a stack store of `0x0` at `sp -0x28`
  (`beq` at 0x530b8, `str` at 0x530cc). With nonzero entry r0, a nonzero entry r1 takes the edge
  into the longer path, while zero falls through to that same store (`bne` at 0x530c4). The
  longer path includes a word store of `[sp-32]` at absolute `0x281ca4` (`str` at 0x53150). A later
  equality branch takes its edge when `[0x2815e4] == 0x1`, and another takes an edge when that word
  equals `0x2` (`beq` at 0x53170, `beq` at 0x53180). Those paths include a call to `0x54938` with
  first candidate `[0x281ca4]` (`bl` at 0x53190). The word’s console encoding and the merged return
  value remain unknown (`ldmdb` at 0x531b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x530a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x530a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x530ac | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x530b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x530bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x530cc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x530e0 | store | 4 | `sp -0x24` | `ret(0x5e7a8)` |
| `ldr` at 0x530e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x530f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53110 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53114 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5311c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5312c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53134 | load | 4 | `abs 0x531b8` | `-` |
| `ldr` at 0x53138 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5313c | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x53148 | load | 4 | `abs 0x531b8` | `-` |
| `ldr` at 0x5314c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53150 | store | 4 | `abs 0x281ca4` | `[sp-32]` |
| `ldr` at 0x53154 | load | 4 | `abs 0x531bc` | `-` |
| `ldr` at 0x53158 | load | 4 | `abs 0x119f90` | `-` |
| `ldr` at 0x53164 | load | 4 | `abs 0x531b8` | `-` |
| `ldr` at 0x53168 | load | 4 | `abs 0x2815e4` | `-` |
| `ldr` at 0x53174 | load | 4 | `abs 0x531b8` | `-` |
| `ldr` at 0x53178 | load | 4 | `abs 0x2815e4` | `-` |
| `ldr` at 0x53188 | load | 4 | `abs 0x531b8` | `-` |
| `ldr` at 0x5318c | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x53198 | load | 4 | `abs 0x531c0` | `-` |
| `ldr` at 0x531a4 | load | 4 | `abs 0x531b8` | `-` |
| `ldr` at 0x531a8 | load | 4 | `abs 0x281ca4` | `-` |
| `str` at 0x531ac | store | 4 | `sp -0x28` | `[0x281ca4]` |
| `ldr` at 0x531b0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x531b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x531b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x531b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x530d8 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r1` |
| `bl` at 0x53130 | `(unknown callee) 0x5ed98` | `ret(0x5e7a8)` | `?` | `?` | `?` |
| `bl` at 0x53190 | `(unknown callee) 0x54938` | `[0x281ca4]` | `?` | `0x281360` | `0x281360` |
| `bl` at 0x531a0 | `(unknown callee) 0x20f14` | `0xe5eb0` | `ret(0x54938)` | `?` | `ret(0x54938)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x530b8 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x530c4 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x53144 | `[sp-32] == [0x281ca4]` | `[sp-32] != [0x281ca4]` |
| `beq` at 0x53160 | `[0x119f90] == 0x0` | `[0x119f90] != 0x0` |
| `beq` at 0x53170 | `[0x2815e4] == 0x1` | `[0x2815e4] != 0x1` |
| `beq` at 0x53180 | `[0x2815e4] == 0x2` | `[0x2815e4] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x531b4 | `r0` | `?` |

## `SMPConsoleReset`

- `SMPConsoleReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x531c4 to 0x5321c.
  Established. The loop initialises `sp -0x14` to word `0x0` and later stores `[sp-20]+1` there
  (`str` at 0x531d8, `str` at 0x53208); its branch retains only a signed less-or-equal flags
  condition with unknown operands for the taken edge (`ble` at 0x531e4). The loop call targets
  `0x2f90c` with first candidate unknown, second `0x0` and the others unknown (`bl` at 0x531fc).
  After the loop, a call to `0x52edc` has first candidate `0x0` (`bl` at 0x53214), and its result
  reaches the return (`ldmdb` at 0x53218). The full set of affected console channels is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x531d8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x531dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x531ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53200 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53208 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x53218 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53218 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53218 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x531fc | `(unknown callee) 0x2f90c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x53214 | `(unknown callee) 0x52edc` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x531e4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53218 | `r0` | `ret(0x52edc)` |

## `SMPConsoleSubchannel`

- `SMPConsoleSubchannel`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52edc to 0x53094.
  Established. A word store first clears absolute `0x1155ec` (`str` at 0x52ef8). After the call to
  `0x5e7a8`, an equality branch takes the bypass edge for `[sp-20] == [ret(0x5e7a8)+20]` and falls
  through for inequality (`bl` at 0x52f00, `beq` at 0x52f1c). The fall-through path stores `[sp-20]`
  at `ret(0x5e7a8) +0x14` before calling `0x2f90c` with candidates `0x1`, `[sp-20]`, `[sp-20]`,
  `ret(0x5e7a8)` (`str` at 0x52f28, `bl` at 0x52f34). Another word store addresses absolute
  `0x281ca4` with value `[sp-32]` (`str` at 0x53020). The return reports `[sp-20]` (`ldmdb` at
  0x53080). The absolute flag and encoded console word meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52eec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52ef0 | load | 4 | `abs 0x53084` | `-` |
| `str` at 0x52ef8 | store | 4 | `abs 0x1155ec` | `0x0` |
| `str` at 0x52f08 | store | 4 | `sp -0x18` | `ret(0x5e7a8)` |
| `ldr` at 0x52f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52f14 | load | 4 | `ret(0x5e7a8) +0x14` | `-` |
| `ldr` at 0x52f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52f24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52f28 | store | 4 | `ret(0x5e7a8) +0x14` | `[sp-20]` |
| `ldr` at 0x52f3c | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x52f40 | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x52f50 | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x52f54 | load | 4 | `abs 0x281ca4` | `-` |
| `str` at 0x52f70 | store | 4 | `sp -0x1c` | `ret(0x5e5a8)` |
| `ldr` at 0x52f74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52f80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52f84 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x52f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52f90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52f94 | load | 4 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x52f98 | load | 4 | `ret(0x5e5a8) +0x3c` | `-` |
| `ldr` at 0x52fa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52fa8 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x52fb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52fb4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x52fc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52fc8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x52fe4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x52fe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52ff0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53000 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53004 | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x53008 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5300c | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x53018 | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x5301c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53020 | store | 4 | `abs 0x281ca4` | `[sp-32]` |
| `ldr` at 0x53024 | load | 4 | `abs 0x5308c` | `-` |
| `ldr` at 0x53028 | load | 4 | `abs 0x119f90` | `-` |
| `ldr` at 0x53034 | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x53038 | load | 4 | `abs 0x2815e4` | `-` |
| `ldr` at 0x53044 | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x53048 | load | 4 | `abs 0x2815e4` | `-` |
| `ldr` at 0x53058 | load | 4 | `abs 0x53088` | `-` |
| `ldr` at 0x5305c | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x53068 | load | 4 | `abs 0x53090` | `-` |
| `ldr` at 0x53074 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53078 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x5307c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x53080 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53080 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52f00 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `0x1155ec` | `0x0` |
| `bl` at 0x52f34 | `(unknown callee) 0x2f90c` | `0x1` | `[sp-20]` | `[sp-20]` | `ret(0x5e7a8)` |
| `bl` at 0x52f38 | `(unknown callee) 0x5edb8` | `ret(0x2f90c)` | `?` | `?` | `?` |
| `bl` at 0x52f68 | `(unknown callee) 0x5e5a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x53060 | `(unknown callee) 0x54938` | `[0x281ca4]` | `[[sp-24]+60]` | `0x281360` | `0x281360` |
| `bl` at 0x53070 | `(unknown callee) 0x20f14` | `0xe5e88` | `ret(0x54938)` | `?` | `ret(0x54938)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52f1c | `[sp-20] == [ret(0x5e7a8)+20]` | `[sp-20] != [ret(0x5e7a8)+20]` |
| `bne` at 0x52f7c | `ret(0x5e5a8) != 0x0` | `ret(0x5e5a8) == 0x0` |
| `beq` at 0x52fa0 | `[[sp-24]+60] == [ret(0x5e5a8)+60]` | `[[sp-24]+60] != [ret(0x5e5a8)+60]` |
| `beq` at 0x53014 | `[sp-32] == [0x281ca4]` | `[sp-32] != [0x281ca4]` |
| `beq` at 0x53030 | `[0x119f90] == 0x0` | `[0x119f90] != 0x0` |
| `beq` at 0x53040 | `[0x2815e4] == 0x1` | `[0x2815e4] != 0x1` |
| `beq` at 0x53050 | `[0x2815e4] == 0x2` | `[0x2815e4] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53080 | `r0` | `[sp-20]` |

## `SMPConsoleSubchannelDefault`

- `SMPConsoleSubchannelDefault`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52e8c to 0x52edc.
  Established. A word load from absolute `0x1155ec` feeds a branch whose taken edge is zero and
  whose fall-through edge is nonzero (`ldr` at 0x52ea4, `beq` at 0x52eac). The nonzero path calls
  `0x52edc` with candidates entry r0, entry r1, entry r2 and `[0x1155ec]` (`bl` at 0x52eb4), saves
  its result at `sp -0x14` (`str` at 0x52ebc), then writes word `0x1` at absolute `0x1155ec` (`str`
  at 0x52ec8). The return value is unknown after the paths merge (`ldmdb` at 0x52ed4). The flag’s
  default-selection policy is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52e9c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52ea0 | load | 4 | `abs 0x52ed8` | `-` |
| `ldr` at 0x52ea4 | load | 4 | `abs 0x1155ec` | `-` |
| `ldr` at 0x52eb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52ebc | store | 4 | `sp -0x14` | `ret(0x52edc)` |
| `ldr` at 0x52ec0 | load | 4 | `abs 0x52ed8` | `-` |
| `str` at 0x52ec8 | store | 4 | `abs 0x1155ec` | `0x1` |
| `ldr` at 0x52ecc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52ed4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52ed4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52ed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52eb4 | `(unknown callee) 0x52edc` | `r0` | `r1` | `r2` | `[0x1155ec]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52eac | `[0x1155ec] == 0x0` | `[0x1155ec] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52ed4 | `r0` | `?` |

## `sn1sc_select_port`

- `sn1sc_select_port`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2c3c to 0xc2c9c.
  Established. The routine saves the low bytes of entry r0 and r1 at `sp -0x11` and `sp -0x12`
  (`strb` at 0xc2c54, `strb` at 0xc2c5c). A halfword store addresses `[0x284340] +0xe`, with unknown
  stored value (`strh` at 0xc2c88). The return reports `0x1` in r0 (`ldmdb` at 0xc2c94). No calls or
  conditional branches are reported. The target window’s physical base and port-control encoding
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc2c54 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xc2c5c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xc2c64 | load | 4 | `abs 0xc2c98` | `-` |
| `ldr` at 0xc2c68 | load | 4 | `abs 0x284340` | `-` |
| `ldrb` at 0xc2c70 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xc2c78 | load | 1 | `sp -0x11` | `-` |
| `strh` at 0xc2c88 | store | 2 | `[0x284340] +0xe` | `?` |
| `ldmdb` at 0xc2c94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2c94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2c94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2c94 | `r0` | `0x1` |
