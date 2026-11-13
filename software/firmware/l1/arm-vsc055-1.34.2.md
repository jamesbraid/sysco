# ARM L1 VSC055 entry paths in 1.34.2

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
No unresolved computed transfers were reported for these routines.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/vsc055-facts.txt) retain all tool sections.

## `vsc055_Init`

- `vsc055_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8c38 to 0xb8d00.
  Established. The word read at 0x27eb74 decides `beq`, whose taken zero path skips the first call
  to 0x1eed0 (`ldr` at 0xb8c54, `beq` at 0xb8c5c, `bl` at 0xb8c78). The call to 0x18920 supplies the
  word stored at 0x27eb78 (`bl` at 0xb8c84, `str` at 0xb8c8c). Later stores write `[sp-24]` at
  0x27eb74 and `[sp-28]` at 0x2841c0 (`str` at 0xb8cd4, `str` at 0xb8ce0). The allocation and device
  meanings of these globals remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8c48 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb8c4c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb8c50 | load | 4 | `abs 0xb8ce8` | `-` |
| `ldr` at 0xb8c54 | load | 4 | `abs 0x27eb74` | `-` |
| `ldr` at 0xb8c60 | load | 4 | `abs 0xb8cec` | `-` |
| `str` at 0xb8c64 | store | 4 | `sp -0x20` | `0x1094dc` |
| `ldr` at 0xb8c6c | load | 4 | `abs 0xb8cf0` | `-` |
| `ldr` at 0xb8c7c | load | 4 | `abs 0xb8cf4` | `-` |
| `ldr` at 0xb8c80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8c8c | store | 4 | `abs 0x27eb78` | `ret(0x18920)` |
| `ldr` at 0xb8c90 | load | 4 | `abs 0x27eb78` | `-` |
| `ldr` at 0xb8c9c | load | 4 | `abs 0xb8cf8` | `-` |
| `str` at 0xb8ca0 | store | 4 | `sp -0x20` | `0x109514` |
| `ldr` at 0xb8ca8 | load | 4 | `abs 0xb8cf0` | `-` |
| `ldr` at 0xb8cb8 | load | 4 | `abs 0xb8cf4` | `-` |
| `ldr` at 0xb8cbc | load | 4 | `abs 0x27eb78` | `-` |
| `ldr` at 0xb8cc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8ccc | load | 4 | `abs 0xb8ce8` | `-` |
| `ldr` at 0xb8cd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8cd4 | store | 4 | `abs 0x27eb74` | `[sp-24]` |
| `ldr` at 0xb8cd8 | load | 4 | `abs 0xb8cfc` | `-` |
| `ldr` at 0xb8cdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8ce0 | store | 4 | `abs 0x2841c0` | `[sp-28]` |
| `ldmdb` at 0xb8ce4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb8ce4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8ce4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8ce4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8c78 | `(unknown callee) 0x1eed0` | `0x9` | `0x1094f8` | `0x3d` | `0x0` |
| `bl` at 0xb8c84 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xb8cb4 | `(unknown callee) 0x1eed0` | `0x9` | `0x1094f8` | `0x44` | `0x0` |
| `bl` at 0xb8cc8 | `(unknown callee) 0x18c20` | `[0x27eb78]` | `0x1` | `[sp-28]` | `0x27eb78` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8c5c | `[0x27eb74] == 0x0` | `[0x27eb74] != 0x0` |
| `bne` at 0xb8c98 | `[0x27eb78] != 0x0` | `[0x27eb78] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8ce4 | `r0` | `ret(0x18c20)` |

## `vsc055_Enable`

