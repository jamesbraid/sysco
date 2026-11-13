# ARM discovery and request startup paths in 1.44.0

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
Computed transfers in L1CfgQuery and B2BInit remain unresolved. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/discovery-request-startup-facts.txt) retain all tool sections.

## `L1CfgFindL2`

- `L1CfgFindL2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x652dc to 0x66694.
  Established. The unnamed `0x488cc` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x652ec).
  A nonzero `[sp-28]` takes `0x65380`, while zero falls through (`bne` at 0x65354).
  The word through `[sp-28] +0x8` is read (`ldr` at 0x65a74).
  L2 selection field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x652f8 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x65300 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x65308 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x6530c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65310 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x65314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65320 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65338 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65340 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6534c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65358 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6535c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65370 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x65378 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x65380 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65384 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65398 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x653a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x653a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x653a8 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x653b4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x653b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x653cc | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x653d4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x653dc | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x653e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x653f4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x653fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65400 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65404 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x65410 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65418 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x65434 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6543c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65448 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6544c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65450 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x6546c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65474 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65484 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65488 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6548c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x654a0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x654a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x654c4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x654cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x654d8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x654dc | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x654e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x654f4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x654fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65518 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65520 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6552c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65530 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65548 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65550 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6556c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65574 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65580 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65584 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6559c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x655a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x655c0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x655c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x655d4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x655d8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x655dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x655f0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x655f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65614 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6561c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65628 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6562c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65644 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6564c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65668 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6567c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65680 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65698 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x656a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x656bc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x656c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x656d0 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x656d4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x656d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x656ec | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x656f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65710 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65718 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65724 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65728 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6572c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65740 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65748 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65764 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6576c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65778 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6577c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65794 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6579c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x657b8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x657c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x657cc | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x657d0 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x657d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x657e8 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x657f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6580c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65820 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65824 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6583c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65844 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65860 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65868 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65874 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65878 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6587c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65890 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x658b4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x658bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x658c8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x658cc | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x658d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x658e4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x658ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65908 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6591c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65920 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65938 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65940 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6595c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65964 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65970 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65974 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6598c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65994 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x659b0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x659b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x659c4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x659c8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x659cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x659e0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x659e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65a04 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65a0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65a18 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65a1c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65a20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65a34 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65a3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65a58 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65a60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65a70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65a74 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65a84 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65a94 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65aa4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65ab4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65ac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65ac4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65ad4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65ae0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65ae4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65af0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65af4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b04 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b14 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b24 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b34 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b44 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b54 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b64 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b74 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65b84 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x65b90 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65b94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65ba8 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x65bb0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x65bb8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65bd0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65bdc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65be0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x65bec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65bf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65c08 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65c20 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x65c28 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x65c30 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65c48 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65c50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65c54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65c58 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x65c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65c70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65c80 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65c84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65c98 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x65ca0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x65ca4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x65cac | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x65cb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65cc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x65cc4 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x65cd0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x65cd4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65cd8 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x65cdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65ce8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65cec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65d00 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65d08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65d14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65d20 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65d24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65d38 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x65d40 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x65d48 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65d60 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65d68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65d6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65d70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65d7c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65d80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65d94 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x65d9c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x65da4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65da8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65dbc | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65dc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65dcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65dd8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65ddc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65de0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65dfc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65e04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65e10 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65e14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65e18 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x65e34 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65e3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65e50 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65e54 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65e6c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65e74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65e90 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65e98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65ea4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65ea8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65eac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65ec0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65ec8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65ee4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65eec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65ef8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65efc | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65f14 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65f1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65f38 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65f40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65f4c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65f50 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65f68 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65f70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65f8c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65f94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65fa0 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65fa4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65fbc | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x65fc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65fe0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x65fe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65ff4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65ff8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x65ffc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66010 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66018 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66034 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6603c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66048 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6604c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66064 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6606c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66088 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x66090 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6609c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x660a0 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x660a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x660b8 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x660c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x660dc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x660e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x660f0 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x660f4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x660f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6610c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66114 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66130 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x66138 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66144 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66148 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6614c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66160 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66168 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66184 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6618c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66198 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6619c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x661a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x661b4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x661bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x661d8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x661e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x661ec | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x661f0 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x661f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66208 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66210 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6622c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x66234 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66240 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66244 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66248 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6625c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66264 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66280 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x66288 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66294 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66298 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x6629c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x662b0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x662b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x662d4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x662dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x662e8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x662ec | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x662f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66304 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6630c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66328 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x66330 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6633c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66340 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66358 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6637c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x66384 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66390 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66394 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x663ac | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x663b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x663d0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x663d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x663e4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x663e8 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x663ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66400 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x66408 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66424 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6642c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6643c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66440 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6644c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66450 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6645c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66460 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6646c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66470 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6647c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66480 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6648c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66490 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6649c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x664ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664b0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x664bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664c0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x664cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664d0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x664dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664e0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x664ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664f0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x664fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66500 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6650c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66510 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6651c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66520 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6652c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66530 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6653c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66540 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6654c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66550 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x6655c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66574 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x6657c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x66584 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6659c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x665a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x665a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x665ac | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x665b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x665c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x665d4 | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x665d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x665ec | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x665f4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x665fc | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66614 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6661c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66624 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x66630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6663c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6664c | load | 4 | `abs 0x65e4c` | `-` |
| `ldr` at 0x66650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66664 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x6666c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x66670 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x66678 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x66680 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66684 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x66688 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x66690 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x66690 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x66690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x652ec | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x6531c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x65348 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x65354 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhs` at 0x653b0 | `? >= [[sp-28]+20] (unsigned, 32-bit)` | `? < [[sp-28]+20] (unsigned, 32-bit)` |
| `bne` at 0x6540c | `? != [[sp-28]+20]` | `? == [[sp-28]+20]` |
| `beq` at 0x65444 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x6547c | `? == 0x44` | `? != 0x44` |
| `beq` at 0x654d4 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x65528 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x6557c | `? == 0x35` | `? != 0x35` |
| `beq` at 0x655d0 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x65624 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x65678 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x656cc | `? == 0x39` | `? != 0x39` |
| `beq` at 0x65720 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x65774 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x657c8 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x6581c | `? == 0x51` | `? != 0x51` |
| `beq` at 0x65870 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x658c4 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x65918 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x6596c | `? == 0x46` | `? != 0x46` |
| `beq` at 0x659c0 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x65a14 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x65a68 | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x65a7c | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x65a8c | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x65a9c | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x65aac | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x65abc | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x65acc | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x65adc | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x65aec | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x65afc | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x65b0c | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x65b1c | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x65b2c | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x65b3c | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x65b4c | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x65b5c | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x65b6c | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x65b7c | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x65b8c | `[[sp-28]+8] == 0x7c` | `[[sp-28]+8] != 0x7c` |
| `bhs` at 0x65be8 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x65bf4 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x65c00 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `bhs` at 0x65c60 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x65c6c | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x65c78 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0x65cbc | `Z == 1` | `not (Z == 1)` |
| `ble` at 0x65ce4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x65d10 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x65d1c | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x65d78 | `C == 1` | `not (C == 1)` |
| `bne` at 0x65dd4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x65e0c | `? == 0x52` | `? != 0x52` |
| `beq` at 0x65e44 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x65ea0 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x65ef4 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x65f48 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x65f9c | `? == 0x34` | `? != 0x34` |
| `beq` at 0x65ff0 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x66044 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x66098 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x660ec | `? == 0x37` | `? != 0x37` |
| `beq` at 0x66140 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x66194 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x661e8 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x6623c | `? == 0x40` | `? != 0x40` |
| `beq` at 0x66290 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x662e4 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x66338 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x6638c | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x663e0 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x66434 | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x66448 | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x66458 | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x66468 | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x66478 | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x66488 | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x66498 | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x664a8 | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x664b8 | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x664c8 | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x664d8 | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x664e8 | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x664f8 | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x66508 | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x66518 | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x66528 | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x66538 | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x66548 | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x66558 | `[[sp-28]+8] == 0x7c` | `[[sp-28]+8] != 0x7c` |
| `bhs` at 0x665b4 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x665c0 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x665cc | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `bhs` at 0x6662c | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x66638 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x66644 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x66690 | `r0` | `?` |

## `L1CfgOptimize`

