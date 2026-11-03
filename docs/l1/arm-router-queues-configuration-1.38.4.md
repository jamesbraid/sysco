# ARM router queues and configuration paths in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/router-queues-configuration-facts.txt) retain all tool sections.

## `IRouterPacketUseCount`

- `IRouterPacketUseCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3a34 to 0xb3b9c.
  Established. A halfword store writes `([sp-32]:2 & 0xffff)` at `[sp-20] +0x4` (`strh` at 0xb3ae0).
  The absolute-word branch takes the bypass for `[0x149afc] == 0x0` and falls through otherwise
  (`beq` at 0xb3af0). The later call to `0x22dc0` has `0x149b58` and three unknown candidates (`bl`
  at 0xb3b34). Use-count encoding and the joined r0 value remain unknown (`ldmdb` at 0xb3b78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3a44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb3a48 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb3a4c | load | 4 | `abs 0xb3b7c` | `-` |
| `str` at 0xb3a5c | store | 4 | `sp -0x1c` | `ret(0x22a38)` |
| `ldr` at 0xb3a60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3a6c | load | 4 | `abs 0xb3b80` | `-` |
| `str` at 0xb3a70 | store | 4 | `sp -0x2c` | `0x1280fc` |
| `ldr` at 0xb3a74 | load | 4 | `abs 0xb3b7c` | `-` |
| `str` at 0xb3a78 | store | 4 | `sp -0x28` | `0x149b58` |
| `ldr` at 0xb3a7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3a80 | store | 4 | `sp -0x24` | `ret(0x22a38)` |
| `ldr` at 0xb3a88 | load | 4 | `abs 0xb3b84` | `-` |
| `ldr` at 0xb3a98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3a9c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb3aa0 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xb3aac | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xb3ab4 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xb3ab8 | store | 2 | `sp -0x20` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xb3abc | load | 4 | `abs 0xb3b88` | `-` |
| `ldrh` at 0xb3ac8 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xb3ad8 | store | 2 | `sp -0x20` | `?` |
| `ldrh` at 0xb3adc | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xb3ae0 | store | 2 | `[sp-20] +0x4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xb3ae4 | load | 4 | `abs 0xb3b8c` | `-` |
| `ldr` at 0xb3ae8 | load | 4 | `abs 0x149afc` | `-` |
| `ldr` at 0xb3af4 | load | 4 | `abs 0xb3b8c` | `-` |
| `ldr` at 0xb3af8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3afc | load | 4 | `abs 0x149afc` | `-` |
| `ldr` at 0xb3b08 | load | 4 | `abs 0xb3b90` | `-` |
| `ldr` at 0xb3b0c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb3b10 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb3b20 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb3b24 | load | 4 | `abs 0xb3b94` | `-` |
| `ldr` at 0xb3b28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb3b30 | load | 4 | `abs 0xb3b7c` | `-` |
| `str` at 0xb3b3c | store | 4 | `sp -0x1c` | `ret(0x22dc0)` |
| `ldr` at 0xb3b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3b4c | load | 4 | `abs 0xb3b98` | `-` |
| `str` at 0xb3b50 | store | 4 | `sp -0x2c` | `0x128154` |
| `ldr` at 0xb3b54 | load | 4 | `abs 0xb3b7c` | `-` |
| `str` at 0xb3b58 | store | 4 | `sp -0x28` | `0x149b58` |
| `ldr` at 0xb3b5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3b60 | store | 4 | `sp -0x24` | `ret(0x22dc0)` |
| `ldr` at 0xb3b68 | load | 4 | `abs 0xb3b84` | `-` |
| `ldmdb` at 0xb3b78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb3b78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb3b78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3a54 | `(unknown callee) 0x22a38` | `0x149b58` | `?` | `r2` | `r3` |
| `bl` at 0xb3a94 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x52` | `0x0` |
| `bl` at 0xb3b2c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x128148` | `[sp-20]` | `?` |
| `bl` at 0xb3b34 | `(unknown callee) 0x22dc0` | `0x149b58` | `?` | `?` | `?` |
| `bl` at 0xb3b74 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb3a68 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0xb3af0 | `[0x149afc] == 0x0` | `[0x149afc] != 0x0` |
| `bne` at 0xb3b04 | `[sp-20] != [0x149afc]` | `[sp-20] == [0x149afc]` |
| `beq` at 0xb3b48 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb3b78 | `r0` | `?` |

## `IRouterRegisterFunction`

