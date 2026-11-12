# Tier-2 routine readings, sysco batch 3

Role statements for ranked targets 75 to 99 of the tier-2 list for sysco. The list ranks
inventory rows at or below the located grade, so some entries are only named or undocumented
in the inventory. Each statement comes from the rendered facts for that routine: registers,
calls, constants and branch structure. Grades are inferred throughout, because the facts show
mechanics and not intent. Names come from the inventory. Where a call target or a constant is
not resolved, the paragraph says so.

All 25 entries are in `6.5.12/usr/sbin/flashsc`, the host-side command client. The rendered
facts show routines that talk to a router connection table (`IRouterConnect`,
`IRouterRequest` and the rest), a small set of global string buffers, and a syslog call
pattern that repeats across the file.

### `6.5.12/usr/sbin/flashsc` at 0x10018d00

`sn1SCConnCmd` calls `sn1SCConnReq` at 0x10018d50 with a0 to a3 at entry passed through (a5 is a stack address, sp-128), then tests the return value at 0x10018d64. A nonzero return reaches `LoggerDefaultMessage` and a priority-3 `syslog` at 0x10018da4, and the routine returns at 0x10018dbc without calling `sn1SCConnRsp`. A zero return branches to 0x10018dc4 and on to the `sn1SCConnRsp` call at 0x10018de4. A nonzero return from that call (tested at 0x10018dfc) likewise logs at priority 3 (0x10018e3c) and returns at 0x10018e54. On the zero path the routine tests a byte at offset 0x12 of a loaded structure at 0x10018e68, and a priority-4 `syslog` at 0x10018ec0 sits behind a further compare at 0x10018e98. Neither has a named meaning in these facts. The final return is 0 at 0x10018ef4.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10018f00

`sn1SCConnCmdParallel` copies `a2 << 2` bytes from `a1` (`memcpy` at 0x10018fac) and zeroes `a2 << 2` bytes (`memset` at 0x10018fc8), four bytes per item. A first loop over the `a2` items (back-edge at 0x10019114) calls `sn1SCConnReq` at 0x10019090 for items that pass tests at 0x10018ff8 to 0x10019048, whose meaning is not resolved. Its return is tested at 0x100190a8 and two word stores follow. When a count tested at 0x10019124 is positive, a second loop (back-edge at 0x10019340) calls `sn1SCConnRsp` at 0x10019140. A nonzero return is compared with `-0x1a4` and `-0x67` (0x10019168, 0x10019188) before `LoggerDefaultMessage` and a priority-3 `syslog` at 0x100191d8. A second priority-3 `syslog` at 0x1001932c sits behind a compare at 0x10019304. The mapping from each item to its stored result is not resolved.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10019430

`sn1SCUsage` returns the global at `0x1004750c`. When that global is zero (the branch at 0x1001944c falls through), it calls `L1DirectUsage` at 0x10019458, sizes a buffer from `strlen` of that result and of the string at `0x10047a00`, and allocates it with `malloc` at 0x1001949c (length sum plus 3). It fills the buffer with `strcpy` from `0x10047a00` and two `strcat` calls, one with the string at `0x10047778` and one with the `L1DirectUsage` result, all through the global's value. When the global is nonzero the branch skips to the return, so the string looks cached after the first call.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10019600

`SCFmtPkt` formats two addresses and a class value into a caller buffer. It loads one word through each of `a4` and `a5`, formats them with `IRFmtAddr` at 0x10019644 and 0x1001965c into stack buffers (sp-144 and sp-112), formats `a1` at entry with `IRFmtClas` at 0x1001966c, and finishes with `sprintf` at 0x1001969c into `a6` at entry, returning `a6`. The constant `0x35910` passed in `a7` at 0x10019644 is not identified, and the format string at `0x10044798` was not read.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x100196c0

