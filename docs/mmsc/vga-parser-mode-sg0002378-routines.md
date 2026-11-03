# MMSC VGA parser and mode paths in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
The analysed spans end at the next text symbol, with exclusive inferred ends.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges.
The computed transfers in `_vgaHook`, `_vgaWriteString`, `_vgaSetMode` and `_vgaSetAttrib` remain unresolved. Effects behind their selector arms are excluded.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. The full
fact sheets also list implicit accesses made by `push` and `pop`. The listings retain
`call` and `ret` instructions without separate rows for their implicit stack accesses.
Candidate slots can contain saved registers, local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/vga-parser-mode-sg0002378-facts.txt) include all tool sections.

## `_vgaHook`

- `_vgaHook`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109128 to 0x109208.
  Established. The range branch is taken when `[sp+12] > 0x4 (unsigned, 32-bit)` (`ja` at 0x109137).
  The three explicit four-byte entry loads read `sp +0x4` (`mov` at 0x10912b), `sp +0x8` (`mov` at
  0x10912e), and `sp +0xc` (`mov` at 0x109131). The known-edge exit reports `[sp+12]` (`ret` at
  0x1091fa). Effects behind the computed transfer remain excluded, and the routine role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10912b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10912e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x109131 | load | 4 | `sp +0xc` | `-` |
