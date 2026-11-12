# Routine readings, sysco tier-1 batch 6

Role statements for targets 150 to 174 of the tier-1 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). Every entry is inferred. The callees are
unnamed in every image here, so the helper roles below come from call shape, string-looking
arguments and constants, and none of them is confirmed.

- `sub_ffe7195c` in `6.5.19/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe7195c
  **Inferred.** It builds a frame and checks six fields of the record at [sp+4] against fixed limits
  (0x258 and 0x1e on the field at +0x14, then 0xc, 0x1f, 0x18, 0x3c and 0x3d on the fields at +0x10,
  +0xc, +0x8, +0x4 and +0x0), branching
  on each compare. It then calls 0xffe713b0 with six stack arguments and 0xffe715a0 twice with a buffer
  at sp-8. The limits resemble calendar and clock maxima, so this may validate a broken-down date or
  time, but nothing in the facts names the fields.

- `sub_fe004efc` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** It calls the helper at 0xfe004610 six times with constant addresses in the 0xe01xxx
  range (0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01428 again, 0xe01474), which look like message strings, so 0xfe004610
  is probably a printer. It calls 0xfe004ff4 with base 0xfe040000 at 0xfe004f38 and skips the block at
  0xfe004f78 when that call returns non-zero. The 0xfa200000 constant passed at 0xfe004f8c hints at a
  board register block.

- `sub_fe0051a0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** The rendered facts show no calls, no branches and no memory accesses. The only exit, at
  0xfe0051ec, returns r3 as it stood on entry, so this may be a pass-through helper. Nothing in the
  facts names it, and the reading is held at that.

- `sub_fe048ba0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** It has a backward branch at 0xfe048c18 to 0xfe048c10, taken while the Z flag from an
  andi. at 0xfe048c14 is clear, so it looks like a polling or scanning loop. The andi. has unmodelled
  effects, so the loop bound and the tested value are unknown. Its caller here is 0xfe048c54, which
  passes 0x700000 in r9. The return value at 0xfe048c50 is not derived.

- `sub_fe048c54` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** It tests the word at absolute 0x705cd8 near the top and branches on it at 0xfe048c8c.
  It calls 0xfe048ba0 with 0x700000 in r9 at 0xfe048c90, runs two counted loops closed by bdnz at
  0xfe048d44 and 0xfe048de4, and calls 0xfe044770 at 0xfe048e08 before returning. The shape suggests an
  initialisation or test pass over a memory region, inferred from loop structure and constants only.

- `sub_c000a5e0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** It opens a 0x20-byte frame with stwu and reads the word at absolute 0xfa200284. It
  reads and rewrites the machine state register with mfmsr and mtmsr at 0xc000a5f8 to 0xc000a608, and
  the effects of those two instructions are unmodelled. It calls 0xc0004710 at 0xc000a5ec and 0xc0010640
  with 0xc00f0164 at 0xc000a61c, which looks like a message. This may set a machine-state bit during
  early boot, but the facts do not settle it.

- `sub_c00a2374` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** It executes tlbia at 0xc00a23d0, which invalidates the TLB, so it looks like an MMU
  setup step. It calls 0xc000b454 at 0xc00a23b8 with a page-aligned address (r3+4096), the register
  block 0xfa200000 and the 0xc0160000 region. The callee is unnamed, so the argument meanings are guesses.

- `sub_c00a23ec` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** It calls 0xc0003300 at 0xc00a2430 with r3=5 and the address 0xc00a247c, then prints
  through 0xc0010094 with 0xc010a1b8 at 0xc00a2444 only when that call returns non-zero. It then calls
  0xc00a250c at 0xc00a2458 and returns its result. The registration-looking call and the print path
  are inferred from call order and the string-looking constant.

- `sub_c00a247c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** It applies bit-field shifts and masks (srwi, slwi, rotlw, slw, and) and then makes an
  indirect call through the link register at 0xc00a24c8, so its target comes from a function pointer
  that the facts do not resolve. A branch at 0xc00a24b8 skips that path when a value is zero. The
  instruction at 0xc00a2508 is not reached in this image, so part of the code is dead or was not
  followed by the renderer.

- `sub_c00a250c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** It saves a block of registers with stmw at 0xc00a2514, builds a value with slwi and slw,
  calls 0xc00a2594 at 0xc00a252c, and prints through 0xc0010640 with 0xc010a1d4 at 0xc00a2558 on one path.
  It restores the saved registers with lmw at 0xc00a2588 before returning. The print helper pattern
  matches 0xc00a4e98 and 0xc013d7e4, but the callee names are not in the facts.

- `sub_c00a2594` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** It has no calls or memory accesses in the facts. It shifts, rotates and masks a value
  with slwi, rotlw and and between 0xc00a25a4 and 0xc00a25c0, and the return at 0xc00a25c8 leaves r3
  unresolved. It is called by 0xc00a250c, so it is probably a bit-field helper for that routine.

- `sub_c00a4e98` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** It saves registers with stmw at 0xc00a4ea0 and prints through 0xc0010640 with 0xc010a7dc
  at 0xc00a4ecc. It calls 0xc00046d4 and then 0xc0004710 with that result at 0xc00a4f40 and 0xc00a4f44.
  Z-flag branches at 0xc00a4ec0, 0xc00a4f78 and 0xc00a4f98 choose among 0xc0004868, 0xc00046d4 and
  0xc00046e0, so it looks like a conditional setup or cleanup path. The meanings of the callees are not
  established.

