# ARM power-bay request and status paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
`PBayCmd` reports one unresolved computed transfer (`ldrls` at 0x8fb64);
its unknown destinations are excluded from the summaries, including their call candidates and effects.
Other routines report no unresolved computed transfers.
Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/powerbay-request-status-facts.txt) retain all tool sections.

## `PBaySdrFlashInfo`

- `PBaySdrFlashInfo`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x93f98 to 0x94138.
  Established. A halfword store writes `([r0+6]:2 & 0xffff)` at `sp -0x1ac`, and a word store writes
  `0xc0` at `sp -0x1a8` (`strh` at 0x93fbc, `str` at 0x93fc4). The first call to `0x90884` carries
  entry r0, `sp-428`, `0xfffffe64`, `sp-428` (`bl` at 0x93ff4). A zero result takes the alternate
  path, while nonzero falls through to the longer calculation (`beq` at 0x94008). The later call
  to `0x18c74` has candidates `sp-516`, `0xfc134`, `[[sp-532]+1]:1`, `[[sp-532]]:1` (`bl` at
  0x940dc). Flash-information fields and physical effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x93fa8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x93fac | load | 4 | `abs 0x9412c` | `-` |
| `ldr` at 0x93fb0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x93fb4 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0x93fbc | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0x93fc4 | store | 4 | `sp -0x1a8` | `0xc0` |
| `str` at 0x93fcc | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0x93fd4 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0x93fdc | store | 4 | `sp -0x114` | `0x0` |
| `str` at 0x93fe4 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0x93fec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x93ffc | store | 4 | `sp -0x1b0` | `ret(0x90884)` |
| `ldr` at 0x94000 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0x9400c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x94010 | store | 4 | `sp -0x214` | `[sp-36]` |
| `ldr` at 0x94014 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0x94018 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x94020 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0x94024 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x94030 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0x94034 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldr` at 0x94040 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0x94044 | load | 1 | `[sp-36] +0x2` | `-` |
| `str` at 0x9404c | store | 4 | `sp -0x1b4` | `?` |
| `str` at 0x94060 | store | 4 | `sp -0x210` | `ret(0x18bc0)` |
| `ldr` at 0x94068 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0x9406c | load | 1 | `[sp-532] +0x1` | `-` |
| `ldr` at 0x94070 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0x94074 | load | 1 | `[sp-532] +0x0` | `-` |
| `ldr` at 0x94078 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0x9407c | load | 4 | `ret(0x18bc0) +0x10` | `-` |
| `str` at 0x94084 | store | 4 | `sp -0x22c` | `[ret(0x18bc0)+16]+1` |
| `ldr` at 0x94088 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0x9408c | load | 4 | `ret(0x18bc0) +0xc` | `-` |
| `str` at 0x94090 | store | 4 | `sp -0x228` | `[ret(0x18bc0)+12]` |
| `ldr` at 0x94094 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0x94098 | load | 4 | `ret(0x18bc0) +0x14` | `-` |
| `str` at 0x940a4 | store | 4 | `sp -0x224` | `[ret(0x18bc0)+20]+1900` |
| `ldr` at 0x940a8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0x940ac | load | 4 | `ret(0x18bc0) +0x8` | `-` |
| `str` at 0x940b0 | store | 4 | `sp -0x220` | `[ret(0x18bc0)+8]` |
| `ldr` at 0x940b4 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0x940b8 | load | 4 | `ret(0x18bc0) +0x4` | `-` |
| `str` at 0x940bc | store | 4 | `sp -0x21c` | `[ret(0x18bc0)+4]` |
| `ldr` at 0x940c0 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0x940c4 | load | 4 | `ret(0x18bc0) +0x0` | `-` |
| `str` at 0x940c8 | store | 4 | `sp -0x218` | `[ret(0x18bc0)]` |
| `ldr` at 0x940d0 | load | 4 | `abs 0x94130` | `-` |
| `str` at 0x940e4 | store | 4 | `sp -0x208` | `sp-516` |
| `ldr` at 0x940ec | load | 4 | `abs 0x94134` | `-` |
| `str` at 0x940f0 | store | 4 | `sp -0x208` | `0xfc174` |
| `ldr` at 0x940f4 | load | 4 | `sp -0x208` | `-` |
| `str` at 0x94110 | store | 4 | `sp -0x20c` | `ret(0x18920)` |
| `ldr` at 0x94114 | load | 4 | `sp -0x20c` | `-` |
| `ldr` at 0x94118 | load | 4 | `sp -0x208` | `-` |
| `ldr` at 0x94120 | load | 4 | `sp -0x20c` | `-` |
| `ldmdb` at 0x94128 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94128 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94128 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x93ff4 | `(unknown callee) 0x90884` | `r0` | `sp-428` | `0xfffffe64` | `sp-428` |
| `bl` at 0x94058 | `(unknown callee) 0x18bc0` | `sp-436` | `?` | `?` | `sp-436` |
| `bl` at 0x940dc | `(unknown callee) 0x18c74` | `sp-516` | `0xfc134` | `[[sp-532]+1]:1` | `[[sp-532]]:1` |
| `bl` at 0x940f8 | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x94108 | `(unknown callee) 0x18920` | `ret(0x18a1c)+1` | `?` | `?` | `ret(0x18a1c)+1` |
| `bl` at 0x9411c | `(unknown callee) 0x18cd4` | `ret(0x18920)` | `[sp-520]` | `?` | `ret(0x18920)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x94008 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94128 | `r0` | `[sp-524]` |

## `PBaySdrGetDpsStateHdr`

