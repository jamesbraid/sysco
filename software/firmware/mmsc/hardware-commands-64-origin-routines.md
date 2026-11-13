# MMSC hardware-command and ELSC-mode callers in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Undecoded tails were read directly: `0x115dec` to exclusive `0x115df0` is
`25 64 00 90`, and `0x11692c` to exclusive `0x116930` is `65 00 90 90`.
No instruction effects are credited to those bytes.
Repeated `ret(F)` labels can refer to distinct call results and do not prove equal addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Calls use
the assumed cdecl32 convention with unknown arity. Printed candidates can be local values,
saved registers or stale stack words. Unknown candidates and effective addresses stay unknown.
The access tables show explicit memory operands, including stack locations, and omit
implicit push/pop accesses. Full facts retain those accesses. Call and return instructions
remain listed without separate implicit stack access rows.
Unsupported instructions and partial-register tracking can leave results unresolved.
An analysis unknown does not establish indeterminate firmware behavior.
Field meanings, hardware roles, command semantics and callee effects remain unknown
unless explicitly established by an entry.

[Fact sheets and complete listings](disasm/hardware-commands-64-origin-facts.txt) include all tool sections.

## `_cmdTTY_REINIT`

- `_cmdTTY_REINIT`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118100 to 0x118208.
  Established. The `_sysSerialHwInit` site retains unknown candidates (`call` at 0x118160). The
  `_ioctl` site has unknown candidate zero, candidate one `0x1007`, and candidate two `0x2` (`call`
  at 0x11817f). A byte store writes `0x0` at `sp -0x2c` (`mov` at 0x118165), while a later four-byte
  store at an unknown target writes `0xb` (`mov` at 0x1181ef). Neither an ioctl meaning nor a serial
  reinitialization outcome is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118109 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11810c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x11810f | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x11811e | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x118123 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11812a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118140 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x118145 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x118165 | store | 1 | `sp -0x2c` | `0x0` |
| `mov` at 0x118177 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1181c8 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1181d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1181da | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1181e3 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1181e8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1181ef | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11812e | `_sprintf 0x171680` | `?` | `0x116c70` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x118153 | `_taskDelay 0x170b80` | `0x3c` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x11815b | `_taskLock 0x170a50` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118160 | `_sysSerialHwInit 0x10b618` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11817f | `_ioctl 0x186f90` | `?` | `0x1007` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1181a7 | `_sprintf 0x171680` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1181b4 | `_strcat 0x17de28` | `sp-44` | `sp-56` | `?` | `?` | `?` | `?` |
| `call` at 0x1181c2 | `_taskUnlock 0x170a80` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1181de | `_sprintf 0x171680` | `?` | `0x1180f5` | `[0x1ee564]` | `sp-44` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x118117 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x11814b | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `je` at 0x118189 | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `jle` at 0x1181c0 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118204 | `eax` | `?` |

## `_cmdREBOOT_FFSC`

- `_cmdREBOOT_FFSC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117fc0 to 0x118060.
  Established. The opening branch is taken when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` (`jg` at
  0x117fd1). The four-byte store at absolute `0x1ebb7c` writes `0x1` (`mov` at 0x11800e). Another
  four-byte store at an unknown target writes `0xb` (`mov` at 0x11803a). The separate exits carry
  `0x0` (`ret` at 0x117ff2) and `0x1` (`ret` at 0x118049). These literal values do not prove a
  reboot.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117fc3 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x117fc6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x117fc9 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x117fd8 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117fdd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117fe4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118000 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x118006 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11800e | store | 4 | `abs 0x1ebb7c` | `0x1` |
| `mov` at 0x11801e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x118025 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11802e | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x118033 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11803a | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117fe8 | `_sprintf 0x171680` | `?` | `0x116c70` | `ebp` | `?` | `?` | `?` |
| `call` at 0x118029 | `_sprintf 0x171680` | `?` | `0x117f9c` | `[0x1ee564]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x117fd1 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x11800c | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x117ff2 | `eax` | `0x0` |
| `ret` at 0x118049 | `eax` | `0x1` |

## `_cmdSCAN`

- `_cmdSCAN`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1136e8 to 0x1138b8.
  Established. The `_elscQueryModuleNum` call presents `[sp-24]` and `sp-8` as its leading
  candidates (`call` at 0x113799). A four-byte store at an unresolved effective address writes
  `[sp-8]` (`mov` at 0x1137ae). A separate four-byte store at an unknown target writes `0xffffffff`
  (`mov` at 0x1137e8). The final `_elscUpdateModuleNums` site retains unknown candidates (`call` at
  0x11382d). Module-discovery effects remain unproved by these names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1136f1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1136f4 | store | 4 | `sp -0x1c` | `0x0` |
| `mov` at 0x1136fb | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x11370a | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11370f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113716 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113728 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11372d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113730 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11373c | store | 4 | `sp -0x18` | `0x0` |
| `mov` at 0x11374c | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113752 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113755 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11375e | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x113766 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113769 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `test` at 0x113775 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x113795 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x1137a5 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x1137a8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137ae | store | 4 | `(unknown)` | `[sp-8]` |
| `mov` at 0x1137b2 | store | 4 | `sp -0x1c` | `0x1` |
| `mov` at 0x1137bd | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1137cf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137d3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1137e8 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1137f0 | store | 4 | `sp -0x1c` | `0x1` |
| `mov` at 0x1137fc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113800 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11380c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113810 | store | 4 | `(unknown)` | `0xb` |
| `inc` at 0x11381a | load | 4 | `sp -0x18` | `-` |
| `inc` at 0x11381a | store | 4 | `sp -0x18` | `?` |
| `cmp` at 0x11381d | load | 4 | `sp -0x18` | `-` |
| `cmp` at 0x113827 | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11371a | `_sprintf 0x171680` | `?` | `0x112927` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x113785 | `_elscSetup 0x1213f0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113799 | `_elscQueryModuleNum 0x120e60` | `[sp-24]` | `sp-8` | `?` | `?` | `?` | `?` |
| `call` at 0x1137c1 | `_ffscModuleNumToString 0x1255cc` | `[sp-8]` | `sp-20` | `?` | `?` | `?` | `?` |
| `call` at 0x1137d7 | `_sprintf 0x171680` | `?` | `0x1136d7` | `sp-20` | `?` | `?` | `?` |
| `call` at 0x113804 | `_sprintf 0x171680` | `?` | `0x1136cf` | `?` | `?` | `?` | `?` |
| `call` at 0x11382d | `_elscUpdateModuleNums 0x1214c0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x113703 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x113736 | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jb` at 0x113764 | `C == 1` | `not (C == 1)` |
| `jge` at 0x11376f | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11377c | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x11378f | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x1137a3 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x1137ac | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x113821 | `[sp-24] <= 0x1 (unsigned, 32-bit)` | `[sp-24] > 0x1 (unsigned, 32-bit)` |
| `je` at 0x11382b | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113840 | `eax` | `?` |

