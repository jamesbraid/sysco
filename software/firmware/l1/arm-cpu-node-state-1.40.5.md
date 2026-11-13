# ARM CPU and node state paths in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
No unresolved computed transfers are reported. Unknown data values and instruction effects
remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/cpu-node-state-facts.txt) retain all tool sections.

## `cpuCount`

- `cpuCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e0a8 to 0x5e0c8.
  Established. The absolute word read uses `0x2af308` (`ldr` at 0x5e0b8). The return reports
  `[0x2af308]` (`ldmdb` at 0x5e0c0). CPU-count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5e0b4 | load | 4 | `abs 0x5e0c4` | `-` |
| `ldr` at 0x5e0b8 | load | 4 | `abs 0x2af308` | `-` |
| `ldmdb` at 0x5e0c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e0c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e0c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e0c0 | `r0` | `[0x2af308]` |

## `cpuPresent`

- `cpuPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e0c8 to 0x5e14c.
  Established. A word is loaded from `0x2af310` (`ldr` at 0x5e0e4). A zero value takes `0x5e114`,
  while nonzero falls through (`beq` at 0x5e0ec). The indirect call has an unknown target and
  carries `(r0 & 0xff)`, `r1`, `(r0 & 0xff)`, `[0x2af310]` (`mov` at 0x5e104). Presence semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e0dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5e0e0 | load | 4 | `abs 0x5e148` | `-` |
| `ldr` at 0x5e0e4 | load | 4 | `abs 0x2af310` | `-` |
| `ldr` at 0x5e0f0 | load | 4 | `abs 0x5e148` | `-` |
| `ldrb` at 0x5e0f4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e0f8 | load | 4 | `abs 0x2af310` | `-` |
| `str` at 0x5e10c | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldrb` at 0x5e114 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e118 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x5e11c | load | 4 | `abs 0x5e148` | `-` |
| `ldr` at 0x5e120 | load | 4 | `abs 0x2af308` | `-` |
| `ldr` at 0x5e124 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e134 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5e138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e13c | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x5e140 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e144 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e144 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e144 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x5e104 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x2af310]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e0ec | `[0x2af310] == 0x0` | `[0x2af310] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e144 | `r0` | `?` |

## `cpuEnabled`

- `cpuEnabled`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e14c to 0x5e1d8.
  Established. The signed condition `(r0 & 0xff) >= [0x2af308]` takes `0x5e1c4`, and smaller values
  fall through (`bge` at 0x5e174). The unnamed `0x1f26c` call carries `[sp-17]:1+88`, unknown,
  unknown, `[sp-17]:1+88` (`bl` at 0x5e19c). Enable-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e160 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5e164 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e168 | load | 4 | `abs 0x5e1d4` | `-` |
