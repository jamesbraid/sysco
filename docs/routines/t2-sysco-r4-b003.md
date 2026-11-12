# Routine role entries, sysco second reading list, round 4, batch 3

Role statements for targets 75 to 99 of the ranked second reading list, all in the router
daemon `6.5.12/usr/sbin/flashsc`. Each role is written from the rendered instruction facts only:
callees, constants, branch targets and the registers they test at the cited addresses. None is
marked established, because the callees that matter most (the unnamed routines and the
indirect calls) are not resolved in the facts. Where a field meaning or a message code is not
in the facts, the entry says so.

- `RouterInput` in `6.5.12/usr/sbin/flashsc` at 0x10028f40
  **Inferred.** Reads input for one client. It calls `read` at 0x10028f88 with the descriptor from [a0], the buffer at [a0+220] and the count at [a0+224], and branches to 0x100290d8 when the count is not positive (branch at 0x10028f64). On a negative result it tests errno against 0xb, 0x80, 0x83, 0x91 and 0x92 (branches at 0x10028fb8 to 0x10028ff8) and reports through `RouterConnectionLost` at 0x10029018 or 0x1002908c, `RouterLogError` at 0x10029044 or `RouterConnectionError` at 0x10029054. On the data path it copies 8 bytes with `memcpy` at 0x100292d0, grows the buffer at [a0+216] with `realloc` at 0x10029380, and calls `RouterMemError` at 0x100293a8 if that fails. What the errno values mean and what the copied 8 bytes hold is not in the facts.
  Reviewed: 2026-11-12

- `RouterProcessPacket` in `6.5.12/usr/sbin/flashsc` at 0x100294a0
  **Inferred.** Handles one unpacked input message. It calls `RouterUnpack` at 0x100294d8 with the client record, the buffer at [a0+216] and the constants 0x10045ba0 and 0x25a70. It then runs a ladder of unsigned compares (branches at 0x10029500 to 0x10029638) on values whose source is not in the facts, and each arm calls one of the unnamed routines at 0x100297e0, 0x100298f0, 0x10029be0, 0x10029c50, 0x10029ce0, 0x10029d70, 0x1002a080 or 0x1002a340. The fall-through arm calls `RouterBadClass` at 0x10029798. The call to `RouterFreeArguments` at 0x100297c0 passes the result of `RouterUnpack` and the word at `InArgList`; the facts do not show which arms reach it. Role (from name): releasing the unpacked arguments. The compare constants look like message or class codes, but the meaning of each arm is unknown.
  Reviewed: 2026-11-12

- `RouterDoOutput` in `6.5.12/usr/sbin/flashsc` at 0x1002a560
  **Inferred.** Output-side wrapper for one message. It calls `RouterPack` at 0x1002a5d4 with its four arguments unchanged, then calls an unnamed routine at 0x1002a680 from 0x1002a660 and returns that routine's value. The branch at 0x1002a5f0 compares a word loaded at 0x1002a5e0 (its address is not shown) with 0x40; when they differ it jumps to 0x1002a650, past the halfword read at [a0+232] (0x1002a5fc) and the later read of it at 0x1002a624. The role of the 0x40 test is not shown.
  Reviewed: 2026-11-12

- `RouterPack` in `6.5.12/usr/sbin/flashsc` at 0x1002ad00
  **Inferred.** Packs one outgoing message. It allocates 0x24 bytes with `malloc` at 0x1002ad80 and reports a null result through `RouterMemError` at 0x1002ada8. It calls `htonl` at 0x1002b08c, copies 4 bytes from 0x10047598 with `memcpy` at 0x1002b0ac and 0x1002b11c, and calls `BuffCompress` at 0x1002b200 on one path. Loads and stores at offsets 0x0, 0x8, 0xc, 0x10, 0x18, 0x1c, 0x20 and 0x24 from a base the facts do not name, and at +0x140 to +0x164 (0x140, 0x148, 0x14c, 0x150, 0x158, 0x15c, 0x160, 0x164) from another, look like counter updates, but which base is `OutputStats` or the client record is not shown. The branches at 0x1002ad4c and 0x1002ad68 test the word at [a0+316] and a bit of the word at [[a0+316]+4], and either one skips to 0x1002ad78. `BuffFree` is called at 0x1002b3f8; what it frees is not shown.
  Reviewed: 2026-11-12

