# ARM Linux 2.6 NVRAM records in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
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
Computed transfers in nvramInit, nvramSet, nvramGet, nvramSlotSet and nvramSlotGet
leave unresolved edges. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-nvram-records-facts.txt) retain all tool sections.

## `nvramInit`

- `nvramInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24af0 to 0x259d4.
  Established. A word assignment increments `[0x1a5948]` by one at `0x1a5948` (`str` at 0x24b44).
  Another word assignment stores `ret(0x2046c)` at `0x1a596c` (`str` at 0x24b78).
  The unnamed `0x20628` call carries `[0x1a596c]`, `0x318480`, `0x2000` and `0x1a596c` (`bl` at 0x24be4).
  The computed transfer remains unresolved, so omitted initialization choices and physical NVRAM roles remain Unknown (`ldrls` at 0x24d18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24b04 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x24b08 | load | 4 | `abs 0x25960` | `-` |
| `ldr` at 0x24b0c | load | 4 | `abs 0x1a5948` | `-` |
| `ldr` at 0x24b18 | load | 4 | `abs 0x25964` | `-` |
| `str` at 0x24b1c | store | 4 | `sp -0x68` | `0x153328` |
| `ldr` at 0x24b24 | load | 4 | `abs 0x25968` | `-` |
| `ldr` at 0x24b34 | load | 4 | `abs 0x25960` | `-` |
| `ldr` at 0x24b38 | load | 4 | `abs 0x25960` | `-` |
| `ldr` at 0x24b3c | load | 4 | `abs 0x1a5948` | `-` |
| `str` at 0x24b44 | store | 4 | `abs 0x1a5948` | `[0x1a5948]+1` |
| `ldr` at 0x24b48 | load | 4 | `abs 0x2596c` | `-` |
| `ldr` at 0x24b58 | load | 4 | `abs 0x25970` | `-` |
| `str` at 0x24b5c | store | 4 | `sp -0x24` | `0x153348` |
| `ldr` at 0x24b60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24b64 | load | 4 | `abs 0x25974` | `-` |
| `ldr` at 0x24b68 | load | 4 | `abs 0x25978` | `-` |
| `ldr` at 0x24b74 | load | 4 | `abs 0x2597c` | `-` |
| `str` at 0x24b78 | store | 4 | `abs 0x1a596c` | `ret(0x2046c)` |
| `ldr` at 0x24b7c | load | 4 | `abs 0x1a596c` | `-` |
| `ldr` at 0x24b90 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x24b9c | load | 4 | `abs 0x25980` | `-` |
| `str` at 0x24ba0 | store | 4 | `sp -0x68` | `0x153358` |
| `ldr` at 0x24ba4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x24ba8 | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x24bac | store | 4 | `sp -0x60` | `ret(0x219a4)` |
| `ldr` at 0x24bb4 | load | 4 | `abs 0x25968` | `-` |
| `ldr` at 0x24bc4 | load | 4 | `abs 0x25984` | `-` |
| `ldr` at 0x24bd4 | load | 4 | `abs 0x2597c` | `-` |
| `ldr` at 0x24bd8 | load | 4 | `abs 0x1a596c` | `-` |
| `ldr` at 0x24bdc | load | 4 | `abs 0x25984` | `-` |
| `ldr` at 0x24bfc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x24c08 | load | 4 | `abs 0x25988` | `-` |
| `str` at 0x24c0c | store | 4 | `sp -0x68` | `0x15337c` |
| `ldr` at 0x24c10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x24c14 | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x24c18 | store | 4 | `sp -0x60` | `ret(0x219a4)` |
| `ldr` at 0x24c20 | load | 4 | `abs 0x25968` | `-` |
| `ldr` at 0x24c30 | load | 4 | `abs 0x2598c` | `-` |
| `ldr` at 0x24c34 | load | 4 | `abs 0x25990` | `-` |
| `ldr` at 0x24c44 | load | 4 | `abs 0x25994` | `-` |
| `ldr` at 0x24c48 | load | 4 | `abs 0x25984` | `-` |
| `str` at 0x24c4c | store | 4 | `abs 0x1a58d4` | `0x318480` |
| `ldr` at 0x24c50 | load | 4 | `abs 0x25994` | `-` |
| `ldr` at 0x24c54 | load | 4 | `abs 0x1a58d4` | `-` |
| `ldr` at 0x24c74 | load | 4 | `abs 0x25994` | `-` |
| `ldr` at 0x24c78 | load | 4 | `abs 0x1a58d4` | `-` |
| `ldrb` at 0x24c7c | load | 1 | `[0x1a58d4] +0x0` | `-` |
| `ldr` at 0x24c90 | load | 4 | `abs 0x25994` | `-` |
| `ldr` at 0x24c94 | load | 4 | `abs 0x1a58d4` | `-` |
| `ldrb` at 0x24c98 | load | 1 | `[0x1a58d4] +0x1` | `-` |
| `ldr` at 0x24ca8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x24cac | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x24cb8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x24cbc | load | 4 | `abs 0x25994` | `-` |
| `ldr` at 0x24cc0 | load | 4 | `abs 0x1a58d4` | `-` |
| `ldrb` at 0x24cc4 | load | 1 | `[0x1a58d4] +0x1` | `-` |
| `ldr` at 0x24ccc | load | 4 | `abs 0x25984` | `-` |
| `str` at 0x24cd4 | store | 4 | `abs 0x1a58dc` | `?` |
| `ldr` at 0x24cd8 | load | 4 | `abs 0x2599c` | `-` |
| `ldr` at 0x24cdc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x24ce0 | load | 4 | `abs 0x1a58dc` | `-` |
| `str` at 0x24cec | store | 4 | `abs 0x1a58e0` | `[0x1a58dc]+2360` |
| `ldr` at 0x24cf0 | load | 4 | `abs 0x259a0` | `-` |
| `ldr` at 0x24cf4 | load | 4 | `abs 0x259a4` | `-` |
| `str` at 0x24cf8 | store | 4 | `abs 0x1a58e4` | `0x31a478` |
| `ldr` at 0x24cfc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x24d00 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldrb` at 0x24d04 | load | 1 | `[0x1a58dc] +0x0` | `-` |
| `ldrls` at 0x24d18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x253b8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x253bc | load | 4 | `abs 0x1a58dc` | `-` |
| `str` at 0x253dc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x253e0 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x253e4 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldrb` at 0x253ec | load | 1 | `[0x1a58dc] +0x7d0` | `-` |
| `ldr` at 0x253fc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25400 | load | 4 | `abs 0x1a58dc` | `-` |
| `str` at 0x25428 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x25430 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25434 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25438 | load | 4 | `abs 0x259b4` | `-` |
| `strb` at 0x25440 | store | 1 | `[0x1a58dc] +0x7d4` | `0x0` |
| `ldr` at 0x25444 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25448 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2544c | load | 4 | `[0x1a58dc] +0x7d4` | `-` |
| `str` at 0x25450 | store | 4 | `sp -0x5c` | `[[0x1a58dc]+2004]` |
| `ldr` at 0x25454 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x2545c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x25460 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x25464 | store | 4 | `[0x1a58dc] +0x7d4` | `[sp-92]` |
| `ldr` at 0x25468 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2546c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25488 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2548c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x254a4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x254a8 | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x254b4 | store | 1 | `[0x1a58dc] +0x7d0` | `0x3` |
| `ldr` at 0x254b8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x254bc | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x254c0 | load | 4 | `abs 0x259ac` | `-` |
| `ldr` at 0x254c4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x254c8 | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x254d4 | store | 1 | `[0x1a58dc] +0x3c3` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x254d8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x254dc | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x254e0 | load | 4 | `abs 0x259b8` | `-` |
| `ldr` at 0x254e4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x254e8 | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x254fc | store | 1 | `[0x1a58dc] +0x883` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x25500 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25504 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25508 | load | 4 | `abs 0x259bc` | `-` |
| `ldrb` at 0x2550c | load | 1 | `[0x1a58dc] +0x788` | `-` |
| `ldr` at 0x2551c | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25520 | load | 4 | `abs 0x1a58dc` | `-` |
| `str` at 0x2554c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x25554 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25558 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2555c | load | 4 | `abs 0x259bc` | `-` |
| `strb` at 0x25564 | store | 1 | `[0x1a58dc] +0x788` | `0x1` |
| `ldr` at 0x25568 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2556c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25570 | load | 4 | `abs 0x259c0` | `-` |
| `ldr` at 0x25574 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25578 | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x25590 | store | 1 | `[0x1a58dc] +0x7ab` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x25594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x255a8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x255ac | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x255b4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x255bc | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x255c8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x255cc | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x255d8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x255dc | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x255f4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x255f8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25604 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25608 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25664 | load | 4 | `abs 0x259a0` | `-` |
| `ldr` at 0x25668 | load | 4 | `abs 0x2599c` | `-` |
| `ldr` at 0x2566c | load | 4 | `abs 0x1a58e4` | `-` |
| `ldr` at 0x25670 | load | 4 | `abs 0x1a58e0` | `-` |
| `ldr` at 0x25678 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2567c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25680 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25684 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25688 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x2568c | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x2569c | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x256a0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x256a4 | load | 4 | `abs 0x2599c` | `-` |
| `ldr` at 0x256a8 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x256ac | load | 4 | `abs 0x1a58e0` | `-` |
| `ldr` at 0x256b8 | load | 4 | `abs 0x2599c` | `-` |
| `ldr` at 0x256bc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x256c0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x256c4 | load | 4 | `abs 0x1a58e0` | `-` |
| `ldr` at 0x256c8 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x256d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x256d4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x256d8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x256dc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x256e0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x256e4 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x256e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x256f0 | store | 4 | `[0x1a58dc] +0x78c` | `([[0x1a58dc]+1932] + [sp-36])` |
| `ldr` at 0x256f4 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x256f8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x256fc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25700 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25704 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x25708 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x25710 | store | 4 | `[0x1a58dc] +0x790` | `([[0x1a58dc]+1936] + [sp-36])` |
| `ldr` at 0x25714 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25718 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2571c | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25720 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25724 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x25728 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x25730 | store | 4 | `[0x1a58dc] +0x794` | `([[0x1a58dc]+1940] + [sp-36])` |
| `ldr` at 0x25734 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25738 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2573c | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25740 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25744 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x25748 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x25750 | store | 4 | `[0x1a58dc] +0x798` | `([[0x1a58dc]+1944] + [sp-36])` |
| `ldr` at 0x25754 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25758 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2575c | load | 4 | `abs 0x259c0` | `-` |
| `ldr` at 0x25760 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25764 | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x2577c | store | 1 | `[0x1a58dc] +0x7ab` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x25780 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25784 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25790 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25794 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x257b0 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x257b4 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x257b8 | load | 4 | `abs 0x2599c` | `-` |
| `ldr` at 0x257bc | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x257c0 | load | 4 | `abs 0x1a58e0` | `-` |
| `ldr` at 0x257cc | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x257d0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x257d4 | load | 4 | `abs 0x259a0` | `-` |
| `ldr` at 0x257d8 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x257dc | load | 4 | `abs 0x1a58e4` | `-` |
| `ldr` at 0x257e8 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x257ec | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x257f0 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x257f4 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x257f8 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x257fc | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x25808 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2580c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25810 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25814 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25818 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x2581c | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x25828 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2582c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25830 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25834 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25838 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x2583c | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x25848 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x2584c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25850 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25854 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25858 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x2585c | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x2586c | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25870 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25874 | load | 4 | `abs 0x2599c` | `-` |
| `ldr` at 0x25878 | load | 4 | `abs 0x1a58e0` | `-` |
| `str` at 0x2587c | store | 4 | `[0x1a58dc] +0x78c` | `[0x1a58e0]` |
| `ldr` at 0x25880 | load | 4 | `abs 0x25998` | `-` |
| `ldr` at 0x25884 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x25888 | load | 4 | `abs 0x259a0` | `-` |
| `ldr` at 0x2588c | load | 4 | `abs 0x1a58e4` | `-` |
| `str` at 0x25890 | store | 4 | `[0x1a58dc] +0x790` | `[0x1a58e4]` |
| `ldr` at 0x25898 | load | 4 | `abs 0x2596c` | `-` |
| `ldr` at 0x2589c | load | 4 | `abs 0x259c4` | `-` |
| `ldrb` at 0x258a0 | load | 1 | `abs 0x1a58f0` | `-` |
| `strb` at 0x258a8 | store | 1 | `abs 0x1a58f0` | `([0x1a58f0]:1+1 & 0xff)` |
| `ldr` at 0x258b0 | load | 4 | `abs 0x259c8` | `-` |
| `str` at 0x258b4 | store | 4 | `(unknown)` | `0x1533b4` |
| `ldr` at 0x258b8 | load | 4 | `abs 0x259c4` | `-` |
| `ldrb` at 0x258bc | load | 1 | `abs 0x1a58f0` | `-` |
| `ldr` at 0x258c8 | load | 4 | `abs 0x259c4` | `-` |
| `strb` at 0x258d0 | store | 1 | `abs 0x1a58f0` | `0x0` |
| `ldr` at 0x258d4 | load | 4 | `abs 0x2596c` | `-` |
| `ldr` at 0x258d8 | load | 4 | `abs 0x259c4` | `-` |
| `ldrb` at 0x258dc | load | 1 | `abs 0x1a58f0` | `-` |
| `ldr` at 0x258e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x258f0 | load | 4 | `abs 0x2596c` | `-` |
| `ldr` at 0x258f4 | load | 4 | `abs 0x259c4` | `-` |
| `ldrb` at 0x258f8 | load | 1 | `abs 0x1a58f0` | `-` |
| `ldr` at 0x25900 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x25908 | load | 4 | `abs 0x259c4` | `-` |
| `ldr` at 0x2590c | load | 4 | `abs 0x259c4` | `-` |
| `ldrb` at 0x25910 | load | 1 | `abs 0x1a58f0` | `-` |
| `strb` at 0x25918 | store | 1 | `abs 0x1a58f0` | `([0x1a58f0]:1+1 & 0xff)` |
| `ldr` at 0x25930 | load | 4 | `abs 0x259cc` | `-` |
| `ldr` at 0x25950 | load | 4 | `abs 0x259d0` | `-` |
| `str` at 0x25954 | store | 4 | `abs 0x1a58ec` | `ret(0x51b28)` |
| `ldm` at 0x2595c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x2595c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x2595c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2595c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2595c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24b30 | `(unknown callee) 0x22070` | `0x9` | `0x1532b0` | `0x71` | `0x0` |
| `bl` at 0x24b54 | `(unknown callee) 0x20550` | `0x1a594c` | `0x0` | `0x20` | `[0x1a5948]+1` |
| `bl` at 0x24b6c | `(unknown callee) 0x2046c` | `0x153348` | `0x1042` | `0x1b6` | `0x153348` |
| `bl` at 0x24b88 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `ret(0x2046c)` | `[0x1a596c]` |
| `bl` at 0x24b94 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x24bc0 | `(unknown callee) 0x22070` | `0x9` | `0x1532b0` | `0x82` | `0x0` |
| `bl` at 0x24bd0 | `(unknown callee) 0x20550` | `0x318480` | `0x0` | `0x2000` | `?` |
| `bl` at 0x24be4 | `(unknown callee) 0x20628` | `[0x1a596c]` | `0x318480` | `0x2000` | `0x1a596c` |
| `bl` at 0x24bf4 | `(unknown callee) 0x20184` | `ret(0x20628)` | `?` | `?` | `ret(0x20628)` |
| `bl` at 0x24c00 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x24c2c | `(unknown callee) 0x22070` | `0x9` | `0x1532b0` | `0x88` | `0x0` |
| `bl` at 0x24c40 | `(unknown callee) 0x22b84` | `0x1a58f4` | `0x1533a8` | `0x1` | `0x9` |
| `bl` at 0x24c5c | `(unknown callee) 0x52794` | `[0x1a58d4]` | `0x8` | `0x1a58d4` | `0x1a58d4` |
| `bl` at 0x24c70 | `(unknown callee) 0x26144` | `ret(0x52794)` | `?` | `?` | `?` |
| `bl` at 0x24ca4 | `(unknown callee) 0x26144` | `?` | `?` | `?` | `?` |
| `bl` at 0x253c4 | `(unknown callee) 0x52794` | `[0x1a58dc]` | `0x3c4` | `?` | `0x1a58dc` |
| `bl` at 0x25410 | `(unknown callee) 0x52794` | `[0x1a58dc]+2000` | `0xb4` | `[0x1a58dc]` | `[0x1a58dc]+2000` |
| `bl` at 0x25484 | `(unknown callee) 0x20550` | `[0x1a58dc]+2008` | `0x0` | `0xa8` | `[0x1a58dc]+2008` |
| `bl` at 0x254a0 | `(unknown callee) 0x20550` | `[0x1a58dc]+956` | `0x0` | `0x7` | `[0x1a58dc]+956` |
| `bl` at 0x254d0 | `(unknown callee) 0x52800` | `[0x1a58dc]` | `0x3c4` | `0x7d0` | `0x1a58dc` |
| `bl` at 0x254f8 | `(unknown callee) 0x52800` | `[0x1a58dc]+2000` | `0xb4` | `?` | `[0x1a58dc]+2000` |
| `bl` at 0x25534 | `(unknown callee) 0x52794` | `[0x1a58dc]+1928` | `0x24` | `[0x1a58dc]` | `[0x1a58dc]+1928` |
| `bl` at 0x2558c | `(unknown callee) 0x52800` | `[0x1a58dc]+1928` | `0x24` | `0x788` | `[0x1a58dc]+1928` |
| `bl` at 0x255a0 | `(unknown callee) 0x259d4` | `?` | `?` | `?` | `[sp-32]` |
| `bl` at 0x255c4 | `(unknown callee) 0x203f4` | `[0x1a58dc]+964` | `[0x1a58dc]` | `0x3c4` | `[0x1a58dc]+964` |
| `bl` at 0x255f0 | `(unknown callee) 0x203f4` | `[0x1a58dc]+2180` | `[0x1a58dc]+2000` | `0xb4` | `[0x1a58dc]+2000` |
| `bl` at 0x25620 | `(unknown callee) 0x203f4` | `[0x1a58dc]+1964` | `[0x1a58dc]+1928` | `0x24` | `[0x1a58dc]+1928` |
| `bl` at 0x25628 | `(unknown callee) 0x28230` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x25640 | `(unknown callee) 0x2624c` | `0x11` | `0x2` | `?` | `ret(0x28230)` |
| `bl` at 0x25648 | `(unknown callee) 0x28230` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x25660 | `(unknown callee) 0x2624c` | `0x3d` | `0x2` | `?` | `ret(0x28230)` |
| `bl` at 0x25778 | `(unknown callee) 0x52800` | `[0x1a58dc]+1928` | `0x24` | `[[0x1a58dc]+1944]` | `[0x1a58dc]+1928` |
| `bl` at 0x257ac | `(unknown callee) 0x203f4` | `[0x1a58dc]+1964` | `[0x1a58dc]+1928` | `0x24` | `[0x1a58dc]+1928` |
| `bl` at 0x25894 | `(unknown callee) 0x29340` | `?` | `?` | `[0x1a58dc]` | `[0x1a58e4]` |
| `bl` at 0x25904 | `(unknown callee) 0x293f8` | `?` | `?` | `0x1a594c` | `?` |
| `bl` at 0x25920 | `(unknown callee) 0x249d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x25928 | `(unknown callee) 0x28230` | `0x46` | `?` | `?` | `?` |
| `bl` at 0x2593c | `(unknown callee) 0x20148` | `0x1533d8` | `ret(0x28230)` | `0x1` | `ret(0x28230)` |
| `bl` at 0x25948 | `(unknown callee) 0x51b28` | `0x5000` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24b14 | `[0x1a5948] == 0x0` | `[0x1a5948] != 0x0` |
| `bge` at 0x24b84 | `[0x1a596c] >= 0x0 (signed, 32-bit)` | `[0x1a596c] < 0x0 (signed, 32-bit)` |
| `bge` at 0x24bf0 | `ret(0x20628) >= 0x0 (signed, 32-bit)` | `ret(0x20628) < 0x0 (signed, 32-bit)` |
| `beq` at 0x24c6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x24c8c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x24ca0 | `[[0x1a58d4]+1]:1 != 0x0` | `[[0x1a58d4]+1]:1 == 0x0` |
| `bne` at 0x24cb4 | `[0x1a58dc] != 0x0` | `[0x1a58dc] == 0x0` |
| `ldrls` at 0x24d18 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |
| `beq` at 0x253d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x253f4 | `[[0x1a58dc]+2000]:1 == 0x3` | `[[0x1a58dc]+2000]:1 != 0x3` |
| `beq` at 0x25420 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x25514 | `[[0x1a58dc]+1928]:1 == 0x1` | `[[0x1a58dc]+1928]:1 != 0x1` |
| `beq` at 0x25544 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2559c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bhi` at 0x25634 | `ret(0x28230) > 0x1 (unsigned, 32-bit)` | `ret(0x28230) <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x25654 | `ret(0x28230) != 0x0` | `ret(0x28230) == 0x0` |
| `bne` at 0x25698 | `? != ?` | `? == ?` |
| `beq` at 0x256b4 | `[[0x1a58dc]+1932] == [0x1a58e0]` | `[[0x1a58dc]+1932] != [0x1a58e0]` |
| `bne` at 0x257c8 | `[[0x1a58dc]+1932] != [0x1a58e0]` | `[[0x1a58dc]+1932] == [0x1a58e0]` |
| `bne` at 0x257e4 | `[[0x1a58dc]+1936] != [0x1a58e4]` | `[[0x1a58dc]+1936] == [0x1a58e4]` |
| `blo` at 0x25804 | `[[0x1a58dc]+1940] < [[0x1a58dc]+1932] (unsigned, 32-bit)` | `[[0x1a58dc]+1940] >= [[0x1a58dc]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x25824 | `[[0x1a58dc]+1940] >= [[0x1a58dc]+1936] (unsigned, 32-bit)` | `[[0x1a58dc]+1940] < [[0x1a58dc]+1936] (unsigned, 32-bit)` |
| `blo` at 0x25844 | `[[0x1a58dc]+1944] < [[0x1a58dc]+1932] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x25864 | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1936] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] < [[0x1a58dc]+1936] (unsigned, 32-bit)` |
| `beq` at 0x258c4 | `[0x1a58f0]:1 == 0x0` | `[0x1a58f0]:1 != 0x0` |
| `beq` at 0x258ec | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2595c | `r0` | `ret(0x51b28)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x24d18).

