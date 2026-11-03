# MMSC ELSC client in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/elsc-client-sg0002378-facts.txt) include all tool sections.

## `_elscInit`

- `_elscInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120a80 to 0x120d70.
  Established. The call to `_nvramRead` has leading candidates `0x5` and `sp-68` (`call` at
  0x120aa6). The three `_msgNew` sites have candidate zero `0xa0` (`call` at 0x120c22), `0x50`
  (`call` at 0x120c56), and `0x50` (`call` at 0x120c8a), respectively. Their candidate one values
  are unknown. The record identities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120a89 | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x120a90 | load | 4 | `abs 0x1ee28c` | `-` |
| `mov` at 0x120a95 | store | 4 | `sp -0x4c` | `[0x1ee28c]` |
| `mov` at 0x120a98 | load | 4 | `abs 0x1ee290` | `-` |
| `mov` at 0x120a9d | store | 4 | `sp -0x48` | `[0x1ee290]` |
| `mov` at 0x120ac1 | store | 4 | `sp -0x7c` | `sp-68` |
| `mov` at 0x120ac9 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120acf | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x120ad2 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120aed | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x120af0 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x120b19 | store | 4 | `sp -0x60` | `sp-68` |
| `mov` at 0x120b1f | store | 4 | `sp -0x64` | `sp-84` |
| `mov` at 0x120b24 | store | 4 | `sp -0x68` | `0x0` |
| `mov` at 0x120b2b | store | 4 | `sp -0x6c` | `0x1dd16e` |
| `mov` at 0x120b32 | store | 4 | `sp -0x70` | `0x0` |
| `mov` at 0x120b39 | store | 4 | `sp -0x74` | `0x1dd164` |
| `mov` at 0x120b40 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x120b4a | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x120b52 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b56 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120b5a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120b5e | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120b66 | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x120b69 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120b6c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b6f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120b73 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120b79 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x120b7d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120b84 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120b8a | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x120b96 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120b99 | load | 4 | `sp -0x70` | `-` |
| `test` at 0x120b9c | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x120ba3 | load | 4 | `([sp-112] + [sp-96]) +0x4` | `-` |
| `mov` at 0x120ba7 | store | 4 | `(unknown)` | `[([sp-112] + [sp-96])+4]` |
| `mov` at 0x120bb0 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120bb8 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120bbb | load | 4 | `sp -0x70` | `-` |
| `test` at 0x120bbe | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x120bcc | load | 4 | `sp -0x6c` | `-` |
| `mov` at 0x120bd8 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x120be5 | load | 4 | `sp -0x6c` | `-` |
| `cmp` at 0x120bf1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120bf8 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x120c06 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120c12 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120c18 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120c2a | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x120c32 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120c3d | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120c49 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120c4f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120c5e | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x120c66 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120c71 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120c7d | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120c83 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120c92 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `mov` at 0x120cd4 | load | 4 | `sp -0x58` | `-` |
| `cmp` at 0x120cd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120cdd | store | 4 | `(unknown)` | `[sp-88]` |
| `mov` at 0x120ce1 | store | 4 | `sp -0x5c` | `0x1` |
| `add` at 0x120ceb | load | 4 | `sp -0x68` | `-` |
| `add` at 0x120ceb | store | 4 | `sp -0x68` | `[sp-104]+16` |
| `add` at 0x120cef | load | 4 | `sp -0x6c` | `-` |
| `add` at 0x120cef | store | 4 | `sp -0x6c` | `[sp-108]+60` |
| `add` at 0x120cf3 | load | 4 | `sp -0x70` | `-` |
| `add` at 0x120cf3 | store | 4 | `sp -0x70` | `[sp-112]+32` |
| `add` at 0x120cf7 | load | 4 | `sp -0x74` | `-` |
| `add` at 0x120cf7 | store | 4 | `sp -0x74` | `[sp-116]+60` |
| `cmp` at 0x120d05 | load | 4 | `sp -0x5c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120aa6 | `_nvramRead 0x127108` | `0x5` | `sp-68` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120ab5 | `_bzero 0x172678` | `sp-68` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x120ae5 | `_strncpy 0x180560` | `?` | `0x120a60` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120b07 | `_nvramWrite 0x127298` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x120b8e | `_bzero 0x172678` | `[sp-116]` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x120bd0 | `_strncpy 0x180560` | `[sp-108]` | `([sp-112] + [sp-96])+8` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120be9 | `_strcpy 0x18038c` | `[sp-108]` | `0x120a60` | `?` | `?` | `?` | `?` |
| `call` at 0x120c0a | `_sprintf 0x172dd8` | `[sp-100]` | `0x120a65` | `?` | `?` | `?` | `?` |
| `call` at 0x120c22 | `_msgNew 0x126108` | `0xa0` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120c41 | `_sprintf 0x172dd8` | `[sp-100]` | `0x120a6c` | `?` | `?` | `?` | `?` |
| `call` at 0x120c56 | `_msgNew 0x126108` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120c75 | `_sprintf 0x172dd8` | `[sp-100]` | `0x120a73` | `?` | `?` | `?` | `?` |
| `call` at 0x120c8a | `_msgNew 0x126108` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120cb7 | `_elscSetup 0x121970` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120cc8 | `_elscQueryModuleNum 0x1213e0` | `?` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x120d0b | `_elscUpdateModuleNums 0x121ae0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120ab0 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x120aff | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120ba1 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x120bc3 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) != 0x0` |
| `jl` at 0x120bf6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x120c30 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `je` at 0x120c64 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `jne` at 0x120c98 | `(ret(_msgNew) & ret(_msgNew)) != 0x0` | `(ret(_msgNew) & ret(_msgNew)) == 0x0` |
| `jne` at 0x120cc1 | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x120cd2 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x120cdb | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x120cff | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120d09 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120d1e | `eax` | `?` |

