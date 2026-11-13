# ARM I/O, display and router controls in 1.44.0

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
Computed transfers leave unresolved edges in the I/O initialization routine, identified below. Tables cover known control-flow edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/io-display-router-controls-facts.txt) retain all tool sections.

## `l1ioInit`

- `l1ioInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a4d0 to 0x5ad40.
  Established. A zero `ret(0x1eeb0)` takes `0x5a5a4`, while nonzero falls through (`beq` at 0x5a544).
  The unnamed `0x1967c` call carries `0xf9a3c`, `0x802`, unknown, `ret(0x1eeb0)` (`bl` at 0x5a550).
  A signed nonnegative `ret(0x1967c)` takes `0x5a590`, while negative falls through (`bge` at 0x5a564).
  I/O initialization effects behind the computed transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a4e0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5a4e8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x5a4f0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x5a4f8 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x5a500 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x5a508 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x5a510 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x5a518 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x5a520 | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x5a528 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x5a52c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a548 | load | 4 | `abs 0x5acc8` | `-` |
| `ldr` at 0x5a54c | load | 4 | `abs 0x5accc` | `-` |
| `str` at 0x5a558 | store | 4 | `sp -0x6c` | `ret(0x1967c)` |
| `ldr` at 0x5a55c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x5a570 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0x5a57c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x5a584 | load | 4 | `abs 0x5acd0` | `-` |
| `ldr` at 0x5a590 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x5a59c | store | 4 | `sp -0x70` | `0x0` |
| `ldrls` at 0x5a5b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a720 | load | 4 | `abs 0x5acf4` | `-` |
| `str` at 0x5a724 | store | 4 | `sp -0x80` | `0xf9b98` |
| `ldr` at 0x5a728 | load | 4 | `abs 0x5acf8` | `-` |
| `str` at 0x5a72c | store | 4 | `sp -0x7c` | `0xf9a30` |
| `ldr` at 0x5a734 | load | 4 | `abs 0x5acfc` | `-` |
| `str` at 0x5a748 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x5acbc | load | 4 | `sp -0x70` | `-` |
| `ldm` at 0x5acc4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5acc4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5acc4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5acc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5a538 | `(unknown callee) 0x1eeb0` | `r0` | `r1` | `r2` | `[sp-24]` |
| `bl` at 0x5a550 | `(unknown callee) 0x1967c` | `0xf9a3c` | `0x802` | `?` | `ret(0x1eeb0)` |
| `bl` at 0x5a568 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0x5a580 | `(unknown callee) 0x1931c` | `[sp-108]` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0x5a588 | `(unknown callee) 0x1e958` | `0xf9a4c` | `?` | `?` | `?` |
| `bl` at 0x5a594 | `(unknown callee) 0x1931c` | `?` | `?` | `?` | `?` |
| `bl` at 0x5a5a4 | `(unknown callee) 0x488cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x5a740 | `(unknown callee) 0x200a0` | `0x4` | `0xf9bd0` | `0x9d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a534 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x5a544 | `ret(0x1eeb0) == 0x0` | `ret(0x1eeb0) != 0x0` |
| `bge` at 0x5a564 | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x5a578 | `[ret(0x193d0)] != 0x13` | `[ret(0x193d0)] == 0x13` |
| `ldrls` at 0x5a5b8 | `? <= 0x33 (unsigned, 32-bit)` | `? > 0x33 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5acc4 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5a5b8).

## `displayInt`

