# ARM board configuration and environment callbacks in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/board-configuration-events-facts.txt) retain all tool sections.

## `Unknown_i2cConfig`

- `Unknown_i2cConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24500 to 0x24588.
  Established. The first call targets `0xd1584` with `0x0`, `0xfea54`, `0x9`, entry r3 (`bl` at
  0x2451c). The loop calls `0xd18b8` with `0x0` and three unknown candidates (`bl` at 0x2454c). A
  byte store initializes `sp -0x11` to `0x0`, and its later update writes `([sp-17]:1+1 & 0xff)`
  (`strb` at 0x2452c, `strb` at 0x24558). The r0 return is `ret(0x20240)` (`ldmdb` at 0x24574). Peripheral configuration
  effects remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24514 | load | 4 | `abs 0x24578` | `-` |
| `ldr` at 0x24520 | load | 4 | `abs 0x2457c` | `-` |
| `strb` at 0x2452c | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0x24530 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x24540 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x24550 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x24558 | store | 1 | `sp -0x11` | `([sp-17]:1+1 & 0xff)` |
| `ldr` at 0x24560 | load | 4 | `abs 0x24580` | `-` |
| `ldr` at 0x2456c | load | 4 | `abs 0x24584` | `-` |
| `ldmdb` at 0x24574 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24574 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24574 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2451c | `(unknown callee) 0xd1584` | `0x0` | `0xfea54` | `0x9` | `r3` |
| `bl` at 0x24524 | `(unknown callee) 0x20240` | `0xfeac0` | `?` | `?` | `?` |
| `bl` at 0x2454c | `(unknown callee) 0xd18b8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x24564 | `(unknown callee) 0x20240` | `0xfeac4` | `?` | `?` | `?` |
| `bl` at 0x24568 | `(unknown callee) 0x40f20` | `ret(0x20240)` | `?` | `?` | `?` |
| `bl` at 0x24570 | `(unknown callee) 0x20240` | `0xfeac8` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x24538 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24574 | `r0` | `ret(0x20240)` |

## `Vortix_i2cConfig`

- `Vortix_i2cConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x258f8 to 0x25c08.
  Established. The call to `0xd5b60` has `0x0`, `0xfef34`, `0x36`, unknown (`bl` at 0x2593c). A
  separate word store places `0x249dc` at `sp -0x18` before the call to `0x3c1c8` with `0x0`,
  `0xffbcc`, `0x29`, `0x249ac` (`str` at 0x25970, `bl` at 0x25984). The first `0x5dec8` result test
  takes the bypass for `ret(0x5dec8) == 0x0` and falls through otherwise (`beq` at 0x25ac8). Device
  setup, argument count and the joined r0 value remain unknown (`ldmdb` at 0x25ba8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x25908 | load | 4 | `abs 0x25bac` | `-` |
| `ldr` at 0x25914 | load | 4 | `abs 0x25bb0` | `-` |
| `ldr` at 0x25924 | load | 4 | `abs 0x25bb4` | `-` |
| `ldr` at 0x25934 | load | 4 | `abs 0x25bb8` | `-` |
| `ldr` at 0x25944 | load | 4 | `abs 0x25bbc` | `-` |
| `ldr` at 0x25954 | load | 4 | `abs 0x25bc0` | `-` |
| `ldr` at 0x2596c | load | 4 | `abs 0x25bc4` | `-` |
| `str` at 0x25970 | store | 4 | `sp -0x18` | `0x249dc` |
| `ldr` at 0x25978 | load | 4 | `abs 0x25bc8` | `-` |
| `ldr` at 0x25980 | load | 4 | `abs 0x25bcc` | `-` |
| `ldr` at 0x2598c | load | 4 | `abs 0x25bd0` | `-` |
| `ldr` at 0x2599c | load | 4 | `abs 0x25bd4` | `-` |
| `ldr` at 0x259ac | load | 4 | `abs 0x25bd8` | `-` |
| `ldr` at 0x259bc | load | 4 | `abs 0x25bdc` | `-` |
| `ldr` at 0x259c8 | load | 4 | `abs 0x25be0` | `-` |
| `ldr` at 0x259d4 | load | 4 | `abs 0x25be4` | `-` |
| `str` at 0x259e4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x259ec | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x259f0 | load | 4 | `abs 0x25be8` | `-` |
| `ldr` at 0x259f8 | load | 4 | `abs 0x25bec` | `-` |
| `ldr` at 0x25a08 | load | 4 | `abs 0x25bf0` | `-` |
| `ldr` at 0x25a14 | load | 4 | `abs 0x25bf4` | `-` |
| `ldr` at 0x25a34 | load | 4 | `abs 0x25bf8` | `-` |
| `ldr` at 0x25a40 | load | 4 | `abs 0x25bfc` | `-` |
| `ldr` at 0x25a54 | load | 4 | `abs 0x25c00` | `-` |
| `ldr` at 0x25a68 | load | 4 | `abs 0x25c04` | `-` |
| `ldr` at 0x25a7c | load | 4 | `abs 0x25bfc` | `-` |
| `ldr` at 0x25a90 | load | 4 | `abs 0x25c00` | `-` |
| `ldr` at 0x25aa4 | load | 4 | `abs 0x25c04` | `-` |
| `ldr` at 0x25acc | load | 4 | `abs 0x25bfc` | `-` |
| `ldr` at 0x25ae0 | load | 4 | `abs 0x25c04` | `-` |
| `ldr` at 0x25b08 | load | 4 | `abs 0x25bfc` | `-` |
| `ldr` at 0x25b1c | load | 4 | `abs 0x25c04` | `-` |
| `ldr` at 0x25b44 | load | 4 | `abs 0x25bfc` | `-` |
| `ldr` at 0x25b58 | load | 4 | `abs 0x25c04` | `-` |
| `ldr` at 0x25b80 | load | 4 | `abs 0x25bfc` | `-` |
| `ldr` at 0x25b94 | load | 4 | `abs 0x25c04` | `-` |
| `ldmdb` at 0x25ba8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25ba8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25ba8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2590c | `(unknown callee) 0xc77ac` | `0x256e4` | `r1` | `r2` | `r3` |
| `bl` at 0x2591c | `(unknown callee) 0xd4590` | `0x0` | `0xfebe4` | `0x5` | `?` |
| `bl` at 0x2592c | `(unknown callee) 0xd8344` | `0x0` | `0xfec1c` | `0x6` | `?` |
| `bl` at 0x2593c | `(unknown callee) 0xd5b60` | `0x0` | `0xfef34` | `0x36` | `?` |
| `bl` at 0x2594c | `(unknown callee) 0xd1584` | `0x0` | `0xff5bc` | `0x48` | `?` |
| `bl` at 0x2595c | `(unknown callee) 0xcbfec` | `0x0` | `0xff97c` | `0x13` | `?` |
| `bl` at 0x25968 | `(unknown callee) 0xcc230` | `0x0` | `0x40` | `?` | `?` |
| `bl` at 0x25984 | `(unknown callee) 0x3c1c8` | `0x0` | `0xffbcc` | `0x29` | `0x249ac` |
| `bl` at 0x25994 | `(unknown callee) 0x5c2c4` | `0x0` | `0x100048` | `0xe` | `?` |
| `bl` at 0x259a4 | `(unknown callee) 0x52c74` | `0x0` | `0x100100` | `0x13` | `?` |
| `bl` at 0x259b4 | `(unknown callee) 0x454a0` | `0x0` | `0x1000b8` | `0x1` | `?` |
| `bl` at 0x259c4 | `(unknown callee) 0x45214` | `0x0` | `0x1000dc` | `0x3` | `?` |
| `bl` at 0x259d0 | `(unknown callee) 0xe9190` | `0x1423f0` | `0x6` | `?` | `?` |
| `bl` at 0x259dc | `(unknown callee) 0x27808` | `0x100248` | `0x2` | `?` | `?` |
| `bl` at 0x25a00 | `(unknown callee) 0x5eed8` | `0x1001f8` | `0x4` | `0x256c0` | `0x0` |
| `bl` at 0x25a10 | `(unknown callee) 0x41208` | `0x0` | `0x100378` | `0x1a` | `?` |
| `bl` at 0x25a1c | `(unknown callee) 0x44a5c` | `0x100198` | `0x4` | `?` | `?` |
| `bl` at 0x25a24 | `(unknown callee) 0x51298` | `0x9` | `?` | `?` | `?` |
| `bl` at 0x25a28 | `(unknown callee) 0x247e4` | `ret(0x51298)` | `?` | `?` | `?` |
| `bl` at 0x25a2c | `(unknown callee) 0x40f20` | `ret(0x247e4)` | `?` | `?` | `?` |
| `bl` at 0x25a30 | `(unknown callee) 0x248d4` | `ret(0x40f20)` | `?` | `?` | `?` |
| `bl` at 0x25a3c | `(unknown callee) 0xc84f8` | `0x142420` | `0x6` | `?` | `?` |
| `bl` at 0x25a50 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x25a64 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x25a78 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x25a8c | `(unknown callee) 0xc8698` | `0xd94a8` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x25aa0 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x1` | `0x9` | `0x9` |
| `bl` at 0x25ab4 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x1` | `0x2` | `0x2` |
| `bl` at 0x25abc | `(unknown callee) 0x5dec8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x25adc | `(unknown callee) 0xc8698` | `0xd94a8` | `0x2` | `0x2` | `0x2` |
| `bl` at 0x25af0 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x2` | `0x3` | `0x6` |
| `bl` at 0x25af8 | `(unknown callee) 0x5dec8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x25b18 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x3` | `0x3` | `0x3` |
| `bl` at 0x25b2c | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x3` | `0x7` | `0xa` |
| `bl` at 0x25b34 | `(unknown callee) 0x5dec8` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x25b54 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x4` | `0x4` | `0x4` |
| `bl` at 0x25b68 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x4` | `0xb` | `0xe` |
| `bl` at 0x25b70 | `(unknown callee) 0x5dec8` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x25b90 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x5` | `0x5` | `0x5` |
| `bl` at 0x25ba4 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x5` | `0xf` | `0x12` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25ac8 | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `beq` at 0x25b04 | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `beq` at 0x25b40 | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `beq` at 0x25b7c | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25ba8 | `r0` | `?` |

