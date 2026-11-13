# ARM request, packet and console paths in 1.40.5

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
L1ReqIRouter and SMPCmdCheckBrickType have unresolved computed transfers. Their access, call and return lists cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/request-packet-console-facts.txt) retain all tool sections.

## `L1ReqIRouter`

- `L1ReqIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x88b50 to 0x8db1c.
  Established. The word comparison takes `[sp-680] == 0x1006` to `0x8b0a4` and falls through
  for unequal values (`beq` at 0x88cfc). An included four-byte assignment at `[sp-1296] +0xc`
  retains `ret(0x84054)` (`str` at 0x8b24c). The unnamed `0xb74d4` call carries `0x2b2594`, `0x0`,
  `[sp-24]`, `[sp-668]` (`bl` at 0x8da4c). The reported return is `ret(0xb5fec)` (`ldmdb` at
  0x8dad8). Request semantics beyond the known edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x88b64 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x88b6c | store | 4 | `sp -0x29c` | `0x0` |
| `str` at 0x88b74 | store | 4 | `sp -0x2a0` | `0x0` |
| `str` at 0x88b84 | store | 4 | `sp -0x2a4` | `?` |
| `ldr` at 0x88b88 | load | 4 | `abs 0x89b58` | `-` |
| `str` at 0x88b98 | store | 4 | `sp -0x2a0` | `ret(0x22ac8)` |
| `ldr` at 0x88b9c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x88ba8 | load | 4 | `abs 0x89b5c` | `-` |
| `str` at 0x88bac | store | 4 | `sp -0x5dc` | `0x112c38` |
| `ldr` at 0x88bb0 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x88bb4 | store | 4 | `sp -0x5d8` | `ret(0x22ac8)` |
| `ldr` at 0x88bbc | load | 4 | `abs 0x89b60` | `-` |
| `ldr` at 0x88bc0 | load | 4 | `abs 0x89b64` | `-` |
| `ldr` at 0x88bcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x88bd0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x88bf8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x88bfc | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x88c0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88c10 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x88c14 | store | 4 | `sp -0x4b0` | `[[sp-24]+40]` |
| `ldr` at 0x88c1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x88c28 | store | 4 | `sp -0x4b0` | `ret(0xb823c)` |
| `ldr` at 0x88c2c | load | 4 | `sp -0x4b0` | `-` |
| `str` at 0x88c30 | store | 4 | `sp -0x2a8` | `[sp-1200]` |
| `str` at 0x88c38 | store | 4 | `sp -0x4b4` | `?` |
| `ldr` at 0x88c3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88c40 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x88c4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88c50 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x88c58 | store | 4 | `sp -0x4b8` | `?` |
| `ldr` at 0x88c5c | load | 4 | `sp -0x4b8` | `-` |
| `str` at 0x88c64 | store | 4 | `sp -0x4b8` | `?` |
| `str` at 0x88c70 | store | 4 | `sp -0x4b8` | `?` |
| `ldr` at 0x88c74 | load | 4 | `sp -0x4b8` | `-` |
| `ldr` at 0x88c84 | load | 4 | `sp -0x4b4` | `-` |
| `strb` at 0x88c88 | store | 1 | `(sp-20 + [sp-1204]) +0x0` | `(ret(0x83fe4) & 0xff)` |
| `ldrb` at 0x88c94 | load | 1 | `(unknown)` | `-` |
| `str` at 0x88ca8 | store | 4 | `sp -0x2b0` | `ret(0x5f314)` |
| `ldr` at 0x88cac | load | 4 | `abs 0x89b68` | `-` |
| `ldr` at 0x88cb0 | load | 4 | `abs 0x149294` | `-` |
| `ldr` at 0x88cbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88cc0 | load | 4 | `abs 0x89b6c` | `-` |
| `ldr` at 0x88cc4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x88cc8 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x88ce8 | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0x88cec | store | 4 | `sp -0x5cc` | `[sp-680]` |
| `ldr` at 0x88cf0 | load | 4 | `abs 0x89b70` | `-` |
| `ldr` at 0x88cf4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d00 | load | 4 | `abs 0x89b70` | `-` |
| `ldr` at 0x88d04 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d10 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d1c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d28 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d34 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d40 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d4c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d58 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d68 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d74 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d84 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d90 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88d9c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88da8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88db8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88dc4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88dd4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88de0 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88dec | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88df8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e04 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e10 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e20 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e2c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e3c | load | 4 | `abs 0x89b74` | `-` |
| `ldr` at 0x88e40 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e4c | load | 4 | `abs 0x89b74` | `-` |
| `ldr` at 0x88e50 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e5c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e68 | load | 4 | `abs 0x89b78` | `-` |
| `ldr` at 0x88e6c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e7c | load | 4 | `abs 0x89b7c` | `-` |
| `ldr` at 0x88e80 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88e90 | load | 4 | `abs 0x89b80` | `-` |
| `ldr` at 0x88e94 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88ea0 | load | 4 | `abs 0x89b80` | `-` |
| `ldr` at 0x88ea4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88eb0 | load | 4 | `abs 0x89b84` | `-` |
| `ldr` at 0x88eb4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88ec0 | load | 4 | `abs 0x89b84` | `-` |
| `ldr` at 0x88ec4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88ed0 | load | 4 | `abs 0x89b88` | `-` |
| `ldr` at 0x88ed4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88ee0 | load | 4 | `abs 0x89b88` | `-` |
| `ldr` at 0x88ee4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88ef0 | load | 4 | `abs 0x89b8c` | `-` |
| `ldr` at 0x88ef4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f00 | load | 4 | `abs 0x89b90` | `-` |
| `ldr` at 0x88f04 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f14 | load | 4 | `abs 0x89b94` | `-` |
| `ldr` at 0x88f18 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f24 | load | 4 | `abs 0x89b98` | `-` |
| `ldr` at 0x88f28 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f38 | load | 4 | `abs 0x89b9c` | `-` |
| `ldr` at 0x88f3c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f48 | load | 4 | `abs 0x89b9c` | `-` |
| `ldr` at 0x88f4c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f58 | load | 4 | `abs 0x89ba0` | `-` |
| `ldr` at 0x88f5c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f68 | load | 4 | `abs 0x89ba4` | `-` |
| `ldr` at 0x88f6c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f7c | load | 4 | `abs 0x89ba8` | `-` |
| `ldr` at 0x88f80 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88f8c | load | 4 | `abs 0x89bac` | `-` |
| `ldr` at 0x88f90 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88fa0 | load | 4 | `abs 0x89bb0` | `-` |
| `ldr` at 0x88fa4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88fb0 | load | 4 | `abs 0x89bb0` | `-` |
| `ldr` at 0x88fb4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88fc0 | load | 4 | `abs 0x89bb4` | `-` |
| `ldr` at 0x88fc4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88fd0 | load | 4 | `abs 0x89bb4` | `-` |
| `ldr` at 0x88fd4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88fe0 | load | 4 | `abs 0x89bb8` | `-` |
| `ldr` at 0x88fe4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88ff0 | load | 4 | `abs 0x89bbc` | `-` |
| `ldr` at 0x88ff4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89004 | load | 4 | `abs 0x89bc0` | `-` |
| `ldr` at 0x89008 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89014 | load | 4 | `abs 0x89bc4` | `-` |
| `ldr` at 0x89018 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89028 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89034 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89040 | load | 4 | `abs 0x89bc8` | `-` |
| `ldr` at 0x89044 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89050 | load | 4 | `abs 0x89bcc` | `-` |
| `ldr` at 0x89054 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89064 | load | 4 | `abs 0x89bd0` | `-` |
| `ldr` at 0x89068 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89074 | load | 4 | `abs 0x89bd0` | `-` |
| `ldr` at 0x89078 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x89084 | load | 4 | `abs 0x89bd4` | `-` |
| `ldr` at 0x89088 | load | 4 | `sp -0x5cc` | `-` |
| `str` at 0x890a4 | store | 4 | `sp -0x2b8` | `ret(0x1f26c)` |
| `ldr` at 0x890d0 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x890fc | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x89128 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x89130 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x8913c | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x89150 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0x89154 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x89158 | store | 4 | `sp -0x2c4` | `[sp-676]` |
| `ldr` at 0x8915c | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x89160 | store | 4 | `sp -0x4bc` | `[sp-676]` |
| `ldr` at 0x89164 | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x89170 | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x8917c | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x8918c | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x8919c | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0x891ac | store | 4 | `sp -0x2c4` | `0x23` |
| `str` at 0x891b8 | store | 4 | `sp -0x2c4` | `0x2c` |
| `ldrb` at 0x891c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x891c8 | load | 4 | `abs 0x89bd8` | `-` |
| `ldr` at 0x891d0 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0x891e8 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0x8920c | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x89220 | store | 4 | `sp -0x2c0` | `?` |
| `str` at 0x89230 | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x89234 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89238 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x89240 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89244 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x8924c | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x89250 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89254 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `ldr` at 0x89260 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x89270 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89274 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `str` at 0x89278 | store | 4 | `sp -0x4c0` | `[ret(0x75194)+16]` |
| `str` at 0x89284 | store | 4 | `sp -0x4c0` | `0x23` |
| `ldr` at 0x89288 | load | 4 | `sp -0x4c0` | `-` |
| `str` at 0x8928c | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x892a0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x892b8 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x892cc | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x892d0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x892d4 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x892dc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x892e0 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x892e8 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x892ec | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x892f0 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `ldr` at 0x892fc | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x8930c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89310 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `str` at 0x89314 | store | 4 | `sp -0x4c4` | `[ret(0x75194)+16]` |
| `str` at 0x89320 | store | 4 | `sp -0x4c4` | `0x23` |
| `ldr` at 0x89324 | load | 4 | `sp -0x4c4` | `-` |
| `str` at 0x89328 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x8933c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x89354 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x89368 | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x8936c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89370 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x89378 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8937c | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x89384 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x89388 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8938c | load | 4 | `ret(0x75194) +0x10` | `-` |
| `ldr` at 0x89398 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x893a8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x893ac | load | 4 | `ret(0x75194) +0x10` | `-` |
| `str` at 0x893b0 | store | 4 | `sp -0x4c8` | `[ret(0x75194)+16]` |
| `str` at 0x893bc | store | 4 | `sp -0x4c8` | `0x23` |
| `ldr` at 0x893c0 | load | 4 | `sp -0x4c8` | `-` |
| `str` at 0x893c4 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x893d8 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x893f0 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x89404 | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x89408 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8940c | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x89414 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89418 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x89420 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x89424 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89428 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `ldr` at 0x89434 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x89444 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89448 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `str` at 0x8944c | store | 4 | `sp -0x4cc` | `[ret(0x75194)+16]` |
| `str` at 0x89458 | store | 4 | `sp -0x4cc` | `0x23` |
| `ldr` at 0x8945c | load | 4 | `sp -0x4cc` | `-` |
| `str` at 0x89460 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x89474 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8948c | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x894bc | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x894c0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x894c4 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x894cc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x894d0 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x894d8 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x894ec | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x894fc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89508 | load | 4 | `[sp-692] +0x10` | `-` |
| `str` at 0x8951c | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x89520 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89524 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x8952c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89530 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x89538 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x8954c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8955c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x89568 | load | 4 | `[sp-692] +0x10` | `-` |
| `ldr` at 0x89574 | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0x89588 | load | 1 | `(unknown)` | `-` |
| `str` at 0x89594 | store | 4 | `sp -0x2bc` | `ret(0x4f588)` |
| `str` at 0x895a0 | store | 4 | `sp -0x2bc` | `ret(0x4f730)` |
| `ldr` at 0x895a4 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x895e4 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x89600 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x89608 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x89634 | load | 4 | `abs 0x89bdc` | `-` |
| `str` at 0x89644 | store | 4 | `sp -0x2b8` | `ret(0x1b4c0)` |
| `ldr` at 0x89658 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x89660 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x8967c | store | 4 | `sp -0x2b8` | `ret(0x1b4c0)` |
| `ldr` at 0x89690 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x89698 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x896b0 | store | 4 | `sp -0x2b8` | `ret(0x1b4c0)` |
| `ldr` at 0x896c4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x896d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x896d4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x896fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89700 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x89710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89714 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x89718 | store | 4 | `sp -0x4d0` | `[[sp-24]+48]` |
| `ldr` at 0x89720 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8972c | store | 4 | `sp -0x4d0` | `ret(0xb823c)` |
| `ldr` at 0x89730 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89734 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x89740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89744 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x8974c | store | 4 | `sp -0x4d4` | `?` |
| `ldr` at 0x89750 | load | 4 | `sp -0x4d4` | `-` |
| `str` at 0x89758 | store | 4 | `sp -0x4d4` | `?` |
| `str` at 0x89764 | store | 4 | `sp -0x4d4` | `?` |
| `ldr` at 0x89768 | load | 4 | `sp -0x4d4` | `-` |
| `ldr` at 0x8976c | load | 4 | `sp -0x4d0` | `-` |
| `str` at 0x89778 | store | 4 | `sp -0x2cc` | `ret(0x880d4)` |
| `str` at 0x89790 | store | 4 | `sp -0x2d0` | `ret(0xd2790)` |
| `ldr` at 0x89794 | load | 4 | `abs 0x89be4` | `-` |
| `ldrb` at 0x8979c | load | 1 | `sp -0x2cc` | `-` |
| `ldr` at 0x897b8 | load | 4 | `abs 0x89be0` | `-` |
| `ldr` at 0x897dc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x897e0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x89828 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8983c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x8984c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89854 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89868 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89870 | load | 4 | `(unknown)` | `-` |
| `str` at 0x89874 | store | 4 | `sp -0x4d8` | `?` |
| `ldr` at 0x89878 | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x89884 | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x8989c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x898b0 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x898c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x898d0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x898e4 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x898f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89904 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89918 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89928 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89950 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89964 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89974 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89984 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89998 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x899a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x899b8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x899cc | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x899dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x899ec | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89a00 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89a10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89a24 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89a38 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89a48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89a58 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89a6c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89a7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89a88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89a8c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x89ab4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89ab8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x89ac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89acc | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x89ad0 | store | 4 | `sp -0x4dc` | `[[sp-24]+48]` |
| `ldr` at 0x89ad8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89ae4 | store | 4 | `sp -0x4dc` | `ret(0xb823c)` |
| `ldr` at 0x89ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89aec | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x89af8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89afc | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x89b04 | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x89b08 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x89b10 | store | 4 | `sp -0x4e0` | `?` |
| `str` at 0x89b1c | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x89b20 | load | 4 | `sp -0x4e0` | `-` |
| `ldr` at 0x89b24 | load | 4 | `sp -0x4dc` | `-` |
| `str` at 0x89b30 | store | 4 | `sp -0x2cc` | `ret(0x880d4)` |
| `ldr` at 0x89b34 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89b38 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x89b48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89b4c | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x89b50 | store | 4 | `sp -0x4e4` | `[[sp-24]+56]` |
| `ldr` at 0x89bfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89c08 | store | 4 | `sp -0x4e4` | `ret(0xb823c)` |
| `ldr` at 0x89c0c | load | 4 | `sp -0x4e4` | `-` |
| `str` at 0x89c10 | store | 4 | `sp -0x2d4` | `[sp-1252]` |
| `ldr` at 0x89c14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89c18 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x89c28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89c2c | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x89c30 | store | 4 | `sp -0x4e8` | `[[sp-24]+64]` |
| `ldr` at 0x89c38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89c44 | store | 4 | `sp -0x4e8` | `ret(0xb823c)` |
| `ldr` at 0x89c48 | load | 4 | `sp -0x4e8` | `-` |
| `str` at 0x89c4c | store | 4 | `sp -0x2d8` | `[sp-1256]` |
| `ldr` at 0x89c50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89c54 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x89c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89c68 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x89c6c | store | 4 | `sp -0x4ec` | `[[sp-24]+72]` |
| `ldr` at 0x89c74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89c80 | store | 4 | `sp -0x4ec` | `ret(0xb823c)` |
| `ldr` at 0x89c84 | load | 4 | `sp -0x4ec` | `-` |
| `str` at 0x89c88 | store | 4 | `sp -0x2dc` | `[sp-1260]` |
| `str` at 0x89c90 | store | 4 | `sp -0x2e0` | `0x0` |
| `str` at 0x89c98 | store | 4 | `sp -0x2e4` | `0x0` |
| `str` at 0x89cb0 | store | 4 | `sp -0x2e8` | `ret(0xd2790)` |
| `ldr` at 0x89cb4 | load | 4 | `abs 0x89be4` | `-` |
| `ldrb` at 0x89cbc | load | 1 | `sp -0x2cc` | `-` |
| `ldr` at 0x89cf4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89cf8 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x89d20 | load | 4 | `sp -0x2d4` | `-` |
| `ldrls` at 0x89d28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8a164 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a170 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a174 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x8a19c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a1a0 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a1a8 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x8a1b4 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x8a1b8 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8a1c0 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0x8a1c8 | store | 4 | `sp -0x2e4` | `0x0` |
| `ldr` at 0x8a1d0 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a1d4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a1dc | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x8a1e4 | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0x8a210 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x8a218 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a22c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a230 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a244 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a250 | load | 4 | `abs 0x89bf4` | `-` |
| `ldr` at 0x8a254 | load | 4 | `abs 0x2af38c` | `-` |
| `ldr` at 0x8a260 | load | 4 | `abs 0x89bf4` | `-` |
| `ldr` at 0x8a264 | load | 4 | `abs 0x2af38c` | `-` |
| `str` at 0x8a278 | store | 4 | `sp -0x2bc` | `ret(unknown)` |
| `str` at 0x8a288 | store | 4 | `sp -0x2bc` | `ret(0x1f26c)` |
| `ldr` at 0x8a28c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x8a2d4 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x8a2f0 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8a2f8 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x8a314 | store | 4 | `sp -0x2b8` | `ret(0x1f26c)` |
| `ldr` at 0x8a340 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8a358 | store | 4 | `sp -0x2b8` | `ret(0x1f26c)` |
| `ldr` at 0x8a384 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8a3a8 | load | 4 | `sp -0x2a4` | `-` |
| `ldrls` at 0x8a3b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8a60c | store | 4 | `sp -0x2b8` | `ret(0x1f26c)` |
| `ldr` at 0x8a620 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8a634 | store | 4 | `sp -0x2b8` | `ret(0x1f26c)` |
| `ldr` at 0x8a648 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8a654 | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0x8a688 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x8a6a8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x8a6b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8a6c0 | load | 4 | `abs 0x89bec` | `-` |
| `ldrb` at 0x8a6fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8a710 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x8a714 | store | 4 | `sp -0x4f0` | `[sp-676]` |
| `ldr` at 0x8a718 | load | 4 | `sp -0x4f0` | `-` |
| `ldr` at 0x8a724 | load | 4 | `sp -0x4f0` | `-` |
| `ldrb` at 0x8a75c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x8a794 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x8a7a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8a7ac | load | 4 | `abs 0x89bf0` | `-` |
| `ldrb` at 0x8a7e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8a854 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8a89c | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x8a8b4 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x8a8c8 | store | 4 | `sp -0x2dc` | `ret(0x5e0a8)` |
| `ldr` at 0x8a8cc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a8f4 | load | 4 | `abs 0x89bf4` | `-` |
| `ldr` at 0x8a8f8 | load | 4 | `abs 0x2af334` | `-` |
| `str` at 0x8a8fc | store | 4 | `sp -0x4f4` | `[0x2af334]` |
| `ldr` at 0x8a900 | load | 4 | `abs 0x89bf4` | `-` |
| `ldr` at 0x8a904 | load | 4 | `abs 0x2af334` | `-` |
| `str` at 0x8a914 | store | 4 | `sp -0x4f4` | `0x1` |
| `ldr` at 0x8a918 | load | 4 | `sp -0x4f4` | `-` |
| `str` at 0x8a91c | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x8a920 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x8a934 | store | 4 | `sp -0x2d0` | `ret(0x1b1b4)` |
| `ldr` at 0x8a938 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8a940 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8a950 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x8a964 | store | 4 | `sp -0x2cc` | `ret(0x1b1b4)` |
| `ldr` at 0x8a968 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8a970 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x8a984 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x8a988 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a98c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a99c | load | 4 | `abs 0x89bf4` | `-` |
| `ldr` at 0x8a9a4 | load | 4 | `abs 0x2af30c` | `-` |
| `str` at 0x8a9ac | store | 4 | `sp -0x2f0` | `?` |
| `ldr` at 0x8a9b0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a9b4 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0x8a9c0 | store | 4 | `sp -0x2f4` | `ret(0xfb0a8)` |
| `ldr` at 0x8a9c4 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a9cc | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0x8a9d8 | load | 4 | `abs 0x89bf4` | `-` |
| `ldr` at 0x8a9dc | load | 4 | `abs 0x2af30c` | `-` |
| `str` at 0x8a9e4 | store | 4 | `sp -0x2f8` | `?` |
| `ldr` at 0x8a9e8 | load | 4 | `abs 0x89bf8` | `-` |
| `ldrb` at 0x8a9f0 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8aa08 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8aa10 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8aa18 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8aa20 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8aa2c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8aa34 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8aa3c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8aa40 | load | 4 | `abs 0x89bf8` | `-` |
| `ldrb` at 0x8aa48 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8aa60 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8aa68 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8aa70 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8aa78 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8aa84 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8aa8c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8aa94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8aa98 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8aaa0 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x8aad0 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8aae8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8aaf4 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x8aaf8 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8aafc | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8ab14 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8ab1c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8ab2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ab3c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8ab44 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8ab54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ab5c | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8ab64 | store | 4 | `sp -0x2d8` | `[sp-728]+1` |
| `ldr` at 0x8ab6c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8ab74 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x8ab84 | store | 4 | `sp -0x2f8` | `sp-868` |
| `str` at 0x8abb0 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x8abb4 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8abc4 | load | 4 | `sp -0x318` | `-` |
| `str` at 0x8abd0 | store | 4 | `sp -0x318` | `?` |
| `ldr` at 0x8abd4 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x8abdc | store | 4 | `sp -0x310` | `?` |
| `ldr` at 0x8abe0 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x8abe8 | store | 4 | `sp -0x310` | `?` |
| `str` at 0x8abf0 | store | 4 | `sp -0x30c` | `0xc5` |
| `ldr` at 0x8abf4 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8ac00 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8ac04 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac08 | load | 4 | `abs 0x8bbac` | `-` |
| `ldr` at 0x8ac14 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac1c | load | 4 | `abs 0x8bbac` | `-` |
| `str` at 0x8ac28 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8ac2c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac30 | load | 4 | `abs 0x8bbb0` | `-` |
| `ldr` at 0x8ac3c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac44 | load | 4 | `abs 0x8bbb0` | `-` |
| `str` at 0x8ac50 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8ac54 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac58 | load | 4 | `abs 0x8bbb4` | `-` |
| `ldr` at 0x8ac64 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac6c | load | 4 | `abs 0x8bbb4` | `-` |
| `str` at 0x8ac78 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8ac7c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ac8c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8aca0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8aca4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8acb4 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8acc8 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8accc | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8acd0 | store | 4 | `sp -0x308` | `[sp-732]` |
| `str` at 0x8ace0 | store | 4 | `sp -0x2a0` | `ret(0xe3dcc)` |
| `ldr` at 0x8ace4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8acf0 | load | 4 | `sp -0x300` | `-` |
| `str` at 0x8acf4 | store | 4 | `sp -0x324` | `[sp-768]` |
| `ldr` at 0x8acf8 | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0x8acfc | store | 4 | `sp -0x320` | `[sp-764]` |
| `ldr` at 0x8ad00 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x8ad04 | store | 4 | `sp -0x31c` | `[sp-760]` |
| `str` at 0x8ad0c | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x8ad10 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ad20 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ad2c | load | 4 | `abs 0x8bbb8` | `-` |
| `ldr` at 0x8ad40 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ad50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ad64 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ad6c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8ad74 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8ad7c | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0x8ad8c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x8ad94 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8ad98 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8ada0 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x8add0 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x8addc | store | 4 | `sp -0x2a0` | `0x0` |
| `ldr` at 0x8ade4 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x8adec | store | 4 | `sp -0x2f8` | `[sp-760]+8` |
| `ldr` at 0x8adf0 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8adf8 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x8ae00 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8ae48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ae4c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8ae74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ae78 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8ae88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ae8c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8ae90 | store | 4 | `sp -0x4f8` | `[[sp-24]+48]` |
| `ldr` at 0x8ae98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8aea4 | store | 4 | `sp -0x4f8` | `ret(0xb823c)` |
| `ldr` at 0x8aeac | load | 4 | `sp -0x4f8` | `-` |
| `ldr` at 0x8aed0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8aed4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8aefc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8af00 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x8af0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8af10 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x8af18 | store | 4 | `sp -0x4fc` | `?` |
| `ldr` at 0x8af1c | load | 4 | `sp -0x4fc` | `-` |
| `str` at 0x8af24 | store | 4 | `sp -0x4fc` | `?` |
| `str` at 0x8af30 | store | 4 | `sp -0x4fc` | `0x60` |
| `ldr` at 0x8af34 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8af38 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8af48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8af4c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8af50 | load | 4 | `sp -0x4fc` | `-` |
| `str` at 0x8af58 | store | 4 | `sp -0x500` | `?` |
| `ldr` at 0x8af60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8af70 | load | 4 | `sp -0x4fc` | `-` |
| `str` at 0x8af78 | store | 4 | `sp -0x500` | `?` |
| `ldr` at 0x8af7c | load | 4 | `sp -0x500` | `-` |
| `str` at 0x8afac | store | 4 | `sp -0x2b4` | `ret(0x75194)` |
| `ldr` at 0x8afb0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8afb4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8afb8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8afbc | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x8afc0 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `ldr` at 0x8afc4 | load | 4 | `ret(0x75194) +0x14` | `-` |
| `ldr` at 0x8afe8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8afec | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b014 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b018 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b044 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b048 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b05c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b060 | store | 4 | `sp -0x504` | `[[sp-24]+48]` |
| `ldr` at 0x8b068 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b074 | store | 4 | `sp -0x504` | `ret(0xb823c)` |
| `ldr` at 0x8b07c | load | 4 | `sp -0x504` | `-` |
| `ldr` at 0x8b0a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b0a8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b0d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b0d4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b0e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b0e8 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b0ec | store | 4 | `sp -0x508` | `[[sp-24]+48]` |
| `ldr` at 0x8b0f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b100 | store | 4 | `sp -0x508` | `ret(0xb823c)` |
| `ldr` at 0x8b108 | load | 4 | `sp -0x508` | `-` |
| `ldr` at 0x8b140 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b144 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b16c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b170 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b184 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b188 | store | 4 | `sp -0x50c` | `[[sp-24]+48]` |
| `ldr` at 0x8b190 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b19c | store | 4 | `sp -0x50c` | `ret(0xb823c)` |
| `ldr` at 0x8b1a4 | load | 4 | `sp -0x50c` | `-` |
| `ldr` at 0x8b1c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b1cc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b1f4 | load | 4 | `abs 0x8bbbc` | `-` |
| `str` at 0x8b1f8 | store | 4 | `sp -0x510` | `0x2b24a0` |
| `ldr` at 0x8b1fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b200 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b210 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b214 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b218 | store | 4 | `sp -0x514` | `[[sp-24]+48]` |
| `ldr` at 0x8b220 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b22c | store | 4 | `sp -0x514` | `ret(0xb823c)` |
| `ldrb` at 0x8b238 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8b240 | load | 4 | `sp -0x514` | `-` |
| `ldr` at 0x8b248 | load | 4 | `sp -0x510` | `-` |
| `str` at 0x8b24c | store | 4 | `[sp-1296] +0xc` | `ret(0x84054)` |
| `ldr` at 0x8b250 | load | 4 | `abs 0x8bbbc` | `-` |
| `ldr` at 0x8b258 | load | 4 | `abs 0x2b24ac` | `-` |
| `ldr` at 0x8b27c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b280 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b2a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b2ac | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b2bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b2c0 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b2c8 | store | 4 | `sp -0x528` | `?` |
| `ldr` at 0x8b2d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b2dc | store | 4 | `sp -0x528` | `ret(0xb823c)` |
| `ldr` at 0x8b2e0 | load | 4 | `sp -0x528` | `-` |
| `str` at 0x8b2e8 | store | 4 | `sp -0x528` | `?` |
| `ldr` at 0x8b2ec | load | 4 | `sp -0x528` | `-` |
| `ldr` at 0x8b2f8 | load | 4 | `sp -0x528` | `-` |
| `ldr` at 0x8b304 | load | 4 | `sp -0x528` | `-` |
| `ldr` at 0x8b314 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b318 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b32c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b330 | store | 4 | `sp -0x518` | `[[sp-24]+48]` |
| `ldr` at 0x8b338 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b344 | store | 4 | `sp -0x518` | `ret(0xb823c)` |
| `ldr` at 0x8b348 | load | 4 | `sp -0x518` | `-` |
| `ldr` at 0x8b358 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b35c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b36c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b370 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b374 | store | 4 | `sp -0x51c` | `[[sp-24]+48]` |
| `ldr` at 0x8b37c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b388 | store | 4 | `sp -0x51c` | `ret(0xb823c)` |
| `ldr` at 0x8b38c | load | 4 | `sp -0x51c` | `-` |
| `ldr` at 0x8b398 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b39c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b3ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b3b0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b3b4 | store | 4 | `sp -0x520` | `[[sp-24]+48]` |
| `ldr` at 0x8b3bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b3c8 | store | 4 | `sp -0x520` | `ret(0xb823c)` |
| `ldr` at 0x8b3cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b3d0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b3e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b3e4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b3ec | store | 4 | `sp -0x524` | `?` |
| `ldr` at 0x8b3f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b400 | store | 4 | `sp -0x524` | `ret(0xb823c)` |
| `ldr` at 0x8b404 | load | 4 | `sp -0x524` | `-` |
| `str` at 0x8b40c | store | 4 | `sp -0x524` | `?` |
| `ldr` at 0x8b410 | load | 4 | `sp -0x520` | `-` |
| `ldr` at 0x8b414 | load | 4 | `sp -0x524` | `-` |
| `ldr` at 0x8b460 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b464 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b48c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b490 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b4a4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b4ac | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8b4b0 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8b4b8 | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8b4c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b4cc | store | 4 | `sp -0x530` | `ret(0xb823c)` |
| `ldr` at 0x8b4d0 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8b4d8 | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8b4dc | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8b4e4 | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8b4e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b4ec | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8b4fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b500 | load | 4 | `[sp-24] +0x40` | `-` |
| `ldr` at 0x8b510 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8b518 | store | 4 | `sp -0x52c` | `?` |
| `ldr` at 0x8b520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b53c | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8b544 | store | 4 | `sp -0x52c` | `?` |
| `ldr` at 0x8b548 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b54c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b55c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b560 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8b568 | load | 4 | `sp -0x52c` | `-` |
| `str` at 0x8b570 | store | 4 | `sp -0x534` | `?` |
| `ldr` at 0x8b578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b58c | load | 4 | `sp -0x52c` | `-` |
| `str` at 0x8b594 | store | 4 | `sp -0x534` | `?` |
| `ldr` at 0x8b598 | load | 4 | `sp -0x534` | `-` |
| `str` at 0x8b59c | store | 4 | `sp -0x2b0` | `[sp-1332]` |
| `ldrb` at 0x8b5a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8b5b4 | load | 4 | `sp -0x2b0` | `-` |
| `ldrb` at 0x8b5c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8b5c8 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x8b5d0 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x8b5e0 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x8b5e8 | store | 4 | `sp -0x5dc` | `?` |
| `ldr` at 0x8b5ec | load | 4 | `abs 0x8bbc0` | `-` |
| `ldr` at 0x8b5f8 | load | 4 | `abs 0x8bbbc` | `-` |
| `ldr` at 0x8b5fc | load | 4 | `abs 0x2b24ac` | `-` |
| `ldr` at 0x8b608 | load | 4 | `abs 0x8bbbc` | `-` |
| `ldr` at 0x8b60c | load | 4 | `abs 0x2b24ac` | `-` |
| `ldrb` at 0x8b620 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8b634 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b638 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x8b668 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8b66c | store | 4 | `sp -0x538` | `?` |
| `ldr` at 0x8b670 | load | 4 | `sp -0x538` | `-` |
| `str` at 0x8b678 | store | 4 | `sp -0x538` | `?` |
| `ldr` at 0x8b67c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b680 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b694 | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x8b698 | load | 4 | `sp -0x538` | `-` |
| `str` at 0x8b6a4 | store | 4 | `sp -0x53c` | `?` |
| `ldr` at 0x8b6ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b6bc | load | 4 | `sp -0x538` | `-` |
| `str` at 0x8b6c8 | store | 4 | `sp -0x53c` | `?` |
| `ldr` at 0x8b6cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b6d0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b6e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b6e4 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b6e8 | store | 4 | `sp -0x540` | `[[sp-24]+48]:1` |
| `ldr` at 0x8b6f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b704 | store | 4 | `sp -0x540` | `?` |
| `ldr` at 0x8b708 | load | 4 | `sp -0x53c` | `-` |
| `ldr` at 0x8b70c | load | 4 | `sp -0x540` | `-` |
| `str` at 0x8b720 | store | 4 | `sp -0x2b8` | `ret(0x4e54c)` |
| `ldr` at 0x8b724 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b758 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b764 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b768 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b774 | load | 4 | `abs 0x8bbc4` | `-` |
| `ldr` at 0x8b778 | load | 4 | `abs 0x14a550` | `-` |
| `ldr` at 0x8b7a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b7a4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b7b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b7b8 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b7bc | store | 4 | `sp -0x544` | `[[sp-24]+48]:1` |
| `ldr` at 0x8b7c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b7d8 | store | 4 | `sp -0x544` | `?` |
| `ldr` at 0x8b7dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b7e0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b7f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b7f4 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b7f8 | store | 4 | `sp -0x548` | `[[sp-24]+56]:1` |
| `ldr` at 0x8b800 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b814 | store | 4 | `sp -0x548` | `?` |
| `ldr` at 0x8b818 | load | 4 | `abs 0x8bbc4` | `-` |
| `ldr` at 0x8b81c | load | 4 | `abs 0x14a550` | `-` |
| `ldr` at 0x8b824 | load | 4 | `sp -0x544` | `-` |
| `ldr` at 0x8b828 | load | 4 | `sp -0x548` | `-` |
| `str` at 0x8b834 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x8b838 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b86c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b878 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b87c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b888 | load | 4 | `abs 0x8bbc8` | `-` |
| `ldr` at 0x8b88c | load | 4 | `abs 0x14a554` | `-` |
| `ldr` at 0x8b8b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b8b8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b8c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b8cc | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b8d0 | store | 4 | `sp -0x54c` | `[[sp-24]+48]:1` |
| `ldr` at 0x8b8d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b8ec | store | 4 | `sp -0x54c` | `?` |
| `ldr` at 0x8b8f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b8f4 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b904 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b908 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b90c | store | 4 | `sp -0x550` | `[[sp-24]+56]:1` |
| `ldr` at 0x8b914 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b928 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0x8b92c | load | 4 | `abs 0x8bbc8` | `-` |
| `ldr` at 0x8b930 | load | 4 | `abs 0x14a554` | `-` |
| `ldr` at 0x8b938 | load | 4 | `sp -0x54c` | `-` |
| `ldr` at 0x8b93c | load | 4 | `sp -0x550` | `-` |
| `str` at 0x8b948 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x8b94c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b980 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b98c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b990 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b99c | load | 4 | `abs 0x8bbcc` | `-` |
| `ldr` at 0x8b9a0 | load | 4 | `abs 0x14a558` | `-` |
| `ldr` at 0x8b9c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b9cc | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b9dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b9e0 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b9e4 | store | 4 | `sp -0x554` | `[[sp-24]+48]:1` |
| `ldr` at 0x8b9ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ba00 | store | 4 | `sp -0x554` | `?` |
| `ldr` at 0x8ba04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ba08 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8ba18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ba1c | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x8ba20 | store | 4 | `sp -0x558` | `[[sp-24]+56]:1` |
| `ldr` at 0x8ba28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ba3c | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0x8ba40 | load | 4 | `abs 0x8bbcc` | `-` |
| `ldr` at 0x8ba44 | load | 4 | `abs 0x14a558` | `-` |
| `ldr` at 0x8ba4c | load | 4 | `sp -0x554` | `-` |
| `ldr` at 0x8ba50 | load | 4 | `sp -0x558` | `-` |
| `str` at 0x8ba5c | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x8ba60 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8ba94 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8baa8 | store | 4 | `sp -0x2b8` | `ret(0x3d890)` |
| `ldr` at 0x8baac | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8bae0 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8baf4 | store | 4 | `sp -0x2b8` | `ret(0x3dc5c)` |
| `ldr` at 0x8baf8 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8bb2c | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8bb4c | store | 4 | `sp -0x2f4` | `ret(0xd2790)` |
| `ldr` at 0x8bb50 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8bb94 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x8bb98 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bb9c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8bc44 | load | 4 | `abs 0x8bc08` | `-` |
| `ldrb` at 0x8bc4c | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bc68 | load | 4 | `abs 0x8bbd0` | `-` |
| `ldr` at 0x8bc6c | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8bc90 | store | 4 | `sp -0x2d0` | `(sp-996 + ret(0x1b2bc))` |
| `str` at 0x8bc98 | store | 4 | `sp -0x2d8` | `0x1` |
| `ldr` at 0x8bc9c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bca0 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x8bcac | load | 4 | `abs 0x8bc08` | `-` |
| `ldrb` at 0x8bcb4 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bcd8 | load | 4 | `abs 0x8bbd4` | `-` |
| `ldr` at 0x8bce4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bce8 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x8bcf4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bcf8 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x8bd04 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bd08 | load | 4 | `abs 0x8bbd8` | `-` |
| `ldr` at 0x8bd18 | load | 4 | `abs 0x8bc08` | `-` |
| `ldrb` at 0x8bd20 | load | 1 | `sp -0x2e0` | `-` |
| `str` at 0x8bd40 | store | 4 | `sp -0x2d4` | `ret(0x8db1c)` |
| `ldr` at 0x8bd44 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8bd50 | load | 4 | `abs 0x8bbdc` | `-` |
| `str` at 0x8bd54 | store | 4 | `sp -0x55c` | `0x112d24` |
| `ldr` at 0x8bd5c | load | 4 | `abs 0x8bbe0` | `-` |
| `str` at 0x8bd60 | store | 4 | `sp -0x55c` | `0x112d2c` |
| `ldr` at 0x8bd64 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bd68 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bd6c | load | 4 | `[sp-760] +0x50` | `-` |
| `str` at 0x8bd70 | store | 4 | `sp -0x5dc` | `[[sp-760]+80]` |
| `ldr` at 0x8bd74 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bd78 | load | 4 | `[sp-760] +0x54` | `-` |
| `str` at 0x8bd7c | store | 4 | `sp -0x5d8` | `[[sp-760]+84]` |
| `ldr` at 0x8bd80 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bd84 | load | 4 | `[sp-760] +0x58` | `-` |
| `ldrb` at 0x8bd88 | load | 1 | `[[sp-760]+88] +0x0` | `-` |
| `ldr` at 0x8bd98 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bd9c | load | 4 | `[sp-760] +0x58` | `-` |
| `str` at 0x8bda4 | store | 4 | `sp -0x5d4` | `?` |
| `ldr` at 0x8bda8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bdac | load | 4 | `[sp-760] +0x5c` | `-` |
| `str` at 0x8bdb0 | store | 4 | `sp -0x5d0` | `[[sp-760]+92]` |
| `ldr` at 0x8bdb4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bdb8 | load | 4 | `abs 0x8bbe4` | `-` |
| `ldr` at 0x8bdbc | load | 4 | `sp -0x55c` | `-` |
| `ldr` at 0x8bdc0 | load | 4 | `[sp-760] +0x4c` | `-` |
| `ldr` at 0x8bdcc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bdd0 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x8bddc | load | 4 | `abs 0x8bc08` | `-` |
| `ldrb` at 0x8bde4 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8be08 | load | 4 | `abs 0x8bbe8` | `-` |
| `ldr` at 0x8be14 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8be18 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8be24 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8be28 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8be34 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8be38 | load | 4 | `abs 0x8bbec` | `-` |
| `ldr` at 0x8be48 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8be4c | load | 4 | `abs 0x8bbf0` | `-` |
| `ldr` at 0x8be54 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8be64 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8be6c | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x1b2bc))` |
| `str` at 0x8be74 | store | 4 | `sp -0x2dc` | `0x40` |
| `ldr` at 0x8be78 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8be88 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8be8c | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8be90 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8be98 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8be9c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bea0 | load | 4 | `abs 0x8bbf4` | `-` |
| `ldr` at 0x8beac | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8beb4 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8beb8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8bec0 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `str` at 0x8becc | store | 4 | `sp -0x2dc` | `0x5f` |
| `ldr` at 0x8bed0 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8bee0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bee4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8bee8 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8bef0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8bef4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bef8 | load | 4 | `abs 0x8bbf4` | `-` |
| `ldr` at 0x8bf04 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8bf0c | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8bf10 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8bf18 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x8bf24 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8bf30 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8bf38 | store | 4 | `sp -0x2d0` | `?` |
| `str` at 0x8bf40 | store | 4 | `sp -0x2dc` | `0x49` |
| `ldr` at 0x8bf44 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8bf54 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bf58 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8bf5c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8bf64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8bf80 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bf84 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bf88 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8bf8c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8bf98 | load | 1 | `([[sp-760]+12] + [sp-732]) +0x0` | `-` |
| `strb` at 0x8bf9c | store | 1 | `[sp-720] +0x0` | `([([[sp-760]+12] + [sp-732])]:1 & 0xff)` |
| `str` at 0x8bfa4 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x8bfb0 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8bfbc | store | 1 | `[sp-720] +0x0` | `0x20` |
| `str` at 0x8bfc4 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x8bfc8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8bfd0 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x8bfdc | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8bfe8 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8bff0 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8bff4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bffc | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c004 | load | 1 | `[[sp-760]+12] +0x40` | `-` |
| `ldr` at 0x8c010 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c018 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c020 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x8c038 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c040 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c048 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x8c060 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c068 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c070 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x8c074 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c07c | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c084 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x8c088 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c08c | load | 4 | `abs 0x8bbf8` | `-` |
| `ldr` at 0x8c098 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8c0a0 | store | 4 | `sp -0x2d0` | `[sp-720]+4` |
| `ldr` at 0x8c0ac | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c0b0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c0b8 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c0c4 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `strb` at 0x8c0c8 | store | 1 | `[sp-720] +0x0` | `([[[sp-760]+12]+91]:1 & 0xff)` |
| `str` at 0x8c0d0 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x8c0d8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c0dc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c0e4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c0f0 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `strb` at 0x8c0f4 | store | 1 | `[sp-720] +0x1` | `([[[sp-760]+12]+92]:1 & 0xff)` |
| `str` at 0x8c0fc | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8c104 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8c110 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8c118 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8c11c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c124 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c12c | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x8c138 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c140 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8c148 | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x8c150 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c154 | load | 4 | `abs 0x8bbfc` | `-` |
| `ldr` at 0x8c160 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c170 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8c178 | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x1b2bc))` |
| `ldr` at 0x8c17c | load | 4 | `abs 0x8bc00` | `-` |
| `ldr` at 0x8c180 | load | 4 | `abs 0x14a564` | `-` |
| `ldr` at 0x8c18c | load | 4 | `abs 0x8bc00` | `-` |
| `ldr` at 0x8c190 | load | 4 | `abs 0x14a564` | `-` |
| `ldr` at 0x8c194 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c1a0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c1b0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c1b4 | load | 4 | `abs 0x8bc04` | `-` |
| `ldr` at 0x8c1c0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c1c4 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x8c1d0 | load | 4 | `abs 0x8bc08` | `-` |
| `ldrb` at 0x8c1d8 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c1fc | load | 4 | `abs 0x8bc0c` | `-` |
| `ldr` at 0x8c208 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c20c | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8c218 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c21c | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8c228 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c22c | load | 4 | `abs 0x8bc10` | `-` |
| `ldr` at 0x8c23c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c240 | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x8c24c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c250 | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x8c25c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c260 | load | 4 | `abs 0x8bc14` | `-` |
| `ldr` at 0x8c270 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c274 | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x8c280 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c284 | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x8c290 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c294 | load | 4 | `abs 0x8bc18` | `-` |
| `ldr` at 0x8c2a4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c2a8 | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x8c2b4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c2b8 | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x8c2c4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c2c8 | load | 4 | `abs 0x8bc1c` | `-` |
| `ldr` at 0x8c2d8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c2dc | load | 4 | `abs 0x8bc20` | `-` |
| `ldr` at 0x8c2e4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c2f4 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8c2fc | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x1b2bc))` |
| `str` at 0x8c304 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x8c308 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8c31c | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8c320 | store | 4 | `sp -0x564` | `?` |
| `str` at 0x8c328 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8c32c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c330 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8c334 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8c33c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c354 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c358 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8c35c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8c364 | load | 1 | `([[sp-760]+68] + [sp-732]) +0x0` | `-` |
| `strb` at 0x8c36c | store | 1 | `sp -0x55d` | `([([[sp-760]+68] + [sp-732])]:1 & 0xff)` |
| `strb` at 0x8c37c | store | 1 | `sp -0x55d` | `0x3f` |
| `ldrb` at 0x8c384 | load | 1 | `sp -0x55d` | `-` |
| `ldr` at 0x8c388 | load | 4 | `sp -0x564` | `-` |
| `strb` at 0x8c38c | store | 1 | `[sp-1380] +0x0` | `?` |
| `ldr` at 0x8c390 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8c398 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x8c3a4 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8c3b0 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8c3b8 | store | 4 | `sp -0x2d0` | `?` |
| `str` at 0x8c3c0 | store | 4 | `sp -0x2dc` | `0x7` |
| `ldr` at 0x8c3c4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8c3d4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c3d8 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8c3e0 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8c3ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c3f0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c3f4 | load | 4 | `abs 0x8bc24` | `-` |
| `ldr` at 0x8c400 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8c408 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8c40c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8c414 | store | 4 | `sp -0x2dc` | `[sp-732]-1` |
| `ldr` at 0x8c420 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8c42c | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8c434 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8c438 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c43c | load | 4 | `[sp-760] +0x24` | `-` |
| `ldrb` at 0x8c440 | load | 1 | `[[sp-760]+36] +0x3` | `-` |
| `ldr` at 0x8c444 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c448 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8c44c | load | 4 | `[sp-760] +0x34` | `-` |
| `str` at 0x8c450 | store | 4 | `sp -0x5dc` | `[[sp-760]+52]` |
| `ldr` at 0x8c454 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c458 | load | 4 | `abs 0x8bc28` | `-` |
| `ldr` at 0x8c45c | load | 4 | `[sp-760] +0x28` | `-` |
| `ldr` at 0x8c470 | load | 4 | `abs 0x8bc2c` | `-` |
| `str` at 0x8c47c | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x8c480 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8c4b0 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8c4c4 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8c4cc | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x8c4d0 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x8c4d8 | store | 4 | `sp -0x2f8` | `[sp-760]+120` |
| `ldr` at 0x8c4e0 | load | 4 | `abs 0x8bc30` | `-` |
| `ldr` at 0x8c4e8 | load | 4 | `abs 0x8bc34` | `-` |
| `ldr` at 0x8c4f0 | load | 4 | `abs 0x2b4430` | `-` |
| `ldr` at 0x8c4fc | load | 4 | `abs 0x8bc38` | `-` |
| `ldr` at 0x8c500 | load | 4 | `abs 0x8bc3c` | `-` |
| `ldrb` at 0x8c504 | load | 1 | `abs 0x112e84` | `-` |
| `strb` at 0x8c50c | store | 1 | `sp -0x4ac` | `([0x112e84]:1 & 0xff)` |
| `ldr` at 0x8c528 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c52c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c53c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c540 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8c544 | store | 4 | `sp -0x568` | `[[sp-24]+48]` |
| `ldr` at 0x8c54c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c558 | store | 4 | `sp -0x568` | `ret(0xb823c)` |
| `ldr` at 0x8c55c | load | 4 | `sp -0x568` | `-` |
| `str` at 0x8c560 | store | 4 | `sp -0x2f4` | `[sp-1384]` |
| `ldr` at 0x8c564 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c568 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8c578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c57c | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8c580 | store | 4 | `sp -0x56c` | `[[sp-24]+56]` |
| `ldr` at 0x8c588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c594 | store | 4 | `sp -0x56c` | `ret(0xb823c)` |
| `ldr` at 0x8c598 | load | 4 | `sp -0x56c` | `-` |
| `str` at 0x8c59c | store | 4 | `sp -0x2dc` | `[sp-1388]` |
| `ldr` at 0x8c5a0 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8c5a4 | store | 4 | `sp -0x580` | `[sp-756]` |
| `ldr` at 0x8c5a8 | load | 4 | `sp -0x580` | `-` |
| `ldr` at 0x8c5b4 | load | 4 | `sp -0x580` | `-` |
| `ldr` at 0x8c5c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c5c8 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8c5d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c5dc | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x8c5e0 | store | 4 | `sp -0x570` | `[[sp-24]+64]` |
| `ldr` at 0x8c5e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c5f4 | store | 4 | `sp -0x570` | `ret(0xb823c)` |
| `ldr` at 0x8c5f8 | load | 4 | `sp -0x570` | `-` |
| `str` at 0x8c5fc | store | 4 | `sp -0x2d8` | `[sp-1392]` |
| `ldr` at 0x8c600 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8c618 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8c62c | store | 4 | `sp -0x2e0` | `ret(0x39a88)` |
| `ldr` at 0x8c630 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c654 | load | 4 | `abs 0x8bc3c` | `-` |
| `ldrb` at 0x8c65c | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8c694 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8c6d4 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c6dc | load | 4 | `abs 0x8bc3c` | `-` |
| `ldrb` at 0x8c6e4 | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8c71c | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8c734 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c738 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8c748 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c74c | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x8c750 | store | 4 | `sp -0x574` | `[[sp-24]+64]` |
| `ldr` at 0x8c758 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c764 | store | 4 | `sp -0x574` | `ret(0xb823c)` |
| `ldr` at 0x8c768 | load | 4 | `sp -0x574` | `-` |
| `str` at 0x8c76c | store | 4 | `sp -0x2d4` | `[sp-1396]` |
| `ldr` at 0x8c770 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c774 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x8c784 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c788 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x8c78c | store | 4 | `sp -0x578` | `[[sp-24]+72]` |
| `ldr` at 0x8c794 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c7a0 | store | 4 | `sp -0x578` | `ret(0xb823c)` |
| `ldr` at 0x8c7a4 | load | 4 | `sp -0x578` | `-` |
| `str` at 0x8c7a8 | store | 4 | `sp -0x2d0` | `[sp-1400]` |
| `ldr` at 0x8c7ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c7b0 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x8c7c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c7c4 | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x8c7c8 | store | 4 | `sp -0x57c` | `[[sp-24]+80]` |
| `ldr` at 0x8c7d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c7dc | store | 4 | `sp -0x57c` | `ret(0xb823c)` |
| `ldr` at 0x8c7e0 | load | 4 | `sp -0x57c` | `-` |
| `str` at 0x8c7e4 | store | 4 | `sp -0x2cc` | `[sp-1404]` |
| `ldr` at 0x8c7e8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8c7f8 | store | 4 | `sp -0x5dc` | `0x0` |
| `str` at 0x8c804 | store | 4 | `sp -0x5d8` | `sp-1196` |
| `ldr` at 0x8c80c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8c810 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8c814 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x8c820 | store | 4 | `sp -0x2e0` | `ret(0x39c80)` |
| `ldr` at 0x8c824 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c848 | load | 4 | `abs 0x8bc3c` | `-` |
| `ldrb` at 0x8c850 | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8c888 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8c8c8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c8d0 | load | 4 | `abs 0x8bc3c` | `-` |
| `ldrb` at 0x8c8d8 | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8c910 | store | 4 | `sp -0x5dc` | `0x1` |
| `str` at 0x8c934 | store | 4 | `sp -0x2b8` | `ret(0x1f26c)` |
| `ldr` at 0x8c974 | load | 4 | `abs 0x8bc40` | `-` |
| `ldr` at 0x8c9a4 | load | 4 | `abs 0x8bc40` | `-` |
| `ldr` at 0x8c9c0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8c9cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c9d0 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8c9fc | store | 4 | `sp -0x584` | `ret(0xdb74c)` |
| `ldr` at 0x8ca00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ca04 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8ca14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ca18 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8ca1c | load | 4 | `sp -0x584` | `-` |
| `ldr` at 0x8ca2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ca3c | load | 4 | `sp -0x584` | `-` |
| `ldr` at 0x8ca68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ca6c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8ca7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ca80 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8ca84 | store | 4 | `sp -0x588` | `[[sp-24]+48]` |
| `ldr` at 0x8ca8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ca98 | store | 4 | `sp -0x588` | `ret(0xb823c)` |
| `ldr` at 0x8ca9c | load | 4 | `sp -0x588` | `-` |
| `str` at 0x8caa8 | store | 4 | `sp -0x2a0` | `ret(0xdb85c)` |
| `ldr` at 0x8caac | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8cae0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8cb08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cb0c | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8cb38 | store | 4 | `sp -0x58c` | `ret(0xdb74c)` |
| `ldr` at 0x8cb3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cb40 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8cb50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb54 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8cb58 | load | 4 | `sp -0x58c` | `-` |
| `ldr` at 0x8cb68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb78 | load | 4 | `sp -0x58c` | `-` |
| `ldr` at 0x8cba4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cba8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8cbb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cbbc | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8cbc0 | store | 4 | `sp -0x590` | `[[sp-24]+48]` |
| `ldr` at 0x8cbc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cbd4 | store | 4 | `sp -0x590` | `ret(0xb823c)` |
| `ldr` at 0x8cbe0 | load | 4 | `sp -0x590` | `-` |
| `str` at 0x8cbf0 | store | 4 | `sp -0x2a0` | `ret(0xdb914)` |
| `ldr` at 0x8cbf4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8cc28 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8cc68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cc6c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8cc78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cc7c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8cc8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cc90 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8cc94 | store | 4 | `sp -0x594` | `[[sp-24]+48]` |
| `ldr` at 0x8cc9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cca8 | store | 4 | `sp -0x594` | `ret(0xb823c)` |
| `ldr` at 0x8ccac | load | 4 | `sp -0x594` | `-` |
| `str` at 0x8ccb0 | store | 4 | `sp -0x2e0` | `[sp-1428]` |
| `ldr` at 0x8ccb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ccb8 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8cd04 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x8cd08 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8cd0c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8cd1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cd20 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8cd34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8cd44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cd48 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8cd5c | load | 1 | `(unknown)` | `-` |
| `str` at 0x8cd60 | store | 4 | `sp -0x598` | `?` |
| `ldr` at 0x8cd68 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8cd70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cd84 | store | 4 | `sp -0x598` | `?` |
| `ldr` at 0x8cd94 | load | 4 | `sp -0x598` | `-` |
| `str` at 0x8cda4 | store | 4 | `sp -0x2a0` | `ret(0xd7530)` |
| `ldr` at 0x8cda8 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8cddc | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8cdf4 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8ce10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8ce20 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8ce28 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x8ce30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ce34 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8ce5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ce60 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8ce70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ce74 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8ce78 | store | 4 | `sp -0x59c` | `[[sp-24]+48]` |
| `ldr` at 0x8ce80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ce8c | store | 4 | `sp -0x59c` | `ret(0xb823c)` |
| `ldr` at 0x8ce90 | load | 4 | `sp -0x59c` | `-` |
| `str` at 0x8ce94 | store | 4 | `sp -0x2e0` | `[sp-1436]` |
| `ldr` at 0x8ce98 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ceac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ceb0 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8cef4 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x8cef8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8cefc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8cf0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cf10 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8cf30 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8cf40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cf44 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8cf64 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8cf68 | store | 4 | `sp -0x5a0` | `?` |
| `ldr` at 0x8cf70 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8cf84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cf98 | store | 4 | `sp -0x5a0` | `?` |
| `ldr` at 0x8cf9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cfa0 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8cfc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8cfd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cfd4 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8cff4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8cff8 | store | 4 | `sp -0x5a4` | `?` |
| `ldr` at 0x8d000 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8d014 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d028 | store | 4 | `sp -0x5a4` | `?` |
| `ldr` at 0x8d02c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d030 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8d050 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d060 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d064 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8d084 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8d088 | store | 4 | `sp -0x5a8` | `?` |
| `ldr` at 0x8d090 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8d0a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d0b8 | store | 4 | `sp -0x5a8` | `?` |
| `ldr` at 0x8d0c0 | load | 4 | `sp -0x5a0` | `-` |
| `ldr` at 0x8d0c4 | load | 4 | `sp -0x5a4` | `-` |
| `ldr` at 0x8d0c8 | load | 4 | `sp -0x5a8` | `-` |
| `str` at 0x8d0d4 | store | 4 | `sp -0x2a0` | `ret(0xd73b0)` |
| `ldr` at 0x8d0d8 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d10c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d124 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8d130 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8d138 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x8d140 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d144 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8d16c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d178 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d184 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d190 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d19c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d1a8 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d1b4 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d1c0 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d1cc | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8d1dc | load | 4 | `abs 0x8dadc` | `-` |
| `ldr` at 0x8d1e0 | load | 4 | `abs 0x14a564` | `-` |
| `ldr` at 0x8d1ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d1f0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8d200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d204 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8d208 | store | 4 | `sp -0x5ac` | `[[sp-24]+48]` |
| `ldr` at 0x8d210 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d21c | store | 4 | `sp -0x5ac` | `ret(0xb823c)` |
| `ldr` at 0x8d220 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d224 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8d234 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d238 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8d23c | store | 4 | `sp -0x5b0` | `[[sp-24]+56]` |
| `ldr` at 0x8d244 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d250 | store | 4 | `sp -0x5b0` | `ret(0xb823c)` |
| `ldr` at 0x8d254 | load | 4 | `abs 0x8dadc` | `-` |
| `ldr` at 0x8d258 | load | 4 | `abs 0x14a564` | `-` |
| `ldr` at 0x8d260 | load | 4 | `sp -0x5ac` | `-` |
| `ldr` at 0x8d264 | load | 4 | `sp -0x5b0` | `-` |
| `str` at 0x8d278 | store | 4 | `sp -0x2f4` | `ret(unknown)` |
| `ldr` at 0x8d27c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8d2b0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8d2e4 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8d330 | store | 4 | `sp -0x2a0` | `ret(0x4e004)` |
| `ldr` at 0x8d344 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d350 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d354 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8d37c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d380 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8d390 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d394 | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x8d398 | load | 4 | `abs 0x8dae0` | `-` |
| `str` at 0x8d3a4 | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0x8d3a8 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8d3b0 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8d3bc | load | 4 | `sp -0x5b4` | `-` |
| `str` at 0x8d3c8 | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0x8d3d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d3dc | store | 4 | `sp -0x5b4` | `ret(0xb823c)` |
| `ldr` at 0x8d3e0 | load | 4 | `abs 0x8dae0` | `-` |
| `ldr` at 0x8d3e4 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8d404 | load | 4 | `sp -0x5b4` | `-` |
| `str` at 0x8d40c | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0x8d410 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d414 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8d424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d428 | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x8d430 | store | 4 | `sp -0x5b8` | `?` |
| `ldr` at 0x8d438 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d444 | store | 4 | `sp -0x5b8` | `ret(0xb823c)` |
| `ldr` at 0x8d448 | load | 4 | `sp -0x5b8` | `-` |
| `str` at 0x8d450 | store | 4 | `sp -0x5b8` | `?` |
| `ldr` at 0x8d454 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d458 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x8d468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d46c | load | 4 | `[sp-24] +0x48` | `-` |
| `ldr` at 0x8d470 | load | 4 | `abs 0x8dae4` | `-` |
| `str` at 0x8d47c | store | 4 | `sp -0x5bc` | `?` |
| `ldr` at 0x8d480 | load | 4 | `sp -0x5bc` | `-` |
| `ldr` at 0x8d488 | load | 4 | `sp -0x5bc` | `-` |
| `str` at 0x8d498 | store | 4 | `sp -0x5bc` | `?` |
| `ldr` at 0x8d4a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d4ac | store | 4 | `sp -0x5bc` | `ret(0xb823c)` |
| `ldr` at 0x8d4b0 | load | 4 | `abs 0x8dae4` | `-` |
| `ldr` at 0x8d4b4 | load | 4 | `sp -0x5bc` | `-` |
| `ldr` at 0x8d4d0 | load | 4 | `sp -0x5bc` | `-` |
| `str` at 0x8d4d8 | store | 4 | `sp -0x5bc` | `?` |
| `ldr` at 0x8d4dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d4e0 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x8d4f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d4f4 | load | 4 | `[sp-24] +0x50` | `-` |
| `ldr` at 0x8d4f8 | load | 4 | `abs 0x8dae8` | `-` |
| `str` at 0x8d504 | store | 4 | `sp -0x5c0` | `?` |
| `ldr` at 0x8d508 | load | 4 | `sp -0x5c0` | `-` |
| `ldr` at 0x8d510 | load | 4 | `sp -0x5c0` | `-` |
| `str` at 0x8d520 | store | 4 | `sp -0x5c0` | `?` |
| `ldr` at 0x8d528 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d534 | store | 4 | `sp -0x5c0` | `ret(0xb823c)` |
| `ldr` at 0x8d538 | load | 4 | `abs 0x8dae8` | `-` |
| `ldr` at 0x8d53c | load | 4 | `sp -0x5c0` | `-` |
| `ldr` at 0x8d558 | load | 4 | `sp -0x5c0` | `-` |
| `str` at 0x8d560 | store | 4 | `sp -0x5c0` | `?` |
| `ldr` at 0x8d564 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d568 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8d578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d57c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8d580 | load | 4 | `abs 0x8daec` | `-` |
| `str` at 0x8d58c | store | 4 | `sp -0x5c4` | `?` |
| `ldr` at 0x8d590 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0x8d598 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0x8d5a4 | load | 4 | `sp -0x5c4` | `-` |
| `str` at 0x8d5bc | store | 4 | `sp -0x5c4` | `?` |
| `ldr` at 0x8d5c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d5d0 | store | 4 | `sp -0x5c4` | `ret(0xb823c)` |
| `ldr` at 0x8d5d4 | load | 4 | `abs 0x8daec` | `-` |
| `ldr` at 0x8d5d8 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0x8d604 | load | 4 | `sp -0x5c4` | `-` |
| `str` at 0x8d60c | store | 4 | `sp -0x5c4` | `?` |
| `ldr` at 0x8d610 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d614 | load | 1 | `[sp-24] +0x54` | `-` |
| `ldr` at 0x8d624 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d628 | load | 4 | `[sp-24] +0x58` | `-` |
| `ldr` at 0x8d62c | load | 4 | `abs 0x8dae8` | `-` |
| `str` at 0x8d638 | store | 4 | `sp -0x5c8` | `?` |
| `ldr` at 0x8d63c | load | 4 | `sp -0x5c8` | `-` |
| `ldr` at 0x8d644 | load | 4 | `sp -0x5c8` | `-` |
| `str` at 0x8d654 | store | 4 | `sp -0x5c8` | `?` |
| `ldr` at 0x8d65c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d668 | store | 4 | `sp -0x5c8` | `ret(0xb823c)` |
| `ldr` at 0x8d66c | load | 4 | `abs 0x8dae8` | `-` |
| `ldr` at 0x8d670 | load | 4 | `sp -0x5c8` | `-` |
| `ldr` at 0x8d68c | load | 4 | `sp -0x5c8` | `-` |
| `str` at 0x8d694 | store | 4 | `sp -0x5c8` | `?` |
| `ldr` at 0x8d6a0 | load | 4 | `sp -0x5bc` | `-` |
| `str` at 0x8d6a4 | store | 4 | `sp -0x5dc` | `[sp-1468]` |
| `ldr` at 0x8d6a8 | load | 4 | `sp -0x5c0` | `-` |
| `str` at 0x8d6ac | store | 4 | `sp -0x5d8` | `[sp-1472]` |
| `ldr` at 0x8d6b0 | load | 4 | `sp -0x5c4` | `-` |
| `str` at 0x8d6b4 | store | 4 | `sp -0x5d4` | `[sp-1476]` |
| `ldr` at 0x8d6b8 | load | 4 | `sp -0x5c8` | `-` |
| `str` at 0x8d6bc | store | 4 | `sp -0x5d0` | `[sp-1480]` |
| `ldr` at 0x8d6c4 | load | 4 | `abs 0x8daf0` | `-` |
| `ldr` at 0x8d6c8 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8d6cc | load | 4 | `sp -0x5b8` | `-` |
| `ldr` at 0x8d6f8 | load | 4 | `abs 0x8daf4` | `-` |
| `str` at 0x8d744 | store | 4 | `sp -0x2f4` | `ret(0x1b274)` |
| `str` at 0x8d758 | store | 4 | `sp -0x2e0` | `ret(0x1b46c)` |
| `ldr` at 0x8d764 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8d768 | load | 4 | `ret(0x1b46c) +0x14` | `-` |
| `ldr` at 0x8d788 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8d78c | load | 4 | `[sp-736] +0x10` | `-` |
| `ldr` at 0x8d7a8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8d7b4 | load | 4 | `[sp-736] +0xc` | `-` |
| `ldr` at 0x8d7c4 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8d7d0 | load | 4 | `[sp-736] +0x8` | `-` |
| `ldr` at 0x8d7e0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8d7ec | load | 4 | `[sp-736] +0x4` | `-` |
| `ldr` at 0x8d7fc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8d808 | load | 4 | `[sp-736] +0x0` | `-` |
| `ldr` at 0x8d814 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d818 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x8d848 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d84c | load | 4 | `abs 0x8daf8` | `-` |
| `ldr` at 0x8d850 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0x8d85c | load | 4 | `abs 0x8daf8` | `-` |
| `ldrb` at 0x8d868 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d890 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x8d898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d8a4 | load | 4 | `abs 0x8daf8` | `-` |
| `ldrb` at 0x8d8b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d8d8 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x8d8e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d914 | load | 4 | `abs 0x8daf8` | `-` |
| `ldrb` at 0x8d920 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d948 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x8d950 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d960 | store | 4 | `sp -0x2a0` | `ret(0x5566c)` |
| `ldr` at 0x8d964 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d998 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d9cc | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d9d8 | load | 4 | `abs 0x8dafc` | `-` |
| `ldr` at 0x8d9dc | load | 4 | `abs 0x14a54c` | `-` |
| `ldr` at 0x8d9e8 | load | 4 | `abs 0x8dafc` | `-` |
| `ldr` at 0x8d9f4 | load | 4 | `abs 0x14a54c` | `-` |
| `ldr` at 0x8d9f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8da28 | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x8da38 | store | 4 | `sp -0x5dc` | `sp-664` |
| `ldr` at 0x8da3c | load | 4 | `abs 0x8db00` | `-` |
| `ldr` at 0x8da44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8da48 | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x8da54 | store | 4 | `sp -0x2a0` | `ret(0xb74d4)` |
| `ldr` at 0x8da58 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8da64 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8da70 | load | 4 | `abs 0x8db04` | `-` |
| `str` at 0x8da74 | store | 4 | `sp -0x5dc` | `0x112fc0` |
| `str` at 0x8da78 | store | 4 | `sp -0x5d8` | `ret(0x21704)` |
| `ldr` at 0x8da80 | load | 4 | `abs 0x8db08` | `-` |
| `ldr` at 0x8da84 | load | 4 | `abs 0x8db0c` | `-` |
| `ldr` at 0x8da90 | load | 4 | `abs 0x8db10` | `-` |
| `str` at 0x8da9c | store | 4 | `sp -0x2a0` | `ret(0x22e50)` |
| `ldr` at 0x8daa0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8daac | load | 4 | `abs 0x8db14` | `-` |
| `str` at 0x8dab0 | store | 4 | `sp -0x5dc` | `0x112fdc` |
| `ldr` at 0x8dab4 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x8dab8 | store | 4 | `sp -0x5d8` | `ret(0x22e50)` |
| `ldr` at 0x8dac0 | load | 4 | `abs 0x8db08` | `-` |
| `ldr` at 0x8dac4 | load | 4 | `abs 0x8db18` | `-` |
| `ldr` at 0x8dad0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8dad8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8dad8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8dad8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8dad8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x88b78 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x88b90 | `(unknown callee) 0x22ac8` | `0x2b2540` | `?` | `?` | `?` |
| `bl` at 0x88bc8 | `(unknown callee) 0x21eb8` | `0x9` | `0x112bf0` | `0x1f1` | `0x0` |
| `bl` at 0x88bf0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88c24 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x88c78 | `(unknown callee) 0x83fe4` | `[sp-1208]` | `?` | `?` | `?` |
| `bl` at 0x88ca0 | `(unknown callee) 0x5f314` | `?` | `[sp-1204]` | `sp-20` | `?` |
| `bl` at 0x88ccc | `(unknown callee) 0x2079c` | `0x112c5c` | `[[sp-24]+28]` | `[sp-680]` | `[sp-24]` |
| `bl` at 0x88ce4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8909c | `(unknown callee) 0x1f26c` | `0x3` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x890bc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x890d4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x890e0 | `(unknown callee) 0x1f26c` | `0x1` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x890f0 | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x89114 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8912c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x891d4 | `(unknown callee) 0x2079c` | `0x112c70` | `?` | `[sp-708]` | `?` |
| `bl` at 0x891ec | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89208 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89228 | `(unknown callee) 0x75194` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x892a4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x892bc | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x892c4 | `(unknown callee) 0x75194` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x89340 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x89358 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x89360 | `(unknown callee) 0x75194` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x893dc | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x893f4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x893fc | `(unknown callee) 0x75194` | `0x4` | `?` | `?` | `?` |
| `bl` at 0x89478 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x89490 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x894ac | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x894b4 | `(unknown callee) 0x75194` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x894f0 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8950c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x89514 | `(unknown callee) 0x75194` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x89550 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8956c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x89590 | `(unknown callee) 0x4f588` | `?` | `sp-20` | `[sp-680]` | `?` |
| `bl` at 0x8959c | `(unknown callee) 0x4f730` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-676]` |
| `bl` at 0x895c4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x895e0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x895e8 | `(unknown callee) 0x1b2bc` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x89610 | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x1b2bc)` |
| `bl` at 0x8962c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89640 | `(unknown callee) 0x1b4c0` | `0x112c90` | `sp-700` | `0xa` | `sp-700` |
| `bl` at 0x8965c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `ret(0x1b4c0)` | `sp-668` |
| `bl` at 0x89678 | `(unknown callee) 0x1b4c0` | `[sp-700]+1` | `sp-700` | `0xa` | `[sp-700]+1` |
| `bl` at 0x89694 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `ret(0x1b4c0)` | `sp-668` |
| `bl` at 0x896ac | `(unknown callee) 0x1b4c0` | `[sp-700]+1` | `0x0` | `0xa` | `[sp-700]+1` |
| `bl` at 0x896c8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `ret(0x1b4c0)` | `sp-668` |
| `bl` at 0x896f4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89728 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x89770 | `(unknown callee) 0x880d4` | `[sp-1236]` | `[sp-1232]` | `?` | `?` |
| `bl` at 0x89788 | `(unknown callee) 0xd2790` | `0x0` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x897a8 | `(unknown callee) 0xd27e4` | `0x0` | `[sp-716]:1` | `?` | `[sp-716]:1` |
| `bl` at 0x897bc | `(unknown callee) 0x202d0` | `0x112c98` | `?` | `?` | `ret(0xd27e4)` |
| `bl` at 0x897d4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89800 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8981c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89850 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x898c4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x898f8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8992c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89944 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `0x8` | `sp-668` |
| `bl` at 0x89978 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x899ac | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x899e0 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89a14 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89a4c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89a80 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89aac | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89ae0 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x89b28 | `(unknown callee) 0x880d4` | `[sp-1248]` | `[sp-1244]` | `?` | `?` |
| `bl` at 0x89c04 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x89c40 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x89c7c | `(unknown callee) 0xb823c` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x89ca8 | `(unknown callee) 0xd2790` | `0x0` | `sp-720` | `[sp-1260]` | `sp-720` |
| `bl` at 0x89cc8 | `(unknown callee) 0xd27e4` | `0x0` | `[sp-716]:1` | `?` | `[sp-716]:1` |
| `bl` at 0x89cec | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89d18 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a160 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a194 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a1fc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a214 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x8a248 | `(unknown callee) 0x69a08` | `sp-668` | `sp-664` | `([sp-736] + [sp-728])` | `[sp-732]` |
| `mov` at 0x8a270 | `(unknown callee) (unknown target)` | `0x0` | `[sp-680]` | `[sp-680]` | `[0x2af38c]` |
| `bl` at 0x8a284 | `(unknown callee) 0x1f26c` | `0xd` | `[sp-680]` | `[sp-680]` | `[0x2af38c]` |
| `bl` at 0x8a290 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x8a2b4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a2d0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a2d8 | `(unknown callee) 0x1b2bc` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x8a300 | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x1b2bc)` |
| `bl` at 0x8a30c | `(unknown callee) 0x1f26c` | `0xe` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8a32c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a344 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8a350 | `(unknown callee) 0x1f26c` | `0x12` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8a370 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a388 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8a3a4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a604 | `(unknown callee) 0x1f26c` | `0x2a` | `?` | `?` | `[sp-676]-48` |
| `bl` at 0x8a624 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `ret(0x1f26c)` | `sp-668` |
| `bl` at 0x8a62c | `(unknown callee) 0x1f26c` | `0x2b` | `?` | `?` | `?` |
| `bl` at 0x8a64c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `ret(0x1f26c)` | `sp-668` |
| `bl` at 0x8a674 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a69c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8a6cc | `(unknown callee) 0x2079c` | `0x112cac` | `?` | `?` | `?` |
| `bl` at 0x8a6e8 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a708 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8a748 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a770 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8a788 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `0x800` | `sp-668` |
| `bl` at 0x8a7b8 | `(unknown callee) 0x2079c` | `0x112cc4` | `?` | `?` | `?` |
| `bl` at 0x8a7d4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a7f4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8a80c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `0x200` | `sp-668` |
| `bl` at 0x8a828 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a840 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a858 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-676]` | `sp-668` |
| `bl` at 0x8a870 | `(unknown callee) 0x4ea4c` | `sp-744` | `sp-740` | `sp-740` | `sp-744` |
| `bl` at 0x8a888 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a8a0 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-744]` | `sp-668` |
| `bl` at 0x8a8b8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x8a8c0 | `(unknown callee) 0x5e0a8` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8a8ec | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a92c | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x8a94c | `(unknown callee) 0x1b4cc` | `ret(0x1b1b4)` | `0x0` | `?` | `?` |
| `bl` at 0x8a95c | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x8a97c | `(unknown callee) 0x1b4cc` | `ret(0x1b1b4)` | `0x0` | `?` | `?` |
| `bl` at 0x8a9b8 | `(unknown callee) 0xfb0a8` | `[sp-736]` | `[sp-752]` | `0x4` | `?` |
| `bl` at 0x8a9d0 | `(unknown callee) 0xfb218` | `[sp-736]` | `[sp-752]` | `?` | `[sp-736]` |
| `bl` at 0x8a9f8 | `(unknown callee) 0x5e0c8` | `[sp-736]:1` | `?` | `ret(0xfb218)` | `[sp-736]:1` |
| `bl` at 0x8aa50 | `(unknown callee) 0x5e14c` | `[sp-736]:1` | `sp-20` | `?` | `[sp-736]:1` |
| `bl` at 0x8aabc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8aad4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-724]` | `sp-668` |
| `bl` at 0x8aaec | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-732]` | `sp-668` |
| `bl` at 0x8ab30 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8ab58 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8ab70 | `(unknown callee) 0x1b490` | `[sp-720]` | `?` | `?` | `[sp-724]` |
| `bl` at 0x8ab78 | `(unknown callee) 0x1b490` | `[sp-716]` | `?` | `?` | `?` |
| `bl` at 0x8ab98 | `(unknown callee) 0x1b4cc` | `sp-868` | `0x0` | `0x40` | `sp-868` |
| `bl` at 0x8aba8 | `(unknown callee) 0xdcc84` | `0x0` | `sp-792` | `?` | `sp-792` |
| `bl` at 0x8acdc | `(unknown callee) 0xe3dcc` | `sp-792` | `?` | `?` | `sp-792` |
| `bl` at 0x8adbc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8add4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8ae20 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8ae40 | `(unknown callee) 0x69a08` | `sp-668` | `sp-664` | `sp-868` | `0x40` |
| `bl` at 0x8ae6c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8aea0 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8aeb0 | `(unknown callee) 0x1d41c` | `0x3` | `[sp-1272]` | `?` | `?` |
| `bl` at 0x8aec8 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8aef4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8af68 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8af80 | `(unknown callee) 0x685bc` | `[sp-1280]` | `?` | `?` | `?` |
| `bl` at 0x8af98 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8afa4 | `(unknown callee) 0x75194` | `0x0` | `[sp-680]` | `[sp-680]` | `0x1003` |
| `bl` at 0x8afc8 | `(unknown callee) 0x68774` | `[ret(0x75194)+8]` | `[ret(0x75194)+12]` | `[ret(0x75194)+20]` | `ret(0x75194)` |
| `bl` at 0x8afe0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b00c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b03c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b070 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8b084 | `(unknown callee) 0x51b28` | `0x32` | `[sp-1284]` | `0x0` | `?` |
| `bl` at 0x8b09c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b0c8 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b0fc | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8b10c | `(unknown callee) 0x1d41c` | `0xe` | `[sp-1288]` | `?` | `?` |
| `bl` at 0x8b114 | `(unknown callee) 0x1f26c` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x8b120 | `(unknown callee) 0x758a0` | `ret(0x1f26c)` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x8b138 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b164 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b198 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8b1a8 | `(unknown callee) 0x1d41c` | `0x12` | `[sp-1292]` | `?` | `?` |
| `bl` at 0x8b1c0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b1ec | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b228 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `0x2b24a0` | `?` |
| `bl` at 0x8b244 | `(unknown callee) 0x84054` | `?` | `[sp-1300]` | `?` | `?` |
| `bl` at 0x8b25c | `(unknown callee) 0x1d41c` | `0x13` | `[0x2b24ac]` | `?` | `0x2b24a0` |
| `bl` at 0x8b274 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b2a0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b2d8 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8b340 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8b350 | `(unknown callee) 0x52f84` | `[sp-1304]` | `0x0` | `?` | `?` |
| `bl` at 0x8b384 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8b390 | `(unknown callee) 0x52ff0` | `[sp-1308]` | `?` | `?` | `?` |
| `bl` at 0x8b3c4 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8b3fc | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x8b41c | `(unknown callee) 0x53044` | `[sp-1312]` | `[sp-1316]` | `0x0` | `?` |
| `bl` at 0x8b438 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b450 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b458 | `(unknown callee) 0x862bc` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `0x100a` |
| `bl` at 0x8b484 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b4c8 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8b528 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x8b580 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8b5b8 | `(unknown callee) 0x5f160` | `?` | `[sp-688]` | `?` | `?` |
| `bl` at 0x8b5f4 | `(unknown callee) 0x2079c` | `0x112cdc` | `?` | `?` | `?` |
| `bl` at 0x8b610 | `(unknown callee) 0x83fe4` | `[0x2b24ac]` | `?` | `?` | `0x2b24a0` |
| `bl` at 0x8b62c | `(unknown callee) 0x862bc` | `sp-20` | `?` | `ret(0x83fe4)` | `?` |
| `bl` at 0x8b658 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b6b4 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8b6f8 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8b710 | `(unknown callee) 0x5e538` | `[sp-1340]` | `?` | `?` | `?` |
| `bl` at 0x8b71c | `(unknown callee) 0x4e54c` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2001` |
| `bl` at 0x8b744 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b75c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b798 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b7cc | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8b808 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8b830 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[0x14a550]` |
| `bl` at 0x8b858 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b870 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b8ac | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b8e0 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8b91c | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8b944 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[0x14a554]` |
| `bl` at 0x8b96c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b984 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b9c0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b9f4 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8ba30 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8ba58 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[0x14a558]` |
| `bl` at 0x8ba80 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ba98 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8baa4 | `(unknown callee) 0x3d890` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2005` |
| `bl` at 0x8bacc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8bae4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8baf0 | `(unknown callee) 0x3dc5c` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2006` |
| `bl` at 0x8bb18 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8bb30 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8bb44 | `(unknown callee) 0xd2790` | `0x0` | `sp-760` | `[sp-680]` | `sp-760` |
| `bl` at 0x8bb70 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8bb8c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8bc58 | `(unknown callee) 0xd2c14` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8bc74 | `(unknown callee) 0x1b52c` | `sp-996` | `0x112cf8` | `[sp-736]` | `ret(0xd2c14)` |
| `bl` at 0x8bc84 | `(unknown callee) 0x1b2bc` | `sp-996` | `?` | `?` | `sp-996` |
| `bl` at 0x8bcc0 | `(unknown callee) 0xd27e4` | `0x0` | `[sp-736]:1` | `sp-20` | `[sp-736]:1` |
| `bl` at 0x8bcdc | `(unknown callee) 0x1b364` | `sp-996` | `0x112d00` | `?` | `sp-996` |
| `bl` at 0x8bcfc | `(unknown callee) 0xd2c7c` | `[[sp-760]+28]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8bd10 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112d14` | `ret(0xd2c7c)` | `ret(0xd2c7c)` |
| `bl` at 0x8bd2c | `(unknown callee) 0xd2c14` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8bd38 | `(unknown callee) 0x8db1c` | `ret(0xd2c14)` | `?` | `?` | `ret(0xd2c14)` |
| `bl` at 0x8bdc4 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112d38` | `[sp-1372]` | `[[sp-760]+76]` |
| `bl` at 0x8bdf0 | `(unknown callee) 0xd27e4` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8be0c | `(unknown callee) 0x1b364` | `sp-996` | `0x112d50` | `?` | `sp-996` |
| `bl` at 0x8be2c | `(unknown callee) 0xd2c7c` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8be40 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112d64` | `ret(0xd2c7c)` | `ret(0xd2c7c)` |
| `bl` at 0x8be50 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112d74` | `?` | `[[sp-760]+4]` |
| `bl` at 0x8be58 | `(unknown callee) 0x1b2bc` | `[sp-720]` | `?` | `?` | `?` |
| `bl` at 0x8bea8 | `(unknown callee) 0x1b52c` | `?` | `0x112d80` | `?` | `?` |
| `bl` at 0x8bf00 | `(unknown callee) 0x1b52c` | `?` | `0x112d80` | `?` | `?` |
| `bl` at 0x8bf6c | `(unknown callee) 0x1b04c` | `?` | `?` | `[[sp-760]+12]` | `?` |
| `bl` at 0x8c028 | `(unknown callee) 0x1b04c` | `[[[sp-760]+12]+91]:1` | `?` | `0x5b` | `[[[sp-760]+12]+91]:1` |
| `bl` at 0x8c050 | `(unknown callee) 0x1b04c` | `[[[sp-760]+12]+92]:1` | `?` | `0x5c` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x8c094 | `(unknown callee) 0x1b52c` | `?` | `0x112d88` | `[[[sp-760]+12]+91]:1` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x8c15c | `(unknown callee) 0x1b52c` | `?` | `0x112d94` | `?` | `?` |
| `bl` at 0x8c164 | `(unknown callee) 0x1b2bc` | `[sp-720]` | `?` | `?` | `?` |
| `mov` at 0x8c1a8 | `(unknown callee) (unknown target)` | `[sp-760]` | `0x0` | `0x0` | `([sp-720] + ret(0x1b2bc))` |
| `bl` at 0x8c1b8 | `(unknown callee) 0x1b598` | `([sp-720] + ret(0x1b2bc))` | `0x112da0` | `ret(0x1b2bc)` | `[0x14a564]` |
| `bl` at 0x8c1e4 | `(unknown callee) 0xd27e4` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8c200 | `(unknown callee) 0x1b364` | `sp-996` | `0x112da4` | `?` | `sp-996` |
| `bl` at 0x8c220 | `(unknown callee) 0xd2c7c` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8c234 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112db8` | `ret(0xd2c7c)` | `ret(0xd2c7c)` |
| `bl` at 0x8c254 | `(unknown callee) 0xd2c7c` | `[[sp-760]+32]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8c268 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112dc8` | `ret(0xd2c7c)` | `ret(0xd2c7c)` |
| `bl` at 0x8c288 | `(unknown callee) 0xd2c7c` | `[[sp-760]+44]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8c29c | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112dec` | `ret(0xd2c7c)` | `ret(0xd2c7c)` |
| `bl` at 0x8c2bc | `(unknown callee) 0xd2c7c` | `[[sp-760]+64]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8c2d0 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112e10` | `ret(0xd2c7c)` | `ret(0xd2c7c)` |
| `bl` at 0x8c2e0 | `(unknown callee) 0x1b52c` | `[sp-720]` | `0x112e38` | `?` | `[[sp-760]+64]` |
| `bl` at 0x8c2e8 | `(unknown callee) 0x1b2bc` | `[sp-720]` | `?` | `?` | `?` |
| `bl` at 0x8c344 | `(unknown callee) 0x1b04c` | `?` | `?` | `[[sp-760]+68]` | `?` |
| `bl` at 0x8c3fc | `(unknown callee) 0x1b52c` | `?` | `0x112e44` | `?` | `?` |
| `bl` at 0x8c460 | `(unknown callee) 0x1b52c` | `?` | `0x112e4c` | `[[[sp-760]+36]+3]:1` | `[[sp-760]+40]` |
| `bl` at 0x8c474 | `(unknown callee) 0x1b364` | `sp-996` | `0x112e60` | `?` | `sp-996` |
| `bl` at 0x8c494 | `(unknown callee) 0x1b2bc` | `sp-996` | `?` | `?` | `sp-996` |
| `bl` at 0x8c4c0 | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `sp-996` | `ret(0x1b2bc)` |
| `bl` at 0x8c4e4 | `(unknown callee) 0x202d0` | `0x112e68` | `[sp-680]` | `[sp-680]` | `0x4002` |
| `bl` at 0x8c4f4 | `(unknown callee) 0x20d6c` | `0x1` | `[0x2b4430]` | `?` | `0x2b4420` |
| `bl` at 0x8c524 | `(unknown callee) 0x1b4cc` | `sp-1195` | `0x0` | `0xc7` | `sp-1195` |
| `bl` at 0x8c554 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8c590 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x8c5f0 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x3` | `[sp-1388]` | `?` |
| `bl` at 0x8c624 | `(unknown callee) 0x39a88` | `?` | `[sp-1392]` | `sp-1196` | `0x0` |
| `bl` at 0x8c650 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c674 | `(unknown callee) 0x1b2bc` | `sp-1196` | `?` | `?` | `sp-1196` |
| `bl` at 0x8c6a4 | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b2bc)` |
| `bl` at 0x8c6c0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c6d8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-736]` | `sp-668` |
| `bl` at 0x8c6fc | `(unknown callee) 0x1b2bc` | `sp-1196` | `sp-20` | `?` | `sp-1196` |
| `bl` at 0x8c72c | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b2bc)` |
| `bl` at 0x8c760 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x3` | `[sp-1388]` | `?` |
| `bl` at 0x8c79c | `(unknown callee) 0xb823c` | `[sp-24]` | `0x4` | `[sp-1396]` | `?` |
| `bl` at 0x8c7d8 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x8c818 | `(unknown callee) 0x39c80` | `?` | `[sp-724]` | `[sp-720]` | `[sp-1404]` |
| `bl` at 0x8c844 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c868 | `(unknown callee) 0x1b2bc` | `sp-1196` | `sp-20` | `?` | `sp-1196` |
| `bl` at 0x8c898 | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b2bc)` |
| `bl` at 0x8c8b4 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c8cc | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-736]` | `sp-668` |
| `bl` at 0x8c8f0 | `(unknown callee) 0x1b2bc` | `sp-1196` | `?` | `sp-20` | `sp-1196` |
| `bl` at 0x8c920 | `(unknown callee) 0x69b84` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b2bc)` |
| `bl` at 0x8c92c | `(unknown callee) 0x1f26c` | `0x7e` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8c94c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c954 | `(unknown callee) 0x1f26c` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x8c978 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `0xffff` | `sp-668` |
| `bl` at 0x8c984 | `(unknown callee) 0x1f26c` | `0x80` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x8c9a8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `0xffff` | `sp-668` |
| `bl` at 0x8c9c4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8c9f0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c9f8 | `(unknown callee) 0xdb74c` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[[sp-24]+32]:1` |
| `bl` at 0x8ca34 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8ca60 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ca94 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8caa0 | `(unknown callee) 0xdb85c` | `[sp-1416]` | `?` | `?` | `?` |
| `bl` at 0x8cacc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8cae4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8cb00 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8cb2c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8cb34 | `(unknown callee) 0xdb74c` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[[sp-24]+32]:1` |
| `bl` at 0x8cb70 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8cb9c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8cbd0 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8cbe8 | `(unknown callee) 0xdb914` | `[sp-1424]` | `sp-745` | `?` | `sp-745` |
| `bl` at 0x8cc14 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8cc2c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8cc48 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8cc60 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8cca4 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8cce0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ccfc | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8cd78 | `(unknown callee) 0xb823c` | `[sp-24]` | `?` | `0x34` | `?` |
| `bl` at 0x8cd9c | `(unknown callee) 0xd7530` | `0x0` | `[sp-1432]` | `sp-745` | `sp-745` |
| `bl` at 0x8cdc8 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8cde0 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8cdf8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8ce1c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8ce54 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ce88 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8ced0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ceec | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8cf8c | `(unknown callee) 0xb823c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x8d01c | `(unknown callee) 0xb823c` | `[sp-24]` | `?` | `[sp-756]` | `?` |
| `bl` at 0x8d0ac | `(unknown callee) 0xb823c` | `[sp-24]` | `?` | `[sp-756]` | `?` |
| `bl` at 0x8d0cc | `(unknown callee) 0xd73b0` | `0x0` | `[sp-1440]` | `[sp-1444]` | `[sp-1448]` |
| `bl` at 0x8d0f8 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d110 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8d128 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8d164 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d218 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8d24c | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8d270 | `(unknown callee) (unknown target)` | `0x0` | `[sp-1452]` | `[sp-1456]` | `0x0` |
| `bl` at 0x8d29c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d2b4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8d2d0 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8d2e8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8d304 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d320 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8d328 | `(unknown callee) 0x4e004` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x8d348 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `ret(0x4e004)` | `sp-668` |
| `bl` at 0x8d374 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d3d8 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8d440 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x8d4a8 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x8d530 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x8d5cc | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8d664 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x6` | `?` | `?` |
| `bl` at 0x8d6d0 | `(unknown callee) 0x1b52c` | `sp-1196` | `0x112f4c` | `[sp-1460]` | `[sp-1464]` |
| `bl` at 0x8d6e0 | `(unknown callee) 0x1b16c` | `sp-1196` | `?` | `?` | `sp-1196` |
| `bl` at 0x8d700 | `(unknown callee) 0x2079c` | `0x112f9c` | `sp-1196` | `?` | `sp-1196` |
| `bl` at 0x8d718 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8d734 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8d73c | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x8d750 | `(unknown callee) 0x1b46c` | `sp-756` | `?` | `?` | `sp-756` |
| `bl` at 0x8d77c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[ret(0x1b46c)+20]+1900` | `[ret(0x1b46c)+20]+1900` |
| `bl` at 0x8d79c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-736]+16]+1` | `[[sp-736]+16]+1` |
| `bl` at 0x8d7b8 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-736]+12]` | `sp-668` |
| `bl` at 0x8d7d4 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-736]+8]` | `sp-668` |
| `bl` at 0x8d7f0 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-736]+4]` | `sp-668` |
| `bl` at 0x8d80c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[[sp-736]]` | `sp-668` |
| `bl` at 0x8d838 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d90c | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d958 | `(unknown callee) 0x5566c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x8d984 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d99c | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8d9b8 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8d9d0 | `(unknown callee) 0x69964` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `mov` at 0x8da08 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-668` | `sp-664` | `[0x14a54c]` |
| `bl` at 0x8da24 | `(unknown callee) 0x69908` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8da4c | `(unknown callee) 0xb74d4` | `0x2b2594` | `0x0` | `[sp-24]` | `[sp-668]` |
| `bl` at 0x8da68 | `(unknown callee) 0x21704` | `ret(0xb74d4)` | `?` | `?` | `ret(0xb74d4)` |
| `bl` at 0x8da8c | `(unknown callee) 0x21eb8` | `0x6` | `0x112bf0` | `0x756` | `0x0` |
| `bl` at 0x8da94 | `(unknown callee) 0x22e50` | `0x2b2540` | `?` | `?` | `?` |
| `bl` at 0x8dacc | `(unknown callee) 0x21eb8` | `0x9` | `0x112bf0` | `0x75f` | `0x0` |
| `bl` at 0x8dad4 | `(unknown callee) 0xb5fec` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x88ba4 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bne` at 0x88bd8 | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x88c08 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x88c48 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x88cb8 | `[0x149294] == 0x0` | `[0x149294] != 0x0` |
| `beq` at 0x88cfc | `[sp-680] == 0x1006` | `[sp-680] != 0x1006` |
| `bhi` at 0x88d0c | `[sp-680] > 0x1006 (unsigned, 32-bit)` | `[sp-680] <= 0x1006 (unsigned, 32-bit)` |
| `beq` at 0x88d18 | `[sp-680] == 0xc` | `[sp-680] != 0xc` |
| `bhi` at 0x88d24 | `[sp-680] > 0xc (unsigned, 32-bit)` | `[sp-680] <= 0xc (unsigned, 32-bit)` |
| `beq` at 0x88d30 | `[sp-680] == 0x6` | `[sp-680] != 0x6` |
| `bhi` at 0x88d3c | `[sp-680] > 0x6 (unsigned, 32-bit)` | `[sp-680] <= 0x6 (unsigned, 32-bit)` |
| `bhi` at 0x88d48 | `[sp-680] > 0x3 (unsigned, 32-bit)` | `[sp-680] <= 0x3 (unsigned, 32-bit)` |
| `bhs` at 0x88d54 | `[sp-680] >= 0x2 (unsigned, 32-bit)` | `[sp-680] < 0x2 (unsigned, 32-bit)` |
| `beq` at 0x88d60 | `[sp-680] == 0x1` | `[sp-680] != 0x1` |
| `beq` at 0x88d70 | `[sp-680] == 0x4` | `[sp-680] != 0x4` |
| `beq` at 0x88d7c | `[sp-680] == 0x5` | `[sp-680] != 0x5` |
| `beq` at 0x88d8c | `[sp-680] == 0x9` | `[sp-680] != 0x9` |
| `bhi` at 0x88d98 | `[sp-680] > 0x9 (unsigned, 32-bit)` | `[sp-680] <= 0x9 (unsigned, 32-bit)` |
| `beq` at 0x88da4 | `[sp-680] == 0x7` | `[sp-680] != 0x7` |
| `beq` at 0x88db0 | `[sp-680] == 0x8` | `[sp-680] != 0x8` |
| `beq` at 0x88dc0 | `[sp-680] == 0xa` | `[sp-680] != 0xa` |
| `beq` at 0x88dcc | `[sp-680] == 0xb` | `[sp-680] != 0xb` |
| `beq` at 0x88ddc | `[sp-680] == 0x12` | `[sp-680] != 0x12` |
| `bhi` at 0x88de8 | `[sp-680] > 0x12 (unsigned, 32-bit)` | `[sp-680] <= 0x12 (unsigned, 32-bit)` |
| `beq` at 0x88df4 | `[sp-680] == 0xf` | `[sp-680] != 0xf` |
| `bhi` at 0x88e00 | `[sp-680] > 0xf (unsigned, 32-bit)` | `[sp-680] <= 0xf (unsigned, 32-bit)` |
| `beq` at 0x88e0c | `[sp-680] == 0xd` | `[sp-680] != 0xd` |
| `beq` at 0x88e18 | `[sp-680] == 0xe` | `[sp-680] != 0xe` |
| `beq` at 0x88e28 | `[sp-680] == 0x10` | `[sp-680] != 0x10` |
| `beq` at 0x88e34 | `[sp-680] == 0x11` | `[sp-680] != 0x11` |
| `beq` at 0x88e48 | `[sp-680] == 0x1002` | `[sp-680] != 0x1002` |
| `bhi` at 0x88e58 | `[sp-680] > 0x1002 (unsigned, 32-bit)` | `[sp-680] <= 0x1002 (unsigned, 32-bit)` |
| `beq` at 0x88e64 | `[sp-680] == 0x13` | `[sp-680] != 0x13` |
| `beq` at 0x88e74 | `[sp-680] == 0x1001` | `[sp-680] != 0x1001` |
| `beq` at 0x88e88 | `[sp-680] == 0x1003` | `[sp-680] != 0x1003` |
| `beq` at 0x88e9c | `[sp-680] == 0x2006` | `[sp-680] != 0x2006` |
| `bhi` at 0x88eac | `[sp-680] > 0x2006 (unsigned, 32-bit)` | `[sp-680] <= 0x2006 (unsigned, 32-bit)` |
| `beq` at 0x88ebc | `[sp-680] == 0x100c` | `[sp-680] != 0x100c` |
| `bhi` at 0x88ecc | `[sp-680] > 0x100c (unsigned, 32-bit)` | `[sp-680] <= 0x100c (unsigned, 32-bit)` |
| `beq` at 0x88edc | `[sp-680] == 0x1009` | `[sp-680] != 0x1009` |
| `bhi` at 0x88eec | `[sp-680] > 0x1009 (unsigned, 32-bit)` | `[sp-680] <= 0x1009 (unsigned, 32-bit)` |
| `beq` at 0x88efc | `[sp-680] == 0x1007` | `[sp-680] != 0x1007` |
| `beq` at 0x88f0c | `[sp-680] == 0x1008` | `[sp-680] != 0x1008` |
| `beq` at 0x88f20 | `[sp-680] == 0x100a` | `[sp-680] != 0x100a` |
| `beq` at 0x88f30 | `[sp-680] == 0x100b` | `[sp-680] != 0x100b` |
| `beq` at 0x88f44 | `[sp-680] == 0x2003` | `[sp-680] != 0x2003` |
| `bhi` at 0x88f54 | `[sp-680] > 0x2003 (unsigned, 32-bit)` | `[sp-680] <= 0x2003 (unsigned, 32-bit)` |
| `beq` at 0x88f64 | `[sp-680] == 0x2001` | `[sp-680] != 0x2001` |
| `beq` at 0x88f74 | `[sp-680] == 0x2002` | `[sp-680] != 0x2002` |
| `beq` at 0x88f88 | `[sp-680] == 0x2004` | `[sp-680] != 0x2004` |
| `beq` at 0x88f98 | `[sp-680] == 0x2005` | `[sp-680] != 0x2005` |
| `beq` at 0x88fac | `[sp-680] == 0x200e` | `[sp-680] != 0x200e` |
| `bhi` at 0x88fbc | `[sp-680] > 0x200e (unsigned, 32-bit)` | `[sp-680] <= 0x200e (unsigned, 32-bit)` |
| `beq` at 0x88fcc | `[sp-680] == 0x200b` | `[sp-680] != 0x200b` |
| `bhi` at 0x88fdc | `[sp-680] > 0x200b (unsigned, 32-bit)` | `[sp-680] <= 0x200b (unsigned, 32-bit)` |
| `beq` at 0x88fec | `[sp-680] == 0x2009` | `[sp-680] != 0x2009` |
| `beq` at 0x88ffc | `[sp-680] == 0x200a` | `[sp-680] != 0x200a` |
| `beq` at 0x89010 | `[sp-680] == 0x200c` | `[sp-680] != 0x200c` |
| `beq` at 0x89020 | `[sp-680] == 0x200d` | `[sp-680] != 0x200d` |
| `beq` at 0x89030 | `[sp-680] == 0x4000` | `[sp-680] != 0x4000` |
| `bhi` at 0x8903c | `[sp-680] > 0x4000 (unsigned, 32-bit)` | `[sp-680] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0x8904c | `[sp-680] == 0x200f` | `[sp-680] != 0x200f` |
| `beq` at 0x8905c | `[sp-680] == 0x3001` | `[sp-680] != 0x3001` |
| `beq` at 0x89070 | `[sp-680] == 0x4002` | `[sp-680] != 0x4002` |
| `blo` at 0x89080 | `[sp-680] < 0x4002 (unsigned, 32-bit)` | `[sp-680] >= 0x4002 (unsigned, 32-bit)` |
| `beq` at 0x89090 | `[sp-680] == 0x4003` | `[sp-680] != 0x4003` |
| `bne` at 0x89138 | `[sp-680] != 0x3` | `[sp-680] == 0x3` |
| `beq` at 0x8916c | `[sp-676] == 0x5a` | `[sp-676] != 0x5a` |
| `bgt` at 0x89178 | `[sp-676] > 0x5a (signed, 32-bit)` | `[sp-676] <= 0x5a (signed, 32-bit)` |
| `beq` at 0x89184 | `[sp-676] == 0x3d` | `[sp-676] != 0x3d` |
| `beq` at 0x89194 | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `bne` at 0x891a4 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x8925c | `[ret(0x75194)+16] != 0x3d` | `[ret(0x75194)+16] == 0x3d` |
| `bne` at 0x89268 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x892f8 | `[ret(0x75194)+16] != 0x3d` | `[ret(0x75194)+16] == 0x3d` |
| `bne` at 0x89304 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x89394 | `[ret(0x75194)+16] != 0x3d` | `[ret(0x75194)+16] == 0x3d` |
| `bne` at 0x893a0 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x89430 | `[ret(0x75194)+16] != 0x3d` | `[ret(0x75194)+16] == 0x3d` |
| `bne` at 0x8943c | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x8957c | `[sp-676] != 0x40` | `[sp-676] == 0x40` |
| `bne` at 0x895ac | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x896dc | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8970c | `? != 0x0` | `? == 0x0` |
| `blt` at 0x8973c | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x897b4 | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `blt` at 0x897e8 | `[sp-716] < [sp-720] (signed, 32-bit)` | `[sp-716] >= [sp-720] (signed, 32-bit)` |
| `beq` at 0x89880 | `[sp-1240] == 0x1` | `[sp-1240] != 0x1` |
| `beq` at 0x8988c | `[sp-1240] == 0x2` | `[sp-1240] != 0x2` |
| `bhi` at 0x89a94 | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x89ac4 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x89af4 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x89b44 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89c24 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89c60 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89cd4 | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `blt` at 0x89d00 | `[sp-716] < [sp-744] (signed, 32-bit)` | `[sp-716] >= [sp-744] (signed, 32-bit)` |
| `ldrls` at 0x89d28 | `[sp-724] <= 0x4 (unsigned, 32-bit)` | `[sp-724] > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x8a16c | `[sp-736] == 0x0` | `[sp-736] != 0x0` |
| `blo` at 0x8a17c | `[sp-728] < [sp-740] (unsigned, 32-bit)` | `[sp-728] >= [sp-740] (unsigned, 32-bit)` |
| `bls` at 0x8a1b0 | `([sp-728] + [sp-732]) <= [sp-740] (unsigned, 32-bit)` | `([sp-728] + [sp-732]) > [sp-740] (unsigned, 32-bit)` |
| `beq` at 0x8a220 | `[sp-732] == 0x0` | `[sp-732] != 0x0` |
| `beq` at 0x8a25c | `[0x2af38c] == 0x0` | `[0x2af38c] != 0x0` |
| `bne` at 0x8a29c | `ret(0x1b2bc) != 0x0` | `ret(0x1b2bc) == 0x0` |
| `ldrls` at 0x8a3b4 | `[sp-676]-48 <= 0x4c (unsigned, 32-bit)` | `[sp-676]-48 > 0x4c (unsigned, 32-bit)` |
| `bne` at 0x8a65c | `[sp-676] != 0x5a` | `[sp-676] == 0x5a` |
| `beq` at 0x8a720 | `[sp-676] == 0x5a` | `[sp-676] != 0x5a` |
| `beq` at 0x8a72c | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `bne` at 0x8a8d4 | `ret(0x5e0a8) != 0x0` | `ret(0x5e0a8) == 0x0` |
| `bne` at 0x8a90c | `[0x2af334] != 0x0` | `[0x2af334] == 0x0` |
| `blt` at 0x8a994 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8aa04 | `ret(0x5e0c8) == 0x0` | `ret(0x5e0c8) != 0x0` |
| `beq` at 0x8aa5c | `ret(0x5e14c) == 0x0` | `ret(0x5e14c) != 0x0` |
| `blt` at 0x8ab04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x8abbc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8acec | `ret(0xe3dcc) == 0x0` | `ret(0xe3dcc) != 0x0` |
| `bls` at 0x8ad18 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x8ad60 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8ae08 | `[sp-672] == 0x0` | `[sp-672] != 0x0` |
| `bhi` at 0x8ae54 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8ae84 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8aedc | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x8af08 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x8af44 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8aff4 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x8b024 | `? == 0x2` | `? != 0x2` |
| `bne` at 0x8b054 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8b0b0 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8b0e0 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8b14c | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8b17c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8b1d4 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8b20c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8b288 | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8b2b8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b2f4 | `[sp-1320] == 0x1` | `[sp-1320] != 0x1` |
| `blo` at 0x8b300 | `[sp-1320] < 0x1 (unsigned, 32-bit)` | `[sp-1320] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x8b30c | `[sp-1320] == 0x2` | `[sp-1320] != 0x2` |
| `bne` at 0x8b324 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b368 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b3a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b3dc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b46c | `[[sp-24]+32]:1 == 0x4` | `[[sp-24]+32]:1 != 0x4` |
| `bne` at 0x8b49c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b4f8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b558 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b604 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x8b628 | `ret(0x83fe4) != ?` | `ret(0x83fe4) == ?` |
| `beq` at 0x8b640 | `[[sp-24]+32]:1 == 0x3` | `[[sp-24]+32]:1 != 0x3` |
| `bne` at 0x8b68c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b6dc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b72c | `ret(0x4e54c) == 0x0` | `ret(0x4e54c) != 0x0` |
| `bls` at 0x8b770 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8b780 | `[0x14a550] != 0x0` | `[0x14a550] == 0x0` |
| `bne` at 0x8b7b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b7ec | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b840 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x8b884 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8b894 | `[0x14a554] != 0x0` | `[0x14a554] == 0x0` |
| `bne` at 0x8b8c4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b900 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b954 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x8b998 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8b9a8 | `[0x14a558] != 0x0` | `[0x14a558] == 0x0` |
| `bne` at 0x8b9d8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ba14 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8ba68 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x8bab4 | `ret(0x3d890) == 0x0` | `ret(0x3d890) != 0x0` |
| `beq` at 0x8bb00 | `ret(0x3dc5c) == 0x0` | `ret(0x3dc5c) != 0x0` |
| `bne` at 0x8bb58 | `ret(0xd2790) != 0x0` | `ret(0xd2790) == 0x0` |
| `blt` at 0x8bba4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8bca8 | `[[sp-760]] != 0x1` | `[[sp-760]] == 0x1` |
| `bne` at 0x8bccc | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `beq` at 0x8bcf0 | `[[sp-760]+28] == 0x0` | `[[sp-760]+28] != 0x0` |
| `beq` at 0x8bd4c | `ret(0x8db1c) == 0x0` | `ret(0x8db1c) != 0x0` |
| `bne` at 0x8bdd8 | `[[sp-760]] != 0x2` | `[[sp-760]] == 0x2` |
| `bne` at 0x8bdfc | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `beq` at 0x8be20 | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `ble` at 0x8be80 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x8bed8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x8bf4c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8bf78 | `ret(0x1b04c) == 0x0` | `ret(0x1b04c) != 0x0` |
| `beq` at 0x8c00c | `[[[sp-760]+12]+64]:1 == 0x2c` | `[[[sp-760]+12]+64]:1 != 0x2c` |
| `beq` at 0x8c034 | `ret(0x1b04c) == 0x0` | `ret(0x1b04c) != 0x0` |
| `bne` at 0x8c05c | `ret(0x1b04c) != 0x0` | `ret(0x1b04c) == 0x0` |
| `beq` at 0x8c188 | `[0x14a564] == 0x0` | `[0x14a564] != 0x0` |
| `bne` at 0x8c1cc | `[[sp-760]] != 0x3` | `[[sp-760]] == 0x3` |
| `bne` at 0x8c1f0 | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `beq` at 0x8c214 | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `beq` at 0x8c248 | `[[sp-760]+32] == 0x0` | `[[sp-760]+32] != 0x0` |
| `beq` at 0x8c27c | `[[sp-760]+44] == 0x0` | `[[sp-760]+44] != 0x0` |
| `beq` at 0x8c2b0 | `[[sp-760]+64] == 0x0` | `[[sp-760]+64] != 0x0` |
| `ble` at 0x8c310 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8c350 | `ret(0x1b04c) == 0x0` | `ret(0x1b04c) != 0x0` |
| `bge` at 0x8c3cc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8c488 | `[sp-728] == 0x0` | `[sp-728] != 0x0` |
| `bne` at 0x8c538 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c574 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8c5b0 | `[sp-756] == 0x1` | `[sp-756] != 0x1` |
| `beq` at 0x8c5bc | `[sp-756] == 0x2` | `[sp-756] != 0x2` |
| `bne` at 0x8c5d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c638 | `ret(0x39a88) != 0x0` | `ret(0x39a88) == 0x0` |
| `beq` at 0x8c664 | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `beq` at 0x8c6ec | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `bne` at 0x8c744 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c780 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c7bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c82c | `ret(0x39c80) != 0x0` | `ret(0x39c80) == 0x0` |
| `beq` at 0x8c858 | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `beq` at 0x8c8e0 | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `beq` at 0x8c960 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x8c990 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `bhi` at 0x8c9d8 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8ca10 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x8ca24 | `[[sp-24]+48] >= ret(0xdb74c) (unsigned, 32-bit)` | `[[sp-24]+48] < ret(0xdb74c) (unsigned, 32-bit)` |
| `bhs` at 0x8ca44 | `ret(0xb823c) >= [sp-1412] (unsigned, 32-bit)` | `ret(0xb823c) < [sp-1412] (unsigned, 32-bit)` |
| `bne` at 0x8ca78 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8cab4 | `ret(0xdb85c) == 0x0` | `ret(0xdb85c) != 0x0` |
| `bne` at 0x8cb14 | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x8cb4c | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x8cb60 | `[[sp-24]+48] >= ret(0xdb74c) (unsigned, 32-bit)` | `[[sp-24]+48] < ret(0xdb74c) (unsigned, 32-bit)` |
| `bhs` at 0x8cb80 | `ret(0xb823c) >= [sp-1420] (unsigned, 32-bit)` | `ret(0xb823c) < [sp-1420] (unsigned, 32-bit)` |
| `bne` at 0x8cbb4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8cbfc | `ret(0xdb914) == 0x0` | `ret(0xdb914) != 0x0` |
| `bls` at 0x8cc74 | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8cc88 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ccc4 | `[sp-1428] != [[sp-24]+32]:1-2` | `[sp-1428] == [[sp-24]+32]:1-2` |
| `blt` at 0x8cd14 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8cd40 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8cdb0 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `bhi` at 0x8ce3c | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x8ce6c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8ceb8 | `? == [[sp-24]+32]:1` | `? != [[sp-24]+32]:1` |
| `blt` at 0x8cf04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8cf3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cfcc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d05c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8d0e0 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `bhi` at 0x8d14c | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x8d174 | `[sp-676] == 0x35` | `[sp-676] != 0x35` |
| `beq` at 0x8d180 | `[sp-676] == 0x40` | `[sp-676] != 0x40` |
| `beq` at 0x8d18c | `[sp-676] == 0x4f` | `[sp-676] != 0x4f` |
| `beq` at 0x8d198 | `[sp-676] == 0x42` | `[sp-676] != 0x42` |
| `beq` at 0x8d1a4 | `[sp-676] == 0x46` | `[sp-676] != 0x46` |
| `beq` at 0x8d1b0 | `[sp-676] == 0x48` | `[sp-676] != 0x48` |
| `beq` at 0x8d1bc | `[sp-676] == 0x4c` | `[sp-676] != 0x4c` |
| `beq` at 0x8d1c8 | `[sp-676] == 0x4d` | `[sp-676] != 0x4d` |
| `beq` at 0x8d1d4 | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `beq` at 0x8d1e8 | `[0x14a564] == 0x0` | `[0x14a564] != 0x0` |
| `bne` at 0x8d1fc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d230 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8d284 | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `bhi` at 0x8d35c | `[[sp-24]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bne` at 0x8d38c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d420 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d464 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d4ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d574 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8d620 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8d6ec | `ret(0x1b16c) == 0x0` | `ret(0x1b16c) != 0x0` |
| `beq` at 0x8d820 | `[[sp-24]+32]:1 == 0x1` | `[[sp-24]+32]:1 != 0x1` |
| `bge` at 0x8d858 | `? >= [0x2b24b0] (signed, 32-bit)` | `? < [0x2b24b0] (signed, 32-bit)` |
| `bne` at 0x8d8a0 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x8d8f0 | `? != 0x4` | `? == 0x4` |
| `bge` at 0x8d96c | `ret(0x5566c) >= 0x0 (signed, 32-bit)` | `ret(0x5566c) < 0x0 (signed, 32-bit)` |
| `beq` at 0x8d9e4 | `[0x14a54c] == 0x0` | `[0x14a54c] != 0x0` |
| `beq` at 0x8da30 | `[sp-668] == 0x0` | `[sp-668] != 0x0` |
| `beq` at 0x8da60 | `ret(0xb74d4) == 0x0` | `ret(0xb74d4) != 0x0` |
| `beq` at 0x8daa8 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8dad8 | `r0` | `ret(0xb5fec)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x89d28).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8a3b4).

