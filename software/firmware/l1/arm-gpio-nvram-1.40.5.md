# ARM GPIO and NVRAM callers in 1.40.5

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
nvramInit, nvramSet and nvramGet have unresolved computed transfers. Their tables and returns
cover known edges only. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/gpio-nvram-facts.txt) retain all tool sections.

## `l1ioGpioInit`

- `l1ioGpioInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9c0c to 0xc9c8c.
  Established. The unnamed `0x1b400` call carries `0x133fb8`, `0x2`, `r2`, `r3` (`bl` at 0xc9c28);
  its result is assigned as four bytes at absolute `0x14370c` (`str` at 0xc9c34). Signed-nonnegative
  `[0x14370c]` takes the branch to `0xc9c78` (`bge` at 0xc9c40). The negative fall-through includes
  an unnamed `0x21eb8` call with `0x1`, `0x133fe0`, `0x34`, `0x0` (`bl` at 0xc9c74). GPIO
  initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9c1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc9c20 | load | 4 | `abs 0xc9c7c` | `-` |
| `ldr` at 0xc9c30 | load | 4 | `abs 0xc9c80` | `-` |
| `str` at 0xc9c34 | store | 4 | `abs 0x14370c` | `ret(0x1b400)` |
| `ldr` at 0xc9c38 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9c4c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xc9c58 | load | 4 | `abs 0xc9c84` | `-` |
| `str` at 0xc9c5c | store | 4 | `sp -0x1c` | `0x133fc8` |
| `str` at 0xc9c60 | store | 4 | `sp -0x18` | `ret(0x1b124)` |
| `ldr` at 0xc9c68 | load | 4 | `abs 0xc9c88` | `-` |
| `ldmdb` at 0xc9c78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9c78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9c78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9c28 | `(unknown callee) 0x1b400` | `0x133fb8` | `0x2` | `r2` | `r3` |
| `bl` at 0xc9c44 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `ret(0x1b400)` | `[0x14370c]` |
| `bl` at 0xc9c50 | `(unknown callee) 0x1b124` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xc9c74 | `(unknown callee) 0x21eb8` | `0x1` | `0x133fe0` | `0x34` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9c40 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9c78 | `r0` | `?` |

## `l1ioGpioReset`

- `l1ioGpioReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9c8c to 0xc9e74.
  Established. A signed-nonnegative `[0x14370c]` takes the branch to `0xc9cc8` (`bge` at 0xc9cb8).
  Included unnamed `0x1b49c` calls carry second candidates `0x40046706`, `0x40046700`, `0x40046704`,
  `0x40046702` and `0x80046708`, with the complete candidates below (`bl` at 0xc9cec, `bl` at
  0xc9d28, `bl` at 0xc9d74, `bl` at 0xc9dc0, `bl` at 0xc9dfc). Two byte stack assignments at
  `sp -0x1b` have unknown values (`strb` at 0xc9d60, `strb` at 0xc9dac). Reset and request-field
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9c9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9ca0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc9ca8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc9cac | load | 4 | `abs 0xc9e58` | `-` |
| `ldr` at 0xc9cb0 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9cbc | load | 4 | `abs 0xc9e5c` | `-` |
| `str` at 0xc9cc0 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc9cc8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9ccc | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc9cd0 | store | 1 | `sp -0x1c` | `([r1+3]:1 & 0xff)` |
| `strb` at 0xc9cd8 | store | 1 | `sp -0x1b` | `0x0` |
| `ldr` at 0xc9cdc | load | 4 | `abs 0xc9e58` | `-` |
| `ldr` at 0xc9ce4 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9ce8 | load | 4 | `abs 0xc9e60` | `-` |
| `ldr` at 0xc9d04 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9d08 | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `ldr` at 0xc9d0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9d10 | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xc9d14 | store | 1 | `sp -0x1b` | `([[sp-24]+6]:1 & 0xff)` |
| `ldr` at 0xc9d18 | load | 4 | `abs 0xc9e58` | `-` |
| `ldr` at 0xc9d20 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9d24 | load | 4 | `abs 0xc9e64` | `-` |
| `ldr` at 0xc9d38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9d4c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9d50 | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `ldr` at 0xc9d54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9d58 | load | 1 | `[sp-24] +0x5` | `-` |
| `strb` at 0xc9d60 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0xc9d64 | load | 4 | `abs 0xc9e58` | `-` |
| `ldr` at 0xc9d6c | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9d70 | load | 4 | `abs 0xc9e68` | `-` |
| `ldr` at 0xc9d84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9d98 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9d9c | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `ldr` at 0xc9da0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9da4 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xc9dac | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0xc9db0 | load | 4 | `abs 0xc9e58` | `-` |
| `ldr` at 0xc9db8 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9dbc | load | 4 | `abs 0xc9e6c` | `-` |
| `ldr` at 0xc9dd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9de4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9de8 | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `ldr` at 0xc9dec | load | 4 | `abs 0xc9e58` | `-` |
| `ldr` at 0xc9df4 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9df8 | load | 4 | `abs 0xc9e70` | `-` |
| `ldr` at 0xc9e0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9e20 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9e24 | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `ldr` at 0xc9e28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9e34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9e40 | store | 4 | `sp -0x24` | `ret(0xca2b8)` |
| `str` at 0xc9e4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9e50 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc9e54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9e54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9e54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9cec | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x40046706` | `sp-28` | `0x14370c` |
| `bl` at 0xc9cfc | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xc9d28 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x40046700` | `sp-28` | `0x14370c` |
| `bl` at 0xc9d44 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc9d74 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x40046704` | `sp-28` | `0x14370c` |
| `bl` at 0xc9d90 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc9dc0 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x40046702` | `sp-28` | `0x14370c` |
| `bl` at 0xc9ddc | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc9dfc | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x80046708` | `sp-28` | `0x14370c` |
| `bl` at 0xc9e18 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc9e38 | `(unknown callee) 0xca2b8` | `[sp-32]` | `?` | `?` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9cb8 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9cf8 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `beq` at 0xc9d34 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `bne` at 0xc9d40 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc9d80 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `bne` at 0xc9d8c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc9dcc | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `bne` at 0xc9dd8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc9e08 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `bne` at 0xc9e14 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xc9e30 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9e54 | `r0` | `?` |

## `l1ioGpioWrite`

- `l1ioGpioWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9e74 to 0xc9f1c.
  Established. The unnamed `0x1b49c` call has `[0x14370c]`, `0x40046700`, `sp-32`, `0x14370c` (`bl`
  at 0xc9ed4). Zero result takes the branch to `0xc9f00` (`beq` at 0xc9ee0). The other path reads
  four bytes at `ret(0x1b148) +0x0`, then calls unnamed `0xca2b8` with `[ret(0x1b148)]`, unknown,
  unknown, `ret(0x1b148)` (`ldr` at 0xc9eec, `bl` at 0xc9ef0). GPIO output and error meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9e84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9e88 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc9e90 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xc9e94 | load | 4 | `abs 0xc9f10` | `-` |
| `ldr` at 0xc9e98 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9ea4 | load | 4 | `abs 0xc9f14` | `-` |
| `str` at 0xc9ea8 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc9eb0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9eb4 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc9eb8 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0xc9ebc | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc9ec0 | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0xc9ec4 | load | 4 | `abs 0xc9f10` | `-` |
| `ldr` at 0xc9ecc | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9ed0 | load | 4 | `abs 0xc9f18` | `-` |
| `ldr` at 0xc9eec | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9ef8 | store | 4 | `sp -0x24` | `ret(0xca2b8)` |
| `str` at 0xc9f04 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9f08 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc9f0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9f0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9f0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9ed4 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x40046700` | `sp-32` | `0x14370c` |
| `bl` at 0xc9ee4 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xc9ef0 | `(unknown callee) 0xca2b8` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9ea0 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9ee0 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9f0c | `r0` | `?` |

## `l1ioGpioRead`

- `l1ioGpioRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9f1c to 0xc9fc4.
  Established. A byte assignment at `sp -0x20` retains `([r1+3]:1 & 0xff)` (`strb` at 0xc9f5c). The
  unnamed `0x1b49c` call uses `[0x14370c]`, `0xc0046701`, `sp-32`, `0x14370c` (`bl` at 0xc9f70); its
  zero result takes the branch to `0xc9f9c`, where a byte store writes `([sp-31]:1 & 0xff)` at
  `[sp-28] +0x0` (`beq` at 0xc9f7c, `strb` at 0xc9fa4). GPIO input and request semantics are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9f2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9f30 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc9f34 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc9f38 | load | 4 | `abs 0xc9fb8` | `-` |
