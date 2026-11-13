# ARM FRU events and dump paths in 1.40.5

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
`L1FRUSHUBDeadlock`, `FRUStatus` and `FruFmtMultiRec` have unresolved computed
transfers. Their access, call and return lists cover known edges only. Unknown data
values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/fru-events-dump-facts.txt) retain all tool sections.

## `L1FRUSHUBDeadlock`

- `L1FRUSHUBDeadlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa902c to 0xa998c.
  Established. The absolute word assignment at `0x2b28a0` writes `0x1e` (`str` at 0xa9088). The
  unnamed `0xeae04` call carries `ret(0x4869c)`, unknown, `sp-116`, `sp-120` (`bl` at 0xa92c4); its
  zero result takes `0xa92e0`, while nonzero falls through (`beq` at 0xa92d8). Deadlock effects
  beyond known edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa903c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa9040 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa9044 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa904c | store | 4 | `sp -0x48` | `0x1e` |
| `str` at 0xa9054 | store | 4 | `sp -0x4c` | `0x5` |
| `str` at 0xa905c | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0xa9064 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xa906c | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0xa9070 | load | 4 | `abs 0xa9960` | `-` |
| `ldr` at 0xa9074 | load | 4 | `abs 0x14a730` | `-` |
| `ldr` at 0xa9080 | load | 4 | `abs 0xa9964` | `-` |
| `str` at 0xa9088 | store | 4 | `abs 0x2b28a0` | `0x1e` |
| `ldr` at 0xa908c | load | 4 | `abs 0xa9968` | `-` |
| `ldr` at 0xa9090 | load | 4 | `abs 0xa9964` | `-` |
| `ldr` at 0xa9094 | load | 4 | `abs 0x2b28a0` | `-` |
| `str` at 0xa90a8 | store | 4 | `abs 0x2b28c0` | `?` |
| `ldr` at 0xa90ac | load | 4 | `abs 0xa996c` | `-` |
| `ldr` at 0xa90b0 | load | 4 | `abs 0xa9968` | `-` |
| `ldr` at 0xa90b4 | load | 4 | `abs 0x2b28c0` | `-` |
| `str` at 0xa90c0 | store | 4 | `abs 0x14a734` | `ret(0x1b1b4)` |
| `ldr` at 0xa90c4 | load | 4 | `abs 0xa9960` | `-` |
| `str` at 0xa90cc | store | 4 | `abs 0x14a730` | `0x1` |
| `ldr` at 0xa90e4 | load | 4 | `abs 0xa996c` | `-` |
| `ldr` at 0xa90e8 | load | 4 | `abs 0xa9968` | `-` |
| `ldr` at 0xa90ec | load | 4 | `abs 0x14a734` | `-` |
| `ldr` at 0xa90f4 | load | 4 | `abs 0x2b28c0` | `-` |
| `str` at 0xa9100 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xa9108 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa910c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9110 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa9120 | load | 4 | `abs 0xa9970` | `-` |
| `ldr` at 0xa9124 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9138 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa913c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa9144 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa9148 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa9150 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa9158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa915c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa9168 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa916c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa918c | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xa9190 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa9194 | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0xa9198 | load | 4 | `sp -0x60` | `-` |
| `strb` at 0xa91a0 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa91a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa91ac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa91b8 | store | 4 | `sp -0x70` | `ret(0x1b2bc)` |
| `ldr` at 0xa91bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa91c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa91c4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa91d8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa91e0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa91f0 | store | 4 | `[sp-28] +0x0` | `ret(0x1b358)` |
| `ldr` at 0xa91f4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa91f8 | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0xa91fc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9200 | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa9208 | store | 4 | `sp -0x60` | `([[sp-28]] + [sp-112])` |
| `str` at 0xa9214 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xa921c | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xa9220 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa9224 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa9234 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa9244 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xa924c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa9254 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xa9264 | store | 4 | `sp -0x70` | `0x2` |
| `str` at 0xa9270 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0xa9278 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa927c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9288 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa928c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa929c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa92a8 | load | 4 | `abs 0xa9970` | `-` |
| `ldr` at 0xa92ac | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa92bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xa92cc | store | 4 | `sp -0x24` | `ret(0xeae04)` |
| `ldr` at 0xa92d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa92e0 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0xa92e8 | store | 1 | `sp -0x80` | `?` |
| `ldr` at 0xa92ec | load | 4 | `sp -0x74` | `-` |
| `strb` at 0xa92f4 | store | 1 | `sp -0x7f` | `?` |
| `ldr` at 0xa92f8 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0xa9300 | store | 1 | `sp -0x7e` | `?` |
| `ldrb` at 0xa9304 | load | 1 | `sp -0x74` | `-` |
| `strb` at 0xa9308 | store | 1 | `sp -0x7d` | `([sp-116]:1 & 0xff)` |
| `ldr` at 0xa930c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xa9314 | store | 1 | `sp -0x7c` | `?` |
| `ldr` at 0xa9318 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xa9320 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0xa9324 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xa932c | store | 1 | `sp -0x7a` | `?` |
| `ldrb` at 0xa9330 | load | 1 | `sp -0x78` | `-` |
| `strb` at 0xa9334 | store | 1 | `sp -0x79` | `([sp-120]:1 & 0xff)` |
| `ldr` at 0xa9338 | load | 4 | `abs 0xa9970` | `-` |
| `ldr` at 0xa933c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9350 | load | 4 | `(unknown)` | `-` |
| `ldrls` at 0xa9358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa95a8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa95b0 | store | 4 | `sp -0x58` | `[sp-88]+1` |
| `ldr` at 0xa95b8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa95c0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa95dc | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa95e4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa95f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa95fc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9600 | load | 4 | `abs 0xa9974` | `-` |
| `ldr` at 0xa9604 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa960c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa961c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa9624 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b2bc))` |
| `str` at 0xa962c | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0xa9638 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa963c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9640 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa9650 | load | 4 | `abs 0xa9970` | `-` |
| `ldr` at 0xa9654 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9668 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0xa966c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa9674 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0xa967c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa9680 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa9690 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa96a0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa96b8 | load | 4 | `abs 0xa996c` | `-` |
| `ldr` at 0xa96bc | load | 4 | `abs 0x14a734` | `-` |
| `ldr` at 0xa96c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa96d4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa96dc | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa96e0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa96f0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa9708 | load | 4 | `abs 0xa996c` | `-` |
| `ldr` at 0xa970c | load | 4 | `abs 0x14a734` | `-` |
| `ldr` at 0xa9714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9724 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xa972c | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa9730 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa9738 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa9740 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa9748 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa9750 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa975c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9768 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa976c | store | 4 | `sp -0xcc` | `[sp-108]` |
| `ldr` at 0xa9770 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9774 | load | 4 | `abs 0xa9978` | `-` |
| `ldr` at 0xa9778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa977c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa9784 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9794 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa979c | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b2bc))` |
| `ldr` at 0xa97a4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xa97a8 | store | 4 | `sp -0xcc` | `[sp-104]` |
| `ldr` at 0xa97ac | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa97b0 | store | 4 | `sp -0xc8` | `[sp-108]` |
| `ldr` at 0xa97b4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa97b8 | load | 4 | `abs 0xa997c` | `-` |
| `ldr` at 0xa97bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa97c0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa97c8 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa97d8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa97e0 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b2bc))` |
| `str` at 0xa97e8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa97ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa97f8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa97fc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa9810 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa9814 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa9824 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9834 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa984c | load | 4 | `abs 0xa996c` | `-` |
| `ldr` at 0xa9850 | load | 4 | `abs 0x14a734` | `-` |
| `ldr` at 0xa9858 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa986c | load | 4 | `abs 0xa9970` | `-` |
| `ldr` at 0xa9870 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa9884 | load | 4 | `abs 0xa9980` | `-` |
| `ldr` at 0xa9888 | load | 4 | `abs 0xa9970` | `-` |
| `ldr` at 0xa988c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa98a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa98b0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa98b8 | load | 4 | `abs 0xa9984` | `-` |
| `ldr` at 0xa98bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa98c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa98cc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa98dc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa98f4 | load | 4 | `abs 0xa996c` | `-` |
| `ldr` at 0xa98f8 | load | 4 | `abs 0x14a734` | `-` |
| `ldr` at 0xa9900 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9904 | load | 4 | `abs 0xa9988` | `-` |
| `ldr` at 0xa990c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9914 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa9924 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa992c | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b2bc))` |
| `ldr` at 0xa9930 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa9938 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa9940 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa9948 | store | 4 | `sp -0x58` | `?` |
| `str` at 0xa9954 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0xa9958 | load | 4 | `sp -0xc4` | `-` |
| `ldmdb` at 0xa995c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa995c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa995c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa995c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa90b8 | `(unknown callee) 0x1b1b4` | `[0x2b28c0]` | `0x2b28c0` | `[0x2b28a0]` | `0x2b28c0` |
| `bl` at 0xa90e0 | `(unknown callee) 0x1b4cc` | `sp-68` | `0x0` | `0x20` | `sp-68` |
| `bl` at 0xa90f8 | `(unknown callee) 0x1b4cc` | `[0x14a734]` | `0x0` | `[0x2b28c0]` | `0x14a734` |
| `bl` at 0xa9184 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa91b0 | `(unknown callee) 0x1b2bc` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xa91e8 | `(unknown callee) 0x1b358` | `[[sp-28]]` | `?` | `?` | `?` |
| `bl` at 0xa920c | `(unknown callee) 0xe3cf8` | `?` | `?` | `[[sp-28]]` | `?` |
| `bl` at 0xa92a0 | `(unknown callee) 0x4869c` | `[sp-24]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xa92c4 | `(unknown callee) 0xeae04` | `ret(0x4869c)` | `?` | `sp-116` | `sp-120` |
| `bl` at 0xa95cc | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0xa95d4 | `(unknown callee) 0xa9b8c` | `0x64` | `?` | `?` | `?` |
| `bl` at 0xa95d8 | `(unknown callee) 0xe3cf8` | `ret(0xa9b8c)` | `?` | `?` | `?` |
| `bl` at 0xa95ec | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0xa9608 | `(unknown callee) 0x1b52c` | `[sp-96]` | `0x127978` | `[sp-24]` | `[sp-36]` |
| `bl` at 0xa9610 | `(unknown callee) 0x1b2bc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xa9780 | `(unknown callee) 0x1b52c` | `[sp-96]` | `0x12799c` | `[sp-24]` | `[sp-104]` |
| `bl` at 0xa9788 | `(unknown callee) 0x1b2bc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xa97c4 | `(unknown callee) 0x1b52c` | `[sp-96]` | `0x1279c4` | `[sp-24]` | `[sp-100]` |
| `bl` at 0xa97cc | `(unknown callee) 0x1b2bc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xa98c4 | `(unknown callee) 0x1b52c` | `sp-192` | `0x1279f0` | `?` | `?` |
| `bl` at 0xa9910 | `(unknown callee) 0x1b52c` | `[sp-96]` | `0x1279f8` | `sp-192` | `?` |
| `bl` at 0xa9918 | `(unknown callee) 0x1b2bc` | `[sp-96]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa907c | `[0x14a730] != 0x0` | `[0x14a730] == 0x0` |
| `blt` at 0xa9118 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa9164 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `blt` at 0xa922c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa923c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa925c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa9284 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xa9294 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa92d8 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `ldrls` at 0xa9358 | `? <= 0x8 (unsigned, 32-bit)` | `? > 0x8 (unsigned, 32-bit)` |
| `bge` at 0xa95c8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa95f8 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `blt` at 0xa9648 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xa9688 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa9758 | `[sp-100] != 0x0` | `[sp-100] == 0x0` |
| `beq` at 0xa9764 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0xa97f4 | `[sp-32] != 0x2` | `[sp-32] == 0x2` |
| `blt` at 0xa9804 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xa981c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa9860 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa995c | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa9358).