- `displayInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23974 to 0x23f94.
  Established. Equality `(r1 & 0xff) == [0x12d3ec]:1` takes `0x239dc`, while inequality falls through (`beq` at 0x239b4).
  Included unnamed calls carry `0x2`, `0xff`, unknown, unknown to `0x47ed0` and `0x2` followed by three unknown candidates to `0x47ad8` (`bl` at 0x23d30, `bl` at 0x23e7c).
  Display interrupt action selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x23984 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x23990 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x23998 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x239a0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x239a4 | load | 4 | `abs 0x23f18` | `-` |
| `ldrb` at 0x239a8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x239ac | load | 1 | `abs 0x12d3ec` | `-` |
| `ldr` at 0x239b8 | load | 4 | `abs 0x23f1c` | `-` |
| `str` at 0x239bc | store | 4 | `sp -0x24` | `0xee358` |
| `ldrb` at 0x239c0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x239c4 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0x239cc | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x239d0 | load | 4 | `abs 0x23f24` | `-` |
| `ldr` at 0x239dc | load | 4 | `abs 0x23f28` | `-` |
| `ldrb` at 0x239e0 | load | 1 | `abs 0x131fe4` | `-` |
| `ldrb` at 0x239f0 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x239f8 | store | 1 | `sp -0x17` | `?` |
| `ldrb` at 0x239fc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x23a0c | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x23a40 | load | 4 | `abs 0x23f2c` | `-` |
| `str` at 0x23a44 | store | 4 | `sp -0x24` | `0xee380` |
| `ldr` at 0x23a4c | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23a50 | load | 4 | `abs 0x23f30` | `-` |
| `ldr` at 0x23a60 | load | 4 | `abs 0x23f34` | `-` |
| `str` at 0x23a7c | store | 4 | `sp -0x1c` | `ret(0x37174)` |
| `ldr` at 0x23a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23a8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23a98 | load | 4 | `abs 0x23f38` | `-` |
| `str` at 0x23a9c | store | 4 | `sp -0x24` | `0xee3d0` |
| `str` at 0x23aa0 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23aa8 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23aac | load | 4 | `abs 0x23f3c` | `-` |
| `str` at 0x23acc | store | 4 | `sp -0x1c` | `ret(0x368ec)` |
| `ldr` at 0x23ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23adc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23ae8 | load | 4 | `abs 0x23f40` | `-` |
| `str` at 0x23aec | store | 4 | `sp -0x24` | `0xee3e8` |
| `str` at 0x23af0 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23af8 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23afc | load | 4 | `abs 0x23f44` | `-` |
| `ldrb` at 0x23b08 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x23b18 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x23ca8 | load | 4 | `abs 0x23f48` | `-` |
| `str` at 0x23cac | store | 4 | `sp -0x24` | `0xee3fc` |
| `ldr` at 0x23cb4 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23cb8 | load | 4 | `abs 0x23f4c` | `-` |
| `ldr` at 0x23cc8 | load | 4 | `abs 0x23f50` | `-` |
| `ldr` at 0x23cdc | load | 4 | `abs 0x23f54` | `-` |
| `str` at 0x23cec | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x23cf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23cfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23d08 | load | 4 | `abs 0x23f58` | `-` |
| `str` at 0x23d0c | store | 4 | `sp -0x24` | `0xee310` |
| `str` at 0x23d10 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23d18 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23d1c | load | 4 | `abs 0x23f5c` | `-` |
| `str` at 0x23d38 | store | 4 | `sp -0x1c` | `ret(0x47ed0)` |
| `ldr` at 0x23d3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23d48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23d54 | load | 4 | `abs 0x23f60` | `-` |
| `str` at 0x23d58 | store | 4 | `sp -0x24` | `0xee44c` |
| `str` at 0x23d5c | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23d64 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23d68 | load | 4 | `abs 0x23f64` | `-` |
| `ldr` at 0x23d78 | load | 4 | `abs 0x23f68` | `-` |
| `str` at 0x23d88 | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x23d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23d98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23da4 | load | 4 | `abs 0x23f58` | `-` |
| `str` at 0x23da8 | store | 4 | `sp -0x24` | `0xee310` |
| `str` at 0x23dac | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23db4 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23db8 | load | 4 | `abs 0x23f6c` | `-` |
| `ldrb` at 0x23dc4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x23dd4 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x23df8 | load | 4 | `abs 0x23f70` | `-` |
| `str` at 0x23dfc | store | 4 | `sp -0x24` | `0xee468` |
| `ldr` at 0x23e04 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23e08 | load | 4 | `abs 0x23f74` | `-` |
| `ldr` at 0x23e18 | load | 4 | `abs 0x23f78` | `-` |
| `ldr` at 0x23e2c | load | 4 | `abs 0x23f7c` | `-` |
| `str` at 0x23e3c | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x23e40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23e58 | load | 4 | `abs 0x23f58` | `-` |
| `str` at 0x23e5c | store | 4 | `sp -0x24` | `0xee310` |
| `str` at 0x23e60 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23e68 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23e6c | load | 4 | `abs 0x23f80` | `-` |
| `str` at 0x23e84 | store | 4 | `sp -0x1c` | `ret(0x47ad8)` |
| `ldr` at 0x23e88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23ea0 | load | 4 | `abs 0x23f84` | `-` |
| `str` at 0x23ea4 | store | 4 | `sp -0x24` | `0xee4c0` |
| `str` at 0x23ea8 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23eb0 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23eb4 | load | 4 | `abs 0x23f88` | `-` |
| `ldr` at 0x23ec4 | load | 4 | `abs 0x23f8c` | `-` |
| `str` at 0x23ed4 | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x23ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23ee4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x23ef0 | load | 4 | `abs 0x23f58` | `-` |
| `str` at 0x23ef4 | store | 4 | `sp -0x24` | `0xee310` |
| `str` at 0x23ef8 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x23f00 | load | 4 | `abs 0x23f20` | `-` |
| `ldr` at 0x23f04 | load | 4 | `abs 0x23f90` | `-` |
| `ldm` at 0x23f14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x23f14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x23f14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x239d8 | `(unknown callee) 0x200a0` | `0x9` | `0xee274` | `0x2de` | `0x0` |
| `bl` at 0x23a1c | `(unknown callee) 0x375e4` | `?` | `?` | `?` | `?` |
| `bl` at 0x23a30 | `(unknown callee) 0x1d444` | `0x77` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x23a58 | `(unknown callee) 0x200a0` | `0x6` | `0xee274` | `0x2fe` | `0x0` |
| `bl` at 0x23a68 | `(unknown callee) 0x22e24` | `0x32` | `0xee3bc` | `0x1` | `?` |
| `bl` at 0x23a74 | `(unknown callee) 0x37174` | `0x2` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x23a90 | `(unknown callee) 0x1fc1c` | `ret(0x37174)` | `?` | `?` | `ret(0x37174)` |
| `bl` at 0x23ab4 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x303` | `0x0` |
| `bl` at 0x23ac4 | `(unknown callee) 0x368ec` | `0x1` | `0x2` | `?` | `ret(0x375e4)` |
| `bl` at 0x23ae0 | `(unknown callee) 0x1fc1c` | `ret(0x368ec)` | `?` | `?` | `ret(0x368ec)` |
| `bl` at 0x23b04 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x307` | `0x0` |
| `bl` at 0x23b28 | `(unknown callee) 0x488cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x23b3c | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23b50 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23b64 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23b78 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23b8c | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23ba0 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23bb4 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23bc8 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23bdc | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23bf0 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c04 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c18 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c2c | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c40 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c54 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c68 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c7c | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x23c98 | `(unknown callee) 0x1d444` | `0x78` | `?` | `?` | `?` |
| `bl` at 0x23cc0 | `(unknown callee) 0x200a0` | `0x6` | `0xee274` | `0x313` | `0x0` |
| `bl` at 0x23cd0 | `(unknown callee) 0x22e24` | `0x32` | `0xee430` | `0x1` | `?` |
| `bl` at 0x23ce4 | `(unknown callee) 0x22e24` | `0x32` | `0xee444` | `0x1` | `ret(0x1d444)` |
| `bl` at 0x23d00 | `(unknown callee) 0x1fc1c` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x23d24 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x317` | `0x0` |
| `bl` at 0x23d30 | `(unknown callee) 0x47ed0` | `0x2` | `0xff` | `?` | `?` |
| `bl` at 0x23d4c | `(unknown callee) 0x1fc1c` | `ret(0x47ed0)` | `?` | `?` | `ret(0x47ed0)` |
| `bl` at 0x23d70 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x31b` | `0x0` |
| `bl` at 0x23d80 | `(unknown callee) 0x22e24` | `0x32` | `0xee45c` | `0x1` | `?` |
| `bl` at 0x23d9c | `(unknown callee) 0x1fc1c` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x23dc0 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x31e` | `0x0` |
| `bl` at 0x23de8 | `(unknown callee) 0x1d444` | `0x79` | `?` | `?` | `?` |
| `bl` at 0x23e10 | `(unknown callee) 0x200a0` | `0x6` | `0xee274` | `0x329` | `0x0` |
| `bl` at 0x23e20 | `(unknown callee) 0x22e24` | `0x32` | `0xee4a0` | `0x1` | `?` |
| `bl` at 0x23e34 | `(unknown callee) 0x22e24` | `0x32` | `0xee4b4` | `0x1` | `ret(0x1d444)` |
| `bl` at 0x23e50 | `(unknown callee) 0x1fc1c` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x23e74 | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x32e` | `0x0` |
| `bl` at 0x23e7c | `(unknown callee) 0x47ad8` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x23e98 | `(unknown callee) 0x1fc1c` | `ret(0x47ad8)` | `?` | `?` | `ret(0x47ad8)` |
| `bl` at 0x23ebc | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x332` | `0x0` |
| `bl` at 0x23ecc | `(unknown callee) 0x22e24` | `0x32` | `0xee4d0` | `0x1` | `?` |
| `bl` at 0x23ee8 | `(unknown callee) 0x1fc1c` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x23f0c | `(unknown callee) 0x200a0` | `0x3` | `0xee274` | `0x335` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x239b4 | `(r1 & 0xff) == [0x12d3ec]:1` | `(r1 & 0xff) != [0x12d3ec]:1` |
| `beq` at 0x239ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23a08 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23a18 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23a28 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `beq` at 0x23a3c | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x23a88 | `ret(0x37174) == 0x0` | `ret(0x37174) != 0x0` |
| `beq` at 0x23ad8 | `ret(0x368ec) == 0x0` | `ret(0x368ec) != 0x0` |
| `beq` at 0x23b14 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23b24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23b38 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x23b4c | `? == 0x33` | `? != 0x33` |
| `beq` at 0x23b60 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x23b74 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x23b88 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x23b9c | `? == 0x36` | `? != 0x36` |
| `beq` at 0x23bb0 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x23bc4 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x23bd8 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x23bec | `? == 0x30` | `? != 0x30` |
| `beq` at 0x23c00 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x23c14 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x23c28 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x23c3c | `? == 0x42` | `? != 0x42` |
| `beq` at 0x23c50 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x23c64 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x23c78 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x23c8c | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x23ca4 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x23cf8 | `ret(0x22e24) == 0x0` | `ret(0x22e24) != 0x0` |
| `beq` at 0x23d44 | `ret(0x47ed0) == 0x0` | `ret(0x47ed0) != 0x0` |
| `beq` at 0x23d94 | `ret(0x22e24) == 0x0` | `ret(0x22e24) != 0x0` |
| `beq` at 0x23dd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23de0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x23df4 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x23e48 | `ret(0x22e24) == 0x0` | `ret(0x22e24) != 0x0` |
| `beq` at 0x23e90 | `ret(0x47ad8) == 0x0` | `ret(0x47ad8) != 0x0` |
| `beq` at 0x23ee0 | `ret(0x22e24) == 0x0` | `ret(0x22e24) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x23f14 | `r0` | `?` |

