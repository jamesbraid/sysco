# Tier-2 routine role statements, sysco round r5 batch 0

These statements describe what each routine does, read from its rendered instruction facts (registers,
calls, constants and branch targets). They say nothing about hardware behaviour. Where the facts do not
settle a callee or a stored value, the statement leaves it out.

- `sub_fe0053a0` in `6.5.10/usr/cpu/firmware/l2.bin(flash)` at 0xfe0053a0
  **Inferred.** Loads the pointer at abs 0xe01804 [@0xfe0053bc] and works on the structure it names. It stores halfwords into that structure at +0x0 and from +0x3340 to +0x33a0 [@0xfe0054c4] [@0xfe00551c] [@0xfe005520], and the loop at 0xfe005524 reloads +0x0 for as long as the branch at 0xfe00552c is taken [@0xfe005524] [@0xfe00552c taken].
  Role (from facts): stores words and halfwords into the structure and halfwords at abs 0xfa200950, 0xfa200952 and 0xfa200954 [@0xfe0053f8] [@0xfe0053e4] [@0xfe0053dc] [@0xfe0053e8]. It returns the pointer at 0xe01804 plus 7752 [@0xfe0055b0].
  Reviewed: 2026-11-12

- `sub_c00c4a5c` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c4a5c
  **Inferred.** Reads the pointer at +0x5c of r3 [@0xc00c4a70]. The record behind that pointer is updated in a loop that the branch at 0xc00c4c48 repeats when taken [@0xc00c4c48 taken]. The word at +0x64 is stored back [@0xc00c4abc] only where the branch at 0xc00c4ab0 falls through [@0xc00c4ab0 fall], and the word at +0x54 is stored [@0xc00c4b88] unless the branch at 0xc00c4b6c is taken first [@0xc00c4b6c taken].
  Role (from facts): updates fields of the record at +0x5c of r3 and returns 0x0 [@0xc00c4c64].
  Reviewed: 2026-11-12

- `sub_c00c5380` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c5380
  **Inferred.** Reads halfwords at r3 +0x50 [@0xc00c5390] [@0xc00c53d0] and the word at r3 +0x0 [@0xc00c53a4]. The call to 0xc0010f08 at 0xc00c53b0 [call 1] sits behind the branch at 0xc00c53a0, which skips it when taken [@0xc00c53a0 fall].
  Role (from facts): stores two words to abs 0xfa200e08 and 0xfa200e0c [@0xc00c53e0] [@0xc00c53e4] on the path where the branch at 0xc00c53d8 falls through [@0xc00c53d8 fall].
  Reviewed: 2026-11-12

- `sub_fe0053a0` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0053a0
  **Inferred.** Loads the pointer at abs 0xe01804 [@0xfe0053bc] and works on the structure it names. It stores halfwords into that structure at +0x0 and from +0x3340 to +0x33a0 [@0xfe0054c4] [@0xfe00551c] [@0xfe005520], and the loop at 0xfe005524 reloads +0x0 for as long as the branch at 0xfe00552c is taken [@0xfe005524] [@0xfe00552c taken].
  Role (from facts): stores words and halfwords into the structure and halfwords at abs 0xfa200950, 0xfa200952 and 0xfa200954 [@0xfe0053f8] [@0xfe0053e4] [@0xfe0053dc] [@0xfe0053e8]. It returns the pointer at 0xe01804 plus 7752 [@0xfe0055b0].
  Reviewed: 2026-11-12

- `sub_fe048c20` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048c20
  **Inferred.** Reads the word at abs 0xc05bf8 [@0xfe048c40]. On the path where that word is zero the branch at 0xfe048c58 falls through to a call of 0xfe048b74 [call 1, @0xfe048c58 fall]. It writes bytes at abs 0xfa200860, 0xfa200870 and 0xfa20086c [@0xfe048cbc] [@0xfe048cc4] [@0xfe048ccc] and stores a halfword through the indexed address at 0xfa2009c0 [@0xfe048cac].
  Role (from facts): stores bytes at 0xfa200860, 0xfa20086c and 0xfa200870 [@0xfe048cbc] [@0xfe048ccc] [@0xfe048cc4] and halfwords through the index at 0xfa2009c0 [@0xfe048cac], then calls 0xfe044770 with r6 = 0xfa200860 [call 2].
  Reviewed: 2026-11-12