## `PktGenMemOp`

- `PktGenMemOp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61974 to 0x62a5c.
  Established. The entry branch takes `r0 == 0x1` to `0x619d8` and falls through for other values
  (`beq` at 0x619c8). An included two-byte absolute assignment writes `0x123` at `0x1491e8` (`strh`
  at 0x61f94). Separate byte assignments clear `0x1491d2`, `0x1491d1`, `0x1491d0` (`strb` at
  0x62014, `strb` at 0x62020, `strb` at 0x6202c). The indirect call has unknown target and
  candidates `[sp-28]`, `[[sp-124]+4]`, `0x40000000`, `0x8` (`mov` at 0x623d4). Memory-operation
  selectors and packet field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61984 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x61988 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x6198c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x61990 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x61998 | store | 4 | `sp -0x7c` | `0x0` |
| `str` at 0x619a0 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x619a8 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x619b0 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x619b4 | load | 4 | `abs 0x628d0` | `-` |
| `ldr` at 0x619b8 | load | 4 | `abs 0x143870` | `-` |
| `str` at 0x619bc | store | 4 | `sp -0xb4` | `[0x143870]` |
| `ldr` at 0x619c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x619cc | load | 4 | `abs 0x628d4` | `-` |
| `str` at 0x619d0 | store | 4 | `sp -0xc0` | `0xfffff377` |
| `ldr` at 0x619d8 | load | 4 | `abs 0x628d8` | `-` |
| `str` at 0x619dc | store | 4 | `sp -0x2c` | `0xeae04` |
| `ldr` at 0x619e0 | load | 4 | `abs 0x628dc` | `-` |
| `str` at 0x619e4 | store | 4 | `sp -0x28` | `0xeaf08` |
| `ldr` at 0x619e8 | load | 4 | `abs 0x628e0` | `-` |
| `ldr` at 0x619ec | load | 4 | `abs 0x1491c8` | `-` |
| `ldr` at 0x619f8 | load | 4 | `abs 0x628e4` | `-` |
| `ldr` at 0x619fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x61a00 | store | 4 | `abs 0x2b16c0` | `0xeae04` |
| `ldr` at 0x61a04 | load | 4 | `abs 0x628e8` | `-` |
| `ldr` at 0x61a08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x61a0c | store | 4 | `abs 0x2b16a0` | `[sp-40]` |
| `ldr` at 0x61a10 | load | 4 | `abs 0x628ec` | `-` |
| `str` at 0x61a14 | store | 4 | `sp -0x2c` | `0x6187c` |
| `ldr` at 0x61a18 | load | 4 | `abs 0x628f0` | `-` |
| `str` at 0x61a1c | store | 4 | `sp -0x28` | `0x618fc` |
| `ldr` at 0x61a20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61a2c | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x61a30 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x61a34 | store | 4 | `sp -0xa8` | `[sp-196]` |
| `ldr` at 0x61a38 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x61a44 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x61a50 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x61a60 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x61a6c | load | 4 | `abs 0x628f4` | `-` |
| `ldr` at 0x61a74 | load | 4 | `abs 0x628d4` | `-` |
| `str` at 0x61a78 | store | 4 | `sp -0xc0` | `0xfffff377` |
| `ldr` at 0x61a80 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x61a84 | store | 4 | `sp -0xc8` | `[sp-196]` |
| `ldr` at 0x61a88 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61a94 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61aa0 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61ab0 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61ac0 | load | 4 | `abs 0x628f8` | `-` |
| `str` at 0x61ac4 | store | 4 | `sp -0x7c` | `0x10d2ec` |
| `str` at 0x61acc | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x61ad0 | load | 4 | `abs 0x628fc` | `-` |
| `str` at 0x61ad4 | store | 4 | `sp -0x84` | `0x10d344` |
| `str` at 0x61adc | store | 4 | `sp -0x88` | `0xb` |
| `ldr` at 0x61ae0 | load | 4 | `abs 0x62900` | `-` |
| `str` at 0x61ae4 | store | 4 | `sp -0x8c` | `0x10d370` |
| `str` at 0x61aec | store | 4 | `sp -0x90` | `0x6` |
| `ldr` at 0x61af0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61af8 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x61b00 | load | 4 | `abs 0x62904` | `-` |
| `str` at 0x61b04 | store | 4 | `sp -0x7c` | `0x10d250` |
| `str` at 0x61b0c | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x61b10 | load | 4 | `abs 0x62908` | `-` |
| `str` at 0x61b14 | store | 4 | `sp -0x84` | `0x10d2a8` |
| `str` at 0x61b1c | store | 4 | `sp -0x88` | `0xb` |
| `str` at 0x61b24 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x61b2c | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x61b30 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61b38 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x61b40 | load | 4 | `abs 0x6290c` | `-` |
| `str` at 0x61b44 | store | 4 | `sp -0x7c` | `0x10d388` |
| `str` at 0x61b4c | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x61b50 | load | 4 | `abs 0x62910` | `-` |
| `str` at 0x61b54 | store | 4 | `sp -0x84` | `0x10d3e0` |
| `str` at 0x61b5c | store | 4 | `sp -0x88` | `0xb` |
| `ldr` at 0x61b60 | load | 4 | `abs 0x62914` | `-` |
| `str` at 0x61b64 | store | 4 | `sp -0x8c` | `0x10d40c` |
| `str` at 0x61b6c | store | 4 | `sp -0x90` | `0x6` |
| `ldr` at 0x61b70 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61b78 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x61b7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x61b88 | store | 4 | `sp -0x30` | `ret(0xebc18)` |
| `ldr` at 0x61b8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61b98 | load | 4 | `abs 0x62918` | `-` |
| `ldr` at 0x61b9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ba4 | load | 4 | `abs 0x6291c` | `-` |
| `str` at 0x61ba8 | store | 4 | `sp -0xc0` | `0xfffff375` |
| `ldr` at 0x61bb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61bc4 | load | 4 | `abs 0x62920` | `-` |
| `ldr` at 0x61bc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61bd0 | load | 4 | `abs 0x62924` | `-` |
| `str` at 0x61bd4 | store | 4 | `sp -0xc0` | `0xfffff374` |
| `ldr` at 0x61bdc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x61be8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61bec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61bf0 | load | 4 | `[sp-124] +0x0` | `-` |
| `ldr` at 0x61c0c | load | 4 | `abs 0x62928` | `-` |
| `str` at 0x61c18 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x61c20 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x61c2c | store | 4 | `sp -0x34` | `?` |
| `str` at 0x61c34 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x61c3c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61c4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61c50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61c54 | load | 4 | `[sp-132] +0x20` | `-` |
| `str` at 0x61c74 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x61c7c | load | 4 | `abs 0x628d0` | `-` |
| `str` at 0x61c84 | store | 4 | `abs 0x143870` | `0x0` |
| `str` at 0x61c8c | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x61c94 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x61c9c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61ca4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61ca8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61cac | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x61cb0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61cb4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61ccc | load | 4 | `abs 0x6292c` | `-` |
| `ldr` at 0x61cd4 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x61cd8 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61ce0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61ce4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61cec | load | 4 | `[sp-132] +0x0` | `-` |
| `ldr` at 0x61cf0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61cf4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61d0c | load | 4 | `abs 0x62930` | `-` |
| `ldr` at 0x61d14 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x61d18 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61d24 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61d2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61d30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61d34 | load | 4 | `[sp-132] +0x1c` | `-` |
| `ldr` at 0x61d38 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61d3c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61d54 | load | 4 | `abs 0x62934` | `-` |
| `ldr` at 0x61d5c | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x61d60 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61d6c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61d74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61d78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61d7c | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x61d80 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61d84 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61d9c | load | 4 | `abs 0x62938` | `-` |
| `ldr` at 0x61da4 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x61da8 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61db0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x61dbc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x61dc0 | store | 4 | `sp -0x54` | `[sp-68]` |
| `ldr` at 0x61dc4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x61dcc | store | 4 | `sp -0x58` | `?` |
| `str` at 0x61dd4 | store | 4 | `sp -0x54` | `?` |
| `str` at 0x61ddc | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61de4 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x61df4 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x61df8 | load | 4 | `abs 0x6293c` | `-` |
| `str` at 0x61dfc | store | 4 | `sp -0x58` | `0x7ffffd` |
| `str` at 0x61e08 | store | 4 | `sp -0x54` | `?` |
| `str` at 0x61e10 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61e18 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61e20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61e24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e28 | load | 4 | `[sp-132] +0x20` | `-` |
| `ldr` at 0x61e2c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61e30 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61e48 | load | 4 | `abs 0x62940` | `-` |
| `ldr` at 0x61e50 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x61e54 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x61e60 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x61e64 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61e68 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x61e78 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61e80 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x61e90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61eb4 | load | 4 | `abs 0x62944` | `-` |
| `str` at 0x61ec0 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x61ec8 | load | 4 | `abs 0x629b4` | `-` |
| `ldr` at 0x61ecc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61ed0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61ed4 | store | 4 | `(unknown)` | `[sp-84]` |
| `ldr` at 0x61ed8 | load | 4 | `abs 0x629b4` | `-` |
| `ldr` at 0x61edc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61ef0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61ef4 | store | 4 | `(unknown)` | `[sp-88]` |
| `ldr` at 0x61ef8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61efc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x61f04 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61f08 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61f10 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x61f18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61f1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61f20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61f24 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61f28 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61f40 | load | 4 | `abs 0x62948` | `-` |
| `ldr` at 0x61f48 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x61f4c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61f54 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x61f5c | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x61f64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61f70 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x61f74 | store | 4 | `sp -0x20` | `[sp-52]` |
| `ldr` at 0x61f78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x61f88 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x61f8c | load | 4 | `abs 0x6294c` | `-` |
| `ldr` at 0x61f90 | load | 4 | `abs 0x62950` | `-` |
| `strh` at 0x61f94 | store | 2 | `abs 0x1491e8` | `0x123` |
| `ldr` at 0x61f98 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x61f9c | load | 1 | `sp -0x38` | `-` |
| `strb` at 0x61fa0 | store | 1 | `abs 0x1491e0` | `([sp-56]:1 & 0xff)` |
| `ldr` at 0x61fa4 | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x61fa8 | load | 2 | `sp -0x34` | `-` |
| `strh` at 0x61fac | store | 2 | `abs 0x1491de` | `([sp-52]:2 & 0xffff)` |
| `ldr` at 0x61fb0 | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x61fb4 | load | 2 | `sp +0x0` | `-` |
| `strh` at 0x61fbc | store | 2 | `abs 0x1491dc` | `?` |
| `ldr` at 0x61fc0 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x61fc4 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x61fc8 | store | 1 | `abs 0x1491d7` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x61fcc | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x61fd0 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x61fd4 | store | 2 | `abs 0x1491d4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x61fd8 | load | 4 | `abs 0x6294c` | `-` |
| `str` at 0x61fdc | store | 4 | `sp -0xcc` | `0x1491d0` |
| `ldr` at 0x61fe0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x61ff0 | store | 1 | `sp -0xcd` | `0x4` |
| `strb` at 0x61ffc | store | 1 | `sp -0xcd` | `0x4c` |
| `ldrb` at 0x62000 | load | 1 | `sp -0xcd` | `-` |
| `ldr` at 0x62004 | load | 4 | `sp -0xcc` | `-` |
| `strb` at 0x62008 | store | 1 | `abs 0x1491d6` | `([sp-205]:1 & 0xff)` |
| `ldr` at 0x6200c | load | 4 | `abs 0x6294c` | `-` |
| `strb` at 0x62014 | store | 1 | `abs 0x1491d2` | `0x0` |
| `ldr` at 0x62018 | load | 4 | `abs 0x6294c` | `-` |
| `strb` at 0x62020 | store | 1 | `abs 0x1491d1` | `0x0` |
| `ldr` at 0x62024 | load | 4 | `abs 0x6294c` | `-` |
| `strb` at 0x6202c | store | 1 | `abs 0x1491d0` | `0x0` |
| `str` at 0x62034 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x62038 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6203c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x6204c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62054 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x6205c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x62060 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x62068 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x62070 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x62074 | load | 4 | `abs 0x6294c` | `-` |
| `ldr` at 0x62078 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x62084 | store | 1 | `abs 0x1491e1` | `?` |
| `ldr` at 0x62088 | load | 4 | `abs 0x6294c` | `-` |
| `ldr` at 0x6208c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x62098 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x620a8 | store | 4 | `abs 0x1491d8` | `?` |
| `ldr` at 0x620ac | load | 4 | `abs 0x6294c` | `-` |
| `ldr` at 0x620b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x620c0 | store | 4 | `abs 0x1491e4` | `?` |
| `ldr` at 0x620c4 | load | 4 | `abs 0x6294c` | `-` |
| `ldr` at 0x620c8 | load | 4 | `abs 0x1491d8` | `-` |
| `ldr` at 0x620d4 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x620d8 | load | 1 | `abs 0x1491d7` | `-` |
| `str` at 0x620e4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x620e8 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x620ec | load | 1 | `abs 0x1491d6` | `-` |
| `ldr` at 0x620f4 | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x620f8 | load | 2 | `abs 0x1491d4` | `-` |
| `ldr` at 0x6210c | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x62110 | load | 1 | `abs 0x1491d2` | `-` |
| `ldr` at 0x62120 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x62124 | load | 1 | `abs 0x1491d1` | `-` |
| `ldr` at 0x62130 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x62134 | load | 1 | `abs 0x1491d0` | `-` |
| `str` at 0x62140 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x62148 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x62150 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62154 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62158 | load | 4 | `[sp-124] +0x8` | `-` |
| `ldr` at 0x6215c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x62160 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62178 | load | 4 | `abs 0x62954` | `-` |
| `ldr` at 0x62180 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x62184 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x6218c | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x62190 | load | 2 | `abs 0x1491e8` | `-` |
| `ldr` at 0x621a0 | load | 4 | `abs 0x6294c` | `-` |
| `ldr` at 0x621a4 | load | 4 | `abs 0x1491e4` | `-` |
| `ldr` at 0x621b8 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x621bc | load | 1 | `abs 0x1491e1` | `-` |
| `str` at 0x621cc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x621d0 | load | 4 | `abs 0x6294c` | `-` |
| `ldrb` at 0x621d4 | load | 1 | `abs 0x1491e0` | `-` |
| `ldr` at 0x621e0 | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x621e4 | load | 2 | `abs 0x1491de` | `-` |
| `ldr` at 0x621f8 | load | 4 | `abs 0x6294c` | `-` |
| `ldrh` at 0x621fc | load | 2 | `abs 0x1491dc` | `-` |
| `str` at 0x6220c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x62214 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6221c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62224 | load | 4 | `[sp-124] +0xc` | `-` |
| `ldr` at 0x62228 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6222c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62244 | load | 4 | `abs 0x62958` | `-` |
| `ldr` at 0x6224c | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x62250 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x6225c | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x62260 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x62268 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x6226c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x6227c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x62288 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x62290 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x62298 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x622a4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x622b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x622b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x622c0 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x622cc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x622d8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x622e4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x622f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x622f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62304 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x62310 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6231c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x62324 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x6232c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x62338 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x62344 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62348 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6234c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62354 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6236c | load | 4 | `abs 0x6295c` | `-` |
| `ldr` at 0x62370 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x62378 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x6237c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x62384 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6238c | store | 4 | `sp -0xac` | `[sp-172]+1` |
| `ldr` at 0x62390 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x62398 | store | 4 | `sp -0xb0` | `[sp-176]+2` |
| `str` at 0x623a4 | store | 4 | `sp -0x54` | `0x40000000` |
| `str` at 0x623ac | store | 4 | `sp -0x58` | `0x8` |
| `ldr` at 0x623b4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x623bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x623c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x623c4 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0x623c8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x623cc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x623e4 | load | 4 | `abs 0x62960` | `-` |
| `ldr` at 0x623ec | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x623f0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x623fc | store | 4 | `sp -0x9c` | `0xa` |
| `str` at 0x62404 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x62408 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x62414 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x6241c | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x62430 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x62440 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62448 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0x62464 | load | 4 | `abs 0x62964` | `-` |
| `str` at 0x62470 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x62478 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x62484 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6248c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x62498 | load | 4 | `abs 0x62968` | `-` |
| `ldr` at 0x624a0 | load | 4 | `abs 0x6296c` | `-` |
| `str` at 0x624a4 | store | 4 | `sp -0xa4` | `0xfffff37e` |
| `str` at 0x624b0 | store | 4 | `sp -0x9c` | `0xa` |
| `str` at 0x624b8 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x624bc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x624c8 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x624d0 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x624e4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x624f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x624f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x624fc | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x62518 | load | 4 | `abs 0x62970` | `-` |
| `str` at 0x62524 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x6252c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x62538 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x62540 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6254c | load | 4 | `abs 0x62974` | `-` |
| `ldr` at 0x62554 | load | 4 | `abs 0x62978` | `-` |
| `str` at 0x62558 | store | 4 | `sp -0xa4` | `0xfffff37d` |
| `ldr` at 0x62564 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x62574 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6257c | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x62598 | load | 4 | `abs 0x6297c` | `-` |
| `str` at 0x625a4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x625ac | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x625bc | load | 4 | `abs 0x62980` | `-` |
| `ldr` at 0x625c4 | load | 4 | `abs 0x62984` | `-` |
| `str` at 0x625c8 | store | 4 | `sp -0xa4` | `0xfffff37c` |
| `ldr` at 0x625d0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x625e0 | load | 4 | `abs 0x62988` | `-` |
| `ldr` at 0x625e8 | load | 4 | `abs 0x6298c` | `-` |
| `str` at 0x625ec | store | 4 | `sp -0xa4` | `0xfffff37b` |
| `ldr` at 0x625f4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x62604 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x6260c | load | 4 | `abs 0x62990` | `-` |
| `ldr` at 0x62618 | load | 4 | `abs 0x62994` | `-` |
| `str` at 0x6261c | store | 4 | `sp -0xa4` | `0xfffff37a` |
| `str` at 0x62628 | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x62630 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x62638 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x62640 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x62648 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x62658 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6265c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62660 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x6267c | load | 4 | `abs 0x62970` | `-` |
| `str` at 0x62688 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x62694 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6269c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x626a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x626a4 | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x626a8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x626ac | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x626c4 | load | 4 | `abs 0x62998` | `-` |
| `ldr` at 0x626cc | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x626d0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x626d8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x626dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x626e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x626e4 | load | 4 | `[sp-132] +0x0` | `-` |
| `ldr` at 0x626e8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x626ec | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62704 | load | 4 | `abs 0x6299c` | `-` |
| `ldr` at 0x6270c | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x62710 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x6271c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x62724 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6272c | load | 4 | `[sp-132] +0x28` | `-` |
| `ldr` at 0x6274c | load | 4 | `abs 0x629a0` | `-` |
| `ldr` at 0x62754 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x62758 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x62764 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6276c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62774 | load | 4 | `[sp-132] +0x24` | `-` |
| `ldr` at 0x62794 | load | 4 | `abs 0x629a4` | `-` |
| `ldr` at 0x6279c | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x627a0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x627ac | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x627bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x627c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x627c4 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x627e0 | load | 4 | `abs 0x62970` | `-` |
| `str` at 0x627ec | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x627f4 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x6280c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x62810 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6281c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x62824 | store | 4 | `sp -0x9c` | `[sp-156]-1` |
| `ldr` at 0x62830 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6283c | load | 4 | `abs 0x629a8` | `-` |
| `ldr` at 0x62844 | load | 4 | `abs 0x629ac` | `-` |
| `str` at 0x62848 | store | 4 | `sp -0xa4` | `0xfffff378` |
| `ldr` at 0x62850 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x62858 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x62860 | load | 4 | `abs 0x628d0` | `-` |
| `ldr` at 0x62864 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x62868 | store | 4 | `abs 0x143870` | `[sp-180]` |
| `ldr` at 0x62870 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x62878 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6287c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62880 | load | 4 | `[sp-132] +0x20` | `-` |
| `ldr` at 0x62884 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x62888 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x628a0 | load | 4 | `abs 0x629b0` | `-` |
| `ldr` at 0x628a8 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x628ac | store | 4 | `sp -0xc0` | `0xfffff380` |
| `str` at 0x628b8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x628bc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x628c0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x629c0 | load | 4 | `abs 0x629b4` | `-` |
| `ldr` at 0x629c4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x629c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x629cc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x629d0 | load | 4 | `abs 0x629b4` | `-` |
| `ldr` at 0x629d4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x629e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x629ec | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x629f0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x629f8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x62a00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62a04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62a08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62a0c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x62a10 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x62a28 | load | 4 | `abs 0x629b8` | `-` |
| `ldr` at 0x62a30 | load | 4 | `abs 0x629bc` | `-` |
| `str` at 0x62a34 | store | 4 | `sp -0xc0` | `0xfffff380` |
| `ldr` at 0x62a3c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x62a44 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x62a4c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x62a50 | store | 4 | `sp -0xc0` | `[sp-164]` |
| `ldr` at 0x62a54 | load | 4 | `sp -0xc0` | `-` |
| `ldmdb` at 0x62a58 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x62a58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x62a58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x62a58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61a70 | `(unknown callee) 0x2079c` | `0x10d444` | `r1` | `[sp-196]` | `[sp-196]` |
| `bl` at 0x61b80 | `(unknown callee) 0xebc18` | `?` | `r1` | `[sp-196]` | `?` |
| `bl` at 0x61ba0 | `(unknown callee) 0x2079c` | `0x10d458` | `[sp-28]` | `?` | `ret(0xebc18)` |
| `bl` at 0x61bb4 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |
| `bl` at 0x61bcc | `(unknown callee) 0x2079c` | `0x10d478` | `[sp-28]` | `?` | `ret(0xe9a1c)` |
| `mov` at 0x61bfc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]]` | `sp-76` | `sp-80` |
| `bl` at 0x61c10 | `(unknown callee) 0x2079c` | `0x10d498` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61c60 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `sp-68` | `sp-72` |
| `mov` at 0x61cbc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `0x0` | `0x0` |
| `bl` at 0x61cd0 | `(unknown callee) 0x2079c` | `0x10d4b8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61cfc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61d10 | `(unknown callee) 0x2079c` | `0x10d4d8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61d44 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+28]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61d58 | `(unknown callee) 0x2079c` | `0x10d4f8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61d8c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61da0 | `(unknown callee) 0x2079c` | `0x10d518` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61e38 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61e4c | `(unknown callee) 0x2079c` | `0x10d534` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61ea4 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `sp-84` | `sp-88` |
| `bl` at 0x61eb8 | `(unknown callee) 0x2079c` | `0x10d554` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61f30 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61f44 | `(unknown callee) 0x2079c` | `0x10d57c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62168 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+8]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x6217c | `(unknown callee) 0x2079c` | `0x10d5a4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62234 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+12]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x62248 | `(unknown callee) 0x2079c` | `0x10d5c4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6235c | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x62374 | `(unknown callee) 0x2079c` | `0x10d5e4` | `[sp-172]` | `?` | `ret(unknown)` |
| `mov` at 0x623d4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+4]` | `0x40000000` | `0x8` |
| `bl` at 0x623e8 | `(unknown callee) 0x2079c` | `0x10d604` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62454 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+4]` | `sp-84` | `sp-88` |
| `bl` at 0x62468 | `(unknown callee) 0x2079c` | `0x10d628` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x6249c | `(unknown callee) 0x2079c` | `0x10d640` | `?` | `?` | `?` |
| `mov` at 0x62508 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-84` | `sp-88` |
| `bl` at 0x6251c | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x62550 | `(unknown callee) 0x2079c` | `0x10d66c` | `?` | `?` | `?` |
| `mov` at 0x62588 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `sp-92` | `sp-96` |
| `bl` at 0x6259c | `(unknown callee) 0x202d0` | `0x10d680` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x625c0 | `(unknown callee) 0x202d0` | `0x10d694` | `?` | `?` | `?` |
| `bl` at 0x625e4 | `(unknown callee) 0x202d0` | `0x10d6a4` | `?` | `?` | `?` |
| `bl` at 0x62614 | `(unknown callee) 0x2079c` | `0x10d6b8` | `?` | `?` | `?` |
| `mov` at 0x6266c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-184` | `sp-188` |
| `bl` at 0x62680 | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x626b4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x626c8 | `(unknown callee) 0x2079c` | `0x10d6d0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x626f4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x62708 | `(unknown callee) 0x2079c` | `0x10d6e4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6273c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+40]` | `?` | `?` |
| `bl` at 0x62750 | `(unknown callee) 0x2079c` | `0x10d6fc` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62784 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+36]` | `?` | `?` |
| `bl` at 0x62798 | `(unknown callee) 0x2079c` | `0x10d718` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x627d0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-184` | `sp-188` |
| `bl` at 0x627e4 | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x62840 | `(unknown callee) 0x2079c` | `0x10d730` | `?` | `?` | `[sp-152]` |
| `mov` at 0x62890 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x628a4 | `(unknown callee) 0x2079c` | `0x10d744` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62a18 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x62a2c | `(unknown callee) 0x2079c` | `0x10d768` | `?` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x619c8 | `r0 == 0x1` | `r0 != 0x1` |
| `beq` at 0x619f4 | `[0x1491c8] == 0x0` | `[0x1491c8] != 0x0` |
| `beq` at 0x61a40 | `[sp-196] == 0x4` | `[sp-196] != 0x4` |
| `bgt` at 0x61a4c | `[sp-196] > 0x4 (signed, 32-bit)` | `[sp-196] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x61a58 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `beq` at 0x61a68 | `[sp-196] == 0x7` | `[sp-196] != 0x7` |
| `beq` at 0x61a90 | `[sp-196] == 0x4` | `[sp-196] != 0x4` |
| `bgt` at 0x61a9c | `[sp-196] > 0x4 (signed, 32-bit)` | `[sp-196] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x61aa8 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `beq` at 0x61ab8 | `[sp-196] == 0x7` | `[sp-196] != 0x7` |
| `bne` at 0x61b94 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0x61bc0 | `ret(0xe9a1c) == 0x0` | `ret(0xe9a1c) != 0x0` |
| `beq` at 0x61c08 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61c6c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61cc8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61d08 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61d50 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61d98 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x61db8 | `[sp-168] != 0x2` | `[sp-168] == 0x2` |
| `bne` at 0x61dec | `[sp-168] != 0x4` | `[sp-168] == 0x4` |
| `beq` at 0x61e44 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x61e70 | `C == 0` | `not (C == 0)` |
| `beq` at 0x61eb0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61f3c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x61f6c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x61f80 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x61fe8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x62044 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x62174 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62240 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x62274 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `ble` at 0x62284 | `[sp-160] <= 0x0 (signed, 32-bit)` | `[sp-160] > 0x0 (signed, 32-bit)` |
| `bne` at 0x622bc | `? != ?` | `? == ?` |
| `bne` at 0x622fc | `? != ?` | `? == ?` |
| `beq` at 0x62368 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x623e0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62410 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x62424 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62460 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62494 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x624c4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x624d8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62514 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62548 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62594 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x625b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x625dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62600 | `? == 0xe0` | `? != 0xe0` |
| `beq` at 0x62678 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x626c0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62700 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62748 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62790 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x627dc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62818 | `[sp-152] != 0x0` | `[sp-152] == 0x0` |
| `bne` at 0x6282c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x62838 | `[sp-152] != 0x0` | `[sp-152] == 0x0` |
| `beq` at 0x6289c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x628c8 | `C == 0` | `not (C == 0)` |
| `beq` at 0x62a24 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x62a58 | `r0` | `?` |