- `PBaySdrGetDpsStateHdr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91918 to 0x91964.
  Established. The first call targets `0x18920` with `0x50`, entry r1, entry r2 and entry r3 (`bl`
  at 0x9192c); its result is saved as a word at `sp -0x14` (`str` at 0x91934). Later calls use
  `ret(0x18920)`, `0xfbf38`, unknown, `ret(0x18920)` for `0x18cd4`, then `[sp-20]`, `0xfbf58`,
  unknown, unknown for `0x18ab8` (`bl` at 0x91940, `bl` at 0x9194c). The return reports `[sp-20]`
  (`ldmdb` at 0x91958). The returned data’s device meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x91934 | store | 4 | `sp -0x14` | `ret(0x18920)` |
| `ldr` at 0x91938 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9193c | load | 4 | `abs 0x9195c` | `-` |
| `ldr` at 0x91944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91948 | load | 4 | `abs 0x91960` | `-` |
| `ldr` at 0x91950 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x91958 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x91958 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x91958 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9192c | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x91940 | `(unknown callee) 0x18cd4` | `ret(0x18920)` | `0xfbf38` | `?` | `ret(0x18920)` |
| `bl` at 0x9194c | `(unknown callee) 0x18ab8` | `[sp-20]` | `0xfbf58` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x91958 | `r0` | `[sp-20]` |

## `PBaySdrGetMain`

- `PBaySdrGetMain`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91e3c to 0x92070.
  Established. An opening word store clears entry `r1 +0x0` (`str` at 0x91e5c). The call to
  `0x94138` has candidates `[sp-20]`, `sp-508`, entry r1, `sp-508` (`bl` at 0x91e88); its nonzero
  result takes the continuation and zero falls through to a stack word of `0x0` at `sp -0x200`
  (`bne` at 0x91e9c, `str` at 0x91ea4). Later, a byte store writes `([sp-500]:1 & 0xff)` at
  `ret(0x94484) +0x3` (`strb` at 0x91fe4). The assembled data’s protocol role and merged r0 return
  are unknown (`ldmdb` at 0x9204c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x91e4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x91e50 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x91e54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x91e5c | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0x91e60 | load | 4 | `abs 0x92050` | `-` |
| `ldr` at 0x91e64 | load | 4 | `abs 0x11b344` | `-` |
| `str` at 0x91e74 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0x91e80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x91e90 | store | 4 | `sp -0x1f4` | `ret(0x94138)` |
| `ldr` at 0x91e94 | load | 4 | `sp -0x1f4` | `-` |
| `str` at 0x91ea4 | store | 4 | `sp -0x200` | `0x0` |
| `str` at 0x91eb8 | store | 4 | `sp -0x20` | `ret(0x18920)` |
| `ldr` at 0x91ebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x91ec0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x91ec4 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x91ec8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x91ed0 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `str` at 0x91ed8 | store | 4 | `sp -0x1f0` | `0x0` |
| `ldr` at 0x91edc | load | 4 | `abs 0x92050` | `-` |
| `ldr` at 0x91ee0 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0x91ee4 | load | 4 | `abs 0x11b344` | `-` |
| `ldr` at 0x91ef8 | load | 4 | `abs 0x92054` | `-` |
| `ldr` at 0x91efc | load | 4 | `sp -0x1f0` | `-` |
| `ldrb` at 0x91f14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x91f18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91f2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x91f3c | store | 4 | `sp -0x1ec` | `ret(0x90884)` |
| `ldr` at 0x91f40 | load | 4 | `sp -0x1ec` | `-` |
| `ldr` at 0x91f4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91f50 | load | 4 | `abs 0x92058` | `-` |
| `str` at 0x91f5c | store | 4 | `sp -0x200` | `?` |
| `ldr` at 0x91f64 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0x91f78 | load | 4 | `abs 0x9205c` | `-` |
| `str` at 0x91f8c | store | 4 | `sp -0x24` | `ret(0x93a4c)` |
| `ldr` at 0x91f90 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x91f94 | load | 1 | `ret(0x93a4c) +0x0` | `-` |
| `ldr` at 0x91fa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91fa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x91fb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91fb4 | load | 4 | `abs 0x92060` | `-` |
| `ldr` at 0x91fbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91fc0 | load | 4 | `abs 0x92064` | `-` |
| `ldr` at 0x91fc8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x91fd0 | store | 4 | `sp -0x1c` | `ret(0x94484)` |
| `ldr` at 0x91fd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x91fd8 | load | 4 | `abs 0x92068` | `-` |
| `ldrb` at 0x91fe0 | load | 1 | `sp -0x1f4` | `-` |
| `strb` at 0x91fe4 | store | 1 | `ret(0x94484) +0x3` | `([sp-500]:1 & 0xff)` |
| `ldr` at 0x91fe8 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0x91ffc | load | 4 | `abs 0x9205c` | `-` |
| `ldr` at 0x92008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x92018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x92024 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x92028 | load | 4 | `abs 0x9206c` | `-` |
| `ldr` at 0x92030 | load | 4 | `sp -0x1f0` | `-` |
| `str` at 0x92038 | store | 4 | `sp -0x1f0` | `[sp-496]+1` |
| `str` at 0x92044 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0x92048 | load | 4 | `sp -0x200` | `-` |
| `ldmdb` at 0x9204c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9204c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9204c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x91e88 | `(unknown callee) 0x94138` | `[sp-20]` | `sp-508` | `r1` | `sp-508` |
| `bl` at 0x91eb0 | `(unknown callee) 0x18920` | `0x100` | `?` | `?` | `ret(0x94138)` |
| `bl` at 0x91f24 | `(unknown callee) 0x9359c` | `[sp-20]` | `sp-488` | `?` | `?` |
| `bl` at 0x91f34 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-488` | `?` | `sp-488` |
| `bl` at 0x91f54 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xfbf84` | `?` | `ret(0x90884)` |
| `bl` at 0x91f84 | `(unknown callee) 0x93a4c` | `?` | `?` | `?` | `?` |
| `bl` at 0x91fa8 | `(unknown callee) 0x18ab8` | `[sp-32]` | `ret(0x93a4c)` | `?` | `[ret(0x93a4c)]:1` |
| `bl` at 0x91fb8 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xfc044` | `?` | `[ret(0x93a4c)]:1` |
| `bl` at 0x91fc4 | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xfc054` | `?` | `?` |
| `bl` at 0x91fcc | `(unknown callee) 0x94484` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x92010 | `(unknown callee) 0x93920` | `[sp-28]` | `?` | `sp-76` | `?` |
| `bl` at 0x92020 | `(unknown callee) 0x18ab8` | `[sp-32]` | `sp-76` | `?` | `sp-76` |
| `bl` at 0x9202c | `(unknown callee) 0x18ab8` | `[sp-32]` | `0xfbff0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x91e6c | `[0x11b344] != 0x0` | `[0x11b344] == 0x0` |
| `bne` at 0x91e9c | `ret(0x94138) != 0x0` | `ret(0x94138) == 0x0` |
| `blt` at 0x91eec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x91f48 | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |
| `beq` at 0x91f9c | `[ret(0x93a4c)]:1 == 0x0` | `[ret(0x93a4c)]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9204c | `r0` | `?` |

## `PBaySdrGetConnector`

- `PBaySdrGetConnector`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x93c68 to 0x93d94.
  Established. A halfword store copies `([[sp-20]+6]:2 & 0xffff)` to `sp -0x1ac` (`strh` at
  0x93c9c). Stack word `sp -0x1a8` receives `0xa` and later `0xb` (`str` at 0x93ca4, `str` at
  0x93d1c). Two calls to `0x90884` have first/second candidates `[sp-20]`, `sp-428`, while their
  respective third/fourth candidates are `0xfffffe6c`, `sp-428` and unknown, `sp-428` (`bl` at
  0x93ce4, `bl` at 0x93d34). The later zero result takes the bypass and nonzero falls through (`beq`
  at 0x93d48). Connector encoding and the return value are unknown (`ldmdb` at 0x93d88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x93c78 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x93c88 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x93c8c | load | 4 | `abs 0x93d8c` | `-` |
| `ldr` at 0x93c90 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x93c94 | load | 2 | `[sp-20] +0x6` | `-` |
| `strh` at 0x93c9c | store | 2 | `sp -0x1ac` | `([[sp-20]+6]:2 & 0xffff)` |
| `str` at 0x93ca4 | store | 4 | `sp -0x1a8` | `0xa` |
| `str` at 0x93cac | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0x93cb4 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0x93cbc | store | 4 | `sp -0x114` | `0x1` |
| `ldr` at 0x93cc0 | load | 4 | `abs 0x93d90` | `-` |
| `strb` at 0x93ccc | store | 1 | `sp -0x1a4` | `0x0` |
| `str` at 0x93cd4 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0x93cdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x93cec | store | 4 | `sp -0x1b0` | `ret(0x90884)` |
| `ldr` at 0x93cf0 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0x93cfc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x93d00 | store | 4 | `sp -0x1b4` | `[sp-36]` |
| `ldr` at 0x93d04 | load | 4 | `sp -0x1b4` | `-` |
| `ldrb` at 0x93d08 | load | 1 | `[sp-36] +0x0` | `-` |
| `str` at 0x93d1c | store | 4 | `sp -0x1a8` | `0xb` |
| `str` at 0x93d24 | store | 4 | `sp -0x114` | `0x0` |
| `ldr` at 0x93d2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x93d3c | store | 4 | `sp -0x1b0` | `ret(0x90884)` |
| `ldr` at 0x93d40 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0x93d4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x93d50 | store | 4 | `sp -0x1b8` | `[sp-36]` |
| `ldr` at 0x93d54 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x93d58 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x93d68 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0x93d6c | load | 1 | `[sp-440] +0x0` | `-` |
| `strb` at 0x93d74 | store | 1 | `(unknown)` | `([[sp-440]]:1 & 0xff)` |
| `ldrb` at 0x93d80 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x93d88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x93d88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x93d88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x93ce4 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-428` | `0xfffffe6c` | `sp-428` |
| `bl` at 0x93d34 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-428` | `?` | `sp-428` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x93cf8 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |
| `beq` at 0x93d14 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x93d48 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |
| `beq` at 0x93d60 | `[[sp-36]]:1 == 0x0` | `[[sp-36]]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x93d88 | `r0` | `?` |

## `PBaySdrGetNbrDps`

- `PBaySdrGetNbrDps`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94138 to 0x94264.
  Established. One byte store writes `0x1` at an unknown target (`strb` at 0x941a4). A later store
  writes `(([ret(0x94484)+2]:1 & 0xff) & 0xff)` at `([sp-24] + [sp-32]) +0x0` (`strb` at 0x94220).
  The call to `0x90884` carries `[sp-20]`, `sp-452`, unknown, `sp-452` (`bl` at 0x941d4); its
  nonzero result takes the continuation, while zero falls through to word `0x0` at `sp -0x1cc`
  (`bne` at 0x941e8, `str` at 0x941f0). The stored data’s hardware interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94148 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9414c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x94154 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x9415c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x94160 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x94170 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x94178 | load | 4 | `abs 0x94260` | `-` |
| `str` at 0x94180 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x94184 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x94188 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x94194 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94198 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x941a4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x941b0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x941b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x941b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x941cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x941dc | store | 4 | `sp -0x1c8` | `ret(0x90884)` |
| `ldr` at 0x941e0 | load | 4 | `sp -0x1c8` | `-` |
| `str` at 0x941f0 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0x941f8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x94200 | store | 4 | `sp -0x28` | `ret(0x94484)` |
| `ldr` at 0x94204 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x94208 | load | 1 | `ret(0x94484) +0x2` | `-` |
| `strb` at 0x9420c | store | 1 | `sp -0x29` | `([ret(0x94484)+2]:1 & 0xff)` |
| `ldr` at 0x94210 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94214 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9421c | load | 1 | `sp -0x29` | `-` |
| `strb` at 0x94220 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `(([ret(0x94484)+2]:1 & 0xff) & 0xff)` |
| `ldrb` at 0x94224 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0x94234 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9423c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x94240 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x94248 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x94250 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x94254 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0x94258 | load | 4 | `sp -0x1cc` | `-` |
| `ldmdb` at 0x9425c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9425c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9425c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x941c4 | `(unknown callee) 0x9359c` | `?` | `sp-452` | `?` | `?` |
| `bl` at 0x941d4 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-452` | `?` | `sp-452` |
| `bl` at 0x941fc | `(unknown callee) 0x94484` | `[sp-60]` | `?` | `?` | `ret(0x90884)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x94168 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x94190 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x941e8 | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |
| `bne` at 0x94230 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9425c | `r0` | `?` |

