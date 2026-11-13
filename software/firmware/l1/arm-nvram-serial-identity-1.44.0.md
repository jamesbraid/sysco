# ARM NVRAM and serial identity paths in 1.44.0

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
Computed transfers in nvramInit, nvramSet, nvramGet, brickGetSerialSlab and brickGetSerial
leave unresolved destinations. Their tables and returns cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/nvram-serial-identity-facts.txt) retain all tool sections.

## `nvramInit`

- `nvramInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x19ec4 to 0x1ad7c.
  Established. The absolute word at `0x12ef40` receives `[0x12ef40]+1` (`str` at 0x19f18). A known
  path stores `0x3` as a byte through `[0x12eedc] +0x7c8` (`strb` at 0x1a884). The unsigned
  condition `ret(0x1d444) > 0x1` takes `0x1aa10`, while less or equal values fall through (`bhi` at 0x1aa00). NVRAM initialization across unresolved dispatch destinations remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x19ed8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x19edc | load | 4 | `abs 0x1ad0c` | `-` |
| `ldr` at 0x19ee0 | load | 4 | `abs 0x12ef40` | `-` |
| `ldr` at 0x19eec | load | 4 | `abs 0x1ad10` | `-` |
| `str` at 0x19ef0 | store | 4 | `sp -0x68` | `0xeb708` |
| `ldr` at 0x19ef8 | load | 4 | `abs 0x1ad14` | `-` |
| `ldr` at 0x19f08 | load | 4 | `abs 0x1ad0c` | `-` |
| `ldr` at 0x19f0c | load | 4 | `abs 0x1ad0c` | `-` |
| `ldr` at 0x19f10 | load | 4 | `abs 0x12ef40` | `-` |
| `str` at 0x19f18 | store | 4 | `abs 0x12ef40` | `[0x12ef40]+1` |
| `ldr` at 0x19f1c | load | 4 | `abs 0x1ad18` | `-` |
| `ldr` at 0x19f2c | load | 4 | `abs 0x1ad1c` | `-` |
| `str` at 0x19f30 | store | 4 | `sp -0x24` | `0xeb728` |
| `ldr` at 0x19f34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x19f38 | load | 4 | `abs 0x1ad20` | `-` |
| `ldr` at 0x19f3c | load | 4 | `abs 0x1ad24` | `-` |
| `ldr` at 0x19f48 | load | 4 | `abs 0x1ad28` | `-` |
| `str` at 0x19f4c | store | 4 | `abs 0x12ef64` | `ret(0x1967c)` |
| `ldr` at 0x19f50 | load | 4 | `abs 0x12ef64` | `-` |
| `ldr` at 0x19f64 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0x19f70 | load | 4 | `abs 0x1ad2c` | `-` |
| `str` at 0x19f74 | store | 4 | `sp -0x68` | `0xeb738` |
| `ldr` at 0x19f78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x19f7c | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x19f80 | store | 4 | `sp -0x60` | `ret(0x1f8e0)` |
| `ldr` at 0x19f88 | load | 4 | `abs 0x1ad14` | `-` |
| `ldr` at 0x19f98 | load | 4 | `abs 0x1ad30` | `-` |
| `ldr` at 0x19fa8 | load | 4 | `abs 0x1ad28` | `-` |
| `ldr` at 0x19fac | load | 4 | `abs 0x12ef64` | `-` |
| `ldr` at 0x19fb0 | load | 4 | `abs 0x1ad30` | `-` |
| `ldr` at 0x19fd0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0x19fdc | load | 4 | `abs 0x1ad34` | `-` |
| `str` at 0x19fe0 | store | 4 | `sp -0x68` | `0xeb75c` |
| `ldr` at 0x19fe4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x19fe8 | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x19fec | store | 4 | `sp -0x60` | `ret(0x1f8e0)` |
| `ldr` at 0x19ff4 | load | 4 | `abs 0x1ad14` | `-` |
| `ldr` at 0x1a004 | load | 4 | `abs 0x1ad38` | `-` |
| `ldr` at 0x1a008 | load | 4 | `abs 0x1ad3c` | `-` |
| `ldr` at 0x1a018 | load | 4 | `abs 0x1ad40` | `-` |
| `ldr` at 0x1a01c | load | 4 | `abs 0x1ad30` | `-` |
| `str` at 0x1a020 | store | 4 | `abs 0x12eed4` | `0x297694` |
| `ldr` at 0x1a024 | load | 4 | `abs 0x1ad40` | `-` |
| `ldr` at 0x1a028 | load | 4 | `abs 0x12eed4` | `-` |
| `ldr` at 0x1a048 | load | 4 | `abs 0x1ad40` | `-` |
| `ldr` at 0x1a04c | load | 4 | `abs 0x12eed4` | `-` |
| `ldrb` at 0x1a050 | load | 1 | `[0x12eed4] +0x0` | `-` |
| `ldr` at 0x1a064 | load | 4 | `abs 0x1ad40` | `-` |
| `ldr` at 0x1a068 | load | 4 | `abs 0x12eed4` | `-` |
| `ldrb` at 0x1a06c | load | 1 | `[0x12eed4] +0x1` | `-` |
| `ldr` at 0x1a07c | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a080 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a08c | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a090 | load | 4 | `abs 0x1ad40` | `-` |
| `ldr` at 0x1a094 | load | 4 | `abs 0x12eed4` | `-` |
| `ldrb` at 0x1a098 | load | 1 | `[0x12eed4] +0x1` | `-` |
| `ldr` at 0x1a0a0 | load | 4 | `abs 0x1ad30` | `-` |
| `str` at 0x1a0a8 | store | 4 | `abs 0x12eedc` | `?` |
| `ldr` at 0x1a0ac | load | 4 | `abs 0x1ad48` | `-` |
| `ldr` at 0x1a0b0 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a0b4 | load | 4 | `abs 0x12eedc` | `-` |
| `str` at 0x1a0bc | store | 4 | `abs 0x12eee0` | `[0x12eedc]+2352` |
| `ldr` at 0x1a0c0 | load | 4 | `abs 0x1ad4c` | `-` |
| `ldr` at 0x1a0c4 | load | 4 | `abs 0x1ad50` | `-` |
| `str` at 0x1a0c8 | store | 4 | `abs 0x12eee4` | `0x29968c` |
| `ldr` at 0x1a0cc | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a0d0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldrb` at 0x1a0d4 | load | 1 | `[0x12eedc] +0x0` | `-` |
| `ldrls` at 0x1a0e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1a788 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a78c | load | 4 | `abs 0x12eedc` | `-` |
| `str` at 0x1a7ac | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1a7b0 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a7b4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a7b8 | load | 4 | `abs 0x1ad60` | `-` |
| `ldrb` at 0x1a7bc | load | 1 | `[0x12eedc] +0x7c8` | `-` |
| `ldr` at 0x1a7cc | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a7d0 | load | 4 | `abs 0x12eedc` | `-` |
| `str` at 0x1a7fc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1a804 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a808 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a80c | load | 4 | `abs 0x1ad64` | `-` |
| `strb` at 0x1a814 | store | 1 | `[0x12eedc] +0x7cc` | `0x0` |
| `ldr` at 0x1a818 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a81c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a820 | load | 4 | `[0x12eedc] +0x7cc` | `-` |
| `str` at 0x1a824 | store | 4 | `sp -0x5c` | `[[0x12eedc]+1996]` |
| `ldr` at 0x1a828 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x1a830 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x1a834 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x1a838 | store | 4 | `[0x12eedc] +0x7cc` | `[sp-92]` |
| `ldr` at 0x1a83c | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a840 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a858 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a85c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a874 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a878 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a87c | load | 4 | `abs 0x1ad60` | `-` |
| `strb` at 0x1a884 | store | 1 | `[0x12eedc] +0x7c8` | `0x3` |
| `ldr` at 0x1a888 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a88c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a890 | load | 4 | `abs 0x1ad58` | `-` |
| `ldr` at 0x1a894 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a898 | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1a8a4 | store | 1 | `[0x12eedc] +0x3bf` | `(ret(0x1ffa8) & 0xff)` |
| `ldr` at 0x1a8a8 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a8ac | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a8b0 | load | 4 | `abs 0x1ad68` | `-` |
| `ldr` at 0x1a8b4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a8b8 | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1a8d0 | store | 1 | `[0x12eedc] +0x87b` | `(ret(0x1ffa8) & 0xff)` |
| `ldr` at 0x1a8d4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a8d8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldrb` at 0x1a8e0 | load | 1 | `[0x12eedc] +0x780` | `-` |
| `ldr` at 0x1a8f0 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a8f4 | load | 4 | `abs 0x12eedc` | `-` |
| `str` at 0x1a91c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1a924 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a928 | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1a934 | store | 1 | `[0x12eedc] +0x780` | `0x1` |
| `ldr` at 0x1a938 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a93c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a940 | load | 4 | `abs 0x1ad6c` | `-` |
| `ldr` at 0x1a944 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a948 | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1a95c | store | 1 | `[0x12eedc] +0x7a3` | `(ret(0x1ffa8) & 0xff)` |
| `ldr` at 0x1a960 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1a974 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a978 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a980 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a988 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a994 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a998 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a9a4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a9a8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a9c4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a9c8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1a9d4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1a9d8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aa30 | load | 4 | `abs 0x1ad4c` | `-` |
| `ldr` at 0x1aa34 | load | 4 | `abs 0x1ad48` | `-` |
| `ldr` at 0x1aa38 | load | 4 | `abs 0x12eee4` | `-` |
| `ldr` at 0x1aa3c | load | 4 | `abs 0x12eee0` | `-` |
| `ldr` at 0x1aa44 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aa48 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aa4c | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aa50 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aa54 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1aa58 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1aa68 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aa6c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aa70 | load | 4 | `abs 0x1ad48` | `-` |
| `ldr` at 0x1aa74 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1aa78 | load | 4 | `abs 0x12eee0` | `-` |
| `ldr` at 0x1aa84 | load | 4 | `abs 0x1ad48` | `-` |
| `ldr` at 0x1aa88 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aa8c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aa90 | load | 4 | `abs 0x12eee0` | `-` |
| `ldr` at 0x1aa94 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `str` at 0x1aa9c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x1aaa0 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aaa4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aaa8 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aaac | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aab0 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1aab4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1aabc | store | 4 | `[0x12eedc] +0x784` | `([[0x12eedc]+1924] + [sp-36])` |
| `ldr` at 0x1aac0 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aac4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aac8 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aacc | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aad0 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1aad4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1aadc | store | 4 | `[0x12eedc] +0x788` | `([[0x12eedc]+1928] + [sp-36])` |
| `ldr` at 0x1aae0 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aae4 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aae8 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1aaec | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1aaf0 | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1aaf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1aafc | store | 4 | `[0x12eedc] +0x78c` | `([[0x12eedc]+1932] + [sp-36])` |
| `ldr` at 0x1ab00 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab04 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab08 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab0c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab10 | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1ab14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1ab1c | store | 4 | `[0x12eedc] +0x790` | `([[0x12eedc]+1936] + [sp-36])` |
| `ldr` at 0x1ab20 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab24 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab28 | load | 4 | `abs 0x1ad6c` | `-` |
| `ldr` at 0x1ab2c | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab30 | load | 4 | `abs 0x12eedc` | `-` |
| `strb` at 0x1ab44 | store | 1 | `[0x12eedc] +0x7a3` | `(ret(0x1ffa8) & 0xff)` |
| `ldr` at 0x1ab48 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab4c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab58 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab5c | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab74 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab78 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab7c | load | 4 | `abs 0x1ad48` | `-` |
| `ldr` at 0x1ab80 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1ab84 | load | 4 | `abs 0x12eee0` | `-` |
| `ldr` at 0x1ab90 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ab94 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ab98 | load | 4 | `abs 0x1ad4c` | `-` |
| `ldr` at 0x1ab9c | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1aba0 | load | 4 | `abs 0x12eee4` | `-` |
| `ldr` at 0x1abac | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1abb0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1abb4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1abb8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1abbc | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1abc0 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1abcc | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1abd0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1abd4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1abd8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1abdc | load | 4 | `[0x12eedc] +0x78c` | `-` |
| `ldr` at 0x1abe0 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1abec | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1abf0 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1abf4 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1abf8 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1abfc | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1ac00 | load | 4 | `[0x12eedc] +0x784` | `-` |
| `ldr` at 0x1ac0c | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ac10 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ac14 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ac18 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ac1c | load | 4 | `[0x12eedc] +0x790` | `-` |
| `ldr` at 0x1ac20 | load | 4 | `[0x12eedc] +0x788` | `-` |
| `ldr` at 0x1ac30 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ac34 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ac38 | load | 4 | `abs 0x1ad48` | `-` |
| `ldr` at 0x1ac3c | load | 4 | `abs 0x12eee0` | `-` |
| `str` at 0x1ac40 | store | 4 | `[0x12eedc] +0x784` | `[0x12eee0]` |
| `ldr` at 0x1ac44 | load | 4 | `abs 0x1ad44` | `-` |
| `ldr` at 0x1ac48 | load | 4 | `abs 0x12eedc` | `-` |
| `ldr` at 0x1ac4c | load | 4 | `abs 0x1ad4c` | `-` |
| `ldr` at 0x1ac50 | load | 4 | `abs 0x12eee4` | `-` |
| `str` at 0x1ac54 | store | 4 | `[0x12eedc] +0x788` | `[0x12eee4]` |
| `ldr` at 0x1ac5c | load | 4 | `abs 0x1ad18` | `-` |
| `ldr` at 0x1ac60 | load | 4 | `abs 0x1ad70` | `-` |
| `ldrb` at 0x1ac64 | load | 1 | `abs 0x12eee8` | `-` |
| `strb` at 0x1ac6c | store | 1 | `abs 0x12eee8` | `([0x12eee8]:1+1 & 0xff)` |
| `ldr` at 0x1ac74 | load | 4 | `abs 0x1ad74` | `-` |
| `str` at 0x1ac78 | store | 4 | `(unknown)` | `0xeb794` |
| `ldr` at 0x1ac7c | load | 4 | `abs 0x1ad70` | `-` |
| `ldrb` at 0x1ac80 | load | 1 | `abs 0x12eee8` | `-` |
| `ldr` at 0x1ac8c | load | 4 | `abs 0x1ad70` | `-` |
| `strb` at 0x1ac94 | store | 1 | `abs 0x12eee8` | `0x0` |
| `ldr` at 0x1ac98 | load | 4 | `abs 0x1ad18` | `-` |
| `ldr` at 0x1ac9c | load | 4 | `abs 0x1ad70` | `-` |
| `ldrb` at 0x1aca0 | load | 1 | `abs 0x12eee8` | `-` |
| `ldr` at 0x1aca8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1acb4 | load | 4 | `abs 0x1ad18` | `-` |
| `ldr` at 0x1acb8 | load | 4 | `abs 0x1ad70` | `-` |
| `ldrb` at 0x1acbc | load | 1 | `abs 0x12eee8` | `-` |
| `ldr` at 0x1acc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1accc | load | 4 | `abs 0x1ad70` | `-` |
| `ldr` at 0x1acd0 | load | 4 | `abs 0x1ad70` | `-` |
| `ldrb` at 0x1acd4 | load | 1 | `abs 0x12eee8` | `-` |
| `strb` at 0x1acdc | store | 1 | `abs 0x12eee8` | `([0x12eee8]:1+1 & 0xff)` |
| `ldr` at 0x1acf4 | load | 4 | `abs 0x1ad78` | `-` |
| `ldm` at 0x1ad08 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1ad08 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1ad08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1ad08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1ad08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x19f04 | `(unknown callee) 0x200a0` | `0x9` | `0xeb690` | `0x71` | `0x0` |
| `bl` at 0x19f28 | `(unknown callee) 0x19748` | `0x12ef44` | `0x0` | `0x20` | `[0x12ef40]+1` |
| `bl` at 0x19f40 | `(unknown callee) 0x1967c` | `0xeb728` | `0x1042` | `0x1b6` | `0xeb728` |
| `bl` at 0x19f5c | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `ret(0x1967c)` | `[0x12ef64]` |
| `bl` at 0x19f68 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0x19f94 | `(unknown callee) 0x200a0` | `0x9` | `0xeb690` | `0x82` | `0x0` |
| `bl` at 0x19fa4 | `(unknown callee) 0x19748` | `0x297694` | `0x0` | `0x2000` | `?` |
| `bl` at 0x19fb8 | `(unknown callee) 0x197fc` | `[0x12ef64]` | `0x297694` | `0x2000` | `0x12ef64` |
| `bl` at 0x19fc8 | `(unknown callee) 0x193d0` | `ret(0x197fc)` | `?` | `?` | `ret(0x197fc)` |
| `bl` at 0x19fd4 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0x1a000 | `(unknown callee) 0x200a0` | `0x9` | `0xeb690` | `0x88` | `0x0` |
| `bl` at 0x1a014 | `(unknown callee) 0x20bb4` | `0x12eeec` | `0xeb788` | `0x1` | `0x9` |
| `bl` at 0x1a030 | `(unknown callee) 0x1ff3c` | `[0x12eed4]` | `0x8` | `0x12eed4` | `0x12eed4` |
| `bl` at 0x1a044 | `(unknown callee) 0x1b4ec` | `ret(0x1ff3c)` | `?` | `?` | `?` |
| `bl` at 0x1a078 | `(unknown callee) 0x1b4ec` | `?` | `?` | `?` | `?` |
| `bl` at 0x1a794 | `(unknown callee) 0x1ff3c` | `[0x12eedc]` | `0x3c0` | `?` | `0x12eedc` |
| `bl` at 0x1a7e4 | `(unknown callee) 0x1ff3c` | `[0x12eedc]+1992` | `0xb4` | `[0x12eedc]` | `[0x12eedc]+1992` |
| `bl` at 0x1a854 | `(unknown callee) 0x19748` | `[0x12eedc]+2000` | `0x0` | `0xa8` | `[0x12eedc]+2000` |
| `bl` at 0x1a870 | `(unknown callee) 0x19748` | `[0x12eedc]+952` | `0x0` | `0x7` | `[0x12eedc]+952` |
| `bl` at 0x1a8a0 | `(unknown callee) 0x1ffa8` | `[0x12eedc]` | `0x3c0` | `0x7c8` | `0x12eedc` |
| `bl` at 0x1a8cc | `(unknown callee) 0x1ffa8` | `[0x12eedc]+1992` | `0xb4` | `?` | `[0x12eedc]+1992` |
| `bl` at 0x1a904 | `(unknown callee) 0x1ff3c` | `[0x12eedc]+1920` | `0x24` | `[0x12eedc]` | `[0x12eedc]+1920` |
| `bl` at 0x1a958 | `(unknown callee) 0x1ffa8` | `[0x12eedc]+1920` | `0x24` | `0x780` | `[0x12eedc]+1920` |
| `bl` at 0x1a96c | `(unknown callee) 0x1ad7c` | `?` | `?` | `?` | `[sp-32]` |
| `bl` at 0x1a990 | `(unknown callee) 0x19610` | `[0x12eedc]+960` | `[0x12eedc]` | `0x3c0` | `[0x12eedc]+960` |
| `bl` at 0x1a9c0 | `(unknown callee) 0x19610` | `[0x12eedc]+2172` | `[0x12eedc]+1992` | `0xb4` | `[0x12eedc]+1992` |
| `bl` at 0x1a9ec | `(unknown callee) 0x19610` | `[0x12eedc]+1956` | `[0x12eedc]+1920` | `0x24` | `[0x12eedc]+1920` |
| `bl` at 0x1a9f4 | `(unknown callee) 0x1d444` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x1aa0c | `(unknown callee) 0x1b5f0` | `0x11` | `0x2` | `?` | `ret(0x1d444)` |
| `bl` at 0x1aa14 | `(unknown callee) 0x1d444` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x1aa2c | `(unknown callee) 0x1b5f0` | `0x3d` | `0x2` | `?` | `ret(0x1d444)` |
| `bl` at 0x1ab40 | `(unknown callee) 0x1ffa8` | `[0x12eedc]+1920` | `0x24` | `[[0x12eedc]+1936]` | `[0x12eedc]+1920` |
| `bl` at 0x1ab70 | `(unknown callee) 0x19610` | `[0x12eedc]+1956` | `[0x12eedc]+1920` | `0x24` | `[0x12eedc]+1920` |
| `bl` at 0x1ac58 | `(unknown callee) 0x1e40c` | `?` | `?` | `[0x12eedc]` | `[0x12eee4]` |
| `bl` at 0x1acc8 | `(unknown callee) 0x1e4ac` | `?` | `?` | `0x12ef44` | `?` |
| `bl` at 0x1ace4 | `(unknown callee) 0x19da8` | `?` | `?` | `?` | `?` |
| `bl` at 0x1acec | `(unknown callee) 0x1d444` | `0x46` | `?` | `?` | `?` |
| `bl` at 0x1ad00 | `(unknown callee) 0x193a0` | `0xeb7b8` | `ret(0x1d444)` | `0x1` | `ret(0x1d444)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x19ee8 | `[0x12ef40] == 0x0` | `[0x12ef40] != 0x0` |
| `bge` at 0x19f58 | `[0x12ef64] >= 0x0 (signed, 32-bit)` | `[0x12ef64] < 0x0 (signed, 32-bit)` |
| `bge` at 0x19fc4 | `ret(0x197fc) >= 0x0 (signed, 32-bit)` | `ret(0x197fc) < 0x0 (signed, 32-bit)` |
| `beq` at 0x1a040 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1a060 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x1a074 | `[[0x12eed4]+1]:1 != 0x0` | `[[0x12eed4]+1]:1 == 0x0` |
| `bne` at 0x1a088 | `[0x12eedc] != 0x0` | `[0x12eedc] == 0x0` |
| `ldrls` at 0x1a0e8 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |
| `beq` at 0x1a7a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1a7c4 | `[[0x12eedc]+1992]:1 == 0x3` | `[[0x12eedc]+1992]:1 != 0x3` |
| `beq` at 0x1a7f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1a8e8 | `[[0x12eedc]+1920]:1 == 0x1` | `[[0x12eedc]+1920]:1 != 0x1` |
| `beq` at 0x1a914 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1a968 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bhi` at 0x1aa00 | `ret(0x1d444) > 0x1 (unsigned, 32-bit)` | `ret(0x1d444) <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x1aa20 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `bne` at 0x1aa64 | `? != ?` | `? == ?` |
| `beq` at 0x1aa80 | `[[0x12eedc]+1924] == [0x12eee0]` | `[[0x12eedc]+1924] != [0x12eee0]` |
| `bne` at 0x1ab8c | `[[0x12eedc]+1924] != [0x12eee0]` | `[[0x12eedc]+1924] == [0x12eee0]` |
| `bne` at 0x1aba8 | `[[0x12eedc]+1928] != [0x12eee4]` | `[[0x12eedc]+1928] == [0x12eee4]` |
| `blo` at 0x1abc8 | `[[0x12eedc]+1932] < [[0x12eedc]+1924] (unsigned, 32-bit)` | `[[0x12eedc]+1932] >= [[0x12eedc]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1abe8 | `[[0x12eedc]+1932] >= [[0x12eedc]+1928] (unsigned, 32-bit)` | `[[0x12eedc]+1932] < [[0x12eedc]+1928] (unsigned, 32-bit)` |
| `blo` at 0x1ac08 | `[[0x12eedc]+1936] < [[0x12eedc]+1924] (unsigned, 32-bit)` | `[[0x12eedc]+1936] >= [[0x12eedc]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1ac28 | `[[0x12eedc]+1936] >= [[0x12eedc]+1928] (unsigned, 32-bit)` | `[[0x12eedc]+1936] < [[0x12eedc]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1ac88 | `[0x12eee8]:1 == 0x0` | `[0x12eee8]:1 != 0x0` |
| `beq` at 0x1acb0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1ad08 | `r0` | `ret(0x193a0)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1a0e8).

