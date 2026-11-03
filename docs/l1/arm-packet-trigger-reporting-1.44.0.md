# ARM packet generators and trigger reporting in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/packet-trigger-reporting-facts.txt) retain all tool sections.

## `PktGenOp`

- `PktGenOp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58b64 to 0x5a31c.
  Established. Included stores write halfword `0x123` to `0x1325a0` and byte `0x0` to `0x132588` (`strh` at 0x5933c, `strb` at 0x593d4).
  An indirect call with unknown target carries `[sp-28]`, `0x90010000`, `sp-72`, `sp-76` (`mov` at 0x5a174).
  A zero `[0x132580]` takes `0x58bec`, while nonzero falls through (`beq` at 0x58bc0).
  Packet generator register ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58b74 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x58b78 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x58b7c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x58b80 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x58b88 | store | 4 | `sp -0x70` | `0x0` |
| `str` at 0x58b90 | store | 4 | `sp -0x78` | `0x0` |
| `str` at 0x58b98 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x58ba0 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x58ba4 | load | 4 | `abs 0x59abc` | `-` |
| `str` at 0x58ba8 | store | 4 | `sp -0x2c` | `0xde16c` |
| `ldr` at 0x58bac | load | 4 | `abs 0x59ac0` | `-` |
| `str` at 0x58bb0 | store | 4 | `sp -0x28` | `0xde274` |
| `ldr` at 0x58bb4 | load | 4 | `abs 0x59ac4` | `-` |
| `ldr` at 0x58bb8 | load | 4 | `abs 0x132580` | `-` |
| `ldr` at 0x58bc4 | load | 4 | `abs 0x59ac8` | `-` |
| `ldr` at 0x58bc8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58bcc | store | 4 | `abs 0x299854` | `0xde16c` |
| `ldr` at 0x58bd0 | load | 4 | `abs 0x59acc` | `-` |
| `ldr` at 0x58bd4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58bd8 | store | 4 | `abs 0x299850` | `[sp-40]` |
| `ldr` at 0x58bdc | load | 4 | `abs 0x59ad0` | `-` |
| `str` at 0x58be0 | store | 4 | `sp -0x2c` | `0x57a08` |
| `ldr` at 0x58be4 | load | 4 | `abs 0x59ad4` | `-` |
| `str` at 0x58be8 | store | 4 | `sp -0x28` | `0x57a8c` |
| `ldr` at 0x58bec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58bf8 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x58bfc | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x58c00 | store | 4 | `sp -0x9c` | `[sp-188]` |
| `ldr` at 0x58c04 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x58c10 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x58c1c | load | 4 | `abs 0x59ad8` | `-` |
| `ldr` at 0x58c24 | load | 4 | `abs 0x59adc` | `-` |
| `str` at 0x58c28 | store | 4 | `sp -0xb8` | `0xfffff377` |
| `ldr` at 0x58c30 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x58c34 | store | 4 | `sp -0xc0` | `[sp-188]` |
| `ldr` at 0x58c38 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x58c44 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x58c54 | load | 4 | `abs 0x59ae0` | `-` |
| `str` at 0x58c58 | store | 4 | `sp -0x70` | `0xf9464` |
| `str` at 0x58c60 | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x58c64 | load | 4 | `abs 0x59ae4` | `-` |
| `str` at 0x58c68 | store | 4 | `sp -0x78` | `0xf94bc` |
| `str` at 0x58c70 | store | 4 | `sp -0x7c` | `0xb` |
| `ldr` at 0x58c74 | load | 4 | `abs 0x59ae8` | `-` |
| `str` at 0x58c78 | store | 4 | `sp -0x80` | `0xf94e8` |
| `str` at 0x58c80 | store | 4 | `sp -0x84` | `0x6` |
| `str` at 0x58c88 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x58c90 | load | 4 | `abs 0x59aec` | `-` |
| `str` at 0x58c94 | store | 4 | `sp -0x70` | `0xf93c8` |
| `str` at 0x58c9c | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x58ca0 | load | 4 | `abs 0x59af0` | `-` |
| `str` at 0x58ca4 | store | 4 | `sp -0x78` | `0xf9420` |
| `str` at 0x58cac | store | 4 | `sp -0x7c` | `0xb` |
| `str` at 0x58cb4 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x58cbc | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x58cc4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x58cc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58cd4 | store | 4 | `sp -0x98` | `ret(0xdcd3c)` |
| `ldr` at 0x58cd8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58ce4 | load | 4 | `abs 0x59af4` | `-` |
| `ldr` at 0x58ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58cf0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58cf4 | load | 4 | `abs 0x59af8` | `-` |
| `ldr` at 0x58d00 | load | 4 | `abs 0x59afc` | `-` |
| `str` at 0x58d04 | store | 4 | `sp -0xc4` | `0xfffff375` |
| `ldr` at 0x58d0c | load | 4 | `abs 0x59b00` | `-` |
| `str` at 0x58d10 | store | 4 | `sp -0xc4` | `0xfffff374` |
| `ldr` at 0x58d14 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x58d18 | store | 4 | `sp -0xb8` | `[sp-196]` |
| `ldr` at 0x58d20 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x58d2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58d30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58d34 | load | 4 | `[sp-112] +0x0` | `-` |
| `ldr` at 0x58d50 | load | 4 | `abs 0x59b04` | `-` |
| `str` at 0x58d5c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x58d64 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x58d70 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x58d78 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x58d80 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58d90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58d94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58d98 | load | 4 | `[sp-120] +0x20` | `-` |
| `str` at 0x58db8 | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x58dc4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x58dcc | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x58dd4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58ddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58de0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58de4 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x58de8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58dec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58e04 | load | 4 | `abs 0x59b08` | `-` |
| `ldr` at 0x58e0c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58e10 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58e1c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58e24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58e28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58e2c | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x58e30 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58e34 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58e4c | load | 4 | `abs 0x59b0c` | `-` |
| `ldr` at 0x58e54 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58e58 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58e64 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58e6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58e70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58e74 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x58e78 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58e7c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58e94 | load | 4 | `abs 0x59b10` | `-` |
| `ldr` at 0x58e9c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58ea0 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58eac | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58eb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58eb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58ebc | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x58ec0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58ec4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58edc | load | 4 | `abs 0x59b14` | `-` |
| `ldr` at 0x58ee4 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58ee8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58ef4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58efc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58f00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58f04 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x58f08 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58f0c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58f24 | load | 4 | `abs 0x59b18` | `-` |
| `ldr` at 0x58f2c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58f30 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58f38 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58f3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58f40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58f44 | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0x58f48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58f4c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58f64 | load | 4 | `abs 0x59b1c` | `-` |
| `ldr` at 0x58f6c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58f70 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58f7c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58f84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58f88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58f8c | load | 4 | `[sp-120] +0x1c` | `-` |
| `ldr` at 0x58f90 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58f94 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58fac | load | 4 | `abs 0x59b20` | `-` |
| `ldr` at 0x58fb4 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x58fb8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x58fc4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x58fcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58fd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58fd4 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x58fd8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x58fdc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x58ff4 | load | 4 | `abs 0x59b24` | `-` |
| `ldr` at 0x58ffc | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59000 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59008 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x59014 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x59018 | store | 4 | `sp -0x48` | `[sp-56]` |
| `ldr` at 0x5901c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x59024 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x5902c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x5903c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x59040 | load | 4 | `abs 0x59b28` | `-` |
| `str` at 0x59044 | store | 4 | `sp -0x4c` | `0x7ffffd` |
| `ldr` at 0x5904c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59054 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5905c | load | 4 | `[sp-120] +0x20` | `-` |
| `ldr` at 0x59060 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59064 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x5907c | load | 4 | `abs 0x59b2c` | `-` |
| `ldr` at 0x59084 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59088 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `str` at 0x59094 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x59098 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x5909c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x590a8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x590b0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x590c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x590c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x590c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x590e4 | load | 4 | `abs 0x59b30` | `-` |
| `str` at 0x590f0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x590f8 | load | 4 | `abs 0x59be4` | `-` |
| `ldr` at 0x590fc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59100 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x59104 | store | 4 | `(unknown)` | `[sp-72]` |
| `ldr` at 0x59108 | load | 4 | `abs 0x59be4` | `-` |
| `ldr` at 0x5910c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59120 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x59124 | store | 4 | `(unknown)` | `[sp-76]` |
| `ldr` at 0x59128 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x5912c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x59134 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x59138 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59140 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x59148 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5914c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59150 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59154 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59158 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59170 | load | 4 | `abs 0x59b34` | `-` |
| `ldr` at 0x59178 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5917c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59184 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x5918c | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `str` at 0x59198 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x5919c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x591a0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x591b8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x591c0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x591c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x591cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x591dc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x591e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x591ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x591f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x591f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59200 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59210 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5921c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59220 | store | 4 | `sp -0x20` | `[sp-48]` |
| `ldr` at 0x59224 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59230 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x59234 | store | 4 | `sp -0xc8` | `[sp-156]` |
| `ldr` at 0x59238 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x59244 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x59260 | load | 4 | `(unknown)` | `-` |
| `str` at 0x59274 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x59280 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0x59288 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x59294 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x592b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x592b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x592c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x592c4 | load | 4 | `abs 0x59ac4` | `-` |
| `ldr` at 0x592c8 | load | 4 | `abs 0x132580` | `-` |
| `ldr` at 0x592d4 | load | 4 | `abs 0x59b38` | `-` |
| `ldr` at 0x592d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x592e0 | load | 4 | `abs 0x59b3c` | `-` |
| `ldr` at 0x592e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x592ec | load | 4 | `abs 0x59b40` | `-` |
| `ldr` at 0x592f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x592f8 | load | 4 | `abs 0x59b44` | `-` |
| `ldr` at 0x592fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59310 | load | 4 | `abs 0x59b48` | `-` |
| `ldr` at 0x59314 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59328 | load | 4 | `abs 0x59b4c` | `-` |
| `ldr` at 0x5932c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59334 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59338 | load | 4 | `abs 0x59b54` | `-` |
| `strh` at 0x5933c | store | 2 | `abs 0x1325a0` | `0x123` |
| `ldr` at 0x59340 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59344 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x59348 | store | 1 | `abs 0x132598` | `([sp-52] & 0xff)` |
| `ldr` at 0x5934c | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59350 | load | 4 | `sp -0x30` | `-` |
| `strh` at 0x59354 | store | 2 | `abs 0x132596` | `([sp-48] & 0xffff)` |
| `ldr` at 0x59358 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x5935c | load | 4 | `sp +0x0` | `-` |
| `strh` at 0x59364 | store | 2 | `abs 0x132594` | `?` |
| `ldr` at 0x59368 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x5936c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x59370 | store | 1 | `abs 0x13258f` | `([sp-36] & 0xff)` |
| `ldr` at 0x59374 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59378 | load | 4 | `sp -0x20` | `-` |
| `strh` at 0x5937c | store | 2 | `abs 0x13258c` | `([sp-32] & 0xffff)` |
| `ldr` at 0x59380 | load | 4 | `abs 0x59b50` | `-` |
| `str` at 0x59384 | store | 4 | `sp -0xcc` | `0x132588` |
| `ldr` at 0x59388 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x59398 | store | 1 | `sp -0xcd` | `0x11` |
| `strb` at 0x593a4 | store | 1 | `sp -0xcd` | `0x31` |
| `ldrb` at 0x593a8 | load | 1 | `sp -0xcd` | `-` |
| `ldr` at 0x593ac | load | 4 | `sp -0xcc` | `-` |
| `strb` at 0x593b0 | store | 1 | `abs 0x13258e` | `([sp-205]:1 & 0xff)` |
| `ldr` at 0x593b4 | load | 4 | `abs 0x59b50` | `-` |
| `strb` at 0x593bc | store | 1 | `abs 0x13258a` | `0x0` |
| `ldr` at 0x593c0 | load | 4 | `abs 0x59b50` | `-` |
| `strb` at 0x593c8 | store | 1 | `abs 0x132589` | `0x0` |
| `ldr` at 0x593cc | load | 4 | `abs 0x59b50` | `-` |
| `strb` at 0x593d4 | store | 1 | `abs 0x132588` | `0x0` |
| `ldr` at 0x593d8 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x593e8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x593f4 | store | 1 | `abs 0x132599` | `?` |
| `ldr` at 0x593f8 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59408 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59420 | load | 4 | `(unknown)` | `-` |
| `str` at 0x59430 | store | 4 | `abs 0x132590` | `?` |
| `ldr` at 0x59434 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59444 | load | 4 | `(unknown)` | `-` |
| `str` at 0x59454 | store | 4 | `abs 0x13259c` | `?` |
| `ldr` at 0x59458 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x5945c | load | 4 | `abs 0x132590` | `-` |
| `ldr` at 0x59468 | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x5946c | load | 1 | `abs 0x13258f` | `-` |
| `str` at 0x59478 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x5947c | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x59480 | load | 1 | `abs 0x13258e` | `-` |
| `ldr` at 0x59488 | load | 4 | `abs 0x59b50` | `-` |
| `ldrh` at 0x5948c | load | 2 | `abs 0x13258c` | `-` |
| `ldr` at 0x594a0 | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x594a4 | load | 1 | `abs 0x13258a` | `-` |
| `ldr` at 0x594b4 | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x594b8 | load | 1 | `abs 0x132589` | `-` |
| `ldr` at 0x594c4 | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x594c8 | load | 1 | `abs 0x132588` | `-` |
| `str` at 0x594d4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x594dc | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x594e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x594e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x594ec | load | 4 | `[sp-112] +0x8` | `-` |
| `ldr` at 0x594f0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x594f4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x5950c | load | 4 | `abs 0x59b58` | `-` |
| `ldr` at 0x59514 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59518 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59520 | load | 4 | `abs 0x59b50` | `-` |
| `ldrh` at 0x59524 | load | 2 | `abs 0x1325a0` | `-` |
| `ldr` at 0x59534 | load | 4 | `abs 0x59b50` | `-` |
| `ldr` at 0x59538 | load | 4 | `abs 0x13259c` | `-` |
| `ldr` at 0x5954c | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x59550 | load | 1 | `abs 0x132599` | `-` |
| `str` at 0x59560 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x59564 | load | 4 | `abs 0x59b50` | `-` |
| `ldrb` at 0x59568 | load | 1 | `abs 0x132598` | `-` |
| `ldr` at 0x59574 | load | 4 | `abs 0x59b50` | `-` |
| `ldrh` at 0x59578 | load | 2 | `abs 0x132596` | `-` |
| `ldr` at 0x5958c | load | 4 | `abs 0x59b50` | `-` |
| `ldrh` at 0x59590 | load | 2 | `abs 0x132594` | `-` |
| `str` at 0x595a0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x595a8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x595b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x595b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x595b8 | load | 4 | `[sp-112] +0xc` | `-` |
| `ldr` at 0x595bc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x595c0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x595d8 | load | 4 | `abs 0x59b5c` | `-` |
| `ldr` at 0x595e0 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x595e4 | store | 4 | `sp -0x98` | `0xfffff380` |
| `str` at 0x595f0 | store | 4 | `sp -0x48` | `0x40000000` |
| `str` at 0x595f8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x595fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59608 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59610 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x59618 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5962c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x59634 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59638 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5963c | load | 4 | `[sp-112] +0x10` | `-` |
| `ldr` at 0x5965c | load | 4 | `abs 0x59b60` | `-` |
| `ldr` at 0x59664 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59668 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59674 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x5967c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59684 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x5968c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59694 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x5969c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x596a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x596a4 | load | 4 | `[sp-112] +0x14` | `-` |
| `ldr` at 0x596a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x596ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x596c4 | load | 4 | `abs 0x59b64` | `-` |
| `ldr` at 0x596cc | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x596d0 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x596dc | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x596e4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x596ec | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x596f4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x596fc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x59704 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5970c | load | 4 | `[sp-112] +0x10` | `-` |
| `ldr` at 0x59710 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5972c | load | 4 | `abs 0x59b60` | `-` |
| `ldr` at 0x59734 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59738 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59744 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x5974c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59750 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59754 | load | 4 | `[sp-112] +0x14` | `-` |
| `ldr` at 0x59774 | load | 4 | `abs 0x59b64` | `-` |
| `ldr` at 0x5977c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59780 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59788 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x59790 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x59798 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x597a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x597a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x597a8 | load | 4 | `[sp-112] +0x4` | `-` |
| `ldr` at 0x597ac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x597b0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x597c8 | load | 4 | `abs 0x59b68` | `-` |
| `ldr` at 0x597d0 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x597d4 | store | 4 | `sp -0x98` | `0xfffff380` |
| `str` at 0x597e0 | store | 4 | `sp -0x90` | `0x14` |
| `str` at 0x597e8 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x597ec | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x597f8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x59800 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x59810 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x59820 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59828 | load | 4 | `[sp-112] +0x4` | `-` |
| `ldr` at 0x59844 | load | 4 | `abs 0x59b6c` | `-` |
| `str` at 0x59850 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x59858 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x59864 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x59868 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x59880 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5988c | load | 4 | `abs 0x59b70` | `-` |
| `ldr` at 0x59894 | load | 4 | `abs 0x59b74` | `-` |
| `str` at 0x59898 | store | 4 | `sp -0x98` | `0xfffff37e` |
| `str` at 0x598a4 | store | 4 | `sp -0x90` | `0x14` |
| `str` at 0x598ac | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x598b0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x598bc | load | 4 | `sp -0x90` | `-` |
| `str` at 0x598c4 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x598d4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x598e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x598e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x598ec | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x59908 | load | 4 | `abs 0x59bc4` | `-` |
| `str` at 0x59914 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x5991c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x59930 | store | 4 | `sp -0xd4` | `0x0` |
| `str` at 0x5993c | store | 4 | `sp -0xd4` | `0x1` |
| `ldr` at 0x59940 | load | 4 | `sp -0xd4` | `-` |
| `str` at 0x59944 | store | 4 | `sp -0x8c` | `[sp-212]` |
| `ldr` at 0x59948 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x59960 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5996c | load | 4 | `abs 0x59b78` | `-` |
| `ldr` at 0x59974 | load | 4 | `abs 0x59b7c` | `-` |
| `str` at 0x59978 | store | 4 | `sp -0x98` | `0xfffff37d` |
| `ldr` at 0x59980 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59990 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x599a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x599a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x599a8 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x599c4 | load | 4 | `abs 0x59b80` | `-` |
| `str` at 0x599d0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x599dc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x599ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x599f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x599f4 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x59a10 | load | 4 | `abs 0x59b80` | `-` |
| `str` at 0x59a1c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x59a28 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59a38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59a3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59a40 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x59a5c | load | 4 | `abs 0x59b80` | `-` |
| `str` at 0x59a68 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x59a74 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59a84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59a8c | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x59aa8 | load | 4 | `abs 0x59b80` | `-` |
| `str` at 0x59ab4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x59bf0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x59c00 | load | 4 | `abs 0x59b84` | `-` |
| `ldr` at 0x59c08 | load | 4 | `abs 0x59b88` | `-` |
| `str` at 0x59c0c | store | 4 | `sp -0x98` | `0xfffff37c` |
| `ldr` at 0x59c14 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x59c24 | load | 4 | `abs 0x59b8c` | `-` |
| `ldr` at 0x59c2c | load | 4 | `abs 0x59b90` | `-` |
| `str` at 0x59c30 | store | 4 | `sp -0x98` | `0xfffff37b` |
| `ldr` at 0x59c38 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x59c48 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x59c50 | load | 4 | `abs 0x59b94` | `-` |
| `ldr` at 0x59c5c | load | 4 | `abs 0x59b98` | `-` |
| `str` at 0x59c60 | store | 4 | `sp -0x98` | `0xfffff37a` |
| `ldr` at 0x59c68 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x59c74 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x59c78 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x59c80 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x59c84 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x59c8c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x59c94 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x59c98 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x59ca0 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x59cb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59cb4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x59ccc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59cd0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x59cf4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x59cf8 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x59cfc | load | 4 | `abs 0x59b9c` | `-` |
| `ldr` at 0x59d00 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59d04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59d08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59d10 | load | 4 | `abs 0x59ba0` | `-` |
| `str` at 0x59d14 | store | 4 | `sp -0x98` | `0xfffff379` |
| `ldr` at 0x59d28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59d38 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59d40 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x59d48 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x59d4c | store | 4 | `(unknown)` | `[sp-96]` |
| `ldr` at 0x59d50 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59d58 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x59d60 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x59d64 | store | 4 | `(unknown)` | `[sp-100]` |
| `ldr` at 0x59d6c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59d74 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x59d7c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x59d80 | store | 4 | `(unknown)` | `[sp-104]` |
| `ldr` at 0x59d84 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59d8c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x59d94 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x59d98 | store | 4 | `(unknown)` | `[sp-108]` |
| `str` at 0x59da0 | store | 4 | `sp -0x90` | `0x1` |
| `str` at 0x59da8 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x59db0 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x59db8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x59dc0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59dd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59dd8 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x59df4 | load | 4 | `abs 0x59bc4` | `-` |
| `str` at 0x59e00 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x59e0c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59e14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59e18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59e1c | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x59e20 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59e24 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59e3c | load | 4 | `abs 0x59ba4` | `-` |
| `ldr` at 0x59e44 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59e48 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59e54 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59e5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59e60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59e64 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x59e68 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59e6c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59e84 | load | 4 | `abs 0x59ba8` | `-` |
| `ldr` at 0x59e8c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59e90 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59e9c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59ea4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59ea8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59eac | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x59eb0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59eb4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59ecc | load | 4 | `abs 0x59bac` | `-` |
| `ldr` at 0x59ed4 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59ed8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59ee4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59eec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59ef4 | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x59ef8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59efc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59f14 | load | 4 | `abs 0x59bb0` | `-` |
| `ldr` at 0x59f1c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59f20 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59f2c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59f34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59f38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59f3c | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x59f40 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59f44 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59f5c | load | 4 | `abs 0x59bb4` | `-` |
| `ldr` at 0x59f64 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59f68 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59f70 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59f74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59f78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59f7c | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0x59f80 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x59f84 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x59f9c | load | 4 | `abs 0x59bb8` | `-` |
| `ldr` at 0x59fa4 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x59fa8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x59fb4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x59fc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59fc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59fcc | load | 4 | `[sp-120] +0x1c` | `-` |
| `ldr` at 0x59fe8 | load | 4 | `abs 0x59bc4` | `-` |
| `str` at 0x59ff4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x5a000 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5a008 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5a00c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a010 | load | 4 | `[sp-120] +0x28` | `-` |
| `ldr` at 0x5a014 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x5a018 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x5a030 | load | 4 | `abs 0x59bbc` | `-` |
| `ldr` at 0x5a038 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a03c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x5a048 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5a050 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5a054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a058 | load | 4 | `[sp-120] +0x24` | `-` |
| `ldr` at 0x5a05c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x5a060 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x5a078 | load | 4 | `abs 0x59bc0` | `-` |
| `ldr` at 0x5a080 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a084 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x5a090 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5a0a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5a0a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a0a8 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x5a0c4 | load | 4 | `abs 0x59bc4` | `-` |
| `str` at 0x5a0d0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x5a0d8 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x5a0f0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x5a0f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5a100 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x5a108 | store | 4 | `sp -0x90` | `[sp-144]-1` |
| `ldr` at 0x5a118 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5a124 | load | 4 | `abs 0x59bc8` | `-` |
| `ldr` at 0x5a12c | load | 4 | `abs 0x59bcc` | `-` |
| `str` at 0x5a130 | store | 4 | `sp -0x98` | `0xfffff378` |
| `ldr` at 0x5a138 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x5a140 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `ldr` at 0x5a148 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x5a15c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5a160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a164 | load | 4 | `abs 0x59bd4` | `-` |
| `ldr` at 0x5a184 | load | 4 | `abs 0x59bd0` | `-` |
| `ldr` at 0x5a18c | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a190 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x5a198 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x5a1a0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x5a1a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5a1a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a1ac | load | 4 | `abs 0x59bd4` | `-` |
| `ldr` at 0x5a1b0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x5a1b4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x5a1cc | load | 4 | `abs 0x59bdc` | `-` |
| `ldr` at 0x5a1d4 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a1d8 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x5a1e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5a1e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a1e8 | load | 4 | `abs 0x59bd8` | `-` |
| `ldr` at 0x5a208 | load | 4 | `abs 0x59bdc` | `-` |
| `ldr` at 0x5a210 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a214 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x5a220 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5a228 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5a22c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a230 | load | 4 | `[sp-120] +0x20` | `-` |
| `ldr` at 0x5a234 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x5a238 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x5a250 | load | 4 | `abs 0x59be0` | `-` |
| `ldr` at 0x5a258 | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a25c | store | 4 | `sp -0xb8` | `0xfffff380` |
| `str` at 0x5a268 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x5a26c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x5a270 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x5a27c | load | 4 | `abs 0x59be4` | `-` |
| `ldr` at 0x5a280 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x5a284 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5a288 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x5a28c | load | 4 | `abs 0x59be4` | `-` |
| `ldr` at 0x5a290 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x5a2a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5a2a8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x5a2ac | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x5a2b4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x5a2bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5a2c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a2c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a2c8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x5a2cc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x5a2e4 | load | 4 | `abs 0x59be8` | `-` |
| `ldr` at 0x5a2ec | load | 4 | `abs 0x59bec` | `-` |
| `str` at 0x5a2f0 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x5a2f8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x5a300 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `ldr` at 0x5a308 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x5a30c | store | 4 | `sp -0xb8` | `[sp-152]` |
| `ldr` at 0x5a310 | load | 4 | `sp -0xb8` | `-` |
| `ldm` at 0x5a318 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5a318 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5a318 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5a318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58c20 | `(unknown callee) 0x1e958` | `0xf9520` | `[sp-188]` | `[sp-188]` | `?` |
| `bl` at 0x58ccc | `(unknown callee) 0xdcd3c` | `?` | `[sp-188]` | `?` | `?` |
| `bl` at 0x58cec | `(unknown callee) 0x1e958` | `0xf9554` | `[sp-28]` | `?` | `ret(0xdcd3c)` |
| `mov` at 0x58d40 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]]` | `sp-64` | `sp-68` |
| `bl` at 0x58d54 | `(unknown callee) 0x1e958` | `0xf9574` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58da4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `sp-56` | `sp-60` |
| `mov` at 0x58df4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `0x0` | `0x0` |
| `bl` at 0x58e08 | `(unknown callee) 0x1e958` | `0xf9594` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58e3c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58e50 | `(unknown callee) 0x1e958` | `0xf9870` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58e84 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58e98 | `(unknown callee) 0x1e958` | `0xf9890` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58ecc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58ee0 | `(unknown callee) 0x1e958` | `0xf98b0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58f14 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+20]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58f28 | `(unknown callee) 0x1e958` | `0xf98d0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58f54 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58f68 | `(unknown callee) 0x1e958` | `0xf95b4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58f9c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+28]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58fb0 | `(unknown callee) 0x1e958` | `0xf95d4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58fe4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x58ff8 | `(unknown callee) 0x1e958` | `0xf95f4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5906c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59080 | `(unknown callee) 0x1e958` | `0xf9610` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x590d4 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `sp-72` | `sp-76` |
| `bl` at 0x590e8 | `(unknown callee) 0x1e958` | `0xf9630` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59160 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59174 | `(unknown callee) 0x1e958` | `0xf9658` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x592dc | `(unknown callee) 0x1e958` | `0xf98f0` | `[sp-48]` | `?` | `[0x132580]` |
| `bl` at 0x592e8 | `(unknown callee) 0x1e958` | `0xf98fc` | `[sp-52]` | `?` | `?` |
| `bl` at 0x592f4 | `(unknown callee) 0x1e958` | `0xf9908` | `[sp-32]` | `?` | `?` |
| `bl` at 0x59300 | `(unknown callee) 0x1e958` | `0xf9914` | `[sp-36]` | `?` | `?` |
| `bl` at 0x59318 | `(unknown callee) 0x1e958` | `0xf9920` | `?` | `?` | `?` |
| `bl` at 0x59330 | `(unknown callee) 0x1e958` | `0xf992c` | `?` | `?` | `?` |
| `mov` at 0x594fc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59510 | `(unknown callee) 0x1e958` | `0xf9680` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x595c8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x595dc | `(unknown callee) 0x1e958` | `0xf96a0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5964c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+16]` | `0x0` | `0x0` |
| `bl` at 0x59660 | `(unknown callee) 0x1e958` | `0xf9938` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x596b4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+20]` | `?` | `?` |
| `bl` at 0x596c8 | `(unknown callee) 0x1e958` | `0xf995c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5971c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+16]` | `?` | `?` |
| `bl` at 0x59730 | `(unknown callee) 0x1e958` | `0xf9938` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59764 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+20]` | `0x0` | `0x0` |
| `bl` at 0x59778 | `(unknown callee) 0x1e958` | `0xf995c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x597b8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+4]` | `?` | `?` |
| `bl` at 0x597cc | `(unknown callee) 0x1e958` | `0xf96e0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59834 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+4]` | `sp-72` | `sp-76` |
| `bl` at 0x59848 | `(unknown callee) 0x1e958` | `0xf9704` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x59878 | `(unknown callee) 0x5a31c` | `0xa` | `?` | `?` | `[sp-140]` |
| `bl` at 0x59890 | `(unknown callee) 0x1e958` | `0xf971c` | `?` | `?` | `?` |
| `mov` at 0x598f8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-72` | `sp-76` |
| `bl` at 0x5990c | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x59958 | `(unknown callee) 0x5a31c` | `0xa` | `?` | `?` | `[sp-212]` |
| `bl` at 0x59970 | `(unknown callee) 0x1e958` | `0xf9748` | `?` | `?` | `?` |
| `mov` at 0x599b4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `sp-80` | `sp-84` |
| `bl` at 0x599c8 | `(unknown callee) 0x1e4ac` | `0xf975c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59a00 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `sp-88` | `sp-92` |
| `bl` at 0x59a14 | `(unknown callee) 0x1e4ac` | `0xf975c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59a4c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `sp-96` | `sp-100` |
| `bl` at 0x59a60 | `(unknown callee) 0x1e4ac` | `0xf975c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59a98 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `sp-104` | `sp-108` |
| `bl` at 0x59aac | `(unknown callee) 0x1e4ac` | `0xf975c` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x59c04 | `(unknown callee) 0x1e4ac` | `0xf9770` | `?` | `?` | `?` |
| `bl` at 0x59c28 | `(unknown callee) 0x1e4ac` | `0xf9780` | `?` | `?` | `?` |
| `bl` at 0x59c58 | `(unknown callee) 0x1e958` | `0xf9794` | `?` | `?` | `?` |
| `bl` at 0x59d0c | `(unknown callee) 0x1e958` | `0xf9980` | `[sp-72]` | `[sp-76]` | `?` |
| `mov` at 0x59de4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-168` | `sp-172` |
| `bl` at 0x59df8 | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59e2c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59e40 | `(unknown callee) 0x1e958` | `0xf97ac` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59e74 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59e88 | `(unknown callee) 0x1e958` | `0xf99a4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59ebc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59ed0 | `(unknown callee) 0x1e958` | `0xf99b8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59f04 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59f18 | `(unknown callee) 0x1e958` | `0xf99cc` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59f4c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+20]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59f60 | `(unknown callee) 0x1e958` | `0xf99e0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59f8c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x59fa0 | `(unknown callee) 0x1e958` | `0xf97c0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x59fd8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+28]` | `sp-176` | `sp-180` |
| `bl` at 0x59fec | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a020 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+40]` | `[sp-176]` | `[sp-180]` |
| `bl` at 0x5a034 | `(unknown callee) 0x1e958` | `0xf97d8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a068 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+36]` | `[sp-168]` | `[sp-172]` |
| `bl` at 0x5a07c | `(unknown callee) 0x1e958` | `0xf97f4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a0b4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-168` | `sp-172` |
| `bl` at 0x5a0c8 | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x5a128 | `(unknown callee) 0x1e958` | `0xf980c` | `?` | `?` | `[sp-140]` |
| `mov` at 0x5a174 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90010000` | `sp-72` | `sp-76` |
| `bl` at 0x5a188 | `(unknown callee) 0x1e958` | `0xf99f4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a1bc | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90010000` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x5a1d0 | `(unknown callee) 0x1e958` | `0xf9a0c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a1f8 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90071100` | `0x0` | `0x0` |
| `bl` at 0x5a20c | `(unknown callee) 0x1e958` | `0xf9a0c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a240 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x5a254 | `(unknown callee) 0x1e958` | `0xf9820` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5a2d4 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5a2e8 | `(unknown callee) 0x1e958` | `0xf9844` | `?` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x58bc0 | `[0x132580] == 0x0` | `[0x132580] != 0x0` |
| `beq` at 0x58c0c | `[sp-188] == 0x2` | `[sp-188] != 0x2` |
| `beq` at 0x58c18 | `[sp-188] == 0x4` | `[sp-188] != 0x4` |
| `beq` at 0x58c40 | `[sp-188] == 0x2` | `[sp-188] != 0x2` |
| `beq` at 0x58c4c | `[sp-188] == 0x4` | `[sp-188] != 0x4` |
| `beq` at 0x58ce0 | `ret(0xdcd3c) == 0x0` | `ret(0xdcd3c) != 0x0` |
| `bne` at 0x58cfc | `[sp-152] != 0xfffff7c6` | `[sp-152] == 0xfffff7c6` |
| `beq` at 0x58d4c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58db0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58e00 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58e48 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58e90 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58ed8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58f20 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58f60 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58fa8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58ff0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x59010 | `[sp-156] != 0x2` | `[sp-156] == 0x2` |
| `bne` at 0x59034 | `[sp-156] != 0x4` | `[sp-156] == 0x4` |
| `beq` at 0x59078 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bhs` at 0x590a4 | `C == 1` | `not (C == 1)` |
| `beq` at 0x590e0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5916c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bge` at 0x591a8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x591fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x59208 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x59218 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5922c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x59240 | `[sp-156] == 0x2` | `[sp-156] != 0x2` |
| `beq` at 0x5924c | `[sp-156] == 0x4` | `[sp-156] != 0x4` |
| `beq` at 0x5926c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x592d0 | `[0x132580] == 0x0` | `[0x132580] != 0x0` |
| `bne` at 0x59390 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x59508 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x595d4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x59604 | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `beq` at 0x59624 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59658 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x596c0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59728 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59770 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x597c4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x597f4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x59808 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x59840 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x59870 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x59888 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x598b8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x598cc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x59904 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x59928 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x59950 | `[sp-212] != 0x0` | `[sp-212] == 0x0` |
| `bne` at 0x59968 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x59988 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x599c0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59a0c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59a58 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59aa4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59bfc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59c20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59c44 | `? == 0xb4` | `? != 0xb4` |
| `beq` at 0x59cbc | `? == [sp-88]` | `? != [sp-88]` |
| `beq` at 0x59cd8 | `? == [sp-92]` | `? != [sp-92]` |
| `beq` at 0x59d34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59df0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59e38 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59e80 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59ec8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59f10 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59f58 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59f98 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x59fe4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5a02c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5a074 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5a0c0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x5a0fc | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `beq` at 0x5a110 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5a120 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x5a150 | `[sp-156] != 0x4` | `[sp-156] == 0x4` |
| `beq` at 0x5a180 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5a1c8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5a204 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x5a24c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bhs` at 0x5a278 | `C == 1` | `not (C == 1)` |
| `beq` at 0x5a2e0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5a318 | `r0` | `?` |

## `PktGenMemOp`

- `PktGenMemOp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57b08 to 0x58b64.
  Established. Equality entry `r0 == 0x1` takes `0x57b6c`, while inequality falls through (`beq` at 0x57b5c).
  Included word stores write `0x0` and `[sp-180]` to `0x12da7c` (`str` at 0x57da0, `str` at 0x5897c).
  An indirect call with unknown target carries `[sp-28]`, `[[sp-124]+4]`, `0x40000000`, `0x8` (`mov` at 0x584d0).
  Memory-operation packet field ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57b18 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x57b1c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x57b20 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x57b24 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x57b2c | store | 4 | `sp -0x7c` | `0x0` |
