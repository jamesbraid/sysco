# ARM system protocol and environment paths in 1.44.0

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
Computed transfers in L1ReqIRouter and EnvQuery remain unresolved. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/system-protocol-environment-facts.txt) retain all tool sections.

## `L1ReqIRouter`

- `L1ReqIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7cb48 to 0x814c0.
  Established. The byte through `[sp-24] +0x20` is read (`ldrb` at 0x7cbc8).
  Nonzero `[[sp-24]+32]:1` takes `0x7cbf0`, while zero falls through (`bne` at 0x7cbd0).
  The unnamed `0xaa090` call carries `0x29a6ac`, `0x0`, `[sp-24]`, `[sp-668]` (`bl` at 0x813fc).
  Request dispatch semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7cb5c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7cb64 | store | 4 | `sp -0x29c` | `0x0` |
| `str` at 0x7cb6c | store | 4 | `sp -0x2a0` | `0x0` |
| `str` at 0x7cb7c | store | 4 | `sp -0x2a4` | `?` |
| `ldr` at 0x7cb80 | load | 4 | `abs 0x7db74` | `-` |
| `str` at 0x7cb90 | store | 4 | `sp -0x2a0` | `ret(0x20cd0)` |
| `ldr` at 0x7cb94 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x7cba0 | load | 4 | `abs 0x7db78` | `-` |
| `str` at 0x7cba4 | store | 4 | `sp -0x570` | `0xfec38` |
| `ldr` at 0x7cba8 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x7cbac | store | 4 | `sp -0x56c` | `ret(0x20cd0)` |
| `ldr` at 0x7cbb4 | load | 4 | `abs 0x7db7c` | `-` |
| `ldr` at 0x7cbb8 | load | 4 | `abs 0x7db80` | `-` |
| `ldr` at 0x7cbc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7cbc8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7cbf0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7cbf4 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x7cc04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7cc08 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x7cc0c | store | 4 | `sp -0x468` | `[[sp-24]+40]` |
| `ldr` at 0x7cc14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7cc20 | store | 4 | `sp -0x468` | `ret(0xaadf4)` |
| `ldr` at 0x7cc24 | load | 4 | `sp -0x468` | `-` |
| `str` at 0x7cc28 | store | 4 | `sp -0x2a8` | `[sp-1128]` |
| `str` at 0x7cc30 | store | 4 | `sp -0x46c` | `?` |
| `ldr` at 0x7cc34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7cc38 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x7cc44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7cc48 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x7cc50 | store | 4 | `sp -0x470` | `?` |
| `ldr` at 0x7cc54 | load | 4 | `sp -0x470` | `-` |
| `str` at 0x7cc5c | store | 4 | `sp -0x470` | `?` |
| `str` at 0x7cc68 | store | 4 | `sp -0x470` | `?` |
| `ldr` at 0x7cc6c | load | 4 | `sp -0x470` | `-` |
| `ldr` at 0x7cc7c | load | 4 | `sp -0x46c` | `-` |
| `strb` at 0x7cc80 | store | 1 | `(sp-20 + [sp-1132]) +0x0` | `(ret(0x781b8) & 0xff)` |
| `ldrb` at 0x7cc8c | load | 1 | `(unknown)` | `-` |
| `str` at 0x7cca0 | store | 4 | `sp -0x2b0` | `ret(0x554b8)` |
| `ldr` at 0x7cca4 | load | 4 | `abs 0x7db84` | `-` |
| `ldr` at 0x7cca8 | load | 4 | `abs 0x132638` | `-` |
| `ldr` at 0x7ccb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ccb8 | load | 4 | `abs 0x7db88` | `-` |
| `ldr` at 0x7ccbc | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x7ccc0 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x7cce0 | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0x7cce4 | store | 4 | `sp -0x560` | `[sp-680]` |
| `ldr` at 0x7cce8 | load | 4 | `abs 0x7db8c` | `-` |
| `ldr` at 0x7ccec | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ccf8 | load | 4 | `abs 0x7db90` | `-` |
| `ldr` at 0x7ccfc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd08 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd14 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd20 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd2c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd38 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd44 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd50 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd60 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd6c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd78 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd84 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cd94 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cda0 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cdb0 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cdbc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cdc8 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cdd4 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cde0 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cdec | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cdfc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce08 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce18 | load | 4 | `abs 0x7db94` | `-` |
| `ldr` at 0x7ce1c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce28 | load | 4 | `abs 0x7db94` | `-` |
| `ldr` at 0x7ce2c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce38 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce44 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce54 | load | 4 | `abs 0x7db98` | `-` |
| `ldr` at 0x7ce58 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce64 | load | 4 | `abs 0x7db9c` | `-` |
| `ldr` at 0x7ce68 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce78 | load | 4 | `abs 0x7dba0` | `-` |
| `ldr` at 0x7ce7c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce88 | load | 4 | `abs 0x7dba0` | `-` |
| `ldr` at 0x7ce8c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ce98 | load | 4 | `abs 0x7dba4` | `-` |
| `ldr` at 0x7ce9c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cea8 | load | 4 | `abs 0x7dba4` | `-` |
| `ldr` at 0x7ceac | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ceb8 | load | 4 | `abs 0x7dba8` | `-` |
| `ldr` at 0x7cebc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cec8 | load | 4 | `abs 0x7dba8` | `-` |
| `ldr` at 0x7cecc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7ced8 | load | 4 | `abs 0x7dbac` | `-` |
| `ldr` at 0x7cedc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cee8 | load | 4 | `abs 0x7dbb0` | `-` |
| `ldr` at 0x7ceec | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cefc | load | 4 | `abs 0x7dbb4` | `-` |
| `ldr` at 0x7cf00 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf0c | load | 4 | `abs 0x7dbb8` | `-` |
| `ldr` at 0x7cf10 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf20 | load | 4 | `abs 0x7dbbc` | `-` |
| `ldr` at 0x7cf24 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf30 | load | 4 | `abs 0x7dbbc` | `-` |
| `ldr` at 0x7cf34 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf40 | load | 4 | `abs 0x7dbc0` | `-` |
| `ldr` at 0x7cf44 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf50 | load | 4 | `abs 0x7dbc4` | `-` |
| `ldr` at 0x7cf54 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf64 | load | 4 | `abs 0x7dbc8` | `-` |
| `ldr` at 0x7cf68 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf74 | load | 4 | `abs 0x7dbcc` | `-` |
| `ldr` at 0x7cf78 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf88 | load | 4 | `abs 0x7dbd0` | `-` |
| `ldr` at 0x7cf8c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cf98 | load | 4 | `abs 0x7dbd0` | `-` |
| `ldr` at 0x7cf9c | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cfa8 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7cfac | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cfb8 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7cfbc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cfc8 | load | 4 | `abs 0x7dbd8` | `-` |
| `ldr` at 0x7cfcc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cfd8 | load | 4 | `abs 0x7dbdc` | `-` |
| `ldr` at 0x7cfdc | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cfec | load | 4 | `abs 0x7dbe0` | `-` |
| `ldr` at 0x7cff0 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7cffc | load | 4 | `abs 0x7dbe4` | `-` |
| `ldr` at 0x7d000 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d010 | load | 4 | `abs 0x7dbe8` | `-` |
| `ldr` at 0x7d014 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d020 | load | 4 | `abs 0x7dbe8` | `-` |
| `ldr` at 0x7d024 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d030 | load | 4 | `abs 0x7dbec` | `-` |
| `ldr` at 0x7d034 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d040 | load | 4 | `abs 0x7dbf0` | `-` |
| `ldr` at 0x7d044 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d054 | load | 4 | `abs 0x7dbf4` | `-` |
| `ldr` at 0x7d058 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d064 | load | 4 | `abs 0x7dbf4` | `-` |
| `ldr` at 0x7d068 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d074 | load | 4 | `sp -0x560` | `-` |
| `ldr` at 0x7d084 | load | 4 | `abs 0x7dbf8` | `-` |
| `ldr` at 0x7d088 | load | 4 | `sp -0x560` | `-` |
| `str` at 0x7d0a4 | store | 4 | `sp -0x2b8` | `ret(0x1d444)` |
| `ldr` at 0x7d0d0 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7d0fc | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d128 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d130 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x7d13c | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x7d150 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0x7d154 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x7d158 | store | 4 | `sp -0x2c4` | `[sp-676]` |
| `ldr` at 0x7d15c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x7d16c | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0x7d17c | store | 4 | `sp -0x2c4` | `0x23` |
| `ldr` at 0x7d190 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0x7d1b4 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x7d1c8 | store | 4 | `sp -0x2c0` | `?` |
| `str` at 0x7d1d8 | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7d1dc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d1e0 | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7d1e8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d1ec | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x7d1f4 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d1f8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d1fc | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `ldr` at 0x7d208 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x7d218 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d21c | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `str` at 0x7d220 | store | 4 | `sp -0x474` | `[ret(0x6a0f8)+16]` |
| `str` at 0x7d22c | store | 4 | `sp -0x474` | `0x23` |
| `ldr` at 0x7d230 | load | 4 | `sp -0x474` | `-` |
| `str` at 0x7d234 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x7d248 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d260 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x7d274 | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7d278 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d27c | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7d284 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d288 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x7d290 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d294 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d298 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `ldr` at 0x7d2a4 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x7d2b4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d2b8 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `str` at 0x7d2bc | store | 4 | `sp -0x478` | `[ret(0x6a0f8)+16]` |
| `str` at 0x7d2c8 | store | 4 | `sp -0x478` | `0x23` |
| `ldr` at 0x7d2cc | load | 4 | `sp -0x478` | `-` |
| `str` at 0x7d2d0 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x7d2e4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d2fc | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x7d310 | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7d314 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d318 | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7d320 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d324 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x7d32c | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d330 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d334 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `ldr` at 0x7d340 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x7d350 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d354 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `str` at 0x7d358 | store | 4 | `sp -0x47c` | `[ret(0x6a0f8)+16]` |
| `str` at 0x7d364 | store | 4 | `sp -0x47c` | `0x23` |
| `ldr` at 0x7d368 | load | 4 | `sp -0x47c` | `-` |
| `str` at 0x7d36c | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x7d380 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d398 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x7d3ac | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7d3b0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d3b4 | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7d3bc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d3c0 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x7d3c8 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d3cc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d3d0 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `ldr` at 0x7d3dc | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x7d3ec | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d3f0 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `str` at 0x7d3f4 | store | 4 | `sp -0x480` | `[ret(0x6a0f8)+16]` |
| `str` at 0x7d400 | store | 4 | `sp -0x480` | `0x23` |
| `ldr` at 0x7d404 | load | 4 | `sp -0x480` | `-` |
| `str` at 0x7d408 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x7d41c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d434 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x7d464 | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7d468 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d46c | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7d474 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d478 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x7d480 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d494 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d4a4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d4b0 | load | 4 | `[sp-692] +0x10` | `-` |
| `str` at 0x7d4c4 | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7d4c8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d4cc | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7d4d4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d4d8 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x7d4e0 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7d4f4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d504 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7d510 | load | 4 | `[sp-692] +0x10` | `-` |
| `ldr` at 0x7d51c | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0x7d530 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7d53c | store | 4 | `sp -0x2bc` | `ret(0x48984)` |
| `str` at 0x7d548 | store | 4 | `sp -0x2bc` | `ret(0x48b30)` |
| `ldr` at 0x7d54c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x7d58c | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x7d5a8 | store | 4 | `sp -0x570` | `0x1` |
| `ldr` at 0x7d5b0 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x7d5dc | load | 4 | `abs 0x7dbfc` | `-` |
| `str` at 0x7d5ec | store | 4 | `sp -0x2b8` | `ret(0x1973c)` |
| `ldr` at 0x7d600 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d608 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x7d624 | store | 4 | `sp -0x2b8` | `ret(0x1973c)` |
| `ldr` at 0x7d638 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d640 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x7d658 | store | 4 | `sp -0x2b8` | `ret(0x1973c)` |
| `ldr` at 0x7d66c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7d678 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7d67c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7d6a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7d6a8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7d6b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7d6bc | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7d6c0 | store | 4 | `sp -0x484` | `[[sp-24]+48]` |
| `ldr` at 0x7d6c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7d6d4 | store | 4 | `sp -0x484` | `ret(0xaadf4)` |
| `ldr` at 0x7d6d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7d6dc | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x7d6e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7d6ec | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x7d6f4 | store | 4 | `sp -0x488` | `?` |
| `ldr` at 0x7d6f8 | load | 4 | `sp -0x488` | `-` |
| `str` at 0x7d700 | store | 4 | `sp -0x488` | `?` |
| `str` at 0x7d70c | store | 4 | `sp -0x488` | `?` |
| `ldr` at 0x7d710 | load | 4 | `sp -0x488` | `-` |
| `ldr` at 0x7d714 | load | 4 | `sp -0x484` | `-` |
| `str` at 0x7d720 | store | 4 | `sp -0x2cc` | `ret(0x7c2d4)` |
| `str` at 0x7d738 | store | 4 | `sp -0x2d0` | `ret(0xc5738)` |
| `ldr` at 0x7d73c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d75c | load | 4 | `abs 0x7dc00` | `-` |
| `ldr` at 0x7d780 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d784 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7d7cc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d7e0 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d7f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d7f8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d80c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d814 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7d818 | store | 4 | `sp -0x48c` | `?` |
| `ldr` at 0x7d81c | load | 4 | `sp -0x48c` | `-` |
| `ldr` at 0x7d828 | load | 4 | `sp -0x48c` | `-` |
| `ldr` at 0x7d840 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d854 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d864 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d874 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d888 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d8a8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d8bc | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d8cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d8f4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d908 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d918 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d928 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d93c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d94c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d95c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d970 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d980 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d990 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d9a4 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d9b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d9c8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7d9dc | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7d9ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7d9fc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7da10 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7da20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7da2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7da30 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7da58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7da5c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7da6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7da70 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7da74 | store | 4 | `sp -0x490` | `[[sp-24]+48]` |
| `ldr` at 0x7da7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7da88 | store | 4 | `sp -0x490` | `ret(0xaadf4)` |
| `ldr` at 0x7da8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7da90 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x7da9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7daa0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x7daa8 | store | 4 | `sp -0x494` | `?` |
| `ldr` at 0x7daac | load | 4 | `sp -0x494` | `-` |
| `str` at 0x7dab4 | store | 4 | `sp -0x494` | `?` |
| `str` at 0x7dac0 | store | 4 | `sp -0x494` | `?` |
| `ldr` at 0x7dac4 | load | 4 | `sp -0x494` | `-` |
| `ldr` at 0x7dac8 | load | 4 | `sp -0x490` | `-` |
| `str` at 0x7dad4 | store | 4 | `sp -0x2cc` | `ret(0x7c2d4)` |
| `ldr` at 0x7dad8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7dadc | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7daec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7daf0 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7daf4 | store | 4 | `sp -0x498` | `[[sp-24]+56]` |
| `ldr` at 0x7dafc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7db08 | store | 4 | `sp -0x498` | `ret(0xaadf4)` |
| `ldr` at 0x7db0c | load | 4 | `sp -0x498` | `-` |
| `str` at 0x7db10 | store | 4 | `sp -0x2d4` | `[sp-1176]` |
| `ldr` at 0x7db14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7db18 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x7db28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7db2c | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x7db30 | store | 4 | `sp -0x49c` | `[[sp-24]+64]` |
| `ldr` at 0x7db38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7db44 | store | 4 | `sp -0x49c` | `ret(0xaadf4)` |
| `ldr` at 0x7db48 | load | 4 | `sp -0x49c` | `-` |
| `str` at 0x7db4c | store | 4 | `sp -0x2d8` | `[sp-1180]` |
| `ldr` at 0x7db50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7db54 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x7db64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7db68 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x7db6c | store | 4 | `sp -0x4a0` | `[[sp-24]+72]` |
| `ldr` at 0x7dc1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7dc28 | store | 4 | `sp -0x4a0` | `ret(0xaadf4)` |
| `ldr` at 0x7dc2c | load | 4 | `sp -0x4a0` | `-` |
| `str` at 0x7dc30 | store | 4 | `sp -0x2dc` | `[sp-1184]` |
| `str` at 0x7dc38 | store | 4 | `sp -0x2e0` | `0x0` |
| `str` at 0x7dc40 | store | 4 | `sp -0x2e4` | `0x0` |
| `str` at 0x7dc58 | store | 4 | `sp -0x2e8` | `ret(0xc5738)` |
| `ldr` at 0x7dc5c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7dc98 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7dc9c | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x7dcc4 | load | 4 | `sp -0x2d4` | `-` |
| `ldrls` at 0x7dccc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7e108 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e114 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e118 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x7e140 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e144 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7e14c | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x7e158 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x7e15c | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x7e164 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0x7e16c | store | 4 | `sp -0x2e4` | `0x0` |
| `ldr` at 0x7e174 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e178 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7e180 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x7e188 | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0x7e1b4 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x7e1bc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7e1d0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e1d4 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e1e8 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7e1f4 | load | 4 | `abs 0x7dc08` | `-` |
| `ldr` at 0x7e1f8 | load | 4 | `abs 0x29768c` | `-` |
| `ldr` at 0x7e204 | load | 4 | `abs 0x7dc08` | `-` |
| `ldr` at 0x7e208 | load | 4 | `abs 0x29768c` | `-` |
| `str` at 0x7e21c | store | 4 | `sp -0x2bc` | `ret(unknown)` |
| `str` at 0x7e22c | store | 4 | `sp -0x2bc` | `ret(0x1d444)` |
| `ldr` at 0x7e230 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x7e278 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x7e294 | store | 4 | `sp -0x570` | `0x1` |
| `ldr` at 0x7e29c | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x7e2b8 | store | 4 | `sp -0x2b8` | `ret(0x1d444)` |
| `ldr` at 0x7e2e4 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7e2fc | store | 4 | `sp -0x2b8` | `ret(0x1d444)` |
| `ldr` at 0x7e328 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7e34c | load | 4 | `sp -0x2a4` | `-` |
| `ldrls` at 0x7e358 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7e5b0 | store | 4 | `sp -0x2b8` | `ret(0x1d444)` |
| `ldr` at 0x7e5c4 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7e5d8 | store | 4 | `sp -0x2b8` | `ret(0x1d444)` |
| `ldr` at 0x7e5ec | load | 4 | `sp -0x2b8` | `-` |
| `ldrb` at 0x7e620 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e674 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x7e6bc | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x7e6d4 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x7e6e8 | store | 4 | `sp -0x2dc` | `ret(0x54224)` |
| `ldr` at 0x7e6ec | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7e714 | load | 4 | `abs 0x7dc08` | `-` |
| `ldr` at 0x7e718 | load | 4 | `abs 0x297634` | `-` |
| `str` at 0x7e71c | store | 4 | `sp -0x4a4` | `[0x297634]` |
| `ldr` at 0x7e720 | load | 4 | `abs 0x7dc08` | `-` |
| `ldr` at 0x7e724 | load | 4 | `abs 0x297634` | `-` |
| `str` at 0x7e734 | store | 4 | `sp -0x4a4` | `0x1` |
| `ldr` at 0x7e738 | load | 4 | `sp -0x4a4` | `-` |
| `str` at 0x7e73c | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x7e740 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x7e754 | store | 4 | `sp -0x2d0` | `ret(0x1943c)` |
| `ldr` at 0x7e758 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7e760 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7e770 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x7e784 | store | 4 | `sp -0x2cc` | `ret(0x1943c)` |
| `ldr` at 0x7e788 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7e790 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x7e7a4 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x7e7a8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e7ac | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7e7b8 | load | 4 | `abs 0x7dc08` | `-` |
| `ldr` at 0x7e7c0 | load | 4 | `abs 0x29760c` | `-` |
| `str` at 0x7e7c8 | store | 4 | `sp -0x2f0` | `?` |
| `ldr` at 0x7e7cc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e7d0 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0x7e7dc | store | 4 | `sp -0x2f4` | `ret(0xeb174)` |
| `ldr` at 0x7e7e0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e7e8 | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0x7e7f4 | load | 4 | `abs 0x7dc08` | `-` |
| `ldr` at 0x7e7f8 | load | 4 | `abs 0x29760c` | `-` |
| `str` at 0x7e800 | store | 4 | `sp -0x2f8` | `?` |
| `ldr` at 0x7e804 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e820 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x7e828 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7e830 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x7e838 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7e844 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7e84c | load | 4 | `(unknown)` | `-` |
| `str` at 0x7e854 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x7e858 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7e874 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x7e87c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7e884 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x7e88c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7e898 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7e8a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7e8a8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x7e8ac | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x7e8b4 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x7e8e4 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7e8fc | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7e908 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x7e90c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e910 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7e924 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e92c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7e93c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7e94c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7e954 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7e964 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7e96c | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x7e974 | store | 4 | `sp -0x2d8` | `[sp-728]+1` |
| `ldr` at 0x7e97c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7e984 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x7e994 | store | 4 | `sp -0x2f8` | `sp-868` |
| `str` at 0x7e9c0 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x7e9c4 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x7e9d0 | load | 4 | `sp -0x318` | `-` |
| `str` at 0x7e9dc | store | 4 | `sp -0x318` | `?` |
| `ldr` at 0x7e9e0 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x7e9e8 | store | 4 | `sp -0x310` | `?` |
| `ldr` at 0x7e9ec | load | 4 | `sp -0x310` | `-` |
| `str` at 0x7e9f4 | store | 4 | `sp -0x310` | `?` |
| `str` at 0x7e9fc | store | 4 | `sp -0x30c` | `0xc5` |
| `ldr` at 0x7ea00 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x7ea0c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7ea10 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea14 | load | 4 | `abs 0x7dc0c` | `-` |
| `ldr` at 0x7ea20 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea28 | load | 4 | `abs 0x7dc0c` | `-` |
| `str` at 0x7ea34 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7ea38 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea3c | load | 4 | `abs 0x7dc10` | `-` |
| `ldr` at 0x7ea48 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea50 | load | 4 | `abs 0x7dc10` | `-` |
| `str` at 0x7ea5c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7ea60 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea64 | load | 4 | `abs 0x7dc14` | `-` |
| `ldr` at 0x7ea70 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea78 | load | 4 | `abs 0x7dc14` | `-` |
| `str` at 0x7ea84 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7ea88 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7ea98 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7eaac | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7eab0 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7eac0 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7ead4 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7ead8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7eadc | store | 4 | `sp -0x308` | `[sp-732]` |
| `str` at 0x7eaec | store | 4 | `sp -0x2a0` | `ret(0xd7168)` |
| `ldr` at 0x7eaf0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x7eafc | load | 4 | `sp -0x300` | `-` |
| `str` at 0x7eb00 | store | 4 | `sp -0x324` | `[sp-768]` |
| `ldr` at 0x7eb04 | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0x7eb08 | store | 4 | `sp -0x320` | `[sp-764]` |
| `ldr` at 0x7eb0c | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x7eb10 | store | 4 | `sp -0x31c` | `[sp-760]` |
| `str` at 0x7eb18 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x7eb1c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7eb28 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7eb34 | load | 4 | `abs 0x7dc18` | `-` |
| `ldr` at 0x7eb48 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7eb58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7eb6c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7eb74 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7eb7c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7eb84 | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0x7eb94 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x7eb9c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x7eba0 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x7eba8 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x7ebd8 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x7ebe4 | store | 4 | `sp -0x2a0` | `0x0` |
| `ldr` at 0x7ebec | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x7ebf4 | store | 4 | `sp -0x2f8` | `[sp-760]+8` |
| `ldr` at 0x7ebf8 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x7ec00 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x7ec08 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x7ec50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ec54 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7ec7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ec80 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7ec90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ec94 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7ec98 | store | 4 | `sp -0x4a8` | `[[sp-24]+48]` |
| `ldr` at 0x7eca0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7ecac | store | 4 | `sp -0x4a8` | `ret(0xaadf4)` |
| `ldr` at 0x7ecb4 | load | 4 | `sp -0x4a8` | `-` |
| `ldr` at 0x7ecd8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ecdc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7ed04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ed08 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x7ed14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ed18 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x7ed20 | store | 4 | `sp -0x4ac` | `?` |
| `ldr` at 0x7ed24 | load | 4 | `sp -0x4ac` | `-` |
| `str` at 0x7ed2c | store | 4 | `sp -0x4ac` | `?` |
| `str` at 0x7ed38 | store | 4 | `sp -0x4ac` | `0x60` |
| `ldr` at 0x7ed3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ed40 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7ed50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ed54 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x7ed58 | load | 4 | `sp -0x4ac` | `-` |
| `str` at 0x7ed60 | store | 4 | `sp -0x4b0` | `?` |
| `ldr` at 0x7ed68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ed78 | load | 4 | `sp -0x4ac` | `-` |
| `str` at 0x7ed80 | store | 4 | `sp -0x4b0` | `?` |
| `ldr` at 0x7ed84 | load | 4 | `sp -0x4b0` | `-` |
| `str` at 0x7edb4 | store | 4 | `sp -0x2b4` | `ret(0x6a0f8)` |
| `ldr` at 0x7edb8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7edbc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7edc0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7edc4 | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x7edc8 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `ldr` at 0x7edcc | load | 4 | `ret(0x6a0f8) +0x14` | `-` |
| `ldr` at 0x7edf0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7edf4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7ee1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ee20 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7ee4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ee50 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7ee60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ee64 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7ee68 | store | 4 | `sp -0x4b4` | `[[sp-24]+48]` |
| `ldr` at 0x7ee70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7ee7c | store | 4 | `sp -0x4b4` | `ret(0xaadf4)` |
| `ldr` at 0x7ee84 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x7eeac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7eeb0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7eed8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7eedc | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7eeec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7eef0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7eef4 | store | 4 | `sp -0x4b8` | `[[sp-24]+48]` |
| `ldr` at 0x7eefc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7ef08 | store | 4 | `sp -0x4b8` | `ret(0xaadf4)` |
| `ldr` at 0x7ef10 | load | 4 | `sp -0x4b8` | `-` |
| `ldr` at 0x7ef48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ef4c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7ef74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7ef78 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7ef88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ef8c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7ef90 | store | 4 | `sp -0x4bc` | `[[sp-24]+48]` |
| `ldr` at 0x7ef98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7efa4 | store | 4 | `sp -0x4bc` | `ret(0xaadf4)` |
| `ldr` at 0x7efac | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x7efd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7efd4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7effc | load | 4 | `abs 0x7ffa4` | `-` |
| `str` at 0x7f000 | store | 4 | `sp -0x4c0` | `0x29a5d4` |
| `ldr` at 0x7f004 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f008 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f01c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f020 | store | 4 | `sp -0x4c4` | `[[sp-24]+48]` |
| `ldr` at 0x7f028 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f034 | store | 4 | `sp -0x4c4` | `ret(0xaadf4)` |
| `ldrb` at 0x7f040 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f048 | load | 4 | `sp -0x4c4` | `-` |
| `ldr` at 0x7f050 | load | 4 | `sp -0x4c0` | `-` |
| `str` at 0x7f054 | store | 4 | `[sp-1216] +0xc` | `ret(0x783d8)` |
| `ldr` at 0x7f058 | load | 4 | `abs 0x7ffa4` | `-` |
| `ldr` at 0x7f060 | load | 4 | `abs 0x29a5e0` | `-` |
| `ldr` at 0x7f084 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f088 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7f0b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f0b4 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f0c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f0c8 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7f0d0 | store | 4 | `sp -0x4d8` | `?` |
| `ldr` at 0x7f0d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f0e4 | store | 4 | `sp -0x4d8` | `ret(0xaadf4)` |
| `ldr` at 0x7f0e8 | load | 4 | `sp -0x4d8` | `-` |
| `str` at 0x7f0f0 | store | 4 | `sp -0x4d8` | `?` |
| `ldr` at 0x7f0f4 | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x7f100 | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x7f10c | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x7f11c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f120 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f134 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f138 | store | 4 | `sp -0x4c8` | `[[sp-24]+48]` |
| `ldr` at 0x7f140 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f14c | store | 4 | `sp -0x4c8` | `ret(0xaadf4)` |
| `ldr` at 0x7f150 | load | 4 | `sp -0x4c8` | `-` |
| `ldr` at 0x7f160 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f164 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f178 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f17c | store | 4 | `sp -0x4cc` | `[[sp-24]+48]` |
| `ldr` at 0x7f184 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f190 | store | 4 | `sp -0x4cc` | `ret(0xaadf4)` |
| `ldr` at 0x7f194 | load | 4 | `sp -0x4cc` | `-` |
| `ldr` at 0x7f1a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f1a4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f1b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f1b8 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f1bc | store | 4 | `sp -0x4d0` | `[[sp-24]+48]` |
| `ldr` at 0x7f1c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f1d0 | store | 4 | `sp -0x4d0` | `ret(0xaadf4)` |
| `ldr` at 0x7f1d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f1d8 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f1e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f1ec | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7f1f4 | store | 4 | `sp -0x4d4` | `?` |
| `ldr` at 0x7f1fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f208 | store | 4 | `sp -0x4d4` | `ret(0xaadf4)` |
| `ldr` at 0x7f20c | load | 4 | `sp -0x4d4` | `-` |
| `str` at 0x7f214 | store | 4 | `sp -0x4d4` | `?` |
| `ldr` at 0x7f218 | load | 4 | `sp -0x4d0` | `-` |
| `ldr` at 0x7f21c | load | 4 | `sp -0x4d4` | `-` |
| `ldr` at 0x7f268 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f26c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7f294 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f298 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f2a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f2ac | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7f2b4 | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x7f2b8 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x7f2c0 | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x7f2c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f2d4 | store | 4 | `sp -0x4e0` | `ret(0xaadf4)` |
| `ldr` at 0x7f2d8 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x7f2e0 | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x7f2e4 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x7f2ec | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x7f2f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f2f4 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x7f304 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f308 | load | 4 | `[sp-24] +0x40` | `-` |
| `ldr` at 0x7f318 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x7f320 | store | 4 | `sp -0x4dc` | `?` |
| `ldr` at 0x7f328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f344 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x7f34c | store | 4 | `sp -0x4dc` | `?` |
| `ldr` at 0x7f350 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f354 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f368 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x7f370 | load | 4 | `sp -0x4dc` | `-` |
| `str` at 0x7f378 | store | 4 | `sp -0x4e4` | `?` |
| `ldr` at 0x7f380 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f394 | load | 4 | `sp -0x4dc` | `-` |
| `str` at 0x7f39c | store | 4 | `sp -0x4e4` | `?` |
| `ldr` at 0x7f3a0 | load | 4 | `sp -0x4e4` | `-` |
| `str` at 0x7f3a4 | store | 4 | `sp -0x2b0` | `[sp-1252]` |
| `ldrb` at 0x7f3b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f3bc | load | 4 | `sp -0x2b0` | `-` |
| `ldrb` at 0x7f3cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f3d0 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x7f3d8 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x7f3e8 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x7f3f0 | store | 4 | `sp -0x570` | `?` |
| `ldr` at 0x7f3f4 | load | 4 | `abs 0x7ffa8` | `-` |
| `ldr` at 0x7f400 | load | 4 | `abs 0x7ffa4` | `-` |
| `ldr` at 0x7f404 | load | 4 | `abs 0x29a5e0` | `-` |
| `ldr` at 0x7f410 | load | 4 | `abs 0x7ffa4` | `-` |
| `ldr` at 0x7f414 | load | 4 | `abs 0x29a5e0` | `-` |
| `ldrb` at 0x7f428 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f43c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f440 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x7f470 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7f474 | store | 4 | `sp -0x4e8` | `?` |
| `ldr` at 0x7f478 | load | 4 | `sp -0x4e8` | `-` |
| `str` at 0x7f480 | store | 4 | `sp -0x4e8` | `?` |
| `ldr` at 0x7f484 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f488 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f49c | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x7f4a0 | load | 4 | `sp -0x4e8` | `-` |
| `str` at 0x7f4ac | store | 4 | `sp -0x4ec` | `?` |
| `ldr` at 0x7f4b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f4c4 | load | 4 | `sp -0x4e8` | `-` |
| `str` at 0x7f4d0 | store | 4 | `sp -0x4ec` | `?` |
| `ldr` at 0x7f4d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f4d8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f4e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f4ec | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f4f4 | store | 4 | `sp -0x4f0` | `?` |
| `ldr` at 0x7f4fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f510 | store | 4 | `sp -0x4f0` | `?` |
| `ldr` at 0x7f514 | load | 4 | `sp -0x4ec` | `-` |
| `ldr` at 0x7f518 | load | 4 | `sp -0x4f0` | `-` |
| `str` at 0x7f52c | store | 4 | `sp -0x2b8` | `ret(0x47ad8)` |
| `ldr` at 0x7f530 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f564 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f570 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f574 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7f580 | load | 4 | `abs 0x7ffac` | `-` |
| `ldr` at 0x7f584 | load | 4 | `abs 0x1338f4` | `-` |
| `ldr` at 0x7f5ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f5b0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f5c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f5c4 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f5cc | store | 4 | `sp -0x4f4` | `?` |
| `ldr` at 0x7f5d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f5e8 | store | 4 | `sp -0x4f4` | `?` |
| `ldr` at 0x7f5ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f5f0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f604 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7f60c | store | 4 | `sp -0x4f8` | `?` |
| `ldr` at 0x7f614 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f628 | store | 4 | `sp -0x4f8` | `?` |
| `ldr` at 0x7f62c | load | 4 | `abs 0x7ffac` | `-` |
| `ldr` at 0x7f630 | load | 4 | `abs 0x1338f4` | `-` |
| `ldr` at 0x7f638 | load | 4 | `sp -0x4f4` | `-` |
| `ldr` at 0x7f63c | load | 4 | `sp -0x4f8` | `-` |
| `str` at 0x7f648 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x7f64c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f680 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f68c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f690 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7f69c | load | 4 | `abs 0x7ffb0` | `-` |
| `ldr` at 0x7f6a0 | load | 4 | `abs 0x1338f8` | `-` |
| `ldr` at 0x7f6c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f6cc | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f6dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f6e0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f6e8 | store | 4 | `sp -0x4fc` | `?` |
| `ldr` at 0x7f6f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f704 | store | 4 | `sp -0x4fc` | `?` |
| `ldr` at 0x7f708 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f70c | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f71c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f720 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7f728 | store | 4 | `sp -0x500` | `?` |
| `ldr` at 0x7f730 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f744 | store | 4 | `sp -0x500` | `?` |
| `ldr` at 0x7f748 | load | 4 | `abs 0x7ffb0` | `-` |
| `ldr` at 0x7f74c | load | 4 | `abs 0x1338f8` | `-` |
| `ldr` at 0x7f754 | load | 4 | `sp -0x4fc` | `-` |
| `ldr` at 0x7f758 | load | 4 | `sp -0x500` | `-` |
| `str` at 0x7f764 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x7f768 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f79c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f7a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f7ac | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x7f7b8 | load | 4 | `abs 0x7ffb4` | `-` |
| `ldr` at 0x7f7bc | load | 4 | `abs 0x1338fc` | `-` |
| `ldr` at 0x7f7e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f7e8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7f7f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f7fc | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x7f804 | store | 4 | `sp -0x504` | `?` |
| `ldr` at 0x7f80c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f820 | store | 4 | `sp -0x504` | `?` |
| `ldr` at 0x7f824 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7f828 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x7f838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f83c | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x7f844 | store | 4 | `sp -0x508` | `?` |
| `ldr` at 0x7f84c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f860 | store | 4 | `sp -0x508` | `?` |
| `ldr` at 0x7f864 | load | 4 | `abs 0x7ffb4` | `-` |
| `ldr` at 0x7f868 | load | 4 | `abs 0x1338fc` | `-` |
| `ldr` at 0x7f870 | load | 4 | `sp -0x504` | `-` |
| `ldr` at 0x7f874 | load | 4 | `sp -0x508` | `-` |
| `str` at 0x7f880 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x7f884 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f8b8 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7f8cc | store | 4 | `sp -0x2b8` | `ret(0x37174)` |
| `ldr` at 0x7f8d0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f904 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7f918 | store | 4 | `sp -0x2b8` | `ret(0x37558)` |
| `ldr` at 0x7f91c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7f950 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7f970 | store | 4 | `sp -0x2f4` | `ret(0xc5738)` |
| `ldr` at 0x7f974 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x7f9b8 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x7f9bc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7f9c0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x7f9cc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7f9ec | load | 4 | `abs 0x7ffb8` | `-` |
| `ldr` at 0x7f9f0 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x7fa14 | store | 4 | `sp -0x2d0` | `(sp-996 + ret(0x19538))` |
| `str` at 0x7fa1c | store | 4 | `sp -0x2d8` | `0x1` |
| `ldr` at 0x7fa20 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fa24 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x7fa30 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7fa58 | load | 4 | `abs 0x7ffbc` | `-` |
| `ldr` at 0x7fa64 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fa68 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x7fa74 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fa78 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x7fa84 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fa88 | load | 4 | `abs 0x7ffc0` | `-` |
| `ldr` at 0x7fa98 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x7fabc | store | 4 | `sp -0x2d4` | `ret(0x814c0)` |
| `ldr` at 0x7fac0 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7facc | load | 4 | `abs 0x7ffc4` | `-` |
| `str` at 0x7fad0 | store | 4 | `sp -0x50c` | `0xfecd4` |
| `ldr` at 0x7fad8 | load | 4 | `abs 0x7ffc8` | `-` |
| `str` at 0x7fadc | store | 4 | `sp -0x50c` | `0xfecdc` |
| `ldr` at 0x7fae0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fae4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fae8 | load | 4 | `[sp-760] +0x50` | `-` |
| `str` at 0x7faec | store | 4 | `sp -0x570` | `[[sp-760]+80]` |
| `ldr` at 0x7faf0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7faf4 | load | 4 | `[sp-760] +0x54` | `-` |
| `str` at 0x7faf8 | store | 4 | `sp -0x56c` | `[[sp-760]+84]` |
| `ldr` at 0x7fafc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fb00 | load | 4 | `[sp-760] +0x58` | `-` |
| `ldrb` at 0x7fb04 | load | 1 | `[[sp-760]+88] +0x0` | `-` |
| `ldr` at 0x7fb14 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fb18 | load | 4 | `[sp-760] +0x58` | `-` |
| `str` at 0x7fb20 | store | 4 | `sp -0x568` | `?` |
| `ldr` at 0x7fb24 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fb28 | load | 4 | `[sp-760] +0x5c` | `-` |
| `str` at 0x7fb2c | store | 4 | `sp -0x564` | `[[sp-760]+92]` |
| `ldr` at 0x7fb30 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fb34 | load | 4 | `abs 0x7ffcc` | `-` |
| `ldr` at 0x7fb38 | load | 4 | `sp -0x50c` | `-` |
| `ldr` at 0x7fb3c | load | 4 | `[sp-760] +0x4c` | `-` |
| `ldr` at 0x7fb48 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fb4c | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x7fb58 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7fb80 | load | 4 | `abs 0x7ffd0` | `-` |
| `ldr` at 0x7fb8c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fb90 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x7fb9c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fba0 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x7fbac | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fbb0 | load | 4 | `abs 0x7ffd4` | `-` |
| `ldr` at 0x7fbc0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fbc4 | load | 4 | `abs 0x7ffd8` | `-` |
| `ldr` at 0x7fbcc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fbdc | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x7fbe4 | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x19538))` |
| `str` at 0x7fbec | store | 4 | `sp -0x2dc` | `0x40` |
| `ldr` at 0x7fbf0 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7fbfc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fc00 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x7fc04 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x7fc0c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7fc10 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fc14 | load | 4 | `abs 0x7ffdc` | `-` |
| `ldr` at 0x7fc20 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x7fc28 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x7fc2c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7fc34 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `str` at 0x7fc40 | store | 4 | `sp -0x2dc` | `0x5f` |
| `ldr` at 0x7fc44 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7fc50 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fc54 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x7fc58 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x7fc60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7fc64 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fc68 | load | 4 | `abs 0x7ffdc` | `-` |
| `ldr` at 0x7fc74 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x7fc7c | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x7fc80 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7fc88 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x7fc94 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x7fca0 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x7fca8 | store | 4 | `sp -0x2d0` | `?` |
| `str` at 0x7fcb0 | store | 4 | `sp -0x2dc` | `0x49` |
| `ldr` at 0x7fcb4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7fcc0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fcc4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x7fcc8 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x7fcd0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7fcec | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fcf0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fcf4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x7fcf8 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x7fd04 | load | 1 | `([[sp-760]+12] + [sp-732]) +0x0` | `-` |
| `strb` at 0x7fd08 | store | 1 | `[sp-720] +0x0` | `([([[sp-760]+12] + [sp-732])]:1 & 0xff)` |
| `str` at 0x7fd10 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x7fd1c | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x7fd28 | store | 1 | `[sp-720] +0x0` | `0x20` |
| `str` at 0x7fd30 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x7fd34 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x7fd3c | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x7fd48 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x7fd54 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x7fd5c | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x7fd60 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fd68 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fd70 | load | 1 | `[[sp-760]+12] +0x40` | `-` |
| `ldr` at 0x7fd7c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fd84 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fd8c | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x7fda4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fdac | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fdb4 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x7fdcc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fdd4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fddc | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x7fde0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fde8 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fdf0 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x7fdf4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fdf8 | load | 4 | `abs 0x7ffe0` | `-` |
| `ldr` at 0x7fe04 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x7fe0c | store | 4 | `sp -0x2d0` | `[sp-720]+4` |
| `ldr` at 0x7fe18 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fe1c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fe24 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fe30 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `strb` at 0x7fe34 | store | 1 | `[sp-720] +0x0` | `([[[sp-760]+12]+91]:1 & 0xff)` |
| `str` at 0x7fe3c | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x7fe44 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fe48 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fe50 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fe5c | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `strb` at 0x7fe60 | store | 1 | `[sp-720] +0x1` | `([[[sp-760]+12]+92]:1 & 0xff)` |
| `str` at 0x7fe68 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x7fe70 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x7fe7c | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x7fe84 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x7fe88 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7fe90 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7fe98 | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x7fea4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7feac | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x7feb4 | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x7febc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fec0 | load | 4 | `abs 0x7ffe4` | `-` |
| `ldr` at 0x7fecc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7fedc | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x7fee4 | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x19538))` |
| `ldr` at 0x7fee8 | load | 4 | `abs 0x8001c` | `-` |
| `ldr` at 0x7feec | load | 4 | `abs 0x133908` | `-` |
| `ldr` at 0x7fef8 | load | 4 | `abs 0x8001c` | `-` |
| `ldr` at 0x7fefc | load | 4 | `abs 0x133908` | `-` |
| `ldr` at 0x7ff00 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7ff0c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7ff1c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7ff20 | load | 4 | `abs 0x7ffe8` | `-` |
| `ldr` at 0x7ff2c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7ff30 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x7ff3c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7ff64 | load | 4 | `abs 0x7ffec` | `-` |
| `ldr` at 0x7ff70 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7ff74 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x7ff80 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7ff84 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x7ff90 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7ff94 | load | 4 | `abs 0x7fff0` | `-` |
| `ldr` at 0x80030 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80034 | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x80040 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80044 | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x80050 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x80054 | load | 4 | `abs 0x7fff4` | `-` |
| `ldr` at 0x80064 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80068 | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x80074 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80078 | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x80084 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x80088 | load | 4 | `abs 0x7fff8` | `-` |
| `ldr` at 0x80098 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8009c | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x800a8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x800ac | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x800b8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x800bc | load | 4 | `abs 0x7fffc` | `-` |
| `ldr` at 0x800cc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x800d0 | load | 4 | `abs 0x80000` | `-` |
| `ldr` at 0x800d8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x800e8 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x800f0 | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x19538))` |
| `str` at 0x800f8 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x800fc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8010c | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x80110 | store | 4 | `sp -0x514` | `?` |
| `str` at 0x80118 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8011c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80120 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x80124 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8012c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x80144 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80148 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8014c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x80154 | load | 1 | `([[sp-760]+68] + [sp-732]) +0x0` | `-` |
| `strb` at 0x8015c | store | 1 | `sp -0x50d` | `([([[sp-760]+68] + [sp-732])]:1 & 0xff)` |
| `strb` at 0x8016c | store | 1 | `sp -0x50d` | `0x3f` |
| `ldrb` at 0x80174 | load | 1 | `sp -0x50d` | `-` |
| `ldr` at 0x80178 | load | 4 | `sp -0x514` | `-` |
| `strb` at 0x8017c | store | 1 | `[sp-1300] +0x0` | `?` |
| `ldr` at 0x80180 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x80188 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x80194 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x801a0 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x801a8 | store | 4 | `sp -0x2d0` | `?` |
| `str` at 0x801b0 | store | 4 | `sp -0x2dc` | `0x7` |
| `ldr` at 0x801b4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x801c0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x801c4 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x801cc | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x801d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x801dc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x801e0 | load | 4 | `abs 0x80004` | `-` |
| `ldr` at 0x801ec | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x801f4 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x801f8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x80200 | store | 4 | `sp -0x2dc` | `[sp-732]-1` |
| `ldr` at 0x8020c | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x80218 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x80220 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x80224 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80228 | load | 4 | `[sp-760] +0x24` | `-` |
| `ldrb` at 0x8022c | load | 1 | `[[sp-760]+36] +0x3` | `-` |
| `ldr` at 0x80230 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80234 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x80238 | load | 4 | `[sp-760] +0x34` | `-` |
| `str` at 0x8023c | store | 4 | `sp -0x570` | `[[sp-760]+52]` |
| `ldr` at 0x80240 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x80244 | load | 4 | `abs 0x80008` | `-` |
| `ldr` at 0x80248 | load | 4 | `[sp-760] +0x28` | `-` |
| `ldr` at 0x8025c | load | 4 | `abs 0x8000c` | `-` |
| `str` at 0x80268 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x8026c | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8029c | store | 4 | `sp -0x570` | `0x1` |
| `ldr` at 0x802b0 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x802b8 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x802bc | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x802c4 | store | 4 | `sp -0x2f8` | `[sp-760]+120` |
| `ldr` at 0x802cc | load | 4 | `abs 0x80010` | `-` |
| `ldr` at 0x802d4 | load | 4 | `abs 0x80014` | `-` |
| `ldr` at 0x802dc | load | 4 | `abs 0x29c3a8` | `-` |
| `str` at 0x802f4 | store | 4 | `sp -0x2b8` | `ret(0x1d444)` |
| `ldr` at 0x80334 | load | 4 | `abs 0x80018` | `-` |
| `ldr` at 0x80364 | load | 4 | `abs 0x80018` | `-` |
| `ldr` at 0x80380 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8038c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80390 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x803bc | store | 4 | `sp -0x518` | `ret(0xceae4)` |
| `ldr` at 0x803c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x803c4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x803d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x803d8 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x803dc | load | 4 | `sp -0x518` | `-` |
| `ldr` at 0x803ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x803fc | load | 4 | `sp -0x518` | `-` |
| `ldr` at 0x80428 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8042c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8043c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80440 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x80444 | store | 4 | `sp -0x51c` | `[[sp-24]+48]` |
| `ldr` at 0x8044c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80458 | store | 4 | `sp -0x51c` | `ret(0xaadf4)` |
| `ldr` at 0x8045c | load | 4 | `sp -0x51c` | `-` |
| `str` at 0x80468 | store | 4 | `sp -0x2a0` | `ret(0xcebfc)` |
| `ldr` at 0x8046c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x804a0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x804c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x804cc | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x804f8 | store | 4 | `sp -0x520` | `ret(0xceae4)` |
| `ldr` at 0x804fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80500 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x80510 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80514 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x80518 | load | 4 | `sp -0x520` | `-` |
| `ldr` at 0x80528 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80538 | load | 4 | `sp -0x520` | `-` |
| `ldr` at 0x80564 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80568 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x80578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8057c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x80580 | store | 4 | `sp -0x524` | `[[sp-24]+48]` |
| `ldr` at 0x80588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80594 | store | 4 | `sp -0x524` | `ret(0xaadf4)` |
| `ldr` at 0x805a0 | load | 4 | `sp -0x524` | `-` |
| `str` at 0x805b0 | store | 4 | `sp -0x2a0` | `ret(0xcecb8)` |
| `ldr` at 0x805b4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x805e8 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x80628 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8062c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x80638 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8063c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8064c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80650 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x80654 | store | 4 | `sp -0x528` | `[[sp-24]+48]` |
| `ldr` at 0x8065c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80668 | store | 4 | `sp -0x528` | `ret(0xaadf4)` |
| `ldr` at 0x8066c | load | 4 | `sp -0x528` | `-` |
| `str` at 0x80670 | store | 4 | `sp -0x2e0` | `[sp-1320]` |
| `ldr` at 0x80674 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80678 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x806c4 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x806c8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x806cc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x806d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x806dc | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x806f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x80700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80704 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80718 | load | 4 | `(unknown)` | `-` |
| `str` at 0x80720 | store | 4 | `sp -0x52c` | `?` |
| `ldr` at 0x80728 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80730 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80744 | store | 4 | `sp -0x52c` | `?` |
| `ldr` at 0x80754 | load | 4 | `sp -0x52c` | `-` |
| `str` at 0x80764 | store | 4 | `sp -0x2a0` | `ret(0xca70c)` |
| `ldr` at 0x80768 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8079c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x807b4 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x807d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x807e0 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x807e8 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x807f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x807f4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8081c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80820 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x80830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80834 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x80838 | store | 4 | `sp -0x530` | `[[sp-24]+48]` |
| `ldr` at 0x80840 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8084c | store | 4 | `sp -0x530` | `ret(0xaadf4)` |
| `ldr` at 0x80850 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x80854 | store | 4 | `sp -0x2e0` | `[sp-1328]` |
| `ldr` at 0x80858 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8086c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80870 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x808b4 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x808b8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x808bc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x808c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x808cc | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x808ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x808fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80900 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80920 | load | 4 | `(unknown)` | `-` |
| `str` at 0x80928 | store | 4 | `sp -0x534` | `?` |
| `ldr` at 0x80930 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80944 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80958 | store | 4 | `sp -0x534` | `?` |
| `ldr` at 0x8095c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80960 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x80980 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x80990 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80994 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x809b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x809bc | store | 4 | `sp -0x538` | `?` |
| `ldr` at 0x809c4 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x809d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x809ec | store | 4 | `sp -0x538` | `?` |
| `ldr` at 0x809f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x809f4 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x80a14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x80a24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80a28 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80a48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x80a50 | store | 4 | `sp -0x53c` | `?` |
| `ldr` at 0x80a58 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80a6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80a80 | store | 4 | `sp -0x53c` | `?` |
| `ldr` at 0x80a88 | load | 4 | `sp -0x534` | `-` |
| `ldr` at 0x80a8c | load | 4 | `sp -0x538` | `-` |
| `ldr` at 0x80a90 | load | 4 | `sp -0x53c` | `-` |
| `str` at 0x80a9c | store | 4 | `sp -0x2a0` | `ret(0xca584)` |
| `ldr` at 0x80aa0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x80ad4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x80aec | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80af8 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x80b00 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x80b08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80b0c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x80b34 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b40 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b4c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b58 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b64 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b70 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b7c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b88 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x80b98 | load | 4 | `abs 0x8001c` | `-` |
| `ldr` at 0x80b9c | load | 4 | `abs 0x133908` | `-` |
| `ldr` at 0x80ba8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80bac | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x80bbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80bc0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x80bc4 | store | 4 | `sp -0x540` | `[[sp-24]+48]` |
| `ldr` at 0x80bcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80bd8 | store | 4 | `sp -0x540` | `ret(0xaadf4)` |
| `ldr` at 0x80bdc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80be0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x80bf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80bf4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x80bf8 | store | 4 | `sp -0x544` | `[[sp-24]+56]` |
| `ldr` at 0x80c00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80c0c | store | 4 | `sp -0x544` | `ret(0xaadf4)` |
| `ldr` at 0x80c10 | load | 4 | `abs 0x8001c` | `-` |
| `ldr` at 0x80c14 | load | 4 | `abs 0x133908` | `-` |
| `ldr` at 0x80c1c | load | 4 | `sp -0x540` | `-` |
| `ldr` at 0x80c20 | load | 4 | `sp -0x544` | `-` |
| `str` at 0x80c34 | store | 4 | `sp -0x2f4` | `ret(unknown)` |
| `ldr` at 0x80c38 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80c6c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x80ca0 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x80cec | store | 4 | `sp -0x2a0` | `ret(0x475e0)` |
| `ldr` at 0x80d00 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x80d0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80d10 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x80d38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80d3c | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x80d4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80d50 | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x80d54 | load | 4 | `abs 0x80020` | `-` |
| `str` at 0x80d60 | store | 4 | `sp -0x548` | `?` |
| `ldr` at 0x80d64 | load | 4 | `sp -0x548` | `-` |
| `ldr` at 0x80d6c | load | 4 | `sp -0x548` | `-` |
| `ldr` at 0x80d78 | load | 4 | `sp -0x548` | `-` |
| `str` at 0x80d84 | store | 4 | `sp -0x548` | `?` |
| `ldr` at 0x80d8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80d98 | store | 4 | `sp -0x548` | `ret(0xaadf4)` |
| `ldr` at 0x80d9c | load | 4 | `abs 0x80020` | `-` |
| `ldr` at 0x80da0 | load | 4 | `sp -0x548` | `-` |
| `ldr` at 0x80dc0 | load | 4 | `sp -0x548` | `-` |
| `str` at 0x80dc8 | store | 4 | `sp -0x548` | `?` |
| `ldr` at 0x80dcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80dd0 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x80de0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80de4 | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x80dec | store | 4 | `sp -0x54c` | `?` |
| `ldr` at 0x80df4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80e00 | store | 4 | `sp -0x54c` | `ret(0xaadf4)` |
| `ldr` at 0x80e04 | load | 4 | `sp -0x54c` | `-` |
| `str` at 0x80e0c | store | 4 | `sp -0x54c` | `?` |
| `ldr` at 0x80e10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80e14 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x80e24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80e28 | load | 4 | `[sp-24] +0x48` | `-` |
| `ldr` at 0x80e2c | load | 4 | `abs 0x80024` | `-` |
| `str` at 0x80e38 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0x80e3c | load | 4 | `sp -0x550` | `-` |
| `ldr` at 0x80e44 | load | 4 | `sp -0x550` | `-` |
| `str` at 0x80e54 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0x80e5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80e68 | store | 4 | `sp -0x550` | `ret(0xaadf4)` |
| `ldr` at 0x80e6c | load | 4 | `abs 0x80024` | `-` |
| `ldr` at 0x80e70 | load | 4 | `sp -0x550` | `-` |
| `ldr` at 0x80e8c | load | 4 | `sp -0x550` | `-` |
| `str` at 0x80e94 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0x80e98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80e9c | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x80eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80eb0 | load | 4 | `[sp-24] +0x50` | `-` |
| `ldr` at 0x80eb4 | load | 4 | `abs 0x8002c` | `-` |
| `str` at 0x80ec0 | store | 4 | `sp -0x554` | `?` |
| `ldr` at 0x80ec4 | load | 4 | `sp -0x554` | `-` |
| `ldr` at 0x80ecc | load | 4 | `sp -0x554` | `-` |
| `str` at 0x80edc | store | 4 | `sp -0x554` | `?` |
| `ldr` at 0x80ee4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80ef0 | store | 4 | `sp -0x554` | `ret(0xaadf4)` |
| `ldr` at 0x80ef4 | load | 4 | `abs 0x8002c` | `-` |
| `ldr` at 0x80ef8 | load | 4 | `sp -0x554` | `-` |
| `ldr` at 0x80f14 | load | 4 | `sp -0x554` | `-` |
| `str` at 0x80f1c | store | 4 | `sp -0x554` | `?` |
| `ldr` at 0x80f20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80f24 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x80f34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80f38 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x80f3c | load | 4 | `abs 0x80028` | `-` |
| `str` at 0x80f48 | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0x80f4c | load | 4 | `sp -0x558` | `-` |
| `ldr` at 0x80f54 | load | 4 | `sp -0x558` | `-` |
| `ldr` at 0x80f60 | load | 4 | `sp -0x558` | `-` |
| `str` at 0x80f78 | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0x80f80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x80f8c | store | 4 | `sp -0x558` | `ret(0xaadf4)` |
| `ldr` at 0x80f90 | load | 4 | `abs 0x80028` | `-` |
| `ldr` at 0x80f94 | load | 4 | `sp -0x558` | `-` |
| `ldr` at 0x80fc0 | load | 4 | `sp -0x558` | `-` |
| `str` at 0x80fc8 | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0x80fcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x80fd0 | load | 1 | `[sp-24] +0x54` | `-` |
| `ldr` at 0x80fe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80fe4 | load | 4 | `[sp-24] +0x58` | `-` |
| `ldr` at 0x80fe8 | load | 4 | `abs 0x8002c` | `-` |
| `str` at 0x80ff4 | store | 4 | `sp -0x55c` | `?` |
| `ldr` at 0x80ff8 | load | 4 | `sp -0x55c` | `-` |
| `ldr` at 0x81000 | load | 4 | `sp -0x55c` | `-` |
| `str` at 0x81010 | store | 4 | `sp -0x55c` | `?` |
| `ldr` at 0x81018 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x81024 | store | 4 | `sp -0x55c` | `ret(0xaadf4)` |
| `ldr` at 0x81028 | load | 4 | `abs 0x81490` | `-` |
| `ldr` at 0x8102c | load | 4 | `sp -0x55c` | `-` |
| `ldr` at 0x81048 | load | 4 | `sp -0x55c` | `-` |
| `str` at 0x81050 | store | 4 | `sp -0x55c` | `?` |
| `ldr` at 0x81058 | load | 4 | `sp -0x550` | `-` |
| `str` at 0x8105c | store | 4 | `sp -0x570` | `[sp-1360]` |
| `ldr` at 0x81060 | load | 4 | `sp -0x554` | `-` |
| `str` at 0x81064 | store | 4 | `sp -0x56c` | `[sp-1364]` |
| `ldr` at 0x81068 | load | 4 | `sp -0x558` | `-` |
| `str` at 0x8106c | store | 4 | `sp -0x568` | `[sp-1368]` |
| `ldr` at 0x81070 | load | 4 | `sp -0x55c` | `-` |
| `str` at 0x81074 | store | 4 | `sp -0x564` | `[sp-1372]` |
| `ldr` at 0x8107c | load | 4 | `abs 0x81494` | `-` |
| `ldr` at 0x81080 | load | 4 | `sp -0x548` | `-` |
| `ldr` at 0x81084 | load | 4 | `sp -0x54c` | `-` |
| `ldr` at 0x810a8 | load | 4 | `abs 0x81498` | `-` |
| `str` at 0x810f4 | store | 4 | `sp -0x2f4` | `ret(0x194f0)` |
| `str` at 0x81108 | store | 4 | `sp -0x2e0` | `ret(0x196e8)` |
| `ldr` at 0x81114 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x81118 | load | 4 | `ret(0x196e8) +0x14` | `-` |
| `ldr` at 0x81138 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8113c | load | 4 | `[sp-736] +0x10` | `-` |
| `ldr` at 0x81158 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x81164 | load | 4 | `[sp-736] +0xc` | `-` |
| `ldr` at 0x81174 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x81180 | load | 4 | `[sp-736] +0x8` | `-` |
| `ldr` at 0x81190 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8119c | load | 4 | `[sp-736] +0x4` | `-` |
| `ldr` at 0x811ac | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x811b8 | load | 4 | `[sp-736] +0x0` | `-` |
| `ldr` at 0x811c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x811c8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x811f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x811fc | load | 4 | `abs 0x8149c` | `-` |
| `ldr` at 0x81200 | load | 4 | `abs 0x29a5e4` | `-` |
| `ldr` at 0x8120c | load | 4 | `abs 0x8149c` | `-` |
| `ldrb` at 0x81218 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x81240 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x81248 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81254 | load | 4 | `abs 0x8149c` | `-` |
| `ldrb` at 0x81260 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x81288 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x81290 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x812c4 | load | 4 | `abs 0x8149c` | `-` |
| `ldrb` at 0x812d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x812f8 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x81300 | load | 4 | `(unknown)` | `-` |
| `str` at 0x81310 | store | 4 | `sp -0x2a0` | `ret(0x4cce8)` |
| `ldr` at 0x81314 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x81348 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8137c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x81388 | load | 4 | `abs 0x814a0` | `-` |
| `ldr` at 0x8138c | load | 4 | `abs 0x1338f0` | `-` |
| `ldr` at 0x81398 | load | 4 | `abs 0x814a0` | `-` |
| `ldr` at 0x813a4 | load | 4 | `abs 0x1338f0` | `-` |
| `ldr` at 0x813a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x813d8 | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x813e8 | store | 4 | `sp -0x570` | `sp-664` |
| `ldr` at 0x813ec | load | 4 | `abs 0x814a4` | `-` |
| `ldr` at 0x813f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x813f8 | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x81404 | store | 4 | `sp -0x2a0` | `ret(0xaa090)` |
| `ldr` at 0x81408 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x81414 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x81420 | load | 4 | `abs 0x814a8` | `-` |
| `str` at 0x81424 | store | 4 | `sp -0x570` | `0xfeea8` |
| `str` at 0x81428 | store | 4 | `sp -0x56c` | `ret(0x1f8e0)` |
| `ldr` at 0x81430 | load | 4 | `abs 0x814ac` | `-` |
| `ldr` at 0x81434 | load | 4 | `abs 0x814b0` | `-` |
| `ldr` at 0x81440 | load | 4 | `abs 0x814b4` | `-` |
| `str` at 0x8144c | store | 4 | `sp -0x2a0` | `ret(0x2105c)` |
| `ldr` at 0x81450 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8145c | load | 4 | `abs 0x814b8` | `-` |
| `str` at 0x81460 | store | 4 | `sp -0x570` | `0xfeec4` |
| `ldr` at 0x81464 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x81468 | store | 4 | `sp -0x56c` | `ret(0x2105c)` |
| `ldr` at 0x81470 | load | 4 | `abs 0x814ac` | `-` |
| `ldr` at 0x81474 | load | 4 | `abs 0x814bc` | `-` |
| `ldr` at 0x81480 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x8148c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x8148c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8148c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8148c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7cb70 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x7cb88 | `(unknown callee) 0x20cd0` | `0x29a658` | `?` | `?` | `?` |
| `bl` at 0x7cbc0 | `(unknown callee) 0x200a0` | `0x9` | `0xfebf0` | `0x1b9` | `0x0` |
| `bl` at 0x7cbe8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7cc1c | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x7cc70 | `(unknown callee) 0x781b8` | `[sp-1136]` | `?` | `?` | `?` |
| `bl` at 0x7cc98 | `(unknown callee) 0x554b8` | `?` | `[sp-1132]` | `sp-20` | `?` |
| `bl` at 0x7ccc4 | `(unknown callee) 0x1e958` | `0xfec5c` | `[[sp-24]+28]` | `[sp-680]` | `[sp-24]` |
| `bl` at 0x7ccdc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d09c | `(unknown callee) 0x1d444` | `0x3` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x7d0bc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d0d4 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d0e0 | `(unknown callee) 0x1d444` | `0x1` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x7d0f0 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x7d114 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d12c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d194 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7d1b0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d1d0 | `(unknown callee) 0x6a0f8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x7d24c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d264 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x7d26c | `(unknown callee) 0x6a0f8` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x7d2e8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d300 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x7d308 | `(unknown callee) 0x6a0f8` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x7d384 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d39c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x7d3a4 | `(unknown callee) 0x6a0f8` | `0x4` | `?` | `?` | `?` |
| `bl` at 0x7d420 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d438 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x7d454 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d45c | `(unknown callee) 0x6a0f8` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x7d498 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d4b4 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x7d4bc | `(unknown callee) 0x6a0f8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x7d4f8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7d514 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x7d538 | `(unknown callee) 0x48984` | `?` | `sp-20` | `[sp-680]` | `?` |
| `bl` at 0x7d544 | `(unknown callee) 0x48b30` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-676]` |
| `bl` at 0x7d56c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7d588 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d590 | `(unknown callee) 0x19538` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x7d5b8 | `(unknown callee) 0x5ed50` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x19538)` |
| `bl` at 0x7d5d4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d5e8 | `(unknown callee) 0x1973c` | `0xfec70` | `sp-700` | `0xa` | `sp-700` |
| `bl` at 0x7d604 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `ret(0x1973c)` | `sp-668` |
| `bl` at 0x7d620 | `(unknown callee) 0x1973c` | `[sp-700]+1` | `sp-700` | `0xa` | `[sp-700]+1` |
| `bl` at 0x7d63c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `ret(0x1973c)` | `sp-668` |
| `bl` at 0x7d654 | `(unknown callee) 0x1973c` | `[sp-700]+1` | `0x0` | `0xa` | `[sp-700]+1` |
| `bl` at 0x7d670 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `ret(0x1973c)` | `sp-668` |
| `bl` at 0x7d69c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7d6d0 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7d718 | `(unknown callee) 0x7c2d4` | `[sp-1160]` | `[sp-1156]` | `?` | `?` |
| `bl` at 0x7d730 | `(unknown callee) 0xc5738` | `0x0` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x7d74c | `(unknown callee) 0xc5790` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7d760 | `(unknown callee) 0x1e4ac` | `0xfec78` | `?` | `?` | `ret(0xc5790)` |
| `bl` at 0x7d778 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7d7a4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7d7c0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7d7f4 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d868 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d89c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d8d0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d8e8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `0x8` | `sp-668` |
| `bl` at 0x7d91c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d950 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d984 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d9b8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7d9f0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7da24 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7da50 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7da84 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7dacc | `(unknown callee) 0x7c2d4` | `[sp-1172]` | `[sp-1168]` | `?` | `?` |
| `bl` at 0x7db04 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x7db40 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x3` | `[sp-1176]` | `?` |
| `bl` at 0x7dc24 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x7dc50 | `(unknown callee) 0xc5738` | `0x0` | `sp-720` | `?` | `sp-720` |
| `bl` at 0x7dc6c | `(unknown callee) 0xc5790` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7dc90 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7dcbc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7e104 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7e138 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7e1a0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e1b8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x7e1ec | `(unknown callee) 0x5ebd0` | `sp-668` | `sp-664` | `([sp-736] + [sp-728])` | `[sp-732]` |
| `mov` at 0x7e214 | `(unknown callee) (unknown target)` | `0x0` | `[sp-680]` | `[sp-680]` | `[0x29768c]` |
| `bl` at 0x7e228 | `(unknown callee) 0x1d444` | `0xd` | `[sp-680]` | `[sp-680]` | `[0x29768c]` |
| `bl` at 0x7e234 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x7e258 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7e274 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e27c | `(unknown callee) 0x19538` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x7e2a4 | `(unknown callee) 0x5ed50` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x19538)` |
| `bl` at 0x7e2b0 | `(unknown callee) 0x1d444` | `0xe` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x7e2d0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e2e8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7e2f4 | `(unknown callee) 0x1d444` | `0x12` | `[sp-680]` | `[sp-680]` | `0x1004` |
| `bl` at 0x7e314 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e32c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7e348 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e5a8 | `(unknown callee) 0x1d444` | `0x2a` | `?` | `?` | `[sp-676]-48` |
| `bl` at 0x7e5c8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `ret(0x1d444)` | `sp-668` |
| `bl` at 0x7e5d0 | `(unknown callee) 0x1d444` | `0x2b` | `?` | `?` | `?` |
| `bl` at 0x7e5f0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `ret(0x1d444)` | `sp-668` |
| `bl` at 0x7e60c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e62c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7e648 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e660 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e678 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-676]` | `sp-668` |
| `bl` at 0x7e690 | `(unknown callee) 0x4800c` | `sp-744` | `sp-740` | `sp-740` | `sp-744` |
| `bl` at 0x7e6a8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e6c0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-744]` | `sp-668` |
| `bl` at 0x7e6d8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x7e6e0 | `(unknown callee) 0x54224` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x7e70c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7e74c | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x7e76c | `(unknown callee) 0x19748` | `ret(0x1943c)` | `0x0` | `?` | `?` |
| `bl` at 0x7e77c | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x7e79c | `(unknown callee) 0x19748` | `ret(0x1943c)` | `0x0` | `?` | `?` |
| `bl` at 0x7e7d4 | `(unknown callee) 0xeb174` | `[sp-736]` | `[sp-752]` | `0x4` | `?` |
| `bl` at 0x7e7ec | `(unknown callee) 0xeb368` | `[sp-736]` | `[sp-752]` | `?` | `[sp-736]` |
| `bl` at 0x7e810 | `(unknown callee) 0x54244` | `?` | `?` | `ret(0xeb368)` | `?` |
| `bl` at 0x7e864 | `(unknown callee) 0x542cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x7e8d0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7e8e8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-724]` | `sp-668` |
| `bl` at 0x7e900 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-732]` | `sp-668` |
| `bl` at 0x7e940 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7e968 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x7e980 | `(unknown callee) 0x1970c` | `[sp-720]` | `?` | `?` | `[sp-724]` |
| `bl` at 0x7e988 | `(unknown callee) 0x1970c` | `[sp-716]` | `?` | `?` | `?` |
| `bl` at 0x7e9a8 | `(unknown callee) 0x19748` | `sp-868` | `0x0` | `0x40` | `sp-868` |
| `bl` at 0x7e9b8 | `(unknown callee) 0xd0098` | `0x0` | `sp-792` | `?` | `sp-792` |
| `bl` at 0x7eae8 | `(unknown callee) 0xd7168` | `sp-792` | `?` | `?` | `sp-792` |
| `bl` at 0x7ebc4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7ebdc | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x7ec28 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7ec48 | `(unknown callee) 0x5ebd0` | `sp-668` | `sp-664` | `sp-868` | `0x40` |
| `bl` at 0x7ec74 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7eca8 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7ecb8 | `(unknown callee) 0x1b5f0` | `0x3` | `[sp-1192]` | `?` | `?` |
| `bl` at 0x7ecd0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7ecfc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7ed70 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7ed88 | `(unknown callee) 0x5d778` | `[sp-1200]` | `?` | `?` | `?` |
| `bl` at 0x7eda0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7edac | `(unknown callee) 0x6a0f8` | `0x0` | `[sp-680]` | `[sp-680]` | `0x1003` |
| `bl` at 0x7edd0 | `(unknown callee) 0x5d934` | `[ret(0x6a0f8)+8]` | `[ret(0x6a0f8)+12]` | `[ret(0x6a0f8)+20]` | `ret(0x6a0f8)` |
| `bl` at 0x7ede8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7ee14 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7ee44 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7ee78 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7ee8c | `(unknown callee) 0x22e24` | `0x32` | `[sp-1204]` | `0x0` | `?` |
| `bl` at 0x7eea4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7eed0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7ef04 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7ef14 | `(unknown callee) 0x1b5f0` | `0xe` | `[sp-1208]` | `?` | `?` |
| `bl` at 0x7ef1c | `(unknown callee) 0x1d444` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x7ef28 | `(unknown callee) 0x6a80c` | `ret(0x1d444)` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x7ef40 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7ef6c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7efa0 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7efb0 | `(unknown callee) 0x1b5f0` | `0x12` | `[sp-1212]` | `?` | `?` |
| `bl` at 0x7efc8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7eff4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f030 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `0x29a5d4` | `?` |
| `bl` at 0x7f04c | `(unknown callee) 0x783d8` | `?` | `[sp-1220]` | `?` | `?` |
| `bl` at 0x7f064 | `(unknown callee) 0x1b5f0` | `0x13` | `[0x29a5e0]` | `?` | `0x29a5d4` |
| `bl` at 0x7f07c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7f0a8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f0e0 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x7f148 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7f158 | `(unknown callee) 0x4abf0` | `[sp-1224]` | `0x0` | `?` | `?` |
| `bl` at 0x7f18c | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7f198 | `(unknown callee) 0x4ac60` | `[sp-1228]` | `?` | `?` | `?` |
| `bl` at 0x7f1cc | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7f204 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x7f224 | `(unknown callee) 0x4acb8` | `[sp-1232]` | `[sp-1236]` | `0x0` | `?` |
| `bl` at 0x7f240 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f258 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7f260 | `(unknown callee) 0x7a790` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `0x100a` |
| `bl` at 0x7f28c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f2d0 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x7f330 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x7f388 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7f3c0 | `(unknown callee) 0x55300` | `?` | `[sp-688]` | `?` | `?` |
| `bl` at 0x7f3fc | `(unknown callee) 0x1e958` | `0xfec8c` | `?` | `?` | `?` |
| `bl` at 0x7f418 | `(unknown callee) 0x781b8` | `[0x29a5e0]` | `?` | `?` | `0x29a5d4` |
| `bl` at 0x7f434 | `(unknown callee) 0x7a790` | `sp-20` | `?` | `ret(0x781b8)` | `?` |
| `bl` at 0x7f460 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f4bc | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x7f504 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x7f51c | `(unknown callee) 0x546c4` | `[sp-1260]` | `[sp-1264]` | `?` | `?` |
| `bl` at 0x7f528 | `(unknown callee) 0x47ad8` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2001` |
| `bl` at 0x7f550 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f568 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7f5a4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f5dc | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7f61c | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x7f644 | `(unknown callee) (unknown target)` | `0x0` | `[sp-1268]` | `[sp-1272]` | `[0x1338f4]` |
| `bl` at 0x7f66c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f684 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7f6c0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f6f8 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7f738 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x7f760 | `(unknown callee) (unknown target)` | `0x0` | `[sp-1276]` | `[sp-1280]` | `[0x1338f8]` |
| `bl` at 0x7f788 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f7a0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7f7dc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f814 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7f854 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x7f87c | `(unknown callee) (unknown target)` | `0x0` | `[sp-1284]` | `[sp-1288]` | `[0x1338fc]` |
| `bl` at 0x7f8a4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f8bc | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7f8c8 | `(unknown callee) 0x37174` | `0x3` | `?` | `[sp-680]` | `0x2005` |
| `bl` at 0x7f8f0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f908 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7f914 | `(unknown callee) 0x37558` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2006` |
| `bl` at 0x7f93c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f954 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7f968 | `(unknown callee) 0xc5738` | `0x0` | `sp-760` | `[sp-680]` | `sp-760` |
| `bl` at 0x7f994 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7f9b0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7f9dc | `(unknown callee) 0xc5bbc` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7f9f8 | `(unknown callee) 0x197a8` | `sp-996` | `0xfeca8` | `[sp-736]` | `ret(0xc5bbc)` |
| `bl` at 0x7fa08 | `(unknown callee) 0x19538` | `sp-996` | `?` | `?` | `sp-996` |
| `bl` at 0x7fa40 | `(unknown callee) 0xc5790` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7fa5c | `(unknown callee) 0x195e0` | `sp-996` | `0xfecb0` | `?` | `sp-996` |
| `bl` at 0x7fa7c | `(unknown callee) 0xc5c28` | `[[sp-760]+28]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x7fa90 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfecc4` | `ret(0xc5c28)` | `ret(0xc5c28)` |
| `bl` at 0x7faa8 | `(unknown callee) 0xc5bbc` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7fab4 | `(unknown callee) 0x814c0` | `ret(0xc5bbc)` | `?` | `?` | `ret(0xc5bbc)` |
| `bl` at 0x7fb40 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfece8` | `[sp-1292]` | `[[sp-760]+76]` |
| `bl` at 0x7fb68 | `(unknown callee) 0xc5790` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7fb84 | `(unknown callee) 0x195e0` | `sp-996` | `0xfed00` | `?` | `sp-996` |
| `bl` at 0x7fba4 | `(unknown callee) 0xc5c28` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x7fbb8 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfed14` | `ret(0xc5c28)` | `ret(0xc5c28)` |
| `bl` at 0x7fbc8 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfed24` | `?` | `[[sp-760]+4]` |
| `bl` at 0x7fbd0 | `(unknown callee) 0x19538` | `[sp-720]` | `?` | `?` | `?` |
| `bl` at 0x7fc1c | `(unknown callee) 0x197a8` | `?` | `0xfed30` | `?` | `?` |
| `bl` at 0x7fc70 | `(unknown callee) 0x197a8` | `?` | `0xfed30` | `?` | `?` |
| `bl` at 0x7fcd8 | `(unknown callee) 0x192d4` | `?` | `?` | `[[sp-760]+12]` | `?` |
| `bl` at 0x7fd94 | `(unknown callee) 0x192d4` | `[[[sp-760]+12]+91]:1` | `?` | `0x5b` | `[[[sp-760]+12]+91]:1` |
| `bl` at 0x7fdbc | `(unknown callee) 0x192d4` | `[[[sp-760]+12]+92]:1` | `?` | `0x5c` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x7fe00 | `(unknown callee) 0x197a8` | `?` | `0xfed38` | `[[[sp-760]+12]+91]:1` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x7fec8 | `(unknown callee) 0x197a8` | `?` | `0xfed44` | `?` | `?` |
| `bl` at 0x7fed0 | `(unknown callee) 0x19538` | `[sp-720]` | `?` | `?` | `?` |
| `mov` at 0x7ff14 | `(unknown callee) (unknown target)` | `[sp-760]` | `0x0` | `0x0` | `([sp-720] + ret(0x19538))` |
| `bl` at 0x7ff24 | `(unknown callee) 0x19808` | `([sp-720] + ret(0x19538))` | `0xfed50` | `ret(0x19538)` | `[0x133908]` |
| `bl` at 0x7ff4c | `(unknown callee) 0xc5790` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7ff68 | `(unknown callee) 0x195e0` | `sp-996` | `0xfed54` | `?` | `sp-996` |
| `bl` at 0x7ff88 | `(unknown callee) 0xc5c28` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x7ff9c | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfed68` | `ret(0xc5c28)` | `ret(0xc5c28)` |
| `bl` at 0x80048 | `(unknown callee) 0xc5c28` | `[[sp-760]+32]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8005c | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfed78` | `ret(0xc5c28)` | `ret(0xc5c28)` |
| `bl` at 0x8007c | `(unknown callee) 0xc5c28` | `[[sp-760]+44]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x80090 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfed9c` | `ret(0xc5c28)` | `ret(0xc5c28)` |
| `bl` at 0x800b0 | `(unknown callee) 0xc5c28` | `[[sp-760]+64]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x800c4 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfedc0` | `ret(0xc5c28)` | `ret(0xc5c28)` |
| `bl` at 0x800d4 | `(unknown callee) 0x197a8` | `[sp-720]` | `0xfede8` | `?` | `[[sp-760]+64]` |
| `bl` at 0x800dc | `(unknown callee) 0x19538` | `[sp-720]` | `?` | `?` | `?` |
| `bl` at 0x80134 | `(unknown callee) 0x192d4` | `?` | `?` | `[[sp-760]+68]` | `?` |
| `bl` at 0x801e8 | `(unknown callee) 0x197a8` | `?` | `0xfedf4` | `?` | `?` |
| `bl` at 0x8024c | `(unknown callee) 0x197a8` | `?` | `0xfedfc` | `[[[sp-760]+36]+3]:1` | `[[sp-760]+40]` |
| `bl` at 0x80260 | `(unknown callee) 0x195e0` | `sp-996` | `0xfee10` | `?` | `sp-996` |
| `bl` at 0x80280 | `(unknown callee) 0x19538` | `sp-996` | `?` | `?` | `sp-996` |
| `bl` at 0x802ac | `(unknown callee) 0x5ed50` | `sp-668` | `sp-664` | `sp-996` | `ret(0x19538)` |
| `bl` at 0x802d0 | `(unknown callee) 0x1e4ac` | `0xfee18` | `[sp-680]` | `[sp-680]` | `0x4002` |
| `bl` at 0x802e0 | `(unknown callee) 0x1ef30` | `0x1` | `[0x29c3a8]` | `?` | `0x29c398` |
| `bl` at 0x802ec | `(unknown callee) 0x1d444` | `0x7e` | `[sp-680]` | `[sp-680]` | `0x1001` |
| `bl` at 0x8030c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x80314 | `(unknown callee) 0x1d444` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x80338 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `0xffff` | `sp-668` |
| `bl` at 0x80344 | `(unknown callee) 0x1d444` | `0x80` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x80368 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `0xffff` | `sp-668` |
| `bl` at 0x80384 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x803b0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x803b8 | `(unknown callee) 0xceae4` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[[sp-24]+32]:1` |
| `bl` at 0x803f4 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x80420 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80454 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x80460 | `(unknown callee) 0xcebfc` | `[sp-1308]` | `?` | `?` | `?` |
| `bl` at 0x8048c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x804a4 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x804c0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x804ec | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x804f4 | `(unknown callee) 0xceae4` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[[sp-24]+32]:1` |
| `bl` at 0x80530 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8055c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80590 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x805a8 | `(unknown callee) 0xcecb8` | `[sp-1316]` | `sp-745` | `?` | `sp-745` |
| `bl` at 0x805d4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x805ec | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x80608 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x80620 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x80664 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x806a0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x806bc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x80738 | `(unknown callee) 0xaadf4` | `[sp-24]` | `?` | `[sp-24]` | `?` |
| `bl` at 0x8075c | `(unknown callee) 0xca70c` | `0x0` | `[sp-1324]` | `sp-745` | `sp-745` |
| `bl` at 0x80788 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x807a0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x807b8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x807dc | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x80814 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80848 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x80890 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x808ac | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8094c | `(unknown callee) 0xaadf4` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x809e0 | `(unknown callee) 0xaadf4` | `[sp-24]` | `?` | `[sp-756]` | `?` |
| `bl` at 0x80a74 | `(unknown callee) 0xaadf4` | `[sp-24]` | `?` | `[sp-756]` | `?` |
| `bl` at 0x80a94 | `(unknown callee) 0xca584` | `0x0` | `[sp-1332]` | `[sp-1336]` | `[sp-1340]` |
| `bl` at 0x80ac0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80ad8 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x80af0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x80b2c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80bd4 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x80c08 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x80c2c | `(unknown callee) (unknown target)` | `0x0` | `[sp-1344]` | `[sp-1348]` | `0x0` |
| `bl` at 0x80c58 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80c70 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x80c8c | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x80ca4 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x80cc0 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80cdc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x80ce4 | `(unknown callee) 0x475e0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x80d04 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `ret(0x475e0)` | `sp-668` |
| `bl` at 0x80d30 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x80d94 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x80dfc | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x80e64 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x80eec | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x80f88 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x81020 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x6` | `?` | `?` |
| `bl` at 0x81088 | `(unknown callee) 0x197a8` | `sp-1124` | `0xfee34` | `[sp-1352]` | `[sp-1356]` |
| `bl` at 0x81094 | `(unknown callee) 0x193f4` | `sp-1124` | `?` | `?` | `sp-1124` |
| `bl` at 0x810b0 | `(unknown callee) 0x1e958` | `0xfee84` | `sp-1124` | `?` | `sp-1124` |
| `bl` at 0x810c8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x810e4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x810ec | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x81100 | `(unknown callee) 0x196e8` | `sp-756` | `?` | `?` | `sp-756` |
| `bl` at 0x8112c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[ret(0x196e8)+20]+1900` | `[ret(0x196e8)+20]+1900` |
| `bl` at 0x8114c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-736]+16]+1` | `[[sp-736]+16]+1` |
| `bl` at 0x81168 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-736]+12]` | `sp-668` |
| `bl` at 0x81184 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-736]+8]` | `sp-668` |
| `bl` at 0x811a0 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-736]+4]` | `sp-668` |
| `bl` at 0x811bc | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[[sp-736]]` | `sp-668` |
| `bl` at 0x811e8 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x812bc | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x81308 | `(unknown callee) 0x4cce8` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x81334 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8134c | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x81368 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x81380 | `(unknown callee) 0x5eb28` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `mov` at 0x813b8 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-668` | `sp-664` | `[0x1338f0]` |
| `bl` at 0x813d4 | `(unknown callee) 0x5eac8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x813fc | `(unknown callee) 0xaa090` | `0x29a6ac` | `0x0` | `[sp-24]` | `[sp-668]` |
| `bl` at 0x81418 | `(unknown callee) 0x1f8e0` | `ret(0xaa090)` | `?` | `?` | `ret(0xaa090)` |
| `bl` at 0x8143c | `(unknown callee) 0x200a0` | `0x6` | `0xfebf0` | `0x6c3` | `0x0` |
| `bl` at 0x81444 | `(unknown callee) 0x2105c` | `0x29a658` | `?` | `?` | `?` |
| `bl` at 0x8147c | `(unknown callee) 0x200a0` | `0x9` | `0xfebf0` | `0x6cc` | `0x0` |
| `bl` at 0x81484 | `(unknown callee) 0xa8b98` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7cb9c | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `bne` at 0x7cbd0 | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x7cc00 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x7cc40 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x7ccb0 | `[0x132638] == 0x0` | `[0x132638] != 0x0` |
| `bhi` at 0x7ccf4 | `[sp-680] > 0x1005 (unsigned, 32-bit)` | `[sp-680] <= 0x1005 (unsigned, 32-bit)` |
| `bhs` at 0x7cd04 | `[sp-680] >= 0x1004 (unsigned, 32-bit)` | `[sp-680] < 0x1004 (unsigned, 32-bit)` |
| `beq` at 0x7cd10 | `[sp-680] == 0xb` | `[sp-680] != 0xb` |
| `bhi` at 0x7cd1c | `[sp-680] > 0xb (unsigned, 32-bit)` | `[sp-680] <= 0xb (unsigned, 32-bit)` |
| `beq` at 0x7cd28 | `[sp-680] == 0x5` | `[sp-680] != 0x5` |
| `bhi` at 0x7cd34 | `[sp-680] > 0x5 (unsigned, 32-bit)` | `[sp-680] <= 0x5 (unsigned, 32-bit)` |
| `bhi` at 0x7cd40 | `[sp-680] > 0x3 (unsigned, 32-bit)` | `[sp-680] <= 0x3 (unsigned, 32-bit)` |
| `bhs` at 0x7cd4c | `[sp-680] >= 0x2 (unsigned, 32-bit)` | `[sp-680] < 0x2 (unsigned, 32-bit)` |
| `beq` at 0x7cd58 | `[sp-680] == 0x1` | `[sp-680] != 0x1` |
| `beq` at 0x7cd68 | `[sp-680] == 0x8` | `[sp-680] != 0x8` |
| `bhi` at 0x7cd74 | `[sp-680] > 0x8 (unsigned, 32-bit)` | `[sp-680] <= 0x8 (unsigned, 32-bit)` |
| `beq` at 0x7cd80 | `[sp-680] == 0x6` | `[sp-680] != 0x6` |
| `beq` at 0x7cd8c | `[sp-680] == 0x7` | `[sp-680] != 0x7` |
| `beq` at 0x7cd9c | `[sp-680] == 0x9` | `[sp-680] != 0x9` |
| `beq` at 0x7cda8 | `[sp-680] == 0xa` | `[sp-680] != 0xa` |
| `beq` at 0x7cdb8 | `[sp-680] == 0x11` | `[sp-680] != 0x11` |
| `bhi` at 0x7cdc4 | `[sp-680] > 0x11 (unsigned, 32-bit)` | `[sp-680] <= 0x11 (unsigned, 32-bit)` |
| `beq` at 0x7cdd0 | `[sp-680] == 0xe` | `[sp-680] != 0xe` |
| `bhi` at 0x7cddc | `[sp-680] > 0xe (unsigned, 32-bit)` | `[sp-680] <= 0xe (unsigned, 32-bit)` |
| `beq` at 0x7cde8 | `[sp-680] == 0xc` | `[sp-680] != 0xc` |
| `beq` at 0x7cdf4 | `[sp-680] == 0xd` | `[sp-680] != 0xd` |
| `beq` at 0x7ce04 | `[sp-680] == 0xf` | `[sp-680] != 0xf` |
| `beq` at 0x7ce10 | `[sp-680] == 0x10` | `[sp-680] != 0x10` |
| `beq` at 0x7ce24 | `[sp-680] == 0x1001` | `[sp-680] != 0x1001` |
| `bhi` at 0x7ce34 | `[sp-680] > 0x1001 (unsigned, 32-bit)` | `[sp-680] <= 0x1001 (unsigned, 32-bit)` |
| `beq` at 0x7ce40 | `[sp-680] == 0x12` | `[sp-680] != 0x12` |
| `beq` at 0x7ce4c | `[sp-680] == 0x13` | `[sp-680] != 0x13` |
| `beq` at 0x7ce60 | `[sp-680] == 0x1002` | `[sp-680] != 0x1002` |
| `beq` at 0x7ce70 | `[sp-680] == 0x1003` | `[sp-680] != 0x1003` |
| `beq` at 0x7ce84 | `[sp-680] == 0x2005` | `[sp-680] != 0x2005` |
| `bhi` at 0x7ce94 | `[sp-680] > 0x2005 (unsigned, 32-bit)` | `[sp-680] <= 0x2005 (unsigned, 32-bit)` |
| `beq` at 0x7cea4 | `[sp-680] == 0x100b` | `[sp-680] != 0x100b` |
| `bhi` at 0x7ceb4 | `[sp-680] > 0x100b (unsigned, 32-bit)` | `[sp-680] <= 0x100b (unsigned, 32-bit)` |
| `beq` at 0x7cec4 | `[sp-680] == 0x1008` | `[sp-680] != 0x1008` |
| `bhi` at 0x7ced4 | `[sp-680] > 0x1008 (unsigned, 32-bit)` | `[sp-680] <= 0x1008 (unsigned, 32-bit)` |
| `beq` at 0x7cee4 | `[sp-680] == 0x1006` | `[sp-680] != 0x1006` |
| `beq` at 0x7cef4 | `[sp-680] == 0x1007` | `[sp-680] != 0x1007` |
| `beq` at 0x7cf08 | `[sp-680] == 0x1009` | `[sp-680] != 0x1009` |
| `beq` at 0x7cf18 | `[sp-680] == 0x100a` | `[sp-680] != 0x100a` |
| `beq` at 0x7cf2c | `[sp-680] == 0x2002` | `[sp-680] != 0x2002` |
| `bhi` at 0x7cf3c | `[sp-680] > 0x2002 (unsigned, 32-bit)` | `[sp-680] <= 0x2002 (unsigned, 32-bit)` |
| `beq` at 0x7cf4c | `[sp-680] == 0x100c` | `[sp-680] != 0x100c` |
| `beq` at 0x7cf5c | `[sp-680] == 0x2001` | `[sp-680] != 0x2001` |
| `beq` at 0x7cf70 | `[sp-680] == 0x2003` | `[sp-680] != 0x2003` |
| `beq` at 0x7cf80 | `[sp-680] == 0x2004` | `[sp-680] != 0x2004` |
| `beq` at 0x7cf94 | `[sp-680] == 0x200d` | `[sp-680] != 0x200d` |
| `bhi` at 0x7cfa4 | `[sp-680] > 0x200d (unsigned, 32-bit)` | `[sp-680] <= 0x200d (unsigned, 32-bit)` |
| `beq` at 0x7cfb4 | `[sp-680] == 0x200a` | `[sp-680] != 0x200a` |
| `bhi` at 0x7cfc4 | `[sp-680] > 0x200a (unsigned, 32-bit)` | `[sp-680] <= 0x200a (unsigned, 32-bit)` |
| `beq` at 0x7cfd4 | `[sp-680] == 0x2006` | `[sp-680] != 0x2006` |
| `beq` at 0x7cfe4 | `[sp-680] == 0x2009` | `[sp-680] != 0x2009` |
| `beq` at 0x7cff8 | `[sp-680] == 0x200b` | `[sp-680] != 0x200b` |
| `beq` at 0x7d008 | `[sp-680] == 0x200c` | `[sp-680] != 0x200c` |
| `beq` at 0x7d01c | `[sp-680] == 0x3001` | `[sp-680] != 0x3001` |
| `bhi` at 0x7d02c | `[sp-680] > 0x3001 (unsigned, 32-bit)` | `[sp-680] <= 0x3001 (unsigned, 32-bit)` |
| `beq` at 0x7d03c | `[sp-680] == 0x200e` | `[sp-680] != 0x200e` |
| `beq` at 0x7d04c | `[sp-680] == 0x200f` | `[sp-680] != 0x200f` |
| `beq` at 0x7d060 | `[sp-680] == 0x4001` | `[sp-680] != 0x4001` |
| `bhi` at 0x7d070 | `[sp-680] > 0x4001 (unsigned, 32-bit)` | `[sp-680] <= 0x4001 (unsigned, 32-bit)` |
| `beq` at 0x7d07c | `[sp-680] == 0x4000` | `[sp-680] != 0x4000` |
| `beq` at 0x7d090 | `[sp-680] == 0x4002` | `[sp-680] != 0x4002` |
| `bne` at 0x7d138 | `[sp-680] != 0x3` | `[sp-680] == 0x3` |
| `beq` at 0x7d164 | `[sp-676] == 0x3d` | `[sp-676] != 0x3d` |
| `bne` at 0x7d174 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x7d204 | `[ret(0x6a0f8)+16] != 0x3d` | `[ret(0x6a0f8)+16] == 0x3d` |
| `bne` at 0x7d210 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x7d2a0 | `[ret(0x6a0f8)+16] != 0x3d` | `[ret(0x6a0f8)+16] == 0x3d` |
| `bne` at 0x7d2ac | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x7d33c | `[ret(0x6a0f8)+16] != 0x3d` | `[ret(0x6a0f8)+16] == 0x3d` |
| `bne` at 0x7d348 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x7d3d8 | `[ret(0x6a0f8)+16] != 0x3d` | `[ret(0x6a0f8)+16] == 0x3d` |
| `bne` at 0x7d3e4 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x7d524 | `[sp-676] != 0x40` | `[sp-676] == 0x40` |
| `bne` at 0x7d554 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x7d684 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x7d6b4 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x7d6e4 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x7d758 | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `blt` at 0x7d78c | `[sp-716] < [sp-720] (signed, 32-bit)` | `[sp-716] >= [sp-720] (signed, 32-bit)` |
| `beq` at 0x7d824 | `[sp-1164] == 0x1` | `[sp-1164] != 0x1` |
| `beq` at 0x7d830 | `[sp-1164] == 0x2` | `[sp-1164] != 0x2` |
| `bhi` at 0x7da38 | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x7da68 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x7da98 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x7dae8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7db24 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7db60 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7dc78 | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `blt` at 0x7dca4 | `[sp-716] < [sp-744] (signed, 32-bit)` | `[sp-716] >= [sp-744] (signed, 32-bit)` |
| `ldrls` at 0x7dccc | `[sp-724] <= 0x4 (unsigned, 32-bit)` | `[sp-724] > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x7e110 | `[sp-736] == 0x0` | `[sp-736] != 0x0` |
| `blo` at 0x7e120 | `[sp-728] < [sp-740] (unsigned, 32-bit)` | `[sp-728] >= [sp-740] (unsigned, 32-bit)` |
| `bls` at 0x7e154 | `([sp-728] + [sp-732]) <= [sp-740] (unsigned, 32-bit)` | `([sp-728] + [sp-732]) > [sp-740] (unsigned, 32-bit)` |
| `beq` at 0x7e1c4 | `[sp-732] == 0x0` | `[sp-732] != 0x0` |
| `beq` at 0x7e200 | `[0x29768c] == 0x0` | `[0x29768c] != 0x0` |
| `bne` at 0x7e240 | `ret(0x19538) != 0x0` | `ret(0x19538) == 0x0` |
| `ldrls` at 0x7e358 | `[sp-676]-48 <= 0x4c (unsigned, 32-bit)` | `[sp-676]-48 > 0x4c (unsigned, 32-bit)` |
| `bne` at 0x7e6f4 | `ret(0x54224) != 0x0` | `ret(0x54224) == 0x0` |
| `bne` at 0x7e72c | `[0x297634] != 0x0` | `[0x297634] == 0x0` |
| `bge` at 0x7e7b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x7e81c | `ret(0x54244) == 0x0` | `ret(0x54244) != 0x0` |
| `beq` at 0x7e870 | `ret(0x542cc) == 0x0` | `ret(0x542cc) != 0x0` |
| `bge` at 0x7e918 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0x7e9cc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x7eaf8 | `ret(0xd7168) == 0x0` | `ret(0xd7168) != 0x0` |
| `bhi` at 0x7eb24 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x7eb68 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ec10 | `[sp-672] == 0x0` | `[sp-672] != 0x0` |
| `bhi` at 0x7ec5c | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x7ec8c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x7ece4 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x7ed10 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x7ed4c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x7edfc | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x7ee2c | `? == 0x2` | `? != 0x2` |
| `bne` at 0x7ee5c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x7eeb8 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x7eee8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x7ef54 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x7ef84 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x7efdc | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x7f014 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x7f090 | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x7f0c0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f0fc | `[sp-1240] == 0x1` | `[sp-1240] != 0x1` |
| `blo` at 0x7f108 | `[sp-1240] < 0x1 (unsigned, 32-bit)` | `[sp-1240] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x7f114 | `[sp-1240] == 0x2` | `[sp-1240] != 0x2` |
| `bne` at 0x7f12c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f170 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f1b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f1e4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f274 | `[[sp-24]+32]:1 == 0x4` | `[[sp-24]+32]:1 != 0x4` |
| `bne` at 0x7f2a4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f300 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f360 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f40c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x7f430 | `ret(0x781b8) != ?` | `ret(0x781b8) == ?` |
| `beq` at 0x7f448 | `[[sp-24]+32]:1 == 0x3` | `[[sp-24]+32]:1 != 0x3` |
| `bne` at 0x7f494 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f4e4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f538 | `ret(0x47ad8) == 0x0` | `ret(0x47ad8) != 0x0` |
| `bls` at 0x7f57c | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x7f58c | `[0x1338f4] != 0x0` | `[0x1338f4] == 0x0` |
| `bne` at 0x7f5bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f5fc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f654 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x7f698 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x7f6a8 | `[0x1338f8] != 0x0` | `[0x1338f8] == 0x0` |
| `bne` at 0x7f6d8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f718 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f770 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x7f7b4 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x7f7c4 | `[0x1338fc] != 0x0` | `[0x1338fc] == 0x0` |
| `bne` at 0x7f7f4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7f834 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f88c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x7f8d8 | `ret(0x37174) == 0x0` | `ret(0x37174) != 0x0` |
| `beq` at 0x7f924 | `ret(0x37558) == 0x0` | `ret(0x37558) != 0x0` |
| `bne` at 0x7f97c | `ret(0xc5738) != 0x0` | `ret(0xc5738) == 0x0` |
| `bge` at 0x7f9c8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x7fa2c | `[[sp-760]] != 0x1` | `[[sp-760]] == 0x1` |
| `bne` at 0x7fa4c | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `beq` at 0x7fa70 | `[[sp-760]+28] == 0x0` | `[[sp-760]+28] != 0x0` |
| `beq` at 0x7fac8 | `ret(0x814c0) == 0x0` | `ret(0x814c0) != 0x0` |
| `bne` at 0x7fb54 | `[[sp-760]] != 0x2` | `[[sp-760]] == 0x2` |
| `bne` at 0x7fb74 | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `beq` at 0x7fb98 | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `bgt` at 0x7fbf8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0x7fc4c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0x7fcbc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x7fce4 | `ret(0x192d4) == 0x0` | `ret(0x192d4) != 0x0` |
| `beq` at 0x7fd78 | `[[[sp-760]+12]+64]:1 == 0x2c` | `[[[sp-760]+12]+64]:1 != 0x2c` |
| `beq` at 0x7fda0 | `ret(0x192d4) == 0x0` | `ret(0x192d4) != 0x0` |
| `bne` at 0x7fdc8 | `ret(0x192d4) != 0x0` | `ret(0x192d4) == 0x0` |
| `beq` at 0x7fef4 | `[0x133908] == 0x0` | `[0x133908] != 0x0` |
| `bne` at 0x7ff38 | `[[sp-760]] != 0x3` | `[[sp-760]] == 0x3` |
| `bne` at 0x7ff58 | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `beq` at 0x7ff7c | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `beq` at 0x8003c | `[[sp-760]+32] == 0x0` | `[[sp-760]+32] != 0x0` |
| `beq` at 0x80070 | `[[sp-760]+44] == 0x0` | `[[sp-760]+44] != 0x0` |
| `beq` at 0x800a4 | `[[sp-760]+64] == 0x0` | `[[sp-760]+64] != 0x0` |
| `bgt` at 0x80104 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x80140 | `ret(0x192d4) == 0x0` | `ret(0x192d4) != 0x0` |
| `blt` at 0x801bc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x80274 | `[sp-728] == 0x0` | `[sp-728] != 0x0` |
| `beq` at 0x80320 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x80350 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `bhi` at 0x80398 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x803d0 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x803e4 | `[[sp-24]+48] >= ret(0xceae4) (unsigned, 32-bit)` | `[[sp-24]+48] < ret(0xceae4) (unsigned, 32-bit)` |
| `bhs` at 0x80404 | `ret(0xaadf4) >= [sp-1304] (unsigned, 32-bit)` | `ret(0xaadf4) < [sp-1304] (unsigned, 32-bit)` |
| `bne` at 0x80438 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x80474 | `ret(0xcebfc) == 0x0` | `ret(0xcebfc) != 0x0` |
| `bne` at 0x804d4 | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x8050c | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x80520 | `[[sp-24]+48] >= ret(0xceae4) (unsigned, 32-bit)` | `[[sp-24]+48] < ret(0xceae4) (unsigned, 32-bit)` |
| `bhs` at 0x80540 | `ret(0xaadf4) >= [sp-1312] (unsigned, 32-bit)` | `ret(0xaadf4) < [sp-1312] (unsigned, 32-bit)` |
| `bne` at 0x80574 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x805bc | `ret(0xcecb8) == 0x0` | `ret(0xcecb8) != 0x0` |
| `bls` at 0x80634 | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x80648 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80684 | `[sp-1320] != [[sp-24]+32]:1-2` | `[sp-1320] == [[sp-24]+32]:1-2` |
| `bge` at 0x806d4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x806fc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x80770 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `bhi` at 0x807fc | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x8082c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x80878 | `? == [[sp-24]+32]:1` | `? != [[sp-24]+32]:1` |
| `bge` at 0x808c4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x808f8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8098c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80a20 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x80aa8 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |
| `bhi` at 0x80b14 | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x80b3c | `[sp-676] == 0x35` | `[sp-676] != 0x35` |
| `beq` at 0x80b48 | `[sp-676] == 0x40` | `[sp-676] != 0x40` |
| `beq` at 0x80b54 | `[sp-676] == 0x4f` | `[sp-676] != 0x4f` |
| `beq` at 0x80b60 | `[sp-676] == 0x42` | `[sp-676] != 0x42` |
| `beq` at 0x80b6c | `[sp-676] == 0x46` | `[sp-676] != 0x46` |
| `beq` at 0x80b78 | `[sp-676] == 0x4c` | `[sp-676] != 0x4c` |
| `beq` at 0x80b84 | `[sp-676] == 0x4d` | `[sp-676] != 0x4d` |
| `beq` at 0x80b90 | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `beq` at 0x80ba4 | `[0x133908] == 0x0` | `[0x133908] != 0x0` |
| `bne` at 0x80bb8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80bec | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x80c40 | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `bhi` at 0x80d18 | `[[sp-24]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bne` at 0x80d48 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80ddc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80e20 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80ea8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80f30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x80fdc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x810a0 | `ret(0x193f4) == 0x0` | `ret(0x193f4) != 0x0` |
| `beq` at 0x811d0 | `[[sp-24]+32]:1 == 0x1` | `[[sp-24]+32]:1 != 0x1` |
| `bge` at 0x81208 | `? >= [0x29a5e4] (signed, 32-bit)` | `? < [0x29a5e4] (signed, 32-bit)` |
| `bne` at 0x81250 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x812a0 | `? != 0x4` | `? == 0x4` |
| `bge` at 0x8131c | `ret(0x4cce8) >= 0x0 (signed, 32-bit)` | `ret(0x4cce8) < 0x0 (signed, 32-bit)` |
| `beq` at 0x81394 | `[0x1338f0] == 0x0` | `[0x1338f0] != 0x0` |
| `beq` at 0x813e0 | `[sp-668] == 0x0` | `[sp-668] != 0x0` |
| `beq` at 0x81410 | `ret(0xaa090) == 0x0` | `ret(0xaa090) != 0x0` |
| `beq` at 0x81458 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8148c | `r0` | `ret(0xa8b98)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7dccc).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7e358).

