# Selected MMSC symbol spans in IRIX 6.5.2, 6.5.5 and 6.5.6

Each entry concerns its exact named carrier and a.out symbol span. Addresses use the
image's linked coordinates. The next text symbol supplies an inferred exclusive end,
which can include unreachable padding or literals. Function extents, hardware roles,
runtime mapping and callee effects remain unknown.

`sp` denotes the stack pointer at the analyzed entry. Brackets denote loaded values.
Calls retain all six candidate stack slots under an assumed cdecl32 convention, with
unknown actual arity. A `ret(F)` expression alone does not identify its producing call
site. Groups count static access shapes and do not establish equal values from separate
loads or an unconditional sequence across branch arms. The word `unknown` represents a
tool question mark.

The [complete facts](disasm/interrupt-uart-tty-remaining-facts.txt) retain every decoded
instruction, including unreached instructions. Ends in the unreached-range sentences are
exclusive. Unsupported effects and undecoded tails remain explicit. Context views do not
establish predecessor flow or full function boundaries.

- `_sysIntEOI`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x108398`.
  Inferred analysis span: 0x108398 to 0x1083b8 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 65, file offset 0x3b8, window SHA256 `143b0110de2e489363d8eee173e0fcafde6a9341af729d0541c5dde8e8e1afa8`.
  Call (`call` at 0x10839f): _sysOutByte, target 0x1081d0, stack arg0=`0x20`, stack arg1=`0x20`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1083ae): _sysOutByte, target 0x1081d0, stack arg0=`0xa0`, stack arg1=`0x20`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  At the return (`ret` at 0x1083b6), eax is `ret(_sysOutByte)`.
  The instructions from 0x1083b7 to 0x1083b8 are not reached by any path from the entry, and their branches are not listed.

- `_sysIntEOI`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x108398`.
  Inferred analysis span: 0x108398 to 0x1083b8 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Symbol index 65, file offset 0x3b8, window SHA256 `143b0110de2e489363d8eee173e0fcafde6a9341af729d0541c5dde8e8e1afa8`.
  Call (`call` at 0x10839f): _sysOutByte, target 0x1081d0, stack arg0=`0x20`, stack arg1=`0x20`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1083ae): _sysOutByte, target 0x1081d0, stack arg0=`0xa0`, stack arg1=`0x20`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The exit (`ret` at 0x1083b6) reports eax = `ret(_sysOutByte)`.
  The instructions from 0x1083b7 to 0x1083b8 are not reached by any path from the entry, and their branches are not listed.

