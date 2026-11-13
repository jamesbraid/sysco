# ARM NVRAM, reset and rack-slot paths in 1.38.4

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
NVRAM initialization, set/get and serial-record selectors have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/nvram-reset-paths-facts.txt) retain all tool sections.

## `nvramInit`

- `nvramInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1bc64 to 0x1cb1c.
  Established. A word store writes `0x2ae618` at `0x144de4` (`str` at 0x1bdc0). A later store writes
  word `[0x144dec]+2352` at `0x144df0` (`str` at 0x1be5c). The computed transfer takes an unresolved
  edge when an unknown value is at most `0x9` unsigned and falls through for greater (`ldrls` at
  0x1be88). Known paths include byte stores of `0x3` at `[0x144dec] +0x7c8` and `0x1` at `[0x144dec]
  +0x780` (`strb` at 0x1c624, `strb` at 0x1c6d4). The known-edge r0 return is `ret(0x1b088)`
  (`ldmdb` at 0x1caa8). Excluded initialization cases and physical NVRAM behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1bc78 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1bc7c | load | 4 | `abs 0x1caac` | `-` |
| `ldr` at 0x1bc80 | load | 4 | `abs 0x144e50` | `-` |
| `ldr` at 0x1bc8c | load | 4 | `abs 0x1cab0` | `-` |
| `str` at 0x1bc90 | store | 4 | `sp -0x68` | `0xfbdac` |
| `ldr` at 0x1bc98 | load | 4 | `abs 0x1cab4` | `-` |
| `ldr` at 0x1bca8 | load | 4 | `abs 0x1caac` | `-` |
| `ldr` at 0x1bcac | load | 4 | `abs 0x1caac` | `-` |
| `ldr` at 0x1bcb0 | load | 4 | `abs 0x144e50` | `-` |
| `str` at 0x1bcb8 | store | 4 | `abs 0x144e50` | `[0x144e50]+1` |
| `ldr` at 0x1bcbc | load | 4 | `abs 0x1cab8` | `-` |
| `ldr` at 0x1bccc | load | 4 | `abs 0x1cabc` | `-` |
| `str` at 0x1bcd0 | store | 4 | `sp -0x24` | `0xfbdcc` |
| `ldr` at 0x1bcd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1bcd8 | load | 4 | `abs 0x1cac0` | `-` |
| `ldr` at 0x1bcdc | load | 4 | `abs 0x1cac4` | `-` |
| `ldr` at 0x1bce8 | load | 4 | `abs 0x1cac8` | `-` |
| `str` at 0x1bcec | store | 4 | `abs 0x144e74` | `ret(0x1b370)` |
| `ldr` at 0x1bcf0 | load | 4 | `abs 0x144e74` | `-` |
| `ldr` at 0x1bd04 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0x1bd10 | load | 4 | `abs 0x1cacc` | `-` |
| `str` at 0x1bd14 | store | 4 | `sp -0x68` | `0xfbddc` |
| `ldr` at 0x1bd18 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1bd1c | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x1bd20 | store | 4 | `sp -0x60` | `ret(0x21674)` |
| `ldr` at 0x1bd28 | load | 4 | `abs 0x1cab4` | `-` |
| `ldr` at 0x1bd38 | load | 4 | `abs 0x1cad0` | `-` |
| `ldr` at 0x1bd48 | load | 4 | `abs 0x1cac8` | `-` |
| `ldr` at 0x1bd4c | load | 4 | `abs 0x144e74` | `-` |
| `ldr` at 0x1bd50 | load | 4 | `abs 0x1cad0` | `-` |
| `ldr` at 0x1bd70 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0x1bd7c | load | 4 | `abs 0x1cad4` | `-` |
| `str` at 0x1bd80 | store | 4 | `sp -0x68` | `0xfbe00` |
| `ldr` at 0x1bd84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1bd88 | store | 4 | `sp -0x64` | `[sp-36]` |
| `str` at 0x1bd8c | store | 4 | `sp -0x60` | `ret(0x21674)` |
| `ldr` at 0x1bd94 | load | 4 | `abs 0x1cab4` | `-` |
| `ldr` at 0x1bda4 | load | 4 | `abs 0x1cad8` | `-` |
| `ldr` at 0x1bda8 | load | 4 | `abs 0x1cadc` | `-` |
| `ldr` at 0x1bdb8 | load | 4 | `abs 0x1cae0` | `-` |
| `ldr` at 0x1bdbc | load | 4 | `abs 0x1cad0` | `-` |
| `str` at 0x1bdc0 | store | 4 | `abs 0x144de4` | `0x2ae618` |
| `ldr` at 0x1bdc4 | load | 4 | `abs 0x1cae0` | `-` |
| `ldr` at 0x1bdc8 | load | 4 | `abs 0x144de4` | `-` |
| `ldr` at 0x1bde8 | load | 4 | `abs 0x1cae0` | `-` |
| `ldr` at 0x1bdec | load | 4 | `abs 0x144de4` | `-` |
| `ldrb` at 0x1bdf0 | load | 1 | `[0x144de4] +0x0` | `-` |
| `ldr` at 0x1be04 | load | 4 | `abs 0x1cae0` | `-` |
| `ldr` at 0x1be08 | load | 4 | `abs 0x144de4` | `-` |
| `ldrb` at 0x1be0c | load | 1 | `[0x144de4] +0x1` | `-` |
| `ldr` at 0x1be1c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1be20 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1be2c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1be30 | load | 4 | `abs 0x1cae0` | `-` |
| `ldr` at 0x1be34 | load | 4 | `abs 0x144de4` | `-` |
| `ldrb` at 0x1be38 | load | 1 | `[0x144de4] +0x1` | `-` |
| `ldr` at 0x1be40 | load | 4 | `abs 0x1cad0` | `-` |
| `str` at 0x1be48 | store | 4 | `abs 0x144dec` | `?` |
| `ldr` at 0x1be4c | load | 4 | `abs 0x1cae8` | `-` |
| `ldr` at 0x1be50 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1be54 | load | 4 | `abs 0x144dec` | `-` |
| `str` at 0x1be5c | store | 4 | `abs 0x144df0` | `[0x144dec]+2352` |
| `ldr` at 0x1be60 | load | 4 | `abs 0x1caec` | `-` |
| `ldr` at 0x1be64 | load | 4 | `abs 0x1caf0` | `-` |
| `str` at 0x1be68 | store | 4 | `abs 0x144df4` | `0x2b0610` |
| `ldr` at 0x1be6c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1be70 | load | 4 | `abs 0x144dec` | `-` |
| `ldrb` at 0x1be74 | load | 1 | `[0x144dec] +0x0` | `-` |
| `ldrls` at 0x1be88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1c528 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c52c | load | 4 | `abs 0x144dec` | `-` |
| `str` at 0x1c54c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1c550 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c554 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c558 | load | 4 | `abs 0x1cb00` | `-` |
| `ldrb` at 0x1c55c | load | 1 | `[0x144dec] +0x7c8` | `-` |
| `ldr` at 0x1c56c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c570 | load | 4 | `abs 0x144dec` | `-` |
| `str` at 0x1c59c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1c5a4 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c5a8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c5ac | load | 4 | `abs 0x1cb04` | `-` |
| `strb` at 0x1c5b4 | store | 1 | `[0x144dec] +0x7cc` | `0x0` |
| `ldr` at 0x1c5b8 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c5bc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c5c0 | load | 4 | `[0x144dec] +0x7cc` | `-` |
| `str` at 0x1c5c4 | store | 4 | `sp -0x5c` | `[[0x144dec]+1996]` |
| `ldr` at 0x1c5c8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x1c5d0 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x1c5d4 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x1c5d8 | store | 4 | `[0x144dec] +0x7cc` | `[sp-92]` |
| `ldr` at 0x1c5dc | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c5e0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c5f8 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c5fc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c614 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c618 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c61c | load | 4 | `abs 0x1cb00` | `-` |
| `strb` at 0x1c624 | store | 1 | `[0x144dec] +0x7c8` | `0x3` |
| `ldr` at 0x1c628 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c62c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c630 | load | 4 | `abs 0x1caf8` | `-` |
| `ldr` at 0x1c634 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c638 | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x1c644 | store | 1 | `[0x144dec] +0x3bf` | `(ret(0x21d34) & 0xff)` |
| `ldr` at 0x1c648 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c64c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c650 | load | 4 | `abs 0x1cb08` | `-` |
| `ldr` at 0x1c654 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c658 | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x1c670 | store | 1 | `[0x144dec] +0x87b` | `(ret(0x21d34) & 0xff)` |
| `ldr` at 0x1c674 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c678 | load | 4 | `abs 0x144dec` | `-` |
| `ldrb` at 0x1c680 | load | 1 | `[0x144dec] +0x780` | `-` |
| `ldr` at 0x1c690 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c694 | load | 4 | `abs 0x144dec` | `-` |
| `str` at 0x1c6bc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x1c6c4 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c6c8 | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x1c6d4 | store | 1 | `[0x144dec] +0x780` | `0x1` |
| `ldr` at 0x1c6d8 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c6dc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c6e0 | load | 4 | `abs 0x1cb0c` | `-` |
| `ldr` at 0x1c6e4 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c6e8 | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x1c6fc | store | 1 | `[0x144dec] +0x7a3` | `(ret(0x21d34) & 0xff)` |
| `ldr` at 0x1c700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1c714 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c718 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c720 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c728 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c734 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c738 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c744 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c748 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c764 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c768 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c774 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c778 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c7d0 | load | 4 | `abs 0x1caec` | `-` |
| `ldr` at 0x1c7d4 | load | 4 | `abs 0x1cae8` | `-` |
| `ldr` at 0x1c7d8 | load | 4 | `abs 0x144df4` | `-` |
| `ldr` at 0x1c7dc | load | 4 | `abs 0x144df0` | `-` |
| `ldr` at 0x1c7e4 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c7e8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c7ec | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c7f0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c7f4 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x1c7f8 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x1c808 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c80c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c810 | load | 4 | `abs 0x1cae8` | `-` |
| `ldr` at 0x1c814 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x1c818 | load | 4 | `abs 0x144df0` | `-` |
| `ldr` at 0x1c824 | load | 4 | `abs 0x1cae8` | `-` |
| `ldr` at 0x1c828 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c82c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c830 | load | 4 | `abs 0x144df0` | `-` |
| `ldr` at 0x1c834 | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x1c83c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x1c840 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c844 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c848 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c84c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c850 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x1c854 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c85c | store | 4 | `[0x144dec] +0x784` | `([[0x144dec]+1924] + [sp-36])` |
| `ldr` at 0x1c860 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c864 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c868 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c86c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c870 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x1c874 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c87c | store | 4 | `[0x144dec] +0x788` | `([[0x144dec]+1928] + [sp-36])` |
| `ldr` at 0x1c880 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c884 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c888 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c88c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c890 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x1c894 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c89c | store | 4 | `[0x144dec] +0x78c` | `([[0x144dec]+1932] + [sp-36])` |
| `ldr` at 0x1c8a0 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c8a4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c8a8 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c8ac | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c8b0 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x1c8b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1c8bc | store | 4 | `[0x144dec] +0x790` | `([[0x144dec]+1936] + [sp-36])` |
| `ldr` at 0x1c8c0 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c8c4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c8c8 | load | 4 | `abs 0x1cb0c` | `-` |
| `ldr` at 0x1c8cc | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c8d0 | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x1c8e4 | store | 1 | `[0x144dec] +0x7a3` | `(ret(0x21d34) & 0xff)` |
| `ldr` at 0x1c8e8 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c8ec | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c8f8 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c8fc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c914 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c918 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c91c | load | 4 | `abs 0x1cae8` | `-` |
| `ldr` at 0x1c920 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x1c924 | load | 4 | `abs 0x144df0` | `-` |
| `ldr` at 0x1c930 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c934 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c938 | load | 4 | `abs 0x1caec` | `-` |
| `ldr` at 0x1c93c | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x1c940 | load | 4 | `abs 0x144df4` | `-` |
| `ldr` at 0x1c94c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c950 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c954 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c958 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c95c | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x1c960 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x1c96c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c970 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c974 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c978 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c97c | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x1c980 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x1c98c | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c990 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c994 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c998 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c99c | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x1c9a0 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x1c9ac | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c9b0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c9b4 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c9b8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c9bc | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x1c9c0 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x1c9d0 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c9d4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c9d8 | load | 4 | `abs 0x1cae8` | `-` |
| `ldr` at 0x1c9dc | load | 4 | `abs 0x144df0` | `-` |
| `str` at 0x1c9e0 | store | 4 | `[0x144dec] +0x784` | `[0x144df0]` |
| `ldr` at 0x1c9e4 | load | 4 | `abs 0x1cae4` | `-` |
| `ldr` at 0x1c9e8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x1c9ec | load | 4 | `abs 0x1caec` | `-` |
| `ldr` at 0x1c9f0 | load | 4 | `abs 0x144df4` | `-` |
| `str` at 0x1c9f4 | store | 4 | `[0x144dec] +0x788` | `[0x144df4]` |
| `ldr` at 0x1c9fc | load | 4 | `abs 0x1cab8` | `-` |
| `ldr` at 0x1ca00 | load | 4 | `abs 0x1cb10` | `-` |
| `ldrb` at 0x1ca04 | load | 1 | `abs 0x144df8` | `-` |
| `strb` at 0x1ca0c | store | 1 | `abs 0x144df8` | `([0x144df8]:1+1 & 0xff)` |
| `ldr` at 0x1ca14 | load | 4 | `abs 0x1cb14` | `-` |
| `str` at 0x1ca18 | store | 4 | `(unknown)` | `0xfbe38` |
| `ldr` at 0x1ca1c | load | 4 | `abs 0x1cb10` | `-` |
| `ldrb` at 0x1ca20 | load | 1 | `abs 0x144df8` | `-` |
| `ldr` at 0x1ca2c | load | 4 | `abs 0x1cb10` | `-` |
| `strb` at 0x1ca34 | store | 1 | `abs 0x144df8` | `0x0` |
| `ldr` at 0x1ca38 | load | 4 | `abs 0x1cab8` | `-` |
| `ldr` at 0x1ca3c | load | 4 | `abs 0x1cb10` | `-` |
| `ldrb` at 0x1ca40 | load | 1 | `abs 0x144df8` | `-` |
| `ldr` at 0x1ca48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1ca58 | load | 4 | `abs 0x1cab8` | `-` |
| `ldr` at 0x1ca5c | load | 4 | `abs 0x1cb10` | `-` |
| `ldrb` at 0x1ca60 | load | 1 | `abs 0x144df8` | `-` |
| `ldr` at 0x1ca68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1ca70 | load | 4 | `abs 0x1cb10` | `-` |
| `ldr` at 0x1ca74 | load | 4 | `abs 0x1cb10` | `-` |
| `ldrb` at 0x1ca78 | load | 1 | `abs 0x144df8` | `-` |
| `strb` at 0x1ca80 | store | 1 | `abs 0x144df8` | `([0x144df8]:1+1 & 0xff)` |
| `ldr` at 0x1ca98 | load | 4 | `abs 0x1cb18` | `-` |
| `ldmdb` at 0x1caa8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1caa8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1caa8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1caa8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1caa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1bca4 | `(unknown callee) 0x21e28` | `0x9` | `0xfbd34` | `0x71` | `0x0` |
| `bl` at 0x1bcc8 | `(unknown callee) 0x1b43c` | `0x144e54` | `0x0` | `0x20` | `[0x144e50]+1` |
| `bl` at 0x1bce0 | `(unknown callee) 0x1b370` | `0xfbdcc` | `0x1042` | `0x1b6` | `0xfbdcc` |
| `bl` at 0x1bcfc | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `ret(0x1b370)` | `[0x144e74]` |
| `bl` at 0x1bd08 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0x1bd34 | `(unknown callee) 0x21e28` | `0x9` | `0xfbd34` | `0x82` | `0x0` |
| `bl` at 0x1bd44 | `(unknown callee) 0x1b43c` | `0x2ae618` | `0x0` | `0x2000` | `?` |
| `bl` at 0x1bd58 | `(unknown callee) 0x1b4fc` | `[0x144e74]` | `0x2ae618` | `0x2000` | `0x144e74` |
| `bl` at 0x1bd68 | `(unknown callee) 0x1b0b8` | `ret(0x1b4fc)` | `?` | `?` | `ret(0x1b4fc)` |
| `bl` at 0x1bd74 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0x1bda0 | `(unknown callee) 0x21e28` | `0x9` | `0xfbd34` | `0x88` | `0x0` |
| `bl` at 0x1bdb4 | `(unknown callee) 0x22920` | `0x144dfc` | `0xfbe2c` | `0x1` | `0x9` |
| `bl` at 0x1bdd0 | `(unknown callee) 0x21cc0` | `[0x144de4]` | `0x8` | `0x144de4` | `0x144de4` |
| `bl` at 0x1bde4 | `(unknown callee) 0x1d288` | `ret(0x21cc0)` | `?` | `?` | `?` |
| `bl` at 0x1be18 | `(unknown callee) 0x1d288` | `?` | `?` | `?` | `?` |
| `bl` at 0x1c534 | `(unknown callee) 0x21cc0` | `[0x144dec]` | `0x3c0` | `?` | `0x144dec` |
| `bl` at 0x1c584 | `(unknown callee) 0x21cc0` | `[0x144dec]+1992` | `0xb4` | `[0x144dec]` | `[0x144dec]+1992` |
| `bl` at 0x1c5f4 | `(unknown callee) 0x1b43c` | `[0x144dec]+2000` | `0x0` | `0xa8` | `[0x144dec]+2000` |
| `bl` at 0x1c610 | `(unknown callee) 0x1b43c` | `[0x144dec]+952` | `0x0` | `0x7` | `[0x144dec]+952` |
| `bl` at 0x1c640 | `(unknown callee) 0x21d34` | `[0x144dec]` | `0x3c0` | `0x7c8` | `0x144dec` |
| `bl` at 0x1c66c | `(unknown callee) 0x21d34` | `[0x144dec]+1992` | `0xb4` | `?` | `[0x144dec]+1992` |
| `bl` at 0x1c6a4 | `(unknown callee) 0x21cc0` | `[0x144dec]+1920` | `0x24` | `[0x144dec]` | `[0x144dec]+1920` |
| `bl` at 0x1c6f8 | `(unknown callee) 0x21d34` | `[0x144dec]+1920` | `0x24` | `0x780` | `[0x144dec]+1920` |
| `bl` at 0x1c70c | `(unknown callee) 0x1cb1c` | `?` | `?` | `?` | `[sp-32]` |
| `bl` at 0x1c730 | `(unknown callee) 0x1b304` | `[0x144dec]+960` | `[0x144dec]` | `0x3c0` | `[0x144dec]+960` |
| `bl` at 0x1c760 | `(unknown callee) 0x1b304` | `[0x144dec]+2172` | `[0x144dec]+1992` | `0xb4` | `[0x144dec]+1992` |
| `bl` at 0x1c78c | `(unknown callee) 0x1b304` | `[0x144dec]+1956` | `[0x144dec]+1920` | `0x24` | `[0x144dec]+1920` |
| `bl` at 0x1c794 | `(unknown callee) 0x1f1dc` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x1c7ac | `(unknown callee) 0x1d38c` | `0x11` | `0x2` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x1c7b4 | `(unknown callee) 0x1f1dc` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x1c7cc | `(unknown callee) 0x1d38c` | `0x3d` | `0x2` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x1c8e0 | `(unknown callee) 0x21d34` | `[0x144dec]+1920` | `0x24` | `[[0x144dec]+1936]` | `[0x144dec]+1920` |
| `bl` at 0x1c910 | `(unknown callee) 0x1b304` | `[0x144dec]+1956` | `[0x144dec]+1920` | `0x24` | `[0x144dec]+1920` |
| `bl` at 0x1c9f8 | `(unknown callee) 0x201a0` | `?` | `?` | `[0x144dec]` | `[0x144df4]` |
| `bl` at 0x1ca6c | `(unknown callee) 0x20240` | `?` | `?` | `0x144e54` | `?` |
| `bl` at 0x1ca88 | `(unknown callee) 0x1bb4c` | `?` | `?` | `?` | `?` |
| `bl` at 0x1ca90 | `(unknown callee) 0x1f1dc` | `0x46` | `?` | `?` | `?` |
| `bl` at 0x1caa4 | `(unknown callee) 0x1b088` | `0xfbe5c` | `ret(0x1f1dc)` | `0x1` | `ret(0x1f1dc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1bc88 | `[0x144e50] == 0x0` | `[0x144e50] != 0x0` |
| `bge` at 0x1bcf8 | `[0x144e74] >= 0x0 (signed, 32-bit)` | `[0x144e74] < 0x0 (signed, 32-bit)` |
| `bge` at 0x1bd64 | `ret(0x1b4fc) >= 0x0 (signed, 32-bit)` | `ret(0x1b4fc) < 0x0 (signed, 32-bit)` |
| `beq` at 0x1bde0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1be00 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x1be14 | `[[0x144de4]+1]:1 != 0x0` | `[[0x144de4]+1]:1 == 0x0` |
| `bne` at 0x1be28 | `[0x144dec] != 0x0` | `[0x144dec] == 0x0` |
| `ldrls` at 0x1be88 | `? <= 0x9 (unsigned, 32-bit)` | `? > 0x9 (unsigned, 32-bit)` |
| `beq` at 0x1c544 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1c564 | `[[0x144dec]+1992]:1 == 0x3` | `[[0x144dec]+1992]:1 != 0x3` |
| `beq` at 0x1c594 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1c688 | `[[0x144dec]+1920]:1 == 0x1` | `[[0x144dec]+1920]:1 != 0x1` |
| `beq` at 0x1c6b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1c708 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bhi` at 0x1c7a0 | `ret(0x1f1dc) > 0x1 (unsigned, 32-bit)` | `ret(0x1f1dc) <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x1c7c0 | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `bne` at 0x1c804 | `? != ?` | `? == ?` |
| `beq` at 0x1c820 | `[[0x144dec]+1924] == [0x144df0]` | `[[0x144dec]+1924] != [0x144df0]` |
| `bne` at 0x1c92c | `[[0x144dec]+1924] != [0x144df0]` | `[[0x144dec]+1924] == [0x144df0]` |
| `bne` at 0x1c948 | `[[0x144dec]+1928] != [0x144df4]` | `[[0x144dec]+1928] == [0x144df4]` |
| `blo` at 0x1c968 | `[[0x144dec]+1932] < [[0x144dec]+1924] (unsigned, 32-bit)` | `[[0x144dec]+1932] >= [[0x144dec]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1c988 | `[[0x144dec]+1932] >= [[0x144dec]+1928] (unsigned, 32-bit)` | `[[0x144dec]+1932] < [[0x144dec]+1928] (unsigned, 32-bit)` |
| `blo` at 0x1c9a8 | `[[0x144dec]+1936] < [[0x144dec]+1924] (unsigned, 32-bit)` | `[[0x144dec]+1936] >= [[0x144dec]+1924] (unsigned, 32-bit)` |
| `bhs` at 0x1c9c8 | `[[0x144dec]+1936] >= [[0x144dec]+1928] (unsigned, 32-bit)` | `[[0x144dec]+1936] < [[0x144dec]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1ca28 | `[0x144df8]:1 == 0x0` | `[0x144df8]:1 != 0x0` |
| `bne` at 0x1ca50 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1caa8 | `r0` | `ret(0x1b088)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1be88).

## `nvramSet`

- `nvramSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1d38c to 0x1f1dc.
  Established. The selector takes an unresolved edge for `r0-1 <= 0xaf` unsigned and falls through
  for greater (`ldrls` at 0x1d3b0). The known fall-through reaches a return retaining entry r0
  (`ldmdb` at 0x1f1d8). Before the selector, word stores save entry r0 at `sp -0x1c` and entry r1 at
  `sp -0x20` (`str` at 0x1d39c, `str` at 0x1d3a0). No calls are reported on known edges. Excluded
  value-specific writes and persistence effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1d39c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1d3a0 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x1d3a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x1d3b0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x1f1d8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1f1d8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1f1d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1f1d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1f1d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1d3b0 | `r0-1 <= 0xaf (unsigned, 32-bit)` | `r0-1 > 0xaf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1f1d8 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1d3b0).

## `nvramGet`

- `nvramGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1f1dc to 0x201a0.
  Established. The computed transfer takes an unresolved edge for `r0-1 <= 0xaf` unsigned and falls
  through for greater (`ldrls` at 0x1f1fc); its default path stores word `0x0` at `sp -0x18`, then
  returns r0 `0x0` (`str` at 0x20180, `ldmdb` at 0x20188). The earlier word store saves entry r0 at
  `sp -0x14` (`str` at 0x1f1ec). Excluded selector destinations do not establish their returned
  values or NVRAM field meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1f1ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1f1f0 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x1f1fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x20180 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x20184 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x20188 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20188 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20188 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1f1fc | `r0-1 <= 0xaf (unsigned, 32-bit)` | `r0-1 > 0xaf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20188 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1f1fc).

## `nvramLogReset`

- `nvramLogReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x201a0 to 0x20240.
  Established. Two word stores write `[[0x144dec]+1924]` at `[0x144dec] +0x78c` and `[0x144dec]
  +0x790` (`str` at 0x201c0, `str` at 0x201d8). A byte store writes `(ret(0x21d34) & 0xff)` at
  `[0x144dec] +0x7a3` (`strb` at 0x20200). The call to `0x1b304` has `[0x144dec]+1956`,
  `[0x144dec]+1920`, `0x24`, `[0x144dec]+1920` (`bl` at 0x2022c). The final r0 value is
  `ret(0x1bb4c)` (`ldmdb` at 0x20234). Physical log reset and record format remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x201ac | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x201b0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x201b4 | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x201b8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x201bc | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x201c0 | store | 4 | `[0x144dec] +0x78c` | `[[0x144dec]+1924]` |
| `ldr` at 0x201c4 | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x201c8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x201cc | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x201d0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x201d4 | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x201d8 | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1924]` |
| `ldr` at 0x201dc | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x201e0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x201e4 | load | 4 | `abs 0x2023c` | `-` |
| `ldr` at 0x201e8 | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x201ec | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x20200 | store | 1 | `[0x144dec] +0x7a3` | `(ret(0x21d34) & 0xff)` |
| `ldr` at 0x20204 | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x20208 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20214 | load | 4 | `abs 0x20238` | `-` |
| `ldr` at 0x20218 | load | 4 | `abs 0x144dec` | `-` |
| `ldmdb` at 0x20234 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x20234 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x20234 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20234 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20234 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x201fc | `(unknown callee) 0x21d34` | `[0x144dec]+1920` | `0x24` | `[0x144dec]` | `[0x144dec]+1920` |
| `bl` at 0x2022c | `(unknown callee) 0x1b304` | `[0x144dec]+1956` | `[0x144dec]+1920` | `0x24` | `[0x144dec]+1920` |
| `bl` at 0x20230 | `(unknown callee) 0x1bb4c` | `ret(0x1b304)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20234 | `r0` | `ret(0x1bb4c)` |

## `nvramLogInsert`

- `nvramLogInsert`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20240 to 0x2070c.
  Established. The absolute-word guard takes the work arm for `[0x144dec] != 0x0` and falls through
  on equality (`bne` at 0x20298). A later byte store writes `0xa` at `[[0x144dec]+1936] +0x0`
  (`strb` at 0x205e0). Another writes `(ret(0x21d34) & 0xff)` at `[0x144dec] +0x7a3` (`strb` at
  0x20648). The call to `0x1b304` uses `[0x144dec]+1956`, `[0x144dec]+1920`, `0x24`,
  `[0x144dec]+1920` (`bl` at 0x20674). Log encoding, external persistence and the joined r0 return
  remain unknown (`ldmdb` at 0x206ec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20250 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x20260 | store | 4 | `sp -0x24` | `ret(0x1b1e4)` |
| `str` at 0x20268 | store | 4 | `sp -0x28` | `sp-36` |
| `str` at 0x20270 | store | 4 | `sp -0x2c` | `0x5` |
| `ldr` at 0x20274 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x20280 | store | 4 | `sp -0x30` | `ret(0x1b22c)` |
| `strb` at 0x20288 | store | 1 | `sp -0x31` | `0x0` |
| `ldr` at 0x2028c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20290 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x202a0 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x202a4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x202a8 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x202ac | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x202b0 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x202b4 | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x202bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x202c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x202c8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x202d4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x202e4 | store | 1 | `sp -0x31` | `0x1` |
| `ldr` at 0x202e8 | load | 4 | `abs 0x206f4` | `-` |
| `ldr` at 0x20300 | load | 4 | `abs 0x206f8` | `-` |
| `ldr` at 0x20304 | load | 4 | `abs 0x1452d8` | `-` |
| `ldr` at 0x20314 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20318 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2031c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20320 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20324 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x20328 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x20334 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20338 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2033c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20340 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20344 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x20348 | load | 4 | `[0x144dec] +0x784` | `-` |
| `ldr` at 0x20350 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20354 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20358 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x2035c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20360 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x20364 | load | 4 | `[0x144dec] +0x790` | `-` |
| `str` at 0x20370 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x20378 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x2037c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20380 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20384 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20388 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x2038c | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x20398 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x2039c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x203a0 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x203a4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x203a8 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x203ac | load | 4 | `[0x144dec] +0x790` | `-` |
| `str` at 0x203b4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x203bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x203c0 | load | 4 | `sp -0x2c` | `-` |
| `ldrsb` at 0x203c8 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x203d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x203e0 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x203e4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x203f0 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldrb` at 0x203f4 | load | 1 | `[[0x144dec]+1932] +0x0` | `-` |
| `str` at 0x203fc | store | 4 | `[0x144dec] +0x78c` | `[[0x144dec]+1932]+1` |
| `ldr` at 0x20410 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x20418 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x2041c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20420 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20424 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20428 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2042c | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x20430 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x2043c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20440 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20444 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20448 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2044c | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x20450 | store | 4 | `[0x144dec] +0x78c` | `[[0x144dec]+1924]` |
| `ldr` at 0x20458 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x20460 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x2046c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x20474 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0x20484 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20490 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20494 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2049c | load | 4 | `[0x144dec] +0x790` | `-` |
| `strb` at 0x204a8 | store | 1 | `[[0x144dec]+1936] +0x0` | `?` |
| `str` at 0x204b0 | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1936]+1` |
| `ldr` at 0x204b8 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x204bc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x204c4 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x204cc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x204d8 | load | 1 | `[sp-40] +0x0` | `-` |
| `strb` at 0x204dc | store | 1 | `[[0x144dec]+1936] +0x0` | `([[sp-40]]:1 & 0xff)` |
| `str` at 0x204e4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x204ec | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1936]+1` |
| `ldr` at 0x204f0 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x204f4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x204f8 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x204fc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20500 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x20504 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x20510 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20514 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20518 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x2051c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20520 | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x20524 | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1924]` |
| `ldr` at 0x20530 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x20538 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0x20548 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x2054c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20554 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x2055c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x20568 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x2056c | store | 1 | `[[0x144dec]+1936] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x20574 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x2057c | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1936]+1` |
| `ldr` at 0x20580 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20584 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20588 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x2058c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20590 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x20594 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x205a0 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x205a4 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x205a8 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x205ac | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x205b0 | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x205b4 | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1924]` |
| `ldrsb` at 0x205bc | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x205c8 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x205cc | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x205d4 | load | 4 | `[0x144dec] +0x790` | `-` |
| `strb` at 0x205e0 | store | 1 | `[[0x144dec]+1936] +0x0` | `0xa` |
| `str` at 0x205e8 | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1936]+1` |
| `ldr` at 0x205ec | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x205f0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x205f4 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x205f8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x205fc | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x20600 | load | 4 | `[0x144dec] +0x788` | `-` |
| `ldr` at 0x2060c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20610 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20614 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20618 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2061c | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x20620 | store | 4 | `[0x144dec] +0x790` | `[[0x144dec]+1924]` |
| `ldr` at 0x20624 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20628 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2062c | load | 4 | `abs 0x206fc` | `-` |
| `ldr` at 0x20630 | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20634 | load | 4 | `abs 0x144dec` | `-` |
| `strb` at 0x20648 | store | 1 | `[0x144dec] +0x7a3` | `(ret(0x21d34) & 0xff)` |
| `ldr` at 0x2064c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20650 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x2065c | load | 4 | `abs 0x206f0` | `-` |
| `ldr` at 0x20660 | load | 4 | `abs 0x144dec` | `-` |
| `str` at 0x20680 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x20684 | load | 4 | `abs 0x206f4` | `-` |
| `ldr` at 0x20698 | load | 4 | `abs 0x206f8` | `-` |
| `ldr` at 0x2069c | load | 4 | `abs 0x1452d8` | `-` |
| `str` at 0x206ac | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x206b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x206b4 | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x206c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x206cc | load | 4 | `abs 0x20700` | `-` |
| `str` at 0x206d0 | store | 4 | `sp -0x44` | `0xfc02c` |
| `str` at 0x206d4 | store | 4 | `sp -0x40` | `ret(0x21674)` |
| `ldr` at 0x206dc | load | 4 | `abs 0x20704` | `-` |
| `ldr` at 0x206e0 | load | 4 | `abs 0x20708` | `-` |
| `ldmdb` at 0x206ec | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x206ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x206ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x206ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x206ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20258 | `(unknown callee) 0x1b1e4` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x20278 | `(unknown callee) 0x1b22c` | `[sp-28]` | `?` | `?` | `0x5` |
| `bl` at 0x202f0 | `(unknown callee) 0x22a38` | `0x144dfc` | `?` | `[sp-28]` | `?` |
| `bl` at 0x20644 | `(unknown callee) 0x21d34` | `[0x144dec]+1920` | `0x24` | `?` | `[0x144dec]+1920` |
| `bl` at 0x20674 | `(unknown callee) 0x1b304` | `[0x144dec]+1956` | `[0x144dec]+1920` | `0x24` | `[0x144dec]+1920` |
| `bl` at 0x20678 | `(unknown callee) 0x1bb4c` | `ret(0x1b304)` | `?` | `?` | `?` |
| `bl` at 0x20688 | `(unknown callee) 0x22dc0` | `0x144dfc` | `?` | `?` | `0x0` |
| `bl` at 0x206c4 | `(unknown callee) 0x21674` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x206e8 | `(unknown callee) 0x21e28` | `0x9` | `0xfbd34` | `0x715` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x20298 | `[0x144dec] != 0x0` | `[0x144dec] == 0x0` |
| `beq` at 0x202dc | `? == 0xa` | `? != 0xa` |
| `beq` at 0x202fc | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0x2030c | `[0x1452d8] == 0x1` | `[0x1452d8] != 0x1` |
| `bls` at 0x20330 | `[[0x144dec]+1936] <= [[0x144dec]+1932] (unsigned, 32-bit)` | `[[0x144dec]+1936] > [[0x144dec]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x20394 | `[[0x144dec]+1936] >= [[0x144dec]+1932] (unsigned, 32-bit)` | `[[0x144dec]+1936] < [[0x144dec]+1932] (unsigned, 32-bit)` |
| `bge` at 0x203d8 | `? >= [sp-56] (signed, 32-bit)` | `? < [sp-56] (signed, 32-bit)` |
| `bne` at 0x20408 | `? != 0xa` | `? == 0xa` |
| `blo` at 0x20438 | `[[0x144dec]+1932] < [[0x144dec]+1928] (unsigned, 32-bit)` | `[[0x144dec]+1932] >= [[0x144dec]+1928] (unsigned, 32-bit)` |
| `bne` at 0x2047c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x2048c | `[sp-44] != 0x4` | `[sp-44] == 0x4` |
| `blo` at 0x2050c | `[[0x144dec]+1936] < [[0x144dec]+1928] (unsigned, 32-bit)` | `[[0x144dec]+1936] >= [[0x144dec]+1928] (unsigned, 32-bit)` |
| `bne` at 0x20540 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x2059c | `[[0x144dec]+1936] < [[0x144dec]+1928] (unsigned, 32-bit)` | `[[0x144dec]+1936] >= [[0x144dec]+1928] (unsigned, 32-bit)` |
| `beq` at 0x205c4 | `? == 0x0` | `? != 0x0` |
| `blo` at 0x20608 | `[[0x144dec]+1936] < [[0x144dec]+1928] (unsigned, 32-bit)` | `[[0x144dec]+1936] >= [[0x144dec]+1928] (unsigned, 32-bit)` |
| `beq` at 0x20694 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0x206a4 | `[0x1452d8] == 0x1` | `[0x1452d8] != 0x1` |
| `beq` at 0x206bc | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x206ec | `r0` | `?` |

## `nvramLogFinsert`

- `nvramLogFinsert`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2070c to 0x20750.
  Established. The first call reaches `0x1af80` with `sp-156`, entry r0, `sp-12`, `sp-156` (`bl` at
  0x20738). The next call to `0x20240` has `sp-156`, unknown, unknown, `sp-156` (`bl` at 0x20744). A
  prior word store writes `sp-12` at `sp -0x24` (`str` at 0x20724). The r0 return is unknown, so
  it is not assigned either callee result (`ldmdb` at 0x2074c). Formatting bounds and physical
  logging effects remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20724 | store | 4 | `sp -0x24` | `sp-12` |
| `ldr` at 0x20730 | load | 4 | `sp -0x10` | `-` |
| `ldr` at 0x20734 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x2074c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x2074c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2074c | load | 4 | `sp -0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20738 | `(unknown callee) 0x1af80` | `sp-156` | `r0` | `sp-12` | `sp-156` |
| `bl` at 0x20744 | `(unknown callee) 0x20240` | `sp-156` | `?` | `?` | `sp-156` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2074c | `r0` | `?` |

