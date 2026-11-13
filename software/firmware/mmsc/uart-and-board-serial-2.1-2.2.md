# MMSC UART and board serial routines

The observations apply to the two exact x86-32 a.out images below. The load address is 0x108000. Cdecl32 is assumed. Every printed stack slot is a candidate argument location, with custom arity unknown. Value roles, pointee meanings, C types and runtime path feasibility remain unknown. Symbol names do not establish a hardware role.

All selected extents are inferred gaps to the next text symbol, rather than recorded function sizes. They may include padding or data. Range ends are exclusive. In unreached-range statements, "any path from the entry" means a path in the recovered known-edge control-flow graph. Computed targets that remain unknown have unresolved effects. Repeated accesses count static records and need not occur along one path. `ret(F)` preserves an analysis expression and does not identify a producing call instruction. The [complete facts](disasm/uart-and-board-serial-2.1-2.2-facts.txt) retain the full analysis.

Image `6.5.6/usr/cpu/firmware/mmscfw.bin` is 1023060 bytes, SHA-256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
The text segment begins at file offset 0x20, loads at 0x108000 and is 687048 bytes.
The data segment begins at file offset 0xa7be8, loads at 0x1afbc8 and is 166912 bytes.

Image `6.5.5/usr/cpu/firmware/mmscfw.bin` is 1023103 bytes, SHA-256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The text segment begins at file offset 0x20, loads at 0x108000 and is 687032 bytes.
The data segment begins at file offset 0xa7bd8, loads at 0x1afbb8 and is 166912 bytes.

- `_ns16550DevInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x1790fc.
  Inferred span 0x1790fc to 0x17915c (exclusive), 96 bytes.
  Call (`call` at 0x179102): _intLock, target 0x175138, stack arg0=`unknown`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x0`, with value `0x1d6c34` (`mov` at 0x17910f).
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x4`, with value `0x1790dc` (`mov` at 0x179118).
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x8`, with value `0x1790dc` (`mov` at 0x179122).
  Stores 2 bytes, base `eax`, resolved as `[sp+4] +0x5c`, with value `0x0` (`mov` at 0x17912c).
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x68`, with value `0xf` (`mov` at 0x179135).
  Call (`call` at 0x179140): _ns16550InitChannel, target 0x17915c, stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x17914c): _intUnlock, target 0x175148, stack arg0=`[sp-8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax = `ret(_intUnlock)` (`ret` at 0x179157).
  The instructions from 0x179158 to 0x17915c are not reached by any path from the entry, and their branches are not listed.

- `_ns16550IntWr`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x179d4c.
  Inferred span 0x179d4c to 0x179dcc (exclusive), 128 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0xc` (`mov` at 0x179d5d).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x4` (`mov` at 0x179d61).
  Call (`call` at 0x179d64): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+12]`, stack arg1=`sp-5`, stack arg2=`ebx`, stack arg3=`unknown`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`je` at 0x179d6e): taken to 0x179d8c when `ret(unknown) == 0xffffffff`, falls through to 0x179d70 when `ret(unknown) != 0xffffffff`.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x179d7b).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x2c`. 2 such accesses, including `mov` at 0x179d7f, `mov` at 0x179db1.
  Call (`call` at 0x179d82): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The instructions from 0x179d89 to 0x179d8c are not reached by any path from the entry, and their branches are not listed.
  Loads 1 byte, base `edx`, resolved as `[sp+4] +0x19`. 2 such accesses, including `mov` at 0x179d92, `mov` at 0x179da3.
  Stores 1 byte, base `eax`, resolved as `[sp+4] +0x19`, with value `unknown` (`mov` at 0x179d9a).
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x34` (`mov` at 0x179dad).
  Call (`call` at 0x179db4): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+52]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax = `ret(unknown)` (`ret` at 0x179dbf).
  The instructions from 0x179dc0 to 0x179dcc are not reached by any path from the entry, and their branches are not listed.

- `_ns16550IntRd`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x179dcc.
  Inferred span 0x179dcc to 0x179e0c (exclusive), 64 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x179dd9).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x28` (`mov` at 0x179ddd).
  Call (`call` at 0x179de0): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x10` (`mov` at 0x179df3).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x8` (`mov` at 0x179df7).
  Call (`call` at 0x179dfa): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+16]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax = `ret(unknown)` (`ret` at 0x179e05).
  The instructions from 0x179e06 to 0x179e0c are not reached by any path from the entry, and their branches are not listed.