| `str` at 0x57b34 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x57b3c | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x57b44 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x57b48 | load | 4 | `abs 0x58a5c` | `-` |
| `ldr` at 0x57b4c | load | 4 | `abs 0x12da7c` | `-` |
| `str` at 0x57b50 | store | 4 | `sp -0xb4` | `[0x12da7c]` |
| `ldr` at 0x57b54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57b60 | load | 4 | `abs 0x58a60` | `-` |
| `str` at 0x57b64 | store | 4 | `sp -0xc0` | `0xfffff377` |
| `ldr` at 0x57b6c | load | 4 | `abs 0x58a64` | `-` |
| `str` at 0x57b70 | store | 4 | `sp -0x2c` | `0xde16c` |
| `ldr` at 0x57b74 | load | 4 | `abs 0x58a68` | `-` |
| `str` at 0x57b78 | store | 4 | `sp -0x28` | `0xde274` |
| `ldr` at 0x57b7c | load | 4 | `abs 0x58a6c` | `-` |
| `ldr` at 0x57b80 | load | 4 | `abs 0x132580` | `-` |
| `ldr` at 0x57b8c | load | 4 | `abs 0x58a70` | `-` |
| `ldr` at 0x57b90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57b94 | store | 4 | `abs 0x299854` | `0xde16c` |
| `ldr` at 0x57b98 | load | 4 | `abs 0x58a74` | `-` |
| `ldr` at 0x57b9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x57ba0 | store | 4 | `abs 0x299850` | `[sp-40]` |
| `ldr` at 0x57ba4 | load | 4 | `abs 0x58a78` | `-` |
| `str` at 0x57ba8 | store | 4 | `sp -0x2c` | `0x57a08` |
| `ldr` at 0x57bac | load | 4 | `abs 0x58a7c` | `-` |
| `str` at 0x57bb0 | store | 4 | `sp -0x28` | `0x57a8c` |
| `ldr` at 0x57bb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57bc0 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x57bc4 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x57bc8 | store | 4 | `sp -0xa8` | `[sp-196]` |
| `ldr` at 0x57bcc | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x57bd8 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x57be4 | load | 4 | `abs 0x58a80` | `-` |
| `ldr` at 0x57bec | load | 4 | `abs 0x58a60` | `-` |
| `str` at 0x57bf0 | store | 4 | `sp -0xc0` | `0xfffff377` |
| `ldr` at 0x57bf8 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x57bfc | store | 4 | `sp -0xc8` | `[sp-196]` |
| `ldr` at 0x57c00 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x57c0c | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x57c1c | load | 4 | `abs 0x58a84` | `-` |
| `str` at 0x57c20 | store | 4 | `sp -0x7c` | `0xf9464` |
| `str` at 0x57c28 | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x57c2c | load | 4 | `abs 0x58a88` | `-` |
| `str` at 0x57c30 | store | 4 | `sp -0x84` | `0xf94bc` |
| `str` at 0x57c38 | store | 4 | `sp -0x88` | `0xb` |
| `ldr` at 0x57c3c | load | 4 | `abs 0x58a8c` | `-` |
| `str` at 0x57c40 | store | 4 | `sp -0x8c` | `0xf94e8` |
| `str` at 0x57c48 | store | 4 | `sp -0x90` | `0x6` |
| `ldr` at 0x57c4c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x57c54 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x57c5c | load | 4 | `abs 0x58a90` | `-` |
| `str` at 0x57c60 | store | 4 | `sp -0x7c` | `0xf93c8` |
| `str` at 0x57c68 | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x57c6c | load | 4 | `abs 0x58a94` | `-` |
| `str` at 0x57c70 | store | 4 | `sp -0x84` | `0xf9420` |
| `str` at 0x57c78 | store | 4 | `sp -0x88` | `0xb` |
| `str` at 0x57c80 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x57c88 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x57c8c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x57c94 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x57c98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57ca4 | store | 4 | `sp -0x30` | `ret(0xdefac)` |
| `ldr` at 0x57ca8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57cb4 | load | 4 | `abs 0x58a98` | `-` |
| `ldr` at 0x57cb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57cc0 | load | 4 | `abs 0x58a9c` | `-` |
| `str` at 0x57cc4 | store | 4 | `sp -0xc0` | `0xfffff375` |
| `ldr` at 0x57ccc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57ce0 | load | 4 | `abs 0x58aa0` | `-` |
| `ldr` at 0x57ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57cec | load | 4 | `abs 0x58aa4` | `-` |
| `str` at 0x57cf0 | store | 4 | `sp -0xc0` | `0xfffff374` |
| `ldr` at 0x57cf8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x57d04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57d0c | load | 4 | `[sp-124] +0x0` | `-` |
| `ldr` at 0x57d28 | load | 4 | `abs 0x58aa8` | `-` |
| `str` at 0x57d34 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x57d3c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x57d48 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x57d50 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x57d58 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57d68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57d6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57d70 | load | 4 | `[sp-132] +0x20` | `-` |
| `str` at 0x57d90 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x57d98 | load | 4 | `abs 0x58a5c` | `-` |
| `str` at 0x57da0 | store | 4 | `abs 0x12da7c` | `0x0` |
| `str` at 0x57da8 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x57db0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x57db8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57dc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57dc8 | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x57dcc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x57dd0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x57de8 | load | 4 | `abs 0x58aac` | `-` |
| `ldr` at 0x57df0 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x57df4 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x57dfc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57e00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57e04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57e08 | load | 4 | `[sp-132] +0x0` | `-` |
| `ldr` at 0x57e0c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x57e10 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x57e28 | load | 4 | `abs 0x58ab4` | `-` |
| `ldr` at 0x57e30 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x57e34 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x57e40 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57e48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57e50 | load | 4 | `[sp-132] +0x1c` | `-` |
| `ldr` at 0x57e54 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x57e58 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x57e70 | load | 4 | `abs 0x58ab8` | `-` |
| `ldr` at 0x57e78 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x57e7c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x57e88 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57e90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57e98 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x57e9c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x57ea0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x57eb8 | load | 4 | `abs 0x58abc` | `-` |
| `ldr` at 0x57ec0 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x57ec4 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x57ecc | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x57ed8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x57edc | store | 4 | `sp -0x54` | `[sp-68]` |
| `ldr` at 0x57ee0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x57ee8 | store | 4 | `sp -0x58` | `?` |
| `str` at 0x57ef0 | store | 4 | `sp -0x54` | `?` |
| `str` at 0x57ef8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x57f00 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x57f10 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x57f14 | load | 4 | `abs 0x58ac0` | `-` |
| `str` at 0x57f18 | store | 4 | `sp -0x58` | `0x7ffffd` |
| `ldr` at 0x57f20 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x57f28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57f30 | load | 4 | `[sp-132] +0x20` | `-` |
| `ldr` at 0x57f34 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x57f38 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x57f50 | load | 4 | `abs 0x58ac4` | `-` |
| `ldr` at 0x57f58 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x57f5c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x57f68 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x57f6c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x57f70 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x57f7c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x57f84 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x57f94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57f98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57f9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x57fb8 | load | 4 | `abs 0x58ac8` | `-` |
| `str` at 0x57fc4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x57fcc | load | 4 | `abs 0x58acc` | `-` |
| `ldr` at 0x57fd0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x57fd4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x57fd8 | store | 4 | `(unknown)` | `[sp-84]` |
| `ldr` at 0x57fdc | load | 4 | `abs 0x58acc` | `-` |
| `ldr` at 0x57fe0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x57ff4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x57ff8 | store | 4 | `(unknown)` | `[sp-88]` |
| `ldr` at 0x57ffc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x58000 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x58008 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x5800c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58014 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5801c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58024 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58028 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x5802c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x58044 | load | 4 | `abs 0x58ad0` | `-` |
| `ldr` at 0x5804c | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58050 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x58058 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x58060 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x58068 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58074 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x58078 | store | 4 | `sp -0x20` | `[sp-52]` |
| `ldr` at 0x5807c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5808c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x58090 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x58094 | load | 4 | `abs 0x58ad8` | `-` |
| `strh` at 0x58098 | store | 2 | `abs 0x1325a0` | `0x123` |
| `ldr` at 0x5809c | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x580a0 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x580a4 | store | 1 | `abs 0x132598` | `([sp-56] & 0xff)` |
| `ldr` at 0x580a8 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x580ac | load | 4 | `sp -0x34` | `-` |
| `strh` at 0x580b0 | store | 2 | `abs 0x132596` | `([sp-52] & 0xffff)` |
| `ldr` at 0x580b4 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x580b8 | load | 4 | `sp +0x0` | `-` |
| `strh` at 0x580c0 | store | 2 | `abs 0x132594` | `?` |
| `ldr` at 0x580c4 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x580c8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x580cc | store | 1 | `abs 0x13258f` | `([sp-36] & 0xff)` |
| `ldr` at 0x580d0 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x580d4 | load | 4 | `sp -0x20` | `-` |
| `strh` at 0x580d8 | store | 2 | `abs 0x13258c` | `([sp-32] & 0xffff)` |
| `ldr` at 0x580dc | load | 4 | `abs 0x58ad4` | `-` |
| `str` at 0x580e0 | store | 4 | `sp -0xcc` | `0x132588` |
| `ldr` at 0x580e4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x580f4 | store | 1 | `sp -0xcd` | `0x4` |
| `strb` at 0x58100 | store | 1 | `sp -0xcd` | `0x4c` |
| `ldrb` at 0x58104 | load | 1 | `sp -0xcd` | `-` |
| `ldr` at 0x58108 | load | 4 | `sp -0xcc` | `-` |
| `strb` at 0x5810c | store | 1 | `abs 0x13258e` | `([sp-205]:1 & 0xff)` |
| `ldr` at 0x58110 | load | 4 | `abs 0x58ad4` | `-` |
| `strb` at 0x58118 | store | 1 | `abs 0x13258a` | `0x0` |
| `ldr` at 0x5811c | load | 4 | `abs 0x58ad4` | `-` |
| `strb` at 0x58124 | store | 1 | `abs 0x132589` | `0x0` |
| `ldr` at 0x58128 | load | 4 | `abs 0x58ad4` | `-` |
| `strb` at 0x58130 | store | 1 | `abs 0x132588` | `0x0` |
| `str` at 0x58138 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x5813c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58140 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x5814c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58154 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x5815c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x58160 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58168 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x58170 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x58174 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x58178 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x58184 | store | 1 | `abs 0x132599` | `?` |
| `ldr` at 0x58188 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x5818c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x58198 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x581a8 | store | 4 | `abs 0x132590` | `?` |
| `ldr` at 0x581ac | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x581b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x581c0 | store | 4 | `abs 0x13259c` | `?` |
| `ldr` at 0x581c4 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x581c8 | load | 4 | `abs 0x132590` | `-` |
| `ldr` at 0x581d4 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x581d8 | load | 1 | `abs 0x13258f` | `-` |
| `str` at 0x581e4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x581e8 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x581ec | load | 1 | `abs 0x13258e` | `-` |
| `ldr` at 0x581f4 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrh` at 0x581f8 | load | 2 | `abs 0x13258c` | `-` |
| `ldr` at 0x5820c | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x58210 | load | 1 | `abs 0x13258a` | `-` |
| `ldr` at 0x58220 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x58224 | load | 1 | `abs 0x132589` | `-` |
| `ldr` at 0x58230 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x58234 | load | 1 | `abs 0x132588` | `-` |
| `str` at 0x58240 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x58248 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58254 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58258 | load | 4 | `[sp-124] +0x8` | `-` |
| `ldr` at 0x5825c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x58260 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x58278 | load | 4 | `abs 0x58adc` | `-` |
| `ldr` at 0x58280 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58284 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x5828c | load | 4 | `abs 0x58ad4` | `-` |
| `ldrh` at 0x58290 | load | 2 | `abs 0x1325a0` | `-` |
| `ldr` at 0x582a0 | load | 4 | `abs 0x58ad4` | `-` |
| `ldr` at 0x582a4 | load | 4 | `abs 0x13259c` | `-` |
| `ldr` at 0x582b8 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x582bc | load | 1 | `abs 0x132599` | `-` |
| `str` at 0x582cc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x582d0 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrb` at 0x582d4 | load | 1 | `abs 0x132598` | `-` |
| `ldr` at 0x582e0 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrh` at 0x582e4 | load | 2 | `abs 0x132596` | `-` |
| `ldr` at 0x582f8 | load | 4 | `abs 0x58ad4` | `-` |
| `ldrh` at 0x582fc | load | 2 | `abs 0x132594` | `-` |
| `str` at 0x5830c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x58314 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x5831c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58324 | load | 4 | `[sp-124] +0xc` | `-` |
| `ldr` at 0x58328 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x5832c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x58344 | load | 4 | `abs 0x58ae0` | `-` |
| `ldr` at 0x5834c | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58350 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x5835c | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x58360 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x58368 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x5836c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x58378 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58384 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x5838c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x58394 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x583a0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x583ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x583b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x583bc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x583c8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x583d4 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x583e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x583ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x583f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58400 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x5840c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58418 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x58420 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x58428 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x58434 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x58440 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5844c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58450 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58468 | load | 4 | `abs 0x58ae4` | `-` |
| `ldr` at 0x5846c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x58474 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58478 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x58480 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x58488 | store | 4 | `sp -0xac` | `[sp-172]+1` |
| `ldr` at 0x5848c | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x58494 | store | 4 | `sp -0xb0` | `[sp-176]+2` |
| `str` at 0x584a0 | store | 4 | `sp -0x54` | `0x40000000` |
| `str` at 0x584a8 | store | 4 | `sp -0x58` | `0x8` |
| `ldr` at 0x584b0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x584b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x584bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x584c0 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0x584c4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x584c8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x584e0 | load | 4 | `abs 0x58ae8` | `-` |
| `ldr` at 0x584e8 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x584ec | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x584f8 | store | 4 | `sp -0x9c` | `0xa` |
| `str` at 0x58500 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x58504 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58510 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x58518 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x58528 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x58538 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5853c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58540 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0x5855c | load | 4 | `abs 0x58aec` | `-` |
| `str` at 0x58568 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x58570 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x5857c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x58584 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58590 | load | 4 | `abs 0x58af0` | `-` |
| `ldr` at 0x58598 | load | 4 | `abs 0x58af4` | `-` |
| `str` at 0x5859c | store | 4 | `sp -0xa4` | `0xfffff37e` |
| `str` at 0x585a8 | store | 4 | `sp -0x9c` | `0xa` |
| `str` at 0x585b0 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x585b4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x585c0 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x585c8 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x585d8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x585e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x585ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x585f0 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x5860c | load | 4 | `abs 0x58af8` | `-` |
| `str` at 0x58618 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x58620 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x58634 | store | 4 | `sp -0xd4` | `0x0` |
| `str` at 0x58640 | store | 4 | `sp -0xd4` | `0x1` |
| `ldr` at 0x58644 | load | 4 | `sp -0xd4` | `-` |
| `str` at 0x58648 | store | 4 | `sp -0x98` | `[sp-212]` |
| `ldr` at 0x58650 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5865c | load | 4 | `abs 0x58afc` | `-` |
| `ldr` at 0x58664 | load | 4 | `abs 0x58b00` | `-` |
| `str` at 0x58668 | store | 4 | `sp -0xa4` | `0xfffff37d` |
| `ldr` at 0x58674 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x58684 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58688 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5868c | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x586a8 | load | 4 | `abs 0x58b04` | `-` |
| `str` at 0x586b4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x586bc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x586cc | load | 4 | `abs 0x58b08` | `-` |
| `ldr` at 0x586d4 | load | 4 | `abs 0x58b0c` | `-` |
| `str` at 0x586d8 | store | 4 | `sp -0xa4` | `0xfffff37c` |
| `ldr` at 0x586e0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x586f0 | load | 4 | `abs 0x58b10` | `-` |
| `ldr` at 0x586f8 | load | 4 | `abs 0x58b14` | `-` |
| `str` at 0x586fc | store | 4 | `sp -0xa4` | `0xfffff37b` |
| `ldr` at 0x58704 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x58714 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x5871c | load | 4 | `abs 0x58b18` | `-` |
| `ldr` at 0x58728 | load | 4 | `abs 0x58b1c` | `-` |
| `str` at 0x5872c | store | 4 | `sp -0xa4` | `0xfffff37a` |
| `str` at 0x58738 | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x58740 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x58748 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x58750 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x58758 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x58768 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5876c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58770 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x5878c | load | 4 | `abs 0x58af8` | `-` |
| `str` at 0x58798 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x587a4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x587ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x587b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x587b4 | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x587b8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x587bc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x587d4 | load | 4 | `abs 0x58b20` | `-` |
| `ldr` at 0x587dc | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x587e0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x587e8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x587ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x587f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x587f4 | load | 4 | `[sp-132] +0x0` | `-` |
| `ldr` at 0x587f8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x587fc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x58814 | load | 4 | `abs 0x58b24` | `-` |
| `ldr` at 0x5881c | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58820 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x5882c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x58834 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58838 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5883c | load | 4 | `[sp-132] +0x28` | `-` |
| `ldr` at 0x5885c | load | 4 | `abs 0x58b28` | `-` |
| `ldr` at 0x58864 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58868 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x58874 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x5887c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58884 | load | 4 | `[sp-132] +0x24` | `-` |
| `ldr` at 0x588a4 | load | 4 | `abs 0x58b2c` | `-` |
| `ldr` at 0x588ac | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x588b0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x588bc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x588cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x588d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x588d4 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x588f0 | load | 4 | `abs 0x58af8` | `-` |
| `str` at 0x588fc | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x58904 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x5891c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x58920 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5892c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x58934 | store | 4 | `sp -0x9c` | `[sp-156]-1` |
| `ldr` at 0x58944 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x58950 | load | 4 | `abs 0x58b30` | `-` |
| `ldr` at 0x58958 | load | 4 | `abs 0x58b34` | `-` |
| `str` at 0x5895c | store | 4 | `sp -0xa4` | `0xfffff378` |
| `ldr` at 0x58964 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x5896c | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x58974 | load | 4 | `abs 0x58a5c` | `-` |
| `ldr` at 0x58978 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x5897c | store | 4 | `abs 0x12da7c` | `[sp-180]` |
| `ldr` at 0x58984 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x5898c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58994 | load | 4 | `[sp-132] +0x20` | `-` |
| `ldr` at 0x58998 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x5899c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x589b4 | load | 4 | `abs 0x58b38` | `-` |
| `ldr` at 0x589bc | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x589c0 | store | 4 | `sp -0xc0` | `0xfffff380` |
| `str` at 0x589cc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x589d0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x589d4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x589e0 | load | 4 | `abs 0x58acc` | `-` |
| `ldr` at 0x589e4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x589e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x589ec | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x589f0 | load | 4 | `abs 0x58acc` | `-` |
| `ldr` at 0x589f4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58a08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x58a0c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x58a10 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58a18 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x58a20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58a24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58a28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58a2c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x58a30 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x58a48 | load | 4 | `abs 0x58b3c` | `-` |
| `ldr` at 0x58a50 | load | 4 | `abs 0x58ab0` | `-` |
| `str` at 0x58a54 | store | 4 | `sp -0xc0` | `0xfffff380` |
| `ldr` at 0x58b40 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x58b48 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x58b50 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x58b54 | store | 4 | `sp -0xc0` | `[sp-164]` |
| `ldr` at 0x58b58 | load | 4 | `sp -0xc0` | `-` |
| `ldm` at 0x58b60 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x58b60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x58b60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x58b60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57be8 | `(unknown callee) 0x1e958` | `0xf9520` | `r1` | `[sp-196]` | `[sp-196]` |
| `bl` at 0x57c9c | `(unknown callee) 0xdefac` | `?` | `r1` | `[sp-196]` | `?` |
| `bl` at 0x57cbc | `(unknown callee) 0x1e958` | `0xf9534` | `[sp-28]` | `?` | `ret(0xdefac)` |
| `bl` at 0x57cd0 | `(unknown callee) 0xdcd98` | `ret(0xdefac)` | `?` | `?` | `ret(0xdefac)` |
| `bl` at 0x57ce8 | `(unknown callee) 0x1e958` | `0xf9554` | `[sp-28]` | `?` | `ret(0xdcd98)` |
| `mov` at 0x57d18 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]]` | `sp-76` | `sp-80` |
| `bl` at 0x57d2c | `(unknown callee) 0x1e958` | `0xf9574` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x57d7c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `sp-68` | `sp-72` |
| `mov` at 0x57dd8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `0x0` | `0x0` |
| `bl` at 0x57dec | `(unknown callee) 0x1e958` | `0xf9594` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x57e18 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x57e2c | `(unknown callee) 0x1e958` | `0xf95b4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x57e60 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+28]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x57e74 | `(unknown callee) 0x1e958` | `0xf95d4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x57ea8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x57ebc | `(unknown callee) 0x1e958` | `0xf95f4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x57f40 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x57f54 | `(unknown callee) 0x1e958` | `0xf9610` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x57fa8 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `sp-84` | `sp-88` |
| `bl` at 0x57fbc | `(unknown callee) 0x1e958` | `0xf9630` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58034 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x58048 | `(unknown callee) 0x1e958` | `0xf9658` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58268 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+8]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x5827c | `(unknown callee) 0x1e958` | `0xf9680` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58334 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+12]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x58348 | `(unknown callee) 0x1e958` | `0xf96a0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58458 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x58470 | `(unknown callee) 0x1e958` | `0xf96c0` | `[sp-172]` | `?` | `ret(unknown)` |
| `mov` at 0x584d0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+4]` | `0x40000000` | `0x8` |
| `bl` at 0x584e4 | `(unknown callee) 0x1e958` | `0xf96e0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5854c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+4]` | `sp-84` | `sp-88` |
| `bl` at 0x58560 | `(unknown callee) 0x1e958` | `0xf9704` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x58594 | `(unknown callee) 0x1e958` | `0xf971c` | `?` | `?` | `?` |
| `mov` at 0x585fc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-84` | `sp-88` |
| `bl` at 0x58610 | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x58660 | `(unknown callee) 0x1e958` | `0xf9748` | `?` | `?` | `?` |
| `mov` at 0x58698 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `sp-92` | `sp-96` |
| `bl` at 0x586ac | `(unknown callee) 0x1e4ac` | `0xf975c` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x586d0 | `(unknown callee) 0x1e4ac` | `0xf9770` | `?` | `?` | `?` |
| `bl` at 0x586f4 | `(unknown callee) 0x1e4ac` | `0xf9780` | `?` | `?` | `?` |
| `bl` at 0x58724 | `(unknown callee) 0x1e958` | `0xf9794` | `?` | `?` | `?` |
| `mov` at 0x5877c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-184` | `sp-188` |
| `bl` at 0x58790 | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x587c4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x587d8 | `(unknown callee) 0x1e958` | `0xf97ac` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58804 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x58818 | `(unknown callee) 0x1e958` | `0xf97c0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x5884c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+40]` | `?` | `?` |
| `bl` at 0x58860 | `(unknown callee) 0x1e958` | `0xf97d8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58894 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+36]` | `?` | `?` |
| `bl` at 0x588a8 | `(unknown callee) 0x1e958` | `0xf97f4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x588e0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-184` | `sp-188` |
| `bl` at 0x588f4 | `(unknown callee) 0x1e958` | `0xf9730` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x58954 | `(unknown callee) 0x1e958` | `0xf980c` | `?` | `?` | `[sp-152]` |
| `mov` at 0x589a4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x589b8 | `(unknown callee) 0x1e958` | `0xf9820` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x58a38 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x58a4c | `(unknown callee) 0x1e958` | `0xf9844` | `?` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x57b5c | `r0 == 0x1` | `r0 != 0x1` |
| `beq` at 0x57b88 | `[0x132580] == 0x0` | `[0x132580] != 0x0` |
| `beq` at 0x57bd4 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `beq` at 0x57be0 | `[sp-196] == 0x4` | `[sp-196] != 0x4` |
| `beq` at 0x57c08 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `beq` at 0x57c14 | `[sp-196] == 0x4` | `[sp-196] != 0x4` |
| `bne` at 0x57cb0 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `beq` at 0x57cdc | `ret(0xdcd98) == 0x0` | `ret(0xdcd98) != 0x0` |
| `beq` at 0x57d24 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x57d88 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x57de4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x57e24 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x57e6c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x57eb4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x57ed4 | `[sp-168] != 0x2` | `[sp-168] == 0x2` |
| `bne` at 0x57f08 | `[sp-168] != 0x4` | `[sp-168] == 0x4` |
| `beq` at 0x57f4c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bhs` at 0x57f78 | `C == 1` | `not (C == 1)` |
| `beq` at 0x57fb4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58040 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x58070 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x58084 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x580ec | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bge` at 0x58148 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x58274 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58340 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bhi` at 0x58374 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `ble` at 0x58380 | `[sp-160] <= 0x0 (signed, 32-bit)` | `[sp-160] > 0x0 (signed, 32-bit)` |
| `bne` at 0x583b8 | `? != ?` | `? == ?` |
| `bne` at 0x583f8 | `? != ?` | `? == ?` |
| `beq` at 0x58464 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x584dc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x5850c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x58520 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x58558 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x5858c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x585bc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x585d0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x58608 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x5862c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x58658 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x586a4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x586c8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x586ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x58710 | `? == 0xe0` | `? != 0xe0` |
| `beq` at 0x58788 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x587d0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58810 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58858 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x588a0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x588ec | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x58928 | `[sp-152] != 0x0` | `[sp-152] == 0x0` |
| `beq` at 0x5893c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5894c | `[sp-152] != 0x0` | `[sp-152] == 0x0` |
| `beq` at 0x589b0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bhs` at 0x589dc | `C == 1` | `not (C == 1)` |
| `beq` at 0x58a44 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x58b60 | `r0` | `?` |

## `TRIGReadDetails`

- `TRIGReadDetails`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb29b0 to 0xb3a14.
  Established. A word read from literal address `0xb387c` supplies `0xffff`, which is assigned to `sp -0x24` (`ldr` at 0xb29e4, `str` at 0xb29e8).
  An unnamed `0xb0840` call carries `[sp-44]`, `0x37`, `0x30` and an unknown fourth candidate (`bl` at 0xb2f60).
  Equality `ret(0xb0840) == 0xff` takes `0xb3010`, while inequality falls through (`beq` at 0xb2f6c).
  A later unnamed `0xb3a14` call carries `ret(0xb0840)`, unknown, unknown and `ret(0xb0840)` (`bl` at 0xb3000).
  The hardware meaning of the selected details remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb29c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb29c4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb29c8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb29cc | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb29d4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb29d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb29e4 | load | 4 | `abs 0xb387c` | `-` |
| `str` at 0xb29e8 | store | 4 | `sp -0x24` | `0xffff` |
| `ldr` at 0xb29ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb29fc | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0xb2a00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb2a10 | store | 4 | `sp -0x24` | `0x7` |
| `ldr` at 0xb2a14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb2a24 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0xb2a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2a34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2a4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb2a60 | store | 4 | `sp -0x24` | `0x7f` |
| `ldr` at 0xb2a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2a74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2a88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb2a98 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb2a9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2aa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb2aac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2ab0 | load | 4 | `abs 0xb3880` | `-` |
| `ldr` at 0xb2ac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2acc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb2ad4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2ad8 | load | 4 | `abs 0xb3884` | `-` |
| `str` at 0xb2af0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xb2af4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2b08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2b0c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2b18 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2b1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2b20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2b2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2b34 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2b3c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2b40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2b54 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2b64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2b68 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2b74 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2b78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2b7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2b88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2b90 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2b98 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2b9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2bb0 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2bb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2bbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2bc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2be0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2be8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2c00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2c14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2c18 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2c24 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2c28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2c2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2c38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2c40 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2c48 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2c4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb2c68 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2c7c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2c80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2c88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2c90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2ca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2cb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2cbc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2cc8 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2ccc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2cd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2cdc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2ce4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2cec | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2cf0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2d04 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2d10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2d18 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2d2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2d40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2d44 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2d50 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2d54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2d58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2d64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2d6c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2d74 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2d78 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2d8c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2d90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2d98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2da0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2dc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2dcc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2dd8 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2ddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2de0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2dec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2df4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2dfc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2e00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2e14 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2e18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2e20 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb2e28 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2e48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2e50 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb2e58 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2e74 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xb2e7c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2e9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ea4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb2eac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2ec8 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xb2ed0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ef8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2f00 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb2f1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2f20 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb2f2c | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb2f30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2f34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2f40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2f48 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2f50 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2f54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb2f70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2f7c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2f90 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2f94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2fa0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2fb4 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2fb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2fc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2fd8 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb2fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2fe8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3008 | store | 4 | `sp -0x24` | `ret(0xb3a14)` |
| `ldr` at 0xb3018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb301c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3028 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb302c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3030 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb303c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3044 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb304c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb3050 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb306c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3078 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb308c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3090 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb30ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb30bc | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb30c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb30d0 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb30d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb30e4 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb30e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb30f8 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb3100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb310c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3120 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3130 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3144 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3148 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3154 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3174 | store | 4 | `sp -0x24` | `ret(0xb3a14)` |
| `ldr` at 0xb317c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3184 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb318c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb31a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb31ac | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb31b8 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb31bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb31c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb31cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb31d4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb31dc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb31e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb31fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3208 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb321c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb322c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3240 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3250 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3264 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3274 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3278 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3284 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb3288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb328c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3298 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb32a0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb32a8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb32ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb32c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb32d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb32e8 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb32ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb3308 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3318 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb331c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb332c | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb3330 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3340 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb3344 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3354 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xb335c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3368 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb337c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb338c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb33a0 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb33a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb33b4 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xb33b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb33c8 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xb33cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb33dc | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xb33e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb33f0 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xb33f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3400 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb3408 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb3424 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xb342c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb344c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3454 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb345c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb3478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb347c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3488 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb348c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3490 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb349c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb34a4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb34ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb34b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb34bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb34d4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb34d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb34e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb34f8 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb34fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3508 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb351c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb352c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb3534 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb3550 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3554 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3560 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb3564 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3568 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3574 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb357c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb3584 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb3588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3594 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb35ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb35b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb35bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb35d0 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb35d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb35e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb35f4 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb35fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3604 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb360c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb3628 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb362c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3638 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb363c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3640 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb364c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3658 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3660 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb3668 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb366c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3680 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3684 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb368c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb3698 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb369c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb36a8 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb36ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb36b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb36bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb36c4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb36cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb36d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb36e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb36ec | store | 4 | `sp -0x24` | `([sp-36] + ret(0xb0840))` |
| `ldr` at 0xb36f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb36fc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3708 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb370c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3710 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb371c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3724 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb372c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb3730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb373c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3750 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3760 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3774 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3780 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3784 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3790 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb3794 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3798 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb37a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb37ac | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb37b4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb37b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb37c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb37e4 | store | 4 | `sp -0x24` | `ret(0xb3a14)` |
| `ldr` at 0xb37e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb37f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3814 | store | 4 | `sp -0x24` | `ret(0xb3a14)` |
| `ldr` at 0xb3818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3824 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3844 | store | 4 | `sp -0x24` | `ret(0xb3a14)` |
| `ldr` at 0xb3848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3854 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3874 | store | 4 | `sp -0x24` | `ret(0xb3a14)` |
| `ldr` at 0xb3888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3890 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb3898 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb38b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb38b8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb38c4 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb38c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb38cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb38d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb38e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb38ec | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb38f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb38f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb390c | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3910 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3918 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb3924 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3928 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3934 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb3938 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb393c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3948 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb3950 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb3958 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb395c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb3970 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3978 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xb0840))` |
| `ldr` at 0xb3984 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3988 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb3994 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb3998 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb399c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb39a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb39b0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb39b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb39bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb39c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb39dc | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb39e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb39ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb3a00 | store | 4 | `sp -0x24` | `ret(0xb0840)` |
| `ldr` at 0xb3a04 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xb3a10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3a10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3a10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb2ab4 | `(unknown callee) 0x19310` | `?` | `0x118c34` | `?` | `?` |
| `bl` at 0xb2adc | `(unknown callee) 0x19310` | `?` | `0x118c40` | `?` | `?` |
| `bl` at 0xb2b10 | `(unknown callee) 0xb3b48` | `0x7` | `0x15` | `[sp-32]` | `[sp]` |
| `bl` at 0xb2b4c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x2a` | `0x20` | `?` |
| `bl` at 0xb2b6c | `(unknown callee) 0xb3b48` | `0x7` | `0x14` | `[sp-32]` | `[sp]` |
| `bl` at 0xb2ba8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x30` | `0x26` | `?` |
| `bl` at 0xb2c1c | `(unknown callee) 0xb3b48` | `0x5` | `0x14` | `?` | `[sp]` |
| `bl` at 0xb2c58 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3f` | `0x3f` | `?` |
| `bl` at 0xb2c74 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x25` | `0x18` | `ret(0xb0840)` |
| `bl` at 0xb2cc0 | `(unknown callee) 0xb3b48` | `0xa` | `0x15` | `?` | `[sp]` |
| `bl` at 0xb2cfc | `(unknown callee) 0xb0840` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xb2d48 | `(unknown callee) 0xb3b48` | `0xc` | `0x15` | `?` | `[sp]` |
| `bl` at 0xb2d84 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xb2dd0 | `(unknown callee) 0xb3b48` | `0x9` | `0x15` | `?` | `[sp]` |
| `bl` at 0xb2e0c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xb2f24 | `(unknown callee) 0xb3b48` | `0x7` | `0x1a` | `?` | `[sp]` |
| `bl` at 0xb2f60 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x37` | `0x30` | `?` |
| `bl` at 0xb2f88 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x2d` | `0x23` | `[sp-20]` |
| `bl` at 0xb2fac | `(unknown callee) 0xb0840` | `[sp-44]` | `0x4` | `0x4` | `[sp-20]` |
| `bl` at 0xb2fd0 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x20` | `0x1f` | `[sp-20]` |
| `bl` at 0xb2ff4 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x37` | `0x30` | `[sp-20]` |
| `bl` at 0xb3000 | `(unknown callee) 0xb3a14` | `ret(0xb0840)` | `?` | `?` | `ret(0xb0840)` |
| `bl` at 0xb3020 | `(unknown callee) 0xb3b48` | `0x7` | `0x25` | `[sp-32]` | `[sp]` |
| `bl` at 0xb305c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3f` | `0x38` | `?` |
| `bl` at 0xb3084 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x30` | `0x26` | `[sp-20]` |
| `bl` at 0xb309c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x25` | `0x20` | `?` |
| `bl` at 0xb3118 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xb313c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xb3160 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3a` | `0x33` | `[sp-20]` |
| `bl` at 0xb316c | `(unknown callee) 0xb3a14` | `ret(0xb0840)` | `?` | `?` | `ret(0xb0840)` |
| `bl` at 0xb31b0 | `(unknown callee) 0xb3b48` | `0x7` | `0x1e` | `?` | `[sp]` |
| `bl` at 0xb31ec | `(unknown callee) 0xb0840` | `[sp-44]` | `0x37` | `0x30` | `?` |
| `bl` at 0xb3214 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x2d` | `0x23` | `[sp-20]` |
| `bl` at 0xb3238 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x4` | `0x4` | `[sp-20]` |
| `bl` at 0xb325c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x20` | `0x1f` | `[sp-20]` |
| `bl` at 0xb327c | `(unknown callee) 0xb3b48` | `0x7` | `0x25` | `[sp-32]` | `[sp]` |
| `bl` at 0xb32b8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3f` | `0x38` | `?` |
| `bl` at 0xb32e0 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x30` | `0x26` | `[sp-20]` |
| `bl` at 0xb32f8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x25` | `0x20` | `?` |
| `bl` at 0xb3374 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xb3398 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xb3480 | `(unknown callee) 0xb3b48` | `0x6` | `0x1a` | `?` | `[sp]` |
| `bl` at 0xb34c8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xb34f0 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xb3514 | `(unknown callee) 0xb0840` | `[sp-44]` | `0xb` | `0xb` | `[sp-20]` |
| `bl` at 0xb3558 | `(unknown callee) 0xb3b48` | `0x6` | `0x1e` | `?` | `[sp]` |
| `bl` at 0xb35a0 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xb35c8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xb35ec | `(unknown callee) 0xb0840` | `[sp-44]` | `0xb` | `0xb` | `[sp-20]` |
| `bl` at 0xb3630 | `(unknown callee) 0xb3b48` | `0x8` | `0x1d` | `?` | `[sp]` |
| `bl` at 0xb3678 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x4` | `0x0` | `?` |
| `bl` at 0xb36a0 | `(unknown callee) 0xb3b48` | `0x8` | `0x1c` | `[sp-32]` | `[sp]` |
| `bl` at 0xb36dc | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3f` | `0x39` | `?` |
| `bl` at 0xb3700 | `(unknown callee) 0xb3b48` | `0x8` | `0x1c` | `[sp-32]` | `[sp]` |
| `bl` at 0xb3748 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x1a` | `0x1a` | `[sp-20]` |
| `bl` at 0xb376c | `(unknown callee) 0xb0840` | `[sp-44]` | `0x36` | `0x35` | `[sp-20]` |
| `bl` at 0xb3788 | `(unknown callee) 0xb3b48` | `0x8` | `0x1a` | `[sp-32]` | `[sp]` |
| `bl` at 0xb37d0 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x7` | `0x0` | `[sp-20]` |
| `bl` at 0xb37dc | `(unknown callee) 0xb3a14` | `ret(0xb0840)` | `?` | `?` | `ret(0xb0840)` |
| `bl` at 0xb3800 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x17` | `0x10` | `[sp-20]` |
| `bl` at 0xb380c | `(unknown callee) 0xb3a14` | `ret(0xb0840)` | `?` | `?` | `ret(0xb0840)` |
| `bl` at 0xb3830 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x27` | `0x20` | `[sp-20]` |
| `bl` at 0xb383c | `(unknown callee) 0xb3a14` | `ret(0xb0840)` | `?` | `?` | `ret(0xb0840)` |
| `bl` at 0xb3860 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x37` | `0x30` | `[sp-20]` |
| `bl` at 0xb386c | `(unknown callee) 0xb3a14` | `ret(0xb0840)` | `?` | `?` | `ret(0xb0840)` |
| `bl` at 0xb38bc | `(unknown callee) 0xb3b48` | `0x8` | `0x21` | `?` | `[sp]` |
| `bl` at 0xb3904 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x4` | `0x0` | `?` |
| `bl` at 0xb392c | `(unknown callee) 0xb3b48` | `0x8` | `0x20` | `[sp-32]` | `[sp]` |
| `bl` at 0xb3968 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3f` | `0x39` | `?` |
| `bl` at 0xb398c | `(unknown callee) 0xb3b48` | `0x8` | `0x20` | `[sp-32]` | `[sp]` |
| `bl` at 0xb39d4 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x1a` | `0x1a` | `[sp-20]` |
| `bl` at 0xb39f8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x36` | `0x35` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb29e0 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xb29f4 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xb2a08 | `r0 != 0x2` | `r0 == 0x2` |
| `bne` at 0xb2a1c | `r0 != 0x7` | `r0 == 0x7` |
| `beq` at 0xb2a30 | `r0 == 0x3` | `r0 != 0x3` |
| `beq` at 0xb2a3c | `r0 == 0x4` | `r0 != 0x4` |
| `beq` at 0xb2a48 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0xb2a54 | `r0 == 0x6` | `r0 != 0x6` |
| `bne` at 0xb2a84 | `? != 0xb` | `? == 0xb` |
| `bne` at 0xb2a90 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xb2ac0 | `ret(0x19310) == 0x0` | `ret(0x19310) != 0x0` |
| `bne` at 0xb2ae8 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb2afc | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb2b28 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xb2b84 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xb2bd4 | `? == 0x5` | `? != 0x5` |
| `beq` at 0xb2bf8 | `? == 0x4` | `? != 0x4` |
| `bne` at 0xb2c08 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb2c34 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xb2c64 | `ret(0xb0840) == 0x0` | `ret(0xb0840) != 0x0` |
| `bne` at 0xb2ca0 | `? != 0x19` | `? == 0x19` |
| `bne` at 0xb2cac | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb2cd8 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb2d28 | `? != 0x1a` | `? == 0x1a` |
| `bne` at 0xb2d34 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb2d60 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb2db0 | `? != 0x18` | `? == 0x18` |
| `bne` at 0xb2dbc | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb2de8 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xb2e44 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb2e94 | `? != 0x4` | `? == 0x4` |
| `bne` at 0xb2ee8 | `? != 0x8` | `? == 0x8` |
| `bne` at 0xb2f10 | `? != 0xa` | `? == 0xa` |
| `bhi` at 0xb2f3c | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xb2f6c | `ret(0xb0840) == 0xff` | `ret(0xb0840) != 0xff` |
| `bne` at 0xb2f78 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xb2f9c | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb2fc0 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xb2fe4 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bhi` at 0xb3038 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb3068 | `ret(0xb0840) != 0xaa` | `ret(0xb0840) == 0xaa` |
| `bne` at 0xb3074 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xb30a8 | `ret(0xb0840) != 0x8` | `ret(0xb0840) == 0x8` |
| `bne` at 0xb30b4 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xb30c8 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xb30dc | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xb30f0 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xb3108 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb312c | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xb3150 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xb319c | `? != 0xd` | `? == 0xd` |
| `bhi` at 0xb31c8 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xb31f8 | `ret(0xb0840) == 0xff` | `ret(0xb0840) != 0xff` |
| `bne` at 0xb3204 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xb3228 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb324c | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bhi` at 0xb3294 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb32c4 | `ret(0xb0840) != 0xaa` | `ret(0xb0840) == 0xaa` |
| `bne` at 0xb32d0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xb3304 | `ret(0xb0840) != 0x8` | `ret(0xb0840) == 0x8` |
| `bne` at 0xb3310 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xb3324 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xb3338 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xb334c | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xb3364 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb3388 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xb33ac | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xb33c0 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xb33d4 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xb33e8 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xb3444 | `? != 0x1c` | `? == 0x1c` |
| `bne` at 0xb346c | `? != 0x6` | `? == 0x6` |
| `bhi` at 0xb3498 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb34b8 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb34e0 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xb3504 | `[sp-20] != 0x7` | `[sp-20] == 0x7` |
| `bne` at 0xb3544 | `? != 0x8` | `? == 0x8` |
| `bhi` at 0xb3570 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb3590 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb35b8 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xb35dc | `[sp-20] != 0x7` | `[sp-20] == 0x7` |
| `bne` at 0xb361c | `? != 0x16` | `? == 0x16` |
| `bhi` at 0xb3648 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb3654 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhi` at 0xb36b8 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xb3718 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb3738 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb375c | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bhi` at 0xb37a0 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb37c0 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xb37f0 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xb3820 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xb3850 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xb38a8 | `? != 0x1b` | `? == 0x1b` |
| `bhi` at 0xb38d4 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb38e0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhi` at 0xb3944 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xb39a4 | `ret(0xb3b48) > [sp-32] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xb39c4 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xb39e8 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3a10 | `r0` | `?` |

## `TRIGSetInXNFIRST`

- `TRIGSetInXNFIRST`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb23f4 to 0xb29b0.
  Established. A literal word read addresses `0xb29ac` and supplies `0x11ba4c` (`ldr` at 0xb2414).
  A word assignment initializes `sp -0x24` to `0x80` (`str` at 0xb2560).
  The later unnamed `0xb3b48` call carries `0x8`, `0x12`, `[sp]` and `[sp+4]` (`bl` at 0xb2954);
  its unsigned result greater than `[sp]` takes `0xb299c`, while a result less than or equal falls through (`bhi` at 0xb296c).
  The meaning of the indexed input selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb2404 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb2408 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb240c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb2410 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb2414 | load | 4 | `abs 0xb29ac` | `-` |
| `str` at 0xb2418 | store | 4 | `sp -0x28` | `0x11ba4c` |
| `str` at 0xb2420 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xb2424 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb242c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2434 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2450 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb2458 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb246c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb2478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2480 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2488 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb24a4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xb24ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb24b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb24c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb24cc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb24d0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb24dc | store | 4 | `sp -0x2c` | `ret(0xb3b48)` |
| `ldr` at 0xb24e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb24e4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb24f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb24f8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb2500 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb2504 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb250c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2514 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2520 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2528 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2530 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb253c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb254c | store | 4 | `sp -0x30` | `ret(0xb0840)` |
| `ldr` at 0xb2550 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb2560 | store | 4 | `sp -0x24` | `0x80` |
| `ldr` at 0xb2564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb256c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2574 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2590 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb2598 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb25b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb25bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb25c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb25e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb25e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb25ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb25f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb25f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2600 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb261c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb2624 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2640 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2650 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb266c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2674 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb2678 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb267c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2684 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb268c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb26a8 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb26b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb26cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb26d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb26dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb26f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2700 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb2704 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb2708 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2710 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2718 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2734 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb273c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2760 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2768 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2784 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb278c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb2790 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb2794 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb279c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb27a4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb27c0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb27c8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb27e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb27ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb27f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2810 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2818 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb281c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb2820 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2828 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2830 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb284c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb2854 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2870 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2878 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb289c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb28a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb28a8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb28ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb28b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb28bc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb28d8 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb28e0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb28fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb290c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2928 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2930 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb2934 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb2938 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb294c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2950 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb295c | store | 4 | `sp -0x2c` | `ret(0xb3b48)` |
| `ldr` at 0xb2960 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb2964 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2970 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb2978 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb2980 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb2984 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb2988 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb298c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb2998 | store | 4 | `sp -0x30` | `ret(0xb0840)` |
| `ldr` at 0xb299c | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xb29a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb29a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb29a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb24d4 | `(unknown callee) 0xb3b48` | `r0` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb2544 | `(unknown callee) 0xb0840` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xb2954 | `(unknown callee) 0xb3b48` | `0x8` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb2990 | `(unknown callee) 0xb0840` | `[sp-52]` | `[sp-36]` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb2474 | `? != r0` | `? == r0` |
| `bne` at 0xb24c0 | `? != r1` | `? == r1` |
| `bhi` at 0xb24ec | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `beq` at 0xb2558 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb25b0 | `? != 0xa` | `? == 0xa` |
| `bne` at 0xb263c | `? != 0xb` | `? == 0xb` |
| `bne` at 0xb26c8 | `? != 0xc` | `? == 0xc` |
| `bne` at 0xb2754 | `? != 0xd` | `? == 0xd` |
| `bne` at 0xb27e0 | `? != 0xe` | `? == 0xe` |
| `bne` at 0xb286c | `? != 0xf` | `? == 0xf` |
| `bne` at 0xb28f8 | `? != 0x9` | `? == 0x9` |
| `bhi` at 0xb2940 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xb296c | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb29a8 | `r0` | `?` |

## `TRIGSetInFIRST`

- `TRIGSetInFIRST`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb17ec to 0xb23f4.
  Established. The signed comparison of an unknown value less than or equal to `0x1b` takes `0xb23e4`, while greater falls through (`ble` at 0xb1960).
  An unnamed `0xb3b48` call carries `0x4`, `0x12`, `[sp]` and `[sp+4]` (`bl` at 0xb1998).
  Separate unnamed `0xb0840` calls carry `[sp-44]` with candidate pairs `0x0`/`0x0` and `0x8`/`0x8`, each followed by an unknown fourth candidate (`bl` at 0xb19f8, `bl` at 0xb22f8).
  The hardware role of these per-case checks remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb17fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb1800 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb1804 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb1808 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb1810 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xb1814 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb181c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1824 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1840 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1848 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb185c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1868 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1870 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1878 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1894 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xb189c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb18a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb18b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb18bc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb18c0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb18cc | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb18d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb18d4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb18e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb18e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb18f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb18f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb18fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1904 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1910 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb192c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb193c | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1940 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1948 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1950 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb196c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1974 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1990 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1994 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb19a0 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb19a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb19a8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb19b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb19bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb19c4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb19c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb19d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb19d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb19ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1a00 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1a04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1a0c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1a14 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1a30 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1a38 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1a54 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1a58 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb1a64 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb1a68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1a6c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1a78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1a84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1a8c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb1a94 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1a98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1aa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1ab4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1ac4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1ad0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1ae0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1ae8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1af0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1af8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1b0c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1b20 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1b24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1b2c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1b34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1b50 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1b58 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1b74 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1b78 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb1b84 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb1b88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1b8c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1b98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1ba4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1bac | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb1bb4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1bb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1bc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1bc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1be4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1bf0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1c00 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1c08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1c10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1c18 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1c2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1c40 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1c44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1c4c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1c54 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1c70 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1c78 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1c94 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1c98 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb1ca4 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb1ca8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1cac | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1cb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1cc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1ccc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb1cd4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1cd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1ce0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1ce8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1cf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1cfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1d04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1d10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1d20 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1d28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1d30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1d38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1d4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1d60 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1d6c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1d74 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1d90 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1d98 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1db4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1db8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb1dc4 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb1dc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1dcc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1dd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1de4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1dec | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb1df4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1df8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1e00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1e08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1e14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1e1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1e24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1e30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1e40 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1e50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1e58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1e6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1e80 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1e84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1e8c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1e94 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1eb0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1eb8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1ed4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1ed8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb1ee4 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb1ee8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1eec | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1ef8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1f04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1f0c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb1f14 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1f18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1f34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1f44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1f50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1f60 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1f68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1f70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1f78 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1f8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1fa0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb1fa4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1fac | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb1fb4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1fd0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb1fd8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb1ff4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb1ff8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2004 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb2008 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb200c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2018 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb202c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb2034 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2038 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2048 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2054 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb205c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2064 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2070 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2080 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb2088 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2090 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb2098 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb20ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb20c0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb20c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb20cc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb20d4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb20f0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb20f8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2114 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2118 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2124 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb2128 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb212c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2138 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2144 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb214c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb2154 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2158 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2168 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2174 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb217c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2184 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2190 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb21a0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb21a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb21b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb21b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb21cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb21e0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb21e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb21ec | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb21f4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2210 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb2218 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2234 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2238 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2244 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb2248 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb224c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2258 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2264 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb226c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb2274 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2278 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2280 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2288 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2294 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb229c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb22a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb22b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb22c0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb22c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb22d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb22d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb22ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2300 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb2304 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb230c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb2314 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2330 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xb2338 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb2354 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2358 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb2364 | store | 4 | `sp -0x24` | `ret(0xb3b48)` |
| `ldr` at 0xb2368 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb236c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xb2378 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2384 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb238c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb2394 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb2398 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb23a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb23a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb23b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb23bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb23c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb23d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb23e0 | store | 4 | `sp -0x28` | `ret(0xb0840)` |
| `ldr` at 0xb23e4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xb23f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb23f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb23f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb18c4 | `(unknown callee) 0xb3b48` | `r0` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb1934 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb1998 | `(unknown callee) 0xb3b48` | `0x4` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb19f8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x0` | `0x0` | `?` |
| `bl` at 0xb1a5c | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb1ad8 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb1b18 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x1` | `0x1` | `?` |
| `bl` at 0xb1b7c | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb1bf8 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb1c38 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x2` | `0x2` | `?` |
| `bl` at 0xb1c9c | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb1d18 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb1d58 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x3` | `0x3` | `?` |
| `bl` at 0xb1dbc | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb1e38 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb1e78 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x4` | `0x4` | `?` |
| `bl` at 0xb1edc | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb1f58 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb1f98 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x5` | `0x5` | `?` |
| `bl` at 0xb1ffc | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb2078 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb20b8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x6` | `0x6` | `?` |
| `bl` at 0xb211c | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb2198 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb21d8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x7` | `0x7` | `?` |
| `bl` at 0xb223c | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb22b8 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb22f8 | `(unknown callee) 0xb0840` | `[sp-44]` | `0x8` | `0x8` | `?` |
| `bl` at 0xb235c | `(unknown callee) 0xb3b48` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xb23d8 | `(unknown callee) 0xb0840` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1864 | `? != r0` | `? == r0` |
| `bne` at 0xb18b0 | `? != r1` | `? == r1` |
| `bhi` at 0xb18dc | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `ble` at 0xb1960 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xb1984 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bhi` at 0xb19b0 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb19e8 | `? != 0x1d` | `? == 0x1d` |
| `bhi` at 0xb1a74 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb1a80 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb1b08 | `? != 0x1e` | `? == 0x1e` |
| `bhi` at 0xb1b94 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb1ba0 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb1c28 | `? != 0x1f` | `? == 0x1f` |
| `bhi` at 0xb1cb4 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb1cc0 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb1d48 | `? != 0x20` | `? == 0x20` |
| `bhi` at 0xb1dd4 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb1de0 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb1e68 | `? != 0x21` | `? == 0x21` |
| `bhi` at 0xb1ef4 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb1f00 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb1f88 | `? != 0x22` | `? == 0x22` |
| `bhi` at 0xb2014 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb2020 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb20a8 | `? != 0x23` | `? == 0x23` |
| `bhi` at 0xb2134 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb2140 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb21c8 | `? != 0x24` | `? == 0x24` |
| `bhi` at 0xb2254 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb2260 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xb22e8 | `? != 0x1c` | `? == 0x1c` |
| `bhi` at 0xb2374 | `ret(0xb3b48) > [sp] (unsigned, 32-bit)` | `ret(0xb3b48) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xb2380 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb23f0 | `r0` | `?` |

## `TRIGTimeStamp`

- `TRIGTimeStamp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0b84 to 0xb17ec.
  Established. An unnamed `0xb3b48` call carries `0x7`, `0x22`, entry `r2` and entry `r3` (`bl` at 0xb0c24).
  The subsequent unnamed `0xb17ec` call carries `0x7`, `0x11`, `[sp-20]` and `[sp-24]` (`bl` at 0xb0c58);
  its nonzero result takes `0xb16f4`, while zero falls through (`bne` at 0xb0c64).
  A later signed byte read addresses `[sp-36] +0x0`, and a signed comparison of an unknown value below zero takes `0xb1734`, while nonnegative falls through (`ldrsb` at 0xb171c, `blt` at 0xb1724).
  Timestamp units and the hardware role remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0b94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb0b98 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb0b9c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb0ba0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb0ba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0bac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0bb4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0bc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0bd0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0bd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0bec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0bf4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0bfc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0c20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0c2c | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb0c30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0c34 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0c38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0c3c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0c40 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0c44 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0c50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0c54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0c68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0c70 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0c78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0c80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0c88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0ca8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0cb4 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb0cb8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0cbc | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0cc0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0cc4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0cc8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0ccc | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0cd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0cdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0cf0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0cf8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0d00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0d08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0d10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0d24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0d2c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0d34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0d48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0d50 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0d58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0d6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0d74 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0d7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0d9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0da0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0dac | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb0db0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0db4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0db8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0dbc | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0dc0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0dc4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0dd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0de8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0df0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0df4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0df8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0dfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0e00 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0e04 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0e08 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0e14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0e2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0e34 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0e38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0e3c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0e40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0e44 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0e48 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0e4c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0e5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0e70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0e78 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0e7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0e80 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0e84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0e88 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0e8c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0e90 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0eb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0ebc | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0ec0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0ec4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0ec8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0ecc | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0ed0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0ed4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0ee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0ee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0ef8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f00 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0f04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f08 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0f0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0f10 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0f14 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0f18 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0f28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0f3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f44 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0f48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f4c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0f50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0f54 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0f58 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0f5c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0f6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0f80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f88 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0f8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0f90 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb0f94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0f98 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb0f9c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb0fa0 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb0fac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0fb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0fc4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0fcc | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb0fd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0fdc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0fe4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0ff8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1000 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1008 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb101c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1024 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb102c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1048 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1050 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1074 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1080 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1084 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1088 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb108c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1090 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb1094 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1098 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb10a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb10a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb10bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb10c4 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb10cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb10d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb10dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb10f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb10fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1108 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1110 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1118 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1120 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb113c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1140 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb114c | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb115c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1164 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1180 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1184 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1190 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb11a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb11a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb11c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb11c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb11d4 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb11dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb11e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb11ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1208 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb120c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1218 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1228 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1230 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb124c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1250 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb125c | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1264 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb126c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1274 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1290 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1294 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb12a0 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb12a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb12b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb12b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb12d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb12d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb12e4 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb12ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb12f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb12fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb131c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1328 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1338 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1340 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb135c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1360 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb136c | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1370 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1374 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb1378 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb137c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb1380 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1384 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb1390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb13a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb13b0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb13b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb13c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb13c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb13e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb13e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb13f4 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb13fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1404 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb140c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1428 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb142c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1438 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb143c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1440 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb1444 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1448 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb144c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1450 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb145c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1474 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb147c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb1480 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1484 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb1488 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb148c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb1490 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1494 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb14a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb14a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb14b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb14c0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb14c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb14c8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb14cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb14d0 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb14d4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb14d8 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb14e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb14e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb14fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1504 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb150c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1514 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb151c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1538 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb153c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1548 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb154c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1550 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb1554 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1558 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb155c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1560 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb156c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1584 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb158c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb1594 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb159c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb15a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb15c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb15c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb15d0 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb15d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb15e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb15e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1608 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1614 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1618 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb161c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb1620 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1624 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb1628 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb162c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb1638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb163c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1650 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1658 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb165c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1660 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb1664 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1668 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb166c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1670 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb167c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1694 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb169c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb16a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb16a4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb16a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb16ac | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb16b0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb16b4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb16c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb16c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb16d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb16e0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb16e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb16f0 | store | 4 | `sp -0x28` | `r2+1` |
