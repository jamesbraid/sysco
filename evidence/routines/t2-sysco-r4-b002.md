# Tier-2 routine readings, sysco batch 2

Role statements for targets 50 to 74 of the tier-2 list, written from the rendered instruction
facts only (registers, calls, constants and branch targets). All 25 are inferred. The grader
accepted every rendered entry, so none were rejected.

- `EventRemoveTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x100242a0
  **Inferred.** Appears to remove a node from the timer list whose head word is at 0x10047568. It walks list nodes in a loop and compares a0 against a loaded value (branches at 0x100242e0 and 0x10024310), and the walk ends when a word is zero (0x100242d0, 0x10024300). When a node is found it stores to the head word (sw at 0x10024338) only when the walk has no previous node (the branch at 0x10024328 is not taken); otherwise it stores through the previous node (sw at 0x10024350). It then stores a word loaded at 0x10024354 into the found node (sw at 0x1002435c) and writes 0x1004756c (sw at 0x10024364). The node layout and the return value (the head word or a loaded value, per the return at 0x10024370) are not settled by the facts.
  Reviewed: 2026-11-12
- `event_process` in `6.5.12/usr/sbin/flashsc` at 0x100246e0
  **Inferred.** Role (from name): the core event loop. It calls gettimeofday at 0x10024768 and again at 0x100249f8 (the second only when the word at 0x10047568 is nonzero) and compares the microsecond word at 0x10047574 against 0xf4240 (1,000,000); the facts do not show what the result is used for beyond comparisons against the timer list at 0x10047568. An indirect call at 0x10024b28 passes words loaded from memory, the callee is not identified, and what it does is not shown. It calls select at 0x10024d0c with nfds from 0x10047560, the sets at 0x100553a8 and 0x10055428, a null third set and a timeout from 0x10047580. When select returns -1 it compares errno with 4 (0x10024d3c) and 9 (0x10024d4c). Errno 9 calls the routine at 0x100253d0; any other errno logs -0x130 through LogError (0x10024d94) and returns its result. EventRemoveInput (0x10024f10) and EventRemoveOutput (0x10025034) are reached on the paths where the callback-table entry at 0x1004f2a8 or 0x1004f2b0 is zero, after a LogError with -0x131 or -0x132 (0x10024f00, 0x10025024); they are not the errno 9 handlers. Otherwise it calls through the table entries by indirect call (0x10024edc, 0x10025000). The flag at 0x10047564 is read at 0x10024b64 and 0x10024df8, and a nonzero flag branches to 0x10025090 (0x10024b70, 0x1002506c). Returns exist at 0x10024c20, 0x10024da8 (the LogError result) and 0x100250a0; their values are mostly not identified.
  Reviewed: 2026-11-12
- `EventStart` in `6.5.12/usr/sbin/flashsc` at 0x100250b0
  **Inferred.** Calls event_process at 0x100250e0 with five zero arguments and returns its result. The zeros suggest an open-ended run until stopped, but nothing in this routine states it.
  Reviewed: 2026-11-12
- `EventDispatch` in `6.5.12/usr/sbin/flashsc` at 0x10025100
  **Inferred.** Calls event_process at 0x10025130 with a0 set to 1 and zeros in the other slots, then returns its result. The leading 1 probably selects a single pass rather than the open-ended loop, but the rendered facts do not show how event_process uses it.
  Reviewed: 2026-11-12
- `EventStartSubset` in `6.5.12/usr/sbin/flashsc` at 0x10025150
  **Inferred.** Passes its incoming argument registers into event_process at 0x10025190, with zero in the first slot and its own arguments shifted up one position. It returns the result of event_process. What the shifted arguments select is not shown.
  Reviewed: 2026-11-12
- `EventStop` in `6.5.12/usr/sbin/flashsc` at 0x10025210
  **Inferred.** Stores 1 to the word at 0x10047564 (sw at 0x10025220) and returns 1. event_process reads that word at 0x10024b64 and 0x10024df8 and leaves its loop when it is set, so this is the stop request.
  Reviewed: 2026-11-12
- `SocketExportInetConnection` in `6.5.12/usr/sbin/flashsc` at 0x100257c0
  **Inferred.** Opens a listening IPv4 stream socket. It calls socket with domain 2 (0x100257f4), setsockopt at level 0xffff with option 4 and a 4-byte value (0x10025860), bind on a 16-byte address built at sp-56 (0x10025900), and listen with backlog 5 (0x10025970). Failures log through LogError or LogErrorHost with code -0xc8 and return -1 (0x10025840, 0x100258bc, 0x1002595c, 0x100259c4). The success path returns a value the facts do not identify, and the name's "export" meaning is not shown.
  Reviewed: 2026-11-12
- `SocketGetSockAddr` in `6.5.12/usr/sbin/flashsc` at 0x10026a00
  **Inferred.** Calls getsockname on the descriptor in a0 (0x10026a38). On success it stores one word through the caller's pointer in a1 (sw at 0x10026a8c), and on failure it logs with code -0xd0 (0x10026a64) and returns -1 (0x10026a7c). Success returns 0 (0x10026aa0). Which address field the stored word comes from is unknown.
  Reviewed: 2026-11-12
- `SocketGetName` in `6.5.12/usr/sbin/flashsc` at 0x10026ab0
  **Inferred.** Role (from name): reverse-resolves an IPv4 address to a host name. It zeros the caller's buffer at arg1 with memset (0x10026aec, length arg2), calls gethostbyaddr with a0 = sp-64, length 4 and family 2 (0x10026b04), and on a nonzero result calls strncpy into the buffer from the first word of the returned record, bounded by arg2 minus one (0x10026b4c). The return is -1 when the lookup returns zero (0x10026b2c) and 0 otherwise (0x10026b64). The facts do not show what is stored in the slot at sp-64 that gethostbyaddr reads, so the source of the address is not established by them.
  Reviewed: 2026-11-12
- `RouterInit` in `6.5.12/usr/sbin/flashsc` at 0x10026ed0
  **Inferred.** Sets up the router's process-wide state. It stores incoming arguments into process-wide words (sw at 0x10026f08, 0x10026f20, 0x10026f2c, 0x10026f38 and 0x10026f44); the store at 0x10026f20 runs only when a0 is below 0xa (the branch at 0x10026f10 is not taken). The memset calls that follow name InputStats and OutputStats with length 0x28 (40 bytes, at 0x10026f58 and 0x10026f70). It calls strrchr on the argument at a2 with '/' (0x2f) at 0x10026f84, which looks like taking a program basename. It calls signal with 13 (SIGPIPE on common Unix ABIs) and handler 1 (0x10026fdc). It clears UserName (9 bytes) and FullName (0x21 bytes), then fills them from getpwuid of getuid (0x1002702c), with strncpy of 8 and 0x20 bytes at 0x10027090 and 0x100270ac. It then calls SocketGetHostName (0x100270bc) and SocketGetAddr with IPAddress (0x100270ec). It returns -1 when getpwuid returns zero (0x10027074) and 0 otherwise (0x10027104). Which store feeds which named global is not shown by the facts.
  Reviewed: 2026-11-12
- `RouterClientsConnected` in `6.5.12/usr/sbin/flashsc` at 0x10027110
  **Inferred.** Walks the ClientList head at sym(ClientList). Each record's flags are tested with a1 masked by 0x2 (0x1002714c) and a2 compared with 0x80 (0x10027164), and the loop continues through the link at 0x10027188. The routine returns 0 or a value from inside the loop (0x1002719c), so whether it reports a count or a yes-or-no is not settled by the facts.
  Reviewed: 2026-11-12
- `RouterHostConnected` in `6.5.12/usr/sbin/flashsc` at 0x10027230
  **Inferred.** Scans ClientList for a record that matches a host. For each record it calls strcmp with the record's name at +44 and a0 (0x1002728c), then compares the words at +8 and +12 with a1 and a2 (0x100272b0, 0x100272c8). On a full match, tests of a word against 1 (0x100272e8), 4 (0x100272f8) and 0x40 (0x1002731c) lead to a return of 1 (0x10027334) or 4 (0x10027354); the test results that select each return are not fully shown. A record that does not match advances to the next one, and the return of 0 (0x10027398) is reached when the list ends (the loop branch at 0x10027380 is not taken). What the mode codes mean to the caller is not shown.
  Reviewed: 2026-11-12
- `RouterAddrConnected` in `6.5.12/usr/sbin/flashsc` at 0x100273a0
  **Inferred.** Scans ClientList the same way as RouterHostConnected but matches on address words. The record words at +4, +8 and +12 are compared with a0, a1 and a2 (0x100273f4, 0x1002740c, 0x10027424). On a full match, tests of a word against 1 (0x10027444), 4 (0x10027454) and 0x40 (0x10027478) lead to a return of 1 (0x1002748c) or 4 (0x100274a8); the test results that select each return are not fully shown. A record that does not match advances to the next one, and the return of 0 (0x100274e8) is reached when the list ends (the loop branch at 0x100274d4 is not taken). The meaning of the three words is not shown by the facts.
  Reviewed: 2026-11-12
- `RouterInitClient` in `6.5.12/usr/sbin/flashsc` at 0x100274f0
  **Inferred.** Fills in fixed fields of the client record at a0: a word at +0x20, a half-word at +0x13a, and words at +0xb0, +0xb4, +0xb8 and +0xbc (stores at 0x10027518 through 0x10027554). It returns the record pointer (0x1002755c). The constants written are not named in the facts, so the role of each field is unknown.
  Reviewed: 2026-11-12
- `RouterRequestOut` in `6.5.12/usr/sbin/flashsc` at 0x10027570
  **Inferred.** Calls RouterPriRequestOut at 0x100275c8 with a0 to a3 taken from its own arguments (the facts list no others) and a7 set to 0, and returns that routine's result. Role (from name): the normal-priority form of the request path; the facts do not show what a7 means to the callee.
  Reviewed: 2026-11-12
- `RouterPriRequestOut` in `6.5.12/usr/sbin/flashsc` at 0x100275f0
  **Inferred.** Role (from name): queues one request on the router output path. It calls RouterCheckOutput (0x10027644) and returns -1 on the path where the branch at 0x1002764c is not taken (0x10027664); the operand that branch tests is not fully identified by the facts. Only when a later test of a flag word against 1 (0x10027674) also falls through and the 32-bit word at +292 is not below the 16-bit word at +314 (0x10027690) does it return -2 (0x100276a8). Otherwise the request goes to RouterDoOutput at 0x100276e0 with message type 0x10, and the return is 0 (0x100276f8). The type 0x10 is inferred from the constant.
  Reviewed: 2026-11-12
- `RouterRelRequestOut` in `6.5.12/usr/sbin/flashsc` at 0x10027700
  **Inferred.** Calls RouterPriRelRequestOut at 0x10027768 with a0 to a3 taken from its own arguments (the facts list no others) and returns that routine's result. Role (from name): the release variant of the request path; the facts do not say what release means here.
  Reviewed: 2026-11-12
- `RouterPriRelRequestOut` in `6.5.12/usr/sbin/flashsc` at 0x10027790
  **Inferred.** Queues a request like RouterPriRequestOut, with the same check of the word at +0x124 and the half-word at +0x13a, and the same count-against-limit test at 0x1002782c. Its message type is 0x20 at the RouterDoOutput call (0x10027880), where RouterPriRequestOut uses 0x10. The returns are -1 (0x10027800), -2 (0x10027844) and 0 (0x10027898).
  Reviewed: 2026-11-12
- `RouterPriEventOut` in `6.5.12/usr/sbin/flashsc` at 0x100279f0
  **Inferred.** Sends an event message of type 0x80 through RouterDoOutput at 0x10027a7c. It starts from the first record of ClientList (0x10027a20) and walks the list (loop back at 0x10027aa0), with a flag test masked by 0x4 at 0x10027a44 gating the send. It returns 0 or the last value from the loop. Which client receives the event and why the mask gates it are not shown.
  Reviewed: 2026-11-12
- `RouterDisconnect` in `6.5.12/usr/sbin/flashsc` at 0x100282c0
  **Inferred.** Stores a word at +0x18 of its argument record (sw at 0x100282f8), tests a1 for greater than zero (0x10028300), stores a word at +0x20 (sw at 0x10028318) and calls RouterDoOutput with message type 0x11 (0x1002834c), returning that result. The facts do not show a difference in what the two sides of the a1 test do for the message send, and the values stored into +0x18 and +0x20 are not shown.
  Reviewed: 2026-11-12
- `RouterPing` in `6.5.12/usr/sbin/flashsc` at 0x100284f0
  **Inferred.** Sends a message of type 0x24 through RouterDoOutput (0x1002853c), then flushes the output with RouterFlushOutput (0x10028550) and returns the flush result. The type 0x24 is not named in the facts, so a ping role rests only on the shape of the routine.
  Reviewed: 2026-11-12
- `RouterClientSetBufferSize` in `6.5.12/usr/sbin/flashsc` at 0x10028660
  **Inferred.** Reads the word at +0 of the client record (0x10028688) and the globals SocketSendBufsize and SocketReceiveBufsize (0x10028690, 0x10028698). It calls SocketSetBufferSize at 0x100286a0 with that word as a0, the two sizes as a1 and a2, and the constant 0x268b0 as a3, and returns its result. Whether the word at +0 is a descriptor, and which option the callee sets, rest only on names.
  Reviewed: 2026-11-12
- `RouterFindClient` in `6.5.12/usr/sbin/flashsc` at 0x10028d80
  **Inferred.** Walks ClientList from its head (0x10028d9c) and compares a word in each record with argument a0 (0x10028db8), following the link at 0x10028de4. It returns a0 when a record matches (0x10028dcc) and 0 when the list is exhausted (0x10028df8).
  Reviewed: 2026-11-12
- `RouterListClients` in `6.5.12/usr/sbin/flashsc` at 0x10028e00
  **Inferred.** Walks ClientList and makes one indirect call per record (jalr at 0x10028e40) while the link word after each call is nonzero (0x10028e58). The callee is not identified and the facts show a0 as unknown, so what is passed to it and what the walk does with each record are not shown. The return is either the list head or a value from the callback (0x10028e6c).
  Reviewed: 2026-11-12
- `RouterStartInput` in `6.5.12/usr/sbin/flashsc` at 0x10028e80
  **Inferred.** Allocates an 8-byte block with malloc (0x10028eb8) only when the word at +0xe4 of its argument record is below 8 (0x10028ea8). A failed allocation calls RouterMemError at 0x10028ed8. The new pointer is stored at +0xd8, the words at +0xdc and +0xe0 and a half-word at +0xea are written (0x10028ec8 through 0x10028f0c), and the routine returns the result of the call at 0x10028f1c. The meaning of the stored values is not shown.
  Reviewed: 2026-11-12
