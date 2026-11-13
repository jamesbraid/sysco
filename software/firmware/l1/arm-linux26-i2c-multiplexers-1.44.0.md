# ARM Linux 2.6 I2C multiplexers in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
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
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-i2c-multiplexers-facts.txt) retain all tool sections.

## `i2cMuxInit`

- `i2cMuxInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x122e6c to 0x123094.
  Established. Included word assignments store `ret(0x6e140)` at `0x314fb8` and `ret(0x204d8)` at `0x314fb4` (`str` at 0x122efc, `str` at 0x122f30).
  A later word assignment writes `ret(0x204d8)` to `[sp-40] +0x8` (`str` at 0x122fb8).
  The unnamed `0x20550` call carries `[[sp-40]+8]`, `0x1`, `[[sp-40]+4]` and `[sp-40]` (`bl` at 0x123070).
  Multiplexer initialization hardware responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x122e7c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x122e80 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x122e84 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x122e88 | load | 4 | `abs 0x12307c` | `-` |
| `ldr` at 0x122e8c | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x122ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x122eac | load | 4 | `abs 0x123080` | `-` |
| `str` at 0x122eb0 | store | 4 | `sp -0x40` | `0x18f98c` |
| `ldr` at 0x122eb4 | load | 4 | `abs 0x12307c` | `-` |
| `ldr` at 0x122eb8 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x122ebc | store | 4 | `sp -0x3c` | `[0x314fb4]` |
| `ldr` at 0x122ec0 | load | 4 | `abs 0x123084` | `-` |
| `ldr` at 0x122ec4 | load | 4 | `abs 0x314fb8` | `-` |
| `str` at 0x122ec8 | store | 4 | `sp -0x38` | `[0x314fb8]` |
| `ldr` at 0x122ed0 | load | 4 | `abs 0x123088` | `-` |
| `ldr` at 0x122ee0 | load | 4 | `abs 0x123084` | `-` |
| `ldr` at 0x122ee4 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x122ef8 | load | 4 | `abs 0x123084` | `-` |
| `str` at 0x122efc | store | 4 | `abs 0x314fb8` | `ret(0x6e140)` |
| `ldr` at 0x122f00 | load | 4 | `abs 0x123084` | `-` |
| `ldr` at 0x122f04 | load | 4 | `abs 0x314fb8` | `-` |
| `str` at 0x122f18 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x122f1c | load | 4 | `abs 0x12307c` | `-` |
| `ldr` at 0x122f20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x122f30 | store | 4 | `abs 0x314fb4` | `ret(0x204d8)` |
| `ldr` at 0x122f34 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x122f40 | load | 4 | `abs 0x12308c` | `-` |
| `str` at 0x122f44 | store | 4 | `sp -0x40` | `0x18f9dc` |
| `ldr` at 0x122f48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x122f4c | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x122f54 | load | 4 | `abs 0x123088` | `-` |
| `ldr` at 0x122f64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x122f78 | load | 4 | `abs 0x12307c` | `-` |
| `ldr` at 0x122f7c | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x122f84 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x122f88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x122f8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x122f90 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x122f94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x122f98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x122f9c | store | 4 | `[sp-40] +0x4` | `[sp-32]` |
| `ldr` at 0x122fa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x122fa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x122fa8 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x122fb8 | store | 4 | `[sp-40] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x122fbc | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x122fc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x122fdc | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x122fe0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x122ff0 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x122ff4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12301c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x123020 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x123024 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x123028 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x123034 | load | 4 | `abs 0x123090` | `-` |
| `str` at 0x123038 | store | 4 | `(unknown)` | `0x18fa18` |
| `str` at 0x12303c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x123040 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123044 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x12304c | load | 4 | `abs 0x123088` | `-` |
| `ldr` at 0x12305c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x123060 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x123064 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x12306c | load | 4 | `[sp-40] +0x4` | `-` |
| `ldm` at 0x123078 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x123078 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x123078 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x123078 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x122e98 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x314fb4]` |
| `bl` at 0x122edc | `(unknown callee) 0x22070` | `0x9` | `0x18f9c0` | `0x63` | `0x0` |
| `bl` at 0x122ef0 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x314fb8]` |
| `bl` at 0x122f28 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `[0x314fb8]` | `?` |
| `bl` at 0x122f60 | `(unknown callee) 0x22070` | `0x9` | `0x18f9c0` | `0x6e` | `0x0` |
| `bl` at 0x122fb0 | `(unknown callee) 0x204d8` | `[[sp-40]+4]` | `0x1` | `[sp-40]` | `[sp-40]` |
| `bl` at 0x122fd8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x122fe4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x12302c | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x123058 | `(unknown callee) 0x22070` | `0x9` | `0x18f9c0` | `0x79` | `0x0` |
| `bl` at 0x123070 | `(unknown callee) 0x20550` | `[[sp-40]+8]` | `0x1` | `[[sp-40]+4]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x122e94 | `[0x314fb4] == 0x0` | `[0x314fb4] != 0x0` |
| `bne` at 0x122ea8 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x122eec | `[0x314fb8] != 0x0` | `[0x314fb8] == 0x0` |
| `bne` at 0x122f3c | `[0x314fb4] != 0x0` | `[0x314fb4] == 0x0` |
| `bne` at 0x122fc4 | `[[sp-40]+8] != 0x0` | `[[sp-40]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x123078 | `r0` | `ret(0x20550)` |

## `i2cMuxRemove`

- `i2cMuxRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123094 to 0x12315c.
  Established. Separate word assignments clear `[sp-24] +0x4`, `[sp-24] +0x0` and `[sp-24] +0x8` (`str` at 0x123124, `str` at 0x123130, `str` at 0x123148).
  The unnamed `0x20514` call carries `[[sp-24]+8]`, entry `r1`, `[sp-24]` and `[sp-24]` (`bl` at 0x12313c).
  Hardware removal and resource ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1230a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1230a8 | load | 4 | `abs 0x123154` | `-` |
| `ldr` at 0x1230ac | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x1230b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1230cc | load | 4 | `abs 0x123158` | `-` |
| `ldr` at 0x1230d0 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x1230d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1230e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1230fc | load | 4 | `abs 0x123158` | `-` |
| `ldr` at 0x123100 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123108 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x12310c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x123110 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x12311c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123124 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x123128 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123130 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x123134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x123138 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x123140 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123148 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldm` at 0x123150 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x123150 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x123150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12313c | `(unknown callee) 0x20514` | `[[sp-24]+8]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1230b4 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x1230e0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x123118 | `[[sp-24]+4] == 0x0` | `[[sp-24]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x123150 | `r0` | `?` |

## `i2cMuxNotPresent`

- `i2cMuxNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12315c to 0x1232b8.
  Established. The signed condition `(r1 & 0xff) >= [[sp-28]+4]` takes `0x12320c`, while lower falls through (`bge` at 0x1231ec).
  The fall-through path clears the byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`strb` at 0x123204).
  An unnamed `0x6e1b0` call carries entry `r0`, `0x1`, `0x0` and `0x0` (`bl` at 0x12321c).
  Presence-state hardware meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12316c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x123174 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x123178 | load | 4 | `abs 0x1232a8` | `-` |
| `ldr` at 0x12317c | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12319c | load | 4 | `abs 0x1232ac` | `-` |
| `ldr` at 0x1231a0 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x1231a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1231b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1231cc | load | 4 | `abs 0x1232ac` | `-` |
| `ldr` at 0x1231d0 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x1231d8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x1231dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1231e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1231e4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x1231f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1231f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1231f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0x123204 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0x12320c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x123220 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x123224 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123234 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x123238 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x123260 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x123264 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x123268 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12326c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x123278 | load | 4 | `abs 0x1232b0` | `-` |
| `str` at 0x12327c | store | 4 | `(unknown)` | `0x18fa58` |
| `str` at 0x123280 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x123284 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x123288 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x123290 | load | 4 | `abs 0x1232b4` | `-` |
| `ldm` at 0x1232a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1232a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1232a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12321c | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x123228 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x123270 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x12329c | `(unknown callee) 0x22070` | `0x6` | `0x18f9c0` | `0x9b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x123184 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x1231b0 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x1231ec | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1232a4 | `r0` | `?` |

## `i2cMuxProbe`

- `i2cMuxProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1232b8 to 0x123480.
  Established. Zero `ret(0x123770)` takes `0x123450`, while nonzero falls through (`beq` at 0x123370).
  The taken path assigns `0x1` to `sp -0x3c` (`str` at 0x123454).
  Another path clears the byte at `([sp-29]:1 + [[sp-44]+8]) +0x0` (`strb` at 0x123448).
  Probe acceptance at the hardware boundary remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1232c8 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0x1232d0 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0x1232d4 | load | 4 | `abs 0x123470` | `-` |
| `ldr` at 0x1232d8 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x1232e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1232f8 | load | 4 | `abs 0x123474` | `-` |
| `ldr` at 0x1232fc | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x123304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x123314 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x123328 | load | 4 | `abs 0x123474` | `-` |
| `ldr` at 0x12332c | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123334 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x123338 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x12333c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x123340 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x12334c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x123350 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x123364 | store | 4 | `sp -0x24` | `ret(0x123770)` |
| `ldr` at 0x123368 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x123374 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x123380 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x123394 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x123398 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1233a8 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x1233ac | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1233d4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1233d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1233dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1233e0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x1233ec | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x1233f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x123400 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12340c | load | 4 | `abs 0x123478` | `-` |
| `str` at 0x123410 | store | 4 | `(unknown)` | `0x18fa80` |
| `str` at 0x123414 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x123418 | store | 4 | `(unknown)` | `ret(0x1239fc)` |
| `str` at 0x12341c | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x123424 | load | 4 | `abs 0x12347c` | `-` |
| `ldr` at 0x123434 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x123438 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x12343c | load | 4 | `[sp-44] +0x8` | `-` |
| `strb` at 0x123448 | store | 1 | `([sp-29]:1 + [[sp-44]+8]) +0x0` | `0x0` |
| `str` at 0x123454 | store | 4 | `sp -0x3c` | `0x1` |
| `str` at 0x123460 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x123464 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x12346c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x12346c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12346c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12346c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12346c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12335c | `(unknown callee) 0x123770` | `r0` | `(r1 & 0xff)` | `0x0` | `(r1 & 0xff)` |
| `bl` at 0x123390 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x12339c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1233e4 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x1233f8 | `(unknown callee) 0x1239fc` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x123404 | `(unknown callee) 0x21ce0` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x123430 | `(unknown callee) 0x22070` | `0x1` | `0x18f9c0` | `0xae` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1232e0 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x12330c | `? != 0x0` | `? == 0x0` |
| `bge` at 0x123348 | `(r1 & 0xff) >= [[sp-44]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-44]+4] (signed, 32-bit)` |
| `beq` at 0x123370 | `ret(0x123770) == 0x0` | `ret(0x123770) != 0x0` |
| `beq` at 0x12337c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12346c | `r0` | `[sp-60]` |

## `i2cMuxReset`

- `i2cMuxReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123480 to 0x1234f4.
  Established. An unnamed `0x10a87c` call carries `0x0`, entry `r1`, entry `r2` and entry `r3` (`bl` at 0x123494);
  its zero result takes `0x1234dc`, while nonzero falls through (`beq` at 0x1234a8).
  The unnamed `0x123a68` call carries `ret(0x10a87c)`, unknown, unknown and `ret(0x10a87c)` (`bl` at 0x1234dc).
  Reset sequencing on the physical bus remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12349c | store | 4 | `sp -0x14` | `ret(0x10a87c)` |
| `ldr` at 0x1234a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1234ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1234b8 | load | 4 | `abs 0x1234ec` | `-` |
| `str` at 0x1234bc | store | 4 | `sp -0x1c` | `0x18faac` |
| `str` at 0x1234c0 | store | 4 | `sp -0x18` | `ret(0x21ce0)` |
| `ldr` at 0x1234c8 | load | 4 | `abs 0x1234f0` | `-` |
| `ldm` at 0x1234e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1234e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1234e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x123494 | `(unknown callee) 0x10a87c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x1234b0 | `(unknown callee) 0x21ce0` | `ret(0x10a87c)` | `?` | `?` | `ret(0x10a87c)` |
| `bl` at 0x1234d4 | `(unknown callee) 0x22070` | `0x1` | `0x18f9c0` | `0xc6` | `0x0` |
| `bl` at 0x1234dc | `(unknown callee) 0x123a68` | `ret(0x10a87c)` | `?` | `?` | `ret(0x10a87c)` |
| `bl` at 0x1234e0 | `(unknown callee) 0x10a9c0` | `ret(0x123a68)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1234a8 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1234e8 | `r0` | `?` |

## `i2cMuxGetCfg`

- `i2cMuxGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1234f4 to 0x1235f8.
  Established. A signed entry `r0 >= [0x314fb8]` takes `0x12354c`, while lower falls through (`bge` at 0x12351c).
  A word assignment places `0xfffffc11` at `sp -0x20` on a separate path (`str` at 0x123598).
  Another word assignment writes `[[sp-28]]` to entry `r1 +0x0` (`str` at 0x1235d0).
  Configuration record ownership and hardware roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x123504 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x123508 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x12350c | load | 4 | `abs 0x1235ec` | `-` |
| `ldr` at 0x123510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123514 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123520 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123534 | load | 4 | `abs 0x1235f0` | `-` |
| `ldr` at 0x123538 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x123540 | load | 4 | `(unknown)` | `-` |
| `str` at 0x123550 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x123558 | load | 4 | `abs 0x1235ec` | `-` |
| `ldr` at 0x12355c | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12357c | load | 4 | `abs 0x1235f0` | `-` |
| `ldr` at 0x123580 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x123588 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x123594 | load | 4 | `abs 0x1235f4` | `-` |
| `str` at 0x123598 | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0x1235a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1235b4 | load | 4 | `abs 0x1235f0` | `-` |
| `ldr` at 0x1235b8 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x1235c0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1235c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1235c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1235cc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x1235d0 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x1235d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1235d8 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x1235dc | store | 4 | `sp -0x20` | `[[sp-28]+4]` |
| `ldr` at 0x1235e0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1235e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1235e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1235e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x12351c | `r0 >= [0x314fb8] (signed, 32-bit)` | `r0 < [0x314fb8] (signed, 32-bit)` |
| `bne` at 0x123548 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x123564 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x123590 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1235e8 | `r0` | `?` |

## `i2cMuxIsPresent`

- `i2cMuxIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1236e8 to 0x123770.
  Established. The signed comparison `(r1 & 0xff) < [[sp-28]+4]` takes `0x123748`, while greater than or equal falls through (`blt` at 0x123738).
  The taken path reads a byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`ldrb` at 0x123758).
  The other path assigns zero to `sp -0x20` (`str` at 0x123740).
  Presence-byte encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1236f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x123700 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x123704 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123718 | load | 4 | `abs 0x12376c` | `-` |