`sn1SCParseCmdline` is the command-line parser. It takes the basename of the program name at `0x10047508` with `strrchr` at 0x10019700 and hands the arguments to `L1DirectParseCmdline` at 0x10019750. A nonzero result returns at 0x10019780. A zero result continues into a loop over the remaining arguments, which matches option words with seven `strcmp` calls. Five of the matches copy a value with `strcpy` into fixed buffers at `0x1004e450`, `0x1004e490`, `0x1004e4d0`, `0x1004e510` and `0x1004e5d0`, each after a check against `-` (0x2d) on the value. Two of them (`0x1004e4d0`, `0x1004e510`) then call the routine at 0x10019500 and print a message when it returns negative. One match calls `atoi` at 0x10019c84, range-checks the result in a global at `0x100474f0` and prints it with `IR_PORT_BIAS` at 0x10019d14. Word stores at 0x10019cf8 and 0x10019d9c have unresolved targets, and the option names are not in the facts.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10019ea0

`sn1SCSetL2Host` copies its first argument into the buffer at `0x1004e450` with `strcpy` at 0x10019ecc and returns the copy's result. Role (from name): it sets the L2 host name. The buffer is one `sn1SCInit` writes (0x1001a210, 0x1001a3c8) and passes to `sn1SCFindL2`, which loads its first byte at 0x1001a5b8.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10019ef0

`sn1SCGetL2Host` calls `strlen` on the buffer at `0x1004e450` at 0x10019f14. It returns the buffer address when the string is nonempty and zero when it is empty. Role (from name): it reads the L2 host name. The `strlen` call also carries `a1 = 0x35020`, a register value the facts do not explain.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10019f60

`sn1SCSetSysName` copies its first argument into the buffer at `0x1004e510` with `strcpy` at 0x10019f8c and returns the copy's result. Role (from name): it sets the system name. The buffer is the one `sn1SCFindL2` passes to `strcmp` at 0x1001a770.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x10019fb0

`sn1SCGetSysName` calls `strlen` on the buffer at `0x1004e510` at 0x10019fd4 and returns the buffer address for a nonempty string, zero otherwise. Role (from name): it reads the system name.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a020

`sn1SCSetSSN` copies its first argument into the buffer at `0x1004e4d0` with `strcpy` at
0x1001a04c and returns the copy's result. The buffer is the one that `sn1SCFindL2` compares
at 0x1001a730. The name suggests a serial-number string, but the facts show only the buffer.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a070

`sn1SCGetSSN` is the reader for the buffer at `0x1004e4d0`. It calls `strlen` at 0x1001a094
and returns the buffer address for a nonempty string, zero otherwise.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a0e0

`sn1SCInit` sets up the client side. It calls `IRouterInit` at 0x1001a124 only when the global at `0x100474e0` is zero. It then loads the word at `0x100474f4` (0x1001a13c) and branches on it. When it is zero it calls `sn1SCClient` at 0x1001a158 and returns `-1` at 0x1001a17c if that returns nonzero. When it is nonzero it calls `L1DirectConnect` at 0x1001a194 and returns its result. A test of the word at `0x100474f8` at 0x1001a1b8 selects between two blocks of five word stores (0x1001a1c8 to 0x1001a1f8, and 0x1001a228 to 0x1001a258), followed by a `strcpy` into `0x1004e450` at 0x1001a210 on the first block's path. The facts show `IRouterGetPacket`, `IRouterRequest`, `IRouterResponse`, `IRouterConnect`, `IRouterDisconnect` and their `OS` and `L3` variants as register values, but not which store takes which. The routine then calls `sn1SCFindL2` at 0x1001a28c and returns `-0x64` on its failures (0x1001a38c, 0x1001a3a8). On success it copies strings with `strcpy` and `strncpy` and calls `sn1SCReconnect` at 0x1001a3f8, returning that call's result when nonzero and `0` otherwise (0x1001a444). The meaning of the two store blocks is not settled: the facts do not name either table.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a450

