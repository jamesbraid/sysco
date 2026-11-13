# ARM configuration state and router queue paths in 1.44.0

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
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/configuration-queues-facts.txt) retain all tool sections.

## `L1CfgAddState`

- `L1CfgAddState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66a10 to 0x66d5c.
  Established. A nonzero `ret(0x65068)` takes `0x66a9c`, while zero falls through (`bne` at 0x66a68).
  The absolute word at `0x132aec` receives `[0x132aec]+16` on an included path (`str` at 0x66b38).
  Another word assignment writes `ret(0x195d4)` to `0x132af0` (`str` at 0x66b6c).
  State insertion ordering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x66a20 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x66a24 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x66a28 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x66a2c | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x66a34 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x66a38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66a44 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66a48 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66a4c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66a50 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x66a54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66a58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66a6c | load | 4 | `abs 0x66d48` | `-` |
| `ldr` at 0x66a70 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x66a80 | load | 4 | `abs 0x66d4c` | `-` |
| `ldr` at 0x66a84 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x66a88 | load | 4 | `abs 0x66d50` | `-` |
| `ldr` at 0x66a8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66a9c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66aa0 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66aa4 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66aa8 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x66aac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66ab0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66abc | store | 4 | `sp -0x28` | `ret(0x65184)` |
| `ldr` at 0x66ac0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66acc | load | 4 | `abs 0x66d48` | `-` |
| `ldr` at 0x66ad0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x66ae0 | load | 4 | `abs 0x66d4c` | `-` |
| `ldr` at 0x66ae4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x66ae8 | load | 4 | `abs 0x66d54` | `-` |
| `ldr` at 0x66aec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66af0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66af8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66afc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66b0c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b10 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x66b18 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b1c | load | 4 | `abs 0x132aec` | `-` |
| `ldr` at 0x66b28 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b2c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b30 | load | 4 | `abs 0x132aec` | `-` |
| `str` at 0x66b38 | store | 4 | `abs 0x132aec` | `[0x132aec]+16` |
| `ldr` at 0x66b3c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b40 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b44 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b48 | load | 4 | `abs 0x132aec` | `-` |
| `ldr` at 0x66b5c | load | 4 | `abs 0x132af0` | `-` |
| `str` at 0x66b6c | store | 4 | `abs 0x132af0` | `ret(0x195d4)` |
| `str` at 0x66b74 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x66b78 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66b7c | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x66b80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66b8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66b94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66b9c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66ba0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66bb4 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66bbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66bc4 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66bc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66bdc | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66be4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66bf4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x66bfc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x66c04 | load | 4 | `abs 0x66d48` | `-` |
| `ldr` at 0x66c08 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x66c18 | load | 4 | `abs 0x66d4c` | `-` |
| `ldr` at 0x66c1c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x66c20 | load | 4 | `abs 0x66d58` | `-` |
| `ldr` at 0x66c24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66c30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66c3c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66c40 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66c48 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66c4c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66c50 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x66c68 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66c78 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66c7c | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x66c80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66c8c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66c90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66ca4 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66cb0 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66cb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66cc8 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66cd0 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66cd4 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x66cd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66cf8 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66cfc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66d10 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x66d1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66d28 | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66d2c | load | 4 | `abs 0x66d44` | `-` |
| `ldr` at 0x66d30 | load | 4 | `abs 0x132ae8` | `-` |
| `str` at 0x66d38 | store | 4 | `abs 0x132ae8` | `[0x132ae8]+1` |
| `ldm` at 0x66d40 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x66d40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x66d40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x66d40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x66a5c | `(unknown callee) 0x65068` | `[0x132ae4]` | `[0x132adc]` | `r0` | `r1` |
| `bl` at 0x66a94 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc300` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x66ab4 | `(unknown callee) 0x65184` | `[0x132af0]` | `[0x132ae8]` | `?` | `?` |
| `bl` at 0x66af4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc320` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x66b04 | `(unknown callee) 0x19610` | `?` | `[sp-32]` | `0x70` | `?` |
| `bl` at 0x66b64 | `(unknown callee) 0x195d4` | `[0x132af0]` | `?` | `[0x132aec]` | `?` |
| `bl` at 0x66c2c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc338` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x66c70 | `(unknown callee) 0x19484` | `[0x132af0]+112` | `[0x132af0]` | `?` | `?` |
| `bl` at 0x66cf4 | `(unknown callee) 0x19484` | `?` | `?` | `?` | `?` |
| `bl` at 0x66d24 | `(unknown callee) 0x19610` | `?` | `[sp-32]` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x66a40 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0x66a68 | `ret(0x65068) != 0x0` | `ret(0x65068) == 0x0` |
| `beq` at 0x66a7c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x66ac8 | `ret(0x65184) == 0x0` | `ret(0x65184) != 0x0` |
| `beq` at 0x66adc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x66b24 | `[0x132ae8]+1 <= [0x132aec] (signed, 32-bit)` | `[0x132ae8]+1 > [0x132aec] (signed, 32-bit)` |
| `ble` at 0x66b88 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x66bf0 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `beq` at 0x66c14 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x66c38 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x66c88 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x66d40 | `r0` | `?` |

