# ARM Linux 2.6 power presence and status in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-power-presence-facts.txt) retain all tool sections.

## `powerInit`

- `powerInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52878 to 0x52b34.
  Established. Included word assignments write `ret(0x6e140)` to `0x1a5c18` and `ret(0x204d8)` to `0x1a5c14` (`str` at 0x528f4, `str` at 0x52928).
  Inequality `[0x1a5c18] != 0x1` takes `0x52abc`, while equality falls through (`bne` at 0x52a90).
  The fall-through path assigns word `0x532fc` to `0x31a600` (`str` at 0x52a9c).
  Power record ownership and hardware initialization remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52888 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5288c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x52890 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x52894 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x52898 | load | 4 | `abs 0x52af0` | `-` |
| `ldr` at 0x5289c | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x528b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x528bc | load | 4 | `abs 0x52af4` | `-` |
| `str` at 0x528c0 | store | 4 | `sp -0x44` | `0x1617fc` |
| `ldr` at 0x528c8 | load | 4 | `abs 0x52af8` | `-` |
| `ldr` at 0x528d8 | load | 4 | `abs 0x52afc` | `-` |
| `ldr` at 0x528dc | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x528f0 | load | 4 | `abs 0x52afc` | `-` |
| `str` at 0x528f4 | store | 4 | `abs 0x1a5c18` | `ret(0x6e140)` |
| `ldr` at 0x528f8 | load | 4 | `abs 0x52afc` | `-` |
| `ldr` at 0x528fc | load | 4 | `abs 0x1a5c18` | `-` |
| `str` at 0x52910 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x52914 | load | 4 | `abs 0x52af0` | `-` |
| `ldr` at 0x52918 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x52928 | store | 4 | `abs 0x1a5c14` | `ret(0x204d8)` |
| `ldr` at 0x5292c | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x52938 | load | 4 | `abs 0x52b00` | `-` |
| `str` at 0x5293c | store | 4 | `sp -0x44` | `0x161834` |
| `ldr` at 0x52940 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x52944 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0x5294c | load | 4 | `abs 0x52af8` | `-` |
| `ldr` at 0x52950 | load | 4 | `abs 0x52b04` | `-` |
| `ldr` at 0x5295c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52970 | load | 4 | `abs 0x52af0` | `-` |
| `ldr` at 0x52974 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x5297c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x52980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x52984 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x52988 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0x5298c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x52990 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52994 | store | 4 | `[sp-44] +0x8` | `[sp-32]` |
| `ldr` at 0x52998 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5299c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x529a0 | store | 4 | `[sp-44] +0x10` | `[sp-36]` |
| `ldr` at 0x529a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x529a8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x529ac | store | 4 | `[sp-44] +0x14` | `[sp]` |
| `ldr` at 0x529b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x529b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x529b8 | load | 4 | `[sp-44] +0x8` | `-` |
| `str` at 0x529c8 | store | 4 | `[sp-44] +0x4` | `ret(0x204d8)` |
| `ldr` at 0x529cc | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x529d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x529ec | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x529f0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x52a00 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x52a04 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x52a2c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x52a30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x52a34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x52a38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x52a44 | load | 4 | `abs 0x52b08` | `-` |
| `str` at 0x52a48 | store | 4 | `(unknown)` | `0x161874` |
| `str` at 0x52a4c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x52a50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52a54 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x52a5c | load | 4 | `abs 0x52af8` | `-` |
| `ldr` at 0x52a60 | load | 4 | `abs 0x52b0c` | `-` |
| `ldr` at 0x52a6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x52a70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x52a74 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x52a7c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x52a84 | load | 4 | `abs 0x52afc` | `-` |
| `ldr` at 0x52a88 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x52a94 | load | 4 | `abs 0x52b10` | `-` |
| `ldr` at 0x52a98 | load | 4 | `abs 0x52b14` | `-` |
| `str` at 0x52a9c | store | 4 | `abs 0x31a600` | `0x532fc` |
| `ldr` at 0x52aa0 | load | 4 | `abs 0x52b18` | `-` |
| `ldr` at 0x52aa4 | load | 4 | `abs 0x52b1c` | `-` |
| `str` at 0x52aa8 | store | 4 | `abs 0x31a5fc` | `0x53a8c` |
| `ldr` at 0x52aac | load | 4 | `abs 0x52b20` | `-` |
| `ldr` at 0x52ab0 | load | 4 | `abs 0x52b24` | `-` |
| `str` at 0x52ab4 | store | 4 | `abs 0x31a604` | `0x5652c` |
| `ldr` at 0x52abc | load | 4 | `abs 0x52b10` | `-` |
| `ldr` at 0x52ac0 | load | 4 | `abs 0x52b28` | `-` |
| `str` at 0x52ac4 | store | 4 | `abs 0x31a600` | `0x54cfc` |
| `ldr` at 0x52ac8 | load | 4 | `abs 0x52b18` | `-` |
| `ldr` at 0x52acc | load | 4 | `abs 0x52b2c` | `-` |
| `str` at 0x52ad0 | store | 4 | `abs 0x31a5fc` | `0x553c0` |
| `ldr` at 0x52ad4 | load | 4 | `abs 0x52b20` | `-` |
| `ldr` at 0x52ad8 | load | 4 | `abs 0x52b30` | `-` |
| `str` at 0x52adc | store | 4 | `abs 0x31a604` | `0x56d20` |
| `ldm` at 0x52aec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x52aec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x52aec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x52aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x528a8 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a5c14]` |
| `bl` at 0x528d4 | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0x200` | `0x0` |
| `bl` at 0x528e8 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a5c18]` |
| `bl` at 0x52920 | `(unknown callee) 0x204d8` | `[sp-40]` | `0x1` | `[0x1a5c18]` | `?` |
| `bl` at 0x52958 | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0x209` | `0x0` |
| `bl` at 0x529c0 | `(unknown callee) 0x204d8` | `[[sp-44]+8]` | `0x1` | `[sp-44]` | `[sp-44]` |
| `bl` at 0x529e8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x529f4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x52a3c | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x52a68 | `(unknown callee) 0x22070` | `0x9` | `0x16181c` | `0x217` | `0x0` |
| `bl` at 0x52a80 | `(unknown callee) 0x20550` | `[[sp-44]+4]` | `0x1` | `[[sp-44]+8]` | `[sp-44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x528a4 | `[0x1a5c14] == 0x0` | `[0x1a5c14] != 0x0` |
| `bne` at 0x528b8 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x528e4 | `[0x1a5c18] != 0x0` | `[0x1a5c18] == 0x0` |
| `bne` at 0x52934 | `[0x1a5c14] != 0x0` | `[0x1a5c14] == 0x0` |
| `bne` at 0x529d4 | `[[sp-44]+4] != 0x0` | `[[sp-44]+4] == 0x0` |
| `bne` at 0x52a90 | `[0x1a5c18] != 0x1` | `[0x1a5c18] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x52aec | `r0` | `0x0` |

## `powerRemove`

- `powerRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52b34 to 0x52bfc.
  Established. Included word assignments clear `[sp-24] +0x8`, `[sp-24] +0x0` and `[sp-24] +0x4` (`str` at 0x52bc4, `str` at 0x52bd0, `str` at 0x52be8).
  The unnamed `0x20514` call carries `[[sp-24]+4]`, entry `r1`, `[sp-24]` and `[sp-24]` (`bl` at 0x52bdc).
  Zero `[0x1a5c18]` takes `0x52bec`, while nonzero falls through (`beq` at 0x52b54).
  Removal from physical power hardware remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52b44 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x52b48 | load | 4 | `abs 0x52bf4` | `-` |
| `ldr` at 0x52b4c | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x52b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52b6c | load | 4 | `abs 0x52bf8` | `-` |
| `ldr` at 0x52b70 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x52b78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x52b88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52b9c | load | 4 | `abs 0x52bf8` | `-` |
| `ldr` at 0x52ba0 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x52ba8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x52bac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52bb0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x52bbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52bc4 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x52bc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52bd0 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x52bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52bd8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x52be0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52be8 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldm` at 0x52bf0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x52bf0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x52bf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52bdc | `(unknown callee) 0x20514` | `[[sp-24]+4]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52b54 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x52b80 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x52bb8 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x52bf0 | `r0` | `?` |

## `powerNotPresent`

- `powerNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52bfc to 0x52d68.
  Established. The byte at `([[sp-28]+4] + r1) +0x0` is cleared (`strb` at 0x52d48).
  Signed entry `r1 < 0x0` takes `0x52c9c`, while nonnegative falls through (`blt` at 0x52c80).
  The unnamed `0x6e1b0` call carries entry `r0`, `0x1`, `0x0` and `0x0` (`bl` at 0x52cac).
  The absent-state relationship to physical power hardware remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52c0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52c10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52c14 | load | 4 | `abs 0x52d54` | `-` |
| `ldr` at 0x52c18 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x52c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52c38 | load | 4 | `abs 0x52d58` | `-` |
| `ldr` at 0x52c3c | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x52c44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x52c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52c68 | load | 4 | `abs 0x52d58` | `-` |
| `ldr` at 0x52c6c | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x52c74 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x52c78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52c84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52c88 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x52c8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52c9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52cb0 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x52cb4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52cc4 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x52cc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x52cf0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x52cf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x52cf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52cfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x52d08 | load | 4 | `abs 0x52d5c` | `-` |
| `str` at 0x52d0c | store | 4 | `(unknown)` | `0x1618b8` |
| `str` at 0x52d10 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x52d14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52d18 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x52d20 | load | 4 | `abs 0x52d60` | `-` |
| `ldr` at 0x52d24 | load | 4 | `abs 0x52d64` | `-` |
| `ldr` at 0x52d34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52d38 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x52d3c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x52d48 | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x0` |
| `ldm` at 0x52d50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x52d50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x52d50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52cac | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x52cb8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x52d00 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x52d2c | `(unknown callee) 0x22070` | `0x6` | `0x16181c` | `0x247` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52c20 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x52c4c | `? != 0x0` | `? == 0x0` |
| `blt` at 0x52c80 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x52c94 | `[[sp-28]+8] <= r1 (signed, 32-bit)` | `[[sp-28]+8] > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x52d50 | `r0` | `?` |

## `powerPresent`

- `powerPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52d68 to 0x52ed4.
  Established. A byte assignment writes `0x1` to `([[sp-28]+4] + r1) +0x0` (`strb` at 0x52eb4).
  The signed comparison `[[sp-28]+8] <= r1` takes `0x52e08`, while greater falls through (`ble` at 0x52e00).
  A separate unnamed `0x22070` call carries `0x6`, `0x16181c`, `0x259` and `0x0` (`bl` at 0x52e98).
  Physical presence reporting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52d78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52d7c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52d80 | load | 4 | `abs 0x52ec0` | `-` |
| `ldr` at 0x52d84 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x52d90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52da4 | load | 4 | `abs 0x52ec4` | `-` |
| `ldr` at 0x52da8 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x52db0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x52dc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52dd4 | load | 4 | `abs 0x52ec4` | `-` |
| `ldr` at 0x52dd8 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x52de0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x52de4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52df4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x52df8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52e08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52e1c | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x52e20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52e30 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x52e34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x52e5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x52e60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x52e64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52e68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x52e74 | load | 4 | `abs 0x52ec8` | `-` |
| `str` at 0x52e78 | store | 4 | `(unknown)` | `0x1618e4` |
| `str` at 0x52e7c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x52e80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52e84 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x52e8c | load | 4 | `abs 0x52ecc` | `-` |
| `ldr` at 0x52e90 | load | 4 | `abs 0x52ed0` | `-` |
| `ldr` at 0x52ea0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52ea4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x52ea8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x52eb4 | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x1` |
| `ldm` at 0x52ebc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x52ebc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x52ebc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52e18 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x52e24 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x52e6c | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x52e98 | `(unknown callee) 0x22070` | `0x6` | `0x16181c` | `0x259` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52d8c | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x52db8 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x52dec | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x52e00 | `[[sp-28]+8] <= r1 (signed, 32-bit)` | `[[sp-28]+8] > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x52ebc | `r0` | `?` |

## `powerReset`

- `powerReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52ed4 to 0x530f4.
  Established. An initial byte assignment to `[sp-48] +0xf` carries an unknown value (`strb` at 0x52f64).
  Later conditional assignments to that byte carry `([sp-32] & 0xff)` and `0x5` on separate paths (`strb` at 0x53060, `strb` at 0x53084).
  Zero `ret(0x595b8)` takes `0x53034`, while nonzero falls through (`beq` at 0x52ff8).
  The unnamed `0x59024` call carries `[sp-20]`, `[sp-24]`, `[sp-25]:1` and `[sp-25]:1` (`bl` at 0x53098).
  The physical reset sequence remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52ee4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52eec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x52ef0 | load | 4 | `abs 0x530e8` | `-` |
| `ldr` at 0x52ef4 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x52f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52f14 | load | 4 | `abs 0x530ec` | `-` |
| `ldr` at 0x52f18 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x52f20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x52f2c | load | 4 | `abs 0x530f0` | `-` |
| `str` at 0x52f30 | store | 4 | `sp -0x34` | `0xfffff9b9` |
| `ldr` at 0x52f38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52f4c | load | 4 | `abs 0x530ec` | `-` |
| `ldr` at 0x52f50 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x52f58 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x52f5c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x52f64 | store | 1 | `[sp-48] +0xf` | `?` |
| `str` at 0x52f6c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x52f70 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x52f74 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0x52f78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x52f84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x52f88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x52f9c | load | 4 | `[sp-48] +0x0` | `-` |
| `str` at 0x52fa4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x52fa8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x52fac | load | 4 | `[sp-48] +0x4` | `-` |
| `ldr` at 0x52fb0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x52fb8 | load | 1 | `([[sp-48]+4] + [sp-44]) +0x0` | `-` |
| `ldr` at 0x52fc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x52fc8 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x52fd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52fd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52fe4 | store | 4 | `sp -0x20` | `ret(0x595b8)` |
| `ldr` at 0x52fe8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52fec | store | 4 | `sp -0x38` | `ret(0x595b8)` |
| `ldr` at 0x52ff0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x52ffc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x53008 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x53014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53018 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x53024 | store | 1 | `sp -0x19` | `(ret(0x594c8) & 0xff)` |
| `str` at 0x53030 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x53034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53038 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x53044 | store | 1 | `sp -0x19` | `(ret(0x5896c) & 0xff)` |
| `ldr` at 0x53048 | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0x5304c | load | 1 | `[sp-48] +0xf` | `-` |
| `ldr` at 0x53058 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5305c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x53060 | store | 1 | `[sp-48] +0xf` | `([sp-32] & 0xff)` |
| `ldr` at 0x53068 | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0x5306c | load | 1 | `[sp-48] +0xf` | `-` |
| `ldr` at 0x53070 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5307c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x53084 | store | 1 | `[sp-48] +0xf` | `0x5` |
| `ldrb` at 0x53088 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x5308c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53090 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x530a0 | store | 4 | `sp -0x28` | `ret(0x59024)` |
| `ldr` at 0x530a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x530b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x530bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x530c0 | store | 4 | `sp -0x24` | `ret(0x59024)` |
| `ldr` at 0x530c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x530cc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x530d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x530d8 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x530dc | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x530e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x530e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x530e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52fdc | `(unknown callee) 0x595b8` | `[sp-20]` | `[sp-24]` | `[[sp-48]+4]` | `[[sp-24]+6]:1` |
| `bl` at 0x5301c | `(unknown callee) 0x594c8` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x595b8)` |
| `bl` at 0x5303c | `(unknown callee) 0x5896c` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x53098 | `(unknown callee) 0x59024` | `[sp-20]` | `[sp-24]` | `[sp-25]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52efc | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x52f28 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x52f80 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x52fc0 | `[([[sp-48]+4] + [sp-44])]:1 == 0x0` | `[([[sp-48]+4] + [sp-44])]:1 != 0x0` |
| `beq` at 0x52fd0 | `[[sp-24]+6]:1 == 0xff` | `[[sp-24]+6]:1 != 0xff` |
| `beq` at 0x52ff8 | `ret(0x595b8) == 0x0` | `ret(0x595b8) != 0x0` |
| `blt` at 0x53004 | `ret(0x595b8) < 0x0 (signed, 32-bit)` | `ret(0x595b8) >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x53010 | `ret(0x595b8) > 0x4 (signed, 32-bit)` | `ret(0x595b8) <= 0x4 (signed, 32-bit)` |
| `bne` at 0x53054 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53078 | `? == [sp-32]` | `? != [sp-32]` |
| `beq` at 0x530ac | `ret(0x59024) == 0x0` | `ret(0x59024) != 0x0` |
| `bne` at 0x530b8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x530e4 | `r0` | `?` |

## `powerGetCfg`

- `powerGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x530f4 to 0x53218.
  Established. A word assignment copies `[[sp-28]]` to entry `r1 +0x0` (`str` at 0x531f0).
  Separate paths place `0x0` and `0xfffff9b9` at `sp -0x20` (`str` at 0x53170, `str` at 0x531b8).
  Zero `[0x1a5c18]` takes `0x5316c`, while nonzero falls through (`beq` at 0x5313c).
  Configuration field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53104 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53108 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5310c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53120 | load | 4 | `abs 0x5320c` | `-` |
| `ldr` at 0x53124 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x5312c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x53130 | load | 4 | `abs 0x53210` | `-` |
| `ldr` at 0x53134 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x53140 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53154 | load | 4 | `abs 0x5320c` | `-` |
| `ldr` at 0x53158 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x53160 | load | 4 | `(unknown)` | `-` |
| `str` at 0x53170 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x53178 | load | 4 | `abs 0x53210` | `-` |
| `ldr` at 0x5317c | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x53188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5319c | load | 4 | `abs 0x5320c` | `-` |
| `ldr` at 0x531a0 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x531a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x531b4 | load | 4 | `abs 0x53214` | `-` |
| `str` at 0x531b8 | store | 4 | `sp -0x20` | `0xfffff9b9` |
| `ldr` at 0x531c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x531d4 | load | 4 | `abs 0x5320c` | `-` |
| `ldr` at 0x531d8 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x531e0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x531e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x531e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x531ec | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x531f0 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x531f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x531f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x531fc | store | 4 | `sp -0x20` | `[[sp-28]+8]` |
| `ldr` at 0x53200 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x53208 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x53208 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x53208 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5313c | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x53168 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x53184 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x531b0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x53208 | `r0` | `?` |

## `powerIsPresent`

- `powerIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53218 to 0x532fc.
  Established. A byte read addresses `([[sp-28]+4] + r1) +0x0`, and its value is written to the word at `sp -0x20` (`ldrb` at 0x532dc, `str` at 0x532e0).
  Signed entry `r1 < 0x0` takes `0x532c0`, while nonnegative falls through (`blt` at 0x532a4).
  Physical presence encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53228 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5322c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x53230 | load | 4 | `abs 0x532f0` | `-` |
| `ldr` at 0x53234 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x53240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53254 | load | 4 | `abs 0x532f4` | `-` |
| `ldr` at 0x53258 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x53260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5326c | load | 4 | `abs 0x532f8` | `-` |
| `str` at 0x53270 | store | 4 | `sp -0x20` | `0xfffff9b9` |
| `ldr` at 0x53278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5328c | load | 4 | `abs 0x532f4` | `-` |
| `ldr` at 0x53290 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x53298 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5329c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x532a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x532ac | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x532b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x532c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x532cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x532d0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x532d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x532dc | load | 1 | `([[sp-28]+4] + r1) +0x0` | `-` |
| `str` at 0x532e0 | store | 4 | `sp -0x20` | `[([[sp-28]+4] + r1)]:1` |
| `ldr` at 0x532e4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x532ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x532ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x532ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5323c | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x53268 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x532a4 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x532b8 | `[[sp-28]+8] <= r1 (signed, 32-bit)` | `[[sp-28]+8] > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x532ec | `r0` | `?` |

## `powerUpSlot`

- `powerUpSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54d34 to 0x54e14.
  Established. Signed nonnegative `ret(0x3e58c)` takes `0x54d78`, while negative falls through (`bge` at 0x54d60).
  Included word assignments place `0xfffff05e`, `0xfffff05c` and `0xfffff05b` at `sp -0x20` on separate paths (`str` at 0x54d90, `str` at 0x54db0, `str` at 0x54dd0).
  The unnamed `0x54778` call carries unknown, `[sp-24]`, `[sp-28]` and unknown (`bl` at 0x54df0).
  Slot power-up signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x54d44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x54d48 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x54d4c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x54d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54d64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x54d70 | store | 4 | `sp -0x20` | `ret(0x3e58c)` |
| `ldr` at 0x54d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54d8c | load | 4 | `abs 0x54e08` | `-` |
| `str` at 0x54d90 | store | 4 | `sp -0x20` | `0xfffff05e` |
| `ldr` at 0x54d98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54dac | load | 4 | `abs 0x54e0c` | `-` |
| `str` at 0x54db0 | store | 4 | `sp -0x20` | `0xfffff05c` |
| `ldr` at 0x54db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54dcc | load | 4 | `abs 0x54e10` | `-` |
| `str` at 0x54dd0 | store | 4 | `sp -0x20` | `0xfffff05b` |
| `ldr` at 0x54ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54de8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54dec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54df8 | store | 4 | `sp -0x20` | `ret(0x54778)` |
| `ldr` at 0x54dfc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x54e04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54e04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54d54 | `(unknown callee) 0x3e58c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x54d68 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x54d7c | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x54d9c | `(unknown callee) 0x3f474` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x54dbc | `(unknown callee) 0x3ec6c` | `[sp-20]` | `?` | `?` | `ret(0x3f474)` |
| `bl` at 0x54df0 | `(unknown callee) 0x54778` | `?` | `[sp-24]` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x54d60 | `ret(0x3e58c) >= 0x0 (signed, 32-bit)` | `ret(0x3e58c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x54d88 | `ret(0x3e58c) != 0x0` | `ret(0x3e58c) == 0x0` |
| `bne` at 0x54da8 | `ret(0x3f474) != 0x0` | `ret(0x3f474) == 0x0` |
| `bne` at 0x54dc8 | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54e04 | `r0` | `?` |

## `powerDownSlot`

- `powerDownSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x552f0 to 0x553c0.
  Established. The unnamed `0x54e14` call carries `[sp-20]`, `[sp-24]`, `0x1` and `ret(0x3ec6c)` (`bl` at 0x5539c);
  its result is assigned to the word at `sp -0x1c` (`str` at 0x553a4).
  Nonzero `ret(0x3f474)` takes `0x55370`, while zero falls through (`bne` at 0x55360).
  Slot shutdown hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55300 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x55304 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x55308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5531c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55328 | store | 4 | `sp -0x1c` | `ret(0x3e58c)` |
| `ldr` at 0x55330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55344 | load | 4 | `abs 0x553b4` | `-` |
| `str` at 0x55348 | store | 4 | `sp -0x1c` | `0xfffff05e` |
| `ldr` at 0x55350 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55364 | load | 4 | `abs 0x553b8` | `-` |
| `str` at 0x55368 | store | 4 | `sp -0x1c` | `0xfffff05c` |
| `ldr` at 0x55370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55384 | load | 4 | `abs 0x553bc` | `-` |
| `str` at 0x55388 | store | 4 | `sp -0x1c` | `0xfffff05b` |
| `ldr` at 0x55390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55394 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x553a4 | store | 4 | `sp -0x1c` | `ret(0x54e14)` |
| `ldr` at 0x553a8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x553b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x553b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x553b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5530c | `(unknown callee) 0x3e58c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x55320 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x55334 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x55354 | `(unknown callee) 0x3f474` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x55374 | `(unknown callee) 0x3ec6c` | `[sp-20]` | `?` | `?` | `ret(0x3f474)` |
| `bl` at 0x5539c | `(unknown callee) 0x54e14` | `[sp-20]` | `[sp-24]` | `0x1` | `ret(0x3ec6c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x55318 | `ret(0x3e58c) >= 0x0 (signed, 32-bit)` | `ret(0x3e58c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x55340 | `ret(0x3e58c) != 0x0` | `ret(0x3e58c) == 0x0` |
| `bne` at 0x55360 | `ret(0x3f474) != 0x0` | `ret(0x3f474) == 0x0` |
| `bne` at 0x55380 | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x553b0 | `r0` | `?` |

## `powerCycle`

- `powerCycle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55654 to 0x556e0.
  Established. Two indirect calls have unknown targets and names (`mov` at 0x55678, `mov` at 0x556b8).
  The latter carries `0x1`, `[sp-20]`, unknown and `[0x31a600]` (`mov` at 0x556b8).
  Zero `ret(unknown)` takes `0x5569c`, while nonzero falls through (`beq` at 0x5568c).
  That taken path includes an unnamed `0x53a64` call with `0xc8`, two unknown candidates and `ret(unknown)` (`bl` at 0x556a0).
  Cycle timing units and physical transitions remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55664 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x55668 | load | 4 | `abs 0x556d8` | `-` |
| `ldr` at 0x5566c | load | 4 | `abs 0x31a5fc` | `-` |
| `ldr` at 0x55670 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55680 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0x55684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55690 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55694 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x556a4 | load | 4 | `abs 0x556dc` | `-` |
| `ldr` at 0x556a8 | load | 4 | `abs 0x31a600` | `-` |
| `ldr` at 0x556b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x556c0 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0x556c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x556c8 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x556cc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x556d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x556d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x556d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x55678 | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `[0x31a5fc]` |
| `bl` at 0x556a0 | `(unknown callee) 0x53a64` | `0xc8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x556b8 | `(unknown callee) (unknown target)` | `0x1` | `[sp-20]` | `?` | `[0x31a600]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5568c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x556d4 | `r0` | `ret(unknown)` |

## `powerStatusVerbose`

- `powerStatusVerbose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55700 to 0x5572c.
  Established. The word at `0x1a5c10` receives entry `r0` (`str` at 0x5571c).
  The return carries `0x1a5c10` in `r0` (`ldm` at 0x55724).
  The meaning of the stored setting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55710 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x55714 | load | 4 | `abs 0x55728` | `-` |
| `ldr` at 0x55718 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5571c | store | 4 | `abs 0x1a5c10` | `r0` |
| `ldm` at 0x55724 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x55724 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55724 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55724 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55724 | `r0` | `0x1a5c10` |

## `powerMarginStatus`

- `powerMarginStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x556e0 to 0x55700.
  Established. A word read addresses `0x1a024c` (`ldr` at 0x556f0).
  The return carries `[0x1a024c]` in `r0` (`ldm` at 0x556f8).
  Margin-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x556ec | load | 4 | `abs 0x556fc` | `-` |
| `ldr` at 0x556f0 | load | 4 | `abs 0x1a024c` | `-` |
| `ldm` at 0x556f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x556f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x556f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x556f8 | `r0` | `[0x1a024c]` |

## `powerStatus`

- `powerStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5572c to 0x558d8.
  Established. The unnamed `0x558d8` call carries `ret(0x6e190)`, two unknown candidates and `ret(0x6e190)` (`bl` at 0x55768).
  Signed `ret(0x3de6c) < 0x0` takes `0x55874`, while nonnegative falls through (`blt` at 0x55804).
  Separate word assignments place an unknown value and `0x1` at `sp -0x2c` on different paths (`str` at 0x55838, `str` at 0x558c0).
  Combined power-status meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55740 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x55748 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x55750 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x55758 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x55770 | store | 4 | `sp -0x18` | `ret(0x558d8)` |
| `ldr` at 0x55774 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55778 | store | 4 | `sp -0x1c` | `ret(0x558d8)` |
| `str` at 0x55780 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x55784 | load | 4 | `abs 0x558d0` | `-` |
| `ldr` at 0x55788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5578c | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x557a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x557b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x557c4 | load | 4 | `abs 0x558d4` | `-` |
| `ldr` at 0x557c8 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x557d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x557e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x557ec | store | 4 | `sp -0x28` | `ret(0x558d8)` |
| `ldr` at 0x557f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x557f8 | store | 4 | `sp -0x30` | `ret(0x3de6c)` |
| `ldr` at 0x557fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55808 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55814 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55824 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x55828 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55838 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x55840 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x55850 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x55854 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x55858 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x5585c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x55860 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55870 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x55874 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5587c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x55884 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55890 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x558a0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x558ac | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x558b0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x558b4 | store | 4 | `sp -0x2c` | `[sp-52]` |
| `str` at 0x558c0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x558c4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x558cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x558cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x558cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5575c | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x55768 | `(unknown callee) 0x558d8` | `ret(0x6e190)` | `?` | `?` | `ret(0x6e190)` |
| `bl` at 0x55798 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `[0x1a5c18]` |
| `bl` at 0x557e4 | `(unknown callee) 0x558d8` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x557f4 | `(unknown callee) 0x3de6c` | `[sp-20]` | `?` | `?` | `ret(0x558d8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x55794 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x557a8 | `ret(0x6e190) != [sp-20]` | `ret(0x6e190) == [sp-20]` |
| `bne` at 0x557d8 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x55804 | `ret(0x3de6c) < 0x0 (signed, 32-bit)` | `ret(0x3de6c) >= 0x0 (signed, 32-bit)` |
| `ble` at 0x55810 | `ret(0x3de6c) <= 0x1 (signed, 32-bit)` | `ret(0x3de6c) > 0x1 (signed, 32-bit)` |
| `beq` at 0x5581c | `ret(0x3de6c) == 0x2` | `ret(0x3de6c) != 0x2` |
| `beq` at 0x55830 | `[sp-40] == [sp-28]` | `[sp-40] != [sp-28]` |
| `beq` at 0x55848 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x55868 | `[sp-40] == [sp-32]` | `[sp-40] != [sp-32]` |
| `bne` at 0x5588c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x55898 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x558cc | `r0` | `?` |

## `powerStatusSlot`

- `powerStatusSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x558d8 to 0x55c7c.
  Established. Included byte reads address `[sp-48] +0x1`, `[sp-48] +0x2` and `[sp-48] +0x3` (`ldrb` at 0x55a58, `ldrb` at 0x55aa0, `ldrb` at 0x55abc).
  The unnamed `0x125c20` call carries `[sp-20]`, `[[sp-48]+2]:1`, unknown and `[[sp-48]+2]:1` (`bl` at 0x55aac).
  Equality `[[sp-48]+2]:1 == 0xff` takes `0x55bf4`, while inequality falls through (`beq` at 0x55a50).
  Slot status bit assignments remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x558e8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x558ec | load | 4 | `abs 0x55c60` | `-` |
| `ldr` at 0x558f0 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x558fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55910 | load | 4 | `abs 0x55c64` | `-` |
| `ldr` at 0x55914 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x5591c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x55928 | load | 4 | `abs 0x55c68` | `-` |
| `str` at 0x5592c | store | 4 | `sp -0x44` | `0xfffff9b9` |
| `ldr` at 0x55934 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55948 | load | 4 | `abs 0x55c64` | `-` |
| `ldr` at 0x5594c | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x55954 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x55958 | load | 4 | `abs 0x55c64` | `-` |
| `ldr` at 0x5595c | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x55968 | load | 4 | `abs 0x55c60` | `-` |
| `ldr` at 0x5596c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55970 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x5597c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55990 | load | 4 | `abs 0x55c64` | `-` |
| `ldr` at 0x55994 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x5599c | load | 4 | `(unknown)` | `-` |
| `str` at 0x559ac | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x559b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x559c8 | load | 4 | `abs 0x55c64` | `-` |
| `ldr` at 0x559cc | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x559d4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x559dc | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x559e4 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x559ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x559f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x559f4 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x559f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55a04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x55a08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55a1c | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x55a24 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x55a28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x55a2c | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x55a30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x55a38 | load | 1 | `([[sp-44]+4] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x55a44 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55a48 | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x55a54 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55a58 | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x55a5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x55a60 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x55a6c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55a70 | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x55a74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55a90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55a98 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x55a9c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55aa0 | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x55aa4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x55ab4 | store | 1 | `sp -0x31` | `(ret(0x125c20) & 0xff)` |
| `ldr` at 0x55ab8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55abc | load | 1 | `[sp-48] +0x3` | `-` |
| `ldrb` at 0x55ac0 | load | 1 | `sp -0x31` | `-` |
| `str` at 0x55ad8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x55adc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55ae8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x55af4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55afc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x55b04 | load | 4 | `abs 0x55c6c` | `-` |
| `ldr` at 0x55b08 | load | 4 | `abs 0x1a5c10` | `-` |
| `ldr` at 0x55b14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55b18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55b20 | store | 4 | `sp -0x48` | `ret(0x58280)` |
| `ldr` at 0x55b24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55b38 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x55b3c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x55b4c | store | 4 | `sp -0x3c` | `ret(0x2031c)+1` |
| `ldr` at 0x55b50 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x55b78 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x55b7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x55b80 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x55b84 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x55b8c | store | 4 | `sp -0x4c` | `ret(0x203f4)` |
| `ldr` at 0x55b90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x55ba4 | load | 4 | `abs 0x55c70` | `-` |
| `str` at 0x55ba8 | store | 4 | `sp -0x50` | `0x161bf8` |
| `ldr` at 0x55bb0 | load | 4 | `abs 0x55c74` | `-` |
| `str` at 0x55bb4 | store | 4 | `sp -0x50` | `0x161c00` |
| `ldr` at 0x55bb8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55bbc | load | 1 | `[sp-48] +0x2` | `-` |
| `str` at 0x55bc0 | store | 4 | `(unknown)` | `[[sp-48]+2]:1` |
| `ldrb` at 0x55bc4 | load | 1 | `sp -0x31` | `-` |
| `str` at 0x55bc8 | store | 4 | `(unknown)` | `[sp-49]:1` |
| `ldr` at 0x55bcc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x55bd0 | load | 1 | `[sp-48] +0x3` | `-` |
| `str` at 0x55bd4 | store | 4 | `(unknown)` | `[[sp-48]+3]:1` |
| `ldr` at 0x55bd8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x55bdc | store | 4 | `(unknown)` | `[sp-80]` |
| `ldr` at 0x55be0 | load | 4 | `abs 0x55c78` | `-` |
| `ldr` at 0x55be4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x55be8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x55bec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55bf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x55bfc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x55c04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55c10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55c14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55c24 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x55c2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55c3c | store | 4 | `sp -0x54` | `?` |
| `str` at 0x55c48 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x55c4c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x55c50 | store | 4 | `sp -0x44` | `[sp-84]` |
| `ldr` at 0x55c54 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0x55c5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55c5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55a7c | `(unknown callee) 0x55c7c` | `[sp-20]` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` |
| `bl` at 0x55aac | `(unknown callee) 0x125c20` | `[sp-20]` | `[[sp-48]+2]:1` | `?` | `[[sp-48]+2]:1` |
| `bl` at 0x55ae0 | `(unknown callee) 0x58bfc` | `[sp-48]` | `?` | `[[sp-48]+3]:1` | `?` |
| `bl` at 0x55b1c | `(unknown callee) 0x58280` | `[sp-20]` | `[sp-24]` | `ret(0x58bfc)` | `[0x1a5c10]` |
| `bl` at 0x55b34 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x55b40 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x55b88 | `(unknown callee) 0x203f4` | `[sp-64]` | `[sp-56]` | `[sp-60]` | `?` |
| `bl` at 0x55b94 | `(unknown callee) 0x58bfc` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x55bf0 | `(unknown callee) 0x29908` | `0x161c08` | `[sp-72]` | `[sp-76]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x558f8 | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x55924 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x55964 | `[0x1a5c14] == 0x0` | `[0x1a5c14] != 0x0` |
| `bge` at 0x55978 | `r0 >= [0x1a5c18] (signed, 32-bit)` | `r0 < [0x1a5c18] (signed, 32-bit)` |
| `bne` at 0x559a4 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x55a00 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x55a40 | `[([[sp-44]+4] + [sp-24])]:1 == 0x0` | `[([[sp-44]+4] + [sp-24])]:1 != 0x0` |
| `beq` at 0x55a50 | `[[sp-48]+2]:1 == 0xff` | `[[sp-48]+2]:1 != 0xff` |
| `bge` at 0x55a68 | `[[sp-48]+1]:1 >= [[sp-44]+8] (signed, 32-bit)` | `[[sp-48]+1]:1 < [[sp-44]+8] (signed, 32-bit)` |
| `bgt` at 0x55a88 | `ret(0x55c7c) > 0x0 (signed, 32-bit)` | `ret(0x55c7c) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x55af0 | `ret(0x58bfc) != [sp-36]` | `ret(0x58bfc) == [sp-36]` |
| `beq` at 0x55b10 | `[0x1a5c10] == 0x0` | `[0x1a5c10] != 0x0` |
| `beq` at 0x55ba0 | `ret(0x58bfc) == 0x0` | `ret(0x58bfc) != 0x0` |
| `beq` at 0x55c0c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x55c1c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x55c34 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55c5c | `r0` | `?` |

## `powerStatusIndex`

- `powerStatusIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55c7c to 0x55e9c.
  Established. The unnamed `0x53218` call carries `[sp-20]`, `[sp-24]` and two unknown candidates (`bl` at 0x55da4).
  Nonzero `ret(0x53218)` takes `0x55dc0`, while zero falls through (`bne` at 0x55db0).
  Another unnamed `0x125c20` call carries `[sp-20]`, `[[sp-28]+2]:1`, unknown and `[[sp-28]+2]:1` (`bl` at 0x55e10).
  A byte read addresses `[sp-28] +0x3` (`ldrb` at 0x55e1c).
  Indexed power-status polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55c8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x55c90 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x55c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55cbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55cc8 | store | 4 | `sp -0x28` | `ret(0x3e58c)` |
| `ldr` at 0x55cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55ce4 | load | 4 | `abs 0x55e80` | `-` |
| `str` at 0x55ce8 | store | 4 | `sp -0x28` | `0xfffff05e` |
| `ldr` at 0x55cf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55d04 | load | 4 | `abs 0x55e84` | `-` |
| `str` at 0x55d08 | store | 4 | `sp -0x28` | `0xfffff05c` |
| `ldr` at 0x55d10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55d24 | load | 4 | `abs 0x55e88` | `-` |
| `str` at 0x55d28 | store | 4 | `sp -0x28` | `0xfffff05b` |
| `ldr` at 0x55d30 | load | 4 | `abs 0x55e8c` | `-` |
| `ldr` at 0x55d34 | load | 4 | `abs 0x1a5c18` | `-` |
| `ldr` at 0x55d40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55d54 | load | 4 | `abs 0x55e90` | `-` |
| `ldr` at 0x55d58 | load | 4 | `abs 0x1a5c14` | `-` |
| `ldr` at 0x55d60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x55d6c | load | 4 | `abs 0x55e94` | `-` |
| `str` at 0x55d70 | store | 4 | `sp -0x28` | `0xfffff9b9` |
| `ldr` at 0x55d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55d8c | load | 4 | `abs 0x55e90` | `-` |
| `ldr` at 0x55d90 | load | 4 | `abs 0x1a5c14` | `-` |
| `str` at 0x55d98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x55d9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55da0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55db4 | load | 4 | `abs 0x55e94` | `-` |
| `str` at 0x55db8 | store | 4 | `sp -0x28` | `0xfffff9b9` |
| `ldr` at 0x55dc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x55dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x55dd8 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x55de0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x55de4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x55de8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x55df4 | load | 4 | `abs 0x55e98` | `-` |
| `str` at 0x55df8 | store | 4 | `sp -0x28` | `0xfffff9b8` |
| `ldr` at 0x55e00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x55e04 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x55e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55e18 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x55e1c | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x55e28 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x55e2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55e40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55e50 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x55e58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55e68 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x55e6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x55e70 | store | 4 | `sp -0x28` | `[sp-44]` |
| `ldr` at 0x55e74 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x55e7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x55e7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x55e7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55c94 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x55cac | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `ret(0x6e190)` | `[sp-20]` |
| `bl` at 0x55cc0 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x55cd4 | `(unknown callee) 0x3e58c` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x55cf4 | `(unknown callee) 0x3f474` | `[sp-20]` | `?` | `?` | `ret(0x3e58c)` |
| `bl` at 0x55d14 | `(unknown callee) 0x3ec6c` | `[sp-20]` | `?` | `?` | `ret(0x3f474)` |
| `bl` at 0x55da4 | `(unknown callee) 0x53218` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x55e10 | `(unknown callee) 0x125c20` | `[sp-20]` | `[[sp-28]+2]:1` | `?` | `[[sp-28]+2]:1` |
| `bl` at 0x55e30 | `(unknown callee) 0x58bfc` | `[sp-28]` | `?` | `ret(0x125c20)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55ca4 | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |
| `bge` at 0x55cb8 | `ret(0x3e58c) >= 0x0 (signed, 32-bit)` | `ret(0x3e58c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x55ce0 | `ret(0x3e58c) != 0x0` | `ret(0x3e58c) == 0x0` |
| `bne` at 0x55d00 | `ret(0x3f474) != 0x0` | `ret(0x3f474) == 0x0` |
| `bne` at 0x55d20 | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |
| `beq` at 0x55d3c | `[0x1a5c18] == 0x0` | `[0x1a5c18] != 0x0` |
| `bne` at 0x55d68 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x55db0 | `ret(0x53218) != 0x0` | `ret(0x53218) == 0x0` |
| `bne` at 0x55df0 | `[[sp-28]+2]:1 != 0xff` | `[[sp-28]+2]:1 == 0xff` |
| `beq` at 0x55e3c | `ret(0x58bfc) == 0x0` | `ret(0x58bfc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x55e7c | `r0` | `?` |