| `jmp` at 0x10913d | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x109137 | `[sp+12] > 0x4 (unsigned, 32-bit)` | `[sp+12] <= 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1091fa | `eax` | `[sp+12]` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10913d).

## `_vgaWriteString`

- `_vgaWriteString`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x109798 to 0x10a5b8.
  Established. A byte store writes `0x1` to `[sp+4] +0x70` (`mov` at 0x1097b3). The branch taken
  when `[sp-5]:1 == 0x7` (`je` at 0x1098a5) reaches `_vgaConBeep` with candidate zero `0x0` (`call`
  at 0x10994a). A two-byte store through `[[[sp+4]+232]+20] +0x0` has unknown value (`mov` at
  0x109846). Neither unresolved selector contributes additional effects to this entry.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1097a1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1097a4 | load | 4 | `[sp+4] +0x50` | `-` |
| `mov` at 0x1097a7 | store | 4 | `sp -0x14` | `[[sp+4]+80]` |
| `mov` at 0x1097aa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1097ad | load | 4 | `[sp+4] +0xe8` | `-` |
| `mov` at 0x1097b3 | store | 1 | `[sp+4] +0x70` | `0x1` |
| `mov` at 0x1097b7 | load | 1 | `[[sp+4]+232] +0x18` | `-` |
| `mov` at 0x1097ba | store | 1 | `sp -0x10` | `[[[sp+4]+232]+24]:1` |
| `mov` at 0x1097bd | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x1097c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1097c7 | load | 1 | `[sp+4] +0x71` | `-` |
| `mov` at 0x1097ce | load | 1 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1097d5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1097d8 | store | 1 | `[sp+4] +0x70` | `0x0` |
| `mov` at 0x1097e8 | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x1097eb | load | 4 | `[sp-20] +0x4` | `-` |
| `mov` at 0x1097ee | load | 4 | `[sp-20] +0x0` | `-` |
| `mov` at 0x1097f8 | load | 4 | `[sp-20] +0xc` | `-` |
| `mov` at 0x1097fb | load | 1 | `([[sp-20]+4] + [[sp-20]+12]) +0x0` | `-` |
| `mov` at 0x1097fe | store | 1 | `sp -0x5` | `[([[sp-20]+4] + [[sp-20]+12])]:1` |
| `mov` at 0x109802 | load | 4 | `[sp-20] +0x8` | `-` |
| `mov` at 0x10980d | load | 4 | `sp -0x14` | `-` |
| `mov` at 0x109810 | store | 4 | `[sp-20] +0x4` | `?` |
| `inc` at 0x109813 | load | 4 | `sp -0xc` | `-` |
| `inc` at 0x109813 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x109816 | load | 2 | `[[sp+4]+232] +0xa8` | `-` |
| `movzx` at 0x109823 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109827 | load | 4 | `[[sp+4]+232] +0x48` | `-` |
| `mov` at 0x10982a | load | 1 | `([sp-5]:1 + [[[sp+4]+232]+72]) +0x0` | `-` |
| `mov` at 0x109831 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x109834 | load | 1 | `sp -0x10` | `-` |
| `mov` at 0x10983b | load | 1 | `([sp-5]:1 + [[[sp+4]+232]+72]) +0x0` | `-` |
| `mov` at 0x109846 | store | 2 | `[[[sp+4]+232]+20] +0x0` | `?` |
| `mov` at 0x109849 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10984d | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x109854 | load | 4 | `[[sp+4]+232] +0x30` | `-` |
| `mov` at 0x109868 | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x10986c | store | 4 | `[[sp+4]+232] +0x10` | `?` |
| `mov` at 0x10986f | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x109872 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x109878 | store | 4 | `[[sp+4]+232] +0x14` | `[[[sp+4]+232]+20]+2` |
| `movzx` at 0x109888 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109968 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10996b | load | 2 | `[sp+4] +0x78` | `-` |
| `mov` at 0x1099a8 | store | 2 | `[[sp+4]+232] +0xa8` | `0x2` |
| `mov` at 0x1099b8 | load | 4 | `abs 0x1ac070` | `-` |
| `mov` at 0x1099bd | store | 4 | `[[sp+4]+232] +0x48` | `[0x1ac070]` |
| `mov` at 0x1099d8 | load | 2 | `[[sp+4]+232] +0xa8` | `-` |
| `mov` at 0x109a58 | store | 2 | `[[sp+4]+232] +0xa8` | `0x1` |
| `movzx` at 0x109a61 | load | 1 | `sp -0x5` | `-` |
| `jmp` at 0x109a71 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x109b88 | store | 2 | `[[sp+4]+232] +0xa8` | `0x4` |
| `mov` at 0x109c6c | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x109c7d | store | 4 | `[[sp+4]+232] +0xac` | `0x0` |
| `mov` at 0x109c87 | store | 2 | `[[sp+4]+232] +0xa8` | `0x8` |
| `cmp` at 0x109c90 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109c96 | store | 2 | `[[sp+4]+232] +0xa8` | `0x20` |
| `cmp` at 0x109ca8 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109cb4 | store | 4 | `[[sp+4]+232] +0xf0` | `?` |
| `mov` at 0x109cba | load | 4 | `[[sp+4]+232] +0xf0` | `-` |
| `cmp` at 0x109cc8 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109cce | load | 4 | `[[sp+4]+232] +0xac` | `-` |
| `mov` at 0x109cd9 | load | 4 | `[[sp+4]+232] +0xac` | `-` |
| `mov` at 0x109ce0 | store | 4 | `[[sp+4]+232] +0xac` | `?` |
| `mov` at 0x109ce6 | load | 4 | `[[sp+4]+232] +0xac` | `-` |
| `mov` at 0x109cf8 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109d01 | load | 4 | `[[sp+4]+232] +0xac` | `-` |
| `mov` at 0x109d07 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x109d13 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x109d1a | load | 4 | `[[sp+4]+232] +0xac` | `-` |
| `mov` at 0x109d20 | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x109d27 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109d2f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x109d48 | store | 2 | `[[sp+4]+232] +0xa8` | `0x10` |
| `mov` at 0x109d51 | store | 2 | `[[sp+4]+232] +0xa8` | `0x1` |
| `movzx` at 0x109d5a | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x109d9a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x109db8 | load | 4 | `[[sp+4]+232] +0xf0` | `-` |
| `mov` at 0x109dc2 | load | 4 | `[[sp+4]+232] +0xb0` | `-` |
| `mov` at 0x109dd8 | load | 4 | `[[sp+4]+232] +0xb0` | `-` |
| `mov` at 0x109df8 | load | 4 | `[[sp+4]+232] +0xf0` | `-` |
| `mov` at 0x109e02 | store | 4 | `[[sp+4]+232] +0xf0` | `0x0` |
| `movzx` at 0x109e18 | load | 1 | `sp -0x5` | `-` |
| `jmp` at 0x109e28 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a188 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a2c8 | store | 2 | `[[sp+4]+232] +0xa8` | `0x1` |
| `cmp` at 0x10a2d1 | load | 1 | `sp -0x5` | `-` |
| `cmp` at 0x10a2f8 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x10a2fe | load | 4 | `abs 0x1ac074` | `-` |
| `mov` at 0x10a303 | store | 4 | `[[sp+4]+232] +0x48` | `[0x1ac074]` |
| `cmp` at 0x10a318 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x10a31e | load | 4 | `abs 0x1ac070` | `-` |
| `cmp` at 0x10a328 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x10a332 | load | 4 | `abs 0x1ac078` | `-` |
| `mov` at 0x10a337 | store | 4 | `[[sp+4]+232] +0x48` | `?` |
| `mov` at 0x10a33a | store | 2 | `[[sp+4]+232] +0xa8` | `0x1` |
| `mov` at 0x10a348 | store | 2 | `[[sp+4]+232] +0xa8` | `0x1` |
| `mov` at 0x10a351 | load | 4 | `[[sp+4]+232] +0x44` | `-` |
| `mov` at 0x10a35c | load | 4 | `[[sp+4]+232] +0x28` | `-` |
| `mov` at 0x10a366 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a36c | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a370 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a377 | load | 4 | `[[sp+4]+232] +0x28` | `-` |
| `mov` at 0x10a37a | store | 4 | `[[sp+4]+232] +0xc` | `[[[sp+4]+232]+40]` |
| `mov` at 0x10a37d | load | 1 | `[[sp+4]+232] +0x19` | `-` |
| `mov` at 0x10a388 | load | 4 | `[[sp+4]+232] +0x44` | `-` |
| `mov` at 0x10a38c | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a399 | load | 4 | `[[sp+4]+232] +0x28` | `-` |
| `mov` at 0x10a3a3 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a3a9 | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a3ad | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a3b8 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a3bd | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a3c2 | store | 4 | `[[sp+4]+232] +0x14` | `?` |
| `mov` at 0x10a3c5 | load | 4 | `[[sp+4]+232] +0x28` | `-` |
| `mov` at 0x10a3cf | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a3d5 | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a3d9 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a3e8 | load | 4 | `[[sp+4]+232] +0x44` | `-` |
| `mov` at 0x10a3f3 | load | 4 | `[[sp+4]+232] +0x24` | `-` |
| `mov` at 0x10a3f8 | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x10a3fe | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a402 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a409 | load | 4 | `[[sp+4]+232] +0x24` | `-` |
| `mov` at 0x10a40c | store | 4 | `[[sp+4]+232] +0xc` | `[[[sp+4]+232]+36]` |
| `mov` at 0x10a40f | load | 1 | `[[sp+4]+232] +0x19` | `-` |
| `mov` at 0x10a41a | load | 4 | `[[sp+4]+232] +0x44` | `-` |
| `mov` at 0x10a41e | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a42b | load | 4 | `[[sp+4]+232] +0x24` | `-` |
| `mov` at 0x10a430 | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x10a436 | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a43a | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a448 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a44b | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a451 | store | 4 | `[[sp+4]+232] +0x14` | `?` |
| `mov` at 0x10a454 | load | 4 | `[[sp+4]+232] +0x24` | `-` |
| `mov` at 0x10a459 | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x10a45f | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a463 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a478 | load | 4 | `[[sp+4]+232] +0x1c` | `-` |
| `mov` at 0x10a47d | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a483 | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a487 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a48e | load | 4 | `[[sp+4]+232] +0x1c` | `-` |
| `mov` at 0x10a493 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a499 | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a49d | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a4a8 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a4ad | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a4b2 | store | 4 | `[[sp+4]+232] +0x14` | `?` |
| `mov` at 0x10a4b5 | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a4b9 | store | 4 | `[[sp+4]+232] +0xc` | `?` |
| `mov` at 0x10a4bc | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a4bf | load | 4 | `[[sp+4]+232] +0x1c` | `-` |
| `mov` at 0x10a4c4 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a4ca | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a4ce | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a4e8 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a4eb | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a4f5 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a4f8 | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a504 | load | 4 | `[[sp+4]+232] +0x20` | `-` |
| `mov` at 0x10a507 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a50d | store | 4 | `[[sp+4]+232] +0x14` | `?` |
| `mov` at 0x10a510 | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a514 | store | 4 | `[[sp+4]+232] +0xc` | `?` |
| `mov` at 0x10a517 | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a51a | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a51d | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a528 | load | 4 | `[[sp+4]+232] +0x14` | `-` |
| `mov` at 0x10a52b | load | 4 | `[[sp+4]+232] +0x0` | `-` |
| `mov` at 0x10a547 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a54a | store | 1 | `[sp+4] +0x70` | `0x0` |
| `mov` at 0x10a54e | load | 4 | `sp -0x14` | `-` |
| `cmp` at 0x10a55a | load | 4 | `abs 0x1ac06c` | `-` |
| `mov` at 0x10a562 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10a571 | load | 4 | `abs 0x1dcce0` | `-` |
| `mov` at 0x10a57c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a585 | load | 4 | `abs 0x1dcce0` | `-` |
| `mov` at 0x10a58c | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10994a | `_vgaConBeep 0x108e08` | `0x0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x109959 | `_vgaBackSpace 0x1095a8` | `[[sp+4]+232]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x109978 | `_vgaCarriageReturn 0x109588` | `[[sp+4]+232]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x109999 | `_vgaCarriageReturn 0x109588` | `[[sp+4]+232]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1099c9 | `_vgaDelLeftChar 0x109548` | `[[sp+4]+232]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x109ba9 | `_vgaLineFeed 0x109678` | `[[sp+4]+232]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x109d9e | `_vgaSetMode 0x10a8d8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a18c | `_vgaEscResponse 0x10a5b8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a1b3 | `_vgaTab 0x1095f8` | `[[sp+4]+232]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a2e0 | `_vgaClear 0x109208` | `[[sp+4]+232]` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10a391 | `_vgaScroll 0x1092f8` | `[[sp+4]+232]` | `[[[sp+4]+232]+12]` | `[[[sp+4]+232]+68]` | `0x1` | `?` | `?` |
| `call` at 0x10a423 | `_vgaScroll 0x1092f8` | `[[sp+4]+232]` | `[[[sp+4]+232]+12]` | `[[[sp+4]+232]+68]` | `0x0` | `?` | `?` |
| `call` at 0x10a53f | `_vgaCursorPos 0x1096a8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a552 | `_rngFreeBytes 0x172948` | `[sp-20]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a569 | `_semGive 0x1729d4` | `[sp+4]+84` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a58a | `(unknown callee) (unknown target)` | `[sp+4]+136` | `0x1` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1097cc | `(([[sp+4]+113]:1 & 0xff) & ([[sp+4]+113]:1 & 0xff)) != 0x0` | `(([[sp+4]+113]:1 & 0xff) & ([[sp+4]+113]:1 & 0xff)) == 0x0` |
| `je` at 0x1097d3 | `(([[sp+4]+116]:1 & 0xff) & ([[sp+4]+116]:1 & 0xff)) == 0x0` | `(([[sp+4]+116]:1 & 0xff) & ([[sp+4]+116]:1 & 0xff)) != 0x0` |
| `je` at 0x1097f2 | `[[sp-20]] == [[sp-20]+4]` | `[[sp-20]] != [[sp-20]+4]` |
| `je` at 0x109809 | `? == [[sp-20]+8]` | `? != [[sp-20]+8]` |
| `jne` at 0x109821 | `([[[sp+4]+232]+168]:2 & 0xffff) != 0x1` | `([[[sp+4]+232]+168]:2 & 0xffff) == 0x1` |
| `je` at 0x10982f | `(([([sp-5]:1 + [[[sp+4]+232]+72])]:1 & 0xff) & ([([sp-5]:1 + [[[sp+4]+232]+72])]:1 & 0xff)) == 0x0` | `(([([sp-5]:1 + [[[sp+4]+232]+72])]:1 & 0xff) & ([([sp-5]:1 + [[[sp+4]+232]+72])]:1 & 0xff)) != 0x0` |
| `jne` at 0x109852 | `[[[sp+4]+232]+16] != ?` | `[[[sp+4]+232]+16] == ?` |
| `je` at 0x109859 | `([[[sp+4]+232]+48] & [[[sp+4]+232]+48]) == 0x0` | `([[[sp+4]+232]+48] & [[[sp+4]+232]+48]) != 0x0` |
| `je` at 0x10988f | `[sp-5]:1 == 0xc` | `[sp-5]:1 != 0xc` |
| `jg` at 0x109895 | `[sp-5]:1 > 0xc (signed, 32-bit)` | `[sp-5]:1 <= 0xc (signed, 32-bit)` |
| `je` at 0x10989a | `[sp-5]:1 == 0x9` | `[sp-5]:1 != 0x9` |
| `jg` at 0x1098a0 | `[sp-5]:1 > 0x9 (signed, 32-bit)` | `[sp-5]:1 <= 0x9 (signed, 32-bit)` |
| `je` at 0x1098a5 | `[sp-5]:1 == 0x7` | `[sp-5]:1 != 0x7` |
| `je` at 0x1098ae | `[sp-5]:1 == 0x8` | `[sp-5]:1 != 0x8` |
| `je` at 0x1098cb | `[sp-5]:1 == 0xa` | `[sp-5]:1 != 0xa` |
| `je` at 0x1098d4 | `[sp-5]:1 == 0xb` | `[sp-5]:1 != 0xb` |
| `je` at 0x1098eb | `[sp-5]:1 == 0xf` | `[sp-5]:1 != 0xf` |
| `jg` at 0x1098f1 | `[sp-5]:1 > 0xf (signed, 32-bit)` | `[sp-5]:1 <= 0xf (signed, 32-bit)` |
| `je` at 0x1098f6 | `[sp-5]:1 == 0xd` | `[sp-5]:1 != 0xd` |
| `je` at 0x1098ff | `[sp-5]:1 == 0xe` | `[sp-5]:1 != 0xe` |
| `je` at 0x10991b | `[sp-5]:1 == 0x7f` | `[sp-5]:1 != 0x7f` |
| `jg` at 0x109921 | `[sp-5]:1 > 0x7f (signed, 32-bit)` | `[sp-5]:1 <= 0x7f (signed, 32-bit)` |
| `je` at 0x109926 | `[sp-5]:1 == 0x1b` | `[sp-5]:1 != 0x1b` |
| `je` at 0x10993d | `[sp-5]:1 == 0x9b` | `[sp-5]:1 != 0x9b` |
| `je` at 0x109971 | `(? & 0x2) == 0x0` | `(? & 0x2) != 0x0` |
| `je` at 0x1099e7 | `? == 0x10` | `? != 0x10` |
| `jg` at 0x1099ed | `? > 0x10 (signed, 32-bit)` | `? <= 0x10 (signed, 32-bit)` |
| `je` at 0x1099f2 | `? == 0x4` | `? != 0x4` |
| `jg` at 0x1099f8 | `? > 0x4 (signed, 32-bit)` | `? <= 0x4 (signed, 32-bit)` |
| `je` at 0x1099fd | `? == 0x2` | `? != 0x2` |
| `je` at 0x109a0b | `? == 0x8` | `? != 0x8` |
| `je` at 0x109a1b | `? == 0x40` | `? != 0x40` |
| `jg` at 0x109a21 | `? > 0x40 (signed, 32-bit)` | `? <= 0x40 (signed, 32-bit)` |
| `je` at 0x109a26 | `? == 0x20` | `? != 0x20` |
| `je` at 0x109a3d | `? == 0x80` | `? != 0x80` |
| `je` at 0x109a48 | `? == 0x100` | `? != 0x100` |
| `ja` at 0x109a6b | `[sp-5]:1-35 > 0x40 (unsigned, 32-bit)` | `[sp-5]:1-35 <= 0x40 (unsigned, 32-bit)` |
| `jle` at 0x109c7b | `? <= 0xf (signed, 32-bit)` | `? > 0xf (signed, 32-bit)` |
| `jne` at 0x109c94 | `([sp-5]:1 & 0xff) != 0x5b` | `([sp-5]:1 & 0xff) == 0x5b` |
| `jne` at 0x109cc2 | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) != 0x0` | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) == 0x0` |
| `jne` at 0x109ccc | `([sp-5]:1 & 0xff) != 0x3b` | `([sp-5]:1 & 0xff) == 0x3b` |
| `jg` at 0x109cd7 | `[[[sp+4]+232]+172] > 0xe (signed, 32-bit)` | `[[[sp+4]+232]+172] <= 0xe (signed, 32-bit)` |
| `ja` at 0x109cff | `([sp-5]:1-48 & 0xff) > 0x9 (unsigned, 8-bit)` | `([sp-5]:1-48 & 0xff) <= 0x9 (unsigned, 8-bit)` |
| `je` at 0x109d61 | `[sp-5]:1 == 0x6c` | `[sp-5]:1 != 0x6c` |
| `jg` at 0x109d63 | `[sp-5]:1 > 0x6c (signed, 32-bit)` | `[sp-5]:1 <= 0x6c (signed, 32-bit)` |
| `je` at 0x109d68 | `[sp-5]:1 == 0x68` | `[sp-5]:1 != 0x68` |
| `je` at 0x109d7b | `[sp-5]:1 == 0x6e` | `[sp-5]:1 != 0x6e` |
| `jne` at 0x109dc0 | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) != 0x0` | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) == 0x0` |
| `jne` at 0x109dcb | `[[[sp+4]+232]+176] != 0x5` | `[[[sp+4]+232]+176] == 0x5` |
| `jne` at 0x109de1 | `[[[sp+4]+232]+176] != 0x6` | `[[[sp+4]+232]+176] == 0x6` |
| `je` at 0x109e00 | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) == 0x0` | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) != 0x0` |
| `ja` at 0x109e22 | `[sp-5]:1-64 > 0x35 (unsigned, 32-bit)` | `[sp-5]:1-64 <= 0x35 (unsigned, 32-bit)` |
| `jne` at 0x10a2d5 | `([sp-5]:1 & 0xff) != 0x38` | `([sp-5]:1 & 0xff) == 0x38` |
| `jne` at 0x10a2fc | `([sp-5]:1 & 0xff) != 0x4f` | `([sp-5]:1 & 0xff) == 0x4f` |
| `jne` at 0x10a31c | `([sp-5]:1 & 0xff) != 0x42` | `([sp-5]:1 & 0xff) == 0x42` |
| `jne` at 0x10a32c | `([sp-5]:1 & 0xff) != 0x55` | `([sp-5]:1 & 0xff) == 0x55` |
| `je` at 0x10a356 | `([[[sp+4]+232]+68] & [[[sp+4]+232]+68]) == 0x0` | `([[[sp+4]+232]+68] & [[[sp+4]+232]+68]) != 0x0` |
| `jb` at 0x10a375 | `[[[sp+4]+232]+20] < ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] >= ? (unsigned, 32-bit)` |
| `jb` at 0x10a3b2 | `[[[sp+4]+232]+20] < ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] >= ? (unsigned, 32-bit)` |
| `jae` at 0x10a3de | `[[[sp+4]+232]+20] >= ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] < ? (unsigned, 32-bit)` |
| `je` at 0x10a3ed | `([[[sp+4]+232]+68] & [[[sp+4]+232]+68]) == 0x0` | `([[[sp+4]+232]+68] & [[[sp+4]+232]+68]) != 0x0` |
| `jae` at 0x10a407 | `[[[sp+4]+232]+20] >= ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] < ? (unsigned, 32-bit)` |
| `jae` at 0x10a43f | `[[[sp+4]+232]+20] >= ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] < ? (unsigned, 32-bit)` |
| `jb` at 0x10a468 | `[[[sp+4]+232]+20] < ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] >= ? (unsigned, 32-bit)` |
| `jbe` at 0x10a48c | `[[[sp+4]+232]+20] <= ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] > ? (unsigned, 32-bit)` |
| `jbe` at 0x10a4a2 | `[[[sp+4]+232]+20] <= ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] > ? (unsigned, 32-bit)` |
| `ja` at 0x10a4d3 | `[[[sp+4]+232]+20] > ? (unsigned, 32-bit)` | `[[[sp+4]+232]+20] <= ? (unsigned, 32-bit)` |
| `jae` at 0x10a4ef | `[[[sp+4]+232]+20] >= [[[sp+4]+232]] (unsigned, 32-bit)` | `[[[sp+4]+232]+20] < [[[sp+4]+232]] (unsigned, 32-bit)` |
| `jae` at 0x10a4fc | `[[[sp+4]+232]+20] >= [[[sp+4]+232]] (unsigned, 32-bit)` | `[[[sp+4]+232]+20] < [[[sp+4]+232]] (unsigned, 32-bit)` |
| `jb` at 0x10a521 | `[[[sp+4]+232]+20] < [[[sp+4]+232]] (unsigned, 32-bit)` | `[[[sp+4]+232]+20] >= [[[sp+4]+232]] (unsigned, 32-bit)` |
| `jg` at 0x10a560 | `[0x1ac06c] > ret(_rngFreeBytes) (signed, 32-bit)` | `[0x1ac06c] <= ret(_rngFreeBytes) (signed, 32-bit)` |
| `je` at 0x10a578 | `[0x1dcce0] == 0x0` | `[0x1dcce0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10a598 | `eax` | `?` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x109a71).

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x109e28).

## `_vgaSetMode`

- `_vgaSetMode`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10a8d8 to 0x10aa48.
  Established. The fall-through predicate `not (Z == 0)` (`jne` at 0x10aa22) reaches a four-byte
  store of `[sp+8]` to `[[sp+4]+232] +0x54` (`mov` at 0x10aa24). The earlier computed-transfer
  operand is a four-byte load with an unknown effective address (`jmp` at 0x10a917); its
  selector-arm effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10a8de | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10a8e1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a8e4 | load | 4 | `[sp+4] +0xe8` | `-` |
| `mov` at 0x10a8f8 | load | 4 | `[[sp+4]+232] +0xf0` | `-` |
| `mov` at 0x10a906 | load | 4 | `(unknown)` | `-` |
| `jmp` at 0x10a917 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10aa18 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10aa24 | store | 4 | `[[sp+4]+232] +0x54` | `[sp+8]` |
| `mov` at 0x10aa28 | load | 4 | `[[sp+4]+232] +0xac` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10a900 | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) == 0x0` | `([[[sp+4]+232]+240] & [[[sp+4]+232]+240]) != 0x0` |
| `ja` at 0x10a911 | `? > 0x18 (unsigned, 32-bit)` | `? <= 0x18 (unsigned, 32-bit)` |
| `jne` at 0x10aa22 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x10aa30 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10aa3f | `eax` | `[[[sp+4]+232]+172]` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10a917).

