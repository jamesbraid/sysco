# ARM I2C configuration and environment paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument
candidates. `i2cMuxIP41BusSelect` and `i2c_soft_l1_reset_check` each report one unresolved
computed transfer. The other thirteen routines report none. Undecodable byte ranges remain
visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/i2c-environment-paths-facts.txt) retain all tool sections.

## `i2cMuxIP41BusSelect`

- `i2cMuxIP41BusSelect`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23248 to 0x2349c.
  Established. The opening equality branch takes the special path for `(r0 & 0xff) == 0x1e` and
  falls through for other values (`beq` at 0x23268). That special path calls `0xb6d4c` with first
  candidate `0x3` (`bl` at 0x23274), then stores word `0x0` at `sp -0x18` (`str` at 0x2327c). The
  other path calls the same target with first candidate `0x1` (`bl` at 0x23288) before an unresolved
  conditional computed transfer (`ldrls` at 0x23294); its reported taken condition is `[sp-17]:1 <=
  0x11` unsigned, with the greater-than case falling through (`ldrls` at 0x23294). A later reachable
  call to `0xb6968` has candidates `[sp-18]:1`, `[sp-19]:1`, `[sp-19]:1`, `[sp-18]:1` (`bl` at
  0x2347c). The computed destinations and bus-to-selector mapping are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2325c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x23260 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2327c | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x2328c | load | 1 | `sp -0x11` | `-` |