## `L1FRU_2m_Events`

- `L1FRU_2m_Events`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa761c to 0xa80a8.
  Established. The unnamed `0x75194` call carries `0x0`, `r1`, `r2`, `r3` (`bl` at 0xa7630). The
  absolute word at `0x2b2874` is read (`ldr` at 0xa76ac). The unnamed `0xc11d0` call at `0xa7cc4`
  carries `0x2b287c`, `[sp-104]`, `[sp-28]`, `[sp-116]` (`bl` at 0xa7cc4). Event construction
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7638 | store | 4 | `sp -0x98` | `ret(0x75194)` |
| `ldr` at 0xa763c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa7640 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `str` at 0xa7644 | store | 4 | `sp -0x1c` | `[ret(0x75194)+8]` |
| `ldr` at 0xa7648 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa764c | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0xa7650 | store | 4 | `sp -0x74` | `[ret(0x75194)+12]` |
| `ldr` at 0xa7654 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa7658 | load | 4 | `ret(0x75194) +0x10` | `-` |
| `str` at 0xa765c | store | 4 | `sp -0x78` | `[ret(0x75194)+16]` |
| `ldr` at 0xa7660 | load | 4 | `abs 0xa809c` | `-` |
| `strb` at 0xa766c | store | 1 | `sp -0x60` | `?` |
| `strb` at 0xa7674 | store | 1 | `sp -0x5f` | `?` |
| `strb` at 0xa767c | store | 1 | `sp -0x5e` | `?` |
| `strb` at 0xa7684 | store | 1 | `sp -0x5d` | `?` |
| `strb` at 0xa768c | store | 1 | `sp -0x5c` | `?` |
| `strb` at 0xa7694 | store | 1 | `sp -0x5b` | `?` |
| `strb` at 0xa769c | store | 1 | `sp -0x5a` | `?` |
| `strb` at 0xa76a4 | store | 1 | `sp -0x59` | `?` |
| `ldr` at 0xa76a8 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa76ac | load | 4 | `abs 0x2b2874` | `-` |
| `str` at 0xa76b0 | store | 4 | `sp -0x80` | `[0x2b2874]` |
| `str` at 0xa76b8 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xa76bc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa76c0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa76d0 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa76d4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa76e8 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa76f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa76f4 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xa76f8 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa76fc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7710 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7718 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa771c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xa7720 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa7724 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7738 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7740 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7744 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xa7748 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa774c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7760 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7768 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa776c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa7770 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa7774 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7788 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7790 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7794 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xa7798 | load | 4 | `abs 0xa80a4` | `-` |
| `str` at 0xa779c | store | 4 | `sp -0x2c` | `0xffff` |
| `str` at 0xa77a4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xa77a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa77ac | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa77bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa77d0 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa77d8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa77f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa77fc | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7804 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa7808 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa781c | store | 4 | `sp -0x2c` | `ret(0xbdc38)` |
| `ldr` at 0xa7824 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7838 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa7840 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa785c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7864 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa786c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa7870 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa7884 | store | 4 | `sp -0x2c` | `ret(0xbdc38)` |
| `ldr` at 0xa7888 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7890 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0xa789c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa78a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa78a4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa78b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa78bc | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa78c4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa78d4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa78dc | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa78e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa78e8 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa78f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa78fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7904 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa790c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7918 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa7928 | store | 4 | `sp -0x50` | `ret(0xbdc38)` |
| `ldr` at 0xa792c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa7938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7940 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa7948 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa795c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7964 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa796c | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7984 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0xa7988 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa798c | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0xa7990 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7998 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa799c | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa79a8 | store | 4 | `sp -0x4c` | `ret(0xbdf7c)` |
| `ldr` at 0xa79ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa79b0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa79bc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa79c4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa79cc | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa79d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa79d8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa79e0 | load | 1 | `[sp-104] +0x1` | `-` |
| `strb` at 0xa79e4 | store | 1 | `[sp-104] +0x0` | `([[sp-104]+1]:1 & 0xff)` |
| `ldr` at 0xa79ec | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa79f8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7a00 | load | 1 | `[sp-104] +0x2` | `-` |
| `strb` at 0xa7a04 | store | 1 | `[sp-104] +0x1` | `([[sp-104]+2]:1 & 0xff)` |
| `ldr` at 0xa7a0c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7a18 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7a20 | load | 1 | `[sp-104] +0x3` | `-` |
| `strb` at 0xa7a24 | store | 1 | `[sp-104] +0x2` | `([[sp-104]+3]:1 & 0xff)` |
| `ldr` at 0xa7a2c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7a38 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7a40 | load | 1 | `[sp-104] +0x4` | `-` |
| `strb` at 0xa7a44 | store | 1 | `[sp-104] +0x3` | `([[sp-104]+4]:1 & 0xff)` |
| `ldr` at 0xa7a4c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7a58 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7a60 | load | 1 | `[sp-104] +0x5` | `-` |
| `strb` at 0xa7a64 | store | 1 | `[sp-104] +0x4` | `([[sp-104]+5]:1 & 0xff)` |
| `ldr` at 0xa7a6c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7a78 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7a80 | load | 1 | `[sp-104] +0x6` | `-` |
| `strb` at 0xa7a84 | store | 1 | `[sp-104] +0x5` | `([[sp-104]+6]:1 & 0xff)` |
| `ldr` at 0xa7a8c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7a98 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7aa0 | load | 1 | `[sp-104] +0x7` | `-` |
| `strb` at 0xa7aa4 | store | 1 | `[sp-104] +0x6` | `([[sp-104]+7]:1 & 0xff)` |
| `ldr` at 0xa7aac | load | 4 | `sp -0x68` | `-` |
| `strb` at 0xa7ab8 | store | 1 | `[sp-104] +0x7` | `0x0` |
| `ldr` at 0xa7abc | load | 4 | `abs 0xa80a4` | `-` |
| `str` at 0xa7ac0 | store | 4 | `sp -0x28` | `0xffff` |
| `str` at 0xa7ac8 | store | 4 | `sp -0x30` | `0x3` |
| `str` at 0xa7ad0 | store | 4 | `sp -0x34` | `0x7` |
| `str` at 0xa7ad8 | store | 4 | `sp -0x38` | `0x3` |
| `str` at 0xa7ae0 | store | 4 | `sp -0x3c` | `0x7f` |
| `str` at 0xa7ae8 | store | 4 | `sp -0x40` | `0x7f` |
| `str` at 0xa7af0 | store | 4 | `sp -0x44` | `0x7f` |
| `str` at 0xa7af8 | store | 4 | `sp -0x48` | `0x7f` |
| `ldr` at 0xa7afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7b04 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7b0c | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7b20 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7b2c | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7b34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7b50 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa7b58 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7b6c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7b70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7b78 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7b80 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7b9c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa7ba4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7ba8 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xa7bac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7bb4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7bbc | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7bc0 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xa7bc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7bcc | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa7bd4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7be0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7be4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xa7be8 | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0xa7bec | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa7bf0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7c04 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7c0c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7c10 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0xa7c14 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa7c18 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7c2c | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7c34 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7c38 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0xa7c3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7c40 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0xa7c44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7c48 | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0xa7c4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa7c50 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0xa7c54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa7c58 | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0xa7c5c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa7c60 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0xa7c64 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa7c68 | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0xa7c6c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa7c70 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0xa7c74 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa7c78 | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0xa7c7c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa7c80 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0xa7c84 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa7c88 | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0xa7c8c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa7c90 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xa7c94 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7c98 | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0xa7c9c | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa7ca0 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0xa7ca4 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa7ca8 | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0xa7cac | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa7cb0 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0xa7cb4 | load | 4 | `abs 0xa809c` | `-` |
| `ldr` at 0xa7cb8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7cbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7cc0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xa7ccc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa7cd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7ce0 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa7ce8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7d00 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0xa7d04 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7d08 | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0xa7d0c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7d10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7d14 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa7d18 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7d24 | store | 4 | `sp -0x4c` | `ret(0xbdf7c)` |
| `ldr` at 0xa7d28 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7d2c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa7d38 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7d40 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7d48 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa7d4c | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7d50 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7d54 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7d58 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7d60 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7d68 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7d74 | store | 4 | `sp -0x28` | `ret(0xbfd9c)` |
| `ldr` at 0xa7d78 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7d7c | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7d80 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7d84 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7d8c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7d90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7d94 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7da0 | store | 4 | `sp -0x30` | `ret(0xbfd9c)` |
| `ldr` at 0xa7da4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7da8 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7dac | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7db0 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7db8 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7dbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7dc0 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7dcc | store | 4 | `sp -0x34` | `ret(0xbfd9c)` |
| `ldr` at 0xa7dd0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7dd4 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7dd8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7ddc | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7de4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7de8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7dec | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7df8 | store | 4 | `sp -0x38` | `ret(0xbfd9c)` |
| `ldr` at 0xa7dfc | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7e00 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7e04 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7e08 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7e10 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7e14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7e18 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7e24 | store | 4 | `sp -0x3c` | `ret(0xbfd9c)` |
| `ldr` at 0xa7e28 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7e2c | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7e30 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7e34 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7e3c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7e44 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7e50 | store | 4 | `sp -0x40` | `ret(0xbfd9c)` |
| `ldr` at 0xa7e54 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7e58 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7e5c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7e60 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7e68 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7e70 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7e7c | store | 4 | `sp -0x44` | `ret(0xbfd9c)` |
| `ldr` at 0xa7e80 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7e84 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7e88 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7e8c | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7e94 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7e98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7e9c | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7ea8 | store | 4 | `sp -0x48` | `ret(0xbfd9c)` |
| `ldr` at 0xa7eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7eb4 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7ebc | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7ed0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7edc | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7ee4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7f00 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa7f08 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7f1c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7f28 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7f30 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7f4c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa7f54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7f58 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xa7f5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7f64 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7f6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7f70 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xa7f74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7f7c | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa7f84 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7f90 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7f94 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xa7f98 | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0xa7f9c | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa7fa0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7fb4 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7fbc | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7fc0 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0xa7fc4 | load | 4 | `abs 0xa80a0` | `-` |
| `ldr` at 0xa7fc8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7fdc | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7fe4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7fe8 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0xa7fec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7ff0 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0xa7ff4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7ff8 | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0xa7ffc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa8000 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0xa8004 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa8008 | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0xa800c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa8010 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0xa8014 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa8018 | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0xa801c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa8020 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0xa8024 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa8028 | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0xa802c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa8030 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0xa8034 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa8038 | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0xa803c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa8040 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xa8044 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa8048 | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0xa804c | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa8050 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0xa8054 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa8058 | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0xa805c | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa8060 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0xa8064 | load | 4 | `abs 0xa809c` | `-` |
| `ldr` at 0xa8068 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa806c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8070 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xa8078 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8080 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xa8088 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xa8090 | store | 4 | `sp -0x7c` | `[sp-124]+1` |
| `ldmdb` at 0xa8098 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8098 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8098 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7630 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa7664 | `(unknown callee) 0xc1008` | `0x2b287c` | `?` | `?` | `[ret(0x75194)+16]` |
| `bl` at 0xa7814 | `(unknown callee) 0xbdc38` | `?` | `0x2a` | `0x20` | `?` |
| `bl` at 0xa787c | `(unknown callee) 0xbdc38` | `?` | `0x15` | `0x8` | `?` |
| `bl` at 0xa7920 | `(unknown callee) 0xbdc38` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xa79a0 | `(unknown callee) 0xbdf7c` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0xa7cc4 | `(unknown callee) 0xc11d0` | `0x2b287c` | `[sp-104]` | `[sp-28]` | `[sp-116]` |
| `bl` at 0xa7d1c | `(unknown callee) 0xbdf7c` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0xa7d6c | `(unknown callee) 0xbfd9c` | `0x0` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7d98 | `(unknown callee) 0xbfd9c` | `0x1` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7dc4 | `(unknown callee) 0xbfd9c` | `0x2` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7df0 | `(unknown callee) 0xbfd9c` | `0x7` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7e1c | `(unknown callee) 0xbfd9c` | `0x3` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7e48 | `(unknown callee) 0xbfd9c` | `0x4` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7e74 | `(unknown callee) 0xbfd9c` | `0x5` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7ea0 | `(unknown callee) 0xbfd9c` | `0x6` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa8074 | `(unknown callee) 0xc11d0` | `0x2b287c` | `[sp-104]` | `[sp-28]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa76c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xa77b4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa77f0 | `? != 0x2` | `? == 0x2` |
| `bne` at 0xa7858 | `? != 0x34` | `? == 0x34` |
| `blt` at 0xa78ac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa7934 | `ret(0xbdc38) == 0x0` | `ret(0xbdc38) != 0x0` |
| `ble` at 0xa7958 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xa797c | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bhi` at 0xa79b8 | `ret(0xbdf7c) > [sp-132] (unsigned, 32-bit)` | `ret(0xbdf7c) <= [sp-132] (unsigned, 32-bit)` |
| `beq` at 0xa7cd4 | `ret(0xbdc38) == 0x0` | `ret(0xbdc38) != 0x0` |
| `beq` at 0xa7cf8 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xa7d34 | `ret(0xbdf7c) > [sp-132] (unsigned, 32-bit)` | `ret(0xbdf7c) <= [sp-132] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8098 | `r0` | `?` |

## `FRUEventInit`

- `FRUEventInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1008 to 0xc1080.
  Established. The word through `r0 +0x0` is read (`ldr` at 0xc1020); its zero value takes
  `0xc1070`, while nonzero falls through (`beq` at 0xc1028). A later word assignment at an unknown
  effective target writes `0x0` (`str` at 0xc1078). Event initialization semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1018 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc101c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1020 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc102c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1030 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc1034 | store | 4 | `sp -0x18` | `[r0]` |