## `syscom_process_inbound_data`

- `syscom_process_inbound_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x786d8 to 0x7881c.
  Established. The unnamed `0x5ed50` call carries `sp-80`, `sp-76`, `[sp-32]`, `[sp-36]` (`bl` at 0x78728).
  A zero `ret(0x7713c)` takes `0x78758`, while nonzero falls through (`beq` at 0x78740).
  An included word store writes `[0x29a5d4]` to an unknown target (`str` at 0x7876c).
  Inbound data framing meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x786e8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x786ec | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x786f0 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x78718 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x78720 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78724 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7872c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78744 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78748 | load | 4 | `abs 0x78810` | `-` |
| `ldr` at 0x7874c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78750 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78764 | load | 4 | `abs 0x78814` | `-` |
| `ldr` at 0x78768 | load | 4 | `abs 0x29a5d4` | `-` |
| `str` at 0x7876c | store | 4 | `(unknown)` | `[0x29a5d4]` |
| `ldr` at 0x78788 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x78790 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x787ac | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x787b8 | load | 4 | `abs 0x78814` | `-` |
| `ldr` at 0x787bc | load | 4 | `abs 0x29a5ec` | `-` |
| `str` at 0x787c0 | store | 4 | `sp -0x58` | `[0x29a5ec]` |
| `ldr` at 0x787cc | load | 4 | `sp -0x50` | `-` |
| `str` at 0x787dc | store | 4 | `sp -0x54` | `ret(0xaa20c)` |
| `ldr` at 0x787e0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x787ec | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x787f8 | load | 4 | `abs 0x78818` | `-` |
| `ldr` at 0x787fc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x7880c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x7880c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7880c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7880c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7880c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78708 | `(unknown callee) 0x5eac8` | `sp-80` | `sp-76` | `0x0` | `sp-80` |
| `bl` at 0x78728 | `(unknown callee) 0x5ed50` | `sp-80` | `sp-76` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x78734 | `(unknown callee) 0x7713c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0x78754 | `(unknown callee) 0x771d8` | `[sp-28]` | `0xfe824` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x78798 | `(unknown callee) 0x783d8` | `[sp-28]` | `0x4` | `sp-24` | `?` |
| `bl` at 0x787d4 | `(unknown callee) 0xaa20c` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x787f0 | `(unknown callee) 0x1f8e0` | `ret(0xaa20c)` | `?` | `?` | `ret(0xaa20c)` |
| `bl` at 0x78804 | `(unknown callee) 0x1e958` | `0xfe834` | `[sp-28]` | `ret(0x1f8e0)` | `ret(0x1f8e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x78740 | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `beq` at 0x787e8 | `ret(0xaa20c) == 0x0` | `ret(0xaa20c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7880c | `r0` | `?` |

## `syscom_process_inbound_request`

- `syscom_process_inbound_request`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7881c to 0x78bfc.
  Established. The byte loads include `[sp-32] +0x1`, `+0x2`, `+0x3`, and `+0x4` (`ldrb` at 0x78880, `ldrb` at 0x78894, `ldrb` at 0x788ac, `ldrb` at 0x788c4).
  A zero `[0x133830]` takes `0x789f8`, while nonzero falls through (`beq` at 0x789d4).
  The included indirect call carries `[sp-28]`, `[sp-144]`, `[sp-32]`, `[sp-36]` (`mov` at 0x789f4).
  Request field decoding meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7882c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x78830 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x78834 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x7883c | store | 4 | `sp -0x80` | `0x1` |
| `str` at 0x78844 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x78848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78860 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78864 | load | 4 | `abs 0x78bec` | `-` |
| `ldr` at 0x78868 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7886c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78878 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78880 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x7888c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78894 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x788a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x788ac | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x788bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x788c4 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x788cc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x788d4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x788dc | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x788e8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x788f0 | load | 1 | `[sp-32] +0x6` | `-` |
| `str` at 0x788f8 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x788fc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x78910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78914 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78918 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78930 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78938 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x78944 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7894c | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x7896c | load | 4 | `abs 0x78bf0` | `-` |
| `ldr` at 0x78970 | load | 4 | `abs 0x29a5d4` | `-` |
| `str` at 0x78974 | store | 4 | `(unknown)` | `[0x29a5d4]` |
| `ldr` at 0x78978 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7897c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x78984 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x789a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x789a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x789ac | load | 4 | `sp -0x90` | `-` |
| `str` at 0x789c4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x789c8 | load | 4 | `abs 0x78bf4` | `-` |
| `ldr` at 0x789cc | load | 4 | `abs 0x133830` | `-` |
| `ldr` at 0x789d8 | load | 4 | `abs 0x78bf4` | `-` |
| `ldr` at 0x789dc | load | 4 | `abs 0x133830` | `-` |
| `ldr` at 0x789e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x789e4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x789e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x789ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x789f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78a08 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78a10 | load | 1 | `[sp-32] +0x7` | `-` |
| `str` at 0x78a14 | store | 4 | `sp -0x94` | `[[sp-32]+7]:1` |
| `ldr` at 0x78a18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78a20 | store | 4 | `sp -0x20` | `[sp-32]+8` |
| `ldr` at 0x78a24 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x78a2c | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `ldr` at 0x78a38 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78a3c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x78a40 | store | 4 | `sp -0x98` | `[[sp-32]]:1` |
| `ldr` at 0x78a44 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x78a50 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x78a6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78a74 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x78a80 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78a88 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x78a98 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78aa0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x78ab0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78ab8 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x78acc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78ad4 | store | 4 | `sp -0x20` | `[sp-32]+5` |
| `ldr` at 0x78adc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78af4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78b00 | store | 4 | `sp -0xa8` | `0x1` |
| `ldr` at 0x78b14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78b24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78b30 | store | 4 | `sp -0x20` | `(ret(0x19538) + [sp-32])+1` |
| `ldr` at 0x78b40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78b48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78b4c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x78b60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78b64 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x78b6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78b74 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x78b84 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x78b88 | store | 4 | `sp -0xa8` | `[sp-128]` |
| `str` at 0x78b90 | store | 4 | `sp -0xa4` | `sp-124` |
| `str` at 0x78b98 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x78ba0 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x78bb8 | store | 4 | `sp -0x84` | `ret(0xa9f4c)` |
| `ldr` at 0x78bbc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x78bc8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x78bd4 | load | 4 | `abs 0x78bf8` | `-` |
| `ldr` at 0x78bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x78be8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x78be8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x78be8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x78be8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x78be8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78850 | `(unknown callee) 0x7713c` | `r0` | `0x2` | `r2` | `0x0` |
| `bl` at 0x78870 | `(unknown callee) 0x771d8` | `[sp-28]` | `0xfe85c` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x7891c | `(unknown callee) 0x7a7bc` | `[sp-28]` | `[sp-32]` | `[sp-36]` | `[sp-140]-12289` |
| `bl` at 0x7895c | `(unknown callee) 0x5eac8` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x789b0 | `(unknown callee) 0x783d8` | `[sp-28]` | `[sp-144]` | `sp-24` | `?` |
| `mov` at 0x789f4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-144]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x78ac8 | `(unknown callee) 0x5eb28` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x78ae0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0x78b10 | `(unknown callee) 0x5ed50` | `sp-128` | `sp-124` | `[sp-32]+1` | `ret(0x19538)+1` |
| `bl` at 0x78b18 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x78b5c | `(unknown callee) 0x5ebd0` | `sp-128` | `sp-124` | `[sp-32]+1` | `?` |
| `bl` at 0x78bb0 | `(unknown callee) 0xa9f4c` | `sp-44` | `sp-136` | `0x0` | `0x0` |
| `bl` at 0x78bcc | `(unknown callee) 0x1f8e0` | `ret(0xa9f4c)` | `?` | `?` | `ret(0xa9f4c)` |
| `bl` at 0x78be0 | `(unknown callee) 0x1e958` | `0xfe86c` | `[sp-28]` | `ret(0x1f8e0)` | `ret(0x1f8e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7885c | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `bhi` at 0x7890c | `[sp-140]-12289 > 0x2 (unsigned, 32-bit)` | `[sp-140]-12289 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x789d4 | `[0x133830] == 0x0` | `[0x133830] != 0x0` |
| `bls` at 0x78a00 | `[sp-36] <= 0x7 (unsigned, 32-bit)` | `[sp-36] > 0x7 (unsigned, 32-bit)` |
| `beq` at 0x78a34 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x78a4c | `[[sp-32]]:1 == 0x0` | `[[sp-32]]:1 != 0x0` |
| `beq` at 0x78a58 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0x78bc4 | `ret(0xa9f4c) == 0x0` | `ret(0xa9f4c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x78be8 | `r0` | `?` |

## `syscom_process_inbound_event`

- `syscom_process_inbound_event`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78bfc to 0x78f20.
  Established. The unnamed `0x5ed50` call carries `sp-80`, `sp-76`, `[sp-32]+1`, `[sp-36]-1` (`bl` at 0x78d1c).
  A zero `[0x13383c]` takes `0x78e78`, while nonzero falls through (`beq` at 0x78e4c).
  An included word store writes `[0x29a5d4]` to an unknown effective target (`str` at 0x78dfc).
  Inbound event forwarding rules remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78c0c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x78c10 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x78c14 | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x78c18 | load | 4 | `abs 0x78f0c` | `-` |
| `ldr` at 0x78c1c | load | 4 | `abs 0x297634` | `-` |
| `str` at 0x78c20 | store | 4 | `sp -0x74` | `[0x297634]` |
| `ldr` at 0x78c24 | load | 4 | `abs 0x78f0c` | `-` |
| `ldr` at 0x78c28 | load | 4 | `abs 0x297634` | `-` |
| `str` at 0x78c38 | store | 4 | `sp -0x74` | `0x1` |
| `ldr` at 0x78c3c | load | 4 | `sp -0x74` | `-` |
| `str` at 0x78c40 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x78c44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78c4c | store | 4 | `sp -0x68` | `r1+1` |
| `ldr` at 0x78c50 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x78c58 | store | 1 | `sp -0x69` | `(r2-1 & 0xff)` |
| `str` at 0x78c60 | store | 4 | `sp -0x70` | `0x0` |
| `ldrb` at 0x78c64 | load | 1 | `sp -0x69` | `-` |
| `strb` at 0x78c6c | store | 1 | `sp -0x69` | `?` |
| `ldr` at 0x78c7c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x78c80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x78c8c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x78c94 | load | 1 | `sp -0x69` | `-` |
| `ldr` at 0x78c98 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x78ca4 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x78cac | store | 4 | `sp -0x70` | `[sp-112]+1` |
| `ldr` at 0x78cb4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x78cbc | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x78cc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78cc8 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x78cd0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x78cdc | load | 4 | `abs 0x78f10` | `-` |
| `ldr` at 0x78ce0 | load | 4 | `abs 0x29a5dc` | `-` |
| `ldr` at 0x78ce8 | load | 4 | `[0x29a5dc] +0x3c` | `-` |
| `ldr` at 0x78cf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78d00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x78d0c | store | 4 | `sp -0x78` | `0x1` |
| `ldr` at 0x78d20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78d38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78d3c | load | 4 | `abs 0x78f14` | `-` |
| `ldr` at 0x78d40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78d44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78d4c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x78d50 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x78d58 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x78d5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78d68 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x78d70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78d74 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x78d80 | store | 4 | `sp -0x60` | `ret(0x783d8)` |
| `str` at 0x78d88 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x78d8c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x78d90 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x78d9c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x78db8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78dc8 | store | 4 | `sp -0x60` | `ret(0x783d8)` |
| `ldr` at 0x78dd0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78dd8 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x78de0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x78df4 | load | 4 | `abs 0x78f10` | `-` |
| `ldr` at 0x78df8 | load | 4 | `abs 0x29a5d4` | `-` |
| `str` at 0x78dfc | store | 4 | `(unknown)` | `[0x29a5d4]` |
| `ldr` at 0x78e18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x78e20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78e24 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x78e3c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x78e40 | load | 4 | `abs 0x78f18` | `-` |
| `ldr` at 0x78e44 | load | 4 | `abs 0x13383c` | `-` |
| `ldr` at 0x78e50 | load | 4 | `abs 0x78f18` | `-` |
| `ldr` at 0x78e54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78e5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78e64 | load | 4 | `abs 0x13383c` | `-` |
| `ldr` at 0x78e68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78e98 | load | 4 | `abs 0x78f10` | `-` |
| `ldr` at 0x78e9c | load | 4 | `abs 0x29a5ec` | `-` |
| `str` at 0x78ea0 | store | 4 | `sp -0x78` | `[0x29a5ec]` |
| `ldr` at 0x78eac | load | 4 | `sp -0x50` | `-` |
| `str` at 0x78ebc | store | 4 | `sp -0x58` | `ret(0xaa20c)` |
| `ldr` at 0x78ec0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x78ecc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x78ed8 | load | 4 | `abs 0x78f1c` | `-` |
| `ldr` at 0x78edc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78ee8 | load | 4 | `abs 0x78f10` | `-` |
| `ldr` at 0x78eec | load | 4 | `abs 0x29a644` | `-` |
| `ldr` at 0x78ef8 | load | 4 | `abs 0x78f10` | `-` |
| `ldr` at 0x78efc | load | 4 | `abs 0x29a64c` | `-` |
| `ldm` at 0x78f08 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x78f08 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x78f08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x78f08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x78f08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78ca0 | `(unknown callee) 0x19610` | `?` | `?` | `?` | `?` |
| `bl` at 0x78cec | `(unknown callee) 0x5eac8` | `sp-80` | `sp-76` | `[[0x29a5dc]+60]` | `[0x29a5dc]` |
| `bl` at 0x78d1c | `(unknown callee) 0x5ed50` | `sp-80` | `sp-76` | `[sp-32]+1` | `[sp-36]-1` |
| `bl` at 0x78d28 | `(unknown callee) 0x7713c` | `[sp-28]` | `0x4` | `?` | `?` |
| `bl` at 0x78d48 | `(unknown callee) 0x771d8` | `[sp-28]` | `0xfe898` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x78d6c | `(unknown callee) 0x55300` | `[sp-28]+2` | `[sp-92]` | `?` | `[sp-28]+2` |
| `bl` at 0x78d78 | `(unknown callee) 0x783d8` | `[sp-28]` | `[sp-92]` | `?` | `?` |
| `bl` at 0x78da8 | `(unknown callee) 0x554b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x78dc0 | `(unknown callee) 0x783d8` | `[sp-84]` | `0x4` | `?` | `ret(0x554b8)` |
| `bl` at 0x78de4 | `(unknown callee) 0x5d778` | `?` | `?` | `?` | `?` |
| `bl` at 0x78e28 | `(unknown callee) 0x783d8` | `[sp-28]` | `[sp-92]` | `sp-24` | `?` |
| `mov` at 0x78e74 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-32]+1` | `[sp-36]-1` | `[sp-36]-1` |
| `bl` at 0x78e7c | `(unknown callee) 0x1d444` | `0x81` | `?` | `?` | `?` |
| `bl` at 0x78eb4 | `(unknown callee) 0xaa20c` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x78ed0 | `(unknown callee) 0x1f8e0` | `ret(0xaa20c)` | `?` | `?` | `ret(0xaa20c)` |
| `bl` at 0x78ee4 | `(unknown callee) 0x1e958` | `0xfe8a8` | `[sp-28]` | `ret(0x1f8e0)` | `ret(0x1f8e0)` |
| `bl` at 0x78f00 | `(unknown callee) 0x78f20` | `[0x29a64c]` | `?` | `?` | `0x29a5d4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x78c30 | `[0x297634] != 0x0` | `[0x297634] == 0x0` |
| `beq` at 0x78c78 | `? == 0xff` | `? != 0xff` |
| `bne` at 0x78c88 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x78d34 | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `bge` at 0x78d98 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x78db4 | `ret(0x554b8) != 0x4` | `ret(0x554b8) == 0x4` |
| `beq` at 0x78e4c | `[0x13383c] == 0x0` | `[0x13383c] != 0x0` |
| `beq` at 0x78e88 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x78ec8 | `ret(0xaa20c) == 0x0` | `ret(0xaa20c) != 0x0` |
| `beq` at 0x78ef4 | `[0x29a644] == 0x0` | `[0x29a644] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x78f08 | `r0` | `?` |

## `syscom_process_inbound_packet`

- `syscom_process_inbound_packet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78f48 to 0x79028.
  Established. Included absolute word stores write `[0x133814]+1` to `0x133814` and `[0x13381c]+1` to `0x13381c` (`str` at 0x78fa4, `str` at 0x78fe4).
  Equality `[sp-32] == 0x2` takes `0x78fd4`, while inequality falls through (`beq` at 0x78f8c).
  The unnamed `0x78bfc` call carries `[sp-20]`, `[sp-24]`, `[sp-28]`, `([0x133828] + [sp-28])` (`bl` at 0x7900c).
  Packet category meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78f58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x78f5c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x78f60 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x78f64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x78f68 | load | 1 | `r1 +0x0` | `-` |