## `nvramSet`

- `nvramSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2624c to 0x28230.
  Established. Word assignments store entry `r0` and entry `r1` at `sp -0x1c` and `sp -0x20` (`str` at 0x2625c, `str` at 0x26260).
  A conditional word read has unknown effective address when `r0-1 <= 0xcb (unsigned, 32-bit)` (`ldrls` at 0x26270).
  The integer return candidate on the known path is entry `r0` (`ldm` at 0x2822c).
  The computed transfer remains unresolved, and omitted setting effects and hardware field encodings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2625c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x26260 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x26264 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x26270 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x2822c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x2822c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x2822c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2822c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2822c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x26270 | `r0-1 <= 0xcb (unsigned, 32-bit)` | `r0-1 > 0xcb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2822c | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x26270).

## `nvramGet`

- `nvramGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28230 to 0x29340.
  Established. A word assignment stores entry `r0` at `sp -0x14` (`str` at 0x28240).
  A conditional word read has unknown effective address when `r0-1 <= 0xcb (unsigned, 32-bit)` (`ldrls` at 0x28250).
  A known path clears the word at `sp -0x18` and has integer return candidate `0x0` (`str` at 0x2931c, `ldm` at 0x29328).
  The computed transfer remains unresolved, and omitted reading effects and hardware field encodings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28240 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x28244 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x28250 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2931c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x29320 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x29328 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29328 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29328 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x28250 | `r0-1 <= 0xcb (unsigned, 32-bit)` | `r0-1 > 0xcb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29328 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x28250).

