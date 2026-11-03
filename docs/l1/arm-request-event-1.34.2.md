# ARM request and event paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates.
For L1ReqIRouter, the analysis reports four unresolved computed transfers on known edges, listed in
its entry. Code outside known edges is excluded from the tables. The other fourteen routines
report no unresolved transfers. Undecodable byte ranges remain visible as
fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/request-event-facts.txt) retain all tool sections.

## `L1ReqIRouter`

- `L1ReqIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70a80 to 0x745a0.
  Established. The comparison tree includes a taken edge for `[sp-680] == 0x1002` and falls through
  for inequality (`beq` at 0x70c2c). Before that tree, a byte load addresses `[sp-24] +0x20` (`ldrb`
  at 0x70b00), and a call to `0x9ca84` has candidates `[sp-24]`, `0x0`, unknown and unknown (`bl` at
  0x70b54). One byte store writes `(ret(0x6c9ec) & 0xff)` at `(sp-20 + [sp-1008]) +0x0` (`strb` at
  0x70bb8). The decoded return reports `ret(0x9a998)` in r0 (`ldmdb` at 0x7459c). The four
  unresolved computed transfers prevent a complete account of dispatch destinations. Request
  meanings and hardware effects of the numeric callees are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x70a94 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x70a9c | store | 4 | `sp -0x29c` | `0x0` |
| `str` at 0x70aa4 | store | 4 | `sp -0x2a0` | `0x0` |
| `str` at 0x70ab4 | store | 4 | `sp -0x2a4` | `?` |
| `ldr` at 0x70ab8 | load | 4 | `abs 0x71a8c` | `-` |
| `str` at 0x70ac8 | store | 4 | `sp -0x2a0` | `ret(0x1fb00)` |
| `ldr` at 0x70acc | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x70ad8 | load | 4 | `abs 0x71a90` | `-` |
| `str` at 0x70adc | store | 4 | `sp -0x4c0` | `0xe8ef4` |
| `ldr` at 0x70ae0 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x70ae4 | store | 4 | `sp -0x4bc` | `ret(0x1fb00)` |
| `ldr` at 0x70aec | load | 4 | `abs 0x71a94` | `-` |
| `ldr` at 0x70afc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x70b00 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x70b28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x70b2c | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x70b3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70b40 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x70b44 | store | 4 | `sp -0x3ec` | `[[sp-24]+40]` |
| `ldr` at 0x70b4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x70b58 | store | 4 | `sp -0x3ec` | `ret(0x9ca84)` |
| `ldr` at 0x70b5c | load | 4 | `sp -0x3ec` | `-` |
| `str` at 0x70b60 | store | 4 | `sp -0x2a8` | `[sp-1004]` |
| `str` at 0x70b68 | store | 4 | `sp -0x3f0` | `?` |
| `ldr` at 0x70b6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70b70 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x70b7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70b80 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x70b88 | store | 4 | `sp -0x3f4` | `?` |
| `ldr` at 0x70b8c | load | 4 | `sp -0x3f4` | `-` |
| `str` at 0x70b94 | store | 4 | `sp -0x3f4` | `?` |
| `str` at 0x70ba0 | store | 4 | `sp -0x3f4` | `?` |
| `ldr` at 0x70ba4 | load | 4 | `sp -0x3f4` | `-` |
| `ldr` at 0x70bb4 | load | 4 | `sp -0x3f0` | `-` |
| `strb` at 0x70bb8 | store | 1 | `(sp-20 + [sp-1008]) +0x0` | `(ret(0x6c9ec) & 0xff)` |
| `ldrb` at 0x70bc4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x70bd8 | store | 4 | `sp -0x2b0` | `ret(0x2fac0)` |
| `ldr` at 0x70bdc | load | 4 | `abs 0x71a98` | `-` |
| `ldr` at 0x70be0 | load | 4 | `abs 0x119f8c` | `-` |
| `ldr` at 0x70bec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70bf0 | load | 4 | `abs 0x71a9c` | `-` |
| `ldr` at 0x70bf4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x70bf8 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x70c18 | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0x70c1c | store | 4 | `sp -0x4b4` | `[sp-680]` |
| `ldr` at 0x70c20 | load | 4 | `abs 0x71aa0` | `-` |
| `ldr` at 0x70c24 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c30 | load | 4 | `abs 0x71aa0` | `-` |
| `ldr` at 0x70c34 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c40 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c4c | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c58 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c64 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c70 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c7c | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c8c | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70c98 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70ca4 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70cb0 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70cc0 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70ccc | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70cd8 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70ce4 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70cf0 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70cfc | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d0c | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d18 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d24 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d30 | load | 4 | `abs 0x71aa4` | `-` |
| `ldr` at 0x70d34 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d44 | load | 4 | `abs 0x71aa8` | `-` |
| `ldr` at 0x70d48 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d54 | load | 4 | `abs 0x71aa8` | `-` |
| `ldr` at 0x70d58 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d64 | load | 4 | `abs 0x71aac` | `-` |
| `ldr` at 0x70d68 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d74 | load | 4 | `abs 0x71aac` | `-` |
| `ldr` at 0x70d78 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d84 | load | 4 | `abs 0x71ab0` | `-` |
| `ldr` at 0x70d88 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70d94 | load | 4 | `abs 0x71ab4` | `-` |
| `ldr` at 0x70d98 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70da8 | load | 4 | `abs 0x71ab8` | `-` |
| `ldr` at 0x70dac | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70db8 | load | 4 | `abs 0x71ab8` | `-` |
| `ldr` at 0x70dbc | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70dc8 | load | 4 | `abs 0x71abc` | `-` |
| `ldr` at 0x70dcc | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70dd8 | load | 4 | `abs 0x71ac0` | `-` |
| `ldr` at 0x70ddc | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70dec | load | 4 | `abs 0x71ac4` | `-` |
| `ldr` at 0x70df0 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70dfc | load | 4 | `abs 0x71ac4` | `-` |
| `ldr` at 0x70e00 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e0c | load | 4 | `abs 0x71ac8` | `-` |
| `ldr` at 0x70e10 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e1c | load | 4 | `abs 0x71ac8` | `-` |
| `ldr` at 0x70e20 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e2c | load | 4 | `abs 0x71acc` | `-` |
| `ldr` at 0x70e30 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e40 | load | 4 | `abs 0x71ad0` | `-` |
| `ldr` at 0x70e44 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e50 | load | 4 | `abs 0x71ad4` | `-` |
| `ldr` at 0x70e54 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e64 | load | 4 | `abs 0x71ad8` | `-` |
| `ldr` at 0x70e68 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e74 | load | 4 | `abs 0x71ad8` | `-` |
| `ldr` at 0x70e78 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e84 | load | 4 | `abs 0x71adc` | `-` |
| `ldr` at 0x70e88 | load | 4 | `sp -0x4b4` | `-` |
| `ldr` at 0x70e94 | load | 4 | `sp -0x4b4` | `-` |
| `str` at 0x70eb0 | store | 4 | `sp -0x2b8` | `ret(0x1c79c)` |
| `ldr` at 0x70edc | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x70f08 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x70f34 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x70f3c | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x70f48 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x70f5c | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0x70f60 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x70f64 | store | 4 | `sp -0x2c4` | `[sp-676]` |
| `ldr` at 0x70f68 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x70f6c | store | 4 | `sp -0x3f8` | `[sp-676]` |
| `ldr` at 0x70f70 | load | 4 | `sp -0x3f8` | `-` |
| `ldr` at 0x70f7c | load | 4 | `sp -0x3f8` | `-` |
| `ldr` at 0x70f8c | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0x70f9c | store | 4 | `sp -0x2c4` | `0x23` |
| `str` at 0x70fbc | store | 4 | `sp -0x2c4` | `0x37` |
| `ldr` at 0x70fd0 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0x70ff4 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x71008 | store | 4 | `sp -0x2c0` | `?` |
| `str` at 0x71018 | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x7101c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71020 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x71028 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7102c | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x71034 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x71038 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7103c | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `ldr` at 0x71048 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x71058 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7105c | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `str` at 0x71060 | store | 4 | `sp -0x3fc` | `[ret(0x5e7a8)+16]` |
| `str` at 0x7106c | store | 4 | `sp -0x3fc` | `0x23` |
| `ldr` at 0x71070 | load | 4 | `sp -0x3fc` | `-` |
| `str` at 0x71074 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x71088 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x710a0 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x710b4 | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x710b8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x710bc | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x710c4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x710c8 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x710d0 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x710d4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x710d8 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `ldr` at 0x710e4 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x710f4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x710f8 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `str` at 0x710fc | store | 4 | `sp -0x400` | `[ret(0x5e7a8)+16]` |
| `str` at 0x71108 | store | 4 | `sp -0x400` | `0x23` |
| `ldr` at 0x7110c | load | 4 | `sp -0x400` | `-` |
| `str` at 0x71110 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x71124 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7113c | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x71150 | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x71154 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71158 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x71160 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71164 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x7116c | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x71170 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71174 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `ldr` at 0x71180 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x71190 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71194 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `str` at 0x71198 | store | 4 | `sp -0x404` | `[ret(0x5e7a8)+16]` |
| `str` at 0x711a4 | store | 4 | `sp -0x404` | `0x23` |
| `ldr` at 0x711a8 | load | 4 | `sp -0x404` | `-` |
| `str` at 0x711ac | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x711c0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x711d8 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x711ec | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x711f0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x711f4 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x711fc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71200 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x71208 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x7120c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71210 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `ldr` at 0x7121c | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x7122c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71230 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `str` at 0x71234 | store | 4 | `sp -0x408` | `[ret(0x5e7a8)+16]` |
| `str` at 0x71240 | store | 4 | `sp -0x408` | `0x23` |
| `ldr` at 0x71244 | load | 4 | `sp -0x408` | `-` |
| `str` at 0x71248 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x7125c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x71274 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x712a4 | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x712a8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x712ac | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x712b4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x712b8 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x712c0 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x712d4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x712e4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x712f0 | load | 4 | `[sp-692] +0x10` | `-` |
| `str` at 0x71304 | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x71308 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x7130c | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x71314 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71318 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x71320 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x71334 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x71344 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x71350 | load | 4 | `[sp-692] +0x10` | `-` |
| `ldr` at 0x7135c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x71368 | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0x71380 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7138c | store | 4 | `sp -0x2bc` | `ret(0x2b424)` |
| `str` at 0x71398 | store | 4 | `sp -0x2bc` | `ret(0x2b63c)` |
| `ldr` at 0x7139c | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x713dc | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x713f8 | store | 4 | `sp -0x4c0` | `0x1` |
| `ldr` at 0x71400 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x7142c | load | 4 | `abs 0x71ae0` | `-` |
| `str` at 0x7143c | store | 4 | `sp -0x2b8` | `ret(0x18c14)` |
| `ldr` at 0x71450 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x71458 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x71474 | store | 4 | `sp -0x2b8` | `ret(0x18c14)` |
| `ldr` at 0x71488 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x71490 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x714a8 | store | 4 | `sp -0x2b8` | `ret(0x18c14)` |
| `ldr` at 0x714bc | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x714c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x714cc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x714f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x714f8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x71508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7150c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x71510 | store | 4 | `sp -0x40c` | `[[sp-24]+48]` |
| `ldr` at 0x71518 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x71524 | store | 4 | `sp -0x40c` | `ret(0x9ca84)` |
| `ldr` at 0x71528 | load | 4 | `abs 0x71ae4` | `-` |
| `str` at 0x7152c | store | 4 | `sp -0x410` | `0x11b130` |
| `ldr` at 0x71530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71534 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x71540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71544 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x7154c | store | 4 | `sp -0x414` | `?` |
| `ldr` at 0x71550 | load | 4 | `sp -0x414` | `-` |
| `str` at 0x71558 | store | 4 | `sp -0x414` | `?` |
| `str` at 0x71564 | store | 4 | `sp -0x414` | `?` |
| `ldr` at 0x71568 | load | 4 | `sp -0x410` | `-` |
| `ldr` at 0x7156c | load | 4 | `[sp-1040] +0x0` | `-` |
| `ldr` at 0x71570 | load | 4 | `sp -0x414` | `-` |
| `ldr` at 0x71574 | load | 4 | `sp -0x40c` | `-` |
| `str` at 0x71584 | store | 4 | `sp -0x2cc` | `ret(unknown)` |
| `str` at 0x71598 | store | 4 | `sp -0x2d0` | `ret(0xb45d0)` |
| `ldr` at 0x7159c | load | 4 | `abs 0x71ae8` | `-` |
| `ldrb` at 0x715a4 | load | 1 | `sp -0x2cc` | `-` |
| `ldr` at 0x715bc | load | 4 | `abs 0x71aec` | `-` |
| `ldr` at 0x715e0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x715e4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7162c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x71640 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x71650 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71658 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7166c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x71674 | load | 4 | `(unknown)` | `-` |
| `str` at 0x71678 | store | 4 | `sp -0x418` | `?` |
| `ldr` at 0x7167c | load | 4 | `sp -0x418` | `-` |
| `ldr` at 0x71688 | load | 4 | `sp -0x418` | `-` |
| `ldr` at 0x716a0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x716b4 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x716c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x716d4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x716e8 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x716f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71708 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7171c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7172c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71754 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x71768 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x71778 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71788 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7179c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x717ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x717bc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x717d0 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x717e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x717f0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x71804 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x71814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71828 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x7183c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x7184c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7185c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x71870 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x71880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7188c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x71890 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x718b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x718bc | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x718cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x718d0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x718d4 | store | 4 | `sp -0x41c` | `[[sp-24]+48]` |
| `ldr` at 0x718dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x718e8 | store | 4 | `sp -0x41c` | `ret(0x9ca84)` |
| `ldr` at 0x718ec | load | 4 | `abs 0x71ae4` | `-` |
| `str` at 0x718f0 | store | 4 | `sp -0x420` | `0x11b130` |
| `ldr` at 0x718f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x718f8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x71904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71908 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x71910 | store | 4 | `sp -0x424` | `?` |
| `ldr` at 0x71914 | load | 4 | `sp -0x424` | `-` |
| `str` at 0x7191c | store | 4 | `sp -0x424` | `?` |
| `str` at 0x71928 | store | 4 | `sp -0x424` | `?` |
| `ldr` at 0x7192c | load | 4 | `sp -0x420` | `-` |
| `ldr` at 0x71930 | load | 4 | `[sp-1056] +0x0` | `-` |
| `ldr` at 0x71934 | load | 4 | `sp -0x424` | `-` |
| `ldr` at 0x71938 | load | 4 | `sp -0x41c` | `-` |
| `str` at 0x71948 | store | 4 | `sp -0x2cc` | `ret(unknown)` |
| `ldr` at 0x7194c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x71950 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x71960 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71964 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x71968 | store | 4 | `sp -0x428` | `[[sp-24]+56]` |
| `ldr` at 0x71970 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7197c | store | 4 | `sp -0x428` | `ret(0x9ca84)` |
| `ldr` at 0x71980 | load | 4 | `sp -0x428` | `-` |
| `str` at 0x71984 | store | 4 | `sp -0x2d4` | `[sp-1064]` |
| `ldr` at 0x71988 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7198c | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x7199c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x719a0 | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x719a4 | store | 4 | `sp -0x42c` | `[[sp-24]+64]` |
| `ldr` at 0x719ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x719b8 | store | 4 | `sp -0x42c` | `ret(0x9ca84)` |
| `ldr` at 0x719bc | load | 4 | `sp -0x42c` | `-` |
| `str` at 0x719c0 | store | 4 | `sp -0x2d8` | `[sp-1068]` |
| `ldr` at 0x719c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x719c8 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x719d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x719dc | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x719e0 | store | 4 | `sp -0x430` | `[[sp-24]+72]` |
| `ldr` at 0x719e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x719f4 | store | 4 | `sp -0x430` | `ret(0x9ca84)` |
| `ldr` at 0x719f8 | load | 4 | `sp -0x430` | `-` |
| `str` at 0x719fc | store | 4 | `sp -0x2dc` | `[sp-1072]` |
| `str` at 0x71a04 | store | 4 | `sp -0x2e0` | `0x0` |
| `str` at 0x71a0c | store | 4 | `sp -0x2e4` | `0x0` |
| `str` at 0x71a20 | store | 4 | `sp -0x2e8` | `ret(0xb45d0)` |
| `ldr` at 0x71a24 | load | 4 | `abs 0x71ae8` | `-` |
| `ldrb` at 0x71a2c | load | 1 | `sp -0x2cc` | `-` |
| `ldr` at 0x71a60 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x71a64 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x71b0c | load | 4 | `sp -0x2d4` | `-` |
| `ldrls` at 0x71b14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71f50 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x71f5c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x71f60 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x71f88 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x71f8c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x71f94 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x71fa0 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x71fa4 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x71fac | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0x71fb4 | store | 4 | `sp -0x2e4` | `0x0` |
| `ldr` at 0x71fbc | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x71fc0 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x71fc8 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x71fd0 | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0x71ffc | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x72004 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x72018 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x7201c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x72030 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7203c | load | 4 | `abs 0x71af4` | `-` |
| `ldr` at 0x72040 | load | 4 | `abs 0x27eec8` | `-` |
| `ldr` at 0x7204c | load | 4 | `abs 0x71af4` | `-` |
| `ldr` at 0x72050 | load | 4 | `abs 0x27eec8` | `-` |
| `str` at 0x72064 | store | 4 | `sp -0x2bc` | `ret(unknown)` |
| `str` at 0x72074 | store | 4 | `sp -0x2bc` | `ret(0x1c79c)` |
| `ldr` at 0x72078 | load | 4 | `sp -0x2bc` | `-` |
| `ldrb` at 0x7207c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x720bc | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x720d8 | store | 4 | `sp -0x4c0` | `0x1` |
| `ldr` at 0x720e0 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x720fc | store | 4 | `sp -0x2b8` | `ret(0x1c79c)` |
| `ldr` at 0x72128 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x72140 | store | 4 | `sp -0x2b8` | `ret(0x1c79c)` |
| `ldr` at 0x7216c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x72190 | load | 4 | `sp -0x2a4` | `-` |
| `ldrls` at 0x7219c | load | 4 | `(unknown)` | `-` |
| `str` at 0x7240c | store | 4 | `sp -0x2b8` | `ret(0x1c79c)` |
| `ldr` at 0x72420 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x72434 | store | 4 | `sp -0x2b8` | `ret(0x1c79c)` |
| `ldr` at 0x72448 | load | 4 | `sp -0x2b8` | `-` |
| `ldrb` at 0x7247c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x724cc | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x724e4 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x724f8 | store | 4 | `sp -0x2dc` | `ret(0x2e858)` |
| `ldr` at 0x724fc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x72524 | load | 4 | `abs 0x71af4` | `-` |
| `ldr` at 0x72528 | load | 4 | `abs 0x27ee70` | `-` |
| `str` at 0x7252c | store | 4 | `sp -0x434` | `[0x27ee70]` |
| `ldr` at 0x72530 | load | 4 | `abs 0x71af4` | `-` |
| `ldr` at 0x72534 | load | 4 | `abs 0x27ee70` | `-` |
| `str` at 0x72544 | store | 4 | `sp -0x434` | `0x1` |
| `ldr` at 0x72548 | load | 4 | `sp -0x434` | `-` |
| `str` at 0x7254c | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x72550 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x72564 | store | 4 | `sp -0x2d0` | `ret(0x18920)` |
| `ldr` at 0x72568 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x72570 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x72580 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x72594 | store | 4 | `sp -0x2cc` | `ret(0x18920)` |
| `ldr` at 0x72598 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x725a0 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x725b4 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x725b8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x725bc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x725cc | load | 4 | `abs 0x71af4` | `-` |
| `ldr` at 0x725d4 | load | 4 | `abs 0x27ee60` | `-` |
| `str` at 0x725dc | store | 4 | `sp -0x2f0` | `?` |
| `ldr` at 0x725e0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x725e4 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0x725f0 | store | 4 | `sp -0x2f4` | `ret(0xd569c)` |
| `ldr` at 0x725f4 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x725fc | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0x72608 | load | 4 | `abs 0x71af4` | `-` |
| `ldr` at 0x7260c | load | 4 | `abs 0x27ee60` | `-` |
| `str` at 0x72614 | store | 4 | `sp -0x2f8` | `?` |
| `ldr` at 0x72618 | load | 4 | `abs 0x71af8` | `-` |
| `ldrb` at 0x72620 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x72638 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x72640 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x72648 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x72650 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7265c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x72664 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7266c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x72670 | load | 4 | `abs 0x71af8` | `-` |
| `ldrb` at 0x72678 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x72690 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x72698 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x726a0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x726a8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x726b4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x726bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x726c4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x726c8 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x726d0 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x72700 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x72718 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x72724 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x72728 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7272c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x72744 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x7274c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x7275c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7276c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x72774 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x72784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7278c | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x72794 | store | 4 | `sp -0x2d8` | `[sp-728]+1` |
| `ldr` at 0x7279c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x727a4 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x727b4 | store | 4 | `sp -0x2f8` | `sp-868` |
| `str` at 0x727e0 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x727e4 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x727f4 | load | 4 | `sp -0x318` | `-` |
| `str` at 0x72800 | store | 4 | `sp -0x318` | `?` |
| `ldr` at 0x72804 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x7280c | store | 4 | `sp -0x310` | `?` |
| `ldr` at 0x72810 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x72818 | store | 4 | `sp -0x310` | `?` |
| `str` at 0x72820 | store | 4 | `sp -0x30c` | `0xc5` |
| `ldr` at 0x72824 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x72830 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x72834 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x72838 | load | 4 | `abs 0x71afc` | `-` |
| `ldr` at 0x72844 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7284c | load | 4 | `abs 0x71afc` | `-` |
| `str` at 0x72858 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x7285c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x72860 | load | 4 | `abs 0x71b00` | `-` |
| `ldr` at 0x7286c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x72874 | load | 4 | `abs 0x71b00` | `-` |
| `str` at 0x72880 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x72884 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x72888 | load | 4 | `abs 0x71b04` | `-` |
| `ldr` at 0x72894 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x7289c | load | 4 | `abs 0x71b04` | `-` |
| `str` at 0x728a8 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x728ac | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x728bc | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x728d0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x728d4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x728e4 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x728f8 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x728fc | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x72900 | store | 4 | `sp -0x308` | `[sp-732]` |
| `str` at 0x72914 | store | 4 | `sp -0x2a0` | `ret(0xc2d70)` |
| `ldr` at 0x72918 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x72924 | load | 4 | `sp -0x300` | `-` |
| `str` at 0x72928 | store | 4 | `sp -0x324` | `[sp-768]` |
| `ldr` at 0x7292c | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0x72930 | store | 4 | `sp -0x320` | `[sp-764]` |
| `ldr` at 0x72934 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x72938 | store | 4 | `sp -0x31c` | `[sp-760]` |
| `str` at 0x72940 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x72944 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x72954 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x72960 | load | 4 | `abs 0x71b08` | `-` |
| `ldr` at 0x72974 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x72984 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72998 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x729a0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x729a8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x729b0 | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0x729c0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x729c8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x729cc | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x729d4 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x72a04 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x72a10 | store | 4 | `sp -0x2a0` | `0x0` |
| `ldr` at 0x72a18 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x72a20 | store | 4 | `sp -0x2f8` | `[sp-760]+8` |
| `ldr` at 0x72a24 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x72a2c | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x72a34 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x72a7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72a80 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72aa8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72aac | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72abc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72ac0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72ac4 | store | 4 | `sp -0x438` | `[[sp-24]+48]` |
| `ldr` at 0x72acc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72ad8 | store | 4 | `sp -0x438` | `ret(0x9ca84)` |
| `ldr` at 0x72ae0 | load | 4 | `sp -0x438` | `-` |
| `ldr` at 0x72b04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72b08 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72b30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72b34 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x72b40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72b44 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x72b4c | store | 4 | `sp -0x43c` | `?` |
| `ldr` at 0x72b50 | load | 4 | `sp -0x43c` | `-` |
| `str` at 0x72b58 | store | 4 | `sp -0x43c` | `?` |
| `str` at 0x72b64 | store | 4 | `sp -0x43c` | `0x60` |
| `ldr` at 0x72b68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72b6c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72b7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72b80 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x72b84 | load | 4 | `sp -0x43c` | `-` |
| `str` at 0x72b8c | store | 4 | `sp -0x440` | `?` |
| `ldr` at 0x72b94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72ba4 | load | 4 | `sp -0x43c` | `-` |
| `str` at 0x72bac | store | 4 | `sp -0x440` | `?` |
| `ldr` at 0x72bb0 | load | 4 | `sp -0x440` | `-` |
| `str` at 0x72be0 | store | 4 | `sp -0x2b4` | `ret(0x5e7a8)` |
| `ldr` at 0x72be4 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x72be8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x72bec | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x72bf0 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x72bf4 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `ldr` at 0x72bf8 | load | 4 | `ret(0x5e7a8) +0x14` | `-` |
| `ldr` at 0x72c1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72c20 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72c48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72c4c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72c78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72c7c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72c8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72c90 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72c94 | store | 4 | `sp -0x444` | `[[sp-24]+48]` |
| `ldr` at 0x72c9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72ca8 | store | 4 | `sp -0x444` | `ret(0x9ca84)` |
| `ldr` at 0x72cb0 | load | 4 | `sp -0x444` | `-` |
| `ldr` at 0x72cd8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72cdc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72d04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72d08 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72d1c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72d20 | store | 4 | `sp -0x448` | `[[sp-24]+48]` |
| `ldr` at 0x72d28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72d34 | store | 4 | `sp -0x448` | `ret(0x9ca84)` |
| `ldr` at 0x72d3c | load | 4 | `sp -0x448` | `-` |
| `ldr` at 0x72d74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72d78 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72da0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72da4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72db4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72db8 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72dbc | store | 4 | `sp -0x44c` | `[[sp-24]+48]` |
| `ldr` at 0x72dc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72dd0 | store | 4 | `sp -0x44c` | `ret(0x9ca84)` |
| `ldr` at 0x72dd8 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x72dfc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72e00 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72e28 | load | 4 | `abs 0x73dfc` | `-` |
| `str` at 0x72e2c | store | 4 | `sp -0x450` | `0x281fc0` |
| `ldr` at 0x72e30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72e34 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72e44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72e48 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72e4c | store | 4 | `sp -0x454` | `[[sp-24]+48]` |
| `ldr` at 0x72e54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72e60 | store | 4 | `sp -0x454` | `ret(0x9ca84)` |
| `ldrb` at 0x72e6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72e74 | load | 4 | `sp -0x454` | `-` |
| `ldr` at 0x72e7c | load | 4 | `sp -0x450` | `-` |
| `str` at 0x72e80 | store | 4 | `[sp-1104] +0xc` | `ret(0x6ca54)` |
| `ldr` at 0x72e84 | load | 4 | `abs 0x73dfc` | `-` |
| `ldr` at 0x72e8c | load | 4 | `abs 0x281fcc` | `-` |
| `ldr` at 0x72eb0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72eb4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72edc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72ee0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x72ef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72ef4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x72efc | store | 4 | `sp -0x468` | `?` |
| `ldr` at 0x72f04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72f10 | store | 4 | `sp -0x468` | `ret(0x9ca84)` |
| `ldr` at 0x72f14 | load | 4 | `sp -0x468` | `-` |
| `str` at 0x72f1c | store | 4 | `sp -0x468` | `?` |
| `ldr` at 0x72f20 | load | 4 | `sp -0x468` | `-` |
| `ldr` at 0x72f2c | load | 4 | `sp -0x468` | `-` |
| `ldr` at 0x72f38 | load | 4 | `sp -0x468` | `-` |
| `ldr` at 0x72f48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72f4c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72f5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72f60 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72f64 | store | 4 | `sp -0x458` | `[[sp-24]+48]` |
| `ldr` at 0x72f6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72f78 | store | 4 | `sp -0x458` | `ret(0x9ca84)` |
| `ldr` at 0x72f7c | load | 4 | `sp -0x458` | `-` |
| `ldr` at 0x72f8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72f90 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72fa4 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72fa8 | store | 4 | `sp -0x45c` | `[[sp-24]+48]` |
| `ldr` at 0x72fb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72fbc | store | 4 | `sp -0x45c` | `ret(0x9ca84)` |
| `ldr` at 0x72fc0 | load | 4 | `sp -0x45c` | `-` |
| `ldr` at 0x72fcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x72fd0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x72fe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72fe4 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72fe8 | store | 4 | `sp -0x460` | `[[sp-24]+48]` |
| `ldr` at 0x72ff0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72ffc | store | 4 | `sp -0x460` | `ret(0x9ca84)` |
| `ldr` at 0x73000 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73004 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x73014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73018 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x73020 | store | 4 | `sp -0x464` | `?` |
| `ldr` at 0x73028 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73034 | store | 4 | `sp -0x464` | `ret(0x9ca84)` |
| `ldr` at 0x73038 | load | 4 | `sp -0x464` | `-` |
| `str` at 0x73040 | store | 4 | `sp -0x464` | `?` |
| `ldr` at 0x73044 | load | 4 | `sp -0x460` | `-` |
| `ldr` at 0x73048 | load | 4 | `sp -0x464` | `-` |
| `ldr` at 0x73094 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73098 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x730c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x730c4 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x730d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x730d8 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x730e0 | store | 4 | `sp -0x470` | `?` |
| `ldr` at 0x730e4 | load | 4 | `sp -0x470` | `-` |
| `str` at 0x730ec | store | 4 | `sp -0x470` | `?` |
| `ldr` at 0x730f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73100 | store | 4 | `sp -0x470` | `ret(0x9ca84)` |
| `ldr` at 0x73104 | load | 4 | `sp -0x470` | `-` |
| `str` at 0x7310c | store | 4 | `sp -0x470` | `?` |
| `ldr` at 0x73110 | load | 4 | `sp -0x470` | `-` |
| `str` at 0x73118 | store | 4 | `sp -0x470` | `?` |
| `ldr` at 0x7311c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73120 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x73130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73134 | load | 4 | `[sp-24] +0x40` | `-` |
| `ldr` at 0x73144 | load | 4 | `sp -0x470` | `-` |
| `str` at 0x7314c | store | 4 | `sp -0x46c` | `?` |
| `ldr` at 0x73154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73170 | load | 4 | `sp -0x470` | `-` |
| `str` at 0x73178 | store | 4 | `sp -0x46c` | `?` |
| `ldr` at 0x7317c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73180 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x73190 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73194 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x7319c | load | 4 | `sp -0x46c` | `-` |
| `str` at 0x731a4 | store | 4 | `sp -0x474` | `?` |
| `ldr` at 0x731ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x731c0 | load | 4 | `sp -0x46c` | `-` |
| `str` at 0x731c8 | store | 4 | `sp -0x474` | `?` |
| `ldr` at 0x731cc | load | 4 | `sp -0x474` | `-` |
| `str` at 0x731d0 | store | 4 | `sp -0x2b0` | `[sp-1140]` |
| `ldrb` at 0x731dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x731e8 | load | 4 | `sp -0x2b0` | `-` |
| `ldrb` at 0x731f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x731fc | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x73204 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x73214 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x7321c | store | 4 | `sp -0x4c0` | `?` |
| `ldr` at 0x73220 | load | 4 | `abs 0x73e00` | `-` |
| `ldr` at 0x7322c | load | 4 | `abs 0x73dfc` | `-` |
| `ldr` at 0x73230 | load | 4 | `abs 0x281fcc` | `-` |
| `ldr` at 0x7323c | load | 4 | `abs 0x73dfc` | `-` |
| `ldr` at 0x73240 | load | 4 | `abs 0x281fcc` | `-` |
| `ldrb` at 0x73254 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73268 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7326c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x7329c | load | 1 | `(unknown)` | `-` |
| `str` at 0x732a0 | store | 4 | `sp -0x478` | `?` |
| `ldr` at 0x732a4 | load | 4 | `sp -0x478` | `-` |
| `str` at 0x732ac | store | 4 | `sp -0x478` | `?` |
| `ldr` at 0x732b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x732b4 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x732c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x732c8 | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x732cc | load | 4 | `sp -0x478` | `-` |
| `str` at 0x732d8 | store | 4 | `sp -0x47c` | `?` |
| `ldr` at 0x732e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x732f0 | load | 4 | `sp -0x478` | `-` |
| `str` at 0x732fc | store | 4 | `sp -0x47c` | `?` |
| `ldr` at 0x73300 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73304 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x73314 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73318 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x7331c | store | 4 | `sp -0x480` | `[[sp-24]+48]:1` |
| `ldr` at 0x73324 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73338 | store | 4 | `sp -0x480` | `?` |
| `ldr` at 0x7333c | load | 4 | `sp -0x47c` | `-` |
| `ldr` at 0x73340 | load | 4 | `sp -0x480` | `-` |
| `str` at 0x73354 | store | 4 | `sp -0x2b8` | `ret(0x43298)` |
| `ldr` at 0x73358 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7338c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x73398 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7339c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x733a8 | load | 4 | `abs 0x73e04` | `-` |
| `ldr` at 0x733ac | load | 4 | `abs 0x11b134` | `-` |
| `ldr` at 0x733d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x733d8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x733e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x733ec | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x733f0 | store | 4 | `sp -0x484` | `[[sp-24]+48]:1` |
| `ldr` at 0x733f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7340c | store | 4 | `sp -0x484` | `?` |
| `ldr` at 0x73410 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73414 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x73424 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73428 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x7342c | store | 4 | `sp -0x488` | `[[sp-24]+56]:1` |
| `ldr` at 0x73434 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73448 | store | 4 | `sp -0x488` | `?` |
| `ldr` at 0x7344c | load | 4 | `abs 0x73e04` | `-` |
| `ldr` at 0x73450 | load | 4 | `abs 0x11b134` | `-` |
| `ldr` at 0x73454 | load | 4 | `sp -0x484` | `-` |
| `ldr` at 0x73458 | load | 4 | `sp -0x488` | `-` |
| `str` at 0x73464 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x73468 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x7349c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x734a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x734ac | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x734b8 | load | 4 | `abs 0x73e08` | `-` |
| `ldr` at 0x734bc | load | 4 | `abs 0x11b138` | `-` |
| `ldr` at 0x734e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x734e8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x734f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x734fc | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x73500 | store | 4 | `sp -0x48c` | `[[sp-24]+48]:1` |
| `ldr` at 0x73508 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7351c | store | 4 | `sp -0x48c` | `?` |
| `ldr` at 0x73520 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73524 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x73534 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73538 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x7353c | store | 4 | `sp -0x490` | `[[sp-24]+56]:1` |
| `ldr` at 0x73544 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73558 | store | 4 | `sp -0x490` | `?` |
| `ldr` at 0x7355c | load | 4 | `abs 0x73e08` | `-` |
| `ldr` at 0x73560 | load | 4 | `abs 0x11b138` | `-` |
| `ldr` at 0x73564 | load | 4 | `sp -0x48c` | `-` |
| `ldr` at 0x73568 | load | 4 | `sp -0x490` | `-` |
| `str` at 0x73574 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x73578 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x735ac | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x735b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x735bc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x735c8 | load | 4 | `abs 0x73e0c` | `-` |
| `ldr` at 0x735cc | load | 4 | `abs 0x11b13c` | `-` |
| `ldr` at 0x735f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x735f8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x73608 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7360c | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x73610 | store | 4 | `sp -0x494` | `[[sp-24]+48]:1` |
| `ldr` at 0x73618 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7362c | store | 4 | `sp -0x494` | `?` |
| `ldr` at 0x73630 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73634 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x73644 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73648 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x7364c | store | 4 | `sp -0x498` | `[[sp-24]+56]:1` |
| `ldr` at 0x73654 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73668 | store | 4 | `sp -0x498` | `?` |
| `ldr` at 0x7366c | load | 4 | `abs 0x73e0c` | `-` |
| `ldr` at 0x73670 | load | 4 | `abs 0x11b13c` | `-` |
| `ldr` at 0x73674 | load | 4 | `sp -0x494` | `-` |
| `ldr` at 0x73678 | load | 4 | `sp -0x498` | `-` |
| `str` at 0x73684 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x73688 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x736bc | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x736d0 | store | 4 | `sp -0x2b8` | `ret(0x3a458)` |
| `ldr` at 0x736d4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x73708 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x7371c | store | 4 | `sp -0x2b8` | `ret(0x3a838)` |
| `ldr` at 0x73720 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x73754 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x73764 | store | 4 | `sp -0x2f8` | `?` |
| `ldr` at 0x73768 | load | 4 | `sp -0x2a4` | `-` |
| `ldrls` at 0x73774 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73810 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73838 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7383c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x73864 | load | 4 | `abs 0x73e10` | `-` |
| `ldrb` at 0x7386c | load | 1 | `sp -0x2f8` | `-` |
| `str` at 0x73888 | store | 4 | `sp -0x2f4` | `ret(0xb7af8)` |
| `ldr` at 0x7388c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x738c0 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x738f4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x73910 | store | 4 | `sp -0x2f8` | `?` |
| `ldr` at 0x73914 | load | 4 | `sp -0x2a4` | `-` |
| `ldrls` at 0x73920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x739bc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x739e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x739e8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x73a10 | load | 4 | `abs 0x73e14` | `-` |
| `str` at 0x73a14 | store | 4 | `sp -0x49c` | `0xfffffcae` |
| `ldr` at 0x73a18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73a1c | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x73a2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73a30 | load | 1 | `[sp-24] +0x38` | `-` |
| `strb` at 0x73a38 | store | 1 | `sp -0x4a0` | `([[sp-24]+56]:1 & 0xff)` |
| `ldrb` at 0x73a40 | load | 1 | `sp -0x4a0` | `-` |
| `strb` at 0x73a4c | store | 1 | `sp -0x4a0` | `?` |
| `ldr` at 0x73a54 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x73a68 | store | 1 | `sp -0x4a0` | `(ret(0x9ca84) & 0xff)` |
| `ldrb` at 0x73a70 | load | 1 | `sp -0x4a0` | `-` |
| `strb` at 0x73a7c | store | 1 | `sp -0x4a0` | `?` |
| `ldrb` at 0x73a84 | load | 1 | `sp -0x4a0` | `-` |
| `ldr` at 0x73a8c | load | 4 | `sp -0x49c` | `-` |
| `strb` at 0x73a90 | store | 1 | `(unknown)` | `([sp-1184]:1 & 0xff)` |
| `str` at 0x73a98 | store | 4 | `sp -0x4a4` | `?` |
| `ldr` at 0x73a9c | load | 4 | `abs 0x73e14` | `-` |
| `ldrb` at 0x73aa4 | load | 1 | `sp -0x366` | `-` |
| `str` at 0x73aa8 | store | 4 | `sp -0x4a8` | `[sp-870]:1` |
| `ldr` at 0x73aac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x73ab0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x73ac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73ac4 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x73ac8 | load | 4 | `sp -0x4a8` | `-` |
| `strb` at 0x73ad4 | store | 1 | `sp -0x4a9` | `?` |
| `ldr` at 0x73adc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73aec | load | 4 | `sp -0x4a8` | `-` |
| `strb` at 0x73af8 | store | 1 | `sp -0x4a9` | `?` |
| `ldrb` at 0x73b00 | load | 1 | `sp -0x4a9` | `-` |
| `ldr` at 0x73b08 | load | 4 | `sp -0x4a4` | `-` |
| `strb` at 0x73b0c | store | 1 | `(sp-20 + [sp-1188]) +0x0` | `([sp-1193]:1 & 0xff)` |
| `ldr` at 0x73b10 | load | 4 | `abs 0x73e10` | `-` |
| `ldrb` at 0x73b18 | load | 1 | `sp -0x2f8` | `-` |
| `ldr` at 0x73b1c | load | 4 | `abs 0x73e14` | `-` |
| `ldrb` at 0x73b24 | load | 1 | `sp -0x366` | `-` |
| `ldrb` at 0x73b30 | load | 1 | `(unknown)` | `-` |
| `str` at 0x73b48 | store | 4 | `sp -0x2f4` | `ret(0xb79a0)` |
| `ldr` at 0x73b4c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x73b80 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x73bb8 | store | 4 | `sp -0x2f4` | `ret(0xb45d0)` |
| `ldr` at 0x73bbc | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x73c00 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x73c04 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x73c08 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x73c18 | load | 4 | `abs 0x73e48` | `-` |
| `ldrb` at 0x73c20 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x73c38 | load | 4 | `abs 0x73e18` | `-` |
| `ldr` at 0x73c3c | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x73c60 | store | 4 | `sp -0x2d4` | `(sp-1000 + ret(0x18a1c))` |
| `str` at 0x73c68 | store | 4 | `sp -0x2d8` | `0x1` |
| `ldr` at 0x73c6c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73c70 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x73c7c | load | 4 | `abs 0x73e48` | `-` |
| `ldrb` at 0x73c84 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x73ca4 | load | 4 | `abs 0x73e1c` | `-` |
| `ldr` at 0x73cb0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73cb4 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x73cc0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73cc4 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x73cd0 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73cd4 | load | 4 | `abs 0x73e20` | `-` |
| `ldr` at 0x73ce4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73ce8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73cec | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73cf0 | load | 4 | `[sp-760] +0x54` | `-` |
| `str` at 0x73cf4 | store | 4 | `sp -0x4c0` | `[[sp-760]+84]` |
| `ldr` at 0x73cf8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73cfc | load | 4 | `[sp-760] +0x58` | `-` |
| `ldrb` at 0x73d00 | load | 1 | `[[sp-760]+88] +0x0` | `-` |
| `ldr` at 0x73d10 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73d14 | load | 4 | `[sp-760] +0x58` | `-` |
| `str` at 0x73d1c | store | 4 | `sp -0x4bc` | `?` |
| `ldr` at 0x73d20 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73d24 | load | 4 | `[sp-760] +0x5c` | `-` |
| `str` at 0x73d28 | store | 4 | `sp -0x4b8` | `[[sp-760]+92]` |
| `ldr` at 0x73d2c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73d30 | load | 4 | `abs 0x73e24` | `-` |
| `ldr` at 0x73d34 | load | 4 | `[sp-760] +0x4c` | `-` |
| `ldr` at 0x73d38 | load | 4 | `[sp-760] +0x50` | `-` |
| `ldr` at 0x73d44 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73d48 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x73d54 | load | 4 | `abs 0x73e48` | `-` |
| `ldrb` at 0x73d5c | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x73d7c | load | 4 | `abs 0x73e28` | `-` |
| `ldr` at 0x73d88 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73d8c | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x73d98 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73d9c | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x73da8 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73dac | load | 4 | `abs 0x73e2c` | `-` |
| `ldr` at 0x73dbc | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73dc0 | load | 4 | `abs 0x73e30` | `-` |
| `ldr` at 0x73dc8 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73dd8 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x73de0 | store | 4 | `sp -0x2d4` | `([sp-724] + ret(0x18a1c))` |
| `str` at 0x73de8 | store | 4 | `sp -0x2dc` | `0x40` |
| `ldr` at 0x73dec | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x73e90 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73e94 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x73e98 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x73ea0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73ea4 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73ea8 | load | 4 | `abs 0x73e34` | `-` |
| `ldr` at 0x73eb4 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x73ebc | store | 4 | `sp -0x2d4` | `[sp-724]+2` |
| `ldr` at 0x73ec0 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x73ec8 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `str` at 0x73ed4 | store | 4 | `sp -0x2dc` | `0x5f` |
| `ldr` at 0x73ed8 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x73ee8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73eec | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x73ef0 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x73ef8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73efc | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73f00 | load | 4 | `abs 0x73e34` | `-` |
| `ldr` at 0x73f0c | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x73f14 | store | 4 | `sp -0x2d4` | `[sp-724]+2` |
| `ldr` at 0x73f18 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x73f20 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x73f2c | load | 4 | `sp -0x2d4` | `-` |
| `strb` at 0x73f38 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x73f40 | store | 4 | `sp -0x2d4` | `?` |
| `str` at 0x73f48 | store | 4 | `sp -0x2dc` | `0x49` |
| `ldr` at 0x73f4c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x73f60 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x73f64 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73f68 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x73f6c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x73f78 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x73f7c | store | 1 | `(unknown)` | `?` |
| `str` at 0x73f84 | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x73f88 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x73f90 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x73f9c | load | 4 | `sp -0x2d4` | `-` |
| `strb` at 0x73fa8 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x73fb0 | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x73fb4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73fbc | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x73fc4 | load | 1 | `[[sp-760]+12] +0x40` | `-` |
| `ldr` at 0x73fd0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x73fd8 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x73fe0 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x73ff8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74000 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x74008 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x74020 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74028 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x74030 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x74034 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7403c | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x74044 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x74048 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7404c | load | 4 | `abs 0x73e38` | `-` |
| `ldr` at 0x74058 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x74060 | store | 4 | `sp -0x2d4` | `[sp-724]+4` |
| `ldr` at 0x7406c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74070 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74078 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x74084 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `strb` at 0x74088 | store | 1 | `[sp-724] +0x0` | `([[[sp-760]+12]+91]:1 & 0xff)` |
| `str` at 0x74090 | store | 4 | `sp -0x2d4` | `[sp-724]+1` |
| `ldr` at 0x74098 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7409c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x740a4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x740b0 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `strb` at 0x740b4 | store | 1 | `[sp-724] +0x1` | `([[[sp-760]+12]+92]:1 & 0xff)` |
| `str` at 0x740bc | store | 4 | `sp -0x2d4` | `[sp-724]+2` |
| `ldr` at 0x740c4 | load | 4 | `sp -0x2d4` | `-` |
| `strb` at 0x740d0 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x740d8 | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x740dc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x740e4 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x740ec | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x740f8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74100 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x74108 | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x74110 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74114 | load | 4 | `abs 0x73e3c` | `-` |
| `ldr` at 0x74120 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74130 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x74138 | store | 4 | `sp -0x2d4` | `([sp-724] + ret(0x18a1c))` |
| `ldr` at 0x7413c | load | 4 | `abs 0x73e40` | `-` |
| `ldr` at 0x74140 | load | 4 | `abs 0x11b148` | `-` |
| `ldr` at 0x7414c | load | 4 | `abs 0x73e40` | `-` |
| `ldr` at 0x74150 | load | 4 | `abs 0x11b148` | `-` |
| `ldr` at 0x74154 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74160 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74170 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74174 | load | 4 | `abs 0x73e44` | `-` |
| `ldr` at 0x74180 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74184 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x74190 | load | 4 | `abs 0x73e48` | `-` |
| `ldrb` at 0x74198 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x741b8 | load | 4 | `abs 0x73e4c` | `-` |
| `ldr` at 0x741c4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x741c8 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x741d4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x741d8 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x741e4 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x741e8 | load | 4 | `abs 0x73e50` | `-` |
| `ldr` at 0x741f8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x741fc | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x74208 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x7420c | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x74218 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x7421c | load | 4 | `abs 0x73e54` | `-` |
| `ldr` at 0x7422c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74230 | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x7423c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74240 | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x7424c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74250 | load | 4 | `abs 0x73e58` | `-` |
| `ldr` at 0x74260 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74264 | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x74270 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74274 | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x74280 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74284 | load | 4 | `abs 0x73e5c` | `-` |
| `ldr` at 0x74294 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74298 | load | 4 | `abs 0x73e60` | `-` |
| `ldr` at 0x742a0 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x742b0 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x742b8 | store | 4 | `sp -0x2d4` | `([sp-724] + ret(0x18a1c))` |
| `str` at 0x742c0 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x742c4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x742d8 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x742dc | store | 4 | `sp -0x4b0` | `?` |
| `str` at 0x742e4 | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x742e8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x742ec | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x742f0 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x742f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x74310 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74314 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x74318 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x74320 | load | 1 | `([[sp-760]+68] + [sp-732]) +0x0` | `-` |
| `strb` at 0x74328 | store | 1 | `sp -0x4aa` | `([([[sp-760]+68] + [sp-732])]:1 & 0xff)` |
| `strb` at 0x74338 | store | 1 | `sp -0x4aa` | `0x3f` |
| `ldrb` at 0x74340 | load | 1 | `sp -0x4aa` | `-` |
| `ldr` at 0x74344 | load | 4 | `sp -0x4b0` | `-` |
| `strb` at 0x74348 | store | 1 | `[sp-1200] +0x0` | `?` |
| `ldr` at 0x7434c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x74354 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x74360 | load | 4 | `sp -0x2d4` | `-` |
| `strb` at 0x7436c | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x74374 | store | 4 | `sp -0x2d4` | `?` |
| `str` at 0x7437c | store | 4 | `sp -0x2dc` | `0x7` |
| `ldr` at 0x74380 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x74390 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74394 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x7439c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x743a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x743ac | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x743b0 | load | 4 | `abs 0x73e64` | `-` |
| `ldr` at 0x743bc | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x743c4 | store | 4 | `sp -0x2d4` | `[sp-724]+2` |
| `ldr` at 0x743c8 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x743d0 | store | 4 | `sp -0x2dc` | `[sp-732]-1` |
| `ldr` at 0x743dc | load | 4 | `sp -0x2d4` | `-` |
| `strb` at 0x743e8 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x743f0 | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x743f4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x743f8 | load | 4 | `[sp-760] +0x24` | `-` |
| `ldrb` at 0x743fc | load | 1 | `[[sp-760]+36] +0x3` | `-` |
| `ldr` at 0x74400 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74404 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x74408 | load | 4 | `[sp-760] +0x34` | `-` |
| `str` at 0x7440c | store | 4 | `sp -0x4c0` | `[[sp-760]+52]` |
| `ldr` at 0x74410 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x74414 | load | 4 | `abs 0x73e68` | `-` |
| `ldr` at 0x74418 | load | 4 | `[sp-760] +0x28` | `-` |
| `ldr` at 0x7442c | load | 4 | `abs 0x73e6c` | `-` |
| `str` at 0x74438 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x7443c | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x7446c | store | 4 | `sp -0x4c0` | `0x1` |
| `ldr` at 0x74480 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x74488 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x7448c | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x74494 | store | 4 | `sp -0x2f8` | `[sp-760]+120` |
| `ldr` at 0x7449c | load | 4 | `abs 0x73e70` | `-` |
| `ldr` at 0x744a0 | load | 4 | `abs 0x11b12c` | `-` |
| `ldr` at 0x744ac | load | 4 | `abs 0x73e70` | `-` |
| `ldr` at 0x744b8 | load | 4 | `abs 0x11b12c` | `-` |
| `ldr` at 0x744bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x744ec | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x744fc | store | 4 | `sp -0x4c0` | `sp-664` |
| `ldr` at 0x74500 | load | 4 | `abs 0x73e74` | `-` |
| `ldr` at 0x74508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7450c | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x74518 | store | 4 | `sp -0x2a0` | `ret(0x9bd94)` |
| `ldr` at 0x7451c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x74528 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x74534 | load | 4 | `abs 0x73e78` | `-` |
| `str` at 0x74538 | store | 4 | `sp -0x4c0` | `0xe90c4` |
| `str` at 0x7453c | store | 4 | `sp -0x4bc` | `ret(0x1e71c)` |
| `ldr` at 0x74544 | load | 4 | `abs 0x73e88` | `-` |
| `ldr` at 0x74548 | load | 4 | `abs 0x73e7c` | `-` |
| `ldr` at 0x74554 | load | 4 | `abs 0x73e80` | `-` |
| `str` at 0x74560 | store | 4 | `sp -0x2a0` | `ret(0x1fe64)` |
| `ldr` at 0x74564 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x74570 | load | 4 | `abs 0x73e84` | `-` |
| `str` at 0x74574 | store | 4 | `sp -0x4c0` | `0xe90e0` |
| `ldr` at 0x74578 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x7457c | store | 4 | `sp -0x4bc` | `ret(0x1fe64)` |
| `ldr` at 0x74584 | load | 4 | `abs 0x73e88` | `-` |
| `ldr` at 0x74588 | load | 4 | `abs 0x73e8c` | `-` |
| `ldr` at 0x74594 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x7459c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7459c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7459c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7459c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x70aa8 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x70ac0 | `(unknown callee) 0x1fb00` | `0x281fe0` | `?` | `?` | `?` |
| `bl` at 0x70af8 | `(unknown callee) 0x1eed0` | `0x9` | `0xe8eac` | `0x79` | `0x0` |
| `bl` at 0x70b20 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x70b54 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x70ba8 | `(unknown callee) 0x6c9ec` | `[sp-1012]` | `?` | `?` | `?` |
| `bl` at 0x70bd0 | `(unknown callee) 0x2fac0` | `?` | `[sp-1008]` | `sp-20` | `?` |
| `bl` at 0x70bfc | `(unknown callee) 0x1dba4` | `0xe8f18` | `[[sp-24]+28]` | `[sp-680]` | `[sp-24]` |
| `bl` at 0x70c14 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x70ea8 | `(unknown callee) 0x1c79c` | `0x3` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x70ec8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x70ee0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x70eec | `(unknown callee) 0x1c79c` | `0x1` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x70efc | `(unknown callee) 0x1c79c` | `0x2` | `?` | `?` | `ret(0x1c79c)` |
| `bl` at 0x70f20 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x70f38 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x70fa4 | `(unknown callee) 0x2b3b8` | `ret(0x54230)` | `?` | `[sp-676]` | `[sp-676]` |
| `bl` at 0x70fd4 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x70ff0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x71010 | `(unknown callee) 0x5e7a8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x7108c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x710a4 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x710ac | `(unknown callee) 0x5e7a8` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x71128 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x71140 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x71148 | `(unknown callee) 0x5e7a8` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x711c4 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x711dc | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x711e4 | `(unknown callee) 0x5e7a8` | `0x4` | `?` | `?` | `?` |
| `bl` at 0x71260 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x71278 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x71294 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7129c | `(unknown callee) 0x5e7a8` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x712d8 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x712f4 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x712fc | `(unknown callee) 0x5e7a8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x71338 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x71354 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x71388 | `(unknown callee) 0x2b424` | `?` | `sp-20` | `[sp-680]` | `?` |
| `bl` at 0x71394 | `(unknown callee) 0x2b63c` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-676]` |
| `bl` at 0x713bc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x713d8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x713e0 | `(unknown callee) 0x18a1c` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x71408 | `(unknown callee) 0x54450` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x18a1c)` |
| `bl` at 0x71424 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x71438 | `(unknown callee) 0x18c14` | `0xe8f2c` | `sp-700` | `0xa` | `sp-700` |
| `bl` at 0x71454 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `ret(0x18c14)` | `sp-668` |
| `bl` at 0x71470 | `(unknown callee) 0x18c14` | `[sp-700]+1` | `sp-700` | `0xa` | `[sp-700]+1` |
| `bl` at 0x7148c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `ret(0x18c14)` | `sp-668` |
| `bl` at 0x714a4 | `(unknown callee) 0x18c14` | `[sp-700]+1` | `0x0` | `0xa` | `[sp-700]+1` |
| `bl` at 0x714c0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `ret(0x18c14)` | `sp-668` |
| `bl` at 0x714ec | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71520 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `mov` at 0x7157c | `(unknown callee) (unknown target)` | `[sp-1044]` | `[sp-1036]` | `0x11b130` | `[[sp-1040]]` |
| `bl` at 0x71590 | `(unknown callee) 0xb45d0` | `sp-712` | `?` | `?` | `sp-712` |
| `bl` at 0x715ac | `(unknown callee) 0xb460c` | `[sp-716]:1` | `sp-20` | `?` | `[sp-716]:1` |
| `bl` at 0x715c0 | `(unknown callee) 0x1d6d8` | `0xe8f34` | `?` | `?` | `ret(0xb460c)` |
| `bl` at 0x715d8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71604 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71620 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x71654 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x716c8 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x716fc | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x71730 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x71748 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `0x8` | `sp-668` |
| `bl` at 0x7177c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x717b0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x717e4 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x71818 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x71850 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x71884 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x718b0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x718e4 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `mov` at 0x71940 | `(unknown callee) (unknown target)` | `[sp-1060]` | `[sp-1052]` | `[sp-1056]` | `[[sp-1056]]` |
| `bl` at 0x71978 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x719b4 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x719f0 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x71a18 | `(unknown callee) 0xb45d0` | `sp-720` | `[sp-1072]` | `?` | `sp-720` |
| `bl` at 0x71a34 | `(unknown callee) 0xb460c` | `[sp-716]:1` | `?` | `sp-20` | `[sp-716]:1` |
| `bl` at 0x71a58 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71a84 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71f4c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71f80 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x71fe8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72000 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x72034 | `(unknown callee) 0x542d4` | `sp-668` | `sp-664` | `([sp-736] + [sp-728])` | `[sp-732]` |
| `mov` at 0x7205c | `(unknown callee) (unknown target)` | `0x0` | `[sp-680]` | `[sp-680]` | `[0x27eec8]` |
| `bl` at 0x72070 | `(unknown callee) 0x1c79c` | `0xd` | `[sp-680]` | `[sp-680]` | `[0x27eec8]` |
| `bl` at 0x7209c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x720b8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x720c0 | `(unknown callee) 0x18a1c` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x720e8 | `(unknown callee) 0x54450` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x18a1c)` |
| `bl` at 0x720f4 | `(unknown callee) 0x1c79c` | `0xe` | `[sp-680]` | `[sp-680]` | `0x1002` |
| `bl` at 0x72114 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7212c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x72138 | `(unknown callee) 0x1c79c` | `0x12` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x72158 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72170 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x7218c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72404 | `(unknown callee) 0x1c79c` | `0x2a` | `?` | `?` | `[sp-676]-48` |
| `bl` at 0x72424 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `ret(0x1c79c)` | `sp-668` |
| `bl` at 0x7242c | `(unknown callee) 0x1c79c` | `0x2b` | `?` | `?` | `?` |
| `bl` at 0x7244c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `ret(0x1c79c)` | `sp-668` |
| `bl` at 0x72468 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72488 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x724a0 | `(unknown callee) 0x43798` | `sp-744` | `sp-740` | `sp-740` | `sp-744` |
| `bl` at 0x724b8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x724d0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-744]` | `sp-668` |
| `bl` at 0x724e8 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x724f0 | `(unknown callee) 0x2e858` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x7251c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7255c | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x7257c | `(unknown callee) 0x18c20` | `ret(0x18920)` | `0x0` | `?` | `?` |
| `bl` at 0x7258c | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x725ac | `(unknown callee) 0x18c20` | `ret(0x18920)` | `0x0` | `?` | `?` |
| `bl` at 0x725e8 | `(unknown callee) 0xd569c` | `[sp-736]` | `[sp-752]` | `0x4` | `?` |
| `bl` at 0x72600 | `(unknown callee) 0xd580c` | `[sp-736]` | `[sp-752]` | `?` | `[sp-736]` |
| `bl` at 0x72628 | `(unknown callee) 0x2e878` | `[sp-736]:1` | `?` | `ret(0xd580c)` | `[sp-736]:1` |
| `bl` at 0x72680 | `(unknown callee) 0x2e8fc` | `[sp-736]:1` | `sp-20` | `?` | `[sp-736]:1` |
| `bl` at 0x726ec | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72704 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-724]` | `sp-668` |
| `bl` at 0x7271c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-732]` | `sp-668` |
| `bl` at 0x72760 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x72788 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x727a0 | `(unknown callee) 0x18be4` | `[sp-720]` | `?` | `?` | `[sp-724]` |
| `bl` at 0x727a8 | `(unknown callee) 0x18be4` | `[sp-716]` | `?` | `?` | `?` |
| `bl` at 0x727c8 | `(unknown callee) 0x18c20` | `sp-868` | `0x0` | `0x40` | `sp-868` |
| `bl` at 0x727d8 | `(unknown callee) 0xbbc50` | `0x0` | `sp-792` | `?` | `sp-792` |
| `bl` at 0x7290c | `(unknown callee) 0xc2d70` | `sp-792` | `?` | `?` | `sp-792` |
| `bl` at 0x729f0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72a08 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x72a54 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72a74 | `(unknown callee) 0x542d4` | `sp-668` | `sp-664` | `sp-868` | `0x40` |
| `bl` at 0x72aa0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72ad4 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x72ae4 | `(unknown callee) 0x1abd0` | `0x3` | `[sp-1080]` | `?` | `?` |
| `bl` at 0x72afc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72b28 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72b9c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x72bb4 | `(unknown callee) 0x52edc` | `[sp-1088]` | `?` | `?` | `?` |
| `bl` at 0x72bcc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72bd8 | `(unknown callee) 0x5e7a8` | `0x0` | `[sp-680]` | `[sp-680]` | `0x1004` |
| `bl` at 0x72bfc | `(unknown callee) 0x53094` | `[ret(0x5e7a8)+8]` | `[ret(0x5e7a8)+12]` | `[ret(0x5e7a8)+20]` | `ret(0x5e7a8)` |
| `bl` at 0x72c14 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72c40 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72c70 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72ca4 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x72cb8 | `(unknown callee) 0x2bfac` | `0x32` | `[sp-1092]` | `0x0` | `?` |
| `bl` at 0x72cd0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72cfc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72d30 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x72d40 | `(unknown callee) 0x1abd0` | `0xe` | `[sp-1096]` | `?` | `?` |
| `bl` at 0x72d48 | `(unknown callee) 0x1c79c` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x72d54 | `(unknown callee) 0x5eea8` | `ret(0x1c79c)` | `?` | `?` | `ret(0x1c79c)` |
| `bl` at 0x72d6c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72d98 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72dcc | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x72ddc | `(unknown callee) 0x1abd0` | `0x12` | `[sp-1100]` | `?` | `?` |
| `bl` at 0x72df4 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72e20 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72e5c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `0x281fc0` | `?` |
| `bl` at 0x72e78 | `(unknown callee) 0x6ca54` | `?` | `[sp-1108]` | `?` | `?` |
| `bl` at 0x72e90 | `(unknown callee) 0x1abd0` | `0x13` | `[0x281fcc]` | `?` | `0x281fc0` |
| `bl` at 0x72ea8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x72ed4 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x72f0c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x72f74 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x72f84 | `(unknown callee) 0x2d164` | `[sp-1112]` | `0x0` | `?` | `?` |
| `bl` at 0x72fb8 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x72fc4 | `(unknown callee) 0x2d1d0` | `[sp-1116]` | `?` | `?` | `?` |
| `bl` at 0x72ff8 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x73030 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x73050 | `(unknown callee) 0x2d224` | `[sp-1120]` | `[sp-1124]` | `0x0` | `?` |
| `bl` at 0x7306c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73084 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x7308c | `(unknown callee) 0x6ebac` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `0x100a` |
| `bl` at 0x730b8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x730fc | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x7315c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x731b4 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x731ec | `(unknown callee) 0x2f90c` | `?` | `[sp-688]` | `?` | `?` |
| `bl` at 0x73228 | `(unknown callee) 0x1dba4` | `0xe8f48` | `?` | `?` | `?` |
| `bl` at 0x73244 | `(unknown callee) 0x6c9ec` | `[0x281fcc]` | `?` | `?` | `0x281fc0` |
| `bl` at 0x73260 | `(unknown callee) 0x6ebac` | `sp-20` | `?` | `ret(0x6c9ec)` | `?` |
| `bl` at 0x7328c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x732e8 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x7332c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x73344 | `(unknown callee) 0x2ece8` | `[sp-1148]` | `?` | `?` | `?` |
| `bl` at 0x73350 | `(unknown callee) 0x43298` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2001` |
| `bl` at 0x73378 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73390 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x733cc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73400 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7343c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x73460 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x11b134]` |
| `bl` at 0x73488 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x734a0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x734dc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73510 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7354c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x73570 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x11b138]` |
| `bl` at 0x73598 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x735b0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x735ec | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73620 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x7365c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x73680 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x11b13c]` |
| `bl` at 0x736a8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x736c0 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x736cc | `(unknown callee) 0x3a458` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2005` |
| `bl` at 0x736f4 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7370c | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x73718 | `(unknown callee) 0x3a838` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2007` |
| `bl` at 0x73740 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73758 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x73830 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x7385c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73880 | `(unknown callee) 0xb7af8` | `[sp-760]:1` | `sp-745` | `[sp-760]:1` | `sp-745` |
| `bl` at 0x738ac | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x738c4 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x738e0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x73904 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x739dc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73a08 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73a5c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x73ae4 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `sp-20` | `?` |
| `bl` at 0x73b40 | `(unknown callee) 0xb79a0` | `[sp-760]:1` | `[sp-870]:1` | `?` | `?` |
| `bl` at 0x73b6c | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73b84 | `(unknown callee) 0x54230` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x73ba0 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x73bb0 | `(unknown callee) 0xb45d0` | `sp-760` | `[sp-680]` | `[sp-680]` | `sp-760` |
| `bl` at 0x73bdc | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x73bf8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x73c28 | `(unknown callee) 0xb4848` | `[sp-736]:1` | `sp-20` | `?` | `[sp-736]:1` |
| `bl` at 0x73c44 | `(unknown callee) 0x18c74` | `sp-1000` | `0xe8f64` | `[sp-736]` | `ret(0xb4848)` |
| `bl` at 0x73c54 | `(unknown callee) 0x18a1c` | `sp-1000` | `?` | `?` | `sp-1000` |
| `bl` at 0x73c8c | `(unknown callee) 0xb460c` | `[sp-736]:1` | `?` | `sp-20` | `[sp-736]:1` |
| `bl` at 0x73ca8 | `(unknown callee) 0x18ab8` | `sp-1000` | `0xe8f6c` | `?` | `sp-1000` |
| `bl` at 0x73cc8 | `(unknown callee) 0xb48a8` | `[[sp-760]+28]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x73cdc | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe8f80` | `ret(0xb48a8)` | `ret(0xb48a8)` |
| `bl` at 0x73d3c | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe8f90` | `[[sp-760]+76]` | `[[sp-760]+80]` |
| `bl` at 0x73d64 | `(unknown callee) 0xb460c` | `[sp-736]:1` | `?` | `?` | `[sp-736]:1` |
| `bl` at 0x73d80 | `(unknown callee) 0x18ab8` | `sp-1000` | `0xe8fac` | `?` | `sp-1000` |
| `bl` at 0x73da0 | `(unknown callee) 0xb48a8` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x73db4 | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe8fc0` | `ret(0xb48a8)` | `ret(0xb48a8)` |
| `bl` at 0x73dc4 | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe8fd0` | `?` | `[[sp-760]+4]` |
| `bl` at 0x73dcc | `(unknown callee) 0x18a1c` | `[sp-724]` | `?` | `?` | `?` |
| `bl` at 0x73eb0 | `(unknown callee) 0x18c74` | `?` | `0xe8fdc` | `?` | `?` |
| `bl` at 0x73f08 | `(unknown callee) 0x18c74` | `?` | `0xe8fdc` | `?` | `?` |
| `bl` at 0x73fe8 | `(unknown callee) 0x187ac` | `[[[sp-760]+12]+91]:1` | `?` | `0x5b` | `[[[sp-760]+12]+91]:1` |
| `bl` at 0x74010 | `(unknown callee) 0x187ac` | `[[[sp-760]+12]+92]:1` | `?` | `0x5c` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x74054 | `(unknown callee) 0x18c74` | `?` | `0xe8fe4` | `[[[sp-760]+12]+91]:1` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x7411c | `(unknown callee) 0x18c74` | `?` | `0xe8ff0` | `?` | `?` |
| `bl` at 0x74124 | `(unknown callee) 0x18a1c` | `[sp-724]` | `?` | `?` | `?` |
| `mov` at 0x74168 | `(unknown callee) (unknown target)` | `[sp-760]` | `0x0` | `0x0` | `([sp-724] + ret(0x18a1c))` |
| `bl` at 0x74178 | `(unknown callee) 0x18cd4` | `([sp-724] + ret(0x18a1c))` | `0xe8ffc` | `ret(0x18a1c)` | `[0x11b148]` |
| `bl` at 0x741a0 | `(unknown callee) 0xb460c` | `[sp-736]:1` | `sp-20` | `?` | `[sp-736]:1` |
| `bl` at 0x741bc | `(unknown callee) 0x18ab8` | `sp-1000` | `0xe9000` | `?` | `sp-1000` |
| `bl` at 0x741dc | `(unknown callee) 0xb48a8` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x741f0 | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe9014` | `ret(0xb48a8)` | `ret(0xb48a8)` |
| `bl` at 0x74210 | `(unknown callee) 0xb48a8` | `[[sp-760]+32]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x74224 | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe9024` | `ret(0xb48a8)` | `ret(0xb48a8)` |
| `bl` at 0x74244 | `(unknown callee) 0xb48a8` | `[[sp-760]+44]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x74258 | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe9048` | `ret(0xb48a8)` | `ret(0xb48a8)` |
| `bl` at 0x74278 | `(unknown callee) 0xb48a8` | `[[sp-760]+64]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x7428c | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe906c` | `ret(0xb48a8)` | `ret(0xb48a8)` |
| `bl` at 0x7429c | `(unknown callee) 0x18c74` | `[sp-724]` | `0xe9094` | `?` | `[[sp-760]+64]` |
| `bl` at 0x742a4 | `(unknown callee) 0x18a1c` | `[sp-724]` | `?` | `?` | `?` |
| `bl` at 0x74300 | `(unknown callee) 0x187ac` | `?` | `?` | `[[sp-760]+68]` | `?` |
| `bl` at 0x743b8 | `(unknown callee) 0x18c74` | `?` | `0xe90a0` | `?` | `?` |
| `bl` at 0x7441c | `(unknown callee) 0x18c74` | `?` | `0xe90a8` | `[[[sp-760]+36]+3]:1` | `[[sp-760]+40]` |
| `bl` at 0x74430 | `(unknown callee) 0x18ab8` | `sp-1000` | `0xe90bc` | `?` | `sp-1000` |
| `bl` at 0x74450 | `(unknown callee) 0x18a1c` | `sp-1000` | `?` | `?` | `sp-1000` |
| `bl` at 0x7447c | `(unknown callee) 0x54450` | `sp-668` | `sp-664` | `sp-1000` | `ret(0x18a1c)` |
| `mov` at 0x744cc | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-668` | `sp-664` | `[0x11b12c]` |
| `bl` at 0x744e8 | `(unknown callee) 0x541d4` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x74510 | `(unknown callee) 0x9bd94` | `0x282034` | `0x0` | `[sp-24]` | `[sp-668]` |
| `bl` at 0x7452c | `(unknown callee) 0x1e71c` | `ret(0x9bd94)` | `?` | `?` | `ret(0x9bd94)` |
| `bl` at 0x74550 | `(unknown callee) 0x1eed0` | `0x6` | `0xe8eac` | `0x4c6` | `0x0` |
| `bl` at 0x74558 | `(unknown callee) 0x1fe64` | `0x281fe0` | `?` | `?` | `?` |
| `bl` at 0x74590 | `(unknown callee) 0x1eed0` | `0x9` | `0xe8eac` | `0x4cf` | `0x0` |
| `bl` at 0x74598 | `(unknown callee) 0x9a998` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x70ad4 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `bne` at 0x70b08 | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x70b38 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x70b78 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x70be8 | `[0x119f8c] == 0x0` | `[0x119f8c] != 0x0` |
| `beq` at 0x70c2c | `[sp-680] == 0x1002` | `[sp-680] != 0x1002` |
| `bhi` at 0x70c3c | `[sp-680] > 0x1002 (unsigned, 32-bit)` | `[sp-680] <= 0x1002 (unsigned, 32-bit)` |
| `beq` at 0x70c48 | `[sp-680] == 0x9` | `[sp-680] != 0x9` |
| `bhi` at 0x70c54 | `[sp-680] > 0x9 (unsigned, 32-bit)` | `[sp-680] <= 0x9 (unsigned, 32-bit)` |
| `beq` at 0x70c60 | `[sp-680] == 0x4` | `[sp-680] != 0x4` |
| `bhi` at 0x70c6c | `[sp-680] > 0x4 (unsigned, 32-bit)` | `[sp-680] <= 0x4 (unsigned, 32-bit)` |
| `beq` at 0x70c78 | `[sp-680] == 0x1` | `[sp-680] != 0x1` |
| `blo` at 0x70c84 | `[sp-680] < 0x1 (unsigned, 32-bit)` | `[sp-680] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x70c94 | `[sp-680] == 0x6` | `[sp-680] != 0x6` |
| `blo` at 0x70ca0 | `[sp-680] < 0x6 (unsigned, 32-bit)` | `[sp-680] >= 0x6 (unsigned, 32-bit)` |
| `beq` at 0x70cac | `[sp-680] == 0x7` | `[sp-680] != 0x7` |
| `beq` at 0x70cb8 | `[sp-680] == 0x8` | `[sp-680] != 0x8` |
| `beq` at 0x70cc8 | `[sp-680] == 0xe` | `[sp-680] != 0xe` |
| `bhi` at 0x70cd4 | `[sp-680] > 0xe (unsigned, 32-bit)` | `[sp-680] <= 0xe (unsigned, 32-bit)` |
| `beq` at 0x70ce0 | `[sp-680] == 0xb` | `[sp-680] != 0xb` |
| `blo` at 0x70cec | `[sp-680] < 0xb (unsigned, 32-bit)` | `[sp-680] >= 0xb (unsigned, 32-bit)` |
| `beq` at 0x70cf8 | `[sp-680] == 0xc` | `[sp-680] != 0xc` |
| `beq` at 0x70d04 | `[sp-680] == 0xd` | `[sp-680] != 0xd` |
| `beq` at 0x70d14 | `[sp-680] == 0x10` | `[sp-680] != 0x10` |
| `blo` at 0x70d20 | `[sp-680] < 0x10 (unsigned, 32-bit)` | `[sp-680] >= 0x10 (unsigned, 32-bit)` |
| `beq` at 0x70d2c | `[sp-680] == 0x11` | `[sp-680] != 0x11` |
| `beq` at 0x70d3c | `[sp-680] == 0x1001` | `[sp-680] != 0x1001` |
| `beq` at 0x70d50 | `[sp-680] == 0x100c` | `[sp-680] != 0x100c` |
| `bhi` at 0x70d60 | `[sp-680] > 0x100c (unsigned, 32-bit)` | `[sp-680] <= 0x100c (unsigned, 32-bit)` |
| `beq` at 0x70d70 | `[sp-680] == 0x1007` | `[sp-680] != 0x1007` |
| `bhi` at 0x70d80 | `[sp-680] > 0x1007 (unsigned, 32-bit)` | `[sp-680] <= 0x1007 (unsigned, 32-bit)` |
| `bhi` at 0x70d90 | `[sp-680] > 0x1005 (unsigned, 32-bit)` | `[sp-680] <= 0x1005 (unsigned, 32-bit)` |
| `bhs` at 0x70da0 | `[sp-680] >= 0x1004 (unsigned, 32-bit)` | `[sp-680] < 0x1004 (unsigned, 32-bit)` |
| `beq` at 0x70db4 | `[sp-680] == 0x1009` | `[sp-680] != 0x1009` |
| `blo` at 0x70dc4 | `[sp-680] < 0x1009 (unsigned, 32-bit)` | `[sp-680] >= 0x1009 (unsigned, 32-bit)` |
| `beq` at 0x70dd4 | `[sp-680] == 0x100a` | `[sp-680] != 0x100a` |
| `beq` at 0x70de4 | `[sp-680] == 0x100b` | `[sp-680] != 0x100b` |
| `beq` at 0x70df8 | `[sp-680] == 0x2005` | `[sp-680] != 0x2005` |
| `bhi` at 0x70e08 | `[sp-680] > 0x2005 (unsigned, 32-bit)` | `[sp-680] <= 0x2005 (unsigned, 32-bit)` |
| `beq` at 0x70e18 | `[sp-680] == 0x2002` | `[sp-680] != 0x2002` |
| `bhi` at 0x70e28 | `[sp-680] > 0x2002 (unsigned, 32-bit)` | `[sp-680] <= 0x2002 (unsigned, 32-bit)` |
| `beq` at 0x70e38 | `[sp-680] == 0x2001` | `[sp-680] != 0x2001` |
| `beq` at 0x70e4c | `[sp-680] == 0x2003` | `[sp-680] != 0x2003` |
| `beq` at 0x70e5c | `[sp-680] == 0x2004` | `[sp-680] != 0x2004` |
| `beq` at 0x70e70 | `[sp-680] == 0x2007` | `[sp-680] != 0x2007` |
| `blo` at 0x70e80 | `[sp-680] < 0x2007 (unsigned, 32-bit)` | `[sp-680] >= 0x2007 (unsigned, 32-bit)` |
| `beq` at 0x70e90 | `[sp-680] == 0x2008` | `[sp-680] != 0x2008` |
| `beq` at 0x70e9c | `[sp-680] == 0x4000` | `[sp-680] != 0x4000` |
| `bne` at 0x70f44 | `[sp-680] != 0x3` | `[sp-680] == 0x3` |
| `beq` at 0x70f78 | `[sp-676] == 0x38` | `[sp-676] != 0x38` |
| `beq` at 0x70f84 | `[sp-676] == 0x3d` | `[sp-676] != 0x3d` |
| `bne` at 0x70f94 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x70fb4 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x71044 | `[ret(0x5e7a8)+16] != 0x3d` | `[ret(0x5e7a8)+16] == 0x3d` |
| `bne` at 0x71050 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x710e0 | `[ret(0x5e7a8)+16] != 0x3d` | `[ret(0x5e7a8)+16] == 0x3d` |
| `bne` at 0x710ec | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x7117c | `[ret(0x5e7a8)+16] != 0x3d` | `[ret(0x5e7a8)+16] == 0x3d` |
| `bne` at 0x71188 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x71218 | `[ret(0x5e7a8)+16] != 0x3d` | `[ret(0x5e7a8)+16] == 0x3d` |
| `bne` at 0x71224 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `beq` at 0x71364 | `[sp-676] == 0x36` | `[sp-676] != 0x36` |
| `beq` at 0x71370 | `[sp-676] == 0x40` | `[sp-676] != 0x40` |
| `bne` at 0x713a4 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x714d4 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x71504 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x7153c | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x715b8 | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `blt` at 0x715ec | `[sp-716] < [sp-720] (signed, 32-bit)` | `[sp-716] >= [sp-720] (signed, 32-bit)` |
| `beq` at 0x71684 | `[sp-1048] == 0x1` | `[sp-1048] != 0x1` |
| `beq` at 0x71690 | `[sp-1048] == 0x2` | `[sp-1048] != 0x2` |
| `bhi` at 0x71898 | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x718c8 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x71900 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x7195c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x71998 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x719d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x71a40 | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `blt` at 0x71a6c | `[sp-716] < [sp-744] (signed, 32-bit)` | `[sp-716] >= [sp-744] (signed, 32-bit)` |
| `ldrls` at 0x71b14 | `[sp-724] <= 0x4 (unsigned, 32-bit)` | `[sp-724] > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x71f58 | `[sp-736] == 0x0` | `[sp-736] != 0x0` |
| `blo` at 0x71f68 | `[sp-728] < [sp-740] (unsigned, 32-bit)` | `[sp-728] >= [sp-740] (unsigned, 32-bit)` |
| `bls` at 0x71f9c | `([sp-728] + [sp-732]) <= [sp-740] (unsigned, 32-bit)` | `([sp-728] + [sp-732]) > [sp-740] (unsigned, 32-bit)` |
| `beq` at 0x7200c | `[sp-732] == 0x0` | `[sp-732] != 0x0` |
| `beq` at 0x72048 | `[0x27eec8] == 0x0` | `[0x27eec8] != 0x0` |
| `bne` at 0x72084 | `Z == 0` | `not (Z == 0)` |
| `ldrls` at 0x7219c | `[sp-676]-48 <= 0x21 (unsigned, 32-bit)` | `[sp-676]-48 > 0x21 (unsigned, 32-bit)` |
| `bne` at 0x72504 | `ret(0x2e858) != 0x0` | `ret(0x2e858) == 0x0` |
| `bne` at 0x7253c | `[0x27ee70] != 0x0` | `[0x27ee70] == 0x0` |
| `blt` at 0x725c4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x72634 | `ret(0x2e878) == 0x0` | `ret(0x2e878) != 0x0` |
| `beq` at 0x7268c | `ret(0x2e8fc) == 0x0` | `ret(0x2e8fc) != 0x0` |
| `blt` at 0x72734 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x727ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x72920 | `ret(0xc2d70) == 0x0` | `ret(0xc2d70) != 0x0` |
| `bls` at 0x7294c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x72994 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72a3c | `[sp-672] == 0x0` | `[sp-672] != 0x0` |
| `bhi` at 0x72a88 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x72ab8 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x72b10 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x72b3c | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x72b78 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x72c28 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x72c58 | `? == 0x2` | `? != 0x2` |
| `bne` at 0x72c88 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x72ce4 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x72d14 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x72d80 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x72db0 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x72e08 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x72e40 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x72ebc | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x72eec | `? != 0x0` | `? == 0x0` |
| `beq` at 0x72f28 | `[sp-1128] == 0x1` | `[sp-1128] != 0x1` |
| `blo` at 0x72f34 | `[sp-1128] < 0x1 (unsigned, 32-bit)` | `[sp-1128] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x72f40 | `[sp-1128] == 0x2` | `[sp-1128] != 0x2` |
| `bne` at 0x72f58 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72f9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72fdc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73010 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x730a0 | `[[sp-24]+32]:1 == 0x4` | `[[sp-24]+32]:1 != 0x4` |
| `bne` at 0x730d0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7312c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7318c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73238 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x7325c | `ret(0x6c9ec) != ?` | `ret(0x6c9ec) == ?` |
| `beq` at 0x73274 | `[[sp-24]+32]:1 == 0x3` | `[[sp-24]+32]:1 != 0x3` |
| `bne` at 0x732c0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73310 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73360 | `ret(0x43298) == 0x0` | `ret(0x43298) != 0x0` |
| `bls` at 0x733a4 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x733b4 | `[0x11b134] != 0x0` | `[0x11b134] == 0x0` |
| `bne` at 0x733e4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73420 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73470 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x734b4 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x734c4 | `[0x11b138] != 0x0` | `[0x11b138] == 0x0` |
| `bne` at 0x734f4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73530 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73580 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x735c4 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x735d4 | `[0x11b13c] != 0x0` | `[0x11b13c] == 0x0` |
| `bne` at 0x73604 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73640 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73690 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x736dc | `ret(0x3a458) == 0x0` | `ret(0x3a458) != 0x0` |
| `beq` at 0x73728 | `ret(0x3a838) == 0x0` | `ret(0x3a838) != 0x0` |
| `ldrls` at 0x73774 | `[sp-676]-56 <= 0x19 (unsigned, 32-bit)` | `[sp-676]-56 > 0x19 (unsigned, 32-bit)` |
| `bge` at 0x73818 | `[sp-760] >= 0x0 (signed, 32-bit)` | `[sp-760] < 0x0 (signed, 32-bit)` |
| `beq` at 0x73844 | `[[sp-24]+32]:1 == 0x1` | `[[sp-24]+32]:1 != 0x1` |
| `beq` at 0x73894 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `ldrls` at 0x73920 | `[sp-676]-56 <= 0x19 (unsigned, 32-bit)` | `[sp-676]-56 > 0x19 (unsigned, 32-bit)` |
| `bge` at 0x739c4 | `[sp-760] >= 0x0 (signed, 32-bit)` | `[sp-760] < 0x0 (signed, 32-bit)` |
| `beq` at 0x739f0 | `[[sp-24]+32]:1 == 0x3` | `[[sp-24]+32]:1 != 0x3` |
| `bne` at 0x73a28 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73abc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73b54 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x73bc4 | `ret(0xb45d0) != 0x0` | `ret(0xb45d0) == 0x0` |
| `blt` at 0x73c10 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x73c78 | `[[sp-760]] != 0x1` | `[[sp-760]] == 0x1` |
| `bne` at 0x73c98 | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `beq` at 0x73cbc | `[[sp-760]+28] == 0x0` | `[[sp-760]+28] != 0x0` |
| `bne` at 0x73d50 | `[[sp-760]] != 0x2` | `[[sp-760]] == 0x2` |
| `bne` at 0x73d70 | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `beq` at 0x73d94 | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `ble` at 0x73df4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x73ee0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x73f54 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x73fcc | `[[[sp-760]+12]+64]:1 == 0x2c` | `[[[sp-760]+12]+64]:1 != 0x2c` |
| `beq` at 0x73ff4 | `ret(0x187ac) == 0x0` | `ret(0x187ac) != 0x0` |
| `bne` at 0x7401c | `ret(0x187ac) != 0x0` | `ret(0x187ac) == 0x0` |
| `beq` at 0x74148 | `[0x11b148] == 0x0` | `[0x11b148] != 0x0` |
| `bne` at 0x7418c | `[[sp-760]] != 0x3` | `[[sp-760]] == 0x3` |
| `bne` at 0x741ac | `ret(0xb460c) != 0x0` | `ret(0xb460c) == 0x0` |
| `beq` at 0x741d0 | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `beq` at 0x74204 | `[[sp-760]+32] == 0x0` | `[[sp-760]+32] != 0x0` |
| `beq` at 0x74238 | `[[sp-760]+44] == 0x0` | `[[sp-760]+44] != 0x0` |
| `beq` at 0x7426c | `[[sp-760]+64] == 0x0` | `[[sp-760]+64] != 0x0` |
| `ble` at 0x742cc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7430c | `ret(0x187ac) == 0x0` | `ret(0x187ac) != 0x0` |
| `bge` at 0x74388 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x74444 | `[sp-728] == 0x0` | `[sp-728] != 0x0` |
| `beq` at 0x744a8 | `[0x11b12c] == 0x0` | `[0x11b12c] != 0x0` |
| `beq` at 0x744f4 | `[sp-668] == 0x0` | `[sp-668] != 0x0` |
| `beq` at 0x74524 | `ret(0x9bd94) == 0x0` | `ret(0x9bd94) != 0x0` |
| `beq` at 0x7456c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7459c | `r0` | `ret(0x9a998)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x71b14).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7219c).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x73774).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x73920).

