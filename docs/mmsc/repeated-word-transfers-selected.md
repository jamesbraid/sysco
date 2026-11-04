# MMSC repeated word transfers in selected IRIX 6.4 through 6.5.6 images

Each entry describes its exact carrier and a.out text-symbol span. Instruction effects
within these spans are established from the named image. The next symbol supplies an
inferred exclusive end, which includes unreached padding. Complete function extents,
hardware roles, segment bases and runtime mapping remain unknown.

`sp` denotes the entry stack pointer. Brackets denote loaded values. The assumed
cdecl32 convention does not establish actual arity. A per-transfer aggregate record
describes repeated effects, with its iteration expression and nonzero condition. It
does not describe one unconditional transfer or equal values across iterations.

String-memory linear effective addresses and transferred values remain unknown.
Segment-offset metadata is not a resolved linear address. The repeated-transfer and
return statements describe normal completion only. Faults, interruptions, permissions
and runtime path feasibility remain unknown. Ends in the unreached-range statements
are exclusive.

The [complete facts](disasm/repeated-word-transfers-selected-facts.txt) retain every
decoded instruction and all analysis sections.

- `_sysInWordString`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1081f0`.
  Inferred analysis span: 0x1081f0 to 0x108210 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 10, file offset 0x210, window SHA256 `6c0deb8688953547f0d18434ba55858da68b6ada10bf16afd73a3331c87d3bc4`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov` at 0x1081f5), `sp +0xc` (`mov` at 0x1081f9).
  Aggregate per-transfer I/O load: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Aggregate per-transfer memory store: width 2 bytes, linear effective address unknown, stored value unknown, segment `es` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Under normal completion, eax at the return is `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108204).
  The instructions from 0x108205 to 0x108210 are not reached by any path from the entry, and their branches are not listed.

- `_sysOutWordString`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x108210`.
  Inferred analysis span: 0x108210 to 0x108230 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Symbol index 13, file offset 0x230, window SHA256 `a72a5460e310dc7c56b8519697d012dda9d7824874be8a9eb0492f2dd18c5f82`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x108211), `sp +0x8` (`mov` at 0x108215), `sp +0xc` (`mov` at 0x108219).
  Aggregate per-transfer memory load: width 2 bytes, linear effective address unknown, segment `ds` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  Aggregate per-transfer I/O store: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, stored value unknown, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  The normal-completion return reports eax = `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108224).
  The instructions from 0x108225 to 0x108230 are not reached by any path from the entry, and their branches are not listed.

- `_sysInWordString`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1081f0`.
  Inferred analysis span: 0x1081f0 to 0x108210 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Symbol index 10, file offset 0x210, window SHA256 `6c0deb8688953547f0d18434ba55858da68b6ada10bf16afd73a3331c87d3bc4`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov` at 0x1081f5), `sp +0xc` (`mov` at 0x1081f9).
  Aggregate per-transfer I/O load: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Aggregate per-transfer memory store: width 2 bytes, linear effective address unknown, stored value unknown, segment `es` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Recorded eax on normal completion is `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108204).
  The instructions from 0x108205 to 0x108210 are not reached by any path from the entry, and their branches are not listed.

- `_sysOutWordString`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x108210`.
  Inferred analysis span: 0x108210 to 0x108230 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Symbol index 13, file offset 0x230, window SHA256 `a72a5460e310dc7c56b8519697d012dda9d7824874be8a9eb0492f2dd18c5f82`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x108211), `sp +0x8` (`mov` at 0x108215), `sp +0xc` (`mov` at 0x108219).
  Aggregate per-transfer memory load: width 2 bytes, linear effective address unknown, segment `ds` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  Aggregate per-transfer I/O store: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, stored value unknown, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  At the return (`ret` at 0x108224), normal completion gives eax = `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)`.
  The instructions from 0x108225 to 0x108230 are not reached by any path from the entry, and their branches are not listed.

- `_sysInWordString`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x1081f0`.
  Inferred analysis span: 0x1081f0 to 0x108210 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Symbol index 10, file offset 0x210, window SHA256 `6c0deb8688953547f0d18434ba55858da68b6ada10bf16afd73a3331c87d3bc4`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov` at 0x1081f5), `sp +0xc` (`mov` at 0x1081f9).
  Aggregate per-transfer I/O load: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Aggregate per-transfer memory store: width 2 bytes, linear effective address unknown, stored value unknown, segment `es` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Under normal completion, eax at the return is `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108204).
  The instructions from 0x108205 to 0x108210 are not reached by any path from the entry, and their branches are not listed.

