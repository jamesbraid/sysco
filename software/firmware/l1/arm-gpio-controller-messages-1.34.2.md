# ARM GPIO and controller message paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates. No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/gpio-controller-messages-facts.txt) retain all tool sections.

## `l1ioGpioInit`

- `l1ioGpioInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacda4 to 0xace20.
  Established. The opening call to `0x18b54` has candidates `0x10807c`, `0x2`, entry r2 and entry r3
  (`bl` at 0xacdbc); its result is stored as a word at absolute `0x11571c` (`str` at 0xacdc8). A
  signed-nonnegative value loaded from that address takes the edge to the return, while a
  negative value falls through into calls to `0x188b4`, `0x18890` and `0x1eed0` (`bge` at 0xacdd4,
  `bl` at 0xacdd8, `bl` at 0xacde4, `bl` at 0xace08). The absolute word’s device role and merged
  return value remain unknown (`ldmdb` at 0xace0c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xacdb4 | load | 4 | `abs 0xace10` | `-` |
| `ldr` at 0xacdc4 | load | 4 | `abs 0xace14` | `-` |
| `str` at 0xacdc8 | store | 4 | `abs 0x11571c` | `ret(0x18b54)` |
| `ldr` at 0xacdcc | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xacde0 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xacdec | load | 4 | `abs 0xace18` | `-` |
| `str` at 0xacdf0 | store | 4 | `sp -0x18` | `0x10808c` |
| `str` at 0xacdf4 | store | 4 | `sp -0x14` | `ret(0x18890)` |
| `ldr` at 0xacdfc | load | 4 | `abs 0xace1c` | `-` |
| `ldmdb` at 0xace0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xace0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xace0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacdbc | `(unknown callee) 0x18b54` | `0x10807c` | `0x2` | `r2` | `r3` |
| `bl` at 0xacdd8 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `ret(0x18b54)` | `[0x11571c]` |
| `bl` at 0xacde4 | `(unknown callee) 0x18890` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xace08 | `(unknown callee) 0x1eed0` | `0x1` | `0x1080a4` | `0x32` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xacdd4 | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xace0c | `r0` | `?` |

## `l1ioGpioWrite`

- `l1ioGpioWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad004 to 0xad0a8.
  Established. A signed-nonnegative word at absolute `0x11571c` takes the edge into the longer path,
  while negative falls through to a stack store of `0xfffff3e4` (`ldr` at 0xad024, `bge` at
  0xad02c, `str` at 0xad034). The longer path includes byte stores of `([r0+3]:1 & 0xff)` at `sp
  -0x1c` and `((r1 & 0xff) & 0xff)` at `sp -0x1b` (`strb` at 0xad044, `strb` at 0xad04c). It calls
  `0x18bf0` with candidates `[0x11571c]`, `0x40046700`, `sp-28`, `0x11571c` (`bl` at 0xad060). A
  zero result takes the edge to the stack store of word `0x0`, whereas a nonzero result falls
  through to calls to `0x188b4` and `0xad434` (`beq` at 0xad06c, `str` at 0xad090, `bl` at
  0xad070, `bl` at 0xad07c). The numeric constant’s hardware meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad014 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xad01c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xad020 | load | 4 | `abs 0xad09c` | `-` |
| `ldr` at 0xad024 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad030 | load | 4 | `abs 0xad0a0` | `-` |
| `str` at 0xad034 | store | 4 | `sp -0x20` | `0xfffff3e4` |
| `ldr` at 0xad03c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xad040 | load | 1 | `r0 +0x3` | `-` |
| `strb` at 0xad044 | store | 1 | `sp -0x1c` | `([r0+3]:1 & 0xff)` |
| `ldrb` at 0xad048 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xad04c | store | 1 | `sp -0x1b` | `((r1 & 0xff) & 0xff)` |
| `ldr` at 0xad050 | load | 4 | `abs 0xad09c` | `-` |
| `ldr` at 0xad058 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad05c | load | 4 | `abs 0xad0a4` | `-` |
| `ldr` at 0xad078 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xad084 | store | 4 | `sp -0x20` | `ret(0xad434)` |
| `str` at 0xad090 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xad094 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xad098 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad098 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad098 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad060 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x40046700` | `sp-28` | `0x11571c` |
| `bl` at 0xad070 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xad07c | `(unknown callee) 0xad434` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xad02c | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xad06c | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad098 | `r0` | `?` |

## `l1ioGpioRead`

- `l1ioGpioRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad0a8 to 0xad14c.
  Established. The byte loaded at `r0 +0x3` is stored at `sp -0x1c` (`ldrb` at 0xad0e0, `strb` at
  0xad0e4). A call to `0x18bf0` uses candidates `[0x11571c]`, `0xc0046701`, `sp-28`, `0x11571c`
  (`bl` at 0xad0f8). A zero result takes the edge to a byte store of `([sp-27]:1 & 0xff)` at
  `[sp-24] +0x0` (`beq` at 0xad104, `strb` at 0xad12c). A nonzero result falls through to the
  two calls to `0x188b4` and `0xad434` (`beq` at 0xad104, `bl` at 0xad108, `bl` at 0xad114). The
  result byte’s GPIO interpretation is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad0b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xad0bc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xad0c0 | load | 4 | `abs 0xad140` | `-` |
