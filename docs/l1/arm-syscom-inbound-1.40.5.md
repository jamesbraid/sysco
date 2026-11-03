# ARM inbound communication and request paths in 1.40.5

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
L1CfgRequest and SysComInit have unresolved computed transfers. Tables cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/syscom-inbound-facts.txt) retain all tool sections.

## `L1CfgDelete`

- `L1CfgDelete`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71dd0 to 0x7210c.
  Established. Two included four-byte assignments at `[sp-24] +0x0` and `[sp-32] +0x0` write
  `[[sp-24]]-1` and `[[sp-32]]-1` respectively (`str` at 0x71f5c, `str` at 0x720e0). Separate
  unnamed `0x1b4cc` calls carry unknown, `0x0`, `0x18`, unknown and unknown, `0x0`, `0x70`, unknown
  (`bl` at 0x71f48, `bl` at 0x720cc). Deletion targets and record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71de0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x71de4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x71de8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x71dec | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x71df4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x71df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71e04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71e0c | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x71e1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71e30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71e38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71e3c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71e48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71e5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71e64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71e68 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x71e74 | load | 4 | `abs 0x720fc` | `-` |
| `ldr` at 0x71e78 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x71e88 | load | 4 | `abs 0x72100` | `-` |
| `ldr` at 0x71e8c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71e90 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x71e94 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x71e98 | load | 4 | `abs 0x72104` | `-` |
| `ldr` at 0x71e9c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71ea0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x71ea8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71ebc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71ec4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71ee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71ee8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71eec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71f18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71f1c | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x71f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71f4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71f50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71f54 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x71f5c | store | 4 | `[sp-24] +0x0` | `[[sp-24]]-1` |
| `ldr` at 0x71f64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71f6c | store | 4 | `sp -0x24` | `?` |
| `str` at 0x71f78 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x71f7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71f88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71f8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71f90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71fa0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71fb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71fbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71fc0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71fcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71fe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71fec | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x71ff8 | load | 4 | `abs 0x720fc` | `-` |
| `ldr` at 0x71ffc | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x7200c | load | 4 | `abs 0x72100` | `-` |
| `ldr` at 0x72010 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x72014 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x72018 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7201c | load | 4 | `abs 0x72108` | `-` |
| `ldr` at 0x72020 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x72024 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7202c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72048 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7205c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72068 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7206c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x72070 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7209c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x720a0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x720b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x720d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x720d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x720d8 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x720e0 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]-1` |
| `ldr` at 0x720e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x720f0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x720f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x720f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x720f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x71ea4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1102c4` | `[sp]` | `[sp+4]` |
| `bl` at 0x71f14 | `(unknown callee) 0x1b1fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x71f48 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `0x18` | `?` |
| `bl` at 0x72028 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1102e4` | `[sp]` | `[sp+4]` |
| `bl` at 0x72098 | `(unknown callee) 0x1b1fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x720cc | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x71e00 | `r0 == 0x0` | `r0 != 0x0` |
| `blt` at 0x71e14 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x71e44 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x71e70 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x71e84 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x71f84 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x71f98 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x71fc8 | `? != [sp]` | `? == [sp]` |
| `bne` at 0x71ff4 | `? != [sp+4]` | `? == [sp+4]` |
| `beq` at 0x72008 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x720f8 | `r0` | `?` |

## `L1CfgUpdateState`

- `L1CfgUpdateState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7210c to 0x722f8.
  Established. A four-byte absolute assignment at `0x1493f8` writes `ret(0x1b274)` (`str` at
  0x721cc). The unnamed `0x71a84` call has `[sp-404]`, `[sp-400]`, `sp-412`, `0x1` (`bl` at
  0x722c4). The earlier branch takes `[sp-292] != [0x1496b4]` to `0x721c0` (`bne` at 0x721a4). State
  update encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7211c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x72120 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72124 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x72134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72138 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x7213c | store | 4 | `sp -0x1a0` | `[r0+40]` |
| `ldr` at 0x72144 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72150 | store | 4 | `sp -0x1a0` | `ret(0xb823c)` |
| `ldr` at 0x72154 | load | 4 | `sp -0x1a0` | `-` |
| `str` at 0x72158 | store | 4 | `sp -0x120` | `[sp-416]` |
| `ldr` at 0x7215c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72160 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x72170 | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0x72174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72178 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x72184 | store | 4 | `sp -0x128` | `?` |
| `ldr` at 0x72188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7218c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x72190 | store | 4 | `sp -0x12c` | `[[sp-24]+12]` |
| `ldr` at 0x72194 | load | 4 | `abs 0x722e0` | `-` |
| `ldr` at 0x72198 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x7219c | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x721a8 | load | 4 | `abs 0x722e0` | `-` |
| `ldr` at 0x721ac | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0x721b0 | load | 4 | `abs 0x1496b8` | `-` |
| `ldr` at 0x721c0 | load | 4 | `abs 0x722e4` | `-` |
| `str` at 0x721cc | store | 4 | `abs 0x1493f8` | `ret(0x1b274)` |
| `ldr` at 0x721d0 | load | 4 | `abs 0x722e8` | `-` |
| `ldr` at 0x721d8 | load | 4 | `abs 0x722ec` | `-` |
| `ldr` at 0x721dc | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x721f8 | load | 4 | `abs 0x722f0` | `-` |
| `ldr` at 0x721fc | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x72200 | store | 4 | `sp -0x1ac` | `[sp-300]` |
| `str` at 0x72204 | store | 4 | `sp -0x1a8` | `ret(0x1b274)` |
| `ldr` at 0x72208 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7220c | load | 4 | `abs 0x722f4` | `-` |
| `ldr` at 0x72210 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x72214 | load | 4 | `sp -0x128` | `-` |
| `str` at 0x72220 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x72224 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x72228 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x72238 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7223c | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x72250 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72264 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x72278 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7227c | store | 4 | `sp -0x1a4` | `?` |
| `ldr` at 0x72284 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x7228c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72298 | store | 4 | `sp -0x1a4` | `ret(0xb823c)` |
| `ldr` at 0x722a4 | load | 4 | `sp -0x1a4` | `-` |
| `ldr` at 0x722b4 | load | 4 | `sp -0x194` | `-` |
| `ldr` at 0x722b8 | load | 4 | `sp -0x190` | `-` |
| `ldr` at 0x722c8 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x722d0 | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldmdb` at 0x722dc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x722dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x722dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x722dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7214c | `(unknown callee) 0xb823c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x721c8 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x721d4 | `(unknown callee) 0x6c95c` | `0x110304` | `?` | `?` | `?` |
| `bl` at 0x721f0 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x72218 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110318` | `[sp-292]` | `[sp-296]` |
| `bl` at 0x72294 | `(unknown callee) 0xb823c` | `[sp-24]` | `?` | `0x2c` | `?` |
| `bl` at 0x722ac | `(unknown callee) 0x72548` | `sp-412` | `?` | `0x70` | `sp-412` |
| `bl` at 0x722c4 | `(unknown callee) 0x71a84` | `[sp-404]` | `[sp-400]` | `sp-412` | `0x1` |
| `bl` at 0x722d8 | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `[sp-288]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72130 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x721a4 | `[sp-292] != [0x1496b4]` | `[sp-292] == [0x1496b4]` |
| `bne` at 0x721b8 | `[sp-296] != [0x1496b8]` | `[sp-296] == [0x1496b8]` |
| `beq` at 0x721e8 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x72230 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x7225c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x722dc | `r0` | `?` |

