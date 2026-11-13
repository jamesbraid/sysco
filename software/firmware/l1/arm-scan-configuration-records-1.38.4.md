# ARM scan configuration and record setup in 1.38.4

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
Computed transfers in two link-control routines have unresolved destinations. Their tables and returns cover only known edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/scan-configuration-records-facts.txt) retain all tool sections.

## `JTAG_Cfg_display_f`

- `JTAG_Cfg_display_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe70e0 to 0xe739c.
  Established. The unsigned global-value test takes its bypass for `[0x2ad25c] <= 0x2` and falls
  through for greater (`bls` at 0xe70fc). The call to `0x21e28` has `0x6`, `0x135958`, `0x1b1`,
  `0x0` (`bl` at 0xe7118). The last call reaches `0x23f2c` with `0x135bdc` and three unknown
  candidates (`bl` at 0xe7360); its result reaches the exit as `ret(0x23f2c)` (`ldmdb` at 0xe7364).
  Display field meanings and the callee roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe70f0 | load | 4 | `abs 0xe7368` | `-` |
| `ldr` at 0xe70f4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe7100 | load | 4 | `abs 0xe736c` | `-` |
| `str` at 0xe7104 | store | 4 | `sp -0x20` | `0x135b20` |
| `ldr` at 0xe710c | load | 4 | `abs 0xe7370` | `-` |
| `ldr` at 0xe7110 | load | 4 | `abs 0xe7374` | `-` |
| `ldr` at 0xe711c | load | 4 | `abs 0xe7378` | `-` |
| `ldr` at 0xe7124 | load | 4 | `abs 0xe737c` | `-` |
| `ldr` at 0xe7128 | load | 4 | `abs 0xe7380` | `-` |
| `ldr` at 0xe712c | load | 4 | `abs 0x2ad268` | `-` |
| `ldr` at 0xe7134 | load | 4 | `abs 0xe7384` | `-` |
| `ldr` at 0xe7138 | load | 4 | `abs 0xe7388` | `-` |
| `ldr` at 0xe713c | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe7148 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xe714c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe715c | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe7160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7180 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7190 | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe7194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe71ac | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe71b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe71d0 | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe71d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe71f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe71f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe71fc | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe7200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7220 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7224 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe7228 | load | 4 | `abs 0xe7390` | `-` |
| `ldr` at 0xe722c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7230 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7234 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7240 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe7244 | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe7248 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7268 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe726c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe727c | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe7280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe72a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe72a8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe72b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe72b4 | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe72b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe72d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe72e0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe72e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe72ec | load | 4 | `abs 0xe738c` | `-` |
| `ldr` at 0xe72f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7318 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7320 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe7324 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7328 | load | 4 | `abs 0xe7394` | `-` |
| `ldr` at 0xe732c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe733c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7344 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xe734c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe7354 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe735c | load | 4 | `abs 0xe7398` | `-` |
| `ldmdb` at 0xe7364 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7364 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7364 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7118 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x1b1` | `0x0` |
| `bl` at 0xe7120 | `(unknown callee) 0x23f2c` | `0x135b34` | `?` | `?` | `?` |
| `bl` at 0xe7130 | `(unknown callee) 0x23f2c` | `0x135b4c` | `[0x2ad268]` | `?` | `0x2ad268` |
| `bl` at 0xe7140 | `(unknown callee) 0x23f2c` | `0x135b5c` | `[0x2ad26c]` | `?` | `0x2ad26c` |
| `bl` at 0xe7238 | `(unknown callee) 0x23f2c` | `0x135b6c` | `?` | `?` | `?` |
| `bl` at 0xe7338 | `(unknown callee) 0x23f2c` | `0x135ba8` | `?` | `?` | `?` |
| `bl` at 0xe7360 | `(unknown callee) 0x23f2c` | `0x135bdc` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe70fc | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xe7154 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xe7188 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe7274 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7364 | `r0` | `ret(0x23f2c)` |

## `JTAG_Cfg_L3SetCfg_f`