| `ldr` at 0xad0c4 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad0d0 | load | 4 | `abs 0xad144` | `-` |
| `str` at 0xad0d4 | store | 4 | `sp -0x20` | `0xfffff3e4` |
| `ldr` at 0xad0dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xad0e0 | load | 1 | `r0 +0x3` | `-` |
| `strb` at 0xad0e4 | store | 1 | `sp -0x1c` | `([r0+3]:1 & 0xff)` |
| `ldr` at 0xad0e8 | load | 4 | `abs 0xad140` | `-` |
| `ldr` at 0xad0f0 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad0f4 | load | 4 | `abs 0xad148` | `-` |
| `ldr` at 0xad110 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xad11c | store | 4 | `sp -0x20` | `ret(0xad434)` |
| `ldr` at 0xad124 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xad128 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0xad12c | store | 1 | `[sp-24] +0x0` | `([sp-27]:1 & 0xff)` |
| `str` at 0xad134 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xad138 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xad13c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad13c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad13c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad0f8 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0xc0046701` | `sp-28` | `0x11571c` |
| `bl` at 0xad108 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xad114 | `(unknown callee) 0xad434` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xad0cc | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xad104 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad13c | `r0` | `?` |

## `l1ioGpioIntSet`

- `l1ioGpioIntSet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad14c to 0xad1f0.
  Established. The body stores `([r0+3]:1 & 0xff)` and `((r1 & 0xff) & 0xff)` as bytes at `sp -0x1c`
  and `sp -0x1b`, respectively (`strb` at 0xad18c, `strb` at 0xad194); its call to `0x18bf0` has
  candidates `[0x11571c]`, `0x40046706`, `sp-28`, `0x11571c` (`bl` at 0xad1a8). A zero result takes
  the edge to the word store of `0x0` at `sp -0x20`, while nonzero falls through to the
  two calls (`beq` at 0xad1b4, `str` at 0xad1d8, `bl` at 0xad1b8, `bl` at 0xad1c4). The
  constant’s interrupt-setting semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad15c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xad164 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xad168 | load | 4 | `abs 0xad1e4` | `-` |
| `ldr` at 0xad16c | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad178 | load | 4 | `abs 0xad1e8` | `-` |
| `str` at 0xad17c | store | 4 | `sp -0x20` | `0xfffff3e4` |
| `ldr` at 0xad184 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xad188 | load | 1 | `r0 +0x3` | `-` |
| `strb` at 0xad18c | store | 1 | `sp -0x1c` | `([r0+3]:1 & 0xff)` |
| `ldrb` at 0xad190 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xad194 | store | 1 | `sp -0x1b` | `((r1 & 0xff) & 0xff)` |
| `ldr` at 0xad198 | load | 4 | `abs 0xad1e4` | `-` |
| `ldr` at 0xad1a0 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad1a4 | load | 4 | `abs 0xad1ec` | `-` |
| `ldr` at 0xad1c0 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xad1cc | store | 4 | `sp -0x20` | `ret(0xad434)` |
| `str` at 0xad1d8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xad1dc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xad1e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad1e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad1e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad1a8 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x40046706` | `sp-28` | `0x11571c` |
| `bl` at 0xad1b8 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xad1c4 | `(unknown callee) 0xad434` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xad174 | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xad1b4 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad1e0 | `r0` | `?` |

## `l1ioGpioIntGet`

- `l1ioGpioIntGet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad1f0 to 0xad294.
  Established. The call to `0x18bf0` carries candidates `[0x11571c]`, `0xc0046707`, `sp-28`,
  `0x11571c` (`bl` at 0xad240). A zero result takes the edge to a byte load from `sp -0x1b` and the
  byte store of `([sp-27]:1 & 0xff)` at `[sp-24] +0x0` (`beq` at 0xad24c, `ldrb` at 0xad270, `strb`
  at 0xad274). A nonzero result falls through to a call to `0x188b4` and then `0xad434` (`beq`
  at 0xad24c, `bl` at 0xad250, `bl` at 0xad25c). The merged return value and the copied byte’s
  interrupt-state interpretation are unknown (`ldmdb` at 0xad284).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad200 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xad204 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xad208 | load | 4 | `abs 0xad288` | `-` |
| `ldr` at 0xad20c | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad218 | load | 4 | `abs 0xad28c` | `-` |
| `str` at 0xad21c | store | 4 | `sp -0x20` | `0xfffff3e4` |
| `ldr` at 0xad224 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xad228 | load | 1 | `r0 +0x3` | `-` |
| `strb` at 0xad22c | store | 1 | `sp -0x1c` | `([r0+3]:1 & 0xff)` |
| `ldr` at 0xad230 | load | 4 | `abs 0xad288` | `-` |
| `ldr` at 0xad238 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad23c | load | 4 | `abs 0xad290` | `-` |
| `ldr` at 0xad258 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xad264 | store | 4 | `sp -0x20` | `ret(0xad434)` |
| `ldr` at 0xad26c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xad270 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0xad274 | store | 1 | `[sp-24] +0x0` | `([sp-27]:1 & 0xff)` |
| `str` at 0xad27c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xad280 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xad284 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad284 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad284 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad240 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0xc0046707` | `sp-28` | `0x11571c` |
| `bl` at 0xad250 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xad25c | `(unknown callee) 0xad434` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xad214 | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xad24c | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad284 | `r0` | `?` |

## `l1ioGpioInt`

- `l1ioGpioInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad294 to 0xad434.
  Established. The loop includes a call to `0x18bf0` with candidates `[0x11571c]`, `0x80046708`,
  `sp-32`, `0x11571c` (`bl` at 0xad30c). A zero result takes the edge to further testing, and a
  nonzero result falls through into calls that include `0xad434` (`beq` at 0xad318, `bl` at
  0xad328). Another call targets `0xb8014` with candidates `[sp-20]`, `[sp-24]`, `[sp-25]:1`,
  `[sp-31]:1` (`bl` at 0xad3a8); its result is added into the word at `sp -0x24` (`str` at 0xad3b8).
  The repeat branch retains only a less-or-equal flags predicate with unknown operands, so no exact
  iteration count is established (`ble` at 0xad2f4). The callee’s interrupt-handling role is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad2a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xad2a8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xad2b0 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `str` at 0xad2b8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xad2bc | load | 4 | `abs 0xad41c` | `-` |
