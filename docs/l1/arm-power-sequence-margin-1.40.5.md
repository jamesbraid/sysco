# ARM power sequence and margin callers in 1.40.5

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
The conditional computed transfers remain unresolved (`ldrls` at 0x3d0c0, 0x3d3b8,
0x3d8f0, 0x3d93c, 0x3e434, 0x4054c, 0x3ef98). Known-edge tables exclude effects reached
only through unknown destinations. Raw undecodable bytes remain in the fact-sheet footer.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/power-sequence-margin-facts.txt) retain all tool sections.

## `powerUp`

- `powerUp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d008 to 0x3d688.
  Established. Four-byte reads at absolute `0x2af384` precede an unknown-target call with candidates
  `ret(0x52014)`, unknown, unknown, `[0x2af384]` (`ldr` at 0x3d048, `ldr` at 0x3d058, `mov` at
  0x3d060). The branch taken when `[0x2af384] == 0x0` skips that call (`beq` at 0x3d050). Byte
  assignments include `0x1` at `[sp-72] +0xe` and `+0xd`, and later `0x0` at `+0xe` (`strb` at
  0x3d1d8, `strb` at 0x3d39c, `strb` at 0x3d604). The two conditional computed transfers have
  unresolved destinations (`ldrls` at 0x3d0c0, `ldrls` at 0x3d3b8). Power-up effects and
  selector-only paths remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d018 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3d01c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3d024 | store | 4 | `sp -0x20` | `0x0` |
| `strb` at 0x3d030 | store | 1 | `sp -0x35` | `(ret(0x4f4d0) & 0xff)` |
| `ldr` at 0x3d034 | load | 4 | `abs 0x3d648` | `-` |
| `ldr` at 0x3d038 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3d03c | store | 4 | `sp -0x48` | `[0x148d00]` |
| `ldr` at 0x3d044 | load | 4 | `abs 0x3d64c` | `-` |
| `ldr` at 0x3d048 | load | 4 | `abs 0x2af384` | `-` |
| `ldr` at 0x3d054 | load | 4 | `abs 0x3d64c` | `-` |
| `ldr` at 0x3d058 | load | 4 | `abs 0x2af384` | `-` |
| `ldr` at 0x3d070 | load | 4 | `abs 0x3d650` | `-` |
| `str` at 0x3d074 | store | 4 | `sp -0x64` | `0x108f18` |
| `ldr` at 0x3d07c | load | 4 | `abs 0x3d654` | `-` |
| `ldr` at 0x3d080 | load | 4 | `abs 0x3d658` | `-` |
| `str` at 0x3d090 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x3d0a0 | store | 4 | `sp -0x28` | `ret(0x50820)` |
| `ldr` at 0x3d0a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3d0b0 | load | 4 | `sp -0x28` | `-` |
| `ldrls` at 0x3d0c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d120 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x3d12c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3d130 | store | 4 | `sp -0x50` | `[sp-40]` |
| `ldr` at 0x3d138 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d140 | store | 4 | `sp -0x54` | `ret(0x4ea90)` |
| `ldr` at 0x3d144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d150 | load | 4 | `abs 0x3d670` | `-` |
| `str` at 0x3d154 | store | 4 | `sp -0x58` | `0x108ef8` |
| `ldr` at 0x3d15c | load | 4 | `abs 0x3d674` | `-` |
| `str` at 0x3d160 | store | 4 | `sp -0x58` | `0x108efc` |
| `ldr` at 0x3d164 | load | 4 | `abs 0x3d678` | `-` |
| `ldr` at 0x3d168 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x3d16c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x3d180 | store | 4 | `sp -0x24` | `ret(0x52048)` |
| `str` at 0x3d194 | store | 4 | `sp -0x24` | `ret(0x52f84)` |
| `ldr` at 0x3d19c | load | 4 | `abs 0x3d67c` | `-` |
| `str` at 0x3d1ac | store | 4 | `sp -0x24` | `ret(0x51b28)` |
| `ldr` at 0x3d1b0 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3d1b8 | store | 1 | `[sp-72] +0xc` | `0x0` |
| `str` at 0x3d1c8 | store | 4 | `sp -0x20` | `ret(0x4e5ec)` |
| `ldr` at 0x3d1d0 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3d1d8 | store | 1 | `[sp-72] +0xe` | `0x1` |
| `str` at 0x3d1e0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3d1e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d1e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3d1ec | load | 4 | `[sp-72] +0x8` | `-` |
| `ldr` at 0x3d1fc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d200 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3d214 | load | 4 | `[sp-72] +0x0` | `-` |
| `str` at 0x3d21c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x3d220 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d224 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x3d228 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3d230 | load | 1 | `([[sp-72]+4] + [sp-44]) +0x0` | `-` |
| `ldr` at 0x3d23c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3d240 | load | 1 | `[sp-76] +0x2` | `-` |
| `ldr` at 0x3d250 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3d25c | store | 4 | `sp -0x24` | `ret(0x40af8)` |
| `ldr` at 0x3d260 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d278 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3d280 | store | 1 | `[sp-72] +0xe` | `0x0` |
| `ldr` at 0x3d284 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d288 | store | 4 | `sp -0x50` | `[sp-36]` |
| `ldr` at 0x3d290 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d294 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3d2a8 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrh` at 0x3d2b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3d2bc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d2c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3d2d4 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrh` at 0x3d2dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3d2f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3d2fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3d30c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d310 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3d31c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3d320 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x3d334 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x3d338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d344 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d350 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d354 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `str` at 0x3d36c | store | 4 | `sp -0x24` | `ret(0x42284)` |
| `ldr` at 0x3d370 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d37c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d388 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d38c | store | 4 | `sp -0x20` | `ret(0x42284)` |
| `ldr` at 0x3d394 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3d39c | store | 1 | `[sp-72] +0xd` | `0x1` |
| `ldr` at 0x3d3a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3d3ac | load | 1 | `sp -0x35` | `-` |
| `ldrls` at 0x3d3b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3d508 | store | 4 | `sp -0x24` | `ret(0x4e54c)` |
| `ldr` at 0x3d50c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d518 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d524 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d528 | store | 4 | `sp -0x20` | `ret(0x4e54c)` |
| `ldr` at 0x3d52c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d544 | load | 4 | `abs 0x3d680` | `-` |
| `str` at 0x3d554 | store | 4 | `sp -0x24` | `ret(0x51b28)` |
| `ldr` at 0x3d560 | load | 4 | `abs 0x3d684` | `-` |
| `str` at 0x3d570 | store | 4 | `sp -0x24` | `ret(0x51b28)` |
| `ldrb` at 0x3d58c | load | 1 | `sp -0x35` | `-` |
| `str` at 0x3d590 | store | 4 | `sp -0x5c` | `[sp-53]:1` |
| `ldr` at 0x3d594 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x3d5a0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x3d5bc | store | 4 | `sp -0x24` | `ret(0x4e54c)` |
| `ldr` at 0x3d5c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d5d8 | store | 4 | `sp -0x24` | `ret(0x4e5ec)` |
| `ldr` at 0x3d5dc | load | 4 | `abs 0x3d64c` | `-` |
| `ldr` at 0x3d5e0 | load | 4 | `abs 0x2af304` | `-` |
| `ldr` at 0x3d5ec | load | 4 | `abs 0x3d64c` | `-` |
| `ldr` at 0x3d5f0 | load | 4 | `abs 0x2af304` | `-` |
| `ldr` at 0x3d5fc | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3d604 | store | 1 | `[sp-72] +0xe` | `0x0` |
| `ldr` at 0x3d608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d61c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d620 | store | 4 | `sp -0x60` | `[sp-32]` |
| `ldr` at 0x3d624 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d630 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3d634 | store | 4 | `sp -0x60` | `[sp-40]` |
| `ldr` at 0x3d638 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x3d63c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3d640 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0x3d644 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d644 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d644 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d028 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3d040 | `(unknown callee) 0x52014` | `ret(0x4f4d0)` | `?` | `?` | `[0x148d00]` |
| `mov` at 0x3d060 | `(unknown callee) (unknown target)` | `ret(0x52014)` | `?` | `?` | `[0x2af384]` |
| `bl` at 0x3d088 | `(unknown callee) 0x21eb8` | `0x6` | `0x108df8` | `0x2d5` | `0x0` |
| `bl` at 0x3d098 | `(unknown callee) 0x50820` | `?` | `?` | `?` | `?` |
| `bl` at 0x3d118 | `(unknown callee) 0x52048` | `0x2` | `?` | `?` | `ret(0x50820)+1704` |
| `bl` at 0x3d128 | `(unknown callee) 0x51b28` | `0x32` | `[sp-76]` | `0x2` | `?` |
| `bl` at 0x3d13c | `(unknown callee) 0x4ea90` | `[sp-24]` | `?` | `?` | `ret(0x50820)` |
| `bl` at 0x3d170 | `(unknown callee) 0x2079c` | `0x108fd0` | `[sp-88]` | `ret(0x4ea90)` | `?` |
| `bl` at 0x3d178 | `(unknown callee) 0x52048` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d18c | `(unknown callee) 0x52f84` | `0x1` | `0x0` | `?` | `ret(0x52048)` |
| `bl` at 0x3d1a4 | `(unknown callee) 0x51b28` | `0x32` | `0x108fe0` | `0x1` | `ret(0x52f84)` |
| `bl` at 0x3d1c0 | `(unknown callee) 0x4e5ec` | `0x5` | `?` | `[sp-72]` | `0x0` |
| `bl` at 0x3d1cc | `(unknown callee) 0x82c60` | `ret(0x4e5ec)` | `?` | `?` | `ret(0x4e5ec)` |
| `bl` at 0x3d254 | `(unknown callee) 0x40af8` | `0x0` | `[sp-76]` | `[[sp-72]+4]` | `[[sp-76]+2]:1` |
| `bl` at 0x3d270 | `(unknown callee) 0x3d890` | `0x5` | `?` | `?` | `ret(0x40af8)` |
| `bl` at 0x3d274 | `(unknown callee) 0x82c78` | `ret(0x3d890)` | `?` | `?` | `?` |
| `bl` at 0x3d2e4 | `(unknown callee) 0x412fc` | `?` | `[sp-72]` | `?` | `?` |
| `bl` at 0x3d2f0 | `(unknown callee) 0x412fc` | `0x19` | `[sp-72]` | `?` | `?` |
| `bl` at 0x3d308 | `(unknown callee) 0x412fc` | `0x64` | `?` | `?` | `[[sp-72]+8]` |
| `mov` at 0x3d32c | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[[sp-72]+16]` |
| `bl` at 0x3d364 | `(unknown callee) 0x42284` | `0x0` | `0xff` | `0x3` | `?` |
| `bl` at 0x3d390 | `(unknown callee) 0x82c78` | `ret(0x42284)` | `?` | `?` | `?` |
| `bl` at 0x3d500 | `(unknown callee) 0x4e54c` | `0x5` | `?` | `[sp-72]` | `[sp-53]:1-48` |
| `bl` at 0x3d53c | `(unknown callee) 0x757f0` | `0x1` | `?` | `?` | `[sp-32]` |
| `bl` at 0x3d54c | `(unknown callee) 0x51b28` | `0x32` | `0x108fec` | `0x0` | `?` |
| `bl` at 0x3d568 | `(unknown callee) 0x51b28` | `0x32` | `0x108ff8` | `0x2` | `[sp-32]` |
| `bl` at 0x3d578 | `(unknown callee) 0x5d810` | `0x0` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x3d580 | `(unknown callee) 0x5d994` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d588 | `(unknown callee) 0x5d034` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d5b4 | `(unknown callee) 0x4e54c` | `0x5` | `?` | `?` | `[sp-53]:1` |
| `bl` at 0x3d5d0 | `(unknown callee) 0x4e5ec` | `0x5` | `?` | `?` | `[sp-20]` |
| `mov` at 0x3d5f8 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x2af304]` |
| `bl` at 0x3d618 | `(unknown callee) 0x82d60` | `0x0` | `?` | `[sp-72]` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d050 | `[0x2af384] == 0x0` | `[0x2af384] != 0x0` |
| `beq` at 0x3d06c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x3d0ac | `ret(0x50820) == 0x0` | `ret(0x50820) != 0x0` |
| `ldrls` at 0x3d0c0 | `ret(0x50820)+1704 <= 0x4 (unsigned, 32-bit)` | `ret(0x50820)+1704 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x3d14c | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `blt` at 0x3d1f4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3d238 | `[([[sp-72]+4] + [sp-44])]:1 == 0x0` | `[([[sp-72]+4] + [sp-44])]:1 != 0x0` |
| `beq` at 0x3d248 | `[[sp-76]+2]:1 == 0xff` | `[[sp-76]+2]:1 != 0xff` |
| `beq` at 0x3d268 | `ret(0x40af8) == 0x0` | `ret(0x40af8) != 0x0` |
| `beq` at 0x3d2b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3d318 | `[[sp-72]+16] == 0x0` | `[[sp-72]+16] != 0x0` |
| `beq` at 0x3d340 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3d34c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3d378 | `ret(0x42284) == 0x0` | `ret(0x42284) != 0x0` |
| `bne` at 0x3d384 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3d3a8 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `ldrls` at 0x3d3b8 | `[sp-53]:1-48 <= 0x4c (unsigned, 32-bit)` | `[sp-53]:1-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0x3d514 | `ret(0x4e54c) == 0x0` | `ret(0x4e54c) != 0x0` |
| `bne` at 0x3d520 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x3d534 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3d59c | `[sp-53]:1 == 0x23` | `[sp-53]:1 != 0x23` |
| `beq` at 0x3d5a8 | `[sp-53]:1 == 0x3d` | `[sp-53]:1 != 0x3d` |
| `bne` at 0x3d5c8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x3d5e8 | `[0x2af304] == 0x0` | `[0x2af304] != 0x0` |
| `beq` at 0x3d610 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `bne` at 0x3d62c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d644 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d0c0).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d3b8).

## `powerDown`

- `powerDown`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d890 to 0x3dc5c.
  Established. The conditional computed transfers are taken when `[sp-20]-1 <= 0x4 (unsigned,
  32-bit)`, with unresolved destinations (`ldrls` at 0x3d8f0, `ldrls` at 0x3d93c). Known-edge calls
  include unnamed `0x4257c` with `0x0`, `0xff`, `0x2`, unknown (`bl` at 0x3da1c). The branch taken
  when `[[sp-120]+20] == 0x0` skips an unknown-target call whose candidates are `0x0`, unknown,
  unknown, `[[sp-120]+20]` (`beq` at 0x3da54, `mov` at 0x3da70). Shutdown semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d8a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3d8a8 | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x3d8b4 | store | 1 | `sp -0x71` | `(ret(0x4f4d0) & 0xff)` |
| `ldr` at 0x3d8b8 | load | 4 | `abs 0x3dc3c` | `-` |
| `ldr` at 0x3d8bc | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3d8c0 | store | 4 | `sp -0x78` | `[0x148d00]` |
| `ldrb` at 0x3d8d4 | load | 1 | `sp -0x71` | `-` |
| `ldr` at 0x3d8e4 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x3d8f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d930 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x3d93c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d9a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d9b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d9c0 | load | 4 | `abs 0x3dc48` | `-` |
| `str` at 0x3d9d8 | store | 4 | `sp -0x1c` | `ret(0x52154)` |
| `ldr` at 0x3d9e0 | load | 4 | `abs 0x3dc4c` | `-` |
| `str` at 0x3d9f0 | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `str` at 0x3da04 | store | 4 | `sp -0x18` | `ret(0x4e5ec)` |
| `str` at 0x3da24 | store | 4 | `sp -0x1c` | `ret(0x4257c)` |
| `ldr` at 0x3da28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3da34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3da40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3da44 | store | 4 | `sp -0x18` | `ret(0x4257c)` |
| `ldr` at 0x3da48 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3da4c | load | 4 | `[sp-120] +0x14` | `-` |
| `str` at 0x3da5c | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x3da60 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3da64 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x3da80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3da90 | store | 4 | `sp -0x84` | `0x1` |
| `ldr` at 0x3da94 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x3da98 | store | 4 | `sp -0x1c` | `[sp-132]` |
| `ldr` at 0x3daa4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3daa8 | store | 4 | `sp -0x18` | `[sp-132]` |
| `ldr` at 0x3daac | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3dab0 | load | 4 | `[sp-120] +0x8` | `-` |
| `str` at 0x3dab8 | store | 4 | `sp -0x20` | `[[sp-120]+8]-1` |
| `ldr` at 0x3dabc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dacc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3dad0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dae4 | load | 4 | `[sp-120] +0x0` | `-` |
| `str` at 0x3daec | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x3daf0 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3daf4 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x3db00 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3db04 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3db08 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3db10 | load | 1 | `([[sp-120]+4] + [sp-32]) +0x0` | `-` |
| `ldr` at 0x3db1c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3db20 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3db24 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3db2c | load | 1 | `([[sp-120]+4] + [sp-32]) +0x0` | `-` |
| `ldr` at 0x3db38 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3db3c | load | 1 | `[sp-124] +0x2` | `-` |
| `str` at 0x3db60 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x3db68 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3db7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3db8c | store | 4 | `sp -0x88` | `0x1` |
| `ldr` at 0x3db90 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x3db94 | store | 4 | `sp -0x1c` | `[sp-136]` |
| `ldr` at 0x3dba0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3dba4 | store | 4 | `sp -0x18` | `[sp-136]` |
| `ldr` at 0x3dba8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3dbb0 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0x3dbb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3dbc8 | load | 4 | `abs 0x3dc50` | `-` |
| `str` at 0x3dbd8 | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x3dbe4 | load | 4 | `abs 0x3dc54` | `-` |
| `str` at 0x3dbf4 | store | 4 | `sp -0x1c` | `ret(0x51b28)` |
| `ldr` at 0x3dc04 | load | 4 | `abs 0x3dc58` | `-` |
| `ldr` at 0x3dc08 | load | 4 | `abs 0x2af304` | `-` |
| `ldr` at 0x3dc1c | load | 4 | `abs 0x3dc58` | `-` |
| `ldr` at 0x3dc20 | load | 4 | `abs 0x2af304` | `-` |
| `ldr` at 0x3dc2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dc30 | store | 4 | `sp -0x80` | `[sp-24]` |
| `ldr` at 0x3dc34 | load | 4 | `sp -0x80` | `-` |
| `ldmdb` at 0x3dc38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dc38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dc38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d8ac | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3d8c4 | `(unknown callee) 0x8627c` | `ret(0x4f4d0)` | `?` | `?` | `[0x148d00]` |
| `bl` at 0x3d9b8 | `(unknown callee) 0x4ea90` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x3d9c8 | `(unknown callee) 0x2079c` | `0x10909c` | `ret(0x4ea90)` | `?` | `ret(0x4ea90)` |
| `bl` at 0x3d9d0 | `(unknown callee) 0x52154` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d9e8 | `(unknown callee) 0x51b28` | `0x32` | `0x1090ac` | `0x1` | `ret(0x52154)` |
| `bl` at 0x3d9f4 | `(unknown callee) 0x82cc4` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x3d9fc | `(unknown callee) 0x4e5ec` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x3da0c | `(unknown callee) 0x5d1e8` | `0x0` | `?` | `?` | `ret(0x4e5ec)` |
| `bl` at 0x3da1c | `(unknown callee) 0x4257c` | `0x0` | `0xff` | `0x2` | `?` |
| `mov` at 0x3da70 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[[sp-120]+20]` |
| `bl` at 0x3db48 | `(unknown callee) 0xd72bc` | `0x0` | `[[sp-124]+2]:1` | `[[sp-120]+4]` | `[[sp-124]+2]:1` |
| `bl` at 0x3db6c | `(unknown callee) 0x40cac` | `0x0` | `[sp-124]` | `?` | `0x0` |
| `bl` at 0x3dbd0 | `(unknown callee) 0x51b28` | `0x32` | `0x1090b8` | `0x0` | `[sp-24]` |
| `bl` at 0x3dbec | `(unknown callee) 0x51b28` | `0x32` | `0x1090c8` | `0x2` | `[sp-24]` |
| `bl` at 0x3dbf8 | `(unknown callee) 0x82cdc` | `ret(0x51b28)` | `?` | `?` | `ret(0x51b28)` |
| `bl` at 0x3dc00 | `(unknown callee) 0x757f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3dc18 | `(unknown callee) 0x412fc` | `0x28` | `?` | `?` | `[0x2af304]` |
| `mov` at 0x3dc28 | `(unknown callee) (unknown target)` | `ret(0x412fc)` | `?` | `?` | `[0x2af304]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d8d0 | `ret(0x8627c) == 0x0` | `ret(0x8627c) != 0x0` |
| `beq` at 0x3d8dc | `[sp-113]:1 == 0x4c` | `[sp-113]:1 != 0x4c` |
| `ldrls` at 0x3d8f0 | `[sp-20]-1 <= 0x4 (unsigned, 32-bit)` | `[sp-20]-1 > 0x4 (unsigned, 32-bit)` |
| `ldrls` at 0x3d93c | `[sp-20]-1 <= 0x4 (unsigned, 32-bit)` | `[sp-20]-1 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x3d9b0 | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `beq` at 0x3da30 | `ret(0x4257c) == 0x0` | `ret(0x4257c) != 0x0` |
| `bne` at 0x3da3c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3da54 | `[[sp-120]+20] == 0x0` | `[[sp-120]+20] != 0x0` |
| `beq` at 0x3da7c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3da88 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3daa0 | `[sp-132] == 0x0` | `[sp-132] != 0x0` |
| `bge` at 0x3dac4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3dafc | `[[sp-124]+2]:1 == 0xff` | `[[sp-124]+2]:1 != 0xff` |
| `bne` at 0x3db18 | `[([[sp-120]+4] + [sp-32])]:1 != 0x0` | `[([[sp-120]+4] + [sp-32])]:1 == 0x0` |
| `bne` at 0x3db34 | `[([[sp-120]+4] + [sp-32])]:1 != 0x0` | `[([[sp-120]+4] + [sp-32])]:1 == 0x0` |
| `bne` at 0x3db54 | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |
| `beq` at 0x3db78 | `ret(0x40cac) == 0x0` | `ret(0x40cac) != 0x0` |
| `bne` at 0x3db84 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3db9c | `[sp-136] == 0x0` | `[sp-136] != 0x0` |
| `bne` at 0x3dbc0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3dc10 | `[0x2af304] == 0x0` | `[0x2af304] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dc38 | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d8f0).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d93c).

