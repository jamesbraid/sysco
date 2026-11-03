# ARM L1 I/O-expander entry paths in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/io-expander-facts.txt) retain all tool sections.

## `ioExpInit`

- `ioExpInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7084 to 0xb71dc.
  Established. The word read at 0x284180 decides `beq`, whose taken zero path skips the first call
  to 0x1eed0 (`ldr` at 0xb70a0, `beq` at 0xb70a8, `bl` at 0xb70c4). The call to 0x18ba8 supplies the
  word stored at 0x284160 (`bl` at 0xb70d4, `str` at 0xb70dc). Later word stores write `[sp-24]` at
  0x284180 and `[sp-28]` at 0x2841a0 (`str` at 0xb7110, `str` at 0xb711c). The loop includes byte
  stores of 0x0 and 0x1 at unknown effective targets (`strb` at 0xb7180, `strb` at 0xb71ac); its
  `blt` predicate has unknown operands (`blt` at 0xb7134). The globals' physical roles remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7094 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb7098 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb709c | load | 4 | `abs 0xb71c4` | `-` |
| `ldr` at 0xb70a0 | load | 4 | `abs 0x284180` | `-` |
| `ldr` at 0xb70ac | load | 4 | `abs 0xb71c8` | `-` |
| `str` at 0xb70b0 | store | 4 | `sp -0x24` | `0x109150` |
| `ldr` at 0xb70b8 | load | 4 | `abs 0xb71cc` | `-` |
| `ldr` at 0xb70c8 | load | 4 | `abs 0xb71d0` | `-` |
| `ldr` at 0xb70cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb70dc | store | 4 | `abs 0x284160` | `ret(0x18ba8)` |
| `ldr` at 0xb70e0 | load | 4 | `abs 0x284160` | `-` |
| `ldr` at 0xb70ec | load | 4 | `abs 0xb71d4` | `-` |
| `str` at 0xb70f0 | store | 4 | `sp -0x24` | `0x109188` |
| `ldr` at 0xb70f8 | load | 4 | `abs 0xb71cc` | `-` |
| `ldr` at 0xb7108 | load | 4 | `abs 0xb71c4` | `-` |
| `ldr` at 0xb710c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7110 | store | 4 | `abs 0x284180` | `[sp-24]` |
| `ldr` at 0xb7114 | load | 4 | `abs 0xb71d8` | `-` |
| `ldr` at 0xb7118 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb711c | store | 4 | `abs 0x2841a0` | `[sp-28]` |
| `str` at 0xb7124 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb7128 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb712c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb713c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7144 | load | 4 | `abs 0xb71c4` | `-` |
| `ldr` at 0xb7148 | load | 4 | `abs 0x284180` | `-` |
| `ldrb` at 0xb7150 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb715c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7170 | load | 4 | `abs 0xb71d0` | `-` |
| `ldr` at 0xb7174 | load | 4 | `abs 0x284160` | `-` |
| `strb` at 0xb7180 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb7188 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb719c | load | 4 | `abs 0xb71d0` | `-` |
| `ldr` at 0xb71a0 | load | 4 | `abs 0x284160` | `-` |
| `strb` at 0xb71ac | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xb71b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb71b8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xb71c0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb71c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb71c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb71c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb70c4 | `(unknown callee) 0x1eed0` | `0x9` | `0x10916c` | `0x48` | `0x0` |
| `bl` at 0xb70d4 | `(unknown callee) 0x18ba8` | `?` | `0xc` | `?` | `?` |
| `bl` at 0xb7104 | `(unknown callee) 0x1eed0` | `0x9` | `0x10916c` | `0x50` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb70a8 | `[0x284180] == 0x0` | `[0x284180] != 0x0` |
| `bne` at 0xb70e8 | `[0x284160] != 0x0` | `[0x284160] == 0x0` |
| `blt` at 0xb7134 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb7158 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb71c0 | `r0` | `?` |

## `ioExpEnable`

- `ioExpEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb71dc to 0xb726c.
  Established. The signed `bge` takes the path to 0x1eed0 when `(r0 & 0xff) >= [0x2841a0]` (`bge` at
  0xb7204, `bl` at 0xb7254). Fall-through reaches a byte store of 0x1 at an unknown effective target
  (`strb` at 0xb722c). The call path instead saves `(r0 & 0xff)` as a word at sp -0x18 before that
  call (`str` at 0xb7240, `bl` at 0xb7254). The hardware-enable behavior is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb71f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb71f4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb71f8 | load | 4 | `abs 0xb725c` | `-` |