| `ldr` at 0xc9f3c | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9f48 | load | 4 | `abs 0xc9fbc` | `-` |
| `str` at 0xc9f4c | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xc9f54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc9f58 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xc9f5c | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0xc9f60 | load | 4 | `abs 0xc9fb8` | `-` |
| `ldr` at 0xc9f68 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9f6c | load | 4 | `abs 0xc9fc0` | `-` |
| `ldr` at 0xc9f88 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc9f94 | store | 4 | `sp -0x24` | `ret(0xca2b8)` |
| `ldr` at 0xc9f9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc9fa0 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xc9fa4 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0xc9fac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9fb0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc9fb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9fb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9fb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9f70 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0xc0046701` | `sp-32` | `0x14370c` |
| `bl` at 0xc9f80 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xc9f8c | `(unknown callee) 0xca2b8` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9f44 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9f7c | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9fb4 | `r0` | `?` |

## `l1ioGpioIntSet`

- `l1ioGpioIntSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9fc4 to 0xca06c.
  Established. The unnamed `0x1b49c` call uses `[0x14370c]`, `0x40046706`, `sp-32`, `0x14370c` (`bl`
  at 0xca024); its zero result takes the branch to `0xca050` (`beq` at 0xca030). Earlier byte
  assignments at `sp -0x20` and `sp -0x1f` retain `([r1+3]:1 & 0xff)` and `((r2 & 0xff) & 0xff)`
  (`strb` at 0xca008, `strb` at 0xca010). Interrupt-mask meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9fd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc9fd8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc9fe0 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xc9fe4 | load | 4 | `abs 0xca060` | `-` |
| `ldr` at 0xc9fe8 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xc9ff4 | load | 4 | `abs 0xca064` | `-` |
| `str` at 0xc9ff8 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xca000 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca004 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xca008 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0xca00c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xca010 | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0xca014 | load | 4 | `abs 0xca060` | `-` |
| `ldr` at 0xca01c | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xca020 | load | 4 | `abs 0xca068` | `-` |
| `ldr` at 0xca03c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xca048 | store | 4 | `sp -0x24` | `ret(0xca2b8)` |
| `str` at 0xca054 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca058 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xca05c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca05c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca05c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca024 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x40046706` | `sp-32` | `0x14370c` |
| `bl` at 0xca034 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xca040 | `(unknown callee) 0xca2b8` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc9ff0 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xca030 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca05c | `r0` | `?` |

## `l1ioGpioIntGet`

- `l1ioGpioIntGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca06c to 0xca114.
  Established. A signed-nonnegative `[0x14370c]` takes the branch to `0xca0a4` (`bge` at 0xca094).
  The unnamed `0x1b49c` call then carries `[0x14370c]`, `0xc0046707`, `sp-32`, `0x14370c` (`bl` at
  0xca0c0); its zero result takes the branch to `0xca0ec`, where a byte store writes
  `([sp-31]:1 & 0xff)` at `[sp-28] +0x0` (`beq` at 0xca0cc, `strb` at 0xca0f4). Interrupt-state
  encoding remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca07c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca080 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca084 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xca088 | load | 4 | `abs 0xca108` | `-` |
| `ldr` at 0xca08c | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xca098 | load | 4 | `abs 0xca10c` | `-` |
| `str` at 0xca09c | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0xca0a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca0a8 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xca0ac | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0xca0b0 | load | 4 | `abs 0xca108` | `-` |
| `ldr` at 0xca0b8 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xca0bc | load | 4 | `abs 0xca110` | `-` |
| `ldr` at 0xca0d8 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xca0e4 | store | 4 | `sp -0x24` | `ret(0xca2b8)` |
| `ldr` at 0xca0ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca0f0 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xca0f4 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0xca0fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca100 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xca104 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca104 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca104 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca0c0 | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0xc0046707` | `sp-32` | `0x14370c` |
| `bl` at 0xca0d0 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xca0dc | `(unknown callee) 0xca2b8` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xca094 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |
| `beq` at 0xca0cc | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca104 | `r0` | `?` |

## `l1ioGpioInt`

- `l1ioGpioInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca114 to 0xca2b8.
  Established. The unnamed `0x1b49c` call carries `[0x14370c]`, `0x80046708`, `sp-36`, `0x14370c`
  (`bl` at 0xca18c). Zero result takes the branch to `0xca1f4` (`beq` at 0xca198). A later byte read
  targets `[sp-28] +0x3` (`ldrb` at 0xca1fc). The unnamed `0xd7afc` call uses `0x0`, `[sp-24]`,
  `[sp-28]`, `[sp-29]:1`, and its result enters a four-byte sum at `sp -0x28` (`bl` at 0xca22c,
  `str` at 0xca23c). Interrupt-source effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca124 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca128 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca12c | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xca130 | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `str` at 0xca138 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xca13c | load | 4 | `abs 0xca2a0` | `-` |
| `ldr` at 0xca140 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xca14c | load | 4 | `abs 0xca2a4` | `-` |
| `str` at 0xca150 | store | 4 | `sp -0x30` | `0xfffff3e4` |
| `ldr` at 0xca158 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca15c | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0xca160 | store | 1 | `sp -0x24` | `([r1+3]:1 & 0xff)` |
| `str` at 0xca168 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xca16c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca17c | load | 4 | `abs 0xca2a0` | `-` |
| `ldr` at 0xca184 | load | 4 | `abs 0x14370c` | `-` |
| `ldr` at 0xca188 | load | 4 | `abs 0xca2a8` | `-` |
| `ldr` at 0xca1a4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xca1bc | load | 4 | `abs 0xca2ac` | `-` |
| `str` at 0xca1c0 | store | 4 | `sp -0x3c` | `0x133fec` |
| `ldr` at 0xca1c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca1c8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xca1cc | store | 4 | `sp -0x38` | `[[sp-24]+12]` |
| `str` at 0xca1d0 | store | 4 | `sp -0x34` | `ret(0x21a38)` |
| `ldr` at 0xca1d8 | load | 4 | `abs 0xca2b0` | `-` |
| `ldr` at 0xca1e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xca1ec | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xca1f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xca1f8 | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0xca1fc | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0xca210 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xca214 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xca218 | store | 4 | `sp -0x3c` | `[sp-35]:1` |
| `ldr` at 0xca220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca234 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xca23c | store | 4 | `sp -0x28` | `([sp-40] + ret(0xd7afc))` |
| `ldr` at 0xca244 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xca248 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xca250 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xca258 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xca260 | load | 4 | `abs 0xca2b4` | `-` |
| `str` at 0xca264 | store | 4 | `sp -0x3c` | `0x134018` |
| `ldr` at 0xca268 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca26c | load | 4 | `(unknown)` | `-` |
| `str` at 0xca270 | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0xca274 | load | 1 | `sp -0x23` | `-` |
| `str` at 0xca278 | store | 4 | `sp -0x34` | `[sp-35]:1` |
| `ldr` at 0xca280 | load | 4 | `abs 0xca2b0` | `-` |
| `ldr` at 0xca290 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xca294 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xca298 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xca29c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca29c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca29c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca18c | `(unknown callee) 0x1b49c` | `[0x14370c]` | `0x80046708` | `sp-36` | `0x14370c` |
| `bl` at 0xca19c | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xca1a8 | `(unknown callee) 0xca2b8` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xca1b4 | `(unknown callee) 0x21a38` | `ret(0xca2b8)` | `?` | `?` | `ret(0xca2b8)` |
| `bl` at 0xca1e4 | `(unknown callee) 0x21eb8` | `0x3` | `0x133fe0` | `0xda` | `0x0` |
| `bl` at 0xca22c | `(unknown callee) 0xd7afc` | `0x0` | `[sp-24]` | `[sp-28]` | `[sp-29]:1` |
| `bl` at 0xca28c | `(unknown callee) 0x21eb8` | `0x3` | `0x133fe0` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xca148 | `[0x14370c] >= 0x0 (signed, 32-bit)` | `[0x14370c] < 0x0 (signed, 32-bit)` |
| `ble` at 0xca174 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xca198 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `beq` at 0xca20c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca29c | `r0` | `?` |

## `nvramInit`