## `PBayCmd`

- `PBayCmd`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f9d8 to 0x8ff7c.
  Established. The first call targets `0x7573c` with `[[[sp-28]+112]]`, `[sp-32]`, entry r1,
  `[[sp-28]+112]` (`bl` at 0x8fa14); its zero result takes the continuation and nonzero falls
  through to word `0x0` at `sp -0x34` (`beq` at 0x8fa20, `str` at 0x8fa28). A later computed
  transfer takes an unresolved edge for `[[[sp-28]+112]+4]-1 <= 0xf` unsigned and falls through
  above that bound (`ldrls` at 0x8fb64). The last call on known edges targets `0x1fe64` with
  `[0x11b32c]+160`, unknown, unknown, `[0x11b32c]+160` (`bl` at 0x8ff40). Excluded destinations’
  commands and physical behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f9e8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x8f9ec | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x8f9f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8f9f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8fa00 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0x8fa04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8fa08 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x8fa0c | load | 4 | `[[sp-28]+112] +0x0` | `-` |
| `ldr` at 0x8fa10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8fa28 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x8fa30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8fa38 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x8fa40 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0x8fa5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8fa6c | store | 4 | `[sp-32] +0x0` | `ret(0x18920)` |
| `ldr` at 0x8fa70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8fa74 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x8fa78 | load | 4 | `abs 0x8ff54` | `-` |
| `ldr` at 0x8fa80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8fa84 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x8fa8c | load | 4 | `abs 0x8ff58` | `-` |
| `ldr` at 0x8fa90 | load | 4 | `abs 0x11564c` | `-` |
| `ldr` at 0x8fa9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8faac | store | 4 | `[sp-32] +0x0` | `ret(0x18920)` |
| `ldr` at 0x8fab0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8fab4 | load | 4 | `abs 0x8ff58` | `-` |
| `ldr` at 0x8fab8 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x8fabc | load | 4 | `abs 0x11564c` | `-` |
| `str` at 0x8fac8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x8fad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8fad8 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x8fae0 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0x8faec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8faf8 | store | 4 | `sp -0x24` | `ret(0x90478)` |
| `ldr` at 0x8fafc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8fb00 | store | 4 | `sp -0x34` | `ret(0x90478)` |
| `ldr` at 0x8fb08 | load | 4 | `abs 0x8ff5c` | `-` |
| `ldr` at 0x8fb0c | load | 4 | `abs 0x11b32c` | `-` |
| `str` at 0x8fb24 | store | 4 | `sp -0x2c` | `ret(0x1fb00)` |
| `ldr` at 0x8fb28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8fb34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8fb38 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x8fb48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8fb50 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x8fb58 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldrls` at 0x8fb64 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8ff08 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8ff0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8ff14 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0x8ff1c | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0x8ff30 | load | 4 | `abs 0x8ff5c` | `-` |
| `ldr` at 0x8ff34 | load | 4 | `abs 0x11b32c` | `-` |
| `ldr` at 0x8ff44 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8ff48 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x8ff4c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x8ff50 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8ff50 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8ff50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8ff50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8ff50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8fa14 | `(unknown callee) 0x7573c` | `[[[sp-28]+112]]` | `[sp-32]` | `r1` | `[[sp-28]+112]` |
| `bl` at 0x8fa4c | `(unknown callee) 0x8f9b8` | `ret(0x7573c)` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0x8fa64 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `ret(0x8f9b8)` |
| `bl` at 0x8fa7c | `(unknown callee) 0x18cd4` | `[[sp-32]]` | `0xfbd9c` | `?` | `[sp-32]` |
| `bl` at 0x8faa4 | `(unknown callee) 0x18920` | `0x50` | `?` | `0x4` | `[0x11564c]` |
| `bl` at 0x8fac0 | `(unknown callee) 0x18cd4` | `[[sp-32]]` | `[0x11564c]` | `0x11564c` | `[sp-32]` |
| `bl` at 0x8faf0 | `(unknown callee) 0x90478` | `[sp-32]` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0x8fb1c | `(unknown callee) 0x1fb00` | `[0x11b32c]+160` | `0x0` | `0x4` | `[0x11b32c]+160` |
| `bl` at 0x8fb44 | `(unknown callee) 0x8f8dc` | `0x3` | `?` | `?` | `ret(0x1fb00)` |
| `bl` at 0x8ff2c | `(unknown callee) 0x8f8dc` | `0x2` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0x8ff40 | `(unknown callee) 0x1fe64` | `[0x11b32c]+160` | `?` | `?` | `[0x11b32c]+160` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8fa20 | `ret(0x7573c) == 0x0` | `ret(0x7573c) != 0x0` |
| `bne` at 0x8fa48 | `[[[sp-28]+112]+4] != 0xf` | `[[[sp-28]+112]+4] == 0xf` |
| `beq` at 0x8fa58 | `ret(0x8f9b8) == 0x5` | `ret(0x8f9b8) != 0x5` |
| `beq` at 0x8fa98 | `[0x11564c] == 0x0` | `[0x11564c] != 0x0` |
| `bne` at 0x8fae8 | `[[[sp-28]+112]+4] != 0x10` | `[[[sp-28]+112]+4] == 0x10` |
| `beq` at 0x8fb30 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `ldrls` at 0x8fb64 | `[[[sp-28]+112]+4]-1 <= 0xf (unsigned, 32-bit)` | `[[[sp-28]+112]+4]-1 > 0xf (unsigned, 32-bit)` |
| `beq` at 0x8ff24 | `[[[sp-28]+112]+4] == 0xf` | `[[[sp-28]+112]+4] != 0xf` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8ff50 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8fb64).

## `PBayPollHwStatus`

- `PBayPollHwStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9125c to 0x918f0.
  Established. The opening call to `0x1c79c` carries `0x55`, entry r1, entry r2, `0x0` (`bl` at
  0x91284). A nonzero result takes the continuation and zero falls through to the return (`bne`
  at 0x91294). Byte stores include `0x0` at `[sp-44] +0x11b334` and `([sp-64]:1 & 0xff)` at `[sp-44]
  +0x11b33b` (`strb` at 0x91510, `strb` at 0x91894). A later call to `0x636f8` has `0x0`,
  `[sp-44]+29440`, `[sp-44]+445`, `sp-168` (`bl` at 0x91880). Polling intervals, event meanings and
  physical status interpretation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9126c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x91274 | store | 4 | `sp -0x4c` | `0x1` |