## `L1ReqIRouter_2m`

- `L1ReqIRouter_2m`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74c48 to 0x75704.
  Established. Dispatch comparisons include `[sp-80] == 0x200b`, `0x2009`, `0x12`, `0x3001`,
  `0x200c` and `0x4001`, each selecting the taken edge of its own equality branch (`beq` at 0x74cd8,
  `beq` at 0x74cf8, `beq` at 0x74d14, `beq` at 0x74d28, `beq` at 0x74d48, `beq` at 0x74d5c). The
  explicit-link indirect call has unknown target and candidates `0x0`, `[sp-124]`, `[sp-128]`, `0x0`
  (`mov` at 0x75600); its continuation takes the signed-positive edge for `ret(unknown) > 0x0` and
  falls through for a nonpositive result (`bgt` at 0x75614). A word store addresses `[sp-24] +0x0`
  with value `[sp-32]` (`str` at 0x756e0). The return value and the request-specific hardware roles
  remain unknown (`ldmdb` at 0x756e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74c58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74c5c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x74c60 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x74c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74c68 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0x74c6c | store | 4 | `sp -0x20` | `[r1]` |
| `str` at 0x74c74 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x74c84 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x74c88 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74c8c | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x74c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74ca0 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x74ca4 | store | 4 | `sp -0x50` | `[[sp-20]+40]` |
| `ldr` at 0x74cac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74cb8 | store | 4 | `sp -0x50` | `ret(0x9ca84)` |
| `ldr` at 0x74cbc | load | 4 | `sp -0x50` | `-` |
| `str` at 0x74cc0 | store | 4 | `sp -0x2c` | `[sp-80]` |
| `ldr` at 0x74cc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x74cc8 | store | 4 | `sp -0x84` | `[sp-80]` |
| `ldr` at 0x74ccc | load | 4 | `abs 0x756e8` | `-` |
| `ldr` at 0x74cd0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74cdc | load | 4 | `abs 0x756e8` | `-` |
| `ldr` at 0x74ce0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74cec | load | 4 | `abs 0x756ec` | `-` |
| `ldr` at 0x74cf0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74cfc | load | 4 | `abs 0x756ec` | `-` |
| `ldr` at 0x74d00 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74d0c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74d1c | load | 4 | `abs 0x756f0` | `-` |
| `ldr` at 0x74d20 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74d2c | load | 4 | `abs 0x756f0` | `-` |
| `ldr` at 0x74d30 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74d3c | load | 4 | `abs 0x756f4` | `-` |
| `ldr` at 0x74d40 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x74d50 | load | 4 | `abs 0x756f8` | `-` |
| `ldr` at 0x74d54 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x74d70 | store | 4 | `sp -0x38` | `ret(0x1c79c)` |
| `ldr` at 0x74d7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74da4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74da8 | load | 4 | `abs 0x756fc` | `-` |
| `ldr` at 0x74dd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74dd4 | load | 4 | `abs 0x756fc` | `-` |
| `ldr` at 0x74de8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74dec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x74df8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74dfc | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x74e10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x74e24 | store | 4 | `sp -0x54` | `ret(0xbb2b0)` |
| `ldr` at 0x74e28 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74e2c | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x74e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74e40 | load | 4 | `[sp-20] +0x28` | `-` |
| `ldr` at 0x74e44 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x74e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74e64 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x74e7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74e8c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74e90 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x74ea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74ea4 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x74ea8 | store | 4 | `sp -0x58` | `[[sp-20]+40]` |
| `ldr` at 0x74eb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74ebc | store | 4 | `sp -0x58` | `ret(0x9ca84)` |
| `ldr` at 0x74ec0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x74ecc | store | 4 | `sp -0x24` | `ret(0xbb3b8)` |
| `ldr` at 0x74ed0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74ee4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74efc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74f10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74f20 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74f24 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x74f38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x74f4c | store | 4 | `sp -0x5c` | `ret(0xbb2b0)` |
| `ldr` at 0x74f50 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74f54 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x74f64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74f68 | load | 4 | `[sp-20] +0x28` | `-` |
| `ldr` at 0x74f6c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x74f7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74f8c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x74fa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74fb4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74fb8 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x74fc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74fcc | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x74fd0 | store | 4 | `sp -0x60` | `[[sp-20]+40]` |
| `ldr` at 0x74fd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74fe4 | store | 4 | `sp -0x60` | `ret(0x9ca84)` |
| `ldr` at 0x74fec | load | 4 | `sp -0x60` | `-` |
| `str` at 0x74ffc | store | 4 | `sp -0x24` | `ret(0xbb470)` |
| `ldr` at 0x75000 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x75014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7502c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x75040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75064 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x75068 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x75074 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x75078 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x75088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7508c | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x75090 | store | 4 | `sp -0x64` | `[[sp-20]+40]` |
| `ldr` at 0x75098 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x750a4 | store | 4 | `sp -0x64` | `ret(0x9ca84)` |
| `ldr` at 0x750a8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x750ac | store | 4 | `sp -0x4c` | `[sp-100]` |
| `ldr` at 0x750b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x750b4 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x750d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x750e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x750f8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x750fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x75100 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x75110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75114 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x75128 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x75138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7513c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x75150 | load | 1 | `(unknown)` | `-` |
| `str` at 0x75154 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x7515c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x75164 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75178 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x75180 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x75190 | store | 4 | `sp -0x24` | `ret(0xb7af8)` |
| `ldr` at 0x75194 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x751a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x751bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x751c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x751d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x751d4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x751e4 | load | 1 | `sp -0x45` | `-` |
| `ldr` at 0x751ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x751f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x751fc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x75204 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x75208 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x7521c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7522c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x75230 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x75240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75244 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x75248 | store | 4 | `sp -0x6c` | `[[sp-20]+40]` |
| `ldr` at 0x75250 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7525c | store | 4 | `sp -0x6c` | `ret(0x9ca84)` |
| `ldr` at 0x75260 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x75264 | store | 4 | `sp -0x4c` | `[sp-108]` |
| `ldr` at 0x75268 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7527c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x75280 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x75294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x752ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x752bc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x752c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x752c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x752d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x752d8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x752f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x75308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7530c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x7532c | load | 1 | `(unknown)` | `-` |
| `str` at 0x75330 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x75338 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7534c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75360 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x75364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75368 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x75388 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x75398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7539c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x753bc | load | 1 | `(unknown)` | `-` |
| `str` at 0x753c0 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x753c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x753dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x753f0 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x753f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x753f8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x75418 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x75428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7542c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x7544c | load | 1 | `(unknown)` | `-` |
| `str` at 0x75450 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x75458 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7546c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75480 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x75484 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x75488 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x7548c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x75498 | store | 4 | `sp -0x24` | `ret(0xb79a0)` |
| `ldr` at 0x7549c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x754b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x754c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x754c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x754d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x754dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x754e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x754f0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x754f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x754fc | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x75510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7552c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75538 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75544 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75550 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7555c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7556c | load | 4 | `abs 0x75700` | `-` |
| `ldr` at 0x75570 | load | 4 | `abs 0x11b148` | `-` |
| `ldr` at 0x7557c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x75580 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x75590 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75594 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x75598 | store | 4 | `sp -0x7c` | `[[sp-20]+48]` |
| `ldr` at 0x755a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x755ac | store | 4 | `sp -0x7c` | `ret(0x9ca84)` |
| `ldr` at 0x755b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x755b4 | load | 1 | `[sp-20] +0x34` | `-` |
| `ldr` at 0x755c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x755c8 | load | 4 | `[sp-20] +0x38` | `-` |
| `str` at 0x755cc | store | 4 | `sp -0x80` | `[[sp-20]+56]` |
| `ldr` at 0x755d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x755e0 | store | 4 | `sp -0x80` | `ret(0x9ca84)` |
| `ldr` at 0x755e4 | load | 4 | `abs 0x75700` | `-` |
| `ldr` at 0x755e8 | load | 4 | `abs 0x11b148` | `-` |
| `ldr` at 0x755f0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x755f4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x75608 | store | 4 | `sp -0x4c` | `ret(unknown)` |
| `ldr` at 0x7560c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x75620 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75638 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7564c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75660 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75664 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x75678 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75690 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x756a8 | store | 4 | `sp -0x24` | `ret(0x2a830)` |
| `ldr` at 0x756b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x756b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x756cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x756d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x756dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x756e0 | store | 4 | `[sp-24] +0x0` | `[sp-32]` |
| `ldmdb` at 0x756e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x756e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x756e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74c78 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x74cb4 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x74d68 | `(unknown callee) 0x1c79c` | `0x7e` | `?` | `[sp-80]` | `[sp-80]` |
| `bl` at 0x74d84 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x74d8c | `(unknown callee) 0x1c79c` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x74dac | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `0xffff` | `sp-32` |
| `bl` at 0x74db8 | `(unknown callee) 0x1c79c` | `0x80` | `?` | `?` | `ret(0x1c79c)` |
| `bl` at 0x74dd8 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `0xffff` | `sp-32` |
| `bl` at 0x74df0 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-56]` | `sp-32` |
| `bl` at 0x74e18 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x74e20 | `(unknown callee) 0xbb2b0` | `?` | `?` | `[sp-80]` | `[[sp-20]+32]:1` |
| `bl` at 0x74e5c | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x74e84 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x74eb8 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x74ec4 | `(unknown callee) 0xbb3b8` | `[sp-88]` | `?` | `?` | `?` |
| `bl` at 0x74eec | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x74f00 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-36]` | `sp-32` |
| `bl` at 0x74f18 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x74f40 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x74f48 | `(unknown callee) 0xbb2b0` | `?` | `?` | `[sp-80]` | `[[sp-20]+32]:1` |
| `bl` at 0x74f84 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x74fac | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x74fe0 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x74ff4 | `(unknown callee) 0xbb470` | `[sp-96]` | `sp-69` | `?` | `sp-69` |
| `bl` at 0x7501c | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x75030 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-36]` | `sp-32` |
| `bl` at 0x75048 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x7505c | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x750a0 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `[sp-80]` | `?` |
| `bl` at 0x750d8 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x750f0 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x7516c | `(unknown callee) 0x9ca84` | `[sp-20]` | `?` | `0x2c` | `?` |
| `bl` at 0x75188 | `(unknown callee) 0xb7af8` | `[sp-104]` | `sp-69` | `?` | `sp-69` |
| `bl` at 0x751b0 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x751c4 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-36]` | `sp-32` |
| `bl` at 0x751d8 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-48]` | `sp-32` |
| `bl` at 0x751f0 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-69]:1` | `sp-32` |
| `bl` at 0x75224 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x75258 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `[sp-80]` | `?` |
| `bl` at 0x7529c | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x752b4 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x75354 | `(unknown callee) 0x9ca84` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x753e4 | `(unknown callee) 0x9ca84` | `[sp-20]` | `?` | `[sp-48]` | `?` |
| `bl` at 0x75474 | `(unknown callee) 0x9ca84` | `[sp-20]` | `?` | `[sp-48]` | `?` |
| `bl` at 0x75490 | `(unknown callee) 0xb79a0` | `[sp-112]` | `[sp-116]` | `[sp-120]` | `?` |
| `bl` at 0x754b8 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x754cc | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-36]` | `sp-32` |
| `bl` at 0x754e0 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-48]` | `sp-32` |
| `bl` at 0x75518 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x755a8 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `[sp-80]` | `?` |
| `bl` at 0x755dc | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x2` | `?` | `?` |
| `mov` at 0x75600 | `(unknown callee) (unknown target)` | `0x0` | `[sp-124]` | `[sp-128]` | `0x0` |
| `bl` at 0x75628 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x7563c | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-76]` | `sp-32` |
| `bl` at 0x75654 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x75668 | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `[sp-76]` | `sp-32` |
| `bl` at 0x75680 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |
| `bl` at 0x75698 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `0x0` | `sp-32` |
| `bl` at 0x756a0 | `(unknown callee) 0x2a830` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x756bc | `(unknown callee) 0x54230` | `sp-32` | `[sp-28]` | `ret(0x2a830)` | `sp-32` |
| `bl` at 0x756d4 | `(unknown callee) 0x541d4` | `sp-32` | `[sp-28]` | `?` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x74c98 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x74cd8 | `[sp-80] == 0x200b` | `[sp-80] != 0x200b` |
| `bhi` at 0x74ce8 | `[sp-80] > 0x200b (unsigned, 32-bit)` | `[sp-80] <= 0x200b (unsigned, 32-bit)` |
| `beq` at 0x74cf8 | `[sp-80] == 0x2009` | `[sp-80] != 0x2009` |
| `bhi` at 0x74d08 | `[sp-80] > 0x2009 (unsigned, 32-bit)` | `[sp-80] <= 0x2009 (unsigned, 32-bit)` |
| `beq` at 0x74d14 | `[sp-80] == 0x12` | `[sp-80] != 0x12` |
| `beq` at 0x74d28 | `[sp-80] == 0x3001` | `[sp-80] != 0x3001` |
| `bhi` at 0x74d38 | `[sp-80] > 0x3001 (unsigned, 32-bit)` | `[sp-80] <= 0x3001 (unsigned, 32-bit)` |
| `beq` at 0x74d48 | `[sp-80] == 0x200c` | `[sp-80] != 0x200c` |
| `beq` at 0x74d5c | `[sp-80] == 0x4001` | `[sp-80] != 0x4001` |
| `beq` at 0x74d98 | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `beq` at 0x74dc4 | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `bne` at 0x74e04 | `[[sp-20]+32]:1 != 0x0` | `[[sp-20]+32]:1 == 0x0` |
| `bne` at 0x74e38 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x74e4c | `[[sp-20]+40] >= ret(0xbb2b0) (unsigned, 32-bit)` | `[[sp-20]+40] < ret(0xbb2b0) (unsigned, 32-bit)` |
| `bhs` at 0x74e6c | `ret(0x9ca84) >= [sp-84] (unsigned, 32-bit)` | `ret(0x9ca84) < [sp-84] (unsigned, 32-bit)` |
| `bne` at 0x74e9c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x74ed8 | `ret(0xbb3b8) == 0x0` | `ret(0xbb3b8) != 0x0` |
| `bne` at 0x74f2c | `[[sp-20]+32]:1 != 0x0` | `[[sp-20]+32]:1 == 0x0` |
| `bne` at 0x74f60 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x74f74 | `[[sp-20]+40] >= ret(0xbb2b0) (unsigned, 32-bit)` | `[[sp-20]+40] < ret(0xbb2b0) (unsigned, 32-bit)` |
| `bhs` at 0x74f94 | `ret(0x9ca84) >= [sp-92] (unsigned, 32-bit)` | `ret(0x9ca84) < [sp-92] (unsigned, 32-bit)` |
| `bne` at 0x74fc4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x75008 | `ret(0xbb470) == 0x0` | `ret(0xbb470) != 0x0` |
| `beq` at 0x75070 | `[[sp-20]+32]:1 == 0x0` | `[[sp-20]+32]:1 != 0x0` |
| `bne` at 0x75084 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x750c0 | `[sp-100] != [[sp-20]+32]:1-1` | `[sp-100] == [[sp-20]+32]:1-1` |
| `blt` at 0x75108 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x75134 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7519c | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `bhi` at 0x75210 | `[[sp-20]+32]:1 > 0x3 (unsigned, 32-bit)` | `[[sp-20]+32]:1 <= 0x3 (unsigned, 32-bit)` |
| `bne` at 0x7523c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x75288 | `? == [[sp-20]+32]:1` | `? != [[sp-20]+32]:1` |
| `blt` at 0x752cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x75304 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x75394 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x75424 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x754a4 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bhi` at 0x75504 | `[[sp-20]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-20]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x75528 | `[sp-40] == 0x35` | `[sp-40] != 0x35` |
| `beq` at 0x75534 | `[sp-40] == 0x40` | `[sp-40] != 0x40` |
| `beq` at 0x75540 | `[sp-40] == 0x4f` | `[sp-40] != 0x4f` |
| `beq` at 0x7554c | `[sp-40] == 0x42` | `[sp-40] != 0x42` |
| `beq` at 0x75558 | `[sp-40] == 0x46` | `[sp-40] != 0x46` |
| `beq` at 0x75564 | `[sp-40] == 0x4c` | `[sp-40] != 0x4c` |
| `beq` at 0x75578 | `[0x11b148] == 0x0` | `[0x11b148] != 0x0` |
| `bne` at 0x7558c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x755c0 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x75614 | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x756e4 | `r0` | `?` |