## `powerMarginAll`

- `powerMarginAll`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e708 to 0x3e76c.
  Established. The unnamed call to `0x3e76c` carries `0x0`, `0x21`, `r0`, `0x0` (`bl` at 0x3e72c);
  its nonzero result takes the branch past a four-byte store at absolute `0x14349c` (`bne` at
  0x3e740). The zero-result fall-through writes `[sp-20]` there and calls unnamed `0x75848` with
  `[sp-20]`, unknown, `0x14349c`, `[sp-20]` (`str` at 0x3e74c, `bl` at 0x3e754). The global value
  role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e718 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3e724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e728 | load | 4 | `abs 0x3e764` | `-` |
| `str` at 0x3e734 | store | 4 | `sp -0x18` | `ret(0x3e76c)` |
| `ldr` at 0x3e738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e744 | load | 4 | `abs 0x3e768` | `-` |
| `ldr` at 0x3e748 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3e74c | store | 4 | `abs 0x14349c` | `[sp-20]` |
| `ldr` at 0x3e758 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3e760 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e760 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e72c | `(unknown callee) 0x3e76c` | `0x0` | `0x21` | `r0` | `0x0` |
| `bl` at 0x3e754 | `(unknown callee) 0x75848` | `[sp-20]` | `?` | `0x14349c` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3e740 | `ret(0x3e76c) != 0x0` | `ret(0x3e76c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e760 | `r0` | `?` |

## `powerMarginStatusAll`

- `powerMarginStatusAll`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dcc4 to 0x3dce4.
  Established. A four-byte load at absolute `0x14349c` supplies the return expression `[0x14349c]`
  (`ldr` at 0x3dcd4, `ldmdb` at 0x3dcdc). The literal supplying that absolute address is loaded from
  `0x3dce0` (`ldr` at 0x3dcd0). A margin-status meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3dcd0 | load | 4 | `abs 0x3dce0` | `-` |
| `ldr` at 0x3dcd4 | load | 4 | `abs 0x14349c` | `-` |
| `ldmdb` at 0x3dcdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dcdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dcdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dcdc | `r0` | `[0x14349c]` |

## `powerSetMargin`

- `powerSetMargin`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e2b8 to 0x3e3c4.
  Established. The unnamed call to `0x40d80` has candidates `[sp-20]`, `ret(0x409a4)`, `[sp-25]:1`,
  `[sp-25]:1` (`bl` at 0x3e340); its zero result takes the branch to `0x3e378` (`beq` at 0x3e354).
  The two arms write four-byte `0x4` or `0x5` at absolute `0x14349c` (`str` at 0x3e380, `str` at
  0x3e360). Later calls include unnamed `0x412c4` and `0x41254` (`bl` at 0x3e398, `bl` at 0x3e3ac).
  Margin-update and backing-storage semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e2c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3e2cc | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x3e2d4 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x3e2d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e2dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3e2f4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3e2fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e300 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3e30c | store | 4 | `sp -0x20` | `ret(0x409a4)` |
| `ldr` at 0x3e310 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e31c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3e328 | store | 4 | `sp -0x28` | `ret(0x40a40)` |
| `ldrb` at 0x3e330 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3e334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e338 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e348 | store | 4 | `sp -0x24` | `ret(0x40d80)` |
| `ldr` at 0x3e34c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3e358 | load | 4 | `abs 0x3e3c0` | `-` |
| `str` at 0x3e360 | store | 4 | `abs 0x14349c` | `0x5` |
| `ldr` at 0x3e36c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3e370 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x3e378 | load | 4 | `abs 0x3e3c0` | `-` |
| `str` at 0x3e380 | store | 4 | `abs 0x14349c` | `0x4` |
| `ldr` at 0x3e38c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e390 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3e39c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3e3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e3a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e3b4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3e3b8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x3e3bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e3bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e3bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e2e0 | `(unknown callee) 0x3cc70` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x3e304 | `(unknown callee) 0x409a4` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x3cc70)` |
| `bl` at 0x3e320 | `(unknown callee) 0x40a40` | `[sp-20]` | `?` | `?` | `ret(0x409a4)` |
| `bl` at 0x3e340 | `(unknown callee) 0x40d80` | `[sp-20]` | `ret(0x409a4)` | `[sp-25]:1` | `[sp-25]:1` |
| `bl` at 0x3e368 | `(unknown callee) 0x75848` | `0x5` | `?` | `0x5` | `0x14349c` |
| `bl` at 0x3e388 | `(unknown callee) 0x75848` | `0x4` | `?` | `0x4` | `0x14349c` |
| `bl` at 0x3e398 | `(unknown callee) 0x412c4` | `[sp-20]` | `[sp-32]` | `0x4` | `?` |
| `bl` at 0x3e3ac | `(unknown callee) 0x41254` | `[sp-20]` | `[sp-32]` | `[sp-25]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3e2ec | `ret(0x3cc70) != 0x0` | `ret(0x3cc70) == 0x0` |
| `bne` at 0x3e318 | `ret(0x409a4) != 0x0` | `ret(0x409a4) == 0x0` |
| `beq` at 0x3e354 | `ret(0x40d80) == 0x0` | `ret(0x40d80) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e3bc | `r0` | `?` |

