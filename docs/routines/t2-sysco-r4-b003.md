# Routine role entries, sysco second reading list, round 4, batch 3

Role statements for targets 75 to 99 of the ranked second reading list, all in the router
daemon `6.5.12/usr/sbin/flashsc`. Each role is written from the rendered instruction facts only:
callees, constants, branch targets and the registers they test at the cited addresses. None is
marked established, because the callees that matter most (the unnamed routines and the
indirect calls) are not resolved in the facts. Where a field meaning or a message code is not
in the facts, the entry says so.

- `RouterInput` in `6.5.12/usr/sbin/flashsc` at 0x10028f40
  **Inferred.** Reads input for one client. It calls `read` at 0x10028f88 with the descriptor from [a0], the buffer at [a0+220] and the count at [a0+224], and branches to 0x100290d8 when the count is not positive (branch at 0x10028f64). On a negative result it tests errno against 0xb, 0x80, 0x83, 0x91 and 0x92 (branches at 0x10028fb8 to 0x10028ff8) and reports through `RouterConnectionLost` at 0x10029018 or 0x1002908c, `RouterLogError` at 0x10029044 or `RouterConnectionError` at 0x10029054. On the data path it copies 8 bytes with `memcpy` at 0x100292d0, grows the buffer at [a0+216] with `realloc` at 0x10029380, and calls `RouterMemError` at 0x100293a8 if that fails. What the errno values mean and what the copied 8 bytes hold is not in the facts.

- `RouterProcessPacket` in `6.5.12/usr/sbin/flashsc` at 0x100294a0
  **Inferred.** Handles one unpacked input message. It calls `RouterUnpack` at 0x100294d8 with the client record, the buffer at [a0+216] and the constants 0x10045ba0 and 0x25a70. It then runs a ladder of unsigned compares (branches at 0x10029500 to 0x10029638) on values whose source is not in the facts, and each arm calls one of the unnamed routines at 0x100297e0, 0x100298f0, 0x10029be0, 0x10029c50, 0x10029ce0, 0x10029d70, 0x1002a080 or 0x1002a340. The fall-through arm calls `RouterBadClass` at 0x10029798, and `RouterFreeArguments` at 0x100297c0 releases the unpacked arguments. The compare constants look like message or class codes, but the meaning of each arm is unknown.

- `RouterDoOutput` in `6.5.12/usr/sbin/flashsc` at 0x1002a560
  **Inferred.** Output-side wrapper for one message. It calls `RouterPack` at 0x1002a5d4 with its four arguments unchanged, then calls an unnamed routine at 0x1002a680 from 0x1002a660 and returns that routine's value. A halfword at [a0+232] is read at 0x1002a5fc and 0x1002a624, and the branch at 0x1002a5f0 tests it against 0x40 to choose between the two paths. The role of the 0x40 test is not shown.

- `RouterPack` in `6.5.12/usr/sbin/flashsc` at 0x1002ad00
  **Inferred.** Packs one outgoing message. It allocates a 0x24-byte header with `malloc` at 0x1002ad80 and reports failure through `RouterMemError` at 0x1002ada8. It byte-swaps fields with `htonl` at 0x1002b08c, copies 4-byte words from 0x10047598 with `memcpy` at 0x1002b0ac and 0x1002b11c, and calls `BuffCompress` at 0x1002b200 on one path. It also updates counters in `OutputStats` at several offsets (0x0, 0x8, 0xc, 0x10, 0x18, 0x1c, 0x20, 0x24) and fields of the client record at +0x140 to +0x164. The condition at 0x1002ad4c and 0x1002ad68 selects an optional path from the word at [a0+316]. The buffer is released with `BuffFree` at 0x1002b3f8.

- `RouterUnpack` in `6.5.12/usr/sbin/flashsc` at 0x1002b4a0
  **Inferred.** Unpacks one incoming message into the global argument list `InArgList`. It may first uncompress with `BuffUncompress` at 0x1002b58c, then grows `InArgList` with `realloc` at 0x1002b7fc to a size computed from `InArgListSize` and `MaxInArgs` shifted left by 3. Fields are read as 4-byte big-endian words from 0x100475a0 with `memcpy` at 0x1002b968 and 0x1002bab4 and `ntohl` at 0x1002b988 and 0x1002bad4. On failure it calls `RouterFreeArguments` at 0x1002bba8 or `RouterMemError` at 0x1002b5a8, 0x1002b824 and 0x1002bbb4. The return value at 0x1002bcdc is unknown.

