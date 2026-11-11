# Tier-1 routine readings, sysco batch 14

Role statements for targets 350 to 374 of the tier-1 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). Every entry is
inferred: the rendered facts show what the code touches, not what it is for. Where a
constant or callee suggests a purpose, the entry says so and says it is a guess.

- `sub_ffe73e48` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xffe73e48
  **Inferred.** Reads six 32-bit fields at offsets +0 to +20 of the record that the stack argument points to, and compares each against fixed limits (0x258, 0x1e, 0xc, 0x1f, 0x18, 0x3c, 0x3d) at 0xffe73e62 to 0xffe73e96. It calls through the pointer at 0xffe75938 at 0xffe73ea4 with d3 to d7, calls the helper at 0xffe75b28 twice (0xffe73eb4, 0xffe73ed2), and adds to the fields at +0x10 to +0x18 of the object that helper returns (0xffe73ee2 to 0xffe73eea). The limits look like a date or time record check, but nothing in the rendered facts names the record, so that reading is a guess.

- `sub_fff63e48` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#3)` at 0xfff63e48
  **Inferred.** The same code shape as the routine at 0xffe73e48, at the same offsets in a second copy in this image: the same six-field limit checks, the same flag-byte work on 0x200007f8 to 0x200007fc, and the same stack frame. Its calls go through 0xfff65938 and 0xfff65b28, which are the copy's own pointer slots. Purpose is unknown for the same reason as its twin.

- `sub_ffe7aa08` in `6.5.30/usr/cpu/firmware/sysco/l1.bin(l1#5)` at 0xffe7aa08
  **Inferred.** A third copy of the same six-field checker, this one calling through 0xffe7e518 and 0xffe7e708 (the helper at 0xffe7e708 is called at 0xffe7aa74 and 0xffe7aa92). It reads and clears the same flag bytes at 0x200007f8 to 0x200007fc. The function is the same as the copy at 0xffe73e48 by its facts, and its intended use is not settled.

- `sub_fe004efc` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** A PowerPC prologue (stwu at 0xfe004efc) followed by a run of calls to 0xfe004610 with the constant addresses 0xe013ec, 0xe01418, 0xe01428, 0xe01454 and 0xe01474 in r3, plus calls to 0xfe0051f0, 0xfe004ff4 and 0xfe004f98 with r3=0xfe040000. The branch at 0xfe004f40 tests the result of 0xfe004ff4, and the routine stores to 0xfa200004 at 0xfe004f84 before its final calls. The constant-address calls look like a sequence of messages passed to 0xfe004610, which is a guess from call shape alone.

- `sub_fe0051a0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** A leaf routine that stores halfwords and words into the 0xfa200000 block at 0xfa200950 to 0xfa200966 (0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966), and at 0xfa200ab8, 0xfa200abc, 0xfa200ac2 and 0xfa200ac4, then returns its r3 argument unchanged. Routine 0xfe048ba0 touches the same words at 0xfa200ab8 to 0xfa200ac2, so the two are likely part of one setup. Which device the block belongs to is not stated in the facts.

- `sub_fe048ba0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Loads and stores back the words at 0xfa200abc and 0xfa200ab8, and the halfword at 0xfa200ac2, then writes halfword and byte fields at 0xfa203c80 to 0xfa203cac and 0xfa2009c0, and a word at 0xfa200030. It also stores a word to 0x705cd8 at 0xfe048c44, the flag that routine 0xfe048c54 reads. A byte-store loop at 0xfe048c10 runs before the return. The load-then-store on the same words suggests a read-modify-write, but the facts do not show what changes.

- `sub_fe048c54` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Calls 0xfe048ba0 with r9=0x700000 when the word at 0x705cd8 is nonzero (branch at 0xfe048c8c), which the routine at 0xfe048ba0 writes just before returning. It then reads halfwords at 0xfa203c80 and 0xfa203c82 in a counted loop (bdnz at 0xfe048d44) and stores to unresolved addresses through stb and sth. It finishes with a call to 0xfe044770 at 0xfe048e08. The loop and the flag at 0x705cd8 are the clearest structure; the purpose of the loop is not settled.