- `L1CfgOptimize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62e24 to 0x640dc.
  Established. A zero `[0x1327a4]` takes `0x62e74`, while nonzero falls through (`beq` at 0x62e40).
  Included absolute word stores clear `0x132ac4` and `0x132aa8` (`str` at 0x63488, `str` at 0x634e0).
  The unnamed `0x652dc` call carries `ret(0x6a0b0)`, `0x132a50`, `0x132a50`, `0x0` (`bl` at 0x634e4).
  Configuration optimization criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x62e34 | load | 4 | `abs 0x63de4` | `-` |
| `ldr` at 0x62e38 | load | 4 | `abs 0x1327a4` | `-` |
| `ldr` at 0x62e44 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x62e48 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x62e58 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x62e5c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x62e60 | load | 4 | `abs 0x63df0` | `-` |
| `ldr` at 0x62e64 | load | 4 | `abs 0x63df4` | `-` |
| `ldr` at 0x62e74 | load | 4 | `abs 0x63df8` | `-` |
| `ldr` at 0x62e78 | load | 4 | `abs 0x1327a0` | `-` |
| `ldr` at 0x62e90 | load | 4 | `abs 0x63df8` | `-` |
| `ldr` at 0x62e94 | load | 4 | `abs 0x1327a0` | `-` |
| `str` at 0x62e9c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x62ea0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x62eac | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x62eb0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x62ec0 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x62ec4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x62ec8 | load | 4 | `abs 0x63dfc` | `-` |
| `ldr` at 0x62ecc | load | 4 | `abs 0x63df4` | `-` |
| `ldr` at 0x62ed4 | load | 4 | `abs 0x63de4` | `-` |
| `ldr` at 0x62ed8 | load | 4 | `abs 0x63df8` | `-` |
| `ldr` at 0x62edc | load | 4 | `abs 0x1327a0` | `-` |
| `str` at 0x62ee4 | store | 4 | `abs 0x1327a4` | `[0x1327a0]+5` |
| `ldr` at 0x62efc | load | 4 | `abs 0x63df8` | `-` |
| `str` at 0x62f00 | store | 4 | `abs 0x1327a0` | `ret(0x194f0)` |
| `ldr` at 0x62f04 | load | 4 | `abs 0x63e00` | `-` |
| `ldr` at 0x62f08 | load | 4 | `abs 0x1329e0` | `-` |
| `ldr` at 0x62f14 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x62f18 | load | 4 | `abs 0x63e00` | `-` |
| `ldr` at 0x62f1c | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x62f20 | load | 4 | `abs 0x1329e0` | `-` |
| `ldr` at 0x62f30 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x62f34 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x62f40 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x62f44 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x62f4c | load | 4 | `abs 0x63e00` | `-` |
| `ldr` at 0x62f50 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x62f54 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x62f70 | store | 4 | `abs 0x1329e0` | `?` |
| `ldr` at 0x62f74 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x62f78 | load | 4 | `abs 0x63e00` | `-` |
| `ldr` at 0x62f7c | load | 4 | `abs 0x1329e0` | `-` |
| `str` at 0x62f9c | store | 4 | `abs 0x1329dc` | `ret(0x1943c)` |
| `ldr` at 0x62fa0 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x62fa4 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x62fb4 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x62fb8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x62fbc | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x62fc0 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x62fd4 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x62fd8 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x62fe4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x62fe8 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x62fec | store | 4 | `sp -0x7c` | `[0x132adc]` |
| `ldr` at 0x62ff0 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x62ff4 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63004 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63008 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6300c | load | 4 | `abs 0x63e08` | `-` |
| `str` at 0x6301c | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x63020 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63024 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x63028 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63034 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63038 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63054 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6305c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63068 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x6306c | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x6307c | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63080 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63084 | load | 4 | `abs 0x63e0c` | `-` |
| `ldr` at 0x63088 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63094 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63098 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x630a8 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x630ac | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x630b0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x630cc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x630d4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x630d8 | load | 4 | `abs 0x63e10` | `-` |
| `ldr` at 0x630dc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x630e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x630ec | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x630f0 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x630f4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63110 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63118 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6311c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x63128 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6312c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63148 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63150 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x63164 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6316c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63170 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63174 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63178 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6317c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63180 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63184 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x63188 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6318c | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x63198 | store | 4 | `sp -0x9c` | `ret(0x65068)` |
| `ldr` at 0x6319c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x631a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x631ac | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x631b4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x631b8 | load | 4 | `ret(0x65068) +0x14` | `-` |
| `ldr` at 0x631c4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x631c8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x631cc | store | 4 | `ret(0x65068) +0x10` | `[sp-128]` |
| `ldr` at 0x631d0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x631d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x631d8 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x631e0 | store | 4 | `[sp-156] +0x14` | `[[sp-152]+20]+1` |
| `ldr` at 0x631e4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x631e8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x631ec | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x631f0 | store | 4 | `[sp-156] +0xc` | `[[sp-152]+12]` |
| `ldr` at 0x631f4 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x631f8 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63208 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x6320c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63210 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63214 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63218 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x63220 | store | 4 | `sp -0xc0` | `[[sp-156]+16]+64` |
| `ldr` at 0x63224 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63228 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6322c | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x63230 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63234 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x63238 | store | 4 | `sp -0xb8` | `[[sp-156]+20]` |
| `ldr` at 0x6323c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63240 | load | 4 | `abs 0x63e14` | `-` |
| `ldr` at 0x63244 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x63248 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x63254 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63258 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6325c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63278 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63280 | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x63284 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63290 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63294 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63298 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x6329c | store | 4 | `sp -0xc0` | `[[sp-152]+4]` |
| `ldr` at 0x632a0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x632a4 | load | 4 | `[sp-152] +0x8` | `-` |
| `str` at 0x632a8 | store | 4 | `sp -0xbc` | `[[sp-152]+8]` |
| `ldr` at 0x632ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x632b0 | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x632b4 | store | 4 | `sp -0xb8` | `[[sp-152]+12]` |
| `ldr` at 0x632b8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x632bc | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x632c4 | store | 4 | `sp -0xb4` | `[[sp-152]+20]+1` |
| `ldr` at 0x632c8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x632cc | store | 4 | `sp -0xb0` | `[sp-128]` |
| `ldr` at 0x632d0 | load | 4 | `abs 0x63e18` | `-` |
| `ldr` at 0x632d4 | load | 4 | `abs 0x63e1c` | `-` |
| `ldr` at 0x632d8 | load | 4 | `abs 0x63e20` | `-` |
| `ldr` at 0x632dc | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x632e8 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x632ec | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x632fc | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63300 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63304 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63308 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x63310 | store | 4 | `sp -0xc0` | `[sp-128]+64` |
| `ldr` at 0x63314 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63318 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x63334 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6333c | load | 4 | `(unknown)` | `-` |
| `str` at 0x63344 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x63348 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6334c | load | 4 | `[sp-152] +0x10` | `-` |
| `str` at 0x63354 | store | 4 | `sp -0xb8` | `[[sp-152]+16]+64` |
| `ldr` at 0x63358 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6335c | load | 4 | `abs 0x63e24` | `-` |
| `ldr` at 0x63360 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x63364 | load | 4 | `[sp-152] +0x4` | `-` |
| `ldr` at 0x6336c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x63374 | store | 4 | `sp -0x84` | `[sp-132]+1` |
| `ldr` at 0x6337c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x63384 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `str` at 0x63390 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x63394 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63398 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x6339c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x633a8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x633ac | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x633c0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x633c8 | load | 4 | `abs 0x63e28` | `-` |
| `ldr` at 0x633cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x633d0 | load | 4 | `abs 0x12d784` | `-` |
| `ldr` at 0x633dc | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x633e0 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x633e4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x633e8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x633fc | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63404 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63408 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6340c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63410 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x63424 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6342c | load | 4 | `(unknown)` | `-` |
| `str` at 0x63430 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x63434 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63438 | load | 4 | `abs 0x63e1c` | `-` |
| `ldr` at 0x6343c | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63440 | load | 4 | `abs 0x63e2c` | `-` |
| `ldr` at 0x63448 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x63450 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x63458 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6345c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63460 | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x63464 | load | 4 | `abs 0x132a5c` | `-` |
| `str` at 0x63470 | store | 4 | `sp -0x90` | `ret(0x6a0b0)` |
| `ldr` at 0x63474 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63478 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x63480 | store | 4 | `ret(0x6a0b0) +0xc` | `0x0` |
| `str` at 0x63488 | store | 4 | `abs 0x132ac4` | `0x0` |
| `ldr` at 0x6348c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63490 | load | 4 | `abs 0x132aa8` | `-` |
| `ldr` at 0x6349c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x634a0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x634a8 | store | 4 | `[sp-144] +0xc` | `?` |
| `str` at 0x634b0 | store | 4 | `abs 0x132ac4` | `?` |
| `ldr` at 0x634b4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x634b8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x634bc | load | 4 | `abs 0x132a58` | `-` |
| `str` at 0x634c0 | store | 4 | `abs 0x132ac8` | `[0x132a58]` |
| `ldr` at 0x634c4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x634c8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x634cc | load | 4 | `abs 0x132a5c` | `-` |
| `str` at 0x634d0 | store | 4 | `abs 0x132acc` | `[0x132a5c]` |
| `ldr` at 0x634d8 | load | 4 | `abs 0x63e6c` | `-` |
| `str` at 0x634e0 | store | 4 | `abs 0x132aa8` | `0x0` |
| `str` at 0x634ec | store | 4 | `sp -0x94` | `ret(0x652dc)` |
| `ldr` at 0x634f0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x634fc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63500 | load | 4 | `ret(0x652dc) +0xc` | `-` |
| `ldr` at 0x6350c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63510 | load | 4 | `[sp-148] +0xc` | `-` |
| `ldr` at 0x6351c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63520 | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x6352c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63530 | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x63540 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63544 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63548 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6354c | store | 4 | `abs 0x132aa8` | `[[sp-148]+16]` |
| `ldr` at 0x63550 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x63554 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63558 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6355c | store | 4 | `[sp-144] +0xc` | `[[sp-148]+16]` |
| `ldr` at 0x63560 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63564 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63568 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6356c | store | 4 | `abs 0x132ac4` | `[[sp-148]+16]` |
| `ldr` at 0x63570 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63574 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63578 | load | 4 | `[sp-148] +0x0` | `-` |
| `str` at 0x6357c | store | 4 | `abs 0x132ac8` | `[[sp-148]]` |
| `ldr` at 0x63580 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63584 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63588 | load | 4 | `[sp-148] +0x4` | `-` |
| `str` at 0x6358c | store | 4 | `abs 0x132acc` | `[[sp-148]+4]` |
| `ldr` at 0x63590 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63594 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63598 | load | 4 | `[sp-148] +0x14` | `-` |
| `str` at 0x6359c | store | 4 | `abs 0x132ad0` | `[[sp-148]+20]` |
| `ldr` at 0x635a0 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x635a4 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x635b4 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x635b8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x635bc | load | 4 | `abs 0x63e30` | `-` |
| `str` at 0x635c8 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x635cc | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x635d0 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x635d4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x635e0 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x635e4 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x635e8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x635ec | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x63600 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63604 | load | 4 | `abs 0x1329dc` | `-` |
| `str` at 0x63624 | store | 4 | `sp -0xa4` | `0x1` |
| `ldr` at 0x63628 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6362c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x63630 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63634 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63644 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63648 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6364c | load | 4 | `abs 0x63e34` | `-` |
| `ldr` at 0x63650 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63658 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63664 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63668 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63678 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x6367c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63680 | load | 4 | `abs 0x63e38` | `-` |
| `str` at 0x6368c | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x63690 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63694 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x63698 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x636a4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x636ac | store | 4 | `sp -0x8c` | `0x1` |
| `ldr` at 0x636b4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x636bc | load | 4 | `abs 0x63e3c` | `-` |
| `ldr` at 0x636c0 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x636c4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x636d8 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x636dc | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x636ec | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x636f4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x636f8 | load | 4 | `abs 0x63e40` | `-` |
| `str` at 0x63704 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x63708 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6370c | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x63710 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6371c | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x63720 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63724 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63738 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63740 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63744 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63758 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63760 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x63764 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x63768 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6376c | load | 4 | `(unknown)` | `-` |
| `str` at 0x63778 | store | 4 | `sp -0x9c` | `ret(0x65068)` |
| `ldr` at 0x6377c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63788 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x63790 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x63798 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6379c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x637b0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x637b8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x637bc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x637d0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x637d8 | load | 4 | `abs 0x63e44` | `-` |
| `ldr` at 0x637dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x637e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x637f4 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x637f8 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63808 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63810 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63814 | load | 4 | `abs 0x63e40` | `-` |
| `ldr` at 0x6381c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63824 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x63830 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x63834 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63838 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x63844 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63848 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6384c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63860 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x63864 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x6386c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63880 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x63884 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x6388c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63890 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x63894 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63898 | load | 4 | `(unknown)` | `-` |
| `str` at 0x638a4 | store | 4 | `sp -0x9c` | `ret(0x65068)` |
| `ldr` at 0x638a8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x638b4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x638bc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x638c4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x638d8 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x638dc | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x638e4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x638f8 | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x638fc | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x63908 | load | 4 | `abs 0x63e48` | `-` |
| `ldr` at 0x6390c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63924 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63928 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63938 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63940 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63944 | load | 4 | `abs 0x63e40` | `-` |
| `ldr` at 0x6394c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63954 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x63960 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x63964 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63968 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x6396c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63978 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6397c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63990 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x63998 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x6399c | load | 4 | `abs 0x63e04` | `-` |
| `ldr` at 0x639a0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x639a4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x639a8 | load | 4 | `abs 0x1329dc` | `-` |
| `ldr` at 0x639ac | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x639b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x639b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x639c0 | store | 4 | `sp -0x98` | `ret(0x65068)` |
| `ldr` at 0x639c4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x639d0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x639d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x639d8 | load | 4 | `[sp-156] +0x14` | `-` |
| `ldr` at 0x639dc | load | 4 | `ret(0x65068) +0x14` | `-` |
| `ldr` at 0x639e8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x639f0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x639f8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x639fc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63a00 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63a04 | load | 4 | `ret(0x65068) +0x14` | `-` |
| `str` at 0x63a08 | store | 4 | `sp -0xc0` | `[ret(0x65068)+20]` |
| `ldr` at 0x63a0c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63a10 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x63a14 | store | 4 | `sp -0xbc` | `[[sp-156]+20]` |
| `ldr` at 0x63a1c | load | 4 | `abs 0x63e4c` | `-` |
| `ldr` at 0x63a20 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x63a24 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x63a38 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63a3c | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63a4c | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63a54 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63a58 | load | 4 | `abs 0x63e40` | `-` |
| `ldr` at 0x63a60 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63a6c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63a70 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63a74 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x63a78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63a84 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x63a8c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x63a94 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63a98 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63a9c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63aa0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63aa4 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x63aa8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63aac | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x63ab0 | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x63ab8 | load | 4 | `abs 0x63e50` | `-` |
| `ldr` at 0x63abc | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x63ac0 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x63ad4 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63ad8 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63ae8 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63af0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63af4 | load | 4 | `abs 0x63e40` | `-` |
| `ldr` at 0x63afc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63b08 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63b0c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63b10 | load | 4 | `[sp-156] +0xc` | `-` |
| `ldr` at 0x63b14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63b20 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x63b28 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x63b30 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63b34 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63b38 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x63b3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x63b40 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x63b44 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63b48 | load | 4 | `[sp-156] +0xc` | `-` |
| `str` at 0x63b4c | store | 4 | `sp -0xbc` | `[[sp-156]+12]` |
| `ldr` at 0x63b54 | load | 4 | `abs 0x63e54` | `-` |
| `ldr` at 0x63b58 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x63b5c | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x63b70 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63b74 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63b84 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63b8c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63b90 | load | 4 | `abs 0x63e40` | `-` |
| `ldr` at 0x63b98 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63ba0 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x63bb0 | load | 4 | `abs 0x63e58` | `-` |
| `ldr` at 0x63bb4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63bc8 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63bcc | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63bdc | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63be4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63be8 | load | 4 | `abs 0x63e40` | `-` |
| `ldr` at 0x63bf0 | load | 4 | `abs 0x63dec` | `-` |
| `ldr` at 0x63bf4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63bf8 | load | 4 | `abs 0x63e38` | `-` |
| `str` at 0x63c04 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x63c08 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63c0c | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x63c10 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x63c20 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x63c24 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63c28 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63c2c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63c30 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63c44 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63c4c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63c50 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63c64 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63c6c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x63c70 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x63c74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63c78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63c84 | store | 4 | `sp -0x9c` | `ret(0x65068)` |
| `ldr` at 0x63c88 | load | 4 | `abs 0x63de8` | `-` |
| `ldr` at 0x63c8c | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63c9c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63cac | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63cb4 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x63cb8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x63cc4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63cc8 | store | 4 | `sp -0xa8` | `[sp-160]` |
| `ldr` at 0x63ccc | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63cd0 | load | 4 | `abs 0x132aa8` | `-` |
| `ldr` at 0x63cdc | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63ce0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63ce4 | load | 4 | `ret(0x65068) +0x10` | `-` |
| `ldr` at 0x63d00 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63d08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63d14 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x63d1c | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x63d20 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x63d24 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x63d28 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63d2c | store | 4 | `sp -0xac` | `[sp-160]` |
| `ldr` at 0x63d30 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63d40 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63d44 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63d48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63d64 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63d6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63d78 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x63d80 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x63d84 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x63d88 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x63d8c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63d98 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63d9c | load | 4 | `abs 0x132aa8` | `-` |
| `ldr` at 0x63da8 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63dac | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63db0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63dcc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63dd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63e74 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63e84 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63e88 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x63e8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63ea8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x63eb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63ebc | load | 4 | `abs 0x63e60` | `-` |
| `ldr` at 0x63ec0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x63ed0 | load | 4 | `abs 0x63e64` | `-` |
| `ldr` at 0x63ed4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63ed8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63eec | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63ef4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63ef8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63f0c | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63f14 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x63f18 | store | 4 | `sp -0xc0` | `[sp-160]` |
| `ldr` at 0x63f1c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x63f20 | load | 4 | `abs 0x63e5c` | `-` |
| `ldr` at 0x63f24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63f30 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63f34 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63f48 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63f50 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63f54 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63f68 | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x63f74 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63f78 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x63f7c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x63fa0 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63fa4 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x63fa8 | load | 4 | `abs 0x132ae8` | `-` |
| `str` at 0x63fb0 | store | 4 | `abs 0x132ae8` | `[0x132ae8]-1` |
| `ldr` at 0x63fb4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x63fbc | store | 4 | `sp -0x88` | `[sp-136]-1` |
| `ldr` at 0x63fc0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x63fc8 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x63fd4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63fe0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63fec | load | 4 | `abs 0x63e60` | `-` |
| `ldr` at 0x63ff0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x64000 | load | 4 | `abs 0x63e64` | `-` |
| `ldr` at 0x64004 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x64008 | load | 4 | `abs 0x63e68` | `-` |
| `str` at 0x64014 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x64018 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x6401c | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x64020 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6402c | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x64030 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6404c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64054 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64064 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x64068 | load | 4 | `abs 0x132ac0` | `-` |
| `ldr` at 0x64074 | load | 4 | `abs 0x63e6c` | `-` |
| `ldr` at 0x64078 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x64094 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6409c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x640a8 | load | 4 | `abs 0x63e70` | `-` |
| `ldr` at 0x640ac | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x640b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x640bc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x640c4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x640cc | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldm` at 0x640d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x640d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x640d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x640d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62e68 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbf94` | `0x29a500` | `0x12eeb8` |
| `bl` at 0x62e6c | `(unknown callee) 0x61a78` | `?` | `?` | `?` | `?` |
| `bl` at 0x62e88 | `(unknown callee) 0x194f0` | `0x0` | `r1` | `r2` | `[0x1327a0]` |
| `bl` at 0x62ed0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbfb4` | `0x29a500` | `0x12eeb8` |
| `bl` at 0x62ee8 | `(unknown callee) 0x61a78` | `?` | `?` | `0x1327a4` | `[0x1327a0]+5` |
| `bl` at 0x62ef4 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x62f48 | `(unknown callee) 0x1970c` | `[0x1329dc]` | `?` | `?` | `0x1329dc` |
| `bl` at 0x62f94 | `(unknown callee) 0x1943c` | `?` | `0x1329e0` | `[0x1329e0]` | `?` |
| `bl` at 0x62fb0 | `(unknown callee) 0x19748` | `[0x1329dc]` | `0x0` | `0x4` | `0x1329dc` |
| `bl` at 0x62fe0 | `(unknown callee) 0x19610` | `[0x1329dc]` | `[0x132ae4]` | `?` | `?` |
| `bl` at 0x63010 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbfd0` | `?` | `0x12eeb8` |
| `bl` at 0x63014 | `(unknown callee) 0x60b1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x6308c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbfe4` | `[sp-128]` | `0x12eeb8` |
| `bl` at 0x630e4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc004` | `[sp-128]` | `?` |
| `bl` at 0x63190 | `(unknown callee) 0x65068` | `[0x132ae4]` | `[0x132adc]` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x6324c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc024` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x632e0 | `(unknown callee) 0x66694` | `0x132ae4` | `0x132adc` | `0x132ae0` | `[[sp-152]]` |
| `bl` at 0x63368 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc054` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x63444 | `(unknown callee) 0x66d5c` | `[0x132ae4]` | `0x132adc` | `[0x132af0]` | `0x132ae8` |
| `bl` at 0x63468 | `(unknown callee) 0x6a0b0` | `[0x132a58]` | `[0x132a5c]` | `0x132a50` | `0x132a50` |
| `bl` at 0x634e4 | `(unknown callee) 0x652dc` | `ret(0x6a0b0)` | `0x132a50` | `0x132a50` | `0x0` |
| `bl` at 0x635c0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc07c` | `?` | `0x12eeb8` |
| `bl` at 0x6360c | `(unknown callee) 0x19580` | `[0x132ae4]` | `[0x1329dc]` | `?` | `?` |
| `bl` at 0x63654 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc090` | `?` | `0x12eeb8` |
| `bl` at 0x63684 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0a8` | `?` | `0x12eeb8` |
| `bl` at 0x636c8 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc0cc` | `[0x132adc]` | `[sp-124]` |
| `bl` at 0x636d4 | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x636fc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x63770 | `(unknown callee) 0x65068` | `[0x1329dc]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x637e4 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc0f8` | `?` | `?` |
| `bl` at 0x637f0 | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x63818 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x6389c | `(unknown callee) 0x65068` | `[0x132ae4]` | `[0x132adc]` | `?` | `?` |
| `bl` at 0x63914 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc108` | `?` | `?` |
| `bl` at 0x63920 | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x63948 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x639b8 | `(unknown callee) 0x65068` | `[0x1329dc]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x63a28 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc11c` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x63a34 | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x63a5c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x63ac4 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc134` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x63ad0 | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x63af8 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x63b60 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc150` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x63b6c | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x63b94 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x63bb8 | `(unknown callee) 0x197a8` | `sp-120` | `0xfc16c` | `?` | `sp-120` |
| `bl` at 0x63bc4 | `(unknown callee) 0x62b50` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x63bec | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0f0` | `sp-120` | `0x12eeb8` |
| `bl` at 0x63bfc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc0a8` | `?` | `0x12eeb8` |
| `bl` at 0x63c7c | `(unknown callee) 0x65068` | `[0x132ae4]` | `[0x132adc]` | `?` | `?` |
| `bl` at 0x63f2c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc17c` | `?` | `?` |
| `bl` at 0x63f9c | `(unknown callee) 0x19610` | `?` | `?` | `?` | `?` |
| `bl` at 0x63fd0 | `(unknown callee) 0x61a78` | `?` | `?` | `[0x132ae8]` | `?` |
| `bl` at 0x6400c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc19c` | `?` | `0x12eeb8` |
| `bl` at 0x640b4 | `(unknown callee) 0x676c0` | `?` | `0x132a50` | `0x132bf4` | `[sp-128]` |
| `bl` at 0x640c0 | `(unknown callee) 0x62048` | `[sp-128]` | `0x132a50` | `?` | `[0x132ac0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x62e40 | `[0x1327a4] == 0x0` | `[0x1327a4] != 0x0` |
| `beq` at 0x62e54 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62e80 | `[0x1327a0] == 0x0` | `[0x1327a0] != 0x0` |
| `bgt` at 0x62ea8 | `[sp-152] > 0x4 (signed, 32-bit)` | `[sp-152] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x62ebc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62f10 | `[0x1329e0] == 0x0` | `[0x1329e0] != 0x0` |
| `bgt` at 0x62f28 | `[0x132adc] > [0x1329e0] (signed, 32-bit)` | `[0x132adc] <= [0x1329e0] (signed, 32-bit)` |
| `beq` at 0x62f3c | `[0x1329dc] == 0x0` | `[0x1329dc] != 0x0` |
| `beq` at 0x63000 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x63030 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x63064 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x63078 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x630a4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x63124 | `? <= [sp-132] (signed, 32-bit)` | `? > [sp-132] (signed, 32-bit)` |
| `beq` at 0x631a4 | `ret(0x65068) == 0x0` | `ret(0x65068) != 0x0` |
| `bhs` at 0x631c0 | `[[sp-152]+20]+1 >= [ret(0x65068)+20] (unsigned, 32-bit)` | `[[sp-152]+20]+1 < [ret(0x65068)+20] (unsigned, 32-bit)` |
| `beq` at 0x63204 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6328c | `[[sp-152]+16] == ?` | `[[sp-152]+16] != ?` |
| `beq` at 0x632f8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x633a4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0x633d8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x63498 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x634f8 | `ret(0x652dc) == 0x0` | `ret(0x652dc) != 0x0` |
| `beq` at 0x63508 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63518 | `[[sp-148]+12] == 0x0` | `[[sp-148]+12] != 0x0` |
| `beq` at 0x63528 | `[[sp-148]+8] == 0x52` | `[[sp-148]+8] != 0x52` |
| `beq` at 0x63538 | `[[sp-148]+8] == 0x44` | `[[sp-148]+8] != 0x44` |
| `beq` at 0x635b0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x635dc | `[0x132adc] != [sp-124]` | `[0x132adc] == [sp-124]` |
| `bne` at 0x63618 | `ret(0x19580) != 0x0` | `ret(0x19580) == 0x0` |
| `beq` at 0x63640 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63660 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63674 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x636a0 | `[0x132adc] == [sp-124]` | `[0x132adc] != [sp-124]` |
| `beq` at 0x636e8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x63718 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x63784 | `ret(0x65068) != 0x0` | `ret(0x65068) == 0x0` |
| `beq` at 0x63804 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x63840 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x638b0 | `ret(0x65068) != 0x0` | `ret(0x65068) == 0x0` |
| `beq` at 0x63934 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x63974 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x639cc | `ret(0x65068) == 0x0` | `ret(0x65068) != 0x0` |
| `beq` at 0x639e4 | `[[sp-156]+20] == [ret(0x65068)+20]` | `[[sp-156]+20] != [ret(0x65068)+20]` |
| `beq` at 0x63a48 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63a68 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63a80 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63ae4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63b04 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63b1c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x63b80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63bd8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x63c18 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x63c98 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x63cc0 | `[sp-160] != 0x0` | `[sp-160] == 0x0` |
| `bne` at 0x63cd8 | `[0x132aa8] != 0x0` | `[0x132aa8] == 0x0` |
| `bne` at 0x63d10 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x63d3c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x63d74 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x63d94 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x63da4 | `[0x132aa8] != 0x0` | `[0x132aa8] == 0x0` |
| `bne` at 0x63ddc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x63e80 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x63eb8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x63ecc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63fdc | `[sp-140] == 0x0` | `[sp-140] != 0x0` |
| `beq` at 0x63fe8 | `[sp-140] == 0x2` | `[sp-140] != 0x2` |
| `beq` at 0x63ffc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x64028 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x6405c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x64070 | `[0x132ac0] == 0x0` | `[0x132ac0] != 0x0` |
| `beq` at 0x640a4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x640d8 | `r0` | `?` |

## `L1CfgInit`

- `L1CfgInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60370 to 0x60b1c.
  Established. The absolute word at `0x132774` receives `[0x132774]+1` (`str` at 0x603cc).
  An included word store writes `0x10` to `0x132ae0` (`str` at 0x604a8).
  The unnamed `0xa75f8` call carries `0x132a48`, `0xffff00e`, `0x3`, `0x681e8` (`bl` at 0x60a24).
  Configuration initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6038c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x60390 | load | 4 | `abs 0x60ab4` | `-` |
| `ldr` at 0x60394 | load | 4 | `abs 0x132774` | `-` |
| `ldr` at 0x603a0 | load | 4 | `abs 0x60ab8` | `-` |
| `str` at 0x603a4 | store | 4 | `sp -0x44` | `0xfbc94` |
| `ldr` at 0x603ac | load | 4 | `abs 0x60abc` | `-` |
| `ldr` at 0x603bc | load | 4 | `abs 0x60ab4` | `-` |
| `ldr` at 0x603c0 | load | 4 | `abs 0x60ab4` | `-` |
| `ldr` at 0x603c4 | load | 4 | `abs 0x132774` | `-` |
| `str` at 0x603cc | store | 4 | `abs 0x132774` | `[0x132774]+1` |
| `ldr` at 0x603d0 | load | 4 | `abs 0x60ac0` | `-` |
| `str` at 0x603dc | store | 4 | `abs 0x13278c` | `ret(0x194f0)` |
| `str` at 0x603f8 | store | 4 | `sp -0x20` | `ret(0x19538)` |
| `ldr` at 0x603fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60428 | load | 4 | `abs 0x60ac4` | `-` |
| `ldr` at 0x60434 | load | 4 | `abs 0x60ac8` | `-` |
| `str` at 0x6043c | store | 4 | `abs 0x132778` | `0x100` |
| `ldr` at 0x60440 | load | 4 | `abs 0x60ac8` | `-` |
| `str` at 0x60448 | store | 4 | `abs 0x132778` | `0x1` |
| `ldr` at 0x6044c | load | 4 | `abs 0x60ac8` | `-` |
| `str` at 0x60454 | store | 4 | `abs 0x132778` | `0x10` |
| `ldr` at 0x60458 | load | 4 | `abs 0x60ac8` | `-` |
| `ldr` at 0x6045c | load | 4 | `abs 0x60acc` | `-` |
| `str` at 0x60460 | store | 4 | `abs 0x132778` | `0x3ff` |
| `ldr` at 0x60464 | load | 4 | `abs 0x60ac8` | `-` |
| `str` at 0x6046c | store | 4 | `abs 0x132778` | `0x0` |
| `ldr` at 0x60470 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x60474 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x60478 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x60484 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x60494 | load | 4 | `abs 0x60ad0` | `-` |
| `str` at 0x6049c | store | 4 | `abs 0x12d784` | `0xc` |
| `ldr` at 0x604a0 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x604a8 | store | 4 | `abs 0x132ae0` | `0x10` |
| `ldr` at 0x604ac | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x604b4 | store | 4 | `abs 0x132adc` | `0x0` |
| `ldr` at 0x604b8 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x604bc | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x604c0 | load | 4 | `abs 0x132ae0` | `-` |
| `str` at 0x604e0 | store | 4 | `abs 0x132ae4` | `ret(0x1943c)` |
| `ldr` at 0x604e4 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x604e8 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x604ec | load | 4 | `abs 0x132ae0` | `-` |
| `ldr` at 0x60500 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x60510 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60518 | store | 4 | `abs 0x132ad4` | `0xd` |
| `ldr` at 0x6051c | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60520 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60524 | load | 4 | `abs 0x132ad4` | `-` |
| `str` at 0x6054c | store | 4 | `abs 0x132ad8` | `ret(0x1943c)` |
| `ldr` at 0x60550 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60554 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60558 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x60574 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x60584 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x6058c | store | 4 | `abs 0x132a50` | `0x1` |
| `ldr` at 0x60590 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60598 | store | 4 | `abs 0x132ab0` | `0x2` |
| `ldr` at 0x6059c | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x605a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x605a4 | store | 4 | `abs 0x132a60` | `[sp-36]` |
| `ldr` at 0x605a8 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x605b4 | store | 4 | `abs 0x132a58` | `ret(0x1d444)` |
| `ldr` at 0x605b8 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x605c4 | store | 4 | `abs 0x132a5c` | `ret(0x1d444)` |
| `ldr` at 0x605c8 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x605d4 | store | 4 | `abs 0x132a78` | `ret(0x1d444)` |
| `ldr` at 0x605d8 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x605dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x605e4 | store | 4 | `abs 0x132ac0` | `ret(0x6a390)` |
| `ldr` at 0x605e8 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x605f4 | store | 4 | `abs 0x132a8c` | `ret(0x1d444)` |
| `ldr` at 0x605f8 | load | 4 | `abs 0x132a8c` | `-` |
| `ldr` at 0x60604 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x6060c | store | 4 | `abs 0x132a8c` | `0x0` |
| `ldr` at 0x60628 | load | 4 | `abs 0x60ad8` | `-` |
| `ldr` at 0x60634 | load | 4 | `abs 0x60adc` | `-` |
| `ldr` at 0x60638 | load | 4 | `abs 0x60ac4` | `-` |
| `ldr` at 0x60640 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x6064c | store | 4 | `abs 0x132a64` | `ret(0x554b8)` |
| `ldr` at 0x60650 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60658 | store | 4 | `abs 0x132a68` | `0x0` |
| `ldr` at 0x6065c | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60664 | store | 4 | `abs 0x132a7c` | `0x0` |
| `ldr` at 0x60668 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60670 | store | 4 | `abs 0x132a84` | `ret(0x375e4)` |
| `ldr` at 0x60674 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x6067c | store | 4 | `abs 0x132a88` | `ret(0x375c4)` |
| `ldr` at 0x60680 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x60688 | store | 4 | `abs 0x132afc` | `0x1` |
| `ldr` at 0x6068c | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x60694 | store | 4 | `abs 0x132b28` | `0x2` |
| `ldr` at 0x60698 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606a0 | store | 4 | `abs 0x132b44` | `0x3` |
| `ldr` at 0x606a4 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606ac | store | 4 | `abs 0x132b48` | `0x4` |
| `ldr` at 0x606b0 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606b8 | store | 4 | `abs 0x132b4c` | `0x5` |
| `ldr` at 0x606bc | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606c4 | store | 4 | `abs 0x132b50` | `0x6` |
| `ldr` at 0x606c8 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606d0 | store | 4 | `abs 0x132b54` | `0x7` |
| `ldr` at 0x606d4 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606dc | store | 4 | `abs 0x132b58` | `0x8` |
| `ldr` at 0x606e0 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606e8 | store | 4 | `abs 0x132af8` | `0x9` |
| `ldr` at 0x606ec | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x606f4 | store | 4 | `abs 0x132b00` | `0xa` |
| `ldr` at 0x606f8 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x60700 | store | 4 | `abs 0x132b2c` | `0xb` |
| `ldr` at 0x60704 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x6070c | store | 4 | `abs 0x132b30` | `0xc` |
| `ldr` at 0x60710 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x60718 | store | 4 | `abs 0x132b04` | `0x9` |
| `ldr` at 0x6071c | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x60724 | store | 4 | `abs 0x132b08` | `0xa` |
| `ldr` at 0x60728 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x60730 | store | 4 | `abs 0x132b24` | `0xb` |
| `ldr` at 0x60734 | load | 4 | `abs 0x60ae0` | `-` |
| `str` at 0x6073c | store | 4 | `abs 0x132b18` | `0xc` |
| `ldr` at 0x60740 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x60748 | store | 4 | `abs 0x132bf8` | `0x2` |
| `ldr` at 0x6074c | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x60754 | store | 4 | `abs 0x132bfc` | `0xd` |
| `ldr` at 0x60758 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x60760 | store | 4 | `abs 0x132c00` | `0x14` |
| `ldr` at 0x60764 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x6076c | store | 4 | `abs 0x132c04` | `0x15` |
| `ldr` at 0x60770 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x60778 | store | 4 | `abs 0x132c08` | `0x16` |
| `ldr` at 0x6077c | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x60784 | store | 4 | `abs 0x132c0c` | `0x17` |
| `ldr` at 0x60788 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x60790 | store | 4 | `abs 0x132c10` | `0x18` |
| `ldr` at 0x60794 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x6079c | store | 4 | `abs 0x132c14` | `0x19` |
| `ldr` at 0x607a0 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x607a8 | store | 4 | `abs 0x132c18` | `0x1` |
| `ldr` at 0x607ac | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x607b4 | store | 4 | `abs 0x132c1c` | `0x3` |
| `ldr` at 0x607b8 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x607c0 | store | 4 | `abs 0x132c20` | `0xe` |
| `ldr` at 0x607c4 | load | 4 | `abs 0x60ae4` | `-` |
| `str` at 0x607cc | store | 4 | `abs 0x132c24` | `0xf` |
| `ldr` at 0x607d0 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x607d4 | load | 4 | `abs 0x132a60` | `-` |
| `str` at 0x607dc | store | 4 | `sp -0x30` | `[0x132a60]-54` |
| `ldr` at 0x607e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x607f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x607f8 | load | 4 | `abs 0x60ae8` | `-` |
| `ldr` at 0x6080c | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60810 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x6081c | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60820 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60830 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60838 | store | 4 | `abs 0x132a8c` | `0x0` |
| `ldr` at 0x60848 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x60854 | store | 4 | `abs 0x132a7c` | `ret(0x1d444)` |
| `ldr` at 0x60858 | load | 4 | `abs 0x132a7c` | `-` |
| `str` at 0x6085c | store | 4 | `sp -0x2c` | `[0x132a7c]` |
| `ldr` at 0x60860 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x60870 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x60884 | load | 4 | `abs 0x60ad4` | `-` |
| `str` at 0x6088c | store | 4 | `abs 0x132a7c` | `0xf` |
| `ldr` at 0x608a0 | load | 4 | `abs 0x60aec` | `-` |
| `ldr` at 0x608a4 | load | 4 | `abs 0x60af0` | `-` |
| `ldr` at 0x608b4 | load | 4 | `abs 0x60af4` | `-` |
| `ldr` at 0x608c4 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x608c8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x608d4 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x608d8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x608e4 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x608e8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x608f4 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x608f8 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60904 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60908 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60914 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60918 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60924 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60928 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60934 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60938 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60944 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60948 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60954 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60958 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60964 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60968 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60974 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60978 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x60984 | load | 4 | `abs 0x60ad4` | `-` |
| `ldr` at 0x60988 | load | 4 | `abs 0x132a60` | `-` |
| `str` at 0x6099c | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x609a4 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x609ac | store | 4 | `sp -0x3c` | `0x14` |
| `str` at 0x609b4 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x609bc | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x609c0 | load | 4 | `abs 0x60af8` | `-` |
| `ldr` at 0x609c4 | load | 4 | `abs 0x60afc` | `-` |
| `ldr` at 0x609c8 | load | 4 | `abs 0x60b00` | `-` |
| `str` at 0x609dc | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x609e4 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x609ec | store | 4 | `sp -0x3c` | `0x14` |
| `str` at 0x609f4 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x609fc | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x60a00 | load | 4 | `abs 0x60af8` | `-` |
| `ldr` at 0x60a04 | load | 4 | `abs 0x60afc` | `-` |
| `ldr` at 0x60a08 | load | 4 | `abs 0x60b04` | `-` |
| `ldr` at 0x60a14 | load | 4 | `abs 0x60af4` | `-` |
| `ldr` at 0x60a18 | load | 4 | `abs 0x60b08` | `-` |
| `ldr` at 0x60a20 | load | 4 | `abs 0x60b0c` | `-` |
| `str` at 0x60a2c | store | 4 | `sp -0x1c` | `ret(0xa75f8)` |
| `ldr` at 0x60a30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60a3c | load | 4 | `abs 0x60b10` | `-` |
| `str` at 0x60a40 | store | 4 | `sp -0x44` | `0xfbce0` |
| `ldr` at 0x60a44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60a48 | store | 4 | `sp -0x40` | `ret(0xa75f8)` |
| `ldr` at 0x60a50 | load | 4 | `abs 0x60abc` | `-` |
| `ldr` at 0x60a60 | load | 4 | `abs 0x60af4` | `-` |
| `ldr` at 0x60a6c | load | 4 | `abs 0x60b0c` | `-` |
| `str` at 0x60a78 | store | 4 | `sp -0x1c` | `ret(0xa7a8c)` |
| `ldr` at 0x60a7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60a88 | load | 4 | `abs 0x60b14` | `-` |
| `str` at 0x60a8c | store | 4 | `sp -0x44` | `0xfbd0c` |
| `ldr` at 0x60a90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60a94 | store | 4 | `sp -0x40` | `ret(0xa7a8c)` |
| `ldr` at 0x60a9c | load | 4 | `abs 0x60abc` | `-` |
| `ldr` at 0x60aa0 | load | 4 | `abs 0x60b18` | `-` |
| `ldm` at 0x60ab0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x60ab0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x60ab0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x60ab0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60380 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x603b8 | `(unknown callee) 0x200a0` | `0x9` | `0xfbcb0` | `0x6d` | `0x0` |
| `bl` at 0x603d8 | `(unknown callee) 0x194f0` | `0x0` | `?` | `0x132774` | `[0x132774]+1` |
| `bl` at 0x603e4 | `(unknown callee) 0x1d444` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x603f0 | `(unknown callee) 0x19538` | `ret(0x1d444)` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x60408 | `(unknown callee) 0x48b30` | `ret(0x19538)` | `?` | `?` | `ret(0x19538)` |
| `bl` at 0x60418 | `(unknown callee) 0x1b5f0` | `0xf` | `ret(0x48b30)` | `?` | `ret(0x48b30)` |
| `bl` at 0x60420 | `(unknown callee) 0x1d444` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x60430 | `(unknown callee) 0x19808` | `0x29a978` | `ret(0x1d444)` | `?` | `ret(0x1d444)` |
| `bl` at 0x604d8 | `(unknown callee) 0x1943c` | `?` | `[sp-36]` | `[0x132ae0]` | `?` |
| `bl` at 0x6050c | `(unknown callee) 0x19748` | `[0x132ae4]` | `0x0` | `?` | `?` |
| `bl` at 0x60544 | `(unknown callee) 0x1943c` | `?` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x60580 | `(unknown callee) 0x19748` | `[0x132ad8]` | `0x0` | `?` | `?` |
| `bl` at 0x605b0 | `(unknown callee) 0x1d444` | `0x1` | `?` | `0x132a50` | `[sp-36]` |
| `bl` at 0x605c0 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x605d0 | `(unknown callee) 0x1d444` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x605e0 | `(unknown callee) 0x6a390` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x605f0 | `(unknown callee) 0x1d444` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x60618 | `(unknown callee) 0x1b5f0` | `0xe` | `0x0` | `0x132a50` | `0x0` |
| `bl` at 0x60620 | `(unknown callee) 0x1d444` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x60630 | `(unknown callee) 0x19808` | `0x132a90` | `ret(0x1d444)` | `?` | `ret(0x1d444)` |
| `bl` at 0x6063c | `(unknown callee) 0x19808` | `0x132a9c` | `0x29a978` | `?` | `?` |
| `bl` at 0x60648 | `(unknown callee) 0x554b8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6066c | `(unknown callee) 0x375e4` | `ret(0x554b8)` | `?` | `0x132a50` | `0x0` |
| `bl` at 0x60678 | `(unknown callee) 0x375c4` | `ret(0x375e4)` | `?` | `?` | `?` |
| `bl` at 0x60844 | `(unknown callee) 0x1b5f0` | `0xe` | `0x0` | `0x132a50` | `0x0` |
| `bl` at 0x60850 | `(unknown callee) 0x1d444` | `0x3c` | `?` | `?` | `?` |
| `bl` at 0x60898 | `(unknown callee) 0x1b5f0` | `0x3c` | `0xf` | `0x132a50` | `0xf` |
| `bl` at 0x6089c | `(unknown callee) 0x60b1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x608b0 | `(unknown callee) 0x20bb4` | `0x1329f0` | `0xfbcc8` | `0x1` | `0x9` |
| `bl` at 0x608c0 | `(unknown callee) 0xa89d0` | `0x132a48` | `0xe` | `0x0` | `?` |
| `bl` at 0x609d0 | `(unknown callee) 0x2066c` | `0x132a44` | `0xfbcd4` | `0x60e78` | `0x0` |
| `bl` at 0x60a10 | `(unknown callee) 0x2066c` | `0x132a44` | `0xfbcd4` | `0x60c60` | `0x0` |
| `bl` at 0x60a24 | `(unknown callee) 0xa75f8` | `0x132a48` | `0xffff00e` | `0x3` | `0x681e8` |
| `bl` at 0x60a5c | `(unknown callee) 0x200a0` | `0x9` | `0xfbcb0` | `0x128` | `0x0` |
| `bl` at 0x60a70 | `(unknown callee) 0xa7a8c` | `0x132a48` | `?` | `0x0` | `0x681e8` |
| `bl` at 0x60aa8 | `(unknown callee) 0x200a0` | `0x9` | `0xfbcb0` | `0x12d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6039c | `[0x132774] == 0x0` | `[0x132774] != 0x0` |
| `beq` at 0x60404 | `ret(0x19538) == 0x6` | `ret(0x19538) != 0x6` |
| `beq` at 0x60480 | `[sp-36] == 0x42` | `[sp-36] != 0x42` |
| `beq` at 0x6048c | `[sp-36] == 0x46` | `[sp-36] != 0x46` |
| `ble` at 0x60600 | `[0x132a8c] <= 0xfe (signed, 32-bit)` | `[0x132a8c] > 0xfe (signed, 32-bit)` |
| `bhi` at 0x607e8 | `[0x132a60]-54 > 0x1c (unsigned, 32-bit)` | `[0x132a60]-54 <= 0x1c (unsigned, 32-bit)` |
| `bne` at 0x60804 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x60818 | `[0x132a60] == 0x52` | `[0x132a60] != 0x52` |
| `beq` at 0x60828 | `[0x132a60] == 0x44` | `[0x132a60] != 0x44` |
| `bhi` at 0x60868 | `[0x132a7c] > 0xf (unsigned, 32-bit)` | `[0x132a7c] <= 0xf (unsigned, 32-bit)` |
| `bne` at 0x60880 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x608d0 | `[0x132a60] == 0x35` | `[0x132a60] != 0x35` |
| `beq` at 0x608e0 | `[0x132a60] == 0x4f` | `[0x132a60] != 0x4f` |
| `beq` at 0x608f0 | `[0x132a60] == 0x42` | `[0x132a60] != 0x42` |
| `beq` at 0x60900 | `[0x132a60] == 0x46` | `[0x132a60] != 0x46` |
| `beq` at 0x60910 | `[0x132a60] == 0x4a` | `[0x132a60] != 0x4a` |
| `beq` at 0x60920 | `[0x132a60] == 0x4b` | `[0x132a60] != 0x4b` |
| `beq` at 0x60930 | `[0x132a60] == 0x21` | `[0x132a60] != 0x21` |
| `beq` at 0x60940 | `[0x132a60] == 0x5e` | `[0x132a60] != 0x5e` |
| `beq` at 0x60950 | `[0x132a60] == 0x2b` | `[0x132a60] != 0x2b` |
| `beq` at 0x60960 | `[0x132a60] == 0x4c` | `[0x132a60] != 0x4c` |
| `beq` at 0x60970 | `[0x132a60] == 0x5a` | `[0x132a60] != 0x5a` |
| `beq` at 0x60980 | `[0x132a60] == 0x7c` | `[0x132a60] != 0x7c` |
| `beq` at 0x60990 | `[0x132a60] == 0x4d` | `[0x132a60] != 0x4d` |
| `beq` at 0x60a38 | `ret(0xa75f8) == 0x0` | `ret(0xa75f8) != 0x0` |
| `beq` at 0x60a84 | `ret(0xa7a8c) == 0x0` | `ret(0xa7a8c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x60ab0 | `r0` | `?` |

## `L1CfgQuery`

- `L1CfgQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x682a8 to 0x69e60.
  Established. A zero entry `r1` takes `0x682e0`, while nonzero falls through (`beq` at 0x682c8).
  An included word store writes `ret(0x1943c)` through `[sp-24] +0x0` (`str` at 0x68314).
  The computed transfer has an unresolved taken edge for unsigned `[[sp-60]+56] <= 0x5`, while greater values fall through (`ldrls` at 0x68790).
  Query flag and output meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x682b8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x682bc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x682c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x682cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x682e0 | load | 4 | `abs 0x691e0` | `-` |
| `ldr` at 0x682e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x682ec | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x682f0 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x68314 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x68318 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x6831c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x68324 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x68328 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x6832c | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x68330 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6833c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68340 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68354 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6835c | load | 4 | `(unknown)` | `-` |
| `str` at 0x68360 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x68364 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68368 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6837c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68384 | load | 4 | `(unknown)` | `-` |
| `str` at 0x68388 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x6838c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68390 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x683a4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x683ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x683b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x683cc | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x683d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x683e4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x683ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68404 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68408 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6841c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68424 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68430 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68434 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68448 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68454 | load | 4 | `abs 0x691e4` | `-` |
| `ldr` at 0x68458 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68464 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68468 | load | 4 | `abs 0x691e8` | `-` |
| `ldr` at 0x68470 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68480 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68488 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x6848c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68490 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x684a4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x684ac | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x684b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x684c4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x684cc | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x684d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x684e4 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x684ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x684f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x684f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x684f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68500 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68510 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68518 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x6851c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68520 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68534 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6853c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68548 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6854c | load | 4 | `abs 0x691ec` | `-` |
| `ldr` at 0x68558 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x6855c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68574 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6857c | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x68580 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6858c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68590 | load | 4 | `abs 0x691f0` | `-` |
| `ldr` at 0x6859c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x685ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x685b4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x685b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x685c8 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x685cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x685e0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x685e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x685ec | load | 4 | `abs 0x691f4` | `-` |
| `ldr` at 0x685f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x685f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68608 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68610 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68614 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68624 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68628 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6863c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68644 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68648 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6865c | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68664 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68668 | load | 4 | `(unknown)` | `-` |
| `str` at 0x68674 | store | 4 | `sp -0x3c` | `ret(0x69eec)` |
| `ldr` at 0x68678 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x6867c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68690 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68698 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x686a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x686a8 | load | 4 | `abs 0x691f8` | `-` |
| `ldr` at 0x686b4 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x686b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x686cc | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x686d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x686e0 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x686e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x686f8 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68704 | load | 4 | `abs 0x691fc` | `-` |
| `ldr` at 0x68708 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68714 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68718 | load | 4 | `abs 0x69200` | `-` |
| `ldr` at 0x68720 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68730 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68738 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x6873c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x68748 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6874c | load | 4 | `abs 0x69204` | `-` |
| `ldr` at 0x68758 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6875c | load | 4 | `[sp-60] +0x34` | `-` |
| `ldr` at 0x68768 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6876c | load | 4 | `abs 0x69208` | `-` |
| `ldr` at 0x68778 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6877c | load | 4 | `abs 0x6920c` | `-` |
| `ldr` at 0x68784 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x68788 | load | 4 | `[sp-60] +0x38` | `-` |
| `ldrls` at 0x68790 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68810 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68814 | load | 4 | `abs 0x69228` | `-` |
| `ldr` at 0x6881c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x68820 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x68830 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x68834 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x68844 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68848 | load | 4 | `abs 0x6922c` | `-` |
| `ldr` at 0x68854 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x68858 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x68868 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6886c | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x6887c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68880 | load | 4 | `abs 0x69230` | `-` |
| `ldr` at 0x6888c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x68890 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x688a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x688a4 | load | 4 | `abs 0x69234` | `-` |
| `ldr` at 0x688b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x688b4 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x688c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x688c8 | load | 4 | `abs 0x69238` | `-` |
| `ldr` at 0x688d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x688d8 | load | 4 | `abs 0x6923c` | `-` |
| `ldr` at 0x688e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x688e4 | load | 4 | `abs 0x692c4` | `-` |
| `ldr` at 0x688ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x688fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68904 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68908 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x68910 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x68918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68928 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x6892c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68930 | load | 4 | `abs 0x69240` | `-` |
| `ldr` at 0x68934 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x6893c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6894c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68954 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68958 | load | 4 | `abs 0x69244` | `-` |
| `ldr` at 0x6895c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68960 | load | 4 | `abs 0x69248` | `-` |
| `ldr` at 0x68964 | load | 4 | `abs 0x132780` | `-` |
| `ldr` at 0x6896c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6897c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68984 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68988 | load | 4 | `abs 0x6924c` | `-` |
| `ldr` at 0x6898c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68990 | load | 4 | `abs 0x69250` | `-` |
| `ldr` at 0x68994 | load | 4 | `abs 0x132784` | `-` |
| `ldr` at 0x6899c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x689ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x689b4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x689b8 | load | 4 | `abs 0x69254` | `-` |
| `ldr` at 0x689bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x689c0 | load | 4 | `abs 0x69258` | `-` |
| `ldr` at 0x689c4 | load | 4 | `abs 0x132788` | `-` |
| `ldr` at 0x689cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x689dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x689e4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x689f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68a00 | load | 4 | `abs 0x6925c` | `-` |
| `ldr` at 0x68a08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68a0c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68a10 | load | 4 | `abs 0x132ad4` | `-` |
| `str` at 0x68a3c | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x68a40 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x68a44 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x68a48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68a4c | load | 4 | `abs 0x69260` | `-` |
| `ldr` at 0x68a54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68a64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68a6c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68a70 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68a74 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68a78 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68a7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68a80 | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x68a84 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x68a88 | load | 4 | `abs 0x132a5c` | `-` |
| `ldr` at 0x68a90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68a94 | load | 4 | `abs 0x692c4` | `-` |
| `ldr` at 0x68a9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68aac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68ab4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x68abc | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x68ac0 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68ac4 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x68ac8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68ad4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68aec | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68af0 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x68af4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68b04 | load | 4 | `abs 0x69264` | `-` |
| `ldr` at 0x68b08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68b10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68b20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68b28 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68b2c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68b30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68b4c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68b54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68b60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68b64 | load | 4 | `abs 0x69268` | `-` |
| `ldr` at 0x68b70 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68b74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68b90 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68ba4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68ba8 | load | 4 | `abs 0x6926c` | `-` |
| `ldr` at 0x68bb4 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68bb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68bd4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68bdc | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68be0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68bfc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68c04 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68c08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68c24 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68c2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68c30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68c34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68c38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68c40 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68c44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68c60 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68c68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68c74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68c78 | load | 4 | `abs 0x69270` | `-` |
| `ldr` at 0x68c84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68c88 | load | 4 | `abs 0x69274` | `-` |
| `ldr` at 0x68c90 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68c94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68cb0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68cb8 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68cbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68cd8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68ce0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68ce4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68cf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68cfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68d00 | load | 4 | `abs 0x69274` | `-` |
| `ldr` at 0x68d08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68d18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68d20 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68d24 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68d28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68d44 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x68d4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68d50 | load | 4 | `abs 0x69278` | `-` |
| `ldr` at 0x68d54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68d5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68d6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68d74 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68d78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68d7c | load | 4 | `abs 0x692c4` | `-` |
| `ldr` at 0x68d84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68d94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68d9c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68da0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x68da8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x68db8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68dc8 | load | 4 | `abs 0x6927c` | `-` |
| `ldr` at 0x68dd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68dd4 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68dd8 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x68df8 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x68dfc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x68e00 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x68e04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68e08 | load | 4 | `abs 0x69260` | `-` |
| `ldr` at 0x68e10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68e20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68e28 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68e2c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68e30 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68e34 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68e38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68e3c | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x68e40 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x68e44 | load | 4 | `abs 0x132a5c` | `-` |
| `ldr` at 0x68e4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68e50 | load | 4 | `abs 0x692c4` | `-` |
| `ldr` at 0x68e58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68e68 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68e70 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x68e78 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x68e7c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68e80 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x68e84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68e90 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68e94 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x68e98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68ea4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68ea8 | load | 4 | `abs 0x692e0` | `-` |
| `ldr` at 0x68eac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68eb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68ec4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68ecc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x68ed4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x68ed8 | load | 4 | `abs 0x69280` | `-` |
| `ldr` at 0x68edc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x68ee0 | load | 4 | `abs 0x12d784` | `-` |
| `str` at 0x68ef0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x68ef4 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68ef8 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x68efc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68f08 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68f0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68f20 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68f2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x68f3c | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68f40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68f54 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68f5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68f60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x68f70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x68f78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68f7c | load | 4 | `abs 0x69284` | `-` |
| `ldr` at 0x68f84 | load | 4 | `abs 0x69288` | `-` |
| `ldr` at 0x68f8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x68f9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x68fa4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x68fa8 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68fac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68fc0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68fc8 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68fcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x68fe0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x68fe8 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x68fec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69000 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x69008 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6900c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69010 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69014 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6901c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69020 | load | 4 | `abs 0x692c4` | `-` |
| `ldr` at 0x69028 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69038 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69040 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69044 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6904c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x69054 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x6905c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x69064 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6906c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x6907c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69080 | load | 4 | `abs 0x6928c` | `-` |
| `ldr` at 0x6908c | load | 4 | `abs 0x692a0` | `-` |
| `str` at 0x69098 | store | 4 | `abs 0x132794` | `ret(0x194f0)` |
| `ldr` at 0x6909c | load | 4 | `abs 0x692a8` | `-` |
| `str` at 0x690a4 | store | 4 | `abs 0x13279c` | `0x0` |
| `ldr` at 0x690a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x690b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x690c8 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x690cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x690d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x690e0 | load | 4 | `abs 0x6929c` | `-` |
| `ldr` at 0x690e4 | load | 4 | `abs 0x13278c` | `-` |
| `str` at 0x690ec | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x690f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x690f4 | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x6910c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x69110 | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x69144 | load | 4 | `abs 0x692b0` | `-` |
| `ldr` at 0x6915c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x69160 | load | 4 | `abs 0x692b0` | `-` |
| `str` at 0x6918c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x69190 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69194 | load | 4 | `abs 0x69290` | `-` |
| `ldr` at 0x691a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x691b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x691bc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x691c0 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x691c4 | load | 4 | `abs 0x132ac0` | `-` |
| `ldr` at 0x691d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x691d4 | load | 4 | `abs 0x69294` | `-` |
| `ldr` at 0x69320 | load | 4 | `abs 0x69298` | `-` |
| `ldr` at 0x69324 | load | 4 | `abs 0x6929c` | `-` |
| `ldr` at 0x69328 | load | 4 | `abs 0x132790` | `-` |
| `ldr` at 0x6932c | load | 4 | `abs 0x13278c` | `-` |
| `str` at 0x69334 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x69344 | load | 4 | `abs 0x692a0` | `-` |
| `ldr` at 0x69348 | load | 4 | `abs 0x132794` | `-` |
| `str` at 0x69350 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x69354 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x69358 | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x69370 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x69374 | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x693a8 | load | 4 | `abs 0x692b0` | `-` |
| `ldr` at 0x693c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x693c4 | load | 4 | `abs 0x692b0` | `-` |
| `str` at 0x693f0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x693f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x693f8 | load | 4 | `abs 0x692ac` | `-` |
| `str` at 0x69410 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x69414 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x69418 | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x6944c | load | 4 | `abs 0x692b0` | `-` |
| `str` at 0x69464 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x69468 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6946c | load | 4 | `abs 0x692b0` | `-` |
| `str` at 0x69498 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x6949c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x694a0 | load | 4 | `abs 0x692a4` | `-` |
| `ldr` at 0x694b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x694c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x694c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x694e0 | load | 4 | `abs 0x692a8` | `-` |
| `ldr` at 0x694e4 | load | 4 | `abs 0x13279c` | `-` |
| `ldr` at 0x694fc | load | 4 | `abs 0x692a8` | `-` |
| `ldr` at 0x69500 | load | 4 | `abs 0x13279c` | `-` |
| `str` at 0x69508 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x6950c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x69510 | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x69528 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6952c | load | 4 | `abs 0x692ac` | `-` |
| `ldr` at 0x69560 | load | 4 | `abs 0x692b0` | `-` |
| `ldr` at 0x69578 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6957c | load | 4 | `abs 0x692b0` | `-` |
| `str` at 0x695a8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x695ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x695b0 | load | 4 | `abs 0x692b4` | `-` |
| `ldr` at 0x695c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x695c8 | load | 4 | `abs 0x692b8` | `-` |
| `ldr` at 0x695d4 | load | 4 | `abs 0x692bc` | `-` |
| `ldr` at 0x695d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x695dc | load | 4 | `abs 0x692c0` | `-` |
| `ldr` at 0x695e0 | load | 4 | `abs 0x12d78c` | `-` |
| `ldr` at 0x695e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x695ec | load | 4 | `abs 0x692c4` | `-` |
| `ldr` at 0x695f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69608 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69618 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x6961c | load | 4 | `(unknown)` | `-` |
| `str` at 0x69620 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x69624 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69628 | load | 4 | `abs 0x692c8` | `-` |
| `ldr` at 0x6962c | load | 4 | `abs 0x692cc` | `-` |
| `ldr` at 0x69638 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69658 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69664 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69674 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69678 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x6967c | load | 4 | `abs 0x132ae8` | `-` |
| `str` at 0x6969c | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x696a0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x696a4 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x696ac | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x696b0 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x696b4 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x696b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x696c4 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x696c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x696dc | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x696e4 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x696e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x696fc | load | 4 | `abs 0x132af0` | `-` |
| `ldr` at 0x69704 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69708 | load | 4 | `abs 0x692d0` | `-` |
| `ldr` at 0x6970c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69710 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69718 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69728 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69730 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69734 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6973c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x69744 | load | 4 | `abs 0x692d4` | `-` |
| `ldr` at 0x69748 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6974c | load | 4 | `abs 0x692d8` | `-` |
| `ldr` at 0x69750 | load | 4 | `abs 0x132ae8` | `-` |
| `ldr` at 0x69758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69768 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69770 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x697b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x697cc | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x697d4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x697d8 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x697dc | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x697e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x697ec | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x697f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6980c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69814 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x69818 | load | 4 | `(unknown)` | `-` |
| `str` at 0x69820 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x69824 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6982c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x69834 | load | 4 | `abs 0x692dc` | `-` |
| `ldr` at 0x6983c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69840 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x69860 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x69864 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x69868 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x69870 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x69874 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69878 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x6987c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69888 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x6988c | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x69890 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6989c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x698a0 | load | 4 | `abs 0x692e0` | `-` |
| `ldr` at 0x698a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x698ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x698bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x698c4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x698cc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x698d0 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x698d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x698f0 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x698f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x698fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6990c | load | 4 | `abs 0x692e4` | `-` |
| `ldr` at 0x69914 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69924 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6992c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69930 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69934 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69950 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69958 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6996c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69974 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69978 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69994 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6999c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x699b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x699b8 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x699bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x699d8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x699e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x699f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x699fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69a00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69a04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69a08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69a10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69a20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69a28 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69a2c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69a30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69a4c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69a54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69a68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69a70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69a7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69a80 | load | 4 | `abs 0x692e8` | `-` |
| `ldr` at 0x69a8c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69a90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69aac | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69ab4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69ac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69ad0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69adc | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69ae0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69afc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69b04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69b18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69b20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69b24 | load | 4 | `abs 0x692ec` | `-` |
| `ldr` at 0x69b28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69b34 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69b38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69b54 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69b5c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69b60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69b7c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69b84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69ba0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69ba4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69bb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69bb4 | load | 4 | `abs 0x692f0` | `-` |
| `ldr` at 0x69bc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69bd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69bd8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69bdc | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69be0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69bfc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69c04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69c18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69c20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69c2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69c30 | load | 4 | `abs 0x692f4` | `-` |
| `ldr` at 0x69c3c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69c40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69c5c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69c64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69c78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69c80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69c8c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69c90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x69cac | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x69cb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69cc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69cd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69cd4 | load | 4 | `abs 0x692f8` | `-` |
| `ldr` at 0x69cd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69ce4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69ce8 | load | 4 | `abs 0x692fc` | `-` |
| `ldr` at 0x69cf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69d00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69d08 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x69d0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x69d14 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x69d1c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x69d24 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x69d34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69d44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69d54 | store | 4 | `[sp-24] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x69d58 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x69d5c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x69d60 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69d64 | load | 4 | `abs 0x132ac4` | `-` |
| `ldr` at 0x69d70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69d74 | load | 4 | `abs 0x69300` | `-` |
| `ldr` at 0x69d80 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69d84 | load | 4 | `abs 0x132ac4` | `-` |
| `ldr` at 0x69d90 | load | 4 | `abs 0x69304` | `-` |
| `ldr` at 0x69d98 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69d9c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69da0 | load | 4 | `abs 0x132ac8` | `-` |
| `ldr` at 0x69da4 | load | 4 | `abs 0x132acc` | `-` |
| `str` at 0x69db0 | store | 4 | `sp -0x3c` | `ret(0x6a0b0)` |
| `ldr` at 0x69db4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x69dc0 | load | 4 | `abs 0x69314` | `-` |
| `str` at 0x69dc4 | store | 4 | `sp -0x40` | `0x132a50` |
| `ldr` at 0x69dc8 | load | 4 | `abs 0x69314` | `-` |
| `str` at 0x69dcc | store | 4 | `sp -0x44` | `0x132a50` |
| `ldr` at 0x69dd0 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69dd4 | load | 4 | `abs 0x132ad0` | `-` |
| `str` at 0x69dd8 | store | 4 | `sp -0x58` | `[0x132ad0]` |
| `ldr` at 0x69ddc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x69de0 | load | 4 | `ret(0x6a0b0) +0xc` | `-` |
| `ldr` at 0x69dec | load | 4 | `abs 0x69308` | `-` |
| `str` at 0x69df0 | store | 4 | `sp -0x48` | `0xfc810` |
| `ldr` at 0x69df8 | load | 4 | `abs 0x6930c` | `-` |
| `str` at 0x69dfc | store | 4 | `sp -0x48` | `0xfc814` |
| `ldr` at 0x69e00 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x69e04 | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0x69e08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69e0c | load | 4 | `abs 0x69310` | `-` |
| `ldr` at 0x69e10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x69e14 | load | 4 | `[sp-64] +0x78` | `-` |
| `ldr` at 0x69e18 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x69e1c | load | 4 | `[sp-68] +0x7c` | `-` |
| `ldr` at 0x69e28 | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69e2c | load | 4 | `abs 0x69314` | `-` |
| `ldr` at 0x69e30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69e34 | load | 4 | `abs 0x69318` | `-` |
| `ldr` at 0x69e38 | load | 4 | `abs 0x132ac8` | `-` |
| `ldr` at 0x69e3c | load | 4 | `abs 0x132acc` | `-` |
| `ldr` at 0x69e4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69e50 | load | 4 | `abs 0x6931c` | `-` |
| `ldm` at 0x69e5c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x69e5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x69e5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x69e5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x682e4 | `(unknown callee) 0x619dc` | `0xfc518` | `r1` | `r2` | `?` |
| `bl` at 0x6830c | `(unknown callee) 0x1943c` | `?` | `?` | `[0x132adc]` | `?` |
| `bl` at 0x683f0 | `(unknown callee) 0x6f580` | `?` | `0x132a50` | `?` | `?` |
| `bl` at 0x6845c | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc524` | `?` | `?` |
| `bl` at 0x6846c | `(unknown callee) 0x19808` | `[sp-32]` | `0xfc52c` | `?` | `?` |
| `bl` at 0x68474 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x684fc | `(unknown callee) 0x5f954` | `([sp-32] + ret(0x19538))` | `?` | `?` | `?` |
| `bl` at 0x68504 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68550 | `(unknown callee) 0x19808` | `([sp-32] + ret(0x19538))` | `0xfc534` | `?` | `?` |
| `bl` at 0x68584 | `(unknown callee) 0x5fa6c` | `[0x132a60]` | `?` | `?` | `?` |
| `bl` at 0x68598 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc53c` | `ret(0x5fa6c)` | `ret(0x5fa6c)` |
| `bl` at 0x685a0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x685f4 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xfc540` | `?` | `?` |
| `bl` at 0x685fc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x6866c | `(unknown callee) 0x69eec` | `?` | `?` | `?` | `?` |
| `bl` at 0x686ac | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc544` | `?` | `?` |
| `bl` at 0x6870c | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc550` | `?` | `?` |
| `bl` at 0x6871c | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc55c` | `?` | `?` |
| `bl` at 0x68724 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68750 | `(unknown callee) 0x195e0` | `([sp-32] + ret(0x19538))` | `0xfc568` | `ret(0x19538)` | `[sp-60]` |
| `bl` at 0x68770 | `(unknown callee) 0x195e0` | `([sp-32] + ret(0x19538))` | `0xfc574` | `ret(0x19538)` | `[[sp-60]+52]` |
| `bl` at 0x68780 | `(unknown callee) 0x195e0` | `([sp-32] + ret(0x19538))` | `0xfc57c` | `ret(0x19538)` | `[[sp-60]+52]` |
| `bl` at 0x68818 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc5b4` | `?` | `[[sp-60]+56]` |
| `bl` at 0x6884c | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc5bc` | `?` | `?` |
| `bl` at 0x68884 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc5c8` | `?` | `?` |
| `bl` at 0x688a8 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc5d4` | `?` | `?` |
| `bl` at 0x688cc | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc5e0` | `?` | `?` |
| `bl` at 0x688dc | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc5ec` | `?` | `?` |
| `bl` at 0x688e8 | `(unknown callee) 0x195e0` | `?` | `0xfbe88` | `?` | `?` |
| `bl` at 0x688f0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68938 | `(unknown callee) 0x197a8` | `?` | `0xfc5f8` | `[0x132adc]` | `0x132a50` |
| `bl` at 0x68940 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68968 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xfc604` | `[0x132780]` | `0x132780` |
| `bl` at 0x68970 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68998 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xfc61c` | `[0x132784]` | `0x132784` |
| `bl` at 0x689a0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x689c8 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xfc634` | `[0x132788]` | `0x132788` |
| `bl` at 0x689d0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x689e8 | `(unknown callee) 0x61a78` | `?` | `?` | `?` | `?` |
| `bl` at 0x68a04 | `(unknown callee) 0x619dc` | `0xfc64c` | `r1` | `r2` | `?` |
| `bl` at 0x68a34 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x68a50 | `(unknown callee) 0x197a8` | `[[sp-24]]` | `0xfc658` | `?` | `[[sp-24]]` |
| `bl` at 0x68a58 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68a8c | `(unknown callee) 0x5f954` | `([sp-32] + ret(0x19538))` | `[0x132a58]` | `[0x132a60]` | `[0x132a5c]` |
| `bl` at 0x68a98 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfbe88` | `?` | `?` |
| `bl` at 0x68aa0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68ad8 | `(unknown callee) 0x6f580` | `?` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x68af8 | `(unknown callee) 0x5fa6c` | `[0x132a60]` | `[sp-48]` | `?` | `0x132a50` |
| `bl` at 0x68b0c | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc664` | `[sp-48]` | `ret(0x5fa6c)` |
| `bl` at 0x68b14 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68b68 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc670` | `?` | `?` |
| `bl` at 0x68bac | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc680` | `?` | `?` |
| `bl` at 0x68c3c | `(unknown callee) 0x5f954` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68c7c | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc69c` | `?` | `?` |
| `bl` at 0x68c8c | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc6a4` | `?` | `?` |
| `bl` at 0x68ce8 | `(unknown callee) 0x5fa6c` | `?` | `?` | `?` | `?` |
| `bl` at 0x68cf8 | `(unknown callee) 0x195e0` | `[sp-32]` | `ret(0x5fa6c)` | `?` | `ret(0x5fa6c)` |
| `bl` at 0x68d04 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfc6a4` | `?` | `?` |
| `bl` at 0x68d0c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68d58 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc6a8` | `?` | `?` |
| `bl` at 0x68d60 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68d80 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfbe88` | `?` | `?` |
| `bl` at 0x68d88 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68db0 | `(unknown callee) 0x61a78` | `?` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x68dcc | `(unknown callee) 0x619dc` | `0xfc6b4` | `r1` | `r2` | `?` |
| `bl` at 0x68df0 | `(unknown callee) 0x1943c` | `?` | `?` | `[0x132adc]` | `?` |
| `bl` at 0x68e0c | `(unknown callee) 0x197a8` | `[[sp-24]]` | `0xfc658` | `?` | `[[sp-24]]` |
| `bl` at 0x68e14 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68e48 | `(unknown callee) 0x5f954` | `([sp-32] + ret(0x19538))` | `[0x132a58]` | `[0x132a60]` | `[0x132a5c]` |
| `bl` at 0x68e54 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfbe88` | `?` | `?` |
| `bl` at 0x68e5c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68e9c | `(unknown callee) 0x5fa6c` | `[0x132a60]` | `?` | `[0x132ad4]` | `0x132a50` |
| `bl` at 0x68eb0 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc7a8` | `[sp-48]` | `ret(0x5fa6c)` |
| `bl` at 0x68eb8 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x68f88 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xfc6c0` | `?` | `0xfc6c4` |
| `bl` at 0x68f90 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69018 | `(unknown callee) 0x5f954` | `([sp-32] + ret(0x19538))` | `?` | `?` | `?` |
| `bl` at 0x69024 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfbe88` | `?` | `?` |
| `bl` at 0x6902c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69074 | `(unknown callee) 0x61a78` | `ret(0x19538)` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x69094 | `(unknown callee) 0x194f0` | `0x0` | `r1` | `r1` | `0x1010` |
| `bl` at 0x690c0 | `(unknown callee) 0x1943c` | `0xc8` | `?` | `?` | `?` |
| `bl` at 0x690d8 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x691a0 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc6c8` | `?` | `?` |
| `bl` at 0x691a8 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x691d8 | `(unknown callee) 0x19808` | `([sp-32] + ret(0x19538))` | `0xfc6e0` | `ret(0x19538)` | `[0x132ac0]` |
| `bl` at 0x6933c | `(unknown callee) 0x194f0` | `0x0` | `[0x132790]` | `0x13278c` | `?` |
| `bl` at 0x694ac | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc6f8` | `?` | `?` |
| `bl` at 0x694b4 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x694d0 | `(unknown callee) 0x1d444` | `0x9a` | `?` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x694f4 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[0x13279c]` |
| `bl` at 0x695bc | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc724` | `?` | `?` |
| `bl` at 0x695cc | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc73c` | `?` | `[0x13279c]` |
| `bl` at 0x695e4 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc750` | `[0x12d78c]` | `0x12d78c` |
| `bl` at 0x695f0 | `(unknown callee) 0x195e0` | `[sp-32]` | `0xfbe88` | `?` | `?` |
| `bl` at 0x69610 | `(unknown callee) 0x1943c` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x69630 | `(unknown callee) 0x197a8` | `?` | `0xfc76c` | `0x29a500` | `?` |
| `bl` at 0x69644 | `(unknown callee) 0x62c00` | `?` | `?` | `?` | `?` |
| `bl` at 0x6965c | `(unknown callee) 0x62c80` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x69694 | `(unknown callee) 0x1943c` | `?` | `?` | `[0x132ae8]` | `?` |
| `bl` at 0x69714 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc784` | `?` | `?` |
| `bl` at 0x6971c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69754 | `(unknown callee) 0x197a8` | `?` | `0xfc790` | `[0x132ae8]` | `0x132a50` |
| `bl` at 0x6975c | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69790 | `(unknown callee) 0x1b5f0` | `0x9a` | `0x0` | `?` | `?` |
| `bl` at 0x697b0 | `(unknown callee) 0x1b5f0` | `0x9a` | `0x1` | `?` | `?` |
| `bl` at 0x69838 | `(unknown callee) 0x619dc` | `0xfc79c` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x69858 | `(unknown callee) 0x1943c` | `?` | `?` | `[sp-60]` | `?` |
| `bl` at 0x69894 | `(unknown callee) 0x5fa6c` | `[0x132a60]` | `?` | `[0x132ad4]` | `0x132a50` |
| `bl` at 0x698a8 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc7a8` | `[sp-48]` | `ret(0x5fa6c)` |
| `bl` at 0x698b0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69910 | `(unknown callee) 0x19808` | `[sp-32]` | `0xfc7b4` | `?` | `[sp-36]` |
| `bl` at 0x69918 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69a0c | `(unknown callee) 0x5f954` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69a14 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69a84 | `(unknown callee) 0x19808` | `[sp-32]` | `0xfc7bc` | `?` | `?` |
| `bl` at 0x69b2c | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc7c4` | `?` | `?` |
| `bl` at 0x69ba8 | `(unknown callee) 0x5fa6c` | `?` | `?` | `?` | `?` |
| `bl` at 0x69bbc | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc7d0` | `ret(0x5fa6c)` | `ret(0x5fa6c)` |
| `bl` at 0x69bc4 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69c34 | `(unknown callee) 0x19808` | `[sp-32]` | `0xfc7dc` | `?` | `?` |
| `bl` at 0x69cdc | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc7e4` | `?` | `?` |
| `bl` at 0x69cec | `(unknown callee) 0x19808` | `[sp-32]` | `0xfc7ec` | `?` | `?` |
| `bl` at 0x69cf4 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x69d2c | `(unknown callee) 0x61a78` | `?` | `?` | `[0x132ad4]` | `?` |
| `bl` at 0x69d4c | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x69d78 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc7f8` | `?` | `[0x132ac4]` |
| `bl` at 0x69d94 | `(unknown callee) 0x619dc` | `0xfc804` | `?` | `?` | `[0x132ac4]` |
| `bl` at 0x69da8 | `(unknown callee) 0x6a0b0` | `[0x132ac8]` | `[0x132acc]` | `0x132a50` | `0x132a50` |
| `bl` at 0x69e20 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc820` | `[[sp-64]+120]` | `[[sp-68]+124]` |
| `bl` at 0x69e40 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc840` | `[0x132ac8]` | `[0x132acc]` |
| `bl` at 0x69e44 | `(unknown callee) 0x61a78` | `ret(0x197a8)` | `?` | `?` | `?` |
| `bl` at 0x69e54 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xfc858` | `?` | `[0x132ac4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x682c8 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x682d8 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x68338 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0x683b4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x683c4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x683fc | `ret(0x6f580) != 0x0` | `ret(0x6f580) == 0x0` |
| `beq` at 0x6842c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x68544 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x685c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x68620 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x686a0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x686dc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x68744 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `beq` at 0x68764 | `[[sp-60]+52] == 0x0` | `[[sp-60]+52] != 0x0` |
| `ldrls` at 0x68790 | `[[sp-60]+56] <= 0x5 (unsigned, 32-bit)` | `[[sp-60]+56] > 0x5 (unsigned, 32-bit)` |
| `beq` at 0x6882c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x68840 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x68864 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x68878 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6889c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x688c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x68924 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x689fc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x68ad0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x68ae4 | `ret(0x6f580) != 0x0` | `ret(0x6f580) == 0x0` |
| `bne` at 0x68b5c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x68ba0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x68c70 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x68dc4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x68e8c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0x68ee8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x68f04 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x68f34 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x68f68 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x69088 | `r1 != 0x1010` | `r1 == 0x1010` |
| `beq` at 0x690b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x691cc | `[0x132ac0] == 0x0` | `[0x132ac0] != 0x0` |
| `beq` at 0x694dc | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `beq` at 0x694ec | `[0x13279c] == 0x0` | `[0x13279c] != 0x0` |
| `beq` at 0x69604 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x69640 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x69654 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x69670 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x696c0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x69784 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x697a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x697c4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x697e8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x69884 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x69904 | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `bne` at 0x69a78 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x69ad8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x69c28 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x69c88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x69d40 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x69d6c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x69d8c | `[0x132ac4] == 0x0` | `[0x132ac4] != 0x0` |
| `beq` at 0x69dbc | `ret(0x6a0b0) == 0x0` | `ret(0x6a0b0) != 0x0` |
| `bne` at 0x69de8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x69e5c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x68790).

## `L1CfgRingTask`

- `L1CfgRingTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60e78 to 0x619b4.
  Established. The unnamed `0x619b4` call carries `0x1f4` and entry `r1`, `r2`, `r3` (`bl` at 0x60e94).
  A zero `[0x1327a4]` takes `0x60f04`, while nonzero falls through (`beq` at 0x60ea4).
  An included absolute word store clears `0x1327a4` (`str` at 0x60ed8).
  Ring task timing and link policy remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x60e88 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x60e8c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x60e98 | load | 4 | `abs 0x6197c` | `-` |
| `ldr` at 0x60e9c | load | 4 | `abs 0x1327a4` | `-` |
| `ldr` at 0x60eb4 | load | 4 | `abs 0x6197c` | `-` |
| `ldr` at 0x60eb8 | load | 4 | `abs 0x1327a4` | `-` |
| `ldr` at 0x60ec8 | load | 4 | `abs 0x61980` | `-` |
| `ldr` at 0x60ed0 | load | 4 | `abs 0x6197c` | `-` |
| `str` at 0x60ed8 | store | 4 | `abs 0x1327a4` | `0x0` |
| `ldr` at 0x60edc | load | 4 | `abs 0x61984` | `-` |
| `ldr` at 0x60ee0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x60ef0 | load | 4 | `abs 0x61988` | `-` |
| `ldr` at 0x60ef4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x60ef8 | load | 4 | `abs 0x6198c` | `-` |
| `ldr` at 0x60f04 | load | 4 | `abs 0x61990` | `-` |
| `ldr` at 0x60f08 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x60f10 | store | 4 | `sp -0x44` | `[0x132ad8]+156` |
| `ldr` at 0x60f14 | load | 4 | `abs 0x61990` | `-` |
| `ldr` at 0x60f18 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x60f20 | store | 4 | `sp -0x48` | `[0x132ad8]+312` |
| `ldr` at 0x60f4c | load | 4 | `abs 0x61994` | `-` |
| `str` at 0x60f54 | store | 4 | `abs 0x13277c` | `?` |
| `ldr` at 0x60f70 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60f74 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x60f80 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60f84 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60f90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60f94 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60fa0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60fa4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60fb0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60fb4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60fc0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60fc4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60fd0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60fd4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60fe0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60fe4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x60ff0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x60ff4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61000 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61004 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61010 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61014 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61020 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61024 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61030 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61034 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61040 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61044 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61064 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61068 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x61074 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61078 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61084 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61088 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61094 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61098 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x610a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x610a8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x610b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x610b8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x610c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x610c8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x610d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x610d8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x610e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x610e8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x610f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x610f8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61104 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61108 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61114 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61118 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61124 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61128 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61134 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61138 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61148 | load | 4 | `abs 0x61990` | `-` |
| `ldr` at 0x6114c | load | 4 | `abs 0x132a60` | `-` |
| `str` at 0x61184 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x61188 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6118c | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x61198 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6119c | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x611ac | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x611b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x611b8 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x611c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x611c8 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x611d8 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x611dc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x611e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x611ec | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x611f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x611fc | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x6120c | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x61214 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61218 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x61224 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61228 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x61238 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6123c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x61248 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6124c | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x61258 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6125c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61268 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6126c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61278 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6127c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61288 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6128c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61298 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6129c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x612a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x612ac | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x612b8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x612bc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x612c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x612cc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x612d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x612dc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x612e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x612ec | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x612f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x612fc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61308 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6130c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61318 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6131c | load | 4 | `[sp-68] +0x10` | `-` |
| `str` at 0x6132c | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x61334 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61338 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x61344 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61348 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61354 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61358 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61364 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61368 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61374 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61378 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61384 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61388 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61394 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61398 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x613a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x613a8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x613b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x613b8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x613c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x613c8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x613d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x613d8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x613e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x613e8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x613f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x613f8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61404 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61408 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x61418 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6141c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x61480 | load | 4 | `abs 0x61990` | `-` |
| `str` at 0x61490 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x61494 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x61498 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x614a0 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x614a4 | load | 4 | `abs 0x61998` | `-` |
| `str` at 0x614bc | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0x614c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x614d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x614d4 | load | 4 | `abs 0x6199c` | `-` |
| `ldr` at 0x614d8 | load | 4 | `abs 0x132ef8` | `-` |
| `str` at 0x614dc | store | 4 | `ret(0xaa424) +0x18` | `[0x132ef8]` |
| `ldr` at 0x614e0 | load | 4 | `abs 0x619a0` | `-` |
| `ldr` at 0x614e4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x614f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6150c | load | 4 | `abs 0x61994` | `-` |
| `str` at 0x61514 | store | 4 | `abs 0x13277c` | `?` |
| `ldr` at 0x6151c | load | 4 | `abs 0x61994` | `-` |
| `ldr` at 0x61520 | load | 4 | `abs 0x13277c` | `-` |
| `ldr` at 0x61554 | load | 4 | `abs 0x61994` | `-` |
| `str` at 0x6155c | store | 4 | `abs 0x13277c` | `0x0` |
| `ldr` at 0x61560 | load | 4 | `abs 0x619a4` | `-` |
| `ldr` at 0x61570 | load | 4 | `abs 0x619a4` | `-` |
| `ldr` at 0x61574 | load | 4 | `abs 0x61990` | `-` |
| `ldr` at 0x61594 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61598 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x615a4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x615a8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x615b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x615b8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x615c4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x615c8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x615d4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x615d8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x615e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x615e8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x615f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x615f8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61604 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61608 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61614 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61618 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61624 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61628 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61634 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61638 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61644 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61648 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61654 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61658 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x61664 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61668 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x616d0 | load | 4 | `abs 0x619a4` | `-` |
| `str` at 0x616e0 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x616e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x616e8 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x616f0 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x616f4 | load | 4 | `abs 0x61998` | `-` |
| `str` at 0x6170c | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0x61710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6171c | load | 4 | `abs 0x619a8` | `-` |
| `ldr` at 0x61728 | load | 4 | `abs 0x61984` | `-` |
| `ldr` at 0x6172c | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x6173c | load | 4 | `abs 0x61988` | `-` |
| `ldr` at 0x61740 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x61744 | load | 4 | `abs 0x619ac` | `-` |
| `ldr` at 0x6174c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61750 | load | 4 | `abs 0x6199c` | `-` |
| `ldr` at 0x61754 | load | 4 | `abs 0x132ef8` | `-` |
| `str` at 0x61758 | store | 4 | `(unknown)` | `[0x132ef8]` |
| `ldr` at 0x6175c | load | 4 | `abs 0x619a0` | `-` |
| `ldr` at 0x61760 | load | 4 | `abs 0x29a5a4` | `-` |
| `ldr` at 0x6176c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61788 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6178c | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x61798 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6179c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x617a8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x617ac | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x617b8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x617bc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x617c8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x617cc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x617d8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x617dc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x617e8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x617ec | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x617f8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x617fc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61808 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6180c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61818 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6181c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61828 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6182c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61838 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6183c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61848 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6184c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x61858 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6185c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x618c4 | load | 4 | `abs 0x619a4` | `-` |
| `str` at 0x618d4 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x618d8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x618dc | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x618e4 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x618e8 | load | 4 | `abs 0x61998` | `-` |
| `str` at 0x61900 | store | 4 | `sp -0x1c` | `ret(0xaa424)` |
| `ldr` at 0x61904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61910 | load | 4 | `abs 0x619a8` | `-` |
| `ldr` at 0x6191c | load | 4 | `abs 0x61984` | `-` |
| `ldr` at 0x61920 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x61930 | load | 4 | `abs 0x61988` | `-` |
| `ldr` at 0x61934 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x61938 | load | 4 | `abs 0x619b0` | `-` |
| `ldr` at 0x61940 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61944 | load | 4 | `abs 0x6199c` | `-` |
| `ldr` at 0x61948 | load | 4 | `abs 0x132ef8` | `-` |
| `str` at 0x6194c | store | 4 | `(unknown)` | `[0x132ef8]` |
| `ldr` at 0x61950 | load | 4 | `abs 0x619a0` | `-` |
| `ldr` at 0x61954 | load | 4 | `abs 0x29a5a8` | `-` |
| `ldr` at 0x61960 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x61978 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61978 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60e94 | `(unknown callee) 0x619b4` | `0x1f4` | `r1` | `r2` | `r3` |
| `bl` at 0x60eac | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `[0x1327a4]` |
| `bl` at 0x60ecc | `(unknown callee) 0x619dc` | `0xfbd2c` | `?` | `[0x1327a4]` | `ret(0x194f0)` |
| `bl` at 0x60efc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbd3c` | `0x1327a4` | `0x12eeb8` |
| `bl` at 0x60f00 | `(unknown callee) 0x62e24` | `?` | `?` | `?` | `?` |
| `bl` at 0x60f28 | `(unknown callee) 0x6f4d4` | `0x1` | `?` | `?` | `[0x132ad8]+312` |
| `bl` at 0x60f3c | `(unknown callee) 0x6f4d4` | `0x2` | `?` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x60f60 | `(unknown callee) 0x6f4d4` | `0x1` | `?` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x61054 | `(unknown callee) 0x6f4d4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6115c | `(unknown callee) 0x6f4d4` | `0x1` | `?` | `?` | `[0x132a60]` |
| `bl` at 0x61170 | `(unknown callee) 0x6f4d4` | `0x2` | `?` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x6143c | `(unknown callee) 0x5eac8` | `sp-64` | `sp-60` | `0x3` | `sp-64` |
| `bl` at 0x61454 | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6146c | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x61488 | `(unknown callee) 0x67280` | `sp-64` | `sp-60` | `0x132a50` | `0x1` |
| `bl` at 0x614b4 | `(unknown callee) 0xaa424` | `0x132a48` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x61508 | `(unknown callee) 0xa9d18` | `?` | `[sp-28]` | `0x29a550` | `?` |
| `bl` at 0x61530 | `(unknown callee) 0x6f4d4` | `0x1` | `?` | `?` | `[0x13277c]` |
| `bl` at 0x61544 | `(unknown callee) 0x6f4d4` | `0x2` | `?` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x6156c | `(unknown callee) 0x19748` | `0x1327a8` | `0x0` | `0x230` | `?` |
| `bl` at 0x6157c | `(unknown callee) 0x19610` | `0x1327a8` | `0x132a50` | `0x70` | `?` |
| `bl` at 0x61584 | `(unknown callee) 0x6f4d4` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6168c | `(unknown callee) 0x5eac8` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x616a4 | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x616bc | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x616d8 | `(unknown callee) 0x67280` | `sp-64` | `sp-60` | `0x1327a8` | `0x5` |
| `bl` at 0x61704 | `(unknown callee) 0xaa424` | `0x132a48` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x61720 | `(unknown callee) 0x1e4ac` | `0xfbd50` | `?` | `?` | `ret(0xaa424)` |
| `bl` at 0x61748 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbd7c` | `?` | `0x12eeb8` |
| `bl` at 0x61770 | `(unknown callee) 0xa9d18` | `[0x29a5a4]+20` | `[sp-28]` | `?` | `[0x29a5a4]+20` |
| `bl` at 0x61778 | `(unknown callee) 0x6f4d4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x61880 | `(unknown callee) 0x5eac8` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x61898 | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x618b0 | `(unknown callee) 0x5eb28` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x618cc | `(unknown callee) 0x67280` | `sp-64` | `sp-60` | `0x1327a8` | `0x5` |
| `bl` at 0x618f8 | `(unknown callee) 0xaa424` | `0x132a48` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x61914 | `(unknown callee) 0x1e4ac` | `0xfbd50` | `?` | `?` | `ret(0xaa424)` |
| `bl` at 0x6193c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbd94` | `?` | `0x12eeb8` |
| `bl` at 0x61964 | `(unknown callee) 0xa9d18` | `[0x29a5a8]+20` | `[sp-28]` | `?` | `[0x29a5a8]+20` |
| `bl` at 0x6196c | `(unknown callee) 0x619b4` | `0x1f4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60ea4 | `[0x1327a4] == 0x0` | `[0x1327a4] != 0x0` |
| `bge` at 0x60ec0 | `ret(0x194f0) >= [0x1327a4] (signed, 32-bit)` | `ret(0x194f0) < [0x1327a4] (signed, 32-bit)` |
| `beq` at 0x60eec | `? == 0x0` | `? != 0x0` |
| `bne` at 0x60f34 | `ret(0x6f4d4) != 0x0` | `ret(0x6f4d4) == 0x0` |
| `bne` at 0x60f48 | `ret(0x6f4d4) != 0x0` | `ret(0x6f4d4) == 0x0` |
| `beq` at 0x60f6c | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x60f7c | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x60f8c | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x60f9c | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x60fac | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x60fbc | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x60fcc | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x60fdc | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x60fec | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x60ffc | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6100c | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6101c | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6102c | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6103c | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `bne` at 0x6104c | `[[sp-68]+16] != 0x4d` | `[[sp-68]+16] == 0x4d` |
| `beq` at 0x61060 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x61070 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x61080 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x61090 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x610a0 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x610b0 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x610c0 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x610d0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x610e0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x610f0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x61100 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x61110 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x61120 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x61130 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `bne` at 0x61140 | `[[sp-72]+16] != 0x4d` | `[[sp-72]+16] == 0x4d` |
| `bne` at 0x61154 | `[0x132a60] != 0x42` | `[0x132a60] == 0x42` |
| `beq` at 0x61168 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x6117c | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x61194 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `bne` at 0x611a4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x611c0 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `bne` at 0x611d0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x611e4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x611f4 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x61204 | `[[sp-68]+120] == 0x0` | `[[sp-68]+120] != 0x0` |
| `beq` at 0x61220 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x61230 | `[[sp-72]+120] == 0x0` | `[[sp-72]+120] != 0x0` |
| `bne` at 0x61244 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x61254 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x61264 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x61274 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x61284 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x61294 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x612a4 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x612b4 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x612c4 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x612d4 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x612e4 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x612f4 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x61304 | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x61314 | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `beq` at 0x61324 | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `beq` at 0x61340 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x61350 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x61360 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x61370 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x61380 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x61390 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x613a0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x613b0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x613c0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x613d0 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x613e0 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x613f0 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x61400 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `beq` at 0x61410 | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `beq` at 0x61424 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x614c8 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `blt` at 0x61528 | `[0x13277c] < 0x0 (signed, 32-bit)` | `[0x13277c] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x6153c | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `bne` at 0x61550 | `ret(0x6f4d4) != 0x0` | `ret(0x6f4d4) == 0x0` |
| `beq` at 0x61590 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x615a0 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x615b0 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x615c0 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x615d0 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x615e0 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x615f0 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x61600 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x61610 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x61620 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x61630 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x61640 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x61650 | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x61660 | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `beq` at 0x61670 | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `bne` at 0x61718 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `beq` at 0x61738 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61784 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x61794 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x617a4 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x617b4 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x617c4 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x617d4 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x617e4 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x617f4 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x61804 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x61814 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x61824 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x61834 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x61844 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x61854 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `beq` at 0x61864 | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `bne` at 0x6190c | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `beq` at 0x6192c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61978 | `r0` | `ret(0xaa424)` |

## `L1CfgDiscoveryRequest`

- `L1CfgDiscoveryRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62048 to 0x6288c.
  Established. An included word store writes `0x3e8` to an unknown effective target (`str` at 0x62128).
  The unnamed `0xaa424` call carries `0x132a48`, `sp-28`, `0x0`, `0x0` (`bl` at 0x627b0);
  its nonzero result takes `0x627dc`, while zero falls through (`bne` at 0x627c4).
  Discovery request packet construction remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62058 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x62060 | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x62064 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62084 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6208c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6209c | load | 4 | `abs 0x62858` | `-` |
| `ldr` at 0x620a0 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x620b0 | load | 4 | `abs 0x6285c` | `-` |
| `ldr` at 0x620b4 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x620b8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x620bc | load | 4 | `abs 0x62860` | `-` |
| `ldr` at 0x620c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x620c4 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x620cc | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x620e8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x620ec | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62108 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6210c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62128 | store | 4 | `(unknown)` | `0x3e8` |
| `ldr` at 0x6212c | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x62148 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6214c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62154 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x62158 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62174 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62178 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62194 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62198 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x621b0 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x621b4 | load | 4 | `abs 0x132a58` | `-` |
| `str` at 0x621b8 | store | 4 | `(unknown)` | `[0x132a58]` |
| `ldr` at 0x621bc | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x621d8 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x621dc | load | 4 | `sp -0x78` | `-` |
| `str` at 0x621e4 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x621e8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62204 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62208 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62224 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62228 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62240 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62244 | load | 4 | `abs 0x132a5c` | `-` |
| `str` at 0x62248 | store | 4 | `(unknown)` | `[0x132a5c]` |
| `ldr` at 0x6224c | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x62268 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6226c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62274 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x62278 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62294 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62298 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x622b4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x622b8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x622d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x622d4 | store | 4 | `(unknown)` | `[sp-20]` |
| `ldr` at 0x622d8 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x622f4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x622f8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62300 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x62304 | load | 4 | `abs 0x62864` | `-` |
| `ldr` at 0x6230c | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62310 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x6231c | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62320 | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x62330 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62350 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x62358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62364 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62384 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6238c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6239c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x623b8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x623bc | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x623d8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x623dc | load | 4 | `sp -0x78` | `-` |
| `str` at 0x623f8 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x623fc | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x62418 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6241c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62424 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `str` at 0x62434 | store | 4 | `sp -0x18` | `ret(0x1943c)` |
| `ldr` at 0x62438 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x6243c | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62440 | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x62444 | load | 4 | `abs 0x132a5c` | `-` |
| `str` at 0x62450 | store | 4 | `sp -0x7c` | `ret(0x6a0b0)` |
| `ldr` at 0x62454 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x62458 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x62464 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62480 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x62484 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x624a0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x624a4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x624bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x624c0 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x624c4 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x624e0 | store | 2 | `(unknown)` | `0x18` |
| `ldr` at 0x624e4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x624ec | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x624f4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62510 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62514 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62530 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62534 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6254c | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62550 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x62554 | store | 4 | `(unknown)` | `[0x132adc]` |
| `ldr` at 0x62558 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x62574 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x62578 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62580 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x62584 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62588 | load | 4 | `abs 0x132adc` | `-` |
| `str` at 0x625a8 | store | 4 | `sp -0x18` | `ret(0x1943c)` |
| `ldr` at 0x625ac | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x625b0 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x625b4 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x625c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x625cc | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x625d8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x625f4 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x625f8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x62614 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x62618 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62630 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x62634 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x62638 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62650 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x62654 | load | 4 | `abs 0x132adc` | `-` |
| `strh` at 0x62668 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x6266c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62674 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6267c | load | 4 | `abs 0x62868` | `-` |
| `str` at 0x62680 | store | 4 | `sp -0x1c` | `0xffff00e` |
| `ldr` at 0x62684 | load | 4 | `abs 0x62858` | `-` |
| `ldr` at 0x62688 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x62698 | load | 4 | `abs 0x6286c` | `-` |
| `ldr` at 0x6269c | load | 4 | `abs 0x12d788` | `-` |
| `ldr` at 0x626a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x626b4 | load | 4 | `abs 0x6285c` | `-` |
| `ldr` at 0x626b8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x626bc | load | 4 | `abs 0x62870` | `-` |
| `ldr` at 0x626c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x626cc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x626d0 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x626d4 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x626d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x626e4 | load | 4 | `abs 0x6285c` | `-` |
| `ldr` at 0x626e8 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x626ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62700 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x62708 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x6270c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62720 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x62728 | load | 4 | `abs 0x62854` | `-` |
| `ldr` at 0x6272c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62740 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x62748 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6274c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x62750 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x62754 | load | 4 | `abs 0x62874` | `-` |
| `ldr` at 0x62758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6275c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62764 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6276c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x62774 | load | 4 | `abs 0x6285c` | `-` |
| `ldr` at 0x62778 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6277c | load | 4 | `abs 0x62878` | `-` |
| `str` at 0x6278c | store | 4 | `sp -0x88` | `0x2` |
| `ldr` at 0x62790 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x62794 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0x6279c | store | 4 | `sp -0x80` | `sp-116` |
| `ldr` at 0x627a0 | load | 4 | `abs 0x6287c` | `-` |
| `str` at 0x627b8 | store | 4 | `sp -0x20` | `ret(0xaa424)` |
| `ldr` at 0x627bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x627c8 | load | 4 | `abs 0x6285c` | `-` |
| `ldr` at 0x627cc | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x627d0 | load | 4 | `abs 0x62880` | `-` |
| `ldr` at 0x627dc | load | 4 | `abs 0x62884` | `-` |
| `ldr` at 0x627e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x627f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62800 | load | 4 | `abs 0x62884` | `-` |
| `ldr` at 0x62804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62818 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62824 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62838 | load | 4 | `abs 0x62888` | `-` |
| `ldr` at 0x6283c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62844 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x62850 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x62850 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x62850 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x620c8 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbe8c` | `[sp-20]` | `[0x132adc]` |
| `bl` at 0x62308 | `(unknown callee) 0x619dc` | `0xfbebc` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x6242c | `(unknown callee) 0x1943c` | `0x18` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x62448 | `(unknown callee) 0x6a0b0` | `[0x132a58]` | `[0x132a5c]` | `0x132a50` | `0x132a50` |
| `bl` at 0x62460 | `(unknown callee) 0x19610` | `[sp-24]` | `ret(0x6a0b0)` | `0x18` | `ret(0x6a0b0)` |
| `bl` at 0x625a0 | `(unknown callee) 0x1943c` | `?` | `sp-16` | `[0x132adc]` | `?` |
| `bl` at 0x625d4 | `(unknown callee) 0x19610` | `ret(0x1943c)` | `[0x132ae4]` | `?` | `?` |
| `bl` at 0x62678 | `(unknown callee) 0x61a78` | `ret(0x19610)` | `?` | `?` | `[sp-120]+1` |
| `bl` at 0x626c4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbed4` | `[sp-20]` | `0x12eeb8` |
| `bl` at 0x62760 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbee0` | `?` | `?` |
| `bl` at 0x62780 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbe88` | `[0x132adc]` | `0x12eeb8` |
| `bl` at 0x627b0 | `(unknown callee) 0xaa424` | `0x132a48` | `sp-28` | `0x0` | `0x0` |
| `bl` at 0x627d4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfbef0` | `?` | `0x12eeb8` |
| `bl` at 0x62828 | `(unknown callee) 0xa9d18` | `?` | `ret(0xaa424)` | `0x29a550` | `?` |
| `bl` at 0x62840 | `(unknown callee) 0x1e958` | `0xfbf24` | `[sp-20]` | `?` | `ret(0xa9d18)` |
| `bl` at 0x62848 | `(unknown callee) 0xa8b98` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x62094 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x620ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62318 | `[0x132a60] == 0x52` | `[0x132a60] != 0x52` |
| `beq` at 0x62328 | `[0x132a60] == 0x44` | `[0x132a60] != 0x44` |
| `beq` at 0x62360 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x62394 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x62694 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x626b0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x626e0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x627c4 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `beq` at 0x627fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62834 | `ret(0xa9d18) == 0x0` | `ret(0xa9d18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x62850 | `r0` | `?` |

## `L1CfgPortConfigure`

- `L1CfgPortConfigure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x643b8 to 0x64c54.
  Established. The byte through entry `r0 +0x24` is read (`ldrb` at 0x643d4).
  An included absolute word assignment writes `[0x132784]+1` to `0x132784` (`str` at 0x64538).
  The unnamed `0x66a10` call at `0x64a84` carries `[sp-148]`, `[sp-144]`, `sp-156`, `0x0` (`bl` at 0x64a84).
  Port configuration field semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x643c8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x643cc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x643d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x643d4 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x643e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x643e8 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x643ec | store | 4 | `sp -0xa8` | `[r0+40]` |
| `ldr` at 0x643f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x64400 | store | 4 | `sp -0xa8` | `ret(0xaadf4)` |
| `ldr` at 0x64404 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x64408 | store | 4 | `sp -0x2c` | `[sp-168]` |
| `ldr` at 0x6440c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64410 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x64420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64424 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x64428 | store | 4 | `sp -0xac` | `[[sp-24]+48]` |
| `ldr` at 0x64430 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6443c | store | 4 | `sp -0xac` | `ret(0xaadf4)` |
| `ldr` at 0x64440 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x64444 | store | 4 | `sp -0xa0` | `[sp-172]` |
| `ldr` at 0x64448 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x64454 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64458 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x6445c | store | 4 | `sp -0xb0` | `[[sp-24]+32]:1` |
| `ldr` at 0x64460 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6446c | load | 4 | `abs 0x64c20` | `-` |
| `str` at 0x64470 | store | 4 | `sp -0xb4` | `0xfc21c` |
| `ldr` at 0x64478 | load | 4 | `abs 0x64c24` | `-` |
| `str` at 0x6447c | store | 4 | `sp -0xb4` | `0xfc224` |
| `ldr` at 0x64480 | load | 4 | `abs 0x64c28` | `-` |
| `ldr` at 0x64484 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x64488 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6449c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x644a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x644ac | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x644b0 | store | 4 | `sp -0x24` | `[[sp-24]+12]` |
| `str` at 0x644b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x644bc | load | 4 | `abs 0x64c2c` | `-` |
| `ldr` at 0x644c4 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x644c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x644e4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x644ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x644f8 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x644fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64518 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x64524 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x64528 | load | 4 | `abs 0x64c34` | `-` |
| `ldr` at 0x6452c | load | 4 | `abs 0x64c34` | `-` |
| `ldr` at 0x64530 | load | 4 | `abs 0x132784` | `-` |
| `str` at 0x64538 | store | 4 | `abs 0x132784` | `[0x132784]+1` |
| `ldr` at 0x6453c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6454c | load | 4 | `abs 0x64c38` | `-` |
| `ldr` at 0x64550 | load | 4 | `abs 0x64c38` | `-` |
| `ldr` at 0x64554 | load | 4 | `abs 0x132788` | `-` |
| `str` at 0x6455c | store | 4 | `abs 0x132788` | `[0x132788]+1` |
| `ldr` at 0x64560 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64564 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x64570 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64574 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64590 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x6459c | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x645a0 | load | 4 | `abs 0x64c3c` | `-` |
| `ldr` at 0x645a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x645a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x645ac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x645b4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x645b8 | load | 2 | `[sp-24] +0x22` | `-` |
| `ldr` at 0x645c4 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x645c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x645e4 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x645f0 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x645f4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x645f8 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0x645fc | store | 4 | `sp -0x28` | `[[sp-24]+34]:2` |
| `ldr` at 0x64600 | load | 4 | `abs 0x64c40` | `-` |
| `ldr` at 0x64604 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x64614 | load | 4 | `abs 0x64c44` | `-` |
| `str` at 0x64618 | store | 4 | `sp -0xb8` | `0x12eeb8` |
| `ldr` at 0x6461c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64628 | load | 4 | `abs 0x64c48` | `-` |
| `str` at 0x6462c | store | 4 | `sp -0xbc` | `0xfc260` |
| `ldr` at 0x64634 | load | 4 | `abs 0x64c4c` | `-` |
| `str` at 0x64638 | store | 4 | `sp -0xbc` | `0xfc268` |
| `ldr` at 0x6463c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x64644 | store | 4 | `sp -0xc0` | `[sp-36]+64` |
| `ldr` at 0x64648 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64654 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6465c | store | 4 | `sp -0xc4` | `[sp-40]+64` |
| `str` at 0x64668 | store | 4 | `sp -0xc4` | `0x3f` |
| `ldr` at 0x6466c | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x64670 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x64674 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x64678 | store | 4 | `sp -0xd8` | `[sp-44]` |
| `ldr` at 0x6467c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x64680 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x64684 | load | 4 | `abs 0x64c50` | `-` |
| `ldr` at 0x64688 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x6468c | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x64694 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64698 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x646b4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x646d4 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x646d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x646f4 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x646fc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x64700 | store | 4 | `(unknown)` | `[sp-148]` |
| `ldr` at 0x64704 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64724 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6472c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x64730 | store | 4 | `(unknown)` | `[sp-144]` |
| `ldr` at 0x64734 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64754 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6475c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x64760 | store | 4 | `(unknown)` | `[sp-140]` |
| `ldr` at 0x64764 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64768 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64784 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6478c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x64794 | store | 4 | `(unknown)` | `ret(0x6a390)` |
| `ldr` at 0x64798 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x6479c | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x647a8 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x647ac | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x647b8 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x647bc | load | 4 | `abs 0x132a60` | `-` |
| `ldr` at 0x647cc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x647d8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x647e4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x647f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64800 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64804 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64820 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64828 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64830 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x6483c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x64840 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64844 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64860 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64868 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6486c | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0x64870 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64874 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64894 | load | 4 | `abs 0x132ad8` | `-` |
| `ldrb` at 0x648a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x648ac | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x648b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x648cc | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x648d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x648f0 | store | 4 | `sp -0xa4` | `ret(0xa1de8)` |
| `ldr` at 0x648f8 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x648fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64918 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64924 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6493c | store | 4 | `sp -0xa4` | `ret(0xa1de8)` |
| `ldr` at 0x64940 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64944 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64960 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64968 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64974 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64978 | load | 4 | `abs 0x132ac0` | `-` |
| `ldr` at 0x64988 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x6498c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x649a8 | load | 4 | `abs 0x132ad8` | `-` |
| `str` at 0x649b4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x649b8 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x649bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x649d8 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x649e4 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x649e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64a04 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64a10 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64a14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64a30 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64a3c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x64a40 | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x64a44 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x64a48 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `ldr` at 0x64a4c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x64a50 | store | 4 | `sp -0xd4` | `[sp-68]` |
| `str` at 0x64a58 | store | 4 | `sp -0xd0` | `0x0` |
| `str` at 0x64a60 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x64a68 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64a74 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64a78 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x64a8c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x64a90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x64a94 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x64a98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x64aa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64aa8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x64abc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x64acc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64ad0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x64ae4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64ae8 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x64af0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64af4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64afc | store | 4 | `sp -0xc8` | `ret(0xaadf4)` |
| `ldr` at 0x64b08 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x64b14 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x64b24 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64b28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64b44 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64b50 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64b54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64b70 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64b7c | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x64b80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64b9c | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x64ba8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x64bac | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x64bb0 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x64bb4 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `str` at 0x64bbc | store | 4 | `sp -0xd4` | `0x0` |
| `str` at 0x64bc4 | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0x64bcc | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x64bd4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64be0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x64be4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x64bf4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64bfc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x64c10 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x64c1c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x64c1c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64c1c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64c1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x643fc | `(unknown callee) 0xaadf4` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x64438 | `(unknown callee) 0xaadf4` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x6448c | `(unknown callee) 0x221f0` | `0xfc230` | `[[sp-24]+32]:1` | `[sp-180]` | `?` |
| `bl` at 0x644a4 | `(unknown callee) 0x674d0` | `sp-156` | `[sp-172]` | `0x70` | `sp-156` |
| `bl` at 0x644c0 | `(unknown callee) 0x619dc` | `0xfc24c` | `?` | `?` | `0x0` |
| `bl` at 0x64690 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc270` | `[sp-188]` | `[sp-36]+64` |
| `bl` at 0x646d0 | `(unknown callee) 0x19610` | `?` | `sp-156` | `0x70` | `?` |
| `bl` at 0x64790 | `(unknown callee) 0x6a390` | `[sp-140]` | `0x132a50` | `?` | `[0x132ad8]` |
| `bl` at 0x648e8 | `(unknown callee) 0xa1de8` | `?` | `[sp-36]+16384` | `?` | `[sp-36]+16384` |
| `bl` at 0x64934 | `(unknown callee) 0xa1de8` | `?` | `[sp-36]+8192` | `?` | `[sp-36]+8192` |
| `bl` at 0x64a6c | `(unknown callee) 0x66694` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x64a84 | `(unknown callee) 0x66a10` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x64af8 | `(unknown callee) 0xaadf4` | `[sp-24]` | `?` | `[sp-24]` | `?` |
| `bl` at 0x64b10 | `(unknown callee) 0x674d0` | `sp-156` | `?` | `0x70` | `sp-156` |
| `bl` at 0x64bd8 | `(unknown callee) 0x66694` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x64bf0 | `(unknown callee) 0x66a10` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x64c04 | `(unknown callee) 0x62e24` | `?` | `?` | `[[sp-24]+32]:1` | `?` |
| `bl` at 0x64c0c | `(unknown callee) 0x61a78` | `ret(0xa1de8)` | `0x132a50` | `?` | `[0x132ac0]` |
| `bl` at 0x64c14 | `(unknown callee) 0x62048` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x643e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6441c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x64450 | `[sp-172] != 0x0` | `[sp-172] == 0x0` |
| `beq` at 0x64468 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x644f4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x64544 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `bne` at 0x6456c | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |
| `beq` at 0x645c0 | `[[sp-24]+34]:2 == 0x0` | `[[sp-24]+34]:2 != 0x0` |
| `beq` at 0x64610 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x64624 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x64650 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x647a4 | `[0x132a60] == 0x52` | `[0x132a60] != 0x52` |
| `beq` at 0x647b4 | `[0x132a60] == 0x44` | `[0x132a60] != 0x44` |
| `beq` at 0x647c4 | `[0x132a60] == 0x40` | `[0x132a60] != 0x40` |
| `beq` at 0x647d4 | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0x647e0 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0x647ec | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `beq` at 0x647fc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x648a8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x64970 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x64980 | `[0x132ac0] != 0x0` | `[0x132ac0] == 0x0` |
| `ble` at 0x64aa0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x64ac8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x64b1c | `[sp-156] != 0x0` | `[sp-156] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64c1c | `r0` | `?` |

## `B2BEventRing`

- `B2BEventRing`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e5e0 to 0x6ee64.
  Established. The signed condition `[r0+28] >= 0x0` takes `0x6e6a4`, while negative values fall through (`bge` at 0x6e66c).
  An included word store writes `[0x132ef8]` to an unknown effective target (`str` at 0x6e718).
  The unnamed `0xa9c40` call carries `0x29a550`, `[sp-20]`, unknown, unknown (`bl` at 0x6e77c).
  Ring event forwarding policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6e5f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6e5f8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6e5fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e600 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x6e604 | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x6e608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e60c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6e61c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6e620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e624 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6e630 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6e634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e638 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6e648 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x6e64c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e650 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6e65c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x6e660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e664 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6e670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e674 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6e67c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x6e680 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e688 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x6e68c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e69c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x6e6a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e6a8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6e6b0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x6e6b4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e6c4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x6e6c8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e6cc | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x6e6d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6e6dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6e6e8 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6e6ec | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e6fc | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6e700 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e704 | load | 4 | `abs 0x6ee30` | `-` |
| `ldr` at 0x6e70c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e710 | load | 4 | `abs 0x6ee34` | `-` |
| `ldr` at 0x6e714 | load | 4 | `abs 0x132ef8` | `-` |
| `str` at 0x6e718 | store | 4 | `(unknown)` | `[0x132ef8]` |
| `ldr` at 0x6e720 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6e724 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e734 | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6e738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e73c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6e74c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e750 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6e75c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e760 | load | 4 | `abs 0x6ee38` | `-` |
| `ldr` at 0x6e768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e774 | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6e778 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e780 | load | 4 | `abs 0x6ee40` | `-` |
| `ldr` at 0x6e784 | load | 4 | `abs 0x132f04` | `-` |
| `ldr` at 0x6e788 | load | 4 | `[0x132f04] +0x58` | `-` |
| `ldr` at 0x6e794 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6e7a0 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6e7a4 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e7b4 | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6e7b8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e7bc | load | 4 | `abs 0x6ee44` | `-` |
| `ldr` at 0x6e7c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e7d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e7e0 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6e7e4 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e7f4 | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6e7f8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e7fc | load | 4 | `abs 0x6ee48` | `-` |
| `ldr` at 0x6e800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e808 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6e818 | store | 4 | `sp -0x44` | `0x2` |
| `str` at 0x6e824 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x6e828 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x6e82c | store | 4 | `sp -0x1c` | `[sp-68]` |
| `ldr` at 0x6e830 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6e83c | store | 4 | `sp -0x38` | `ret(0x6a328)` |
| `ldr` at 0x6e840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e854 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e858 | load | 4 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x6e864 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e868 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e874 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e878 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e884 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e888 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e894 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e898 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e8a4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e8a8 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e8b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e8b8 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e8c4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e8c8 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e8d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e8d8 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e8e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e8e8 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e8f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e8f8 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e904 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e908 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e914 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e918 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e924 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e928 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6e938 | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6e93c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e950 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e95c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6e968 | store | 4 | `sp -0x20` | `ret(0xa9d18)` |
| `ldr` at 0x6e96c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e97c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e988 | load | 4 | `abs 0x6ee4c` | `-` |
| `ldr` at 0x6e98c | load | 4 | `abs 0x13277c` | `-` |
| `ldr` at 0x6e998 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6e99c | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e9ac | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6e9b0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e9b4 | load | 4 | `abs 0x6ee50` | `-` |
| `ldr` at 0x6e9bc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6e9cc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x6e9d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e9dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e9f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e9fc | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6ea00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ea14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ea20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ea34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ea40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6ea48 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x6ea50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6ea60 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x6ea64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ea70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ea84 | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6ea88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ea9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6eaa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6eac0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6eac8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x6ead0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6eae0 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x6eae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6eaf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6eb04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6eb10 | store | 4 | `sp -0x38` | `ret(0x6a328)` |
| `ldr` at 0x6eb14 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6eb18 | load | 4 | `ret(0x6a328) +0x4` | `-` |
| `ldr` at 0x6eb24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6eb28 | load | 4 | `ret(0x6a328) +0x78` | `-` |
| `ldr` at 0x6eb34 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6eb38 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6eb48 | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6eb4c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6eb50 | load | 4 | `abs 0x6ee54` | `-` |
| `ldr` at 0x6eb54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6eb5c | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6eb60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6eb74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6eb80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6eb8c | store | 4 | `sp -0x20` | `ret(0xa9d18)` |
| `ldr` at 0x6eb90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6eba0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ebac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6ebb4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x6ebc0 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x6ebc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ebd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ebe4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6ebf0 | store | 4 | `sp -0x38` | `ret(0x6a328)` |
| `ldr` at 0x6ebf4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ebf8 | load | 4 | `ret(0x6a328) +0x4` | `-` |
| `ldr` at 0x6ec04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ec08 | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `ldr` at 0x6ec14 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ec18 | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `ldr` at 0x6ec24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ec28 | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `ldr` at 0x6ec34 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ec38 | load | 4 | `ret(0x6a328) +0x10` | `-` |
| `ldr` at 0x6ec48 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6ec4c | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6ec5c | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6ec60 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6ec64 | load | 4 | `abs 0x6ee58` | `-` |
| `ldr` at 0x6ec68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ec70 | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6ec74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ec88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ec94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6eca0 | store | 4 | `sp -0x20` | `ret(0xa9d18)` |
| `ldr` at 0x6eca4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6ecb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ecc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6ecc8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x6ecd0 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6ecd4 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6ece4 | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6ece8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6ecec | load | 4 | `abs 0x6ee5c` | `-` |
| `str` at 0x6ecf8 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x6ecfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ed08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ed20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6ed38 | store | 4 | `sp -0x38` | `ret(0x6a328)` |
| `str` at 0x6ed48 | store | 4 | `sp -0x3c` | `ret(0x6a328)` |
| `ldr` at 0x6ed4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6ed50 | load | 4 | `ret(0x6a328) +0x4` | `-` |
| `ldr` at 0x6ed5c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ed60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6ed64 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x6ed68 | load | 4 | `ret(0x6a328) +0x8` | `-` |
| `ldr` at 0x6ed74 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6ed78 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6ed7c | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0x6ed80 | load | 4 | `ret(0x6a328) +0xc` | `-` |
| `ldr` at 0x6ed90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ed9c | load | 4 | `abs 0x6ee3c` | `-` |
| `ldr` at 0x6eda0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6edb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6edc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6edd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ede0 | load | 4 | `abs 0x6ee28` | `-` |
| `ldr` at 0x6ede4 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6edf4 | load | 4 | `abs 0x6ee2c` | `-` |
| `ldr` at 0x6edf8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6edfc | load | 4 | `abs 0x6ee60` | `-` |
| `ldr` at 0x6ee00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ee08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6ee10 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x6ee18 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6ee24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6ee24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6ee24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6e708 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcff8` | `r2` | `0x12eeb8` |
| `bl` at 0x6e764 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd010` | `?` | `?` |
| `bl` at 0x6e770 | `(unknown callee) 0xa6eb0` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x6e77c | `(unknown callee) 0xa9c40` | `0x29a550` | `[sp-20]` | `?` | `?` |
| `bl` at 0x6e7c0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcdc8` | `?` | `0x12eeb8` |
| `bl` at 0x6e7c8 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e804 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd034` | `[sp-28]` | `0x12eeb8` |
| `bl` at 0x6e834 | `(unknown callee) 0x6a328` | `[sp-68]` | `?` | `?` | `[sp-68]` |
| `bl` at 0x6e844 | `(unknown callee) 0x6f4d4` | `[sp-28]` | `?` | `?` | `ret(0x6a328)` |
| `bl` at 0x6e960 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6e980 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e9b8 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd050` | `?` | `0x12eeb8` |
| `bl` at 0x6e9e0 | `(unknown callee) 0x6f4d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x6e9f8 | `(unknown callee) 0xa6eb0` | `[sp-20]` | `0x1` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x6ea24 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6ea3c | `(unknown callee) 0xa6eb0` | `[sp-20]` | `?` | `?` | `ret(0xa9d18)` |
| `bl` at 0x6ea54 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6ea74 | `(unknown callee) 0x6f4d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x6eaac | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6ead4 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6eaf4 | `(unknown callee) 0x6f4d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x6eb08 | `(unknown callee) 0x6a328` | `[sp-28]` | `?` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x6eb58 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd07c` | `[sp-28]` | `0x12eeb8` |
| `bl` at 0x6eb84 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6eba4 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `ret(0xa9d18)` |
| `bl` at 0x6ebd4 | `(unknown callee) 0x6f4d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x6ebe8 | `(unknown callee) 0x6a328` | `[sp-28]` | `?` | `?` | `ret(0x6f4d4)` |
| `bl` at 0x6ec6c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd0a0` | `[sp-28]` | `0x12eeb8` |
| `bl` at 0x6ec98 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6ecb8 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `ret(0xa9d18)` |
| `bl` at 0x6ecf0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd0c4` | `?` | `0x12eeb8` |
| `bl` at 0x6ed0c | `(unknown callee) 0x6f4d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x6ed30 | `(unknown callee) 0x6a328` | `0x1` | `?` | `?` | `[sp-28]` |
| `bl` at 0x6ed40 | `(unknown callee) 0x6a328` | `0x2` | `?` | `?` | `ret(0x6a328)` |
| `bl` at 0x6ed98 | `(unknown callee) 0xa6eb0` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x6edc4 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6eddc | `(unknown callee) 0xa6eb0` | `[sp-20]` | `?` | `?` | `ret(0xa9d18)` |
| `bl` at 0x6ee04 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfd0e0` | `[sp-28]` | `0x12eeb8` |
| `bl` at 0x6ee1c | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x6e66c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x6e6d8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x6e6e4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x6e6f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e730 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e790 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6e79c | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x6e7b0 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x6e7dc | `[sp-28]-1 > 0x1 (unsigned, 32-bit)` | `[sp-28]-1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x6e7f0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e810 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `beq` at 0x6e850 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x6e860 | `[[sp-56]+4] == 0x0` | `[[sp-56]+4] != 0x0` |
| `beq` at 0x6e870 | `[[sp-56]+16] == 0x35` | `[[sp-56]+16] != 0x35` |
| `beq` at 0x6e880 | `[[sp-56]+16] == 0x4f` | `[[sp-56]+16] != 0x4f` |
| `beq` at 0x6e890 | `[[sp-56]+16] == 0x42` | `[[sp-56]+16] != 0x42` |
| `beq` at 0x6e8a0 | `[[sp-56]+16] == 0x46` | `[[sp-56]+16] != 0x46` |
| `beq` at 0x6e8b0 | `[[sp-56]+16] == 0x4a` | `[[sp-56]+16] != 0x4a` |
| `beq` at 0x6e8c0 | `[[sp-56]+16] == 0x4b` | `[[sp-56]+16] != 0x4b` |
| `beq` at 0x6e8d0 | `[[sp-56]+16] == 0x21` | `[[sp-56]+16] != 0x21` |
| `beq` at 0x6e8e0 | `[[sp-56]+16] == 0x5e` | `[[sp-56]+16] != 0x5e` |
| `beq` at 0x6e8f0 | `[[sp-56]+16] == 0x2b` | `[[sp-56]+16] != 0x2b` |
| `beq` at 0x6e900 | `[[sp-56]+16] == 0x4c` | `[[sp-56]+16] != 0x4c` |
| `beq` at 0x6e910 | `[[sp-56]+16] == 0x5a` | `[[sp-56]+16] != 0x5a` |
| `beq` at 0x6e920 | `[[sp-56]+16] == 0x7c` | `[[sp-56]+16] != 0x7c` |
| `beq` at 0x6e930 | `[[sp-56]+16] == 0x4d` | `[[sp-56]+16] != 0x4d` |
| `bne` at 0x6e974 | `ret(0xa9d18) != 0x0` | `ret(0xa9d18) == 0x0` |
| `ble` at 0x6e994 | `[0x13277c] <= 0x0 (signed, 32-bit)` | `[0x13277c] > 0x0 (signed, 32-bit)` |
| `beq` at 0x6e9a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e9c4 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `bgt` at 0x6e9d8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x6e9ec | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x6ea30 | `ret(0xa9d18) == 0x0` | `ret(0xa9d18) != 0x0` |
| `bgt` at 0x6ea6c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x6ea80 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `bne` at 0x6eab8 | `ret(0xa9d18) != 0x0` | `ret(0xa9d18) == 0x0` |
| `bgt` at 0x6eaec | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x6eb00 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x6eb20 | `[ret(0x6a328)+4] == 0x0` | `[ret(0x6a328)+4] != 0x0` |
| `bne` at 0x6eb30 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6eb44 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6eb98 | `ret(0xa9d18) != 0x0` | `ret(0xa9d18) == 0x0` |
| `bgt` at 0x6ebcc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x6ebe0 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x6ec00 | `[ret(0x6a328)+4] == 0x0` | `[ret(0x6a328)+4] != 0x0` |
| `beq` at 0x6ec10 | `[ret(0x6a328)+16] == 0x52` | `[ret(0x6a328)+16] != 0x52` |
| `beq` at 0x6ec20 | `[ret(0x6a328)+16] == 0x44` | `[ret(0x6a328)+16] != 0x44` |
| `beq` at 0x6ec30 | `[ret(0x6a328)+16] == 0x36` | `[ret(0x6a328)+16] != 0x36` |
| `beq` at 0x6ec40 | `[ret(0x6a328)+16] == 0x40` | `[ret(0x6a328)+16] != 0x40` |
| `beq` at 0x6ec58 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6ecac | `ret(0xa9d18) != 0x0` | `ret(0xa9d18) == 0x0` |
| `beq` at 0x6ece0 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x6ed04 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x6ed18 | `ret(0x6f4d4) != 0x0` | `ret(0x6f4d4) == 0x0` |
| `bne` at 0x6ed28 | `[sp-28] != 0x2` | `[sp-28] == 0x2` |
| `beq` at 0x6ed58 | `[ret(0x6a328)+4] == 0x0` | `[ret(0x6a328)+4] != 0x0` |
| `bne` at 0x6ed70 | `[[sp-56]+8] != [ret(0x6a328)+8]` | `[[sp-56]+8] == [ret(0x6a328)+8]` |
| `bne` at 0x6ed88 | `[[sp-56]+12] != [ret(0x6a328)+12]` | `[[sp-56]+12] == [ret(0x6a328)+12]` |
| `beq` at 0x6edd0 | `ret(0xa9d18) == 0x0` | `ret(0xa9d18) != 0x0` |
| `beq` at 0x6edf0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6ee24 | `r0` | `?` |

## `B2BEventMesh`

- `B2BEventMesh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6db58 to 0x6e5e0.
  Established. The signed condition `[r0+28] >= 0x0` takes `0x6dbc4`, while negative values fall through (`bge` at 0x6db8c).
  An included absolute word store writes `[sp-20]` to `0x133c1c` (`str` at 0x6de44).
  Another clears the word through `[sp-20] +0x18` (`str` at 0x6dea4).
  Mesh event routing meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6db68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6db70 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6db74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6db78 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x6db7c | store | 4 | `sp -0x20` | `[r0+12]` |
| `ldr` at 0x6db80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6db84 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6db90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6db94 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6db9c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x6dba0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x6dba8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x6dbac | load | 4 | `sp -0x50` | `-` |
| `str` at 0x6dbbc | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x6dbc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dbc8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dbd0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x6dbd4 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x6dbe4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x6dbe8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x6dbec | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0x6dbf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dbf4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6dc00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dc04 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dc0c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x6dc10 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6dc18 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x6dc1c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6dc2c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x6dc34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dc38 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dc40 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x6dc44 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6dc54 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x6dc58 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x6dc5c | store | 4 | `sp -0x28` | `[sp-84]` |
| `ldr` at 0x6dc60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dc64 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6dc70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dc74 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dc7c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6dc80 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x6dc88 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6dc8c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x6dc9c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6dca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dca8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dcb0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6dcb4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x6dcc4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6dcc8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x6dccc | store | 4 | `sp -0x2c` | `[sp-88]` |
| `ldr` at 0x6dcd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dcd4 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6dce4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x6dce8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dcec | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6dcf8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x6dcfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dd00 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x6dd0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dd10 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6dd24 | store | 4 | `sp -0x5c` | `0xe` |
| `str` at 0x6dd30 | store | 4 | `sp -0x5c` | `0xf` |
| `ldr` at 0x6dd38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dd3c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6dd44 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x6dd48 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x6dd50 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x6dd54 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x6dd58 | store | 4 | `sp -0x38` | `[sp-92]` |
| `ldr` at 0x6dd5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dd60 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dd70 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x6dd74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dd78 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6dd84 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x6dd8c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x6dd90 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6dd94 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6dda4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dda8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6ddb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ddb8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6ddd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ddd4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6dde4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dde8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6ddf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ddf8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6de10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6de14 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x6de28 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6de2c | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6de30 | load | 4 | `abs 0x132efc` | `-` |
| `str` at 0x6de38 | store | 4 | `abs 0x132efc` | `?` |
| `ldr` at 0x6de3c | load | 4 | `abs 0x6e57c` | `-` |
| `ldr` at 0x6de40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6de44 | store | 4 | `abs 0x133c1c` | `[sp-20]` |
| `ldr` at 0x6de48 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6de4c | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6de5c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6de60 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6de64 | load | 4 | `abs 0x6e584` | `-` |
| `ldr` at 0x6de68 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6de6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x6de74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6de84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6de9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6dea4 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `str` at 0x6deac | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x6deb4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x6deb8 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6debc | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6decc | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6ded0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6ded4 | load | 4 | `abs 0x6e588` | `-` |
| `ldr` at 0x6ded8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x6dedc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x6dee4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6def0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6defc | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0x6df04 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6df08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6df14 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6df20 | load | 4 | `abs 0x6e58c` | `-` |
| `ldr` at 0x6df24 | load | 4 | `abs 0x132f04` | `-` |
| `ldr` at 0x6df28 | load | 4 | `[0x132f04] +0x58` | `-` |
| `ldr` at 0x6df34 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6df40 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6df44 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6df54 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6df58 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6df5c | load | 4 | `abs 0x6e590` | `-` |
| `ldr` at 0x6df64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6df8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6df98 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6df9c | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6dfac | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dfb0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6dfb4 | load | 4 | `abs 0x6e594` | `-` |
| `ldr` at 0x6dfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dfc8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6dfd8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6dff0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x6dff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e010 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e014 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e024 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e028 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6e02c | store | 4 | `sp -0x7c` | `[sp-32]` |
| `ldr` at 0x6e030 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e034 | load | 4 | `abs 0x6e598` | `-` |
| `ldr` at 0x6e038 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6e03c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6e044 | load | 4 | `abs 0x6e59c` | `-` |
| `ldr` at 0x6e048 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6e04c | load | 4 | `abs 0x132f08` | `-` |
| `ldr` at 0x6e058 | load | 4 | `abs 0x6e5a0` | `-` |
| `ldr` at 0x6e05c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6e060 | load | 4 | `abs 0x132f0c` | `-` |
| `ldr` at 0x6e06c | load | 4 | `abs 0x6e5a4` | `-` |
| `str` at 0x6e070 | store | 4 | `sp -0x4c` | `0x299868` |
| `ldr` at 0x6e074 | load | 4 | `abs 0x6e58c` | `-` |
| `ldr` at 0x6e078 | load | 4 | `abs 0x132f04` | `-` |
| `ldr` at 0x6e07c | load | 4 | `[0x132f04] +0x58` | `-` |
| `ldr` at 0x6e0a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e0b0 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e0b4 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e0c4 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e0c8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e0cc | load | 4 | `abs 0x6e5a8` | `-` |
| `ldr` at 0x6e0d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e0e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6e0f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6e108 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x6e10c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e12c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e130 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e13c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e140 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e15c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e170 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e174 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e184 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e18c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e19c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e1a0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e1ac | load | 4 | `sp -0x48` | `-` |
| `str` at 0x6e1b0 | store | 4 | `sp -0x7c` | `[sp-72]` |
| `ldr` at 0x6e1b4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e1b8 | load | 4 | `abs 0x6e5ac` | `-` |
| `ldr` at 0x6e1c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6e1d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e1dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e1e8 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e1ec | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e1fc | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e204 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e214 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e218 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e224 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e228 | load | 4 | `abs 0x6e5b0` | `-` |
| `ldr` at 0x6e230 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6e234 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6e240 | store | 4 | `sp -0x1c` | `ret(0xa9d18)` |
| `ldr` at 0x6e244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e250 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6e25c | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e260 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e27c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e280 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e284 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x6e288 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e28c | load | 4 | `abs 0x6e5b4` | `-` |
| `ldr` at 0x6e29c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e2a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e2b4 | load | 4 | `abs 0x6e5b8` | `-` |
| `ldr` at 0x6e2bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6e2c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6e2cc | store | 4 | `sp -0x18` | `ret(0x6a0b0)` |
| `ldr` at 0x6e2d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6e2e0 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e2e4 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e2f4 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e2f8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e2fc | load | 4 | `abs 0x6e5bc` | `-` |
| `ldr` at 0x6e300 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6e304 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x6e30c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6e31c | load | 4 | `ret(0x6a0b0) +0x10` | `-` |
| `str` at 0x6e320 | store | 4 | `sp -0x20` | `[ret(0x6a0b0)+16]` |
| `ldr` at 0x6e328 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e33c | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e340 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e350 | load | 4 | `abs 0x6e580` | `-` |
| `str` at 0x6e354 | store | 4 | `sp -0x60` | `0x12eeb8` |
| `ldr` at 0x6e358 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6e364 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6e370 | load | 4 | `abs 0x6e5c0` | `-` |
| `str` at 0x6e374 | store | 4 | `sp -0x64` | `0xfcf2c` |
| `ldr` at 0x6e37c | load | 4 | `abs 0x6e5c4` | `-` |
| `str` at 0x6e380 | store | 4 | `sp -0x64` | `0xfcf30` |
| `ldr` at 0x6e388 | load | 4 | `abs 0x6e5c8` | `-` |
| `str` at 0x6e38c | store | 4 | `sp -0x64` | `0xfcf34` |
| `ldr` at 0x6e390 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6e394 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x6e398 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6e39c | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x6e3a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6e3a4 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x6e3a8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e3ac | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x6e3b0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x6e3b4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e3b8 | load | 4 | `abs 0x6e5cc` | `-` |
| `ldr` at 0x6e3bc | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x6e3c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e3c8 | load | 4 | `abs 0x6e5d0` | `-` |
| `ldr` at 0x6e3cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e3e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e3ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6e3f8 | store | 4 | `sp -0x1c` | `ret(0xa9d18)` |
| `ldr` at 0x6e3fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e408 | load | 4 | `abs 0x6e5d0` | `-` |
| `ldr` at 0x6e40c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e420 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e42c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e438 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e43c | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e44c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e458 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e45c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e460 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x6e464 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e468 | load | 4 | `abs 0x6e5d4` | `-` |
| `ldr` at 0x6e478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e484 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e488 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e498 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e49c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6e4a0 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x6e4a4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6e4a8 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x6e4ac | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6e4b0 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x6e4b4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e4b8 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x6e4bc | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e4c0 | load | 4 | `abs 0x6e5d8` | `-` |
| `ldr` at 0x6e4c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e4c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e4d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e4dc | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e4e0 | load | 4 | `abs 0x132efc` | `-` |
| `ldr` at 0x6e4f0 | load | 4 | `abs 0x6e580` | `-` |
| `str` at 0x6e4f4 | store | 4 | `sp -0x68` | `0x12eeb8` |
| `ldr` at 0x6e4f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6e504 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6e510 | load | 4 | `abs 0x6e5c0` | `-` |
| `str` at 0x6e514 | store | 4 | `sp -0x6c` | `0xfcf2c` |
| `ldr` at 0x6e51c | load | 4 | `abs 0x6e5c4` | `-` |
| `str` at 0x6e520 | store | 4 | `sp -0x6c` | `0xfcf30` |
| `ldr` at 0x6e528 | load | 4 | `abs 0x6e5c8` | `-` |
| `str` at 0x6e52c | store | 4 | `sp -0x6c` | `0xfcf34` |
| `ldr` at 0x6e530 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6e534 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x6e538 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x6e53c | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x6e540 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6e544 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x6e548 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6e54c | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x6e550 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x6e554 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6e558 | load | 4 | `abs 0x6e5dc` | `-` |
| `ldr` at 0x6e55c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x6e560 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6e568 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6e574 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6e574 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6e574 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6de70 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcd7c` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x6de88 | `(unknown callee) 0x6a400` | `?` | `?` | `?` | `?` |
| `bl` at 0x6dee0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcd9c` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x6def4 | `(unknown callee) 0x68070` | `[sp-20]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x6df60 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcdc8` | `?` | `0x12eeb8` |
| `bl` at 0x6df68 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6df88 | `(unknown callee) 0x6a2ac` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x6dfb8 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcde0` | `?` | `0x12eeb8` |
| `bl` at 0x6dfc0 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e00c | `(unknown callee) 0x19610` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x6e040 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfce0c` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x6e0a0 | `(unknown callee) 0x6a2ac` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x6e0d0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfce28` | `?` | `0x12eeb8` |
| `bl` at 0x6e0d8 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e124 | `(unknown callee) 0x19610` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x6e1c0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfce5c` | `?` | `?` |
| `bl` at 0x6e1d4 | `(unknown callee) 0x681e8` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x6e1e0 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x6e22c | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfce94` | `?` | `?` |
| `bl` at 0x6e238 | `(unknown callee) 0xa9d18` | `[sp-76]` | `[sp-20]` | `?` | `?` |
| `bl` at 0x6e254 | `(unknown callee) 0x773ac` | `[sp-76]` | `?` | `?` | `ret(0xa9d18)` |
| `bl` at 0x6e274 | `(unknown callee) 0x1f8e0` | `ret(0xa9d18)` | `?` | `?` | `?` |
| `bl` at 0x6e298 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcebc` | `[[sp-20]+22]:1` | `ret(0x1f8e0)` |
| `bl` at 0x6e2a0 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e2b8 | `(unknown callee) 0x619dc` | `0xfcee8` | `?` | `?` | `[sp-32]` |
| `bl` at 0x6e2c4 | `(unknown callee) 0x6a0b0` | `[sp-48]` | `[sp-52]` | `?` | `?` |
| `bl` at 0x6e2dc | `(unknown callee) 0x61a78` | `ret(0x6a0b0)` | `?` | `?` | `ret(0x6a0b0)` |
| `bl` at 0x6e308 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcef8` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x6e310 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e324 | `(unknown callee) 0x61a78` | `ret(0x6a0b0)` | `?` | `?` | `[ret(0x6a0b0)+16]` |
| `bl` at 0x6e32c | `(unknown callee) 0x6f4d4` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x6e3c4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcf38` | `?` | `[sp-56]` |
| `bl` at 0x6e3f0 | `(unknown callee) 0xa9d18` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6e430 | `(unknown callee) 0x773ac` | `?` | `[sp-20]` | `0x29a550` | `?` |
| `bl` at 0x6e450 | `(unknown callee) 0x1f8e0` | `ret(0xa9d18)` | `?` | `?` | `?` |
| `bl` at 0x6e474 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcf68` | `[[sp-20]+22]:1` | `ret(0x1f8e0)` |
| `bl` at 0x6e47c | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e4cc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcf94` | `[sp-32]` | `[sp-56]` |
| `bl` at 0x6e4d4 | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6e564 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcfc8` | `?` | `[sp-56]` |
| `bl` at 0x6e56c | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x6db8c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x6dbfc | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x6dc6c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x6dd08 | `[r0+24] >= 0x0 (signed, 32-bit)` | `[r0+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x6dd1c | `? != 0x2` | `? == 0x2` |
| `beq` at 0x6dda0 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x6ddb0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x6ddc8 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x6dde0 | `? == 0x5` | `? != 0x5` |
| `bge` at 0x6ddf0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x6de08 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x6de20 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x6de58 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x6de80 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x6de94 | `ret(0x6a400) != 0x0` | `ret(0x6a400) == 0x0` |
| `beq` at 0x6dec8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6deec | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0x6df10 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0x6df1c | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `bne` at 0x6df30 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6df3c | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x6df50 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6df94 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x6dfa8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e020 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e054 | `[sp-48] != [0x132f08]` | `[sp-48] == [0x132f08]` |
| `bne` at 0x6e068 | `[sp-52] != [0x132f0c]` | `[sp-52] == [0x132f0c]` |
| `beq` at 0x6e084 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x6e0ac | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x6e0c0 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x6e138 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x6e150 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x6e168 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x6e180 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e1cc | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x6e1f8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e24c | `ret(0xa9d18) != 0x0` | `ret(0xa9d18) == 0x0` |
| `beq` at 0x6e26c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e2b0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6e2d8 | `ret(0x6a0b0) != 0x0` | `ret(0x6a0b0) == 0x0` |
| `beq` at 0x6e2f0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e338 | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x6e34c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e360 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x6e36c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x6e404 | `ret(0xa9d18) != 0x0` | `ret(0xa9d18) == 0x0` |
| `beq` at 0x6e448 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e494 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e4ec | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e500 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x6e50c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6e574 | `r0` | `?` |

## `B2BInit`

- `B2BInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c4e8 to 0x6cfdc.
  Established. The word at absolute `0x132ef4` receives `[0x132ef4]+1` (`str` at 0x6c534).
  Included word stores assign `0xfff`, `0x1`, and `0x0` to `0x132efc` (`str` at 0x6c540, `str` at 0x6c54c, `str` at 0x6c558).
  The computed transfer takes its unresolved edge when unsigned `[sp-52]-43 <= 0x33`, while greater values fall through (`ldrls` at 0x6c598).
  B2B initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6c4f8 | load | 4 | `abs 0x6cf54` | `-` |
| `ldr` at 0x6c4fc | load | 4 | `abs 0x132ef4` | `-` |
| `ldr` at 0x6c508 | load | 4 | `abs 0x6cf58` | `-` |
| `str` at 0x6c50c | store | 4 | `sp -0x64` | `0xfcb24` |
| `ldr` at 0x6c514 | load | 4 | `abs 0x6cf5c` | `-` |
| `ldr` at 0x6c524 | load | 4 | `abs 0x6cf54` | `-` |
| `ldr` at 0x6c528 | load | 4 | `abs 0x6cf54` | `-` |
| `ldr` at 0x6c52c | load | 4 | `abs 0x132ef4` | `-` |
| `str` at 0x6c534 | store | 4 | `abs 0x132ef4` | `[0x132ef4]+1` |
| `ldr` at 0x6c538 | load | 4 | `abs 0x6cf60` | `-` |
| `ldr` at 0x6c53c | load | 4 | `abs 0x6cf64` | `-` |
| `str` at 0x6c540 | store | 4 | `abs 0x132efc` | `0xfff` |
| `ldr` at 0x6c544 | load | 4 | `abs 0x6cf60` | `-` |
| `str` at 0x6c54c | store | 4 | `abs 0x132efc` | `0x1` |
| `ldr` at 0x6c550 | load | 4 | `abs 0x6cf60` | `-` |
| `str` at 0x6c558 | store | 4 | `abs 0x132efc` | `0x0` |
| `str` at 0x6c568 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x6c56c | load | 4 | `abs 0x6cf68` | `-` |
| `ldr` at 0x6c57c | load | 4 | `abs 0x6cf6c` | `-` |
| `ldr` at 0x6c58c | load | 4 | `sp -0x34` | `-` |
| `ldrls` at 0x6c598 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c7b0 | load | 4 | `abs 0x6cf70` | `-` |
| `ldr` at 0x6c7b4 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6c7b8 | load | 4 | `abs 0x6cf74` | `-` |
| `ldm` at 0x6cf50 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6cf50 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6cf50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6cf50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6cf50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c520 | `(unknown callee) 0x200a0` | `0x9` | `0xfcb40` | `0x66` | `0x0` |
| `bl` at 0x6c55c | `(unknown callee) 0x488cc` | `?` | `?` | `0x132efc` | `0x0` |
| `bl` at 0x6c578 | `(unknown callee) 0x19748` | `0x29a550` | `0x0` | `0x84` | `?` |
| `bl` at 0x6c588 | `(unknown callee) 0x19748` | `0x29a56c` | `?` | `0x34` | `?` |
| `bl` at 0x6c7bc | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfcb58` | `?` | `0x12eeb8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6c504 | `[0x132ef4] == 0x0` | `[0x132ef4] != 0x0` |
| `ldrls` at 0x6c598 | `[sp-52]-43 <= 0x33 (unsigned, 32-bit)` | `[sp-52]-43 > 0x33 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6cf50 | `r0` | `ret(0x222f0)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6c598).

## `FlashIRouter`

- `FlashIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98744 to 0x98fdc.
  Established. Equality `[sp-32] == 0x73` takes `0x98e70`, while inequality falls through (`beq` at 0x987a4).
  A separate comparison takes `0x98d50` for `[sp-32] == 0x74`, with inequality falling through (`beq` at 0x987cc).
  The unnamed `0xbead0` call carries `[sp-164]`, `[sp-196]`, `[[sp-20]+46]:2`, `sp-172` (`bl` at 0x98f04).
  Flash request operation meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x98754 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9875c | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x98768 | store | 4 | `sp -0x1c` | `ret(0x488cc)` |
| `ldr` at 0x9876c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x98770 | load | 2 | `[sp-20] +0x22` | `-` |
| `str` at 0x98774 | store | 4 | `sp -0x20` | `[[sp-20]+34]:2` |
| `str` at 0x9877c | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x98780 | load | 4 | `abs 0x98fc8` | `-` |
| `ldr` at 0x98784 | load | 4 | `abs 0x133a9c` | `-` |
| `ldr` at 0x98794 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x98798 | store | 4 | `sp -0xcc` | `[sp-32]` |
| `ldr` at 0x9879c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x987a8 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x987b4 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x987c4 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x987d4 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x987d8 | load | 4 | `abs 0x29c3ac` | `-` |
| `ldr` at 0x987e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x987e8 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x987f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x987f8 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x98808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9880c | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x9881c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9883c | store | 4 | `sp -0x94` | `0x5a` |
| `ldr` at 0x98840 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98844 | load | 4 | `abs 0x29c3ac` | `-` |
| `ldr` at 0x98850 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x98854 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x98860 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x98864 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x98874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98878 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x98888 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x988a8 | store | 4 | `sp -0x94` | `0x5a` |
| `strb` at 0x988b0 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0x988b8 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0x988bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x988c0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x988c8 | store | 4 | `sp -0x84` | `([sp-24] + [sp-148])` |
| `strh` at 0x988d0 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0x988d4 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x988d8 | load | 4 | `abs 0x29c3c0` | `-` |
| `strb` at 0x988e8 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x988f0 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0x988f4 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x988f8 | load | 4 | `abs 0x29c3c0` | `-` |
| `ldr` at 0x988fc | load | 4 | `[0x29c3c0] +0x0` | `-` |
| `str` at 0x98900 | store | 4 | `sp -0x7c` | `[[0x29c3c0]]` |
| `strh` at 0x98908 | store | 2 | `sp -0x7e` | `0x0` |
| `strb` at 0x98914 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x9891c | store | 1 | `sp -0x7f` | `0x0` |
| `str` at 0x98924 | store | 4 | `sp -0x7c` | `0x0` |
| `strh` at 0x9892c | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0x98930 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98934 | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0x98940 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98944 | load | 4 | `abs 0x98fcc` | `-` |
| `str` at 0x9894c | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0x98954 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0x9895c | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0x98964 | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0x98968 | load | 4 | `abs 0x29c3b0` | `-` |
| `str` at 0x98978 | store | 4 | `sp -0x90` | `ret(0x9802c)` |
| `ldr` at 0x9897c | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x9898c | store | 1 | `sp -0x78` | `0x0` |
| `strb` at 0x98994 | store | 1 | `sp -0x77` | `0x0` |
| `str` at 0x9899c | store | 4 | `sp -0x74` | `0x0` |
| `strh` at 0x989a4 | store | 2 | `sp -0x76` | `0x0` |
| `strb` at 0x989ac | store | 1 | `sp -0x70` | `0x0` |
| `strb` at 0x989b4 | store | 1 | `sp -0x6f` | `0x0` |
| `str` at 0x989bc | store | 4 | `sp -0x6c` | `0x0` |
| `strh` at 0x989c4 | store | 2 | `sp -0x6e` | `0x0` |
| `ldr` at 0x989cc | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x989d0 | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0x989dc | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x989e0 | load | 4 | `abs 0x29c3a0` | `-` |
| `str` at 0x989e8 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x989ec | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x989f0 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x98a00 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x98a08 | store | 4 | `sp -0xa4` | `(ret(0x19280) + [sp-160])` |
| `strb` at 0x98a10 | store | 1 | `sp -0x78` | `0x1` |
| `strb` at 0x98a18 | store | 1 | `sp -0x77` | `0x0` |
| `ldr` at 0x98a1c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x98a20 | store | 4 | `sp -0x74` | `(ret(0x19280) + [sp-160])` |
| `ldr` at 0x98a24 | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0x98a34 | store | 2 | `sp -0x76` | `(ret(0x19538)+1 & 0xffff)` |
| `ldr` at 0x98a38 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x98a3c | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x98a4c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x98a54 | store | 4 | `sp -0xa4` | `(ret(0x19280) + [sp-160])` |
| `strb` at 0x98a5c | store | 1 | `sp -0x70` | `0x1` |
| `strb` at 0x98a64 | store | 1 | `sp -0x6f` | `0x0` |
| `ldr` at 0x98a68 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x98a6c | store | 4 | `sp -0x6c` | `(ret(0x19280) + [sp-160])` |
| `ldr` at 0x98a70 | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0x98a80 | store | 2 | `sp -0x6e` | `(ret(0x19538)+1 & 0xffff)` |
| `ldr` at 0x98a84 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98a88 | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0x98a94 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98a98 | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0x98aa4 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98aa8 | load | 4 | `abs 0x98fcc` | `-` |
| `str` at 0x98ab0 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0x98ab8 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0x98ac0 | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0x98ac8 | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0x98acc | load | 4 | `abs 0x29c3b0` | `-` |
| `str` at 0x98adc | store | 4 | `sp -0x90` | `ret(0x9802c)` |
| `str` at 0x98ae8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x98aec | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x98afc | store | 1 | `sp -0x68` | `0x0` |
| `strb` at 0x98b04 | store | 1 | `sp -0x67` | `0x0` |
| `str` at 0x98b0c | store | 4 | `sp -0x64` | `0x0` |
| `strh` at 0x98b14 | store | 2 | `sp -0x66` | `0x0` |
| `strb` at 0x98b1c | store | 1 | `sp -0x60` | `0x0` |
| `strb` at 0x98b24 | store | 1 | `sp -0x5f` | `0x0` |
| `str` at 0x98b2c | store | 4 | `sp -0x5c` | `0x0` |
| `strh` at 0x98b34 | store | 2 | `sp -0x5e` | `0x0` |
| `ldr` at 0x98b3c | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98b40 | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0x98b4c | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98b50 | load | 4 | `abs 0x29c3a4` | `-` |
| `str` at 0x98b58 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x98b5c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x98b60 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x98b70 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x98b78 | store | 4 | `sp -0xa0` | `(ret(0x19280) + [sp-164])` |
| `strb` at 0x98b80 | store | 1 | `sp -0x68` | `0x1` |
| `strb` at 0x98b88 | store | 1 | `sp -0x67` | `0x0` |
| `ldr` at 0x98b8c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x98b90 | store | 4 | `sp -0x64` | `(ret(0x19280) + [sp-164])` |
| `ldr` at 0x98b94 | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0x98ba4 | store | 2 | `sp -0x66` | `(ret(0x19538)+1 & 0xffff)` |
| `ldr` at 0x98ba8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x98bac | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x98bbc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x98bc4 | store | 4 | `sp -0xa0` | `(ret(0x19280) + [sp-164])` |
| `strb` at 0x98bcc | store | 1 | `sp -0x60` | `0x1` |
| `strb` at 0x98bd4 | store | 1 | `sp -0x5f` | `0x0` |
| `ldr` at 0x98bd8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x98bdc | store | 4 | `sp -0x5c` | `(ret(0x19280) + [sp-164])` |
| `ldr` at 0x98be0 | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0x98bf0 | store | 2 | `sp -0x5e` | `(ret(0x19538)+1 & 0xffff)` |
| `strb` at 0x98bf8 | store | 1 | `sp -0x58` | `0x0` |
| `strb` at 0x98c00 | store | 1 | `sp -0x57` | `0x0` |
| `ldr` at 0x98c04 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98c08 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98c0c | load | 4 | `abs 0x29c3a8` | `-` |
| `ldr` at 0x98c10 | load | 4 | `abs 0x29c3a0` | `-` |
| `str` at 0x98c20 | store | 4 | `sp -0x54` | `?` |
| `strh` at 0x98c28 | store | 2 | `sp -0x56` | `0x0` |
| `strb` at 0x98c30 | store | 1 | `sp -0x50` | `0x0` |
| `strb` at 0x98c38 | store | 1 | `sp -0x4f` | `0x0` |
| `ldr` at 0x98c3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x98c40 | store | 4 | `sp -0x4c` | `[sp-28]` |
| `strh` at 0x98c48 | store | 2 | `sp -0x4e` | `0x0` |
| `strb` at 0x98c50 | store | 1 | `sp -0x48` | `0x0` |
| `strb` at 0x98c58 | store | 1 | `sp -0x47` | `0x0` |
| `ldr` at 0x98c5c | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98c60 | load | 4 | `abs 0x29c3ac` | `-` |
| `str` at 0x98c64 | store | 4 | `sp -0x44` | `[0x29c3ac]` |
| `strh` at 0x98c6c | store | 2 | `sp -0x46` | `0x0` |
| `strb` at 0x98c74 | store | 1 | `sp -0x40` | `0x0` |
| `strb` at 0x98c7c | store | 1 | `sp -0x3f` | `0x0` |
| `ldr` at 0x98c80 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98c84 | load | 4 | `abs 0x29c3a0` | `-` |
| `str` at 0x98c88 | store | 4 | `sp -0x3c` | `[0x29c3a0]` |
| `strh` at 0x98c90 | store | 2 | `sp -0x3e` | `0x0` |
| `strb` at 0x98c98 | store | 1 | `sp -0x38` | `0x0` |
| `strb` at 0x98ca0 | store | 1 | `sp -0x37` | `0x0` |
| `ldr` at 0x98ca4 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98ca8 | load | 4 | `abs 0x29c3a4` | `-` |
| `str` at 0x98cac | store | 4 | `sp -0x34` | `[0x29c3a4]` |
| `strh` at 0x98cb4 | store | 2 | `sp -0x36` | `0x0` |
| `str` at 0x98cbc | store | 4 | `sp -0x8c` | `0xb` |
| `ldr` at 0x98cc0 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98cc4 | load | 4 | `abs 0x29c3bc` | `-` |
| `ldr` at 0x98cd0 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98cd4 | load | 4 | `abs 0x29c3b8` | `-` |
| `strb` at 0x98ce4 | store | 1 | `sp -0x30` | `0x0` |
| `strb` at 0x98cec | store | 1 | `sp -0x2f` | `0x0` |
| `ldr` at 0x98cf0 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98cf4 | load | 4 | `abs 0x29c3bc` | `-` |
| `str` at 0x98cf8 | store | 4 | `sp -0x2c` | `[0x29c3bc]` |
| `strh` at 0x98d00 | store | 2 | `sp -0x2e` | `0x0` |
| `strb` at 0x98d08 | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0x98d10 | store | 1 | `sp -0x27` | `0x0` |
| `ldr` at 0x98d14 | load | 4 | `abs 0x98fcc` | `-` |
| `ldr` at 0x98d18 | load | 4 | `abs 0x29c3b8` | `-` |
| `str` at 0x98d1c | store | 4 | `sp -0x24` | `[0x29c3b8]` |
| `strh` at 0x98d24 | store | 2 | `sp -0x26` | `0x0` |
| `str` at 0x98d2c | store | 4 | `sp -0x8c` | `0xd` |
| `str` at 0x98d34 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0x98d40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d44 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x98d50 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x98d54 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x98d64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d68 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x98d6c | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0x98d74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x98d80 | store | 4 | `sp -0xb4` | `ret(0xaadf4)` |
| `ldr` at 0x98d84 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x98d88 | store | 4 | `sp -0xa4` | `[sp-180]` |
| `ldr` at 0x98d8c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x98d90 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x98da0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98da4 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x98da8 | store | 4 | `sp -0xb8` | `[[sp-20]+48]` |
| `ldr` at 0x98db0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x98dbc | store | 4 | `sp -0xb8` | `ret(0xaadf4)` |
| `ldr` at 0x98dc0 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x98dc4 | store | 4 | `sp -0xa0` | `[sp-184]` |
| `ldr` at 0x98dcc | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x98dd0 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x98de0 | store | 4 | `sp -0x90` | `ret(0xbea9c)` |
| `strb` at 0x98de8 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0x98df0 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0x98df4 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x98df8 | store | 4 | `sp -0x84` | `ret(0xbea9c)` |
| `strh` at 0x98e00 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0x98e04 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x98e14 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x98e1c | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0x98e20 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x98e24 | store | 4 | `sp -0x7c` | `[sp-168]` |
| `strh` at 0x98e2c | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0x98e30 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x98e40 | store | 4 | `sp -0xbc` | `0x2` |
| `str` at 0x98e4c | store | 4 | `sp -0xbc` | `0x1` |
| `str` at 0x98e54 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0x98e60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98e64 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x98e70 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x98e74 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x98e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98e88 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x98e8c | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0x98e94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x98ea0 | store | 4 | `sp -0xc0` | `ret(0xaadf4)` |
| `ldr` at 0x98ea4 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x98ea8 | store | 4 | `sp -0xa4` | `[sp-192]` |
| `ldr` at 0x98eac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x98eb0 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x98ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98ec4 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x98ec8 | store | 4 | `sp -0xc4` | `[[sp-20]+48]` |
| `ldr` at 0x98ed0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x98edc | store | 4 | `sp -0xc4` | `ret(0xaadf4)` |
| `ldr` at 0x98ee0 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x98ee4 | store | 4 | `sp -0xb0` | `[sp-196]` |
| `ldr` at 0x98ee8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x98eec | load | 2 | `[sp-20] +0x2e` | `-` |
| `str` at 0x98ef0 | store | 4 | `sp -0xa0` | `[[sp-20]+46]:2` |
| `ldr` at 0x98ef8 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x98efc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x98f00 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x98f0c | store | 4 | `sp -0x90` | `ret(0xbead0)` |
| `strb` at 0x98f14 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0x98f1c | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0x98f20 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x98f24 | store | 4 | `sp -0x84` | `ret(0xbead0)` |
| `strh` at 0x98f2c | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0x98f30 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0x98f40 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0x98f48 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0x98f4c | load | 4 | `sp -0xac` | `-` |
| `str` at 0x98f50 | store | 4 | `sp -0x7c` | `[sp-172]` |
| `strh` at 0x98f58 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0x98f5c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x98f6c | store | 4 | `sp -0xc8` | `0x2` |
| `str` at 0x98f78 | store | 4 | `sp -0xc8` | `0x1` |
| `str` at 0x98f80 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0x98f8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98f90 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x98f9c | load | 4 | `abs 0x98fd0` | `-` |
| `str` at 0x98fa0 | store | 4 | `sp -0xd8` | `0x112024` |
| `ldr` at 0x98fa4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x98fa8 | store | 4 | `sp -0xd4` | `[sp-32]` |
| `ldr` at 0x98fb0 | load | 4 | `abs 0x98fd4` | `-` |
| `ldr` at 0x98fb4 | load | 4 | `abs 0x98fd8` | `-` |
| `ldm` at 0x98fc4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x98fc4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x98fc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98760 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x98790 | `(unknown callee) 0x9871c` | `ret(0x488cc)` | `?` | `?` | `[0x133a9c]` |
| `bl` at 0x98824 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x98890 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x98938 | `(unknown callee) 0xbe850` | `[0x29c3a0]` | `?` | `[sp-24]` | `0x29c398` |
| `bl` at 0x98970 | `(unknown callee) 0x9802c` | `ret(0xbe850)` | `[0x29c3a0]` | `[0x29c3b0]` | `0x0` |
| `bl` at 0x989d4 | `(unknown callee) 0xbe850` | `[0x29c3a0]` | `?` | `?` | `0x29c398` |
| `bl` at 0x989f4 | `(unknown callee) 0x19280` | `[[sp-152]+20]` | `?` | `ret(0xbe850)` | `[sp-152]` |
| `bl` at 0x98a28 | `(unknown callee) 0x19538` | `(ret(0x19280) + [sp-160])` | `?` | `ret(0x19280)` | `(ret(0x19280) + [sp-160])` |
| `bl` at 0x98a40 | `(unknown callee) 0x19280` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0x98a74 | `(unknown callee) 0x19538` | `(ret(0x19280) + [sp-160])` | `?` | `ret(0x19280)` | `(ret(0x19280) + [sp-160])` |
| `bl` at 0x98a9c | `(unknown callee) 0xbe850` | `[0x29c3a4]` | `?` | `?` | `0x29c398` |
| `bl` at 0x98ad4 | `(unknown callee) 0x9802c` | `ret(0xbe850)` | `[0x29c3a4]` | `[0x29c3b0]` | `0x0` |
| `bl` at 0x98b44 | `(unknown callee) 0xbe850` | `[0x29c3a4]` | `?` | `?` | `0x29c398` |
| `bl` at 0x98b64 | `(unknown callee) 0x19280` | `[[sp-152]+20]` | `?` | `ret(0xbe850)` | `[sp-152]` |
| `bl` at 0x98b98 | `(unknown callee) 0x19538` | `(ret(0x19280) + [sp-164])` | `?` | `ret(0x19280)` | `(ret(0x19280) + [sp-164])` |
| `bl` at 0x98bb0 | `(unknown callee) 0x19280` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0x98be4 | `(unknown callee) 0x19538` | `(ret(0x19280) + [sp-164])` | `?` | `ret(0x19280)` | `(ret(0x19280) + [sp-164])` |
| `bl` at 0x98d48 | `(unknown callee) 0xaa090` | `0x0` | `0x0` | `[sp-20]` | `?` |
| `bl` at 0x98d7c | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x98db8 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x98dd8 | `(unknown callee) 0xbea9c` | `[sp-164]` | `[sp-184]` | `sp-168` | `sp-168` |
| `bl` at 0x98e68 | `(unknown callee) 0xaa090` | `0x0` | `0x0` | `[sp-20]` | `[sp-188]` |
| `bl` at 0x98e9c | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x98ed8 | `(unknown callee) 0xaadf4` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x98f04 | `(unknown callee) 0xbead0` | `[sp-164]` | `[sp-196]` | `[[sp-20]+46]:2` | `sp-172` |
| `bl` at 0x98f94 | `(unknown callee) 0xaa090` | `0x0` | `0x0` | `[sp-20]` | `[sp-200]` |
| `bl` at 0x98fbc | `(unknown callee) 0x200a0` | `0x4` | `0x111f90` | `0xce1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9878c | `[0x133a9c] != 0x0` | `[0x133a9c] == 0x0` |
| `beq` at 0x987a4 | `[sp-32] == 0x73` | `[sp-32] != 0x73` |
| `bgt` at 0x987b0 | `[sp-32] > 0x73 (signed, 32-bit)` | `[sp-32] <= 0x73 (signed, 32-bit)` |
| `beq` at 0x987bc | `[sp-32] == 0x72` | `[sp-32] != 0x72` |
| `beq` at 0x987cc | `[sp-32] == 0x74` | `[sp-32] != 0x74` |
| `bne` at 0x987e0 | `[0x29c3ac] != 0x2` | `[0x29c3ac] == 0x2` |
| `bls` at 0x987f0 | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x98804 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x98814 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0x98830 | `ret(0xaadf4) <= 0x3 (unsigned, 32-bit)` | `ret(0xaadf4) > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x9884c | `[0x29c3ac] != 0x3` | `[0x29c3ac] == 0x3` |
| `bls` at 0x9885c | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x98870 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x98880 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0x9889c | `ret(0xaadf4) <= 0x3 (unsigned, 32-bit)` | `ret(0xaadf4) > 0x3 (unsigned, 32-bit)` |
| `beq` at 0x988e0 | `[0x29c3c0] == 0x0` | `[0x29c3c0] != 0x0` |
| `bge` at 0x98984 | `ret(0x9802c) >= 0x0 (signed, 32-bit)` | `ret(0x9802c) < 0x0 (signed, 32-bit)` |
| `beq` at 0x98a90 | `[0x29c3a4] == 0x0` | `[0x29c3a4] != 0x0` |
| `bge` at 0x98af4 | `[sp-144] >= 0x0 (signed, 32-bit)` | `[sp-144] < 0x0 (signed, 32-bit)` |
| `beq` at 0x98ccc | `[0x29c3bc] == 0x0` | `[0x29c3bc] != 0x0` |
| `beq` at 0x98cdc | `[0x29c3b8] == 0x0` | `[0x29c3b8] != 0x0` |
| `bne` at 0x98d60 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x98d9c | `? != 0x0` | `? == 0x0` |
| `bge` at 0x98e0c | `ret(0xbea9c) >= 0x0 (signed, 32-bit)` | `ret(0xbea9c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x98e38 | `ret(0xbea9c) >= 0x0 (signed, 32-bit)` | `ret(0xbea9c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x98e80 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x98ebc | `? != 0x0` | `? == 0x0` |
| `bge` at 0x98f38 | `ret(0xbead0) >= 0x0 (signed, 32-bit)` | `ret(0xbead0) < 0x0 (signed, 32-bit)` |
| `bge` at 0x98f64 | `ret(0xbead0) >= 0x0 (signed, 32-bit)` | `ret(0xbead0) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x98fc4 | `r0` | `?` |

## `main_task`

- `main_task`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5adc4 to 0x5afa0.
  Established. The unnamed `0xbcfbc` call carries `0x1`, `0x2`, `0x9600`, unknown (`bl` at 0x5ae40).
  A zero `[0x133ab4]` takes `0x5af00`, while nonzero falls through (`beq` at 0x5aeec).
  The included indirect call at `0x5aefc` carries `ret(0xbbda8)`, unknown, unknown, `[0x133ab4]` (`mov` at 0x5aefc).
  Startup hook behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5add4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5add8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5ade0 | store | 4 | `sp -0x20` | `0x2` |
| `str` at 0x5ade8 | store | 4 | `sp -0x24` | `0x6` |
| `str` at 0x5adf0 | store | 4 | `sp -0x30` | `0x1e` |
| `ldr` at 0x5adf8 | load | 4 | `abs 0x5af80` | `-` |
| `ldr` at 0x5adfc | load | 4 | `abs 0x5af84` | `-` |
| `ldr` at 0x5ae0c | load | 4 | `abs 0x5af88` | `-` |
| `ldr` at 0x5ae10 | load | 4 | `abs 0x297580` | `-` |
| `ldr` at 0x5ae1c | load | 4 | `abs 0x5af88` | `-` |
| `ldr` at 0x5ae20 | load | 4 | `abs 0x297580` | `-` |
| `ldr` at 0x5ae64 | load | 4 | `abs 0x5af88` | `-` |
| `ldr` at 0x5ae68 | load | 4 | `abs 0x297584` | `-` |
| `ldr` at 0x5aec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5aee0 | load | 4 | `abs 0x5af8c` | `-` |
| `ldr` at 0x5aee4 | load | 4 | `abs 0x133ab4` | `-` |
| `ldr` at 0x5aef0 | load | 4 | `abs 0x5af8c` | `-` |
| `ldr` at 0x5aef4 | load | 4 | `abs 0x133ab4` | `-` |
| `ldr` at 0x5af24 | load | 4 | `abs 0x5af90` | `-` |
| `ldr` at 0x5af34 | load | 4 | `abs 0x5af94` | `-` |
| `ldr` at 0x5af4c | load | 4 | `abs 0x5af98` | `-` |
| `ldr` at 0x5af6c | load | 4 | `abs 0x5af9c` | `-` |
| `ldm` at 0x5af7c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5af7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5af7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5af7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5adf4 | `(unknown callee) 0x19ec4` | `r0` | `r1` | `r2` | `0x1e` |
| `bl` at 0x5ae00 | `(unknown callee) 0x1e958` | `0xf9d60` | `0xf9d70` | `?` | `?` |
| `bl` at 0x5ae04 | `(unknown callee) 0x54f04` | `ret(0x1e958)` | `?` | `?` | `?` |
| `bl` at 0x5ae08 | `(unknown callee) 0x48194` | `ret(0x54f04)` | `?` | `?` | `?` |
| `mov` at 0x5ae28 | `(unknown callee) (unknown target)` | `ret(0x48194)` | `?` | `?` | `[0x297580]` |
| `bl` at 0x5ae30 | `(unknown callee) 0x5a4d0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5ae40 | `(unknown callee) 0xbcfbc` | `0x1` | `0x2` | `0x9600` | `?` |
| `bl` at 0x5ae48 | `(unknown callee) 0x21dfc` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x5ae54 | `(unknown callee) 0x20020` | `0x0` | `0x0` | `?` | `?` |
| `bl` at 0x5ae5c | `(unknown callee) 0xbe494` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5ae60 | `(unknown callee) 0xd6e58` | `ret(0xbe494)` | `?` | `?` | `?` |
| `mov` at 0x5ae70 | `(unknown callee) (unknown target)` | `ret(0xd6e58)` | `?` | `?` | `[0x297584]` |
| `bl` at 0x5ae74 | `(unknown callee) 0x4a5c4` | `ret(unknown)` | `?` | `?` | `?` |
| `bl` at 0x5ae78 | `(unknown callee) 0x1eff8` | `ret(0x4a5c4)` | `?` | `?` | `?` |
| `bl` at 0x5ae80 | `(unknown callee) 0x1d444` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x5ae98 | `(unknown callee) 0x1d444` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x5aeac | `(unknown callee) 0xa88fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x5aeb0 | `(unknown callee) 0x7ca54` | `ret(0xa88fc)` | `?` | `?` | `?` |
| `bl` at 0x5aeb4 | `(unknown callee) 0x8cec4` | `ret(0x7ca54)` | `?` | `?` | `?` |
| `bl` at 0x5aeb8 | `(unknown callee) 0x964c0` | `ret(0x8cec4)` | `?` | `?` | `?` |
| `bl` at 0x5aebc | `(unknown callee) 0xa44f0` | `ret(0x964c0)` | `?` | `?` | `?` |
| `bl` at 0x5aec0 | `(unknown callee) 0x5e7ec` | `ret(0xa44f0)` | `?` | `?` | `?` |
| `bl` at 0x5aec4 | `(unknown callee) 0x60370` | `ret(0x5e7ec)` | `?` | `?` | `?` |
| `bl` at 0x5aecc | `(unknown callee) 0x5bd54` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5aed0 | `(unknown callee) 0x70014` | `ret(0x5bd54)` | `?` | `?` | `?` |
| `bl` at 0x5aed4 | `(unknown callee) 0xdc69c` | `ret(0x70014)` | `?` | `?` | `?` |
| `bl` at 0x5aed8 | `(unknown callee) 0x984b4` | `ret(0xdc69c)` | `?` | `?` | `?` |
| `bl` at 0x5aedc | `(unknown callee) 0xbbda8` | `ret(0x984b4)` | `?` | `?` | `?` |
| `mov` at 0x5aefc | `(unknown callee) (unknown target)` | `ret(0xbbda8)` | `?` | `?` | `[0x133ab4]` |
| `bl` at 0x5af00 | `(unknown callee) 0x5afa0` | `?` | `?` | `?` | `?` |
| `bl` at 0x5af04 | `(unknown callee) 0x6c4e8` | `ret(0x5afa0)` | `?` | `?` | `?` |
| `bl` at 0x5af08 | `(unknown callee) 0x77e0c` | `ret(0x6c4e8)` | `?` | `?` | `?` |
| `bl` at 0x5af0c | `(unknown callee) 0x488cc` | `ret(0x77e0c)` | `?` | `?` | `?` |
| `bl` at 0x5af2c | `(unknown callee) 0x22e24` | `0x31` | `0xf9d78` | `0x2` | `?` |
| `bl` at 0x5af3c | `(unknown callee) 0x22e24` | `0x32` | `0xf9d88` | `0x2` | `?` |
| `bl` at 0x5af44 | `(unknown callee) 0x5f70c` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0x5af54 | `(unknown callee) 0x22e24` | `0x32` | `0xf9d98` | `0x0` | `?` |
| `bl` at 0x5af58 | `(unknown callee) 0x1eeb0` | `ret(0x22e24)` | `?` | `?` | `?` |
| `bl` at 0x5af74 | `(unknown callee) 0x22e24` | `0x32` | `0xf9da4` | `0x1` | `ret(0x1eeb0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ae18 | `[0x297580] == 0x0` | `[0x297580] != 0x0` |
| `beq` at 0x5aeec | `[0x133ab4] == 0x0` | `[0x133ab4] != 0x0` |
| `bne` at 0x5af1c | `? != 0x3f` | `? == 0x3f` |
| `beq` at 0x5af64 | `ret(0x1eeb0) == 0x0` | `ret(0x1eeb0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5af7c | `r0` | `?` |

## `L1ReqInit`

- `L1ReqInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7ca54 to 0x7cb48.
  Established. The unnamed `0xa75f8` call carries `0x29a6ac`, `0xffff006`, `0x1`, `0x7cb48` (`bl` at 0x7cac4);
  its zero result takes `0x7cb00`, while nonzero falls through (`beq` at 0x7cad8).
  The absolute word at `0x1338ec` receives `[0x1338ec]+1` (`str` at 0x7cb10).
  Request registration policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x7ca64 | load | 4 | `abs 0x7cb1c` | `-` |
| `ldr` at 0x7ca68 | load | 4 | `abs 0x1338ec` | `-` |
| `ldr` at 0x7ca74 | load | 4 | `abs 0x7cb20` | `-` |
| `str` at 0x7ca78 | store | 4 | `sp -0x1c` | `0xfebd4` |
| `ldr` at 0x7ca80 | load | 4 | `abs 0x7cb24` | `-` |
| `ldr` at 0x7ca84 | load | 4 | `abs 0x7cb28` | `-` |
| `ldr` at 0x7ca90 | load | 4 | `abs 0x7cb2c` | `-` |
| `ldr` at 0x7ca94 | load | 4 | `abs 0x7cb30` | `-` |
| `ldr` at 0x7caa4 | load | 4 | `abs 0x7cb34` | `-` |
| `ldr` at 0x7cab4 | load | 4 | `abs 0x7cb34` | `-` |
| `ldr` at 0x7cab8 | load | 4 | `abs 0x7cb38` | `-` |
| `ldr` at 0x7cac0 | load | 4 | `abs 0x7cb3c` | `-` |
| `str` at 0x7cacc | store | 4 | `sp -0x14` | `ret(0xa75f8)` |
| `ldr` at 0x7cad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7cadc | load | 4 | `abs 0x7cb40` | `-` |
| `str` at 0x7cae0 | store | 4 | `sp -0x1c` | `0xfec14` |
| `ldr` at 0x7cae4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7cae8 | store | 4 | `sp -0x18` | `ret(0xa75f8)` |
| `ldr` at 0x7caf0 | load | 4 | `abs 0x7cb24` | `-` |
| `ldr` at 0x7caf4 | load | 4 | `abs 0x7cb44` | `-` |
| `ldr` at 0x7cb00 | load | 4 | `abs 0x7cb1c` | `-` |
| `ldr` at 0x7cb04 | load | 4 | `abs 0x7cb1c` | `-` |
| `ldr` at 0x7cb08 | load | 4 | `abs 0x1338ec` | `-` |
| `str` at 0x7cb10 | store | 4 | `abs 0x1338ec` | `[0x1338ec]+1` |
| `ldm` at 0x7cb18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7cb18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7cb18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7ca8c | `(unknown callee) 0x200a0` | `0x9` | `0xfebf0` | `0x18e` | `0x0` |
| `bl` at 0x7caa0 | `(unknown callee) 0x20bb4` | `0x29a658` | `0xfec08` | `0x1` | `0x9` |
| `bl` at 0x7cab0 | `(unknown callee) 0xa89d0` | `0x29a6ac` | `0x6` | `0x0` | `?` |
| `bl` at 0x7cac4 | `(unknown callee) 0xa75f8` | `0x29a6ac` | `0xffff006` | `0x1` | `0x7cb48` |
| `bl` at 0x7cafc | `(unknown callee) 0x200a0` | `0x9` | `0xfebf0` | `0x1a2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7ca70 | `[0x1338ec] == 0x0` | `[0x1338ec] != 0x0` |
| `beq` at 0x7cad8 | `ret(0xa75f8) == 0x0` | `ret(0xa75f8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7cb18 | `r0` | `?` |

## `FlashInit`

- `FlashInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x984b4 to 0x98648.
  Established. The absolute word at `0x133a30` receives `[0x133a30]+1` (`str` at 0x98500).
  The unnamed `0xa76a4` call carries `0x29a8a0`, `[0x29a8a0]`, `0x1`, `0x29a8ac` (`bl` at 0x98588);
  its zero result takes `0x985cc`, while nonzero falls through (`beq` at 0x9859c).
  Flash task setup meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x984c4 | load | 4 | `abs 0x98610` | `-` |
| `ldr` at 0x984c8 | load | 4 | `abs 0x133a30` | `-` |
| `ldr` at 0x984d4 | load | 4 | `abs 0x98614` | `-` |
| `str` at 0x984d8 | store | 4 | `sp -0x30` | `0x111f78` |
| `ldr` at 0x984e0 | load | 4 | `abs 0x98618` | `-` |
| `ldr` at 0x984e4 | load | 4 | `abs 0x9861c` | `-` |
| `ldr` at 0x984f0 | load | 4 | `abs 0x98610` | `-` |
| `ldr` at 0x984f4 | load | 4 | `abs 0x98610` | `-` |
| `ldr` at 0x984f8 | load | 4 | `abs 0x133a30` | `-` |
| `str` at 0x98500 | store | 4 | `abs 0x133a30` | `[0x133a30]+1` |
| `ldr` at 0x98504 | load | 4 | `abs 0x98620` | `-` |
| `str` at 0x98518 | store | 4 | `sp -0x18` | `0x19` |
| `str` at 0x98520 | store | 4 | `sp -0x1c` | `0x14` |
| `ldr` at 0x98524 | load | 4 | `abs 0x98624` | `-` |
| `ldr` at 0x98528 | load | 4 | `abs 0x98628` | `-` |
| `str` at 0x98538 | store | 4 | `sp -0x14` | `ret(0xa8af0)` |
| `ldr` at 0x9853c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98548 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98554 | load | 4 | `abs 0x9862c` | `-` |
| `str` at 0x98558 | store | 4 | `sp -0x30` | `0x111fb4` |
| `str` at 0x9855c | store | 4 | `sp -0x2c` | `ret(0x1f8e0)` |
| `ldr` at 0x98564 | load | 4 | `abs 0x98618` | `-` |
| `ldr` at 0x98568 | load | 4 | `abs 0x98630` | `-` |
| `ldr` at 0x98574 | load | 4 | `abs 0x98620` | `-` |
| `ldr` at 0x98578 | load | 4 | `abs 0x98620` | `-` |
| `ldr` at 0x9857c | load | 4 | `abs 0x29a8a0` | `-` |
| `ldr` at 0x98584 | load | 4 | `abs 0x98624` | `-` |
| `str` at 0x98590 | store | 4 | `sp -0x14` | `ret(0xa76a4)` |
| `ldr` at 0x98594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x985a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x985ac | load | 4 | `abs 0x98634` | `-` |
| `str` at 0x985b0 | store | 4 | `sp -0x30` | `0x111fd8` |
| `str` at 0x985b4 | store | 4 | `sp -0x2c` | `ret(0x1f8e0)` |
| `ldr` at 0x985bc | load | 4 | `abs 0x98618` | `-` |
| `ldr` at 0x985c0 | load | 4 | `abs 0x98638` | `-` |
| `str` at 0x985d0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x985d8 | store | 4 | `sp -0x2c` | `0x400` |
| `ldr` at 0x985dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x985e0 | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0x985e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x985e8 | store | 4 | `sp -0x24` | `[sp-28]` |
| `str` at 0x985f0 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x985f4 | load | 4 | `abs 0x9863c` | `-` |
| `ldr` at 0x985f8 | load | 4 | `abs 0x98640` | `-` |
| `ldr` at 0x985fc | load | 4 | `abs 0x98644` | `-` |
| `ldm` at 0x9860c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9860c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9860c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x984ec | `(unknown callee) 0x200a0` | `0x9` | `0x111f90` | `0xbc6` | `0x0` |
| `bl` at 0x98510 | `(unknown callee) 0xa89d0` | `0x29a8a0` | `0x10` | `0x0` | `[0x133a30]+1` |
| `bl` at 0x98530 | `(unknown callee) 0xa8af0` | `0x29a8ac` | `0x111fac` | `0x5` | `0x14` |
| `bl` at 0x9854c | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x98570 | `(unknown callee) 0x200a0` | `0x9` | `0x111f90` | `0xbde` | `0x0` |
| `bl` at 0x98588 | `(unknown callee) 0xa76a4` | `0x29a8a0` | `[0x29a8a0]` | `0x1` | `0x29a8ac` |
| `bl` at 0x985a4 | `(unknown callee) 0x1f8e0` | `ret(0xa76a4)` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0x985c8 | `(unknown callee) 0x200a0` | `0x9` | `0x111f90` | `0xbe7` | `0x0` |
| `bl` at 0x98604 | `(unknown callee) 0x2066c` | `0x29a8a8` | `0x111ffc` | `0x98648` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x984d0 | `[0x133a30] == 0x0` | `[0x133a30] != 0x0` |
| `beq` at 0x98544 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0x9859c | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9860c | `r0` | `ret(0x2066c)` |
