# MMSC tty and controller-link paths

The exact images below use their linked a.out addresses and load 0x108000. Runtime
placement remains unknown. Symbol-table extents use the next symbol in the same segment
and are inferred analysis windows. They do not establish complete function boundaries.
Roles and pointee meanings remain unknown.

The assumed cdecl32 convention supplies six candidate stack locations for every call.
Actual arity and custom conventions remain unknown. Bare general-register expressions
denote their entry values. A ret(F) expression alone does not identify a producing
instruction. Unsupported effects retain unknown values, effective addresses or flag
operands. Unreached ranges refer to the analyzer's modeled edges. Static access groups
establish neither equal loaded values nor one path across branch arms.

The [complete facts](disasm/x86-variants/tty-controller-link-paths-facts.txt) preserve warnings, frame accesses, unreached instruction records and undecoded-tail reports. Each carrier's header, mapped segments and symbol-table identity were checked against its exact bytes.

- `_routeProcessELSCCommand`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1299c4`.
  Role: unknown. Inferred analysis span: 0x1299c4 to 0x129ab4 (exclusive), 240 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 722, file offset 0x219e4.
  Read window: SHA256 `5794644199e481b18289e9de78ca8173d417d175b284ac40ac5479c6f9f5cef1`.
  Four-byte load: target `sp +0xc` (`mov` at 0x1299ca).
  Grouped accesses: 2 4-byte load records with target `abs 0x1f0d0c`, including `mov` at 0x1299cd, `mov` at 0x1299e0.
  Byte load: target `([0x1f0d0c] + [sp+12]) +0x4` (`mov` at 0x1299d2).
  Branch (`je` at 0x1299d8): taken to 0x129a49 when `(([([0x1f0d0c] + [sp+12])+4]:1 & 0xff) & ([([0x1f0d0c] + [sp+12])+4]:1 & 0xff)) == 0x0`, falls through to 0x1299da when `(([([0x1f0d0c] + [sp+12])+4]:1 & 0xff) & ([([0x1f0d0c] + [sp+12])+4]:1 & 0xff)) != 0x0`.
  Grouped accesses: 2 1-byte load records with target `([0x1f0d0c] + [sp+12]) +0x4`, including `mov` at 0x1299e6, `mov` at 0x129a14.
  Branch (`jae` at 0x1299f2): taken to 0x129a14 when `C == 0`, falls through to 0x1299f4 when `not (C == 0)`.
  Four-byte load: target `sp +0x8` (`mov` at 0x1299f4).
  Four-byte load: target `[sp+8] +0x20` (`mov` at 0x1299f7).
  Call (`call` at 0x1299fc): `_elscSendPassword`, target 0x121820, stack arg0=`unknown`, stack arg1=`[[sp+8]+32]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0x4` (`mov` at 0x129a04).
  Call (`call` at 0x129a09): `_elscSendCommand`, target 0x121680, stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x129a11): target 0x129a40.
  The instructions from 0x129a13 to 0x129a13 are not reached by any path from the entry, and their branches are not listed.
  Branch (`jge` at 0x129a1a): taken to 0x129a40 when `ge flags condition (operands unknown)`, falls through to 0x129a1c when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `esi`, including `mov` at 0x129a21, `mov` at 0x129a33.
  Four-byte load: target `(unknown effective address)`, base register `eax` (`mov` at 0x129a27).
  Call (`call` at 0x129a2b): `_sprintf`, target 0x172ed8, stack arg0=`unknown`, stack arg1=`0x1299ba`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `0xb`, base register `eax` (`mov` at 0x129a39).
  Branch (`jbe` at 0x129a47): taken to 0x1299e0 when `unknown <= 0x1 (unsigned, 32-bit)`, falls through to 0x129a49 when `unknown > 0x1 (unsigned, 32-bit)`.
  At the return (`ret` at 0x129a52), eax is `unknown`.
  The instructions from 0x129a53 to 0x129ab3 are not reached by any path from the entry, and their branches are not listed.

- `_cmdTTY_REINIT`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1185a0`.
  Role: unknown. Inferred analysis span: 0x1185a0 to 0x118668 (exclusive), 200 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 630, file offset 0x105c0.
  Read window: SHA256 `1b549a873537a82151a4602ea0c80540b15c1979e23c483ee3407b70f617c66f`.
  Four-byte load: target `sp +0x8` (`mov` at 0x1185a3).
  Four-byte load: target `sp +0xc` (`mov` at 0x1185a6).
  Four-byte load: target `[sp+8] +0x20` (`mov` at 0x1185a9).
  Branch (`jg` at 0x1185b1): taken to 0x1185e0 when `[[sp+8]+32] > 0x3e7 (signed, 32-bit)`, falls through to 0x1185b3 when `[[sp+8]+32] <= 0x3e7 (signed, 32-bit)`.
  Grouped accesses: 4 4-byte load records with target `abs 0x1f0d0c`, including `mov` at 0x1185b8, `mov` at 0x1185e0, `mov` at 0x11862a.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, including `mov` at 0x1185bd, `mov` at 0x118635, `mov` at 0x11864a.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `eax`, including `mov` at 0x1185c4, `mov` at 0x11863c.
  Call (`call` at 0x1185c8): `_sprintf`, target 0x172ed8, stack arg0=`unknown`, stack arg1=`0x116e80`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`ret` at 0x1185d2) reports eax = `0x0`.
  The instructions from 0x1185d3 to 0x1185df are not reached by any path from the entry, and their branches are not listed.
  Byte load: target `([0x1f0d0c] + [sp+12]) +0x4` (`mov` at 0x1185e5).
  Branch (`jne` at 0x1185eb): taken to 0x118600 when `(([([0x1f0d0c] + [sp+12])+4]:1 & 0xff) & ([([0x1f0d0c] + [sp+12])+4]:1 & 0xff)) != 0x0`, falls through to 0x1185ed when `(([([0x1f0d0c] + [sp+12])+4]:1 & 0xff) & ([([0x1f0d0c] + [sp+12])+4]:1 & 0xff)) == 0x0`.
  Recorded eax at the return is `0x1` (`ret` at 0x1185f5).
  The instructions from 0x1185f6 to 0x1185ff are not reached by any path from the entry, and their branches are not listed.
  Call (`call` at 0x118602): `_taskDelay`, target 0x1723d8, stack arg0=`0x3c`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11860a): `_taskLock`, target 0x1722a8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x11860f): `_ttyReInit`, target 0x12bba4, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x118616): taken to 0x118625 when `(ret(_ttyReInit) & ret(_ttyReInit)) == 0x0`, falls through to 0x118618 when `(ret(_ttyReInit) & ret(_ttyReInit)) != 0x0`.
  Call (`call` at 0x11861d): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x11856b`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x118625): `_taskUnlock`, target 0x1722d8, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x118640): `_sprintf`, target 0x172ed8, stack arg0=`unknown`, stack arg1=`0x116ff9`, stack arg2=`[0x1f0d0c]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `0xb`, base register `eax` (`mov` at 0x118651).
  `ret` at 0x118660 has recorded eax = `0x1`.
  The instructions from 0x118661 to 0x118666 are not reached by any path from the entry, and their branches are not listed.