| `ldr` at 0x5e16c | load | 4 | `abs 0x2af308` | `-` |
| `ldrb` at 0x5e178 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5e190 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e1a0 | store | 4 | `sp -0x1c` | `ret(0x1f26c)` |
| `ldr` at 0x5e1a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e1b4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5e1b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e1bc | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x5e1c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e1cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e1d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e1d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e1d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e180 | `(unknown callee) 0x5e0c8` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x5e19c | `(unknown callee) 0x1f26c` | `[sp-17]:1+88` | `?` | `?` | `[sp-17]:1+88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x5e174 | `(r0 & 0xff) >= [0x2af308] (signed, 32-bit)` | `(r0 & 0xff) < [0x2af308] (signed, 32-bit)` |
| `beq` at 0x5e18c | `ret(0x5e0c8) == 0x0` | `ret(0x5e0c8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e1d0 | `r0` | `?` |

## `cpuDisable`

- `cpuDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e1d8 to 0x5e2cc.
  Established. The unnamed `0x3dce4` call carries `r0`, `r1`, `r2`, `0x0` (`bl` at 0x5e1f8); its
  zero result takes `0x5e214`, while nonzero falls through (`beq` at 0x5e204). The word assignment
  at `sp -0x24` writes `0xfffff63a` (`str` at 0x5e20c). Disable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e1ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5e1f4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e208 | load | 4 | `abs 0x5e2c4` | `-` |
| `str` at 0x5e20c | store | 4 | `sp -0x24` | `0xfffff63a` |
| `ldrb` at 0x5e214 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e224 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5e228 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5e22c | load | 4 | `abs 0x5e2c8` | `-` |
| `ldr` at 0x5e230 | load | 4 | `abs 0x2af308` | `-` |
| `ldrb` at 0x5e240 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5e258 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5e268 | store | 4 | `sp -0x1c` | `ret(0x5e3c0)` |
| `ldr` at 0x5e26c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e284 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e288 | store | 4 | `sp -0x18` | `ret(0x5e3c0)` |
| `ldrb` at 0x5e28c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5e294 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5e29c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e2a0 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5e2a8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e2b8 | store | 4 | `sp -0x24` | `ret(0x5e3c0)` |
| `ldr` at 0x5e2bc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e2c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e2c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e2c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e1f8 | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5e248 | `(unknown callee) 0x5e0c8` | `?` | `?` | `[0x2af308]` | `?` |
| `bl` at 0x5e260 | `(unknown callee) 0x5e3c0` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5e2b0 | `(unknown callee) 0x5e3c0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e204 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `bne` at 0x5e21c | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5e238 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e254 | `ret(0x5e0c8) == 0x0` | `ret(0x5e0c8) != 0x0` |
| `beq` at 0x5e274 | `ret(0x5e3c0) == 0x0` | `ret(0x5e3c0) != 0x0` |
| `bne` at 0x5e280 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e2c0 | `r0` | `?` |

## `cpuEnable`

- `cpuEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e2cc to 0x5e3c0.
  Established. The word at `sp -0x24` receives `0xfffff63a` (`str` at 0x5e300). The non-`0xff` byte
  condition `[sp-17]:1 != 0xff` takes `0x5e39c`, while equality falls through (`bne` at 0x5e310).
  The unnamed `0x5e47c` call at `0x5e3a4` carries `[sp-17]:1`, unknown, unknown, `[sp-17]:1` (`bl`
  at 0x5e3a4). Enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e2e0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5e2e8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e2fc | load | 4 | `abs 0x5e3b8` | `-` |
| `str` at 0x5e300 | store | 4 | `sp -0x24` | `0xfffff63a` |
| `ldrb` at 0x5e308 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e318 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5e31c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5e320 | load | 4 | `abs 0x5e3bc` | `-` |
| `ldr` at 0x5e324 | load | 4 | `abs 0x2af308` | `-` |
| `ldrb` at 0x5e334 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5e34c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5e35c | store | 4 | `sp -0x1c` | `ret(0x5e47c)` |
| `ldr` at 0x5e360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e36c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e378 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e37c | store | 4 | `sp -0x18` | `ret(0x5e47c)` |
| `ldrb` at 0x5e380 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5e388 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5e390 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e394 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5e39c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e3ac | store | 4 | `sp -0x24` | `ret(0x5e47c)` |
| `ldr` at 0x5e3b0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e3b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e3b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e3b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e2ec | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5e33c | `(unknown callee) 0x5e0c8` | `?` | `?` | `[0x2af308]` | `?` |
| `bl` at 0x5e354 | `(unknown callee) 0x5e47c` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5e3a4 | `(unknown callee) 0x5e47c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e2f8 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `bne` at 0x5e310 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5e32c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e348 | `ret(0x5e0c8) == 0x0` | `ret(0x5e0c8) != 0x0` |
| `beq` at 0x5e368 | `ret(0x5e47c) == 0x0` | `ret(0x5e47c) != 0x0` |
| `bne` at 0x5e374 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e3b4 | `r0` | `?` |

## `cpuSetState`

- `cpuSetState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e538 to 0x5e600.
  Established. The unnamed `0x5f160` call at `0x5e5e4` carries `(r0 & 0xff)+10`, `(r1 & 0xff)`, `(r1 & 0xff)`, `(r0 & 0xff)+10` (`bl` at 0x5e5e4). The word assignment at `sp -0x18` writes
  `0xfffff63b` (`str` at 0x5e5c8). CPU-state values remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e550 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x5e558 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x5e55c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e56c | store | 1 | `sp -0x13` | `0x0` |
| `ldrb` at 0x5e570 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x5e574 | load | 4 | `abs 0x5e5f8` | `-` |
| `ldr` at 0x5e578 | load | 4 | `abs 0x2af308` | `-` |
| `ldrb` at 0x5e588 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x5e590 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x5e5a0 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0x5e5a8 | store | 1 | `sp -0x13` | `([sp-19]:1+1 & 0xff)` |
| `ldrb` at 0x5e5b0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e5b4 | load | 4 | `abs 0x5e5f8` | `-` |
| `ldr` at 0x5e5b8 | load | 4 | `abs 0x2af308` | `-` |
| `ldr` at 0x5e5c4 | load | 4 | `abs 0x5e5fc` | `-` |
| `str` at 0x5e5c8 | store | 4 | `sp -0x18` | `0xfffff63b` |
| `ldrb` at 0x5e5d0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5e5d8 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x5e5ec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e5f0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e5f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e5f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e5f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e59c | `(unknown callee) 0x5f160` | `?` | `?` | `?` | `?` |
| `bl` at 0x5e5e4 | `(unknown callee) 0x5f160` | `(r0 & 0xff)+10` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5e564 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x5e580 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5e5c0 | `(r0 & 0xff) < [0x2af308] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2af308] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e5f4 | `r0` | `?` |