| `ldr` at 0xad2c0 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad2cc | load | 4 | `abs 0xad420` | `-` |
| `str` at 0xad2d0 | store | 4 | `sp -0x2c` | `0xfffff3e4` |
| `ldr` at 0xad2d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xad2dc | load | 1 | `r0 +0x3` | `-` |
| `strb` at 0xad2e0 | store | 1 | `sp -0x20` | `([r0+3]:1 & 0xff)` |
| `str` at 0xad2e8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xad2ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xad2fc | load | 4 | `abs 0xad41c` | `-` |
| `ldr` at 0xad304 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xad308 | load | 4 | `abs 0xad424` | `-` |
| `ldr` at 0xad324 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xad33c | load | 4 | `abs 0xad428` | `-` |
| `str` at 0xad340 | store | 4 | `sp -0x38` | `0x1080b0` |
| `ldr` at 0xad344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad348 | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xad34c | store | 4 | `sp -0x34` | `[[sp-20]+12]` |
| `str` at 0xad350 | store | 4 | `sp -0x30` | `ret(0x1ea50)` |
| `ldr` at 0xad358 | load | 4 | `abs 0xad42c` | `-` |
| `ldr` at 0xad368 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad36c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xad374 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xad378 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0xad37c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xad390 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xad394 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0xad398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad39c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xad3b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad3b8 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xb8014))` |
| `ldr` at 0xad3c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad3c4 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xad3cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xad3d4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xad3dc | load | 4 | `abs 0xad430` | `-` |
| `str` at 0xad3e0 | store | 4 | `sp -0x38` | `0x1080dc` |
| `ldr` at 0xad3e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad3e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xad3ec | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0xad3f0 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xad3f4 | store | 4 | `sp -0x30` | `[sp-31]:1` |
| `ldr` at 0xad3fc | load | 4 | `abs 0xad42c` | `-` |
| `ldr` at 0xad40c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad410 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xad414 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xad418 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad418 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad30c | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x80046708` | `sp-32` | `0x11571c` |
| `bl` at 0xad31c | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xad328 | `(unknown callee) 0xad434` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xad334 | `(unknown callee) 0x1ea50` | `ret(0xad434)` | `?` | `?` | `ret(0xad434)` |
| `bl` at 0xad364 | `(unknown callee) 0x1eed0` | `0x3` | `0x1080a4` | `0xce` | `0x0` |
| `bl` at 0xad3a8 | `(unknown callee) 0xb8014` | `[sp-20]` | `[sp-24]` | `[sp-25]:1` | `[sp-31]:1` |
| `bl` at 0xad408 | `(unknown callee) 0x1eed0` | `0x3` | `0x1080a4` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xad2c8 | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |
| `ble` at 0xad2f4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xad318 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `beq` at 0xad38c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad418 | `r0` | `?` |

## `l1ioGpioReset`