| `str` at 0x78f74 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x78f78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78f84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78f94 | load | 4 | `abs 0x79018` | `-` |
| `ldr` at 0x78f98 | load | 4 | `abs 0x79018` | `-` |
| `ldr` at 0x78f9c | load | 4 | `abs 0x133814` | `-` |
| `str` at 0x78fa4 | store | 4 | `abs 0x133814` | `[0x133814]+1` |
| `ldr` at 0x78fa8 | load | 4 | `abs 0x7901c` | `-` |
| `ldr` at 0x78fac | load | 4 | `abs 0x7901c` | `-` |
| `ldr` at 0x78fb0 | load | 4 | `abs 0x133820` | `-` |
| `ldr` at 0x78fb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x78fbc | store | 4 | `abs 0x133820` | `([0x133820] + [sp-28])` |
| `ldr` at 0x78fc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78fc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x78fc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78fd4 | load | 4 | `abs 0x79020` | `-` |
| `ldr` at 0x78fd8 | load | 4 | `abs 0x79020` | `-` |
| `ldr` at 0x78fdc | load | 4 | `abs 0x13381c` | `-` |
| `str` at 0x78fe4 | store | 4 | `abs 0x13381c` | `[0x13381c]+1` |
| `ldr` at 0x78fe8 | load | 4 | `abs 0x79024` | `-` |
| `ldr` at 0x78fec | load | 4 | `abs 0x79024` | `-` |
| `ldr` at 0x78ff0 | load | 4 | `abs 0x133828` | `-` |
| `ldr` at 0x78ff4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x78ffc | store | 4 | `abs 0x133828` | `([0x133828] + [sp-28])` |
| `ldr` at 0x79000 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79008 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x79014 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x79014 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x79014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78fcc | `(unknown callee) 0x7881c` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x133820] + [sp-28])` |
| `bl` at 0x7900c | `(unknown callee) 0x78bfc` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x133828] + [sp-28])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x78f80 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x78f8c | `[sp-32] == 0x2` | `[sp-32] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x79014 | `r0` | `?` |

