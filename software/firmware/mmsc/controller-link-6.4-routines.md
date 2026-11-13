# MMSC controller link in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved transfers were found in these spans.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/controller-link-6.4-facts.txt) include all tool sections.

## `_elscCheckResponse`

- `_elscCheckResponse`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120430 to 0x1205d0.
  Established. The `_strchr` call has leading candidates unknown and `0x14` (`call` at 0x12047f).
  The branch takes `(ret(_strchr) & ret(_strchr)) != 0x0` and falls through for zero (`jne` at
  0x12048b); its taken route writes byte zero at `ret(_strchr) +0x0` (`mov` at 0x120440). Another
  `_strchr` call has leading candidates unknown and `0xd` (`call` at 0x120446). The
  `_elscProcessCommand` call presents leading candidate `[sp+4]` and unknown remaining candidates
  (`call` at 0x120471). The response grammar and command identity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120436 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120440 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x120452 | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `test` at 0x120455 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x12046d | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120446 | `_strchr 0x17e8f8` | `?` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x120464 | `_ffscMsg 0x11edf0` | `0x120400` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120471 | `_elscProcessCommand 0x120cf0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12047f | `_strchr 0x17e8f8` | `?` | `0x14` | `?` | `?` | `?` | `?` |
| `call` at 0x120492 | `_strlen 0x1739b4` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12049d | `_memmove 0x17e880` | `[sp+8]` | `?` | `ret(_strlen)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120450 | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x12045c | `(([0x1db370]:1 & 0xff) & 0x10) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x12048b | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x12048f | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1204ab | `eax` | `?` |

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1205d0 to 0x120830.
  Established. A four-byte store writes `0x1` at an unknown effective target (`mov` at 0x120637). A
  later four-byte store has unknown target and value `0x5` (`mov` at 0x120721). The branch takes
  `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` and falls through for nonzero (`je` at 0x1206ac). A
  byte store writes zero at `ret(_strpbrk) +0x0` (`mov` at 0x120785). The administrative transaction
  and message-state meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1205d9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1205dc | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1205e2 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1205f1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120610 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120623 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120627 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120633 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120637 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x12063e | load | 1 | `abs 0x1db370` | `-` |
| `movsx` at 0x120647 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x12065b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120675 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120682 | load | 4 | `abs 0x1dae28` | `-` |
| `mov` at 0x120688 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x12068e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12069d | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1206ae | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1206bd | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1206d0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1206d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1206dc | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1206e0 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1206f3 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120702 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120710 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12071d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120721 | store | 4 | `(unknown)` | `0x5` |
| `mov` at 0x120728 | load | 4 | `abs 0x1dae14` | `-` |
| `mov` at 0x12072e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120743 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120747 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12074f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120775 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120785 | store | 1 | `ret(_strpbrk) +0x0` | `0x0` |
| `test` at 0x120788 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x120791 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120795 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120799 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12079c | load | 1 | `[sp+4] +0x1ab740` | `-` |
| `cmp` at 0x1207b3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1207ba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1207be | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1207d4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1207e1 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1207e7 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1207ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1207f0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1207f7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1207fb | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1205fd | `_ffscMsg 0x11edf0` | `0x1204ac` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x120615 | `_msgClear 0x125d48` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12062b | `_sprintf 0x172120` | `?` | `0x1204d7` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x120653 | `_ffscMsg 0x11edf0` | `0x1204dc` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x12066c | `_sprintf 0x172120` | `sp-84` | `0x12050d` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12067a | `_logWriteLine 0x124e9c` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x1206a2 | `_msgWrite 0x126698` | `?` | `[0x1dae28]` | `?` | `?` | `?` | `?` |
| `call` at 0x1206c1 | `_sprintf 0x172120` | `[sp+12]` | `0x120523` | `?` | `?` | `?` | `?` |
| `call` at 0x1206eb | `_ffscMsg 0x11edf0` | `0x12052e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120706 | `_sprintf 0x172120` | `[sp+12]` | `0x120523` | `?` | `?` | `?` | `?` |
| `call` at 0x120715 | `_msgClear 0x125d48` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120733 | `_msgRead 0x126048` | `?` | `[0x1dae14]` | `?` | `?` | `?` | `?` |
| `call` at 0x120758 | `_ffscMsg 0x11edf0` | `0x120560` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120779 | `_strpbrk 0x17ec94` | `?` | `0x120588` | `?` | `?` | `?` | `?` |
| `call` at 0x1207ab | `_ffscMsg 0x11edf0` | `0x12058b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1207cb | `_sprintf 0x172120` | `sp-84` | `0x1205b5` | `?` | `?` | `?` | `?` |
| `call` at 0x1207d9 | `_logWriteLine 0x124e9c` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x1207ff | `_strcpy 0x17ea7c` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x1205f6 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `je` at 0x120645 | `(([0x1db370]:1 & 0xff) & 0x10) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x120660 | `? == 0x0` | `? != 0x0` |
| `je` at 0x1206ac | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x1206b2 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `je` at 0x1206da | `? == 0xb` | `? != 0xb` |
| `je` at 0x1206f7 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jne` at 0x12073d | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x12074d | `? == 0xb` | `? != 0xb` |
| `je` at 0x120783 | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` |
| `je` at 0x12078f | `(([0x1db370]:1 & 0xff) & 0x10) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x1207b8 | `? == 0x0` | `? != 0x0` |
| `je` at 0x1207e5 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12080f | `eax` | `?` |

## `_elscInit`

- `_elscInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120830 to 0x120b10.
  Established. The `_nvramRead` call has leading candidates `0x5` and `sp-68` (`call` at 0x120856).
  The branch takes `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` and falls through for nonzero (`je`
  at 0x120860). The fall-through calls `_bzero` with leading candidates `sp-68` and `0x40` (`call`
  at 0x120865). Stores in that initialization path include four-byte `0xffffffff` at unknown
  effective targets (`mov` at 0x120879, `mov` at 0x120882). The controller-record layout and default
  values are not physically identified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120839 | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x120840 | load | 4 | `abs 0x1ec8cc` | `-` |
