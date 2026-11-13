# Tier-2 routine readings, sysco round 4 batch 4

Role statements for targets 100 to 124 of the tier-2 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). All 25 are in `6.5.12/usr/sbin/flashsc`.
All 25 are inferred. The grader accepted every entry, so none were rejected.

- `BuffUncompress` in `6.5.12/usr/sbin/flashsc` at 0x1002dbc0
  **Inferred.** Decodes a byte stream into a buffer it allocates. It reads a 4-byte big-endian length word from a0 (lbu at 0x1002dc1c through 0x1002dc80), passes it to malloc at 0x1002dca4, and returns -1 at 0x1002dcd4 when the allocation fails. The decode loop compares loaded bytes with 0xfe and 0xfd (bne at 0x1002dd08 and 0x1002ddbc) and writes output bytes with sb. After the loop, the sw stores at 0x1002dee8 and 0x1002def4 are both skipped when a2 is zero or a3 is zero (beq at 0x1002decc and 0x1002ded8); the facts leave the stored values unresolved. It returns 0 at 0x1002df08. The two marker constants are the only evidence for the encoding.
  Reviewed: 2026-11-12
- `BuffFree` in `6.5.12/usr/sbin/flashsc` at 0x1002df10
  **Inferred.** Null-checked wrapper around free. It skips the call when a0 is zero (beq at 0x1002df30), otherwise calls free at 0x1002df40 with its four argument registers and returns 0 or -1 depending on the free result (returns at 0x1002df78 and 0x1002df60).
  Reviewed: 2026-11-12
- `RouterMakeConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002df80
  **Inferred.** Thin wrapper that forwards its arguments to the routine at 0x1002e310 through jal at 0x1002e004, shifting them into a4, a5 and a7 and setting a3 to the constant 0xc241 and a6 to 0. It returns that routine's result (return at 0x1002e018). The constant looks like a connection type or flag word.
  Reviewed: 2026-11-12
- `RouterMakeGroupConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002e020
  **Inferred.** Same shape as RouterMakeConnection: a wrapper that calls the routine at 0x1002e310 through jal at 0x1002e0a4 with a0, a1 and a2 passed through and a3 set to the constant 0xc241. The caller's a3 is moved to a6 and the caller's a6 to a7. It returns the callee's result at 0x1002e0b8. The facts show no value set in a4 or a5 for this call, so the argument layout differs from RouterMakeConnection at least in a4 to a7.
  Reviewed: 2026-11-12
- `RouterMakeTransConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002e0c0
  **Inferred.** Wrapper that calls the routine at 0x1002e310 through jal at 0x1002e138, with a3 set to the constant 0xc441 rather than 0xc241 and a6 set to 0, and forwards the rest of the arguments. It returns the callee's result at 0x1002e14c. The different constant suggests a different connection kind from RouterMakeConnection.
  Reviewed: 2026-11-12
- `RouterRetryConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002e160
  **Inferred.** Re-issues a connection request from fields of the record at a0. It loads words at offsets 0x8, 0xc, 0x18, 0x20, 0x24, 0xb0, 0xb4, 0xb8 and 0xbc and a halfword at 0x13a (lw at 0x1002e180 through 0x1002e1f4, lh at 0x1002e1b8), then calls the routine at 0x1002e310 through jal at 0x1002e208. In the argument registers it passes a0+44, the words at offsets 0x8, 0xc and 0x20, the halfword at 0x13a, the address a0+16 and the word at 0xb0, plus an a3 that the facts do not resolve; the facts show no register argument for the words at 0xb4, 0xb8 and 0xbc. It returns the callee's result at 0x1002e21c. The same callee at 0x1002e310 is the one the three connection wrappers above call, so this reads as a re-issued connection setup built from the stored record.
  Reviewed: 2026-11-12
- `RouterSetGroups` in `6.5.12/usr/sbin/flashsc` at 0x1002e230
  **Inferred.** Replaces the group list held in the record at a0. It calls RouterFreeGroups at 0x1002e258 first. When a1 is nonzero and the word at a1 is nonzero (beq at 0x1002e264 and 0x1002e274), it stores the word at a1 into a0+0x10 (sw at 0x1002e288), allocates a block with malloc at 0x1002e2a0 and stores the malloc result at a0+0x14 (sw at 0x1002e2b0). When malloc returns zero it calls RouterMemError at 0x1002e2c0 (bne at 0x1002e2b4 not taken). It then copies with memcpy at 0x1002e2ec, taking the source from the word at a1+4 and the destination from a0+0x14. It returns a1 when either check skips the work, otherwise the memcpy result (return at 0x1002e300).
  Reviewed: 2026-11-12