## `syscom_process_outbound_request`

- `syscom_process_outbound_request`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79298 to 0x79718.
  Established. Equality `ret(0x375e4) == 0x1` takes `0x7934c`, while inequality falls through (`beq` at 0x792b8).
  The word through `[sp-20] +0x18` is read (`ldr` at 0x79350).
  The unnamed `0xaa090` call at `0x796e8` carries `0x29a5d4`, `0x0`, `[sp-20]`, `[sp-120]` (`bl` at 0x796e8).
  Outbound request transport selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x792a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x792d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792d8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x792ec | load | 4 | `abs 0x796f4` | `-` |
| `str` at 0x79318 | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x79330 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x79334 | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x7933c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79340 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x7934c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79350 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x7935c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79360 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x79368 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x7936c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x79374 | store | 4 | `sp -0x8c` | `?` |
| `str` at 0x79380 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x79384 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x79388 | store | 4 | `sp -0x84` | `[sp-140]` |
| `ldr` at 0x7938c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x79398 | store | 4 | `sp -0x88` | `ret(0x781b8)` |
| `ldr` at 0x7939c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x793a8 | store | 4 | `sp -0x84` | `ret(0x78298)` |
| `ldr` at 0x793ac | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x793b0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x793d4 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x793dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x793e8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x793ec | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x793fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79400 | load | 4 | `[sp-20] +0x30` | `-` |
| `strb` at 0x79404 | store | 1 | `sp -0x8d` | `([[sp-20]+48] & 0xff)` |
| `ldr` at 0x7940c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x7941c | store | 1 | `sp -0x8d` | `(ret(0xaadf4) & 0xff)` |
| `ldrb` at 0x79420 | load | 1 | `sp -0x8d` | `-` |
| `strb` at 0x79424 | store | 1 | `sp -0x7c` | `?` |
| `strb` at 0x7942c | store | 1 | `sp -0x7b` | `0x0` |
| `ldr` at 0x79430 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x7944c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x79450 | load | 4 | `abs 0x796fc` | `-` |
| `ldr` at 0x79460 | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x79464 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x79488 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x79494 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79498 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x794b0 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x794b4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x794c4 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x794cc | load | 4 | `sp -0x84` | `-` |
| `strb` at 0x794d0 | store | 1 | `sp -0x7c` | `([sp-132] & 0xff)` |
| `ldr` at 0x794d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x794d8 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x794e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x794ec | load | 4 | `[sp-20] +0x30` | `-` |
| `strb` at 0x794f0 | store | 1 | `sp -0x8e` | `([[sp-20]+48] & 0xff)` |
| `ldr` at 0x794f8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x79508 | store | 1 | `sp -0x8e` | `(ret(0xaadf4) & 0xff)` |
| `ldrb` at 0x7950c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x79510 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0x79514 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x79530 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x79534 | load | 4 | `abs 0x79700` | `-` |
| `ldr` at 0x79544 | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x79548 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x7956c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x79578 | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x7957c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x795a0 | load | 4 | `abs 0x29a5e8` | `-` |
| `str` at 0x795b0 | store | 4 | `sp -0x94` | `0x2` |
| `ldr` at 0x795b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x795bc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x795cc | store | 4 | `sp -0x80` | `ret(0xaddfc)` |
| `ldr` at 0x795d0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x795e0 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x795f4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x795fc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x79608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7960c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x79620 | load | 4 | `abs 0x79704` | `-` |
| `ldr` at 0x79628 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7962c | load | 4 | `abs 0x79708` | `-` |
| `ldr` at 0x79640 | load | 4 | `abs 0x7970c` | `-` |
| `ldr` at 0x79654 | load | 4 | `abs 0x79710` | `-` |
| `ldr` at 0x7965c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x79660 | load | 4 | `abs 0x79708` | `-` |
| `ldr` at 0x7966c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x79690 | load | 4 | `abs 0x79714` | `-` |
| `str` at 0x796bc | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x796d4 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x796d8 | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x796e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x796e4 | load | 4 | `sp -0x78` | `-` |
| `ldm` at 0x796f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x796f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x796f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x792ac | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x792d0 | `(unknown callee) 0x5eac8` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x792dc | `(unknown callee) 0x5ef20` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x792f0 | `(unknown callee) 0x197a8` | `sp-100` | `0xfe90c` | `ret(0x5ef20)` | `sp-100` |
| `bl` at 0x792fc | `(unknown callee) 0x19538` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x79328 | `(unknown callee) 0x5ed50` | `sp-120` | `sp-116` | `sp-100` | `ret(0x19538)` |
| `bl` at 0x79344 | `(unknown callee) 0xaa090` | `0x29a5d4` | `0x0` | `[sp-20]` | `[sp-120]` |
| `bl` at 0x79390 | `(unknown callee) 0x781b8` | `[sp-140]` | `?` | `?` | `[sp-140]` |
| `bl` at 0x793a0 | `(unknown callee) 0x78298` | `[sp-132]` | `?` | `?` | `ret(0x781b8)` |
| `bl` at 0x79414 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x79438 | `(unknown callee) 0x7713c` | `[sp-136]` | `0x8` | `?` | `0x0` |
| `bl` at 0x7945c | `(unknown callee) 0x771d8` | `[sp-136]` | `0xfe930` | `sp-124` | `0x1` |
| `mov` at 0x794a8 | `(unknown callee) (unknown target)` | `[sp-136]` | `sp-124` | `0x1` | `?` |
| `bl` at 0x79500 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x7951c | `(unknown callee) 0x7713c` | `[sp-136]` | `0x10` | `?` | `?` |
| `bl` at 0x79540 | `(unknown callee) 0x771d8` | `[sp-136]` | `0xfe940` | `sp-124` | `0x2` |
| `bl` at 0x795c4 | `(unknown callee) 0xaddfc` | `?` | `?` | `[sp-136]` | `sp-124` |
| `bl` at 0x795f8 | `(unknown callee) 0x5eac8` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x79610 | `(unknown callee) 0x5f254` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x79624 | `(unknown callee) 0x197a8` | `sp-100` | `0xfe954` | `ret(0x5f254)` | `sp-100` |
| `bl` at 0x79644 | `(unknown callee) 0x195e0` | `sp-100` | `0xfe968` | `[sp-128]` | `sp-100` |
| `bl` at 0x79658 | `(unknown callee) 0x195e0` | `sp-100` | `0xfe980` | `[sp-128]` | `sp-100` |
| `bl` at 0x79670 | `(unknown callee) 0x1f8e0` | `[sp-128]` | `?` | `[sp-128]` | `0xfffff69e` |
| `bl` at 0x79684 | `(unknown callee) 0x195e0` | `sp-100` | `ret(0x1f8e0)` | `ret(0x1f8e0)` | `sp-100` |
| `bl` at 0x79694 | `(unknown callee) 0x195e0` | `sp-100` | `0xfe98c` | `?` | `sp-100` |
| `bl` at 0x796a0 | `(unknown callee) 0x19538` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x796cc | `(unknown callee) 0x5ed50` | `sp-120` | `sp-116` | `sp-100` | `ret(0x19538)` |
| `bl` at 0x796e8 | `(unknown callee) 0xaa090` | `0x29a5d4` | `0x0` | `[sp-20]` | `[sp-120]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x792b8 | `ret(0x375e4) == 0x1` | `ret(0x375e4) != 0x1` |
| `blt` at 0x79358 | `[[sp-20]+24] < 0x0 (signed, 32-bit)` | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x793e4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x793f8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x79444 | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `bne` at 0x794bc | `ret(unknown) != 0x1` | `ret(unknown) == 0x1` |
| `bne` at 0x794e4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x79528 | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `bne` at 0x795d8 | `ret(0xaddfc) != 0x2` | `ret(0xaddfc) == 0x2` |
| `beq` at 0x79604 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `bne` at 0x79634 | `[sp-128] != 0xfffff69e` | `[sp-128] == 0xfffff69e` |
| `beq` at 0x79668 | `[sp-128] == 0xfffff69e` | `[sp-128] != 0xfffff69e` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x796f0 | `r0` | `ret(0xaa090)` |

## `syscom_process_outbound_event`

- `syscom_process_outbound_event`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79718 to 0x79df8.
  Established. The word at absolute `0x29a5e0` is read (`ldr` at 0x79738).
  Signed `[[sp-20]+28] >= 0x0` takes `0x7979c`, while negative values fall through (`bge` at 0x7977c).
  The included indirect call at `0x79d5c` carries `[sp-176]`, `sp-148`, `[sp-152]`, unknown (`mov` at 0x79d5c).
  Outbound event serialization meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79728 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x79730 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x79734 | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x79738 | load | 4 | `abs 0x29a5e0` | `-` |
| `ldr` at 0x79748 | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x7974c | load | 4 | `abs 0x29a5e0` | `-` |
| `str` at 0x79758 | store | 4 | `sp -0xb0` | `ret(0x781b8)` |
| `ldr` at 0x7975c | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x79760 | load | 4 | `abs 0x29a5e0` | `-` |
| `str` at 0x7976c | store | 4 | `sp -0xac` | `ret(0x78298)` |
| `ldr` at 0x79770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79774 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x79780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79784 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x7979c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797a0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x797b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797b4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x797c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797c4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x797dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797e0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x797f0 | load | 4 | `sp -0xac` | `-` |
| `strb` at 0x797f8 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x797fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79800 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x79804 | store | 4 | `sp -0xa8` | `[[sp-20]+28]` |
| `ldr` at 0x79808 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79810 | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x79814 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x7981c | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x79820 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79828 | store | 1 | `sp -0x91` | `?` |
| `ldr` at 0x7982c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79830 | store | 1 | `sp -0x90` | `([[sp-20]+28] & 0xff)` |
| `ldr` at 0x79834 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79838 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x79848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7984c | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x79850 | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0x79858 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79864 | store | 4 | `sp -0xb4` | `ret(0xaadf4)` |
| `ldr` at 0x79868 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x7986c | store | 4 | `sp -0xa8` | `[sp-180]` |
| `ldr` at 0x79870 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79878 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x7987c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79884 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x79888 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79890 | store | 1 | `sp -0x8d` | `?` |
| `ldr` at 0x79894 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79898 | store | 1 | `sp -0x8c` | `([sp-180] & 0xff)` |
| `strb` at 0x798a0 | store | 1 | `sp -0x8b` | `0x0` |
| `str` at 0x798a8 | store | 4 | `sp -0x98` | `0xa` |
| `str` at 0x798b0 | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x798b4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x798c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x798c4 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x798c8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x798d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x798d8 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x798ec | load | 1 | `(unknown)` | `-` |
| `str` at 0x798f4 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x798f8 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x79904 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x79910 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x79920 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x79930 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x79944 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x79950 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x7996c | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x79974 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x79978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7997c | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x79990 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x799a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x799a4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x799b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x799bc | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x799c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x799c8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x799d0 | store | 4 | `sp -0xb8` | `ret(0xaadf4)` |
| `ldr` at 0x799d4 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x799d8 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x799e0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x799f8 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79a00 | store | 1 | `(unknown)` | `?` |
| `str` at 0x79a08 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x79a10 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79a28 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79a30 | store | 1 | `(unknown)` | `?` |
| `str` at 0x79a38 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x79a40 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79a58 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79a60 | store | 1 | `(unknown)` | `?` |
| `str` at 0x79a68 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x79a70 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79a88 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79a8c | store | 1 | `(unknown)` | `([sp-168] & 0xff)` |
| `str` at 0x79a94 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x79a9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79aa0 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x79ab4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x79ab8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x79abc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79ac0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x79ad4 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x79ae0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79af8 | load | 4 | `sp -0xa4` | `-` |
| `strb` at 0x79b04 | store | 1 | `(unknown)` | `?` |
| `str` at 0x79b0c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x79b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79b14 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x79b28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x79b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79b3c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x79b50 | load | 4 | `(unknown)` | `-` |
| `str` at 0x79b54 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x79b5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79b60 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x79b68 | store | 4 | `sp -0xbc` | `ret(0xaadf4)` |
| `ldr` at 0x79b70 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79b7c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x79b80 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x79b88 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79b8c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x79b94 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x79b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79ba0 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x79bb4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x79bb8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x79bbc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79bc0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x79bd4 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x79be0 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x79bfc | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x79c04 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x79c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c0c | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x79c20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x79c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c34 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x79c48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x79c4c | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x79c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c58 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x79c60 | store | 4 | `sp -0xc0` | `ret(0xaadf4)` |
| `ldr` at 0x79c68 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79c74 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x79c7c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79c80 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x79c88 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x79c8c | load | 1 | `sp -0x8b` | `-` |
| `strb` at 0x79c94 | store | 1 | `sp -0x8b` | `?` |
| `ldr` at 0x79c98 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x79ca0 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x79ca8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79cc4 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79cc8 | load | 4 | `abs 0x79df4` | `-` |
| `ldr` at 0x79cd0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79cd8 | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x79cdc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79d00 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x79d08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79d14 | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x79d18 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79d3c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x79d48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79d4c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79d54 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x79d64 | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x79d68 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79d8c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x79d98 | load | 4 | `abs 0x79df0` | `-` |
| `ldr` at 0x79d9c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x79dc0 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x79dcc | load | 4 | `sp -0x98` | `-` |
| `str` at 0x79dd0 | store | 4 | `sp -0xc8` | `[sp-152]` |
| `ldr` at 0x79dd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79ddc | load | 4 | `sp -0xb0` | `-` |
| `ldm` at 0x79dec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x79dec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x79dec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79750 | `(unknown callee) 0x781b8` | `[0x29a5e0]` | `r1` | `r2` | `0x29a5d4` |
| `bl` at 0x79764 | `(unknown callee) 0x78298` | `[0x29a5e0]` | `?` | `?` | `0x29a5d4` |
| `bl` at 0x79860 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x799cc | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-160]` | `[sp-20]` | `?` |
| `bl` at 0x79b64 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-160]` | `[sp-20]` | `?` |
| `bl` at 0x79b84 | `(unknown callee) 0x19610` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x79c5c | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-160]` | `[sp-20]` | `?` |
| `bl` at 0x79c78 | `(unknown callee) 0x19808` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x79cb0 | `(unknown callee) 0x7713c` | `[sp-176]` | `0x20` | `?` | `?` |
| `bl` at 0x79cd4 | `(unknown callee) 0x771d8` | `[sp-176]` | `0xfe990` | `sp-148` | `[sp-152]` |
| `mov` at 0x79d5c | `(unknown callee) (unknown target)` | `[sp-176]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x79de4 | `(unknown callee) 0xaddfc` | `?` | `?` | `[sp-176]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x79740 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x7977c | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x79794 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x797ac | `? == 0x5` | `? != 0x5` |
| `bge` at 0x797bc | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x797d4 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x797ec | `? != 0x4` | `? == 0x4` |
| `bne` at 0x79844 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x798bc | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `bls` at 0x798d0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x79900 | `[sp-196] == 0x1` | `[sp-196] != 0x1` |
| `bgt` at 0x7990c | `[sp-196] > 0x1 (signed, 32-bit)` | `[sp-196] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x79918 | `[sp-196] == 0x0` | `[sp-196] != 0x0` |
| `beq` at 0x79928 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `bls` at 0x7993c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x7999c | `? != 0x0` | `? == 0x0` |
| `bls` at 0x79acc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x79b34 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x79bcc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x79c2c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x79cbc | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `bne` at 0x79d10 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x79dec | `r0` | `?` |

## `syscom_process_outbound_response`

- `syscom_process_outbound_response`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79df8 to 0x7a568.
  Established. Signed `[r0+24] < 0x0` takes `0x79e4c`, while nonnegative values fall through (`blt` at 0x79e28).
  The included indirect call at `0x7a440` carries `[sp-180]`, `[sp-176]`, `sp-148`, `[sp-152]` (`mov` at 0x7a440).
  The unnamed `0x19610` call carries `(sp-148 + [sp-152])`, unknown, `[sp-164]`, `(sp-148 + [sp-152])` (`bl` at 0x7a2c0).
  Response field encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79e08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x79e10 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x79e18 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x79e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e20 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x79e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e30 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x79e38 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x79e3c | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x79e44 | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x79e50 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x79e54 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x79e58 | store | 4 | `sp -0xb0` | `[sp-184]` |
| `ldr` at 0x79e5c | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x79e68 | store | 4 | `sp -0xb4` | `ret(0x781b8)` |
| `ldr` at 0x79e6c | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x79e78 | store | 4 | `sp -0xb0` | `ret(0x78298)` |
| `ldr` at 0x79e7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e80 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x79e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e90 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x79e98 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x79e9c | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x79ea4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x79eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79eb0 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x79eb8 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x79ebc | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x79ec0 | store | 4 | `sp -0xac` | `[sp-188]` |
| `ldr` at 0x79ec4 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x79ed0 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x79ed4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79ee0 | load | 4 | `sp -0xb0` | `-` |
| `strb` at 0x79ee8 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x79eec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79ef0 | load | 1 | `[sp-20] +0x13` | `-` |
| `ldr` at 0x79f00 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79f04 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x79f14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f18 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x79f1c | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0x79f24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79f30 | store | 4 | `sp -0xc0` | `ret(0xaadf4)` |
| `ldr` at 0x79f34 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x79f38 | store | 4 | `sp -0xa8` | `[sp-192]` |
| `ldr` at 0x79f3c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79f44 | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x79f48 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79f50 | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x79f54 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79f5c | store | 1 | `sp -0x91` | `?` |
| `ldr` at 0x79f60 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x79f64 | store | 1 | `sp -0x90` | `([sp-192] & 0xff)` |
| `strb` at 0x79f6c | store | 1 | `sp -0x8f` | `0x0` |
| `str` at 0x79f74 | store | 4 | `sp -0x98` | `0x6` |
| `strb` at 0x79f80 | store | 1 | `sp -0x93` | `?` |
| `strb` at 0x79f88 | store | 1 | `sp -0x92` | `?` |
| `strb` at 0x79f90 | store | 1 | `sp -0x91` | `?` |
| `strb` at 0x79f98 | store | 1 | `sp -0x90` | `?` |
| `ldr` at 0x79f9c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79fa0 | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x79fa8 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x79fac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79fb0 | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x79fb8 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x79fbc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79fc0 | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x79fc8 | store | 1 | `sp -0x8d` | `?` |
| `ldr` at 0x79fcc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79fd0 | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x79fd4 | store | 1 | `sp -0x8c` | `([[sp-20]+19]:1 & 0xff)` |
| `str` at 0x79fdc | store | 4 | `sp -0x98` | `0x9` |
| `str` at 0x79fe4 | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x79fec | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x79ff0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x79ffc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7a000 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x7a004 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x7a010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a014 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x7a028 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7a030 | store | 4 | `sp -0xd0` | `?` |
| `ldr` at 0x7a034 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x7a040 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x7a04c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x7a05c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x7a06c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x7a080 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x7a08c | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x7a0a8 | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x7a0b0 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x7a0b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a0b8 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x7a0cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a0dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a0e0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x7a0f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7a0f8 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x7a100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a104 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x7a10c | store | 4 | `sp -0xc4` | `ret(0xaadf4)` |
| `ldr` at 0x7a110 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x7a114 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x7a11c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a134 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x7a13c | store | 1 | `(unknown)` | `?` |
| `str` at 0x7a144 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x7a14c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a164 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x7a16c | store | 1 | `(unknown)` | `?` |
| `str` at 0x7a174 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x7a17c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a194 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x7a19c | store | 1 | `(unknown)` | `?` |
| `str` at 0x7a1a4 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x7a1ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a1c4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x7a1c8 | store | 1 | `(unknown)` | `([sp-168] & 0xff)` |
| `str` at 0x7a1d0 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x7a1d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a1dc | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x7a1f0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x7a1f4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x7a1f8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a1fc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x7a210 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x7a21c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a234 | load | 4 | `sp -0xa4` | `-` |
| `strb` at 0x7a240 | store | 1 | `(unknown)` | `?` |
| `str` at 0x7a248 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x7a24c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a250 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x7a264 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a278 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x7a28c | load | 4 | `(unknown)` | `-` |
| `str` at 0x7a290 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x7a298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a29c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x7a2a4 | store | 4 | `sp -0xc8` | `ret(0xaadf4)` |
| `ldr` at 0x7a2ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a2b8 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x7a2bc | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x7a2c4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a2c8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x7a2d0 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x7a2d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a2dc | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x7a2f0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x7a2f4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x7a2f8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a2fc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x7a310 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x7a31c | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x7a338 | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x7a340 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x7a344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a348 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x7a35c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a36c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a370 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x7a384 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7a388 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x7a390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a394 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x7a39c | store | 4 | `sp -0xcc` | `ret(0xaadf4)` |
| `ldr` at 0x7a3a4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a3b0 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x7a3b8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a3bc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x7a3c4 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x7a3c8 | load | 1 | `sp -0x8f` | `-` |
| `strb` at 0x7a3d0 | store | 1 | `sp -0x8f` | `([sp-143]:1+1 & 0xff)` |
| `ldr` at 0x7a3d4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x7a3dc | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x7a3e4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a400 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a404 | load | 4 | `abs 0x7a55c` | `-` |
| `ldr` at 0x7a40c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a414 | load | 4 | `abs 0x7a560` | `-` |
| `ldr` at 0x7a418 | load | 4 | `abs 0x133834` | `-` |
| `ldr` at 0x7a424 | load | 4 | `abs 0x7a560` | `-` |
| `ldr` at 0x7a42c | load | 4 | `abs 0x133834` | `-` |
| `ldr` at 0x7a430 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a434 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x7a438 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a444 | load | 4 | `abs 0x7a564` | `-` |
| `ldr` at 0x7a448 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a46c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x7a474 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a480 | load | 4 | `abs 0x7a564` | `-` |
| `ldr` at 0x7a484 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a4a8 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x7a4b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a4b8 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a4c0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7a4d0 | load | 4 | `abs 0x7a564` | `-` |
| `ldr` at 0x7a4d4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a4f8 | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x7a504 | load | 4 | `abs 0x7a564` | `-` |
| `ldr` at 0x7a508 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x7a52c | load | 4 | `abs 0x29a5e8` | `-` |
| `ldr` at 0x7a538 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x7a53c | store | 4 | `sp -0xd4` | `[sp-152]` |
| `ldr` at 0x7a544 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a548 | load | 4 | `sp -0xb4` | `-` |
| `ldm` at 0x7a558 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a558 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79e60 | `(unknown callee) 0x781b8` | `[sp-184]` | `r1` | `r2` | `?` |
| `bl` at 0x79e70 | `(unknown callee) 0x78298` | `[sp-176]` | `?` | `?` | `ret(0x781b8)` |
| `bl` at 0x79ed8 | `(unknown callee) 0x7b528` | `[sp-180]` | `[sp-20]` | `?` | `[sp-188]` |
| `bl` at 0x79f2c | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x7a108 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-160]` | `[sp-20]` | `?` |
| `bl` at 0x7a2a0 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-160]` | `[sp-20]` | `?` |
| `bl` at 0x7a2c0 | `(unknown callee) 0x19610` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x7a398 | `(unknown callee) 0xaadf4` | `[sp-20]` | `[sp-160]` | `[sp-20]` | `?` |
| `bl` at 0x7a3b4 | `(unknown callee) 0x19808` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x7a3ec | `(unknown callee) 0x7713c` | `[sp-180]` | `0x40` | `?` | `?` |
| `bl` at 0x7a410 | `(unknown callee) 0x771d8` | `[sp-180]` | `0xfe9a0` | `sp-148` | `[sp-152]` |
| `mov` at 0x7a440 | `(unknown callee) (unknown target)` | `[sp-180]` | `[sp-176]` | `sp-148` | `[sp-152]` |
| `mov` at 0x7a4c8 | `(unknown callee) (unknown target)` | `[sp-180]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x7a550 | `(unknown callee) 0xaddfc` | `?` | `?` | `[sp-180]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x79e28 | `[r0+24] < 0x0 (signed, 32-bit)` | `[r0+24] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x79e88 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x79ecc | `[sp-188] != 0x15` | `[sp-188] == 0x15` |
| `beq` at 0x79ef8 | `[[sp-20]+19]:1 == 0x0` | `[[sp-20]+19]:1 != 0x0` |
| `bne` at 0x79f10 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x79ff8 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `bls` at 0x7a00c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x7a03c | `[sp-208] == 0x1` | `[sp-208] != 0x1` |
| `bgt` at 0x7a048 | `[sp-208] > 0x1 (signed, 32-bit)` | `[sp-208] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x7a054 | `[sp-208] == 0x0` | `[sp-208] != 0x0` |
| `beq` at 0x7a064 | `[sp-208] == 0x2` | `[sp-208] != 0x2` |
| `bls` at 0x7a078 | `[sp-152]+5 <= 0x7f (unsigned, 32-bit)` | `[sp-152]+5 > 0x7f (unsigned, 32-bit)` |
| `bne` at 0x7a0d8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x7a208 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x7a270 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x7a308 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x7a368 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7a3f8 | `ret(0x7713c) == 0x0` | `ret(0x7713c) != 0x0` |
| `beq` at 0x7a420 | `[0x133834] == 0x0` | `[0x133834] != 0x0` |
| `bne` at 0x7a47c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a558 | `r0` | `?` |

## `syscom_process_outbound_packet`

- `syscom_process_outbound_packet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a568 to 0x7a5e8.
  Established. The byte at entry `r0 +0x16` is read (`ldrb` at 0x7a580).
  Equality `[r0+22]:1 == 0x1` takes `0x7a5d8`, while inequality falls through (`beq` at 0x7a590).
  The unnamed `0x79df8` call carries entry `r0`, `r1`, `r2`, and `[r0+22]:1` (`bl` at 0x7a5dc).
  Outbound packet selector meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a578 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7a57c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7a580 | load | 1 | `r0 +0x16` | `-` |
