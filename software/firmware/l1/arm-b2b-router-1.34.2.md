# ARM board-link and router entry paths in 1.34.2

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
Computed transfers in B2BInit and B2BReqRsp leave unresolved edges. Their tables cover known edges only. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/b2b-router-facts.txt) retain all tool sections.

## `B2BInit`

- `B2BInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x603dc to 0x60e50.
  Established. The word at `0x11a744` receives `[0x11a744]+1` (`str` at 0x60428). Successive word
  stores to `0x11a74c` contain `0xfff` (`str` at 0x60434), one (`str` at 0x60440) and zero (`str` at
  0x6044c). Two calls to `0x18c20` have first and third candidates `(0x281f20, 0x84)` (`bl` at
  0x6046c) and `(0x281f3c, 0x34)` (`bl` at 0x6047c), respectively. The computed transfer is taken
  for unsigned `[sp-52]-53 <= 0x29` and falls through when greater (`ldrls` at 0x6048c); its taken
  targets and the board-link initialization semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x603ec | load | 4 | `abs 0x60dc8` | `-` |
| `ldr` at 0x603f0 | load | 4 | `abs 0x11a744` | `-` |
| `ldr` at 0x603fc | load | 4 | `abs 0x60dcc` | `-` |
| `str` at 0x60400 | store | 4 | `sp -0x64` | `0xe6eac` |
| `ldr` at 0x60408 | load | 4 | `abs 0x60dd0` | `-` |
| `ldr` at 0x60418 | load | 4 | `abs 0x60dc8` | `-` |
| `ldr` at 0x6041c | load | 4 | `abs 0x60dc8` | `-` |
| `ldr` at 0x60420 | load | 4 | `abs 0x11a744` | `-` |
| `str` at 0x60428 | store | 4 | `abs 0x11a744` | `[0x11a744]+1` |
| `ldr` at 0x6042c | load | 4 | `abs 0x60dd4` | `-` |
| `ldr` at 0x60430 | load | 4 | `abs 0x60dd8` | `-` |
| `str` at 0x60434 | store | 4 | `abs 0x11a74c` | `0xfff` |
| `ldr` at 0x60438 | load | 4 | `abs 0x60dd4` | `-` |
| `str` at 0x60440 | store | 4 | `abs 0x11a74c` | `0x1` |
| `ldr` at 0x60444 | load | 4 | `abs 0x60dd4` | `-` |
| `str` at 0x6044c | store | 4 | `abs 0x11a74c` | `0x0` |
| `str` at 0x6045c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x60460 | load | 4 | `abs 0x60ddc` | `-` |
| `ldr` at 0x60470 | load | 4 | `abs 0x60de0` | `-` |
| `ldr` at 0x60480 | load | 4 | `sp -0x34` | `-` |
| `ldrls` at 0x6048c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6066c | load | 4 | `abs 0x60de4` | `-` |
| `ldr` at 0x60670 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x60674 | load | 4 | `abs 0x60de8` | `-` |
| `ldmdb` at 0x60dc4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x60dc4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x60dc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60dc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60dc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60414 | `(unknown callee) 0x1eed0` | `0x9` | `0xe6ec8` | `0x66` | `0x0` |
| `bl` at 0x60450 | `(unknown callee) 0x2b31c` | `?` | `?` | `0x11a74c` | `0x0` |
| `bl` at 0x6046c | `(unknown callee) 0x18c20` | `0x281f20` | `0x0` | `0x84` | `?` |
| `bl` at 0x6047c | `(unknown callee) 0x18c20` | `0x281f3c` | `?` | `0x34` | `?` |
| `bl` at 0x60678 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6ee0` | `?` | `0x116928` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x603f8 | `[0x11a744] == 0x0` | `[0x11a744] != 0x0` |
| `ldrls` at 0x6048c | `[sp-52]-53 <= 0x29 (unsigned, 32-bit)` | `[sp-52]-53 > 0x29 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60dc4 | `r0` | `ret(0x21010)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6048c).

## `B2BEventMesh`

- `B2BEventMesh`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x619a4 to 0x623f4.
  Established. The first branch takes the signed nonnegative arm for `[r0+28]`, with negative values
  falling through (`bge` at 0x619d8). Explicit word writes include an unknown value at `0x11a74c`
  (`str` at 0x61c84), `[sp-20]` at `0x11b454` (`str` at 0x61c90) and zero at `[sp-20] +0x18` (`str`
  at 0x61cf0). A call to `0x5e958` has candidates `(sp-32, sp-48, sp-52, sp-32)` (`bl` at 0x61dd4).
  A later nonzero result from `0x9ba98` takes the branch, while zero falls through to the exit
  (`bne` at 0x62098). Mesh routing and field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x619b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x619bc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x619c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x619c4 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x619c8 | store | 4 | `sp -0x20` | `[r0+12]` |