## `PktGenOp`

- `PktGenOp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62a5c to 0x642c8.
  Established. A two-byte assignment at absolute `0x1491e8` writes `0x123` (`strh` at 0x632f0). The
  later branch takes `[sp-156] != 0x4` to `0x641c8` and falls through on equality (`bne` at
  0x640fc). The indirect call at an unknown target carries `[sp-28]`, `0x90071100`, `0x0`, `0x0`
  (`mov` at 0x641a4). Separate byte assignments clear `0x1491d2`, `0x1491d1`, `0x1491d0` (`strb` at
  0x63370, `strb` at 0x6337c, `strb` at 0x63388). Packet selectors and the device meaning of call
  arguments remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62a6c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x62a70 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x62a74 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x62a78 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x62a80 | store | 4 | `sp -0x70` | `0x0` |
| `str` at 0x62a88 | store | 4 | `sp -0x78` | `0x0` |
| `str` at 0x62a90 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x62a98 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x62a9c | load | 4 | `abs 0x639c4` | `-` |
| `str` at 0x62aa0 | store | 4 | `sp -0x2c` | `0xeae04` |
| `ldr` at 0x62aa4 | load | 4 | `abs 0x639c8` | `-` |
| `str` at 0x62aa8 | store | 4 | `sp -0x28` | `0xeaf08` |
| `ldr` at 0x62aac | load | 4 | `abs 0x639cc` | `-` |
| `ldr` at 0x62ab0 | load | 4 | `abs 0x1491c8` | `-` |
| `ldr` at 0x62abc | load | 4 | `abs 0x639d0` | `-` |
| `ldr` at 0x62ac0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x62ac4 | store | 4 | `abs 0x2b16c0` | `0xeae04` |
| `ldr` at 0x62ac8 | load | 4 | `abs 0x639d4` | `-` |
| `ldr` at 0x62acc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x62ad0 | store | 4 | `abs 0x2b16a0` | `[sp-40]` |
| `ldr` at 0x62ad4 | load | 4 | `abs 0x639d8` | `-` |
| `str` at 0x62ad8 | store | 4 | `sp -0x2c` | `0x6187c` |
| `ldr` at 0x62adc | load | 4 | `abs 0x639dc` | `-` |
| `str` at 0x62ae0 | store | 4 | `sp -0x28` | `0x618fc` |
| `ldr` at 0x62ae4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62af0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x62af4 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x62af8 | store | 4 | `sp -0x9c` | `[sp-188]` |
| `ldr` at 0x62afc | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x62b08 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x62b14 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x62b24 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x62b30 | load | 4 | `abs 0x639e0` | `-` |
| `ldr` at 0x62b38 | load | 4 | `abs 0x639e4` | `-` |
| `str` at 0x62b3c | store | 4 | `sp -0xb8` | `0xfffff377` |
| `ldr` at 0x62b44 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x62b48 | store | 4 | `sp -0xc0` | `[sp-188]` |
| `ldr` at 0x62b4c | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x62b58 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x62b64 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x62b74 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x62b84 | load | 4 | `abs 0x639e8` | `-` |
| `str` at 0x62b88 | store | 4 | `sp -0x70` | `0x10d2ec` |
| `str` at 0x62b90 | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x62b94 | load | 4 | `abs 0x639ec` | `-` |
| `str` at 0x62b98 | store | 4 | `sp -0x78` | `0x10d344` |
| `str` at 0x62ba0 | store | 4 | `sp -0x7c` | `0xb` |
| `ldr` at 0x62ba4 | load | 4 | `abs 0x639f0` | `-` |
| `str` at 0x62ba8 | store | 4 | `sp -0x80` | `0x10d370` |
| `str` at 0x62bb0 | store | 4 | `sp -0x84` | `0x6` |
| `str` at 0x62bb8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x62bc0 | load | 4 | `abs 0x639f4` | `-` |
| `str` at 0x62bc4 | store | 4 | `sp -0x70` | `0x10d250` |
| `str` at 0x62bcc | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x62bd0 | load | 4 | `abs 0x639f8` | `-` |
| `str` at 0x62bd4 | store | 4 | `sp -0x78` | `0x10d2a8` |
| `str` at 0x62bdc | store | 4 | `sp -0x7c` | `0xb` |
| `str` at 0x62be4 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x62bec | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x62bf4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x62bfc | load | 4 | `abs 0x639fc` | `-` |
| `str` at 0x62c00 | store | 4 | `sp -0x70` | `0x10d388` |
| `str` at 0x62c08 | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x62c0c | load | 4 | `abs 0x63a00` | `-` |
| `str` at 0x62c10 | store | 4 | `sp -0x78` | `0x10d3e0` |
| `str` at 0x62c18 | store | 4 | `sp -0x7c` | `0xb` |
| `ldr` at 0x62c1c | load | 4 | `abs 0x63a04` | `-` |
| `str` at 0x62c20 | store | 4 | `sp -0x80` | `0x10d40c` |
| `str` at 0x62c28 | store | 4 | `sp -0x84` | `0x6` |
| `ldr` at 0x62c2c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x62c34 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x62c38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62c44 | store | 4 | `sp -0x98` | `ret(0xe99c4)` |
| `ldr` at 0x62c48 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x62c54 | load | 4 | `abs 0x63a08` | `-` |
| `ldr` at 0x62c58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62c60 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x62c64 | load | 4 | `abs 0x63a0c` | `-` |
| `ldr` at 0x62c70 | load | 4 | `abs 0x63a10` | `-` |
| `str` at 0x62c74 | store | 4 | `sp -0xc4` | `0xfffff375` |
| `ldr` at 0x62c7c | load | 4 | `abs 0x63a14` | `-` |
| `str` at 0x62c80 | store | 4 | `sp -0xc4` | `0xfffff374` |
| `ldr` at 0x62c84 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x62c88 | store | 4 | `sp -0xb8` | `[sp-196]` |
| `ldr` at 0x62c90 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62c9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62ca0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62ca4 | load | 4 | `[sp-112] +0x0` | `-` |
| `ldr` at 0x62cc0 | load | 4 | `abs 0x63a18` | `-` |
| `str` at 0x62ccc | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x62cd4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x62ce0 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x62ce8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x62cf0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62d00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62d04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62d08 | load | 4 | `[sp-120] +0x20` | `-` |
| `str` at 0x62d28 | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x62d34 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x62d3c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x62d44 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62d4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62d50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62d54 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x62d58 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62d5c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62d74 | load | 4 | `abs 0x63a1c` | `-` |
| `ldr` at 0x62d7c | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62d80 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62d8c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62d94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62d98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62d9c | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x62da0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62da4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62dbc | load | 4 | `abs 0x63a20` | `-` |
| `ldr` at 0x62dc4 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62dc8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62dd4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62ddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62de0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62de4 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x62de8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62dec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62e04 | load | 4 | `abs 0x63a24` | `-` |
| `ldr` at 0x62e0c | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62e10 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62e1c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62e24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62e28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62e2c | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x62e30 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62e34 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62e4c | load | 4 | `abs 0x63a28` | `-` |
| `ldr` at 0x62e54 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62e58 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62e64 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62e6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62e70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62e74 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x62e78 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62e7c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62e94 | load | 4 | `abs 0x63a2c` | `-` |
| `ldr` at 0x62e9c | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62ea0 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62ea8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62eac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62eb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62eb4 | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0x62eb8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62ebc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62ed4 | load | 4 | `abs 0x63a30` | `-` |
| `ldr` at 0x62edc | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62ee0 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62eec | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62efc | load | 4 | `[sp-120] +0x1c` | `-` |
| `ldr` at 0x62f00 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62f04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62f1c | load | 4 | `abs 0x63a34` | `-` |
| `ldr` at 0x62f24 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62f28 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62f34 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62f3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62f40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62f44 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x62f48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62f4c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62f64 | load | 4 | `abs 0x63a38` | `-` |
| `ldr` at 0x62f6c | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x62f70 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62f78 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x62f84 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x62f88 | store | 4 | `sp -0x48` | `[sp-56]` |
| `ldr` at 0x62f8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x62f94 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x62f9c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x62fac | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x62fb0 | load | 4 | `abs 0x63a3c` | `-` |
| `str` at 0x62fb4 | store | 4 | `sp -0x4c` | `0x7ffffd` |
| `str` at 0x62fc0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x62fc4 | load | 4 | `abs 0x63a40` | `-` |
| `str` at 0x62fc8 | store | 4 | `sp -0x4c` | `0x1fffffd` |
| `ldr` at 0x62fd0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62fd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62fdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62fe0 | load | 4 | `[sp-120] +0x20` | `-` |
| `ldr` at 0x62fe4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62fe8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63000 | load | 4 | `abs 0x63a44` | `-` |
| `ldr` at 0x63008 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x6300c | store | 4 | `sp -0xb8` | `0xfffff380` |
| `str` at 0x63018 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x6301c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63020 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x63030 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63038 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63048 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6304c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63050 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6306c | load | 4 | `abs 0x63a48` | `-` |
| `str` at 0x63078 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63080 | load | 4 | `abs 0x63afc` | `-` |
| `ldr` at 0x63084 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63088 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x6308c | store | 4 | `(unknown)` | `[sp-72]` |
| `ldr` at 0x63090 | load | 4 | `abs 0x63afc` | `-` |
| `ldr` at 0x63094 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x630a8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x630ac | store | 4 | `(unknown)` | `[sp-76]` |
| `ldr` at 0x630b0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x630b4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x630bc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x630c0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x630c8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x630d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x630d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x630d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x630dc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x630e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x630f8 | load | 4 | `abs 0x63a4c` | `-` |
| `ldr` at 0x63100 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63104 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x6310c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x63114 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `str` at 0x63120 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x63124 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63128 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x63144 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6314c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x63154 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63158 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x63168 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63170 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x63178 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6317c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x63180 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6318c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6319c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x631a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x631ac | store | 4 | `sp -0x20` | `[sp-48]` |
| `ldr` at 0x631b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x631bc | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x631c0 | store | 4 | `sp -0xc8` | `[sp-156]` |
| `ldr` at 0x631c4 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x631d0 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x631dc | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x631ec | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x63208 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6321c | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x63228 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0x63230 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6323c | store | 4 | `sp -0x24` | `?` |
| `str` at 0x63248 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x63264 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6326c | load | 4 | `(unknown)` | `-` |
| `str` at 0x63274 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x63278 | load | 4 | `abs 0x639cc` | `-` |
| `ldr` at 0x6327c | load | 4 | `abs 0x1491c8` | `-` |
| `ldr` at 0x63288 | load | 4 | `abs 0x63a50` | `-` |
| `ldr` at 0x6328c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x63294 | load | 4 | `abs 0x63a54` | `-` |
| `ldr` at 0x63298 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x632a0 | load | 4 | `abs 0x63a58` | `-` |
| `ldr` at 0x632a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x632ac | load | 4 | `abs 0x63a5c` | `-` |
| `ldr` at 0x632b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x632c4 | load | 4 | `abs 0x63a60` | `-` |
| `ldr` at 0x632c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x632dc | load | 4 | `abs 0x63a64` | `-` |
| `ldr` at 0x632e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x632e8 | load | 4 | `abs 0x63a68` | `-` |
| `ldr` at 0x632ec | load | 4 | `abs 0x63a6c` | `-` |
| `strh` at 0x632f0 | store | 2 | `abs 0x1491e8` | `0x123` |
| `ldr` at 0x632f4 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x632f8 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0x632fc | store | 1 | `abs 0x1491e0` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0x63300 | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x63304 | load | 2 | `sp -0x30` | `-` |
| `strh` at 0x63308 | store | 2 | `abs 0x1491de` | `([sp-48]:2 & 0xffff)` |
| `ldr` at 0x6330c | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x63310 | load | 2 | `sp +0x0` | `-` |
| `strh` at 0x63318 | store | 2 | `abs 0x1491dc` | `?` |
| `ldr` at 0x6331c | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x63320 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x63324 | store | 1 | `abs 0x1491d7` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x63328 | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x6332c | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x63330 | store | 2 | `abs 0x1491d4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x63334 | load | 4 | `abs 0x63a68` | `-` |
| `str` at 0x63338 | store | 4 | `sp -0xcc` | `0x1491d0` |
| `ldr` at 0x6333c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x6334c | store | 1 | `sp -0xcd` | `0x11` |
| `strb` at 0x63358 | store | 1 | `sp -0xcd` | `0x31` |
| `ldrb` at 0x6335c | load | 1 | `sp -0xcd` | `-` |
| `ldr` at 0x63360 | load | 4 | `sp -0xcc` | `-` |
| `strb` at 0x63364 | store | 1 | `abs 0x1491d6` | `([sp-205]:1 & 0xff)` |
| `ldr` at 0x63368 | load | 4 | `abs 0x63a68` | `-` |
| `strb` at 0x63370 | store | 1 | `abs 0x1491d2` | `0x0` |
| `ldr` at 0x63374 | load | 4 | `abs 0x63a68` | `-` |
| `strb` at 0x6337c | store | 1 | `abs 0x1491d1` | `0x0` |
| `ldr` at 0x63380 | load | 4 | `abs 0x63a68` | `-` |
| `strb` at 0x63388 | store | 1 | `abs 0x1491d0` | `0x0` |
| `ldr` at 0x6338c | load | 4 | `abs 0x63a68` | `-` |
| `ldr` at 0x6339c | load | 4 | `(unknown)` | `-` |
| `strb` at 0x633a8 | store | 1 | `abs 0x1491e1` | `?` |
| `ldr` at 0x633ac | load | 4 | `abs 0x63a68` | `-` |
| `ldr` at 0x633bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x633d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x633e4 | store | 4 | `abs 0x1491d8` | `?` |
| `ldr` at 0x633e8 | load | 4 | `abs 0x63a68` | `-` |
| `ldr` at 0x633f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63408 | store | 4 | `abs 0x1491e4` | `?` |
| `ldr` at 0x6340c | load | 4 | `abs 0x63a68` | `-` |
| `ldr` at 0x63410 | load | 4 | `abs 0x1491d8` | `-` |
| `ldr` at 0x6341c | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x63420 | load | 1 | `abs 0x1491d7` | `-` |
| `str` at 0x6342c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x63430 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x63434 | load | 1 | `abs 0x1491d6` | `-` |
| `ldr` at 0x6343c | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x63440 | load | 2 | `abs 0x1491d4` | `-` |
| `ldr` at 0x63454 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x63458 | load | 1 | `abs 0x1491d2` | `-` |
| `ldr` at 0x63468 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x6346c | load | 1 | `abs 0x1491d1` | `-` |
| `ldr` at 0x63478 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x6347c | load | 1 | `abs 0x1491d0` | `-` |
| `str` at 0x63488 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x63490 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x63498 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6349c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x634a0 | load | 4 | `[sp-112] +0x8` | `-` |
| `ldr` at 0x634a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x634a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x634c0 | load | 4 | `abs 0x63a70` | `-` |
| `ldr` at 0x634c8 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x634cc | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x634d4 | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x634d8 | load | 2 | `abs 0x1491e8` | `-` |
| `ldr` at 0x634e8 | load | 4 | `abs 0x63a68` | `-` |
| `ldr` at 0x634ec | load | 4 | `abs 0x1491e4` | `-` |
| `ldr` at 0x63500 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x63504 | load | 1 | `abs 0x1491e1` | `-` |
| `str` at 0x63514 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x63518 | load | 4 | `abs 0x63a68` | `-` |
| `ldrb` at 0x6351c | load | 1 | `abs 0x1491e0` | `-` |
| `ldr` at 0x63528 | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x6352c | load | 2 | `abs 0x1491de` | `-` |
| `ldr` at 0x63540 | load | 4 | `abs 0x63a68` | `-` |
| `ldrh` at 0x63544 | load | 2 | `abs 0x1491dc` | `-` |
| `str` at 0x63554 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x6355c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x63564 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6356c | load | 4 | `[sp-112] +0xc` | `-` |
| `ldr` at 0x63570 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63574 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6358c | load | 4 | `abs 0x63a74` | `-` |
| `ldr` at 0x63594 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63598 | store | 4 | `sp -0x98` | `0xfffff380` |
| `str` at 0x635a4 | store | 4 | `sp -0x48` | `0x40000000` |
| `str` at 0x635ac | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x635b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x635bc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x635c4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x635cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x635e0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x635e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x635ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x635f0 | load | 4 | `[sp-112] +0x10` | `-` |
| `ldr` at 0x63610 | load | 4 | `abs 0x63a78` | `-` |
| `ldr` at 0x63618 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x6361c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63628 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x63630 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63638 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63640 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63648 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63650 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63658 | load | 4 | `[sp-112] +0x14` | `-` |
| `ldr` at 0x6365c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63660 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63678 | load | 4 | `abs 0x63a7c` | `-` |
| `ldr` at 0x63680 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63684 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63690 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x63698 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x636a0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x636a8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x636b0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x636b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x636bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x636c0 | load | 4 | `[sp-112] +0x10` | `-` |
| `ldr` at 0x636c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x636c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x636e0 | load | 4 | `abs 0x63a78` | `-` |
| `ldr` at 0x636e8 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x636ec | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x636f8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x63700 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63708 | load | 4 | `[sp-112] +0x14` | `-` |
| `ldr` at 0x63728 | load | 4 | `abs 0x63a7c` | `-` |
| `ldr` at 0x63730 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63734 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x6373c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x63744 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x6374c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x63754 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63758 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6375c | load | 4 | `[sp-112] +0x4` | `-` |
| `ldr` at 0x63760 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63764 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6377c | load | 4 | `abs 0x63a80` | `-` |
| `ldr` at 0x63784 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63788 | store | 4 | `sp -0x98` | `0xfffff380` |
| `str` at 0x63794 | store | 4 | `sp -0x90` | `0x14` |
| `str` at 0x6379c | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x637a0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x637ac | load | 4 | `sp -0x90` | `-` |
| `str` at 0x637b4 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x637c8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x637d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x637dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x637e0 | load | 4 | `[sp-112] +0x4` | `-` |
| `ldr` at 0x637fc | load | 4 | `abs 0x63a84` | `-` |
| `str` at 0x63808 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63810 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x6381c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x63820 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63838 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63844 | load | 4 | `abs 0x63a88` | `-` |
| `ldr` at 0x6384c | load | 4 | `abs 0x63a8c` | `-` |
| `str` at 0x63850 | store | 4 | `sp -0x98` | `0xfffff37e` |
| `str` at 0x6385c | store | 4 | `sp -0x90` | `0x14` |
| `str` at 0x63864 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x63868 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63874 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6387c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x63890 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x638a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x638a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x638a8 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x638c4 | load | 4 | `abs 0x63adc` | `-` |
| `str` at 0x638d0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x638d8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x638e4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x638e8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63900 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6390c | load | 4 | `abs 0x63a90` | `-` |
| `ldr` at 0x63914 | load | 4 | `abs 0x63a94` | `-` |
| `str` at 0x63918 | store | 4 | `sp -0x98` | `0xfffff37d` |
| `ldr` at 0x63920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x63930 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63940 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63944 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63948 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x63964 | load | 4 | `abs 0x63a98` | `-` |
| `str` at 0x63970 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6397c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6398c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63994 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x639b0 | load | 4 | `abs 0x63a98` | `-` |
| `str` at 0x639bc | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63b0c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63b1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63b20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63b24 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x63b40 | load | 4 | `abs 0x63a98` | `-` |
| `str` at 0x63b4c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63b58 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63b68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63b6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63b70 | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x63b8c | load | 4 | `abs 0x63a98` | `-` |
| `str` at 0x63b98 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63ba0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x63bb0 | load | 4 | `abs 0x63a9c` | `-` |
| `ldr` at 0x63bb8 | load | 4 | `abs 0x63aa0` | `-` |
| `str` at 0x63bbc | store | 4 | `sp -0x98` | `0xfffff37c` |
| `ldr` at 0x63bc4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x63bd4 | load | 4 | `abs 0x63aa4` | `-` |
| `ldr` at 0x63bdc | load | 4 | `abs 0x63aa8` | `-` |
| `str` at 0x63be0 | store | 4 | `sp -0x98` | `0xfffff37b` |
| `ldr` at 0x63be8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x63bf8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x63c00 | load | 4 | `abs 0x63aac` | `-` |
| `ldr` at 0x63c0c | load | 4 | `abs 0x63ab0` | `-` |
| `str` at 0x63c10 | store | 4 | `sp -0x98` | `0xfffff37a` |
| `ldr` at 0x63c18 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x63c24 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x63c28 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x63c30 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x63c34 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x63c3c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x63c44 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x63c48 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x63c50 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x63c60 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x63c64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63c7c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x63c80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63ca4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63ca8 | store | 4 | `sp -0xd4` | `?` |
| `ldr` at 0x63cac | load | 4 | `abs 0x63ab4` | `-` |
| `ldr` at 0x63cb0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63cb4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63cb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63cc0 | load | 4 | `abs 0x63ab8` | `-` |
| `str` at 0x63cc4 | store | 4 | `sp -0x98` | `0xfffff379` |
| `ldr` at 0x63cd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63ce8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63cf0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63cf8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x63cfc | store | 4 | `(unknown)` | `[sp-96]` |
| `ldr` at 0x63d00 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63d08 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63d10 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x63d14 | store | 4 | `(unknown)` | `[sp-100]` |
| `ldr` at 0x63d1c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63d24 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63d2c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x63d30 | store | 4 | `(unknown)` | `[sp-104]` |
| `ldr` at 0x63d34 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63d3c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63d44 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x63d48 | store | 4 | `(unknown)` | `[sp-108]` |
| `str` at 0x63d50 | store | 4 | `sp -0x90` | `0x1` |
| `str` at 0x63d58 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x63d60 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x63d68 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x63d70 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63d80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63d84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63d88 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x63da4 | load | 4 | `abs 0x63adc` | `-` |
| `str` at 0x63db0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63dbc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63dc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63dcc | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x63dd0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63dd4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63dec | load | 4 | `abs 0x63abc` | `-` |
| `ldr` at 0x63df4 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63df8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63e04 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63e0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63e10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63e14 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x63e18 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63e1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63e34 | load | 4 | `abs 0x63ac0` | `-` |
| `ldr` at 0x63e3c | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63e40 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63e4c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63e54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63e58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63e5c | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x63e60 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63e64 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63e7c | load | 4 | `abs 0x63ac4` | `-` |
| `ldr` at 0x63e84 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63e88 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63e94 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63e9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63ea0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63ea4 | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x63ea8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63eac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63ec4 | load | 4 | `abs 0x63ac8` | `-` |
| `ldr` at 0x63ecc | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63ed0 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63edc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63ee4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63ee8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63eec | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x63ef0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63ef4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63f0c | load | 4 | `abs 0x63acc` | `-` |
| `ldr` at 0x63f14 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63f18 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63f20 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63f24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63f28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63f2c | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0x63f30 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63f34 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63f4c | load | 4 | `abs 0x63ad0` | `-` |
| `ldr` at 0x63f54 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63f58 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63f64 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63f74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63f78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63f7c | load | 4 | `[sp-120] +0x1c` | `-` |
| `ldr` at 0x63f98 | load | 4 | `abs 0x63adc` | `-` |
| `str` at 0x63fa4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63fb0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63fb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63fbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63fc0 | load | 4 | `[sp-120] +0x28` | `-` |
| `ldr` at 0x63fc4 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x63fc8 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x63fe0 | load | 4 | `abs 0x63ad4` | `-` |
| `ldr` at 0x63fe8 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x63fec | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63ff8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x64000 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64004 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64008 | load | 4 | `[sp-120] +0x24` | `-` |
| `ldr` at 0x6400c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x64010 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x64028 | load | 4 | `abs 0x63ad8` | `-` |
| `ldr` at 0x64030 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x64034 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x64040 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x64050 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x64054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64058 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x64074 | load | 4 | `abs 0x63adc` | `-` |
| `str` at 0x64080 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x64088 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x640a0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x640a4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x640b0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x640b8 | store | 4 | `sp -0x90` | `[sp-144]-1` |
| `ldr` at 0x640c4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x640d0 | load | 4 | `abs 0x63ae0` | `-` |
| `ldr` at 0x640d8 | load | 4 | `abs 0x63ae4` | `-` |
| `str` at 0x640dc | store | 4 | `sp -0x98` | `0xfffff378` |
| `ldr` at 0x640e4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x640ec | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `ldr` at 0x640f4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x64108 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6410c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64110 | load | 4 | `abs 0x63aec` | `-` |
| `ldr` at 0x64130 | load | 4 | `abs 0x63ae8` | `-` |
| `ldr` at 0x64138 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x6413c | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x64144 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x6414c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x64150 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64154 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64158 | load | 4 | `abs 0x63aec` | `-` |
| `ldr` at 0x6415c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x64160 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x64178 | load | 4 | `abs 0x63af4` | `-` |
| `ldr` at 0x64180 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x64184 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x6418c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64190 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64194 | load | 4 | `abs 0x63af0` | `-` |
| `ldr` at 0x641b4 | load | 4 | `abs 0x63af4` | `-` |
| `ldr` at 0x641bc | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x641c0 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x641cc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x641d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x641d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x641dc | load | 4 | `[sp-120] +0x20` | `-` |
| `ldr` at 0x641e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x641e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x641fc | load | 4 | `abs 0x63af8` | `-` |
| `ldr` at 0x64204 | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x64208 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `str` at 0x64214 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x64218 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6421c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6422c | load | 4 | `abs 0x63afc` | `-` |
| `ldr` at 0x64230 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64234 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64238 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x6423c | load | 4 | `abs 0x63afc` | `-` |
| `ldr` at 0x64240 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64254 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64258 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x6425c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64264 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6426c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64274 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64278 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6427c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x64294 | load | 4 | `abs 0x63b00` | `-` |
| `ldr` at 0x6429c | load | 4 | `abs 0x63b04` | `-` |
| `str` at 0x642a0 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x642a8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x642b0 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `ldr` at 0x642b8 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x642bc | store | 4 | `sp -0xb8` | `[sp-152]` |
| `ldr` at 0x642c0 | load | 4 | `sp -0xb8` | `-` |
| `ldmdb` at 0x642c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x642c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x642c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x642c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62b34 | `(unknown callee) 0x2079c` | `0x10d444` | `[sp-188]` | `[sp-188]` | `?` |
| `bl` at 0x62c3c | `(unknown callee) 0xe99c4` | `?` | `[sp-188]` | `[sp-188]` | `?` |
| `bl` at 0x62c5c | `(unknown callee) 0x2079c` | `0x10d478` | `[sp-28]` | `?` | `ret(0xe99c4)` |
| `mov` at 0x62cb0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]]` | `sp-64` | `sp-68` |
| `bl` at 0x62cc4 | `(unknown callee) 0x2079c` | `0x10d498` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62d14 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `sp-56` | `sp-60` |
| `mov` at 0x62d64 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `0x0` | `0x0` |
| `bl` at 0x62d78 | `(unknown callee) 0x2079c` | `0x10d4b8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62dac | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62dc0 | `(unknown callee) 0x2079c` | `0x10d794` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62df4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62e08 | `(unknown callee) 0x2079c` | `0x10d7b4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62e3c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62e50 | `(unknown callee) 0x2079c` | `0x10d7d4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62e84 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+20]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62e98 | `(unknown callee) 0x2079c` | `0x10d7f4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62ec4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62ed8 | `(unknown callee) 0x2079c` | `0x10d4d8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62f0c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+28]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62f20 | `(unknown callee) 0x2079c` | `0x10d4f8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62f54 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62f68 | `(unknown callee) 0x2079c` | `0x10d518` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62ff0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `?` | `[sp-76]` |
| `bl` at 0x63004 | `(unknown callee) 0x2079c` | `0x10d534` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6305c | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `sp-72` | `sp-76` |
| `bl` at 0x63070 | `(unknown callee) 0x2079c` | `0x10d554` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x630e8 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x630fc | `(unknown callee) 0x2079c` | `0x10d57c` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x63290 | `(unknown callee) 0x2079c` | `0x10d814` | `[sp-48]` | `?` | `[0x1491c8]` |
| `bl` at 0x6329c | `(unknown callee) 0x2079c` | `0x10d820` | `[sp-52]` | `?` | `?` |
| `bl` at 0x632a8 | `(unknown callee) 0x2079c` | `0x10d82c` | `[sp-32]` | `?` | `?` |
| `bl` at 0x632b4 | `(unknown callee) 0x2079c` | `0x10d838` | `[sp-36]` | `?` | `?` |
| `bl` at 0x632cc | `(unknown callee) 0x2079c` | `0x10d844` | `?` | `?` | `?` |
| `bl` at 0x632e4 | `(unknown callee) 0x2079c` | `0x10d850` | `?` | `?` | `?` |
| `mov` at 0x634b0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x634c4 | `(unknown callee) 0x2079c` | `0x10d5a4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6357c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63590 | `(unknown callee) 0x2079c` | `0x10d5c4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63600 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+16]` | `0x0` | `0x0` |
| `bl` at 0x63614 | `(unknown callee) 0x2079c` | `0x10d85c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63668 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+20]` | `?` | `?` |
| `bl` at 0x6367c | `(unknown callee) 0x2079c` | `0x10d880` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x636d0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+16]` | `?` | `?` |
| `bl` at 0x636e4 | `(unknown callee) 0x2079c` | `0x10d85c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63718 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+20]` | `0x0` | `0x0` |
| `bl` at 0x6372c | `(unknown callee) 0x2079c` | `0x10d880` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6376c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+4]` | `?` | `?` |
| `bl` at 0x63780 | `(unknown callee) 0x2079c` | `0x10d604` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x637ec | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+4]` | `sp-72` | `sp-76` |
| `bl` at 0x63800 | `(unknown callee) 0x2079c` | `0x10d628` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x63830 | `(unknown callee) 0x6444c` | `0xa` | `?` | `?` | `[sp-140]` |
| `bl` at 0x63848 | `(unknown callee) 0x2079c` | `0x10d640` | `?` | `?` | `?` |
| `mov` at 0x638b4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-72` | `sp-76` |
| `bl` at 0x638c8 | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x638f8 | `(unknown callee) 0x6444c` | `0xa` | `?` | `?` | `[sp-140]` |
| `bl` at 0x63910 | `(unknown callee) 0x2079c` | `0x10d66c` | `?` | `?` | `?` |
| `mov` at 0x63954 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `sp-80` | `sp-84` |
| `bl` at 0x63968 | `(unknown callee) 0x202d0` | `0x10d680` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x639a0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `sp-88` | `sp-92` |
| `bl` at 0x639b4 | `(unknown callee) 0x202d0` | `0x10d680` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63b30 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `sp-96` | `sp-100` |
| `bl` at 0x63b44 | `(unknown callee) 0x202d0` | `0x10d680` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63b7c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `sp-104` | `sp-108` |
| `bl` at 0x63b90 | `(unknown callee) 0x202d0` | `0x10d680` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x63bb4 | `(unknown callee) 0x202d0` | `0x10d694` | `?` | `?` | `?` |
| `bl` at 0x63bd8 | `(unknown callee) 0x202d0` | `0x10d6a4` | `?` | `?` | `?` |
| `bl` at 0x63c08 | `(unknown callee) 0x2079c` | `0x10d6b8` | `?` | `?` | `?` |
| `bl` at 0x63cbc | `(unknown callee) 0x2079c` | `0x10d8a4` | `[sp-72]` | `[sp-76]` | `?` |
| `mov` at 0x63d94 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-168` | `sp-172` |
| `bl` at 0x63da8 | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63ddc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63df0 | `(unknown callee) 0x2079c` | `0x10d6d0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63e24 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63e38 | `(unknown callee) 0x2079c` | `0x10d8c8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63e6c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63e80 | `(unknown callee) 0x2079c` | `0x10d8dc` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63eb4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63ec8 | `(unknown callee) 0x2079c` | `0x10d8f0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63efc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+20]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63f10 | `(unknown callee) 0x2079c` | `0x10d904` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63f3c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63f50 | `(unknown callee) 0x2079c` | `0x10d6e4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63f88 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+28]` | `sp-176` | `sp-180` |
| `bl` at 0x63f9c | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63fd0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+40]` | `[sp-176]` | `[sp-180]` |
| `bl` at 0x63fe4 | `(unknown callee) 0x2079c` | `0x10d6fc` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x64018 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+36]` | `[sp-168]` | `[sp-172]` |
| `bl` at 0x6402c | `(unknown callee) 0x2079c` | `0x10d718` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x64064 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-168` | `sp-172` |
| `bl` at 0x64078 | `(unknown callee) 0x2079c` | `0x10d654` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x640d4 | `(unknown callee) 0x2079c` | `0x10d730` | `?` | `?` | `[sp-140]` |
| `mov` at 0x64120 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90010000` | `sp-72` | `sp-76` |
| `bl` at 0x64134 | `(unknown callee) 0x2079c` | `0x10d918` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x64168 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90010000` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x6417c | `(unknown callee) 0x2079c` | `0x10d930` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x641a4 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90071100` | `0x0` | `0x0` |
| `bl` at 0x641b8 | `(unknown callee) 0x2079c` | `0x10d930` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x641ec | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x64200 | `(unknown callee) 0x2079c` | `0x10d744` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x64284 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x64298 | `(unknown callee) 0x2079c` | `0x10d768` | `?` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x62ab8 | `[0x1491c8] == 0x0` | `[0x1491c8] != 0x0` |
| `beq` at 0x62b04 | `[sp-188] == 0x4` | `[sp-188] != 0x4` |
| `bgt` at 0x62b10 | `[sp-188] > 0x4 (signed, 32-bit)` | `[sp-188] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x62b1c | `[sp-188] == 0x2` | `[sp-188] != 0x2` |
| `beq` at 0x62b2c | `[sp-188] == 0x7` | `[sp-188] != 0x7` |
| `beq` at 0x62b54 | `[sp-188] == 0x4` | `[sp-188] != 0x4` |
| `bgt` at 0x62b60 | `[sp-188] > 0x4 (signed, 32-bit)` | `[sp-188] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x62b6c | `[sp-188] == 0x2` | `[sp-188] != 0x2` |
| `beq` at 0x62b7c | `[sp-188] == 0x7` | `[sp-188] != 0x7` |
| `beq` at 0x62c50 | `ret(0xe99c4) == 0x0` | `ret(0xe99c4) != 0x0` |
| `bne` at 0x62c6c | `[sp-152] != 0xfffff7c6` | `[sp-152] == 0xfffff7c6` |
| `beq` at 0x62cbc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62d20 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62d70 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62db8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62e00 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62e48 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62e90 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62ed0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62f18 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62f60 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62f80 | `[sp-156] != 0x2` | `[sp-156] == 0x2` |
| `bne` at 0x62fa4 | `[sp-156] != 0x4` | `[sp-156] == 0x4` |
| `beq` at 0x62ffc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x63028 | `C == 0` | `not (C == 0)` |
| `beq` at 0x63068 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x630f4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blt` at 0x63130 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x63188 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63194 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x631a4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x631b8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x631cc | `[sp-156] == 0x4` | `[sp-156] != 0x4` |
| `bgt` at 0x631d8 | `[sp-156] > 0x4 (signed, 32-bit)` | `[sp-156] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x631e4 | `[sp-156] == 0x2` | `[sp-156] != 0x2` |
| `beq` at 0x631f4 | `[sp-156] == 0x7` | `[sp-156] != 0x7` |
| `beq` at 0x63214 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63284 | `[0x1491c8] == 0x0` | `[0x1491c8] != 0x0` |
| `bne` at 0x63344 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x634bc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63588 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x635b8 | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `beq` at 0x635d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6360c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63674 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x636dc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63724 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63778 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x637a8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x637bc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x637f8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x63828 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x63840 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x63870 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x63884 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x638c0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x638f0 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x63908 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x63928 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x63960 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x639ac | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63b3c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63b88 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63bac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63bd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63bf4 | `? == 0xb4` | `? != 0xb4` |
| `beq` at 0x63c6c | `[sp-88] == ?` | `[sp-88] != ?` |
| `beq` at 0x63c88 | `[sp-92] == ?` | `[sp-92] != ?` |
| `beq` at 0x63ce4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63da0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63de8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63e30 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63e78 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63ec0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63f08 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63f48 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63f94 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63fdc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x64024 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x64070 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x640ac | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x640c0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x640cc | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x640fc | `[sp-156] != 0x4` | `[sp-156] == 0x4` |
| `beq` at 0x6412c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x64174 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x641b0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x641f8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x64224 | `C == 0` | `not (C == 0)` |
| `beq` at 0x64290 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x642c4 | `r0` | `?` |