| `ldr` at 0xb16f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb16f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1704 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb170c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb1714 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb1718 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xb171c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xb1728 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1730 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb1734 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb173c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1744 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1760 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1768 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1784 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1788 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1794 | store | 4 | `sp -0x28` | `ret(0xb3b48)` |
| `ldr` at 0xb1798 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb179c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb17a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb17a4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xb17a8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb17ac | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xb17b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb17bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb17d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb17d8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xb17dc | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xb17e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb17e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb17e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0c24 | `(unknown callee) 0xb3b48` | `0x7` | `0x22` | `r2` | `r3` |
| `bl` at 0xb0c58 | `(unknown callee) 0xb17ec` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0cac | `(unknown callee) 0xb3b48` | `0x6` | `0x22` | `r2` | `r3` |
| `bl` at 0xb0ce0 | `(unknown callee) 0xb17ec` | `0x6` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0da4 | `(unknown callee) 0xb3b48` | `0x8` | `0x22` | `r2` | `r3` |
| `bl` at 0xb0dd8 | `(unknown callee) 0xb17ec` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0e1c | `(unknown callee) 0xb23f4` | `0xa` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0e60 | `(unknown callee) 0xb23f4` | `0xb` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0ea4 | `(unknown callee) 0xb23f4` | `0xc` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0ee8 | `(unknown callee) 0xb23f4` | `0xd` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0f2c | `(unknown callee) 0xb23f4` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0f70 | `(unknown callee) 0xb23f4` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb0fb4 | `(unknown callee) 0xb23f4` | `0x9` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb1078 | `(unknown callee) 0xb3b48` | `0x5` | `0x22` | `r2` | `r3` |
| `bl` at 0xb10ac | `(unknown callee) 0xb17ec` | `0x5` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb1100 | `(unknown callee) 0xb3b48` | `0x10` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1144 | `(unknown callee) 0xb3b48` | `0x14` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1188 | `(unknown callee) 0xb3b48` | `0x16` | `0x22` | `r2` | `r3` |
| `bl` at 0xb11cc | `(unknown callee) 0xb3b48` | `0x18` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1210 | `(unknown callee) 0xb3b48` | `0x1a` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1254 | `(unknown callee) 0xb3b48` | `0x15` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1298 | `(unknown callee) 0xb3b48` | `0x17` | `0x22` | `r2` | `r3` |
| `bl` at 0xb12dc | `(unknown callee) 0xb3b48` | `0x19` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1320 | `(unknown callee) 0xb3b48` | `0x1b` | `0x22` | `r2` | `r3` |
| `bl` at 0xb1364 | `(unknown callee) 0xb3b48` | `0x7` | `0x23` | `r2` | `r3` |
| `bl` at 0xb1398 | `(unknown callee) 0xb17ec` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb13ec | `(unknown callee) 0xb3b48` | `0x6` | `0x23` | `r2` | `r3` |
| `bl` at 0xb1430 | `(unknown callee) 0xb3b48` | `0x8` | `0x23` | `r2` | `r3` |
| `bl` at 0xb1464 | `(unknown callee) 0xb17ec` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb14a8 | `(unknown callee) 0xb23f4` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb14ec | `(unknown callee) 0xb23f4` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb1540 | `(unknown callee) 0xb3b48` | `0x7` | `0x24` | `r2` | `r3` |
| `bl` at 0xb1574 | `(unknown callee) 0xb17ec` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb15c8 | `(unknown callee) 0xb3b48` | `0x6` | `0x24` | `r2` | `r3` |
| `bl` at 0xb160c | `(unknown callee) 0xb3b48` | `0x8` | `0x24` | `r2` | `r3` |
| `bl` at 0xb1640 | `(unknown callee) 0xb17ec` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb1684 | `(unknown callee) 0xb23f4` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb16c8 | `(unknown callee) 0xb23f4` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xb178c | `(unknown callee) 0xb3b48` | `0x4` | `0x22` | `[sp-28]` | `[sp-32]` |
| `bl` at 0xb17c0 | `(unknown callee) 0xb17ec` | `0x4` | `0x11` | `[sp-20]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb0bc4 | `? == 0xc` | `? != 0xc` |
| `beq` at 0xb0be8 | `? == 0xb` | `? != 0xb` |
| `beq` at 0xb0c0c | `? == 0x9` | `? != 0x9` |
| `bne` at 0xb0c64 | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb0c98 | `? != 0x7` | `? == 0x7` |
| `bne` at 0xb0cec | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `beq` at 0xb0d20 | `? == 0x17` | `? != 0x17` |
| `beq` at 0xb0d44 | `? == 0x19` | `? != 0x19` |
| `beq` at 0xb0d68 | `? == 0x1a` | `? != 0x1a` |
| `beq` at 0xb0d8c | `? == 0x18` | `? != 0x18` |
| `bne` at 0xb0de4 | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb0e28 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb0e6c | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb0eb0 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb0ef4 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb0f38 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb0f7c | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb0fc0 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `beq` at 0xb0ff4 | `? == 0x2` | `? != 0x2` |
| `beq` at 0xb1018 | `? == 0x3` | `? != 0x3` |
| `beq` at 0xb103c | `? == 0x4` | `? != 0x4` |
| `beq` at 0xb1060 | `? == 0x5` | `? != 0x5` |
| `bne` at 0xb10b8 | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb10ec | `? != 0x1` | `? == 0x1` |
| `bne` at 0xb1130 | `? != 0xe` | `? == 0xe` |
| `bne` at 0xb1174 | `? != 0x10` | `? == 0x10` |
| `bne` at 0xb11b8 | `? != 0x12` | `? == 0x12` |
| `bne` at 0xb11fc | `? != 0x14` | `? == 0x14` |
| `bne` at 0xb1240 | `? != 0xf` | `? == 0xf` |
| `bne` at 0xb1284 | `? != 0x11` | `? == 0x11` |
| `bne` at 0xb12c8 | `? != 0x13` | `? == 0x13` |
| `bne` at 0xb130c | `? != 0x15` | `? == 0x15` |
| `bne` at 0xb1350 | `? != 0xa` | `? == 0xa` |
| `bne` at 0xb13a4 | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb13d8 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xb141c | `? != 0x16` | `? == 0x16` |
| `bne` at 0xb1470 | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb14b4 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb14f8 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb152c | `? != 0xd` | `? == 0xd` |
| `bne` at 0xb1580 | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb15b4 | `? != 0x8` | `? == 0x8` |
| `bne` at 0xb15f8 | `? != 0x1b` | `? == 0x1b` |
| `bne` at 0xb164c | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |
| `bne` at 0xb1690 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bne` at 0xb16d4 | `ret(0xb23f4) != 0x0` | `ret(0xb23f4) == 0x0` |
| `bhi` at 0xb1700 | `[sp-40] > [sp-28] (unsigned, 32-bit)` | `[sp-40] <= [sp-28] (unsigned, 32-bit)` |
| `blt` at 0xb1724 | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `ble` at 0xb1754 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xb1778 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bne` at 0xb17cc | `ret(0xb17ec) != 0x0` | `ret(0xb17ec) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb17e8 | `r0` | `[sp-40]` |

