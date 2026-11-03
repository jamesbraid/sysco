# ARM link and controller communication paths in 1.44.0

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
The computed transfer in SysComInit remains unresolved; its tables cover known edges only.
The affected entry identifies its computed transfer.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/link-controller-communication-facts.txt) retain all tool sections.

## `LinkErrChangeFreq`

- `LinkErrChangeFreq`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35d68 to 0x35ddc.
  Established. The absolute word at `0x12d6f0` receives entry `r0` (`str` at 0x35d84). The signed
  condition `[0x12d6f0] <= 0x176f` takes `0x35dac`, while greater values fall through (`ble` at 0x35d98). Included assignments at `0x12d6f4` write zero and `ret(0xeb174)` in separate arms (`str` at 0x35da4, `str` at 0x35dc4). Link sampling frequency meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x35d78 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x35d7c | load | 4 | `abs 0x35dcc` | `-` |
| `ldr` at 0x35d80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x35d84 | store | 4 | `abs 0x12d6f0` | `r0` |
| `ldr` at 0x35d88 | load | 4 | `abs 0x35dcc` | `-` |
| `ldr` at 0x35d8c | load | 4 | `abs 0x12d6f0` | `-` |
| `ldr` at 0x35d90 | load | 4 | `abs 0x35dd0` | `-` |
| `ldr` at 0x35d9c | load | 4 | `abs 0x35dd4` | `-` |
| `str` at 0x35da4 | store | 4 | `abs 0x12d6f4` | `0x0` |
| `ldr` at 0x35dac | load | 4 | `abs 0x35dd4` | `-` |
| `ldr` at 0x35db0 | load | 4 | `abs 0x35dcc` | `-` |
| `ldr` at 0x35db4 | load | 4 | `abs 0x35dd8` | `-` |
| `ldr` at 0x35db8 | load | 4 | `abs 0x12d6f0` | `-` |
| `str` at 0x35dc4 | store | 4 | `abs 0x12d6f4` | `ret(0xeb174)` |
| `ldm` at 0x35dc8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x35dc8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x35dc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x35dc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x35dc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35dbc | `(unknown callee) 0xeb174` | `0x1770` | `[0x12d6f0]` | `[0x12d6f0]` | `0x12d6f0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x35d98 | `[0x12d6f0] <= 0x176f (signed, 32-bit)` | `[0x12d6f0] > 0x176f (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x35dc8 | `r0` | `?` |

## `LinkErrActive`

- `LinkErrActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35ddc to 0x35e44.
  Established. The unnamed `0x1d444` call carries `0x9b` and three entry register candidates (`bl` at 0x35df0). A zero `(ret(0x1d444) & 0xff)` takes `0x35e2c`, while nonzero falls through (`beq` at 0x35e10). Separate included assignments at `sp -0x18` write `0x1` and zero (`str` at 0x35e24,
  `str` at 0x35e30). Link activity policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x35df8 | store | 1 | `sp -0x11` | `(ret(0x1d444) & 0xff)` |
| `ldr` at 0x35dfc | load | 4 | `abs 0x35e40` | `-` |
| `ldr` at 0x35e00 | load | 4 | `abs 0x12d6ec` | `-` |
| `strb` at 0x35e04 | store | 1 | `sp -0x12` | `([0x12d6ec] & 0xff)` |
| `ldrb` at 0x35e08 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x35e14 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x35e24 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x35e30 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x35e34 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x35e3c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x35e3c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x35e3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35df0 | `(unknown callee) 0x1d444` | `0x9b` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x35e10 | `(ret(0x1d444) & 0xff) == 0x0` | `(ret(0x1d444) & 0xff) != 0x0` |
| `beq` at 0x35e1c | `([0x12d6ec] & 0xff) == 0x0` | `([0x12d6ec] & 0xff) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x35e3c | `r0` | `?` |

## `LinkErrTaskInit`

- `LinkErrTaskInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35cec to 0x35d68.
  Established. The unnamed `0x2066c` call carries `0x132190`, `0xf53bc`, `0x35344`, `0x1` (`bl` at 0x35d48). The words at `sp -0x20` and `sp -0x14` receive `0x800` and `0x9` (`str` at 0x35d1c,
  `str` at 0x35d34). The included return record gives `ret(0x2066c)` in `r0` (`ldm` at 0x35d50).
  Link task setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x35cfc | load | 4 | `abs 0x35d54` | `-` |
| `ldr` at 0x35d00 | load | 4 | `abs 0x35d58` | `-` |
| `str` at 0x35d14 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x35d1c | store | 4 | `sp -0x20` | `0x800` |
| `str` at 0x35d24 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x35d2c | store | 4 | `sp -0x18` | `0xa` |
| `str` at 0x35d34 | store | 4 | `sp -0x14` | `0x9` |
| `ldr` at 0x35d38 | load | 4 | `abs 0x35d5c` | `-` |
| `ldr` at 0x35d3c | load | 4 | `abs 0x35d60` | `-` |
| `ldr` at 0x35d40 | load | 4 | `abs 0x35d64` | `-` |
| `ldm` at 0x35d50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x35d50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x35d50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35d0c | `(unknown callee) 0x20bb4` | `0x13213c` | `0xf53b0` | `0x0` | `0x9` |
| `bl` at 0x35d48 | `(unknown callee) 0x2066c` | `0x132190` | `0xf53bc` | `0x35344` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x35d50 | `r0` | `ret(0x2066c)` |

## `LinkErrReadStatus`

- `LinkErrReadStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34c90 to 0x34ebc.
  Established. The byte through entry `r1 +0x0` receives `0x1` (`strb` at 0x34cc4). Included byte
  assignments through `[sp-24] +0x1` and `[sp-24] +0x2` write `0x1` at separate sites (`strb` at 0x34e44, `strb` at 0x34e84). The unnamed `0xde16c` call at `0x34cfc` carries `[sp-36]`, `0x1000`,
  `sp-48`, `sp-52` (`bl` at 0x34cfc). Link error status meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34ca4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x34ca8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x34cb0 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x34cb8 | store | 4 | `sp -0x24` | `0x600` |
| `ldr` at 0x34cbc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x34cc4 | store | 1 | `r1 +0x0` | `0x1` |
| `ldr` at 0x34cc8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x34cd0 | store | 1 | `[sp-24] +0x1` | `0x0` |
| `ldr` at 0x34cd4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x34cdc | store | 1 | `[sp-24] +0x2` | `0x0` |
| `ldr` at 0x34cec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34d04 | store | 4 | `sp -0x1c` | `ret(0xde16c)` |
| `ldr` at 0x34d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x34d18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34d1c | store | 4 | `sp -0x40` | `[sp-28]` |
| `ldr` at 0x34d28 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x34d38 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x34d48 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x34d4c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x34d54 | load | 4 | `abs 0x34eb0` | `-` |
| `str` at 0x34d5c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x34d6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34d70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x34d84 | store | 4 | `sp -0x1c` | `ret(0xde16c)` |
| `ldr` at 0x34d88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34d9c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x34dac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34db0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x34dc4 | store | 4 | `sp -0x1c` | `ret(0xde16c)` |
| `ldr` at 0x34dc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34ddc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x34de8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x34dfc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x34e04 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x34e0c | store | 1 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x34e10 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x34e20 | load | 4 | `abs 0x34eb4` | `-` |
| `ldrb` at 0x34e24 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x34e28 | load | 4 | `abs 0x34eb4` | `-` |
| `ldrb` at 0x34e2c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x34e30 | load | 4 | `(unknown)` | `-` |
| `str` at 0x34e38 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x34e3c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x34e44 | store | 1 | `[sp-24] +0x1` | `0x1` |
| `str` at 0x34e4c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x34e50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x34e60 | load | 4 | `abs 0x34eb8` | `-` |
| `ldrb` at 0x34e64 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x34e68 | load | 4 | `abs 0x34eb8` | `-` |
| `ldrb` at 0x34e6c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x34e70 | load | 4 | `(unknown)` | `-` |
| `str` at 0x34e78 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x34e7c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x34e84 | store | 1 | `[sp-24] +0x2` | `0x1` |
| `str` at 0x34e8c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x34e90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34e94 | store | 4 | `sp -0x40` | `[sp-32]` |
| `str` at 0x34ea0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x34ea4 | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0x34eac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x34eac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x34eac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34ce0 | `(unknown callee) 0xd708c` | `r0` | `r1` | `[sp-24]` | `0x0` |
| `bl` at 0x34cfc | `(unknown callee) 0xde16c` | `[sp-36]` | `0x1000` | `sp-48` | `sp-52` |
| `bl` at 0x34d14 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34d24 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34d60 | `(unknown callee) 0xd708c` | `ret(0xd70fc)` | `?` | `?` | `?` |
| `bl` at 0x34d7c | `(unknown callee) 0xde16c` | `[sp-36]` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0x34d94 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34dbc | `(unknown callee) 0xde16c` | `[sp-36]` | `[sp-44]` | `sp-56` | `sp-60` |
| `bl` at 0x34dd4 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |
| `bl` at 0x34de4 | `(unknown callee) 0xd70fc` | `ret(0xde16c)` | `?` | `?` | `ret(0xde16c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x34d10 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x34d34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x34d90 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `beq` at 0x34dd0 | `ret(0xde16c) == 0x0` | `ret(0xde16c) != 0x0` |
| `bne` at 0x34df4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x34e1c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x34e5c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x34eac | `r0` | `?` |

## `LinkErrMonitor`

- `LinkErrMonitor`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35344 to 0x35670.
  Established. Included absolute word assignments at `0x132130`, `0x132134`, `0x132138` write zero
  (`str` at 0x353e4, `str` at 0x353f0, `str` at 0x353fc). A zero `ret(0x35ddc)` takes `0x3560c`,
  while nonzero falls through (`beq` at 0x35370). The unnamed `0x6f93c` call at `0x355ec` carries
  `0x1`, `0x8102`, `0x0`, `sp-268` (`bl` at 0x355ec). Link error monitoring policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x35354 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x35358 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x35360 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x35384 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x35390 | load | 4 | `abs 0x35634` | `-` |
| `str` at 0x3539c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x353a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x353ac | load | 4 | `abs 0x35638` | `-` |
| `ldr` at 0x353b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x353b8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x353bc | load | 4 | `abs 0x3563c` | `-` |
| `ldr` at 0x353c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x353c8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x353cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x353d4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x353dc | load | 4 | `abs 0x35640` | `-` |
| `str` at 0x353e4 | store | 4 | `abs 0x132130` | `0x0` |
| `ldr` at 0x353e8 | load | 4 | `abs 0x35644` | `-` |
| `str` at 0x353f0 | store | 4 | `abs 0x132134` | `0x0` |
| `ldr` at 0x353f4 | load | 4 | `abs 0x35648` | `-` |
| `str` at 0x353fc | store | 4 | `abs 0x132138` | `0x0` |
| `str` at 0x35404 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x35408 | load | 4 | `abs 0x3564c` | `-` |
| `str` at 0x35414 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x35418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x35424 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35440 | store | 4 | `sp -0x20` | `ret(0x34c90)` |
| `ldr` at 0x35444 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35454 | store | 4 | `sp -0x110` | `sp-268` |
| `ldr` at 0x35458 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35460 | store | 4 | `sp -0x114` | `[sp-28]+65` |
| `ldrb` at 0x35464 | load | 1 | `sp -0x43` | `-` |
| `ldr` at 0x35470 | load | 4 | `abs 0x35650` | `-` |
| `str` at 0x35474 | store | 4 | `sp -0x118` | `0xf50dc` |
| `ldr` at 0x3547c | load | 4 | `abs 0x35654` | `-` |
| `str` at 0x35480 | store | 4 | `sp -0x118` | `0xf50e0` |
| `ldrb` at 0x35484 | load | 1 | `sp -0x42` | `-` |
| `ldr` at 0x35490 | load | 4 | `abs 0x35650` | `-` |
| `str` at 0x35494 | store | 4 | `sp -0x11c` | `0xf50dc` |
| `ldr` at 0x3549c | load | 4 | `abs 0x35654` | `-` |
| `str` at 0x354a0 | store | 4 | `sp -0x11c` | `0xf50e0` |
| `ldr` at 0x354a4 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x354a8 | store | 4 | `sp -0x124` | `[sp-284]` |
| `ldr` at 0x354ac | load | 4 | `sp -0x110` | `-` |
| `ldr` at 0x354b0 | load | 4 | `abs 0x35658` | `-` |
| `ldr` at 0x354b4 | load | 4 | `sp -0x114` | `-` |
| `ldr` at 0x354b8 | load | 4 | `sp -0x118` | `-` |
| `ldrb` at 0x354c0 | load | 1 | `sp -0x43` | `-` |
| `str` at 0x354d4 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x354dc | store | 4 | `sp -0x120` | `0x0` |
| `ldrb` at 0x354f0 | load | 1 | `sp -0x42` | `-` |
| `str` at 0x35504 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x3550c | store | 4 | `sp -0x120` | `0x0` |
| `ldr` at 0x35514 | load | 4 | `abs 0x3565c` | `-` |
| `ldr` at 0x3552c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35534 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x35540 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x35544 | load | 4 | `abs 0x35660` | `-` |
| `str` at 0x35550 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x35554 | load | 4 | `abs 0x12d6f4` | `-` |
| `str` at 0x35564 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x3556c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x35570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3557c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x355a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x355ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x355b0 | store | 4 | `sp -0x124` | `[sp-44]` |
| `ldr` at 0x355b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x355bc | load | 4 | `abs 0x35664` | `-` |
| `str` at 0x355d0 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x355d8 | store | 4 | `sp -0x120` | `0x0` |
| `ldr` at 0x355e0 | load | 4 | `abs 0x35668` | `-` |
| `ldr` at 0x355fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35604 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x35620 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x35624 | load | 4 | `abs 0x3566c` | `-` |
| `ldr` at 0x35628 | load | 4 | `abs 0x12d6f0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35364 | `(unknown callee) 0x35ddc` | `?` | `?` | `?` | `?` |
| `bl` at 0x35374 | `(unknown callee) 0x375e4` | `ret(0x35ddc)` | `?` | `?` | `ret(0x35ddc)` |
| `bl` at 0x35394 | `(unknown callee) 0x1e4ac` | `0xf50c0` | `?` | `?` | `[sp-36]` |
| `bl` at 0x3540c | `(unknown callee) 0x35670` | `0x1770` | `?` | `0x132138` | `0x0` |
| `bl` at 0x35438 | `(unknown callee) 0x34c90` | `?` | `sp-68` | `sp-68` | `?` |
| `bl` at 0x354bc | `(unknown callee) 0x197a8` | `sp-268` | `0xf50e4` | `[sp-28]+65` | `[sp-280]` |
| `bl` at 0x354ec | `(unknown callee) 0x6f93c` | `0x1` | `0x8100` | `0x0` | `sp-268` |
| `bl` at 0x3551c | `(unknown callee) 0x6f93c` | `0x1` | `0x8101` | `0x0` | `sp-268` |
| `bl` at 0x35528 | `(unknown callee) 0x1e4ac` | `sp-268` | `?` | `?` | `sp-268` |
| `bl` at 0x35590 | `(unknown callee) 0x34ebc` | `?` | `sp-64` | `sp-64` | `?` |
| `bl` at 0x355c4 | `(unknown callee) 0x197a8` | `sp-268` | `0xf5118` | `[sp-28]+65` | `[sp-48]` |
| `bl` at 0x355ec | `(unknown callee) 0x6f93c` | `0x1` | `0x8102` | `0x0` | `sp-268` |
| `bl` at 0x355f8 | `(unknown callee) 0x1e4ac` | `sp-268` | `?` | `?` | `sp-268` |
| `bl` at 0x3560c | `(unknown callee) 0x375e4` | `?` | `?` | `?` | `?` |
| `bl` at 0x3562c | `(unknown callee) 0x35670` | `[0x12d6f0]` | `?` | `?` | `0x12d6f0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x35370 | `ret(0x35ddc) == 0x0` | `ret(0x35ddc) != 0x0` |
| `beq` at 0x35380 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `beq` at 0x3538c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bgt` at 0x353a8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0x35420 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x3544c | `ret(0x34c90) == 0x0` | `ret(0x34c90) != 0x0` |
| `beq` at 0x3546c | `[sp-67]:1 == 0x0` | `[sp-67]:1 != 0x0` |
| `beq` at 0x3548c | `[sp-66]:1 == 0x0` | `[sp-66]:1 != 0x0` |
| `beq` at 0x354c8 | `[sp-67]:1 == 0x0` | `[sp-67]:1 != 0x0` |
| `beq` at 0x354f8 | `[sp-66]:1 == 0x0` | `[sp-66]:1 != 0x0` |
| `blt` at 0x3555c | `[sp-40] < [0x12d6f4] (signed, 32-bit)` | `[sp-40] >= [0x12d6f4] (signed, 32-bit)` |
| `bgt` at 0x35578 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x3559c | `ret(0x34ebc) != 0x1` | `ret(0x34ebc) == 0x1` |
| `bne` at 0x35618 | `ret(0x375e4) != 0x0` | `ret(0x375e4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `SysComReset`

- `SysComReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x77234 to 0x773ac.
  Established. The absolute word at `0x29a5e4` is read (`ldr` at 0x77250). The unnamed `0xadd98`
  calls at `0x77314` and `0x77390` carry unknown, `0x3`, unknown, unknown (`bl` at 0x77314, `bl` at 0x77390). The word at `sp -0x14` receives `[sp-20]+1` in the loop (`str` at 0x7739c). Controller
  communication reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x77248 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x7724c | load | 4 | `abs 0x773a8` | `-` |
| `ldr` at 0x77250 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0x77254 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x77260 | load | 4 | `abs 0x773a8` | `-` |
| `ldr` at 0x77264 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x77288 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x77290 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7729c | load | 4 | `abs 0x773a8` | `-` |
| `ldr` at 0x772a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x772c4 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x772cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x772d8 | load | 4 | `abs 0x773a8` | `-` |
| `ldr` at 0x772dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x77300 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x77318 | load | 4 | `abs 0x773a8` | `-` |
| `ldr` at 0x7731c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x77340 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x77348 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x77354 | load | 4 | `abs 0x773a8` | `-` |
| `ldr` at 0x77358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7737c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x77394 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7739c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0x773a4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x773a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x773a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x773a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x77314 | `(unknown callee) 0xadd98` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x77350 | `(unknown callee) 0xbdb28` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x77390 | `(unknown callee) 0xadd98` | `?` | `0x3` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x7725c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x77298 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x772d4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x773a4 | `r0` | `?` |

## `SysComThrottle`

- `SysComThrottle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x773ac to 0x775b0.
  Established. A nonzero word at `0x13384c` takes `0x773d4`, while zero falls through (`bne` at 0x773cc). The absolute word at `0x29a644` receives `[sp-20]` and zero on separate included paths
  (`str` at 0x7747c, `str` at 0x7754c). The word at `0x29a648` receives `[0x29a648]+1` (`str` at 0x77490). Controller throttling behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x773bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x773c0 | load | 4 | `abs 0x77598` | `-` |
| `ldr` at 0x773c4 | load | 4 | `abs 0x13384c` | `-` |
| `ldr` at 0x773d4 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x773d8 | load | 4 | `abs 0x29a644` | `-` |
| `ldr` at 0x773e4 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x773e8 | load | 4 | `abs 0x29a644` | `-` |
| `ldr` at 0x773ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x773fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x77408 | store | 4 | `sp -0x1c` | `ret(0x21da8)` |
| `ldr` at 0x7740c | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x77410 | load | 4 | `abs 0x29a650` | `-` |
| `ldr` at 0x77414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x77420 | load | 4 | `abs 0x775a0` | `-` |
| `str` at 0x77430 | store | 4 | `sp -0x18` | `ret(0x20cd0)` |
| `ldr` at 0x77434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x77440 | load | 4 | `abs 0x775a4` | `-` |
| `str` at 0x77444 | store | 4 | `sp -0x24` | `0xfe3e0` |
| `ldr` at 0x77448 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7744c | store | 4 | `sp -0x20` | `ret(0x20cd0)` |
| `ldr` at 0x77454 | load | 4 | `abs 0x775a8` | `-` |
| `ldr` at 0x77464 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x77468 | load | 4 | `abs 0x29a644` | `-` |
| `ldr` at 0x77474 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x77478 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7747c | store | 4 | `abs 0x29a644` | `[sp-20]` |
| `ldr` at 0x77480 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x77484 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x77488 | load | 4 | `abs 0x29a648` | `-` |
| `str` at 0x77490 | store | 4 | `abs 0x29a648` | `[0x29a648]+1` |
| `ldr` at 0x77494 | load | 4 | `abs 0x775a0` | `-` |
| `str` at 0x774a0 | store | 4 | `sp -0x18` | `ret(0x2105c)` |
| `ldr` at 0x774a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x774b0 | load | 4 | `abs 0x775ac` | `-` |
| `str` at 0x774b4 | store | 4 | `sp -0x24` | `0xfe424` |
| `ldr` at 0x774b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x774bc | store | 4 | `sp -0x20` | `ret(0x2105c)` |
| `ldr` at 0x774c4 | load | 4 | `abs 0x775a8` | `-` |
| `ldr` at 0x774d8 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x774dc | load | 4 | `abs 0x29a654` | `-` |
| `ldr` at 0x774e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x774ec | load | 4 | `abs 0x775a0` | `-` |
| `str` at 0x774fc | store | 4 | `sp -0x18` | `ret(0x20cd0)` |
| `ldr` at 0x77500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7750c | load | 4 | `abs 0x775a4` | `-` |
| `str` at 0x77510 | store | 4 | `sp -0x24` | `0xfe3e0` |
| `ldr` at 0x77514 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x77518 | store | 4 | `sp -0x20` | `ret(0x20cd0)` |
| `ldr` at 0x77520 | load | 4 | `abs 0x775a8` | `-` |
| `ldr` at 0x77530 | load | 4 | `abs 0x7759c` | `-` |
| `ldr` at 0x77534 | load | 4 | `abs 0x29a644` | `-` |
| `ldr` at 0x77538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x77544 | load | 4 | `abs 0x7759c` | `-` |
| `str` at 0x7754c | store | 4 | `abs 0x29a644` | `0x0` |
| `ldr` at 0x77550 | load | 4 | `abs 0x775a0` | `-` |
| `str` at 0x7755c | store | 4 | `sp -0x18` | `ret(0x2105c)` |
| `ldr` at 0x77560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7756c | load | 4 | `abs 0x775ac` | `-` |
| `str` at 0x77570 | store | 4 | `sp -0x24` | `0xfe424` |
| `ldr` at 0x77574 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x77578 | store | 4 | `sp -0x20` | `ret(0x2105c)` |
| `ldr` at 0x77580 | load | 4 | `abs 0x775a8` | `-` |
| `ldm` at 0x77594 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x77594 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x77594 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x77400 | `(unknown callee) 0x21da8` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x77428 | `(unknown callee) 0x20cd0` | `0x29a5f0` | `?` | `[0x29a650]` | `ret(0x21da8)` |
| `bl` at 0x77460 | `(unknown callee) 0x200a0` | `0x9` | `0xfe40c` | `0xa7` | `0x0` |
| `bl` at 0x77498 | `(unknown callee) 0x2105c` | `0x29a5f0` | `?` | `?` | `?` |
| `bl` at 0x774d0 | `(unknown callee) 0x200a0` | `0x9` | `0xfe40c` | `0xaf` | `0x0` |
| `bl` at 0x774f4 | `(unknown callee) 0x20cd0` | `0x29a5f0` | `?` | `[0x29a654]` | `ret(0x21da8)` |
| `bl` at 0x7752c | `(unknown callee) 0x200a0` | `0x9` | `0xfe40c` | `0xb4` | `0x0` |
| `bl` at 0x77554 | `(unknown callee) 0x2105c` | `0x29a5f0` | `?` | `?` | `?` |
| `bl` at 0x7758c | `(unknown callee) 0x200a0` | `0x9` | `0xfe40c` | `0xba` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x773cc | `[0x13384c] != 0x0` | `[0x13384c] == 0x0` |
| `beq` at 0x773e0 | `[0x29a644] == 0x0` | `[0x29a644] != 0x0` |
| `beq` at 0x773f4 | `[0x29a644] == r0` | `[0x29a644] != r0` |
| `bgt` at 0x7741c | `[0x29a650] > ret(0x21da8) (signed, 32-bit)` | `[0x29a650] <= ret(0x21da8) (signed, 32-bit)` |
| `beq` at 0x7743c | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `bne` at 0x77470 | `[0x29a644] != 0x0` | `[0x29a644] == 0x0` |
| `beq` at 0x774ac | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `ble` at 0x774e8 | `[0x29a654] <= ret(0x21da8) (signed, 32-bit)` | `[0x29a654] > ret(0x21da8) (signed, 32-bit)` |
| `beq` at 0x77508 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `bne` at 0x77540 | `[0x29a644] != [sp-20]` | `[0x29a644] == [sp-20]` |
| `beq` at 0x77568 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x77594 | `r0` | `?` |

## `SysComEnable`

- `SysComEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7863c to 0x786d8.
  Established. The absolute word at `0x29a5e4` is read (`ldr` at 0x78658). The unnamed `0xca8c4`
  call carries `0x0`, unknown, `0x2`, unknown (`bl` at 0x786b8). The word at `sp -0x14` receives
  `[sp-20]+1` in the loop (`str` at 0x786c4). Controller communication enable policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78650 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x78654 | load | 4 | `abs 0x786d0` | `-` |
| `ldr` at 0x78658 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0x7865c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78668 | load | 4 | `abs 0x786d4` | `-` |
| `ldr` at 0x7866c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78674 | load | 4 | `abs 0x786d0` | `-` |
| `ldr` at 0x78678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7869c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x786a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x786bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x786c4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0x786cc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x786cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x786cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x786cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78670 | `(unknown callee) 0x1e958` | `0xfe814` | `?` | `[0x29a5e4]` | `?` |
| `bl` at 0x786b8 | `(unknown callee) 0xca8c4` | `0x0` | `?` | `0x2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x78664 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x786cc | `r0` | `?` |

## `SysComTasknum`

- `SysComTasknum`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x783d8 to 0x7841c.
  Established. The word at `0x29a5dc` is read (`ldr` at 0x783f4). The word through `[0x29a5dc] +0x10` is read (`ldr` at 0x783f8). The unnamed `0x7841c` call carries `[[0x29a5dc]+16]`, entry
  `r0`, entry `r1`, `[0x29a5dc]` (`bl` at 0x78404). Controller task-number lookup remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x783e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x783ec | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x783f0 | load | 4 | `abs 0x78418` | `-` |
| `ldr` at 0x783f4 | load | 4 | `abs 0x29a5dc` | `-` |
| `ldr` at 0x783f8 | load | 4 | `[0x29a5dc] +0x10` | `-` |
| `ldr` at 0x783fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78400 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x78414 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x78414 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x78414 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78404 | `(unknown callee) 0x7841c` | `[[0x29a5dc]+16]` | `r0` | `r1` | `[0x29a5dc]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x78414 | `r0` | `ret(0x7841c)` |

## `SysComJTagReadReady`

- `SysComJTagReadReady`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78534 to 0x7863c.
  Established. The signed condition `[0x29a5e4] <= r0` takes `0x78624`, while greater values fall
  through (`ble` at 0x78558). The unnamed `0x2105c` call carries unknown, `[sp-20]`, unknown,
  unknown (`bl` at 0x785e0). A zero result takes `0x78624`, while nonzero falls through (`beq` at 0x785f4). JTAG read readiness semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78544 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x78548 | load | 4 | `abs 0x7862c` | `-` |
| `ldr` at 0x7854c | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0x78550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7855c | load | 4 | `abs 0x7862c` | `-` |
| `ldr` at 0x78560 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x7856c | load | 4 | `abs 0x7862c` | `-` |
| `ldr` at 0x78570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78594 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x7859c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x785a8 | load | 4 | `abs 0x7862c` | `-` |
| `ldr` at 0x785ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x785d0 | load | 4 | `abs 0x29a5e8` | `-` |
| `str` at 0x785e8 | store | 4 | `sp -0x18` | `ret(0x2105c)` |
| `ldr` at 0x785ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x785f8 | load | 4 | `abs 0x78630` | `-` |
| `str` at 0x785fc | store | 4 | `sp -0x24` | `0xfe7e8` |
| `ldr` at 0x78600 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x78604 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x78608 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7860c | store | 4 | `sp -0x1c` | `ret(0x2105c)` |
| `ldr` at 0x78614 | load | 4 | `abs 0x78634` | `-` |
| `ldr` at 0x78618 | load | 4 | `abs 0x78638` | `-` |
| `ldm` at 0x78628 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x78628 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x78628 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x785e0 | `(unknown callee) 0x2105c` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0x78620 | `(unknown callee) 0x200a0` | `0x9` | `0xfe40c` | `0x262` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x78558 | `[0x29a5e4] <= r0 (signed, 32-bit)` | `[0x29a5e4] > r0 (signed, 32-bit)` |
| `beq` at 0x78568 | `[0x29a5e8] == 0x0` | `[0x29a5e8] != 0x0` |
| `beq` at 0x785a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x785f4 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x78628 | `r0` | `?` |

## `SysComInit`

- `SysComInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x77e0c to 0x781b8.
  Established. Included word assignments at `0x29a5ec`, `0x29a650`, `0x29a654` write `0xc8`, `0x50`,
  `0x5` (`str` at 0x77e88, `str` at 0x77ea0, `str` at 0x77eac). The word at `0x13384c` receives
  `0x1` (`str` at 0x77eb8). The unnamed `0x6a0f8` call carries `0x0` and three unknown candidates
  (`bl` at 0x77ed4). Controller communication initialization remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x77e1c | load | 4 | `abs 0x78154` | `-` |
| `ldr` at 0x77e20 | load | 4 | `abs 0x133808` | `-` |
| `ldr` at 0x77e2c | load | 4 | `abs 0x78158` | `-` |
| `str` at 0x77e30 | store | 4 | `sp -0x34` | `0xfe77c` |
| `ldr` at 0x77e38 | load | 4 | `abs 0x7815c` | `-` |
| `ldr` at 0x77e3c | load | 4 | `abs 0x78160` | `-` |
| `ldr` at 0x77e48 | load | 4 | `abs 0x78154` | `-` |
| `ldr` at 0x77e4c | load | 4 | `abs 0x78154` | `-` |
| `ldr` at 0x77e50 | load | 4 | `abs 0x133808` | `-` |
| `str` at 0x77e58 | store | 4 | `abs 0x133808` | `[0x133808]+1` |
| `ldr` at 0x77e5c | load | 4 | `abs 0x78164` | `-` |
| `ldr` at 0x77e6c | load | 4 | `abs 0x78168` | `-` |
| `ldr` at 0x77e70 | load | 4 | `abs 0x7816c` | `-` |
| `ldr` at 0x77e80 | load | 4 | `abs 0x78164` | `-` |
| `str` at 0x77e88 | store | 4 | `abs 0x29a5ec` | `0xc8` |
| `ldr` at 0x77e8c | load | 4 | `abs 0x78164` | `-` |
| `str` at 0x77e94 | store | 4 | `abs 0x29a64c` | `0x3` |
| `ldr` at 0x77e98 | load | 4 | `abs 0x78164` | `-` |
| `str` at 0x77ea0 | store | 4 | `abs 0x29a650` | `0x50` |
| `ldr` at 0x77ea4 | load | 4 | `abs 0x78164` | `-` |
| `str` at 0x77eac | store | 4 | `abs 0x29a654` | `0x5` |
| `ldr` at 0x77eb0 | load | 4 | `abs 0x78170` | `-` |
| `str` at 0x77eb8 | store | 4 | `abs 0x13384c` | `0x1` |
| `ldr` at 0x77ebc | load | 4 | `abs 0x78164` | `-` |
| `ldr` at 0x77ecc | load | 4 | `abs 0x78164` | `-` |
| `str` at 0x77ed8 | store | 4 | `abs 0x29a5dc` | `ret(0x6a0f8)` |
| `ldr` at 0x77edc | load | 4 | `abs 0x78164` | `-` |
| `ldr` at 0x77ee0 | load | 4 | `abs 0x29a5dc` | `-` |
| `ldr` at 0x77ee4 | load | 4 | `[0x29a5dc] +0x10` | `-` |
| `ldrls` at 0x77ef0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x78150 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x78150 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x78150 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x78150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x77e44 | `(unknown callee) 0x200a0` | `0x9` | `0xfe40c` | `0x1b1` | `0x0` |
| `bl` at 0x77e68 | `(unknown callee) 0x19748` | `0x29a5d4` | `0x0` | `0x84` | `[0x133808]+1` |
| `bl` at 0x77e7c | `(unknown callee) 0x20bb4` | `0x29a5f0` | `0xfe7a0` | `0x1` | `0x9` |
| `bl` at 0x77ec8 | `(unknown callee) 0xa89d0` | `0x29a5d4` | `0x5` | `0x0` | `0x1` |
| `bl` at 0x77ed4 | `(unknown callee) 0x6a0f8` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x77e28 | `[0x133808] == 0x0` | `[0x133808] != 0x0` |
| `ldrls` at 0x77ef0 | `[[0x29a5dc]+16]-43 <= 0x33 (unsigned, 32-bit)` | `[[0x29a5dc]+16]-43 > 0x33 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x78150 | `r0` | `ret(0x6a0f8)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x77ef0).

## `SysComSlab`

- `SysComSlab`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x781b8 to 0x781f0.
  Established. The unnamed `0x781f0` call carries `[[0x29a5dc]+16]`, entry `r0`, entry `r2`,
  `[0x29a5dc]` (`bl` at 0x781dc). The word through `[0x29a5dc] +0x10` is read (`ldr` at 0x781d4).
  The included return record gives `ret(0x781f0)` in `r0` (`ldm` at 0x781e8). Controller slab
  selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x781c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x781cc | load | 4 | `abs 0x781ec` | `-` |
| `ldr` at 0x781d0 | load | 4 | `abs 0x29a5dc` | `-` |
| `ldr` at 0x781d4 | load | 4 | `[0x29a5dc] +0x10` | `-` |
| `ldr` at 0x781d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x781e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x781e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x781e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x781e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x781dc | `(unknown callee) 0x781f0` | `[[0x29a5dc]+16]` | `r0` | `r2` | `[0x29a5dc]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x781e8 | `r0` | `ret(0x781f0)` |

## `SysComSubchan`

- `SysComSubchan`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78298 to 0x782d0.
  Established. The absolute word at `0x29a5dc` is read (`ldr` at 0x782b0). The unnamed `0x782d0`
  call carries `[[0x29a5dc]+16]`, entry `r0`, entry `r2`, `[0x29a5dc]` (`bl` at 0x782bc). The
  included return record carries `ret(0x782d0)` in `r0` (`ldm` at 0x782c8). Subchannel lookup
  behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x782a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x782ac | load | 4 | `abs 0x782cc` | `-` |
| `ldr` at 0x782b0 | load | 4 | `abs 0x29a5dc` | `-` |
| `ldr` at 0x782b4 | load | 4 | `[0x29a5dc] +0x10` | `-` |
| `ldr` at 0x782b8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x782c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x782c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x782c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x782c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x782bc | `(unknown callee) 0x782d0` | `[[0x29a5dc]+16]` | `r0` | `r2` | `[0x29a5dc]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x782c8 | `r0` | `ret(0x782d0)` |

## `L1CfgGetL2`

- `L1CfgGetL2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a2ac to 0x6a328.
  Established. A zero entry `r0` takes `0x6a2e4`, while nonzero falls through (`beq` at 0x6a2d0). An
  included word assignment through entry `r0 +0x0` writes `[0x132ac4]` (`str` at 0x6a2e0). Other
  included assignments at unknown targets write `[0x132ac8]` and `[0x132acc]` (`str` at 0x6a2fc,
  `str` at 0x6a318). L2 configuration representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a2bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a2c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6a2c4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x6a2c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a2d8 | load | 4 | `abs 0x6a324` | `-` |
| `ldr` at 0x6a2dc | load | 4 | `abs 0x132ac4` | `-` |
| `str` at 0x6a2e0 | store | 4 | `r0 +0x0` | `[0x132ac4]` |
| `ldr` at 0x6a2e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a2f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a2f4 | load | 4 | `abs 0x6a324` | `-` |
| `ldr` at 0x6a2f8 | load | 4 | `abs 0x132ac8` | `-` |
| `str` at 0x6a2fc | store | 4 | `(unknown)` | `[0x132ac8]` |
| `ldr` at 0x6a300 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a30c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a310 | load | 4 | `abs 0x6a324` | `-` |
| `ldr` at 0x6a314 | load | 4 | `abs 0x132acc` | `-` |
| `str` at 0x6a318 | store | 4 | `(unknown)` | `[0x132acc]` |
| `ldm` at 0x6a320 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a320 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a320 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a2d0 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x6a2ec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6a308 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a320 | `r0` | `r0` |

## `L1CfgGetPort`

- `L1CfgGetPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a328 to 0x6a370.
  Established. The absolute word at `0x132ad8` is read (`ldr` at 0x6a35c). Entry `r0` is stored at
  `sp -0x14` (`str` at 0x6a338). The included return record gives an unknown integer value in `r0`
  (`ldm` at 0x6a368). Port lookup result meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a338 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6a33c | load | 4 | `abs 0x6a36c` | `-` |
| `ldr` at 0x6a340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a35c | load | 4 | `abs 0x132ad8` | `-` |
| `ldm` at 0x6a368 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6a368 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a368 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a368 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a368 | `r0` | `?` |