## `_cmdCOM`

- `_cmdCOM`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x112bb8 to 0x113268.
  Established. The `_nvramWrite` sites present leading candidates `0x4` and `0x1da030` (`call` at
  0x112e3e, `call` at 0x112f62, `call` at 0x11303e, `call` at 0x11317e). The `_ioctl` site has
  unknown candidate zero, candidate one `0x4`, and candidate two `ret(_strtol)` (`call` at
  0x113136). A two-byte store at an unknown target writes `0xffff` (`mov` at 0x112f34). No
  serial-option, record-layout or ioctl meaning is assigned from these values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x112bc1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x112bc4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x112bc7 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x112bca | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x112bd0 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x112c27 | store | 4 | `sp -0x24` | `ret(_strtol)` |
| `mov` at 0x112c2a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x112c2d | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x112c33 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x112c58 | store | 4 | `sp -0x28` | `?` |
| `mov` at 0x112c77 | load | 4 | `sp -0x28` | `-` |
| `movsx` at 0x112c82 | load | 2 | `(unknown)` | `-` |
| `test` at 0x112c9a | load | 1 | `(unknown)` | `-` |
| `test` at 0x112cac | load | 1 | `(unknown)` | `-` |
| `mov` at 0x112cba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112cc1 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112cc6 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x112cd1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112cd8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112ce8 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x112d1e | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112d48 | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x112d9e | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112dde | load | 4 | `sp -0x28` | `-` |
| `cmp` at 0x112e08 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x112e0e | load | 4 | `sp -0x28` | `-` |
| `and` at 0x112e14 | load | 2 | `(unknown)` | `-` |
| `and` at 0x112e14 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x112e28 | load | 4 | `sp -0x28` | `-` |
| `or` at 0x112e2e | load | 2 | `(unknown)` | `-` |
| `or` at 0x112e2e | store | 2 | `(unknown)` | `?` |
| `mov` at 0x112e50 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x112e8e | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112ee8 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112eee | load | 2 | `sp -0x24` | `-` |
| `mov` at 0x112ef2 | store | 2 | `(unknown)` | `[sp-36]:2` |
| `test` at 0x112ef9 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x112f02 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x112f06 | load | 4 | `sp -0x28` | `-` |
| `movsx` at 0x112f25 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x112f2a | load | 4 | `sp -0x24` | `-` |
| `cmp` at 0x112f2f | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112f34 | store | 2 | `(unknown)` | `0xffff` |
| `test` at 0x112f3b | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x112f6a | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x112f9e | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x112fde | load | 4 | `sp -0x28` | `-` |
| `cmp` at 0x113008 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x11300e | load | 4 | `sp -0x28` | `-` |
| `or` at 0x113014 | load | 2 | `(unknown)` | `-` |
| `or` at 0x113014 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x113028 | load | 4 | `sp -0x28` | `-` |
| `and` at 0x11302e | load | 2 | `(unknown)` | `-` |
| `and` at 0x11302e | store | 2 | `(unknown)` | `?` |
| `mov` at 0x113050 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113076 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x11307b | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113086 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11308d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1130be | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x1130f9 | store | 4 | `sp -0x24` | `ret(_strtol)` |
| `mov` at 0x1130fc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1130ff | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x11312b | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x11312e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113142 | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x113146 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x113149 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113168 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x11316e | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x113171 | store | 4 | `(unknown)` | `[sp-36]` |
| `test` at 0x113186 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11318f | load | 4 | `sp -0x24` | `-` |
| `mov` at 0x113193 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x1131a5 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1131b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131c8 | load | 4 | `sp -0x28` | `-` |
| `mov` at 0x1131cd | load | 4 | `sp -0x20` | `-` |
| `mov` at 0x1131e3 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1131e8 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1131ef | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113208 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11320d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113214 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x112be4 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x112bf7 | `_ffscMsg 0x11ebd0` | `0x1128ba` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112c1f | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x112c42 | `_ffscMsg 0x11ebd0` | `0x1128da` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x112c63 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x112c8d | `_ffscIntToToken 0x12554c` | `0x1aa878` | `?` | `sp-28` | `0x14` | `?` | `?` |
| `call` at 0x112cdc | `_sprintf 0x171680` | `?` | `0x1128f0` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x112d12 | `_ffscTokenToInt 0x125aec` | `0x1aa850` | `ret(_strtok_r)` | `sp-32` | `?` | `?` | `?` |
| `call` at 0x112d29 | `_ffscMsg 0x11ebd0` | `0x112937` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x112d90 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x112da8 | `_ffscMsg 0x11ebd0` | `0x11295a` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112dd2 | `_ffscTokenToInt 0x125aec` | `0x1aac84` | `ret(_strtok_r)` | `sp-36` | `?` | `?` | `?` |
| `call` at 0x112de9 | `_ffscMsg 0x11ebd0` | `0x11297e` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x112e3e | `_nvramWrite 0x126e80` | `0x4` | `0x1da030` | `?` | `?` | `?` | `?` |
| `call` at 0x112e58 | `_cmCheckTTYFlags 0x119c20` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112e80 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x112e98 | `_ffscMsg 0x11ebd0` | `0x1129fa` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112ec2 | `_ffscTokenToInt 0x125aec` | `0x1aa878` | `ret(_strtok_r)` | `sp-36` | `?` | `?` | `?` |
| `call` at 0x112ed4 | `_ffscMsg 0x11ebd0` | `0x112a1e` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x112f10 | `_ffscMsg 0x11ebd0` | `0x112a34` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112f4d | `_ffscMsg 0x11ebd0` | `0x112a50` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112f62 | `_nvramWrite 0x126e80` | `0x4` | `0x1da030` | `?` | `?` | `?` | `?` |
| `call` at 0x112f90 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x112fa8 | `_ffscMsg 0x11ebd0` | `0x112ab3` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x112fd2 | `_ffscTokenToInt 0x125aec` | `0x1aac84` | `ret(_strtok_r)` | `sp-36` | `?` | `?` | `?` |
| `call` at 0x112fe9 | `_ffscMsg 0x11ebd0` | `0x112ad7` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x11303e | `_nvramWrite 0x126e80` | `0x4` | `0x1da030` | `?` | `?` | `?` | `?` |
| `call` at 0x113058 | `_cmCheckTTYFlags 0x119c20` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113091 | `_sprintf 0x171680` | `?` | `0x1129a9` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x1130b0 | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x1130c8 | `_ffscMsg 0x11ebd0` | `0x112afa` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1130ef | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11310f | `_ffscMsg 0x11ebd0` | `0x112b17` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x113136 | `_ioctl 0x186f90` | `?` | `0x4` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x113156 | `_ffscMsgS 0x11eca0` | `0x112b2e` | `?` | `[sp-36]` | `?` | `?` | `?` |
| `call` at 0x11317e | `_nvramWrite 0x126e80` | `0x4` | `0x1da030` | `?` | `?` | `?` | `?` |
| `call` at 0x11319d | `_ffscMsg 0x11ebd0` | `0x112b58` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1131bb | `_sprintf 0x171680` | `?` | `?` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x1131d6 | `_ffscMsg 0x11ebd0` | `0x112b79` | `[sp-32]` | `?` | `?` | `?` | `?` |
| `call` at 0x1131f3 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x112bd6 | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x112bf0 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x112c31 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x112c3a | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |
| `jne` at 0x112c6f | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x112c9f | `(? & 0x10) == 0x0` | `(? & 0x10) != 0x0` |
| `je` at 0x112cb1 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `jg` at 0x112cf0 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x112d1c | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) == 0x0` | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) != 0x0` |
| `je` at 0x112d4e | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `jg` at 0x112d54 | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `je` at 0x112d59 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `je` at 0x112d6b | `[sp-32] == 0x3` | `[sp-32] != 0x3` |
| `je` at 0x112d74 | `[sp-32] == 0x4` | `[sp-32] != 0x4` |
| `jne` at 0x112d9c | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x112ddc | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) == 0x0` | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) != 0x0` |
| `je` at 0x112e0c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `jle` at 0x112e6c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x112e8c | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x112ecc | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) == 0x0` | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) != 0x0` |
| `je` at 0x112f00 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jne` at 0x112f2d | `[sp-36] != ?` | `[sp-36] == ?` |
| `je` at 0x112f32 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x112f42 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jle` at 0x112f59 | `? <= 0x5 (signed, 32-bit)` | `? > 0x5 (signed, 32-bit)` |
| `jne` at 0x112f9c | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x112fdc | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) == 0x0` | `(ret(_ffscTokenToInt) & ret(_ffscTokenToInt)) != 0x0` |
| `je` at 0x11300c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `jle` at 0x11306c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jg` at 0x113070 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `jne` at 0x1130bc | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x113103 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jg` at 0x113107 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x113140 | `(ret(_ioctl) & ret(_ioctl)) == 0x0` | `(ret(_ioctl) & ret(_ioctl)) != 0x0` |
| `je` at 0x11318d | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113229 | `eax` | `?` |

## `_cmdDOWNLOADER`

- `_cmdDOWNLOADER`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116e00 to 0x1170e0.
  Established. The `_enable_downloader` (`call` at 0x116ee3) and `_disable_downloader` (`call` at
  0x116ef0) sites both retain unknown candidate slots. The branch deciding between their paths is
  taken when `Z == 1` (`je` at 0x116ee1). A later four-byte store at an unresolved target writes
  `0xb` (`mov` at 0x116f1c). The exit remains unknown (`ret` at 0x116f30), so no enabling or
  disabling effect is inferred from the call names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x116e05 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116e08 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x116e0b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116e0e | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x116eb5 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116eba | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ec1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116ed2 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116ed7 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x116ef5 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116f00 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116f07 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116f10 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116f15 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116f1c | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116e38 | `_strtok_r 0x17e358` | `0x0` | `0x116c80` | `[sp+4]` | `ebx` | `esi` | `ebp` |
| `call` at 0x116e4b | `_ffscMsg 0x11ebd0` | `0x116d72` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116e61 | `_ffscConvertToUpperCase 0x12543c` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116e6f | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x116db3` | `?` | `?` | `?` | `?` |
| `call` at 0x116e96 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x116dba` | `?` | `?` | `?` | `?` |
| `call` at 0x116ea8 | `_ffscMsg 0x11ebd0` | `0x116dc2` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x116ec5 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116ee3 | `_enable_downloader 0x10cb28` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116ef0 | `_disable_downloader 0x10cb68` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116f0b | `_sprintf 0x171680` | `?` | `0x116de9` | `[0x1ee564]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x116e16 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `jne` at 0x116e44 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x116e79 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x116ea0 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `je` at 0x116edd | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `je` at 0x116ee1 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116f30 | `eax` | `?` |