## `pcf2119_Setup`

- `pcf2119_Setup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce308 to 0xce32c.
  Established. A word store writes `0xce260` to `0x12d8e4` (`str` at 0xce31c).
  The return record preserves entry `r0` (`ldm` at 0xce320).
  Display-controller setup callback meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xce314 | load | 4 | `abs 0xce324` | `-` |
| `ldr` at 0xce318 | load | 4 | `abs 0xce328` | `-` |
| `str` at 0xce31c | store | 4 | `abs 0x12d8e4` | `0xce260` |
| `ldm` at 0xce320 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xce320 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xce320 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xce320 | `r0` | `r0` |

## `NL4R_set_link_enable`

- `NL4R_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29784 to 0x2980c.
  Established. A nonzero `[0x13208d]:1` takes `0x297dc`, while zero falls through (`bne` at 0x297cc).
  The unnamed `0x2ab3c` call carries entry `r0`, entry `r1`, entry `r2`, `[0x13208d]:1` (`bl` at 0x297e8).
  NL4R link enable state meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29794 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29798 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2979c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x297a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x297b8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x297c0 | load | 4 | `abs 0x29808` | `-` |
| `ldrb` at 0x297c4 | load | 1 | `abs 0x13208d` | `-` |
| `str` at 0x297d4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x297dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x297e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x297e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x297f0 | store | 4 | `sp -0x20` | `ret(0x2ab3c)` |
| `ldr` at 0x297f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x297f8 | store | 4 | `sp -0x24` | `ret(0x2ab3c)` |
| `ldr` at 0x297fc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x29804 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29804 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x297e8 | `(unknown callee) 0x2ab3c` | `r0` | `r1` | `r2` | `[0x13208d]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x297b0 | `? == 0x6` | `? != 0x6` |
| `bne` at 0x297cc | `[0x13208d]:1 != 0x0` | `[0x13208d]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29804 | `r0` | `?` |