## `TRIGSyndToBit`

- `TRIGSyndToBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3a14 to 0xb3a8c.
  Established. A word assignment writes `0x7f` to `sp -0x18` (`str` at 0xb3a2c).
  The indexed word read has unknown effective target (`ldr` at 0xb3a4c).
  The branch takes `0xb3a68` when `Z == 0`, with the complement falling through (`bne` at 0xb3a58).
  Syndrome table encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3a24 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3a2c | store | 4 | `sp -0x18` | `0x7f` |
| `str` at 0xb3a34 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb3a38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3a44 | load | 4 | `abs 0xb3a88` | `-` |
| `ldr` at 0xb3a48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3a4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb3a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3a5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3a60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb3a68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3a70 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb3a78 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb3a84 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3a84 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3a84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xb3a40 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xb3a58 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3a84 | `r0` | `?` |

## `TRIGBitToSide`

- `TRIGBitToSide`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3a8c to 0xb3b48.
  Established. Unsigned entry `r0 <= 0xb` takes `0xb3ac0`, while greater falls through (`bls` at 0xb3ab0).
  The later unsigned condition `r0 <= 0xf` takes `0xb3b30`, while greater falls through (`bls` at 0xb3abc).
  A word assignment writes `0x1` to `sp -0x18` (`str` at 0xb3b34).
  Trigger side ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3a9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3aa4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb3aa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3ab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3ac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3acc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3ad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3ae4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3b08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3b14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3b20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb3b34 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xb3b38 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb3b44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3b44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3b44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb3ab0 | `r0 <= 0xb (unsigned, 32-bit)` | `r0 > 0xb (unsigned, 32-bit)` |
| `bls` at 0xb3abc | `r0 <= 0xf (unsigned, 32-bit)` | `r0 > 0xf (unsigned, 32-bit)` |
| `bls` at 0xb3ac8 | `r0 <= 0x17 (unsigned, 32-bit)` | `r0 > 0x17 (unsigned, 32-bit)` |
| `bls` at 0xb3ad4 | `r0 <= 0x19 (unsigned, 32-bit)` | `r0 > 0x19 (unsigned, 32-bit)` |
| `bls` at 0xb3ae0 | `r0 <= 0x1d (unsigned, 32-bit)` | `r0 > 0x1d (unsigned, 32-bit)` |
| `bls` at 0xb3aec | `r0 <= 0x2f (unsigned, 32-bit)` | `r0 > 0x2f (unsigned, 32-bit)` |
| `bls` at 0xb3af8 | `r0 <= 0x37 (unsigned, 32-bit)` | `r0 > 0x37 (unsigned, 32-bit)` |
| `bls` at 0xb3b04 | `r0 <= 0x39 (unsigned, 32-bit)` | `r0 > 0x39 (unsigned, 32-bit)` |
| `bls` at 0xb3b10 | `r0 <= 0x3d (unsigned, 32-bit)` | `r0 > 0x3d (unsigned, 32-bit)` |
| `bls` at 0xb3b1c | `r0 <= 0x3f (unsigned, 32-bit)` | `r0 > 0x3f (unsigned, 32-bit)` |
| `bhi` at 0xb3b28 | `r0 > 0x3f (unsigned, 32-bit)` | `r0 <= 0x3f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3b44 | `r0` | `?` |