| `str` at 0xc103c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc1040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1054 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc1058 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc105c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1064 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc1068 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0xc1070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc1078 | store | 4 | `(unknown)` | `0x0` |
| `ldmdb` at 0xc107c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc107c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc107c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1060 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc1028 | `[r0] == 0x0` | `[r0] != 0x0` |
| `bne` at 0xc1048 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc107c | `r0` | `?` |

## `FRUEventAdd`

- `FRUEventAdd`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1080 to 0xc1174.
  Established. The unnamed `0x1b304` call at `0xc10c8` carries `r1`, `[r0]`, `0x8`, `r0` (`bl` at 0xc10c8); its positive result takes `0xc10f8`, while nonpositive falls through (`bgt` at 0xc10d4).
  The word assignment at `[sp-24] +0xa8` writes `[[sp-20]]` (`str` at 0xc10e4). Event ordering
  policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1090 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc1094 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc1098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc109c | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc10a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc10ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc10b0 | store | 4 | `r0 +0x0` | `r1` |
| `ldr` at 0xc10b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc10bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc10c0 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc10d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc10dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc10e0 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc10e4 | store | 4 | `[sp-24] +0xa8` | `[[sp-20]]` |
| `ldr` at 0xc10e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc10ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc10f0 | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xc10f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc10fc | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc1100 | store | 4 | `sp -0x1c` | `[[sp-20]]` |
| `ldr` at 0xc1104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1108 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xc110c | load | 4 | `[[sp-20]] +0xa8` | `-` |
| `str` at 0xc1110 | store | 4 | `sp -0x20` | `[[[sp-20]]+168]` |
| `ldr` at 0xc1114 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1124 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1140 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc1144 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xc1148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc114c | load | 4 | `[sp-32] +0xa8` | `-` |
| `str` at 0xc1150 | store | 4 | `sp -0x20` | `[[sp-32]+168]` |
| `ldr` at 0xc1158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc115c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc1160 | store | 4 | `[sp-24] +0xa8` | `?` |
| `ldr` at 0xc1164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1168 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc116c | store | 4 | `[sp-28] +0xa8` | `[sp-24]` |
| `ldmdb` at 0xc1170 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc1170 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc1170 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc10c8 | `(unknown callee) 0x1b304` | `r1` | `[r0]` | `0x8` | `r0` |
| `bl` at 0xc112c | `(unknown callee) 0x1b304` | `[sp-24]` | `?` | `0x8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc10a4 | `[r0] != 0x0` | `[r0] == 0x0` |
| `bgt` at 0xc10d4 | `ret(0x1b304) > 0x0 (signed, 32-bit)` | `ret(0x1b304) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xc111c | `Z == 1` | `not (Z == 1)` |
| `bge` at 0xc1138 | `ret(0x1b304) >= 0x0 (signed, 32-bit)` | `ret(0x1b304) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc1170 | `r0` | `?` |

## `FRUEventMerge`

- `FRUEventMerge`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1174 to 0xc11d0.
  Established. The word assignment at an unknown target writes `0x0` (`str` at 0xc11b8). The unnamed
  `0xc1080` call carries `[sp-20]`, `[sp-28]`, unknown, `0x0` (`bl` at 0xc11c4). Merge semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1184 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc1188 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc118c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc119c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc11a0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc11a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc11a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc11ac | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc11b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc11b8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc11bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc11c0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc11cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc11cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc11cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc11c4 | `(unknown callee) 0xc1080` | `[sp-20]` | `[sp-28]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc1194 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc11cc | `r0` | `?` |

## `FRUEventCreate`

- `FRUEventCreate`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc11d0 to 0xc1374.
  Established. The unnamed `0x1b1b4` call carries `0xac`, `r1`, `r2`, `r3` (`bl` at 0xc11f4). The
  halfword assignments at `[sp-36] +0x1c` and `[sp-36] +0x1e` write `0x0` (`strh` at 0xc1294, `strh` at 0xc12a0). The word at `[sp-36] +0xa8` also receives `0x0` (`str` at 0xc1360). Event record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc11e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc11e4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc11e8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc11ec | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc11fc | store | 4 | `sp -0x24` | `ret(0x1b1b4)` |