## `nvramLogInsert`

- `nvramLogInsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x293f8 to 0x29908.
  Established. A byte assignment writes `0xa` to `[[0x1a58dc]+1944] +0x0` (`strb` at 0x297c8).
  A word assignment writes `[[0x1a58dc]+1944]+1` to `[0x1a58dc] +0x798` (`str` at 0x297d0).
  The unnamed `0x51cb8` call carries `[0x1a58ec]`, `[sp-28]`, `ret(0x2031c)` and `0x1a58ec` (`bl` at 0x2950c).
  Log persistence and physical NVRAM responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29408 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x29418 | store | 4 | `sp -0x24` | `ret(0x202c8)` |
| `str` at 0x29420 | store | 4 | `sp -0x28` | `sp-36` |
| `str` at 0x29428 | store | 4 | `sp -0x2c` | `0x5` |
| `ldr` at 0x2942c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29438 | store | 4 | `sp -0x30` | `ret(0x2031c)` |
| `strb` at 0x29440 | store | 1 | `sp -0x31` | `0x0` |
| `ldr` at 0x29444 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29448 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29458 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2945c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29460 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29464 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29468 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x2946c | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29474 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x29478 | load | 4 | `abs 0x298e8` | `-` |
| `ldr` at 0x2947c | load | 4 | `abs 0x1a58e8` | `-` |
| `ldr` at 0x29490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29494 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x294a0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x294b0 | store | 1 | `sp -0x31` | `0x1` |
| `ldr` at 0x294b4 | load | 4 | `abs 0x298ec` | `-` |
| `ldr` at 0x294cc | load | 4 | `abs 0x298f0` | `-` |
| `ldr` at 0x294d0 | load | 4 | `abs 0x1a2d00` | `-` |
| `ldr` at 0x294e0 | load | 4 | `abs 0x298f4` | `-` |
| `ldr` at 0x294e4 | load | 4 | `abs 0x1a58ec` | `-` |
| `ldr` at 0x294f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29500 | load | 4 | `abs 0x298f4` | `-` |
| `ldr` at 0x29504 | load | 4 | `abs 0x1a58ec` | `-` |
| `ldr` at 0x29508 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x29510 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29514 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29518 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2951c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29520 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x29524 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x29530 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29534 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29538 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2953c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29540 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x29544 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `ldr` at 0x2954c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29550 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29554 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29558 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2955c | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x29560 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `str` at 0x2956c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x29574 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29578 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2957c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29580 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29584 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x29588 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x29594 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29598 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2959c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x295a0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x295a4 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x295a8 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `str` at 0x295b0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x295b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x295b8 | load | 4 | `sp -0x2c` | `-` |
| `ldrsb` at 0x295c0 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x295c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x295d4 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x295d8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x295e4 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldrb` at 0x295e8 | load | 1 | `[[0x1a58dc]+1940] +0x0` | `-` |
| `str` at 0x295f0 | store | 4 | `[0x1a58dc] +0x794` | `[[0x1a58dc]+1940]+1` |
| `ldr` at 0x29600 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x29608 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x2960c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29610 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29614 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29618 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2961c | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x29620 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x2962c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29630 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29634 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29638 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2963c | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29640 | store | 4 | `[0x1a58dc] +0x794` | `[[0x1a58dc]+1932]` |
| `ldr` at 0x29648 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x29650 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x29658 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x29660 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0x2966c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x29678 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2967c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29688 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `strb` at 0x29694 | store | 1 | `[[0x1a58dc]+1944] +0x0` | `?` |
| `str` at 0x2969c | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1944]+1` |
| `ldr` at 0x296a4 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x296a8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x296b4 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x296bc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x296c0 | load | 1 | `[sp-40] +0x0` | `-` |
| `str` at 0x296c8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strb` at 0x296d0 | store | 1 | `[[0x1a58dc]+1944] +0x0` | `([[sp-40]]:1 & 0xff)` |
| `str` at 0x296d8 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1944]+1` |
| `ldr` at 0x296dc | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x296e0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x296e4 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x296e8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x296ec | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x296f0 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x296fc | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29700 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29704 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29708 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2970c | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29710 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1932]` |
| `ldr` at 0x29718 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x29720 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0x2972c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29730 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2973c | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x29744 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x29748 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x29750 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `strb` at 0x29758 | store | 1 | `[[0x1a58dc]+1944] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x29760 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1944]+1` |
| `ldr` at 0x29764 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29768 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2976c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29770 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29774 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x29778 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x29784 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29788 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x2978c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29790 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29794 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29798 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1932]` |
| `ldrsb` at 0x297a0 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x297ac | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x297b0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x297bc | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `strb` at 0x297c8 | store | 1 | `[[0x1a58dc]+1944] +0x0` | `0xa` |
| `str` at 0x297d0 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1944]+1` |
| `ldr` at 0x297d4 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x297d8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x297dc | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x297e0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x297e4 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x297e8 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `ldr` at 0x297f4 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x297f8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x297fc | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29800 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29804 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29808 | store | 4 | `[0x1a58dc] +0x798` | `[[0x1a58dc]+1932]` |
| `ldr` at 0x2980c | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x29810 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29814 | load | 4 | `abs 0x298f8` | `-` |
| `ldr` at 0x29818 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2981c | load | 4 | `abs 0x1a58dc` | `-` |
| `strb` at 0x29834 | store | 1 | `[0x1a58dc] +0x7ab` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x29838 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2983c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29848 | load | 4 | `abs 0x298e4` | `-` |
| `ldr` at 0x2984c | load | 4 | `abs 0x1a58dc` | `-` |
| `str` at 0x29870 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x29874 | load | 4 | `abs 0x298ec` | `-` |
| `ldr` at 0x29888 | load | 4 | `abs 0x298f0` | `-` |
| `ldr` at 0x2988c | load | 4 | `abs 0x1a2d00` | `-` |
| `str` at 0x2989c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x298a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x298a4 | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x298b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x298bc | load | 4 | `abs 0x298fc` | `-` |
| `str` at 0x298c0 | store | 4 | `sp -0x44` | `0x1535a8` |
| `str` at 0x298c4 | store | 4 | `sp -0x40` | `ret(0x219a4)` |
| `ldr` at 0x298cc | load | 4 | `abs 0x29900` | `-` |
| `ldr` at 0x298d0 | load | 4 | `abs 0x29904` | `-` |
| `ldm` at 0x298e0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x298e0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x298e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x298e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x298e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29410 | `(unknown callee) 0x202c8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x29430 | `(unknown callee) 0x2031c` | `[sp-28]` | `?` | `?` | `0x5` |
| `bl` at 0x294bc | `(unknown callee) 0x22ca0` | `0x1a58f4` | `?` | `[sp-28]` | `?` |
| `bl` at 0x294f4 | `(unknown callee) 0x2031c` | `[sp-28]` | `?` | `?` | `[0x1a58ec]` |
| `bl` at 0x2950c | `(unknown callee) 0x51cb8` | `[0x1a58ec]` | `[sp-28]` | `ret(0x2031c)` | `0x1a58ec` |
| `bl` at 0x29830 | `(unknown callee) 0x52800` | `[0x1a58dc]+1928` | `0x24` | `?` | `[0x1a58dc]+1928` |
| `bl` at 0x29864 | `(unknown callee) 0x203f4` | `[0x1a58dc]+1964` | `[0x1a58dc]+1928` | `0x24` | `[0x1a58dc]+1928` |
| `bl` at 0x29868 | `(unknown callee) 0x249d4` | `ret(0x203f4)` | `?` | `?` | `?` |
| `bl` at 0x29878 | `(unknown callee) 0x2302c` | `0x1a58f4` | `?` | `?` | `0x0` |
| `bl` at 0x298b4 | `(unknown callee) 0x219a4` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x298d8 | `(unknown callee) 0x22070` | `0x9` | `0x1532b0` | `0x77a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x29450 | `[0x1a58dc] != 0x0` | `[0x1a58dc] == 0x0` |
| `beq` at 0x29484 | `[0x1a58e8] == 0x0` | `[0x1a58e8] != 0x0` |
| `beq` at 0x294a8 | `? == 0xa` | `? != 0xa` |
| `beq` at 0x294c8 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0x294d8 | `[0x1a2d00] == 0x1` | `[0x1a2d00] != 0x1` |
| `beq` at 0x294ec | `[0x1a58ec] == 0x0` | `[0x1a58ec] != 0x0` |
| `bls` at 0x2952c | `[[0x1a58dc]+1944] <= [[0x1a58dc]+1940] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] > [[0x1a58dc]+1940] (unsigned, 32-bit)` |
| `bhs` at 0x29590 | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1940] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] < [[0x1a58dc]+1940] (unsigned, 32-bit)` |
| `blt` at 0x295d0 | `? < [sp-56] (signed, 32-bit)` | `? >= [sp-56] (signed, 32-bit)` |
| `beq` at 0x295fc | `? == 0xa` | `? != 0xa` |
| `blo` at 0x29628 | `[[0x1a58dc]+1940] < [[0x1a58dc]+1936] (unsigned, 32-bit)` | `[[0x1a58dc]+1940] >= [[0x1a58dc]+1936] (unsigned, 32-bit)` |
| `beq` at 0x29668 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x29674 | `[sp-44] != 0x4` | `[sp-44] == 0x4` |
| `blo` at 0x296f8 | `[[0x1a58dc]+1944] < [[0x1a58dc]+1936] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1936] (unsigned, 32-bit)` |
| `beq` at 0x29728 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0x29780 | `[[0x1a58dc]+1944] < [[0x1a58dc]+1936] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1936] (unsigned, 32-bit)` |
| `beq` at 0x297a8 | `? == 0x0` | `? != 0x0` |
| `blo` at 0x297f0 | `[[0x1a58dc]+1944] < [[0x1a58dc]+1936] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1936] (unsigned, 32-bit)` |
| `beq` at 0x29884 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |
| `beq` at 0x29894 | `[0x1a2d00] == 0x1` | `[0x1a2d00] != 0x1` |
| `beq` at 0x298ac | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x298e0 | `r0` | `?` |

## `nvramLogFetch`

- `nvramLogFetch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29950 to 0x29d2c.
  Established. A byte assignment clears `ret(0x201fc) +0x0` (`strb` at 0x29a50).
  Nonzero `ret(0x201fc)` takes `0x29a48`, while zero falls through (`bne` at 0x29a38).
  The unnamed `0x20214` call carries `([sp-20] + [sp-68])`, `0x13`, `0x1535d0` and `ret(0x20094)` (`bl` at 0x29c38).
  Fetched log representation and hardware ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29964 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x29968 | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x2996c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29970 | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x29974 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29978 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x2997c | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x29988 | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x2998c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29990 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `str` at 0x29994 | store | 4 | `sp -0x18` | `[[0x1a58dc]+1940]` |
