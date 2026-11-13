# ARM EEPROM and expander access in 1.40.5

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
`eepromReadData`, `eepromFreeData` and `eepromStatus` have unresolved computed transfers. Their access, call and return lists cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/eeprom-expander-access-facts.txt) retain all tool sections.

## `eepromInit`

- `eepromInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1f18 to 0xd21b4.
  Established. The absolute word at `0x2adf88` receives `0x1` (`str` at 0xd1f84). Another absolute
  word at `0x2adf84` receives `ret(0x1b454)` (`str` at 0xd1fac). Included four-byte assignments at
  `[sp-44] +0x8` and `[sp-44] +0xc` retain `ret(0x1b1b4)` and `ret(0x1b454)` (`str` at 0xd2024,
  `str` at 0xd20f0). The unnamed `0x1b4cc` call carries `[[sp-44]+8]`, `0x1`, `[[sp-44]+4]`,
  `[sp-44]` (`bl` at 0xd20d4). Allocation and device effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1f28 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd1f2c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd1f30 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd1f34 | load | 4 | `abs 0xd2190` | `-` |
| `ldr` at 0xd1f38 | load | 4 | `abs 0x2adf84` | `-` |
| `ldr` at 0xd1f44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1f50 | load | 4 | `abs 0xd2194` | `-` |
| `str` at 0xd1f54 | store | 4 | `sp -0x3c` | `0x134cd0` |
| `ldr` at 0xd1f5c | load | 4 | `abs 0xd2198` | `-` |
| `ldr` at 0xd1f6c | load | 4 | `abs 0xd219c` | `-` |
| `ldr` at 0xd1f70 | load | 4 | `abs 0x2adf88` | `-` |
| `ldr` at 0xd1f7c | load | 4 | `abs 0xd219c` | `-` |
| `str` at 0xd1f84 | store | 4 | `abs 0x2adf88` | `0x1` |
| `ldr` at 0xd1f88 | load | 4 | `abs 0xd219c` | `-` |
| `ldr` at 0xd1f8c | load | 4 | `abs 0x2adf88` | `-` |
| `str` at 0xd1f94 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd1f98 | load | 4 | `abs 0xd2190` | `-` |
| `ldr` at 0xd1f9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1fac | store | 4 | `abs 0x2adf84` | `ret(0x1b454)` |
| `ldr` at 0xd1fb0 | load | 4 | `abs 0x2adf84` | `-` |
| `ldr` at 0xd1fbc | load | 4 | `abs 0xd21a0` | `-` |
| `str` at 0xd1fc0 | store | 4 | `sp -0x3c` | `0x134d10` |
| `ldr` at 0xd1fc4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1fc8 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xd1fd0 | load | 4 | `abs 0xd2198` | `-` |
| `ldr` at 0xd1fe0 | load | 4 | `abs 0xd2190` | `-` |
| `ldr` at 0xd1fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1fec | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd1ff4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd1ff8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1ffc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2000 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0xd2004 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2008 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd200c | store | 4 | `[sp-44] +0x4` | `[sp-32]` |
| `ldr` at 0xd2010 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2014 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2018 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0xd2024 | store | 4 | `[sp-44] +0x8` | `ret(0x1b1b4)` |
| `ldr` at 0xd2028 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0xd2034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd203c | load | 4 | `abs 0xd21a4` | `-` |
| `ldr` at 0xd2040 | load | 4 | `abs 0xd21a8` | `-` |
| `str` at 0xd206c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd2070 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2078 | load | 4 | `abs 0xd21a4` | `-` |
| `ldr` at 0xd207c | load | 4 | `abs 0xd21a8` | `-` |
| `ldr` at 0xd2088 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2098 | load | 4 | `abs 0xd21ac` | `-` |
| `str` at 0xd209c | store | 4 | `(unknown)` | `0x134d54` |
| `str` at 0xd20a0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd20a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd20a8 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd20b0 | load | 4 | `abs 0xd2198` | `-` |
| `ldr` at 0xd20c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd20c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd20c8 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0xd20d0 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0xd20d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd20dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd20e0 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0xd20f0 | store | 4 | `[sp-44] +0xc` | `ret(0x1b454)` |
| `ldr` at 0xd20f4 | load | 4 | `[sp-44] +0xc` | `-` |
| `ldr` at 0xd2100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2108 | load | 4 | `abs 0xd21a4` | `-` |
| `ldr` at 0xd210c | load | 4 | `abs 0xd21a8` | `-` |
| `str` at 0xd2138 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd213c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2144 | load | 4 | `abs 0xd21a4` | `-` |
| `ldr` at 0xd2148 | load | 4 | `abs 0xd21a8` | `-` |
| `ldr` at 0xd2154 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2164 | load | 4 | `abs 0xd21b0` | `-` |
| `str` at 0xd2168 | store | 4 | `(unknown)` | `0x134d90` |
| `str` at 0xd216c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd2170 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd2174 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd217c | load | 4 | `abs 0xd2198` | `-` |
| `ldmdb` at 0xd218c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd218c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd218c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd218c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1f68 | `(unknown callee) 0x21eb8` | `0x9` | `0x134cf4` | `0xe3` | `0x0` |
| `bl` at 0xd1fa4 | `(unknown callee) 0x1b454` | `[sp-40]` | `0x1` | `0x2adf88` | `?` |
| `bl` at 0xd1fdc | `(unknown callee) 0x21eb8` | `0x9` | `0x134cf4` | `0xed` | `0x0` |
| `bl` at 0xd201c | `(unknown callee) 0x1b1b4` | `[[sp-44]+4]` | `0x2adf84` | `[sp-44]` | `[sp-44]` |
| `bl` at 0xd2044 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134d48` | `0x134d50` |
| `bl` at 0xd2050 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd2080 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134d48` | `0x134d50` |
| `bl` at 0xd2090 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd20bc | `(unknown callee) 0x21eb8` | `0x9` | `0x134cf4` | `0xf9` | `0x0` |
| `bl` at 0xd20d4 | `(unknown callee) 0x1b4cc` | `[[sp-44]+8]` | `0x1` | `[[sp-44]+4]` | `[sp-44]` |
| `bl` at 0xd20e8 | `(unknown callee) 0x1b454` | `[[sp-44]+4]` | `0x78` | `?` | `[sp-44]` |
| `bl` at 0xd2110 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134d48` | `0x134d50` |
| `bl` at 0xd211c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd214c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134d48` | `0x134d50` |
| `bl` at 0xd215c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd2188 | `(unknown callee) 0x21eb8` | `0x9` | `0x134cf4` | `0xfe` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd1f40 | `[0x2adf84] == 0x0` | `[0x2adf84] != 0x0` |
| `bne` at 0xd1f4c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd1f78 | `[0x2adf88] != 0x0` | `[0x2adf88] == 0x0` |
| `bne` at 0xd1fb8 | `[0x2adf84] != 0x0` | `[0x2adf84] == 0x0` |
| `bne` at 0xd2030 | `[[sp-44]+8] != 0x0` | `[[sp-44]+8] == 0x0` |
| `bne` at 0xd20fc | `[[sp-44]+12] != 0x0` | `[[sp-44]+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd218c | `r0` | `?` |

## `eepromProbe`

- `eepromProbe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd224c to 0xd23e4.
  Established. The unnamed `0xd27e4` call carries `r0`, `(r1 & 0xff)`, unknown, `(r1 & 0xff)` (`bl`
  at 0xd2294); its nonzero result takes `0xd22b0`, while zero falls through (`bne` at 0xd22a0). A
  later byte assignment clears `([sp-29]:1 + [[sp-36]+8]) +0x0` (`strb` at 0xd23b4). Probe semantics
  beyond these effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd225c | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xd2264 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xd2268 | load | 4 | `abs 0xd23c8` | `-` |
| `ldr` at 0xd226c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2274 | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd227c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xd2284 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd2288 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd228c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd22a4 | load | 4 | `abs 0xd23cc` | `-` |
| `str` at 0xd22a8 | store | 4 | `sp -0x38` | `0xfffffbb4` |
| `ldrb` at 0xd22b0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd22bc | store | 4 | `sp -0x48` | `0x8` |
| `ldr` at 0xd22c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd22d8 | store | 4 | `sp -0x28` | `ret(0xd285c)` |
| `ldr` at 0xd22dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd22e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd22f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd22fc | load | 4 | `abs 0xd23d0` | `-` |
| `ldr` at 0xd2300 | load | 4 | `abs 0xd23d4` | `-` |
| `str` at 0xd232c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd2330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2338 | load | 4 | `abs 0xd23d0` | `-` |
| `ldr` at 0xd233c | load | 4 | `abs 0xd23d4` | `-` |
| `ldr` at 0xd2348 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xd2358 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd235c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd236c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd2378 | load | 4 | `abs 0xd23d8` | `-` |
| `str` at 0xd237c | store | 4 | `(unknown)` | `0x134de8` |
| `str` at 0xd2380 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0xd2384 | store | 4 | `(unknown)` | `ret(0xd2c14)` |
| `str` at 0xd2388 | store | 4 | `(unknown)` | `ret(0x21704)` |
| `ldr` at 0xd2390 | load | 4 | `abs 0xd23dc` | `-` |
| `ldr` at 0xd2394 | load | 4 | `abs 0xd23e0` | `-` |
| `ldr` at 0xd23a0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd23a4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd23a8 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd23b4 | store | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `0x0` |
| `ldr` at 0xd23b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd23bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd23c0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xd23c4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd23c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd23c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd23c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd23c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2294 | `(unknown callee) 0xd27e4` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd22d0 | `(unknown callee) 0xd285c` | `[sp-28]` | `[sp-29]:1` | `0x0` | `sp-48` |
| `bl` at 0xd2304 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x134d48` | `0x134d50` |
| `bl` at 0xd2310 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd2340 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x134d48` | `0x134d50` |
| `bl` at 0xd2350 | `(unknown callee) 0x1b598` | `[sp-52]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd2364 | `(unknown callee) 0xd2c14` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xd2370 | `(unknown callee) 0x21704` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd239c | `(unknown callee) 0x21eb8` | `0x1` | `0x134cf4` | `0x11d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd22a0 | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `beq` at 0xd22e4 | `ret(0xd285c) == 0x0` | `ret(0xd285c) != 0x0` |
| `beq` at 0xd22f0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd23c4 | `r0` | `?` |

## `eepromNotPresent`

- `eepromNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd21b4 to 0xd224c.
  Established. The signed branch takes `(r1 & 0xff) >= [[sp-28]+4]` to `0xd2218`, and falls
  through below the bound (`bge` at 0xd21f8). The fall-through path clears a byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`strb` at 0xd2210). The taken path includes the unnamed `0x21eb8` call with
  `0x6`, `0x134cf4`, `0x10c`, `0x0` (`bl` at 0xd2238). Presence-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd21c4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd21cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd21d0 | load | 4 | `abs 0xd2240` | `-` |