- `_sysOutWordString`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x108210`.
  Inferred analysis span: 0x108210 to 0x108230 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Symbol index 13, file offset 0x230, window SHA256 `a72a5460e310dc7c56b8519697d012dda9d7824874be8a9eb0492f2dd18c5f82`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x108211), `sp +0x8` (`mov` at 0x108215), `sp +0xc` (`mov` at 0x108219).
  Aggregate per-transfer memory load: width 2 bytes, linear effective address unknown, segment `ds` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  Aggregate per-transfer I/O store: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, stored value unknown, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  The normal-completion return reports eax = `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108224).
  The instructions from 0x108225 to 0x108230 are not reached by any path from the entry, and their branches are not listed.

- `_sysInWordString`, `6.4/usr/cpu/firmware/mmscfw.bin`, `0x1081f0`.
  Inferred analysis span: 0x1081f0 to 0x108210 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 998378 bytes, SHA256 `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
  Symbol index 10, file offset 0x210, window SHA256 `6c0deb8688953547f0d18434ba55858da68b6ada10bf16afd73a3331c87d3bc4`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov` at 0x1081f5), `sp +0xc` (`mov` at 0x1081f9).
  Aggregate per-transfer I/O load: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Aggregate per-transfer memory store: width 2 bytes, linear effective address unknown, stored value unknown, segment `es` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Recorded eax on normal completion is `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108204).
  The instructions from 0x108205 to 0x108210 are not reached by any path from the entry, and their branches are not listed.

- `_sysOutWordString`, `6.4/usr/cpu/firmware/mmscfw.bin`, `0x108210`.
  Inferred analysis span: 0x108210 to 0x108230 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 998378 bytes, SHA256 `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
  Symbol index 13, file offset 0x230, window SHA256 `a72a5460e310dc7c56b8519697d012dda9d7824874be8a9eb0492f2dd18c5f82`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x108211), `sp +0x8` (`mov` at 0x108215), `sp +0xc` (`mov` at 0x108219).
  Aggregate per-transfer memory load: width 2 bytes, linear effective address unknown, segment `ds` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  Aggregate per-transfer I/O store: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, stored value unknown, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  At the return (`ret` at 0x108224), normal completion gives eax = `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)`.
  The instructions from 0x108225 to 0x108230 are not reached by any path from the entry, and their branches are not listed.

- `_sysInWordString`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1081f0`.
  Inferred analysis span: 0x1081f0 to 0x108210 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Symbol index 10, file offset 0x210, window SHA256 `6c0deb8688953547f0d18434ba55858da68b6ada10bf16afd73a3331c87d3bc4`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x1081f1), `sp +0x8` (`mov` at 0x1081f5), `sp +0xc` (`mov` at 0x1081f9).
  Aggregate per-transfer I/O load: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Aggregate per-transfer memory store: width 2 bytes, linear effective address unknown, stored value unknown, segment `es` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep insw` at 0x1081fe).
  Under normal completion, eax at the return is `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108204).
  The instructions from 0x108205 to 0x108210 are not reached by any path from the entry, and their branches are not listed.

- `_sysOutWordString`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x108210`.
  Inferred analysis span: 0x108210 to 0x108230 (exclusive), 32 bytes. Role: unknown.
  x86-32 a.out load `0x108000`, carrier 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Symbol index 13, file offset 0x230, window SHA256 `a72a5460e310dc7c56b8519697d012dda9d7824874be8a9eb0492f2dd18c5f82`.
  Three 4-byte loads have targets `sp +0x4` (`mov` at 0x108211), `sp +0x8` (`mov` at 0x108215), `sp +0xc` (`mov` at 0x108219).
  Aggregate per-transfer memory load: width 2 bytes, linear effective address unknown, segment `ds` base unknown, iterations `[sp+12]` from `ecx`, stride `2`, address width 32 bits, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  Aggregate per-transfer I/O store: width 2 bytes, target `([sp+4] & 0xffff) +0x0`, iterations `[sp+12]` from `ecx`, stride `0`, port address width 16 bits, stored value unknown, conditioned on `[sp+12] != 0x0`, under normal completion (`rep outsw` at 0x10821e).
  The normal-completion return reports eax = `(([sp+8] + ([sp+12] * 0x2)) & 0xffffffff)` (`ret` at 0x108224).
  The instructions from 0x108225 to 0x108230 are not reached by any path from the entry, and their branches are not listed.
