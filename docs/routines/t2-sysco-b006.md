# Routine role statements: flashsc packet router and event loop

These entries give a short role statement for each routine, taken from its rendered facts (register use, calls, constants, branch shape) in the host tool `6.5.12/usr/sbin/flashsc`. They describe what the code appears to do in software, not hardware behavior. Every role is inferred from partial facts; none has been checked against a full listing, so each entry ends "Unverified." and counts at most as located. Names of callees are the names the facts give, and `unknown` marks a role the facts leave open.

- `IRFmtPkt` in `6.5.12/usr/sbin/flashsc` at 0x10022060.
  It reads a set of small fields from the record in a1 (offsets 0x13 to 0x21), then calls IRFmtAddr twice (at 0x10022094 and 0x100220ac) and IRFmtClas once (at 0x100220c0) to turn address and class fields into text. It finishes with a sprintf at 0x10022114 that takes its format string from 0x100458c8 and returns the destination pointer from a2. The format string and the meaning of each field are unknown.
  Grade: inferred. Unverified.

- `IRFmtError` in `6.5.12/usr/sbin/flashsc` at 0x10022140.
  When a0 is nonzero it calls LoggerDefaultMessage at 0x10022178, passing its own arguments through. When a0 is zero it returns the fixed address 0x100458f0 instead. The code from 0x100221b0 to 0x100221c0 is not reached from the entry, so its purpose is unknown.
  Grade: inferred. Unverified.

- `IRouterConnectOS` in `6.5.12/usr/sbin/flashsc` at 0x100221d0.
  It calls open with flags 0x2 and stores the result into the pointer in a3. If open returns -1, it writes a syslog message at 0x1002224c (priority 3, text at 0x10045900, length 0x6a, errno passed in a6) and calls exit with -1. On success it returns 0.
  Grade: inferred. Unverified.

- `IRouterDisconnectOS` in `6.5.12/usr/sbin/flashsc` at 0x10022290.
  It calls close on the descriptor held in the global at 0x10047a10 and returns close's result. The routine is short and has no other branches.
  Grade: inferred. Unverified.

- `IRouterRequestOS` in `6.5.12/usr/sbin/flashsc` at 0x100222d0.
  It reads a 16-bit field at a1+6 and stores a word through the pointer in a6. It builds a packet with IRouterMakePacket (at 0x10022368) and, if that succeeds and the descriptor in 0x10047a10 is valid, writes it with IRouterWritePacket (at 0x100223b4) using the buffer at 0x10022830. Error returns are -0x67 when the descriptor is invalid and -0x194 when the packet build fails.
  Grade: inferred. Unverified.

- `IRouterResponseOS` in `6.5.12/usr/sbin/flashsc` at 0x100223e0.
  This is the reply counterpart of IRouterRequestOS. It reads fields from the record in a3 (offsets 0x13, 0x14, 0x21 and a halfword at 0x22), builds a packet with IRouterMakePacket at 0x10022460, and writes it with IRouterWritePacket at 0x10022518 using a6 = 0x64 as its final argument. The meaning of that 0x64 argument is unknown; it may be a timeout or a retry count.
  Grade: inferred. Unverified.

- `IRouterGetPacketOS` in `6.5.12/usr/sbin/flashsc` at 0x10022540.
  It clears a 0x80-byte local set with memset, waits with select at 0x10022680 on the descriptor in 0x10047a10 plus one, then reads one packet with IRouterReadPacket (at 0x10022734) into the buffer at 0x10022a40. It frees the packet with IRouterFreePacket at 0x10022794 and returns either the read result or 0. Several error returns (-0x67, -0x19f, -0x1a0) sit on the select and read paths; the code after 0x100227c8 is not reached from the entry.
  Grade: inferred. Unverified.