## `NL4RTarget`

- `NL4RTarget`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x291a4 to 0x291e0.
  Established. A word read addresses `0x132090` (`ldr` at 0x291c8).
  A later word read has an unknown effective target (`ldr` at 0x291d0).
  NL4R target index bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x291b8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x291bc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x291c4 | load | 4 | `abs 0x291dc` | `-` |
| `ldr` at 0x291c8 | load | 4 | `abs 0x132090` | `-` |
| `ldr` at 0x291d0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x291d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x291d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x291d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x291d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x291d8 | `r0` | `?` |

## `nl4r_init_icntlr`

- `nl4r_init_icntlr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29d80 to 0x2a22c.
  Established. The unnamed `0xe0540` call carries `[sp-20]`, `0x2`, `0x6`, `0x0` (`bl` at 0x29e98).
  A signed nonnegative `ret(0xe0540)` takes `0x29ebc`, while negative falls through (`bge` at 0x29eac).
  NL4R interrupt-controller field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29d90 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29d94 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x29d98 | load | 4 | `abs 0x2a218` | `-` |
| `ldr` at 0x29d9c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x29da8 | load | 4 | `abs 0x2a21c` | `-` |
| `ldr` at 0x29dac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29db0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29db8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29dcc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x29dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29dfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29e24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29e60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29e70 | load | 4 | `abs 0x2a218` | `-` |
| `ldr` at 0x29e74 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x29e80 | load | 4 | `abs 0x2a220` | `-` |
| `ldr` at 0x29e88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29ea0 | store | 4 | `sp -0x20` | `ret(0xe0540)` |
| `ldr` at 0x29ea4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x29eb0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x29eb4 | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `str` at 0x29ec0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x29ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29edc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29ef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29f04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29f18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29f2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29f40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29f64 | load | 4 | `abs 0x2a218` | `-` |
| `ldr` at 0x29f68 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x29f74 | load | 4 | `abs 0x2a224` | `-` |
| `ldr` at 0x29f7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29f94 | store | 4 | `sp -0x20` | `ret(0xe0540)` |
| `ldr` at 0x29f98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x29fa4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x29fa8 | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `ldr` at 0x29fb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29fc4 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x29fcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29fe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a01c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a058 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a06c | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x2a074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a09c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a0b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a0c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a0d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a0ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a100 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a114 | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x2a11c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a16c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a1a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a1b8 | load | 4 | `abs 0x2a218` | `-` |
| `ldr` at 0x2a1bc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x2a1c8 | load | 4 | `abs 0x2a228` | `-` |
| `ldr` at 0x2a1d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a1e8 | store | 4 | `sp -0x20` | `ret(0xe0540)` |
| `ldr` at 0x2a1ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2a1f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2a1fc | store | 4 | `sp -0x2c` | `ret(0xe0540)` |
| `str` at 0x2a208 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x2a20c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x2a214 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2a214 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2a214 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29db4 | `(unknown callee) 0x221f0` | `0xf1300` | `r0` | `r1` | `[0x29737c]` |
| `bl` at 0x29de0 | `(unknown callee) 0xdfc34` | `?` | `0x6` | `0x59` | `sp-28` |
| `bl` at 0x29df4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x4f` | `sp-28` |
| `bl` at 0x29e08 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x45` | `sp-28` |
| `bl` at 0x29e1c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x3b` | `sp-28` |
| `bl` at 0x29e30 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x31` | `sp-28` |
| `bl` at 0x29e44 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x27` | `sp-28` |
| `bl` at 0x29e58 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x1d` | `sp-28` |
| `bl` at 0x29e6c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x13` | `sp-28` |
| `bl` at 0x29e84 | `(unknown callee) 0x221f0` | `0xf132c` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x29e98 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `0x6` | `0x0` |
| `bl` at 0x29ed4 | `(unknown callee) 0xdfc34` | `?` | `0x6` | `0x59` | `sp-28` |
| `bl` at 0x29ee8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x4f` | `sp-28` |
| `bl` at 0x29efc | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x45` | `sp-28` |
| `bl` at 0x29f10 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x3b` | `sp-28` |
| `bl` at 0x29f24 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x31` | `sp-28` |
| `bl` at 0x29f38 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x27` | `sp-28` |
| `bl` at 0x29f4c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x1d` | `sp-28` |
| `bl` at 0x29f60 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x13` | `sp-28` |
| `bl` at 0x29f78 | `(unknown callee) 0x221f0` | `0xf1364` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x29f8c | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `0x6` | `0x0` |
| `bl` at 0x29fd8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x5c` | `sp-28` |
| `bl` at 0x29fec | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x52` | `sp-28` |
| `bl` at 0x2a000 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x48` | `sp-28` |
| `bl` at 0x2a014 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x3e` | `sp-28` |
| `bl` at 0x2a028 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x34` | `sp-28` |
| `bl` at 0x2a03c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x2a` | `sp-28` |
| `bl` at 0x2a050 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x20` | `sp-28` |
| `bl` at 0x2a064 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x16` | `sp-28` |
| `bl` at 0x2a080 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x5b` | `sp-28` |
| `bl` at 0x2a094 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x51` | `sp-28` |
| `bl` at 0x2a0a8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x47` | `sp-28` |
| `bl` at 0x2a0bc | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x3d` | `sp-28` |
| `bl` at 0x2a0d0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x33` | `sp-28` |
| `bl` at 0x2a0e4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x29` | `sp-28` |
| `bl` at 0x2a0f8 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x1f` | `sp-28` |
| `bl` at 0x2a10c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x15` | `sp-28` |
| `bl` at 0x2a128 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x5a` | `sp-28` |
| `bl` at 0x2a13c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x50` | `sp-28` |
| `bl` at 0x2a150 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x46` | `sp-28` |
| `bl` at 0x2a164 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x3c` | `sp-28` |
| `bl` at 0x2a178 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x32` | `sp-28` |
| `bl` at 0x2a18c | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x28` | `sp-28` |
| `bl` at 0x2a1a0 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x1e` | `sp-28` |
| `bl` at 0x2a1b4 | `(unknown callee) 0xdfc34` | `[sp-20]` | `0x6` | `0x14` | `sp-28` |
| `bl` at 0x2a1cc | `(unknown callee) 0x221f0` | `0xf139c` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x2a1e0 | `(unknown callee) 0xe0540` | `[sp-20]` | `0x2` | `0x6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x29da4 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x29dc4 | `? == 0x0` | `? != 0x0` |
| `bls` at 0x29e7c | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x29eac | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bls` at 0x29f70 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x29fa0 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0x29fbc | `? == 0x0` | `? != 0x0` |
| `bls` at 0x2a1c4 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a1f4 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2a214 | `r0` | `?` |

