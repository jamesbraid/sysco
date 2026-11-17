# Readings: `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)`

One entry per routine, filed here from the reading batches by binary.

- `sub_fe004efc` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A flash-resident routine that never returns. It calls 0xfe0051f0 with 0xe01868 first, then the message
  emitter at 0xfe004610 with 0xe013ec, and tests the result of 0xfe004ff4 with r3=0xfe040000 (bne at 0xfe004f40). Only
  when that result is zero it emits 0xe01418, calls 0xfe004f98 and emits 0xe01428, 0xe01454 and 0xe01428 again. On
  either path it emits 0xe01474 (the emitter runs six times with five distinct constant pointers when the result is
  zero), stores 0xffffff80 to the device word 0xfa200004, calls 0xfe0046c8 and ends in a branch to itself
  (0xfe004f94). It looks like a boot-time check that reports its progress.
  Reviewed: 2026-11-12

- `sub_fe0051a0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A short leaf with no calls that stores immediate constants to fixed device words: halfwords at
  0xfa200950 (0), 0xfa200952 (0xff), 0xfa200954 (0), 0xfa200960 (0x704), 0xfa200962 (0), 0xfa200964 (0), 0xfa200966
  (0x700) and 0xfa200ac2 (0x30), and words at 0xfa200ab8 (0x3030), 0xfa200abc (0x30) and 0xfa200ac4 (0x3000). It does
  not touch r3, so r3 returns unchanged. The stores are constants, not copies of anything the routine reads;
  sub_fe048ba0 read-modify-writes three of these words, so the pair looks like an initialise and then adjust of one
  device block.
  Reviewed: 2026-11-12

- `sub_fe044b94` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe044b94
  **Inferred.** Stores to the halfword at 0xfa20000e [@0xfe044bc8] [@0xfe044bd8] and again [@0xfe044ee4] [@0xfe044efc]. It loads the word at abs 0x705c84 [@0xfe044be8]. It stores words at 0x705cb4 [@0xfe044c4c], 0x705cb8 [@0xfe044c50], 0x705cb0 [@0xfe044c54] and 0x705cbc [@0xfe044c58], and later rewrites the word at 0x705cb0 [@0xfe044c8c]. Its calls to 0xfe044770 include [call 3], [call 10], [call 11], [call 13] and [call 14], and it calls 0xfe044704 at [call 4]. It calls 0xfe0448dc with r3=0x0, r4=0x400000 and r5=[0x705cb4] [call 12]. It returns the word at abs 0x705c84 [@0xfe044f54]. Role (from facts): computes and stores four words at 0x705cb0 to 0x705cbc [@0xfe044c54] [@0xfe044c58], passes the word at 0x705cb4 to 0xfe0448dc [call 12] and returns the word at 0x705c84 [@0xfe044f54], among calls to 0xfe044770 and 0xfe044704 whose roles are not in these facts.
  Reviewed: 2026-11-12

- `sub_fe048ba0` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Sets bits in the device registers at 0xfa200abc and 0xfa200ab8 (word read-modify-write, OR 0x30) and
  0xfa200ac2 (halfword, OR 0x30), the same addresses sub_fe0051a0 initialised. It stores constants to a block at
  0xfa203c80 (halfwords at +0x0 and +0x2, bytes at +0x4 and +0x5, halfwords at +0x6 and +0x2c), writes 0x11 to the
  halfword at 0xfa2009c0 and polls it until bit 0 clears (halfword loop at 0xfe048c10 to 0xfe048c18), then stores
  bytes at 0xfa200860, 0xfa200864, 0xfa200868, 0xfa200870 and 0xfa200874 and finishes by storing 1 to 0x705cd8 and to
  0xfa200030. The use of the 0xfa203c80 block is not in the facts.
  Reviewed: 2026-11-12

- `sub_fe048c54` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Calls sub_fe048ba0 only when the word at 0x705cd8 is zero (bne at 0xfe048c8c skips the call on
  nonzero). It then reads the halfwords at 0xfa203c82 and 0xfa203c80, aligns a buffer address near 0x706d6c to 16
  bytes, writes status bytes at 0xfa200860, 0xfa200870 and 0xfa20086c, and runs two timed polling loops (bdnz at
  0xfe048d44 and 0xfe048de4) that wait for bit 0x8000 of a halfword to clear, with a counter of 0x989680. On a timeout
  it calls 0xfe044770 with a constant pointer (0x705720 or 0x705740) and returns -1; otherwise it returns the halfword
  at +0x2 of a descriptor. The meaning of the status bytes is not resolved.
  Reviewed: 2026-11-12

- `sub_fe0490cc` in `patches/SG0007149/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0490cc
  **Inferred.** Writes words into the record at r3 +0x0, +0x4, +0x8, +0xc and +0x18 [@0xfe049120] [@0xfe049124] [@0xfe049128] [@0xfe049130] [@0xfe049110]. It stores halfwords to 0xfa2009c0 through 0xfa2009d2 [@0xfe049138] [@0xfe049148] [@0xfe049158] and reads 0xfa2009c0 back [@0xfe04913c]. A byte loop at 0xfe0491a0 loads with lbzx [@0xfe0491a0] and stores with stbx [@0xfe0491a4], and its branch at 0xfe0491c8 taken goes back to 0xfe0491a0 [@0xfe0491c8 taken]. It calls 0xfe048e80 with r3=0x4 [call 10] and r3=0x5 [call 11], and returns ret(0xfe048e80) [@0xfe04921c]. Role (from facts): fills a small record and a halfword area [@0xfe049120] [@0xfe049138], runs a byte copy loop [@0xfe0491a0], and returns ret(0xfe048e80) [@0xfe04921c].
  Reviewed: 2026-11-12
