# MMSC BSP serial and NVRAM routines

Read from IRIX 6.5.30 MMSC 2.3 at 6.5.30/usr/cpu/firmware/mmscfw.bin. evidence/controllers.md identifies the image as x86 486-class; its text load base is 0x108000. The symbol names below come from the image’s nlist table. The listings use Capstone x86 32-bit mode.

- _sysNvRamGet — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x1087c8
  This symbol’s body returns -1: it loads 0xffffffff into eax and returns (mov at 0x1087cb; ret at 0x1087d3). The body does not read NVRAM.

- _sysNvRamSet — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x1087d8
  This symbol’s body also returns -1 without an NVRAM access (mov at 0x1087db; ret at 0x1087e3).

- _sysSerialHwInit — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x10b618
  It uses table 0x1b0454 and record base 0x1d8a10 (mov at 0x10b628; mov at 0x10b62d). Each pass clears the word at record +0x5c, stores 0x1c2000 at +0x64, and writes 0x1081d0 and 0x1081b0 at +0x2c and +0x28 (mov at 0x10b640; mov at 0x10b647; mov at 0x10b709; mov at 0x10b711). It calls 0x17910c with the current record (call at 0x10b720). Starting from zero, the loop increments its index and takes jle at 0x10b73c while the index is at most 5, so it processes six records (mov at 0x10b621; inc at 0x10b732; cmp at 0x10b738; jle at 0x10b73c).

- _sysSerialHwInit2 — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x10b758
  It scans six records using the same 12-byte table stride and 0x6c-byte record stride (add at 0x10b79f; add at 0x10b7a2; cmp at 0x10b7ab; jle at 0x10b7af). For a zero word at the current table entry, je at 0x10b778 takes the skip path. Otherwise it calls 0x17a210 with the scaled word, 0x179e2c, and the current record pointer (push at 0x10b77a; push at 0x10b77b; shl at 0x10b783; push at 0x10b786; call at 0x10b787). It then passes the word at table +0x0a to 0x108428 (movzx at 0x10b78f; call at 0x10b797).

- _sysSerialChanGet — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x10b7c8
  A signed argument above 5 takes jle’s opposite path and returns -1 (cmp at 0x10b7ce; jle at 0x10b7d1; mov at 0x10b7d3; ret at 0x10b7db). The jle path, including negative values, computes 0x1d8a10 + 0x6c*argument and returns that address (lea at 0x10b7e8; sub at 0x10b7ef; shl at 0x10b7f1; sub at 0x10b7f4; lea at 0x10b7f6; ret at 0x10b800).

- _ns16550DevInit — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x17910c
  It calls 0x175148 and saves the result (call at 0x179112; mov at 0x179119). For the supplied record it stores 0x1d6c44 at +0, 0x1790ec at +4 and +8, clears the word at +0x5c, and stores 0xf at +0x68 (mov at 0x17911f; mov at 0x179128; mov at 0x179132; mov at 0x17913c; mov at 0x179145). It calls 0x17916c with that record and then calls 0x175158 with the saved result (call at 0x179150; call at 0x17915c).

- _ns16550IntWr — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x179d5c
  It calls a function pointer at record +4 with record +0x0c and a local byte pointer (push at 0x179d66; push at 0x179d6d; call ebx at 0x179d74). When eax is -1, je at 0x179d7e takes the branch that clears bit 1 of the byte at record +0x19 and calls the pointer at +0x2c with record +0x34 and that byte (je at 0x179d7e; and at 0x179da7; mov at 0x179daa; call ebx at 0x179dc4). Otherwise it calls the pointer at +0x2c with record +0x30 and the returned byte (movsx at 0x179d83; mov at 0x179d8b; call ebx at 0x179d92).

- _ns16550IntRd — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x179ddc
  It calls the pointer at record +0x28 with record +0x30 and saves the returned byte (push at 0x179de9; call ebx at 0x179df0; mov at 0x179df5). It then calls the pointer at +8 with record +0x10 and that byte sign-extended (movsx at 0x179dfb; push at 0x179dff; mov at 0x179e03; call ebx at 0x179e0a).

- _ns16550PollInput — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x17a13c
  It reads status through the function pointer at record +0x28, passing the word at +0x48 (push at 0x17a149; call ebx at 0x17a150). It tests status bit 0; jne at 0x17a15f takes the bit-0-set path, while the bit-0-clear path returns 0x0b (and at 0x17a15b; test at 0x17a15d; jne at 0x17a15f; mov at 0x17a161). On the ready path, it reads through the same function pointer using record +0x30, stores the returned byte through argument 1, and returns zero (push at 0x17a172; call ebx at 0x17a179; mov at 0x17a183; xor at 0x17a185).

- _ns16550PollOutput — 6.5.30/usr/cpu/firmware/mmscfw.bin, 0x17a0cc
  It reads status through the pointer at record +0x28 using the word at +0x48, then tests bit 5 (push at 0x17a0e0; call esi at 0x17a0e7; and at 0x17a0f2; test at 0x17a0f4). jne at 0x17a0f6 takes the bit-5-set path; the bit-5-clear path returns 0x0b (jne at 0x17a0f6; mov at 0x17a0f8). The ready path calls the pointer at record +0x2c with the word at +0x30 and the byte from argument 1, then returns zero (push at 0x17a113; push at 0x17a117; call esi at 0x17a11e; xor at 0x17a123).