## `L1CfgMeshTask`

- `L1CfgMeshTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60c60 to 0x60e78.
  Established. The signed condition `ret(0x194f0) >= [0x1327a4]` takes `0x60cc8`, while smaller values fall through (`bge` at 0x60cc0).
  An included word store clears `0x1327a4` (`str` at 0x60cd8).
  Separate included stores write `0x5` and `0xf` to `0x12d78c` (`str` at 0x60d0c, `str` at 0x60e2c).
  Mesh configuration scheduling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x60c70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x60c74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x60c7c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x60c84 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x60c8c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x60c98 | load | 4 | `abs 0x60e50` | `-` |
| `ldr` at 0x60c9c | load | 4 | `abs 0x1327a4` | `-` |
| `ldr` at 0x60cb4 | load | 4 | `abs 0x60e50` | `-` |
| `ldr` at 0x60cb8 | load | 4 | `abs 0x1327a4` | `-` |
| `ldr` at 0x60cc8 | load | 4 | `abs 0x60e54` | `-` |
| `ldr` at 0x60cd0 | load | 4 | `abs 0x60e50` | `-` |
| `str` at 0x60cd8 | store | 4 | `abs 0x1327a4` | `0x0` |
| `ldr` at 0x60cdc | load | 4 | `abs 0x60e58` | `-` |
| `ldr` at 0x60ce0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x60cf0 | load | 4 | `abs 0x60e5c` | `-` |
| `ldr` at 0x60cf4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x60cf8 | load | 4 | `abs 0x60e60` | `-` |
| `ldr` at 0x60d04 | load | 4 | `abs 0x60e64` | `-` |
| `str` at 0x60d0c | store | 4 | `abs 0x12d78c` | `0x5` |
| `ldr` at 0x60d14 | load | 4 | `abs 0x60e68` | `-` |
| `ldr` at 0x60d18 | load | 4 | `abs 0x132794` | `-` |
| `ldr` at 0x60d30 | load | 4 | `abs 0x60e68` | `-` |
| `ldr` at 0x60d34 | load | 4 | `abs 0x132794` | `-` |
| `str` at 0x60d3c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x60d40 | load | 4 | `abs 0x60e6c` | `-` |
| `ldr` at 0x60d44 | load | 4 | `abs 0x132798` | `-` |
| `ldr` at 0x60d5c | load | 4 | `abs 0x60e6c` | `-` |
| `ldr` at 0x60d60 | load | 4 | `abs 0x132798` | `-` |
| `str` at 0x60d68 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x60d6c | load | 4 | `abs 0x60e70` | `-` |
| `ldr` at 0x60d70 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x60d7c | load | 4 | `abs 0x60e64` | `-` |
| `ldr` at 0x60d80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60d84 | load | 4 | `abs 0x12d78c` | `-` |
| `str` at 0x60d98 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x60da0 | load | 4 | `abs 0x60e74` | `-` |
| `ldr` at 0x60da4 | load | 4 | `abs 0x132790` | `-` |
| `ldr` at 0x60db0 | load | 4 | `abs 0x60e74` | `-` |
| `ldr` at 0x60db4 | load | 4 | `abs 0x60e68` | `-` |
| `ldr` at 0x60db8 | load | 4 | `abs 0x132794` | `-` |
| `str` at 0x60dbc | store | 4 | `abs 0x132790` | `[0x132794]` |
| `str` at 0x60dc8 | store | 4 | `sp -0x28` | `ret(0x1d444)` |
| `ldr` at 0x60dcc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x60dd0 | store | 4 | `sp -0x2c` | `ret(0x1d444)` |
| `ldr` at 0x60dd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x60de0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x60df0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60dfc | load | 4 | `abs 0x60e64` | `-` |
| `ldr` at 0x60e00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60e04 | load | 4 | `abs 0x12d78c` | `-` |
| `ldr` at 0x60e18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60e24 | load | 4 | `abs 0x60e64` | `-` |
| `str` at 0x60e2c | store | 4 | `abs 0x12d78c` | `0xf` |
| `ldr` at 0x60e34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x60e48 | store | 4 | `sp -0x24` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60c94 | `(unknown callee) 0x20974` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x60cac | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[0x1327a4]` |
| `bl` at 0x60ccc | `(unknown callee) 0x619dc` | `0xfbd2c` | `?` | `[0x1327a4]` | `ret(0x194f0)` |
| `bl` at 0x60cfc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbd3c` | `0x1327a4` | `0x12eeb8` |
| `bl` at 0x60d00 | `(unknown callee) 0x62e24` | `?` | `?` | `?` | `?` |
| `bl` at 0x60d28 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[0x132794]` |
| `bl` at 0x60d54 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[0x132798]` |
| `bl` at 0x60dc4 | `(unknown callee) 0x1d444` | `0x9a` | `?` | `?` | `?` |
| `bl` at 0x60e14 | `(unknown callee) 0x6288c` | `ret(0x1d444)` | `?` | `?` | `?` |
| `bl` at 0x60e40 | `(unknown callee) 0x6288c` | `ret(0x1d444)` | `?` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60ca4 | `[0x1327a4] == 0x0` | `[0x1327a4] != 0x0` |
| `bge` at 0x60cc0 | `ret(0x194f0) >= [0x1327a4] (signed, 32-bit)` | `ret(0x194f0) < [0x1327a4] (signed, 32-bit)` |
| `beq` at 0x60cec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x60d20 | `[0x132794] == 0x0` | `[0x132794] != 0x0` |
| `beq` at 0x60d4c | `[0x132798] == 0x0` | `[0x132798] != 0x0` |
| `beq` at 0x60d78 | `[0x132adc] == 0x1` | `[0x132adc] != 0x1` |
| `blt` at 0x60d8c | `[sp-28] < [0x12d78c] (signed, 32-bit)` | `[sp-28] >= [0x12d78c] (signed, 32-bit)` |
| `bne` at 0x60dac | `[0x132790] != 0x0` | `[0x132790] == 0x0` |
| `beq` at 0x60ddc | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x60de8 | `ret(0x1d444) == 0x1` | `ret(0x1d444) != 0x1` |
| `beq` at 0x60df8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bge` at 0x60e0c | `[sp-32] >= [0x12d78c] (signed, 32-bit)` | `[sp-32] < [0x12d78c] (signed, 32-bit)` |
| `ble` at 0x60e20 | `[sp-28] <= 0x3c (signed, 32-bit)` | `[sp-28] > 0x3c (signed, 32-bit)` |
| `beq` at 0x60e3c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `L1CfgPortDisable`