- `_cmCheckTTYFlags`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x11a080`.
  Role: unknown. Inferred analysis span: 0x11a080 to 0x11a230 (exclusive), 432 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 759, file offset 0x120a0.
  Read window: SHA256 `a9bd3c8b731ecf86ca39250d6d46ed4c36e03fbf94c943d744aa023f0f1c526d`.
  Four-byte load: target `sp +0x4` (`mov` at 0x11a089).
  Grouped accesses: 10 4-byte load records with target `[sp+4] +0x3c`, including `mov` at 0x11a08c, `mov` at 0x11a0be, `mov` at 0x11a0d0.
  Call (`call` at 0x11a099): `_ttyGetInfo`, target 0x12b754, stack arg0=`unknown`, stack arg1=`sp-8`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x11a0a5): taken to 0x11a16b when `(ret(_ttyGetInfo) & ret(_ttyGetInfo)) == 0x0`, falls through to 0x11a0ab when `(ret(_ttyGetInfo) & ret(_ttyGetInfo)) != 0x0`.
  Four-byte load: target `[sp+4] +0x70` (`mov` at 0x11a0ab).
  Four-byte load: target `[[sp+4]+112] +0x0` (`mov` at 0x11a0ae).
  Branch (`jne` at 0x11a0b3): taken to 0x11a10b when `[sp-8] != [[[sp+4]+112]]`, falls through to 0x11a0b5 when `[sp-8] == [[[sp+4]+112]]`.
  Grouped accesses: 4 2-byte load records with target `ret(_ttyGetInfo) +0x4`, including `mov` at 0x11a0b5, `mov` at 0x11a0db, `mov` at 0x11a115.
  Branch (`je` at 0x11a0bc): taken to 0x11a0d0 when `(unknown & 0x20) == 0x0`, falls through to 0x11a0be when `(unknown & 0x20) != 0x0`.
  Jump (`jmp` at 0x11a0c6): target 0x11a0d8.
  The instructions from 0x11a0c8 to 0x11a0cf are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 4 4-byte store records with target `[sp+4] +0x3c`, reported value `unknown` in every record, including `mov` at 0x11a0d8, `mov` at 0x11a108, `mov` at 0x11a138.
  Branch (`je` at 0x11a0e2): taken to 0x11a100 when `(unknown & 0x10) == 0x0`, falls through to 0x11a0e4 when `(unknown & 0x10) != 0x0`.
  Grouped accesses: 2 1-byte load records with target `abs 0x1dcdcb`, including `test` at 0x11a0e4, `test` at 0x11a144.
  Branch (`jne` at 0x11a0eb): taken to 0x11a100 when `(([0x1dcdcb]:1 & 0xff) & 0x10) != 0x0`, falls through to 0x11a0ed when `(([0x1dcdcb]:1 & 0xff) & 0x10) == 0x0`.
  Jump (`jmp` at 0x11a0f5): target 0x11a108.
  The instructions from 0x11a0f7 to 0x11a0ff are not reached by any path from the entry, and their branches are not listed.
  Four-byte load: target `[sp+4] +0x68` (`mov` at 0x11a10b).
  Four-byte load: target `[[sp+4]+104] +0x0` (`mov` at 0x11a10e).
  Branch (`jne` at 0x11a113): taken to 0x11a16b when `[sp-8] != [[[sp+4]+104]]`, falls through to 0x11a115 when `[sp-8] == [[[sp+4]+104]]`.
  Branch (`je` at 0x11a11c): taken to 0x11a130 when `(unknown & 0x20) == 0x0`, falls through to 0x11a11e when `(unknown & 0x20) != 0x0`.
  Jump (`jmp` at 0x11a126): target 0x11a138.
  The instructions from 0x11a128 to 0x11a12f are not reached by any path from the entry, and their branches are not listed.
  Branch (`je` at 0x11a142): taken to 0x11a160 when `(unknown & 0x10) == 0x0`, falls through to 0x11a144 when `(unknown & 0x10) != 0x0`.
  Branch (`jne` at 0x11a14b): taken to 0x11a160 when `(([0x1dcdcb]:1 & 0xff) & 0x20) != 0x0`, falls through to 0x11a14d when `(([0x1dcdcb]:1 & 0xff) & 0x20) == 0x0`.
  Jump (`jmp` at 0x11a155): target 0x11a168.
  The instructions from 0x11a157 to 0x11a15f are not reached by any path from the entry, and their branches are not listed.
  Branch (`jle` at 0x11a16f): taken to 0x11a094 when `unknown <= 0x5 (signed, 32-bit)`, falls through to 0x11a175 when `unknown > 0x5 (signed, 32-bit)`.
  Recorded eax at the return is `unknown` (`ret` at 0x11a18b).
  The instructions from 0x11a18c to 0x11a22f are not reached by any path from the entry, and their branches are not listed.

- `_ttyGetInfo`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12b754`.
  Role: unknown. Inferred analysis span: 0x12b754 to 0x12b794 (exclusive), 64 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 782, file offset 0x23774.
  Read window: SHA256 `2928274c8946df3dbf0ac7033d8cadd2b20f15eaa2b57a0d3067922c2cdd92bf`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12b757).
  Four-byte load: target `sp +0x8` (`mov` at 0x12b75a).
  Branch (`jbe` at 0x12b760): taken to 0x12b774 when `[sp+4] <= 0x5 (unsigned, 32-bit)`, falls through to 0x12b762 when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  `ret` at 0x12b767 has recorded eax = `0x0`.
  The instructions from 0x12b768 to 0x12b773 are not reached by any path from the entry, and their branches are not listed.
  Branch (`je` at 0x12b776): taken to 0x12b781 when `([sp+8] & [sp+8]) == 0x0`, falls through to 0x12b778 when `([sp+8] & [sp+8]) != 0x0`.
  Four-byte load: target `(unknown effective address)` (`mov` at 0x12b778).
  Four-byte store: target `[sp+8] +0x0`, value `unknown` (`mov` at 0x12b77f).
  Return (`ret` at 0x12b78e): eax = `unknown`.
  The instructions from 0x12b78f to 0x12b793 are not reached by any path from the entry, and their branches are not listed.

