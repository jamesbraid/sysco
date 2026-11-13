# ARM configuration events and network setup in 1.40.5

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
These routines have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/configuration-event-setup-facts.txt) retain all tool sections.

## `L1NetInit`

- `L1NetInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65df0 to 0x65f84.
  Established. An absolute four-byte assignment at `0x149280` writes `[0x149280]+1` (`str` at
  0x65e3c). The unnamed `0xb5f4c` call carries `0x2b16f0`, `0x10dd48`, `0x64`, unknown (`bl` at
  0x65e6c). Two unnamed `0x22480` calls have `0x2b17e0`, `0x10dd84`, `0x66588`, `0x0`, and
  `0x2b17dc`, `0x10dd90`, `0x65fa4`, `0x0` (`bl` at 0x65efc, `bl` at 0x65f38). Network task and
  endpoint meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65e00 | load | 4 | `abs 0x65f40` | `-` |
| `ldr` at 0x65e04 | load | 4 | `abs 0x149280` | `-` |
| `ldr` at 0x65e10 | load | 4 | `abs 0x65f44` | `-` |
| `str` at 0x65e14 | store | 4 | `sp -0x30` | `0x10dd24` |
| `ldr` at 0x65e1c | load | 4 | `abs 0x65f48` | `-` |
| `ldr` at 0x65e2c | load | 4 | `abs 0x65f40` | `-` |
| `ldr` at 0x65e30 | load | 4 | `abs 0x65f40` | `-` |
| `ldr` at 0x65e34 | load | 4 | `abs 0x149280` | `-` |
| `str` at 0x65e3c | store | 4 | `abs 0x149280` | `[0x149280]+1` |
| `ldr` at 0x65e40 | load | 4 | `abs 0x65f4c` | `-` |
| `ldr` at 0x65e50 | load | 4 | `abs 0x65f4c` | `-` |
| `ldr` at 0x65e60 | load | 4 | `abs 0x65f50` | `-` |
| `ldr` at 0x65e64 | load | 4 | `abs 0x65f54` | `-` |
| `str` at 0x65e74 | store | 4 | `sp -0x14` | `ret(0xb5f4c)` |
| `ldr` at 0x65e78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65e90 | load | 4 | `abs 0x65f58` | `-` |
| `str` at 0x65e94 | store | 4 | `sp -0x30` | `0x10dd54` |
| `str` at 0x65e98 | store | 4 | `sp -0x2c` | `ret(0x21704)` |
| `ldr` at 0x65ea0 | load | 4 | `abs 0x65f48` | `-` |
| `ldr` at 0x65eb0 | load | 4 | `abs 0x65f5c` | `-` |
| `ldr` at 0x65eb4 | load | 4 | `abs 0x65f60` | `-` |
| `str` at 0x65ec8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x65ecc | load | 4 | `abs 0x65f64` | `-` |
| `str` at 0x65ed0 | store | 4 | `sp -0x2c` | `0x5dc` |
| `str` at 0x65ed8 | store | 4 | `sp -0x28` | `0xf` |
| `str` at 0x65ee0 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x65ee8 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x65eec | load | 4 | `abs 0x65f68` | `-` |
| `ldr` at 0x65ef0 | load | 4 | `abs 0x65f6c` | `-` |
| `ldr` at 0x65ef4 | load | 4 | `abs 0x65f70` | `-` |
| `str` at 0x65f04 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x65f08 | load | 4 | `abs 0x65f74` | `-` |
| `str` at 0x65f0c | store | 4 | `sp -0x2c` | `0xc18` |
| `str` at 0x65f14 | store | 4 | `sp -0x28` | `0xf` |
| `str` at 0x65f1c | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x65f24 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x65f28 | load | 4 | `abs 0x65f78` | `-` |
| `ldr` at 0x65f2c | load | 4 | `abs 0x65f7c` | `-` |
| `ldr` at 0x65f30 | load | 4 | `abs 0x65f80` | `-` |
| `ldmdb` at 0x65f3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65f3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65f3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65e28 | `(unknown callee) 0x21eb8` | `0x9` | `0x10dd40` | `0x43` | `0x0` |
| `bl` at 0x65e4c | `(unknown callee) 0x1b4cc` | `0x2b16e0` | `0x0` | `0x104` | `[0x149280]+1` |
| `bl` at 0x65e5c | `(unknown callee) 0xb5e34` | `0x2b16e0` | `0x11` | `0x0` | `?` |
| `bl` at 0x65e6c | `(unknown callee) 0xb5f4c` | `0x2b16f0` | `0x10dd48` | `0x64` | `?` |
| `bl` at 0x65e88 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0x65eac | `(unknown callee) 0x21eb8` | `0x9` | `0x10dd40` | `0x54` | `0x0` |
| `bl` at 0x65ec0 | `(unknown callee) 0x229b0` | `0x2b1788` | `0x10dd78` | `0x1` | `0x9` |
| `bl` at 0x65efc | `(unknown callee) 0x22480` | `0x2b17e0` | `0x10dd84` | `0x66588` | `0x0` |
| `bl` at 0x65f38 | `(unknown callee) 0x22480` | `0x2b17dc` | `0x10dd90` | `0x65fa4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x65e0c | `[0x149280] == 0x0` | `[0x149280] != 0x0` |
| `beq` at 0x65e80 | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65f3c | `r0` | `ret(0x22480)` |