- `l1ioGpioReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xace20 to 0xad004.
  Established. The sequence includes five calls to `0x18bf0`, with second candidates `0x40046706`,
  `0x40046700`, `0x40046704`, `0x40046702` and `0x80046708` (`bl` at 0xace7c, `bl` at 0xaceb8, `bl`
  at 0xacf04, `bl` at 0xacf50, `bl` at 0xacf8c). All five have first candidate `[0x11571c]` and
  third candidate `sp-24` (`bl` at 0xace7c, `bl` at 0xaceb8, `bl` at 0xacf04, `bl` at 0xacf50, `bl`
  at 0xacf8c). A byte store first writes `0x0` at `sp -0x17`, while a later store there writes
  `([[sp-20]+6]:1 & 0xff)` (`strb` at 0xace68, `strb` at 0xacea4). The final equality branch takes
  the edge to a word store of `0x0` at `sp -0x20` when `[sp-28] == 0x0`, while nonzero falls
  through to a call to `0xad434` (`beq` at 0xacfc0, `str` at 0xacfdc, `bl` at 0xacfc8). The
  five call constants’ electrical meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xace30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xace38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xace3c | load | 4 | `abs 0xacfe8` | `-` |
| `ldr` at 0xace40 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xace4c | load | 4 | `abs 0xacfec` | `-` |
| `str` at 0xace50 | store | 4 | `sp -0x20` | `0xfffff3e4` |
| `ldr` at 0xace58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xace5c | load | 1 | `r0 +0x3` | `-` |
| `strb` at 0xace60 | store | 1 | `sp -0x18` | `([r0+3]:1 & 0xff)` |
| `strb` at 0xace68 | store | 1 | `sp -0x17` | `0x0` |
| `ldr` at 0xace6c | load | 4 | `abs 0xacfe8` | `-` |
| `ldr` at 0xace74 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xace78 | load | 4 | `abs 0xacff0` | `-` |
| `ldr` at 0xace94 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xace98 | store | 4 | `sp -0x1c` | `[ret(0x188b4)]` |
| `ldr` at 0xace9c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xacea0 | load | 1 | `[sp-20] +0x6` | `-` |
| `strb` at 0xacea4 | store | 1 | `sp -0x17` | `([[sp-20]+6]:1 & 0xff)` |
| `ldr` at 0xacea8 | load | 4 | `abs 0xacfe8` | `-` |
| `ldr` at 0xaceb0 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xaceb4 | load | 4 | `abs 0xacff4` | `-` |
| `ldr` at 0xacec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacedc | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xacee0 | store | 4 | `sp -0x1c` | `[ret(0x188b4)]` |
| `ldr` at 0xacee4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xacee8 | load | 1 | `[sp-20] +0x5` | `-` |
| `strb` at 0xacef0 | store | 1 | `sp -0x17` | `?` |
| `ldr` at 0xacef4 | load | 4 | `abs 0xacfe8` | `-` |
| `ldr` at 0xacefc | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xacf00 | load | 4 | `abs 0xacff8` | `-` |
| `ldr` at 0xacf14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacf28 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xacf2c | store | 4 | `sp -0x1c` | `[ret(0x188b4)]` |
| `ldr` at 0xacf30 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xacf34 | load | 1 | `[sp-20] +0x4` | `-` |
| `strb` at 0xacf3c | store | 1 | `sp -0x17` | `?` |
| `ldr` at 0xacf40 | load | 4 | `abs 0xacfe8` | `-` |
| `ldr` at 0xacf48 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xacf4c | load | 4 | `abs 0xacffc` | `-` |
| `ldr` at 0xacf60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacf74 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xacf78 | store | 4 | `sp -0x1c` | `[ret(0x188b4)]` |
| `ldr` at 0xacf7c | load | 4 | `abs 0xacfe8` | `-` |
| `ldr` at 0xacf84 | load | 4 | `abs 0x11571c` | `-` |
| `ldr` at 0xacf88 | load | 4 | `abs 0xad000` | `-` |
| `ldr` at 0xacf9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacfb0 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xacfb4 | store | 4 | `sp -0x1c` | `[ret(0x188b4)]` |
| `ldr` at 0xacfb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacfc4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xacfd0 | store | 4 | `sp -0x20` | `ret(0xad434)` |
| `str` at 0xacfdc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xacfe0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xacfe4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xacfe4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xacfe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xace7c | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x40046706` | `sp-24` | `0x11571c` |
| `bl` at 0xace8c | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xaceb8 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x40046700` | `sp-24` | `0x11571c` |
| `bl` at 0xaced4 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xacf04 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x40046704` | `sp-24` | `0x11571c` |
| `bl` at 0xacf20 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xacf50 | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x40046702` | `sp-24` | `0x11571c` |
| `bl` at 0xacf6c | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xacf8c | `(unknown callee) 0x18bf0` | `[0x11571c]` | `0x80046708` | `sp-24` | `0x11571c` |
| `bl` at 0xacfa8 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xacfc8 | `(unknown callee) 0xad434` | `[sp-28]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xace48 | `[0x11571c] >= 0x0 (signed, 32-bit)` | `[0x11571c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xace88 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `beq` at 0xacec4 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `bne` at 0xaced0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xacf10 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `bne` at 0xacf1c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xacf5c | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `bne` at 0xacf68 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xacf98 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `bne` at 0xacfa4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xacfc0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xacfe4 | `r0` | `?` |

## `SysComEventDisable`

- `SysComEventDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ebac to 0x6ebd8.
  Established. A word store addresses absolute `0x281fcc` with unknown value (`str` at 0x6ebc0). The
  next call targets `0x1abd0` with candidates `0x13`, `0x0`, `0x281fc0` and unknown (`bl` at
  0x6ebcc). The return reports `ret(0x1abd0)` in r0 (`ldmdb` at 0x6ebd0). The absolute word’s
  event-disable encoding is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6ebb8 | load | 4 | `abs 0x6ebd4` | `-` |
