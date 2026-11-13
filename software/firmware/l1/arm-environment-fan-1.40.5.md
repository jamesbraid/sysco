# ARM environment interrupts and fan callers in 1.40.5

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
The computed transfers in `autofanEnv` and `autofanEnvQuery` remain unresolved
(`ldrls` at 0x321f8, 0x327b4). Known-edge tables exclude effects reached only
through unknown destinations. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/environment-fan-facts.txt) retain all tool sections.

## `EnvPowerInt`

- `EnvPowerInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7df6c to 0x7e1d0.
  Established. Four-byte reads at absolute `0x149e74` and `0x149e80` precede byte comparisons (`ldr`
  at 0x7dfa0, `ldr` at 0x7dfc8). The branches take unequal byte pairs `[[sp-32]+4]:1 != [sp-21]:1`
  or `[[sp-32]+5]:1 != [sp-22]:1` to `0x7e000` (`bne` at 0x7dfe4, `bne` at 0x7dff8). A later byte
  store writes `0x1` at an unresolved effective address (`strb` at 0x7e170). The unnamed call to
  `0x22e50` carries `0x149bac`, unknown, unknown, `0x1` (`bl` at 0x7e178); its zero result takes the
  branch past the byte-zero store at `[sp-36] +0x1` (`beq` at 0x7e184, `strb` at 0x7e190).
  Interrupt-record and notification semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7df7c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7df84 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7df8c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7df94 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7df98 | load | 4 | `abs 0x7e1a4` | `-` |
| `ldr` at 0x7df9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7dfa0 | load | 4 | `abs 0x149e74` | `-` |
| `ldr` at 0x7dfb0 | load | 4 | `abs 0x7e1a4` | `-` |
| `ldr` at 0x7dfb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7dfc8 | load | 4 | `abs 0x149e80` | `-` |
| `str` at 0x7dfd0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7dfd4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7dfd8 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldrb` at 0x7dfdc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7dfe8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7dfec | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x7dff0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7e000 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7e008 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x7e010 | load | 4 | `abs 0x7e1a4` | `-` |
| `ldr` at 0x7e014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e018 | load | 4 | `abs 0x149e74` | `-` |
| `ldr` at 0x7e024 | load | 4 | `abs 0x7e1a8` | `-` |
| `str` at 0x7e028 | store | 4 | `sp -0x38` | `0x111568` |
| `ldrb` at 0x7e02c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e030 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x7e034 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7e038 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x7e040 | load | 4 | `abs 0x7e1ac` | `-` |
| `ldr` at 0x7e044 | load | 4 | `abs 0x7e1b0` | `-` |
| `str` at 0x7e054 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7e05c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e064 | load | 4 | `abs 0x7e1b4` | `-` |
| `str` at 0x7e06c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7e070 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e074 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e080 | load | 4 | `abs 0x7e1a4` | `-` |
| `ldr` at 0x7e084 | load | 4 | `abs 0x149e68` | `-` |
| `ldr` at 0x7e090 | load | 4 | `abs 0x7e1a4` | `-` |
| `ldr` at 0x7e094 | load | 4 | `abs 0x149e6c` | `-` |
| `ldr` at 0x7e0a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e0c0 | load | 4 | `abs 0x7e1b8` | `-` |
| `str` at 0x7e0c4 | store | 4 | `sp -0x38` | `0x1115a0` |
| `str` at 0x7e0c8 | store | 4 | `sp -0x34` | `ret(0x403f4)` |
| `ldrb` at 0x7e0cc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e0d0 | store | 4 | `sp -0x30` | `[sp-21]:1` |
| `ldrb` at 0x7e0d4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7e0d8 | store | 4 | `sp -0x2c` | `[sp-22]:1` |
| `ldr` at 0x7e0e0 | load | 4 | `abs 0x7e1ac` | `-` |
| `ldr` at 0x7e0e4 | load | 4 | `abs 0x7e1bc` | `-` |
| `str` at 0x7e0f4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7e0fc | load | 4 | `abs 0x7e1c0` | `-` |
| `ldr` at 0x7e100 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x7e114 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e130 | load | 4 | `abs 0x7e1c4` | `-` |
| `ldr` at 0x7e13c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e140 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e14c | load | 4 | `abs 0x7e1c0` | `-` |
| `ldr` at 0x7e150 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x7e160 | load | 4 | `abs 0x7e1c8` | `-` |
| `ldr` at 0x7e168 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x7e170 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x7e174 | load | 4 | `abs 0x7e1cc` | `-` |
| `ldr` at 0x7e188 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x7e190 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `str` at 0x7e198 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x7e19c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x7e1a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7e1a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7e1a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e04c | `(unknown callee) 0x21eb8` | `0x1` | `0x111244` | `0x517` | `0x0` |
| `bl` at 0x7e0a8 | `(unknown callee) 0x408ec` | `0x0` | `[sp-32]` | `?` | `[0x149e6c]` |
| `bl` at 0x7e0b8 | `(unknown callee) 0x403f4` | `0x0` | `ret(0x408ec)` | `?` | `ret(0x408ec)` |
| `bl` at 0x7e0ec | `(unknown callee) 0x21eb8` | `0x1` | `0x111244` | `0x524` | `0x0` |
| `bl` at 0x7e118 | `(unknown callee) 0x408ec` | `0x0` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7e128 | `(unknown callee) 0x403f4` | `0x0` | `ret(0x408ec)` | `?` | `ret(0x408ec)` |
| `bl` at 0x7e138 | `(unknown callee) 0x23fbc` | `0x1115d4` | `ret(0x403f4)` | `?` | `ret(0x403f4)` |
| `bl` at 0x7e164 | `(unknown callee) 0x23fbc` | `0x1115e8` | `?` | `?` | `?` |
| `bl` at 0x7e178 | `(unknown callee) 0x22e50` | `0x149bac` | `?` | `?` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x7dfa8 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7dfe4 | `[[sp-32]+4]:1 != [sp-21]:1` | `[[sp-32]+4]:1 == [sp-21]:1` |
| `bne` at 0x7dff8 | `[[sp-32]+5]:1 != [sp-22]:1` | `[[sp-32]+5]:1 == [sp-22]:1` |
| `blo` at 0x7e020 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7e07c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7e08c | `[0x149e68] != 0x0` | `[0x149e68] == 0x0` |
| `bne` at 0x7e09c | `[0x149e6c] != 0x0` | `[0x149e6c] == 0x0` |
| `beq` at 0x7e10c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7e148 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x7e15c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7e184 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7e1a0 | `r0` | `?` |

## `EnvFanInt`

- `EnvFanInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e1d0 to 0x7e4ac.
  Established. A byte store at `[sp-40] +0x1` retains `([[sp-40]+1]:1+1 & 0xff)` (`strb` at
  0x7e3b4). The unnamed call to `0xce554` has candidates `0x0`, `[[sp-36]+1]:1`, `[[sp-36]+2]:1`,
  `[[sp-36]+2]:1` (`bl` at 0x7e3f0). A later byte assignment writes `0x1` at `[sp-40] +0x2` (`strb`
  at 0x7e44c). The unnamed call to `0x22e50` carries `0x149c00`, unknown, `[sp-40]`, `0x1` (`bl` at
  0x7e454); its zero result takes the branch past a byte-zero store at `[sp-40] +0x2` (`beq` at
  0x7e460, `strb` at 0x7e46c). Fan-interrupt and notification semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e1e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7e1e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7e1f0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7e1f8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7e1fc | load | 4 | `abs 0x7e480` | `-` |
| `ldr` at 0x7e200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e204 | load | 4 | `abs 0x149e78` | `-` |
| `ldr` at 0x7e214 | load | 4 | `abs 0x7e480` | `-` |
| `ldr` at 0x7e218 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e22c | load | 4 | `abs 0x149e84` | `-` |
| `str` at 0x7e234 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7e238 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e23c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7e240 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7e24c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e250 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7e254 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7e264 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7e26c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7e274 | load | 4 | `abs 0x7e480` | `-` |
| `ldr` at 0x7e278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e27c | load | 4 | `abs 0x149e78` | `-` |
| `ldr` at 0x7e288 | load | 4 | `abs 0x7e484` | `-` |
| `str` at 0x7e28c | store | 4 | `sp -0x3c` | `0x111600` |
| `ldrb` at 0x7e290 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e294 | store | 4 | `sp -0x38` | `(r1 & 0xff)` |
| `ldrb` at 0x7e298 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7e29c | store | 4 | `sp -0x34` | `(r2 & 0xff)` |
| `ldr` at 0x7e2a4 | load | 4 | `abs 0x7e488` | `-` |
| `ldr` at 0x7e2a8 | load | 4 | `abs 0x7e48c` | `-` |
| `str` at 0x7e2b8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x7e2c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e2d4 | load | 4 | `abs 0x7e490` | `-` |
| `str` at 0x7e2dc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7e2e0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7e2e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e2f0 | load | 4 | `abs 0x7e480` | `-` |
| `ldr` at 0x7e2f4 | load | 4 | `abs 0x149e68` | `-` |
| `ldr` at 0x7e300 | load | 4 | `abs 0x7e480` | `-` |
| `ldr` at 0x7e304 | load | 4 | `abs 0x149e6c` | `-` |
| `ldrb` at 0x7e310 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x7e324 | load | 4 | `abs 0x7e494` | `-` |
| `str` at 0x7e328 | store | 4 | `sp -0x3c` | `0x111634` |
| `str` at 0x7e32c | store | 4 | `sp -0x38` | `ret(0x473a0)` |
| `ldrb` at 0x7e330 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e334 | store | 4 | `sp -0x34` | `[sp-21]:1` |
| `ldrb` at 0x7e338 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7e33c | store | 4 | `sp -0x30` | `[sp-22]:1` |
| `ldr` at 0x7e344 | load | 4 | `abs 0x7e488` | `-` |
| `ldr` at 0x7e348 | load | 4 | `abs 0x7e498` | `-` |
| `str` at 0x7e358 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x7e378 | store | 4 | `sp -0x20` | `0x1e` |
| `str` at 0x7e398 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0x7e3a4 | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x7e3a8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7e3ac | load | 1 | `[sp-40] +0x1` | `-` |
| `strb` at 0x7e3b4 | store | 1 | `[sp-40] +0x1` | `([[sp-40]+1]:1+1 & 0xff)` |
| `ldr` at 0x7e3bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7e3cc | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x7e3d4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e3d8 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x7e3dc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e3e0 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x7e3f4 | load | 4 | `abs 0x7e49c` | `-` |
| `ldr` at 0x7e3f8 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x7e408 | load | 4 | `abs 0x7e4a0` | `-` |
| `ldr` at 0x7e40c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e414 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7e418 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x7e424 | load | 4 | `abs 0x7e49c` | `-` |
| `ldr` at 0x7e428 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x7e438 | load | 4 | `abs 0x7e4a4` | `-` |
| `ldr` at 0x7e43c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e444 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x7e44c | store | 1 | `[sp-40] +0x2` | `0x1` |
| `ldr` at 0x7e450 | load | 4 | `abs 0x7e4a8` | `-` |
| `ldr` at 0x7e464 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x7e46c | store | 1 | `[sp-40] +0x2` | `0x0` |
| `str` at 0x7e474 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x7e478 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x7e47c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7e47c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7e47c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e2b0 | `(unknown callee) 0x21eb8` | `0x1` | `0x111244` | `0x556` | `0x0` |
| `bl` at 0x7e31c | `(unknown callee) 0x473a0` | `0x0` | `[sp-28]:1` | `?` | `[sp-28]:1` |
| `bl` at 0x7e350 | `(unknown callee) 0x21eb8` | `0x1` | `0x111244` | `0x563` | `0x0` |
| `bl` at 0x7e360 | `(unknown callee) 0x4f4d0` | `r0` | `?` | `?` | `?` |
| `bl` at 0x7e380 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x7e3f0 | `(unknown callee) 0xce554` | `0x0` | `[[sp-36]+1]:1` | `[[sp-36]+2]:1` | `[[sp-36]+2]:1` |
| `bl` at 0x7e410 | `(unknown callee) 0x23fbc` | `0x111664` | `[sp-28]` | `?` | `?` |
| `bl` at 0x7e440 | `(unknown callee) 0x23fbc` | `0x111678` | `[sp-28]` | `?` | `?` |
| `bl` at 0x7e454 | `(unknown callee) 0x22e50` | `0x149c00` | `?` | `[sp-40]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x7e20c | `C == 0` | `not (C == 0)` |
| `bne` at 0x7e248 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7e25c | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x7e284 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7e2ec | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7e2fc | `[0x149e68] != 0x0` | `[0x149e68] == 0x0` |
| `bne` at 0x7e30c | `[0x149e6c] != 0x0` | `[0x149e6c] == 0x0` |
| `bne` at 0x7e370 | `? != 0x47` | `? == 0x47` |
| `bne` at 0x7e390 | `? != 0x4d` | `? == 0x4d` |
| `bge` at 0x7e3c4 | `? >= [sp-32] (signed, 32-bit)` | `? < [sp-32] (signed, 32-bit)` |
| `beq` at 0x7e404 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7e420 | `[[sp-40]+2]:1 != 0x0` | `[[sp-40]+2]:1 == 0x0` |
| `beq` at 0x7e434 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7e460 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7e47c | `r0` | `?` |