| `ldr` at 0x12371c | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123724 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x123728 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x12372c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x123730 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x123740 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x123748 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x12374c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x123750 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x123758 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0x12375c | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0x123760 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x123768 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x123768 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x123768 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x123738 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x123768 | `r0` | `?` |

## `i2cMuxSelect`

- `i2cMuxSelect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123770 to 0x123900.
  Established. The unnamed `0x10a4ec` call carries `[sp-20]`, `[ret(0x1244a0)+1]:1`, `[sp-21]:1` and `[ret(0x1244a0)+1]:1` (`bl` at 0x123850).
  A later unnamed `0x123dbc` call carries `ret(0x6e190)`, `0x0`, `[sp-22]:1` and `ret(0x6e190)` (`bl` at 0x1238b0).
  Zero `ret(0x10a87c)` takes `0x123890`, while nonzero falls through (`beq` at 0x123880).
  Channel-selection effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x123780 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x123788 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x123790 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x123794 | load | 4 | `abs 0x1238f4` | `-` |
| `ldr` at 0x123798 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x1237a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1237b8 | load | 4 | `abs 0x1238f8` | `-` |
| `ldr` at 0x1237bc | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x1237c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1237d0 | load | 4 | `abs 0x1238fc` | `-` |
| `str` at 0x1237d4 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0x1237dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1237f0 | load | 4 | `abs 0x1238f8` | `-` |
| `ldr` at 0x1237f4 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x1237fc | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x123800 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x123804 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x123814 | store | 4 | `sp -0x1c` | `ret(0x1244a0)` |
| `ldr` at 0x123818 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x12381c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x123820 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldrb` at 0x123828 | load | 1 | `([sp-21]:1 + [[sp-40]+8]) +0x0` | `-` |
| `ldr` at 0x123834 | load | 4 | `abs 0x1238fc` | `-` |
| `str` at 0x123838 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0x123840 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x123844 | load | 1 | `ret(0x1244a0) +0x1` | `-` |
| `ldr` at 0x123848 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12385c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x123860 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x123874 | store | 4 | `sp -0x20` | `ret(0x10a87c)` |
| `ldr` at 0x123878 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x123884 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123888 | store | 4 | `sp -0x2c` | `ret(0x10a87c)` |
| `ldr` at 0x123890 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x1238a4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x1238b8 | store | 4 | `sp -0x20` | `ret(0x123dbc)` |
| `ldrb` at 0x1238c0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x1238c4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x1238c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1238d8 | store | 4 | `sp -0x20` | `ret(0x123fc0)` |
| `ldr` at 0x1238e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1238e4 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x1238e8 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x1238f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1238f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1238f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12380c | `(unknown callee) 0x1244a0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x123850 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[ret(0x1244a0)+1]:1` | `[sp-21]:1` | `[ret(0x1244a0)+1]:1` |
| `bl` at 0x12386c | `(unknown callee) 0x10a87c` | `?` | `?` | `?` | `?` |
| `bl` at 0x12389c | `(unknown callee) 0x6e190` | `ret(0x10a87c)` | `?` | `?` | `[sp-36]` |
| `bl` at 0x1238b0 | `(unknown callee) 0x123dbc` | `ret(0x6e190)` | `0x0` | `[sp-22]:1` | `ret(0x6e190)` |
| `bl` at 0x1238d0 | `(unknown callee) 0x123fc0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x1238dc | `(unknown callee) 0x10a9c0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1237a0 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x1237cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x123830 | `[([sp-21]:1 + [[sp-40]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-40]+8])]:1 == 0x0` |
| `beq` at 0x123880 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |
| `bne` at 0x123898 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1238f0 | `r0` | `?` |