## `L1CfgRequest`

- `L1CfgRequest`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72734 to 0x72978.
  Established. The unnamed `0x7a658` call carries `[sp-84]`, `r1`, `0x149750`, unknown (`bl` at
  0x72764); its nonzero result takes the branch to `0x72778` (`bne` at 0x72770). The unresolved
  computed transfer takes unsigned `[sp-20]-1 <= 0x18` and falls through for `[sp-20]-1 > 0x18`
  (`ldrls` at 0x7279c). Selector destinations and request meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72744 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7274c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x72750 | load | 4 | `abs 0x72950` | `-` |
| `ldr` at 0x72754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72758 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7275c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x72760 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x72788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72790 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x7279c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x7294c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7294c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7294c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72764 | `(unknown callee) 0x7a658` | `[sp-84]` | `r1` | `0x149750` | `?` |
| `bl` at 0x7278c | `(unknown callee) 0x69908` | `sp-76` | `sp-72` | `[sp-20]` | `sp-76` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72770 | `ret(0x7a658) != 0x0` | `ret(0x7a658) == 0x0` |
| `ldrls` at 0x7279c | `[sp-20]-1 <= 0x18 (unsigned, 32-bit)` | `[sp-20]-1 > 0x18 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7294c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7279c).

## `L1CfgProcessRequest`

- `L1CfgProcessRequest`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72978 to 0x73034.
  Established. Included four-byte absolute assignments at `0x149640`, `0x149644` and `0x149648`
  write `[0x149738]`, `ret(0x1b1b4)` and `ret(0x1b1b4)` (`str` at 0x72bf4, `str` at 0x72c0c, `str`
  at 0x72c38). The unnamed `0xb74d4` call uses `0x1496a4`, `0x0`, `[sp-24]`, `[sp-100]` (`bl` at
  0x72e6c). Request codes, allocation sizes and pointee layouts remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72988 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x72990 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x72994 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72998 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x729a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x729ac | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x729b0 | store | 4 | `sp -0x8c` | `[r0+40]` |
| `ldr` at 0x729b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x729c4 | store | 4 | `sp -0x8c` | `ret(0xb823c)` |
| `ldr` at 0x729c8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x729cc | store | 4 | `sp -0x74` | `[sp-140]` |
| `ldr` at 0x729e0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x729e8 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x729ec | store | 4 | `sp -0x98` | `[sp-116]` |
| `ldr` at 0x729f0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x729fc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a08 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a14 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a20 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a30 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a40 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a4c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a58 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72a68 | load | 4 | `abs 0x72ff4` | `-` |
| `str` at 0x72a74 | store | 4 | `abs 0x1493f0` | `ret(0x1b274)` |
| `ldr` at 0x72a78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72a84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72a88 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72a94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72a98 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72aa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72aac | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72ab0 | store | 4 | `sp -0x90` | `[[sp-24]+48]` |
| `ldr` at 0x72ab8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72ac4 | store | 4 | `sp -0x90` | `ret(0xb823c)` |
| `ldr` at 0x72ac8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x72acc | store | 4 | `sp -0x78` | `[sp-144]` |
| `ldr` at 0x72ad0 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x72ae0 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x72ae4 | load | 4 | `abs 0x72ff8` | `-` |
| `ldr` at 0x72ae8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x72afc | load | 4 | `abs 0x72ff8` | `-` |
| `ldr` at 0x72b00 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x72b0c | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x72b10 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x72b2c | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x72b30 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x72b44 | load | 4 | `abs 0x72ffc` | `-` |
| `ldr` at 0x72b4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72b50 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72b5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72b60 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x72b70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72b74 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x72b78 | store | 4 | `sp -0x94` | `[[sp-24]+56]` |
| `ldr` at 0x72b80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72b8c | store | 4 | `sp -0x94` | `ret(0xb823c)` |
| `ldr` at 0x72b90 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x72b94 | store | 4 | `sp -0x7c` | `[sp-148]` |
| `ldr` at 0x72b98 | load | 4 | `abs 0x73000` | `-` |
| `ldr` at 0x72b9c | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72ba0 | load | 4 | `abs 0x149640` | `-` |
| `ldr` at 0x72ba4 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x72bb0 | load | 4 | `abs 0x73008` | `-` |
| `ldr` at 0x72bb4 | load | 4 | `abs 0x149644` | `-` |
| `ldr` at 0x72bc0 | load | 4 | `abs 0x73008` | `-` |
| `ldr` at 0x72bc4 | load | 4 | `abs 0x149644` | `-` |
| `ldr` at 0x72bcc | load | 4 | `abs 0x7300c` | `-` |
| `ldr` at 0x72bd0 | load | 4 | `abs 0x149648` | `-` |
| `ldr` at 0x72bdc | load | 4 | `abs 0x7300c` | `-` |
| `ldr` at 0x72be0 | load | 4 | `abs 0x149648` | `-` |
| `ldr` at 0x72be8 | load | 4 | `abs 0x73000` | `-` |
| `ldr` at 0x72bec | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72bf0 | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x72bf4 | store | 4 | `abs 0x149640` | `[0x149738]` |
| `ldr` at 0x72bf8 | load | 4 | `abs 0x73008` | `-` |
| `ldr` at 0x72bfc | load | 4 | `abs 0x73000` | `-` |
| `ldr` at 0x72c00 | load | 4 | `abs 0x149640` | `-` |
| `str` at 0x72c0c | store | 4 | `abs 0x149644` | `ret(0x1b1b4)` |
| `ldr` at 0x72c10 | load | 4 | `abs 0x7300c` | `-` |
| `ldr` at 0x72c14 | load | 4 | `abs 0x73000` | `-` |
| `ldr` at 0x72c18 | load | 4 | `abs 0x149640` | `-` |
| `str` at 0x72c38 | store | 4 | `abs 0x149648` | `ret(0x1b1b4)` |
| `ldr` at 0x72c54 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x72c64 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x72c6c | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x72c74 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x72c78 | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72c7c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x72c80 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x72c90 | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72c94 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x72ca8 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x72cb0 | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72cb4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x72cc8 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x72cd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72cd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x72ce0 | store | 4 | `sp -0x78` | `ret(0x74f94)` |
| `ldr` at 0x72ce4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x72cf0 | load | 4 | `abs 0x73008` | `-` |
| `ldr` at 0x72cf4 | load | 4 | `abs 0x149644` | `-` |
| `ldr` at 0x72cf8 | load | 4 | `sp -0x88` | `-` |
| `ldrb` at 0x72d00 | load | 1 | `sp -0x80` | `-` |
| `strb` at 0x72d04 | store | 1 | `([0x149644] + [sp-136]) +0x0` | `([sp-128]:1 & 0xff)` |
| `ldr` at 0x72d08 | load | 4 | `abs 0x7300c` | `-` |
| `ldr` at 0x72d0c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72d20 | load | 4 | `abs 0x149648` | `-` |
| `ldr` at 0x72d2c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x72d38 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x72d40 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x72d44 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x72d4c | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldr` at 0x72d64 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x72d7c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72da4 | load | 4 | `abs 0x73008` | `-` |
| `ldr` at 0x72db0 | load | 4 | `abs 0x149644` | `-` |
| `ldr` at 0x72db4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72dc4 | load | 4 | `abs 0x7300c` | `-` |
| `ldr` at 0x72dd0 | load | 4 | `abs 0x149648` | `-` |
| `ldr` at 0x72dd4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72df0 | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72e0c | load | 4 | `abs 0x73010` | `-` |
| `ldr` at 0x72e28 | load | 4 | `abs 0x73010` | `-` |
| `ldr` at 0x72e44 | load | 4 | `abs 0x73010` | `-` |
| `str` at 0x72e58 | store | 4 | `sp -0xa4` | `sp-96` |
| `ldr` at 0x72e5c | load | 4 | `abs 0x73014` | `-` |
| `ldr` at 0x72e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72e68 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x72e74 | store | 4 | `sp -0x68` | `ret(0xb74d4)` |
| `ldr` at 0x72e7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72e98 | load | 4 | `abs 0x73004` | `-` |
| `ldr` at 0x72eb4 | load | 4 | `abs 0x73010` | `-` |
| `ldr` at 0x72ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72ec4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x72ec8 | store | 4 | `sp -0x20` | `[[sp-24]+28]` |
| `ldr` at 0x72ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72ed0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x72ed4 | store | 4 | `sp -0x70` | `[[sp-24]+12]` |
| `ldr` at 0x72ed8 | load | 4 | `abs 0x73018` | `-` |
| `ldr` at 0x72edc | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x72eec | load | 4 | `abs 0x7301c` | `-` |
| `ldr` at 0x72ef0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x72ef4 | load | 4 | `abs 0x73020` | `-` |
| `ldr` at 0x72ef8 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x72f08 | store | 4 | `sp -0xa4` | `0x2` |
| `ldr` at 0x72f0c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x72f10 | store | 4 | `sp -0xa0` | `[sp-100]` |
| `str` at 0x72f18 | store | 4 | `sp -0x9c` | `sp-96` |
| `ldr` at 0x72f1c | load | 4 | `abs 0x73014` | `-` |
| `ldr` at 0x72f24 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x72f34 | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0x72f38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72f44 | load | 4 | `abs 0x7301c` | `-` |
| `ldr` at 0x72f48 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x72f4c | load | 4 | `abs 0x73024` | `-` |
| `ldr` at 0x72f58 | load | 4 | `abs 0x73018` | `-` |
| `ldr` at 0x72f5c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x72f6c | load | 4 | `abs 0x7301c` | `-` |
| `ldr` at 0x72f70 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x72f74 | load | 4 | `abs 0x73028` | `-` |
| `ldr` at 0x72f78 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x72f80 | load | 4 | `abs 0x7302c` | `-` |
| `ldr` at 0x72f84 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x72f98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72fa4 | load | 4 | `abs 0x7302c` | `-` |
| `ldr` at 0x72fa8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x72fbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72fc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72fdc | load | 4 | `abs 0x73030` | `-` |
| `ldr` at 0x72fe0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x72fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x72ff0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x72ff0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x72ff0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x72ff0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x729c0 | `(unknown callee) 0xb823c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x729e4 | `(unknown callee) 0x69908` | `sp-100` | `sp-96` | `[sp-140]` | `sp-100` |
| `bl` at 0x72a70 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[sp-116]` |
| `bl` at 0x72a7c | `(unknown callee) 0x6ca7c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x72ac0 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x72aec | `(unknown callee) 0x1b088` | `0x149710` | `[sp-144]` | `?` | `0x0` |
| `bl` at 0x72b04 | `(unknown callee) 0x1b598` | `0x149710` | `[sp-120]` | `?` | `ret(0x1b088)` |
| `bl` at 0x72b18 | `(unknown callee) 0xaf274` | `[sp-120]` | `0x8000` | `?` | `?` |
| `bl` at 0x72b40 | `(unknown callee) 0x75a50` | `0x1` | `?` | `?` | `[sp-124]` |
| `bl` at 0x72b48 | `(unknown callee) 0x6c95c` | `0x1103a8` | `?` | `?` | `?` |
| `bl` at 0x72b88 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x72bc8 | `(unknown callee) 0x1b490` | `[0x149644]` | `[0x149640]` | `0x1496ac` | `0x149644` |
| `bl` at 0x72be4 | `(unknown callee) 0x1b490` | `[0x149648]` | `?` | `?` | `0x149648` |
| `bl` at 0x72c04 | `(unknown callee) 0x1b1b4` | `[0x149640]` | `?` | `0x149640` | `0x149640` |
| `bl` at 0x72c30 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[0x149640]` | `?` |
| `bl` at 0x72c50 | `(unknown callee) 0x69908` | `sp-100` | `sp-96` | `0x7` | `sp-100` |
| `bl` at 0x72cd8 | `(unknown callee) 0x74f94` | `?` | `?` | `?` | `?` |
| `bl` at 0x72d34 | `(unknown callee) 0x1b394` | `?` | `[sp-120]` | `0x70` | `?` |
| `bl` at 0x72d68 | `(unknown callee) 0x69964` | `sp-100` | `sp-96` | `[sp-124]` | `sp-100` |
| `bl` at 0x72d80 | `(unknown callee) 0x69964` | `sp-100` | `sp-96` | `[sp-136]` | `sp-100` |
| `bl` at 0x72d98 | `(unknown callee) 0x69964` | `sp-100` | `sp-96` | `0x70` | `sp-100` |
| `bl` at 0x72db8 | `(unknown callee) 0x69adc` | `sp-100` | `sp-96` | `[0x149644]` | `[sp-136]` |
| `bl` at 0x72dd8 | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `[0x149648]` | `[sp-136]` |
| `bl` at 0x72df8 | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `0x1496ac` | `0x1` |
| `bl` at 0x72e14 | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `0x149950` | `0x1` |
| `bl` at 0x72e30 | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `0x149950` | `0x1` |
| `bl` at 0x72e4c | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `0x149950` | `0x1` |
| `bl` at 0x72e50 | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `?` |
| `bl` at 0x72e6c | `(unknown callee) 0xb74d4` | `0x1496a4` | `0x0` | `[sp-24]` | `[sp-100]` |
| `bl` at 0x72e84 | `(unknown callee) 0x6f354` | `[sp-24]` | `0x1` | `?` | `[sp-116]` |
| `bl` at 0x72ea0 | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `0x1496ac` | `0x1` |
| `bl` at 0x72ebc | `(unknown callee) 0x722f8` | `sp-100` | `sp-96` | `0x149950` | `0x1` |
| `bl` at 0x72efc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1103c0` | `[[sp-24]+12]` | `0x145ae8` |
| `bl` at 0x72f2c | `(unknown callee) 0xb785c` | `0x1496a4` | `sp-32` | `?` | `0x1` |
| `bl` at 0x72f50 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1103e0` | `?` | `0x145ae8` |
| `bl` at 0x72f7c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110418` | `[sp-112]` | `0x145ae8` |
| `bl` at 0x72fcc | `(unknown callee) 0xb7164` | `?` | `?` | `0x50` | `?` |
| `bl` at 0x72fe4 | `(unknown callee) 0x2079c` | `0x11043c` | `[sp-112]` | `?` | `ret(0xb7164)` |
| `bl` at 0x72fec | `(unknown callee) 0xb5fec` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x729a4 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x729f8 | `[sp-116] > 0xf (signed, 32-bit)` | `[sp-116] <= 0xf (signed, 32-bit)` |
| `bge` at 0x72a04 | `[sp-116] >= 0xc (signed, 32-bit)` | `[sp-116] < 0xc (signed, 32-bit)` |
| `beq` at 0x72a10 | `[sp-116] == 0x6` | `[sp-116] != 0x6` |
| `bgt` at 0x72a1c | `[sp-116] > 0x6 (signed, 32-bit)` | `[sp-116] <= 0x6 (signed, 32-bit)` |
| `blt` at 0x72a28 | `[sp-116] < 0x1 (signed, 32-bit)` | `[sp-116] >= 0x1 (signed, 32-bit)` |
| `beq` at 0x72a38 | `[sp-116] == 0x9` | `[sp-116] != 0x9` |
| `blt` at 0x72a48 | `[sp-116] < 0x14 (signed, 32-bit)` | `[sp-116] >= 0x14 (signed, 32-bit)` |
| `ble` at 0x72a54 | `[sp-116] <= 0x19 (signed, 32-bit)` | `[sp-116] > 0x19 (signed, 32-bit)` |
| `beq` at 0x72a60 | `[sp-116] == 0x3e8` | `[sp-116] != 0x3e8` |
| `bls` at 0x72a90 | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x72aa4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x72ad8 | `[sp-144] == 0x0` | `[sp-144] != 0x0` |
| `beq` at 0x72af8 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0x72b24 | `ret(0xaf274) == 0x0` | `ret(0xaf274) != 0x0` |
| `beq` at 0x72b38 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `bls` at 0x72b58 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x72b6c | `? != 0x0` | `? == 0x0` |
| `bge` at 0x72bac | `[0x149640] >= [0x149738] (signed, 32-bit)` | `[0x149640] < [0x149738] (signed, 32-bit)` |
| `beq` at 0x72bbc | `[0x149644] == 0x0` | `[0x149644] != 0x0` |
| `beq` at 0x72bd8 | `[0x149648] == 0x0` | `[0x149648] != 0x0` |
| `bne` at 0x72c5c | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `blt` at 0x72c88 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x72cec | `ret(0x74f94) == 0x0` | `ret(0x74f94) != 0x0` |
| `beq` at 0x72ee8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x72f40 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `beq` at 0x72f68 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72fa0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72fd8 | `ret(0xb7164) == 0x0` | `ret(0xb7164) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x72ff0 | `r0` | `?` |