## `nvramSet`

- `nvramSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1b5f0 to 0x1d444.
  Established. Entry `r0` and entry `r1` are saved at `sp -0x1c` and `sp -0x20` as words (`str` at 0x1b600, `str` at 0x1b604). The computed transfer is taken when unsigned `r0-1 <= 0xaf`, with
  fall-through when greater (`ldrls` at 0x1b614). The known-edge return record retains entry `r0`
  (`ldm` at 0x1d440). Setting writes beyond the unresolved transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1b600 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1b604 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x1b608 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x1b614 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x1d440 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1d440 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1d440 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1d440 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1d440 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1b614 | `r0-1 <= 0xaf (unsigned, 32-bit)` | `r0-1 > 0xaf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1d440 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1b614).

## `nvramGet`

- `nvramGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1d444 to 0x1e40c.
  Established. The entry selector is saved as a word at `sp -0x14` (`str` at 0x1d454). The unsigned
  condition `r0-1 <= 0xaf` takes the computed transfer, while greater values fall through (`ldrls`
  at 0x1d464). The included fallback writes zero to `sp -0x18` (`str` at 0x1e3e8). NVRAM selector
  meanings across unresolved arms remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1d454 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1d458 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x1d464 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1e3e8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x1e3ec | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1e3f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1e3f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1e3f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1d464 | `r0-1 <= 0xaf (unsigned, 32-bit)` | `r0-1 > 0xaf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1e3f4 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1d464).