| `ldr` at 0xd21d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd21dc | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd21e4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd21e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd21ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd21f0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd21fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd2200 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd2204 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd2210 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xd2218 | load | 4 | `abs 0xd2244` | `-` |
| `str` at 0xd221c | store | 4 | `sp -0x24` | `0x134dcc` |
| `ldrb` at 0xd2220 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd2224 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0xd222c | load | 4 | `abs 0xd2248` | `-` |
| `ldmdb` at 0xd223c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd223c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd223c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2238 | `(unknown callee) 0x21eb8` | `0x6` | `0x134cf4` | `0x10c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd21f8 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd223c | `r0` | `?` |

## `eepromReadAllData`

- `eepromReadAllData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd23e4 to 0xd247c.
  Established. A byte stack assignment clears `sp -0x19` (`strb` at 0xd2414). The loop branch takes
  `lt flags condition (operands unknown)` to `0xd2430`, and falls through when that condition is
  false (`blt` at 0xd2428). The unnamed `0xd247c` call has `[sp-20]` followed by three unknown
  candidates (`bl` at 0xd2460). Record traversal details remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd23f4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd23f8 | load | 4 | `abs 0xd2478` | `-` |
| `ldr` at 0xd23fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2404 | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd240c | store | 4 | `sp -0x18` | `?` |
| `strb` at 0xd2414 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xd2418 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd241c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2420 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd2430 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd2434 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd2438 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd244c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xd2454 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd2464 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd246c | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0xd2474 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2474 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2474 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2460 | `(unknown callee) 0xd247c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd2428 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2474 | `r0` | `?` |

## `eepromReadData`

- `eepromReadData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd247c to 0xd25a8.
  Established. The byte at `[sp-32] +0x4` is read (`ldrb` at 0xd24cc) and its value `[[sp-32]+4]:1`
  is stored as a word at `[sp-28] +0x0` (`str` at 0xd24d0). The unnamed `0x1b4cc` call carries
  `[sp-28]`, `0x0`, `0x78`, `ret(0xd2d18)` (`bl` at 0xd24c0). The selector takes unsigned
  `[[sp-28]]-1 <= 0x3` to an unresolved destination, and falls through above the bound (`ldrls` at
  0xd2518). Selector-specific read effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd248c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd2494 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xd2498 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd249c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd24a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd24b0 | store | 4 | `sp -0x20` | `ret(0xd2d18)` |