## `_cmdOPTIONS`

- `_cmdOPTIONS`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113388 to 0x113528.
  Established. The `_strtol` site presents `ret(_strtok_r)`, `sp-8`, and `0x0` as its first three
  candidates (`call` at 0x1133ef). The four-byte store through `[sp+8] +0x1c` retains `ret(_strtol)`
  (`mov` at 0x113438). The following `_userUpdateSaveInfo` call has candidate zero `[sp+8]` (`call`
  at 0x11343c). The common exit reports `0x0` in `eax` (`ret` at 0x11346c). Persistence and
  option-field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113390 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113393 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1133b0 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x1133bf | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1133c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1133cb | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1133f9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1133ff | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x113402 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x11341b | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113420 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113427 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113438 | store | 4 | `[sp+8] +0x1c` | `ret(_strtol)` |
| `mov` at 0x113444 | load | 4 | `[sp+8] +0x1c` | `-` |
| `mov` at 0x11344d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113452 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113459 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11339e | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `ebx` | `esi` | `?` |
| `call` at 0x1133cf | `_sprintf 0x171680` | `?` | `0x112927` | `?` | `?` | `?` | `?` |
| `call` at 0x1133ef | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x11340e | `_ffscMsg 0x11ebd0` | `0x11334d` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x11342b | `_sprintf 0x171680` | `?` | `0x1128d0` | `?` | `?` | `?` | `?` |
| `call` at 0x11343c | `_userUpdateSaveInfo 0x12b8b8` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11345d | `_sprintf 0x171680` | `?` | `0x11336f` | `[[sp+8]+28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1133aa | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `jg` at 0x1133b8 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x1133fd | `[sp-8] == 0x0` | `[sp-8] != 0x0` |
| `je` at 0x113406 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11346c | `eax` | `0x0` |

## `_cmdNAP_TIME`

- `_cmdNAP_TIME`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113268 to 0x113388.
  Established. The `_strtoul` site has leading candidates `ret(_strtok_r)`, `0x0`, and `0x0` (`call`
  at 0x1132c8). The comparison-result branch is taken when `(ret(_strcmp) & ret(_strcmp)) == 0x0`
  (`je` at 0x1132c1). The `_sprintf` site presents candidate one `0x113247`, with its other
  candidate slots unknown (`call` at 0x113313). No duration or scheduling effect is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11326d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x113270 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11328d | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x113303 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113308 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11330f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11332d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113332 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113339 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11327b | `_strtok_r 0x17e358` | `0x0` | `0x1128b7` | `[sp+4]` | `ebx` | `esi` | `ebp` |
| `call` at 0x1132a9 | `_ffscConvertToUpperCase 0x12543c` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1132b7 | `_strcmp 0x17de80` | `ret(_strtok_r)` | `0x11322a` | `?` | `?` | `?` | `?` |
| `call` at 0x1132c8 | `_strtoul 0x17db9c` | `ret(_strtok_r)` | `0x0` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1132da | `_ffscMsg 0x11ebd0` | `0x113232` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x113313 | `_sprintf 0x171680` | `?` | `0x113247` | `?` | `?` | `?` | `?` |
| `call` at 0x11333d | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x113287 | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `jg` at 0x113295 | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x1132c1 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jg` at 0x1132d2 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11334c | `eax` | `0x0` |