## `L1NetActive`

- `L1NetActive`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65f84 to 0x65fa4.
  Established. The byte read at absolute `0x2b16e8` supplies the printed return `[0x2b16e8]:1`
  (`ldrb` at 0x65f94, `ldmdb` at 0x65f9c). Activity encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65f90 | load | 4 | `abs 0x65fa0` | `-` |
| `ldrb` at 0x65f94 | load | 1 | `abs 0x2b16e8` | `-` |
| `ldmdb` at 0x65f9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65f9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65f9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65f9c | `r0` | `[0x2b16e8]:1` |

## `L1CfgPortEnable`

- `L1CfgPortEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f27c to 0x6f354.
  Established. Two four-byte stores with unknown effective targets write `0x1` and `0x0` (`str` at
  0x6f2ec, `str` at 0x6f31c). A later four-byte absolute assignment at `0x1493dc` retains
  `[0x1493dc]+1` (`str` at 0x6f330). The unnamed `0x6c9f4` call has unknown, `0x1496ac`, `0x1493dc`,
  `[0x1493dc]+1` (`bl` at 0x6f334). Port-state effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f28c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6f290 | load | 4 | `abs 0x6f33c` | `-` |
| `ldr` at 0x6f298 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f29c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6f2ac | load | 4 | `abs 0x6f344` | `-` |
| `ldr` at 0x6f2b0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6f2b4 | load | 4 | `abs 0x6f348` | `-` |
| `ldr` at 0x6f2b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f2c0 | load | 4 | `abs 0x6f34c` | `-` |
| `ldr` at 0x6f2c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f2e0 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6f2ec | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6f2f0 | load | 4 | `abs 0x6f34c` | `-` |
| `ldr` at 0x6f2f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f310 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6f31c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6f320 | load | 4 | `abs 0x6f350` | `-` |
| `ldr` at 0x6f324 | load | 4 | `abs 0x6f350` | `-` |
| `ldr` at 0x6f328 | load | 4 | `abs 0x1493dc` | `-` |
| `str` at 0x6f330 | store | 4 | `abs 0x1493dc` | `[0x1493dc]+1` |
| `ldmdb` at 0x6f338 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f338 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f338 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6f294 | `(unknown callee) 0x6c95c` | `0x110168` | `r1` | `r2` | `r3` |
| `bl` at 0x6f2bc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110178` | `[sp-20]` | `0x145ae8` |
| `bl` at 0x6f334 | `(unknown callee) 0x6c9f4` | `?` | `0x1496ac` | `0x1493dc` | `[0x1493dc]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6f2a8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f338 | `r0` | `ret(0x6c9f4)` |

## `L1CfgBrickCount`

- `L1CfgBrickCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75404 to 0x75424.
  Established. An absolute four-byte read at `0x149738` supplies the printed return `[0x149738]`
  (`ldr` at 0x75414, `ldmdb` at 0x7541c). Count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x75410 | load | 4 | `abs 0x75420` | `-` |
| `ldr` at 0x75414 | load | 4 | `abs 0x149738` | `-` |
| `ldmdb` at 0x7541c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7541c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7541c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7541c | `r0` | `[0x149738]` |

## `L1CfgTypeNonRoutered`