| `ldr` at 0xd24b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd24c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd24c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd24cc | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0xd24d0 | store | 4 | `[sp-28] +0x0` | `[[sp-32]+4]:1` |
| `ldrb` at 0xd24d4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd24e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd24e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd24fc | load | 4 | `abs 0xd25a0` | `-` |
| `str` at 0xd2500 | store | 4 | `sp -0x24` | `0xfffffbb4` |
| `ldr` at 0xd2508 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd250c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd2518 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2590 | load | 4 | `abs 0xd25a4` | `-` |
| `str` at 0xd2594 | store | 4 | `sp -0x24` | `0xfffffbb1` |
| `ldr` at 0xd2598 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd259c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd259c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd259c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd24a8 | `(unknown callee) 0xd2d18` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd24c0 | `(unknown callee) 0x1b4cc` | `[sp-28]` | `0x0` | `0x78` | `ret(0xd2d18)` |
| `bl` at 0xd24ec | `(unknown callee) 0xd27e4` | `[sp-20]` | `[sp-21]:1` | `[sp-28]` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xd24dc | `[sp-21]:1 > 0xf4 (unsigned, 32-bit)` | `[sp-21]:1 <= 0xf4 (unsigned, 32-bit)` |
| `bne` at 0xd24f8 | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `ldrls` at 0xd2518 | `[[sp-28]]-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-28]]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd259c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd2518).

## `eepromFreeData`

- `eepromFreeData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd25a8 to 0xd273c.
  Established. The entry word at `r0 +0x0` is read (`ldr` at 0xd25c0). The computed transfer takes
  unsigned `[r0]-1 <= 0x3` to an unresolved target, and falls through above that bound (`ldrls` at
  0xd25cc). The known call to unnamed `0x1b4cc` carries `r0`, `0x0`, `0x78`, `[r0]-1` (`bl` at
  0xd2734). Ownership and deallocation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd25b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd25bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd25c0 | load | 4 | `r0 +0x0` | `-` |