## `powerGetMargin`

- `powerGetMargin`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e3c4 to 0x3e708.
  Established. The unnamed call to `0x409a4` carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x3e3e8).
  A byte read at `ret(0x409a4) +0x6` precedes the branch taken when `[ret(0x409a4)+6]:1 == 0xff`
  (`ldrb` at 0x3e418, `beq` at 0x3e420). That arm writes four-byte `0xfffff9be` at `sp -0x34` (`str`
  at 0x3e6d8). The conditional computed transfer is taken for `[ret(0x409a4)+8]-1 <= 0x4 (unsigned,
  32-bit)` but has unknown destination (`ldrls` at 0x3e434). Selector-only effects and margin
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e3d4 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3e3d8 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x3e3dc | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x3e3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e3e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e3f0 | store | 4 | `sp -0x28` | `ret(0x409a4)` |
| `ldr` at 0x3e3f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e400 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e40c | store | 4 | `sp -0x34` | `ret(0x40a40)` |
| `ldr` at 0x3e414 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3e418 | load | 1 | `ret(0x409a4) +0x6` | `-` |
| `ldr` at 0x3e424 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e428 | load | 4 | `ret(0x409a4) +0x8` | `-` |
| `ldrls` at 0x3e434 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3e640 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e648 | load | 4 | `abs 0x3e6f0` | `-` |
| `ldr` at 0x3e64c | load | 4 | `abs 0x3e6f4` | `-` |
| `str` at 0x3e678 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3e67c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e684 | load | 4 | `abs 0x3e6f0` | `-` |
| `ldr` at 0x3e688 | load | 4 | `abs 0x3e6f4` | `-` |
| `ldr` at 0x3e694 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3e6a4 | load | 4 | `abs 0x3e6f8` | `-` |
| `str` at 0x3e6a8 | store | 4 | `(unknown)` | `0x1090dc` |
| `str` at 0x3e6ac | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x3e6b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e6b4 | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x3e6b8 | store | 4 | `(unknown)` | `[[sp-40]+8]` |
| `ldr` at 0x3e6c0 | load | 4 | `abs 0x3e6fc` | `-` |
| `ldr` at 0x3e6c4 | load | 4 | `abs 0x3e700` | `-` |
| `ldr` at 0x3e6d4 | load | 4 | `abs 0x3e704` | `-` |
| `str` at 0x3e6d8 | store | 4 | `sp -0x34` | `0xfffff9be` |
| `str` at 0x3e6e4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3e6e8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3e6ec | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3e6ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3e6ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e6ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e6ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e3e8 | `(unknown callee) 0x409a4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3e404 | `(unknown callee) 0x40a40` | `[sp-28]` | `?` | `?` | `ret(0x409a4)` |
| `bl` at 0x3e650 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3e65c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3e68c | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3e69c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3e6cc | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x5cf` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3e3fc | `ret(0x409a4) != 0x0` | `ret(0x409a4) == 0x0` |
| `beq` at 0x3e420 | `[ret(0x409a4)+6]:1 == 0xff` | `[ret(0x409a4)+6]:1 != 0xff` |
| `ldrls` at 0x3e434 | `[ret(0x409a4)+8]-1 <= 0x4 (unsigned, 32-bit)` | `[ret(0x409a4)+8]-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e6ec | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3e434).

## `powerMargin`

- `powerMargin`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e76c to 0x3ee14.
  Established. A four-byte store writes `0x5` at absolute `0x14349c` (`str` at 0x3e90c). The unnamed
  call to `0x3ee14` has candidates `[sp-68]`, `[sp-20]`, `[sp-40]`, `[sp-28]` (`bl` at 0x3e9f0); its
  zero result takes the branch to `0x3ea68` (`beq` at 0x3ea04). Later calls include unnamed
  `0x3f630` with unknown, `sp-64`, unknown, `sp-64` (`bl` at 0x3ebf0). Another four-byte assignment
  at absolute `0x14349c` retains `[sp-28]` (`str` at 0x3edb4). Record roles and margin-control
  effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e77c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3e780 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3e784 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3e788 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x3e790 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x3e794 | load | 4 | `abs 0x3edd0` | `-` |
| `ldr` at 0x3e798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e7ac | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3e7b4 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x3e7bc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x3e7c4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3e7c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e7cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e7d0 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x3e7e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e7e4 | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x3e7e8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3e7f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e7fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e814 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrb` at 0x3e81c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e838 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e848 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3e850 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x3e854 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e85c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3e864 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3e874 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x3e87c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3e8a4 | store | 4 | `sp -0x34` | `ret(0x1b1b4)` |
| `ldr` at 0x3e8a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e8b4 | load | 4 | `abs 0x3edd4` | `-` |
| `str` at 0x3e8b8 | store | 4 | `sp -0x64` | `0x1090f8` |
| `ldr` at 0x3e8bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3e8c0 | store | 4 | `sp -0x60` | `[sp-20]` |
| `ldr` at 0x3e8c8 | load | 4 | `abs 0x3edd8` | `-` |
| `ldr` at 0x3e8cc | load | 4 | `abs 0x3eddc` | `-` |
| `ldr` at 0x3e8d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3e8f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e904 | load | 4 | `abs 0x3ede0` | `-` |
| `str` at 0x3e90c | store | 4 | `abs 0x14349c` | `0x5` |
| `str` at 0x3e91c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x3e924 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3e928 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e92c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e930 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x3e940 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e944 | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x3e948 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3e950 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e95c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e974 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrb` at 0x3e97c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e998 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e9a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e9c4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3e9cc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x3e9d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e9d4 | store | 4 | `sp -0x64` | `[sp-32]` |
| `str` at 0x3e9dc | store | 4 | `sp -0x60` | `sp-64` |
| `ldr` at 0x3e9e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e9e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e9e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e9ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e9f8 | store | 4 | `sp -0x24` | `ret(0x3ee14)` |
| `ldr` at 0x3e9fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ea08 | load | 4 | `abs 0x3ede4` | `-` |
| `ldr` at 0x3ea0c | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3ea18 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3ea1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ea30 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x3ea38 | load | 4 | `abs 0x3ede8` | `-` |
| `ldr` at 0x3ea3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3ea44 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3ea4c | store | 4 | `[sp-68] +0xc` | `0x0` |
| `ldr` at 0x3ea50 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ea5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ea60 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x3ea68 | load | 4 | `abs 0x3ede4` | `-` |
| `ldr` at 0x3ea6c | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3ea78 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ea7c | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3ea80 | load | 1 | `[sp-68] +0x25` | `-` |
| `ldr` at 0x3ea84 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3ea88 | load | 1 | `[sp-68] +0x24` | `-` |
| `ldr` at 0x3ea8c | load | 4 | `abs 0x3edec` | `-` |
| `ldr` at 0x3ea90 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x3ea98 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ea9c | load | 4 | `[sp-68] +0x20` | `-` |
| `ldr` at 0x3eaac | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3eab0 | load | 1 | `[sp-68] +0x29` | `-` |
| `ldr` at 0x3eab4 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3eab8 | load | 1 | `[sp-68] +0x26` | `-` |
| `str` at 0x3eabc | store | 4 | `sp -0x64` | `[[sp-68]+38]:1` |
| `ldr` at 0x3eac0 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3eac4 | load | 1 | `[sp-68] +0x28` | `-` |
| `str` at 0x3eac8 | store | 4 | `sp -0x60` | `[[sp-68]+40]:1` |
| `ldr` at 0x3eacc | load | 4 | `abs 0x3edf0` | `-` |
| `ldr` at 0x3ead8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3eadc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3eae0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3eae4 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3eae8 | load | 1 | `[sp-68] +0x2c` | `-` |
| `str` at 0x3eaec | store | 4 | `sp -0x64` | `[[sp-68]+44]:1` |
| `ldr` at 0x3eaf0 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3eaf4 | load | 1 | `[sp-68] +0x2d` | `-` |
| `str` at 0x3eaf8 | store | 4 | `sp -0x60` | `[[sp-68]+45]:1` |
| `ldr` at 0x3eafc | load | 4 | `abs 0x3edf4` | `-` |
| `ldr` at 0x3eb00 | load | 4 | `[sp-68] +0x18` | `-` |
| `ldr` at 0x3eb04 | load | 4 | `[sp-68] +0x14` | `-` |
| `ldr` at 0x3eb08 | load | 4 | `[sp-68] +0x30` | `-` |
| `ldr` at 0x3eb10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3eb1c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3eb24 | store | 4 | `[sp-68] +0xc` | `0x0` |
| `ldr` at 0x3eb28 | load | 4 | `abs 0x3ede4` | `-` |
| `ldr` at 0x3eb2c | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3eb38 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3eb3c | load | 4 | `abs 0x3edf8` | `-` |
| `ldr` at 0x3eb40 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x3eb48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3eb50 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3eb54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3eb5c | store | 4 | `sp -0x28` | `?` |
| `str` at 0x3eb68 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3eb6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3eb80 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x3eb90 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3eb94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3eb98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3eba8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ebc4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3ebcc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x3ebd0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ebd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3ebe8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3ebf8 | store | 4 | `sp -0x24` | `ret(0x3f630)` |
| `ldr` at 0x3ebfc | load | 4 | `abs 0x3ede4` | `-` |
| `ldr` at 0x3ec00 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3ec0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ec10 | load | 4 | `abs 0x3edfc` | `-` |
| `ldr` at 0x3ec14 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3ec1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ec20 | load | 4 | `[sp-72] +0x1c` | `-` |
| `stm` at 0x3ec2c | store | 4 | `sp -0x54` | `ret(0xfb644)` |
| `stm` at 0x3ec2c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3ec30 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ec34 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ec38 | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x3ec3c | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x3ec4c | load | 4 | `abs 0x3ee00` | `-` |
| `str` at 0x3ec50 | store | 4 | `sp -0x58` | `0x1091f4` |
| `ldr` at 0x3ec58 | load | 4 | `abs 0x3ee04` | `-` |
| `str` at 0x3ec5c | store | 4 | `sp -0x58` | `0x1091fc` |
| `ldr` at 0x3ec60 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3ec64 | load | 1 | `[sp-72] +0x29` | `-` |
| `str` at 0x3ec68 | store | 4 | `sp -0x64` | `[[sp-72]+41]:1` |
| `ldr` at 0x3ec6c | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3ec70 | load | 1 | `[sp-72] +0x26` | `-` |
| `str` at 0x3ec74 | store | 4 | `sp -0x60` | `[[sp-72]+38]:1` |
| `ldr` at 0x3ec78 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3ec7c | load | 1 | `[sp-72] +0x28` | `-` |
| `str` at 0x3ec80 | store | 4 | `sp -0x5c` | `[[sp-72]+40]:1` |
| `ldr` at 0x3ec84 | load | 4 | `abs 0x3ee08` | `-` |
| `ldm` at 0x3ec8c | load | 4 | `sp -0x54` | `-` |
| `ldm` at 0x3ec8c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3ec90 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x3ec98 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ec9c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3eca0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3eca4 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3eca8 | load | 1 | `[sp-72] +0x2c` | `-` |
| `str` at 0x3ecac | store | 4 | `sp -0x64` | `[[sp-72]+44]:1` |
| `ldr` at 0x3ecb0 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3ecb4 | load | 1 | `[sp-72] +0x2d` | `-` |
| `str` at 0x3ecb8 | store | 4 | `sp -0x60` | `[[sp-72]+45]:1` |
| `ldr` at 0x3ecbc | load | 4 | `abs 0x3edf4` | `-` |
| `ldr` at 0x3ecc0 | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x3ecc4 | load | 4 | `[sp-72] +0x14` | `-` |
| `ldr` at 0x3ecc8 | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x3ecd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ecdc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ece8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ecec | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3ecf0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3ecf8 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x3ecfc | load | 4 | `abs 0x3ede4` | `-` |
| `ldr` at 0x3ed00 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3ed0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ed10 | load | 4 | `abs 0x3ee0c` | `-` |
| `ldr` at 0x3ed14 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3ed20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3ed2c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3ed34 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x3ed38 | load | 4 | `abs 0x3ede4` | `-` |
| `ldr` at 0x3ed3c | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3ed48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ed4c | load | 4 | `abs 0x3ee10` | `-` |
| `ldr` at 0x3ed50 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3ed5c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3ed64 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x3ed68 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3ed70 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3ed78 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ed88 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3ed90 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x3ed98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3eda0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3edac | load | 4 | `abs 0x3ede0` | `-` |
| `ldr` at 0x3edb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3edb4 | store | 4 | `abs 0x14349c` | `[sp-28]` |
| `ldr` at 0x3edc0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3edc4 | store | 4 | `sp -0x4c` | `[sp-56]` |
| `ldr` at 0x3edc8 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x3edcc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3edcc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3edcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e89c | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x3e8d4 | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x608` | `0x0` |
| `bl` at 0x3e900 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x3e914 | `(unknown callee) 0x75848` | `0x5` | `?` | `0x14349c` | `0x5` |
| `bl` at 0x3e9f0 | `(unknown callee) 0x3ee14` | `[sp-68]` | `[sp-20]` | `[sp-40]` | `[sp-28]` |
| `bl` at 0x3ea40 | `(unknown callee) 0x23fbc` | `0x10911c` | `?` | `?` | `?` |
| `bl` at 0x3ea94 | `(unknown callee) 0x23fbc` | `0x109130` | `[[sp-68]+16]` | `[[sp-68]+37]:1` | `[[sp-68]+36]:1` |
| `bl` at 0x3eaa0 | `(unknown callee) 0xfb644` | `[[sp-68]+32]` | `?` | `?` | `[sp-68]` |
| `bl` at 0x3ead4 | `(unknown callee) 0x23fbc` | `0x109154` | `ret(0xfb644)` | `?` | `[[sp-68]+41]:1` |
| `bl` at 0x3eb0c | `(unknown callee) 0x23fbc` | `0x109190` | `[[sp-68]+24]` | `[[sp-68]+20]` | `[[sp-68]+48]` |
| `bl` at 0x3eb44 | `(unknown callee) 0x23fbc` | `0x1091d8` | `[[sp-68]+16]` | `[sp-68]` | `[sp-68]` |
| `bl` at 0x3eb88 | `(unknown callee) 0x412fc` | `0x6e` | `?` | `?` | `0x0` |
| `bl` at 0x3ebf0 | `(unknown callee) 0x3f630` | `?` | `sp-64` | `?` | `sp-64` |
| `bl` at 0x3ec18 | `(unknown callee) 0x23fbc` | `0x1091e8` | `[[sp-72]+16]` | `?` | `[sp-72]` |
| `bl` at 0x3ec24 | `(unknown callee) 0xfb644` | `[[sp-72]+28]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x3ec94 | `(unknown callee) 0x23fbc` | `0x109204` | `[sp-84]` | `[sp-80]` | `[sp-88]` |
| `bl` at 0x3eccc | `(unknown callee) 0x23fbc` | `0x109190` | `[[sp-72]+24]` | `[[sp-72]+20]` | `[[sp-72]+48]` |
| `bl` at 0x3ed18 | `(unknown callee) 0x23fbc` | `0x109244` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x3ed54 | `(unknown callee) 0x23fbc` | `0x109258` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x3ed9c | `(unknown callee) 0x1b490` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x3edbc | `(unknown callee) 0x75848` | `[sp-28]` | `?` | `0x14349c` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3e7d8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3e7f8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3e824 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x3e830 | `[sp-24] == 0x21` | `[sp-24] != 0x21` |
| `beq` at 0x3e840 | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `bne` at 0x3e86c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3e8b0 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `blt` at 0x3e938 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3e958 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3e984 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x3e990 | `[sp-24] == 0x21` | `[sp-24] != 0x21` |
| `beq` at 0x3e9a0 | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `beq` at 0x3ea04 | `ret(0x3ee14) == 0x0` | `ret(0x3ee14) != 0x0` |
| `beq` at 0x3ea14 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `bne` at 0x3ea58 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x3ea74 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `beq` at 0x3eb18 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0x3eb34 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `ble` at 0x3eb74 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x3eba0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3ebdc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3ec08 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `ble` at 0x3ec48 | `? <= 0x0 (signed, 32-bit)` | `? > 0x0 (signed, 32-bit)` |
| `beq` at 0x3ecd8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3ece4 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x3ed08 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `beq` at 0x3ed28 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0x3ed44 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `bne` at 0x3ed80 | `[sp-68] != 0x0` | `[sp-68] == 0x0` |
| `bne` at 0x3eda8 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3edcc | `r0` | `?` |

## `powerMarginName`

- `powerMarginName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x404e4 to 0x405f4.
  Established. The unnamed call to `0x4128c` has candidates `[sp-20]`, `ret(0x409a4)`, unknown,
  `[ret(0x409a4)+6]:1` (`bl` at 0x40540). The conditional computed transfer is taken for
  `ret(0x4128c) <= 0x5 (unsigned, 32-bit)` and has unresolved destination (`ldrls` at 0x4054c). The
  known fall-through writes four-byte `0x109484` at `sp -0x20` (`str` at 0x405b8). Other known-edge
  stores there include `0x109454` and `0x10948c` (`str` at 0x40520, `str` at 0x405c4). String
  meanings and selector-only effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x404f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x404f8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x404fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40500 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4050c | store | 4 | `sp -0x1c` | `ret(0x409a4)` |