- `L1CfgPortDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64c54 to 0x65068.
  Established. An included word assignment at `0x132780` writes `[0x132780]-1` (`str` at 0x64d6c).
  The unnamed `0xaa424` call carries `0x132a48`, `0x0`, `0x64`, `0x2` (`bl` at 0x64f98).
  A nonzero result takes `0x64fb4`, while zero falls through (`bne` at 0x64fac).
  Port removal notification policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64c64 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x64c68 | load | 4 | `abs 0x6503c` | `-` |
| `ldr` at 0x64c70 | load | 4 | `abs 0x65040` | `-` |
| `ldr` at 0x64c74 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x64c84 | load | 4 | `abs 0x65044` | `-` |
| `ldr` at 0x64c88 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x64c8c | load | 4 | `abs 0x65048` | `-` |
| `ldr` at 0x64c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64c98 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64cb8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64cc0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64cc4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x64cc8 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64ce8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64cf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64cfc | load | 4 | `abs 0x65050` | `-` |
| `ldr` at 0x64d00 | load | 4 | `abs 0x65050` | `-` |
| `ldr` at 0x64d04 | load | 4 | `abs 0x132784` | `-` |
| `str` at 0x64d0c | store | 4 | `abs 0x132784` | `[0x132784]-1` |
| `ldr` at 0x64d10 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64d14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64d30 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64d38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64d48 | load | 4 | `abs 0x65054` | `-` |
| `ldr` at 0x64d4c | load | 4 | `abs 0x65054` | `-` |
| `ldr` at 0x64d50 | load | 4 | `abs 0x132788` | `-` |
| `str` at 0x64d58 | store | 4 | `abs 0x132788` | `[0x132788]-1` |
| `ldr` at 0x64d5c | load | 4 | `abs 0x65058` | `-` |
| `ldr` at 0x64d60 | load | 4 | `abs 0x65058` | `-` |
| `ldr` at 0x64d64 | load | 4 | `abs 0x132780` | `-` |
| `str` at 0x64d6c | store | 4 | `abs 0x132780` | `[0x132780]-1` |
| `ldr` at 0x64d70 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64d74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64d90 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x64d9c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x64da0 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64da4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64dc0 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x64dcc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x64dd0 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64df0 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x64dfc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x64e00 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64e04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64e20 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64e3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64e54 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64e58 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64e64 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64e68 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64e74 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64e78 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64e84 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64e88 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64e94 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64e98 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64ea4 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64ea8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64eb4 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64eb8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64ec4 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64ec8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64ed4 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64ed8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64ee4 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64ee8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64ef4 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64ef8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64f04 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64f08 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x64f14 | load | 4 | `abs 0x6504c` | `-` |
| `ldr` at 0x64f18 | load | 4 | `abs 0x132a60` | `-` |
| `str` at 0x64f74 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x64f78 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x64f7c | store | 4 | `sp -0x48` | `[sp-64]` |
| `str` at 0x64f84 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0x64f88 | load | 4 | `abs 0x6505c` | `-` |
| `str` at 0x64fa0 | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0x64fa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64fb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64fb8 | load | 4 | `abs 0x65060` | `-` |
| `ldr` at 0x64fbc | load | 4 | `abs 0x132ef8` | `-` |
| `str` at 0x64fc0 | store | 4 | `ret(0xaa424) +0x18` | `[0x132ef8]` |
| `ldr` at 0x64fc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64fe4 | load | 4 | `abs 0x65064` | `-` |
| `ldr` at 0x64fe8 | load | 4 | `abs 0x29a5a8` | `-` |
| `ldr` at 0x64ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64ffc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6501c | load | 4 | `abs 0x65064` | `-` |
| `ldr` at 0x65020 | load | 4 | `abs 0x29a5a4` | `-` |
| `ldr` at 0x6502c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x65038 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65038 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65038 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64c6c | `(unknown callee) 0x619dc` | `0xfc29c` | `r1` | `r2` | `r3` |
| `bl` at 0x64c94 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc2b0` | `[sp-20]` | `0x12eeb8` |
| `bl` at 0x64e38 | `(unknown callee) 0x19748` | `?` | `0x0` | `0x70` | `?` |
| `bl` at 0x64e48 | `(unknown callee) 0x62e24` | `ret(0x19748)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x64e50 | `(unknown callee) 0x61a78` | `ret(0x19748)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x64f3c | `(unknown callee) 0x5eac8` | `sp-64` | `sp-60` | `0x1` | `sp-64` |
| `bl` at 0x64f54 | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x64f6c | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x64f98 | `(unknown callee) 0xaa424` | `0x132a48` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x64fd4 | `(unknown callee) 0x6f4d4` | `0x2` | `?` | `ret(0xaa424)` | `[sp-20]` |
| `bl` at 0x64ff8 | `(unknown callee) 0xa9d18` | `[0x29a5a8]+20` | `[sp-28]` | `?` | `[0x29a5a8]+20` |
| `bl` at 0x6500c | `(unknown callee) 0x6f4d4` | `0x1` | `?` | `?` | `[sp-20]` |
| `bl` at 0x65030 | `(unknown callee) 0xa9d18` | `[0x29a5a4]+20` | `[sp-28]` | `?` | `[0x29a5a4]+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64c80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x64cf8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x64d40 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x64e44 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x64e60 | `[0x132a60] == 0x35` | `[0x132a60] != 0x35` |
| `beq` at 0x64e70 | `[0x132a60] == 0x4f` | `[0x132a60] != 0x4f` |
| `beq` at 0x64e80 | `[0x132a60] == 0x42` | `[0x132a60] != 0x42` |
| `beq` at 0x64e90 | `[0x132a60] == 0x46` | `[0x132a60] != 0x46` |
| `beq` at 0x64ea0 | `[0x132a60] == 0x4a` | `[0x132a60] != 0x4a` |
| `beq` at 0x64eb0 | `[0x132a60] == 0x4b` | `[0x132a60] != 0x4b` |
| `beq` at 0x64ec0 | `[0x132a60] == 0x21` | `[0x132a60] != 0x21` |
| `beq` at 0x64ed0 | `[0x132a60] == 0x5e` | `[0x132a60] != 0x5e` |
| `beq` at 0x64ee0 | `[0x132a60] == 0x2b` | `[0x132a60] != 0x2b` |
| `beq` at 0x64ef0 | `[0x132a60] == 0x4c` | `[0x132a60] != 0x4c` |
| `beq` at 0x64f00 | `[0x132a60] == 0x5a` | `[0x132a60] != 0x5a` |
| `beq` at 0x64f10 | `[0x132a60] == 0x7c` | `[0x132a60] != 0x7c` |
| `beq` at 0x64f20 | `[0x132a60] == 0x4d` | `[0x132a60] != 0x4d` |
| `bne` at 0x64fac | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `bne` at 0x64fcc | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0x64fe0 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `bne` at 0x65004 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `beq` at 0x65018 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65038 | `r0` | `?` |

## `L1CfgAddBrick`

- `L1CfgAddBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66694 to 0x66a10.
  Established. A zero `ret(0x65068)` takes `0x666f4`, while nonzero falls through (`beq` at 0x666e0).
  The latter arm stores `[sp+8]` as a word through `ret(0x65068) +0xc` (`str` at 0x666ec).
  An included absolute word assignment at `0x132770` writes `[[sp-28]]` (`str` at 0x669f4).
  Brick insertion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x666a4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x666a8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x666ac | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x666b0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x666b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x666b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x666bc | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x666c0 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x666c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x666c8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x666d4 | store | 4 | `sp -0x30` | `ret(0x65068)` |