## `SysComThrottle`

- `SysComThrottle`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83128 to 0x83328.
  Established. The branch takes signed `ret(0x23b84) < [0x2b251c]` to `0x83254` (`blt` at 0x83198);
  its fall-through includes the unnamed `0x22ac8` call with `0x2b24bc`, unknown, `ret(0x23b84)`,
  `[0x2b251c]` (`bl` at 0x831a4). Included four-byte absolute stores at `0x2b2510` write `[sp-20]`
  and `0x0` (`str` at 0x831f8, `str` at 0x832c8). Throttle units and synchronization meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83138 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8313c | load | 4 | `abs 0x83310` | `-` |
| `ldr` at 0x83140 | load | 4 | `abs 0x14a4a8` | `-` |
| `ldr` at 0x83150 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x83154 | load | 4 | `abs 0x2b2510` | `-` |
| `ldr` at 0x83160 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x83164 | load | 4 | `abs 0x2b2510` | `-` |
| `ldr` at 0x83168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83178 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x83184 | store | 4 | `sp -0x1c` | `ret(0x23b84)` |
| `ldr` at 0x83188 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x8318c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83190 | load | 4 | `abs 0x2b251c` | `-` |
| `ldr` at 0x8319c | load | 4 | `abs 0x83318` | `-` |
| `str` at 0x831ac | store | 4 | `sp -0x18` | `ret(0x22ac8)` |
| `ldr` at 0x831b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x831bc | load | 4 | `abs 0x8331c` | `-` |
| `str` at 0x831c0 | store | 4 | `sp -0x24` | `0x112354` |
| `ldr` at 0x831c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x831c8 | store | 4 | `sp -0x20` | `ret(0x22ac8)` |
| `ldr` at 0x831d0 | load | 4 | `abs 0x83320` | `-` |
| `ldr` at 0x831e0 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x831e4 | load | 4 | `abs 0x2b2510` | `-` |
| `ldr` at 0x831f0 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x831f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x831f8 | store | 4 | `abs 0x2b2510` | `[sp-20]` |
| `ldr` at 0x831fc | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x83200 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x83204 | load | 4 | `abs 0x2b2514` | `-` |
| `str` at 0x8320c | store | 4 | `abs 0x2b2514` | `[0x2b2514]+1` |
| `ldr` at 0x83210 | load | 4 | `abs 0x83318` | `-` |
| `str` at 0x8321c | store | 4 | `sp -0x18` | `ret(0x22e50)` |
| `ldr` at 0x83220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8322c | load | 4 | `abs 0x83324` | `-` |
| `str` at 0x83230 | store | 4 | `sp -0x24` | `0x112398` |
| `ldr` at 0x83234 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x83238 | store | 4 | `sp -0x20` | `ret(0x22e50)` |
| `ldr` at 0x83240 | load | 4 | `abs 0x83320` | `-` |
| `ldr` at 0x83254 | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x83258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8325c | load | 4 | `abs 0x2b2520` | `-` |
| `ldr` at 0x83268 | load | 4 | `abs 0x83318` | `-` |
| `str` at 0x83278 | store | 4 | `sp -0x18` | `ret(0x22ac8)` |
| `ldr` at 0x8327c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x83288 | load | 4 | `abs 0x8331c` | `-` |
| `str` at 0x8328c | store | 4 | `sp -0x24` | `0x112354` |
| `ldr` at 0x83290 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x83294 | store | 4 | `sp -0x20` | `ret(0x22ac8)` |
| `ldr` at 0x8329c | load | 4 | `abs 0x83320` | `-` |
| `ldr` at 0x832ac | load | 4 | `abs 0x83314` | `-` |
| `ldr` at 0x832b0 | load | 4 | `abs 0x2b2510` | `-` |
| `ldr` at 0x832b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x832c0 | load | 4 | `abs 0x83314` | `-` |
| `str` at 0x832c8 | store | 4 | `abs 0x2b2510` | `0x0` |
| `ldr` at 0x832cc | load | 4 | `abs 0x83318` | `-` |
| `str` at 0x832d8 | store | 4 | `sp -0x18` | `ret(0x22e50)` |
| `ldr` at 0x832dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x832e8 | load | 4 | `abs 0x83324` | `-` |
| `str` at 0x832ec | store | 4 | `sp -0x24` | `0x112398` |
| `ldr` at 0x832f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x832f4 | store | 4 | `sp -0x20` | `ret(0x22e50)` |
| `ldr` at 0x832fc | load | 4 | `abs 0x83320` | `-` |
| `ldmdb` at 0x8330c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8330c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8330c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8317c | `(unknown callee) 0x23b84` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x831a4 | `(unknown callee) 0x22ac8` | `0x2b24bc` | `?` | `ret(0x23b84)` | `[0x2b251c]` |
| `bl` at 0x831dc | `(unknown callee) 0x21eb8` | `0x9` | `0x112380` | `0xa8` | `0x0` |
| `bl` at 0x83214 | `(unknown callee) 0x22e50` | `0x2b24bc` | `?` | `?` | `?` |
| `bl` at 0x8324c | `(unknown callee) 0x21eb8` | `0x9` | `0x112380` | `0xb0` | `0x0` |
| `bl` at 0x83270 | `(unknown callee) 0x22ac8` | `0x2b24bc` | `?` | `ret(0x23b84)` | `[0x2b2520]` |
| `bl` at 0x832a8 | `(unknown callee) 0x21eb8` | `0x9` | `0x112380` | `0xb5` | `0x0` |
| `bl` at 0x832d0 | `(unknown callee) 0x22e50` | `0x2b24bc` | `?` | `?` | `?` |
| `bl` at 0x83308 | `(unknown callee) 0x21eb8` | `0x9` | `0x112380` | `0xbb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x83148 | `[0x14a4a8] != 0x0` | `[0x14a4a8] == 0x0` |
| `beq` at 0x8315c | `[0x2b2510] == 0x0` | `[0x2b2510] != 0x0` |
| `beq` at 0x83170 | `[0x2b2510] == r0` | `[0x2b2510] != r0` |
| `blt` at 0x83198 | `ret(0x23b84) < [0x2b251c] (signed, 32-bit)` | `ret(0x23b84) >= [0x2b251c] (signed, 32-bit)` |
| `beq` at 0x831b8 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bne` at 0x831ec | `[0x2b2510] != 0x0` | `[0x2b2510] == 0x0` |
| `beq` at 0x83228 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `bge` at 0x83264 | `ret(0x23b84) >= [0x2b2520] (signed, 32-bit)` | `ret(0x23b84) < [0x2b2520] (signed, 32-bit)` |
| `beq` at 0x83284 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bne` at 0x832bc | `[0x2b2510] != [sp-20]` | `[0x2b2510] == [sp-20]` |
| `beq` at 0x832e4 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8330c | `r0` | `?` |

## `SysComInit`

- `SysComInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83b98 to 0x83fe4.
  Established. Included four-byte absolute stores write `0xc8` at `0x2b24b8`, `0x3` at `0x2b2518`,
  `0x50` at `0x2b251c` and `0x5` at `0x2b2520` (`str` at 0x83c14, `str` at 0x83c20, `str` at
  0x83c2c, `str` at 0x83c38). The unnamed `0xb5e34` call has `0x2b24a0`, `0x5`, `0x0`, `0x1` (`bl`
  at 0x83c54). A computed transfer leaves unresolved edges (`ldrls` at 0x83c7c). Setup fields and
  selector-specific effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x83ba8 | load | 4 | `abs 0x83f74` | `-` |