| `ldr` at 0x40510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4051c | load | 4 | `abs 0x405d0` | `-` |
| `str` at 0x40520 | store | 4 | `sp -0x20` | `0x109454` |
| `ldr` at 0x40528 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4052c | load | 1 | `ret(0x409a4) +0x6` | `-` |
| `ldr` at 0x40538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4053c | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x4054c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x405b4 | load | 4 | `abs 0x405ec` | `-` |
| `str` at 0x405b8 | store | 4 | `sp -0x20` | `0x109484` |
| `ldr` at 0x405c0 | load | 4 | `abs 0x405f0` | `-` |
| `str` at 0x405c4 | store | 4 | `sp -0x20` | `0x10948c` |
| `ldr` at 0x405c8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x405cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x405cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x405cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40504 | `(unknown callee) 0x409a4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x40540 | `(unknown callee) 0x4128c` | `[sp-20]` | `ret(0x409a4)` | `?` | `[ret(0x409a4)+6]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x40518 | `ret(0x409a4) != 0x0` | `ret(0x409a4) == 0x0` |
| `beq` at 0x40534 | `[ret(0x409a4)+6]:1 == 0xff` | `[ret(0x409a4)+6]:1 != 0xff` |
| `ldrls` at 0x4054c | `ret(0x4128c) <= 0x5 (unsigned, 32-bit)` | `ret(0x4128c) > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x405cc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4054c).

## `powerGetIndexOfConfig`

- `powerGetIndexOfConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x408ec to 0x409a4.
  Established. The four-byte load reads absolute `0x148d00` (`ldr` at 0x4091c). The branch takes its
  unequal arm for `? != [sp-24]` (`bne` at 0x40970); its fall-through stores `[sp-28]` as four bytes
  at `sp -0x24` (`str` at 0x40978). The later alternative store there has unknown value (`str` at
  0x40994). The exit carries `[sp-36]` (`ldmdb` at 0x4099c). Configuration-record matching is
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x408fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40900 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40904 | load | 4 | `abs 0x409a0` | `-` |
| `ldr` at 0x40908 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4091c | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x40924 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x4092c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x40930 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x40934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40938 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x40948 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4094c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40960 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x40968 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40974 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40978 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x40980 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40988 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x40994 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x40998 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4099c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4099c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4099c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x40940 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x40970 | `? != [sp-24]` | `? == [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4099c | `r0` | `[sp-36]` |

## `power_margin_multi_init`

- `power_margin_multi_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ee14 to 0x3f630.
  Established. Four-byte assignments at `[sp-24] +0x18` and `+0x14` write `0x4`, while byte stores
  at `+0x2c` and `+0x2d` write `0x2` and `0x6` (`str` at 0x3ee80, `str` at 0x3ee88, `strb` at
  0x3eec4, `strb` at 0x3eed0). The call to unnamed `0x412c4` has candidates `[sp-28]`,
  `[[sp-24]+12]`, `0x5`, `[sp-24]` (`bl` at 0x3ef6c). The conditional computed transfer has
  unresolved destination and is taken for `[[[sp-24]+12]+8]-1 <= 0x4 (unsigned, 32-bit)` (`ldrls` at
  0x3ef98). Later byte stores at `[sp-24] +0x29` and `+0x28` retain unknown values (`strb` at
  0x3f4c0, `strb` at 0x3f4dc). Selector-only effects and margin-record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ee24 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3ee28 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3ee2c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x3ee30 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x3ee34 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x3ee3c | store | 4 | `[sp+4] +0x0` | `0x0` |