## `Gemini_i2cConfig`

- `Gemini_i2cConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x273a8 to 0x27604.
  Established. The call to `0xd1584` carries `0x0`, `0x100b5c`, `0x15`, unknown (`bl` at 0x27404).
  The later absolute-byte test takes the bypass for `[0x147eb5]:1 == 0x0` and falls through on
  nonzero (`beq` at 0x27548). After that join, a call to `0xc8698` uses `0xd94a8`, `0x2`, `0x0`,
  `0x0` (`bl` at 0x27584). The final r0 value is `ret(0xc8698)`, while configuration-record meanings
  and physical bus actions remain unproved (`ldmdb` at 0x2759c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x273b8 | load | 4 | `abs 0x275a0` | `-` |
| `ldr` at 0x273c4 | load | 4 | `abs 0x275a4` | `-` |
| `ldr` at 0x273dc | load | 4 | `abs 0x275a8` | `-` |
| `ldr` at 0x273ec | load | 4 | `abs 0x275ac` | `-` |
| `ldr` at 0x273fc | load | 4 | `abs 0x275b0` | `-` |
| `ldr` at 0x2740c | load | 4 | `abs 0x275b4` | `-` |
| `ldr` at 0x27418 | load | 4 | `abs 0x275b8` | `-` |
| `str` at 0x2741c | store | 4 | `sp -0x18` | `0x26308` |
| `ldr` at 0x27424 | load | 4 | `abs 0x275bc` | `-` |
| `ldr` at 0x2742c | load | 4 | `abs 0x275c0` | `-` |
| `ldr` at 0x27438 | load | 4 | `abs 0x275c4` | `-` |
| `ldr` at 0x27448 | load | 4 | `abs 0x275c8` | `-` |
| `ldr` at 0x27458 | load | 4 | `abs 0x275cc` | `-` |
| `ldr` at 0x27468 | load | 4 | `abs 0x275d0` | `-` |
| `ldr` at 0x27474 | load | 4 | `abs 0x275d4` | `-` |
| `str` at 0x27484 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x2748c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27490 | load | 4 | `abs 0x275d8` | `-` |
| `ldr` at 0x27498 | load | 4 | `abs 0x275dc` | `-` |
| `ldr` at 0x274a8 | load | 4 | `abs 0x275e0` | `-` |
| `ldr` at 0x274b4 | load | 4 | `abs 0x275e4` | `-` |
| `ldr` at 0x274d4 | load | 4 | `abs 0x275e8` | `-` |
| `ldr` at 0x274e8 | load | 4 | `abs 0x275ec` | `-` |
| `ldr` at 0x274f4 | load | 4 | `abs 0x275f0` | `-` |
| `ldr` at 0x27500 | load | 4 | `abs 0x275f4` | `-` |
| `ldr` at 0x27514 | load | 4 | `abs 0x275f8` | `-` |
| `ldr` at 0x27528 | load | 4 | `abs 0x275f4` | `-` |
| `ldr` at 0x2753c | load | 4 | `abs 0x275fc` | `-` |
| `ldrb` at 0x27540 | load | 1 | `abs 0x147eb5` | `-` |
| `ldr` at 0x2754c | load | 4 | `abs 0x27600` | `-` |
| `ldr` at 0x27560 | load | 4 | `abs 0x275f8` | `-` |
| `ldr` at 0x27574 | load | 4 | `abs 0x27600` | `-` |
| `ldr` at 0x27588 | load | 4 | `abs 0x275f8` | `-` |
| `ldmdb` at 0x2759c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2759c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2759c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x273bc | `(unknown callee) 0xc77ac` | `0x271cc` | `r1` | `r2` | `r3` |
| `bl` at 0x273cc | `(unknown callee) 0xd4590` | `0x0` | `0x100878` | `0x2` | `?` |
| `bl` at 0x273d4 | `(unknown callee) 0xc9278` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x273e4 | `(unknown callee) 0xd8344` | `0x0` | `0x100890` | `0x2` | `?` |
| `bl` at 0x273f4 | `(unknown callee) 0xd5b60` | `0x0` | `0x1009a4` | `0x12` | `?` |
| `bl` at 0x27404 | `(unknown callee) 0xd1584` | `0x0` | `0x100b5c` | `0x15` | `?` |
| `bl` at 0x27414 | `(unknown callee) 0xcbfec` | `0x0` | `0x100c94` | `0x7` | `?` |
| `bl` at 0x27430 | `(unknown callee) 0x3c1c8` | `0x0` | `0x100dac` | `0x14` | `0x262d4` |
| `bl` at 0x27440 | `(unknown callee) 0x52c74` | `0x0` | `0x1010f4` | `0x7` | `?` |
| `bl` at 0x27450 | `(unknown callee) 0x5c2c4` | `0x0` | `0x100fdc` | `0x5` | `?` |
| `bl` at 0x27460 | `(unknown callee) 0x454a0` | `0x0` | `0x101004` | `0x4` | `?` |
| `bl` at 0x27470 | `(unknown callee) 0x45214` | `0x0` | `0x101088` | `0x9` | `?` |
| `bl` at 0x2747c | `(unknown callee) 0xe9190` | `0x142494` | `0x2` | `?` | `?` |
| `bl` at 0x274a0 | `(unknown callee) 0x5eed8` | `0x101274` | `0x1` | `0x26f78` | `0x0` |
| `bl` at 0x274b0 | `(unknown callee) 0x41208` | `0x0` | `0x1011d8` | `0x6` | `?` |
| `bl` at 0x274bc | `(unknown callee) 0x44a5c` | `0x101268` | `0x1` | `?` | `?` |
| `bl` at 0x274c4 | `(unknown callee) 0x51298` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x274c8 | `(unknown callee) 0x25f1c` | `ret(0x51298)` | `?` | `?` | `?` |
| `bl` at 0x274cc | `(unknown callee) 0x40f20` | `ret(0x25f1c)` | `?` | `?` | `?` |
| `bl` at 0x274d0 | `(unknown callee) 0x26228` | `ret(0x40f20)` | `?` | `?` | `?` |
| `bl` at 0x274e0 | `(unknown callee) 0x532a0` | `0x10115c` | `0x2` | `0x1` | `?` |
| `bl` at 0x274e4 | `(unknown callee) 0x534f4` | `ret(0x532a0)` | `?` | `?` | `?` |
| `bl` at 0x274f0 | `(unknown callee) 0xdad0c` | `0x101190` | `0x1` | `?` | `?` |
| `bl` at 0x274fc | `(unknown callee) 0xc84f8` | `0x1424a4` | `0x4` | `?` | `?` |
| `bl` at 0x27510 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x0` | `0x11` | `0x11` |
| `bl` at 0x27524 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x27538 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x2755c | `(unknown callee) 0xc8698` | `0xd94a8` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x27570 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x27584 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x27598 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x27548 | `[0x147eb5]:1 == 0x0` | `[0x147eb5]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2759c | `r0` | `ret(0xc8698)` |

## `OpusTio_i2cConfig`

- `OpusTio_i2cConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bbb0 to 0x2be58.
  Established. The byte test takes the bypass for `[0x147ece]:1 == 0x0` and falls through otherwise
  (`beq` at 0x2bce8); its fall-through stores word `0x102ea0` at `sp -0x1c`, word `0x4` at `sp
  -0x18`, and word `0x43520000` at `sp -0x14` (`str` at 0x2bd04, `str` at 0x2bd0c, `str` at
  0x2bd14). The following call to `0x574f4` has `0x0`, `0x1`, `0x102e88`, `0x2` (`bl` at 0x2bd28).
  The r0 return is `ret(0xc8698)` (`ldmdb` at 0x2bde4). Physical devices and stack argument count
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2bbc0 | load | 4 | `abs 0x2bde8` | `-` |
| `ldr` at 0x2bbcc | load | 4 | `abs 0x2bdec` | `-` |
| `ldr` at 0x2bbe4 | load | 4 | `abs 0x2bdf0` | `-` |
| `ldr` at 0x2bbf4 | load | 4 | `abs 0x2bdf4` | `-` |
| `ldr` at 0x2bc04 | load | 4 | `abs 0x2bdf8` | `-` |
| `ldr` at 0x2bc14 | load | 4 | `abs 0x2bdfc` | `-` |
| `ldr` at 0x2bc20 | load | 4 | `abs 0x2be00` | `-` |
| `str` at 0x2bc24 | store | 4 | `sp -0x1c` | `0x2aaa4` |
| `ldr` at 0x2bc2c | load | 4 | `abs 0x2be04` | `-` |
| `ldr` at 0x2bc34 | load | 4 | `abs 0x2be08` | `-` |
| `ldr` at 0x2bc40 | load | 4 | `abs 0x2be0c` | `-` |
| `ldr` at 0x2bc50 | load | 4 | `abs 0x2be10` | `-` |
| `ldr` at 0x2bc60 | load | 4 | `abs 0x2be14` | `-` |
| `ldr` at 0x2bc70 | load | 4 | `abs 0x2be18` | `-` |
| `ldr` at 0x2bc7c | load | 4 | `abs 0x2be1c` | `-` |
| `str` at 0x2bc8c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2bc94 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2bc98 | load | 4 | `abs 0x2be20` | `-` |
| `ldr` at 0x2bca0 | load | 4 | `abs 0x2be24` | `-` |
| `ldr` at 0x2bcb0 | load | 4 | `abs 0x2be28` | `-` |
| `ldr` at 0x2bcbc | load | 4 | `abs 0x2be2c` | `-` |
| `ldr` at 0x2bcdc | load | 4 | `abs 0x2be30` | `-` |
| `ldrb` at 0x2bce0 | load | 1 | `abs 0x147ece` | `-` |
| `ldr` at 0x2bcec | load | 4 | `abs 0x2be34` | `-` |
| `ldr` at 0x2bd00 | load | 4 | `abs 0x2be38` | `-` |
| `str` at 0x2bd04 | store | 4 | `sp -0x1c` | `0x102ea0` |
| `str` at 0x2bd0c | store | 4 | `sp -0x18` | `0x4` |
| `ldr` at 0x2bd10 | load | 4 | `abs 0x2be3c` | `-` |
| `str` at 0x2bd14 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x2bd20 | load | 4 | `abs 0x2be40` | `-` |
| `ldr` at 0x2bd2c | load | 4 | `abs 0x2be44` | `-` |
| `ldr` at 0x2bd38 | load | 4 | `abs 0x2be48` | `-` |
| `ldr` at 0x2bd4c | load | 4 | `abs 0x2be4c` | `-` |
| `ldr` at 0x2bd60 | load | 4 | `abs 0x2be48` | `-` |
| `ldr` at 0x2bd74 | load | 4 | `abs 0x2be50` | `-` |
| `ldrb` at 0x2bd78 | load | 1 | `abs 0x147ecd` | `-` |
| `ldr` at 0x2bd84 | load | 4 | `abs 0x2be54` | `-` |
| `ldr` at 0x2bd98 | load | 4 | `abs 0x2be4c` | `-` |
| `ldr` at 0x2bdac | load | 4 | `abs 0x2be30` | `-` |
| `ldrb` at 0x2bdb0 | load | 1 | `abs 0x147ece` | `-` |
| `ldr` at 0x2bdbc | load | 4 | `abs 0x2be54` | `-` |
| `ldr` at 0x2bdd0 | load | 4 | `abs 0x2be4c` | `-` |
| `ldmdb` at 0x2bde4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bde4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bde4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2bbc4 | `(unknown callee) 0xc77ac` | `0x2b9d4` | `r1` | `r2` | `r3` |
| `bl` at 0x2bbd4 | `(unknown callee) 0xd4590` | `0x0` | `0x102570` | `0x2` | `?` |
| `bl` at 0x2bbdc | `(unknown callee) 0xc9278` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2bbec | `(unknown callee) 0xd8344` | `0x0` | `0x102588` | `0x2` | `?` |
| `bl` at 0x2bbfc | `(unknown callee) 0xd5b60` | `0x0` | `0x1026b4` | `0x14` | `?` |
| `bl` at 0x2bc0c | `(unknown callee) 0xd1584` | `0x0` | `0x102894` | `0x16` | `?` |
| `bl` at 0x2bc1c | `(unknown callee) 0xcbfec` | `0x0` | `0x1029d8` | `0x7` | `?` |
| `bl` at 0x2bc38 | `(unknown callee) 0x3c1c8` | `0x0` | `0x102aec` | `0x13` | `0x2aa70` |
| `bl` at 0x2bc48 | `(unknown callee) 0x52c74` | `0x0` | `0x102e18` | `0x7` | `?` |
| `bl` at 0x2bc58 | `(unknown callee) 0x5c2c4` | `0x0` | `0x102d00` | `0x5` | `?` |
| `bl` at 0x2bc68 | `(unknown callee) 0x454a0` | `0x0` | `0x102d28` | `0x4` | `?` |
| `bl` at 0x2bc78 | `(unknown callee) 0x45214` | `0x0` | `0x102dac` | `0x9` | `?` |
| `bl` at 0x2bc84 | `(unknown callee) 0xe9190` | `0x14256c` | `0x2` | `?` | `?` |
| `bl` at 0x2bca8 | `(unknown callee) 0x5eed8` | `0x102f50` | `0x1` | `0x2b780` | `0x0` |
| `bl` at 0x2bcb8 | `(unknown callee) 0x41208` | `0x0` | `0x102ee4` | `0x4` | `?` |
| `bl` at 0x2bcc4 | `(unknown callee) 0x44a5c` | `0x102f44` | `0x1` | `?` | `?` |
| `bl` at 0x2bccc | `(unknown callee) 0x51298` | `0x13` | `?` | `?` | `?` |
| `bl` at 0x2bcd0 | `(unknown callee) 0x2a698` | `ret(0x51298)` | `?` | `?` | `?` |
| `bl` at 0x2bcd4 | `(unknown callee) 0x40f20` | `ret(0x2a698)` | `?` | `?` | `?` |
| `bl` at 0x2bcd8 | `(unknown callee) 0x2a9c0` | `ret(0x40f20)` | `?` | `?` | `?` |
| `bl` at 0x2bcf8 | `(unknown callee) 0x532a0` | `0x102e74` | `0x1` | `0x1` | `[0x147ece]:1` |
| `bl` at 0x2bcfc | `(unknown callee) 0x534f4` | `ret(0x532a0)` | `?` | `?` | `?` |
| `bl` at 0x2bd28 | `(unknown callee) 0x574f4` | `0x0` | `0x1` | `0x102e88` | `0x2` |
| `bl` at 0x2bd34 | `(unknown callee) 0xc84f8` | `0x14257c` | `0x4` | `?` | `?` |
| `bl` at 0x2bd48 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x0` | `0x13` | `0x13` |
| `bl` at 0x2bd5c | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x2bd70 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x2bd94 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x2bda8 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x2bdcc | `(unknown callee) 0xc8698` | `0xd94a8` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x2bde0 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2bce8 | `[0x147ece]:1 == 0x0` | `[0x147ece]:1 != 0x0` |
| `beq` at 0x2bd80 | `[0x147ecd]:1 == 0x0` | `[0x147ecd]:1 != 0x0` |
| `beq` at 0x2bdb8 | `[0x147ece]:1 == 0x0` | `[0x147ece]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bde4 | `r0` | `ret(0xc8698)` |