| `ldrls` at 0xd25cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2728 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd2738 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2738 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2738 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2734 | `(unknown callee) 0x1b4cc` | `r0` | `0x0` | `0x78` | `[r0]-1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xd25cc | `[r0]-1 <= 0x3 (unsigned, 32-bit)` | `[r0]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2738 | `r0` | `ret(0x1b4cc)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd25cc).

## `eepromGetCfg`

- `eepromGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd273c to 0xd2790.
  Established. A word read uses absolute `0x2adf84` (`ldr` at 0xd2760). The word at `[sp-28] +0x0`
  is read (`ldr` at 0xd2774) and assigned at `r1 +0x0` with value `[[sp-28]]` (`str` at 0xd2778).
  The return reports `[[sp-28]+4]` (`ldmdb` at 0xd2788). Configuration field meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd274c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd2750 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd2754 | load | 4 | `abs 0xd278c` | `-` |
| `ldr` at 0xd2758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2760 | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd2768 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd276c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2774 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd2778 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd277c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2780 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd2788 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2788 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2788 | `r0` | `[[sp-28]+4]` |

## `eepromGetData`

- `eepromGetData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2790 to 0xd27e4.
  Established. The word at `[sp-28] +0xc` is read (`ldr` at 0xd27c8); its value `[[sp-28]+12]` is
  assigned as a word at `r1 +0x0` (`str` at 0xd27cc). The later word read uses `[sp-28] +0x4` (`ldr`
  at 0xd27d4). Data ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd27a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd27a4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd27a8 | load | 4 | `abs 0xd27e0` | `-` |
| `ldr` at 0xd27ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd27b4 | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd27bc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd27c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd27c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd27c8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd27cc | store | 4 | `r1 +0x0` | `[[sp-28]+12]` |
| `ldr` at 0xd27d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd27d4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd27dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd27dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd27dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd27dc | `r0` | `[[sp-28]+4]` |

## `eepromIsPresent`

- `eepromIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd27e4 to 0xd285c.
  Established. The signed comparison takes `(r1 & 0xff) < [[sp-28]+4]` to `0xd2838` and falls
  through at or above the bound (`blt` at 0xd2828). The taken path includes a byte read at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`ldrb` at 0xd2848). Presence encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd27f4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd27fc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd2800 | load | 4 | `abs 0xd2858` | `-` |