| `ldr` at 0x666d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x666e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x666e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x666ec | store | 4 | `ret(0x65068) +0xc` | `[sp+8]` |
| `ldr` at 0x666f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x666f8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x66700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66704 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x66710 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66714 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66718 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x66720 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]+16` |
| `ldr` at 0x66724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6672c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66730 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x66744 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x66754 | store | 4 | `[sp-24] +0x0` | `ret(0x195d4)` |
| `ldr` at 0x66758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6675c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x66760 | store | 4 | `sp -0x2c` | `[[sp-24]]` |
| `str` at 0x66768 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6676c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66770 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x66774 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66780 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66788 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x66790 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x667a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x667ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x667b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x667c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x667d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x667e0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x667e8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x667f0 | load | 4 | `abs 0x66a00` | `-` |
| `ldr` at 0x667f4 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x66804 | load | 4 | `abs 0x66a04` | `-` |
| `ldr` at 0x66808 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x6680c | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0x66810 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x66814 | store | 4 | `sp -0x40` | `[sp+16]` |
| `ldr` at 0x66818 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x6681c | store | 4 | `sp -0x3c` | `[sp+12]` |
| `ldr` at 0x66820 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x66824 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x66828 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x6682c | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0x66830 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x66834 | load | 4 | `abs 0x66a08` | `-` |
| `ldr` at 0x66838 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6683c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66850 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66858 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6685c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x66874 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66884 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66888 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x6688c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66898 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x668ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x668b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x668cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x668d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x668d8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x668dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x668fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66910 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66918 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6691c | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x66920 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66934 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6693c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x66940 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x66944 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x66958 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66960 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x66964 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x66968 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6697c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x66984 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x66988 | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x6698c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x669a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x669a8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x669ac | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x669b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x669c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x669cc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x669d0 | store | 4 | `(unknown)` | `[sp+16]` |
| `ldr` at 0x669d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x669d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x669dc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x669e4 | store | 4 | `[sp-28] +0x0` | `[[sp-28]]+1` |
| `ldr` at 0x669e8 | load | 4 | `abs 0x66a0c` | `-` |
| `ldr` at 0x669ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x669f0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x669f4 | store | 4 | `abs 0x132770` | `[[sp-28]]` |
| `ldm` at 0x669fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x669fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x669fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x669fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x666cc | `(unknown callee) 0x65068` | `[r0]` | `[r1]` | `r3` | `[sp]` |
| `bl` at 0x6674c | `(unknown callee) 0x195d4` | `[[sp-24]]` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0x66840 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc2c8` | `[[sp-24]]` | `[sp-36]` |
| `bl` at 0x6687c | `(unknown callee) 0x19484` | `?` | `?` | `?` | `?` |
| `bl` at 0x668f8 | `(unknown callee) 0x19484` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x666e0 | `ret(0x65068) == 0x0` | `ret(0x65068) != 0x0` |
| `ble` at 0x6670c | `[[sp-28]]+1 <= [[sp-32]] (signed, 32-bit)` | `[[sp-28]]+1 > [[sp-32]] (signed, 32-bit)` |
| `ble` at 0x6677c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blo` at 0x667dc | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `beq` at 0x66800 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6684c | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x66894 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x669fc | `r0` | `?` |

## `L1CfgRingEvent`

- `L1CfgRingEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x640dc to 0x642e0.
  Established. The byte through entry `r0 +0x24` is read (`ldrb` at 0x64124).
  The unsigned condition `[sp-48]-1 > 0x2` takes `0x642b4`, while less or equal values fall through (`bhi` at 0x64160).
  An included word store through `[sp-20] +0x30` writes `[[sp-20]+48]+1` (`str` at 0x64208).
  Ring event interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x640ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x640f0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x640f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x640f8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x64108 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6410c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64110 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6411c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x64120 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x64124 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x64134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64138 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x6413c | store | 4 | `sp -0x30` | `[r0+40]` |