| `ldr` at 0x83bac | load | 4 | `abs 0x14a464` | `-` |
| `ldr` at 0x83bb8 | load | 4 | `abs 0x83f78` | `-` |
| `str` at 0x83bbc | store | 4 | `sp -0x34` | `0x11277c` |
| `ldr` at 0x83bc4 | load | 4 | `abs 0x83f7c` | `-` |
| `ldr` at 0x83bc8 | load | 4 | `abs 0x83f80` | `-` |
| `ldr` at 0x83bd4 | load | 4 | `abs 0x83f74` | `-` |
| `ldr` at 0x83bd8 | load | 4 | `abs 0x83f74` | `-` |
| `ldr` at 0x83bdc | load | 4 | `abs 0x14a464` | `-` |
| `str` at 0x83be4 | store | 4 | `abs 0x14a464` | `[0x14a464]+1` |
| `ldr` at 0x83be8 | load | 4 | `abs 0x83f84` | `-` |
| `ldr` at 0x83bf8 | load | 4 | `abs 0x83f88` | `-` |
| `ldr` at 0x83bfc | load | 4 | `abs 0x83f8c` | `-` |
| `ldr` at 0x83c0c | load | 4 | `abs 0x83f84` | `-` |
| `str` at 0x83c14 | store | 4 | `abs 0x2b24b8` | `0xc8` |
| `ldr` at 0x83c18 | load | 4 | `abs 0x83f84` | `-` |
| `str` at 0x83c20 | store | 4 | `abs 0x2b2518` | `0x3` |
| `ldr` at 0x83c24 | load | 4 | `abs 0x83f84` | `-` |
| `str` at 0x83c2c | store | 4 | `abs 0x2b251c` | `0x50` |
| `ldr` at 0x83c30 | load | 4 | `abs 0x83f84` | `-` |
| `str` at 0x83c38 | store | 4 | `abs 0x2b2520` | `0x5` |
| `ldr` at 0x83c3c | load | 4 | `abs 0x83f90` | `-` |
| `str` at 0x83c44 | store | 4 | `abs 0x14a4a8` | `0x1` |
| `ldr` at 0x83c48 | load | 4 | `abs 0x83f84` | `-` |
| `ldr` at 0x83c58 | load | 4 | `abs 0x83f84` | `-` |
| `str` at 0x83c64 | store | 4 | `abs 0x2b24a8` | `ret(0x75194)` |
| `ldr` at 0x83c68 | load | 4 | `abs 0x83f84` | `-` |
| `ldr` at 0x83c6c | load | 4 | `abs 0x2b24a8` | `-` |
| `ldr` at 0x83c70 | load | 4 | `[0x2b24a8] +0x10` | `-` |
| `ldrls` at 0x83c7c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x83f70 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x83f70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83f70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83f70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83bd0 | `(unknown callee) 0x21eb8` | `0x9` | `0x112380` | `0x1ca` | `0x0` |
| `bl` at 0x83bf4 | `(unknown callee) 0x1b4cc` | `0x2b24a0` | `0x0` | `0x84` | `[0x14a464]+1` |
| `bl` at 0x83c08 | `(unknown callee) 0x229b0` | `0x2b24bc` | `0x1127a0` | `0x1` | `0x9` |
| `bl` at 0x83c54 | `(unknown callee) 0xb5e34` | `0x2b24a0` | `0x5` | `0x0` | `0x1` |
| `bl` at 0x83c60 | `(unknown callee) 0x75194` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x83bb4 | `[0x14a464] == 0x0` | `[0x14a464] != 0x0` |
| `ldrls` at 0x83c7c | `[[0x2b24a8]+16]-43 <= 0x51 (unsigned, 32-bit)` | `[[0x2b24a8]+16]-43 > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83f70 | `r0` | `ret(0x75194)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x83c7c).