- `RouterDebugStats` in `6.5.12/usr/sbin/flashsc` at 0x1002bcf0
  **Inferred.** A thin wrapper that prints process statistics to a stdio stream. It passes `sym(__iob)+16` as the stream to `RouterPrintProcessStats` at 0x1002bd1c, with its own arguments a1 to a3 forwarded, and returns that routine's result. The stream choice comes from the address expression, not a name.

- `RouterPrintProcessStats` in `6.5.12/usr/sbin/flashsc` at 0x1002bd40
  **Inferred.** Prints the input and output statistics blocks and then clears them. It calls `fprintf` at 0x1002bd7c with format 0x10045c88 and at 0x1002bdac with format 0x10045ca8, and an unnamed routine at 0x1002bfd0 on `InputStats` and `OutputStats` between them. `memset` at 0x1002bde8 and 0x1002be00 zeroes both blocks, 0x28 bytes each. The format strings are not in the facts, so the printed fields are unknown.

- `RouterPrintAllStats` in `6.5.12/usr/sbin/flashsc` at 0x1002be20
  **Inferred.** Prints a header and then the statistics of each client. It calls `fprintf` at 0x1002be78 with format 0x10045cc8, then `RouterPrintClientStats` at 0x1002be90 with the argument list from a0 and a1. Its return value is either the first entry of `ClientList` or the return of `RouterPrintClientStats`, which the facts do not resolve.

- `RouterPrintClientStats` in `6.5.12/usr/sbin/flashsc` at 0x1002bed0
  **Inferred.** Prints one client's two statistics blocks and clears them. The blocks sit inside the client record at offsets +244 (0xf4) and +320 (0x140). It calls `fprintf` at 0x1002bf1c with format 0x10045c88, the same format `RouterPrintProcessStats` uses, and at 0x1002bf50 with 0x10045ca8, with an unnamed routine at 0x1002bfd0 on each block between. `memset` at 0x1002bf94 and 0x1002bfb0 zeroes each block afterwards.

- `RouterFreeArguments` in `6.5.12/usr/sbin/flashsc` at 0x1002c5b0
  **Inferred.** Frees an array of argument pointers. It loops (branch at 0x1002c68c back to 0x1002c5e8) over a count taken from a0, calling `free` at 0x1002c66c on each element in a1, and the tests at 0x1002c610, 0x1002c630 and 0x1002c64c skip the free when a0 is not 1 or when a1 or a2 is zero. It returns a0 or an unknown value at 0x1002c6a0. The meaning of the flag is not shown.

- `RouterFreeClient` in `6.5.12/usr/sbin/flashsc` at 0x1002c6b0
  **Inferred.** Releases one client record. It calls `RouterFreeGroups` at 0x1002c6d8, then frees the buffers at [a0+216], [a0+240] and [a0+316] when they are non-zero (branches at 0x1002c6e8, 0x1002c70c and 0x1002c730). The branch at 0x1002c748 skips the second free when [a0+240] and [a0+316] are the same pointer. It calls two unnamed routines at 0x1002c860 on fields +204 and +284 and one at 0x1002c8e0 on field +296, then frees the record itself at 0x1002c7cc.

- `RouterFreeGroups` in `6.5.12/usr/sbin/flashsc` at 0x1002c7f0
  **Inferred.** Frees the group block at [a0+20] when it is non-zero (branch at 0x1002c814), with `free` at 0x1002c828. Its return is either the constant 0x22720 or the return of `free`, which the facts do not resolve. The record it belongs to is not named in the facts.

- `RouterAddTimer` in `6.5.12/usr/sbin/flashsc` at 0x1002cb30
  **Inferred.** Re-arms a timeout for a client. When the handle at [a0+200] is non-zero (branch at 0x1002cb5c) it cancels it with `EventRemoveTimeOut` at 0x1002cb70, then adds a new timeout with `EventAddTimeOut` at 0x1002cb8c and returns its result. The argument order into `EventAddTimeOut` is not clear from the facts, so what the timeout fires on is unknown.

- `RouterRemoveTimer` in `6.5.12/usr/sbin/flashsc` at 0x1002cbb0
  **Inferred.** Cancels the timeout handle at [a0+200] by calling `EventRemoveTimeOut` at 0x1002cbd8. It passes the constant 0x22360 as the second argument and its own a2 and a3 unchanged, and returns the call's result. The meaning of 0x22360 is not shown.

- `RouterLogError` in `6.5.12/usr/sbin/flashsc` at 0x1002cc00
  **Inferred.** Logs an error for a client. It calls `LoggerLogMessage` at 0x1002cc70 with its own a0, a1 and a2 and three values derived from a3: a3+44, the word at [a3+8] and a3+109, plus a pointer passed on the stack at sp-24. It returns the logger's result. The fields of the client record used here are not named in the facts.

