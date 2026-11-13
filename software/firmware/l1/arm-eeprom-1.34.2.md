# ARM L1 EEPROM entry paths in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/eeprom-facts.txt) retain all tool sections.

## `eepromInit`

- `eepromInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3fc8 to 0xb40dc.
  Established. The first `beq` takes the zero-word path from 0x27eb5c past the initial call to
  0x1eed0 (`beq` at 0xb3fec, `bl` at 0xb4008). Two calls to 0x18ba8 supply word stores at 0x27eb60
  and 0x27eb68 (`bl` at 0xb4018, `str` at 0xb4020, `bl` at 0xb406c, `str` at 0xb4074). The second
  call has r0 candidate `[sp-28]` and r1 candidate 0x78 (`bl` at 0xb406c). Later word stores write
  `[sp-24]` at 0x27eb5c and `[sp-28]` at 0x27eb64 (`str` at 0xb40a8, `str` at 0xb40b4). The globals'
  allocation and EEPROM roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3fd8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb3fdc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb3fe0 | load | 4 | `abs 0xb40bc` | `-` |
| `ldr` at 0xb3fe4 | load | 4 | `abs 0x27eb5c` | `-` |
| `ldr` at 0xb3ff0 | load | 4 | `abs 0xb40c0` | `-` |
| `str` at 0xb3ff4 | store | 4 | `sp -0x24` | `0x108cc4` |
| `ldr` at 0xb3ffc | load | 4 | `abs 0xb40c4` | `-` |
| `ldr` at 0xb400c | load | 4 | `abs 0xb40c8` | `-` |
| `ldr` at 0xb4010 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb4020 | store | 4 | `abs 0x27eb60` | `ret(0x18ba8)` |
| `ldr` at 0xb4024 | load | 4 | `abs 0x27eb60` | `-` |
| `ldr` at 0xb4030 | load | 4 | `abs 0xb40cc` | `-` |
| `str` at 0xb4034 | store | 4 | `sp -0x24` | `0x108cfc` |
| `ldr` at 0xb403c | load | 4 | `abs 0xb40c4` | `-` |
| `ldr` at 0xb404c | load | 4 | `abs 0xb40c8` | `-` |
| `ldr` at 0xb4050 | load | 4 | `abs 0x27eb60` | `-` |
| `ldr` at 0xb4058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4060 | load | 4 | `abs 0xb40d0` | `-` |
| `ldr` at 0xb4064 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb4074 | store | 4 | `abs 0x27eb68` | `ret(0x18ba8)` |
| `ldr` at 0xb4078 | load | 4 | `abs 0x27eb68` | `-` |
| `ldr` at 0xb4084 | load | 4 | `abs 0xb40d4` | `-` |
| `str` at 0xb4088 | store | 4 | `sp -0x24` | `0x108d1c` |
| `ldr` at 0xb4090 | load | 4 | `abs 0xb40c4` | `-` |
| `ldr` at 0xb40a0 | load | 4 | `abs 0xb40bc` | `-` |
| `ldr` at 0xb40a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb40a8 | store | 4 | `abs 0x27eb5c` | `[sp-24]` |
| `ldr` at 0xb40ac | load | 4 | `abs 0xb40d8` | `-` |
| `ldr` at 0xb40b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb40b4 | store | 4 | `abs 0x27eb64` | `[sp-28]` |
| `ldmdb` at 0xb40b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb40b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb40b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb40b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4008 | `(unknown callee) 0x1eed0` | `0x9` | `0x108ce0` | `0xd4` | `0x0` |
| `bl` at 0xb4018 | `(unknown callee) 0x18ba8` | `?` | `0x1` | `?` | `?` |
| `bl` at 0xb4048 | `(unknown callee) 0x1eed0` | `0x9` | `0x108ce0` | `0xdb` | `0x0` |
| `bl` at 0xb405c | `(unknown callee) 0x18c20` | `[0x27eb60]` | `0x1` | `[sp-28]` | `0x27eb60` |
| `bl` at 0xb406c | `(unknown callee) 0x18ba8` | `[sp-28]` | `0x78` | `?` | `?` |
| `bl` at 0xb409c | `(unknown callee) 0x1eed0` | `0x9` | `0x108ce0` | `0xe4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb3fec | `[0x27eb5c] == 0x0` | `[0x27eb5c] != 0x0` |
| `bne` at 0xb402c | `[0x27eb60] != 0x0` | `[0x27eb60] == 0x0` |
| `bne` at 0xb4080 | `[0x27eb68] != 0x0` | `[0x27eb68] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb40b8 | `r0` | `?` |

## `eepromDisable`

- `eepromDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb40dc to 0xb415c.
  Established. `bge` takes the path to 0x1eed0 for `(r0 & 0xff) >= [0x27eb64]` signed (`bge` at
  0xb4104, `bl` at 0xb4144). Fall-through instead writes byte 0x0 at `((r0 & 0xff) + [0x27eb60])
  +0x0` (`strb` at 0xb411c). The physical disable effect of that indexed byte is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb40f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb40f4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb40f8 | load | 4 | `abs 0xb414c` | `-` |