- `L1CfgTypeNonRoutered`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75424 to 0x75494.
  Established. Three branches take `r0 == 0x44`, `r0 == 0x40` and `r0 == 0x53` to `0x75478` (`beq`
  at 0x75448, `beq` at 0x75460, `beq` at 0x75470). Separate four-byte stack assignments
  at `sp -0x18` write `0x0` and `0x1` (`str` at 0x7547c, `str` at 0x75488). Type identifiers and routing
  meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75434 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x75438 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7543c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x75440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7544c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75458 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75468 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7547c | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x75488 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x7548c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x75490 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75490 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75490 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75448 | `r0 == 0x44` | `r0 != 0x44` |
| `bgt` at 0x75454 | `r0 > 0x44 (signed, 32-bit)` | `r0 <= 0x44 (signed, 32-bit)` |
| `beq` at 0x75460 | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0x75470 | `r0 == 0x53` | `r0 != 0x53` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75490 | `r0` | `?` |

## `L1CfgPortNonRoutered`

- `L1CfgPortNonRoutered`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75494 to 0x75504.
  Established. The branch takes `r0 >= [0x149730]`, signed 32-bit, to `0x754f0` (`bge` at 0x754b8);
  its taken arm includes four-byte `0x0` at `sp -0x18` (`str` at 0x754f4). On fall-through, a
  four-byte read has unknown effective target, followed by an unknown-valued four-byte assignment at
  `sp -0x18` (`ldr` at 0x754e4, `str` at 0x754e8). Port classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x754a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x754a8 | load | 4 | `abs 0x75500` | `-` |
| `ldr` at 0x754ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x754b0 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x754bc | load | 4 | `abs 0x75500` | `-` |
| `ldr` at 0x754c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x754dc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x754e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x754e8 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x754f4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x754f8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x754fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x754fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x754fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x754b8 | `r0 >= [0x149730] (signed, 32-bit)` | `r0 < [0x149730] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x754fc | `r0` | `?` |

## `L1CfgEventSystemConsole`

- `L1CfgEventSystemConsole`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75790 to 0x757b0.
  Established. The unnamed `0x75504` call carries `0x1`, `0x0`, `0x0`, `r3` (`bl` at 0x757a8); its
  result is the printed `r0` return (`ldmdb` at 0x757ac). System-console notification semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x757ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x757ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x757ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x757a8 | `(unknown callee) 0x75504` | `0x1` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x757ac | `r0` | `ret(0x75504)` |

## `L1CfgEventConsoleSubchannel`

- `L1CfgEventConsoleSubchannel`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x757b0 to 0x757d0.
  Established. The call to unnamed `0x75504` uses `0x2`, `0x0`, `0x0`, `r3` (`bl` at 0x757c8). The
  exit retains `ret(0x75504)` (`ldmdb` at 0x757cc). Console-subchannel event meaning remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x757cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x757cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x757cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x757c8 | `(unknown callee) 0x75504` | `0x2` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x757cc | `r0` | `ret(0x75504)` |

## `L1CfgEventConsoleEnable`

- `L1CfgEventConsoleEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x757d0 to 0x757f0.
  Established. Candidates for the unnamed `0x75504` call are `0x3`, `0x0`, `0x0`, `r3` (`bl` at
  0x757e8). The printed return follows that result (`ldmdb` at 0x757ec). Console-enable encoding
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x757ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x757ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x757ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x757e8 | `(unknown callee) 0x75504` | `0x3` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x757ec | `r0` | `ret(0x75504)` |

## `L1CfgEventMarginState`

- `L1CfgEventMarginState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75848 to 0x758a0.
  Established. The branch takes `[sp-20] == [ret(0x75194)+56]` to `0x7589c` (`beq` at 0x7587c).
  Fall-through includes a four-byte assignment at `ret(0x75194) +0x38` with `[sp-20]` (`str` at
  0x75888). The unnamed `0x75504` call has `0x5`, `0x0`, `0x0`, `[sp-20]` (`bl` at 0x75898).
  Margin-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75858 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75868 | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `ldr` at 0x7586c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75874 | load | 4 | `ret(0x75194) +0x38` | `-` |
| `ldr` at 0x75880 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75884 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75888 | store | 4 | `ret(0x75194) +0x38` | `[sp-20]` |
| `ldmdb` at 0x7589c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7589c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7589c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75860 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x75898 | `(unknown callee) 0x75504` | `0x5` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7587c | `[sp-20] == [ret(0x75194)+56]` | `[sp-20] != [ret(0x75194)+56]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7589c | `r0` | `?` |

## `L1CfgEventPartition`