## `EnvTempInt`

- `EnvTempInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e4ac to 0x7e72c.
  Established. Four-byte reads target absolute `0x149e7c` and `0x149e88` (`ldr` at 0x7e4e0, `ldr` at
  0x7e4fc). A later unnamed call to `0x5370c` has candidates `0x0`, `[sp-32]:1`, unknown,
  `[sp-32]:1` (`bl` at 0x7e6bc). The following byte store writes `0x1` at `[sp-28] +0x1` (`strb` at
  0x7e6c8). The unnamed call to `0x22e50` carries `0x149c54`, unknown, `[sp-28]`, `0x1` (`bl` at
  0x7e6d0); its zero result takes the branch past the byte-zero assignment at `[sp-28] +0x1` (`beq`
  at 0x7e6dc, `strb` at 0x7e6e8). Temperature-interrupt roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e4bc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7e4c4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7e4cc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7e4d4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7e4d8 | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e4dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e4e0 | load | 4 | `abs 0x149e7c` | `-` |
| `ldr` at 0x7e4f0 | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e4f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e4fc | load | 4 | `abs 0x149e88` | `-` |
| `ldrb` at 0x7e504 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7e508 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7e514 | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e518 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e520 | load | 4 | `abs 0x149e88` | `-` |
| `ldrb` at 0x7e528 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7e52c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7e53c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7e544 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7e54c | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e550 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e554 | load | 4 | `abs 0x149e7c` | `-` |
| `ldr` at 0x7e560 | load | 4 | `abs 0x7e700` | `-` |
| `str` at 0x7e564 | store | 4 | `sp -0x34` | `0x11168c` |
| `ldrb` at 0x7e568 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e56c | store | 4 | `sp -0x30` | `(r1 & 0xff)` |
| `ldrb` at 0x7e570 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7e574 | store | 4 | `sp -0x2c` | `(r2 & 0xff)` |
| `ldr` at 0x7e57c | load | 4 | `abs 0x7e704` | `-` |
| `ldr` at 0x7e580 | load | 4 | `abs 0x7e708` | `-` |
| `str` at 0x7e590 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7e598 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e5a0 | load | 4 | `abs 0x7e70c` | `-` |
| `str` at 0x7e5a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7e5ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e5b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e5bc | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e5c0 | load | 4 | `abs 0x149e68` | `-` |
| `ldr` at 0x7e5cc | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e5d0 | load | 4 | `abs 0x149e6c` | `-` |
| `ldr` at 0x7e5dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e5e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e5e4 | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e5e8 | load | 4 | `abs 0x7e6fc` | `-` |
| `ldr` at 0x7e5ec | load | 4 | `abs 0x7e710` | `-` |
| `ldr` at 0x7e5f4 | load | 4 | `abs 0x149e68` | `-` |
| `ldr` at 0x7e5f8 | load | 4 | `abs 0x149e6c` | `-` |
| `ldrb` at 0x7e600 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x7e614 | load | 4 | `abs 0x7e714` | `-` |
| `str` at 0x7e618 | store | 4 | `sp -0x34` | `0x1116e8` |
| `str` at 0x7e61c | store | 4 | `sp -0x30` | `ret(0x53874)` |
| `ldrb` at 0x7e620 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e624 | store | 4 | `sp -0x2c` | `[sp-21]:1` |
| `ldrb` at 0x7e628 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7e62c | store | 4 | `sp -0x28` | `[sp-22]:1` |
| `ldr` at 0x7e634 | load | 4 | `abs 0x7e704` | `-` |
| `ldr` at 0x7e638 | load | 4 | `abs 0x7e718` | `-` |
| `str` at 0x7e648 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0x7e650 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x7e660 | load | 4 | `abs 0x7e71c` | `-` |
| `ldr` at 0x7e664 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x7e674 | load | 4 | `abs 0x7e720` | `-` |
| `ldr` at 0x7e678 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e680 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e684 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x7e690 | load | 4 | `abs 0x7e71c` | `-` |
| `ldr` at 0x7e694 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x7e6a4 | load | 4 | `abs 0x7e724` | `-` |
| `ldr` at 0x7e6a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7e6b0 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x7e6c0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x7e6c8 | store | 1 | `[sp-28] +0x1` | `0x1` |
| `ldr` at 0x7e6cc | load | 4 | `abs 0x7e728` | `-` |
| `ldr` at 0x7e6e0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x7e6e8 | store | 1 | `[sp-28] +0x1` | `0x0` |
| `str` at 0x7e6f0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x7e6f4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x7e6f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7e6f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7e6f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e588 | `(unknown callee) 0x21eb8` | `0x1` | `0x111244` | `0x5a2` | `0x0` |
| `bl` at 0x7e5fc | `(unknown callee) 0x2079c` | `0x1116c8` | `?` | `[0x149e68]` | `[0x149e6c]` |
| `bl` at 0x7e60c | `(unknown callee) 0x53874` | `0x0` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x7e640 | `(unknown callee) 0x21eb8` | `0x1` | `0x111244` | `0x5b1` | `0x0` |
| `bl` at 0x7e65c | `(unknown callee) 0x5370c` | `0x0` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x7e67c | `(unknown callee) 0x23fbc` | `0x111720` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7e6ac | `(unknown callee) 0x23fbc` | `0x111734` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7e6bc | `(unknown callee) 0x5370c` | `0x0` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x7e6d0 | `(unknown callee) 0x22e50` | `0x149c54` | `?` | `[sp-28]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x7e4e8 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7e510 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7e534 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x7e55c | `C == 0` | `not (C == 0)` |
| `bne` at 0x7e5b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7e5c8 | `[0x149e68] != 0x0` | `[0x149e68] == 0x0` |
| `bne` at 0x7e5d8 | `[0x149e6c] != 0x0` | `[0x149e6c] == 0x0` |
| `beq` at 0x7e670 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7e68c | `[[sp-28]+1]:1 != 0x0` | `[[sp-28]+1]:1 == 0x0` |
| `beq` at 0x7e6a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7e6dc | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7e6f8 | `r0` | `?` |

## `EnvTemp`