## `brickGetSerialSlab`

- `brickGetSerialSlab`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48984 to 0x48b30.
  Established. A nonzero `ret(0xc5738)` takes `0x489dc`, while zero falls through (`bne` at 0x489cc). The computed transfer is taken for unsigned `[sp-28] <= 0x5`, with fall-through when
  greater (`ldrls` at 0x489fc). An included store writes `ret(0x192bc)` to an unknown word target
  (`str` at 0x48b00). Slab serial selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48994 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x4899c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x489a0 | load | 4 | `abs 0x48b20` | `-` |
| `ldr` at 0x489a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x489a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x489d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x489f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x489fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x48a64 | load | 4 | `abs 0x48b24` | `-` |
| `str` at 0x48a68 | store | 4 | `sp -0x2c` | `0xf7a50` |
| `ldr` at 0x48a70 | load | 4 | `abs 0x48b28` | `-` |
| `ldr` at 0x48a74 | load | 4 | `abs 0x48b2c` | `-` |
| `ldr` at 0x48a80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48a94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48a9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x48aa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48abc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48ac4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x48ad0 | load | 4 | `abs 0x48b20` | `-` |
| `ldr` at 0x48ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48ad8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48aec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48af4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x48b00 | store | 4 | `(unknown)` | `ret(0x192bc)` |
| `ldr` at 0x48b04 | load | 4 | `abs 0x48b20` | `-` |
| `ldr` at 0x48b08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48b0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x48b10 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x48b14 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x48b1c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x48b1c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x48b1c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48b1c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48b1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x489c0 | `(unknown callee) 0xc5738` | `0x0` | `sp-36` | `0x132454` | `sp-36` |
| `bl` at 0x489dc | `(unknown callee) 0x488cc` | `ret(0xc5738)` | `?` | `?` | `ret(0xc5738)` |
| `bl` at 0x48a7c | `(unknown callee) 0x200a0` | `0x9` | `0xf7a88` | `0x195` | `0x0` |
| `bl` at 0x48af8 | `(unknown callee) 0x192bc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x489b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x489cc | `ret(0xc5738) != 0x0` | `ret(0xc5738) == 0x0` |
| `beq` at 0x489ec | `? == 0x40` | `? != 0x40` |
| `ldrls` at 0x489fc | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |
| `bne` at 0x48aa4 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x48acc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48b1c | `r0` | `[sp-40]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x489fc).