| `str` at 0x7a584 | store | 4 | `sp -0x18` | `[r0+22]:1` |
| `ldr` at 0x7a588 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a594 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a5a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a5b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a5c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a5cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a5d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7a5e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a5e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a5e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a5c4 | `(unknown callee) 0x79298` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x7a5d0 | `(unknown callee) 0x79718` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x7a5dc | `(unknown callee) 0x79df8` | `r0` | `r1` | `r2` | `[r0+22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7a590 | `[r0+22]:1 == 0x1` | `[r0+22]:1 != 0x1` |
| `bgt` at 0x7a59c | `[r0+22]:1 > 0x1 (signed, 32-bit)` | `[r0+22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x7a5a8 | `[r0+22]:1 == 0x0` | `[r0+22]:1 != 0x0` |
| `beq` at 0x7a5b8 | `[r0+22]:1 == 0x2` | `[r0+22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a5e4 | `r0` | `?` |

## `i2c_soft_l1_reset_check`

- `i2c_soft_l1_reset_check`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3aa28 to 0x3ab78.
  Established. Included word stores write `0xf5cfc` and `0xf5d18` through `[sp-20] +0x0` (`str` at 0x3aab8, `str` at 0x3aad4).
  The unnamed `0x40294` call carries `0x0`, `0xff`, unknown, unknown (`bl` at 0x3aa70).
  A nonzero `ret(0x37674)` takes `0x3aaa4`, while zero falls through (`bne` at 0x3aa94).
  Reset-check result and pointee meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3aa38 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3aa40 | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x3aa60 | store | 1 | `sp -0x19` | `0x1` |
| `strb` at 0x3aa78 | store | 1 | `sp -0x19` | `(ret(0x40294) & 0xff)` |
| `str` at 0x3aa88 | store | 4 | `sp -0x20` | `ret(0x37674)` |
| `ldr` at 0x3aa8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3aa9c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3aaa4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3aab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aab4 | load | 4 | `abs 0x3ab68` | `-` |
| `str` at 0x3aab8 | store | 4 | `[sp-20] +0x0` | `0xf5cfc` |
| `ldrb` at 0x3aac0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3aacc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aad0 | load | 4 | `abs 0x3ab6c` | `-` |
| `str` at 0x3aad4 | store | 4 | `[sp-20] +0x0` | `0xf5d18` |
| `ldr` at 0x3ab00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ab04 | load | 4 | `abs 0x3ab70` | `-` |
| `str` at 0x3ab08 | store | 4 | `[sp-20] +0x0` | `0xf5d3c` |
| `ldr` at 0x3ab24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ab28 | load | 4 | `abs 0x3ab74` | `-` |
| `str` at 0x3ab2c | store | 4 | `[sp-20] +0x0` | `0xf5d6c` |
| `ldr` at 0x3ab30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ab34 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x3ab44 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x3ab50 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3ab54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3ab58 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x3ab5c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x3ab64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3ab64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3ab64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3aa44 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3aa70 | `(unknown callee) 0x40294` | `0x0` | `0xff` | `?` | `?` |
| `bl` at 0x3aa80 | `(unknown callee) 0x37674` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3aadc | `(unknown callee) 0x22e04` | `ret(0x37674)` | `?` | `?` | `[sp-25]:1` |
| `bl` at 0x3aaf0 | `(unknown callee) 0x2356c` | `0x0` | `?` | `?` | `ret(0x22e04)` |
| `bl` at 0x3ab14 | `(unknown callee) 0x2356c` | `0x2` | `?` | `?` | `ret(0x2356c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3aa54 | `? == 0x4f` | `? != 0x4f` |
| `bne` at 0x3aa94 | `ret(0x37674) != 0x0` | `ret(0x37674) == 0x0` |
| `bge` at 0x3aaac | `ret(0x37674) >= 0x0 (signed, 32-bit)` | `ret(0x37674) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3aac8 | `[sp-25]:1 != 0x0` | `[sp-25]:1 == 0x0` |
| `beq` at 0x3aae8 | `ret(0x22e04) == 0x0` | `ret(0x22e04) != 0x0` |
| `beq` at 0x3aafc | `ret(0x2356c) == 0x1` | `ret(0x2356c) != 0x1` |
| `bne` at 0x3ab20 | `ret(0x2356c) != 0x1` | `ret(0x2356c) == 0x1` |
| `beq` at 0x3ab3c | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3ab64 | `r0` | `[sp-36]` |

## `i2cTest`

- `i2cTest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa60ec to 0xa63a8.
  Established. The unnamed `0xc8538` call carries `0x0`, `sp-28`, entry `r2`, `sp-28` (`bl` at 0xa6114).
  An included word store assigns `([sp-60] + ret(0xa63a8))` to `sp -0x3c` (`str` at 0xa61e8).
  A zero `[sp-60]` takes `0xa6384`, while nonzero falls through (`beq` at 0xa6374).
  I2C diagnostic acceptance criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa60fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa6104 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xa611c | store | 4 | `sp -0x18` | `ret(0xc8538)` |
| `str` at 0xa6134 | store | 4 | `sp -0x20` | `ret(0xca3d0)` |
| `str` at 0xa614c | store | 4 | `sp -0x28` | `ret(0xc56e0)` |
| `str` at 0xa6164 | store | 4 | `sp -0x30` | `ret(0xbfe74)` |
| `str` at 0xa616c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xa6170 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa6174 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa6184 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa6188 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa618c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6198 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa61b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa61c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa61c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa61e0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa61e8 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xa63a8))` |
| `ldr` at 0xa61ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa61f4 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xa6200 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa6204 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa6208 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa6214 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa6234 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa623c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa6244 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa625c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa6264 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xa64e0))` |
| `ldr` at 0xa6268 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa6270 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xa627c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa6280 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa6284 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6290 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa62b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa62c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa62cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa62e4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa62ec | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xa661c))` |
| `ldr` at 0xa62f0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa62f8 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xa6304 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa6308 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa630c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6318 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa6338 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa6350 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa6358 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xc16ec))` |
| `ldr` at 0xa635c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa6364 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xa636c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa637c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa6384 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa638c | store | 4 | `sp -0x38` | `?` |
| `str` at 0xa6398 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xa639c | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0xa63a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa63a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa63a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6114 | `(unknown callee) 0xc8538` | `0x0` | `sp-28` | `r2` | `sp-28` |
| `bl` at 0xa612c | `(unknown callee) 0xca3d0` | `0x0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xa6144 | `(unknown callee) 0xc56e0` | `0x0` | `sp-44` | `?` | `sp-44` |
| `bl` at 0xa615c | `(unknown callee) 0xbfe74` | `0x0` | `sp-52` | `?` | `sp-52` |
| `bl` at 0xa61a8 | `(unknown callee) 0xc85ec` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa61d8 | `(unknown callee) 0xa63a8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa6224 | `(unknown callee) 0xca484` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa6254 | `(unknown callee) 0xa64e0` | `?` | `?` | `?` | `?` |
| `bl` at 0xa62a0 | `(unknown callee) 0xc5790` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa62dc | `(unknown callee) 0xa661c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa6328 | `(unknown callee) 0xbfb1c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa6348 | `(unknown callee) 0xc16ec` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa617c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xa6194 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa61b4 | `ret(0xc85ec) == 0x0` | `ret(0xc85ec) != 0x0` |
| `bge` at 0xa6210 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa6230 | `ret(0xca484) == 0x0` | `ret(0xca484) != 0x0` |
| `bge` at 0xa628c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa62ac | `ret(0xc5790) == 0x0` | `ret(0xc5790) != 0x0` |
| `bge` at 0xa6314 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa6334 | `ret(0xbfb1c) == 0x0` | `ret(0xbfb1c) != 0x0` |
| `beq` at 0xa6374 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa63a4 | `r0` | `?` |