## `Dorado_i2cConfig`

- `Dorado_i2cConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ebb0 to 0x2ee30.
  Established. The call to `0x3c1c8` uses `0x0`, `0x10410c`, `0xe`, `0x2d3bc` (`bl` at 0x2ec38). A
  later word store writes `0x43520000` at `sp -0x14` before the call to `0x574f4` with `0x0`, `0x1`,
  `0x1044d0`, `0x4` (`str` at 0x2ed10, `bl` at 0x2ed24). The byte test takes the bypass for
  `[0x147f15]:1 == 0x0` and falls through otherwise (`beq` at 0x2ed7c). The returned r0 value is
  `ret(0xc8698)`, and external configuration effects remain unknown (`ldmdb` at 0x2edbc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2ebc0 | load | 4 | `abs 0x2edc0` | `-` |
| `ldr` at 0x2ebcc | load | 4 | `abs 0x2edc4` | `-` |
| `ldr` at 0x2ebe4 | load | 4 | `abs 0x2edc8` | `-` |
| `ldr` at 0x2ebf4 | load | 4 | `abs 0x2edcc` | `-` |
| `ldr` at 0x2ec04 | load | 4 | `abs 0x2edd0` | `-` |
| `ldr` at 0x2ec14 | load | 4 | `abs 0x2edd4` | `-` |
| `ldr` at 0x2ec20 | load | 4 | `abs 0x2edd8` | `-` |
| `str` at 0x2ec24 | store | 4 | `sp -0x1c` | `0x2d3ec` |
| `ldr` at 0x2ec2c | load | 4 | `abs 0x2eddc` | `-` |
| `ldr` at 0x2ec34 | load | 4 | `abs 0x2ede0` | `-` |
| `ldr` at 0x2ec40 | load | 4 | `abs 0x2ede4` | `-` |
| `ldr` at 0x2ec50 | load | 4 | `abs 0x2ede8` | `-` |
| `ldr` at 0x2ec60 | load | 4 | `abs 0x2edec` | `-` |
| `ldr` at 0x2ec70 | load | 4 | `abs 0x2edf0` | `-` |
| `ldr` at 0x2ec7c | load | 4 | `abs 0x2edf4` | `-` |
| `ldr` at 0x2ec88 | load | 4 | `abs 0x2edf8` | `-` |
| `str` at 0x2ec98 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2eca0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2eca4 | load | 4 | `abs 0x2edfc` | `-` |
| `ldr` at 0x2ecac | load | 4 | `abs 0x2ee00` | `-` |
| `ldr` at 0x2ecbc | load | 4 | `abs 0x2ee04` | `-` |
| `ldr` at 0x2ecdc | load | 4 | `abs 0x2ee08` | `-` |
| `ldr` at 0x2ece8 | load | 4 | `abs 0x2ee0c` | `-` |
| `ldr` at 0x2ecfc | load | 4 | `abs 0x2ee10` | `-` |
| `str` at 0x2ed00 | store | 4 | `sp -0x1c` | `0x104500` |
| `str` at 0x2ed08 | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x2ed0c | load | 4 | `abs 0x2ee14` | `-` |
| `str` at 0x2ed10 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x2ed1c | load | 4 | `abs 0x2ee18` | `-` |
| `ldr` at 0x2ed28 | load | 4 | `abs 0x2ee1c` | `-` |
| `ldr` at 0x2ed34 | load | 4 | `abs 0x2ee20` | `-` |
| `ldr` at 0x2ed48 | load | 4 | `abs 0x2ee24` | `-` |
| `ldr` at 0x2ed5c | load | 4 | `abs 0x2ee28` | `-` |
| `ldr` at 0x2ed70 | load | 4 | `abs 0x2ee2c` | `-` |
| `ldrb` at 0x2ed74 | load | 1 | `abs 0x147f15` | `-` |
| `ldr` at 0x2ed80 | load | 4 | `abs 0x2ee20` | `-` |
| `ldr` at 0x2ed94 | load | 4 | `abs 0x2ee24` | `-` |
| `ldr` at 0x2eda8 | load | 4 | `abs 0x2ee28` | `-` |
| `ldmdb` at 0x2edbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2edbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2edbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2ebc4 | `(unknown callee) 0xc77ac` | `0x2e9d4` | `r1` | `r2` | `r3` |
| `bl` at 0x2ebd4 | `(unknown callee) 0xd4590` | `0x0` | `0x103bc0` | `0x2` | `?` |
| `bl` at 0x2ebdc | `(unknown callee) 0xc9278` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2ebec | `(unknown callee) 0xd8344` | `0x0` | `0x103bd0` | `0x2` | `?` |
| `bl` at 0x2ebfc | `(unknown callee) 0xd5b60` | `0x0` | `0x103d10` | `0x16` | `?` |
| `bl` at 0x2ec0c | `(unknown callee) 0xd1584` | `0x0` | `0x103f08` | `0x14` | `?` |
| `bl` at 0x2ec1c | `(unknown callee) 0xcbfec` | `0x0` | `0x104034` | `0x7` | `?` |
| `bl` at 0x2ec38 | `(unknown callee) 0x3c1c8` | `0x0` | `0x10410c` | `0xe` | `0x2d3bc` |
| `bl` at 0x2ec48 | `(unknown callee) 0x52c74` | `0x0` | `0x104438` | `0x7` | `?` |
| `bl` at 0x2ec58 | `(unknown callee) 0x5c2c4` | `0x0` | `0x104294` | `0x4` | `?` |
| `bl` at 0x2ec68 | `(unknown callee) 0x454a0` | `0x0` | `0x1042b4` | `0x4` | `?` |
| `bl` at 0x2ec78 | `(unknown callee) 0x45214` | `0x0` | `0x104320` | `0x8` | `?` |
| `bl` at 0x2ec84 | `(unknown callee) 0x308f8` | `0x104388` | `0x4` | `?` | `?` |
| `bl` at 0x2ec90 | `(unknown callee) 0xe9190` | `0x1425fc` | `0x2` | `?` | `?` |
| `bl` at 0x2ecb4 | `(unknown callee) 0x5eed8` | `0x104608` | `0x1` | `0x2e784` | `0x0` |
| `bl` at 0x2ecc4 | `(unknown callee) 0x41208` | `0x0` | `0x104570` | `0x6` | `?` |
| `bl` at 0x2eccc | `(unknown callee) 0x51298` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x2ecd0 | `(unknown callee) 0x2d288` | `ret(0x51298)` | `?` | `?` | `?` |
| `bl` at 0x2ecd4 | `(unknown callee) 0x40f20` | `ret(0x2d288)` | `?` | `?` | `?` |
| `bl` at 0x2ecd8 | `(unknown callee) 0x2d310` | `ret(0x40f20)` | `?` | `?` | `?` |
| `bl` at 0x2ece4 | `(unknown callee) 0x526e8` | `0x10461c` | `0x3` | `?` | `?` |
| `bl` at 0x2ecf4 | `(unknown callee) 0x532a0` | `0x1044a8` | `0x2` | `0x1` | `?` |
| `bl` at 0x2ecf8 | `(unknown callee) 0x534f4` | `ret(0x532a0)` | `?` | `?` | `?` |
| `bl` at 0x2ed24 | `(unknown callee) 0x574f4` | `0x0` | `0x1` | `0x1044d0` | `0x4` |
| `bl` at 0x2ed30 | `(unknown callee) 0xc84f8` | `0x14260c` | `0x3` | `?` | `?` |
| `bl` at 0x2ed44 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x2ed58 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x0` | `0x0` | `0x2` |
| `bl` at 0x2ed6c | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x2` | `0x0` | `0x3` |
| `bl` at 0x2ed90 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x2eda4 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x2edb8 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x0` | `0x15` | `0x15` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2ed7c | `[0x147f15]:1 == 0x0` | `[0x147f15]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2edbc | `r0` | `ret(0xc8698)` |

## `OpusIP65_i2cConfig`

- `OpusIP65_i2cConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3046c to 0x306f4.
  Established. The call to `0xd5b60` carries `0x0`, `0x104924`, `0xe`, unknown (`bl` at 0x304b8).
  The byte guard takes the bypass for `[0x147f22]:1 == 0x0` and falls through on nonzero (`beq` at
  0x30598). That fall-through includes `0x532a0` with `0x10503c`, `0x1`, `0x1`, `[0x147f22]:1` (`bl`
  at 0x305a8). The final r0 value is `ret(0xc8698)` (`ldmdb` at 0x30688). Bus configuration semantics are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3047c | load | 4 | `abs 0x3068c` | `-` |