## `EvtIRouter`

- `EvtIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53fe0 to 0x540d4.
  Established. The entry word load addresses `r0 +0x1c` (`ldr` at 0x53ff8). A nonzero byte at `r0
  +0x16` takes the branch that bypasses the byte store and the two dispatch calls (`ldrb` at
  0x54064, `bne` at 0x5406c). On the zero-byte path, a zero halfword at `r0 +0x22` takes the edge to
  the byte store of `0x2` at `r0 +0x16` (`ldrh` at 0x54074, `beq` at 0x5407c, `strb` at 0x5408c).
  Dispatch equality branches compare `[sp-28]` with `0x4` and `0xe`, leading to calls to `0x6e984`
  and `0x5c778`, respectively (`beq` at 0x540a0, `beq` at 0x540ac, `bl` at 0x540c4, `bl` at
  0x540b8). The final call is to `0x9a998`, and the return reports that result in r0 (`bl` at
  0x540cc, `ldmdb` at 0x540d0). The event labels and the callees’ device effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53ff0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x53ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53ff8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x53ffc | store | 4 | `sp -0x18` | `[r0+28]` |
| `ldr` at 0x54000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5400c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54014 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x54018 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x54020 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x54028 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54030 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x54034 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x54038 | store | 4 | `sp -0x1c` | `[sp-40]` |
| `ldr` at 0x5403c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5404c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x54050 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5405c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x54060 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x54064 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x54070 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x54074 | load | 2 | `r0 +0x22` | `-` |
| `ldr` at 0x54084 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x5408c | store | 1 | `r0 +0x16` | `0x2` |
| `ldr` at 0x54090 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54094 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x54098 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x540a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x540b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x540c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x540c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x540d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x540d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x540d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x540b8 | `(unknown callee) 0x5c778` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x540c4 | `(unknown callee) 0x6e984` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x540cc | `(unknown callee) 0x9a998` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x54008 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x5406c | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `beq` at 0x5407c | `[r0+34]:2 == 0x0` | `[r0+34]:2 != 0x0` |
| `beq` at 0x540a0 | `[sp-28] == 0x4` | `[sp-28] != 0x4` |
| `beq` at 0x540ac | `[sp-28] == 0xe` | `[sp-28] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x540d0 | `r0` | `ret(0x9a998)` |