- `JTAG_Cfg_L3SetCfg_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe739c to 0xe77f4.
  Established. The word store at `[sp-20] +0x0` has unknown value (`str` at 0xe7564). Later byte
  stores have unknown targets, carrying unknown and `([sp-32]:1 & 0xff)`, respectively (`strb` at
  0xe7618, `strb` at 0xe7658). The final call to `0xe974c` has `[0x2ad26c]`, `0x14`, unknown,
  `0x2ad26c` (`bl` at 0xe77bc). Physical configuration semantics and r0 at the common exit are
  unresolved (`ldmdb` at 0xe77cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe73ac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe73b0 | load | 4 | `abs 0xe77d0` | `-` |
| `ldr` at 0xe73b4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe73c0 | load | 4 | `abs 0xe77d4` | `-` |
| `str` at 0xe73c4 | store | 4 | `sp -0x3c` | `0x135be0` |
| `ldr` at 0xe73cc | load | 4 | `abs 0xe77d8` | `-` |
| `ldr` at 0xe73e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe73e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe73ec | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe73f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe73fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7400 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe7404 | load | 4 | `abs 0xe77d0` | `-` |
| `ldr` at 0xe7408 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe7414 | load | 4 | `abs 0xe77dc` | `-` |
| `str` at 0xe7418 | store | 4 | `sp -0x3c` | `0x135bf4` |
| `ldr` at 0xe741c | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe7420 | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe7424 | store | 4 | `sp -0x38` | `[0x2ad26c]` |
| `ldr` at 0xe7428 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe742c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe7430 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7434 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe743c | load | 4 | `abs 0xe77d8` | `-` |
| `ldr` at 0xe7440 | load | 4 | `abs 0xe77e4` | `-` |
| `ldr` at 0xe744c | load | 4 | `abs 0xe77d0` | `-` |
| `ldr` at 0xe7450 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe745c | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe7460 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe7464 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe747c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe748c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7498 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe749c | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe74a0 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe74c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe74c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe74d0 | load | 4 | `abs 0xe77ec` | `-` |
| `str` at 0xe74d4 | store | 4 | `sp -0x3c` | `0x135c24` |
| `ldr` at 0xe74d8 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe74dc | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe74e0 | store | 4 | `sp -0x38` | `[0x2ad26c]` |
| `ldr` at 0xe74e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe74e8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe74ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe74f0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe74f4 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe74f8 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe74fc | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe751c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7520 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7528 | load | 4 | `abs 0xe77d8` | `-` |
| `ldr` at 0xe752c | load | 4 | `abs 0xe77f0` | `-` |
| `ldr` at 0xe7538 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe753c | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe7540 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe755c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xe7564 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xe756c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe7574 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe7578 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe757c | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe7594 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7598 | store | 4 | `(unknown)` | `[sp-24]` |
| `strb` at 0xe75a0 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0xe75a4 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe75a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xe75b8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe75c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe75d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe75d4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe75d8 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe75dc | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe75e0 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldrb` at 0xe7600 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe7608 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7610 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xe7618 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe761c | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe7620 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe7624 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldrb` at 0xe7644 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe764c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7654 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xe7658 | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xe765c | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe7660 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe7664 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldrb` at 0xe7684 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe768c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe7694 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe76a0 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe76a4 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe76a8 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldrb` at 0xe76c8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe76d0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xe76d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe76f8 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe76fc | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe7700 | load | 4 | `abs 0x2ad26c` | `-` |
| `ldrb` at 0xe7720 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xe7728 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xe7734 | store | 1 | `(unknown)` | `0x3` |
| `ldrb` at 0xe7738 | load | 1 | `sp -0x22` | `-` |
| `strb` at 0xe7740 | store | 1 | `sp -0x22` | `([sp-34]:1+1 & 0xff)` |
| `ldr` at 0xe7748 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe774c | load | 4 | `abs 0x2ad26c` | `-` |
| `ldr` at 0xe7754 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe7758 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe775c | load | 4 | `abs 0x2ad26c` | `-` |
| `strb` at 0xe7780 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7784 | load | 4 | `abs 0xe77e8` | `-` |
| `ldr` at 0xe7788 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe778c | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe77b0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe77b4 | load | 4 | `abs 0xe77e0` | `-` |
| `ldr` at 0xe77b8 | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe77c4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xe77c8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe77cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe77cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe77cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe73d8 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x1d4` | `0x0` |
| `bl` at 0xe7448 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x1dd` | `0x0` |
| `bl` at 0xe7534 | `(unknown callee) 0x21e28` | `0x6` | `0x135958` | `0x1e2` | `0x0` |
| `bl` at 0xe7544 | `(unknown callee) 0xe6310` | `[0x2ad26c]` | `?` | `?` | `0x2ad26c` |
| `bl` at 0xe76e8 | `(unknown callee) 0xeb1d0` | `0x3` | `?` | `?` | `?` |
| `bl` at 0xe7750 | `(unknown callee) 0xe6a88` | `[0x2ad26c]` | `?` | `?` | `0x2ad26c` |
| `bl` at 0xe77bc | `(unknown callee) 0xe974c` | `[0x2ad26c]` | `0x14` | `?` | `0x2ad26c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe73bc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0xe7410 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0xe7458 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0xe7488 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe7494 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe74cc | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe7550 | `ret(0xe6310) != 0x0` | `ret(0xe6310) == 0x0` |
| `blo` at 0xe75b0 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe769c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe76e0 | `? != 0x8` | `? == 0x8` |
| `beq` at 0xe76f4 | `ret(0xeb1d0) == 0x0` | `ret(0xeb1d0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe77cc | `r0` | `?` |

## `JChip_link_summary_f`

- `JChip_link_summary_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf3d60 to 0xf4250.
  Established. The known-edge call to `0xf39c4` uses `[sp-20]`, `[sp-24]`, `0x0`, unknown (`bl` at
  0xf3e30); its signed result takes the continuation for at least `0x0` and falls through when
  negative (`bge` at 0xf3e44). A computed transfer later has unresolved taken destinations (`ldrls`
  at 0xf3e88). The byte store at entry `r3 +0x0` writes `0x0` (`strb` at 0xf3db0). Summary
  interpretation and the reported known-edge r0 are unknown (`ldmdb` at 0xf421c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf3d70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf3d74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf3d78 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf3d7c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xf3d84 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xf3d8c | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xf3d94 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xf3d98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf3d9c | store | 4 | `sp -0x64` | `r3` |
| `ldr` at 0xf3da0 | load | 4 | `abs 0xf4220` | `-` |
| `str` at 0xf3da4 | store | 4 | `sp -0x68` | `0x139784` |
| `ldr` at 0xf3da8 | load | 4 | `sp -0x64` | `-` |
| `strb` at 0xf3db0 | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0xf3db4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf3dc4 | store | 4 | `sp -0x74` | `0x0` |
| `str` at 0xf3dd4 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xf3dd8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3dec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3dfc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf3e08 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xf3e10 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf3e1c | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xf3e20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf3e24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf3e38 | store | 4 | `sp -0x4c` | `ret(0xf39c4)` |
| `ldr` at 0xf3e3c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf3e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3e4c | load | 4 | `abs 0xf4224` | `-` |
| `ldr` at 0xf3e54 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xf3e58 | store | 4 | `sp -0x74` | `[sp-76]` |
| `ldr` at 0xf3e60 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf3e68 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xf3e74 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xf3e88 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3ec0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xf3ec4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3ed4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3eec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf3ef0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xf3efc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3f14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf3f18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xf3f2c | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xf3f30 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf3f34 | store | 4 | `sp -0x88` | `[sp-60]` |
| `ldr` at 0xf3f38 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf3f3c | load | 4 | `abs 0xf4228` | `-` |
| `ldr` at 0xf3f40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf3f44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf3f4c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf3f5c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf3f64 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b22c))` |
| `ldr` at 0xf3f68 | load | 4 | `abs 0xf422c` | `-` |
| `str` at 0xf3f6c | store | 4 | `sp -0x68` | `0x1397d4` |
| `ldr` at 0xf3f74 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf3f7c | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xf3f84 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3f90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3f9c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf3fa0 | load | 4 | `abs 0xf4230` | `-` |
| `ldr` at 0xf3fa8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf3fb8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf3fc0 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b22c))` |
| `ldr` at 0xf3fc4 | load | 4 | `abs 0xf422c` | `-` |
| `str` at 0xf3fc8 | store | 4 | `sp -0x68` | `0x1397d4` |
| `str` at 0xf3fd0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xf3fd4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3fe4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf3ffc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf4000 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf4010 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xf4014 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xf402c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf4030 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf4040 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xf4044 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xf404c | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xf4054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4060 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf406c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf4078 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xf4088 | load | 4 | `abs 0xf4220` | `-` |
| `str` at 0xf408c | store | 4 | `sp -0x6c` | `0x139784` |
| `ldr` at 0xf4090 | load | 4 | `abs 0xf4220` | `-` |
| `str` at 0xf4094 | store | 4 | `sp -0x70` | `0x139784` |
| `ldr` at 0xf4098 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf40a4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xf40b0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xf40bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf40c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf40d4 | load | 4 | `abs 0xf4234` | `-` |
| `str` at 0xf40d8 | store | 4 | `sp -0x6c` | `0x1397e8` |
| `ldr` at 0xf40dc | load | 4 | `abs 0xf4238` | `-` |
| `str` at 0xf40e0 | store | 4 | `sp -0x70` | `0x1397f8` |
| `ldr` at 0xf40e4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xf40f0 | load | 4 | `abs 0xf423c` | `-` |
| `str` at 0xf40f4 | store | 4 | `sp -0x78` | `0x1397fc` |
| `ldr` at 0xf40fc | load | 4 | `abs 0xf4240` | `-` |
| `str` at 0xf4100 | store | 4 | `sp -0x78` | `0x13981c` |
| `ldr` at 0xf4104 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf4108 | store | 4 | `sp -0x88` | `[sp-44]` |
| `ldr` at 0xf410c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf4110 | store | 4 | `sp -0x84` | `[sp-40]` |
| `ldr` at 0xf4114 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf4118 | store | 4 | `sp -0x80` | `[sp-36]` |
| `ldr` at 0xf411c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4120 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xf4124 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xf4128 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0xf4130 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4140 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf4148 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b22c))` |
| `ldr` at 0xf414c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xf4158 | load | 4 | `abs 0xf4244` | `-` |
| `str` at 0xf415c | store | 4 | `sp -0x7c` | `0x139830` |
| `ldr` at 0xf4164 | load | 4 | `abs 0xf4248` | `-` |
| `str` at 0xf4168 | store | 4 | `sp -0x7c` | `0x13984c` |
| `ldr` at 0xf416c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf4170 | store | 4 | `sp -0x88` | `[sp-48]` |
| `ldr` at 0xf4174 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4178 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xf417c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf4180 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf4188 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4198 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf41a0 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b22c))` |
| `ldr` at 0xf41a4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf41a8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xf41b0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf41c0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf41c8 | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b22c))` |
| `ldr` at 0xf41d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf41dc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf41e8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf41ec | load | 4 | `abs 0xf424c` | `-` |
| `ldr` at 0xf41f4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xf4204 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xf420c | store | 4 | `sp -0x64` | `([sp-100] + ret(0x1b22c))` |
| `str` at 0xf4214 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0xf4218 | load | 4 | `sp -0x74` | `-` |
| `ldmdb` at 0xf421c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf421c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf421c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf3dcc | `(unknown callee) 0xe335c` | `r0` | `r1` | `r3` | `[sp-20]` |
| `bl` at 0xf3e30 | `(unknown callee) 0xf39c4` | `[sp-20]` | `[sp-24]` | `0x0` | `?` |
| `bl` at 0xf3e50 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x139788` | `?` | `ret(0xf39c4)` |
| `bl` at 0xf3e70 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `?` |
| `bl` at 0xf3f48 | `(unknown callee) 0x1b49c` | `[sp-100]` | `0x1397ac` | `[sp-68]` | `[sp-64]` |
| `bl` at 0xf3f50 | `(unknown callee) 0x1b22c` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf3fa4 | `(unknown callee) 0x1b49c` | `[sp-100]` | `0x1397d8` | `?` | `[sp-28]` |
| `bl` at 0xf3fac | `(unknown callee) 0x1b22c` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf412c | `(unknown callee) 0x1b49c` | `[sp-100]` | `[sp-120]` | `[sp-104]` | `?` |
| `bl` at 0xf4134 | `(unknown callee) 0x1b22c` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf4184 | `(unknown callee) 0x1b49c` | `([sp-100] + ret(0x1b22c))` | `[sp-124]` | `[sp-56]` | `[sp-52]` |
| `bl` at 0xf418c | `(unknown callee) 0x1b22c` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf41ac | `(unknown callee) 0x1b2d4` | `([sp-100] + ret(0x1b22c))` | `[sp-112]` | `ret(0x1b22c)` | `([sp-100] + ret(0x1b22c))` |
| `bl` at 0xf41b4 | `(unknown callee) 0x1b22c` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xf41f0 | `(unknown callee) 0x1b49c` | `[sp-100]` | `0x139858` | `?` | `[sp-56]` |
| `bl` at 0xf41f8 | `(unknown callee) 0x1b22c` | `[sp-100]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf3dbc | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `ble` at 0xf3de0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xf3e44 | `ret(0xf39c4) >= 0x0 (signed, 32-bit)` | `ret(0xf39c4) < 0x0 (signed, 32-bit)` |
| `ldrls` at 0xf3e88 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |
| `ble` at 0xf3ecc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xf3ef8 | `? != [sp-84]` | `? == [sp-84]` |
| `bne` at 0xf3f20 | `? != [sp-68]` | `? == [sp-68]` |
| `bne` at 0xf3f8c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xf3f98 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `ble` at 0xf3fdc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xf4008 | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xf4038 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xf405c | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xf4068 | `[sp-88] != 0x0` | `[sp-88] == 0x0` |
| `bne` at 0xf4074 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `bne` at 0xf4080 | `[sp-96] != 0x0` | `[sp-96] == 0x0` |
| `bne` at 0xf40a0 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `bne` at 0xf40ac | `[sp-96] != 0x0` | `[sp-96] == 0x0` |
| `bne` at 0xf40b8 | `[sp-88] != 0x0` | `[sp-88] == 0x0` |
| `bne` at 0xf40c4 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `bne` at 0xf40d0 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0xf40ec | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0xf4154 | `[sp-96] == 0x0` | `[sp-96] != 0x0` |
| `bne` at 0xf41d8 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `bne` at 0xf41e4 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf421c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf3e88).

## `JTAG_LLP_SC_f`

- `JTAG_LLP_SC_f`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf4250 to 0xf4c90.
  Established. The opening call to `0xeb47c` has unknown, unknown, `0x4`, unknown (`bl` at 0xf429c).
  The later call to `0xea118` carries `[sp-24]`, unknown, unknown, `[0x2ad25c]` (`bl` at 0xf42d4).
  The computed selector is taken for an unknown unsigned value at most `0x5` and falls through for
  greater, with its taken destinations unresolved (`ldrls` at 0xf4348). Link-control paths beyond
  those edges and the known-edge return value remain unknown (`ldmdb` at 0xf4c68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf4260 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf4268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4270 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xf4280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4288 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xf42a4 | store | 4 | `sp -0x18` | `ret(0xeb47c)` |
| `ldr` at 0xf42a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf42b4 | load | 4 | `abs 0xf4c6c` | `-` |
| `str` at 0xf42b8 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xf42c0 | load | 4 | `abs 0xf4c70` | `-` |
| `ldr` at 0xf42c4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xf42d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf42dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf42e0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xf42ec | load | 4 | `abs 0xf4c74` | `-` |
| `ldr` at 0xf42f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf42fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf4304 | store | 4 | `sp -0x2c` | `[sp-20]+16` |
| `ldr` at 0xf4308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf430c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xf4318 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf431c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf4320 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xf4330 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf4334 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0xf4348 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf4408 | load | 4 | `abs 0xf4c78` | `-` |
| `str` at 0xf440c | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xf4c64 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xf4c68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf4c68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf4c68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf429c | `(unknown callee) 0xeb47c` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xf42d4 | `(unknown callee) 0xea118` | `[sp-24]` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0xf42f8 | `(unknown callee) 0x23f2c` | `0x139874` | `ret(0xea118)` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf42b0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xf42cc | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0xf4348 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf4c68 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf4348).

## `llpcmd_to_tdr`

- `llpcmd_to_tdr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf4c90 to 0xf50e8.
  Established. Calls include `0xebed0` with `[sp-20]`, `[sp-24]`, `0x0`, `sp-36` and later
  `[sp-20]`, `[sp-24]`, `0x1`, `sp-36` (`bl` at 0xf4ce4, `bl` at 0xf4d78). One intervening word
  store writes `0x0` at an unknown target (`str` at 0xf4cf8). The final call to the same target has
  `[sp-20]`, `[sp-24]`, `0xc`, `sp-36` (`bl` at 0xf50e0). The return propagates `ret(0xebed0)`
  (`ldmdb` at 0xf50e4). Command-to-register encoding is not established by these argument
  candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf4ca0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf4ca4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf4ca8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf4cbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4cc4 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf4cd0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4cd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4cdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf4cf8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf4d0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4d14 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf4d20 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4d3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf4d4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4d54 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xf4d64 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4d6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4d70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4d94 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf4da0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4da8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4dac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4dd0 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf4dd8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4de0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4de4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4e00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4e08 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf4e14 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4e20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4e3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4e44 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf4e4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4e58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4e74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4e7c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf4e88 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4e94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4eb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4eb8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf4ec4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4ecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4ed0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4eec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4ef4 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf4efc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4f04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4f2c | load | 4 | `[sp-28] +0x14` | `-` |
| `str` at 0xf4f30 | store | 4 | `(unknown)` | `[[sp-28]+20]` |
| `ldr` at 0xf4f38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4f3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4f58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4f60 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf4f6c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4f74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4f78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4f94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4f9c | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf4fa8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4fb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4fb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4fd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf4fd8 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xf4fe8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf4ff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4ff4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5010 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5018 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xf5024 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf502c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf504c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5054 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xf5058 | store | 4 | `(unknown)` | `[[sp-28]+28]` |
| `ldr` at 0xf506c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5074 | load | 4 | `[sp-28] +0x20` | `-` |
| `str` at 0xf5078 | store | 4 | `(unknown)` | `[[sp-28]+32]` |
| `ldr` at 0xf5080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5084 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf50a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf50a8 | load | 4 | `[sp-28] +0x24` | `-` |
| `str` at 0xf50ac | store | 4 | `(unknown)` | `[[sp-28]+36]` |
| `ldr` at 0xf50c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf50c8 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0xf50cc | store | 4 | `(unknown)` | `[[sp-28]+40]` |
| `ldr` at 0xf50d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf50d8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xf50e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf50e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf50e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf4ce4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x0` | `sp-36` |
| `bl` at 0xf4d78 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xf4db4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xf4dec | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xf4e28 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xf4e60 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xf4e9c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xf4ed8 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xf4f10 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xf4f44 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xf4f80 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xf4fbc | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xf4ffc | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xf5038 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xf508c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xf50e0 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf50e4 | `r0` | `ret(0xebed0)` |

## `llpcmd_from_tdr`

- `llpcmd_from_tdr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf50e8 to 0xf5668.
  Established. The first call to `0xeb9a0` carries entry r0, entry r1, `0x0`, `sp-36` (`bl` at
  0xf5114). Two later word stores at `[sp-28] +0x4` and `[sp-28] +0x10` have unknown values (`str`
  at 0xf5154, `str` at 0xf51b8). A separate word store writes `[sp-40]` at `[sp-28] +0xc` (`str` at
  0xf51e8). The exit reports r0 `sp-16` (`ldmdb` at 0xf5664). No conditional branch is reported, and
  register-to-command field meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf50f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf50fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf5100 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf5108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf510c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf511c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5130 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5140 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5148 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf5154 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf515c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5160 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5178 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5184 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf518c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5194 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf51a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf51ac | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf51b8 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf51c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf51d4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf51dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf51e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf51e8 | store | 4 | `[sp-28] +0xc` | `[sp-40]` |
| `ldr` at 0xf51f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf51f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5218 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5228 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5230 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf523c | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf5244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5248 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf526c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5280 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf528c | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xf5294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf52a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf52bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf52cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf52d4 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf52e0 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf52e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf52ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf52fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5310 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf531c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5324 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xf5330 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xf5338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf533c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf534c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5378 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf5384 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xf538c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5390 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf53a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf53b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf53c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf53cc | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xf53d8 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xf53e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf53e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf53f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5408 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf541c | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5428 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf5430 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5458 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf545c | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0xf5464 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5478 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf548c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf549c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf54a4 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf54b0 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf54b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf54bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf54cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf54e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf54f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf54f8 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xf5504 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xf550c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5510 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf552c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5534 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xf554c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5560 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xf5568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf556c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf557c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5588 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5590 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xf55a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf55b8 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xf55c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf55c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf55d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf55e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf55ec | store | 4 | `[sp-28] +0x1c` | `?` |
| `ldr` at 0xf55f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5608 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf560c | store | 4 | `[sp-28] +0x20` | `?` |
| `ldr` at 0xf5614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf5618 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5628 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf563c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5640 | store | 4 | `[sp-28] +0x24` | `?` |
| `ldr` at 0xf5648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf565c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5660 | store | 4 | `[sp-28] +0x28` | `?` |
| `ldmdb` at 0xf5664 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf5664 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf5664 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf5114 | `(unknown callee) 0xeb9a0` | `r0` | `r1` | `0x0` | `sp-36` |
| `bl` at 0xf5168 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xf51fc | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xf5250 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xf52a0 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xf52f4 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xf5344 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xf5398 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xf53ec | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xf543c | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xf5470 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xf54c4 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xf5518 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xf5574 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xf55cc | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xf5620 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf5664 | `r0` | `sp-16` |