- `IRouterRegisterFunction`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4178 to 0xb4220.
  Established. The first call to `0xb3b9c` carries entry r0, entry r1, `0x1`, `0x0` (`bl` at
  0xb41c8). A separate stack word store before it carries entry r3 at `sp -0x28` (`str` at 0xb41b4).
  The later call to the same target has unknown, unknown, `0x2`, `0x0` (`bl` at 0xb4208). The
  intermediate bypass branch retains only `Z == 0` for its taken edge and falls through otherwise
  (`bne` at 0xb41dc). Registration semantics and the r0 return remain unproven (`ldmdb` at 0xb421c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4188 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb418c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4190 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4194 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb419c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb41a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb41b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb41b4 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xb41b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb41bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb41d0 | store | 4 | `sp -0x24` | `ret(0xb3b9c)` |
| `ldr` at 0xb41d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb41e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb41f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb41f4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb41f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb41fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4210 | store | 4 | `sp -0x24` | `ret(0xb3b9c)` |
| `ldr` at 0xb4214 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb421c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb421c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb421c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb41c8 | `(unknown callee) 0xb3b9c` | `r0` | `r1` | `0x1` | `0x0` |
| `bl` at 0xb4208 | `(unknown callee) 0xb3b9c` | `?` | `?` | `0x2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb41ac | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb41dc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb41ec | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb421c | `r0` | `?` |

## `IRouterRegisterQueue`

- `IRouterRegisterQueue`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4220 to 0xb42c8.
  Established. The call at the first selected site reaches `0xb3b9c` with entry r0, entry r1, `0x1`,
  `0x1` (`bl` at 0xb4270). A later call to that target uses unknown, unknown, `0x2`, `0x1` (`bl` at
  0xb42b0). The prior word store at `sp -0x28` has an unknown value (`str` at 0xb429c). Queue
  registration effects and the joined return are unknown (`ldmdb` at 0xb42c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4230 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4234 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4238 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb423c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb4244 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb4248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4258 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb425c | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xb4260 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4264 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4278 | store | 4 | `sp -0x24` | `ret(0xb3b9c)` |
| `ldr` at 0xb427c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4298 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb429c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb42a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb42a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb42b8 | store | 4 | `sp -0x24` | `ret(0xb3b9c)` |
| `ldr` at 0xb42bc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb42c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb42c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb42c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4270 | `(unknown callee) 0xb3b9c` | `r0` | `r1` | `0x1` | `0x1` |
| `bl` at 0xb42b0 | `(unknown callee) 0xb3b9c` | `?` | `?` | `0x2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4254 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb4284 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb4294 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb42c4 | `r0` | `?` |

## `IRouterEventRegisterFunction`

- `IRouterEventRegisterFunction`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4604 to 0xb464c.
  Established. The only call reaches `0xb42c8` with entry r0, entry r1, entry r2, `0x0` (`bl` at
  0xb463c). Before it, a word store writes entry r3 at `sp -0x24` (`str` at 0xb4628). The callee
  result supplies r0 at the return (`ldmdb` at 0xb4648). Callback registration and stack argument
  count are not established by this forwarding body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4614 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4618 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb461c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4620 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb4624 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4628 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb462c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4634 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb4648 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4648 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4648 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb463c | `(unknown callee) 0xb42c8` | `r0` | `r1` | `r2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4648 | `r0` | `ret(0xb42c8)` |

## `IRouterEventRegisterQueue`

- `IRouterEventRegisterQueue`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb464c to 0xb4694.
  Established. A word store saves entry r3 at `sp -0x24` before the call to `0xb42c8` (`str` at
  0xb4670, `bl` at 0xb4684). That call has entry r0, entry r1, entry r2, `0x1` (`bl` at 0xb4684).
  The returned r0 value is `ret(0xb42c8)` (`ldmdb` at 0xb4690). No conditional branches are
  reported, and event queue semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb465c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4660 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4664 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb4668 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb466c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb4670 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb4674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb467c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb4690 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4690 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4684 | `(unknown callee) 0xb42c8` | `r0` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4690 | `r0` | `ret(0xb42c8)` |

## `IRouterUnregister`

- `IRouterUnregister`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4694 to 0xb4a04.
  Established. One word store writes `[[sp-36]+40]` at `0x2b2080`; another arm writes that value at
  `[sp-40] +0x28` (`str` at 0xb4858, `str` at 0xb486c). A later pair writes `[[sp-48]+24]` at
  `0x2b1cc0` or `[sp-52] +0x18` on separate arms (`str` at 0xb4934, `str` at 0xb4948). A later call to
  `0x1b400` has `[sp-48]`, `[[sp-48]]`, `[[sp-48]+24]`, `[[sp-48]+24]`
  (`bl` at 0xb4964). The return has r0 `0x0`; registration ownership and physical effects remain
  unidentified (`ldmdb` at 0xb49d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb46a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb46a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb46ac | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb46b0 | load | 4 | `abs 0xb49d8` | `-` |
| `ldr` at 0xb46b4 | load | 4 | `abs 0x2b2080` | `-` |
| `str` at 0xb46b8 | store | 4 | `sp -0x24` | `[0x2b2080]` |
| `str` at 0xb46c0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb46c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb46cc | load | 4 | `abs 0xb49dc` | `-` |
| `ldr` at 0xb46d0 | load | 4 | `abs 0x2b1cc0` | `-` |
| `str` at 0xb46d4 | store | 4 | `sp -0x30` | `[0x2b1cc0]` |
| `str` at 0xb46dc | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xb46e4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb46e8 | load | 4 | `abs 0xb49e0` | `-` |
| `str` at 0xb46f8 | store | 4 | `sp -0x3c` | `ret(0x22a38)` |
| `ldr` at 0xb46fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb4708 | load | 4 | `abs 0xb49e4` | `-` |
| `str` at 0xb470c | store | 4 | `sp -0x48` | `0x1280fc` |
| `ldr` at 0xb4710 | load | 4 | `abs 0xb49e0` | `-` |
| `str` at 0xb4714 | store | 4 | `sp -0x44` | `0x149b04` |
| `ldr` at 0xb4718 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb471c | store | 4 | `sp -0x40` | `ret(0x22a38)` |
| `ldr` at 0xb4724 | load | 4 | `abs 0xb49e8` | `-` |
| `ldr` at 0xb4728 | load | 4 | `abs 0xb49ec` | `-` |
| `ldr` at 0xb4734 | load | 4 | `abs 0xb49f0` | `-` |
| `ldr` at 0xb4738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb473c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4740 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4748 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb475c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb4760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4770 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb477c | load | 4 | `abs 0xb49f4` | `-` |
| `ldr` at 0xb4780 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb4798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb479c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb47a0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb47ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb47c0 | load | 4 | `abs 0xb49f4` | `-` |
| `ldr` at 0xb47d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb47e0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb47ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb47fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4804 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xb4808 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb4814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4820 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4824 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xb4828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4838 | load | 4 | `abs 0xb49d8` | `-` |
| `ldr` at 0xb483c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4840 | load | 4 | `abs 0x2b2080` | `-` |
| `ldr` at 0xb484c | load | 4 | `abs 0xb49d8` | `-` |
| `ldr` at 0xb4850 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4854 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb4858 | store | 4 | `abs 0x2b2080` | `[[sp-36]+40]` |
| `ldr` at 0xb4860 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb4864 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4868 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb486c | store | 4 | `[sp-40] +0x28` | `[[sp-36]+40]` |
| `ldr` at 0xb4870 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb4874 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xb4878 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb487c | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb4880 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xb4884 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb4890 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb4894 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xb4898 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb489c | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb48a0 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xb48a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb48b4 | load | 4 | `abs 0xb49f8` | `-` |
| `ldr` at 0xb48b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb48bc | load | 4 | `abs 0x2b1c80` | `-` |
| `ldr` at 0xb48c0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb48cc | load | 4 | `abs 0xb49f8` | `-` |
| `ldr` at 0xb48dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb48ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb48fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb4900 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4904 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xb4908 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb4914 | load | 4 | `abs 0xb49dc` | `-` |
| `ldr` at 0xb4918 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb491c | load | 4 | `abs 0x2b1cc0` | `-` |
| `ldr` at 0xb4928 | load | 4 | `abs 0xb49dc` | `-` |
| `ldr` at 0xb492c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb4930 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb4934 | store | 4 | `abs 0x2b1cc0` | `[[sp-48]+24]` |
| `ldr` at 0xb493c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb4940 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb4944 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb4948 | store | 4 | `[sp-52] +0x18` | `[[sp-48]+24]` |
| `ldr` at 0xb494c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb4950 | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xb4954 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb4958 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb495c | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xb4960 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb496c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb4970 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xb4974 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb4978 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb497c | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xb4984 | load | 4 | `abs 0xb49e0` | `-` |
| `str` at 0xb4990 | store | 4 | `sp -0x3c` | `ret(0x22dc0)` |
| `ldr` at 0xb4994 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb49a0 | load | 4 | `abs 0xb49fc` | `-` |
| `str` at 0xb49a4 | store | 4 | `sp -0x48` | `0x128154` |
| `ldr` at 0xb49a8 | load | 4 | `abs 0xb49e0` | `-` |
| `str` at 0xb49ac | store | 4 | `sp -0x44` | `0x149b04` |
| `ldr` at 0xb49b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb49b4 | store | 4 | `sp -0x40` | `ret(0x22dc0)` |
| `ldr` at 0xb49bc | load | 4 | `abs 0xb49e8` | `-` |
| `ldr` at 0xb49c0 | load | 4 | `abs 0xb4a00` | `-` |
| `ldmdb` at 0xb49d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb49d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb49d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb46f0 | `(unknown callee) 0x22a38` | `0x149b04` | `?` | `r2` | `0x0` |
| `bl` at 0xb4730 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x167` | `0x0` |
| `bl` at 0xb4744 | `(unknown callee) 0x2070c` | `0x128184` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xb47d4 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `0x24` | `?` |
| `bl` at 0xb4888 | `(unknown callee) 0x1b400` | `[sp-36]` | `[[sp-36]]` | `[[sp-36]+40]` | `[[sp-36]+40]` |
| `bl` at 0xb48d8 | `(unknown callee) 0x1b43c` | `0x2b1c80` | `0x0` | `0x24` | `[[sp-20]]` |
| `bl` at 0xb4964 | `(unknown callee) 0x1b400` | `[sp-48]` | `[[sp-48]]` | `[[sp-48]+24]` | `[[sp-48]+24]` |
| `bl` at 0xb4988 | `(unknown callee) 0x22dc0` | `0x149b04` | `?` | `?` | `?` |
| `bl` at 0xb49c8 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x1a6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4704 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0xb4754 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xb4768 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb4778 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xb47a8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb47f4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0xb4810 | `[[sp-36]] != [[sp-20]]` | `[[sp-36]] == [[sp-20]]` |
| `beq` at 0xb481c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xb4830 | `[[sp-36]+4] == [sp-24]` | `[[sp-36]+4] != [sp-24]` |
| `bne` at 0xb4848 | `[sp-36] != [0x2b2080]` | `[sp-36] == [0x2b2080]` |
| `bne` at 0xb48b0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xb48c8 | `[0x2b1c80] != [[sp-20]]` | `[0x2b1c80] == [[sp-20]]` |
| `beq` at 0xb48e8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb48f4 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0xb4910 | `[[sp-48]] != [[sp-20]]` | `[[sp-48]] == [[sp-20]]` |
| `bne` at 0xb4924 | `[sp-48] != [0x2b1cc0]` | `[sp-48] == [0x2b1cc0]` |
| `beq` at 0xb499c | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb49d4 | `r0` | `0x0` |

## `IRouterPacketCount`

- `IRouterPacketCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4a04 to 0xb4b2c.
  Established. Two word stores write `([0x149af0] + [sp-20])` at `0x149af0` and `([0x149af4] +
  [sp-20])` at `0x149af4` (`str` at 0xb4a78, `str` at 0xb4a90). The signed branch takes the bypass
  for `[0x149af4] <= [0x149af8]` and falls through for greater (`ble` at 0xb4aa8). Fall-through
  stores word `[0x149af4]` at `0x149af8` (`str` at 0xb4ab8). Counter units and the r0 value remain
  unknown (`ldmdb` at 0xb4b04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4a14 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb4a18 | load | 4 | `abs 0xb4b08` | `-` |
| `str` at 0xb4a28 | store | 4 | `sp -0x18` | `ret(0x22a38)` |
| `ldr` at 0xb4a2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4a38 | load | 4 | `abs 0xb4b0c` | `-` |
| `str` at 0xb4a3c | store | 4 | `sp -0x24` | `0x1280fc` |
| `ldr` at 0xb4a40 | load | 4 | `abs 0xb4b08` | `-` |
| `str` at 0xb4a44 | store | 4 | `sp -0x20` | `0x149b58` |
| `ldr` at 0xb4a48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4a4c | store | 4 | `sp -0x1c` | `ret(0x22a38)` |
| `ldr` at 0xb4a54 | load | 4 | `abs 0xb4b10` | `-` |
| `ldr` at 0xb4a58 | load | 4 | `abs 0xb4b14` | `-` |
| `ldr` at 0xb4a64 | load | 4 | `abs 0xb4b18` | `-` |
| `ldr` at 0xb4a68 | load | 4 | `abs 0xb4b18` | `-` |
| `ldr` at 0xb4a6c | load | 4 | `abs 0x149af0` | `-` |
| `ldr` at 0xb4a70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb4a78 | store | 4 | `abs 0x149af0` | `([0x149af0] + [sp-20])` |
| `ldr` at 0xb4a7c | load | 4 | `abs 0xb4b1c` | `-` |
| `ldr` at 0xb4a80 | load | 4 | `abs 0xb4b1c` | `-` |
| `ldr` at 0xb4a84 | load | 4 | `abs 0x149af4` | `-` |
| `ldr` at 0xb4a88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb4a90 | store | 4 | `abs 0x149af4` | `([0x149af4] + [sp-20])` |
| `ldr` at 0xb4a94 | load | 4 | `abs 0xb4b1c` | `-` |
| `ldr` at 0xb4a98 | load | 4 | `abs 0xb4b20` | `-` |
| `ldr` at 0xb4a9c | load | 4 | `abs 0x149af4` | `-` |
| `ldr` at 0xb4aa0 | load | 4 | `abs 0x149af8` | `-` |
| `ldr` at 0xb4aac | load | 4 | `abs 0xb4b20` | `-` |
| `ldr` at 0xb4ab0 | load | 4 | `abs 0xb4b1c` | `-` |
| `ldr` at 0xb4ab4 | load | 4 | `abs 0x149af4` | `-` |
| `str` at 0xb4ab8 | store | 4 | `abs 0x149af8` | `[0x149af4]` |
| `ldr` at 0xb4abc | load | 4 | `abs 0xb4b08` | `-` |
| `str` at 0xb4ac8 | store | 4 | `sp -0x18` | `ret(0x22dc0)` |
| `ldr` at 0xb4acc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb4ad8 | load | 4 | `abs 0xb4b24` | `-` |
| `str` at 0xb4adc | store | 4 | `sp -0x24` | `0x128154` |
| `ldr` at 0xb4ae0 | load | 4 | `abs 0xb4b08` | `-` |
| `str` at 0xb4ae4 | store | 4 | `sp -0x20` | `0x149b58` |
| `ldr` at 0xb4ae8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb4aec | store | 4 | `sp -0x1c` | `ret(0x22dc0)` |
| `ldr` at 0xb4af4 | load | 4 | `abs 0xb4b10` | `-` |
| `ldr` at 0xb4af8 | load | 4 | `abs 0xb4b28` | `-` |
| `ldmdb` at 0xb4b04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4b04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4a20 | `(unknown callee) 0x22a38` | `0x149b58` | `?` | `r2` | `r3` |
| `bl` at 0xb4a60 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x1b5` | `0x0` |
| `bl` at 0xb4ac0 | `(unknown callee) 0x22dc0` | `0x149b58` | `[0x149af4]` | `?` | `?` |
| `bl` at 0xb4b00 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x1bd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4a34 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `ble` at 0xb4aa8 | `[0x149af4] <= [0x149af8] (signed, 32-bit)` | `[0x149af4] > [0x149af8] (signed, 32-bit)` |
| `beq` at 0xb4ad4 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4b04 | `r0` | `?` |

## `IRouterFreePacket`

- `IRouterFreePacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb56f4 to 0xb5ac0.
  Established. A halfword store writes `([sp-40]:2 & 0xffff)` at `[sp-20] +0x4` (`strh` at 0xb58c4).
  Later word stores clear `0x149afc` to `0x0` and write `[0x149af4]-1` at `0x149af4` (`str` at
  0xb59d0, `str` at 0xb59f8). The call to `0x1b400` has `[[sp-20]+8]`,
  unknown, `[[sp-20]+32]:1`, `[sp-20]` (`bl` at 0xb5968). Both final paths return r0 `0x0`; packet
  ownership and reference-count encoding are unproved (`ldmdb` at 0xb5a78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5704 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5708 | load | 4 | `abs 0xb5a7c` | `-` |
| `str` at 0xb5718 | store | 4 | `sp -0x20` | `ret(0x22a38)` |
| `ldr` at 0xb571c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5728 | load | 4 | `abs 0xb5a80` | `-` |
| `str` at 0xb572c | store | 4 | `sp -0x34` | `0x1280fc` |
| `ldr` at 0xb5730 | load | 4 | `abs 0xb5a7c` | `-` |
| `str` at 0xb5734 | store | 4 | `sp -0x30` | `0x149b58` |
| `ldr` at 0xb5738 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb573c | store | 4 | `sp -0x2c` | `ret(0x22a38)` |
| `ldr` at 0xb5744 | load | 4 | `abs 0xb5a84` | `-` |
| `ldr` at 0xb5748 | load | 4 | `abs 0xb5a88` | `-` |
| `ldr` at 0xb5754 | load | 4 | `abs 0xb5a8c` | `-` |
| `ldr` at 0xb5758 | load | 4 | `abs 0x149ae4` | `-` |
| `ldr` at 0xb5764 | load | 4 | `abs 0xb5a90` | `-` |
| `ldr` at 0xb5768 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb576c | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb5780 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb5784 | load | 4 | `abs 0xb5a94` | `-` |
| `ldr` at 0xb5788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5790 | load | 4 | `abs 0xb5a90` | `-` |
| `ldr` at 0xb5794 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb579c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb57a0 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb57c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb57c4 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xb57c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb57cc | load | 4 | `abs 0xb5a98` | `-` |
| `ldr` at 0xb57d4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xb57e0 | load | 4 | `abs 0xb5a7c` | `-` |
| `str` at 0xb57ec | store | 4 | `sp -0x20` | `ret(0x22dc0)` |
| `ldr` at 0xb57f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb57fc | load | 4 | `abs 0xb5a9c` | `-` |
| `str` at 0xb5800 | store | 4 | `sp -0x34` | `0x128154` |
| `ldr` at 0xb5804 | load | 4 | `abs 0xb5a7c` | `-` |
| `str` at 0xb5808 | store | 4 | `sp -0x30` | `0x149b58` |
| `ldr` at 0xb580c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb5810 | store | 4 | `sp -0x2c` | `ret(0x22dc0)` |
| `ldr` at 0xb5818 | load | 4 | `abs 0xb5a84` | `-` |
| `ldr` at 0xb581c | load | 4 | `abs 0xb5aa0` | `-` |
| `str` at 0xb582c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb5834 | load | 4 | `abs 0xb5aa4` | `-` |
| `ldr` at 0xb5838 | load | 4 | `abs 0x149afc` | `-` |
| `ldr` at 0xb5844 | load | 4 | `abs 0xb5aa4` | `-` |
| `ldr` at 0xb5848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb584c | load | 4 | `abs 0x149afc` | `-` |
| `ldr` at 0xb5858 | load | 4 | `abs 0xb5a90` | `-` |
| `ldr` at 0xb585c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb5860 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb5874 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb5878 | load | 4 | `abs 0xb5aa8` | `-` |
| `ldr` at 0xb587c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5888 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb588c | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xb5898 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xb589c | store | 2 | `sp -0x28` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xb58a0 | load | 4 | `abs 0xb5aac` | `-` |
| `ldrh` at 0xb58ac | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb58bc | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0xb58c0 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb58c4 | store | 2 | `[sp-20] +0x4` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xb58e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb58e8 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xb58f8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb58fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb5900 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xb5904 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5918 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5920 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb5928 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5934 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5938 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5940 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb5948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5950 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb5958 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb5960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5964 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb596c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5970 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0xb5974 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5984 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb5988 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb598c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5990 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb5994 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb5998 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb59a4 | load | 4 | `abs 0xb5aa4` | `-` |
| `ldr` at 0xb59a8 | load | 4 | `abs 0x149afc` | `-` |
| `ldr` at 0xb59b4 | load | 4 | `abs 0xb5aa4` | `-` |
| `ldr` at 0xb59b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb59bc | load | 4 | `abs 0x149afc` | `-` |
| `ldr` at 0xb59c8 | load | 4 | `abs 0xb5aa4` | `-` |
| `str` at 0xb59d0 | store | 4 | `abs 0x149afc` | `0x0` |
| `ldr` at 0xb59d4 | load | 4 | `abs 0xb5a90` | `-` |
| `ldr` at 0xb59d8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb59dc | load | 4 | `abs 0xb5ab0` | `-` |
| `ldr` at 0xb59e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb59e8 | load | 4 | `abs 0xb5ab4` | `-` |
| `ldr` at 0xb59ec | load | 4 | `abs 0xb5ab4` | `-` |
| `ldr` at 0xb59f0 | load | 4 | `abs 0x149af4` | `-` |
| `str` at 0xb59f8 | store | 4 | `abs 0x149af4` | `[0x149af4]-1` |
| `ldr` at 0xb59fc | load | 4 | `abs 0xb5a8c` | `-` |
| `ldr` at 0xb5a00 | load | 4 | `abs 0x149ae4` | `-` |
| `ldr` at 0xb5a0c | load | 4 | `abs 0xb5a90` | `-` |
| `ldr` at 0xb5a10 | load | 4 | `abs 0xb5ab4` | `-` |
| `ldr` at 0xb5a14 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb5a18 | load | 4 | `abs 0xb5ab8` | `-` |
| `ldr` at 0xb5a1c | load | 4 | `abs 0x149af4` | `-` |
| `ldr` at 0xb5a24 | load | 4 | `abs 0xb5a7c` | `-` |
| `str` at 0xb5a30 | store | 4 | `sp -0x20` | `ret(0x22dc0)` |
| `ldr` at 0xb5a34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5a40 | load | 4 | `abs 0xb5a9c` | `-` |
| `str` at 0xb5a44 | store | 4 | `sp -0x34` | `0x128154` |
| `ldr` at 0xb5a48 | load | 4 | `abs 0xb5a7c` | `-` |
| `str` at 0xb5a4c | store | 4 | `sp -0x30` | `0x149b58` |
| `ldr` at 0xb5a50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb5a54 | store | 4 | `sp -0x2c` | `ret(0x22dc0)` |
| `ldr` at 0xb5a5c | load | 4 | `abs 0xb5a84` | `-` |
| `ldr` at 0xb5a60 | load | 4 | `abs 0xb5abc` | `-` |
| `str` at 0xb5a70 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb5a74 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb5a78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5a78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5a78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5710 | `(unknown callee) 0x22a38` | `0x149b58` | `?` | `r2` | `r3` |
| `bl` at 0xb5750 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x3d9` | `0x0` |
| `bl` at 0xb578c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1281b8` | `[sp-20]` | `?` |
| `bl` at 0xb5798 | `(unknown callee) 0x1b01c` | `[0x144dc8]` | `?` | `?` | `0x144dc8` |
| `bl` at 0xb57b8 | `(unknown callee) 0x22698` | `?` | `?` | `?` | `?` |
| `bl` at 0xb57dc | `(unknown callee) 0x2070c` | `0x1281c8` | `[[sp-20]+22]:1` | `[[sp-20]+28]` | `ret(0x22698)` |
| `bl` at 0xb57e4 | `(unknown callee) 0x22dc0` | `0x149b58` | `?` | `?` | `?` |
| `bl` at 0xb5824 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x3e6` | `0x0` |
| `bl` at 0xb5880 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1281f4` | `[sp-20]` | `?` |
| `bl` at 0xb594c | `(unknown callee) 0x1b400` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0xb5968 | `(unknown callee) 0x1b400` | `[[sp-20]+8]` | `?` | `[[sp-20]+32]:1` | `[sp-20]` |
| `bl` at 0xb599c | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0xb59e4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x128200` | `[sp-20]` | `0x144dc8` |
| `bl` at 0xb5a20 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x12820c` | `[0x149af4]` | `0x144dc8` |
| `bl` at 0xb5a28 | `(unknown callee) 0x22dc0` | `0x149b58` | `?` | `?` | `?` |
| `bl` at 0xb5a68 | `(unknown callee) 0x21e28` | `0x9` | `0x128128` | `0x416` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb5724 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0xb5760 | `[0x149ae4] == 0x0` | `[0x149ae4] != 0x0` |
| `bne` at 0xb57b4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb57f8 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0xb5840 | `[0x149afc] == 0x0` | `[0x149afc] != 0x0` |
| `bne` at 0xb5854 | `[sp-20] != [0x149afc]` | `[sp-20] == [0x149afc]` |
| `bne` at 0xb58e0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb58f0 | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |
| `blt` at 0xb590c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb5930 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb597c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb59b0 | `[0x149afc] == 0x0` | `[0x149afc] != 0x0` |
| `bne` at 0xb59c4 | `[sp-20] != [0x149afc]` | `[sp-20] == [0x149afc]` |
| `beq` at 0xb5a08 | `[0x149ae4] == 0x0` | `[0x149ae4] != 0x0` |
| `beq` at 0xb5a3c | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5a78 | `r0` | `0x0` |

## `IRouterSetQueueSuspendTime`

- `IRouterSetQueueSuspendTime`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5ac0 to 0xb5afc.
  Established. Word stores write entry r0 at `0x1429ac` and `[sp-24]` at `0x1429b0` (`str` at
  0xb5ae0, `str` at 0xb5aec). The return retains entry r0 (`ldmdb` at 0xb5af0). There are no calls
  or conditional branches in the reported facts. Suspension units and how the stored words affect
  queues remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5ad0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5ad4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb5ad8 | load | 4 | `abs 0xb5af4` | `-` |
| `ldr` at 0xb5adc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5ae0 | store | 4 | `abs 0x1429ac` | `r0` |
| `ldr` at 0xb5ae4 | load | 4 | `abs 0xb5af8` | `-` |
| `ldr` at 0xb5ae8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb5aec | store | 4 | `abs 0x1429b0` | `[sp-24]` |
| `ldmdb` at 0xb5af0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5af0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5af0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5af0 | `r0` | `r0` |

## `IRouterGetQueueSuspendTime`

- `IRouterGetQueueSuspendTime`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5afc to 0xb5b40.
  Established. The first output word receives `[0x1429ac]` at entry `r0 +0x0` (`str` at 0xb5b20).
  The second word store writes `[0x1429b0]` at `[sp-24] +0x0` (`str` at 0xb5b30). The r0 return
  retains entry r0 (`ldmdb` at 0xb5b34). Time units, output-field meanings and queue effects remain
  unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5b0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5b10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb5b14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5b18 | load | 4 | `abs 0xb5b38` | `-` |
| `ldr` at 0xb5b1c | load | 4 | `abs 0x1429ac` | `-` |
| `str` at 0xb5b20 | store | 4 | `r0 +0x0` | `[0x1429ac]` |
| `ldr` at 0xb5b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5b28 | load | 4 | `abs 0xb5b3c` | `-` |
| `ldr` at 0xb5b2c | load | 4 | `abs 0x1429b0` | `-` |
| `str` at 0xb5b30 | store | 4 | `[sp-24] +0x0` | `[0x1429b0]` |
| `ldmdb` at 0xb5b34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5b34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5b34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5b34 | `r0` | `r0` |

## `EvtIRouter`

- `EvtIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68e88 to 0x68f7c.
  Established. The byte guard takes the bypass for `[r0+22]:1 != 0x0` and falls through on equality
  (`bne` at 0x68f14). The following halfword test takes the update arm for `[r0+34]:2 == 0x0` and
  falls through otherwise (`beq` at 0x68f24). That update writes byte `0x2` at entry `r0 +0x16`
  (`strb` at 0x68f34). The final call to `0xb56f4` has all four candidates unknown and supplies r0
  `ret(0xb56f4)` at the return (`bl` at 0x68f74, `ldmdb` at 0x68f78). Event identity and ownership
  effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68e98 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x68e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68ea0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x68ea4 | store | 4 | `sp -0x18` | `[r0+28]` |
| `ldr` at 0x68ea8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68eb4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x68ebc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x68ec0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x68ec8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x68ed0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x68ed8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x68edc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x68ee0 | store | 4 | `sp -0x1c` | `[sp-40]` |
| `ldr` at 0x68ee4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x68ef4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x68ef8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x68f04 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x68f08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68f0c | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x68f18 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x68f1c | load | 2 | `r0 +0x22` | `-` |
| `ldr` at 0x68f2c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x68f34 | store | 1 | `r0 +0x16` | `0x2` |
| `ldr` at 0x68f38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x68f3c | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x68f40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x68f4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x68f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68f70 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x68f78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x68f78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x68f78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68f60 | `(unknown callee) 0x729ec` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x68f6c | `(unknown callee) 0x857e4` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x68f74 | `(unknown callee) 0xb56f4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x68eb0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x68f14 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `beq` at 0x68f24 | `[r0+34]:2 == 0x0` | `[r0+34]:2 != 0x0` |
| `beq` at 0x68f48 | `[sp-28] == 0x4` | `[sp-28] != 0x4` |
| `beq` at 0x68f54 | `[sp-28] == 0xe` | `[sp-28] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x68f78 | `r0` | `ret(0xb56f4)` |

## `L1CfgIRouter`

- `L1CfgIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x729ec to 0x72aa8.
  Established. The first branch takes its alternate arm for `[r0+22]:1 != 0x2` and falls through on
  equality (`bne` at 0x72a14). Later byte tests take bypasses for `[r0+22]:1 != 0x0` and `[r0+22]:1
  != 0x1`, respectively, with equality falling through (`bne` at 0x72a74, `bne` at 0x72a90). Their
  following calls target `0x720ec` and `0x727a8`, respectively, each with entry r0, entry r1, entry
  r2, `[r0+22]:1` (`bl` at 0x72a7c, `bl` at 0x72a98). The r0 return is `ret(0xb56f4)`; protocol
  dispatch meanings remain unknown (`ldmdb` at 0x72aa4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x729fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x72a04 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x72a08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x72a0c | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x72a18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a1c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x72a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a2c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x72a44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a48 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x72a5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a68 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x72a6c | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x72a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a84 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x72a88 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x72a94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a9c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x72aa4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x72aa4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x72aa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72a60 | `(unknown callee) 0x72878` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x72a7c | `(unknown callee) 0x720ec` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x72a98 | `(unknown callee) 0x727a8` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x72aa0 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72a14 | `[r0+22]:1 != 0x2` | `[r0+22]:1 == 0x2` |
| `bge` at 0x72a24 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x72a3c | `? == 0xe` | `? != 0xe` |
| `beq` at 0x72a54 | `? == 0xe` | `? != 0xe` |
| `bne` at 0x72a74 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `bne` at 0x72a90 | `[r0+22]:1 != 0x1` | `[r0+22]:1 == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x72aa4 | `r0` | `ret(0xb56f4)` |

## `L1CfgGetL2`

- `L1CfgGetL2`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74ab8 to 0x74b30.
  Established. The opening test takes the bypass for `r0 == 0x0` and falls through otherwise (`beq`
  at 0x74adc). Fall-through stores word `[0x1489a0]` at entry `r0 +0x0` (`str` at 0x74aec). Later
  word stores have unknown targets and values `[0x1489a4]` and `[0x1489a8]`, respectively (`str` at
  0x74b08, `str` at 0x74b24). The return retains entry r0, while L2 record meanings are unknown
  (`ldmdb` at 0x74b28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74ac8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74acc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x74ad0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x74ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74ae4 | load | 4 | `abs 0x74b2c` | `-` |
| `ldr` at 0x74ae8 | load | 4 | `abs 0x1489a0` | `-` |
| `str` at 0x74aec | store | 4 | `r0 +0x0` | `[0x1489a0]` |
| `ldr` at 0x74af0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74b00 | load | 4 | `abs 0x74b2c` | `-` |
| `ldr` at 0x74b04 | load | 4 | `abs 0x1489a4` | `-` |
| `str` at 0x74b08 | store | 4 | `(unknown)` | `[0x1489a4]` |
| `ldr` at 0x74b0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74b18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74b1c | load | 4 | `abs 0x74b2c` | `-` |
| `ldr` at 0x74b20 | load | 4 | `abs 0x1489a8` | `-` |
| `str` at 0x74b24 | store | 4 | `(unknown)` | `[0x1489a8]` |
| `ldmdb` at 0x74b28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74b28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74b28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74adc | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x74af8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x74b14 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74b28 | `r0` | `r0` |

## `L1CfgTypeNonRoutered`

- `L1CfgTypeNonRoutered`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74b98 to 0x74c08.
  Established. Three equality tests take their shared arm for `r0 == 0x44`, `r0 == 0x40`, and `r0 ==
  0x53`, respectively; each falls through on inequality (`beq` at 0x74bbc, `beq` at 0x74bd4, `beq`
  at 0x74be4). That arm stores word `0x0` at `sp -0x18`; the other result arm stores word `0x1`
  there (`str` at 0x74bf0, `str` at 0x74bfc). The joined r0 value remains unknown, as do the type
  encodings and routing meaning (`ldmdb` at 0x74c04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74ba8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x74bac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74bb0 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x74bb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74bc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74bcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74bdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x74bf0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x74bfc | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x74c00 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x74c04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74c04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74c04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74bbc | `r0 == 0x44` | `r0 != 0x44` |
| `bgt` at 0x74bc8 | `r0 > 0x44 (signed, 32-bit)` | `r0 <= 0x44 (signed, 32-bit)` |
| `beq` at 0x74bd4 | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0x74be4 | `r0 == 0x53` | `r0 != 0x53` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74c04 | `r0` | `?` |

## `L1CfgPortNonRoutered`

- `L1CfgPortNonRoutered`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74c08 to 0x74c78.
  Established. The signed bound takes the zero arm for `r0 >= [0x1489b0]` and falls through for less
  (`bge` at 0x74c2c). The zero arm stores word `0x0` at `sp -0x18` (`str` at 0x74c68). Fall-through
  includes a word read at `0x1489b4` and a later word read with unknown target (`ldr` at 0x74c50,
  `ldr` at 0x74c58). Port layout, routing classification and the returned r0 value remain unproven
  (`ldmdb` at 0x74c70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74c18 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x74c1c | load | 4 | `abs 0x74c74` | `-` |
| `ldr` at 0x74c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74c24 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x74c30 | load | 4 | `abs 0x74c74` | `-` |
| `ldr` at 0x74c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74c50 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74c58 | load | 4 | `(unknown)` | `-` |
| `str` at 0x74c5c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x74c68 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x74c6c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x74c70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74c70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74c70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x74c2c | `r0 >= [0x1489b0] (signed, 32-bit)` | `r0 < [0x1489b0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74c70 | `r0` | `?` |