## `cpuState`

- `cpuState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e600 to 0x5e664.
  Established. The absolute four-byte load uses `0x2af308` (`ldr` at 0x5e620). The signed
  smaller-than condition `(r0 & 0xff) < [0x2af308]` takes `0x5e638`, while greater-or-equal falls
  through (`blt` at 0x5e628). The unnamed `0x5f314` call carries `(r0 & 0xff)+10`, `r1`, `(r0 & 0xff)`, `(r0 & 0xff)+10` (`bl` at 0x5e644). Returned state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e614 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5e618 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e61c | load | 4 | `abs 0x5e65c` | `-` |
| `ldr` at 0x5e620 | load | 4 | `abs 0x2af308` | `-` |
| `ldr` at 0x5e62c | load | 4 | `abs 0x5e660` | `-` |
| `str` at 0x5e630 | store | 4 | `sp -0x18` | `0xfffff63b` |
| `ldrb` at 0x5e638 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e650 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5e654 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e658 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e658 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e644 | `(unknown callee) 0x5f314` | `(r0 & 0xff)+10` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5e628 | `(r0 & 0xff) < [0x2af308] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2af308] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e658 | `r0` | `?` |

## `cpuStateString`

- `cpuStateString`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e664 to 0x5e734.
  Established. The unnamed `0x5f314` call carries `(r0 & 0xff)+10`, `r1`, `r2`, `(r0 & 0xff)+10`
  (`bl` at 0x5e688). The absolute word at `0x143568` is read (`ldr` at 0x5e6a0). The word
  assignment at `sp -0x20` writes `0x10ce2c` (`str` at 0x5e718). State-label contents remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e678 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5e67c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e68c | store | 4 | `sp -0x1c` | `ret(0x5f314)` |
| `str` at 0x5e694 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e698 | load | 4 | `abs 0x5e728` | `-` |
| `ldr` at 0x5e69c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e6a0 | load | 4 | `abs 0x143568` | `-` |
| `ldr` at 0x5e6b0 | load | 4 | `abs 0x5e72c` | `-` |
| `ldr` at 0x5e6b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e6b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e6bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e6cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e6d4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5e6dc | load | 4 | `abs 0x5e728` | `-` |
| `ldr` at 0x5e6e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e6e4 | load | 4 | `abs 0x143568` | `-` |
| `ldr` at 0x5e6f0 | load | 4 | `abs 0x5e72c` | `-` |
| `ldr` at 0x5e6f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e708 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5e70c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5e714 | load | 4 | `abs 0x5e730` | `-` |
| `str` at 0x5e718 | store | 4 | `sp -0x20` | `0x10ce2c` |
| `ldr` at 0x5e71c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5e724 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e724 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e724 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e688 | `(unknown callee) 0x5f314` | `(r0 & 0xff)+10` | `r1` | `r2` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5e6a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5e6c4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5e6ec | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e724 | `r0` | `?` |