## `TRIGFindAddrIndex`

- `TRIGFindAddrIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3b48 to 0xb3c24.
  Established. A word assignment writes `r2+1` to `sp -0x28` (`str` at 0xb3b70).
  Included halfword and word reads have unknown effective targets (`ldrh` at 0xb3ba8, `ldr` at 0xb3be4).
  The return record has an unknown integer candidate (`ldm` at 0xb3c20).
  Trigger address record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3b58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3b5c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb3b60 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb3b64 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb3b68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3b70 | store | 4 | `sp -0x28` | `r2+1` |
| `str` at 0xb3b78 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb3b7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3b80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3b8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3ba0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb3ba8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb3bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3bc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3bdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb3be4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb3bf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3bfc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3c00 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb3c04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3c0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb3c14 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xb3c20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb3c20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb3c20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xb3b88 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xb3bc4 | `? != r0` | `? == r0` |
| `bne` at 0xb3bf8 | `? != r1` | `? == r1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb3c20 | `r0` | `?` |

## `get_data`

- `get_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0840 to 0xb09cc.
  Established. The unnamed `0x19748` call carries `0x133f14`, `0x0`, `0x8`, entry `r3` (`bl` at 0xb0868).
  The signed condition `[sp-32] < [sp-28]` takes `0xb097c`, while greater or equal falls through (`blt` at 0xb0890).
  Included byte reads address `0x133f18` and `0x133f1b` (`ldrb` at 0xb0980, `ldrb` at 0xb09ac).
  Trigger data bit numbering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0850 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb0854 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb0858 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb085c | load | 4 | `abs 0xb09c8` | `-` |
| `ldr` at 0xb086c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0870 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xb0874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0878 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0880 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb0884 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0894 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb08a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb08b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb08b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb08dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb08fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb0900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb090c | load | 4 | `abs 0xb09c8` | `-` |
| `ldr` at 0xb0910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0928 | load | 4 | `abs 0xb09c8` | `-` |
| `ldr` at 0xb0930 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb0954 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xb095c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb0960 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0968 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xb096c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb0974 | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldr` at 0xb097c | load | 4 | `abs 0xb09c8` | `-` |
| `ldrb` at 0xb0980 | load | 1 | `abs 0x133f18` | `-` |
| `ldr` at 0xb0988 | load | 4 | `abs 0xb09c8` | `-` |
| `ldrb` at 0xb098c | load | 1 | `abs 0x133f19` | `-` |
| `ldr` at 0xb0998 | load | 4 | `abs 0xb09c8` | `-` |
| `ldrb` at 0xb099c | load | 1 | `abs 0x133f1a` | `-` |
| `ldr` at 0xb09a8 | load | 4 | `abs 0xb09c8` | `-` |
| `ldrb` at 0xb09ac | load | 1 | `abs 0x133f1b` | `-` |
| `str` at 0xb09b4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb09b8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xb09c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb09c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb09c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0868 | `(unknown callee) 0x19748` | `0x133f14` | `0x0` | `0x8` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb0890 | `[sp-32] < [sp-28] (signed, 32-bit)` | `[sp-32] >= [sp-28] (signed, 32-bit)` |
| `beq` at 0xb0908 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb09c4 | `r0` | `[sp-40]` |

