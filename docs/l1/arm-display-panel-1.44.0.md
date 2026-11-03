# ARM display and panel paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/display-panel-facts.txt) retain all tool sections.

## `displayProbe`

- `displayProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x229c8 to 0x22b80.
  Established. An included byte assignment at `0x131f8d` writes `0x1` (`strb` at 0x22a88). Separate
  included assignments write byte `0x76` at `0x131fe1` and byte `0x10` at `0x131fe2` (`strb` at 0x22ad0, `strb` at 0x22b54). The indirect call carries `0x131fe0`, unknown, `0x131fe0`,
  `[0x12d8e4]` (`mov` at 0x22af8). Display probing effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x229dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x229e0 | load | 4 | `abs 0x22b60` | `-` |
| `ldrb` at 0x229e4 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x229f0 | load | 4 | `abs 0x22b64` | `-` |
| `str` at 0x229f4 | store | 4 | `sp -0x28` | `0xee25c` |
| `ldr` at 0x229fc | load | 4 | `abs 0x22b68` | `-` |
| `ldrb` at 0x22a0c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x22a24 | store | 4 | `sp -0x1c` | `ret(0xca70c)` |
| `ldr` at 0x22a28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22a34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22a40 | load | 4 | `abs 0x22b6c` | `-` |
| `str` at 0x22a44 | store | 4 | `sp -0x28` | `0xee28c` |
| `str` at 0x22a48 | store | 4 | `sp -0x24` | `ret(0x1fc1c)` |
| `ldr` at 0x22a50 | load | 4 | `abs 0x22b68` | `-` |
| `ldrb` at 0x22a60 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x22a74 | load | 4 | `abs 0x22b60` | `-` |
| `ldrb` at 0x22a78 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x22a7c | store | 1 | `abs 0x12d3ec` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x22a80 | load | 4 | `abs 0x22b70` | `-` |
| `strb` at 0x22a88 | store | 1 | `abs 0x131f8d` | `0x1` |
| `ldr` at 0x22a8c | load | 4 | `abs 0x22b60` | `-` |
| `ldrb` at 0x22a90 | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x22aa4 | store | 4 | `sp -0x18` | `ret(0xcb198)` |
| `ldr` at 0x22aa8 | load | 4 | `abs 0x22b74` | `-` |
| `ldr` at 0x22aac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x22ab0 | load | 1 | `ret(0xcb198) +0x1` | `-` |
| `strb` at 0x22ab4 | store | 1 | `abs 0x131fe0` | `([ret(0xcb198)+1]:1 & 0xff)` |
| `ldr` at 0x22ab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x22abc | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x22ac8 | load | 4 | `abs 0x22b74` | `-` |
| `strb` at 0x22ad0 | store | 1 | `abs 0x131fe1` | `0x76` |
| `ldr` at 0x22ad4 | load | 4 | `abs 0x22b74` | `-` |
| `ldr` at 0x22ad8 | load | 4 | `abs 0x22b74` | `-` |
| `ldrb` at 0x22adc | load | 1 | `abs 0x131fe4` | `-` |
| `strb` at 0x22ae4 | store | 1 | `abs 0x131fe4` | `?` |
| `ldr` at 0x22ae8 | load | 4 | `abs 0x22b78` | `-` |
| `ldr` at 0x22aec | load | 4 | `abs 0x12d8e4` | `-` |
| `ldr` at 0x22af0 | load | 4 | `abs 0x22b74` | `-` |
| `ldr` at 0x22b00 | load | 4 | `abs 0x22b74` | `-` |
| `strb` at 0x22b08 | store | 1 | `abs 0x131fe1` | `?` |
| `ldr` at 0x22b0c | load | 4 | `abs 0x22b74` | `-` |
| `ldr` at 0x22b14 | load | 4 | `abs 0x22b74` | `-` |
| `ldrb` at 0x22b18 | load | 1 | `abs 0x131fe2` | `-` |
| `ldr` at 0x22b24 | load | 4 | `abs 0x22b7c` | `-` |
| `str` at 0x22b28 | store | 4 | `sp -0x28` | `0xee2b8` |
| `ldr` at 0x22b2c | load | 4 | `abs 0x22b74` | `-` |
| `ldrb` at 0x22b30 | load | 1 | `abs 0x131fe2` | `-` |
| `str` at 0x22b34 | store | 4 | `sp -0x24` | `[0x131fe2]:1` |
| `ldr` at 0x22b3c | load | 4 | `abs 0x22b68` | `-` |
| `ldr` at 0x22b4c | load | 4 | `abs 0x22b74` | `-` |
| `strb` at 0x22b54 | store | 1 | `abs 0x131fe2` | `0x10` |
| `ldm` at 0x22b5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x22b5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x22b5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22a08 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0x81` | `0x0` |
| `bl` at 0x22a1c | `(unknown callee) 0xca70c` | `0x0` | `?` | `sp-29` | `?` |
| `bl` at 0x22a38 | `(unknown callee) 0x1fc1c` | `ret(0xca70c)` | `?` | `?` | `ret(0xca70c)` |
| `bl` at 0x22a5c | `(unknown callee) 0x200a0` | `0x1` | `0xee274` | `0x87` | `0x0` |
| `bl` at 0x22a6c | `(unknown callee) 0xc9a44` | `0x0` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0x22a9c | `(unknown callee) 0xcb198` | `0x0` | `[0x12d3ec]:1` | `0x131f8d` | `[0x12d3ec]:1` |
| `mov` at 0x22af8 | `(unknown callee) (unknown target)` | `0x131fe0` | `?` | `0x131fe0` | `[0x12d8e4]` |
| `bl` at 0x22b10 | `(unknown callee) 0xcee80` | `0x131fe0` | `?` | `?` | `0x131fe0` |
| `bl` at 0x22b48 | `(unknown callee) 0x200a0` | `0x1` | `0xee274` | `0xa4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x229ec | `[0x12d3ec]:1 == 0xff` | `[0x12d3ec]:1 != 0xff` |
| `beq` at 0x22a30 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `bne` at 0x22ac4 | `[[sp-24]]:1 != 0x2` | `[[sp-24]]:1 == 0x2` |
| `bls` at 0x22b20 | `[0x131fe2]:1 <= 0x10 (unsigned, 32-bit)` | `[0x131fe2]:1 > 0x10 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x22b5c | `r0` | `?` |

