# ARM fan, link and routing monitors in 1.38.4

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
`autofanEnv`, `autofanEnvQuery` and `brickGetSerial` have unresolved computed transfers.
Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges
remain visible as fact-sheet warnings. The absence of recognized returns in `autofanEnv`
and `LinkErrMonitor` does not establish that either routine never returns.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/fan-link-routing-monitor-facts.txt) retain all tool sections.

## `autofanMonitor`

- `autofanMonitor`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30a68 to 0x31e18.
  Established. The signed comparison takes the bypass for `[sp-48] >= [sp-80]` and falls through for
  less (`bge` at 0x31bfc). Fall-through includes word stores of `[sp-52]` at `[sp-32] +0x0`,
  `ret(0xfba2c)` at `[sp-36] +0x0`, and another `ret(0xfba2c)` at `[sp-40] +0x0` (`str` at 0x31c08,
  `str` at 0x31c1c, `str` at 0x31c48). A later call to `0x467bc` has `0x0`, unknown, `[sp-84]:1`,
  unknown (`bl` at 0x31d6c). Fan-speed calculations, sensor units and the joined r0 return remain
  unknown (`ldmdb` at 0x31e14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30a78 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x30a7c | store | 4 | `sp -0x24` | `r1` |
| `str` at 0x30a80 | store | 4 | `sp -0x28` | `r2` |
| `str` at 0x30a88 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x30a90 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x30a98 | store | 4 | `sp -0x130` | `sp-300` |
| `ldr` at 0x30a9c | load | 4 | `abs 0x31684` | `-` |
| `ldrb` at 0x30aa0 | load | 1 | `abs 0x148038` | `-` |
| `str` at 0x30ab0 | store | 4 | `sp -0x134` | `0x0` |
| `ldr` at 0x30ab8 | load | 4 | `abs 0x31688` | `-` |
| `ldr` at 0x30abc | load | 4 | `abs 0x1426a8` | `-` |
| `ldr` at 0x30ac8 | load | 4 | `abs 0x31688` | `-` |
| `ldr` at 0x30acc | load | 4 | `abs 0x31688` | `-` |
| `ldr` at 0x30ad0 | load | 4 | `abs 0x1426a8` | `-` |
| `str` at 0x30ad8 | store | 4 | `abs 0x1426a8` | `[0x1426a8]+1` |
| `ldr` at 0x30ae0 | load | 4 | `abs 0x31688` | `-` |
| `str` at 0x30ae8 | store | 4 | `abs 0x1426a8` | `0x0` |
| `str` at 0x30af0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x30af4 | load | 4 | `abs 0x3168c` | `-` |
| `ldr` at 0x30af8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30afc | load | 4 | `abs 0x147f28` | `-` |
| `str` at 0x30b10 | store | 4 | `sp -0x50` | `0x2` |
| `ldr` at 0x30b14 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30b30 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30b34 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30b3c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30b48 | load | 4 | `abs 0x3169c` | `-` |
| `str` at 0x30b4c | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x30b54 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x30b5c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x30b60 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30b7c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30b80 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30b88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30b8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x30b9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30bb8 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30bbc | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x30bc4 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x30bcc | load | 1 | `(unknown)` | `-` |
| `strb` at 0x30bd0 | store | 1 | `sp -0x55` | `?` |
| `ldrb` at 0x30bd4 | load | 1 | `sp -0x55` | `-` |
| `ldrb` at 0x30bf0 | load | 1 | `sp -0x55` | `-` |
| `ldr` at 0x30c04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x30c10 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x30c20 | store | 4 | `sp -0x60` | `ret(0xfb44c)` |
| `ldr` at 0x30c24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x30c28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x30c34 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x30c38 | store | 4 | `sp -0x40` | `[sp-60]` |
| `ldr` at 0x30c3c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x30c44 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0x30c4c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30c68 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30c6c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30c74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30c84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x30c90 | store | 4 | `sp -0x5c` | `ret(0xfb614)` |
| `ldr` at 0x30c98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30cb4 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30cb8 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30cc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30cd0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x30ce0 | store | 4 | `sp -0x5c` | `ret(0xfb7f0)` |
| `ldr` at 0x30ce4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30cec | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x30cf0 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x30cf8 | load | 4 | `abs 0x31688` | `-` |
| `ldr` at 0x30cfc | load | 4 | `abs 0x1426a8` | `-` |
| `ldr` at 0x30d10 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x30d14 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x30d18 | load | 4 | `abs 0x3169c` | `-` |
| `str` at 0x30d1c | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x30d24 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x30d28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30d38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30d40 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x30d44 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x30d4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30d60 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x30d64 | load | 4 | `(unknown)` | `-` |
| `str` at 0x30d70 | store | 4 | `sp -0x60` | `ret(0xfb44c)` |
| `ldr` at 0x30d74 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x30d7c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x30d84 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x30d88 | load | 4 | `abs 0x31690` | `-` |
| `str` at 0x30d94 | store | 4 | `sp -0x5c` | `ret(0xfb7f0)` |
| `ldr` at 0x30d98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30db4 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30db8 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x30dc0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x30dd4 | store | 4 | `sp -0x13c` | `?` |
| `ldr` at 0x30dd8 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x30ddc | load | 4 | `abs 0x31694` | `-` |
| `ldr` at 0x30de0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x30de8 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x30df8 | load | 4 | `sp -0x130` | `-` |
| `str` at 0x30e00 | store | 4 | `sp -0x130` | `([sp-304] + ret(0x1b22c))` |
| `ldr` at 0x30e04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30e20 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30e24 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30e2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30e40 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30e5c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30e60 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30e68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30e78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30e94 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30e98 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x30ea0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x30ea4 | load | 4 | `abs 0x31698` | `-` |
| `ldr` at 0x30ea8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x30eb0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x30ec0 | load | 4 | `sp -0x130` | `-` |
| `str` at 0x30ec8 | store | 4 | `sp -0x130` | `([sp-304] + ret(0x1b22c))` |
| `ldr` at 0x30ecc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30ee8 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30eec | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30ef4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30f10 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30f14 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30f1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30f30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30f4c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30f50 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x30f58 | load | 1 | `sp -0x58` | `-` |
| `ldr` at 0x30f6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x30f78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30f94 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x30f98 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x30fa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x30fbc | store | 4 | `sp -0x54` | `ret(0xfba2c)` |
| `ldrb` at 0x30fc0 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x30fc8 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x30fdc | load | 1 | `sp -0x34` | `-` |
| `str` at 0x30fec | store | 4 | `sp -0x54` | `ret(0x32ad4)` |
| `ldrb` at 0x30ff0 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x30ff8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x3100c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31028 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x3102c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31034 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31038 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x31040 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3105c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31060 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31068 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31078 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31094 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31098 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x310a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x310b0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x310cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x310e8 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x310ec | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x310f4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x310f8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x31100 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3111c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31120 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x31128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31134 | load | 4 | `sp -0x5c` | `-` |
| `ldrb` at 0x31150 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31160 | store | 4 | `sp -0x54` | `ret(0x32ad4)` |
| `ldr` at 0x31168 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31184 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31188 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x31190 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31194 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x311a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x311bc | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x311c0 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x311cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x311e4 | store | 4 | `sp -0x54` | `ret(0xfba2c)` |
| `ldrb` at 0x311e8 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x311f0 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x31204 | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31214 | store | 4 | `sp -0x54` | `ret(0x32ad4)` |
| `ldr` at 0x3121c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31238 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x3123c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31244 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31258 | store | 4 | `sp -0x138` | `?` |
| `str` at 0x31264 | store | 4 | `sp -0x138` | `0x1` |
| `ldr` at 0x31268 | load | 4 | `sp -0x138` | `-` |
| `str` at 0x3126c | store | 4 | `sp -0x38` | `[sp-312]` |
| `ldr` at 0x31270 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31278 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x3127c | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31284 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x31288 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31294 | store | 4 | `sp -0x64` | `ret(0xfb448)` |
| `ldr` at 0x31298 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x312a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x312c4 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x312c8 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x312d0 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x312f0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x312f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31324 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3132c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31330 | load | 4 | `abs 0x147f2c` | `-` |
| `ldrb` at 0x31338 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31344 | load | 1 | `sp -0x57` | `-` |
| `str` at 0x31348 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x31350 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x31354 | load | 4 | `abs 0x3169c` | `-` |
| `ldr` at 0x3136c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31374 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31378 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31380 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31384 | load | 4 | `abs 0x3169c` | `-` |
| `ldr` at 0x3139c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x313a4 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x313a8 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x313b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x313b8 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x313bc | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x313c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x313cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x313d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x313ec | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x313f0 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x313f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31404 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3140c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31410 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31418 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31434 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31438 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31440 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31444 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x31464 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x31470 | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x31474 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x31478 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31494 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31498 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x314a0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x314a4 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x314ac | load | 4 | `sp -0x38` | `-` |
| `str` at 0x314b8 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x314c0 | load | 1 | `sp -0x57` | `-` |
| `str` at 0x314c4 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x314cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x314d4 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x314d8 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x314e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x314e8 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x314ec | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x314f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x314fc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31500 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3151c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31520 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31528 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31534 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3153c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31540 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31548 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31564 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31568 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31570 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31574 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x31594 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x315a0 | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x315a4 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x315a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x315c4 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x315c8 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x315d0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x315d4 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x315dc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x315e8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x315f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3160c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31610 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31618 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31628 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x31644 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3164c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31650 | load | 4 | `abs 0x147f2c` | `-` |
| `strb` at 0x3165c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x31660 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x31664 | load | 4 | `abs 0x3169c` | `-` |
| `ldr` at 0x316c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x316c8 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x316cc | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x316d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x316d8 | load | 4 | `abs 0x3169c` | `-` |
| `ldr` at 0x316f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x316f8 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x316fc | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31704 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3170c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31710 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31718 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31720 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31724 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31740 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31744 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x3174c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31758 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31760 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31764 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x3176c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31788 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x3178c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31794 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31798 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x317b8 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x317c4 | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x317c8 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x317cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x317e8 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x317ec | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x317f4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x317f8 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x31800 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3180c | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31814 | load | 1 | `sp -0x57` | `-` |
| `str` at 0x31818 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x31820 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31828 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x3182c | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31834 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3183c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31840 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31848 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31850 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31854 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31870 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31874 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x3187c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31888 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31890 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31894 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x3189c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x318b8 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x318bc | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x318c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x318c8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x318e8 | store | 1 | `sp -0x56` | `?` |
| `strb` at 0x318f4 | store | 1 | `sp -0x56` | `0x0` |
| `ldrb` at 0x318f8 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x318fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31918 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x3191c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31924 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31928 | load | 1 | `sp -0x56` | `-` |
| `ldr` at 0x31930 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3193c | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31940 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31948 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x3195c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31978 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x3197c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31984 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31988 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31990 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31998 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x319ac | load | 1 | `sp -0x34` | `-` |
| `str` at 0x319bc | store | 4 | `sp -0x54` | `ret(0x32ad4)` |
| `str` at 0x319dc | store | 4 | `sp -0x44` | `0x5` |
| `str` at 0x319e4 | store | 4 | `sp -0x48` | `0xa` |
| `str` at 0x319f0 | store | 4 | `sp -0x44` | `0x9` |
| `str` at 0x319f8 | store | 4 | `sp -0x48` | `0xd` |
| `ldr` at 0x319fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31a18 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31a1c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31a24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31a28 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x31a3c | load | 4 | `sp -0x5c` | `-` |
| `ldrb` at 0x31a58 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31a60 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x31a78 | store | 4 | `sp -0x50` | `0x10` |
| `ldr` at 0x31a80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31a9c | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31aa0 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31aa8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31aac | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x31ac0 | load | 4 | `sp -0x5c` | `-` |
| `ldrb` at 0x31adc | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31ae4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x31afc | store | 4 | `sp -0x50` | `0x8` |
| `ldrb` at 0x31b04 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0x31b08 | load | 1 | `sp -0x54` | `-` |
| `str` at 0x31b28 | store | 4 | `sp -0x50` | `0x4` |
| `ldrb` at 0x31b30 | load | 1 | `sp -0x57` | `-` |
| `str` at 0x31b34 | store | 4 | `sp -0x54` | `[sp-87]:1` |
| `ldr` at 0x31b38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31b40 | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31b44 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31b4c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x31b50 | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x31b54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31b5c | load | 4 | `abs 0x316a0` | `-` |
| `ldr` at 0x31b60 | load | 4 | `abs 0x147f2c` | `-` |
| `ldr` at 0x31b68 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x31b6c | store | 4 | `(unknown)` | `[sp-100]` |
| `ldrb` at 0x31b70 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31b78 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31b8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31ba8 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31bac | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31bb4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31bb8 | store | 4 | `sp -0x54` | `?` |
| `ldrb` at 0x31bc0 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0x31bc8 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x31bdc | load | 1 | `sp -0x34` | `-` |
| `str` at 0x31bec | store | 4 | `sp -0x54` | `ret(0x32ad4)` |
| `ldr` at 0x31bf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x31bf4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x31c00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x31c04 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x31c08 | store | 4 | `[sp-32] +0x0` | `[sp-52]` |
| `ldr` at 0x31c0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x31c10 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x31c1c | store | 4 | `[sp-36] +0x0` | `ret(0xfba2c)` |
| `ldr` at 0x31c20 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x31c24 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0x31c28 | load | 1 | `sp -0x54` | `-` |
| `str` at 0x31c48 | store | 4 | `[sp-40] +0x0` | `ret(0xfba2c)` |
| `ldr` at 0x31c4c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x31c50 | store | 4 | `sp -0x30` | `[sp-80]` |
| `ldr` at 0x31c54 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x31c58 | load | 4 | `abs 0x316a4` | `-` |
| `ldr` at 0x31c5c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31c64 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x31c74 | load | 4 | `sp -0x130` | `-` |
| `str` at 0x31c7c | store | 4 | `sp -0x130` | `([sp-304] + ret(0x1b22c))` |
| `ldrb` at 0x31c80 | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x31c84 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x31c90 | load | 4 | `abs 0x316b8` | `-` |
| `ldr` at 0x31c94 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x31ca4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31cc0 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31cc4 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31ccc | load | 1 | `sp -0x57` | `-` |
| `ldr` at 0x31cd0 | load | 4 | `abs 0x316a8` | `-` |
| `ldr` at 0x31cd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31cd8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x31ce4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x31ce8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31d04 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31d08 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x31d10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31d14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31d24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31d40 | load | 4 | `abs 0x316ac` | `-` |
| `ldr` at 0x31d44 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x31d50 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x31d5c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31d60 | load | 1 | `sp -0x54` | `-` |
| `ldr` at 0x31d70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31d78 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x31d80 | load | 4 | `abs 0x316b0` | `-` |
| `ldr` at 0x31d84 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x31d8c | load | 1 | `sp -0x50` | `-` |
| `strb` at 0x31d90 | store | 1 | `[sp-52] +0x147f30` | `([sp-80]:1 & 0xff)` |
| `ldr` at 0x31d94 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x31d9c | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x31da4 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x31da8 | load | 4 | `abs 0x316b4` | `-` |
| `ldr` at 0x31db0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0x31dc0 | load | 4 | `sp -0x130` | `-` |
| `str` at 0x31dc8 | store | 4 | `sp -0x130` | `([sp-304] + ret(0x1b22c))` |
| `ldr` at 0x31dcc | load | 4 | `abs 0x316b8` | `-` |
| `ldr` at 0x31dd0 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x31de4 | load | 4 | `abs 0x316bc` | `-` |
| `ldr` at 0x31df0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31e00 | store | 4 | `sp -0x134` | `0x2` |
| `ldr` at 0x31e08 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31e0c | store | 4 | `sp -0x134` | `[sp-48]` |
| `ldr` at 0x31e10 | load | 4 | `sp -0x134` | `-` |
| `ldmdb` at 0x31e14 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x31e14 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x31e14 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x31e14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x31e14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x31e14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30be0 | `(unknown callee) 0x52e30` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x30c00 | `(unknown callee) 0x52fb8` | `0x0` | `[sp-85]:1` | `sp-60` | `[sp-85]:1` |
| `bl` at 0x30c08 | `(unknown callee) 0xfb614` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x30c18 | `(unknown callee) 0xfb44c` | `[sp-96]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x30c88 | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x30cc8 | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x30cd8 | `(unknown callee) 0xfb7f0` | `[sp-96]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x30d68 | `(unknown callee) 0xfb44c` | `?` | `?` | `0x8` | `?` |
| `bl` at 0x30d8c | `(unknown callee) 0xfb7f0` | `?` | `0x40400000` | `?` | `?` |
| `bl` at 0x30dc4 | `(unknown callee) 0xfaca8` | `ret(0xfb7f0)` | `?` | `?` | `[0x147f24]` |
| `bl` at 0x30de4 | `(unknown callee) 0x1b49c` | `[sp-304]` | `0x1051b0` | `?` | `ret(0xfaca8)` |
| `bl` at 0x30dec | `(unknown callee) 0x1b22c` | `[sp-304]` | `?` | `?` | `?` |
| `bl` at 0x30e3c | `(unknown callee) 0x46c64` | `0x0` | `?` | `sp-87` | `?` |
| `bl` at 0x30eac | `(unknown callee) 0x1b49c` | `[sp-304]` | `0x1051bc` | `?` | `?` |
| `bl` at 0x30eb4 | `(unknown callee) 0x1b22c` | `[sp-304]` | `?` | `?` | `?` |
| `bl` at 0x30f2c | `(unknown callee) 0x46c64` | `0x0` | `?` | `sp-88` | `?` |
| `bl` at 0x30f60 | `(unknown callee) 0xfb614` | `[sp-88]:1` | `?` | `?` | `[sp-88]:1` |
| `bl` at 0x30f70 | `(unknown callee) 0xfb660` | `ret(0xfb614)` | `?` | `?` | `ret(0xfb614)` |
| `bl` at 0x30fa8 | `(unknown callee) 0xfb44c` | `ret(0xfb660)` | `?` | `?` | `?` |
| `bl` at 0x30fb4 | `(unknown callee) 0xfba2c` | `ret(0xfb44c)` | `?` | `?` | `ret(0xfb44c)` |
| `bl` at 0x30fcc | `(unknown callee) 0x32b6c` | `[sp-52]:1` | `ret(0xfba2c)` | `?` | `[sp-52]:1` |
| `bl` at 0x30fe4 | `(unknown callee) 0x32ad4` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x30ffc | `(unknown callee) 0x32c3c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x310a8 | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x310b8 | `(unknown callee) 0xfb998` | `[sp-92]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x3112c | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x3113c | `(unknown callee) 0xfb990` | `[sp-92]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x31158 | `(unknown callee) 0x32ad4` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x31198 | `(unknown callee) 0xfb660` | `?` | `[sp-92]` | `?` | `?` |
| `bl` at 0x311d0 | `(unknown callee) 0xfb44c` | `ret(0xfb660)` | `?` | `?` | `?` |
| `bl` at 0x311dc | `(unknown callee) 0xfba2c` | `ret(0xfb44c)` | `?` | `?` | `ret(0xfb44c)` |
| `bl` at 0x311f4 | `(unknown callee) 0x32b6c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x3120c | `(unknown callee) 0x32ad4` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x3128c | `(unknown callee) 0xfb448` | `[sp-92]` | `?` | `?` | `?` |
| `bl` at 0x3129c | `(unknown callee) 0xfaca8` | `[sp-92]` | `?` | `?` | `ret(0xfb448)` |
| `bl` at 0x312d8 | `(unknown callee) 0xfac7c` | `?` | `?` | `?` | `?` |
| `bl` at 0x312f4 | `(unknown callee) 0xfa96c` | `ret(0xfac7c)` | `?` | `?` | `?` |
| `bl` at 0x31310 | `(unknown callee) 0xfb1c0` | `ret(0xfaca8)` | `?` | `ret(0xfa96c)` | `?` |
| `bl` at 0x31358 | `(unknown callee) 0xfb990` | `[sp-100]` | `0x0` | `?` | `?` |
| `bl` at 0x31388 | `(unknown callee) 0xfb990` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x31450 | `(unknown callee) 0xfa87c` | `?` | `?` | `?` | `?` |
| `bl` at 0x31580 | `(unknown callee) 0xfa87c` | `?` | `?` | `?` | `?` |
| `bl` at 0x31620 | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x31630 | `(unknown callee) 0xfb990` | `[sp-92]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x31668 | `(unknown callee) 0xfb998` | `[sp-100]` | `0x0` | `?` | `0x1` |
| `bl` at 0x316dc | `(unknown callee) 0xfb998` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x317a4 | `(unknown callee) 0xfa87c` | `?` | `?` | `?` | `?` |
| `bl` at 0x318d4 | `(unknown callee) 0xfa87c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3194c | `(unknown callee) 0x32c3c` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x3199c | `(unknown callee) 0x32b6c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x319b4 | `(unknown callee) 0x32ad4` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x319c0 | `(unknown callee) 0x4f090` | `?` | `?` | `?` | `?` |
| `bl` at 0x31a34 | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x31a44 | `(unknown callee) 0xfb990` | `[sp-92]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x31a64 | `(unknown callee) 0x32b6c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31ab8 | `(unknown callee) 0xfb614` | `?` | `?` | `?` | `?` |
| `bl` at 0x31ac8 | `(unknown callee) 0xfb990` | `[sp-92]` | `ret(0xfb614)` | `?` | `ret(0xfb614)` |
| `bl` at 0x31ae8 | `(unknown callee) 0x32b6c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31b14 | `(unknown callee) 0x32e70` | `[sp-52]:1` | `[sp-84]:1` | `[sp-84]:1` | `[sp-52]:1` |
| `bl` at 0x31b7c | `(unknown callee) 0x32c3c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31bcc | `(unknown callee) 0x32b6c` | `[sp-52]:1` | `[sp-84]` | `?` | `[sp-52]:1` |
| `bl` at 0x31be4 | `(unknown callee) 0x32ad4` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0x31c14 | `(unknown callee) 0xfba2c` | `[sp-92]` | `?` | `[sp-32]` | `[sp-52]` |
| `bl` at 0x31c34 | `(unknown callee) 0x32d38` | `[sp-52]:1` | `[sp-84]:1` | `[sp-84]:1` | `[sp-52]:1` |
| `bl` at 0x31c40 | `(unknown callee) 0xfba2c` | `ret(0x32d38)` | `?` | `?` | `ret(0x32d38)` |
| `bl` at 0x31c60 | `(unknown callee) 0x1b49c` | `[sp-304]` | `0x1051c4` | `[sp-84]` | `?` |
| `bl` at 0x31c68 | `(unknown callee) 0x1b22c` | `[sp-304]` | `?` | `?` | `?` |
| `bl` at 0x31cdc | `(unknown callee) 0x2070c` | `0x1051cc` | `?` | `[sp-87]:1` | `[sp-84]` |
| `bl` at 0x31d6c | `(unknown callee) 0x467bc` | `0x0` | `?` | `[sp-84]:1` | `?` |
| `bl` at 0x31dac | `(unknown callee) 0x1b49c` | `[sp-304]` | `0x1051dc` | `?` | `[0x147f28]` |
| `bl` at 0x31db4 | `(unknown callee) 0x1b22c` | `[sp-304]` | `?` | `?` | `?` |
| `bl` at 0x31dec | `(unknown callee) 0x23f2c` | `0x1051e0` | `sp-300` | `ret(0x1b22c)` | `sp-300` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30aa8 | `[0x148038]:1 == 0x0` | `[0x148038]:1 != 0x0` |
| `bgt` at 0x30ac4 | `[0x1426a8] > 0x1 (signed, 32-bit)` | `[0x1426a8] <= 0x1 (signed, 32-bit)` |
| `blt` at 0x30b04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x30b44 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x30b94 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x30bec | `ret(0x52e30) == 0x0` | `ret(0x52e30) != 0x0` |
| `ble` at 0x30c30 | `[sp-60] <= [sp-64] (signed, 32-bit)` | `[sp-60] > [sp-64] (signed, 32-bit)` |
| `beq` at 0x30c80 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x30d30 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x30e74 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x30fd8 | `ret(0x32b6c) == 0x0` | `ret(0x32b6c) != 0x0` |
| `beq` at 0x31008 | `ret(0x32c3c) == 0x0` | `ret(0x32c3c) != 0x0` |
| `beq` at 0x31074 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x310c4 | `ret(0xfb998) <= 0x0 (signed, 32-bit)` | `ret(0xfb998) > 0x0 (signed, 32-bit)` |
| `bgt` at 0x31148 | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31200 | `ret(0x32b6c) == 0x0` | `ret(0x32b6c) != 0x0` |
| `beq` at 0x31250 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x3131c | `ret(0xfb1c0) < 0x0 (signed, 32-bit)` | `ret(0xfb1c0) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x31340 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x31364 | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `bgt` at 0x31394 | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31400 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31530 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x3163c | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `blt` at 0x31674 | `ret(0xfb998) < 0x0 (signed, 32-bit)` | `ret(0xfb998) >= 0x0 (signed, 32-bit)` |
| `blt` at 0x316e8 | `ret(0xfb998) < 0x0 (signed, 32-bit)` | `ret(0xfb998) >= 0x0 (signed, 32-bit)` |
| `beq` at 0x31754 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31884 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31958 | `ret(0x32c3c) == 0x0` | `ret(0x32c3c) != 0x0` |
| `beq` at 0x319a8 | `ret(0x32b6c) == 0x0` | `ret(0x32b6c) != 0x0` |
| `beq` at 0x319d0 | `? == 0x4c` | `? != 0x4c` |
| `bgt` at 0x31a50 | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31a70 | `ret(0x32b6c) == 0x0` | `ret(0x32b6c) != 0x0` |
| `bgt` at 0x31ad4 | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x31af4 | `ret(0x32b6c) == 0x0` | `ret(0x32b6c) != 0x0` |
| `beq` at 0x31b20 | `ret(0x32e70) == 0x0` | `ret(0x32e70) != 0x0` |
| `beq` at 0x31b88 | `ret(0x32c3c) == 0x0` | `ret(0x32c3c) != 0x0` |
| `beq` at 0x31bd8 | `ret(0x32b6c) == 0x0` | `ret(0x32b6c) != 0x0` |
| `bge` at 0x31bfc | `[sp-48] >= [sp-80] (signed, 32-bit)` | `[sp-48] < [sp-80] (signed, 32-bit)` |
| `beq` at 0x31c8c | `[sp-84] == [sp-87]:1` | `[sp-84] != [sp-87]:1` |
| `beq` at 0x31ca0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x31d1c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x31ddc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x31df8 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x31e14 | `r0` | `?` |

## `autofanEnv`

- `autofanEnv`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x31e18 to 0x322d0.
  Established. The call to `0x30a68` uses `sp-44`, `sp-48`, `sp-52`, `sp-44` (`bl` at 0x31e74). The
  computed transfer takes an unresolved edge for `ret(0x30a68)-2 <= 0xe` unsigned and falls through
  for greater (`ldrls` at 0x31e8c). A known later word store writes `[sp-40]` at `0x1426ac` (`str`
  at 0x321dc). Other known paths store byte `0x1` and byte `0x0` at unknown targets (`strb` at
  0x32228, `strb` at 0x3226c). No return is recognized in the selected span. Excluded fan actions,
  shutdown behavior and timing units remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x31e28 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x31e2c | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x31e30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x31e34 | store | 4 | `sp -0x8c` | `r1` |
| `str` at 0x31e7c | store | 4 | `sp -0x28` | `ret(0x30a68)` |
| `ldr` at 0x31e80 | load | 4 | `sp -0x28` | `-` |
| `ldrls` at 0x31e8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x321c8 | load | 4 | `abs 0x322c4` | `-` |
| `ldr` at 0x321cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x321d4 | load | 4 | `abs 0x3229c` | `-` |
| `ldr` at 0x321d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x321dc | store | 4 | `abs 0x1426ac` | `[sp-40]` |
| `str` at 0x321fc | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x32200 | load | 4 | `abs 0x322c8` | `-` |
| `ldr` at 0x32204 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x32208 | load | 4 | `abs 0x147f28` | `-` |
| `ldr` at 0x32218 | load | 4 | `abs 0x322cc` | `-` |
| `ldr` at 0x3221c | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x32228 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x3222c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x32234 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `str` at 0x32240 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x32244 | load | 4 | `abs 0x322c8` | `-` |
| `ldr` at 0x32248 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3224c | load | 4 | `abs 0x147f28` | `-` |
| `ldr` at 0x3225c | load | 4 | `abs 0x322cc` | `-` |
| `ldr` at 0x32260 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3226c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x32270 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x32278 | store | 4 | `sp -0x38` | `[sp-56]+1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x31e38 | `(unknown callee) 0x3d8e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31e4c | `(unknown callee) 0x1f1dc` | `0x6` | `?` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x31e74 | `(unknown callee) 0x30a68` | `sp-44` | `sp-48` | `sp-52` | `sp-44` |
| `bl` at 0x321d0 | `(unknown callee) 0x2070c` | `0x1052a4` | `ret(0x30a68)` | `?` | `ret(0x30a68)-2` |
| `bl` at 0x321e8 | `(unknown callee) 0x1f1dc` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x32284 | `(unknown callee) 0x32fbc` | `0x7d0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x31e44 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `beq` at 0x31e58 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `ldrls` at 0x31e8c | `ret(0x30a68)-2 <= 0xe (unsigned, 32-bit)` | `ret(0x30a68)-2 > 0xe (unsigned, 32-bit)` |
| `bne` at 0x321f4 | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `blt` at 0x32210 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x32254 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x31e8c).

## `autofanEnvQuery`

- `autofanEnvQuery`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x322d0 to 0x32a6c.
  Established. The computed transfer retains only an `ls flags condition (operands unknown)` for its
  taken edge and falls through for its negation (`ldrls` at 0x3236c). A known later call to
  `0x46c64` has `0x0`, unknown, `sp-57`, unknown (`bl` at 0x32948). The subsequent call to `0x32d38`
  uses `[sp-32]:1`, `[sp-57]:1`, `[sp-57]:1`, `[sp-32]:1` (`bl` at 0x3295c). The known-edge r0
  return is `ret(0x1b508)` (`ldmdb` at 0x32a68). Excluded selector meanings, fan records and numeric
  units remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x322e0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x322e8 | store | 4 | `sp -0x50` | `sp-76` |
| `str` at 0x322f0 | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0x322f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32304 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3230c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b22c))` |
| `ldr` at 0x32310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32314 | load | 4 | `abs 0x328cc` | `-` |
| `ldr` at 0x3231c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3232c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x32334 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b22c))` |
| `str` at 0x3233c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x32340 | load | 4 | `abs 0x328d0` | `-` |
| `ldr` at 0x32344 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32348 | load | 4 | `abs 0x147f28` | `-` |
| `ldr` at 0x32358 | load | 4 | `abs 0x328d4` | `-` |
| `ldr` at 0x3235c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x32364 | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x3236c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x323fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32418 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x3241c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x32424 | load | 1 | `(unknown)` | `-` |
| `str` at 0x32438 | store | 4 | `sp -0x50` | `sp-76` |
| `ldr` at 0x3243c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x32440 | load | 4 | `abs 0x328f0` | `-` |
| `ldr` at 0x32444 | load | 4 | `abs 0x328f4` | `-` |
| `ldr` at 0x32454 | load | 4 | `abs 0x328f0` | `-` |
| `ldr` at 0x32458 | load | 4 | `abs 0x328f4` | `-` |
| `ldr` at 0x32468 | load | 4 | `abs 0x328f0` | `-` |
| `ldr` at 0x3246c | load | 4 | `abs 0x328f4` | `-` |
| `str` at 0x3247c | store | 4 | `sp -0x50` | `sp-76` |
| `str` at 0x32484 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x32488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x324a4 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x324a8 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x324b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x324b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x324c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x324d0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x324d4 | load | 4 | `abs 0x328f8` | `-` |
| `ldr` at 0x324dc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x324ec | load | 4 | `sp -0x50` | `-` |
| `str` at 0x324f4 | store | 4 | `sp -0x50` | `([sp-80] + ret(0x1b22c))` |
| `ldr` at 0x324f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32514 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32518 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x32520 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x32528 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3252c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x32530 | load | 4 | `abs 0x328fc` | `-` |
| `ldr` at 0x3253c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3254c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x32554 | store | 4 | `sp -0x50` | `([sp-80] + ret(0x1b22c))` |
| `ldr` at 0x32558 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x32560 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x32568 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32584 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32588 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x32590 | load | 1 | `(unknown)` | `-` |
| `str` at 0x32594 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3259c | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x325b8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x325b8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x325d4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x325d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x325f8 | load | 4 | `abs 0x32900` | `-` |
| `ldr` at 0x325fc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x32608 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x32610 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x32618 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3261c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32638 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x3263c | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x32644 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32648 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32658 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32674 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32678 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x32680 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x32688 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3269c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x326a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x326a8 | store | 4 | `sp -0x28` | `([sp-40] + [sp-44])` |
| `ldr` at 0x326ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x326b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x326bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x326c0 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0x326c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x326cc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x326d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x326f0 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x326f4 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x326fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3270c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x32710 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x32718 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32734 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32738 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x32740 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32744 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x32754 | store | 4 | `sp -0x38` | `ret(0xfa70c)` |
| `ldr` at 0x3275c | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x32778 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32778 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32794 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x32794 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x327b8 | load | 4 | `abs 0x32900` | `-` |
| `ldr` at 0x327bc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x327c8 | store | 4 | `sp -0x60` | `sp-92` |
| `str` at 0x327d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x327d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x327f0 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x327f4 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x327fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32800 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32810 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3281c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32820 | load | 4 | `abs 0x328f8` | `-` |
| `ldr` at 0x32828 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32838 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x32840 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b22c))` |
| `ldr` at 0x32844 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32860 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32864 | load | 4 | `abs 0x147f24` | `-` |
| `ldr` at 0x32870 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3287c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32880 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x32884 | load | 4 | `abs 0x328fc` | `-` |
| `ldr` at 0x32890 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x328a0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x328a8 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b22c))` |
| `ldr` at 0x328ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x328b4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x32910 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3292c | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32930 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x32938 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3294c | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x32950 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x32974 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x32978 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32994 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x32998 | load | 4 | `abs 0x147f24` | `-` |
| `ldrb` at 0x329a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x329bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x329d8 | load | 4 | `abs 0x32904` | `-` |
| `ldr` at 0x329dc | load | 4 | `abs 0x147f24` | `-` |
| `str` at 0x329e8 | store | 4 | `sp -0x98` | `sp-76` |
| `str` at 0x329f0 | store | 4 | `sp -0x94` | `sp-120` |
| `str` at 0x329f8 | store | 4 | `sp -0x90` | `sp-108` |
| `str` at 0x32a00 | store | 4 | `sp -0x8c` | `sp-92` |
| `stm` at 0x32a0c | store | 4 | `sp -0x88` | `ret(0xfaca8)` |
| `stm` at 0x32a0c | store | 4 | `sp -0x84` | `?` |
| `stm` at 0x32a18 | store | 4 | `sp -0x80` | `ret(0xfaca8)` |
| `stm` at 0x32a18 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x32a1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32a20 | load | 4 | `abs 0x32908` | `-` |
| `ldr` at 0x32a24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32a28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x32a30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32a40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x32a48 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b22c))` |
| `ldr` at 0x32a4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x32a54 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x32a5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32a60 | load | 4 | `abs 0x3290c` | `-` |
| `ldmdb` at 0x32a68 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x32a68 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x32a68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x32a68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x32a68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x322f8 | `(unknown callee) 0x1b22c` | `r0` | `r1` | `r2` | `sp-92` |
| `bl` at 0x32318 | `(unknown callee) 0x1b49c` | `([sp-28] + ret(0x1b22c))` | `0x1052c8` | `ret(0x1b22c)` | `([sp-28] + ret(0x1b22c))` |
| `bl` at 0x32320 | `(unknown callee) 0x1b22c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x32448 | `(unknown callee) 0x1b49c` | `sp-76` | `0x1053ec` | `0x1053f0` | `sp-76` |
| `bl` at 0x3245c | `(unknown callee) 0x1b49c` | `sp-120` | `0x1053ec` | `0x1053f0` | `sp-120` |
| `bl` at 0x32470 | `(unknown callee) 0x1b49c` | `sp-108` | `0x1053ec` | `0x1053f0` | `sp-108` |
| `bl` at 0x324d8 | `(unknown callee) 0x1b49c` | `?` | `0x1053f4` | `?` | `?` |
| `bl` at 0x324e0 | `(unknown callee) 0x1b22c` | `[sp-80]` | `?` | `?` | `?` |
| `bl` at 0x32538 | `(unknown callee) 0x1b49c` | `?` | `0x1053f8` | `?` | `?` |
| `bl` at 0x32540 | `(unknown callee) 0x1b22c` | `[sp-80]` | `?` | `?` | `?` |
| `bl` at 0x325a0 | `(unknown callee) 0xfac7c` | `?` | `?` | `?` | `?` |
| `bl` at 0x325bc | `(unknown callee) 0xfacf4` | `ret(0xfac7c)` | `?` | `?` | `?` |
| `bl` at 0x325d8 | `(unknown callee) 0xfa970` | `ret(0xfacf4)` | `?` | `?` | `?` |
| `bl` at 0x325ec | `(unknown callee) 0xfb25c` | `ret(0xfa970)` | `?` | `?` | `ret(0xfa970)` |
| `bl` at 0x32600 | `(unknown callee) 0x1b49c` | `sp-120` | `0x1053fc` | `[sp-52]` | `ret(0xfb25c)` |
| `bl` at 0x32698 | `(unknown callee) 0x52fb8` | `0x0` | `?` | `sp-44` | `?` |
| `bl` at 0x3274c | `(unknown callee) 0xfa70c` | `?` | `?` | `?` | `?` |
| `bl` at 0x32760 | `(unknown callee) 0xfac7c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3277c | `(unknown callee) 0xfacf4` | `ret(0xfac7c)` | `?` | `?` | `?` |
| `bl` at 0x32798 | `(unknown callee) 0xfa970` | `ret(0xfacf4)` | `?` | `?` | `?` |
| `bl` at 0x327ac | `(unknown callee) 0xfb25c` | `ret(0xfa970)` | `?` | `?` | `ret(0xfa970)` |
| `bl` at 0x327c0 | `(unknown callee) 0x1b49c` | `sp-108` | `0x1053fc` | `[sp-56]` | `ret(0xfb25c)` |
| `bl` at 0x32824 | `(unknown callee) 0x1b49c` | `?` | `0x1053f4` | `?` | `?` |
| `bl` at 0x3282c | `(unknown callee) 0x1b22c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x3288c | `(unknown callee) 0x1b49c` | `?` | `0x1053f8` | `?` | `?` |
| `bl` at 0x32894 | `(unknown callee) 0x1b22c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x32948 | `(unknown callee) 0x46c64` | `0x0` | `?` | `sp-57` | `?` |
| `bl` at 0x3295c | `(unknown callee) 0x32d38` | `[sp-32]:1` | `[sp-57]:1` | `[sp-57]:1` | `[sp-32]:1` |
| `bl` at 0x32968 | `(unknown callee) 0xfaca8` | `ret(0x32d38)` | `?` | `?` | `ret(0x32d38)` |
| `bl` at 0x329ac | `(unknown callee) 0x32d38` | `[sp-32]:1` | `?` | `?` | `?` |
| `bl` at 0x329b8 | `(unknown callee) 0xfaca8` | `ret(0x32d38)` | `?` | `?` | `ret(0x32d38)` |
| `bl` at 0x32a2c | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x105408` | `?` | `[sp-64]` |
| `bl` at 0x32a34 | `(unknown callee) 0x1b22c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x32a64 | `(unknown callee) 0x1b508` | `([sp-28] + ret(0x1b22c))` | `0x1051dc` | `?` | `[0x147f28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x32350 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ldrls` at 0x3236c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x32430 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x324bc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x324cc | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x32650 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x326b8 | `[sp-44] <= [sp-48] (signed, 32-bit)` | `[sp-44] > [sp-48] (signed, 32-bit)` |
| `beq` at 0x32708 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x32808 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x32818 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x32a68 | `r0` | `ret(0x1b508)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3236c).

## `LinkErrReadStatus`

- `LinkErrReadStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x361b8 to 0x363e0.
  Established. Three early byte stores write `0x1` at entry `r1 +0x0`, then `0x0` at `[sp-24] +0x1`
  and `[sp-24] +0x2` (`strb` at 0x361ec, `strb` at 0x361f8, `strb` at 0x36204). The first call to
  `0xea468` has `[sp-36]`, `0x1000`, `sp-48`, `sp-52` (`bl` at 0x36224); its zero-result branch
  takes the continuation and nonzero falls through (`beq` at 0x36238). A later path clears `[sp-24]
  +0x0` with byte `0x0` (`strb` at 0x36334). Link-status encoding and the merged r0 value remain
  unknown (`ldmdb` at 0x363d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x361cc | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x361d0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x361d8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x361e0 | store | 4 | `sp -0x24` | `0x600` |
| `ldr` at 0x361e4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x361ec | store | 1 | `r1 +0x0` | `0x1` |
| `ldr` at 0x361f0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x361f8 | store | 1 | `[sp-24] +0x1` | `0x0` |
| `ldr` at 0x361fc | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36204 | store | 1 | `[sp-24] +0x2` | `0x0` |
| `ldr` at 0x36214 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3622c | store | 4 | `sp -0x1c` | `ret(0xea468)` |
| `ldr` at 0x36230 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36240 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36244 | store | 4 | `sp -0x40` | `[sp-28]` |
| `ldr` at 0x36250 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x36260 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x36270 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x36274 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3627c | load | 4 | `abs 0x363d4` | `-` |
| `str` at 0x36284 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x36294 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36298 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x362ac | store | 4 | `sp -0x1c` | `ret(0xea468)` |
| `ldr` at 0x362b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x362c4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x362d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x362d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x362ec | store | 4 | `sp -0x1c` | `ret(0xea468)` |
| `ldr` at 0x362f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36304 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x36310 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x36324 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x3632c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36334 | store | 1 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x36338 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36348 | load | 4 | `abs 0x363d8` | `-` |
| `ldrb` at 0x3634c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36350 | load | 4 | `abs 0x363d8` | `-` |
| `ldrb` at 0x36354 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36358 | load | 4 | `(unknown)` | `-` |
| `str` at 0x36360 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36364 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3636c | store | 1 | `[sp-24] +0x1` | `0x1` |
| `str` at 0x36374 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x36378 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x36388 | load | 4 | `abs 0x363dc` | `-` |
| `ldrb` at 0x3638c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36390 | load | 4 | `abs 0x363dc` | `-` |
| `ldrb` at 0x36394 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36398 | load | 4 | `(unknown)` | `-` |
| `str` at 0x363a0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x363a4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x363ac | store | 1 | `[sp-24] +0x2` | `0x1` |
| `str` at 0x363b4 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x363b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x363bc | store | 4 | `sp -0x40` | `[sp-32]` |
| `str` at 0x363c8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x363cc | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0x363d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x363d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x363d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36208 | `(unknown callee) 0xe335c` | `r0` | `r1` | `[sp-24]` | `0x0` |
| `bl` at 0x36224 | `(unknown callee) 0xea468` | `[sp-36]` | `0x1000` | `sp-48` | `sp-52` |
| `bl` at 0x3623c | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x3624c | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x36288 | `(unknown callee) 0xe335c` | `ret(0xe33c8)` | `?` | `?` | `?` |
| `bl` at 0x362a4 | `(unknown callee) 0xea468` | `[sp-36]` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0x362bc | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x362e4 | `(unknown callee) 0xea468` | `[sp-36]` | `[sp-44]` | `sp-56` | `sp-60` |
| `bl` at 0x362fc | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x3630c | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36238 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x3625c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x362b8 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x362f8 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `bne` at 0x3631c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x36344 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x36384 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x363d0 | `r0` | `?` |

## `LinkErrReadBER`

- `LinkErrReadBER`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x363e0 to 0x3690c.
  Established. The call to `0xea56c` carries `[sp-36]`, `[sp-48]`, `0x0`, `0x0` (`bl` at 0x36590).
  Later word stores write `[sp-104]` at `[sp-28] +0xc`, `[sp-100]` at `[sp-28] +0x8`, and `[sp-60]`
  at `[sp-28] +0x4` (`str` at 0x36664, `str` at 0x36670, `str` at 0x3667c). The final multiword
  store has unknown word values at `[sp-28] +0x10` and `[sp-28] +0x14` (`stm` at 0x36850).
  Error-rate calculations, time units and the joined r0 return remain unproved (`ldmdb` at 0x368c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x363f4 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x363f8 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `str` at 0x36400 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x36404 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3640c | store | 1 | `r1 +0x0` | `0x1` |
| `str` at 0x36414 | store | 4 | `sp -0x24` | `0x600` |
| `str` at 0x3641c | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x36424 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x36434 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3644c | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x36450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36460 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36464 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x36470 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x36480 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x36488 | load | 4 | `abs 0x368f4` | `-` |
| `str` at 0x36490 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x36494 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3649c | load | 4 | `abs 0x368f8` | `-` |
| `str` at 0x364a4 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x364a8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x364b0 | load | 4 | `abs 0x368fc` | `-` |
| `str` at 0x364b8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x364c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x364cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x364e0 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x364e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x364f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x364f8 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x36508 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3650c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x36520 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x36524 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36534 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36538 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x36548 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3654c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36560 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x36564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36574 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36578 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x36580 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36584 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x36598 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x365ac | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x365b4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x365bc | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x365c0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x365d4 | store | 4 | `sp -0x84` | `0x1` |
| `str` at 0x365e0 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x365e4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x365e8 | store | 4 | `sp -0x38` | `[sp-132]` |
| `ldr` at 0x365ec | load | 4 | `abs 0x36900` | `-` |
| `ldr` at 0x365f0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x365f8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x365fc | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x36608 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x3660c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x36614 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x36620 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x36624 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x36630 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36638 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x3663c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36644 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x36648 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x36650 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x36658 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x3665c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36660 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36664 | store | 4 | `[sp-28] +0xc` | `[sp-104]` |
| `ldr` at 0x36668 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3666c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x36670 | store | 4 | `[sp-28] +0x8` | `[sp-100]` |
| `ldr` at 0x36674 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36678 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3667c | store | 4 | `[sp-28] +0x4` | `[sp-60]` |
| `ldr` at 0x36680 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36684 | store | 4 | `sp -0x6c` | `[sp-104]` |
| `ldr` at 0x36688 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x3668c | store | 4 | `sp -0x70` | `[sp-100]` |
| `ldr` at 0x36690 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x36694 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x36698 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x366a4 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x366b4 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x366bc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x366c0 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x366c8 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x366cc | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x366d4 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x366d8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x366e4 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x366ec | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x366f0 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x366f8 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x36700 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x3670c | load | 4 | `sp -0x70` | `-` |
| `ldm` at 0x36728 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36728 | load | 4 | `(unknown)` | `-` |
| `stm` at 0x3672c | store | 4 | `(unknown)` | `?` |
| `stm` at 0x3672c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36730 | load | 4 | `abs 0x36904` | `-` |
| `ldr` at 0x3673c | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x36754 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x36758 | load | 4 | `sp -0x74` | `-` |
| `stmib` at 0x36764 | store | 4 | `sp -0x90` | `ret(0xfac7c)` |
| `stmib` at 0x36764 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x36768 | load | 4 | `sp -0x74` | `-` |
| `ldmib` at 0x36778 | load | 4 | `sp -0x90` | `-` |
| `ldmib` at 0x36778 | load | 4 | `sp -0x8c` | `-` |
| `ldm` at 0x36780 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36780 | load | 4 | `(unknown)` | `-` |
| `stmib` at 0x36794 | store | 4 | `sp -0x90` | `ret(0xfa970)` |
| `stmib` at 0x36794 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x36798 | load | 4 | `sp -0x6c` | `-` |
| `stmda` at 0x367a4 | store | 4 | `sp -0x98` | `ret(0xfac7c)` |
| `stmda` at 0x367a4 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x367a8 | load | 4 | `sp -0x6c` | `-` |
| `ldmda` at 0x367b8 | load | 4 | `sp -0x98` | `-` |
| `ldmda` at 0x367b8 | load | 4 | `sp -0x94` | `-` |
| `ldm` at 0x367c0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x367c0 | load | 4 | `(unknown)` | `-` |
| `stmda` at 0x367d4 | store | 4 | `sp -0x98` | `ret(0xfa970)` |
| `stmda` at 0x367d4 | store | 4 | `sp -0x94` | `?` |
| `ldmib` at 0x367dc | load | 4 | `sp -0x90` | `-` |
| `ldmib` at 0x367dc | load | 4 | `sp -0x8c` | `-` |
| `ldmda` at 0x367e4 | load | 4 | `sp -0x98` | `-` |
| `ldmda` at 0x367e4 | load | 4 | `sp -0x94` | `-` |
| `ldm` at 0x36800 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36800 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36810 | load | 4 | `sp -0x88` | `-` |
| `stm` at 0x36814 | store | 4 | `[sp-136] +0x0` | `ret(0xfaf4c)` |
| `stm` at 0x36814 | store | 4 | `[sp-136] +0x4` | `?` |
| `ldm` at 0x3682c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3682c | load | 4 | `(unknown)` | `-` |
| `stm` at 0x36830 | store | 4 | `(unknown)` | `?` |
| `stm` at 0x36830 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36834 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x3684c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3684c | load | 4 | `(unknown)` | `-` |
| `stm` at 0x36850 | store | 4 | `[sp-28] +0x10` | `?` |
| `stm` at 0x36850 | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0x36854 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x3686c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3686c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36874 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36874 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36898 | load | 4 | `abs 0x36908` | `-` |
| `ldm` at 0x3689c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3689c | load | 4 | `(unknown)` | `-` |
| `str` at 0x368a8 | store | 4 | `sp -0x80` | `0x1` |
| `ldr` at 0x368b0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x368b4 | store | 4 | `sp -0x80` | `[sp-52]` |
| `str` at 0x368c0 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x368c4 | load | 4 | `sp -0x80` | `-` |
| `ldmdb` at 0x368c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x368c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x368c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x368c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36428 | `(unknown callee) 0xe335c` | `r0` | `r1` | `r1` | `?` |
| `bl` at 0x36444 | `(unknown callee) 0xea468` | `[sp-36]` | `0x1000` | `sp-76` | `sp-80` |
| `bl` at 0x3645c | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x3646c | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x364bc | `(unknown callee) 0xe335c` | `ret(0xe33c8)` | `?` | `?` | `?` |
| `bl` at 0x364d8 | `(unknown callee) 0xea468` | `[sp-36]` | `[sp-40]` | `sp-84` | `sp-88` |
| `bl` at 0x364f0 | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x36518 | `(unknown callee) 0xea468` | `[sp-36]` | `[sp-44]` | `sp-92` | `sp-96` |
| `bl` at 0x36530 | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x36558 | `(unknown callee) 0xea468` | `[sp-36]` | `[sp-48]` | `sp-100` | `sp-104` |
| `bl` at 0x36570 | `(unknown callee) 0xe33c8` | `ret(0xea468)` | `?` | `?` | `ret(0xea468)` |
| `bl` at 0x36590 | `(unknown callee) 0xea56c` | `[sp-36]` | `[sp-48]` | `0x0` | `0x0` |
| `bl` at 0x36594 | `(unknown callee) 0xe33c8` | `ret(0xea56c)` | `?` | `?` | `?` |
| `bl` at 0x36734 | `(unknown callee) 0x2070c` | `0x106a88` | `sp-20` | `?` | `?` |
| `bl` at 0x3675c | `(unknown callee) 0xfac7c` | `[sp-116]` | `?` | `?` | `?` |
| `bl` at 0x36784 | `(unknown callee) 0xfa970` | `ret(0xfac7c)` | `[sp-140]` | `?` | `?` |
| `bl` at 0x3679c | `(unknown callee) 0xfac7c` | `[sp-108]` | `?` | `?` | `?` |
| `bl` at 0x367c4 | `(unknown callee) 0xfa970` | `ret(0xfac7c)` | `[sp-148]` | `?` | `?` |
| `bl` at 0x367e8 | `(unknown callee) 0xfaf4c` | `[sp-144]` | `[sp-140]` | `?` | `[sp-148]` |
| `bl` at 0x36804 | `(unknown callee) 0xfaf4c` | `ret(0xfaf4c)` | `?` | `?` | `?` |
| `bl` at 0x36878 | `(unknown callee) 0xfb1b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x368a0 | `(unknown callee) 0x2070c` | `0x106aa0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36458 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x3647c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x364ec | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x3652c | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x3656c | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `bne` at 0x365a4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x365cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3662c | `[sp-132] == 0x0` | `[sp-132] != 0x0` |
| `bls` at 0x366a0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x366ac | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x366e0 | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `bne` at 0x36708 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x36714 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x36744 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x36770 | `[sp-116] >= 0x0 (signed, 32-bit)` | `[sp-116] < 0x0 (signed, 32-bit)` |
| `bge` at 0x367b0 | `[sp-108] >= 0x0 (signed, 32-bit)` | `[sp-108] < 0x0 (signed, 32-bit)` |
| `beq` at 0x3685c | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bgt` at 0x36884 | `ret(0xfb1b8) > 0x0 (signed, 32-bit)` | `ret(0xfb1b8) <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x368c8 | `r0` | `?` |

## `LinkErrMonitor`

- `LinkErrMonitor`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3690c to 0x36c48.
  Established. Three word stores clear `0x147f54`, `0x147f58` and `0x147f5c` to `0x0` (`str` at
  0x369b0, `str` at 0x369bc, `str` at 0x369c8). Calls to `0x7a24c` include candidates `0x1`,
  `0x8100`, `0x0`, `sp-268`, followed at another site by `0x1`, `0x8101`, `0x0`, `sp-268` (`bl` at
  0x36ab8, `bl` at 0x36ae8). The later result branch takes its bypass for `ret(0x363e0) != 0x1` and
  falls through on equality (`bne` at 0x36b68). No return is recognized. Physical link monitoring,
  event delivery and interval units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3691c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x36920 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x36928 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3694c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36958 | load | 4 | `abs 0x36c0c` | `-` |
| `str` at 0x36964 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x36968 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36978 | load | 4 | `abs 0x36c10` | `-` |
| `ldr` at 0x3697c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36984 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x36988 | load | 4 | `abs 0x36c14` | `-` |
| `ldr` at 0x3698c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36994 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x36998 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x369a0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x369a8 | load | 4 | `abs 0x36c18` | `-` |
| `str` at 0x369b0 | store | 4 | `abs 0x147f54` | `0x0` |
| `ldr` at 0x369b4 | load | 4 | `abs 0x36c1c` | `-` |
| `str` at 0x369bc | store | 4 | `abs 0x147f58` | `0x0` |
| `ldr` at 0x369c0 | load | 4 | `abs 0x36c20` | `-` |
| `str` at 0x369c8 | store | 4 | `abs 0x147f5c` | `0x0` |
| `str` at 0x369d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x369d4 | load | 4 | `abs 0x36c24` | `-` |
| `str` at 0x369e0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x369e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x369f4 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x36a0c | store | 4 | `sp -0x20` | `ret(0x361b8)` |
| `ldr` at 0x36a10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36a20 | store | 4 | `sp -0x110` | `sp-268` |
| `ldr` at 0x36a24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36a2c | store | 4 | `sp -0x114` | `[sp-28]+65` |
| `ldrb` at 0x36a30 | load | 1 | `sp -0x43` | `-` |
| `ldr` at 0x36a3c | load | 4 | `abs 0x36c28` | `-` |
| `str` at 0x36a40 | store | 4 | `sp -0x118` | `0x106ae8` |
| `ldr` at 0x36a48 | load | 4 | `abs 0x36c2c` | `-` |
| `str` at 0x36a4c | store | 4 | `sp -0x118` | `0x106aec` |
| `ldrb` at 0x36a50 | load | 1 | `sp -0x42` | `-` |
| `ldr` at 0x36a5c | load | 4 | `abs 0x36c28` | `-` |
| `str` at 0x36a60 | store | 4 | `sp -0x11c` | `0x106ae8` |
| `ldr` at 0x36a68 | load | 4 | `abs 0x36c2c` | `-` |
| `str` at 0x36a6c | store | 4 | `sp -0x11c` | `0x106aec` |
| `ldr` at 0x36a70 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x36a74 | store | 4 | `sp -0x124` | `[sp-284]` |
| `ldr` at 0x36a78 | load | 4 | `sp -0x110` | `-` |
| `ldr` at 0x36a7c | load | 4 | `abs 0x36c30` | `-` |
| `ldr` at 0x36a80 | load | 4 | `sp -0x114` | `-` |
| `ldr` at 0x36a84 | load | 4 | `sp -0x118` | `-` |
| `ldrb` at 0x36a8c | load | 1 | `sp -0x43` | `-` |
| `str` at 0x36aa0 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x36aa8 | store | 4 | `sp -0x120` | `0x0` |
| `ldrb` at 0x36abc | load | 1 | `sp -0x42` | `-` |
| `str` at 0x36ad0 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x36ad8 | store | 4 | `sp -0x120` | `0x0` |
| `ldr` at 0x36ae0 | load | 4 | `abs 0x36c34` | `-` |
| `ldr` at 0x36af8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36b00 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x36b0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36b10 | load | 4 | `abs 0x36c38` | `-` |
| `str` at 0x36b1c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x36b20 | load | 4 | `abs 0x142760` | `-` |
| `str` at 0x36b30 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x36b38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x36b3c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x36b4c | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x36b70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36b84 | load | 4 | `(unknown)` | `-` |
| `str` at 0x36b88 | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0x36b8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36b94 | load | 4 | `abs 0x36c3c` | `-` |
| `str` at 0x36ba8 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x36bb0 | store | 4 | `sp -0x120` | `0x0` |
| `ldr` at 0x36bb8 | load | 4 | `abs 0x36c40` | `-` |
| `ldr` at 0x36bd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36bdc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x36bf8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x36bfc | load | 4 | `abs 0x36c44` | `-` |
| `ldr` at 0x36c00 | load | 4 | `abs 0x14275c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3692c | `(unknown callee) 0x37430` | `?` | `?` | `?` | `?` |
| `bl` at 0x3693c | `(unknown callee) 0x3d8e0` | `ret(0x37430)` | `?` | `?` | `ret(0x37430)` |
| `bl` at 0x3695c | `(unknown callee) 0x20240` | `0x106acc` | `?` | `?` | `[sp-36]` |
| `bl` at 0x369d8 | `(unknown callee) 0x37494` | `0x1770` | `?` | `0x147f5c` | `0x0` |
| `bl` at 0x36a04 | `(unknown callee) 0x361b8` | `[sp-28]:1` | `sp-68` | `sp-68` | `[sp-28]:1` |
| `bl` at 0x36a88 | `(unknown callee) 0x1b49c` | `sp-268` | `0x106af0` | `[sp-28]+65` | `[sp-280]` |
| `bl` at 0x36ab8 | `(unknown callee) 0x7a24c` | `0x1` | `0x8100` | `0x0` | `sp-268` |
| `bl` at 0x36ae8 | `(unknown callee) 0x7a24c` | `0x1` | `0x8101` | `0x0` | `sp-268` |
| `bl` at 0x36af4 | `(unknown callee) 0x20240` | `sp-268` | `?` | `?` | `sp-268` |
| `bl` at 0x36b5c | `(unknown callee) 0x363e0` | `[sp-28]:1` | `sp-64` | `sp-64` | `[sp-28]:1` |
| `bl` at 0x36b9c | `(unknown callee) 0x1b49c` | `sp-268` | `0x106b24` | `[sp-28]+65` | `?` |
| `bl` at 0x36bc4 | `(unknown callee) 0x7a24c` | `0x1` | `0x8102` | `0x0` | `sp-268` |
| `bl` at 0x36bd0 | `(unknown callee) 0x20240` | `sp-268` | `?` | `?` | `sp-268` |
| `bl` at 0x36be4 | `(unknown callee) 0x3d8e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x36c04 | `(unknown callee) 0x37494` | `[0x14275c]` | `?` | `?` | `0x14275c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36938 | `ret(0x37430) == 0x0` | `ret(0x37430) != 0x0` |
| `beq` at 0x36948 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `beq` at 0x36954 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `ble` at 0x36970 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x369ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x36a18 | `ret(0x361b8) == 0x0` | `ret(0x361b8) != 0x0` |
| `beq` at 0x36a38 | `[sp-67]:1 == 0x0` | `[sp-67]:1 != 0x0` |
| `beq` at 0x36a58 | `[sp-66]:1 == 0x0` | `[sp-66]:1 != 0x0` |
| `beq` at 0x36a94 | `[sp-67]:1 == 0x0` | `[sp-67]:1 != 0x0` |
| `beq` at 0x36ac4 | `[sp-66]:1 == 0x0` | `[sp-66]:1 != 0x0` |
| `blt` at 0x36b28 | `[sp-40] < [0x142760] (signed, 32-bit)` | `[sp-40] >= [0x142760] (signed, 32-bit)` |
| `ble` at 0x36b44 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x36b68 | `ret(0x363e0) != 0x1` | `ret(0x363e0) == 0x1` |
| `bne` at 0x36bf0 | `ret(0x3d8e0) != 0x0` | `ret(0x3d8e0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `LinkErrQuery`

- `LinkErrQuery`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36c48 to 0x37344.
  Established. The first call targets `0x1b124` with `0x5dc`, entry r1, entry r2, entry r3 (`bl` at
  0x36c64). A word store writes `ret(0x1b124)` at entry `r0 +0x0`, then a byte store writes `0x0` at
  `[[sp-24]] +0x0` (`str` at 0x36c6c, `strb` at 0x36c84). Later calls reach `0x361b8` with
  `[sp-28]:1`, `sp-68`, `sp-68`, `[sp-28]:1` and `0x363e0` with `[sp-28]:1`, `sp-64`, `sp-64`,
  `[sp-28]:1` (`bl` at 0x370e0, `bl` at 0x3721c). The return has r0 `ret(0x1b22c)` (`ldmdb` at
  0x372c0). Buffer extent and error-statistic interpretation remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36c58 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x36c5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36c60 | load | 4 | `abs 0x372c4` | `-` |
| `str` at 0x36c6c | store | 4 | `r0 +0x0` | `ret(0x1b124)` |
| `ldr` at 0x36c70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36c74 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x36c78 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x36c7c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x36c84 | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x36c88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36c98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36ca0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x36cb8 | load | 4 | `abs 0x372c8` | `-` |
| `str` at 0x36cbc | store | 4 | `sp -0x48` | `0x106b40` |
| `ldr` at 0x36cc4 | load | 4 | `abs 0x372cc` | `-` |
| `str` at 0x36cc8 | store | 4 | `sp -0x48` | `0x106b48` |
| `ldr` at 0x36ccc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36cd0 | load | 4 | `abs 0x372d0` | `-` |
| `ldr` at 0x36cd4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36cdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36cec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36cf4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x36cf8 | load | 4 | `abs 0x372d4` | `-` |
| `ldr` at 0x36cfc | load | 4 | `abs 0x142758` | `-` |
| `ldr` at 0x36d08 | load | 4 | `abs 0x372d8` | `-` |
| `str` at 0x36d0c | store | 4 | `sp -0x4c` | `0x106b70` |
| `ldr` at 0x36d14 | load | 4 | `abs 0x372dc` | `-` |
| `str` at 0x36d18 | store | 4 | `sp -0x4c` | `0x106b78` |
| `ldr` at 0x36d1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36d20 | load | 4 | `abs 0x372e0` | `-` |
| `ldr` at 0x36d24 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x36d2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36d3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36d44 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x36d48 | load | 4 | `abs 0x372e4` | `-` |
| `ldr` at 0x36d4c | load | 4 | `abs 0x14275c` | `-` |
| `str` at 0x36d6c | store | 4 | `sp -0x24` | `ret(0xfb614)` |
| `ldr` at 0x36d70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36d74 | load | 4 | `abs 0x372e8` | `-` |
| `ldr` at 0x36d7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36d8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36d94 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x36d98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36d9c | load | 4 | `abs 0x372ec` | `-` |
| `ldr` at 0x36db4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36db8 | load | 4 | `abs 0x372ec` | `-` |
| `ldr` at 0x36dd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36dd8 | load | 4 | `abs 0x372f0` | `-` |
| `ldr` at 0x36dec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36df0 | load | 4 | `abs 0x372f4` | `-` |
| `ldr` at 0x36e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36e0c | load | 4 | `abs 0x372f4` | `-` |
| `ldr` at 0x36e28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36e2c | load | 4 | `abs 0x372f8` | `-` |
| `ldr` at 0x36e40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36e50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36e54 | load | 4 | `abs 0x372fc` | `-` |
| `ldr` at 0x36e64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36e74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36e7c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x36e80 | load | 4 | `abs 0x372e4` | `-` |
| `ldr` at 0x36e84 | load | 4 | `abs 0x37300` | `-` |
| `ldr` at 0x36e88 | load | 4 | `abs 0x14275c` | `-` |
| `ldr` at 0x36e8c | load | 4 | `abs 0x142760` | `-` |
| `str` at 0x36eb0 | store | 4 | `sp -0x28` | `ret(0xfb614)` |
| `ldr` at 0x36eb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36eb8 | load | 4 | `abs 0x37304` | `-` |
| `ldr` at 0x36ec0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36ed0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36ed8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x36edc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36ee0 | load | 4 | `abs 0x372ec` | `-` |
| `ldr` at 0x36ef8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36efc | load | 4 | `abs 0x372ec` | `-` |
| `ldr` at 0x36f18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36f1c | load | 4 | `abs 0x372f0` | `-` |
| `ldr` at 0x36f30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36f34 | load | 4 | `abs 0x372f4` | `-` |
| `ldr` at 0x36f4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36f50 | load | 4 | `abs 0x372f4` | `-` |
| `ldr` at 0x36f6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36f70 | load | 4 | `abs 0x372f8` | `-` |
| `ldr` at 0x36f84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36f88 | load | 4 | `abs 0x37308` | `-` |
| `ldr` at 0x36fa0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36fa4 | load | 4 | `abs 0x372ec` | `-` |
| `ldr` at 0x36fc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36fc4 | load | 4 | `abs 0x372f0` | `-` |
| `ldr` at 0x36fd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36fe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36fec | load | 4 | `abs 0x372fc` | `-` |
| `ldr` at 0x36ffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3700c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37014 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x37018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3701c | load | 4 | `abs 0x3730c` | `-` |
| `ldr` at 0x37024 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37034 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3703c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x37040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37044 | load | 4 | `abs 0x37310` | `-` |
| `ldr` at 0x3704c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3705c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37064 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x37068 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3706c | load | 4 | `abs 0x37314` | `-` |
| `ldr` at 0x37074 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37084 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3708c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x37090 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37094 | load | 4 | `abs 0x37318` | `-` |
| `ldr` at 0x3709c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x370ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x370b4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `str` at 0x370bc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x370c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x370d0 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x370e4 | load | 1 | `sp -0x44` | `-` |
| `ldr` at 0x370f0 | load | 4 | `abs 0x3731c` | `-` |
| `str` at 0x370f4 | store | 4 | `sp -0x5c` | `0x106ca8` |
| `ldr` at 0x370f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x370fc | load | 4 | `abs 0x37320` | `-` |
| `ldr` at 0x37100 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37104 | load | 4 | `abs 0x3731c` | `-` |
| `ldrb` at 0x37110 | load | 1 | `sp -0x43` | `-` |
| `ldrb` at 0x37114 | load | 1 | `sp -0x42` | `-` |
| `str` at 0x37118 | store | 4 | `sp -0x5c` | `[sp-66]:1` |
| `ldr` at 0x3711c | load | 4 | `abs 0x37324` | `-` |
| `ldr` at 0x37120 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37124 | load | 4 | `(unknown)` | `-` |
| `str` at 0x37128 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x3712c | load | 4 | `abs 0x37328` | `-` |
| `ldr` at 0x37130 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37134 | load | 4 | `(unknown)` | `-` |
| `str` at 0x37138 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x3713c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37140 | load | 4 | `abs 0x3732c` | `-` |
| `ldr` at 0x37144 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37150 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37160 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37168 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x3716c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37174 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x3717c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37180 | load | 4 | `abs 0x37330` | `-` |
| `ldr` at 0x37188 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37198 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x371a0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x371a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x371a8 | load | 4 | `abs 0x37334` | `-` |
| `ldr` at 0x371b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x371c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x371c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x371cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x371d0 | load | 4 | `abs 0x37338` | `-` |
| `ldr` at 0x371d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x371e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x371f0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `str` at 0x371f8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x371fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3720c | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x37220 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x3722c | load | 4 | `abs 0x3731c` | `-` |
| `str` at 0x37230 | store | 4 | `sp -0x5c` | `0x106ca8` |
| `ldr` at 0x37234 | load | 4 | `abs 0x3731c` | `-` |
| `str` at 0x37238 | store | 4 | `sp -0x58` | `0x106ca8` |
| `ldr` at 0x3723c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37240 | load | 4 | `abs 0x3733c` | `-` |
| `ldr` at 0x37244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37248 | load | 4 | `abs 0x3731c` | `-` |
| `ldr` at 0x37254 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x37258 | store | 4 | `sp -0x5c` | `[sp-52]` |
| `ldr` at 0x3725c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x37260 | store | 4 | `sp -0x58` | `[sp-60]` |
| `ldm` at 0x37278 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x37278 | load | 4 | `(unknown)` | `-` |
| `stm` at 0x3727c | store | 4 | `sp -0x54` | `?` |
| `stm` at 0x3727c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x37280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37284 | load | 4 | `abs 0x37340` | `-` |
| `ldr` at 0x37288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3728c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x37294 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x372a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x372ac | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x372b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x372b8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x372c0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x372c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x372c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x372c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36c64 | `(unknown callee) 0x1b124` | `0x5dc` | `r1` | `r2` | `r3` |
| `bl` at 0x36c8c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `[[sp-24]]` | `0x0` |
| `bl` at 0x36ca8 | `(unknown callee) 0x1f1dc` | `0x9b` | `?` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x36cd8 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106b54` | `[sp-72]` | `?` |
| `bl` at 0x36ce0 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x36d28 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106b80` | `[sp-76]` | `?` |
| `bl` at 0x36d30 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x36d64 | `(unknown callee) 0xfb614` | `?` | `?` | `[0x14275c]` | `?` |
| `bl` at 0x36d78 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106ba0` | `?` | `ret(0xfb614)` |
| `bl` at 0x36d80 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x36da0 | `(unknown callee) 0xfb990` | `[sp-36]` | `0x476a6000` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x36dbc | `(unknown callee) 0xfb7f0` | `[sp-36]` | `0x476a6000` | `?` | `ret(0xfb990)` |
| `bl` at 0x36dc8 | `(unknown callee) 0xfaca8` | `ret(0xfb7f0)` | `?` | `?` | `ret(0xfb7f0)` |
| `bl` at 0x36de4 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106bc4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x36df4 | `(unknown callee) 0xfb990` | `[sp-36]` | `0x447a0000` | `?` | `ret(0xfb990)` |
| `bl` at 0x36e10 | `(unknown callee) 0xfb7f0` | `[sp-36]` | `0x447a0000` | `?` | `ret(0xfb990)` |
| `bl` at 0x36e1c | `(unknown callee) 0xfaca8` | `ret(0xfb7f0)` | `?` | `?` | `ret(0xfb7f0)` |
| `bl` at 0x36e38 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106bd4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x36e44 | `(unknown callee) 0xfaca8` | `[sp-36]` | `?` | `?` | `ret(0xfb990)` |
| `bl` at 0x36e60 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106be4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x36e68 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x36ea8 | `(unknown callee) 0xfb614` | `?` | `[0x14275c]` | `?` | `?` |
| `bl` at 0x36ebc | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106bf0` | `?` | `ret(0xfb614)` |
| `bl` at 0x36ec4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x36ee4 | `(unknown callee) 0xfb990` | `[sp-40]` | `0x476a6000` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x36f00 | `(unknown callee) 0xfb7f0` | `[sp-40]` | `0x476a6000` | `?` | `ret(0xfb990)` |
| `bl` at 0x36f0c | `(unknown callee) 0xfaca8` | `ret(0xfb7f0)` | `?` | `?` | `ret(0xfb7f0)` |
| `bl` at 0x36f28 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106bc4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x36f38 | `(unknown callee) 0xfb990` | `[sp-40]` | `0x447a0000` | `?` | `ret(0xfb990)` |
| `bl` at 0x36f54 | `(unknown callee) 0xfb7f0` | `[sp-40]` | `0x447a0000` | `?` | `ret(0xfb990)` |
| `bl` at 0x36f60 | `(unknown callee) 0xfaca8` | `ret(0xfb7f0)` | `?` | `?` | `ret(0xfb7f0)` |
| `bl` at 0x36f7c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106bd4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x36f8c | `(unknown callee) 0xfb9a0` | `[sp-40]` | `0x0` | `?` | `ret(0xfb990)` |
| `bl` at 0x36fa8 | `(unknown callee) 0xfb7f0` | `[sp-36]` | `0x476a6000` | `?` | `ret(0xfb9a0)` |
| `bl` at 0x36fb4 | `(unknown callee) 0xfaca8` | `ret(0xfb7f0)` | `?` | `?` | `ret(0xfb7f0)` |
| `bl` at 0x36fd0 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106bc4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x36fdc | `(unknown callee) 0xfaca8` | `[sp-40]` | `?` | `?` | `ret(0xfb9a0)` |
| `bl` at 0x36ff8 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106be4` | `ret(0xfaca8)` | `?` |
| `bl` at 0x37000 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37020 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106c10` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x37028 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37048 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106c2c` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x37050 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37070 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106c58` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x37078 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37098 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106c80` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x370a0 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x370e0 | `(unknown callee) 0x361b8` | `[sp-28]:1` | `sp-68` | `sp-68` | `[sp-28]:1` |
| `bl` at 0x37108 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106cb4` | `[sp-28]` | `0x106ca8` |
| `bl` at 0x3714c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106ccc` | `[sp-28]` | `[sp-67]:1` |
| `bl` at 0x37154 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37184 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106cec` | `ret(0x1b22c)` | `?` |
| `bl` at 0x3718c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x371ac | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106d0c` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x371b4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x371d4 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x106d44` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x371dc | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3721c | `(unknown callee) 0x363e0` | `[sp-28]:1` | `sp-64` | `sp-64` | `[sp-28]:1` |
| `bl` at 0x3724c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106d7c` | `[sp-28]` | `0x106ca8` |
| `bl` at 0x37290 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x106d94` | `[sp-28]` | `[sp-56]` |
| `bl` at 0x37298 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36cb4 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x36d04 | `[0x142758] == 0x0` | `[0x142758] != 0x0` |
| `bge` at 0x36dac | `ret(0xfb990) >= 0x0 (signed, 32-bit)` | `ret(0xfb990) < 0x0 (signed, 32-bit)` |
| `bge` at 0x36e00 | `ret(0xfb990) >= 0x0 (signed, 32-bit)` | `ret(0xfb990) < 0x0 (signed, 32-bit)` |
| `bge` at 0x36ef0 | `ret(0xfb990) >= 0x0 (signed, 32-bit)` | `ret(0xfb990) < 0x0 (signed, 32-bit)` |
| `bge` at 0x36f44 | `ret(0xfb990) >= 0x0 (signed, 32-bit)` | `ret(0xfb990) < 0x0 (signed, 32-bit)` |
| `beq` at 0x36f98 | `ret(0xfb9a0) == 0x0` | `ret(0xfb9a0) != 0x0` |
| `ble` at 0x370c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x370ec | `[sp-68]:1 == 0x0` | `[sp-68]:1 != 0x0` |
| `ble` at 0x37204 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x37228 | `[sp-64]:1 == 0x0` | `[sp-64]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x372c0 | `r0` | `ret(0x1b22c)` |

## `RouteInit`

- `RouteInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb10a0 to 0xb1174.
  Established. The call to `0xb4178` has `0x2b1b60`, `0xffff017`, `0x1`, `0xb1174` (`bl` at
  0xb10fc); its zero result takes the continuation and nonzero falls through (`beq` at 0xb1110). A
  later word store advances `0x149aac` with `[0x149aac]+1` (`str` at 0xb1150). Router registration
  semantics and the joined r0 value remain unknown (`ldmdb` at 0xb1154).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb10b0 | load | 4 | `abs 0xb1158` | `-` |
| `ldr` at 0xb10b4 | load | 4 | `abs 0x149aac` | `-` |
| `ldr` at 0xb10c0 | load | 4 | `abs 0xb115c` | `-` |
| `str` at 0xb10c4 | store | 4 | `sp -0x1c` | `0x1276e4` |
| `ldr` at 0xb10cc | load | 4 | `abs 0xb1160` | `-` |
| `ldr` at 0xb10dc | load | 4 | `abs 0xb1164` | `-` |
| `ldr` at 0xb10ec | load | 4 | `abs 0xb1164` | `-` |
| `ldr` at 0xb10f0 | load | 4 | `abs 0xb1168` | `-` |
| `ldr` at 0xb10f8 | load | 4 | `abs 0xb116c` | `-` |
| `str` at 0xb1104 | store | 4 | `sp -0x14` | `ret(0xb4178)` |
| `ldr` at 0xb1108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1120 | load | 4 | `abs 0xb1170` | `-` |
| `str` at 0xb1124 | store | 4 | `sp -0x1c` | `0x127720` |
| `str` at 0xb1128 | store | 4 | `sp -0x18` | `ret(0x21674)` |
| `ldr` at 0xb1130 | load | 4 | `abs 0xb1160` | `-` |
| `ldr` at 0xb1140 | load | 4 | `abs 0xb1158` | `-` |
| `ldr` at 0xb1144 | load | 4 | `abs 0xb1158` | `-` |
| `ldr` at 0xb1148 | load | 4 | `abs 0x149aac` | `-` |
| `str` at 0xb1150 | store | 4 | `abs 0x149aac` | `[0x149aac]+1` |
| `ldmdb` at 0xb1154 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1154 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1154 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb10d8 | `(unknown callee) 0x21e28` | `0x9` | `0x127708` | `0x2a` | `0x0` |
| `bl` at 0xb10e8 | `(unknown callee) 0xb553c` | `0x2b1b60` | `0x17` | `0x0` | `?` |
| `bl` at 0xb10fc | `(unknown callee) 0xb4178` | `0x2b1b60` | `0xffff017` | `0x1` | `0xb1174` |
| `bl` at 0xb1118 | `(unknown callee) 0x21674` | `ret(0xb4178)` | `?` | `?` | `ret(0xb4178)` |
| `bl` at 0xb113c | `(unknown callee) 0x21e28` | `0x9` | `0x127708` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb10bc | `[0x149aac] == 0x0` | `[0x149aac] != 0x0` |
| `beq` at 0xb1110 | `ret(0xb4178) == 0x0` | `ret(0xb4178) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1154 | `r0` | `?` |

## `RouteIRouter`

- `RouteIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1174 to 0xb133c.
  Established. Two byte stores and a later halfword store each write `0x0` at unknown effective
  targets (`strb` at 0xb11dc, `strb` at 0xb11fc, `strh` at 0xb12a0). The call to `0xb6f64` uses
  `0x2b1b60`, `sp-24`, `0x0`, `0x1` (`bl` at 0xb12e0); its nonzero result takes the arm that stores
  word `[[sp-20]+12]` at `ret(0xb6f64) +0xc`; zero falls through (`bne` at 0xb12f4, `str` at
  0xb1310). The final r0 value is `ret(0xb56f4)` (`ldmdb` at 0xb1328). Packet-field meanings and
  routing effects remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1184 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb1188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb118c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xb1190 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xb1198 | store | 4 | `sp -0x220` | `0x0` |
| `ldr` at 0xb119c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb11a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb11a4 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb11b0 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb11c0 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb11c4 | load | 4 | `abs 0xb132c` | `-` |
| `strb` at 0xb11dc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb11e0 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb11e4 | load | 4 | `abs 0xb132c` | `-` |
| `strb` at 0xb11fc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb1200 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1204 | load | 4 | `abs 0xb1330` | `-` |
| `str` at 0xb1218 | store | 4 | `sp -0x224` | `?` |
| `ldr` at 0xb121c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1220 | load | 4 | `sp -0x220` | `-` |
| `ldrb` at 0xb1234 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1248 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb125c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1260 | store | 4 | `sp -0x228` | `?` |
| `ldr` at 0xb1268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb126c | load | 4 | `sp -0x220` | `-` |
| `str` at 0xb1274 | store | 4 | `sp -0x228` | `ret(0xb7944)` |
| `ldr` at 0xb1278 | load | 4 | `sp -0x228` | `-` |
| `ldr` at 0xb127c | load | 4 | `sp -0x224` | `-` |
| `str` at 0xb1280 | store | 4 | `[sp-548] +0x0` | `?` |
| `ldr` at 0xb1284 | load | 4 | `sp -0x220` | `-` |
| `ldr` at 0xb1288 | load | 4 | `abs 0xb132c` | `-` |
| `strh` at 0xb12a0 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xb12a4 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xb12ac | store | 4 | `sp -0x220` | `[sp-544]+1` |
| `str` at 0xb12bc | store | 4 | `sp -0x234` | `0x2` |
| `ldr` at 0xb12c0 | load | 4 | `sp -0x220` | `-` |
| `str` at 0xb12c4 | store | 4 | `sp -0x230` | `?` |
| `str` at 0xb12cc | store | 4 | `sp -0x22c` | `sp-540` |
| `ldr` at 0xb12d0 | load | 4 | `abs 0xb1334` | `-` |
| `str` at 0xb12e8 | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0xb12ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb12f8 | load | 4 | `abs 0xb1338` | `-` |
| `ldr` at 0xb1304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb130c | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb1310 | store | 4 | `ret(0xb6f64) +0xc` | `[[sp-20]+12]` |
| `ldr` at 0xb1314 | load | 4 | `abs 0xb1334` | `-` |
| `ldr` at 0xb1318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1320 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1328 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1328 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1328 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1270 | `(unknown callee) 0xb7944` | `?` | `?` | `0x24` | `?` |
| `bl` at 0xb12e0 | `(unknown callee) 0xb6f64` | `0x2b1b60` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0xb12fc | `(unknown callee) 0x20240` | `0x127750` | `?` | `?` | `ret(0xb6f64)` |
| `bl` at 0xb131c | `(unknown callee) 0xb6798` | `0x2b1b60` | `[sp-28]` | `[[sp-20]+12]` | `ret(0xb6f64)` |
| `bl` at 0xb1324 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb11ac | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xb11b8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb1240 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb12f4 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1328 | `r0` | `ret(0xb56f4)` |

## `RouteProcessIn`

- `RouteProcessIn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb133c to 0xb14ec.
  Established. The absolute-word guard takes the bypass for `[0x149ab0] != 0x0` and falls through
  otherwise (`bne` at 0xb1378). The fall-through call to `0x74908` has `0x0`, entry r1, entry r2,
  `[0x149ab0]`, with its result stored as a word at `0x149ab0` (`bl` at 0xb1380, `str` at 0xb138c).
  Later word stores have unknown targets and values `[sp-44]` and `ret(0x1b1e4)`, respectively
  (`str` at 0xb14a4, `str` at 0xb14c8). Inbound route encoding and the returned r0 value remain
  unknown (`ldmdb` at 0xb14e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb134c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb1350 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb1354 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb1358 | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xb1368 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb136c | load | 4 | `abs 0xb14e8` | `-` |
| `ldr` at 0xb1370 | load | 4 | `abs 0x149ab0` | `-` |
| `ldr` at 0xb1388 | load | 4 | `abs 0xb14e8` | `-` |
| `str` at 0xb138c | store | 4 | `abs 0x149ab0` | `ret(0x74908)` |
| `str` at 0xb1394 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xb139c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb13a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb13ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb13bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb13c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb13d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb13e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb13e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb13fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb140c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1410 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1424 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb142c | load | 4 | `abs 0xb14e8` | `-` |
| `ldr` at 0xb1430 | load | 4 | `abs 0x149ab0` | `-` |
| `ldr` at 0xb1434 | load | 4 | `[0x149ab0] +0xc` | `-` |
| `ldr` at 0xb1444 | load | 4 | `abs 0xb14e8` | `-` |
| `ldr` at 0xb1448 | load | 4 | `abs 0x149ab0` | `-` |
| `ldr` at 0xb144c | load | 4 | `[0x149ab0] +0x8` | `-` |
| `ldr` at 0xb145c | load | 4 | `abs 0xb14e8` | `-` |
| `ldr` at 0xb1460 | load | 4 | `abs 0x149ab0` | `-` |
| `ldr` at 0xb1464 | load | 4 | `[0x149ab0] +0x10` | `-` |
| `ldr` at 0xb1474 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1484 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb148c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb14a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb14a4 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xb14a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb14ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb14c8 | store | 4 | `(unknown)` | `ret(0x1b1e4)` |
| `str` at 0xb14d0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xb14d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb14dc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xb14e4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb14e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb14e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb14e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1380 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `[0x149ab0]` |
| `bl` at 0xb1414 | `(unknown callee) 0xb7944` | `?` | `?` | `0x24` | `?` |
| `bl` at 0xb14c4 | `(unknown callee) 0x1b1e4` | `0x0` | `[sp-24]` | `0x30` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1378 | `[0x149ab0] != 0x0` | `[0x149ab0] == 0x0` |
| `bne` at 0xb13a8 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xb13b4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb13e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb1404 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb1420 | `ret(0xb7944) != 0x0` | `ret(0xb7944) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb14e4 | `r0` | `?` |

## `RouteProcessOut`

- `RouteProcessOut`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb14ec to 0xb1650.
  Established. One word store writes `ret(0x74908)` at `0x149ab4` (`str` at 0xb153c). A later call
  to `0xb7944` carries unknown, unknown, `0x24`, unknown (`bl` at 0xb15c4); its nonzero result takes
  the update arm and zero falls through (`bne` at 0xb15d0). The update word store has unknown target
  and value `[sp-40]` (`str` at 0xb162c). Outbound route fields and the joined r0 return remain
  unidentified (`ldmdb` at 0xb1648).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb14fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb1500 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb1504 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1508 | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xb1518 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb151c | load | 4 | `abs 0xb164c` | `-` |
| `ldr` at 0xb1520 | load | 4 | `abs 0x149ab4` | `-` |
| `ldr` at 0xb1538 | load | 4 | `abs 0xb164c` | `-` |
| `str` at 0xb153c | store | 4 | `abs 0x149ab4` | `ret(0x74908)` |
| `str` at 0xb1544 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xb154c | store | 4 | `sp -0x1c` | `0x3f` |
| `ldr` at 0xb1550 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb155c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb156c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1570 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1584 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb15ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb15bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb15c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb15d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb15dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb15f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb15f4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb15f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1600 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb160c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1610 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1614 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1628 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb162c | store | 4 | `(unknown)` | `[sp-40]` |
| `str` at 0xb1634 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xb1638 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1640 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldmdb` at 0xb1648 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1648 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1648 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1530 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `[0x149ab4]` |
| `bl` at 0xb15c4 | `(unknown callee) 0xb7944` | `?` | `?` | `0x24` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1528 | `[0x149ab4] != 0x0` | `[0x149ab4] == 0x0` |
| `bne` at 0xb1558 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xb1564 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xb1590 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb15b4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb15d0 | `ret(0xb7944) != 0x0` | `ret(0xb7944) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1648 | `r0` | `?` |

## `routerExitServiceMode`

- `routerExitServiceMode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7480 to 0xc75ac.
  Established. The branch takes the alternate path for `[sp-20] != 0x1` and falls through on
  equality (`bne` at 0xc74d4). That alternate path stores word `0x2` at `0x14829c` (`str` at
  0xc74f4). Calls on the shared prefix to `0x1d38c` include `0x3e`, `0x0`, unknown, unknown and
  `0x3f`, `0x0`, unknown, unknown (`bl` at 0xc74bc, `bl` at 0xc74c8). The return has r0
  `ret(0x20240)` (`ldmdb` at 0xc759c). Service-mode state changes and physical power effects remain
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7490 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc7494 | load | 4 | `abs 0xc75a0` | `-` |
| `ldm` at 0xc749c | load | 4 | `abs 0x132ee0` | `-` |
| `ldm` at 0xc749c | load | 4 | `abs 0x132ee4` | `-` |
| `ldm` at 0xc749c | load | 4 | `abs 0x132ee8` | `-` |
| `stm` at 0xc74a0 | store | 4 | `sp -0x20` | `[0x132ee0]` |
| `stm` at 0xc74a0 | store | 4 | `sp -0x1c` | `[0x132ee4]` |
| `strb` at 0xc74a4 | store | 1 | `sp -0x18` | `([0x132ee8] & 0xff)` |
| `ldr` at 0xc74cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc74ec | load | 4 | `abs 0xc75a4` | `-` |
| `str` at 0xc74f4 | store | 4 | `abs 0x14829c` | `0x2` |
| `ldr` at 0xc7594 | load | 4 | `abs 0xc75a8` | `-` |
| `ldmdb` at 0xc759c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc759c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc759c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc74b0 | `(unknown callee) 0x1d38c` | `0x50` | `0x0` | `[0x132ee8]` | `0x132ee0` |
| `bl` at 0xc74bc | `(unknown callee) 0x1d38c` | `0x3e` | `0x0` | `?` | `?` |
| `bl` at 0xc74c8 | `(unknown callee) 0x1d38c` | `0x3f` | `0x0` | `?` | `?` |
| `bl` at 0xc74e4 | `(unknown callee) 0x1d38c` | `0xd` | `sp-32` | `?` | `sp-32` |
| `bl` at 0xc7504 | `(unknown callee) 0x1d38c` | `0xd` | `sp-32` | `0x14829c` | `sp-32` |
| `bl` at 0xc7510 | `(unknown callee) 0x1d38c` | `0x3d` | `0x2` | `?` | `?` |
| `bl` at 0xc751c | `(unknown callee) 0x1d38c` | `0x7b` | `0x1` | `?` | `?` |
| `bl` at 0xc7520 | `(unknown callee) 0x4f090` | `ret(0x1d38c)` | `?` | `?` | `?` |
| `bl` at 0xc7534 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0xc7548 | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0xc755c | `(unknown callee) 0x4f090` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0xc7580 | `(unknown callee) 0x42178` | `0x0` | `0x6` | `0x2` | `?` |
| `bl` at 0xc7590 | `(unknown callee) 0x42178` | `0x0` | `0x7` | `0x2` | `?` |
| `bl` at 0xc7598 | `(unknown callee) 0x20240` | `0x132eec` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc74d4 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0xc7530 | `? == 0x52` | `? != 0x52` |
| `beq` at 0xc7544 | `? == 0x44` | `? != 0x44` |
| `beq` at 0xc7558 | `? == 0x36` | `? != 0x36` |
| `beq` at 0xc756c | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc759c | `r0` | `ret(0x20240)` |

## `brickGetSerial`

- `brickGetSerial`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f2f0 to 0x4f564.
  Established. The computed transfer takes an unresolved edge for `[0x14279c]:1-35 <= 0x59` unsigned
  and falls through above the bound (`ldrls` at 0x4f350). The known later call to `0x1afa4` has all
  four candidates unknown (`bl` at 0x4f53c). A word store writes its result at `0x14824c` (`str` at
  0x4f544). Excluded selector cases, serial-record layout and the joined r0 value remain unknown
  (`ldmdb` at 0x4f558).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f304 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4f308 | load | 4 | `abs 0x4f55c` | `-` |
| `ldr` at 0x4f30c | load | 4 | `abs 0x14824c` | `-` |
| `str` at 0x4f338 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4f340 | load | 4 | `abs 0x4f560` | `-` |
| `ldrb` at 0x4f344 | load | 1 | `abs 0x14279c` | `-` |
| `ldrls` at 0x4f350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f4c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f4dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f4e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f4f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f504 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f50c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f518 | load | 4 | `abs 0x4f55c` | `-` |
| `ldr` at 0x4f51c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f530 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f538 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f544 | store | 4 | `abs 0x14824c` | `ret(0x1afa4)` |
| `ldr` at 0x4f548 | load | 4 | `abs 0x4f55c` | `-` |
| `ldr` at 0x4f54c | load | 4 | `abs 0x14824c` | `-` |
| `str` at 0x4f550 | store | 4 | `sp -0x24` | `[0x14824c]` |
| `ldr` at 0x4f554 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4f558 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4f558 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f558 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4f324 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x4f53c | `(unknown callee) 0x1afa4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f314 | `[0x14824c] != 0x0` | `[0x14824c] == 0x0` |
| `bne` at 0x4f330 | `ret(0xd1dfc) != 0x0` | `ret(0xd1dfc) == 0x0` |
| `ldrls` at 0x4f350 | `[0x14279c]:1-35 <= 0x59 (unsigned, 32-bit)` | `[0x14279c]:1-35 > 0x59 (unsigned, 32-bit)` |
| `bne` at 0x4f4ec | `? != 0x1` | `? == 0x1` |
| `beq` at 0x4f514 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f558 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4f350).

## `powerQuery`

- `powerQuery`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x945c0 to 0x949d8.
  Established. The first call to `0x3c80c` has `0x0`, `sp-36`, entry r2, `sp-36` (`bl` at 0x945ec);
  its nonzero result takes the longer arm and zero falls through (`bne` at 0x945fc). Both arms
  include word stores of `ret(0x1b124)` at `[sp-24] +0x0` (`str` at 0x94610, `str` at 0x94650). A
  later call to `0x3dfc0` uses `0x0`, `[sp-44]`, `sp-57`, `sp-57` (`bl` at 0x948e0). Physical
  power-status interpretation, output extent and the joined r0 value remain unknown (`ldmdb` at
  0x94990).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x945d0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x945d4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x945dc | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x945f0 | store | 4 | `sp -0x28` | `ret(0x3c80c)` |
| `ldr` at 0x945f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x94600 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x94610 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x94614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94618 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x9461c | load | 4 | `abs 0x94994` | `-` |
| `ldr` at 0x94628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9462c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x94650 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x94654 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x94658 | store | 4 | `sp -0x4c` | `[[sp-24]]` |
| `ldr` at 0x9465c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x94668 | load | 4 | `abs 0x94998` | `-` |
| `str` at 0x9466c | store | 4 | `sp -0x50` | `0x11c5b0` |
| `ldr` at 0x94674 | load | 4 | `abs 0x9499c` | `-` |
| `str` at 0x94678 | store | 4 | `sp -0x50` | `0x11574c` |
| `ldr` at 0x9467c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x94688 | load | 4 | `abs 0x949a0` | `-` |
| `str` at 0x9468c | store | 4 | `sp -0x54` | `0x11c5b8` |
| `ldr` at 0x94694 | load | 4 | `abs 0x9499c` | `-` |
| `str` at 0x94698 | store | 4 | `sp -0x54` | `0x11574c` |
| `ldr` at 0x9469c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x946a0 | load | 4 | `abs 0x949a4` | `-` |
| `ldr` at 0x946a4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x946a8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x946b0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x946c0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x946c8 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b22c))` |
| `str` at 0x946d0 | store | 4 | `sp -0x2c` | `0x0` |
| `strb` at 0x946d8 | store | 1 | `sp -0x2d` | `?` |
| `ldr` at 0x946dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x946e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x946f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x94700 | store | 4 | `sp -0x34` | `ret(0x3fff0)` |
| `ldr` at 0x94704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x94710 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x94724 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x9472c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x9473c | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0x94744 | store | 1 | `sp -0x2d` | `([sp-45]:1+1 & 0xff)` |
| `ldr` at 0x9474c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x94764 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x94770 | store | 4 | `sp -0x40` | `ret(0x3db38)` |
| `ldr` at 0x94774 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x94778 | load | 4 | `abs 0x949a8` | `-` |
| `ldr` at 0x94784 | load | 4 | `abs 0x949ac` | `-` |
| `str` at 0x94788 | store | 4 | `sp -0x44` | `0x11c624` |
| `ldr` at 0x94790 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9479c | load | 4 | `abs 0x949b0` | `-` |
| `str` at 0x947a0 | store | 4 | `sp -0x58` | `0x11264c` |
| `ldr` at 0x947a8 | load | 4 | `abs 0x949b4` | `-` |
| `str` at 0x947ac | store | 4 | `sp -0x58` | `0x112668` |
| `ldr` at 0x947b0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x947b4 | store | 4 | `sp -0x44` | `[sp-88]` |
| `ldr` at 0x947b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x947cc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x947d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x947e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x947e4 | load | 4 | `abs 0x949b8` | `-` |
| `ldr` at 0x947e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x947ec | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x94800 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x94810 | store | 4 | `sp -0x20` | `ret(0x3fcf8)` |
| `ldr` at 0x94814 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x94820 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x94824 | store | 4 | `sp -0x64` | `ret(0x3fcf8)` |
| `ldr` at 0x94828 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9482c | load | 4 | `abs 0x949bc` | `-` |
| `ldr` at 0x94830 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x94834 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x94840 | load | 4 | `sp -0x48` | `-` |
| `stm` at 0x94850 | store | 4 | `sp -0x64` | `ret(0xfaca8)` |
| `stm` at 0x94850 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x94854 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94858 | load | 4 | `abs 0x949c0` | `-` |
| `ldr` at 0x9485c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x94860 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x94868 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94878 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x94880 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b22c))` |
| `ldr` at 0x94884 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x94898 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x948a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x948ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x948b0 | load | 4 | `abs 0x949c4` | `-` |
| `ldr` at 0x948c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x948cc | store | 4 | `sp -0x38` | `ret(0x400e0)` |
| `ldr` at 0x948d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x948e8 | store | 4 | `sp -0x20` | `ret(0x3dfc0)` |
| `ldr` at 0x948ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x948f8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x948fc | load | 4 | `abs 0x949c8` | `-` |
| `ldr` at 0x94900 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x94904 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x94910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9491c | load | 4 | `abs 0x949cc` | `-` |
| `str` at 0x94920 | store | 4 | `sp -0x5c` | `0x11c698` |
| `ldr` at 0x94928 | load | 4 | `abs 0x949d0` | `-` |
| `str` at 0x9492c | store | 4 | `sp -0x5c` | `0x11c6a8` |
| `ldrb` at 0x94930 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x94934 | load | 1 | `sp -0x2d` | `-` |
| `str` at 0x94938 | store | 4 | `sp -0x64` | `[sp-45]:1` |
| `ldr` at 0x9493c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94940 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x94944 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x94954 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94958 | load | 4 | `abs 0x949d4` | `-` |
| `ldr` at 0x9495c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x94964 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x94974 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9497c | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b22c))` |
| `ldr` at 0x94980 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x94988 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x94990 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x94990 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94990 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94990 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x945ec | `(unknown callee) 0x3c80c` | `0x0` | `sp-36` | `r2` | `sp-36` |
| `bl` at 0x94608 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `ret(0x3c80c)` |
| `bl` at 0x94620 | `(unknown callee) 0x1b508` | `[[sp-24]]` | `0x11c588` | `?` | `[sp-24]` |
| `bl` at 0x94648 | `(unknown callee) 0x1b124` | `?` | `?` | `ret(0x3c80c)` | `?` |
| `bl` at 0x946ac | `(unknown callee) 0x1b49c` | `[[sp-24]]` | `0x11c5c0` | `[sp-80]` | `[sp-84]` |
| `bl` at 0x946b4 | `(unknown callee) 0x1b22c` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x946f8 | `(unknown callee) 0x3fff0` | `0x0` | `[sp-44]` | `[sp-44]` | `[sp-40]` |
| `bl` at 0x94750 | `(unknown callee) 0x3c86c` | `0x0` | `[sp-44]` | `?` | `[sp-45]:1+1` |
| `bl` at 0x94768 | `(unknown callee) 0x3db38` | `0x0` | `[sp-44]` | `?` | `ret(0x3c86c)` |
| `bl` at 0x947f0 | `(unknown callee) 0x1b49c` | `[sp-76]` | `0x11c628` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x94808 | `(unknown callee) 0x3fcf8` | `0x0` | `[sp-44]` | `0x0` | `sp-72` |
| `bl` at 0x94838 | `(unknown callee) 0x1b49c` | `[sp-76]` | `0x11c644` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x94844 | `(unknown callee) 0xfaca8` | `[sp-72]` | `?` | `?` | `ret(0x3fcf8)` |
| `bl` at 0x94864 | `(unknown callee) 0x1b49c` | `[sp-76]` | `0x11c660` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x9486c | `(unknown callee) 0x1b22c` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x948b4 | `(unknown callee) 0x1b49c` | `[sp-76]` | `0x11c678` | `?` | `?` |
| `bl` at 0x948c4 | `(unknown callee) 0x400e0` | `0x0` | `[sp-44]` | `?` | `?` |
| `bl` at 0x948e0 | `(unknown callee) 0x3dfc0` | `0x0` | `[sp-44]` | `sp-57` | `sp-57` |
| `bl` at 0x94908 | `(unknown callee) 0x1b49c` | `[sp-76]` | `0x11c684` | `[sp-56]` | `ret(0x3dfc0)` |
| `bl` at 0x9494c | `(unknown callee) 0x1b49c` | `[sp-76]` | `[sp-92]` | `[sp-56]` | `[sp-57]:1` |
| `bl` at 0x94960 | `(unknown callee) 0x1b49c` | `[sp-76]` | `0x11c6b4` | `[sp-52]` | `ret(0x3c86c)` |
| `bl` at 0x94968 | `(unknown callee) 0x1b22c` | `[sp-76]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x945fc | `ret(0x3c80c) != 0x0` | `ret(0x3c80c) == 0x0` |
| `beq` at 0x94664 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x94684 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blo` at 0x946e8 | `[sp-44] < [sp-40] (unsigned, 32-bit)` | `[sp-44] >= [sp-40] (unsigned, 32-bit)` |
| `beq` at 0x9470c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x94734 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x9475c | `ret(0x3c86c) == 0x0` | `ret(0x3c86c) != 0x0` |
| `bne` at 0x94780 | `ret(0x3db38) != 0xfffff9b8` | `ret(0x3db38) == 0xfffff9b8` |
| `bne` at 0x94798 | `ret(0x3db38) != 0x1` | `ret(0x3db38) == 0x1` |
| `bne` at 0x947dc | `? != 0xff` | `? == 0xff` |
| `beq` at 0x9481c | `ret(0x3fcf8) == 0x0` | `ret(0x3fcf8) != 0x0` |
| `bne` at 0x948a8 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x948f4 | `ret(0x3dfc0) == 0x0` | `ret(0x3dfc0) != 0x0` |
| `beq` at 0x94918 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94990 | `r0` | `?` |

## `i2cTest`

- `i2cTest`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb2c90 to 0xb2f3c.
  Established. The four opening calls have candidates `0x0`, `sp-28`, entry r2, `sp-28`; `0x0`,
  `sp-36`, unknown, `sp-36`; `0x0`, `sp-44`, unknown, `sp-44`; and `0x0`, `sp-52`, unknown, `sp-52`
  (`bl` at 0xb2cb8, `bl` at 0xb2cd0, `bl` at 0xb2ce8, `bl` at 0xb2d00). Their numeric targets are
  `0xd4aac`, `0xd687c`, `0xd1da8` and `0xcc7c8`, respectively (`bl` at 0xb2cb8, `bl` at 0xb2cd0,
  `bl` at 0xb2ce8, `bl` at 0xb2d00). Later word stores accumulate `([sp-60] + ret(0xb2f3c))` and
  `([sp-60] + ret(0xb3070))` at `sp -0x3c` (`str` at 0xb2d8c, `str` at 0xb2e04). The final zero test
  takes the continuation for `[sp-60] == 0x0` and falls through otherwise (`beq` at 0xb2f0c). Actual
  I2C test coverage and the merged r0 return remain unproved (`ldmdb` at 0xb2f38).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb2ca0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb2ca8 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xb2cc0 | store | 4 | `sp -0x18` | `ret(0xd4aac)` |
| `str` at 0xb2cd8 | store | 4 | `sp -0x20` | `ret(0xd687c)` |
| `str` at 0xb2cf0 | store | 4 | `sp -0x28` | `ret(0xd1da8)` |
| `str` at 0xb2d08 | store | 4 | `sp -0x30` | `ret(0xcc7c8)` |
| `str` at 0xb2d10 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb2d14 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb2d18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb2d2c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb2d30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2d34 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb2d44 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2d60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2d68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb2d70 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2d84 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb2d8c | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb2f3c))` |
| `ldr` at 0xb2d90 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb2d98 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xb2da4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb2da8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2dac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb2dbc | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2dd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2de0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb2de8 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2dfc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb2e04 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb3070))` |
| `ldr` at 0xb2e08 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb2e10 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xb2e1c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb2e20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2e24 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb2e34 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2e50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2e64 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb2e6c | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2e80 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb2e88 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb31a8))` |
| `ldr` at 0xb2e8c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb2e94 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xb2ea0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb2ea4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb2ea8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xb2eb8 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0xb2ed4 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0xb2ee8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb2ef0 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xcdfcc))` |
| `ldr` at 0xb2ef4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb2efc | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xb2f04 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb2f14 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb2f1c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb2f24 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xb2f30 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xb2f34 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xb2f38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb2f38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb2f38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb2cb8 | `(unknown callee) 0xd4aac` | `0x0` | `sp-28` | `r2` | `sp-28` |
| `bl` at 0xb2cd0 | `(unknown callee) 0xd687c` | `0x0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xb2ce8 | `(unknown callee) 0xd1da8` | `0x0` | `sp-44` | `?` | `sp-44` |
| `bl` at 0xb2d00 | `(unknown callee) 0xcc7c8` | `0x0` | `sp-52` | `?` | `sp-52` |
| `bl` at 0xb2d50 | `(unknown callee) 0xd4b58` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb2d7c | `(unknown callee) 0xb2f3c` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0xb2dc8 | `(unknown callee) 0xd6928` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb2df4 | `(unknown callee) 0xb3070` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0xb2e40 | `(unknown callee) 0xd1e50` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb2e78 | `(unknown callee) 0xb31a8` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0xb2ec4 | `(unknown callee) 0xcc494` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |
| `bl` at 0xb2ee0 | `(unknown callee) 0xcdfcc` | `0x0` | `[sp-64]:1` | `?` | `[sp-64]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb2d20 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb2d3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb2d5c | `ret(0xd4b58) == 0x0` | `ret(0xd4b58) != 0x0` |
| `blt` at 0xb2db4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb2dd4 | `ret(0xd6928) == 0x0` | `ret(0xd6928) != 0x0` |
| `blt` at 0xb2e2c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb2e4c | `ret(0xd1e50) == 0x0` | `ret(0xd1e50) != 0x0` |
| `blt` at 0xb2eb0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb2ed0 | `ret(0xcc494) == 0x0` | `ret(0xcc494) != 0x0` |
| `beq` at 0xb2f0c | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb2f38 | `r0` | `?` |