| `ldr` at 0x619cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x619d0 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x619dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x619e0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x619e8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x619ec | load | 4 | `sp -0x50` | `-` |
| `str` at 0x619f4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x619f8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x61a08 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x61a10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61a14 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61a1c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x61a20 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x61a30 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x61a34 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x61a38 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0x61a3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61a40 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61a4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61a50 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61a58 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x61a5c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61a64 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x61a68 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61a78 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x61a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61a84 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61a8c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x61a90 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61aa0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x61aa4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61aa8 | store | 4 | `sp -0x28` | `[sp-84]` |
| `ldr` at 0x61aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61ab0 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61abc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61ac0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61ac8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61acc | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61ad4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61ad8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61ae8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61af4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61afc | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61b00 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61b10 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61b14 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61b18 | store | 4 | `sp -0x2c` | `[sp-88]` |
| `ldr` at 0x61b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b20 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x61b30 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x61b34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b38 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x61b44 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x61b48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b4c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x61b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b5c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x61b70 | store | 4 | `sp -0x5c` | `0xe` |
| `str` at 0x61b7c | store | 4 | `sp -0x5c` | `0xf` |
| `ldr` at 0x61b84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61b88 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x61b90 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x61b94 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x61b9c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x61ba0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x61ba4 | store | 4 | `sp -0x38` | `[sp-92]` |
| `ldr` at 0x61ba8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61bac | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61bbc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x61bc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61bc4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x61bd0 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x61bd8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x61bdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61be0 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61bf0 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61c0c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61c20 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61c2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61c30 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61c48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61c4c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x61c60 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61c64 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61c74 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61c78 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61c7c | load | 4 | `abs 0x11a74c` | `-` |
| `str` at 0x61c84 | store | 4 | `abs 0x11a74c` | `?` |
| `ldr` at 0x61c88 | load | 4 | `abs 0x62390` | `-` |
| `ldr` at 0x61c8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61c90 | store | 4 | `abs 0x11b454` | `[sp-20]` |
| `ldr` at 0x61c94 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61c98 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61ca8 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61cac | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x61cb0 | load | 4 | `abs 0x62398` | `-` |
| `ldr` at 0x61cb4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x61cb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x61cc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61cd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61ce8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61cf0 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `str` at 0x61cf8 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x61d00 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x61d04 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61d08 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61d18 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61d1c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x61d20 | load | 4 | `abs 0x6239c` | `-` |
| `ldr` at 0x61d24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x61d28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x61d30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x61d3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61d48 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0x61d50 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x61d54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61d60 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x61d6c | load | 4 | `abs 0x623a0` | `-` |
| `ldr` at 0x61d70 | load | 4 | `abs 0x11a754` | `-` |
| `ldr` at 0x61d74 | load | 4 | `[0x11a754] +0x58` | `-` |
| `ldr` at 0x61d80 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x61d8c | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61d90 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61da0 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61da4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x61da8 | load | 4 | `abs 0x623a4` | `-` |
| `ldr` at 0x61db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61dd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61de4 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61de8 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61df8 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61dfc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x61e00 | load | 4 | `abs 0x623a8` | `-` |
| `ldr` at 0x61e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61e14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61e24 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x61e3c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x61e40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61e5c | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61e60 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61e70 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61e74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61e78 | store | 4 | `sp -0x7c` | `[sp-32]` |
| `ldr` at 0x61e7c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x61e80 | load | 4 | `abs 0x623ac` | `-` |
| `ldr` at 0x61e84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61e88 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x61e90 | load | 4 | `abs 0x623b0` | `-` |
| `ldr` at 0x61e94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61e98 | load | 4 | `abs 0x11a758` | `-` |
| `ldr` at 0x61ea4 | load | 4 | `abs 0x623b4` | `-` |
| `ldr` at 0x61ea8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x61eac | load | 4 | `abs 0x11a75c` | `-` |
| `ldr` at 0x61eb8 | load | 4 | `abs 0x623b8` | `-` |
| `str` at 0x61ebc | store | 4 | `sp -0x4c` | `0x281270` |
| `ldr` at 0x61ec0 | load | 4 | `abs 0x623a0` | `-` |
| `ldr` at 0x61ec4 | load | 4 | `abs 0x11a754` | `-` |
| `ldr` at 0x61ec8 | load | 4 | `[0x11a754] +0x58` | `-` |
| `ldr` at 0x61ef0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61efc | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61f00 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61f10 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61f14 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x61f18 | load | 4 | `abs 0x623bc` | `-` |
| `ldr` at 0x61f20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61f3c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x61f54 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x61f58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f7c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x61f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61f8c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x61fa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61fa8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x61fbc | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x61fc0 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x61fd0 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x61fd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61fd8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x61fe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61fec | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x61ff8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x61ffc | store | 4 | `sp -0x7c` | `[sp-72]` |
| `ldr` at 0x62000 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62004 | load | 4 | `abs 0x623c0` | `-` |
| `ldr` at 0x62010 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6201c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62034 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x62038 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62048 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x6204c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62050 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x62060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62064 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x62070 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62074 | load | 4 | `abs 0x623c4` | `-` |
| `ldr` at 0x6207c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62080 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6208c | store | 4 | `sp -0x1c` | `ret(0x9ba98)` |
| `ldr` at 0x62090 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x620a0 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x620a4 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x620b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x620c0 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x620c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x620c8 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x620cc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x620d0 | load | 4 | `abs 0x623c8` | `-` |
| `ldr` at 0x620e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x620ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x620f8 | load | 4 | `abs 0x623cc` | `-` |
| `ldr` at 0x62100 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62104 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x62110 | store | 4 | `sp -0x18` | `ret(0x5e764)` |
| `ldr` at 0x62114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x62124 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x62128 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62138 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x6213c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62140 | load | 4 | `abs 0x623d0` | `-` |
| `ldr` at 0x62144 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62148 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x62150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6215c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x62160 | load | 4 | `ret(0x5e764) +0x10` | `-` |
| `str` at 0x62164 | store | 4 | `sp -0x20` | `[ret(0x5e764)+16]` |
| `ldr` at 0x6216c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62180 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x62184 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62194 | load | 4 | `abs 0x62394` | `-` |
| `str` at 0x62198 | store | 4 | `sp -0x60` | `0x116928` |
| `ldr` at 0x6219c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x621a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x621b4 | load | 4 | `abs 0x623d4` | `-` |
| `str` at 0x621b8 | store | 4 | `sp -0x64` | `0xe72b4` |
| `ldr` at 0x621c0 | load | 4 | `abs 0x623d8` | `-` |
| `str` at 0x621c4 | store | 4 | `sp -0x64` | `0xe72b8` |
| `ldr` at 0x621cc | load | 4 | `abs 0x623dc` | `-` |
| `str` at 0x621d0 | store | 4 | `sp -0x64` | `0xe72bc` |
| `ldr` at 0x621d4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x621d8 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x621dc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x621e0 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x621e4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x621e8 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x621ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0x621f0 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x621f4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x621f8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x621fc | load | 4 | `abs 0x623e0` | `-` |
| `ldr` at 0x62200 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x62204 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6220c | load | 4 | `abs 0x623e4` | `-` |
| `ldr` at 0x62210 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62224 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62230 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6223c | store | 4 | `sp -0x1c` | `ret(0x9ba98)` |
| `ldr` at 0x62240 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62250 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x62254 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62264 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62270 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x62274 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x62278 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x6227c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62280 | load | 4 | `abs 0x623e8` | `-` |
| `ldr` at 0x62290 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6229c | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x622a0 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x622b0 | load | 4 | `abs 0x62394` | `-` |
| `ldr` at 0x622b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x622b8 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x622bc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x622c0 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x622c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x622c8 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x622cc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x622d0 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x622d4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x622d8 | load | 4 | `abs 0x623ec` | `-` |
| `ldr` at 0x622dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x622e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x622e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x622f4 | load | 4 | `abs 0x6238c` | `-` |
| `ldr` at 0x622f8 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62308 | load | 4 | `abs 0x62394` | `-` |
| `str` at 0x6230c | store | 4 | `sp -0x68` | `0x116928` |
| `ldr` at 0x62310 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6231c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62328 | load | 4 | `abs 0x623d4` | `-` |
| `str` at 0x6232c | store | 4 | `sp -0x6c` | `0xe72b4` |
| `ldr` at 0x62334 | load | 4 | `abs 0x623d8` | `-` |
| `str` at 0x62338 | store | 4 | `sp -0x6c` | `0xe72b8` |
| `ldr` at 0x62340 | load | 4 | `abs 0x623dc` | `-` |
| `str` at 0x62344 | store | 4 | `sp -0x6c` | `0xe72bc` |
| `ldr` at 0x62348 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6234c | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x62350 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x62354 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x62358 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6235c | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x62360 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x62364 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x62368 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x6236c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62370 | load | 4 | `abs 0x623f0` | `-` |
| `ldr` at 0x62374 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x62378 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62380 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x62388 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x62388 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x62388 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61cbc | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7104` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x61cd4 | `(unknown callee) 0x5eaa8` | `?` | `?` | `?` | `?` |
| `bl` at 0x61d2c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7124` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x61d40 | `(unknown callee) 0x5c56c` | `[sp-20]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x61dac | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7150` | `?` | `0x116928` |
| `bl` at 0x61db4 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x61dd4 | `(unknown callee) 0x5e958` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x61e04 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7168` | `?` | `0x116928` |
| `bl` at 0x61e0c | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x61e58 | `(unknown callee) 0x18ae8` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x61e8c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7194` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x61eec | `(unknown callee) 0x5e958` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x61f1c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe71b0` | `?` | `0x116928` |
| `bl` at 0x61f24 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x61f70 | `(unknown callee) 0x18ae8` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x6200c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe71e4` | `?` | `?` |
| `bl` at 0x62020 | `(unknown callee) 0x5c778` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x6202c | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x62078 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe721c` | `?` | `?` |
| `bl` at 0x62084 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0x620b8 | `(unknown callee) 0x1e71c` | `ret(0x9ba98)` | `?` | `?` | `?` |
| `bl` at 0x620dc | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7244` | `[[sp-20]+22]:1` | `ret(0x1e71c)` |
| `bl` at 0x620e4 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x620fc | `(unknown callee) 0x56518` | `0xe7270` | `?` | `?` | `[sp-32]` |
| `bl` at 0x62108 | `(unknown callee) 0x5e764` | `[sp-48]` | `[sp-52]` | `?` | `?` |
| `bl` at 0x62120 | `(unknown callee) 0x5659c` | `ret(0x5e764)` | `?` | `?` | `ret(0x5e764)` |
| `bl` at 0x6214c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7280` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x62154 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x62168 | `(unknown callee) 0x5659c` | `ret(0x5e764)` | `?` | `?` | `[ret(0x5e764)+16]` |
| `bl` at 0x62170 | `(unknown callee) 0x63278` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x62208 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe72c0` | `?` | `[sp-56]` |
| `bl` at 0x62234 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x62268 | `(unknown callee) 0x1e71c` | `ret(0x9ba98)` | `?` | `?` | `?` |
| `bl` at 0x6228c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe72f0` | `[[sp-20]+22]:1` | `ret(0x1e71c)` |
| `bl` at 0x62294 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x622e4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe731c` | `[sp-32]` | `[sp-56]` |
| `bl` at 0x622ec | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6237c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7350` | `?` | `[sp-56]` |
| `bl` at 0x62384 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x619d8 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x61a48 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x61ab8 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x61b54 | `[r0+24] >= 0x0 (signed, 32-bit)` | `[r0+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x61b68 | `? != 0x2` | `? == 0x2` |
| `bge` at 0x61be8 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x61c00 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x61c18 | `? == 0x5` | `? != 0x5` |
| `bge` at 0x61c28 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x61c40 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x61c58 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x61c70 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61ca4 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x61ccc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x61ce0 | `ret(0x5eaa8) != 0x0` | `ret(0x5eaa8) == 0x0` |
| `beq` at 0x61d14 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61d38 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0x61d5c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0x61d68 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x61d7c | `[[0x11a754]+88] == 0x0` | `[[0x11a754]+88] != 0x0` |
| `bne` at 0x61d88 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x61d9c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x61de0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x61df4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61e6c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x61ea0 | `[sp-48] != [0x11a758]` | `[sp-48] == [0x11a758]` |
| `bne` at 0x61eb4 | `[sp-52] != [0x11a75c]` | `[sp-52] == [0x11a75c]` |
| `bne` at 0x61ed0 | `[[0x11a754]+88] != 0x0` | `[[0x11a754]+88] == 0x0` |
| `bne` at 0x61ef8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x61f0c | `? == 0x0` | `? != 0x0` |
| `bge` at 0x61f84 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x61f9c | `? == 0xe` | `? != 0xe` |
| `beq` at 0x61fb4 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x61fcc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x62018 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x62044 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x62098 | `ret(0x9ba98) != 0x0` | `ret(0x9ba98) == 0x0` |
| `beq` at 0x620b0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x620f4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6211c | `ret(0x5e764) != 0x0` | `ret(0x5e764) == 0x0` |
| `beq` at 0x62134 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6217c | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x62190 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x621a4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x621b0 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x62248 | `ret(0x9ba98) != 0x0` | `ret(0x9ba98) == 0x0` |
| `beq` at 0x62260 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x622ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62304 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x62318 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x62324 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x62388 | `r0` | `?` |

## `B2BEventRing`

- `B2BEventRing`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x623f4 to 0x62b70.
  Established. The initial comparison branches when `[r0+28]` is signed nonnegative, and falls
  through when negative (`bge` at 0x62480). A word store contains `[0x11a748]` at an unknown
  effective target (`str` at 0x6252c). Calls include `0x9b9c4` with candidates `(0x281f20, [sp-20],
  unknown, unknown)` (`bl` at 0x62590) and `0x9ba98` with candidates `(unknown, [sp-20], 0x50,
  unknown)` (`bl` at 0x62734). A zero result from `0x63278` takes its branch, while nonzero falls
  through (`beq` at 0x62664). Ring topology and ownership are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62404 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6240c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x62410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62414 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x62418 | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x6241c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62420 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x62430 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x62434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62438 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x62444 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x62448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6244c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6245c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x62460 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62464 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x62470 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x62474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62478 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x62484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62488 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x62490 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x62494 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6249c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x624a0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x624b0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x624b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x624bc | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x624c4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x624c8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x624d8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x624dc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x624e0 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x624e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x624f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x624fc | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x62500 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62510 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x62514 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62518 | load | 4 | `abs 0x62b40` | `-` |
| `ldr` at 0x62520 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62524 | load | 4 | `abs 0x62b44` | `-` |
| `ldr` at 0x62528 | load | 4 | `abs 0x11a748` | `-` |
| `str` at 0x6252c | store | 4 | `(unknown)` | `[0x11a748]` |
| `ldr` at 0x62534 | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x62538 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62548 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x6254c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62550 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x62560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62564 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x62570 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62574 | load | 4 | `abs 0x62b48` | `-` |
| `ldr` at 0x6257c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62588 | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x6258c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62594 | load | 4 | `abs 0x62b50` | `-` |
| `ldr` at 0x62598 | load | 4 | `abs 0x11a754` | `-` |
| `ldr` at 0x6259c | load | 4 | `[0x11a754] +0x58` | `-` |
| `ldr` at 0x625a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x625b4 | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x625b8 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x625c8 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x625cc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x625d0 | load | 4 | `abs 0x62b54` | `-` |
| `ldr` at 0x625d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x625e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x625f4 | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x625f8 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62608 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x6260c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62610 | load | 4 | `abs 0x62b58` | `-` |
| `ldr` at 0x62614 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6261c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6262c | store | 4 | `sp -0x44` | `0x2` |
| `str` at 0x62638 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x6263c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x62640 | store | 4 | `sp -0x1c` | `[sp-68]` |
| `ldr` at 0x62644 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62650 | store | 4 | `sp -0x38` | `ret(0x5e9d0)` |
| `ldr` at 0x62654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62668 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6266c | load | 4 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x62678 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6267c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x62688 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6268c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x62698 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x6269c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x626a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x626ac | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x626b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x626bc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x626c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x626cc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x626d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x626dc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x626e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x626ec | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x626f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x626fc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x6270c | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x62710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62724 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6273c | store | 4 | `sp -0x20` | `ret(0x9ba98)` |
| `ldr` at 0x62740 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6275c | load | 4 | `abs 0x62b5c` | `-` |
| `ldr` at 0x62760 | load | 4 | `abs 0x11a0cc` | `-` |
| `ldr` at 0x6276c | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x62770 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62780 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x62784 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62788 | load | 4 | `abs 0x62b60` | `-` |
| `ldr` at 0x62790 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x627a0 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x627a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x627b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x627c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x627d4 | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x627d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x627ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x627f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6280c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62818 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62820 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x62828 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x62838 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x6283c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6284c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62860 | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x62864 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62878 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6289c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x628a4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x628ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x628bc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x628c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x628d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x628e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x628f0 | store | 4 | `sp -0x38` | `ret(0x5e9d0)` |
| `ldr` at 0x628f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x628f8 | load | 4 | `ret(0x5e9d0) +0x4` | `-` |
| `ldr` at 0x62904 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62908 | load | 4 | `ret(0x5e9d0) +0x78` | `-` |
| `ldr` at 0x62914 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62918 | load | 4 | `ret(0x5e9d0) +0x10` | `-` |
| `ldr` at 0x62924 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62928 | load | 4 | `ret(0x5e9d0) +0x10` | `-` |
| `ldr` at 0x62934 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62938 | load | 4 | `ret(0x5e9d0) +0x10` | `-` |
| `ldr` at 0x62944 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62948 | load | 4 | `ret(0x5e9d0) +0x10` | `-` |
| `ldr` at 0x62958 | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x6295c | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x6296c | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x62970 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62974 | load | 4 | `abs 0x62b64` | `-` |
| `ldr` at 0x62978 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62980 | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x62984 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62998 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x629a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x629b0 | store | 4 | `sp -0x20` | `ret(0x9ba98)` |
| `ldr` at 0x629b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x629c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x629d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x629d8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x629e0 | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x629e4 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x629f4 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x629f8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x629fc | load | 4 | `abs 0x62b68` | `-` |
| `str` at 0x62a08 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x62a0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62a1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62a34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62a4c | store | 4 | `sp -0x38` | `ret(0x5e9d0)` |
| `str` at 0x62a5c | store | 4 | `sp -0x3c` | `ret(0x5e9d0)` |
| `ldr` at 0x62a60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x62a64 | load | 4 | `ret(0x5e9d0) +0x4` | `-` |
| `ldr` at 0x62a70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62a74 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x62a78 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x62a7c | load | 4 | `ret(0x5e9d0) +0x8` | `-` |
| `ldr` at 0x62a88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x62a8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x62a90 | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0x62a94 | load | 4 | `ret(0x5e9d0) +0xc` | `-` |
| `ldr` at 0x62aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62ab0 | load | 4 | `abs 0x62b4c` | `-` |
| `ldr` at 0x62ab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62ac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62af4 | load | 4 | `abs 0x62b38` | `-` |
| `ldr` at 0x62af8 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x62b08 | load | 4 | `abs 0x62b3c` | `-` |
| `ldr` at 0x62b0c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x62b10 | load | 4 | `abs 0x62b6c` | `-` |
| `ldr` at 0x62b14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62b1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62b24 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x62b2c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x62b34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x62b34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x62b34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6251c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7380` | `r2` | `0x116928` |
| `bl` at 0x62578 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7398` | `?` | `?` |
| `bl` at 0x62584 | `(unknown callee) 0x98d34` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x62590 | `(unknown callee) 0x9b9c4` | `0x281f20` | `[sp-20]` | `?` | `?` |
| `bl` at 0x625d4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7150` | `?` | `0x116928` |
| `bl` at 0x625dc | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x62618 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe73bc` | `[sp-28]` | `0x116928` |
| `bl` at 0x62648 | `(unknown callee) 0x5e9d0` | `[sp-68]` | `?` | `?` | `[sp-68]` |
| `bl` at 0x62658 | `(unknown callee) 0x63278` | `[sp-28]` | `?` | `?` | `ret(0x5e9d0)` |
| `bl` at 0x62734 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x62754 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x6278c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe73d8` | `?` | `0x116928` |
| `bl` at 0x627b8 | `(unknown callee) 0x63278` | `?` | `?` | `?` | `?` |
| `bl` at 0x627d0 | `(unknown callee) 0x98d34` | `[sp-20]` | `0x1` | `?` | `ret(0x63278)` |
| `bl` at 0x627fc | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x62814 | `(unknown callee) 0x98d34` | `[sp-20]` | `?` | `?` | `ret(0x9ba98)` |
| `bl` at 0x6282c | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x62850 | `(unknown callee) 0x63278` | `?` | `?` | `?` | `?` |
| `bl` at 0x62888 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x628b0 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x628d4 | `(unknown callee) 0x63278` | `?` | `?` | `?` | `?` |
| `bl` at 0x628e8 | `(unknown callee) 0x5e9d0` | `[sp-28]` | `?` | `?` | `ret(0x63278)` |
| `bl` at 0x6297c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7404` | `[sp-28]` | `0x116928` |
| `bl` at 0x629a8 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x629c8 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `ret(0x9ba98)` |
| `bl` at 0x62a00 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe742c` | `?` | `0x116928` |
| `bl` at 0x62a20 | `(unknown callee) 0x63278` | `?` | `?` | `?` | `?` |
| `bl` at 0x62a44 | `(unknown callee) 0x5e9d0` | `0x1` | `?` | `?` | `[sp-28]` |
| `bl` at 0x62a54 | `(unknown callee) 0x5e9d0` | `0x2` | `?` | `?` | `ret(0x5e9d0)` |
| `bl` at 0x62aac | `(unknown callee) 0x98d34` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x62ad8 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x62af0 | `(unknown callee) 0x98d34` | `[sp-20]` | `?` | `?` | `ret(0x9ba98)` |
| `bl` at 0x62b18 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7448` | `[sp-28]` | `0x116928` |
| `bl` at 0x62b30 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62480 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x624ec | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x624f8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x6250c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62544 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x625a4 | `[[0x11a754]+88] == 0x0` | `[[0x11a754]+88] != 0x0` |
| `bne` at 0x625b0 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x625c4 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x625f0 | `[sp-28]-1 > 0x1 (unsigned, 32-bit)` | `[sp-28]-1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x62604 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x62624 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `beq` at 0x62664 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x62674 | `[[sp-56]+4] == 0x0` | `[[sp-56]+4] != 0x0` |
| `beq` at 0x62684 | `[[sp-56]+16] == 0x35` | `[[sp-56]+16] != 0x35` |
| `beq` at 0x62694 | `[[sp-56]+16] == 0x4f` | `[[sp-56]+16] != 0x4f` |
| `beq` at 0x626a4 | `[[sp-56]+16] == 0x42` | `[[sp-56]+16] != 0x42` |
| `beq` at 0x626b4 | `[[sp-56]+16] == 0x46` | `[[sp-56]+16] != 0x46` |
| `beq` at 0x626c4 | `[[sp-56]+16] == 0x4a` | `[[sp-56]+16] != 0x4a` |
| `beq` at 0x626d4 | `[[sp-56]+16] == 0x4b` | `[[sp-56]+16] != 0x4b` |
| `beq` at 0x626e4 | `[[sp-56]+16] == 0x21` | `[[sp-56]+16] != 0x21` |
| `beq` at 0x626f4 | `[[sp-56]+16] == 0x5e` | `[[sp-56]+16] != 0x5e` |
| `beq` at 0x62704 | `[[sp-56]+16] == 0x4c` | `[[sp-56]+16] != 0x4c` |
| `bne` at 0x62748 | `ret(0x9ba98) != 0x0` | `ret(0x9ba98) == 0x0` |
| `ble` at 0x62768 | `[0x11a0cc] <= 0x0 (signed, 32-bit)` | `[0x11a0cc] > 0x0 (signed, 32-bit)` |
| `beq` at 0x6277c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62798 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `ble` at 0x627ac | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x627c4 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x62808 | `ret(0x9ba98) == 0x0` | `ret(0x9ba98) != 0x0` |
| `ble` at 0x62844 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x6285c | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `bne` at 0x62894 | `ret(0x9ba98) != 0x0` | `ret(0x9ba98) == 0x0` |
| `ble` at 0x628c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x628e0 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x62900 | `[ret(0x5e9d0)+4] == 0x0` | `[ret(0x5e9d0)+4] != 0x0` |
| `bne` at 0x62910 | `[ret(0x5e9d0)+120] != 0x0` | `[ret(0x5e9d0)+120] == 0x0` |
| `beq` at 0x62920 | `[ret(0x5e9d0)+16] == 0x52` | `[ret(0x5e9d0)+16] != 0x52` |
| `beq` at 0x62930 | `[ret(0x5e9d0)+16] == 0x44` | `[ret(0x5e9d0)+16] != 0x44` |
| `beq` at 0x62940 | `[ret(0x5e9d0)+16] == 0x36` | `[ret(0x5e9d0)+16] != 0x36` |
| `beq` at 0x62950 | `[ret(0x5e9d0)+16] == 0x40` | `[ret(0x5e9d0)+16] != 0x40` |
| `beq` at 0x62968 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x629bc | `ret(0x9ba98) != 0x0` | `ret(0x9ba98) == 0x0` |
| `beq` at 0x629f0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x62a14 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x62a2c | `ret(0x63278) != 0x0` | `ret(0x63278) == 0x0` |
| `bne` at 0x62a3c | `[sp-28] != 0x2` | `[sp-28] == 0x2` |
| `beq` at 0x62a6c | `[ret(0x5e9d0)+4] == 0x0` | `[ret(0x5e9d0)+4] != 0x0` |
| `bne` at 0x62a84 | `[[sp-56]+8] != [ret(0x5e9d0)+8]` | `[[sp-56]+8] == [ret(0x5e9d0)+8]` |
| `bne` at 0x62a9c | `[[sp-56]+12] != [ret(0x5e9d0)+12]` | `[[sp-56]+12] == [ret(0x5e9d0)+12]` |
| `beq` at 0x62ae4 | `ret(0x9ba98) == 0x0` | `ret(0x9ba98) != 0x0` |
| `beq` at 0x62b04 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x62b34 | `r0` | `?` |

## `B2BReqRsp`

- `B2BReqRsp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x62b70 to 0x63278.
  Established. The opening signed comparison branches for nonnegative `[[sp-20]+24]` and falls through
  when negative (`bge` at 0x62bc4). The computed transfer is taken when unsigned `[sp-60]-1 <= 0xe`, and falls through when
  greater (`ldrls` at 0x62c6c); its taken targets remain unresolved. On known edges, calls to
  `0x9bd94` have candidates `(0x281f20, 0xfffffda0, [sp-20], 0x0)` (`bl` at 0x63128) and `(0x281f20,
  0xfffffd9e, [sp-20], 0x0)` (`bl` at 0x631cc). The request/response encodings and delivery
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x62b80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x62b88 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x62b8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62b90 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x62ba0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x62ba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62ba8 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x62bb4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x62bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62bbc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x62bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62bcc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x62bd4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x62bd8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x62be0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x62be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62bec | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x62bf4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x62bf8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x62bfc | store | 4 | `sp -0x2c` | `[sp-56]` |
| `ldr` at 0x62c00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62c04 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x62c10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62c14 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x62c28 | store | 4 | `sp -0x3c` | `0xe` |
| `str` at 0x62c34 | store | 4 | `sp -0x3c` | `0xf` |
| `ldr` at 0x62c3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x62c40 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x62c48 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x62c4c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x62c54 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x62c58 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x62c5c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0x62c60 | load | 4 | `sp -0x30` | `-` |
| `ldrls` at 0x62c6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62fb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62fbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62fc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62fd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62fe0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62fec | load | 4 | `abs 0x63254` | `-` |
| `ldr` at 0x62ff4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62ff8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x63010 | store | 4 | `sp -0x34` | `ret(0x5e6f0)` |
| `ldr` at 0x63014 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63020 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x63024 | load | 4 | `ret(0x5e6f0) +0x10` | `-` |
| `str` at 0x63028 | store | 4 | `sp -0x20` | `[ret(0x5e6f0)+16]` |
| `ldr` at 0x6302c | load | 4 | `abs 0x63234` | `-` |
| `ldr` at 0x63030 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x63040 | load | 4 | `abs 0x63238` | `-` |
| `ldr` at 0x63044 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63048 | store | 4 | `sp -0x48` | `[ret(0x5e6f0)+16]` |
| `ldr` at 0x6304c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x63050 | load | 4 | `abs 0x63258` | `-` |
| `ldr` at 0x63054 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63064 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x63070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6307c | store | 4 | `sp -0x20` | `ret(0x63368)` |
| `ldr` at 0x63080 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x63090 | load | 4 | `abs 0x63234` | `-` |
| `ldr` at 0x63094 | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x630a4 | load | 4 | `abs 0x63238` | `-` |
| `ldr` at 0x630a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x630ac | store | 4 | `sp -0x48` | `ret(0x63368)` |
| `ldr` at 0x630b0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x630b4 | load | 4 | `abs 0x6325c` | `-` |
| `ldr` at 0x630b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x630bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x630c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x630d8 | load | 4 | `abs 0x63234` | `-` |
| `ldr` at 0x630dc | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x630ec | load | 4 | `abs 0x63238` | `-` |
| `ldr` at 0x630f0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x630f4 | load | 4 | `abs 0x63260` | `-` |
| `ldr` at 0x630f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x63100 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x63104 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x63114 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x63118 | load | 4 | `abs 0x6324c` | `-` |
| `ldr` at 0x6311c | load | 4 | `abs 0x63250` | `-` |
| `ldr` at 0x63120 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6312c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63138 | load | 4 | `abs 0x6324c` | `-` |
| `ldr` at 0x6313c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x63150 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6315c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63168 | store | 4 | `sp -0x18` | `ret(0x9ba98)` |
| `ldr` at 0x6316c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x63178 | load | 4 | `abs 0x63234` | `-` |
| `ldr` at 0x6317c | load | 4 | `abs 0x11a74c` | `-` |
| `ldr` at 0x6318c | load | 4 | `abs 0x63238` | `-` |
| `ldr` at 0x63190 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x63194 | load | 4 | `abs 0x63264` | `-` |
| `ldr` at 0x63198 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6319c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x631a4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x631a8 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x631b8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x631bc | load | 4 | `abs 0x6324c` | `-` |
| `ldr` at 0x631c0 | load | 4 | `abs 0x63268` | `-` |
| `ldr` at 0x631c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x631d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x631d8 | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x631dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x631e0 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x631ec | load | 4 | `abs 0x6326c` | `-` |
| `str` at 0x631f0 | store | 4 | `sp -0x44` | `0xe7574` |
| `ldr` at 0x631f8 | load | 4 | `abs 0x63270` | `-` |
| `str` at 0x631fc | store | 4 | `sp -0x44` | `0xe7578` |
| `ldr` at 0x63200 | load | 4 | `abs 0x63274` | `-` |
| `ldr` at 0x63204 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x63208 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6320c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x63214 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6321c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6321c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6321c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x62ff0 | `(unknown callee) 0x56518` | `0xe74c4` | `r1` | `r2` | `[sp-40]` |
| `bl` at 0x62ffc | `(unknown callee) 0x5e5f8` | `[sp-36]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x63008 | `(unknown callee) 0x5e6f0` | `ret(0x5e5f8)` | `?` | `?` | `ret(0x5e5f8)` |
| `bl` at 0x6305c | `(unknown callee) 0x21010` | `[0x116928]` | `0xe74d0` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x63060 | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `?` |
| `bl` at 0x63074 | `(unknown callee) 0x63368` | `[sp-20]` | `?` | `?` | `[sp-32]` |
| `bl` at 0x630c0 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe74f4` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x630c8 | `(unknown callee) 0x63278` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x630fc | `(unknown callee) 0x21010` | `[0x116928]` | `0xe7530` | `[sp-32]` | `0x116928` |
| `bl` at 0x63128 | `(unknown callee) 0x9bd94` | `0x281f20` | `0xfffffda0` | `[sp-20]` | `0x0` |
| `bl` at 0x63130 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x63160 | `(unknown callee) 0x9ba98` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x631a0 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe754c` | `[sp-32]` | `ret(0x9ba98)` |
| `bl` at 0x631cc | `(unknown callee) 0x9bd94` | `0x281f20` | `0xfffffd9e` | `[sp-20]` | `0x0` |
| `bl` at 0x631d4 | `(unknown callee) 0x1e71c` | `?` | `?` | `?` | `?` |
| `bl` at 0x63210 | `(unknown callee) 0x1dba4` | `0xe757c` | `[sp-32]` | `[sp-68]` | `ret(0x1e71c)` |
| `bl` at 0x63218 | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x62bc4 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bge` at 0x62c0c | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x62c20 | `? != 0x2` | `? == 0x2` |
| `ldrls` at 0x62c6c | `[sp-60]-1 <= 0xe (unsigned, 32-bit)` | `[sp-60]-1 > 0xe (unsigned, 32-bit)` |
| `bne` at 0x62fb8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62fc4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x62fd0 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x62fdc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x62fe8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6301c | `ret(0x5e6f0) == 0x0` | `ret(0x5e6f0) != 0x0` |
| `beq` at 0x6303c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6306c | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x63088 | `ret(0x63368) > 0x0 (signed, 32-bit)` | `ret(0x63368) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x630a0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x630d4 | `ret(0x63278) != 0x0` | `ret(0x63278) == 0x0` |
| `beq` at 0x630e8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6310c | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `beq` at 0x63174 | `ret(0x9ba98) == 0x0` | `ret(0x9ba98) != 0x0` |
| `beq` at 0x63188 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x631b0 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `bne` at 0x631e8 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6321c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x62c6c).

## `B2BActive`

- `B2BActive`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63278 to 0x63320.
  Established. A stack word at `sp -0x18` is first cleared (`str` at 0x63290) and later can receive
  one (`str` at 0x6330c). Three branches each take their arm for an unknown zero value and fall
  through otherwise (`beq` at 0x632b4, `beq` at 0x632dc, `beq` at 0x63304). The merged result in
  `r0` is unknown (`ldmdb` at 0x63318). No call is recorded, and the active-state meaning is not
  established by the symbol.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63288 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x63290 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x63294 | load | 4 | `abs 0x6331c` | `-` |
| `ldr` at 0x63298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x632ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x632b8 | load | 4 | `abs 0x6331c` | `-` |
| `ldr` at 0x632bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x632d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x632d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x632e0 | load | 4 | `abs 0x6331c` | `-` |
| `ldr` at 0x632e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x632f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x632fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x6330c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x63310 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x63318 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63318 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x632b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x632dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63304 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63318 | `r0` | `?` |

## `B2BConfigured`

- `B2BConfigured`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63320 to 0x63368.
  Established. The entry word is saved at `sp -0x14` (`str` at 0x63330). The eventual `r0` value
  remains unknown at the exit (`ldmdb` at 0x63360). The body records no call or conditional branch.
  It does not establish a configured-state predicate in these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63330 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x63334 | load | 4 | `abs 0x63364` | `-` |
| `ldr` at 0x63338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6334c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x63360 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63360 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63360 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63360 | `r0` | `?` |

## `IRouterInit`

- `IRouterInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a710 to 0x9a7e0.
  Established. The comparison branches on `[sp-21]:1 == 0`, with nonzero falling through (`beq` at
  0x9a734). The fall-through path writes word value one (`str` at 0x9a740), while the taken path writes
  two (`str` at 0x9a750), at `0x11b438`. Another word store writes an unknown value at `0x2824e0` (`str` at 0x9a77c). The call
  to `0x9a7e0` has candidates `(0x2824c0, 0x1, 0x3, unknown)` (`bl` at 0x9a78c). Router identity and
  authority encodings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a720 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a728 | store | 4 | `sp -0x18` | `0x1` |
| `ldrb` at 0x9a72c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x9a738 | load | 4 | `abs 0x9a7c4` | `-` |
| `str` at 0x9a740 | store | 4 | `abs 0x11b438` | `0x1` |
| `ldr` at 0x9a748 | load | 4 | `abs 0x9a7c4` | `-` |
| `str` at 0x9a750 | store | 4 | `abs 0x11b438` | `0x2` |
| `str` at 0x9a758 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x9a75c | load | 4 | `abs 0x9a7c8` | `-` |
| `ldr` at 0x9a760 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9a77c | store | 4 | `abs 0x2824e0` | `?` |
| `ldr` at 0x9a780 | load | 4 | `abs 0x9a7cc` | `-` |
| `ldr` at 0x9a790 | load | 4 | `abs 0x9a7d0` | `-` |
| `ldr` at 0x9a794 | load | 4 | `abs 0x9a7d4` | `-` |
| `ldr` at 0x9a7a4 | load | 4 | `abs 0x9a7d8` | `-` |
| `ldr` at 0x9a7a8 | load | 4 | `abs 0x9a7dc` | `-` |
| `ldr` at 0x9a7b8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x9a7c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9a7c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9a7c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a78c | `(unknown callee) 0x9a7e0` | `0x2824c0` | `0x1` | `0x3` | `?` |
| `bl` at 0x9a7a0 | `(unknown callee) 0x1f9e8` | `0x11b45c` | `0xfcf00` | `0x1` | `0x9` |
| `bl` at 0x9a7b4 | `(unknown callee) 0x1f9e8` | `0x11b4b0` | `0xfcf0c` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9a734 | `[sp-21]:1 == 0x0` | `[sp-21]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9a7c0 | `r0` | `[sp-24]` |

## `IRouterInitID`

- `IRouterInitID`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a7e0 to 0x9a8c8.
  Established. The first explicit word write to entry `r0 +0x0` has an unknown value (`str` at
  0x9a828). The later signed comparison takes the branch for `[[sp-20]] >= 0`, and falls through
  when negative (`bge` at 0x9a840). A byte store at `[sp-20] +0x4` contains `([sp-28]:1 & 0xff)`
  (`strb` at 0x9a8a8), followed by a halfword clear at `[sp-20] +0x6` (`strh` at 0x9a8b4). The exit
  records zero in `r0` (`ldmdb` at 0x9a8c0). Identifier layout and field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a7f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a7f4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9a7f8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x9a7fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a804 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x9a810 | load | 4 | `abs 0x9a8c4` | `-` |
| `ldr` at 0x9a814 | load | 4 | `abs 0x2824e0` | `-` |
| `str` at 0x9a828 | store | 4 | `r0 +0x0` | `?` |
| `ldr` at 0x9a82c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9a830 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x9a834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a838 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x9a844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a84c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x9a854 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9a864 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0x9a868 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9a870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a878 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x9a880 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9a88c | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0x9a890 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9a894 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a898 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9a89c | store | 4 | `[sp-32] +0x0` | `[sp-36]` |
| `ldr` at 0x9a8a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9a8a4 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x9a8a8 | store | 1 | `[sp-20] +0x4` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x9a8ac | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x9a8b4 | store | 2 | `[sp-20] +0x6` | `0x0` |
| `ldmdb` at 0x9a8c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9a8c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9a8c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x9a840 | `[[sp-20]] >= 0x0 (signed, 32-bit)` | `[[sp-20]] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9a8c0 | `r0` | `0x0` |

## `IRouterSetAuthority`

- `IRouterSetAuthority`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a8c8 to 0x9a8f8.
  Established. A byte at entry `r0 +0x4` receives `([sp-24]:1 & 0xff)` (`strb` at 0x9a8e8). The
  recorded exit value is zero (`ldmdb` at 0x9a8f4). There is no call or conditional branch record.
  The authority represented by that byte is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a8d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a8dc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9a8e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9a8e4 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x9a8e8 | store | 1 | `r0 +0x4` | `([sp-24]:1 & 0xff)` |
| `ldmdb` at 0x9a8f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9a8f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9a8f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9a8f4 | `r0` | `0x0` |

## `IRouterCreateQueue`

- `IRouterCreateQueue`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a8f8 to 0x9a93c.
  Established. A word store places one at `sp -0x20` (`str` at 0x9a918). The call to `0x2002c` has
  candidates `(entry r0, entry r1, entry r2, 0x7)` (`bl` at 0x9a92c); its result reaches the return
  (`ldmdb` at 0x9a938). No conditional branch appears, and queue allocation semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a908 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a90c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9a910 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9a918 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x9a91c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a924 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x9a938 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9a938 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9a938 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a92c | `(unknown callee) 0x2002c` | `r0` | `r1` | `r2` | `0x7` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9a938 | `r0` | `ret(0x2002c)` |

## `IRouterGetPacket`

- `IRouterGetPacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a93c to 0x9a998.
  Established. The call to `0x203bc` carries candidates `(entry r1, entry r0, entry r2, entry r3)`
  (`bl` at 0x9a964); its zero result takes the following branch, while nonzero falls through (`beq`
  at 0x9a978). The two paths store zero (`str` at 0x9a98c) or `ret(0x203bc)` (`str` at 0x9a980) in
  the word at `sp -0x28`. The exit reports `[sp-40]` (`ldmdb` at 0x9a994). The result cannot be
  assigned packet ownership from these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a94c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a950 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9a954 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x9a958 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a95c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a960 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9a96c | store | 4 | `sp -0x24` | `ret(0x203bc)` |
| `ldr` at 0x9a970 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a97c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9a980 | store | 4 | `sp -0x28` | `ret(0x203bc)` |
| `str` at 0x9a98c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9a990 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x9a994 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9a994 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9a994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a964 | `(unknown callee) 0x203bc` | `r1` | `r0` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9a978 | `ret(0x203bc) == 0x0` | `ret(0x203bc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9a994 | `r0` | `[sp-40]` |

## `IRouterFreePacket`

- `IRouterFreePacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a998 to 0x9ad60.
  Established. The halfword at `[sp-20] +0x4` receives `([sp-40]:2 & 0xffff)` (`strh` at 0x9ab68).
  Later word updates clear `0x11b454` (`str` at 0x9ac74) and write `[0x11b44c]-1` at `0x11b44c`
  (`str` at 0x9ac9c). The final call to `0x1fe64` uses first candidate `0x11b4b0` (`bl` at 0x9accc);
  its zero result takes the branch past the following call, while nonzero falls through
  to that call (`beq` at 0x9ace0). The exit reports zero
  (`ldmdb` at 0x9ad1c), but deallocation and reference-count semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a9a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9a9ac | load | 4 | `abs 0x9ad20` | `-` |
| `str` at 0x9a9bc | store | 4 | `sp -0x20` | `ret(0x1fb00)` |
| `ldr` at 0x9a9c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9a9cc | load | 4 | `abs 0x9ad24` | `-` |
| `str` at 0x9a9d0 | store | 4 | `sp -0x34` | `0xfce5c` |
| `ldr` at 0x9a9d4 | load | 4 | `abs 0x9ad20` | `-` |
| `str` at 0x9a9d8 | store | 4 | `sp -0x30` | `0x11b4b0` |
| `ldr` at 0x9a9dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9a9e0 | store | 4 | `sp -0x2c` | `ret(0x1fb00)` |
| `ldr` at 0x9a9e8 | load | 4 | `abs 0x9ad28` | `-` |
| `ldr` at 0x9a9ec | load | 4 | `abs 0x9ad2c` | `-` |
| `ldr` at 0x9a9f8 | load | 4 | `abs 0x9ad30` | `-` |
| `ldr` at 0x9a9fc | load | 4 | `abs 0x11b43c` | `-` |
| `ldr` at 0x9aa08 | load | 4 | `abs 0x9ad34` | `-` |
| `ldr` at 0x9aa0c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9aa10 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x9aa24 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9aa28 | load | 4 | `abs 0x9ad38` | `-` |
| `ldr` at 0x9aa2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9aa34 | load | 4 | `abs 0x9ad34` | `-` |
| `ldr` at 0x9aa38 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9aa40 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9aa44 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x9aa64 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9aa68 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x9aa6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9aa70 | load | 4 | `abs 0x9ad3c` | `-` |
| `ldr` at 0x9aa78 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9aa84 | load | 4 | `abs 0x9ad20` | `-` |
| `str` at 0x9aa90 | store | 4 | `sp -0x20` | `ret(0x1fe64)` |
| `ldr` at 0x9aa94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9aaa0 | load | 4 | `abs 0x9ad40` | `-` |
| `str` at 0x9aaa4 | store | 4 | `sp -0x34` | `0xfceb4` |
| `ldr` at 0x9aaa8 | load | 4 | `abs 0x9ad20` | `-` |
| `str` at 0x9aaac | store | 4 | `sp -0x30` | `0x11b4b0` |
| `ldr` at 0x9aab0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9aab4 | store | 4 | `sp -0x2c` | `ret(0x1fe64)` |
| `ldr` at 0x9aabc | load | 4 | `abs 0x9ad28` | `-` |
| `str` at 0x9aad0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9aad8 | load | 4 | `abs 0x9ad44` | `-` |
| `ldr` at 0x9aadc | load | 4 | `abs 0x11b454` | `-` |
| `ldr` at 0x9aae8 | load | 4 | `abs 0x9ad44` | `-` |
| `ldr` at 0x9aaec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9aaf0 | load | 4 | `abs 0x11b454` | `-` |
| `ldr` at 0x9aafc | load | 4 | `abs 0x9ad34` | `-` |
| `ldr` at 0x9ab00 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9ab04 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x9ab18 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9ab1c | load | 4 | `abs 0x9ad48` | `-` |
| `ldr` at 0x9ab20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ab28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ab2c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9ab30 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0x9ab3c | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0x9ab40 | store | 2 | `sp -0x28` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0x9ab44 | load | 4 | `abs 0x9ad4c` | `-` |
| `ldrh` at 0x9ab50 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0x9ab60 | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0x9ab64 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0x9ab68 | store | 2 | `[sp-20] +0x4` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0x9ab88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ab8c | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x9ab9c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9aba0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9aba4 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x9aba8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9abb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9abbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9abc4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9abcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9abd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9abdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9abe4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9abec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9abf4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9abfc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9ac04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ac08 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x9ac10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ac14 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0x9ac18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ac28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9ac2c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x9ac30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ac34 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9ac38 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9ac3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ac48 | load | 4 | `abs 0x9ad44` | `-` |
| `ldr` at 0x9ac4c | load | 4 | `abs 0x11b454` | `-` |
| `ldr` at 0x9ac58 | load | 4 | `abs 0x9ad44` | `-` |
| `ldr` at 0x9ac5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ac60 | load | 4 | `abs 0x11b454` | `-` |
| `ldr` at 0x9ac6c | load | 4 | `abs 0x9ad44` | `-` |
| `str` at 0x9ac74 | store | 4 | `abs 0x11b454` | `0x0` |
| `ldr` at 0x9ac78 | load | 4 | `abs 0x9ad34` | `-` |
| `ldr` at 0x9ac7c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9ac80 | load | 4 | `abs 0x9ad50` | `-` |
| `ldr` at 0x9ac84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ac8c | load | 4 | `abs 0x9ad54` | `-` |
| `ldr` at 0x9ac90 | load | 4 | `abs 0x9ad54` | `-` |
| `ldr` at 0x9ac94 | load | 4 | `abs 0x11b44c` | `-` |
| `str` at 0x9ac9c | store | 4 | `abs 0x11b44c` | `[0x11b44c]-1` |
| `ldr` at 0x9aca0 | load | 4 | `abs 0x9ad30` | `-` |
| `ldr` at 0x9aca4 | load | 4 | `abs 0x11b43c` | `-` |
| `ldr` at 0x9acb0 | load | 4 | `abs 0x9ad34` | `-` |
| `ldr` at 0x9acb4 | load | 4 | `abs 0x9ad54` | `-` |
| `ldr` at 0x9acb8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9acbc | load | 4 | `abs 0x9ad58` | `-` |
| `ldr` at 0x9acc0 | load | 4 | `abs 0x11b44c` | `-` |
| `ldr` at 0x9acc8 | load | 4 | `abs 0x9ad20` | `-` |
| `str` at 0x9acd4 | store | 4 | `sp -0x20` | `ret(0x1fe64)` |
| `ldr` at 0x9acd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ace4 | load | 4 | `abs 0x9ad40` | `-` |
| `str` at 0x9ace8 | store | 4 | `sp -0x34` | `0xfceb4` |
| `ldr` at 0x9acec | load | 4 | `abs 0x9ad20` | `-` |
| `str` at 0x9acf0 | store | 4 | `sp -0x30` | `0x11b4b0` |
| `ldr` at 0x9acf4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9acf8 | store | 4 | `sp -0x2c` | `ret(0x1fe64)` |
| `ldr` at 0x9ad00 | load | 4 | `abs 0x9ad28` | `-` |
| `ldr` at 0x9ad04 | load | 4 | `abs 0x9ad5c` | `-` |
| `str` at 0x9ad14 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9ad18 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x9ad1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9ad1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9ad1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a9b4 | `(unknown callee) 0x1fb00` | `0x11b4b0` | `?` | `r2` | `r3` |
| `bl` at 0x9a9f4 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x3d7` | `0x0` |
| `bl` at 0x9aa30 | `(unknown callee) 0x21010` | `[0x116928]` | `0xfcf18` | `[sp-20]` | `?` |
| `bl` at 0x9aa3c | `(unknown callee) 0x18818` | `[0x116928]` | `?` | `?` | `0x116928` |
| `bl` at 0x9aa5c | `(unknown callee) 0x1f740` | `?` | `?` | `?` | `?` |
| `bl` at 0x9aa80 | `(unknown callee) 0x1dba4` | `0xfcf28` | `[[sp-20]+22]:1` | `[[sp-20]+28]` | `ret(0x1f740)` |
| `bl` at 0x9aa88 | `(unknown callee) 0x1fe64` | `0x11b4b0` | `?` | `?` | `?` |
| `bl` at 0x9aac8 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x3e4` | `0x0` |
| `bl` at 0x9ab24 | `(unknown callee) 0x21010` | `[0x116928]` | `0xfcf54` | `[sp-20]` | `?` |
| `bl` at 0x9abf0 | `(unknown callee) 0x18be4` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0x9ac0c | `(unknown callee) 0x18be4` | `[[sp-20]+8]` | `?` | `[[sp-20]+32]:1` | `[sp-20]` |
| `bl` at 0x9ac40 | `(unknown callee) 0x18be4` | `?` | `?` | `?` | `?` |
| `bl` at 0x9ac88 | `(unknown callee) 0x21010` | `[0x116928]` | `0xfcf60` | `[sp-20]` | `0x116928` |
| `bl` at 0x9acc4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xfcf6c` | `[0x11b44c]` | `0x116928` |
| `bl` at 0x9accc | `(unknown callee) 0x1fe64` | `0x11b4b0` | `?` | `?` | `?` |
| `bl` at 0x9ad0c | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x414` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9a9c8 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x9aa04 | `[0x11b43c] == 0x0` | `[0x11b43c] != 0x0` |
| `bne` at 0x9aa58 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9aa9c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |
| `beq` at 0x9aae4 | `[0x11b454] == 0x0` | `[0x11b454] != 0x0` |
| `bne` at 0x9aaf8 | `[sp-20] != [0x11b454]` | `[sp-20] == [0x11b454]` |
| `bne` at 0x9ab84 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9ab94 | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |
| `blt` at 0x9abb0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x9abd4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9ac20 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9ac54 | `[0x11b454] == 0x0` | `[0x11b454] != 0x0` |
| `bne` at 0x9ac68 | `[sp-20] != [0x11b454]` | `[sp-20] == [0x11b454]` |
| `beq` at 0x9acac | `[0x11b43c] == 0x0` | `[0x11b43c] != 0x0` |
| `beq` at 0x9ace0 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9ad1c | `r0` | `0x0` |

## `IRouterSendPacket`

- `IRouterSendPacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b9c4 to 0x9ba98.
  Established. The comparison takes its signed less-than branch when `[r1+22]:1 < 0`, and falls
  through otherwise (`blt` at 0x9b9f0). Another branch takes the equality arm for `[r1+22]:1 == 0x2`
  and falls through otherwise (`beq` at 0x9ba08). Calls include `0x9b02c` with third candidate one (`bl` at 0x9ba1c) and
  `0x9bd94` with candidates `(0x2824c0, ret(0x9b02c), [sp-24], 0x0)` (`bl` at 0x9ba5c). A subsequent
  call to `0x9a998` uses `[sp-24]` as its first candidate (`bl` at 0x9ba84). Transmission and
  ownership effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9b9d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9b9d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9b9dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9b9e0 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0x9b9e4 | store | 4 | `sp -0x20` | `[r1+22]:1` |
| `ldr` at 0x9b9e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b9f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ba00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9ba10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ba14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ba24 | store | 4 | `sp -0x1c` | `ret(0x9b02c)` |
| `ldr` at 0x9ba28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ba34 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9ba38 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0x9ba48 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9ba4c | load | 4 | `abs 0x9ba94` | `-` |
| `ldr` at 0x9ba50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ba54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ba64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ba68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ba74 | store | 4 | `sp -0x1c` | `ret(0x9b67c)` |
| `str` at 0x9ba7c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9ba80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ba88 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x9ba90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9ba90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9ba90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ba1c | `(unknown callee) 0x9b02c` | `r0` | `r1` | `0x1` | `[r1+22]:1` |
| `bl` at 0x9ba5c | `(unknown callee) 0x9bd94` | `0x2824c0` | `ret(0x9b02c)` | `[sp-24]` | `0x0` |
| `bl` at 0x9ba6c | `(unknown callee) 0x9b67c` | `r0` | `r1` | `r2` | `[r1+22]:1` |
| `bl` at 0x9ba84 | `(unknown callee) 0x9a998` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x9b9f0 | `[r1+22]:1 < 0x0 (signed, 32-bit)` | `[r1+22]:1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x9b9fc | `[r1+22]:1 <= 0x1 (signed, 32-bit)` | `[r1+22]:1 > 0x1 (signed, 32-bit)` |
| `beq` at 0x9ba08 | `[r1+22]:1 == 0x2` | `[r1+22]:1 != 0x2` |
| `beq` at 0x9ba30 | `ret(0x9b02c) == 0x0` | `ret(0x9b02c) != 0x0` |
| `bne` at 0x9ba40 | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9ba90 | `r0` | `[sp-28]` |

## `IRouterQPacket`

- `IRouterQPacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ba98 to 0x9bc5c.
  Established. The first comparison branches for `[r1+22]:1 != 0x2`, with equality falling through
  (`bne` at 0x9bac4). The taken path puts `[0x1156f0]` (`str` at 0x9bae0), while fall-through puts `[0x1156f4]`
  (`str` at 0x9bad0), in the word at `sp -0x24`. A call to `0x20710` has candidates `([sp-20], sp-24, [sp-36],
  0x9ade0)` (`bl` at 0x9bb38). Explicit word stores also place `0x9af18` at `sp -0x6c` (`str` at
  0x9bb14) and `sp-32` at `sp -0x68` (`str` at 0x9bb1c). The stored `0x9af18` and candidate `0x9ade0` do not establish calls
  to those addresses. Queue and suspension semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9baa8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9baac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9bab4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9bab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9babc | load | 1 | `r1 +0x16` | `-` |
| `ldr` at 0x9bac8 | load | 4 | `abs 0x9bc34` | `-` |
| `ldr` at 0x9bacc | load | 4 | `abs 0x1156f4` | `-` |
| `str` at 0x9bad0 | store | 4 | `sp -0x24` | `[0x1156f4]` |
| `ldr` at 0x9bad8 | load | 4 | `abs 0x9bc38` | `-` |
| `ldr` at 0x9badc | load | 4 | `abs 0x1156f0` | `-` |
| `str` at 0x9bae0 | store | 4 | `sp -0x24` | `[0x1156f0]` |
| `str` at 0x9baf4 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x9bb08 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9bb10 | load | 4 | `abs 0x9bc3c` | `-` |
| `str` at 0x9bb14 | store | 4 | `sp -0x6c` | `0x9af18` |
| `str` at 0x9bb1c | store | 4 | `sp -0x68` | `sp-32` |
| `str` at 0x9bb24 | store | 4 | `sp -0x64` | `0x1` |
| `ldr` at 0x9bb28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bb30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bb34 | load | 4 | `abs 0x9bc40` | `-` |
| `str` at 0x9bb40 | store | 4 | `sp -0x28` | `ret(0x20710)` |
| `ldr` at 0x9bb44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9bb50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9bb54 | store | 4 | `sp -0x60` | `ret(0x20710)` |
| `ldr` at 0x9bb58 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9bb64 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x9bb74 | load | 4 | `abs 0x9bc44` | `-` |
| `str` at 0x9bb78 | store | 4 | `sp -0x28` | `0xfffffd9d` |
| `str` at 0x9bb80 | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0x9bb84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9bb8c | store | 4 | `sp -0x58` | `[sp-20]+120` |
| `ldr` at 0x9bb90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9bb94 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x9bba0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9bba4 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x9bbb0 | load | 4 | `abs 0x9bc48` | `-` |
| `str` at 0x9bbb4 | store | 4 | `sp -0x5c` | `0xfcf80` |
| `ldr` at 0x9bbbc | load | 4 | `abs 0x9bc4c` | `-` |
| `str` at 0x9bbc0 | store | 4 | `sp -0x5c` | `0xfd018` |
| `ldr` at 0x9bbc8 | load | 4 | `abs 0x9bc50` | `-` |
| `str` at 0x9bbcc | store | 4 | `sp -0x5c` | `0xfcf7c` |
| `ldr` at 0x9bbd0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x9bbd4 | load | 4 | `abs 0x9bc54` | `-` |
| `ldr` at 0x9bbd8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x9bbdc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x9bbf4 | load | 4 | `abs 0x9bc58` | `-` |
| `str` at 0x9bbf8 | store | 4 | `sp -0x28` | `0xfffffd9e` |
| `ldr` at 0x9bc08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bc20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bc28 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x9bc30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9bc30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9bc30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9bb38 | `(unknown callee) 0x20710` | `[sp-20]` | `sp-24` | `[sp-36]` | `0x9ade0` |
| `bl` at 0x9bbe0 | `(unknown callee) 0x18c74` | `sp-80` | `0xfd01c` | `[sp-20]+120` | `?` |
| `bl` at 0x9bbec | `(unknown callee) 0x1d6d8` | `sp-80` | `?` | `?` | `sp-80` |
| `bl` at 0x9bc24 | `(unknown callee) 0x9a998` | `?` | `sp-16` | `sp-16` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9bac4 | `[r1+22]:1 != 0x2` | `[r1+22]:1 == 0x2` |
| `beq` at 0x9bb4c | `ret(0x20710) == 0x0` | `ret(0x20710) != 0x0` |
| `beq` at 0x9bb60 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9bb6c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9bb9c | `[[sp-24]+22]:1 == 0x0` | `[[sp-24]+22]:1 != 0x0` |
| `bne` at 0x9bbac | `[[sp-24]+22]:1 != 0x1` | `[[sp-24]+22]:1 == 0x1` |
| `beq` at 0x9bc10 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9bc30 | `r0` | `[sp-40]` |

## `IRouterRequest`

- `IRouterRequest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9bc5c to 0x9bd94.
  Established. The first zero comparison takes its branch for `[sp+8] == 0`, and falls through
  otherwise (`beq` at 0x9bc84). That fall-through writes the word `[r0+6]:2` at `[sp+8] +0x0` (`str`
  at 0x9bc94). A later halfword store has unknown value and effective target (`strh` at 0x9bcc8).
  The call to `0x99f58` has candidates `([sp-48], 0x0, 0x0, [sp-52])` (`bl` at 0x9bd40); its zero
  result takes the branch into the remaining call sequence, while nonzero falls through
  to the path that skips those calls (`beq` at 0x9bd54). Request identity and
  response matching are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9bc6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9bc70 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9bc74 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9bc78 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9bc7c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9bc88 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9bc8c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9bc90 | load | 2 | `r0 +0x6` | `-` |
| `str` at 0x9bc94 | store | 4 | `[sp+8] +0x0` | `[r0+6]:2` |
| `str` at 0x9bc9c | store | 4 | `sp -0x30` | `sp-40` |
| `ldr` at 0x9bca0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9bca4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9bca8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9bcac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9bcb0 | store | 4 | `sp -0x58` | `?` |
| `str` at 0x9bcb8 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x9bcbc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9bcc0 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x9bcc8 | store | 2 | `(unknown)` | `?` |
| `str` at 0x9bcd4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x9bcd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9bcdc | store | 4 | `sp -0x4c` | `[sp-24]` |
| `ldr` at 0x9bce0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bcec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9bcf0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x9bcfc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9bd00 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0x9bd08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9bd0c | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x9bd10 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x9bd14 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x9bd18 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x9bd1c | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0x9bd20 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x9bd24 | store | 4 | `sp -0x40` | `[sp+4]` |
| `ldr` at 0x9bd28 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x9bd2c | store | 4 | `sp -0x3c` | `[sp+12]` |
| `ldr` at 0x9bd30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9bd3c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x9bd48 | store | 4 | `sp -0x24` | `ret(0x99f58)` |
| `ldr` at 0x9bd4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bd58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9bd5c | store | 4 | `sp -0x2c` | `ret(0x99f58)` |
| `ldr` at 0x9bd64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bd68 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9bd78 | store | 4 | `sp -0x24` | `ret(0x9b02c)` |
| `ldr` at 0x9bd7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9bd84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9bd88 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x9bd8c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x9bd90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9bd90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9bd90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9bd40 | `(unknown callee) 0x99f58` | `[sp-48]` | `0x0` | `0x0` | `[sp-52]` |
| `bl` at 0x9bd70 | `(unknown callee) 0x9b02c` | `[sp-20]` | `[sp-40]` | `0x1` | `ret(0x99f58)` |
| `bl` at 0x9bd80 | `(unknown callee) 0x9a998` | `[sp-40]` | `?` | `?` | `ret(0x9b02c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9bc84 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0x9bce8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x9bcf8 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `beq` at 0x9bd54 | `ret(0x99f58) == 0x0` | `ret(0x99f58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9bd90 | `r0` | `?` |