- `RouterCheckOutput` in `6.5.12/usr/sbin/flashsc` at 0x1002cc90
  **Inferred.** Checks a condition on the record at a0 and returns 0 or -1. The branch at 0x1002ccbc tests bit 0x4 of the word at [a0+24]. On the path where that bit is clear the facts show `LogError` at 0x1002ccd8 with code -0x6e and a return of -1 at 0x1002ccf0. When the bit is set the routine returns 0 at 0x1002cd08. Which of these two arms is the failure is an inference from the code order.

- `RouterConnectionLost` in `6.5.12/usr/sbin/flashsc` at 0x1002cd10
  **Inferred.** Handles a connection that dropped. It closes the connection with `RouterClose` at 0x1002cd90, passing a4 as 0x80, then reports through `RouterClientError` at 0x1002cdf8 with one of four codes (-0x6b, -0x6a, -0x67 or -0x64). The branches at 0x1002cd40 to 0x1002cd80 choose the code from values that are not in the facts, so which condition gives which code is unknown.

- `RouterConnectionTimeout` in `6.5.12/usr/sbin/flashsc` at 0x1002ce20
  **Inferred.** Handles a connection that timed out, with the same shape as `RouterConnectionLost`. It calls `RouterClose` at 0x1002cea0 with a4 as 0x80, then `RouterClientError` at 0x1002cf08 with one of four codes (-0x6b, -0x6a, -0x68 or -0x65). The codes differ from the lost-connection case, and the facts do not show which one applies to which timeout.

- `RouterConnectionError` in `6.5.12/usr/sbin/flashsc` at 0x1002cf30
  **Inferred.** Reports a connection error without closing the connection. It calls `RouterClientError` at 0x1002cfcc with one of three codes (-0x6c, -0x69 or -0x66) and returns its result. Its callers in the facts include `RouterInput` at 0x10029054 and `RouterBadClass` at 0x1002d26c.

- `RouterClientError` in `6.5.12/usr/sbin/flashsc` at 0x1002cff0
  **Inferred.** Reports an error for one client and then closes it. When [a0+188] is non-zero (branch at 0x1002d030) it makes an indirect call at 0x1002d060 with the record and its fields +44, +8 and +36 as arguments; the callee is not resolved, so the report target is unknown. It then calls `RouterClose` at 0x1002d070 and returns its result.

- `RouterBadClass` in `6.5.12/usr/sbin/flashsc` at 0x1002d090
  **Inferred.** Handles a message whose class or field values are rejected. A compare ladder (branches at 0x1002d0cc to 0x1002d25c) on values that are not in the facts selects one of two `RouterLogError` calls, at 0x1002d22c with code -0x70 or 0x1002d254 with code -0x71. Each path ends by calling `RouterConnectionError` at 0x1002d26c and returning its result. The compare values are unnamed, so which bad field each arm reports is unknown.

- `RouterMemError` in `6.5.12/usr/sbin/flashsc` at 0x1002d290
  **Inferred.** Out-of-memory path. It calls `MemUsage` at 0x1002d2ac, then `RouterShutDownClients` at 0x1002d2b8, then `LogError` at 0x1002d2d4 with code -0x6f. If the pointer at `sym(FatalCB)` is non-zero (branch at 0x1002d2e4) it calls that pointer at 0x1002d2f8, and it calls `abort` at 0x1002d304 last. The callee at 0x1002d2f8 is indirect and unresolved.

- `RouterClose` in `6.5.12/usr/sbin/flashsc` at 0x1002d320
  **Inferred.** A wrapper that closes a client with the flag set to 1. It calls `RouterDoClose` at 0x1002d348 with a1 set to 0x1 and its own a0, a2 and a3 passed through, and returns the result at 0x1002d35c. The meaning of the flag is not shown.

- `RouterDoClose` in `6.5.12/usr/sbin/flashsc` at 0x1002d370
  **Inferred.** Closes a client connection. It calls two unnamed routines at 0x1002ca80 and 0x1002cb00, then `SocketClose` at 0x1002d3f8 on the word at [a0], which the branch at 0x1002d39c skips when that word is not positive. It removes the client with `RouterRemoveClient` at 0x1002d410. The branches at 0x1002d3c0 and 0x1002d3d0 test [a0+304] and [a0+312] before the close, but their role is not shown.

- `BuffCompress` in `6.5.12/usr/sbin/flashsc` at 0x1002d430
  **Inferred.** Compresses a buffer into a block it allocates. Its only callee in the facts is `malloc` at 0x1002d4a0, with the constant 0x21ae0 as the size, and it returns -1 at 0x1002d4cc on the failure path and 0 at 0x1002dbb4. The compression loop itself is not shown in the callee list, so the compressed format is unknown.