| `ldr` at 0x29998 | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x2999c | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x299a0 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `str` at 0x299a4 | store | 4 | `sp -0x1c` | `[[0x1a58dc]+1944]` |
| `str` at 0x299ac | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x299b4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x299bc | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x299c0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x299c4 | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x299c8 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x299cc | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `ldr` at 0x299d0 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `ldr` at 0x299dc | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x299e0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x299e4 | load | 4 | `[0x1a58dc] +0x794` | `-` |
| `str` at 0x299e8 | store | 4 | `sp -0x18` | `[[0x1a58dc]+1940]` |
| `ldr` at 0x299ec | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x299f0 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x299f4 | load | 4 | `[0x1a58dc] +0x790` | `-` |
| `str` at 0x299f8 | store | 4 | `sp -0x1c` | `[[0x1a58dc]+1936]` |
| `ldr` at 0x299fc | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x29a00 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29a04 | load | 4 | `[0x1a58dc] +0x78c` | `-` |
| `str` at 0x29a08 | store | 4 | `sp -0x20` | `[[0x1a58dc]+1932]` |
| `ldr` at 0x29a0c | load | 4 | `abs 0x29d24` | `-` |
| `ldr` at 0x29a10 | load | 4 | `abs 0x1a58dc` | `-` |
| `ldr` at 0x29a14 | load | 4 | `[0x1a58dc] +0x798` | `-` |
| `str` at 0x29a18 | store | 4 | `sp -0x24` | `[[0x1a58dc]+1944]` |
| `str` at 0x29a2c | store | 4 | `sp -0x14` | `ret(0x201fc)` |
| `ldr` at 0x29a30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29a40 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x29a48 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x29a50 | store | 1 | `ret(0x201fc) +0x0` | `0x0` |
| `ldr` at 0x29a54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29a58 | store | 4 | `sp -0x48` | `[sp-20]` |
| `str` at 0x29a64 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x29a68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29a6c | store | 4 | `sp -0x2c` | `[[0x1a58dc]+1940]` |
| `ldr` at 0x29a70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29a74 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x29a78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x29a84 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x29a88 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x29a8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x29a90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x29a9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x29aa8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x29ab0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x29ab8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x29abc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x29ac8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x29ad0 | store | 4 | `sp -0x44` | `?` |
| `str` at 0x29ad8 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x29ae0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x29ae8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x29aec | load | 4 | `sp -0x28` | `-` |
| `str` at 0x29af4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x29afc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x29b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x29b10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x29b14 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x29b18 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x29b1c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x29b20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x29b24 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x29b28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x29b2c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x29b34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x29b40 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x29b48 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x29b54 | store | 4 | `sp -0x4c` | `0x64` |
| `ldr` at 0x29b58 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x29b5c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x29b60 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x29b6c | store | 4 | `sp -0x14` | `ret(0x201fc)` |
| `ldr` at 0x29b70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29b80 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x29b8c | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x29b94 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x29b98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29b9c | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x29ba0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29ba4 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x29ba8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x29bb4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x29bb8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x29bbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x29bc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x29bcc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x29bdc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x29be0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x29be8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x29bec | store | 1 | `[sp-60] +0x0` | `?` |
| `str` at 0x29bf4 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x29bf8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x29c00 | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0x29c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29c20 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x29c30 | load | 4 | `abs 0x29d28` | `-` |
| `ldr` at 0x29c3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29c40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x29c58 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x29c60 | store | 4 | `sp -0x44` | `([sp-68] + ret(0x2031c))` |
| `ldr` at 0x29c68 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x29c6c | load | 1 | `(unknown)` | `-` |
| `str` at 0x29c7c | store | 4 | `sp -0x3c` | `sp-56` |
| `str` at 0x29c84 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x29c90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x29c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29ca0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x29ca4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x29ca8 | store | 1 | `(unknown)` | `?` |
| `str` at 0x29cb0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x29cb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x29cbc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x29cc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x29cc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x29cd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x29cdc | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x29ce0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x29ce4 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x29ce8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x29cec | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x29cf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x29cf4 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x29cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29d00 | load | 4 | `sp -0x44` | `-` |
| `strb` at 0x29d0c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x29d10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29d14 | store | 4 | `sp -0x48` | `[sp-20]` |
| `ldr` at 0x29d18 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0x29d20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x29d20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x29d20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29a24 | `(unknown callee) 0x201fc` | `0x1` | `r1` | `[[0x1a58dc]+1944]` | `[[0x1a58dc]+1940]` |
| `bl` at 0x29b64 | `(unknown callee) 0x201fc` | `?` | `r1` | `?` | `?` |
| `bl` at 0x29c14 | `(unknown callee) 0x20094` | `sp-56` | `[sp-60]` | `[sp-60]+1` | `sp-56` |
| `bl` at 0x29c38 | `(unknown callee) 0x20214` | `([sp-20] + [sp-68])` | `0x13` | `0x1535d0` | `ret(0x20094)` |
| `bl` at 0x29c4c | `(unknown callee) 0x2031c` | `([sp-20] + [sp-68])` | `?` | `[sp-20]` | `([sp-20] + [sp-68])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x29984 | `[[0x1a58dc]+1944] <= [[0x1a58dc]+1940] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] > [[0x1a58dc]+1940] (unsigned, 32-bit)` |
| `bhs` at 0x299d8 | `[[0x1a58dc]+1944] >= [[0x1a58dc]+1940] (unsigned, 32-bit)` | `[[0x1a58dc]+1944] < [[0x1a58dc]+1940] (unsigned, 32-bit)` |
| `bne` at 0x29a38 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0x29a80 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x29a98 | `C == 1` | `not (C == 1)` |
| `beq` at 0x29aa4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x29ac4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x29b08 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x29b3c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x29b78 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0x29bb0 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x29bc8 | `C == 1` | `not (C == 1)` |
| `beq` at 0x29bd4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x29c08 | `[sp-52]-1 != 0x0` | `[sp-52]-1 == 0x0` |
| `bne` at 0x29c74 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x29cd0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x29d20 | `r0` | `?` |

## `nvramSlotInit`

- `nvramSlotInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41814 to 0x41a38.
  Established. Word assignments store `ret(0x204d8)` at `0x1a5ab4` and `ret(0x2046c)` at `0x1a5b34` (`str` at 0x4188c, `str` at 0x418d0).
  The unnamed `0x20628` call carries `[0x1a5b34]`, `[0x1a5ab4]`, unknown and unknown (`bl` at 0x41960).
  The unnamed `0x22b84` call carries `0x1a5abc`, `0x15af60`, `0x1` and `0x9` (`bl` at 0x419bc).
  Slot initialization and physical NVRAM ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41824 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x41828 | load | 4 | `abs 0x419fc` | `-` |
