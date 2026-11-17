# Readings: `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)`

One entry per routine, filed here from the reading batches by binary.

- `sub_fe004efc` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A near-sibling of sub_fe004ef0 in the 6.5.9 image. It makes the same run of calls to
  0xfe004610, with constants 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474, and calls 0xfe004ff4 at
  0xfe004f38, branching on its result at 0xfe004f40. It also stores a word to 0xfa200004 at 0xfe004f84. The build difference is
  in the callee addresses and constants, so the message role is inferred from the shared shape.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine with no calls and no loads. It stores constants to the 0xfa200000 page: halfwords
  0xff to 0xfa200952, 0 to 0xfa200950, 0xfa200954 and 0xfa200962, 0x704 to 0xfa200960, 0 to 0xfa200964 and
  0x700 to 0xfa200966, words 0x30 to 0xfa200abc, 0x3030 to 0xfa200ab8 and 0x3000 to 0xfa200ac4 and the
  halfword 0x30 to 0xfa200ac2 (for example `sth` at 0xfe0051a8 and `stw` at 0xfe0051bc), then returns. Every
  stored value is an immediate and the stores match those of sub_fe005194 in 6.5.9, so this may be a fixed
  initial-state write for those registers. The role is not settled.
  Reviewed: 2026-11-12

- `sub_fe044b94` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe044b94
  **Inferred.** Stores to the halfword at 0xfa20000e [@0xfe044bc8] [@0xfe044bd8] and again [@0xfe044ee4] [@0xfe044efc]. It loads the word at abs 0x705c84 [@0xfe044be8]. It stores words at 0x705cb4 [@0xfe044c4c], 0x705cb8 [@0xfe044c50], 0x705cb0 [@0xfe044c54] and 0x705cbc [@0xfe044c58], and later rewrites the word at 0x705cb0 [@0xfe044c8c]. Its calls to 0xfe044770 include [call 3], [call 10], [call 11], [call 13] and [call 14], and it calls 0xfe044704 at [call 4]. It calls 0xfe0448dc with r3=0x0, r4=0x400000 and r5=[0x705cb4] [call 12]. It returns the word at abs 0x705c84 [@0xfe044f54]. Role (from facts): computes and stores four words at 0x705cb0 to 0x705cbc [@0xfe044c54] [@0xfe044c58], passes the word at 0x705cb4 to 0xfe0448dc [call 12] and returns the word at 0x705c84 [@0xfe044f54], among calls to 0xfe044770 and 0xfe044704 whose roles are not in these facts.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Sets bits 0x30 in the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 (read, OR,
  write back, 0xfe048ba8 to 0xfe048bd4). It stores constants to 0xfa203c80, 0xfa203c82, 0xfa203c84, 0xfa203c85,
  0xfa203c86 and 0xfa203cac, writes the halfword 0x11 to 0xfa2009c0 and polls that halfword in the loop at
  0xfe048c10 to 0xfe048c18 until bit 0 reads clear. It then stores bytes to 0xfa200860, 0xfa200864, 0xfa200868, 0xfa200870 and 0xfa200874, writes 1 to the flag word at abs 0x705cd8 at 0xfe048c44 and 1 to 0xfa200030, and returns.
  It is called by sub_fe048c54 at 0xfe048c90. It matches sub_fe048b88 in 6.5.9 except for the stored
  constants (0x100 not 0x80 at 0xfa203c86, 0x80 not 0x34 at 0xfa200864, 4 not 7 at 0xfa200868), the extra
  stores to 0xfa203cac and 0xfa200860, and the flag address. The role is not settled.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Saves its four arguments (r3 to r6) in r28 to r31. It tests the flag word at 0x705cd8 at
  0xfe048c8c and calls sub_fe048ba0 at 0xfe048c90 only when the flag is zero. It then reads the halfwords at
  0xfa203c80 and 0xfa203c82, builds a buffer pointer from 0x706d6c rounded to 16 bytes and stores a
  descriptor made from its four arguments into it and into two slots near 0xfa2009c0 chosen by those halfwords. Around those stores it
  writes bytes 1, 0xff and 0x81 to 0xfa200860, 0xfa200870 and 0xfa20086c. It runs two polling loops on the
  halfword at slot r5 until bit 0x8000 clears, each bounded by a `bdnz` count of 0x989680 (the loop with the `bdnz`
  at 0xfe048d44 and the second at 0xfe048de4). On a timeout it calls 0xfe044770 at 0xfe048e08 with r3=0x705720 or
  0x705740 and returns -1. Otherwise it returns the halfword at r7+2. It resembles sub_fe048c30 in 6.5.9 but adds the timeout checks.
  The role is not settled.
  Reviewed: 2026-11-12

- `sub_fe0490cc` in `cd-ist-3.24/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0490cc
  **Inferred.** Writes words into the record at r3 +0x0, +0x4, +0x8, +0xc and +0x18 [@0xfe049120] [@0xfe049124] [@0xfe049128] [@0xfe049130] [@0xfe049110]. It stores halfwords to 0xfa2009c0 through 0xfa2009d2 [@0xfe049138] [@0xfe049148] [@0xfe049158] and reads 0xfa2009c0 back [@0xfe04913c]. A byte loop at 0xfe0491a0 loads with lbzx [@0xfe0491a0] and stores with stbx [@0xfe0491a4], and its branch at 0xfe0491c8 taken goes back to 0xfe0491a0 [@0xfe0491c8 taken]. It calls 0xfe048e80 with r3=0x4 [call 10] and r3=0x5 [call 11], and returns ret(0xfe048e80) [@0xfe04921c]. Role (from facts): fills a small record and a halfword area [@0xfe049120] [@0xfe049138], runs a byte copy loop [@0xfe0491a0], and returns ret(0xfe048e80) [@0xfe04921c].
  Reviewed: 2026-11-12