| `ldr` at 0xc1200 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc1210 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc1214 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1220 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc1224 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc1228 | store | 1 | `[sp-36] +0x8` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc122c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc1230 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc1234 | store | 1 | `[sp-36] +0x9` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc1238 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc123c | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xc1240 | store | 1 | `[sp-36] +0xa` | `([sp]:1 & 0xff)` |
| `ldr` at 0xc1244 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc1248 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xc124c | store | 1 | `[sp-36] +0xb` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xc1250 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc1254 | load | 1 | `sp +0x8` | `-` |
| `strb` at 0xc1258 | store | 1 | `[sp-36] +0xc` | `([sp+8]:1 & 0xff)` |
| `ldr` at 0xc125c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc1260 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc1264 | store | 4 | `[sp-36] +0x10` | `[sp+12]` |
| `ldr` at 0xc1268 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc126c | load | 1 | `sp +0x10` | `-` |
| `strb` at 0xc1270 | store | 1 | `[sp-36] +0xd` | `([sp+16]:1 & 0xff)` |
| `ldr` at 0xc1274 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc1278 | load | 2 | `sp +0x14` | `-` |
| `strh` at 0xc127c | store | 2 | `[sp-36] +0x18` | `([sp+20]:2 & 0xffff)` |
| `ldr` at 0xc1280 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc1284 | load | 2 | `sp +0x18` | `-` |
| `strh` at 0xc1288 | store | 2 | `[sp-36] +0x1a` | `([sp+24]:2 & 0xffff)` |
| `ldr` at 0xc128c | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xc1294 | store | 2 | `[sp-36] +0x1c` | `0x0` |
| `ldr` at 0xc1298 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xc12a0 | store | 2 | `[sp-36] +0x1e` | `0x0` |
| `ldr` at 0xc12a4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12a8 | load | 1 | `sp +0x1c` | `-` |
| `strb` at 0xc12ac | store | 1 | `[sp-36] +0x14` | `([sp+28]:1 & 0xff)` |
| `ldr` at 0xc12b0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12b4 | load | 1 | `sp +0x20` | `-` |
| `strb` at 0xc12b8 | store | 1 | `[sp-36] +0x20` | `([sp+32]:1 & 0xff)` |
| `ldr` at 0xc12bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12c0 | load | 1 | `sp +0x24` | `-` |
| `strb` at 0xc12c4 | store | 1 | `[sp-36] +0x15` | `([sp+36]:1 & 0xff)` |
| `ldr` at 0xc12c8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12cc | load | 1 | `sp +0x28` | `-` |
| `strb` at 0xc12d0 | store | 1 | `[sp-36] +0x21` | `([sp+40]:1 & 0xff)` |
| `ldr` at 0xc12d4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12d8 | load | 1 | `sp +0x2c` | `-` |
| `strb` at 0xc12dc | store | 1 | `[sp-36] +0x22` | `([sp+44]:1 & 0xff)` |
| `ldr` at 0xc12e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12e4 | load | 1 | `sp +0x30` | `-` |
| `strb` at 0xc12e8 | store | 1 | `[sp-36] +0x23` | `([sp+48]:1 & 0xff)` |
| `ldr` at 0xc12ec | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12f0 | load | 1 | `sp +0x34` | `-` |
| `strb` at 0xc12f4 | store | 1 | `[sp-36] +0x24` | `([sp+52]:1 & 0xff)` |
| `ldr` at 0xc12f8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc12fc | load | 1 | `sp +0x38` | `-` |
| `strb` at 0xc1300 | store | 1 | `[sp-36] +0x26` | `([sp+56]:1 & 0xff)` |
| `ldr` at 0xc1304 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc1308 | load | 1 | `sp +0x3c` | `-` |
| `strb` at 0xc130c | store | 1 | `[sp-36] +0x25` | `([sp+60]:1 & 0xff)` |
| `ldr` at 0xc1310 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc131c | load | 4 | `sp +0x40` | `-` |
| `ldr` at 0xc1328 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc1330 | store | 1 | `[sp-36] +0x67` | `0x0` |
| `ldr` at 0xc1334 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc1340 | load | 4 | `sp +0x44` | `-` |
| `ldr` at 0xc134c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc1354 | store | 1 | `[sp-36] +0xa7` | `0x0` |
| `ldr` at 0xc1358 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc1360 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xc1364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1368 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc1370 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc1370 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc1370 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc11f4 | `(unknown callee) 0x1b1b4` | `0xac` | `r1` | `r2` | `r3` |
| `bl` at 0xc121c | `(unknown callee) 0x1b394` | `ret(0x1b1b4)` | `[sp-24]` | `0x8` | `ret(0x1b1b4)` |
| `bl` at 0xc1324 | `(unknown callee) 0x1b4e4` | `[sp-36]+40` | `[sp+64]` | `0x40` | `[sp-36]+40` |
| `bl` at 0xc1348 | `(unknown callee) 0x1b4e4` | `[sp-36]+104` | `[sp+68]` | `0x40` | `[sp-36]+104` |
| `bl` at 0xc136c | `(unknown callee) 0xc1080` | `[sp-20]` | `[sp-36]` | `[sp-36]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc1208 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc1370 | `r0` | `?` |

## `FRUEventRank`

- `FRUEventRank`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1374 to 0xc166c.
  Established. The unnamed `0xbdc38` call at `0xc13e8` carries `[sp-20]`, `0x1f`, `0x0`,
  `ret(0xbdc38)` (`bl` at 0xc13e8). The halfword assignment at `[sp-20] +0x1c` writes `([sp-32]:2 & 0xffff)` (`strh` at 0xc14ac). The halfword at `[sp-20] +0x1e` receives `([sp-36]:2 & 0xffff)`
  (`strh` at 0xc158c). Ranking meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1384 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc138c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xc1394 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc139c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc13a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc13a4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc13a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc13ac | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xc13b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc13c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc13dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc13f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc13fc | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc1408 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc140c | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc1418 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc1420 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xc1424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1428 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xc142c | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xc1434 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc1438 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc143c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc144c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1484 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc1488 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc1494 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc1498 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc14a4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc14a8 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xc14ac | store | 2 | `[sp-20] +0x1c` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xc14b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc14b8 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xc14bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc14c0 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `str` at 0xc14c8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc14cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc14dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc14f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1514 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1518 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xc1524 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1528 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xc1534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1538 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc153c | load | 2 | `[sp-28] +0x18` | `-` |
| `ldrh` at 0xc1540 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xc154c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1550 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc1554 | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldrh` at 0xc1558 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xc1568 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc1570 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc1574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1578 | load | 4 | `[sp-28] +0xa8` | `-` |
| `str` at 0xc157c | store | 4 | `sp -0x1c` | `[[sp-28]+168]` |
| `ldr` at 0xc1584 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc1588 | load | 2 | `sp -0x24` | `-` |
| `strh` at 0xc158c | store | 2 | `[sp-20] +0x1e` | `([sp-36]:2 & 0xffff)` |
| `ldr` at 0xc1590 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1594 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xc1598 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xc15a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc15a4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc15a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc15bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc15c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc15c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc15c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc15d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc15dc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc15e0 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xc15e4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc15f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc15f4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc15f8 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xc15fc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc160c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc1610 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc161c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1620 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc1624 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc1630 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1634 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1638 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc163c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1640 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc1644 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc164c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc1650 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xc1654 | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xc1658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc165c | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xc1660 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldmdb` at 0xc1668 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc1668 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc1668 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc13cc | `(unknown callee) 0xbdc38` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xc13e8 | `(unknown callee) 0xbdc38` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xbdc38)` |
| `bl` at 0xc1458 | `(unknown callee) 0xbdc38` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xc1474 | `(unknown callee) 0xbdc38` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xbdc38)` |
| `bl` at 0xc14e8 | `(unknown callee) 0xbdc38` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xc1504 | `(unknown callee) 0xbdc38` | `[sp-28]` | `0x1f` | `0x0` | `ret(0xbdc38)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc13b8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc13d8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc13f4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1404 | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xc1414 | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `bne` at 0xc1444 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc1464 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1480 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1490 | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xc14a0 | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `bne` at 0xc14d4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc14f4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1510 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1520 | `[[sp-28]+38]:1 == 0x4` | `[[sp-28]+38]:1 != 0x4` |
| `beq` at 0xc1530 | `[[sp-28]+38]:1 == 0x8` | `[[sp-28]+38]:1 != 0x8` |
| `beq` at 0xc1548 | `[[sp-28]+24]:2 == [[sp-20]+24]:2` | `[[sp-28]+24]:2 != [[sp-20]+24]:2` |
| `beq` at 0xc1560 | `[[sp-28]+26]:2 == [[sp-20]+24]:2` | `[[sp-28]+26]:2 != [[sp-20]+24]:2` |
| `bne` at 0xc15b0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc15d0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc15ec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1604 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc1618 | `Z == 1` | `not (Z == 1)` |
| `bls` at 0xc162c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc1668 | `r0` | `?` |

## `FRUEventPrint`

- `FRUEventPrint`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc166c to 0xc1a30.
  Established. The indirect call at `0xc16ec` has an unknown target and candidates `[sp-28]`,
  `sp-164`, `ret(0x1b2bc)`, `sp-164` (`mov` at 0xc16ec). The unnamed `0x1b52c` call at `0xc1724`
  carries `sp-164`, `0x13320c`, `0x133230`, `0x133234` (`bl` at 0xc1724). Event output meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc167c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc1680 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc1684 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc1688 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc1690 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0xc1694 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc1698 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xc169c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc16a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc16bc | load | 4 | `abs 0xc19f8` | `-` |