## `i2cMuxRead`

- `i2cMuxRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123900 to 0x1239fc.
  Established. A word assignment stores `0xfffffc11` at `sp -0x28` (`str` at 0x123960).
  An included byte read addresses `([sp-21]:1 + [[sp-36]+8]) +0x0` (`ldrb` at 0x1239b4).
  The unnamed `0x124718` call carries `[sp-20]`, `ret(0x1244a0)`, `[sp-28]` and `[([sp-21]:1 + [[sp-36]+8])]:1` (`bl` at 0x1239d8).
  Multiplexer read data and wire behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x123910 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x123918 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x12391c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x123920 | load | 4 | `abs 0x1239f0` | `-` |
| `ldr` at 0x123924 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123944 | load | 4 | `abs 0x1239f4` | `-` |
| `ldr` at 0x123948 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x123950 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12395c | load | 4 | `abs 0x1239f8` | `-` |
| `str` at 0x123960 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x123968 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12397c | load | 4 | `abs 0x1239f4` | `-` |
| `ldr` at 0x123980 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123988 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x12398c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x123990 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1239a0 | store | 4 | `sp -0x20` | `ret(0x1244a0)` |
| `ldr` at 0x1239a4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x1239a8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1239ac | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0x1239b4 | load | 1 | `([sp-21]:1 + [[sp-36]+8]) +0x0` | `-` |
| `ldr` at 0x1239c0 | load | 4 | `abs 0x1239f8` | `-` |
| `str` at 0x1239c4 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x1239cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1239d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1239d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1239e0 | store | 4 | `sp -0x28` | `ret(0x124718)` |
| `ldr` at 0x1239e4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x1239ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1239ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1239ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x123998 | `(unknown callee) 0x1244a0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x1239d8 | `(unknown callee) 0x124718` | `[sp-20]` | `ret(0x1244a0)` | `[sp-28]` | `[([sp-21]:1 + [[sp-36]+8])]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12392c | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x123958 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x1239bc | `[([sp-21]:1 + [[sp-36]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-36]+8])]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1239ec | `r0` | `?` |

