# Readings: `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)`

One entry per routine, filed here from the reading batches by binary.

- `sub_fe004efc` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Sets up a 0x20-byte frame and stores a word to 0xfa200004 at 0xfe004f84. It makes a sequence of
  calls to 0xfe004610 with the constant addresses 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474
  (0xfe004f30 to 0xfe004f8c), and calls 0xfe0051f0 with r3=0xe01868 at 0xfe004f24. It calls 0xfe004ff4 and
  0xfe004f98 with r3=0xfe040000, and branches at 0xfe004f40 on the result of 0xfe004ff4, so the later calls
  depend on that result. The constant addresses are not read here, so what the calls report is unknown.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A routine with no frame and no calls. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954
  and 0xfa200962 to 0xfa200966, and stores words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, with a halfword to
  0xfa200ac2 (stores at 0xfe0051a8 to 0xfe0051e8). Its return at 0xfe0051ec gives back its r3 unchanged. It
  looks like a field initialiser for the 0xfa2009xx and 0xfa200axx block, but the field names are not known.
  Reviewed: 2026-11-12

- `sub_fe044b94` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe044b94
  **Inferred.** Stores to the halfword at 0xfa20000e [@0xfe044bc8] [@0xfe044bd8] and again [@0xfe044ee4] [@0xfe044efc]. It loads the word at abs 0x705c84 [@0xfe044be8]. It stores words at 0x705cb4 [@0xfe044c4c], 0x705cb8 [@0xfe044c50], 0x705cb0 [@0xfe044c54] and 0x705cbc [@0xfe044c58], and later rewrites the word at 0x705cb0 [@0xfe044c8c]. Its calls to 0xfe044770 include [call 3], [call 10], [call 11], [call 13] and [call 14], and it calls 0xfe044704 at [call 4]. It calls 0xfe0448dc with r3=0x0, r4=0x400000 and r5=[0x705cb4] [call 12]. It returns the word at abs 0x705c84 [@0xfe044f54]. Role (from facts): computes and stores four words at 0x705cb0 to 0x705cbc [@0xfe044c54] [@0xfe044c58], passes the word at 0x705cb4 to 0xfe0448dc [call 12] and returns the word at 0x705c84 [@0xfe044f54], among calls to 0xfe044770 and 0xfe044704 whose roles are not in these facts.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Reads and rewrites the words at 0xfa200abc and 0xfa200ab8 (loads at 0xfe048ba8 and 0xfe048bb8,
  stores at 0xfe048bb4 and 0xfe048bc4), ORs 0x30 into the halfword at 0xfa200ac2 (0xfe048bc8 to 0xfe048bd4), and
  writes fields at 0xfa203c80 to 0xfa203cac (stores at 0xfe048bd8 to 0xfe048bf8). It writes 0x11 to the halfword
  at 0xfa2009c0 (0xfe048c08), then polls that halfword until bit 0 clears (loop at 0xfe048c10 to 0xfe048c18).
  After the poll it makes five byte stores to 0xfa200860 to 0xfa200874 (0xfe048c1c to 0xfe048c38), and writes 1
  to 0x705cd8 and 0xfa200030 at 0xfe048c44 and 0xfe048c4c. The routine at 0xfe048c54 calls it at 0xfe048c90 with
  r9=0x700000 when the word at 0x705cd8 is zero. The purpose of the writes to 0xfa203c8x is not settled.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** A routine with a 0x20-byte frame that loads the word at 0x705cd8 (0xfe048c74) and calls
  0xfe048ba0 at 0xfe048c90 with r9=0x700000 only when that word is zero (the bne at 0xfe048c8c skips the call when
  it is nonzero). It reads the halfwords at 0xfa203c80 and 0xfa203c82 and adds 0x1640 and the base 0xfa2009c0 to
  form two addresses. It stores bytes to 0xfa200860, 0xfa20086c and 0xfa200870, and runs two counted loops (bdnz
  at 0xfe048d44 and 0xfe048de4, count 0x989680) that poll a halfword until its bit 0x8000 clears. When either
  poll runs out it calls 0xfe044770 at 0xfe048e08 with r3=0x705720 or 0x705740 and returns -1. The meaning of the
  descriptors and the purpose are unclear.
  Reviewed: 2026-11-12

- `sub_fe0490cc` in `patches/SG0005762/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0490cc
  **Inferred.** Writes words into the record at r3 +0x0, +0x4, +0x8, +0xc and +0x18 [@0xfe049120] [@0xfe049124] [@0xfe049128] [@0xfe049130] [@0xfe049110]. It stores halfwords to 0xfa2009c0 through 0xfa2009d2 [@0xfe049138] [@0xfe049148] [@0xfe049158] and reads 0xfa2009c0 back [@0xfe04913c]. A byte loop at 0xfe0491a0 loads with lbzx [@0xfe0491a0] and stores with stbx [@0xfe0491a4], and its branch at 0xfe0491c8 taken goes back to 0xfe0491a0 [@0xfe0491c8 taken]. It calls 0xfe048e80 with r3=0x4 [call 10] and r3=0x5 [call 11], and returns ret(0xfe048e80) [@0xfe04921c]. Role (from facts): fills a small record and a halfword area [@0xfe049120] [@0xfe049138], runs a byte copy loop [@0xfe0491a0], and returns ret(0xfe048e80) [@0xfe04921c].
  Reviewed: 2026-11-12
