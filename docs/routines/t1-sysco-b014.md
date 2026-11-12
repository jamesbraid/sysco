# Tier-1 routine readings, sysco batch 14

Role statements for targets 350 to 374 of the tier-1 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). Every entry is
inferred: the rendered facts show what the code touches, not what it is for. Where a
constant or callee suggests a purpose, the entry says so and says it is a guess.

- `sub_ffe73e48` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73e48
  **Inferred.** Reads six 32-bit fields at offsets +0 to +20 of the record that the stack argument points to and range-checks them with signed compares (0xffe73e62 to 0xffe73e96). It jumps to the epilogue at 0xffe7405e (0xffe73e9e) without calling anything when the field at +0x14 is above 0x258, or is above 0 and below 0x1e, or when +0x10 is above 0xc, +0xc above 0x1f, +0x8 above 0x18, +0x4 above 0x3c or +0 above 0x3d. Otherwise it calls 0xffe75938 directly (0xffe73ea4) with the record pointer and d3 to d7 pushed, then calls 0xffe75b28 (0xffe73eb4). It calls 0xffe75b28 a second time (0xffe73ed2) only when the word at +0x20 of the object the first call returned is nonzero (the beq at 0xffe73ec2). It then adds 1900 to the field at +0x14 and 1 to the fields at +0x10 and +0x18 of the returned object (0xffe73ee2 to 0xffe73eea) and does load-and-store byte work on 0x200007f8 to 0x200007ff. The limits look like a date or time record check, but nothing in the facts names the record, so that reading is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fff63e48` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xfff63e48
  **Inferred.** The same code shape as the routine at 0xffe73e48, at the same offsets in a second copy in this image: the same six-field limit checks with the same bail-out branches, the same three call sites (the second call to the helper is conditional, as in the twin) and the same byte-wise load-and-store work on 0x200007f8 to 0x200007ff. The facts differ only in the call targets, which are direct calls to 0xfff65938 and 0xfff65b28 (the copy's own addresses), and in the branch targets inside the copy. Purpose is unknown for the same reason as its twin.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe7aa08` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#5)` at 0xffe7aa08
  **Inferred.** A third copy of the same six-field limit checker, this one with direct calls to 0xffe7e518 (0xffe7aa64) and 0xffe7e708 (0xffe7aa74, and again at 0xffe7aa92 only when the word at +0x20 of the returned object is nonzero). It does load-and-store byte work on the flag bytes at 0x200007f8 to 0x200007ff and ends with a bclr.b at 0xffe7ac18 on an address the facts do not resolve. The stored values are not in the facts, so the byte work is not shown to be clearing. It has the same shape as the copy at 0xffe73e48, and its intended use is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A PowerPC prologue (stwu at 0xfe004efc) followed by calls to 0xfe004610 with the constants 0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01474 and 0xe01428 again in r3 (0xfe004f30 to 0xfe004f8c). It also calls 0xfe0051f0 with r3=0xe01868 and r9=0xfe040000 (0xfe004f24), calls 0xfe004ff4 and 0xfe004f98 with r3=0xfe040000, and ends with a call to 0xfe0046c8 that takes the result of the last 0xfe004610 call. The branch at 0xfe004f40 tests the result of 0xfe004ff4 and skips the calls from 0xfe004f4c to 0xfe004f74 (to 0xfe004f78) when it is nonzero. The routine stores 0xffffff80 to 0xfa200004 at 0xfe004f84 before its final calls. The constants passed to 0xfe004610 may be addresses of messages, which is a guess from call shape alone.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine that stores halfwords and words into the 0xfa200000 block at 0xfa200950 to 0xfa200966 (0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966), and at 0xfa200ab8, 0xfa200abc, 0xfa200ac2 and 0xfa200ac4, then returns its r3 argument unchanged. Routine 0xfe048ba0 touches the same words at 0xfa200ab8 to 0xfa200ac2, which may mean the two are related, but the facts do not show how. Which device the block belongs to is not stated in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Loads and stores back the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 (the facts do not show what changes), then writes halfword and byte fields at 0xfa203c80 to 0xfa203cac and the halfword 0x11 to 0xfa2009c0. It then re-reads a halfword with lhz at 0xfe048c10 in a loop that repeats while bit 0 of the value is set (the bne at 0xfe048c18), writes five bytes through a base the facts leave unresolved (0xfe048c1c to 0xfe048c38), stores a word (1) to 0x705cd8 at 0xfe048c44, the flag that routine 0xfe048c54 reads, and stores a word (1) to 0xfa200030 before the return. No calls.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Reads the word at 0x705cd8 (r9=0x700000) and calls 0xfe048ba0 only when it is zero (the bne at 0xfe048c8c skips the call at 0xfe048c90 when it is nonzero); routine 0xfe048ba0 stores 1 to that word just before returning. It then reads the halfwords at 0xfa203c82 and 0xfa203c80 (0xfe048c9c, 0xfe048cb8), stores bytes, halfwords and a word through bases the facts leave unresolved, writes bytes to 0xfa200860, 0xfa200870 and 0xfa20086c (0xfe048cf0 to 0xfe048d00), and has two counted loops (bdnz at 0xfe048d44 and 0xfe048de4). It finishes with a call to 0xfe044770 at 0xfe048e08. The loops and the flag at 0x705cd8 are the clearest structure; the purpose of the loops is not settled.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Calls 0xc0004710 with its argument registers untouched, reads the word at 0xfa200284 and writes a word back at +0x284 of the same base (0xc000a600), reads a byte at +0x28c (0xc000a60c) and stores it to the stack. It then calls 0xc0010640 with r3=0xc00effd8. It has no return: the last instruction, at 0xc000a620, is a jump to itself. Routine 0xc0010640 takes a string-like constant in many routines in this image, so it may be a message print, but that is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores 1 to 0xfa200030 and stores to the kernel words 0xc015e4e8, 0xc015e4ec, 0xc015e4f4 and 0xc015e4f8. It calls 0xc000b454 (0xc00a23b8) with r3 unchanged, r4=r3+4096, r5=0xfa200000, r6=0xc0160000, r7=1 and r8 to r10=0xc0160000, then reads and writes a word at the address that call returns (0xc00a23bc, 0xc00a23cc). The argument pattern, a page-sized offset and a fixed base, looks like a mapping or region setup. The callee is not resolved in the facts, so that reading stays a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores the words 0xe45f00 to 0xfa200940 and 0 to 0xfa200948 (0xc00a2414, 0xc00a2418). It calls 0xc0003300 with r3=5 and r4=0xc00a247c (routine 0xc00a247c) and branches on the result at 0xc00a2438. Only when the result is nonzero does it call 0xc0010094 with the string-like r3=0xc0108ea0. It then calls 0xc00a250c with r3=0, r4=0xc00a2508 and r5=0, reads the word at 0xfa200940 (0xc00a245c) and writes it back at 0xc00a2464, and returns the result of the 0xc00a250c call. Passing the address of the next routine as a function argument suggests a registration call, but 0xc0003300 is not resolved here.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores the halfword 1 to 0xfa200930, reads it back, and uses a value derived from it as the index of two table loads with lwzx (0xc00a24ac, 0xc00a24c0). It looks up one table entry, it does not walk the table. When the first loaded word is nonzero it makes an indirect call (blrl at 0xc00a24c8, target unknown); otherwise it loads and stores back a word at +0x948 from the r8 base (0xc00a24d0, 0xc00a24e0). It then stores a word to 0xfa20094c. It may be the callback registered by the routine at 0xc00a23ec, given the 0xfa2009xx words both use, but that link is inferred.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves r27 to r31 with stmw and restores them with lmw. When its r4 argument is zero (the bne at 0xc00a2528) it calls 0xc00a2594 (0xc00a252c) and returns. Otherwise it loads a word from an indexed table (lwzx at 0xc00a2540) and, only when that word is nonzero, calls 0xc0010640 with r3=0xc0108ebc (0xc00a2558). It then stores two words into the table with stwx (0xc00a2560, 0xc00a2564), and reads (0xc00a256c) and writes back (0xc00a257c) the word at 0xfa200948, which routine 0xc00a23ec initialises to 0. It may be a register-or-clear routine for table slots with 0xc00a2594 as the clear side, which is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A short leaf that stores two words with indexed stores (stwx at 0xc00a25a8 and 0xc00a25ac), reads the word at 0xfa200948, and stores a word to an unresolved address before returning. Its role is not clear from the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4e98` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Saves registers with stmw. It reads a halfword through an unresolved pointer and, when its 0x8000 bit is set (the beq at 0xc00a4ec0), calls 0xc0010640 with r3=0xc0109480, stores 0 at +0x9c of an unresolved base and returns early. Otherwise it stores halfword and word fields through unresolved pointers (there is no backward branch, so no loop), calls 0xc0004868, then 0xc00046d4 followed by 0xc0004710 with its result, reads the kernel word at 0xc015e178, stores to 0xfa200e54, and finishes with a call to 0xc00046e0 (0xc00a4fc0). The enter and exit pattern (0xc00046d4 then 0xc0004710, then 0xc00046e0) recurs in routines 0xc00a515c and 0xc00a56e0, so those three calls may be a lock or a bracket pair, but that is not shown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a515c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** A 688-byte routine that saves r23 to r31 with stmw. It reads the word at 0xfa200e44 and returns without doing the rest when that word is zero (branch at 0xc00a5188 to the epilogue at 0xc00a53f8). Otherwise it stores the word back (0xc00a51a0), re-reads it at 0xc00a53ec and goes round again while it is nonzero (0xc00a53f4). In between it runs many loads and stores through unresolved pointers and calls 0xc0010640 with r3=0xc0109538 (0xc00a51b4) and r3=0xc0109548 (0xc00a527c), both conditional, 0xc00a540c (0xc00a51c4), 0xc00a563c (0xc00a53e8), and the 0xc00046d4, 0xc0004710 and 0xc00046e0 bracket, 0xc0013dec and 0xc0013764. The 0xfa200e44 word is the one that routine 0xc00a5eb0 writes, so this may be the handler gated on that device state, but the facts do not show it.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a563c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Works on a list at the kernel words 0xc015e504 and 0xc015e508. It stores 0 to 0xfa200e84 (0xc00a5664) and reads 0xfa200e80 (0xc00a5668) first. When the word at 0xc015e508 is zero it calls 0xc0010640 with r3=0xc01095b4 and returns. Otherwise it loads the words at +4 and +8 of the object at 0xc015e508. When the word at +4 is nonzero it makes an indirect call through that word (blrl at 0xc00a5698) with r3 set to the value read from 0xfa200e80. It then moves the object's +8 link into 0xc015e508, stores the old 0xc015e504 into the object's +8, stores the object into 0xc015e504 and, if the new 0xc015e508 is nonzero, writes the word at +0 of that object to 0xfa200e80 and 0x14 to 0xfa200e84. The shape looks like a queue or callback dispatch, which is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a56e0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Uses the kernel words 0xc015e504, 0xc015e508 and 0xc015e50c (routine 0xc00a563c uses the first two). It calls the 0xc00046d4 and 0xc0004710 pair first and 0xc00046e0 last (r10=[0xc015e504] in the facts). When the word at 0xc015e504 is zero it skips the rest and returns 1, otherwise 0. Otherwise it takes the object at 0xc015e504 off that list (0xc015e504 becomes the object's +8 link), stores its first two arguments at +0 and +4 of the object and 0 at +8, and links the object onto the list at 0xc015e508 and 0xc015e50c. When 0xc015e508 was zero it also stores 0x14 to 0xfa200e84 and the first argument to 0xfa200e80. It may be the counterpart of routine 0xc00a563c, but that pairing is inferred from the shared addresses alone.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5eb0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A setup routine (568 bytes) that stores four words to 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, reads 16-bit values from the kernel table at 0xc012cac8 and 0xc012caca, and runs byte and halfword loops with stbx, sth and lhzu. It calls 0xc00046a0, 0xc00aa0a0, 0xc0004c2c (with r9 from 0xc015e518 and r10=8), and 0xc00a56e0 (routine 0xc00a56e0), with r3=0x600a0000 and r4=0xc00a5a60. It writes the 0xfa200e44 word that routine 0xc00a515c reads. The field layout and the 0x600a0000 argument are not explained by the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01324f0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01324f0
  **Inferred.** Stores two words (0xaa3355cc, 0x55ccaa33) to 0xfa200380 and reads 0xfa200280 (0xc0132518 to 0xc0132524), reads halfwords and stores halfwords into bases the facts leave unresolved, and calls 0xc0010640 with r3=0xc00eff90 and r5=0x3c. It calls 0xc0004630, then 0xc0003300 with r3=0xf and r4=0xc000a584, and branches on the result at 0xc01325e8. Only when the result is nonzero does it call 0xc0010094 with r3=0xc00effb8. The pattern matches routine 0xc00a23ec: a registration call through 0xc0003300 followed by a conditional print, so the two may be parallel setup routines, which is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013b7e4` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013b7e4
  **Inferred.** The largest routine in the batch (1892 bytes). It stores a long run of words and halfwords into the kernel block from 0xc0196ab0 up to at least 0xc0196b64, with further stores at 0xc0196b7c to 0xc0196c28. It calls 0xc000cbe4 with r3=0xc0196ab0 and r4=0xc4 (196), calls 0xc000c8d4 with r3=0xc0196acc, r4=0xc017c40c and r5=0x2c (44), and again later with other arguments, and calls 0xc002778c with r3=0x90 and r4=0xf0. It calls 0xc00a25cc and 0xc00a2600 with r3 set to 0x20 and 0x80, twice each (0xc013bc40, 0xc013bc4c, 0xc013bcf4, 0xc013bd00), and calls 0xc00a250c (0xc013bf04) near the end. It reads the kernel words at 0xc012f684, 0xc012f688, 0xc015e4e8 and 0xc015e500. The callees are not resolved, so whether 0xc000cbe4 clears a region and 0xc000c8d4 copies, as the size arguments suggest, is a guess. This looks like a large initialization of a kernel table, but the table's use is not given by the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe486c4` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe486c4
  **Inferred.** A coldfire routine (572 bytes) that sets and clears bits in the flag byte at 0x200007f8 (bset.b at 0xffe486d2, bclr.b at 0xffe48802) and reads the status bytes at 0x200007f9 to 0x200007ff. It adds a longword loaded from the table at 0x30000590, indexed by d6 times 4 (0xffe4881c, 0xffe48894), into a running value. It then pushes 0x4a and calls 0xffe43b70 (0xffe4889e). Depending on compares against its result it calls it again with 0x4c (0xffe488ae) and then with 0x49 (0xffe488be), or instead with 0x48 (0xffe488d2), so two or three of the four call sites run on any path, not all four. It stores the call result to 0x3000fc18 at 0xffe488c6 or 0xffe488da. When its stack argument is nonzero it stores a longword through it (0xffe488f0). Its role is not settled; the bit flags suggest a status or control word for a device, which is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe48a44` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe48a44
  **Inferred.** A coldfire routine (680 bytes) with the same flag-byte pattern as routine 0xffe486c4: bset.b on 0x200007f8 at 0xffe48a62, bclr.b on it at 0xffe48cdc, clr.b of 0x200007ff (0xffe48ab8) and of 0x200007f9 (0xffe48be0), and paired loads and stores of the bytes 0x200007fa to 0x200007fe. It loads four stack arguments into d7, d6, d3 and d2 (0xffe48a50 to 0xffe48a5c) and pushes and pops intermediate values on the stack around divs.l operations. It loads a longword from the table at 0x30000590 (0xffe48c3c). The facts show no direct calls, so the callee structure is not known.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe48cec` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe48cec
  **Inferred.** A coldfire routine (480 bytes) with no direct calls in the facts. It sets the flag at 0x200007f8 (bset.b at 0xffe48cf2), applies bclr.b to it at 0xffe48e60 and reads the status bytes at 0x200007f9 to 0x200007ff. Between those it makes straight-line add.l updates of stack slots (0xffe48d30, 0xffe48d4c, 0xffe48d5c and others) with no backward branch, so there is no loop, then a sign test at 0xffe48e7e. It loads a longword from the table at 0x30000590 indexed by a stack value (0xffe48e70), copies nine longwords between unresolved pointers (0xffe48eb2 to 0xffe48ec2) and returns with d0=0x0. The constant return suggests a status code, which is a guess.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe49498` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe49498
  **Inferred.** A coldfire routine (336 bytes) that sets and clears the flag at 0x200007f8, reads the byte at 0x200007ff, and does a signed multiply (muls.l at 0xffe4953e). It calls the local routine at 0xffe495e8 twice (bsr.w at 0xffe49548 and 0xffe49580) and calls 0xffe43438 twice through jsr (0xffe495c4, 0xffe495d2), then returns with d0=0x0. The shape resembles a small status computation with a helper, but no field is named.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffeb86c4` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffeb86c4
  **Inferred.** The same coldfire shape as routine 0xffe486c4 at a second address in this image (572 bytes), with the same flag-byte operations on 0x200007f8 and the same table read at 0x30000590. Its four call sites sit at the same offsets (0xffeb889e, 0xffeb88ae, 0xffeb88be, 0xffeb88d2) and call 0xffeb3b70 where routine 0xffe486c4 calls 0xffe43b70, so the copies are 0x70000 apart. As in the twin, two or three of the four call sites run on any path. Its role is as unsettled as its twin's.
  Reviewed: 2026-11-12 (sonnet)