| `ldr` at 0xd2804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd280c | load | 4 | `abs 0x2adf84` | `-` |
| `str` at 0xd2814 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd2818 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd281c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2820 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd2830 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd2838 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd283c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd2840 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd2848 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xd284c | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xd2850 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd2854 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2854 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd2828 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2854 | `r0` | `?` |

## `eepromRead`

- `eepromRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd285c to 0xd2a54.
  Established. The halfword at `ret(0xd2d18) +0x2` is read (`ldrh` at 0xd28bc). The unsigned branch
  takes `([sp-24]:2 + [sp-30]:2) <= [ret(0xd2d18)+2]:2` to `0xd2918`, and falls through above the
  bound (`bls` at 0xd28c4). A later halfword assignment at `sp -0x2c` writes `0x20` (`strh` at
  0xd2984). The unnamed `0xc878c` call carries unknown, `[sp-42]:1`, `[sp-28]`, `[sp-44]:2` (`bl` at
  0xd29f8). Transfer effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd286c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd2874 | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0xd2878 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd2880 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xd2884 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0xd2888 | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0xd2890 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd2894 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd2898 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd28a8 | store | 4 | `sp -0x24` | `ret(0xd2d18)` |
| `ldrh` at 0xd28ac | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd28b0 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd28b8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd28bc | load | 2 | `ret(0xd2d18) +0x2` | `-` |
| `ldr` at 0xd28c8 | load | 4 | `abs 0xd2a4c` | `-` |
| `str` at 0xd28cc | store | 4 | `sp -0x44` | `0x134e08` |
| `ldr` at 0xd28d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd28d4 | load | 4 | `ret(0xd2d18) +0x8` | `-` |
| `str` at 0xd28d8 | store | 4 | `sp -0x40` | `[ret(0xd2d18)+8]` |
| `ldrh` at 0xd28dc | load | 2 | `sp -0x18` | `-` |
| `str` at 0xd28e0 | store | 4 | `sp -0x3c` | `[sp-24]:2` |
| `ldrh` at 0xd28e4 | load | 2 | `sp -0x1e` | `-` |
| `str` at 0xd28e8 | store | 4 | `sp -0x38` | `[sp-30]:2` |
| `ldr` at 0xd28ec | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd28f0 | load | 2 | `ret(0xd2d18) +0x2` | `-` |
| `str` at 0xd28f4 | store | 4 | `sp -0x34` | `[ret(0xd2d18)+2]:2` |
| `ldr` at 0xd28fc | load | 4 | `abs 0xd2a50` | `-` |
| `str` at 0xd2910 | store | 4 | `sp -0x30` | `0x0` |
| `ldrh` at 0xd291c | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd2928 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xd2934 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xd2940 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd2944 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd2948 | store | 1 | `sp -0x2a` | `([sp-24]:1 & 0xff)` |
| `ldrb` at 0xd294c | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0xd2954 | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0xd2960 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd296c | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xd2970 | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldrh` at 0xd2974 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd2984 | store | 2 | `sp -0x2c` | `0x20` |
| `ldr` at 0xd2988 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd298c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xd2990 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd29b0 | store | 4 | `sp -0x28` | `ret(0xc8348)` |
| `ldr` at 0xd29b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd29c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd29c4 | store | 4 | `sp -0x30` | `ret(0xc8348)` |
| `ldr` at 0xd29cc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd29d0 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd29d4 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xd29e0 | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0xd29e4 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xd29f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2a00 | store | 4 | `sp -0x28` | `ret(0xc878c)` |
| `ldrh` at 0xd2a08 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd2a0c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd2a14 | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0xd2a18 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2a24 | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0xd2a28 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd2a2c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd2a34 | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0xd2a3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2a40 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd2a44 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd2a48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2a48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2a48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd28a0 | `(unknown callee) 0xd2d18` | `r0` | `(r1 & 0xff)` | `[sp]` | `(r1 & 0xff)` |
| `bl` at 0xd2908 | `(unknown callee) 0x21eb8` | `0x6` | `0x134cf4` | `0x1e4` | `0x0` |
| `bl` at 0xd2998 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0xd29a8 | `(unknown callee) 0xc8348` | `?` | `?` | `?` | `?` |
| `bl` at 0xd29f8 | `(unknown callee) 0xc878c` | `?` | `[sp-42]:1` | `[sp-28]` | `[sp-44]:2` |
| `bl` at 0xd2a04 | `(unknown callee) 0xc8488` | `ret(0xc878c)` | `?` | `?` | `ret(0xc878c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd28c4 | `([sp-24]:2 + [sp-30]:2) <= [ret(0xd2d18)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0xd2d18)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xd2924 | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0xd2930 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0xd2968 | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |
| `bls` at 0xd297c | `[sp-44]:2 <= 0x20 (unsigned, 32-bit)` | `[sp-44]:2 > 0x20 (unsigned, 32-bit)` |
| `beq` at 0xd29bc | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2a48 | `r0` | `?` |