- `nvramInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1bcf4 to 0x1cbac.
  Established. A four-byte absolute assignment writes `[0x145b70]+1` at `0x145b70` (`str` at
  0x1bd48). The unnamed `0x1b400` call carries `0xfc768`, `0x1042`, `0x1b6`, `0xfc768`, with its
  result written as four bytes at `0x145b94` (`bl` at 0x1bd70, `str` at 0x1bd7c). The computed
  transfer has an unknown target and takes its edge when an unknown expression is at most `0x9`,
  unsigned 32-bit (`ldrls` at 0x1bf18). Known later byte stores include `0x0` at `[0x145b0c] +0x7cc`
  and `0x3` at `[0x145b0c] +0x7c8` (`strb` at 0x1c644, `strb` at 0x1c6b4). NVRAM format and selector
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1bd08 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1bd0c | load | 4 | `abs 0x1cb3c` | `-` |
| `ldr` at 0x1bd10 | load | 4 | `abs 0x145b70` | `-` |
| `ldr` at 0x1bd1c | load | 4 | `abs 0x1cb40` | `-` |
| `str` at 0x1bd20 | store | 4 | `sp -0x68` | `0xfc748` |
| `ldr` at 0x1bd28 | load | 4 | `abs 0x1cb44` | `-` |
| `ldr` at 0x1bd38 | load | 4 | `abs 0x1cb3c` | `-` |
| `ldr` at 0x1bd3c | load | 4 | `abs 0x1cb3c` | `-` |
| `ldr` at 0x1bd40 | load | 4 | `abs 0x145b70` | `-` |
| `str` at 0x1bd48 | store | 4 | `abs 0x145b70` | `[0x145b70]+1` |
| `ldr` at 0x1bd4c | load | 4 | `abs 0x1cb48` | `-` |
| `ldr` at 0x1bd5c | load | 4 | `abs 0x1cb4c` | `-` |
| `str` at 0x1bd60 | store | 4 | `sp -0x24` | `0xfc768` |
| `ldr` at 0x1bd64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1bd68 | load | 4 | `abs 0x1cb50` | `-` |
| `ldr` at 0x1bd6c | load | 4 | `abs 0x1cb54` | `-` |
| `ldr` at 0x1bd78 | load | 4 | `abs 0x1cb58` | `-` |
| `str` at 0x1bd7c | store | 4 | `abs 0x145b94` | `ret(0x1b400)` |
| `ldr` at 0x1bd80 | load | 4 | `abs 0x145b94` | `-` |
| `ldr` at 0x1bd94 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0x1bda0 | load | 4 | `abs 0x1cb5c` | `-` |
| `str` at 0x1bda4 | store | 4 | `sp -0x68` | `0xfc778` |
| `ldr` at 0x1bda8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1bdac | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x1bdb0 | store | 4 | `sp -0x60` | `ret(0x21704)` |
| `ldr` at 0x1bdb8 | load | 4 | `abs 0x1cb44` | `-` |
| `ldr` at 0x1bdc8 | load | 4 | `abs 0x1cb60` | `-` |
| `ldr` at 0x1bdd8 | load | 4 | `abs 0x1cb58` | `-` |
| `ldr` at 0x1bddc | load | 4 | `abs 0x145b94` | `-` |
| `ldr` at 0x1bde0 | load | 4 | `abs 0x1cb60` | `-` |
| `ldr` at 0x1be00 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0x1be0c | load | 4 | `abs 0x1cb64` | `-` |
| `str` at 0x1be10 | store | 4 | `sp -0x68` | `0xfc79c` |
| `ldr` at 0x1be14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1be18 | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x1be1c | store | 4 | `sp -0x60` | `ret(0x21704)` |
| `ldr` at 0x1be24 | load | 4 | `abs 0x1cb44` | `-` |
| `ldr` at 0x1be34 | load | 4 | `abs 0x1cb68` | `-` |
| `ldr` at 0x1be38 | load | 4 | `abs 0x1cb6c` | `-` |
| `ldr` at 0x1be48 | load | 4 | `abs 0x1cb70` | `-` |
| `ldr` at 0x1be4c | load | 4 | `abs 0x1cb60` | `-` |
| `str` at 0x1be50 | store | 4 | `abs 0x145b04` | `0x2af398` |
| `ldr` at 0x1be54 | load | 4 | `abs 0x1cb70` | `-` |
| `ldr` at 0x1be58 | load | 4 | `abs 0x145b04` | `-` |
| `ldr` at 0x1be78 | load | 4 | `abs 0x1cb70` | `-` |
| `ldr` at 0x1be7c | load | 4 | `abs 0x145b04` | `-` |
| `ldrb` at 0x1be80 | load | 1 | `[0x145b04] +0x0` | `-` |
| `ldr` at 0x1be94 | load | 4 | `abs 0x1cb70` | `-` |
| `ldr` at 0x1be98 | load | 4 | `abs 0x145b04` | `-` |
| `ldrb` at 0x1be9c | load | 1 | `[0x145b04] +0x1` | `-` |
| `ldr` at 0x1beac | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1beb0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1bebc | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1bec0 | load | 4 | `abs 0x1cb70` | `-` |
| `ldr` at 0x1bec4 | load | 4 | `abs 0x145b04` | `-` |
| `ldrb` at 0x1bec8 | load | 1 | `[0x145b04] +0x1` | `-` |
| `ldr` at 0x1bed0 | load | 4 | `abs 0x1cb60` | `-` |
| `str` at 0x1bed8 | store | 4 | `abs 0x145b0c` | `?` |
| `ldr` at 0x1bedc | load | 4 | `abs 0x1cb78` | `-` |
| `ldr` at 0x1bee0 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1bee4 | load | 4 | `abs 0x145b0c` | `-` |
| `str` at 0x1beec | store | 4 | `abs 0x145b10` | `[0x145b0c]+2352` |
| `ldr` at 0x1bef0 | load | 4 | `abs 0x1cb7c` | `-` |
| `ldr` at 0x1bef4 | load | 4 | `abs 0x1cb80` | `-` |
| `str` at 0x1bef8 | store | 4 | `abs 0x145b14` | `0x2b1390` |
| `ldr` at 0x1befc | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1bf00 | load | 4 | `abs 0x145b0c` | `-` |
| `ldrb` at 0x1bf04 | load | 1 | `[0x145b0c] +0x0` | `-` |
| `ldrls` at 0x1bf18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1c5b8 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c5bc | load | 4 | `abs 0x145b0c` | `-` |
| `str` at 0x1c5dc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1c5e0 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c5e4 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c5e8 | load | 4 | `abs 0x1cb90` | `-` |
| `ldrb` at 0x1c5ec | load | 1 | `[0x145b0c] +0x7c8` | `-` |
| `ldr` at 0x1c5fc | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c600 | load | 4 | `abs 0x145b0c` | `-` |
| `str` at 0x1c62c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1c634 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c638 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c63c | load | 4 | `abs 0x1cb94` | `-` |
| `strb` at 0x1c644 | store | 1 | `[0x145b0c] +0x7cc` | `0x0` |
| `ldr` at 0x1c648 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c64c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c650 | load | 4 | `[0x145b0c] +0x7cc` | `-` |
| `str` at 0x1c654 | store | 4 | `sp -0x5c` | `[[0x145b0c]+1996]` |
| `ldr` at 0x1c658 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x1c660 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x1c664 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x1c668 | store | 4 | `[0x145b0c] +0x7cc` | `[sp-92]` |
| `ldr` at 0x1c66c | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c670 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c688 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c68c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c6a4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c6a8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c6ac | load | 4 | `abs 0x1cb90` | `-` |
| `strb` at 0x1c6b4 | store | 1 | `[0x145b0c] +0x7c8` | `0x3` |
| `ldr` at 0x1c6b8 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c6bc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c6c0 | load | 4 | `abs 0x1cb88` | `-` |
| `ldr` at 0x1c6c4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c6c8 | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x1c6d4 | store | 1 | `[0x145b0c] +0x3bf` | `(ret(0x21dc4) & 0xff)` |
| `ldr` at 0x1c6d8 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c6dc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c6e0 | load | 4 | `abs 0x1cb98` | `-` |
| `ldr` at 0x1c6e4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c6e8 | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x1c700 | store | 1 | `[0x145b0c] +0x87b` | `(ret(0x21dc4) & 0xff)` |
| `ldr` at 0x1c704 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c708 | load | 4 | `abs 0x145b0c` | `-` |
| `ldrb` at 0x1c710 | load | 1 | `[0x145b0c] +0x780` | `-` |
| `ldr` at 0x1c720 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c724 | load | 4 | `abs 0x145b0c` | `-` |
| `str` at 0x1c74c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1c754 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c758 | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x1c764 | store | 1 | `[0x145b0c] +0x780` | `0x1` |
| `ldr` at 0x1c768 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c76c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c770 | load | 4 | `abs 0x1cb9c` | `-` |
| `ldr` at 0x1c774 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c778 | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x1c78c | store | 1 | `[0x145b0c] +0x7a3` | `(ret(0x21dc4) & 0xff)` |
| `ldr` at 0x1c790 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1c7a4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c7a8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c7b0 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c7b8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c7c4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c7c8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c7d4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c7d8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c7f4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c7f8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c804 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c808 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c860 | load | 4 | `abs 0x1cb7c` | `-` |
| `ldr` at 0x1c864 | load | 4 | `abs 0x1cb78` | `-` |
| `ldr` at 0x1c868 | load | 4 | `abs 0x145b14` | `-` |
| `ldr` at 0x1c86c | load | 4 | `abs 0x145b10` | `-` |
| `ldr` at 0x1c874 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c878 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c87c | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c880 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c884 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x1c888 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x1c898 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c89c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c8a0 | load | 4 | `abs 0x1cb78` | `-` |
| `ldr` at 0x1c8a4 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x1c8a8 | load | 4 | `abs 0x145b10` | `-` |
| `ldr` at 0x1c8b4 | load | 4 | `abs 0x1cb78` | `-` |
| `ldr` at 0x1c8b8 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c8bc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c8c0 | load | 4 | `abs 0x145b10` | `-` |
| `ldr` at 0x1c8c4 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x1c8cc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x1c8d0 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c8d4 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c8d8 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c8dc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c8e0 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x1c8e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c8ec | store | 4 | `[0x145b0c] +0x784` | `([[0x145b0c]+1924] + [sp-36])` |
| `ldr` at 0x1c8f0 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c8f4 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c8f8 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c8fc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c900 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x1c904 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c90c | store | 4 | `[0x145b0c] +0x788` | `([[0x145b0c]+1928] + [sp-36])` |
| `ldr` at 0x1c910 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c914 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c918 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c91c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c920 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x1c924 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c92c | store | 4 | `[0x145b0c] +0x78c` | `([[0x145b0c]+1932] + [sp-36])` |
| `ldr` at 0x1c930 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c934 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c938 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c93c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c940 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x1c944 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c94c | store | 4 | `[0x145b0c] +0x790` | `([[0x145b0c]+1936] + [sp-36])` |
| `ldr` at 0x1c950 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c954 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c958 | load | 4 | `abs 0x1cb9c` | `-` |
| `ldr` at 0x1c95c | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c960 | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x1c974 | store | 1 | `[0x145b0c] +0x7a3` | `(ret(0x21dc4) & 0xff)` |
| `ldr` at 0x1c978 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c97c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c988 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c98c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c9a4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c9a8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c9ac | load | 4 | `abs 0x1cb78` | `-` |
| `ldr` at 0x1c9b0 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x1c9b4 | load | 4 | `abs 0x145b10` | `-` |
| `ldr` at 0x1c9c0 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c9c4 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c9c8 | load | 4 | `abs 0x1cb7c` | `-` |
| `ldr` at 0x1c9cc | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x1c9d0 | load | 4 | `abs 0x145b14` | `-` |
| `ldr` at 0x1c9dc | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c9e0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c9e4 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1c9e8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1c9ec | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x1c9f0 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x1c9fc | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca00 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca04 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca08 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca0c | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x1ca10 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x1ca1c | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca20 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca24 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca28 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca2c | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x1ca30 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x1ca3c | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca40 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca44 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca48 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca4c | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x1ca50 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x1ca60 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca64 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca68 | load | 4 | `abs 0x1cb78` | `-` |
| `ldr` at 0x1ca6c | load | 4 | `abs 0x145b10` | `-` |
| `str` at 0x1ca70 | store | 4 | `[0x145b0c] +0x784` | `[0x145b10]` |
| `ldr` at 0x1ca74 | load | 4 | `abs 0x1cb74` | `-` |
| `ldr` at 0x1ca78 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x1ca7c | load | 4 | `abs 0x1cb7c` | `-` |
| `ldr` at 0x1ca80 | load | 4 | `abs 0x145b14` | `-` |
| `str` at 0x1ca84 | store | 4 | `[0x145b0c] +0x788` | `[0x145b14]` |
| `ldr` at 0x1ca8c | load | 4 | `abs 0x1cb48` | `-` |
| `ldr` at 0x1ca90 | load | 4 | `abs 0x1cba0` | `-` |
| `ldrb` at 0x1ca94 | load | 1 | `abs 0x145b18` | `-` |
| `strb` at 0x1ca9c | store | 1 | `abs 0x145b18` | `([0x145b18]:1+1 & 0xff)` |
| `ldr` at 0x1caa4 | load | 4 | `abs 0x1cba4` | `-` |
| `str` at 0x1caa8 | store | 4 | `(unknown)` | `0xfc7d4` |
| `ldr` at 0x1caac | load | 4 | `abs 0x1cba0` | `-` |
| `ldrb` at 0x1cab0 | load | 1 | `abs 0x145b18` | `-` |
| `ldr` at 0x1cabc | load | 4 | `abs 0x1cba0` | `-` |
| `strb` at 0x1cac4 | store | 1 | `abs 0x145b18` | `0x0` |
| `ldr` at 0x1cac8 | load | 4 | `abs 0x1cb48` | `-` |
| `ldr` at 0x1cacc | load | 4 | `abs 0x1cba0` | `-` |
| `ldrb` at 0x1cad0 | load | 1 | `abs 0x145b18` | `-` |
| `ldr` at 0x1cad8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1cae8 | load | 4 | `abs 0x1cb48` | `-` |
| `ldr` at 0x1caec | load | 4 | `abs 0x1cba0` | `-` |
| `ldrb` at 0x1caf0 | load | 1 | `abs 0x145b18` | `-` |
| `ldr` at 0x1caf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1cb00 | load | 4 | `abs 0x1cba0` | `-` |
| `ldr` at 0x1cb04 | load | 4 | `abs 0x1cba0` | `-` |
| `ldrb` at 0x1cb08 | load | 1 | `abs 0x145b18` | `-` |
| `strb` at 0x1cb10 | store | 1 | `abs 0x145b18` | `([0x145b18]:1+1 & 0xff)` |
| `ldr` at 0x1cb28 | load | 4 | `abs 0x1cba8` | `-` |
| `ldmdb` at 0x1cb38 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1cb38 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1cb38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1cb38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1cb38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1bd34 | `(unknown callee) 0x21eb8` | `0x9` | `0xfc6d0` | `0x71` | `0x0` |
| `bl` at 0x1bd58 | `(unknown callee) 0x1b4cc` | `0x145b74` | `0x0` | `0x20` | `[0x145b70]+1` |
| `bl` at 0x1bd70 | `(unknown callee) 0x1b400` | `0xfc768` | `0x1042` | `0x1b6` | `0xfc768` |
| `bl` at 0x1bd8c | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `ret(0x1b400)` | `[0x145b94]` |
| `bl` at 0x1bd98 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0x1bdc4 | `(unknown callee) 0x21eb8` | `0x9` | `0xfc6d0` | `0x82` | `0x0` |
| `bl` at 0x1bdd4 | `(unknown callee) 0x1b4cc` | `0x2af398` | `0x0` | `0x2000` | `?` |
| `bl` at 0x1bde8 | `(unknown callee) 0x1b58c` | `[0x145b94]` | `0x2af398` | `0x2000` | `0x145b94` |
| `bl` at 0x1bdf8 | `(unknown callee) 0x1b148` | `ret(0x1b58c)` | `?` | `?` | `ret(0x1b58c)` |
| `bl` at 0x1be04 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0x1be30 | `(unknown callee) 0x21eb8` | `0x9` | `0xfc6d0` | `0x88` | `0x0` |
| `bl` at 0x1be44 | `(unknown callee) 0x229b0` | `0x145b1c` | `0xfc7c8` | `0x1` | `0x9` |
| `bl` at 0x1be60 | `(unknown callee) 0x21d50` | `[0x145b04]` | `0x8` | `0x145b04` | `0x145b04` |
| `bl` at 0x1be74 | `(unknown callee) 0x1d318` | `ret(0x21d50)` | `?` | `?` | `?` |
| `bl` at 0x1bea8 | `(unknown callee) 0x1d318` | `?` | `?` | `?` | `?` |
| `bl` at 0x1c5c4 | `(unknown callee) 0x21d50` | `[0x145b0c]` | `0x3c0` | `?` | `0x145b0c` |
| `bl` at 0x1c614 | `(unknown callee) 0x21d50` | `[0x145b0c]+1992` | `0xb4` | `[0x145b0c]` | `[0x145b0c]+1992` |
| `bl` at 0x1c684 | `(unknown callee) 0x1b4cc` | `[0x145b0c]+2000` | `0x0` | `0xa8` | `[0x145b0c]+2000` |
| `bl` at 0x1c6a0 | `(unknown callee) 0x1b4cc` | `[0x145b0c]+952` | `0x0` | `0x7` | `[0x145b0c]+952` |
| `bl` at 0x1c6d0 | `(unknown callee) 0x21dc4` | `[0x145b0c]` | `0x3c0` | `0x7c8` | `0x145b0c` |
| `bl` at 0x1c6fc | `(unknown callee) 0x21dc4` | `[0x145b0c]+1992` | `0xb4` | `?` | `[0x145b0c]+1992` |
| `bl` at 0x1c734 | `(unknown callee) 0x21d50` | `[0x145b0c]+1920` | `0x24` | `[0x145b0c]` | `[0x145b0c]+1920` |
| `bl` at 0x1c788 | `(unknown callee) 0x21dc4` | `[0x145b0c]+1920` | `0x24` | `0x780` | `[0x145b0c]+1920` |
| `bl` at 0x1c79c | `(unknown callee) 0x1cbac` | `?` | `?` | `?` | `[sp-32]` |
| `bl` at 0x1c7c0 | `(unknown callee) 0x1b394` | `[0x145b0c]+960` | `[0x145b0c]` | `0x3c0` | `[0x145b0c]+960` |
| `bl` at 0x1c7f0 | `(unknown callee) 0x1b394` | `[0x145b0c]+2172` | `[0x145b0c]+1992` | `0xb4` | `[0x145b0c]+1992` |
| `bl` at 0x1c81c | `(unknown callee) 0x1b394` | `[0x145b0c]+1956` | `[0x145b0c]+1920` | `0x24` | `[0x145b0c]+1920` |
| `bl` at 0x1c824 | `(unknown callee) 0x1f26c` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x1c83c | `(unknown callee) 0x1d41c` | `0x11` | `0x2` | `?` | `ret(0x1f26c)` |
| `bl` at 0x1c844 | `(unknown callee) 0x1f26c` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x1c85c | `(unknown callee) 0x1d41c` | `0x3d` | `0x2` | `?` | `ret(0x1f26c)` |
| `bl` at 0x1c970 | `(unknown callee) 0x21dc4` | `[0x145b0c]+1920` | `0x24` | `[[0x145b0c]+1936]` | `[0x145b0c]+1920` |
| `bl` at 0x1c9a0 | `(unknown callee) 0x1b394` | `[0x145b0c]+1956` | `[0x145b0c]+1920` | `0x24` | `[0x145b0c]+1920` |
| `bl` at 0x1ca88 | `(unknown callee) 0x20230` | `?` | `?` | `[0x145b0c]` | `[0x145b14]` |
| `bl` at 0x1cafc | `(unknown callee) 0x202d0` | `?` | `?` | `0x145b74` | `?` |
| `bl` at 0x1cb18 | `(unknown callee) 0x1bbdc` | `?` | `?` | `?` | `?` |
| `bl` at 0x1cb20 | `(unknown callee) 0x1f26c` | `0x46` | `?` | `?` | `?` |
| `bl` at 0x1cb34 | `(unknown callee) 0x1b118` | `0xfc7f8` | `ret(0x1f26c)` | `0x1` | `ret(0x1f26c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1bd18 | `[0x145b70] == 0x0` | `[0x145b70] != 0x0` |
| `bge` at 0x1bd88 | `[0x145b94] >= 0x0 (signed, 32-bit)` | `[0x145b94] < 0x0 (signed, 32-bit)` |
| `bge` at 0x1bdf4 | `ret(0x1b58c) >= 0x0 (signed, 32-bit)` | `ret(0x1b58c) < 0x0 (signed, 32-bit)` |
| `beq` at 0x1be70 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1be90 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x1bea4 | `[[0x145b04]+1]:1 != 0x0` | `[[0x145b04]+1]:1 == 0x0` |
| `bne` at 0x1beb8 | `[0x145b0c] != 0x0` | `[0x145b0c] == 0x0` |
| `ldrls` at 0x1bf18 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |
| `beq` at 0x1c5d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1c5f4 | `[[0x145b0c]+1992]:1 == 0x3` | `[[0x145b0c]+1992]:1 != 0x3` |
| `beq` at 0x1c624 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1c718 | `[[0x145b0c]+1920]:1 == 0x1` | `[[0x145b0c]+1920]:1 != 0x1` |
| `beq` at 0x1c744 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1c798 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bhi` at 0x1c830 | `ret(0x1f26c) > 0x1 (unsigned, 32-bit)` | `ret(0x1f26c) <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x1c850 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `bne` at 0x1c894 | `? != ?` | `? == ?` |
| `beq` at 0x1c8b0 | `[[0x145b0c]+1924] == [0x145b10]` | `[[0x145b0c]+1924] != [0x145b10]` |
| `bne` at 0x1c9bc | `[[0x145b0c]+1924] != [0x145b10]` | `[[0x145b0c]+1924] == [0x145b10]` |
| `bne` at 0x1c9d8 | `[[0x145b0c]+1928] != [0x145b14]` | `[[0x145b0c]+1928] == [0x145b14]` |
| `blo` at 0x1c9f8 | `[[0x145b0c]+1932] < [[0x145b0c]+1924] (unsigned, 32-bit)` | `[[0x145b0c]+1932] >= [[0x145b0c]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1ca18 | `[[0x145b0c]+1932] >= [[0x145b0c]+1928] (unsigned, 32-bit)` | `[[0x145b0c]+1932] < [[0x145b0c]+1928] (unsigned, 32-bit)` |
| `blo` at 0x1ca38 | `[[0x145b0c]+1936] < [[0x145b0c]+1924] (unsigned, 32-bit)` | `[[0x145b0c]+1936] >= [[0x145b0c]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1ca58 | `[[0x145b0c]+1936] >= [[0x145b0c]+1928] (unsigned, 32-bit)` | `[[0x145b0c]+1936] < [[0x145b0c]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1cab8 | `[0x145b18]:1 == 0x0` | `[0x145b18]:1 != 0x0` |
| `bne` at 0x1cae0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1cb38 | `r0` | `ret(0x1b118)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1bf18).

## `nvramSet`

- `nvramSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1d41c to 0x1f26c.
  Established. The prefix saves four-byte entry `r0` at `sp -0x1c` and `r1` at `sp -0x20` (`str` at
  0x1d42c, `str` at 0x1d430). The computed transfer takes an unresolved edge when `r0-1 <= 0xaf`
  unsigned 32-bit, and falls through when `r0-1 > 0xaf` (`ldrls` at 0x1d440). The known fall-through
  exit carries entry `r0` (`ldmdb` at 0x1f268). Selector-specific writes and persistent-storage
  semantics are Unknown in this analysis.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1d42c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1d430 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x1d434 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x1d440 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x1f268 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1f268 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1f268 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1f268 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1f268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1d440 | `r0-1 <= 0xaf (unsigned, 32-bit)` | `r0-1 > 0xaf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1f268 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1d440).