## `displayInit`

- `displayInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22ce4 to 0x22e04.
  Established. The absolute byte at `0x131f8c` receives `0x1` (`strb` at 0x22d3c). The unnamed
  `0x23664` call carries `0x1`, unknown, `0x131f8c`, `0x1` (`bl` at 0x22d44); its zero result takes
  `0x22d88`, while nonzero falls through (`beq` at 0x22d58). Display initialization policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x22cf4 | load | 4 | `abs 0x22de0` | `-` |
| `ldrb` at 0x22cf8 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x22d08 | load | 4 | `abs 0x22de4` | `-` |
| `ldrb` at 0x22d0c | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x22d18 | load | 4 | `abs 0x22de8` | `-` |
| `str` at 0x22d1c | store | 4 | `sp -0x1c` | `0xee2d0` |
| `ldr` at 0x22d24 | load | 4 | `abs 0x22dec` | `-` |
| `ldr` at 0x22d34 | load | 4 | `abs 0x22de4` | `-` |
| `strb` at 0x22d3c | store | 1 | `abs 0x131f8c` | `0x1` |
| `str` at 0x22d4c | store | 4 | `sp -0x14` | `ret(0x23664)` |
| `ldr` at 0x22d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22d5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22d68 | load | 4 | `abs 0x22df0` | `-` |
| `str` at 0x22d6c | store | 4 | `sp -0x1c` | `0xee2ec` |
| `str` at 0x22d70 | store | 4 | `sp -0x18` | `ret(0x1fc1c)` |
| `ldr` at 0x22d78 | load | 4 | `abs 0x22dec` | `-` |
| `ldr` at 0x22d7c | load | 4 | `abs 0x22df4` | `-` |
| `ldr` at 0x22d8c | load | 4 | `abs 0x22df8` | `-` |
| `str` at 0x22d9c | store | 4 | `sp -0x14` | `ret(0x22e24)` |
| `ldr` at 0x22da0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22dac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22db8 | load | 4 | `abs 0x22dfc` | `-` |
| `str` at 0x22dbc | store | 4 | `sp -0x1c` | `0xee310` |
| `str` at 0x22dc0 | store | 4 | `sp -0x18` | `ret(0x1fc1c)` |
| `ldr` at 0x22dc8 | load | 4 | `abs 0x22dec` | `-` |
| `ldr` at 0x22dcc | load | 4 | `abs 0x22e00` | `-` |
| `ldm` at 0x22ddc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x22ddc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x22ddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22d30 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0xeb` | `0x0` |
| `bl` at 0x22d44 | `(unknown callee) 0x23664` | `0x1` | `?` | `0x131f8c` | `0x1` |
| `bl` at 0x22d60 | `(unknown callee) 0x1fc1c` | `ret(0x23664)` | `?` | `?` | `ret(0x23664)` |
| `bl` at 0x22d84 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x106` | `0x0` |
| `bl` at 0x22d94 | `(unknown callee) 0x22e24` | `0x32` | `0xee304` | `0x0` | `?` |
| `bl` at 0x22db0 | `(unknown callee) 0x1fc1c` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x22dd4 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x116` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x22d00 | `[0x12d3ec]:1 != 0xff` | `[0x12d3ec]:1 == 0xff` |
| `beq` at 0x22d14 | `[0x131f8c]:1 == 0x0` | `[0x131f8c]:1 != 0x0` |
| `beq` at 0x22d58 | `ret(0x23664) == 0x0` | `ret(0x23664) != 0x0` |
| `beq` at 0x22da8 | `ret(0x22e24) == 0x0` | `ret(0x22e24) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x22ddc | `r0` | `?` |

## `displayPresent`

- `displayPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22e04 to 0x22e24.
  Established. The byte at `0x131f8d` is read (`ldrb` at 0x22e14). The included return record
  carries `[0x131f8d]:1` in integer `r0` (`ldm` at 0x22e1c). Presence flag meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x22e10 | load | 4 | `abs 0x22e20` | `-` |
| `ldrb` at 0x22e14 | load | 1 | `abs 0x131f8d` | `-` |
| `ldm` at 0x22e1c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x22e1c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x22e1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x22e1c | `r0` | `[0x131f8d]:1` |

## `displayString`

- `displayString`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22e24 to 0x230a0.
  Established. Entry `r0 != 0x31` takes `0x22e80`, while equality falls through (`bne` at 0x22e64).
  Included word assignments at `sp -0x20` write `0x132000` and `0x132011` in separate arms (`str` at 0x22e6c, `str` at 0x22ec8). The indirect call at `0x23030` carries `[0x131fe8]`, `[sp-32]`,
  `[sp-36]`, `[0x131ff8]` (`mov` at 0x23030). Display string transfer policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x22e34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x22e38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x22e3c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x22e40 | load | 4 | `abs 0x23080` | `-` |