## `SMPInit`

- `SMPInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66ba0 to 0x67078.
  Established. Included four-byte assignments write `0x0` at `0x2b1a8c` (`str` at 0x66bfc), `0x3` at
  `0x2b1a90` (`str` at 0x66c08), and `0x2b1a94` at `0x2b1ae4` (`str` at 0x66c1c). The branch takes
  `[sp-24] != 0x0` to `0x66ef8` and falls through when zero (`bne` at 0x66e7c). The unnamed
  `0xbb094` call carries `0x2b215c`, `0x2b2198`, `0x100`, `0x2b2298` (`bl` at 0x66dbc).
  Initialisation field roles and transport semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x66bb0 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x66bb4 | load | 4 | `abs 0x66fe0` | `-` |
| `ldr` at 0x66bb8 | load | 4 | `abs 0x1492ac` | `-` |
| `ldr` at 0x66bc4 | load | 4 | `abs 0x66fe4` | `-` |
| `str` at 0x66bc8 | store | 4 | `sp -0x40` | `0x10f4c0` |
| `ldr` at 0x66bd0 | load | 4 | `abs 0x66fe8` | `-` |
| `ldr` at 0x66be0 | load | 4 | `abs 0x66fe0` | `-` |
| `ldr` at 0x66be4 | load | 4 | `abs 0x66fe0` | `-` |
| `ldr` at 0x66be8 | load | 4 | `abs 0x1492ac` | `-` |
| `str` at 0x66bf0 | store | 4 | `abs 0x1492ac` | `[0x1492ac]+1` |
| `ldr` at 0x66bf4 | load | 4 | `abs 0x66fec` | `-` |
| `str` at 0x66bfc | store | 4 | `abs 0x2b1a8c` | `0x0` |
| `ldr` at 0x66c00 | load | 4 | `abs 0x66fec` | `-` |
| `str` at 0x66c08 | store | 4 | `abs 0x2b1a90` | `0x3` |
| `str` at 0x66c10 | store | 4 | `sp -0x24` | `0x14` |
| `ldr` at 0x66c14 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66c18 | load | 4 | `abs 0x66ff0` | `-` |
| `str` at 0x66c1c | store | 4 | `abs 0x2b1ae4` | `0x2b1a94` |
| `ldr` at 0x66c20 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66c24 | load | 4 | `abs 0x2b1ae4` | `-` |
| `strb` at 0x66c2c | store | 1 | `[0x2b1ae4] +0x0` | `0x0` |
| `ldr` at 0x66c30 | load | 4 | `abs 0x66ff4` | `-` |
| `str` at 0x66c40 | store | 4 | `abs 0x1492e4` | `?` |
| `ldr` at 0x66c44 | load | 4 | `abs 0x66fec` | `-` |
| `str` at 0x66c4c | store | 4 | `abs 0x2b2148` | `0x0` |
| `ldr` at 0x66c50 | load | 4 | `abs 0x66fec` | `-` |
| `str` at 0x66c58 | store | 4 | `abs 0x2b214c` | `0x0` |
| `ldr` at 0x66c5c | load | 4 | `abs 0x66ff8` | `-` |
| `ldr` at 0x66c60 | load | 4 | `abs 0x66ffc` | `-` |
| `str` at 0x66c70 | store | 4 | `sp -0x1c` | `ret(0xb5f4c)` |
| `ldr` at 0x66c74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66c80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66c8c | load | 4 | `abs 0x67000` | `-` |
| `str` at 0x66c90 | store | 4 | `sp -0x40` | `0x10f500` |
| `str` at 0x66c94 | store | 4 | `sp -0x3c` | `ret(0x21704)` |
| `ldr` at 0x66c9c | load | 4 | `abs 0x66fe8` | `-` |
| `ldr` at 0x66cac | load | 4 | `abs 0x67004` | `-` |
| `ldr` at 0x66cb0 | load | 4 | `abs 0x67008` | `-` |
| `str` at 0x66cc0 | store | 4 | `sp -0x1c` | `ret(0xb5f4c)` |
| `ldr` at 0x66cc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66cdc | load | 4 | `abs 0x6700c` | `-` |
| `str` at 0x66ce0 | store | 4 | `sp -0x40` | `0x10f52c` |
| `str` at 0x66ce4 | store | 4 | `sp -0x3c` | `ret(0x21704)` |
| `ldr` at 0x66cec | load | 4 | `abs 0x66fe8` | `-` |
| `ldr` at 0x66cfc | load | 4 | `abs 0x67010` | `-` |
| `ldr` at 0x66d00 | load | 4 | `abs 0x67014` | `-` |
| `str` at 0x66d10 | store | 4 | `sp -0x1c` | `ret(0xb5f4c)` |
| `ldr` at 0x66d14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66d20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66d2c | load | 4 | `abs 0x67018` | `-` |
| `str` at 0x66d30 | store | 4 | `sp -0x40` | `0x10f55c` |
| `str` at 0x66d34 | store | 4 | `sp -0x3c` | `ret(0x21704)` |
| `ldr` at 0x66d3c | load | 4 | `abs 0x66fe8` | `-` |
| `ldr` at 0x66d4c | load | 4 | `abs 0x6701c` | `-` |
| `ldr` at 0x66d50 | load | 4 | `abs 0x67020` | `-` |
| `ldr` at 0x66d60 | load | 4 | `abs 0x67024` | `-` |
| `ldr` at 0x66d64 | load | 4 | `abs 0x67028` | `-` |
| `str` at 0x66d78 | store | 4 | `sp -0x40` | `0x100` |
| `str` at 0x66d80 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x66d88 | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0x66d90 | store | 4 | `sp -0x34` | `0x10` |
| `str` at 0x66d98 | store | 4 | `sp -0x30` | `0x410` |
| `ldr` at 0x66d9c | load | 4 | `abs 0x6702c` | `-` |
| `str` at 0x66da0 | store | 4 | `sp -0x2c` | `0x690e4` |
| `str` at 0x66da8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x66dac | load | 4 | `abs 0x67030` | `-` |
| `ldr` at 0x66db0 | load | 4 | `abs 0x67034` | `-` |
| `ldr` at 0x66db8 | load | 4 | `abs 0x67038` | `-` |
| `ldr` at 0x66dc0 | load | 4 | `abs 0x66fec` | `-` |
| `str` at 0x66ddc | store | 4 | `sp -0x20` | `ret(0x75194)` |
| `ldr` at 0x66de0 | load | 4 | `abs 0x6703c` | `-` |
| `ldr` at 0x66de4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66de8 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `str` at 0x66dec | store | 4 | `abs 0x149370` | `[ret(0x75194)+8]` |
| `ldr` at 0x66df0 | load | 4 | `abs 0x67040` | `-` |
| `ldr` at 0x66df4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66df8 | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0x66dfc | store | 4 | `abs 0x149374` | `[[sp-32]+12]` |
| `ldr` at 0x66e00 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66e04 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66e08 | load | 4 | `abs 0x2b1800` | `-` |
| `str` at 0x66e20 | store | 4 | `abs 0x2b2150` | `?` |
| `ldr` at 0x66e24 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66e28 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66e2c | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x66e34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66e38 | load | 4 | `[sp-32] +0x14` | `-` |
| `str` at 0x66e48 | store | 4 | `abs 0x2b2150` | `?` |
| `ldr` at 0x66e4c | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66e50 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66e54 | load | 4 | `abs 0x2b1800` | `-` |
| `str` at 0x66e6c | store | 4 | `abs 0x2b2158` | `?` |
| `ldr` at 0x66e74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66e84 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x66e88 | load | 4 | `abs 0x67044` | `-` |
| `str` at 0x66e8c | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x66e94 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x66e9c | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x66ea4 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x66ea8 | load | 4 | `abs 0x67048` | `-` |
| `ldr` at 0x66eac | load | 4 | `abs 0x6704c` | `-` |
| `ldr` at 0x66eb0 | load | 4 | `abs 0x67050` | `-` |
| `str` at 0x66ec0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x66ec4 | load | 4 | `abs 0x67044` | `-` |
| `str` at 0x66ec8 | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x66ed0 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x66ed8 | store | 4 | `sp -0x34` | `0x28` |
| `str` at 0x66ee0 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x66ee4 | load | 4 | `abs 0x67054` | `-` |
| `ldr` at 0x66ee8 | load | 4 | `abs 0x67058` | `-` |
| `ldr` at 0x66eec | load | 4 | `abs 0x6705c` | `-` |
| `str` at 0x66efc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x66f00 | load | 4 | `abs 0x67044` | `-` |
| `str` at 0x66f04 | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x66f0c | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x66f14 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x66f1c | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x66f20 | load | 4 | `abs 0x67060` | `-` |
| `ldr` at 0x66f24 | load | 4 | `abs 0x67064` | `-` |
| `ldr` at 0x66f28 | load | 4 | `abs 0x67068` | `-` |
| `ldr` at 0x66f34 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66f38 | load | 4 | `abs 0x6706c` | `-` |
| `ldr` at 0x66f40 | load | 4 | `abs 0x67010` | `-` |
| `str` at 0x66f4c | store | 4 | `sp -0x1c` | `ret(0xb4b18)` |
| `ldr` at 0x66f50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66f5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66f68 | load | 4 | `abs 0x67070` | `-` |
| `str` at 0x66f6c | store | 4 | `sp -0x40` | `0x10f5bc` |
| `str` at 0x66f70 | store | 4 | `sp -0x3c` | `ret(0x21704)` |
| `ldr` at 0x66f78 | load | 4 | `abs 0x66fe8` | `-` |
| `ldr` at 0x66f88 | load | 4 | `abs 0x66fec` | `-` |
| `ldr` at 0x66f8c | load | 4 | `abs 0x6706c` | `-` |
| `ldr` at 0x66f94 | load | 4 | `abs 0x66ff8` | `-` |
| `str` at 0x66fa0 | store | 4 | `sp -0x1c` | `ret(0xb4b18)` |
| `ldr` at 0x66fa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66fb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66fbc | load | 4 | `abs 0x67074` | `-` |
| `str` at 0x66fc0 | store | 4 | `sp -0x40` | `0x10f5e0` |
| `str` at 0x66fc4 | store | 4 | `sp -0x3c` | `ret(0x21704)` |
| `ldr` at 0x66fcc | load | 4 | `abs 0x66fe8` | `-` |
| `ldmdb` at 0x66fdc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x66fdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x66fdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x66fdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x66bdc | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x62` | `0x0` |
| `bl` at 0x66c34 | `(unknown callee) 0x4f4d0` | `?` | `?` | `[0x2b1ae4]` | `0x0` |
| `bl` at 0x66c68 | `(unknown callee) 0xb5f4c` | `0x2b1810` | `0x10f4f4` | `0x19` | `0x0` |
| `bl` at 0x66c84 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0x66ca8 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x80` | `0x0` |
| `bl` at 0x66cb8 | `(unknown callee) 0xb5f4c` | `0x2b18a8` | `0x10f520` | `0x19` | `?` |
| `bl` at 0x66cd4 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0x66cf8 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x83` | `0x0` |
| `bl` at 0x66d08 | `(unknown callee) 0xb5f4c` | `0x2b1940` | `0x10f550` | `0x19` | `?` |
| `bl` at 0x66d24 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0x66d48 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x86` | `0x0` |
| `bl` at 0x66d5c | `(unknown callee) 0x229b0` | `0x2b19d8` | `0x10f580` | `0x1` | `0x9` |
| `bl` at 0x66d70 | `(unknown callee) 0x229b0` | `0x2b1a2c` | `0x10f58c` | `0x1` | `0x9` |
| `bl` at 0x66dbc | `(unknown callee) 0xbb094` | `0x2b215c` | `0x2b2198` | `0x100` | `0x2b2298` |
| `bl` at 0x66dcc | `(unknown callee) 0xb5e34` | `0x2b1800` | `0x2` | `0x0` | `?` |
| `bl` at 0x66dd4 | `(unknown callee) 0x75194` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x66e70 | `(unknown callee) 0x69370` | `ret(0x75194)` | `0x2b1800` | `0x2b1800` | `?` |
| `bl` at 0x66eb8 | `(unknown callee) 0x22480` | `0x2b1a80` | `0x10f598` | `0x67154` | `0x0` |
| `bl` at 0x66ef4 | `(unknown callee) 0x22480` | `0x2b1a84` | `0x10f5a4` | `0x67634` | `0x0` |
| `bl` at 0x66f30 | `(unknown callee) 0x22480` | `0x2b1a88` | `0x10f5b0` | `0x68358` | `0x0` |
| `bl` at 0x66f44 | `(unknown callee) 0xb4b18` | `0x2b1800` | `0xffff002` | `0x1` | `0x2b1940` |
| `bl` at 0x66f60 | `(unknown callee) 0x21704` | `ret(0xb4b18)` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0x66f84 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0xd0` | `0x0` |
| `bl` at 0x66f98 | `(unknown callee) 0xb4b18` | `0x2b1800` | `0xffff002` | `0x2` | `0x2b1810` |
| `bl` at 0x66fb4 | `(unknown callee) 0x21704` | `ret(0xb4b18)` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0x66fd8 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0xdc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x66bc0 | `[0x1492ac] == 0x0` | `[0x1492ac] != 0x0` |
| `beq` at 0x66c7c | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0x66ccc | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0x66d1c | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `bne` at 0x66e7c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x66f58 | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |
| `beq` at 0x66fac | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x66fdc | `r0` | `?` |