- `_ns16550PollInput`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x17a12c.
  Inferred span 0x17a12c to 0x17a18c (exclusive), 96 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x48` (`mov` at 0x17a139).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x28`. 2 such accesses, including `mov` at 0x17a13d, `mov` at 0x17a166.
  Call (`call` at 0x17a140): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+72]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x17a14f): taken to 0x17a15c when `(unknown & unknown) != 0x0`, falls through to 0x17a151 when `(unknown & unknown) == 0x0`.
  Sets `eax` to `0xb` (`mov` at 0x17a151).
  The instructions from 0x17a158 to 0x17a15c are not reached by any path from the entry, and their branches are not listed.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x17a162).
  Call (`call` at 0x17a169): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Stores 1 byte, base `edx`, resolved as `[sp+8] +0x0`, with value `unknown` (`mov` at 0x17a173).
  Sets `eax` to `0x0` (`xor` at 0x17a175).
  The instructions from 0x17a179 to 0x17a17c are not reached by any path from the entry, and their branches are not listed.
  Recorded eax = `unknown` (`ret` at 0x17a182).
  The instructions from 0x17a183 to 0x17a18c are not reached by any path from the entry, and their branches are not listed.

- `_ns16550PollOutput`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x17a0bc.
  Inferred span 0x17a0bc to 0x17a12c (exclusive), 112 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x48` (`mov` at 0x17a0d0).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x28` (`mov` at 0x17a0d4).
  Call (`call` at 0x17a0d7): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+72]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`unknown`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`jne` at 0x17a0e6): taken to 0x17a0fc when `(unknown & unknown) != 0x0`, falls through to 0x17a0e8 when `(unknown & unknown) == 0x0`.
  Sets `eax` to `0xb` (`mov` at 0x17a0e8).
  The instructions from 0x17a0ef to 0x17a0fc are not reached by any path from the entry, and their branches are not listed.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x17a107).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x2c` (`mov` at 0x17a10b).
  Call (`call` at 0x17a10e): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Sets `eax` to `0x0` (`xor` at 0x17a113).
  The instructions from 0x17a117 to 0x17a11c are not reached by any path from the entry, and their branches are not listed.
  Recorded eax = `unknown` (`ret` at 0x17a124).
  The instructions from 0x17a125 to 0x17a12c are not reached by any path from the entry, and their branches are not listed.

- `_sysSerialHwInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x10b618.
  Inferred span 0x10b618 to 0x10b758 (exclusive), 320 bytes.
  Analysis warning: 0x10b732 inc: unsupported effects invalidated.
  Sets `ecx` to `0x1d8a00` (`mov` at 0x10b628).
  Sets `edi` to `0x1b0444` (`mov` at 0x10b62d).
  Sets `ebx` to `0x0` (`xor` at 0x10b639).
  Sets `esi` to `0x0` (`xor` at 0x10b63b).
  Stores 2 bytes, base `ecx`, with unknown effective address, with value `0x0` (`mov` at 0x10b640).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `0x1c2000` (`mov` at 0x10b647).
  This group includes 12 static loads of 4 bytes with base `edi` and unknown effective addresses (`mov` at 0x10b64f, `mov` at 0x10b669, `add` at 0x10b676).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b653).
  Loads 1 byte, base `edi`, with unknown effective address (`mov` at 0x10b657).
  Stores 1 byte, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b65b).
  This group includes 10 static loads of 2 bytes with base `edi` and unknown effective addresses (`mov` at 0x10b65f, `movzx` at 0x10b671, `movzx` at 0x10b67e).
  Stores 2 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b664).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b66d).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b67a).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b689).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b698).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b6a8).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b6b8).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b6c8).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b6da).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b6f0).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b6f8).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `unknown` (`mov` at 0x10b705).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `0x1081d0` (`mov` at 0x10b709).
  Stores 4 bytes, base `ecx`, with unknown effective address, with value `0x1081b0` (`mov` at 0x10b711).
  Call (`call` at 0x10b720): _ns16550DevInit, target 0x1790fc, stack arg0=`[sp-12]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x10b73c): taken to 0x10b640 when `[sp-8] <= 0x5 (signed, 32-bit)`, falls through to 0x10b742 when `[sp-8] > 0x5 (signed, 32-bit)`.
  Recorded eax = `ret(_ns16550DevInit)` (`ret` at 0x10b74b).
  The instructions from 0x10b74c to 0x10b758 are not reached by any path from the entry, and their branches are not listed.

- `_sysSerialHwInit2`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x10b758.
  Inferred span 0x10b758 to 0x10b7c8 (exclusive), 112 bytes.
  Analysis warning: 0x10b7a8 inc: unsupported effects invalidated.
  Sets `ebx` to `0x1b0444` (`mov` at 0x10b768).
  Sets `edi` to `0x1d8a00` (`mov` at 0x10b76d).
  Sets `esi` to `0x0` (`xor` at 0x10b772).
  This group includes 2 static loads of 2 bytes with base `ebx` and unknown effective addresses (`cmp` at 0x10b774, `movzx` at 0x10b780).
  Branch (`je` at 0x10b778): taken to 0x10b79f when `Z == 1`, falls through to 0x10b77a when `not (Z == 1)`.
  Call (`call` at 0x10b787): _intConnect, target 0x17a200, stack arg0=`unknown`, stack arg1=`0x179e1c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Loads 2 bytes, base `esi`, with unknown effective address (`movzx` at 0x10b78f).
  Call (`call` at 0x10b797): _sysIntEnablePIC, target 0x108428, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x10b7af): taken to 0x10b774 when `[sp-8] <= 0x5 (signed, 32-bit)`, falls through to 0x10b7b1 when `[sp-8] > 0x5 (signed, 32-bit)`.
  Recorded eax = `unknown` (`ret` at 0x10b7ba).
  The instructions from 0x10b7bb to 0x10b7c8 are not reached by any path from the entry, and their branches are not listed.