| `str` at 0x4182c | store | 4 | `sp -0x20` | `0x15ae98` |
| `ldr` at 0x41830 | load | 4 | `abs 0x41a00` | `-` |
| `ldr` at 0x41834 | load | 4 | `abs 0x1a5b10` | `-` |
| `ldr` at 0x41840 | load | 4 | `abs 0x41a04` | `-` |
| `str` at 0x41844 | store | 4 | `sp -0x2c` | `0x15aeb0` |
| `ldr` at 0x4184c | load | 4 | `abs 0x41a08` | `-` |
| `ldr` at 0x4185c | load | 4 | `abs 0x41a00` | `-` |
| `ldr` at 0x41860 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x41864 | store | 4 | `abs 0x1a5b10` | `?` |
| `ldr` at 0x41868 | load | 4 | `abs 0x41a0c` | `-` |
| `ldr` at 0x41878 | load | 4 | `abs 0x41a10` | `-` |
| `ldr` at 0x4187c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41880 | load | 4 | `abs 0x41a14` | `-` |
| `str` at 0x4188c | store | 4 | `abs 0x1a5ab4` | `ret(0x204d8)` |
| `ldr` at 0x41890 | load | 4 | `abs 0x1a5ab4` | `-` |
| `ldr` at 0x4189c | load | 4 | `abs 0x41a18` | `-` |
| `str` at 0x418a0 | store | 4 | `sp -0x2c` | `0x15aed8` |
| `ldr` at 0x418a8 | load | 4 | `abs 0x41a08` | `-` |
| `ldr` at 0x418b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x418bc | load | 4 | `abs 0x41a1c` | `-` |
| `ldr` at 0x418c0 | load | 4 | `abs 0x41a20` | `-` |
| `ldr` at 0x418cc | load | 4 | `abs 0x41a24` | `-` |
| `str` at 0x418d0 | store | 4 | `abs 0x1a5b34` | `ret(0x2046c)` |
| `ldr` at 0x418d4 | load | 4 | `abs 0x1a5b34` | `-` |
| `ldr` at 0x418e8 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x418f4 | load | 4 | `abs 0x41a28` | `-` |
| `str` at 0x418f8 | store | 4 | `sp -0x2c` | `0x15af08` |
| `ldr` at 0x418fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x41900 | store | 4 | `sp -0x28` | `[sp-32]` |
| `str` at 0x41904 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x4190c | load | 4 | `abs 0x41a08` | `-` |
| `ldr` at 0x4191c | load | 4 | `abs 0x41a24` | `-` |
| `ldr` at 0x41920 | load | 4 | `abs 0x41a10` | `-` |
| `ldr` at 0x41924 | load | 4 | `abs 0x41a00` | `-` |
| `ldr` at 0x41928 | load | 4 | `abs 0x1a5b10` | `-` |
| `ldr` at 0x41954 | load | 4 | `abs 0x1a5b34` | `-` |
| `ldr` at 0x41958 | load | 4 | `abs 0x1a5ab4` | `-` |
| `ldr` at 0x41978 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x41984 | load | 4 | `abs 0x41a2c` | `-` |
| `str` at 0x41988 | store | 4 | `sp -0x2c` | `0x15af30` |
| `ldr` at 0x4198c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x41990 | store | 4 | `sp -0x28` | `[sp-32]` |
| `str` at 0x41994 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x4199c | load | 4 | `abs 0x41a08` | `-` |
| `ldr` at 0x419ac | load | 4 | `abs 0x41a30` | `-` |
| `ldr` at 0x419b0 | load | 4 | `abs 0x41a34` | `-` |
| `str` at 0x419c4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x419c8 | load | 4 | `abs 0x41a00` | `-` |
| `ldr` at 0x419cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x419d0 | load | 4 | `abs 0x1a5b10` | `-` |
| `ldr` at 0x419dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x419e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x419ec | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x419f8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x419f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x419f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x419f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41858 | `(unknown callee) 0x22070` | `0x9` | `0x15adf0` | `0xbc` | `0x0` |
| `bl` at 0x41874 | `(unknown callee) 0x20550` | `0x1a5b14` | `0x0` | `0x20` | `?` |
| `bl` at 0x41884 | `(unknown callee) 0x204d8` | `[sp-24]` | `0x6718` | `?` | `?` |
| `bl` at 0x418b4 | `(unknown callee) 0x22070` | `0x9` | `0x15adf0` | `0xc7` | `0x0` |
| `bl` at 0x418c4 | `(unknown callee) 0x2046c` | `[sp-32]` | `0x1042` | `0x1b6` | `?` |
| `bl` at 0x418e0 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `ret(0x2046c)` | `[0x1a5b34]` |
| `bl` at 0x418ec | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x41918 | `(unknown callee) 0x22070` | `0x9` | `0x15adf0` | `0xd0` | `0x0` |
| `bl` at 0x41960 | `(unknown callee) 0x20628` | `[0x1a5b34]` | `[0x1a5ab4]` | `?` | `?` |
| `bl` at 0x41970 | `(unknown callee) 0x20184` | `ret(0x20628)` | `?` | `?` | `ret(0x20628)` |
| `bl` at 0x4197c | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x419a8 | `(unknown callee) 0x22070` | `0x9` | `0x15adf0` | `0xd5` | `0x0` |
| `bl` at 0x419bc | `(unknown callee) 0x22b84` | `0x1a5abc` | `0x15af60` | `0x1` | `0x9` |
| `bl` at 0x419e0 | `(unknown callee) 0x41518` | `?` | `?` | `?` | `[0x1a5b10]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4183c | `[0x1a5b10] == 0x0` | `[0x1a5b10] != 0x0` |
| `bne` at 0x41898 | `[0x1a5ab4] != 0x0` | `[0x1a5ab4] == 0x0` |
| `bge` at 0x418dc | `[0x1a5b34] >= 0x0 (signed, 32-bit)` | `[0x1a5b34] < 0x0 (signed, 32-bit)` |
| `bge` at 0x4196c | `ret(0x20628) >= 0x0 (signed, 32-bit)` | `ret(0x20628) < 0x0 (signed, 32-bit)` |
| `bge` at 0x419d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x419f8 | `r0` | `?` |

## `nvramSlotSet`

- `nvramSlotSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41d74 to 0x42428.
  Established. An included word read addresses `0x1a5ab4` (`ldr` at 0x41dc0).
  A word assignment stores entry `r2` at `sp -0x24` (`str` at 0x41d8c).
  A conditional word read has unknown effective address when `r1-501 <= 0x36 (unsigned, 32-bit)` (`ldrls` at 0x41ddc).
  The computed transfer remains unresolved, and omitted slot-setting effects and physical NVRAM fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41d84 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x41d88 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x41d8c | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x41d90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41dbc | load | 4 | `abs 0x4241c` | `-` |