- `sub_c00a515c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** It prints two messages through 0xc0010640 with 0xc010a894 at 0xc00a51b4 and 0xc010a8a4
  at 0xc00a527c. It calls the sub-helper at 0xc00a540c at 0xc00a51c4, and it repeats the pattern of
  0xc00046d4 then 0xc0004710 at 0xc00a52c4 and 0xc00a5384 with 0xc0013dec(0, 1) at 0xc00a52e0 and
  0xc00a53a0. The repetition suggests two similar setup steps, and a final call to 0xc0013764 at
  0xc00a53b0 ends it. Nothing in the facts names the items.

- `sub_c00a563c` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** It loads a pointer from the global at 0xc0160508, and when that is set it tests the words
  at +4 and +8 of the pointed-to record at 0xc00a568c and 0xc00a56b8. It prints through 0xc0010640 with
  0xc010a910 at 0xc00a567c and calls through a function pointer loaded from the 0xfa200e80 word at
  0xc00a5698. This looks like an optional hook dispatch, inferred from the null tests and the indirect
  call only.

- `sub_c00a56e0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** It tests the globals at 0xc0160504 and 0xc0160508 at 0xc00a5714 and 0xc00a5738. When they
  are set it calls 0xc00046d4 and then 0xc0004710 at 0xc00a56fc and 0xc00a5700, and it calls 0xc00046e0
  with the first global in r10 at 0xc00a577c. Read with 0xc00a563c, which uses the same 0xc0160508 word,
  this looks like a reset of a pair of pointers, inferred from shared globals and null tests only.

- `sub_c00a5eb0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** It is a loop-based routine with bdnz counters at 0xc00a5f58 and 0xc00a6018 and value
  tests at 0xc00a5fc0 to 0xc00a5ffc. It calls 0xc00046a0 at 0xc00a5ef0, 0xc00aa0a0 at 0xc00a5fec and
  0xc0004c2c at 0xc00a6094 with the global at 0xc0160518 and a count of 8. It ends by calling 0xc00a56e0
  at 0xc00a60c0 with 0x600a0000 and 0xc00a5a60. The overall shape suggests a table walk that sets up or
  tears down entries, but the loop count, the table base and the callee roles are not established.

- `sub_c01344f0` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** It prints through 0xc0010640 with 0xc00f011c and a count of 0x3c at 0xc0134548, and calls
  0xc0004630 at 0xc0134580. It calls 0xc0003300 at 0xc01345e0 with 0xf, the address 0xc000a584 and
  0xc00f013c, then prints through 0xc0010094 with 0xc00f0144 at 0xc01345f4 only when that call returns
  non-zero. The same registration-looking call appears in 0xc00a23ec with a different first argument,
  so this is probably another instance of one setup pattern.

- `sub_c013d7e4` in `6.5.19/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** It is the largest routine in the batch, 1892 bytes. It calls 0xc0013c38 at 0xc013d800 with
  3 and 0xc00a3000, then runs a long series of calls to 0xc000c8d4 that move or size blocks of 0x2c and
  0xc4 bytes between 0xc0198ab0 and 0xc0198c58, interleaved with prints through 0xc0010640 and 0xc0010094.
  It calls 0xc00a25cc and 0xc00a2600 with 0x20 and 0x80, then 0xc00a250c at 0xc013df04 and 0xc00a2630
  at 0xc013df1c. The 0xc0198xxx addresses suggest a table of structures, but
  the copy helper and the size meanings are unnamed.

- `sub_ffe58768` in `6.5.20/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe58768
  **Inferred.** It has the same record check as the 6.5.19 build at 0xffe7195c, with the same limits
  (0x258, 0x1e, 0xc, 0x1f, 0x18, 0x3c and 0x3d) on the same offsets. It calls 0xffe62d84 with six stack
  arguments at 0xffe587c4, and 0xffe62f74 twice with a buffer at sp-8 at 0xffe587d4 and 0xffe587f2. The
  helpers differ from the 6.5.19 build, so the same check is wired to different callees there. Inferred
  from the facts only.

- `sub_fe004efc` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** The rendered facts match the 6.5.19 build at the same address exactly, so the reading is
  the same. It calls 0xfe004610 six times with the 0xe01xxx constants, calls 0xfe004ff4 with 0xfe040000,
  and skips the block at 0xfe004f78 when that call returns non-zero. The helper roles are inferred, as
  in the 6.5.19 entry.

- `sub_fe0051a0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. There are no calls or
  branches, and the only exit at 0xfe0051ec returns r3 as it stood on entry, so this may be a
  pass-through helper.

- `sub_fe048ba0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. It has a backward branch
  at 0xfe048c18 to 0xfe048c10, taken while the Z flag from the andi. at 0xfe048c14 is clear, so it looks
  like a polling or scanning loop. The loop bound and the tested value are unknown.

- `sub_fe048c54` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. It tests the word at
  absolute 0x705cd8 and branches on it at 0xfe048c8c, calls 0xfe048ba0 with 0x700000 in r9 at 0xfe048c90,
  runs two counted loops closed by bdnz at 0xfe048d44 and 0xfe048de4, and calls 0xfe044770 at 0xfe048e08.
  The shape suggests an initialisation or test pass, inferred from loop structure and constants only.

- `sub_c000a5e0` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. It calls 0xc0004710 at
  0xc000a5ec and 0xc0010640 with 0xc00f0164 at 0xc000a61c, which looks like a message print after a
  setup call. The frame and the machine-state accesses are as in the 6.5.19 entry, and their meaning is
  not settled by the facts.

- `sub_c00a2374` in `6.5.20/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** The rendered facts match the 6.5.19 build at the same address. It runs tlbia at
  0xc00a23d0, which invalidates the TLB, so it looks like an MMU setup step, and it calls 0xc000b454 at
  0xc00a23b8 with a page-aligned address and the 0xfa200000 and 0xc0160000 constants. The callee is
  unnamed, so the argument meanings are guesses.