| `ldr` at 0x3ee40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ee58 | store | 4 | `[sp-24] +0x0` | `[sp-28]` |
| `ldr` at 0x3ee5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ee64 | store | 4 | `[sp-24] +0x4` | `[sp-32]` |
| `ldr` at 0x3ee68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ee70 | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0x3ee74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ee80 | store | 4 | `[sp-24] +0x18` | `0x4` |
| `str` at 0x3ee88 | store | 4 | `[sp-24] +0x14` | `0x4` |
| `ldr` at 0x3ee8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee90 | load | 4 | `abs 0x3f608` | `-` |
| `str` at 0x3ee94 | store | 4 | `[sp-24] +0x1c` | `0x0` |
| `ldr` at 0x3ee98 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eea0 | store | 1 | `[sp-24] +0x25` | `0x0` |
| `ldr` at 0x3eea4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eeac | store | 1 | `[sp-24] +0x27` | `0x0` |
| `ldr` at 0x3eeb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3eeb8 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0x3eebc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eec4 | store | 1 | `[sp-24] +0x2c` | `0x2` |
| `ldr` at 0x3eec8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eed0 | store | 1 | `[sp-24] +0x2d` | `0x6` |
| `ldr` at 0x3eed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eedc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3eee4 | store | 4 | `[sp-24] +0xc` | `ret(0x409a4)` |
| `ldr` at 0x3eee8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eef4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ef00 | store | 4 | `sp -0x40` | `ret(0x40a40)` |
| `ldr` at 0x3ef08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ef14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef18 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3ef1c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3ef28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef2c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3ef30 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3ef3c | load | 4 | `abs 0x3f60c` | `-` |
| `str` at 0x3ef40 | store | 4 | `sp -0x40` | `0xfffff9b8` |
| `ldr` at 0x3ef48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ef50 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x3ef58 | store | 4 | `sp -0x34` | `ret(0x4128c)` |
| `ldr` at 0x3ef5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ef64 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3ef70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef74 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldrb` at 0x3ef78 | load | 1 | `[[sp-24]+12] +0x6` | `-` |
| `ldr` at 0x3ef84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef88 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3ef8c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldrls` at 0x3ef98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f1c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f1d0 | load | 4 | `abs 0x3f614` | `-` |
| `ldr` at 0x3f1d4 | load | 4 | `abs 0x3f618` | `-` |
| `str` at 0x3f200 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3f204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f20c | load | 4 | `abs 0x3f614` | `-` |
| `ldr` at 0x3f210 | load | 4 | `abs 0x3f618` | `-` |
| `ldr` at 0x3f21c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3f22c | load | 4 | `abs 0x3f61c` | `-` |
| `str` at 0x3f230 | store | 4 | `(unknown)` | `0x1090dc` |
| `str` at 0x3f234 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x3f238 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f23c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f240 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `str` at 0x3f244 | store | 4 | `(unknown)` | `[[[sp-24]+12]+8]` |
| `ldr` at 0x3f24c | load | 4 | `abs 0x3f620` | `-` |
| `ldr` at 0x3f250 | load | 4 | `abs 0x3f624` | `-` |
| `ldr` at 0x3f260 | load | 4 | `abs 0x3f628` | `-` |
| `str` at 0x3f264 | store | 4 | `sp -0x40` | `0xfffff9be` |
| `ldr` at 0x3f26c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f27c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f280 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f284 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3f28c | store | 1 | `[sp-24] +0x26` | `(ret(0x4089c) & 0xff)` |
| `ldr` at 0x3f290 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f298 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f2a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f2a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f2ac | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f2b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f2b4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f2c0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x3f2c8 | store | 4 | `[sp+4] +0x0` | `0x1` |
| `ldr` at 0x3f2cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f2d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f2d4 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f2d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f2dc | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x3f2ec | store | 4 | `sp -0x40` | `ret(0x40d80)` |
| `ldr` at 0x3f2f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f2f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f304 | store | 4 | `sp -0x2c` | `ret(0x3df3c)` |
| `ldr` at 0x3f308 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f314 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f318 | load | 4 | `abs 0x3f60c` | `-` |
| `ldr` at 0x3f324 | load | 4 | `abs 0x3f62c` | `-` |
| `str` at 0x3f328 | store | 4 | `sp -0x40` | `0xfffff9bd` |
| `ldr` at 0x3f330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f334 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f338 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f340 | store | 4 | `[sp-24] +0x20` | `ret(0x40754)` |
| `ldr` at 0x3f344 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f348 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0x3f34c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3f358 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3f364 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3f374 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3f384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f388 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f38c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f390 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3f398 | store | 1 | `[sp-24] +0x26` | `(ret(0x40874) & 0xff)` |
| `ldr` at 0x3f39c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f3ac | store | 4 | `[sp-24] +0x20` | `ret(0x405f4)` |
| `ldr` at 0x3f3b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f3b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f3bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3c0 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3f3c8 | store | 1 | `[sp-24] +0x26` | `(ret(0x4089c) & 0xff)` |
| `ldr` at 0x3f3d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f3d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f3d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3dc | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3f3e4 | store | 1 | `[sp-24] +0x26` | `(ret(0x408c4) & 0xff)` |
| `ldr` at 0x3f3e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f3ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f3f8 | store | 4 | `[sp-24] +0x20` | `ret(0x406a4)` |
| `ldr` at 0x3f400 | load | 4 | `abs 0x3f610` | `-` |
| `str` at 0x3f404 | store | 4 | `sp -0x40` | `0xfffff9bc` |
| `ldr` at 0x3f40c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f410 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f41c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f420 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f424 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3f430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f434 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f438 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3f448 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f44c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f454 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x3f46c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3f470 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f474 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f47c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x3f494 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3f498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f49c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x3f4a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f4ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f4b0 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x3f4b4 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3f4c0 | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x3f4c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f4c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f4cc | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f4d0 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x3f4dc | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x3f4e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f4e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f4ec | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x3f4f0 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3f4fc | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x3f500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f504 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f508 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f50c | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x3f518 | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x3f520 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f524 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f528 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f52c | load | 1 | `[sp-24] +0x25` | `-` |
| `str` at 0x3f534 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3f538 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f53c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f540 | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x3f544 | store | 1 | `[sp-24] +0x29` | `([[sp-24]+37]:1 & 0xff)` |
| `ldr` at 0x3f548 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f54c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f550 | load | 1 | `[sp-24] +0x24` | `-` |
| `strb` at 0x3f554 | store | 1 | `[sp-24] +0x28` | `([[sp-24]+36]:1 & 0xff)` |
| `ldr` at 0x3f558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f55c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f560 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f564 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f568 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f56c | load | 1 | `[sp-24] +0x25` | `-` |
| `ldrb` at 0x3f584 | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x3f58c | store | 1 | `[sp-24] +0x26` | `?` |
| `ldr` at 0x3f590 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3f598 | store | 1 | `[sp-24] +0x2d` | `0x2` |
| `ldr` at 0x3f59c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f5a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f5a4 | load | 1 | `[sp-24] +0x29` | `-` |
| `strb` at 0x3f5a8 | store | 1 | `[sp-24] +0x2b` | `([[sp-24]+41]:1 & 0xff)` |
| `ldr` at 0x3f5ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f5b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f5b4 | load | 1 | `[sp-24] +0x28` | `-` |
| `strb` at 0x3f5b8 | store | 1 | `[sp-24] +0x2a` | `([[sp-24]+40]:1 & 0xff)` |
| `ldr` at 0x3f5bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f5c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f5c4 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f5c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f5cc | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x3f5dc | store | 4 | `sp -0x2c` | `ret(0x40d80)` |
| `ldr` at 0x3f5e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f5ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3f5f0 | store | 4 | `sp -0x40` | `ret(0x40d80)` |
| `str` at 0x3f5fc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x3f600 | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0x3f604 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3f604 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3f604 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3f604 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ee4c | `(unknown callee) 0x1b4cc` | `[sp-24]` | `0x0` | `0x4` | `0x0` |
| `bl` at 0x3eee0 | `(unknown callee) 0x409a4` | `[sp-28]` | `[sp-32]` | `[sp-24]` | `0x6` |
| `bl` at 0x3eef8 | `(unknown callee) 0x40a40` | `[sp-28]` | `?` | `?` | `[[sp-24]+12]` |
| `bl` at 0x3ef54 | `(unknown callee) 0x4128c` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3ef6c | `(unknown callee) 0x412c4` | `[sp-28]` | `[[sp-24]+12]` | `0x5` | `[sp-24]` |
| `bl` at 0x3f1d8 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3f1e4 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3f214 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3f224 | `(unknown callee) 0x1b598` | `[sp-56]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3f258 | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x6fd` | `0x0` |
| `bl` at 0x3f288 | `(unknown callee) 0x4089c` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3f2a0 | `(unknown callee) 0x412c4` | `[sp-28]` | `[[sp-24]+12]` | `0x0` | `[sp-24]` |
| `bl` at 0x3f2bc | `(unknown callee) 0x41254` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x3f2e4 | `(unknown callee) 0x40d80` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x3f2fc | `(unknown callee) 0x3df3c` | `[sp-28]` | `[sp-32]` | `?` | `[sp-36]` |
| `bl` at 0x3f33c | `(unknown callee) 0x40754` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3f394 | `(unknown callee) 0x40874` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3f3a8 | `(unknown callee) 0x405f4` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3f3c4 | `(unknown callee) 0x4089c` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3f3e0 | `(unknown callee) 0x408c4` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3f3f4 | `(unknown callee) 0x406a4` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3f5d4 | `(unknown callee) 0x40d80` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3eef0 | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `bne` at 0x3ef10 | `[sp-36] != 0x6` | `[sp-36] == 0x6` |
| `beq` at 0x3ef24 | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x3ef38 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `beq` at 0x3ef80 | `[[[sp-24]+12]+6]:1 == 0xff` | `[[[sp-24]+12]+6]:1 != 0xff` |
| `ldrls` at 0x3ef98 | `[[[sp-24]+12]+8]-1 <= 0x4 (unsigned, 32-bit)` | `[[[sp-24]+12]+8]-1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0x3f274 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x3f310 | `ret(0x3df3c) == 0x1` | `ret(0x3df3c) != 0x1` |
| `beq` at 0x3f320 | `ret(0x3df3c) == 0xfffff9b8` | `ret(0x3df3c) != 0xfffff9b8` |
| `beq` at 0x3f354 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0x3f360 | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x3f36c | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0x3f37c | `[sp-36] == 0x3` | `[sp-36] != 0x3` |
| `beq` at 0x3f418 | `[[sp-24]+38]:1 == 0x0` | `[[sp-24]+38]:1 != 0x0` |
| `beq` at 0x3f42c | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x3f440 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `ble` at 0x3f4a4 | `[[sp-24]+48] <= 0x0 (signed, 32-bit)` | `[[sp-24]+48] > 0x0 (signed, 32-bit)` |
| `beq` at 0x3f5e8 | `ret(0x40d80) == 0x0` | `ret(0x40d80) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3f604 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3ef98).