## `_vgaClearLine`

- `_vgaClearLine`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10aab8 to 0x10ab68.
  Established. The shared loop contains a two-byte store with unknown target and value (`mov` at
  0x10ab48). The four-byte zero store to `[sp+4] +0x10` (`mov` at 0x10ab30) lies on the path following
  both `([[sp+4]+176] & [[sp+4]+176]) != 0x0` (`jne` at 0x10aad5) and `[[sp+4]+176] != 0x1` (`jne`
  at 0x10ab01). No total store extent is resolved for the loop.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10aabd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10aac0 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x10aacd | load | 4 | `[sp+4] +0xb0` | `-` |
| `mov` at 0x10aad7 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10aadd | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10aae0 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10aae6 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10aaeb | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10aaf8 | load | 4 | `[sp+4] +0xb0` | `-` |
| `mov` at 0x10ab03 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ab06 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ab0c | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10ab11 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10ab18 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ab1b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ab21 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10ab26 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ab2d | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10ab30 | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x10ab48 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10aad5 | `([[sp+4]+176] & [[sp+4]+176]) != 0x0` | `([[sp+4]+176] & [[sp+4]+176]) == 0x0` |
| `jne` at 0x10ab01 | `[[sp+4]+176] != 0x1` | `[[sp+4]+176] == 0x1` |
| `jbe` at 0x10ab50 | `? <= ? (unsigned, 32-bit)` | `? > ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ab5a | `eax` | `?` |

## `_vgaInsertLine`

- `_vgaInsertLine`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ab68 to 0x10abf8.
  Established. The `_vgaScroll` site has leading candidates `[sp+4]`, `[[sp+4]+12]`, unknown, and
  `0x0` (`call` at 0x10abc1). Four-byte stores include `[[sp+4]+12]` at `[sp+4] +0x24` (`mov` at
  0x10ab7a) and zero at `[sp+4] +0x10` (`mov` at 0x10abd3). The final four-byte stores have values
  `[[sp+4]+36]` (`mov` at 0x10abda) and `[[sp+4]+40]` (`mov` at 0x10abdd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ab6e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ab71 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10ab74 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10ab77 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ab7a | store | 4 | `[sp+4] +0x24` | `[[sp+4]+12]` |
| `mov` at 0x10ab7d | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10ab81 | store | 4 | `[sp+4] +0x28` | `?` |
| `mov` at 0x10ab84 | load | 4 | `[sp+4] +0xb0` | `-` |
| `mov` at 0x10ab8a | load | 4 | `[sp+4] +0xb0` | `-` |
| `mov` at 0x10ab99 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ab9e | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10aba5 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10aba8 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10abb0 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x10abbc | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10abc6 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10abcb | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10abd0 | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10abd3 | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x10abda | store | 4 | `[sp+4] +0x24` | `[[sp+4]+36]` |
| `mov` at 0x10abdd | store | 4 | `[sp+4] +0x28` | `[[sp+4]+40]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10abc1 | `_vgaScroll 0x1092f8` | `[sp+4]` | `[[sp+4]+12]` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ab92 | `([[sp+4]+176] & [[sp+4]+176]) != 0x0` | `([[sp+4]+176] & [[sp+4]+176]) == 0x0` |
| `jl` at 0x10aba3 | `? < [[sp+4]+28] (signed, 32-bit)` | `? >= [[sp+4]+28] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10abe9 | `eax` | `?` |

## `_vgaDelLines`

- `_vgaDelLines`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10abf8 to 0x10ac78.
  Established. The `_vgaScroll` call presents `[sp+4]`, `[[sp+4]+40]`, unknown, and `0x1` as its
  first four candidates (`call` at 0x10ac43). A subsequent four-byte store clears `[sp+4] +0x10`
  (`mov` at 0x10ac55). The four-byte store through `[sp+4] +0x28` has unknown value (`mov` at
  0x10ac11).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10abfe | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ac01 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10ac04 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10ac07 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ac0a | store | 4 | `[sp+4] +0x24` | `[[sp+4]+12]` |
| `mov` at 0x10ac0d | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10ac11 | store | 4 | `[sp+4] +0x28` | `?` |
| `mov` at 0x10ac14 | load | 4 | `[sp+4] +0xb0` | `-` |
| `mov` at 0x10ac23 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ac28 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10ac2f | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10ac32 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x10ac3e | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10ac48 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10ac4d | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10ac52 | store | 4 | `[sp+4] +0x14` | `?` |
| `mov` at 0x10ac55 | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x10ac5c | store | 4 | `[sp+4] +0x24` | `[[sp+4]+36]` |
| `mov` at 0x10ac5f | store | 4 | `[sp+4] +0x28` | `[[sp+4]+40]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ac43 | `_vgaScroll 0x1092f8` | `[sp+4]` | `[[sp+4]+40]` | `?` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10ac1c | `([[sp+4]+176] & [[sp+4]+176]) != 0x0` | `([[sp+4]+176] & [[sp+4]+176]) == 0x0` |
| `jle` at 0x10ac2d | `? <= [[sp+4]+28] (signed, 32-bit)` | `? > [[sp+4]+28] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ac6b | `eax` | `?` |

## `_vgaDelRightChars`

- `_vgaDelRightChars`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ac78 to 0x10ad08.
  Established. The first branch is taken when `([[sp+4]+16] + [sp+8]) < [[sp+4]+32] (signed,
  32-bit)` (`jl` at 0x10ac9b). The two-byte store sites retain unknown effective addresses and
  values (`mov` at 0x10acdc, `mov` at 0x10acea). The loop repeat predicate is `? < [[sp+4]+32]
  (signed, 32-bit)` (`jl` at 0x10ace8), leaving its first operand unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ac7e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ac81 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ac84 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x10ac91 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10ac96 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ac9d | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10aca0 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10acc8 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10accb | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x10acd8 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10acdc | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10ace3 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10acea | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10ac9b | `([[sp+4]+16] + [sp+8]) < [[sp+4]+32] (signed, 32-bit)` | `([[sp+4]+16] + [sp+8]) >= [[sp+4]+32] (signed, 32-bit)` |
| `jne` at 0x10acba | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `jl` at 0x10ace8 | `? < [[sp+4]+32] (signed, 32-bit)` | `? >= [[sp+4]+32] (signed, 32-bit)` |
| `jne` at 0x10acf1 | `? != 0xffffffff` | `? == 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10acfc | `eax` | `?` |