| `ldr` at 0xb40fc | load | 4 | `abs 0x27eb64` | `-` |
| `ldrb` at 0xb4108 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb410c | load | 4 | `abs 0xb4150` | `-` |
| `ldr` at 0xb4110 | load | 4 | `abs 0x27eb60` | `-` |
| `strb` at 0xb411c | store | 1 | `((r0 & 0xff) + [0x27eb60]) +0x0` | `0x0` |
| `ldr` at 0xb4124 | load | 4 | `abs 0xb4154` | `-` |
| `str` at 0xb4128 | store | 4 | `sp -0x1c` | `0x108d3c` |
| `ldrb` at 0xb412c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb4130 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xb4138 | load | 4 | `abs 0xb4158` | `-` |
| `ldmdb` at 0xb4148 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4148 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4144 | `(unknown callee) 0x1eed0` | `0x6` | `0x108ce0` | `0xf7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb4104 | `(r0 & 0xff) >= [0x27eb64] (signed, 32-bit)` | `(r0 & 0xff) < [0x27eb64] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4148 | `r0` | `?` |

## `eepromProbe`

- `eepromProbe`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb415c to 0xb4254.
  Established. The first call targets 0xb460c with r0 candidate `(r0 & 0xff)` (`bl` at 0xb4184).
  `bne` takes its nonzero-result path to 0xb466c, whose candidates are `[sp-21]:1`, 0x0, sp-36
  and 0x8 (`bne` at 0xb4190, `bl` at 0xb41b4). The later `beq` takes that second call's zero-result
  path past the byte-clear at `([sp-21]:1 + [0x27eb60]) +0x0` (`beq` at 0xb41c8, `strb` at 0xb422c).
  Another `beq` retains only Z == 1 (`beq` at 0xb41d4). The probe result's hardware meaning remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb4170 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `str` at 0xb4178 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0xb417c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb4194 | load | 4 | `abs 0xb4240` | `-` |
| `str` at 0xb4198 | store | 4 | `sp -0x28` | `0xfffffbb4` |
| `ldrb` at 0xb41a0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb41bc | store | 4 | `sp -0x1c` | `ret(0xb466c)` |
| `ldr` at 0xb41c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb41cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb41d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb41e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb41f4 | load | 4 | `abs 0xb4244` | `-` |
| `str` at 0xb41f8 | store | 4 | `sp -0x34` | `0x108d58` |
| `str` at 0xb41fc | store | 4 | `sp -0x30` | `ret(0xb4848)` |
| `str` at 0xb4200 | store | 4 | `sp -0x2c` | `ret(0x1e71c)` |
| `ldr` at 0xb4208 | load | 4 | `abs 0xb4248` | `-` |
| `ldr` at 0xb420c | load | 4 | `abs 0xb424c` | `-` |
| `ldrb` at 0xb4218 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb421c | load | 4 | `abs 0xb4250` | `-` |
| `ldr` at 0xb4220 | load | 4 | `abs 0x27eb60` | `-` |
| `strb` at 0xb422c | store | 1 | `([sp-21]:1 + [0x27eb60]) +0x0` | `0x0` |
| `ldr` at 0xb4230 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb4234 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb4238 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4184 | `(unknown callee) 0xb460c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb41b4 | `(unknown callee) 0xb466c` | `[sp-21]:1` | `0x0` | `sp-36` | `0x8` |
| `bl` at 0xb41e0 | `(unknown callee) 0xb4848` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |
| `bl` at 0xb41ec | `(unknown callee) 0x1e71c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb4214 | `(unknown callee) 0x1eed0` | `0x1` | `0x108ce0` | `0x107` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb4190 | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `beq` at 0xb41c8 | `ret(0xb466c) == 0x0` | `ret(0xb466c) != 0x0` |
| `beq` at 0xb41d4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb423c | `r0` | `?` |

