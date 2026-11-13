# ARM interrupts and image synchronization in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/interrupt-image-sync-facts.txt) retain all tool sections.

## `extIntInit`

- `extIntInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbba24 to 0xbbbbc.
  Established. The unnamed `0x196d0` call carries `[0x2972b4]:1`, `0x38`, `0x2972b4`, `[0x2972b4]:1` (`bl` at 0xbbabc).
  A word store records `ret(0x196d0)` at `0x2972b0` (`str` at 0xbbac0).
  Included stores write an unknown byte to `[sp-36] +0x5` and clear the word at `[sp-36] +0x8` (`strb` at 0xbbb60, `str` at 0xbbb88).
  Interrupt descriptor allocation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbba34 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbba38 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xbba3c | load | 4 | `abs 0xbbba4` | `-` |
| `ldrb` at 0xbba40 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbba4c | load | 4 | `abs 0xbbba8` | `-` |
| `str` at 0xbba50 | store | 4 | `sp -0x2c` | `0x11ee8c` |
| `ldr` at 0xbba54 | load | 4 | `abs 0xbbba4` | `-` |
| `ldrb` at 0xbba58 | load | 1 | `abs 0x2972b4` | `-` |
| `str` at 0xbba5c | store | 4 | `sp -0x28` | `[0x2972b4]:1` |
| `ldr` at 0xbba64 | load | 4 | `abs 0xbbbac` | `-` |
| `ldr` at 0xbba74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbba80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbba8c | load | 4 | `abs 0xbbbb0` | `-` |
| `str` at 0xbba90 | store | 4 | `sp -0x18` | `0x12d8e8` |
| `str` at 0xbba98 | store | 4 | `sp -0x1c` | `0x6` |
| `ldr` at 0xbba9c | load | 4 | `abs 0xbbba4` | `-` |
| `ldr` at 0xbbaa0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xbbaa4 | store | 1 | `abs 0x2972b4` | `?` |
| `ldr` at 0xbbaa8 | load | 4 | `abs 0xbbbb4` | `-` |
| `ldr` at 0xbbaac | load | 4 | `abs 0xbbba4` | `-` |
| `ldrb` at 0xbbab0 | load | 1 | `abs 0x2972b4` | `-` |
| `str` at 0xbbac0 | store | 4 | `abs 0x2972b0` | `ret(0x196d0)` |
| `ldr` at 0xbbac4 | load | 4 | `abs 0x2972b0` | `-` |
| `ldr` at 0xbbad0 | load | 4 | `abs 0xbbbb8` | `-` |
| `str` at 0xbbad4 | store | 4 | `sp -0x2c` | `0x11eec4` |
| `ldr` at 0xbbad8 | load | 4 | `abs 0xbbba4` | `-` |
| `ldrb` at 0xbbadc | load | 1 | `abs 0x2972b4` | `-` |
| `str` at 0xbbaf0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xbbaf8 | load | 4 | `abs 0xbbbac` | `-` |
| `str` at 0xbbb0c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbbb10 | load | 4 | `abs 0xbbba4` | `-` |
| `ldrb` at 0xbbb14 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbb18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbb24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbb38 | load | 4 | `abs 0xbbbb4` | `-` |
| `ldr` at 0xbbb3c | load | 4 | `abs 0x2972b0` | `-` |
| `str` at 0xbbb44 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbbb48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbb4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbb54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbbb5c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbbb60 | store | 1 | `[sp-36] +0x5` | `?` |
| `ldr` at 0xbbb64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbb68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbb70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbb78 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbbb7c | store | 4 | `[sp-36] +0xc` | `?` |
| `ldr` at 0xbbb80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbbb88 | store | 4 | `[sp-36] +0x8` | `0x0` |
| `ldr` at 0xbbb8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbbb94 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0xbbba0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbbba0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbba0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbba0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbba70 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0x60` | `0x0` |
| `bl` at 0xbbabc | `(unknown callee) 0x196d0` | `[0x2972b4]:1` | `0x38` | `0x2972b4` | `[0x2972b4]:1` |
| `bl` at 0xbbb04 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0x70` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbba48 | `[0x2972b4]:1 == 0x0` | `[0x2972b4]:1 != 0x0` |
| `beq` at 0xbba7c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbba88 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xbbacc | `[0x2972b0] != 0x0` | `[0x2972b0] == 0x0` |
| `ble` at 0xbbb20 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbba0 | `r0` | `?` |

## `extIntRegister`

- `extIntRegister`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbbbc to 0xbbd0c.
  Established. A word store writes `[sp-20]` to `[sp-28] +0x0` (`str` at 0xbbcc0).
  Byte stores write `([sp-22]:1 & 0xff)` to `[sp-28] +0x4` and `([sp-23]:1 & 0xff)` to `[sp-28] +0x5` (`strb` at 0xbbccc, `strb` at 0xbbcd8).
  The unsigned condition `[[sp-32]+44]:1 <= 0x2` takes `0xbbc80`, while greater falls through (`bls` at 0xbbc54).
  Interrupt registration capacity meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbbcc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xbbbd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xbbbe0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xbbbe8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xbbbec | load | 4 | `abs 0xbbcf8` | `-` |
| `ldrb` at 0xbbbf0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xbbbf4 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbc00 | load | 4 | `abs 0xbbcfc` | `-` |
| `str` at 0xbbc04 | store | 4 | `sp -0x28` | `0x11eef4` |
| `ldrb` at 0xbbc08 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xbbc0c | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0xbbc14 | load | 4 | `abs 0xbbd00` | `-` |
| `ldrb` at 0xbbc24 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbbc38 | load | 4 | `abs 0xbbd04` | `-` |
| `ldr` at 0xbbc3c | load | 4 | `abs 0x2972b0` | `-` |
| `str` at 0xbbc44 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbbc48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbbc4c | load | 1 | `[sp-32] +0x2c` | `-` |
| `ldr` at 0xbbc58 | load | 4 | `abs 0xbbd08` | `-` |
| `str` at 0xbbc5c | store | 4 | `sp -0x28` | `0x11ef38` |
| `ldr` at 0xbbc60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbbc64 | load | 1 | `[sp-32] +0x2c` | `-` |
| `str` at 0xbbc68 | store | 4 | `sp -0x24` | `[[sp-32]+44]:1` |
| `ldr` at 0xbbc70 | load | 4 | `abs 0xbbd00` | `-` |
| `ldrb` at 0xbbc80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbbc94 | load | 4 | `abs 0xbbd04` | `-` |
| `ldr` at 0xbbc98 | load | 4 | `abs 0x2972b0` | `-` |
| `ldrb` at 0xbbca0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbbca8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbbcb4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xbbcb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbbcbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbcc0 | store | 4 | `[sp-28] +0x0` | `[sp-20]` |
| `ldr` at 0xbbcc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbbcc8 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xbbccc | store | 1 | `[sp-28] +0x4` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0xbbcd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbbcd4 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xbbcd8 | store | 1 | `[sp-28] +0x5` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0xbbcdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbce0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbbce4 | load | 1 | `[sp-32] +0x2c` | `-` |
| `strb` at 0xbbcec | store | 1 | `[sp-32] +0x2c` | `([[sp-32]+44]:1+1 & 0xff)` |
| `ldm` at 0xbbcf4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbcf4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbcf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbc20 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0x8e` | `0x0` |
| `bl` at 0xbbc7c | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0x96` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbbbfc | `(r1 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |
| `bls` at 0xbbc54 | `[[sp-32]+44]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-32]+44]:1 > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbcf4 | `r0` | `?` |

## `extIntEnable`

- `extIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbda8 to 0xbbe84.
  Established. A byte store writes `0x1` to `[sp-24] +0x4` (`strb` at 0xbbe14).
  The unnamed `0x2066c` call carries `[sp-24]+16`, `[[sp-24]+12]`, `0xbc23c`, `0x1` (`bl` at 0xbbe5c).
  A nonzero `[[sp-24]+44]:1` takes `0xbbe0c`, while zero falls through (`bne` at 0xbbe04).
  Interrupt task activation guarantees remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbdbc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xbbdc0 | load | 4 | `abs 0xbbe78` | `-` |
| `ldrb` at 0xbbdc4 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbdc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbdd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbde8 | load | 4 | `abs 0xbbe7c` | `-` |
| `ldr` at 0xbbdec | load | 4 | `abs 0x2972b0` | `-` |
| `str` at 0xbbdf4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xbbdf8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbbdfc | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0xbbe0c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xbbe14 | store | 1 | `[sp-24] +0x4` | `0x1` |
| `ldr` at 0xbbe18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbe28 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `str` at 0xbbe30 | store | 4 | `sp -0x28` | `0x1000` |
| `str` at 0xbbe38 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0xbbe40 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xbbe48 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0xbbe50 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbbe54 | load | 4 | `abs 0xbbe80` | `-` |
| `ldr` at 0xbbe60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbe68 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0xbbe74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbe74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbe74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbe5c | `(unknown callee) 0x2066c` | `[sp-24]+16` | `[[sp-24]+12]` | `0xbc23c` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xbbdd0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xbbe04 | `[[sp-24]+44]:1 != 0x0` | `[[sp-24]+44]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbe74 | `r0` | `?` |

## `extIntIRQ`

- `extIntIRQ`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbfc4 to 0xbc054.
  Established. The unsigned comparison `(r0 & 0xff) < [0x2972b4]:1` takes `0xbc014`, while greater or equal falls through (`blo` at 0xbbfec).
  A word read addresses `0x2972b0` (`ldr` at 0xbc02c).
  A byte read has an unknown effective target (`ldrb` at 0xbc034).
  External interrupt IRQ mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbbfd8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbbfdc | load | 4 | `abs 0xbc044` | `-` |
| `ldrb` at 0xbbfe0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbbfe4 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbff0 | load | 4 | `abs 0xbc048` | `-` |
| `str` at 0xbbff4 | store | 4 | `sp -0x1c` | `0x11efdc` |
| `ldrb` at 0xbbff8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbbffc | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xbc004 | load | 4 | `abs 0xbc04c` | `-` |
| `ldrb` at 0xbc014 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbc028 | load | 4 | `abs 0xbc050` | `-` |
| `ldr` at 0xbc02c | load | 4 | `abs 0x2972b0` | `-` |
| `ldrb` at 0xbc034 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0xbc040 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbc040 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbc040 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbc010 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0xe8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbbfec | `(r0 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbc040 | `r0` | `?` |

## `extIntIsPolling`

- `extIntIsPolling`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc054 to 0xbc0e4.
  Established. A byte read addresses `0x2972b4` (`ldrb` at 0xbc074).
  The unsigned comparison `(r0 & 0xff) < [0x2972b4]:1` takes `0xbc0a4`, while greater or equal falls through (`blo` at 0xbc07c).
  A later byte read has an unknown effective target (`ldrb` at 0xbc0c4).
  Polling flag interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbc068 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbc06c | load | 4 | `abs 0xbc0d4` | `-` |
| `ldrb` at 0xbc070 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbc074 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbc080 | load | 4 | `abs 0xbc0d8` | `-` |
| `str` at 0xbc084 | store | 4 | `sp -0x1c` | `0x11f014` |
| `ldrb` at 0xbc088 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbc08c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xbc094 | load | 4 | `abs 0xbc0dc` | `-` |
| `ldrb` at 0xbc0a4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbc0b8 | load | 4 | `abs 0xbc0e0` | `-` |
| `ldr` at 0xbc0bc | load | 4 | `abs 0x2972b0` | `-` |
| `ldrb` at 0xbc0c4 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0xbc0d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbc0d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbc0d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbc0a0 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0xf2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbc07c | `(r0 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbc0d0 | `r0` | `?` |

## `extIntPollingMask`

- `extIntPollingMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc0e4 to 0xbc178.
  Established. A byte read addresses `0x2972b4` (`ldrb` at 0xbc108).
  A word read addresses `0x2972b0` (`ldr` at 0xbc130).
  Included word stores clear `sp -0x14` and later write an unknown value there (`str` at 0xbc0f8, `str` at 0xbc14c).
  Polling-mask bit assignments remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc0f8 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0xbc100 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xbc104 | load | 4 | `abs 0xbc170` | `-` |
| `ldrb` at 0xbc108 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbc10c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc118 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc12c | load | 4 | `abs 0xbc174` | `-` |
| `ldr` at 0xbc130 | load | 4 | `abs 0x2972b0` | `-` |
| `ldrb` at 0xbc138 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbc13c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc144 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbc14c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xbc150 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbc158 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xbc160 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbc16c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbc16c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbc16c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xbc114 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbc16c | `r0` | `?` |

## `extIntIgnoreSpurious`

- `extIntIgnoreSpurious`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc178 to 0xbc23c.
  Established. The unsigned comparison `(r0 & 0xff) < [0x2972b4]:1` takes `0xbc1cc`, while greater or equal falls through (`blo` at 0xbc1a4).
  A byte store has unknown value and effective target (`strb` at 0xbc21c).
  Spurious-interrupt suppression policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc18c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xbc190 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbc194 | load | 4 | `abs 0xbc228` | `-` |
| `ldrb` at 0xbc198 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbc19c | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbc1a8 | load | 4 | `abs 0xbc22c` | `-` |
| `str` at 0xbc1ac | store | 4 | `sp -0x20` | `0x11f050` |
| `ldrb` at 0xbc1b0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbc1b4 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xbc1bc | load | 4 | `abs 0xbc230` | `-` |
| `ldr` at 0xbc1c0 | load | 4 | `abs 0xbc234` | `-` |
| `ldrb` at 0xbc1cc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbc1e0 | load | 4 | `abs 0xbc238` | `-` |
| `ldr` at 0xbc1e4 | load | 4 | `abs 0x2972b0` | `-` |
| `ldrb` at 0xbc1ec | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbc200 | load | 4 | `abs 0xbc238` | `-` |
| `ldr` at 0xbc204 | load | 4 | `abs 0x2972b0` | `-` |
| `ldr` at 0xbc20c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbc210 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbc21c | store | 1 | `(unknown)` | `?` |
| `ldm` at 0xbc224 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbc224 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbc224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbc1c8 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0x10b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbc1a4 | `(r0 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbc224 | `r0` | `?` |

## `extIntMaxInts`

- `extIntMaxInts`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbe84 to 0xbbea4.
  Established. A byte read addresses `0x2972b4` (`ldrb` at 0xbbe94).
  The integer return candidate is `[0x2972b4]:1` (`ldm` at 0xbbe9c).
  Maximum interrupt index convention remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbbe90 | load | 4 | `abs 0xbbea0` | `-` |
| `ldrb` at 0xbbe94 | load | 1 | `abs 0x2972b4` | `-` |
| `ldm` at 0xbbe9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbe9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbe9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbe9c | `r0` | `[0x2972b4]:1` |

## `extIntSetPollTimeout`

- `extIntSetPollTimeout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbd0c to 0xbbda8.
  Established. A word store writes `[sp-24]` to `[sp-32] +0x8` (`str` at 0xbbd8c).
  The unsigned comparison `(r0 & 0xff) < [0x2972b4]:1` takes `0xbbd60`, while greater or equal falls through (`blo` at 0xbbd38).
  Poll timeout units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbd20 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xbbd24 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbbd28 | load | 4 | `abs 0xbbd98` | `-` |
| `ldrb` at 0xbbd2c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbbd30 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbd3c | load | 4 | `abs 0xbbd9c` | `-` |
| `str` at 0xbbd40 | store | 4 | `sp -0x28` | `0x11eef4` |
| `ldrb` at 0xbbd44 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbbd48 | store | 4 | `sp -0x24` | `(r0 & 0xff)` |
| `ldr` at 0xbbd50 | load | 4 | `abs 0xbbda0` | `-` |
| `ldrb` at 0xbbd60 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbbd74 | load | 4 | `abs 0xbbda4` | `-` |
| `ldr` at 0xbbd78 | load | 4 | `abs 0x2972b0` | `-` |
| `str` at 0xbbd80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbbd84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbd88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbd8c | store | 4 | `[sp-32] +0x8` | `[sp-24]` |
| `ldm` at 0xbbd94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbd94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbd94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbd5c | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0xac` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbbd38 | `(r0 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbd94 | `r0` | `?` |

## `imageSyncInit`

- `imageSyncInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcea34 to 0xceaa8.
  Established. Included word stores write an unknown value to `0x29c414` and `[sp-24]` to `0x297354` (`str` at 0xcea80, `str` at 0xcea8c).
  A zero `[0x297354]` takes `0xcea78`, while nonzero falls through (`beq` at 0xcea58).
  Image synchronization configuration ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcea44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcea48 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcea4c | load | 4 | `abs 0xcea98` | `-` |
| `ldr` at 0xcea50 | load | 4 | `abs 0x297354` | `-` |
| `ldr` at 0xcea5c | load | 4 | `abs 0xcea9c` | `-` |
| `str` at 0xcea60 | store | 4 | `sp -0x20` | `0x120bf4` |
| `ldr` at 0xcea68 | load | 4 | `abs 0xceaa0` | `-` |
| `ldr` at 0xcea78 | load | 4 | `abs 0xceaa4` | `-` |
| `ldr` at 0xcea7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcea80 | store | 4 | `abs 0x29c414` | `?` |
| `ldr` at 0xcea84 | load | 4 | `abs 0xcea98` | `-` |
| `ldr` at 0xcea88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcea8c | store | 4 | `abs 0x297354` | `[sp-24]` |
| `ldm` at 0xcea94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcea94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcea94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcea74 | `(unknown callee) 0x200a0` | `0x9` | `0x120c14` | `0x35` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcea58 | `[0x297354] == 0x0` | `[0x297354] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcea94 | `r0` | `?` |

## `imageSyncGetCfg`

- `imageSyncGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xceaa8 to 0xceae4.
  Established. A word store copies `[0x29c414]` to entry `r0 +0x0` (`str` at 0xceac8).
  The integer return candidate is `[0x297354]` (`ldm` at 0xcead8).
  Image synchronization configuration result structure remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xceab8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xceabc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xceac0 | load | 4 | `abs 0xceadc` | `-` |
| `ldr` at 0xceac4 | load | 4 | `abs 0x29c414` | `-` |
| `str` at 0xceac8 | store | 4 | `r0 +0x0` | `[0x29c414]` |
| `ldr` at 0xceacc | load | 4 | `abs 0xceae0` | `-` |
| `ldr` at 0xcead0 | load | 4 | `abs 0x297354` | `-` |
| `ldm` at 0xcead8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcead8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcead8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcead8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcead8 | `r0` | `[0x297354]` |

## `imageSyncCount`

- `imageSyncCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xceae4 to 0xceb04.
  Established. A word read addresses `0x297354` (`ldr` at 0xceaf4).
  The return record preserves `[0x297354]` (`ldm` at 0xceafc).
  Image synchronization count domain remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xceaf0 | load | 4 | `abs 0xceb00` | `-` |
| `ldr` at 0xceaf4 | load | 4 | `abs 0x297354` | `-` |
| `ldm` at 0xceafc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xceafc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xceafc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xceafc | `r0` | `[0x297354]` |

## `imageSyncClear`

- `imageSyncClear`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcebfc to 0xcecb8.
  Established. Included unnamed `0xbb408` calls carry `[ret(0xcedbc)]:1`, `[ret(0xcedbc)+1]:1`, `0x1`, `[ret(0xcedbc)+1]:1` and `[[sp-24]]:1`, `[[sp-24]+1]:1`, `0x0`, `[[sp-24]+1]:1` (`bl` at 0xcec5c, `bl` at 0xcec9c).
  A zero `ret(0xbb408)` takes `0xcec80`, while nonzero falls through (`beq` at 0xcec70).
  Image synchronization clear polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xcec10 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xcec14 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xcec24 | store | 4 | `sp -0x18` | `ret(0xcedbc)` |
| `ldr` at 0xcec28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcec34 | load | 4 | `abs 0xcecb4` | `-` |
| `str` at 0xcec38 | store | 4 | `sp -0x20` | `0xfffffc10` |
| `ldr` at 0xcec40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcec44 | load | 1 | `ret(0xcedbc) +0x0` | `-` |
| `ldr` at 0xcec48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcec4c | load | 1 | `ret(0xcedbc) +0x1` | `-` |
| `str` at 0xcec64 | store | 4 | `sp -0x1c` | `ret(0xbb408)` |
| `ldr` at 0xcec68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcec74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcec78 | store | 4 | `sp -0x20` | `ret(0xbb408)` |
| `ldr` at 0xcec80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcec84 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xcec88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcec8c | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xceca4 | store | 4 | `sp -0x20` | `ret(0xbb408)` |
| `ldr` at 0xceca8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xcecb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcecb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcecb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcec1c | `(unknown callee) 0xcedbc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xcec5c | `(unknown callee) 0xbb408` | `[ret(0xcedbc)]:1` | `[ret(0xcedbc)+1]:1` | `0x1` | `[ret(0xcedbc)+1]:1` |
| `bl` at 0xcec9c | `(unknown callee) 0xbb408` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `0x0` | `[[sp-24]+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcec30 | `ret(0xcedbc) != 0x0` | `ret(0xcedbc) == 0x0` |
| `beq` at 0xcec70 | `ret(0xbb408) == 0x0` | `ret(0xbb408) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcecb0 | `r0` | `?` |

## `imageSyncRead`

- `imageSyncRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcecb8 to 0xced38.
  Established. The unnamed `0xbb53c` call carries `[ret(0xcedbc)]:1`, `[ret(0xcedbc)+1]:1`, `[sp-24]`, `[ret(0xcedbc)+1]:1` (`bl` at 0xced1c).
  A nonzero `ret(0xcedbc)` takes `0xced00`, while zero falls through (`bne` at 0xcecf0).
  Image synchronization read result layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xceccc | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xcecd0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xcecd4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xcece4 | store | 4 | `sp -0x1c` | `ret(0xcedbc)` |
| `ldr` at 0xcece8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcecf4 | load | 4 | `abs 0xced34` | `-` |
| `str` at 0xcecf8 | store | 4 | `sp -0x20` | `0xfffffc10` |
| `ldr` at 0xced00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xced04 | load | 1 | `ret(0xcedbc) +0x0` | `-` |
| `ldr` at 0xced08 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xced0c | load | 1 | `ret(0xcedbc) +0x1` | `-` |
| `ldr` at 0xced18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xced24 | store | 4 | `sp -0x20` | `ret(0xbb53c)` |
| `ldr` at 0xced28 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xced30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xced30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xced30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcecdc | `(unknown callee) 0xcedbc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xced1c | `(unknown callee) 0xbb53c` | `[ret(0xcedbc)]:1` | `[ret(0xcedbc)+1]:1` | `[sp-24]` | `[ret(0xcedbc)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcecf0 | `ret(0xcedbc) != 0x0` | `ret(0xcedbc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xced30 | `r0` | `?` |

## `imageSyncName`

- `imageSyncName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xced38 to 0xcedbc.
  Established. A word read addresses `ret(0xcedbc) +0x4` (`ldr` at 0xced80).
  A nonzero `[ret(0xcedbc)+4]` takes `0xceda0`, while zero falls through (`bne` at 0xced94).
  Included word stores write `0x120c34` to `sp -0x1c` and `0x120c3c` to `sp -0x20` (`str` at 0xced74, `str` at 0xced9c).
  Synchronization name fallback contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xced4c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xced50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xced60 | store | 4 | `sp -0x18` | `ret(0xcedbc)` |
| `ldr` at 0xced64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xced70 | load | 4 | `abs 0xcedb4` | `-` |
| `str` at 0xced74 | store | 4 | `sp -0x1c` | `0x120c34` |
| `ldr` at 0xced7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xced80 | load | 4 | `ret(0xcedbc) +0x4` | `-` |
| `str` at 0xced84 | store | 4 | `sp -0x20` | `[ret(0xcedbc)+4]` |
| `ldr` at 0xced88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xced8c | load | 4 | `ret(0xcedbc) +0x4` | `-` |
| `ldr` at 0xced98 | load | 4 | `abs 0xcedb8` | `-` |
| `str` at 0xced9c | store | 4 | `sp -0x20` | `0x120c3c` |
| `ldr` at 0xceda0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xceda4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xceda8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xcedb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcedb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcedb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xced58 | `(unknown callee) 0xcedbc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xced6c | `ret(0xcedbc) != 0x0` | `ret(0xcedbc) == 0x0` |
| `bne` at 0xced94 | `[ret(0xcedbc)+4] != 0x0` | `[ret(0xcedbc)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcedb0 | `r0` | `?` |