| `ldr` at 0xb71fc | load | 4 | `abs 0x2841a0` | `-` |
| `ldrb` at 0xb7208 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb721c | load | 4 | `abs 0xb7260` | `-` |
| `ldr` at 0xb7220 | load | 4 | `abs 0x284160` | `-` |
| `strb` at 0xb722c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xb7234 | load | 4 | `abs 0xb7264` | `-` |
| `str` at 0xb7238 | store | 4 | `sp -0x1c` | `0x1091a4` |
| `ldrb` at 0xb723c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7240 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xb7248 | load | 4 | `abs 0xb7268` | `-` |
| `ldmdb` at 0xb7258 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7258 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7254 | `(unknown callee) 0x1eed0` | `0x6` | `0x10916c` | `0x6a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb7204 | `(r0 & 0xff) >= [0x2841a0] (signed, 32-bit)` | `(r0 & 0xff) < [0x2841a0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7258 | `r0` | `?` |

## `ioExpDisable`

- `ioExpDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb726c to 0xb72fc.
  Established. The byte at sp -0x11 retains `(r0 & 0xff)` (`strb` at 0xb7280). The following `bge`
  takes the path to 0x1eed0 for `(r0 & 0xff) >= [0x2841a0]` signed (`bge` at 0xb7294, `bl` at
  0xb72e4); its fall-through reaches a byte store of 0x0 at an unknown effective target (`strb` at
  0xb72bc). The physical disable meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7280 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb7284 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb7288 | load | 4 | `abs 0xb72ec` | `-` |
| `ldr` at 0xb728c | load | 4 | `abs 0x2841a0` | `-` |
| `ldrb` at 0xb7298 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb72ac | load | 4 | `abs 0xb72f0` | `-` |
| `ldr` at 0xb72b0 | load | 4 | `abs 0x284160` | `-` |
| `strb` at 0xb72bc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb72c4 | load | 4 | `abs 0xb72f4` | `-` |
| `str` at 0xb72c8 | store | 4 | `sp -0x1c` | `0x1091c4` |
| `ldrb` at 0xb72cc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb72d0 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xb72d8 | load | 4 | `abs 0xb72f8` | `-` |
| `ldmdb` at 0xb72e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb72e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb72e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb72e4 | `(unknown callee) 0x1eed0` | `0x6` | `0x10916c` | `0x76` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb7294 | `(r0 & 0xff) >= [0x2841a0] (signed, 32-bit)` | `(r0 & 0xff) < [0x2841a0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb72e8 | `r0` | `?` |

## `ioExpReset`

- `ioExpReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb72fc to 0xb747c.
  Established. The initial `beq` takes the zero-byte `[[sp-28]]:1` path to the return (`beq` at
  0xb735c, `ldmdb` at 0xb7470). Nonzero fall-through reaches an unknown-valued byte store at
  `[sp-28] +0x1` before a computed transfer (`strb` at 0xb7378, `ldrls` at 0xb7388). The `ldrls`
  taken condition is `[[sp-24]]:1 <= 0x3 (unsigned, 32-bit)`, with an unresolved taken edge (`ldrls`
  at 0xb7388). On known later paths, a `bne` skips 0xb86d4 when `[sp-32] != 0x0`, and a byte store
  clears `[sp-28] +0x3` (`bne` at 0xb7414, `bl` at 0xb7428, `strb` at 0xb743c). Additional dispatch
  effects and physical reset behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7310 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xb7314 | load | 4 | `abs 0xb7474` | `-` |
| `ldrb` at 0xb7318 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb7320 | load | 4 | `abs 0x284180` | `-` |
| `str` at 0xb7328 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb732c | load | 4 | `abs 0xb7478` | `-` |
| `ldrb` at 0xb7330 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb7344 | load | 4 | `abs 0x284160` | `-` |
| `str` at 0xb734c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb7350 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb7354 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xb7360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7368 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb736c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldrb` at 0xb7370 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xb7378 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0xb737c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7380 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xb7388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb740c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7420 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7430 | store | 4 | `sp -0x20` | `ret(0xb86d4)` |
| `ldr` at 0xb7434 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb743c | store | 1 | `[sp-28] +0x3` | `0x0` |
| `ldr` at 0xb7440 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb744c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb7450 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7454 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xb746c | store | 4 | `sp -0x20` | `ret(0xb880c)` |
| `ldmdb` at 0xb7470 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7470 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7470 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7428 | `(unknown callee) 0xb86d4` | `[sp-24]` | `[sp-28]+2` | `[sp-24]` | `[sp-28]+2` |
| `bl` at 0xb7464 | `(unknown callee) 0xb880c` | `[sp-17]:1` | `[[sp-24]+7]:1` | `0x1` | `[[sp-24]+7]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb735c | `[[sp-28]]:1 == 0x0` | `[[sp-28]]:1 != 0x0` |
| `ldrls` at 0xb7388 | `[[sp-24]]:1 <= 0x3 (unsigned, 32-bit)` | `[[sp-24]]:1 > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xb7414 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xb7448 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7470 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb7388).

## `ioExpHardReset`

- `ioExpHardReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb747c to 0xb74d4.
  Established. The first call targets 0xb8f44 (`bl` at 0xb748c). A byte at sp -0x11 then begins at
  0x0 (`strb` at 0xb7494). The loop's `blt` uses unknown operands, and its body calls 0xb72fc with
  an unknown first candidate (`blt` at 0xb74a8, `bl` at 0xb74b8). The byte store at the back-edge
  writes `([sp-17]:1+1 & 0xff)` to sp -0x11 (`strb` at 0xb74c4). The iteration count and hard-reset
  semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7494 | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0xb7498 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb749c | load | 4 | `abs 0xb74d0` | `-` |
| `ldr` at 0xb74a0 | load | 4 | `abs 0x2841a0` | `-` |
| `ldrb` at 0xb74b0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb74bc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb74c4 | store | 1 | `sp -0x11` | `([sp-17]:1+1 & 0xff)` |
| `ldmdb` at 0xb74cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb74cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb74cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb748c | `(unknown callee) 0xb8f44` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb74b8 | `(unknown callee) 0xb72fc` | `?` | `?` | `[0x2841a0]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb74a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb74cc | `r0` | `?` |

## `ioExpSoftResetData`

- `ioExpSoftResetData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb74d4 to 0xb7644.
  Established. The call to 0xab428 is followed by `beq`, whose taken zero-result path reaches the
  byte initialization at sp -0x19 (`bl` at 0xb74e4, `beq` at 0xb74f8, `strb` at 0xb7538). The loop's
  call to 0xb86d4 has four unknown candidates (`bl` at 0xb75b0). `beq` takes its zero-result path
  past 0x1e71c and 0x1dba4 (`beq` at 0xb75c4, `bl` at 0xb75cc, `bl` at 0xb75e0). That path later
  stores an unknown byte at `[sp-24] +0x1` (`strb` at 0xb7608). The soft-reset data meaning remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb74ec | store | 4 | `sp -0x20` | `ret(0xab428)` |
| `ldr` at 0xb74f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb74fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7508 | load | 4 | `abs 0xb762c` | `-` |
| `str` at 0xb750c | store | 4 | `sp -0x2c` | `0x1091e4` |
| `str` at 0xb7510 | store | 4 | `sp -0x28` | `ret(0x1e71c)` |
| `ldr` at 0xb7518 | load | 4 | `abs 0xb7630` | `-` |
| `str` at 0xb752c | store | 4 | `sp -0x24` | `?` |
| `strb` at 0xb7538 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xb753c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb7540 | load | 4 | `abs 0xb7634` | `-` |
| `ldr` at 0xb7544 | load | 4 | `abs 0x2841a0` | `-` |
| `ldr` at 0xb7554 | load | 4 | `abs 0xb7638` | `-` |
| `ldrb` at 0xb7558 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb7560 | load | 4 | `abs 0x284180` | `-` |
| `str` at 0xb7568 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb756c | load | 4 | `abs 0xb763c` | `-` |
| `ldrb` at 0xb7570 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb7584 | load | 4 | `abs 0x284160` | `-` |
| `str` at 0xb758c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb7590 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7594 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb75a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb75a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb75b8 | store | 4 | `sp -0x20` | `ret(0xb86d4)` |
| `ldr` at 0xb75bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb75c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb75d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb75d8 | load | 4 | `abs 0xb7640` | `-` |
| `ldr` at 0xb75dc | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb75e8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb75f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb75f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb75f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb75fc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xb7600 | load | 1 | `[sp-20] +0x4` | `-` |
| `strb` at 0xb7608 | store | 1 | `[sp-24] +0x1` | `?` |
| `ldrb` at 0xb760c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xb7614 | store | 1 | `sp -0x19` | `?` |
| `str` at 0xb7620 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb7624 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7628 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7628 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7628 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb74e4 | `(unknown callee) 0xab428` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb7500 | `(unknown callee) 0x1e71c` | `ret(0xab428)` | `?` | `?` | `ret(0xab428)` |
| `bl` at 0xb7524 | `(unknown callee) 0x1eed0` | `0x1` | `0x10916c` | `0xdb` | `0x0` |
| `bl` at 0xb75b0 | `(unknown callee) 0xb86d4` | `?` | `?` | `?` | `?` |
| `bl` at 0xb75cc | `(unknown callee) 0x1e71c` | `ret(0xb86d4)` | `?` | `?` | `ret(0xb86d4)` |
| `bl` at 0xb75e0 | `(unknown callee) 0x1dba4` | `0x109210` | `[[sp-20]+12]` | `ret(0x1e71c)` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb74f8 | `ret(0xab428) == 0x0` | `ret(0xab428) != 0x0` |
| `blt` at 0xb754c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb759c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb75c4 | `ret(0xb86d4) == 0x0` | `ret(0xb86d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7628 | `r0` | `?` |

## `ioExpSoftResetInt`

- `ioExpSoftResetInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7644 to 0xb7858.
  Established. The loop initializes a byte at `sp -0x19` to zero (`strb` at 0xb7658) and retains an
  `lt` flags condition with unknown operands (`blt` at 0xb766c). An indirect call is visible with
  all four argument candidates unknown (`mov` at 0xb778c). The call target is unknown. A byte store
  elsewhere has unknown value and effective target (`strb` at 0xb7764). Callable roles of stored
  constants and the interrupt-reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7658 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xb765c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb7660 | load | 4 | `abs 0xb783c` | `-` |
| `ldr` at 0xb7664 | load | 4 | `abs 0x2841a0` | `-` |
| `ldr` at 0xb7674 | load | 4 | `abs 0xb7840` | `-` |
| `ldrb` at 0xb7678 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb7680 | load | 4 | `abs 0x284180` | `-` |
| `str` at 0xb7688 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb768c | load | 4 | `abs 0xb7844` | `-` |
| `ldrb` at 0xb7690 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb76a4 | load | 4 | `abs 0x284160` | `-` |
| `str` at 0xb76ac | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb76b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb76b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb76c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb76c4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb76c8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb76cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb76d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb76e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb76f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7704 | load | 4 | `abs 0xb7848` | `-` |
| `str` at 0xb7708 | store | 4 | `sp -0x20` | `0xb96cc` |
| `ldr` at 0xb770c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7710 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb7714 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7730 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb7738 | load | 4 | `abs 0xb784c` | `-` |
| `str` at 0xb773c | store | 4 | `sp -0x20` | `0xba1cc` |
| `ldr` at 0xb7744 | load | 4 | `abs 0xb7850` | `-` |
| `str` at 0xb7748 | store | 4 | `sp -0x20` | `0xad1f0` |
| `str` at 0xb7754 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb7758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb775c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7760 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xb7764 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb7768 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb777c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7780 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb77a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb77a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb77ac | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xb77b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb77b4 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb77c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb77cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb77d0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xb77d4 | load | 4 | `abs 0xb7854` | `-` |
| `ldr` at 0xb77d8 | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb77e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb77ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb77f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb77fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7800 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb780c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb781c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xb7824 | store | 1 | `sp -0x19` | `?` |
| `str` at 0xb7830 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb7834 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7838 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7838 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb771c | `(unknown callee) 0xb9268` | `?` | `?` | `?` | `?` |
| `mov` at 0xb778c | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` |
| `bl` at 0xb77dc | `(unknown callee) 0x1dba4` | `0x109234` | `[[sp-20]+12]` | `[[sp-24]+3]:1` | `[sp-20]` |
| `bl` at 0xb7818 | `(unknown callee) 0xb7c2c` | `?` | `0xff` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb766c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb76bc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb76d4 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0xb76e0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xb76ec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb76fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb7728 | `ret(0xb9268) == 0x0` | `ret(0xb9268) != 0x0` |
| `beq` at 0xb7770 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb7798 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb77c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb77f8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb7808 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7838 | `r0` | `?` |

## `ioExpGetCfg`

- `ioExpGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7858 to 0xb7894.
  Established. A word load from 0x284180 supplies the store of `[0x284180]` at entry r0 +0x0 (`ldr`
  at 0xb7874, `str` at 0xb7878). The word loaded separately from 0x2841a0 reaches the return as
  `[0x2841a0]` (`ldr` at 0xb7880, `ldmdb` at 0xb7888). The returned configuration interpretation is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7868 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb786c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7870 | load | 4 | `abs 0xb788c` | `-` |
| `ldr` at 0xb7874 | load | 4 | `abs 0x284180` | `-` |
| `str` at 0xb7878 | store | 4 | `r0 +0x0` | `[0x284180]` |
| `ldr` at 0xb787c | load | 4 | `abs 0xb7890` | `-` |
| `ldr` at 0xb7880 | load | 4 | `abs 0x2841a0` | `-` |
| `ldmdb` at 0xb7888 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7888 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7888 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7888 | `r0` | `[0x2841a0]` |

## `ioExpCount`

- `ioExpCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7894 to 0xb78b4.
  Established. The word read at 0x2841a0 becomes the return expression `[0x2841a0]` (`ldr` at
  0xb78a4, `ldmdb` at 0xb78ac). This body alone does not establish what the word counts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb78a0 | load | 4 | `abs 0xb78b0` | `-` |
| `ldr` at 0xb78a4 | load | 4 | `abs 0x2841a0` | `-` |
| `ldmdb` at 0xb78ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb78ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb78ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb78ac | `r0` | `[0x2841a0]` |

## `ioExpPresent`

- `ioExpPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb78b4 to 0xb790c.
  Established. The call to 0xb8348 has first candidate `(r0 & 0xff)` (`bl` at 0xb78d4). `bne` takes
  its nonzero-result path to a byte load at `ret(0xb8348) +0x0` and the subsequent word save at sp
  -0x1c (`bne` at 0xb78e8, `ldrb` at 0xb78fc, `str` at 0xb7900). Zero fall-through saves 0x0 to that
  slot and jumps to the join (`str` at 0xb78f0, `b` at 0xb78f4). The return retains `[sp-28]`, with
  the physical presence meaning unknown (`ldmdb` at 0xb7908).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb78c8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb78cc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb78dc | store | 4 | `sp -0x18` | `ret(0xb8348)` |
| `ldr` at 0xb78e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb78f0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb78f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb78fc | load | 1 | `ret(0xb8348) +0x0` | `-` |
| `str` at 0xb7900 | store | 4 | `sp -0x1c` | `[ret(0xb8348)]:1` |
| `ldr` at 0xb7904 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb7908 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7908 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7908 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb78d4 | `(unknown callee) 0xb8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb78e8 | `ret(0xb8348) != 0x0` | `ret(0xb8348) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7908 | `r0` | `[sp-28]` |

## `ioExpSetBit`

- `ioExpSetBit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb790c to 0xb7958.
  Established. The two byte saves retain `(r0 & 0xff)` at sp -0x11 and `(r1 & 0xff)` at sp -0x12
  (`strb` at 0xb7924, `strb` at 0xb792c). The call to 0xb79a0 has candidates `(r0 & 0xff)`, `(r1 &
  0xff)`, `(r1 & 0xff)` and `(r0 & 0xff)` (`bl` at 0xb7948); its result is retained in r0 at return
  (`ldmdb` at 0xb7954). The set-bit hardware semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7924 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb792c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb7930 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb7934 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb7938 | load | 1 | `sp -0x12` | `-` |
| `ldmdb` at 0xb7954 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7954 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7954 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7948 | `(unknown callee) 0xb79a0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7954 | `r0` | `ret(0xb79a0)` |

## `ioExpClearBit`

- `ioExpClearBit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7958 to 0xb79a0.
  Established. The call to 0xb79a0 uses candidates `(r0 & 0xff)`, `(r1 & 0xff)`, 0x0 and `(r0 &
  0xff)` (`bl` at 0xb7990). Before it, byte stores retain the masked entry values at sp -0x11 and sp
  -0x12 (`strb` at 0xb7970, `strb` at 0xb7978). The return's r0 expression is `ret(0xb79a0)`
  (`ldmdb` at 0xb799c). No physical bit-clear operation is established by this forwarding body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7970 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7978 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb797c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb7980 | load | 1 | `sp -0x12` | `-` |
| `ldmdb` at 0xb799c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb799c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb799c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7990 | `(unknown callee) 0xb79a0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `0x0` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb799c | `r0` | `ret(0xb79a0)` |

## `ioExpWrite`

- `ioExpWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb79a0 to 0xb7a78.
  Established. Calls first target 0xb82c8 and 0xb8348 (`bl` at 0xb79d0, `bl` at 0xb79e4). The byte
  read at `ret(0xb8348) +0x0` decides `bne`, whose taken nonzero path reaches 0xab648 (`ldrb` at
  0xb79f4, `bne` at 0xb79fc, `bl` at 0xb7a18). Zero fall-through stores 0xfffffc11 at sp -0x24
  (`str` at 0xb7a04). A later `beq` takes the zero-result path from 0xab648 to 0xb83d4, whose
  candidates are `[sp-24]`, `[sp-28]`, `[sp-18]:1` and `[sp-19]:1` (`beq` at 0xb7a2c, `bl` at
  0xb7a54). The same path then calls 0xab788 (`bl` at 0xb7a60). The write protocol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb79b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb79bc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb79c4 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xb79c8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb79d8 | store | 4 | `sp -0x18` | `ret(0xb82c8)` |
| `ldrb` at 0xb79dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb79ec | store | 4 | `sp -0x1c` | `ret(0xb8348)` |
| `ldr` at 0xb79f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb79f4 | load | 1 | `ret(0xb8348) +0x0` | `-` |
| `ldr` at 0xb7a00 | load | 4 | `abs 0xb7a74` | `-` |
| `str` at 0xb7a04 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xb7a0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7a10 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xb7a20 | store | 4 | `sp -0x20` | `ret(0xab648)` |
| `ldr` at 0xb7a24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7a30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7a34 | store | 4 | `sp -0x24` | `ret(0xab648)` |
| `ldrb` at 0xb7a3c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb7a40 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0xb7a44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7a48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7a5c | store | 4 | `sp -0x20` | `ret(0xb83d4)` |
| `ldr` at 0xb7a64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7a68 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xb7a6c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7a70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7a70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7a70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb79d0 | `(unknown callee) 0xb82c8` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb79e4 | `(unknown callee) 0xb8348` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb7a18 | `(unknown callee) 0xab648` | `[[sp-24]+1]:1` | `?` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xb7a54 | `(unknown callee) 0xb83d4` | `[sp-24]` | `[sp-28]` | `[sp-18]:1` | `[sp-19]:1` |
| `bl` at 0xb7a60 | `(unknown callee) 0xab788` | `ret(0xb83d4)` | `?` | `?` | `ret(0xb83d4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb79fc | `[ret(0xb8348)]:1 != 0x0` | `[ret(0xb8348)]:1 == 0x0` |
| `beq` at 0xb7a2c | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7a70 | `r0` | `?` |

## `ioExpGetBit`

- `ioExpGetBit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7a78 to 0xb7af8.
  Established. The call to 0xb7af8 has first candidate `(r0 & 0xff)` and second candidate sp-25
  (`bl` at 0xb7aac). `beq` takes its zero-result path to two byte loads from sp -0x19 and sp -0x12
  (`beq` at 0xb7ac0, `ldrb` at 0xb7ad0, `ldrb` at 0xb7ad4). That path's later word store at sp -0x20
  has an unknown value (`str` at 0xb7aec). Nonzero fall-through instead saves `ret(0xb7af8)` to that
  slot and jumps to the join (`str` at 0xb7ac8, `b` at 0xb7acc). The resulting physical bit meaning
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7a90 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7a98 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb7a9c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7ab4 | store | 4 | `sp -0x18` | `ret(0xb7af8)` |
| `ldr` at 0xb7ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7ac4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7ac8 | store | 4 | `sp -0x20` | `ret(0xb7af8)` |
| `ldrb` at 0xb7ad0 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb7ad4 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb7aec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb7af0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb7af4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7af4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7af4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7aac | `(unknown callee) 0xb7af8` | `(r0 & 0xff)` | `sp-25` | `sp-25` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7ac0 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7af4 | `r0` | `?` |

## `ioExpRead`

- `ioExpRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7af8 to 0xb7b78.
  Established. The first two calls target 0xb82c8 and 0xb8348 (`bl` at 0xb7b1c, `bl` at 0xb7b30).
  The later `bne` takes the nonzero-byte path from `[ret(0xb8348)]:1` to 0xb86d4 (`bne` at
  0xb7b48, `bl` at 0xb7b60). That call has first candidate `[sp-28]` and second candidate `[sp-24]`
  (`bl` at 0xb7b60). Zero fall-through instead saves 0xfffffc11 at sp -0x24 (`str` at 0xb7b50). The
  physical read operation and data format remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7b0c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xb7b10 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb7b14 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7b24 | store | 4 | `sp -0x1c` | `ret(0xb82c8)` |
| `ldrb` at 0xb7b28 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7b38 | store | 4 | `sp -0x20` | `ret(0xb8348)` |
| `ldr` at 0xb7b3c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb7b40 | load | 1 | `ret(0xb8348) +0x0` | `-` |
| `ldr` at 0xb7b4c | load | 4 | `abs 0xb7b74` | `-` |
| `str` at 0xb7b50 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xb7b58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7b5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7b68 | store | 4 | `sp -0x24` | `ret(0xb86d4)` |
| `ldr` at 0xb7b6c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7b70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7b70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7b70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7b1c | `(unknown callee) 0xb82c8` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb7b30 | `(unknown callee) 0xb8348` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb7b60 | `(unknown callee) 0xb86d4` | `[sp-28]` | `[sp-24]` | `?` | `[ret(0xb8348)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7b48 | `[ret(0xb8348)]:1 != 0x0` | `[ret(0xb8348)]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7b70 | `r0` | `?` |