## `eepromReadAllData`

- `eepromReadAllData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4254 to 0xb42cc.
  Established. The byte at sp -0x11 begins at 0x0 (`strb` at 0xb4268). The loop's `blt` predicate
  has unknown operands (`blt` at 0xb427c); its call to 0xb42cc has four unknown candidates (`bl` at
  0xb42ac). The back-edge follows a byte store of `([sp-17]:1+1 & 0xff)` at sp -0x11 (`strb` at
  0xb42b8, `b` at 0xb42bc). The iteration count and read-all-data semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb4268 | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0xb426c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4270 | load | 4 | `abs 0xb42c4` | `-` |
| `ldr` at 0xb4274 | load | 4 | `abs 0x27eb64` | `-` |
| `ldrb` at 0xb4284 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4288 | load | 4 | `abs 0xb42c8` | `-` |
| `ldrb` at 0xb428c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb42a0 | load | 4 | `abs 0x27eb68` | `-` |
| `ldrb` at 0xb42b0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb42b8 | store | 1 | `sp -0x11` | `([sp-17]:1+1 & 0xff)` |
| `ldmdb` at 0xb42c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb42c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb42c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb42ac | `(unknown callee) 0xb42cc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb427c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb42c0 | `r0` | `?` |

## `eepromReadData`

- `eepromReadData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb42cc to 0xb43f0.
  Established. The call to 0x18c20 has r0 candidate `[sp-24]`, r1 candidate 0x0 and r2 candidate
  0x78 (`bl` at 0xb4308). A subsequent word store at `[sp-24] +0x0` contains `[[sp-28]+4]:1` (`str`
  at 0xb4318). The `ldrls` taken condition is `[[sp-24]]-1 <= 0x3 (unsigned, 32-bit)`, and its taken
  edge is unresolved (`ldrls` at 0xb435c). Known fall-through stores 0xfffffbb1 at sp -0x20 (`str`
  at 0xb43d8). Additional dispatch effects and the record format remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb42e0 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xb42e4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb42e8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb42f8 | store | 4 | `sp -0x1c` | `ret(0xb4944)` |
| `ldr` at 0xb42fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb430c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4310 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb4314 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0xb4318 | store | 4 | `[sp-24] +0x0` | `[[sp-28]+4]:1` |
| `ldrb` at 0xb431c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb4328 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4340 | load | 4 | `abs 0xb43e4` | `-` |
| `str` at 0xb4344 | store | 4 | `sp -0x20` | `0xfffffbb4` |
| `ldr` at 0xb434c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4350 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xb435c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb43d4 | load | 4 | `abs 0xb43ec` | `-` |
| `str` at 0xb43d8 | store | 4 | `sp -0x20` | `0xfffffbb1` |
| `ldr` at 0xb43dc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb43e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb43e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb43e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb42f0 | `(unknown callee) 0xb4944` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb4308 | `(unknown callee) 0x18c20` | `[sp-24]` | `0x0` | `0x78` | `ret(0xb4944)` |
| `bl` at 0xb4330 | `(unknown callee) 0xb460c` | `[sp-17]:1` | `?` | `[sp-24]` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xb4324 | `[sp-17]:1 > 0xf4 (unsigned, 32-bit)` | `[sp-17]:1 <= 0xf4 (unsigned, 32-bit)` |
| `bne` at 0xb433c | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `ldrls` at 0xb435c | `[[sp-24]]-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-24]]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb43e0 | `r0` | `[sp-32]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb435c).

## `eepromFreeData`

- `eepromFreeData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb43f0 to 0xb4594.
  Established. A word load at entry r0 +0x0 precedes the conditional computed transfer (`ldr` at
  0xb4408, `ldrls` at 0xb4414); its `ldrls` taken condition is `[r0]-1 <= 0x3 (unsigned, 32-bit)`,
  with an unresolved taken edge (`ldrls` at 0xb4414). Known fall-through reaches 0x18c20 with
  candidates entry r0, 0x0, 0x78 and `[r0]-1` (`bl` at 0xb4588). The known return carries
  `ret(0x18c20)` (`ldmdb` at 0xb458c). Allocation-release semantics and additional transfer effects
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4400 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb4404 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4408 | load | 4 | `r0 +0x0` | `-` |
| `ldrls` at 0xb4414 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb457c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb458c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb458c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb458c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4588 | `(unknown callee) 0x18c20` | `r0` | `0x0` | `0x78` | `[r0]-1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb4414 | `[r0]-1 <= 0x3 (unsigned, 32-bit)` | `[r0]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb458c | `r0` | `ret(0x18c20)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb4414).

## `eepromGetCfg`

- `eepromGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4594 to 0xb45d0.
  Established. The word read at 0x27eb5c is stored as `[0x27eb5c]` at entry r0 +0x0 (`ldr` at
  0xb45b0, `str` at 0xb45b4). A separate word load from 0x27eb64 supplies return expression
  `[0x27eb64]` (`ldr` at 0xb45bc, `ldmdb` at 0xb45c4). The two words' configuration meanings are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb45a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb45a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb45ac | load | 4 | `abs 0xb45c8` | `-` |