- `RouterUnpack` in `6.5.12/usr/sbin/flashsc` at 0x1002b4a0
  **Inferred.** Unpacks one incoming message into the global argument list `InArgList`. It may first uncompress with `BuffUncompress` at 0x1002b58c (conditional on the word at [a0+240] and a bit test), then grows `InArgList` with `realloc` at 0x1002b7fc to a size computed from `InArgListSize` and `MaxInArgs` added and shifted left by 3. It copies 4 bytes into the scratch word at 0x100475a0 with `memcpy` at 0x1002b968 and 0x1002bab4 and converts that word with `ntohl` at 0x1002b988 and 0x1002bad4. It calls `RouterMemError` at 0x1002b5a8 when `BuffUncompress` returns -1, at 0x1002b824 when `realloc` returns null and at 0x1002bbb4 when `malloc` returns null; on that last path `RouterFreeArguments` at 0x1002bba8 runs first. The return value at 0x1002bcdc is unknown.
  Reviewed: 2026-11-12

- `RouterDebugStats` in `6.5.12/usr/sbin/flashsc` at 0x1002bcf0
  **Inferred.** A thin wrapper around `RouterPrintProcessStats`. It calls it at 0x1002bd1c with `sym(__iob)+16` as a0, its own a0 as a1 and its own a2 and a3 as a2 and a3 (its own a1 is not forwarded), and returns that routine's result. Role (from name): printing process statistics. That `sym(__iob)+16` is a stdio stream is inferred from the symbol name, not from the facts.
  Reviewed: 2026-11-12

- `RouterPrintProcessStats` in `6.5.12/usr/sbin/flashsc` at 0x1002bd40
  **Inferred.** Role (from name): prints the input and output statistics blocks and clears them. The branch at 0x1002bd64 skips the print calls when a0 is zero, and the branch at 0x1002bdd0 skips the two `memset` calls when a1 is zero. When printing, it calls `fprintf` at 0x1002bd7c with format 0x10045c88, then an unnamed routine at 0x1002bfd0 on `InputStats`, then `fprintf` at 0x1002bdac with format 0x10045ca8, then the same unnamed routine on `OutputStats` at 0x1002bdc4. `memset` at 0x1002bde8 and 0x1002be00 zeroes `InputStats` and `OutputStats`, 0x28 bytes each. The format strings are not in the facts, so the printed fields are unknown.
  Reviewed: 2026-11-12

- `RouterPrintAllStats` in `6.5.12/usr/sbin/flashsc` at 0x1002be20
  **Inferred.** Role (from name): prints the statistics of each client. The word at `ClientList` is tested at 0x1002be50, and the branch at 0x1002bea8 loops back to 0x1002be58. When a0 is non-zero (branch at 0x1002be5c) it calls `fprintf` at 0x1002be78 with format 0x10045cc8. It calls `RouterPrintClientStats` at 0x1002be90 with its own a0 and a1 as that routine's a1 and a2. Its return value is either the word loaded from `ClientList` or the return of `RouterPrintClientStats`, which the facts do not resolve. What the format prints is not in the facts.
  Reviewed: 2026-11-12

- `RouterPrintClientStats` in `6.5.12/usr/sbin/flashsc` at 0x1002bed0
  **Inferred.** Role (from name): prints one client's two statistics blocks and clears them. The blocks sit inside the client record at offsets +244 (0xf4) and +320 (0x140). The branch at 0x1002bef8 skips the print calls when a1 is zero. When printing, it calls `fprintf` at 0x1002bf1c with format 0x10045c88 (the same format `RouterPrintProcessStats` uses), then an unnamed routine at 0x1002bfd0 on the block at +244, then `fprintf` at 0x1002bf50 with format 0x10045ca8, then the same unnamed routine on the block at +320 at 0x1002bf6c. When a2 is non-zero (branch at 0x1002bf78), `memset` at 0x1002bf94 and 0x1002bfb0 zeroes each block, 0x28 bytes each.
  Reviewed: 2026-11-12

- `RouterFreeArguments` in `6.5.12/usr/sbin/flashsc` at 0x1002c5b0
  **Inferred.** Role (from name): frees argument pointers. It loops (branch at 0x1002c68c back to 0x1002c5e8) while a counter is below its a0, calling `free` at 0x1002c66c; the free call is passed a1 at entry as its a1 and its own a0 is not shown. Three tests on words loaded inside the loop skip the free: at 0x1002c610 when the word loaded at 0x1002c5f8 is not 1, at 0x1002c630 when a word loaded at 0x1002c628 is zero, and at 0x1002c64c when a word loaded at 0x1002c648 is zero. It returns either its a0 or an unknown value at 0x1002c6a0. What the loaded words are is not shown.
  Reviewed: 2026-11-12