## `brickGetSerial`

- `brickGetSerial`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48b30 to 0x48da8.
  Established. A nonzero `[0x13241c]` takes `0x48d88`, while zero falls through (`bne` at 0x48b54).
  The byte at `0x12d704` is read (`ldrb` at 0x48b84). An included word assignment at `0x13241c`
  writes `ret(0x192bc)` (`str` at 0x48d84). Serial lookup through unresolved selection arms remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48b44 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x48b48 | load | 4 | `abs 0x48da0` | `-` |
| `ldr` at 0x48b4c | load | 4 | `abs 0x13241c` | `-` |
| `str` at 0x48b78 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x48b80 | load | 4 | `abs 0x48da4` | `-` |
| `ldrb` at 0x48b84 | load | 1 | `abs 0x12d704` | `-` |
| `ldrls` at 0x48b90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x48d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48d1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48d24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x48d30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48d44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48d4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x48d58 | load | 4 | `abs 0x48da0` | `-` |
| `ldr` at 0x48d5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48d70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x48d78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x48d84 | store | 4 | `abs 0x13241c` | `ret(0x192bc)` |
| `ldr` at 0x48d88 | load | 4 | `abs 0x48da0` | `-` |
| `ldr` at 0x48d8c | load | 4 | `abs 0x13241c` | `-` |
| `str` at 0x48d90 | store | 4 | `sp -0x24` | `[0x13241c]` |
| `ldr` at 0x48d94 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x48d9c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x48d9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48d9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48d9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48b64 | `(unknown callee) 0xc5738` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x48d7c | `(unknown callee) 0x192bc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x48b54 | `[0x13241c] != 0x0` | `[0x13241c] == 0x0` |
| `bne` at 0x48b70 | `ret(0xc5738) != 0x0` | `ret(0xc5738) == 0x0` |
| `ldrls` at 0x48b90 | `[0x12d704]:1-35 <= 0x59 (unsigned, 32-bit)` | `[0x12d704]:1-35 > 0x59 (unsigned, 32-bit)` |
| `bne` at 0x48d2c | `? != 0x1` | `? == 0x1` |
| `beq` at 0x48d54 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48d9c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x48b90).

## `networkSerial`

- `networkSerial`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25eac to 0x25fb8.
  Established. The unnamed `0xca584` call carries `0x0`, `0x5`, `0x6e`, `0x44` (`bl` at 0x25f50);
  its zero result takes `0x25f98`, while nonzero falls through (`beq` at 0x25f64). The taken arm
  stores byte `0x1` at `0x132074` (`strb` at 0x25fa0). Serial-network setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25ec8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x25ecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25ee4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x25ef4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x25f08 | store | 1 | `sp -0x15` | `0x5` |
| `strb` at 0x25f10 | store | 1 | `sp -0x16` | `0x6e` |
| `strb` at 0x25f18 | store | 1 | `sp -0x17` | `0x44` |
| `strb` at 0x25f24 | store | 1 | `sp -0x15` | `0x5` |
| `strb` at 0x25f2c | store | 1 | `sp -0x16` | `0x6e` |
| `strb` at 0x25f34 | store | 1 | `sp -0x17` | `0x44` |
| `ldrb` at 0x25f38 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x25f3c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x25f40 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x25f58 | store | 4 | `sp -0x14` | `ret(0xca584)` |
| `ldr` at 0x25f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f74 | load | 4 | `abs 0x25fac` | `-` |
| `str` at 0x25f78 | store | 4 | `sp -0x24` | `0xee978` |
| `str` at 0x25f7c | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x25f84 | load | 4 | `abs 0x25fb0` | `-` |
| `ldr` at 0x25f98 | load | 4 | `abs 0x25fb4` | `-` |
| `strb` at 0x25fa0 | store | 1 | `abs 0x132074` | `0x1` |
| `ldm` at 0x25fa8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x25fa8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x25fa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25ebc | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x25f50 | `(unknown callee) 0xca584` | `0x0` | `0x5` | `0x6e` | `0x44` |
| `bl` at 0x25f6c | `(unknown callee) 0x1fc1c` | `ret(0xca584)` | `?` | `?` | `ret(0xca584)` |
| `bl` at 0x25f90 | `(unknown callee) 0x200a0` | `0x3` | `0xee9b4` | `0x90` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25ed4 | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `bgt` at 0x25ee0 | `[sp-28] > 0x35 (signed, 32-bit)` | `[sp-28] <= 0x35 (signed, 32-bit)` |
| `beq` at 0x25eec | `[sp-28] == 0x21` | `[sp-28] != 0x21` |
| `beq` at 0x25efc | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0x25f64 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x25fa8 | `r0` | `?` |

## `SHUB_ScanSetRackSlot`

- `SHUB_ScanSetRackSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x422ec to 0x42488.
  Established. An included stack byte at `sp -0x21` receives `0x6c` (`strb` at 0x42304). The unnamed
  `0x4182c` call carries `[sp-20]`, `0x101d0100`, `[sp-28]`, unknown (`bl` at 0x42418). A result
  unequal to `0x1` takes `0x42434`, while equality falls through (`bne` at 0x42424). Rack-slot scan
  programming remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x422fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x42304 | store | 1 | `sp -0x21` | `0x6c` |