`sn1SCReconnect` loads a word at 0x1001a488 and calls it indirectly at 0x1001a48c with the buffers at `0x1004e550` and `0x1004e450`, two words from `0x10047500` and `0x10047504`, and the address `0x100474e4`. When the result is nonzero (the test at 0x1001a4a0 falls through) it calls `LoggerDefaultMessage` and writes a priority-3 `syslog` message (line 0x1d9). It returns the indirect call's result. The callee behind the indirect slot is not resolved in these facts, and the facts do not tie it to `IRouterConnect`.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a520

`sn1SCDisconnect` is a thin wrapper. It loads a word at 0x1001a540 and calls it indirectly at 0x1001a544 with `a0` set to the global at `0x100474e4` and `a1` to `a3` passed through from entry, and returns the call's result. The callee behind the indirect slot is not resolved in these facts.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a570

`sn1SCFindL2` matches a discovered L2 against two configured name buffers. When the word at `0x100474f4` is nonzero it returns `-1` at 0x1001a5ac. Otherwise, when the first byte of the buffer at `0x1004e450` is nonzero it can copy that buffer with `strcpy` at 0x1001a5f4 and return `0` (0x1001a60c). Failing that it calls `sn1SCDiscoverL2` at 0x1001a630 with the buffer at `0x1004e490` and a count of 1 (`a4 = 1`), and logs at priority 3 and returns `-1` on failure. It then compares against the buffers at `0x1004e4d0` and `0x1004e510` using `strcmp` at 0x1001a730 and 0x1001a770 when their first bytes are nonzero. Priority-3 `syslog` calls at 0x1001a874, 0x1001a8bc, 0x1001a91c and 0x1001a960 precede the `-2` returns (0x1001a8d4, 0x1001a978). On a match it converts an address with `inet_ntoa` at 0x1001a994 and copies the text with `strcpy` into `a0` at entry, returning `0` (0x1001a9c4). The mismatch and match paths are inferred from the calls, not from a full comparison of the names.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001a9d0

`sn1SCSocket` loads the global at `0x100474e4` and returns `-1` when it is null. Otherwise it loads the word at offset 0 of the structure that pointer addresses and returns it. Role (from name): the returned word is a socket. The facts show only the load.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001aa20

`sn1SCHost` returns the address `0x1004e450` when the global at `0x100474e4` is nonzero and zero when it is null. It is a null-checked accessor with no call. Role (from name): `0x1004e450` is the host name buffer.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001aa70

`sn1SCClient` returns the global at `0x100474e4`. When that global is nonzero (the branch at 0x1001aa8c falls through) it calls `RouterFindClient` at 0x1001aa9c with the global as `a0` and `a1` to `a3` from entry, and tests the result at 0x1001aaa4. When the global is zero the call is skipped. The facts list no store to the global, so they do not show a failed lookup changing it. The routine returns the global's value at 0x1001aac0, which is zero when the global was zero.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001aad0

`sn1SCCopy` only reads the word at the global `0x100474f0` and returns it. The name suggests a
copy, but the routine copies nothing, and the global is not named in these facts.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001aaf0

`sn1SCReq` sends one request. When the global at `0x100474e0` is zero it logs at priority 3 (0x1001ab44) and calls `exit(1)` at 0x1001ab54. When the word at `0x100474f4` is zero it calls `sn1SCClient` at 0x1001ab6c and returns `-0x67` if that returns zero. When `0x100474f4` is nonzero it calls `L1DirectRequest` at 0x1001abc8 and, on a nonzero result, logs at priority 3 and returns that result. When it is zero it calls a function loaded at 0x1001ac64 indirectly at 0x1001ac68, which the facts do not resolve and does not name `IRouterRequest`, and logs on a nonzero result. When the `ir_show` flag is nonzero it formats the packet with `SCFmtPkt` (0x1001ad0c) and prints it with `fprintf`. The routine returns `0` at 0x1001ad44 on that path. The `exit` guard is inferred from the branch sense alone.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001ad50