- `_ttyGetRxBuf`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12b794`.
  Role: unknown. Inferred analysis span: 0x12b794 to 0x12b7e4 (exclusive), 80 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 1066, file offset 0x237b4.
  Read window: SHA256 `d6fa3db16b8f23f67d320a07105fcd63fdfe6abd0de4e9bdd1ab02da7279545d`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12b797).
  Branch (`jbe` at 0x12b79d): taken to 0x12b7b4 when `[sp+4] <= 0x5 (unsigned, 32-bit)`, falls through to 0x12b79f when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  Return (`ret` at 0x12b7a7): eax = `0xffffffff`.
  The instructions from 0x12b7a8 to 0x12b7b3 are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `eax`, including `cmp` at 0x12b7be, `mov` at 0x12b7c5, `mov` at 0x12b7d9.
  Branch (`je` at 0x12b7c3): taken to 0x12b7d4 when `unknown == 0x0`, falls through to 0x12b7c5 when `unknown != 0x0`.
  At the exit (`ret` at 0x12b7cc), eax is `unknown`.
  The instructions from 0x12b7cd to 0x12b7d3 are not reached by any path from the entry, and their branches are not listed.
  At the return (`ret` at 0x12b7e0), eax is `unknown`.
  The instructions from 0x12b7e1 to 0x12b7e3 are not reached by any path from the entry, and their branches are not listed.

- `_ttyGetTxBuf`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12b7e4`.
  Role: unknown. Inferred analysis span: 0x12b7e4 to 0x12b8d4 (exclusive), 240 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 1067, file offset 0x23804.
  Read window: SHA256 `f730ce24ff4667ae7d2b2e7207d0efd9edd3cee886b8eee9a4585bf89fc60750`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12b7e7).
  Branch (`jbe` at 0x12b7ed): taken to 0x12b804 when `[sp+4] <= 0x5 (unsigned, 32-bit)`, falls through to 0x12b7ef when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  At the exit (`ret` at 0x12b7f7), eax is `0xffffffff`.
  The instructions from 0x12b7f8 to 0x12b803 are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `eax`, including `cmp` at 0x12b80e, `mov` at 0x12b815, `mov` at 0x12b829.
  Branch (`je` at 0x12b813): taken to 0x12b824 when `unknown == 0x0`, falls through to 0x12b815 when `unknown != 0x0`.
  At the return (`ret` at 0x12b81c), eax is `unknown`.
  The instructions from 0x12b81d to 0x12b823 are not reached by any path from the entry, and their branches are not listed.
  The exit (`ret` at 0x12b830) reports eax = `unknown`.
  The instructions from 0x12b831 to 0x12b8d0 are not reached by any path from the entry, and their branches are not listed.
  Decoding stops at 0x12b8d2 with bytes `0090` remaining before 0x12b8d4 (exclusive). These bytes are not decoded instructions, and this window does not establish a complete function.