## `power_margin_multi`

- `power_margin_multi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f630 to 0x40068.
  Established. The unnamed call to `0x400fc` has candidates unknown, unknown, `0x0`, unknown (`bl`
  at 0x3f6a8); its zero result takes the branch to `0x3f6cc` (`beq` at 0x3f6bc). Four-byte stores at
  `[sp-32] +0x0` include `0x0` and `0x1` in distinct paths (`str` at 0x3f6d4, `str` at 0x3f724).
  Later calls include unnamed `0x40d80` with `[[sp-28]]`, `[[sp-28]+12]`, `[[sp-28]+38]:1`,
  `[[sp-28]+38]:1` (`bl` at 0x3fa24). A byte assignment at `[sp-28] +0x2d` writes `0x2` (`strb` at
  0x3ffb4). Feedback-control and voltage-unit interpretations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f640 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3f644 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x3f648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f658 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x3f664 | load | 4 | `abs 0x40028` | `-` |
| `str` at 0x3f668 | store | 4 | `sp -0x40` | `0x109268` |
| `ldr` at 0x3f670 | load | 4 | `abs 0x4002c` | `-` |
| `ldr` at 0x3f674 | load | 4 | `abs 0x40030` | `-` |
| `ldr` at 0x3f680 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f684 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3f688 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3f68c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f694 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f69c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f6a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3f6b0 | store | 4 | `sp -0x24` | `ret(0x400fc)` |
| `ldr` at 0x3f6b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f6c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f6c4 | store | 4 | `sp -0x2c` | `ret(0x400fc)` |
| `ldr` at 0x3f6cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f6d4 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x3f6d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6e0 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f6e4 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f6fc | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3f700 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3f70c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f710 | load | 4 | `abs 0x40038` | `-` |
| `ldr` at 0x3f714 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f71c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f724 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f72c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f730 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f734 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f74c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f750 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f754 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f76c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f770 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f784 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3f788 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3f794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f798 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f7a8 | load | 4 | `abs 0x4003c` | `-` |
| `ldr` at 0x3f7ac | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f7bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f7c8 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3f7cc | load | 1 | `[sp-28] +0x18` | `-` |
| `strb` at 0x3f7d4 | store | 1 | `[sp-28] +0x26` | `?` |
| `ldr` at 0x3f7d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f7e0 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f7e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7ec | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f7f0 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f80c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f820 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f828 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f82c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3f83c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f844 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f848 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f85c | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3f860 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3f86c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f870 | load | 4 | `abs 0x40040` | `-` |
| `ldr` at 0x3f874 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f87c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f884 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f88c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f890 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f894 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f8ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f8b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f8b4 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f8cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8d0 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f8d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f8d8 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f8e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8e4 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f8f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f8f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8f8 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldrb` at 0x3f8fc | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0x3f908 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3f90c | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3f918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f91c | load | 4 | `abs 0x40044` | `-` |
| `ldr` at 0x3f920 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f92c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f930 | load | 1 | `[sp-28] +0x28` | `-` |
| `strb` at 0x3f934 | store | 1 | `[sp-28] +0x26` | `([[sp-28]+40]:1 & 0xff)` |
| `ldr` at 0x3f938 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f940 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f948 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f94c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f950 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f968 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f96c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f970 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f974 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f97c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f980 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3f98c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f990 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f994 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldrb` at 0x3f998 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0x3f9a4 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3f9a8 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3f9b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f9b8 | load | 4 | `abs 0x40048` | `-` |
| `ldr` at 0x3f9bc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f9c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f9cc | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f9d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3f9d4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x3f9e0 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3f9e4 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3f9f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f9f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f9f8 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f9fc | load | 4 | `abs 0x4004c` | `-` |
| `ldr` at 0x3fa00 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fa08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fa14 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fa18 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fa1c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x3fa2c | store | 4 | `sp -0x24` | `ret(0x40d80)` |
| `ldr` at 0x3fa30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fa3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3fa44 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x3fa48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3fa4c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x3fa54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa60 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fa64 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fa68 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3fa70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fa7c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fa80 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fa84 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x3fa94 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3fa9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3faa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3faa4 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3faa8 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3fab8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fabc | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3fad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fad4 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3fae8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3faf0 | store | 1 | `[sp-28] +0x2c` | `0x1` |
| `ldr` at 0x3faf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3faf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fafc | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0x3fb00 | store | 4 | `[sp-28] +0x14` | `[[sp-28]+24]` |
| `ldr` at 0x3fb04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb0c | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3fb10 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3fb28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb2c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb30 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3fb34 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0x3fb40 | load | 4 | `abs 0x40050` | `-` |
| `str` at 0x3fb44 | store | 4 | `sp -0x2c` | `0xfffff9ba` |
| `ldr` at 0x3fb4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb54 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3fb58 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fb64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb68 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fb6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb70 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fb74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb78 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3fb8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb90 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb94 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldrb` at 0x3fb98 | load | 1 | `[sp-28] +0x2a` | `-` |
| `ldr` at 0x3fba4 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3fba8 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3fbb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fbb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fbbc | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fbc0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fbc4 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fbc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fbcc | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0x3fbd0 | store | 4 | `sp -0x40` | `[[sp-28]+48]` |
| `ldr` at 0x3fbd4 | load | 4 | `abs 0x40054` | `-` |
| `ldr` at 0x3fbd8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fbe8 | load | 4 | `abs 0x40058` | `-` |
| `str` at 0x3fbec | store | 4 | `sp -0x2c` | `0xfffff9b7` |
| `ldr` at 0x3fbf4 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3fbf8 | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3fc04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc08 | load | 4 | `abs 0x4005c` | `-` |
| `ldr` at 0x3fc0c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fc14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc20 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fc24 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fc28 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3fc30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fc3c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fc40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fc44 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fc50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3fc58 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `str` at 0x3fc60 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3fc68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc6c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3fc80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc84 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fc88 | load | 1 | `[sp-28] +0x26` | `-` |
| `strb` at 0x3fc8c | store | 1 | `[sp-28] +0x29` | `([[sp-28]+38]:1 & 0xff)` |
| `ldr` at 0x3fc90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3fc94 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x3fc98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fc9c | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fca0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fca4 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3fcbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fcc0 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3fcdc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fce0 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fce4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fce8 | load | 1 | `[sp-28] +0x29` | `-` |
| `str` at 0x3fcfc | store | 4 | `sp -0x34` | `ret(0x1b520)` |
| `ldr` at 0x3fd00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd04 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3fd08 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3fd18 | store | 4 | `sp -0x34` | `ret(0xfb0a8)` |
| `str` at 0x3fd24 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x3fd28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3fd2c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3fd30 | store | 4 | `[sp-48] +0x18` | `?` |
| `ldr` at 0x3fd38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fd3c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd40 | load | 1 | `[sp-28] +0x2c` | `-` |
| `str` at 0x3fd44 | store | 4 | `[sp-28] +0x18` | `[[sp-28]+44]:1` |
| `ldr` at 0x3fd4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fd50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd54 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3fd58 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0x3fd64 | load | 4 | `abs 0x40050` | `-` |
| `str` at 0x3fd68 | store | 4 | `sp -0x2c` | `0xfffff9ba` |
| `ldr` at 0x3fd70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fd74 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd78 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3fd7c | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3fd88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd8c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fd90 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fd94 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3fd98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fd9c | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3fdb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fdb4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fdb8 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldrb` at 0x3fdbc | load | 1 | `[sp-28] +0x2b` | `-` |
| `ldr` at 0x3fdc8 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3fdcc | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3fdd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fddc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fde0 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fde4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fde8 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3fdec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fdf0 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0x3fdf4 | store | 4 | `sp -0x40` | `[[sp-28]+48]` |
| `ldr` at 0x3fdf8 | load | 4 | `abs 0x40060` | `-` |
| `ldr` at 0x3fdfc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fe0c | load | 4 | `abs 0x40058` | `-` |
| `str` at 0x3fe10 | store | 4 | `sp -0x2c` | `0xfffff9b7` |
| `ldr` at 0x3fe18 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3fe1c | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x3fe28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe2c | load | 4 | `abs 0x4005c` | `-` |
| `ldr` at 0x3fe30 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fe38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe44 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fe48 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fe4c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3fe54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fe60 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fe64 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fe68 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fe74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3fe7c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `str` at 0x3fe84 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3fe8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe90 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3fea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fea8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3feac | load | 1 | `[sp-28] +0x26` | `-` |
| `strb` at 0x3feb0 | store | 1 | `[sp-28] +0x28` | `([[sp-28]+38]:1 & 0xff)` |
| `ldr` at 0x3feb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3feb8 | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x3febc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fec0 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fec4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fec8 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3fee0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fee4 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3ff00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ff04 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3ff08 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ff0c | load | 1 | `[sp-28] +0x29` | `-` |
| `str` at 0x3ff20 | store | 4 | `sp -0x3c` | `ret(0x1b520)` |
| `ldr` at 0x3ff24 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ff28 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3ff2c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3ff40 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x3ff4c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3ff50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3ff54 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3ff58 | store | 4 | `[sp-56] +0x18` | `[sp-60]` |
| `ldr` at 0x3ff60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ff64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ff68 | load | 1 | `[sp-28] +0x2c` | `-` |
| `str` at 0x3ff70 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0x3ff74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ff78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ff7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ff80 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3ff84 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0x3ff8c | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0x3ff90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ff94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ff98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ff9c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3ffa0 | load | 1 | `[sp-28] +0x18` | `-` |
| `strb` at 0x3ffa8 | store | 1 | `[sp-28] +0x26` | `(([[sp-28]+38]:1 + [[sp-28]+24]:1) & 0xff)` |
| `ldr` at 0x3ffac | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3ffb4 | store | 1 | `[sp-28] +0x2d` | `0x2` |
| `ldr` at 0x3ffb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ffbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ffc0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ffc4 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3ffc8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3ffcc | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x3ffdc | store | 4 | `sp -0x24` | `ret(0x40d80)` |
| `ldr` at 0x3ffe0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ffec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3fff0 | store | 4 | `sp -0x2c` | `ret(0x40d80)` |
| `ldr` at 0x3fff8 | load | 4 | `abs 0x40034` | `-` |
| `ldr` at 0x3fffc | load | 4 | `abs 0x1492a0` | `-` |
| `ldr` at 0x40008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4000c | load | 4 | `abs 0x40064` | `-` |
| `ldr` at 0x40010 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x4001c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x40020 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x40024 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x40024 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x40024 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40024 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40024 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f67c | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x763` | `0x0` |
| `bl` at 0x3f6a8 | `(unknown callee) 0x400fc` | `?` | `?` | `0x0` | `?` |
| `bl` at 0x3f6e8 | `(unknown callee) 0xfc33c` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f718 | `(unknown callee) 0x23fbc` | `0x1092a4` | `[[sp-28]+16]` | `?` | `[sp-28]` |
| `bl` at 0x3f738 | `(unknown callee) 0xfc334` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f758 | `(unknown callee) 0xfc32c` | `[sp-40]` | `[[sp-28]+32]` | `?` | `[sp-28]` |
| `bl` at 0x3f774 | `(unknown callee) 0x1b520` | `[[sp-28]+24]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3f79c | `(unknown callee) 0xfb644` | `[sp-40]` | `?` | `?` | `[0x1492a0]` |
| `bl` at 0x3f7b8 | `(unknown callee) 0x23fbc` | `0x1092c4` | `[[sp-28]+16]` | `ret(0xfb644)` | `?` |
| `bl` at 0x3f7f4 | `(unknown callee) 0xfc32c` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f810 | `(unknown callee) 0x1b520` | `[[sp-28]+24]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3f878 | `(unknown callee) 0x23fbc` | `0x1092fc` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f898 | `(unknown callee) 0xfc334` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f8b8 | `(unknown callee) 0xfc334` | `[sp-40]` | `[[sp-28]+32]` | `?` | `[sp-28]` |
| `bl` at 0x3f924 | `(unknown callee) 0x23fbc` | `0x109334` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f954 | `(unknown callee) 0xfc32c` | `[sp-40]` | `[[sp-28]+32]` | `?` | `[sp-28]` |
| `bl` at 0x3f9c0 | `(unknown callee) 0x23fbc` | `0x109360` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3fa04 | `(unknown callee) 0x23fbc` | `0x10939c` | `[[sp-28]+16]` | `[[sp-28]+38]:1` | `[sp-28]` |
| `bl` at 0x3fa24 | `(unknown callee) 0x40d80` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3fa6c | `(unknown callee) 0x412c4` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0x3fa8c | `(unknown callee) 0x41254` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3fac0 | `(unknown callee) 0x1b520` | `[[sp-28]+24]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3fad8 | `(unknown callee) 0x1b520` | `[[sp-28]+20]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3fb14 | `(unknown callee) 0xfc334` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3fbe4 | `(unknown callee) 0x23fbc` | `0x1093bc` | `[[sp-28]+16]` | `[[sp-28]+38]:1` | `[[sp-28]+40]:1` |
| `bl` at 0x3fc10 | `(unknown callee) 0x23fbc` | `0x1093ec` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3fc2c | `(unknown callee) 0x412c4` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0x3fc4c | `(unknown callee) 0x41254` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3fc70 | `(unknown callee) 0x1b520` | `[[sp-28]+20]` | `[[sp-28]+38]:1` | `[[sp-28]+40]:1` | `[sp-28]` |
| `bl` at 0x3fcb0 | `(unknown callee) 0x1b520` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3fccc | `(unknown callee) 0xfb0a8` | `ret(0x1b520)` | `[[sp-28]+45]:1` | `ret(0x1b520)` | `[[sp-28]+45]:1` |
| `bl` at 0x3fcf4 | `(unknown callee) 0x1b520` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3fd10 | `(unknown callee) 0xfb0a8` | `ret(0x1b520)` | `[[sp-28]+45]:1` | `?` | `[[sp-28]+45]:1` |
| `bl` at 0x3fe08 | `(unknown callee) 0x23fbc` | `0x109414` | `[[sp-28]+16]` | `[[sp-28]+38]:1` | `[[sp-28]+41]:1` |
| `bl` at 0x3fe34 | `(unknown callee) 0x23fbc` | `0x1093ec` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3fe50 | `(unknown callee) 0x412c4` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0x3fe70 | `(unknown callee) 0x41254` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3fe94 | `(unknown callee) 0x1b520` | `[[sp-28]+20]` | `[[sp-28]+38]:1` | `[[sp-28]+41]:1` | `[sp-28]` |
| `bl` at 0x3fed4 | `(unknown callee) 0x1b520` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3fef0 | `(unknown callee) 0xfb0a8` | `ret(0x1b520)` | `[[sp-28]+45]:1` | `ret(0x1b520)` | `[[sp-28]+45]:1` |
| `bl` at 0x3ff18 | `(unknown callee) 0x1b520` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3ff34 | `(unknown callee) 0xfb0a8` | `ret(0x1b520)` | `[[sp-28]+45]:1` | `?` | `[[sp-28]+45]:1` |
| `bl` at 0x3ffd4 | `(unknown callee) 0x40d80` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x40014 | `(unknown callee) 0x23fbc` | `0x109444` | `[[sp-28]+16]` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3f650 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x3f660 | `[r0+12] != 0x0` | `[r0+12] == 0x0` |
| `beq` at 0x3f6bc | `ret(0x400fc) == 0x0` | `ret(0x400fc) != 0x0` |
| `beq` at 0x3f6f4 | `ret(0xfc33c) == 0x0` | `ret(0xfc33c) != 0x0` |
| `beq` at 0x3f708 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `blt` at 0x3f744 | `ret(0xfc334) < 0x0 (signed, 32-bit)` | `ret(0xfc334) >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3f764 | `ret(0xfc32c) >= 0x0 (signed, 32-bit)` | `ret(0xfc32c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3f780 | `ret(0x1b520) != 0x1` | `ret(0x1b520) == 0x1` |
| `beq` at 0x3f790 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `bge` at 0x3f800 | `ret(0xfc32c) >= 0x0 (signed, 32-bit)` | `ret(0xfc32c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3f81c | `ret(0x1b520) != 0x1` | `ret(0x1b520) == 0x1` |
| `bne` at 0x3f838 | `([[sp-28]+24] + [[sp-28]+20]) != 0x0` | `([[sp-28]+24] + [[sp-28]+20]) == 0x0` |
| `bgt` at 0x3f854 | `? > 0x0 (signed, 32-bit)` | `? <= 0x0 (signed, 32-bit)` |
| `beq` at 0x3f868 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `blt` at 0x3f8a4 | `ret(0xfc334) < 0x0 (signed, 32-bit)` | `ret(0xfc334) >= 0x0 (signed, 32-bit)` |
| `blt` at 0x3f8c4 | `ret(0xfc334) < 0x0 (signed, 32-bit)` | `ret(0xfc334) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x3f8ec | `([[sp-28]+38]:1 + [[sp-28]+48]) != [[sp-28]+40]:1` | `([[sp-28]+38]:1 + [[sp-28]+48]) == [[sp-28]+40]:1` |
| `beq` at 0x3f904 | `[[sp-28]+40]:1 == [[sp-28]+36]:1` | `[[sp-28]+40]:1 != [[sp-28]+36]:1` |
| `beq` at 0x3f914 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `bgt` at 0x3f960 | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x3f988 | `? != [[sp-28]+41]:1` | `? == [[sp-28]+41]:1` |
| `beq` at 0x3f9a0 | `[[sp-28]+41]:1 == [[sp-28]+37]:1` | `[[sp-28]+41]:1 != [[sp-28]+37]:1` |
| `beq` at 0x3f9b0 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `beq` at 0x3f9dc | `[[sp-32]] == 0x0` | `[[sp-32]] != 0x0` |
| `beq` at 0x3f9ec | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `beq` at 0x3fa38 | `ret(0x40d80) == 0x0` | `ret(0x40d80) != 0x0` |
| `bge` at 0x3fab4 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bne` at 0x3facc | `ret(0x1b520) != 0x2` | `ret(0x1b520) == 0x2` |
| `bne` at 0x3fae4 | `ret(0x1b520) != 0x2` | `ret(0x1b520) == 0x2` |
| `blt` at 0x3fb20 | `ret(0xfc334) < 0x0 (signed, 32-bit)` | `ret(0xfc334) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x3fb3c | `[[sp-28]+38]:1 != [[sp-28]+36]:1` | `[[sp-28]+38]:1 == [[sp-28]+36]:1` |
| `beq` at 0x3fb60 | `[[sp-28]+38]:1 == [[sp-28]+40]:1` | `[[sp-28]+38]:1 != [[sp-28]+40]:1` |
| `beq` at 0x3fb84 | `[[sp-28]+38]:1 == ([[sp-28]+40]:1 + [[sp-28]+48])` | `[[sp-28]+38]:1 != ([[sp-28]+40]:1 + [[sp-28]+48])` |
| `bne` at 0x3fba0 | `[[sp-28]+40]:1 != [[sp-28]+42]:1` | `[[sp-28]+40]:1 == [[sp-28]+42]:1` |
| `beq` at 0x3fbb0 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `beq` at 0x3fc00 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `ble` at 0x3fc7c | `ret(0x1b520) <= 0x3 (signed, 32-bit)` | `ret(0x1b520) > 0x3 (signed, 32-bit)` |
| `ble` at 0x3fcd8 | `ret(0xfb0a8) <= 0x1 (signed, 32-bit)` | `ret(0xfb0a8) > 0x1 (signed, 32-bit)` |
| `bne` at 0x3fd60 | `[[sp-28]+38]:1 != [[sp-28]+37]:1` | `[[sp-28]+38]:1 == [[sp-28]+37]:1` |
| `beq` at 0x3fd84 | `[[sp-28]+38]:1 == [[sp-28]+41]:1` | `[[sp-28]+38]:1 != [[sp-28]+41]:1` |
| `beq` at 0x3fda8 | `[[sp-28]+38]:1 == ?` | `[[sp-28]+38]:1 != ?` |
| `bne` at 0x3fdc4 | `[[sp-28]+41]:1 != [[sp-28]+43]:1` | `[[sp-28]+41]:1 == [[sp-28]+43]:1` |
| `beq` at 0x3fdd4 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `beq` at 0x3fe24 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |
| `ble` at 0x3fea0 | `ret(0x1b520) <= 0x3 (signed, 32-bit)` | `ret(0x1b520) > 0x3 (signed, 32-bit)` |
| `ble` at 0x3fefc | `ret(0xfb0a8) <= 0x1 (signed, 32-bit)` | `ret(0xfb0a8) > 0x1 (signed, 32-bit)` |
| `beq` at 0x3ffe8 | `ret(0x40d80) == 0x0` | `ret(0x40d80) != 0x0` |
| `beq` at 0x40004 | `[0x1492a0] == 0x0` | `[0x1492a0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40024 | `r0` | `?` |

## `EnvPowerUpStart`

- `EnvPowerUpStart`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82c60 to 0x82c78.
  Established. The call to unnamed `0x8107c` has candidates `0x2`, `r1`, `r2`, `r3` (`bl` at
  0x82c70). The exit retains `ret(0x8107c)` (`ldmdb` at 0x82c74). The event meaning of the first
  candidate is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x82c74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82c74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82c74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82c70 | `(unknown callee) 0x8107c` | `0x2` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82c74 | `r0` | `ret(0x8107c)` |

## `EnvPowerUpDone`

- `EnvPowerUpDone`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82c78 to 0x82ca4.
  Established. The unnamed call to `0x1b274` carries `0x0`, `r1`, `r2`, `r3`, and its result is
  assigned as four bytes at absolute `0x149b9c` (`bl` at 0x82c8c, `str` at 0x82c90). The subsequent
  call to `0x8107c` has first candidate `0x4` and three unknown candidates (`bl` at 0x82c98). The
  stored value and event roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x82c84 | load | 4 | `abs 0x82ca0` | `-` |
| `str` at 0x82c90 | store | 4 | `abs 0x149b9c` | `ret(0x1b274)` |
| `ldmdb` at 0x82c9c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82c9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82c9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82c9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82c8c | `(unknown callee) 0x1b274` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x82c98 | `(unknown callee) 0x8107c` | `0x4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82c9c | `r0` | `ret(0x8107c)` |

## `EnvPowerDownStart`

- `EnvPowerDownStart`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82cc4 to 0x82cdc.
  Established. The unnamed call to `0x8107c` uses `0x3`, `r1`, `r2`, `r3` (`bl` at 0x82cd4). The
  return carries `ret(0x8107c)` (`ldmdb` at 0x82cd8). Power-down notification semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x82cd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82cd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82cd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82cd4 | `(unknown callee) 0x8107c` | `0x3` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82cd8 | `r0` | `ret(0x8107c)` |

## `EnvPowerDownDone`

- `EnvPowerDownDone`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82cdc to 0x82cf8.
  Established. The call to unnamed `0x8107c` carries `0x4`, `r1`, `r2`, `r3` (`bl` at 0x82cec). A
  following call to `0x862bc` has `ret(0x8107c)` and three unknown candidates (`bl` at 0x82cf0). The
  exit retains `ret(0x862bc)` (`ldmdb` at 0x82cf4). The two callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x82cf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82cf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82cf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82cec | `(unknown callee) 0x8107c` | `0x4` | `r1` | `r2` | `r3` |
| `bl` at 0x82cf0 | `(unknown callee) 0x862bc` | `ret(0x8107c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82cf4 | `r0` | `ret(0x862bc)` |