## `_vgaEscResponse`

- `_vgaEscResponse`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10a5b8 to 0x10a688.
  Established. The `_tyIoctl` call has leading candidates `[sp+4]`, `0x1a`, and `0x0` (`call` at
  0x10a5cf). The `_rngBufPut` site presents `[[sp+4]+16]`, `[[sp+4]+232]+244`, and `ret(_strlen)`
  (`call` at 0x10a654). The final `_selWakeupAll` site has candidates `[sp+4]+136` and `0x0` (`call`
  at 0x10a671). These values do not establish the callee effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10a5be | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a5c1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10a5c4 | load | 4 | `[sp+4] +0xe8` | `-` |
| `mov` at 0x10a5db | load | 4 | `[[sp+4]+232] +0x10` | `-` |
| `mov` at 0x10a5df | load | 4 | `[[sp+4]+232] +0xc` | `-` |
| `mov` at 0x10a638 | store | 1 | `[[sp+4]+232] +0xf4` | `0x0` |
| `mov` at 0x10a650 | load | 4 | `[sp+4] +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10a5cf | `_tyIoctl 0x16f748` | `[sp+4]` | `0x1a` | `0x0` | `ebx` | `esi` | `edi` |
| `call` at 0x10a5ef | `_sprintf 0x172dd8` | `[[sp+4]+232]+244` | `0x10a599` | `[[[sp+4]+232]+12]` | `[[[sp+4]+232]+16]` | `?` | `?` |
| `call` at 0x10a629 | `_sprintf 0x172dd8` | `[[sp+4]+232]+244` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a646 | `_strlen 0x17466c` | `[[sp+4]+232]+244` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a654 | `_rngBufPut 0x172858` | `[[sp+4]+16]` | `[[sp+4]+232]+244` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10a660 | `_semGive 0x1729d4` | `[sp+4]+20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10a671 | `_selWakeupAll 0x174b70` | `[sp+4]+136` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10a5d9 | `([sp+8] & [sp+8]) != 0x0` | `([sp+8] & [sp+8]) == 0x0` |
| `jne` at 0x10a60b | `[sp+8] != 0x1` | `[sp+8] == 0x1` |
| `jne` at 0x10a61b | `[sp+8] != 0x2` | `[sp+8] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10a67f | `eax` | `ret(_selWakeupAll)` |

## `_vgaSetAttrib`

- `_vgaSetAttrib`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10a6e8 to 0x10a8d8.
  Established. The initial byte load reads `[sp+4] +0x18` (`mov` at 0x10a6f0). The later byte store
  to that same resolved target has unknown value (`mov` at 0x10a8c8). The range branch reports `a
  flags condition (operands unknown)` as its taken predicate (`ja` at 0x10a712). No effects reached
  only through the unresolved selector are credited.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10a6ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a6f0 | load | 1 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10a708 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a718 | load | 4 | `(unknown)` | `-` |
| `jmp` at 0x10a71f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a858 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a864 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a874 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a87b | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10a888 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a894 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a8a5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10a8ac | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10a8ba | load | 4 | `[sp+4] +0xac` | `-` |
| `mov` at 0x10a8c8 | store | 1 | `[sp+4] +0x18` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x10a712 | `a flags condition (operands unknown)` | `not (a flags condition (operands unknown))` |
| `jle` at 0x10a862 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jg` at 0x10a86e | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jle` at 0x10a892 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jg` at 0x10a89e | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jle` at 0x10a8c2 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10a8d3 | `eax` | `[[sp+4]+172]` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x10a71f).

## `_vgaPutCursor`

- `_vgaPutCursor`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10aa48 to 0x10aab8.
  Established. On the fall-through predicate `not (ge flags condition (operands unknown))` (`jge` at
  0x10aa53), a four-byte store clears `[sp+4] +0x10` (`mov` at 0x10aa55). The final four-byte store
  through `[sp+4] +0x14` has unknown value (`mov` at 0x10aaac). The intervening signed branch is
  taken when `[[sp+4]+12] >= [[sp+4]+36] (signed, 32-bit)` (`jge` at 0x10aa81).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10aa4b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10aa4e | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10aa55 | store | 4 | `[sp+4] +0x10` | `0x0` |
| `mov` at 0x10aa68 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10aa6b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10aa72 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10aa76 | store | 4 | `[sp+4] +0x10` | `?` |
| `mov` at 0x10aa79 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10aa7c | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10aa83 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10aa88 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10aa8b | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10aa92 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10aa95 | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10aa98 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10aa9b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10aaa1 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10aaa6 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10aaac | store | 4 | `[sp+4] +0x14` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10aa53 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x10aa70 | `[[sp+4]+16] < [[sp+4]+32] (signed, 32-bit)` | `[[sp+4]+16] >= [[sp+4]+32] (signed, 32-bit)` |
| `jge` at 0x10aa81 | `[[sp+4]+12] >= [[sp+4]+36] (signed, 32-bit)` | `[[sp+4]+12] < [[sp+4]+36] (signed, 32-bit)` |
| `jl` at 0x10aa90 | `[[sp+4]+12] < [[sp+4]+40] (signed, 32-bit)` | `[[sp+4]+12] >= [[sp+4]+40] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10aab2 | `eax` | `?` |

