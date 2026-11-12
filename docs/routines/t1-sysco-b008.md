# Tier-1 routine readings, sysco batch 8

Role statements for targets 200 to 224 of the tier-1 list, written from the rendered facts only
(registers, calls, constants, branch targets and memory accesses). Each role is a reading of those
facts. All 25 entries are inferred, and none is marked established. The grader accepted all 25,
so none were rejected.

- `sub_c00a5eb0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Non-leaf routine that saves a register block with `stmw` at 0xc00a5eb8 and restores it with `lmw` at 0xc00a60dc. Its loops at 0xc00a5f48 and 0xc00a6010 use a `bdnz` count, and the `lwarx`/`stwcx.` pair at 0xc00a5fd4 and 0xc00a5fdc suggests an atomic update of a shared word. It calls `sub_c00a56e0` at 0xc00a60c0 with r3=0x600a0000, and the helpers at 0xc00046a0, 0xc00aa0a0 and 0xc0004c2c; the purpose is inferred from that shape, not from the callee names.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01344f0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Saves registers with `stmw` at 0xc01344f8 and does a signed divide (`divw` at 0xc0134568 and 0xc0134578) on values it shifts first. It calls `0xc0010640` at 0xc0134548 with a string at 0xc00f011c, then calls `0xc0003300` at 0xc01345e0 with the constant 0xf and the pointer 0xc000a584 and tests its return at 0xc01345e8: only when the return is nonzero does it call `0xc0010094` with the string at 0xc00f0144.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013d7e4` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** Large setup routine of about 1.9 KB that works on blocks in the 0xc0198xxx area. It calls `0xc000cbe4` at 0xc013d82c with r3=0xc0198ab0 and r4=0xc4, and `0xc000c8d4` with r5=0x2c at 0xc013d888 (r3=0xc0198acc, r4=0xc017e40c) and with r5=0xc4 at 0xc013d9b4 (r3=0xc0198b74, r4=0xc0198ab0). It calls `0xc006b73c` at 0xc013d9e4 and 0xc013da04 and, when a return is nonzero (the branches at 0xc013d9f0 and 0xc013da0c), calls `0xc0010094` with a message string. It also calls `0xc0013c38` and `0xc002778c`, and calls `0xc00a25cc` and `0xc00a2600` with 0x20 and 0x80 at 0xc013dc40 to 0xc013dd00. The order of the setup steps is taken from the call sites.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5a240` in `6.5.22/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a240
  **Inferred.** Saves registers with `movem.l` (0xffe5a248) and checks six fields of a record passed at `[sp+4]`. The fields at offsets +0x14, +0x10, +0xc, +0x8, +0x4 and +0x0 are compared with the constants 0x258 and 0x1e (offset +0x14), 0xc, 0x1f, 0x18, 0x3c and 0x3d, which look like calendar or clock limits, and a failed check branches to 0xffe5a290, which loads 0xfffff7cc into d0 and goes to the exit at 0xffe5a456. On the passing path it calls `0xffe59c94` at 0xffe5a29c with the record pointer and `0xffe59e84` at 0xffe5a2ac with the address of a stack word, and it tests a flag at +0x20 of that result at 0xffe5a2b6. Only when the flag is nonzero does it call `0xffe59e84` a second time at 0xffe5a2ca. The field meanings are inferred from the constant limits only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe58ddc` in `6.5.22/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe58ddc
  **Inferred.** Same shape as the `l1#0` copy at 0xffe5a240: it saves registers, range-checks the same six record fields against the same constants (a failed check branches to 0xffe58e2c), and calls `0xffe58830` at 0xffe58e38 with the record pointer as its stack argument. It then calls `0xffe58a20` at 0xffe58e48 with the address of a stack word, tests a flag at +0x20 of its result at 0xffe58e52 (branch at 0xffe58e56), and calls `0xffe58a20` again at 0xffe58e66 only when the flag is nonzero. The code after the second call differs from the `l1#0` copy (`addq.l #1,-$5556(a5)` at 0xffe58e7a where the `l1#0` copy has `addq.l #1,$10(a5)`), so the role is inferred from the shared checks and constants only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Routine in the flash image that never returns: it ends with a call to `0xfe0046c8` at 0xfe004f90 and a branch to itself at 0xfe004f94. It first calls `0xfe0051f0` at 0xfe004f24 with r3=0xe01868, then calls `0xfe004610` six times with string-like constants (0xe013ec at 0xfe004f30, then 0xe01418, 0xe01428, 0xe01454, 0xe01428 again, and 0xe01474 at 0xfe004f8c). It calls `0xfe004ff4` at 0xfe004f38 with the flash window base 0xfe040000 and tests its return at 0xfe004f3c. When the return is nonzero, the branch at 0xfe004f40 skips everything from 0xfe004f44 to 0xfe004f74: the calls of `0xfe004610` with 0xe01418, 0xe01428, 0xe01454 and 0xe01428, and the call of `0xfe004f98` with 0xfe040000. On every path it then stores 0xffffff80 to the board window word at 0xfa200004 (0xfe004f84) before the last `0xfe004610` call. It looks like a status or report sequence that ends in a loop on itself; the messages themselves are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe0051a0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** Leaf routine with no calls. It stores halfwords to the board window at 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, then returns with r3 unchanged (the routine neither reads nor writes r3). It looks like a block of board-register setup values; the facts do not name the registers.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048ba0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Leaf routine with no calls. It reads and rewrites the words at 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 in the board window, and writes bytes and halfwords at 0xfa203c80 through 0xfa203cac. It then stores 0x11 to the halfword at 0xfa2009c0 and polls a halfword through the same register in a loop at 0xfe048c10, where the `bne` at 0xfe048c18 repeats while bit 0 is set. After the loop it stores bytes through a pointer the facts leave unresolved, stores 1 to the word at 0x705cd8 (0xfe048c44), and ends with a store of 1 to 0xfa200030 (0xfe048c4c).
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe048c54` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Flash-region routine that tests the global word at 0x705cd8 and calls `sub_fe048ba0` at 0xfe048c90 only when that word is zero (the `bne` at 0xfe048c8c skips the call otherwise). The r9=0x700000 at that call is a leftover from the load and is not an argument. It then runs two polling loops at 0xfe048d34 and 0xfe048dd4, each reading a halfword, testing bit 0x8000 with `andi.` and counting down with `bdnz` from 0x989680, and leaving early when the bit is clear. If the count reaches zero after either loop (checks at 0xfe048d48 and 0xfe048de8) it calls `0xfe044770` at 0xfe048e08 with a pointer (0x705720 or 0x705740) and returns -1. Otherwise it returns a halfword read at 0xfe048df8 without that call. It does not store to 0x705cd8 itself; that store is in `sub_fe048ba0`.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c000a5e0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Short 68-byte routine with a 32-byte frame (`stwu` at 0xc000a5e0) that never returns: it ends with a branch to itself at 0xc000a620. It calls `0xc0004710` at 0xc000a5ec, reads the board word at 0xfa200284 and stores it back through the same base register with 0x80 or-ed in (0xc000a600), reads and writes the machine state register with `mfmsr` at 0xc000a5f8 and `mtmsr` at 0xc000a608 around an `rlwinm` bit change, reads a byte at +0x28c from the same base (0xc000a60c), and calls `0xc0010640` with the string pointer 0xc00f0164 at 0xc000a61c. The MSR bit being changed is not named in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2374` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Calls `0xc000b454` at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000, r7=1 and r6, r8, r9 and r10 all 0xc0160000, then executes `tlbia` at 0xc00a23d0, which invalidates the TLB. The 4096 step and the fixed window base may point to a mapping set-up, but the facts do not show it, and the callee is not named in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a23ec` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores to the board words at 0xfa200940 and 0xfa200948, then passes the address of `sub_c00a247c` (0xc00a247c) to the helper `0xc0003300` at 0xc00a2430 as r4 with the code 5 in r3, and branches on its return at 0xc00a2438: only when the return is nonzero does it print a message through `0xc0010094` (string at 0xc010a1b8). It then calls `sub_c00a250c` at 0xc00a2458 with r3=0, r4=0xc00a2508 and r5=0, sets bit 0x80 in the word at 0xfa200940, and returns with r3 as that call left it. The same helper `0xc0003300` is also called from 0xc01345e0 with the code 0xf and a code address as r4.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a247c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Bit-manipulation routine: `srwi`, `slwi`, `rotlw`, `and` and `slw` at 0xc00a24a0 to 0xc00a24e8 build a shifted mask, and a `beq` at 0xc00a24b8 selects the path. It makes an indirect call through the link register with `blrl` at 0xc00a24c8, whose target the facts do not resolve, and it returns at 0xc00a2504 with r3 unresolved. It makes no call to `sub_c00a250c`, which is the next routine in the image; the caller `sub_c00a23ec` calls that one. The address is passed to `0xc0003300` by its caller `sub_c00a23ec`, which suggests it is used as a callback, but that is inferred from the call site only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a250c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Branches on its second argument r4 (`or.` at 0xc00a251c, `bne` at 0xc00a2528). When r4 is zero it calls the 56-byte routine `sub_c00a2594` at 0xc00a252c and returns without printing. When r4 is nonzero it indexes an 8-byte-entry table at 0xc01989b0 with its first argument (shifted left by 3) and, only when that entry's first word is already nonzero, prints a message through `0xc0010640` with the string at 0xc010a1d4 (call at 0xc00a2558). It then stores r5 and r4 into the entry and sets bit r3 of the word at 0xfa200948. The message text is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a2594` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Leaf routine of 56 bytes with no calls. It reads the board word at 0xfa200948 (`lwz` at 0xc00a25b4), stores two words with indexed stores (`stwx` at 0xc00a25a8 and 0xc00a25ac) and one more word at 0xc00a25c4, then returns with r3 unknown. Its caller `sub_c00a250c` at 0xc00a252c is the only caller in the batch, so the role is inferred from that call alone.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a4e98` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Tests bit 0x8000 of a halfword read through a pointer chained from r4 (`andi.` at 0xc00a4ebc, `beq` at 0xc00a4ec0). When the bit is set it prints a message through `0xc0010640` (string at 0xc010a7dc), stores 0 at +0x9c of the record loaded from +0x60 of r4, and returns 1, skipping the rest. When the bit is clear it calls `0xc0004868` at 0xc00a4f38, then `0xc00046d4` at 0xc00a4f40 and passes its result to `0xc0004710` at 0xc00a4f44 (the same `0xc00046d4` then `0xc0004710` sequence used by `sub_c00a515c` and `sub_c00a56e0`), and calls `0xc00046e0` at 0xc00a4fc0 before returning 0.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a515c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Loop of about 688 bytes over a status word. It loads the word at 0xfa200e44 (0xc00a5174), stores it back to the same address (0xc00a51a0), and returns when the word is zero. When bits 0xe0400000 are set it prints a message through `0xc0010640` (string at 0xc010a894, call at 0xc00a51b4). When bits 0x03000000 are set it calls `sub_c00a540c` at 0xc00a51c4. When bits 0x0c000000 are set it runs a loop (0xc00a51d8 to 0xc00a53d8) over 8-byte descriptors that increments counters for descriptor status bits and, when bit 0x8000 of the descriptor halfword is set, prints the string at 0xc010a8a4 (call at 0xc00a527c). In that loop an atomic decrement (`lwarx`/`stwcx.` at 0xc00a52a8 and 0xc00a52b0) that reaches zero leads to the `0xc00046d4`/`0xc0004710` pair, a call of `0xc0013dec` with r3=0 and r4=1 (0xc00a52e0) and a call of `0xc00046e0` (0xc00a52e8). Under further conditions the same sequence repeats at 0xc00a5384 to 0xc00a53a8, and `0xc0013764` is called at 0xc00a53b0. When bit 0x00800000 is set it calls `sub_c00a563c` at 0xc00a53e8, then re-reads the status word (0xc00a53ec) and repeats while it is nonzero.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a563c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Loads a word from 0xc0160508 (0xc00a5650). When it is zero the routine prints a message through `0xc0010640` (string at 0xc010a910, call at 0xc00a567c) and returns. When it is nonzero it reads a word at +4 of the record that points to (0xc00a5684) and, if that is nonzero, calls it through `blrl` at 0xc00a5698 with r3 set to the word read from 0xfa200e80 (0xc00a5668); the facts do not resolve the target. The routine also stores 0 to 0xfa200e84 (0xc00a5664). Afterwards it moves the record from the word at 0xc0160508 to the word at 0xc0160504 (stores at 0xc00a56a8 to 0xc00a56b4) and, when a next record exists, copies its first word to 0xfa200e80 and stores 0x14 to 0xfa200e84.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a56e0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Runs the `0xc00046d4` then `0xc0004710` pair (calls at 0xc00a56fc and 0xc00a5700, result slot at sp+8), then reads the word at 0xc0160504 into r10 (0xc00a5708). When it is zero the routine returns 1. Otherwise it takes that record off the word at 0xc0160504, stores r4 at +4, r3 at +0 and 0 at +8 of it (0xc00a572c to 0xc00a5734), appends it to the chain headed at 0xc0160508 with its tail at 0xc016050c (or, when that chain is empty, starts it and stores r3 and 0x14 to 0xfa200e80 and 0xfa200e84), and returns 0. On both paths it calls `0xc00046e0` at 0xc00a577c with r3 loaded from sp+8; the r10=[0xc0160504] shown in the call facts is a leftover from the load, not an argument. It is called from `sub_c00a5eb0` at 0xc00a60c0 with r3=0x600a0000.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c00a5eb0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Same routine as the 6.5.21 entry at this address: it saves registers with `stmw`, runs bounded loops, performs an `lwarx`/`stwcx.` update at 0xc00a5fd4 and 0xc00a5fdc, and calls `sub_c00a56e0` at 0xc00a60c0 with r3=0x600a0000. It also calls `0xc00046a0`, `0xc00aa0a0` and `0xc0004c2c` (with r9 from 0xc0160518 at 0xc00a6094). The helper roles are inferred from the call sites.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c01344f0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Same code as the 6.5.21 entry at this address (identical instructions at identical addresses): a `stmw` frame with divide and shift arithmetic, a call to `0xc0003300` with the code 0xf at 0xc01345e0, and a test of its return at 0xc01345e8. The message call through `0xc0010640` at 0xc0134548 is unconditional; the one through `0xc0010094` at 0xc01345f4 runs only when the return is nonzero. The role is inferred from the same call sites.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c013d7e4` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** Same code as the 6.5.21 entry at this address (identical instructions at identical addresses): the same calls to `0xc000cbe4` and `0xc000c8d4` on blocks in the 0xc0198xxx area, the same tests of the `0xc006b73c` returns, and the same `0xc00a25cc`/`0xc00a2600` calls with 0x20 and 0x80. The role is inferred from the same call sites.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5fd4c` in `6.5.23/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5fd4c
  **Inferred.** Saves registers with `movem.l`, sets bit 6 of the byte at 0x200007f8 with `bset.b` at 0xffe5fd62 and clears it again with `bclr.b` at 0xffe5fe4e. Between them it reads the bytes at 0x200007f8 and 0x200007f9 to 0x200007ff (not 0x200007fc), splits each into high and low nibbles and combines them with multiplies by 10 (and by 1000 and 100 for the byte at 0x200007f8), which looks like decoding packed-decimal clock digits. It returns -1 (0xffe5fe7a) when all the decoded fields are zero or the first is outside 1930 to 2500. Otherwise it does a longer arithmetic step on the fields, calls `0xffe58a44` at 0xffe5ff32 with 0x48 as its only stack argument, adds the result, stores the sum through the pointer at `[sp+4]` when that is nonzero, and returns it. The clock reading is inferred from the arithmetic only.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffecfd4c` in `6.5.23/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffecfd4c
  **Inferred.** Copy of the routine at 0xffe5fd4c, offset by 0x70000 in the same image. It sets the same byte-state bit at 0x200007f8, reads the same status bytes, and calls `0xffec8a44` at 0xffecff32, which is the same helper at the offset address. The role is inferred from the matching body.
  Reviewed: 2026-11-12 (sonnet)

- `sub_fe004efc` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Same code as the 6.5.22 entry at this address (identical instructions at identical addresses): it calls `0xfe0051f0`, then `0xfe004610` with the same string-like constants, and calls `0xfe004ff4` with the flash base 0xfe040000. A nonzero return of `0xfe004ff4` skips (branch at 0xfe004f40) the calls of `0xfe004610` and `0xfe004f98` up to 0xfe004f74. It then stores 0xffffff80 to 0xfa200004, calls `0xfe004610` and `0xfe0046c8`, and branches to itself at 0xfe004f94. The role is inferred from the same call sites.
  Reviewed: 2026-11-12 (sonnet)