- `_sysIntEOI`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x108398`.
  Inferred analysis span: 0x108398 to 0x1083b8 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Symbol index 65, file offset 0x3b8, window SHA256 `143b0110de2e489363d8eee173e0fcafde6a9341af729d0541c5dde8e8e1afa8`.
  Call (`call` at 0x10839f): _sysOutByte, target 0x1081d0, stack arg0=`0x20`, stack arg1=`0x20`, stack arg2=`ebp`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1083ae): _sysOutByte, target 0x1081d0, stack arg0=`0xa0`, stack arg1=`0x20`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `ret` at 0x1083b6 has recorded eax = `ret(_sysOutByte)`.
  The instructions from 0x1083b7 to 0x1083b8 are not reached by any path from the entry, and their branches are not listed.

- `_ns16550DevInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x175394`.
  Inferred analysis span: 0x175394 to 0x1753f4 (exclusive), 96 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 197, file offset 0x6d3b4, window SHA256 `9ea63d40927089cee660385c0d25149dcac36ef05c464b5d9f710ff1607e73de`.
  Call (`call` at 0x17539a): _intLock, target 0x1713d0, stack arg0=`unknown`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 6 4-byte load records with target `sp +0x4`, including `mov` at 0x1753a4, `mov` at 0x1753ad, `mov` at 0x1753b7.
  Four-byte store: target `[sp+4] +0x0`, value `0x1d2e14` (`mov` at 0x1753a7).
  Four-byte store: target `[sp+4] +0x4`, value `0x175374` (`mov` at 0x1753b0).
  Four-byte store: target `[sp+4] +0x8`, value `0x175374` (`mov` at 0x1753ba).
  Two-byte store: target `[sp+4] +0x5c`, value `0x0` (`mov` at 0x1753c4).
  Four-byte store: target `[sp+4] +0x68`, value `0xf` (`mov` at 0x1753cd).
  Call (`call` at 0x1753d8): _ns16550InitChannel, target 0x1753f4, stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x1753e4): _intUnlock, target 0x1713e0, stack arg0=`[sp-8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax at the return is `ret(_intUnlock)` (`ret` at 0x1753ef).
  The instructions from 0x1753f0 to 0x1753f4 are not reached by any path from the entry, and their branches are not listed.

- `_ns16550PollInput`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1763c4`.
  Inferred analysis span: 0x1763c4 to 0x176424 (exclusive), 96 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 4695, file offset 0x6e3e4, window SHA256 `25cfd745b124a98e2ac73891d88478d9921ff462e2ad1d8730255ebc85eb9591`.
  Grouped accesses: 4 4-byte load records with target `sp +0x4`, including `mov` at 0x1763cb, `mov` at 0x1763ce, `mov` at 0x1763f4.
  Four-byte load: target `[sp+4] +0x48` (`mov` at 0x1763d1).
  Grouped accesses: 2 4-byte load records with target `[sp+4] +0x28`, including `mov` at 0x1763d5, `mov` at 0x1763fe.
  Call (`call` at 0x1763d8): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+72]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x1763e7): taken to 0x1763f4 when `(unknown & unknown) != 0x0`, falls through to 0x1763e9 when `(unknown & unknown) == 0x0`.
  Jump (`jmp` at 0x1763ee): target 0x176414.
  The instructions from 0x1763f0 to 0x1763f4 are not reached by any path from the entry, and their branches are not listed.
  Four-byte load: target `[sp+4] +0x30` (`mov` at 0x1763fa).
  Call (`call` at 0x176401): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `sp +0x8` (`mov` at 0x176408).
  Byte store: target `[sp+8] +0x0`, value `unknown` (`mov` at 0x17640b).
  Jump (`jmp` at 0x17640f): target 0x176414.
  The instructions from 0x176411 to 0x176414 are not reached by any path from the entry, and their branches are not listed.
  At the return (`ret` at 0x17641a), eax is `unknown`.
  The instructions from 0x17641b to 0x176424 are not reached by any path from the entry, and their branches are not listed.

- `_ns16550PollOutput`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x176354`.
  Inferred analysis span: 0x176354 to 0x1763c4 (exclusive), 112 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 4696, file offset 0x6e374, window SHA256 `5c10e8bfb063bb804246650f8b8e4000bcd717b6769b06a6d697116c531d8abe`.
  Four-byte load: target `sp +0x8` (`mov` at 0x17635c).
  Grouped accesses: 4 4-byte load records with target `sp +0x4`, including `mov` at 0x176362, `mov` at 0x176365, `mov` at 0x176394.
  Four-byte load: target `[sp+4] +0x48` (`mov` at 0x176368).
  Four-byte load: target `[sp+4] +0x28` (`mov` at 0x17636c).
  Call (`call` at 0x17636f): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+72]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`unknown`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`jne` at 0x17637e): taken to 0x176394 when `(unknown & unknown) != 0x0`, falls through to 0x176380 when `(unknown & unknown) == 0x0`.
  Jump (`jmp` at 0x176385): target 0x1763b4.
  The instructions from 0x176387 to 0x176394 are not reached by any path from the entry, and their branches are not listed.
  Four-byte load: target `[sp+4] +0x30` (`mov` at 0x17639f).
  Four-byte load: target `[sp+4] +0x2c` (`mov` at 0x1763a3).
  Call (`call` at 0x1763a6): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x1763ad): target 0x1763b4.
  The instructions from 0x1763af to 0x1763b4 are not reached by any path from the entry, and their branches are not listed.
  The exit (`ret` at 0x1763bc) reports eax = `unknown`.
  The instructions from 0x1763bd to 0x1763c4 are not reached by any path from the entry, and their branches are not listed.