| `ldrls` at 0x23294 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x23448 | load | 4 | `abs 0x23490` | `-` |
| `str` at 0x2344c | store | 4 | `sp -0x20` | `0xda560` |
| `ldrb` at 0x23450 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x23454 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x2345c | load | 4 | `abs 0x23494` | `-` |
| `ldr` at 0x23460 | load | 4 | `abs 0x23498` | `-` |
| `ldrb` at 0x2346c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x23470 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x23484 | store | 4 | `sp -0x18` | `ret(0xb6968)` |
| `ldr` at 0x23488 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2348c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2348c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2348c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23274 | `(unknown callee) 0xb6d4c` | `0x3` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x23288 | `(unknown callee) 0xb6d4c` | `0x1` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x23468 | `(unknown callee) 0x1eed0` | `0x9` | `0xd9624` | `0xa19` | `0x0` |
| `bl` at 0x2347c | `(unknown callee) 0xb6968` | `[sp-18]:1` | `[sp-19]:1` | `[sp-19]:1` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23268 | `(r0 & 0xff) == 0x1e` | `(r0 & 0xff) != 0x1e` |
| `ldrls` at 0x23294 | `[sp-17]:1 <= 0x11 (unsigned, 32-bit)` | `[sp-17]:1 > 0x11 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2348c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x23294).

## `ip41_i2cConfig`

- `ip41_i2cConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23754 to 0x238b4.
  Established. The opening call to `0xab3d8` has candidates `0x23248`, entry r1, entry r2 and entry
  r3 (`bl` at 0x23764). A later call to `0x395a0` uses `0xda28c`, `0xb`, `0x21a94`, `0x21b38` (`bl`
  at 0x237b4). A zero result from `0x21d5c` takes the edge to the call to `0x41b04` with first
  candidate `0xda3f4` (`bl` at 0x237f8, `beq` at 0x23804, `bl` at 0x23820); its nonzero result falls
  through to the call with first candidate `0xda400` (`beq` at 0x23804, `bl` at 0x23810). The body
  also includes calls to `0xac18c` with `0xb9800`, `0x2`, `0x0`, `0x2` and with `0xb12dc`, `0x3`,
  `0x0`, `0x1` (`bl` at 0x2384c, `bl` at 0x23860). The configuration meanings of these candidates
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x23760 | load | 4 | `abs 0x2386c` | `-` |
| `ldr` at 0x23768 | load | 4 | `abs 0x23870` | `-` |
| `ldr` at 0x23774 | load | 4 | `abs 0x23874` | `-` |
| `ldr` at 0x23780 | load | 4 | `abs 0x23878` | `-` |
| `ldr` at 0x2378c | load | 4 | `abs 0x2387c` | `-` |
| `ldr` at 0x23798 | load | 4 | `abs 0x23880` | `-` |
| `ldr` at 0x237a4 | load | 4 | `abs 0x23884` | `-` |
| `ldr` at 0x237ac | load | 4 | `abs 0x23888` | `-` |
| `ldr` at 0x237b0 | load | 4 | `abs 0x2388c` | `-` |
| `ldr` at 0x237b8 | load | 4 | `abs 0x23890` | `-` |
| `ldr` at 0x237c4 | load | 4 | `abs 0x23894` | `-` |
| `ldr` at 0x237d0 | load | 4 | `abs 0x23898` | `-` |
| `ldr` at 0x237dc | load | 4 | `abs 0x2389c` | `-` |
| `ldr` at 0x23808 | load | 4 | `abs 0x238a0` | `-` |
| `ldr` at 0x23818 | load | 4 | `abs 0x238a4` | `-` |
| `ldr` at 0x23824 | load | 4 | `abs 0x238a8` | `-` |
| `ldr` at 0x2383c | load | 4 | `abs 0x238ac` | `-` |
| `ldr` at 0x23850 | load | 4 | `abs 0x238b0` | `-` |
| `ldmdb` at 0x23868 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23868 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23868 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x23764 | `(unknown callee) 0xab3d8` | `0x23248` | `r1` | `r2` | `r3` |
| `bl` at 0x23770 | `(unknown callee) 0xb61f4` | `0xd9a6c` | `0x6` | `?` | `?` |
| `bl` at 0x2377c | `(unknown callee) 0xb8c38` | `0xd9ab4` | `0x3` | `?` | `?` |
| `bl` at 0x23788 | `(unknown callee) 0xb7084` | `0xd9c20` | `0x17` | `?` | `?` |
| `bl` at 0x23794 | `(unknown callee) 0xb3fc8` | `0xd9f98` | `0x2e` | `?` | `?` |
| `bl` at 0x237a0 | `(unknown callee) 0xafafc` | `0xda1d8` | `0x7` | `?` | `?` |
| `bl` at 0x237b4 | `(unknown callee) 0x395a0` | `0xda28c` | `0xb` | `0x21a94` | `0x21b38` |
| `bl` at 0x237c0 | `(unknown callee) 0x2d664` | `0xda448` | `0x6` | `?` | `?` |
| `bl` at 0x237cc | `(unknown callee) 0x2daac` | `0xda394` | `0xc` | `?` | `?` |
| `bl` at 0x237d8 | `(unknown callee) 0x3e5ec` | `0xda498` | `0x6` | `?` | `?` |
| `bl` at 0x237e4 | `(unknown callee) 0x41234` | `0xda558` | `0x2` | `?` | `?` |
| `bl` at 0x237ec | `(unknown callee) 0x2bccc` | `0x16` | `?` | `?` | `?` |
| `bl` at 0x237f0 | `(unknown callee) 0x214e8` | `ret(0x2bccc)` | `?` | `?` | `?` |
| `bl` at 0x237f4 | `(unknown callee) 0xb4254` | `ret(0x214e8)` | `?` | `?` | `?` |
| `bl` at 0x237f8 | `(unknown callee) 0x21d5c` | `ret(0xb4254)` | `?` | `?` | `?` |
| `bl` at 0x23810 | `(unknown callee) 0x41b04` | `0xda400` | `0x1` | `?` | `ret(0x21d5c)` |
| `bl` at 0x23820 | `(unknown callee) 0x41b04` | `0xda3f4` | `0x1` | `?` | `ret(0x21d5c)` |
| `bl` at 0x2382c | `(unknown callee) 0x419d8` | `0xda424` | `0x3` | `?` | `?` |
| `bl` at 0x23830 | `(unknown callee) 0x3e2bc` | `ret(0x419d8)` | `?` | `?` | `?` |
| `bl` at 0x23834 | `(unknown callee) 0x2349c` | `ret(0x3e2bc)` | `?` | `?` | `?` |
| `bl` at 0x23838 | `(unknown callee) 0x41350` | `ret(0x2349c)` | `?` | `?` | `?` |
| `bl` at 0x2384c | `(unknown callee) 0xac18c` | `0xb9800` | `0x2` | `0x0` | `0x2` |
| `bl` at 0x23860 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x3` | `0x0` | `0x1` |
| `bl` at 0x23864 | `(unknown callee) 0x2fc44` | `ret(0xac18c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x23804 | `ret(0x21d5c) == 0x0` | `ret(0x21d5c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23868 | `r0` | `ret(0x2fc44)` |

## `Dorado_i2cConfig`

- `Dorado_i2cConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39164 to 0x3936c.
  Established. A call to `0x395a0` carries candidates `0xe0ef0`, `0xe`, `0x38004`, `0x38030` (`bl`
  at 0x391cc). The stack receives word `0x43520000` at `sp -0x18` (`str` at 0x39264), before a call
  to `0x48d10` with candidates `0xe11dc`, `0x4`, `0xe120c`, `0x7` (`bl` at 0x39278). A zero byte at
  absolute `0x119dd1` takes the edge that bypasses two additional calls to `0xac18c`, while a
  nonzero byte falls through to those calls (`ldrb` at 0x392bc, `beq` at 0x392c4, `bl` at 0x392d8,
  `bl` at 0x392ec). The last call uses candidates `0xb7d04`, `0x0`, `0x15`, `0x15`, and its result
  reaches the return (`bl` at 0x39300, `ldmdb` at 0x39304). No device operation is established for
  the numeric callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x39174 | load | 4 | `abs 0x39308` | `-` |
| `ldr` at 0x3917c | load | 4 | `abs 0x3930c` | `-` |
| `ldr` at 0x3918c | load | 4 | `abs 0x39310` | `-` |
| `ldr` at 0x39198 | load | 4 | `abs 0x39314` | `-` |
| `ldr` at 0x391a4 | load | 4 | `abs 0x39318` | `-` |
| `ldr` at 0x391b0 | load | 4 | `abs 0x3931c` | `-` |
| `ldr` at 0x391bc | load | 4 | `abs 0x39320` | `-` |
| `ldr` at 0x391c4 | load | 4 | `abs 0x39324` | `-` |
| `ldr` at 0x391c8 | load | 4 | `abs 0x39328` | `-` |
| `ldr` at 0x391d0 | load | 4 | `abs 0x3932c` | `-` |
| `str` at 0x391e0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x391e8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x391ec | load | 4 | `abs 0x39330` | `-` |
| `ldr` at 0x391f4 | load | 4 | `abs 0x39334` | `-` |
| `ldr` at 0x39200 | load | 4 | `abs 0x39338` | `-` |
| `ldr` at 0x3920c | load | 4 | `abs 0x3933c` | `-` |
| `ldr` at 0x39218 | load | 4 | `abs 0x39340` | `-` |
| `ldr` at 0x39224 | load | 4 | `abs 0x39344` | `-` |
| `ldr` at 0x39244 | load | 4 | `abs 0x39348` | `-` |
| `ldr` at 0x39250 | load | 4 | `abs 0x3934c` | `-` |
| `ldr` at 0x39260 | load | 4 | `abs 0x39350` | `-` |
| `str` at 0x39264 | store | 4 | `sp -0x18` | `0x43520000` |
| `ldr` at 0x39268 | load | 4 | `abs 0x39354` | `-` |
| `ldr` at 0x39270 | load | 4 | `abs 0x39358` | `-` |
| `ldr` at 0x3927c | load | 4 | `abs 0x3935c` | `-` |
| `ldr` at 0x39290 | load | 4 | `abs 0x39360` | `-` |
| `ldr` at 0x392a4 | load | 4 | `abs 0x39364` | `-` |
| `ldr` at 0x392b8 | load | 4 | `abs 0x39368` | `-` |
| `ldrb` at 0x392bc | load | 1 | `abs 0x119dd1` | `-` |
| `ldr` at 0x392c8 | load | 4 | `abs 0x3935c` | `-` |
| `ldr` at 0x392dc | load | 4 | `abs 0x39360` | `-` |
| `ldr` at 0x392f0 | load | 4 | `abs 0x39364` | `-` |
| `ldmdb` at 0x39304 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39304 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39304 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39178 | `(unknown callee) 0xab3d8` | `0x38f90` | `r1` | `r2` | `r3` |
| `bl` at 0x39184 | `(unknown callee) 0xb61f4` | `0xe09a4` | `0x2` | `?` | `?` |
| `bl` at 0x39188 | `(unknown callee) 0xacda4` | `ret(0xb61f4)` | `?` | `?` | `?` |
| `bl` at 0x39194 | `(unknown callee) 0xb8c38` | `0xe09b4` | `0x2` | `?` | `?` |
| `bl` at 0x391a0 | `(unknown callee) 0xb7084` | `0xe0af4` | `0x16` | `?` | `?` |
| `bl` at 0x391ac | `(unknown callee) 0xb3fc8` | `0xe0cec` | `0x14` | `?` | `?` |
| `bl` at 0x391b8 | `(unknown callee) 0xafafc` | `0xe0e18` | `0x7` | `?` | `?` |
| `bl` at 0x391cc | `(unknown callee) 0x395a0` | `0xe0ef0` | `0xe` | `0x38004` | `0x38030` |
| `bl` at 0x391d8 | `(unknown callee) 0x2d664` | `0xe1174` | `0x7` | `?` | `?` |
| `bl` at 0x391fc | `(unknown callee) 0x4c4c8` | `0xe1314` | `0x1` | `0x38d54` | `0x0` |
| `bl` at 0x39208 | `(unknown callee) 0x2daac` | `0xe1040` | `0x4` | `?` | `?` |
| `bl` at 0x39214 | `(unknown callee) 0x41b04` | `0xe1060` | `0x5` | `?` | `?` |
| `bl` at 0x39220 | `(unknown callee) 0x419d8` | `0xe10f0` | `0xb` | `?` | `?` |
| `bl` at 0x3922c | `(unknown callee) 0x3e5ec` | `0xe127c` | `0x6` | `?` | `?` |
| `bl` at 0x39234 | `(unknown callee) 0x2bccc` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x39238 | `(unknown callee) 0x37d90` | `ret(0x2bccc)` | `?` | `?` | `?` |
| `bl` at 0x3923c | `(unknown callee) 0x3e2bc` | `ret(0x37d90)` | `?` | `?` | `?` |
| `bl` at 0x39240 | `(unknown callee) 0x37f60` | `ret(0x3e2bc)` | `?` | `?` | `?` |
| `bl` at 0x3924c | `(unknown callee) 0x2d0dc` | `0xe1328` | `0x3` | `?` | `?` |
| `bl` at 0x3925c | `(unknown callee) 0x45448` | `0xe11bc` | `0x2` | `0x1` | `?` |
| `bl` at 0x39278 | `(unknown callee) 0x48d10` | `0xe11dc` | `0x4` | `0xe120c` | `0x7` |
| `bl` at 0x3928c | `(unknown callee) 0xac18c` | `0xb9800` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x392a0 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x0` | `0x0` | `0x2` |
| `bl` at 0x392b4 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x5` | `0x0` | `0x3` |
| `bl` at 0x392d8 | `(unknown callee) 0xac18c` | `0xb9800` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x392ec | `(unknown callee) 0xac18c` | `0xb12dc` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x39300 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x0` | `0x15` | `0x15` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x392c4 | `[0x119dd1]:1 == 0x0` | `[0x119dd1]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39304 | `r0` | `ret(0xac18c)` |

## `Gemini_i2cConfig`

- `Gemini_i2cConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x329a8 to 0x32bac.
  Established. The call to `0x395a0` has candidates `0xde164`, `0x14`, `0x3193c`, `0x3195c` (`bl` at
  0x32a10). Two word stores put `0x0` at `sp -0x18` and `sp -0x14` (`str` at 0x32a24, `str` at
  0x32a2c), before a call to `0x4c4c8` with `0xde5b4`, `0x1`, `0x32598`, `0x0` (`bl` at 0x32a40). A
  zero byte at absolute `0x119d75` takes the edge that skips the calls at `0x32b0c` and `0x32b20`,
  with nonzero falling through (`ldrb` at 0x32af0, `beq` at 0x32af8). After the join, two calls to
  `0xac18c` use `0xb9800`, `0x2`, `0x0`, `0x0` and `0xb12dc`, `0x2`, `0x2`, `0x2` (`bl` at 0x32b34,
  `bl` at 0x32b48). Their physical configuration effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x329b8 | load | 4 | `abs 0x32b50` | `-` |
| `ldr` at 0x329c0 | load | 4 | `abs 0x32b54` | `-` |
| `ldr` at 0x329d0 | load | 4 | `abs 0x32b58` | `-` |
| `ldr` at 0x329dc | load | 4 | `abs 0x32b5c` | `-` |
| `ldr` at 0x329e8 | load | 4 | `abs 0x32b60` | `-` |
| `ldr` at 0x329f4 | load | 4 | `abs 0x32b64` | `-` |
| `ldr` at 0x32a00 | load | 4 | `abs 0x32b68` | `-` |
| `ldr` at 0x32a08 | load | 4 | `abs 0x32b6c` | `-` |
| `ldr` at 0x32a0c | load | 4 | `abs 0x32b70` | `-` |
| `ldr` at 0x32a14 | load | 4 | `abs 0x32b74` | `-` |
| `str` at 0x32a24 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x32a2c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x32a30 | load | 4 | `abs 0x32b78` | `-` |
| `ldr` at 0x32a38 | load | 4 | `abs 0x32b7c` | `-` |
| `ldr` at 0x32a44 | load | 4 | `abs 0x32b80` | `-` |
| `ldr` at 0x32a50 | load | 4 | `abs 0x32b84` | `-` |
| `ldr` at 0x32a5c | load | 4 | `abs 0x32b88` | `-` |
| `ldr` at 0x32a68 | load | 4 | `abs 0x32b8c` | `-` |
| `ldr` at 0x32a74 | load | 4 | `abs 0x32b90` | `-` |
| `ldr` at 0x32a94 | load | 4 | `abs 0x32b94` | `-` |
| `ldr` at 0x32aa4 | load | 4 | `abs 0x32b98` | `-` |
| `ldr` at 0x32ab0 | load | 4 | `abs 0x32b9c` | `-` |
| `ldr` at 0x32ac4 | load | 4 | `abs 0x32ba0` | `-` |
| `ldr` at 0x32ad8 | load | 4 | `abs 0x32b9c` | `-` |
| `ldr` at 0x32aec | load | 4 | `abs 0x32ba4` | `-` |
| `ldrb` at 0x32af0 | load | 1 | `abs 0x119d75` | `-` |
| `ldr` at 0x32afc | load | 4 | `abs 0x32ba8` | `-` |
| `ldr` at 0x32b10 | load | 4 | `abs 0x32ba0` | `-` |
| `ldr` at 0x32b24 | load | 4 | `abs 0x32ba8` | `-` |
| `ldr` at 0x32b38 | load | 4 | `abs 0x32ba0` | `-` |
| `ldmdb` at 0x32b4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x32b4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x32b4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x329bc | `(unknown callee) 0xab3d8` | `0x327d4` | `r1` | `r2` | `r3` |
| `bl` at 0x329c8 | `(unknown callee) 0xb61f4` | `0xddc30` | `0x2` | `?` | `?` |
| `bl` at 0x329cc | `(unknown callee) 0xacda4` | `ret(0xb61f4)` | `?` | `?` | `?` |
| `bl` at 0x329d8 | `(unknown callee) 0xb8c38` | `0xddc48` | `0x2` | `?` | `?` |
| `bl` at 0x329e4 | `(unknown callee) 0xb7084` | `0xddd5c` | `0x12` | `?` | `?` |
| `bl` at 0x329f0 | `(unknown callee) 0xb3fc8` | `0xddf14` | `0x15` | `?` | `?` |
| `bl` at 0x329fc | `(unknown callee) 0xafafc` | `0xde04c` | `0x7` | `?` | `?` |
| `bl` at 0x32a10 | `(unknown callee) 0x395a0` | `0xde164` | `0x14` | `0x3193c` | `0x3195c` |
| `bl` at 0x32a1c | `(unknown callee) 0x2d664` | `0xde45c` | `0x7` | `?` | `?` |
| `bl` at 0x32a40 | `(unknown callee) 0x4c4c8` | `0xde5b4` | `0x1` | `0x32598` | `0x0` |
| `bl` at 0x32a4c | `(unknown callee) 0x2daac` | `0xde344` | `0x5` | `?` | `?` |
| `bl` at 0x32a58 | `(unknown callee) 0x41b04` | `0xde36c` | `0x4` | `?` | `?` |
| `bl` at 0x32a64 | `(unknown callee) 0x419d8` | `0xde3f0` | `0x9` | `?` | `?` |
| `bl` at 0x32a70 | `(unknown callee) 0x3e5ec` | `0xde518` | `0x6` | `?` | `?` |
| `bl` at 0x32a7c | `(unknown callee) 0x41234` | `0xde5a8` | `0x1` | `?` | `?` |
| `bl` at 0x32a84 | `(unknown callee) 0x2bccc` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x32a88 | `(unknown callee) 0x315b0` | `ret(0x2bccc)` | `?` | `?` | `?` |
| `bl` at 0x32a8c | `(unknown callee) 0x3e2bc` | `ret(0x315b0)` | `?` | `?` | `?` |
| `bl` at 0x32a90 | `(unknown callee) 0x31898` | `ret(0x3e2bc)` | `?` | `?` | `?` |
| `bl` at 0x32aa0 | `(unknown callee) 0x45448` | `0xde4a4` | `0x2` | `0x1` | `?` |
| `bl` at 0x32aac | `(unknown callee) 0xbb204` | `0xde4d0` | `0x1` | `?` | `?` |
| `bl` at 0x32ac0 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x0` | `0x11` | `0x11` |
| `bl` at 0x32ad4 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x32ae8 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x5` | `0x0` | `0x3` |
| `bl` at 0x32b0c | `(unknown callee) 0xac18c` | `0xb9800` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x32b20 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x32b34 | `(unknown callee) 0xac18c` | `0xb9800` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x32b48 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x32af8 | `[0x119d75]:1 == 0x0` | `[0x119d75]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x32b4c | `r0` | `ret(0xac18c)` |

## `Vortix_i2cConfig`

- `Vortix_i2cConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30fe0 to 0x3129c.
  Established. The body includes a call to `0xafbfc` with first candidate `0x40` and all other
  candidates unknown (`bl` at 0x31038). It also calls `0x395a0` with `0xdd078`, `0x29`, `0x300bc`,
  `0x300e4` (`bl` at 0x3104c). Four calls to `0x2ef00` have first candidates `0x0`, `0x1`, `0x2` and
  `0x3` (`bl` at 0x31158, `bl` at 0x31194, `bl` at 0x311d0, `bl` at 0x3120c). Each following
  equality branch takes the zero-result bypass and falls through for a nonzero result (`beq` at
  0x31164, `beq` at 0x311a0, `beq` at 0x311dc, `beq` at 0x31218). The final fall-through pair of
  calls to `0xac18c` uses `0xb9800`, `0x5`, `0x5`, `0x5` and `0xb12dc`, `0x5`, `0xf`, `0x12` (`bl`
  at 0x3122c, `bl` at 0x31240). The predicates’ hardware meanings and merged return value are
  unknown (`ldmdb` at 0x31244).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x30ff0 | load | 4 | `abs 0x31248` | `-` |
| `ldr` at 0x30ff8 | load | 4 | `abs 0x3124c` | `-` |
| `ldr` at 0x31004 | load | 4 | `abs 0x31250` | `-` |
| `ldr` at 0x31010 | load | 4 | `abs 0x31254` | `-` |
| `ldr` at 0x3101c | load | 4 | `abs 0x31258` | `-` |
| `ldr` at 0x31028 | load | 4 | `abs 0x3125c` | `-` |
| `ldr` at 0x3103c | load | 4 | `abs 0x31260` | `-` |
| `ldr` at 0x31044 | load | 4 | `abs 0x31264` | `-` |
| `ldr` at 0x31048 | load | 4 | `abs 0x31268` | `-` |
| `ldr` at 0x31050 | load | 4 | `abs 0x3126c` | `-` |
| `ldr` at 0x3105c | load | 4 | `abs 0x31270` | `-` |
| `ldr` at 0x31068 | load | 4 | `abs 0x31274` | `-` |
| `str` at 0x31078 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x31080 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x31084 | load | 4 | `abs 0x31278` | `-` |
| `ldr` at 0x3108c | load | 4 | `abs 0x3127c` | `-` |
| `ldr` at 0x31098 | load | 4 | `abs 0x31280` | `-` |
| `ldr` at 0x310a4 | load | 4 | `abs 0x31284` | `-` |
| `ldr` at 0x310b0 | load | 4 | `abs 0x31288` | `-` |
| `ldr` at 0x310bc | load | 4 | `abs 0x3128c` | `-` |
| `ldr` at 0x310dc | load | 4 | `abs 0x31290` | `-` |
| `ldr` at 0x310f0 | load | 4 | `abs 0x31294` | `-` |
| `ldr` at 0x31104 | load | 4 | `abs 0x31298` | `-` |
| `ldr` at 0x31118 | load | 4 | `abs 0x31290` | `-` |
| `ldr` at 0x3112c | load | 4 | `abs 0x31294` | `-` |
| `ldr` at 0x31140 | load | 4 | `abs 0x31298` | `-` |
| `ldr` at 0x31168 | load | 4 | `abs 0x31290` | `-` |
| `ldr` at 0x3117c | load | 4 | `abs 0x31298` | `-` |
| `ldr` at 0x311a4 | load | 4 | `abs 0x31290` | `-` |
| `ldr` at 0x311b8 | load | 4 | `abs 0x31298` | `-` |
| `ldr` at 0x311e0 | load | 4 | `abs 0x31290` | `-` |
| `ldr` at 0x311f4 | load | 4 | `abs 0x31298` | `-` |
| `ldr` at 0x3121c | load | 4 | `abs 0x31290` | `-` |
| `ldr` at 0x31230 | load | 4 | `abs 0x31298` | `-` |
| `ldmdb` at 0x31244 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x31244 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x31244 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30ff4 | `(unknown callee) 0xab3d8` | `0x30dd0` | `r1` | `r2` | `r3` |
| `bl` at 0x31000 | `(unknown callee) 0xb61f4` | `0xdc090` | `0x5` | `?` | `?` |
| `bl` at 0x3100c | `(unknown callee) 0xb8c38` | `0xdc0c8` | `0x6` | `?` | `?` |
| `bl` at 0x31018 | `(unknown callee) 0xb7084` | `0xdc3e0` | `0x36` | `?` | `?` |
| `bl` at 0x31024 | `(unknown callee) 0xb3fc8` | `0xdca68` | `0x48` | `?` | `?` |
| `bl` at 0x31030 | `(unknown callee) 0xafafc` | `0xdce28` | `0x13` | `?` | `?` |
| `bl` at 0x31038 | `(unknown callee) 0xafbfc` | `0x40` | `?` | `?` | `?` |
| `bl` at 0x3104c | `(unknown callee) 0x395a0` | `0xdd078` | `0x29` | `0x300bc` | `0x300e4` |
| `bl` at 0x31058 | `(unknown callee) 0x2daac` | `0xdd450` | `0xe` | `?` | `?` |
| `bl` at 0x31064 | `(unknown callee) 0x2d664` | `0xdd508` | `0x13` | `?` | `?` |
| `bl` at 0x31070 | `(unknown callee) 0x32db0` | `0xdd980` | `0x2` | `?` | `?` |
| `bl` at 0x31094 | `(unknown callee) 0x4c4c8` | `0xdd930` | `0x4` | `0x30dac` | `0x0` |
| `bl` at 0x310a0 | `(unknown callee) 0x41b04` | `0xdd4c0` | `0x1` | `?` | `?` |
| `bl` at 0x310ac | `(unknown callee) 0x419d8` | `0xdd4e4` | `0x3` | `?` | `?` |
| `bl` at 0x310b8 | `(unknown callee) 0x3e5ec` | `0xdd6b0` | `0x1a` | `?` | `?` |
| `bl` at 0x310c4 | `(unknown callee) 0x41234` | `0xdd920` | `0x4` | `?` | `?` |
| `bl` at 0x310cc | `(unknown callee) 0x2bccc` | `0x9` | `?` | `?` | `?` |
| `bl` at 0x310d0 | `(unknown callee) 0x2fefc` | `ret(0x2bccc)` | `?` | `?` | `?` |
| `bl` at 0x310d4 | `(unknown callee) 0x3e2bc` | `ret(0x2fefc)` | `?` | `?` | `?` |
| `bl` at 0x310d8 | `(unknown callee) 0x2ffec` | `ret(0x3e2bc)` | `?` | `?` | `?` |
| `bl` at 0x310ec | `(unknown callee) 0xac18c` | `0xb9800` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x31100 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x31114 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x31128 | `(unknown callee) 0xac18c` | `0xb9800` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x3113c | `(unknown callee) 0xac18c` | `0xb7d04` | `0x1` | `0x9` | `0x9` |
| `bl` at 0x31150 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x1` | `0x2` | `0x2` |
| `bl` at 0x31158 | `(unknown callee) 0x2ef00` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x31178 | `(unknown callee) 0xac18c` | `0xb9800` | `0x2` | `0x2` | `0x2` |
| `bl` at 0x3118c | `(unknown callee) 0xac18c` | `0xb12dc` | `0x2` | `0x3` | `0x6` |
| `bl` at 0x31194 | `(unknown callee) 0x2ef00` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x311b4 | `(unknown callee) 0xac18c` | `0xb9800` | `0x3` | `0x3` | `0x3` |
| `bl` at 0x311c8 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x3` | `0x7` | `0xa` |
| `bl` at 0x311d0 | `(unknown callee) 0x2ef00` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x311f0 | `(unknown callee) 0xac18c` | `0xb9800` | `0x4` | `0x4` | `0x4` |
| `bl` at 0x31204 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x4` | `0xb` | `0xe` |
| `bl` at 0x3120c | `(unknown callee) 0x2ef00` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x3122c | `(unknown callee) 0xac18c` | `0xb9800` | `0x5` | `0x5` | `0x5` |
| `bl` at 0x31240 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x5` | `0xf` | `0x12` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x31164 | `ret(0x2ef00) == 0x0` | `ret(0x2ef00) != 0x0` |
| `beq` at 0x311a0 | `ret(0x2ef00) == 0x0` | `ret(0x2ef00) != 0x0` |
| `beq` at 0x311dc | `ret(0x2ef00) == 0x0` | `ret(0x2ef00) != 0x0` |
| `beq` at 0x31218 | `ret(0x2ef00) == 0x0` | `ret(0x2ef00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x31244 | `r0` | `?` |

## `OpusTio_i2cConfig`

- `OpusTio_i2cConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36f48 to 0x37188.
  Established. The call to `0x395a0` uses candidates `0xdfd94`, `0x13`, `0x35e78`, `0x35e98` (`bl`
  at 0x36fb0). A zero byte at absolute `0x119d8e` takes the edge that skips a call pair (`ldrb` at
  0x37038, `beq` at 0x37040). On the nonzero fall-through path, word `0x43520000` is stored at `sp
  -0x18` before a call to `0x48d10` with `0xe00c0`, `0x2`, `0xe00d8`, `0x4` (`str` at 0x37058, `bl`
  at 0x3706c). Later zero-byte tests at `0x119d8d` and `0x119d8e` take edges that bypass further
  call pairs (`ldrb` at 0x370b0, `beq` at 0x370b8, `ldrb` at 0x370e8, `beq` at 0x370f0). The return
  reports `ret(0xac18c)` (`ldmdb` at 0x3711c), but the callees’ configuration semantics remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x36f58 | load | 4 | `abs 0x37120` | `-` |
| `ldr` at 0x36f60 | load | 4 | `abs 0x37124` | `-` |
| `ldr` at 0x36f70 | load | 4 | `abs 0x37128` | `-` |
| `ldr` at 0x36f7c | load | 4 | `abs 0x3712c` | `-` |
| `ldr` at 0x36f88 | load | 4 | `abs 0x37130` | `-` |
| `ldr` at 0x36f94 | load | 4 | `abs 0x37134` | `-` |
| `ldr` at 0x36fa0 | load | 4 | `abs 0x37138` | `-` |
| `ldr` at 0x36fa8 | load | 4 | `abs 0x3713c` | `-` |
| `ldr` at 0x36fac | load | 4 | `abs 0x37140` | `-` |
| `ldr` at 0x36fb4 | load | 4 | `abs 0x37144` | `-` |
| `str` at 0x36fc4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x36fcc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x36fd0 | load | 4 | `abs 0x37148` | `-` |
| `ldr` at 0x36fd8 | load | 4 | `abs 0x3714c` | `-` |
| `ldr` at 0x36fe4 | load | 4 | `abs 0x37150` | `-` |
| `ldr` at 0x36ff0 | load | 4 | `abs 0x37154` | `-` |
| `ldr` at 0x36ffc | load | 4 | `abs 0x37158` | `-` |
| `ldr` at 0x37008 | load | 4 | `abs 0x3715c` | `-` |
| `ldr` at 0x37014 | load | 4 | `abs 0x37160` | `-` |
| `ldr` at 0x37034 | load | 4 | `abs 0x37164` | `-` |
| `ldrb` at 0x37038 | load | 1 | `abs 0x119d8e` | `-` |
| `ldr` at 0x37044 | load | 4 | `abs 0x37168` | `-` |
| `ldr` at 0x37054 | load | 4 | `abs 0x3716c` | `-` |
| `str` at 0x37058 | store | 4 | `sp -0x18` | `0x43520000` |
| `ldr` at 0x3705c | load | 4 | `abs 0x37170` | `-` |
| `ldr` at 0x37064 | load | 4 | `abs 0x37174` | `-` |
| `ldr` at 0x37070 | load | 4 | `abs 0x37178` | `-` |
| `ldr` at 0x37084 | load | 4 | `abs 0x3717c` | `-` |
| `ldr` at 0x37098 | load | 4 | `abs 0x37178` | `-` |
| `ldr` at 0x370ac | load | 4 | `abs 0x37180` | `-` |
| `ldrb` at 0x370b0 | load | 1 | `abs 0x119d8d` | `-` |
| `ldr` at 0x370bc | load | 4 | `abs 0x37184` | `-` |
| `ldr` at 0x370d0 | load | 4 | `abs 0x3717c` | `-` |
| `ldr` at 0x370e4 | load | 4 | `abs 0x37164` | `-` |
| `ldrb` at 0x370e8 | load | 1 | `abs 0x119d8e` | `-` |
| `ldr` at 0x370f4 | load | 4 | `abs 0x37184` | `-` |
| `ldr` at 0x37108 | load | 4 | `abs 0x3717c` | `-` |
| `ldmdb` at 0x3711c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3711c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3711c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36f5c | `(unknown callee) 0xab3d8` | `0x36d74` | `r1` | `r2` | `r3` |
| `bl` at 0x36f68 | `(unknown callee) 0xb61f4` | `0xdf81c` | `0x2` | `?` | `?` |
| `bl` at 0x36f6c | `(unknown callee) 0xacda4` | `ret(0xb61f4)` | `?` | `?` | `?` |
| `bl` at 0x36f78 | `(unknown callee) 0xb8c38` | `0xdf834` | `0x2` | `?` | `?` |
| `bl` at 0x36f84 | `(unknown callee) 0xb7084` | `0xdf960` | `0x14` | `?` | `?` |
| `bl` at 0x36f90 | `(unknown callee) 0xb3fc8` | `0xdfb40` | `0x16` | `?` | `?` |
| `bl` at 0x36f9c | `(unknown callee) 0xafafc` | `0xdfc84` | `0x7` | `?` | `?` |
| `bl` at 0x36fb0 | `(unknown callee) 0x395a0` | `0xdfd94` | `0x13` | `0x35e78` | `0x35e98` |
| `bl` at 0x36fbc | `(unknown callee) 0x2d664` | `0xe0074` | `0x7` | `?` | `?` |
| `bl` at 0x36fe0 | `(unknown callee) 0x4c4c8` | `0xe0188` | `0x1` | `0x36b38` | `0x0` |
| `bl` at 0x36fec | `(unknown callee) 0x2daac` | `0xdff5c` | `0x5` | `?` | `?` |
| `bl` at 0x36ff8 | `(unknown callee) 0x41b04` | `0xdff84` | `0x4` | `?` | `?` |
| `bl` at 0x37004 | `(unknown callee) 0x419d8` | `0xe0008` | `0x9` | `?` | `?` |
| `bl` at 0x37010 | `(unknown callee) 0x3e5ec` | `0xe011c` | `0x4` | `?` | `?` |
| `bl` at 0x3701c | `(unknown callee) 0x41234` | `0xe017c` | `0x1` | `?` | `?` |
| `bl` at 0x37024 | `(unknown callee) 0x2bccc` | `0x13` | `?` | `?` | `?` |
| `bl` at 0x37028 | `(unknown callee) 0x35ad4` | `ret(0x2bccc)` | `?` | `?` | `?` |
| `bl` at 0x3702c | `(unknown callee) 0x3e2bc` | `ret(0x35ad4)` | `?` | `?` | `?` |
| `bl` at 0x37030 | `(unknown callee) 0x35dd4` | `ret(0x3e2bc)` | `?` | `?` | `?` |
| `bl` at 0x37050 | `(unknown callee) 0x45448` | `0xe00b0` | `0x1` | `0x1` | `[0x119d8e]:1` |
| `bl` at 0x3706c | `(unknown callee) 0x48d10` | `0xe00c0` | `0x2` | `0xe00d8` | `0x4` |
| `bl` at 0x37080 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x0` | `0x13` | `0x13` |
| `bl` at 0x37094 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x370a8 | `(unknown callee) 0xac18c` | `0xb7d04` | `0x5` | `0x0` | `0x3` |
| `bl` at 0x370cc | `(unknown callee) 0xac18c` | `0xb9800` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x370e0 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x37104 | `(unknown callee) 0xac18c` | `0xb9800` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x37118 | `(unknown callee) 0xac18c` | `0xb12dc` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x37040 | `[0x119d8e]:1 == 0x0` | `[0x119d8e]:1 != 0x0` |
| `beq` at 0x370b8 | `[0x119d8d]:1 == 0x0` | `[0x119d8d]:1 != 0x0` |
| `beq` at 0x370f0 | `[0x119d8e]:1 == 0x0` | `[0x119d8e]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3711c | `r0` | `ret(0xac18c)` |

## `i2cMuxResetRaw`

- `i2cMuxResetRaw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6728 to 0xb6968.
  Established. Four byte stores address absolute `0x115828`, `0x115829`, `0x11582a` and `0x11582b`,
  each with unknown stored value (`strb` at 0xb67e8, `strb` at 0xb67f4, `strb` at 0xb68e8, `strb` at
  0xb68f4). A byte load from `0x115828` feeds an equality branch: `0xfe` takes the edge to the final
  load and return, while other values fall through to another call to `0xb6c08` (`ldrb` at
  0xb68fc, `beq` at 0xb6904, `bl` at 0xb6914). That call has first candidate `[0x115828]:1`, second
  unknown, third `0x11582b`, fourth `[0x115828]:1` (`bl` at 0xb6914). The return reports `[sp-20]`
  (`ldmdb` at 0xb6948). The absolute bytes’ device role and the reset operations performed by the
  callees are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb673c | store | 4 | `sp -0x14` | `0x0` |
| `strb` at 0xb6744 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xb6748 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb674c | load | 4 | `abs 0xb694c` | `-` |
| `ldr` at 0xb6750 | load | 4 | `abs 0x284120` | `-` |
| `ldrb` at 0xb6760 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb6764 | load | 4 | `abs 0xb6950` | `-` |
| `ldr` at 0xb6768 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb6770 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb677c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb6784 | load | 4 | `abs 0xb6954` | `-` |
| `ldr` at 0xb6788 | load | 4 | `abs 0x284100` | `-` |
| `ldrb` at 0xb6790 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb679c | load | 1 | `sp -0x19` | `-` |
| `str` at 0xb67ac | store | 4 | `sp -0x18` | `ret(0xb6c08)` |
| `ldr` at 0xb67b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb67bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb67c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb67cc | store | 4 | `sp -0x14` | `ret(0xb6c08)` |
| `ldrb` at 0xb67d0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xb67d8 | store | 1 | `sp -0x19` | `?` |
| `ldr` at 0xb67e0 | load | 4 | `abs 0xb6958` | `-` |
| `strb` at 0xb67e8 | store | 1 | `abs 0x115828` | `?` |
| `ldr` at 0xb67ec | load | 4 | `abs 0xb695c` | `-` |
| `strb` at 0xb67f4 | store | 1 | `abs 0x115829` | `?` |
| `strb` at 0xb67fc | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xb6800 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb6804 | load | 4 | `abs 0xb694c` | `-` |
| `ldr` at 0xb6808 | load | 4 | `abs 0x284120` | `-` |
| `ldrb` at 0xb6818 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb681c | load | 4 | `abs 0xb6950` | `-` |
| `ldr` at 0xb6820 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb6828 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb6834 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb683c | load | 4 | `abs 0xb6954` | `-` |
| `ldr` at 0xb6840 | load | 4 | `abs 0x284100` | `-` |
| `ldrb` at 0xb6848 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb6854 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb685c | load | 4 | `abs 0xb6954` | `-` |
| `ldr` at 0xb6860 | load | 4 | `abs 0x284100` | `-` |
| `ldrb` at 0xb6868 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb6878 | store | 4 | `sp -0x18` | `ret(0xab87c)` |
| `ldr` at 0xb687c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6894 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6898 | store | 4 | `sp -0x14` | `ret(0xab87c)` |
| `ldrb` at 0xb689c | load | 1 | `sp -0x19` | `-` |
| `str` at 0xb68ac | store | 4 | `sp -0x18` | `ret(0xb6c08)` |
| `ldr` at 0xb68b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb68bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb68c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb68cc | store | 4 | `sp -0x14` | `ret(0xb6c08)` |
| `ldrb` at 0xb68d0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xb68d8 | store | 1 | `sp -0x19` | `?` |
| `ldr` at 0xb68e0 | load | 4 | `abs 0xb6960` | `-` |
| `strb` at 0xb68e8 | store | 1 | `abs 0x11582a` | `?` |
| `ldr` at 0xb68ec | load | 4 | `abs 0xb6964` | `-` |
| `strb` at 0xb68f4 | store | 1 | `abs 0x11582b` | `?` |
| `ldr` at 0xb68f8 | load | 4 | `abs 0xb6958` | `-` |
| `ldrb` at 0xb68fc | load | 1 | `abs 0x115828` | `-` |
| `ldr` at 0xb6908 | load | 4 | `abs 0xb6958` | `-` |
| `ldrb` at 0xb690c | load | 1 | `abs 0x115828` | `-` |
| `str` at 0xb691c | store | 4 | `sp -0x18` | `ret(0xb6c08)` |
| `ldr` at 0xb6920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb692c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6938 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb693c | store | 4 | `sp -0x14` | `ret(0xb6c08)` |
| `ldr` at 0xb6940 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb6948 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6948 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6948 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb67a4 | `(unknown callee) 0xb6c08` | `?` | `?` | `?` | `?` |
| `bl` at 0xb6870 | `(unknown callee) 0xab87c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb68a4 | `(unknown callee) 0xb6c08` | `[sp-25]:1` | `?` | `?` | `[sp-25]:1` |
| `bl` at 0xb6914 | `(unknown callee) 0xb6c08` | `[0x115828]:1` | `?` | `0x11582b` | `[0x115828]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb6758 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb6778 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb6798 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb67b8 | `ret(0xb6c08) == 0x0` | `ret(0xb6c08) != 0x0` |
| `bne` at 0xb67c4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `blt` at 0xb6810 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb6830 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb6850 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb6884 | `ret(0xab87c) == 0x0` | `ret(0xab87c) != 0x0` |
| `bne` at 0xb6890 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xb68b8 | `ret(0xb6c08) == 0x0` | `ret(0xb6c08) != 0x0` |
| `bne` at 0xb68c4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xb6904 | `[0x115828]:1 == 0xfe` | `[0x115828]:1 != 0xfe` |
| `beq` at 0xb6928 | `ret(0xb6c08) == 0x0` | `ret(0xb6c08) != 0x0` |
| `bne` at 0xb6934 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6948 | `r0` | `[sp-20]` |

## `i2c_soft_l1_reset_check`

- `i2c_soft_l1_reset_check`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e300 to 0x3e4dc.
  Established. The early conditional computed transfer remains unresolved, with taken condition `?
  <= 0x26` unsigned and falls-through condition `? > 0x26` (`ldrls` at 0x3e328). The known
  fall-through path includes a call to `0x42710` with first candidate `0xff` (`bl` at 0x3e3dc),
  followed by `0x3a8c0` (`bl` at 0x3e3e8). A nonzero result of the latter takes the edge to a signed
  comparison (`bne` at 0x3e3fc). A negative value falls through that signed branch to a word store
  of `0xe1c40` at `[sp-20] +0x0`, while a nonnegative value takes the other edge (`bge` at 0x3e414,
  `str` at 0x3e420). Other stores to that target include `0xe1c5c`, `0xe1c80` and `0xe1cb0` (`str`
  at 0x3e43c, `str` at 0x3e470, `str` at 0x3e494). Reset eligibility semantics and computed-transfer
  destinations are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e310 | store | 4 | `sp -0x14` | `r0` |
| `ldrls` at 0x3e328 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x3e3e4 | store | 1 | `sp -0x15` | `(ret(0x42710) & 0xff)` |
| `str` at 0x3e3f0 | store | 4 | `sp -0x1c` | `ret(0x3a8c0)` |
| `ldr` at 0x3e3f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e404 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3e40c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e41c | load | 4 | `abs 0x3e4cc` | `-` |
| `str` at 0x3e420 | store | 4 | `[sp-20] +0x0` | `0xe1c40` |
| `ldrb` at 0x3e428 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3e434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e438 | load | 4 | `abs 0x3e4d0` | `-` |
| `str` at 0x3e43c | store | 4 | `[sp-20] +0x0` | `0xe1c5c` |
| `ldr` at 0x3e468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e46c | load | 4 | `abs 0x3e4d4` | `-` |
| `str` at 0x3e470 | store | 4 | `[sp-20] +0x0` | `0xe1c80` |
| `ldr` at 0x3e48c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e490 | load | 4 | `abs 0x3e4d8` | `-` |
| `str` at 0x3e494 | store | 4 | `[sp-20] +0x0` | `0xe1cb0` |
| `ldr` at 0x3e498 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e49c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x3e4ac | store | 4 | `sp -0x24` | `?` |
| `str` at 0x3e4b8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3e4bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3e4c0 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x3e4c4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3e4c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e4c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e4c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e314 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3e3dc | `(unknown callee) 0x42710` | `0xff` | `?` | `?` | `?` |
| `bl` at 0x3e3e8 | `(unknown callee) 0x3a8c0` | `ret(0x42710)` | `?` | `?` | `ret(0x42710)` |
| `bl` at 0x3e444 | `(unknown callee) 0x2bf8c` | `ret(0x3a8c0)` | `?` | `?` | `[sp-21]:1` |
| `bl` at 0x3e458 | `(unknown callee) 0x2c71c` | `0x0` | `?` | `?` | `ret(0x2bf8c)` |
| `bl` at 0x3e47c | `(unknown callee) 0x2c71c` | `0x2` | `?` | `?` | `ret(0x2c71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x3e328 | `? <= 0x26 (unsigned, 32-bit)` | `? > 0x26 (unsigned, 32-bit)` |
| `bne` at 0x3e3fc | `ret(0x3a8c0) != 0x0` | `ret(0x3a8c0) == 0x0` |
| `bge` at 0x3e414 | `ret(0x3a8c0) >= 0x0 (signed, 32-bit)` | `ret(0x3a8c0) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3e430 | `[sp-21]:1 != 0x0` | `[sp-21]:1 == 0x0` |
| `beq` at 0x3e450 | `ret(0x2bf8c) == 0x0` | `ret(0x2bf8c) != 0x0` |
| `beq` at 0x3e464 | `ret(0x2c71c) == 0x1` | `ret(0x2c71c) != 0x1` |
| `bne` at 0x3e488 | `ret(0x2c71c) != 0x1` | `ret(0x2c71c) == 0x1` |
| `beq` at 0x3e4a4 | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e4c8 | `r0` | `[sp-32]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3e328).

## `EnvFanInt`

- `EnvFanInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6712c to 0x673e0.
  Established. The body includes word stores of `0x1e` and `0xf` at `sp -0x1c` on separate paths
  (`str` at 0x672d0, `str` at 0x672dc). A later byte store increments the value at `[sp-36] +0x1`,
  with truncation to eight bits (`strb` at 0x672ec). The call to `0xb1198` has candidates
  `[[sp-32]+1]:1`, `[[sp-32]+2]:1`, `[[sp-32]+1]:1`, `[[sp-32]+2]:1` (`bl` at 0x67324). A nonzero
  byte at `[sp-36] +0x2` takes the bypass edge (`ldrb` at 0x6734c, `bne` at 0x67354). On the
  zero-byte path, byte `0x1` is stored at that offset before a call to `0x1fe64` (`strb` at 0x67380,
  `bl` at 0x67388). A zero result takes the edge past the byte store of `0x0`, while nonzero falls
  through to that store (`beq` at 0x67394, `strb` at 0x673a0). The counters’ time units and the
  external fan action are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x67144 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x6714c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x67154 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x67158 | load | 4 | `abs 0x673b4` | `-` |
| `ldr` at 0x6715c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67160 | load | 4 | `abs 0x11aa6c` | `-` |
| `ldr` at 0x67170 | load | 4 | `abs 0x673b4` | `-` |
| `ldr` at 0x67174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67188 | load | 4 | `abs 0x11aa78` | `-` |
| `str` at 0x67190 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x67194 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x67198 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x6719c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x671a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x671ac | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x671b0 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x671c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x671c8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x671d0 | load | 4 | `abs 0x673b4` | `-` |
| `ldr` at 0x671d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x671d8 | load | 4 | `abs 0x11aa6c` | `-` |
| `ldr` at 0x671e4 | load | 4 | `abs 0x673b8` | `-` |
| `str` at 0x671e8 | store | 4 | `sp -0x38` | `0xe7a20` |
| `ldrb` at 0x671ec | load | 1 | `sp -0x11` | `-` |
| `str` at 0x671f0 | store | 4 | `sp -0x34` | `(r0 & 0xff)` |
| `ldrb` at 0x671f4 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x671f8 | store | 4 | `sp -0x30` | `(r1 & 0xff)` |
| `ldr` at 0x67200 | load | 4 | `abs 0x673bc` | `-` |
| `ldr` at 0x67204 | load | 4 | `abs 0x673c0` | `-` |
| `str` at 0x67214 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6721c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67230 | load | 4 | `abs 0x673c4` | `-` |
| `str` at 0x67238 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6723c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x67240 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6724c | load | 4 | `abs 0x673b4` | `-` |
| `ldr` at 0x67250 | load | 4 | `abs 0x11aa5c` | `-` |
| `ldr` at 0x6725c | load | 4 | `abs 0x673b4` | `-` |
| `ldr` at 0x67260 | load | 4 | `abs 0x11aa60` | `-` |
| `ldrb` at 0x6726c | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x6727c | load | 4 | `abs 0x673c8` | `-` |
| `str` at 0x67280 | store | 4 | `sp -0x38` | `0xe7a54` |
| `str` at 0x67284 | store | 4 | `sp -0x34` | `ret(0x43064)` |
| `ldrb` at 0x67288 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6728c | store | 4 | `sp -0x30` | `[sp-17]:1` |
| `ldrb` at 0x67290 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x67294 | store | 4 | `sp -0x2c` | `[sp-18]:1` |
| `ldr` at 0x6729c | load | 4 | `abs 0x673bc` | `-` |
| `ldr` at 0x672a0 | load | 4 | `abs 0x673cc` | `-` |
| `str` at 0x672b0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x672d0 | store | 4 | `sp -0x1c` | `0x1e` |
| `str` at 0x672dc | store | 4 | `sp -0x1c` | `0xf` |
| `ldr` at 0x672e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x672e4 | load | 1 | `[sp-36] +0x1` | `-` |
| `strb` at 0x672ec | store | 1 | `[sp-36] +0x1` | `([[sp-36]+1]:1+1 & 0xff)` |
| `ldr` at 0x672f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x67304 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x6730c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x67310 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x67314 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x67318 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x67328 | load | 4 | `abs 0x673d0` | `-` |
| `ldr` at 0x6732c | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x6733c | load | 4 | `abs 0x673d4` | `-` |
| `ldr` at 0x67340 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67348 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x6734c | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x67358 | load | 4 | `abs 0x673d0` | `-` |
| `ldr` at 0x6735c | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x6736c | load | 4 | `abs 0x673d8` | `-` |
| `ldr` at 0x67370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67378 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x67380 | store | 1 | `[sp-36] +0x2` | `0x1` |
| `ldr` at 0x67384 | load | 4 | `abs 0x673dc` | `-` |
| `ldr` at 0x67398 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x673a0 | store | 1 | `[sp-36] +0x2` | `0x0` |
| `str` at 0x673a8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x673ac | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x673b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x673b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x673b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6720c | `(unknown callee) 0x1eed0` | `0x1` | `0xe7664` | `0x5d9` | `0x0` |
| `bl` at 0x67274 | `(unknown callee) 0x43064` | `[sp-24]:1` | `?` | `?` | `[sp-24]:1` |
| `bl` at 0x672a8 | `(unknown callee) 0x1eed0` | `0x1` | `0xe7664` | `0x5e6` | `0x0` |
| `bl` at 0x672b8 | `(unknown callee) 0x2b31c` | `r0` | `?` | `?` | `?` |
| `bl` at 0x67324 | `(unknown callee) 0xb1198` | `[[sp-32]+1]:1` | `[[sp-32]+2]:1` | `[[sp-32]+1]:1` | `[[sp-32]+2]:1` |
| `bl` at 0x67344 | `(unknown callee) 0x20f14` | `0xe7a84` | `[sp-24]` | `?` | `?` |
| `bl` at 0x67374 | `(unknown callee) 0x20f14` | `0xe7a98` | `[sp-24]` | `?` | `?` |
| `bl` at 0x67388 | `(unknown callee) 0x1fe64` | `0x11a7f4` | `?` | `[sp-36]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x67168 | `C == 0` | `not (C == 0)` |
| `bne` at 0x671a4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x671b8 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x671e0 | `C == 0` | `not (C == 0)` |
| `bne` at 0x67248 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x67258 | `[0x11aa5c] != 0x0` | `[0x11aa5c] == 0x0` |
| `bne` at 0x67268 | `[0x11aa60] != 0x0` | `[0x11aa60] == 0x0` |
| `bne` at 0x672c8 | `? != 0x47` | `? == 0x47` |
| `bge` at 0x672fc | `? >= [sp-28] (signed, 32-bit)` | `? < [sp-28] (signed, 32-bit)` |
| `beq` at 0x67338 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x67354 | `[[sp-36]+2]:1 != 0x0` | `[[sp-36]+2]:1 == 0x0` |
| `beq` at 0x67368 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x67394 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x673b0 | `r0` | `?` |

## `EnvTempInt`

- `EnvTempInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x673e0 to 0x67650.
  Established. Two calls to `0x2d7e0` use first candidate `[sp-28]:1`, with second and third
  candidates unknown and fourth `[sp-28]:1` (`bl` at 0x67588, `bl` at 0x675e4). A nonzero byte at
  `[sp-24] +0x1` takes the edge that bypasses the second call and both byte stores (`ldrb` at
  0x675b0, `bne` at 0x675b8). The zero-byte path includes a byte store of `0x1` at `[sp-24] +0x1`
  before calling `0x1fe64` with candidates `0x11a848`, unknown, `[sp-24]`, `0x1` (`strb` at 0x675f0,
  `bl` at 0x675f8). A zero return takes the edge past a byte store of `0x0` at the same offset,
  while nonzero falls through to it (`beq` at 0x67604, `strb` at 0x67610). Temperature units and
  hardware acknowledgement semantics are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x673f8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x67400 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x67408 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x6740c | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x67410 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67414 | load | 4 | `abs 0x11aa70` | `-` |
| `ldr` at 0x67424 | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x67428 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67430 | load | 4 | `abs 0x11aa7c` | `-` |
| `ldrb` at 0x67438 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x6743c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x67448 | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x6744c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67454 | load | 4 | `abs 0x11aa7c` | `-` |
| `ldrb` at 0x6745c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x67460 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x67470 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x67478 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x67480 | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x67484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67488 | load | 4 | `abs 0x11aa70` | `-` |
| `ldr` at 0x67494 | load | 4 | `abs 0x67628` | `-` |
| `str` at 0x67498 | store | 4 | `sp -0x30` | `0xe7aac` |
| `ldrb` at 0x6749c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x674a0 | store | 4 | `sp -0x2c` | `(r0 & 0xff)` |
| `ldrb` at 0x674a4 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x674a8 | store | 4 | `sp -0x28` | `(r1 & 0xff)` |
| `ldr` at 0x674b0 | load | 4 | `abs 0x6762c` | `-` |
| `ldr` at 0x674b4 | load | 4 | `abs 0x67630` | `-` |
| `str` at 0x674c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x674cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x674d4 | load | 4 | `abs 0x67634` | `-` |
| `str` at 0x674dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x674e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x674e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x674f0 | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x674f4 | load | 4 | `abs 0x11aa5c` | `-` |
| `ldr` at 0x67500 | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x67504 | load | 4 | `abs 0x11aa60` | `-` |
| `ldr` at 0x67510 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67514 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x67518 | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x6751c | load | 4 | `abs 0x67624` | `-` |
| `ldr` at 0x67520 | load | 4 | `abs 0x67638` | `-` |
| `ldr` at 0x67528 | load | 4 | `abs 0x11aa5c` | `-` |
| `ldr` at 0x6752c | load | 4 | `abs 0x11aa60` | `-` |
| `ldrb` at 0x67534 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x67544 | load | 4 | `abs 0x6763c` | `-` |
| `str` at 0x67548 | store | 4 | `sp -0x30` | `0xe7b08` |
| `str` at 0x6754c | store | 4 | `sp -0x2c` | `ret(0x2d92c)` |
| `ldrb` at 0x67550 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x67554 | store | 4 | `sp -0x28` | `[sp-17]:1` |
| `ldrb` at 0x67558 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x6755c | store | 4 | `sp -0x24` | `[sp-18]:1` |
| `ldr` at 0x67564 | load | 4 | `abs 0x6762c` | `-` |
| `str` at 0x67578 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x67580 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x6758c | load | 4 | `abs 0x67640` | `-` |
| `ldr` at 0x67590 | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x675a0 | load | 4 | `abs 0x67644` | `-` |
| `ldr` at 0x675a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x675ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x675b0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x675bc | load | 4 | `abs 0x67640` | `-` |
| `ldr` at 0x675c0 | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x675d0 | load | 4 | `abs 0x67648` | `-` |
| `ldr` at 0x675d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x675dc | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x675e8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x675f0 | store | 1 | `[sp-24] +0x1` | `0x1` |
| `ldr` at 0x675f4 | load | 4 | `abs 0x6764c` | `-` |
| `ldr` at 0x67608 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x67610 | store | 1 | `[sp-24] +0x1` | `0x0` |
| `str` at 0x67618 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x6761c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x67620 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x67620 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x67620 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x674bc | `(unknown callee) 0x1eed0` | `0x1` | `0xe7664` | `0x621` | `0x0` |
| `bl` at 0x67530 | `(unknown callee) 0x1dba4` | `0xe7ae8` | `?` | `[0x11aa5c]` | `[0x11aa60]` |
| `bl` at 0x6753c | `(unknown callee) 0x2d92c` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x67570 | `(unknown callee) 0x1eed0` | `0x1` | `0xe7664` | `0x630` | `0x0` |
| `bl` at 0x67588 | `(unknown callee) 0x2d7e0` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x675a8 | `(unknown callee) 0x20f14` | `0xe7b40` | `[sp-28]` | `?` | `?` |
| `bl` at 0x675d8 | `(unknown callee) 0x20f14` | `0xe7b54` | `[sp-28]` | `?` | `?` |
| `bl` at 0x675e4 | `(unknown callee) 0x2d7e0` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x675f8 | `(unknown callee) 0x1fe64` | `0x11a848` | `?` | `[sp-24]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x6741c | `C == 0` | `not (C == 0)` |
| `bne` at 0x67444 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x67468 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x67490 | `C == 0` | `not (C == 0)` |
| `bne` at 0x674ec | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x674fc | `[0x11aa5c] != 0x0` | `[0x11aa5c] == 0x0` |
| `bne` at 0x6750c | `[0x11aa60] != 0x0` | `[0x11aa60] == 0x0` |
| `beq` at 0x6759c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x675b8 | `[[sp-24]+1]:1 != 0x0` | `[[sp-24]+1]:1 == 0x0` |
| `beq` at 0x675cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x67604 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x67620 | `r0` | `?` |

## `env_pwrdwn`

- `env_pwrdwn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64eec to 0x65110.
  Established. The stack word at `sp -0x80` starts at `0x1e` and is later decremented by `0x5`
  (`str` at 0x64f20, `str` at 0x64fb8). The loop branch takes the repeat edge when `[sp-128] > 0x0`
  signed and falls through for a nonpositive value (`bgt` at 0x6507c). A call inside the loop
  targets `0x6bbec` with first candidate `0x1f4` (`bl` at 0x64fc0). After loop exit, the body stores
  word `0x0` at absolute `0x11560c`, calls `0x3a458` with candidates `0x4`, unknown, `0x11560c`,
  `0x0`, then stores word `0x1` at that same absolute address (`str` at 0x650c0, `bl` at 0x650c8,
  `str` at 0x650d4). The delay unit and the physical power effect of that call remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64efc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x64f00 | load | 4 | `abs 0x650e4` | `-` |
| `ldm` at 0x64f0c | load | 4 | `abs 0xe77dc` | `-` |
| `ldm` at 0x64f0c | load | 4 | `abs 0xe77e0` | `-` |
| `ldm` at 0x64f0c | load | 4 | `abs 0xe77e4` | `-` |
| `ldm` at 0x64f0c | load | 4 | `abs 0xe77e8` | `-` |
| `stm` at 0x64f10 | store | 4 | `sp -0x2c` | `[0xe77dc]` |
| `stm` at 0x64f10 | store | 4 | `sp -0x28` | `[0xe77e0]` |
| `stm` at 0x64f10 | store | 4 | `sp -0x24` | `[0xe77e4]` |
| `stm` at 0x64f10 | store | 4 | `sp -0x20` | `[0xe77e8]` |
| `ldm` at 0x64f14 | load | 4 | `abs 0xe77ec` | `-` |
| `ldm` at 0x64f14 | load | 4 | `abs 0xe77f0` | `-` |
| `stm` at 0x64f18 | store | 4 | `sp -0x1c` | `[0xe77ec]` |
| `stm` at 0x64f18 | store | 4 | `sp -0x18` | `[0xe77f0]` |
| `str` at 0x64f20 | store | 4 | `sp -0x80` | `0x1e` |
| `ldr` at 0x64f24 | load | 4 | `abs 0x650e8` | `-` |
| `ldr` at 0x64f4c | load | 4 | `abs 0x650ec` | `-` |
| `ldr` at 0x64f50 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x64f58 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x64f5c | load | 4 | `abs 0x650f0` | `-` |
| `str` at 0x64f90 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x64f98 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x64fa0 | load | 4 | `abs 0x650f4` | `-` |
| `ldr` at 0x64fa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64fb0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x64fb8 | store | 4 | `sp -0x80` | `[sp-128]-5` |
| `ldr` at 0x64fdc | load | 4 | `abs 0x650f8` | `-` |
| `str` at 0x64fec | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x64ff4 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x64ffc | load | 4 | `abs 0x650fc` | `-` |
| `ldr` at 0x65000 | load | 4 | `abs 0x65100` | `-` |
| `ldr` at 0x6500c | load | 4 | `abs 0x650e8` | `-` |
| `ldr` at 0x65018 | load | 4 | `abs 0x650e8` | `-` |
| `ldr` at 0x65038 | load | 4 | `abs 0x65104` | `-` |
| `str` at 0x65048 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x65050 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x65058 | load | 4 | `abs 0x650fc` | `-` |
| `ldr` at 0x6505c | load | 4 | `abs 0x65100` | `-` |
| `ldr` at 0x65068 | load | 4 | `abs 0x650e8` | `-` |
| `ldr` at 0x65074 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x65088 | load | 4 | `abs 0x65108` | `-` |
| `str` at 0x65098 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x650a0 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x650a8 | load | 4 | `abs 0x650fc` | `-` |
| `ldr` at 0x650b8 | load | 4 | `abs 0x6510c` | `-` |
| `str` at 0x650c0 | store | 4 | `abs 0x11560c` | `0x0` |
| `ldr` at 0x650cc | load | 4 | `abs 0x6510c` | `-` |
| `str` at 0x650d4 | store | 4 | `abs 0x11560c` | `0x1` |
| `ldr` at 0x650d8 | load | 4 | `abs 0x650e8` | `-` |
| `ldmdb` at 0x650e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x650e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x650e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64f2c | `(unknown callee) 0x1fb00` | `0x11a9ec` | `0x0` | `[0xe77e4]` | `0x1e` |
| `bl` at 0x64f54 | `(unknown callee) 0x18c74` | `sp-124` | `0xe77f4` | `[sp-128]` | `sp-124` |
| `bl` at 0x64fac | `(unknown callee) 0x636f8` | `0x2` | `0x4101` | `?` | `sp-124` |
| `bl` at 0x64fc0 | `(unknown callee) 0x6bbec` | `0x1f4` | `?` | `?` | `[sp-128]-5` |
| `bl` at 0x64fc4 | `(unknown callee) 0x3a8c0` | `ret(0x6bbec)` | `?` | `?` | `?` |
| `bl` at 0x64fe0 | `(unknown callee) 0x18c74` | `sp-124` | `0xe7818` | `?` | `sp-124` |
| `bl` at 0x65008 | `(unknown callee) 0x636f8` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x65010 | `(unknown callee) 0x1fe64` | `0x11a9ec` | `?` | `?` | `?` |
| `bl` at 0x65020 | `(unknown callee) 0x1fb00` | `0x11a9ec` | `0x0` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x6503c | `(unknown callee) 0x18c74` | `sp-124` | `0xe7840` | `?` | `sp-124` |
| `bl` at 0x65064 | `(unknown callee) 0x636f8` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x6506c | `(unknown callee) 0x1fe64` | `0x11a9ec` | `?` | `?` | `?` |
| `bl` at 0x6508c | `(unknown callee) 0x18c74` | `sp-124` | `0xe7870` | `?` | `sp-124` |
| `bl` at 0x650b4 | `(unknown callee) 0x636f8` | `0x2` | `0x4102` | `0x164` | `sp-124` |
| `bl` at 0x650c8 | `(unknown callee) 0x3a458` | `0x4` | `?` | `0x11560c` | `0x0` |
| `bl` at 0x650dc | `(unknown callee) 0x1fe64` | `0x11a9ec` | `?` | `0x11560c` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64f38 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `bne` at 0x64fd0 | `ret(0x3a8c0) != 0x0` | `ret(0x3a8c0) == 0x0` |
| `bne` at 0x6502c | `ret(0x1fb00) != 0x0` | `ret(0x1fb00) == 0x0` |
| `bgt` at 0x6507c | `[sp-128] > 0x0 (signed, 32-bit)` | `[sp-128] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x650e0 | `r0` | `?` |

## `EnvState`

- `EnvState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69e48 to 0x69f30.
  Established. Three decoded paths change absolute words `0x11aa5c` and `0x11aa60`. One writes `0x1`
  and `0x0`, then calls `0x6b748` (`str` at 0x69ec4, `str` at 0x69ed0, `bl` at 0x69ed4). Another
  writes `0x0` and `0x1`, then calls the same target (`str` at 0x69ee4, `str` at 0x69ef0, `bl` at
  0x69ef4). The third writes `0x0` at `0x11aa5c`, calls `0x6b748`, then writes `0x0` at `0x11aa60`
  (`str` at 0x69f04, `bl` at 0x69f08, `str` at 0x69f14). The equality branches selecting these paths
  retain only `Z == 1` as their taken predicate, so the selector values are unknown in the analysis
  (`beq` at 0x69ea4, `beq` at 0x69e8c, `beq` at 0x69eb4). The return reports `0x0` in r0 (`ldmdb` at
  0x69f20). The two absolute words’ hardware meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69e58 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x69e5c | load | 4 | `abs 0x69f24` | `-` |
| `ldr` at 0x69e60 | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x69e70 | load | 4 | `abs 0x69f28` | `-` |
| `ldr` at 0x69e74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69e7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x69e80 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x69e84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69e90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69e9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69ebc | load | 4 | `abs 0x69f2c` | `-` |
| `str` at 0x69ec4 | store | 4 | `abs 0x11aa5c` | `0x1` |
| `ldr` at 0x69ec8 | load | 4 | `abs 0x69f2c` | `-` |
| `str` at 0x69ed0 | store | 4 | `abs 0x11aa60` | `0x0` |
| `ldr` at 0x69edc | load | 4 | `abs 0x69f2c` | `-` |
| `str` at 0x69ee4 | store | 4 | `abs 0x11aa5c` | `0x0` |
| `ldr` at 0x69ee8 | load | 4 | `abs 0x69f2c` | `-` |
| `str` at 0x69ef0 | store | 4 | `abs 0x11aa60` | `0x1` |
| `ldr` at 0x69efc | load | 4 | `abs 0x69f2c` | `-` |
| `str` at 0x69f04 | store | 4 | `abs 0x11aa5c` | `0x0` |
| `ldr` at 0x69f0c | load | 4 | `abs 0x69f2c` | `-` |
| `str` at 0x69f14 | store | 4 | `abs 0x11aa60` | `0x0` |
| `ldmdb` at 0x69f20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x69f20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x69f20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69e78 | `(unknown callee) 0x20f14` | `0xe7ef8` | `r0` | `r2` | `?` |
| `bl` at 0x69ed4 | `(unknown callee) 0x6b748` | `?` | `?` | `0x11aa58` | `0x0` |
| `bl` at 0x69ef4 | `(unknown callee) 0x6b748` | `?` | `?` | `0x11aa58` | `0x1` |
| `bl` at 0x69f08 | `(unknown callee) 0x6b748` | `?` | `?` | `0x11aa58` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x69e6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x69e8c | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x69e98 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x69ea4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x69eb4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x69f20 | `r0` | `0x0` |

## `EnvAutoFanControl`

- `EnvAutoFanControl`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6baec to 0x6bb0c.
  Established. The body writes word `0x1` at absolute `0x11a774` (`str` at 0x6bb00). Entry r0
  remains the reported return value (`ldmdb` at 0x6bb04). No calls or conditional branches are
  reported. The write’s relationship to automatic fan control is not established within this
  routine.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6baf8 | load | 4 | `abs 0x6bb08` | `-` |
| `str` at 0x6bb00 | store | 4 | `abs 0x11a774` | `0x1` |
| `ldmdb` at 0x6bb04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6bb04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6bb04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6bb04 | `r0` | `r0` |

## `EnvTempConfigure`

- `EnvTempConfigure`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ba8c to 0x6bab8.
  Established. Entry r0 is saved at `sp -0x14` (`str` at 0x6ba9c) before a call to `0x66e68` with
  entry r0 through r3 as candidates (`bl` at 0x6baa0). The next call targets `0x66ba4` with
  candidates `[sp-20]`, `ret(0x66e68)`, unknown, `ret(0x66e68)` (`bl` at 0x6bab0); its result is
  reported in r0 at the return (`ldmdb` at 0x6bab4). The values’ temperature configuration meanings
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ba9c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6baa8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6bab4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6bab4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6bab4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6baa0 | `(unknown callee) 0x66e68` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6bab0 | `(unknown callee) 0x66ba4` | `[sp-20]` | `ret(0x66e68)` | `?` | `ret(0x66e68)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6bab4 | `r0` | `ret(0x66ba4)` |

## `EnvGetStatus`

- `EnvGetStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ba50 to 0x6ba8c.
  Established. A word load from absolute `0x115608` supplies the value stored at entry `r0 +0x0`
  (`ldr` at 0x6ba6c, `str` at 0x6ba70). A later word load addresses `0x11aa70` (`ldr` at
  0x6ba78), and the return reports `[0x11aa70]` in r0 (`ldmdb` at 0x6ba80). No call or conditional
  branch is reported. The copied word’s status-bit interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ba60 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ba64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ba68 | load | 4 | `abs 0x6ba84` | `-` |
| `ldr` at 0x6ba6c | load | 4 | `abs 0x115608` | `-` |
| `str` at 0x6ba70 | store | 4 | `r0 +0x0` | `[0x115608]` |
| `ldr` at 0x6ba74 | load | 4 | `abs 0x6ba88` | `-` |
| `ldr` at 0x6ba78 | load | 4 | `abs 0x11aa70` | `-` |
| `ldmdb` at 0x6ba80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ba80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ba80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ba80 | `r0` | `[0x11aa70]` |