## `_cmdCONSOLE`

- `_cmdCONSOLE`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1159a0 to 0x115b90.
  Established. A four-byte load reads through `[0x1d9b9c] +0x84` (`mov` at 0x115a4c). The
  `_userSendCommand` site has leading candidates `[[0x1d9b9c]+132]` and `sp-248` (`call` at
  0x115a53); its result branch is taken when `(ret(_userSendCommand) & ret(_userSendCommand)) !=
  0x0` (`jne` at 0x115a5d). The common exit carries `0x0` in `eax` (`ret` at 0x115b07), without
  proving console routing.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1159ab | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1159ae | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1159b1 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1159d1 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x1159d7 | load | 4 | `abs 0x1d9b9c` | `-` |
| `cmp` at 0x1159f0 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x115a10 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x115a15 | load | 4 | `[0x1d9b9c] +0x18` | `-` |
| `mov` at 0x115a36 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115a3b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115a42 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115a47 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x115a4c | load | 4 | `[0x1d9b9c] +0x84` | `-` |
| `mov` at 0x115a70 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x115a78 | load | 4 | `[sp+8] +0x10` | `-` |
| `cmp` at 0x115aa0 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x115aa9 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x115aae | load | 4 | `[0x1d9b9c] +0x18` | `-` |
| `mov` at 0x115abd | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115ac2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115ac9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115ae5 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115aea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115af1 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1159c1 | `_strspn 0x17e264` | `[[sp+4]]` | `0x115930` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x115a2e | `_sprintf 0x171680` | `sp-248` | `0x115954` | `([[sp+4]] + ret(_strspn))` | `?` | `?` | `?` |
| `call` at 0x115a53 | `_userSendCommand 0x12b7d8` | `[[0x1d9b9c]+132]` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x115a86 | `_ffscMsg 0x11ebd0` | `0x11595e` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x115acd | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115af5 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1159b5 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x1159cb | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x1159dd | `[0x1d9b9c] != [[sp+8]+148]` | `[0x1d9b9c] == [[sp+8]+148]` |
| `jne` at 0x1159f7 | `[0x1d9b9c] != 0x0` | `[0x1d9b9c] == 0x0` |
| `jne` at 0x115a1b | `[[0x1d9b9c]+24] != 0x9` | `[[0x1d9b9c]+24] == 0x9` |
| `jne` at 0x115a5d | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` |
| `je` at 0x115a76 | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x115a7e | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x115aa7 | `[0x1d9b9c] == 0x0` | `[0x1d9b9c] != 0x0` |
| `jne` at 0x115ab4 | `[[0x1d9b9c]+24] != 0x9` | `[[0x1d9b9c]+24] == 0x9` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x115b07 | `eax` | `0x0` |

## `_cmdDIRECT`

- `_cmdDIRECT`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x115b90 to 0x115df0.
  Established. The `_userChangeInputMode` site has unknown candidate zero, candidate one `0x8`, and
  candidate two `0x0` (`call` at 0x115bf5); its result branch is taken when
  `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` (`je` at 0x115bff). A four-byte
  store writes `0x1` at absolute `0x1eebc0` (`mov` at 0x115c43). The common exit reports `0x0`
  (`ret` at 0x115c80). A mode-change effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x115b94 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x115b97 | load | 4 | `abs 0x1da96c` | `-` |
| `mov` at 0x115b9f | load | 4 | `abs 0x1da970` | `-` |
| `cmp` at 0x115bb0 | load | 4 | `abs 0x1da970` | `-` |
| `mov` at 0x115bd0 | load | 4 | `abs 0x1da96c` | `-` |
| `mov` at 0x115bd6 | load | 4 | `[sp+8] +0x20` | `-` |
| `mov` at 0x115c10 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c16 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c20 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c2e | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115c33 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c3a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c43 | store | 4 | `abs 0x1eebc0` | `0x1` |
| `mov` at 0x115c63 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x115c68 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c6f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x115c7a | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x115bbe | `_ffscMsg 0x11ebd0` | `0x115b08` | `[sp+8]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x115bf5 | `_userChangeInputMode 0x12aa48` | `?` | `0x8` | `0x0` | `ebx` | `ebp` | `?` |
| `call` at 0x115c3e | `_sprintf 0x171680` | `?` | `0x115b70` | `0x8000` | `?` | `0x7` | `?` |
| `call` at 0x115c56 | `_ffscMsg 0x11ebd0` | `0x115b4f` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x115c73 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x115b9d | `[0x1da96c] != [sp+8]` | `[0x1da96c] == [sp+8]` |
| `je` at 0x115bb6 | `[0x1da970] == [sp+8]` | `[0x1da970] != [sp+8]` |
| `jg` at 0x115bde | `[[sp+8]+32] > 0x3e7 (signed, 32-bit)` | `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)` |
| `je` at 0x115bff | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `jne` at 0x115c1c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x115c80 | `eax` | `0x0` |

## `_cmdFFSC`