- `IRouterForwardPacket` in `6.5.12/usr/sbin/flashsc` at 0x10022dd0.
  It checks a0+8 against the global IR_PORT_BIAS plus 9001, and checks a byte at a2+22 against 2. When the checks pass it mallocs a small record (0xa or 0x14 bytes) and sets byte order with IRouterSetPacketByteOrder at 0x10023004 and 0x1002317c, then calls RouterPriClientEventOut at 0x10023134 and frees the record. It also frees the packet with IRouterFreePacket at 0x100231b8 and flushes output with RouterFlushOutput at 0x100231dc. Returns -0x194 on malloc failure and 0 otherwise. What the port bias means is unknown.
  Grade: inferred. Unverified.

- `IRouterPacketToRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023200.
  It passes all four of its arguments unchanged to IRouterForwardPacket (call at 0x10023234) and returns that result. It adds no logic of its own.
  Grade: inferred. Unverified.

- `IRouterPacketFromRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023250.
  It compares a string at a1+4 against a literal at 0x100478b0 with strcmp (at 0x10023280), and the result selects one of two paths. It then loops over a0-1 entries, loading words from pointers and storing fields, and calls IRouterSetPacketByteOrder at 0x100232f4 with the global IRByteOrder. The literal's meaning and the record layout are unknown.
  Grade: inferred. Unverified.

- `EventStats` in `6.5.12/usr/sbin/flashsc` at 0x10023410.
  It reads the four event counters EventInputCBCount, EventOutputCBCount, EventTimerCBCount and EventSelectCount, and formats them with sprintf at 0x10023460 into a static buffer at 0x100554a8. The four stores at 0x1002347c to 0x10023498 write those counters back, but the facts do not show what values they store. It returns the buffer address on one path.
  Grade: inferred. Unverified.

- `EventAddInput` in `6.5.12/usr/sbin/flashsc` at 0x100234c0.
  It checks a0 against a limit of 0x400 (returning -0x12c through LogError at 0x10023514 when it is out of range) and then works on a table at 0x1004f2a8 indexed by the descriptor number. The branch at 0x10023554 tests whether the slot is empty; the other path logs -0x12d at 0x10023570, which suggests a duplicate registration is refused. It returns the index or 0x400 on one path, and the table at 0x100552a8 is updated in the same routine.
  Grade: inferred. Unverified.

- `EventReplaceInput` in `6.5.12/usr/sbin/flashsc` at 0x10023680.
  It works on the same descriptor table as EventAddInput (0x1004f2a8 and 0x1004f2ac, with the shadow table at 0x100552a8). It checks the index against a limit at 0x100236c0 and logs -0x12c through LogError at 0x100236dc when it is out of range. It reads the two slots and stores those previous values through a3 and a4, then writes the slots at 0x1002376c and 0x10023790 and the shadow entry at 0x100552a8. The values written to the slots and the return value are not shown by the facts.
  Grade: inferred. Unverified.

- `EventReplaceInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023850.
  It saves the current slot values at 0x1004f2a8 and 0x1004f2ac into a second pair of table entries at 0x1004f2b8 and 0x1004f2bc, then writes the replacement values into the live slots. The index range check and the -0x12c error use the same pattern as EventReplaceInput. The purpose of the temporary pair is inferred from its name only.
  Grade: inferred. Unverified.

- `EventRestoreInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023a30.
  It is the reverse of EventReplaceInputTemp: it copies the saved pair at 0x1004f2b8 back into the live slots at 0x1004f2a8 and 0x1004f2ac. It has the same index check and -0x12c error. Whether it also clears the saved pair is not shown by the facts.
  Grade: inferred. Unverified.

- `EventRemoveInput` in `6.5.12/usr/sbin/flashsc` at 0x10023be0.
  It stores to the input-table slots at 0x1004f2a8 and 0x1004f2ac and updates the entries at 0x100552a8 and 0x100553a8; the stored values are not shown in the facts, so whether these are clears is unknown. When a0 is at least 0x400 it calls an unnamed callee at 0x10023cf0 (target 0x10025230) with the table base 0x1004f2a8. Otherwise it returns the constant 0x2b330 after the in-range work. The callee's role is unknown.
  Grade: inferred. Unverified.