- `EnvTemp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x828fc to 0x82934.
  Established. The unnamed call to `0x1d41c` carries `0x7`, unknown, `r2`, unknown (`bl` at
  0x82928). A following call to `0x829f0` has `ret(0x1d41c)` and three unknown candidates (`bl` at
  0x8292c). The exit retains `ret(0x829f0)` (`ldmdb` at 0x82930). Temperature-control effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8290c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82910 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82930 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82930 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82930 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82928 | `(unknown callee) 0x1d41c` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x8292c | `(unknown callee) 0x829f0` | `ret(0x1d41c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82930 | `r0` | `ret(0x829f0)` |

## `EnvPower`

- `EnvPower`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82934 to 0x8295c.
  Established. The unnamed call to `0x1d41c` has candidates `0x7d`, `r0`, `r2`, `r3` (`bl` at
  0x82950); its result becomes the first candidate of the following unnamed call to `0x829f0`, whose
  other candidates are unknown (`bl` at 0x82954). Power policy and callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82944 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8294c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82958 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82958 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82958 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82950 | `(unknown callee) 0x1d41c` | `0x7d` | `r0` | `r2` | `r3` |
| `bl` at 0x82954 | `(unknown callee) 0x829f0` | `ret(0x1d41c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82958 | `r0` | `ret(0x829f0)` |

## `EnvPowerDown`

- `EnvPowerDown`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82ca4 to 0x82cc4.
  Established. A four-byte stack store retains entry `r0` at `sp -0x14` (`str` at 0x82cb4). The
  unnamed call to `0x7c3a8` carries `r0`, `r1`, `r2`, `r3` (`bl` at 0x82cbc). The exit preserves
  `ret(0x7c3a8)` (`ldmdb` at 0x82cc0). Power-down behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82cb4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82cb8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82cc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82cc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82cc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82cbc | `(unknown callee) 0x7c3a8` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82cc0 | `r0` | `ret(0x7c3a8)` |

## `EnvTempConfigure`

- `EnvTempConfigure`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82d34 to 0x82d60.
  Established. The first unnamed call targets `0x7defc` with entry `r0`, `r1`, `r2`, `r3` (`bl` at
  0x82d48). The following call to `0x7dc24` uses `[sp-20]`, `ret(0x7defc)`, unknown, `ret(0x7defc)`
  (`bl` at 0x82d58); its return is retained at the exit (`ldmdb` at 0x82d5c). Temperature
  configuration semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82d44 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82d50 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82d5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82d5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82d5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82d48 | `(unknown callee) 0x7defc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x82d58 | `(unknown callee) 0x7dc24` | `[sp-20]` | `ret(0x7defc)` | `?` | `ret(0x7defc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82d5c | `r0` | `ret(0x7dc24)` |

## `i2cInitialize`

- `i2cInitialize`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41324 to 0x41384.
  Established. The unnamed calls to `0xd53d0` and `0x414d0` precede a branch taken when
  `ret(0x414d0) == 0x0` (`bl` at 0x41330, `bl` at 0x41334, `beq` at 0x41340). That taken arm skips
  the unnamed call to `0xd6cd4` (`beq` at 0x41340, `bl` at 0x41348). Subsequent calls include unnamed `0x3ca44` and
  `0x45af8`, each with first candidate `0x0` and three unknown candidates (`bl` at 0x41360, `bl` at
  0x41370). Initialization effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x41380 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41380 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41380 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41330 | `(unknown callee) 0xd53d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x41334 | `(unknown callee) 0x414d0` | `ret(0xd53d0)` | `?` | `?` | `?` |
| `bl` at 0x41348 | `(unknown callee) 0xd6cd4` | `0x0` | `?` | `?` | `ret(0x414d0)` |
| `bl` at 0x41350 | `(unknown callee) 0xd23e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41358 | `(unknown callee) 0xcce6c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41360 | `(unknown callee) 0x3ca44` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41368 | `(unknown callee) 0x5cf94` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41370 | `(unknown callee) 0x45af8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41378 | `(unknown callee) 0x42028` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x4137c | `(unknown callee) 0x519ec` | `ret(0x42028)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x41340 | `ret(0x414d0) == 0x0` | `ret(0x414d0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41380 | `r0` | `ret(0x519ec)` |

## `i2c_soft_l1_reset_check`

- `i2c_soft_l1_reset_check`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41384 to 0x414d0.
  Established. The unnamed call to `0x3dd74` has first candidate `0x0` and three unknown candidates
  (`bl` at 0x413dc). A nonzero result takes the branch to `0x41400` (`bne` at 0x413f0). Included
  four-byte assignments at `[sp-20] +0x0` retain `0x109498`, `0x1094b4`, `0x1094d8` and `0x109508`
  in separate arms (`str` at 0x41414, `str` at 0x41430, `str` at 0x41464, `str` at 0x41488). The
  exit carries `[sp-36]` (`ldmdb` at 0x414bc). Reset-check meanings of these stored values are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41394 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4139c | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x413bc | store | 1 | `sp -0x19` | `0x1` |
| `strb` at 0x413d4 | store | 1 | `sp -0x19` | `(ret(0x46990) & 0xff)` |
| `str` at 0x413e4 | store | 4 | `sp -0x20` | `ret(0x3dd74)` |
| `ldr` at 0x413e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x413f8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x41400 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4140c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41410 | load | 4 | `abs 0x414c0` | `-` |
| `str` at 0x41414 | store | 4 | `[sp-20] +0x0` | `0x109498` |
| `ldrb` at 0x4141c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x41428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4142c | load | 4 | `abs 0x414c4` | `-` |
| `str` at 0x41430 | store | 4 | `[sp-20] +0x0` | `0x1094b4` |
| `ldr` at 0x4145c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41460 | load | 4 | `abs 0x414c8` | `-` |
| `str` at 0x41464 | store | 4 | `[sp-20] +0x0` | `0x1094d8` |
| `ldr` at 0x41480 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41484 | load | 4 | `abs 0x414cc` | `-` |
| `str` at 0x41488 | store | 4 | `[sp-20] +0x0` | `0x109508` |
| `ldr` at 0x4148c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41490 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x414a0 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x414ac | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x414b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x414b4 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x414b8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x414bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x414bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x414bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x413a0 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x413cc | `(unknown callee) 0x46990` | `0x0` | `0xff` | `?` | `?` |
| `bl` at 0x413dc | `(unknown callee) 0x3dd74` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x41438 | `(unknown callee) 0x51b08` | `ret(0x3dd74)` | `?` | `?` | `[sp-25]:1` |
| `bl` at 0x4144c | `(unknown callee) 0x52264` | `0x0` | `?` | `?` | `ret(0x51b08)` |
| `bl` at 0x41470 | `(unknown callee) 0x52264` | `0x2` | `?` | `?` | `ret(0x52264)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x413b0 | `? == 0x4f` | `? != 0x4f` |
| `bne` at 0x413f0 | `ret(0x3dd74) != 0x0` | `ret(0x3dd74) == 0x0` |
| `bge` at 0x41408 | `ret(0x3dd74) >= 0x0 (signed, 32-bit)` | `ret(0x3dd74) < 0x0 (signed, 32-bit)` |
| `bne` at 0x41424 | `[sp-25]:1 != 0x0` | `[sp-25]:1 == 0x0` |
| `beq` at 0x41444 | `ret(0x51b08) == 0x0` | `ret(0x51b08) != 0x0` |
| `beq` at 0x41458 | `ret(0x52264) == 0x1` | `ret(0x52264) != 0x1` |
| `bne` at 0x4147c | `ret(0x52264) != 0x1` | `ret(0x52264) == 0x1` |
| `beq` at 0x41498 | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x414bc | `r0` | `[sp-36]` |

## `autofanInit`

- `autofanInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30a48 to 0x30c28.
  Established. A four-byte store at absolute `0x148c48` retains `[sp-28]` (`str` at 0x30abc). A
  branch takes signed-positive `[sp-32]` to `0x30ae4` (`bgt` at 0x30ac8); its fall-through includes
  a four-byte `0x1` assignment at absolute `0x148c60` (`str` at 0x30adc). The unnamed call to
  `0xfb0a8` uses `0x3c`, `[sp-32]`, unknown, unknown, and its result is stored at absolute
  `0x148c5c` (`bl` at 0x30b14, `str` at 0x30b1c). A later four-byte store at absolute `0x148c4c`
  retains `ret(0x1b1b4)` (`str` at 0x30bc4). Automatic fan-control initialization effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30a58 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x30a5c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x30a60 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x30a64 | load | 4 | `abs 0x30c00` | `-` |
| `ldr` at 0x30a68 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x30a74 | load | 4 | `abs 0x30c04` | `-` |
| `str` at 0x30a78 | store | 4 | `sp -0x40` | `0x105bb0` |
| `ldr` at 0x30a7c | load | 4 | `abs 0x30c00` | `-` |
| `ldr` at 0x30a80 | load | 4 | `abs 0x148c44` | `-` |
| `str` at 0x30a84 | store | 4 | `sp -0x3c` | `[0x148c44]` |
| `ldr` at 0x30a88 | load | 4 | `abs 0x30c08` | `-` |
| `ldr` at 0x30a8c | load | 4 | `abs 0x148c48` | `-` |
| `str` at 0x30a90 | store | 4 | `sp -0x38` | `[0x148c48]` |
| `ldr` at 0x30a98 | load | 4 | `abs 0x30c0c` | `-` |
| `ldr` at 0x30aa8 | load | 4 | `abs 0x30c00` | `-` |
| `ldr` at 0x30aac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x30ab0 | store | 4 | `abs 0x148c44` | `?` |
| `ldr` at 0x30ab4 | load | 4 | `abs 0x30c08` | `-` |
| `ldr` at 0x30ab8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x30abc | store | 4 | `abs 0x148c48` | `[sp-28]` |
| `ldr` at 0x30ac0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x30acc | load | 4 | `abs 0x30c10` | `-` |
| `ldr` at 0x30ad4 | load | 4 | `abs 0x30c14` | `-` |
| `str` at 0x30adc | store | 4 | `abs 0x148c60` | `0x1` |
| `ldr` at 0x30ae4 | load | 4 | `abs 0x30c14` | `-` |
| `ldr` at 0x30ae8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x30b04 | store | 4 | `abs 0x148c60` | `?` |
| `ldr` at 0x30b08 | load | 4 | `abs 0x30c18` | `-` |
| `ldr` at 0x30b10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x30b1c | store | 4 | `abs 0x148c5c` | `ret(0xfb0a8)` |
| `strb` at 0x30b38 | store | 1 | `sp -0x2d` | `0x1` |
| `strb` at 0x30b54 | store | 1 | `sp -0x2d` | `0x2` |
| `strb` at 0x30b60 | store | 1 | `sp -0x2d` | `0x0` |
| `str` at 0x30b68 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x30b6c | load | 4 | `abs 0x30c08` | `-` |
| `ldr` at 0x30b70 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30b74 | load | 4 | `abs 0x148c48` | `-` |
| `ldr` at 0x30b84 | load | 4 | `abs 0x30c1c` | `-` |
| `ldr` at 0x30b88 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x30b94 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x30b98 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x30ba0 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x30ba8 | load | 4 | `abs 0x30c20` | `-` |
| `ldr` at 0x30bac | load | 4 | `abs 0x30c08` | `-` |
| `ldr` at 0x30bb0 | load | 4 | `abs 0x148c48` | `-` |
| `str` at 0x30bc4 | store | 4 | `abs 0x148c4c` | `ret(0x1b1b4)` |
| `ldr` at 0x30bc8 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x30bd4 | load | 4 | `abs 0x30c24` | `-` |
| `ldr` at 0x30bdc | load | 4 | `abs 0x30c20` | `-` |
| `ldr` at 0x30be0 | load | 4 | `abs 0x30c08` | `-` |
| `ldr` at 0x30be4 | load | 4 | `abs 0x148c48` | `-` |
| `ldr` at 0x30bec | load | 4 | `abs 0x148c4c` | `-` |
| `ldmdb` at 0x30bfc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x30bfc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x30bfc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x30bfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30aa4 | `(unknown callee) 0x21eb8` | `0x9` | `0x105be0` | `0x49` | `0x0` |
| `bl` at 0x30ad0 | `(unknown callee) 0x202d0` | `0x105bf8` | `?` | `0x148c48` | `[sp-32]` |
| `bl` at 0x30b14 | `(unknown callee) 0xfb0a8` | `0x3c` | `[sp-32]` | `?` | `?` |
| `bl` at 0x30b24 | `(unknown callee) 0x1f26c` | `0x6` | `?` | `?` | `ret(0xfb0a8)` |
| `bl` at 0x30b40 | `(unknown callee) 0x3dce4` | `ret(0x1f26c)` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x30bbc | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x30bd8 | `(unknown callee) 0x202d0` | `0x105c28` | `?` | `?` | `[0x148c4c]` |
| `bl` at 0x30bf8 | `(unknown callee) 0x1b4cc` | `[0x148c4c]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30a70 | `[0x148c44] == 0x0` | `[0x148c44] != 0x0` |
| `bgt` at 0x30ac8 | `[sp-32] > 0x0 (signed, 32-bit)` | `[sp-32] <= 0x0 (signed, 32-bit)` |
| `bne` at 0x30b30 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `beq` at 0x30b4c | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `blt` at 0x30b7c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x30bd0 | `[0x148c4c] != 0x0` | `[0x148c4c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x30bfc | `r0` | `ret(0x1b4cc)` |

## `autofanMonitor`

- `autofanMonitor`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30c28 to 0x32184.
  Established. The initial byte read at absolute `0x148d6c` precedes a branch taken when
  `[0x148d6c]:1 == 0x0` (`ldrb` at 0x30c60, `beq` at 0x30c68). Included four-byte stores at absolute
  `0x1433b4` retain `[0x1433b4]+1` and `0x0` in separate arms (`str` at 0x30ca4, `str` at 0x30cb4).
  Later four-byte assignments at `[sp-32] +0x0`, `[sp-36] +0x0` and `[sp-40] +0x0` retain `[sp-52]`,
  `ret(0xfc3c8)` and `ret(0xfc3c8)` respectively (`str` at 0x31db4, `str` at 0x31dc8, `str` at
  0x31df4). A byte store at absolute `0x2b1400` writes `0x1` (`strb` at 0x31f38). The unnamed call
  to `0x46bc0` has candidates `0x0`, unknown, `[sp-84]:1`, unknown (`bl` at 0x320d8). Control-law
  and measurement-unit interpretations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30c38 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x30c3c | store | 4 | `sp -0x24` | `r1` |
| `str` at 0x30c40 | store | 4 | `sp -0x28` | `r2` |
| `str` at 0x30c48 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x30c50 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x30c58 | store | 4 | `sp -0x144` | `sp-320` |
| `ldr` at 0x30c5c | load | 4 | `abs 0x317b0` | `-` |
| `ldrb` at 0x30c60 | load | 1 | `abs 0x148d6c` | `-` |
| `str` at 0x30c70 | store | 4 | `sp -0x148` | `0x0` |
| `ldr` at 0x30c78 | load | 4 | `abs 0x317b4` | `-` |
| `ldr` at 0x30c7c | load | 4 | `abs 0x317b8` | `-` |
| `ldr` at 0x30c80 | load | 4 | `abs 0x148c5c` | `-` |
| `ldr` at 0x30c88 | load | 4 | `abs 0x1433b4` | `-` |
| `ldr` at 0x30c94 | load | 4 | `abs 0x317b4` | `-` |
| `ldr` at 0x30c98 | load | 4 | `abs 0x317b4` | `-` |
| `ldr` at 0x30c9c | load | 4 | `abs 0x1433b4` | `-` |
| `str` at 0x30ca4 | store | 4 | `abs 0x1433b4` | `[0x1433b4]+1` |
| `ldr` at 0x30cac | load | 4 | `abs 0x317b4` | `-` |
| `str` at 0x30cb4 | store | 4 | `abs 0x1433b4` | `0x0` |
| `str` at 0x30cbc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x30cc0 | load | 4 | `abs 0x317bc` | `-` |
| `ldr` at 0x30cc4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30cc8 | load | 4 | `abs 0x148c48` | `-` |
| `str` at 0x30cdc | store | 4 | `sp -0x50` | `0x2` |
| `ldr` at 0x30ce0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30cf4 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30cf8 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x30d00 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30d0c | load | 4 | `abs 0x317c0` | `-` |
| `str` at 0x30d10 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x30d18 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x30d20 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x30d24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30d38 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30d3c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x30d44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30d48 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x30d58 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30d6c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30d70 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x30d78 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x30d80 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x30d84 | store | 1 | `sp -0x55` | `?` |
| `ldrb` at 0x30d88 | load | 1 | `sp -0x55` | `-` |
| `ldrb` at 0x30da4 | load | 1 | `sp -0x55` | `-` |
| `ldr` at 0x30db8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x30dc4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x30dd4 | store | 4 | `sp -0x68` | `ret(0xfbde8)` |
| `ldr` at 0x30dd8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x30ddc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x30de8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x30dec | store | 4 | `sp -0x40` | `[sp-60]` |
| `ldr` at 0x30df0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x30df8 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0x30e00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30e14 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30e18 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x30e20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30e30 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x30e3c | store | 4 | `sp -0x64` | `ret(0xfbfb0)` |
| `ldr` at 0x30e44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30e58 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30e5c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x30e64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30e74 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x30e84 | store | 4 | `sp -0x64` | `ret(0xfc18c)` |
| `ldr` at 0x30e88 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30e90 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x30e94 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x30e9c | load | 4 | `abs 0x317b4` | `-` |
| `ldr` at 0x30ea0 | load | 4 | `abs 0x1433b4` | `-` |
| `ldr` at 0x30eb4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x30eb8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x30ebc | load | 4 | `abs 0x317c0` | `-` |
| `str` at 0x30ec0 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x30ec8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x30ecc | load | 4 | `abs 0x317b8` | `-` |
| `ldr` at 0x30ed0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30ed4 | load | 4 | `abs 0x148c5c` | `-` |
| `ldr` at 0x30ee4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30eec | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x30ef0 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x30ef8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30f0c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x30f10 | load | 4 | `(unknown)` | `-` |
| `str` at 0x30f1c | store | 4 | `sp -0x68` | `ret(0xfbde8)` |
| `ldr` at 0x30f20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x30f28 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x30f30 | load | 4 | `abs 0x317b8` | `-` |
| `ldr` at 0x30f34 | load | 4 | `abs 0x148c5c` | `-` |
| `ldr` at 0x30f40 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x30f50 | store | 4 | `sp -0x64` | `ret(0xfc18c)` |
| `ldr` at 0x30f54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30f68 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30f6c | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x30f74 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x30f88 | store | 4 | `sp -0x164` | `?` |
| `ldr` at 0x30f8c | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x30f90 | load | 4 | `abs 0x317c4` | `-` |
| `ldr` at 0x30f94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x30f9c | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x30fac | load | 4 | `sp -0x144` | `-` |
| `str` at 0x30fb4 | store | 4 | `sp -0x144` | `([sp-324] + ret(0x1b2bc))` |
| `ldr` at 0x30fb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30fcc | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x30fd0 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x30fd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30fec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31000 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31004 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x3100c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3101c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31030 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31034 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x3103c | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x31040 | load | 4 | `abs 0x317c8` | `-` |
| `ldr` at 0x31044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3104c | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x3105c | load | 4 | `sp -0x144` | `-` |
| `str` at 0x31064 | store | 4 | `sp -0x144` | `([sp-324] + ret(0x1b2bc))` |
| `ldr` at 0x31068 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3107c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31080 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31088 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3109c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x310a0 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x310a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x310bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x310d0 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x310d4 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x310dc | load | 1 | `sp -0x58` | `-` |
| `ldr` at 0x310f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x310fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31110 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31114 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x31120 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31138 | store | 4 | `sp -0x54` | `ret(0xfc3c8)` |
| `ldrb` at 0x3113c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31144 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x31158 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31168 | store | 4 | `sp -0x54` | `ret(0x32f04)` |
| `ldrb` at 0x3116c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31174 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31188 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3119c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x311a0 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x311a8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x311ac | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x311b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x311c8 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x311cc | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x311d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x311e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x311f8 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x311fc | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31204 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31214 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x31230 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31244 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31248 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31250 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31254 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x3125c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31270 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31274 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x3127c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31288 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0x312a4 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x312b4 | store | 4 | `sp -0x54` | `ret(0x32f04)` |
| `ldr` at 0x312bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x312d0 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x312d4 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x312dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x312e0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x312ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31300 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31304 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x31310 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31328 | store | 4 | `sp -0x54` | `ret(0xfc3c8)` |
| `ldrb` at 0x3132c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31334 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x31348 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31358 | store | 4 | `sp -0x54` | `ret(0x32f04)` |
| `ldr` at 0x31360 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31374 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31378 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31380 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31394 | store | 4 | `sp -0x14c` | `?` |
| `str` at 0x313a0 | store | 4 | `sp -0x14c` | `0x1` |
| `ldr` at 0x313a4 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0x313a8 | store | 4 | `sp -0x38` | `[sp-332]` |
| `ldr` at 0x313ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x313b4 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x313b8 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x313c0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x313c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x313d0 | store | 4 | `sp -0x6c` | `ret(0xfbde4)` |
| `ldr` at 0x313d4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x313e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x313f8 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x313fc | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31404 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x31424 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x31424 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31458 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31460 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31464 | load | 4 | `abs 0x148c4c` | `-` |
| `ldrb` at 0x3146c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31478 | load | 1 | `sp -0x57` | `-` |
| `str` at 0x3147c | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x31484 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x31488 | load | 4 | `abs 0x317c0` | `-` |
| `ldr` at 0x314a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x314a8 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x314ac | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x314b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x314b8 | load | 4 | `abs 0x317c0` | `-` |
| `ldr` at 0x314d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x314d8 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x314dc | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x314e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x314ec | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x314f0 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x314f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31500 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31504 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31518 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x3151c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31524 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31530 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31538 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x3153c | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31544 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31558 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x3155c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31564 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31568 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x31588 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x31594 | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x31598 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x3159c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x315b0 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x315b4 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x315bc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x315c0 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x315c8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x315d4 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x315dc | load | 1 | `sp -0x57` | `-` |
| `str` at 0x315e0 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x315e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x315f0 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x315f4 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x315fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31604 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31608 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31610 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31618 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3161c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31630 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31634 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x3163c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31648 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31650 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31654 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x3165c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31670 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31674 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x3167c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31680 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x316a0 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x316ac | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x316b0 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x316b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x316c8 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x316cc | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x316d4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x316d8 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x316e0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x316ec | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x316f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31708 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x3170c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31714 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31724 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x31740 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31748 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x3174c | load | 4 | `abs 0x148c4c` | `-` |
| `strb` at 0x31758 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x3175c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x31760 | load | 4 | `abs 0x317c0` | `-` |
| `ldr` at 0x31778 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31780 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31784 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x3178c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31790 | load | 4 | `abs 0x317c0` | `-` |
| `ldr` at 0x31810 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31818 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x3181c | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31824 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3182c | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31830 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31838 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31840 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31844 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31858 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x3185c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31864 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31870 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31878 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x3187c | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31884 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31898 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x3189c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x318a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x318a8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x318c8 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x318d4 | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x318d8 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x318dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x318f0 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x318f4 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x318fc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31900 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x31908 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x31914 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x3191c | load | 1 | `sp -0x57` | `-` |
| `str` at 0x31920 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x31928 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31930 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31934 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x3193c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31944 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31948 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31950 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31958 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3195c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31970 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31974 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x3197c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31988 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31990 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31994 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x3199c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x319b0 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x319b4 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x319bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x319c0 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x319e0 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x319ec | store | 1 | `sp -0x56` | `0x0` |
| `ldr` at 0x319f0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x319fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31a10 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31a14 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31a1c | load | 1 | `(unknown)` | `-` |
| `str` at 0x31a30 | store | 4 | `sp -0x60` | `ret(0xfbfb0)` |
| `ldr` at 0x31a34 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x31a38 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x31a48 | load | 4 | `abs 0x317cc` | `-` |
| `str` at 0x31a54 | store | 4 | `sp -0x70` | `ret(0xfc18c)` |
| `ldr` at 0x31a58 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x31a5c | load | 4 | `abs 0x317d0` | `-` |
| `ldr` at 0x31a74 | load | 4 | `abs 0x317d0` | `-` |
| `str` at 0x31a78 | store | 4 | `sp -0x70` | `0x3f800000` |
| `ldr` at 0x31a7c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31a90 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31a94 | load | 4 | `abs 0x148c44` | `-` |
| `ldrsb` at 0x31a9c | load | 1 | `(unknown)` | `-` |
| `str` at 0x31aa8 | store | 4 | `sp -0x150` | `ret(0xfbfb0)` |
| `ldrsb` at 0x31aac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31ab8 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0x31abc | load | 4 | `abs 0x317d4` | `-` |
| `str` at 0x31ac8 | store | 4 | `sp -0x150` | `ret(0xfbde8)` |
| `ldr` at 0x31acc | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0x31ad0 | load | 4 | `sp -0x70` | `-` |
| `strb` at 0x31ae8 | store | 1 | `sp -0x59` | `(ret(0xfc430) & 0xff)` |
| `ldrb` at 0x31aec | load | 1 | `sp -0x57` | `-` |
| `ldrb` at 0x31af0 | load | 1 | `sp -0x59` | `-` |
| `ldrb` at 0x31af4 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x31afc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x31b08 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31b0c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31b14 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31b28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31b3c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31b40 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31b48 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31b4c | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31b54 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31b5c | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x31b70 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31b80 | store | 4 | `sp -0x54` | `ret(0x32f04)` |
| `str` at 0x31ba0 | store | 4 | `sp -0x44` | `0x5` |
| `str` at 0x31ba8 | store | 4 | `sp -0x48` | `0xa` |
| `str` at 0x31bb4 | store | 4 | `sp -0x44` | `0x9` |
| `str` at 0x31bbc | store | 4 | `sp -0x48` | `0xd` |
| `ldr` at 0x31bc0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31bd4 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31bd8 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31be0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31be4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x31bf8 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0x31c14 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31c1c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x31c34 | store | 4 | `sp -0x50` | `0x10` |
| `ldr` at 0x31c3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31c50 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31c54 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31c5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31c60 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x31c74 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0x31c90 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31c98 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x31cb0 | store | 4 | `sp -0x50` | `0x8` |
| `ldrb` at 0x31cb8 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0x31cbc | load | 1 | `sp -0x54` | `-` |
| `str` at 0x31cdc | store | 4 | `sp -0x50` | `0x4` |
| `ldrb` at 0x31ce4 | load | 1 | `sp -0x57` | `-` |
| `str` at 0x31ce8 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x31cec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31cf4 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31cf8 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31d00 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x31d04 | store | 4 | `(unknown)` | `[sp-100]` |
| `ldr` at 0x31d08 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31d10 | load | 4 | `abs 0x317d8` | `-` |
| `ldr` at 0x31d14 | load | 4 | `abs 0x148c4c` | `-` |
| `ldr` at 0x31d1c | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x31d20 | store | 4 | `(unknown)` | `[sp-108]` |
| `ldrb` at 0x31d24 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31d2c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31d40 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31d54 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31d58 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31d60 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31d64 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31d6c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31d74 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x31d88 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31d98 | store | 4 | `sp -0x54` | `ret(0x32f04)` |
| `ldr` at 0x31d9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x31da0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x31dac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x31db0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x31db4 | store | 4 | `[sp-32] +0x0` | `[sp-52]` |
| `ldr` at 0x31db8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x31dbc | load | 4 | `sp -0x64` | `-` |
| `str` at 0x31dc8 | store | 4 | `[sp-36] +0x0` | `ret(0xfc3c8)` |
| `ldr` at 0x31dcc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x31dd0 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0x31dd4 | load | 1 | `sp -0x54` | `-` |
| `str` at 0x31df4 | store | 4 | `[sp-40] +0x0` | `ret(0xfc3c8)` |
| `ldr` at 0x31df8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x31dfc | store | 4 | `sp -0x30` | `[sp-80]` |
| `ldr` at 0x31e00 | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x31e04 | load | 4 | `abs 0x317dc` | `-` |
| `ldr` at 0x31e08 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31e10 | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x31e20 | load | 4 | `sp -0x144` | `-` |
| `str` at 0x31e28 | store | 4 | `sp -0x144` | `([sp-324] + ret(0x1b2bc))` |
| `ldr` at 0x31e2c | load | 4 | `abs 0x317f0` | `-` |
| `ldr` at 0x31e30 | load | 4 | `abs 0x1433b0` | `-` |
| `str` at 0x31e40 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x31e44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31e58 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31e5c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x31e64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31e68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31e78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31e8c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31e90 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x31e9c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x31ea8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x31eac | store | 1 | `sp -0x71` | `?` |
| `ldrb` at 0x31eb0 | load | 1 | `sp -0x71` | `-` |
| `ldr` at 0x31ec4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31ed8 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x31edc | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x31ee4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x31ee8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31ef4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x31f00 | load | 4 | `abs 0x317ec` | `-` |
| `ldr` at 0x31f04 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x31f0c | load | 1 | `[sp-52] +0x148c64` | `-` |
| `ldr` at 0x31f18 | load | 4 | `abs 0x317ec` | `-` |
| `ldr` at 0x31f1c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x31f28 | store | 1 | `[sp-52] +0x148c64` | `0x1` |
| `ldr` at 0x31f30 | load | 4 | `abs 0x317f4` | `-` |
| `strb` at 0x31f38 | store | 1 | `abs 0x2b1400` | `0x1` |
| `ldrb` at 0x31f3c | load | 1 | `sp -0x71` | `-` |
| `ldr` at 0x31f50 | load | 4 | `abs 0x317e0` | `-` |
| `str` at 0x31f54 | store | 4 | `sp -0x164` | `0x105c60` |
| `ldr` at 0x31f58 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x31f5c | store | 4 | `sp -0x160` | `[sp-52]` |
| `ldr` at 0x31f60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31f64 | store | 4 | `sp -0x15c` | `[sp-44]` |
| `str` at 0x31f68 | store | 4 | `sp -0x158` | `ret(0x473a0)` |
| `ldr` at 0x31f6c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x31f70 | store | 4 | `sp -0x154` | `[sp-120]` |
| `ldr` at 0x31f78 | load | 4 | `abs 0x317e4` | `-` |
| `ldr` at 0x31f7c | load | 4 | `abs 0x317e8` | `-` |
| `ldr` at 0x31f8c | load | 4 | `abs 0x317ec` | `-` |
| `ldr` at 0x31f90 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x31f9c | store | 1 | `[sp-52] +0x148c64` | `0x0` |
| `ldr` at 0x31fa0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31fa8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x31fb0 | load | 4 | `abs 0x317f0` | `-` |
| `ldr` at 0x31fb4 | load | 4 | `abs 0x317f0` | `-` |
| `ldr` at 0x31fb8 | load | 4 | `abs 0x1433b0` | `-` |
| `str` at 0x31fc0 | store | 4 | `abs 0x1433b0` | `[0x1433b0]-1` |
| `ldr` at 0x31fc4 | load | 4 | `abs 0x317f4` | `-` |
| `ldrb` at 0x31fc8 | load | 1 | `abs 0x2b1400` | `-` |
| `ldr` at 0x31fd4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31fe4 | store | 4 | `sp -0x30` | `0x20` |
| `str` at 0x31fec | store | 4 | `sp -0x50` | `0x20` |
| `ldrb` at 0x31ff0 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x32000 | store | 4 | `sp -0x54` | `ret(0x32f04)` |
| `ldrb` at 0x32004 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x32008 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x32014 | load | 4 | `abs 0x31808` | `-` |
| `ldr` at 0x32018 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x32028 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3203c | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x32040 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32048 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x3204c | load | 4 | `abs 0x317f8` | `-` |
| `ldr` at 0x32050 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32054 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x32060 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x32064 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x32078 | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x3207c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32084 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32088 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x32098 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x320ac | load | 4 | `abs 0x317fc` | `-` |
| `ldr` at 0x320b0 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x320bc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x320c8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x320cc | load | 1 | `sp -0x54` | `-` |
| `ldr` at 0x320dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x320e4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x320ec | load | 4 | `abs 0x31800` | `-` |
| `ldr` at 0x320f0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x320f8 | load | 1 | `sp -0x50` | `-` |
| `strb` at 0x320fc | store | 1 | `[sp-52] +0x148c50` | `([sp-80]:1 & 0xff)` |
| `ldr` at 0x32100 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x32108 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x32110 | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x32114 | load | 4 | `abs 0x31804` | `-` |
| `ldr` at 0x3211c | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x3212c | load | 4 | `sp -0x144` | `-` |
| `str` at 0x32134 | store | 4 | `sp -0x144` | `([sp-324] + ret(0x1b2bc))` |
| `ldr` at 0x32138 | load | 4 | `abs 0x31808` | `-` |
| `ldr` at 0x3213c | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x32150 | load | 4 | `abs 0x3180c` | `-` |
| `ldr` at 0x3215c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3216c | store | 4 | `sp -0x148` | `0x2` |
| `ldr` at 0x32174 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x32178 | store | 4 | `sp -0x148` | `[sp-48]` |
| `ldr` at 0x3217c | load | 4 | `sp -0x148` | `-` |
| `ldmdb` at 0x32180 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x32180 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x32180 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x32180 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x32180 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x32180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30d94 | `(unknown callee) 0x53644` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x30db4 | `(unknown callee) 0x537cc` | `0x0` | `[sp-85]:1` | `sp-60` | `[sp-85]:1` |
| `bl` at 0x30dbc | `(unknown callee) 0xfbfb0` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x30dcc | `(unknown callee) 0xfbde8` | `[sp-104]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x30e34 | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x30e6c | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x30e7c | `(unknown callee) 0xfc18c` | `[sp-104]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x30f14 | `(unknown callee) 0xfbde8` | `?` | `?` | `0x8` | `?` |
| `bl` at 0x30f38 | `(unknown callee) 0xfbfb0` | `[0x148c5c]` | `?` | `?` | `0x148c5c` |
| `bl` at 0x30f48 | `(unknown callee) 0xfc18c` | `[sp-104]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x30f78 | `(unknown callee) 0xfb644` | `ret(0xfc18c)` | `?` | `?` | `[0x148c44]` |
| `bl` at 0x30f98 | `(unknown callee) 0x1b52c` | `[sp-324]` | `0x105c44` | `?` | `ret(0xfb644)` |
| `bl` at 0x30fa0 | `(unknown callee) 0x1b2bc` | `[sp-324]` | `?` | `?` | `?` |
| `bl` at 0x30fe8 | `(unknown callee) 0x47068` | `0x0` | `?` | `sp-87` | `?` |
| `bl` at 0x31048 | `(unknown callee) 0x1b52c` | `[sp-324]` | `0x105c50` | `?` | `?` |
| `bl` at 0x31050 | `(unknown callee) 0x1b2bc` | `[sp-324]` | `?` | `?` | `?` |
| `bl` at 0x310b8 | `(unknown callee) 0x47068` | `0x0` | `?` | `sp-88` | `?` |
| `bl` at 0x310e4 | `(unknown callee) 0xfbfb0` | `[sp-88]:1` | `?` | `?` | `[sp-88]:1` |
| `bl` at 0x310f4 | `(unknown callee) 0xfbffc` | `ret(0xfbfb0)` | `?` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x31124 | `(unknown callee) 0xfbde8` | `ret(0xfbffc)` | `?` | `?` | `?` |
| `bl` at 0x31130 | `(unknown callee) 0xfc3c8` | `ret(0xfbde8)` | `?` | `?` | `ret(0xfbde8)` |
| `bl` at 0x31148 | `(unknown callee) 0x32f8c` | `[sp-52]:1` | `ret(0xfc3c8)` | `?` | `[sp-52]:1` |
| `bl` at 0x31160 | `(unknown callee) 0x32f04` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x31178 | `(unknown callee) 0x3304c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x3120c | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x3121c | `(unknown callee) 0xfc334` | `[sp-100]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x31280 | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31290 | `(unknown callee) 0xfc32c` | `[sp-100]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x312ac | `(unknown callee) 0x32f04` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x312e4 | `(unknown callee) 0xfbffc` | `?` | `[sp-100]` | `?` | `?` |
| `bl` at 0x31314 | `(unknown callee) 0xfbde8` | `ret(0xfbffc)` | `?` | `?` | `?` |
| `bl` at 0x31320 | `(unknown callee) 0xfc3c8` | `ret(0xfbde8)` | `?` | `?` | `ret(0xfbde8)` |
| `bl` at 0x31338 | `(unknown callee) 0x32f8c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31350 | `(unknown callee) 0x32f04` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x313c8 | `(unknown callee) 0xfbde4` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x313d8 | `(unknown callee) 0xfb644` | `[sp-100]` | `?` | `?` | `ret(0xfbde4)` |
| `bl` at 0x3140c | `(unknown callee) 0xfb618` | `?` | `?` | `?` | `?` |
| `bl` at 0x31428 | `(unknown callee) 0xfb308` | `ret(0xfb618)` | `?` | `?` | `?` |
| `bl` at 0x31444 | `(unknown callee) 0xfbb5c` | `ret(0xfb644)` | `?` | `ret(0xfb308)` | `?` |
| `bl` at 0x3148c | `(unknown callee) 0xfc32c` | `[sp-108]` | `0x0` | `?` | `?` |
| `bl` at 0x314bc | `(unknown callee) 0xfc32c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x31574 | `(unknown callee) 0xfb218` | `?` | `?` | `?` | `?` |
| `bl` at 0x3168c | `(unknown callee) 0xfb218` | `?` | `?` | `?` | `?` |
| `bl` at 0x3171c | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x3172c | `(unknown callee) 0xfc32c` | `[sp-100]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x31764 | `(unknown callee) 0xfc334` | `[sp-108]` | `0x0` | `?` | `0x1` |
| `bl` at 0x31794 | `(unknown callee) 0xfc334` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x318b4 | `(unknown callee) 0xfb218` | `?` | `?` | `?` | `?` |
| `bl` at 0x319cc | `(unknown callee) 0xfb218` | `?` | `?` | `?` | `?` |
| `bl` at 0x319f4 | `(unknown callee) 0xfc3c8` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x31a28 | `(unknown callee) 0xfbfb0` | `?` | `ret(0xfc3c8)` | `?` | `?` |
| `bl` at 0x31a3c | `(unknown callee) 0xfbffc` | `ret(0xfbfb0)` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x31a4c | `(unknown callee) 0xfc18c` | `ret(0xfbffc)` | `0x41000000` | `?` | `ret(0xfbffc)` |
| `bl` at 0x31a60 | `(unknown callee) 0xfc334` | `ret(0xfc18c)` | `0x3f800000` | `?` | `ret(0xfc18c)` |
| `bl` at 0x31aa4 | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31ac0 | `(unknown callee) 0xfbde8` | `ret(0xfbfb0)` | `0x43800000` | `?` | `?` |
| `bl` at 0x31ad4 | `(unknown callee) 0xfbffc` | `?` | `[sp-112]` | `?` | `?` |
| `bl` at 0x31ae0 | `(unknown callee) 0xfc430` | `ret(0xfbffc)` | `?` | `?` | `ret(0xfbffc)` |
| `bl` at 0x31b18 | `(unknown callee) 0x3304c` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x31b60 | `(unknown callee) 0x32f8c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31b78 | `(unknown callee) 0x32f04` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x31b84 | `(unknown callee) 0x4f4d0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31bf0 | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31c00 | `(unknown callee) 0xfc32c` | `[sp-100]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x31c20 | `(unknown callee) 0x32f8c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31c6c | `(unknown callee) 0xfbfb0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31c7c | `(unknown callee) 0xfc32c` | `[sp-100]` | `ret(0xfbfb0)` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x31c9c | `(unknown callee) 0x32f8c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31cc8 | `(unknown callee) 0x33250` | `[sp-52]:1` | `[sp-84]:1` | `[sp-84]:1` | `[sp-52]:1` |
| `bl` at 0x31d30 | `(unknown callee) 0x3304c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31d78 | `(unknown callee) 0x32f8c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31d90 | `(unknown callee) 0x32f04` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x31dc0 | `(unknown callee) 0xfc3c8` | `[sp-100]` | `?` | `[sp-32]` | `[sp-52]` |
| `bl` at 0x31de0 | `(unknown callee) 0x33130` | `[sp-52]:1` | `[sp-84]:1` | `[sp-84]:1` | `[sp-52]:1` |
| `bl` at 0x31dec | `(unknown callee) 0xfc3c8` | `ret(0x33130)` | `?` | `?` | `ret(0x33130)` |
| `bl` at 0x31e0c | `(unknown callee) 0x1b52c` | `[sp-324]` | `0x105c58` | `[sp-84]` | `?` |
| `bl` at 0x31e14 | `(unknown callee) 0x1b2bc` | `[sp-324]` | `?` | `?` | `?` |
| `bl` at 0x31ec0 | `(unknown callee) 0x471f4` | `0x0` | `[sp-113]:1` | `sp-120` | `[sp-113]:1` |
| `bl` at 0x31f48 | `(unknown callee) 0x473a0` | `0x0` | `[sp-113]:1` | `0x2b1400` | `[sp-113]:1` |
| `bl` at 0x31f84 | `(unknown callee) 0x21eb8` | `0x1` | `0x105be0` | `0x189` | `0x0` |
| `bl` at 0x31ff8 | `(unknown callee) 0x32f04` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x32058 | `(unknown callee) 0x2079c` | `0x105c88` | `?` | `[sp-87]:1` | `[sp-84]` |
| `bl` at 0x320d8 | `(unknown callee) 0x46bc0` | `0x0` | `?` | `[sp-84]:1` | `?` |
| `bl` at 0x32118 | `(unknown callee) 0x1b52c` | `[sp-324]` | `0x105c98` | `?` | `[0x148c48]` |
| `bl` at 0x32120 | `(unknown callee) 0x1b2bc` | `[sp-324]` | `?` | `?` | `?` |
| `bl` at 0x32158 | `(unknown callee) 0x23fbc` | `0x105c9c` | `sp-320` | `ret(0x1b2bc)` | `sp-320` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30c68 | `[0x148d6c]:1 == 0x0` | `[0x148d6c]:1 != 0x0` |
| `bge` at 0x30c90 | `[0x1433b4] >= [0x148c5c]-1 (signed, 32-bit)` | `[0x1433b4] < [0x148c5c]-1 (signed, 32-bit)` |
| `blt` at 0x30cd0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x30d08 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x30d50 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x30da0 | `ret(0x53644) == 0x0` | `ret(0x53644) != 0x0` |
| `ble` at 0x30de4 | `[sp-60] <= [sp-64] (signed, 32-bit)` | `[sp-60] > [sp-64] (signed, 32-bit)` |
| `beq` at 0x30e2c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x30edc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x31018 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31154 | `ret(0x32f8c) == 0x0` | `ret(0x32f8c) != 0x0` |
| `beq` at 0x31184 | `ret(0x3304c) == 0x0` | `ret(0x3304c) != 0x0` |
| `beq` at 0x311e0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x31228 | `ret(0xfc334) <= 0x0 (signed, 32-bit)` | `ret(0xfc334) > 0x0 (signed, 32-bit)` |
| `bgt` at 0x3129c | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31344 | `ret(0x32f8c) == 0x0` | `ret(0x32f8c) != 0x0` |
| `beq` at 0x3138c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x31450 | `ret(0xfbb5c) < 0x0 (signed, 32-bit)` | `ret(0xfbb5c) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x31474 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x31498 | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `bgt` at 0x314c8 | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x3152c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31644 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x31738 | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `blt` at 0x31770 | `ret(0xfc334) < 0x0 (signed, 32-bit)` | `ret(0xfc334) >= 0x0 (signed, 32-bit)` |
| `blt` at 0x317a0 | `ret(0xfc334) < 0x0 (signed, 32-bit)` | `ret(0xfc334) >= 0x0 (signed, 32-bit)` |
| `beq` at 0x3186c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31984 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x31a6c | `ret(0xfc334) <= 0x0 (signed, 32-bit)` | `ret(0xfc334) > 0x0 (signed, 32-bit)` |
| `bge` at 0x31ab4 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `beq` at 0x31b24 | `ret(0x3304c) == 0x0` | `ret(0x3304c) != 0x0` |
| `beq` at 0x31b6c | `ret(0x32f8c) == 0x0` | `ret(0x32f8c) != 0x0` |
| `bhi` at 0x31b98 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `bgt` at 0x31c0c | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31c2c | `ret(0x32f8c) == 0x0` | `ret(0x32f8c) != 0x0` |
| `bgt` at 0x31c88 | `ret(0xfc32c) > 0x0 (signed, 32-bit)` | `ret(0xfc32c) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31ca8 | `ret(0x32f8c) == 0x0` | `ret(0x32f8c) != 0x0` |
| `beq` at 0x31cd4 | `ret(0x33250) == 0x0` | `ret(0x33250) != 0x0` |
| `beq` at 0x31d3c | `ret(0x3304c) == 0x0` | `ret(0x3304c) != 0x0` |
| `beq` at 0x31d84 | `ret(0x32f8c) == 0x0` | `ret(0x32f8c) != 0x0` |
| `bge` at 0x31da8 | `[sp-48] >= [sp-80] (signed, 32-bit)` | `[sp-48] < [sp-80] (signed, 32-bit)` |
| `bgt` at 0x31e38 | `[0x1433b0] > 0x0 (signed, 32-bit)` | `[0x1433b0] <= 0x0 (signed, 32-bit)` |
| `blt` at 0x31e70 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0x31ef0 | `[sp-120] >= ? (unsigned, 32-bit)` | `[sp-120] < ? (unsigned, 32-bit)` |
| `bne` at 0x31efc | `[sp-120] != 0x0` | `[sp-120] == 0x0` |
| `bne` at 0x31f14 | `[[sp-52]+1346660]:1 != 0x0` | `[[sp-52]+1346660]:1 == 0x0` |
| `beq` at 0x31fd0 | `[0x2b1400]:1 == 0x0` | `[0x2b1400]:1 != 0x0` |
| `bgt` at 0x31fdc | `[sp-48] > 0x3 (signed, 32-bit)` | `[sp-48] <= 0x3 (signed, 32-bit)` |
| `beq` at 0x32010 | `[sp-84] == [sp-87]:1` | `[sp-84] != [sp-87]:1` |
| `beq` at 0x32024 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x32090 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x32148 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x32164 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x32180 | `r0` | `?` |

## `autofanEnv`

- `autofanEnv`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32184 to 0x32718.
  Established. The unnamed call to `0x30c28` carries `sp-44`, `sp-48`, `sp-52`, `sp-44` (`bl` at
  0x321e0); its result supplies the conditional computed transfer taken for `ret(0x30c28)-2 <= 0x1e
  (unsigned, 32-bit)`, with unresolved destination (`ldrls` at 0x321f8). Known-edge four-byte stores
  include `0x5` at absolute `0x1433b0` and `0x2` at absolute `0x1433b8` (`str` at 0x32604, `str` at
  0x32610). The later unnamed call to `0x33384` has candidates `[0x148c60]`, unknown, unknown,
  `0x148c60` (`bl` at 0x326b8). Event semantics and selector-only effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x32194 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x32198 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x3219c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x321a0 | store | 4 | `sp -0x8c` | `r1` |
| `str` at 0x321e8 | store | 4 | `sp -0x28` | `ret(0x30c28)` |
| `ldr` at 0x321ec | load | 4 | `sp -0x28` | `-` |
| `ldrls` at 0x321f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x325d4 | load | 4 | `abs 0x32700` | `-` |
| `ldr` at 0x325d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x325e0 | load | 4 | `abs 0x326d0` | `-` |
| `ldr` at 0x325e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x325e8 | store | 4 | `abs 0x1433b8` | `[sp-40]` |
| `ldr` at 0x325f0 | load | 4 | `abs 0x32704` | `-` |
| `strb` at 0x325f8 | store | 1 | `abs 0x2b1400` | `0x0` |
| `ldr` at 0x325fc | load | 4 | `abs 0x32708` | `-` |
| `str` at 0x32604 | store | 4 | `abs 0x1433b0` | `0x5` |
| `ldr` at 0x32608 | load | 4 | `abs 0x326d0` | `-` |
| `str` at 0x32610 | store | 4 | `abs 0x1433b8` | `0x2` |
| `str` at 0x3262c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x32630 | load | 4 | `abs 0x3270c` | `-` |
| `ldr` at 0x32634 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x32638 | load | 4 | `abs 0x148c48` | `-` |
| `ldr` at 0x32648 | load | 4 | `abs 0x32710` | `-` |
| `ldr` at 0x3264c | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x32658 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x3265c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x32664 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `str` at 0x32670 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x32674 | load | 4 | `abs 0x3270c` | `-` |
| `ldr` at 0x32678 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3267c | load | 4 | `abs 0x148c48` | `-` |
| `ldr` at 0x3268c | load | 4 | `abs 0x32710` | `-` |
| `ldr` at 0x32690 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3269c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x326a0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x326a8 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x326b0 | load | 4 | `abs 0x32714` | `-` |
| `ldr` at 0x326b4 | load | 4 | `abs 0x148c60` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x321a4 | `(unknown callee) 0x3dce4` | `?` | `?` | `?` | `?` |
| `bl` at 0x321b8 | `(unknown callee) 0x1f26c` | `0x6` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x321e0 | `(unknown callee) 0x30c28` | `sp-44` | `sp-48` | `sp-52` | `sp-44` |
| `bl` at 0x325dc | `(unknown callee) 0x2079c` | `0x105d74` | `ret(0x30c28)` | `?` | `ret(0x30c28)-2` |
| `bl` at 0x32618 | `(unknown callee) 0x1f26c` | `0x6` | `?` | `0x1433b8` | `0x2` |
| `bl` at 0x326b8 | `(unknown callee) 0x33384` | `[0x148c60]` | `?` | `?` | `0x148c60` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x321b0 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `beq` at 0x321c4 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `ldrls` at 0x321f8 | `ret(0x30c28)-2 <= 0x1e (unsigned, 32-bit)` | `ret(0x30c28)-2 > 0x1e (unsigned, 32-bit)` |
| `bne` at 0x32624 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `blt` at 0x32640 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x32684 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x321f8).