- `_cmdFFSC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116250 to 0x116600.
  Established. The `_cmdProcessFFSCCommand` site presents `[[sp+4]]`, `[sp+8]`, and `[sp+12]` as its
  first three candidates (`call` at 0x116295). The four-byte store through `[[sp+8]+132] +0x14`
  writes `0xffffffff` (`mov` at 0x116406). A subsequent byte store at an unknown target writes `0x0`
  (`mov` at 0x11641c). Another four-byte store writes `0x1` at absolute `0x1eebc0` (`mov` at
  0x11651e). These stores do not establish field roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11625c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11625f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116262 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116265 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x116279 | load | 1 | `(ret(_strspn) + [[sp+4]]) +0x0` | `-` |
| `mov` at 0x116280 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116285 | load | 1 | `([0x1ee564] + [sp+12]) +0x4` | `-` |
| `mov` at 0x11628f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x116292 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1162ab | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1162b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162b7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1162e0 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x1162e7 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1162ef | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116340 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11634c | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x11634f | load | 4 | `abs 0x1ee564` | `-` |
| `test` at 0x116370 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x116379 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x11637f | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1163b2 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1163b8 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x1163d0 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1163d5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1163dc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116400 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116406 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x11640d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116412 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116419 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11641c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x116430 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x116433 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11643f | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116459 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11645e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116465 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116494 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116497 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11649e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164ad | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1164b2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164b9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164dd | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1164e2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1164e9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11650c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116513 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11651e | store | 4 | `abs 0x1eebc0` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x116271 | `_strspn 0x17e264` | `[[sp+4]]` | `0x115930` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x116295 | `_cmdProcessFFSCCommand 0x111770` | `[[sp+4]]` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x1162bb | `_sprintf 0x171680` | `?` | `0x116166` | `?` | `?` | `?` | `?` |
| `call` at 0x1162fd | `_ffscMsg 0x11ebd0` | `0x116170` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x116315 | `_destNumRacks 0x11ef3c` | `[sp+12]` | `sp-8` | `?` | `?` | `?` | `?` |
| `call` at 0x116328 | `_ffscMsg 0x11ebd0` | `0x116191` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x11635c | `_ffscMsg 0x11ebd0` | `0x1161ba` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116388 | `_ffscMsg 0x11ebd0` | `0x1161ee` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x116391 | `_userAcquireRemote 0x12a978` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1163c8 | `_sprintf 0x171680` | `sp-252` | `0x11621b` | `[[[sp+8]+132]+20]` | `0x2` | `?` | `?` |
| `call` at 0x1163e2 | `_userSendCommand 0x12b7d8` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x1163ef | `_userReleaseRemote 0x12b738` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11644c | `_ffscMsg 0x11ebd0` | `0x115d77` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116469 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116488 | `_remStartConnectCommand 0x128568` | `[sp+8]` | `0x11622a` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x1164bd | `_sprintf 0x171680` | `?` | `0x115dc6` | `?` | `?` | `?` | `?` |
| `call` at 0x1164ed | `_sprintf 0x171680` | `?` | `0x115b70` | `0x8000` | `ret(_remStartConnectCommand)` | `0x4` | `?` |
| `call` at 0x116517 | `_sprintf 0x171680` | `?` | `0x11622f` | `0x8000` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x116269 | `([[sp+4]] & [[sp+4]]) == 0x0` | `([[sp+4]] & [[sp+4]]) != 0x0` |
| `je` at 0x11627e | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) == 0x0` | `(([(ret(_strspn) + [[sp+4]])]:1 & 0xff) & ([(ret(_strspn) + [[sp+4]])]:1 & 0xff)) != 0x0` |
| `je` at 0x11628b | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) == 0x0` | `(([([0x1ee564] + [sp+12])+4]:1 & 0xff) & ([([0x1ee564] + [sp+12])+4]:1 & 0xff)) != 0x0` |
| `jne` at 0x1162a0 | `ret(_cmdProcessFFSCCommand) != 0x2` | `ret(_cmdProcessFFSCCommand) == 0x2` |
| `je` at 0x1162ed | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x1162f5 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x116320 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x116346 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x116355 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `je` at 0x116377 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x11639d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x1163ec | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x116439 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `jne` at 0x116445 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116492 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116534 | `eax` | `?` |

## `_cmdRAT`

- `_cmdRAT`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116600 to 0x116930.
  Established. The `_userSendCommand` site presents `ret(_userAcquireRemote)` and `sp-252` as its
  first two candidates (`call` at 0x116772); its result branch is taken when `(ret(_userSendCommand)
  & ret(_userSendCommand)) == 0x0` (`je` at 0x11677c). A later four-byte store through `[[sp+8]+132]
  +0x14` writes `0xffffffff` (`mov` at 0x116796). The four-byte store at absolute `0x1eebc0` writes
  `0x1` (`mov` at 0x116897). Remote-mode semantics remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11660c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11660f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x116612 | store | 4 | `sp -0x8` | `0xffffffff` |
| `mov` at 0x116619 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116621 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11663c | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116641 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116648 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116685 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11668a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116691 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166a0 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1166ac | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x1166af | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1166c9 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1166ce | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1166d5 | load | 4 | `(unknown)` | `-` |
| `test` at 0x1166f0 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1166f9 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x1166ff | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x116724 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116729 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116730 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116742 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116748 | load | 4 | `[[sp+8]+132] +0x14` | `-` |
| `mov` at 0x116760 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116765 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11676c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116790 | load | 4 | `[sp+8] +0x84` | `-` |
| `mov` at 0x116796 | store | 4 | `[[sp+8]+132] +0x14` | `0xffffffff` |
| `mov` at 0x11679d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1167a2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1167a9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1167ac | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x1167c0 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x1167c3 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1167cf | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x1167e9 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1167ee | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1167f5 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116824 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x116827 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11682e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116834 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11683d | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116842 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116849 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116868 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11686d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116874 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116887 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11688e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116897 | store | 4 | `abs 0x1eebc0` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11662f | `_ffscMsg 0x11ebd0` | `0x116535` | `[sp+8]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x11664c | `_sprintf 0x171680` | `?` | `0x115982` | `?` | `?` | `?` | `?` |
| `call` at 0x116665 | `_destNumRacks 0x11ef3c` | `[sp+12]` | `sp-8` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x116678 | `_ffscMsg 0x11ebd0` | `0x116555` | `ret(_destNumRacks)` | `?` | `?` | `?` | `?` |
| `call` at 0x116695 | `_sprintf 0x171680` | `?` | `0x115cc8` | `?` | `?` | `?` | `?` |
| `call` at 0x1166bc | `_ffscMsg 0x11ebd0` | `0x11657d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1166d9 | `_sprintf 0x171680` | `?` | `0x115cc8` | `?` | `?` | `?` | `?` |
| `call` at 0x116708 | `_ffscMsg 0x11ebd0` | `0x1165b0` | `[[[sp+8]+132]+20]` | `?` | `?` | `?` | `?` |
| `call` at 0x116711 | `_userAcquireRemote 0x12a978` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x116734 | `_sprintf 0x171680` | `?` | `0x11593d` | `?` | `?` | `?` | `?` |
| `call` at 0x116758 | `_sprintf 0x171680` | `sp-252` | `0x11621b` | `[[[sp+8]+132]+20]` | `0xa` | `?` | `?` |
| `call` at 0x116772 | `_userSendCommand 0x12b7d8` | `ret(_userAcquireRemote)` | `sp-252` | `?` | `?` | `?` | `?` |
| `call` at 0x11677f | `_userReleaseRemote 0x12b738` | `ret(_userAcquireRemote)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1167dc | `_ffscMsg 0x11ebd0` | `0x115d77` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1167f9 | `_sprintf 0x171680` | `?` | `0x115db1` | `?` | `?` | `?` | `?` |
| `call` at 0x116818 | `_remStartConnectCommand 0x128568` | `[sp+8]` | `0x1165dc` | `[sp-8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x11684d | `_sprintf 0x171680` | `?` | `0x115dc6` | `?` | `?` | `?` | `?` |
| `call` at 0x116878 | `_sprintf 0x171680` | `?` | `0x1165e0` | `ret(_remStartConnectCommand)` | `0x4` | `?` | `?` |
| `call` at 0x116892 | `_sprintf 0x171680` | `?` | `0x11598f` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11661f | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x116627 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `je` at 0x116670 | `ret(_destNumRacks) == 0x1` | `ret(_destNumRacks) != 0x1` |
| `jne` at 0x1166a6 | `[[sp+8]+16] != 0x6` | `[[sp+8]+16] == 0x6` |
| `je` at 0x1166b5 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `je` at 0x1166f7 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `jne` at 0x11671d | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) != 0x0` | `(ret(_userAcquireRemote) & ret(_userAcquireRemote)) == 0x0` |
| `je` at 0x11677c | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x1167c9 | `[0x1ee564] == [sp-8]` | `[0x1ee564] != [sp-8]` |
| `jne` at 0x1167d5 | `[[sp+8]+16] != 0x4` | `[[sp+8]+16] == 0x4` |
| `jge` at 0x116822 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1168af | `eax` | `0x0` |