## `EnvInit`

- `EnvInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70014 to 0x70554.
  Established. An included absolute word store writes `[0x132f10]+1` to `0x132f10` (`str` at 0x700c8).
  Unsigned `ret(0x70554) <= 0x7ffff` takes `0x7004c`, while larger values fall through (`bls` at 0x7003c).
  The unnamed `0x2066c` call at `0x701c0` carries `0x1331f0`, `0xfd390`, `0x73070`, `[sp-32]` (`bl` at 0x701c0).
  Environment initialization record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x70028 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x70034 | load | 4 | `abs 0x70474` | `-` |
| `str` at 0x70044 | store | 4 | `sp -0x2c` | `0x960` |
| `ldr` at 0x7004c | load | 4 | `abs 0x70478` | `-` |
| `str` at 0x70050 | store | 4 | `sp -0x2c` | `0x5dc` |
| `ldr` at 0x70054 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x70058 | store | 4 | `sp -0x24` | `[sp-44]` |
| `ldr` at 0x70064 | load | 4 | `abs 0x70474` | `-` |
| `str` at 0x70074 | store | 4 | `sp -0x30` | `0x960` |
| `ldr` at 0x7007c | load | 4 | `abs 0x70478` | `-` |
| `str` at 0x70080 | store | 4 | `sp -0x30` | `0x5dc` |
| `ldr` at 0x70084 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x70088 | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0x7008c | load | 4 | `abs 0x7047c` | `-` |
| `ldr` at 0x70090 | load | 4 | `abs 0x132f10` | `-` |
| `ldr` at 0x7009c | load | 4 | `abs 0x70480` | `-` |
| `str` at 0x700a0 | store | 4 | `sp -0x44` | `0xfd314` |
| `ldr` at 0x700a8 | load | 4 | `abs 0x70484` | `-` |
| `ldr` at 0x700ac | load | 4 | `abs 0x70488` | `-` |
| `ldr` at 0x700b8 | load | 4 | `abs 0x7047c` | `-` |
| `ldr` at 0x700bc | load | 4 | `abs 0x7047c` | `-` |
| `ldr` at 0x700c0 | load | 4 | `abs 0x132f10` | `-` |
| `str` at 0x700c8 | store | 4 | `abs 0x132f10` | `[0x132f10]+1` |
| `ldr` at 0x700cc | load | 4 | `abs 0x7048c` | `-` |
| `str` at 0x700d4 | store | 4 | `abs 0x132f18` | `0x0` |
| `ldr` at 0x700d8 | load | 4 | `abs 0x70490` | `-` |
| `ldr` at 0x700e8 | load | 4 | `abs 0x70494` | `-` |
| `ldr` at 0x700ec | load | 4 | `abs 0x70498` | `-` |
| `ldr` at 0x700fc | load | 4 | `abs 0x7049c` | `-` |
| `ldr` at 0x70100 | load | 4 | `abs 0x704a0` | `-` |
| `ldr` at 0x70110 | load | 4 | `abs 0x704a4` | `-` |
| `ldr` at 0x70114 | load | 4 | `abs 0x704a8` | `-` |
| `ldr` at 0x70124 | load | 4 | `abs 0x704ac` | `-` |
| `ldr` at 0x70128 | load | 4 | `abs 0x704b0` | `-` |
| `ldr` at 0x70138 | load | 4 | `abs 0x704b4` | `-` |
| `ldr` at 0x7013c | load | 4 | `abs 0x704b8` | `-` |
| `ldr` at 0x7014c | load | 4 | `abs 0x704bc` | `-` |
| `ldr` at 0x70150 | load | 4 | `abs 0x704c0` | `-` |
| `ldr` at 0x70160 | load | 4 | `abs 0x704c4` | `-` |
| `ldr` at 0x70164 | load | 4 | `abs 0x704c8` | `-` |
| `ldr` at 0x70174 | load | 4 | `abs 0x704cc` | `-` |
| `ldr` at 0x70178 | load | 4 | `abs 0x704d0` | `-` |
| `str` at 0x7018c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x70190 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x70194 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7019c | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x701a4 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x701ac | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x701b0 | load | 4 | `abs 0x704d4` | `-` |
| `ldr` at 0x701b4 | load | 4 | `abs 0x704d8` | `-` |
| `ldr` at 0x701b8 | load | 4 | `abs 0x704dc` | `-` |
| `ldr` at 0x701bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x701c8 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x701cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x701d0 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x701d8 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x701e0 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x701e8 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x701ec | load | 4 | `abs 0x704e0` | `-` |
| `ldr` at 0x701f0 | load | 4 | `abs 0x704e4` | `-` |
| `ldr` at 0x701f4 | load | 4 | `abs 0x704e8` | `-` |
| `ldr` at 0x701f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x70204 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x70208 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7020c | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x70214 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7021c | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x70224 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x70228 | load | 4 | `abs 0x704ec` | `-` |
| `ldr` at 0x7022c | load | 4 | `abs 0x704f0` | `-` |
| `ldr` at 0x70230 | load | 4 | `abs 0x704f4` | `-` |
| `ldr` at 0x70234 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x70240 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x70244 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x70248 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x70250 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x70258 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x70260 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x70264 | load | 4 | `abs 0x704f8` | `-` |
| `ldr` at 0x70268 | load | 4 | `abs 0x704fc` | `-` |
| `ldr` at 0x7026c | load | 4 | `abs 0x70500` | `-` |
| `ldr` at 0x70270 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7027c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x70280 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x70284 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7028c | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x70294 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7029c | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x702a0 | load | 4 | `abs 0x70504` | `-` |
| `ldr` at 0x702a4 | load | 4 | `abs 0x70508` | `-` |
| `ldr` at 0x702a8 | load | 4 | `abs 0x7050c` | `-` |
| `ldr` at 0x702ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x702b8 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x702bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x702c0 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x702c8 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x702d0 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x702d8 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x702dc | load | 4 | `abs 0x70510` | `-` |
| `ldr` at 0x702e0 | load | 4 | `abs 0x70514` | `-` |
| `ldr` at 0x702e4 | load | 4 | `abs 0x70518` | `-` |
| `ldr` at 0x702e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x702f0 | load | 4 | `abs 0x7051c` | `-` |
| `ldr` at 0x702f4 | load | 4 | `abs 0x29767c` | `-` |
| `ldr` at 0x70300 | load | 4 | `abs 0x7051c` | `-` |
| `ldr` at 0x70304 | load | 4 | `abs 0x29767c` | `-` |
| `ldr` at 0x70308 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7030c | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x70318 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x70328 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x70330 | load | 4 | `abs 0x70524` | `-` |
| `str` at 0x70338 | store | 4 | `abs 0x133218` | `ret(0x364c4)` |
| `ldr` at 0x7033c | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x70340 | load | 4 | `abs 0x133218` | `-` |
| `ldr` at 0x7034c | load | 4 | `abs 0x70528` | `-` |
| `str` at 0x70350 | store | 4 | `sp -0x44` | `0xfd3d8` |
| `ldr` at 0x70358 | load | 4 | `abs 0x70484` | `-` |
| `ldr` at 0x7035c | load | 4 | `abs 0x7052c` | `-` |
| `ldr` at 0x7036c | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x70370 | load | 4 | `abs 0x133218` | `-` |
| `ldr` at 0x70378 | load | 4 | `abs 0x70530` | `-` |
| `ldr` at 0x70388 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x70390 | load | 4 | `abs 0x70534` | `-` |
| `str` at 0x70398 | store | 4 | `abs 0x13321c` | `ret(0x3fa14)` |
| `ldr` at 0x7039c | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x703a0 | load | 4 | `abs 0x13321c` | `-` |
| `ldr` at 0x703ac | load | 4 | `abs 0x70538` | `-` |
| `str` at 0x703b0 | store | 4 | `sp -0x44` | `0xfd3fc` |
| `ldr` at 0x703b8 | load | 4 | `abs 0x70484` | `-` |
| `ldr` at 0x703cc | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x703d0 | load | 4 | `abs 0x13321c` | `-` |
| `ldr` at 0x703e4 | load | 4 | `abs 0x7053c` | `-` |
| `ldr` at 0x703f4 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x703fc | load | 4 | `abs 0x70540` | `-` |
| `str` at 0x70404 | store | 4 | `abs 0x133220` | `ret(0x24340)` |
| `ldr` at 0x70408 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x7040c | load | 4 | `abs 0x133220` | `-` |
| `ldr` at 0x70418 | load | 4 | `abs 0x70544` | `-` |
| `str` at 0x7041c | store | 4 | `sp -0x44` | `0xfd418` |
| `ldr` at 0x70424 | load | 4 | `abs 0x70484` | `-` |
| `ldr` at 0x70428 | load | 4 | `abs 0x70548` | `-` |
| `ldr` at 0x70438 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x7043c | load | 4 | `abs 0x133220` | `-` |
| `ldr` at 0x70444 | load | 4 | `abs 0x7054c` | `-` |
| `ldr` at 0x70454 | load | 4 | `abs 0x70520` | `-` |
| `ldr` at 0x7045c | load | 4 | `abs 0x70550` | `-` |
| `str` at 0x70464 | store | 4 | `abs 0x133230` | `ret(0xbfe74)` |
| `ldm` at 0x70470 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x70470 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x70470 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x70470 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7002c | `(unknown callee) 0x70554` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x7005c | `(unknown callee) 0x70554` | `ret(0x70554)` | `?` | `ret(0x70554)` | `[sp-44]` |
| `bl` at 0x700b4 | `(unknown callee) 0x200a0` | `0x9` | `0xfd2d0` | `0x163` | `0x0` |
| `bl` at 0x700e4 | `(unknown callee) 0xa89d0` | `0x132f48` | `0x4` | `0x0` | `0x0` |
| `bl` at 0x700f8 | `(unknown callee) 0x20bb4` | `0x132f50` | `0xfd330` | `0x0` | `0x9` |
| `bl` at 0x7010c | `(unknown callee) 0x20bb4` | `0x132fa4` | `0xfd33c` | `0x0` | `0x9` |
| `bl` at 0x70120 | `(unknown callee) 0x20bb4` | `0x132ff8` | `0xfd348` | `0x0` | `0x9` |
| `bl` at 0x70134 | `(unknown callee) 0x20bb4` | `0x13304c` | `0xfd354` | `0x0` | `0x9` |
| `bl` at 0x70148 | `(unknown callee) 0x20bb4` | `0x1330a0` | `0xfd360` | `0x0` | `0x9` |
| `bl` at 0x7015c | `(unknown callee) 0x20bb4` | `0x1330f4` | `0xfd36c` | `0x0` | `0x9` |
| `bl` at 0x70170 | `(unknown callee) 0x20bb4` | `0x133148` | `0xfd378` | `0x1` | `0x9` |
| `bl` at 0x70184 | `(unknown callee) 0x20bb4` | `0x13319c` | `0xfd384` | `0x1` | `0x9` |
| `bl` at 0x701c0 | `(unknown callee) 0x2066c` | `0x1331f0` | `0xfd390` | `0x73070` | `[sp-32]` |
| `bl` at 0x701fc | `(unknown callee) 0x2066c` | `0x1331f4` | `0xfd39c` | `0x739dc` | `[sp-32]` |
| `bl` at 0x70238 | `(unknown callee) 0x2066c` | `0x1331f8` | `0xfd3a8` | `0x73f08` | `[sp-32]` |
| `bl` at 0x70274 | `(unknown callee) 0x2066c` | `0x1331fc` | `0xfd3b4` | `0x7470c` | `[sp-32]` |
| `bl` at 0x702b0 | `(unknown callee) 0x2066c` | `0x133200` | `0xfd3c0` | `0x74a10` | `[sp-32]` |
| `bl` at 0x702ec | `(unknown callee) 0x2066c` | `0x133204` | `0xfd3cc` | `0x74d54` | `[sp-32]` |
| `mov` at 0x70314 | `(unknown callee) (unknown target)` | `[sp-32]` | `0x133208` | `?` | `[0x29767c]` |
| `bl` at 0x70324 | `(unknown callee) 0x19748` | `0x133208` | `0x0` | `0x4c0` | `?` |
| `bl` at 0x70334 | `(unknown callee) 0x364c4` | `0x0` | `0x133224` | `?` | `?` |
| `bl` at 0x70364 | `(unknown callee) 0x200a0` | `0x9` | `0xfd2d0` | `0x1a1` | `0x0` |
| `bl` at 0x70384 | `(unknown callee) 0x19748` | `0x133238` | `0x0` | `?` | `?` |
| `bl` at 0x70394 | `(unknown callee) 0x3fa14` | `0x0` | `0x133228` | `?` | `?` |
| `bl` at 0x703c4 | `(unknown callee) 0x200a0` | `0x9` | `0xfd2d0` | `0x1ac` | `0x0` |
| `bl` at 0x703f0 | `(unknown callee) 0x19748` | `0x1333a4` | `0x0` | `?` | `?` |
| `bl` at 0x70400 | `(unknown callee) 0x24340` | `0x0` | `0x13322c` | `?` | `?` |
| `bl` at 0x70430 | `(unknown callee) 0x200a0` | `0x9` | `0xfd2d0` | `0x1b7` | `0x0` |
| `bl` at 0x70450 | `(unknown callee) 0x19748` | `0x133678` | `0x0` | `?` | `?` |
| `bl` at 0x70460 | `(unknown callee) 0xbfe74` | `0x0` | `0x133234` | `?` | `?` |
| `bl` at 0x70468 | `(unknown callee) 0x76ca4` | `ret(0xbfe74)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x7003c | `ret(0x70554) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x70554) > 0x7ffff (unsigned, 32-bit)` |
| `bls` at 0x7006c | `ret(0x70554) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x70554) > 0x7ffff (unsigned, 32-bit)` |
| `beq` at 0x70098 | `[0x132f10] == 0x0` | `[0x132f10] != 0x0` |
| `beq` at 0x702fc | `[0x29767c] == 0x0` | `[0x29767c] != 0x0` |
| `ble` at 0x70348 | `[0x133218] <= 0x2a (signed, 32-bit)` | `[0x133218] > 0x2a (signed, 32-bit)` |
| `ble` at 0x703a8 | `[0x13321c] <= 0x14 (signed, 32-bit)` | `[0x13321c] > 0x14 (signed, 32-bit)` |
| `ble` at 0x70414 | `[0x133220] <= 0x14 (signed, 32-bit)` | `[0x133220] > 0x14 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x70470 | `r0` | `ret(0x76ca4)` |

## `EnvEvent`

- `EnvEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f93c to 0x70014.
  Established. Included word stores target absolute `0x132f30` with unknown values (`str` at 0x6fb48, `str` at 0x6fc20).
  A zero `[0x132f34]` takes `0x6fa68`, while nonzero falls through (`beq` at 0x6fa54).
  The unnamed `0xaa20c` call carries `0x132f48`, `0x0`, `[sp-84]`, `sp-80` (`bl` at 0x6faec).
  Event classification and status-bit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f94c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x6f950 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x6f954 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x6f958 | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0x6f964 | store | 1 | `sp -0xf5` | `(ret(0x488cc) & 0xff)` |