- `_ns16550IntWr`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x175fe4`.
  Inferred analysis span: 0x175fe4 to 0x176064 (exclusive), 128 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 4702, file offset 0x6e004, window SHA256 `4c7e920c1288156618672eb54598017a5d35c0780378d55258deee04dbae013f`.
  Grouped accesses: 9 4-byte load records with target `sp +0x4`, including `mov` at 0x175feb, `mov` at 0x175ff2, `mov` at 0x176008.
  Four-byte load: target `[sp+4] +0xc` (`mov` at 0x175ff5).
  Four-byte load: target `[sp+4] +0x4` (`mov` at 0x175ff9).
  Call (`call` at 0x175ffc): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+12]`, stack arg1=`sp-5`, stack arg2=`ebx`, stack arg3=`unknown`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`je` at 0x176006): taken to 0x176024 when `ret(unknown) == 0xffffffff`, falls through to 0x176008 when `ret(unknown) != 0xffffffff`.
  Four-byte load: target `[sp+4] +0x30` (`mov` at 0x176013).
  Grouped accesses: 2 4-byte load records with target `[sp+4] +0x2c`, including `mov` at 0x176017, `mov` at 0x176049.
  Call (`call` at 0x17601a): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Jump (`jmp` at 0x17601f): target 0x176051.
  The instructions from 0x176021 to 0x176024 are not reached by any path from the entry, and their branches are not listed.
  Grouped accesses: 2 1-byte load records with target `[sp+4] +0x19`, including `mov` at 0x17602a, `mov` at 0x17603b.
  Byte store: target `[sp+4] +0x19`, value `unknown` (`mov` at 0x176032).
  Four-byte load: target `[sp+4] +0x34` (`mov` at 0x176045).
  Call (`call` at 0x17604c): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+52]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  `ret` at 0x176057 has recorded eax = `ret(unknown)`.
  The instructions from 0x176058 to 0x176064 are not reached by any path from the entry, and their branches are not listed.

- `_ns16550IntRd`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x176064`.
  Inferred analysis span: 0x176064 to 0x1760a4 (exclusive), 64 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 4703, file offset 0x6e084, window SHA256 `c58546a02bc8bb5ed2e14cfbd0e12eadbe043152315ad3bb10db5fb72fdfeb18`.
  Grouped accesses: 4 4-byte load records with target `sp +0x4`, including `mov` at 0x17606b, `mov` at 0x17606e, `mov` at 0x176080.
  Four-byte load: target `[sp+4] +0x30` (`mov` at 0x176071).
  Four-byte load: target `[sp+4] +0x28` (`mov` at 0x176075).
  Call (`call` at 0x176078): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Four-byte load: target `[sp+4] +0x10` (`mov` at 0x17608b).
  Four-byte load: target `[sp+4] +0x8` (`mov` at 0x17608f).
  Call (`call` at 0x176092): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+16]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax at the return is `ret(unknown)` (`ret` at 0x17609d).
  The instructions from 0x17609e to 0x1760a4 are not reached by any path from the entry, and their branches are not listed.

- `_sysSerialChanGet`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x10b7c8`.
  Inferred analysis span: 0x10b7c8 to 0x10b808 (exclusive), 64 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 201, file offset 0x37e8, window SHA256 `1ac4975f376f2c61828bfde586ea5ad98bf57d8034df23d3ed013dc4886b3b02`.
  Four-byte load: target `sp +0x4` (`mov` at 0x10b7cb).
  Branch (`jle` at 0x10b7d1): taken to 0x10b7e8 when `[sp+4] <= 0x5 (signed, 32-bit)`, falls through to 0x10b7d3 when `[sp+4] > 0x5 (signed, 32-bit)`.
  At the return (`ret` at 0x10b7db), eax is `0xffffffff`.
  The instructions from 0x10b7dc to 0x10b7e8 are not reached by any path from the entry, and their branches are not listed.
  At the return (`ret` at 0x10b800), eax is `unknown`.
  The instructions from 0x10b801 to 0x10b806 are not reached by any path from the entry, and their branches are not listed.
  Decoding stops at 0x10b806. The remaining bytes are not instructions in this analysis.

