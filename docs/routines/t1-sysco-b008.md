# Tier-1 routine readings, sysco batch 8

Role statements for targets 200 to 224 of the tier-1 list, written from the rendered facts only
(registers, calls, constants, branch targets and memory accesses). Each role is a reading of those
facts. All 25 entries are inferred, and none is marked established. The grader accepted all 25,
so none were rejected.

- `sub_c00a5eb0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Non-leaf routine that saves a register block with `stmw` at 0xc00a5eb8 and restores it with `lmw` at 0xc00a60dc. Its loops at 0xc00a5f48 and 0xc00a6010 use a `bdnz` count, and the `lwarx`/`stwcx.` pair at 0xc00a5fd4 and 0xc00a5fdc suggests an atomic update of a shared word. It calls `sub_c00a56e0` at 0xc00a60c0 with r3=0x600a0000, and the helpers at 0xc00046a0, 0xc00aa0a0 and 0xc0004c2c; the purpose is inferred from that shape, not from the callee names.

- `sub_c01344f0` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Saves registers with `stmw` at 0xc01344f8 and does a signed divide (`divw` at 0xc0134568 and 0xc0134578) on values it shifts first. It calls `0xc0003300` at 0xc01345e0 with the constant 0xf and the pointer 0xc000a584, branches on its return at 0xc01345e8, and emits messages through `0xc0010640` (string at 0xc00f011c) and `0xc0010094` (string at 0xc00f0144). The message path is inferred from the string arguments and the return test.

- `sub_c013d7e4` in `6.5.21/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** Large setup routine of about 1.9 KB that builds two records in the 0xc0198xxx area. It passes sizes of 0xc4 and 0x2c bytes to `0xc000c8d4` and clears a block with `0xc000cbe4` at 0xc013d82c. It checks the return of `0xc006b73c` at 0xc013d9f0 and 0xc013da0c, calls `0xc0013c38` and `0xc002778c`, and calls the helpers `0xc00a25cc` and `0xc00a2600` with 0x20 and 0x80 at 0xc013dc40 to 0xc013dd00, which looks like memory sizing. The order of the setup steps is taken from the call sites.

- `sub_ffe5a240` in `6.5.22/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a240
  **Inferred.** Saves registers with `movem.l` (0xffe5a248) and checks six fields of a record passed at `[sp+4]`. The fields at offsets +0x14, +0x10, +0xc, +0x8, +0x4 and +0x0 are compared with the constants 0x258, 0x1e, 0xc, 0x1f, 0x18, 0x3c and 0x3d, which look like calendar or clock limits, and a failed check branches to 0xffe5a290. On the passing path it calls `0xffe59c94` at 0xffe5a29c, and `0xffe59e84` at 0xffe5a2ac and 0xffe5a2ca, and it tests a flag at +0x20 of the callee's result. The field meanings are inferred from the constant limits only.

- `sub_ffe58ddc` in `6.5.22/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe58ddc
  **Inferred.** Same shape as the `l1#0` copy at 0xffe5a240: it saves registers, range-checks the same six record fields against the same constants (branches to 0xffe58e2c), and calls `0xffe58830` at 0xffe58e38 with the record fields as stack arguments. It calls `0xffe58a20` twice (0xffe58e48 and 0xffe58e66) and tests a flag at +0x20 of its result at 0xffe58e56. The role is inferred from the matching copy and the limit constants.

- `sub_fe004efc` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Flash-image routine that makes a sequence of calls to `0xfe004610` with different string-like constants (0xe013ec, 0xe01418, 0xe01428, 0xe01454, 0xe01474). It calls `0xfe004ff4` and `0xfe004f98` with the flash window base 0xfe040000, and branches on the return of `0xfe004ff4` at 0xfe004f40 to skip one step. It also writes to the board window at 0xfa200000 through `r9` before the last call, so it looks like a status or report sequence; the messages themselves are not in the facts.