| `str` at 0x9127c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x91288 | store | 4 | `sp -0x18` | `ret(0x1c79c)` |
| `ldr` at 0x9128c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x912b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x912e0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x912e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x912f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x912fc | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x9130c | store | 4 | `sp -0x24` | `ret(0x94138)` |
| `ldr` at 0x91310 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9132c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x91334 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x9133c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x91340 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91354 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x9135c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91364 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x9136c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9137c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9138c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x91390 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x91398 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9139c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x913a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x913b0 | load | 4 | `abs 0x918c8` | `-` |
| `str` at 0x913b4 | store | 4 | `sp -0xac` | `0xfbe9c` |
| `ldr` at 0x913b8 | load | 4 | `abs 0x918cc` | `-` |
| `str` at 0x913bc | store | 4 | `sp -0x44` | `0xbbf` |
| `str` at 0x913c4 | store | 4 | `sp -0x48` | `0x7500` |
| `ldr` at 0x913cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x913d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x913e0 | load | 4 | `abs 0x918d0` | `-` |
| `str` at 0x913e4 | store | 4 | `sp -0xac` | `0xfbea8` |
| `str` at 0x913ec | store | 4 | `sp -0x44` | `0xbc0` |
| `str` at 0x913f4 | store | 4 | `sp -0x48` | `0x7400` |
| `ldr` at 0x913f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91404 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x91418 | load | 4 | `abs 0x918d4` | `-` |
| `ldr` at 0x9141c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x91420 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x9142c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x91430 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0x91434 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x91438 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0x91440 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x91444 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x91450 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91458 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x91460 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x91464 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x91470 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x91488 | store | 4 | `sp -0x58` | `0x3` |
| `str` at 0x91490 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x91494 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x914a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x914b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x914b8 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x914c8 | store | 4 | `sp -0x24` | `ret(0x94138)` |
| `ldr` at 0x914cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x914e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x914f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x91500 | load | 4 | `abs 0x918d8` | `-` |
| `ldr` at 0x91504 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x91510 | store | 1 | `[sp-44] +0x11b334` | `0x0` |
| `str` at 0x91518 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0x9151c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91524 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x91530 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x91540 | load | 4 | `abs 0x918d8` | `-` |
| `ldr` at 0x91544 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9154c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x91550 | load | 1 | `[sp-44] +0x11b334` | `-` |
| `strb` at 0x91558 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0x9155c | load | 1 | `sp -0x39` | `-` |
| `str` at 0x9156c | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0x91570 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91578 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x91584 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x91594 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x91598 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x915a0 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0x915a4 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x915b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x915c8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x915d4 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0x915e4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x915ec | load | 4 | `abs 0x918dc` | `-` |
| `str` at 0x915f0 | store | 4 | `sp -0xac` | `0xfbec4` |
| `ldr` at 0x915f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91600 | store | 4 | `sp -0x44` | `[sp-44]+427` |
| `ldr` at 0x91604 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9160c | store | 4 | `sp -0x48` | `[sp-44]+28672` |
| `ldrb` at 0x91614 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x91624 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x91634 | load | 4 | `abs 0x918e0` | `-` |
| `str` at 0x91638 | store | 4 | `sp -0xac` | `0xfbee0` |
| `ldr` at 0x9163c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91648 | store | 4 | `sp -0x44` | `[sp-44]+433` |
| `ldr` at 0x9164c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91654 | store | 4 | `sp -0x48` | `[sp-44]+28928` |
| `ldrb` at 0x9165c | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x9166c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9167c | load | 4 | `abs 0x918e4` | `-` |
| `str` at 0x91680 | store | 4 | `sp -0xac` | `0xfbef8` |
| `ldr` at 0x91684 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91690 | store | 4 | `sp -0x44` | `[sp-44]+439` |
| `ldr` at 0x91694 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9169c | store | 4 | `sp -0x48` | `[sp-44]+29184` |
| `str` at 0x916a8 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x916ac | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x916c0 | load | 4 | `abs 0x918d4` | `-` |
| `ldr` at 0x916c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x916c8 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x916d4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x916d8 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0x916dc | load | 4 | `sp -0x50` | `-` |
| `str` at 0x916e0 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0x916e8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x916ec | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x916f8 | load | 4 | `abs 0x918d8` | `-` |
| `ldr` at 0x916fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x91708 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x91718 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x9171c | store | 1 | `[sp-44] +0x11b334` | `?` |
| `str` at 0x91724 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0x91728 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91730 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x91738 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9174c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x91750 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91764 | store | 4 | `sp -0x58` | `0x2` |
| `str` at 0x9176c | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0x91770 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x91784 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x91788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9178c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x91790 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9179c | store | 4 | `sp -0x40` | `ret(0x91c10)` |
| `ldr` at 0x917a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x917ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x917bc | load | 4 | `abs 0x918e8` | `-` |
| `ldr` at 0x917c0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x917c8 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0x917cc | load | 1 | `[sp-44] +0x11b33b` | `-` |
| `strb` at 0x917d8 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0x917dc | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x917ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x91800 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9180c | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0x9181c | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0x91834 | load | 4 | `abs 0x918ec` | `-` |
| `ldr` at 0x91838 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x91840 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9184c | store | 4 | `sp -0x44` | `[sp-44]+445` |
| `ldr` at 0x91850 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x91858 | store | 4 | `sp -0x48` | `[sp-44]+29440` |
| `ldr` at 0x91860 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x91864 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0x91868 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x9186c | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0x91874 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x91878 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x91884 | load | 4 | `abs 0x918e8` | `-` |
| `ldr` at 0x91888 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x91890 | load | 1 | `sp -0x40` | `-` |
| `strb` at 0x91894 | store | 1 | `[sp-44] +0x11b33b` | `([sp-64]:1 & 0xff)` |
| `ldr` at 0x9189c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x918a4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x918b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x918c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x918c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x918c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x91284 | `(unknown callee) 0x1c79c` | `0x55` | `r1` | `r2` | `0x0` |
| `bl` at 0x9129c | `(unknown callee) 0x8f9b8` | `ret(0x1c79c)` | `?` | `?` | `ret(0x1c79c)` |
| `bl` at 0x912c0 | `(unknown callee) 0x1fb00` | `[sp-20]+160` | `0x0` | `?` | `[sp-20]+160` |
| `bl` at 0x912d8 | `(unknown callee) 0x8f8dc` | `0x3` | `?` | `?` | `ret(0x1fb00)` |
| `bl` at 0x91304 | `(unknown callee) 0x94138` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0x91320 | `(unknown callee) 0x1c79c` | `0x69` | `?` | `?` | `ret(0x94138)` |
| `bl` at 0x91424 | `(unknown callee) 0x18c74` | `sp-168` | `0xfbeb0` | `[sp-44]` | `[sp-172]` |
| `bl` at 0x9144c | `(unknown callee) 0x636f8` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0x91480 | `(unknown callee) 0x1abd0` | `0x69` | `?` | `[sp-32]` | `?` |
| `bl` at 0x914c0 | `(unknown callee) 0x94138` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0x916cc | `(unknown callee) 0x18c74` | `sp-168` | `0xfbeb0` | `[sp-44]` | `?` |
| `bl` at 0x916f4 | `(unknown callee) 0x636f8` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0x91794 | `(unknown callee) 0x91c10` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x91824 | `(unknown callee) 0x918f0` | `0x64` | `[sp-88]` | `[sp-88]-1` | `0x1` |
| `bl` at 0x9183c | `(unknown callee) 0x18c74` | `sp-168` | `0xfbf08` | `[sp-44]` | `sp-168` |
| `bl` at 0x91880 | `(unknown callee) 0x636f8` | `0x0` | `[sp-44]+29440` | `[sp-44]+445` | `sp-168` |
| `bl` at 0x918b0 | `(unknown callee) 0x8f8dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x918c0 | `(unknown callee) 0x1fe64` | `[sp-20]+160` | `?` | `?` | `[sp-20]+160` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x91294 | `ret(0x1c79c) != 0x0` | `ret(0x1c79c) == 0x0` |
| `beq` at 0x912a8 | `ret(0x8f9b8) == 0x2` | `ret(0x8f9b8) != 0x2` |
| `beq` at 0x912cc | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x912f0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91318 | `ret(0x94138) == 0x0` | `ret(0x94138) != 0x0` |
| `ble` at 0x91348 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x91388 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x913ac | `? != 0x0` | `? == 0x0` |
| `beq` at 0x913dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91400 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x9140c | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0x9146c | `Z == 1` | `not (Z == 1)` |
| `ble` at 0x9149c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x914ac | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x914d4 | `ret(0x94138) != 0x0` | `ret(0x94138) == 0x0` |
| `beq` at 0x914fc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x91564 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0x915b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x915c0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x915dc | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |
| `beq` at 0x91620 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91630 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91668 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91678 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x916b4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x91744 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x91758 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x91778 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x917a8 | `ret(0x91c10) < 0x0 (signed, 32-bit)` | `ret(0x91c10) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x917b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x917e4 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0x917f8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x91814 | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x918c4 | `r0` | `?` |

## `PBaySdrGetDcPortState`