- `_ttySetSpeed`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12bc04`.
  Inferred analysis span: 0x12bc04 to 0x12bc28 (exclusive), 36 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 1068, file offset 0x23c24, window SHA256 `e0f16d522c78403bd568cb9d74737bbf383c81a9c7cede7e14a691c3f452b4df`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12bc07).
  Four-byte load: target `sp +0x8` (`mov` at 0x12bc0a).
  Call (`call` at 0x12bc11): _ioctl, target 0x189440, stack arg0=`[sp+4]`, stack arg1=`0x4`, stack arg2=`[sp+8]`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x12bc1a): taken to 0x12bc21 when `ge flags condition (operands unknown)`, falls through to 0x12bc1c when `not (ge flags condition (operands unknown))`.
  The exit (`ret` at 0x12bc26) reports eax = `unknown`.
  The instructions from 0x12bc27 to 0x12bc28 are not reached by any path from the entry, and their branches are not listed.

- `_ttyReInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x12bba4`.
  Inferred analysis span: 0x12bba4 to 0x12bc04 (exclusive), 96 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 742, file offset 0x23bc4, window SHA256 `381f1ec28b73a7289bfc8c8b867c2bb10422faa88492c4a5904634ec3102c4a4`.
  Call (`call` at 0x12bbaa): _sysSerialHwInit, target 0x10b618, stack arg0=`ebx`, stack arg1=`esi`, stack arg2=`edi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, including `mov` at 0x12bbc3, `mov` at 0x12bbd7.
  Call (`call` at 0x12bbcb): _ioctl, target 0x189440, stack arg0=`unknown`, stack arg1=`0x1007`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12bbd5): taken to 0x12bbec when `(ret(_ioctl) & ret(_ioctl)) == 0x0`, falls through to 0x12bbd7 when `(ret(_ioctl) & ret(_ioctl)) != 0x0`.
  Call (`call` at 0x12bbe4): _ffscMsg, target 0x11f0d0, stack arg0=`0x12bb82`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Effects of `inc` at 0x12bbef are unsupported and were invalidated.
  Branch (`jle` at 0x12bbf2): taken to 0x12bbbc when `le flags condition (operands unknown)`, falls through to 0x12bbf4 when `not (le flags condition (operands unknown))`.
  `ret` at 0x12bbff has recorded eax = `0x0`.
  The instructions from 0x12bc00 to 0x12bc04 are not reached by any path from the entry, and their branches are not listed.