| `ldr` at 0x30488 | load | 4 | `abs 0x30690` | `-` |
| `ldr` at 0x304a0 | load | 4 | `abs 0x30694` | `-` |
| `ldr` at 0x304b0 | load | 4 | `abs 0x30698` | `-` |
| `ldr` at 0x304c0 | load | 4 | `abs 0x3069c` | `-` |
| `ldr` at 0x304d0 | load | 4 | `abs 0x306a0` | `-` |
| `ldr` at 0x304dc | load | 4 | `abs 0x306a4` | `-` |
| `str` at 0x304e0 | store | 4 | `sp -0x1c` | `0x2f434` |
| `ldr` at 0x304e8 | load | 4 | `abs 0x306a8` | `-` |
| `ldr` at 0x304f0 | load | 4 | `abs 0x306ac` | `-` |
| `ldr` at 0x304fc | load | 4 | `abs 0x306b0` | `-` |
| `ldr` at 0x3050c | load | 4 | `abs 0x306b4` | `-` |
| `ldr` at 0x3051c | load | 4 | `abs 0x306b8` | `-` |
| `ldr` at 0x3052c | load | 4 | `abs 0x306bc` | `-` |
| `ldr` at 0x30538 | load | 4 | `abs 0x306c0` | `-` |
| `str` at 0x30548 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x30550 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x30554 | load | 4 | `abs 0x306c4` | `-` |
| `ldr` at 0x3055c | load | 4 | `abs 0x306c8` | `-` |
| `ldr` at 0x3056c | load | 4 | `abs 0x306cc` | `-` |
| `ldr` at 0x3058c | load | 4 | `abs 0x306d0` | `-` |
| `ldrb` at 0x30590 | load | 1 | `abs 0x147f22` | `-` |
| `ldr` at 0x3059c | load | 4 | `abs 0x306d4` | `-` |
| `ldr` at 0x305b0 | load | 4 | `abs 0x306d8` | `-` |
| `str` at 0x305b4 | store | 4 | `sp -0x1c` | `0x105068` |
| `str` at 0x305bc | store | 4 | `sp -0x18` | `0x4` |
| `ldr` at 0x305c0 | load | 4 | `abs 0x306dc` | `-` |
| `str` at 0x305c4 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x305d0 | load | 4 | `abs 0x306e0` | `-` |
| `ldr` at 0x305dc | load | 4 | `abs 0x306e4` | `-` |
| `ldr` at 0x305f0 | load | 4 | `abs 0x306e8` | `-` |
| `ldr` at 0x30604 | load | 4 | `abs 0x306e4` | `-` |
| `ldr` at 0x30618 | load | 4 | `abs 0x306ec` | `-` |
| `ldrb` at 0x3061c | load | 1 | `abs 0x147f21` | `-` |
| `ldr` at 0x30628 | load | 4 | `abs 0x306e4` | `-` |
| `ldr` at 0x3063c | load | 4 | `abs 0x306e8` | `-` |
| `ldr` at 0x30650 | load | 4 | `abs 0x306d0` | `-` |
| `ldrb` at 0x30654 | load | 1 | `abs 0x147f22` | `-` |
| `ldr` at 0x30660 | load | 4 | `abs 0x306f0` | `-` |
| `ldr` at 0x30674 | load | 4 | `abs 0x306e8` | `-` |
| `ldmdb` at 0x30688 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x30688 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x30688 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30480 | `(unknown callee) 0xc77ac` | `0x30290` | `r1` | `r2` | `r3` |
| `bl` at 0x30490 | `(unknown callee) 0xd4590` | `0x0` | `0x104848` | `0x2` | `?` |
| `bl` at 0x30498 | `(unknown callee) 0xc9278` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x304a8 | `(unknown callee) 0xd8344` | `0x0` | `0x104860` | `0x1` | `?` |
| `bl` at 0x304b8 | `(unknown callee) 0xd5b60` | `0x0` | `0x104924` | `0xe` | `?` |
| `bl` at 0x304c8 | `(unknown callee) 0xd1584` | `0x0` | `0x104a88` | `0x14` | `?` |
| `bl` at 0x304d8 | `(unknown callee) 0xcbfec` | `0x0` | `0x104bb4` | `0x6` | `?` |
| `bl` at 0x304f4 | `(unknown callee) 0x3c1c8` | `0x0` | `0x104cd8` | `0x14` | `0x2f400` |
| `bl` at 0x30504 | `(unknown callee) 0x52c74` | `0x0` | `0x104fe0` | `0x7` | `?` |
| `bl` at 0x30514 | `(unknown callee) 0x5c2c4` | `0x0` | `0x104f08` | `0x3` | `?` |
| `bl` at 0x30524 | `(unknown callee) 0x454a0` | `0x0` | `0x104f20` | `0x4` | `?` |
| `bl` at 0x30534 | `(unknown callee) 0x45214` | `0x0` | `0x104f84` | `0x7` | `?` |
| `bl` at 0x30540 | `(unknown callee) 0xe9190` | `0x142670` | `0x2` | `?` | `?` |
| `bl` at 0x30564 | `(unknown callee) 0x63be8` | `0x105114` | `0x1` | `0x30048` | `0x0` |
| `bl` at 0x30574 | `(unknown callee) 0x41208` | `0x0` | `0x1050ac` | `0x4` | `?` |
| `bl` at 0x3057c | `(unknown callee) 0x51298` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x30580 | `(unknown callee) 0x2f0bc` | `ret(0x51298)` | `?` | `?` | `?` |
| `bl` at 0x30584 | `(unknown callee) 0x40f20` | `ret(0x2f0bc)` | `?` | `?` | `?` |
| `bl` at 0x30588 | `(unknown callee) 0x2f36c` | `ret(0x40f20)` | `?` | `?` | `?` |
| `bl` at 0x305a8 | `(unknown callee) 0x532a0` | `0x10503c` | `0x1` | `0x1` | `[0x147f22]:1` |
| `bl` at 0x305ac | `(unknown callee) 0x534f4` | `ret(0x532a0)` | `?` | `?` | `?` |
| `bl` at 0x305d8 | `(unknown callee) 0x574f4` | `0x0` | `0x1` | `0x105050` | `0x2` |
| `bl` at 0x305ec | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x0` | `0xd` | `0xd` |
| `bl` at 0x30600 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x30614 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x30638 | `(unknown callee) 0xc8698` | `0xd6dd4` | `0x1` | `0xb` | `0xc` |
| `bl` at 0x3064c | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x1` | `0x3` | `0x5` |
| `bl` at 0x30670 | `(unknown callee) 0xc8698` | `0xd94a8` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x30684 | `(unknown callee) 0xc8698` | `0xcdd1c` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30598 | `[0x147f22]:1 == 0x0` | `[0x147f22]:1 != 0x0` |
| `beq` at 0x30624 | `[0x147f21]:1 == 0x0` | `[0x147f21]:1 != 0x0` |
| `beq` at 0x3065c | `[0x147f22]:1 == 0x0` | `[0x147f22]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x30688 | `r0` | `ret(0xc8698)` |

## `networkSerial`

- `networkSerial`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x246bc to 0x247c4.
  Established. The call to `0xd6a1c` has `0x0`, `0x5`, `0x6e`, `0x44` (`bl` at 0x24760); its zero
  result takes the arm that stores byte `0x1` at `0x147ea0`, while nonzero falls through (`beq` at
  0x24774, `strb` at 0x247b0). The nonzero arm calls `0x21e28` with `0x3`, `0xfeb3c`, `0x90`, `0x0`
  (`bl` at 0x247a0). Physical serial routing and the joined r0 return remain unproven (`ldmdb` at
  0x247b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x246d8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x246dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x246e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x246f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24704 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x24718 | store | 1 | `sp -0x15` | `0x5` |
| `strb` at 0x24720 | store | 1 | `sp -0x16` | `0x6e` |
| `strb` at 0x24728 | store | 1 | `sp -0x17` | `0x44` |
| `strb` at 0x24734 | store | 1 | `sp -0x15` | `0x5` |
| `strb` at 0x2473c | store | 1 | `sp -0x16` | `0x6e` |
| `strb` at 0x24744 | store | 1 | `sp -0x17` | `0x44` |
| `ldrb` at 0x24748 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2474c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x24750 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x24768 | store | 4 | `sp -0x14` | `ret(0xd6a1c)` |
| `ldr` at 0x2476c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24778 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24784 | load | 4 | `abs 0x247b8` | `-` |
| `str` at 0x24788 | store | 4 | `sp -0x24` | `0xfeb00` |
| `str` at 0x2478c | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x24794 | load | 4 | `abs 0x247bc` | `-` |
| `ldr` at 0x247a8 | load | 4 | `abs 0x247c0` | `-` |
| `strb` at 0x247b0 | store | 1 | `abs 0x147ea0` | `0x1` |
| `ldmdb` at 0x247b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x247b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x247b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x246cc | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x24760 | `(unknown callee) 0xd6a1c` | `0x0` | `0x5` | `0x6e` | `0x44` |
| `bl` at 0x2477c | `(unknown callee) 0x219a8` | `ret(0xd6a1c)` | `?` | `?` | `ret(0xd6a1c)` |
| `bl` at 0x247a0 | `(unknown callee) 0x21e28` | `0x3` | `0xfeb3c` | `0x90` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x246e4 | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `bgt` at 0x246f0 | `[sp-28] > 0x35 (signed, 32-bit)` | `[sp-28] <= 0x35 (signed, 32-bit)` |
| `beq` at 0x246fc | `[sp-28] == 0x21` | `[sp-28] != 0x21` |
| `beq` at 0x2470c | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0x24774 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x247b4 | `r0` | `?` |

## `L1CfgEventNVRAM`

- `L1CfgEventNVRAM`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x752c8 to 0x752f4.
  Established. The sole call reaches `0x74c78` with `0x66`, entry r0, entry r1, entry r3 (`bl` at
  0x752ec); its result reaches r0 at the return (`ldmdb` at 0x752f0). The two explicit word saves
  place entry r0 at `sp -0x14` and entry r1 at `sp -0x18` (`str` at 0x752d8, `str` at 0x752dc).
  Event transport and NVRAM modification are not proved by this wrapper.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x752d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x752dc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x752e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x752e8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x752f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x752f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x752f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x752ec | `(unknown callee) 0x74c78` | `0x66` | `r0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x752f0 | `r0` | `ret(0x74c78)` |