| `ldr` at 0xc16dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc16e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc16f4 | load | 4 | `abs 0xc19fc` | `-` |
| `str` at 0xc16f8 | store | 4 | `sp -0xb8` | `0x1331f0` |
| `ldr` at 0xc16fc | load | 4 | `abs 0xc1a00` | `-` |
| `str` at 0xc1700 | store | 4 | `sp -0xb4` | `0x1331f8` |
| `ldr` at 0xc1704 | load | 4 | `abs 0xc1a04` | `-` |
| `str` at 0xc1708 | store | 4 | `sp -0xb0` | `0x133200` |
| `ldr` at 0xc170c | load | 4 | `abs 0xc1a08` | `-` |
| `str` at 0xc1710 | store | 4 | `sp -0xac` | `0x133208` |
| `ldr` at 0xc1718 | load | 4 | `abs 0xc1a0c` | `-` |
| `ldr` at 0xc171c | load | 4 | `abs 0xc1a10` | `-` |
| `ldr` at 0xc1720 | load | 4 | `abs 0xc1a14` | `-` |
| `ldr` at 0xc1740 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1744 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1778 | load | 4 | `abs 0xc1a18` | `-` |
| `ldr` at 0xc1798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc179c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc17b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc17c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc17cc | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc17d0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc17dc | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc17e0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc17e4 | load | 4 | `abs 0xc1a1c` | `-` |
| `ldr` at 0xc17f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc17fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1804 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1808 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc180c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xc1810 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1814 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1818 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xc181c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1820 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1824 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xc1828 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc182c | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1830 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xc1838 | load | 4 | `abs 0xc1a20` | `-` |
| `ldr` at 0xc1850 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1858 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1860 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1864 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1868 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xc186c | load | 4 | `abs 0xc1a24` | `-` |
| `str` at 0xc1870 | store | 4 | `sp -0xb4` | `0x1332a4` |
| `ldr` at 0xc1874 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1878 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc187c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xc1880 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1884 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1888 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xc1890 | load | 4 | `abs 0xc1a28` | `-` |
| `ldr` at 0xc18b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc18bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc18d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc18dc | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc18e0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc18e4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc18e8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc18f0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc18fc | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1900 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc1904 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1908 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1910 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xc1914 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1918 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc191c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc1924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1930 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc1944 | load | 4 | `abs 0xc1a2c` | `-` |
| `ldr` at 0xc1964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1968 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc197c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc198c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc1990 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc1994 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc19a4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc19a8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc19ac | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc19b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc19b8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc19c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc19c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc19cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc19d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc19d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc19dc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc19e4 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xc19ec | store | 4 | `sp -0xa8` | `?` |
| `ldmdb` at 0xc19f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc19f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc19f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc16c0 | `(unknown callee) 0x1b52c` | `sp-164` | `0x1331c4` | `r2` | `sp-164` |
| `bl` at 0xc16cc | `(unknown callee) 0x1b2bc` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc16ec | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b2bc)` | `sp-164` |
| `bl` at 0xc1724 | `(unknown callee) 0x1b52c` | `sp-164` | `0x13320c` | `0x133230` | `0x133234` |
| `bl` at 0xc1730 | `(unknown callee) 0x1b2bc` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc1750 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b2bc)` | `sp-164` |
| `bl` at 0xc177c | `(unknown callee) 0x1b52c` | `sp-164` | `0x13323c` | `r2` | `sp-164` |
| `bl` at 0xc1788 | `(unknown callee) 0x1b2bc` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc17a8 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b2bc)` | `sp-164` |
| `bl` at 0xc1844 | `(unknown callee) 0x1b52c` | `sp-164` | `0x133280` | `?` | `?` |
| `bl` at 0xc189c | `(unknown callee) 0x1b52c` | `sp-164` | `0x1332ac` | `?` | `?` |
| `bl` at 0xc18a8 | `(unknown callee) 0x1b2bc` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc18c8 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b2bc)` | `sp-164` |
| `bl` at 0xc1948 | `(unknown callee) 0x1b52c` | `sp-164` | `0x1332d0` | `?` | `sp-164` |
| `bl` at 0xc1954 | `(unknown callee) 0x1b2bc` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc1974 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b2bc)` | `sp-164` |
| `bl` at 0xc19d0 | `(unknown callee) 0xc1a30` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc16a4 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xc16b0 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xc1760 | `r0 != 0x2` | `r0 == 0x2` |
| `beq` at 0xc176c | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xc17b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc17c8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc17d8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc17ec | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc18d8 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xc18f8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc192c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc1938 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc1984 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc199c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc19c0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc19f4 | `r0` | `?` |

## `FRUEventGuess`

- `FRUEventGuess`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1a30 to 0xc25b4.
  Established. The byte read uses `[sp-28] +0xd` (`ldrb` at 0xc1a60). Nonzero takes `0xc1a7c`, while
  zero falls through (`bne` at 0xc1a68). The indirect call at `0xc2278` has an unknown target and
  candidates `[sp-24]`, `sp-924`, `ret(0x1b2bc)`, `sp-924` (`mov` at 0xc2278). Event diagnosis
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1a40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc1a44 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc1a48 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc1a54 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1a5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1a60 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1a74 | load | 4 | `abs 0xc255c` | `-` |
| `ldr` at 0xc1a7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1a80 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1a98 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1ab4 | load | 4 | `abs 0xc2560` | `-` |
| `ldr` at 0xc1ac4 | load | 4 | `abs 0xc2564` | `-` |
| `ldr` at 0xc1acc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ad0 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1ae8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1b04 | load | 4 | `abs 0xc2560` | `-` |
| `ldr` at 0xc1b14 | load | 4 | `abs 0xc2568` | `-` |
| `ldr` at 0xc1b1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1b20 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1b38 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1b54 | load | 4 | `abs 0xc2560` | `-` |
| `ldr` at 0xc1b64 | load | 4 | `abs 0xc256c` | `-` |
| `ldr` at 0xc1b6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1b70 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1b88 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1ba4 | load | 4 | `abs 0xc2560` | `-` |
| `ldr` at 0xc1bb4 | load | 4 | `abs 0xc2570` | `-` |
| `ldr` at 0xc1bbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1bc0 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1bd8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1bf4 | load | 4 | `abs 0xc2560` | `-` |
| `ldr` at 0xc1c04 | load | 4 | `abs 0xc2574` | `-` |
| `ldr` at 0xc1c0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1c10 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1c28 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1c44 | load | 4 | `abs 0xc2560` | `-` |
| `ldr` at 0xc1c54 | load | 4 | `abs 0xc2578` | `-` |
| `ldr` at 0xc1c64 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1c6c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1c70 | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldr` at 0xc1c74 | load | 4 | `abs 0xc257c` | `-` |
| `ldr` at 0xc1c80 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1c84 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1c94 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc1c98 | load | 2 | `[sp-28] +0x1a` | `-` |
| `str` at 0xc1ca4 | store | 4 | `sp -0x3a4` | `?` |
| `ldr` at 0xc1ca8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1cac | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc1cb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1cbc | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc1cc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ccc | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc1cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1cdc | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc1cf0 | load | 4 | `abs 0xc2580` | `-` |
| `ldr` at 0xc1d00 | load | 4 | `abs 0xc2584` | `-` |
| `ldr` at 0xc1d04 | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xc1d10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d14 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc1d20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d24 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc1d30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d34 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc1d40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d44 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc1d58 | load | 4 | `abs 0xc2588` | `-` |
| `ldr` at 0xc1d68 | load | 4 | `abs 0xc2584` | `-` |
| `ldr` at 0xc1d6c | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xc1d80 | load | 4 | `abs 0xc2584` | `-` |
| `ldr` at 0xc1d84 | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xc1d8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1d90 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1da8 | load | 4 | `abs 0xc258c` | `-` |
| `ldr` at 0xc1db8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1dc8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1dd8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1de8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc1df0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1df4 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xc1e0c | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc1e1c | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc1e2c | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc1e3c | load | 4 | `abs 0xc2590` | `-` |
| `str` at 0xc1e48 | store | 4 | `sp -0x3a8` | `0x8` |
| `ldr` at 0xc1e4c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1e50 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1e5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1e60 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xc1e6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1e70 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xc1e78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1e7c | load | 1 | `[sp-28] +0x14` | `-` |
| `str` at 0xc1e84 | store | 4 | `sp -0x3a8` | `?` |
| `str` at 0xc1e8c | store | 4 | `sp -0x3ac` | `0x3` |
| `str` at 0xc1e94 | store | 4 | `sp -0x3b0` | `0x3` |
| `str` at 0xc1e9c | store | 4 | `sp -0x3b4` | `0x3` |
| `str` at 0xc1ea4 | store | 4 | `sp -0x3b8` | `0x3` |
| `ldr` at 0xc1ea8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1eac | load | 1 | `[sp-28] +0x15` | `-` |
| `ldr` at 0xc1eb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ebc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ecc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1edc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ee0 | load | 1 | `[sp-28] +0x15` | `-` |
| `str` at 0xc1ee4 | store | 4 | `sp -0x3ac` | `[[sp-28]+21]:1` |
| `ldr` at 0xc1ee8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1eec | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1efc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1f0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f10 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc1f1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f20 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xc1f30 | store | 4 | `sp -0x3ac` | `ret(0xc0e74)` |
| `ldr` at 0xc1f34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f38 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1f44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f48 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc1f54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f58 | load | 1 | `[sp-28] +0x22` | `-` |
| `str` at 0xc1f68 | store | 4 | `sp -0x3b0` | `ret(0xc0e74)` |
| `ldr` at 0xc1f6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f70 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc1f7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f80 | load | 1 | `[sp-28] +0x23` | `-` |
| `str` at 0xc1f90 | store | 4 | `sp -0x3b4` | `ret(0xc0e74)` |
| `ldr` at 0xc1f94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1f98 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc1fa4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1fa8 | load | 1 | `[sp-28] +0x24` | `-` |
| `str` at 0xc1fb8 | store | 4 | `sp -0x3b8` | `ret(0xc0e74)` |
| `ldr` at 0xc1fbc | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc1fc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1fcc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc1fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1fdc | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xc1fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1fec | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xc1ff8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1ffc | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xc2008 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc200c | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc201c | load | 4 | `abs 0xc2594` | `-` |
| `ldr` at 0xc2020 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xc2028 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc202c | load | 4 | `abs 0xc2598` | `-` |
| `ldr` at 0xc2030 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2034 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc2038 | load | 1 | `[[sp-28]+20]:1 +0x12984c` | `-` |
| `str` at 0xc203c | store | 4 | `sp -0x3c0` | `[[[sp-28]+20]:1+1218636]:1` |
| `ldr` at 0xc2044 | load | 4 | `abs 0xc259c` | `-` |
| `ldr` at 0xc2048 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc205c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2060 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc2064 | load | 4 | `abs 0xc2594` | `-` |
| `ldr` at 0xc2068 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xc2070 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc2074 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc2078 | load | 4 | `abs 0xc2598` | `-` |
| `ldr` at 0xc207c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2080 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc2084 | load | 1 | `[[sp-28]+20]:1 +0x12984c` | `-` |
| `str` at 0xc2088 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1218636]:1` |
| `ldr` at 0xc2090 | load | 4 | `abs 0xc25a0` | `-` |
| `ldr` at 0xc2094 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc20a4 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc20b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc20b4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xc20c0 | load | 4 | `sp -0x3ac` | `-` |
| `ldr` at 0xc20d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc20d4 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc20d8 | load | 4 | `abs 0xc2594` | `-` |
| `ldr` at 0xc20dc | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xc20e4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc20e8 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc20ec | load | 4 | `abs 0xc2598` | `-` |
| `ldr` at 0xc20f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc20f4 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc20f8 | load | 1 | `[[sp-28]+20]:1 +0x12984c` | `-` |
| `str` at 0xc20fc | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1218636]:1` |
| `ldr` at 0xc2104 | load | 4 | `abs 0xc25a0` | `-` |
| `ldr` at 0xc2108 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc2114 | load | 4 | `sp -0x3b0` | `-` |
| `ldr` at 0xc2124 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2128 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc212c | load | 4 | `abs 0xc2594` | `-` |
| `ldr` at 0xc2130 | load | 4 | `sp -0x3b0` | `-` |
| `ldrb` at 0xc2138 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc213c | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc2140 | load | 4 | `abs 0xc2598` | `-` |
| `ldr` at 0xc2144 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2148 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc214c | load | 1 | `[[sp-28]+20]:1 +0x12984c` | `-` |
| `str` at 0xc2150 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1218636]:1` |
| `ldr` at 0xc2158 | load | 4 | `abs 0xc25a0` | `-` |
| `ldr` at 0xc215c | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc2168 | load | 4 | `sp -0x3b4` | `-` |
| `ldr` at 0xc2178 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc217c | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc2180 | load | 4 | `abs 0xc2594` | `-` |
| `ldr` at 0xc2184 | load | 4 | `sp -0x3b4` | `-` |
| `ldrb` at 0xc218c | load | 1 | `(unknown)` | `-` |
| `str` at 0xc2190 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc2194 | load | 4 | `abs 0xc2598` | `-` |
| `ldr` at 0xc2198 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc219c | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc21a0 | load | 1 | `[[sp-28]+20]:1 +0x12984c` | `-` |
| `str` at 0xc21a4 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1218636]:1` |
| `ldr` at 0xc21ac | load | 4 | `abs 0xc25a0` | `-` |
| `ldr` at 0xc21b0 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc21bc | load | 4 | `sp -0x3b8` | `-` |
| `ldr` at 0xc21cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc21d0 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xc21d4 | load | 4 | `abs 0xc2594` | `-` |
| `ldr` at 0xc21d8 | load | 4 | `sp -0x3b8` | `-` |
| `ldrb` at 0xc21e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc21e4 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xc21e8 | load | 4 | `abs 0xc2598` | `-` |
| `ldr` at 0xc21ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc21f0 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xc21f4 | load | 1 | `[[sp-28]+20]:1 +0x12984c` | `-` |
| `str` at 0xc21f8 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1218636]:1` |
| `ldr` at 0xc2200 | load | 4 | `abs 0xc25a0` | `-` |
| `ldr` at 0xc2204 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xc2210 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc2214 | load | 2 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0xc2218 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc221c | load | 2 | `[sp-28] +0x1e` | `-` |
| `str` at 0xc2224 | store | 4 | `sp -0x3a0` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` |
| `ldr` at 0xc222c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc2230 | load | 2 | `[sp-28] +0x18` | `-` |
| `str` at 0xc2238 | store | 4 | `sp -0x3c0` | `sp-284` |
| `ldr` at 0xc2240 | load | 4 | `abs 0xc25a4` | `-` |
| `ldr` at 0xc2244 | load | 4 | `sp -0x3a0` | `-` |
| `ldr` at 0xc2268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc226c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2280 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc2284 | load | 2 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xc2288 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc228c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xc2290 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc2294 | load | 1 | `[sp-28] +0x9` | `-` |
| `str` at 0xc2298 | store | 4 | `sp -0x3c0` | `[[sp-28]+9]:1` |
| `ldr` at 0xc22a0 | load | 4 | `abs 0xc25a8` | `-` |
| `ldr` at 0xc22c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc22cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc22e4 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc2304 | load | 4 | `abs 0xc25ac` | `-` |
| `ldr` at 0xc2324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2340 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc235c | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc237c | load | 4 | `abs 0xc25ac` | `-` |
| `ldr` at 0xc239c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc23a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc23b8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc23d4 | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc23f4 | load | 4 | `abs 0xc25ac` | `-` |
| `ldr` at 0xc2414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2418 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2430 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc244c | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc246c | load | 4 | `abs 0xc25ac` | `-` |
| `ldr` at 0xc248c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc24a8 | load | 4 | `abs 0xc2558` | `-` |
| `ldr` at 0xc24c4 | load | 4 | `abs 0xc2590` | `-` |
| `ldr` at 0xc24e4 | load | 4 | `abs 0xc25ac` | `-` |
| `ldr` at 0xc2504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2520 | load | 4 | `abs 0xc25b0` | `-` |
| `ldr` at 0xc2540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc2544 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc2554 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc2554 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc2554 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1a58 | `(unknown callee) 0x1b598` | `sp-284` | `0x1332f8` | `r2` | `sp-284` |
| `bl` at 0xc1a78 | `(unknown callee) 0x1b364` | `sp-284` | `0x1332fc` | `?` | `sp-284` |
| `bl` at 0xc1a9c | `(unknown callee) 0x1b088` | `sp-284` | `0x1332f8` | `?` | `sp-284` |
| `bl` at 0xc1ab8 | `(unknown callee) 0x1b364` | `sp-284` | `0x133304` | `?` | `sp-284` |
| `bl` at 0xc1ac8 | `(unknown callee) 0x1b364` | `sp-284` | `0x133308` | `?` | `sp-284` |
| `bl` at 0xc1aec | `(unknown callee) 0x1b088` | `sp-284` | `0x1332f8` | `?` | `sp-284` |
| `bl` at 0xc1b08 | `(unknown callee) 0x1b364` | `sp-284` | `0x133304` | `?` | `sp-284` |
| `bl` at 0xc1b18 | `(unknown callee) 0x1b364` | `sp-284` | `0x13331c` | `?` | `sp-284` |
| `bl` at 0xc1b3c | `(unknown callee) 0x1b088` | `sp-284` | `0x1332f8` | `?` | `sp-284` |
| `bl` at 0xc1b58 | `(unknown callee) 0x1b364` | `sp-284` | `0x133304` | `?` | `sp-284` |
| `bl` at 0xc1b68 | `(unknown callee) 0x1b364` | `sp-284` | `0x133328` | `?` | `sp-284` |
| `bl` at 0xc1b8c | `(unknown callee) 0x1b088` | `sp-284` | `0x1332f8` | `?` | `sp-284` |
| `bl` at 0xc1ba8 | `(unknown callee) 0x1b364` | `sp-284` | `0x133304` | `?` | `sp-284` |
| `bl` at 0xc1bb8 | `(unknown callee) 0x1b364` | `sp-284` | `0x13333c` | `?` | `sp-284` |
| `bl` at 0xc1bdc | `(unknown callee) 0x1b088` | `sp-284` | `0x1332f8` | `?` | `sp-284` |
| `bl` at 0xc1bf8 | `(unknown callee) 0x1b364` | `sp-284` | `0x133304` | `?` | `sp-284` |
| `bl` at 0xc1c08 | `(unknown callee) 0x1b364` | `sp-284` | `0x133344` | `?` | `sp-284` |
| `bl` at 0xc1c2c | `(unknown callee) 0x1b088` | `sp-284` | `0x1332f8` | `?` | `sp-284` |
| `bl` at 0xc1c48 | `(unknown callee) 0x1b364` | `sp-284` | `0x133304` | `?` | `sp-284` |
| `bl` at 0xc1c58 | `(unknown callee) 0x1b364` | `sp-284` | `0x133368` | `?` | `sp-284` |
| `bl` at 0xc1c68 | `(unknown callee) 0x1b598` | `sp-156` | `0x1332f8` | `?` | `sp-156` |
| `bl` at 0xc1cf4 | `(unknown callee) 0x1b52c` | `sp-284` | `0x133374` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xc1d08 | `(unknown callee) 0x1b52c` | `sp-156` | `0x1333a4` | `[sp-932]` | `sp-156` |
| `bl` at 0xc1d5c | `(unknown callee) 0x1b52c` | `sp-284` | `0x1333d4` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xc1d70 | `(unknown callee) 0x1b52c` | `sp-156` | `0x1333a4` | `[sp-932]` | `sp-156` |
| `bl` at 0xc1d88 | `(unknown callee) 0x1b52c` | `sp-156` | `0x1333a4` | `[sp-932]` | `sp-156` |
| `bl` at 0xc1dac | `(unknown callee) 0x1b52c` | `sp-284` | `0x1333fc` | `?` | `sp-284` |
| `bl` at 0xc1dbc | `(unknown callee) 0x1b598` | `sp-412` | `0x1332f8` | `?` | `sp-412` |
| `bl` at 0xc1dcc | `(unknown callee) 0x1b598` | `sp-540` | `0x1332f8` | `?` | `sp-540` |
| `bl` at 0xc1ddc | `(unknown callee) 0x1b598` | `sp-668` | `0x1332f8` | `?` | `sp-668` |
| `bl` at 0xc1dec | `(unknown callee) 0x1b598` | `sp-796` | `0x1332f8` | `?` | `sp-796` |
| `bl` at 0xc1e10 | `(unknown callee) 0x1b364` | `sp-412` | `0x133434` | `?` | `sp-412` |
| `bl` at 0xc1e20 | `(unknown callee) 0x1b364` | `sp-540` | `0x133434` | `?` | `sp-540` |
| `bl` at 0xc1e30 | `(unknown callee) 0x1b364` | `sp-668` | `0x133434` | `?` | `sp-668` |
| `bl` at 0xc1e40 | `(unknown callee) 0x1b364` | `sp-796` | `0x133434` | `?` | `sp-796` |
| `bl` at 0xc1f28 | `(unknown callee) 0xc0e74` | `[[sp-28]+33]:1` | `?` | `?` | `[[sp-28]+33]:1` |
| `bl` at 0xc1f60 | `(unknown callee) 0xc0e74` | `[[sp-28]+34]:1` | `?` | `?` | `[[sp-28]+34]:1` |
| `bl` at 0xc1f88 | `(unknown callee) 0xc0e74` | `[[sp-28]+35]:1` | `?` | `?` | `[[sp-28]+35]:1` |
| `bl` at 0xc1fb0 | `(unknown callee) 0xc0e74` | `[[sp-28]+36]:1` | `?` | `?` | `[[sp-28]+36]:1` |
| `bl` at 0xc2050 | `(unknown callee) 0x1b52c` | `sp-412` | `0x133438` | `?` | `?` |
| `bl` at 0xc209c | `(unknown callee) 0x1b52c` | `sp-412` | `0x133468` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc2110 | `(unknown callee) 0x1b52c` | `sp-412` | `0x133468` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc2164 | `(unknown callee) 0x1b52c` | `sp-540` | `0x133468` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc21b8 | `(unknown callee) 0x1b52c` | `sp-668` | `0x133468` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc220c | `(unknown callee) 0x1b52c` | `sp-796` | `0x133468` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xc224c | `(unknown callee) 0x1b52c` | `sp-924` | `0x133498` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` | `[[sp-28]+24]:2` |
| `bl` at 0xc2258 | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc2278 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc22ac | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334b0` | `[[sp-28]+24]:2` | `[[sp-28]+8]:1` |
| `bl` at 0xc22b8 | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc22d8 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc22e8 | `(unknown callee) 0x1b088` | `sp-156` | `0x1332f8` | `?` | `sp-156` |
| `bl` at 0xc2308 | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334e0` | `sp-156` | `sp-924` |
| `bl` at 0xc2314 | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc2334 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc2344 | `(unknown callee) 0x1b088` | `sp-412` | `0x1332f8` | `?` | `sp-412` |
| `bl` at 0xc2360 | `(unknown callee) 0x1b088` | `sp-412` | `0x133434` | `?` | `sp-412` |
| `bl` at 0xc2380 | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334e0` | `sp-412` | `sp-924` |
| `bl` at 0xc238c | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc23ac | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc23bc | `(unknown callee) 0x1b088` | `sp-540` | `0x1332f8` | `?` | `sp-540` |
| `bl` at 0xc23d8 | `(unknown callee) 0x1b088` | `sp-540` | `0x133434` | `?` | `sp-540` |
| `bl` at 0xc23f8 | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334e0` | `sp-540` | `sp-924` |
| `bl` at 0xc2404 | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc2424 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc2434 | `(unknown callee) 0x1b088` | `sp-668` | `0x1332f8` | `?` | `sp-668` |
| `bl` at 0xc2450 | `(unknown callee) 0x1b088` | `sp-668` | `0x133434` | `?` | `sp-668` |
| `bl` at 0xc2470 | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334e0` | `sp-668` | `sp-924` |
| `bl` at 0xc247c | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc249c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc24ac | `(unknown callee) 0x1b088` | `sp-796` | `0x1332f8` | `?` | `sp-796` |
| `bl` at 0xc24c8 | `(unknown callee) 0x1b088` | `sp-796` | `0x133434` | `?` | `sp-796` |
| `bl` at 0xc24e8 | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334e0` | `sp-796` | `sp-924` |
| `bl` at 0xc24f4 | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc2514 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |
| `bl` at 0xc2524 | `(unknown callee) 0x1b52c` | `sp-924` | `0x1334e8` | `?` | `sp-924` |
| `bl` at 0xc2530 | `(unknown callee) 0x1b2bc` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xc2550 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x1b2bc)` | `sp-924` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc1a68 | `[[sp-28]+13]:1 != 0x0` | `[[sp-28]+13]:1 == 0x0` |
| `beq` at 0xc1a8c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1aa8 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc1adc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1af8 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc1b2c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1b48 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc1b7c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1b98 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc1bcc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1be8 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc1c1c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1c38 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc1c7c | `[[sp-28]+26]:2 == 0xffff` | `[[sp-28]+26]:2 != 0xffff` |
| `beq` at 0xc1c90 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc1cb4 | `[[sp-28]+33]:1 != 0x77` | `[[sp-28]+33]:1 == 0x77` |
| `bne` at 0xc1cc4 | `[[sp-28]+34]:1 != 0x77` | `[[sp-28]+34]:1 == 0x77` |
| `bne` at 0xc1cd4 | `[[sp-28]+35]:1 != 0x77` | `[[sp-28]+35]:1 == 0x77` |
| `bne` at 0xc1ce4 | `[[sp-28]+36]:1 != 0x77` | `[[sp-28]+36]:1 == 0x77` |
| `bne` at 0xc1d1c | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xc1d2c | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xc1d3c | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xc1d4c | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `beq` at 0xc1d9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1e00 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xc1e58 | `[[sp-28]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bhi` at 0xc1e68 | `[[sp-28]+20]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+20]:1 <= 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xc1eb4 | `[[sp-28]+21]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+21]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xc1ec4 | `[[sp-28]+37]:1 == 0x6` | `[[sp-28]+37]:1 != 0x6` |
| `beq` at 0xc1ed4 | `[[sp-28]+37]:1 == 0x8` | `[[sp-28]+37]:1 != 0x8` |
| `beq` at 0xc1ef4 | `[[sp-28]+37]:1 == 0xa` | `[[sp-28]+37]:1 != 0xa` |
| `beq` at 0xc1f04 | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bhi` at 0xc1f18 | `[[sp-28]+33]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+33]:1 <= 0x7e (unsigned, 32-bit)` |
| `bne` at 0xc1f40 | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xc1f50 | `[[sp-28]+34]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+34]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xc1f78 | `[[sp-28]+35]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+35]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xc1fa0 | `[[sp-28]+36]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+36]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xc1fc4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xc1fd4 | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bne` at 0xc1fe4 | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xc1ff4 | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xc2004 | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xc2014 | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `bhi` at 0xc20ac | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xc20bc | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xc20c8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xc211c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xc2170 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xc21c4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xc22f4 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc2350 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc236c | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc23c8 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc23e4 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc2440 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc245c | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc24b8 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xc24d4 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc2554 | `r0` | `ret(unknown)` |

## `FRUInit`

- `FRUInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbdc28 to 0xbdc38.
  Established. The return reports entry `r0` (`ldmdb` at 0xbdc34). That instruction reads words at
  `sp -0x10`, `sp -0xc` and `sp -0x8` (`ldmdb` at 0xbdc34). FRU initialization effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0xbdc34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbdc34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbdc34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbdc34 | `r0` | `r0` |