| `ldr` at 0x41dc0 | load | 4 | `abs 0x1a5ab4` | `-` |
| `str` at 0x41dc8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x41dcc | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0x41ddc | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x42418 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x42418 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x42418 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42418 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x41ddc | `r1-501 <= 0x36 (unsigned, 32-bit)` | `r1-501 > 0x36 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42418 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x41ddc).

## `nvramSlotGet`

- `nvramSlotGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42428 to 0x4281c.
  Established. An included word read addresses `0x1a5ab4` (`ldr` at 0x42470).
  A conditional word read has unknown effective address when `r1-501 <= 0x36 (unsigned, 32-bit)` (`ldrls` at 0x4248c).
  A known path clears the word at `sp -0x20` and has integer return candidate `0x0` (`str` at 0x42808, `ldm` at 0x42814).
  The computed transfer remains unresolved, and omitted slot-reading effects and hardware fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42438 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4243c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x42440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4246c | load | 4 | `abs 0x42818` | `-` |
| `ldr` at 0x42470 | load | 4 | `abs 0x1a5ab4` | `-` |
| `str` at 0x42478 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4247c | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x4248c | load | 4 | `(unknown)` | `-` |
| `str` at 0x42808 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4280c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x42814 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42814 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42814 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x4248c | `r1-501 <= 0x36 (unsigned, 32-bit)` | `r1-501 > 0x36 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42814 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4248c).

## `nvramSlotLogReset`

- `nvramSlotLogReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4281c to 0x428ec.
  Established. Word assignments write `[[sp-32]+1708]` to `[sp-32] +0x6b4` and `[sp-32] +0x6b8` (`str` at 0x42878, `str` at 0x42888).
  A byte assignment writes `(ret(0x52800) & 0xff)` to `[sp-32] +0x6df` (`strb` at 0x428ac).
  The unnamed `0x203f4` call carries `[sp-32]+1760`, `[sp-32]+1704`, `0x38` and `[sp-32]+1704` (`bl` at 0x428d0).
  Slot log reset semantics and physical persistence remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4282c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x42830 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4285c | load | 4 | `abs 0x428e4` | `-` |
| `ldr` at 0x42860 | load | 4 | `abs 0x1a5ab4` | `-` |
| `str` at 0x42868 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4286c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42870 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42874 | load | 4 | `[sp-32] +0x6ac` | `-` |
| `str` at 0x42878 | store | 4 | `[sp-32] +0x6b4` | `[[sp-32]+1708]` |
| `ldr` at 0x4287c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42880 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42884 | load | 4 | `[sp-32] +0x6ac` | `-` |
| `str` at 0x42888 | store | 4 | `[sp-32] +0x6b8` | `[[sp-32]+1708]` |
| `ldr` at 0x4288c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42890 | load | 4 | `abs 0x428e8` | `-` |
| `ldr` at 0x42894 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x428ac | store | 1 | `[sp-32] +0x6df` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x428b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x428b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x428d4 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x428e0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x428e0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x428e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x428e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x428e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x428a8 | `(unknown callee) 0x52800` | `[sp-32]+1704` | `0x38` | `[sp-32]` | `[sp-32]+1704` |
| `bl` at 0x428d0 | `(unknown callee) 0x203f4` | `[sp-32]+1760` | `[sp-32]+1704` | `0x38` | `[sp-32]+1704` |
| `bl` at 0x428d8 | `(unknown callee) 0x411f0` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x428e0 | `r0` | `ret(0x411f0)` |

## `nvramSlotLogInsert`

