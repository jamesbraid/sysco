# Tier-2 routine readings, sysco round 4 batch 4

Role statements for targets 100 to 124 of the tier-2 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). All 25 are in `6.5.12/usr/sbin/flashsc`.
All 25 are inferred. The grader accepted every entry, so none were rejected.

- `BuffUncompress` in `6.5.12/usr/sbin/flashsc` at 0x1002dbc0
  **Inferred.** Decodes a byte stream into a buffer it allocates. It reads a 4-byte big-endian length word from a0 (lbu at 0x1002dc1c through 0x1002dc80), passes it to malloc at 0x1002dca4, and returns -1 at 0x1002dcd4 when the allocation fails. The decode loop compares loaded bytes with 0xfe and 0xfd (bne at 0x1002dd08 and 0x1002ddbc) and writes output bytes with sb. It stores two result words through a2 and a3 (sw at 0x1002dee8 and 0x1002def4) and returns 0 at 0x1002df08. The two marker constants are the only evidence for the encoding.
- `BuffFree` in `6.5.12/usr/sbin/flashsc` at 0x1002df10
  **Inferred.** Null-checked wrapper around free. It skips the call when a0 is zero (beq at 0x1002df30), otherwise calls free at 0x1002df40 with its four argument registers and returns 0 or -1 depending on the free result (returns at 0x1002df78 and 0x1002df60).
- `RouterMakeConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002df80
  **Inferred.** Thin wrapper that forwards its arguments to the routine at 0x1002e310 through jal at 0x1002e004, shifting them into a4, a5 and a7 and setting a3 to the constant 0xc241 and a6 to 0. It returns that routine's result (return at 0x1002e018). The constant looks like a connection type or flag word.
- `RouterMakeGroupConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002e020
  **Inferred.** Same shape as RouterMakeConnection: a wrapper that calls the routine at 0x1002e310 through jal at 0x1002e0a4 with a3 fixed to the constant 0xc241 and the caller's a3 passed on in a6 and a7. It returns the callee's result at 0x1002e0b8. The group variant differs only in its argument layout.