## `SMPCmdIRouter`

- `SMPCmdIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82334 to 0x82648.
  Established. This body repeats from the unconditional branch back to `0x82350` (`b` at 0x8261c).
  The call at the loop entry targets `0x9a93c` with candidates `sp-2104`, `0x282048`, unknown,
  `sp-2104` (`bl` at 0x82368). Two explicit-link calls have unknown targets and first candidates
  `0x1` and `0x2`, respectively (`mov` at 0x82548, `mov` at 0x8259c). A later word store writes
  `0x0` at absolute address `0x11b14c` (`str` at 0x825b0), before a call to `0x9bd94` with
  candidates `0x282040`, `0x0`, `[sp-2104]`, `[sp-32]` (`bl` at 0x825d4). No decoded return is
  reported. The loop’s command interpretation and the absolute word’s role are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82348 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8234c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x82360 | load | 4 | `abs 0x82620` | `-` |
| `str` at 0x82370 | store | 4 | `sp -0x1c` | `ret(0x9a93c)` |
| `ldr` at 0x82374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8238c | load | 4 | `abs 0x82624` | `-` |
| `str` at 0x82390 | store | 4 | `sp -0x848` | `0xf46e0` |
| `str` at 0x82394 | store | 4 | `sp -0x844` | `ret(0x1e71c)` |
| `ldr` at 0x8239c | load | 4 | `abs 0x82628` | `-` |
| `ldr` at 0x823a0 | load | 4 | `abs 0x8262c` | `-` |
| `ldr` at 0x823ac | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x823b0 | load | 1 | `[sp-2104] +0x20` | `-` |
| `str` at 0x823c0 | store | 4 | `sp -0x834` | `0x0` |
| `ldr` at 0x823c8 | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x823cc | load | 1 | `[sp-2104] +0x2c` | `-` |
| `ldr` at 0x823dc | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x823e0 | load | 4 | `[sp-2104] +0x30` | `-` |
| `str` at 0x823e4 | store | 4 | `sp -0x840` | `[[sp-2104]+48]` |
| `ldr` at 0x823ec | load | 4 | `sp -0x838` | `-` |
| `str` at 0x823f8 | store | 4 | `sp -0x840` | `ret(0x9ca84)` |
| `ldr` at 0x823fc | load | 4 | `sp -0x840` | `-` |
| `str` at 0x82400 | store | 4 | `sp -0x834` | `[sp-2112]` |
| `ldr` at 0x82410 | load | 4 | `sp -0x834` | `-` |
| `ldr` at 0x82418 | load | 4 | `sp -0x838` | `-` |
| `str` at 0x82424 | store | 4 | `sp -0x1c` | `ret(0x81090)` |
| `ldr` at 0x82440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82448 | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x82458 | store | 4 | `sp -0x830` | `0x0` |
| `ldr` at 0x8245c | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x82464 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x8246c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8247c | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x82484 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x8248c | load | 4 | `(unknown)` | `-` |
| `str` at 0x82490 | store | 4 | `sp -0x83c` | `?` |
| `ldr` at 0x82494 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x8249c | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x824a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x824c4 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x824cc | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x824d8 | store | 4 | `sp -0x848` | `0x0` |
| `ldr` at 0x824dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x824e8 | load | 4 | `sp -0x830` | `-` |
| `str` at 0x824f0 | store | 4 | `sp -0x830` | `[sp-2096]+1` |
| `ldr` at 0x824f8 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x82500 | load | 4 | `abs 0x82630` | `-` |
| `ldr` at 0x82504 | load | 4 | `abs 0x11b30c` | `-` |
| `ldr` at 0x82510 | load | 4 | `abs 0x82634` | `-` |
| `ldr` at 0x82514 | load | 4 | `abs 0x11b14c` | `-` |
| `ldr` at 0x82524 | load | 4 | `abs 0x82630` | `-` |
| `ldr` at 0x82534 | load | 4 | `abs 0x11b30c` | `-` |
| `ldr` at 0x8253c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x82550 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x82554 | load | 4 | `abs 0x82630` | `-` |
| `ldr` at 0x82558 | load | 4 | `abs 0x11b30c` | `-` |
| `ldr` at 0x82564 | load | 4 | `abs 0x82634` | `-` |
| `ldr` at 0x82568 | load | 4 | `abs 0x11b14c` | `-` |
| `ldr` at 0x82578 | load | 4 | `abs 0x82630` | `-` |
| `ldr` at 0x82588 | load | 4 | `abs 0x11b30c` | `-` |
| `ldr` at 0x82590 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x825a4 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x825a8 | load | 4 | `abs 0x82634` | `-` |
| `str` at 0x825b0 | store | 4 | `abs 0x11b14c` | `0x0` |
| `str` at 0x825c0 | store | 4 | `sp -0x848` | `sp-2088` |
| `ldr` at 0x825c4 | load | 4 | `abs 0x82638` | `-` |
| `ldr` at 0x825cc | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x825d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x825dc | store | 4 | `sp -0x1c` | `ret(0x9bd94)` |
| `ldr` at 0x825e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x825ec | load | 4 | `abs 0x8263c` | `-` |
| `ldr` at 0x825f0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x825f4 | load | 4 | `abs 0x82640` | `-` |
| `ldr` at 0x825fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82608 | load | 4 | `abs 0x82644` | `-` |
| `ldr` at 0x82614 | load | 4 | `sp -0x838` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82368 | `(unknown callee) 0x9a93c` | `sp-2104` | `0x282048` | `?` | `sp-2104` |
| `bl` at 0x82384 | `(unknown callee) 0x1e71c` | `ret(0x9a93c)` | `?` | `?` | `ret(0x9a93c)` |
| `bl` at 0x823a8 | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x1a0c` | `0x0` |
| `bl` at 0x823f4 | `(unknown callee) 0x9ca84` | `[sp-2104]` | `0x1` | `?` | `?` |
| `bl` at 0x8241c | `(unknown callee) 0x81090` | `?` | `sp-2092` | `[sp-2104]` | `sp-2092` |
| `bl` at 0x82444 | `(unknown callee) 0x541d4` | `sp-32` | `sp-2088` | `ret(0x81090)` | `sp-2088` |
| `bl` at 0x824a8 | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x824e4 | `(unknown callee) 0x54450` | `sp-32` | `sp-2088` | `?` | `ret(0x18a1c)` |
| `bl` at 0x824fc | `(unknown callee) 0x18be4` | `[sp-2092]` | `?` | `?` | `?` |
| `mov` at 0x82548 | `(unknown callee) (unknown target)` | `0x1` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `mov` at 0x8259c | `(unknown callee) (unknown target)` | `0x2` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `bl` at 0x825d4 | `(unknown callee) 0x9bd94` | `0x282040` | `0x0` | `[sp-2104]` | `[sp-32]` |
| `bl` at 0x825f8 | `(unknown callee) 0x21010` | `[0x116928]` | `0xf4704` | `?` | `0x116928` |
| `bl` at 0x82600 | `(unknown callee) 0x1e71c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x82610 | `(unknown callee) 0x1dba4` | `0xf4724` | `ret(0x1e71c)` | `?` | `ret(0x1e71c)` |
| `bl` at 0x82618 | `(unknown callee) 0x9a998` | `[sp-2104]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8237c | `ret(0x9a93c) == 0x0` | `ret(0x9a93c) != 0x0` |
| `beq` at 0x823b8 | `[[sp-2104]+32]:1 == 0x2` | `[[sp-2104]+32]:1 != 0x2` |
| `bne` at 0x823d8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x82450 | `[sp-2092] == 0x0` | `[sp-2092] != 0x0` |
| `bne` at 0x82474 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x8250c | `[0x11b30c] == 0x0` | `[0x11b30c] != 0x0` |
| `beq` at 0x82520 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x82560 | `[0x11b30c] == 0x0` | `[0x11b30c] != 0x0` |
| `beq` at 0x82574 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x825e8 | `ret(0x9bd94) == 0x0` | `ret(0x9bd94) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `LpbkIRouter`

- `LpbkIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x89bf8 to 0x8a05c.
  Established. Equality branches select paths for `[sp-100]` equal to `0x0`, `0x1` and `0x2` (`beq`
  at 0x89c88, `beq` at 0x89c70, `beq` at 0x89c98). These paths include byte stores of `0x0`, `0x1`
  and `0x2` at unresolved effective addresses (`strb` at 0x89cbc, `strb` at 0x89da4, `strb` at
  0x89ea4). Another halfword store writes `(ret(0x18a1c)+1 & 0xffff)` at `[sp-92] +0x2` (`strh` at
  0x89fd8). The call to `0x9c10c` has candidates `0x2821e0`, `sp-24`, `0x0`, `0x1` (`bl` at
  0x8a018); its nonzero result takes the edge to a call to `0x9b9c4`, while zero falls through
  to a call to `0x1d6d8` (`bne` at 0x8a02c, `bl` at 0x8a044, `bl` at 0x8a034). What those
  constructed values represent is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x89c08 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x89c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89c10 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x89c14 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0x89c1c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x89c20 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x89c24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89c28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89c34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89c48 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x89c5c | load | 1 | `(unknown)` | `-` |