## `print_data`

- `print_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb09cc to 0xb0b84.
  Established. The unnamed `0x197a8` call carries `0x133f1c`, `0x11e3cc`, `[sp-36]:1`, `[sp-35]:1` (`bl` at 0xb0b68).
  Included byte reads address `sp -0x24` and `sp -0x1d` (`ldrb` at 0xb0b24, `ldrb` at 0xb0b54).
  The integer return record carries `0x133f1c` (`ldm` at 0xb0b78).
  Printed trigger data interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb09dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb09e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb09e4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb09fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0a00 | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0xb0a04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0a08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb0a10 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb0a14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0a24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a38 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb0a40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb0a44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb0a8c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb0a90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0a9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0ad8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb0afc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xb0b04 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb0b08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb0b10 | store | 4 | `sp -0x28` | `[sp-40]-1` |
| `ldr` at 0xb0b14 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb0b1c | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldrb` at 0xb0b24 | load | 1 | `sp -0x24` | `-` |
| `ldrb` at 0xb0b28 | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0xb0b2c | load | 1 | `sp -0x22` | `-` |
| `str` at 0xb0b30 | store | 4 | `sp -0x48` | `[sp-34]:1` |
| `ldrb` at 0xb0b34 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xb0b38 | store | 4 | `sp -0x44` | `[sp-33]:1` |
| `ldrb` at 0xb0b3c | load | 1 | `sp -0x20` | `-` |
| `str` at 0xb0b40 | store | 4 | `sp -0x40` | `[sp-32]:1` |
| `ldrb` at 0xb0b44 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xb0b48 | store | 4 | `sp -0x3c` | `[sp-31]:1` |
| `ldrb` at 0xb0b4c | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xb0b50 | store | 4 | `sp -0x38` | `[sp-30]:1` |
| `ldrb` at 0xb0b54 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xb0b58 | store | 4 | `sp -0x34` | `[sp-29]:1` |
| `ldr` at 0xb0b5c | load | 4 | `abs 0xb0b7c` | `-` |
| `ldr` at 0xb0b60 | load | 4 | `abs 0xb0b80` | `-` |
| `ldr` at 0xb0b6c | load | 4 | `abs 0xb0b7c` | `-` |
| `ldm` at 0xb0b78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb0b78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb0b78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb09f8 | `(unknown callee) 0x19748` | `sp-36` | `0x0` | `0x8` | `sp-36` |
| `bl` at 0xb0b68 | `(unknown callee) 0x197a8` | `0x133f1c` | `0x11e3cc` | `[sp-36]:1` | `[sp-35]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb0a20 | `[sp-40] < [sp-28] (signed, 32-bit)` | `[sp-40] >= [sp-28] (signed, 32-bit)` |
| `beq` at 0xb0a98 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb0b78 | `r0` | `0x133f1c` |

## `IRLogFrame`

- `IRLogFrame`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab5e4 to 0xab6f8.
  Established. A nonzero entry `r1` takes `0xab61c`, while zero falls through (`bne` at 0xab604).
  Included halfword reads address `r1 +0x10` and `r1 +0x22` (`ldrh` at 0xab630, `ldrh` at 0xab6bc).
  The unnamed `0x197a8` call carries `sp-152`, `0x1141b8`, entry `r0`, entry `r1` (`bl` at 0xab6d4).
  Logged frame field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab5f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab5f8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xab5fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab608 | load | 4 | `abs 0xab6ec` | `-` |
| `ldr` at 0xab610 | load | 4 | `abs 0xab6f0` | `-` |
| `ldr` at 0xab620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab624 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xab628 | store | 4 | `sp -0xcc` | `[r1]` |
| `ldr` at 0xab62c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xab630 | load | 2 | `r1 +0x10` | `-` |
| `str` at 0xab634 | store | 4 | `sp -0xc8` | `[r1+16]:2` |
| `ldr` at 0xab638 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab63c | load | 1 | `r1 +0x12` | `-` |
| `str` at 0xab640 | store | 4 | `sp -0xc4` | `[r1+18]:1` |
| `ldr` at 0xab644 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab648 | load | 1 | `r1 +0x13` | `-` |
| `str` at 0xab64c | store | 4 | `sp -0xc0` | `[r1+19]:1` |
| `ldr` at 0xab650 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab654 | load | 1 | `r1 +0x14` | `-` |
| `str` at 0xab658 | store | 4 | `sp -0xbc` | `[r1+20]:1` |
| `ldr` at 0xab65c | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0xab660 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xab668 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xab66c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab670 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xab678 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xab67c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab680 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xab684 | store | 4 | `sp -0xb0` | `[r1+22]:1` |
| `ldr` at 0xab688 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab68c | load | 4 | `r1 +0x18` | `-` |
| `str` at 0xab690 | store | 4 | `sp -0xac` | `[r1+24]` |
| `ldr` at 0xab694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab698 | load | 4 | `r1 +0x1c` | `-` |
| `str` at 0xab69c | store | 4 | `sp -0xa8` | `[r1+28]` |
| `ldr` at 0xab6a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab6a4 | load | 1 | `r1 +0x20` | `-` |
| `str` at 0xab6a8 | store | 4 | `sp -0xa4` | `[r1+32]:1` |
| `ldr` at 0xab6ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab6b0 | load | 1 | `r1 +0x21` | `-` |
| `str` at 0xab6b4 | store | 4 | `sp -0xa0` | `[r1+33]:1` |
| `ldr` at 0xab6b8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xab6bc | load | 2 | `r1 +0x22` | `-` |
| `str` at 0xab6c0 | store | 4 | `sp -0x9c` | `[r1+34]:2` |
| `ldr` at 0xab6c8 | load | 4 | `abs 0xab6f4` | `-` |
| `ldr` at 0xab6cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab6d0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xab6e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab6e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab6e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab60c | `(unknown callee) 0x1e4ac` | `0x114118` | `r1` | `r2` | `r1` |
| `bl` at 0xab614 | `(unknown callee) 0x1e4ac` | `0x114168` | `?` | `?` | `?` |
| `bl` at 0xab6d4 | `(unknown callee) 0x197a8` | `sp-152` | `0x1141b8` | `r0` | `r1` |
| `bl` at 0xab6e0 | `(unknown callee) 0x1e4ac` | `sp-152` | `?` | `?` | `sp-152` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xab604 | `r1 != 0x0` | `r1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab6e8 | `r0` | `ret(0x1e4ac)` |