- `_oobChecksumMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12782c`.
  Role: unknown. Inferred analysis span: 0x12782c to 0x12793c (exclusive), 272 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 734, file offset 0x1f84c.
  Read window: SHA256 `1e010456386865e784506394472d93122e638b61b4d340ad3e908973099b5571`.
  Four-byte load: target `sp +0x4` (`mov` at 0x127831).
  Byte load: target `[sp+4] +0x2` (`mov` at 0x127834).
  Byte load: target `[sp+4] +0x3` (`mov` at 0x12783c).
  Branch (`jge` at 0x127855): taken to 0x127867 when `0x1 >= unknown (signed, 32-bit)`, falls through to 0x127857 when `0x1 < unknown (signed, 32-bit)`.
  Byte load: target `(unknown effective address)`, base register `edx` (`mov` at 0x127858).
  Branch (`jl` at 0x127865): taken to 0x127858 when `unknown < unknown (signed, 32-bit)`, falls through to 0x127867 when `unknown >= unknown (signed, 32-bit)`.
  At the return (`ret` at 0x127872), eax is `unknown`.
  The instructions from 0x127873 to 0x12793b are not reached by any path from the entry, and their branches are not listed.

- `_oobProcessMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12793c`.
  Role: unknown. Inferred analysis span: 0x12793c to 0x127acc (exclusive), 400 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 828, file offset 0x1f95c.
  Read window: SHA256 `8c7870d9ea23a249a754b82d65839566ff55ad4e268669e2479546f7be44c242`.
  Four-byte load: target `sp +0x4` (`mov` at 0x127941).
  Four-byte load: target `sp +0x8` (`mov` at 0x127944).
  Branch (`jne` at 0x127949): taken to 0x12795c when `([sp+8] & [sp+8]) != 0x0`, falls through to 0x12794b when `([sp+8] & [sp+8]) == 0x0`.
  Call (`call` at 0x127951): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x127873`, stack arg1=`[sp+4]`, stack arg2=`ebx`, stack arg3=`esi`, stack arg4=`ebp`, stack arg5=`unknown`.
  Jump (`jmp` at 0x127956): target 0x127a9c.
  The instructions from 0x12795b to 0x12795b are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 2 1-byte load records with target `[sp+8] +0x0`, including `mov` at 0x12795c, `mov` at 0x127962.
  Branch (`je` at 0x127960): taken to 0x12797c when `([[sp+8]]:1 & 0xff) == 0xa0`, falls through to 0x127962 when `([[sp+8]]:1 & 0xff) != 0xa0`.
  Call (`call` at 0x127970): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x12789d`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`ebp`.
  Jump (`jmp` at 0x127975): target 0x127a9c.
  The instructions from 0x12797a to 0x12797b are not reached by any path from the entry, and their branches are not listed.
  Call (`call` at 0x12797d): `_oobChecksumMsg`, target 0x12782c, stack arg0=`[sp+8]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 1-byte load records with target `[sp+8] +0x2`, including `mov` at 0x127987, `mov` at 0x1279aa.
  Grouped accesses: 2 1-byte load records with target `[sp+8] +0x3`, including `mov` at 0x12798f, `mov` at 0x1279b2.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `eax`, including `mov` at 0x12799f, `mov` at 0x1279c2.
  Branch (`je` at 0x1279a4): taken to 0x1279ec when `unknown == unknown`, falls through to 0x1279a6 when `unknown != unknown`.
  Call (`call` at 0x1279d1): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x1278d7`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x1279de): target 0x127a91.
  The instructions from 0x1279e3 to 0x1279eb are not reached by any path from the entry, and their branches are not listed.
  Byte load: target `abs 0x1dcdc9` (`test` at 0x1279ec).
  Branch (`je` at 0x1279f3): taken to 0x127a0c when `(([0x1dcdc9]:1 & 0xff) & 0x8) == 0x0`, falls through to 0x1279f5 when `(([0x1dcdc9]:1 & 0xff) & 0x8) != 0x0`.
  Grouped accesses: 2 1-byte load records with target `[sp+8] +0x1`, including `mov` at 0x1279f5, `mov` at 0x127a0c.
  Call (`call` at 0x127a04): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x127916`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jg` at 0x127a17): taken to 0x127a2c when `unknown > 0xb (signed, 32-bit)`, falls through to 0x127a19 when `unknown <= 0xb (signed, 32-bit)`.
  Branch (`jge` at 0x127a1c): taken to 0x127a4c when `unknown >= 0x1 (signed, 32-bit)`, falls through to 0x127a1e when `unknown < 0x1 (signed, 32-bit)`.
  Branch (`je` at 0x127a20): taken to 0x127a3c when `(unknown & unknown) == 0x0`, falls through to 0x127a22 when `(unknown & unknown) != 0x0`.
  Jump (`jmp` at 0x127a22): target 0x127a8c.
  The instructions from 0x127a24 to 0x127a2b are not reached by any path from the entry, and their branches are not listed.
  Branch (`je` at 0x127a2f): taken to 0x127a5c when `unknown == 0x3d`, falls through to 0x127a31 when `unknown != 0x3d`.
  Branch (`je` at 0x127a34): taken to 0x127a7c when `unknown == 0x3e`, falls through to 0x127a36 when `unknown != 0x3e`.
  Jump (`jmp` at 0x127a36): target 0x127a8c.
  The instructions from 0x127a38 to 0x127a3b are not reached by any path from the entry, and their branches are not listed.
  Jump (`jmp` at 0x127a41): target 0x127a91.
  The instructions from 0x127a43 to 0x127a4b are not reached by any path from the entry, and their branches are not listed.
  Call (`call` at 0x127a4e): `_oobSendGraphicsCmd`, target 0x127bec, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x127a53): target 0x127a9c.
  The instructions from 0x127a55 to 0x127a5b are not reached by any path from the entry, and their branches are not listed.
  Four-byte load: target `[sp+4] +0x3c` (`mov` at 0x127a5c).
  Branch (`jne` at 0x127a64): taken to 0x127a8c when `([[sp+4]+60] & 0x40000) != 0x0`, falls through to 0x127a66 when `([[sp+4]+60] & 0x40000) == 0x0`.
  Call (`call` at 0x127a6b): `_oobSendFFSCCmd`, target 0x127acc, stack arg0=`[sp+4]`, stack arg1=`[sp+8]+4`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x127a70): target 0x127a9c.
  The instructions from 0x127a72 to 0x127a7b are not reached by any path from the entry, and their branches are not listed.
  Jump (`jmp` at 0x127a81): target 0x127a91.
  The instructions from 0x127a83 to 0x127a8b are not reached by any path from the entry, and their branches are not listed.
  Four-byte load: target `[sp+4] +0x6c` (`mov` at 0x127a91).
  Four-byte load: target `[[sp+4]+108] +0x0` (`mov` at 0x127a94).
  Call (`call` at 0x127a97): `_oobSendMsg`, target 0x127d3c, stack arg0=`[[[sp+4]+108]]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`ret` at 0x127aa4) reports eax = `unknown`.
  The instructions from 0x127aa5 to 0x127acb are not reached by any path from the entry, and their branches are not listed.

- `_oobSendGraphicsCmd`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127bec`.
  Role: unknown. Inferred analysis span: 0x127bec to 0x127d3c (exclusive), 336 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 1003, file offset 0x1fc0c.
  Read window: SHA256 `c82c85394c5ef1046971a60b3abe9cd9d15c14a0c11fcd5444482e4aad2a5a85`.
  Four-byte load: target `sp +0x4` (`mov` at 0x127bf8).
  Four-byte load: target `sp +0x8` (`mov` at 0x127bfb).
  Call (`call` at 0x127c12): `_sprintf`, target 0x172ed8, stack arg0=`sp-248`, stack arg1=`0x127bc9`, stack arg2=`[sp+8]`, stack arg3=`sp-2120`, stack arg4=`ebx`, stack arg5=`esi`.
  Call (`call` at 0x127c23): `_cmSendFFSCCmd`, target 0x11a6f0, stack arg0=`[sp+4]`, stack arg1=`sp-248`, stack arg2=`sp-332`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Byte load: target `[sp+4] +0x93` (`mov` at 0x127c2b).
  Call (`call` at 0x127c36): `_strchr`, target 0x180308, stack arg0=`sp-332`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x127c40): taken to 0x127c4c when `(ret(_strchr) & ret(_strchr)) != 0x0`, falls through to 0x127c42 when `(ret(_strchr) & ret(_strchr)) == 0x0`.
  Jump (`jmp` at 0x127c44): target 0x127c4d.
  The instructions from 0x127c46 to 0x127c4b are not reached by any path from the entry, and their branches are not listed.
  Call (`call` at 0x127c55): `_strncmp`, target 0x176e68, stack arg0=`unknown`, stack arg1=`0x127be7`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x127c5f): taken to 0x127cce when `(ret(_strncmp) & ret(_strncmp)) == 0x0`, falls through to 0x127c61 when `(ret(_strncmp) & ret(_strncmp)) != 0x0`.
  Call (`call` at 0x127c68): `_strlen`, target 0x17476c, stack arg0=`sp-332`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x127c87): taken to 0x127c8f when `ge flags condition (operands unknown)`, falls through to 0x127c89 when `not (ge flags condition (operands unknown))`.
  Call (`call` at 0x127cae): `_strcpy`, target 0x18048c, stack arg0=`sp-2116`, stack arg1=`sp-332`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x127cbf): `_oobChecksumMsg`, target 0x12782c, stack arg0=`sp-2120`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `[sp+4] +0x6c` (`mov` at 0x127cd5).
  Four-byte load: target `[[sp+4]+108] +0x0` (`mov` at 0x127cd8).
  Call (`call` at 0x127cdb): `_oobSendMsg`, target 0x127d3c, stack arg0=`[[[sp+4]+108]]`, stack arg1=`sp-2120`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax at the return is `ret(_oobSendMsg)` (`ret` at 0x127cec).
  The instructions from 0x127ced to 0x127d3b are not reached by any path from the entry, and their branches are not listed.