- `_sysSerialChanGet`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, 0x10b7c8.
  Inferred span 0x10b7c8 to 0x10b808 (exclusive), 64 bytes.
  Analysis warning: decoder stopped at 0x10b806; remaining bytes are not instructions.
  Loads 4 bytes, base `ebp`, resolved as `sp +0x4` (`mov` at 0x10b7cb).
  Branch (`jle` at 0x10b7d1): taken to 0x10b7e8 when `[sp+4] <= 0x5 (signed, 32-bit)`, falls through to 0x10b7d3 when `[sp+4] > 0x5 (signed, 32-bit)`.
  Sets `eax` to `0xffffffff` (`mov` at 0x10b7d3).
  Recorded eax = `0xffffffff` (`ret` at 0x10b7db).
  The instructions from 0x10b7dc to 0x10b7e8 are not reached by any path from the entry, and their branches are not listed.
  Recorded eax = `unknown` (`ret` at 0x10b800).
  The instructions from 0x10b801 to 0x10b806 are not reached by any path from the entry, and their branches are not listed.

- `_ns16550DevInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x1790ec.
  Inferred span 0x1790ec to 0x17914c (exclusive), 96 bytes.
  Call (`call` at 0x1790f2): _intLock, target 0x175128, stack arg0=`unknown`, stack arg1=`ebp`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x0`, with value `0x1d6c24` (`mov` at 0x1790ff).
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x4`, with value `0x1790cc` (`mov` at 0x179108).
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x8`, with value `0x1790cc` (`mov` at 0x179112).
  Stores 2 bytes, base `eax`, resolved as `[sp+4] +0x5c`, with value `0x0` (`mov` at 0x17911c).
  Stores 4 bytes, base `eax`, resolved as `[sp+4] +0x68`, with value `0xf` (`mov` at 0x179125).
  Call (`call` at 0x179130): _ns16550InitChannel, target 0x17914c, stack arg0=`[sp+4]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Call (`call` at 0x17913c): _intUnlock, target 0x175138, stack arg0=`[sp-8]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax = `ret(_intUnlock)` (`ret` at 0x179147).
  The instructions from 0x179148 to 0x17914c are not reached by any path from the entry, and their branches are not listed.