- `_ttySetSpeed`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12e928`.
  Inferred analysis span: 0x12e928 to 0x12e958 (exclusive), 48 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Symbol index 1108, file offset 0x26948, window SHA256 `9a49bdfd12f48bb3f994245dac7b8d6a384c586e14c2546aa44375d73aab8d92`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12e92b).
  Four-byte load: target `sp +0x8` (`mov` at 0x12e92e).
  Call (`call` at 0x12e935): _ioctl, target 0x18d198, stack arg0=`[sp+4]`, stack arg1=`0x4`, stack arg2=`[sp+8]`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x12e93e): taken to 0x12e945 when `ge flags condition (operands unknown)`, falls through to 0x12e940 when `not (ge flags condition (operands unknown))`.
  Recorded eax at the return is `unknown` (`ret` at 0x12e94a).
  The instructions from 0x12e94b to 0x12e958 are not reached by any path from the entry, and their branches are not listed.

- `_ttyReInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x12e8c8`.
  Inferred analysis span: 0x12e8c8 to 0x12e928 (exclusive), 96 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Symbol index 777, file offset 0x268e8, window SHA256 `ffb01a87e93f349f912aa05d4c09aa8e598e6383340b7b5fffc508c3519327b3`.
  Call (`call` at 0x12e8ce): _sysSerialHwInit, target 0x10b618, stack arg0=`ebx`, stack arg1=`esi`, stack arg2=`edi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, including `mov` at 0x12e8e7, `mov` at 0x12e8fb.
  Call (`call` at 0x12e8ef): _ioctl, target 0x18d198, stack arg0=`unknown`, stack arg1=`0x1007`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12e8f9): taken to 0x12e910 when `(ret(_ioctl) & ret(_ioctl)) == 0x0`, falls through to 0x12e8fb when `(ret(_ioctl) & ret(_ioctl)) != 0x0`.
  Call (`call` at 0x12e908): _ffscMsg, target 0x12153c, stack arg0=`0x12e8a6`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Effects of `inc` at 0x12e913 are unsupported and were invalidated.
  Branch (`jle` at 0x12e916): taken to 0x12e8e0 when `le flags condition (operands unknown)`, falls through to 0x12e918 when `not (le flags condition (operands unknown))`.
  At the return (`ret` at 0x12e923), eax is `0x0`.
  The instructions from 0x12e924 to 0x12e928 are not reached by any path from the entry, and their branches are not listed.

- `_ttySetSpeed`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e938`.
  Inferred analysis span: 0x12e938 to 0x12e968 (exclusive), 48 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Symbol index 1108, file offset 0x26958, window SHA256 `9a49bdfd12f48bb3f994245dac7b8d6a384c586e14c2546aa44375d73aab8d92`.
  Four-byte load: target `sp +0x4` (`mov` at 0x12e93b).
  Four-byte load: target `sp +0x8` (`mov` at 0x12e93e).
  Call (`call` at 0x12e945): _ioctl, target 0x18d1a8, stack arg0=`[sp+4]`, stack arg1=`0x4`, stack arg2=`[sp+8]`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jge` at 0x12e94e): taken to 0x12e955 when `ge flags condition (operands unknown)`, falls through to 0x12e950 when `not (ge flags condition (operands unknown))`.
  The exit (`ret` at 0x12e95a) reports eax = `unknown`.
  The instructions from 0x12e95b to 0x12e968 are not reached by any path from the entry, and their branches are not listed.

- `_ttyReInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x12e8d8`.
  Inferred analysis span: 0x12e8d8 to 0x12e938 (exclusive), 96 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Symbol index 777, file offset 0x268f8, window SHA256 `b5391156901571637bd3b1ee860ade46cc8ff3437a2378aad3650cfe63e25995`.
  Call (`call` at 0x12e8de): _sysSerialHwInit, target 0x10b618, stack arg0=`ebx`, stack arg1=`esi`, stack arg2=`edi`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `(unknown effective address)`, including `mov` at 0x12e8f7, `mov` at 0x12e90b.
  Call (`call` at 0x12e8ff): _ioctl, target 0x18d1a8, stack arg0=`unknown`, stack arg1=`0x1007`, stack arg2=`0x2`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`je` at 0x12e909): taken to 0x12e920 when `(ret(_ioctl) & ret(_ioctl)) == 0x0`, falls through to 0x12e90b when `(ret(_ioctl) & ret(_ioctl)) != 0x0`.
  Call (`call` at 0x12e918): _ffscMsg, target 0x12154c, stack arg0=`0x12e8b6`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Effects of `inc` at 0x12e923 are unsupported and were invalidated.
  Branch (`jle` at 0x12e926): taken to 0x12e8f0 when `le flags condition (operands unknown)`, falls through to 0x12e928 when `not (le flags condition (operands unknown))`.
  `ret` at 0x12e933 has recorded eax = `0x0`.
  The instructions from 0x12e934 to 0x12e938 are not reached by any path from the entry, and their branches are not listed.