## `IP57_Get_Cfg`

- `IP57_Get_Cfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x578d0 to 0x57934.
  Established. A nonzero `[0x132560]` takes `0x57900`, while zero falls through (`bne` at 0x578f0).
  A word store copies `[0x132560]` to entry `r0 +0x0` (`str` at 0x5790c).
  Another word read addresses `0x13255c` (`ldr` at 0x57914).
  IP57 configuration result meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x578e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x578e4 | load | 4 | `abs 0x57928` | `-` |
| `ldr` at 0x578e8 | load | 4 | `abs 0x132560` | `-` |
| `ldr` at 0x578f4 | load | 4 | `abs 0x5792c` | `-` |
| `str` at 0x578f8 | store | 4 | `sp -0x18` | `0xfffff510` |
| `ldr` at 0x57900 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57904 | load | 4 | `abs 0x57928` | `-` |
| `ldr` at 0x57908 | load | 4 | `abs 0x132560` | `-` |
| `str` at 0x5790c | store | 4 | `r0 +0x0` | `[0x132560]` |
| `ldr` at 0x57910 | load | 4 | `abs 0x57930` | `-` |
| `ldr` at 0x57914 | load | 4 | `abs 0x13255c` | `-` |
| `str` at 0x57918 | store | 4 | `sp -0x18` | `[0x13255c]` |
| `ldr` at 0x5791c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x57924 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x57924 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x57924 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x578f0 | `[0x132560] != 0x0` | `[0x132560] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x57924 | `r0` | `?` |