- `_oobSendFFSCCmd`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127acc`.
  Role: unknown. Inferred analysis span: 0x127acc to 0x127bec (exclusive), 288 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 1004, file offset 0x1faec.
  Read window: SHA256 `3b3a0df4cc1f0c5cb0f4e8a6d71a68b5ffb2d974abeb6951df22147cd2771d49`.
  Four-byte load: target `sp +0x4` (`mov` at 0x127ad8).
  Four-byte load: target `sp +0x8` (`mov` at 0x127adb).
  Call (`call` at 0x127af5): `_cmSendFFSCCmdNoPB`, target 0x11a910, stack arg0=`[sp+4]`, stack arg1=`[sp+8]`, stack arg2=`sp-2032`, stack arg3=`ebx`, stack arg4=`esi`, stack arg5=`edi`.
  Byte load: target `[sp+4] +0x92` (`mov` at 0x127afd).
  Call (`call` at 0x127b08): `_strchr`, target 0x180308, stack arg0=`sp-2032`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x127b14): taken to 0x127b34 when `(ret(_strchr) & ret(_strchr)) == 0x0`, falls through to 0x127b16 when `(ret(_strchr) & ret(_strchr)) != 0x0`.
  Call (`call` at 0x127b21): `_strcpy`, target 0x18048c, stack arg0=`sp-248`, stack arg1=`ret(_strchr)+1`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `[sp+4] +0x34` (`mov` at 0x127b29).
  Branch (`je` at 0x127b2f): taken to 0x127b34 when `[[sp+4]+52] == 0x3`, falls through to 0x127b31 when `[[sp+4]+52] != 0x3`.
  Byte store: target `ret(_strchr) +0x0`, value `0x0` (`mov` at 0x127b31).
  Call (`call` at 0x127b35): `_strlen`, target 0x17476c, stack arg0=`sp-2032`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x127b46): taken to 0x127b4e when `ge flags condition (operands unknown)`, falls through to 0x127b48 when `not (ge flags condition (operands unknown))`.
  Call (`call` at 0x127b6a): `_oobChecksumMsg`, target 0x12782c, stack arg0=`sp-2036`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`ret(_strlen)+1`, stack arg5=`unknown`.
  Four-byte load: target `[sp+4] +0x6c` (`mov` at 0x127b80).
  Four-byte load: target `[[sp+4]+108] +0x0` (`mov` at 0x127b83).
  Call (`call` at 0x127b86): `_oobSendMsg`, target 0x127d3c, stack arg0=`[[[sp+4]+108]]`, stack arg1=`sp-2036`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x127b90): taken to 0x127bbc when `(ret(_strchr) & ret(_strchr)) == 0x0`, falls through to 0x127b92 when `(ret(_strchr) & ret(_strchr)) != 0x0`.
  Byte load: target `abs 0x1dcdc8` (`test` at 0x127b92).
  Branch (`je` at 0x127b99): taken to 0x127baf when `(([0x1dcdc8]:1 & 0xff) & 0x4) == 0x0`, falls through to 0x127b9b when `(([0x1dcdc8]:1 & 0xff) & 0x4) != 0x0`.
  Call (`call` at 0x127ba7): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x127aa5`, stack arg1=`sp-248`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x127bb7): `_crProcessCmd`, target 0x11c170, stack arg0=`[sp+4]`, stack arg1=`sp-248`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `ret` at 0x127bc8 has recorded eax = `unknown`.
  The instructions from 0x127bc9 to 0x127be6 are not reached by any path from the entry, and their branches are not listed.
  Decoding stops at 0x127be9 with bytes `009090` remaining before 0x127bec (exclusive). These bytes are not decoded instructions, and this window does not establish a complete function.