- `sub_fe048dfc` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048dfc
  **Inferred.** It writes the halfwords at abs 0xfa200950, 0xfa200952 and 0xfa200954 [@0xfe048e0c] [@0xfe048e04] [@0xfe048e10], and the words at 0xfa200ab8, 0xfa200abc and 0xfa200ac4 [@0xfe048e20] [@0xfe048e18] [@0xfe048e2c].
  Role (from facts): stores a fixed group of halfwords, including those at abs 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966 [@0xfe048e38] [@0xfe048e30] [@0xfe048e3c] [@0xfe048e44], and returns r3 unchanged [@0xfe048e48].
  Reviewed: 2026-11-12

- `sub_c00c4de0` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4de0
  **Inferred.** Reads the pointer at +0x5c of r3 [@0xc00c4df4]. The record behind that pointer is updated in a loop that the branch at 0xc00c4fcc repeats when taken [@0xc00c4fcc taken]. The word at +0x64 is stored back [@0xc00c4e40] only where the branch at 0xc00c4e34 falls through [@0xc00c4e34 fall], and the word at +0x54 is stored [@0xc00c4f0c] unless the branch at 0xc00c4ef0 is taken first [@0xc00c4ef0 taken].
  Role (from facts): updates fields of the record at +0x5c of r3 and returns 0x0 [@0xc00c4fe8].
  Reviewed: 2026-11-12

- `sub_c00efdc8` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00efdc8
  **Inferred.** Stores words to abs 0xfa200300, 0xfa200320 and 0xfa20030c [@0xc00efde4] [@0xc00efde8] [@0xc00efdf0]. It reads the halfword at abs 0xfa200220 [@0xc00efe04] and writes the halfword at 0xfa200220 [@0xc00efe10]. The call to 0xc0003db8 at 0xc00efe34 [call 1] is followed by a branch at 0xc00efe3c that is taken when the result is zero [@0xc00efe3c taken zero].
  Role (from facts): calls 0xc0003db8 [call 1], then calls 0xc0010870 [call 2, @0xc00efe3c fall] only when that result is nonzero, and returns the word at abs 0xfa200224 [@0xc00efe60].
  Reviewed: 2026-11-12

- `sub_ffe5d5c4` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d5c4
  **Inferred.** It writes the byte at abs 0x200007f8 [@0xffe5d5d2] [@0xffe5d702], and reads the bytes at 0x200007f9 to 0x200007ff [@0xffe5d6da] [@0xffe5d60e] [@0xffe5d636].
  Role (from facts): calls 0xffe58ee8 with stack argument 0x4a [call 1]. It makes the call with 0x4c [call 2, @0xffe5d7a8 fall] only where the branch at 0xffe5d7a8 falls through, and the call with 0x49 [call 3, @0xffe5d7b8 fall] only where the branch at 0xffe5d7b8 also falls through. It calls it with 0x48 [call 4, @0xffe5d7b8 taken] when the branch at 0xffe5d7b8 is taken, or when the branch at 0xffe5d7a8 is taken [@0xffe5d7a8 taken].
  Reviewed: 2026-11-12

- `sub_ffecd5c4` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecd5c4
  **Inferred.** It writes the byte at abs 0x200007f8 [@0xffecd5d2] [@0xffecd702], and reads the bytes at 0x200007f9 to 0x200007ff [@0xffecd6da] [@0xffecd60e] [@0xffecd636].
  Role (from facts): calls 0xffec8ee8 with stack argument 0x4a [call 1]. It makes the call with 0x4c [call 2, @0xffecd7a8 fall] only where the branch at 0xffecd7a8 falls through, and the call with 0x49 [call 3, @0xffecd7b8 fall] only where the branch at 0xffecd7b8 also falls through. It calls it with 0x48 [call 4, @0xffecd7b8 taken] when the branch at 0xffecd7b8 is taken, or when the branch at 0xffecd7a8 is taken [@0xffecd7a8 taken].
  Reviewed: 2026-11-12