## `SMPLock`

- `SMPLock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67078 to 0x67108.
  Established. A four-byte assignment at `0x1492b0` retains `ret(0x23f54)` (`str` at 0x670d8). The
  prior branch takes that call result nonzero to `0x670b4` and falls through when zero (`bne` at
  0x670a4). The unnamed `0xcade4` call carries `[0x1492b0]`, `0x1492e8`, `0x1492b0`, `0x1492b0`
  (`bl` at 0x670e8). Lock ownership and synchronization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67088 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6708c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67098 | store | 4 | `sp -0x18` | `ret(0x23f54)` |
| `ldr` at 0x6709c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x670ac | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x670b4 | load | 4 | `abs 0x670fc` | `-` |
| `ldr` at 0x670b8 | load | 4 | `abs 0x2b180c` | `-` |
| `ldr` at 0x670c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x670c8 | store | 4 | `sp -0x1c` | `ret(0x23f54)` |
| `ldr` at 0x670d0 | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x670d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x670d8 | store | 4 | `abs 0x1492b0` | `ret(0x23f54)` |
| `ldr` at 0x670dc | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x670e0 | load | 4 | `abs 0x1492b0` | `-` |
| `ldr` at 0x670e4 | load | 4 | `abs 0x67104` | `-` |
| `ldr` at 0x670ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x670f0 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x670f4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x670f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x670f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x670f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67090 | `(unknown callee) 0x23f54` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x670e8 | `(unknown callee) 0xcade4` | `[0x1492b0]` | `0x1492e8` | `0x1492b0` | `0x1492b0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x670a4 | `ret(0x23f54) != 0x0` | `ret(0x23f54) == 0x0` |
| `bne` at 0x670c0 | `[0x2b180c] != 0x0` | `[0x2b180c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x670f8 | `r0` | `?` |

## `SMPUnlock`

- `SMPUnlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67108 to 0x67154.
  Established. The nonzero word at `0x1492b0` takes the branch to `0x67128`, while zero falls
  through (`bne` at 0x67120). A later four-byte assignment clears that absolute word (`str` at
  0x67140). The unnamed `0xcae20` call uses `[0x1492b0]`, `0x1492e8`, `r2`, `0x1492b0` (`bl` at
  0x67134). Lock release and wakeup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x67114 | load | 4 | `abs 0x6714c` | `-` |