- `vsc055_Enable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8d00 to 0xb8e24.
  Established. `bge` takes the path to 0x1eed0 for `(r0 & 0xff) >= [0x2841c0]` signed (`bge` at
  0xb8d28, `bl` at 0xb8e08). Fall-through calls 0xb7858 with r0 candidate sp-28 (`bl` at 0xb8d4c).
  Two `bne` sites take unequal-byte paths past 0xb71dc (`bne` at 0xb8d88, `bne` at 0xb8da0, `bl` at
  0xb8dac). After the loop, a byte store writes 0x1 at `([sp-17]:1 + [0x27eb78]) +0x0` (`strb` at
  0xb8de0). The loop's `blt` predicate has unknown operands (`blt` at 0xb8d6c). The physical enable
  effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb8d14 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb8d18 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8d1c | load | 4 | `abs 0xb8e10` | `-` |
| `ldr` at 0xb8d20 | load | 4 | `abs 0x2841c0` | `-` |
| `ldr` at 0xb8d2c | load | 4 | `abs 0xb8e14` | `-` |
| `ldrb` at 0xb8d30 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8d38 | load | 4 | `abs 0x27eb74` | `-` |
| `str` at 0xb8d40 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xb8d54 | store | 4 | `sp -0x20` | `ret(0xb7858)` |
| `strb` at 0xb8d5c | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xb8d60 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xb8d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8d74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8d78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8d7c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0xb8d80 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb8d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8d90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8d94 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xb8d98 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldrb` at 0xb8da4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xb8db0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8db8 | store | 4 | `sp -0x1c` | `[sp-28]+16` |
| `ldrb` at 0xb8dbc | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xb8dc4 | store | 1 | `sp -0x21` | `?` |
| `ldrb` at 0xb8dcc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8dd0 | load | 4 | `abs 0xb8e18` | `-` |
| `ldr` at 0xb8dd4 | load | 4 | `abs 0x27eb78` | `-` |
| `strb` at 0xb8de0 | store | 1 | `([sp-17]:1 + [0x27eb78]) +0x0` | `0x1` |
| `ldr` at 0xb8de8 | load | 4 | `abs 0xb8e1c` | `-` |
| `str` at 0xb8dec | store | 4 | `sp -0x2c` | `0x109534` |
| `ldrb` at 0xb8df0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8df4 | store | 4 | `sp -0x28` | `(r0 & 0xff)` |
| `ldr` at 0xb8dfc | load | 4 | `abs 0xb8e20` | `-` |
| `ldmdb` at 0xb8e0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8e0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8e0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8d4c | `(unknown callee) 0xb7858` | `sp-28` | `0x27eb74` | `?` | `sp-28` |
| `bl` at 0xb8dac | `(unknown callee) 0xb71dc` | `?` | `[[sp-28]+2]:1` | `[sp-24]` | `?` |
| `bl` at 0xb8e08 | `(unknown callee) 0x1eed0` | `0x6` | `0x1094f8` | `0x6b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8d28 | `(r0 & 0xff) >= [0x2841c0] (signed, 32-bit)` | `(r0 & 0xff) < [0x2841c0] (signed, 32-bit)` |
| `blt` at 0xb8d6c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb8d88 | `[[sp-28]+1]:1 != [[sp-24]]:1` | `[[sp-28]+1]:1 == [[sp-24]]:1` |
| `bne` at 0xb8da0 | `[[sp-28]+2]:1 != [[sp-24]+1]:1` | `[[sp-28]+2]:1 == [[sp-24]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8e0c | `r0` | `?` |

## `vsc055_Disable`

- `vsc055_Disable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8e24 to 0xb8f44.
  Established. The signed `bge` takes the call path when `(r0 & 0xff) >= [0x2841c0]` (`bge` at
  0xb8e4c, `bl` at 0xb8f28); its fall-through first writes byte 0x0 at `((r0 & 0xff) + [0x27eb78])
  +0x0` (`strb` at 0xb8e64). It then calls 0xb7858 with first candidate sp-28 (`bl` at 0xb8e88).
  Within the loop, both `bne` unequal-byte paths skip 0xb726c (`bne` at 0xb8ec4, `bne` at 0xb8edc,
  `bl` at 0xb8ee8). The target's physical disable behavior is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb8e38 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb8e3c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8e40 | load | 4 | `abs 0xb8f30` | `-` |
| `ldr` at 0xb8e44 | load | 4 | `abs 0x2841c0` | `-` |
| `ldrb` at 0xb8e50 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8e54 | load | 4 | `abs 0xb8f34` | `-` |
| `ldr` at 0xb8e58 | load | 4 | `abs 0x27eb78` | `-` |
| `strb` at 0xb8e64 | store | 1 | `((r0 & 0xff) + [0x27eb78]) +0x0` | `0x0` |
| `ldr` at 0xb8e68 | load | 4 | `abs 0xb8f38` | `-` |
| `ldrb` at 0xb8e6c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8e74 | load | 4 | `abs 0x27eb74` | `-` |
| `str` at 0xb8e7c | store | 4 | `sp -0x18` | `?` |
| `str` at 0xb8e90 | store | 4 | `sp -0x20` | `ret(0xb7858)` |
| `strb` at 0xb8e98 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xb8e9c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xb8ea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8eb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8eb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8eb8 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0xb8ebc | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb8ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8ecc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8ed0 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xb8ed4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldrb` at 0xb8ee0 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xb8eec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8ef4 | store | 4 | `sp -0x1c` | `[sp-28]+16` |
| `ldrb` at 0xb8ef8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xb8f00 | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xb8f08 | load | 4 | `abs 0xb8f3c` | `-` |
| `str` at 0xb8f0c | store | 4 | `sp -0x2c` | `0x109554` |
| `ldrb` at 0xb8f10 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8f14 | store | 4 | `sp -0x28` | `(r0 & 0xff)` |
| `ldr` at 0xb8f1c | load | 4 | `abs 0xb8f40` | `-` |
| `ldmdb` at 0xb8f2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8f2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8f2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8e88 | `(unknown callee) 0xb7858` | `sp-28` | `0x27eb74` | `?` | `sp-28` |
| `bl` at 0xb8ee8 | `(unknown callee) 0xb726c` | `?` | `[[sp-28]+2]:1` | `[sp-24]` | `?` |
| `bl` at 0xb8f28 | `(unknown callee) 0x1eed0` | `0x6` | `0x1094f8` | `0x89` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8e4c | `(r0 & 0xff) >= [0x2841c0] (signed, 32-bit)` | `(r0 & 0xff) < [0x2841c0] (signed, 32-bit)` |
| `blt` at 0xb8ea8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb8ec4 | `[[sp-28]+1]:1 != [[sp-24]]:1` | `[[sp-28]+1]:1 == [[sp-24]]:1` |
| `bne` at 0xb8edc | `[[sp-28]+2]:1 != [[sp-24]+1]:1` | `[[sp-28]+2]:1 == [[sp-24]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8f2c | `r0` | `?` |

## `vsc055_ResetAll`

- `vsc055_ResetAll`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8f44 to 0xb8fb8.
  Established. The byte at sp -0x11 begins at 0x0 (`strb` at 0xb8f58). The loop's `blt` predicate
  has unknown operands, and the inner `beq` retains only Z == 1 (`blt` at 0xb8f6c, `beq` at
  0xb8f8c). That taken inner path skips the call to 0xb8fb8, whose four candidates are unknown
  (`beq` at 0xb8f8c, `bl` at 0xb8f98). The loop's later byte store back to sp -0x11 also has an
  unknown value (`strb` at 0xb8fa4); its iteration count and reset semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb8f58 | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0xb8f5c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8f60 | load | 4 | `abs 0xb8fb0` | `-` |
| `ldr` at 0xb8f64 | load | 4 | `abs 0x2841c0` | `-` |
| `ldrb` at 0xb8f74 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8f78 | load | 4 | `abs 0xb8fb4` | `-` |
| `ldr` at 0xb8f7c | load | 4 | `abs 0x27eb78` | `-` |
| `ldrb` at 0xb8f84 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb8f90 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb8f9c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb8fa4 | store | 1 | `sp -0x11` | `?` |
| `ldmdb` at 0xb8fac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8fac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8fac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8f98 | `(unknown callee) 0xb8fb8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb8f6c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb8f8c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8fac | `r0` | `?` |

## `vsc055_Reset`

- `vsc055_Reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8fb8 to 0xb9170.
  Established. The first `ble` takes the path past 0x1eed0 for `(r0 & 0xff) <= [0x2841c0]` signed
  (`ble` at 0xb8fe0, `bl` at 0xb9004). A later call to 0xabb28 has candidates `[[sp-24]]:1`,
  `[[sp-24]+1]:1`, 0xfc and 0x80 (`bl` at 0xb9078); its zero-result path selected by `beq` reaches
  0xb9f1c with first candidate 0x2 (`beq` at 0xb908c, `bl` at 0xb9098). The following call to
  0xabc4c has r2 candidate 0xff and r3 candidate sp-25 (`bl` at 0xb90c0). The later `bne` takes the
  path back to the loop increment when `[sp-25]:1 != 0x21`, while equality falls through to a
  jump to the return (`bne` at 0xb90e4, `b` at 0xb90e8). The hardware reset contract remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb8fcc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb8fd0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8fd4 | load | 4 | `abs 0xb9154` | `-` |
| `ldr` at 0xb8fd8 | load | 4 | `abs 0x2841c0` | `-` |
| `ldr` at 0xb8fe4 | load | 4 | `abs 0xb9158` | `-` |
| `str` at 0xb8fe8 | store | 4 | `sp -0x34` | `0x109574` |
| `ldrb` at 0xb8fec | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8ff0 | store | 4 | `sp -0x30` | `(r0 & 0xff)` |
| `ldr` at 0xb8ff8 | load | 4 | `abs 0xb915c` | `-` |
| `ldrb` at 0xb9008 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb900c | load | 4 | `abs 0xb9160` | `-` |
| `ldr` at 0xb9010 | load | 4 | `abs 0x27eb78` | `-` |
| `ldrb` at 0xb9018 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb9028 | load | 4 | `abs 0xb9164` | `-` |
| `ldrb` at 0xb902c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb9034 | load | 4 | `abs 0x27eb74` | `-` |
| `str` at 0xb903c | store | 4 | `sp -0x18` | `?` |
| `str` at 0xb9044 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb9048 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9058 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb905c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb9060 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb9064 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xb9080 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xb9084 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb909c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb90a0 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb90a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb90a8 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xb90c8 | store | 4 | `sp -0x20` | `ret(0xabc4c)` |
| `ldr` at 0xb90cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb90dc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb90ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb90f4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb90fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb9108 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb9110 | store | 4 | `sp -0x28` | `ret(0x1ea50)` |
| `ldr` at 0xb9118 | load | 4 | `abs 0xb9168` | `-` |
| `str` at 0xb911c | store | 4 | `sp -0x28` | `0x109588` |
| `ldr` at 0xb9120 | load | 4 | `abs 0xb916c` | `-` |
| `str` at 0xb9124 | store | 4 | `sp -0x34` | `0x1095a0` |
| `ldr` at 0xb9128 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb912c | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xb9130 | store | 4 | `sp -0x30` | `[[sp-24]+4]` |
| `ldr` at 0xb9134 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb9138 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb9140 | load | 4 | `abs 0xb915c` | `-` |
| `ldmdb` at 0xb9150 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9150 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9004 | `(unknown callee) 0x1eed0` | `0x9` | `0x1094f8` | `0xab` | `0x0` |
| `bl` at 0xb9078 | `(unknown callee) 0xabb28` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `0xfc` | `0x80` |
| `bl` at 0xb9098 | `(unknown callee) 0xb9f1c` | `0x2` | `?` | `?` | `ret(0xabb28)` |
| `bl` at 0xb90c0 | `(unknown callee) 0xabc4c` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `0xff` | `sp-25` |
| `bl` at 0xb910c | `(unknown callee) 0x1ea50` | `[sp-32]` | `?` | `?` | `[sp-32]` |
| `bl` at 0xb914c | `(unknown callee) 0x1eed0` | `0x3` | `0x1094f8` | `0xcb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb8fe0 | `(r0 & 0xff) <= [0x2841c0] (signed, 32-bit)` | `(r0 & 0xff) > [0x2841c0] (signed, 32-bit)` |
| `bne` at 0xb9020 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xb9050 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb908c | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `beq` at 0xb90d4 | `ret(0xabc4c) == 0x0` | `ret(0xabc4c) != 0x0` |
| `bne` at 0xb90e4 | `[sp-25]:1 != 0x21` | `[sp-25]:1 == 0x21` |
| `beq` at 0xb9104 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9150 | `r0` | `?` |

## `vsc055_PortReset`

- `vsc055_PortReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9170 to 0xb9268.
  Established. Byte reads at entry r0 +0x5 and +0x6 precede an unknown-valued byte save at sp -0x16
  (`ldrb` at 0xb918c, `ldrb` at 0xb9190, `strb` at 0xb9198). The first call to 0xabb28 receives
  `[[sp-20]+1]:1`, `[[sp-20]+2]:1`, `[[sp-20]+3]:1` and `[sp-22]:1` (`bl` at 0xb91f4). The second
  call to that same target has an unknown third candidate and `[sp-21]:1` as its fourth (`bl` at
  0xb9230). `beq` takes the second call's zero-result path past a potential word-result replacement,
  and `bne` also skips that replacement for nonzero `[sp-28]` (`beq` at 0xb9244, `bne` at 0xb9250,
  `str` at 0xb9258). The physical port-reset meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9180 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb9184 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb9188 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb918c | load | 1 | `r0 +0x5` | `-` |
| `ldrb` at 0xb9190 | load | 1 | `r0 +0x6` | `-` |
| `strb` at 0xb9198 | store | 1 | `sp -0x16` | `?` |
| `ldr` at 0xb919c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb91a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb91a4 | load | 1 | `r0 +0x5` | `-` |
| `ldr` at 0xb91ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb91b4 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldrb` at 0xb91bc | load | 1 | `r0 +0x4` | `-` |
| `strb` at 0xb91c4 | store | 1 | `sp -0x15` | `?` |
| `ldr` at 0xb91c8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb91cc | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb91d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb91d4 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xb91d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb91dc | load | 1 | `[sp-20] +0x3` | `-` |
| `ldrb` at 0xb91e0 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xb91fc | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldr` at 0xb9200 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9204 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb9208 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb920c | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xb9210 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9214 | load | 1 | `[sp-20] +0x3` | `-` |
| `ldrb` at 0xb9220 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb9238 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xb923c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb9248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb9254 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb9258 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldr` at 0xb925c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb9264 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9264 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb91f4 | `(unknown callee) 0xabb28` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `[[sp-20]+3]:1` | `[sp-22]:1` |
| `bl` at 0xb9230 | `(unknown callee) 0xabb28` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb9244 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `bne` at 0xb9250 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9264 | `r0` | `[sp-28]` |

## `vsc055_PortSoftReset`

- `vsc055_PortSoftReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9268 to 0xb9510.
  Established. The initial call to 0xabc4c has candidates `[r0+1]:1`, `[r0+2]:1`, unknown and
  sp-23 (`bl` at 0xb92b4). `beq` takes its zero-result path into further work, while nonzero
  fall-through copies the result to sp -0x20 and jumps to the return join (`beq` at 0xb92c8, `str`
  at 0xb92d0, `b` at 0xb92d4). A later `beq` takes the zero-byte `[sp-22]:1` path past the three
  calls to 0xabb28 (`beq` at 0xb9374, `bl` at 0xb9420, `bl` at 0xb9474, `bl` at 0xb94d8). The last
  call's fourth candidate is unknown (`bl` at 0xb94d8). The soft-reset and mask semantics are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9278 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xb9280 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xb9284 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9288 | load | 1 | `r0 +0x1` | `-` |
| `ldr` at 0xb928c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9290 | load | 1 | `r0 +0x2` | `-` |
| `ldr` at 0xb9294 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9298 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xb92bc | store | 4 | `sp -0x1c` | `ret(0xabc4c)` |
| `ldr` at 0xb92c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb92cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb92d0 | store | 4 | `sp -0x20` | `ret(0xabc4c)` |
| `ldr` at 0xb92d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb92dc | load | 1 | `[sp-20] +0x4` | `-` |
| `ldrb` at 0xb92e0 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xb92f8 | store | 1 | `sp -0x16` | `?` |
| `ldr` at 0xb92fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9300 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb9308 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb930c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldrb` at 0xb9314 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xb9324 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xb932c | store | 1 | `sp -0x16` | `?` |
| `ldr` at 0xb9330 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9334 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb9338 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb933c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldrb` at 0xb9344 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xb9350 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xb9360 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xb9368 | store | 1 | `sp -0x16` | `?` |
| `ldrb` at 0xb936c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xb9378 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb937c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb9380 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9384 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldrb` at 0xb9388 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldrb` at 0xb9398 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xb93a0 | load | 1 | `[sp-20] +0x4` | `-` |
| `strb` at 0xb93a8 | store | 1 | `sp -0x18` | `?` |
| `ldr` at 0xb93ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb93b0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xb93b4 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xb93b8 | load | 1 | `sp -0x18` | `-` |
| `str` at 0xb93bc | store | 4 | `sp -0x28` | `[sp-24]:1` |
| `ldrb` at 0xb93c0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb93c4 | store | 4 | `sp -0x24` | `[sp-21]:1` |
| `ldr` at 0xb93c8 | load | 4 | `abs 0xb950c` | `-` |
| `ldr` at 0xb93cc | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xb93dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb93e0 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb93e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb93e8 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xb93ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb93f0 | load | 1 | `[sp-20] +0x3` | `-` |
| `ldr` at 0xb93f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb93f8 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldrb` at 0xb93fc | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xb9408 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb9428 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldr` at 0xb942c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb9438 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb943c | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xb9444 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9448 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb944c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9450 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xb9454 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9458 | load | 1 | `[sp-20] +0x3` | `-` |
| `ldrb` at 0xb9464 | load | 1 | `sp -0x18` | `-` |
| `str` at 0xb947c | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldr` at 0xb9480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb948c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb9490 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xb9498 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb949c | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb94a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb94a4 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xb94a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb94ac | load | 1 | `[sp-20] +0x3` | `-` |
| `ldr` at 0xb94b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb94b4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb94b8 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldrb` at 0xb94bc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xb94c4 | load | 1 | `[sp-20] +0x4` | `-` |
| `str` at 0xb94e0 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldr` at 0xb94e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb94f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb94f4 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `str` at 0xb9500 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb9504 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb9508 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9508 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9508 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb92b4 | `(unknown callee) 0xabc4c` | `[r0+1]:1` | `[r0+2]:1` | `?` | `sp-23` |
| `bl` at 0xb93d8 | `(unknown callee) 0x1dba4` | `0x1095c4` | `[[sp-20]+12]` | `[sp-22]:1` | `[sp-23]:1` |
| `bl` at 0xb9420 | `(unknown callee) 0xabb28` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `[[sp-20]+3]:1` | `?` |
| `bl` at 0xb9474 | `(unknown callee) 0xabb28` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `?` | `[sp-24]:1` |
| `bl` at 0xb94d8 | `(unknown callee) 0xabb28` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `[[sp-20]+3]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb92c8 | `ret(0xabc4c) == 0x0` | `ret(0xabc4c) != 0x0` |
| `beq` at 0xb9320 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb935c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb9374 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xb9434 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `beq` at 0xb9488 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `beq` at 0xb94ec | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9508 | `r0` | `?` |

## `vsc055_GetCfg`

- `vsc055_GetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9510 to 0xb954c.
  Established. The word loaded from 0x27eb74 is stored at entry r0 +0x0 (`ldr` at 0xb952c, `str` at
  0xb9530). A separate word load from 0x2841c0 supplies return expression `[0x2841c0]` (`ldr` at
  0xb9538, `ldmdb` at 0xb9540). Neither word's configuration interpretation is established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9520 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb9524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb9528 | load | 4 | `abs 0xb9544` | `-` |
| `ldr` at 0xb952c | load | 4 | `abs 0x27eb74` | `-` |
| `str` at 0xb9530 | store | 4 | `r0 +0x0` | `[0x27eb74]` |
| `ldr` at 0xb9534 | load | 4 | `abs 0xb9548` | `-` |
| `ldr` at 0xb9538 | load | 4 | `abs 0x2841c0` | `-` |
| `ldmdb` at 0xb9540 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9540 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9540 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9540 | `r0` | `[0x2841c0]` |

## `vsc055_Count`

- `vsc055_Count`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb954c to 0xb956c.
  Established. The word read at 0x2841c0 supplies r0 expression `[0x2841c0]` at return (`ldr` at
  0xb955c, `ldmdb` at 0xb9564). The returned word's meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb9558 | load | 4 | `abs 0xb9568` | `-` |
| `ldr` at 0xb955c | load | 4 | `abs 0x2841c0` | `-` |
| `ldmdb` at 0xb9564 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9564 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9564 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9564 | `r0` | `[0x2841c0]` |

## `vsc055_Present`

- `vsc055_Present`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb956c to 0xb95cc.
  Established. `blt` takes the indexed-byte path when `(r0 & 0xff) < [0x2841c0]` signed (`blt` at
  0xb9594). That path reads a byte at `((r0 & 0xff) + [0x27eb78]) +0x0` and saves its expression as
  a word at sp -0x18 (`ldrb` at 0xb95b4, `str` at 0xb95b8). Fall-through instead saves 0x0 there and
  jumps to the join (`str` at 0xb959c, `b` at 0xb95a0). The merged return and physical presence
  meaning remain unknown (`ldmdb` at 0xb95c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb9580 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb9584 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb9588 | load | 4 | `abs 0xb95c4` | `-` |
| `ldr` at 0xb958c | load | 4 | `abs 0x2841c0` | `-` |
| `str` at 0xb959c | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0xb95a4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb95a8 | load | 4 | `abs 0xb95c8` | `-` |
| `ldr` at 0xb95ac | load | 4 | `abs 0x27eb78` | `-` |
| `ldrb` at 0xb95b4 | load | 1 | `((r0 & 0xff) + [0x27eb78]) +0x0` | `-` |
| `str` at 0xb95b8 | store | 4 | `sp -0x18` | `[((r0 & 0xff) + [0x27eb78])]:1` |
| `ldr` at 0xb95bc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb95c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb95c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb95c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb9594 | `(r0 & 0xff) < [0x2841c0] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2841c0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb95c0 | `r0` | `?` |

## `vsc055_IntSet`

- `vsc055_IntSet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb95cc to 0xb96cc.
  Established. A byte read at entry r2 +0x11582c supplies the masked save `([r2+1136684]:1 & 0xff)`
  at sp -0x1f (`ldrb` at 0xb95f8, `strb` at 0xb95fc). The loop's `bls` and inner `beq` retain
  unknown operands (`bls` at 0xb9610, `beq` at 0xb9624). The call to 0xabb28 has candidates
  `[[sp-20]+1]:1`, `[[sp-20]+2]:1`, `[sp-30]:1` and `[sp-31]:1` (`bl` at 0xb9678); its following
  `beq` takes the zero-result path to the loop's byte updates (`beq` at 0xb968c, `strb` at
  0xb96a4, `strb` at 0xb96b0). The interrupt programming meaning and iteration count are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb95dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb95e4 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xb95e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xb95ec | load | 4 | `abs 0xb96c8` | `-` |
| `ldr` at 0xb95f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb95f8 | load | 1 | `r2 +0x11582c` | `-` |
| `strb` at 0xb95fc | store | 1 | `sp -0x1f` | `([r2+1136684]:1 & 0xff)` |
| `strb` at 0xb9604 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xb9608 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xb9618 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb9628 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb962c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb9638 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xb9650 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xb9654 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9658 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb965c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9660 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldrb` at 0xb9664 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0xb9668 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xb9680 | store | 4 | `sp -0x24` | `ret(0xabb28)` |
| `ldr` at 0xb9684 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9690 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb9694 | store | 4 | `sp -0x28` | `ret(0xabb28)` |
| `ldrb` at 0xb969c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xb96a4 | store | 1 | `sp -0x15` | `?` |
| `ldrb` at 0xb96a8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xb96b0 | store | 1 | `sp -0x1d` | `?` |
| `str` at 0xb96bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb96c0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb96c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb96c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb96c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9678 | `(unknown callee) 0xabb28` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `[sp-30]:1` | `[sp-31]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb9610 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xb9624 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb968c | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb96c4 | `r0` | `?` |

## `vsc055_IntGet`

- `vsc055_IntGet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb96cc to 0xb9800.
  Established. The routine clears the byte at entry r1 +0x0 (`strb` at 0xb96ec); its call to 0xabc4c
  uses candidates `[[sp-20]+1]:1`, `[[sp-20]+2]:1`, `[sp-26]:1` and sp-27 (`bl` at 0xb9758). `beq`
  takes the zero-result path into tests of the word at sp -0x28 (`beq` at 0xb976c, `ldr` at
  0xb9788). Later `beq` sites take the same byte-update path for `[sp-40]` equal to 0xa, 0x6 or 0xe
  (`beq` at 0xb9790, `beq` at 0xb97a8, `beq` at 0xb97b8). That byte store at `[sp-24] +0x0` has an
  unknown value (`strb` at 0xb97dc). The interrupt-status encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb96dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb96e0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb96e4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb96ec | store | 1 | `r1 +0x0` | `0x0` |
| `strb` at 0xb96f4 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xb96f8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb9708 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb970c | load | 1 | `[sp-20] +0x3` | `-` |
| `ldrb` at 0xb9718 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xb9730 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xb9734 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9738 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xb973c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9740 | load | 1 | `[sp-20] +0x2` | `-` |
| `ldrb` at 0xb9744 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xb9760 | store | 4 | `sp -0x20` | `ret(0xabc4c)` |
| `ldr` at 0xb9764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb9770 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb9774 | store | 4 | `sp -0x24` | `ret(0xabc4c)` |
| `ldrb` at 0xb977c | load | 1 | `sp -0x1b` | `-` |
| `str` at 0xb9784 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb9788 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb9794 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb97a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb97b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb97c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb97c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb97c8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb97d4 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0xb97dc | store | 1 | `[sp-24] +0x0` | `?` |
| `ldrb` at 0xb97e0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xb97e8 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `str` at 0xb97f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb97f8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb97fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb97fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb97fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9758 | `(unknown callee) 0xabc4c` | `[[sp-20]+1]:1` | `[[sp-20]+2]:1` | `[sp-26]:1` | `sp-27` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb9700 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xb976c | `ret(0xabc4c) == 0x0` | `ret(0xabc4c) != 0x0` |
| `beq` at 0xb9790 | `[sp-40] == 0xa` | `[sp-40] != 0xa` |
| `bgt` at 0xb979c | `[sp-40] > 0xa (signed, 32-bit)` | `[sp-40] <= 0xa (signed, 32-bit)` |
| `beq` at 0xb97a8 | `[sp-40] == 0x6` | `[sp-40] != 0x6` |
| `beq` at 0xb97b8 | `[sp-40] == 0xe` | `[sp-40] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb97fc | `r0` | `?` |

## `vsc055_Int`

- `vsc055_Int`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9800 to 0xb98b4.
  Established. The saved byte values `(r0 & 0xff)` and `(r1 & 0xff)` occupy sp -0x11 and sp -0x12
  (`strb` at 0xb9818, `strb` at 0xb9820). A word at sp -0x1c begins at 0x0 (`str` at 0xb9828). The
  loop's `ble` has unknown operands, and an inner `beq` retains only Z == 1 (`ble` at 0xb9840, `beq`
  at 0xb9860). The call to 0xb98b4 has first candidate unknown and second candidate 0x27eb74 (`bl`
  at 0xb987c); its result is included in the word store `([sp-28] + ret(0xb98b4))` at sp -0x1c
  (`str` at 0xb988c). The aggregate's interrupt meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb9818 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb9820 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0xb9828 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0xb982c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb9830 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldrb` at 0xb9834 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb9838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb9848 | load | 4 | `abs 0xb98ac` | `-` |