- `L1CfgEventPartition`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x758a0 to 0x75a50.
  Established. The branch takes `[sp-20] == [ret(0x75194)+60]` to `0x75a4c` (`beq` at 0x758d4).
  Fall-through assigns four-byte `[sp-20]` at `ret(0x75194) +0x3c` (`str` at 0x758e0). Later
  included unnamed `0x72734` calls have first candidates `0x1`, `0x3`, `0xe` and `0xf` (`bl` at
  0x75a30, `bl` at 0x75a38, `bl` at 0x75a40, `bl` at 0x75a48). Partition and request meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x758b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x758c0 | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `ldr` at 0x758c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x758c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x758cc | load | 4 | `ret(0x75194) +0x3c` | `-` |
| `ldr` at 0x758d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x758dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x758e0 | store | 4 | `ret(0x75194) +0x3c` | `[sp-20]` |
| `ldr` at 0x758f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x758fc | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7590c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75918 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7591c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7592c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7593c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75948 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7594c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75958 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7595c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75968 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7596c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75978 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7597c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7598c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75998 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7599c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x759a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x759ac | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x759b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x759bc | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x759c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x759cc | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x759d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x759dc | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x759e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x759ec | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x759f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x759fc | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75a08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75a0c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x75a18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75a1c | load | 4 | `[sp-24] +0x10` | `-` |
| `ldmdb` at 0x75a4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75a4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75a4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x758b8 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x758e4 | `(unknown callee) 0x6a384` | `ret(0x75194)` | `?` | `ret(0x75194)` | `[sp-20]` |
| `bl` at 0x758f4 | `(unknown callee) 0x75504` | `0x6` | `0x0` | `0x0` | `?` |
| `bl` at 0x75a30 | `(unknown callee) 0x72734` | `0x1` | `?` | `?` | `[[sp-24]+16]` |
| `bl` at 0x75a38 | `(unknown callee) 0x72734` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x75a40 | `(unknown callee) 0x72734` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x75a48 | `(unknown callee) 0x72734` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x758d4 | `[sp-20] == [ret(0x75194)+60]` | `[sp-20] != [ret(0x75194)+60]` |
| `beq` at 0x75904 | `[[sp-24]+16] == 0x43` | `[[sp-24]+16] != 0x43` |
| `beq` at 0x75914 | `[[sp-24]+16] == 0x33` | `[[sp-24]+16] != 0x33` |
| `beq` at 0x75924 | `[[sp-24]+16] == 0x35` | `[[sp-24]+16] != 0x35` |
| `beq` at 0x75934 | `[[sp-24]+16] == 0x34` | `[[sp-24]+16] != 0x34` |
| `beq` at 0x75944 | `[[sp-24]+16] == 0x41` | `[[sp-24]+16] != 0x41` |
| `beq` at 0x75954 | `[[sp-24]+16] == 0x36` | `[[sp-24]+16] != 0x36` |
| `beq` at 0x75964 | `[[sp-24]+16] == 0x39` | `[[sp-24]+16] != 0x39` |
| `beq` at 0x75974 | `[[sp-24]+16] == 0x37` | `[[sp-24]+16] != 0x37` |
| `beq` at 0x75984 | `[[sp-24]+16] == 0x38` | `[[sp-24]+16] != 0x38` |
| `beq` at 0x75994 | `[[sp-24]+16] == 0x30` | `[[sp-24]+16] != 0x30` |
| `beq` at 0x759a4 | `[[sp-24]+16] == 0x51` | `[[sp-24]+16] != 0x51` |
| `beq` at 0x759b4 | `[[sp-24]+16] == 0x40` | `[[sp-24]+16] != 0x40` |
| `beq` at 0x759c4 | `[[sp-24]+16] == 0x4f` | `[[sp-24]+16] != 0x4f` |
| `beq` at 0x759d4 | `[[sp-24]+16] == 0x42` | `[[sp-24]+16] != 0x42` |
| `beq` at 0x759e4 | `[[sp-24]+16] == 0x46` | `[[sp-24]+16] != 0x46` |
| `beq` at 0x759f4 | `[[sp-24]+16] == 0x48` | `[[sp-24]+16] != 0x48` |
| `beq` at 0x75a04 | `[[sp-24]+16] == 0x4c` | `[[sp-24]+16] != 0x4c` |
| `beq` at 0x75a14 | `[[sp-24]+16] == 0x4d` | `[[sp-24]+16] != 0x4d` |
| `beq` at 0x75a24 | `[[sp-24]+16] == 0x7c` | `[[sp-24]+16] != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75a4c | `r0` | `?` |

## `L1CfgEventSystemSN`

- `L1CfgEventSystemSN`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75a50 to 0x75afc.
  Established. Two unnamed `0x1b598` calls carry `[sp-24]+76`, `ret(0x1f26c)`, `ret(0x1f26c)`,
  `[sp-24]+76`, and `[sp-24]+100`, `ret(0x1f26c)`, `[sp-24]+100`, `ret(0x1f26c)` (`bl` at 0x75a98,
  `bl` at 0x75ac4). A zero `[sp-20]` takes the branch past the second call to `0x75ac8` (`beq` at
  0x75aa4). A later unnamed `0x6bb3c` call has `ret(0x75504)`, unknown, unknown, unknown (`bl` at
  0x75af0). Serial-number representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75a60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75a70 | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `str` at 0x75a78 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x75a88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75a9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75ab4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75ad4 | load | 4 | `abs 0x75af8` | `-` |
| `ldmdb` at 0x75af4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75af4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75af4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75a68 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x75a80 | `(unknown callee) 0x1f26c` | `0xd` | `?` | `?` | `0x0` |
| `bl` at 0x75a98 | `(unknown callee) 0x1b598` | `[sp-24]+76` | `ret(0x1f26c)` | `ret(0x1f26c)` | `[sp-24]+76` |
| `bl` at 0x75aac | `(unknown callee) 0x1f26c` | `0xd` | `?` | `?` | `[sp-20]` |
| `bl` at 0x75ac4 | `(unknown callee) 0x1b598` | `[sp-24]+100` | `ret(0x1f26c)` | `[sp-24]+100` | `ret(0x1f26c)` |
| `bl` at 0x75acc | `(unknown callee) 0x1f26c` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x75adc | `(unknown callee) 0x1b598` | `0x2b28c8` | `ret(0x1f26c)` | `?` | `ret(0x1f26c)` |
| `bl` at 0x75aec | `(unknown callee) 0x75504` | `0x7` | `0x0` | `0x0` | `?` |
| `bl` at 0x75af0 | `(unknown callee) 0x6bb3c` | `ret(0x75504)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75aa4 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75af4 | `r0` | `ret(0x6bb3c)` |