- `PBaySdrGetDcPortState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91cf0 to 0x91e3c.
  Established. A word store clears entry `r2 +0x0` (`str` at 0x91d14). The initial call to `0x91c10`
  uses `[sp-20]`, `[sp-24]`, entry r2, `0x0` (`bl` at 0x91d20). A later byte store writes
  `([sp-24]:1+48 & 0xff)` at `[sp-40] +0x3` (`strb` at 0x91d80). The signed comparison takes its
  continuation for `[sp-36] >= 0x0` and falls through below zero (`bge` at 0x91d8c). Port-state
  meanings and the joined return are unknown (`ldmdb` at 0x91e1c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x91d00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x91d04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x91d08 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x91d0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x91d14 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0x91d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91d1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x91d28 | store | 4 | `sp -0x24` | `ret(0x91c10)` |
| `ldr` at 0x91d2c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x91d3c | store | 4 | `sp -0x1cc` | `0x0` |
| `str` at 0x91d50 | store | 4 | `sp -0x28` | `ret(0x18920)` |
| `ldr` at 0x91d54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x91d58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x91d5c | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x91d60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91d64 | load | 4 | `abs 0x91e20` | `-` |
| `ldr` at 0x91d70 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x91d78 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x91d80 | store | 1 | `[sp-40] +0x3` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0x91d84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x91d90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91d94 | load | 4 | `abs 0x91e24` | `-` |
| `str` at 0x91da0 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0x91da8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x91db8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91dbc | load | 4 | `abs 0x91e28` | `-` |
| `ldr` at 0x91dc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91dcc | load | 4 | `abs 0x91e2c` | `-` |
| `ldr` at 0x91dd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x91de4 | load | 4 | `abs 0x91e30` | `-` |
| `str` at 0x91de8 | store | 4 | `sp -0x1d0` | `0xfbfd4` |
| `ldr` at 0x91df0 | load | 4 | `abs 0x91e34` | `-` |
| `str` at 0x91df4 | store | 4 | `sp -0x1d0` | `0xfc040` |
| `ldr` at 0x91df8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91dfc | load | 4 | `sp -0x1d0` | `-` |
| `ldr` at 0x91e04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91e08 | load | 4 | `abs 0x91e38` | `-` |
| `str` at 0x91e14 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0x91e18 | load | 4 | `sp -0x1cc` | `-` |
| `ldmdb` at 0x91e1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x91e1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x91e1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x91d20 | `(unknown callee) 0x91c10` | `[sp-20]` | `[sp-24]` | `r2` | `0x0` |
| `bl` at 0x91d48 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `ret(0x91c10)` |
| `bl` at 0x91d68 | `(unknown callee) 0x18cd4` | `[sp-40]` | `0xfc024` | `[sp-28]` | `ret(0x18920)` |
| `bl` at 0x91d98 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `[sp-40]+3` | `[sp-36]` |
| `bl` at 0x91dc0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfc02c` | `[sp-40]+3` | `?` |
| `bl` at 0x91dd0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfc034` | `[sp-40]+3` | `?` |
| `bl` at 0x91e00 | `(unknown callee) 0x18ab8` | `[sp-40]` | `[sp-464]` | `?` | `?` |
| `bl` at 0x91e0c | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbff0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x91d34 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x91d8c | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0x91db4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91de0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x91e1c | `r0` | `?` |

## `PBaySdrGetDcPort`

- `PBaySdrGetDcPort`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91c10 to 0x91cf0.
  Established. Two signed tests take the bypass for entry `r1 <= 0x0` and entry `r1 > 0x8`,
  respectively, and fall through for their complementary predicates (`ble` at 0x91c30, `bgt` at
  0x91c3c). The call to `0x9359c` carries entry r0, `sp-440`, `[[sp-28]]:1`, `[[sp-28]]:1` (`bl` at
  0x91c98). The following `0x90884` call uses `[sp-20]`, `sp-440`, unknown, `sp-440` (`bl` at
  0x91ca8). The final result-slot store writes `[ret(0x94484)+2]:1` at `sp -0x1c0` (`str` at
  0x91ce0). The returned `[sp-448]` value’s physical interpretation is unknown (`ldmdb` at 0x91ce8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x91c20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x91c24 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x91c28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x91c34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x91c48 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0x91c50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x91c58 | load | 4 | `abs 0x91cec` | `-` |
| `str` at 0x91c60 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x91c64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x91c68 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x91c78 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0x91c84 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x91c88 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x91c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91ca0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x91cb0 | store | 4 | `sp -0x1bc` | `ret(0x90884)` |
| `ldr` at 0x91cb4 | load | 4 | `sp -0x1bc` | `-` |
| `str` at 0x91cc4 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0x91ccc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x91cd4 | store | 4 | `sp -0x20` | `ret(0x94484)` |
| `ldr` at 0x91cd8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x91cdc | load | 1 | `ret(0x94484) +0x2` | `-` |
| `str` at 0x91ce0 | store | 4 | `sp -0x1c0` | `[ret(0x94484)+2]:1` |
| `ldr` at 0x91ce4 | load | 4 | `sp -0x1c0` | `-` |
| `ldmdb` at 0x91ce8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x91ce8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x91ce8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x91c98 | `(unknown callee) 0x9359c` | `r0` | `sp-440` | `[[sp-28]]:1` | `[[sp-28]]:1` |
| `bl` at 0x91ca8 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-440` | `?` | `sp-440` |
| `bl` at 0x91cd0 | `(unknown callee) 0x94484` | `[sp-48]` | `?` | `?` | `ret(0x90884)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x91c30 | `r1 <= 0x0 (signed, 32-bit)` | `r1 > 0x0 (signed, 32-bit)` |
| `bgt` at 0x91c3c | `r1 > 0x8 (signed, 32-bit)` | `r1 <= 0x8 (signed, 32-bit)` |
| `bne` at 0x91c70 | `[[sp-28]]:1 != 0xff` | `[[sp-28]]:1 == 0xff` |
| `bne` at 0x91cbc | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x91ce8 | `r0` | `[sp-448]` |

## `PBaySdrGetFruHdr`

- `PBaySdrGetFruHdr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x92070 to 0x920bc.
  Established. The call to `0x18920` uses `0xa0`, entry r1, entry r2 and entry r3 (`bl` at 0x92084).
  A stack word at `sp -0x14` retains its result (`str` at 0x9208c). Calls to `0x18cd4` and `0x18ab8`
  have second candidates `0xfc058` and `0xfc09c`, respectively (`bl` at 0x92098, `bl` at 0x920a4).
  The return reports `[sp-20]` (`ldmdb` at 0x920b0). No conditional branches are reported, and the
  returned data’s structure is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9208c | store | 4 | `sp -0x14` | `ret(0x18920)` |