| `mov` at 0x120845 | store | 4 | `sp -0x4c` | `[0x1ec8cc]` |
| `mov` at 0x120848 | load | 4 | `abs 0x1ec8d0` | `-` |
| `mov` at 0x12084d | store | 4 | `sp -0x48` | `[0x1ec8d0]` |
| `mov` at 0x120871 | store | 4 | `sp -0x7c` | `sp-68` |
| `mov` at 0x120879 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x12087f | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x120882 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x12089d | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x1208a0 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x1208c9 | store | 4 | `sp -0x60` | `sp-68` |
| `mov` at 0x1208cf | store | 4 | `sp -0x64` | `sp-84` |
| `mov` at 0x1208d4 | store | 4 | `sp -0x68` | `0x0` |
| `mov` at 0x1208db | store | 4 | `sp -0x6c` | `0x1db7b6` |
| `mov` at 0x1208e2 | store | 4 | `sp -0x70` | `0x0` |
| `mov` at 0x1208e9 | store | 4 | `sp -0x74` | `0x1db7ac` |
| `mov` at 0x1208f0 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x1208fa | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x120902 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120906 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12090a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12090e | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120916 | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x120919 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x12091c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12091f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x120923 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120929 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x12092d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120934 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12093a | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x120946 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x120949 | load | 4 | `sp -0x70` | `-` |
| `test` at 0x12094c | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x120953 | load | 4 | `([sp-112] + [sp-96]) +0x4` | `-` |
| `mov` at 0x120957 | store | 4 | `(unknown)` | `[([sp-112] + [sp-96])+4]` |
| `mov` at 0x120960 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x120968 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x12096b | load | 4 | `sp -0x70` | `-` |
| `test` at 0x12096e | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x12097c | load | 4 | `sp -0x6c` | `-` |
| `mov` at 0x120988 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x120995 | load | 4 | `sp -0x6c` | `-` |
| `cmp` at 0x1209a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209a8 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x1209b6 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1209c2 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1209c8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1209da | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x1209e2 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1209ed | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1209f9 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1209ff | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a0e | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x120a16 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x120a21 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120a2d | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x120a33 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a42 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `mov` at 0x120a84 | load | 4 | `sp -0x58` | `-` |
| `cmp` at 0x120a87 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x120a8d | store | 4 | `(unknown)` | `[sp-88]` |
| `mov` at 0x120a91 | store | 4 | `sp -0x5c` | `0x1` |
| `add` at 0x120a9b | load | 4 | `sp -0x68` | `-` |
| `add` at 0x120a9b | store | 4 | `sp -0x68` | `[sp-104]+16` |
| `add` at 0x120a9f | load | 4 | `sp -0x6c` | `-` |
| `add` at 0x120a9f | store | 4 | `sp -0x6c` | `[sp-108]+60` |
| `add` at 0x120aa3 | load | 4 | `sp -0x70` | `-` |
| `add` at 0x120aa3 | store | 4 | `sp -0x70` | `[sp-112]+32` |
| `add` at 0x120aa7 | load | 4 | `sp -0x74` | `-` |
| `add` at 0x120aa7 | store | 4 | `sp -0x74` | `[sp-116]+60` |
| `cmp` at 0x120ab5 | load | 4 | `sp -0x5c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120856 | `_nvramRead 0x126da8` | `0x5` | `sp-68` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120865 | `_bzero 0x1719c0` | `sp-68` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x120895 | `_strncpy 0x17ec50` | `?` | `0x120810` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1208b7 | `_nvramWrite 0x126f38` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x12093e | `_bzero 0x1719c0` | `[sp-116]` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x120980 | `_strncpy 0x17ec50` | `[sp-108]` | `([sp-112] + [sp-96])+8` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120999 | `_strcpy 0x17ea7c` | `[sp-108]` | `0x120810` | `?` | `?` | `?` | `?` |
| `call` at 0x1209ba | `_sprintf 0x172120` | `[sp-100]` | `0x120815` | `?` | `?` | `?` | `?` |
| `call` at 0x1209d2 | `_msgNew 0x125df8` | `0xf0` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x1209f1 | `_sprintf 0x172120` | `[sp-100]` | `0x12081c` | `?` | `?` | `?` | `?` |
| `call` at 0x120a06 | `_msgNew 0x125df8` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120a25 | `_sprintf 0x172120` | `[sp-100]` | `0x120823` | `?` | `?` | `?` | `?` |
| `call` at 0x120a3a | `_msgNew 0x125df8` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x120a67 | `_elscSetup 0x121700` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120a78 | `_elscQueryModuleNum 0x121170` | `?` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x120abb | `_elscUpdateModuleNums 0x121870` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120860 | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x1208af | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120951 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x120973 | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) != 0x0` |
| `jl` at 0x1209a6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x1209e0 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `je` at 0x120a14 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `jne` at 0x120a48 | `(ret(_msgNew) & ret(_msgNew)) != 0x0` | `(ret(_msgNew) & ret(_msgNew)) == 0x0` |
| `jne` at 0x120a71 | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x120a82 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x120a8b | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x120aaf | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x120ab9 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120ace | `eax` | `?` |