## `i2cMuxResetRaw`

- `i2cMuxResetRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123a68 to 0x123dbc.
  Established. Separate byte assignments clear `0x314fb0` and `0x314fb1` (`strb` at 0x123ba4, `strb` at 0x123d14).
  Equality `[0x1a0830]:1 == 0xfe` takes `0x123d84`, while inequality falls through (`beq` at 0x123d3c).
  The unnamed `0x124158` call carries `[0x314fb0]:1`, `[0x1a0830]:1`, `[0x314fb0]:1` and `[0x1a0830]:1` (`bl` at 0x123d58).
  Raw-reset hardware semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x123a7c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x123a80 | load | 4 | `abs 0x123d98` | `-` |
| `ldr` at 0x123a84 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123aa8 | load | 4 | `abs 0x123d9c` | `-` |
| `ldr` at 0x123aac | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x123ab4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x123ac0 | load | 4 | `abs 0x123da0` | `-` |
| `str` at 0x123ac4 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x123ae4 | load | 4 | `abs 0x123d9c` | `-` |
| `ldr` at 0x123ae8 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123af0 | store | 4 | `sp -0x24` | `?` |
| `strb` at 0x123af8 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x123afc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123b00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x123b04 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x123b10 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x123b14 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123b18 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0x123b20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x123b2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x123b30 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123b38 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x123b40 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x123b54 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x123b68 | store | 4 | `sp -0x18` | `ret(0x124158)` |
| `ldr` at 0x123b6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x123b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123b84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123b88 | store | 4 | `sp -0x14` | `ret(0x124158)` |
| `ldrb` at 0x123b8c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x123b94 | store | 1 | `sp -0x19` | `?` |
| `ldr` at 0x123b9c | load | 4 | `abs 0x123da4` | `-` |
| `strb` at 0x123ba4 | store | 1 | `abs 0x314fb0` | `0x0` |
| `ldr` at 0x123ba8 | load | 4 | `abs 0x123da8` | `-` |
| `strb` at 0x123bb0 | store | 1 | `abs 0x1a0830` | `?` |
| `ldr` at 0x123bb4 | load | 4 | `abs 0x123dac` | `-` |
| `strb` at 0x123bbc | store | 1 | `abs 0x1a0831` | `?` |
| `str` at 0x123bc4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x123bc8 | load | 4 | `abs 0x123d98` | `-` |
| `ldr` at 0x123bcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x123bd0 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123bdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x123bf0 | load | 4 | `abs 0x123d9c` | `-` |
| `ldr` at 0x123bf4 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123bfc | store | 4 | `sp -0x24` | `?` |
| `strb` at 0x123c04 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x123c08 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123c0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x123c10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x123c1c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x123c20 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123c24 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x123c2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x123c38 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x123c3c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123c44 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x123c4c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x123c58 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x123c5c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123c64 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x123c6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x123c70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123c90 | store | 4 | `sp -0x18` | `ret(0x10aab8)` |
| `ldr` at 0x123c94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x123ca0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123cac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123cb0 | store | 4 | `sp -0x14` | `ret(0x10aab8)` |
| `ldrb` at 0x123cb4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x123cb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123cc8 | store | 4 | `sp -0x18` | `ret(0x124158)` |
| `ldr` at 0x123ccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x123cd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123ce4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123ce8 | store | 4 | `sp -0x14` | `ret(0x124158)` |
| `ldrb` at 0x123cec | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x123cf4 | store | 1 | `sp -0x19` | `?` |
| `ldr` at 0x123cfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123d04 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x123d0c | load | 4 | `abs 0x123db0` | `-` |
| `strb` at 0x123d14 | store | 1 | `abs 0x314fb1` | `0x0` |
| `ldr` at 0x123d18 | load | 4 | `abs 0x123db4` | `-` |
| `strb` at 0x123d20 | store | 1 | `abs 0x1a0832` | `?` |
| `ldr` at 0x123d24 | load | 4 | `abs 0x123db8` | `-` |
| `strb` at 0x123d2c | store | 1 | `abs 0x1a0833` | `?` |
| `ldr` at 0x123d30 | load | 4 | `abs 0x123da8` | `-` |
| `ldrb` at 0x123d34 | load | 1 | `abs 0x1a0830` | `-` |
| `ldr` at 0x123d40 | load | 4 | `abs 0x123da4` | `-` |
| `ldrb` at 0x123d44 | load | 1 | `abs 0x314fb0` | `-` |
| `ldr` at 0x123d48 | load | 4 | `abs 0x123da8` | `-` |
| `ldrb` at 0x123d4c | load | 1 | `abs 0x1a0830` | `-` |
| `str` at 0x123d60 | store | 4 | `sp -0x18` | `ret(0x124158)` |
| `ldr` at 0x123d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x123d70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123d7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x123d80 | store | 4 | `sp -0x14` | `ret(0x124158)` |
| `ldr` at 0x123d84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x123d88 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0x123d8c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x123d94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x123d94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x123d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x123a90 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x314fb8]` |
| `bl` at 0x123acc | `(unknown callee) 0x6e190` | `ret(0x6e190)` | `?` | `?` | `?` |
| `bl` at 0x123b4c | `(unknown callee) 0x6e190` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x123b60 | `(unknown callee) 0x124158` | `ret(0x6e190)` | `[sp-25]:1` | `[sp-25]:1` | `ret(0x6e190)` |
| `bl` at 0x123c78 | `(unknown callee) 0x10a4ec` | `?` | `?` | `?` | `?` |
| `bl` at 0x123c88 | `(unknown callee) 0x10aab8` | `?` | `?` | `?` | `?` |
| `bl` at 0x123cc0 | `(unknown callee) 0x124158` | `[sp-32]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x123d58 | `(unknown callee) 0x124158` | `[0x314fb0]:1` | `[0x1a0830]:1` | `[0x314fb0]:1` | `[0x1a0830]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x123a8c | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x123abc | `? != 0x0` | `? == 0x0` |
| `bge` at 0x123b0c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x123b28 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x123b48 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x123b74 | `ret(0x124158) == 0x0` | `ret(0x124158) != 0x0` |
| `bne` at 0x123b80 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bge` at 0x123bd8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0x123c18 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x123c34 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x123c54 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x123c9c | `ret(0x10aab8) == 0x0` | `ret(0x10aab8) != 0x0` |
| `bne` at 0x123ca8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x123cd4 | `ret(0x124158) == 0x0` | `ret(0x124158) != 0x0` |
| `bne` at 0x123ce0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x123d3c | `[0x1a0830]:1 == 0xfe` | `[0x1a0830]:1 != 0xfe` |
| `beq` at 0x123d6c | `ret(0x124158) == 0x0` | `ret(0x124158) != 0x0` |
| `bne` at 0x123d78 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x123d94 | `r0` | `?` |

## `i2cMuxSelectRaw`

- `i2cMuxSelectRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123dbc to 0x123fc0.
  Established. Included byte assignments write `([sp-20] & 0xff)`, `([sp-21]:1 & 0xff)` and `([sp-22]:1 & 0xff)` to `0x314fb0`, `0x1a0830` and `0x1a0831` (`strb` at 0x123f6c, `strb` at 0x123f78, `strb` at 0x123f84).
  The unnamed `0x1245b8` call carries `[sp-28]`, `[sp-22]:1`, `0x1a0833` and `[sp-22]:1` (`bl` at 0x123f40);
  its zero result takes `0x123f64`, while nonzero falls through (`beq` at 0x123f54).
  The cached selection fields and physical routing remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x123dcc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x123dd4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x123ddc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x123de0 | load | 4 | `abs 0x123f9c` | `-` |