| `ldr` at 0xb984c | load | 4 | `abs 0x27eb78` | `-` |
| `ldr` at 0xb9850 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb9858 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb9864 | load | 4 | `abs 0xb98b0` | `-` |
| `ldr` at 0xb9868 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb9870 | load | 4 | `abs 0x27eb74` | `-` |
| `ldr` at 0xb9884 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb988c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0xb98b4))` |
| `ldr` at 0xb9890 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb9898 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb98a0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb98a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb98a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb98a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb987c | `(unknown callee) 0xb98b4` | `?` | `0x27eb74` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb9840 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb9860 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb98a8 | `r0` | `?` |

## `vsc055_FanPWMInit`

- `vsc055_FanPWMInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9d98 to 0xb9e38.
  Established. A byte at sp -0x29 is initialized to 0x0 before the call to 0xb7858 (`strb` at
  0xb9dc4, `bl` at 0xb9dd0). A later byte store to that slot has an unknown value (`strb` at
  0xb9dec). The call to 0xabb28 has candidates `[sp-17]:1`, `[sp-18]:1`, unknown and `[sp-41]:1`
  (`bl` at 0xb9e20); its result is retained as the return's r0 expression (`ldmdb` at 0xb9e34). The
  PWM-register meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9da8 | store | 4 | `sp -0x18` | `r2` |