## `FRUStatus`

- `FRUStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4804 to 0xc48e0.
  Established. The selector takes unsigned `r0 <= 0x8` to an unresolved destination, while larger
  values fall through (`ldrls` at 0xc4820). The known word assignment at `sp -0x18` writes
  `0x1335b4` (`str` at 0xc48b0). Status label meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4814 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc4818 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc4820 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc48ac | load | 4 | `abs 0xc48dc` | `-` |
| `str` at 0xc48b0 | store | 4 | `sp -0x18` | `0x1335b4` |
| `ldr` at 0xc48b4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc48b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc48b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc48b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc4820 | `r0 <= 0x8 (unsigned, 32-bit)` | `r0 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc48b8 | `r0` | `0x1335b4` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc4820).

## `FRUDumpBuff`

- `FRUDumpBuff`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc500c to 0xc5058.
  Established. The unnamed `0xc4df0` call carries `0xc5d90`, `ret(0xc5cd4)`, `[sp-20]`,
  `ret(0xc5cd4)` (`bl` at 0xc5038). The unnamed `0xc5d24` call carries `[sp-24]`, `0x0`, unknown,
  unknown (`bl` at 0xc5044). Dump-buffer contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc501c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5028 | store | 4 | `sp -0x18` | `ret(0xc5cd4)` |