| `str` at 0x42314 | store | 4 | `sp -0x28` | `ret(0x6a0f8)` |
| `ldr` at 0x42318 | load | 4 | `abs 0x42468` | `-` |
| `ldr` at 0x4231c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x42328 | load | 4 | `abs 0x4246c` | `-` |
| `str` at 0x4232c | store | 4 | `sp -0x38` | `0xf6850` |
| `ldr` at 0x42334 | load | 4 | `abs 0x42470` | `-` |
| `ldr` at 0x42338 | load | 4 | `abs 0x42474` | `-` |
| `ldr` at 0x42344 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42348 | load | 4 | `(unknown)` | `-` |
| `str` at 0x42354 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x42358 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4235c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42368 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4236c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4237c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4238c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42390 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x423a0 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x423a8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x423ac | load | 4 | `abs 0x42468` | `-` |
| `ldr` at 0x423b0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x423bc | load | 4 | `abs 0x42478` | `-` |
| `str` at 0x423c0 | store | 4 | `sp -0x38` | `0xf6868` |
| `ldr` at 0x423c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x423c8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x423cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x423d0 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x423d8 | load | 4 | `abs 0x42470` | `-` |
| `str` at 0x423ec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x423f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x423fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42404 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4240c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42410 | load | 4 | `abs 0x4247c` | `-` |
| `ldr` at 0x42414 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4242c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x42434 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4243c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x42444 | load | 4 | `abs 0x42480` | `-` |
| `ldr` at 0x42448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4244c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42454 | load | 4 | `abs 0x42484` | `-` |
| `str` at 0x42458 | store | 4 | `sp -0x2c` | `0xfffff88e` |
| `ldr` at 0x4245c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x42464 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42464 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42464 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4230c | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `0x6c` |
| `bl` at 0x42340 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x1cd` | `0x0` |
| `bl` at 0x423e4 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x1d8` | `0x0` |
| `bl` at 0x42418 | `(unknown callee) 0x4182c` | `[sp-20]` | `0x101d0100` | `[sp-28]` | `?` |
| `bl` at 0x42450 | `(unknown callee) 0x1e958` | `0xf689c` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42324 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x423b8 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bgt` at 0x423f8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x42424 | `ret(0x4182c) != 0x1` | `ret(0x4182c) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42464 | `r0` | `?` |