| `ldr` at 0x6f968 | load | 4 | `abs 0x6ffe0` | `-` |
| `str` at 0x6f978 | store | 4 | `sp -0xf0` | `ret(0x20cd0)` |
| `ldr` at 0x6f97c | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x6f988 | load | 4 | `abs 0x6ffe4` | `-` |
| `str` at 0x6f98c | store | 4 | `sp -0x104` | `0xfd2b0` |
| `ldr` at 0x6f990 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x6f994 | store | 4 | `sp -0x100` | `ret(0x20cd0)` |
| `ldr` at 0x6f99c | load | 4 | `abs 0x6ffe8` | `-` |
| `ldr` at 0x6f9ac | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x6f9b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f9c8 | load | 4 | `abs 0x6ffec` | `-` |
| `ldrb` at 0x6fa00 | load | 1 | `sp -0xf5` | `-` |
| `ldr` at 0x6fa20 | load | 4 | `abs 0x6fff0` | `-` |
| `ldr` at 0x6fa30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fa40 | load | 4 | `abs 0x6ffec` | `-` |
| `ldr` at 0x6fa48 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fa4c | load | 4 | `abs 0x132f34` | `-` |
| `ldr` at 0x6fa58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6fa78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6fa90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6fabc | store | 4 | `sp -0x104` | `0x1` |
| `str` at 0x6fad8 | store | 4 | `sp -0x104` | `0x0` |
| `ldr` at 0x6fadc | load | 4 | `abs 0x6fff8` | `-` |
| `ldr` at 0x6fae4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6faf4 | store | 4 | `sp -0xf0` | `ret(0xaa20c)` |
| `ldr` at 0x6faf8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fb14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fb24 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fb34 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fb38 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fb3c | load | 4 | `abs 0x132f30` | `-` |
| `ldr` at 0x6fb40 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x6fb48 | store | 4 | `abs 0x132f30` | `?` |
| `ldr` at 0x6fb4c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fb68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fb74 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fb84 | load | 4 | `abs 0x70000` | `-` |
| `ldr` at 0x6fb88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fb94 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fba4 | load | 4 | `abs 0x70004` | `-` |
| `ldr` at 0x6fba8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fbb4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fbc4 | load | 4 | `abs 0x70008` | `-` |
| `ldr` at 0x6fbc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fbd4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fbe4 | load | 4 | `abs 0x7000c` | `-` |
| `ldr` at 0x6fbe8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fbf4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fc04 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fc08 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x6fc14 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fc18 | load | 4 | `abs 0x132f30` | `-` |
| `str` at 0x6fc20 | store | 4 | `abs 0x132f30` | `?` |
| `ldr` at 0x6fc28 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fc2c | load | 4 | `abs 0x132f30` | `-` |
| `ldr` at 0x6fc40 | load | 4 | `abs 0x70000` | `-` |
| `ldr` at 0x6fc50 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fc54 | load | 4 | `abs 0x132f30` | `-` |
| `ldr` at 0x6fc68 | load | 4 | `abs 0x70004` | `-` |
| `ldr` at 0x6fc78 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fc7c | load | 4 | `abs 0x132f30` | `-` |
| `ldr` at 0x6fc90 | load | 4 | `abs 0x70008` | `-` |
| `ldr` at 0x6fca0 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fca4 | load | 4 | `abs 0x132f30` | `-` |
| `ldr` at 0x6fcb8 | load | 4 | `abs 0x7000c` | `-` |
| `ldr` at 0x6fcd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6fce0 | load | 4 | `abs 0x6ffe0` | `-` |
| `str` at 0x6fcec | store | 4 | `sp -0xf0` | `ret(0x2105c)` |
| `ldr` at 0x6fcf0 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x6fcfc | load | 4 | `abs 0x70010` | `-` |
| `str` at 0x6fd00 | store | 4 | `sp -0x104` | `0xfd2f0` |
| `ldr` at 0x6fd04 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x6fd08 | store | 4 | `sp -0x100` | `ret(0x2105c)` |
| `ldr` at 0x6fd10 | load | 4 | `abs 0x6ffe8` | `-` |
| `ldr` at 0x6fd2c | load | 4 | `ret(0x6a0f8) +0x5c` | `-` |
| `str` at 0x6fd30 | store | 4 | `sp -0xf4` | `[ret(0x6a0f8)+92]` |
| `ldr` at 0x6fd34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6fd3c | store | 4 | `sp -0xfc` | `?` |
| `ldr` at 0x6fd40 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd4c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd58 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd64 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd70 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd7c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd88 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fd94 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fda4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fdb0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fdc0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fdcc | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fdd8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fde4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fdf4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe00 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe10 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe1c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe28 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe34 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe40 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe4c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe5c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe68 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe78 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe84 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe90 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fe9c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6feac | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6feb8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fec4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fed4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x6fee4 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6feec | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6fef4 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6fefc | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff04 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff0c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff14 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff1c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff24 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff2c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff34 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff3c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff44 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff4c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff54 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff5c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff64 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff6c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff74 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff7c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff84 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff8c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ff94 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ff9c | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ffa4 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ffac | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ffb4 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ffbc | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ffc4 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x6ffcc | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x6ffd0 | load | 4 | `sp -0xf4` | `-` |
| `ldm` at 0x6ffdc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6ffdc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6ffdc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6ffdc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6ffdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6f95c | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6f970 | `(unknown callee) 0x20cd0` | `0x133148` | `?` | `?` | `ret(0x488cc)` |
| `bl` at 0x6f9a8 | `(unknown callee) 0x200a0` | `0x9` | `0xfd2d0` | `0xb0` | `0x0` |
| `bl` at 0x6f9bc | `(unknown callee) 0x1e4ac` | `[sp-40]` | `?` | `?` | `[sp]` |
| `bl` at 0x6f9cc | `(unknown callee) 0x19808` | `sp-236` | `0xfd2e4` | `?` | `sp-236` |
| `bl` at 0x6f9dc | `(unknown callee) 0x19538` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x6f9ec | `(unknown callee) 0x1d444` | `0x1` | `?` | `?` | `ret(0x19538)` |
| `bl` at 0x6f9f8 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6fa14 | `(unknown callee) 0x5f954` | `(sp-236 + ret(0x19538))` | `ret(0x1d444)` | `[sp-245]:1` | `ret(0x1d444)` |
| `bl` at 0x6fa24 | `(unknown callee) 0x195e0` | `sp-236` | `0xfd2e8` | `?` | `sp-236` |
| `bl` at 0x6fa34 | `(unknown callee) 0x195e0` | `sp-236` | `[sp-40]` | `?` | `sp-236` |
| `bl` at 0x6fa44 | `(unknown callee) 0x195e0` | `sp-236` | `0xfd2e4` | `?` | `sp-236` |
| `bl` at 0x6fa7c | `(unknown callee) 0x5eac8` | `sp-84` | `sp-80` | `[sp-32]` | `sp-84` |
| `bl` at 0x6fa94 | `(unknown callee) 0x5eb28` | `sp-84` | `sp-80` | `[sp-36]` | `sp-84` |
| `bl` at 0x6faa0 | `(unknown callee) 0x19538` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x6facc | `(unknown callee) 0x5ed50` | `sp-84` | `sp-80` | `sp-236` | `ret(0x19538)` |
| `bl` at 0x6faec | `(unknown callee) 0xaa20c` | `0x132f48` | `0x0` | `[sp-84]` | `sp-80` |
| `bl` at 0x6fb0c | `(unknown callee) 0x23348` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6fb1c | `(unknown callee) 0x22e24` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x6fb60 | `(unknown callee) 0x23348` | `0x1` | `0x132f30` | `[0x132f30]` | `?` |
| `bl` at 0x6fb70 | `(unknown callee) 0x22e24` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x6fb8c | `(unknown callee) 0x19808` | `0x1336c8` | `[sp-40]` | `?` | `?` |
| `bl` at 0x6fbac | `(unknown callee) 0x19808` | `0x133718` | `[sp-40]` | `?` | `?` |
| `bl` at 0x6fbcc | `(unknown callee) 0x19808` | `0x133768` | `[sp-40]` | `?` | `?` |
| `bl` at 0x6fbec | `(unknown callee) 0x19808` | `0x1337b8` | `[sp-40]` | `?` | `?` |
| `bl` at 0x6fc24 | `(unknown callee) 0x23314` | `?` | `0x132f30` | `?` | `?` |
| `bl` at 0x6fc48 | `(unknown callee) 0x22e24` | `0x32` | `0x1336c8` | `0x2` | `?` |
| `bl` at 0x6fc70 | `(unknown callee) 0x22e24` | `0x32` | `0x133718` | `0x2` | `?` |
| `bl` at 0x6fc98 | `(unknown callee) 0x22e24` | `0x32` | `0x133768` | `0x2` | `?` |
| `bl` at 0x6fcc0 | `(unknown callee) 0x22e24` | `0x32` | `0x1337b8` | `0x2` | `?` |
| `bl` at 0x6fccc | `(unknown callee) 0x23458` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6fcdc | `(unknown callee) 0x22e24` | `0x32` | `[sp-40]` | `0x1` | `?` |
| `bl` at 0x6fce4 | `(unknown callee) 0x2105c` | `0x133148` | `?` | `?` | `?` |
| `bl` at 0x6fd1c | `(unknown callee) 0x200a0` | `0x9` | `0xfd2d0` | `0x118` | `0x0` |
| `bl` at 0x6fd24 | `(unknown callee) 0x6a0f8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6ffd4 | `(unknown callee) 0x6aa60` | `[sp-244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6f984 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0x6f9b4 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x6fa54 | `[0x132f34] == 0x0` | `[0x132f34] != 0x0` |
| `bgt` at 0x6fa60 | `[sp-28] > 0x1 (signed, 32-bit)` | `[sp-28] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x6fb04 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fb30 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fb58 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fb80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fba0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fbc0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fbe0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fc00 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fc38 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fc60 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fc88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fcb0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fcf8 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0x6fd48 | `[sp-252] == 0x3300` | `[sp-252] != 0x3300` |
| `bhi` at 0x6fd54 | `[sp-252] > 0x3300 (unsigned, 32-bit)` | `[sp-252] <= 0x3300 (unsigned, 32-bit)` |
| `beq` at 0x6fd60 | `[sp-252] == 0x2000` | `[sp-252] != 0x2000` |
| `bhi` at 0x6fd6c | `[sp-252] > 0x2000 (unsigned, 32-bit)` | `[sp-252] <= 0x2000 (unsigned, 32-bit)` |
| `beq` at 0x6fd78 | `[sp-252] == 0x1200` | `[sp-252] != 0x1200` |
| `bhi` at 0x6fd84 | `[sp-252] > 0x1200 (unsigned, 32-bit)` | `[sp-252] <= 0x1200 (unsigned, 32-bit)` |
| `beq` at 0x6fd90 | `[sp-252] == 0x1000` | `[sp-252] != 0x1000` |
| `beq` at 0x6fd9c | `[sp-252] == 0x1100` | `[sp-252] != 0x1100` |
| `beq` at 0x6fdac | `[sp-252] == 0x1300` | `[sp-252] != 0x1300` |
| `beq` at 0x6fdb8 | `[sp-252] == 0x1400` | `[sp-252] != 0x1400` |
| `beq` at 0x6fdc8 | `[sp-252] == 0x3000` | `[sp-252] != 0x3000` |
| `bhi` at 0x6fdd4 | `[sp-252] > 0x3000 (unsigned, 32-bit)` | `[sp-252] <= 0x3000 (unsigned, 32-bit)` |
| `beq` at 0x6fde0 | `[sp-252] == 0x2100` | `[sp-252] != 0x2100` |
| `beq` at 0x6fdec | `[sp-252] == 0x2200` | `[sp-252] != 0x2200` |
| `beq` at 0x6fdfc | `[sp-252] == 0x3100` | `[sp-252] != 0x3100` |
| `beq` at 0x6fe08 | `[sp-252] == 0x3200` | `[sp-252] != 0x3200` |
| `beq` at 0x6fe18 | `[sp-252] == 0x6000` | `[sp-252] != 0x6000` |
| `bhi` at 0x6fe24 | `[sp-252] > 0x6000 (unsigned, 32-bit)` | `[sp-252] <= 0x6000 (unsigned, 32-bit)` |
| `beq` at 0x6fe30 | `[sp-252] == 0x4000` | `[sp-252] != 0x4000` |
| `bhi` at 0x6fe3c | `[sp-252] > 0x4000 (unsigned, 32-bit)` | `[sp-252] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0x6fe48 | `[sp-252] == 0x3400` | `[sp-252] != 0x3400` |
| `beq` at 0x6fe54 | `[sp-252] == 0x3500` | `[sp-252] != 0x3500` |
| `beq` at 0x6fe64 | `[sp-252] == 0x4100` | `[sp-252] != 0x4100` |
| `beq` at 0x6fe70 | `[sp-252] == 0x5000` | `[sp-252] != 0x5000` |
| `beq` at 0x6fe80 | `[sp-252] == 0x7200` | `[sp-252] != 0x7200` |
| `bhi` at 0x6fe8c | `[sp-252] > 0x7200 (unsigned, 32-bit)` | `[sp-252] <= 0x7200 (unsigned, 32-bit)` |
| `beq` at 0x6fe98 | `[sp-252] == 0x7000` | `[sp-252] != 0x7000` |
| `beq` at 0x6fea4 | `[sp-252] == 0x7100` | `[sp-252] != 0x7100` |
| `beq` at 0x6feb4 | `[sp-252] == 0x7400` | `[sp-252] != 0x7400` |
| `bhi` at 0x6fec0 | `[sp-252] > 0x7400 (unsigned, 32-bit)` | `[sp-252] <= 0x7400 (unsigned, 32-bit)` |
| `beq` at 0x6fecc | `[sp-252] == 0x7300` | `[sp-252] != 0x7300` |
| `beq` at 0x6fedc | `[sp-252] == 0x7500` | `[sp-252] != 0x7500` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6ffdc | `r0` | `ret(0x6aa60)` |

## `env_pwrdwn`

- `env_pwrdwn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7111c to 0x71344.
  Established. The absolute word at `0x12d7a0` receives `0x0` and later `0x1` at the included store sites (`str` at 0x712f0, `str` at 0x71304).
  The unnamed `0x6f93c` call at `0x712e4` carries `0x2`, `0x4102`, `0x164`, `sp-124` (`bl` at 0x712e4).
  Signed `[sp-128] <= 0x0` takes `0x712b0`, while positive values fall through (`ble` at 0x712a8).
  Power-down request policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7112c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x71130 | load | 4 | `abs 0x71318` | `-` |
| `ldm` at 0x7113c | load | 4 | `abs 0xfd448` | `-` |
| `ldm` at 0x7113c | load | 4 | `abs 0xfd44c` | `-` |
| `ldm` at 0x7113c | load | 4 | `abs 0xfd450` | `-` |
| `ldm` at 0x7113c | load | 4 | `abs 0xfd454` | `-` |
| `stm` at 0x71140 | store | 4 | `sp -0x2c` | `[0xfd448]` |
| `stm` at 0x71140 | store | 4 | `sp -0x28` | `[0xfd44c]` |
| `stm` at 0x71140 | store | 4 | `sp -0x24` | `[0xfd450]` |
| `stm` at 0x71140 | store | 4 | `sp -0x20` | `[0xfd454]` |
| `ldm` at 0x71144 | load | 4 | `abs 0xfd458` | `-` |
| `ldm` at 0x71144 | load | 4 | `abs 0xfd45c` | `-` |
| `stm` at 0x71148 | store | 4 | `sp -0x1c` | `[0xfd458]` |
| `stm` at 0x71148 | store | 4 | `sp -0x18` | `[0xfd45c]` |
| `str` at 0x71150 | store | 4 | `sp -0x80` | `0x1e` |
| `ldr` at 0x71154 | load | 4 | `abs 0x7131c` | `-` |
| `ldr` at 0x71178 | load | 4 | `abs 0x71320` | `-` |
| `ldr` at 0x7117c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x71184 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x71188 | load | 4 | `abs 0x71324` | `-` |
| `str` at 0x711bc | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x711c4 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x711cc | load | 4 | `abs 0x71328` | `-` |
| `ldr` at 0x711d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x711dc | load | 4 | `sp -0x80` | `-` |
| `str` at 0x711e4 | store | 4 | `sp -0x80` | `[sp-128]-5` |
| `ldr` at 0x71208 | load | 4 | `abs 0x7132c` | `-` |
| `str` at 0x71218 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x71220 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x71228 | load | 4 | `abs 0x71330` | `-` |
| `ldr` at 0x7122c | load | 4 | `abs 0x71334` | `-` |
| `ldr` at 0x71238 | load | 4 | `abs 0x7131c` | `-` |
| `ldr` at 0x71244 | load | 4 | `abs 0x7131c` | `-` |
| `ldr` at 0x71264 | load | 4 | `abs 0x71338` | `-` |
| `str` at 0x71274 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7127c | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x71284 | load | 4 | `abs 0x71330` | `-` |
| `ldr` at 0x71288 | load | 4 | `abs 0x71334` | `-` |
| `ldr` at 0x71294 | load | 4 | `abs 0x7131c` | `-` |
| `ldr` at 0x712a0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x712b8 | load | 4 | `abs 0x7133c` | `-` |
| `str` at 0x712c8 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x712d0 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x712d8 | load | 4 | `abs 0x71330` | `-` |
| `ldr` at 0x712e8 | load | 4 | `abs 0x71340` | `-` |
| `str` at 0x712f0 | store | 4 | `abs 0x12d7a0` | `0x0` |
| `ldr` at 0x712fc | load | 4 | `abs 0x71340` | `-` |
| `str` at 0x71304 | store | 4 | `abs 0x12d7a0` | `0x1` |
| `ldr` at 0x71308 | load | 4 | `abs 0x7131c` | `-` |
| `ldm` at 0x71314 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x71314 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x71314 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7115c | `(unknown callee) 0x20cd0` | `0x13319c` | `0x0` | `[0xfd450]` | `0x1e` |
| `bl` at 0x71180 | `(unknown callee) 0x197a8` | `sp-124` | `0xfd460` | `[sp-128]` | `sp-124` |
| `bl` at 0x711d8 | `(unknown callee) 0x6f93c` | `0x2` | `0x4101` | `?` | `sp-124` |
| `bl` at 0x711ec | `(unknown callee) 0x71344` | `0x1f4` | `?` | `?` | `[sp-128]-5` |
| `bl` at 0x711f0 | `(unknown callee) 0x375e4` | `ret(0x71344)` | `?` | `?` | `?` |
| `bl` at 0x7120c | `(unknown callee) 0x197a8` | `sp-124` | `0xfd484` | `?` | `sp-124` |
| `bl` at 0x71234 | `(unknown callee) 0x6f93c` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x7123c | `(unknown callee) 0x2105c` | `0x13319c` | `?` | `?` | `?` |
| `bl` at 0x7124c | `(unknown callee) 0x20cd0` | `0x13319c` | `0x0` | `?` | `ret(0x375e4)` |
| `bl` at 0x71268 | `(unknown callee) 0x197a8` | `sp-124` | `0xfd4ac` | `?` | `sp-124` |
| `bl` at 0x71290 | `(unknown callee) 0x6f93c` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x71298 | `(unknown callee) 0x2105c` | `0x13319c` | `?` | `?` | `?` |
| `bl` at 0x712bc | `(unknown callee) 0x197a8` | `sp-124` | `0xfd4dc` | `?` | `sp-124` |
| `bl` at 0x712e4 | `(unknown callee) 0x6f93c` | `0x2` | `0x4102` | `0x164` | `sp-124` |
| `bl` at 0x712f8 | `(unknown callee) 0x37174` | `0x4` | `?` | `0x12d7a0` | `0x0` |
| `bl` at 0x7130c | `(unknown callee) 0x2105c` | `0x13319c` | `?` | `0x12d7a0` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x71168 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `bne` at 0x711fc | `ret(0x375e4) != 0x0` | `ret(0x375e4) == 0x0` |
| `bne` at 0x71258 | `ret(0x20cd0) != 0x0` | `ret(0x20cd0) == 0x0` |
| `ble` at 0x712a8 | `[sp-128] <= 0x0 (signed, 32-bit)` | `[sp-128] > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x71314 | `r0` | `?` |

## `EnvQuery`

- `EnvQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x767f8 to 0x76bb0.
  Established. An included word store writes `ret(0x1943c)` through `[sp-24] +0x0` (`str` at 0x7688c).
  The unnamed `0x1d444` call at `0x76814` carries `0x6` and entry `r1`, `r2`, `r3` (`bl` at 0x76814).
  Unsigned `[0x133214] <= 0x3` selects an unresolved computed edge, while larger values fall through (`ldrls` at 0x7699c).
  Environment query mode meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x76808 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7680c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x76828 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x76844 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x76848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7684c | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x76850 | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x76854 | load | 4 | `abs 0x133218` | `-` |
| `ldr` at 0x76858 | load | 4 | `abs 0x13321c` | `-` |
| `ldr` at 0x76860 | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x76864 | load | 4 | `abs 0x133220` | `-` |
| `str` at 0x7688c | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x76890 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x7689c | load | 4 | `abs 0x76b70` | `-` |
| `ldr` at 0x768ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x768b0 | store | 4 | `[sp-24] +0x0` | `ret(0x192bc)` |
| `ldr` at 0x768b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x768bc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x768c0 | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0x768c4 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x768cc | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x768d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x768dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x768e0 | load | 4 | `abs 0x76b74` | `-` |
| `ldr` at 0x768e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x768ec | load | 4 | `abs 0x76b78` | `-` |
| `ldr` at 0x768f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x768f8 | load | 4 | `abs 0x76b7c` | `-` |
| `ldr` at 0x76900 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7690c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76910 | load | 4 | `abs 0x76b80` | `-` |
| `ldr` at 0x76918 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7691c | load | 4 | `abs 0x76b84` | `-` |
| `ldr` at 0x76924 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76928 | load | 4 | `abs 0x76b88` | `-` |
| `ldr` at 0x76930 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76934 | load | 4 | `abs 0x76b8c` | `-` |
| `ldr` at 0x76940 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76944 | load | 4 | `abs 0x76b90` | `-` |
| `ldr` at 0x7694c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7695c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76964 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0x76968 | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x7696c | load | 4 | `abs 0x133208` | `-` |
| `str` at 0x76970 | store | 4 | `sp -0x2c` | `[0x133208]` |
| `ldr` at 0x76974 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x76980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x76990 | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x76994 | load | 4 | `abs 0x133214` | `-` |
| `ldrls` at 0x7699c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x769f4 | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x769f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x769fc | load | 4 | `abs 0x76ba4` | `-` |
| `ldr` at 0x76a00 | load | 4 | `abs 0x133214` | `-` |
| `ldr` at 0x76a0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76a10 | load | 4 | `abs 0x76ba8` | `-` |
| `ldr` at 0x76a1c | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x76a20 | load | 4 | `abs 0x76b6c` | `-` |
| `ldr` at 0x76a24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76a28 | load | 4 | `abs 0x76bac` | `-` |
| `ldr` at 0x76a2c | load | 4 | `abs 0x133208` | `-` |
| `ldr` at 0x76a30 | load | 4 | `abs 0x133214` | `-` |
| `ldr` at 0x76a38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76a48 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76a50 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0x76a54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x76a64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76a68 | load | 4 | `abs 0x76b94` | `-` |
| `ldr` at 0x76a70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76a80 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76a88 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0x76a8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76a94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76aa4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76aac | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0x76ab0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76ab8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76ac8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76ad0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0x76ad4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76adc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76aec | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76af4 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x19538))` |
| `ldr` at 0x76af8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x76b04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b08 | load | 4 | `abs 0x76b74` | `-` |
| `ldr` at 0x76b10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b14 | load | 4 | `abs 0x76b78` | `-` |
| `ldr` at 0x76b1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b20 | load | 4 | `abs 0x76b7c` | `-` |
| `ldr` at 0x76b28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76b34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b38 | load | 4 | `abs 0x76b80` | `-` |
| `ldr` at 0x76b40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b44 | load | 4 | `abs 0x76b84` | `-` |
| `ldr` at 0x76b4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b50 | load | 4 | `abs 0x76b88` | `-` |
| `ldr` at 0x76b58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76b5c | load | 4 | `abs 0x76b8c` | `-` |
| `ldm` at 0x76b68 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x76b68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76b68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76b68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76814 | `(unknown callee) 0x1d444` | `0x6` | `r1` | `r2` | `r3` |
| `bl` at 0x76830 | `(unknown callee) 0x1d444` | `0x7` | `?` | `?` | `?` |
| `bl` at 0x76884 | `(unknown callee) 0x1943c` | `?` | `[0x133218]` | `(([0x133218] + [0x13321c]) + [0x133220])` | `?` |
| `bl` at 0x768a0 | `(unknown callee) 0x192bc` | `0xfdfe0` | `?` | `?` | `[[sp-24]]` |
| `bl` at 0x768e4 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe010` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x768f0 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe044` | `?` | `?` |
| `bl` at 0x768fc | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe078` | `?` | `?` |
| `bl` at 0x76914 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe0ac` | `?` | `[sp-36]` |
| `bl` at 0x76920 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe0e0` | `?` | `?` |
| `bl` at 0x7692c | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe114` | `?` | `?` |
| `bl` at 0x76938 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe144` | `?` | `?` |
| `bl` at 0x76948 | `(unknown callee) 0x19808` | `[sp-40]` | `0xfe178` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x76950 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x76a04 | `(unknown callee) 0x197a8` | `([sp-40] + ret(0x19538))` | `0xfe21c` | `[0x133214]` | `0x133208` |
| `bl` at 0x76a14 | `(unknown callee) 0x19808` | `([sp-40] + ret(0x19538))` | `0xfe24c` | `ret(0x19538)` | `[0x133208]` |
| `bl` at 0x76a34 | `(unknown callee) 0x197a8` | `([sp-40] + ret(0x19538))` | `0xfe25c` | `[0x133208]` | `[0x133214]` |
| `bl` at 0x76a3c | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x76a6c | `(unknown callee) 0x195e0` | `([sp-40] + ret(0x19538))` | `0xfdce8` | `ret(0x19538)` | `[sp-28]` |
| `bl` at 0x76a74 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x76a90 | `(unknown callee) 0x75740` | `([sp-40] + ret(0x19538))` | `?` | `ret(0x19538)` | `([sp-40] + ret(0x19538))` |
| `bl` at 0x76a98 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x76ab4 | `(unknown callee) 0x75b40` | `([sp-40] + ret(0x19538))` | `?` | `ret(0x19538)` | `([sp-40] + ret(0x19538))` |
| `bl` at 0x76abc | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x76ad8 | `(unknown callee) 0x75e08` | `([sp-40] + ret(0x19538))` | `?` | `ret(0x19538)` | `([sp-40] + ret(0x19538))` |
| `bl` at 0x76ae0 | `(unknown callee) 0x19538` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x76b0c | `(unknown callee) 0x195e0` | `([sp-40] + ret(0x19538))` | `0xfe010` | `ret(0x19538)` | `[sp-32]` |
| `bl` at 0x76b18 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe044` | `?` | `?` |
| `bl` at 0x76b24 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe078` | `?` | `?` |
| `bl` at 0x76b3c | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe0ac` | `?` | `[sp-36]` |
| `bl` at 0x76b48 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe0e0` | `?` | `?` |
| `bl` at 0x76b54 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe114` | `?` | `?` |
| `bl` at 0x76b60 | `(unknown callee) 0x195e0` | `[sp-40]` | `0xfe144` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x76898 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0x768d8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x76908 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x7697c | `[0x133208] == 0x0` | `[0x133208] != 0x0` |
| `beq` at 0x76988 | `[0x133208] == 0x1` | `[0x133208] != 0x1` |
| `ldrls` at 0x7699c | `[0x133214] <= 0x3 (unsigned, 32-bit)` | `[0x133214] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x76a5c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x76b00 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x76b30 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76b68 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7699c).
