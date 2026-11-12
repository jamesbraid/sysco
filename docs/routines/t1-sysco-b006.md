# Routine readings, sysco tier-1 batch 6

Role statements for targets 150 to 174 of the tier-1 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). Every entry is inferred. The callees are
unnamed in every image here, so the helper roles below come from call shape, string-looking
arguments and constants, and none of them is confirmed.

- `sub_ffe7195c` in `6.5.19/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe7195c
  **Inferred.** It takes one stack argument, a pointer to a record, and compares six longword fields of that record against fixed limits, branching on each compare. The field at +0x14 must be at most 0x258 and, when it is above zero, at least 0x1e; the fields at +0x10, +0xc, +0x8, +0x4 and +0x0 must be at most 0xc, 0x1f, 0x18, 0x3c and 0x3d. Any failure returns the constant 0xfffff7cc. Otherwise it calls 0xffe713b0 with the record pointer as the only stack argument, then calls 0xffe715a0 with the address of a local longword, and calls 0xffe715a0 a second time the same way only when the longword at +0x20 of the first call's result is non-zero. On success it also adds 0x76c to the field at +0x14, increments the fields at +0x10 and +0x18, and stores packed-decimal-looking bytes derived from the fields to 0x200007f8 through 0x200007ff. The limits resemble calendar and clock maxima, so this may validate a broken-down date or time and write it to a clock, but nothing here names the fields.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** It calls 0xfe0051f0 with 0xe01868, then calls the helper at 0xfe004610 with 0xe013ec, and calls 0xfe004ff4 with 0xfe040000 in r3. Only when 0xfe004ff4 returns zero does it run the block at 0xfe004f44 to 0xfe004f74, which calls 0xfe004610 with 0xe01418, calls 0xfe004f98 with the same base in r3, then calls 0xfe004610 with 0xe01428, 0xe01454 and 0xe01428 again; a non-zero return branches at 0xfe004f40 over that block to 0xfe004f78. From 0xfe004f78 it always stores 0xffffff80 at 0xfa200004, calls 0xfe004610 with 0xe01474 and calls 0xfe0046c8, then loops on itself at 0xfe004f94, so it does not return. The 0xe01xxx arguments look like message strings, so 0xfe004610 is probably a printer, and the 0xfa200000 constant may be a board register block. Both are inferred from shape only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** It makes no calls and has no branches, but it does store: it writes constants to the block at 0xfa200000, halfwords at +0x950, +0x952, +0x954, +0x960, +0x962, +0x964, +0x966 and +0xac2, and words at +0xab8, +0xabc and +0xac4 (values 0, 0xff, 0x30, 0x3030, 0x3000, 0x700 and 0x704). It does not touch r3 and returns at 0xfe0051ec. It looks like a block of register initialisation, but the facts do not name the registers.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** It makes no calls. It sets bits 0x30 in words at +0xf8 and +0xfc and in the halfword at +0x102 from the base 0xfa2009c0, stores a few constants through 0xfa203c80, then writes 0x11 to the halfword at 0xfa2009c0 and polls it: the backward branch at 0xfe048c18 to 0xfe048c10 is taken while bit 0 of that halfword (the andi. at 0xfe048c14) is set, with no iteration limit. After the poll it stores a few bytes at 0xfa200860, then stores 1 to the word at 0x705cd8 and to 0xfa200030, and returns at 0xfe048c50 with r3 untouched. It looks like a controller start-up step with a wait for a busy bit, inferred from the accesses only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** It tests the word at absolute 0x705cd8 and calls 0xfe048ba0 only when that word is zero (the branch at 0xfe048c8c skips the call otherwise). It then writes to structures addressed from 0xfa2009c0 and 0xfa203c80, and polls a halfword for bit 0x8000 twice, each poll limited to 10,000,000 iterations by a bdnz at 0xfe048d44 or 0xfe048de4. A timeout in either poll calls 0xfe044770 at 0xfe048e08 with a constant address (0x705720 or 0x705740) and returns -1. When both polls complete it returns the halfword at 2(r7) without that call. The shape looks like a polled transfer with a timeout, inferred from loop structure and constants only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** It opens a 0x20-byte frame, calls 0xc0004710 at 0xc000a5ec, then reads, sets bit 0x80 in and writes back the word at absolute 0xfa200284 (lwz, ori, stw). It reads the machine state register with mfmsr at 0xc000a5f8, clears one bit with rlwinm and writes it back with mtmsr at 0xc000a608. It loads the byte at 0xfa20028c into the stack and calls 0xc0010640 with 0xc00f0164 at 0xc000a61c, which looks like a message. It then branches to itself at 0xc000a620 and never returns, so it looks like a halt or fatal-error path; the facts do not settle what condition leads here.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** It stores 1 at 0xfa200030, stores constants 0x800 and 0xf00 and its r3 argument and r3+0x1000 into words at 0xc01604ec to 0xc01604f8, then calls 0xc000b454 at 0xc00a23b8 with r3 unchanged from entry. It treats the returned value as a pointer, sets bit 0x2 in the word it points to, executes tlbia at 0xc00a23d0, and stores 0xfa2009c0 at 0xc01604e8. The tlbia invalidates the TLB, so this may be an address-space setup step. The callee is unnamed, so the meanings of the stored words are guesses.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** It calls 0xc0003300 at 0xc00a2430 with r3=5 and the address 0xc00a247c, then prints
  through 0xc0010094 with 0xc010a1b8 at 0xc00a2444 only when that call returns non-zero. It then calls
  0xc00a250c at 0xc00a2458 and returns its result. The registration-looking call and the print path
  are inferred from call order and the string-looking constant.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** It writes 1 to the halfword at 0xfa200930, reads it back and uses the value shifted right by 11 as an index into an 8-byte-entry table at 0xc01989b0. When the first word of that entry is non-zero it calls through it (mtlr then blrl at 0xc00a24c8) with the entry's second word in r3 and its own r5 in r4; when it is zero it clears bit n of the word at 0xfa200948 instead. Either way it then writes 1 << n to 0xfa20094c and returns. The 0xc00a2508 instruction is a separate one-instruction blr just after this routine's return.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** It saves registers with stmw at 0xc00a2514 and restores them with lmw at 0xc00a2588. When its r4 argument is zero it calls 0xc00a2594 with r3 (the call at 0xc00a252c) and returns. Otherwise it indexes an 8-byte-entry table at 0xc01989b0 with r3, prints through 0xc0010640 with 0xc010a1d4 at 0xc00a2558 only when the entry's first word is already non-zero, stores r4 and r5 into the entry, and sets bit r3 in the word at 0xfa200948. It looks like installing a handler in a table, with 0xc00a2594 clearing one, but the callee names are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** It makes no calls but it does access memory. It stores zero into both words of the 8-byte table entry at 0xc01989b0 indexed by r3 (stwx at 0xc00a25a8 and 0xc00a25ac), clears bit r3 in the word at 0xfa200948 (lwz at 0xc00a25b4, stw at 0xc00a25c4), and returns at 0xc00a25c8 with r3 unchanged. It is called by 0xc00a250c when that routine's r4 is zero, so it looks like the table-entry removal that goes with 0xc00a250c.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4e98` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** It saves registers with stmw at 0xc00a4ea0 and loads a descriptor pointer from the structure at r4+0x60. If bit 0x8000 of the halfword the descriptor points to is set, it prints through 0xc0010640 with 0xc010a7dc at 0xc00a4ecc, clears the word at +0x9c and returns 1. Otherwise it updates the descriptor, calls 0xc0004868, then 0xc00046d4 with the address of a local and 0xc0004710 with that result (0xc00a4f38 to 0xc00a4f44), and later calls 0xc00046e0 with the local value at 0xc00a4fc0; those calls do not depend on the later branches, and 0xc0004710 receives r3 as 0xc00046d4 left it. The branch at 0xc00a4f78 picks the next descriptor, and the branch at 0xc00a4f98 decides whether to set byte +0x98 and set bit 0 of the word at r4+0x2c with an atomic or. It returns 0 on this path. The callees' meanings are not established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a515c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** It loops on a flag word at +0x44 of one structure and acts on its bits. It prints through 0xc0010640 with 0xc010a894 at 0xc00a51b4 only when bits 0xe0400000 of that word are set, and with 0xc010a8a4 at 0xc00a527c only when bit 0x8000 of a value tested at 0xc00a526c is set. It calls 0xc00a540c at 0xc00a51c4 only when bits 0x03000000 are set, and calls 0xc00a563c at 0xc00a53e8 only when bit 0x00800000 is set. Inside the loop, behind atomic updates and flag tests (the first group only when the value from the atomic exchange at 0xc00a52a8 is zero), it calls 0xc00046d4 then 0xc0004710 then 0xc0013dec(0, 1) then 0xc00046e0 at 0xc00a52c4 to 0xc00a52e8 and again at 0xc00a5384 to 0xc00a53a8, and calls 0xc0013764 at 0xc00a53b0. That last call is inside the loop, not a final step. Nothing here names the items being processed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a563c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** It loads a record pointer from the global at 0xc0160508 and stores zero at 0xfa200e84. When the pointer is null it prints through 0xc0010640 with 0xc010a910 at 0xc00a567c and returns. When it is set and the word at +4 of the record is non-zero, it calls through that word (mtlr, blrl at 0xc00a5698) with the word read from 0xfa200e80 in r3 and its own r3 in r4. It then unlinks the record: it moves the record's +8 word into the global at 0xc0160508, pushes the record onto the list at 0xc0160504, and when a next record exists writes that record's first word to 0xfa200e80 and 0x14 to 0xfa200e84. It looks like dispatching and retiring a queued callback, inferred from the loads, stores and indirect call only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a56e0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** It calls 0xc00046d4 with the address of a local and then 0xc0004710 unconditionally at 0xc00a56fc and 0xc00a5700, before any test. It then tests the global at 0xc0160504: when that is zero it returns 1; otherwise it takes that record off the 0xc0160504 list, stores r3, r4 and zero into the record's +0, +4 and +8 words, and appends it to the list headed at 0xc0160508 and ending at 0xc016050c. When the 0xc0160508 list was empty (the test at 0xc00a5738) it also writes r3 to 0xfa200e80 and 0x14 to 0xfa200e84. It calls 0xc00046e0 at 0xc00a577c with the value saved from the first call's local, not with a global, and returns 0 on that path. Read with 0xc00a563c, which uses the same 0xc0160504 and 0xc0160508 globals, it looks like the enqueue side of that routine's queue, inferred from the shared globals and stores only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5eb0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** It has three bdnz loops at 0xc00a5f58 (a 6-byte copy), 0xc00a6018 and 0xc00a6048 (8-entry, 8-byte-stride tables, the first filled with halfword 0x8000 and the second zeroed, each then getting 0x2000 set in its last entry), and a pointer-table loop at 0xc00a5fb8 to 0xc00a5ffc. That pointer loop releases each non-null pointer with an atomic decrement and calls 0xc00aa0a0 at 0xc00a5fec only when the count was 1 or reaches zero. It calls 0xc00046a0 at 0xc00a5ef0 and calls 0xc0004c2c at 0xc00a6094 with no argument set up in this routine; the returned value is compared with 0x500 to choose the value stored to 0xfa200970. The global at 0xc0160518 is tested at 0xc00a6060 to choose whether 4 and 4 or 6 and 0 are stored to 0xfa200f44 and 0xfa200f84. It then calls 0xc00a56e0 at 0xc00a60c0 with r3 = 0x600a0000 and r4 = 0xc00a5a60, and finishes with a few stores. The shape suggests setting up descriptor rings for a controller, but the table base and the callee roles are not established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01344f0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** It writes the words 0xaa3355cc and then 0x55ccaa33 to 0xfa200380, and to 0xfa200300, 0xfa200320 and 0xfa20030c, and sets bit 0x02000000 in the word at 0xfa200280. It calls 0xc0010640 with r3 = 0xc00f011c and 0x3c in r5 at 0xc0134548, and calls 0xc0004630 at 0xc0134580. It calls 0xc0003300 at 0xc01345e0 with r3 = 0xf, r4 = 0xc000a584, r5 = 0 and r6 = 0xc00f013c, then prints through 0xc0010094 with 0xc00f0144 at 0xc01345f4 only when that call returns non-zero. The same registration-looking call appears in 0xc00a23ec with a different first argument, so this is probably another instance of one setup pattern.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013d7e4` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** It is the largest routine in the batch, 1892 bytes. It calls 0xc0013c38 at 0xc013d800 with r3 = 3 and r4 = 0xc00a3000, and calls 0xc000c8d4 four times (at 0xc013d888, 0xc013d9b4, 0xc013db10 and 0xc013db28) with 0x2c, 0xc4, 0x2c and 0x2c in r5, which look like block copies. It calls 0xc000cbe4 with 0xc4 and 0x90 in r4 at 0xc013d82c and 0xc013dbd4. Prints through 0xc0010640 and 0xc0010094 are interleaved, and the 0xc0010094 prints at 0xc013d9fc and 0xc013da18 follow non-zero returns from 0xc006b73c. It calls 0xc00a25cc with 0x20 and 0xc00a2600 with 0x80 twice each, and calls 0xc00a250c at 0xc013df04 and 0xc00a2630 at 0xc013df1c inside a loop that branches back to 0xc013dad8. The 0xc0198xxx addresses suggest structures, but the copy helper and the size meanings are unnamed.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe58768` in `6.5.20/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe58768
  **Inferred.** It has the same record check as the 6.5.19 build at 0xffe7195c: one stack argument, a record pointer, with the same limits (0x258, 0x1e, 0xc, 0x1f, 0x18, 0x3c and 0x3d) on the same offsets, and a failure path that returns 0xfffff7cc. It calls 0xffe62d84 at 0xffe587c4 with the record pointer as its only stack argument, then 0xffe62f74 at 0xffe587d4 with the address of a local longword, and calls 0xffe62f74 again at 0xffe587f2 only when the longword at +0x20 of the first result is non-zero. The helpers sit at different addresses from the 6.5.19 build, and the facts do not say whether they are the same routines.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** The routine's bytes are identical to the 6.5.19 build at the same address (same code hash in the inventory), so the reading is the same. It calls 0xfe0051f0, then 0xfe004610 with 0xe013ec, then 0xfe004ff4 with 0xfe040000 in r3. Only when that call returns zero does it run the block at 0xfe004f44 to 0xfe004f74, which calls 0xfe004610 with 0xe01418, calls 0xfe004f98 and calls 0xfe004610 three more times; a non-zero return branches over it to 0xfe004f78. From there it always stores 0xffffff80 at 0xfa200004, calls 0xfe004610 with 0xe01474 and 0xfe0046c8, and loops on itself at 0xfe004f94 without returning. The helper roles are inferred, as in the 6.5.19 entry.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** The routine's bytes are identical to the 6.5.19 build at the same address (same code hash in the inventory). It makes no calls and has no branches, but it stores constants to the block at 0xfa200000 (halfwords at +0x950 to +0x966 and +0xac2, words at +0xab8, +0xabc and +0xac4). It leaves r3 untouched and returns at 0xfe0051ec. It looks like register initialisation, not a pass-through, but the facts do not name the registers.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. It has a backward branch
  at 0xfe048c18 to 0xfe048c10, taken while the Z flag from the andi. at 0xfe048c14 is clear, so it looks
  like a polling or scanning loop. The loop bound and the tested value are unknown.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** The routine's bytes are identical to the 6.5.19 build at the same address (same code hash in the inventory). It tests the word at absolute 0x705cd8 and calls 0xfe048ba0 at 0xfe048c90 only when that word is zero. It then polls a halfword for bit 0x8000 twice, each poll limited to 10,000,000 iterations by a bdnz at 0xfe048d44 or 0xfe048de4. A timeout in either poll calls 0xfe044770 at 0xfe048e08 and returns -1; a completed transfer returns without that call. The shape looks like a polled transfer with a timeout, inferred from loop structure and constants only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. It calls 0xc0004710 at
  0xc000a5ec and 0xc0010640 with 0xc00f0164 at 0xc000a61c, which looks like a message print after a
  setup call. The frame and the machine-state accesses are as in the 6.5.19 entry, and their meaning is
  not settled by the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** The routine's bytes are identical to the 6.5.19 build at the same address (same code hash in the inventory). It stores to 0xfa200030 and to words at 0xc01604e8 to 0xc01604f8, calls 0xc000b454 at 0xc00a23b8 with r3 unchanged from entry, treats the returned value as a pointer and sets bit 0x2 in the word it points to, then executes tlbia at 0xc00a23d0. The tlbia invalidates the TLB, so it may be an address-space setup step. The callee is unnamed, so the meanings of the stored words are guesses.
  Reviewed: 2026-11-12 (sonnet)