## `PBaySdrGetCardSerial`

- `PBaySdrGetCardSerial`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9fef8 to 0xa03c4.
  Established. The byte at `0x12d868` is read (`ldrb` at 0x9ff1c); its inequality with `0xff` takes
  `0x9ff40`, while equality falls through (`bne` at 0x9ff24). An included store clears the word
  through entry `r1 +0x0` (`str` at 0x9ff30). Card serial record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ff08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9ff0c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9ff10 | load | 4 | `abs 0xa039c` | `-` |
| `str` at 0x9ff14 | store | 4 | `sp -0x120` | `0x12d868` |
| `ldr` at 0x9ff18 | load | 4 | `sp -0x120` | `-` |
| `ldrb` at 0x9ff1c | load | 1 | `abs 0x12d868` | `-` |
| `ldr` at 0x9ff28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ff30 | store | 4 | `r1 +0x0` | `0x0` |
| `str` at 0x9ff38 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x9ff40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ff58 | store | 4 | `sp -0x2cc` | `ret(0x1943c)` |
| `ldr` at 0x9ff5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ff60 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x9ff64 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x9ff68 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x9ff78 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x9ff80 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x9ff84 | load | 4 | `abs 0xa03a0` | `-` |
| `ldr` at 0x9ff8c | load | 4 | `abs 0xa03a4` | `-` |
| `ldr` at 0x9ff90 | load | 4 | `abs 0x12d8b4` | `-` |
| `str` at 0x9ff94 | store | 4 | `sp -0x2dc` | `[0x12d8b4]` |
| `ldr` at 0x9ff9c | load | 4 | `abs 0xa03a0` | `-` |
| `str` at 0x9ffa0 | store | 4 | `sp -0x2cc` | `0x12d888` |
| `ldr` at 0x9ffa4 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0x9ffac | store | 1 | `abs 0x12d888` | `0x0` |
| `ldr` at 0x9ffb0 | load | 4 | `abs 0xa03a8` | `-` |
| `strh` at 0x9ffbc | store | 2 | `sp -0x2c2` | `0x0` |
| `ldr` at 0x9ffc0 | load | 4 | `abs 0xa03ac` | `-` |
| `strb` at 0x9ffcc | store | 1 | `sp -0x2c3` | `0x8` |
| `ldr` at 0x9ffd4 | load | 4 | `abs 0xa03a8` | `-` |
| `ldrh` at 0x9ffdc | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0x9ffe0 | load | 4 | `abs 0xa03ac` | `-` |
| `ldrb` at 0x9ffe8 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0x9ffec | store | 4 | `sp -0x2e4` | `0x8` |
| `ldr` at 0x9fff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fff8 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xa0008 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa0018 | store | 4 | `sp -0x2bc` | `ret(0x9dc3c)` |
| `ldr` at 0xa001c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xa0028 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xa002c | load | 4 | `abs 0xa03b0` | `-` |
| `ldr` at 0xa0034 | load | 4 | `abs 0xa03a4` | `-` |
| `str` at 0xa003c | store | 4 | `abs 0x12d8b4` | `?` |
| `str` at 0xa0044 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xa004c | load | 4 | `sp -0x130` | `-` |
| `str` at 0xa0054 | store | 4 | `sp -0x2d0` | `ret(0xa1808)` |
| `ldr` at 0xa005c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xa0074 | load | 4 | `abs 0xa03b4` | `-` |
| `ldrb` at 0xa0080 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xa008c | load | 4 | `abs 0xa03b4` | `-` |
| `ldrb` at 0xa0098 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xa00a4 | load | 4 | `abs 0xa03a8` | `-` |
| `ldr` at 0xa00a8 | load | 4 | `abs 0xa03b4` | `-` |
| `ldrb` at 0xa00b4 | load | 1 | `sp -0x2d4` | `-` |
| `strh` at 0xa00c0 | store | 2 | `sp -0x2c2` | `?` |
| `ldr` at 0xa00c4 | load | 4 | `abs 0xa03ac` | `-` |
| `strb` at 0xa00d0 | store | 1 | `sp -0x2c3` | `0x3` |
| `ldr` at 0xa00d8 | load | 4 | `abs 0xa03a8` | `-` |
| `ldrh` at 0xa00e0 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xa00e4 | load | 4 | `abs 0xa03ac` | `-` |
| `ldrb` at 0xa00ec | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xa00f0 | store | 4 | `sp -0x2e4` | `0x3` |
| `str` at 0xa00f8 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xa00fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa0104 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xa0114 | load | 4 | `abs 0xa03b8` | `-` |
| `strb` at 0xa011c | store | 1 | `sp -0x2c4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0xa0120 | load | 4 | `abs 0xa03b8` | `-` |
| `ldrb` at 0xa0128 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xa0134 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xa0138 | load | 4 | `abs 0xa03b0` | `-` |
| `ldr` at 0xa0140 | load | 4 | `abs 0xa03a4` | `-` |
| `str` at 0xa0148 | store | 4 | `abs 0x12d8b4` | `?` |
| `str` at 0xa0150 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xa0158 | load | 4 | `abs 0xa03ac` | `-` |
| `ldr` at 0xa015c | load | 4 | `abs 0xa03bc` | `-` |
| `ldrb` at 0xa0164 | load | 1 | `sp -0x117` | `-` |
| `strb` at 0xa0170 | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xa0174 | load | 4 | `abs 0xa03ac` | `-` |
| `ldr` at 0xa0178 | load | 4 | `abs 0xa03ac` | `-` |
| `ldr` at 0xa017c | load | 4 | `abs 0xa03b8` | `-` |
| `ldrb` at 0xa0184 | load | 1 | `sp -0x2c3` | `-` |
| `ldrb` at 0xa018c | load | 1 | `sp -0x2c4` | `-` |
| `strb` at 0xa0198 | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xa019c | load | 4 | `abs 0xa03a8` | `-` |
| `ldr` at 0xa01a0 | load | 4 | `abs 0xa03a8` | `-` |
| `ldrh` at 0xa01a8 | load | 2 | `sp -0x2c2` | `-` |
| `strh` at 0xa01b4 | store | 2 | `sp -0x2c2` | `([sp-706]:2+3 & 0xffff)` |
| `ldr` at 0xa01bc | load | 4 | `abs 0xa03a8` | `-` |
| `ldrh` at 0xa01c4 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xa01c8 | load | 4 | `abs 0xa03ac` | `-` |
| `ldrb` at 0xa01d0 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xa01d4 | store | 4 | `sp -0x2e4` | `[sp-707]:1` |
| `str` at 0xa01dc | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xa01e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa01e8 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xa01f8 | load | 4 | `abs 0xa03b8` | `-` |
| `strb` at 0xa0200 | store | 1 | `sp -0x2c4` | `(ret(0xa0bac) & 0xff)` |
| `ldr` at 0xa0204 | load | 4 | `abs 0xa03b8` | `-` |
| `ldrb` at 0xa020c | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xa0218 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xa021c | load | 4 | `abs 0xa03b0` | `-` |
| `ldr` at 0xa0224 | load | 4 | `abs 0xa03a4` | `-` |
| `str` at 0xa022c | store | 4 | `abs 0x12d8b4` | `?` |
| `str` at 0xa0234 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0xa0240 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xa0248 | store | 4 | `sp -0x2c0` | `0x0` |
| `ldr` at 0xa024c | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xa0258 | load | 4 | `abs 0xa03c0` | `-` |
| `ldr` at 0xa025c | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xa026c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa0280 | load | 4 | `abs 0xa03c0` | `-` |
| `ldr` at 0xa0284 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xa0294 | load | 1 | `(sp-16 + [sp-284]) -0x108` | `-` |
| `strb` at 0xa02a0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa02a4 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xa02ac | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldr` at 0xa02b0 | load | 4 | `sp -0x2c0` | `-` |
| `ldrb` at 0xa02c4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa02dc | store | 1 | `(unknown)` | `0x28` |
| `ldr` at 0xa02e4 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xa02f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa02f8 | load | 4 | `sp -0x2cc` | `-` |
| `ldrb` at 0xa0310 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa0318 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xa0324 | store | 1 | `(unknown)` | `0xa` |
| `ldrb` at 0xa0330 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa033c | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xa034c | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0xa035c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa0360 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xa0368 | store | 4 | `sp -0x11c` | `?` |
| `ldr` at 0xa036c | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xa0374 | store | 4 | `sp -0x2c0` | `[sp-704]+1` |
| `ldr` at 0xa037c | load | 4 | `abs 0xa03a4` | `-` |
| `str` at 0xa0384 | store | 4 | `abs 0x12d8b4` | `0x0` |
| `str` at 0xa038c | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xa0390 | load | 4 | `sp -0x2dc` | `-` |
| `ldm` at 0xa0398 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa0398 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa0398 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ff50 | `(unknown callee) 0x1943c` | `0x2a` | `r1` | `r2` | `r1` |
| `bl` at 0x9ff88 | `(unknown callee) 0x19808` | `[sp-716]` | `0x12d888` | `[sp-24]` | `[sp-716]` |
| `bl` at 0xa0000 | `(unknown callee) 0xa0a88` | `[sp-20]` | `sp-696` | `[sp-288]` | `0x0` |
| `bl` at 0xa0010 | `(unknown callee) 0x9dc3c` | `[sp-20]` | `sp-696` | `?` | `sp-696` |
| `bl` at 0xa0030 | `(unknown callee) 0x195e0` | `[sp-716]` | `0x112ee0` | `?` | `ret(0x9dc3c)` |
| `bl` at 0xa0050 | `(unknown callee) 0xa1808` | `[sp-304]` | `?` | `?` | `ret(0x9dc3c)` |
| `bl` at 0xa0070 | `(unknown callee) 0x19610` | `sp-728` | `ret(0xa1808)+1` | `0x8` | `ret(0xa1808)+1` |
| `bl` at 0xa010c | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-696` | `[sp-288]` | `[sp-706]:2` |
| `bl` at 0xa013c | `(unknown callee) 0x195e0` | `[sp-716]` | `0x112ee0` | `ret(0xa0bac)` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0xa01f0 | `(unknown callee) 0xa0bac` | `[sp-20]` | `sp-696` | `[sp-288]` | `([sp-706]:2+3 & 0xffff)` |
| `bl` at 0xa0220 | `(unknown callee) 0x195e0` | `[sp-716]` | `0x112ee0` | `sp-16` | `(ret(0xa0bac) & 0xff)` |
| `bl` at 0xa0304 | `(unknown callee) 0x19610` | `[sp-716]` | `(sp-280 + [sp-284])` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9ff24 | `[0x12d868]:1 != 0xff` | `[0x12d868]:1 == 0xff` |
| `beq` at 0x9ff48 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x9ff70 | `[sp-716] != 0x0` | `[sp-716] == 0x0` |
| `bne` at 0xa0024 | `ret(0x9dc3c) != 0x0` | `ret(0x9dc3c) == 0x0` |
| `beq` at 0xa0088 | `[sp-724]:1 == 0x0` | `[sp-724]:1 != 0x0` |
| `beq` at 0xa00a0 | `[sp-724]:1 == 0xff` | `[sp-724]:1 != 0xff` |
| `bne` at 0xa0130 | `(ret(0xa0bac) & 0xff) != 0x0` | `(ret(0xa0bac) & 0xff) == 0x0` |
| `bne` at 0xa0214 | `(ret(0xa0bac) & 0xff) != 0x0` | `(ret(0xa0bac) & 0xff) == 0x0` |
| `bgt` at 0xa0254 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xa0274 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa02b8 | `[sp-704] != 0x4` | `[sp-704] == 0x4` |
| `bls` at 0xa02cc | `? <= 0x28 (unsigned, 32-bit)` | `? > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa0398 | `r0` | `?` |