## `lbi_jtag_configure`

- `lbi_jtag_configure`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf5bb4 to 0xf62fc.
  Established. The first result test takes its continuation for `ret(0x1b124) != 0x0` and falls
  through on zero (`bne` at 0xf5c04). A word store later writes `0xa0000000` at `[sp-56] +0x0`
  (`str` at 0xf5cc8). The last call to `0x1b49c` has `[sp-32]`, `0x139aa8`, unknown, unknown (`bl`
  at 0xf62b4). The return reports r0 `[sp-72]` (`ldmdb` at 0xf62d0). Configuration stream meaning
  and physical JTAG effects are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf5bc4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf5bc8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf5bcc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf5bd0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf5bd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf5be0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf5be4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf5bf8 | store | 4 | `sp -0x38` | `ret(0x1b124)` |
| `ldr` at 0xf5bfc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf5c08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf5c0c | load | 4 | `abs 0xf62d4` | `-` |
| `str` at 0xf5c18 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf5c20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf5c28 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf5c2c | load | 4 | `abs 0xf62d8` | `-` |
| `ldr` at 0xf5c34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5c38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf5c44 | store | 4 | `sp -0x34` | `ret(0xf9ab8)` |
| `ldr` at 0xf5c48 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf5c58 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf5c64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5c6c | load | 4 | `abs 0xf62dc` | `-` |
| `ldr` at 0xf5c70 | load | 4 | `abs 0x144dc4` | `-` |
| `ldr` at 0xf5c78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5c7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf5c80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf5c94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5c98 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf5cb0 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf5cbc | store | 4 | `sp -0x28` | `0x6` |
| `ldr` at 0xf5cc0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf5cc8 | store | 4 | `[sp-56] +0x0` | `0xa0000000` |
| `str` at 0xf5cdc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf5ce0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf5ce4 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xf5ce8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf5cec | store | 4 | `sp -0x54` | `[sp-56]` |
| `str` at 0xf5cf4 | store | 4 | `sp -0x50` | `sp-64` |
| `ldr` at 0xf5cf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf5cfc | store | 4 | `sp -0x4c` | `[sp-32]` |
| `str` at 0xf5d24 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf5d44 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5d4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5d68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5d6c | load | 4 | `abs 0xf62e0` | `-` |
| `ldr` at 0xf5d84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5d8c | load | 4 | `abs 0xf62e0` | `-` |
| `str` at 0xf5d98 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5db4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5db8 | load | 4 | `abs 0xf62e4` | `-` |
| `ldr` at 0xf5dd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5dd8 | load | 4 | `abs 0xf62e4` | `-` |
| `str` at 0xf5de4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5e00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5e04 | load | 4 | `abs 0xf62e8` | `-` |
| `ldr` at 0xf5e1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5e24 | load | 4 | `abs 0xf62e8` | `-` |
| `str` at 0xf5e30 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5e4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5e68 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5e7c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5e98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5eb4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5ec8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5ed8 | load | 4 | `abs 0xf62ec` | `-` |
| `ldr` at 0xf5edc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5ee4 | load | 4 | `abs 0xf62f0` | `-` |
| `str` at 0xf5ef0 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0xf5ef4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5ef8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5efc | load | 4 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xf5f00 | load | 4 | `[sp-52] +0x18` | `-` |
| `ldr` at 0xf5f10 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5f14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf5f18 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf5f20 | store | 4 | `sp -0x30` | `ret(0xfa1d8)` |
| `ldr` at 0xf5f24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf5f30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5f34 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf5f4c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf5f54 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf5f5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf5f60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf5f64 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xf5f68 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf5f6c | store | 4 | `sp -0x54` | `[sp-56]` |
| `ldr` at 0xf5f70 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf5f74 | store | 4 | `sp -0x50` | `[sp-56]` |
| `ldr` at 0xf5f78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf5f7c | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf5f84 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xf5fa4 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf5fb0 | store | 4 | `sp -0x44` | `0x6` |
| `ldr` at 0xf5fc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf5fd8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf5fdc | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf5ff4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf5ffc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6000 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6004 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6018 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf601c | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf6034 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf603c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6040 | load | 4 | `[sp-52] +0x1c` | `-` |
| `str` at 0xf6058 | store | 4 | `sp -0x28` | `0x6` |
| `ldr` at 0xf605c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6064 | store | 4 | `[sp-56] +0x0` | `0xfc000000` |
| `str` at 0xf6078 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf607c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf6080 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xf6084 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6088 | store | 4 | `sp -0x54` | `[sp-56]` |
| `str` at 0xf6090 | store | 4 | `sp -0x50` | `sp-64` |
| `ldr` at 0xf6094 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6098 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `str` at 0xf60c0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf60e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf60e8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6104 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6108 | load | 4 | `abs 0xf62e0` | `-` |
| `ldr` at 0xf6120 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6128 | load | 4 | `abs 0xf62e0` | `-` |
| `str` at 0xf6134 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6150 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6154 | load | 4 | `abs 0xf62e4` | `-` |
| `ldr` at 0xf616c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6174 | load | 4 | `abs 0xf62e4` | `-` |
| `str` at 0xf6180 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf619c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf61a0 | load | 4 | `abs 0xf62e8` | `-` |
| `ldr` at 0xf61b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf61c0 | load | 4 | `abs 0xf62e8` | `-` |
| `str` at 0xf61cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf61e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6204 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6218 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6234 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6250 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6264 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf6274 | load | 4 | `abs 0xf62f4` | `-` |
| `ldr` at 0xf6278 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf628c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf62a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf62ac | load | 4 | `abs 0xf62f8` | `-` |
| `ldr` at 0xf62b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf62bc | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf62c8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xf62cc | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xf62d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf62d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf62d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf5bf0 | `(unknown callee) 0x1b124` | `?` | `r1` | `r2` | `?` |
| `bl` at 0xf5c10 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x1399d0` | `?` | `ret(0x1b124)` |
| `bl` at 0xf5c30 | `(unknown callee) 0x23f2c` | `0x139a00` | `?` | `?` | `?` |
| `bl` at 0xf5c3c | `(unknown callee) 0xf9ab8` | `[sp-24]` | `[sp-32]` | `?` | `?` |
| `bl` at 0xf5c68 | `(unknown callee) 0xfa534` | `0x0` | `ret(0xf9ab8)` | `?` | `ret(0xf9ab8)` |
| `bl` at 0xf5c74 | `(unknown callee) 0x1b01c` | `[0x144dc4]` | `?` | `?` | `0x144dc4` |
| `bl` at 0xf5c84 | `(unknown callee) 0xf62fc` | `[sp-52]` | `[sp-56]` | `[sp-32]` | `?` |
| `bl` at 0xf5c9c | `(unknown callee) 0x1b310` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf5ca8 | `(unknown callee) 0xfa450` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf5d10 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf5ee0 | `(unknown callee) 0x23f2c` | `0x139a28` | `?` | `?` | `?` |
| `bl` at 0xf5ee8 | `(unknown callee) 0x23f2c` | `0x139a4c` | `?` | `?` | `?` |
| `bl` at 0xf5f1c | `(unknown callee) 0xfa1d8` | `[sp-52]` | `[sp-44]` | `[sp-56]` | `[[sp-52]+24]` |
| `bl` at 0xf5f38 | `(unknown callee) 0x1b310` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf5f44 | `(unknown callee) 0xfa450` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf5f90 | `(unknown callee) 0xdcf54` | `0x1` | `[sp-68]` | `0x6` | `0x0` |
| `bl` at 0xf5fc8 | `(unknown callee) 0xdcb88` | `0x6` | `0x0` | `0x0` | `[sp-32]` |
| `bl` at 0xf5fe0 | `(unknown callee) 0x1b310` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf5fec | `(unknown callee) 0xfa450` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf6008 | `(unknown callee) 0xf69b8` | `[sp-52]` | `[sp-56]` | `[sp-32]` | `ret(0xdcb88)` |
| `bl` at 0xf6020 | `(unknown callee) 0x1b310` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf602c | `(unknown callee) 0xfa450` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf6044 | `(unknown callee) 0x1b310` | `[[sp-52]+28]` | `?` | `?` | `[sp-52]` |
| `bl` at 0xf6050 | `(unknown callee) 0xfa450` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xf60ac | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf627c | `(unknown callee) 0x23f2c` | `0x139a84` | `?` | `?` | `?` |
| `bl` at 0xf62b4 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x139aa8` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf5c04 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xf5c50 | `ret(0xf9ab8) != 0x0` | `ret(0xf9ab8) == 0x0` |
| `beq` at 0xf5c90 | `ret(0xf62fc) == 0x0` | `ret(0xf62fc) != 0x0` |
| `bne` at 0xf5d1c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blt` at 0xf5f08 | `[[sp-52]+32] < [[sp-52]+24] (signed, 32-bit)` | `[[sp-52]+32] >= [[sp-52]+24] (signed, 32-bit)` |
| `bne` at 0xf5f2c | `ret(0xfa1d8) != 0x0` | `ret(0xfa1d8) == 0x0` |
| `bne` at 0xf5f9c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf5fd4 | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `beq` at 0xf6014 | `ret(0xf69b8) == 0x0` | `ret(0xf69b8) != 0x0` |
| `bne` at 0xf60b8 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xf6298 | `? == 0x25` | `? != 0x25` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf62d0 | `r0` | `[sp-72]` |

## `lbi_configure_pre_commands`

- `lbi_configure_pre_commands`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf62fc to 0xf69b8.
  Established. Word stores include `0x90000000` at `[sp-24] +0x0`, then `0xa800ffea` at that printed
  target and `0xabcdef01` at `[sp-24] +0x4` (`str` at 0xf6350, `str` at 0xf63f4, `str` at 0xf6408).
  The first call to `0xdcf54` has `0x0`, `0x1`, `0x1`, `0x0` (`bl` at 0xf638c); its nonzero result
  takes the continuation and zero falls through (`bne` at 0xf6398). The exit supplies r0 `[sp-60]`
  (`ldmdb` at 0xf697c). Device command meanings and timing units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf630c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf6310 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf6314 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf6320 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6338 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6344 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6348 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6350 | store | 4 | `[sp-24] +0x0` | `0x90000000` |
| `str` at 0xf6358 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf635c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6360 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6364 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6368 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6370 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6374 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6378 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf63a0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf63a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf63b0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf63b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf63c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf63c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf63cc | load | 4 | `abs 0xf6980` | `-` |
| `ldr` at 0xf63d0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf63dc | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf63e8 | store | 4 | `sp -0x20` | `0x40` |
| `ldr` at 0xf63ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf63f0 | load | 4 | `abs 0xf6984` | `-` |
| `str` at 0xf63f4 | store | 4 | `[sp-24] +0x0` | `0xa800ffea` |
| `ldr` at 0xf63fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6404 | load | 4 | `abs 0xf6988` | `-` |
| `str` at 0xf6408 | store | 4 | `[sp-24] +0x4` | `0xabcdef01` |
| `str` at 0xf6410 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xf6418 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xf641c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6420 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6424 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6428 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6430 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6434 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6438 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6460 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6468 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf646c | load | 4 | `abs 0xf698c` | `-` |
| `ldr` at 0xf6478 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6480 | load | 4 | `abs 0xf698c` | `-` |
| `str` at 0xf648c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6490 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6494 | load | 4 | `abs 0xf6990` | `-` |
| `ldr` at 0xf64a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf64a8 | load | 4 | `abs 0xf6990` | `-` |
| `str` at 0xf64b4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf64b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf64bc | load | 4 | `abs 0xf6994` | `-` |
| `ldr` at 0xf64c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf64d0 | load | 4 | `abs 0xf6994` | `-` |
| `str` at 0xf64dc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf64e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf64f0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6504 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6508 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6518 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf652c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf6530 | load | 4 | `abs 0xf6998` | `-` |
| `ldr` at 0xf6534 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf653c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf6540 | load | 4 | `abs 0xf699c` | `-` |
| `ldr` at 0xf6548 | load | 4 | `abs 0xf69a0` | `-` |
| `ldr` at 0xf6554 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6558 | load | 4 | `abs 0xf69a4` | `-` |
| `ldr` at 0xf655c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf6568 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6570 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf6574 | load | 4 | `abs 0xf6984` | `-` |
| `ldr` at 0xf6580 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6584 | load | 4 | `abs 0xf69a8` | `-` |
| `ldr` at 0xf6588 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf6594 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf65a0 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf65a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf65ac | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf65b4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf65b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf65bc | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf65c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf65c4 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf65cc | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf65d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf65d4 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf65fc | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6608 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf660c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6614 | store | 4 | `[sp-24] +0x0` | `0xd0000000` |
| `str` at 0xf661c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6620 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6624 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6628 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf662c | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6634 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6638 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf663c | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6664 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6674 | load | 4 | `abs 0xf69ac` | `-` |
| `ldr` at 0xf6678 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6690 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf669c | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf66a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf66a8 | store | 4 | `[sp-24] +0x0` | `0xa0000000` |
| `str` at 0xf66b0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf66b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf66b8 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf66bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf66c0 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf66c8 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf66cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf66d0 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf66f8 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6704 | store | 4 | `sp -0x20` | `0xc0` |
| `ldr` at 0xf6708 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6710 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xf6718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6720 | load | 4 | `abs 0xf69b0` | `-` |
| `str` at 0xf6724 | store | 4 | `[sp-24] +0x4` | `0xaa995566` |
| `ldr` at 0xf672c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6734 | load | 4 | `abs 0xf69b4` | `-` |
| `str` at 0xf6738 | store | 4 | `[sp-24] +0x8` | `0x30008001` |
| `ldr` at 0xf6740 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf674c | store | 4 | `[sp-24] +0xc` | `0x7` |
| `ldr` at 0xf6754 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6760 | store | 4 | `[sp-24] +0x10` | `0x0` |
| `ldr` at 0xf6768 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6774 | store | 4 | `[sp-24] +0x14` | `0x0` |
| `ldr` at 0xf6778 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf677c | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6780 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6784 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf678c | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6790 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6794 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf67bc | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf67c8 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf67cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf67d4 | store | 4 | `[sp-24] +0x0` | `0xb0000000` |
| `str` at 0xf67dc | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf67e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf67e4 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf67e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf67ec | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf67f4 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf67f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf67fc | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6824 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6838 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6850 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6860 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6878 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6884 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6888 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6890 | store | 4 | `[sp-24] +0x0` | `0xa0000000` |
| `str` at 0xf6898 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf689c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf68a0 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf68a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf68a8 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf68b0 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf68b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf68b8 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf68e0 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf68ec | store | 4 | `sp -0x20` | `0x60` |
| `ldr` at 0xf68f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf68f4 | load | 4 | `abs 0xf69b0` | `-` |
| `str` at 0xf68f8 | store | 4 | `[sp-24] +0x0` | `0xaa995566` |
| `ldr` at 0xf6900 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf690c | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xf6914 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6920 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0xf6924 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6928 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf692c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6930 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6938 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf693c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6940 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6968 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6974 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xf6978 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xf697c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf697c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf697c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf6324 | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `r2` | `r3` |
| `bl` at 0xf638c | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf63d4 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x139ad4` | `[[sp-24]]` | `[sp-24]` |
| `bl` at 0xf644c | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6538 | `(unknown callee) 0x23f2c` | `0x139b00` | `[sp-56]` | `?` | `?` |
| `bl` at 0xf6560 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x139978` | `[sp-56]` | `?` |
| `bl` at 0xf658c | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x1399a4` | `[sp-52]` | `0xa800ffea` |
| `bl` at 0xf65e8 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6650 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf667c | `(unknown callee) 0xdcb88` | `0x1` | `0x0` | `0x2710` | `[sp-28]` |
| `bl` at 0xf66e4 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf67a8 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6810 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf683c | `(unknown callee) 0xdcb88` | `0x1` | `0x0` | `0xc` | `[sp-28]` |
| `bl` at 0xf6864 | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdcb88)` |
| `bl` at 0xf68cc | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6954 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf6330 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf6398 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xf63c0 | `? == 0x80000000` | `? != 0x80000000` |
| `bne` at 0xf6458 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xf6550 | `? == 0x1400093` | `? != 0x1400093` |
| `beq` at 0xf657c | `[sp-52] == 0xa800ffea` | `[sp-52] != 0xa800ffea` |
| `bne` at 0xf65f4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf665c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6688 | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `bne` at 0xf66f0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf67b4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf681c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6848 | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `bne` at 0xf6870 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf68d8 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6960 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf697c | `r0` | `[sp-60]` |

## `lbi_configure_post_commands`

- `lbi_configure_post_commands`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf69b8 to 0xf6ce4.
  Established. The initial two calls to `0xdca64` use `0x1`, `0x0`, entry r2, entry r3 and `0x1`,
  `0x0`, `[sp-28]`, `ret(0xdca64)`, respectively (`bl` at 0xf69e0, `bl` at 0xf6a08). Later word
  stores write `0x30000000` and `0xfc000000` at `[sp-24] +0x0` (`str` at 0xf6a34, `str` at 0xf6ac8).
  The final call to `0xdcf54` carries `0x0`, `0x1`, `0x1`, `0x0` (`bl` at 0xf6cb8). The exit reports
  r0 `[sp-60]` (`ldmdb` at 0xf6ce0). Post-configuration protocol meaning is unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf69c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf69cc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf69d0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf69dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf69f4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6a04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6a1c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6a28 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6a2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6a34 | store | 4 | `[sp-24] +0x0` | `0x30000000` |
| `str` at 0xf6a3c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6a40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6a44 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6a48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6a4c | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6a54 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6a58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6a5c | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6a84 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6a98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6ab0 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6abc | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6ac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6ac8 | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf6ad0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6ad4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6ad8 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6adc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6ae0 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6ae8 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6aec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6af0 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6b18 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6b24 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6b28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6b30 | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf6b38 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6b3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6b40 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6b44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6b48 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6b50 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6b54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6b58 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6b80 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6b90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6ba8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6bb8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6bd0 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6bdc | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6be0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6be8 | store | 4 | `[sp-24] +0x0` | `0x30000000` |
| `str` at 0xf6bf0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6bf4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6bf8 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6bfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6c00 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6c08 | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6c0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6c10 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6c38 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf6c4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6c64 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6c70 | store | 4 | `sp -0x20` | `0x6` |
| `ldr` at 0xf6c74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6c7c | store | 4 | `[sp-24] +0x0` | `0xfc000000` |
| `str` at 0xf6c84 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf6c88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6c8c | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xf6c90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6c94 | store | 4 | `sp -0x48` | `[sp-24]` |
| `str` at 0xf6c9c | store | 4 | `sp -0x44` | `sp-56` |
| `ldr` at 0xf6ca0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6ca4 | store | 4 | `sp -0x40` | `[sp-28]` |
| `str` at 0xf6ccc | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf6cd8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xf6cdc | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xf6ce0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf6ce0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf6ce0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf69e0 | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `r2` | `r3` |
| `bl` at 0xf6a08 | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdca64)` |
| `bl` at 0xf6a70 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6a9c | `(unknown callee) 0xdcb88` | `0x1` | `0x0` | `0xc` | `[sp-28]` |
| `bl` at 0xf6b04 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6b6c | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6b94 | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdcf54)` |
| `bl` at 0xf6bbc | `(unknown callee) 0xdca64` | `0x1` | `0x0` | `[sp-28]` | `ret(0xdca64)` |
| `bl` at 0xf6c24 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xf6c50 | `(unknown callee) 0xdcb88` | `0x1` | `0x0` | `0xc` | `[sp-28]` |
| `bl` at 0xf6cb8 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf69ec | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf6a14 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf6a7c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6aa8 | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `bne` at 0xf6b10 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6b78 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6ba0 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf6bc8 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf6c30 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xf6c5c | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `bne` at 0xf6cc4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf6ce0 | `r0` | `[sp-60]` |

## `setup_tdrdata_shub`

- `setup_tdrdata_shub`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf191c to 0xf1f24.
  Established. The call to `0x1b124` uses `0x3c` and three unknown candidates (`bl` at 0xf1948); its
  result is written as a word at `[sp-24] +0x28` (`str` at 0xf1950). The final call to `0xf07c8`
  carries `0x1377fc`, `0x63`, `0x144a3c`, unknown (`bl` at 0xf1e54). The exit retains r0 `[sp-36]`
  (`ldmdb` at 0xf1eb4). SHUB data-register identities and record ownership are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf192c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf1934 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1938 | load | 4 | `abs 0xf1eb8` | `-` |
| `ldr` at 0xf1940 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1950 | store | 4 | `[sp-24] +0x28` | `ret(0x1b124)` |
| `ldr` at 0xf1954 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1964 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xf1970 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf1974 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1988 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1990 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf199c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf19a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf19a8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf19b0 | load | 4 | `abs 0xf1ebc` | `-` |
| `ldr` at 0xf19b8 | load | 4 | `abs 0xf1ec0` | `-` |
| `ldr` at 0xf19c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf19cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf19d8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf19e0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf19e8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf19ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xf19fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1a04 | load | 4 | `abs 0xf1ec4` | `-` |
| `ldr` at 0xf1a0c | load | 4 | `abs 0xf1ec8` | `-` |
| `ldr` at 0xf1a18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1a20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1a2c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1a34 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1a3c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1a40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1a50 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1a58 | load | 4 | `abs 0xf1ecc` | `-` |
| `ldr` at 0xf1a60 | load | 4 | `abs 0xf1ed0` | `-` |
| `ldr` at 0xf1a6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1a74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1a80 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1a88 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1a90 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1a94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1aa4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1aac | load | 4 | `abs 0xf1ed4` | `-` |
| `ldr` at 0xf1ab4 | load | 4 | `abs 0xf1ed8` | `-` |
| `ldr` at 0xf1ac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1ac8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1ad4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1adc | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1ae4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1ae8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1af8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1b00 | load | 4 | `abs 0xf1edc` | `-` |
| `ldr` at 0xf1b08 | load | 4 | `abs 0xf1ee0` | `-` |
| `ldr` at 0xf1b14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1b1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1b28 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1b30 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1b38 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1b3c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1b4c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1b54 | load | 4 | `abs 0xf1ee4` | `-` |
| `ldr` at 0xf1b5c | load | 4 | `abs 0xf1ee8` | `-` |
| `ldr` at 0xf1b68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1b70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1b7c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1b84 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1b8c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1b90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1ba0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1ba8 | load | 4 | `abs 0xf1eec` | `-` |
| `ldr` at 0xf1bb0 | load | 4 | `abs 0xf1ef0` | `-` |
| `ldr` at 0xf1bbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1bc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1bd0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1bd8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1be0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1be4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1bf4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1bfc | load | 4 | `abs 0xf1ef4` | `-` |
| `ldr` at 0xf1c04 | load | 4 | `abs 0xf1ef8` | `-` |
| `ldr` at 0xf1c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1c18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1c24 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1c2c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1c34 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1c38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1c48 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1c50 | load | 4 | `abs 0xf1efc` | `-` |
| `ldr` at 0xf1c58 | load | 4 | `abs 0xf1ef8` | `-` |
| `ldr` at 0xf1c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1c6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1c78 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1c80 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1c88 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1c8c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1c9c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1ca4 | load | 4 | `abs 0xf1f00` | `-` |
| `ldr` at 0xf1cac | load | 4 | `abs 0xf1f04` | `-` |
| `ldr` at 0xf1cb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1cc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1ccc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1cd4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1cdc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1ce0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1cf0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1cf8 | load | 4 | `abs 0xf1f08` | `-` |
| `ldr` at 0xf1d00 | load | 4 | `abs 0xf1f04` | `-` |
| `ldr` at 0xf1d0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1d14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1d20 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1d28 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1d30 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1d34 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1d44 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1d4c | load | 4 | `abs 0xf1f0c` | `-` |
| `ldr` at 0xf1d54 | load | 4 | `abs 0xf1f10` | `-` |
| `ldr` at 0xf1d60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1d68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1d74 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1d7c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1d84 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1d88 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1d98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1da0 | load | 4 | `abs 0xf1f14` | `-` |
| `ldr` at 0xf1da8 | load | 4 | `abs 0xf1f10` | `-` |
| `ldr` at 0xf1db4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1dbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1dc8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1dd0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1dd8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1ddc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1dec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1df4 | load | 4 | `abs 0xf1f18` | `-` |
| `ldr` at 0xf1dfc | load | 4 | `abs 0xf1f1c` | `-` |
| `ldr` at 0xf1e08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1e10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1e1c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1e24 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1e2c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1e30 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1e40 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1e48 | load | 4 | `abs 0xf1f20` | `-` |
| `ldr` at 0xf1e50 | load | 4 | `abs 0xf1f1c` | `-` |
| `ldr` at 0xf1e5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1e64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1e70 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1e78 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1e80 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1e84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1e94 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf1e9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1ea0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1ea4 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf1eac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf1eb0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf1eb4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf1eb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1eb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1eb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf193c | `(unknown callee) 0x23f2c` | `0x137738` | `r1` | `r2` | `0x0` |
| `bl` at 0xf1948 | `(unknown callee) 0x1b124` | `0x3c` | `?` | `?` | `?` |
| `bl` at 0xf19bc | `(unknown callee) 0xf07c8` | `0x137750` | `0x7c` | `0x144878` | `?` |
| `bl` at 0xf1a10 | `(unknown callee) 0xf07c8` | `0x13775c` | `0x7d` | `0x144890` | `?` |
| `bl` at 0xf1a64 | `(unknown callee) 0xf07c8` | `0x137768` | `0x7f` | `0x1448a8` | `?` |
| `bl` at 0xf1ab8 | `(unknown callee) 0xf07c8` | `0x137770` | `0x7e` | `0x142c38` | `?` |
| `bl` at 0xf1b0c | `(unknown callee) 0xf07c8` | `0x137778` | `0x7` | `0x142cc8` | `?` |
| `bl` at 0xf1b60 | `(unknown callee) 0xf07c8` | `0x137780` | `0x5` | `0x142d04` | `?` |
| `bl` at 0xf1bb4 | `(unknown callee) 0xf07c8` | `0x13778c` | `0x6` | `0x142ec0` | `?` |
| `bl` at 0xf1c08 | `(unknown callee) 0xf07c8` | `0x137794` | `0xa` | `0x142f68` | `?` |
| `bl` at 0xf1c5c | `(unknown callee) 0xf07c8` | `0x1377a0` | `0xb` | `0x142f68` | `?` |
| `bl` at 0xf1cb0 | `(unknown callee) 0xf07c8` | `0x1377ac` | `0x1a` | `0x1449c4` | `?` |
| `bl` at 0xf1d04 | `(unknown callee) 0xf07c8` | `0x1377bc` | `0x1b` | `0x1449c4` | `?` |
| `bl` at 0xf1d58 | `(unknown callee) 0xf07c8` | `0x1377cc` | `0x60` | `0x142fe0` | `?` |
| `bl` at 0xf1dac | `(unknown callee) 0xf07c8` | `0x1377dc` | `0x62` | `0x142fe0` | `?` |
| `bl` at 0xf1e00 | `(unknown callee) 0xf07c8` | `0x1377ec` | `0x61` | `0x144a3c` | `?` |
| `bl` at 0xf1e54 | `(unknown callee) 0xf07c8` | `0x1377fc` | `0x63` | `0x144a3c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf195c | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf197c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf19f4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1a48 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1a9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1af0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1b44 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1b98 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1bec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1c40 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1c94 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1ce8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1d3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1d90 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1de4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1e38 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf1e8c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1eb4 | `r0` | `[sp-36]` |

