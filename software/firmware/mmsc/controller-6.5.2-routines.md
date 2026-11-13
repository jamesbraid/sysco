# MMSC controller command and save paths in IRIX 6.5.2

Read from `6.5.2/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.2 carrier).
Image SHA-256: `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/controller-6.5.2-facts.txt) include all tool sections.

## `_elscCheckResponse`

- `_elscCheckResponse`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1207a0 to 0x120940.
  Established. The two `_strchr` calls have second candidates 0xd and 0x14, with first candidates
  unknown (`call` at 0x1207b6, `call` at 0x1207ef). Two byte-store sites write zero through
  expressions rendered as `ret(_strchr) +0x0` (`mov` at 0x1207b0, `mov` at 0x1207c2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1207a6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1207b0 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x1207c2 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `test` at 0x1207c5 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x1207dd | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1207b6 | `_strchr 0x180308` | `?` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x1207d4 | `_ffscMsg 0x11f0d0` | `0x120770` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1207e1 | `_elscProcessCommand 0x121080` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1207ef | `_strchr 0x180308` | `?` | `0x14` | `?` | `?` | `?` | `?` |
| `call` at 0x120802 | `_strlen 0x17476c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12080d | `_memmove 0x180290` | `[sp+8]` | `?` | `ret(_strlen)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1207c0 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x1207cc | `(([0x1dcdc8]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x1207fb | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x1207ff | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12081b | `eax` | `?` |

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120940 to 0x120ba0.
  Established. The `_msgWrite` candidates begin unknown and `[0x1dc880]` and `_msgRead` begins
  unknown and `[0x1dc86c]` (`call` at 0x120a12, `call` at 0x120aa3). Separate four-byte store sites
  write 1 and 5 to unresolved effective targets (`mov` at 0x1209a7, `mov` at 0x120a91). A later
  byte-zero store renders as `ret(_strpbrk) +0x0` (`mov` at 0x120af5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120949 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12094c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x120952 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x120961 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120980 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120993 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120997 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209a3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209a7 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x1209ae | load | 1 | `abs 0x1dcdc8` | `-` |
| `movsx` at 0x1209b7 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1209cb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209f2 | load | 4 | `abs 0x1dc880` | `-` |
| `mov` at 0x1209f8 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1209fe | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120a0d | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x120a1e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120a2d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120a40 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a44 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a4c | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x120a50 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x120a63 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120a72 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120a80 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a8d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a91 | store | 4 | `(unknown)` | `0x5` |
| `mov` at 0x120a98 | load | 4 | `abs 0x1dc86c` | `-` |
| `mov` at 0x120a9e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120ab3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120ab7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120abf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120ae5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120af5 | store | 1 | `ret(_strpbrk) +0x0` | `0x0` |
| `test` at 0x120af8 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x120b01 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b05 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b09 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120b0c | load | 1 | `[sp+4] +0x1ad158` | `-` |
| `cmp` at 0x120b23 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b2a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b2e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b44 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x120b51 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120b57 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x120b5d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120b60 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b67 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b6b | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12096d | `_ffscMsg 0x11f0d0` | `0x12081c` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x120985 | `_msgClear 0x126178` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12099b | `_sprintf 0x172ed8` | `?` | `0x120847` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1209c3 | `_ffscMsg 0x11f0d0` | `0x12084c` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1209dc | `_sprintf 0x172ed8` | `sp-84` | `0x12087d` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1209ea | `_logWriteLine 0x1252cc` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x120a12 | `_msgWrite 0x126a48` | `?` | `[0x1dc880]` | `?` | `?` | `?` | `?` |
| `call` at 0x120a31 | `_sprintf 0x172ed8` | `[sp+12]` | `0x120893` | `?` | `?` | `?` | `?` |
| `call` at 0x120a5b | `_ffscMsg 0x11f0d0` | `0x12089e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120a76 | `_sprintf 0x172ed8` | `[sp+12]` | `0x120893` | `?` | `?` | `?` | `?` |
| `call` at 0x120a85 | `_msgClear 0x126178` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120aa3 | `_msgRead 0x126438` | `?` | `[0x1dc86c]` | `?` | `?` | `?` | `?` |
| `call` at 0x120ac8 | `_ffscMsg 0x11f0d0` | `0x1208d0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120ae9 | `_strpbrk 0x1806a4` | `?` | `0x1208f8` | `?` | `?` | `?` | `?` |
| `call` at 0x120b1b | `_ffscMsg 0x11f0d0` | `0x1208fb` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120b3b | `_sprintf 0x172ed8` | `sp-84` | `0x120925` | `?` | `?` | `?` | `?` |
| `call` at 0x120b49 | `_logWriteLine 0x1252cc` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x120b6f | `_strcpy 0x18048c` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x120966 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `je` at 0x1209b5 | `(([0x1dcdc8]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x1209d0 | `? == 0x0` | `? != 0x0` |
| `je` at 0x120a1c | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x120a22 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `je` at 0x120a4a | `? == 0xb` | `? != 0xb` |
| `je` at 0x120a67 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jne` at 0x120aad | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x120abd | `? == 0xb` | `? != 0xb` |
| `je` at 0x120af3 | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` |
| `je` at 0x120aff | `(([0x1dcdc8]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x120b28 | `? == 0x0` | `? != 0x0` |
| `je` at 0x120b55 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120b7f | `eax` | `?` |

## `_elscInit`

- `_elscInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120ba0 to 0x120e90.
  Established. The three `_msgNew` sites have first candidates 0xa0, 0x50 and 0x50 respectively,
  each with third candidate 1 (`call` at 0x120d42, `call` at 0x120d76, `call` at 0x120daa). The
  `_nvramRead` and `_nvramWrite` calls begin with candidates 5 and `sp-68` (`call` at 0x120bc6,
  `call` at 0x120c27).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120ba9 | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x120bb0 | load | 4 | `abs 0x1ee394` | `-` |
| `mov` at 0x120bb5 | store | 4 | `sp -0x4c` | `[0x1ee394]` |
| `mov` at 0x120bb8 | load | 4 | `abs 0x1ee398` | `-` |
| `mov` at 0x120bbd | store | 4 | `sp -0x48` | `[0x1ee398]` |
| `mov` at 0x120be1 | store | 4 | `sp -0x7c` | `sp-68` |
| `mov` at 0x120be9 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120bef | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x120bf2 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120c0d | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x120c10 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x120c39 | store | 4 | `sp -0x60` | `sp-68` |
| `mov` at 0x120c3f | store | 4 | `sp -0x64` | `sp-84` |
| `mov` at 0x120c44 | store | 4 | `sp -0x68` | `0x0` |
| `mov` at 0x120c4b | store | 4 | `sp -0x6c` | `0x1dd276` |
| `mov` at 0x120c52 | store | 4 | `sp -0x70` | `0x0` |
| `mov` at 0x120c59 | store | 4 | `sp -0x74` | `0x1dd26c` |
| `mov` at 0x120c60 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x120c6a | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x120c72 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120c76 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120c7a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120c7e | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120c86 | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x120c89 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120c8c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120c8f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120c93 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120c99 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x120c9d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120ca4 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120caa | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x120cb6 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120cb9 | load | 4 | `sp -0x70` | `-` |
| `test` at 0x120cbc | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x120cc3 | load | 4 | `([sp-112] + [sp-96]) +0x4` | `-` |
| `mov` at 0x120cc7 | store | 4 | `(unknown)` | `[([sp-112] + [sp-96])+4]` |
| `mov` at 0x120cd0 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120cd8 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120cdb | load | 4 | `sp -0x70` | `-` |
| `test` at 0x120cde | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x120cec | load | 4 | `sp -0x6c` | `-` |
| `mov` at 0x120cf8 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x120d05 | load | 4 | `sp -0x6c` | `-` |
| `cmp` at 0x120d11 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120d18 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x120d26 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120d32 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120d38 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120d4a | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x120d52 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120d5d | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120d69 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120d6f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120d7e | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x120d86 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120d91 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120d9d | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120da3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120db2 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `mov` at 0x120df4 | load | 4 | `sp -0x58` | `-` |
| `cmp` at 0x120df7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120dfd | store | 4 | `(unknown)` | `[sp-88]` |
| `mov` at 0x120e01 | store | 4 | `sp -0x5c` | `0x1` |
| `add` at 0x120e0b | load | 4 | `sp -0x68` | `-` |
| `add` at 0x120e0b | store | 4 | `sp -0x68` | `[sp-104]+16` |
| `add` at 0x120e0f | load | 4 | `sp -0x6c` | `-` |
| `add` at 0x120e0f | store | 4 | `sp -0x6c` | `[sp-108]+60` |
| `add` at 0x120e13 | load | 4 | `sp -0x70` | `-` |
| `add` at 0x120e13 | store | 4 | `sp -0x70` | `[sp-112]+32` |
| `add` at 0x120e17 | load | 4 | `sp -0x74` | `-` |
| `add` at 0x120e17 | store | 4 | `sp -0x74` | `[sp-116]+60` |
| `cmp` at 0x120e25 | load | 4 | `sp -0x5c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120bc6 | `_nvramRead 0x1270f8` | `0x5` | `sp-68` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120bd5 | `_bzero 0x172778` | `sp-68` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x120c05 | `_strncpy 0x180660` | `?` | `0x120b80` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120c27 | `_nvramWrite 0x127288` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x120cae | `_bzero 0x172778` | `[sp-116]` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x120cf0 | `_strncpy 0x180660` | `[sp-108]` | `([sp-112] + [sp-96])+8` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120d09 | `_strcpy 0x18048c` | `[sp-108]` | `0x120b80` | `?` | `?` | `?` | `?` |
| `call` at 0x120d2a | `_sprintf 0x172ed8` | `[sp-100]` | `0x120b85` | `?` | `?` | `?` | `?` |
| `call` at 0x120d42 | `_msgNew 0x126228` | `0xa0` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120d61 | `_sprintf 0x172ed8` | `[sp-100]` | `0x120b8c` | `?` | `?` | `?` | `?` |
| `call` at 0x120d76 | `_msgNew 0x126228` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120d95 | `_sprintf 0x172ed8` | `[sp-100]` | `0x120b93` | `?` | `?` | `?` | `?` |
| `call` at 0x120daa | `_msgNew 0x126228` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120dd7 | `_elscSetup 0x121a90` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120de8 | `_elscQueryModuleNum 0x121500` | `?` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x120e2b | `_elscUpdateModuleNums 0x121c00` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120bd0 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x120c1f | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120cc1 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x120ce3 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) != 0x0` |
| `jl` at 0x120d16 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x120d50 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `je` at 0x120d84 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `jne` at 0x120db8 | `(ret(_msgNew) & ret(_msgNew)) != 0x0` | `(ret(_msgNew) & ret(_msgNew)) == 0x0` |
| `jne` at 0x120de1 | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x120df2 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x120dfb | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x120e1f | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120e29 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120e3e | `eax` | `?` |

## `_elscPrintMsg`

- `_elscPrintMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120e90 to 0x121080.
  Established. The two `_userSendCommand` calls receive candidates beginning `[0x1dd09c]`, `sp-248`,
  zero and `[0x1dd0a0]`, `sp-736`, zero respectively (`call` at 0x120ee3, `call` at 0x120f6b). The
  `_routeDoAdminCommand` calls later in each region have first candidates `sp-492` and `sp-980`
  (`call` at 0x120f36, `call` at 0x120fa6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120e9c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120e9f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120ea2 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120eaa | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120eb6 | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x120eb9 | load | 4 | `abs 0x1f0d0c` | `-` |
| `mov` at 0x120ec2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120edd | load | 4 | `abs 0x1dd09c` | `-` |
| `mov` at 0x120f10 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120f19 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120f1d | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120f3e | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x120f41 | load | 4 | `abs 0x1f0d0c` | `-` |
| `mov` at 0x120f4a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120f65 | load | 4 | `abs 0x1dd0a0` | `-` |
| `mov` at 0x120f80 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x120f89 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120f8d | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120eae | `_logWriteLine 0x1252cc` | `[[sp+4]+44]` | `[sp+12]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120ed2 | `_sprintf 0x172ed8` | `sp-248` | `0x120e3f` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x120ee3 | `_userSendCommand 0x12cb48` | `[0x1dd09c]` | `sp-248` | `0x0` | `?` | `?` | `?` |
| `call` at 0x120ef5 | `_ffscMsg 0x11f0d0` | `0x120e49` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x120f2d | `_sprintf 0x172ed8` | `sp-492` | `0x120e53` | `[[sp+4]+20]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120f36 | `_routeDoAdminCommand 0x1299a4` | `sp-492` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120f5a | `_sprintf 0x172ed8` | `sp-736` | `0x120e3f` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x120f6b | `_userSendCommand 0x12cb48` | `[0x1dd0a0]` | `sp-736` | `0x0` | `?` | `?` | `?` |
| `call` at 0x120f9d | `_sprintf 0x172ed8` | `sp-980` | `0x120e69` | `[[sp+4]+24]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120fa6 | `_routeDoAdminCommand 0x1299a4` | `sp-980` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120ea7 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x120ebf | `[0x1f0d0c] != [[sp+4]+20]` | `[0x1f0d0c] == [[sp+4]+20]` |
| `je` at 0x120eed | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120f16 | `[[sp+4]+20] == 0xffffffff` | `[[sp+4]+20] != 0xffffffff` |
| `jne` at 0x120f47 | `[0x1f0d0c] != [[sp+4]+24]` | `[0x1f0d0c] == [[sp+4]+24]` |
| `je` at 0x120f72 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120f86 | `[[sp+4]+24] == 0xffffffff` | `[[sp+4]+24] != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120fb9 | `eax` | `?` |

## `_elscProcessCommand`

- `_elscProcessCommand`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121080 to 0x121300.
  Established. The `_strtok_r` candidates begin `[sp+8]`, 0x120fd4 and `sp-8` (`call` at 0x1210c1).
  A later four-byte store writes `[sp-92]` through `[sp+4] +0x10` (`mov` at 0x1211cc). The
  `_elscPrintMsg` call has candidates beginning `[sp+4]`, `sp-88` and `[sp-8]` (`call` at 0x121216).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12108c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12108f | load | 4 | `sp +0x8` | `-` |
| `test` at 0x121092 | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x12109c | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1210b0 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x1210cf | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12111d | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121131 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x121168 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x12116f | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121188 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1211a9 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1211bd | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x1211c0 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1211c9 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1211cc | store | 4 | `[sp+4] +0x10` | `[sp-92]` |
| `mov` at 0x1211f2 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1211f9 | load | 4 | `abs 0x1f0d0c` | `-` |
| `mov` at 0x121210 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x121221 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12123c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x121244 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1210a8 | `_ffscMsg 0x11f0d0` | `0x120fba` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` |
| `call` at 0x1210c1 | `_strtok_r 0x180808` | `[sp+8]` | `0x120fd4` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x1210db | `_ffscMsg 0x11f0d0` | `0x120fd7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1210f1 | `_ffscConvertToUpperCase 0x125718` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1210ff | `_strcmp 0x180330` | `ret(_strtok_r)` | `0x120ff1` | `?` | `?` | `?` | `?` |
| `call` at 0x121111 | `_strcmp 0x180330` | `ret(_strtok_r)` | `0x120ff5` | `?` | `?` | `?` | `?` |
| `call` at 0x121129 | `_ffscMsg 0x11f0d0` | `0x120ff8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121136 | `_elscUpdateDisplay 0x121b90` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x121148 | `_strncmp 0x176e68` | `ret(_strtok_r)` | `0x121024` | `0x4` | `?` | `?` | `?` |
| `call` at 0x12115c | `_strncmp 0x176e68` | `ret(_strtok_r)` | `0x121029` | `0x3` | `?` | `?` | `?` |
| `call` at 0x12117f | `_sprintf 0x172ed8` | `sp-88` | `0x12102d` | `?` | `?` | `?` | `?` |
| `call` at 0x12118c | `_logWriteLine 0x1252cc` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x121195 | `_elscSetup 0x121a90` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1211ad | `_elscQueryModuleNum 0x121500` | `[[sp+4]+12]` | `sp-92` | `?` | `?` | `?` | `?` |
| `call` at 0x1211cf | `_elscUpdateModuleNums 0x121c00` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1211e6 | `_strcmp 0x180330` | `ret(_strtok_r)` | `0x121046` | `?` | `?` | `?` | `?` |
| `call` at 0x121208 | `_sprintf 0x172ed8` | `sp-88` | `0x12104a` | `[0x1f0d0c]` | `?` | `?` | `?` |
| `call` at 0x121216 | `_elscPrintMsg 0x120e90` | `[sp+4]` | `sp-88` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x121234 | `_sprintf 0x172ed8` | `sp-348` | `0x121051` | `?` | `[sp+8]` | `?` | `?` |
| `call` at 0x121248 | `_logWriteLine 0x1252cc` | `[[sp+4]+44]` | `sp-348` | `?` | `?` | `?` | `?` |
| `call` at 0x121251 | `_ffscMsg 0x11f0d0` | `sp-348` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121099 | `(([0x1dcdc8]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x1210cd | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x121109 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x12111b | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x121152 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x121166 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x12116d | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x12119f | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x1211b7 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x1211c3 | `[sp-92] == [[sp+4]+16]` | `[sp-92] != [[sp+4]+16]` |
| `jne` at 0x1211f0 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x121241 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12126e | `eax` | `?` |

## `_elscProcessInput`

- `_elscProcessInput`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121300 to 0x121500.
  Established. The first `je` selects the `_msgClear` route for `[[sp+4]+4] == 0x1` (`je` at
  0x121311, `call` at 0x121334). A following four-byte store places 4 through `[[sp+4]+48] +0x1c`,
  before `_msgRead` receives candidates beginning `[[sp+4]+48]` and `[0x1dc868]` (`mov` at 0x12133f,
  `call` at 0x121350).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121308 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12130b | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x121313 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x121330 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x12133c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x12133f | store | 4 | `[[sp+4]+48] +0x1c` | `0x4` |
| `mov` at 0x121346 | load | 4 | `abs 0x1dc868` | `-` |
| `mov` at 0x12134c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121370 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121373 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x12137b | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x12137f | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121390 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121398 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x12139e | store | 4 | `[[sp+4]+48] +0xc` | `[[[sp+4]+48]+12]-2` |
| `mov` at 0x1213b0 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x1213b8 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1213bb | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1213c2 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1213c9 | load | 4 | `abs 0x1f0d0c` | `-` |
| `mov` at 0x1213e0 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1213e4 | store | 4 | `[[sp+4]+48] +0xc` | `?` |
| `mov` at 0x1213e7 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1213ea | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1213ed | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1213f0 | store | 1 | `[[[sp+4]+48]+12] +0x0` | `0x0` |
| `mov` at 0x1213f3 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1213f6 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1213f9 | load | 1 | `[[[sp+4]+48]+8] +0x0` | `-` |
| `mov` at 0x1213ff | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x121402 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121405 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12140c | load | 4 | `abs 0x1f0d0c` | `-` |
| `mov` at 0x121423 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121426 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121440 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121448 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121460 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121464 | load | 4 | `[sp+4] +0x30` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12131c | `_ffscMsg 0x11f0d0` | `0x12126f` | `[[sp+4]+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x121334 | `_msgClear 0x126178` | `[[sp+4]+48]` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x121350 | `_msgRead 0x126438` | `[[sp+4]+48]` | `[0x1dc868]` | `?` | `?` | `?` | `?` |
| `call` at 0x12141b | `_sprintf 0x172ed8` | `sp-16` | `0x1212cc` | `[0x1f0d0c]` | `?` | `?` | `?` |
| `call` at 0x12142c | `_elscPrintMsg 0x120e90` | `[sp+4]` | `sp-16` | `[[[sp+4]+48]+8]` | `?` | `?` | `?` |
| `call` at 0x12144e | `_elscProcessCommand 0x121080` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12146d | `_ffscMsg 0x11f0d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121311 | `[[sp+4]+4] == 0x1` | `[[sp+4]+4] != 0x1` |
| `je` at 0x12135a | `(ret(_msgRead) & ret(_msgRead)) == 0x0` | `(ret(_msgRead) & ret(_msgRead)) != 0x0` |
| `jne` at 0x121379 | `[[[sp+4]+48]+28] != 0xa` | `[[[sp+4]+48]+28] == 0xa` |
| `jne` at 0x121396 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |
| `je` at 0x1213b6 | `[[[sp+4]+48]+28] == 0x17` | `[[[sp+4]+48]+28] != 0x17` |
| `jne` at 0x1213c0 | `[[[sp+4]+48]+12] != [[[sp+4]+48]+8]` | `[[[sp+4]+48]+12] == [[[sp+4]+48]+8]` |
| `je` at 0x1213fd | `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14` | `([[[[sp+4]+48]+8]]:1 & 0xff) != 0x14` |
| `jne` at 0x121446 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12147c | `eax` | `?` |

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121500 to 0x121680.
  Established. The explicit four-byte store through `[sp+8] +0x0` carries 0xffffffff (`mov` at
  0x121513). `_strtol` later receives first candidates `sp-85`, `sp-92` and 0x10 (`call` at
  0x1215aa). The subsequent `jbe` is taken for `ret(_strtol) <= 0xff (unsigned, 32-bit)` and falls
  through for a greater result (`jbe` at 0x1215c3).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121509 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12150c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121513 | store | 4 | `[sp+8] +0x0` | `0xffffffff` |
| `movsx` at 0x121539 | load | 1 | `(unknown)` | `-` |
| `or` at 0x12154b | load | 1 | `(unknown)` | `-` |
| `or` at 0x12154b | store | 1 | `(unknown)` | `?` |
| `movsx` at 0x12157c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1215b4 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1215b7 | load | 1 | `[sp-92] +0x0` | `-` |
| `movsx` at 0x1215cd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1215fd | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121523 | `_elscDoAdminCommand 0x120940` | `[sp+4]` | `0x12147d` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x121546 | `_ffscMsg 0x11f0d0` | `0x121481` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121568 | `_strncmp 0x176e68` | `sp-88` | `0x1214ac` | `0x2` | `?` | `?` | `?` |
| `call` at 0x12158a | `_ffscMsg 0x11f0d0` | `0x1214af` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x1215aa | `_strtol 0x17fea8` | `sp-85` | `sp-92` | `0x10` | `?` | `?` | `?` |
| `call` at 0x1215db | `_ffscMsg 0x11f0d0` | `0x1214d1` | `?` | `sp-85` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121511 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x12152d | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x121572 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x1215bb | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) != 0x0` | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x1215c3 | `ret(_strtol) <= 0xff (unsigned, 32-bit)` | `ret(_strtol) > 0xff (unsigned, 32-bit)` |
| `jne` at 0x1215f2 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x1215fb | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12160a | `eax` | `?` |

## `_elscSendCommand`

- `_elscSendCommand`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121680 to 0x121820.
  Established. Distinct four-byte store sites carry 0xb, 1 and 5 to unknown targets (`mov` at
  0x1216cb, `mov` at 0x121707, `mov` at 0x1217a4). `_msgWrite` has an unknown first candidate and
  `[0x1dc880]` as its second (`call` at 0x121736). The recorded EAX exit is zero (`ret` at
  0x1217dd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121689 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12168c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x121692 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1216a1 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1216ae | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216ba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216be | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216c7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216cb | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1216e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216f3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216f7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121703 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121707 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x12170e | load | 1 | `abs 0x1dcdc8` | `-` |
| `movsx` at 0x121717 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12172b | load | 4 | `abs 0x1dc880` | `-` |
| `mov` at 0x121731 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121747 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12174b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121760 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121764 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12176c | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x121770 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x121788 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12178c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1217a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1217a4 | store | 4 | `(unknown)` | `0x5` |
| `cmp` at 0x1217ab | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1217c5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1216a6 | `_msgClear 0x126178` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1216c2 | `_sprintf 0x172ed8` | `?` | `0x12160b` | `?` | `?` | `?` | `?` |
| `call` at 0x1216e5 | `_msgClear 0x126178` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1216fb | `_sprintf 0x172ed8` | `?` | `0x120847` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121723 | `_ffscMsg 0x11f0d0` | `0x121617` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x121736 | `_msgWrite 0x126a48` | `?` | `[0x1dc880]` | `?` | `?` | `?` | `?` |
| `call` at 0x12174f | `_strcpy 0x18048c` | `?` | `0x120893` | `?` | `?` | `?` | `?` |
| `call` at 0x12177b | `_ffscMsg 0x11f0d0` | `0x12163a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121790 | `_strcpy 0x18048c` | `?` | `0x120893` | `?` | `?` | `?` | `?` |
| `call` at 0x1217bc | `_sprintf 0x172ed8` | `sp-124` | `0x121666` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1217ca | `_logWriteLine 0x1252cc` | `?` | `sp-124` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1216b3 | `? == 0x1` | `? != 0x1` |
| `je` at 0x121715 | `(([0x1dcdc8]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x121740 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x12176a | `? == 0xb` | `? != 0xb` |
| `je` at 0x1217b0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1217dd | `eax` | `0x0` |

## `_elscSendPassword`

- `_elscSendPassword`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121820 to 0x1218e0.
  Established. The `_elscDoAdminCommand` call begins with candidates `[sp+4]`, `sp-88` and `sp-172`
  (`call` at 0x121883); its nonzero result takes `jne` to the all-ones EAX load, while zero falls
  through to `_strncmp` with first candidates `sp-172`, 0x1214ac and 2 (`jne` at 0x12188d, `mov`
  at 0x1218bb, `call` at 0x121897).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12182b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x121838 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x121841 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121853 | `_sprintf 0x172ed8` | `sp-88` | `0x1217de` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x12186f | `_sprintf 0x172ed8` | `sp-88` | `0x1217e2` | `?` | `ebx` | `esi` | `?` |
| `call` at 0x121883 | `_elscDoAdminCommand 0x120940` | `[sp+4]` | `sp-88` | `sp-172` | `?` | `?` | `?` |
| `call` at 0x121897 | `_strncmp 0x176e68` | `sp-172` | `0x1214ac` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1218b6 | `_ffscMsg 0x11f0d0` | `0x1217e9` | `sp-172` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12183f | `? == 0x0` | `? != 0x0` |
| `jg` at 0x121848 | `[sp+8] > 0x1f3 (signed, 32-bit)` | `[sp+8] <= 0x1f3 (signed, 32-bit)` |
| `jne` at 0x12188d | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x1218a1 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1218cb | `eax` | `?` |

## `_elscSetPassword`

- `_elscSetPassword`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1218e0 to 0x121a90.
  Established. Calls `_elscSendPassword` with candidates beginning `[sp+4]` and 0x3e8 (`call` at
  0x121969). The later `_elscDoAdminCommand` first candidates are `[sp+4]`, `sp-88` and zero (`call`
  at 0x121991). The two four-byte store sites carrying 0xb both have unresolved effective targets
  (`mov` at 0x12194a, `mov` at 0x121a04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1218e9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1218ec | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1218f2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121901 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x121916 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121920 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121923 | load | 1 | `[sp+8] +0x0` | `-` |
| `cmp` at 0x12192d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121939 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12193d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121946 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12194a | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x121965 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121975 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12198d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1219a2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1219a6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1219b6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1219c5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1219c9 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1219cf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1219f3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1219f7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121a00 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121a04 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121906 | `_msgClear 0x126178` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x121941 | `_sprintf 0x172ed8` | `?` | `0x12160b` | `?` | `?` | `?` | `?` |
| `call` at 0x121969 | `_elscSendPassword 0x121820` | `[sp+4]` | `0x3e8` | `?` | `?` | `?` | `?` |
| `call` at 0x121982 | `_sprintf 0x172ed8` | `sp-88` | `0x1218cc` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121991 | `_elscDoAdminCommand 0x120940` | `[sp+4]` | `sp-88` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1219aa | `_strcmp 0x180330` | `?` | `0x1214ac` | `?` | `?` | `?` | `?` |
| `call` at 0x1219bb | `_msgClear 0x126178` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1219dc | `_strncpy 0x180660` | `?` | `[sp+8]` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1219e4 | `_elscUpdateSaveInfo 0x121c80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1219fb | `_sprintf 0x172ed8` | `?` | `0x1218d7` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121910 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x12191a | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `je` at 0x121927 | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) == 0x0` | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) != 0x0` |
| `je` at 0x121932 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x121973 | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` |
| `jne` at 0x12199b | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x1219b4 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121a16 | `eax` | `?` |

## `_elscSetup`

- `_elscSetup`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121a90 to 0x121b90.
  Established. The first `_elscDoAdminCommand` call has candidates beginning `[[sp+4]+12]`, 0x121a2b
  and `sp-88` (`call` at 0x121ac5); its zero result takes `je` to `_strncmp`, while nonzero
  falls through to `_ffscMsg` and a four-byte store of unknown value through `[sp+4] +0x0` (`je`
  at 0x121acf, `call` at 0x121af8, `call` at 0x121add, `mov` at 0x121b1a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121a98 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x121a9b | load | 1 | `abs 0x1dcdc8` | `-` |
| `mov` at 0x121aa4 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121ac1 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x121ad1 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121b05 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121b16 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x121b1a | store | 4 | `[sp+4] +0x0` | `?` |
| `mov` at 0x121b30 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x121b3f | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x121b48 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x121b4c | store | 4 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121ab0 | `_ffscMsg 0x11f0d0` | `0x121a17` | `?` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x121ac5 | `_elscDoAdminCommand 0x120940` | `[[sp+4]+12]` | `0x121a2b` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x121add | `_ffscMsg 0x11f0d0` | `0x121a31` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121af8 | `_strncmp 0x176e68` | `sp-88` | `0x1214ac` | `0x2` | `?` | `?` | `?` |
| `call` at 0x121b11 | `_ffscMsg 0x11f0d0` | `0x121a5c` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x121b43 | `_elscDoAdminCommand 0x120940` | `[[sp+4]+12]` | `0x121a80` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121aa2 | `(([0x1dcdc8]:1 & 0xff) & 0x2) == 0x0` | `(([0x1dcdc8]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x121acf | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x121b02 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x121b36 | `[[sp+4]+4] != 0x1` | `[[sp+4]+4] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121b58 | `eax` | `?` |

## `_elscUpdateDisplay`

- `_elscUpdateDisplay`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121b90 to 0x121c00.
  Established. The `_bzero` call begins with candidates `[sp+4]+28` and 0xa (`call` at 0x121ba5).
  `_strncpy` has first candidates `[sp+4]+28`, `[sp+8]` and 0xa (`call` at 0x121bb5). The later byte
  read renders as `[sp+4] +0x24` (`mov` at 0x121bc4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121b99 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121b9c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121bbd | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x121bc4 | load | 1 | `[sp+4] +0x24` | `-` |
| `mov` at 0x121bde | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121ba5 | `_bzero 0x172778` | `[sp+4]+28` | `0xa` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121bb5 | `_strncpy 0x180660` | `[sp+4]+28` | `[sp+8]` | `0xa` | `?` | `?` | `?` |
| `call` at 0x121bd5 | `_sprintf 0x172ed8` | `sp-84` | `0x121b59` | `[sp+4]+28` | `?` | `?` | `?` |
| `call` at 0x121be2 | `_logWriteLine 0x1252cc` | `[[sp+4]+44]` | `sp-84` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121baf | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x121bc2 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121bf0 | `eax` | `?` |

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121c00 to 0x121c80.
  Established. The `rep movsd` site includes a four-byte read at absolute 0x1f0d00 and a four-byte
  store with unknown target and value (`rep movsd` at 0x121c16). The `_addrdUpdateLocalIdentity`
  call receives first candidate `sp-32`, and its result is retained in EAX at return (`call` at
  0x121c39, `ret` at 0x121c46).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x121c16 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x121c16 | load | 4 | `abs 0x1f0d00` | `-` |
| `mov` at 0x121c24 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121c28 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121c39 | `_addrdUpdateLocalIdentity 0x10dbf0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x121c33 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121c46 | `eax` | `ret(_addrdUpdateLocalIdentity)` |

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121c80 to 0x121d30.
  Established. The `_nvramWrite` candidates begin 5 and `sp-68` (`call` at 0x121cf2); its nonzero
  result takes `jne` to `_ffscMsg`, with zero falling through to the exit (`jne` at
  0x121cfc, `call` at 0x121d15). An earlier four-byte store writes value 3 to an unresolved target
  (`mov` at 0x121cc2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121c99 | store | 4 | `sp -0x48` | `sp-68` |
| `mov` at 0x121c9c | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x121c9f | store | 4 | `sp -0x54` | `0x1dd276` |
| `mov` at 0x121cad | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121cb3 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121cb5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121cbb | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x121cbe | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121cc2 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x121ccc | load | 4 | `sp -0x54` | `-` |
| `add` at 0x121cdf | load | 4 | `sp -0x54` | `-` |
| `add` at 0x121cdf | store | 4 | `sp -0x54` | `[sp-84]+60` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121c8f | `_bzero 0x172778` | `sp-68` | `0x40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121cd4 | `_strncpy 0x180660` | `?` | `?` | `0x4` | `?` | `?` | `?` |
| `call` at 0x121cf2 | `_nvramWrite 0x127288` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x121d15 | `_ffscMsg 0x11f0d0` | `0x121c47` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x121cea | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x121cfc | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121d28 | `eax` | `?` |

## `_routeDoAdminCommand`

- `_routeDoAdminCommand`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1299a4 to 0x1299c4.
  Established. The `_ProcessCommand` candidates begin `[sp+4]` and `[0x1dd0ac]` (`call` at
  0x1299b1); its result remains the recorded EAX exit value (`ret` at 0x1299b9). The prior
  absolute four-byte read is at 0x1dd0ac (`mov` at 0x1299aa).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1299a7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1299aa | load | 4 | `abs 0x1dd0ac` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1299b1 | `_ProcessCommand 0x129e84` | `[sp+4]` | `[0x1dd0ac]` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1299b9 | `eax` | `ret(_ProcessCommand)` |