## `SysComSlab`

- `SysComSlab`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83fe4 to 0x8401c.
  Established. A four-byte absolute read targets `0x2b24a8`, followed by a four-byte read at
  `[0x2b24a8] +0x10` (`ldr` at 0x83ffc, `ldr` at 0x84000). The unnamed `0x87e18` call carries
  `[[0x2b24a8]+16]`, `r0`, `r2`, `[0x2b24a8]` (`bl` at 0x84008); its result is reported as
  `ret(0x87e18)` at the return (`ldmdb` at 0x84014). Slab identity and result meaning remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83ff4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x83ff8 | load | 4 | `abs 0x84018` | `-` |
| `ldr` at 0x83ffc | load | 4 | `abs 0x2b24a8` | `-` |
| `ldr` at 0x84000 | load | 4 | `[0x2b24a8] +0x10` | `-` |
| `ldr` at 0x84004 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x84014 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84014 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84008 | `(unknown callee) 0x87e18` | `[[0x2b24a8]+16]` | `r0` | `r2` | `[0x2b24a8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84014 | `r0` | `ret(0x87e18)` |

## `SysComSubchan`

- `SysComSubchan`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8401c to 0x84054.
  Established. The unnamed `0x87ebc` call has `[[0x2b24a8]+16]`, `r0`, `r2`, `[0x2b24a8]` (`bl` at
  0x84040). A prior four-byte stack assignment at `sp -0x14` retains entry `r0` (`str` at
  0x8402c). The return reports `ret(0x87ebc)` (`ldmdb` at 0x8404c). Subchannel interpretation
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8402c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x84030 | load | 4 | `abs 0x84050` | `-` |
| `ldr` at 0x84034 | load | 4 | `abs 0x2b24a8` | `-` |
| `ldr` at 0x84038 | load | 4 | `[0x2b24a8] +0x10` | `-` |
| `ldr` at 0x8403c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8404c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8404c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8404c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84040 | `(unknown callee) 0x87ebc` | `[[0x2b24a8]+16]` | `r0` | `r2` | `[0x2b24a8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8404c | `r0` | `ret(0x87ebc)` |

## `SysComTasknum`

- `SysComTasknum`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84054 to 0x84094.
  Established. The four-byte stack assignments at `sp -0x14` and `sp -0x18` preserve entry `r0` and
  `r1` (`str` at 0x84064, `str` at 0x84068). The unnamed `0x87fc0` call carries `[[0x2b24a8]+16]`,
  `r0`, `r1`, `[0x2b24a8]` (`bl` at 0x84080). Task mapping and candidate arity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84064 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x84068 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8406c | load | 4 | `abs 0x84090` | `-` |
| `ldr` at 0x84070 | load | 4 | `abs 0x2b24a8` | `-` |
| `ldr` at 0x84074 | load | 4 | `[0x2b24a8] +0x10` | `-` |
| `ldr` at 0x84078 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8407c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8408c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8408c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8408c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84080 | `(unknown callee) 0x87fc0` | `[[0x2b24a8]+16]` | `r0` | `r1` | `[0x2b24a8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8408c | `r0` | `ret(0x87fc0)` |

## `SysComEnable`

- `SysComEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84198 to 0x84234.
  Established. An included byte read has an unknown effective target (`ldrb` at 0x84204). The
  unnamed `0xd76d4` call uses `0x0`, unknown, `0x2`, unknown (`bl` at 0x84214). A later four-byte
  stack assignment at `sp -0x14` writes `[sp-20]+1` (`str` at 0x84220). Enable effects and the loop
  comparison operands remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x841ac | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x841b0 | load | 4 | `abs 0x8422c` | `-` |
| `ldr` at 0x841b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x841b8 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0x841c8 | load | 4 | `abs 0x84230` | `-` |
| `ldr` at 0x841cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x841d4 | load | 4 | `abs 0x8422c` | `-` |
| `ldr` at 0x841d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x841fc | load | 4 | `abs 0x2b24b4` | `-` |
| `ldrb` at 0x84204 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x84218 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x84220 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x84228 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84228 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84228 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x841d0 | `(unknown callee) 0x2079c` | `0x112814` | `?` | `?` | `[0x2b24b0]` |
| `bl` at 0x84214 | `(unknown callee) 0xd76d4` | `0x0` | `?` | `0x2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x841c0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84228 | `r0` | `?` |

## `syscom_process_inbound_data`

- `syscom_process_inbound_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84234 to 0x84374.
  Established. The unnamed `0x69908` call has `sp-80`, `sp-76`, `0x0`, `sp-80` (`bl` at 0x84264).
  Two included four-byte stores with unknown effective targets retain `[0x2b24a0]` and unknown
  respectively (`str` at 0x842c8, `str` at 0x84308). The unnamed `0xb764c` call carries `sp-44`,
  `0x0`, `[sp-80]`, `sp-76` (`bl` at 0x84330). Inbound data encoding and delivery effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84244 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x84248 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x8424c | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x84274 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x8427c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84280 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x84288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x842a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x842a4 | load | 4 | `abs 0x84368` | `-` |
| `ldr` at 0x842a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x842ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x842c0 | load | 4 | `abs 0x8436c` | `-` |
| `ldr` at 0x842c4 | load | 4 | `abs 0x2b24a0` | `-` |
| `str` at 0x842c8 | store | 4 | `(unknown)` | `[0x2b24a0]` |
| `ldr` at 0x842e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x842ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x84308 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x84314 | load | 4 | `abs 0x8436c` | `-` |
| `ldr` at 0x84318 | load | 4 | `abs 0x2b24b8` | `-` |
| `str` at 0x8431c | store | 4 | `sp -0x58` | `[0x2b24b8]` |
| `ldr` at 0x84328 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x84338 | store | 4 | `sp -0x54` | `ret(0xb764c)` |
| `ldr` at 0x8433c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x84348 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x84354 | load | 4 | `abs 0x84370` | `-` |
| `ldr` at 0x84358 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x84364 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x84364 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x84364 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84364 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84364 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84264 | `(unknown callee) 0x69908` | `sp-80` | `sp-76` | `0x0` | `sp-80` |
| `bl` at 0x84284 | `(unknown callee) 0x69b84` | `sp-80` | `sp-76` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x84290 | `(unknown callee) 0x82ebc` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0x842b0 | `(unknown callee) 0x82f54` | `[sp-28]` | `0x112824` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x842f4 | `(unknown callee) 0x84054` | `[sp-28]` | `0x4` | `sp-24` | `?` |
| `bl` at 0x84330 | `(unknown callee) 0xb764c` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x8434c | `(unknown callee) 0x21704` | `ret(0xb764c)` | `?` | `?` | `ret(0xb764c)` |
| `bl` at 0x84360 | `(unknown callee) 0x2079c` | `0x112834` | `[sp-28]` | `ret(0x21704)` | `ret(0x21704)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8429c | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `beq` at 0x84344 | `ret(0xb764c) == 0x0` | `ret(0xb764c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84364 | `r0` | `?` |

## `syscom_process_inbound_request`

- `syscom_process_inbound_request`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84374 to 0x84754.
  Established. Included byte reads target `[sp-32] +0x1`, `[sp-32] +0x2`, `[sp-32] +0x3` and
  `[sp-32] +0x4` (`ldrb` at 0x843d8, `ldrb` at 0x843ec, `ldrb` at 0x84404, `ldrb` at 0x8441c). The
  branch takes unsigned `[sp-140]-12289 > 0x2` to `0x8447c` (`bhi` at 0x84464). An indirect call has
  an unknown target and candidates `[sp-28]`, `[sp-144]`, `[sp-32]`, `[sp-36]` (`mov` at 0x8454c).
  Request fields and callback identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84384 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x84388 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x8438c | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x84394 | store | 4 | `sp -0x80` | `0x1` |
| `str` at 0x8439c | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x843a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x843b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x843bc | load | 4 | `abs 0x84744` | `-` |
| `ldr` at 0x843c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x843c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x843d0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x843d8 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x843e4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x843ec | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x843fc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84404 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x84414 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x8441c | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x84424 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x8442c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84434 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x84440 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84448 | load | 1 | `[sp-32] +0x6` | `-` |
| `str` at 0x84450 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x84454 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x84468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8446c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x84488 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84490 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x8449c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x844a4 | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x844c4 | load | 4 | `abs 0x84748` | `-` |
| `ldr` at 0x844c8 | load | 4 | `abs 0x2b24a0` | `-` |
| `str` at 0x844cc | store | 4 | `(unknown)` | `[0x2b24a0]` |
| `ldr` at 0x844d0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x844d4 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x844dc | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x844f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84500 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84504 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x8451c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x84520 | load | 4 | `abs 0x8474c` | `-` |
| `ldr` at 0x84524 | load | 4 | `abs 0x14a48c` | `-` |
| `ldr` at 0x84530 | load | 4 | `abs 0x8474c` | `-` |
| `ldr` at 0x84534 | load | 4 | `abs 0x14a48c` | `-` |
| `ldr` at 0x84538 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8453c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x84540 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84544 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x84550 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x84560 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84568 | load | 1 | `[sp-32] +0x7` | `-` |
| `str` at 0x8456c | store | 4 | `sp -0x94` | `[[sp-32]+7]:1` |
| `ldr` at 0x84570 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x84578 | store | 4 | `sp -0x20` | `[sp-32]+8` |
| `ldr` at 0x8457c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x84584 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `ldr` at 0x84594 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84598 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x8459c | store | 4 | `sp -0x98` | `[[sp-32]]:1` |
| `ldr` at 0x845a0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x845ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x845c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x845d0 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x845dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x845e4 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x845f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x845fc | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x8460c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x84614 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x84628 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x84630 | store | 4 | `sp -0x20` | `[sp-32]+5` |
| `ldr` at 0x84638 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84650 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8465c | store | 4 | `sp -0xa8` | `0x1` |
| `ldr` at 0x84670 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84680 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8468c | store | 4 | `sp -0x20` | `(ret(0x1b2bc) + [sp-32])+1` |
| `ldr` at 0x8469c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x846a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x846a8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x846bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x846c0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x846c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x846d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x846e0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x846e4 | store | 4 | `sp -0xa8` | `[sp-128]` |
| `str` at 0x846ec | store | 4 | `sp -0xa4` | `sp-124` |
| `str` at 0x846f4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x846fc | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x84714 | store | 4 | `sp -0x84` | `ret(0xb7394)` |
| `ldr` at 0x84718 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x84724 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x84730 | load | 4 | `abs 0x84750` | `-` |
| `ldr` at 0x84734 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x84740 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x84740 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x84740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x843a8 | `(unknown callee) 0x82ebc` | `r0` | `0x2` | `r2` | `0x0` |
| `bl` at 0x843c8 | `(unknown callee) 0x82f54` | `[sp-28]` | `0x11285c` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x84474 | `(unknown callee) 0x862e8` | `[sp-28]` | `[sp-32]` | `[sp-36]` | `[sp-140]-12289` |
| `bl` at 0x844b4 | `(unknown callee) 0x69908` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x84508 | `(unknown callee) 0x84054` | `[sp-28]` | `[sp-144]` | `sp-24` | `?` |
| `mov` at 0x8454c | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-144]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x84624 | `(unknown callee) 0x69964` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x8463c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0x8466c | `(unknown callee) 0x69b84` | `sp-128` | `sp-124` | `[sp-32]+1` | `ret(0x1b2bc)+1` |
| `bl` at 0x84674 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x846b8 | `(unknown callee) 0x69a08` | `sp-128` | `sp-124` | `[sp-32]+1` | `?` |
| `bl` at 0x8470c | `(unknown callee) 0xb7394` | `sp-44` | `sp-136` | `0x0` | `0x0` |
| `bl` at 0x84728 | `(unknown callee) 0x21704` | `ret(0xb7394)` | `?` | `?` | `ret(0xb7394)` |
| `bl` at 0x8473c | `(unknown callee) 0x2079c` | `0x11286c` | `[sp-28]` | `ret(0x21704)` | `ret(0x21704)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x843b4 | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `bhi` at 0x84464 | `[sp-140]-12289 > 0x2 (unsigned, 32-bit)` | `[sp-140]-12289 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x8452c | `[0x14a48c] == 0x0` | `[0x14a48c] != 0x0` |
| `bls` at 0x84558 | `[sp-36] <= 0x7 (unsigned, 32-bit)` | `[sp-36] > 0x7 (unsigned, 32-bit)` |
| `bne` at 0x8458c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x845a8 | `[[sp-32]]:1 == 0x0` | `[[sp-32]]:1 != 0x0` |
| `beq` at 0x845b4 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0x84720 | `ret(0xb7394) == 0x0` | `ret(0xb7394) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84740 | `r0` | `?` |

## `syscom_process_inbound_event`

- `syscom_process_inbound_event`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84754 to 0x84a7c.
  Established. An included four-byte absolute read targets `0x2af334` (`ldr` at 0x84774). The
  unnamed `0x69908` call carries `sp-80`, `sp-76`, `[[0x2b24a8]+60]`, `[0x2b24a8]` (`bl` at
  0x84848). A later indirect call has an unknown target and candidates `[sp-28]`, `[sp-32]+1`,
  `[sp-36]-1`, `[sp-36]-1` (`mov` at 0x849d4). Event framing and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84764 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x84768 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x8476c | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x84770 | load | 4 | `abs 0x84a68` | `-` |
| `ldr` at 0x84774 | load | 4 | `abs 0x2af334` | `-` |
| `str` at 0x84778 | store | 4 | `sp -0x74` | `[0x2af334]` |
| `ldr` at 0x8477c | load | 4 | `abs 0x84a68` | `-` |
| `ldr` at 0x84780 | load | 4 | `abs 0x2af334` | `-` |
| `str` at 0x84790 | store | 4 | `sp -0x74` | `0x1` |
| `ldr` at 0x84794 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x84798 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x8479c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x847a4 | store | 4 | `sp -0x68` | `r1+1` |
| `ldrb` at 0x847a8 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x847b0 | store | 1 | `sp -0x69` | `([sp-36]:1-1 & 0xff)` |
| `str` at 0x847b8 | store | 4 | `sp -0x70` | `0x0` |
| `ldrb` at 0x847bc | load | 1 | `sp -0x69` | `-` |
| `strb` at 0x847c4 | store | 1 | `sp -0x69` | `?` |
| `ldr` at 0x847d8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x847dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x847e8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x847f0 | load | 1 | `sp -0x69` | `-` |
| `ldr` at 0x847f4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x84800 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x84808 | store | 4 | `sp -0x70` | `[sp-112]+1` |
| `ldr` at 0x84810 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x84818 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x84820 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x84824 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x8482c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x84838 | load | 4 | `abs 0x84a6c` | `-` |
| `ldr` at 0x8483c | load | 4 | `abs 0x2b24a8` | `-` |
| `ldr` at 0x84844 | load | 4 | `[0x2b24a8] +0x3c` | `-` |
| `ldr` at 0x84854 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8485c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x84868 | store | 4 | `sp -0x78` | `0x1` |
| `ldr` at 0x8487c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84894 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84898 | load | 4 | `abs 0x84a70` | `-` |
| `ldr` at 0x8489c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x848a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x848a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x848ac | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x848b4 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x848b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x848c4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x848cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x848d0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x848dc | store | 4 | `sp -0x60` | `ret(0x84054)` |
| `str` at 0x848e4 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x848e8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x848ec | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x848fc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x84918 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x84928 | store | 4 | `sp -0x60` | `ret(0x84054)` |
| `ldr` at 0x84930 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x84938 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x84940 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x84954 | load | 4 | `abs 0x84a6c` | `-` |
| `ldr` at 0x84958 | load | 4 | `abs 0x2b24a0` | `-` |
| `str` at 0x8495c | store | 4 | `(unknown)` | `[0x2b24a0]` |
| `ldr` at 0x84978 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84980 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84984 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x8499c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x849a0 | load | 4 | `abs 0x84a74` | `-` |
| `ldr` at 0x849a4 | load | 4 | `abs 0x14a498` | `-` |
| `ldr` at 0x849b0 | load | 4 | `abs 0x84a74` | `-` |
| `ldr` at 0x849b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x849bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x849c4 | load | 4 | `abs 0x14a498` | `-` |
| `ldr` at 0x849c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x849f8 | load | 4 | `abs 0x84a6c` | `-` |
| `ldr` at 0x849fc | load | 4 | `abs 0x2b24b8` | `-` |
| `str` at 0x84a00 | store | 4 | `sp -0x78` | `[0x2b24b8]` |
| `ldr` at 0x84a0c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x84a1c | store | 4 | `sp -0x58` | `ret(0xb764c)` |
| `ldr` at 0x84a20 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x84a2c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x84a38 | load | 4 | `abs 0x84a78` | `-` |
| `ldr` at 0x84a3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84a48 | load | 4 | `abs 0x84a6c` | `-` |
| `ldr` at 0x84a4c | load | 4 | `abs 0x2b2510` | `-` |
| `ldr` at 0x84a58 | load | 4 | `abs 0x84a6c` | `-` |
| `ldr` at 0x84a5c | load | 4 | `abs 0x2b2518` | `-` |
| `ldmdb` at 0x84a64 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x84a64 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x84a64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84a64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84a64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x847fc | `(unknown callee) 0x1b394` | `?` | `?` | `?` | `?` |
| `bl` at 0x84848 | `(unknown callee) 0x69908` | `sp-80` | `sp-76` | `[[0x2b24a8]+60]` | `[0x2b24a8]` |
| `bl` at 0x84878 | `(unknown callee) 0x69b84` | `sp-80` | `sp-76` | `[sp-32]+1` | `[sp-36]-1` |
| `bl` at 0x84884 | `(unknown callee) 0x82ebc` | `[sp-28]` | `0x4` | `?` | `?` |
| `bl` at 0x848a4 | `(unknown callee) 0x82f54` | `[sp-28]` | `0x112898` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x848c8 | `(unknown callee) 0x5f160` | `[sp-28]+2` | `[sp-92]` | `?` | `[sp-28]+2` |
| `bl` at 0x848d4 | `(unknown callee) 0x84054` | `[sp-28]` | `[sp-92]` | `?` | `?` |
| `bl` at 0x84908 | `(unknown callee) 0x5f314` | `?` | `?` | `?` | `?` |
| `bl` at 0x84920 | `(unknown callee) 0x84054` | `[sp-84]` | `0x4` | `?` | `ret(0x5f314)` |
| `bl` at 0x84944 | `(unknown callee) 0x685bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x84988 | `(unknown callee) 0x84054` | `[sp-28]` | `[sp-92]` | `sp-24` | `?` |
| `mov` at 0x849d4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-32]+1` | `[sp-36]-1` | `[sp-36]-1` |
| `bl` at 0x849dc | `(unknown callee) 0x1f26c` | `0x81` | `?` | `?` | `?` |
| `bl` at 0x84a14 | `(unknown callee) 0xb764c` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x84a30 | `(unknown callee) 0x21704` | `ret(0xb764c)` | `?` | `?` | `ret(0xb764c)` |
| `bl` at 0x84a44 | `(unknown callee) 0x2079c` | `0x1128a8` | `[sp-28]` | `ret(0x21704)` | `ret(0x21704)` |
| `bl` at 0x84a60 | `(unknown callee) 0x87df0` | `[0x2b2518]` | `?` | `?` | `0x2b24a0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x84788 | `[0x2af334] != 0x0` | `[0x2af334] == 0x0` |
| `bne` at 0x847d0 | `? != 0xff` | `? == 0xff` |
| `bne` at 0x847e4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x84890 | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `blt` at 0x848f4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x84914 | `ret(0x5f314) != 0x4` | `ret(0x5f314) == 0x4` |
| `beq` at 0x849ac | `[0x14a498] == 0x0` | `[0x14a498] != 0x0` |
| `beq` at 0x849e8 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x84a28 | `ret(0xb764c) == 0x0` | `ret(0xb764c) != 0x0` |
| `beq` at 0x84a54 | `[0x2b2510] == 0x0` | `[0x2b2510] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84a64 | `r0` | `?` |

## `syscom_process_inbound_packet`

- `syscom_process_inbound_packet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84a7c to 0x84b58.
  Established. The branch takes `[sp-32] == 0x0` to `0x84ac8` (`beq` at 0x84ab4). That arm includes
  four-byte absolute stores of `[0x14a470]+1` at `0x14a470` and `([0x14a47c] + [sp-28])` at
  `0x14a47c` (`str` at 0x84ad8, `str` at 0x84af0). A separate branch takes `[sp-32] == 0x2` to
  `0x84b08` (`beq` at 0x84ac0). Packet type meanings and accumulator units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84a8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x84a90 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x84a94 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x84a98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x84a9c | load | 1 | `r1 +0x0` | `-` |