- `_ns16550IntWr`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x179d3c.
  Inferred span 0x179d3c to 0x179dbc (exclusive), 128 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0xc` (`mov` at 0x179d4d).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x4` (`mov` at 0x179d51).
  Call (`call` at 0x179d54): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+12]`, stack arg1=`sp-5`, stack arg2=`ebx`, stack arg3=`unknown`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`je` at 0x179d5e): taken to 0x179d7c when `ret(unknown) == 0xffffffff`, falls through to 0x179d60 when `ret(unknown) != 0xffffffff`.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x179d6b).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x2c`. 2 such accesses, including `mov` at 0x179d6f, `mov` at 0x179da1.
  Call (`call` at 0x179d72): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  The instructions from 0x179d79 to 0x179d7c are not reached by any path from the entry, and their branches are not listed.
  Loads 1 byte, base `edx`, resolved as `[sp+4] +0x19`. 2 such accesses, including `mov` at 0x179d82, `mov` at 0x179d93.
  Stores 1 byte, base `eax`, resolved as `[sp+4] +0x19`, with value `unknown` (`mov` at 0x179d8a).
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x34` (`mov` at 0x179d9d).
  Call (`call` at 0x179da4): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+52]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax = `ret(unknown)` (`ret` at 0x179daf).
  The instructions from 0x179db0 to 0x179dbc are not reached by any path from the entry, and their branches are not listed.

- `_ns16550IntRd`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x179dbc.
  Inferred span 0x179dbc to 0x179dfc (exclusive), 64 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x179dc9).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x28` (`mov` at 0x179dcd).
  Call (`call` at 0x179dd0): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x10` (`mov` at 0x179de3).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x8` (`mov` at 0x179de7).
  Call (`call` at 0x179dea): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+16]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Recorded eax = `ret(unknown)` (`ret` at 0x179df5).
  The instructions from 0x179df6 to 0x179dfc are not reached by any path from the entry, and their branches are not listed.

- `_ns16550PollInput`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x17a11c.
  Inferred span 0x17a11c to 0x17a17c (exclusive), 96 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x48` (`mov` at 0x17a129).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x28`. 2 such accesses, including `mov` at 0x17a12d, `mov` at 0x17a156.
  Call (`call` at 0x17a130): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+72]`, stack arg1=`ebx`, stack arg2=`unknown`, stack arg3=`ebp`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jne` at 0x17a13f): taken to 0x17a14c when `(unknown & unknown) != 0x0`, falls through to 0x17a141 when `(unknown & unknown) == 0x0`.
  Sets `eax` to `0xb` (`mov` at 0x17a141).
  The instructions from 0x17a148 to 0x17a14c are not reached by any path from the entry, and their branches are not listed.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x17a152).
  Call (`call` at 0x17a159): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Stores 1 byte, base `edx`, resolved as `[sp+8] +0x0`, with value `unknown` (`mov` at 0x17a163).
  Sets `eax` to `0x0` (`xor` at 0x17a165).
  The instructions from 0x17a169 to 0x17a16c are not reached by any path from the entry, and their branches are not listed.
  Recorded eax = `unknown` (`ret` at 0x17a172).
  The instructions from 0x17a173 to 0x17a17c are not reached by any path from the entry, and their branches are not listed.