- `RouterFreeClient` in `6.5.12/usr/sbin/flashsc` at 0x1002c6b0
  **Inferred.** Releases one client record. It calls `RouterFreeGroups` at 0x1002c6d8, then calls `free` on [a0+216] at 0x1002c6fc, on [a0+240] at 0x1002c720 and on [a0+316] at 0x1002c75c, each only when that word is non-zero (branches at 0x1002c6e8, 0x1002c70c and 0x1002c730). The branch at 0x1002c748 skips the free of [a0+316] when [a0+240] and [a0+316] are the same pointer. It calls an unnamed routine at 0x1002c860 with a0+204 as its a1 (0x1002c77c), one at 0x1002c8e0 with the word [a0+296] as its a1 (0x1002c79c), and the one at 0x1002c860 again with a0+284 as its a1 (0x1002c7bc). It then frees the record itself at 0x1002c7cc.
  Reviewed: 2026-11-12

- `RouterFreeGroups` in `6.5.12/usr/sbin/flashsc` at 0x1002c7f0
  **Inferred.** Frees the group block at [a0+20] when it is non-zero (branch at 0x1002c814), with `free` at 0x1002c828. Its return is either the constant 0x22720 or the return of `free`, which the facts do not resolve. The record it belongs to is not named in the facts.
  Reviewed: 2026-11-12

- `RouterAddTimer` in `6.5.12/usr/sbin/flashsc` at 0x1002cb30
  **Inferred.** Re-arms a timeout for a client. When the handle at [a0+200] is non-zero (branch at 0x1002cb5c) it cancels it with `EventRemoveTimeOut` at 0x1002cb70, then adds a new timeout with `EventAddTimeOut` at 0x1002cb8c and returns its result. The argument order into `EventAddTimeOut` is not clear from the facts, so what the timeout fires on is unknown.
  Reviewed: 2026-11-12

- `RouterRemoveTimer` in `6.5.12/usr/sbin/flashsc` at 0x1002cbb0
  **Inferred.** Cancels the timeout handle at [a0+200] by calling `EventRemoveTimeOut` at 0x1002cbd8. It passes the constant 0x22360 as the second argument and its own a2 and a3 unchanged, and returns the call's result. The meaning of 0x22360 is not shown.
  Reviewed: 2026-11-12

- `RouterLogError` in `6.5.12/usr/sbin/flashsc` at 0x1002cc00
  **Inferred.** Role (from name): logs an error for a client. It calls `LoggerLogMessage` at 0x1002cc70 with its own a0, a1 and a2 as a0 to a2, a3+44 as a3, the word [a3+8] as a4, a3+109 as a5, its own a4 as a6, and the stack address sp-24 as a7, and returns the logger's result. The fields of the record at its a3 are not named in the facts.
  Reviewed: 2026-11-12

- `RouterCheckOutput` in `6.5.12/usr/sbin/flashsc` at 0x1002cc90
  **Inferred.** Checks a condition on the record at a0 and returns 0 or -1. The branch at 0x1002ccbc tests bit 0x4 of the word at [a0+24]. On the path where that bit is clear the facts show `LogError` at 0x1002ccd8 with code -0x6e and a return of -1 at 0x1002ccf0. When the bit is set the routine returns 0 at 0x1002cd08. Which of these two arms is the failure is an inference from the code order.
  Reviewed: 2026-11-12

- `RouterConnectionLost` in `6.5.12/usr/sbin/flashsc` at 0x1002cd10
  **Inferred.** Role (from name): handles a connection that dropped. Branches at 0x1002cd40 to 0x1002cd80 test values that are not in the facts. When the value tested at 0x1002cd80 equals 0x80 it calls `RouterClose` at 0x1002cd90, with a4 set to 0x80, and returns its result at 0x1002cda4. Another path calls `RouterClientError` at 0x1002cdf8 and returns its result; the facts show -0x6b as the code there and do not resolve the others the branches select. Which condition leads to which path is unknown.
  Reviewed: 2026-11-12