- `sub_fe00518c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe00518c
  **Inferred.** It writes the halfwords at abs 0xfa200950, 0xfa200952 and 0xfa200954 [@0xfe00519c] [@0xfe005194] [@0xfe0051a0], and the words at 0xfa200ab8, 0xfa200abc and 0xfa200ac4 [@0xfe0051b0] [@0xfe0051a8] [@0xfe0051bc].
  Role (from facts): stores a fixed group of halfwords, including those at abs 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966 [@0xfe0051c8] [@0xfe0051c0] [@0xfe0051cc] [@0xfe0051d4], and returns r3 unchanged [@0xfe0051d8].
  Reviewed: 2026-11-12

- `sub_fe005388` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe005388
  **Inferred.** Loads the pointer at abs 0xe0185c [@0xfe0053a4] and works on the structure it names. It stores halfwords into that structure at +0x3340 and at +0x0 [@0xfe0054ac] [@0xfe005508], and the loop at 0xfe00550c reloads +0x0 for as long as the branch at 0xfe005514 is taken [@0xfe00550c] [@0xfe005514 taken].
  Role (from facts): stores halfwords at abs 0xfa200950, 0xfa200952 and 0xfa200954 [@0xfe0053cc] [@0xfe0053c4] [@0xfe0053d0], and returns the pointer at 0xe0185c plus 7752 [@0xfe005590].
  Reviewed: 2026-11-12

- `sub_fe048b1c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048b1c
  **Inferred.** Writes the words at abs 0xfa200abc and 0xfa200ab8 and the halfword at 0xfa200ac2 [@0xfe048b30] [@0xfe048b40] [@0xfe048b50], and writes halfwords and bytes at abs 0xfa203c80, 0xfa203c82, 0xfa203c84 and 0xfa203cac [@0xfe048b54] [@0xfe048b5c] [@0xfe048b68] [@0xfe048b70]. The loop at 0xfe048b8c re-reads the halfword at abs 0xfa2009c0 for as long as the branch at 0xfe048b94 is taken [@0xfe048b8c] [@0xfe048b94 taken].
  Role (from facts): fills the byte table at abs 0xfa200860 to 0xfa200874 [@0xfe048b98] [@0xfe048bb4] and stores the word at abs 0x705ce0 [@0xfe048bc0], then returns r3 unchanged [@0xfe048bcc].
  Reviewed: 2026-11-12

- `sub_fe048bd0` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048bd0
  **Inferred.** Reads the word at abs 0x705ce0 [@0xfe048bf0]. When that word is zero the branch at 0xfe048c08 falls through to a call of 0xfe048b1c [call 1, @0xfe048c08 fall]. It loads halfwords from abs 0xfa203c82 and 0xfa203c80 [@0xfe048c18] [@0xfe048c34] and stores bytes at 0xfa200860, 0xfa200870 and 0xfa20086c [@0xfe048c6c] [@0xfe048c74] [@0xfe048c7c].
  Role (from facts): after those loads and stores, calls 0xfe04475c with r6 = 0xfa200860 [call 2].
  Reviewed: 2026-11-12