## `nvramGet`

- `nvramGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1f26c to 0x20230.
  Established. Entry `r0` is saved as four bytes at `sp -0x14` (`str` at 0x1f27c). The computed
  transfer has unresolved destinations for `r0-1 <= 0xaf` unsigned 32-bit (`ldrls` at 0x1f28c); its
  `r0-1 > 0xaf` fall-through reaches the four-byte zero assignment at `sp -0x18`, followed by the
  zero-valued exit (`ldrls` at 0x1f28c, `str` at 0x20210, `ldmdb` at 0x20218). Selector-specific
  values and NVRAM field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1f27c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1f280 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x1f28c | load | 4 | `(unknown)` | `-` |
| `str` at 0x20210 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x20214 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x20218 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20218 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20218 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1f28c | `r0-1 <= 0xaf (unsigned, 32-bit)` | `r0-1 > 0xaf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20218 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1f28c).

## `nvramLogReset`

- `nvramLogReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20230 to 0x202d0.
  Established. Two four-byte stores assign `[[0x145b0c]+1924]` at `[0x145b0c] +0x78c` and `+0x790`
  (`str` at 0x20250, `str` at 0x20268). The unnamed `0x21dc4` call has `[0x145b0c]+1920`, `0x24`,
  `[0x145b0c]`, `[0x145b0c]+1920` (`bl` at 0x2028c). A later byte store retains
  `(ret(0x21dc4) & 0xff)` at `[0x145b0c] +0x7a3` (`strb` at 0x20290). Log-pointer and checksum
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2023c | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x20240 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20244 | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x20248 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x2024c | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x20250 | store | 4 | `[0x145b0c] +0x78c` | `[[0x145b0c]+1924]` |
| `ldr` at 0x20254 | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x20258 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x2025c | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x20260 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20264 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x20268 | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1924]` |
| `ldr` at 0x2026c | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x20270 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20274 | load | 4 | `abs 0x202cc` | `-` |
| `ldr` at 0x20278 | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x2027c | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x20290 | store | 1 | `[0x145b0c] +0x7a3` | `(ret(0x21dc4) & 0xff)` |
| `ldr` at 0x20294 | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x20298 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x202a4 | load | 4 | `abs 0x202c8` | `-` |
| `ldr` at 0x202a8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldmdb` at 0x202c4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x202c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x202c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x202c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x202c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2028c | `(unknown callee) 0x21dc4` | `[0x145b0c]+1920` | `0x24` | `[0x145b0c]` | `[0x145b0c]+1920` |
| `bl` at 0x202bc | `(unknown callee) 0x1b394` | `[0x145b0c]+1956` | `[0x145b0c]+1920` | `0x24` | `[0x145b0c]+1920` |
| `bl` at 0x202c0 | `(unknown callee) 0x1bbdc` | `ret(0x1b394)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x202c4 | `r0` | `ret(0x1bbdc)` |