- `_ns16550PollOutput`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x17a0ac.
  Inferred span 0x17a0ac to 0x17a11c (exclusive), 112 bytes.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x48` (`mov` at 0x17a0c0).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x28` (`mov` at 0x17a0c4).
  Call (`call` at 0x17a0c7): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+72]`, stack arg1=`ebx`, stack arg2=`esi`, stack arg3=`unknown`, stack arg4=`ebp`, stack arg5=`unknown`.
  Branch (`jne` at 0x17a0d6): taken to 0x17a0ec when `(unknown & unknown) != 0x0`, falls through to 0x17a0d8 when `(unknown & unknown) == 0x0`.
  Sets `eax` to `0xb` (`mov` at 0x17a0d8).
  The instructions from 0x17a0df to 0x17a0ec are not reached by any path from the entry, and their branches are not listed.
  Loads 4 bytes, base `edx`, resolved as `[sp+4] +0x30` (`mov` at 0x17a0f7).
  Loads 4 bytes, base `eax`, resolved as `[sp+4] +0x2c` (`mov` at 0x17a0fb).
  Call (`call` at 0x17a0fe): (unknown callee), target (unknown target), stack arg0=`[[sp+4]+48]`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Sets `eax` to `0x0` (`xor` at 0x17a103).
  The instructions from 0x17a107 to 0x17a10c are not reached by any path from the entry, and their branches are not listed.
  Recorded eax = `unknown` (`ret` at 0x17a114).
  The instructions from 0x17a115 to 0x17a11c are not reached by any path from the entry, and their branches are not listed.

- `_sysSerialHwInit2`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x10b758.
  Inferred span 0x10b758 to 0x10b7c8 (exclusive), 112 bytes.
  Analysis warning: 0x10b7a8 inc: unsupported effects invalidated.
  Sets `ebx` to `0x1b0434` (`mov` at 0x10b768).
  Sets `edi` to `0x1d89f0` (`mov` at 0x10b76d).
  Sets `esi` to `0x0` (`xor` at 0x10b772).
  This group includes 2 static loads of 2 bytes with base `ebx` and unknown effective addresses (`cmp` at 0x10b774, `movzx` at 0x10b780).
  Branch (`je` at 0x10b778): taken to 0x10b79f when `Z == 1`, falls through to 0x10b77a when `not (Z == 1)`.
  Call (`call` at 0x10b787): _intConnect, target 0x17a1f0, stack arg0=`unknown`, stack arg1=`0x179e0c`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Loads 2 bytes, base `esi`, with unknown effective address (`movzx` at 0x10b78f).
  Call (`call` at 0x10b797): _sysIntEnablePIC, target 0x108428, stack arg0=`unknown`, stack arg1=`unknown`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`jle` at 0x10b7af): taken to 0x10b774 when `[sp-8] <= 0x5 (signed, 32-bit)`, falls through to 0x10b7b1 when `[sp-8] > 0x5 (signed, 32-bit)`.
  Recorded eax = `unknown` (`ret` at 0x10b7ba).
  The instructions from 0x10b7bb to 0x10b7c8 are not reached by any path from the entry, and their branches are not listed.

- `_sysSerialChanGet`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, 0x10b7c8.
  Inferred span 0x10b7c8 to 0x10b808 (exclusive), 64 bytes.
  Analysis warning: decoder stopped at 0x10b806; remaining bytes are not instructions.
  Loads 4 bytes, base `ebp`, resolved as `sp +0x4` (`mov` at 0x10b7cb).
  Branch (`jle` at 0x10b7d1): taken to 0x10b7e8 when `[sp+4] <= 0x5 (signed, 32-bit)`, falls through to 0x10b7d3 when `[sp+4] > 0x5 (signed, 32-bit)`.
  Sets `eax` to `0xffffffff` (`mov` at 0x10b7d3).
  Recorded eax = `0xffffffff` (`ret` at 0x10b7db).
  The instructions from 0x10b7dc to 0x10b7e8 are not reached by any path from the entry, and their branches are not listed.
  Recorded eax = `unknown` (`ret` at 0x10b800).
  The instructions from 0x10b801 to 0x10b806 are not reached by any path from the entry, and their branches are not listed.
