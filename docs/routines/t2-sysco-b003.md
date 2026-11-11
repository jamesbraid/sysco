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

`sn1SCConnCmd` calls `sn1SCConnReq` at 0x10018d50 with its four argument registers passed
through, then tests the return value at 0x10018d64. A nonzero return falls into a priority-3
`syslog` message at 0x10018da4 before the routine reaches `sn1SCConnRsp` at 0x10018de4. The
priority-4 log at 0x10018ec0 and the field test at 0x10018e68 have no named meaning in these
facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10018f00

`sn1SCConnCmdParallel` sizes a local block at four bytes per item (`memcpy` at 0x10018fac,
`memset` at 0x10018fc8, both with `a2 << 2`), then loops over the items. Each pass calls
`sn1SCConnReq` at 0x10019090 and `sn1SCConnRsp` at 0x10019140, and failures are logged through
`syslog` at 0x100191d8 and 0x1001932c. The loop bound comes from the count in `a2`, but the
mapping from each item to its stored result is not resolved.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10019430

`sn1SCUsage` calls `L1DirectUsage` at 0x10019458 and builds one heap string from its result.
It sizes the buffer with `strlen` and `malloc` at 0x1001949c, then fills it with `strcpy` and
two `strcat` calls. A branch at 0x1001944c checks the global at `0x1004750c` first, so the
string looks cached after the first call; the returned pointer is that global's value.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10019600

`SCFmtPkt` formats a packet's two address fields and its class field into a caller buffer.
It loads two pointers from `a4` and `a5`, formats each with `IRFmtAddr` at 0x10019644 and
0x1001965c, formats the class with `IRFmtClas` at 0x1001966c, and finishes with `sprintf` at
0x1001969c. The constant `0x35910` passed in `a7` at 0x10019644 is not identified, and the
format string at `0x10044798` was not read.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x100196c0

`sn1SCParseCmdline` is the command-line parser. It takes the basename of the program name
with `strrchr` at 0x10019700, hands the arguments to `L1DirectParseCmdline` at 0x10019750, and
matches the remaining option words with seven `strcmp` calls. Option values are copied into
fixed buffers at `0x1004e450`, `0x1004e490`, `0x1004e4d0`, `0x1004e510` and `0x1004e5d0`, with
a check against `-` (0x2d) before each copy. Which option sets which global is only partly
visible: `IR_PORT_BIAS` (0x10019cf8) and `ir_show` (0x10019d9c) are stored here, but the
option names are not in the facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10019ea0

`sn1SCSetL2Host` copies its first argument into the L2 host-name buffer at `0x1004e450` with
`strcpy` at 0x10019ecc and returns the copy's result. The other argument registers pass
through unchanged. The buffer is the one that `sn1SCInit` and `sn1SCFindL2` read.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10019ef0

`sn1SCGetL2Host` measures the L2 host-name buffer at `0x1004e450` with `strlen` at 0x10019f14.
It returns the buffer address when the string is nonempty and zero when it is empty. The
`strlen` call also carries `a1 = 0x35020`, a register value the facts do not explain.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10019f60

`sn1SCSetSysName` copies its first argument into the system-name buffer at `0x1004e510` with
`strcpy` at 0x10019f8c and returns the copy's result. The buffer is the one that `sn1SCFindL2`
compares against at 0x1001a770.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x10019fb0

`sn1SCGetSysName` is the reader for the system-name buffer at `0x1004e510`. It calls `strlen`
at 0x10019fd4 and returns the buffer address for a nonempty string, zero otherwise.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a020

`sn1SCSetSSN` copies its first argument into the buffer at `0x1004e4d0` with `strcpy` at
0x1001a04c and returns the copy's result. The buffer is the one that `sn1SCFindL2` compares
at 0x1001a730. The name suggests a serial-number string, but the facts show only the buffer.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a070

`sn1SCGetSSN` is the reader for the buffer at `0x1004e4d0`. It calls `strlen` at 0x1001a094
and returns the buffer address for a nonempty string, zero otherwise.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a0e0

`sn1SCInit` sets up the client side. It reads `IR_PORT_BIAS` at 0x1001a13c and stores the
router callback slots (`IRouterConnect`, `IRouterDisconnect`, `IRouterRequest`,
`IRouterResponse`, `IRouterGetPacket`) in pairs between 0x1001a1c8 and 0x1001a258. It calls
`IRouterInit` at 0x1001a124, `sn1SCClient` at 0x1001a158, `L1DirectConnect` at 0x1001a194,
`sn1SCFindL2` at 0x1001a28c and `sn1SCReconnect` at 0x1001a3f8, and returns `-1`, `-0x64`, or
one of those calls' results. The meaning of the paired stores is not settled: the second slot
set may be a separate OS-side table, but the facts do not name it.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a450

`sn1SCReconnect` calls the `IRouterConnect` slot indirectly at 0x1001a48c, passing the host
buffers at `0x1004e550` and `0x1004e450` and two words from `0x10047500` and `0x10047504`. A
test at 0x1001a4a0 decides whether a priority-3 `syslog` message (line 0x1d9) is written. The
callee behind the indirect slot is not resolved in these facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a520