| `str` at 0x84aa8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x84aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84ab8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84ac8 | load | 4 | `abs 0x84b48` | `-` |
| `ldr` at 0x84acc | load | 4 | `abs 0x84b48` | `-` |
| `ldr` at 0x84ad0 | load | 4 | `abs 0x14a470` | `-` |
| `str` at 0x84ad8 | store | 4 | `abs 0x14a470` | `[0x14a470]+1` |
| `ldr` at 0x84adc | load | 4 | `abs 0x84b4c` | `-` |
| `ldr` at 0x84ae0 | load | 4 | `abs 0x84b4c` | `-` |
| `ldr` at 0x84ae4 | load | 4 | `abs 0x14a47c` | `-` |
| `ldr` at 0x84ae8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x84af0 | store | 4 | `abs 0x14a47c` | `([0x14a47c] + [sp-28])` |
| `ldr` at 0x84af4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84af8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x84afc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84b08 | load | 4 | `abs 0x84b50` | `-` |
| `ldr` at 0x84b0c | load | 4 | `abs 0x84b50` | `-` |
| `ldr` at 0x84b10 | load | 4 | `abs 0x14a478` | `-` |
| `str` at 0x84b18 | store | 4 | `abs 0x14a478` | `[0x14a478]+1` |
| `ldr` at 0x84b1c | load | 4 | `abs 0x84b54` | `-` |
| `ldr` at 0x84b20 | load | 4 | `abs 0x84b54` | `-` |
| `ldr` at 0x84b24 | load | 4 | `abs 0x14a484` | `-` |
| `ldr` at 0x84b28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x84b30 | store | 4 | `abs 0x14a484` | `([0x14a484] + [sp-28])` |
| `ldr` at 0x84b34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84b38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x84b3c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x84b44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84b44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84b44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84b00 | `(unknown callee) 0x84374` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x14a47c] + [sp-28])` |
| `bl` at 0x84b40 | `(unknown callee) 0x84754` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x14a484] + [sp-28])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x84ab4 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x84ac0 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84b44 | `r0` | `?` |

## `SysComEventEnabled`

- `SysComEventEnabled`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8627c to 0x862bc.
  Established. A four-byte absolute read targets `0x2b24ac` (`ldr` at 0x86290). The branch
  takes `Z == 0` to `0x862a8` and falls through for `not (Z == 0)` (`bne` at 0x86298). The two arms include
  four-byte stack assignments at `sp -0x14` of `0x1` and `0x0` respectively (`str` at 0x862ac, `str`
  at 0x862a0). Event-enabled interpretation and flag operands remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8628c | load | 4 | `abs 0x862b8` | `-` |
| `ldr` at 0x86290 | load | 4 | `abs 0x2b24ac` | `-` |
| `str` at 0x862a0 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x862ac | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x862b0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x862b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x862b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x862b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x86298 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x862b4 | `r0` | `[sp-20]` |