## `eepromWrite`

- `eepromWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2a54 to 0xd2c14.
  Established. Included halfword stack assignments retain `(r2 & 0xffff)` at `sp -0x18` (`strh` at
  0xd2a7c) and `([sp] & 0xffff)` at `sp -0x1e` (`strh` at 0xd2a80). The unnamed `0xd4d64` call
  carries unknown, unknown, `[sp-42]:1`, `[sp-28]` (`bl` at 0xd2bb8). The later word assignment at
  `sp -0x1c` writes `([sp-28] + [sp-44]:2)` (`str` at 0xd2be0). Device write effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd2a64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd2a6c | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0xd2a70 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xd2a78 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xd2a7c | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0xd2a80 | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0xd2a88 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xd2a8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd2a90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd2aa0 | store | 4 | `sp -0x24` | `ret(0xd2d18)` |
| `ldrh` at 0xd2aa4 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd2aa8 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd2ab0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd2ab4 | load | 2 | `ret(0xd2d18) +0x2` | `-` |
| `ldr` at 0xd2ac0 | load | 4 | `abs 0xd2c08` | `-` |
| `str` at 0xd2ac4 | store | 4 | `sp -0x44` | `0x134e08` |
| `ldr` at 0xd2ac8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd2acc | load | 4 | `ret(0xd2d18) +0x8` | `-` |
| `str` at 0xd2ad0 | store | 4 | `sp -0x40` | `[ret(0xd2d18)+8]` |
| `ldrh` at 0xd2ad4 | load | 2 | `sp -0x18` | `-` |
| `str` at 0xd2ad8 | store | 4 | `sp -0x3c` | `[sp-24]:2` |
| `ldrh` at 0xd2adc | load | 2 | `sp -0x1e` | `-` |
| `str` at 0xd2ae0 | store | 4 | `sp -0x38` | `[sp-30]:2` |
| `ldr` at 0xd2ae4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xd2ae8 | load | 2 | `ret(0xd2d18) +0x2` | `-` |
| `str` at 0xd2aec | store | 4 | `sp -0x34` | `[ret(0xd2d18)+2]:2` |
| `ldr` at 0xd2af4 | load | 4 | `abs 0xd2c0c` | `-` |
| `ldr` at 0xd2af8 | load | 4 | `abs 0xd2c10` | `-` |
| `str` at 0xd2b08 | store | 4 | `sp -0x30` | `0x0` |
| `ldrh` at 0xd2b14 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xd2b20 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xd2b2c | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xd2b38 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd2b3c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd2b40 | store | 1 | `sp -0x2a` | `([sp-24]:1 & 0xff)` |
| `ldrb` at 0xd2b44 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0xd2b4c | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0xd2b58 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd2b64 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xd2b68 | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xd2b6c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd2b70 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xd2b74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2b88 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd2b8c | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd2b90 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xd2b9c | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0xd2ba0 | load | 2 | `sp -0x2c` | `-` |
| `str` at 0xd2ba4 | store | 4 | `sp -0x44` | `[sp-44]:2` |
| `ldr` at 0xd2bb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2bc0 | store | 4 | `sp -0x28` | `ret(0xd4d64)` |
| `ldrh` at 0xd2bc4 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xd2bc8 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd2bd0 | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0xd2bd4 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xd2bd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2be0 | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0xd2be4 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xd2be8 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xd2bf0 | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0xd2bf8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2bfc | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd2c00 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd2c04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2c04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2c04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2a98 | `(unknown callee) 0xd2d18` | `r0` | `(r1 & 0xff)` | `[sp]` | `(r1 & 0xff)` |
| `bl` at 0xd2b00 | `(unknown callee) 0x21eb8` | `0x6` | `0x134cf4` | `0x22d` | `0x0` |
| `bl` at 0xd2b7c | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0xd2bb8 | `(unknown callee) 0xd4d64` | `?` | `?` | `[sp-42]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd2abc | `([sp-24]:2 + [sp-30]:2) <= [ret(0xd2d18)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0xd2d18)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xd2b1c | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0xd2b28 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0xd2b60 | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2c04 | `r0` | `?` |

## `eepromName`

- `eepromName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2c14 to 0xd2c7c.
  Established. The unnamed `0xd2d18` call carries `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl` at
  0xd2c3c). A nonzero `[ret(0xd2d18)+8]` takes `0xd2c6c`, while zero falls through (`bne` at
  0xd2c60). The zero path assigns word `0x134e48` at `sp -0x20` (`str` at 0xd2c68). Label
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd2c24 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd2c2c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd2c30 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd2c34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd2c44 | store | 4 | `sp -0x1c` | `ret(0xd2d18)` |
| `ldr` at 0xd2c48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2c4c | load | 4 | `ret(0xd2d18) +0x8` | `-` |
| `str` at 0xd2c50 | store | 4 | `sp -0x20` | `[ret(0xd2d18)+8]` |
| `ldr` at 0xd2c54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2c58 | load | 4 | `ret(0xd2d18) +0x8` | `-` |
| `ldr` at 0xd2c64 | load | 4 | `abs 0xd2c78` | `-` |
| `str` at 0xd2c68 | store | 4 | `sp -0x20` | `0x134e48` |
| `ldr` at 0xd2c6c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd2c74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2c74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2c74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2c3c | `(unknown callee) 0xd2d18` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd2c60 | `[ret(0xd2d18)+8] != 0x0` | `[ret(0xd2d18)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2c74 | `r0` | `?` |

## `eepromStatus`

- `eepromStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2c7c to 0xd2d18.
  Established. The computed transfer takes unsigned `r0 <= 0x4` to an unresolved destination, and
  falls through when `r0 > 0x4` (`ldrls` at 0xd2c98). The known fallback assigns word `0x134e9c` at
  `sp -0x18` (`str` at 0xd2cf4) and reports `0x134e9c` at the return (`ldmdb` at 0xd2cfc).
  Status-label semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd2c8c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd2c90 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xd2c98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2cf0 | load | 4 | `abs 0xd2d14` | `-` |
