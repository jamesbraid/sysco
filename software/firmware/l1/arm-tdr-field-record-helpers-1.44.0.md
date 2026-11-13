# ARM TDR field and record helpers in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/tdr-field-record-helpers-facts.txt) retain all tool sections.

## `tdrfld_create`

- `tdrfld_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3a04 to 0xe3a7c.
  Established. The unnamed `0x1943c` call carries `0xc` and entry `r1`, `r2`, `r3` (`bl` at 0xe3a1c).
  A nonzero `ret(0xe3a7c)` takes `0xe3a68`, while zero falls through (`bne` at 0xe3a50).
  An included word assignment preserves `[sp-24]` at `sp -0x1c` (`str` at 0xe3a6c).
  Field allocation lifetime remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3a14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3a20 | store | 4 | `sp -0x18` | `ret(0x1943c)` |
| `ldr` at 0xe3a24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3a34 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe3a3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3a54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3a60 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe3a68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3a6c | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xe3a70 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xe3a78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3a78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3a78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3a1c | `(unknown callee) 0x1943c` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xe3a44 | `(unknown callee) 0xe3a7c` | `ret(0x1943c)` | `[sp-20]` | `?` | `ret(0x1943c)` |
| `bl` at 0xe3a58 | `(unknown callee) 0x1970c` | `[sp-24]` | `?` | `?` | `ret(0xe3a7c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3a2c | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xe3a50 | `ret(0xe3a7c) != 0x0` | `ret(0xe3a7c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3a78 | `r0` | `?` |

## `tdrfld_init`

- `tdrfld_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3a7c to 0xe3b10.
  Established. A zero entry `r1` takes `0xe3acc`, while nonzero falls through (`beq` at 0xe3a9c).
  Halfword stores clear `[sp-24] +0x4` and `[sp-24] +0x6`, followed by a byte clear at `[sp-24] +0x8` (`strh` at 0xe3ae0, `strh` at 0xe3aec, `strb` at 0xe3af8).
  TDR field descriptor meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3a8c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe3a90 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe3a94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3aa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3aa4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3aac | store | 4 | `r0 +0x0` | `ret(0x192bc)` |
| `ldr` at 0xe3ab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3ab4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe3ac4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe3acc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3ad4 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xe3ad8 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xe3ae0 | store | 2 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xe3ae4 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xe3aec | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xe3af0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xe3af8 | store | 1 | `[sp-24] +0x8` | `0x0` |
| `str` at 0xe3b00 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xe3b04 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xe3b0c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe3b0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3b0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3aa8 | `(unknown callee) 0x192bc` | `r1` | `r1` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe3a9c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xe3abc | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3b0c | `r0` | `?` |

## `tdrfld_dump`

- `tdrfld_dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3b10 to 0xe3c28.
  Established. Halfword reads address `[sp-28] +0x4` and `[sp-28] +0x6` (`ldrh` at 0xe3bb8, `ldrh` at 0xe3bd0).
  An included unnamed `0x222f0` call carries `[sp-20]`, `0x122d98`, `[[sp-28]+8]:1`, `[[sp-28]+8]:1` (`bl` at 0xe3bfc).
  Field dump output destination remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3b20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3b24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3b28 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe3b2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3b3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3b48 | load | 4 | `abs 0xe3c08` | `-` |
| `ldr` at 0xe3b4c | load | 4 | `abs 0x12eeb4` | `-` |
| `str` at 0xe3b50 | store | 4 | `sp -0x14` | `[0x12eeb4]` |
| `ldr` at 0xe3b54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3b64 | load | 4 | `abs 0xe3c0c` | `-` |
| `ldr` at 0xe3b70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3b74 | load | 4 | `abs 0xe3c10` | `-` |
| `ldr` at 0xe3b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3b80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3b84 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe3b88 | store | 4 | `sp -0x20` | `[[sp-28]]` |
| `ldr` at 0xe3b8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3b90 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe3b9c | load | 4 | `abs 0xe3c14` | `-` |
| `str` at 0xe3ba0 | store | 4 | `sp -0x20` | `0x122d6c` |
| `ldr` at 0xe3ba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3ba8 | load | 4 | `abs 0xe3c18` | `-` |
| `ldr` at 0xe3bac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3bb4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3bb8 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xe3bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3bc0 | load | 4 | `abs 0xe3c1c` | `-` |
| `ldr` at 0xe3bcc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3bd0 | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xe3bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3bd8 | load | 4 | `abs 0xe3c20` | `-` |
| `ldr` at 0xe3be4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe3be8 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xe3bec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe3bf0 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xe3bf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3bf8 | load | 4 | `abs 0xe3c24` | `-` |
| `ldm` at 0xe3c04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3c04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3c04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3b68 | `(unknown callee) 0x222f0` | `?` | `0x122d48` | `r2` | `r1` |
| `bl` at 0xe3b7c | `(unknown callee) 0x222f0` | `?` | `0x122d68` | `r1` | `r1` |
| `bl` at 0xe3bb0 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122d74` | `?` | `?` |
| `bl` at 0xe3bc8 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122d80` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xe3be0 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122d8c` | `[[sp-28]+6]:2` | `[[sp-28]+6]:2` |
| `bl` at 0xe3bfc | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122d98` | `[[sp-28]+8]:1` | `[[sp-28]+8]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3b34 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xe3b44 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe3b5c | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xe3b98 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3c04 | `r0` | `?` |

## `tdrdef_create`

- `tdrdef_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3c28 to 0xe3ca8.
  Established. The unnamed `0xe3ca8` call carries `ret(0x1943c)`, `[sp-20]`, `[sp-24]`, `ret(0x1943c)` (`bl` at 0xe3c70).
  A nonzero `ret(0xe3ca8)` takes `0xe3c94`, while zero falls through (`bne` at 0xe3c7c).
  Definition constructor ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3c38 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3c3c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3c48 | store | 4 | `sp -0x1c` | `ret(0x1943c)` |
| `ldr` at 0xe3c4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3c5c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe3c64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3c68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3c6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3c80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3c8c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe3c94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3c98 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xe3c9c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xe3ca4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3ca4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3ca4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3c44 | `(unknown callee) 0x1943c` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xe3c70 | `(unknown callee) 0xe3ca8` | `ret(0x1943c)` | `[sp-20]` | `[sp-24]` | `ret(0x1943c)` |
| `bl` at 0xe3c84 | `(unknown callee) 0x1970c` | `[sp-28]` | `?` | `?` | `ret(0xe3ca8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3c54 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xe3c7c | `ret(0xe3ca8) != 0x0` | `ret(0xe3ca8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3ca4 | `r0` | `?` |

## `tdrdef_init`

- `tdrdef_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3ca8 to 0xe3db8.
  Established. Halfword stores write `([sp-32] & 0xffff)` to `[sp-24] +0x2` and `([sp-28] & 0xffff)` to `[sp-24] +0x0` (`strh` at 0xe3d20, `strh` at 0xe3da0).
  An included unnamed `0x19748` call carries `[[sp-24]+8]`, `0x0`, `[sp-36]`, `[sp-24]` (`bl` at 0xe3d94).
  Definition field-count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3cb8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe3cbc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xe3cc0 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xe3cc4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe3cd8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3cdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3ce0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3ce8 | store | 4 | `r0 +0x4` | `ret(0x1943c)` |
| `ldr` at 0xe3cec | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xe3cfc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe3d04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3d08 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3d10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3d1c | load | 4 | `sp -0x20` | `-` |
| `strh` at 0xe3d20 | store | 2 | `[sp-24] +0x2` | `([sp-32] & 0xffff)` |
| `ldr` at 0xe3d24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3d3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe3d4c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3d50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3d54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3d5c | store | 4 | `[sp-24] +0x8` | `ret(0x1943c)` |
| `ldr` at 0xe3d60 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe3d6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3d70 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xe3d7c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe3d84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3d88 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe3d90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3d98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3d9c | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xe3da0 | store | 2 | `[sp-24] +0x0` | `([sp-28] & 0xffff)` |
| `str` at 0xe3da8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xe3dac | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xe3db4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe3db4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3db4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3db4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3ce4 | `(unknown callee) 0x1943c` | `[sp-36]` | `r1` | `r2` | `?` |
| `bl` at 0xe3d14 | `(unknown callee) 0x19748` | `[[sp-24]+4]` | `0x0` | `[sp-36]` | `[sp-24]` |
| `bl` at 0xe3d58 | `(unknown callee) 0x1943c` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xe3d74 | `(unknown callee) 0x1970c` | `[[sp-24]+4]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xe3d94 | `(unknown callee) 0x19748` | `[[sp-24]+8]` | `0x0` | `[sp-36]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3cf4 | `[r0+4] != 0x0` | `[r0+4] == 0x0` |
| `bne` at 0xe3d68 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3db4 | `r0` | `?` |

## `tdrdef_dump`

- `tdrdef_dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3db8 to 0xe3f60.
  Established. Halfword reads address `[sp-28] +0x0` and `[sp-28] +0x2` (`ldrh` at 0xe3e2c, `ldrh` at 0xe3e44).
  An included unnamed `0xe8e78` call carries `[sp-20]`, `0x122df4`, `[sp-76]`, `[[sp-28]+8]` (`bl` at 0xe3f38).
  Definition dump field ordering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3dc8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3dcc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3dd0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe3dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3df0 | load | 4 | `abs 0xe3f44` | `-` |
| `ldr` at 0xe3df4 | load | 4 | `abs 0x12eeb4` | `-` |
| `str` at 0xe3df8 | store | 4 | `sp -0x14` | `[0x12eeb4]` |
| `ldr` at 0xe3dfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3e0c | load | 4 | `abs 0xe3f48` | `-` |
| `ldr` at 0xe3e18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3e1c | load | 4 | `abs 0xe3f4c` | `-` |
| `ldr` at 0xe3e20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e28 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3e2c | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe3e30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3e34 | load | 4 | `abs 0xe3f50` | `-` |
| `ldr` at 0xe3e40 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3e44 | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xe3e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3e4c | load | 4 | `abs 0xe3f54` | `-` |
| `str` at 0xe3e5c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe3e60 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3e64 | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xe3e68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3e7c | load | 4 | `abs 0xe3f58` | `-` |
| `ldr` at 0xe3e80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3e90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3ea4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xe3eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3eb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe3ec0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe3ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3ecc | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe3ee4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3ee8 | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xe3ef8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe3f00 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe3f04 | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xe3f14 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe3f18 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe3f20 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xe3f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3f28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3f2c | load | 4 | `abs 0xe3f5c` | `-` |
| `ldr` at 0xe3f30 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe3f34 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldm` at 0xe3f40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3f40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3f40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3e10 | `(unknown callee) 0x222f0` | `?` | `0x122db0` | `r2` | `r1` |
| `bl` at 0xe3e24 | `(unknown callee) 0x222f0` | `?` | `0x122d68` | `r1` | `r1` |
| `bl` at 0xe3e3c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122d8c` | `[[sp-28]]:2` | `[[sp-28]]:2` |
| `bl` at 0xe3e54 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122dd8` | `[[sp-28]+2]:2` | `[[sp-28]+2]:2` |
| `bl` at 0xe3e84 | `(unknown callee) 0x197a8` | `sp-72` | `0x122de4` | `?` | `sp-72` |
| `bl` at 0xe3eb4 | `(unknown callee) 0xe3b10` | `[sp-20]` | `sp-72` | `?` | `?` |
| `bl` at 0xe3f38 | `(unknown callee) 0xe8e78` | `[sp-20]` | `0x122df4` | `[sp-76]` | `[[sp-28]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3ddc | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xe3dec | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe3e04 | `r1 != 0x0` | `r1 == 0x0` |
| `bls` at 0xe3e70 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe3ee0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3f40 | `r0` | `?` |

## `tdrdef_get_field`

- `tdrdef_get_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe41a8 to 0xe4488.
  Established. A word assignment stores `0x80000000` at `sp -0x34` (`str` at 0xe42cc).
  An included word store clears an unknown effective target (`str` at 0xe4338).
  Inequality `[sp-56] != 0x80000000` takes `0xe4434`, while equality falls through (`bne` at 0xe4418).
  Field extraction bit numbering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe41b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe41bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe41c0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe41c4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe41c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe41cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe41e0 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe41e8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4218 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe4220 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe4230 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe4238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe423c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4250 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe4258 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe4268 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe426c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe4274 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe4278 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe427c | store | 4 | `sp -0x28` | `[sp-60]` |
| `ldr` at 0xe4280 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe4288 | store | 4 | `sp -0x30` | `[sp-60]-1` |
| `str` at 0xe4290 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xe4294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe42ac | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe42b4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe42b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe42c4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xe42cc | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xe42d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe42d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe42e8 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe42f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe42f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4300 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe4308 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe4310 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe4314 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4318 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe4324 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe432c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe4338 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe433c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4344 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe4350 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe4354 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4358 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe436c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe4374 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4378 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4384 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe438c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4394 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4398 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe43a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe43b0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe43b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe43c0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe43c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe43cc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe43d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe43dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe43e4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe43ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe43f4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe43fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe4404 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe440c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe4410 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe441c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe4424 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `str` at 0xe442c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xe4434 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe443c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe4440 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe444c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe4454 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe445c | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xe4464 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe446c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe4470 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4478 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0xe4484 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4484 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4484 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe41fc | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xe4320 | `C == 1` | `not (C == 1)` |
| `bls` at 0xe4380 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe43a4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe4418 | `[sp-56] != 0x80000000` | `[sp-56] == 0x80000000` |
| `bne` at 0xe4448 | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4484 | `r0` | `?` |

## `tdrdef_set_field`

- `tdrdef_set_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3f60 to 0xe41a8.
  Established. Included word stores have unknown values and effective targets (`str` at 0xe40f4, `str` at 0xe412c).
  Inequality `[sp-48] != 0x80000000` takes `0xe4154`, while equality falls through (`bne` at 0xe4138).
  Field replacement bit order remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3f70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3f74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3f78 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe3f7c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe3f80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3f84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3f98 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe3fa0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe3fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3fc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3fd4 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xe3fdc | load | 2 | `(unknown)` | `-` |
| `str` at 0xe3ff0 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xe3ff8 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe3ffc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4014 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xe401c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4020 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe402c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe4030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4048 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xe4050 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4054 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4068 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe4070 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe4074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4078 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe408c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xe4094 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4098 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe40a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe40ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe40b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe40b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe40c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe40d0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe40d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe40e0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe40e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe40ec | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe40f4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe40fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe4104 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe410c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe4114 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe411c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe4124 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe412c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe4130 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe413c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe4144 | store | 4 | `sp -0x28` | `[sp-40]-1` |
| `str` at 0xe414c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe4154 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe415c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe4160 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe416c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe4174 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe417c | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xe4184 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe418c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe4190 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4198 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0xe41a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe41a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe41a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe40a0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe40c4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe4138 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xe4168 | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe41a4 | `r0` | `?` |

## `tdrdef_find_field`

- `tdrdef_find_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4488 to 0xe4524.
  Established. A nonzero `ret(0x19310)` takes `0xe4500`, while zero falls through (`bne` at 0xe44f0).
  A word store increments `sp -0x1c` with `[sp-28]+1` (`str` at 0xe4508).
  The unnamed `0x19310` call has four unknown candidates (`bl` at 0xe44e4).
  Field search sentinel meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4498 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe449c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe44a4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe44a8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xe44ac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe44b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe44bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe44c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe44d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe44dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe44e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe44f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe44f8 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xe4500 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe4508 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe4510 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe4514 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe4518 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xe4520 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4520 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe44e4 | `(unknown callee) 0x19310` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe44b8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe44f0 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4520 | `r0` | `?` |

## `tdrdata_create`

- `tdrdata_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4524 to 0xe45b4.
  Established. The unnamed `0xe45b4` call carries `ret(0x1943c)`, `[sp-20]`, `[sp-22]:2`, `[sp-28]` (`bl` at 0xe457c).
  A halfword store preserves `(r1 & 0xffff)` at `sp -0x16` (`strh` at 0xe4540).
  Data constructor failure interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4534 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe453c | store | 4 | `sp -0x1c` | `r2` |
| `strh` at 0xe4540 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `str` at 0xe454c | store | 4 | `sp -0x20` | `ret(0x1943c)` |
| `ldr` at 0xe4550 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4560 | store | 4 | `sp -0x24` | `0x0` |
| `ldrh` at 0xe4568 | load | 2 | `sp -0x16` | `-` |
| `ldr` at 0xe456c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe458c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4598 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe45a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe45a4 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xe45a8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xe45b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe45b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe45b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe4548 | `(unknown callee) 0x1943c` | `0x14` | `r1` | `r2` | `r1` |
| `bl` at 0xe457c | `(unknown callee) 0xe45b4` | `ret(0x1943c)` | `[sp-20]` | `[sp-22]:2` | `[sp-28]` |
| `bl` at 0xe4590 | `(unknown callee) 0x1970c` | `[sp-32]` | `?` | `?` | `ret(0xe45b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe4558 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xe4588 | `ret(0xe45b4) != 0x0` | `ret(0xe45b4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe45b0 | `r0` | `?` |

## `tdrdata_init`

- `tdrdata_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe45b4 to 0xe4750.
  Established. An included word store writes `[sp-36]` to `[sp-24] +0x8` (`str` at 0xe4638).
  The unnamed `0x19748` call carries `[[sp-24]+12]`, `0x0`, `[[sp-36]+2]:2`, `[sp-24]` (`bl` at 0xe4730).
  A nonzero `[[sp-24]+12]` takes `0xe471c`, while zero falls through (`bne` at 0xe46e4).
  TDR data buffer capacity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe45c4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe45c8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xe45cc | store | 4 | `sp -0x24` | `r3` |
| `strh` at 0xe45d0 | store | 2 | `sp -0x1e` | `(r2 & 0xffff)` |
| `ldr` at 0xe45d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe45e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe45e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe45ec | store | 4 | `r0 +0x0` | `ret(0x192bc)` |
| `ldr` at 0xe45f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe45f4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe4604 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe460c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe4614 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xe4618 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xe461c | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xe4620 | store | 2 | `[sp-24] +0x4` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xe4624 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xe462c | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xe4630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4634 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4638 | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0xe463c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xe4640 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xe4660 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xe4664 | load | 2 | `[sp-36] +0x0` | `-` |
| `str` at 0xe467c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe4680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4684 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe468c | store | 4 | `[sp-24] +0x10` | `ret(0x1943c)` |
| `ldr` at 0xe4690 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xe469c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe46a0 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xe46ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe46b0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe46bc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe46c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe46c8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xe46cc | load | 2 | `[sp-36] +0x2` | `-` |
| `str` at 0xe46d8 | store | 4 | `[sp-24] +0xc` | `ret(0x1943c)` |
| `ldr` at 0xe46dc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xe46e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe46ec | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xe46f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe46fc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xe4704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4708 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xe4714 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe471c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4720 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xe4724 | load | 2 | `[sp-36] +0x2` | `-` |
| `ldr` at 0xe4728 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xe4734 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe4740 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe4744 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xe474c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe474c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe474c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe474c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe45e8 | `(unknown callee) 0x192bc` | `r1` | `r1` | `r2` | `r1` |
| `bl` at 0xe4688 | `(unknown callee) 0x1943c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xe46b4 | `(unknown callee) 0x1970c` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xe46d4 | `(unknown callee) 0x1943c` | `[[sp-36]+2]:2` | `?` | `?` | `[[sp-36]+2]:2` |
| `bl` at 0xe4700 | `(unknown callee) 0x1970c` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xe470c | `(unknown callee) 0x1970c` | `[[sp-24]+16]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xe4730 | `(unknown callee) 0x19748` | `[[sp-24]+12]` | `0x0` | `[[sp-36]+2]:2` | `[sp-24]` |
| `bl` at 0xe4738 | `(unknown callee) 0xe5150` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe45dc | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xe45fc | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0xe4698 | `[[sp-24]+16] != 0x0` | `[[sp-24]+16] == 0x0` |
| `beq` at 0xe46a8 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |
| `bne` at 0xe46e4 | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `beq` at 0xe46f4 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe474c | `r0` | `?` |

## `tdrdata_dump`

- `tdrdata_dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4750 to 0xe4a88.
  Established. Halfword reads address `[sp-28] +0x4` and `[sp-28] +0x6` (`ldrh` at 0xe47fc, `ldrh` at 0xe4818).
  An included unnamed `0xe4a88` call carries `[sp-20]`, `[sp-28]`, `0x1`, unknown (`bl` at 0xe4a4c).
  TDR dump flag meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4760 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe4764 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe4768 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe476c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe4770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe478c | load | 4 | `abs 0xe4a58` | `-` |
| `ldr` at 0xe4790 | load | 4 | `abs 0x12eeb4` | `-` |
| `str` at 0xe4794 | store | 4 | `sp -0x14` | `[0x12eeb4]` |
| `ldr` at 0xe4798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe47a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe47a8 | load | 4 | `abs 0xe4a5c` | `-` |
| `ldr` at 0xe47b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe47b8 | load | 4 | `abs 0xe4a60` | `-` |
| `ldr` at 0xe47bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe47c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe47c8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe47cc | store | 4 | `sp -0x34` | `[[sp-28]]` |
| `ldr` at 0xe47d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe47d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe47e0 | load | 4 | `abs 0xe4a64` | `-` |
| `str` at 0xe47e4 | store | 4 | `sp -0x34` | `0x122e34` |
| `ldr` at 0xe47e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe47ec | load | 4 | `abs 0xe4a68` | `-` |
| `ldr` at 0xe47f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe47f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe47fc | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xe4800 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe4804 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xe4808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe480c | load | 4 | `abs 0xe4a6c` | `-` |
| `ldr` at 0xe4814 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe4818 | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xe481c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xe4820 | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xe4824 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4828 | load | 4 | `abs 0xe4a70` | `-` |
| `ldr` at 0xe4830 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4844 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xe4850 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4858 | load | 4 | `abs 0xe4a74` | `-` |
| `ldr` at 0xe485c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xe4864 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4878 | load | 4 | `abs 0xe4a78` | `-` |
| `ldr` at 0xe4880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4884 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xe4888 | load | 2 | `[[sp-28]+8] +0x2` | `-` |
| `str` at 0xe4898 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe489c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe48a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xe48a4 | load | 2 | `[[sp-28]+8] +0x2` | `-` |
| `str` at 0xe48b4 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xe48bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe48c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe48c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe48d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe48d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe48e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe48e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe48f0 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xe48f8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe4900 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe4904 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4908 | load | 4 | `abs 0xe4a7c` | `-` |
| `ldr` at 0xe4914 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe491c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe4924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4928 | load | 4 | `abs 0xe4a80` | `-` |
| `ldr` at 0xe4930 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4938 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe4940 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe4950 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe4954 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4958 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe4964 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4968 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe4970 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xe4978 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xe4980 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe4984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4988 | load | 4 | `abs 0xe4a7c` | `-` |
| `ldr` at 0xe4994 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe499c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe49a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe49a8 | load | 4 | `abs 0xe4a80` | `-` |
| `ldr` at 0xe49b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe49b4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xe49b8 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `ldr` at 0xe49d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe49d4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xe49d8 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `str` at 0xe49e8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe49f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe49f4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xe49f8 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `str` at 0xe4a08 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe4a0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe4a14 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xe4a18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4a20 | load | 4 | `abs 0xe4a84` | `-` |
| `ldr` at 0xe4a24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe4a28 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xe4a30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4a44 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xe4a54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4a54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4a54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe47ac | `(unknown callee) 0x222f0` | `?` | `0x122e10` | `r2` | `r1` |
| `bl` at 0xe47c0 | `(unknown callee) 0x222f0` | `?` | `0x122e30` | `r1` | `r1` |
| `bl` at 0xe47f4 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122e3c` | `?` | `?` |
| `bl` at 0xe4810 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122e48` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xe482c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122e60` | `[[sp-28]+6]:2` | `[[sp-28]+6]:2` |
| `bl` at 0xe4860 | `(unknown callee) 0xe3db8` | `[sp-20]` | `0x122e78` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0xe487c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122e94` | `?` | `?` |
| `bl` at 0xe4910 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122eb0` | `?` | `?` |
| `bl` at 0xe492c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122eb8` | `?` | `?` |
| `bl` at 0xe4990 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122eb0` | `?` | `?` |
| `bl` at 0xe49ac | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122eb8` | `?` | `[sp-48]` |
| `bl` at 0xe4a2c | `(unknown callee) 0xe8e78` | `[sp-20]` | `0x122ebc` | `[sp-56]` | `[[sp-28]+16]` |
| `bl` at 0xe4a4c | `(unknown callee) 0xe4a88` | `[sp-20]` | `[sp-28]` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe4778 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xe4788 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe47a0 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xe47dc | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `beq` at 0xe483c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe484c | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xe4870 | `? == 0x0` | `? != 0x0` |
| `bhs` at 0xe48cc | `C == 1` | `not (C == 1)` |
| `bhi` at 0xe48e0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xe4948 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bhs` at 0xe4960 | `C == 1` | `not (C == 1)` |
| `bne` at 0xe49cc | `? != 0x0` | `? == 0x0` |
| `beq` at 0xe4a3c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4a54 | `r0` | `?` |

## `tdrdata_dump_fields`

- `tdrdata_dump_fields`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4a88 to 0xe4dc0.
  Established. Word stores write `0x0` and `[0x2974bc]+1` to `0x2974bc` (`str` at 0xe4bdc, `str` at 0xe4cdc).
  The unnamed `0xe4dc0` call carries unknown, `[sp-36]`, `sp-116`, unknown (`bl` at 0xe4d3c).
  Field-dump selection beyond the printed predicates remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4a98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe4a9c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe4aa0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe4aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4ab0 | load | 4 | `abs 0xe4db0` | `-` |
| `ldr` at 0xe4ab4 | load | 4 | `abs 0x12eeb4` | `-` |
| `str` at 0xe4ab8 | store | 4 | `sp -0x14` | `[0x12eeb4]` |
| `ldr` at 0xe4abc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4ac0 | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xe4ad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4ad4 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xe4ad8 | load | 2 | `[r1+8] +0x0` | `-` |
| `ldr` at 0xe4af0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4af4 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xe4af8 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xe4b08 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xe4b0c | load | 4 | `sp -0x78` | `-` |
| `str` at 0xe4b14 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xe4b1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4b20 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xe4b24 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xe4b34 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xe4b38 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xe4b44 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xe4b48 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xe4b50 | store | 4 | `sp -0x24` | `ret(0x1943c)` |
| `ldr` at 0xe4b54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4b64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4b68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4b6c | load | 4 | `abs 0xe4db4` | `-` |
| `ldr` at 0xe4b70 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe4b7c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe4b80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4b84 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xe4b88 | load | 2 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xe4b8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4b98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4bb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4bb4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xe4bb8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe4bc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe4bd4 | load | 4 | `abs 0xe4db8` | `-` |
| `str` at 0xe4bdc | store | 4 | `abs 0x2974bc` | `0x0` |
| `ldr` at 0xe4be0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4be4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe4be8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4bfc | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xe4c04 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4c1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4c20 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe4c24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4c38 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xe4c40 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4c50 | load | 4 | `abs 0xe4db8` | `-` |
| `ldr` at 0xe4c54 | load | 4 | `abs 0x2974bc` | `-` |
| `ldr` at 0xe4c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4c68 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe4c6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4c80 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xe4c88 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4c9c | load | 4 | `abs 0xe4db8` | `-` |
| `ldr` at 0xe4ca0 | load | 4 | `abs 0x2974bc` | `-` |
| `ldr` at 0xe4cb0 | load | 4 | `abs 0xe4db8` | `-` |
| `ldr` at 0xe4cb4 | load | 4 | `abs 0x2974bc` | `-` |
| `ldr` at 0xe4cbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4cc8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe4ccc | load | 4 | `abs 0xe4db8` | `-` |
| `ldr` at 0xe4cd0 | load | 4 | `abs 0xe4db8` | `-` |
| `ldr` at 0xe4cd4 | load | 4 | `abs 0x2974bc` | `-` |
| `str` at 0xe4cdc | store | 4 | `abs 0x2974bc` | `[0x2974bc]+1` |
| `ldr` at 0xe4ce4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4ce8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4cec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4cf0 | store | 4 | `sp -0x7c` | `[sp-36]` |
| `ldr` at 0xe4cf4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe4cf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4d00 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xe4d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4d0c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe4d10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4d24 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xe4d2c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe4d38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4d40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4d44 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe4d48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4d5c | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldr` at 0xe4d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4d68 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xe4d6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe4d74 | load | 1 | `([[sp-24]+12] + [sp-32]) +0x0` | `-` |
| `ldr` at 0xe4d7c | load | 4 | `abs 0xe4dbc` | `-` |
| `ldr` at 0xe4d80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4d90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4d98 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe4da0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xe4dac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4dac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4dac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe4b4c | `(unknown callee) 0x1943c` | `[sp-120]` | `r1` | `r2` | `?` |
| `bl` at 0xe4b74 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x122ed0` | `[[sp-24]]` | `[sp-24]` |
| `bl` at 0xe4d04 | `(unknown callee) 0xe41a8` | `[[sp-24]+8]` | `[sp-32]` | `0x0` | `[[sp-24]+16]` |
| `bl` at 0xe4d3c | `(unknown callee) 0xe4dc0` | `?` | `[sp-36]` | `sp-116` | `?` |
| `bl` at 0xe4d8c | `(unknown callee) 0x221f0` | `0x122ef0` | `?` | `[([[sp-24]+12] + [sp-32])]:1` | `sp-116` |
| `bl` at 0xe4da4 | `(unknown callee) 0x1970c` | `[sp-36]` | `?` | `[[[sp-24]+8]+2]:2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe4aac | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe4ac8 | `[r1+8] != 0x0` | `[r1+8] == 0x0` |
| `bne` at 0xe4aec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe4b5c | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bls` at 0xe4b94 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe4ba0 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xe4bac | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xe4bcc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe4c18 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe4c5c | `? > [0x2974bc] (unsigned, 32-bit)` | `? <= [0x2974bc] (unsigned, 32-bit)` |
| `bhi` at 0xe4ca8 | `? > [0x2974bc] (unsigned, 32-bit)` | `? <= [0x2974bc] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4dac | `r0` | `?` |

## `tdrdata_create_field_str`

- `tdrdata_create_field_str`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4dc0 to 0xe4ed4.
  Established. Included byte stores write an unknown value and `0x0` to unknown effective targets (`strb` at 0xe4e6c, `strb` at 0xe4ec4).
  Inequality `[sp-44] != 0x0` takes `0xe4ea0`, while equality falls through (`bne` at 0xe4e84).
  The integer return candidate retains entry `r0` (`ldm` at 0xe4ecc).
  Field-string bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4dd0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe4dd4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe4dd8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe4ddc | load | 4 | `abs 0xe4ed0` | `-` |
| `str` at 0xe4de0 | store | 4 | `sp -0x34` | `0x122f10` |
| `str` at 0xe4de8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe4dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4e00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe4e0c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe4e10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe4e20 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xe4e28 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe4e2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4e30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe4e3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4e40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4e48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4e50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4e54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe4e60 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe4e68 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe4e6c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe4e70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe4e78 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe4e7c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe4e8c | store | 4 | `sp -0x2c` | `0x1c` |
| `ldr` at 0xe4e90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe4e98 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe4ea0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe4ea8 | store | 4 | `sp -0x2c` | `[sp-44]-4` |
| `ldr` at 0xe4eac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4eb4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe4ebc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xe4ec4 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0xe4ecc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4ecc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4ecc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xe4e38 | `C == 1` | `not (C == 1)` |
| `bne` at 0xe4e84 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4ecc | `r0` | `r0` |

## `tdrdata_override_field`

- `tdrdata_override_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe53fc to 0xe5470.
  Established. A byte store writes an unknown value to `([r0+12] + r1) +0x0` (`strb` at 0xe5440).
  The unnamed `0xe3f60` call carries `[[sp-20]+8]`, `[sp-24]`, `[sp-28]`, `0x0` (`bl` at 0xe5464).
  The integer return candidate is `ret(0xe3f60)` (`ldm` at 0xe546c).
  Override flag interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe540c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5410 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe5414 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe5418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe541c | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xe5420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe542c | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xe5430 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xe5438 | load | 1 | `([r0+12] + r1) +0x0` | `-` |
| `strb` at 0xe5440 | store | 1 | `([r0+12] + r1) +0x0` | `?` |
| `ldr` at 0xe5444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe544c | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xe5450 | store | 4 | `sp -0x20` | `[[sp-20]+16]` |
| `ldr` at 0xe5454 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xe5458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe545c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xe546c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe546c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe546c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5464 | `(unknown callee) 0xe3f60` | `[[sp-20]+8]` | `[sp-24]` | `[sp-28]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe546c | `r0` | `ret(0xe3f60)` |

## `tdrdata_add_overrides`

- `tdrdata_add_overrides`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5470 to 0xe557c.
  Established. A byte read has an unknown effective target (`ldrb` at 0xe54bc).
  The unnamed `0xd5e9c` call presents four unknown candidates (`bl` at 0xe5560).
  A word store writes an unknown value to `sp -0x20` (`str` at 0xe556c).
  Bulk override placement remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5480 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5484 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe5488 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe5490 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe5494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5498 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe549c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe54a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe54ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe54b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe54b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe54bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe54cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe54d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe54d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe54e8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe54f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe54f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe54f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe54fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5500 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5514 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe551c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe5520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5528 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe552c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5530 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5544 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe554c | load | 2 | `(unknown)` | `-` |
| `str` at 0xe5550 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5554 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe555c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5564 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe556c | store | 4 | `sp -0x20` | `?` |
| `ldm` at 0xe5578 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe5578 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe5578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5560 | `(unknown callee) 0xd5e9c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe54a8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe54c8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe5578 | `r0` | `?` |

## `tdrdata_build_mask`

- `tdrdata_build_mask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe557c to 0xe5678.
  Established. A halfword read addresses `[r0+8] +0x0` (`ldrh` at 0xe55a4).
  The unnamed `0xd5c60` call carries entry `r2`, `[[r0+8]]:2`, entry `r3`, `[[r0+8]]:2` (`bl` at 0xe55b4).
  Another unnamed `0xd5d84` call carries two unknown candidates, `[sp-32]`, unknown (`bl` at 0xe565c).
  Mask construction coverage remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe558c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5590 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe5594 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe5598 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe559c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe55a0 | load | 4 | `r0 +0x8` | `-` |
| `ldrh` at 0xe55a4 | load | 2 | `[r0+8] +0x0` | `-` |
| `ldr` at 0xe55a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe55b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe55bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe55c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe55c4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrh` at 0xe55c8 | load | 2 | `[[sp-20]+8] +0x2` | `-` |
| `ldr` at 0xe55cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe55d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe55dc | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xe55e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xe55e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe55ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe55fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5600 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xe5604 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5618 | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xe5620 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe5624 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe562c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5630 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xe5634 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5648 | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xe5650 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe5658 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5660 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5668 | store | 4 | `sp -0x24` | `?` |
| `ldm` at 0xe5674 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe5674 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe5674 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe55b4 | `(unknown callee) 0xd5c60` | `r2` | `[[r0+8]]:2` | `r3` | `[[r0+8]]:2` |
| `bl` at 0xe565c | `(unknown callee) 0xd5d84` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe55d4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xe55f8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe5674 | `r0` | `?` |