| `ldr` at 0xb45b0 | load | 4 | `abs 0x27eb5c` | `-` |
| `str` at 0xb45b4 | store | 4 | `r0 +0x0` | `[0x27eb5c]` |
| `ldr` at 0xb45b8 | load | 4 | `abs 0xb45cc` | `-` |
| `ldr` at 0xb45bc | load | 4 | `abs 0x27eb64` | `-` |
| `ldmdb` at 0xb45c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb45c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb45c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb45c4 | `r0` | `[0x27eb64]` |

## `eepromGetData`

- `eepromGetData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb45d0 to 0xb460c.
  Established. A word load from 0x27eb68 supplies the store of `[0x27eb68]` at entry r0 +0x0 (`ldr`
  at 0xb45ec, `str` at 0xb45f0). The return instead carries the separate word load `[0x27eb64]`
  (`ldr` at 0xb45f8, `ldmdb` at 0xb4600). The returned and stored words' interpretations
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb45e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb45e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb45e8 | load | 4 | `abs 0xb4604` | `-` |
| `ldr` at 0xb45ec | load | 4 | `abs 0x27eb68` | `-` |
| `str` at 0xb45f0 | store | 4 | `r0 +0x0` | `[0x27eb68]` |
| `ldr` at 0xb45f4 | load | 4 | `abs 0xb4608` | `-` |
| `ldr` at 0xb45f8 | load | 4 | `abs 0x27eb64` | `-` |
| `ldmdb` at 0xb4600 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4600 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4600 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4600 | `r0` | `[0x27eb64]` |

## `eepromPresent`

- `eepromPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb460c to 0xb466c.
  Established. The signed `blt` takes the indexed-byte path for `(r0 & 0xff) < [0x27eb64]` (`blt` at
  0xb4634). That path reads a byte at `((r0 & 0xff) + [0x27eb60]) +0x0` and saves its expression as
  a word at sp -0x18 (`ldrb` at 0xb4654, `str` at 0xb4658). Fall-through saves 0x0 there and jumps
  to the join (`str` at 0xb463c, `b` at 0xb4640). The merged return and physical presence meaning
  remain unknown (`ldmdb` at 0xb4660).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb4620 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb4624 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4628 | load | 4 | `abs 0xb4664` | `-` |