| `str` at 0x6ebc0 | store | 4 | `abs 0x281fcc` | `?` |
| `ldmdb` at 0x6ebd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ebd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ebd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ebcc | `(unknown callee) 0x1abd0` | `0x13` | `0x0` | `0x281fc0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ebd0 | `r0` | `ret(0x1abd0)` |

## `SysComEventEnabled`

- `SysComEventEnabled`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6eb6c to 0x6ebac.
  Established. The word load addresses absolute `0x281fcc` (`ldr` at 0x6eb80). The branch retains `Z
  == 0` as its taken condition and `not (Z == 0)` as its fall-through condition (`bne` at 0x6eb88).
  The two arms store word `0x1` and word `0x0` at `sp -0x14`, respectively (`str` at 0x6eb9c, `str`
  at 0x6eb90). The return reports `[sp-20]` in r0 (`ldmdb` at 0x6eba4). Which loaded values mean
  enabled is unknown in the reported predicate.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6eb7c | load | 4 | `abs 0x6eba8` | `-` |
| `ldr` at 0x6eb80 | load | 4 | `abs 0x281fcc` | `-` |
| `str` at 0x6eb90 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x6eb9c | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x6eba0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6eba4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6eba4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6eba4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6eb88 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6eba4 | `r0` | `[sp-20]` |

## `syscom_process_inbound_data`

- `syscom_process_inbound_data`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6cc2c to 0x6cd60.
  Established. The body calls `0x541d4` with candidates `sp-80`, `sp-76`, `0x0`, `sp-80`, followed
  by `0x54450` with `sp-80`, `sp-76`, `[sp-32]`, `[sp-36]` (`bl` at 0x6cc5c, `bl` at 0x6cc7c). A
  zero result from `0x6bc14` takes the edge that bypasses the call to `0x6bcac`, while nonzero falls
  through to it (`bl` at 0x6cc88, `beq` at 0x6cc94, `bl` at 0x6cca8). A later call to `0x9bf04` has
  candidates `sp-44`, `0x0`, `[sp-80]`, `sp-76` (`bl` at 0x6cd1c). Two word stores have unresolved
  effective targets, one storing `[0x281fc0]` and the other an unknown value (`str` at 0x6ccc0,
  `str` at 0x6cd00). Data framing and task selection remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6cc3c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x6cc40 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x6cc44 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x6cc6c | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x6cc74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cc78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6cc80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cc98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cc9c | load | 4 | `abs 0x6cd54` | `-` |
| `ldr` at 0x6cca0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cca4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ccb8 | load | 4 | `abs 0x6cd58` | `-` |
| `ldr` at 0x6ccbc | load | 4 | `abs 0x281fc0` | `-` |
| `str` at 0x6ccc0 | store | 4 | `(unknown)` | `[0x281fc0]` |
| `ldr` at 0x6ccdc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cce4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6cd00 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6cd14 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x6cd24 | store | 4 | `sp -0x54` | `ret(0x9bf04)` |
| `ldr` at 0x6cd28 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6cd34 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6cd40 | load | 4 | `abs 0x6cd5c` | `-` |
| `ldr` at 0x6cd44 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6cd50 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6cd50 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6cd50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6cd50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6cd50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6cc5c | `(unknown callee) 0x541d4` | `sp-80` | `sp-76` | `0x0` | `sp-80` |
| `bl` at 0x6cc7c | `(unknown callee) 0x54450` | `sp-80` | `sp-76` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x6cc88 | `(unknown callee) 0x6bc14` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0x6cca8 | `(unknown callee) 0x6bcac` | `[sp-28]` | `0xe8ae0` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x6ccec | `(unknown callee) 0x6ca54` | `[sp-28]` | `0x4` | `sp-24` | `?` |
| `bl` at 0x6cd1c | `(unknown callee) 0x9bf04` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x6cd38 | `(unknown callee) 0x1e71c` | `ret(0x9bf04)` | `?` | `?` | `ret(0x9bf04)` |
| `bl` at 0x6cd4c | `(unknown callee) 0x1dba4` | `0xe8af0` | `[sp-28]` | `ret(0x1e71c)` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6cc94 | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `beq` at 0x6cd30 | `ret(0x9bf04) == 0x0` | `ret(0x9bf04) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6cd50 | `r0` | `?` |

## `syscom_process_inbound_request`

- `syscom_process_inbound_request`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6cd60 to 0x6d140.
  Established. The byte accesses include `[sp-32] +0x1`, `+0x2`, `+0x3`, `+0x4`, `+0x5` and
  `+0x6` (`ldrb` at 0x6cdc4, `ldrb` at 0x6cdd8, `ldrb` at 0x6cdf0, `ldrb` at 0x6ce08, `ldrb` at
  0x6ce20, `ldrb` at 0x6ce34). An unsigned branch takes its edge for `[sp-140]-12289 > 0x2` and
  falls through for the complementary range (`bhi` at 0x6ce50). A later explicit-link call has
  unknown target with candidates `[sp-28]`, `[sp-144]`, `[sp-32]`, `[sp-36]` (`mov` at 0x6cf38). Two
  equality branches select paths for `[[sp-32]]:1 == 0x0` and `[[sp-32]]:1 == 0x1` (`beq` at
  0x6cf94, `beq` at 0x6cfa0). A call to `0x9bc5c` later uses candidates `sp-44`, `sp-136`, `0x0`,
  `0x0` (`bl` at 0x6d0f8). The request-field meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6cd70 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x6cd74 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x6cd78 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x6cd80 | store | 4 | `sp -0x80` | `0x1` |