## `nodeCount`

- `nodeCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e734 to 0x5e754.
  Established. The four-byte absolute load reads `0x2af334` (`ldr` at 0x5e744); its expression
  `[0x2af334]` is the reported return value (`ldmdb` at 0x5e74c). Node-count units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5e740 | load | 4 | `abs 0x5e750` | `-` |
| `ldr` at 0x5e744 | load | 4 | `abs 0x2af334` | `-` |
| `ldmdb` at 0x5e74c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e74c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e74c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e74c | `r0` | `[0x2af334]` |

## `nodePresent`

- `nodePresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e754 to 0x5e7d8.
  Established. The word at `0x2af338` is loaded (`ldr` at 0x5e770). Equality with zero takes
  `0x5e7a0`, and nonzero falls through (`beq` at 0x5e778). The indirect call has an unknown target
  with candidates `(r0 & 0xff)`, `r1`, `(r0 & 0xff)`, `[0x2af338]` (`mov` at 0x5e790). Node-presence
  policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e768 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5e76c | load | 4 | `abs 0x5e7d4` | `-` |
| `ldr` at 0x5e770 | load | 4 | `abs 0x2af338` | `-` |
| `ldr` at 0x5e77c | load | 4 | `abs 0x5e7d4` | `-` |
| `ldrb` at 0x5e780 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e784 | load | 4 | `abs 0x2af338` | `-` |
| `str` at 0x5e798 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldrb` at 0x5e7a0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e7a4 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x5e7a8 | load | 4 | `abs 0x5e7d4` | `-` |
| `ldr` at 0x5e7ac | load | 4 | `abs 0x2af334` | `-` |
| `ldr` at 0x5e7b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e7c0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5e7c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e7c8 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x5e7cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e7d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e7d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e7d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x5e790 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x2af338]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e778 | `[0x2af338] == 0x0` | `[0x2af338] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e7d0 | `r0` | `?` |

## `nodeEnabled`

- `nodeEnabled`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e7d8 to 0x5e874.
  Established. The unnamed `0x5e754` call carries `(r0 & 0xff)`, `r1`, `(r0 & 0xff)`, `(r0 & 0xff)`
  (`bl` at 0x5e80c); its zero result takes `0x5e860`, while nonzero falls through (`beq` at
  0x5e818). The unnamed `0x1f26c` call carries `0x70`, unknown, unknown, `ret(0x5e754)` (`bl` at
  0x5e820). Node-enable mask meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e7ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5e7f0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e7f4 | load | 4 | `abs 0x5e870` | `-` |
| `ldr` at 0x5e7f8 | load | 4 | `abs 0x2af334` | `-` |
| `ldrb` at 0x5e804 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e824 | store | 4 | `sp -0x1c` | `ret(0x1f26c)` |
| `ldrb` at 0x5e828 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5e834 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e83c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5e840 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e850 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5e854 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e858 | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x5e864 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e868 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5e86c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e86c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e86c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e80c | `(unknown callee) 0x5e754` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x5e820 | `(unknown callee) 0x1f26c` | `0x70` | `?` | `?` | `ret(0x5e754)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x5e800 | `(r0 & 0xff) >= [0x2af334] (signed, 32-bit)` | `(r0 & 0xff) < [0x2af334] (signed, 32-bit)` |
| `beq` at 0x5e818 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e86c | `r0` | `?` |

## `nodeDisable`

- `nodeDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e874 to 0x5e968.
  Established. A word at `sp -0x24` receives `0xfffff5d6` (`str` at 0x5e8a8). The unnamed `0x5ebc0`
  call at `0x5e94c` carries `[sp-17]:1`, unknown, unknown, `[sp-17]:1` (`bl` at 0x5e94c).
  Disable-result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e888 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5e890 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e8a4 | load | 4 | `abs 0x5e960` | `-` |