## `_vgaSaveCurAttrib`

- `_vgaSaveCurAttrib`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10a688 to 0x10a6b8.
  Established. The four-byte stores include `[[sp+4]+44]` at `[sp+4] +0x34` (`mov` at 0x10a691) and
  `[[sp+4]+16]` at `[sp+4] +0x3c` (`mov` at 0x10a697). The byte store writes `[[sp+4]+24]:1` to
  `[sp+4] +0x40` (`mov` at 0x10a6a3). The record fields have unknown meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10a68b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a68e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10a691 | store | 4 | `[sp+4] +0x34` | `[[sp+4]+44]` |
| `mov` at 0x10a694 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10a697 | store | 4 | `[sp+4] +0x3c` | `[[sp+4]+16]` |
| `mov` at 0x10a69a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10a69d | store | 4 | `[sp+4] +0x38` | `[[sp+4]+12]` |
| `mov` at 0x10a6a0 | load | 1 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10a6a3 | store | 1 | `[sp+4] +0x40` | `[[sp+4]+24]:1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10a6a9 | `eax` | `[sp+4]` |

## `_vgaRestAttrib`

- `_vgaRestAttrib`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10a6b8 to 0x10a6e8.
  Established. The four-byte store to `[sp+4] +0x2c` has value `[[sp+4]+52]` (`mov` at 0x10a6c1).
  The separate four-byte store to `[sp+4] +0xc` has value `[[sp+4]+56]` (`mov` at 0x10a6cd). A byte
  store places `[[sp+4]+64]:1` at `[sp+4] +0x18` (`mov` at 0x10a6d3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10a6bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10a6be | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x10a6c1 | store | 4 | `[sp+4] +0x2c` | `[[sp+4]+52]` |
| `mov` at 0x10a6c4 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x10a6c7 | store | 4 | `[sp+4] +0x10` | `[[sp+4]+60]` |
| `mov` at 0x10a6ca | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x10a6cd | store | 4 | `[sp+4] +0xc` | `[[sp+4]+56]` |
| `mov` at 0x10a6d0 | load | 1 | `[sp+4] +0x40` | `-` |
| `mov` at 0x10a6d3 | store | 1 | `[sp+4] +0x18` | `[[sp+4]+64]:1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10a6d9 | `eax` | `[sp+4]` |