## `_elscPrintMsg`

- `_elscPrintMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120b10 to 0x120cf0.
  Established. The first branch takes `([[sp+4]+44] & [[sp+4]+44]) == 0x0` and falls through for
  nonzero (`je` at 0x120b27); its fall-through includes `_logWriteLine` with leading candidates
  `[[sp+4]+44]` and `[sp+12]` (`call` at 0x120b2e). A later branch takes `[0x1ef244] != [[sp+4]+20]`
  and falls through for equality (`jne` at 0x120b3f). That fall-through includes `_userSendCommand`
  with leading candidates `[0x1db644]`, `sp-248`, and `0x0` (`call` at 0x120b63). The routing and
  message presentation rules remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120b1c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120b1f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x120b22 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120b2a | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120b36 | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x120b39 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x120b42 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120b5d | load | 4 | `abs 0x1db644` | `-` |
| `mov` at 0x120b80 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120b89 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120b8d | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x120bae | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x120bb1 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x120bba | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120bd5 | load | 4 | `abs 0x1db648` | `-` |
| `mov` at 0x120bf0 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x120bf9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x120bfd | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120b2e | `_logWriteLine 0x124e9c` | `[[sp+4]+44]` | `[sp+12]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x120b52 | `_sprintf 0x172120` | `sp-248` | `0x120acf` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x120b63 | `_userSendCommand 0x12c1d4` | `[0x1db644]` | `sp-248` | `0x0` | `?` | `?` | `?` |
| `call` at 0x120b9d | `_sprintf 0x172120` | `sp-492` | `0x120ad9` | `[[sp+4]+20]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120ba6 | `_routeDoAdminCommand 0x12932c` | `sp-492` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120bca | `_sprintf 0x172120` | `sp-736` | `0x120acf` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x120bdb | `_userSendCommand 0x12c1d4` | `[0x1db648]` | `sp-736` | `0x0` | `?` | `?` | `?` |
| `call` at 0x120c0d | `_sprintf 0x172120` | `sp-980` | `0x120aef` | `[[sp+4]+24]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x120c16 | `_routeDoAdminCommand 0x12932c` | `sp-980` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120b27 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x120b3f | `[0x1ef244] != [[sp+4]+20]` | `[0x1ef244] == [[sp+4]+20]` |
| `je` at 0x120b6d | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120b86 | `[[sp+4]+20] == 0xffffffff` | `[[sp+4]+20] != 0xffffffff` |
| `jne` at 0x120bb7 | `[0x1ef244] != [[sp+4]+24]` | `[0x1ef244] == [[sp+4]+24]` |
| `je` at 0x120be2 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x120bf6 | `[[sp+4]+24] == 0xffffffff` | `[[sp+4]+24] != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120c29 | `eax` | `?` |

## `_elscProcessCommand`

- `_elscProcessCommand`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120cf0 to 0x120f70.
  Established. The `_strtok_r` call presents leading candidates `[sp+8]`, `0x120c44`, and `sp-8`
  (`call` at 0x120d31). The branch takes `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` and falls
  through for zero (`jne` at 0x120d3d). The taken route calls `_ffscConvertToUpperCase` with leading
  candidate `ret(_strtok_r)` (`call` at 0x120d61). A later four-byte store writes `[sp-92]` at
  `[sp+4] +0x10` (`mov` at 0x120e3c). The parsed command language and that field meaning remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120cfc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120cff | load | 4 | `sp +0x8` | `-` |
| `test` at 0x120d02 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x120d0c | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120d20 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x120d3f | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120d8d | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120da1 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x120dd8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120ddf | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120df8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120e19 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x120e2d | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x120e30 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120e39 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x120e3c | store | 4 | `[sp+4] +0x10` | `[sp-92]` |
| `mov` at 0x120e62 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120e69 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x120e80 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x120e91 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x120eac | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x120eb4 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120d18 | `_ffscMsg 0x11edf0` | `0x120c2a` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` |
| `call` at 0x120d31 | `_strtok_r 0x17edf8` | `[sp+8]` | `0x120c44` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x120d4b | `_ffscMsg 0x11edf0` | `0x120c47` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120d61 | `_ffscConvertToUpperCase 0x1252e8` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120d6f | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x120c61` | `?` | `?` | `?` | `?` |
| `call` at 0x120d81 | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x120c65` | `?` | `?` | `?` | `?` |
| `call` at 0x120d99 | `_ffscMsg 0x11edf0` | `0x120c68` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120da6 | `_elscUpdateDisplay 0x121800` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x120db8 | `_strncmp 0x175458` | `ret(_strtok_r)` | `0x120c94` | `0x4` | `?` | `?` | `?` |
| `call` at 0x120dcc | `_strncmp 0x175458` | `ret(_strtok_r)` | `0x120c99` | `0x3` | `?` | `?` | `?` |
| `call` at 0x120def | `_sprintf 0x172120` | `sp-88` | `0x120c9d` | `?` | `?` | `?` | `?` |
| `call` at 0x120dfc | `_logWriteLine 0x124e9c` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x120e05 | `_elscSetup 0x121700` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120e1d | `_elscQueryModuleNum 0x121170` | `[[sp+4]+12]` | `sp-92` | `?` | `?` | `?` | `?` |
| `call` at 0x120e3f | `_elscUpdateModuleNums 0x121870` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x120e56 | `_strcmp 0x17e920` | `ret(_strtok_r)` | `0x120cb6` | `?` | `?` | `?` | `?` |
| `call` at 0x120e78 | `_sprintf 0x172120` | `sp-88` | `0x120cba` | `[0x1ef244]` | `?` | `?` | `?` |
| `call` at 0x120e86 | `_elscPrintMsg 0x120b10` | `[sp+4]` | `sp-88` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x120ea4 | `_sprintf 0x172120` | `sp-348` | `0x120cc1` | `?` | `[sp+8]` | `?` | `?` |
| `call` at 0x120eb8 | `_logWriteLine 0x124e9c` | `[[sp+4]+44]` | `sp-348` | `?` | `?` | `?` | `?` |
| `call` at 0x120ec1 | `_ffscMsg 0x11edf0` | `sp-348` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120d09 | `(([0x1db370]:1 & 0xff) & 0x10) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x120d3d | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x120d79 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x120d8b | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x120dc2 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x120dd6 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x120ddd | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x120e0f | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x120e27 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x120e33 | `[sp-92] == [[sp+4]+16]` | `[sp-92] != [[sp+4]+16]` |
| `jne` at 0x120e60 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x120eb1 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x120ede | `eax` | `?` |

## `_elscProcessInput`

- `_elscProcessInput`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x120f70 to 0x121170.
  Established. The branch takes `[[sp+4]+4] == 0x1` and falls through for inequality (`je` at
  0x120f81); its taken route writes four bytes containing `0x4` at `[[sp+4]+48] +0x1c` (`mov` at
  0x120faf). `_msgRead` has leading candidates `[[sp+4]+48]` and `[0x1dae10]` (`call` at 0x120fc0).
  The result branch takes `(ret(_msgRead) & ret(_msgRead)) == 0x0` and falls through for nonzero
  (`je` at 0x120fca). The message-state and input-owner semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x120f78 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x120f7b | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x120f83 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x120fa0 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120fac | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120faf | store | 4 | `[[sp+4]+48] +0x1c` | `0x4` |
| `mov` at 0x120fb6 | load | 4 | `abs 0x1dae10` | `-` |
| `mov` at 0x120fbc | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120fe0 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x120fe3 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x120feb | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x120fef | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121000 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121008 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x12100e | store | 4 | `[[sp+4]+48] +0xc` | `[[[sp+4]+48]+12]-2` |
| `mov` at 0x121020 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x121028 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x12102b | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121032 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121039 | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x121050 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x121054 | store | 4 | `[[sp+4]+48] +0xc` | `?` |
| `mov` at 0x121057 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x12105a | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x12105d | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x121060 | store | 1 | `[[[sp+4]+48]+12] +0x0` | `0x0` |
| `mov` at 0x121063 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121066 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121069 | load | 1 | `[[[sp+4]+48]+8] +0x0` | `-` |
| `mov` at 0x12106f | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x121072 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x121075 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12107c | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x121093 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x121096 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1210b0 | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x1210b8 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1210d0 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x1210d4 | load | 4 | `[sp+4] +0x30` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x120f8c | `_ffscMsg 0x11edf0` | `0x120edf` | `[[sp+4]+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x120fa4 | `_msgClear 0x125d48` | `[[sp+4]+48]` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x120fc0 | `_msgRead 0x126048` | `[[sp+4]+48]` | `[0x1dae10]` | `?` | `?` | `?` | `?` |
| `call` at 0x12108b | `_sprintf 0x172120` | `sp-16` | `0x120f3c` | `[0x1ef244]` | `?` | `?` | `?` |
| `call` at 0x12109c | `_elscPrintMsg 0x120b10` | `[sp+4]` | `sp-16` | `[[[sp+4]+48]+8]` | `?` | `?` | `?` |
| `call` at 0x1210be | `_elscProcessCommand 0x120cf0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1210dd | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x120f81 | `[[sp+4]+4] == 0x1` | `[[sp+4]+4] != 0x1` |
| `je` at 0x120fca | `(ret(_msgRead) & ret(_msgRead)) == 0x0` | `(ret(_msgRead) & ret(_msgRead)) != 0x0` |
| `jne` at 0x120fe9 | `[[[sp+4]+48]+28] != 0xa` | `[[[sp+4]+48]+28] == 0xa` |
| `jne` at 0x121006 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |
| `je` at 0x121026 | `[[[sp+4]+48]+28] == 0x17` | `[[[sp+4]+48]+28] != 0x17` |
| `jne` at 0x121030 | `[[[sp+4]+48]+12] != [[[sp+4]+48]+8]` | `[[[sp+4]+48]+12] == [[[sp+4]+48]+8]` |
| `je` at 0x12106d | `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14` | `([[[[sp+4]+48]+8]]:1 & 0xff) != 0x14` |
| `jne` at 0x1210b6 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1210ec | `eax` | `?` |

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121170 to 0x1212f0.
  Established. The first branch takes `([sp+8] & [sp+8]) == 0x0` and falls through for nonzero (`je`
  at 0x121181); its fall-through stores four-byte `0xffffffff` at `[sp+8] +0x0` (`mov` at 0x121183).
  `_elscDoAdminCommand` has leading candidates `[sp+4]`, `0x1210ed`, and `sp-88` (`call` at
  0x121193). The later `_strtol` call presents `sp-85`, `sp-92`, and `0x10` (`call` at 0x12121a).
  The module-number response and error policy are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121179 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12117c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121183 | store | 4 | `[sp+8] +0x0` | `0xffffffff` |
| `movsx` at 0x1211a9 | load | 1 | `(unknown)` | `-` |
| `or` at 0x1211bb | load | 1 | `(unknown)` | `-` |
| `or` at 0x1211bb | store | 1 | `(unknown)` | `?` |
| `movsx` at 0x1211ec | load | 1 | `(unknown)` | `-` |
| `mov` at 0x121224 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x121227 | load | 1 | `[sp-92] +0x0` | `-` |
| `movsx` at 0x12123d | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12126d | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121193 | `_elscDoAdminCommand 0x1205d0` | `[sp+4]` | `0x1210ed` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x1211b6 | `_ffscMsg 0x11edf0` | `0x1210f1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1211d8 | `_strncmp 0x175458` | `sp-88` | `0x12111c` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1211fa | `_ffscMsg 0x11edf0` | `0x12111f` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x12121a | `_strtol 0x17e498` | `sp-85` | `sp-92` | `0x10` | `?` | `?` | `?` |
| `call` at 0x12124b | `_ffscMsg 0x11edf0` | `0x121141` | `?` | `sp-85` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121181 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x12119d | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x1211e2 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x12122b | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) != 0x0` | `(([[sp-92]]:1 & 0xff) & ([[sp-92]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x121233 | `ret(_strtol) <= 0xff (unsigned, 32-bit)` | `ret(_strtol) > 0xff (unsigned, 32-bit)` |
| `jne` at 0x121262 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x12126b | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12127a | `eax` | `?` |

## `_elscSendCommand`

- `_elscSendCommand`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1212f0 to 0x121490.
  Established. The branch takes `? == 0x1` and falls through for inequality, with the compared value
  unknown (`je` at 0x121323). A four-byte store on the fall-through path writes `0xb` at an unknown
  effective target (`mov` at 0x12133b). The taken route includes a four-byte store of `0x1` at an
  unknown target (`mov` at 0x121377). `_msgWrite` presents leading candidates unknown and
  `[0x1dae28]` (`call` at 0x1213a6). The exit returns EAX zero (`ret` at 0x12144d), but the command
  state and transport destination remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1212f9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1212fc | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x121302 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121311 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x12131e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12132a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12132e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121337 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12133b | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x121350 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121363 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121367 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121373 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121377 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x12137e | load | 1 | `abs 0x1db370` | `-` |
| `movsx` at 0x121387 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12139b | load | 4 | `abs 0x1dae28` | `-` |
| `mov` at 0x1213a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1213b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1213bb | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1213d0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1213d4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1213dc | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x1213e0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1213f8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1213fc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121410 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121414 | store | 4 | `(unknown)` | `0x5` |
| `cmp` at 0x12141b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121435 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121316 | `_msgClear 0x125d48` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x121332 | `_sprintf 0x172120` | `?` | `0x12127b` | `?` | `?` | `?` | `?` |
| `call` at 0x121355 | `_msgClear 0x125d48` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12136b | `_sprintf 0x172120` | `?` | `0x1204d7` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121393 | `_ffscMsg 0x11edf0` | `0x121287` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1213a6 | `_msgWrite 0x126698` | `?` | `[0x1dae28]` | `?` | `?` | `?` | `?` |
| `call` at 0x1213bf | `_strcpy 0x17ea7c` | `?` | `0x120523` | `?` | `?` | `?` | `?` |
| `call` at 0x1213eb | `_ffscMsg 0x11edf0` | `0x1212aa` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121400 | `_strcpy 0x17ea7c` | `?` | `0x120523` | `?` | `?` | `?` | `?` |
| `call` at 0x12142c | `_sprintf 0x172120` | `sp-124` | `0x1212d6` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12143a | `_logWriteLine 0x124e9c` | `?` | `sp-124` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121323 | `? == 0x1` | `? != 0x1` |
| `je` at 0x121385 | `(([0x1db370]:1 & 0xff) & 0x10) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x1213b0 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x1213da | `? == 0xb` | `? != 0xb` |
| `je` at 0x121420 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12144d | `eax` | `0x0` |

## `_elscSendPassword`

- `_elscSendPassword`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121490 to 0x121550.
  Established. The signed branch takes `[sp+8] > 0x1f3` and falls through for values at most `0x1f3`
  (`jg` at 0x1214b8). The alternative `_sprintf` calls present leading pairs `sp-88`, `0x12144e`
  (`call` at 0x1214c3) or `sp-88`, `0x121452` (`call` at 0x1214df). `_elscDoAdminCommand` has
  leading candidates `[sp+4]`, `sp-88`, and `sp-172` (`call` at 0x1214f3); its result branch takes
  `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` and falls through for zero (`jne`
  at 0x1214fd). The password-version interpretation of the threshold is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12149b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1214a8 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x1214b1 | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1214c3 | `_sprintf 0x172120` | `sp-88` | `0x12144e` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x1214df | `_sprintf 0x172120` | `sp-88` | `0x121452` | `?` | `ebx` | `esi` | `?` |
| `call` at 0x1214f3 | `_elscDoAdminCommand 0x1205d0` | `[sp+4]` | `sp-88` | `sp-172` | `?` | `?` | `?` |
| `call` at 0x121507 | `_strncmp 0x175458` | `sp-172` | `0x12111c` | `0x2` | `?` | `?` | `?` |
| `call` at 0x121526 | `_ffscMsg 0x11edf0` | `0x121459` | `sp-172` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1214af | `? == 0x0` | `? != 0x0` |
| `jg` at 0x1214b8 | `[sp+8] > 0x1f3 (signed, 32-bit)` | `[sp+8] <= 0x1f3 (signed, 32-bit)` |
| `jne` at 0x1214fd | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x121511 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12153b | `eax` | `?` |

## `_elscSetPassword`

- `_elscSetPassword`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121550 to 0x121700.
  Established. The entry branch takes `([sp+12] & [sp+12]) == 0x0` and falls through for nonzero
  (`je` at 0x121580). The `_elscSendPassword` call has leading candidates `[sp+4]` and `0x3e8`
  (`call` at 0x1215d9). Another call to `_elscDoAdminCommand` presents `[sp+4]`, `sp-88`, and `0x0`
  (`call` at 0x121601). Alternative paths include four-byte stores of `0xb` at unknown effective
  targets (`mov` at 0x1215ba, `mov` at 0x121674). The authorization and persisted-password behavior
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121559 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12155c | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x121562 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121571 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x121586 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121590 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121593 | load | 1 | `[sp+8] +0x0` | `-` |
| `cmp` at 0x12159d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215a9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215ad | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215b6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1215ba | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x1215d5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1215e5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1215fd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121612 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121616 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121626 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121635 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x121639 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x12163f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x121663 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121667 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121670 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121674 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121576 | `_msgClear 0x125d48` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x1215b1 | `_sprintf 0x172120` | `?` | `0x12127b` | `?` | `?` | `?` | `?` |
| `call` at 0x1215d9 | `_elscSendPassword 0x121490` | `[sp+4]` | `0x3e8` | `?` | `?` | `?` | `?` |
| `call` at 0x1215f2 | `_sprintf 0x172120` | `sp-88` | `0x12153c` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x121601 | `_elscDoAdminCommand 0x1205d0` | `[sp+4]` | `sp-88` | `0x0` | `?` | `?` | `?` |
| `call` at 0x12161a | `_strcmp 0x17e920` | `?` | `0x12111c` | `?` | `?` | `?` | `?` |
| `call` at 0x12162b | `_msgClear 0x125d48` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12164c | `_strncpy 0x17ec50` | `?` | `[sp+8]` | `0x4` | `?` | `?` | `?` |
| `call` at 0x121654 | `_elscUpdateSaveInfo 0x1218f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12166b | `_sprintf 0x172120` | `?` | `0x121547` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121580 | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x12158a | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `je` at 0x121597 | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) == 0x0` | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) != 0x0` |
| `je` at 0x1215a2 | `? == 0x1` | `? != 0x1` |
| `jne` at 0x1215e3 | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` |
| `jne` at 0x12160b | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x121624 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121686 | `eax` | `?` |

## `_elscSetup`

- `_elscSetup`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121700 to 0x121800.
  Established. The `_elscDoAdminCommand` call presents leading candidates `[[sp+4]+12]`, `0x12169b`,
  and `sp-88` (`call` at 0x121735); its branch takes `(ret(_elscDoAdminCommand) &
  ret(_elscDoAdminCommand)) == 0x0` and falls through for nonzero (`je` at 0x12173f). Alternative
  four-byte stores at `[sp+4] +0x0` have unknown values (`mov` at 0x12178a, `mov` at 0x1217bc). The
  controller setup flags and external command effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121708 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x12170b | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x121714 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121731 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x121741 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121775 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x121786 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x12178a | store | 4 | `[sp+4] +0x0` | `?` |
| `mov` at 0x1217a0 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1217af | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1217b8 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1217bc | store | 4 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121720 | `_ffscMsg 0x11edf0` | `0x121687` | `?` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x121735 | `_elscDoAdminCommand 0x1205d0` | `[[sp+4]+12]` | `0x12169b` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x12174d | `_ffscMsg 0x11edf0` | `0x1216a1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x121768 | `_strncmp 0x175458` | `sp-88` | `0x12111c` | `0x2` | `?` | `?` | `?` |
| `call` at 0x121781 | `_ffscMsg 0x11edf0` | `0x1216cc` | `?` | `sp-88` | `?` | `?` | `?` |
| `call` at 0x1217b3 | `_elscDoAdminCommand 0x1205d0` | `[[sp+4]+12]` | `0x1216f0` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x121712 | `(([0x1db370]:1 & 0xff) & 0x2) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x2) != 0x0` |
| `je` at 0x12173f | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x121772 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x1217a6 | `[[sp+4]+4] != 0x1` | `[[sp+4]+4] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1217c8 | `eax` | `?` |

## `_elscUpdateDisplay`

- `_elscUpdateDisplay`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121800 to 0x121870.
  Established. The `_bzero` call has leading candidates `[sp+4]+28` and `0xa` (`call` at 0x121815).
  The branch takes `([sp+8] & [sp+8]) == 0x0` and falls through for nonzero (`je` at 0x12181f). That
  fall-through calls `_strncpy` with candidates `[sp+4]+28`, `[sp+8]`, and `0xa` (`call` at
  0x121825). A later `_logWriteLine` call has leading candidates `[[sp+4]+44]` and `sp-84` (`call`
  at 0x121852). The displayed field and physical display effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121809 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12180c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12182d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x121834 | load | 1 | `[sp+4] +0x24` | `-` |
| `mov` at 0x12184e | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x121815 | `_bzero 0x1719c0` | `[sp+4]+28` | `0xa` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121825 | `_strncpy 0x17ec50` | `[sp+4]+28` | `[sp+8]` | `0xa` | `?` | `?` | `?` |
| `call` at 0x121845 | `_sprintf 0x172120` | `sp-84` | `0x1217c9` | `[sp+4]+28` | `?` | `?` | `?` |
| `call` at 0x121852 | `_logWriteLine 0x124e9c` | `[[sp+4]+44]` | `sp-84` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12181f | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x121832 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121860 | `eax` | `?` |

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x121870 to 0x1218f0.
  Established. The explicit four-byte store associated with `rep movsd` has unknown target and value (`rep
  movsd` at 0x121886). Another four-byte store also has unknown effective target and value (`mov` at
  0x121898). The unsigned branch takes `? <= 0x1` and falls through for greater values (`jbe` at
  0x1218a3). `_addrdUpdateLocalIdentity` receives leading candidate `sp-32` (`call` at 0x1218a9),
  and its result reaches the EAX return (`ret` at 0x1218b6). The module-number record and update
  semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x121886 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x121886 | load | 4 | `abs 0x1ef238` | `-` |
| `mov` at 0x121894 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121898 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1218a9 | `_addrdUpdateLocalIdentity 0x10db80` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1218a3 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1218b6 | `eax` | `ret(_addrdUpdateLocalIdentity)` |

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1218f0 to 0x1219a0.
  Established. The `_bzero` call presents leading candidates `sp-68` and `0x40` (`call` at
  0x1218ff). A four-byte store has unknown effective target and value `0x3` (`mov` at 0x121932).
  `_nvramWrite` has leading candidates `0x5` and `sp-68` (`call` at 0x121962). The result branch
  takes `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` and falls through for zero (`jne` at
  0x12196c). The saved record format and persistence semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x121909 | store | 4 | `sp -0x48` | `sp-68` |
| `mov` at 0x12190c | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x12190f | store | 4 | `sp -0x54` | `0x1db7b6` |
| `mov` at 0x12191d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x121923 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121925 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12192b | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x12192e | store | 4 | `(unknown)` | `?` |
| `mov` at 0x121932 | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x12193c | load | 4 | `sp -0x54` | `-` |
| `add` at 0x12194f | load | 4 | `sp -0x54` | `-` |
| `add` at 0x12194f | store | 4 | `sp -0x54` | `[sp-84]+60` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1218ff | `_bzero 0x1719c0` | `sp-68` | `0x40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x121944 | `_strncpy 0x17ec50` | `?` | `?` | `0x4` | `?` | `?` | `?` |
| `call` at 0x121962 | `_nvramWrite 0x126f38` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x121985 | `_ffscMsg 0x11edf0` | `0x1218b7` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x12195a | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x12196c | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x121998 | `eax` | `?` |

## `_clLeaveFlashMode`

- `_clLeaveFlashMode`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x119b08 to 0x119b98.
  Established. Four-byte stores clear `[sp+4] +0x18` (`mov` at 0x119b0f), `[sp+4] +0x1c` (`mov` at
  0x119b16), `[sp+4] +0x20` (`mov` at 0x119b1d), `[sp+4] +0x24` (`mov` at 0x119b24), and `[sp+4]
  +0x28` (`mov` at 0x119b2b). The branch takes `[[sp+4]+12] != 0x4` and falls through for equality
  (`jne` at 0x119b40). The fall-through calls `_close` with leading candidate `[[sp+4]+96]` (`call`
  at 0x119b46). After the paths join, a four-byte store writes `0xffffffff` at `[sp+4] +0x60` (`mov`
  at 0x119b4e). The flash-mode and descriptor-state meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x119b0c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x119b0f | store | 4 | `[sp+4] +0x18` | `0x0` |
| `mov` at 0x119b16 | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x119b1d | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x119b24 | store | 4 | `[sp+4] +0x24` | `0x0` |
| `mov` at 0x119b2b | store | 4 | `[sp+4] +0x28` | `0x0` |
| `mov` at 0x119b32 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x119b37 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x119b3a | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x119b42 | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x119b4e | store | 4 | `[sp+4] +0x60` | `0xffffffff` |
| `test` at 0x119b55 | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x119b6b | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x119b46 | `_close 0x187970` | `[[sp+4]+96]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119b64 | `_ffscMsg 0x11edf0` | `0x119add` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x119b40 | `[[sp+4]+12] != 0x4` | `[[sp+4]+12] == 0x4` |
| `je` at 0x119b5c | `(([0x1db370]:1 & 0xff) & 0x4) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x119b71 | `eax` | `0x0` |