| `str` at 0x5e8a8 | store | 4 | `sp -0x24` | `0xfffff5d6` |
| `ldrb` at 0x5e8b0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e8c0 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5e8c4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5e8c8 | load | 4 | `abs 0x5e964` | `-` |
| `ldr` at 0x5e8cc | load | 4 | `abs 0x2af334` | `-` |
| `ldrb` at 0x5e8dc | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5e8f4 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5e904 | store | 4 | `sp -0x1c` | `ret(0x5ebc0)` |
| `ldr` at 0x5e908 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e914 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e920 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e924 | store | 4 | `sp -0x18` | `ret(0x5ebc0)` |
| `ldrb` at 0x5e928 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5e930 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5e938 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e93c | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5e944 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5e954 | store | 4 | `sp -0x24` | `ret(0x5ebc0)` |
| `ldr` at 0x5e958 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e95c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e95c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e894 | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5e8e4 | `(unknown callee) 0x5e754` | `?` | `?` | `[0x2af334]` | `?` |
| `bl` at 0x5e8fc | `(unknown callee) 0x5ebc0` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5e94c | `(unknown callee) 0x5ebc0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e8a0 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `bne` at 0x5e8b8 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5e8d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e8f0 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `beq` at 0x5e910 | `ret(0x5ebc0) == 0x0` | `ret(0x5ebc0) != 0x0` |
| `bne` at 0x5e91c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e95c | `r0` | `?` |

## `nodeEnable`

- `nodeEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e968 to 0x5ea5c.
  Established. The word assignment at `sp -0x24` writes `0xfffff5d6` (`str` at 0x5e99c). The unnamed
  `0x5ec90` call at `0x5ea40` carries `[sp-17]:1`, unknown, unknown, `[sp-17]:1` (`bl` at 0x5ea40).
  Node activation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5e97c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5e984 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5e998 | load | 4 | `abs 0x5ea54` | `-` |
| `str` at 0x5e99c | store | 4 | `sp -0x24` | `0xfffff5d6` |
| `ldrb` at 0x5e9a4 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5e9b4 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5e9b8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5e9bc | load | 4 | `abs 0x5ea58` | `-` |
| `ldr` at 0x5e9c0 | load | 4 | `abs 0x2af334` | `-` |
| `ldrb` at 0x5e9d0 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5e9e8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5e9f8 | store | 4 | `sp -0x1c` | `ret(0x5ec90)` |
| `ldr` at 0x5e9fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ea08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ea14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ea18 | store | 4 | `sp -0x18` | `ret(0x5ec90)` |
| `ldrb` at 0x5ea1c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5ea24 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5ea2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5ea30 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5ea38 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5ea48 | store | 4 | `sp -0x24` | `ret(0x5ec90)` |
| `ldr` at 0x5ea4c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5ea50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ea50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ea50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e988 | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5e9d8 | `(unknown callee) 0x5e754` | `?` | `?` | `[0x2af334]` | `?` |
| `bl` at 0x5e9f0 | `(unknown callee) 0x5ec90` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5ea40 | `(unknown callee) 0x5ec90` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e994 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `bne` at 0x5e9ac | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5e9c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5e9e4 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `beq` at 0x5ea04 | `ret(0x5ec90) == 0x0` | `ret(0x5ec90) != 0x0` |
| `bne` at 0x5ea10 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ea50 | `r0` | `?` |

## `nodeOp`