| `ldr` at 0x64144 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x64150 | store | 4 | `sp -0x30` | `ret(0xaadf4)` |
| `ldr` at 0x64154 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x64164 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x64168 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x64178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6417c | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x6418c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x641a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x641a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x641ac | store | 4 | `[sp-20] +0x38` | `[sp-24]` |
| `ldr` at 0x641b0 | load | 4 | `abs 0x642c8` | `-` |
| `ldr` at 0x641b4 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x641c4 | load | 4 | `abs 0x642cc` | `-` |
| `ldr` at 0x641c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x641cc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x641dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x641e0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x641ec | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x641f0 | load | 4 | `abs 0x642d0` | `-` |
| `ldr` at 0x641f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x641fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64200 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x64208 | store | 4 | `[sp-20] +0x30` | `[[sp-20]+48]+1` |
| `ldr` at 0x6420c | load | 4 | `abs 0x642d4` | `-` |
| `ldr` at 0x64210 | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x64214 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64220 | load | 4 | `abs 0x642d4` | `-` |
| `ldr` at 0x64224 | load | 4 | `abs 0x132a5c` | `-` |
| `ldr` at 0x64228 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x64234 | load | 4 | `abs 0x642d8` | `-` |
| `str` at 0x64238 | store | 4 | `sp -0x28` | `0x13277c` |
| `ldr` at 0x6423c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x64240 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x64250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64254 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x64258 | store | 4 | `sp -0x2c` | `[[sp-20]+48]` |
| `ldr` at 0x64260 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6426c | store | 4 | `sp -0x2c` | `ret(0xaadf4)` |
| `ldr` at 0x64270 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x64274 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x64278 | store | 4 | `[sp-40] +0x0` | `[sp-44]` |
| `ldr` at 0x6427c | load | 4 | `abs 0x642c8` | `-` |
| `ldr` at 0x64280 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x64290 | load | 4 | `abs 0x642cc` | `-` |
| `ldr` at 0x64294 | load | 4 | `abs 0x642d8` | `-` |
| `ldr` at 0x64298 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6429c | load | 4 | `abs 0x642dc` | `-` |
| `ldr` at 0x642a0 | load | 4 | `abs 0x13277c` | `-` |
| `str` at 0x642ac | store | 4 | `sp -0x24` | `?` |
| `str` at 0x642b8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x642bc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x642c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x642c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x642c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6414c | `(unknown callee) 0xaadf4` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x64194 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `[sp-48]` | `?` |
| `bl` at 0x641f4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc1b0` | `?` | `?` |
| `bl` at 0x64268 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `[0x132a5c]` | `?` |
| `bl` at 0x642a4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc1d8` | `[0x13277c]` | `0x12eeb8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64130 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x64160 | `[sp-48]-1 > 0x2 (unsigned, 32-bit)` | `[sp-48]-1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x64174 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x64184 | `[[sp-20]+48] != 0x0` | `[[sp-20]+48] == 0x0` |
| `bne` at 0x641a0 | `ret(0xaadf4) != 0x0` | `ret(0xaadf4) == 0x0` |
| `beq` at 0x641c0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6421c | `[0x132a58] != [sp-28]` | `[0x132a58] == [sp-28]` |
| `bne` at 0x64230 | `[0x132a5c] != [sp-32]` | `[0x132a5c] == [sp-32]` |
| `bne` at 0x6424c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6428c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x642c4 | `r0` | `?` |