| `ldr` at 0x67118 | load | 4 | `abs 0x1492b0` | `-` |
| `ldr` at 0x67128 | load | 4 | `abs 0x6714c` | `-` |
| `ldr` at 0x6712c | load | 4 | `abs 0x1492b0` | `-` |
| `ldr` at 0x67130 | load | 4 | `abs 0x67150` | `-` |
| `ldr` at 0x67138 | load | 4 | `abs 0x6714c` | `-` |
| `str` at 0x67140 | store | 4 | `abs 0x1492b0` | `0x0` |
| `ldmdb` at 0x67148 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x67148 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x67148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67134 | `(unknown callee) 0xcae20` | `[0x1492b0]` | `0x1492e8` | `r2` | `0x1492b0` |
| `bl` at 0x67144 | `(unknown callee) 0x23fa0` | `ret(0xcae20)` | `?` | `0x1492b0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x67120 | `[0x1492b0] != 0x0` | `[0x1492b0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x67148 | `r0` | `?` |

## `SMPConsoleSubchannelDefault`

- `SMPConsoleSubchannelDefault`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6856c to 0x685bc.
  Established. The branch takes `[0x143574] == 0x0` to `0x685ac`, bypassing the call on that edge
  (`beq` at 0x6858c). The nonzero fall-through reaches the unnamed `0x685bc` call with `r0`, `r1`,
  `r2`, `[0x143574]` (`bl` at 0x68594). A four-byte assignment at `0x143574` writes `0x1` (`str` at
  0x685a8). Default subchannel selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6857c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x68580 | load | 4 | `abs 0x685b8` | `-` |
| `ldr` at 0x68584 | load | 4 | `abs 0x143574` | `-` |
| `ldr` at 0x68590 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6859c | store | 4 | `sp -0x14` | `ret(0x685bc)` |
| `ldr` at 0x685a0 | load | 4 | `abs 0x685b8` | `-` |
| `str` at 0x685a8 | store | 4 | `abs 0x143574` | `0x1` |
| `ldr` at 0x685ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x685b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x685b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x685b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68594 | `(unknown callee) 0x685bc` | `r0` | `r1` | `r2` | `[0x143574]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6858c | `[0x143574] == 0x0` | `[0x143574] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x685b4 | `r0` | `?` |

## `SMPConsoleSubchannel`

- `SMPConsoleSubchannel`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x685bc to 0x68774.
  Established. A four-byte store clears absolute `0x143574` (`str` at 0x685d8). The equality branch
  takes `[sp-20] == [ret(0x75194)+20]` to `0x6861c` and falls through for unequal values (`beq` at
  0x685fc). A subsequent four-byte assignment at `ret(0x75194) +0x14` retains `[sp-20]` (`str` at
  0x68608). Console subchannel field encoding and update effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x685cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x685d0 | load | 4 | `abs 0x68764` | `-` |
| `str` at 0x685d8 | store | 4 | `abs 0x143574` | `0x0` |
| `str` at 0x685e8 | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `ldr` at 0x685ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x685f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x685f4 | load | 4 | `ret(0x75194) +0x14` | `-` |
| `ldr` at 0x68600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68608 | store | 4 | `ret(0x75194) +0x14` | `[sp-20]` |
| `ldr` at 0x6861c | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x68620 | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x68630 | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x68634 | load | 4 | `abs 0x2b2150` | `-` |
| `str` at 0x68650 | store | 4 | `sp -0x1c` | `ret(0x74f94)` |
| `ldr` at 0x68654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68660 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68664 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x6866c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68670 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68674 | load | 4 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x68678 | load | 4 | `ret(0x74f94) +0x3c` | `-` |
| `ldr` at 0x68684 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68688 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x68690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68694 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x686a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x686a8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x686c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x686c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x686d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x686e0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x686e4 | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x686e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x686ec | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x686f8 | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x686fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68700 | store | 4 | `abs 0x2b2150` | `[sp-32]` |
| `ldr` at 0x68704 | load | 4 | `abs 0x6876c` | `-` |
| `ldr` at 0x68708 | load | 4 | `abs 0x149298` | `-` |
| `ldr` at 0x68714 | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x68718 | load | 4 | `abs 0x2b1a90` | `-` |
| `ldr` at 0x68724 | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x68728 | load | 4 | `abs 0x2b1a90` | `-` |
| `ldr` at 0x68738 | load | 4 | `abs 0x68768` | `-` |
| `ldr` at 0x6873c | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x68748 | load | 4 | `abs 0x68770` | `-` |
| `ldr` at 0x68754 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68758 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x6875c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x68760 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x68760 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x68760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x685e0 | `(unknown callee) 0x75194` | `0x0` | `r1` | `0x143574` | `0x0` |
| `bl` at 0x68614 | `(unknown callee) 0x5f160` | `0x1` | `[sp-20]` | `[sp-20]` | `ret(0x75194)` |
| `bl` at 0x68618 | `(unknown callee) 0x757b0` | `ret(0x5f160)` | `?` | `?` | `?` |
| `bl` at 0x68648 | `(unknown callee) 0x74f94` | `?` | `?` | `?` | `?` |
| `bl` at 0x68740 | `(unknown callee) 0x6a080` | `[0x2b2150]` | `[[sp-24]+60]` | `0x2b1800` | `0x2b1800` |
| `bl` at 0x68750 | `(unknown callee) 0x23fbc` | `0x10f8c8` | `ret(0x6a080)` | `?` | `ret(0x6a080)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x685fc | `[sp-20] == [ret(0x75194)+20]` | `[sp-20] != [ret(0x75194)+20]` |
| `bne` at 0x6865c | `ret(0x74f94) != 0x0` | `ret(0x74f94) == 0x0` |
| `beq` at 0x68680 | `[[sp-24]+60] == [ret(0x74f94)+60]` | `[[sp-24]+60] != [ret(0x74f94)+60]` |
| `beq` at 0x686f4 | `[sp-32] == [0x2b2150]` | `[sp-32] != [0x2b2150]` |
| `beq` at 0x68710 | `[0x149298] == 0x0` | `[0x149298] != 0x0` |
| `beq` at 0x68720 | `[0x2b1a90] == 0x1` | `[0x2b1a90] != 0x1` |
| `beq` at 0x68730 | `[0x2b1a90] == 0x2` | `[0x2b1a90] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x68760 | `r0` | `[sp-20]` |

## `SMPSystemConsole`

- `SMPSystemConsole`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68774 to 0x688a4.
  Established. The four-byte assignment at `0x2b2150` writes `[sp-32]` (`str` at 0x68830). The
  equality branch takes `[sp-32] == [0x2b2150]` to `0x68884` and falls through on inequality (`beq`
  at 0x68824). The unnamed `0x6a080` call carries `[0x2b2150]`, an unknown second candidate,
  `0x2b1800`, `0x2b1800` (`bl` at 0x68870). Console routing fields and notification effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68784 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x68788 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6878c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x68790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6879c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x687ac | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x687c0 | store | 4 | `sp -0x24` | `ret(0x75194)` |
| `ldr` at 0x687c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x687d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x687f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x687f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x687fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6880c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x68814 | load | 4 | `abs 0x68898` | `-` |
| `ldr` at 0x68818 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6881c | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x68828 | load | 4 | `abs 0x68898` | `-` |
| `ldr` at 0x6882c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68830 | store | 4 | `abs 0x2b2150` | `[sp-32]` |
| `ldr` at 0x68834 | load | 4 | `abs 0x6889c` | `-` |
| `ldr` at 0x68838 | load | 4 | `abs 0x149298` | `-` |
| `ldr` at 0x68844 | load | 4 | `abs 0x68898` | `-` |
| `ldr` at 0x68848 | load | 4 | `abs 0x2b1a90` | `-` |
| `ldr` at 0x68854 | load | 4 | `abs 0x68898` | `-` |
| `ldr` at 0x68858 | load | 4 | `abs 0x2b1a90` | `-` |
| `ldr` at 0x68868 | load | 4 | `abs 0x68898` | `-` |
| `ldr` at 0x6886c | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x68878 | load | 4 | `abs 0x688a0` | `-` |
| `ldr` at 0x68884 | load | 4 | `abs 0x68898` | `-` |
| `ldr` at 0x68888 | load | 4 | `abs 0x2b2150` | `-` |
| `str` at 0x6888c | store | 4 | `sp -0x28` | `[0x2b2150]` |
| `ldr` at 0x68890 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x68894 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x68894 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x68894 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x687b8 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r1` |
| `bl` at 0x68810 | `(unknown callee) 0x75790` | `ret(0x75194)` | `?` | `?` | `?` |
| `bl` at 0x68870 | `(unknown callee) 0x6a080` | `[0x2b2150]` | `?` | `0x2b1800` | `0x2b1800` |
| `bl` at 0x68880 | `(unknown callee) 0x23fbc` | `0x10f8f0` | `ret(0x6a080)` | `?` | `ret(0x6a080)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x68798 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x687a4 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x68824 | `[sp-32] == [0x2b2150]` | `[sp-32] != [0x2b2150]` |
| `beq` at 0x68840 | `[0x149298] == 0x0` | `[0x149298] != 0x0` |
| `beq` at 0x68850 | `[0x2b1a90] == 0x1` | `[0x2b1a90] != 0x1` |
| `beq` at 0x68860 | `[0x2b1a90] == 0x2` | `[0x2b1a90] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x68894 | `r0` | `?` |

## `SMPConsoleReset`

- `SMPConsoleReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x688a4 to 0x688fc.
  Established. The first explicit four-byte stack assignment writes `0x0` at `sp -0x14` (`str` at
  0x688b8). The unnamed `0x5f160` call has unknown first, third and fourth candidates with second
  candidate `0x0` (`bl` at 0x688dc). A subsequent four-byte store at `sp -0x14` writes `[sp-20]+1`
  (`str` at 0x688e8). Console reset effects and the loop predicate operands remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x688b8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x688bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x688cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x688e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x688e8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x688f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x688f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x688f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x688dc | `(unknown callee) 0x5f160` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x688f4 | `(unknown callee) 0x685bc` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x688c4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x688f8 | `r0` | `ret(0x685bc)` |

## `SMPCmdCheckBrickType`

- `SMPCmdCheckBrickType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8dbc0 to 0x8ded4.
  Established. The byte assignment at `sp -0x1d` retains `(ret(0x4f4d0) & 0xff)` (`strb` at
  0x8dbdc). The pointer test branch takes `[sp-24] != 0x0` to `0x8dc18`, while zero falls through
  (`bne` at 0x8dc08). Included four-byte writes at `[sp-28] +0x0` retain `0x0` and `ret(0x1b1b4)`
  (`str` at 0x8de8c, `str` at 0x8dea0). Brick filtering beyond the unresolved computed edge remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8dbd0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8dbd4 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x8dbdc | store | 1 | `sp -0x1d` | `(ret(0x4f4d0) & 0xff)` |
| `strb` at 0x8dbe4 | store | 1 | `sp -0x1e` | `0x0` |
| `str` at 0x8dbec | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x8dbf4 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x8dbfc | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x8dc00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dc10 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8dc18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8dc1c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x8dc2c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x8dc30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dc38 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrb` at 0x8dc3c | load | 1 | `sp -0x1d` | `-` |
| `ldrls` at 0x8dc48 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x8dde0 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x8dde4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x8dde8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8de04 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x8de0c | store | 4 | `sp -0x34` | `0x0` |
| `ldrb` at 0x8de10 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x8de1c | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x8de20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8de3c | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x8de40 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x8de44 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x8de48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8de4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8de58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8de5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8de68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8de6c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8de78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8de7c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8de84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8de8c | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x8de90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8dea0 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x8dea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8dea8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8deac | load | 4 | `abs 0x8ded0` | `-` |
| `str` at 0x8deb8 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x8dec4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8dec8 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x8decc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8decc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8decc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8decc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8dbd8 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x8ddf0 | `(unknown callee) 0x1b01c` | `[sp-24]` | `(ret(0x4f4d0) & 0xff)` | `?` | `(ret(0x4f4d0) & 0xff)` |
| `bl` at 0x8de28 | `(unknown callee) 0x1b01c` | `[sp-24]` | `[sp-30]:1` | `?` | `[sp-30]:1` |
| `bl` at 0x8de80 | `(unknown callee) 0x1b490` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x8de98 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x8deb0 | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x11b3c4` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8dc08 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x8dc24 | `[[sp-24]]:1 != 0x7e` | `[[sp-24]]:1 == 0x7e` |
| `ldrls` at 0x8dc48 | `(ret(0x4f4d0) & 0xff)-35 <= 0x59 (unsigned, 32-bit)` | `(ret(0x4f4d0) & 0xff)-35 > 0x59 (unsigned, 32-bit)` |
| `beq` at 0x8de18 | `[sp-30]:1 == 0x0` | `[sp-30]:1 != 0x0` |
| `beq` at 0x8de34 | `ret(0x1b01c) == 0x0` | `ret(0x1b01c) != 0x0` |
| `beq` at 0x8de54 | `[sp-36] == [sp-44]` | `[sp-36] != [sp-44]` |
| `beq` at 0x8de64 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8de74 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8decc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8dc48).

## `SMPCmdResponseDelay`

- `SMPCmdResponseDelay`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8ded4 to 0x8df9c.
  Established. The unnamed `0xb74d4` call uses `0x2b25a0`, `0xfffffd95`, `[[sp-28]]`, `[sp-36]`
  (`bl` at 0x8df60). A zero result takes the branch to `0x8df84`, and a nonzero result falls through
  (`beq` at 0x8df74). The zero-result path writes `0x0` at `sp -0x38` (`str` at 0x8df88). Response
  scheduling and delay units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8dee4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8dee8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8deec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x8df00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8df08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8df14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8df30 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x8df38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8df44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8df4c | store | 4 | `sp -0x3c` | `sp-52` |
| `ldr` at 0x8df50 | load | 4 | `abs 0x8df94` | `-` |
| `ldr` at 0x8df54 | load | 4 | `abs 0x8df98` | `-` |
| `ldr` at 0x8df58 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8df5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8df68 | store | 4 | `sp -0x20` | `ret(0xb74d4)` |
| `ldr` at 0x8df6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8df7c | store | 4 | `sp -0x38` | `?` |
| `str` at 0x8df88 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x8df8c | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x8df90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8df90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8df90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8df04 | `(unknown callee) 0x69908` | `sp-36` | `sp-52` | `r0` | `sp-36` |
| `bl` at 0x8df18 | `(unknown callee) 0x1b2bc` | `[sp-24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x8df40 | `(unknown callee) 0x69b84` | `sp-36` | `sp-52` | `[sp-24]` | `ret(0x1b2bc)` |
| `bl` at 0x8df60 | `(unknown callee) 0xb74d4` | `0x2b25a0` | `0xfffffd95` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8df10 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x8df74 | `ret(0xb74d4) == 0x0` | `ret(0xb74d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8df90 | `r0` | `[sp-56]` |

## `SMPCmdNoResponse`

- `SMPCmdNoResponse`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e454 to 0x8e4a0.
  Established. The unnamed `0x1b088` call has candidates `r0`, `0x1159e4`, `r2`, `r3` (`bl` at
  0x8e470); its nonzero result takes the branch to `0x8e48c`, and zero falls through (`bne` at
  0x8e47c). The zero-result fall-through writes `0x1` at `sp -0x18` (`str` at 0x8e484). The taken
  nonzero arm writes `0x0` at `sp -0x18` (`str` at 0x8e490). The meaning of suppressing a response
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e464 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8e468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e46c | load | 4 | `abs 0x8e49c` | `-` |
| `str` at 0x8e484 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x8e490 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x8e494 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8e498 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e498 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e498 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e470 | `(unknown callee) 0x1b088` | `r0` | `0x1159e4` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8e47c | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e498 | `r0` | `[sp-24]` |

## `SMPCmdPostProcess`

- `SMPCmdPostProcess`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e4a0 to 0x8e560.
  Established. Included four-byte assignments retain `[0x14a724]` at `[sp-24] +0x410` (`str` at
  0x8e534) and `0x11b5a8` at `[sp-20] +0x0` (`str` at 0x8e540). The early branch takes `ret(0x1b478) != 0x2` to `0x8e544` and falls through on equality (`bne` at 0x8e4dc). Command parsing and
  post-processing roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e4b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e4b4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8e4b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e4c4 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x8e4c8 | load | 4 | `abs 0x8e548` | `-` |
| `ldr` at 0x8e4e0 | load | 4 | `abs 0x8e54c` | `-` |
| `ldr` at 0x8e4e4 | load | 4 | `abs 0x14a70c` | `-` |
| `ldr` at 0x8e4f8 | load | 4 | `abs 0x8e550` | `-` |
| `ldr` at 0x8e514 | load | 4 | `abs 0x8e554` | `-` |
| `ldr` at 0x8e528 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e52c | load | 4 | `abs 0x8e558` | `-` |
| `ldr` at 0x8e530 | load | 4 | `abs 0x14a724` | `-` |
| `str` at 0x8e534 | store | 4 | `[sp-24] +0x410` | `[0x14a724]` |
| `ldr` at 0x8e538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e53c | load | 4 | `abs 0x8e55c` | `-` |
| `str` at 0x8e540 | store | 4 | `[sp-20] +0x0` | `0x11b5a8` |
| `ldmdb` at 0x8e544 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e544 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e544 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e4d0 | `(unknown callee) 0x1b478` | `[r0]` | `0x11b590` | `sp-56` | `sp-88` |
| `bl` at 0x8e4fc | `(unknown callee) 0x1b088` | `sp-56` | `0x11b59c` | `?` | `sp-56` |
| `bl` at 0x8e518 | `(unknown callee) 0x1b088` | `sp-88` | `0x11b5a0` | `?` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8e4dc | `ret(0x1b478) != 0x2` | `ret(0x1b478) == 0x2` |
| `beq` at 0x8e4ec | `[0x14a70c] == 0x0` | `[0x14a70c] != 0x0` |
| `bne` at 0x8e508 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0x8e524 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e544 | `r0` | `?` |

## `SMPCmdInit`

- `SMPCmdInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99514 to 0x997ac.
  Established. A four-byte assignment at absolute `0x14a55c` writes `[0x14a55c]+1` (`str` at
  0x9973c). The unsigned branch takes `ret(0x9adf8) <= 0x7ffff` to `0x996dc` and falls through above
  that bound (`bls` at 0x996cc). The unnamed `0x22480` call carries `0x2b26d8`, `0x11e4a4`,
  `0x9aab0`, `0x0` (`bl` at 0x9971c). Command task roles and stack field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x99524 | load | 4 | `abs 0x99744` | `-` |
| `ldr` at 0x99528 | load | 4 | `abs 0x14a55c` | `-` |
| `ldr` at 0x99534 | load | 4 | `abs 0x99748` | `-` |
| `str` at 0x99538 | store | 4 | `sp -0x2c` | `0x11e3b0` |
| `ldr` at 0x99540 | load | 4 | `abs 0x9974c` | `-` |
| `ldr` at 0x99544 | load | 4 | `abs 0x99750` | `-` |
| `ldr` at 0x99550 | load | 4 | `abs 0x99754` | `-` |
| `ldr` at 0x99560 | load | 4 | `abs 0x99758` | `-` |
| `ldr` at 0x99564 | load | 4 | `abs 0x9975c` | `-` |
| `str` at 0x99574 | store | 4 | `sp -0x14` | `ret(0xb5f4c)` |
| `ldr` at 0x99578 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99584 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99590 | load | 4 | `abs 0x99760` | `-` |
| `str` at 0x99594 | store | 4 | `sp -0x2c` | `0x11e3f8` |
| `str` at 0x99598 | store | 4 | `sp -0x28` | `ret(0x21704)` |
| `ldr` at 0x995a0 | load | 4 | `abs 0x9974c` | `-` |
| `ldr` at 0x995a4 | load | 4 | `abs 0x99764` | `-` |
| `ldr` at 0x995b0 | load | 4 | `abs 0x99754` | `-` |
| `ldr` at 0x995b4 | load | 4 | `abs 0x99768` | `-` |
| `ldr` at 0x995bc | load | 4 | `abs 0x99758` | `-` |
| `str` at 0x995c8 | store | 4 | `sp -0x14` | `ret(0xb4b18)` |
| `ldr` at 0x995cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x995d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x995e4 | load | 4 | `abs 0x9976c` | `-` |
| `str` at 0x995e8 | store | 4 | `sp -0x2c` | `0x11e41c` |
| `str` at 0x995ec | store | 4 | `sp -0x28` | `ret(0x21704)` |
| `ldr` at 0x995f4 | load | 4 | `abs 0x9974c` | `-` |
| `ldr` at 0x995f8 | load | 4 | `abs 0x99770` | `-` |
| `ldr` at 0x99604 | load | 4 | `abs 0x99774` | `-` |
| `ldr` at 0x99608 | load | 4 | `abs 0x99778` | `-` |
| `str` at 0x99618 | store | 4 | `sp -0x14` | `ret(0xb5f4c)` |
| `ldr` at 0x9961c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99634 | load | 4 | `abs 0x9977c` | `-` |
| `str` at 0x99638 | store | 4 | `sp -0x2c` | `0x11e44c` |
| `str` at 0x9963c | store | 4 | `sp -0x28` | `ret(0x21704)` |
| `ldr` at 0x99644 | load | 4 | `abs 0x9974c` | `-` |
| `ldr` at 0x99648 | load | 4 | `abs 0x99780` | `-` |
| `ldr` at 0x99654 | load | 4 | `abs 0x99754` | `-` |
| `ldr` at 0x99658 | load | 4 | `abs 0x99768` | `-` |
| `ldr` at 0x99660 | load | 4 | `abs 0x99774` | `-` |
| `str` at 0x9966c | store | 4 | `sp -0x14` | `ret(0xb4b18)` |
| `ldr` at 0x99670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9967c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99688 | load | 4 | `abs 0x99784` | `-` |
| `str` at 0x9968c | store | 4 | `sp -0x2c` | `0x11e470` |
| `str` at 0x99690 | store | 4 | `sp -0x28` | `ret(0x21704)` |
| `ldr` at 0x99698 | load | 4 | `abs 0x9974c` | `-` |
| `ldr` at 0x9969c | load | 4 | `abs 0x99788` | `-` |
| `ldr` at 0x996a8 | load | 4 | `abs 0x9978c` | `-` |
| `ldr` at 0x996ac | load | 4 | `abs 0x99790` | `-` |
| `ldr` at 0x996c4 | load | 4 | `abs 0x99794` | `-` |
| `str` at 0x996d4 | store | 4 | `sp -0x18` | `0x1800` |
| `str` at 0x996e0 | store | 4 | `sp -0x18` | `0x1000` |
| `str` at 0x996e8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x996ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x996f0 | store | 4 | `sp -0x28` | `[sp-24]` |
| `str` at 0x996f8 | store | 4 | `sp -0x24` | `0x14` |
| `str` at 0x99700 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x99708 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0x9970c | load | 4 | `abs 0x99798` | `-` |
| `ldr` at 0x99710 | load | 4 | `abs 0x9979c` | `-` |
| `ldr` at 0x99714 | load | 4 | `abs 0x997a0` | `-` |
| `ldr` at 0x99720 | load | 4 | `abs 0x997a4` | `-` |
| `ldr` at 0x99724 | load | 4 | `abs 0x997a8` | `-` |
| `ldr` at 0x9972c | load | 4 | `abs 0x99744` | `-` |
| `ldr` at 0x99730 | load | 4 | `abs 0x99744` | `-` |
| `ldr` at 0x99734 | load | 4 | `abs 0x14a55c` | `-` |
| `str` at 0x9973c | store | 4 | `abs 0x14a55c` | `[0x14a55c]+1` |
| `ldmdb` at 0x99740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x99740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x99740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9954c | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x17a3` | `0x0` |
| `bl` at 0x9955c | `(unknown callee) 0xb5e34` | `0x2b25a0` | `0x3` | `0x0` | `?` |
| `bl` at 0x9956c | `(unknown callee) 0xb5f4c` | `0x2b25a8` | `0x11e3ec` | `0xa` | `?` |
| `bl` at 0x99588 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0x995ac | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x17b0` | `0x0` |
| `bl` at 0x995c0 | `(unknown callee) 0xb4b18` | `0x2b25a0` | `0xffff003` | `0x1` | `0x2b25a8` |
| `bl` at 0x995dc | `(unknown callee) 0x21704` | `ret(0xb4b18)` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0x99600 | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x17b8` | `0x0` |
| `bl` at 0x99610 | `(unknown callee) 0xb5f4c` | `0x2b2640` | `0x11e440` | `0xa` | `?` |
| `bl` at 0x9962c | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0x99650 | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x17bf` | `0x0` |
| `bl` at 0x99664 | `(unknown callee) 0xb4b18` | `0x2b25a0` | `0xffff003` | `0x2` | `0x2b2640` |
| `bl` at 0x99680 | `(unknown callee) 0x21704` | `ret(0xb4b18)` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0x996a4 | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x17c7` | `0x0` |
| `bl` at 0x996b8 | `(unknown callee) 0x229b0` | `0x2b26dc` | `0x11e498` | `0x1` | `0x9` |
| `bl` at 0x996bc | `(unknown callee) 0x9adf8` | `ret(0x229b0)` | `?` | `?` | `?` |
| `bl` at 0x9971c | `(unknown callee) 0x22480` | `0x2b26d8` | `0x11e4a4` | `0x9aab0` | `0x0` |
| `bl` at 0x99728 | `(unknown callee) 0xbc05c` | `0x1162c0` | `0x131` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x99530 | `[0x14a55c] == 0x0` | `[0x14a55c] != 0x0` |
| `beq` at 0x99580 | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0x995d4 | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |
| `beq` at 0x99624 | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0x99678 | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |
| `bls` at 0x996cc | `ret(0x9adf8) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x9adf8) > 0x7ffff (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x99740 | `r0` | `ret(0xbc05c)` |