| `ldr` at 0x92090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92094 | load | 4 | `abs 0x920b4` | `-` |
| `ldr` at 0x9209c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x920a0 | load | 4 | `abs 0x920b8` | `-` |
| `ldr` at 0x920a8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x920b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x920b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x920b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x92084 | `(unknown callee) 0x18920` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0x92098 | `(unknown callee) 0x18cd4` | `ret(0x18920)` | `0xfc058` | `?` | `ret(0x18920)` |
| `bl` at 0x920a4 | `(unknown callee) 0x18ab8` | `[sp-20]` | `0xfc09c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x920b0 | `r0` | `[sp-20]` |

## `PBaySdrGetFru`

- `PBaySdrGetFru`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x920bc to 0x92b44.
  Established. A halfword store clears `sp -0x2d2`, and a byte store writes `0x8` at `sp -0x2d3`
  (`strh` at 0x922b4, `strb` at 0x922c4). The first call to `0x937d8` uses `[sp-20]`, `sp-712`,
  `[sp-304]`, `[sp-722]:2` (`bl` at 0x923f8); its low-byte result being nonzero takes the
  continuation while zero falls through to the later return path (`bne` at 0x9241c). Later byte
  stores write `0x20` at `[sp-36] +0x0` and `0x0` at `[sp-36] +0x1` (`strb` at 0x9268c, `strb` at
  0x926a0). The call to `0x98568` carries `[sp-764]:1`, `[sp-762]:1`, `sp-296`, `([sp-40] +
  ret(0x18a1c))` (`bl` at 0x92a88). Record meanings, checks and physical effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x920cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x920d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x920d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x920d8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x920dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x920e4 | store | 4 | `r3 +0x0` | `0x0` |
| `ldr` at 0x920e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x920f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x92108 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x92110 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9211c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x92124 | load | 4 | `abs 0x92afc` | `-` |
| `str` at 0x9212c | store | 4 | `sp -0x130` | `?` |
| `ldr` at 0x92134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x92140 | load | 4 | `abs 0x92b00` | `-` |
| `str` at 0x92144 | store | 4 | `sp -0x130` | `0x1156b0` |
| `ldr` at 0x9214c | load | 4 | `abs 0x92afc` | `-` |
| `str` at 0x92150 | store | 4 | `sp -0x130` | `0x115694` |
| `ldr` at 0x92154 | load | 4 | `sp -0x130` | `-` |
| `ldrb` at 0x92158 | load | 1 | `[sp-304] +0x0` | `-` |
| `str` at 0x92168 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x92170 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9217c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x92184 | load | 4 | `abs 0x92b04` | `-` |
| `str` at 0x9218c | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0x92190 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0x92194 | load | 1 | `[sp-740] +0x0` | `-` |
| `str` at 0x921a4 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x921b0 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0x921b4 | load | 1 | `[sp-740] +0x0` | `-` |
| `ldr` at 0x921b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x921cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x921dc | store | 4 | `sp -0x2cc` | `ret(0x90884)` |
| `ldr` at 0x921e0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x921ec | load | 4 | `sp -0x140` | `-` |
| `str` at 0x921f4 | store | 4 | `sp -0x2e8` | `ret(0x94484)` |
| `ldr` at 0x921fc | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0x92200 | load | 1 | `[sp-744] +0x2` | `-` |
| `strb` at 0x92208 | store | 1 | `(unknown)` | `([[sp-744]+2]:1 & 0xff)` |
| `ldrb` at 0x92214 | load | 1 | `(unknown)` | `-` |
| `str` at 0x92228 | store | 4 | `sp -0x300` | `0x0` |
| `str` at 0x9223c | store | 4 | `sp -0x28` | `ret(0x18920)` |
| `ldr` at 0x92240 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x92244 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x92248 | store | 4 | `[sp-32] +0x0` | `ret(0x18920)` |
| `ldr` at 0x9224c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x92258 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9225c | load | 4 | `abs 0x92b08` | `-` |
| `ldr` at 0x92268 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x92270 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x92278 | store | 1 | `[sp-40] +0x1` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0x92280 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9228c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92290 | load | 4 | `abs 0x92b0c` | `-` |
| `ldr` at 0x9229c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x922a0 | load | 4 | `abs 0x92b10` | `-` |
| `ldr` at 0x922a8 | load | 4 | `abs 0x92b14` | `-` |
| `strh` at 0x922b4 | store | 2 | `sp -0x2d2` | `0x0` |
| `ldr` at 0x922b8 | load | 4 | `abs 0x92b18` | `-` |
| `strb` at 0x922c4 | store | 1 | `sp -0x2d3` | `0x8` |
| `ldr` at 0x922cc | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x922d4 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x922d8 | load | 4 | `abs 0x92b18` | `-` |
| `ldrb` at 0x922e0 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x922e4 | store | 4 | `sp -0x308` | `0x8` |
| `ldr` at 0x922e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x922f0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x92300 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x92310 | store | 4 | `sp -0x2cc` | `ret(0x90884)` |
| `ldr` at 0x92314 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x92320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92324 | load | 4 | `abs 0x92b1c` | `-` |
| `str` at 0x92330 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x92338 | load | 4 | `sp -0x140` | `-` |
| `str` at 0x92340 | store | 4 | `sp -0x2ec` | `ret(0x94484)` |
| `ldr` at 0x92348 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0x92360 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x9236c | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0x92378 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x92384 | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0x92390 | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x92394 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x923a0 | load | 1 | `sp -0x2f0` | `-` |
| `strh` at 0x923ac | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0x923b0 | load | 4 | `abs 0x92b18` | `-` |
| `strb` at 0x923bc | store | 1 | `sp -0x2d3` | `0x3` |
| `ldr` at 0x923c4 | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x923cc | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x923d0 | load | 4 | `abs 0x92b18` | `-` |
| `ldrb` at 0x923d8 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x923dc | store | 4 | `sp -0x308` | `0x3` |
| `str` at 0x923e4 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x923e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x923f0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x92400 | load | 4 | `abs 0x92b24` | `-` |
| `strb` at 0x92408 | store | 1 | `sp -0x2d4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x9240c | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x92414 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x92420 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92424 | load | 4 | `abs 0x92b1c` | `-` |
| `str` at 0x92430 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x92438 | load | 4 | `abs 0x92b18` | `-` |
| `ldr` at 0x9243c | load | 4 | `abs 0x92b28` | `-` |
| `ldrb` at 0x92444 | load | 1 | `sp -0x127` | `-` |
| `strb` at 0x92450 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0x92454 | load | 4 | `abs 0x92b18` | `-` |
| `ldr` at 0x92458 | load | 4 | `abs 0x92b18` | `-` |
| `ldr` at 0x9245c | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x92464 | load | 1 | `sp -0x2d3` | `-` |
| `ldrb` at 0x9246c | load | 1 | `sp -0x2d4` | `-` |
| `strb` at 0x92478 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0x9247c | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x92480 | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x92488 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0x92494 | store | 2 | `sp -0x2d2` | `([sp-722]:2+3 & 0xffff)` |
| `ldr` at 0x9249c | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x924a4 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x924a8 | load | 4 | `abs 0x92b18` | `-` |
| `ldrb` at 0x924b0 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x924b4 | store | 4 | `sp -0x308` | `[sp-723]:1` |
| `str` at 0x924bc | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x924c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x924c8 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x924d8 | load | 4 | `abs 0x92b24` | `-` |
| `strb` at 0x924e0 | store | 1 | `sp -0x2d4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x924e4 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x924ec | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x924f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x924fc | load | 4 | `abs 0x92b1c` | `-` |
| `str` at 0x92508 | store | 4 | `sp -0x300` | `?` |
| `str` at 0x92514 | store | 4 | `sp -0x12c` | `0x0` |
| `str` at 0x9251c | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0x92520 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x92530 | load | 4 | `abs 0x92b2c` | `-` |
| `ldr` at 0x92534 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0x92544 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92558 | load | 4 | `abs 0x92b2c` | `-` |
| `ldr` at 0x9255c | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0x9256c | load | 1 | `(sp-16 + [sp-300]) -0x118` | `-` |
| `strb` at 0x92578 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x9257c | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x92584 | store | 4 | `sp -0x12c` | `[sp-300]+1` |
| `ldr` at 0x92588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92598 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x925a0 | store | 4 | `sp -0x24` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x925a8 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0x925b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x925bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x925d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x925d8 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x925e0 | store | 4 | `sp -0x12c` | `?` |
| `ldrb` at 0x925ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x925f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x925f8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x92600 | load | 4 | `abs 0x92b30` | `-` |
| `ldr` at 0x92604 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0x92610 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x92614 | load | 1 | `[sp-720] +0x1156e4` | `-` |
| `ldr` at 0x92620 | load | 4 | `abs 0x92b30` | `-` |
| `ldr` at 0x92624 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0x9262c | load | 1 | `[sp-720] +0x1156e4` | `-` |
| `ldrb` at 0x92638 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92640 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x92650 | load | 4 | `abs 0x92b30` | `-` |
| `ldr` at 0x92654 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0x9265c | load | 1 | `[sp-720] +0x1156e4` | `-` |
| `ldrb` at 0x92668 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92670 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x92678 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x92680 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x9268c | store | 1 | `[sp-36] +0x0` | `0x20` |
| `str` at 0x92694 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x92698 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x926a0 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `ldr` at 0x926a4 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x926ac | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x926b4 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x926c0 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0x926cc | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x926d8 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0x926e4 | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x926e8 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x926f4 | load | 1 | `sp -0x2f1` | `-` |
| `strh` at 0x92700 | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0x92704 | load | 4 | `abs 0x92b18` | `-` |
| `strb` at 0x92710 | store | 1 | `sp -0x2d3` | `0x6` |
| `ldr` at 0x92718 | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x92720 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x92724 | load | 4 | `abs 0x92b18` | `-` |
| `ldrb` at 0x9272c | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0x92730 | store | 4 | `sp -0x308` | `0x6` |
| `str` at 0x92738 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x9273c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92744 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x92754 | load | 4 | `abs 0x92b24` | `-` |
| `strb` at 0x9275c | store | 1 | `sp -0x2d4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x92760 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x92768 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x92774 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92778 | load | 4 | `abs 0x92b1c` | `-` |
| `str` at 0x92784 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x9278c | load | 4 | `abs 0x92b34` | `-` |
| `ldrb` at 0x92794 | load | 1 | `sp -0x123` | `-` |
| `str` at 0x92798 | store | 4 | `sp -0x2dc` | `[sp-291]:1` |
| `ldr` at 0x9279c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x927a4 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x927a8 | load | 4 | `abs 0x92b38` | `-` |
| `ldrb` at 0x927b0 | load | 1 | `sp -0x124` | `-` |
| `ldr` at 0x927b4 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x927bc | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x927c0 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x927c8 | store | 4 | `sp -0x2dc` | `?` |
| `ldrb` at 0x927d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x927d8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x927e0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x927e4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x927f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x927fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x92808 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x92814 | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0x92820 | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x9282c | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0x92838 | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x9283c | load | 4 | `abs 0x92b20` | `-` |
| `ldrb` at 0x92848 | load | 1 | `sp -0x2ef` | `-` |
| `strh` at 0x92854 | store | 2 | `sp -0x2d2` | `?` |
| `str` at 0x9285c | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0x92864 | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x9286c | load | 2 | `sp -0x2d2` | `-` |
| `str` at 0x92874 | store | 4 | `sp -0x308` | `0x8` |
| `str` at 0x9287c | store | 4 | `sp -0x304` | `sp-764` |
| `ldr` at 0x92880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92888 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x92898 | load | 4 | `abs 0x92b24` | `-` |
| `strb` at 0x928a0 | store | 1 | `sp -0x2d4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x928a4 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x928ac | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x928b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x928bc | load | 4 | `abs 0x92b1c` | `-` |
| `str` at 0x928c8 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x928f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x928f4 | load | 4 | `abs 0x92b3c` | `-` |
| `str` at 0x92900 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x92908 | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x9290c | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x92910 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x92918 | load | 1 | `sp -0x2d4` | `-` |
| `ldrh` at 0x92924 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0x92930 | store | 2 | `sp -0x2d2` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `ldr` at 0x92938 | load | 4 | `abs 0x92b14` | `-` |
| `ldrh` at 0x92940 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0x92944 | load | 4 | `abs 0x92b40` | `-` |
| `ldrb` at 0x92950 | load | 1 | `sp -0x2fa` | `-` |
| `str` at 0x92954 | store | 4 | `sp -0x308` | `[sp-762]:1` |
| `str` at 0x9295c | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0x92960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92968 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x92978 | load | 4 | `abs 0x92b24` | `-` |
| `strb` at 0x92980 | store | 1 | `sp -0x2d4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x92984 | load | 4 | `abs 0x92b24` | `-` |
| `ldr` at 0x92988 | load | 4 | `abs 0x92b40` | `-` |
| `ldrb` at 0x92998 | load | 1 | `sp -0x2d4` | `-` |
| `ldrb` at 0x9299c | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0x929a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x929ac | load | 4 | `abs 0x92b1c` | `-` |
| `str` at 0x929b8 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x929c0 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x929c8 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x929d0 | load | 4 | `abs 0x92b2c` | `-` |
| `ldr` at 0x929e0 | load | 4 | `abs 0x92b40` | `-` |
| `ldrb` at 0x929ec | load | 1 | `sp -0x2f9` | `-` |
| `strb` at 0x929f0 | store | 1 | `(unknown)` | `([sp-761]:1 & 0xff)` |
| `ldr` at 0x929f8 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x92a00 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x92a24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92a28 | load | 4 | `abs 0x92b3c` | `-` |
| `str` at 0x92a34 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0x92a3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x92a4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x92a54 | store | 4 | `sp -0x24` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x92a58 | load | 4 | `abs 0x92b40` | `-` |
| `ldrb` at 0x92a64 | load | 1 | `sp -0x2fc` | `-` |
| `ldr` at 0x92a68 | load | 4 | `abs 0x92b40` | `-` |
| `ldrb` at 0x92a74 | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0x92a84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x92a8c | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x92a90 | load | 4 | `abs 0x92b14` | `-` |
| `ldr` at 0x92a94 | load | 4 | `abs 0x92b24` | `-` |
| `ldrb` at 0x92a9c | load | 1 | `sp -0x2d4` | `-` |
| `ldrh` at 0x92aa8 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0x92ab4 | store | 2 | `sp -0x2d2` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `ldr` at 0x92ab8 | load | 4 | `abs 0x92b40` | `-` |
| `ldrsb` at 0x92ac4 | load | 1 | `sp -0x2fb` | `-` |
| `ldr` at 0x92ad4 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x92ae0 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `str` at 0x92af0 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0x92af4 | load | 4 | `sp -0x300` | `-` |
| `ldmdb` at 0x92af8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x92af8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x92af8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x921c4 | `(unknown callee) 0x9359c` | `[sp-20]` | `sp-712` | `[[sp-740]]:1` | `[[sp-740]]:1` |
| `bl` at 0x921d4 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x921f0 | `(unknown callee) 0x94484` | `[sp-320]` | `?` | `?` | `ret(0x90884)` |
| `bl` at 0x92234 | `(unknown callee) 0x18920` | `0x400` | `?` | `?` | `?` |
| `bl` at 0x92260 | `(unknown callee) 0x18cd4` | `[sp-40]` | `0xfc0e0` | `[sp-32]` | `[sp-24]` |
| `bl` at 0x92294 | `(unknown callee) 0x18cd4` | `[sp-40]` | `0xfc0e8` | `[sp-32]` | `[sp-24]` |
| `bl` at 0x922a4 | `(unknown callee) 0x18cd4` | `[sp-40]` | `0xfc0f0` | `[sp-32]` | `[sp-24]` |
| `bl` at 0x922f8 | `(unknown callee) 0x936bc` | `[sp-20]` | `sp-712` | `[sp-304]` | `0x0` |
| `bl` at 0x92308 | `(unknown callee) 0x90884` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x92328 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `?` | `ret(0x90884)` |
| `bl` at 0x9233c | `(unknown callee) 0x94484` | `[sp-320]` | `?` | `?` | `ret(0x90884)` |
| `bl` at 0x9235c | `(unknown callee) 0x18ae8` | `sp-756` | `ret(0x94484)+1` | `0x8` | `ret(0x94484)+1` |
| `bl` at 0x923f8 | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0x92428 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `ret(0x937d8)` | `(ret(0x937d8) & 0xff)` |
| `bl` at 0x924d0 | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-712` | `[sp-304]` | `([sp-722]:2+3 & 0xffff)` |
| `bl` at 0x92500 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `sp-16` | `(ret(0x937d8) & 0xff)` |
| `bl` at 0x9258c | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `sp-16` | `[sp-300]+1` |
| `bl` at 0x925c8 | `(unknown callee) 0x18ae8` | `[sp-36]` | `(sp-296 + [sp-300])` | `?` | `?` |
| `bl` at 0x9264c | `(unknown callee) 0x18c20` | `[sp-36]` | `0x20` | `?` | `?` |
| `bl` at 0x9274c | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0x9277c | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `ret(0x937d8)` | `(ret(0x937d8) & 0xff)` |
| `bl` at 0x927e8 | `(unknown callee) 0x93d94` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0x927f8 | `(unknown callee) 0x18ab8` | `[sp-40]` | `ret(0x93d94)` | `?` | `ret(0x93d94)` |
| `bl` at 0x92890 | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0x928c0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `ret(0x937d8)` | `(ret(0x937d8) & 0xff)` |
| `bl` at 0x928dc | `(unknown callee) 0x93dec` | `sp-764` | `0x8` | `ret(0x937d8)` | `sp-764` |
| `bl` at 0x928f8 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfc0f8` | `?` | `?` |
| `bl` at 0x92970 | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-712` | `[sp-304]` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `bl` at 0x929b0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `(ret(0x937d8) & 0xff)` | `[sp-762]:1` |
| `bl` at 0x92a10 | `(unknown callee) 0x93dec` | `sp-296` | `[sp-724]:1+1` | `sp-296` | `[sp-724]:1+1` |
| `bl` at 0x92a2c | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfc0f8` | `?` | `?` |
| `bl` at 0x92a40 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x92a88 | `(unknown callee) 0x98568` | `[sp-764]:1` | `[sp-762]:1` | `sp-296` | `([sp-40] + ret(0x18a1c))` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x920f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0x920fc | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `ble` at 0x92118 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0x9213c | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x92160 | `[[sp-304]]:1 != 0xff` | `[[sp-304]]:1 == 0xff` |
| `ble` at 0x92178 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bne` at 0x9219c | `[[sp-740]]:1 != 0xff` | `[[sp-740]]:1 == 0xff` |
| `beq` at 0x921e8 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |
| `beq` at 0x92220 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x92254 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0x92288 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x9231c | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |
| `beq` at 0x92374 | `[sp-752]:1 == 0x0` | `[sp-752]:1 != 0x0` |
| `beq` at 0x9238c | `[sp-752]:1 == 0xff` | `[sp-752]:1 != 0xff` |
| `bne` at 0x9241c | `(ret(0x937d8) & 0xff) != 0x0` | `(ret(0x937d8) & 0xff) == 0x0` |
| `bne` at 0x924f4 | `(ret(0x937d8) & 0xff) != 0x0` | `(ret(0x937d8) & 0xff) == 0x0` |
| `ble` at 0x92528 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9254c | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x9261c | `? >= [[sp-720]+1136356]:1 (unsigned, 32-bit)` | `? < [[sp-720]+1136356]:1 (unsigned, 32-bit)` |
| `beq` at 0x926c8 | `[sp-753]:1 == 0x0` | `[sp-753]:1 != 0x0` |
| `beq` at 0x926e0 | `[sp-753]:1 == 0xff` | `[sp-753]:1 != 0xff` |
| `bne` at 0x92770 | `(ret(0x937d8) & 0xff) != 0x0` | `(ret(0x937d8) & 0xff) == 0x0` |
| `beq` at 0x92804 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x9281c | `[sp-751]:1 == 0x0` | `[sp-751]:1 != 0x0` |
| `beq` at 0x92834 | `[sp-751]:1 == 0xff` | `[sp-751]:1 != 0xff` |
| `beq` at 0x928b4 | `(ret(0x937d8) & 0xff) == 0x8` | `(ret(0x937d8) & 0xff) != 0x8` |
| `beq` at 0x928ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x929a4 | `(ret(0x937d8) & 0xff) == [sp-762]:1` | `(ret(0x937d8) & 0xff) != [sp-762]:1` |
| `beq` at 0x92a20 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x92acc | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `ble` at 0x92ae8 | `[sp-720] <= 0x4 (signed, 32-bit)` | `[sp-720] > 0x4 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x92af8 | `r0` | `?` |