## `L1CfgEventEnvironmental`

- `L1CfgEventEnvironmental`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75afc to 0x75b54.
  Established. A four-byte read targets `ret(0x75194) +0x5c` (`ldr` at 0x75b28). The branch takes
  `[sp-20] == [ret(0x75194)+92]` to `0x75b50` (`beq` at 0x75b30). Fall-through includes four-byte
  `[sp-20]` at `ret(0x75194) +0x5c` and the unnamed `0x75504` call with `0x8`, `0x0`, `0x0`,
  `[sp-20]` (`str` at 0x75b3c, `bl` at 0x75b4c). Environmental-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75b0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75b1c | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `ldr` at 0x75b20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75b28 | load | 4 | `ret(0x75194) +0x5c` | `-` |
| `ldr` at 0x75b34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75b38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75b3c | store | 4 | `ret(0x75194) +0x5c` | `[sp-20]` |
| `ldmdb` at 0x75b50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75b50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75b50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75b14 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x75b4c | `(unknown callee) 0x75504` | `0x8` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75b30 | `[sp-20] == [ret(0x75194)+92]` | `[sp-20] != [ret(0x75194)+92]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75b50 | `r0` | `?` |

## `L1ReqInit`

- `L1ReqInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x88a60 to 0x88b50.
  Established. The unnamed `0xb4a70` call carries `0x2b2594`, `0xffff006`, `0x1`, `0x88b50` (`bl` at
  0x88ad0); its zero result takes the branch to `0x88b0c` (`beq` at 0x88ae4). A later four-byte
  absolute store at `0x14a548` writes `[0x14a548]+1` (`str` at 0x88b1c). Request-registration
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x88a70 | load | 4 | `abs 0x88b24` | `-` |
| `ldr` at 0x88a74 | load | 4 | `abs 0x14a548` | `-` |
| `ldr` at 0x88a80 | load | 4 | `abs 0x88b28` | `-` |
| `str` at 0x88a84 | store | 4 | `sp -0x1c` | `0x112bd4` |
| `ldr` at 0x88a8c | load | 4 | `abs 0x88b2c` | `-` |
| `ldr` at 0x88a90 | load | 4 | `abs 0x88b30` | `-` |
| `ldr` at 0x88a9c | load | 4 | `abs 0x88b34` | `-` |
| `ldr` at 0x88aa0 | load | 4 | `abs 0x88b38` | `-` |
| `ldr` at 0x88ab0 | load | 4 | `abs 0x88b3c` | `-` |
| `ldr` at 0x88ac0 | load | 4 | `abs 0x88b3c` | `-` |
| `ldr` at 0x88ac4 | load | 4 | `abs 0x88b40` | `-` |
| `ldr` at 0x88acc | load | 4 | `abs 0x88b44` | `-` |
| `str` at 0x88ad8 | store | 4 | `sp -0x14` | `ret(0xb4a70)` |
| `ldr` at 0x88adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x88ae8 | load | 4 | `abs 0x88b48` | `-` |
| `str` at 0x88aec | store | 4 | `sp -0x1c` | `0x112c14` |
| `ldr` at 0x88af0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x88af4 | store | 4 | `sp -0x18` | `ret(0xb4a70)` |
| `ldr` at 0x88afc | load | 4 | `abs 0x88b2c` | `-` |
| `ldr` at 0x88b00 | load | 4 | `abs 0x88b4c` | `-` |
| `ldr` at 0x88b0c | load | 4 | `abs 0x88b24` | `-` |
| `ldr` at 0x88b10 | load | 4 | `abs 0x88b24` | `-` |
| `ldr` at 0x88b14 | load | 4 | `abs 0x14a548` | `-` |
| `str` at 0x88b1c | store | 4 | `abs 0x14a548` | `[0x14a548]+1` |
| `ldmdb` at 0x88b20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x88b20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x88b20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x88a98 | `(unknown callee) 0x21eb8` | `0x9` | `0x112bf0` | `0x1c6` | `0x0` |
| `bl` at 0x88aac | `(unknown callee) 0x229b0` | `0x2b2540` | `0x112c08` | `0x1` | `0x9` |
| `bl` at 0x88abc | `(unknown callee) 0xb5e34` | `0x2b2594` | `0x6` | `0x0` | `?` |
| `bl` at 0x88ad0 | `(unknown callee) 0xb4a70` | `0x2b2594` | `0xffff006` | `0x1` | `0x88b50` |
| `bl` at 0x88b08 | `(unknown callee) 0x21eb8` | `0x9` | `0x112bf0` | `0x1da` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x88a7c | `[0x14a548] == 0x0` | `[0x14a548] != 0x0` |
| `beq` at 0x88ae4 | `ret(0xb4a70) == 0x0` | `ret(0xb4a70) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x88b20 | `r0` | `?` |

## `EnvTestTemp`

- `EnvTestTemp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82bd8 to 0x82c60.
  Established. Included four-byte absolute assignments write `0x1` at `0x149b84`, `[sp-20]` at
  `0x149b88` and `[sp]` at `0x149b98` (`str` at 0x82c00, `str` at 0x82c0c, `str` at 0x82c3c). The
  unnamed `0x829f0` call carries `r0`, `r1`, `0x149b98`, `[sp]` (`bl` at 0x82c40). Temperature-test
  parameters and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82be8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x82bec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x82bf0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x82bf4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x82bf8 | load | 4 | `abs 0x82c48` | `-` |