- `RouterMakeTransConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002e0c0
  **Inferred.** Wrapper that calls the routine at 0x1002e310 through jal at 0x1002e138, with a3 set to the constant 0xc441 rather than 0xc241 and a6 set to 0, and forwards the rest of the arguments. It returns the callee's result at 0x1002e14c. The different constant suggests a different connection kind from RouterMakeConnection.
- `RouterRetryConnection` in `6.5.12/usr/sbin/flashsc` at 0x1002e160
  **Inferred.** Re-issues a connection request from fields of the record at a0. It loads words at offsets 0x8, 0xc, 0x18, 0x20, 0x24, 0xb0, 0xb4, 0xb8 and 0xbc and a halfword at 0x13a (lw at 0x1002e180 through 0x1002e1f4, lh at 0x1002e1b8), then calls the routine at 0x1002e310 through jal at 0x1002e208 with a0+44 and the loaded values as arguments. It returns that result at 0x1002e21c. The same callee at 0x1002e310 is the one the three connection wrappers above call, so this reads as a re-issued connection setup built from the stored record.
- `RouterSetGroups` in `6.5.12/usr/sbin/flashsc` at 0x1002e230
  **Inferred.** Replaces the group list held in the record at a0. It calls RouterFreeGroups at 0x1002e258 first, then allocates a block with malloc at 0x1002e2a0 sized from the word at a1, reports allocation failure through RouterMemError at 0x1002e2c0, and copies the caller's data into the block with memcpy at 0x1002e2ec. The pointer is kept at a0+0x10 and a0+0x14 (sw at 0x1002e288 and 0x1002e2b0). It returns a1 or the memcpy result (return at 0x1002e300).
- `SocketRead` in `6.5.12/usr/sbin/flashsc` at 0x1002f090
  **Inferred.** Reads from a file descriptor in a loop. It calls read at 0x1002f0e0 with a0 as the descriptor, sets errno (sw at 0x1002f0cc) and retries when the stored errno equals 0xb (bne at 0x1002f0f8), which matches the usual EAGAIN value. It stops when read returns zero or the remaining count a2 falls to zero (bgtz at 0x1002f174). It returns the read result at 0x1002f13c.
- `SocketWrite` in `6.5.12/usr/sbin/flashsc` at 0x1002f1a0
  **Inferred.** Writes to a file descriptor in a loop. It calls write at 0x1002f1e8 with a0 as the descriptor and loops while the remaining count a2 stays positive (bgtz at 0x1002f248). Unlike SocketRead it has no errno access, so it does not retry on a specific error code. It returns the write result at 0x1002f210.
- `LoggerInit` in `6.5.12/usr/sbin/flashsc` at 0x1002f270
  **Inferred.** Trims a program path to its last path component. It calls strrchr at 0x1002f298 with the slash character 0x2f on a0, takes a separate path when strrchr finds nothing (bne at 0x1002f2a4), and returns the strrchr result at 0x1002f2d4. The result is likely the name used to tag log lines.
- `LoggerHandler` in `6.5.12/usr/sbin/flashsc` at 0x1002f2e0
  **Inferred.** Stores its argument into the global word at abs 0x10047a28 (sw at 0x1002f2f8) and returns a0 (return at 0x1002f300). That reads as a handler registration setter.
- `LoggerEnable` in `6.5.12/usr/sbin/flashsc` at 0x1002f310
  **Inferred.** Reads the flag word at abs 0x10047a2c (lw at 0x1002f320) and writes it back with sw at 0x1002f32c, then returns the flag value (return at 0x1002f338). The value written is not a constant in the facts, so this is probably the enable half of an enable and disable pair. LoggerLogMessage gates its output on the same word (beq at 0x1002fc9c).
- `LoggerDisable` in `6.5.12/usr/sbin/flashsc` at 0x1002f340
  **Inferred.** Reads and writes the same flag word at abs 0x10047a2c (lw at 0x1002f350, sw at 0x1002f358) and returns its value (return at 0x1002f364). It is the counterpart of LoggerEnable, which uses the same word. The stored value comes from outside the rendered facts.
- `LoggerRestore` in `6.5.12/usr/sbin/flashsc` at 0x1002f370
  **Inferred.** Stores its argument into the flag word at abs 0x10047a2c (sw at 0x1002f388) and returns a0 (return at 0x1002f390). It looks like the restore half of a save and restore of the logger enable state.
- `LogError` in `6.5.12/usr/sbin/flashsc` at 0x1002f3a0
  **Inferred.** Error-logging entry point. It forwards its arguments to LoggerLogMessage at 0x1002fc60 through jal at 0x1002f404, passing the fixed pointer 0x10047978 in a5 and a stack buffer at sp-32 in a7, and returns the callee's result (return at 0x1002f418).
- `LogErrorHost` in `6.5.12/usr/sbin/flashsc` at 0x1002f420
  **Inferred.** Host-side variant of LogError. It forwards its four argument registers to LoggerLogMessage at 0x1002fc60 through jal at 0x1002f484, with the fixed pointer 0x10047978 in a5 and a stack buffer at sp-16 in a7, and returns the callee's result (return at 0x1002f498).
- `LoggerTime` in `6.5.12/usr/sbin/flashsc` at 0x1002f4a0
  **Inferred.** Produces a human-readable time string. It calls ctime at 0x1002f4c8 on the pointer at a0+4, measures the result with strlen at 0x1002f4e0, and overwrites the last character with a NUL byte (sb at 0x1002f4f8), which removes the newline that ctime appends. It returns the ctime string (return at 0x1002f50c).
- `LoggerFormat` in `6.5.12/usr/sbin/flashsc` at 0x1002f520
  **Inferred.** Thin wrapper that fills the fixed buffer at 0x100557e8 by calling LoggerStrFormat at 0x1002f570 (jal at 0x1002f54c) with the format string at 0x1f9f0 and the caller's a0 and a3. It returns the buffer pointer 0x100557e8 (return at 0x1002f568).
- `LoggerStrFormat` in `6.5.12/usr/sbin/flashsc` at 0x1002f570
  **Inferred.** Builds one log line into the buffer at a1. It copies a template with strcpy at 0x1002f5a0 and appends fields with strcat at 0x1002f5d8 through 0x1002f81c, using sprintf at 0x1002f688 and 0x1002f7b4 for numeric fields and strerror at 0x1002f63c for an error code at a0+16. Flag bytes at a0+12, a0+20, a0+28 and a0+32 are checked before the optional fields are appended (lbu at 0x1002f5c0, 0x1002f714, 0x1002f7d4, 0x1002f6c4). It returns the buffer pointer, or the last strcat result (return at 0x1002f830).
- `LoggerDefaultMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002f840
  **Inferred.** Formats a default message into the shared buffer at 0x10055bf0. It checks its first argument against 0 and 0x100 (blez at 0x1002f8b8, beq at 0x1002f8c8), calls strerror at 0x1002f8d8 on a value in the error range, and checks two more bounds, -0x63 and -0x320 (beq at 0x1002f918 and 0x1002f928). It calls sprintf at 0x1002f900, 0x1002f95c and 0x1002f9c8 and LoggerModuleMessage at 0x1002fa50 twice (jal at 0x1002f9a0 and 0x1002f9e0), and returns the buffer at 0x10055bf0 (return at 0x1002fa48).