| `ldr` at 0xc502c | load | 4 | `abs 0xc5054` | `-` |
| `ldr` at 0xc5030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc5034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc503c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc5050 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5050 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5020 | `(unknown callee) 0xc5cd4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc5038 | `(unknown callee) 0xc4df0` | `0xc5d90` | `ret(0xc5cd4)` | `[sp-20]` | `ret(0xc5cd4)` |
| `bl` at 0xc5044 | `(unknown callee) 0xc5d24` | `[sp-24]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5050 | `r0` | `ret(0xc5d24)` |

## `_FRUDump`

- `_FRUDump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc48e0 to 0xc4df0.
  Established. The unnamed `0xc4804` call carries `[sp+12]`, `r1`, `r2`, `r3` (`bl` at 0xc4904).
  Entry-stack word `[sp+12] == 0x0` takes `0xc4968`, while nonzero falls through (`beq` at 0xc4960).
  The unnamed `0xc47d4` call at `0xc4d5c` carries `[sp-20]`, `[sp-24]`, `[sp+24]`, `[sp+24]` (`bl` at 0xc4d5c). Decoded FRU field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc48f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc48f4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc48f8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc48fc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc4900 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc4914 | load | 4 | `abs 0xc4dcc` | `-` |
| `ldr` at 0xc4920 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4924 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4938 | load | 4 | `abs 0xc4dd0` | `-` |
| `ldr` at 0xc493c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc494c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4958 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc4980 | load | 4 | `abs 0xc4dd4` | `-` |
| `ldr` at 0xc498c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4990 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc49a0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc49a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc49a8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc49b8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc49c8 | store | 4 | `sp -0x9c` | `ret(0xc5428)` |
| `ldr` at 0xc49d0 | load | 4 | `abs 0xc4dd8` | `-` |
| `ldr` at 0xc49d4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xc49d8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc49e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc49e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc49f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4a08 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4a10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc4a14 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xc4a18 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc4a24 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc4a30 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc4a40 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc4a54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4a68 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4a74 | load | 4 | `abs 0xc4ddc` | `-` |
| `ldr` at 0xc4a78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4a88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4a9c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4aa8 | load | 4 | `abs 0xc4de0` | `-` |
| `ldr` at 0xc4aac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4abc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4ad0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4adc | load | 4 | `abs 0xc4de4` | `-` |
| `ldr` at 0xc4ae0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4aec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4af0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc4b00 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc4b04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4b18 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4b20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4b24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4b38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4b4c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4b54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4b5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4b64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4b78 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4b80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4b88 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4b90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4b94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4ba8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4bb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4bb8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4bc0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4bc4 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xc4bc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4bdc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4be4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4bec | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4bf4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4bf8 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xc4bfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4c10 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4c18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4c20 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4c28 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4c2c | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xc4c30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4c44 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4c4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4c54 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4c5c | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4c60 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xc4c64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4c78 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4c80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4c88 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4c90 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4c94 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xc4c98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4cac | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4cb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4cbc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4cc4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4cc8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xc4ccc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4ce0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4ce8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4cf0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4cf8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4cfc | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xc4d00 | load | 4 | `abs 0xc4de8` | `-` |
| `ldr` at 0xc4d04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4d14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4d24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4d2c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xc4d34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc4d3c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc4d44 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc4d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4d58 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc4d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4d68 | load | 4 | `abs 0xc4dec` | `-` |
| `ldr` at 0xc4d70 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc4d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4d84 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc4d8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4d94 | load | 4 | `abs 0xc4dec` | `-` |
| `ldr` at 0xc4d9c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc4da8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4dac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4db0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc4db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4dbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4dc0 | load | 4 | `abs 0xc4dec` | `-` |
| `ldmdb` at 0xc4dc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4dc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4dc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4904 | `(unknown callee) 0xc4804` | `[sp+12]` | `r1` | `r2` | `r3` |
| `bl` at 0xc4918 | `(unknown callee) 0x1b52c` | `sp-124` | `0x1335c8` | `ret(0xc4804)` | `sp-124` |
| `bl` at 0xc492c | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4940 | `(unknown callee) 0x1b52c` | `sp-124` | `0x1335d8` | `[sp-28]` | `sp-124` |
| `bl` at 0xc4954 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4970 | `(unknown callee) 0x1b3c4` | `sp-32` | `?` | `?` | `sp-32` |
| `bl` at 0xc4984 | `(unknown callee) 0x1b52c` | `sp-124` | `0x1335ec` | `ret(0x1b3c4)` | `sp-124` |
| `bl` at 0xc4998 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc49bc | `(unknown callee) 0xc5428` | `[sp]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xc49dc | `(unknown callee) 0x1b52c` | `sp-124` | `0x1335fc` | `[sp+4]` | `[sp+8]` |
| `bl` at 0xc49f0 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4a7c | `(unknown callee) 0x1b52c` | `sp-124` | `0x13360c` | `?` | `?` |
| `bl` at 0xc4ab0 | `(unknown callee) 0x1b52c` | `sp-124` | `0x133618` | `?` | `?` |
| `bl` at 0xc4ae4 | `(unknown callee) 0x1b52c` | `sp-124` | `0x133624` | `?` | `?` |
| `bl` at 0xc4af8 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4d0c | `(unknown callee) 0x1b52c` | `sp-124` | `0x13362c` | `?` | `?` |
| `bl` at 0xc4d20 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4d5c | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `[sp+24]` | `[sp+24]` |
| `bl` at 0xc4d6c | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `0x1334e8` | `?` |
| `bl` at 0xc4d88 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `[sp+28]` | `[sp+28]` |
| `bl` at 0xc4d98 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `0x1334e8` | `?` |
| `bl` at 0xc4db4 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `[sp+32]` | `[sp+32]` |
| `bl` at 0xc4dc4 | `(unknown callee) 0xc47d4` | `[sp-20]` | `[sp-24]` | `0x1334e8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc4960 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `blt` at 0xc49b0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc4a20 | `[sp-128] == 0x2` | `[sp-128] != 0x2` |
| `bgt` at 0xc4a2c | `[sp-128] > 0x2 (signed, 32-bit)` | `[sp-128] <= 0x2 (signed, 32-bit)` |
| `beq` at 0xc4a38 | `[sp-128] == 0x1` | `[sp-128] != 0x1` |
| `beq` at 0xc4a48 | `[sp-128] == 0x3` | `[sp-128] != 0x3` |
| `blt` at 0xc4b2c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc4d4c | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `beq` at 0xc4d78 | `[sp+28] == 0x0` | `[sp+28] != 0x0` |
| `beq` at 0xc4da4 | `[sp+32] == 0x0` | `[sp+32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4dc8 | `r0` | `ret(0xc47d4)` |

## `FRUDump`

- `FRUDump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4df0 to 0xc500c.
  Established. The unnamed `0xc37dc` call carries `r2`, `sp-32`, `sp-36`, `sp-40` (`bl` at 0xc4e88).
  The unnamed `0xc48e0` call carries `[sp-20]`, `[sp-24]`, `[sp-32]`, `[sp-36]` (`bl` at 0xc4ee4).
  The zero-word condition `[sp-64] == 0x0` takes `0xc4fb8`, while nonzero falls through (`beq` at 0xc4ef8). FRU dump structure remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4e00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc4e04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc4e08 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc4e10 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xc4e18 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xc4e20 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xc4e34 | store | 4 | `sp -0xc4` | `sp-44` |
| `str` at 0xc4e3c | store | 4 | `sp -0xc0` | `sp-48` |
| `str` at 0xc4e44 | store | 4 | `sp -0xbc` | `sp-52` |
| `str` at 0xc4e4c | store | 4 | `sp -0xb8` | `sp-56` |
| `str` at 0xc4e54 | store | 4 | `sp -0xb4` | `sp-60` |
| `str` at 0xc4e5c | store | 4 | `sp -0xb0` | `sp-64` |
| `str` at 0xc4e64 | store | 4 | `sp -0xac` | `sp-72` |
| `str` at 0xc4e6c | store | 4 | `sp -0xa8` | `sp-76` |
| `str` at 0xc4e74 | store | 4 | `sp -0xa4` | `sp-80` |
| `ldr` at 0xc4e78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4e8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc4e90 | store | 4 | `sp -0xc4` | `[sp-44]` |
| `ldr` at 0xc4e94 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc4e98 | store | 4 | `sp -0xc0` | `[sp-48]` |
| `ldr` at 0xc4e9c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc4ea0 | store | 4 | `sp -0xbc` | `[sp-52]` |
| `ldr` at 0xc4ea4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc4ea8 | store | 4 | `sp -0xb8` | `[sp-56]` |
| `ldr` at 0xc4eac | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc4eb0 | store | 4 | `sp -0xb4` | `[sp-60]` |
| `ldr` at 0xc4eb4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc4eb8 | store | 4 | `sp -0xb0` | `[sp-64]` |
| `ldr` at 0xc4ebc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc4ec0 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xc4ec4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xc4ec8 | store | 4 | `sp -0xa8` | `[sp-76]` |
| `ldr` at 0xc4ecc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xc4ed0 | store | 4 | `sp -0xa4` | `[sp-80]` |
| `ldr` at 0xc4ed4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ed8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4edc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc4ee0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc4eec | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xc4ef0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4efc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4f00 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc4f10 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4f24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4f2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4f38 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4f4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4f54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4f5c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4f70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4f78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4f84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4f98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4fa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4fa8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc4fb0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc4fb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4fc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4fcc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc4fd8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc4fe0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc4fec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc4ff4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xc5000 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xc5008 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5008 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4e88 | `(unknown callee) 0xc37dc` | `r2` | `sp-32` | `sp-36` | `sp-40` |
| `bl` at 0xc4ee4 | `(unknown callee) 0xc48e0` | `[sp-20]` | `[sp-24]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0xc4f58 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xc4fa4 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xc4fc8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xc4fdc | `(unknown callee) 0x1b490` | `[sp-72]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xc4ff0 | `(unknown callee) 0x1b490` | `[sp-76]` | `?` | `?` | `[sp-76]` |
| `bl` at 0xc5004 | `(unknown callee) 0x1b490` | `[sp-80]` | `?` | `?` | `[sp-80]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc4ef8 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `blt` at 0xc4f08 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc4f34 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc4f80 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc4fc0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xc4fd4 | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `beq` at 0xc4fe8 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `beq` at 0xc4ffc | `[sp-80] == 0x0` | `[sp-80] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5008 | `r0` | `?` |

## `FruFmtMultiRec`

- `FruFmtMultiRec`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3b60 to 0xb429c.
  Established. The byte assignment through `r3 +0x0` writes `0x0` (`strb` at 0xb3b8c). The selector
  takes unsigned `[sp-28] <= 0x5` to an unresolved destination, while larger values fall through
  (`ldrls` at 0xb3b98). The known return reports entry `r0` (`ldmdb` at 0xb423c). Multi-record
  formatting effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3b70 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xb3b74 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xb3b78 | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0xb3b80 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xb3b84 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xb3b8c | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0xb3b90 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0xb3b98 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb423c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb3b98 | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb423c | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb3b98).