## `networkInit`

- `networkInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25e54 to 0x25eac.
  Established. A zero `ret(0x1d444)` takes `0x25e90`, while nonzero falls through (`beq` at 0x25e70). The unnamed `0x1b5f0` call carries `0x34`, `0x0`, unknown, unknown (`bl` at 0x25e84).
  The absolute word at `0x12d43c` receives zero (`str` at 0x25e9c). Network initialization policy
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x25e74 | load | 4 | `abs 0x25ea4` | `-` |
| `ldr` at 0x25e94 | load | 4 | `abs 0x25ea8` | `-` |
| `str` at 0x25e9c | store | 4 | `abs 0x12d43c` | `0x0` |
| `ldm` at 0x25ea0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x25ea0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x25ea0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25e64 | `(unknown callee) 0x1d444` | `0x34` | `r1` | `r2` | `r3` |
| `bl` at 0x25e78 | `(unknown callee) 0x1e958` | `0xee944` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x25e84 | `(unknown callee) 0x1b5f0` | `0x34` | `0x0` | `?` | `?` |
| `bl` at 0x25e88 | `(unknown callee) 0x25eac` | `ret(0x1b5f0)` | `?` | `?` | `?` |
| `bl` at 0x25e90 | `(unknown callee) 0x25eac` | `ret(0x1d444)` | `?` | `?` | `ret(0x1d444)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25e70 | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x25ea0 | `r0` | `ret(0x25eac)` |