| `ldr` at 0xb462c | load | 4 | `abs 0x27eb64` | `-` |
| `str` at 0xb463c | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0xb4644 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4648 | load | 4 | `abs 0xb4668` | `-` |
| `ldr` at 0xb464c | load | 4 | `abs 0x27eb60` | `-` |
| `ldrb` at 0xb4654 | load | 1 | `((r0 & 0xff) + [0x27eb60]) +0x0` | `-` |
| `str` at 0xb4658 | store | 4 | `sp -0x18` | `[((r0 & 0xff) + [0x27eb60])]:1` |
| `ldr` at 0xb465c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb4660 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4660 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4660 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb4634 | `(r0 & 0xff) < [0x27eb64] (signed, 32-bit)` | `(r0 & 0xff) >= [0x27eb64] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4660 | `r0` | `?` |

## `eepromRead`

- `eepromRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb466c to 0xb4848.
  Established. The first `bls` takes the path past 0x1eed0 for `([sp-20]:2 + [sp-26]:2) <=
  [ret(0xb4944)+2]:2` unsigned (`bls` at 0xb46c8, `bl` at 0xb470c). A later `bls` takes the path
  past the halfword store of 0x20 at sp -0x28 when `[sp-40]:2 <= 0x20` unsigned (`bls` at 0xb4780,
  `strh` at 0xb4788). The call to 0xaba40 has first candidate unknown, followed by `[sp-38]:1`,
  `[sp-24]` and `[sp-40]:2` (`bl` at 0xb47e8). The same path then calls 0xab788 (`bl` at 0xb47f4).
  The transfer units and EEPROM read protocol are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb467c | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb4688 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xb468c | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strh` at 0xb4690 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `str` at 0xb4698 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xb469c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb46ac | store | 4 | `sp -0x20` | `ret(0xb4944)` |
| `ldrh` at 0xb46b0 | load | 2 | `sp -0x14` | `-` |
| `ldrh` at 0xb46b4 | load | 2 | `sp -0x1a` | `-` |
| `ldr` at 0xb46bc | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb46c0 | load | 2 | `ret(0xb4944) +0x2` | `-` |
| `ldr` at 0xb46cc | load | 4 | `abs 0xb483c` | `-` |
| `str` at 0xb46d0 | store | 4 | `sp -0x40` | `0x108d74` |
| `ldr` at 0xb46d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb46d8 | load | 4 | `ret(0xb4944) +0x8` | `-` |
| `str` at 0xb46dc | store | 4 | `sp -0x3c` | `[ret(0xb4944)+8]` |
| `ldrh` at 0xb46e0 | load | 2 | `sp -0x14` | `-` |
| `str` at 0xb46e4 | store | 4 | `sp -0x38` | `[sp-20]:2` |
| `ldrh` at 0xb46e8 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xb46ec | store | 4 | `sp -0x34` | `[sp-26]:2` |
| `ldr` at 0xb46f0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb46f4 | load | 2 | `ret(0xb4944) +0x2` | `-` |
| `str` at 0xb46f8 | store | 4 | `sp -0x30` | `[ret(0xb4944)+2]:2` |
| `ldr` at 0xb4700 | load | 4 | `abs 0xb4840` | `-` |
| `ldr` at 0xb4704 | load | 4 | `abs 0xb4844` | `-` |
| `str` at 0xb4714 | store | 4 | `sp -0x2c` | `0x0` |
| `ldrh` at 0xb4720 | load | 2 | `sp -0x1a` | `-` |
| `ldr` at 0xb472c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb4738 | load | 2 | `sp -0x14` | `-` |
| `strb` at 0xb4744 | store | 1 | `sp -0x25` | `?` |
| `ldrb` at 0xb4748 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0xb474c | store | 1 | `sp -0x26` | `([sp-20]:1 & 0xff)` |
| `ldrb` at 0xb4750 | load | 1 | `sp -0x26` | `-` |
| `strh` at 0xb4758 | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0xb4764 | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0xb4770 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0xb4774 | store | 2 | `sp -0x28` | `([sp-26]:2 & 0xffff)` |
| `ldrh` at 0xb4778 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb4788 | store | 2 | `sp -0x28` | `0x20` |
| `ldr` at 0xb478c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb4790 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xb47a0 | store | 4 | `sp -0x24` | `ret(0xab648)` |
| `ldr` at 0xb47a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb47b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb47b4 | store | 4 | `sp -0x2c` | `ret(0xab648)` |
| `ldr` at 0xb47bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb47c0 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xb47c4 | load | 1 | `sp -0x25` | `-` |
| `ldrb` at 0xb47d0 | load | 1 | `sp -0x26` | `-` |
| `ldrh` at 0xb47d4 | load | 2 | `sp -0x28` | `-` |
| `ldr` at 0xb47e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb47f0 | store | 4 | `sp -0x24` | `ret(0xaba40)` |
| `ldrh` at 0xb47f8 | load | 2 | `sp -0x14` | `-` |
| `ldrh` at 0xb47fc | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb4804 | store | 2 | `sp -0x14` | `(([sp-20]:2 + [sp-40]:2) & 0xffff)` |
| `ldrh` at 0xb4808 | load | 2 | `sp -0x28` | `-` |
| `ldr` at 0xb480c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4814 | store | 4 | `sp -0x18` | `([sp-24] + [sp-40]:2)` |
| `ldrh` at 0xb4818 | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0xb481c | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb4824 | store | 2 | `sp -0x1a` | `?` |
| `ldr` at 0xb482c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb4830 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xb4834 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb4838 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4838 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb46a4 | `(unknown callee) 0xb4944` | `(r0 & 0xff)` | `r1` | `r3` | `(r0 & 0xff)` |
| `bl` at 0xb470c | `(unknown callee) 0x1eed0` | `0x6` | `0x108ce0` | `0x1cb` | `0x0` |
| `bl` at 0xb4798 | `(unknown callee) 0xab648` | `[[sp-32]]:1` | `?` | `?` | `[[sp-32]]:1` |
| `bl` at 0xb47e8 | `(unknown callee) 0xaba40` | `?` | `[sp-38]:1` | `[sp-24]` | `[sp-40]:2` |
| `bl` at 0xb47f4 | `(unknown callee) 0xab788` | `ret(0xaba40)` | `?` | `?` | `ret(0xaba40)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb46c8 | `([sp-20]:2 + [sp-26]:2) <= [ret(0xb4944)+2]:2 (unsigned, 32-bit)` | `([sp-20]:2 + [sp-26]:2) > [ret(0xb4944)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xb4728 | `[sp-26]:2 == 0x0` | `[sp-26]:2 != 0x0` |
| `bne` at 0xb4734 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bls` at 0xb476c | `? <= [sp-26]:2 (unsigned, 32-bit)` | `? > [sp-26]:2 (unsigned, 32-bit)` |
| `bls` at 0xb4780 | `[sp-40]:2 <= 0x20 (unsigned, 32-bit)` | `[sp-40]:2 > 0x20 (unsigned, 32-bit)` |
| `beq` at 0xb47ac | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4838 | `r0` | `?` |