## `L1CfgDelete`

- `L1CfgDelete`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66d5c to 0x67094.
  Established. A zero entry `r0` takes `0x66efc`, while nonzero falls through (`beq` at 0x66d8c).
  Included word stores through `[sp-24] +0x0` and `[sp-32] +0x0` write `[[sp-24]]-1` and `[[sp-32]]-1` (`str` at 0x66ee4, `str` at 0x67064).
  Configuration record removal rules remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x66d6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x66d70 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x66d74 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x66d78 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x66d80 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x66d84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66d90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66d94 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x66d98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66da4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66dc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66dc4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x66dd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66dec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66df0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x66dfc | load | 4 | `abs 0x67084` | `-` |
| `ldr` at 0x66e00 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x66e10 | load | 4 | `abs 0x67088` | `-` |
| `ldr` at 0x66e14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x66e18 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x66e1c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x66e20 | load | 4 | `abs 0x6708c` | `-` |
| `ldr` at 0x66e24 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x66e28 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x66e30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66e44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66e4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66e60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66e70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66e74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66ea4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x66eb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66ed8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66edc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x66ee4 | store | 4 | `[sp-24] +0x0` | `[[sp-24]]-1` |
| `ldr` at 0x66eec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x66ef4 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x66f00 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x66f04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66f10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66f14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66f18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66f24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66f38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66f40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66f44 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x66f50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66f64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66f6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66f70 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x66f7c | load | 4 | `abs 0x67084` | `-` |
| `ldr` at 0x66f80 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x66f90 | load | 4 | `abs 0x67088` | `-` |
| `ldr` at 0x66f94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x66f98 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x66f9c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x66fa0 | load | 4 | `abs 0x67090` | `-` |
| `ldr` at 0x66fa4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x66fa8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x66fb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66fc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66fcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x66fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66fec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66ff0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x66ff4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x67020 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67024 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x67038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67054 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67058 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6705c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x67064 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]-1` |
| `ldr` at 0x6706c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x67074 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0x67080 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x67080 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x67080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x66e2c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc350` | `[sp]` | `[sp+4]` |
| `bl` at 0x66e9c | `(unknown callee) 0x19484` | `?` | `?` | `?` | `?` |
| `bl` at 0x66ed0 | `(unknown callee) 0x19748` | `?` | `0x0` | `0x18` | `?` |
| `bl` at 0x66fac | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc370` | `[sp]` | `[sp+4]` |
| `bl` at 0x6701c | `(unknown callee) 0x19484` | `?` | `?` | `?` | `?` |
| `bl` at 0x67050 | `(unknown callee) 0x19748` | `?` | `0x0` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x66d8c | `r0 == 0x0` | `r0 != 0x0` |
| `ble` at 0x66da0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x66dcc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x66df8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x66e0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x66f0c | `Z == 1` | `not (Z == 1)` |
| `ble` at 0x66f20 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x66f4c | `? != [sp]` | `? == [sp]` |
| `bne` at 0x66f78 | `? != [sp+4]` | `? == [sp+4]` |
| `beq` at 0x66f8c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x67080 | `r0` | `?` |

## `IRouterRegisterFunction`

- `IRouterRegisterFunction`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa75f8 to 0xa76a4.
  Established. The unnamed `0xa7020` call at `0xa7648` carries entry `r0`, entry `r1`, `0x1`, `0x0` (`bl` at 0xa7648);
  its later call carries unknown, unknown, `0x2`, `0x0` (`bl` at 0xa7688).
  The stack word at `sp -0x24` receives `ret(0xa7020)` at two sites (`str` at 0xa7650, `str` at 0xa7690).
  Function registration semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7608 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa760c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7610 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa7614 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xa761c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa7620 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7630 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7634 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xa7638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa763c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7650 | store | 4 | `sp -0x24` | `ret(0xa7020)` |
| `ldr` at 0xa7654 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7660 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7670 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7674 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa7678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa767c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7690 | store | 4 | `sp -0x24` | `ret(0xa7020)` |
| `ldr` at 0xa7694 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xa76a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa76a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa76a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7648 | `(unknown callee) 0xa7020` | `r0` | `r1` | `0x1` | `0x0` |
| `bl` at 0xa7688 | `(unknown callee) 0xa7020` | `?` | `?` | `0x2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa762c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa765c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa766c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa76a0 | `r0` | `?` |