- `sub_c000a5e0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Reads a word from 0xfa200284 and a byte from an unresolved pointer, stores a byte back, and calls 0xc0004710 with no argument set and 0xc0010640 with r3=0xc00effd8. Routine 0xc0010640 takes a string-like constant in many routines in this image, so it may be a message print, but that is a guess.

- `sub_c00a2374` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Stores to 0xfa200030 and to the kernel words at 0xc015e4e8 to 0xc015e4f8, reads and writes a word through the pointer at 0xc000b454, and calls 0xc000b454 with r4=r3+4096, r5=0xfa200000, r6 to r10=0xc0160000 (r7=1). The argument pattern, a page-sized offset and a fixed base, looks like a mapping or region setup. The callee is not resolved in the facts, so that reading stays a guess.

- `sub_c00a23ec` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Stores words to 0xfa200940 and 0xfa200948 and reads them back at 0xc00a245c. It calls 0xc0003300 with r3=5 and r4=0xc00a247c (routine 0xc00a247c), and branches on the result at 0xc00a2438, then calls 0xc0010094 with a string-like r3 and 0xc00a250c with r4=0xc00a2508. Passing the address of the next routine as a function argument suggests a registration call, but 0xc0003300 is not resolved here.

- `sub_c00a247c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Stores a halfword to 0xfa200930, reads it back, and walks a table with lwzx and lwz loads. It makes an indirect call at 0xc00a24c8 (blrl, target unknown) and stores a word to 0xfa20094c. It is most likely the callback registered by the routine at 0xc00a23ec, given the 0xfa2009xx words both use, but that link is inferred.

- `sub_c00a250c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Saves r13 to r31 with stmw, calls 0xc00a2594 (routine 0xc00a2594) and 0xc0010640 with r3=0xc0108ebc, reads the word at 0xfa200948 that routine 0xc00a23ec writes, and restores the registers with lmw. The routine is a save-and-call wrapper around the next helper, and what the helper does is not in the facts.

- `sub_c00a2594` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** A short leaf that stores two words with indexed stores (stwx at 0xc00a25a8 and 0xc00a25ac), reads the word at 0xfa200948, and stores a word to an unresolved address before returning. Its role is not clear from the facts.

- `sub_c00a4e98` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Saves registers with stmw and runs a loop over halfword and word fields through unresolved pointers. It calls 0xc0010640 with r3=0xc0109480, 0xc0004868, and 0xc00046d4 followed by 0xc0004710 with its result, and then 0xc00046e0. It reads the kernel word at 0xc015e178 and stores to 0xfa200e54. The enter and exit pattern (0xc00046d4 then 0xc0004710, then 0xc00046e0) recurs in routines 0xc00a515c and 0xc00a56e0, so those three calls may be a lock or a bracket pair, but that is not shown.

- `sub_c00a515c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** The largest routine of this group (688 bytes). It reads the word at 0xfa200e44 and skips a block at 0xc00a53f8 when that word is zero (branch at 0xc00a5188), runs many loads and stores through unresolved pointers, and calls 0xc0010640 with the strings at 0xc0109538 and 0xc0109548, the routine at 0xc00a540c, and the routine at 0xc00a563c (routine 0xc00a563c). The 0xfa200e44 word is the one that routine 0xc00a5eb0 writes, so this may be the handler gated on that device state, but the rendered facts do not show it.

- `sub_c00a563c` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Works on a small pointer structure at the kernel words 0xc015e504 and 0xc015e508: it loads the words at +4 and +8 of the object at 0xc015e508, and stores to 0xc015e504 and 0xc015e508. It writes 0xfa200e80 and 0xfa200e84 and makes an indirect call through the pointer loaded from 0xfa200e80 (blrl at 0xc00a5698). The shape looks like a queue or callback dispatch, which is a guess.

- `sub_c00a56e0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Uses the same kernel words as routine 0xc00a563c (0xc015e504, 0xc015e508 and 0xc015e50c), with stores that rewrite the same fields, and writes 0xfa200e80 and 0xfa200e84 to match. It calls the 0xc00046d4 and 0xc0004710 pair and 0xc00046e0 with r10 set from 0xc015e504, the same bracket pattern as routine 0xc00a4e98. It reads as the teardown side of routine 0xc00a563c, but that pairing is inferred from the shared addresses alone.

- `sub_c00a5eb0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** A setup routine (568 bytes) that stores four words to 0xfa200e40, 0xfa200e44, 0xfa200e48 and 0xfa200e4c, reads 16-bit values from the kernel table at 0xc012cac8 and 0xc012caca, and runs byte and halfword loops with stbx, sth and lhzu. It calls 0xc00046a0, 0xc00aa0a0, 0xc0004c2c (with r9 from 0xc015e518 and r10=8), and 0xc00a56e0 (routine 0xc00a56e0), with r3=0x600a0000 and r4=0xc00a5a60. It writes the 0xfa200e44 word that routine 0xc00a515c reads. The field layout and the 0x600a0000 argument are not explained by the facts.