## `_cmdDEST`

- `_cmdDEST`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x113c70 to 0x113d60.
  Established. The `_memcpy` site presents `[sp+8]+36`, `[sp+12]`, and `0x60` as leading candidates
  (`call` at 0x113c87). A four-byte store through `[sp+8] +0x24` retains an unknown value (`mov` at
  0x113c94). The `_strlen` site has candidate zero `[sp+8]+49` (`call` at 0x113cbf); its comparison
  branch is taken when `ret(_strlen) <= 0x46 (unsigned, 32-bit)` (`jbe` at 0x113ccf). Candidate
  values do not prove a destination-record size.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x113c74 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x113c77 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x113c7a | load | 4 | `[sp+12] +0x0` | `-` |
| `mov` at 0x113c8f | load | 4 | `[sp+8] +0x24` | `-` |
| `mov` at 0x113c94 | store | 4 | `[sp+8] +0x24` | `?` |
| `test` at 0x113c97 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x113cdd | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x113ce2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113ce9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x113cf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x113c87 | `_memcpy 0x17dda0` | `[sp+8]+36` | `[sp+12]` | `0x60` | `ebx` | `ebp` | `?` |
| `call` at 0x113caa | `_ffscMsg 0x11ebd0` | `0x113c2c` | `[sp+8]` | `[sp+8]+49` | `?` | `?` | `?` |
| `call` at 0x113cb3 | `_userUpdateSaveInfo 0x12b8b8` | `[sp+8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113cbf | `_strlen 0x172f14` | `[sp+8]+49` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x113ced | `_sprintf 0x171680` | `?` | `0x113c5e` | `[sp+8]+49` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x113c7e | `(? & 0x2) != 0x0` | `(? & 0x2) == 0x0` |
| `je` at 0x113c9e | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `jbe` at 0x113ccf | `ret(_strlen) <= 0x46 (unsigned, 32-bit)` | `ret(_strlen) > 0x46 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x113cfa | `eax` | `0x0` |

## `_ceEnterELSCMode`