- `EventAddOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023d10.
  It is the output-side counterpart of EventAddInput, using the table at 0x1004f2b0 and 0x1004f2b4 with the shadow table at 0x10055328. It checks the index against 0x400 and logs -0x12c at 0x10023d64 when it is out of range, and it logs -0x12e at 0x10023dc0 on the duplicate path. It returns the index or 0x400 on one path.
  Grade: inferred. Unverified.

- `EventRemoveOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023ed0.
  It stores to the output-table slots at 0x1004f2b0 and 0x1004f2b4 and updates the entries at 0x10055328 and 0x10055428, with stored values not shown in the facts. Like EventRemoveInput, it calls the unnamed callee at 0x10025230 (call at 0x10023fe0) when the index is at least 0x400, and otherwise returns the constant 0x2b040. Its role is inferred from the input-side routine only.
  Grade: inferred. Unverified.

- `EventAddTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024000.
  It reads a global at 0x1004756c and, when that value is nonzero, mallocs a 0x14-byte record. If malloc fails it logs -0x12f through LogError at 0x10024090. Otherwise it fills the record with EventCreateTimeOut at 0x100240c8, then calls an unnamed callee at 0x100240dc.
  Grade: inferred. Unverified.

- `EventCreateTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024100.
  It stores five words into the record at a0 (offsets 0x0, 0x4, 0x8, 0xc and 0x10) and reads the current time with gettimeofday at 0x10024144 into a local. It normalizes microseconds against 0xf4240 (1,000,000), which fits an absolute deadline built from the current time plus an interval. The facts do not show which stored word is the deadline, so the exact layout is unknown.
  Grade: inferred. Unverified.

- `EventRemoveTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x100242a0.
  It walks a list whose head is the global at 0x10047568, comparing each node with the argument in a0. On a match it unlinks the node and writes the list count at 0x1004756c. The walk stops when the pointer is null or the match is found; the matched node's free is not shown in the facts.
  Grade: inferred. Unverified.

- `event_process` in `6.5.12/usr/sbin/flashsc` at 0x100246e0.
  This is the main event loop. It reads the time with gettimeofday at 0x10024768 and 0x100249f8, runs the timer list from 0x10047568 and calls callbacks through unknown callees at 0x100248a4 and 0x10024b28. It calls select at 0x10024d0c on the input and output sets at 0x100553a8 and 0x10055428, then dispatches ready descriptors through the callee at 0x10024edc or 0x10025000. On select failure it logs -0x130 at 0x10024d94, and it compares errno with 4 and 9 at 0x10024d3c and 0x10024d4c, which look like EINTR and EBADF. It also removes a failing input or output with EventRemoveInput or EventRemoveOutput at 0x10024f10 and 0x10025034. Returns 0x0 or an unknown value on the end paths.
  Grade: inferred. Unverified.

- `EventStart` in `6.5.12/usr/sbin/flashsc` at 0x100250b0.
  It calls event_process at 0x100250e0 with all five arguments set to zero and returns its result. It is a plain entry point into the loop with no flags.
  Grade: inferred. Unverified.

- `EventDispatch` in `6.5.12/usr/sbin/flashsc` at 0x10025100.
  It calls event_process at 0x10025130 with a0 = 1 and all other arguments zero, then returns the result. The meaning of the 1 is unknown; it may select a dispatch-only pass.
  Grade: inferred. Unverified.

- `EventStartSubset` in `6.5.12/usr/sbin/flashsc` at 0x10025150.
  It calls event_process at 0x10025190 with a zero in a0 and its own four arguments shifted up by one (a1 = arg0, a2 = arg1, a3 = arg2, a4 = arg3). It returns the loop's result. The facts do not say what the leading zero means to event_process.
  Grade: inferred. Unverified.