- `sub_fe0051a0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0051a0
  **Inferred.** Leaf routine with no calls. It stores halfwords to the board window at 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966, and words to 0xfa200ab8, 0xfa200abc and 0xfa200ac4, then returns its first argument in r3 (the return at 0xfe0051ec passes r3 through). It looks like a block of board-register setup values; the facts do not name the registers.

- `sub_fe048ba0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048ba0
  **Inferred.** Leaf routine with no calls that reads and rewrites words and halfwords in the board window at 0xfa200ab8, 0xfa200abc and 0xfa200ac2, and writes bytes and halfwords to 0xfa203c80 through 0xfa203cac. A loop at 0xfe048c10 with a `bne` at 0xfe048c18 stores bytes through an unresolved pointer, and the routine ends with a store to 0xfa200030. Its caller `sub_fe048c54` passes r9=0x700000 at 0xfe048c90.

- `sub_fe048c54` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c54
  **Inferred.** Flash-region routine that tests the global at 0x705cd8 (`bne` at 0xfe048c8c) and, on that path, calls `sub_fe048ba0` at 0xfe048c90 with r9=0x700000. It walks a count with `bdnz` loops at 0xfe048d34 and 0xfe048dd4 that read `mfctr` and mask with `andi.`, and it calls `0xfe044770` at 0xfe048e08 on the way out. It stores a word to 0x705cd8 at the end of the setup path, which looks like a flag being cleared or set; the facts do not confirm which.

- `sub_c000a5e0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Short 68-byte routine with a 32-byte frame (`stwu` at 0xc000a5e0). It reads the board word at 0xfa200284, reads and writes the machine state register with `mfmsr` at 0xc000a5f8 and `mtmsr` at 0xc000a608 around an `rlwinm` bit change, and calls `0xc0004710` at 0xc000a5ec and `0xc0010640` with the string pointer 0xc00f0164 at 0xc000a61c. The MSR bit being changed is not named in the facts.

- `sub_c00a2374` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2374
  **Inferred.** Clears the TLB with `tlbia` at 0xc00a23d0, then calls `0xc000b454` at 0xc00a23b8 with r4=r3+4096, r5=0xfa200000 and r6 through r10 set to the 0xc0160000 state block. The 4096 step and the fixed window base suggest a page-table or mapping reset; the callee is not named in the facts.

- `sub_c00a23ec` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a23ec
  **Inferred.** Passes the address of `sub_c00a247c` (0xc00a247c) to the helper `0xc0003300` at 0xc00a2430 as r4 with the code 5, and branches on its return at 0xc00a2438. It prints a message through `0xc0010094` (string at 0xc010a1b8) and calls `sub_c00a250c` at 0xc00a2458, and returns that call's result. The same helper `0xc0003300` is also called from 0xc01345e0 with the code 0xf, so it looks like a shared reporting or dispatch call.

- `sub_c00a247c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a247c
  **Inferred.** Bit-manipulation routine: `srwi`, `slwi`, `rotlw`, `and` and `slw` at 0xc00a24a0 to 0xc00a24e8 build a shifted mask, and a `beq` at 0xc00a24b8 selects the path. It makes an indirect call through the link register with `blrl` at 0xc00a24c8, whose target the facts do not resolve, and it returns the result of `sub_c00a250c` at 0xc00a2504. The address is passed to `0xc0003300` by its caller `sub_c00a23ec`, which suggests it is used as a callback, but that is inferred from the call site only.

- `sub_c00a250c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a250c
  **Inferred.** Calls the 56-byte routine `sub_c00a2594` at 0xc00a252c, then prints a message through `0xc0010640` with the string at 0xc010a1d4 (call at 0xc00a2558) before returning. It is a small wrapper that pairs a status step with a message; the message text is not in the facts.

- `sub_c00a2594` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a2594
  **Inferred.** Leaf routine of 56 bytes with no calls. It reads the board word at 0xfa200948 (`lwz` at 0xc00a25b4), stores two words with indexed stores (`stwx` at 0xc00a25a8 and 0xc00a25ac) and one more word at 0xc00a25c4, then returns with r3 unknown. Its caller `sub_c00a250c` at 0xc00a252c is the only caller in the batch, so the role is inferred from that call alone.

- `sub_c00a4e98` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Prints a message through `0xc0010640` (string at 0xc010a7dc), then calls `0xc00046d4` at 0xc00a4f40 and passes its result to `0xc0004710` at 0xc00a4f44. The pair is the same `0xc00046d4` then `0xc0004710` sequence used by `sub_c00a515c` and `sub_c00a56e0`, and it calls `0xc00046e0` at 0xc00a4fc0 to finish.

- `sub_c00a515c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a515c
  **Inferred.** Two-phase routine of about 688 bytes. Each phase prints a message through `0xc0010640` (strings at 0xc010a894 and 0xc010a8a4), then runs the same `0xc00046d4`/`0xc0004710` pair and the `0xc0013dec`/`0xc00046e0` pair (calls at 0xc00a52e0 and 0xc00a53a0). The second phase repeats the first at 0xc00a5384 to 0xc00a53a8. It ends with a call to `0xc0013764` at 0xc00a53b0 and a call to `sub_c00a563c` at 0xc00a53e8.