| `ldrb` at 0x22e44 | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x22e50 | load | 4 | `abs 0x23084` | `-` |
| `str` at 0x22e54 | store | 4 | `sp -0x30` | `0xfffff8f7` |
| `ldr` at 0x22e5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x22e68 | load | 4 | `abs 0x23088` | `-` |
| `str` at 0x22e6c | store | 4 | `sp -0x20` | `0x132000` |
| `ldr` at 0x22e70 | load | 4 | `abs 0x2308c` | `-` |
| `ldrb` at 0x22e74 | load | 1 | `abs 0x131fe2` | `-` |
| `str` at 0x22e78 | store | 4 | `sp -0x24` | `[0x131fe2]:1` |
| `ldr` at 0x22e80 | load | 4 | `abs 0x23090` | `-` |
| `ldrb` at 0x22e84 | load | 1 | `abs 0x131f8e` | `-` |
| `ldr` at 0x22e90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22e9c | load | 4 | `abs 0x23094` | `-` |
| `ldr` at 0x22ea0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22eac | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x22eb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22ec4 | load | 4 | `abs 0x23098` | `-` |
| `str` at 0x22ec8 | store | 4 | `sp -0x20` | `0x132011` |
| `ldr` at 0x22ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22edc | load | 4 | `abs 0x2308c` | `-` |
| `ldrb` at 0x22ee0 | load | 1 | `abs 0x131fe2` | `-` |
| `str` at 0x22ef0 | store | 4 | `sp -0x24` | `0x48` |
| `ldr` at 0x22ef8 | load | 4 | `abs 0x2308c` | `-` |
| `ldrb` at 0x22efc | load | 1 | `abs 0x131fe2` | `-` |
| `str` at 0x22f00 | store | 4 | `sp -0x24` | `[0x131fe2]:1` |
| `ldr` at 0x22f04 | load | 4 | `abs 0x23094` | `-` |
| `ldr` at 0x22f08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22f14 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x22f18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x22f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x22f28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x22f2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x22f38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x22f3c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x22f44 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x22f48 | load | 4 | `abs 0x2308c` | `-` |
| `ldrb` at 0x22f4c | load | 1 | `abs 0x131fe2` | `-` |
| `ldr` at 0x22f50 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x22f60 | store | 1 | `sp -0x35` | `0x0` |
| `strb` at 0x22f6c | store | 1 | `sp -0x35` | `0x20` |
| `ldrb` at 0x22f70 | load | 1 | `sp -0x35` | `-` |
| `ldr` at 0x22f74 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x22f78 | store | 1 | `(unknown)` | `([sp-53]:1 & 0xff)` |
| `ldr` at 0x22f80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x22f84 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x22f8c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x22f9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x22fa0 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x22fa8 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x22fb0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x22fc0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x22fc4 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x22fc8 | store | 1 | `sp -0x3d` | `([[sp-24]]:1 & 0xff)` |
| `strb` at 0x22fd4 | store | 1 | `sp -0x3d` | `0x3f` |
| `ldrb` at 0x22fd8 | load | 1 | `sp -0x3d` | `-` |
| `ldr` at 0x22fdc | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x22fe0 | store | 1 | `[sp-60] +0x0` | `?` |
| `ldr` at 0x22fe4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22fec | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x22ff0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x22ff8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x23000 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x23004 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x23010 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x23014 | load | 4 | `abs 0x2308c` | `-` |
| `ldr` at 0x23018 | load | 4 | `abs 0x2308c` | `-` |
| `ldr` at 0x2301c | load | 4 | `abs 0x131ff8` | `-` |
| `ldr` at 0x23020 | load | 4 | `abs 0x131fe8` | `-` |
| `ldr` at 0x23024 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x23028 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x23034 | load | 4 | `abs 0x2309c` | `-` |
| `ldrb` at 0x23038 | load | 1 | `abs 0x13205a` | `-` |
| `ldr` at 0x23044 | load | 4 | `abs 0x2308c` | `-` |
| `ldr` at 0x23048 | load | 4 | `abs 0x131ff0` | `-` |
| `ldr` at 0x2304c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x23050 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2305c | store | 4 | `sp -0x44` | `ret(unknown)` |
| `str` at 0x23068 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x2306c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x23070 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x23074 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x2307c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2307c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2307c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22ea4 | `(unknown callee) 0x19808` | `0x12d3f0` | `r1` | `r2` | `r2` |
| `bl` at 0x22ec0 | `(unknown callee) 0x232f4` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x22ed0 | `(unknown callee) 0x19538` | `[sp-24]` | `?` | `?` | `0x132011` |
| `bl` at 0x22f0c | `(unknown callee) 0x19808` | `0x12d3f0` | `[sp-24]` | `ret(0x19538)` | `?` |
| `bl` at 0x22f90 | `(unknown callee) 0x197d8` | `?` | `?` | `?` | `?` |
| `mov` at 0x23030 | `(unknown callee) (unknown target)` | `[0x131fe8]` | `[sp-32]` | `[sp-36]` | `[0x131ff8]` |
| `mov` at 0x23058 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-36]` | `?` | `[0x131ff0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x22e4c | `[0x131f8c]:1 != 0x0` | `[0x131f8c]:1 == 0x0` |
| `bne` at 0x22e64 | `r0 != 0x31` | `r0 == 0x31` |
| `beq` at 0x22e8c | `[0x131f8e]:1 == 0x0` | `[0x131f8e]:1 != 0x0` |
| `bne` at 0x22e98 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0x22ebc | `r2 != 0x2` | `r2 == 0x2` |
| `bls` at 0x22ee8 | `ret(0x19538) <= [0x131fe2]:1 (unsigned, 32-bit)` | `ret(0x19538) > [0x131fe2]:1 (unsigned, 32-bit)` |
| `bge` at 0x22f24 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x22f34 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x22f58 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x22fbc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x23040 | `[0x13205a]:1 != 0x0` | `[0x13205a]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2307c | `r0` | `?` |

## `displayChar`

- `displayChar`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x230a0 to 0x23210.
  Established. The unsigned condition `[0x131fe2]:1 > (r1 & 0xff)` takes `0x2311c`, while less or
  equal values fall through (`bhi` at 0x2310c). An included byte assignment at `sp -0x16` writes
  `0x3f` (`strb` at 0x2314c). The indirect call at `0x2319c` carries `sp-23`, `sp-22`, `0x1`,
  `[0x131ff8]` (`mov` at 0x2319c). Character transfer semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x230b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x230b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x230c0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x230c4 | load | 4 | `abs 0x231f8` | `-` |
| `ldrb` at 0x230c8 | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x230d4 | load | 4 | `abs 0x231fc` | `-` |
| `str` at 0x230d8 | store | 4 | `sp -0x1c` | `0xfffff8f7` |
| `ldr` at 0x230e0 | load | 4 | `abs 0x23200` | `-` |
| `ldr` at 0x230e4 | load | 4 | `abs 0x131ff4` | `-` |
| `str` at 0x230f4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x230fc | load | 4 | `abs 0x23200` | `-` |
| `ldrb` at 0x23100 | load | 1 | `abs 0x131fe2` | `-` |
| `ldrb` at 0x23104 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x23114 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x23128 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x23130 | load | 4 | `ret(0x197d8) +0x0` | `-` |
| `ldrh` at 0x23138 | load | 2 | `(unknown)` | `-` |
| `strb` at 0x2314c | store | 1 | `sp -0x16` | `0x3f` |
| `ldr` at 0x23150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2315c | load | 4 | `abs 0x23204` | `-` |
| `ldrb` at 0x23160 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x23164 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x23168 | store | 1 | `[sp-21]:1 +0x132000` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x23170 | load | 4 | `abs 0x23208` | `-` |
| `ldrb` at 0x23174 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x23178 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x2317c | store | 1 | `[sp-21]:1 +0x132011` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x23180 | load | 4 | `abs 0x23200` | `-` |
| `ldr` at 0x2318c | load | 4 | `abs 0x131ff8` | `-` |
| `ldr` at 0x231a0 | load | 4 | `abs 0x2320c` | `-` |
| `ldrb` at 0x231a4 | load | 1 | `abs 0x13205a` | `-` |
| `ldr` at 0x231b0 | load | 4 | `abs 0x23200` | `-` |
| `ldrb` at 0x231b4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x231b8 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x231bc | load | 4 | `abs 0x131ff4` | `-` |
| `ldr` at 0x231c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x231d4 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `str` at 0x231e0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x231e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x231e8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x231ec | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x231f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x231f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x231f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2311c | `(unknown callee) 0x197d8` | `r0` | `r1` | `[0x131fe2]:1` | `(r1 & 0xff)` |
| `mov` at 0x2319c | `(unknown callee) (unknown target)` | `sp-23` | `sp-22` | `0x1` | `[0x131ff8]` |
| `mov` at 0x231d0 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-21]:1` | `[sp-23]:1` | `[0x131ff4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x230d0 | `[0x131f8c]:1 != 0x0` | `[0x131f8c]:1 == 0x0` |
| `bne` at 0x230ec | `[0x131ff4] != 0x0` | `[0x131ff4] == 0x0` |
| `bhi` at 0x2310c | `[0x131fe2]:1 > (r1 & 0xff) (unsigned, 32-bit)` | `[0x131fe2]:1 <= (r1 & 0xff) (unsigned, 32-bit)` |
| `bne` at 0x23144 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x23158 | `[sp-20] != 0x31` | `[sp-20] == 0x31` |
| `bne` at 0x231ac | `[0x13205a]:1 != 0x0` | `[0x13205a]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x231f4 | `r0` | `?` |

## `displayHeatbeat`

- `displayHeatbeat`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23210 to 0x232f4.
  Established. An included byte assignment at `0x131fd8` writes `([0x131fd8]:1+1 & 0xff)` (`strb` at 0x232a4). A zero word at `0x131ff4` takes `0x23244`, while nonzero falls through (`beq` at 0x2322c). The indirect call carries `0x31`, unknown, unknown, unknown (`mov` at 0x232c8). Display
  heartbeat behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x23220 | load | 4 | `abs 0x232dc` | `-` |
| `ldr` at 0x23224 | load | 4 | `abs 0x131ff4` | `-` |
| `ldr` at 0x23230 | load | 4 | `abs 0x232e0` | `-` |
| `ldrb` at 0x23234 | load | 1 | `abs 0x13205a` | `-` |
| `str` at 0x23248 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x23250 | load | 4 | `abs 0x232e4` | `-` |
| `ldr` at 0x23254 | load | 4 | `abs 0x232dc` | `-` |
| `ldrb` at 0x23258 | load | 1 | `abs 0x131fe2` | `-` |
| `ldr` at 0x23268 | load | 4 | `abs 0x232e8` | `-` |
| `ldr` at 0x2326c | load | 4 | `abs 0x232ec` | `-` |
| `ldrb` at 0x23270 | load | 1 | `abs 0x131fd8` | `-` |
| `ldrb` at 0x23278 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x2327c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x23280 | load | 4 | `abs 0x232dc` | `-` |
| `ldr` at 0x23284 | load | 4 | `abs 0x232dc` | `-` |
| `ldrb` at 0x23288 | load | 1 | `abs 0x131fe2` | `-` |
| `ldr` at 0x23294 | load | 4 | `abs 0x232f0` | `-` |
| `ldr` at 0x23298 | load | 4 | `abs 0x232ec` | `-` |
| `ldrb` at 0x2329c | load | 1 | `abs 0x131fd8` | `-` |
| `strb` at 0x232a4 | store | 1 | `abs 0x131fd8` | `([0x131fd8]:1+1 & 0xff)` |
| `ldrb` at 0x232b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x232b4 | load | 4 | `abs 0x131ff4` | `-` |
| `str` at 0x232d0 | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0x232d4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x232d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x232d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x232d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x232d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x232c8 | `(unknown callee) (unknown target)` | `0x31` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2322c | `[0x131ff4] == 0x0` | `[0x131ff4] != 0x0` |
| `bne` at 0x2323c | `[0x13205a]:1 != 0x0` | `[0x13205a]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x232d8 | `r0` | `?` |

## `displayLock`

- `displayLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x232f4 to 0x23314.
  Established. The absolute byte at `0x131f8e` receives `0x1` (`strb` at 0x23308). The included
  return record retains entry `r0` (`ldm` at 0x2330c). Lock ownership semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x23300 | load | 4 | `abs 0x23310` | `-` |
| `strb` at 0x23308 | store | 1 | `abs 0x131f8e` | `0x1` |
| `ldm` at 0x2330c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2330c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2330c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2330c | `r0` | `r0` |

## `displayUnlock`

- `displayUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23314 to 0x23348.
  Established. The byte at `0x131f8e` receives zero (`strb` at 0x23328). The unnamed `0x22e24` call
  carries `0x32`, `0x12d3f0`, `0x0`, `0x131f8e` (`bl` at 0x23338). The included return record
  carries `ret(0x22e24)` in `r0` (`ldm` at 0x2333c). Display release policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x23320 | load | 4 | `abs 0x23340` | `-` |
| `strb` at 0x23328 | store | 1 | `abs 0x131f8e` | `0x0` |
| `ldr` at 0x23330 | load | 4 | `abs 0x23344` | `-` |
| `ldm` at 0x2333c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2333c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2333c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23338 | `(unknown callee) 0x22e24` | `0x32` | `0x12d3f0` | `0x0` | `0x131f8e` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2333c | `r0` | `ret(0x22e24)` |

## `displayLedOn`

- `displayLedOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23348 to 0x23458.
  Established. The unnamed `0xca538` call at `0x233e0` carries `0x0`, `[0x12d3ec]:1`, `0x10`,
  `[0x12d3ec]:1` (`bl` at 0x233e0). A nonzero byte at `0x131f8c` takes `0x23378`, while zero falls
  through (`bne` at 0x23368). The word at `sp -0x18` receives `0xfffff8f7` in the included fallback
  arm (`str` at 0x23370). Display LED activation meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23358 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2335c | load | 4 | `abs 0x23444` | `-` |
| `ldrb` at 0x23360 | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x2336c | load | 4 | `abs 0x23448` | `-` |
| `str` at 0x23370 | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x23378 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2337c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x23380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2338c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x233a8 | load | 4 | `abs 0x2344c` | `-` |
| `ldrb` at 0x233ac | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x233c4 | store | 4 | `sp -0x18` | `ret(0xca538)` |
| `ldr` at 0x233cc | load | 4 | `abs 0x2344c` | `-` |
| `ldrb` at 0x233d0 | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x233e8 | store | 4 | `sp -0x18` | `ret(0xca538)` |
| `ldr` at 0x233f0 | load | 4 | `abs 0x2344c` | `-` |
| `ldrb` at 0x233f4 | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x2340c | store | 4 | `sp -0x18` | `ret(0xca538)` |
| `ldr` at 0x23414 | load | 4 | `abs 0x23450` | `-` |
| `str` at 0x23418 | store | 4 | `sp -0x24` | `0xee328` |
| `ldr` at 0x2341c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23420 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x23428 | load | 4 | `abs 0x23454` | `-` |
| `ldr` at 0x23438 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x23440 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x23440 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x23440 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x233bc | `(unknown callee) 0xca538` | `0x0` | `[0x12d3ec]:1` | `0x40` | `[0x12d3ec]:1` |
| `bl` at 0x233e0 | `(unknown callee) 0xca538` | `0x0` | `[0x12d3ec]:1` | `0x10` | `[0x12d3ec]:1` |
| `bl` at 0x23404 | `(unknown callee) 0xca538` | `0x0` | `[0x12d3ec]:1` | `0x20` | `[0x12d3ec]:1` |
| `bl` at 0x23434 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0x218` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x23368 | `[0x131f8c]:1 != 0x0` | `[0x131f8c]:1 == 0x0` |
| `beq` at 0x23388 | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x23394 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x233a0 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x23440 | `r0` | `?` |

## `displayLedOff`

- `displayLedOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23458 to 0x2356c.
  Established. The unnamed `0xca4e8` call at `0x23514` carries `0x0`, `[0x12d3ec]:1`, `0x20`,
  `[0x12d3ec]:1` (`bl` at 0x23514). Entry `r0 == 0x1` takes `0x234dc`, while inequality falls
  through (`beq` at 0x23498). The byte at `0x12d3ec` is read on an included path (`ldrb` at 0x234e0). Display LED deactivation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23468 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2346c | load | 4 | `abs 0x23554` | `-` |
| `ldrb` at 0x23470 | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x2347c | load | 4 | `abs 0x23558` | `-` |
| `str` at 0x23480 | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x23488 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2348c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x23490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2349c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x234a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x234b8 | load | 4 | `abs 0x2355c` | `-` |
| `ldrb` at 0x234bc | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x234d4 | store | 4 | `sp -0x18` | `ret(0xca4e8)` |
| `ldr` at 0x234dc | load | 4 | `abs 0x2355c` | `-` |
| `ldrb` at 0x234e0 | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x234f8 | store | 4 | `sp -0x18` | `ret(0xca4e8)` |
| `ldr` at 0x23500 | load | 4 | `abs 0x2355c` | `-` |
| `ldrb` at 0x23504 | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x2351c | store | 4 | `sp -0x18` | `ret(0xca4e8)` |
| `ldr` at 0x23524 | load | 4 | `abs 0x23560` | `-` |
| `str` at 0x23528 | store | 4 | `sp -0x24` | `0xee328` |
| `ldr` at 0x2352c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23530 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x23538 | load | 4 | `abs 0x23564` | `-` |
| `ldr` at 0x2353c | load | 4 | `abs 0x23568` | `-` |
| `ldr` at 0x23548 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x23550 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x23550 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x23550 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x234cc | `(unknown callee) 0xca4e8` | `0x0` | `[0x12d3ec]:1` | `0x40` | `[0x12d3ec]:1` |
| `bl` at 0x234f0 | `(unknown callee) 0xca4e8` | `0x0` | `[0x12d3ec]:1` | `0x10` | `[0x12d3ec]:1` |
| `bl` at 0x23514 | `(unknown callee) 0xca4e8` | `0x0` | `[0x12d3ec]:1` | `0x20` | `[0x12d3ec]:1` |
| `bl` at 0x23544 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0x235` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x23478 | `[0x131f8c]:1 != 0x0` | `[0x131f8c]:1 == 0x0` |
| `beq` at 0x23498 | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x234a4 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x234b0 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x23550 | `r0` | `?` |

## `displayLedStatus`

- `displayLedStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2356c to 0x23664.
  Established. Separate included byte assignments at `sp -0x15` write `0x40`, `0x10` and `0x20`
  (`strb` at 0x235d0, `strb` at 0x235dc, `strb` at 0x235e8). The unnamed `0xca7a0` call carries
  `0x0`, `[0x12d3ec]:1`, unknown, `[0x12d3ec]:1` (`bl` at 0x23624). An included word assignment at
  `sp -0x1c` writes `0xfffff8f8` (`str` at 0x23594). Display LED status interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2357c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23580 | load | 4 | `abs 0x23654` | `-` |
| `ldrb` at 0x23584 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x23590 | load | 4 | `abs 0x23658` | `-` |
| `str` at 0x23594 | store | 4 | `sp -0x1c` | `0xfffff8f8` |
| `ldr` at 0x2359c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x235a0 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x235a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x235b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x235bc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x235d0 | store | 1 | `sp -0x15` | `0x40` |
| `strb` at 0x235dc | store | 1 | `sp -0x15` | `0x10` |
| `strb` at 0x235e8 | store | 1 | `sp -0x15` | `0x20` |
| `ldr` at 0x235f0 | load | 4 | `abs 0x2365c` | `-` |
| `str` at 0x235f4 | store | 4 | `sp -0x28` | `0xee328` |
| `ldr` at 0x235f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x235fc | store | 4 | `sp -0x24` | `r0` |
| `ldr` at 0x23604 | load | 4 | `abs 0x23660` | `-` |
| `ldr` at 0x23614 | load | 4 | `abs 0x23654` | `-` |
| `ldrb` at 0x23618 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldrb` at 0x2362c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x23644 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x23648 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x23650 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x23650 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x23650 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23610 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0x258` | `0x0` |
| `bl` at 0x23624 | `(unknown callee) 0xca7a0` | `0x0` | `[0x12d3ec]:1` | `?` | `[0x12d3ec]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2358c | `[0x12d3ec]:1 != 0xff` | `[0x12d3ec]:1 == 0xff` |
| `beq` at 0x235ac | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x235b8 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x235c4 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x23650 | `r0` | `?` |

## `displayReset`

- `displayReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23664 to 0x23874.
  Established. Included byte assignments at `0x13205a` write `0x1` and zero at separate sites
  (`strb` at 0x236f8, `strb` at 0x23798). The indirect call at `0x2378c` carries `0x13205b`,
  `0xee254`, `0x4`, `[0x131ff8]` (`mov` at 0x2378c). The unnamed `0xca584` call at `0x237bc` carries
  `0x0`, `[0x12d3ec]:1`, `0x70`, `[sp-21]:1` (`bl` at 0x237bc). Display reset sequencing remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23674 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23678 | load | 4 | `abs 0x23844` | `-` |
| `ldrb` at 0x2367c | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x23688 | load | 4 | `abs 0x23848` | `-` |
| `str` at 0x2368c | store | 4 | `sp -0x20` | `0xfffff8f7` |
| `ldr` at 0x23694 | load | 4 | `abs 0x2384c` | `-` |
| `ldrb` at 0x23698 | load | 1 | `abs 0x12d3ec` | `-` |
| `strb` at 0x236b0 | store | 1 | `sp -0x15` | `?` |
| `ldr` at 0x236b4 | load | 4 | `abs 0x2384c` | `-` |
| `ldrb` at 0x236b8 | load | 1 | `abs 0x12d3ec` | `-` |
| `str` at 0x236d4 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x236d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x236e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x236e8 | store | 4 | `sp -0x20` | `ret(0xca584)` |
| `ldr` at 0x236f0 | load | 4 | `abs 0x23850` | `-` |
| `strb` at 0x236f8 | store | 1 | `abs 0x13205a` | `0x1` |
| `ldr` at 0x236fc | load | 4 | `abs 0x23854` | `-` |
| `ldr` at 0x23700 | load | 4 | `abs 0x2384c` | `-` |
| `ldrb` at 0x23704 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x23708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x23724 | load | 4 | `abs 0x131fec` | `-` |
| `str` at 0x23738 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x2373c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23754 | load | 4 | `abs 0x23858` | `-` |
| `str` at 0x23758 | store | 4 | `sp -0x28` | `0xee2ec` |
| `str` at 0x2375c | store | 4 | `sp -0x24` | `ret(0x1fc1c)` |
| `ldr` at 0x23764 | load | 4 | `abs 0x2385c` | `-` |
| `ldr` at 0x23768 | load | 4 | `abs 0x23860` | `-` |
| `ldr` at 0x23774 | load | 4 | `abs 0x23854` | `-` |
| `ldr` at 0x23778 | load | 4 | `abs 0x131ff8` | `-` |
| `ldr` at 0x2377c | load | 4 | `abs 0x23864` | `-` |
| `ldr` at 0x23780 | load | 4 | `abs 0x23868` | `-` |
| `ldr` at 0x23790 | load | 4 | `abs 0x23850` | `-` |
| `strb` at 0x23798 | store | 1 | `abs 0x13205a` | `0x0` |
| `ldr` at 0x237a0 | load | 4 | `abs 0x2384c` | `-` |
| `ldrb` at 0x237a4 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldrb` at 0x237a8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x237c4 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x237c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x237d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x237d8 | store | 4 | `sp -0x20` | `ret(0xca584)` |
| `ldr` at 0x237e0 | load | 4 | `abs 0x2386c` | `-` |
| `ldrb` at 0x237e4 | load | 1 | `abs 0x132000` | `-` |
| `ldr` at 0x237f4 | load | 4 | `abs 0x2386c` | `-` |
| `str` at 0x23804 | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x23808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23814 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x23818 | store | 4 | `sp -0x20` | `ret(0x22e24)` |
| `ldr` at 0x23824 | load | 4 | `abs 0x23870` | `-` |
| `str` at 0x23834 | store | 4 | `sp -0x20` | `ret(0x22e24)` |
| `ldr` at 0x23838 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x23840 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x23840 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x23840 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x236a4 | `(unknown callee) 0xca7a0` | `0x0` | `[0x12d3ec]:1` | `r2` | `[0x12d3ec]:1` |
| `bl` at 0x236cc | `(unknown callee) 0xca584` | `0x0` | `[0x12d3ec]:1` | `0x70` | `0x0` |
| `mov` at 0x23730 | `(unknown callee) (unknown target)` | `[0x12d3ec]:1` | `?` | `[0x131fec]` | `?` |
| `bl` at 0x2374c | `(unknown callee) 0x1fc1c` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x23770 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x27f` | `0x0` |
| `mov` at 0x2378c | `(unknown callee) (unknown target)` | `0x13205b` | `0xee254` | `0x4` | `[0x131ff8]` |
| `bl` at 0x2379c | `(unknown callee) 0x23314` | `ret(unknown)` | `?` | `0x13205a` | `0x0` |
| `bl` at 0x237bc | `(unknown callee) 0xca584` | `0x0` | `[0x12d3ec]:1` | `0x70` | `[sp-21]:1` |
| `bl` at 0x237fc | `(unknown callee) 0x22e24` | `0x31` | `0x132000` | `0x0` | `[0x132000]:1` |
| `bl` at 0x2382c | `(unknown callee) 0x22e24` | `0x32` | `0xee348` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x23684 | `[0x131f8c]:1 != 0x0` | `[0x131f8c]:1 == 0x0` |
| `beq` at 0x236e0 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `beq` at 0x23744 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x237d0 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `beq` at 0x237ec | `[0x132000]:1 == 0x0` | `[0x132000]:1 != 0x0` |
| `beq` at 0x23810 | `ret(0x22e24) == 0x0` | `ret(0x22e24) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x23840 | `r0` | `?` |

## `displayStatus`

- `displayStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23874 to 0x23974.
  Established. Included byte assignments at `0x131fda` and `0x131fdc` write `0x2e` and zero (`strb`
  at 0x238c8, `strb` at 0x238ec). The word through `[sp-20] +0x8` receives `0x131fd9` (`str` at 0x23910). The unnamed `0xca70c` call carries `0x0`, `[0x12d3ec]:1`, `[sp-20]+16`, `[sp-20]+16`
  (`bl` at 0x23940). Status record layout meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23884 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x23888 | load | 4 | `abs 0x23958` | `-` |
| `ldrb` at 0x2388c | load | 1 | `abs 0x131f8c` | `-` |
| `ldr` at 0x23898 | load | 4 | `abs 0x2395c` | `-` |
| `str` at 0x2389c | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x238a4 | load | 4 | `abs 0x23960` | `-` |
| `ldr` at 0x238a8 | load | 4 | `abs 0x23964` | `-` |
| `ldrb` at 0x238ac | load | 1 | `abs 0x131fe3` | `-` |
| `strb` at 0x238bc | store | 1 | `abs 0x131fd9` | `?` |
| `ldr` at 0x238c0 | load | 4 | `abs 0x23960` | `-` |
| `strb` at 0x238c8 | store | 1 | `abs 0x131fda` | `0x2e` |
| `ldr` at 0x238cc | load | 4 | `abs 0x23960` | `-` |
| `ldr` at 0x238d0 | load | 4 | `abs 0x23964` | `-` |
| `ldrb` at 0x238d4 | load | 1 | `abs 0x131fe3` | `-` |
| `strb` at 0x238e0 | store | 1 | `abs 0x131fdb` | `?` |
| `ldr` at 0x238e4 | load | 4 | `abs 0x23960` | `-` |
| `strb` at 0x238ec | store | 1 | `abs 0x131fdc` | `0x0` |
| `ldr` at 0x238f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x238f4 | load | 4 | `abs 0x23968` | `-` |
| `str` at 0x238f8 | store | 4 | `[sp-20] +0x0` | `0x132000` |
| `ldr` at 0x238fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x23900 | load | 4 | `abs 0x2396c` | `-` |
| `str` at 0x23904 | store | 4 | `[sp-20] +0x4` | `0x132011` |
| `ldr` at 0x23908 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2390c | load | 4 | `abs 0x23960` | `-` |
| `str` at 0x23910 | store | 4 | `[sp-20] +0x8` | `0x131fd9` |
| `ldr` at 0x23914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x23918 | load | 4 | `abs 0x23964` | `-` |
| `ldrb` at 0x2391c | load | 1 | `abs 0x131fe3` | `-` |
| `strb` at 0x23920 | store | 1 | `[sp-20] +0xc` | `([0x131fe3]:1 & 0xff)` |
| `ldr` at 0x23924 | load | 4 | `abs 0x23970` | `-` |
| `ldrb` at 0x23928 | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x2392c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x23948 | store | 4 | `sp -0x18` | `ret(0xca70c)` |
| `ldr` at 0x2394c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x23954 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x23954 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x23954 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23940 | `(unknown callee) 0xca70c` | `0x0` | `[0x12d3ec]:1` | `[sp-20]+16` | `[sp-20]+16` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x23894 | `[0x131f8c]:1 != 0x0` | `[0x131f8c]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x23954 | `r0` | `?` |

## `panelProbe`

- `panelProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22b80 to 0x22ce4.
  Established. The byte at `0x131fe2` receives `0x10` on an included path (`strb` at 0x22c4c).
  Included word assignments at `0x131fec`, `0x131ff0`, `0x131ff4`, `0x131ff8` write `0x24138`,
  `0x24194`, `0x241bc`, `0x241f0` (`str` at 0x22c88, `str` at 0x22c94, `str` at 0x22ca0, `str` at 0x22cac). These stored values do not establish calls. Panel setup roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x22b94 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x22b98 | load | 4 | `abs 0x22cb8` | `-` |
| `ldrb` at 0x22b9c | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x22ba8 | load | 4 | `abs 0x22cbc` | `-` |
| `str` at 0x22bac | store | 4 | `sp -0x24` | `0xee25c` |
| `ldr` at 0x22bb4 | load | 4 | `abs 0x22cc0` | `-` |
| `ldrb` at 0x22bc4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x22bdc | store | 4 | `sp -0x18` | `ret(0xca70c)` |
| `ldr` at 0x22be0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22bec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22bf8 | load | 4 | `abs 0x22cc4` | `-` |
| `str` at 0x22bfc | store | 4 | `sp -0x24` | `0xee28c` |
| `str` at 0x22c00 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x22c08 | load | 4 | `abs 0x22cc0` | `-` |
| `ldrb` at 0x22c18 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x22c2c | load | 4 | `abs 0x22cb8` | `-` |
| `ldrb` at 0x22c30 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x22c34 | store | 1 | `abs 0x12d3ec` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x22c38 | load | 4 | `abs 0x22cc8` | `-` |
| `strb` at 0x22c40 | store | 1 | `abs 0x131f8d` | `0x1` |
| `ldr` at 0x22c44 | load | 4 | `abs 0x22ccc` | `-` |
| `strb` at 0x22c4c | store | 1 | `abs 0x131fe2` | `0x10` |
| `ldr` at 0x22c50 | load | 4 | `abs 0x22ccc` | `-` |
| `strb` at 0x22c58 | store | 1 | `abs 0x131fe3` | `0x0` |
| `ldr` at 0x22c5c | load | 4 | `abs 0x22ccc` | `-` |
| `strb` at 0x22c64 | store | 1 | `abs 0x131fe4` | `0x0` |
| `ldr` at 0x22c68 | load | 4 | `abs 0x22ccc` | `-` |
| `strb` at 0x22c70 | store | 1 | `abs 0x131fe5` | `0x0` |
| `ldr` at 0x22c74 | load | 4 | `abs 0x22ccc` | `-` |
| `ldr` at 0x22c78 | load | 4 | `abs 0x22cd0` | `-` |
| `str` at 0x22c7c | store | 4 | `abs 0x131fe8` | `0x131f8f` |
| `ldr` at 0x22c80 | load | 4 | `abs 0x22ccc` | `-` |
| `ldr` at 0x22c84 | load | 4 | `abs 0x22cd4` | `-` |
| `str` at 0x22c88 | store | 4 | `abs 0x131fec` | `0x24138` |
| `ldr` at 0x22c8c | load | 4 | `abs 0x22ccc` | `-` |
| `ldr` at 0x22c90 | load | 4 | `abs 0x22cd8` | `-` |
| `str` at 0x22c94 | store | 4 | `abs 0x131ff0` | `0x24194` |
| `ldr` at 0x22c98 | load | 4 | `abs 0x22ccc` | `-` |
| `ldr` at 0x22c9c | load | 4 | `abs 0x22cdc` | `-` |
| `str` at 0x22ca0 | store | 4 | `abs 0x131ff4` | `0x241bc` |
| `ldr` at 0x22ca4 | load | 4 | `abs 0x22ccc` | `-` |
| `ldr` at 0x22ca8 | load | 4 | `abs 0x22ce0` | `-` |
| `str` at 0x22cac | store | 4 | `abs 0x131ff8` | `0x241f0` |
| `ldm` at 0x22cb4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x22cb4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x22cb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22bc0 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0xb8` | `0x0` |
| `bl` at 0x22bd4 | `(unknown callee) 0xca70c` | `0x0` | `?` | `sp-25` | `?` |
| `bl` at 0x22bf0 | `(unknown callee) 0x1fc1c` | `ret(0xca70c)` | `?` | `?` | `ret(0xca70c)` |
| `bl` at 0x22c14 | `(unknown callee) 0x200a0` | `0x1` | `0xee274` | `0xbe` | `0x0` |
| `bl` at 0x22c24 | `(unknown callee) 0xc9a44` | `0x0` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x22ba4 | `[0x12d3ec]:1 == 0xff` | `[0x12d3ec]:1 != 0xff` |
| `beq` at 0x22be8 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x22cb4 | `r0` | `?` |

## `panelInt`

- `panelInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23f94 to 0x24138.
  Established. The equality `(r1 & 0xff) == [0x12d3ec]:1` takes `0x23ffc`, while inequality falls
  through (`beq` at 0x23fd4). The unnamed `0x37174` call carries `0x2`, unknown, unknown,
  `ret(0x1d444)` (`bl` at 0x24074). The unnamed `0x368ec` call carries `0x1`, `0x2`, unknown,
  `ret(0x375e4)` (`bl` at 0x240c4). Panel interrupt policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23fa4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x23fb0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x23fb8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x23fc0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x23fc4 | load | 4 | `abs 0x24110` | `-` |
| `ldrb` at 0x23fc8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x23fcc | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x23fd8 | load | 4 | `abs 0x24114` | `-` |
| `str` at 0x23fdc | store | 4 | `sp -0x24` | `0xee358` |
| `ldrb` at 0x23fe0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x23fe4 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0x23fec | load | 4 | `abs 0x24118` | `-` |
| `ldr` at 0x23ff0 | load | 4 | `abs 0x2411c` | `-` |
| `ldrb` at 0x23ffc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x2400c | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x24040 | load | 4 | `abs 0x24120` | `-` |
| `str` at 0x24044 | store | 4 | `sp -0x24` | `0xee380` |
| `ldr` at 0x2404c | load | 4 | `abs 0x24118` | `-` |
| `ldr` at 0x24050 | load | 4 | `abs 0x24124` | `-` |
| `ldr` at 0x24060 | load | 4 | `abs 0x24128` | `-` |
| `str` at 0x2407c | store | 4 | `sp -0x1c` | `ret(0x37174)` |
| `ldr` at 0x24080 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2408c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24098 | load | 4 | `abs 0x2412c` | `-` |
| `str` at 0x2409c | store | 4 | `sp -0x24` | `0xee3d0` |
| `str` at 0x240a0 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x240a8 | load | 4 | `abs 0x24118` | `-` |
| `ldr` at 0x240ac | load | 4 | `abs 0x24130` | `-` |
| `str` at 0x240cc | store | 4 | `sp -0x1c` | `ret(0x368ec)` |
| `ldr` at 0x240d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x240dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x240e8 | load | 4 | `abs 0x24134` | `-` |
| `str` at 0x240ec | store | 4 | `sp -0x24` | `0xee3e8` |
| `str` at 0x240f0 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x240f8 | load | 4 | `abs 0x24118` | `-` |
| `ldm` at 0x2410c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2410c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2410c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23ff8 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0x391` | `0x0` |
| `bl` at 0x2401c | `(unknown callee) 0x375e4` | `?` | `?` | `?` | `?` |
| `bl` at 0x24030 | `(unknown callee) 0x1d444` | `0x77` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x24058 | `(unknown callee) 0x200a0` | `0x6` | `0xee274` | `0x39d` | `0x0` |
| `bl` at 0x24068 | `(unknown callee) 0x22e24` | `0x32` | `0xee3bc` | `0x1` | `?` |
| `bl` at 0x24074 | `(unknown callee) 0x37174` | `0x2` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x24090 | `(unknown callee) 0x1fc1c` | `ret(0x37174)` | `?` | `?` | `ret(0x37174)` |
| `bl` at 0x240b4 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x3a1` | `0x0` |
| `bl` at 0x240c4 | `(unknown callee) 0x368ec` | `0x1` | `0x2` | `?` | `ret(0x375e4)` |
| `bl` at 0x240e0 | `(unknown callee) 0x1fc1c` | `ret(0x368ec)` | `?` | `?` | `ret(0x368ec)` |
| `bl` at 0x24104 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x3a4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23fd4 | `(r1 & 0xff) == [0x12d3ec]:1` | `(r1 & 0xff) != [0x12d3ec]:1` |
| `beq` at 0x24008 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x24018 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x24028 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `beq` at 0x2403c | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x24088 | `ret(0x37174) == 0x0` | `ret(0x37174) != 0x0` |
| `beq` at 0x240d8 | `ret(0x368ec) == 0x0` | `ret(0x368ec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2410c | `r0` | `?` |