| `ldr` at 0x123de4 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123df0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123e04 | load | 4 | `abs 0x123fa0` | `-` |
| `ldr` at 0x123e08 | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x123e10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x123e1c | load | 4 | `abs 0x123fa4` | `-` |
| `str` at 0x123e20 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x123e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123e3c | load | 4 | `abs 0x123fa0` | `-` |
| `ldr` at 0x123e40 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x123e48 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x123e4c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x123e50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x123e60 | store | 4 | `sp -0x1c` | `ret(0x1244a0)` |
| `ldr` at 0x123e64 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x123e68 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x123e6c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0x123e74 | load | 1 | `([sp-21]:1 + [[sp-36]+8]) +0x0` | `-` |
| `ldr` at 0x123e80 | load | 4 | `abs 0x123fa4` | `-` |
| `str` at 0x123e84 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x123e8c | load | 4 | `abs 0x123fa8` | `-` |
| `ldrb` at 0x123e90 | load | 1 | `abs 0x314fb0` | `-` |
| `ldr` at 0x123e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x123ea0 | load | 4 | `abs 0x123fac` | `-` |
| `ldrb` at 0x123ea4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x123ea8 | load | 1 | `abs 0x1a0830` | `-` |
| `ldr` at 0x123eb4 | load | 4 | `abs 0x123fb0` | `-` |
| `ldrb` at 0x123eb8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x123ebc | load | 1 | `abs 0x1a0831` | `-` |
| `str` at 0x123ecc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x123ed4 | load | 4 | `abs 0x123fac` | `-` |
| `ldrb` at 0x123ed8 | load | 1 | `abs 0x1a0830` | `-` |
| `ldr` at 0x123ee4 | load | 4 | `abs 0x123fb0` | `-` |
| `ldrb` at 0x123ee8 | load | 1 | `abs 0x1a0831` | `-` |
| `ldr` at 0x123ef4 | load | 4 | `abs 0x123fa8` | `-` |
| `ldrb` at 0x123ef8 | load | 1 | `abs 0x314fb0` | `-` |
| `ldr` at 0x123efc | load | 4 | `abs 0x123fac` | `-` |
| `ldrb` at 0x123f00 | load | 1 | `abs 0x1a0830` | `-` |
| `ldr` at 0x123f10 | load | 4 | `abs 0x123fb4` | `-` |
| `strb` at 0x123f18 | store | 1 | `abs 0x314fb1` | `0x0` |
| `ldr` at 0x123f1c | load | 4 | `abs 0x123fb8` | `-` |
| `strb` at 0x123f24 | store | 1 | `abs 0x1a0832` | `?` |
| `ldr` at 0x123f28 | load | 4 | `abs 0x123fbc` | `-` |
| `strb` at 0x123f30 | store | 1 | `abs 0x1a0833` | `?` |
| `ldrb` at 0x123f34 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x123f38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x123f48 | store | 4 | `sp -0x20` | `ret(0x1245b8)` |
| `ldr` at 0x123f4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x123f58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x123f5c | store | 4 | `sp -0x28` | `ret(0x1245b8)` |
| `ldr` at 0x123f64 | load | 4 | `abs 0x123fa8` | `-` |
| `ldr` at 0x123f68 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x123f6c | store | 1 | `abs 0x314fb0` | `([sp-20] & 0xff)` |
| `ldr` at 0x123f70 | load | 4 | `abs 0x123fac` | `-` |
| `ldrb` at 0x123f74 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x123f78 | store | 1 | `abs 0x1a0830` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x123f7c | load | 4 | `abs 0x123fb0` | `-` |
| `ldrb` at 0x123f80 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x123f84 | store | 1 | `abs 0x1a0831` | `([sp-22]:1 & 0xff)` |
| `str` at 0x123f8c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x123f90 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x123f98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x123f98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x123f98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x123e58 | `(unknown callee) 0x1244a0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x123f0c | `(unknown callee) 0x124158` | `[0x314fb0]:1` | `[0x1a0830]:1` | `[0x314fb0]:1` | `[0x1a0830]:1` |
| `bl` at 0x123f40 | `(unknown callee) 0x1245b8` | `[sp-28]` | `[sp-22]:1` | `0x1a0833` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x123dec | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x123e18 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x123e7c | `[([sp-21]:1 + [[sp-36]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-36]+8])]:1 == 0x0` |
| `bne` at 0x123e9c | `[0x314fb0]:1 != [sp-20]` | `[0x314fb0]:1 == [sp-20]` |
| `bne` at 0x123eb0 | `[sp-21]:1 != [0x1a0830]:1` | `[sp-21]:1 == [0x1a0830]:1` |
| `bne` at 0x123ec4 | `[sp-22]:1 != [0x1a0831]:1` | `[sp-22]:1 == [0x1a0831]:1` |
| `beq` at 0x123ee0 | `[0x1a0830]:1 == 0xfe` | `[0x1a0830]:1 != 0xfe` |
| `beq` at 0x123ef0 | `[0x1a0831]:1 == 0x0` | `[0x1a0831]:1 != 0x0` |
| `beq` at 0x123f54 | `ret(0x1245b8) == 0x0` | `ret(0x1245b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x123f98 | `r0` | `?` |

## `i2cMuxSelectSubRaw`

- `i2cMuxSelectSubRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x123fc0 to 0x124158.
  Established. The unnamed `0x1245b8` call carries `ret(0x1244a0)`, `[sp-22]:1`, unknown and `[sp-22]:1` (`bl` at 0x1240e4).
  Inequality `[sp-22]:1 != [0x1a0833]:1` takes `0x1240d8`, while equality falls through (`bne` at 0x1240c8).
  Included byte assignments write `([sp-20] & 0xff)` to `0x314fb1` and `([sp-21]:1 & 0xff)` to `0x1a0832` (`strb` at 0x124110, `strb` at 0x12411c).
  Subchannel routing ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x123fd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x123fd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x123fe0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x123fe4 | load | 4 | `abs 0x124140` | `-` |
| `ldr` at 0x123fe8 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x123ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124008 | load | 4 | `abs 0x124144` | `-` |
| `ldr` at 0x12400c | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x124014 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x124020 | load | 4 | `abs 0x124148` | `-` |
| `str` at 0x124024 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x12402c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124040 | load | 4 | `abs 0x124144` | `-` |
| `ldr` at 0x124044 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x12404c | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x124050 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124054 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x124064 | store | 4 | `sp -0x1c` | `ret(0x1244a0)` |
| `ldr` at 0x124068 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x12406c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124070 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0x124078 | load | 1 | `([sp-21]:1 + [[sp-36]+8]) +0x0` | `-` |
| `ldr` at 0x124084 | load | 4 | `abs 0x124148` | `-` |
| `str` at 0x124088 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x124090 | load | 4 | `abs 0x12414c` | `-` |
| `ldrb` at 0x124094 | load | 1 | `abs 0x314fb1` | `-` |
| `ldr` at 0x124098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1240a4 | load | 4 | `abs 0x124150` | `-` |
| `ldrb` at 0x1240a8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x1240ac | load | 1 | `abs 0x1a0832` | `-` |
| `ldr` at 0x1240b8 | load | 4 | `abs 0x124154` | `-` |
| `ldrb` at 0x1240bc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x1240c0 | load | 1 | `abs 0x1a0833` | `-` |
| `str` at 0x1240d0 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0x1240d8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x1240dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1240ec | store | 4 | `sp -0x20` | `ret(0x1245b8)` |
| `ldr` at 0x1240f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1240fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x124100 | store | 4 | `sp -0x28` | `ret(0x1245b8)` |
| `ldr` at 0x124108 | load | 4 | `abs 0x12414c` | `-` |
| `ldr` at 0x12410c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x124110 | store | 1 | `abs 0x314fb1` | `([sp-20] & 0xff)` |
| `ldr` at 0x124114 | load | 4 | `abs 0x124150` | `-` |
| `ldrb` at 0x124118 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x12411c | store | 1 | `abs 0x1a0832` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x124120 | load | 4 | `abs 0x124154` | `-` |
| `ldrb` at 0x124124 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x124128 | store | 1 | `abs 0x1a0833` | `([sp-22]:1 & 0xff)` |
| `str` at 0x124130 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x124134 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x12413c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12413c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12413c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12405c | `(unknown callee) 0x1244a0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x1240e4 | `(unknown callee) 0x1245b8` | `ret(0x1244a0)` | `[sp-22]:1` | `?` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x123ff0 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x12401c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x124080 | `[([sp-21]:1 + [[sp-36]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-36]+8])]:1 == 0x0` |
| `bne` at 0x1240a0 | `[0x314fb1]:1 != [sp-20]` | `[0x314fb1]:1 == [sp-20]` |
| `bne` at 0x1240b4 | `[sp-21]:1 != [0x1a0832]:1` | `[sp-21]:1 == [0x1a0832]:1` |
| `bne` at 0x1240c8 | `[sp-22]:1 != [0x1a0833]:1` | `[sp-22]:1 == [0x1a0833]:1` |
| `beq` at 0x1240f8 | `ret(0x1245b8) == 0x0` | `ret(0x1245b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12413c | `r0` | `?` |

## `i2cMuxDeselectRaw`

- `i2cMuxDeselectRaw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124158 to 0x1243c0.
  Established. The unnamed `0x1245b8` call carries `ret(0x1244a0)`, `0x0`, `[sp-29]:1` and `[([sp-29]:1 + [[sp-44]+8])]:1` (`bl` at 0x124228);
  its zero result takes `0x124300`, while nonzero falls through (`beq` at 0x12423c).
  Separate byte assignments clear `0x314fb0` and `0x314fb1` (`strb` at 0x124340, `strb` at 0x124364).
  Deselect behavior on the physical multiplexer remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x124168 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0x124170 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0x124174 | load | 4 | `abs 0x124394` | `-` |
| `ldr` at 0x124178 | load | 4 | `abs 0x314fb8` | `-` |
| `ldr` at 0x124184 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x124198 | load | 4 | `abs 0x124398` | `-` |
| `ldr` at 0x12419c | load | 4 | `abs 0x314fb4` | `-` |
| `ldr` at 0x1241a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1241b0 | load | 4 | `abs 0x12439c` | `-` |
| `str` at 0x1241b4 | store | 4 | `sp -0x3c` | `0xfffffc11` |
| `ldr` at 0x1241bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1241d0 | load | 4 | `abs 0x124398` | `-` |
| `ldr` at 0x1241d4 | load | 4 | `abs 0x314fb4` | `-` |
| `str` at 0x1241dc | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x1241e0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x1241e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1241f4 | store | 4 | `sp -0x24` | `ret(0x1244a0)` |
| `ldr` at 0x1241f8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x1241fc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x124200 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x124208 | load | 1 | `([sp-29]:1 + [[sp-44]+8]) +0x0` | `-` |
| `ldr` at 0x124214 | load | 4 | `abs 0x12439c` | `-` |
| `str` at 0x124218 | store | 4 | `sp -0x3c` | `0xfffffc11` |
| `ldr` at 0x124220 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x124230 | store | 4 | `sp -0x28` | `ret(0x1245b8)` |
| `ldr` at 0x124234 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x124240 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x124254 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x124258 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x124268 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x12426c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x124294 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x124298 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12429c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1242a0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x1242ac | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x1242b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1242c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1242cc | load | 4 | `abs 0x1243a0` | `-` |
| `str` at 0x1242d0 | store | 4 | `(unknown)` | `0x18fad4` |
| `str` at 0x1242d4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x1242d8 | store | 4 | `(unknown)` | `ret(0x1239fc)` |
| `str` at 0x1242dc | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x1242e4 | load | 4 | `abs 0x1243a4` | `-` |
| `ldr` at 0x1242f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1242f8 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0x124300 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x124304 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x124310 | load | 4 | `abs 0x1243a8` | `-` |
| `ldrb` at 0x124314 | load | 1 | `abs 0x314fb0` | `-` |
| `ldr` at 0x124318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x124324 | load | 4 | `abs 0x1243ac` | `-` |
| `ldrb` at 0x124328 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x12432c | load | 1 | `abs 0x1a0830` | `-` |
| `ldr` at 0x124338 | load | 4 | `abs 0x1243a8` | `-` |
| `strb` at 0x124340 | store | 1 | `abs 0x314fb0` | `0x0` |
| `ldr` at 0x124344 | load | 4 | `abs 0x1243ac` | `-` |
| `strb` at 0x12434c | store | 1 | `abs 0x1a0830` | `?` |
| `ldr` at 0x124350 | load | 4 | `abs 0x1243b0` | `-` |
| `strb` at 0x124358 | store | 1 | `abs 0x1a0831` | `?` |
| `ldr` at 0x12435c | load | 4 | `abs 0x1243b4` | `-` |
| `strb` at 0x124364 | store | 1 | `abs 0x314fb1` | `0x0` |
| `ldr` at 0x124368 | load | 4 | `abs 0x1243b8` | `-` |
| `strb` at 0x124370 | store | 1 | `abs 0x1a0832` | `?` |
| `ldr` at 0x124374 | load | 4 | `abs 0x1243bc` | `-` |
| `strb` at 0x12437c | store | 1 | `abs 0x1a0833` | `?` |
| `str` at 0x124384 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x124388 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x124390 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x124390 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x124390 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124390 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1241ec | `(unknown callee) 0x1244a0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x124228 | `(unknown callee) 0x1245b8` | `ret(0x1244a0)` | `0x0` | `[sp-29]:1` | `[([sp-29]:1 + [[sp-44]+8])]:1` |
| `bl` at 0x124250 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x12425c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1242a4 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x1242b8 | `(unknown callee) 0x1239fc` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x1242c4 | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x1242f0 | `(unknown callee) 0x22070` | `0x1` | `0x18f9c0` | `0x234` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x124180 | `[0x314fb8] == 0x0` | `[0x314fb8] != 0x0` |
| `bne` at 0x1241ac | `? != 0x0` | `? == 0x0` |
| `bne` at 0x124210 | `[([sp-29]:1 + [[sp-44]+8])]:1 != 0x0` | `[([sp-29]:1 + [[sp-44]+8])]:1 == 0x0` |
| `beq` at 0x12423c | `ret(0x1245b8) == 0x0` | `ret(0x1245b8) != 0x0` |
| `bne` at 0x12430c | `[[sp-36]+1]:1 != 0x0` | `[[sp-36]+1]:1 == 0x0` |
| `bne` at 0x124320 | `[0x314fb0]:1 != [sp-28]` | `[0x314fb0]:1 == [sp-28]` |
| `bne` at 0x124334 | `[sp-29]:1 != [0x1a0830]:1` | `[sp-29]:1 == [0x1a0830]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124390 | `r0` | `?` |

## `i2cMuxSetSwitch`

- `i2cMuxSetSwitch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1243c0 to 0x124480.
  Established. The unsigned condition `(r0 & 0xff) <= 0xf` takes `0x1243f0`, while higher falls through (`bls` at 0x1243e0).
  Included byte assignments clear `0x314fb0` and `0x314fb1` (`strb` at 0x1243f8, `strb` at 0x12441c).
  A byte read addresses `[sp-17]:1 +0x18f97a` (`ldrb` at 0x124440).
  The table mapping from software switch values to hardware remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x1243d4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x1243d8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x1243e4 | load | 4 | `abs 0x124460` | `-` |
| `str` at 0x1243e8 | store | 4 | `sp -0x18` | `0xfffff573` |
| `ldr` at 0x1243f0 | load | 4 | `abs 0x124464` | `-` |
| `strb` at 0x1243f8 | store | 1 | `abs 0x314fb0` | `0x0` |
| `ldr` at 0x1243fc | load | 4 | `abs 0x124468` | `-` |
| `strb` at 0x124404 | store | 1 | `abs 0x1a0830` | `?` |
| `ldr` at 0x124408 | load | 4 | `abs 0x12446c` | `-` |
| `strb` at 0x124410 | store | 1 | `abs 0x1a0831` | `?` |
| `ldr` at 0x124414 | load | 4 | `abs 0x124470` | `-` |
| `strb` at 0x12441c | store | 1 | `abs 0x314fb1` | `0x0` |
| `ldr` at 0x124420 | load | 4 | `abs 0x124474` | `-` |
| `strb` at 0x124428 | store | 1 | `abs 0x1a0832` | `?` |
| `ldr` at 0x12442c | load | 4 | `abs 0x124478` | `-` |
| `strb` at 0x124434 | store | 1 | `abs 0x1a0833` | `?` |
| `ldr` at 0x124438 | load | 4 | `abs 0x12447c` | `-` |
| `ldrb` at 0x12443c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x124440 | load | 1 | `[sp-17]:1 +0x18f97a` | `-` |
| `str` at 0x124450 | store | 4 | `sp -0x18` | `ret(0x10a7d8)` |
| `ldr` at 0x124454 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x12445c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12445c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12445c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x124448 | `(unknown callee) 0x10a7d8` | `[[sp-17]:1+1636730]:1` | `r1` | `0x18f97a` | `[[sp-17]:1+1636730]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1243e0 | `(r0 & 0xff) <= 0xf (unsigned, 32-bit)` | `(r0 & 0xff) > 0xf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12445c | `r0` | `?` |

## `i2cMuxLastChannel`

- `i2cMuxLastChannel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124480 to 0x1244a0.
  Established. A byte read addresses `0x1a0831` (`ldrb` at 0x124490).
  The integer return candidate is `[0x1a0831]:1` (`ldm` at 0x124498).
  Last-channel state ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x12448c | load | 4 | `abs 0x12449c` | `-` |
| `ldrb` at 0x124490 | load | 1 | `abs 0x1a0831` | `-` |
| `ldm` at 0x124498 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124498 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124498 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124498 | `r0` | `[0x1a0831]:1` |