## `PBaySdrGetDcPortStateHdr`

- `PBaySdrGetDcPortStateHdr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91bc4 to 0x91c10.
  Established. A word store puts `ret(0x18920)` at `sp -0x14` (`str` at 0x91be0). The later call to
  `0x18cd4` carries that result, `0xfbff4`, unknown, `ret(0x18920)` (`bl` at 0x91bec). The final
  call targets `0x18ab8` with `[sp-20]`, `0xfc008`, unknown, unknown (`bl` at 0x91bf8). The body
  returns `[sp-20]` (`ldmdb` at 0x91c04). There are no reported conditional branches, and the returned data’s
  interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x91be0 | store | 4 | `sp -0x14` | `ret(0x18920)` |
| `ldr` at 0x91be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91be8 | load | 4 | `abs 0x91c08` | `-` |
| `ldr` at 0x91bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91bf4 | load | 4 | `abs 0x91c0c` | `-` |
| `ldr` at 0x91bfc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x91c04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x91c04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x91c04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x91bd8 | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x91bec | `(unknown callee) 0x18cd4` | `ret(0x18920)` | `0xfbff4` | `?` | `ret(0x18920)` |
| `bl` at 0x91bf8 | `(unknown callee) 0x18ab8` | `[sp-20]` | `0xfc008` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x91c04 | `r0` | `[sp-20]` |

## `PBaySdrGetDpsState`

- `PBaySdrGetDpsState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x91964 to 0x91bc4.
  Established. The opening word store clears entry `r2 +0x0` (`str` at 0x91988). Signed tests take
  the bypass for `[sp-24] <= 0x0` and `[sp-24] > 0x6`, respectively, while their complementary
  predicates fall through (`ble` at 0x91994, `bgt` at 0x919a0). A byte store writes `([sp-24]:1+48 &
  0xff)` at `[sp-40] +0x1` (`strb` at 0x91a20). The call to `0x90884` uses `[sp-20]`, `sp-456`,
  unknown, `sp-456` (`bl` at 0x91a4c). The data’s physical supply-state interpretation and joined r0
  return remain unknown (`ldmdb` at 0x91b8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x91974 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x91978 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9197c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x91980 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x91988 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0x9198c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x91998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x919ac | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0x919b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x919bc | load | 4 | `abs 0x91b90` | `-` |
| `str` at 0x919c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x919c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x919cc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x919dc | store | 4 | `sp -0x1d0` | `0x0` |
| `str` at 0x919f0 | store | 4 | `sp -0x28` | `ret(0x18920)` |
| `ldr` at 0x919f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x919f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x919fc | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x91a00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91a04 | load | 4 | `abs 0x91b94` | `-` |
| `ldr` at 0x91a10 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x91a18 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x91a20 | store | 1 | `[sp-40] +0x1` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0x91a28 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x91a2c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x91a30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x91a44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x91a54 | store | 4 | `sp -0x1cc` | `ret(0x90884)` |
| `ldr` at 0x91a58 | load | 4 | `sp -0x1cc` | `-` |
| `ldr` at 0x91a64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91a68 | load | 4 | `abs 0x91b98` | `-` |
| `str` at 0x91a74 | store | 4 | `sp -0x1d0` | `?` |
| `ldr` at 0x91a7c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x91a84 | store | 4 | `sp -0x24` | `ret(0x94484)` |
| `ldr` at 0x91a88 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x91a8c | load | 1 | `ret(0x94484) +0x2` | `-` |
| `strb` at 0x91a90 | store | 1 | `sp -0x2d` | `([ret(0x94484)+2]:1 & 0xff)` |
| `ldrb` at 0x91a94 | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0x91aa4 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x91ab4 | load | 4 | `abs 0x91b9c` | `-` |
| `str` at 0x91ab8 | store | 4 | `sp -0x2c` | `0xfbf94` |
| `ldr` at 0x91ac0 | load | 4 | `abs 0x91ba0` | `-` |
| `str` at 0x91ac4 | store | 4 | `sp -0x2c` | `0xfbf9c` |
| `ldr` at 0x91ac8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91acc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x91ad4 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x91ae4 | load | 4 | `abs 0x91ba4` | `-` |
| `str` at 0x91ae8 | store | 4 | `sp -0x2c` | `0xfbfa4` |
| `ldr` at 0x91af0 | load | 4 | `abs 0x91ba8` | `-` |
| `str` at 0x91af4 | store | 4 | `sp -0x2c` | `0xfbfb0` |
| `ldr` at 0x91af8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91afc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x91b04 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x91b14 | load | 4 | `abs 0x91bac` | `-` |
| `str` at 0x91b18 | store | 4 | `sp -0x2c` | `0xfbfbc` |
| `ldr` at 0x91b20 | load | 4 | `abs 0x91bb0` | `-` |
| `str` at 0x91b24 | store | 4 | `sp -0x2c` | `0xfbfc8` |
| `ldr` at 0x91b28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91b2c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x91b34 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x91b44 | load | 4 | `abs 0x91bb4` | `-` |
| `str` at 0x91b48 | store | 4 | `sp -0x2c` | `0xfbfd4` |
| `ldr` at 0x91b50 | load | 4 | `abs 0x91bb8` | `-` |
| `str` at 0x91b54 | store | 4 | `sp -0x2c` | `0xfbfdc` |
| `ldr` at 0x91b58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91b5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x91b68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91b6c | load | 4 | `abs 0x91bbc` | `-` |
| `ldr` at 0x91b74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x91b78 | load | 4 | `abs 0x91bc0` | `-` |
| `str` at 0x91b84 | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0x91b88 | load | 4 | `sp -0x1d0` | `-` |
| `ldmdb` at 0x91b8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x91b8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x91b8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x919e8 | `(unknown callee) 0x18920` | `0x50` | `r1` | `?` | `[[sp-32]]:1` |
| `bl` at 0x91a08 | `(unknown callee) 0x18cd4` | `[sp-40]` | `0xfbf7c` | `[sp-28]` | `ret(0x18920)` |
| `bl` at 0x91a3c | `(unknown callee) 0x9359c` | `[sp-20]` | `sp-456` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0x91a4c | `(unknown callee) 0x90884` | `[sp-20]` | `sp-456` | `?` | `sp-456` |
| `bl` at 0x91a6c | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbf84` | `?` | `ret(0x90884)` |
| `bl` at 0x91a80 | `(unknown callee) 0x94484` | `[sp-64]` | `?` | `?` | `ret(0x90884)` |
| `bl` at 0x91ad0 | `(unknown callee) 0x18ab8` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x91b00 | `(unknown callee) 0x18ab8` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x91b30 | `(unknown callee) 0x18ab8` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x91b60 | `(unknown callee) 0x18ab8` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0x91b70 | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbfe4` | `?` | `?` |
| `bl` at 0x91b7c | `(unknown callee) 0x18ab8` | `[sp-40]` | `0xfbff0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x91994 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bgt` at 0x919a0 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `bne` at 0x919d4 | `[[sp-32]]:1 != 0xff` | `[[sp-32]]:1 == 0xff` |
| `bne` at 0x91a60 | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |
| `bne` at 0x91aa0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x91ab0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91ae0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91b10 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x91b40 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x91b8c | `r0` | `?` |

## `cpuEnabled`

- `cpuEnabled`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e8fc to 0x2e988.
  Established. Entry r0 is saved as byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x2e910). The first
  comparison takes its bypass for `(r0 & 0xff) >= [0x27ee5c]` signed and falls through below that
  value (`bge` at 0x2e924). The later call to `0x1c79c` carries `[sp-17]:1+88`, unknown, unknown,
  `[sp-17]:1+88` (`bl` at 0x2e94c). The return is unknown after the path join (`ldmdb` at 0x2e980).
  A physical CPU-enable state is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2e910 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2e914 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e918 | load | 4 | `abs 0x2e984` | `-` |