## `EnvPower`

- `EnvPower`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82084 to 0x820ac.
  Established. The first call reaches `0x1d38c` with `0x7d`, entry r0, entry r2, entry r3 (`bl` at
  0x820a0). The following call to `0x82140` has `ret(0x1d38c)` and three unknown candidates (`bl` at
  0x820a4); its result remains in r0 at the return (`ldmdb` at 0x820a8). This wrapper alone does not
  establish physical power behavior.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82094 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8209c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x820a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x820a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x820a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x820a0 | `(unknown callee) 0x1d38c` | `0x7d` | `r0` | `r2` | `r3` |
| `bl` at 0x820a4 | `(unknown callee) 0x82140` | `ret(0x1d38c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x820a8 | `r0` | `ret(0x82140)` |

## `EnvPowerUpStart`

- `EnvPowerUpStart`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x823b0 to 0x823c8.
  Established. The call to `0x807cc` uses `0x2`, entry r1, entry r2, entry r3 (`bl` at 0x823c0). The
  r0 return is `ret(0x807cc)` (`ldmdb` at 0x823c4). No conditional branch or explicit store is
  reported beyond the implicit stack save in the full facts. The constant's external meaning
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x823c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x823c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x823c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x823c0 | `(unknown callee) 0x807cc` | `0x2` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x823c4 | `r0` | `ret(0x807cc)` |

## `EnvPowerUpDone`

- `EnvPowerUpDone`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x823c8 to 0x823f4.
  Established. A word store writes `ret(0x1b1e4)` at absolute `0x148e1c` (`str` at 0x823e0). The
  prior call has `0x0`, entry r1, entry r2, entry r3 (`bl` at 0x823dc). The next call reaches
  `0x807cc` with `0x4` and three unknown candidates, supplying r0 `ret(0x807cc)` at the return (`bl`
  at 0x823e8, `ldmdb` at 0x823ec). The stored value's units and power-completion interpretation
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x823d4 | load | 4 | `abs 0x823f0` | `-` |
| `str` at 0x823e0 | store | 4 | `abs 0x148e1c` | `ret(0x1b1e4)` |
| `ldmdb` at 0x823ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x823ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x823ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x823ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x823dc | `(unknown callee) 0x1b1e4` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x823e8 | `(unknown callee) 0x807cc` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x823ec | `r0` | `ret(0x807cc)` |

## `EnvPowerDown`

- `EnvPowerDown`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x823f4 to 0x82414.
  Established. Entry r0 is saved as a word at `sp -0x14` and reloaded before the call (`str` at
  0x82404, `ldr` at 0x82408). The call to `0x7bb1c` has entry r0, entry r1, entry r2, entry r3 (`bl`
  at 0x8240c); its result is retained in r0 at the return (`ldmdb` at 0x82410). No external shutdown
  sequence can be assigned from this forwarding body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82404 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82408 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82410 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82410 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82410 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8240c | `(unknown callee) 0x7bb1c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82410 | `r0` | `ret(0x7bb1c)` |

