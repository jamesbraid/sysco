# L1 CPU features beyond plain ColdFire

Which MCF5206e instructions and control features the five ColdFire packages of the 1.44.0 L1 container use, with the address of each use. The question this answers: does software that runs this firmware need a core that is specifically an MCF5206e, or is a plain ColdFire integer core enough?

Short answer, with grades in the sections below:

- The integer code is the base ColdFire instruction set (ISA_A) plus the 32-bit hardware multiply and divide. **Established** for every instruction reached by control flow; **inferred** for the unreached remainder (see [Coverage](#coverage-and-method)).
- The MAC unit is never used, in any package. No line-A word occurs at an instruction boundary in reached code. The core has no use for MAC state (`ACC`, `MACSR`, `MASK`). There is no EMAC in the manual for this part either.
- What is specific to the part is not an instruction: it is four `MOVEC` control-register codes (`CACR`, `VBR`, `RAMBAR`, `MBAR`), written once each at start-up, plus the ColdFire exception frame, the single supervisor stack and the `HALT` instruction.
- No instruction outside the base set was found: no `TRAP`, `STOP`, `CPUSHL`, `WDDATA`, `WDEBUG`, `PULSE`, `TAS`, `SATS`, `MOV3Q`, `MVS`, `MVZ`, `BYTEREV`, `FF1`, `LINK.L` or long-displacement branch at an instruction boundary in reached code.

Source: IRIX 6.5.30 `l1.bin` (container SHA-256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`, L1 1.44.0). Package numbers are container ranges 0 to 6 as in [package-container.md](package-container.md). Addresses are load addresses (`0xffe00000` base). Every package holds two code banks; bank 2 sits `0x70000` (1 MB packages) or `0xf0000` (2 MB packages) above bank 1. Addresses below are in the first bank or the shared first-stage loader; add the bank offset for the copy. The manual is the MCF5206e User's Manual, MCF5206EUM/D rev. 1, cited by printed page, with the rev. 3 errata where noted.

## Packages scanned

| Range | Label | Body bytes | Loader candidates (entry) | Reached instructions, bank 1 plus loader | Code bytes in that region |
|---|---|---:|---|---:|---|
| 0 | Base 1MB | 982,376 | `0xffe10000` (`0xffe10b02`), `0xffe80000` (`0xffe80b02`) | 93,398 | 288,752 of 524,288 (55.1%) |
| 1 | Fuel/PE/O300 1MB | 982,368 | `0xffe10000` (`0xffe10b02`), `0xffe80000` (`0xffe80b02`) | 94,355 | 291,828 of 524,288 (55.7%) |
| 2 | MIPS 2MB | 1,755,520 | `0xffe10000` (`0xffe10b02`), `0xfff00000` (`0xfff00b02`) | 118,431 | 366,488 of 1,048,576 (35.0%) |
| 3 | Legacy 2MB | 1,772,576 | `0xffe10000` (`0xffe10a92`), `0xfff00000` (`0xfff00a92`) | 128,073 | 395,634 of 1,048,576 (37.7%) |
| 5 | 2MB | 1,778,732 | `0xffe10000` (`0xffe10ab2`), `0xfff00000` (`0xfff00ab2`) | 135,262 | 419,178 of 1,048,576 (40.0%) |

Ranges 4 and 6 are the two ARM (S3C2410) Linux packages; the MCF5206e never executes them, so they have no ColdFire features to list ([package-container.md](package-container.md)). Entry addresses are the third word of each candidate record (established, read at each candidate).

The first-stage code (`0xffe00400` to `0xffe00b3f`) is byte-identical in packages 0 and 1; in packages 2, 3 and 5 it differs from package 1 in exactly three 32-bit constants (`0xffe008c4`, `0xffe009b0`, `0xffe00ae4`: the upper search bound `0xfff00000` becomes `0xfffffff8`). Every opcode word at every reached bank-1 instruction is identical at the same offset in bank 2 (92,871, 93,828, 117,904, 127,546 and 134,735 instructions compared in packages 0, 1, 2, 3 and 5; zero differences). Established.

## Control-register use (`MOVEC`)

`MOVEC Rn,Rc` is opcode `0x4e7b` with the control-register code in the low 12 bits of the extension word. Twelve `MOVEC` instructions are reached in each package, all register-to-control writes; no `MOVEC` read (`0x4e7a`) exists at an instruction boundary. The manual describes `CACR`, `RAMBAR` and `MBAR` as write-only supervisor registers (pp. 4-6, 5-2, 8-7), which agrees. Established for all five packages.

| Rc | Register | Value written | First-stage loader | Program entry, bank 1 | Grade |
|---|---|---|---|---|---|
| `0xc04` | `RAMBAR` | `0x30100001` (SRAM base `0x30100000`, valid, no address-space mask bits, not write-protected) | `0xffe0040c` (`move.l #0x30100000,d0; ori.l #1,d0`) | `0xffe10b0e` (packages 0, 1, 2), `0xffe10a9e` (3), `0xffe10abe` (5) | established |
| `0xc0f` | `MBAR` | `0x10000001` (peripheral block at `0x10000000`, valid, all address spaces enabled) | none | `0xffe10b2e` (0, 1, 2), `0xffe10abe` (3), `0xffe10ade` (5) | established |
| `0x801` | `VBR` | `0x30100000` | `0xffe0089c` (from `A3`) | `0xffe10b50` (0, 1, 2), `0xffe10ae0` (3), `0xffe10b00` (5) | established |
| `0x002` | `CACR` | `0x01000000` (CINV), then `0x80000000` (CENB) | none | `0xffe10b88` and `0xffe10b92` (0, 1, 2), `0xffe10b18` and `0xffe10b22` (3), `0xffe10b38` and `0xffe10b42` (5) | established |
| `0x004`, `0x005` | `ACR0`, `ACR1` | never written | none | none | established as absence |

Bank 2 repeats the program-entry rows at the bank offset. The immediate values come from the `move.l #imm,dN` (and `ori.l #1,dN`) just before each `MOVEC`; established by reading those instructions in every package. The Grade column restates that reading. The package 1 rows were read again against the committed listing ([l1/disasm/mcf5206e-1.44.0-fuel-pe-o300](disasm/mcf5206e-1.44.0-fuel-pe-o300/README.md)): the loader's `RAMBAR` and `VBR` writes at `0xffe0040c` and `0xffe0089c`, and the program-entry writes at `0xffe10b0e`, `0xffe10b2e`, `0xffe10b50`, `0xffe10b88` and `0xffe10b92`, carry the immediates in the table.

Notes on the values:

- `RAMBAR` and `MBAR` both carry only the valid bit in their low half. With the valid bit set and the mask bits clear, the SRAM block and the peripheral block answer in supervisor and user, code and data space (manual pp. 5-2, 8-7). `MBAR` base bits `[31:14]` are the valid address bits per the rev. 3 errata (item for 8.3.2.1); `0x10000000` satisfies that.
- Two `CACR` writes: invalidate first, then enable (manual p. 4-6: the cache must be invalidated after reset before it is enabled). Every other `CACR` field is zero: default cacheable, no buffered writes, no default write protection, `CLNF` zero, noncacheable bursts off. The cache is an instruction cache only (manual p. 4-1: 4 KB, direct-mapped), so data accesses, including those to the peripheral block, are not affected.
- `ACR0` and `ACR1` stay at their reset value (zero) in every package: no region is marked noncacheable. Established as absence: every reached `MOVEC` extension word is one of the four codes above.
- The SRAM at `0x30100000` holds the vector table, the first stack and RAM variables ([board-boot-contract.md](board-boot-contract.md)). The manual gives 8 KB (p. 5-1); the image uses `0x30100000` to `0x30101fff` at most (stack top `0x30101800`, values near `0x30101990`).

## Vector base and exception frame

- **Established.** The first-stage loader copies `0x400` bytes from the word after a candidate's first word (the candidate's `SP`, reset PC and 254 handler words) to `0x30100000` (`0xffe0085c`, loop `0xffe00894` to `0xffe0089a`), loads `A3 = 0x30100000`, executes `MOVEC A3,VBR` (`0xffe0089c`) and calls the candidate entry (`jsr (a4)` at `0xffe008a0`). The candidate then sets `VBR` again from `move.l #0x30100000,d0`. The `VBR` low 20 bits read as zero, so `0x30100000` is a legal 1 MB-aligned base (manual p. 3-5).
- **Established.** In the candidate's copy of the table, every handler word from vector 2 to 255 except vector 29 points at a four-byte `bsr.w` stub; every stub branches to one common routine (`0xffe10fc8` in packages 0, 1 and 2, `0xffe10f58` in package 3, `0xffe10f78` in package 5; in package 1 the vector number is recovered from the pushed return address, `sub.l #0xffe10bd0`, `asr.l #2`, `addq.l #1`). Vector 29 (level 5 autovector) points at a real handler: `0xffe10ff4` in packages 0, 1, 2, `0xffe10f84` in package 3, `0xffe10fa4` in package 5. It stores `0xff` to `0x10000111` (the timer 1 event register, manual p. 14-3 register map) and jumps to `0xffe4032e` (package 1), which [board-boot-contract.md](board-boot-contract.md) describes as the level-5 timer tick path. The loader's own table in the packaged vector region (`0xffe0042c` on) routes every vector to `0xffe00824`, which pushes the vector number and executes `HALT` (`0xffe0083e`).
- **Established.** The kernel builds start-up frames for new tasks by hand in the ColdFire layout: program counter longword, then the status word `0x2000`, then the format/vector word `0x4000` (format 4, vector 0) (`0xffe41228`, `0xffe41230`, `0xffe41238` in package 1). The ColdFire frame is the format/vector word and `SR` in the first longword and `PC` in the second (manual p. 3-7, figure 3-5, table 3-2). A 68000-style frame (`SR` then `PC`) would break this. That the frames are consumed by `RTE` is **inferred** from the register-restore and `RTE` sequences below. Tasks start with `SR = 0x2000`: supervisor, interrupt priority zero. A second task-setup site stores `0x2000` into a control-block field (`0xffe54a4e`, package 1); that this is the same saved `SR` is **inferred** from the field being written next to the stack pointer and entry address.
- **Established.** `RTE` is reached at four sites per bank in every package (package 1: `0xffe413c0`, `0xffe41696`, `0xffe416be`, `0xffe41872`), each after restoring the saved register block or dropping stack words (package 1 `0xffe413ba` to `0xffe413c0`: `lea 0x3c(a7),a7; addq.l #4,a7; rte`). The other packages place them at: package 0 same offsets; package 2 `0xffe47430`, `0xffe47706`, `0xffe4772e`, `0xffe478e2`; package 3 `0xffe4d098`, `0xffe4d36e`, `0xffe4d396`, `0xffe4d54a`; package 5 `0xffe508b0`, `0xffe50b86`, `0xffe50bae`, `0xffe50d62`.
- No reached instruction uses a word-sized index register or a scale factor of 8, and no full-format extension word occurs. The manual says those raise an address error on this part (p. 3-9). Established for reached code.

## Supervisor mode and the status register

- **Established.** The only immediate ever written to `SR` is `0x2700` (supervisor, interrupt mask 7). A word search of the loader and bank 1 finds 18 `move.w #0x2700,sr` words at even addresses in every package: the loader's reset entry, the program entry, and 16 kernel sites. In package 1 these are `0xffe00416` (reset), `0xffe10b1e` (program entry) and, in the kernel's interrupt-disable and interrupt-entry paths, `0xffe411ac`, `0xffe411e8`, `0xffe41356`, `0xffe413c2`, `0xffe41410`, `0xffe41478`, `0xffe414c0`, `0xffe414fa`, `0xffe41572`, `0xffe415a0`, `0xffe415e8`, `0xffe41680`, `0xffe416e4`, `0xffe4177a`, `0xffe41794` and `0xffe41840`; each sits in a plausible instruction run (disassembled around `0xffe411a4`, `0xffe411e8` and `0xffe41768`). The earlier text gave 16 for bank 1 plus the loader, which matches none of the readings of the scan, and the scan's own count was lower than 18: `0xffe411e8` has no reference to it, and `0xffe4177a` and `0xffe41794` are in a routine that is referenced only as data, so whether the scan reached them depends on how the data reference is followed. The 18 repeat in the bank-2 copy (35 words over both banks plus the loader). The conclusion does not depend on the count: no other immediate is written. Seven further `move.w d0,sr` and eight `move.w d1,sr` restore a value saved earlier by `move.w sr,dN` (14 such saves per package in bank 1; 15 restores). A plain word search finds more even-address `move.w sr,d0/d1` (17) and `move.w d0/d1,sr` (18) words in package 1's loader and bank 1; the difference from the reached counts is code the scan did not reach or data that looks like an opcode, and has not been split.
- **Inferred.** The image never leaves supervisor mode: every `SR` write is either the `0x2700` immediate or a restore of an `SR` value read from `SR` itself, and the two task-frame builders above use `0x2000`. The restored values come from register contents, so a user-mode value could in principle arrive through a task frame built from data I did not trace. Test in [Unknowns](#unknowns).
- **Established.** There is no `STOP` word (`0x4e72`) at any even offset of any body, and the idle path of the kernel spins in a loop ([board-boot-contract.md](board-boot-contract.md)). No `TRAP`, no `MOVE USP`, no user stack pointer use.

## `HALT`, `ILLEGAL` and the deliberate faults

`HALT` is `0x4ac8` (a 68040-mode disassembler does not decode it; every other reached word was cross-checked against that disassembler and agrees on instruction length, with `HALT` the only word it cannot decode). It suspends the core (manual pp. 1-13, 15-5; supervisor-only by default).

| Package | `HALT` sites (bank 1 and loader) | Role |
|---|---|---|
| all | `0xffe0083e` | end of the loader's own exception routine `0xffe00824`; all 254 handler words of the packaged vector table are `bsr.w` stubs to it (established, all five packages) |
| all | `0xffe00b1c` | end of the loader; **inferred** from the branch structure (`0xffe00b06` to `0xffe00b1c`) to be reached when no valid candidate is found or the candidate returns |
| 0 | `0xffe56a6c` | late-start guard, below |
| 1 | `0xffe566bc` | late-start guard, below |
| 2 | `0xffe76ed0` | late-start guard |
| 3 | `0xffe69a08` | late-start guard |
| 5 | `0xffe78764` | late-start guard |

The late-start guard (package 1, `0xffe566a8` to `0xffe566bc`): after the routine that arms the software watchdog, the code tests bit 7 of the byte at offset `0x41` from the peripheral base (`SYPCR`, bit `SWE`, manual p. 8-13) and executes `HALT` when it is clear. **Inferred** role: an assertion that the watchdog got armed. The arming routine (`0xffe562d4`) writes `0xfe` to `SYPCR` (`MBAR+0x41`; `moveq #-2`) when its once-only flag at `0x3000030c` is nonzero, and clears the flag; if the flag is zero it writes nothing, `SYPCR` bit 7 stays clear and the guard halts. **Established:** no instruction names the flag's address except that routine (the address occurs only at `0xffe562da`, and in bank 2 at `0xffec62da`), and the program entry's start-up routine (`0xffe54b0c`, called at `0xffe10b3a`) copies the initialised-data image from `0xffe7f0e4` to `0x30000000` up to `0x30000c38` (loop `0xffe54bbc` to `0xffe54bc0`). The word at flag offset `0x30c` of that image, `0xffe7f3f0`, is `0x00000001`. So the flag starts nonzero, the first call arms the watchdog, and the guard `HALT` is not executed on a start whose `SYPCR` read-back returns the byte just written. The arming routine is called from `0xffe1076a` in the main start-up path and again from the guard's function at `0xffe566a8` (**inferred**: the second call finds the flag clear and writes nothing, so the guard then relies on `SYPCR` keeping bit 7 (it is write-once, manual p. 8-13)).

`ILLEGAL` (`0x4afc`) appears once per bank, directly after an unconditional branch inside a small shift-and-compare helper (package 1: `0xffe7ee7a`; package 0 `0xffe7eeaa`; package 2 `0xffea8d8a`; package 3 `0xffeae0ae`; package 5 `0xffeb013e`). It is not on any reached path. Established.

## MAC and EMAC

The MCF5206e has a MAC unit (manual pp. 1-1, 1-5, 3-5; the rev. 3 errata add that the 5206e MAC has no fractional operand support). It has no EMAC. Every MAC instruction and every move to or from `ACC`, `MACSR` and `MASK` lives in line A; the manual says any non-MAC line-A word raises vector 10 (p. 3-9).

- **Established, reached code.** Across all five packages, no reached instruction has a word whose top nibble is `0xa`. Line-A words occur only as operand words of reached instructions (low halves of absolute addresses, immediates, displacements) and in unreached bytes. Bank 1 plus loader, packages 0, 1, 2, 3, 5: 851, 878, 714, 370 and 781 line-A words inside reached instructions' operands; 345, 353, 1,059, 1,520 and 1,223 in unreached bytes.
- **Inferred, unreached bytes.** In package 1, 329 of the 353 unreached line-A words are the low half of a 32-bit address or constant, the preceding word being `0xffe0` to `0xffff`, `0x3000`, `0x1000` or zero. The other 24 are in six places: a table of address pairs (`0xffe44968`, `0xffe449d8`), an immediate that the scan mis-aligned (`0xffe494c2`, inside `move.l #0x3c9ba5e3`), the 256-entry CRC table (`0xffe62a60` to `0xffe62c36`, which [board-boot-contract.md](board-boot-contract.md) already identifies), two words at `0xffe7dbae` and the image metadata word at `0xffe7fd3c`. Package 0 has the same three non-address groups. In packages 2, 3 and 5, 846, 1,395 and 1,084 unreached line-A words sit mostly in dense tables of ascending 16-bit offsets paired with small flag words (for example package 2 `0xffeabe1e`-`0xffeac0ae`, `0xffeb031e`-`0xffeb04ce`; package 3 `0xffe7ee02`-`0xffe7fa42`, `0xffeb149a`-`0xffeb17d6`; package 5 `0xffeb392a`-`0xffeb3d72`), a pattern no instruction stream produces.
- The MAC model state (`ACC`, `MACSR`, `MASK`) has no reader or writer anywhere in reached code.

## Instruction classes the core must execute

Counts are instructions reached in bank 1 plus the loader, per package (0, 1, 2, 3, 5), so about half the whole-image figure. All forms are the ColdFire encodings; the 68k-only forms never appear (checked: every reached word decoded by a decoder written for the base set plus MAC, and every reached instruction length equals the length a 68040-mode disassembler gives).

| Class | Counts | Notes |
|---|---|---|
| `MULS.L` (32x32 to 32, signed) | 268, 272, 362, 561, 655 | No unsigned long multiply, no 64-bit result form |
| `DIVS.L`, `DIVU.L` quotient | 79+19, 79+19, 82+23, 87+27, 101+23 | 32/32 only |
| `REMS.L`, `REMU.L` (remainder to a different register) | 63+4, 63+4, 68+4, 67+13, 78+11 | same opcode, remainder register differs from quotient register |
| `MULU.W`, `MULS.W` | 4 and 1 in every package | |
| `EXTB.L` | 249, 250, 259, 287, 290 | `0x49c0` |
| `EXT.W`, `EXT.L` | 46 and 162/157/191/147/155 | |
| `SWAP`, `Scc`, `ADDX`, `NEGX.L` | 15, 31 to 43, 8, 2 | |
| shifts `ASR`, `LSL`, `LSR`, `ASL` | data-register destination, long size | one `ASL` per bank in every package: `asl.l #2,d1` (word `0xe581`) in a small table-lookup helper, package 1 at `0xffe10bc8` (helper from `0xffe10bbe`; called at `0xffe425f8`; same word at `0xffe80bc8`); package 0 `0xffe10c04`, package 2 `0xffe10bc8`, package 3 `0xffe10b58`, package 5 `0xffe10b78`. `ASL.L` is in the base set, so the conclusion is unchanged |
| `LINK.W`, `UNLK`, `MOVEM.L`, `LEA`, `PEA`, `JSR`, `JMP`, `BSR`, `Bcc` | thousands | `LINK.L`, `MOVEM.W` and long-displacement branches have zero reached instances |
| `NOP` | 1, 1, 9, 9, 9 | |
| `MOVE` to `SR` and from `SR` | 31 and 14 | see supervisor section |

Multiply and divide use the extension word with bits 15, 10 and 9-3 zero (32-bit forms only); [board-boot-contract.md](board-boot-contract.md) cites 376 divide sites in package 1 from its linear listing, while this scan reaches 126+158+38+8 = 330 `DIVx.L`/`REMx.L` instructions over both banks (165 per bank: 79+19 quotient plus 63+4 remainder; an earlier draft of this sentence read 159 for the second term, which is 2 x 79 = 158). A plain word search of package 1's loader and bank 1 for `0x4c40` to `0x4c7f` followed by a ColdFire-form extension word (bits 15 and 10 and 9-3 clear) finds 183 words (102 with quotient and remainder register equal, 81 with them different; 158 signed, 25 unsigned), so 366 over both banks. Neither figure equals 376. The 18 words per bank beyond the reached 165 are unclassified; see [Unknowns](#unknowns).

The manual lists these instructions and says the 5206e adds hardware divide and the MAC to the MCF5206 (pp. 1-1, 1-5, 1-13 to 1-15, table 1-4: `EXTB`, `MULS`/`MULU`, `DIVS`/`DIVU`, `REMS`/`REMU`, `HALT`, `MOVEC`, `MOVE from SR`). **Inferred** from that wording: a core limited to the original MCF5206 instruction set would not decode `MULS.L`, `DIVx.L` and `REMx.L`, which the image uses in several hundred places.

## What is not used

Reached-code opcode positions with zero hits in every package: line A (which also covers `MOV3Q`); `WDDATA` (`0xfbxx`), `WDEBUG` (`0xfbcx`), `CPUSHL` (`0xf42x`) and every other line-F word; `MVS` and `MVZ` (`0x7xxx` with bit 8 set); `TAS` and `SATS` words; `BYTEREV` and `FF1` words; `PULSE`; `TRAPF`; `STOP`; `TRAP`; `LINK.L`; `MOVEC` from a control register. Line-F words (`0xffxx` erased-flash fill and high address halves) are common as operand words and in unused flash, never as opcodes. One reached word each in packages 2 and 5 decodes as line-F (`0xfff07e88` and `0xfff16592`, the upper half of an absolute address inside an instruction of bank 2); neither has a bank-1 counterpart, and the scan enters there only through a spurious root. Treated as scan noise, not as code. Three more landings of the same kind exist, all in bank 2: a `MOVE (d16,A4),SR` word at `0xfff38280` in package 2 (the low half of a `move.l` absolute address) and two `MULU.L` words at `0xfff42f2e` and `0xfff42f34` in package 5. Bank 1 has none, and the mirror check shows bank 2's real instruction stream equals bank 1's.

## What the image asks of a core

Established for reached code, inferred for the unreached remainder:

- Decode the base ColdFire set as listed above, including `MULS.L`, `DIVS.L`, `DIVU.L`, `REMS.L`, `REMU.L` (32-bit forms), `MULS.W`, `MULU.W`, `EXTB.L`, `MOVE` to and from `SR`, `RTE` and `HALT`. Nothing outside that set, and in particular no MAC, no `MOV3Q`/`MVS`/`MVZ`/`SATS`/`BYTEREV`/`FF1`, and no `TRAP` or `STOP`.
- Accept `MOVEC` writes to `CACR` (`0x002`), `VBR` (`0x801`), `RAMBAR` (`0xc04`) and `MBAR` (`0xc0f`). Both base registers need their valid bit: the peripheral block (`0x10000000`) and the SRAM (`0x30100000`) are reached through them, and the loader reads the reset-status byte at `0x10000040` before `MBAR` is written. These two facts conflict: the manual (section 8.2.1, p. 8-7) clears the `MBAR` valid bit at reset, so on that part the early read is not answered by the peripheral block. The loader code is `0xffe00ac8` (`move.b 0x40(a0),d0` with `a0` the `0x10000000` argument passed at `0xffe0041e`), followed by `btst #5` and a branch at `0xffe00ad0`; the loader has no `MBAR` write. A core model has to choose what that read returns, and the choice steers the branch. The board-level reading (an external read decoded by the boot-flash chip select, returning the flash byte) is in [board-boot-contract.md](board-boot-contract.md).
- Build and unwind the ColdFire exception frame (format/vector word and `SR`, then `PC`), fetch vectors from `VBR + 4 * n`, and run everything in supervisor mode with the single stack pointer.
- Treat `HALT` as terminal: the image uses it only as an end-of-line fault stop and nothing in it arranges to resume. The manual (section 15.2.1) describes the halted state as one a debug module can leave (the `GO` command); only the fault-on-fault halt is stated to need a reset (section 3.5.10). A core with no debug module can therefore stop for good, which loses nothing here.

So the instruction decoder does not need to be MCF5206e-specific: a base-set core with hardware multiply and divide is enough. What is specific to the 5206e is the register file around the core (`MBAR` and `RAMBAR` with their valid bits, the instruction-cache enable and invalidate handshake in `CACR`), not any instruction. A core limited to the original MCF5206 set, without the 32-bit multiply and divide, would not decode the `MULS.L` and `DIVS.L` the kernel executes (inferred from the manual's description of what the 5206e adds).

## Coverage and method

A decoder for the base set plus MAC (instruction lengths and operand-extension rules from the manual's addressing-mode tables) was run by control flow from: the reset vector and all 254 handler words of the packaged table; the first-stage loader; each candidate entry and each of its 254 handler words; every `JSR`/`BSR`/`JMP`/`Bcc` target; every code address that appears as an immediate, `LEA`/`PEA` operand or aligned 32-bit data word and that starts a run of valid instructions; and the case blocks of every `jmp d8(pc,Xn)` jump table (table length from its first entry). Reached code was then checked word for word against the mirrored bank. Cross-check: for every reached instruction a 68040-mode disassembler (the public Capstone library) gives the same length. Zero length mismatches over 186,461, 188,363, 236,447, 255,931 and 269,061 reached instructions (packages 0, 1, 2, 3, 5, both banks). It cannot decode `HALT` (four sites per package), and cannot decode the few noise landings above.

Reached code is 55% of the 1 MB packages' bank-1 region and 35% to 40% of the 2 MB packages'. The rest is text, tables, erased flash (0xff fill is 10.7% of the 1 MB bodies and 21% to 23% of the 2 MB bodies) and code not reached. The reach loop left two undecodable words in each 1 MB package and three or four in each 2 MB package (both banks), all at text or data (for example the two bytes `4e 56` that begin a text string at `0xffe5b154` in package 1), none a real instruction.

## Unknowns

| Unknown | Why it stays open | Software test that settles it |
|---|---|---|
| Whether any unreached code uses MAC or another extension instruction | 45% to 65% of each bank is not reached by control flow; the unreached line-A and line-F words are explained as data (above) but not proven | Extend the reached set to a fixed point over indirect calls: list every `jsr (An)` and `jmp (An)` site, trace each register to its last load (RAM variable, task descriptor, static table), add the resolved targets as roots, and re-run the opcode-position scan. Settled when the root set stops growing and the scan still reports only `MOVEC`, `HALT` and `RTE` |
| Whether the image ever runs in user mode | `SR` restores take register values saved from `SR`; the second task-setup site stores `0x2000` through a control-block pointer I did not follow to the frame | Find every store to the task-control-block field at offset `0x70` (package 1, set up at `0xffe54a52`) and every read of it; confirm each value is `0x2000` or a saved `SR` and that the frame builder at `0xffe41230` is the only reader. Then confirm no `move.w #imm,sr` has an immediate with bit 13 clear (already true: only `0x2700`) |
| Divide count against the earlier figure of 376 | The earlier count came from a retained linear listing that is not in this repository. The word search above gives 183 per bank (366 over both) against 165 reached | List the 18 per-bank words the scan did not reach and classify each as the middle of a longer instruction, data, or code the scan missed (disassemble 16 bytes before it and check that the run ends exactly at the word); the 376 is settled when the listing's extra 10 are shown to be words without a ColdFire-form extension |
| Behaviour of the other firmware versions | Only 1.44.0 bytes were read; the 1.4.1 to 1.62.1 sets listed in [firmware-versions.md](../../../evidence/firmware-versions.md) share the structure but were not scanned | Run the same scan on each version's container and diff the `MOVEC`, `HALT` and `RTE` site lists |
| Whether the late-start guard `HALT` is ever executed | The once-only flag starts at 1 (data image word at `0xffe7f3f0`), so the guard is skipped when `SYPCR` reads back bit 7 set after the `0xfe` write. Not shown: every other writer of the flag through a computed pointer or of `SYPCR` with bit 7 clear | Search all stores to `MBAR+0x41` and every store through a pointer that can alias `0x3000030c` for a value that clears the flag early or `SYPCR` bit 7; if none, the guard is dead on every start |

## Review (2026-11-10)

An independent review of the earlier revision of this file checked it against `l1.bin` with a separate disassembly and a separate control-flow scan. Each point was re-checked here against the image and the manual.

Changed:

- **`ASL` is used.** The shift row said there was no `ASL`; there is one `asl.l #2,d1` per bank in each of packages 0, 1, 2, 3 and 5 (addresses in the table). Reached status is inferred: the helper's caller is in a routine whose address sits in a pointer table at `0xffe10422`. No effect on the conclusion.
- **Divide arithmetic.** The sum was 2 x 79 = 158, not 159, so 330 over both banks. The added word-search figure (183 per bank) does not close the gap to the earlier 376 either; the unknown stays open with a sharper test.
- **`move.w #0x2700,sr` count.** 18 words per package in the loader and bank 1, listed for package 1; the old 16 matched no reading. The conclusion (only `0x2700` is ever written) stands.
- **`HALT` exit.** The manual allows a debug module to resume a halted core; only fault-on-fault needs a reset. The modelling advice is reworded; behaviour of the image is unchanged.
- **Reset-status read.** The conflict between the early read at `0x10000040` and the cleared `MBAR` valid bit is now stated here, with the loader addresses, and points to the board-level reading.
- **Late-start guard.** The review left this open as a depends-on-initialised-data question. It is answered: the start-up copy loop gives the flag the value 1, so the arming routine writes `SYPCR` and the guard is skipped. This is established for the image; the remaining unknown is only about aliasing writers.

Rejected: nothing in the review was wrong on its facts. Not adopted: the review's own reached-instruction count (91,263 for package 1 against 94,355 here) because the two scanners treat data-referenced code differently and neither has been shown complete; the figures in the packages table stay as measured by the original scan.

Not rechecked: the exact reached-instruction and line-A counts, user-mode use through the task control block field at offset `0x70`, and whether unreached code uses MAC.

## Sources

- MCF5206e User's Manual, MCF5206EUM/D rev. 1 (1998): table 1-4 (instruction set summary, pp. 1-12 to 1-15), section 3.2.4.2 (`VBR`, p. 3-5), section 3.4 (exception frame, p. 3-7), sections 3.5.2 and 3.5.3 (address error, line A and line F, p. 3-9), chapter 4 (`CACR`, `ACR`, pp. 4-5 to 4-8), chapter 5 (`RAMBAR`, pp. 5-1 to 5-3), chapter 8 (`MBAR` and `SYPCR`, pp. 8-6 to 8-13), table 15-5 (control register map, p. 15-22), section 15.2.1 (`HALT`, p. 15-5). Rev. 3 errata, items 3.2.2 and 8.3.2.1.
- [board-boot-contract.md](board-boot-contract.md), [coldfire-package-sections.md](coldfire-package-sections.md), [package-container.md](package-container.md), [mcf5206e-peripherals.md](mcf5206e-peripherals.md): reset order, loader candidates, register values read earlier for package 1.