## `nvramLogFetch`

- `nvramLogFetch`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20750 to 0x20b38.
  Established. The first call to `0x1b124` has `0x1`, entry r1, `[[0x144dec]+1936]`,
  `[[0x144dec]+1932]` (`bl` at 0x20824); its nonzero result takes the arm that stores byte `0x0` at
  `ret(0x1b124) +0x0` (`bne` at 0x20838, `strb` at 0x20850). Zero falls through (`bne` at 0x20838). The later call to
  `0x1b13c` uses `([sp-20] + [sp-68])`, `0x13`, `0xfc054`, `ret(0x1afe0)` (`bl` at 0x20a48). A final
  byte store writes `0x0` at an unknown target (`strb` at 0x20b1c). Log record interpretation,
  allocation extent and the joined r0 value remain unknown (`ldmdb` at 0x20b2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20764 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x20768 | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x2076c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20770 | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x20774 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20778 | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x2077c | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x20788 | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x2078c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20790 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `str` at 0x20794 | store | 4 | `sp -0x18` | `[[0x144dec]+1932]` |
| `ldr` at 0x20798 | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x2079c | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x207a0 | load | 4 | `[0x144dec] +0x790` | `-` |
| `str` at 0x207a4 | store | 4 | `sp -0x1c` | `[[0x144dec]+1936]` |
| `str` at 0x207ac | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x207b4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x207bc | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x207c0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x207c4 | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x207c8 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x207cc | load | 4 | `[0x144dec] +0x790` | `-` |
| `ldr` at 0x207d0 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `ldr` at 0x207dc | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x207e0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x207e4 | load | 4 | `[0x144dec] +0x78c` | `-` |
| `str` at 0x207e8 | store | 4 | `sp -0x18` | `[[0x144dec]+1932]` |
| `ldr` at 0x207ec | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x207f0 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x207f4 | load | 4 | `[0x144dec] +0x788` | `-` |
| `str` at 0x207f8 | store | 4 | `sp -0x1c` | `[[0x144dec]+1928]` |
| `ldr` at 0x207fc | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x20800 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20804 | load | 4 | `[0x144dec] +0x784` | `-` |
| `str` at 0x20808 | store | 4 | `sp -0x20` | `[[0x144dec]+1924]` |
| `ldr` at 0x2080c | load | 4 | `abs 0x20b30` | `-` |
| `ldr` at 0x20810 | load | 4 | `abs 0x144dec` | `-` |
| `ldr` at 0x20814 | load | 4 | `[0x144dec] +0x790` | `-` |
| `str` at 0x20818 | store | 4 | `sp -0x24` | `[[0x144dec]+1936]` |
| `str` at 0x2082c | store | 4 | `sp -0x14` | `ret(0x1b124)` |
| `ldr` at 0x20830 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20840 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x20848 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x20850 | store | 1 | `ret(0x1b124) +0x0` | `0x0` |
| `ldr` at 0x20854 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20858 | store | 4 | `sp -0x48` | `[sp-20]` |
| `str` at 0x20864 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x20868 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2086c | store | 4 | `sp -0x2c` | `[[0x144dec]+1932]` |
| `ldr` at 0x20870 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x20874 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x20878 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20888 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x2088c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20890 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x20894 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x208a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x208b0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x208b8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x208c0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x208c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x208d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x208d8 | store | 4 | `sp -0x44` | `?` |
| `str` at 0x208e0 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x208e8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x208f0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x208f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x208fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20904 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x20918 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2091c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x20920 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x20924 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x20928 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2092c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x20930 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x20934 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x2093c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20948 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x20950 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x2095c | store | 4 | `sp -0x4c` | `0x64` |
| `ldr` at 0x20960 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x20964 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x20968 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x20974 | store | 4 | `sp -0x14` | `ret(0x1b124)` |
| `ldr` at 0x20978 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20988 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x20994 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x2099c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x209a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x209a4 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x209a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x209ac | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x209b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x209c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x209c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x209c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x209cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x209dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x209ec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x209f0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x209f8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x209fc | store | 1 | `[sp-60] +0x0` | `?` |
| `str` at 0x20a04 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x20a08 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x20a10 | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0x20a2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20a30 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20a40 | load | 4 | `abs 0x20b34` | `-` |
| `ldr` at 0x20a4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20a50 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20a68 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x20a70 | store | 4 | `sp -0x44` | `([sp-68] + ret(0x1b22c))` |
| `ldr` at 0x20a78 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20a7c | load | 1 | `(unknown)` | `-` |
| `str` at 0x20a8c | store | 4 | `sp -0x3c` | `sp-56` |
| `str` at 0x20a94 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x20aa0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x20aa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20ab0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x20ab4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x20ab8 | store | 1 | `(unknown)` | `?` |
| `str` at 0x20ac0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x20ac4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x20acc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x20ad4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x20ad8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x20ae8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x20aec | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x20af0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x20af4 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x20af8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x20afc | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x20b00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x20b04 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x20b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20b10 | load | 4 | `sp -0x44` | `-` |
| `strb` at 0x20b1c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x20b20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20b24 | store | 4 | `sp -0x48` | `[sp-20]` |
| `ldr` at 0x20b28 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0x20b2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20b2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20b2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20824 | `(unknown callee) 0x1b124` | `0x1` | `r1` | `[[0x144dec]+1936]` | `[[0x144dec]+1932]` |
| `bl` at 0x2096c | `(unknown callee) 0x1b124` | `?` | `r1` | `?` | `?` |
| `bl` at 0x20a24 | `(unknown callee) 0x1afe0` | `sp-56` | `[sp-60]` | `[sp-60]+1` | `sp-56` |
| `bl` at 0x20a48 | `(unknown callee) 0x1b13c` | `([sp-20] + [sp-68])` | `0x13` | `0xfc054` | `ret(0x1afe0)` |
| `bl` at 0x20a5c | `(unknown callee) 0x1b22c` | `([sp-20] + [sp-68])` | `?` | `[sp-20]` | `([sp-20] + [sp-68])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x20784 | `[[0x144dec]+1936] <= [[0x144dec]+1932] (unsigned, 32-bit)` | `[[0x144dec]+1936] > [[0x144dec]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x207d8 | `[[0x144dec]+1936] >= [[0x144dec]+1932] (unsigned, 32-bit)` | `[[0x144dec]+1936] < [[0x144dec]+1932] (unsigned, 32-bit)` |
| `bne` at 0x20838 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0x20880 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x2089c | `C == 0` | `not (C == 0)` |
| `beq` at 0x208ac | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x208cc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x20910 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x20944 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x20980 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0x209b8 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x209d4 | `C == 0` | `not (C == 0)` |
| `beq` at 0x209e4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x20a18 | `[sp-52]-1 != 0x0` | `[sp-52]-1 == 0x0` |
| `bne` at 0x20a84 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x20ae0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20b2c | `r0` | `?` |

## `exceptionInit`

- `exceptionInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20c18 to 0x20c60.
  Established. The call to `0x1b154` uses entry r0, entry r1, entry r2, `0xfc064` (`bl` at 0x20c34);
  its following absolute word store writes an unknown value at `0x144e78` (`str` at 0x20c48). The
  return has r0 `0x0` (`ldmdb` at 0x20c54). Reset-source interpretation and the stored value
  encoding are unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x20c28 | load | 4 | `abs 0x20c58` | `-` |
| `str` at 0x20c2c | store | 4 | `sp -0x1c` | `0xfc064` |
| `ldr` at 0x20c30 | load | 4 | `abs 0x20c5c` | `-` |
| `str` at 0x20c48 | store | 4 | `abs 0x144e78` | `?` |
| `ldmdb` at 0x20c54 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x20c54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20c54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20c54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20c34 | `(unknown callee) 0x1b154` | `r0` | `r1` | `r2` | `0xfc064` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20c54 | `r0` | `0x0` |

## `exceptionIsSoftReset`

- `exceptionIsSoftReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20c60 to 0x20cdc.
  Established. The absolute-word test takes the one-result arm for `[0x144e78] != 0x0` and falls
  through otherwise (`bne` at 0x20c7c). The fall-through call to `0x1b010` has `0xfc074`, entry r1,
  entry r2, `[0x144e78]` (`bl` at 0x20c84). A later call to `0x1aff8` uses `ret(0x1b010)`,
  `0xfc084`, unknown, `ret(0x1b010)` (`bl` at 0x20ca4). Separate result arms store word `0x1` and
  `0x0` at `sp -0x18` (`str` at 0x20cb8, `str` at 0x20cc4). The joined return and physical reset
  classification are unknown (`ldmdb` at 0x20ccc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x20c70 | load | 4 | `abs 0x20cd0` | `-` |
| `ldr` at 0x20c74 | load | 4 | `abs 0x144e78` | `-` |
| `ldr` at 0x20c80 | load | 4 | `abs 0x20cd4` | `-` |
| `str` at 0x20c8c | store | 4 | `sp -0x14` | `ret(0x1b010)` |
| `ldr` at 0x20c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20ca0 | load | 4 | `abs 0x20cd8` | `-` |
| `str` at 0x20cb8 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x20cc4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x20cc8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x20ccc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20ccc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20ccc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20c84 | `(unknown callee) 0x1b010` | `0xfc074` | `r1` | `r2` | `[0x144e78]` |
| `bl` at 0x20ca4 | `(unknown callee) 0x1aff8` | `ret(0x1b010)` | `0xfc084` | `?` | `ret(0x1b010)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x20c7c | `[0x144e78] != 0x0` | `[0x144e78] == 0x0` |
| `beq` at 0x20c98 | `ret(0x1b010) == 0x0` | `ret(0x1b010) != 0x0` |
| `bne` at 0x20cb0 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20ccc | `r0` | `?` |

## `exceptionReset`

- `exceptionReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20cdc to 0x20d8c.
  Established. The opening absolute-word branch takes the bypass for `[0x144e78] == 0x0` and falls
  through for nonzero (`beq` at 0x20d00). Later calls include `0x1b0dc` with `0xfc0c8` and three
  unknown candidates, then `0x226f0` with `0x1388` and three unknown candidates (`bl` at 0x20d4c,
  `bl` at 0x20d54). The last reported call targets `0x1b3ac` with all candidates unknown (`bl` at
  0x20d6c). No return is recognized in the selected span. Physical reset effects, timing units and
  whether called routines return remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20cec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x20cf0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x20cf4 | load | 4 | `abs 0x20d70` | `-` |
| `ldr` at 0x20cf8 | load | 4 | `abs 0x144e78` | `-` |
| `ldr` at 0x20d04 | load | 4 | `abs 0x20d74` | `-` |
| `ldr` at 0x20d08 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x20d0c | load | 4 | `abs 0x20d78` | `-` |
| `ldr` at 0x20d1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20d20 | load | 4 | `abs 0x20d7c` | `-` |
| `ldr` at 0x20d34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20d40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20d48 | load | 4 | `abs 0x20d80` | `-` |
| `ldr` at 0x20d50 | load | 4 | `abs 0x20d84` | `-` |
| `ldr` at 0x20d58 | load | 4 | `abs 0x20d74` | `-` |
| `ldr` at 0x20d5c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x20d60 | load | 4 | `abs 0x20d88` | `-` |
| `ldrdeq` at 0x20d88 | load | 8 | `abs 0x20d90` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20d10 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0xfc08c` | `r2` | `0x144dc8` |
| `bl` at 0x20d18 | `(unknown callee) 0x1b3ac` | `?` | `?` | `?` | `?` |
| `bl` at 0x20d30 | `(unknown callee) 0x1b3ac` | `?` | `?` | `?` | `0x3e7` |
| `bl` at 0x20d44 | `(unknown callee) 0xcb8ec` | `?` | `?` | `?` | `?` |
| `bl` at 0x20d4c | `(unknown callee) 0x1b0dc` | `0xfc0c8` | `?` | `?` | `?` |
| `bl` at 0x20d54 | `(unknown callee) 0x226f0` | `0x1388` | `?` | `?` | `?` |
| `bl` at 0x20d64 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0xfc0d8` | `?` | `0x144dc8` |
| `bl` at 0x20d6c | `(unknown callee) 0x1b3ac` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x20d00 | `[0x144e78] == 0x0` | `[0x144e78] != 0x0` |
| `bne` at 0x20d28 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x20d3c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|

## `exceptionBootComplete`

- `exceptionBootComplete`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20d8c to 0x20da4.
  Established. The decoded body sets r3 to `0x0`, copies it into r0 and restores the saved registers
  at return (`mov` at 0x20d98, `mov` at 0x20d9c, `ldmdb` at 0x20da0). The facts report r0 `0x0`
  there (`ldmdb` at 0x20da0). There are no calls or conditional branches. A boot-completion action
  is not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x20da0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20da0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20da0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20da0 | `r0` | `0x0` |

## `exceptionResetReport`

- `exceptionResetReport`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20da4 to 0x20db4.
  Established. The body saves its frame, establishes fp and immediately restores the frame at return
  (`push` at 0x20da8, `sub` at 0x20dac, `ldmdb` at 0x20db0). The reported r0 value remains entry r0
  (`ldmdb` at 0x20db0). No calls, conditional branches or explicit data stores appear in the fact
  sections. A reset report is not emitted by any visible call.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x20db0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20db0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20db0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20db0 | `r0` | `r0` |

## `exceptionDumpPANIC`

- `exceptionDumpPANIC`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20db4 to 0x20dc4.
  Established. The frame setup and return are the complete decoded body after the initial ip
  assignment (`mov` at 0x20db4, `push` at 0x20db8, `sub` at 0x20dbc, `ldmdb` at 0x20dc0). The exit
  retains entry r0 (`ldmdb` at 0x20dc0). Calls and conditional branches are absent from the facts.
  The routine does not establish a panic-reporting role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x20dc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20dc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20dc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20dc0 | `r0` | `r0` |

## `SHUB_ScanSetRackSlot`

- `SHUB_ScanSetRackSlot`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48548 to 0x486e4.
  Established. The loop call to `0x47a98` has `[sp-20]`, `0x101d0100`, `[sp-28]`, unknown (`bl` at
  0x48678); its result test takes the retry path for `ret(0x47a98) != 0x1` and falls through on
  equality (`bne` at 0x48684). The equality path stores word `0x0` at `sp -0x2c`, while a later
  path stores word `0xfffff88e` there (`str` at 0x4868c, `str` at 0x486b8). Rack-slot
  encoding, register-write semantics and the joined r0 value remain unproved (`ldmdb` at 0x486c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48558 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x48560 | store | 1 | `sp -0x21` | `0x6c` |
| `str` at 0x48570 | store | 4 | `sp -0x28` | `ret(0x74908)` |
| `ldr` at 0x48574 | load | 4 | `abs 0x486c4` | `-` |
| `ldr` at 0x48578 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x48584 | load | 4 | `abs 0x486c8` | `-` |
| `str` at 0x48588 | store | 4 | `sp -0x38` | `0x1094e0` |
| `ldr` at 0x48590 | load | 4 | `abs 0x486cc` | `-` |
| `ldr` at 0x48594 | load | 4 | `abs 0x486d0` | `-` |
| `ldr` at 0x485a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x485a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x485b0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x485b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x485b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x485c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x485c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x485d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x485e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x485ec | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x485fc | load | 1 | `sp -0x21` | `-` |
| `str` at 0x48604 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x48608 | load | 4 | `abs 0x486c4` | `-` |
| `ldr` at 0x4860c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x48618 | load | 4 | `abs 0x486d4` | `-` |
| `str` at 0x4861c | store | 4 | `sp -0x38` | `0x1094f8` |
| `ldr` at 0x48620 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48624 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x48628 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4862c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x48634 | load | 4 | `abs 0x486cc` | `-` |
| `str` at 0x48648 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4864c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4865c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4866c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48670 | load | 4 | `abs 0x486d8` | `-` |
| `ldr` at 0x48674 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4868c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x48694 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4869c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x486a4 | load | 4 | `abs 0x486dc` | `-` |
| `ldr` at 0x486a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x486ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x486b4 | load | 4 | `abs 0x486e0` | `-` |
| `str` at 0x486b8 | store | 4 | `sp -0x2c` | `0xfffff88e` |
| `ldr` at 0x486bc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x486c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x486c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x486c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48568 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `0x6c` |
| `bl` at 0x4859c | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x1cd` | `0x0` |
| `bl` at 0x48640 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x1d8` | `0x0` |
| `bl` at 0x48678 | `(unknown callee) 0x47a98` | `[sp-20]` | `0x101d0100` | `[sp-28]` | `?` |
| `bl` at 0x486b0 | `(unknown callee) 0x2070c` | `0x10952c` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x48580 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x48614 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `ble` at 0x48654 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x48684 | `ret(0x47a98) != 0x1` | `ret(0x47a98) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x486c0 | `r0` | `?` |

## `brickGetSerialSlab`

- `brickGetSerialSlab`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f148 to 0x4f2f0.
  Established. The computed selector takes an unresolved edge for `[sp-28] <= 0x5` unsigned and
  falls through above that bound (`ldrls` at 0x4f1c0). A known later call to `0x1afa4` has all four
  candidates unknown (`bl` at 0x4f2bc); its result is stored as a word at an unknown effective
  target (`str` at 0x4f2c4). The known-edge r0 return is `[sp-40]` (`ldmdb` at
  0x4f2dc). Excluded selector cases, serial-record layout and caching semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f158 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x4f160 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4f164 | load | 4 | `abs 0x4f2e0` | `-` |
| `ldr` at 0x4f168 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f16c | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f198 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4f1b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x4f1c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f228 | load | 4 | `abs 0x4f2e4` | `-` |
| `str` at 0x4f22c | store | 4 | `sp -0x2c` | `0x10a72c` |
| `ldr` at 0x4f234 | load | 4 | `abs 0x4f2e8` | `-` |
| `ldr` at 0x4f238 | load | 4 | `abs 0x4f2ec` | `-` |
| `ldr` at 0x4f244 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4f260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f26c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f280 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4f288 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f294 | load | 4 | `abs 0x4f2e0` | `-` |
| `ldr` at 0x4f298 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f29c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f2b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4f2b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f2c4 | store | 4 | `(unknown)` | `ret(0x1afa4)` |
| `ldr` at 0x4f2c8 | load | 4 | `abs 0x4f2e0` | `-` |
| `ldr` at 0x4f2cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f2d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f2d4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4f2d8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4f2dc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4f2dc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4f2dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f2dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f2dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4f184 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-36` | `0x148284` | `sp-36` |
| `bl` at 0x4f1a0 | `(unknown callee) 0x4f090` | `ret(0xd1dfc)` | `?` | `?` | `ret(0xd1dfc)` |
| `bl` at 0x4f240 | `(unknown callee) 0x21e28` | `0x9` | `0x10a764` | `0x1af` | `0x0` |
| `bl` at 0x4f2bc | `(unknown callee) 0x1afa4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f174 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x4f190 | `ret(0xd1dfc) != 0x0` | `ret(0xd1dfc) == 0x0` |
| `beq` at 0x4f1b0 | `? == 0x40` | `? != 0x40` |
| `ldrls` at 0x4f1c0 | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |
| `bne` at 0x4f268 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x4f290 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f2dc | `r0` | `[sp-40]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4f1c0).