- `RouterConnectionTimeout` in `6.5.12/usr/sbin/flashsc` at 0x1002ce20
  **Inferred.** Role (from name): handles a connection that timed out, with the same shape as `RouterConnectionLost`. Branches at 0x1002ce50 to 0x1002ce90 test values that are not in the facts. When the value tested at 0x1002ce90 equals 0x80 it calls `RouterClose` at 0x1002cea0, with a4 set to 0x80, and returns its result at 0x1002ceb4. Another path calls `RouterClientError` at 0x1002cf08 and returns its result; the facts show -0x6b as the code there and do not resolve the others. Which condition leads to which path is unknown.
  Reviewed: 2026-11-12

- `RouterConnectionError` in `6.5.12/usr/sbin/flashsc` at 0x1002cf30
  **Inferred.** Role (from name): reports a connection error. Branches at 0x1002cf60 to 0x1002cf80 test values that are not in the facts. It calls `RouterClientError` at 0x1002cfcc with a code the facts show as -0x6c, without resolving the others the branches may select, and returns its result. Its callers in the facts include `RouterInput` at 0x10029054 and `RouterBadClass` at 0x1002d26c.
  Reviewed: 2026-11-12

- `RouterClientError` in `6.5.12/usr/sbin/flashsc` at 0x1002cff0
  **Inferred.** Role (from name): reports an error for one client, and calls `RouterClose` afterwards. When [a0+188] is non-zero (branch at 0x1002d030) it makes an indirect call at 0x1002d060 with its own a0 and a1, the address a0+44, and the words [a0+8] and [a0+36] as a2 to a4; the callee is not resolved, so the report target is unknown. It then calls `RouterClose` at 0x1002d070 and returns its result.
  Reviewed: 2026-11-12

- `RouterBadClass` in `6.5.12/usr/sbin/flashsc` at 0x1002d090
  **Inferred.** Role (from name): handles a message whose class or field values are rejected. A compare ladder (branches at 0x1002d0cc to 0x1002d204) on values that are not in the facts leads to one of two `RouterLogError` calls, at 0x1002d22c with code -0x70 or at 0x1002d254 with code -0x71. Each path ends by calling `RouterConnectionError` at 0x1002d26c and returning its result. The compare values are unnamed, so which bad field each arm reports is unknown.
  Reviewed: 2026-11-12

- `RouterMemError` in `6.5.12/usr/sbin/flashsc` at 0x1002d290
  **Inferred.** Out-of-memory path. It calls `MemUsage` at 0x1002d2ac, then `RouterShutDownClients` at 0x1002d2b8, then `LogError` at 0x1002d2d4 with code -0x6f. If the pointer at `sym(FatalCB)` is non-zero (branch at 0x1002d2e4) it calls that pointer at 0x1002d2f8, and it calls `abort` at 0x1002d304 last. The callee at 0x1002d2f8 is indirect and unresolved.
  Reviewed: 2026-11-12

- `RouterClose` in `6.5.12/usr/sbin/flashsc` at 0x1002d320
  **Inferred.** A wrapper that closes a client with the flag set to 1. It calls `RouterDoClose` at 0x1002d348 with a1 set to 0x1 and its own a0, a2 and a3 passed through, and returns the result at 0x1002d35c. The meaning of the flag is not shown.
  Reviewed: 2026-11-12

- `RouterDoClose` in `6.5.12/usr/sbin/flashsc` at 0x1002d370
  **Inferred.** Closes a client connection. The branch at 0x1002d39c skips to 0x1002d404 when the word at [a0] is not positive, past the unnamed routine at 0x1002ca80 (0x1002d3b0), the one at 0x1002cb00 (0x1002d3e4) and `SocketClose` at 0x1002d3f8, which takes [a0]. The call to 0x1002cb00 is also skipped when [a0+304] is not positive or the halfword at [a0+312] is non-zero (branches at 0x1002d3c0 and 0x1002d3d0); the role of those tests is not shown. It then calls `RouterRemoveClient` at 0x1002d410 whichever way those branches go.
  Reviewed: 2026-11-12

- `BuffCompress` in `6.5.12/usr/sbin/flashsc` at 0x1002d430
  **Inferred.** Role (from name): compresses a buffer. Its only callee in the facts is `malloc` at 0x1002d4a0; the constant 0x21ae0 is in a4 at that call, not in malloc's size argument a0, whose value is not shown. It returns -1 at 0x1002d4cc when the result is null and 0 at 0x1002dbb4. The compression loop is byte loads and stores with no calls, so the compressed format is unknown.
  Reviewed: 2026-11-12