## `autofanEnvQuery`

- `autofanEnvQuery`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32718 to 0x32e9c.
  Established. The conditional computed transfer has an unresolved destination and unknown operands
  in its printed `ls flags condition (operands unknown)` predicate (`ldrls` at 0x327b4). Later
  unnamed calls to `0x1b52c` include `sp-76`, `0x105ec8`, `0x105ecc`, `sp-76`, and `sp-108`,
  `0x105ed8`, `[sp-56]`, `ret(0xfbbf8)` (`bl` at 0x328d4, `bl` at 0x32c14). Four-byte stack stores
  retain `sp-76`, `sp-120`, `sp-108` and `sp-92` at `sp -0x98`, `-0x94`, `-0x90` and `-0x8c` (`str`
  at 0x32e18, `str` at 0x32e20, `str` at 0x32e28, `str` at 0x32e30). The exit carries `ret(0x1b598)`
  (`ldmdb` at 0x32e98). Report-format and selector-only meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x32728 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x32730 | store | 4 | `sp -0x50` | `sp-76` |
| `str` at 0x32738 | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0x3273c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3274c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x32754 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b2bc))` |
| `ldr` at 0x32758 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3275c | load | 4 | `abs 0x32d10` | `-` |
| `ldr` at 0x32764 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32774 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3277c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b2bc))` |
| `str` at 0x32784 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x32788 | load | 4 | `abs 0x32d14` | `-` |
| `ldr` at 0x3278c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32790 | load | 4 | `abs 0x148c48` | `-` |
| `ldr` at 0x327a0 | load | 4 | `abs 0x32d18` | `-` |
| `ldr` at 0x327a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x327ac | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x327b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32890 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x328a4 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x328a8 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x328b0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x328c4 | store | 4 | `sp -0x50` | `sp-76` |
| `ldr` at 0x328c8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x328cc | load | 4 | `abs 0x32d38` | `-` |
| `ldr` at 0x328d0 | load | 4 | `abs 0x32d3c` | `-` |
| `ldr` at 0x328e0 | load | 4 | `abs 0x32d38` | `-` |
| `ldr` at 0x328e4 | load | 4 | `abs 0x32d3c` | `-` |
| `ldr` at 0x328f4 | load | 4 | `abs 0x32d38` | `-` |
| `ldr` at 0x328f8 | load | 4 | `abs 0x32d3c` | `-` |
| `str` at 0x32908 | store | 4 | `sp -0x50` | `sp-76` |
| `str` at 0x32910 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x32914 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32928 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x3292c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32934 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32938 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32948 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32954 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x32958 | load | 4 | `abs 0x32d40` | `-` |
| `ldr` at 0x32960 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x32970 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x32978 | store | 4 | `sp -0x50` | `([sp-80] + ret(0x1b2bc))` |
| `ldr` at 0x3297c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32990 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32994 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x3299c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x329a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x329a8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x329ac | load | 4 | `abs 0x32d44` | `-` |
| `ldr` at 0x329b8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x329c8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x329d0 | store | 4 | `sp -0x50` | `([sp-80] + ret(0x1b2bc))` |
| `ldr` at 0x329d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x329dc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x329e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x329f8 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x329fc | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32a04 | load | 1 | `(unknown)` | `-` |
| `str` at 0x32a08 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x32a10 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x32a2c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32a2c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32a48 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32a48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32a6c | load | 4 | `abs 0x32d48` | `-` |
| `ldr` at 0x32a70 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x32a7c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x32a84 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x32a8c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x32a90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32aa4 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32aa8 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32ab0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32ab4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32ac4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32ad8 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32adc | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x32ae4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x32aec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32b00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x32b04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x32b0c | store | 4 | `sp -0x28` | `([sp-40] + [sp-44])` |
| `ldr` at 0x32b10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x32b14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x32b20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x32b24 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0x32b28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x32b30 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x32b38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32b4c | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32b50 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32b58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32b68 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x32b6c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x32b74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32b88 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32b8c | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32b94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32b98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x32ba8 | store | 4 | `sp -0x38` | `ret(0xfb0a8)` |
| `ldr` at 0x32bb0 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x32bcc | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32bcc | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32be8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32be8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32c0c | load | 4 | `abs 0x32d48` | `-` |
| `ldr` at 0x32c10 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x32c1c | store | 4 | `sp -0x60` | `sp-92` |
| `str` at 0x32c24 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x32c28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32c3c | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32c40 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32c48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32c4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32c5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32c68 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32c6c | load | 4 | `abs 0x32d40` | `-` |
| `ldr` at 0x32c74 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32c84 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x32c8c | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b2bc))` |
| `ldr` at 0x32c90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32ca4 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32ca8 | load | 4 | `abs 0x148c44` | `-` |
| `ldr` at 0x32cb4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x32cc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32cc4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32cc8 | load | 4 | `abs 0x32d44` | `-` |
| `ldr` at 0x32cd4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32ce4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x32cec | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b2bc))` |
| `ldr` at 0x32cf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x32cf8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x32d58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32d6c | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32d70 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32d78 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x32d8c | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x32d90 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x32db4 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x32db8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32dcc | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32dd0 | load | 4 | `abs 0x148c44` | `-` |
| `ldrb` at 0x32dd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32df4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32e08 | load | 4 | `abs 0x32d4c` | `-` |
| `ldr` at 0x32e0c | load | 4 | `abs 0x148c44` | `-` |
| `str` at 0x32e18 | store | 4 | `sp -0x98` | `sp-76` |
| `str` at 0x32e20 | store | 4 | `sp -0x94` | `sp-120` |
| `str` at 0x32e28 | store | 4 | `sp -0x90` | `sp-108` |
| `str` at 0x32e30 | store | 4 | `sp -0x8c` | `sp-92` |
| `stm` at 0x32e3c | store | 4 | `sp -0x88` | `ret(0xfb644)` |
| `stm` at 0x32e3c | store | 4 | `sp -0x84` | `?` |
| `stm` at 0x32e48 | store | 4 | `sp -0x80` | `ret(0xfb644)` |
| `stm` at 0x32e48 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x32e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32e50 | load | 4 | `abs 0x32d50` | `-` |
| `ldr` at 0x32e54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32e58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x32e60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32e70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x32e78 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b2bc))` |
| `ldr` at 0x32e7c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x32e84 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x32e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32e90 | load | 4 | `abs 0x32d54` | `-` |
| `ldmdb` at 0x32e98 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x32e98 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x32e98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x32e98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x32e98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x32740 | `(unknown callee) 0x1b2bc` | `r0` | `r1` | `r2` | `sp-92` |
| `bl` at 0x32760 | `(unknown callee) 0x1b52c` | `([sp-28] + ret(0x1b2bc))` | `0x105d98` | `ret(0x1b2bc)` | `([sp-28] + ret(0x1b2bc))` |
| `bl` at 0x32768 | `(unknown callee) 0x1b2bc` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x328d4 | `(unknown callee) 0x1b52c` | `sp-76` | `0x105ec8` | `0x105ecc` | `sp-76` |
| `bl` at 0x328e8 | `(unknown callee) 0x1b52c` | `sp-120` | `0x105ec8` | `0x105ecc` | `sp-120` |
| `bl` at 0x328fc | `(unknown callee) 0x1b52c` | `sp-108` | `0x105ec8` | `0x105ecc` | `sp-108` |
| `bl` at 0x3295c | `(unknown callee) 0x1b52c` | `?` | `0x105ed0` | `?` | `?` |
| `bl` at 0x32964 | `(unknown callee) 0x1b2bc` | `[sp-80]` | `?` | `?` | `?` |
| `bl` at 0x329b4 | `(unknown callee) 0x1b52c` | `?` | `0x105ed4` | `?` | `?` |
| `bl` at 0x329bc | `(unknown callee) 0x1b2bc` | `[sp-80]` | `?` | `?` | `?` |
| `bl` at 0x32a14 | `(unknown callee) 0xfb618` | `?` | `?` | `?` | `?` |
| `bl` at 0x32a30 | `(unknown callee) 0xfb690` | `ret(0xfb618)` | `?` | `?` | `?` |
| `bl` at 0x32a4c | `(unknown callee) 0xfb30c` | `ret(0xfb690)` | `?` | `?` | `?` |
| `bl` at 0x32a60 | `(unknown callee) 0xfbbf8` | `ret(0xfb30c)` | `?` | `?` | `ret(0xfb30c)` |
| `bl` at 0x32a74 | `(unknown callee) 0x1b52c` | `sp-120` | `0x105ed8` | `[sp-52]` | `ret(0xfbbf8)` |
| `bl` at 0x32afc | `(unknown callee) 0x537cc` | `0x0` | `?` | `sp-44` | `?` |
| `bl` at 0x32ba0 | `(unknown callee) 0xfb0a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x32bb4 | `(unknown callee) 0xfb618` | `?` | `?` | `?` | `?` |
| `bl` at 0x32bd0 | `(unknown callee) 0xfb690` | `ret(0xfb618)` | `?` | `?` | `?` |
| `bl` at 0x32bec | `(unknown callee) 0xfb30c` | `ret(0xfb690)` | `?` | `?` | `?` |
| `bl` at 0x32c00 | `(unknown callee) 0xfbbf8` | `ret(0xfb30c)` | `?` | `?` | `ret(0xfb30c)` |
| `bl` at 0x32c14 | `(unknown callee) 0x1b52c` | `sp-108` | `0x105ed8` | `[sp-56]` | `ret(0xfbbf8)` |
| `bl` at 0x32c70 | `(unknown callee) 0x1b52c` | `?` | `0x105ed0` | `?` | `?` |
| `bl` at 0x32c78 | `(unknown callee) 0x1b2bc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x32cd0 | `(unknown callee) 0x1b52c` | `?` | `0x105ed4` | `?` | `?` |
| `bl` at 0x32cd8 | `(unknown callee) 0x1b2bc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x32d88 | `(unknown callee) 0x47068` | `0x0` | `?` | `sp-57` | `?` |
| `bl` at 0x32d9c | `(unknown callee) 0x33130` | `[sp-32]:1` | `[sp-57]:1` | `[sp-57]:1` | `[sp-32]:1` |
| `bl` at 0x32da8 | `(unknown callee) 0xfb644` | `ret(0x33130)` | `?` | `?` | `ret(0x33130)` |
| `bl` at 0x32de4 | `(unknown callee) 0x33130` | `[sp-32]:1` | `?` | `?` | `?` |
| `bl` at 0x32df0 | `(unknown callee) 0xfb644` | `ret(0x33130)` | `?` | `?` | `ret(0x33130)` |
| `bl` at 0x32e5c | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x105ee4` | `?` | `[sp-64]` |
| `bl` at 0x32e64 | `(unknown callee) 0x1b2bc` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x32e94 | `(unknown callee) 0x1b598` | `([sp-28] + ret(0x1b2bc))` | `0x105c98` | `?` | `[0x148c48]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x32798 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ldrls` at 0x327b4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x328bc | `? == 0x0` | `? != 0x0` |
| `blt` at 0x32940 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x32950 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x32abc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x32b1c | `[sp-44] <= [sp-48] (signed, 32-bit)` | `[sp-44] > [sp-48] (signed, 32-bit)` |
| `beq` at 0x32b64 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x32c54 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x32c64 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x32e98 | `r0` | `ret(0x1b598)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x327b4).