- `_ceEnterELSCMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1185f8 to 0x1188a8.
  Established. Four-byte stores through `[sp+4] +0x18` write `0x3` (`mov` at 0x118648), through
  `[sp+4] +0x1c` write `0x1` (`mov` at 0x11864f), and through `[sp+4] +0x24` write `0x1` (`mov` at
  0x118656). The `_elscSendPassword` call presents `[[[sp+4]+124]+12]` and `[[[sp+4]+132]+32]` as
  its leading candidates (`call` at 0x118790). A four-byte store at absolute `0x1ebc4c` writes `0x1`
  (`mov` at 0x1186e6). Field and mode meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118603 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x118606 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11861d | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x118620 | load | 4 | `[[sp+4]+124] +0x4` | `-` |
| `mov` at 0x118628 | store | 4 | `[sp+4] +0x7c` | `0x0` |
| `mov` at 0x11862f | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118634 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118648 | store | 4 | `[sp+4] +0x18` | `0x3` |
| `mov` at 0x11864f | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x118656 | store | 4 | `[sp+4] +0x24` | `0x1` |
| `mov` at 0x11865d | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118669 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118675 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x118678 | load | 4 | `[[sp+4]+124] +0x8` | `-` |
| `mov` at 0x11867c | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x118688 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11868b | load | 4 | `[[sp+4]+124] +0x8` | `-` |
| `mov` at 0x11868f | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x11869b | load | 4 | `[sp+4] +0x50` | `-` |
| `mov` at 0x11869f | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x1186ab | load | 4 | `[sp+4] +0x50` | `-` |
| `mov` at 0x1186af | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1186bb | load | 4 | `[sp+4] +0x80` | `-` |
| `mov` at 0x1186c2 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1186ce | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x1186d1 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1186e3 | store | 4 | `[[sp+4]+124] +0x4` | `?` |
| `mov` at 0x1186e6 | store | 4 | `abs 0x1ebc4c` | `0x1` |
| `mov` at 0x1186f0 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1186f8 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11871a | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118726 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11872c | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11872f | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11873b | load | 1 | `[sp+4] +0x90` | `-` |
| `mov` at 0x11876a | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118776 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x11877c | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x11877f | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x118785 | load | 4 | `[[sp+4]+132] +0x20` | `-` |
| `mov` at 0x118789 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11878c | load | 4 | `[[sp+4]+124] +0xc` | `-` |
| `mov` at 0x1187bb | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1187c7 | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x1187d1 | load | 4 | `[sp+4] +0x84` | `-` |
| `mov` at 0x1187d7 | load | 4 | `[[sp+4]+132] +0x1c` | `-` |
| `mov` at 0x1187e5 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x1187e8 | load | 4 | `[[sp+4]+124] +0xc` | `-` |
| `mov` at 0x1187f4 | load | 4 | `[sp+4] +0x68` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118615 | `_ffscMsg 0x11ebd0` | `0x118568` | `[sp+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x118638 | `_ceEnterFFSCMode 0x1188a8` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118661 | `_bufClear 0x10f438` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11866d | `_bufClear 0x10f438` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118680 | `_bufAttachFD 0x10f378` | `[[sp+4]+104]` | `[[[sp+4]+124]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x118693 | `_bufAttachFD 0x10f378` | `[[sp+4]+108]` | `[[[sp+4]+124]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1186a3 | `_bufAttachFD 0x10f378` | `[[sp+4]+112]` | `[[sp+4]+80]` | `?` | `?` | `?` | `?` |
| `call` at 0x1186b3 | `_bufAttachFD 0x10f378` | `[[sp+4]+116]` | `[[sp+4]+80]` | `?` | `?` | `?` | `?` |
| `call` at 0x1186c6 | `_bufAttachLog 0x10f418` | `[[sp+4]+116]` | `[[sp+4]+128]` | `?` | `?` | `?` | `?` |
| `call` at 0x118707 | `_sprintf 0x171680` | `sp-20` | `0x11837d` | `[0x1ee564]` | `?` | `?` | `?` |
| `call` at 0x118710 | `_strlen 0x172f14` | `sp-20` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11871e | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-20` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x118745 | `_ffscCtrlCharToString 0x1254ac` | `?` | `sp-120` | `?` | `?` | `?` | `?` |
| `call` at 0x118757 | `_sprintf 0x171680` | `sp-100` | `0x118585` | `sp-120` | `?` | `?` | `?` |
| `call` at 0x118760 | `_strlen 0x172f14` | `sp-100` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11876e | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-100` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x118790 | `_elscSendPassword 0x121180` | `[[[sp+4]+124]+12]` | `[[[sp+4]+132]+32]` | `?` | `?` | `?` | `?` |
| `call` at 0x1187a8 | `_sprintf 0x171680` | `sp-200` | `0x1185b4` | `?` | `?` | `?` | `?` |
| `call` at 0x1187b1 | `_strlen 0x172f14` | `sp-200` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1187bf | `_bufWrite 0x10fb78` | `[[sp+4]+116]` | `sp-200` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x1187ec | `_elscDoAdminCommand 0x1202e0` | `[[[sp+4]+124]+12]` | `0x1185e4` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1187f8 | `_bufRelease 0x10fa18` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11860d | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x118626 | `[[[sp+4]+124]+4] == 0x1` | `[[[sp+4]+124]+4] != 0x1` |
| `jne` at 0x1186dc | `[[sp+4]+12] != 0x4` | `[[sp+4]+12] == 0x4` |
| `je` at 0x1186f6 | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jge` at 0x118735 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x11879a | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` |
| `je` at 0x1187cf | `([[sp+4]+132] & [[sp+4]+132]) == 0x0` | `([[sp+4]+132] & [[sp+4]+132]) != 0x0` |
| `je` at 0x1187dc | `(? & 0x4) == 0x0` | `(? & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11880a | `eax` | `?` |

## `_clLeaveELSCMode`

- `_clLeaveELSCMode`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x119598 to 0x119698.
  Established. Four-byte stores write `0x0` through `[sp+4] +0x18` (`mov` at 0x1195ed), `+0x1c`
  (`mov` at 0x1195f4), `+0x20` (`mov` at 0x1195fb), `+0x24` (`mov` at 0x119602), and `+0x28` (`mov`
  at 0x119609). A separate four-byte store writes `0x1` at absolute `0x1ebc4c` (`mov` at 0x11962d).
  The `_elscSetup` site has candidate zero `[[sp+4]+124]` (`call` at 0x119625). No field-clearing or
  mode-exit role is inferred from these literals.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1195a2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1195a5 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x1195b1 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x1195bd | load | 4 | `[sp+4] +0x6c` | `-` |
| `mov` at 0x1195c9 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x1195d5 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1195e1 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x1195ed | store | 4 | `[sp+4] +0x18` | `0x0` |
| `mov` at 0x1195f4 | store | 4 | `[sp+4] +0x1c` | `0x0` |
| `mov` at 0x1195fb | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x119602 | store | 4 | `[sp+4] +0x24` | `0x0` |
| `mov` at 0x119609 | store | 4 | `[sp+4] +0x28` | `0x0` |
| `mov` at 0x119610 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x119617 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11961a | store | 4 | `[[sp+4]+124] +0x4` | `0x1` |
| `mov` at 0x119621 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11962d | store | 4 | `abs 0x1ebc4c` | `0x1` |
| `mov` at 0x11964c | store | 4 | `[sp+4] +0x7c` | `0x0` |
| `test` at 0x119653 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x119669 | load | 4 | `sp -0x5bc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1195a9 | `_bufHold 0x10f678` | `[[sp+4]+104]` | `ebx` | `?` | `?` | `?` | `?` |
| `call` at 0x1195b5 | `_bufDetachFD 0x10f478` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1195c1 | `_bufDetachFD 0x10f478` | `[[sp+4]+108]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1195cd | `_bufDetachFD 0x10f478` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1195d9 | `_bufDetachFD 0x10f478` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1195e5 | `_bufDetachLog 0x10f4e8` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119625 | `_elscSetup 0x1213f0` | `[[sp+4]+124]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x119644 | `_cmSendFFSCCmd 0x11a290` | `[sp+4]` | `0x119566` | `sp-1464` | `?` | `?` | `?` |
| `call` at 0x119662 | `_ffscMsg 0x11ebd0` | `0x11956e` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x119615 | `([[sp+4]+124] & [[sp+4]+124]) == 0x0` | `([[sp+4]+124] & [[sp+4]+124]) != 0x0` |
| `je` at 0x11965a | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x119672 | `eax` | `0x0` |

## `_csSysInELSC`

- `_csSysInELSC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x11d090 to 0x11d400.
  Established. The four-byte stores through `[sp+4] +0x1c` include `0x3` (`mov` at 0x11d126), `0x2`
  (`mov` at 0x11d15f, `mov` at 0x11d30f), and `0x1` (`mov` at 0x11d240). Four-byte stores through
  `[sp+4] +0x20` write `0x0` (`mov` at 0x11d247) and `0x1` (`mov` at 0x11d316). The
  `_elscProcessCommand` site presents candidate zero `[[sp+4]+124]` (`call` at 0x11d224). These
  assignments do not by themselves define parser states or protocol framing.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x11d099 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x11d09c | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x11d09f | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x11d0a2 | store | 4 | `sp -0x8` | `[[sp+4]+116]` |
| `mov` at 0x11d0b0 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x11d0e0 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d0e3 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d0e6 | store | 1 | `([[[sp+4]+104]+16] + [[[sp+4]+104]+4]) +0x0` | `0x0` |
| `mov` at 0x11d0ea | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d0ed | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d0f2 | store | 4 | `sp -0xc` | `([[[sp+4]+104]+4] + [[sp+4]+32])` |
| `mov` at 0x11d106 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d109 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `sub` at 0x11d111 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x11d114 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d119 | store | 4 | `[sp+4] +0x20` | `?` |
| `mov` at 0x11d11c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x11d126 | store | 4 | `[sp+4] +0x1c` | `0x3` |
| `mov` at 0x11d12d | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d131 | store | 4 | `[sp+4] +0x20` | `?` |
| `mov` at 0x11d134 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d137 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d13a | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d13d | load | 1 | `([[sp+4]+32] + [[[sp+4]+104]+4]) +0x0` | `-` |
| `mov` at 0x11d148 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d14b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d14e | load | 1 | `([[sp+4]+32] + [[[sp+4]+104]+4]) +0x0` | `-` |
| `mov` at 0x11d155 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d159 | store | 4 | `[sp+4] +0x20` | `?` |
| `mov` at 0x11d15c | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d15f | store | 4 | `[sp+4] +0x1c` | `0x2` |
| `mov` at 0x11d170 | load | 1 | `[[[sp+4]+104]+4] +0x0` | `-` |
| `mov` at 0x11d181 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d199 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `test` at 0x11d1aa | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11d1c1 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11d1c4 | load | 4 | `[[sp+4]+124] +0x2c` | `-` |
| `mov` at 0x11d1cb | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d1cf | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d1d3 | load | 4 | `[[sp+4]+124] +0x2c` | `-` |
| `mov` at 0x11d1df | load | 4 | `[sp+4] +0x78` | `-` |
| `mov` at 0x11d1e3 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d1e9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11d210 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d213 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d216 | store | 1 | `([[[sp+4]+104]+4] + [[sp+4]+32]) -0x1` | `0x0` |
| `mov` at 0x11d21b | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d220 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11d22c | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d22f | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x11d240 | store | 4 | `[sp+4] +0x1c` | `0x1` |
| `mov` at 0x11d247 | store | 4 | `[sp+4] +0x20` | `0x0` |
| `mov` at 0x11d260 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d266 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d278 | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11d27b | load | 4 | `[[sp+4]+124] +0x2c` | `-` |
| `mov` at 0x11d282 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d286 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d28a | load | 4 | `[[sp+4]+124] +0x2c` | `-` |
| `mov` at 0x11d296 | load | 4 | `[sp+4] +0x78` | `-` |
| `mov` at 0x11d29a | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d29f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11d2c0 | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d2c5 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x11d2cc | load | 4 | `[sp+4] +0x7c` | `-` |
| `mov` at 0x11d2cf | load | 4 | `[[sp+4]+124] +0x2c` | `-` |
| `mov` at 0x11d2d6 | load | 4 | `[[sp+4]+104] +0x10` | `-` |
| `mov` at 0x11d2da | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `mov` at 0x11d2de | load | 4 | `[[sp+4]+124] +0x2c` | `-` |
| `mov` at 0x11d2ea | load | 4 | `[sp+4] +0x78` | `-` |
| `mov` at 0x11d2ee | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x11d2f3 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x11d2ff | load | 4 | `[[sp+4]+104] +0x4` | `-` |
| `add` at 0x11d302 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x11d30f | store | 4 | `[sp+4] +0x1c` | `0x2` |
| `mov` at 0x11d316 | store | 4 | `[sp+4] +0x20` | `0x1` |
| `test` at 0x11d31d | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x11d334 | load | 4 | `[[sp+4]+104] +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11d0f8 | `_strchr 0x17de58` | `([[[sp+4]+104]+4] + [[sp+4]+32])` | `0xd` | `?` | `?` | `?` | `([[[sp+4]+104]+4] + [[sp+4]+32])` |
| `call` at 0x11d186 | `_strncmp 0x1749b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d19e | `_strncmp 0x1749b8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d1b9 | `_ffscMsg 0x11ebd0` | `0x11d039` | `[sp+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d1d7 | `_logWrite 0x124de0` | `[[[sp+4]+124]+44]` | `[[[sp+4]+104]+4]` | `[[[sp+4]+104]+16]` | `?` | `?` | `?` |
| `call` at 0x11d1ed | `_bufAppendAddMsgs 0x10eea8` | `[sp-8]` | `[[sp+4]+104]` | `?` | `?` | `?` | `?` |
| `call` at 0x11d224 | `_elscProcessCommand 0x120970` | `[[sp+4]+124]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x11d238 | `_bufLeftAlign 0x10f838` | `[[sp+4]+104]` | `([[sp+4]+32] + [[[sp+4]+104]+4])+1` | `?` | `?` | `?` | `?` |
| `call` at 0x11d26a | `_memchr 0x17dd2c` | `[[[sp+4]+104]+4]` | `0x14` | `[[[sp+4]+104]+16]` | `?` | `?` | `?` |
| `call` at 0x11d28e | `_logWrite 0x124de0` | `[[[sp+4]+124]+44]` | `[[[sp+4]+104]+4]` | `[[[sp+4]+104]+16]` | `?` | `?` | `?` |
| `call` at 0x11d2a3 | `_bufAppendAddMsgs 0x10eea8` | `[sp-8]` | `[[sp+4]+104]` | `[[[sp+4]+104]+16]` | `[[sp+4]+120]` | `?` | `?` |
| `call` at 0x11d2e2 | `_logWrite 0x124de0` | `[[[sp+4]+124]+44]` | `[[[sp+4]+104]+4]` | `[[[sp+4]+104]+16]` | `?` | `?` | `?` |
| `call` at 0x11d2f7 | `_bufAppendAddMsgs 0x10eea8` | `[sp-8]` | `[[sp+4]+104]` | `[sp-12]` | `[[sp+4]+120]` | `?` | `?` |
| `call` at 0x11d307 | `_bufLeftAlign 0x10f838` | `[[sp+4]+104]` | `([[[sp+4]+104]+4] + [sp-12])` | `?` | `?` | `?` | `?` |
| `call` at 0x11d32c | `_ffscMsg 0x11ebd0` | `0x11d068` | `[sp+4]` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11d0b6 | `[[sp+4]+28] == 0x2` | `[[sp+4]+28] != 0x2` |
| `jg` at 0x11d0b8 | `[[sp+4]+28] > 0x2 (signed, 32-bit)` | `[[sp+4]+28] <= 0x2 (signed, 32-bit)` |
| `je` at 0x11d0bd | `[[sp+4]+28] == 0x1` | `[[sp+4]+28] != 0x1` |
| `je` at 0x11d0d3 | `[[sp+4]+28] == 0x3` | `[[sp+4]+28] != 0x3` |
| `jne` at 0x11d104 | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x11d120 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x11d142 | `(([([[sp+4]+32] + [[[sp+4]+104]+4])]:1 & 0xff) & ([([[sp+4]+32] + [[[sp+4]+104]+4])]:1 & 0xff)) == 0x0` | `(([([[sp+4]+32] + [[[sp+4]+104]+4])]:1 & 0xff) & ([([[sp+4]+32] + [[[sp+4]+104]+4])]:1 & 0xff)) != 0x0` |
| `je` at 0x11d153 | `([([[sp+4]+32] + [[[sp+4]+104]+4])]:1 & 0xff) == 0xa` | `([([[sp+4]+32] + [[[sp+4]+104]+4])]:1 & 0xff) != 0xa` |
| `jne` at 0x11d174 | `([[[[sp+4]+104]+4]]:1 & 0xff) != 0x14` | `([[[[sp+4]+104]+4]]:1 & 0xff) == 0x14` |
| `je` at 0x11d190 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x11d1a8 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x11d1b1 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x11d1c9 | `([[[sp+4]+124]+44] & [[[sp+4]+124]+44]) == 0x0` | `([[[sp+4]+124]+44] & [[[sp+4]+124]+44]) != 0x0` |
| `jge` at 0x11d1f7 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x11d276 | `(ret(_memchr) & ret(_memchr)) != 0x0` | `(ret(_memchr) & ret(_memchr)) == 0x0` |
| `je` at 0x11d280 | `([[[sp+4]+124]+44] & [[[sp+4]+124]+44]) == 0x0` | `([[[sp+4]+124]+44] & [[[sp+4]+124]+44]) != 0x0` |
| `jge` at 0x11d2ad | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x11d2ca | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `je` at 0x11d2d4 | `([[[sp+4]+124]+44] & [[[sp+4]+124]+44]) == 0x0` | `([[[sp+4]+124]+44] & [[[sp+4]+124]+44]) != 0x0` |
| `je` at 0x11d324 | `(([0x1da698]:1 & 0xff) & 0x4) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x4) != 0x0` |
| `jg` at 0x11d339 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x11d34a | `eax` | `?` |