## `IP57_CpuPresent`

- `IP57_CpuPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x56824 to 0x568a0.
  Established. The unnamed `0x1d444` call carries `0x7b`, entry `r1`, entry `r2`, unknown (`bl` at 0x56858).
  Inequality `ret(0x1d444) != 0x1` takes `0x56874`, while equality falls through (`bne` at 0x56864).
  IP57 CPU presence bitmap encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x56838 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5683c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x56844 | store | 1 | `sp -0x12` | `?` |
| `ldrb` at 0x56848 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x56850 | store | 1 | `sp -0x13` | `?` |
| `str` at 0x5686c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x56874 | load | 4 | `abs 0x5689c` | `-` |
| `ldrb` at 0x56878 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x5687c | load | 1 | `[sp-18]:1 +0x13256c` | `-` |
| `ldrb` at 0x56880 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x5688c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x56890 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x56898 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x56898 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x56898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x56858 | `(unknown callee) 0x1d444` | `0x7b` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x56864 | `ret(0x1d444) != 0x1` | `ret(0x1d444) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x56898 | `r0` | `?` |

## `pcixcmdInit`

- `pcixcmdInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x97fac to 0x9802c.
  Established. Included word stores write `0x4fa44` to `0x1338f4`, `0x4fd48` to `0x1338f8`, `0x50004` to `0x1338fc` and `0x1` to `0x133a2c` (`str` at 0x97fdc, `str` at 0x97fe8, `str` at 0x97ff4, `str` at 0x98000).
  A nonzero `[0x133a2c]` takes `0x98004`, while zero falls through (`bne` at 0x97fc4).
  PCI-X command callback roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x97fb8 | load | 4 | `abs 0x9800c` | `-` |
| `ldr` at 0x97fbc | load | 4 | `abs 0x133a2c` | `-` |
| `ldr` at 0x97fc8 | load | 4 | `abs 0x98010` | `-` |
| `ldr` at 0x97fd4 | load | 4 | `abs 0x98014` | `-` |
| `ldr` at 0x97fd8 | load | 4 | `abs 0x98018` | `-` |
| `str` at 0x97fdc | store | 4 | `abs 0x1338f4` | `0x4fa44` |
| `ldr` at 0x97fe0 | load | 4 | `abs 0x9801c` | `-` |
| `ldr` at 0x97fe4 | load | 4 | `abs 0x98020` | `-` |
| `str` at 0x97fe8 | store | 4 | `abs 0x1338f8` | `0x4fd48` |
| `ldr` at 0x97fec | load | 4 | `abs 0x98024` | `-` |
| `ldr` at 0x97ff0 | load | 4 | `abs 0x98028` | `-` |
| `str` at 0x97ff4 | store | 4 | `abs 0x1338fc` | `0x50004` |
| `ldr` at 0x97ff8 | load | 4 | `abs 0x9800c` | `-` |
| `str` at 0x98000 | store | 4 | `abs 0x133a2c` | `0x1` |
| `ldm` at 0x98008 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x98008 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x98008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x97fd0 | `(unknown callee) 0xaec8c` | `0x110f68` | `0x19` | `r2` | `[0x133a2c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x97fc4 | `[0x133a2c] != 0x0` | `[0x133a2c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x98008 | `r0` | `?` |

## `pcixOverrideSpeed`

- `pcixOverrideSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5183c to 0x519a8.
  Established. The unnamed `0x1b5f0` call carries `([sp-22]:1-1 & 0xff)+170`, `[sp-30]:1`, `[sp-30]:1`, `([sp-22]:1-1 & 0xff)+170` (`bl` at 0x51984).
  A nonzero `[0x1324a0]` takes `0x518b8`, while zero falls through (`bne` at 0x51888).
  PCI-X override speed codes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5184c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x51858 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x51860 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x51868 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x5186c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x51870 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x51878 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x5187c | load | 4 | `abs 0x5199c` | `-` |
| `ldr` at 0x51880 | load | 4 | `abs 0x1324a0` | `-` |
| `ldrb` at 0x5188c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x5189c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x518a4 | load | 4 | `abs 0x519a0` | `-` |
| `str` at 0x518a8 | store | 4 | `sp -0x28` | `0xfffff703` |
| `ldr` at 0x518ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x518b0 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x518b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x518cc | load | 4 | `abs 0x519a4` | `-` |
| `ldr` at 0x518d0 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x518d8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x518dc | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x518ec | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x518f0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x518f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x518f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x51904 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5190c | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x5191c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x51924 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldrb` at 0x5192c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x51938 | load | 4 | `abs 0x519a0` | `-` |
| `str` at 0x5193c | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x51944 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x5194c | store | 1 | `sp -0x16` | `([sp-22]:1-1 & 0xff)` |
| `ldrb` at 0x51950 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x51954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51958 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x51964 | load | 4 | `abs 0x519a0` | `-` |
| `str` at 0x51968 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x51970 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x51978 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x5198c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x51990 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x51998 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x51998 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x51998 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51918 | `(unknown callee) 0x1b5f0` | `?` | `[sp-30]:1` | `[sp-30]:1` | `?` |
| `bl` at 0x51984 | `(unknown callee) 0x1b5f0` | `([sp-22]:1-1 & 0xff)+170` | `[sp-30]:1` | `[sp-30]:1` | `([sp-22]:1-1 & 0xff)+170` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51888 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x51894 | `(r2 & 0xff) != 0xff` | `(r2 & 0xff) == 0xff` |
| `bne` at 0x518e4 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bge` at 0x51900 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x51934 | `[sp-22]:1 != 0x0` | `[sp-22]:1 == 0x0` |
| `ble` at 0x51960 | `([sp-22]:1-1 & 0xff) <= [[sp-28]+8] (signed, 32-bit)` | `([sp-22]:1-1 & 0xff) > [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x51998 | `r0` | `?` |

## `pcixStatus`

- `pcixStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5057c to 0x506f0.
  Established. The unnamed `0x53658` call carries `[sp-20]`, `ret(0x53100)`, `[sp-28]`, `ret(0x53100)` (`bl` at 0x50638).
  An indirect call with unknown target carries `[sp-20]`, `ret(0x53024)`, `[sp]`, unknown (`mov` at 0x506c8).
  PCI-X status callback output layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5058c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x50590 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x50598 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x505a0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x505a4 | load | 4 | `abs 0x506e0` | `-` |
| `ldr` at 0x505a8 | load | 4 | `abs 0x1324a0` | `-` |
| `ldr` at 0x505b4 | load | 4 | `abs 0x506e4` | `-` |
| `str` at 0x505b8 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x505c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x505d4 | load | 4 | `abs 0x506e8` | `-` |
| `ldr` at 0x505d8 | load | 4 | `abs 0x13249c` | `-` |
| `str` at 0x505e0 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x505e4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x505e8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x505ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x505fc | store | 4 | `sp -0x24` | `ret(0x53100)` |
| `ldr` at 0x50600 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5060c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x50610 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x50614 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50624 | store | 4 | `sp -0x30` | `ret(0x532e0)` |
| `ldr` at 0x5062c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50630 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50634 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x50640 | store | 4 | `sp -0x2c` | `ret(0x53658)` |
| `ldr` at 0x50644 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50650 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50654 | store | 4 | `sp -0x30` | `ret(0x53658)` |
| `ldrb` at 0x5065c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x50660 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50670 | store | 4 | `sp -0x28` | `ret(0x53024)` |
| `ldr` at 0x50674 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50680 | load | 4 | `abs 0x506e4` | `-` |
| `str` at 0x50684 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x5068c | load | 4 | `abs 0x506ec` | `-` |
| `ldr` at 0x50690 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x50694 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x506b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x506b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x506bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x506c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x506d0 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0x506d4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x506dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x506dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x506dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x505f4 | `(unknown callee) 0x53100` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x5061c | `(unknown callee) 0x532e0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x50638 | `(unknown callee) 0x53658` | `[sp-20]` | `ret(0x53100)` | `[sp-28]` | `ret(0x53100)` |
| `bl` at 0x50668 | `(unknown callee) 0x53024` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `mov` at 0x506c8 | `(unknown callee) (unknown target)` | `[sp-20]` | `ret(0x53024)` | `[sp]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x505b0 | `[0x1324a0] != 0x0` | `[0x1324a0] == 0x0` |
| `bne` at 0x50608 | `ret(0x53100) != 0x0` | `ret(0x53100) == 0x0` |
| `beq` at 0x5064c | `ret(0x53658) == 0x0` | `ret(0x53658) != 0x0` |
| `bne` at 0x5067c | `ret(0x53024) != 0x0` | `ret(0x53024) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x506dc | `r0` | `?` |

## `promIceEnable`

- `promIceEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3eab8 to 0x3eb8c.
  Established. Inequality `(r0 & 0xff) != 0xff` takes `0x3eb68`, while equality falls through (`bne` at 0x3eae0).
  An included unnamed `0x3ec60` call carries `(r0 & 0xff)`, entry `r1`, entry `r2`, `(r0 & 0xff)` (`bl` at 0x3eb70).
  PROM enable command effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3eacc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x3ead4 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x3ead8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x3eae8 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x3eaec | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3eaf0 | load | 4 | `abs 0x3eb88` | `-` |
| `ldr` at 0x3eaf4 | load | 4 | `abs 0x29972c` | `-` |
| `ldrb` at 0x3eb00 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x3eb18 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x3eb28 | store | 4 | `sp -0x1c` | `ret(0x3ec60)` |
| `ldr` at 0x3eb2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eb38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3eb48 | store | 4 | `sp -0x18` | `ret(0x3ec60)` |
| `ldrb` at 0x3eb4c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x3eb54 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x3eb5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3eb60 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x3eb68 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3eb78 | store | 4 | `sp -0x24` | `ret(0x3ec60)` |
| `ldr` at 0x3eb7c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x3eb84 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3eb84 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3eb84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3eb08 | `(unknown callee) 0x3e93c` | `?` | `?` | `[0x29972c]` | `?` |
| `bl` at 0x3eb20 | `(unknown callee) 0x3ec60` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x3eb70 | `(unknown callee) 0x3ec60` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3eae0 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `bge` at 0x3eafc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3eb14 | `ret(0x3e93c) == 0x0` | `ret(0x3e93c) != 0x0` |
| `beq` at 0x3eb34 | `ret(0x3ec60) == 0x0` | `ret(0x3ec60) != 0x0` |
| `bne` at 0x3eb40 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3eb84 | `r0` | `?` |

## `promIceCount`

- `promIceCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e91c to 0x3e93c.
  Established. A word read addresses `0x29972c` (`ldr` at 0x3e92c).
  The integer return candidate is `[0x29972c]` (`ldm` at 0x3e934).
  PROM count unit remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3e928 | load | 4 | `abs 0x3e938` | `-` |
| `ldr` at 0x3e92c | load | 4 | `abs 0x29972c` | `-` |
| `ldm` at 0x3e934 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3e934 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3e934 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3e934 | `r0` | `[0x29972c]` |

## `extIntCount`

- `extIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbea4 to 0xbbf34.
  Established. The unsigned comparison `(r0 & 0xff) < [0x2972b4]:1` takes `0xbbef4`, while greater or equal falls through (`blo` at 0xbbecc).
  A word read addresses `0x2972b0` (`ldr` at 0xbbf0c).
  A later word read has an unknown effective target (`ldr` at 0xbbf14).
  External-interrupt counter interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbbeb8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbbebc | load | 4 | `abs 0xbbf24` | `-` |
| `ldrb` at 0xbbec0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbbec4 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbed0 | load | 4 | `abs 0xbbf28` | `-` |
| `str` at 0xbbed4 | store | 4 | `sp -0x1c` | `0x11ef64` |
| `ldrb` at 0xbbed8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbbedc | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xbbee4 | load | 4 | `abs 0xbbf2c` | `-` |
| `ldrb` at 0xbbef4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbbf08 | load | 4 | `abs 0xbbf30` | `-` |
| `ldr` at 0xbbf0c | load | 4 | `abs 0x2972b0` | `-` |
| `ldr` at 0xbbf14 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbbf20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbf20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbf20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbef0 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0xd2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbbecc | `(r0 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbf20 | `r0` | `?` |

## `extIntName`

- `extIntName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbf34 to 0xbbfc4.
  Established. A byte read addresses `0x2972b4` (`ldrb` at 0xbbf54).
  The unsigned comparison `(r0 & 0xff) < [0x2972b4]:1` takes `0xbbf84`, while greater or equal falls through (`blo` at 0xbbf5c).
  A word read addresses `0x2972b0` (`ldr` at 0xbbf9c).
  External-interrupt name table bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbbf48 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbbf4c | load | 4 | `abs 0xbbfb4` | `-` |
| `ldrb` at 0xbbf50 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbbf54 | load | 1 | `abs 0x2972b4` | `-` |
| `ldr` at 0xbbf60 | load | 4 | `abs 0xbbfb8` | `-` |
| `str` at 0xbbf64 | store | 4 | `sp -0x1c` | `0x11efa0` |
| `ldrb` at 0xbbf68 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbbf6c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xbbf74 | load | 4 | `abs 0xbbfbc` | `-` |
| `ldrb` at 0xbbf84 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbbf98 | load | 4 | `abs 0xbbfc0` | `-` |
| `ldr` at 0xbbf9c | load | 4 | `abs 0x2972b0` | `-` |
| `ldr` at 0xbbfa4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xbbfb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbbfb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbbfb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbf80 | `(unknown callee) 0x200a0` | `0x9` | `0x11eeb8` | `0xdd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xbbf5c | `(r0 & 0xff) < [0x2972b4]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2972b4]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbbfb0 | `r0` | `?` |