| `str` at 0xb9dac | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xb9db4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb9dbc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb9dc4 | store | 1 | `sp -0x29` | `0x0` |
| `str` at 0xb9dd8 | store | 4 | `sp -0x24` | `ret(0xb7858)` |
| `ldr` at 0xb9ddc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb9de4 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xb9dec | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xb9df0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb9df4 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb9e00 | load | 1 | `sp -0x18` | `-` |
| `ldrb` at 0xb9e0c | load | 1 | `sp -0x29` | `-` |
| `str` at 0xb9e28 | store | 4 | `sp -0x28` | `ret(0xabb28)` |
| `ldr` at 0xb9e2c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb9e34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9e34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9e34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9dd0 | `(unknown callee) 0xb7858` | `sp-32` | `r1` | `r2` | `sp-32` |
| `bl` at 0xb9e20 | `(unknown callee) 0xabb28` | `[sp-17]:1` | `[sp-18]:1` | `?` | `[sp-41]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9e34 | `r0` | `ret(0xabb28)` |

## `vsc055_FanMonitorInit`

- `vsc055_FanMonitorInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9e38 to 0xb9f1c.
  Established. The initial byte store at sp -0x2a writes 0x0 (`strb` at 0xb9e64). The first call to
  0xabb28 has fourth candidate 0xff (`bl` at 0xb9ea8). The second call to that target has candidates
  `[sp-17]:1`, `[sp-18]:1`, `[sp-41]:1` and `[sp-42]:1` (`bl` at 0xb9f04). The result of that second
  call reaches the return in r0 (`ldmdb` at 0xb9f18). The monitor-register selection and stored bit
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9e48 | store | 4 | `sp -0x18` | `r2` |
| `str` at 0xb9e4c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xb9e54 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb9e5c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb9e64 | store | 1 | `sp -0x2a` | `0x0` |
| `str` at 0xb9e78 | store | 4 | `sp -0x24` | `ret(0xb7858)` |
| `ldr` at 0xb9e7c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb9e88 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xb9e8c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb9e90 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb9e94 | load | 1 | `sp -0x29` | `-` |
| `str` at 0xb9eb0 | store | 4 | `sp -0x28` | `ret(0xabb28)` |
| `ldrb` at 0xb9eb4 | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0xb9ec0 | store | 1 | `sp -0x2a` | `?` |
| `ldrb` at 0xb9ec4 | load | 1 | `sp -0x2a` | `-` |
| `ldrb` at 0xb9ec8 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xb9ed0 | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0xb9ed4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb9ee0 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xb9ee4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb9ee8 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb9eec | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xb9ef0 | load | 1 | `sp -0x2a` | `-` |
| `str` at 0xb9f0c | store | 4 | `sp -0x28` | `ret(0xabb28)` |
| `ldr` at 0xb9f10 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb9f18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9f18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9f18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9e70 | `(unknown callee) 0xb7858` | `sp-32` | `r1` | `r2` | `sp-32` |
| `bl` at 0xb9ea8 | `(unknown callee) 0xabb28` | `[sp-17]:1` | `[sp-18]:1` | `[sp-41]:1` | `0xff` |
| `bl` at 0xb9f04 | `(unknown callee) 0xabb28` | `[sp-17]:1` | `[sp-18]:1` | `[sp-41]:1` | `[sp-42]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9f18 | `r0` | `ret(0xabb28)` |
