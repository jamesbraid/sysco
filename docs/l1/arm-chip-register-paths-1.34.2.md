# ARM chip lookup and register entry paths in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/chip-register-paths-facts.txt) retain all tool sections.

## `JChipSetup`

- `JChipSetup`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8d5c to 0xc8d88.
  Established. One explicit word store writes `0xc99b0` at absolute `0x115854` (`str` at 0xc8d70).
  This body stores the code address without calling it. r0 receives `0x0` and returns that value
  (`mov` at 0xc8d74, `mov` at 0xc8d78, `ldmdb` at 0xc8d7c). No calls or conditional branches are
  reported. The slot’s consumer is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc8d68 | load | 4 | `abs 0xc8d80` | `-` |
| `ldr` at 0xc8d6c | load | 4 | `abs 0xc8d84` | `-` |
| `str` at 0xc8d70 | store | 4 | `abs 0x115854` | `0xc99b0` |
| `ldmdb` at 0xc8d7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8d7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8d7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8d7c | `r0` | `0x0` |

## `JChipBrickSetup`

- `JChipBrickSetup`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8d88 to 0xc8e70.
  Established. Two indirect calls retain unknown targets. The first carries unknown, `0x0`,
  `0x2845a8`, unknown, while the second has unknown, `[[sp-20]+13]:1`, `[0x11584c]`,
  `[[sp-20]+13]:1` (`mov` at 0xc8df0, `mov` at 0xc8e4c). A zero word at absolute `0x27eea8` takes
  the bypass while nonzero falls through to the first call (`beq` at 0xc8db8). One bypass path
  stores word `0xfffff7c5` at `sp -0x1c` (`str` at 0xc8e18). Physical setup behavior and the joined
  return are unknown (`ldmdb` at 0xc8e5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8d98 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc8d9c | load | 4 | `sp -0x14` | `-` |
| `ldrsb` at 0xc8da0 | load | 1 | `r0 +0x15` | `-` |
| `ldr` at 0xc8dac | load | 4 | `abs 0xc8e60` | `-` |
| `ldr` at 0xc8db0 | load | 4 | `abs 0x27eea8` | `-` |
| `ldr` at 0xc8dbc | load | 4 | `abs 0xc8e60` | `-` |
| `ldr` at 0xc8dc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8dc4 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xc8dc8 | load | 4 | `[[sp-20]] +0x0` | `-` |
| `ldr` at 0xc8dd0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8dd4 | load | 1 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xc8ddc | load | 4 | `abs 0x27eea8` | `-` |
| `ldr` at 0xc8de8 | load | 4 | `abs 0xc8e64` | `-` |
| `str` at 0xc8df8 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xc8dfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8e08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8e0c | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0xc8e14 | load | 4 | `abs 0xc8e68` | `-` |
| `str` at 0xc8e18 | store | 4 | `sp -0x1c` | `0xfffff7c5` |
| `ldr` at 0xc8e20 | load | 4 | `abs 0xc8e6c` | `-` |
| `ldr` at 0xc8e24 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8e28 | load | 1 | `[sp-20] +0xd` | `-` |
| `ldr` at 0xc8e34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc8e38 | load | 1 | `[sp-20] +0xd` | `-` |
| `ldr` at 0xc8e3c | load | 4 | `abs 0x11584c` | `-` |
| `str` at 0xc8e54 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc8e58 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc8e5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8e5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8e5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xc8df0 | `(unknown callee) (unknown target)` | `?` | `0x0` | `0x2845a8` | `?` |
| `mov` at 0xc8e4c | `(unknown callee) (unknown target)` | `?` | `[[sp-20]+13]:1` | `[0x11584c]` | `[[sp-20]+13]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8da8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc8db8 | `[0x27eea8] == 0x0` | `[0x27eea8] != 0x0` |
| `beq` at 0xc8e04 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8e5c | `r0` | `?` |

## `JChipInit`

- `JChipInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8e70 to 0xc91a0.
  Established. Absolute word stores put `ret(0x18aac)` at `0x284580` and `[0x2845a0]+1` at
  `0x2845a0` (`str` at 0xc8eb4, `str` at 0xc8efc). Byte stores at `[sp-36] +0xd` and `[sp-36] +0x14`
  write `([sp-25]:1 & 0xff)` and `0x0` (`strb` at 0xc8f20, `strb` at 0xc8f38). An indirect call has
  unknown target and candidates `[sp-36]`, unknown, unknown, `[[[sp-36]]+12]` (`mov` at 0xc8fe4);
  its signed nonnegative result takes the continuation while a negative result falls through to
  `0x1eed0` (`bge` at 0xc8ff0, `bl` at 0xc9018). The physical chip initialization is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8e80 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc8e88 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xc8e8c | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `ldr` at 0xc8e90 | load | 4 | `abs 0xc9180` | `-` |
| `ldr` at 0xc8e94 | load | 4 | `abs 0xc9180` | `-` |
| `ldr` at 0xc8e98 | load | 4 | `abs 0xc9184` | `-` |
| `ldr` at 0xc8e9c | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc8ea8 | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xc8eb4 | store | 4 | `abs 0x284580` | `ret(0x18aac)` |
| `ldr` at 0xc8eb8 | load | 4 | `abs 0x284580` | `-` |
| `ldr` at 0xc8ec4 | load | 4 | `abs 0xc9188` | `-` |
| `str` at 0xc8ec8 | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xc8ed0 | load | 4 | `abs 0xc9180` | `-` |
| `ldr` at 0xc8ed4 | load | 4 | `abs 0xc9184` | `-` |
| `ldr` at 0xc8ed8 | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc8ee0 | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xc8ee8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc8eec | load | 4 | `abs 0xc9184` | `-` |
| `ldr` at 0xc8ef0 | load | 4 | `abs 0xc9184` | `-` |
| `ldr` at 0xc8ef4 | load | 4 | `abs 0x2845a0` | `-` |
| `str` at 0xc8efc | store | 4 | `abs 0x2845a0` | `[0x2845a0]+1` |
| `ldr` at 0xc8f00 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc8f0c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc8f14 | store | 1 | `[sp-36] +0xc` | `?` |
| `ldr` at 0xc8f18 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc8f1c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc8f20 | store | 1 | `[sp-36] +0xd` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xc8f24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8f28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc8f2c | store | 4 | `[sp-36] +0x10` | `[sp-32]` |
| `ldr` at 0xc8f30 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc8f38 | store | 1 | `[sp-36] +0x14` | `0x0` |
| `ldr` at 0xc8f3c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc8f44 | store | 1 | `[sp-36] +0x15` | `?` |
| `ldr` at 0xc8f48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8f4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8f5c | store | 4 | `[sp-36] +0x0` | `ret(0xca3ec)` |
| `ldr` at 0xc8f60 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc8f6c | load | 4 | `abs 0xc918c` | `-` |
| `str` at 0xc8f70 | store | 4 | `sp -0x48` | `0x10a948` |
| `ldr` at 0xc8f74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8f7c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc8f84 | load | 4 | `abs 0xc9190` | `-` |
| `ldr` at 0xc8f94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8f9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8fa0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc8fa4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc8fa8 | load | 1 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xc8fb0 | load | 4 | `abs 0xc9194` | `-` |
| `ldr` at 0xc8fb4 | load | 4 | `[[sp-36]] +0x4` | `-` |
| `ldr` at 0xc8fbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8fc0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc8fc4 | load | 4 | `[[sp-36]] +0xc` | `-` |
| `ldr` at 0xc8fd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8fd4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc8fd8 | load | 4 | `[[sp-36]] +0xc` | `-` |
| `ldr` at 0xc8fdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8ff4 | load | 4 | `abs 0xc9198` | `-` |
| `str` at 0xc8ff8 | store | 4 | `sp -0x48` | `0x10a994` |
| `ldr` at 0xc8ffc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9004 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc900c | load | 4 | `abs 0xc9190` | `-` |
| `str` at 0xc9020 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc9024 | load | 4 | `abs 0xc9184` | `-` |
| `ldr` at 0xc9028 | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc9030 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9040 | load | 4 | `abs 0xc9180` | `-` |
| `ldr` at 0xc9044 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc904c | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xc9054 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc9058 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc905c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9060 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc9064 | load | 1 | `[sp-36] +0xd` | `-` |
| `ldr` at 0xc9070 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9074 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9078 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc907c | load | 4 | `[sp-36] +0x10` | `-` |
| `ldr` at 0xc9088 | load | 4 | `abs 0xc919c` | `-` |
| `str` at 0xc908c | store | 4 | `sp -0x48` | `0x10a9cc` |
| `ldr` at 0xc9090 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9094 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc9098 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc909c | load | 1 | `[sp-36] +0xd` | `-` |
| `str` at 0xc90a0 | store | 4 | `sp -0x40` | `[[sp-36]+13]:1` |
| `ldr` at 0xc90a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc90a8 | load | 4 | `[sp-36] +0x10` | `-` |
| `str` at 0xc90ac | store | 4 | `sp -0x3c` | `[[sp-36]+16]` |
| `ldr` at 0xc90b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc90b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc90b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc90bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc90c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc90c4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc90c8 | load | 4 | `[[sp-36]] +0x0` | `-` |
| `str` at 0xc90cc | store | 4 | `sp -0x34` | `[[[sp-36]]]` |
| `ldr` at 0xc90d4 | load | 4 | `abs 0xc9190` | `-` |
| `ldr` at 0xc90e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc90e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc90ec | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc90f0 | load | 1 | `[sp-36] +0xd` | `-` |
| `ldr` at 0xc90fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9100 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9104 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9108 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc910c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9110 | load | 4 | `[[sp-36]] +0x0` | `-` |
| `ldr` at 0xc911c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9120 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9124 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9128 | load | 4 | `[sp-36] +0x10` | `-` |
| `ldr` at 0xc9134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9138 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc913c | load | 1 | `[sp-36] +0x14` | `-` |
| `strb` at 0xc9144 | store | 1 | `[sp-36] +0x14` | `([[sp-36]+20]:1+1 & 0xff)` |
| `ldr` at 0xc914c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9150 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc9154 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc915c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc9160 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9168 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc9174 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc9178 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xc917c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc917c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc917c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc917c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8eb0 | `(unknown callee) 0x18aac` | `[0x284580]` | `?` | `0x284580` | `?` |
| `bl` at 0xc8f58 | `(unknown callee) 0xca3ec` | `?` | `[sp-36]` | `[sp-36]` | `?` |
| `bl` at 0xc8f90 | `(unknown callee) 0x1eed0` | `0x9` | `0x10a974` | `0xc6` | `0x0` |
| `bl` at 0xc8fb8 | `(unknown callee) 0x18c74` | `[sp-36]+4` | `0x10a98c` | `[[[sp-36]]+4]` | `[[sp-36]+12]:1` |
| `mov` at 0xc8fe4 | `(unknown callee) (unknown target)` | `[sp-36]` | `?` | `?` | `[[[sp-36]]+12]` |
| `bl` at 0xc9018 | `(unknown callee) 0x1eed0` | `0x9` | `0x10a974` | `0xce` | `0x0` |
| `bl` at 0xc90e0 | `(unknown callee) 0x1eed0` | `0x9` | `0x10a974` | `0xdf` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8ec0 | `[0x284580] != 0x0` | `[0x284580] == 0x0` |
| `bne` at 0xc8f68 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `beq` at 0xc8fcc | `[[[sp-36]]+12] == 0x0` | `[[[sp-36]]+12] != 0x0` |
| `bge` at 0xc8ff0 | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `blt` at 0xc9038 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc906c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc9084 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc90f8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc9118 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xc9130 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc917c | `r0` | `?` |

## `JChipUpdateConfig`

- `JChipUpdateConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc91a0 to 0xc9420.
  Established. The call to `0xc6d3c` has candidates `[[sp-44]+13]:1`, unknown, `[[sp-44]+20]:1`,
  `[[[sp-44]]]` (`bl` at 0xc9380); its low byte is stored at `[sp-44] +0x15` (`strb` at 0xc9388).
  The final unsigned comparison takes its bypass for `[0x27eb98] <= 0x2` and falls through above
  that bound (`bls` at 0xc9398). The later `0x20f14` call uses `0x10aa3c`, `[[[sp-44]]+4]`,
  `[[sp-44]+12]:1`, `[[sp-44]+13]:1` (`bl` at 0xc93ec). The return reports `0x0`, with configuration
  meanings unknown (`ldmdb` at 0xc9408).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc91b4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc91b8 | load | 4 | `abs 0xc940c` | `-` |
| `ldr` at 0xc91bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc91c0 | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc91d0 | load | 4 | `abs 0xc9410` | `-` |
| `ldr` at 0xc91d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc91dc | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xc91e4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc91e8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc91ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc9200 | load | 4 | `abs 0xc9414` | `-` |
| `str` at 0xc9208 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc920c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9210 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9218 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xc921c | load | 1 | `[sp-48] +0x10` | `-` |
| `ldr` at 0xc9228 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc922c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9244 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9248 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0xc9258 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc925c | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0xc9268 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc9270 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc9274 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc9278 | load | 1 | `[sp-44] +0x14` | `-` |
| `str` at 0xc927c | store | 4 | `sp -0x24` | `[[sp-44]+20]:1` |
| `str` at 0xc9284 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc9288 | load | 4 | `abs 0xc940c` | `-` |
| `ldr` at 0xc928c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9290 | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc92a0 | load | 4 | `abs 0xc9410` | `-` |
| `ldr` at 0xc92a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc92ac | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xc92b4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc92b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc92bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xc92c0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc92c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc92c8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc92cc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc92d0 | load | 1 | `[sp-44] +0xd` | `-` |
| `ldr` at 0xc92dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc92e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc92e4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc92e8 | load | 1 | `[sp-44] +0x14` | `-` |
| `ldr` at 0xc92f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc92fc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xc9300 | load | 1 | `[sp-48] +0x10` | `-` |
| `ldr` at 0xc930c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9324 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9328 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0xc9338 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9340 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc9344 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc934c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc9354 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9358 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc935c | load | 1 | `[sp-44] +0xd` | `-` |
| `ldr` at 0xc9360 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc9364 | load | 1 | `[sp-44] +0x14` | `-` |
| `ldr` at 0xc9368 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc936c | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0xc937c | load | 4 | `[[sp-44]] +0x0` | `-` |
| `strb` at 0xc9388 | store | 1 | `[sp-44] +0x15` | `(ret(0xc6d3c) & 0xff)` |
| `ldr` at 0xc938c | load | 4 | `abs 0xc9418` | `-` |
| `ldr` at 0xc9390 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc939c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc93a0 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0xc93a4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc93a8 | load | 1 | `[sp-44] +0xc` | `-` |
| `ldr` at 0xc93ac | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc93b0 | load | 1 | `[sp-44] +0xd` | `-` |
| `ldr` at 0xc93b4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc93b8 | load | 1 | `[sp-44] +0x14` | `-` |
| `str` at 0xc93bc | store | 4 | `sp -0x44` | `[[sp-44]+20]:1` |
| `ldr` at 0xc93c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc93c4 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0xc93c8 | load | 4 | `[[sp-44]] +0x0` | `-` |
| `str` at 0xc93cc | store | 4 | `sp -0x40` | `[[[sp-44]]]` |
| `ldr` at 0xc93d0 | load | 4 | `sp -0x2c` | `-` |
| `ldrsb` at 0xc93d4 | load | 1 | `[sp-44] +0x15` | `-` |
| `str` at 0xc93d8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc93dc | load | 4 | `abs 0xc941c` | `-` |
| `ldr` at 0xc93e0 | load | 4 | `[[sp-44]] +0x4` | `-` |
| `ldr` at 0xc93f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc93f8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xc9408 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc9408 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9408 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9408 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9380 | `(unknown callee) 0xc6d3c` | `[[sp-44]+13]:1` | `?` | `[[sp-44]+20]:1` | `[[[sp-44]]]` |
| `bl` at 0xc93ec | `(unknown callee) 0x20f14` | `0x10aa3c` | `[[[sp-44]]+4]` | `[[sp-44]+12]:1` | `[[sp-44]+13]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc91c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc9224 | `? != [[sp-48]+16]:1` | `? == [[sp-48]+16]:1` |
| `beq` at 0xc9254 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc9264 | `[[sp-48]+8] != 0x0` | `[[sp-48]+8] == 0x0` |
| `blt` at 0xc9298 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc92d8 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xc92f0 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc9308 | `? != [[sp-48]+16]:1` | `? == [[sp-48]+16]:1` |
| `bne` at 0xc9334 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xc9398 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9408 | `r0` | `0x0` |

## `JChipTargetByName`

- `JChipTargetByName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9420 to 0xc960c.
  Established. The opening call to `0x18788` carries all four entry register candidates (`bl` at
  0xc9438). A later byte store at an unknown target writes `(ret(0x18aa0) & 0xff)` (`strb` at
  0xc9488). Another clears `([sp-32]-1 + [sp-28]) +0x0` (`strb` at 0xc9510). The call to `0xca498`
  uses `[sp-48]` followed by three unknown candidates (`bl` at 0xc95ac); its zero result takes the
  alternate path and nonzero falls through (`beq` at 0xc95b8). Target-name parsing semantics and the
  returned `[sp-52]` interpretation are unknown (`ldmdb` at 0xc9600).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9430 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc9434 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9440 | store | 4 | `sp -0x1c` | `ret(0x18788)` |
| `ldr` at 0xc9444 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9450 | store | 4 | `sp -0x20` | `ret(0x18a1c)` |
| `ldr` at 0xc9454 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9458 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0xc945c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9460 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc9470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9474 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9478 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc9488 | store | 1 | `(unknown)` | `(ret(0x18aa0) & 0xff)` |
| `ldr` at 0xc948c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9494 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc94a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc94a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc94b0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc94cc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc94d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc94dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc94e8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc94f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc94f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9500 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xc9504 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc9510 | store | 1 | `([sp-32]-1 + [sp-28]) +0x0` | `0x0` |
| `str` at 0xc9518 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc951c | load | 4 | `abs 0xc9604` | `-` |
| `ldr` at 0xc9520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9524 | load | 4 | `abs 0x11585c` | `-` |
| `ldr` at 0xc9534 | load | 4 | `abs 0xc9608` | `-` |
| `ldr` at 0xc9538 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9558 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc955c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9570 | load | 4 | `abs 0xc9608` | `-` |
| `ldr` at 0xc9574 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc958c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9594 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc959c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc95a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc95a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc95bc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc95c0 | store | 4 | `sp -0x38` | `[sp-48]` |
| `str` at 0xc95cc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc95d0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc95d4 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0xc95dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc95e4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xc95ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc95f8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc95fc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xc9600 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc9600 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9600 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9600 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9438 | `(unknown callee) 0x18788` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc9448 | `(unknown callee) 0x18a1c` | `[sp-24]` | `?` | `?` | `ret(0x18788)` |
| `bl` at 0xc9480 | `(unknown callee) 0x18aa0` | `?` | `?` | `?` | `?` |
| `bl` at 0xc94b8 | `(unknown callee) 0x18b24` | `?` | `?` | `[sp-28]` | `?` |
| `bl` at 0xc9560 | `(unknown callee) 0x187e8` | `?` | `[sp-28]` | `?` | `?` |
| `bl` at 0xc95a4 | `(unknown callee) 0x18be4` | `[sp-28]` | `0x10a8e8` | `?` | `?` |
| `bl` at 0xc95ac | `(unknown callee) 0xca498` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xc95f0 | `(unknown callee) 0x18be4` | `[sp-28]` | `?` | `?` | `[0x11585c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc9468 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc94c4 | `ret(0x18b24) != 0x0` | `ret(0x18b24) == 0x0` |
| `blt` at 0xc952c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc956c | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |
| `beq` at 0xc95b8 | `ret(0xca498) == 0x0` | `ret(0xca498) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9600 | `r0` | `[sp-52]` |

## `JChipName`

- `JChipName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc960c to 0xc965c.
  Established. The only call targets `0xca498` with all four entry candidates (`bl` at 0xc9624). A
  zero result takes the alternate path and nonzero falls through (`beq` at 0xc9638). The nonzero
  path stores `ret(0xca498)+4` at `sp -0x1c`, while the zero path stores `0x0` there (`str` at
  0xc9644, `str` at 0xc9650). The joined r0 return is unknown (`ldmdb` at 0xc9658). A string-pointer
  interpretation remains unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc961c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc9620 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc962c | store | 4 | `sp -0x18` | `ret(0xca498)` |
| `ldr` at 0xc9630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc963c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9644 | store | 4 | `sp -0x1c` | `ret(0xca498)+4` |
| `str` at 0xc9650 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc9654 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc9658 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9658 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9624 | `(unknown callee) 0xca498` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc9638 | `ret(0xca498) == 0x0` | `ret(0xca498) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9658 | `r0` | `?` |

## `JChipGetChipList`

- `JChipGetChipList`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc965c to 0xc97a8.
  Established. The call to `0x18aac` uses `[sp-32]`, `[sp-28]+50`, `[sp-24]`, `[sp-28]+50` (`bl` at
  0xc9704); its result is stored as a word at `sp -0x20` (`str` at 0xc9708). The later signed
  comparison takes its edge for `[sp-20] >= [0x2845a0]-1` and falls through below that value (`bge`
  at 0xc9744). A call to `0x18ab8` on the fall-through path carries `[sp-36]`, `0x10aa88`,
  `[0x2845a0]-1`, `[sp-20]` (`bl` at 0xc9750). The list contents’ device meaning and merged return
  are unknown (`ldmdb` at 0xc9798).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9670 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xc9678 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xc9680 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xc9688 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc9690 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xc9694 | load | 4 | `abs 0xc979c` | `-` |
| `ldr` at 0xc9698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc969c | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc96ac | load | 4 | `abs 0xc97a0` | `-` |
| `ldr` at 0xc96b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc96b8 | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xc96c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc96c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc96dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc96e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc96f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc96f8 | store | 4 | `sp -0x1c` | `[sp-28]+50` |
| `ldr` at 0xc96fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9700 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9708 | store | 4 | `sp -0x20` | `ret(0x18aac)` |
| `ldr` at 0xc970c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9710 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9718 | store | 4 | `sp -0x24` | `(ret(0x18aac) + [sp-24])` |
| `ldr` at 0xc971c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc9724 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9730 | load | 4 | `abs 0xc979c` | `-` |
| `ldr` at 0xc9734 | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xc973c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9748 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc974c | load | 4 | `abs 0xc97a4` | `-` |
| `ldr` at 0xc9754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9764 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc976c | store | 4 | `sp -0x24` | `([sp-36] + ret(0x18a1c))` |
| `ldr` at 0xc9770 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9774 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc977c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc9780 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9788 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xc9790 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc9798 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9798 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9798 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc96d0 | `(unknown callee) 0x18a1c` | `?` | `0x284580` | `?` | `?` |
| `bl` at 0xc9704 | `(unknown callee) 0x18aac` | `[sp-32]` | `[sp-28]+50` | `[sp-24]` | `[sp-28]+50` |
| `bl` at 0xc972c | `(unknown callee) 0x18cd4` | `[sp-36]` | `[sp-40]+4` | `?` | `[sp-40]+4` |
| `bl` at 0xc9750 | `(unknown callee) 0x18ab8` | `[sp-36]` | `0x10aa88` | `[0x2845a0]-1` | `[sp-20]` |
| `bl` at 0xc9758 | `(unknown callee) 0x18a1c` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc96a4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0xc96ec | `ret(0x18a1c)+1 <= ? (unsigned, 32-bit)` | `ret(0x18a1c)+1 > ? (unsigned, 32-bit)` |
| `bge` at 0xc9744 | `[sp-20] >= [0x2845a0]-1 (signed, 32-bit)` | `[sp-20] < [0x2845a0]-1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9798 | `r0` | `?` |

## `JChipGetType`

- `JChipGetType`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca3ec to 0xca498.
  Established. The loop branch takes its edge for a signed less-than flags predicate with unknown
  operands and falls through otherwise (`blt` at 0xca418). A second branch retains only `Z == 0` as
  its taken predicate (`bne` at 0xca448). Word stores at `sp -0x1c` include an unknown value and
  `0x0` on separate paths (`str` at 0xca468, `str` at 0xca484). No calls are reported. The
  type-selection rule and joined r0 return remain unknown (`ldmdb` at 0xca48c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca3fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca404 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xca408 | load | 4 | `abs 0xca490` | `-` |
| `ldr` at 0xca40c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca410 | load | 4 | `abs 0x11585c` | `-` |
| `ldr` at 0xca420 | load | 4 | `abs 0xca494` | `-` |
| `ldr` at 0xca424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca43c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca44c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca460 | load | 4 | `abs 0xca494` | `-` |
| `str` at 0xca468 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xca470 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca478 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xca484 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xca488 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xca48c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca48c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca48c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xca418 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xca448 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca48c | `r0` | `?` |

## `JChipGetChip`

- `JChipGetChip`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca498 to 0xca57c.
  Established. The body saves entry r0 as a word at `sp -0x14` (`str` at 0xca4a8). A later stack
  byte store at `sp -0x1d` has unknown value (`strb` at 0xca4c8). Two branches retain `Z == 0` as
  their taken predicates and fall through for `not (Z == 0)` (`bne` at 0xca510, `bne` at 0xca534).
  The selected result slot `sp -0x24` receives unknown value or zero on separate paths (`str` at
  0xca54c, `str` at 0xca568). No calls are reported, and the matching criteria remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca4a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xca4ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca4b4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xca4c0 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0xca4c8 | store | 1 | `sp -0x1d` | `?` |
| `str` at 0xca4d0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xca4d4 | load | 4 | `abs 0xca574` | `-` |
| `ldr` at 0xca4d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca4dc | load | 4 | `abs 0x2845a0` | `-` |
| `ldr` at 0xca4ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca4f4 | load | 4 | `abs 0xca578` | `-` |
| `ldr` at 0xca4f8 | load | 4 | `abs 0x284580` | `-` |
| `ldr` at 0xca500 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca504 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca508 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca514 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca51c | load | 4 | `abs 0xca578` | `-` |
| `ldr` at 0xca520 | load | 4 | `abs 0x284580` | `-` |
| `ldrb` at 0xca528 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xca52c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xca538 | load | 4 | `abs 0xca578` | `-` |
| `ldr` at 0xca53c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca544 | load | 4 | `abs 0x284580` | `-` |
| `str` at 0xca54c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xca554 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca55c | store | 4 | `sp -0x18` | `?` |
| `str` at 0xca568 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca56c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xca570 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca570 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca570 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xca4e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xca510 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xca534 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca570 | `r0` | `?` |

## `JChip_MMR_Read`

- `JChip_MMR_Read`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc97a8 to 0xc98ac.
  Established. The first call targets `0xca498` with all four entry candidates (`bl` at 0xc97cc).
  The reached indirect call has unknown target and candidates `[[sp-36]+13]:1`, unknown, `0x5`,
  `[sp-24]` (`mov` at 0xc9868); its result being `0x1` takes the path to word `0x0` at `sp -0x28`,
  while inequality falls through to `0xfffff7c4` there (`beq` at 0xc9874, `str` at 0xc9888,
  `str` at 0xc987c). The physical register-read effect is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc97b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc97bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc97c0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc97c4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc97c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc97d4 | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xc97d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc97e4 | load | 4 | `abs 0xc9894` | `-` |
| `str` at 0xc97e8 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xc97f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9804 | load | 4 | `abs 0xc9898` | `-` |
| `str` at 0xc9808 | store | 4 | `sp -0x28` | `0xfffff7c5` |
| `ldr` at 0xc9810 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9814 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc9818 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc981c | load | 1 | `[sp-36] +0xd` | `-` |
| `ldr` at 0xc9820 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xc9824 | load | 1 | `[sp-36] +0x15` | `-` |
| `ldr` at 0xc9828 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc982c | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xc9830 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9834 | store | 4 | `sp -0x38` | `[sp-32]` |
| `ldr` at 0xc9838 | load | 4 | `abs 0xc989c` | `-` |
| `str` at 0xc983c | store | 4 | `sp -0x34` | `0x27ebb8` |
| `ldr` at 0xc9840 | load | 4 | `abs 0xc98a0` | `-` |
| `str` at 0xc9844 | store | 4 | `sp -0x30` | `0x27ec38` |
| `ldr` at 0xc9848 | load | 4 | `abs 0xc98a4` | `-` |
| `str` at 0xc984c | store | 4 | `sp -0x2c` | `0x2845a8` |
| `ldr` at 0xc9850 | load | 4 | `[[sp-36]] +0x10` | `-` |
| `ldr` at 0xc9860 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9878 | load | 4 | `abs 0xc98a8` | `-` |
| `str` at 0xc987c | store | 4 | `sp -0x28` | `0xfffff7c4` |
| `str` at 0xc9888 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc988c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xc9890 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9890 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9890 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc97cc | `(unknown callee) 0xca498` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc97f4 | `(unknown callee) 0xc8d88` | `ret(0xca498)` | `?` | `?` | `ret(0xca498)` |
| `mov` at 0xc9868 | `(unknown callee) (unknown target)` | `[[sp-36]+13]:1` | `?` | `0x5` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc97e0 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `beq` at 0xc9800 | `ret(0xc8d88) == 0x0` | `ret(0xc8d88) != 0x0` |
| `beq` at 0xc9874 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9890 | `r0` | `?` |

## `JChip_MMR_Write`

- `JChip_MMR_Write`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc98ac to 0xc99b0.
  Established. The indirect call has unknown target and candidates `[[sp-36]+13]:1`, unknown, `0x5`,
  `[sp-24]` (`mov` at 0xc996c). Before it, stack words at `sp -0x34`, `sp -0x30`, `sp -0x2c` receive
  `0x27ebb8`, `0x27ec38`, `0x2845a8` (`str` at 0xc9940, `str` at 0xc9948, `str` at 0xc9950).
  Equality of the indirect result to `0x1` takes the path to word `0x0` at `sp -0x28`, while
  inequality falls through to `0xfffff7c4` (`beq` at 0xc9978, `str` at 0xc998c, `str` at
  0xc9980). The physical write operation and merged return are unknown (`ldmdb` at 0xc9994).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc98bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc98c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc98c4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc98c8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc98cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc98d8 | store | 4 | `sp -0x24` | `ret(0xca498)` |
| `ldr` at 0xc98dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc98e8 | load | 4 | `abs 0xc9998` | `-` |
| `str` at 0xc98ec | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xc98f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9908 | load | 4 | `abs 0xc999c` | `-` |
| `str` at 0xc990c | store | 4 | `sp -0x28` | `0xfffff7c5` |
| `ldr` at 0xc9914 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9918 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc991c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9920 | load | 1 | `[sp-36] +0xd` | `-` |
| `ldr` at 0xc9924 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xc9928 | load | 1 | `[sp-36] +0x15` | `-` |
| `ldr` at 0xc992c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9930 | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xc9934 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9938 | store | 4 | `sp -0x38` | `[sp-32]` |
| `ldr` at 0xc993c | load | 4 | `abs 0xc99a0` | `-` |
| `str` at 0xc9940 | store | 4 | `sp -0x34` | `0x27ebb8` |
| `ldr` at 0xc9944 | load | 4 | `abs 0xc99a4` | `-` |
| `str` at 0xc9948 | store | 4 | `sp -0x30` | `0x27ec38` |
| `ldr` at 0xc994c | load | 4 | `abs 0xc99a8` | `-` |
| `str` at 0xc9950 | store | 4 | `sp -0x2c` | `0x2845a8` |
| `ldr` at 0xc9954 | load | 4 | `[[sp-36]] +0x14` | `-` |
| `ldr` at 0xc9964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc997c | load | 4 | `abs 0xc99ac` | `-` |
| `str` at 0xc9980 | store | 4 | `sp -0x28` | `0xfffff7c4` |
| `str` at 0xc998c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc9990 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xc9994 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9994 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc98d0 | `(unknown callee) 0xca498` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc98f8 | `(unknown callee) 0xc8d88` | `ret(0xca498)` | `?` | `?` | `ret(0xca498)` |
| `mov` at 0xc996c | `(unknown callee) (unknown target)` | `[[sp-36]+13]:1` | `?` | `0x5` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc98e4 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `beq` at 0xc9904 | `ret(0xc8d88) == 0x0` | `ret(0xc8d88) != 0x0` |
| `beq` at 0xc9978 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9994 | `r0` | `?` |

## `JChip_MMR_SC_f`

- `JChip_MMR_SC_f`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc99b0 to 0xc9f88.
  Established. The calls include `0xc98ac` with `[sp-28]`, `([sp-72] + [sp-76])`, `[sp-64]`,
  `[sp-68]` and `0xc97a8` with `[sp-28]`, unknown, `sp-64`, `sp-68` on separate paths (`bl` at
  0xc9d6c, `bl` at 0xc9e7c). Each zero result takes its continuation while nonzero falls through
  to a word store with unknown value at `[sp-20] +0x0` (`beq` at 0xc9d78, `str` at 0xc9d90,
  `beq` at 0xc9e88, `str` at 0xc9ea0). Later word stores at unknown targets write `[sp-64]` and
  `[sp-68]` (`str` at 0xc9f1c, `str` at 0xc9f4c). The physical register transaction and merged
  return remain unknown (`ldmdb` at 0xc9f6c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc99c0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc99c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc99d0 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xc99e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc99f0 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xc99fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc9a00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a04 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc9a10 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc9a14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9a1c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc9a20 | load | 4 | `abs 0xc9f70` | `-` |
| `ldr` at 0xc9a24 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc9a34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a3c | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xc9a4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a54 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xc9a60 | load | 4 | `abs 0xc9f74` | `-` |
| `ldr` at 0xc9a70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9a88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9aac | store | 4 | `sp -0x1c` | `ret(0xca63c)` |
| `ldr` at 0xc9ab0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9ac0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xc9ac8 | load | 4 | `abs 0xc9f70` | `-` |
| `ldr` at 0xc9acc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc9ad8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9ae4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9ae8 | store | 4 | `sp -0x70` | `[sp-32]` |
| `ldr` at 0xc9aec | load | 4 | `abs 0xc9f78` | `-` |
| `ldr` at 0xc9af4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9af8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9b00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9b10 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc9b14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9b1c | store | 4 | `sp -0x30` | `?` |
| `str` at 0xc9b24 | store | 4 | `sp -0x34` | `0x2` |
| `ldr` at 0xc9b28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9b3c | store | 4 | `sp -0x54` | `0x4` |
| `str` at 0xc9b48 | store | 4 | `sp -0x54` | `0x2` |
| `ldr` at 0xc9b4c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xc9b50 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xc9b54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9b64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9b68 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc9b74 | store | 4 | `sp -0x58` | `ret(0xd569c)` |
| `ldr` at 0xc9b7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9b80 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc9b88 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xc9b8c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xc9b90 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc9b9c | store | 4 | `sp -0x58` | `ret(0xd569c)` |
| `ldr` at 0xc9ba0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xc9ba4 | store | 4 | `sp -0x3c` | `ret(0xd569c)` |
| `str` at 0xc9bb0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc9bb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9bc8 | store | 4 | `sp -0x5c` | `0x2` |
| `str` at 0xc9bd4 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xc9bd8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xc9bdc | store | 4 | `sp -0x30` | `[sp-92]` |
| `str` at 0xc9be4 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xc9bec | store | 4 | `sp -0x38` | `0x2` |
| `ldr` at 0xc9bf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9c00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9c04 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc9c10 | store | 4 | `sp -0x60` | `ret(0xd569c)` |
| `str` at 0xc9c1c | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xc9c20 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc9c24 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc9c28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc9c38 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc9c3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9c40 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc9c50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9c54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9c5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9c78 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9c7c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xc9c80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9c90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9c98 | store | 4 | `sp -0x64` | `?` |
| `str` at 0xc9ca4 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xc9ca8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc9cac | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xc9cb0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9cbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc9ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9cd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9cdc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc9ce0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9ce4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9cec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc9cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9d08 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9d0c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc9d10 | load | 4 | `abs 0xc9f70` | `-` |
| `ldr` at 0xc9d14 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc9d20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9d2c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc9d30 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xc9d34 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc9d38 | store | 4 | `sp -0x6c` | `[sp-68]` |
| `ldr` at 0xc9d3c | load | 4 | `abs 0xc9f7c` | `-` |
| `ldr` at 0xc9d44 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc9d48 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc9d50 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc9d54 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc9d5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9d64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc9d68 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc9d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9d80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9d84 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc9d90 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xc9d98 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xc9da0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9da8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xc9db4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc9db8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9dbc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc9dcc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9dd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9dd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9df4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9df8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xc9dfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9e0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9e14 | store | 4 | `sp -0x68` | `?` |
| `str` at 0xc9e20 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xc9e24 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xc9e28 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xc9e2c | load | 4 | `abs 0xc9f70` | `-` |
| `ldr` at 0xc9e30 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc9e3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9e48 | load | 4 | `abs 0xc9f80` | `-` |
| `ldr` at 0xc9e50 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc9e54 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc9e5c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc9e60 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc9e70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9e94 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc9ea0 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xc9ea8 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xc9eb0 | load | 4 | `abs 0xc9f70` | `-` |
| `ldr` at 0xc9eb4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc9ec0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9ecc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc9ed0 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xc9ed4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc9ed8 | store | 4 | `sp -0x6c` | `[sp-68]` |
| `ldr` at 0xc9edc | load | 4 | `abs 0xc9f84` | `-` |
| `ldr` at 0xc9ee4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc9ee8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc9ef0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9ef4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9efc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc9f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9f18 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc9f1c | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0xc9f20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9f24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc9f2c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc9f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9f48 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc9f4c | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xc9f50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9f58 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xc9f64 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xc9f68 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xc9f6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9f6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9f6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9a68 | `(unknown callee) 0x20f14` | `0x10aa8c` | `?` | `?` | `?` |
| `bl` at 0xc9aa4 | `(unknown callee) 0xca63c` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xc9adc | `(unknown callee) 0xc960c` | `[sp-28]` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xc9afc | `(unknown callee) 0x20f14` | `0x10aaac` | `ret(0xc960c)` | `[sp-28]` | `[sp-36]` |
| `bl` at 0xc9b6c | `(unknown callee) 0xd569c` | `[sp-32]` | `[sp-84]` | `?` | `?` |
| `bl` at 0xc9b94 | `(unknown callee) 0xd569c` | `[sp-88]` | `[sp-56]` | `0x2` | `?` |
| `bl` at 0xc9c08 | `(unknown callee) 0xd569c` | `[sp-32]` | `0x2` | `?` | `?` |
| `bl` at 0xc9d24 | `(unknown callee) 0xc960c` | `[sp-28]` | `0x14` | `?` | `[0x27eb98]` |
| `bl` at 0xc9d4c | `(unknown callee) 0x20f14` | `0x10aae4` | `ret(0xc960c)` | `[sp-72]` | `[sp-76]` |
| `bl` at 0xc9d6c | `(unknown callee) 0xc98ac` | `[sp-28]` | `([sp-72] + [sp-76])` | `[sp-64]` | `[sp-68]` |
| `bl` at 0xc9e40 | `(unknown callee) 0xc960c` | `[sp-28]` | `0x14` | `?` | `[0x27eb98]` |
| `bl` at 0xc9e58 | `(unknown callee) 0x20f14` | `0x10ab1c` | `ret(0xc960c)` | `[sp-76]` | `[sp-72]` |
| `bl` at 0xc9e7c | `(unknown callee) 0xc97a8` | `[sp-28]` | `?` | `sp-64` | `sp-68` |
| `bl` at 0xc9ec4 | `(unknown callee) 0xc960c` | `[sp-28]` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xc9eec | `(unknown callee) 0x20f14` | `0x10ab48` | `ret(0xc960c)` | `[sp-76]` | `[sp-72]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc9a2c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xc9ab8 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xc9ad4 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xc9b08 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xc9b34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc9b60 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc9bc0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc9bfc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc9c30 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `blt` at 0xc9c48 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc9c8c | `? == 0x0` | `? != 0x0` |
| `bls` at 0xc9d1c | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xc9d78 | `ret(0xc98ac) == 0x0` | `ret(0xc98ac) != 0x0` |
| `blt` at 0xc9dc4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc9e08 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xc9e38 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xc9e88 | `ret(0xc97a8) == 0x0` | `ret(0xc97a8) != 0x0` |
| `bls` at 0xc9ebc | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9f6c | `r0` | `?` |

## `JChip_MMR_SC_stub`

- `JChip_MMR_SC_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7ad4 to 0xc7af4.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0xc7ae4). The following moves
  select `0x0` for r0, and the return reports it (`mov` at 0xc7ae8, `mov` at 0xc7aec, `ldmdb` at
  0xc7af0). No calls or conditional branches are reported. The symbol’s register-operation meaning
  is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7ae4 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xc7af0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7af0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7af0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7af0 | `r0` | `0x0` |

## `JChip_ir_load`

- `JChip_ir_load`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb4cc to 0xcb8f4.
  Established. Absolute word stores clear `0x27ebb4` and later write `[0x27ebb4]+1` there (`str` at
  0xcb69c, `str` at 0xcb6e4). The loop test takes its edge for `[0x27ebb4] < [sp-32]` unsigned and
  falls through otherwise (`blo` at 0xcb6b0). A word store writes `[[sp-44]+4]:2` at `[sp-52] +0x0`
  (`str` at 0xcb6f8). The call to `0xbc8b4` has candidates `0x0`, `0x1`, `0x1`, `0x0` (`bl` at
  0xcb7f4). Instruction-register loading and the joined return are unproven (`ldmdb` at 0xcb8a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb4dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcb4e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcb4e4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcb4e8 | load | 4 | `abs 0xcb8a8` | `-` |
| `ldr` at 0xcb4ec | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb4f8 | load | 4 | `abs 0xcb8ac` | `-` |
| `ldr` at 0xcb4fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb50c | load | 4 | `abs 0xcb8b0` | `-` |
| `str` at 0xcb510 | store | 4 | `sp -0x30` | `0x27ebb8` |
| `ldr` at 0xcb514 | load | 4 | `abs 0xcb8b4` | `-` |
| `str` at 0xcb518 | store | 4 | `sp -0x34` | `0x27ec38` |
| `ldr` at 0xcb51c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcb528 | store | 4 | `sp -0x28` | `ret(0xca498)` |
| `ldr` at 0xcb52c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb538 | load | 4 | `abs 0xcb8b8` | `-` |
| `ldr` at 0xcb53c | load | 4 | `abs 0xcb8bc` | `-` |
| `ldr` at 0xcb540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcb548 | load | 4 | `abs 0xcb8b8` | `-` |
| `str` at 0xcb54c | store | 4 | `sp -0x48` | `0x2845a8` |
| `ldr` at 0xcb554 | load | 4 | `abs 0xcb8c0` | `-` |
| `ldr` at 0xcb558 | load | 4 | `abs 0xcb8c4` | `-` |
| `ldr` at 0xcb564 | load | 4 | `abs 0xcb8c8` | `-` |
| `str` at 0xcb568 | store | 4 | `sp -0x38` | `0xfffff7c6` |
| `ldr` at 0xcb570 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb584 | load | 4 | `abs 0xcb8cc` | `-` |
| `str` at 0xcb588 | store | 4 | `sp -0x38` | `0xfffff7c5` |
| `ldr` at 0xcb590 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xcb594 | load | 1 | `[sp-40] +0x15` | `-` |
| `str` at 0xcb598 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcb59c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb5a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb5a8 | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xcb5b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb5b4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcb5b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcb5cc | load | 4 | `abs 0xcb8b8` | `-` |
| `ldr` at 0xcb5e0 | load | 4 | `abs 0xcb8b8` | `-` |
| `str` at 0xcb5e4 | store | 4 | `sp -0x48` | `0x2845a8` |
| `ldr` at 0xcb5ec | load | 4 | `abs 0xcb8c0` | `-` |
| `ldr` at 0xcb5f0 | load | 4 | `abs 0xcb8d0` | `-` |
| `ldr` at 0xcb5fc | load | 4 | `abs 0xcb8d4` | `-` |
| `str` at 0xcb600 | store | 4 | `sp -0x38` | `0xfffff7ca` |
| `ldr` at 0xcb608 | load | 4 | `abs 0xcb8a8` | `-` |
| `ldr` at 0xcb60c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb618 | load | 4 | `abs 0xcb8d8` | `-` |
| `ldr` at 0xcb620 | load | 4 | `abs 0xcb8dc` | `-` |
| `ldr` at 0xcb624 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb628 | load | 1 | `[sp-40] +0xd` | `-` |
| `ldr` at 0xcb648 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb65c | load | 4 | `abs 0xcb8dc` | `-` |
| `ldr` at 0xcb660 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb664 | load | 1 | `[sp-40] +0xd` | `-` |
| `ldr` at 0xcb684 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb690 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb694 | load | 4 | `abs 0xcb8e0` | `-` |
| `str` at 0xcb69c | store | 4 | `abs 0x27ebb4` | `0x0` |
| `ldr` at 0xcb6a0 | load | 4 | `abs 0xcb8e0` | `-` |
| `ldr` at 0xcb6a4 | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcb6a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb6b8 | load | 4 | `abs 0xcb8e0` | `-` |
| `ldr` at 0xcb6bc | load | 4 | `abs 0x27ebb4` | `-` |
| `ldr` at 0xcb6c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xcb6d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcb6d4 | load | 4 | `abs 0xcb8e0` | `-` |
| `ldr` at 0xcb6d8 | load | 4 | `abs 0xcb8e0` | `-` |
| `ldr` at 0xcb6dc | load | 4 | `abs 0x27ebb4` | `-` |
| `str` at 0xcb6e4 | store | 4 | `abs 0x27ebb4` | `[0x27ebb4]+1` |
| `ldr` at 0xcb6ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb6f0 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcb6f4 | load | 2 | `[sp-44] +0x4` | `-` |
| `str` at 0xcb6f8 | store | 4 | `[sp-52] +0x0` | `[[sp-44]+4]:2` |
| `ldr` at 0xcb704 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb70c | load | 4 | `abs 0xcb8dc` | `-` |
| `ldr` at 0xcb710 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb714 | load | 1 | `[sp-40] +0xd` | `-` |
| `ldr` at 0xcb734 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb73c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcb744 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xcb748 | load | 4 | `abs 0xcb8dc` | `-` |
| `ldr` at 0xcb74c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb750 | load | 1 | `[sp-40] +0xd` | `-` |
| `ldr` at 0xcb770 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb778 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcb780 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcb784 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xcb78c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcb794 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb79c | load | 4 | `abs 0xcb8dc` | `-` |
| `ldr` at 0xcb7a0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb7a4 | load | 1 | `[sp-40] +0xd` | `-` |
| `ldr` at 0xcb7c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcb7c8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xcb7cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xcb7d0 | store | 4 | `sp -0x44` | `[sp-48]` |
| `ldr` at 0xcb7d4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcb7d8 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0xcb7dc | load | 4 | `abs 0xcb8b8` | `-` |
| `str` at 0xcb7e0 | store | 4 | `sp -0x3c` | `0x2845a8` |
| `ldr` at 0xcb804 | load | 4 | `abs 0xcb8b8` | `-` |
| `str` at 0xcb808 | store | 4 | `sp -0x48` | `0x2845a8` |
| `ldr` at 0xcb810 | load | 4 | `abs 0xcb8c0` | `-` |
| `ldr` at 0xcb814 | load | 4 | `abs 0xcb8e4` | `-` |
| `ldr` at 0xcb820 | load | 4 | `abs 0xcb8d4` | `-` |
| `str` at 0xcb824 | store | 4 | `sp -0x38` | `0xfffff7ca` |
| `ldr` at 0xcb82c | load | 4 | `abs 0xcb8a8` | `-` |
| `ldr` at 0xcb830 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcb83c | load | 4 | `abs 0xcb8dc` | `-` |
| `ldr` at 0xcb840 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xcb844 | load | 1 | `[sp-40] +0xd` | `-` |
| `ldr` at 0xcb864 | load | 4 | `abs 0xcb8e8` | `-` |
| `ldr` at 0xcb868 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcb874 | load | 4 | `abs 0xcb8ec` | `-` |
| `ldr` at 0xcb878 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb87c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcb888 | load | 4 | `abs 0xcb8f0` | `-` |
| `ldr` at 0xcb88c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcb890 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xcb89c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xcb8a0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xcb8a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb8a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb8a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb508 | `(unknown callee) 0x20f14` | `0x10af1c` | `r0` | `r1` | `r2` |
| `bl` at 0xcb520 | `(unknown callee) 0xca498` | `?` | `?` | `?` | `0x27ec38` |
| `bl` at 0xcb544 | `(unknown callee) 0x18c74` | `0x2845a8` | `0x10af50` | `[sp-20]` | `ret(0xca498)` |
| `bl` at 0xcb560 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x402` | `0x0` |
| `bl` at 0xcb574 | `(unknown callee) 0xc8d88` | `ret(0xca498)` | `?` | `?` | `ret(0xca498)` |
| `bl` at 0xcb5d0 | `(unknown callee) 0xbc3c4` | `0x1` | `0x0` | `0x2845a8` | `[sp-24]` |
| `bl` at 0xcb5f8 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x412` | `0x0` |
| `bl` at 0xcb61c | `(unknown callee) 0x20f14` | `0x10af80` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xcb708 | `(unknown callee) 0xc1c9c` | `0x0` | `0x20` | `[sp-52]` | `[[sp-44]+4]:2` |
| `bl` at 0xcb798 | `(unknown callee) 0xc1ab4` | `0x0` | `[sp-52]` | `?` | `[sp-48]` |
| `bl` at 0xcb7f4 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcb81c | `(unknown callee) 0x1eed0` | `0x3` | `0x10a974` | `0x427` | `0x0` |
| `bl` at 0xcb86c | `(unknown callee) 0x20f14` | `0x10afb4` | `?` | `[[sp-40]+13]:1` | `?` |
| `bl` at 0xcb880 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b008` | `[sp-32]` | `[sp-48]` |
| `bl` at 0xcb894 | `(unknown callee) 0xd2e88` | `0x0` | `0x10b014` | `[sp-32]` | `[sp-52]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xcb4f4 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcb534 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `beq` at 0xcb580 | `ret(0xc8d88) == 0x0` | `ret(0xc8d88) != 0x0` |
| `beq` at 0xcb5c0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0xcb5dc | `ret(0xbc3c4) != 0x0` | `ret(0xbc3c4) == 0x0` |
| `bls` at 0xcb614 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |
| `blo` at 0xcb6b0 | `[0x27ebb4] < [sp-32] (unsigned, 32-bit)` | `[0x27ebb4] >= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xcb800 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcb838 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb8a4 | `r0` | `?` |

## `JChip_read_dmin_dmax`

- `JChip_read_dmin_dmax`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0e3c to 0xd10c8.
  Established. The call to `0xcb020` uses `[sp-20]`, `[sp-36]`, `0x1`, `sp-40` (`bl` at 0xd0fc0).
  Later calls to `0xcab20` carry those first, second and fourth candidates with third values `0x3`
  and `0x4` (`bl` at 0xd1054, `bl` at 0xd1074). Word stores copy `[sp-40]` to `[sp-32] +0x0` and
  `[sp] +0x0` (`str` at 0xd1060, `str` at 0xd1080). The branch after `0xcb8f4` takes its
  continuation for a signed nonnegative result and falls through on negative (`bge` at 0xd0fe8). The
  output words’ delay meanings and the return value remain unknown (`ldmdb` at 0xd10ac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0e4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd0e50 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd0e54 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd0e58 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd0e5c | load | 4 | `abs 0xd10b0` | `-` |
| `ldr` at 0xd0e60 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xd0e6c | load | 4 | `abs 0xd10b4` | `-` |
| `ldr` at 0xd0e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0e78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0e80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd0e88 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd0e8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0e98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0ea4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0eb4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0ec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0ed0 | load | 4 | `abs 0xd10b8` | `-` |
| `str` at 0xd0ed4 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xd0edc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd0ee4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0eec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0ef8 | load | 4 | `abs 0xd10b8` | `-` |
| `str` at 0xd0efc | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xd0f04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd0f0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0f14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0f20 | load | 4 | `abs 0xd10b8` | `-` |
| `str` at 0xd0f24 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xd0f2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd0f34 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0f3c | load | 4 | `abs 0xd10b8` | `-` |
| `str` at 0xd0f40 | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xd0f48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd0f54 | store | 4 | `sp -0x2c` | `ret(0xca498)` |
| `ldr` at 0xd0f58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd0f64 | load | 4 | `abs 0xd10bc` | `-` |
| `str` at 0xd0f68 | store | 4 | `sp -0x34` | `0xfffff7c6` |
| `ldr` at 0xd0f70 | load | 4 | `abs 0xd10b0` | `-` |
| `ldr` at 0xd0f74 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xd0f80 | load | 4 | `abs 0xd10c0` | `-` |
| `ldr` at 0xd0f88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd0f8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0f94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd0f9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd0fa4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd0fac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd0fb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0fb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0fc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0fcc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0fdc | store | 4 | `sp -0x30` | `ret(0xcb8f4)` |
| `ldr` at 0xd0fe0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd0fec | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd0ff0 | store | 4 | `sp -0x34` | `ret(0xcb8f4)` |
| `ldr` at 0xd0ff8 | load | 4 | `abs 0xd10b0` | `-` |
| `ldr` at 0xd0ffc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xd1008 | load | 4 | `abs 0xd10c4` | `-` |
| `ldr` at 0xd1010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1018 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd1028 | store | 4 | `sp -0x30` | `ret(0xcb8f4)` |
| `ldr` at 0xd102c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd1038 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd103c | store | 4 | `sp -0x34` | `ret(0xcb8f4)` |
| `ldr` at 0xd1048 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd104c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1058 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd105c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1060 | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `ldr` at 0xd1068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd106c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1078 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xd107c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1080 | store | 4 | `[sp] +0x0` | `[sp-40]` |
| `ldr` at 0xd1084 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1088 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1090 | load | 4 | `[sp-44] +0x1c` | `-` |
| `ldr` at 0xd1098 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd10a4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd10a8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd10ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd10ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd10ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd0e7c | `(unknown callee) 0x20f14` | `0x10cf4c` | `r0` | `r1` | `r2` |
| `bl` at 0xd0f4c | `(unknown callee) 0xca498` | `?` | `?` | `?` | `?` |
| `bl` at 0xd0f84 | `(unknown callee) 0x20f14` | `0x10cf84` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xd0fa0 | `(unknown callee) 0xcf520` | `?` | `?` | `?` | `?` |
| `bl` at 0xd0fc0 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-36]` | `0x1` | `sp-40` |
| `bl` at 0xd0fd4 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x3` | `[sp-36]` | `0x0` |
| `bl` at 0xd100c | `(unknown callee) 0x20f14` | `0x10cfa8` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0xd1020 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0xc` | `[sp-36]` | `0x0` |
| `bl` at 0xd1054 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-36]` | `0x3` | `sp-40` |
| `bl` at 0xd1074 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-36]` | `0x4` | `sp-40` |
| `bl` at 0xd109c | `(unknown callee) 0xcf520` | `?` | `[sp-44]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd0e68 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xd0e94 | `[sp-56] == 0x4` | `[sp-56] != 0x4` |
| `bgt` at 0xd0ea0 | `[sp-56] > 0x4 (signed, 32-bit)` | `[sp-56] <= 0x4 (signed, 32-bit)` |
| `beq` at 0xd0eac | `[sp-56] == 0x2` | `[sp-56] != 0x2` |
| `beq` at 0xd0ebc | `[sp-56] == 0x6` | `[sp-56] != 0x6` |
| `bls` at 0xd0ecc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd0ef4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd0f1c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd0f60 | `ret(0xca498) != 0x0` | `ret(0xca498) == 0x0` |
| `bls` at 0xd0f7c | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0xd0fe8 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bls` at 0xd1004 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0xd1034 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd10ac | `r0` | `?` |