| `str` at 0xd2cf4 | store | 4 | `sp -0x18` | `0x134e9c` |
| `ldr` at 0xd2cf8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd2cfc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2cfc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2cfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xd2c98 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2cfc | `r0` | `0x134e9c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd2c98).

## `mic74_IntSet`

- `mic74_IntSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaba4 to 0xdac0c.
  Established. The byte read at `r1 +0x1` supplies a candidate to the unnamed `0xc807c` call (`ldrb`
  at 0xdabc8, `bl` at 0xdabd4). The unnamed `0xc8874` call carries unknown, `[[sp-24]+2]:1`, `0x4`,
  `[sp-25]:1` (`bl` at 0xdabfc). The return reports `ret(0xc8874)` (`ldmdb` at 0xdac08). Interrupt
  register roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdabb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdabb8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdabc0 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xdabc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdabc8 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xdabcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdabe0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdabe4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xdabe8 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0xdac08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdac08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdac08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdabd4 | `(unknown callee) 0xc807c` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xdabfc | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdac08 | `r0` | `ret(0xc8874)` |

## `mic74_IntGet`

- `mic74_IntGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdac0c to 0xdac6c.
  Established. A byte read uses `[sp-24] +0x2` (`ldrb` at 0xdac48). The unnamed `0xc8998` call
  carries unknown, `[[sp-24]+2]:1`, `0x4`, `[sp-28]` (`bl` at 0xdac5c); its result is reported at
  the return (`ldmdb` at 0xdac68). Expander register semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdac1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdac20 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdac24 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdac28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdac2c | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xdac30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdac44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdac48 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xdac58 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdac68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdac68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdac68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdac38 | `(unknown callee) 0xc807c` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xdac5c | `(unknown callee) 0xc8998` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdac68 | `r0` | `ret(0xc8998)` |