## `nvramLogInsert`

- `nvramLogInsert`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x202d0 to 0x2079c.
  Established. The unsigned-lower-or-same branch takes `[[0x145b0c]+1936] <= [[0x145b0c]+1932]` to
  `0x20408` (`bls` at 0x203c0). Included byte assignments at `[[0x145b0c]+1936] +0x0` retain
  `([[sp-40]]:1 & 0xff)`, `([[sp-28]]:1 & 0xff)` and `0xa` in different paths (`strb` at 0x2056c,
  `strb` at 0x205fc, `strb` at 0x20670). The unnamed `0x22e50` call carries `0x145b1c`, unknown,
  unknown, `0x0` (`bl` at 0x20718). Log-record and synchronization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x202e0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x202f0 | store | 4 | `sp -0x24` | `ret(0x1b274)` |
| `str` at 0x202f8 | store | 4 | `sp -0x28` | `sp-36` |
| `str` at 0x20300 | store | 4 | `sp -0x2c` | `0x5` |
| `ldr` at 0x20304 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x20310 | store | 4 | `sp -0x30` | `ret(0x1b2bc)` |
| `strb` at 0x20318 | store | 1 | `sp -0x31` | `0x0` |
| `ldr` at 0x2031c | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20320 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20330 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20334 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20338 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2033c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20340 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x20344 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x2034c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x20354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20358 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x20364 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x20374 | store | 1 | `sp -0x31` | `0x1` |
| `ldr` at 0x20378 | load | 4 | `abs 0x20784` | `-` |
| `ldr` at 0x20390 | load | 4 | `abs 0x20788` | `-` |
| `ldr` at 0x20394 | load | 4 | `abs 0x145ff8` | `-` |
| `ldr` at 0x203a4 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x203a8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x203ac | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x203b0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x203b4 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x203b8 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x203c4 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x203c8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x203cc | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x203d0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x203d4 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x203d8 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `ldr` at 0x203e0 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x203e4 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x203e8 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x203ec | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x203f0 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x203f4 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `str` at 0x20400 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x20408 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2040c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20410 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20414 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20418 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x2041c | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x20428 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2042c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20430 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20434 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20438 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x2043c | load | 4 | `[0x145b0c] +0x790` | `-` |
| `str` at 0x20444 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x2044c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x20450 | load | 4 | `sp -0x2c` | `-` |
| `ldrsb` at 0x20458 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x20460 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x20470 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20474 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20480 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldrb` at 0x20484 | load | 1 | `[[0x145b0c]+1932] +0x0` | `-` |
| `str` at 0x2048c | store | 4 | `[0x145b0c] +0x78c` | `[[0x145b0c]+1932]+1` |
| `ldr` at 0x204a0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x204a8 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x204ac | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x204b0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x204b4 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x204b8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x204bc | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x204c0 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x204cc | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x204d0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x204d4 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x204d8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x204dc | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x204e0 | store | 4 | `[0x145b0c] +0x78c` | `[[0x145b0c]+1924]` |
| `ldr` at 0x204e8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x204f0 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x204fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x20504 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0x20514 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20520 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20524 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x2052c | load | 4 | `[0x145b0c] +0x790` | `-` |
| `strb` at 0x20538 | store | 1 | `[[0x145b0c]+1936] +0x0` | `?` |
| `str` at 0x20540 | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1936]+1` |
| `ldr` at 0x20548 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2054c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20554 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x2055c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20568 | load | 1 | `[sp-40] +0x0` | `-` |
| `strb` at 0x2056c | store | 1 | `[[0x145b0c]+1936] +0x0` | `([[sp-40]]:1 & 0xff)` |
| `str` at 0x20574 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x2057c | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1936]+1` |
| `ldr` at 0x20580 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20584 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20588 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2058c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20590 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x20594 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x205a0 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x205a4 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x205a8 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x205ac | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x205b0 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x205b4 | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1924]` |
| `ldr` at 0x205c0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x205c8 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0x205d8 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x205dc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x205e4 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x205ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x205f8 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x205fc | store | 1 | `[[0x145b0c]+1936] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x20604 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x2060c | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1936]+1` |
| `ldr` at 0x20610 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20614 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20618 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2061c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20620 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x20624 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x20630 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20634 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20638 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2063c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20640 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x20644 | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1924]` |
| `ldrsb` at 0x2064c | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x20658 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x2065c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20664 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `strb` at 0x20670 | store | 1 | `[[0x145b0c]+1936] +0x0` | `0xa` |
| `str` at 0x20678 | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1936]+1` |
| `ldr` at 0x2067c | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20680 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20684 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x20688 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x2068c | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x20690 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `ldr` at 0x2069c | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x206a0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x206a4 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x206a8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x206ac | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x206b0 | store | 4 | `[0x145b0c] +0x790` | `[[0x145b0c]+1924]` |
| `ldr` at 0x206b4 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x206b8 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x206bc | load | 4 | `abs 0x2078c` | `-` |
| `ldr` at 0x206c0 | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x206c4 | load | 4 | `abs 0x145b0c` | `-` |
| `strb` at 0x206d8 | store | 1 | `[0x145b0c] +0x7a3` | `(ret(0x21dc4) & 0xff)` |
| `ldr` at 0x206dc | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x206e0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x206ec | load | 4 | `abs 0x20780` | `-` |
| `ldr` at 0x206f0 | load | 4 | `abs 0x145b0c` | `-` |
| `str` at 0x20710 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x20714 | load | 4 | `abs 0x20784` | `-` |
| `ldr` at 0x20728 | load | 4 | `abs 0x20788` | `-` |
| `ldr` at 0x2072c | load | 4 | `abs 0x145ff8` | `-` |
| `str` at 0x2073c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x20740 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x20744 | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x20750 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2075c | load | 4 | `abs 0x20790` | `-` |
| `str` at 0x20760 | store | 4 | `sp -0x44` | `0xfc9c8` |
| `str` at 0x20764 | store | 4 | `sp -0x40` | `ret(0x21704)` |
| `ldr` at 0x2076c | load | 4 | `abs 0x20794` | `-` |
| `ldr` at 0x20770 | load | 4 | `abs 0x20798` | `-` |
| `ldmdb` at 0x2077c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2077c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2077c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2077c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2077c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x202e8 | `(unknown callee) 0x1b274` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x20308 | `(unknown callee) 0x1b2bc` | `[sp-28]` | `?` | `?` | `0x5` |
| `bl` at 0x20380 | `(unknown callee) 0x22ac8` | `0x145b1c` | `?` | `[sp-28]` | `?` |
| `bl` at 0x206d4 | `(unknown callee) 0x21dc4` | `[0x145b0c]+1920` | `0x24` | `?` | `[0x145b0c]+1920` |
| `bl` at 0x20704 | `(unknown callee) 0x1b394` | `[0x145b0c]+1956` | `[0x145b0c]+1920` | `0x24` | `[0x145b0c]+1920` |
| `bl` at 0x20708 | `(unknown callee) 0x1bbdc` | `ret(0x1b394)` | `?` | `?` | `?` |
| `bl` at 0x20718 | `(unknown callee) 0x22e50` | `0x145b1c` | `?` | `?` | `0x0` |
| `bl` at 0x20754 | `(unknown callee) 0x21704` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x20778 | `(unknown callee) 0x21eb8` | `0x9` | `0xfc6d0` | `0x715` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x20328 | `[0x145b0c] != 0x0` | `[0x145b0c] == 0x0` |
| `beq` at 0x2036c | `? == 0xa` | `? != 0xa` |
| `beq` at 0x2038c | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0x2039c | `[0x145ff8] == 0x1` | `[0x145ff8] != 0x1` |
| `bls` at 0x203c0 | `[[0x145b0c]+1936] <= [[0x145b0c]+1932] (unsigned, 32-bit)` | `[[0x145b0c]+1936] > [[0x145b0c]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x20424 | `[[0x145b0c]+1936] >= [[0x145b0c]+1932] (unsigned, 32-bit)` | `[[0x145b0c]+1936] < [[0x145b0c]+1932] (unsigned, 32-bit)` |
| `bge` at 0x20468 | `? >= [sp-56] (signed, 32-bit)` | `? < [sp-56] (signed, 32-bit)` |
| `bne` at 0x20498 | `? != 0xa` | `? == 0xa` |
| `blo` at 0x204c8 | `[[0x145b0c]+1932] < [[0x145b0c]+1928] (unsigned, 32-bit)` | `[[0x145b0c]+1932] >= [[0x145b0c]+1928] (unsigned, 32-bit)` |
| `bne` at 0x2050c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x2051c | `[sp-44] != 0x4` | `[sp-44] == 0x4` |
| `blo` at 0x2059c | `[[0x145b0c]+1936] < [[0x145b0c]+1928] (unsigned, 32-bit)` | `[[0x145b0c]+1936] >= [[0x145b0c]+1928] (unsigned, 32-bit)` |
| `bne` at 0x205d0 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x2062c | `[[0x145b0c]+1936] < [[0x145b0c]+1928] (unsigned, 32-bit)` | `[[0x145b0c]+1936] >= [[0x145b0c]+1928] (unsigned, 32-bit)` |
| `beq` at 0x20654 | `? == 0x0` | `? != 0x0` |
| `blo` at 0x20698 | `[[0x145b0c]+1936] < [[0x145b0c]+1928] (unsigned, 32-bit)` | `[[0x145b0c]+1936] >= [[0x145b0c]+1928] (unsigned, 32-bit)` |
| `beq` at 0x20724 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0x20734 | `[0x145ff8] == 0x1` | `[0x145ff8] != 0x1` |
| `beq` at 0x2074c | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2077c | `r0` | `?` |

## `nvramLogFinsert`

- `nvramLogFinsert`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2079c to 0x207e0.
  Established. A four-byte stack store retains `sp-12` at `sp -0x24` (`str` at 0x207b4). The first
  unnamed call targets `0x1b010` with `sp-156`, `r0`, `sp-12`, `sp-156` (`bl` at 0x207c8). The
  following unnamed `0x202d0` call carries `sp-156`, unknown, unknown, `sp-156` (`bl` at 0x207d4).
  The exit value is unknown (`ldmdb` at 0x207dc). Formatted-log behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x207b4 | store | 4 | `sp -0x24` | `sp-12` |
| `ldr` at 0x207c0 | load | 4 | `sp -0x10` | `-` |
| `ldr` at 0x207c4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x207dc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x207dc | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x207dc | load | 4 | `sp -0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x207c8 | `(unknown callee) 0x1b010` | `sp-156` | `r0` | `sp-12` | `sp-156` |
| `bl` at 0x207d4 | `(unknown callee) 0x202d0` | `sp-156` | `?` | `?` | `sp-156` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x207dc | `r0` | `?` |