## `eepromName`

- `eepromName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4848 to 0xb48a8.
  Established. After the call to 0xb4944, the first word read at `ret(0xb4944) +0x8` is saved at sp
  -0x1c (`bl` at 0xb4868, `ldr` at 0xb4878, `str` at 0xb487c). A second word load from the same
  effective location decides `bne`, whose taken nonzero path skips the fallback store of 0x108db4
  (`ldr` at 0xb4884, `bne` at 0xb488c, `str` at 0xb4894). The loads need not agree. The loaded
  word's name-string interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb485c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb4860 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb4870 | store | 4 | `sp -0x18` | `ret(0xb4944)` |
| `ldr` at 0xb4874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4878 | load | 4 | `ret(0xb4944) +0x8` | `-` |
| `str` at 0xb487c | store | 4 | `sp -0x1c` | `[ret(0xb4944)+8]` |
| `ldr` at 0xb4880 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4884 | load | 4 | `ret(0xb4944) +0x8` | `-` |
| `ldr` at 0xb4890 | load | 4 | `abs 0xb48a4` | `-` |
| `str` at 0xb4894 | store | 4 | `sp -0x1c` | `0x108db4` |
| `ldr` at 0xb4898 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb48a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb48a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb48a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4868 | `(unknown callee) 0xb4944` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb488c | `[ret(0xb4944)+8] != 0x0` | `[ret(0xb4944)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb48a0 | `r0` | `?` |

## `eepromStatus`

- `eepromStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb48a8 to 0xb4944.
  Established. The `ldrls` taken condition is `r0 <= 0x4 (unsigned, 32-bit)`, and its taken edge is
  unresolved (`ldrls` at 0xb48c4). Known fall-through loads the literal at 0xb4940 and saves
  0x108e08 at sp -0x18 (`ldr` at 0xb491c, `str` at 0xb4920). The known return has r0 value 0x108e08
  (`ldmdb` at 0xb4928). The status encoding and additional transfer effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb48b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb48bc | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xb48c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb491c | load | 4 | `abs 0xb4940` | `-` |
| `str` at 0xb4920 | store | 4 | `sp -0x18` | `0x108e08` |
| `ldr` at 0xb4924 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb4928 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4928 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4928 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb48c4 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4928 | `r0` | `0x108e08` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb48c4).

## `eeprom_get_cfg`

- `eeprom_get_cfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4944 to 0xb4ad0.
  Established. The conditional computed transfer has taken condition `(r0 & 0xff)-245 <= 0xa
  (unsigned, 32-bit)`, with an unresolved taken edge (`ldrls` at 0xb4968); its known fall-through
  later uses `blt` to take the signed `(r0 & 0xff) < [0x27eb64]` path past the call to 0x1eed0
  (`blt` at 0xb4a30, `bl` at 0xb4a60). The later word load from 0x27eb5c precedes an unknown-valued
  store at sp -0x18 (`ldr` at 0xb4a7c, `str` at 0xb4a84). The known return retains `[sp-24]`, while
  configuration interpretation and additional dispatch effects remain unknown (`ldmdb` at 0xb4a8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb4958 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb495c | load | 1 | `sp -0x11` | `-` |
| `ldrls` at 0xb4968 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb4a20 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4a24 | load | 4 | `abs 0xb4abc` | `-` |
| `ldr` at 0xb4a28 | load | 4 | `abs 0x27eb64` | `-` |
| `ldr` at 0xb4a34 | load | 4 | `abs 0xb4ac0` | `-` |
| `str` at 0xb4a38 | store | 4 | `sp -0x24` | `0x108e18` |
| `ldrb` at 0xb4a3c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb4a40 | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldr` at 0xb4a44 | load | 4 | `abs 0xb4abc` | `-` |
| `ldr` at 0xb4a48 | load | 4 | `abs 0x27eb64` | `-` |
| `str` at 0xb4a4c | store | 4 | `sp -0x1c` | `[0x27eb64]` |
| `ldr` at 0xb4a54 | load | 4 | `abs 0xb4ac4` | `-` |
| `ldr` at 0xb4a58 | load | 4 | `abs 0xb4ac8` | `-` |
| `ldr` at 0xb4a64 | load | 4 | `abs 0xb4acc` | `-` |
| `ldrb` at 0xb4a68 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb4a7c | load | 4 | `abs 0x27eb5c` | `-` |
| `str` at 0xb4a84 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb4a88 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb4a8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4a8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4a8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4a60 | `(unknown callee) 0x1eed0` | `0x9` | `0x108ce0` | `0x24e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb4968 | `(r0 & 0xff)-245 <= 0xa (unsigned, 32-bit)` | `(r0 & 0xff)-245 > 0xa (unsigned, 32-bit)` |
| `blt` at 0xb4a30 | `(r0 & 0xff) < [0x27eb64] (signed, 32-bit)` | `(r0 & 0xff) >= [0x27eb64] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4a8c | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb4968).

## `eepromWrite`

- `eepromWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5ef4 to 0xb6090.
  Established. The first `bls` takes the path past 0x1eed0 for `([sp-20]:2 + [sp-26]:2) <=
  [ret(0xb4944)+2]:2` unsigned (`bls` at 0xb5f50, `bl` at 0xb5f94). The later outgoing word store at
  sp -0x40 contains `[sp-40]:2` (`str` at 0xb6024). The call to 0xb6090 has candidates
  `[[sp-32]]:1`, unknown, `[sp-38]:1` and `[sp-24]` (`bl` at 0xb6038). Subsequent stores include
  `(([sp-20]:2 + [sp-40]:2) & 0xffff)` at sp -0x14 and `([sp-24] + [sp-40]:2)` at sp -0x18 (`strh`
  at 0xb6050, `str` at 0xb6060). The physical write protocol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5f04 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb5f10 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xb5f14 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strh` at 0xb5f18 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `str` at 0xb5f20 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xb5f24 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb5f34 | store | 4 | `sp -0x20` | `ret(0xb4944)` |
| `ldrh` at 0xb5f38 | load | 2 | `sp -0x14` | `-` |
| `ldrh` at 0xb5f3c | load | 2 | `sp -0x1a` | `-` |
| `ldr` at 0xb5f44 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb5f48 | load | 2 | `ret(0xb4944) +0x2` | `-` |
| `ldr` at 0xb5f54 | load | 4 | `abs 0xb6088` | `-` |
| `str` at 0xb5f58 | store | 4 | `sp -0x40` | `0x108f4c` |
| `ldr` at 0xb5f5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5f60 | load | 4 | `ret(0xb4944) +0x8` | `-` |
| `str` at 0xb5f64 | store | 4 | `sp -0x3c` | `[ret(0xb4944)+8]` |
| `ldrh` at 0xb5f68 | load | 2 | `sp -0x14` | `-` |
| `str` at 0xb5f6c | store | 4 | `sp -0x38` | `[sp-20]:2` |
| `ldrh` at 0xb5f70 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xb5f74 | store | 4 | `sp -0x34` | `[sp-26]:2` |
| `ldr` at 0xb5f78 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb5f7c | load | 2 | `ret(0xb4944) +0x2` | `-` |
| `str` at 0xb5f80 | store | 4 | `sp -0x30` | `[ret(0xb4944)+2]:2` |
| `ldr` at 0xb5f88 | load | 4 | `abs 0xb608c` | `-` |
| `str` at 0xb5f9c | store | 4 | `sp -0x2c` | `0x0` |
| `ldrh` at 0xb5fa8 | load | 2 | `sp -0x1a` | `-` |
| `ldr` at 0xb5fb4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb5fc0 | load | 2 | `sp -0x14` | `-` |
| `strb` at 0xb5fcc | store | 1 | `sp -0x25` | `?` |
| `ldrb` at 0xb5fd0 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0xb5fd4 | store | 1 | `sp -0x26` | `([sp-20]:1 & 0xff)` |
| `ldrb` at 0xb5fd8 | load | 1 | `sp -0x26` | `-` |
| `strh` at 0xb5fe0 | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0xb5fec | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0xb5ff8 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0xb5ffc | store | 2 | `sp -0x28` | `([sp-26]:2 & 0xffff)` |
| `ldr` at 0xb6000 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb6004 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb6008 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb600c | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xb6010 | load | 1 | `sp -0x25` | `-` |
| `ldrb` at 0xb601c | load | 1 | `sp -0x26` | `-` |
| `ldrh` at 0xb6020 | load | 2 | `sp -0x28` | `-` |
| `str` at 0xb6024 | store | 4 | `sp -0x40` | `[sp-40]:2` |
| `ldr` at 0xb6034 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6040 | store | 4 | `sp -0x24` | `ret(0xb6090)` |
| `ldrh` at 0xb6044 | load | 2 | `sp -0x14` | `-` |
| `ldrh` at 0xb6048 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb6050 | store | 2 | `sp -0x14` | `(([sp-20]:2 + [sp-40]:2) & 0xffff)` |
| `ldrh` at 0xb6054 | load | 2 | `sp -0x28` | `-` |
| `ldr` at 0xb6058 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6060 | store | 4 | `sp -0x18` | `([sp-24] + [sp-40]:2)` |
| `ldrh` at 0xb6064 | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0xb6068 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb6070 | store | 2 | `sp -0x1a` | `?` |
| `ldr` at 0xb6078 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb607c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xb6080 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb6084 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6084 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5f2c | `(unknown callee) 0xb4944` | `(r0 & 0xff)` | `r1` | `r3` | `(r0 & 0xff)` |
| `bl` at 0xb5f94 | `(unknown callee) 0x1eed0` | `0x6` | `0x108f8c` | `0x4d` | `0x0` |
| `bl` at 0xb6038 | `(unknown callee) 0xb6090` | `[[sp-32]]:1` | `?` | `[sp-38]:1` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb5f50 | `([sp-20]:2 + [sp-26]:2) <= [ret(0xb4944)+2]:2 (unsigned, 32-bit)` | `([sp-20]:2 + [sp-26]:2) > [ret(0xb4944)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xb5fb0 | `[sp-26]:2 == 0x0` | `[sp-26]:2 != 0x0` |
| `bne` at 0xb5fbc | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bls` at 0xb5ff4 | `? <= [sp-26]:2 (unsigned, 32-bit)` | `? > [sp-26]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6084 | `r0` | `?` |

## `eeprom_2m_Setup`

- `eeprom_2m_Setup`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba394 to 0xba3cc.
  Established. Two word stores place 0xba4bc at 0x115710 and 0xba3cc at 0x115714 (`str` at 0xba3a8,
  `str` at 0xba3b4). The return carries entry r0 (`ldmdb` at 0xba3b8). These stored addresses do not
  establish callable roles or a physical setup operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xba3a0 | load | 4 | `abs 0xba3bc` | `-` |
| `ldr` at 0xba3a4 | load | 4 | `abs 0xba3c0` | `-` |
| `str` at 0xba3a8 | store | 4 | `abs 0x115710` | `0xba4bc` |
| `ldr` at 0xba3ac | load | 4 | `abs 0xba3c4` | `-` |
| `ldr` at 0xba3b0 | load | 4 | `abs 0xba3c8` | `-` |
| `str` at 0xba3b4 | store | 4 | `abs 0x115714` | `0xba3cc` |
| `ldmdb` at 0xba3b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba3b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba3b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba3b8 | `r0` | `r0` |