- `SocketRead` in `6.5.12/usr/sbin/flashsc` at 0x1002f090
  **Inferred.** Reads from a file descriptor in a loop. Each pass stores a word at 0x1002f0cc, calls read at 0x1002f0e0 with a0 as the descriptor, then loads a word and compares it with 0xb (bne at 0x1002f0f8). The facts show the equal case reaching a return at 0x1002f118, not a retry. When read returns a negative value it returns that result at 0x1002f13c. When read returns zero it leaves the loop (beq at 0x1002f148), and otherwise it loops again while the remaining count a2 is positive (bgtz at 0x1002f174). The returns at 0x1002f118 and 0x1002f194 have values the facts do not resolve.
  Reviewed: 2026-11-12
- `SocketWrite` in `6.5.12/usr/sbin/flashsc` at 0x1002f1a0
  **Inferred.** Writes to a file descriptor in a loop. It calls write at 0x1002f1e8 with a0 as the descriptor. When write returns a negative value it returns that result at 0x1002f210. When write returns zero it leaves the loop (beq at 0x1002f21c), and otherwise it loops while the remaining count a2 stays positive (bgtz at 0x1002f248). The return at 0x1002f268 has a value the facts do not resolve. Unlike SocketRead the facts show no load of an errno-like word.
  Reviewed: 2026-11-12
- `LoggerInit` in `6.5.12/usr/sbin/flashsc` at 0x1002f270
  **Inferred.** Role (from name): initialisation of the logger; the facts do not show what is initialised. It calls strrchr at 0x1002f298 with the slash character 0x2f in a1 and a0 as the string, then branches on whether the result is nonzero (bne at 0x1002f2a4); the facts do not describe the two paths. The return value shown at 0x1002f2d4 is the strrchr result.
  Reviewed: 2026-11-12
- `LoggerHandler` in `6.5.12/usr/sbin/flashsc` at 0x1002f2e0
  **Inferred.** Stores its argument into the global word at abs 0x10047a28 (sw at 0x1002f2f8) and returns a0 (return at 0x1002f300). That reads as a handler registration setter.
  Reviewed: 2026-11-12
- `LoggerEnable` in `6.5.12/usr/sbin/flashsc` at 0x1002f310
  **Inferred.** Reads the flag word at abs 0x10047a2c (lw at 0x1002f320), stores a new value to it with sw at 0x1002f32c, then returns the value read before the store (return at 0x1002f338). The facts leave the stored value unresolved. Role (from name): the enable half of an enable and disable pair. LoggerLogMessage gates its output on the same word (beq at 0x1002fc9c).
  Reviewed: 2026-11-12
- `LoggerDisable` in `6.5.12/usr/sbin/flashsc` at 0x1002f340
  **Inferred.** Reads the flag word at abs 0x10047a2c (lw at 0x1002f350), stores a new value to it with sw at 0x1002f358 and returns the value read before the store (return at 0x1002f364). The facts leave the stored value unresolved. Role (from name): the disable counterpart of LoggerEnable, which uses the same word.
  Reviewed: 2026-11-12
- `LoggerRestore` in `6.5.12/usr/sbin/flashsc` at 0x1002f370
  **Inferred.** Stores its argument into the flag word at abs 0x10047a2c (sw at 0x1002f388) and returns a0 (return at 0x1002f390). It looks like the restore half of a save and restore of the logger enable state.
  Reviewed: 2026-11-12
- `LogError` in `6.5.12/usr/sbin/flashsc` at 0x1002f3a0
  **Inferred.** Error-logging entry point. It calls LoggerLogMessage at 0x1002fc60 through jal at 0x1002f404, passing a0, a1 and a2 through, moving the caller's a3 to a6, and setting a3 and a5 to the fixed pointer 0x10047978, a4 to 0 and a7 to a stack address at sp-32. It returns the callee's result (return at 0x1002f418).
  Reviewed: 2026-11-12
- `LogErrorHost` in `6.5.12/usr/sbin/flashsc` at 0x1002f420
  **Inferred.** Role (from name): host-side variant of LogError. It calls LoggerLogMessage at 0x1002fc60 through jal at 0x1002f484, passing a0 to a3 through unchanged, moving the caller's a5 to a6, and setting a5 to the fixed pointer 0x10047978 and a7 to a stack address at sp-16. It returns the callee's result (return at 0x1002f498).
  Reviewed: 2026-11-12
- `LoggerTime` in `6.5.12/usr/sbin/flashsc` at 0x1002f4a0
  **Inferred.** Likely produces a time string. It calls ctime at 0x1002f4c8 with the address a0+4 as its argument, measures the result with strlen at 0x1002f4e0, then stores one byte at offset -1 (sb at 0x1002f4f8). The facts do not show the stored value or its base, so this probably overwrites the last character of the ctime string, such as the trailing newline. It returns the ctime result (return at 0x1002f50c).
  Reviewed: 2026-11-12