## `nvramLogFetch`

- `nvramLogFetch`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x207e0 to 0x20bc8.
  Established. The first unsigned branch takes `[[0x145b0c]+1936] <= [[0x145b0c]+1932]` to `0x2084c`
  (`bls` at 0x20814). One unnamed `0x1b1b4` call uses `0x1`, `r1`, `[[0x145b0c]+1936]`,
  `[[0x145b0c]+1932]` (`bl` at 0x208b4); its nonzero result takes the branch to `0x208d8`, followed
  by a byte zero store at `ret(0x1b1b4) +0x0` (`bne` at 0x208c8, `strb` at 0x208e0). A later unnamed
  `0x1b1cc` call carries `([sp-20] + [sp-68])`, `0x13`, `0xfc9f0`, `ret(0x1b070)` (`bl` at 0x20ad8).
  Log decoding and ownership of returned storage are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x207f4 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x207f8 | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x207fc | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20800 | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x20804 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20808 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x2080c | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x20818 | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x2081c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20820 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `str` at 0x20824 | store | 4 | `sp -0x18` | `[[0x145b0c]+1932]` |
| `ldr` at 0x20828 | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x2082c | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20830 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `str` at 0x20834 | store | 4 | `sp -0x1c` | `[[0x145b0c]+1936]` |
| `str` at 0x2083c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x20844 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x2084c | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x20850 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20854 | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x20858 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x2085c | load | 4 | `[0x145b0c] +0x790` | `-` |
| `ldr` at 0x20860 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `ldr` at 0x2086c | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x20870 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20874 | load | 4 | `[0x145b0c] +0x78c` | `-` |
| `str` at 0x20878 | store | 4 | `sp -0x18` | `[[0x145b0c]+1932]` |
| `ldr` at 0x2087c | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x20880 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20884 | load | 4 | `[0x145b0c] +0x788` | `-` |
| `str` at 0x20888 | store | 4 | `sp -0x1c` | `[[0x145b0c]+1928]` |
| `ldr` at 0x2088c | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x20890 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x20894 | load | 4 | `[0x145b0c] +0x784` | `-` |
| `str` at 0x20898 | store | 4 | `sp -0x20` | `[[0x145b0c]+1924]` |
| `ldr` at 0x2089c | load | 4 | `abs 0x20bc0` | `-` |
| `ldr` at 0x208a0 | load | 4 | `abs 0x145b0c` | `-` |
| `ldr` at 0x208a4 | load | 4 | `[0x145b0c] +0x790` | `-` |
| `str` at 0x208a8 | store | 4 | `sp -0x24` | `[[0x145b0c]+1936]` |
| `str` at 0x208bc | store | 4 | `sp -0x14` | `ret(0x1b1b4)` |
| `ldr` at 0x208c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x208d0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x208d8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x208e0 | store | 1 | `ret(0x1b1b4) +0x0` | `0x0` |
| `ldr` at 0x208e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x208e8 | store | 4 | `sp -0x48` | `[sp-20]` |
| `str` at 0x208f4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x208f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x208fc | store | 4 | `sp -0x2c` | `[[0x145b0c]+1932]` |
| `ldr` at 0x20900 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x20904 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x20908 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20918 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x2091c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20920 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x20924 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x20934 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x20940 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x20948 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x20950 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20954 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x20960 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x20968 | store | 4 | `sp -0x44` | `?` |
| `str` at 0x20970 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x20978 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x20980 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x20984 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x2098c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20994 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20998 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x209a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x209ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x209b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x209b4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x209b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x209bc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x209c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x209c4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x209cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x209d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x209e0 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x209ec | store | 4 | `sp -0x4c` | `0x64` |
| `ldr` at 0x209f0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x209f4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x209f8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x20a04 | store | 4 | `sp -0x14` | `ret(0x1b1b4)` |
| `ldr` at 0x20a08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20a18 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x20a24 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x20a2c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x20a30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x20a34 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x20a38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x20a3c | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x20a40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20a50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x20a54 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20a58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x20a5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x20a6c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x20a7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x20a80 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20a88 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x20a8c | store | 1 | `[sp-60] +0x0` | `?` |
| `str` at 0x20a94 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x20a98 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x20aa0 | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0x20abc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20ac0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20ad0 | load | 4 | `abs 0x20bc4` | `-` |
| `ldr` at 0x20adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20ae0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20af8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x20b00 | store | 4 | `sp -0x44` | `([sp-68] + ret(0x1b2bc))` |
| `ldr` at 0x20b08 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20b0c | load | 1 | `(unknown)` | `-` |
| `str` at 0x20b1c | store | 4 | `sp -0x3c` | `sp-56` |
| `str` at 0x20b24 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x20b30 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20b40 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20b44 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x20b48 | store | 1 | `(unknown)` | `?` |
| `str` at 0x20b50 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x20b54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x20b5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20b64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20b68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x20b78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x20b7c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x20b80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x20b84 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x20b88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x20b8c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x20b90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x20b94 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x20b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20ba0 | load | 4 | `sp -0x44` | `-` |
| `strb` at 0x20bac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x20bb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20bb4 | store | 4 | `sp -0x48` | `[sp-20]` |
| `ldr` at 0x20bb8 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0x20bbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20bbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20bbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x208b4 | `(unknown callee) 0x1b1b4` | `0x1` | `r1` | `[[0x145b0c]+1936]` | `[[0x145b0c]+1932]` |
| `bl` at 0x209fc | `(unknown callee) 0x1b1b4` | `?` | `r1` | `?` | `?` |
| `bl` at 0x20ab4 | `(unknown callee) 0x1b070` | `sp-56` | `[sp-60]` | `[sp-60]+1` | `sp-56` |
| `bl` at 0x20ad8 | `(unknown callee) 0x1b1cc` | `([sp-20] + [sp-68])` | `0x13` | `0xfc9f0` | `ret(0x1b070)` |
| `bl` at 0x20aec | `(unknown callee) 0x1b2bc` | `([sp-20] + [sp-68])` | `?` | `[sp-20]` | `([sp-20] + [sp-68])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x20814 | `[[0x145b0c]+1936] <= [[0x145b0c]+1932] (unsigned, 32-bit)` | `[[0x145b0c]+1936] > [[0x145b0c]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x20868 | `[[0x145b0c]+1936] >= [[0x145b0c]+1932] (unsigned, 32-bit)` | `[[0x145b0c]+1936] < [[0x145b0c]+1932] (unsigned, 32-bit)` |
| `bne` at 0x208c8 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0x20910 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x2092c | `C == 0` | `not (C == 0)` |
| `beq` at 0x2093c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x2095c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x209a0 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x209d4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x20a10 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0x20a48 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x20a64 | `C == 0` | `not (C == 0)` |
| `beq` at 0x20a74 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x20aa8 | `[sp-52]-1 != 0x0` | `[sp-52]-1 == 0x0` |
| `bne` at 0x20b14 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x20b70 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20bbc | `r0` | `?` |

## `powerQuery`

- `powerQuery`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94eb8 to 0x952d0.
  Established. The unnamed `0x3cc10` call carries `0x0`, `sp-36`, `r2`, `sp-36` (`bl` at 0x94ee4);
  its nonzero result takes the branch to `0x94f20` (`bne` at 0x94ef4). Zero falls through
  (`bne` at 0x94ef4). Both included four-byte stores through `[sp-24] +0x0` retain `ret(0x1b1b4)` in separate paths (`str` at
  0x94f08, `str` at 0x94f48). A later unnamed `0x3e3c4` call has `0x0`, `[sp-44]`, `sp-57`, `sp-57`,
  and its zero result takes the branch to `0x95208` (`bl` at 0x951d8, `beq` at 0x951ec). Power-query
  semantics and result units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94ec8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x94ecc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x94ed4 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x94ee8 | store | 4 | `sp -0x28` | `ret(0x3cc10)` |
| `ldr` at 0x94eec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x94ef8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x94f08 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x94f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94f10 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x94f14 | load | 4 | `abs 0x9528c` | `-` |
| `ldr` at 0x94f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94f24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x94f48 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x94f4c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x94f50 | store | 4 | `sp -0x4c` | `[[sp-24]]` |
| `ldr` at 0x94f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x94f60 | load | 4 | `abs 0x95290` | `-` |
| `str` at 0x94f64 | store | 4 | `sp -0x50` | `0x11d100` |
| `ldr` at 0x94f6c | load | 4 | `abs 0x95294` | `-` |
| `str` at 0x94f70 | store | 4 | `sp -0x50` | `0x11629c` |
| `ldr` at 0x94f74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x94f80 | load | 4 | `abs 0x95298` | `-` |
| `str` at 0x94f84 | store | 4 | `sp -0x54` | `0x11d108` |
| `ldr` at 0x94f8c | load | 4 | `abs 0x95294` | `-` |
| `str` at 0x94f90 | store | 4 | `sp -0x54` | `0x11629c` |
| `ldr` at 0x94f94 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94f98 | load | 4 | `abs 0x9529c` | `-` |
| `ldr` at 0x94f9c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x94fa0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x94fa8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94fb8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x94fc0 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b2bc))` |
| `str` at 0x94fc8 | store | 4 | `sp -0x2c` | `0x0` |
| `strb` at 0x94fd0 | store | 1 | `sp -0x2d` | `?` |
| `ldr` at 0x94fd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x94fd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x94fec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x94ff8 | store | 4 | `sp -0x34` | `ret(0x403f4)` |
| `ldr` at 0x94ffc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x95008 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9501c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x95024 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x95034 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0x9503c | store | 1 | `sp -0x2d` | `([sp-45]:1+1 & 0xff)` |
| `ldr` at 0x95044 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9505c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x95068 | store | 4 | `sp -0x40` | `ret(0x3df3c)` |
| `ldr` at 0x9506c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x95070 | load | 4 | `abs 0x952a0` | `-` |
| `ldr` at 0x9507c | load | 4 | `abs 0x952a4` | `-` |
| `str` at 0x95080 | store | 4 | `sp -0x44` | `0x11d174` |
| `ldr` at 0x95088 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x95094 | load | 4 | `abs 0x952a8` | `-` |
| `str` at 0x95098 | store | 4 | `sp -0x58` | `0x11319c` |
| `ldr` at 0x950a0 | load | 4 | `abs 0x952ac` | `-` |
| `str` at 0x950a4 | store | 4 | `sp -0x58` | `0x1131b8` |
| `ldr` at 0x950a8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x950ac | store | 4 | `sp -0x44` | `[sp-88]` |
| `ldr` at 0x950b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x950c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x950cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x950d8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x950dc | load | 4 | `abs 0x952b0` | `-` |
| `ldr` at 0x950e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x950e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x950f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x95108 | store | 4 | `sp -0x20` | `ret(0x400fc)` |
| `ldr` at 0x9510c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x95118 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9511c | store | 4 | `sp -0x64` | `ret(0x400fc)` |
| `ldr` at 0x95120 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x95124 | load | 4 | `abs 0x952b4` | `-` |
| `ldr` at 0x95128 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9512c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x95138 | load | 4 | `sp -0x48` | `-` |
| `stm` at 0x95148 | store | 4 | `sp -0x64` | `ret(0xfb644)` |
| `stm` at 0x95148 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x9514c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x95150 | load | 4 | `abs 0x952b8` | `-` |
| `ldr` at 0x95154 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x95158 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x95160 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x95170 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x95178 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b2bc))` |
| `ldr` at 0x9517c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x95190 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x95198 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x951a4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x951a8 | load | 4 | `abs 0x952bc` | `-` |
| `ldr` at 0x951b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x951c4 | store | 4 | `sp -0x38` | `ret(0x404e4)` |
| `ldr` at 0x951d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x951e0 | store | 4 | `sp -0x20` | `ret(0x3e3c4)` |
| `ldr` at 0x951e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x951f0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x951f4 | load | 4 | `abs 0x952c0` | `-` |
| `ldr` at 0x951f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x951fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x95208 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x95214 | load | 4 | `abs 0x952c4` | `-` |
| `str` at 0x95218 | store | 4 | `sp -0x5c` | `0x11d1e8` |
| `ldr` at 0x95220 | load | 4 | `abs 0x952c8` | `-` |
| `str` at 0x95224 | store | 4 | `sp -0x5c` | `0x11d1f8` |
| `ldrb` at 0x95228 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x9522c | load | 1 | `sp -0x2d` | `-` |
| `str` at 0x95230 | store | 4 | `sp -0x64` | `[sp-45]:1` |
| `ldr` at 0x95234 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x95238 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9523c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x9524c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x95250 | load | 4 | `abs 0x952cc` | `-` |
| `ldr` at 0x95254 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9525c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9526c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x95274 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b2bc))` |
| `ldr` at 0x95278 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x95280 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x95288 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x95288 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x95288 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x95288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x94ee4 | `(unknown callee) 0x3cc10` | `0x0` | `sp-36` | `r2` | `sp-36` |
| `bl` at 0x94f00 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `ret(0x3cc10)` |
| `bl` at 0x94f18 | `(unknown callee) 0x1b598` | `[[sp-24]]` | `0x11d0d8` | `?` | `[sp-24]` |
| `bl` at 0x94f40 | `(unknown callee) 0x1b1b4` | `?` | `?` | `ret(0x3cc10)` | `?` |
| `bl` at 0x94fa4 | `(unknown callee) 0x1b52c` | `[[sp-24]]` | `0x11d110` | `[sp-80]` | `[sp-84]` |
| `bl` at 0x94fac | `(unknown callee) 0x1b2bc` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x94ff0 | `(unknown callee) 0x403f4` | `0x0` | `[sp-44]` | `[sp-44]` | `[sp-40]` |
| `bl` at 0x95048 | `(unknown callee) 0x3cc70` | `0x0` | `[sp-44]` | `?` | `[sp-45]:1+1` |
| `bl` at 0x95060 | `(unknown callee) 0x3df3c` | `0x0` | `[sp-44]` | `?` | `ret(0x3cc70)` |
| `bl` at 0x950e8 | `(unknown callee) 0x1b52c` | `[sp-76]` | `0x11d178` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x95100 | `(unknown callee) 0x400fc` | `0x0` | `[sp-44]` | `0x0` | `sp-72` |
| `bl` at 0x95130 | `(unknown callee) 0x1b52c` | `[sp-76]` | `0x11d194` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x9513c | `(unknown callee) 0xfb644` | `[sp-72]` | `?` | `?` | `ret(0x400fc)` |
| `bl` at 0x9515c | `(unknown callee) 0x1b52c` | `[sp-76]` | `0x11d1b0` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x95164 | `(unknown callee) 0x1b2bc` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x951ac | `(unknown callee) 0x1b52c` | `[sp-76]` | `0x11d1c8` | `?` | `?` |
| `bl` at 0x951bc | `(unknown callee) 0x404e4` | `0x0` | `[sp-44]` | `?` | `?` |
| `bl` at 0x951d8 | `(unknown callee) 0x3e3c4` | `0x0` | `[sp-44]` | `sp-57` | `sp-57` |
| `bl` at 0x95200 | `(unknown callee) 0x1b52c` | `[sp-76]` | `0x11d1d4` | `[sp-56]` | `ret(0x3e3c4)` |
| `bl` at 0x95244 | `(unknown callee) 0x1b52c` | `[sp-76]` | `[sp-92]` | `[sp-56]` | `[sp-57]:1` |
| `bl` at 0x95258 | `(unknown callee) 0x1b52c` | `[sp-76]` | `0x11d204` | `[sp-52]` | `ret(0x3cc70)` |
| `bl` at 0x95260 | `(unknown callee) 0x1b2bc` | `[sp-76]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x94ef4 | `ret(0x3cc10) != 0x0` | `ret(0x3cc10) == 0x0` |
| `beq` at 0x94f5c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x94f7c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blo` at 0x94fe0 | `[sp-44] < [sp-40] (unsigned, 32-bit)` | `[sp-44] >= [sp-40] (unsigned, 32-bit)` |
| `beq` at 0x95004 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x9502c | `? != 0xff` | `? == 0xff` |
| `beq` at 0x95054 | `ret(0x3cc70) == 0x0` | `ret(0x3cc70) != 0x0` |
| `bne` at 0x95078 | `ret(0x3df3c) != 0xfffff9b8` | `ret(0x3df3c) == 0xfffff9b8` |
| `bne` at 0x95090 | `ret(0x3df3c) != 0x1` | `ret(0x3df3c) == 0x1` |
| `bne` at 0x950d4 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x95114 | `ret(0x400fc) == 0x0` | `ret(0x400fc) != 0x0` |
| `bne` at 0x951a0 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x951ec | `ret(0x3e3c4) == 0x0` | `ret(0x3e3c4) != 0x0` |
| `beq` at 0x95210 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x95288 | `r0` | `?` |