`sn1SCRsp` receives one response. When the global at `0x100474e0` is zero it returns `-1`. When the word at `0x100474f4` is zero it calls `sn1SCClient` at 0x1001ada8 and returns `-0x67` if that returns zero. When `0x100474f4` is nonzero it calls `L1DirectGetPacket` at 0x1001adf0 with `a2 = 1` and logs and returns the result when it is nonzero. When it is zero it calls a function loaded at 0x1001ae84 indirectly at 0x1001ae88 with `a4 = 1`, which the facts do not resolve, and on a nonzero result logs and may call `RouterFindClient` at 0x1001aef8. When the `ir_show` flag is nonzero a trace path formats the packet through `IRFmtPkt` at 0x1001af50 and prints it with `fprintf` at 0x1001af70. A word store at 0x1001afa8 is skipped when `a1` is zero at 0x1001af7c, and its target is not resolved. The sequence check is not visible in these facts.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001afd0

`sn1SCPkt` is close to a copy of `sn1SCRsp`. The differences the facts show include: the count passed to `L1DirectGetPacket` at 0x1001b06c is 0 here and 1 in `sn1SCRsp`, its `a1` is `a1` at entry where `sn1SCRsp` passes `a2`, the indirect call at 0x1001b114 passes `a4 = 0` where `sn1SCRsp` passes 1, and this routine has no trailing store. The log line numbers also differ (0x321 here), and it compares the `L1DirectGetPacket` result with `-0x1a4` at 0x1001b094. What the routine does with the packet after receiving it is not clear, so the role rests on the shared shape.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001b230

`sn1SCSendRsp` sends one response packet. When the global at `0x100474e0` is zero it logs at priority 3 (line 0x34f) and calls `exit(1)` at 0x1001b290. When the word at `0x100474f4` is zero it calls `sn1SCClient` at 0x1001b2a8 and returns `-0x67` if that returns zero. When `0x100474f4` is nonzero it calls `L1DirectResponse` at 0x1001b300, and logs and returns its result when nonzero. When it is zero it calls a function loaded at 0x1001b398 indirectly at 0x1001b39c, which the facts do not resolve and do not name `IRouterResponse`. When the `ir_show` flag is nonzero a trace path formats the packet with `SCFmtPkt` at 0x1001b444 and prints it with `fprintf`. The exit guard is inferred from the same branch sense as `sn1SCReq`.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001b490

`sn1SCCmd` runs one request and its response. It calls `sn1SCReq` at 0x1001b4d8. A nonzero return logs at priority 3 (line 0x38d) and returns at 0x1001b540 without reaching `sn1SCRsp`. A zero return goes on to `sn1SCRsp` at 0x1001b564, whose nonzero return logs at priority 3 (line 0x395) and returns at 0x1001b5d0. On the zero path it loads a byte at offset 0x12 of a structure at 0x1001b5e0, compares two values the facts leave unresolved at 0x1001b614, and may log at priority 4 (line 0x39c) at 0x1001b638. The routine returns `0` at 0x1001b66c. The facts do not show a loop or retry in this routine.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)

### `6.5.12/usr/sbin/flashsc` at 0x1001b680

`sn1SCCmdParallel` handles a batch of items. Role (from name): it is the batched form of `sn1SCCmd`. It copies `a1 << 2` bytes from `a0` (`memcpy` at 0x1001b72c) and zeroes `a1 << 2` bytes (`memset` at 0x1001b748), four bytes per item. A first loop over the `a1` items (back-edge at 0x1001b890) calls `sn1SCReq` at 0x1001b80c for items that pass tests at 0x1001b778 to 0x1001b7c8, whose meaning is not resolved. When a count tested at 0x1001b8a0 is positive, a second loop (back-edge at 0x1001bab0) calls `sn1SCRsp` at 0x1001b8b8. A nonzero return, compared with `-0x1a4` and `-0x67`, leads to `LoggerDefaultMessage` and a priority-3 `syslog` (line 0x3ea). A further priority-3 log (line 0x418) sits behind a compare at 0x1001ba78. The mapping from each item to its stored result is not resolved.

Grade: inferred.

Reviewed: 2026-11-12 (sonnet)