## `IRouterRegisterQueue`

- `IRouterRegisterQueue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa76a4 to 0xa7750.
  Established. The unnamed `0xa7020` call at `0xa76f4` carries entry `r0`, entry `r1`, `0x1`, `0x1` (`bl` at 0xa76f4).
  A separate call carries unknown, unknown, `0x2`, `0x1` (`bl` at 0xa7734).
  The included integer return value is unknown (`ldm` at 0xa774c).
  Queue registration policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa76b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa76b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa76bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa76c0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xa76c8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa76cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa76dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa76e0 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xa76e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa76e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa76fc | store | 4 | `sp -0x24` | `ret(0xa7020)` |
| `ldr` at 0xa7700 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa770c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa771c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7720 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa7724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7728 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa773c | store | 4 | `sp -0x24` | `ret(0xa7020)` |
| `ldr` at 0xa7740 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xa774c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa774c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa774c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa76f4 | `(unknown callee) 0xa7020` | `r0` | `r1` | `0x1` | `0x1` |
| `bl` at 0xa7734 | `(unknown callee) 0xa7020` | `?` | `?` | `0x2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa76d8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa7708 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa7718 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa774c | `r0` | `?` |

## `IRouterEventRegisterFunction`

- `IRouterEventRegisterFunction`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7a8c to 0xa7ad8.
  Established. The word at `sp -0x24` receives entry `r3` (`str` at 0xa7ab0).
  The unnamed `0xa7750` call carries entry `r0`, entry `r1`, entry `r2`, `0x0` (`bl` at 0xa7ac4).
  The return record gives its result in `r0` (`ldm` at 0xa7ad4).
  Event function registration meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7a9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7aa0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7aa4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa7aa8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa7aac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7ab0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xa7ab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7abc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa7ad4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa7ad4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa7ad4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7ac4 | `(unknown callee) 0xa7750` | `r0` | `r1` | `r2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa7ad4 | `r0` | `ret(0xa7750)` |

## `IRouterEventRegisterQueue`

- `IRouterEventRegisterQueue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7ad8 to 0xa7b24.
  Established. Entry `r3` is written as a word at `sp -0x24` (`str` at 0xa7afc).
  The unnamed `0xa7750` call receives entry `r0`, entry `r1`, entry `r2`, `0x1` (`bl` at 0xa7b10).
  The included return record carries `ret(0xa7750)` (`ldm` at 0xa7b20).
  Event queue binding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7ae8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7aec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7af0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa7af4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa7af8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7afc | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xa7b00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7b08 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa7b20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa7b20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa7b20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7b10 | `(unknown callee) 0xa7750` | `r0` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa7b20 | `r0` | `ret(0xa7750)` |