- `LoggerFormat` in `6.5.12/usr/sbin/flashsc` at 0x1002f520
  **Inferred.** Thin wrapper that fills the fixed buffer at 0x100557e8 by calling LoggerStrFormat at 0x1002f570 (jal at 0x1002f54c) with the caller's a0 and a3 passed through, the buffer address in a1 and the constant 0x1f9f0 in a2. The facts do not show how a2 is used. It returns the buffer pointer 0x100557e8 (return at 0x1002f568).
  Reviewed: 2026-11-12
- `LoggerStrFormat` in `6.5.12/usr/sbin/flashsc` at 0x1002f570
  **Inferred.** Builds one log line into the buffer at a1. It calls strcpy at 0x1002f5a0 with the buffer as destination and the word at a0 as source, then appends with strcat (calls at 0x1002f5d8 through 0x1002f81c), using sprintf at 0x1002f688 and 0x1002f7b4 to append values from a0+16 and a0+24, and strerror at 0x1002f63c on the word at a0+16. The words at a0+12, a0+32, a0+20 and a0+28 are each loaded as pointers and their first byte tested (lbu at 0x1002f5c0, 0x1002f6c4, 0x1002f714, 0x1002f7d4); the matching strcat calls run only when the pointer and its first byte are nonzero. The word at a0+24 is appended only when it is positive (blez at 0x1002f768). It returns the buffer pointer or the last strcat result (return at 0x1002f830).
  Reviewed: 2026-11-12
- `LoggerDefaultMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002f840
  **Inferred.** Formats a default message into the shared buffer at 0x10055bf0. It checks its first argument against 0 and 0x100 (blez at 0x1002f8b8, beq at 0x1002f8c8) and, on the path where it is between them, calls strerror at 0x1002f8d8 and sprintf at 0x1002f900. It checks two more bounds, -0x63 and -0x320 (beq at 0x1002f918 and 0x1002f928), on values the facts do not resolve. Other paths call sprintf at 0x1002f95c, 0x1002f9c8 and 0x1002fa2c and LoggerModuleMessage at 0x1002fa50 twice (jal at 0x1002f9a0 and 0x1002f9e0). It returns the buffer at 0x10055bf0 (return at 0x1002fa48).
  Reviewed: 2026-11-12
- `LoggerModuleMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002fa50
  **Inferred.** Chooses between a fixed string pointer at 0x10047998 (return at 0x1002fad0) and a second value loaded through abs 0x100473dc (lw at 0x1002fae8). It makes that choice from two signed range checks against -0x63 and -0x320 (beq at 0x1002faac and 0x1002fabc), the same bounds LoggerDefaultMessage uses. Which range maps to which result is inferred from the branch layout.
  Reviewed: 2026-11-12
- `LoggerErrorMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002fb20
  **Inferred.** Selects an error message string. It checks the same -0x63 and -0x320 ranges as LoggerModuleMessage (beq at 0x1002fb98 and 0x1002fba8) and compares a loaded value against another (beq at 0x1002fbec). It has three returns: the fixed pointer 0x10046a00 (return at 0x1002fbc0), the fixed pointer 0x10046a10 (return at 0x1002fc04) and a loaded value the facts do not resolve (return at 0x1002fc40). Which range leads to which return is inferred from the branch layout.
  Reviewed: 2026-11-12
- `LoggerLogMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002fc60
  **Inferred.** Core log writer. When the flag word at abs 0x10047a2c is nonzero (beq at 0x1002fc9c), it calls time at 0x1002fcb4. It tests the first byte of a6 (lbu at 0x1002fcf4) and, when that byte is nonzero, calls vsprintf at 0x1002fd18 with a0 = sp-1120, a1 = the entry's a6, a2 = sp-8 and a3 = the entry's a3. An indirect call at 0x1002fd40 follows with a0 = sp-1168; the facts do not resolve its callee.
  Reviewed: 2026-11-12
- `PPPInit` in `6.5.12/usr/sbin/flashsc` at 0x1002ffd0
  **Inferred.** Initialises a 0x38-byte state record at a0. It zeroes the record with memset at 0x10030018 (a2 = 0x38), writes ten fields at offsets 0x0, 0x4, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c, 0x30 and 0x34 (sw at 0x10030028 through 0x10030098), and calls PPPReset at 0x100300a8 with a1 = 3. The name suggests a point-to-point link state record.
  Reviewed: 2026-11-12
- `PPPReset` in `6.5.12/usr/sbin/flashsc` at 0x100300d0
  **Inferred.** Updates two fields of the record at a0, at offsets 0xc and 0x1c (sw at 0x100300f4 and 0x1003010c). The store at 0xc is skipped when bit 0x1 of a1 is clear (beq at 0x100300e8), and the store at 0x1c is skipped when bit 0x2 of a1 is clear (beq at 0x10030100). The facts leave the stored values unresolved. Role (from name): a reset of those fields, selected by a1 as a bit mask.
  Reviewed: 2026-11-12