- `_oobSendMsg`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x127d3c`.
  Role: unknown. Inferred analysis span: 0x127d3c to 0x127dd0 (exclusive), 148 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 1005, file offset 0x1fd5c.
  Read window: SHA256 `b7e658b97f9f84bd009fe2500ae82f4ab8a16cdc8a5b23f68e226d8f9b1d8481`.
  Four-byte load: target `sp +0x8` (`mov` at 0x127d42).
  Byte load: target `[sp+8] +0x2` (`mov` at 0x127d45).
  Byte load: target `[sp+8] +0x3` (`mov` at 0x127d4d).
  Branch (`jle` at 0x127d61): taken to 0x127dc4 when `le flags condition (operands unknown)`, falls through to 0x127d63 when `not (le flags condition (operands unknown))`.
  Four-byte load: target `abs 0x1dc8b8` (`mov` at 0x127d64).
  Grouped accesses: 2 4-byte load records with target `sp +0x4`, including `mov` at 0x127d6f, `mov` at 0x127d9c.
  Call (`call` at 0x127d73): `_timeoutWrite`, target 0x12b198, stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`[0x1dc8b8]`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x127d7d): taken to 0x127dbc when `ge flags condition (operands unknown)`, falls through to 0x127d7f when `not (ge flags condition (operands unknown))`.
  Call (`call` at 0x127d7f): `___errno`, target 0x1712d0, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `ret(___errno) +0x0` (`mov` at 0x127d84).
  Branch (`jne` at 0x127d89): taken to 0x127d9c when `[ret(___errno)] != 0x4`, falls through to 0x127d8b when `[ret(___errno)] == 0x4`.
  Call (`call` at 0x127d90): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x127ced`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x127d95): target 0x127daa.
  The instructions from 0x127d97 to 0x127d9b are not reached by any path from the entry, and their branches are not listed.
  Call (`call` at 0x127da5): `_ffscMsgS`, target 0x11f200, stack arg0=`0x127d0b`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x127daf): target 0x127dc6.
  The instructions from 0x127db1 to 0x127dbb are not reached by any path from the entry, and their branches are not listed.
  Branch (`jg` at 0x127dc2): taken to 0x127d64 when `g flags condition (operands unknown)`, falls through to 0x127dc4 when `not (g flags condition (operands unknown))`.
  Return (`ret` at 0x127dcf): eax = `unknown`.