- `nvramSlotLogInsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x428ec to 0x42e1c.
  Established. A byte assignment writes `0xa` to `([[sp-36]+1720] + [sp-36]) +0x718` (`strb` at 0x42c94).
  A word assignment writes `[[sp-36]+1720]+1` to `[sp-36] +0x6b8` (`str` at 0x42c9c).
  The unnamed `0x203f4` call carries `[sp-36]+1760`, `[sp-36]+1704`, `0x38` and `[sp-36]+1704` (`bl` at 0x42d0c).
  Slot log insertion and physical persistence remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x428fc | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x42900 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x42910 | store | 4 | `sp -0x2c` | `ret(0x202c8)` |
| `str` at 0x42918 | store | 4 | `sp -0x30` | `sp-44` |
| `str` at 0x42920 | store | 4 | `sp -0x34` | `0x5` |
| `ldr` at 0x42924 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42930 | store | 4 | `sp -0x38` | `ret(0x2031c)` |
| `strb` at 0x42938 | store | 1 | `sp -0x39` | `0x0` |
| `ldr` at 0x4293c | load | 4 | `abs 0x42e00` | `-` |
| `ldr` at 0x42940 | load | 4 | `abs 0x1a5ab4` | `-` |
| `ldr` at 0x42950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4297c | load | 4 | `abs 0x42e00` | `-` |
| `ldr` at 0x42980 | load | 4 | `abs 0x1a5ab4` | `-` |
| `str` at 0x42988 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4298c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42990 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42994 | load | 4 | `[sp-36] +0x6b0` | `-` |
| `ldr` at 0x42998 | load | 4 | `[sp-36] +0x6ac` | `-` |
| `str` at 0x429a0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x429a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x429ac | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0x429b8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x429c8 | store | 1 | `sp -0x39` | `0x1` |
| `ldr` at 0x429cc | load | 4 | `abs 0x42e04` | `-` |
| `ldr` at 0x429e4 | load | 4 | `abs 0x42e08` | `-` |
| `ldr` at 0x429e8 | load | 4 | `abs 0x1a2d00` | `-` |
| `ldr` at 0x429f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x429fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a00 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42a04 | load | 4 | `[sp-36] +0x6b4` | `-` |
| `ldr` at 0x42a10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a18 | load | 4 | `[sp-36] +0x6b4` | `-` |
| `ldr` at 0x42a1c | load | 4 | `[sp-36] +0x6ac` | `-` |
| `ldr` at 0x42a24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a2c | load | 4 | `[sp-36] +0x6b0` | `-` |
| `ldr` at 0x42a30 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `str` at 0x42a3c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x42a44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a4c | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42a50 | load | 4 | `[sp-36] +0x6b4` | `-` |
| `ldr` at 0x42a5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42a64 | load | 4 | `[sp-36] +0x6b4` | `-` |
| `ldr` at 0x42a68 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `str` at 0x42a70 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x42a74 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x42a78 | load | 4 | `sp -0x34` | `-` |
| `ldrsb` at 0x42a80 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x42a88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x42a94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42aa0 | load | 4 | `[sp-36] +0x6b4` | `-` |
| `ldr` at 0x42aa4 | load | 4 | `abs 0x42e0c` | `-` |
| `ldr` at 0x42aac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42abc | store | 4 | `[sp-36] +0x6b4` | `[[sp-36]+1716]+1` |
| `ldrb` at 0x42ac0 | load | 1 | `([[sp-36]+1716] + [sp-36]) +0x718` | `-` |
| `ldr` at 0x42acc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x42ad4 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0x42ad8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42adc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42ae0 | load | 4 | `[sp-36] +0x6b4` | `-` |
| `ldr` at 0x42ae4 | load | 4 | `[sp-36] +0x6b0` | `-` |
| `ldr` at 0x42af0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42af4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42af8 | load | 4 | `[sp-36] +0x6ac` | `-` |
| `str` at 0x42afc | store | 4 | `[sp-36] +0x6b4` | `[[sp-36]+1708]` |
| `ldr` at 0x42b04 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x42b0c | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0x42b14 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x42b1c | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0x42b28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42b34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42b40 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42b44 | load | 4 | `abs 0x42e0c` | `-` |
| `ldr` at 0x42b4c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x42b5c | store | 1 | `([[sp-36]+1720] + [sp-36]) +0x718` | `?` |
| `str` at 0x42b64 | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1720]+1` |
| `ldr` at 0x42b6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42b78 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42b7c | load | 4 | `abs 0x42e0c` | `-` |
| `ldr` at 0x42b84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42b94 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x42b98 | load | 1 | `[sp-48] +0x0` | `-` |
| `str` at 0x42ba0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `strb` at 0x42ba4 | store | 1 | `([[sp-36]+1720] + [sp-36]) +0x718` | `([[sp-48]]:1 & 0xff)` |
| `str` at 0x42bac | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1720]+1` |
| `ldr` at 0x42bb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42bb8 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42bbc | load | 4 | `[sp-36] +0x6b0` | `-` |
| `ldr` at 0x42bc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42bcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42bd0 | load | 4 | `[sp-36] +0x6ac` | `-` |
| `str` at 0x42bd4 | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1708]` |
| `ldr` at 0x42bdc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x42be4 | store | 4 | `sp -0x38` | `[sp-56]-1` |
| `ldr` at 0x42bf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42bfc | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42c00 | load | 4 | `abs 0x42e0c` | `-` |
| `ldr` at 0x42c08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42c18 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42c1c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x42c24 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `strb` at 0x42c28 | store | 1 | `([[sp-36]+1720] + [sp-36]) +0x718` | `([[sp-32]]:1 & 0xff)` |
| `str` at 0x42c30 | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1720]+1` |
| `ldr` at 0x42c34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42c38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42c3c | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42c40 | load | 4 | `[sp-36] +0x6b0` | `-` |
| `ldr` at 0x42c4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42c50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42c54 | load | 4 | `[sp-36] +0x6ac` | `-` |
| `str` at 0x42c58 | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1708]` |
| `ldrsb` at 0x42c60 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x42c6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42c78 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42c7c | load | 4 | `abs 0x42e0c` | `-` |
| `ldr` at 0x42c84 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x42c94 | store | 1 | `([[sp-36]+1720] + [sp-36]) +0x718` | `0xa` |
| `str` at 0x42c9c | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1720]+1` |
| `ldr` at 0x42ca0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42ca4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42ca8 | load | 4 | `[sp-36] +0x6b8` | `-` |
| `ldr` at 0x42cac | load | 4 | `[sp-36] +0x6b0` | `-` |
| `ldr` at 0x42cb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42cbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42cc0 | load | 4 | `[sp-36] +0x6ac` | `-` |
| `str` at 0x42cc4 | store | 4 | `[sp-36] +0x6b8` | `[[sp-36]+1708]` |
| `ldr` at 0x42cc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42ccc | load | 4 | `abs 0x42e10` | `-` |
| `ldr` at 0x42cd0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x42ce8 | store | 1 | `[sp-36] +0x6df` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x42cec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42cf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42d10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42d1c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x42d20 | load | 4 | `abs 0x42e04` | `-` |
| `ldr` at 0x42d34 | load | 4 | `abs 0x42e08` | `-` |
| `ldr` at 0x42d38 | load | 4 | `abs 0x1a2d00` | `-` |
| `str` at 0x42d48 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0x42d4c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x42d50 | store | 4 | `sp -0x28` | `[sp-80]` |
| `ldr` at 0x42d5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42d70 | store | 4 | `sp -0x44` | `ret(0x6e1b0)` |
| `ldr` at 0x42d74 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x42d84 | store | 4 | `sp -0x48` | `ret(0x2031c)+1` |
| `ldr` at 0x42d88 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x42db0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x42db4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x42db8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x42dbc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x42dc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42dd4 | load | 4 | `abs 0x42e14` | `-` |
| `str` at 0x42dd8 | store | 4 | `(unknown)` | `0x15b060` |
| `str` at 0x42ddc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x42de0 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x42de8 | load | 4 | `abs 0x42e18` | `-` |
| `ldm` at 0x42dfc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x42dfc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x42dfc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42dfc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42dfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42908 | `(unknown callee) 0x202c8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x42928 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `0x5` |
| `bl` at 0x429d4 | `(unknown callee) 0x22ca0` | `0x1a5abc` | `?` | `[sp-32]` | `?` |
| `bl` at 0x42ce4 | `(unknown callee) 0x52800` | `[sp-36]+1704` | `0x38` | `?` | `[sp-36]+1704` |
| `bl` at 0x42d0c | `(unknown callee) 0x203f4` | `[sp-36]+1760` | `[sp-36]+1704` | `0x38` | `[sp-36]+1704` |
| `bl` at 0x42d14 | `(unknown callee) 0x411f0` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x42d24 | `(unknown callee) 0x2302c` | `0x1a5abc` | `?` | `?` | `0x0` |
| `bl` at 0x42d6c | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x42d78 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x42dc0 | `(unknown callee) 0x203f4` | `[sp-76]` | `[sp-68]` | `[sp-72]` | `?` |
| `bl` at 0x42dcc | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x42df4 | `(unknown callee) 0x22070` | `0x9` | `0x15adf0` | `0x2cc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42948 | `[0x1a5ab4] != 0x0` | `[0x1a5ab4] == 0x0` |
| `beq` at 0x429c0 | `? == 0xa` | `? != 0xa` |
| `beq` at 0x429e0 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0x429f0 | `[0x1a2d00] == 0x1` | `[0x1a2d00] != 0x1` |
| `bls` at 0x42a0c | `[[sp-36]+1720] <= [[sp-36]+1716] (unsigned, 32-bit)` | `[[sp-36]+1720] > [[sp-36]+1716] (unsigned, 32-bit)` |
| `bhs` at 0x42a58 | `[[sp-36]+1720] >= [[sp-36]+1716] (unsigned, 32-bit)` | `[[sp-36]+1720] < [[sp-36]+1716] (unsigned, 32-bit)` |
| `blt` at 0x42a90 | `? < [sp-64] (signed, 32-bit)` | `? >= [sp-64] (signed, 32-bit)` |
| `beq` at 0x42ac8 | `[([[sp-36]+1716] + [sp-36])+1816]:1 == 0xa` | `[([[sp-36]+1716] + [sp-36])+1816]:1 != 0xa` |
| `blo` at 0x42aec | `[[sp-36]+1716] < [[sp-36]+1712] (unsigned, 32-bit)` | `[[sp-36]+1716] >= [[sp-36]+1712] (unsigned, 32-bit)` |
| `beq` at 0x42b24 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x42b30 | `[sp-52] != 0x4` | `[sp-52] == 0x4` |
| `blo` at 0x42bc4 | `[[sp-36]+1720] < [[sp-36]+1712] (unsigned, 32-bit)` | `[[sp-36]+1720] >= [[sp-36]+1712] (unsigned, 32-bit)` |
| `beq` at 0x42bec | `Z == 1` | `not (Z == 1)` |
| `blo` at 0x42c48 | `[[sp-36]+1720] < [[sp-36]+1712] (unsigned, 32-bit)` | `[[sp-36]+1720] >= [[sp-36]+1712] (unsigned, 32-bit)` |
| `beq` at 0x42c68 | `? == 0x0` | `? != 0x0` |
| `blo` at 0x42cb4 | `[[sp-36]+1720] < [[sp-36]+1712] (unsigned, 32-bit)` | `[[sp-36]+1720] >= [[sp-36]+1712] (unsigned, 32-bit)` |
| `beq` at 0x42d30 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |
| `beq` at 0x42d40 | `[0x1a2d00] == 0x1` | `[0x1a2d00] != 0x1` |
| `beq` at 0x42d58 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42dfc | `r0` | `?` |

## `nvramSlotLogFinsert`

- `nvramSlotLogFinsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42e1c to 0x42e70.
  Established. The unnamed `0x20034` call carries `sp-156`, entry `r1`, `sp-8` and `sp-156` (`bl` at 0x42e4c).
  The unnamed `0x428ec` call carries `[sp-32]`, `sp-156`, unknown and `sp-156` (`bl` at 0x42e5c).
  The integer return candidate is `0x0` (`ldm` at 0x42e6c).
  Formatted slot log behavior and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42e30 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x42e38 | store | 4 | `sp -0x24` | `sp-8` |