## `IRFmtPkt`

- `IRFmtPkt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab514 to 0xab5e4.
  Established. Included word reads address `r1 +0x18` and `[sp-24] +0x1c` (`ldr` at 0xab538, `ldr` at 0xab54c).
  The unnamed `0x197a8` call carries `[sp-28]`, `0x1140f0`, `[sp-20]`, unknown (`bl` at 0xab5cc).
  Packet formatting record ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab524 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab528 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xab52c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xab530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab538 | load | 4 | `r1 +0x18` | `-` |
| `ldr` at 0xab544 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab54c | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xab558 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab55c | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xab56c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab570 | load | 1 | `[sp-24] +0x14` | `-` |
| `ldr` at 0xab574 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab578 | load | 1 | `[sp-24] +0x21` | `-` |
| `str` at 0xab584 | store | 4 | `sp -0x74` | `ret(0xab1a0)` |
| `ldr` at 0xab588 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab58c | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0xab590 | store | 4 | `sp -0x70` | `[[sp-24]+32]:1` |
| `ldr` at 0xab594 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab598 | load | 1 | `[sp-24] +0x13` | `-` |
| `str` at 0xab59c | store | 4 | `sp -0x6c` | `[[sp-24]+19]:1` |
| `ldr` at 0xab5a0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xab5a4 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0xab5a8 | store | 4 | `sp -0x68` | `[[sp-24]+34]:2` |
| `str` at 0xab5b0 | store | 4 | `sp -0x64` | `sp-60` |
| `str` at 0xab5b8 | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0xab5bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab5c0 | load | 4 | `abs 0xab5e0` | `-` |
| `ldr` at 0xab5c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab5d0 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xab5dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab5dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab5dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab540 | `(unknown callee) 0xab23c` | `[r1+24]` | `sp-60` | `sp-60` | `r1` |
| `bl` at 0xab554 | `(unknown callee) 0xab23c` | `[[sp-24]+28]` | `sp-92` | `sp-92` | `[sp-24]` |
| `bl` at 0xab564 | `(unknown callee) 0xab1a0` | `[[sp-24]+22]:1` | `?` | `?` | `[[sp-24]+22]:1` |
| `bl` at 0xab5cc | `(unknown callee) 0x197a8` | `[sp-28]` | `0x1140f0` | `[sp-20]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab5dc | `r0` | `[sp-28]` |