- `_ffscParseRackID`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x125a98`.
  Role: unknown. Inferred analysis span: 0x125a98 to 0x125b38 (exclusive), 160 bytes.
  Carrier SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`, symbol index 698, file offset 0x1dab8.
  Read window: SHA256 `0034272c9041bc8e95eac467eaaa15687ea1a34f27f49a3588d0d0ac963f775d`.
  Four-byte load: target `sp +0x4` (`mov` at 0x125aa0).
  Four-byte load: target `sp +0x8` (`mov` at 0x125aa3).
  Call (`call` at 0x125ab4): `_sscanf`, target 0x173c88, stack arg0=`[sp+4]`, stack arg1=`0x125a7c`, stack arg2=`sp-8`, stack arg3=`sp-9`, stack arg4=`ebx`, stack arg5=`esi`.
  Branch (`jne` at 0x125abf): taken to 0x125ac7 when `ret(_sscanf) != 0x1`, falls through to 0x125ac1 when `ret(_sscanf) == 0x1`.
  Branch (`jbe` at 0x125ac5): taken to 0x125ae8 when `[sp-8] <= 0xa (unsigned, 32-bit)`, falls through to 0x125ac7 when `[sp-8] > 0xa (unsigned, 32-bit)`.
  Call (`call` at 0x125acd): `_ffscMsg`, target 0x11f0d0, stack arg0=`0x125a81`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x125ad7): target 0x125af3.
  The instructions from 0x125ad9 to 0x125ae7 are not reached by any path from the entry, and their branches are not listed.
  Branch (`je` at 0x125aea): taken to 0x125af1 when `([sp+8] & [sp+8]) == 0x0`, falls through to 0x125aec when `([sp+8] & [sp+8]) != 0x0`.
  Four-byte store: target `[sp+8] +0x0`, value `[sp-8]` (`mov` at 0x125aef).
  At the exit (`ret` at 0x125afb), eax is `unknown`.
  The instructions from 0x125afc to 0x125b37 are not reached by any path from the entry, and their branches are not listed.

- `_oobChecksumMsg`, `6.4/usr/cpu/firmware/mmscfw.bin`, `0x12716c`.
  Role: unknown. Inferred analysis span: 0x12716c to 0x12727c (exclusive), 272 bytes.
  Carrier SHA256 `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`, symbol index 731, file offset 0x1f18c.
  Read window: SHA256 `1e010456386865e784506394472d93122e638b61b4d340ad3e908973099b5571`.
  Four-byte load: target `sp +0x4` (`mov` at 0x127171).
  Byte load: target `[sp+4] +0x2` (`mov` at 0x127174).
  Byte load: target `[sp+4] +0x3` (`mov` at 0x12717c).
  Branch (`jge` at 0x127195): taken to 0x1271a7 when `0x1 >= unknown (signed, 32-bit)`, falls through to 0x127197 when `0x1 < unknown (signed, 32-bit)`.
  Byte load: target `(unknown effective address)`, base register `edx` (`mov` at 0x127198).
  Branch (`jl` at 0x1271a5): taken to 0x127198 when `unknown < unknown (signed, 32-bit)`, falls through to 0x1271a7 when `unknown >= unknown (signed, 32-bit)`.
  At the return (`ret` at 0x1271b2), eax is `unknown`.
  The instructions from 0x1271b3 to 0x12727b are not reached by any path from the entry, and their branches are not listed.