| `str` at 0x6cd88 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x6cd8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cda4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cda8 | load | 4 | `abs 0x6d130` | `-` |
| `ldr` at 0x6cdac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cdb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6cdbc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cdc4 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x6cdd0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cdd8 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x6cde8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cdf0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x6ce00 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6ce08 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x6ce10 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x6ce18 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6ce20 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x6ce2c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6ce34 | load | 1 | `[sp-32] +0x6` | `-` |
| `str` at 0x6ce3c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6ce40 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ce54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ce58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6ce5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ce74 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6ce7c | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x6ce88 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6ce90 | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x6ceb0 | load | 4 | `abs 0x6d134` | `-` |
| `ldr` at 0x6ceb4 | load | 4 | `abs 0x281fc0` | `-` |
| `str` at 0x6ceb8 | store | 4 | `(unknown)` | `[0x281fc0]` |
| `ldr` at 0x6cebc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cec0 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x6cec8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x6cee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ceec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cef0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6cf08 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6cf0c | load | 4 | `abs 0x6d138` | `-` |
| `ldr` at 0x6cf10 | load | 4 | `abs 0x11b084` | `-` |
| `ldr` at 0x6cf1c | load | 4 | `abs 0x6d138` | `-` |
| `ldr` at 0x6cf20 | load | 4 | `abs 0x11b084` | `-` |
| `ldr` at 0x6cf24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cf28 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x6cf2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cf30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6cf3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6cf4c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cf54 | load | 1 | `[sp-32] +0x7` | `-` |
| `str` at 0x6cf58 | store | 4 | `sp -0x94` | `[[sp-32]+7]:1` |
| `ldr` at 0x6cf5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6cf64 | store | 4 | `sp -0x20` | `[sp-32]+8` |
| `ldr` at 0x6cf68 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x6cf70 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `ldr` at 0x6cf80 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cf84 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x6cf88 | store | 4 | `sp -0x98` | `[[sp-32]]:1` |
| `ldr` at 0x6cf8c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6cf98 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6cfb4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cfbc | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x6cfc8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cfd0 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x6cfe0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6cfe8 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x6cff8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6d000 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x6d014 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6d01c | store | 4 | `sp -0x20` | `[sp-32]+5` |
| `ldr` at 0x6d024 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d03c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6d048 | store | 4 | `sp -0xa8` | `0x1` |
| `ldr` at 0x6d05c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d06c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6d078 | store | 4 | `sp -0x20` | `(ret(0x18a1c) + [sp-32])+1` |
| `ldr` at 0x6d088 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d090 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6d094 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x6d0a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6d0ac | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x6d0b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6d0bc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6d0cc | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6d0d0 | store | 4 | `sp -0xa8` | `[sp-128]` |
| `str` at 0x6d0d8 | store | 4 | `sp -0xa4` | `sp-124` |
| `str` at 0x6d0e0 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x6d0e8 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x6d100 | store | 4 | `sp -0x84` | `ret(0x9bc5c)` |
| `ldr` at 0x6d104 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6d110 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6d11c | load | 4 | `abs 0x6d13c` | `-` |
| `ldr` at 0x6d120 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6d12c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6d12c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6d12c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6d12c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6d12c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6cd94 | `(unknown callee) 0x6bc14` | `r0` | `0x2` | `r2` | `0x0` |
| `bl` at 0x6cdb4 | `(unknown callee) 0x6bcac` | `[sp-28]` | `0xe8b18` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x6ce60 | `(unknown callee) 0x6ebd8` | `[sp-28]` | `[sp-32]` | `[sp-36]` | `[sp-140]-12289` |
| `bl` at 0x6cea0 | `(unknown callee) 0x541d4` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x6cef4 | `(unknown callee) 0x6ca54` | `[sp-28]` | `[sp-144]` | `sp-24` | `?` |
| `mov` at 0x6cf38 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-144]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x6d010 | `(unknown callee) 0x54230` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x6d028 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0x6d058 | `(unknown callee) 0x54450` | `sp-128` | `sp-124` | `[sp-32]+1` | `ret(0x18a1c)+1` |
| `bl` at 0x6d060 | `(unknown callee) 0x18a1c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x6d0a4 | `(unknown callee) 0x542d4` | `sp-128` | `sp-124` | `[sp-32]+1` | `?` |
| `bl` at 0x6d0f8 | `(unknown callee) 0x9bc5c` | `sp-44` | `sp-136` | `0x0` | `0x0` |
| `bl` at 0x6d114 | `(unknown callee) 0x1e71c` | `ret(0x9bc5c)` | `?` | `?` | `ret(0x9bc5c)` |
| `bl` at 0x6d128 | `(unknown callee) 0x1dba4` | `0xe8b28` | `[sp-28]` | `ret(0x1e71c)` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6cda0 | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `bhi` at 0x6ce50 | `[sp-140]-12289 > 0x2 (unsigned, 32-bit)` | `[sp-140]-12289 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x6cf18 | `[0x11b084] == 0x0` | `[0x11b084] != 0x0` |
| `bls` at 0x6cf44 | `[sp-36] <= 0x7 (unsigned, 32-bit)` | `[sp-36] > 0x7 (unsigned, 32-bit)` |
| `bne` at 0x6cf78 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6cf94 | `[[sp-32]]:1 == 0x0` | `[[sp-32]]:1 != 0x0` |
| `beq` at 0x6cfa0 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0x6d10c | `ret(0x9bc5c) == 0x0` | `ret(0x9bc5c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6d12c | `r0` | `?` |

## `syscom_process_inbound_event`

- `syscom_process_inbound_event`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6d140 to 0x6d36c.
  Established. A nonzero absolute word at `0x27ee70` takes the bypass edge, while zero falls through
  to a word store of `0x1` at `sp -0x68` (`ldr` at 0x6d16c, `bne` at 0x6d174, `str` at 0x6d17c).
  The call to `0x54450` has candidates `sp-80`, `sp-76`, `[sp-32]+1`, `[sp-36]-1` (`bl` at 0x6d1e4).
  Another call targets `0x2f90c` with candidates `[sp-28]+2`, `[sp-92]`, unknown and `[sp-28]+2`
  (`bl` at 0x6d234). A result of `0x4` from `0x2fac0` falls through its branch to `0x6ca54`,
  while other values take the edge to continued iteration (`bl` at 0x6d274, `bne` at 0x6d280, `bl`
  at 0x6d28c). The body later calls `0x52edc` with all candidates unknown (`bl` at 0x6d2b0). Event
  routing meanings are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6d150 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x6d154 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x6d158 | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x6d15c | load | 4 | `abs 0x6d35c` | `-` |
| `ldr` at 0x6d160 | load | 4 | `abs 0x27ee70` | `-` |
| `str` at 0x6d164 | store | 4 | `sp -0x68` | `[0x27ee70]` |
| `ldr` at 0x6d168 | load | 4 | `abs 0x6d35c` | `-` |
| `ldr` at 0x6d16c | load | 4 | `abs 0x27ee70` | `-` |
| `str` at 0x6d17c | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0x6d180 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x6d184 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x6d1c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d1c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6d1d4 | store | 4 | `sp -0x6c` | `0x1` |
| `ldr` at 0x6d1e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6d200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6d204 | load | 4 | `abs 0x6d360` | `-` |
| `ldr` at 0x6d208 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d20c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6d214 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6d218 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x6d220 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x6d224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6d230 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x6d238 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6d23c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x6d248 | store | 4 | `sp -0x60` | `ret(0x6ca54)` |
| `str` at 0x6d250 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x6d254 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6d258 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x6d268 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6d284 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6d294 | store | 4 | `sp -0x60` | `ret(0x6ca54)` |
| `ldr` at 0x6d29c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6d2a4 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x6d2ac | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x6d2c0 | load | 4 | `abs 0x6d364` | `-` |
| `ldr` at 0x6d2c4 | load | 4 | `abs 0x281fc0` | `-` |
| `str` at 0x6d2c8 | store | 4 | `(unknown)` | `[0x281fc0]` |
| `ldr` at 0x6d2e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d2ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6d2f0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x6d308 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6d31c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x6d32c | store | 4 | `sp -0x58` | `ret(0x9bf04)` |
| `ldr` at 0x6d330 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x6d33c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x6d348 | load | 4 | `abs 0x6d368` | `-` |
| `ldr` at 0x6d34c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6d358 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6d358 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6d358 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6d358 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6d358 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6d18c | `(unknown callee) 0x1c79c` | `0x81` | `?` | `?` | `[0x27ee70]` |
| `bl` at 0x6d1b4 | `(unknown callee) 0x541d4` | `sp-80` | `sp-76` | `0x0` | `sp-80` |
| `bl` at 0x6d1e4 | `(unknown callee) 0x54450` | `sp-80` | `sp-76` | `[sp-32]+1` | `[sp-36]-1` |
| `bl` at 0x6d1f0 | `(unknown callee) 0x6bc14` | `[sp-28]` | `0x4` | `?` | `?` |
| `bl` at 0x6d210 | `(unknown callee) 0x6bcac` | `[sp-28]` | `0xe8b54` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x6d234 | `(unknown callee) 0x2f90c` | `[sp-28]+2` | `[sp-92]` | `?` | `[sp-28]+2` |
| `bl` at 0x6d240 | `(unknown callee) 0x6ca54` | `[sp-28]` | `[sp-92]` | `?` | `?` |
| `bl` at 0x6d274 | `(unknown callee) 0x2fac0` | `?` | `?` | `?` | `?` |
| `bl` at 0x6d28c | `(unknown callee) 0x6ca54` | `[sp-84]` | `0x4` | `?` | `ret(0x2fac0)` |
| `bl` at 0x6d2b0 | `(unknown callee) 0x52edc` | `?` | `?` | `?` | `?` |
| `bl` at 0x6d2f4 | `(unknown callee) 0x6ca54` | `[sp-28]` | `[sp-92]` | `sp-24` | `?` |
| `bl` at 0x6d324 | `(unknown callee) 0x9bf04` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x6d340 | `(unknown callee) 0x1e71c` | `ret(0x9bf04)` | `?` | `?` | `ret(0x9bf04)` |
| `bl` at 0x6d354 | `(unknown callee) 0x1dba4` | `0xe8b64` | `[sp-28]` | `ret(0x1e71c)` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6d174 | `[0x27ee70] != 0x0` | `[0x27ee70] == 0x0` |
| `beq` at 0x6d198 | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `beq` at 0x6d1fc | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `blt` at 0x6d260 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6d280 | `ret(0x2fac0) != 0x4` | `ret(0x2fac0) == 0x4` |
| `beq` at 0x6d338 | `ret(0x9bf04) == 0x0` | `ret(0x9bf04) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6d358 | `r0` | `?` |