## `setup_tdrdata_tio`

- `setup_tdrdata_tio`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1f24 to 0xf25e4.
  Established. The word store at `[sp-24] +0x28` carries `ret(0x1b124)` (`str` at 0xf1f58); its
  pointer test takes the continuation for `[[sp-24]+40] != 0x0` and falls through on zero (`bne` at
  0xf1f64). The last call to `0xf07c8` has `0x137ee0`, `0x63`, `0x144a3c`, unknown (`bl` at
  0xf2504). The return supplies r0 `[sp-36]` (`ldmdb` at 0xf2564). TIO register meaning and
  constructor ownership remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1f34 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf1f3c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1f40 | load | 4 | `abs 0xf2568` | `-` |
| `ldr` at 0xf1f48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1f58 | store | 4 | `[sp-24] +0x28` | `ret(0x1b124)` |
| `ldr` at 0xf1f5c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1f6c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xf1f78 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf1f7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1f90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1f98 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1fa4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf1fa8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1fb0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf1fb8 | load | 4 | `abs 0xf256c` | `-` |
| `ldr` at 0xf1fc0 | load | 4 | `abs 0xf2570` | `-` |
| `ldr` at 0xf1fcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1fd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1fe0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf1fe8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf1ff0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1ff4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2004 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf200c | load | 4 | `abs 0xf2574` | `-` |
| `ldr` at 0xf2014 | load | 4 | `abs 0xf2578` | `-` |
| `ldr` at 0xf2020 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2028 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2034 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf203c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2044 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2048 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2058 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2060 | load | 4 | `abs 0xf257c` | `-` |
| `ldr` at 0xf2068 | load | 4 | `abs 0xf2580` | `-` |
| `ldr` at 0xf2074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf207c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2088 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2090 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2098 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf209c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf20ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf20b4 | load | 4 | `abs 0xf2584` | `-` |
| `ldr` at 0xf20bc | load | 4 | `abs 0xf2588` | `-` |
| `ldr` at 0xf20c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf20d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf20dc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf20e4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf20ec | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf20f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2100 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2108 | load | 4 | `abs 0xf258c` | `-` |
| `ldr` at 0xf2110 | load | 4 | `abs 0xf2590` | `-` |
| `ldr` at 0xf211c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2124 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2130 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2138 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2140 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2144 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2154 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf215c | load | 4 | `abs 0xf2594` | `-` |
| `ldr` at 0xf2164 | load | 4 | `abs 0xf2598` | `-` |
| `ldr` at 0xf2170 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2178 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2184 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf218c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2194 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2198 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf21a8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf21b0 | load | 4 | `abs 0xf259c` | `-` |
| `ldr` at 0xf21b8 | load | 4 | `abs 0xf25a0` | `-` |
| `ldr` at 0xf21c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf21cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf21d8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf21e0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf21e8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf21ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xf21fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2204 | load | 4 | `abs 0xf25a4` | `-` |
| `ldr` at 0xf220c | load | 4 | `abs 0xf25a8` | `-` |
| `ldr` at 0xf2218 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2220 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf222c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2234 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf223c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2240 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2250 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2258 | load | 4 | `abs 0xf25ac` | `-` |
| `ldr` at 0xf2260 | load | 4 | `abs 0xf25b0` | `-` |
| `ldr` at 0xf226c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2274 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2280 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2288 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2290 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2294 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf22a4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf22ac | load | 4 | `abs 0xf25b4` | `-` |
| `ldr` at 0xf22b4 | load | 4 | `abs 0xf25b8` | `-` |
| `ldr` at 0xf22c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf22c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf22d4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf22dc | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf22e4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf22e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf22f8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2300 | load | 4 | `abs 0xf25bc` | `-` |
| `ldr` at 0xf2308 | load | 4 | `abs 0xf25b8` | `-` |
| `ldr` at 0xf2314 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf231c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2328 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2330 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2338 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf233c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf234c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2354 | load | 4 | `abs 0xf25c0` | `-` |
| `ldr` at 0xf235c | load | 4 | `abs 0xf25c4` | `-` |
| `ldr` at 0xf2368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2370 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf237c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2384 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf238c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2390 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf23a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf23a8 | load | 4 | `abs 0xf25c8` | `-` |
| `ldr` at 0xf23b0 | load | 4 | `abs 0xf25c4` | `-` |
| `ldr` at 0xf23bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf23c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf23d0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf23d8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf23e0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf23e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf23f4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf23fc | load | 4 | `abs 0xf25cc` | `-` |
| `ldr` at 0xf2404 | load | 4 | `abs 0xf25d0` | `-` |
| `ldr` at 0xf2410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2418 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2424 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf242c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2434 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2438 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2448 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2450 | load | 4 | `abs 0xf25d4` | `-` |
| `ldr` at 0xf2458 | load | 4 | `abs 0xf25d0` | `-` |
| `ldr` at 0xf2464 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf246c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2478 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2480 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2488 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf248c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf249c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf24a4 | load | 4 | `abs 0xf25d8` | `-` |
| `ldr` at 0xf24ac | load | 4 | `abs 0xf25dc` | `-` |
| `ldr` at 0xf24b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf24c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf24cc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf24d4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf24dc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf24e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf24f0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf24f8 | load | 4 | `abs 0xf25e0` | `-` |
| `ldr` at 0xf2500 | load | 4 | `abs 0xf25dc` | `-` |
| `ldr` at 0xf250c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2514 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2520 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2528 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2530 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2534 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2544 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf254c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2550 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf2554 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf255c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf2560 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf2564 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf2564 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf2564 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf2564 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1f44 | `(unknown callee) 0x23f2c` | `0x137dfc` | `r1` | `r2` | `0x0` |
| `bl` at 0xf1f50 | `(unknown callee) 0x1b124` | `0x44` | `?` | `?` | `?` |
| `bl` at 0xf1fc4 | `(unknown callee) 0xf07c8` | `0x137e14` | `0x7c` | `0x144878` | `?` |
| `bl` at 0xf2018 | `(unknown callee) 0xf07c8` | `0x137e20` | `0x7d` | `0x144890` | `?` |
| `bl` at 0xf206c | `(unknown callee) 0xf07c8` | `0x137e2c` | `0x7f` | `0x1448a8` | `?` |
| `bl` at 0xf20c0 | `(unknown callee) 0xf07c8` | `0x137e34` | `0x7e` | `0x1430a4` | `?` |
| `bl` at 0xf2114 | `(unknown callee) 0xf07c8` | `0x137e3c` | `0x7` | `0x143170` | `?` |
| `bl` at 0xf2168 | `(unknown callee) 0xf07c8` | `0x137e44` | `0x5` | `0x1431ac` | `?` |
| `bl` at 0xf21bc | `(unknown callee) 0xf07c8` | `0x137e50` | `0x6` | `0x143390` | `?` |
| `bl` at 0xf2210 | `(unknown callee) 0xf07c8` | `0x137e58` | `0x12` | `0x1448c0` | `?` |
| `bl` at 0xf2264 | `(unknown callee) 0xf07c8` | `0x137e68` | `0x11` | `0x1448d8` | `?` |
| `bl` at 0xf22b8 | `(unknown callee) 0xf07c8` | `0x137e78` | `0xa` | `0x143470` | `?` |
| `bl` at 0xf230c | `(unknown callee) 0xf07c8` | `0x137e84` | `0xb` | `0x143470` | `?` |
| `bl` at 0xf2360 | `(unknown callee) 0xf07c8` | `0x137e90` | `0x1a` | `0x1449c4` | `?` |
| `bl` at 0xf23b4 | `(unknown callee) 0xf07c8` | `0x137ea0` | `0x1b` | `0x1449c4` | `?` |
| `bl` at 0xf2408 | `(unknown callee) 0xf07c8` | `0x137eb0` | `0x60` | `0x143510` | `?` |
| `bl` at 0xf245c | `(unknown callee) 0xf07c8` | `0x137ec0` | `0x62` | `0x143510` | `?` |
| `bl` at 0xf24b0 | `(unknown callee) 0xf07c8` | `0x137ed0` | `0x61` | `0x144a3c` | `?` |
| `bl` at 0xf2504 | `(unknown callee) 0xf07c8` | `0x137ee0` | `0x63` | `0x144a3c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf1f64 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf1f84 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf1ffc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2050 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf20a4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf20f8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf214c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf21a0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf21f4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2248 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf229c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf22f0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2344 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2398 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf23ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2440 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2494 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf24e8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf253c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf2564 | `r0` | `[sp-36]` |

## `setup_tdrdata_nl4r`

- `setup_tdrdata_nl4r`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf25e4 to 0xf2d4c.
  Established. The call to `0x1b124` carries `0xa8` and three unknown candidates (`bl` at 0xf2618).
  A later word store has unknown target and writes `0x0` (`str` at 0xf266c). The final call to
  `0xf07c8` receives `sp-64`, `[sp-36]:2`, `0x144a3c`, `sp-64` (`bl` at 0xf2c54); its common exit
  reports r0 `[sp-72]` (`ldmdb` at 0xf2cd0). Router scan-record layout and naming are not
  established by these values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf25f4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf25fc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf2600 | load | 4 | `abs 0xf2cd4` | `-` |
| `str` at 0xf2604 | store | 4 | `sp -0x44` | `0x138988` |
| `ldr` at 0xf2608 | load | 4 | `abs 0xf2cd8` | `-` |
| `ldr` at 0xf2610 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf2620 | store | 4 | `[sp-24] +0x28` | `ret(0x1b124)` |
| `ldr` at 0xf2624 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2634 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf2640 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf2644 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2660 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf266c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf2670 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2678 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2680 | load | 4 | `abs 0xf2cdc` | `-` |
| `ldr` at 0xf2688 | load | 4 | `abs 0xf2ce0` | `-` |
| `ldr` at 0xf2694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf269c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf26a8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf26b0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf26b8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf26bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf26cc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf26d4 | load | 4 | `abs 0xf2ce4` | `-` |
| `ldr` at 0xf26dc | load | 4 | `abs 0xf2ce8` | `-` |
| `ldr` at 0xf26e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf26f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf26fc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2704 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf270c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2710 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2720 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2728 | load | 4 | `abs 0xf2cec` | `-` |
| `ldr` at 0xf2730 | load | 4 | `abs 0xf2cf0` | `-` |
| `ldr` at 0xf273c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2744 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2750 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2758 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2760 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2764 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2774 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf277c | load | 4 | `abs 0xf2cf4` | `-` |
| `ldr` at 0xf2784 | load | 4 | `abs 0xf2cf8` | `-` |
| `ldr` at 0xf2790 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2798 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf27a4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf27ac | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf27b4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf27b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf27c8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf27d0 | load | 4 | `abs 0xf2cfc` | `-` |
| `ldr` at 0xf27d8 | load | 4 | `abs 0xf2d00` | `-` |
| `ldr` at 0xf27e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf27ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf27f8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2800 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2808 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf280c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf281c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2824 | load | 4 | `abs 0xf2d04` | `-` |
| `ldr` at 0xf282c | load | 4 | `abs 0xf2d08` | `-` |
| `ldr` at 0xf2838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2840 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf284c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2854 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf285c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2860 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2870 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2878 | load | 4 | `abs 0xf2d0c` | `-` |
| `ldr` at 0xf2880 | load | 4 | `abs 0xf2d10` | `-` |
| `ldr` at 0xf288c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2894 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf28a0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf28a8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf28b0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf28b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf28c4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf28cc | load | 4 | `abs 0xf2d14` | `-` |
| `ldr` at 0xf28d4 | load | 4 | `abs 0xf2d18` | `-` |
| `ldr` at 0xf28e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf28e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf28f4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf28fc | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2904 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2908 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2918 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2920 | load | 4 | `abs 0xf2d1c` | `-` |
| `ldr` at 0xf2928 | load | 4 | `abs 0xf2d20` | `-` |
| `ldr` at 0xf2934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf293c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2948 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2950 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2958 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf295c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf296c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2974 | load | 4 | `abs 0xf2d24` | `-` |
| `ldr` at 0xf297c | load | 4 | `abs 0xf2d28` | `-` |
| `ldr` at 0xf2988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2990 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf299c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf29a4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf29ac | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf29b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf29c0 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf29cc | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf29d4 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xf29d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf29ec | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf29f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf29f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf2a00 | load | 4 | `abs 0xf2d2c` | `-` |
| `ldrh` at 0xf2a10 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf2a1c | load | 4 | `abs 0xf2d30` | `-` |
| `ldr` at 0xf2a28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2a30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2a3c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2a44 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2a4c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2a50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2a60 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2a68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2a70 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2a74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf2a7c | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xf2a88 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf2a90 | store | 4 | `sp -0x24` | `0x11` |
| `ldr` at 0xf2a94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2aa8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf2aac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf2ab4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf2abc | load | 4 | `abs 0xf2d34` | `-` |
| `ldrh` at 0xf2acc | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf2ad8 | load | 4 | `abs 0xf2d38` | `-` |
| `ldr` at 0xf2ae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2aec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2af8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2b00 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2b08 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2b0c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2b1c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2b24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2b2c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2b30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf2b38 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xf2b44 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf2b4c | store | 4 | `sp -0x24` | `0x60` |
| `ldr` at 0xf2b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2b64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf2b68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf2b70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf2b78 | load | 4 | `abs 0xf2d3c` | `-` |
| `ldrh` at 0xf2b88 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf2b94 | load | 4 | `abs 0xf2d40` | `-` |
| `ldr` at 0xf2ba0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2ba8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2bb4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2bbc | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2bc4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2bc8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2bd8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2be0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2be8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2bec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf2bf4 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xf2c00 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf2c08 | store | 4 | `sp -0x24` | `0x61` |
| `ldr` at 0xf2c0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2c20 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf2c24 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf2c2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf2c34 | load | 4 | `abs 0xf2d44` | `-` |
| `ldrh` at 0xf2c44 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf2c50 | load | 4 | `abs 0xf2d48` | `-` |
| `ldr` at 0xf2c5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2c64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2c70 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2c78 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2c80 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2c84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2c94 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf2c9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2ca4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2ca8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf2cb0 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0xf2cb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2cbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf2cc0 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf2cc8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xf2ccc | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xf2cd0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf2cd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf2cd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf2cd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf260c | `(unknown callee) 0x23f2c` | `0x138994` | `r1` | `r2` | `0x138988` |
| `bl` at 0xf2618 | `(unknown callee) 0x1b124` | `0xa8` | `?` | `?` | `?` |
| `bl` at 0xf268c | `(unknown callee) 0xf07c8` | `0x1389ac` | `0x7c` | `0x144878` | `?` |
| `bl` at 0xf26e0 | `(unknown callee) 0xf07c8` | `0x1389b8` | `0x7d` | `0x144890` | `?` |
| `bl` at 0xf2734 | `(unknown callee) 0xf07c8` | `0x1389c4` | `0x7f` | `0x1448a8` | `?` |
| `bl` at 0xf2788 | `(unknown callee) 0xf07c8` | `0x1389cc` | `0x7e` | `0x143534` | `?` |
| `bl` at 0xf27dc | `(unknown callee) 0xf07c8` | `0x1389d4` | `0x7` | `0x1435a0` | `?` |
| `bl` at 0xf2830 | `(unknown callee) 0xf07c8` | `0x1389dc` | `0x5` | `0x1435dc` | `?` |
| `bl` at 0xf2884 | `(unknown callee) 0xf07c8` | `0x1389e8` | `0x6` | `0x143a68` | `?` |
| `bl` at 0xf28d8 | `(unknown callee) 0xf07c8` | `0x1389f0` | `0xd` | `0x1448c0` | `?` |
| `bl` at 0xf292c | `(unknown callee) 0xf07c8` | `0x138a00` | `0xc` | `0x1448d8` | `?` |
| `bl` at 0xf2980 | `(unknown callee) 0xf07c8` | `0x138a10` | `0x4b` | `0x144a58` | `?` |
| `bl` at 0xf2a08 | `(unknown callee) 0x1b49c` | `sp-64` | `0x138a1c` | `?` | `?` |
| `bl` at 0xf2a20 | `(unknown callee) 0xf07c8` | `sp-64` | `[sp-36]:2` | `0x143c64` | `sp-64` |
| `bl` at 0xf2ac4 | `(unknown callee) 0x1b49c` | `sp-64` | `0x138a24` | `?` | `?` |
| `bl` at 0xf2adc | `(unknown callee) 0xf07c8` | `sp-64` | `[sp-36]:2` | `0x1449c4` | `sp-64` |
| `bl` at 0xf2b80 | `(unknown callee) 0x1b49c` | `sp-64` | `0x138a30` | `?` | `?` |
| `bl` at 0xf2b98 | `(unknown callee) 0xf07c8` | `sp-64` | `[sp-36]:2` | `0x143d40` | `sp-64` |
| `bl` at 0xf2c3c | `(unknown callee) 0x1b49c` | `sp-64` | `0x138a40` | `?` | `?` |
| `bl` at 0xf2c54 | `(unknown callee) 0xf07c8` | `sp-64` | `[sp-36]:2` | `0x144a3c` | `sp-64` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf262c | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf264c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf26c4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2718 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf276c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf27c0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2814 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2868 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf28bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2910 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2964 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf29b8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf29e0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2a58 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf2a9c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2b14 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf2b58 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2bd0 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf2c14 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2c8c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf2cd0 | `r0` | `[sp-72]` |

## `setup_tdrdata_shub2`

- `setup_tdrdata_shub2`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf2d4c to 0xf36e0.
  Established. The initial call to `0x1b124` has `0x64`, entry r1, entry r2, `0x0` (`bl` at
  0xf2d70). The result is stored as a word at entry `r0 +0x28` (`str` at 0xf2d78). The final call to
  `0xf07c8` uses `0x1396f8`, `0x3`, `0x144aac`, unknown (`bl` at 0xf35c4). The return reports r0
  `[sp-36]` (`ldmdb` at 0xf3624). SHUB2 field identities and physical register effects remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf2d5c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf2d64 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf2d68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf2d78 | store | 4 | `r0 +0x28` | `ret(0x1b124)` |
| `ldr` at 0xf2d7c | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xf2d8c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xf2d98 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf2d9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2dac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2db0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2db8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2dc4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf2dc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2dd0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2dd8 | load | 4 | `abs 0xf3628` | `-` |
| `ldr` at 0xf2de0 | load | 4 | `abs 0xf362c` | `-` |
| `ldr` at 0xf2dec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2df4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2e00 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2e08 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2e10 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2e14 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2e24 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2e2c | load | 4 | `abs 0xf3630` | `-` |
| `ldr` at 0xf2e34 | load | 4 | `abs 0xf3634` | `-` |
| `ldr` at 0xf2e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2e54 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2e5c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2e64 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2e68 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2e78 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2e80 | load | 4 | `abs 0xf3638` | `-` |
| `ldr` at 0xf2e88 | load | 4 | `abs 0xf363c` | `-` |
| `ldr` at 0xf2e94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2e9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2ea8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2eb0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2eb8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2ebc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2ecc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2ed4 | load | 4 | `abs 0xf3640` | `-` |
| `ldr` at 0xf2edc | load | 4 | `abs 0xf3644` | `-` |
| `ldr` at 0xf2ee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2ef0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2efc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2f04 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2f0c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2f10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2f20 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2f28 | load | 4 | `abs 0xf3648` | `-` |
| `ldr` at 0xf2f30 | load | 4 | `abs 0xf364c` | `-` |
| `ldr` at 0xf2f3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2f44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2f50 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2f58 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2f60 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2f64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2f74 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2f7c | load | 4 | `abs 0xf3650` | `-` |
| `ldr` at 0xf2f84 | load | 4 | `abs 0xf3654` | `-` |
| `ldr` at 0xf2f90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2f98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2fa4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2fac | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf2fb4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2fb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2fc8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2fd0 | load | 4 | `abs 0xf3658` | `-` |
| `ldr` at 0xf2fd8 | load | 4 | `abs 0xf365c` | `-` |
| `ldr` at 0xf2fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2fec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2ff8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3000 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3008 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf300c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf301c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3024 | load | 4 | `abs 0xf3660` | `-` |
| `ldr` at 0xf302c | load | 4 | `abs 0xf3664` | `-` |
| `ldr` at 0xf3038 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf304c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3054 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf305c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3060 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3070 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3078 | load | 4 | `abs 0xf3668` | `-` |
| `ldr` at 0xf3080 | load | 4 | `abs 0xf366c` | `-` |
| `ldr` at 0xf308c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3094 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf30a0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf30a8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf30b0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf30b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf30c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf30cc | load | 4 | `abs 0xf3670` | `-` |
| `ldr` at 0xf30d4 | load | 4 | `abs 0xf3674` | `-` |
| `ldr` at 0xf30e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf30e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf30f4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf30fc | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3104 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3108 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3118 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3120 | load | 4 | `abs 0xf3678` | `-` |
| `ldr` at 0xf3128 | load | 4 | `abs 0xf3674` | `-` |
| `ldr` at 0xf3134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf313c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3148 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3150 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3158 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf315c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf316c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3174 | load | 4 | `abs 0xf367c` | `-` |
| `ldr` at 0xf317c | load | 4 | `abs 0xf3680` | `-` |
| `ldr` at 0xf3188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3190 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf319c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf31a4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf31ac | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf31b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf31c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf31c8 | load | 4 | `abs 0xf3684` | `-` |
| `ldr` at 0xf31d0 | load | 4 | `abs 0xf3680` | `-` |
| `ldr` at 0xf31dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf31e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf31f0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf31f8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3200 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3204 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3214 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf321c | load | 4 | `abs 0xf3688` | `-` |
| `ldr` at 0xf3224 | load | 4 | `abs 0xf368c` | `-` |
| `ldr` at 0xf3230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3238 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3244 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf324c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3254 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3258 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3268 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3270 | load | 4 | `abs 0xf3690` | `-` |
| `ldr` at 0xf3278 | load | 4 | `abs 0xf368c` | `-` |
| `ldr` at 0xf3284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf328c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3298 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf32a0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf32a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf32ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xf32bc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf32c4 | load | 4 | `abs 0xf3694` | `-` |
| `ldr` at 0xf32cc | load | 4 | `abs 0xf3698` | `-` |
| `ldr` at 0xf32d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf32e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf32ec | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf32f4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf32fc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3300 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3310 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3318 | load | 4 | `abs 0xf369c` | `-` |
| `ldr` at 0xf3320 | load | 4 | `abs 0xf3698` | `-` |
| `ldr` at 0xf332c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3334 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3340 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3348 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3350 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3354 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3364 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf336c | load | 4 | `abs 0xf36a0` | `-` |
| `ldr` at 0xf3374 | load | 4 | `abs 0xf36a4` | `-` |
| `ldr` at 0xf3380 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3388 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3394 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf339c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf33a4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf33a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf33b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf33c0 | load | 4 | `abs 0xf36a8` | `-` |
| `ldr` at 0xf33c8 | load | 4 | `abs 0xf36ac` | `-` |
| `ldr` at 0xf33d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf33dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf33e8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf33f0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf33f8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf33fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf340c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3414 | load | 4 | `abs 0xf36b0` | `-` |
| `ldr` at 0xf341c | load | 4 | `abs 0xf36b4` | `-` |
| `ldr` at 0xf3428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3430 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf343c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3444 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf344c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3450 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3460 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3468 | load | 4 | `abs 0xf36b8` | `-` |
| `ldr` at 0xf3470 | load | 4 | `abs 0xf36bc` | `-` |
| `ldr` at 0xf347c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3484 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3490 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3498 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf34a0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf34a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf34b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf34bc | load | 4 | `abs 0xf36c0` | `-` |
| `ldr` at 0xf34c4 | load | 4 | `abs 0xf36c4` | `-` |
| `ldr` at 0xf34d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf34d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf34e4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf34ec | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf34f4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf34f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3508 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3510 | load | 4 | `abs 0xf36c8` | `-` |
| `ldr` at 0xf3518 | load | 4 | `abs 0xf36cc` | `-` |
| `ldr` at 0xf3524 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf352c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3538 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3540 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf3548 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf354c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf355c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3564 | load | 4 | `abs 0xf36d0` | `-` |
| `ldr` at 0xf356c | load | 4 | `abs 0xf36d4` | `-` |
| `ldr` at 0xf3578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf358c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3594 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf359c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf35a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf35b0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf35b8 | load | 4 | `abs 0xf36d8` | `-` |
| `ldr` at 0xf35c0 | load | 4 | `abs 0xf36dc` | `-` |
| `ldr` at 0xf35cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf35d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf35e0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf35e8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf35f0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf35f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3604 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf360c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3610 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf3614 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf361c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf3620 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf3624 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf3624 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf3624 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf3624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf2d70 | `(unknown callee) 0x1b124` | `0x64` | `r1` | `r2` | `0x0` |
| `bl` at 0xf2de4 | `(unknown callee) 0xf07c8` | `0x1395c0` | `0x7c` | `0x144878` | `?` |
| `bl` at 0xf2e38 | `(unknown callee) 0xf07c8` | `0x1395cc` | `0x7d` | `0x144890` | `?` |
| `bl` at 0xf2e8c | `(unknown callee) 0xf07c8` | `0x1395d8` | `0x7f` | `0x1448a8` | `?` |
| `bl` at 0xf2ee0 | `(unknown callee) 0xf07c8` | `0x1395e0` | `0x7e` | `0x143ee4` | `?` |
| `bl` at 0xf2f34 | `(unknown callee) 0xf07c8` | `0x1395e8` | `0x7` | `0x143f80` | `?` |
| `bl` at 0xf2f88 | `(unknown callee) 0xf07c8` | `0x1395f0` | `0x5` | `0x144090` | `?` |
| `bl` at 0xf2fdc | `(unknown callee) 0xf07c8` | `0x1395fc` | `0x6` | `0x1442dc` | `?` |
| `bl` at 0xf3030 | `(unknown callee) 0xf07c8` | `0x139604` | `0x12` | `0x1448c0` | `?` |
| `bl` at 0xf3084 | `(unknown callee) 0xf07c8` | `0x139614` | `0x11` | `0x1448d8` | `?` |
| `bl` at 0xf30d8 | `(unknown callee) 0xf07c8` | `0x139624` | `0xa` | `0x14452c` | `?` |
| `bl` at 0xf312c | `(unknown callee) 0xf07c8` | `0x139630` | `0xb` | `0x14452c` | `?` |
| `bl` at 0xf3180 | `(unknown callee) 0xf07c8` | `0x13963c` | `0x1a` | `0x1449c4` | `?` |
| `bl` at 0xf31d4 | `(unknown callee) 0xf07c8` | `0x13964c` | `0x1b` | `0x1449c4` | `?` |
| `bl` at 0xf3228 | `(unknown callee) 0xf07c8` | `0x13965c` | `0x60` | `0x144608` | `?` |
| `bl` at 0xf327c | `(unknown callee) 0xf07c8` | `0x13966c` | `0x62` | `0x144608` | `?` |
| `bl` at 0xf32d0 | `(unknown callee) 0xf07c8` | `0x13967c` | `0x61` | `0x144a3c` | `?` |
| `bl` at 0xf3324 | `(unknown callee) 0xf07c8` | `0x13968c` | `0x63` | `0x144a3c` | `?` |
| `bl` at 0xf3378 | `(unknown callee) 0xf07c8` | `0x13969c` | `0x14` | `0x144a58` | `?` |
| `bl` at 0xf33cc | `(unknown callee) 0xf07c8` | `0x1396a8` | `0x9` | `0x14431c` | `?` |
| `bl` at 0xf3420 | `(unknown callee) 0xf07c8` | `0x1396b4` | `0xc` | `0x143f9c` | `?` |
| `bl` at 0xf3474 | `(unknown callee) 0xf07c8` | `0x1396c4` | `0xe` | `0x143fb4` | `?` |
| `bl` at 0xf34c8 | `(unknown callee) 0xf07c8` | `0x1396d4` | `0xd` | `0x144004` | `?` |
| `bl` at 0xf351c | `(unknown callee) 0xf07c8` | `0x1396e0` | `0x15` | `0x144060` | `?` |
| `bl` at 0xf3570 | `(unknown callee) 0xf07c8` | `0x1396ec` | `0x13` | `0x144a70` | `?` |
| `bl` at 0xf35c4 | `(unknown callee) 0xf07c8` | `0x1396f8` | `0x3` | `0x144aac` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf2d84 | `[r0+40] != 0x0` | `[r0+40] == 0x0` |
| `bls` at 0xf2da4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2e1c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2e70 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2ec4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2f18 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2f6c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2fc0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3014 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3068 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf30bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3110 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3164 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf31b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf320c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3260 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf32b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3308 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf335c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf33b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3404 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3458 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf34ac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3500 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3554 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf35a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf35fc | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf3624 | `r0` | `[sp-36]` |

## `setup_tdrdata_psic`

- `setup_tdrdata_psic`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf6ce4 to 0xf7300.
  Established. The call to `0x1b124` carries `0x16c` and three unknown candidates (`bl` at 0xf6d10);
  its result is saved as a word at `[sp-24] +0x28` (`str` at 0xf6d18). The final call to `0x21e28`
  has `0x9`, `0x139dcc`, `0x153`, `0x0` (`bl` at 0xf7280). The exit supplies r0 `[sp-112]` (`ldmdb`
  at 0xf729c). PSIC data-register encoding and the numeric callee roles remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf6cf4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf6cfc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf6d00 | load | 4 | `abs 0xf72a0` | `-` |
| `ldr` at 0xf6d08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf6d18 | store | 4 | `[sp-24] +0x28` | `ret(0x1b124)` |
| `ldr` at 0xf6d1c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6d2c | store | 4 | `sp -0x70` | `?` |
| `str` at 0xf6d38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf6d3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6d4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6d50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6d58 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6d64 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf6d68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6d70 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf6d78 | load | 4 | `abs 0xf72a4` | `-` |
| `ldr` at 0xf6d80 | load | 4 | `abs 0xf72a8` | `-` |
| `ldr` at 0xf6d8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6d94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf6da0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6da8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf6db0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf6db4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6dc4 | store | 4 | `sp -0x70` | `?` |
| `str` at 0xf6dd0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf6dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6de4 | load | 4 | `abs 0xf72ac` | `-` |
| `ldm` at 0xf6df0 | load | 4 | `abs 0x139cc4` | `-` |
| `ldm` at 0xf6df0 | load | 4 | `abs 0x139cc8` | `-` |
| `ldm` at 0xf6df0 | load | 4 | `abs 0x139ccc` | `-` |
| `ldm` at 0xf6df0 | load | 4 | `abs 0x139cd0` | `-` |
| `stm` at 0xf6df4 | store | 4 | `sp -0x58` | `[0x139cc4]` |
| `stm` at 0xf6df4 | store | 4 | `sp -0x54` | `[0x139cc8]` |
| `stm` at 0xf6df4 | store | 4 | `sp -0x50` | `[0x139ccc]` |
| `stm` at 0xf6df4 | store | 4 | `sp -0x4c` | `[0x139cd0]` |
| `ldr` at 0xf6df8 | load | 4 | `abs 0x139cd4` | `-` |
| `str` at 0xf6dfc | store | 4 | `sp -0x48` | `[0x139cd4]` |
| `ldr` at 0xf6e00 | load | 4 | `abs 0xf72b0` | `-` |
| `ldm` at 0xf6e0c | load | 4 | `abs 0x139cd8` | `-` |
| `ldm` at 0xf6e0c | load | 4 | `abs 0x139cdc` | `-` |
| `ldm` at 0xf6e0c | load | 4 | `abs 0x139ce0` | `-` |
| `ldm` at 0xf6e0c | load | 4 | `abs 0x139ce4` | `-` |
| `stm` at 0xf6e10 | store | 4 | `sp -0x6c` | `[0x139cd8]` |
| `stm` at 0xf6e10 | store | 4 | `sp -0x68` | `[0x139cdc]` |
| `stm` at 0xf6e10 | store | 4 | `sp -0x64` | `[0x139ce0]` |
| `stm` at 0xf6e10 | store | 4 | `sp -0x60` | `[0x139ce4]` |
| `ldr` at 0xf6e14 | load | 4 | `abs 0x139ce8` | `-` |
| `str` at 0xf6e18 | store | 4 | `sp -0x5c` | `[0x139ce8]` |
| `str` at 0xf6e20 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf6e24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf6e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6e54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6e58 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xf6e64 | load | 2 | `sp -0x20` | `-` |
| `ldrh` at 0xf6e6c | load | 2 | `sp -0x1c` | `-` |
| `ldr` at 0xf6e80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf6e98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf6ea4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6eac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf6eb8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6ec0 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf6ec8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf6ecc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6edc | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf6ee4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf6eec | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf6ef4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf6efc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xf6f04 | load | 4 | `abs 0xf72b4` | `-` |
| `ldr` at 0xf6f0c | load | 4 | `abs 0xf72b8` | `-` |
| `ldr` at 0xf6f18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6f20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf6f2c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6f34 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf6f3c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf6f40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6f50 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf6f58 | load | 4 | `abs 0xf72bc` | `-` |
| `ldr` at 0xf6f60 | load | 4 | `abs 0xf72c0` | `-` |
| `ldr` at 0xf6f6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6f74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf6f80 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6f88 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf6f90 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf6f94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6fa4 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf6fac | load | 4 | `abs 0xf72c4` | `-` |
| `ldr` at 0xf6fb4 | load | 4 | `abs 0xf72c0` | `-` |
| `ldr` at 0xf6fc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf6fc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf6fd4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf6fdc | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf6fe4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf6fe8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf6ff8 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7000 | load | 4 | `abs 0xf72c8` | `-` |
| `ldr` at 0xf7008 | load | 4 | `abs 0xf72cc` | `-` |
| `ldr` at 0xf7014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf701c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7028 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7030 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf7038 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf703c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf704c | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7054 | load | 4 | `abs 0xf72d0` | `-` |
| `ldr` at 0xf705c | load | 4 | `abs 0xf72cc` | `-` |
| `ldr` at 0xf7068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7070 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf707c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7084 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf708c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7090 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf70a0 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf70a8 | load | 4 | `abs 0xf72d4` | `-` |
| `ldr` at 0xf70b0 | load | 4 | `abs 0xf72d8` | `-` |
| `ldr` at 0xf70bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf70c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf70d0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf70d8 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf70e0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf70e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf70f4 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf70fc | load | 4 | `abs 0xf72dc` | `-` |
| `ldr` at 0xf7104 | load | 4 | `abs 0xf72d8` | `-` |
| `ldr` at 0xf7110 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7124 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf712c | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf7134 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7138 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7148 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7150 | load | 4 | `abs 0xf72e0` | `-` |
| `ldr` at 0xf7158 | load | 4 | `abs 0xf72e4` | `-` |
| `ldr` at 0xf7164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf716c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7178 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7180 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf7188 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf718c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf719c | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf71a4 | load | 4 | `abs 0xf72e8` | `-` |
| `ldr` at 0xf71ac | load | 4 | `abs 0xf72e4` | `-` |
| `ldr` at 0xf71b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf71c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf71cc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf71d4 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf71dc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf71e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf71f0 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf71f8 | load | 4 | `abs 0xf72ec` | `-` |
| `ldr` at 0xf7200 | load | 4 | `abs 0xf72f0` | `-` |
| `ldr` at 0xf720c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7214 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7220 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7228 | store | 4 | `(unknown)` | `ret(0xf07c8)` |
| `str` at 0xf7230 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7234 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7244 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf724c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7258 | load | 4 | `abs 0xf72f4` | `-` |
| `str` at 0xf725c | store | 4 | `sp -0x7c` | `0x139d88` |
| `str` at 0xf7264 | store | 4 | `sp -0x78` | `0x5b` |
| `ldr` at 0xf7268 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf726c | store | 4 | `sp -0x74` | `[sp-36]+1` |
| `ldr` at 0xf7274 | load | 4 | `abs 0xf72f8` | `-` |
| `ldr` at 0xf7278 | load | 4 | `abs 0xf72fc` | `-` |
| `ldr` at 0xf7284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7288 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf728c | store | 4 | `[sp-24] +0x24` | `?` |
| `str` at 0xf7294 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xf7298 | load | 4 | `sp -0x70` | `-` |
| `ldmdb` at 0xf729c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf729c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf729c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf729c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf6d04 | `(unknown callee) 0x23f2c` | `0x139cec` | `r1` | `r2` | `0x0` |
| `bl` at 0xf6d10 | `(unknown callee) 0x1b124` | `0x16c` | `?` | `?` | `?` |
| `bl` at 0xf6d84 | `(unknown callee) 0xf07c8` | `0x139d04` | `0x8` | `0x144620` | `?` |
| `bl` at 0xf6e5c | `(unknown callee) 0x1b49c` | `sp-68` | `?` | `[sp-32]` | `?` |
| `bl` at 0xf6e9c | `(unknown callee) 0xf07c8` | `sp-68` | `?` | `?` | `?` |
| `bl` at 0xf6f10 | `(unknown callee) 0xf07c8` | `0x139d10` | `0x10` | `0x144638` | `?` |
| `bl` at 0xf6f64 | `(unknown callee) 0xf07c8` | `0x139d1c` | `0x11` | `0x144650` | `?` |
| `bl` at 0xf6fb8 | `(unknown callee) 0xf07c8` | `0x139d28` | `0x12` | `0x144650` | `?` |
| `bl` at 0xf700c | `(unknown callee) 0xf07c8` | `0x139d34` | `0x13` | `0x144668` | `?` |
| `bl` at 0xf7060 | `(unknown callee) 0xf07c8` | `0x139d40` | `0x14` | `0x144668` | `?` |
| `bl` at 0xf70b4 | `(unknown callee) 0xf07c8` | `0x139d4c` | `0x15` | `0x1446d4` | `?` |
| `bl` at 0xf7108 | `(unknown callee) 0xf07c8` | `0x139d58` | `0x16` | `0x1446d4` | `?` |
| `bl` at 0xf715c | `(unknown callee) 0xf07c8` | `0x139d64` | `0x17` | `0x1446ec` | `?` |
| `bl` at 0xf71b0 | `(unknown callee) 0xf07c8` | `0x139d70` | `0x18` | `0x1446ec` | `?` |
| `bl` at 0xf7204 | `(unknown callee) 0xf07c8` | `0x139d7c` | `0x19` | `0x144818` | `?` |
| `bl` at 0xf7280 | `(unknown callee) 0x21e28` | `0x9` | `0x139dcc` | `0x153` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf6d24 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf6d44 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf6dbc | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf6ddc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xf6e2c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf6ed4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf6f48 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf6f9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf6ff0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7044 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7098 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf70ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7140 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7194 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf71e8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf723c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xf7254 | `[sp-36]+1 == 0x5b` | `[sp-36]+1 != 0x5b` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf729c | `r0` | `[sp-112]` |

## `smpcmd_scan_ids`

- `smpcmd_scan_ids`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8128 to 0xe8628.
  Established. The unsigned count test takes its continuation for `[[sp-24]+40] <= 0x40` and falls
  through for greater (`bls` at 0xe8194). Two word stores later write `0x0` at unknown targets
  (`str` at 0xe82c8, `str` at 0xe82e0). The final call to `0x1b400` has `[sp-68]` and three unknown
  candidates (`bl` at 0xe85d8). The exit reports r0 `0x0` (`ldmdb` at 0xe85e8). Device-ID
  interpretation and buffer ownership remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8138 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe813c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8140 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8150 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe8154 | load | 4 | `abs 0xe85ec` | `-` |
| `ldr` at 0xe8158 | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe8164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8168 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe816c | load | 4 | `abs 0xe85f0` | `-` |
| `str` at 0xe8178 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8184 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe8188 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xe818c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8198 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe819c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe81a0 | load | 4 | `abs 0xe85f4` | `-` |
| `str` at 0xe81b0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe81b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe81c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe81c8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe81cc | load | 4 | `abs 0xe85f8` | `-` |
| `str` at 0xe81d8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe81e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe81e8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe81ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe8200 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe820c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe8210 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8224 | store | 4 | `sp -0x40` | `ret(0x1b124)` |
| `ldr` at 0xe8228 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8234 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8238 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe823c | load | 4 | `abs 0xe85fc` | `-` |
| `str` at 0xe8248 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8250 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8264 | store | 4 | `sp -0x44` | `ret(0x1b124)` |
| `ldr` at 0xe8268 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8274 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8278 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe827c | load | 4 | `abs 0xe85fc` | `-` |
| `ldr` at 0xe8284 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8290 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xe829c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe82a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe82a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe82b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe82bc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe82c8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe82cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe82d4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe82e0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe82e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe82ec | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe8304 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8308 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8310 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8314 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8328 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe832c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8330 | load | 4 | `abs 0xe85f0` | `-` |
| `ldr` at 0xe8338 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8340 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe834c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8354 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe8358 | store | 4 | `sp -0x58` | `[sp-56]` |
| `ldr` at 0xe835c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8360 | store | 4 | `sp -0x54` | `[sp-64]` |
| `ldr` at 0xe8364 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8368 | store | 4 | `sp -0x50` | `[sp-68]` |
| `ldr` at 0xe836c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8370 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe8374 | store | 4 | `sp -0x4c` | `[[sp-28]]` |
| `ldr` at 0xe8384 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe839c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe83a0 | load | 4 | `abs 0xe85f0` | `-` |
| `ldr` at 0xe83a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe83b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe83bc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe83c4 | load | 4 | `abs 0xe8600` | `-` |
| `ldr` at 0xe83cc | load | 4 | `abs 0xe8604` | `-` |
| `ldr` at 0xe83d4 | load | 4 | `abs 0xe8608` | `-` |
| `ldr` at 0xe83dc | load | 4 | `abs 0xe860c` | `-` |
| `str` at 0xe83e8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xe83f0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe83f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe83f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8408 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe840c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8420 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe8428 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe842c | load | 4 | `abs 0xe8610` | `-` |
| `ldr` at 0xe8430 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8440 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xe844c | store | 4 | `sp -0x58` | `0x20` |
| `ldr` at 0xe8450 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe8454 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8464 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe846c | store | 4 | `sp -0x3c` | `[sp-60]+32` |
| `ldr` at 0xe8470 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8474 | load | 4 | `abs 0xe8614` | `-` |
| `ldr` at 0xe8480 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8488 | load | 4 | `abs 0xe8614` | `-` |
| `str` at 0xe8494 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8498 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe849c | load | 4 | `abs 0xe8618` | `-` |
| `ldr` at 0xe84a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe84b0 | load | 4 | `abs 0xe8618` | `-` |
| `str` at 0xe84bc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe84c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe84c4 | load | 4 | `abs 0xe861c` | `-` |
| `ldr` at 0xe84d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe84d8 | load | 4 | `abs 0xe861c` | `-` |
| `str` at 0xe84e4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe84e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe84f8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe850c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8510 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8520 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8534 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe853c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe8548 | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0xe8550 | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0xe8560 | load | 2 | `sp -0x2e` | `-` |
| `str` at 0xe8578 | store | 4 | `sp -0x58` | `?` |
| `ldrb` at 0xe857c | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xe8588 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe858c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8590 | store | 4 | `sp -0x50` | `[sp-52]` |
| `ldr` at 0xe8594 | load | 4 | `abs 0xe8620` | `-` |
| `ldr` at 0xe8598 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe85a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe85ac | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe85b4 | load | 4 | `abs 0xe85f0` | `-` |
| `ldr` at 0xe85bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85c0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe85c4 | load | 4 | `abs 0xe8624` | `-` |
| `ldr` at 0xe85cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe85d4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe85e0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe85e4 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe85e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8148 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe8170 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe81a8 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135dbc` | `0x40` | `[sp-28]` |
| `bl` at 0xe81d0 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135de4` | `?` | `[sp-28]` |
| `bl` at 0xe821c | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xe8240 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135e08` | `?` | `[sp-28]` |
| `bl` at 0xe825c | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xe8280 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135e08` | `?` | `[sp-28]` |
| `bl` at 0xe8288 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe82f4 | `(unknown callee) 0xe58a0` | `ret(0x1b124)` | `?` | `?` | `[sp-36]` |
| `bl` at 0xe8318 | `(unknown callee) 0xdc9a8` | `0x1` | `[sp-40]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xe8334 | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe833c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe8344 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe8388 | `(unknown callee) 0xdcf54` | `0x1` | `0x0` | `0x1` | `[sp-40]` |
| `bl` at 0xe83a4 | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe83ac | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe83b4 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe83c8 | `(unknown callee) 0x23f2c` | `0x135e34` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xe83d0 | `(unknown callee) 0x23f2c` | `0x135e38` | `?` | `?` | `?` |
| `bl` at 0xe83d8 | `(unknown callee) 0x23f2c` | `0x135e50` | `?` | `?` | `?` |
| `bl` at 0xe83e0 | `(unknown callee) 0x23f2c` | `0x135e80` | `?` | `?` | `?` |
| `bl` at 0xe8410 | `(unknown callee) 0xe1cec` | `?` | `[sp-68]` | `?` | `[sp-32]` |
| `bl` at 0xe8434 | `(unknown callee) 0x23f2c` | `0x135eb0` | `[sp-44]` | `?` | `[sp-60]+1` |
| `bl` at 0xe8460 | `(unknown callee) 0xe2174` | `[sp-60]` | `[sp-68]` | `0x0` | `sp-52` |
| `bl` at 0xe8544 | `(unknown callee) 0xe886c` | `[sp-52]` | `sp-48` | `?` | `sp-48` |
| `bl` at 0xe85a0 | `(unknown callee) 0x23f2c` | `0x135edc` | `[sp-44]` | `?` | `?` |
| `bl` at 0xe85b8 | `(unknown callee) 0x23f2c` | `0x135ce8` | `?` | `?` | `[sp-32]` |
| `bl` at 0xe85c8 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135f14` | `?` | `[sp-28]` |
| `bl` at 0xe85d0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe85d8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8160 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `bls` at 0xe8194 | `[[sp-24]+40] <= 0x40 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x40 (unsigned, 32-bit)` |
| `bne` at 0xe81c0 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bne` at 0xe8230 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xe8270 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `blo` at 0xe82ac | `C == 0` | `not (C == 0)` |
| `bne` at 0xe8324 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xe8394 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xe8400 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe841c | `ret(0xe1cec) != 0x0` | `ret(0xe1cec) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe85e8 | `r0` | `0x0` |