## `_elscSetup`

- `_elscSetup`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121970 to 0x121a70.
  Established. One `_elscDoAdminCommand` site has leading candidates `[[sp+4]+12]`, `0x12190b`, and
  `sp-88` (`call` at 0x1219a5). The `_strncmp` site uses `sp-88`, `0x12138c`, and `0x2` (`call` at
  0x1219d8). Both four-byte stores to `[sp+4] +0x0` have unknown values (`mov` at 0x1219fa, `mov` at
  0x121a2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121978 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x12197b | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x121984 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1219a1 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1219b1 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1219e5 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1219f6 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1219fa | store | 4 | `[sp+4] +0x0` | `?` |
| `mov` at 0x121a10 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x121a1f | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x121a28 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x121a2c | store | 4 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121990 | `_ffscMsg 0x11f040` | `0x1218f7` | `?` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x1219a5 | `_elscDoAdminCommand 0x120820` | `[[sp+4]+12]` | `0x12190b` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x1219bd | `_ffscMsg 0x11f040` | `0x121911` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1219d8 | `_strncmp 0x176d68` | `sp-88` | `0x12138c` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1219f1 | `_ffscMsg 0x11f040` | `0x12193c` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x121a23 | `_elscDoAdminCommand 0x120820` | `[[sp+4]+12]` | `0x121960` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121982 | `(([0x1dccc0]:1 & 0xff) & 0x2) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x1219af | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x1219e2 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x121a16 | `[[sp+4]+4] != 0x1` | `[[sp+4]+4] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121a38 | `eax` | `?` |

## `_elscCheckResponse`

- `_elscCheckResponse`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120680 to 0x120820.
  Established. The `_strchr` sites include candidate one `0xd` (`call` at 0x120696) and `0x14`
  (`call` at 0x1206cf). A byte store through `ret(_strchr) +0x0` writes zero (`mov` at 0x120690).
  The later byte store writes zero through the same symbolic target (`mov` at 0x1206a2), reached on
  the fall-through of the zero-result branch (`je` at 0x1206a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120686 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120690 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x1206a2 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `test` at 0x1206a5 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x1206bd | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120696 | `_strchr 0x180208` | `?` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x1206b4 | `_ffscMsg 0x11f040` | `0x120650` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1206c1 | `_elscProcessCommand 0x120f60` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1206cf | `_strchr 0x180208` | `?` | `0x14` | `?` | `?` | `?` | `?` |
| `call` at 0x1206e2 | `_strlen 0x17466c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1206ed | `_memmove 0x180190` | `[sp+8]` | `?` | `ret(_strlen)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1206a0 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x1206ac | `(([0x1dccc0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x1206db | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x1206df | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1206fb | `eax` | `?` |

## `_elscSendCommand`

- `_elscSendCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121560 to 0x121700.
  Established. The `_msgWrite` site has unknown candidate zero and candidate one `[0x1dc778]`
  (`call` at 0x121616). The two `_strcpy` sites have unknown candidate zero and candidate one
  `0x120773` (`call` at 0x12162f, `call` at 0x121670). These are separate conditional paths. The
  call to `_logWriteLine` has candidate one `sp-124` (`call` at 0x1216aa).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121569 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12156c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x121572 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121581 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x12158e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12159a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12159e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215a7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215ab | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1215c0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215d7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215e3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215e7 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x1215ee | load | 1 | `abs 0x1dccc0` | `-` |
| `movsx` at 0x1215f7 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12160b | load | 4 | `abs 0x1dc778` | `-` |
| `mov` at 0x121611 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121627 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12162b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121640 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121644 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12164c | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x121650 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x121668 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12166c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121680 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121684 | store | 4 | `(unknown)` | `0x5` |
| `cmp` at 0x12168b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1216a5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121586 | `_msgClear 0x126058` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1215a2 | `_sprintf 0x172dd8` | `?` | `0x1214eb` | `?` | `?` | `?` | `?` |
| `call` at 0x1215c5 | `_msgClear 0x126058` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1215db | `_sprintf 0x172dd8` | `?` | `0x120727` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121603 | `_ffscMsg 0x11f040` | `0x1214f7` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x121616 | `_msgWrite 0x1269c8` | `?` | `[0x1dc778]` | `?` | `?` | `?` | `?` |
| `call` at 0x12162f | `_strcpy 0x18038c` | `?` | `0x120773` | `?` | `?` | `?` | `?` |
| `call` at 0x12165b | `_ffscMsg 0x11f040` | `0x12151a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121670 | `_strcpy 0x18038c` | `?` | `0x120773` | `?` | `?` | `?` | `?` |
| `call` at 0x12169c | `_sprintf 0x172dd8` | `sp-124` | `0x121546` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1216aa | `_logWriteLine 0x1251ac` | `?` | `sp-124` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121593 | `? == 0x1` | `? != 0x1` |
| `je` at 0x1215f5 | `(([0x1dccc0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x121620 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x12164a | `? == 0xb` | `? != 0xb` |
| `je` at 0x121690 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1216bd | `eax` | `0x0` |

## `_elscSendPassword`

- `_elscSendPassword`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121700 to 0x1217c0.
  Established. The call to `_elscDoAdminCommand` presents `[sp+4]`, `sp-88`, and `sp-172` as its
  first three candidates (`call` at 0x121763); its result branch is taken when
  `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` (`jne` at 0x12176d). The `_strncmp`
  site has candidates `sp-172`, `0x12138c`, and `0x2` (`call` at 0x121777).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12170b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x121718 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x121721 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121733 | `_sprintf 0x172dd8` | `sp-88` | `0x1216be` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x12174f | `_sprintf 0x172dd8` | `sp-88` | `0x1216c2` | `?` | `ebx` | `esi` | `?` |
| `call` at 0x121763 | `_elscDoAdminCommand 0x120820` | `[sp+4]` | `sp-88` | `sp-172` | `?` | `?` | `?` |
| `call` at 0x121777 | `_strncmp 0x176d68` | `sp-172` | `0x12138c` | `0x2` | `?` | `?` | `?` |
| `call` at 0x121796 | `_ffscMsg 0x11f040` | `0x1216c9` | `sp-172` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12171f | `? == 0x0` | `? != 0x0` |
| `jg` at 0x121728 | `[sp+8] > 0x1f3 (signed, 32-bit)` | `[sp+8] <= 0x1f3 (signed, 32-bit)` |
| `jne` at 0x12176d | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x121781 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1217ab | `eax` | `?` |

## `_elscSetPassword`

- `_elscSetPassword`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1217c0 to 0x121970.
  Established. The `_elscSendPassword` call has leading candidates `[sp+4]` and `0x3e8` (`call` at
  0x121849). A later `_strncpy` site has unknown candidate zero, `[sp+8]` as candidate one, and
  `0x4` as candidate two (`call` at 0x1218bc). The call to `_elscUpdateSaveInfo` has unknown
  candidate values (`call` at 0x1218c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1217c9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1217cc | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1217d2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1217e1 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1217f6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121800 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121803 | load | 1 | `[sp+8] +0x0` | `-` |
| `cmp` at 0x12180d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121819 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12181d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121826 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12182a | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x121845 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121855 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12186d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121882 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121886 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121896 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1218a5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1218a9 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1218af | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1218d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1218d7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1218e0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1218e4 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1217e6 | `_msgClear 0x126058` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x121821 | `_sprintf 0x172dd8` | `?` | `0x1214eb` | `?` | `?` | `?` | `?` |
| `call` at 0x121849 | `_elscSendPassword 0x121700` | `[sp+4]` | `0x3e8` | `?` | `?` | `?` | `?` |
| `call` at 0x121862 | `_sprintf 0x172dd8` | `sp-88` | `0x1217ac` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121871 | `_elscDoAdminCommand 0x120820` | `[sp+4]` | `sp-88` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12188a | `_strcmp 0x180230` | `?` | `0x12138c` | `?` | `?` | `?` | `?` |
| `call` at 0x12189b | `_msgClear 0x126058` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1218bc | `_strncpy 0x180560` | `?` | `[sp+8]` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1218c4 | `_elscUpdateSaveInfo 0x121b60` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1218db | `_sprintf 0x172dd8` | `?` | `0x1217b7` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1217f0 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x1217fa | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `je` at 0x121807 | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) == 0x0` | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) != 0x0` |
| `je` at 0x121812 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x121853 | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` |
| `jne` at 0x12187b | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x121894 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1218f6 | `eax` | `?` |

## `_elscProcessInput`

- `_elscProcessInput`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1211e0 to 0x1213e0.
  Established. The four-byte store to `[[sp+4]+48] +0x1c` writes `0x4` (`mov` at 0x12121f). The
  `_msgRead` call presents `[[sp+4]+48]` and `[0x1dc760]` as leading candidates (`call` at
  0x121230). The byte store through `[[[sp+4]+48]+12] +0x0` writes zero (`mov` at 0x1212d0). The
  meaning of the referenced record remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1211e8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1211eb | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1211f3 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x121210 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x12121c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x12121f | store | 4 | `[[sp+4]+48] +0x1c` | `0x4` |
| `mov` at 0x121226 | load | 4 | `abs 0x1dc760` | `-` |
| `mov` at 0x12122c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121250 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121253 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x12125b | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x12125f | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121270 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121278 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x12127e | store | 4 | `[[sp+4]+48] +0xc` | `[[[sp+4]+48]+12]-2` |
| `mov` at 0x121290 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121298 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x12129b | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1212a2 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1212a9 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1212c0 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1212c4 | store | 4 | `[[sp+4]+48] +0xc` | `?` |
| `mov` at 0x1212c7 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1212ca | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1212cd | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1212d0 | store | 1 | `[[[sp+4]+48]+12] +0x0` | `0x0` |
| `mov` at 0x1212d3 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1212d6 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1212d9 | load | 1 | `[[[sp+4]+48]+8] +0x0` | `-` |
| `mov` at 0x1212df | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x1212e2 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1212e5 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1212ec | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x121303 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121306 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121320 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121328 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121340 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121344 | load | 4 | `[sp+4] +0x30` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1211fc | `_ffscMsg 0x11f040` | `0x12114f` | `[[sp+4]+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x121214 | `_msgClear 0x126058` | `[[sp+4]+48]` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x121230 | `_msgRead 0x126348` | `[[sp+4]+48]` | `[0x1dc760]` | `?` | `?` | `?` | `?` |
| `call` at 0x1212fb | `_sprintf 0x172dd8` | `sp-16` | `0x1211ac` | `[0x1f0c04]` | `?` | `?` | `?` |
| `call` at 0x12130c | `_elscPrintMsg 0x120d70` | `[sp+4]` | `sp-16` | `[[[sp+4]+48]+8]` | `?` | `?` | `?` |
| `call` at 0x12132e | `_elscProcessCommand 0x120f60` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12134d | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1211f1 | `[[sp+4]+4] == 0x1` | `[[sp+4]+4] != 0x1` |
| `je` at 0x12123a | `(ret(_msgRead) & ret(_msgRead)) == 0x0` | `(ret(_msgRead) & ret(_msgRead)) != 0x0` |
| `jne` at 0x121259 | `[[[sp+4]+48]+28] != 0xa` | `[[[sp+4]+48]+28] == 0xa` |
| `jne` at 0x121276 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |
| `je` at 0x121296 | `[[[sp+4]+48]+28] == 0x17` | `[[[sp+4]+48]+28] != 0x17` |
| `jne` at 0x1212a0 | `[[[sp+4]+48]+12] != [[[sp+4]+48]+8]` | `[[[sp+4]+48]+12] == [[[sp+4]+48]+8]` |
| `je` at 0x1212dd | `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14` | `([[[[sp+4]+48]+8]]:1 & 0xff) != 0x14` |
| `jne` at 0x121326 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12135c | `eax` | `?` |

## `_elscProcessCommand`

- `_elscProcessCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120f60 to 0x1211e0.
  Established. The `_strtok_r` site has leading candidates `[sp+8]`, `0x120eb4`, and `sp-8` (`call`
  at 0x120fa1). One path calls `_elscUpdateDisplay` with leading candidates `[sp+4]` and `[sp-8]`
  (`call` at 0x121016).
  A separate site stores `[sp-92]` as four bytes to `[sp+4] +0x10` (`mov` at 0x1210ac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120f6c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120f6f | load | 4 | `sp +0x8` | `-` |
| `test` at 0x120f72 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x120f7c | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120f90 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x120faf | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120ffd | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121011 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x121048 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x12104f | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121068 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x121089 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x12109d | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x1210a0 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1210a9 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x1210ac | store | 4 | `[sp+4] +0x10` | `[sp-92]` |
| `mov` at 0x1210d2 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1210d9 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x1210f0 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x121101 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12111c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x121124 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120f88 | `_ffscMsg 0x11f040` | `0x120e9a` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` |
| `call` at 0x120fa1 | `_strtok_r 0x180708` | `[sp+8]` | `0x120eb4` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x120fbb | `_ffscMsg 0x11f040` | `0x120eb7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120fd1 | `_ffscConvertToUpperCase 0x1255f8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120fdf | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x120ed1` | `?` | `?` | `?` | `?` |
| `call` at 0x120ff1 | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x120ed5` | `?` | `?` | `?` | `?` |
| `call` at 0x121009 | `_ffscMsg 0x11f040` | `0x120ed8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121016 | `_elscUpdateDisplay 0x121a70` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x121028 | `_strncmp 0x176d68` | `ret(_strtok_r)` | `0x120f04` | `0x4` | `?` | `?` | `?` |
| `call` at 0x12103c | `_strncmp 0x176d68` | `ret(_strtok_r)` | `0x120f09` | `0x3` | `?` | `?` | `?` |
| `call` at 0x12105f | `_sprintf 0x172dd8` | `sp-88` | `0x120f0d` | `?` | `?` | `?` | `?` |
| `call` at 0x12106c | `_logWriteLine 0x1251ac` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x121075 | `_elscSetup 0x121970` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12108d | `_elscQueryModuleNum 0x1213e0` | `[[sp+4]+12]` | `sp-92` | `?` | `?` | `?` | `?` |
| `call` at 0x1210af | `_elscUpdateModuleNums 0x121ae0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1210c6 | `_strcmp 0x180230` | `ret(_strtok_r)` | `0x120f26` | `?` | `?` | `?` | `?` |
| `call` at 0x1210e8 | `_sprintf 0x172dd8` | `sp-88` | `0x120f2a` | `[0x1f0c04]` | `?` | `?` | `?` |
| `call` at 0x1210f6 | `_elscPrintMsg 0x120d70` | `[sp+4]` | `sp-88` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x121114 | `_sprintf 0x172dd8` | `sp-348` | `0x120f31` | `?` | `[sp+8]` | `?` | `?` |
| `call` at 0x121128 | `_logWriteLine 0x1251ac` | `[[sp+4]+44]` | `sp-348` | `?` | `?` | `?` | `?` |
| `call` at 0x121131 | `_ffscMsg 0x11f040` | `sp-348` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120f79 | `(([0x1dccc0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x120fad | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x120fe9 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x120ffb | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x121032 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x121046 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x12104d | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x12107f | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x121097 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x1210a3 | `[sp-92] == [[sp+4]+16]` | `[sp-92] != [[sp+4]+16]` |
| `jne` at 0x1210d0 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x121121 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12114e | `eax` | `?` |

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120820 to 0x120a80.
  Established. The `_msgWrite` site uses unknown candidate zero and candidate one `[0x1dc778]`
  (`call` at 0x1208f2). The `_msgRead` site instead has candidate one `[0x1dc764]` (`call` at
  0x120983). A byte store writes zero through `ret(_strpbrk) +0x0` (`mov` at 0x1209d5). The final
  `_strcpy` site has candidate zero `[sp+12]` and unknown candidate one (`call` at
  0x120a4f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120829 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12082c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x120832 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x120841 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120860 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120873 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120877 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120883 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120887 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x12088e | load | 1 | `abs 0x1dccc0` | `-` |
| `movsx` at 0x120897 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1208ab | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1208c5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1208d2 | load | 4 | `abs 0x1dc778` | `-` |
| `mov` at 0x1208d8 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1208de | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1208ed | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1208fe | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12090d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120920 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120924 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12092c | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x120930 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x120943 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120952 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120960 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12096d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120971 | store | 4 | `(unknown)` | `0x5` |
| `mov` at 0x120978 | load | 4 | `abs 0x1dc764` | `-` |
| `mov` at 0x12097e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120993 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120997 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12099f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209c5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209d5 | store | 1 | `ret(_strpbrk) +0x0` | `0x0` |
| `test` at 0x1209d8 | load | 1 | `abs 0x1dccc0` | `-` |
| `mov` at 0x1209e1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209e5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209e9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1209ec | load | 1 | `[sp+4] +0x1ad058` | `-` |
| `cmp` at 0x120a03 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a0a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a0e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a24 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x120a31 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120a37 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x120a3d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120a40 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a47 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a4b | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12084d | `_ffscMsg 0x11f040` | `0x1206fc` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x120865 | `_msgClear 0x126058` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12087b | `_sprintf 0x172dd8` | `?` | `0x120727` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1208a3 | `_ffscMsg 0x11f040` | `0x12072c` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1208bc | `_sprintf 0x172dd8` | `sp-84` | `0x12075d` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1208ca | `_logWriteLine 0x1251ac` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x1208f2 | `_msgWrite 0x1269c8` | `?` | `[0x1dc778]` | `?` | `?` | `?` | `?` |
| `call` at 0x120911 | `_sprintf 0x172dd8` | `[sp+12]` | `0x120773` | `?` | `?` | `?` | `?` |
| `call` at 0x12093b | `_ffscMsg 0x11f040` | `0x12077e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120956 | `_sprintf 0x172dd8` | `[sp+12]` | `0x120773` | `?` | `?` | `?` | `?` |
| `call` at 0x120965 | `_msgClear 0x126058` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120983 | `_msgRead 0x126348` | `?` | `[0x1dc764]` | `?` | `?` | `?` | `?` |
| `call` at 0x1209a8 | `_ffscMsg 0x11f040` | `0x1207b0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1209c9 | `_strpbrk 0x1805a4` | `?` | `0x1207d8` | `?` | `?` | `?` | `?` |
| `call` at 0x1209fb | `_ffscMsg 0x11f040` | `0x1207db` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120a1b | `_sprintf 0x172dd8` | `sp-84` | `0x120805` | `?` | `?` | `?` | `?` |
| `call` at 0x120a29 | `_logWriteLine 0x1251ac` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x120a4f | `_strcpy 0x18038c` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x120846 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `je` at 0x120895 | `(([0x1dccc0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x1208b0 | `? == 0x0` | `? != 0x0` |
| `je` at 0x1208fc | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x120902 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `je` at 0x12092a | `? == 0xb` | `? != 0xb` |
| `je` at 0x120947 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jne` at 0x12098d | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x12099d | `? == 0xb` | `? != 0xb` |
| `je` at 0x1209d3 | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` |
| `je` at 0x1209df | `(([0x1dccc0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1dccc0]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x120a08 | `? == 0x0` | `? != 0x0` |
| `je` at 0x120a35 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120a5f | `eax` | `?` |

## `_elscPrintMsg`

- `_elscPrintMsg`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120d70 to 0x120f60.
  Established. The first `_logWriteLine` call has candidates `[[sp+4]+44]` and `[sp+12]` (`call` at
  0x120d8e). The two `_userSendCommand` sites have candidate zero `[0x1dcf94]` (`call` at 0x120dc3)
  and `[0x1dcf98]` (`call` at 0x120e4b). The `_routeDoAdminCommand` sites use
  candidate zero `sp-492` (`call` at 0x120e16) and `sp-980` (`call` at 0x120e86).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120d7c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120d7f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120d82 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120d8a | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120d96 | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x120d99 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x120da2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120dbd | load | 4 | `abs 0x1dcf94` | `-` |
| `mov` at 0x120df0 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120df9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120dfd | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120e1e | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x120e21 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x120e2a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120e45 | load | 4 | `abs 0x1dcf98` | `-` |
| `mov` at 0x120e60 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x120e69 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120e6d | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120d8e | `_logWriteLine 0x1251ac` | `[[sp+4]+44]` | `[sp+12]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120db2 | `_sprintf 0x172dd8` | `sp-248` | `0x120d1f` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x120dc3 | `_userSendCommand 0x12ca44` | `[0x1dcf94]` | `sp-248` | `0x0` | `?` | `?` | `?` |
| `call` at 0x120dd5 | `_ffscMsg 0x11f040` | `0x120d29` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x120e0d | `_sprintf 0x172dd8` | `sp-492` | `0x120d33` | `[[sp+4]+20]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120e16 | `_routeDoAdminCommand 0x129a0c` | `sp-492` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120e3a | `_sprintf 0x172dd8` | `sp-736` | `0x120d1f` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x120e4b | `_userSendCommand 0x12ca44` | `[0x1dcf98]` | `sp-736` | `0x0` | `?` | `?` | `?` |
| `call` at 0x120e7d | `_sprintf 0x172dd8` | `sp-980` | `0x120d49` | `[[sp+4]+24]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120e86 | `_routeDoAdminCommand 0x129a0c` | `sp-980` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120d87 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x120d9f | `[0x1f0c04] != [[sp+4]+20]` | `[0x1f0c04] == [[sp+4]+20]` |
| `je` at 0x120dcd | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120df6 | `[[sp+4]+20] == 0xffffffff` | `[[sp+4]+20] != 0xffffffff` |
| `jne` at 0x120e27 | `[0x1f0c04] != [[sp+4]+24]` | `[0x1f0c04] == [[sp+4]+24]` |
| `je` at 0x120e52 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120e66 | `[[sp+4]+24] == 0xffffffff` | `[[sp+4]+24] != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120e99 | `eax` | `?` |

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1213e0 to 0x121560.
  Established. The initial four-byte store through `[sp+8] +0x0` writes `0xffffffff` (`mov` at
  0x1213f3). The `_strtol` site presents `sp-85`, `sp-92`, and `0x10` as its first three candidates
  (`call` at 0x12148a). The later four-byte store to `[sp+8] +0x0` has an unknown value (`mov` at
  0x1214dd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1213e9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1213ec | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1213f3 | store | 4 | `[sp+8] +0x0` | `0xffffffff` |
| `movsx` at 0x121419 | load | 1 | `(unknown)` | `-` |
| `or` at 0x12142b | load | 1 | `(unknown)` | `-` |
| `or` at 0x12142b | store | 1 | `(unknown)` | `?` |
| `movsx` at 0x12145c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x121494 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x121497 | load | 1 | `[sp-92] +0x0` | `-` |
| `movsx` at 0x1214ad | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1214dd | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121403 | `_elscDoAdminCommand 0x120820` | `[sp+4]` | `0x12135d` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x121426 | `_ffscMsg 0x11f040` | `0x121361` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121448 | `_strncmp 0x176d68` | `sp-88` | `0x12138c` | `0x2` | `?` | `?` | `?` |
| `call` at 0x12146a | `_ffscMsg 0x11f040` | `0x12138f` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x12148a | `_strtol 0x17fda8` | `sp-85` | `sp-92` | `0x10` | `?` | `?` | `?` |
| `call` at 0x1214bb | `_ffscMsg 0x11f040` | `0x1213b1` | `?` | `sp-85` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1213f1 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x12140d | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x121452 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x12149b | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) != 0x0` | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x1214a3 | `ret(_strtol) <= 0xff (unsigned, 32-bit)` | `ret(_strtol) > 0xff (unsigned, 32-bit)` |
| `jne` at 0x1214d2 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x1214db | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1214ea | `eax` | `?` |

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121ae0 to 0x121b60.
  Established. The two four-byte store sites have unknown effective addresses and values (`rep
  movsd` at 0x121af6, `mov` at 0x121b08). The `_addrdUpdateLocalIdentity` call has candidate zero
  `sp-32` and unknown remaining candidates (`call` at 0x121b19). The relationship between that stack
  record and these stores remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x121af6 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x121af6 | load | 4 | `abs 0x1f0bf8` | `-` |
| `mov` at 0x121b04 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121b08 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121b19 | `_addrdUpdateLocalIdentity 0x10dbf0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x121b13 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121b26 | `eax` | `ret(_addrdUpdateLocalIdentity)` |

## `_elscUpdateDisplay`

- `_elscUpdateDisplay`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121a70 to 0x121ae0.
  Established. The `_bzero` call has leading candidates `[sp+4]+28` and `0xa` (`call` at 0x121a85).
  The `_strncpy` call uses `[sp+4]+28`, `[sp+8]`, and `0xa` (`call` at 0x121a95). The branch
  bypasses the `_strncpy` call when `([sp+8] & [sp+8]) == 0x0` (`je` at 0x121a8f).
  The `_logWriteLine` site has leading candidates `[[sp+4]+44]` and `sp-84` (`call` at 0x121ac2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121a79 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121a7c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121a9d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x121aa4 | load | 1 | `[sp+4] +0x24` | `-` |
| `mov` at 0x121abe | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121a85 | `_bzero 0x172678` | `[sp+4]+28` | `0xa` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121a95 | `_strncpy 0x180560` | `[sp+4]+28` | `[sp+8]` | `0xa` | `?` | `?` | `?` |
| `call` at 0x121ab5 | `_sprintf 0x172dd8` | `sp-84` | `0x121a39` | `[sp+4]+28` | `?` | `?` | `?` |
| `call` at 0x121ac2 | `_logWriteLine 0x1251ac` | `[[sp+4]+44]` | `sp-84` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121a8f | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x121aa2 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121ad0 | `eax` | `?` |

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121b60 to 0x121c10.
  Established. The `_bzero` site presents `sp-68` and `0x40` as leading candidates (`call` at
  0x121b6f). An indexed four-byte store writes `0x3` to an unknown effective address (`mov` at
  0x121ba2). The `_nvramWrite` call has leading candidates `0x5` and `sp-68` (`call` at 0x121bd2);
  its result branch is taken when `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` (`jne` at
  0x121bdc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121b79 | store | 4 | `sp -0x48` | `sp-68` |
| `mov` at 0x121b7c | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x121b7f | store | 4 | `sp -0x54` | `0x1dd16e` |
| `mov` at 0x121b8d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121b93 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121b95 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121b9b | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x121b9e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121ba2 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x121bac | load | 4 | `sp -0x54` | `-` |
| `add` at 0x121bbf | load | 4 | `sp -0x54` | `-` |
| `add` at 0x121bbf | store | 4 | `sp -0x54` | `[sp-84]+60` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121b6f | `_bzero 0x172678` | `sp-68` | `0x40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121bb4 | `_strncpy 0x180560` | `?` | `?` | `0x4` | `?` | `?` | `?` |
| `call` at 0x121bd2 | `_nvramWrite 0x127298` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x121bf5 | `_ffscMsg 0x11f040` | `0x121b27` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x121bca | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x121bdc | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121c08 | `eax` | `?` |

## `_routeProcessELSCCommand`

- `_routeProcessELSCCommand`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x129a2c to 0x129b1c.
  Established. The `_elscSendPassword` site has unknown candidate zero and candidate one
  `[[sp+8]+32]` (`call` at 0x129a64). The distinct `_elscSendCommand` site instead has candidate one
  `[sp+4]` (`call` at 0x129a71). A four-byte store writes `0xb` through an unknown effective address
  (`mov` at 0x129aa1). The controller and record roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x129a32 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x129a35 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x129a3a | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x129a48 | load | 4 | `abs 0x1f0c04` | `-` |
| `mov` at 0x129a4e | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x129a5c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x129a5f | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x129a6c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x129a7c | load | 1 | `([0x1f0c04] + [sp+12]) +0x4` | `-` |
| `mov` at 0x129a89 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129a8f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129a9b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x129aa1 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x129a64 | `_elscSendPassword 0x121700` | `?` | `[[sp+8]+32]` | `?` | `?` | `?` | `?` |
| `call` at 0x129a71 | `_elscSendCommand 0x121560` | `?` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x129a93 | `_sprintf 0x172dd8` | `?` | `0x129a22` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x129a40 | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1f0c04] + [sp+12])+4]:1 & 0xff) & ([([0x1f0c04] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jae` at 0x129a5a | `C == 0` | `not (C == 0)` |
| `jge` at 0x129a82 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jbe` at 0x129aaf | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x129aba | `eax` | `?` |