| `ldr` at 0x2e91c | load | 4 | `abs 0x27ee5c` | `-` |
| `ldrb` at 0x2e928 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x2e940 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2e950 | store | 4 | `sp -0x1c` | `ret(0x1c79c)` |
| `ldr` at 0x2e954 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2e964 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2e968 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2e96c | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x2e978 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2e97c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2e980 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2e980 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2e980 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e930 | `(unknown callee) 0x2e878` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x2e94c | `(unknown callee) 0x1c79c` | `[sp-17]:1+88` | `?` | `?` | `[sp-17]:1+88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2e924 | `(r0 & 0xff) >= [0x27ee5c] (signed, 32-bit)` | `(r0 & 0xff) < [0x27ee5c] (signed, 32-bit)` |
| `beq` at 0x2e93c | `ret(0x2e878) == 0x0` | `ret(0x2e878) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2e980 | `r0` | `?` |

## `nodeEnabled`

- `nodeEnabled`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ef84 to 0x2f020.
  Established. The first call targets `0x2ef00` with `(r0 & 0xff)`, entry r1, `(r0 & 0xff)`, `(r0 &
  0xff)` (`bl` at 0x2efb8); its zero result takes the bypass and nonzero falls through to the
  next call (`beq` at 0x2efc4). That call to `0x1c79c` uses `0x70`, unknown, unknown, `ret(0x2ef00)`
  (`bl` at 0x2efcc). The stack word at `sp -0x1c` first receives `ret(0x1c79c)` and later unknown
  values (`str` at 0x2efd0, `str` at 0x2efe8, `str` at 0x2effc). Node-enable interpretation and the
  merged return are unknown (`ldmdb` at 0x2f018).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2ef98 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2ef9c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2efa0 | load | 4 | `abs 0x2f01c` | `-` |
| `ldr` at 0x2efa4 | load | 4 | `abs 0x27ee70` | `-` |
| `ldrb` at 0x2efb0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2efd0 | store | 4 | `sp -0x1c` | `ret(0x1c79c)` |
| `ldrb` at 0x2efd4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2efe0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2efe8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2efec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2effc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2f000 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2f004 | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x2f010 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2f014 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2f018 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2f018 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2f018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2efb8 | `(unknown callee) 0x2ef00` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x2efcc | `(unknown callee) 0x1c79c` | `0x70` | `?` | `?` | `ret(0x2ef00)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2efac | `(r0 & 0xff) >= [0x27ee70] (signed, 32-bit)` | `(r0 & 0xff) < [0x27ee70] (signed, 32-bit)` |
| `beq` at 0x2efc4 | `ret(0x2ef00) == 0x0` | `ret(0x2ef00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2f018 | `r0` | `?` |