| `str` at 0x82c00 | store | 4 | `abs 0x149b84` | `0x1` |
| `ldr` at 0x82c04 | load | 4 | `abs 0x82c4c` | `-` |
| `ldr` at 0x82c08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x82c0c | store | 4 | `abs 0x149b88` | `[sp-20]` |
| `ldr` at 0x82c10 | load | 4 | `abs 0x82c50` | `-` |
| `ldr` at 0x82c14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82c18 | store | 4 | `abs 0x149b8c` | `[sp-24]` |
| `ldr` at 0x82c1c | load | 4 | `abs 0x82c54` | `-` |
| `ldr` at 0x82c20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x82c24 | store | 4 | `abs 0x149b90` | `[sp-28]` |
| `ldr` at 0x82c28 | load | 4 | `abs 0x82c58` | `-` |
| `ldr` at 0x82c2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x82c30 | store | 4 | `abs 0x149b94` | `[sp-32]` |
| `ldr` at 0x82c34 | load | 4 | `abs 0x82c5c` | `-` |
| `ldr` at 0x82c38 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x82c3c | store | 4 | `abs 0x149b98` | `[sp]` |
| `ldmdb` at 0x82c44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82c44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82c44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82c40 | `(unknown callee) 0x829f0` | `r0` | `r1` | `0x149b98` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82c44 | `r0` | `ret(0x829f0)` |