## `IRPrintFrame`

- `IRPrintFrame`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab6f8 to 0xab814.
  Established. A nonzero entry `r1` takes `0xab740`, while zero falls through (`bne` at 0xab718).
  The unnamed `0x222f0` call carries `[0x12eeb8]`, `0x1142a8`, entry `r0`, entry `r1` (`bl` at 0xab7f8).
  Included byte reads address `r1 +0x20` and `r1 +0x21` (`ldrb` at 0xab7c8, `ldrb` at 0xab7d4).
  Frame output destination semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab708 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab70c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xab710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab71c | load | 4 | `abs 0xab804` | `-` |
| `ldr` at 0xab720 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xab724 | load | 4 | `abs 0xab808` | `-` |
| `ldr` at 0xab72c | load | 4 | `abs 0xab804` | `-` |
| `ldr` at 0xab730 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xab734 | load | 4 | `abs 0xab80c` | `-` |
| `ldr` at 0xab740 | load | 4 | `abs 0xab804` | `-` |
| `ldr` at 0xab744 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab748 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xab74c | store | 4 | `sp -0xcc` | `[r1]` |
| `ldr` at 0xab750 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xab754 | load | 2 | `r1 +0x10` | `-` |
| `str` at 0xab758 | store | 4 | `sp -0xc8` | `[r1+16]:2` |
| `ldr` at 0xab75c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab760 | load | 1 | `r1 +0x12` | `-` |
| `str` at 0xab764 | store | 4 | `sp -0xc4` | `[r1+18]:1` |
| `ldr` at 0xab768 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab76c | load | 1 | `r1 +0x13` | `-` |
| `str` at 0xab770 | store | 4 | `sp -0xc0` | `[r1+19]:1` |
| `ldr` at 0xab774 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab778 | load | 1 | `r1 +0x14` | `-` |
| `str` at 0xab77c | store | 4 | `sp -0xbc` | `[r1+20]:1` |
| `ldr` at 0xab780 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0xab784 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xab78c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xab790 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab794 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xab79c | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xab7a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab7a4 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xab7a8 | store | 4 | `sp -0xb0` | `[r1+22]:1` |
| `ldr` at 0xab7ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab7b0 | load | 4 | `r1 +0x18` | `-` |
| `str` at 0xab7b4 | store | 4 | `sp -0xac` | `[r1+24]` |
| `ldr` at 0xab7b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab7bc | load | 4 | `r1 +0x1c` | `-` |
| `str` at 0xab7c0 | store | 4 | `sp -0xa8` | `[r1+28]` |
| `ldr` at 0xab7c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab7c8 | load | 1 | `r1 +0x20` | `-` |
| `str` at 0xab7cc | store | 4 | `sp -0xa4` | `[r1+32]:1` |
| `ldr` at 0xab7d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xab7d4 | load | 1 | `r1 +0x21` | `-` |
| `str` at 0xab7d8 | store | 4 | `sp -0xa0` | `[r1+33]:1` |
| `ldr` at 0xab7dc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xab7e0 | load | 2 | `r1 +0x22` | `-` |
| `str` at 0xab7e4 | store | 4 | `sp -0x9c` | `[r1+34]:2` |
| `ldr` at 0xab7e8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xab7ec | load | 4 | `abs 0xab810` | `-` |
| `ldr` at 0xab7f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab7f4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xab800 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab800 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab800 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab728 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x114208` | `r2` | `0x12eeb8` |
| `bl` at 0xab738 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x114258` | `?` | `0x12eeb8` |
| `bl` at 0xab7f8 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x1142a8` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xab718 | `r1 != 0x0` | `r1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab800 | `r0` | `ret(0x222f0)` |

## `IRPktSize`

- `IRPktSize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab870 to 0xab8cc.
  Established. A word assignment writes `0x0` to `sp -0x18` (`str` at 0xab888).
  The loop includes a halfword read and a word read whose effective targets are unknown (`ldrh` at 0xab89c, `ldr` at 0xab8b0).
  The conditional branch takes `0xab8bc` when `Z == 1`, with the complement falling through (`beq` at 0xab894).
  Packet size units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab880 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab888 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xab88c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab898 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xab89c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xab8a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab8a8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xab8ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab8b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xab8b4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xab8bc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xab8c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab8c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab8c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab894 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab8c8 | `r0` | `?` |