## `syscom_process_outbound_packet`

- `syscom_process_outbound_packet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e984 to 0x6ea00.
  Established. A byte load from `r0 +0x16` feeds the comparison tree (`ldrb` at 0x6e99c); its
  equality branches take paths for `0x1`, `0x0` and `0x2` (`beq` at 0x6e9ac, `beq` at 0x6e9c4, `beq`
  at 0x6e9d4). These lead to calls to `0x6e214`, `0x6d6b8` and `0x6db34`, respectively (`bl` at
  0x6e9f8, `bl` at 0x6e9e0, `bl` at 0x6e9ec). Each call has candidates entry r0, entry r1, entry r2
  and `[r0+22]:1` (`bl` at 0x6e9f8, `bl` at 0x6e9e0, `bl` at 0x6e9ec). The packet-class meanings and
  merged return value remain unknown (`ldmdb` at 0x6e9fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6e994 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6e998 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e99c | load | 1 | `r0 +0x16` | `-` |
| `str` at 0x6e9a0 | store | 4 | `sp -0x18` | `[r0+22]:1` |
| `ldr` at 0x6e9a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6e9b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6e9bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6e9cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6e9dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e9e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e9f4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6e9fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6e9fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6e9fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6e9e0 | `(unknown callee) 0x6d6b8` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x6e9ec | `(unknown callee) 0x6db34` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x6e9f8 | `(unknown callee) 0x6e214` | `r0` | `r1` | `r2` | `[r0+22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6e9ac | `[r0+22]:1 == 0x1` | `[r0+22]:1 != 0x1` |
| `bgt` at 0x6e9b8 | `[r0+22]:1 > 0x1 (signed, 32-bit)` | `[r0+22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x6e9c4 | `[r0+22]:1 == 0x0` | `[r0+22]:1 != 0x0` |
| `beq` at 0x6e9d4 | `[r0+22]:1 == 0x2` | `[r0+22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6e9fc | `r0` | `?` |

## `syscom_process_inbound_packet`

- `syscom_process_inbound_packet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6d36c to 0x6d448.
  Established. The two equality branches take their edges when `[sp-32]` equals `0x0` or `0x2`,
  respectively (`beq` at 0x6d3a4, `beq` at 0x6d3b0). The first path increments absolute word
  `0x11b068` and stores `([0x11b074] + [sp-28])` at absolute `0x11b074` before calling `0x6cd60`
  (`str` at 0x6d3c8, `str` at 0x6d3e0, `bl` at 0x6d3f0). The second increments `0x11b070` and stores
  `([0x11b07c] + [sp-28])` at `0x11b07c` before calling `0x6d140` (`str` at 0x6d408, `str` at
  0x6d420, `bl` at 0x6d430). The input class and counter units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6d37c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6d380 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6d384 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x6d388 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6d38c | load | 1 | `r1 +0x0` | `-` |
| `str` at 0x6d398 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6d39c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d3a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d3b8 | load | 4 | `abs 0x6d438` | `-` |
| `ldr` at 0x6d3bc | load | 4 | `abs 0x6d438` | `-` |
| `ldr` at 0x6d3c0 | load | 4 | `abs 0x11b068` | `-` |
| `str` at 0x6d3c8 | store | 4 | `abs 0x11b068` | `[0x11b068]+1` |
| `ldr` at 0x6d3cc | load | 4 | `abs 0x6d43c` | `-` |
| `ldr` at 0x6d3d0 | load | 4 | `abs 0x6d43c` | `-` |
| `ldr` at 0x6d3d4 | load | 4 | `abs 0x11b074` | `-` |
| `ldr` at 0x6d3d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6d3e0 | store | 4 | `abs 0x11b074` | `([0x11b074] + [sp-28])` |
| `ldr` at 0x6d3e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d3e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6d3ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6d3f8 | load | 4 | `abs 0x6d440` | `-` |
| `ldr` at 0x6d3fc | load | 4 | `abs 0x6d440` | `-` |
| `ldr` at 0x6d400 | load | 4 | `abs 0x11b070` | `-` |
| `str` at 0x6d408 | store | 4 | `abs 0x11b070` | `[0x11b070]+1` |
| `ldr` at 0x6d40c | load | 4 | `abs 0x6d444` | `-` |
| `ldr` at 0x6d410 | load | 4 | `abs 0x6d444` | `-` |
| `ldr` at 0x6d414 | load | 4 | `abs 0x11b07c` | `-` |
| `ldr` at 0x6d418 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6d420 | store | 4 | `abs 0x11b07c` | `([0x11b07c] + [sp-28])` |
| `ldr` at 0x6d424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6d42c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6d434 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6d434 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6d434 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6d3f0 | `(unknown callee) 0x6cd60` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x11b074] + [sp-28])` |
| `bl` at 0x6d430 | `(unknown callee) 0x6d140` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x11b07c] + [sp-28])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6d3a4 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x6d3b0 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6d434 | `r0` | `?` |