## `_enereset`

- `_enereset`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1829f0 to 0x182a10.
  Established. The `_eneReset` site has candidate zero `[sp+4]` (`call` at 0x1829f7), sourced from a
  four-byte load at `sp +0x4` (`mov` at 0x1829f3). The exit retains `ret(_eneReset)` (`ret` at
  0x182a02). No independent reset effect is established by this wrapper body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1829f3 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1829f7 | `_eneReset 0x1834d0` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x182a02 | `eax` | `ret(_eneReset)` |

## `_eneoutput`

- `_eneoutput`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1831b0 to 0x183200.
  Established. The `_ether_output` call presents `[sp+4]`, `[sp+8]`, `[sp+12]`, and `0x183200` as
  its first four candidates (`call` at 0x1831d6). The two-byte load through `[sp+4] +0x4` is
  resolved (`mov` at 0x1831b6), while the following indexed four-byte load has unknown effective
  address (`mov` at 0x1831bd). The fourth candidate does not establish a callback role or its execution.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1831b3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1831b6 | load | 2 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1831bd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1831ca | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1831ce | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1831d2 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1831d6 | `_ether_output 0x197df8` | `[sp+4]` | `[sp+8]` | `[sp+12]` | `0x183200` | `?` | `ebp` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1831f3 | `eax` | `ret(_ether_output)` |

## `_eneinit`

- `_eneinit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x182a10 to 0x182a40.
  Established. The two-byte store has an unknown effective address and stored value (`mov` at
  0x182a26); its prior two-byte load is also unresolved (`mov` at 0x182a1d). The sole exit
  reports `0x0` in `eax` (`ret` at 0x182a33), without establishing an initialization effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x182a13 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x182a16 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x182a1d | load | 2 | `(unknown)` | `-` |
| `mov` at 0x182a26 | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x182a33 | `eax` | `0x0` |