- `LoggerModuleMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002fa50
  **Inferred.** Chooses between a fixed string pointer at 0x10047998 (return at 0x1002fad0) and a second value loaded through abs 0x100473dc (lw at 0x1002fae8). It makes that choice from two signed range checks against -0x63 and -0x320 (beq at 0x1002faac and 0x1002fabc), the same bounds LoggerDefaultMessage uses. Which range maps to which result is inferred from the branch layout.
- `LoggerErrorMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002fb20
  **Inferred.** Selects an error message string. It reads words at abs 0x10046f4c and 0x100473dc, checks the same -0x63 and -0x320 ranges (beq at 0x1002fb98 and 0x1002fba8), and compares a value against a0 (beq at 0x1002fbec). It returns one of two fixed string pointers, 0x10046a00 (return at 0x1002fbc0) and 0x10046a10 (return at 0x1002fc04).
- `LoggerLogMessage` in `6.5.12/usr/sbin/flashsc` at 0x1002fc60
  **Inferred.** Core log writer. When the flag word at abs 0x10047a2c is nonzero (beq at 0x1002fc9c), it takes the time through time at 0x1002fcb4. It formats the caller's format string a1 with the variable arguments a2 and a3 through vsprintf at 0x1002fd18 into a 1120-byte stack buffer at sp-1120, and skips empty messages based on the first byte of a6 (lbu at 0x1002fcf4). It then passes the line to an indirect call at 0x1002fd40 that is not resolved in the rendered facts.
- `PPPInit` in `6.5.12/usr/sbin/flashsc` at 0x1002ffd0
  **Inferred.** Initialises a 0x38-byte state record at a0. It zeroes the record with memset at 0x10030018 (a2 = 0x38), writes ten fields at offsets 0x0, 0x4, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c, 0x30 and 0x34 (sw at 0x10030028 through 0x10030098), and calls PPPReset at 0x100300a8 with a1 = 3. The name suggests a point-to-point link state record.
- `PPPReset` in `6.5.12/usr/sbin/flashsc` at 0x100300d0
  **Inferred.** Updates two fields of the record at a0, at offsets 0xc and 0x1c (sw at 0x100300f4 and 0x1003010c), under control of the argument a1. Bit 0x1 of a1 (beq at 0x100300e8) and bit 0x2 of a1 (beq at 0x10030100) select the branches that write those fields, so a1 is a mask of which fields to reset.