- `sub_fe048dac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe048dac
  **Inferred.** It writes the halfwords at abs 0xfa200950, 0xfa200952 and 0xfa200954 [@0xfe048dbc] [@0xfe048db4] [@0xfe048dc0], and the words at 0xfa200ab8, 0xfa200abc and 0xfa200ac4 [@0xfe048dd0] [@0xfe048dc8] [@0xfe048ddc].
  Role (from facts): stores a fixed group of halfwords, including those at abs 0xfa200960, 0xfa200962, 0xfa200964 and 0xfa200966 [@0xfe048de8] [@0xfe048de0] [@0xfe048dec] [@0xfe048df4], and returns r3 unchanged [@0xfe048df8].
  Reviewed: 2026-11-12

- `sub_c00a5394` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5394
  **Inferred.** Loads the word at abs 0xfa200e40 [@0xc00a53ac]. The call to 0xc00a5538 at 0xc00a53c4 [call 1] runs only on the fall-through side of the branch at 0xc00a53bc [@0xc00a53bc fall]. It loads the word at +0xb0 of the structure at +0x60 of its argument [@0xc00a53c8].
  Role (from facts): when that pointer is nonzero [@0xc00a53dc fall] it calls 0xc00a4c6c [call 2] [call 3] [call 4], calls 0xc000cfc0 inside a loop that the branch at 0xc00a5420 repeats [call 5, @0xc00a5420 taken], calls 0xc00a4c6c again [call 6], and calls 0xc00a4ea0 [call 7].
  Reviewed: 2026-11-12

- `sub_c00a54bc` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a54bc
  **Inferred.** Reads the halfword at r3 +0x54 [@0xc00a54d0] and the word at abs 0xfa200f44 [@0xc00a54f0], and stores the word back to 0xfa200f44 [@0xc00a54f8] [@0xc00a5508]. The call to 0xc00102c8 at 0xc00a54ec [call 1] is reached only on the fall-through side of the branch at 0xc00a54e0 [@0xc00a54e0 fall].
  Role (from facts): stores the words at abs 0xfa200e08 and 0xfa200e0c [@0xc00a551c] [@0xc00a5520] on the fall-through side of the branch at 0xc00a5514 [@0xc00a5514 fall]. The facts do not show its return value [@0xc00a5534].
  Reviewed: 2026-11-12

- `sub_c0139dc8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc0139dc8
  **Inferred.** Stores the word at abs 0xfa200e40 [@0xc0139e20] and reads it back [@0xc0139e24]. It calls 0xc0083ca8 with r3 = 0 and r4 = 0 [call 3, @0xc0139df8 fall], and the structure it returns is filled: words are stored at its +0x88, +0xe8 and +0xf0 [@0xc0139fa0] [@0xc0139fd0] [@0xc0139fec].
  Role (from facts): sets up the structure that 0xc0083ca8 returns, and stores the word at abs 0xc015b4e8 [@0xc013a028]. It calls 0xc00a5538 [call 15] only on the fall-through side of the branch at 0xc013a0c0 [@0xc013a0c0 fall].
  Reviewed: 2026-11-12

- `sub_fe044b94` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe044b94
  **Inferred.** Reads the word at abs 0x705cfc [@0xfe044be8] and stores words to abs 0x705d00, 0x705d04 and 0x705d28 to 0x705d34 [@0xfe044c70] [@0xfe044c74] [@0xfe044c54] [@0xfe044c58]. The call to 0xfe0447cc at 0xfe044bf4 [call 1] runs only on the fall-through side of the branch at 0xfe044be0 [@0xfe044be0 fall].
  Role (from facts): stores those words and returns the word at abs 0x705cfc [@0xfe044f54]. It calls 0xfe044770 with a constant first argument [call 3] [call 10] [call 11] [call 13] [call 14], and once more inside the loop that the branch at 0xfe044eb4 repeats when taken [call 5, @0xfe044eb4 taken].
  Reviewed: 2026-11-12

- `sub_fe0490cc` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0490cc
  **Inferred.** Writes a byte at abs 0xfa100003 [@0xfe0490f8] and stores five words into the record at r3 +0x0 to +0x18 [@0xfe049120] [@0xfe049110]. It re-reads the halfword at abs 0xfa2009c0 [@0xfe04913c] for as long as the branch at 0xfe049144 is taken [@0xfe049144 taken].
  Role (from facts): calls 0xfe048e30 with r9 = 0xfa100003 [call 1] and calls 0xfe048e80 twice, with r4 = 0xc0000000 and then 0xc0001000 [call 10] [call 11], and returns the result of 0xfe048e80 [@0xfe04921c].
  Reviewed: 2026-11-12