## `EnvPowerDownStart`

- `EnvPowerDownStart`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82414 to 0x8242c.
  Established. The only call targets `0x807cc` with `0x3`, entry r1, entry r2, entry r3 (`bl` at
  0x82424). The returned r0 value is `ret(0x807cc)` (`ldmdb` at 0x82428). The facts contain no
  conditional branch or explicit data store. Power-down state semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x82428 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82428 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82428 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82424 | `(unknown callee) 0x807cc` | `0x3` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82428 | `r0` | `ret(0x807cc)` |

## `EnvPowerDownDone`

- `EnvPowerDownDone`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8242c to 0x82448.
  Established. The opening call reaches `0x807cc` with `0x4`, entry r1, entry r2, entry r3 (`bl` at
  0x8243c). The following call to `0x85a0c` has `ret(0x807cc)` and three unknown candidates (`bl` at
  0x82440). The r0 return is `ret(0x85a0c)` (`ldmdb` at 0x82444). Neither a completion signal nor a
  physical power change is established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x82444 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82444 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82444 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8243c | `(unknown callee) 0x807cc` | `0x4` | `r1` | `r2` | `r3` |
| `bl` at 0x82440 | `(unknown callee) 0x85a0c` | `ret(0x807cc)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82444 | `r0` | `ret(0x85a0c)` |

## `EnvIgnoreLowPower`

- `EnvIgnoreLowPower`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x824b0 to 0x824e4.
  Established. The absolute word store addresses `0x148dfc` with an unknown value (`str` at
  0x824d8). Entry r0 is saved as a word at `sp -0x14`, and the return retains entry r0 (`str` at
  0x824c0, `ldmdb` at 0x824dc). No calls or conditional branches are reported. The stored value's interpretation
  and low-power suppression behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x824c0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x824c4 | load | 4 | `abs 0x824e0` | `-` |
| `ldr` at 0x824c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x824d8 | store | 4 | `abs 0x148dfc` | `?` |
| `ldmdb` at 0x824dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x824dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x824dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x824dc | `r0` | `r0` |