- `_ttyGetRxBuf`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12e4d8`.
  Role: unknown. Inferred analysis span: 0x12e4d8 to 0x12e528 (exclusive), 80 bytes.
  Carrier SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`, symbol index 1106, file offset 0x264f8.
  Read window: SHA256 `393d72e35ec01ff3c7ffef831dd193e6a1b00e2c63ee3cdee1c8abf020406d12`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12e4db).
  Branch (`jbe` at 0x12e4e1): taken to 0x12e4f8 when `[sp+4] <= 0x5 (unsigned, 32-bit)`, falls through to 0x12e4e3 when `[sp+4] > 0x5 (unsigned, 32-bit)`.
  The exit (`ret` at 0x12e4eb) reports eax = `0xffffffff`.
  The instructions from 0x12e4ec to 0x12e4f7 are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 3 4-byte load records with target `(unknown effective address)`, base register `eax`, including `cmp` at 0x12e502, `mov` at 0x12e509, `mov` at 0x12e51d.
  Branch (`je` at 0x12e507): taken to 0x12e518 when `unknown == 0x0`, falls through to 0x12e509 when `unknown != 0x0`.
  Recorded eax at the return is `unknown` (`ret` at 0x12e510).
  The instructions from 0x12e511 to 0x12e517 are not reached by any path from the entry, and their branches are not listed.
  `ret` at 0x12e524 has recorded eax = `unknown`.
  The instructions from 0x12e525 to 0x12e527 are not reached by any path from the entry, and their branches are not listed.

- `_routeProcessELSCCommand`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12c4e4`.
  Role: unknown. Inferred analysis span: 0x12c4e4 to 0x12c5d4 (exclusive), 240 bytes.
  Carrier SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`, symbol index 747, file offset 0x24504.
  Read window: SHA256 `17c06fbad3f812cd5987aca765bb8ea6396aa4208205590e09806623b4953464`.
  Four-byte load: target `sp +0xc` (`mov` at 0x12c4ea).
  Grouped accesses: 2 4-byte load records with target `abs 0x1f4b7c`, including `mov` at 0x12c4ed, `mov` at 0x12c500.
  Byte load: target `([0x1f4b7c] + [sp+12]) +0x4` (`mov` at 0x12c4f2).
  Branch (`je` at 0x12c4f8): taken to 0x12c569 when `(([([0x1f4b7c] + [sp+12])+4]:1 & 0xff) & ([([0x1f4b7c] + [sp+12])+4]:1 & 0xff)) == 0x0`, falls through to 0x12c4fa when `(([([0x1f4b7c] + [sp+12])+4]:1 & 0xff) & ([([0x1f4b7c] + [sp+12])+4]:1 & 0xff)) != 0x0`.
  Grouped accesses: 2 1-byte load records with target `([0x1f4b7c] + [sp+12]) +0x4`, including `mov` at 0x12c506, `mov` at 0x12c534.
  Branch (`jae` at 0x12c512): taken to 0x12c534 when `C == 0`, falls through to 0x12c514 when `not (C == 0)`.
  Four-byte load: target `sp +0x8` (`mov` at 0x12c514).
  Four-byte load: target `[sp+8] +0x20` (`mov` at 0x12c517).
  Call (`call` at 0x12c51c): `_elscSendPassword`, target 0x1240ec, stack arg0=`unknown`, stack arg1=`[[sp+8]+32]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12c524).
  Call (`call` at 0x12c529): `_elscSendCommand`, target 0x123f4c, stack arg0=`unknown`, stack arg1=`[sp+4]`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x12c531): target 0x12c560.
  The instructions from 0x12c533 to 0x12c533 are not reached by any path from the entry, and their branches are not listed.
  Branch (`jge` at 0x12c53a): taken to 0x12c560 when `ge flags condition (operands unknown)`, falls through to 0x12c53c when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, base register `esi`, including `mov` at 0x12c541, `mov` at 0x12c553.
  Four-byte load: target `(unknown effective address)`, base register `eax` (`mov` at 0x12c547).
  Call (`call` at 0x12c54b): `_sprintf`, target 0x176c40, stack arg0=`unknown`, stack arg1=`0x12c4da`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte store: target `(unknown effective address)`, value `0xb`, base register `eax` (`mov` at 0x12c559).
  Branch (`jbe` at 0x12c567): taken to 0x12c500 when `unknown <= 0x1 (unsigned, 32-bit)`, falls through to 0x12c569 when `unknown > 0x1 (unsigned, 32-bit)`.
  Recorded eax at the return is `unknown` (`ret` at 0x12c572).
  The instructions from 0x12c573 to 0x12c5d3 are not reached by any path from the entry, and their branches are not listed.