| `ldr` at 0x42e44 | load | 4 | `sp -0xc` | `-` |
| `ldr` at 0x42e48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42e54 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x42e6c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x42e6c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x42e6c | load | 4 | `sp -0x14` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42e4c | `(unknown callee) 0x20034` | `sp-156` | `r1` | `sp-8` | `sp-156` |
| `bl` at 0x42e5c | `(unknown callee) 0x428ec` | `[sp-32]` | `sp-156` | `?` | `sp-156` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42e6c | `r0` | `0x0` |

## `nvramSlotLogFetch`

- `nvramSlotLogFetch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42e70 to 0x432c4.
  Established. A byte assignment clears `ret(0x201fc) +0x0` (`strb` at 0x42fe8).
  Word reads address `[sp-24] +0x6b8` and `[sp-24] +0x6b4` (`ldr` at 0x42ed0, `ldr` at 0x42ed4).
  The unnamed `0x20214` call carries `([sp-28] + [sp-76])`, `0x13`, `0x15b08c` and `ret(0x20094)` (`bl` at 0x431d0).
  Fetched slot log representation and physical NVRAM ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42e80 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x42e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42eb0 | load | 4 | `abs 0x432bc` | `-` |
| `ldr` at 0x42eb4 | load | 4 | `abs 0x1a5ab4` | `-` |
| `str` at 0x42ebc | store | 4 | `sp -0x18` | `?` |
| `str` at 0x42ec4 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x42ec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42ed0 | load | 4 | `[sp-24] +0x6b8` | `-` |
| `ldr` at 0x42ed4 | load | 4 | `[sp-24] +0x6b4` | `-` |
| `ldr` at 0x42ee0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42ee4 | load | 4 | `[sp-24] +0x6b4` | `-` |
| `ldr` at 0x42ee8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42ef8 | store | 4 | `sp -0x20` | `([[sp-24]+1716] + [sp-24])+1816` |
| `ldr` at 0x42efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f00 | load | 4 | `[sp-24] +0x6b8` | `-` |
| `ldr` at 0x42f04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42f14 | store | 4 | `sp -0x24` | `([[sp-24]+1720] + [sp-24])+1816` |
| `str` at 0x42f1c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x42f24 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x42f2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f34 | load | 4 | `[sp-24] +0x6b8` | `-` |
| `ldr` at 0x42f38 | load | 4 | `[sp-24] +0x6b4` | `-` |
| `ldr` at 0x42f44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f48 | load | 4 | `[sp-24] +0x6b4` | `-` |
| `ldr` at 0x42f4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42f5c | store | 4 | `sp -0x20` | `([[sp-24]+1716] + [sp-24])+1816` |
| `ldr` at 0x42f60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f64 | load | 4 | `[sp-24] +0x6b0` | `-` |
| `ldr` at 0x42f68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42f78 | store | 4 | `sp -0x24` | `([[sp-24]+1712] + [sp-24])+1816` |
| `ldr` at 0x42f7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f80 | load | 4 | `[sp-24] +0x6ac` | `-` |
| `ldr` at 0x42f84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42f94 | store | 4 | `sp -0x28` | `([[sp-24]+1708] + [sp-24])+1816` |
| `ldr` at 0x42f98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f9c | load | 4 | `[sp-24] +0x6b8` | `-` |
| `ldr` at 0x42fa0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42fb0 | store | 4 | `sp -0x2c` | `([[sp-24]+1720] + [sp-24])+1816` |
| `str` at 0x42fc4 | store | 4 | `sp -0x1c` | `ret(0x201fc)` |
| `ldr` at 0x42fc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42fd8 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x42fe0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x42fe8 | store | 1 | `ret(0x201fc) +0x0` | `0x0` |
| `ldr` at 0x42fec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42ff0 | store | 4 | `sp -0x50` | `[sp-28]` |
| `str` at 0x42ffc | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x43000 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x43004 | store | 4 | `sp -0x34` | `([[sp-24]+1716] + [sp-24])+1816` |
| `ldr` at 0x43008 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4300c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x43010 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4301c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x43020 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x43024 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x43028 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x43034 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x43040 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x43048 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x43050 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x43054 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x43060 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x43068 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x43070 | store | 4 | `sp -0x3c` | `0x4` |
| `ldr` at 0x43078 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x43080 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x43084 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x4308c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x43094 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x43098 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x430a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x430ac | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x430b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x430b4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x430b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x430bc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x430c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x430c4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x430cc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x430d8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x430e0 | store | 4 | `sp -0x54` | `?` |
| `str` at 0x430ec | store | 4 | `sp -0x54` | `0x64` |
| `ldr` at 0x430f0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x430f4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x430f8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x43104 | store | 4 | `sp -0x1c` | `ret(0x201fc)` |
| `ldr` at 0x43108 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x43118 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x43124 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x4312c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x43130 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x43134 | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0x43138 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4313c | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x43140 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4314c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x43150 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x43154 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x43158 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x43164 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x43174 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x43178 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x43180 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x43184 | store | 1 | `[sp-68] +0x0` | `?` |
| `str` at 0x4318c | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x43190 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x43198 | store | 4 | `sp -0x3c` | `[sp-60]-1` |
| `ldr` at 0x431b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x431b8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x431c8 | load | 4 | `abs 0x432c0` | `-` |
| `ldr` at 0x431d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x431d8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x431f0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x431f8 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x2031c))` |
| `ldr` at 0x43200 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x43204 | load | 1 | `(unknown)` | `-` |
| `str` at 0x43214 | store | 4 | `sp -0x44` | `sp-64` |
| `str` at 0x4321c | store | 4 | `sp -0x3c` | `0x4` |
| `ldr` at 0x43228 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x43230 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x43238 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x4323c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x43240 | store | 1 | `(unknown)` | `?` |
| `str` at 0x43248 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x4324c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x43254 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4325c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x43260 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x43270 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x43274 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0x43278 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4327c | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x43280 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x43284 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0x43288 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4328c | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x43294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x43298 | load | 4 | `sp -0x4c` | `-` |
| `strb` at 0x432a4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x432a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x432ac | store | 4 | `sp -0x50` | `[sp-28]` |
| `ldr` at 0x432b0 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x432b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x432b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x432b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42fbc | `(unknown callee) 0x201fc` | `0x1` | `[[sp-24]+1720]` | `[sp-24]` | `[[sp-24]+1716]` |
| `bl` at 0x430fc | `(unknown callee) 0x201fc` | `?` | `[[sp-24]+1720]` | `?` | `?` |
| `bl` at 0x431ac | `(unknown callee) 0x20094` | `sp-64` | `[sp-68]` | `[sp-68]+1` | `sp-64` |
| `bl` at 0x431d0 | `(unknown callee) 0x20214` | `([sp-28] + [sp-76])` | `0x13` | `0x15b08c` | `ret(0x20094)` |
| `bl` at 0x431e4 | `(unknown callee) 0x2031c` | `([sp-28] + [sp-76])` | `?` | `[sp-28]` | `([sp-28] + [sp-76])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x42edc | `[[sp-24]+1720] <= [[sp-24]+1716] (unsigned, 32-bit)` | `[[sp-24]+1720] > [[sp-24]+1716] (unsigned, 32-bit)` |
| `bhs` at 0x42f40 | `[[sp-24]+1720] >= [[sp-24]+1716] (unsigned, 32-bit)` | `[[sp-24]+1720] < [[sp-24]+1716] (unsigned, 32-bit)` |
| `bne` at 0x42fd0 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0x43018 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x43030 | `C == 1` | `not (C == 1)` |
| `beq` at 0x4303c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x4305c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x430a0 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x430d4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x43110 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0x43148 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x43160 | `C == 1` | `not (C == 1)` |
| `beq` at 0x4316c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x431a0 | `[sp-60]-1 != 0x0` | `[sp-60]-1 == 0x0` |
| `bne` at 0x4320c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x43268 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x432b8 | `r0` | `?` |