- `nodeOp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ea5c to 0x5ebc0.
  Established. The byte assignment at `sp -0x15` writes `(r1 & 0xff)` (`strb` at 0x5ea74). The
  indirect call at `0x5ead4` carries `[sp-21]:1`, unknown, `[sp-20]`, `[sp-21]:1` with an unknown
  target (`mov` at 0x5ead4). The word assignment at `sp -0x24` writes `0xfffff5d7` (`str` at
  0x5eb34). Node operation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ea6c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5ea74 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x5ea7c | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x5ea80 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x5ea90 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x5ea94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5ea98 | load | 4 | `abs 0x5ebb0` | `-` |
| `ldr` at 0x5ea9c | load | 4 | `abs 0x2af334` | `-` |
| `ldrb` at 0x5eaac | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5eac4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5eac8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5eadc | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x5eae0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5eaec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5eaf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5eafc | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldrb` at 0x5eb00 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x5eb08 | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldr` at 0x5eb10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5eb14 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x5eb1c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5eb20 | load | 4 | `abs 0x5ebb0` | `-` |
| `ldr` at 0x5eb24 | load | 4 | `abs 0x2af334` | `-` |
| `ldr` at 0x5eb30 | load | 4 | `abs 0x5ebb4` | `-` |
| `str` at 0x5eb34 | store | 4 | `sp -0x24` | `0xfffff5d7` |
| `ldrb` at 0x5eb3c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5eb54 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5eb58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5eb6c | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldrb` at 0x5eb74 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5eb8c | load | 4 | `abs 0x5ebb8` | `-` |
| `str` at 0x5eb90 | store | 4 | `sp -0x28` | `0xfffff5d5` |
| `ldr` at 0x5eb98 | load | 4 | `abs 0x5ebbc` | `-` |
| `str` at 0x5eb9c | store | 4 | `sp -0x28` | `0xfffff5d4` |
| `ldr` at 0x5eba0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5eba4 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x5eba8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5ebac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ebac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ebac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5eab4 | `(unknown callee) 0x5e7d8` | `?` | `?` | `[0x2af334]` | `?` |
| `mov` at 0x5ead4 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-20]` | `[sp-21]:1` |
| `bl` at 0x5eb44 | `(unknown callee) 0x5e7d8` | `(r1 & 0xff)` | `r1` | `[0x2af334]` | `(r1 & 0xff)` |
| `mov` at 0x5eb64 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-20]` | `[sp-21]:1` |
| `bl` at 0x5eb7c | `(unknown callee) 0x5e754` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5ea88 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `blt` at 0x5eaa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5eac0 | `ret(0x5e7d8) == 0x0` | `ret(0x5e7d8) != 0x0` |
| `beq` at 0x5eae8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x5eaf4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x5eb2c | `(r1 & 0xff) < [0x2af334] (signed, 32-bit)` | `(r1 & 0xff) >= [0x2af334] (signed, 32-bit)` |
| `beq` at 0x5eb50 | `ret(0x5e7d8) == 0x0` | `ret(0x5e7d8) != 0x0` |
| `beq` at 0x5eb88 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ebac | `r0` | `?` |

## `nmiMaskSet`

- `nmiMaskSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e998 to 0x4ea1c.
  Established. The absolute word stores at `0x148f74` both have unknown values (`str` at 0x4e9c4,
  `str` at 0x4e9dc). The stores at `0x148f78` likewise write unknown words (`str` at 0x4e9f4, `str`
  at 0x4ea0c). Mask bit selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e9a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4e9ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4e9b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e9bc | load | 4 | `abs 0x4ea14` | `-` |
| `str` at 0x4e9c4 | store | 4 | `abs 0x148f74` | `?` |
| `ldr` at 0x4e9cc | load | 4 | `abs 0x4ea14` | `-` |
| `ldr` at 0x4e9d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4e9dc | store | 4 | `abs 0x148f74` | `?` |
| `ldr` at 0x4e9e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e9ec | load | 4 | `abs 0x4ea18` | `-` |
| `str` at 0x4e9f4 | store | 4 | `abs 0x148f78` | `?` |
| `ldr` at 0x4e9fc | load | 4 | `abs 0x4ea18` | `-` |
| `ldr` at 0x4ea04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4ea0c | store | 4 | `abs 0x148f78` | `?` |
| `ldmdb` at 0x4ea10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4ea10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4ea10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4e9b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x4e9e8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4ea10 | `r0` | `r0` |