## `brickSetType`

- `brickSetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x488ec to 0x48918.
  Established. The stack byte at `sp -0x11` receives `(r0 & 0xff)` (`strb` at 0x48900). The byte at
  `0x12d704` receives `((r0 & 0xff) & 0xff)` (`strb` at 0x4890c). The return record preserves entry
  `r0` (`ldm` at 0x48910). Board type encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x48900 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x48904 | load | 4 | `abs 0x48914` | `-` |
| `ldrb` at 0x48908 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4890c | store | 1 | `abs 0x12d704` | `((r0 & 0xff) & 0xff)` |
| `ldm` at 0x48910 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x48910 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48910 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48910 | `r0` | `r0` |

## `brickGetFlashType`

- `brickGetFlashType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48964 to 0x48984.
  Established. The absolute byte at `0x132419` is loaded (`ldrb` at 0x48974). The included integer
  return record carries `[0x132419]:1` in `r0` (`ldm` at 0x4897c). Flash type interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48970 | load | 4 | `abs 0x48980` | `-` |
| `ldrb` at 0x48974 | load | 1 | `abs 0x132419` | `-` |
| `ldm` at 0x4897c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4897c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4897c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4897c | `r0` | `[0x132419]:1` |

## `brickGetSubType`

- `brickGetSubType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48918 to 0x48938.
  Established. The absolute byte at `0x132418` is read (`ldrb` at 0x48928). The return record
  reports `[0x132418]:1` in `r0` (`ldm` at 0x48930). Subtype meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48924 | load | 4 | `abs 0x48934` | `-` |
| `ldrb` at 0x48928 | load | 1 | `abs 0x132418` | `-` |
| `ldm` at 0x48930 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48930 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48930 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48930 | `r0` | `[0x132418]:1` |

## `brickGetType`

- `brickGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x488cc to 0x488ec.
  Established. The absolute byte at `0x12d704` is read (`ldrb` at 0x488dc); its byte-valued
  expression `[0x12d704]:1` reaches the included `r0` return record (`ldm` at 0x488e4). Board type
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x488d8 | load | 4 | `abs 0x488e8` | `-` |
| `ldrb` at 0x488dc | load | 1 | `abs 0x12d704` | `-` |
| `ldm` at 0x488e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x488e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x488e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x488e4 | `r0` | `[0x12d704]:1` |

## `brickSetSubType`

- `brickSetSubType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48938 to 0x48964.
  Established. The stack byte at `sp -0x11` receives `(r0 & 0xff)` (`strb` at 0x4894c). A byte store
  writes `((r0 & 0xff) & 0xff)` to `0x132418` (`strb` at 0x48958). The included integer return
  record retains entry `r0` (`ldm` at 0x4895c). Subtype assignment policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4894c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x48950 | load | 4 | `abs 0x48960` | `-` |
| `ldrb` at 0x48954 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x48958 | store | 1 | `abs 0x132418` | `((r0 & 0xff) & 0xff)` |
| `ldm` at 0x4895c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4895c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4895c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4895c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4895c | `r0` | `r0` |

## `brickSlotLabel`

- `brickSlotLabel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48da8 to 0x48de8.
  Established. Entry candidates `r0`, `r1`, `r2`, `r3` are saved as words at `sp -0x14`, `sp -0x18`,
  `sp -0x1c`, `sp -0x20` (`str` at 0x48db8, `str` at 0x48dbc, `str` at 0x48dc0, `str` at 0x48dc4).
  The absolute byte at `0x132420` receives zero (`strb` at 0x48dd0). The included return record
  carries `0x132420` in `r0` (`ldm` at 0x48de0). Slot label presentation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48db8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x48dbc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x48dc0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x48dc4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x48dc8 | load | 4 | `abs 0x48de4` | `-` |
| `strb` at 0x48dd0 | store | 1 | `abs 0x132420` | `0x0` |
| `ldr` at 0x48dd4 | load | 4 | `abs 0x48de4` | `-` |
| `ldm` at 0x48de0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48de0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48de0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48de0 | `r0` | `0x132420` |
