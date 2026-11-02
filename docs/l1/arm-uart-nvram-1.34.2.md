# ARM L1 UART and NVRAM entry paths in 1.34.2

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
values, and brackets denote loaded values. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. `?` and `(unknown)` mark values or
effective addresses the analysis cannot resolve, including unsupported or merged effects.
They do not imply that the firmware itself uses an indeterminate value. Stack stores appear
in the access tables.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/uart-nvram-facts.txt) retain all tool sections.

## `uartInit`

- `uartInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad4d4 to 0xad6a0.
  Established. The word read at 0x282b80 decides `ble`, whose taken path skips the first call to
  0x1eed0 when that word is at most 0x13 signed (`ldr` at 0xad4f4, `ble` at 0xad4fc, `bl` at
  0xad518). A later word store writes `[0x282b80]+1` back there (`str` at 0xad588). The call to
  0x18c20 has r0 candidate `[sp-32]`, r1 candidate 0x0 and r2 candidate 0x108 (`bl` at 0xad598).
  Subsequent record stores include `[sp-20]` at `[sp-32] +0x8`, `[sp-24]` at +0xc and `[sp-28]` at
  +0x2c (`str` at 0xad5a4, `str` at 0xad5b0, `str` at 0xad5bc). The record and UART numbering remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad4e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xad4e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xad4ec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xad4f0 | load | 4 | `abs 0xad684` | `-` |
| `ldr` at 0xad4f4 | load | 4 | `abs 0x282b80` | `-` |
| `ldr` at 0xad500 | load | 4 | `abs 0xad688` | `-` |
| `str` at 0xad504 | store | 4 | `sp -0x2c` | `0x10826c` |
| `ldr` at 0xad50c | load | 4 | `abs 0xad68c` | `-` |
| `ldr` at 0xad51c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad530 | load | 4 | `abs 0xad690` | `-` |
| `str` at 0xad534 | store | 4 | `sp -0x2c` | `0x108298` |
| `ldr` at 0xad538 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xad53c | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xad544 | load | 4 | `abs 0xad68c` | `-` |
| `ldr` at 0xad554 | load | 4 | `abs 0xad684` | `-` |
| `ldr` at 0xad558 | load | 4 | `abs 0x282b80` | `-` |
| `ldr` at 0xad56c | load | 4 | `abs 0xad694` | `-` |
| `str` at 0xad574 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xad578 | load | 4 | `abs 0xad684` | `-` |
| `ldr` at 0xad57c | load | 4 | `abs 0xad684` | `-` |
| `ldr` at 0xad580 | load | 4 | `abs 0x282b80` | `-` |
| `str` at 0xad588 | store | 4 | `abs 0x282b80` | `[0x282b80]+1` |
| `ldr` at 0xad58c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad59c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad5a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xad5a4 | store | 4 | `[sp-32] +0x8` | `[sp-20]` |
| `ldr` at 0xad5a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad5ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xad5b0 | store | 4 | `[sp-32] +0xc` | `[sp-24]` |
| `ldr` at 0xad5b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad5b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xad5bc | store | 4 | `[sp-32] +0x2c` | `[sp-28]` |
| `ldr` at 0xad5c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xad5c8 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xad5cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xad5d4 | store | 4 | `[sp-32] +0x4` | `?` |
| `ldr` at 0xad5d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad5e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad5f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad5f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xad5f8 | load | 4 | `abs 0xad698` | `-` |
| `strb` at 0xad608 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xad610 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad61c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad628 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad62c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xad630 | load | 4 | `abs 0xad69c` | `-` |
| `strb` at 0xad640 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xad648 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad660 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad664 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xad674 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xad678 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xad680 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad680 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad680 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad518 | `(unknown callee) 0x1eed0` | `0x9` | `0x108290` | `0x58` | `0x0` |
| `bl` at 0xad520 | `(unknown callee) 0xae248` | `?` | `?` | `?` | `?` |
| `bl` at 0xad550 | `(unknown callee) 0x1eed0` | `0x9` | `0x108290` | `0x5b` | `0x0` |
| `bl` at 0xad598 | `(unknown callee) 0x18c20` | `[sp-32]` | `0x0` | `0x108` | `[0x282b80]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xad4fc | `[0x282b80] <= 0x13 (signed, 32-bit)` | `[0x282b80] > 0x13 (signed, 32-bit)` |
| `beq` at 0xad52c | `ret(0xae248) == 0x0` | `ret(0xae248) != 0x0` |
| `bls` at 0xad5e0 | `[sp-20] <= 0x4 (unsigned, 32-bit)` | `[sp-20] > 0x4 (unsigned, 32-bit)` |
| `bhi` at 0xad5ec | `[sp-20] > 0xc (unsigned, 32-bit)` | `[sp-20] <= 0xc (unsigned, 32-bit)` |
| `bls` at 0xad618 | `[sp-20] <= 0xc (unsigned, 32-bit)` | `[sp-20] > 0xc (unsigned, 32-bit)` |
| `bhi` at 0xad624 | `[sp-20] > 0x10 (unsigned, 32-bit)` | `[sp-20] <= 0x10 (unsigned, 32-bit)` |
| `bls` at 0xad650 | `[sp-20] <= 0x10 (unsigned, 32-bit)` | `[sp-20] > 0x10 (unsigned, 32-bit)` |
| `bhi` at 0xad65c | `[sp-20] > 0x16 (unsigned, 32-bit)` | `[sp-20] <= 0x16 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad680 | `r0` | `[sp-32]` |

## `uartOpen`

- `uartOpen`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad6a0 to 0xadb18.
  Established. The initial call targets 0xae248 with entry r0 as its first candidate (`bl` at
  0xad6c4). `bne` takes its nonzero-result path into the remaining setup, while zero fall-through
  stores 0x0 at sp -0x34 and jumps to the final return sequence (`bne` at 0xad6d8, `str` at 0xad6e0,
  `b` at 0xad6e4). Two later calls to 0x18bf0 use r1 candidates 0x80047503 and 0x80047506, with r2
  candidates 0x400 and 0x1 (`bl` at 0xada4c, `bl` at 0xadacc). Each following `bge` takes its
  nonnegative-result path past a call to 0xadb18 (`bge` at 0xada58, `bl` at 0xada64, `bge` at
  0xadad8, `bl` at 0xadae4). The numeric request meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad6b0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xad6b4 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xad6bc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xad6c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xad6cc | store | 4 | `sp -0x24` | `ret(0xae248)` |
| `ldr` at 0xad6d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad6e0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xad6e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xad6f0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xad6f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xad700 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xad70c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xad71c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xad72c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad730 | load | 4 | `ret(0xae248) +0x34` | `-` |
| `ldr` at 0xad73c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad748 | load | 4 | `ret(0xae248) +0x34` | `-` |
| `str` at 0xad758 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xad760 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad768 | store | 4 | `(unknown)` | `ret(0x1f740)` |
| `str` at 0xad770 | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xad778 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad77c | load | 4 | `ret(0xae248) +0x38` | `-` |
| `ldr` at 0xad788 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad794 | load | 4 | `ret(0xae248) +0x38` | `-` |
| `str` at 0xad7a4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xad7ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad7b4 | store | 4 | `(unknown)` | `ret(0x1f740)` |
| `str` at 0xad7bc | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xad7c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad7c8 | load | 4 | `ret(0xae248) +0x34` | `-` |
| `ldr` at 0xad7d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad7d8 | load | 4 | `ret(0xae248) +0x38` | `-` |
| `ldr` at 0xad7e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad7f4 | load | 4 | `ret(0xae248) +0x34` | `-` |
| `ldr` at 0xad800 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad80c | load | 4 | `[sp-36] +0x38` | `-` |
| `str` at 0xad820 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xad828 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad82c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad834 | store | 4 | `(unknown)` | `ret(0x1f740)` |
| `ldr` at 0xad838 | load | 4 | `(unknown)` | `-` |
| `str` at 0xad83c | store | 4 | `(unknown)` | `?` |
| `str` at 0xad844 | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xad848 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xad860 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad864 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xad870 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad874 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xad880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad884 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xad894 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad89c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xad8a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xad8a8 | store | 4 | `[sp-36] +0x4` | `0x1` |
| `ldr` at 0xad8ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad8b0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xad8c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad8cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xad8d8 | load | 4 | `abs 0xadb00` | `-` |
| `ldr` at 0xad8dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xad8e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xad8e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xad8f0 | store | 4 | `sp -0x30` | `ret(0x18b54)` |
| `ldr` at 0xad8f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xad908 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xad91c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xad934 | load | 4 | `abs 0xadb04` | `-` |
| `str` at 0xad938 | store | 4 | `sp -0x50` | `0x1082bc` |
| `ldr` at 0xad93c | load | 4 | `abs 0xadb08` | `-` |
| `str` at 0xad940 | store | 4 | `sp -0x4c` | `0x1082d8` |
| `str` at 0xad944 | store | 4 | `sp -0x48` | `ret(0x1e71c)` |
| `ldr` at 0xad948 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xad94c | store | 4 | `sp -0x44` | `[ret(0x188b4)]` |
| `ldr` at 0xad954 | load | 4 | `abs 0xadb0c` | `-` |
| `str` at 0xad968 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xad96c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xad980 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xad98c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad998 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xad9a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad9ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xad9b0 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xad9b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad9c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xad9d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad9d4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xad9d8 | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xad9dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad9e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad9ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xad9f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad9fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xada0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xada1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xada20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xada24 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xada2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xada30 | load | 4 | `(unknown)` | `-` |
| `str` at 0xada34 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xada38 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xada3c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xada40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xada44 | load | 4 | `abs 0xadb10` | `-` |
| `ldr` at 0xada5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xada60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xada6c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xada70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xada7c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xada80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xada8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xada9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xadaa0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xadaa4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xadaac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xadab0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xadab4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xadab8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xadabc | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xadac0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xadac4 | load | 4 | `abs 0xadb14` | `-` |
| `ldr` at 0xadadc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xadae0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xadaec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xadaf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xadaf4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xadaf8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xadafc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xadafc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xadafc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadafc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadafc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad6c4 | `(unknown callee) 0xae248` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xad740 | `(unknown callee) 0x1f740` | `ret(0xae248)` | `?` | `?` | `[ret(0xae248)+52]` |
| `bl` at 0xad764 | `(unknown callee) 0x1f740` | `?` | `?` | `?` | `[ret(0xae248)+52]` |
| `bl` at 0xad78c | `(unknown callee) 0x1f740` | `ret(0xae248)` | `?` | `?` | `[ret(0xae248)+56]` |
| `bl` at 0xad7b0 | `(unknown callee) 0x1f740` | `?` | `?` | `?` | `[ret(0xae248)+56]` |
| `bl` at 0xad7ec | `(unknown callee) 0x1f740` | `ret(0xae248)` | `?` | `?` | `?` |
| `bl` at 0xad804 | `(unknown callee) 0x1f740` | `ret(0x1f740)` | `?` | `ret(0x1f740)` | `[ret(0xae248)+52]` |
| `bl` at 0xad830 | `(unknown callee) 0x1f740` | `?` | `?` | `?` | `?` |
| `bl` at 0xad8b8 | `(unknown callee) 0xae344` | `[[sp-36]]` | `0xf` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xad8e8 | `(unknown callee) 0x18b54` | `?` | `[sp-32]` | `0x115720` | `[sp-28]` |
| `bl` at 0xad900 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0xad914 | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `[ret(0x188b4)]` |
| `bl` at 0xad920 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xad928 | `(unknown callee) 0x188b4` | `ret(0x1e71c)` | `?` | `?` | `?` |
| `bl` at 0xad960 | `(unknown callee) 0x1eed0` | `0x3` | `0x108290` | `0xcf` | `0x0` |
| `bl` at 0xad978 | `(unknown callee) 0x188b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xada4c | `(unknown callee) 0x18bf0` | `[sp-60]` | `0x80047503` | `0x400` | `[sp-60]` |
| `bl` at 0xada64 | `(unknown callee) 0xadb18` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x18bf0)` |
| `bl` at 0xadacc | `(unknown callee) 0x18bf0` | `[sp-64]` | `0x80047506` | `0x1` | `[sp-64]` |
| `bl` at 0xadae4 | `(unknown callee) 0xadb18` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x18bf0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xad6d8 | `ret(0xae248) != 0x0` | `ret(0xae248) == 0x0` |
| `beq` at 0xad6fc | `[sp-56] == 0x1` | `[sp-56] != 0x1` |
| `bgt` at 0xad708 | `[sp-56] > 0x1 (signed, 32-bit)` | `[sp-56] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xad714 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0xad724 | `[sp-56] == 0x2` | `[sp-56] != 0x2` |
| `beq` at 0xad738 | `[ret(0xae248)+52] == 0x0` | `[ret(0xae248)+52] != 0x0` |
| `beq` at 0xad750 | `[ret(0xae248)+52] == ret(0x1f740)` | `[ret(0xae248)+52] != ret(0x1f740)` |
| `beq` at 0xad784 | `[ret(0xae248)+56] == 0x0` | `[ret(0xae248)+56] != 0x0` |
| `beq` at 0xad79c | `[ret(0xae248)+56] == ret(0x1f740)` | `[ret(0xae248)+56] != ret(0x1f740)` |
| `bne` at 0xad7d0 | `[ret(0xae248)+52] != 0x0` | `[ret(0xae248)+52] == 0x0` |
| `bne` at 0xad7e0 | `[ret(0xae248)+56] != 0x0` | `[ret(0xae248)+56] == 0x0` |
| `bne` at 0xad7fc | `[ret(0xae248)+52] != ret(0x1f740)` | `[ret(0xae248)+52] == ret(0x1f740)` |
| `bne` at 0xad814 | `[[sp-36]+56] != ret(0x1f740)` | `[[sp-36]+56] == ret(0x1f740)` |
| `beq` at 0xad850 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xad85c | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `bge` at 0xad86c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xad87c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xad88c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xad8c8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xad8d4 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bge` at 0xad8fc | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `beq` at 0xad910 | `[ret(0x188b4)] == 0x4` | `[ret(0x188b4)] != 0x4` |
| `bge` at 0xad974 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xad988 | `[ret(0x188b4)] == 0x4` | `[ret(0x188b4)] != 0x4` |
| `beq` at 0xad994 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xad9a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xad9bc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xad9cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xad9e4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xad9f4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xada04 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xada18 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xada58 | `ret(0x18bf0) >= 0x0 (signed, 32-bit)` | `ret(0x18bf0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xada78 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xada88 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xada98 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xadad8 | `ret(0x18bf0) >= 0x0 (signed, 32-bit)` | `ret(0x18bf0) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadafc | `r0` | `?` |

## `uartClose`

- `uartClose`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadb18 to 0xadd10.
  Established. The early branches select paths by the saved word `[sp-48]`, comparing it with 0x1,
  0x0 and 0x2 (`beq` at 0xadb54, `beq` at 0xadb6c, `beq` at 0xadb7c). The body includes word stores
  of 0x0 at `[sp-24] +0x34` and `[sp-24] +0x38` (`str` at 0xadc44, `str` at 0xadc50). Later word
  stores at that loaded base +0x0 and +0x4 have unknown values (`str` at 0xadcd8, `str` at 0xadd00).
  Calls include 0xae488 and two sites targeting 0x18800 (`bl` at 0xadc94, `bl` at 0xadccc, `bl` at
  0xadcf4). The descriptor ownership and device-release semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadb28 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xadb2c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xadb34 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xadb3c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xadb40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xadb48 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xadb4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xadb58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xadb64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xadb74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xadb84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadb90 | load | 4 | `r0 +0x34` | `-` |
| `str` at 0xadba0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xadba8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadbb0 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `str` at 0xadbb8 | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xadbc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadbcc | load | 4 | `r0 +0x38` | `-` |
| `str` at 0xadbdc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xadbe4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadbec | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xadbf4 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xadbfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc08 | load | 4 | `r0 +0x34` | `-` |
| `ldr` at 0xadc14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc20 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0xadc34 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xadc3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadc44 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `ldr` at 0xadc48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadc50 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xadc58 | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xadc5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xadc6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xadc7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xadc8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xadc98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadc9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadca4 | store | 4 | `[sp-24] +0x4` | `?` |
| `str` at 0xadcac | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xadcb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xadcc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadcc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xadcd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadcd8 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xadcdc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xadcec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadcf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xadcf8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadd00 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xadd04 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xadd0c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xadd0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadd0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadd0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xadb88 | `(unknown callee) 0x1f740` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xadbc4 | `(unknown callee) 0x1f740` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xadc00 | `(unknown callee) 0x1f740` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xadc18 | `(unknown callee) 0x1f740` | `ret(0x1f740)` | `?` | `ret(0x1f740)` | `[r0+52]` |
| `bl` at 0xadc94 | `(unknown callee) 0xae488` | `?` | `?` | `?` | `?` |
| `bl` at 0xadccc | `(unknown callee) 0x18800` | `?` | `?` | `?` | `?` |
| `bl` at 0xadcf4 | `(unknown callee) 0x18800` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xadb54 | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `bgt` at 0xadb60 | `[sp-48] > 0x1 (signed, 32-bit)` | `[sp-48] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xadb6c | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xadb7c | `[sp-48] == 0x2` | `[sp-48] != 0x2` |
| `beq` at 0xadb98 | `[r0+52] == ret(0x1f740)` | `[r0+52] != ret(0x1f740)` |
| `beq` at 0xadbd4 | `[r0+56] == ret(0x1f740)` | `[r0+56] != ret(0x1f740)` |
| `bne` at 0xadc10 | `[r0+52] != ret(0x1f740)` | `[r0+52] == ret(0x1f740)` |
| `bne` at 0xadc28 | `[[sp-24]+56] != ret(0x1f740)` | `[[sp-24]+56] == ret(0x1f740)` |
| `bne` at 0xadc68 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xadc78 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xadc88 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xadcc0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xadce8 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadd0c | `r0` | `?` |

## `uartRead`

- `uartRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadd10 to 0xaded0.
  Established. `bgt` takes the positive-entry-r2 path into the body. Fall-through saves 0x0 at sp
  -0xac and jumps to the return join (`bgt` at 0xadd34, `str` at 0xadd3c, `b` at 0xadd40). The call
  to 0x18860 uses r0 candidate `[[sp-20]]+1`, r1 candidate sp-160 and two zero candidates (`bl` at
  0xade04). A later call to 0x18cc8 uses `[[sp-20]]`, `[sp-24]`, `[sp-28]` and `[sp-20]` (`bl` at
  0xade40). The later `bge` takes the nonnegative `[sp-32]` path past the call to 0xae02c (`bge` at
  0xade94, `bl` at 0xadea0). The call targets' input meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadd20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xadd24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xadd28 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xadd2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xadd3c | store | 4 | `sp -0xac` | `0x0` |
| `str` at 0xadd48 | store | 4 | `sp -0xa8` | `sp-160` |
| `str` at 0xadd50 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0xadd54 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xadd64 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xadd68 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xadd70 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xadd74 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xadd7c | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xadd84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaddc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaddc4 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xaddd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xadddc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xadde0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadde4 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xaddf4 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0xade1c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xade24 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xade30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xade34 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xade38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xade3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xade48 | store | 4 | `sp -0x20` | `ret(0x18cc8)` |
| `ldr` at 0xade4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xade60 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xade70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xade74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xade78 | load | 4 | `abs 0xadecc` | `-` |
| `ldr` at 0xade7c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xade80 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xade8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xade98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadeac | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xadeb4 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xadebc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xadec0 | store | 4 | `sp -0xac` | `[sp-32]` |
| `ldr` at 0xadec4 | load | 4 | `sp -0xac` | `-` |
| `ldmdb` at 0xadec8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadec8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadec8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xade04 | `(unknown callee) 0x18860` | `[[sp-20]]+1` | `sp-160` | `0x0` | `0x0` |
| `bl` at 0xade14 | `(unknown callee) 0x188b4` | `ret(0x18860)` | `?` | `?` | `ret(0x18860)` |
| `bl` at 0xade40 | `(unknown callee) 0x18cc8` | `[[sp-20]]` | `[sp-24]` | `[sp-28]` | `[sp-20]` |
| `bl` at 0xade58 | `(unknown callee) 0x188b4` | `ret(0x18cc8)` | `?` | `?` | `ret(0x18cc8)` |
| `bl` at 0xade84 | `(unknown callee) 0x1dba4` | `0x1082e4` | `[[sp-20]+8]` | `[[sp-20]]` | `[sp-20]` |
| `bl` at 0xadea0 | `(unknown callee) 0xae02c` | `[sp-20]` | `0x1` | `?` | `[sp-32]` |
| `bl` at 0xadea4 | `(unknown callee) 0x188b4` | `ret(0xae02c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xadd34 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bls` at 0xadd5c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bge` at 0xade10 | `ret(0x18860) >= 0x0 (signed, 32-bit)` | `ret(0x18860) < 0x0 (signed, 32-bit)` |
| `bne` at 0xade54 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xade68 | `[ret(0x188b4)] == 0x23` | `[ret(0x188b4)] != 0x23` |
| `bge` at 0xade94 | `[sp-32] >= 0x0 (signed, 32-bit)` | `[sp-32] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadec8 | `r0` | `?` |

## `uartWrite`

- `uartWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaded0 to 0xadf60.
  Established. The first `bgt` takes the positive-entry-r2 path to the call to 0x187b8. Fall-through saves zero at sp -0x24 (`bgt` at 0xadef4, `bl` at 0xadf14, `str` at 0xadefc). That
  call receives candidates `[r0+4]`, entry r1, entry r2 and entry r0 (`bl` at 0xadf14). The
  following `bge` takes its nonnegative-result path past 0xae02c and 0x188b4 (`bge` at 0xadf28, `bl`
  at 0xadf34, `bl` at 0xadf38). The result saved on the negative-result path after those calls remains unknown (`str` at
  0xadf48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadee0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xadee4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xadee8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xadeec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xadefc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xadf04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf08 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xadf0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadf10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xadf1c | store | 4 | `sp -0x20` | `ret(0x187b8)` |
| `ldr` at 0xadf20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xadf2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf40 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xadf48 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xadf50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xadf54 | store | 4 | `sp -0x24` | `ret(0x187b8)` |
| `ldr` at 0xadf58 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xadf5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadf5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadf5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xadf14 | `(unknown callee) 0x187b8` | `[r0+4]` | `r1` | `r2` | `r0` |
| `bl` at 0xadf34 | `(unknown callee) 0xae02c` | `[sp-20]` | `0x2` | `?` | `ret(0x187b8)` |
| `bl` at 0xadf38 | `(unknown callee) 0x188b4` | `ret(0xae02c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xadef4 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bge` at 0xadf28 | `ret(0x187b8) >= 0x0 (signed, 32-bit)` | `ret(0x187b8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadf5c | `r0` | `?` |

## `uartGetConfig`

- `uartGetConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadf60 to 0xadf9c.
  Established. The two `ldm` sites read seven words from entry r0 offsets +0x10 through +0x28 (`ldm`
  at 0xadf88, `ldm` at 0xadf90). The two `stm` sites write those loaded expressions to entry r1
  offsets +0x0 through +0x18 (`stm` at 0xadf8c, `stm` at 0xadf94). The return's r0 value is
  `[r0+32]` (`ldmdb` at 0xadf98). The copied record's fields are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadf70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xadf74 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xadf78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadf7c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xadf88 | load | 4 | `r0 +0x10` | `-` |
| `ldm` at 0xadf88 | load | 4 | `r0 +0x14` | `-` |
| `ldm` at 0xadf88 | load | 4 | `r0 +0x18` | `-` |
| `ldm` at 0xadf88 | load | 4 | `r0 +0x1c` | `-` |
| `stm` at 0xadf8c | store | 4 | `r1 +0x0` | `[r0+16]` |
| `stm` at 0xadf8c | store | 4 | `r1 +0x4` | `[r0+20]` |
| `stm` at 0xadf8c | store | 4 | `r1 +0x8` | `[r0+24]` |
| `stm` at 0xadf8c | store | 4 | `r1 +0xc` | `[r0+28]` |
| `ldm` at 0xadf90 | load | 4 | `r0 +0x20` | `-` |
| `ldm` at 0xadf90 | load | 4 | `r0 +0x24` | `-` |
| `ldm` at 0xadf90 | load | 4 | `r0 +0x28` | `-` |
| `stm` at 0xadf94 | store | 4 | `r1 +0x10` | `[r0+32]` |
| `stm` at 0xadf94 | store | 4 | `r1 +0x14` | `[r0+36]` |
| `stm` at 0xadf94 | store | 4 | `r1 +0x18` | `[r0+40]` |
| `ldmdb` at 0xadf98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadf98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadf98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadf98 | `r0` | `[r0+32]` |

## `uartSetConfig`

- `uartSetConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadf9c to 0xae02c.
  Established. The byte at entry r1 +0x16 decides `bne`, whose taken unequal-to-0x1 path skips the
  store of 0x2 back to that byte (`ldrb` at 0xadfb8, `bne` at 0xadfc0, `strb` at 0xadfcc). Two
  further byte stores write 0x2 at unknown effective targets (`strb` at 0xadfe8, `strb` at 0xae004).
  The following multiword stores also have unknown targets and values (`stm` at 0xae01c, `stm` at
  0xae024). The later `bne` predicates retain only Z == 0 (`bne` at 0xadfdc, `bne` at 0xadff8). No
  additional field offsets are established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadfac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xadfb0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xadfb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xadfb8 | load | 1 | `r1 +0x16` | `-` |
| `ldr` at 0xadfc4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xadfcc | store | 1 | `r1 +0x16` | `0x2` |
| `ldr` at 0xadfd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xadfd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xadfe0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xadfe8 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xadfec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xadff0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xadffc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xae004 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xae008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae00c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xae018 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xae018 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xae018 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xae018 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xae01c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xae01c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xae01c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xae01c | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xae020 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xae020 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xae020 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xae024 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xae024 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xae024 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xae028 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae028 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae028 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xadfc0 | `[r1+22]:1 != 0x1` | `[r1+22]:1 == 0x1` |
| `bne` at 0xadfdc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xadff8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae028 | `r0` | `?` |

## `uartFlush`

- `uartFlush`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae02c to 0xae0d4.
  Established. The first `beq` tests an unknown expression against zero (`beq` at 0xae050); its
  taken path saves `[r0]` at sp -0x20, while fall-through saves `[r0+4]` there (`str` at 0xae06c,
  `str` at 0xae05c). The call to 0x18bf0 has candidates `[sp-32]`, 0x80047502, entry r1 and
  `[sp-32]` (`bl` at 0xae084). `bge` takes its nonnegative-result path past the three later calls
  (`bge` at 0xae090, `bl` at 0xae094, `bl` at 0xae0a0, `bl` at 0xae0c0). The numeric operation's
  hardware meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae03c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xae040 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xae044 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae054 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae058 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xae05c | store | 4 | `sp -0x20` | `[r0+4]` |
| `ldr` at 0xae064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae068 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xae06c | store | 4 | `sp -0x20` | `[r0]` |
| `ldr` at 0xae070 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xae074 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xae078 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xae07c | load | 4 | `abs 0xae0c8` | `-` |
| `ldr` at 0xae080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae09c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xae0a8 | load | 4 | `abs 0xae0cc` | `-` |
| `ldr` at 0xae0ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae0b0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0xae0b4 | load | 4 | `abs 0xae0d0` | `-` |
| `ldr` at 0xae0b8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldmdb` at 0xae0c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae0c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae0c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae084 | `(unknown callee) 0x18bf0` | `[sp-32]` | `0x80047502` | `r1` | `[sp-32]` |
| `bl` at 0xae094 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xae0a0 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xae0c0 | `(unknown callee) 0x21010` | `[0x116928]` | `0x108308` | `[[sp-20]+8]` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae050 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xae090 | `ret(0x18bf0) >= 0x0 (signed, 32-bit)` | `ret(0x18bf0) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae0c4 | `r0` | `?` |

## `uartSendBreak`

- `uartSendBreak`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae0d4 to 0xae14c.
  Established. The word read at entry r0 +0x4 becomes the first candidate for the call to 0x18bf0
  (`ldr` at 0xae0f0, `bl` at 0xae0fc). That call's next two candidates are 0x80047504 and 0x0 (`bl`
  at 0xae0fc). `bge` takes its nonnegative-result path to the return, while negative fall-through
  reaches calls to 0x188b4, 0x1e71c and 0x21010 (`bge` at 0xae108, `bl` at 0xae10c, `bl` at 0xae118,
  `bl` at 0xae138). The wire-level break behavior is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae0e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xae0e8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xae0ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae0f0 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xae0f4 | load | 4 | `abs 0xae140` | `-` |
| `ldr` at 0xae114 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xae120 | load | 4 | `abs 0xae144` | `-` |
| `ldr` at 0xae124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae128 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0xae12c | load | 4 | `abs 0xae148` | `-` |
| `ldr` at 0xae130 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldmdb` at 0xae13c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae13c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae13c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae0fc | `(unknown callee) 0x18bf0` | `[r0+4]` | `0x80047504` | `0x0` | `r0` |
| `bl` at 0xae10c | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xae118 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xae138 | `(unknown callee) 0x21010` | `[0x116928]` | `0x108330` | `[[sp-20]+8]` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xae108 | `ret(0x18bf0) >= 0x0 (signed, 32-bit)` | `ret(0x18bf0) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae13c | `r0` | `?` |

## `uartShowStatus`

- `uartShowStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae14c to 0xae248.
  Established. After the call to 0xae248, `bne` takes its nonzero-result path into the body (`bl` at
  0xae16c, `bne` at 0xae180). Halfword loads at `[sp-44] +0x9c`, +0x9e, +0xa0 and +0x104 feed
  four-byte outgoing stores (`ldrh` at 0xae1e8, `ldrh` at 0xae1f4, `ldrh` at 0xae200, `ldrh` at
  0xae218, `str` at 0xae1ec, `str` at 0xae1f8, `str` at 0xae204, `str` at 0xae21c). The final call
  to 0x18c74 has candidates `[sp-40]`, 0x10835c, `[sp-36]` and `[[sp-44]+44]` (`bl` at 0xae230). The
  counter and formatting meanings of those values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae15c | store | 4 | `sp -0x20` | `r0` |
| `str` at 0xae160 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0xae164 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0xae168 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xae174 | store | 4 | `sp -0x2c` | `ret(0xae248)` |
| `ldr` at 0xae178 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xae188 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xae190 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xae1a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xae1a8 | load | 4 | `[sp-44] +0x98` | `-` |
| `ldr` at 0xae1b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xae1c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xae1cc | load | 4 | `[sp-44] +0x100` | `-` |
| `ldr` at 0xae1d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xae1dc | store | 4 | `sp -0x50` | `ret(0xae2fc)` |
| `str` at 0xae1e0 | store | 4 | `sp -0x4c` | `ret(0xae320)` |
| `ldr` at 0xae1e4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xae1e8 | load | 2 | `[sp-44] +0x9c` | `-` |
| `str` at 0xae1ec | store | 4 | `sp -0x48` | `[[sp-44]+156]:2` |
| `ldr` at 0xae1f0 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xae1f4 | load | 2 | `[sp-44] +0x9e` | `-` |
| `str` at 0xae1f8 | store | 4 | `sp -0x44` | `[[sp-44]+158]:2` |
| `ldr` at 0xae1fc | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xae200 | load | 2 | `[sp-44] +0xa0` | `-` |
| `str` at 0xae204 | store | 4 | `sp -0x40` | `[[sp-44]+160]:2` |
| `str` at 0xae208 | store | 4 | `sp -0x3c` | `ret(0xae2fc)` |
| `str` at 0xae20c | store | 4 | `sp -0x38` | `ret(0xae320)` |
| `ldr` at 0xae210 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xae218 | load | 2 | `[sp-44] +0x104` | `-` |
| `str` at 0xae21c | store | 4 | `sp -0x34` | `[[sp-44]+260]:2` |
| `ldr` at 0xae220 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xae224 | load | 4 | `abs 0xae244` | `-` |
| `ldr` at 0xae228 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae22c | load | 4 | `[sp-44] +0x2c` | `-` |
| `str` at 0xae238 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xae23c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xae240 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xae240 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae240 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xae240 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae240 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae240 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae16c | `(unknown callee) 0xae248` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xae19c | `(unknown callee) 0xae2fc` | `ret(0xae248)+60` | `?` | `?` | `ret(0xae248)+60` |
| `bl` at 0xae1ac | `(unknown callee) 0xae320` | `[[sp-44]+152]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xae1c0 | `(unknown callee) 0xae2fc` | `[sp-44]+164` | `?` | `?` | `[sp-44]+164` |
| `bl` at 0xae1d0 | `(unknown callee) 0xae320` | `[[sp-44]+256]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xae230 | `(unknown callee) 0x18c74` | `[sp-40]` | `0x10835c` | `[sp-36]` | `[[sp-44]+44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xae180 | `ret(0xae248) != 0x0` | `ret(0xae248) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae240 | `r0` | `[sp-48]` |

## `nvramRead`

- `nvramRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1dfd0 to 0x1e040.
  Established. `ble` takes the path past the overwrite of sp -0x1c when `(r0 + r2) <= 0x1fff` signed
  (`ble` at 0x1e000, `str` at 0x1e00c). On the other path that word's stored value is unknown (`str`
  at 0x1e00c). The single call targets 0x18ae8 with four unknown candidates (`bl` at 0x1e028). The
  return carries `[sp-28]` (`ldmdb` at 0x1e034). The exact storage mapping and transfer arguments
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1dfe0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1dfe4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1dfe8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1dfec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1dff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1dff8 | load | 4 | `abs 0x1e038` | `-` |
| `ldr` at 0x1e004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1e00c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1e010 | load | 4 | `abs 0x1e03c` | `-` |
| `ldr` at 0x1e014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1e01c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1e024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1e02c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x1e034 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1e034 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1e034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e028 | `(unknown callee) 0x18ae8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x1e000 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1e034 | `r0` | `[sp-28]` |

## `nvramWrite`

- `nvramWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e040 to 0x1e0b0.
  Established. Entry r0, r1 and r2 are saved in word slots at sp -0x14, sp -0x18 and sp -0x1c (`str`
  at 0x1e050, `str` at 0x1e054, `str` at 0x1e058). `ble` takes the path past the unknown-valued
  overwrite of the third slot when `(r0 + r2) <= 0x1fff` signed (`ble` at 0x1e070, `str` at
  0x1e07c). The call to 0x18ae8 leaves all four candidates unknown (`bl` at 0x1e098); its
  surrounding arithmetic does not establish a physical NVRAM address.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1e050 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1e054 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1e058 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1e05c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1e060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1e068 | load | 4 | `abs 0x1e0a8` | `-` |
| `ldr` at 0x1e074 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1e07c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1e080 | load | 4 | `abs 0x1e0ac` | `-` |
| `ldr` at 0x1e084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1e090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1e094 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1e09c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x1e0a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1e0a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1e0a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e098 | `(unknown callee) 0x18ae8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x1e070 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1e0a4 | `r0` | `[sp-28]` |

## `nvramLogReset`

- `nvramLogReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1d638 to 0x1d6d8.
  Established. Separate word reads from `[0x11694c] +0x784` supply stores at that loaded base +0x78c
  and +0x790 (`ldr` at 0x1d654, `str` at 0x1d658, `ldr` at 0x1d66c, `str` at 0x1d670). The byte
  store at `[0x11694c] +0x7a3` contains `(ret(0x1eddc) & 0xff)` (`strb` at 0x1d698). The call to
  0x18ae8 has candidates `[0x11694c]+1956`, `[0x11694c]+1920`, 0x24 and `[0x11694c]+1920` (`bl` at
  0x1d6c4). The pointed storage's physical identity and log structure remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1d644 | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d648 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d64c | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d650 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d654 | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1d658 | store | 4 | `[0x11694c] +0x78c` | `[[0x11694c]+1924]` |
| `ldr` at 0x1d65c | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d660 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d664 | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d668 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d66c | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1d670 | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1924]` |
| `ldr` at 0x1d674 | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d678 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d67c | load | 4 | `abs 0x1d6d4` | `-` |
| `ldr` at 0x1d680 | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d684 | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x1d698 | store | 1 | `[0x11694c] +0x7a3` | `(ret(0x1eddc) & 0xff)` |
| `ldr` at 0x1d69c | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d6a0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d6ac | load | 4 | `abs 0x1d6d0` | `-` |
| `ldr` at 0x1d6b0 | load | 4 | `abs 0x11694c` | `-` |
| `ldmdb` at 0x1d6cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1d6cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1d6cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1d6cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1d6cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1d694 | `(unknown callee) 0x1eddc` | `[0x11694c]+1920` | `0x24` | `[0x11694c]` | `[0x11694c]+1920` |
| `bl` at 0x1d6c4 | `(unknown callee) 0x18ae8` | `[0x11694c]+1956` | `[0x11694c]+1920` | `0x24` | `[0x11694c]+1920` |
| `bl` at 0x1d6c8 | `(unknown callee) 0x19390` | `ret(0x18ae8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1d6cc | `r0` | `ret(0x19390)` |

## `nvramLogFinsert`

- `nvramLogFinsert`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1dba4 to 0x1dbe8.
  Established. A word at sp -0x24 receives the stack expression sp-12 (`str` at 0x1dbbc). The call
  to 0x18764 has candidates sp-156, entry r0, sp-12 and sp-156 (`bl` at 0x1dbd0). A second call
  targets 0x1d6d8 with sp-156 as its first candidate (`bl` at 0x1dbdc). The formatted-record purpose
  suggested by the name remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1dbbc | store | 4 | `sp -0x24` | `sp-12` |
| `ldr` at 0x1dbc8 | load | 4 | `sp -0x10` | `-` |
| `ldr` at 0x1dbcc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x1dbe4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x1dbe4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x1dbe4 | load | 4 | `sp -0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1dbd0 | `(unknown callee) 0x18764` | `sp-156` | `r0` | `sp-12` | `sp-156` |
| `bl` at 0x1dbdc | `(unknown callee) 0x1d6d8` | `sp-156` | `?` | `?` | `sp-156` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1dbe4 | `r0` | `?` |

## `powerGetCfg`

- `powerGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39994 to 0x399d0.
  Established. The routine loads a word from 0x281120 and stores `[0x281120]` at entry r0 +0x0
  (`ldr` at 0x399b0, `str` at 0x399b4); its separate word load from 0x281100 supplies r0 expression
  `[0x281100]` at return (`ldr` at 0x399bc, `ldmdb` at 0x399c4). The pointed configuration and count
  meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x399a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x399a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x399ac | load | 4 | `abs 0x399c8` | `-` |
| `ldr` at 0x399b0 | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x399b4 | store | 4 | `r0 +0x0` | `[0x281120]` |
| `ldr` at 0x399b8 | load | 4 | `abs 0x399cc` | `-` |
| `ldr` at 0x399bc | load | 4 | `abs 0x281100` | `-` |
| `ldmdb` at 0x399c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x399c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x399c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x399c4 | `r0` | `[0x281100]` |