| `str` at 0x89c64 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x89c68 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x89c74 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x89c80 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x89c90 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x89ca0 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x89cbc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x89cc0 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x89cdc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x89ce0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89cf8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x89cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89d00 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x89d14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89d24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89d28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89d3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x89d40 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x89d48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89d4c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89d54 | store | 4 | `sp -0x48` | `ret(0x9ca84)` |
| `ldr` at 0x89d58 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x89d5c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x89d60 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x89d64 | load | 4 | `sp -0x40` | `-` |
| `strh` at 0x89d80 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x89d88 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x89da4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x89da8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x89dc4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x89dc8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89de0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x89de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89de8 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x89dfc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89e0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89e10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89e24 | load | 4 | `(unknown)` | `-` |
| `str` at 0x89e28 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x89e30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89e34 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89e3c | store | 4 | `sp -0x50` | `ret(0x9ca84)` |
| `ldr` at 0x89e40 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x89e44 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x89e48 | store | 4 | `[sp-76] +0x0` | `?` |
| `ldr` at 0x89e4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89e68 | load | 4 | `sp -0x40` | `-` |
| `ldrh` at 0x89e7c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x89e80 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x89e88 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x89ea4 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x89ea8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x89ec4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x89ec8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89ee0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x89ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89ee8 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x89efc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89f10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89f24 | load | 4 | `(unknown)` | `-` |
| `str` at 0x89f28 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x89f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89f34 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89f3c | store | 4 | `sp -0x58` | `ret(0x9ca84)` |
| `ldr` at 0x89f40 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x89f44 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x89f48 | store | 4 | `[sp-84] +0x0` | `?` |
| `ldr` at 0x89f4c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89f64 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x89f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89f6c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x89f80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89f90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89f94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x89fa8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x89fac | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x89fb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x89fb8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89fc0 | store | 4 | `sp -0x60` | `ret(0x9ca84)` |
| `ldr` at 0x89fc4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x89fd4 | load | 4 | `sp -0x5c` | `-` |
| `strh` at 0x89fd8 | store | 2 | `[sp-92] +0x2` | `(ret(0x18a1c)+1 & 0xffff)` |
| `ldr` at 0x89fdc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89fe4 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x89ff4 | store | 4 | `sp -0x70` | `0x2` |
| `ldr` at 0x89ff8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x89ffc | store | 4 | `sp -0x6c` | `?` |
| `str` at 0x8a004 | store | 4 | `sp -0x68` | `sp-60` |
| `ldr` at 0x8a008 | load | 4 | `abs 0x8a054` | `-` |
| `str` at 0x8a020 | store | 4 | `sp -0x1c` | `ret(0x9c10c)` |
| `ldr` at 0x8a024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8a030 | load | 4 | `abs 0x8a058` | `-` |
| `ldr` at 0x8a03c | load | 4 | `abs 0x8a054` | `-` |
| `ldr` at 0x8a040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8a048 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8a050 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8a050 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8a050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x89d50 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0x89e38 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0x89f38 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0x89fbc | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0x89fc8 | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x8a018 | `(unknown callee) 0x9c10c` | `0x2821e0` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0x8a034 | `(unknown callee) 0x1d6d8` | `0xfa358` | `?` | `?` | `ret(0x9c10c)` |
| `bl` at 0x8a044 | `(unknown callee) 0x9b9c4` | `0x2821e0` | `ret(0x9c10c)` | `?` | `ret(0x9c10c)` |
| `bl` at 0x8a04c | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x89c30 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x89c3c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x89c70 | `[sp-100] == 0x1` | `[sp-100] != 0x1` |
| `bgt` at 0x89c7c | `[sp-100] > 0x1 (signed, 32-bit)` | `[sp-100] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x89c88 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0x89c98 | `[sp-100] == 0x2` | `[sp-100] != 0x2` |
| `bne` at 0x89d20 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89e08 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89f08 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89f8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8a02c | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8a050 | `r0` | `ret(0x9a998)` |

## `FlashIRouter`

- `FlashIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8b3b8 to 0x8bc3c.
  Established. The halfword load addresses `[sp-20] +0x22` (`ldrh` at 0x8b3e4). The branch tree
  includes equality tests of `[sp-32]` against `0x73`, `0x72` and `0x74` (`beq` at 0x8b418, `beq` at
  0x8b430, `beq` at 0x8b440). The body includes calls to `0xaef6c` with candidates `[sp-164]`,
  `[sp-184]`, `sp-168`, `sp-168`, and to `0xaef9c` with candidates `[sp-164]`, `[sp-196]`,
  `[[sp-20]+46]:2`, `sp-172` (`bl` at 0x8ba3c, `bl` at 0x8bb68). The first call’s signed-nonnegative
  result takes the branch (`bge` at 0x8ba70). A negative result falls through into stores that
  include `[sp-168]` at `sp -0x7c` (`bge` at 0x8ba70, `str` at 0x8ba88). Those callees’ effects are
  not established by this body, and its return value is unknown (`ldmdb` at 0x8bc24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8b3c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8b3d0 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x8b3dc | store | 4 | `sp -0x1c` | `ret(0x2b31c)` |
| `ldr` at 0x8b3e0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x8b3e4 | load | 2 | `[sp-20] +0x22` | `-` |
| `str` at 0x8b3e8 | store | 4 | `sp -0x20` | `[[sp-20]+34]:2` |
| `str` at 0x8b3f0 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x8b3f4 | load | 4 | `abs 0x8bc28` | `-` |
| `ldr` at 0x8b3f8 | load | 4 | `abs 0x11b2d8` | `-` |
| `ldr` at 0x8b408 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8b40c | store | 4 | `sp -0xcc` | `[sp-32]` |
| `ldr` at 0x8b410 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x8b41c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x8b428 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x8b438 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x8b448 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b44c | load | 4 | `abs 0x284074` | `-` |
| `ldr` at 0x8b458 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8b45c | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x8b468 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8b46c | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x8b47c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b480 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x8b490 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8b4b0 | store | 4 | `sp -0x94` | `0x5a` |
| `ldr` at 0x8b4b4 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b4b8 | load | 4 | `abs 0x284074` | `-` |
| `ldr` at 0x8b4c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8b4c8 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x8b4d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8b4d8 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x8b4e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b4ec | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x8b4fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8b51c | store | 4 | `sp -0x94` | `0x5a` |
| `strb` at 0x8b524 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0x8b52c | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0x8b530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8b534 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8b53c | store | 4 | `sp -0x84` | `([sp-24] + [sp-148])` |
| `strh` at 0x8b544 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0x8b548 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b54c | load | 4 | `abs 0x284088` | `-` |
| `strb` at 0x8b55c | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x8b564 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0x8b568 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b56c | load | 4 | `abs 0x284088` | `-` |
| `ldr` at 0x8b570 | load | 4 | `[0x284088] +0x0` | `-` |
| `str` at 0x8b574 | store | 4 | `sp -0x7c` | `[[0x284088]]` |
| `strh` at 0x8b57c | store | 2 | `sp -0x7e` | `0x0` |
| `strb` at 0x8b588 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x8b590 | store | 1 | `sp -0x7f` | `0x0` |
| `str` at 0x8b598 | store | 4 | `sp -0x7c` | `0x0` |
| `strh` at 0x8b5a0 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0x8b5a4 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b5a8 | load | 4 | `abs 0x284068` | `-` |
| `ldr` at 0x8b5b4 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b5b8 | load | 4 | `abs 0x8bc2c` | `-` |
| `str` at 0x8b5c0 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0x8b5c8 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0x8b5d0 | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0x8b5d8 | load | 4 | `abs 0x284068` | `-` |
| `ldr` at 0x8b5dc | load | 4 | `abs 0x284078` | `-` |
| `str` at 0x8b5ec | store | 4 | `sp -0x90` | `ret(0x8acc8)` |
| `ldr` at 0x8b5f0 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x8b600 | store | 1 | `sp -0x78` | `0x0` |
| `strb` at 0x8b608 | store | 1 | `sp -0x77` | `0x0` |
| `str` at 0x8b610 | store | 4 | `sp -0x74` | `0x0` |
| `strh` at 0x8b618 | store | 2 | `sp -0x76` | `0x0` |
| `strb` at 0x8b620 | store | 1 | `sp -0x70` | `0x0` |
| `strb` at 0x8b628 | store | 1 | `sp -0x6f` | `0x0` |
| `str` at 0x8b630 | store | 4 | `sp -0x6c` | `0x0` |
| `strh` at 0x8b638 | store | 2 | `sp -0x6e` | `0x0` |
| `ldr` at 0x8b640 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b644 | load | 4 | `abs 0x284068` | `-` |
| `ldr` at 0x8b650 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b654 | load | 4 | `abs 0x284068` | `-` |
| `str` at 0x8b65c | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x8b660 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8b664 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x8b670 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8b678 | store | 4 | `sp -0xa4` | `(ret(0x1874c) + [sp-160])` |
| `strb` at 0x8b680 | store | 1 | `sp -0x78` | `0x1` |
| `strb` at 0x8b688 | store | 1 | `sp -0x77` | `0x0` |
| `ldr` at 0x8b68c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x8b690 | store | 4 | `sp -0x74` | `(ret(0x1874c) + [sp-160])` |
| `ldr` at 0x8b694 | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0x8b6a4 | store | 2 | `sp -0x76` | `(ret(0x18a1c)+1 & 0xffff)` |
| `ldr` at 0x8b6a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8b6ac | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x8b6b8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8b6c0 | store | 4 | `sp -0xa4` | `(ret(0x1874c) + [sp-160])` |
| `strb` at 0x8b6c8 | store | 1 | `sp -0x70` | `0x1` |
| `strb` at 0x8b6d0 | store | 1 | `sp -0x6f` | `0x0` |
| `ldr` at 0x8b6d4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x8b6d8 | store | 4 | `sp -0x6c` | `(ret(0x1874c) + [sp-160])` |
| `ldr` at 0x8b6dc | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0x8b6ec | store | 2 | `sp -0x6e` | `(ret(0x18a1c)+1 & 0xffff)` |
| `ldr` at 0x8b6f0 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b6f4 | load | 4 | `abs 0x28406c` | `-` |
| `ldr` at 0x8b700 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b704 | load | 4 | `abs 0x28406c` | `-` |
| `ldr` at 0x8b710 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b714 | load | 4 | `abs 0x8bc2c` | `-` |
| `str` at 0x8b71c | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0x8b724 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0x8b72c | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0x8b734 | load | 4 | `abs 0x28406c` | `-` |
| `ldr` at 0x8b738 | load | 4 | `abs 0x284078` | `-` |
| `str` at 0x8b748 | store | 4 | `sp -0x90` | `ret(0x8acc8)` |
| `str` at 0x8b754 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x8b758 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x8b768 | store | 1 | `sp -0x68` | `0x0` |
| `strb` at 0x8b770 | store | 1 | `sp -0x67` | `0x0` |
| `str` at 0x8b778 | store | 4 | `sp -0x64` | `0x0` |
| `strh` at 0x8b780 | store | 2 | `sp -0x66` | `0x0` |
| `strb` at 0x8b788 | store | 1 | `sp -0x60` | `0x0` |
| `strb` at 0x8b790 | store | 1 | `sp -0x5f` | `0x0` |
| `str` at 0x8b798 | store | 4 | `sp -0x5c` | `0x0` |
| `strh` at 0x8b7a0 | store | 2 | `sp -0x5e` | `0x0` |
| `ldr` at 0x8b7a8 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b7ac | load | 4 | `abs 0x28406c` | `-` |
| `ldr` at 0x8b7b8 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b7bc | load | 4 | `abs 0x28406c` | `-` |
| `str` at 0x8b7c4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x8b7c8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8b7cc | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x8b7d8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x8b7e0 | store | 4 | `sp -0xa0` | `(ret(0x1874c) + [sp-164])` |
| `strb` at 0x8b7e8 | store | 1 | `sp -0x68` | `0x1` |
| `strb` at 0x8b7f0 | store | 1 | `sp -0x67` | `0x0` |
| `ldr` at 0x8b7f4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8b7f8 | store | 4 | `sp -0x64` | `(ret(0x1874c) + [sp-164])` |
| `ldr` at 0x8b7fc | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0x8b80c | store | 2 | `sp -0x66` | `(ret(0x18a1c)+1 & 0xffff)` |
| `ldr` at 0x8b810 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8b814 | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x8b820 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x8b828 | store | 4 | `sp -0xa0` | `(ret(0x1874c) + [sp-164])` |
| `strb` at 0x8b830 | store | 1 | `sp -0x60` | `0x1` |
| `strb` at 0x8b838 | store | 1 | `sp -0x5f` | `0x0` |
| `ldr` at 0x8b83c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8b840 | store | 4 | `sp -0x5c` | `(ret(0x1874c) + [sp-164])` |
| `ldr` at 0x8b844 | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0x8b854 | store | 2 | `sp -0x5e` | `(ret(0x18a1c)+1 & 0xffff)` |
| `strb` at 0x8b85c | store | 1 | `sp -0x58` | `0x0` |
| `strb` at 0x8b864 | store | 1 | `sp -0x57` | `0x0` |
| `ldr` at 0x8b868 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b86c | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b870 | load | 4 | `abs 0x284070` | `-` |
| `ldr` at 0x8b874 | load | 4 | `abs 0x284068` | `-` |
| `str` at 0x8b884 | store | 4 | `sp -0x54` | `?` |
| `strh` at 0x8b88c | store | 2 | `sp -0x56` | `0x0` |
| `strb` at 0x8b894 | store | 1 | `sp -0x50` | `0x0` |
| `strb` at 0x8b89c | store | 1 | `sp -0x4f` | `0x0` |
| `ldr` at 0x8b8a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8b8a4 | store | 4 | `sp -0x4c` | `[sp-28]` |
| `strh` at 0x8b8ac | store | 2 | `sp -0x4e` | `0x0` |
| `strb` at 0x8b8b4 | store | 1 | `sp -0x48` | `0x0` |
| `strb` at 0x8b8bc | store | 1 | `sp -0x47` | `0x0` |
| `ldr` at 0x8b8c0 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b8c4 | load | 4 | `abs 0x284074` | `-` |
| `str` at 0x8b8c8 | store | 4 | `sp -0x44` | `[0x284074]` |
| `strh` at 0x8b8d0 | store | 2 | `sp -0x46` | `0x0` |
| `strb` at 0x8b8d8 | store | 1 | `sp -0x40` | `0x0` |
| `strb` at 0x8b8e0 | store | 1 | `sp -0x3f` | `0x0` |
| `ldr` at 0x8b8e4 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b8e8 | load | 4 | `abs 0x284068` | `-` |
| `str` at 0x8b8ec | store | 4 | `sp -0x3c` | `[0x284068]` |
| `strh` at 0x8b8f4 | store | 2 | `sp -0x3e` | `0x0` |
| `strb` at 0x8b8fc | store | 1 | `sp -0x38` | `0x0` |
| `strb` at 0x8b904 | store | 1 | `sp -0x37` | `0x0` |
| `ldr` at 0x8b908 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b90c | load | 4 | `abs 0x28406c` | `-` |
| `str` at 0x8b910 | store | 4 | `sp -0x34` | `[0x28406c]` |
| `strh` at 0x8b918 | store | 2 | `sp -0x36` | `0x0` |
| `str` at 0x8b920 | store | 4 | `sp -0x8c` | `0xb` |
| `ldr` at 0x8b924 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b928 | load | 4 | `abs 0x284084` | `-` |
| `ldr` at 0x8b934 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b938 | load | 4 | `abs 0x284080` | `-` |
| `strb` at 0x8b948 | store | 1 | `sp -0x30` | `0x0` |
| `strb` at 0x8b950 | store | 1 | `sp -0x2f` | `0x0` |
| `ldr` at 0x8b954 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b958 | load | 4 | `abs 0x284084` | `-` |
| `str` at 0x8b95c | store | 4 | `sp -0x2c` | `[0x284084]` |
| `strh` at 0x8b964 | store | 2 | `sp -0x2e` | `0x0` |
| `strb` at 0x8b96c | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0x8b974 | store | 1 | `sp -0x27` | `0x0` |
| `ldr` at 0x8b978 | load | 4 | `abs 0x8bc2c` | `-` |
| `ldr` at 0x8b97c | load | 4 | `abs 0x284080` | `-` |
| `str` at 0x8b980 | store | 4 | `sp -0x24` | `[0x284080]` |
| `strh` at 0x8b988 | store | 2 | `sp -0x26` | `0x0` |
| `str` at 0x8b990 | store | 4 | `sp -0x8c` | `0xd` |
| `str` at 0x8b998 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0x8b9a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b9a8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x8b9b4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8b9b8 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x8b9c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b9cc | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x8b9d0 | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0x8b9d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8b9e4 | store | 4 | `sp -0xb4` | `ret(0x9ca84)` |
| `ldr` at 0x8b9e8 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x8b9ec | store | 4 | `sp -0xa4` | `[sp-180]` |
| `ldr` at 0x8b9f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8b9f4 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x8ba04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ba08 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x8ba0c | store | 4 | `sp -0xb8` | `[[sp-20]+48]` |
| `ldr` at 0x8ba14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8ba20 | store | 4 | `sp -0xb8` | `ret(0x9ca84)` |
| `ldr` at 0x8ba24 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x8ba28 | store | 4 | `sp -0xa0` | `[sp-184]` |
| `ldr` at 0x8ba30 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x8ba34 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8ba44 | store | 4 | `sp -0x90` | `ret(0xaef6c)` |
| `strb` at 0x8ba4c | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0x8ba54 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0x8ba58 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x8ba5c | store | 4 | `sp -0x84` | `ret(0xaef6c)` |
| `strh` at 0x8ba64 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0x8ba68 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x8ba78 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x8ba80 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0x8ba84 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x8ba88 | store | 4 | `sp -0x7c` | `[sp-168]` |
| `strh` at 0x8ba90 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0x8ba94 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x8baa4 | store | 4 | `sp -0xbc` | `0x2` |
| `str` at 0x8bab0 | store | 4 | `sp -0xbc` | `0x1` |
| `str` at 0x8bab8 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0x8bac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8bac8 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x8bad4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8bad8 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x8bae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8baec | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x8baf0 | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0x8baf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8bb04 | store | 4 | `sp -0xc0` | `ret(0x9ca84)` |
| `ldr` at 0x8bb08 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x8bb0c | store | 4 | `sp -0xa4` | `[sp-192]` |
| `ldr` at 0x8bb10 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8bb14 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x8bb24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8bb28 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x8bb2c | store | 4 | `sp -0xc4` | `[[sp-20]+48]` |
| `ldr` at 0x8bb34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8bb40 | store | 4 | `sp -0xc4` | `ret(0x9ca84)` |
| `ldr` at 0x8bb44 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x8bb48 | store | 4 | `sp -0xb0` | `[sp-196]` |
| `ldr` at 0x8bb4c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x8bb50 | load | 2 | `[sp-20] +0x2e` | `-` |
| `str` at 0x8bb54 | store | 4 | `sp -0xa0` | `[[sp-20]+46]:2` |
| `ldr` at 0x8bb5c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x8bb60 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8bb64 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8bb70 | store | 4 | `sp -0x90` | `ret(0xaef9c)` |
| `strb` at 0x8bb78 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0x8bb80 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0x8bb84 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x8bb88 | store | 4 | `sp -0x84` | `ret(0xaef9c)` |
| `strh` at 0x8bb90 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0x8bb94 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x8bba4 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x8bbac | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0x8bbb0 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x8bbb4 | store | 4 | `sp -0x7c` | `[sp-172]` |
| `strh` at 0x8bbbc | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0x8bbc0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x8bbd0 | store | 4 | `sp -0xc8` | `0x2` |
| `str` at 0x8bbdc | store | 4 | `sp -0xc8` | `0x1` |
| `str` at 0x8bbe4 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0x8bbf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8bbf4 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x8bc00 | load | 4 | `abs 0x8bc30` | `-` |
| `str` at 0x8bc04 | store | 4 | `sp -0xd8` | `0xfb0bc` |
| `ldr` at 0x8bc08 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8bc0c | store | 4 | `sp -0xd4` | `[sp-32]` |
| `ldr` at 0x8bc14 | load | 4 | `abs 0x8bc34` | `-` |
| `ldr` at 0x8bc18 | load | 4 | `abs 0x8bc38` | `-` |
| `ldmdb` at 0x8bc24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8bc24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8bc24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8b3d4 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x8b404 | `(unknown callee) 0x8b390` | `ret(0x2b31c)` | `?` | `?` | `[0x11b2d8]` |
| `bl` at 0x8b498 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x8b504 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x8b5ac | `(unknown callee) 0xaed2c` | `[0x284068]` | `?` | `[sp-24]` | `0x284060` |
| `bl` at 0x8b5e4 | `(unknown callee) 0x8acc8` | `ret(0xaed2c)` | `[0x284068]` | `[0x284078]` | `0x0` |
| `bl` at 0x8b648 | `(unknown callee) 0xaed2c` | `[0x284068]` | `?` | `?` | `0x284060` |
| `bl` at 0x8b668 | `(unknown callee) 0x1874c` | `[[sp-152]+20]` | `?` | `ret(0xaed2c)` | `[sp-152]` |
| `bl` at 0x8b698 | `(unknown callee) 0x18a1c` | `(ret(0x1874c) + [sp-160])` | `?` | `ret(0x1874c)` | `(ret(0x1874c) + [sp-160])` |
| `bl` at 0x8b6b0 | `(unknown callee) 0x1874c` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0x8b6e0 | `(unknown callee) 0x18a1c` | `(ret(0x1874c) + [sp-160])` | `?` | `ret(0x1874c)` | `(ret(0x1874c) + [sp-160])` |
| `bl` at 0x8b708 | `(unknown callee) 0xaed2c` | `[0x28406c]` | `?` | `?` | `0x284060` |
| `bl` at 0x8b740 | `(unknown callee) 0x8acc8` | `ret(0xaed2c)` | `[0x28406c]` | `[0x284078]` | `0x0` |
| `bl` at 0x8b7b0 | `(unknown callee) 0xaed2c` | `[0x28406c]` | `?` | `?` | `0x284060` |
| `bl` at 0x8b7d0 | `(unknown callee) 0x1874c` | `[[sp-152]+20]` | `?` | `ret(0xaed2c)` | `[sp-152]` |
| `bl` at 0x8b800 | `(unknown callee) 0x18a1c` | `(ret(0x1874c) + [sp-164])` | `?` | `ret(0x1874c)` | `(ret(0x1874c) + [sp-164])` |
| `bl` at 0x8b818 | `(unknown callee) 0x1874c` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0x8b848 | `(unknown callee) 0x18a1c` | `(ret(0x1874c) + [sp-164])` | `?` | `ret(0x1874c)` | `(ret(0x1874c) + [sp-164])` |
| `bl` at 0x8b9ac | `(unknown callee) 0x9bd94` | `0x0` | `0x0` | `[sp-20]` | `?` |
| `bl` at 0x8b9e0 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x8ba1c | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x8ba3c | `(unknown callee) 0xaef6c` | `[sp-164]` | `[sp-184]` | `sp-168` | `sp-168` |
| `bl` at 0x8bacc | `(unknown callee) 0x9bd94` | `0x0` | `0x0` | `[sp-20]` | `[sp-188]` |
| `bl` at 0x8bb00 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x8bb3c | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x8bb68 | `(unknown callee) 0xaef9c` | `[sp-164]` | `[sp-196]` | `[[sp-20]+46]:2` | `sp-172` |
| `bl` at 0x8bbf8 | `(unknown callee) 0x9bd94` | `0x0` | `0x0` | `[sp-20]` | `[sp-200]` |
| `bl` at 0x8bc20 | `(unknown callee) 0x1eed0` | `0x4` | `0xfb028` | `0xcdc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8b400 | `[0x11b2d8] != 0x0` | `[0x11b2d8] == 0x0` |
| `beq` at 0x8b418 | `[sp-32] == 0x73` | `[sp-32] != 0x73` |
| `bgt` at 0x8b424 | `[sp-32] > 0x73 (signed, 32-bit)` | `[sp-32] <= 0x73 (signed, 32-bit)` |
| `beq` at 0x8b430 | `[sp-32] == 0x72` | `[sp-32] != 0x72` |
| `beq` at 0x8b440 | `[sp-32] == 0x74` | `[sp-32] != 0x74` |
| `bne` at 0x8b454 | `[0x284074] != 0x2` | `[0x284074] == 0x2` |
| `bls` at 0x8b464 | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8b478 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x8b488 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0x8b4a4 | `ret(0x9ca84) <= 0x3 (unsigned, 32-bit)` | `ret(0x9ca84) > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x8b4c0 | `[0x284074] != 0x3` | `[0x284074] == 0x3` |
| `bls` at 0x8b4d0 | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8b4e4 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x8b4f4 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0x8b510 | `ret(0x9ca84) <= 0x3 (unsigned, 32-bit)` | `ret(0x9ca84) > 0x3 (unsigned, 32-bit)` |
| `beq` at 0x8b554 | `[0x284088] == 0x0` | `[0x284088] != 0x0` |
| `bge` at 0x8b5f8 | `ret(0x8acc8) >= 0x0 (signed, 32-bit)` | `ret(0x8acc8) < 0x0 (signed, 32-bit)` |
| `beq` at 0x8b6fc | `[0x28406c] == 0x0` | `[0x28406c] != 0x0` |
| `bge` at 0x8b760 | `[sp-144] >= 0x0 (signed, 32-bit)` | `[sp-144] < 0x0 (signed, 32-bit)` |
| `beq` at 0x8b930 | `[0x284084] == 0x0` | `[0x284084] != 0x0` |
| `beq` at 0x8b940 | `[0x284080] == 0x0` | `[0x284080] != 0x0` |
| `bne` at 0x8b9c4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ba00 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x8ba70 | `ret(0xaef6c) >= 0x0 (signed, 32-bit)` | `ret(0xaef6c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x8ba9c | `ret(0xaef6c) >= 0x0 (signed, 32-bit)` | `ret(0xaef6c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x8bae4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8bb20 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x8bb9c | `ret(0xaef9c) >= 0x0 (signed, 32-bit)` | `ret(0xaef9c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x8bbc8 | `ret(0xaef9c) >= 0x0 (signed, 32-bit)` | `ret(0xaef9c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8bc24 | `r0` | `?` |

## `RouteIRouter`

- `RouteIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x96618 to 0x967e0.
  Established. The indexed construction includes zero byte stores and a zero halfword store at
  unresolved addresses (`strb` at 0x96680, `strb` at 0x966a0, `strh` at 0x96744). A word store uses
  `[sp-548] +0x0` with unknown value (`str` at 0x96724). The call to `0x9c10c` has candidates
  `0x282360`, `sp-24`, `0x0`, `0x1` (`bl` at 0x96784). A nonzero result takes the edge to a word
  store of `[[sp-20]+12]` at `ret(0x9c10c) +0xc` (`bne` at 0x96798, `str` at 0x967b4). That path
  calls `0x9b9c4`, and the joined path calls `0x9a998` (`bl` at 0x967c0, `bl` at 0x967c8).
  Route-field meanings and external effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x96628 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9662c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96630 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x96634 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0x9663c | store | 4 | `sp -0x220` | `0x0` |
| `ldr` at 0x96640 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x96644 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x96648 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x96654 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x96664 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x96668 | load | 4 | `abs 0x967d0` | `-` |
| `strb` at 0x96680 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x96684 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x96688 | load | 4 | `abs 0x967d0` | `-` |
| `strb` at 0x966a0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x966a4 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x966a8 | load | 4 | `abs 0x967d4` | `-` |
| `str` at 0x966bc | store | 4 | `sp -0x224` | `?` |
| `ldr` at 0x966c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x966c4 | load | 4 | `sp -0x220` | `-` |
| `ldrb` at 0x966d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x966e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x966ec | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x96700 | load | 4 | `(unknown)` | `-` |
| `str` at 0x96704 | store | 4 | `sp -0x228` | `?` |
| `ldr` at 0x9670c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96710 | load | 4 | `sp -0x220` | `-` |
| `str` at 0x96718 | store | 4 | `sp -0x228` | `ret(0x9ca84)` |
| `ldr` at 0x9671c | load | 4 | `sp -0x228` | `-` |
| `ldr` at 0x96720 | load | 4 | `sp -0x224` | `-` |
| `str` at 0x96724 | store | 4 | `[sp-548] +0x0` | `?` |
| `ldr` at 0x96728 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0x9672c | load | 4 | `abs 0x967d0` | `-` |
| `strh` at 0x96744 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x96748 | load | 4 | `sp -0x220` | `-` |
| `str` at 0x96750 | store | 4 | `sp -0x220` | `[sp-544]+1` |
| `str` at 0x96760 | store | 4 | `sp -0x234` | `0x2` |
| `ldr` at 0x96764 | load | 4 | `sp -0x220` | `-` |
| `str` at 0x96768 | store | 4 | `sp -0x230` | `?` |
| `str` at 0x96770 | store | 4 | `sp -0x22c` | `sp-540` |
| `ldr` at 0x96774 | load | 4 | `abs 0x967d8` | `-` |
| `str` at 0x9678c | store | 4 | `sp -0x1c` | `ret(0x9c10c)` |
| `ldr` at 0x96790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9679c | load | 4 | `abs 0x967dc` | `-` |
| `ldr` at 0x967a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x967ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x967b0 | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0x967b4 | store | 4 | `ret(0x9c10c) +0xc` | `[[sp-20]+12]` |
| `ldr` at 0x967b8 | load | 4 | `abs 0x967d8` | `-` |
| `ldr` at 0x967bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x967c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x967cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x967cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x967cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x96714 | `(unknown callee) 0x9ca84` | `?` | `?` | `0x24` | `?` |
| `bl` at 0x96784 | `(unknown callee) 0x9c10c` | `0x282360` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0x967a0 | `(unknown callee) 0x1d6d8` | `0xfc4b4` | `?` | `?` | `ret(0x9c10c)` |
| `bl` at 0x967c0 | `(unknown callee) 0x9b9c4` | `0x282360` | `[sp-28]` | `[[sp-20]+12]` | `ret(0x9c10c)` |
| `bl` at 0x967c8 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x96650 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x9665c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x966e4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x96798 | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x967cc | `r0` | `ret(0x9a998)` |

## `IRouterSetFrameByteOrder`

- `IRouterSetFrameByteOrder`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c808 to 0x9ca84.
  Established. The halfword at `r0 +0x6` is compared with entry r1: inequality takes the path
  containing calls and stores, while equality falls through to the return (`ldrh` at 0x9c824,
  `bne` at 0x9c830). The next branch takes the non-`0x2` path for `r1 != 0x2` and falls through for
  equality (`bne` at 0x9c840). The equality path includes calls to `0x189f8` and `0x1874c`, and
  ultimately stores halfword `0x2` at `[sp-24] +0x6` (`bl` at 0x9c854, `bl` at 0x9c86c, `strh` at
  0x9c95c). The other path includes calls to `0x18c08` and `0x1886c`, followed by halfword `0x1` at
  the same effective offset (`bl` at 0x9c974, `bl` at 0x9c98c, `strh` at 0x9ca7c). Both paths also
  contain indexed stores whose effective targets remain unresolved (`strh` at 0x9c904, `str` at
  0x9c940, `strh` at 0x9ca24, `str` at 0x9ca60). The numeric callees’ transformations and the return
  value are unknown (`ldmdb` at 0x9ca80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c818 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x9c81c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x9c820 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9c824 | load | 2 | `r0 +0x6` | `-` |
| `ldr` at 0x9c828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9c838 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9c844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c848 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9c84c | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0x9c85c | store | 2 | `r0 +0x10` | `(ret(0x189f8) & 0xffff)` |
| `ldr` at 0x9c860 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c868 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0x9c870 | store | 4 | `[sp-24] +0x18` | `ret(0x1874c)` |
| `ldr` at 0x9c874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c87c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x9c884 | store | 4 | `[sp-24] +0x1c` | `ret(0x1874c)` |
| `ldr` at 0x9c888 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c88c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9c890 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0x9c8a0 | store | 2 | `[sp-24] +0x22` | `(ret(0x189f8) & 0xffff)` |
| `str` at 0x9c8a8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9c8ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9c8b0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x9c8b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c8c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c8c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c8dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c8e0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x9c8f4 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x9c904 | store | 2 | `(unknown)` | `(ret(0x189f8) & 0xffff)` |
| `ldr` at 0x9c908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c90c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c924 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c938 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c940 | store | 4 | `(unknown)` | `ret(0x1874c)` |
| `ldr` at 0x9c944 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9c94c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x9c954 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x9c95c | store | 2 | `[sp-24] +0x6` | `0x2` |
| `ldr` at 0x9c964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c968 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9c96c | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0x9c97c | store | 2 | `r0 +0x10` | `(ret(0x18c08) & 0xffff)` |
| `ldr` at 0x9c980 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c988 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0x9c990 | store | 4 | `[sp-24] +0x18` | `ret(0x1886c)` |
| `ldr` at 0x9c994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c998 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c99c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x9c9a4 | store | 4 | `[sp-24] +0x1c` | `ret(0x1886c)` |
| `ldr` at 0x9c9a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c9ac | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9c9b0 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0x9c9c0 | store | 2 | `[sp-24] +0x22` | `(ret(0x18c08) & 0xffff)` |
| `str` at 0x9c9c8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9c9cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9c9d0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x9c9d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c9e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c9e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c9fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ca00 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x9ca14 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x9ca24 | store | 2 | `(unknown)` | `(ret(0x18c08) & 0xffff)` |
| `ldr` at 0x9ca28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ca2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ca40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ca44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ca58 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9ca60 | store | 4 | `(unknown)` | `ret(0x1886c)` |
| `ldr` at 0x9ca64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ca6c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x9ca74 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x9ca7c | store | 2 | `[sp-24] +0x6` | `0x1` |
| `ldmdb` at 0x9ca80 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x9ca80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9ca80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9ca80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c854 | `(unknown callee) 0x189f8` | `[r0+16]:2` | `r1` | `[r0+6]:2` | `[r0+16]:2` |
| `bl` at 0x9c86c | `(unknown callee) 0x1874c` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9c880 | `(unknown callee) 0x1874c` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9c898 | `(unknown callee) 0x189f8` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0x9c8fc | `(unknown callee) 0x189f8` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0x9c93c | `(unknown callee) 0x1874c` | `?` | `[sp-24]` | `0x28` | `?` |
| `bl` at 0x9c974 | `(unknown callee) 0x18c08` | `[r0+16]:2` | `r1` | `[r0+6]:2` | `[r0+16]:2` |
| `bl` at 0x9c98c | `(unknown callee) 0x1886c` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9c9a0 | `(unknown callee) 0x1886c` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9c9b8 | `(unknown callee) 0x18c08` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0x9ca1c | `(unknown callee) 0x18c08` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0x9ca5c | `(unknown callee) 0x1886c` | `?` | `[sp-24]` | `0x28` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9c830 | `[r0+6]:2 != r1` | `[r0+6]:2 == r1` |
| `bne` at 0x9c840 | `r1 != 0x2` | `r1 == 0x2` |
| `blt` at 0x9c8bc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x9c9dc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9ca80 | `r0` | `?` |

## `EnvPowerUpDone`

- `EnvPowerUpDone`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b9d0 to 0x6b9fc.
  Established. A call to `0x189c8` has candidates `0x0`, entry r1, entry r2 and entry r3 (`bl` at
  0x6b9e4); its result is stored as a word at absolute address `0x11a790` (`str` at 0x6b9e8). A
  subsequent call to `0x69e48` uses first candidate `0x4` with the others unknown (`bl` at 0x6b9f0),
  and its result reaches r0 at the return (`ldmdb` at 0x6b9f4). The stored word’s power-state
  meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6b9dc | load | 4 | `abs 0x6b9f8` | `-` |
| `str` at 0x6b9e8 | store | 4 | `abs 0x11a790` | `ret(0x189c8)` |
| `ldmdb` at 0x6b9f4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b9f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b9f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b9f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b9e4 | `(unknown callee) 0x189c8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6b9f0 | `(unknown callee) 0x69e48` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b9f4 | `r0` | `ret(0x69e48)` |

## `EnvPowerDown`

- `EnvPowerDown`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b9fc to 0x6ba1c.
  Established. The wrapper saves entry r0 at `sp -0x14` (`str` at 0x6ba0c) and calls `0x64eec` with
  entry r0 through r3 as candidates (`bl` at 0x6ba14). The return reports `ret(0x64eec)` in r0
  (`ldmdb` at 0x6ba18). Whether the callee changes power hardware is unknown here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ba0c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ba10 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6ba18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ba18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ba18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ba14 | `(unknown callee) 0x64eec` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ba18 | `r0` | `ret(0x64eec)` |

## `EnvPowerDownStart`

- `EnvPowerDownStart`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ba1c to 0x6ba34.
  Established. The only call targets `0x69e48` with candidates `0x3`, entry r1, entry r2 and entry
  r3 (`bl` at 0x6ba2c); its result is the reported r0 value at the return (`ldmdb` at 0x6ba30). The
  meaning of selector `0x3` is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x6ba30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ba30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ba30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ba2c | `(unknown callee) 0x69e48` | `0x3` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ba30 | `r0` | `ret(0x69e48)` |

## `EnvPowerDownDone`

- `EnvPowerDownDone`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ba34 to 0x6ba50.
  Established. This wrapper first calls `0x69e48` with candidates `0x4`, entry r1, entry r2 and
  entry r3 (`bl` at 0x6ba44). It then calls `0x6ebac` with first candidate `ret(0x69e48)` and the
  remaining candidates unknown (`bl` at 0x6ba48). The final return reports `ret(0x6ebac)` in r0
  (`ldmdb` at 0x6ba4c). The sequence’s device effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x6ba4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ba4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ba4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ba44 | `(unknown callee) 0x69e48` | `0x4` | `r1` | `r2` | `r3` |
| `bl` at 0x6ba48 | `(unknown callee) 0x6ebac` | `ret(0x69e48)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ba4c | `r0` | `ret(0x6ebac)` |

## `EnvIgnoreLowPower`

- `EnvIgnoreLowPower`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bab8 to 0x6baec.
  Established. Entry r0 is saved at `sp -0x14` (`str` at 0x6bac8). A word store addresses absolute
  `0x11a770`, but the stored value is unknown after the conditional moves (`moveq` at 0x6bad8,
  `movne` at 0x6badc, `str` at 0x6bae0). The return retains entry r0 (`ldmdb` at 0x6bae4). No calls
  are reported, and the absolute word’s purpose remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6bac8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6bacc | load | 4 | `abs 0x6bae8` | `-` |
| `ldr` at 0x6bad0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6bae0 | store | 4 | `abs 0x11a770` | `?` |
| `ldmdb` at 0x6bae4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6bae4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6bae4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6bae4 | `r0` | `r0` |

## `nl4r_init_ports`

- `nl4r_init_ports`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x33ebc to 0x342c4.
  Established. The repeated call sequence includes `0xcb020` with candidates `[sp-20]`,
  `[sp-24]+10`, `0x2`, `sp-32`, followed by `0xcb8f4` with `[sp-20]`, `0x2`, `[sp-24]+10`, `0x0`
  (`bl` at 0x33ff8, `bl` at 0x34044). A signed-nonnegative result from the latter takes the edge to
  continued processing, while a negative result falls through to a word store at `sp -0x2c` and a
  branch to the return (`bge` at 0x34058, `str` at 0x34060, `b` at 0x34064). Later calls to
  `0xcb8f4` use third candidates `0x5` and `0x9`, respectively (`bl` at 0x34220, `bl` at 0x3426c).
  The path after the last nonnegative call result stores `0x0` at `sp -0x2c` (`bge` at 0x34280,
  `str` at 0x34294). The merged return value is unknown (`ldmdb` at 0x3429c). The callees’ effects
  are unknown within this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x33ecc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x33ed0 | load | 4 | `abs 0x342a0` | `-` |
| `str` at 0x33ed4 | store | 4 | `sp -0x28` | `0xde8a0` |
| `ldr` at 0x33ed8 | load | 4 | `abs 0x342a4` | `-` |
| `ldr` at 0x33edc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x33ee8 | load | 4 | `abs 0x342a8` | `-` |
| `ldr` at 0x33eec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x33ef8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x33f00 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x33f08 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x33f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33f1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33f38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33f40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x33f48 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x33f50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x33f54 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x33f58 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x33f5c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x33f60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x33f68 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x33f70 | load | 4 | `abs 0x342ac` | `-` |
| `ldr` at 0x33f74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x33f80 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x33f84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x33f98 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x33f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33fc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x33fd8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x33fdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x33fe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33ffc | load | 4 | `abs 0x342a4` | `-` |
| `ldr` at 0x34000 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x3400c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x34010 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34018 | load | 1 | `([sp-40] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x3401c | load | 4 | `abs 0x342b0` | `-` |
| `ldr` at 0x34024 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3402c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34034 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3404c | store | 4 | `sp -0x24` | `ret(0xcb8f4)` |
| `ldr` at 0x34050 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3405c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34060 | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x34068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34088 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34098 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x340a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x340a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x340ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x340b4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x340c0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x340c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x340d4 | load | 4 | `abs 0x342a4` | `-` |
| `ldr` at 0x340d8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x340e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x340e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x340f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x340f4 | load | 4 | `abs 0x342b4` | `-` |
| `ldr` at 0x34100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34108 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34120 | store | 4 | `sp -0x24` | `ret(0xcb8f4)` |
| `ldr` at 0x34124 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34130 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34134 | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x3413c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x34144 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x34150 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x34154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34164 | load | 4 | `abs 0x342a4` | `-` |
| `ldr` at 0x34168 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x34174 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x34178 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34180 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34184 | load | 4 | `abs 0x342b8` | `-` |
| `ldr` at 0x34190 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x341b0 | store | 4 | `sp -0x24` | `ret(0xcb8f4)` |
| `ldr` at 0x341b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x341c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x341c4 | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x341cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x341d4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x341dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x341e0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x341e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x341f8 | load | 4 | `abs 0x342a4` | `-` |
| `ldr` at 0x341fc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x34208 | load | 4 | `abs 0x342bc` | `-` |
| `ldr` at 0x34210 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34228 | store | 4 | `sp -0x24` | `ret(0xcb8f4)` |
| `ldr` at 0x3422c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34238 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3423c | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x34244 | load | 4 | `abs 0x342a4` | `-` |
| `ldr` at 0x34248 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x34254 | load | 4 | `abs 0x342c0` | `-` |
| `ldr` at 0x3425c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34274 | store | 4 | `sp -0x24` | `ret(0xcb8f4)` |
| `ldr` at 0x34278 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34284 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34288 | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `str` at 0x34294 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x34298 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3429c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3429c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3429c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x33ef0 | `(unknown callee) 0x20f14` | `0xde8ac` | `r0` | `r2` | `[0x27eb98]` |
| `bl` at 0x33f24 | `(unknown callee) 0x349b0` | `?` | `?` | `?` | `?` |
| `bl` at 0x33f78 | `(unknown callee) 0x1dba4` | `0xde8d0` | `?` | `?` | `?` |
| `bl` at 0x33fb8 | `(unknown callee) 0xcb020` | `[sp-20]` | `?` | `0x1` | `sp-32` |
| `bl` at 0x33fc4 | `(unknown callee) 0x349b0` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x33ff8 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x34028 | `(unknown callee) 0x20f14` | `0xde8dc` | `[([sp-40] + [sp-24])]:1` | `[sp-32]` | `[([sp-40] + [sp-24])]:1` |
| `bl` at 0x34044 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x2` | `[sp-24]+10` | `0x0` |
| `bl` at 0x34084 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]+10` | `0x0` | `sp-32` |
| `bl` at 0x340fc | `(unknown callee) 0x20f14` | `0xde908` | `?` | `[sp-40]` | `?` |
| `bl` at 0x34118 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x3418c | `(unknown callee) 0x20f14` | `0xde92c` | `?` | `[sp-40]` | `?` |
| `bl` at 0x341a8 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x341f4 | `(unknown callee) 0xcb020` | `[sp-20]` | `0x5` | `0x0` | `sp-32` |
| `bl` at 0x3420c | `(unknown callee) 0x20f14` | `0xde950` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0x34220 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x2` | `0x5` | `0x0` |
| `bl` at 0x34258 | `(unknown callee) 0x20f14` | `0xde970` | `?` | `?` | `[0x27eb98]` |
| `bl` at 0x3426c | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x2` | `0x9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x33ee4 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bls` at 0x33f14 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x33f30 | `ret(0x349b0) == 0x0` | `ret(0x349b0) != 0x0` |
| `bls` at 0x33f8c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x34008 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x34058 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bne` at 0x34090 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bls` at 0x340cc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x340e0 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3412c | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x3415c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x34170 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x341bc | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x34204 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x34234 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x34250 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x34280 | `ret(0xcb8f4) >= 0x0 (signed, 32-bit)` | `ret(0xcb8f4) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3429c | `r0` | `?` |

## `portName`

- `portName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f6e4 to 0x3f754.
  Established. The call to `0x40650` has candidates `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0x3f704). A zero result takes the fallback edge, and a zero word at `ret(0x40650)
  +0x8` takes that edge on the nonzero-result path (`beq` at 0x3f718, `ldr` at 0x3f720, `beq` at
  0x3f728). The fallback stores `0xe1f54` at `sp -0x1c`, whereas the other arm stores
  `[ret(0x40650)+8]` there (`str` at 0x3f740, `str` at 0x3f734). The return value is unknown after
  the paths merge (`ldmdb` at 0x3f74c). Neither the callee’s effect nor the returned pointee type is
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3f6f8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3f6fc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3f70c | store | 4 | `sp -0x18` | `ret(0x40650)` |
| `ldr` at 0x3f710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f71c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f720 | load | 4 | `ret(0x40650) +0x8` | `-` |
| `ldr` at 0x3f72c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f730 | load | 4 | `ret(0x40650) +0x8` | `-` |
| `str` at 0x3f734 | store | 4 | `sp -0x1c` | `[ret(0x40650)+8]` |
| `ldr` at 0x3f73c | load | 4 | `abs 0x3f750` | `-` |
| `str` at 0x3f740 | store | 4 | `sp -0x1c` | `0xe1f54` |
| `ldr` at 0x3f744 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3f74c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3f74c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3f74c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f704 | `(unknown callee) 0x40650` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3f718 | `ret(0x40650) == 0x0` | `ret(0x40650) != 0x0` |
| `beq` at 0x3f728 | `[ret(0x40650)+8] == 0x0` | `[ret(0x40650)+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3f74c | `r0` | `?` |