`sn1SCDisconnect` is a thin wrapper. It loads the `IRouterDisconnect` slot at 0x1001a540,
calls it indirectly at 0x1001a544 with its argument registers passed through, and returns the
call's result.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a570

`sn1SCFindL2` looks for the L2 controller that matches the configured host and system names.
It calls `sn1SCDiscoverL2` at 0x1001a630 with the buffer at `0x1004e490` and a count of 1, then
compares the discovered name with the two configured buffers using `strcmp` at 0x1001a730 and
0x1001a770. On a match it converts the address with `inet_ntoa` at 0x1001a994 and copies the
text with `strcpy`, and the returns are `-1`, `0` and `-2`. The mismatch and match paths are
inferred from the calls, not from a full comparison of the names.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001a9d0

`sn1SCSocket` returns the socket word of the current client. It loads the client pointer from
the global at `0x100474e4` and returns `-1` when that pointer is null; otherwise it returns the
word at the pointer's target. The field offset inside the client structure is not given in the
facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001aa20

`sn1SCHost` returns the address of the host-name buffer at `0x1004e450` when the global client
pointer at `0x100474e4` is set, and zero when it is null. It is a null-checked accessor with no
call.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001aa70

`sn1SCClient` returns the global client pointer and tries to create one when it is null. In
that case it calls `RouterFindClient` at 0x1001aa9c with its arguments, then returns the global
at `0x100474e4`. The lookup result is tested at 0x1001aaa4, but the facts do not show what a
failed lookup leaves in the global.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001aad0

`sn1SCCopy` only reads the word at the global `0x100474f0` and returns it. The name suggests a
copy, but the routine copies nothing, and the global is not named in these facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001aaf0

`sn1SCReq` sends one request. It gets the client through `sn1SCClient` at 0x1001ab6c and, when
the global at `0x100474e0` is zero, logs at priority 3 (0x1001ab44) and calls `exit(1)` at
0x1001ab54. With a client present it calls `L1DirectRequest` at 0x1001abc8 when the global at
`0x100474f4` is set, and otherwise calls the `IRouterRequest` slot indirectly at 0x1001ac68.
When the `ir_show` trace flag is set, it formats the packet with `SCFmtPkt` and prints it with
`fprintf`; the no-client return is `-0x67`. The `exit` guard is inferred from the branch sense
alone.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001ad50

`sn1SCRsp` receives one response. It gets the client through `sn1SCClient` at 0x1001ada8 and
calls `L1DirectGetPacket` at 0x1001adf0 with a count of 1 when the direct transport is in use;
otherwise it calls the `IRouterGetPacket` slot indirectly at 0x1001ae88 with a count of 1. A
trace path formats the packet through `IRFmtPkt` at 0x1001af50 and prints it with `fprintf` at
0x1001af70. It stores one word through its second argument at 0x1001afa8, and the sequence check
is not visible in these facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001afd0

`sn1SCPkt` is close to a copy of `sn1SCRsp`. The facts show two differences: the count passed to
`L1DirectGetPacket` at 0x1001b06c is 0 here and 1 in `sn1SCRsp`, and the indirect get-packet
call is at 0x1001b114. The log line numbers also differ (0x321 here). What the routine does with
the packet after receiving it is not clear, so the role rests on the shared shape.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001b230

`sn1SCSendRsp` sends one response packet. It gets the client through `sn1SCClient` at 0x1001b2a8
and, when the global at `0x100474e0` is zero, logs at priority 3 (line 0x34f) and calls `exit(1)`
at 0x1001b290. Otherwise it calls `L1DirectResponse` at 0x1001b300 for direct transport, or the
`IRouterResponse` slot indirectly at 0x1001b39c. A trace path formats the packet with `SCFmtPkt`
at 0x1001b444 and prints it with `fprintf`; the exit guard is inferred from the same branch sense
as `sn1SCReq`.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001b490

`sn1SCCmd` runs one request and its response. It calls `sn1SCReq` at 0x1001b4d8, then `sn1SCRsp`
at 0x1001b564 with a count of 1, and logs at priority 3 (lines 0x38d and 0x395) and priority 4
(line 0x39c). The response is compared with the request through the values tested at 0x1001b614,
and the routine returns `0` on the completed path. A back-edge at 0x1001b654 returns to the
request step at 0x1001b554, and the facts do not show what bounds it, so the retry behaviour is
unknown.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` at 0x1001b680

`sn1SCCmdParallel` is the batched form of `sn1SCCmd`. It sizes a request block at four bytes per
item with `memcpy` at 0x1001b72c and `memset` at 0x1001b748, then loops over the items: each pass
calls `sn1SCReq` at 0x1001b80c, and the batch ends with `sn1SCRsp` at 0x1001b8b8. Failures are
logged at priority 3 (lines 0x3ea and 0x418). The loop counts come from the item count in `a1`,
and the mapping from each item to its stored result is not resolved.

Grade: inferred.