- `sub_c00a563c` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a563c
  **Inferred.** Prints a message through `0xc0010640` (string at 0xc010a910) and calls through a function pointer. The pointer is read from the board window at 0xfa200e80 (`lwz` at 0xc00a567c and the `blrl` at 0xc00a5698 use r3 and r9 from that word), and the target is not resolved in the facts.

- `sub_c00a56e0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a56e0
  **Inferred.** Runs the `0xc00046d4` then `0xc0004710` pair (calls at 0xc00a56fc and 0xc00a5700), then calls `0xc00046e0` at 0xc00a577c with r10 loaded from 0xc0160504. It is called from `sub_c00a5eb0` at 0xc00a60c0 with r3=0x600a0000, so it is a shared helper in that routine's path.

- `sub_c00a5eb0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Same routine as the 6.5.21 entry at this address: it saves registers with `stmw`, runs bounded loops, performs an `lwarx`/`stwcx.` update at 0xc00a5fd4 and 0xc00a5fdc, and calls `sub_c00a56e0` at 0xc00a60c0 with r3=0x600a0000. It also calls `0xc00046a0`, `0xc00aa0a0` and `0xc0004c2c` (with r9 from 0xc0160518 at 0xc00a6094). The helper roles are inferred from the call sites.

- `sub_c01344f0` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01344f0
  **Inferred.** Same routine as the 6.5.21 entry at this address: a `stmw` frame with divide and shift arithmetic, a call to `0xc0003300` with the code 0xf at 0xc01345e0, a branch on its return at 0xc01345e8, and messages through `0xc0010640` and `0xc0010094`. The role is inferred from the same call sites.

- `sub_c013d7e4` in `6.5.22/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc013d7e4
  **Inferred.** Same routine as the 6.5.21 entry at this address: the same setup of two records with `0xc000c8d4` copies, the same status checks on `0xc006b73c`, and the same `0xc00a25cc`/`0xc00a2600` calls with 0x20 and 0x80. The role is inferred from the same call sites.

- `sub_ffe5fd4c` in `6.5.23/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffe5fd4c
  **Inferred.** Saves registers with `movem.l` and sets a bit in the byte at 0x200007f8 with `bset.b` at 0xffe5fd62. It reads several bytes at 0x200007f9 through 0x200007ff, branches on their values in a series of `bne.b` and `bls.b` tests, and calls `0xffe58a44` at 0xffe5ff32 near the end. The part after 0xffe5ff4e is not reached from the entry, so the routine's full job is inferred only from the bit-set and the single call.

- `sub_ffecfd4c` in `6.5.23/usr/cpu/firmware/sysco/l1.bin(l1#1)` at 0xffecfd4c
  **Inferred.** Copy of the routine at 0xffe5fd4c, offset by 0x70000 in the same image. It sets the same byte-state bit at 0x200007f8, reads the same status bytes, and calls `0xffec8a44` at 0xffecff32, which is the same helper at the offset address. The role is inferred from the matching body.

- `sub_fe004efc` in `6.5.23/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe004efc
  **Inferred.** Same routine as the 6.5.22 entry at this address: the sequence of `0xfe004610` calls with the same string-like constants, the calls to `0xfe004ff4` and `0xfe004f98` with the flash base 0xfe040000, and the branch on the return of `0xfe004ff4`. The role is inferred from the same call sites.