## `AutofanEnvInit`

- `AutofanEnvInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32e9c to 0x32f04.
  Established. The unnamed call to `0x22480` carries `0x148c70`, `0x105f14`, `0x32184`, `[sp-20]`
  (`bl` at 0x32ef0). Earlier four-byte stack stores retain `0x800`, `0xa`, `0xa` and `0x9` at
  `sp -0x2c`, `-0x28`, `-0x24` and `-0x20` (`str` at 0x32ec4, `str` at 0x32ecc, `str` at 0x32ed4, `str`
  at 0x32edc). The callee's role and stack-slot argument interpretation are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x32eac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x32eb0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x32eb8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x32ebc | store | 4 | `sp -0x30` | `[sp-24]` |
| `str` at 0x32ec4 | store | 4 | `sp -0x2c` | `0x800` |
| `str` at 0x32ecc | store | 4 | `sp -0x28` | `0xa` |
| `str` at 0x32ed4 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x32edc | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x32ee0 | load | 4 | `abs 0x32ef8` | `-` |
| `ldr` at 0x32ee4 | load | 4 | `abs 0x32efc` | `-` |
| `ldr` at 0x32ee8 | load | 4 | `abs 0x32f00` | `-` |
| `ldr` at 0x32eec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x32ef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x32ef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x32ef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x32eb4 | `(unknown callee) 0x82d94` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x32ef0 | `(unknown callee) 0x22480` | `0x148c70` | `0x105f14` | `0x32184` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x32ef4 | `r0` | `ret(0x22480)` |

## `EnvAutoFanControl`

- `EnvAutoFanControl`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82d94 to 0x82db4.
  Established. The four-byte store writes `0x1` at absolute `0x149b80` (`str` at 0x82da8). A literal
  read at `0x82db0` supplies the address (`ldr` at 0x82da0). The exit carries entry `r0` (`ldmdb` at
  0x82dac). The stored value's automatic fan-control meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x82da0 | load | 4 | `abs 0x82db0` | `-` |
| `str` at 0x82da8 | store | 4 | `abs 0x149b80` | `0x1` |
| `ldmdb` at 0x82dac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82dac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82dac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82dac | `r0` | `r0` |