## `IRouterCreateQueue`

- `IRouterCreateQueue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8af0 to 0xa8b38.
  Established. A word assignment at `sp -0x20` writes `0x1` (`str` at 0xa8b10).
  The unnamed `0x21228` call carries entry `r0`, entry `r1`, entry `r2`, `0x7` (`bl` at 0xa8b24);
  its result is the included `r0` return value (`ldm` at 0xa8b34).
  Queue creation parameters remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8b00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8b04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa8b08 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa8b10 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xa8b14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8b18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8b1c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa8b34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8b34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8b34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8b24 | `(unknown callee) 0x21228` | `r0` | `r1` | `r2` | `0x7` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8b34 | `r0` | `ret(0x21228)` |

## `IRouterGetPacket`

- `IRouterGetPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8b38 to 0xa8b98.
  Established. The unnamed `0x215f0` call carries entry `r1`, entry `r0`, entry `r2`, entry `r3` (`bl` at 0xa8b60);
  its zero result takes `0xa8b84`, while nonzero falls through (`beq` at 0xa8b74).
  The taken arm writes zero to `sp -0x28` as a word (`str` at 0xa8b88).
  Packet retrieval behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8b48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8b4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa8b50 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xa8b54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8b5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8b68 | store | 4 | `sp -0x24` | `ret(0x215f0)` |
| `ldr` at 0xa8b6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8b78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa8b7c | store | 4 | `sp -0x28` | `ret(0x215f0)` |
| `str` at 0xa8b88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa8b8c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xa8b94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8b94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8b94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8b60 | `(unknown callee) 0x215f0` | `r1` | `r0` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa8b74 | `ret(0x215f0) == 0x0` | `ret(0x215f0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8b94 | `r0` | `[sp-40]` |

## `IRouterSetAuthority`

- `IRouterSetAuthority`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8abc to 0xa8af0.
  Established. The byte through entry `r0 +0x4` receives `(r1 & 0xff)` (`strb` at 0xa8adc).
  The included return record carries zero in `r0` (`ldm` at 0xa8aec).
  Authority encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8acc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8ad0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa8ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ad8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa8adc | store | 1 | `r0 +0x4` | `(r1 & 0xff)` |
| `ldm` at 0xa8aec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8aec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8aec | `r0` | `0x0` |

## `IRouterSetQueueSuspendTime`

- `IRouterSetQueueSuspendTime`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8f60 to 0xa8fa0.
  Established. The absolute word at `0x12d8c4` receives entry `r0` (`str` at 0xa8f80).
  The absolute word at `0x12d8c8` receives `[sp-24]` (`str` at 0xa8f8c).
  The included integer return record retains entry `r0` (`ldm` at 0xa8f94).
  Queue suspension units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8f70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8f74 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa8f78 | load | 4 | `abs 0xa8f98` | `-` |
| `ldr` at 0xa8f7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8f80 | store | 4 | `abs 0x12d8c4` | `r0` |
| `ldr` at 0xa8f84 | load | 4 | `abs 0xa8f9c` | `-` |
| `ldr` at 0xa8f88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8f8c | store | 4 | `abs 0x12d8c8` | `[sp-24]` |
| `ldm` at 0xa8f94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8f94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8f94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8f94 | `r0` | `r0` |

## `IRouterGetQueueSuspendTime`

- `IRouterGetQueueSuspendTime`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8fa0 to 0xa8fe8.
  Established. The absolute words at `0x12d8c4` and `0x12d8c8` are loaded (`ldr` at 0xa8fc0, `ldr` at 0xa8fd0).
  Word stores through entry `r0 +0x0` and `[sp-24] +0x0` write `[0x12d8c4]` and `[0x12d8c8]` (`str` at 0xa8fc4, `str` at 0xa8fd4).
  Suspension setting interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8fb0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8fb4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa8fb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8fbc | load | 4 | `abs 0xa8fe0` | `-` |
| `ldr` at 0xa8fc0 | load | 4 | `abs 0x12d8c4` | `-` |
| `str` at 0xa8fc4 | store | 4 | `r0 +0x0` | `[0x12d8c4]` |
| `ldr` at 0xa8fc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8fcc | load | 4 | `abs 0xa8fe4` | `-` |
| `ldr` at 0xa8fd0 | load | 4 | `abs 0x12d8c8` | `-` |
| `str` at 0xa8fd4 | store | 4 | `[sp-24] +0x0` | `[0x12d8c8]` |
| `ldm` at 0xa8fdc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8fdc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8fdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8fdc | `r0` | `r0` |