- `sub_c01324f0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01324f0
  **Inferred.** Stores words to 0xfa200380 and reads 0xfa200280, reads halfwords and stores halfwords into unresolved structures, and calls 0xc0010640 with r3=0xc00eff90 and r5=0x3c. It calls 0xc0004630, then 0xc0003300 with r3=0xf and r4=0xc000a584, and branches on the result. It ends with a call to 0xc0010094 with r3=0xc00effb8. The pattern matches routine 0xc00a23ec: a registration call through 0xc0003300 followed by a print, so the two may be parallel setup routines, which is a guess.

- `sub_c013b7e4` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013b7e4
  **Inferred.** The largest routine in the batch (1892 bytes). It stores a long run of words and halfwords into the kernel block at 0xc0196ab0 to 0xc0196b64, calls 0xc000cbe4 with r3=0xc0196ab0 and r4=0xc4 (a 196-byte region, probably cleared), calls 0xc000c8d4 with r3=0xc0196acc, r4=0xc017c40c and r5=0x2c (44 bytes), which looks like a copy, and calls 0xc002778c with r3=0x90 and r4=0xf0. Near the end it calls 0xc00a25cc and 0xc00a2600 with r3 set to 0x20 and 0x80, then 0xc00a250c (routine 0xc00a250c). It reads the kernel words at 0xc012f684, 0xc012f688 and 0xc015e500. This looks like a large initialization of a kernel table, but the table's use is not given by the facts.

- `sub_ffe486c4` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe486c4
  **Inferred.** A coldfire routine (572 bytes) that sets and clears bits in the flag byte at 0x200007f8 (bset.b at 0xffe486d2, bclr.b at 0xffe48802) and reads the status bytes at 0x200007f9 to 0x200007ff. It adds to a value in the table at 0x30000590 indexed by an unknown register, and calls 0xffe43b70 four times with pushed arguments (0xffe4889e, 0xffe488ae, 0xffe488be, 0xffe488d2). It stores a long to 0x3000fc18 at 0xffe488c6 and 0xffe488da. Its role is not settled; the bit flags suggest a status or control word for a device.

- `sub_ffe48a44` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe48a44
  **Inferred.** A coldfire routine (680 bytes) with the same flag-byte pattern as routine 0xffe486c4: bset.b at 0xffe48a62 and clr.b at 0xffe48ab8 on 0x200007f8 and 0x200007ff, and paired loads and stores on 0x200007fb, 0x200007fd and 0x200007fe. It moves four stack words into registers and runs eleven loads and seven stores through unresolved pointers. The facts show no direct calls, so the callee structure is not known.

- `sub_ffe48cec` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe48cec
  **Inferred.** A coldfire routine (480 bytes) with no direct calls in the rendered facts. It sets the flag at 0x200007f8, reads the status bytes at 0x200007f9 to 0x200007ff, and runs an add loop (add.l pairs at 0xffe48d30, 0xffe48d4c, 0xffe48d5c) with a sign test at 0xffe48e7e. It loads a value from the table at 0x30000590 and returns with d0=0x0. The return value suggests a status check, but the table and the loop bound are not named by the facts.

- `sub_ffe49498` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffe49498
  **Inferred.** A coldfire routine (336 bytes) that sets and clears the flag at 0x200007f8, reads the byte at 0x200007ff, and does a signed multiply (muls.l at 0xffe4953e). It calls the local routine at 0xffe495e8 twice (bsr.w at 0xffe49548 and 0xffe49580) and calls 0xffe43438 twice through jsr (0xffe495c4, 0xffe495d2), then returns with d0=0x0. The shape resembles a small status computation with a helper, but no field is named.

- `sub_ffeb86c4` in `6.5.9/usr/cpu/firmware/l1.bin(l1#0)` at 0xffeb86c4
  **Inferred.** The same coldfire shape as routine 0xffe486c4 at a second address in this image (572 bytes), with the same flag-byte operations on 0x200007f8 and the same table read at 0x30000590. It calls 0xffeb3b70 four times (0xffeb889e, 0xffeb88ae, 0xffeb88be, 0xffeb88d2), where routine 0xffe486c4 calls 0xffe43b70 at the same offsets, so the two copies differ in the callee address (0x80000 apart). Its role is as unsettled as its twin.
