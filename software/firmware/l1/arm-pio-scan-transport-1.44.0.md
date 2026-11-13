# ARM PIO and scan transport paths in 1.44.0

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
`jmstr_scan` and `jmstr_move_to_idle` each have one unresolved computed transfer.
Their tables cover known edges only. The other thirteen routines report none.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/pio-scan-transport-facts.txt) retain all tool sections.

## `PIOWrite_Common`

- `PIOWrite_Common`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1d54 to 0xe24f8.
  Established. Included word stores clear absolute `0x2974b4` and later write `[0x2974b4]+1` (`str` at 0xe2314, `str` at 0xe235c).
  The unnamed `0xd0754` call carries `0x1`, `0x0`, `[sp+16]`, unknown (`bl` at 0xe1e50).
  A nonzero `ret(0xd0d10)` takes `0xe1f2c`, while zero falls through (`bne` at 0xe1f1c).
  Common PIO write transaction semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe1d64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe1d68 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe1d6c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe1d70 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe1d74 | load | 4 | `abs 0xe24c0` | `-` |
| `ldr` at 0xe1d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1d98 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1d9c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe1da0 | load | 4 | `abs 0xe24c0` | `-` |
| `ldr` at 0xe1da4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1dc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1dc8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe1dcc | load | 4 | `abs 0xe24c0` | `-` |
| `ldr` at 0xe1dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1df0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1df8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe1e00 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe1e04 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe1e08 | load | 4 | `abs 0xe24c0` | `-` |
| `ldr` at 0xe1e0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1e2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1e34 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe1e3c | load | 2 | `(unknown)` | `-` |
| `str` at 0xe1e40 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe1e4c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe1e54 | load | 4 | `abs 0xe24c0` | `-` |
| `ldr` at 0xe1e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1e78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1e80 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe1e88 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe1e98 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xe1ea0 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xe1eac | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xe1eb4 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xe1eb8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe1ebc | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xe1ec4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe1ec8 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xe1ecc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe1ed0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe1ed4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe1ee0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe1ee4 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xe1ee8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe1eec | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe1ef0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1ef4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe1ef8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1efc | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xe1f24 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe1f2c | load | 4 | `abs 0xe24c4` | `-` |
| `ldr` at 0xe1f30 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe1f3c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1f40 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe1f44 | load | 4 | `abs 0xe24c8` | `-` |
| `ldr` at 0xe1f48 | load | 4 | `abs 0xe24cc` | `-` |
| `ldr` at 0xe1f4c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe1f50 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1f58 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe1f5c | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xe1f60 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe1f64 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xe1f84 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xe1f88 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe1f8c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe1f90 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe1f94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1fa4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1fa8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xe1fac | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe1fb0 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe1fb4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1fb8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe1fbc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1fc0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xe1fe8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe1ff0 | load | 4 | `abs 0xe24c4` | `-` |
| `ldr` at 0xe1ff4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2000 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2004 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe2008 | load | 4 | `abs 0xe24d0` | `-` |
| `ldr` at 0xe200c | load | 4 | `abs 0xe24cc` | `-` |
| `ldr` at 0xe2010 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2014 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe201c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe2020 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xe2028 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe202c | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xe2030 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe2034 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe2038 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe2044 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe2048 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xe204c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2050 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe2054 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2058 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe205c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2060 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xe2088 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe2090 | load | 4 | `abs 0xe24c4` | `-` |
| `ldr` at 0xe2094 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe20a0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe20a4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe20a8 | load | 4 | `abs 0xe24d4` | `-` |
| `ldr` at 0xe20ac | load | 4 | `abs 0xe24cc` | `-` |
| `ldr` at 0xe20b0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe20b4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe20bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe20c8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe20cc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe20d4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe20d8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe20e0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe20e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe20e8 | load | 4 | `abs 0xe24d8` | `-` |
| `ldr` at 0xe20f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe20fc | load | 4 | `abs 0xe24d8` | `-` |
| `str` at 0xe2108 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe210c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2110 | load | 4 | `abs 0xe24dc` | `-` |
| `ldr` at 0xe211c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2124 | load | 4 | `abs 0xe24dc` | `-` |
| `str` at 0xe2130 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2134 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2138 | load | 4 | `abs 0xe24e0` | `-` |
| `ldr` at 0xe2144 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe214c | load | 4 | `abs 0xe24e0` | `-` |
| `str` at 0xe2158 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe215c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe216c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe2180 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2184 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2194 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe21a8 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xe21b4 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xe21b8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe21bc | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe21c0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe21c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe21d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe21d8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xe21dc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe21e0 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe21e4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe21e8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe21ec | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe21f0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xe2218 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe2220 | load | 4 | `abs 0xe24c4` | `-` |
| `ldr` at 0xe2224 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2230 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2234 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe2238 | load | 4 | `abs 0xe24e4` | `-` |
| `ldr` at 0xe223c | load | 4 | `abs 0xe24cc` | `-` |
| `ldr` at 0xe2240 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2244 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe224c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe225c | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xe2264 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe2268 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xe2270 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2274 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xe2278 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe227c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe2280 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe228c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe2290 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xe2294 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2298 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe229c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe22a0 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe22a4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe22a8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xe22d0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe22d8 | load | 4 | `abs 0xe24c4` | `-` |
| `ldr` at 0xe22dc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe22e8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe22ec | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe22f0 | load | 4 | `abs 0xe24c8` | `-` |
| `ldr` at 0xe22f4 | load | 4 | `abs 0xe24cc` | `-` |
| `ldr` at 0xe22f8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe22fc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2308 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe230c | load | 4 | `abs 0xe24e8` | `-` |
| `str` at 0xe2314 | store | 4 | `abs 0x2974b4` | `0x0` |
| `ldr` at 0xe2318 | load | 4 | `abs 0xe24e8` | `-` |
| `ldr` at 0xe231c | load | 4 | `abs 0x2974b4` | `-` |
| `ldr` at 0xe2328 | load | 4 | `abs 0xe24e8` | `-` |
| `ldr` at 0xe232c | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe2348 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe234c | load | 4 | `abs 0xe24e8` | `-` |
| `ldr` at 0xe2350 | load | 4 | `abs 0xe24e8` | `-` |
| `ldr` at 0xe2354 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe235c | store | 4 | `abs 0x2974b4` | `[0x2974b4]+1` |
| `str` at 0xe236c | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xe2370 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2374 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe2378 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe237c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2388 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2394 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe23a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe23a4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xe23a8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe23ac | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xe23b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe23b4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe23b8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe23bc | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xe23e4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe23ec | load | 4 | `abs 0xe24c4` | `-` |
| `ldr` at 0xe23f0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe23fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2400 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe2404 | load | 4 | `abs 0xe24ec` | `-` |
| `ldr` at 0xe2408 | load | 4 | `abs 0xe24cc` | `-` |
| `ldr` at 0xe240c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2410 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe2418 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2424 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe2438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2444 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe2458 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe245c | load | 4 | `abs 0xe24f0` | `-` |
| `str` at 0xe2468 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe2474 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe2478 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2480 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xe2488 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe2494 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe2498 | load | 4 | `abs 0xe24f4` | `-` |
| `str` at 0xe24a4 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe24b0 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xe24b4 | load | 4 | `sp -0x58` | `-` |
| `ldm` at 0xe24bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe24bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe24bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe1e50 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xe1edc | `(unknown callee) 0xdf0f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xe1f10 | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe1f54 | `(unknown callee) 0xdf464` | `0x122a50` | `0x122a60` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xe1f78 | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe1fa0 | `(unknown callee) 0xdf374` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xe1fd4 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2018 | `(unknown callee) 0xdf464` | `0x122af8` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe2040 | `(unknown callee) 0xdf0f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xe2074 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe20b8 | `(unknown callee) 0xdf464` | `0x122b08` | `0x122a60` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xe21d0 | `(unknown callee) 0xdf374` | `[sp-72]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xe2204 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2248 | `(unknown callee) 0xdf464` | `0x122ae8` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe2288 | `(unknown callee) 0xdf0f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xe22bc | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2300 | `(unknown callee) 0xdf464` | `0x122a50` | `0x122a60` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xe2384 | `(unknown callee) 0xdf374` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xe23d0 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2414 | `(unknown callee) 0xdf464` | `0x122a64` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe2428 | `(unknown callee) 0xd5a08` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xe2448 | `(unknown callee) 0xd5a08` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xe2460 | `(unknown callee) 0x19808` | `[sp+16]` | `0x122a7c` | `?` | `ret(0xd5a08)` |
| `bl` at 0xe249c | `(unknown callee) 0x19808` | `[sp+16]` | `0x122ab0` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe1e90 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xe1f1c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1f38 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe1fe0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1ffc | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2080 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe209c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2210 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe222c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2254 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xe22c8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe22e4 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe2324 | `[0x2974b4] > 0x2 (unsigned, 32-bit)` | `[0x2974b4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0xe2390 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xe239c | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xe23dc | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe23f8 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe2434 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `beq` at 0xe2454 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `bne` at 0xe2490 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe24bc | `r0` | `?` |

## `PIORead_Common`

- `PIORead_Common`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe24f8 to 0xe2b8c.
  Established. The unnamed `0xdf0f0` call carries `[sp-92]`, `[sp-100]`, `[sp-96]`, `sp-56` (`bl` at 0xe2698).
  A nonzero `ret(0xd0d10)` takes `0xe26e8`, while zero falls through (`bne` at 0xe26d8).
  The absolute word at `0x2974b4` receives `0x0` at an included assignment (`str` at 0xe2960).
  Common PIO read result framing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2508 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe250c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe2510 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe2514 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe2518 | load | 4 | `abs 0xe2b58` | `-` |
| `ldr` at 0xe251c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe253c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2540 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xe2544 | load | 4 | `abs 0xe2b58` | `-` |
| `ldr` at 0xe2548 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2568 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe256c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xe2570 | load | 4 | `abs 0xe2b58` | `-` |
| `ldr` at 0xe2574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2594 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe259c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe25a4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe25a8 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xe25ac | load | 4 | `abs 0xe2b58` | `-` |
| `ldr` at 0xe25b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe25d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe25d8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe25e0 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe25e4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xe25f0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe25f8 | load | 4 | `abs 0xe2b58` | `-` |
| `ldr` at 0xe25fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe261c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2624 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe262c | load | 1 | `(unknown)` | `-` |
| `str` at 0xe263c | store | 4 | `sp -0x50` | `0xd0000000` |
| `str` at 0xe2644 | store | 4 | `sp -0x54` | `0x50000000` |
| `str` at 0xe2650 | store | 4 | `sp -0x50` | `0x8000000` |
| `str` at 0xe2658 | store | 4 | `sp -0x54` | `0xf0000000` |
| `ldr` at 0xe265c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe2660 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe2668 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xe2670 | store | 4 | `[sp] +0x0` | `0x0` |
| `ldr` at 0xe2674 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe2678 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xe2680 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2684 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xe2688 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe268c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe2690 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xe269c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xe26a0 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xe26a4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe26a8 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xe26ac | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe26b0 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe26b4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe26b8 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xe26e0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xe26e8 | load | 4 | `abs 0xe2b5c` | `-` |
| `ldr` at 0xe26ec | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe26f8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe26fc | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xe2700 | load | 4 | `abs 0xe2b60` | `-` |
| `ldr` at 0xe2704 | load | 4 | `abs 0xe2b64` | `-` |
| `ldr` at 0xe2708 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe270c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe2714 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2720 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2724 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe272c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2730 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2734 | load | 4 | `abs 0xe2b68` | `-` |
| `ldr` at 0xe2740 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2748 | load | 4 | `abs 0xe2b68` | `-` |
| `str` at 0xe2754 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2758 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe275c | load | 4 | `abs 0xe2b6c` | `-` |
| `ldr` at 0xe2768 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2770 | load | 4 | `abs 0xe2b6c` | `-` |
| `str` at 0xe277c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2780 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2784 | load | 4 | `abs 0xe2b70` | `-` |
| `ldr` at 0xe2790 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2798 | load | 4 | `abs 0xe2b70` | `-` |
| `str` at 0xe27a4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe27a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe27b8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe27cc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe27d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe27e0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe27f4 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xe2800 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xe2804 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2808 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xe280c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe2810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2820 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2824 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xe2828 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe282c | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xe2830 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2834 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe2838 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe283c | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xe2864 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xe286c | load | 4 | `abs 0xe2b5c` | `-` |
| `ldr` at 0xe2870 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe287c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2880 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xe2884 | load | 4 | `abs 0xe2b74` | `-` |
| `ldr` at 0xe2888 | load | 4 | `abs 0xe2b64` | `-` |
| `ldr` at 0xe288c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2890 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe2898 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe28a8 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xe28b0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe28b4 | store | 4 | `sp -0x38` | `[sp-80]` |
| `ldr` at 0xe28bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe28c0 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xe28c4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe28c8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe28cc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xe28d8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xe28dc | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xe28e0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe28e4 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xe28e8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe28ec | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe28f0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe28f4 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xe291c | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xe2924 | load | 4 | `abs 0xe2b5c` | `-` |
| `ldr` at 0xe2928 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2934 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2938 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xe293c | load | 4 | `abs 0xe2b78` | `-` |
| `ldr` at 0xe2940 | load | 4 | `abs 0xe2b64` | `-` |
| `ldr` at 0xe2944 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe2948 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2954 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xe2958 | load | 4 | `abs 0xe2b7c` | `-` |
| `str` at 0xe2960 | store | 4 | `abs 0x2974b4` | `0x0` |
| `ldr` at 0xe2964 | load | 4 | `abs 0xe2b7c` | `-` |
| `ldr` at 0xe2968 | load | 4 | `abs 0x2974b4` | `-` |
| `ldr` at 0xe2974 | load | 4 | `abs 0xe2b7c` | `-` |
| `ldr` at 0xe2978 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe2994 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe2998 | load | 4 | `abs 0xe2b7c` | `-` |
| `ldr` at 0xe299c | load | 4 | `abs 0xe2b7c` | `-` |
| `ldr` at 0xe29a0 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe29a8 | store | 4 | `abs 0x2974b4` | `[0x2974b4]+1` |
| `str` at 0xe29b8 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xe29bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe29c0 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xe29c4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe29c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe29d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe29e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe29ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe29f0 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xe29f4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe29f8 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xe29fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2a00 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe2a04 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2a08 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xe2a30 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xe2a38 | load | 4 | `abs 0xe2b5c` | `-` |
| `ldr` at 0xe2a3c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2a48 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2a4c | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xe2a50 | load | 4 | `abs 0xe2b80` | `-` |
| `ldr` at 0xe2a54 | load | 4 | `abs 0xe2b64` | `-` |
| `ldr` at 0xe2a58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2a5c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe2a64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2a70 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe2a84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2a90 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe2aa4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe2aa8 | load | 4 | `abs 0xe2b84` | `-` |
| `str` at 0xe2ab4 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xe2ac4 | store | 4 | `sp -0x78` | `0x42` |
| `ldr` at 0xe2ac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2acc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe2af0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe2af4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe2af8 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xe2afc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe2b00 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe2b04 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xe2b0c | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xe2b10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2b18 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xe2b20 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe2b2c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe2b30 | load | 4 | `abs 0xe2b88` | `-` |
| `str` at 0xe2b3c | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xe2b48 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xe2b4c | load | 4 | `sp -0x68` | `-` |
| `ldm` at 0xe2b54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe2b54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe2b54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe25f4 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xe2698 | `(unknown callee) 0xdf0f0` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xe26cc | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe2710 | `(unknown callee) 0xdf464` | `0x122b18` | `0x122a60` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xe281c | `(unknown callee) 0xdf374` | `[sp-88]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xe2850 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2894 | `(unknown callee) 0xdf464` | `0x122b28` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe28d4 | `(unknown callee) 0xdf0f0` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xe2908 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe294c | `(unknown callee) 0xdf464` | `0x122b38` | `0x122a60` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xe29d0 | `(unknown callee) 0xdf374` | `[sp-88]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xe2a1c | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2a60 | `(unknown callee) 0xdf464` | `0x122b48` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe2a74 | `(unknown callee) 0xd5a08` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xe2a94 | `(unknown callee) 0xd5a08` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xe2aac | `(unknown callee) 0x19808` | `[sp+16]` | `0x122a7c` | `?` | `ret(0xd5a08)` |
| `bl` at 0xe2ad8 | `(unknown callee) 0xd5e9c` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xe2aec | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xe2b34 | `(unknown callee) 0x19808` | `[sp+16]` | `0x122b60` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe2634 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xe26d8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe26f4 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe285c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2878 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe28a0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xe2914 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2930 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe2970 | `[0x2974b4] > 0x2 (unsigned, 32-bit)` | `[0x2974b4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0xe29dc | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xe29e8 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xe2a28 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2a44 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe2a80 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `beq` at 0xe2aa0 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `bne` at 0xe2b28 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe2b54 | `r0` | `?` |

## `PIOWrite_PIC`

- `PIOWrite_PIC`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2b8c to 0xe32a4.
  Established. Included word stores clear absolute `0x2974b4` and later assign `[0x2974b4]+1` there (`str` at 0xe30c8, `str` at 0xe3110).
  The unnamed `0xd0754` call carries `0x1`, `0x0`, `[sp+16]`, unknown (`bl` at 0xe2c88).
  A nonzero `ret(0xd0d10)` takes `0xe2d00`, while zero falls through (`bne` at 0xe2cf0).
  PIC PIO write completion interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2b9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2ba0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe2ba4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe2ba8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe2bac | load | 4 | `abs 0xe3274` | `-` |
| `ldr` at 0xe2bb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2bd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2bd4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe2bd8 | load | 4 | `abs 0xe3274` | `-` |
| `ldr` at 0xe2bdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2bfc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2c00 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe2c04 | load | 4 | `abs 0xe3274` | `-` |
| `ldr` at 0xe2c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2c28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2c30 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe2c38 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe2c3c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe2c40 | load | 4 | `abs 0xe3274` | `-` |
| `ldr` at 0xe2c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2c6c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe2c74 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe2c78 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe2c84 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2c90 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xe2c98 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2c9c | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xe2ca0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2ca4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe2ca8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe2cb4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe2cb8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xe2cbc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2cc0 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2cc4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2cc8 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe2ccc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2cd0 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe2cf8 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe2d00 | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe2d04 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2d10 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2d14 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe2d18 | load | 4 | `abs 0xe327c` | `-` |
| `ldr` at 0xe2d1c | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe2d20 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2d24 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe2d2c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe2d30 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xe2d34 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe2d38 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xe2d58 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xe2d5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2d60 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2d64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe2d68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2d78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2d7c | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xe2d80 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2d84 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2d88 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2d8c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe2d90 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2d94 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe2dbc | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe2dc4 | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe2dc8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2dd4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2dd8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe2ddc | load | 4 | `abs 0xe3284` | `-` |
| `ldr` at 0xe2de0 | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe2de4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2de8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2df4 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xe2dfc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2e00 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xe2e04 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2e08 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe2e0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe2e18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe2e1c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xe2e20 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2e24 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2e28 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2e2c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe2e30 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2e34 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe2e5c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe2e64 | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe2e68 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2e74 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2e78 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe2e7c | load | 4 | `abs 0xe3288` | `-` |
| `ldr` at 0xe2e80 | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe2e84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2e88 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2e94 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xe2e98 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe2ea0 | store | 4 | `sp -0x38` | `0x1000000` |
| `ldr` at 0xe2ea4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2ea8 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xe2ec8 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xe2ecc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2ed0 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2ed4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe2ed8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2ee8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2eec | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xe2ef0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2ef4 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2ef8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2efc | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe2f00 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2f04 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe2f2c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe2f34 | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe2f38 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2f44 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2f48 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe2f4c | load | 4 | `abs 0xe328c` | `-` |
| `ldr` at 0xe2f50 | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe2f54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2f58 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2f64 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xe2f6c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2f70 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xe2f74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2f78 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe2f7c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe2f88 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe2f8c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xe2f90 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe2f94 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe2f98 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2f9c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe2fa0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe2fa4 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe2fcc | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe2fd4 | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe2fd8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe2fe4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe2fe8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe2fec | load | 4 | `abs 0xe3290` | `-` |
| `ldr` at 0xe2ff0 | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe2ff4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2ff8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe3000 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3010 | store | 4 | `sp -0x50` | `0x1` |
| `str` at 0xe301c | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xe3024 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3028 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xe302c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe3030 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe3034 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe3040 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe3044 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xe3048 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe304c | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe3050 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3054 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe3058 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe305c | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe3084 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe308c | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe3090 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe309c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe30a0 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe30a4 | load | 4 | `abs 0xe3288` | `-` |
| `ldr` at 0xe30a8 | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe30ac | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe30b0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe30bc | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe30c0 | load | 4 | `abs 0xe3294` | `-` |
| `str` at 0xe30c8 | store | 4 | `abs 0x2974b4` | `0x0` |
| `ldr` at 0xe30cc | load | 4 | `abs 0xe3294` | `-` |
| `ldr` at 0xe30d0 | load | 4 | `abs 0x2974b4` | `-` |
| `ldr` at 0xe30dc | load | 4 | `abs 0xe3294` | `-` |
| `ldr` at 0xe30e0 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe30fc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe3100 | load | 4 | `abs 0xe3294` | `-` |
| `ldr` at 0xe3104 | load | 4 | `abs 0xe3294` | `-` |
| `ldr` at 0xe3108 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe3110 | store | 4 | `abs 0x2974b4` | `[0x2974b4]+1` |
| `str` at 0xe3120 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xe3124 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3128 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe312c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe3130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe313c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3148 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe3154 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3158 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xe315c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3160 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xe3164 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3168 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xe316c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe3170 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xe3198 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xe31a0 | load | 4 | `abs 0xe3278` | `-` |
| `ldr` at 0xe31a4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe31b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe31b4 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xe31b8 | load | 4 | `abs 0xe3298` | `-` |
| `ldr` at 0xe31bc | load | 4 | `abs 0xe3280` | `-` |
| `ldr` at 0xe31c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe31c4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe31cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe31d8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe31ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe31f8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe320c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe3210 | load | 4 | `abs 0xe329c` | `-` |
| `str` at 0xe321c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xe3228 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe322c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3234 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xe323c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe3248 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe324c | load | 4 | `abs 0xe32a0` | `-` |
| `str` at 0xe3258 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xe3264 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xe3268 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0xe3270 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3270 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe2c88 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xe2cb0 | `(unknown callee) 0xdf0f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xe2ce4 | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe2d28 | `(unknown callee) 0xdf464` | `0x122b98` | `0x122a60` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xe2d4c | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe2d74 | `(unknown callee) 0xdf374` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xe2da8 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2dec | `(unknown callee) 0xdf464` | `0x122bac` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe2e14 | `(unknown callee) 0xdf0f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xe2e48 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2e8c | `(unknown callee) 0xdf464` | `0x122bc0` | `0x122a60` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xe2ebc | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe2ee4 | `(unknown callee) 0xdf374` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xe2f18 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2f5c | `(unknown callee) 0xdf464` | `0x122bd4` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe2f84 | `(unknown callee) 0xdf0f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xe2fb8 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe2ffc | `(unknown callee) 0xdf464` | `0x122be8` | `0x122a60` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xe303c | `(unknown callee) 0xdf0f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xe3070 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe30b4 | `(unknown callee) 0xdf464` | `0x122bc0` | `0x122a60` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xe3138 | `(unknown callee) 0xdf374` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xe3184 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe31c8 | `(unknown callee) 0xdf464` | `0x122bfc` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe31dc | `(unknown callee) 0xd5a08` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xe31fc | `(unknown callee) 0xd5a08` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xe3214 | `(unknown callee) 0x19808` | `[sp+16]` | `0x122c18` | `?` | `ret(0xd5a08)` |
| `bl` at 0xe3250 | `(unknown callee) 0x19808` | `[sp+16]` | `0x122c50` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe2cf0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2d0c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2db4 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2dd0 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2e54 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2e70 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2f24 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2f40 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe2fc4 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe2fe0 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe3008 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xe307c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe3098 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe30d8 | `[0x2974b4] > 0x1 (unsigned, 32-bit)` | `[0x2974b4] <= 0x1 (unsigned, 32-bit)` |
| `ble` at 0xe3144 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xe3150 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xe3190 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe31ac | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe31e8 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `beq` at 0xe3208 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `bne` at 0xe3244 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3270 | `r0` | `?` |

## `PIORead_PIC`

- `PIORead_PIC`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe32a4 to 0xe3a04.
  Established. The word targets `[sp+4] +0x0` and `[sp] +0x0` receive `0x0` at included stores (`str` at 0xe33b0, `str` at 0xe33b8).
  The unnamed `0xdf0f0` call carries `[sp-84]`, `[sp-92]`, `[sp-88]`, `sp-56` (`bl` at 0xe33e0).
  Nonzero `ret(0xd0d10)` takes `0xe3430`, while zero falls through (`bne` at 0xe3420).
  PIC PIO read data interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe32b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe32b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe32bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe32c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe32c4 | load | 4 | `abs 0xe39d4` | `-` |
| `ldr` at 0xe32c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe32e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe32ec | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe32f0 | load | 4 | `abs 0xe39d4` | `-` |
| `ldr` at 0xe32f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3314 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe3318 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe331c | load | 4 | `abs 0xe39d4` | `-` |
| `ldr` at 0xe3320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3340 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3348 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe3350 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe3354 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xe3358 | load | 4 | `abs 0xe39d4` | `-` |
| `ldr` at 0xe335c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe337c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3384 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe338c | load | 2 | `(unknown)` | `-` |
| `str` at 0xe3390 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xe339c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe33a4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe33a8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe33b0 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xe33b8 | store | 4 | `[sp] +0x0` | `0x0` |
| `str` at 0xe33c0 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xe33c8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe33cc | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xe33d0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe33d4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe33d8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe33e4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe33e8 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xe33ec | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe33f0 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe33f4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe33f8 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe33fc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe3400 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe3428 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe3430 | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe3434 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe3440 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3444 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe3448 | load | 4 | `abs 0xe39dc` | `-` |
| `ldr` at 0xe344c | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe3450 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe3454 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3460 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xe3464 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe3468 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xe3488 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xe348c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3490 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe3494 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe3498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe34a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe34ac | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xe34b0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe34b4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe34b8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe34bc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe34c0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe34c4 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe34ec | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe34f4 | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe34f8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe3504 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3508 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe350c | load | 4 | `abs 0xe39e4` | `-` |
| `ldr` at 0xe3510 | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe3514 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3518 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3524 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xe352c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3530 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xe3534 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe3538 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe353c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe3548 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe354c | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xe3550 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3554 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe3558 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe355c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe3560 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe3564 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe358c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe3594 | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe3598 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe35a4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe35a8 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe35ac | load | 4 | `abs 0xe39e8` | `-` |
| `ldr` at 0xe35b0 | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe35b4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe35b8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe35c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe35d0 | store | 4 | `sp -0x60` | `0x1` |
| `str` at 0xe35dc | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xe35e4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe35e8 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xe35ec | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe35f0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe35f4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe3600 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe3604 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xe3608 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe360c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe3610 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3614 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe3618 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe361c | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe3644 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe364c | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe3650 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe365c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3660 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe3664 | load | 4 | `abs 0xe39dc` | `-` |
| `ldr` at 0xe3668 | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe366c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe3670 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe367c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xe3680 | load | 4 | `abs 0xe39ec` | `-` |
| `str` at 0xe3688 | store | 4 | `abs 0x2974b4` | `0x0` |
| `ldr` at 0xe368c | load | 4 | `abs 0xe39ec` | `-` |
| `ldr` at 0xe3690 | load | 4 | `abs 0x2974b4` | `-` |
| `ldr` at 0xe369c | load | 4 | `abs 0xe39ec` | `-` |
| `ldr` at 0xe36a0 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe36bc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe36c0 | load | 4 | `abs 0xe39ec` | `-` |
| `ldr` at 0xe36c4 | load | 4 | `abs 0xe39ec` | `-` |
| `ldr` at 0xe36c8 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe36d0 | store | 4 | `abs 0x2974b4` | `[0x2974b4]+1` |
| `str` at 0xe36e0 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xe36e4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe36e8 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe36ec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe36f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe36fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3708 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe3714 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3718 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xe371c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3720 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe3724 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3728 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe372c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe3730 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe3758 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe3760 | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe3764 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe3770 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3774 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe3778 | load | 4 | `abs 0xe39f0` | `-` |
| `ldr` at 0xe377c | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe3780 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3784 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe378c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3798 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe37ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe37b8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe37cc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe37d0 | load | 4 | `abs 0xe39f4` | `-` |
| `str` at 0xe37dc | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xe37e8 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xe37f0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe37f4 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xe37f8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe37fc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xe3800 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe380c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe3810 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xe3814 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3818 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe381c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3820 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe3824 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe3828 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe3850 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe3858 | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe385c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe3868 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe386c | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe3870 | load | 4 | `abs 0xe39f8` | `-` |
| `ldr` at 0xe3874 | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe3878 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe387c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe38a0 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xe38a4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe38a8 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe38ac | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe38b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe38c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe38c4 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xe38c8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe38cc | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xe38d0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe38d4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe38d8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe38dc | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xe3904 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xe390c | load | 4 | `abs 0xe39d8` | `-` |
| `ldr` at 0xe3910 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe391c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe3920 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xe3924 | load | 4 | `abs 0xe39fc` | `-` |
| `ldr` at 0xe3928 | load | 4 | `abs 0xe39e0` | `-` |
| `ldr` at 0xe392c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3930 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe3940 | store | 4 | `sp -0x70` | `0x40` |
| `ldr` at 0xe3944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3948 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe396c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe3970 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe3974 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xe3978 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe397c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe3980 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xe3988 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xe398c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3994 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xe399c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe39a8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe39ac | load | 4 | `abs 0xe3a00` | `-` |
| `str` at 0xe39b8 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xe39c4 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xe39c8 | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0xe39d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe39d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe39d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe33a0 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xe33e0 | `(unknown callee) 0xdf0f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xe3414 | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe3458 | `(unknown callee) 0xdf464` | `0x122c8c` | `0x122a60` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xe347c | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe34a4 | `(unknown callee) 0xdf374` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xe34d8 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe351c | `(unknown callee) 0xdf464` | `0x122ca0` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe3544 | `(unknown callee) 0xdf0f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xe3578 | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe35bc | `(unknown callee) 0xdf464` | `0x122cb4` | `0x122a60` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xe35fc | `(unknown callee) 0xdf0f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xe3630 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe3674 | `(unknown callee) 0xdf464` | `0x122c8c` | `0x122a60` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xe36f8 | `(unknown callee) 0xdf374` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xe3744 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe3788 | `(unknown callee) 0xdf464` | `0x122cc8` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe379c | `(unknown callee) 0xd5a08` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xe37bc | `(unknown callee) 0xd5a08` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xe37d4 | `(unknown callee) 0x19808` | `[sp+16]` | `0x122c18` | `?` | `ret(0xd5a08)` |
| `bl` at 0xe3808 | `(unknown callee) 0xdf0f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xe383c | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe3880 | `(unknown callee) 0xdf464` | `0x122ce4` | `0x122a60` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xe3894 | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe38bc | `(unknown callee) 0xdf374` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xe38f0 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe3934 | `(unknown callee) 0xdf464` | `0x122cf8` | `0x122a60` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xe3954 | `(unknown callee) 0xd5e9c` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xe3968 | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xe39b0 | `(unknown callee) 0x19808` | `[sp+16]` | `0x122d0c` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3420 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe343c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe34e4 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe3500 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe3584 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe35a0 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe35c8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xe363c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe3658 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe3698 | `[0x2974b4] > 0x1 (unsigned, 32-bit)` | `[0x2974b4] <= 0x1 (unsigned, 32-bit)` |
| `ble` at 0xe3704 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xe3710 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xe3750 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe376c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe37a8 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `beq` at 0xe37c8 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `bne` at 0xe3848 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe3864 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe38fc | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe3918 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe39a4 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe39d0 | `r0` | `?` |

## `PIOWrite_Special`

- `PIOWrite_Special`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1618 to 0xe1d54.
  Established. A nonzero entry `r0` takes `0xe1724`, while zero falls through (`bne` at 0xe1710).
  Included word assignments put `0xd0000000` at `sp -0x40` and `0x50000000` at `sp -0x44` (`str` at 0xe1768, `str` at 0xe1770).
  The unnamed `0xdf0f0` call carries `[sp-76]`, `[sp-84]`, `[sp-80]`, `sp-56` (`bl` at 0xe17b8).
  Special PIO command encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe1628 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe162c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe1630 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe1634 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe1638 | load | 4 | `abs 0xe1d24` | `-` |
| `ldr` at 0xe163c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe165c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe1660 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe1664 | load | 4 | `abs 0xe1d24` | `-` |
| `ldr` at 0xe1668 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1688 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe168c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe1690 | load | 4 | `abs 0xe1d24` | `-` |
| `ldr` at 0xe1694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe16b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe16bc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe16c4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe16c8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe16cc | load | 4 | `abs 0xe1d24` | `-` |
| `ldr` at 0xe16d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe16f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe16f8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe1700 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe1704 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe1708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe171c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xe1724 | load | 4 | `abs 0xe1d24` | `-` |
| `ldr` at 0xe1728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1750 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe1758 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe1768 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xe1770 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xe177c | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xe1784 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xe1788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1794 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe1798 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xe17a0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe17a4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe17a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe17ac | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe17b0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe17bc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe17c0 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xe17c4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe17c8 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe17cc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe17d0 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xe17d4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe17d8 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xe1800 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe1808 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe180c | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0xe1810 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe1814 | store | 4 | `sp -0x38` | `[sp+4]` |
| `str` at 0xe1834 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xe1838 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe183c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1840 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe1844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1854 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1858 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xe185c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1860 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1864 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1868 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xe186c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1870 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xe1898 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe18a0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe18a4 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xe18ac | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe18b0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe18b4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe18b8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe18bc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe18c8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe18cc | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xe18d0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe18d4 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe18d8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe18dc | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xe18e0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe18e4 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xe190c | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe1918 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xe1920 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe192c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1930 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1938 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe193c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1944 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1948 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe194c | load | 4 | `abs 0xe1d28` | `-` |
| `ldr` at 0xe1958 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1960 | load | 4 | `abs 0xe1d28` | `-` |
| `str` at 0xe196c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1970 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1974 | load | 4 | `abs 0xe1d2c` | `-` |
| `ldr` at 0xe1980 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1988 | load | 4 | `abs 0xe1d2c` | `-` |
| `str` at 0xe1994 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1998 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe199c | load | 4 | `abs 0xe1d30` | `-` |
| `ldr` at 0xe19a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe19b0 | load | 4 | `abs 0xe1d30` | `-` |
| `str` at 0xe19bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe19c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe19d0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe19e4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe19e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe19f8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1a0c | store | 4 | `sp -0x38` | `?` |
| `str` at 0xe1a18 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xe1a1c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1a20 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1a24 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe1a28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1a38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1a3c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xe1a40 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1a44 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1a48 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1a4c | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xe1a50 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1a54 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xe1a7c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe1a84 | load | 4 | `abs 0xe1d34` | `-` |
| `ldr` at 0xe1a88 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe1a94 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1a98 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe1a9c | load | 4 | `abs 0xe1d38` | `-` |
| `ldr` at 0xe1aa0 | load | 4 | `abs 0xe1d3c` | `-` |
| `ldr` at 0xe1aa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1aa8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe1ab0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe1ac0 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xe1ac8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe1acc | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xe1ad4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1ad8 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xe1adc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe1ae0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe1ae4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe1af0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xe1af4 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xe1af8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1afc | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1b00 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1b04 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xe1b08 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1b0c | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xe1b34 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe1b3c | load | 4 | `abs 0xe1d34` | `-` |
| `ldr` at 0xe1b40 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe1b4c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1b50 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe1b54 | load | 4 | `abs 0xe1d40` | `-` |
| `ldr` at 0xe1b58 | load | 4 | `abs 0xe1d3c` | `-` |
| `ldr` at 0xe1b5c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe1b60 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1b6c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe1b70 | load | 4 | `abs 0xe1d44` | `-` |
| `str` at 0xe1b78 | store | 4 | `abs 0x2974b4` | `0x0` |
| `ldr` at 0xe1b7c | load | 4 | `abs 0xe1d44` | `-` |
| `ldr` at 0xe1b80 | load | 4 | `abs 0x2974b4` | `-` |
| `ldr` at 0xe1b8c | load | 4 | `abs 0xe1d44` | `-` |
| `ldr` at 0xe1b90 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe1bac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe1bb0 | load | 4 | `abs 0xe1d44` | `-` |
| `ldr` at 0xe1bb4 | load | 4 | `abs 0xe1d44` | `-` |
| `ldr` at 0xe1bb8 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe1bc0 | store | 4 | `abs 0x2974b4` | `[0x2974b4]+1` |
| `str` at 0xe1bd0 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xe1bd4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1bd8 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1bdc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe1be0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1bec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1bf8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe1c04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1c08 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xe1c0c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe1c10 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0xe1c14 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1c18 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0xe1c1c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1c20 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0xe1c48 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xe1c50 | load | 4 | `abs 0xe1d34` | `-` |
| `ldr` at 0xe1c54 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe1c60 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1c64 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe1c68 | load | 4 | `abs 0xe1d48` | `-` |
| `ldr` at 0xe1c6c | load | 4 | `abs 0xe1d3c` | `-` |
| `ldr` at 0xe1c70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1c74 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe1c7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1c88 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe1c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1ca8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe1cbc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xe1cc0 | load | 4 | `abs 0xe1d4c` | `-` |
| `str` at 0xe1ccc | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe1cd8 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe1cdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe1ce4 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xe1cec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe1cf8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xe1cfc | load | 4 | `abs 0xe1d50` | `-` |
| `str` at 0xe1d08 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe1d14 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xe1d18 | load | 4 | `sp -0x58` | `-` |
| `ldm` at 0xe1d20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe1d20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe1d20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe1720 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `[sp+20]` | `r0` |
| `bl` at 0xe17b8 | `(unknown callee) 0xdf0f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xe17ec | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xe1828 | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe1850 | `(unknown callee) 0xdf374` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0xe1884 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe18c4 | `(unknown callee) 0xdf0f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xe18f8 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe1a34 | `(unknown callee) 0xdf374` | `[sp-72]` | `?` | `0x1e` | `sp-56` |
| `bl` at 0xe1a68 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe1aac | `(unknown callee) 0xdf464` | `0x122ae8` | `0x122a60` | `[sp-36]` | `[sp+12]` |
| `bl` at 0xe1aec | `(unknown callee) 0xdf0f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xe1b20 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe1b64 | `(unknown callee) 0xdf464` | `0x122a50` | `0x122a60` | `[sp-76]` | `[sp+12]` |
| `bl` at 0xe1be8 | `(unknown callee) 0xdf374` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0xe1c34 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe1c78 | `(unknown callee) 0xdf464` | `0x122a64` | `0x122a60` | `[sp-36]` | `[sp+12]` |
| `bl` at 0xe1c8c | `(unknown callee) 0xd5a08` | `[sp-28]+64` | `[sp+16]` | `?` | `[sp-28]+64` |
| `bl` at 0xe1cac | `(unknown callee) 0xd5a08` | `[sp-28]+65` | `[sp+16]` | `?` | `[sp-28]+65` |
| `bl` at 0xe1cc4 | `(unknown callee) 0x19808` | `[sp+20]` | `0x122a7c` | `?` | `ret(0xd5a08)` |
| `bl` at 0xe1d00 | `(unknown callee) 0x19808` | `[sp+20]` | `0x122ab0` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe1710 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe1760 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xe1790 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe17f8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe1890 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe1904 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe1a74 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1a90 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xe1ab8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xe1b2c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1b48 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe1b88 | `[0x2974b4] > 0x2 (unsigned, 32-bit)` | `[0x2974b4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0xe1bf4 | `[sp-32] <= 0x0 (signed, 32-bit)` | `[sp-32] > 0x0 (signed, 32-bit)` |
| `bne` at 0xe1c00 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xe1c40 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1c5c | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe1c98 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `beq` at 0xe1cb8 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `bne` at 0xe1cf4 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe1d20 | `r0` | `?` |

## `PIOWrite_Multi`

- `PIOWrite_Multi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0eb0 to 0xe1618.
  Established. The unnamed `0xd0754` call carries `0x1`, `0x0`, `[sp+24]`, unknown (`bl` at 0xe0fbc).
  Nonzero `ret(0xd0d10)` takes `0xe10ac`, while zero falls through (`bne` at 0xe109c).
  Included word stores clear `0x2974b4` and then assign `[0x2974b4]+1` there (`str` at 0xe1440, `str` at 0xe1488).
  Multi-operation PIO sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe0ec0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe0ec4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe0ec8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe0ecc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe0ed0 | load | 4 | `abs 0xe15ec` | `-` |
| `ldr` at 0xe0ed4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0ef4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0ef8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe0efc | load | 4 | `abs 0xe15ec` | `-` |
| `ldr` at 0xe0f00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0f20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0f24 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe0f28 | load | 4 | `abs 0xe15ec` | `-` |
| `ldr` at 0xe0f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0f4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0f54 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe0f5c | load | 1 | `(unknown)` | `-` |
| `str` at 0xe0f60 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe0f64 | load | 4 | `abs 0xe15ec` | `-` |
| `ldr` at 0xe0f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0f88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0f90 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe0f98 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe0f9c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xe0fa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0fb8 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe0fc4 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xe0fd0 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe0fd4 | load | 4 | `abs 0xe15ec` | `-` |
| `ldr` at 0xe0fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0ff8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1000 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe1008 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe1018 | store | 4 | `sp -0x44` | `0xd0000000` |
| `str` at 0xe1020 | store | 4 | `sp -0x48` | `0x50000000` |
| `str` at 0xe102c | store | 4 | `sp -0x44` | `0x8000000` |
| `str` at 0xe1034 | store | 4 | `sp -0x48` | `0xf0000000` |
| `ldr` at 0xe1038 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe103c | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xe1044 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1048 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xe104c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe1050 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe1054 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe1060 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe1064 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xe1068 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe106c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe1070 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1074 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe1078 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe107c | store | 4 | `sp -0x60` | `[sp+24]` |
| `ldr` at 0xe1084 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe10a4 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe10ac | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe10b0 | store | 4 | `sp -0x34` | `[sp+12]` |
| `ldr` at 0xe10b4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xe10b8 | store | 4 | `sp -0x38` | `[sp+8]` |
| `str` at 0xe10d8 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xe10dc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe10e0 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe10e4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe10e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe10f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe10fc | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xe1100 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1104 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe1108 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe110c | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe1110 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe1114 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xe113c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe1144 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xe1148 | store | 4 | `sp -0x38` | `[sp-72]` |
| `ldr` at 0xe1150 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1154 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xe1158 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe115c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe1160 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe116c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe1170 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xe1174 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1178 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe117c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1180 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe1184 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe1188 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xe11b0 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe11b8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe11c4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe11c8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe11d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe11d4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe11dc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe11e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe11e4 | load | 4 | `abs 0xe15f0` | `-` |
| `ldr` at 0xe11f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe11f8 | load | 4 | `abs 0xe15f0` | `-` |
| `str` at 0xe1204 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1208 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe120c | load | 4 | `abs 0xe15f4` | `-` |
| `ldr` at 0xe1218 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1220 | load | 4 | `abs 0xe15f4` | `-` |
| `str` at 0xe122c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1230 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1234 | load | 4 | `abs 0xe15f8` | `-` |
| `ldr` at 0xe1240 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1248 | load | 4 | `abs 0xe15f8` | `-` |
| `str` at 0xe1254 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1258 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1268 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe127c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe1280 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1290 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe12a4 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xe12b0 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xe12b4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe12b8 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe12bc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe12c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe12d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe12d4 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xe12d8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe12dc | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe12e0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe12e4 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe12e8 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe12ec | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xe1314 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe131c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe132c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1330 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xe1334 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1338 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe133c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1340 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe1344 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe1348 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xe1370 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe1378 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe1388 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xe1390 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe1394 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xe139c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe13a0 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0xe13a4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe13a8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xe13ac | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xe13b8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe13bc | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0xe13c0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe13c4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe13c8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe13cc | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe13d0 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe13d4 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xe13fc | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe1404 | load | 4 | `abs 0xe15fc` | `-` |
| `ldr` at 0xe1408 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe1414 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe1418 | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0xe141c | load | 4 | `abs 0xe1600` | `-` |
| `ldr` at 0xe1420 | load | 4 | `abs 0xe1604` | `-` |
| `ldr` at 0xe1424 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xe1428 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe1434 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xe1438 | load | 4 | `abs 0xe1608` | `-` |
| `str` at 0xe1440 | store | 4 | `abs 0x2974b4` | `0x0` |
| `ldr` at 0xe1444 | load | 4 | `abs 0xe1608` | `-` |
| `ldr` at 0xe1448 | load | 4 | `abs 0x2974b4` | `-` |
| `ldr` at 0xe1454 | load | 4 | `abs 0xe1608` | `-` |
| `ldr` at 0xe1458 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe1474 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe1478 | load | 4 | `abs 0xe1608` | `-` |
| `ldr` at 0xe147c | load | 4 | `abs 0xe1608` | `-` |
| `ldr` at 0xe1480 | load | 4 | `abs 0x2974b4` | `-` |
| `str` at 0xe1488 | store | 4 | `abs 0x2974b4` | `[0x2974b4]+1` |
| `str` at 0xe1498 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0xe149c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe14a0 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe14a4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe14a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe14b4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe14c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe14cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe14d0 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0xe14d4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xe14d8 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0xe14dc | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe14e0 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0xe14e4 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xe14e8 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0xe1510 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe1518 | load | 4 | `abs 0xe15fc` | `-` |
| `ldr` at 0xe151c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe1528 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xe152c | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0xe1530 | load | 4 | `abs 0xe160c` | `-` |
| `ldr` at 0xe1534 | load | 4 | `abs 0xe1604` | `-` |
| `ldr` at 0xe1538 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe153c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xe1544 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1550 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xe1564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1570 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xe1584 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xe1588 | load | 4 | `abs 0xe1610` | `-` |
| `str` at 0xe1594 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xe15a0 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xe15a4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe15ac | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0xe15b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe15c0 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xe15c4 | load | 4 | `abs 0xe1614` | `-` |
| `str` at 0xe15d0 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xe15dc | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xe15e0 | load | 4 | `sp -0x5c` | `-` |
| `ldm` at 0xe15e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe15e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe15e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe0fbc | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `[sp+24]` | `?` |
| `bl` at 0xe105c | `(unknown callee) 0xdf0f0` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xe1090 | `(unknown callee) 0xd0d10` | `0x0` | `[sp-60]` | `0x1` | `0x0` |
| `bl` at 0xe10cc | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xe10f4 | `(unknown callee) 0xdf374` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0xe1128 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe1168 | `(unknown callee) 0xdf0f0` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xe119c | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe12cc | `(unknown callee) 0xdf374` | `[sp-76]` | `[sp-32]` | `0x1e` | `sp-56` |
| `bl` at 0xe1300 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe135c | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe13b4 | `(unknown callee) 0xdf0f0` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0xe13e8 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe142c | `(unknown callee) 0xdf464` | `0x122a50` | `0x122a60` | `[sp-80]` | `[sp+16]` |
| `bl` at 0xe14b0 | `(unknown callee) 0xdf374` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0xe14fc | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xe1540 | `(unknown callee) 0xdf464` | `0x122a64` | `0x122a60` | `[sp-36]` | `[sp+16]` |
| `bl` at 0xe1554 | `(unknown callee) 0xd5a08` | `[sp-32]+64` | `[sp+20]` | `?` | `[sp-32]+64` |
| `bl` at 0xe1574 | `(unknown callee) 0xd5a08` | `[sp-32]+65` | `[sp+20]` | `?` | `[sp-32]+65` |
| `bl` at 0xe158c | `(unknown callee) 0x19808` | `[sp+24]` | `0x122a7c` | `?` | `ret(0xd5a08)` |
| `bl` at 0xe15c8 | `(unknown callee) 0x19808` | `[sp+24]` | `0x122ab0` | `?` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe0fac | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe1010 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xe109c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe1134 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe11a8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe130c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xe1328 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe1368 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xe1380 | `[sp] != 0x0` | `[sp] == 0x0` |
| `bne` at 0xe13f4 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1410 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xe1450 | `[0x2974b4] > 0x2 (unsigned, 32-bit)` | `[0x2974b4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0xe14bc | `[sp] <= 0x0 (signed, 32-bit)` | `[sp] > 0x0 (signed, 32-bit)` |
| `bne` at 0xe14c8 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0xe1508 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bls` at 0xe1524 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xe1560 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `beq` at 0xe1580 | `ret(0xd5a08) == 0x0` | `ret(0xd5a08) != 0x0` |
| `bne` at 0xe15bc | `[sp-64] != 0x0` | `[sp-64] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe15e8 | `r0` | `?` |

## `jmstr_scan`

- `jmstr_scan`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0d10 to 0xd12a4.
  Established. A nonzero `[sp]` takes `0xd0d50`, while zero falls through (`bne` at 0xd0d40).
  The word at `sp -0x34` receives `0x10` at an included assignment (`str` at 0xd0d54).
  The unnamed `0x19808` call carries `[sp+12]`, `0x1210e4`, entry `r2`, entry `r1` (`bl` at 0xd0ed8).
  Scan state transition semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0d20 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xd0d24 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xd0d28 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xd0d2c | store | 4 | `sp -0x28` | `r3` |
| `str` at 0xd0d34 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd0d38 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xd0d48 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xd0d54 | store | 4 | `sp -0x34` | `0x10` |
| `ldr` at 0xd0d58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd0d64 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd0d6c | store | 4 | `sp -0x34` | `0x18` |
| `ldr` at 0xd0d70 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xd0d78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd0ed0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xd0ed4 | load | 4 | `abs 0xd1294` | `-` |
| `str` at 0xd0ee0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1288 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xd1290 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xd1290 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd1290 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd1290 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd1290 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd0ed8 | `(unknown callee) 0x19808` | `[sp+12]` | `0x1210e4` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd0d40 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0xd0d60 | `r3 == 0x0` | `r3 != 0x0` |
| `ldrls` at 0xd0d78 | `r1 <= 0xd (unsigned, 32-bit)` | `r1 > 0xd (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd1290 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd0d78).

## `jmstr_run_tck`

- `jmstr_run_tck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0bc0 to 0xd0c68.
  Established. The word at `sp -0x20` receives `0x10` and, on an included path, `0x18` (`str` at 0xd0be0, `str` at 0xd0bf8).
  A zero entry `r1` takes `0xd0c4c`, while nonzero falls through (`beq` at 0xd0c04).
  The indirect call carries unknown, entry `r1`, `0x0`, `0x0` (`mov` at 0xd0c24).
  Clock-run command interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0bd0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd0bd4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd0bd8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd0be0 | store | 4 | `sp -0x20` | `0x10` |
| `ldr` at 0xd0be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0bf0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd0bf8 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0xd0bfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0c08 | load | 4 | `abs 0xd0c60` | `-` |
| `ldr` at 0xd0c0c | load | 4 | `abs 0x29736c` | `-` |
| `ldr` at 0xd0c10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0c34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0c38 | load | 4 | `abs 0xd0c64` | `-` |
| `str` at 0xd0c44 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xd0c50 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xd0c54 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xd0c5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0c5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xd0c24 | `(unknown callee) (unknown target)` | `?` | `r1` | `0x0` | `0x0` |
| `bl` at 0xd0c3c | `(unknown callee) 0x197a8` | `[sp-28]` | `0x12108c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd0bec | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xd0c04 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xd0c30 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0c5c | `r0` | `?` |

## `jmstr_custom_seq`

- `jmstr_custom_seq`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0c68 to 0xd0d10.
  Established. The included indirect call carries unknown and entry `r1`, `r2`, `r3` (`mov` at 0xd0ccc);
  its nonzero result takes `0xd0cf4`, while zero falls through (`bne` at 0xd0cd8).
  The word at `sp -0x2c` receives `[sp]` (`str` at 0xd0cb0).
  Custom sequence transport semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0c78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd0c7c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd0c80 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd0c84 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xd0c8c | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xd0c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0c9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0ca4 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xd0ca8 | load | 4 | `abs 0xd0d08` | `-` |
| `ldr` at 0xd0cac | load | 4 | `sp +0x0` | `-` |
| `str` at 0xd0cb0 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xd0cb4 | load | 4 | `abs 0x297368` | `-` |
| `ldr` at 0xd0cb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0cbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0cc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0cc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0cdc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xd0ce0 | load | 4 | `abs 0xd0d0c` | `-` |
| `str` at 0xd0cec | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xd0cf8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xd0cfc | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xd0d04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0d04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0d04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xd0ccc | `(unknown callee) (unknown target)` | `?` | `r1` | `r2` | `r3` |
| `bl` at 0xd0ce4 | `(unknown callee) 0x197a8` | `[sp+4]` | `0x12108c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd0c98 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xd0cd8 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0d04 | `r0` | `?` |

## `jmstr_move_to_idle`

- `jmstr_move_to_idle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd093c to 0xd0bc0.
  Established. A zero entry `r2` takes `0xd09f4`, while nonzero falls through (`beq` at 0xd0974).
  An included word store puts `0x10` at `sp -0x28` (`str` at 0xd09f8).
  The unnamed `0x197a8` call carries `[sp-36]`, `0x1210c0`, unknown, `ret(0x1943c)` (`bl` at 0xd09b8).
  Idle transition outcomes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd094c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd0950 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd0954 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xd0958 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xd0960 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xd0968 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd096c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0978 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd098c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd09a0 | store | 4 | `sp -0x30` | `ret(0x1943c)` |
| `ldr` at 0xd09a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd09b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd09b4 | load | 4 | `abs 0xd0bb0` | `-` |
| `str` at 0xd09c0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xd09c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd09dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd09e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd09f8 | store | 4 | `sp -0x28` | `0x10` |
| `ldr` at 0xd09fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0a08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd0a10 | store | 4 | `sp -0x28` | `0x18` |
| `ldr` at 0xd0a14 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0xd0a1c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0ab0 | store | 4 | `sp -0x2c` | `0xff000000` |
| `str` at 0xd0ab8 | store | 4 | `sp -0x38` | `0x5` |
| `ldr` at 0xd0abc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0ac8 | load | 4 | `abs 0xd0bb4` | `-` |
| `str` at 0xd0ad8 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0xd0adc | load | 4 | `abs 0x297368` | `-` |
| `ldr` at 0xd0ae0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd0ae4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0b04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0b08 | load | 4 | `abs 0xd0bb8` | `-` |
| `ldr` at 0xd0b10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd0b1c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd0b28 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xd0b30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0b3c | load | 4 | `abs 0xd0bbc` | `-` |
| `ldr` at 0xd0b40 | load | 4 | `abs 0x29736c` | `-` |
| `ldr` at 0xd0b44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd0b48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0b4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd0b50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd0b68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0b6c | load | 4 | `abs 0xd0bb8` | `-` |
| `ldr` at 0xd0b74 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd0b80 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xd0b88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0b94 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd0ba0 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xd0ba4 | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0xd0bac | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd0bac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0bac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0bac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd099c | `(unknown callee) 0x1943c` | `?` | `r1` | `?` | `?` |
| `bl` at 0xd09b8 | `(unknown callee) 0x197a8` | `[sp-36]` | `0x1210c0` | `?` | `ret(0x1943c)` |
| `bl` at 0xd09f0 | `(unknown callee) 0x19748` | `ret(0x1943c)` | `0x0` | `?` | `?` |
| `mov` at 0xd0af4 | `(unknown callee) (unknown target)` | `?` | `0x5` | `sp-60` | `sp-44` |
| `bl` at 0xd0b0c | `(unknown callee) 0x197a8` | `[sp-36]` | `0x12108c` | `?` | `ret(unknown)` |
| `bl` at 0xd0b20 | `(unknown callee) 0x1970c` | `[sp-48]` | `?` | `?` | `[sp-48]` |
| `mov` at 0xd0b58 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` |
| `bl` at 0xd0b70 | `(unknown callee) 0x197a8` | `[sp-36]` | `0x12108c` | `?` | `ret(unknown)` |
| `bl` at 0xd0b78 | `(unknown callee) 0x1970c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xd0b98 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd0974 | `r2 == 0x0` | `r2 != 0x0` |
| `bne` at 0xd09ac | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `beq` at 0xd0a04 | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xd0a1c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd0ac4 | `0x5 == 0x0` | `0x5 != 0x0` |
| `bne` at 0xd0b00 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xd0b18 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xd0b38 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xd0b64 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xd0b90 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0bac | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd0a1c).

## `jmstr_set_functions`

- `jmstr_set_functions`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0714 to 0xd0754.
  Established. Absolute word stores assign entry `r0` to `0x297368` and `[sp-24]` to `0x29736c` (`str` at 0xd0734, `str` at 0xd0740).
  The integer return record preserves entry `r0` (`ldm` at 0xd0748).
  Registered pointer roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0724 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd0728 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd072c | load | 4 | `abs 0xd074c` | `-` |
| `ldr` at 0xd0730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd0734 | store | 4 | `abs 0x297368` | `r0` |
| `ldr` at 0xd0738 | load | 4 | `abs 0xd0750` | `-` |
| `ldr` at 0xd073c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd0740 | store | 4 | `abs 0x29736c` | `[sp-24]` |
| `ldm` at 0xd0748 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0748 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0748 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0748 | `r0` | `r0` |

## `scan_multi_sic_ir`

- `scan_multi_sic_ir`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4ff0 to 0xd53c4.
  Established. The unnamed `0xd4b48` call carries `sp-328`, unknown, unknown, `sp-136` (`bl` at 0xd50b0).
  The subsequent unnamed `0xd0d10` call carries `0x0`, `[sp-32]`, `[sp]`, `[sp+4]` (`bl` at 0xd50ec);
  its nonzero result takes `0xd5108`, while zero falls through (`bne` at 0xd50f8).
  Multi-device SIC scan comparison semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5000 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd5004 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd5008 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd500c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xd5014 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd5018 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd501c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5028 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd502c | load | 4 | `abs 0xd53ac` | `-` |
| `ldr` at 0xd5040 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5044 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd5048 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5064 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd5068 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5084 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd5088 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5090 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xd50a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd50a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd50b8 | store | 4 | `sp -0x28` | `ret(0xd4b48)` |
| `ldr` at 0xd50bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd50c0 | store | 4 | `sp -0x168` | `ret(0xd4b48)` |
| `str` at 0xd50c8 | store | 4 | `sp -0x164` | `sp-136` |
| `str` at 0xd50d0 | store | 4 | `sp -0x160` | `sp-232` |
| `ldr` at 0xd50d4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xd50d8 | store | 4 | `sp -0x15c` | `[sp+12]` |
| `ldr` at 0xd50e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd50e4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xd50e8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xd5100 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xd5108 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd5114 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd511c | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0xd5120 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd5128 | store | 4 | `sp -0x150` | `?` |
| `str` at 0xd5130 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd5134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5138 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xd5144 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd515c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5164 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd516c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd5170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd517c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5184 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd518c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd51a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd51a8 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xd51ac | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xd51b0 | load | 4 | `abs 0xd53b0` | `-` |
| `ldr` at 0xd51b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd51b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd51c4 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xd51cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd51d4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xd51e0 | store | 4 | `sp -0x154` | `0x0` |
| `str` at 0xd51e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd51ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd51f0 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0xd5200 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5208 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xd5210 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xd5214 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd521c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd5224 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5228 | load | 4 | `abs 0xd53b4` | `-` |
| `ldr` at 0xd5234 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd523c | load | 4 | `abs 0xd53b4` | `-` |
| `str` at 0xd5248 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xd524c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5250 | load | 4 | `abs 0xd53b8` | `-` |
| `ldr` at 0xd525c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5264 | load | 4 | `abs 0xd53b8` | `-` |
| `str` at 0xd5270 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xd5274 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5278 | load | 4 | `abs 0xd53bc` | `-` |
| `ldr` at 0xd5284 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd528c | load | 4 | `abs 0xd53bc` | `-` |
| `str` at 0xd5298 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xd529c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd52ac | load | 4 | `sp -0x154` | `-` |
| `str` at 0xd52c0 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xd52c4 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd52d4 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xd52e8 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xd52ec | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xd5304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd5308 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5310 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xd5318 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd5320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd5324 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5334 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xd533c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd5344 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd5348 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xd5350 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xd5368 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd536c | load | 4 | `sp -0x154` | `-` |
| `str` at 0xd5374 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xd5378 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xd537c | load | 4 | `abs 0xd53c0` | `-` |
| `ldr` at 0xd5380 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0xd5390 | store | 4 | `sp -0x158` | `0x0` |
| `str` at 0xd539c | store | 4 | `sp -0x158` | `0x1` |
| `ldr` at 0xd53a0 | load | 4 | `sp -0x158` | `-` |
| `ldm` at 0xd53a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd53a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd53a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd50b0 | `(unknown callee) 0xd4b48` | `sp-328` | `?` | `?` | `sp-136` |
| `bl` at 0xd50ec | `(unknown callee) 0xd0d10` | `0x0` | `[sp-32]` | `[sp]` | `[sp+4]` |
| `bl` at 0xd51bc | `(unknown callee) 0x197a8` | `[sp+12]` | `0x12128c` | `[sp-36]` | `?` |
| `bl` at 0xd5388 | `(unknown callee) 0x197a8` | `[sp+12]` | `0x1212dc` | `[sp-332]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xd5024 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd50f8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd5110 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `bhs` at 0xd5140 | `C == 1` | `not (C == 1)` |
| `beq` at 0xd5178 | `? == ?` | `? != ?` |
| `bhs` at 0xd51f8 | `C == 1` | `not (C == 1)` |
| `beq` at 0xd5330 | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd53a8 | `r0` | `?` |

## `scancmd_init`

- `scancmd_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0098 to 0xd0138.
  Established. A nonzero entry `r1` takes `0xd00c8`, while zero falls through (`bne` at 0xd00b8).
  The unnamed `0x19748` call carries entry `r1`, `0x0`, `[sp-36]`, unknown (`bl` at 0xd0120).
  An included word assignment writes `0x1` to `sp -0x20` (`str` at 0xd0128).
  Scan command record initialization remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd00a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd00ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd00b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd00c0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd00c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd00d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd00e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd00e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd00f0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd00f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd0100 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0104 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0110 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd011c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0128 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xd012c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xd0134 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0134 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0134 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd0120 | `(unknown callee) 0x19748` | `r1` | `0x0` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd00b8 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xd00d4 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0134 | `r0` | `?` |

## `scancmd_create`

- `scancmd_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcffd4 to 0xd0098.
  Established. The unnamed `0x1943c` call carries `[sp-32]`, entry `r1`, entry `r2`, unknown (`bl` at 0xd0038);
  its nonzero result takes `0xd0058`, while zero falls through (`bne` at 0xd0048).
  The word at `sp -0x1c` receives `[sp-24]` on an included path (`str` at 0xd0088).
  Command allocation failure semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcffe4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcffe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcfff8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd0000 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd0004 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd0010 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd0018 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd0020 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd0024 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd0030 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd0034 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd003c | store | 4 | `sp -0x18` | `ret(0x1943c)` |
| `ldr` at 0xd0040 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd0050 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd0058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd005c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0070 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd007c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd0084 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd0088 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xd008c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xd0094 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0094 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0094 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd0038 | `(unknown callee) 0x1943c` | `[sp-32]` | `r1` | `r2` | `?` |
| `bl` at 0xd0060 | `(unknown callee) 0xd0098` | `[sp-20]` | `ret(0x1943c)` | `?` | `ret(0x1943c)` |
| `bl` at 0xd0074 | `(unknown callee) 0x1970c` | `[sp-24]` | `?` | `?` | `ret(0xd0098)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcfff4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0048 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xd006c | `ret(0xd0098) != 0x0` | `ret(0xd0098) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0094 | `r0` | `?` |

## `scancmd_destroy`

- `scancmd_destroy`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0138 to 0xd0168.
  Established. A nonzero entry `r0` takes `0xd015c`, while zero falls through (`bne` at 0xd0154).
  The unnamed `0x1970c` call carries entry `r0`, `r1`, `r2`, `r0` (`bl` at 0xd0160).
  Command destruction ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0148 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd014c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd015c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd0164 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd0164 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0164 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd0160 | `(unknown callee) 0x1970c` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd0154 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0164 | `r0` | `?` |