- `sub_c00a5e34` in `6.5.14/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5e34
  **Inferred.** Reads halfwords at r3 +0x54 [@0xc00a5e48] [@0xc00a5e84] and the word at abs 0xfa200f44 [@0xc00a5e68], and stores the word to 0xfa200f44 [@0xc00a5e70] [@0xc00a5e80]. The call to 0xc0010640 at 0xc00a5e64 [call 1] is reached only on the fall-through side of the branch at 0xc00a5e58 [@0xc00a5e58 fall].
  Role (from facts): stores the words at abs 0xfa200e08 and 0xfa200e0c [@0xc00a5e94] [@0xc00a5e98] only where the branch at 0xc00a5e8c falls through [@0xc00a5e8c fall].
  Reviewed: 2026-11-12

- `sub_fe044b94` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe044b94
  **Inferred.** Reads the word at abs 0x705c84 [@0xfe044be8] and stores words into a block at abs 0x705cb0 to 0x705cbc [@0xfe044c4c] [@0xfe044c58]. The call to 0xfe0447cc at 0xfe044bf4 [call 1] runs only on the fall-through side of the branch at 0xfe044be0 [@0xfe044be0 fall].
  Role (from facts): stores that block of words at abs 0x705cb0 to 0x705cbc and returns the word at abs 0x705c84 [@0xfe044f54]. It calls 0xfe044770 with a constant first argument [call 3] [call 10] [call 11] [call 13] [call 14], and once more inside the loop that the branch at 0xfe044eb4 repeats when taken [call 5, @0xfe044eb4 taken].
  Reviewed: 2026-11-12

- `sub_fe0490cc` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe0490cc
  **Inferred.** Writes a byte at abs 0xfa100003 [@0xfe0490f8] and stores five words into the record at r3 +0x0 to +0x18 [@0xfe049120] [@0xfe049110]. It re-reads the halfword at abs 0xfa2009c0 [@0xfe04913c] for as long as the branch at 0xfe049144 is taken [@0xfe049144 taken].
  Role (from facts): calls 0xfe048e30 with r9 = 0xfa100003 [call 1] and calls 0xfe048e80 twice, with r4 = 0xc0000000 and then 0xc0001000 [call 10] [call 11], and returns the result of 0xfe048e80 [@0xfe04921c].
  Reviewed: 2026-11-12

- `sub_c00a5e34` in `6.5.15/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5e34
  **Inferred.** Reads halfwords at r3 +0x54 [@0xc00a5e48] [@0xc00a5e84] and the word at abs 0xfa200f44 [@0xc00a5e68], and stores the word to 0xfa200f44 [@0xc00a5e70] [@0xc00a5e80]. The call to 0xc0010640 at 0xc00a5e64 [call 1] is reached only on the fall-through side of the branch at 0xc00a5e58 [@0xc00a5e58 fall].
  Role (from facts): stores the words at abs 0xfa200e08 and 0xfa200e0c [@0xc00a5e94] [@0xc00a5e98] only where the branch at 0xc00a5e8c falls through [@0xc00a5e8c fall].
  Reviewed: 2026-11-12

- `sub_fe044b94` in `6.5.16/usr/cpu/firmware/sysco/l2.bin(flash)` at 0xfe044b94
  **Inferred.** Reads the word at abs 0x705c84 [@0xfe044be8] and stores words into a block at abs 0x705cb0 to 0x705cbc [@0xfe044c4c] [@0xfe044c58]. The call to 0xfe0447cc at 0xfe044bf4 [call 1] runs only on the fall-through side of the branch at 0xfe044be0 [@0xfe044be0 fall].
  Role (from facts): stores that block of words at abs 0x705cb0 to 0x705cbc and returns the word at abs 0x705c84 [@0xfe044f54]. It calls 0xfe044770 with a constant first argument [call 3] [call 10] [call 11] [call 13] [call 14], and once more inside the loop that the branch at 0xfe044eb4 repeats when taken [call 5, @0xfe044eb4 taken].
  Reviewed: 2026-11-12