## `SysComEnable`

- `SysComEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6cb94 to 0x6cc2c.
  Established. A word at `sp -0x14` starts at `0x0` and is updated to `[sp-20]+1` on each decoded
  repeat path (`str` at 0x6cba8, `str` at 0x6cc18). The loop branch reports a signed less-than flags
  condition with unknown operands (`blt` at 0x6cbbc). One call targets `0xb7c74` with first
  candidate unknown, second `0x2` and the remaining candidates unknown (`bl` at 0x6cc0c). The return
  value is unknown (`ldmdb` at 0x6cc20). Neither the exact iteration bound nor the enabled hardware
  state is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6cba8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x6cbac | load | 4 | `abs 0x6cc24` | `-` |
| `ldr` at 0x6cbb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cbb4 | load | 4 | `abs 0x281fd0` | `-` |
| `ldr` at 0x6cbc4 | load | 4 | `abs 0x6cc28` | `-` |
| `ldr` at 0x6cbc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cbd0 | load | 4 | `abs 0x6cc24` | `-` |
| `ldr` at 0x6cbd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cbf8 | load | 4 | `abs 0x281fd4` | `-` |
| `ldrb` at 0x6cc00 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6cc10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6cc18 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x6cc20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6cc20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6cc20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6cbcc | `(unknown callee) 0x1dba4` | `0xe8ad0` | `?` | `?` | `[0x281fd0]` |
| `bl` at 0x6cc0c | `(unknown callee) 0xb7c74` | `?` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6cbbc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6cc20 | `r0` | `?` |
