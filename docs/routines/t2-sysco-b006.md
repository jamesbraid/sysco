# Routine role statements: flashsc packet router and event loop

These entries give a short role statement for each routine, taken from its rendered facts (register use, calls, constants, branch shape) in the host tool `6.5.12/usr/sbin/flashsc`. They describe what the code appears to do in software, not hardware behavior. Every role is inferred from partial facts; none has been checked against a full listing, so each entry ends "Unverified." and counts at most as located. Names of callees are the names the facts give, and `unknown` marks a role the facts leave open.

- `IRFmtPkt` in `6.5.12/usr/sbin/flashsc` at 0x10022060.
  It reads a set of small fields from the record in a1 (offsets 0x13 to 0x21), then calls IRFmtAddr twice (at 0x10022094 and 0x100220ac) and IRFmtClas once (at 0x100220c0) to turn address and class fields into text. It finishes with a sprintf at 0x10022114 that takes its format string from 0x100458c8 and returns the destination pointer from a2. The format string and the meaning of each field are unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRFmtError` in `6.5.12/usr/sbin/flashsc` at 0x10022140.
  When a0 is nonzero it calls LoggerDefaultMessage at 0x10022178 with a1 = its a0, a2 = its a2 and a3 = its a3 (the
  facts do not show a0 of the call) and returns that call's result. When a0 is zero it returns the fixed address
  0x100458f0.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterConnectOS` in `6.5.12/usr/sbin/flashsc` at 0x100221d0.
  It calls open at 0x10022208 with a0 = its a1, a1 = 0x2 and a2 = its a2. When open returns -1 it calls syslog at
  0x1002224c with 0x3, 0x10045900 and 0x6a in the first three slots and errno in a6, then calls exit with -1.
  Otherwise it stores a word at +0x0 of a pointer (0x10022270; the facts do not show which pointer or value) and
  returns 0.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterDisconnectOS` in `6.5.12/usr/sbin/flashsc` at 0x10022290.
  It calls close on the descriptor held in the global at 0x10047a10 and returns close's result. The routine is short and has no other branches.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterRequestOS` in `6.5.12/usr/sbin/flashsc` at 0x100222d0.
  It first compares the global at 0x10047a10 with -1 and returns -0x67 when they are equal. When a6 is nonzero it
  loads a halfword at +0x6 and stores a word at +0x0 (the facts do not show the bases or the stored value). It then
  calls IRouterMakePacket at 0x10022368 with a0 = its a1, a1 = its a2, a2 = its a3 and a3 = 0. When that returns zero
  the routine returns -0x194. Otherwise it calls IRouterWritePacket at 0x100223b4 with a0 = its a1, a1 = 0x10022830,
  a2 = the global at 0x10047a10 and a3 = the packet result, and returns that call's result.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterResponseOS` in `6.5.12/usr/sbin/flashsc` at 0x100223e0.
  Role (from name): the reply side of IRouterRequestOS. It first compares the global at 0x10047a10 with -1 and returns
  -0x67 when they are equal. It loads a halfword at +0x22 and calls IRouterMakePacket at 0x10022460 with a0 = its a1,
  a1 = its a3 + 28, a2 = the halfword at +0x22 of its a3 and a3 = 1; a zero result returns -0x194. It then does byte
  stores at +0x13, +0x14 and +0x21 (with byte loads at +0x14 and +0x21 between them; the bases are not shown) and
  calls IRouterWritePacket at 0x10022518 with a0 = its a1, a1 = 0x10022830, a2 = the global at 0x10047a10, a3 to a5 =
  the packet result and a6 = 0x64. It returns that call's result. The meaning of the 0x64 argument is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterGetPacketOS` in `6.5.12/usr/sbin/flashsc` at 0x10022540.
  It returns -0x67 when the global at 0x10047a10 equals -1. It clears 0x80 bytes of stack with memset at 0x100225ec,
  then calls select at 0x10022680 with a0 = that global + 1 and a1 = the cleared buffer. A negative result returns
  -0x19f, and a masked test after it (the operands are not shown) can return -0x1a0. It then calls IRouterReadPacket
  at 0x10022734 with a1 = 0x10022a40 and a2 = the global at 0x10047a10; a nonzero result is returned directly.
  Otherwise it calls IRouterFreePacket at 0x10022794 only when one value is nonzero and a byte at +0x16 of a loaded
  record is not 1 (branches at 0x1002276c and 0x10022784), and returns 0.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterForwardPacket` in `6.5.12/usr/sbin/flashsc` at 0x10022dd0.
  It compares the word at a0+8 with the global IR_PORT_BIAS plus 9001 (the branch at 0x10022e0c skips ahead when they
  differ) and a byte at +0x16 of a2 with 2 (branches at 0x10022e20 and 0x10022ee4). It calls malloc at 0x10022f2c,
  which both paths appear to reach; the facts show only 0xa as a candidate in the third argument slot, so the
  allocation size is not shown. A zero result returns -0x194. It then calls IRouterSetPacketByteOrder at 0x10023004
  (a1 = 1), RouterPriClientEventOut at 0x10023134 (a0 = its a0, a1 = 0, a3 = the malloc result) and free on the malloc
  result at 0x10023144. It calls IRouterSetPacketByteOrder again at 0x1002317c (a1 = the global IRByteOrder) and
  IRouterFreePacket at 0x100231b8 on its a2. It calls RouterFlushOutput at 0x100231dc with a1 = 5 only when the global
  IRouterEventLoop is zero, and returns 0. What the port bias means is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterPacketToRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023200.
  It passes all four of its arguments unchanged to IRouterForwardPacket (call at 0x10023234) and returns that result. It adds no logic of its own.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `IRouterPacketFromRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023250.
  It calls strcmp at 0x10023280 with a0 = the literal at 0x100478b0 and a1 = the pointer loaded from a1+4. A nonzero
  result returns 0 (0x100232a4). On zero, it runs a loop over a0 - 1 entries (skipped when that is not positive),
  loading words and storing words and a halfword (bases not shown). Each pass calls IRouterSetPacketByteOrder at
  0x100232f4 with a1 = the global IRByteOrder. The return is 0 or an unknown value. The literal's meaning and the
  record layout are unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventStats` in `6.5.12/usr/sbin/flashsc` at 0x10023410.
  It loads the four event counters EventInputCBCount, EventOutputCBCount, EventTimerCBCount and EventSelectCount and
  calls sprintf at 0x10023460 with a0 = 0x100554a8, the format at 0x10045a10 and the four values. When its a0 is
  nonzero it then does four word stores at 0x1002347c to 0x10023498; the facts show neither their targets nor their
  values. It returns 0x100554a8.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventAddInput` in `6.5.12/usr/sbin/flashsc` at 0x100234c0.
  It compares a0 with 0x401 (signed). Out of range, it calls LogError at 0x10023514 with -0x12c and returns 0x400. In
  range, it loads a word at the entry for 0x1004f2a8 and tests it: when it is zero the branch at 0x10023554 skips the
  LogError call with -0x12d at 0x10023570, so a nonzero entry is what leads to that log. The facts show later stores
  (0x10023598, 0x100235bc, 0x10023638) without their targets, and a compare of a0 with the global at 0x10047560 before
  it returns a0. Calling it a duplicate registration is a guess from the log call.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventReplaceInput` in `6.5.12/usr/sbin/flashsc` at 0x10023680.
  It compares a0 with 0x401 (signed) and, out of range, calls LogError at 0x100236dc with -0x12c. In range it loads a
  word and stores it (0x1002370c, 0x10023714) only when a3 is nonzero, and loads and stores another (0x10023740,
  0x10023748) only when a4 is nonzero; the bases are not shown. It then has two further stores (0x1002376c,
  0x10023790), a load and store (0x100237b8, 0x1002380c), and a compare of a0 with the global at 0x10047560. The
  tables it touches are not shown by the facts. The return value is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventReplaceInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023850.
  Role (from name): a temporary replacement of an input slot. It compares a0 with 0x401 (signed) and, out of range,
  calls LogError at 0x100238a4 with -0x12c. In range it does two load-and-store pairs (0x100238c8 to 0x10023928), two
  further stores (0x1002394c, 0x10023970), a load and store (0x10023998, 0x100239ec) and a compare of a0 with the
  global at 0x10047560. The facts show no table addresses or stored values, so which slots it saves or writes is not
  established. The return value is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventRestoreInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023a30.
  Role (from name): undoes EventReplaceInputTemp. It compares a0 with 0x401 (signed) and, out of range, calls LogError
  at 0x10023a7c with -0x12c. In range it loads the entry for 0x1004f2b8 and tests it for zero (branch at 0x10023aa4).
  It then does two load-and-store pairs (0x10023ac8 to 0x10023b28) and a load and store (0x10023b50, 0x10023ba4), and
  compares a0 with the global at 0x10047560. The facts show no store targets or values, so whether it copies a saved
  pair back or clears one is not established. The return value is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventRemoveInput` in `6.5.12/usr/sbin/flashsc` at 0x10023be0.
  It compares a0 with 0x400 (unsigned). When a0 is below 0x400 it falls through to stores and loads at 0x10023c28 to
  0x10023ce0 (targets and values not shown) and then calls an unnamed callee at 0x10023cf0 (target 0x10025230) with a0
  = its a0, a1 = 0x1004f2a8, a2 = its a2 and a3 = its a3. When a0 is 0x400 or more, the branch at 0x10023c04 jumps to
  0x10023cf8, past that call. It returns one of the constant 0x2b330 and the callee's result; the facts do not say
  which path gives which. The callee's role is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventAddOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023d10.
  Role (from name): the output-side counterpart of EventAddInput. It compares a0 with 0x401 (signed). Out of range, it
  calls LogError at 0x10023d64 with -0x12c and returns 0x400. In range, it loads the entry for 0x1004f2b0 and tests
  it: when it is zero the branch at 0x10023da4 skips the LogError call with -0x12e at 0x10023dc0, so a nonzero entry
  leads to that log. Later stores (0x10023de8, 0x10023e0c, 0x10023e88) have targets the facts do not show. It compares
  a0 with the global at 0x10047560 and returns a0.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventRemoveOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023ed0.
  Role (from name): the output-side counterpart of EventRemoveInput. It compares a0 with 0x400 (unsigned). When a0 is
  below 0x400 it falls through to stores and loads at 0x10023f18 to 0x10023fd0 (targets and values not shown) and then
  calls an unnamed callee at 0x10023fe0 (target 0x10025230) with a0 = its a0, a1 = 0x1004f2a8 (the same table base as
  the input side), a2 = its a2 and a3 = its a3. When a0 is 0x400 or more, the branch at 0x10023ef4 jumps to
  0x10023fe8, past that call. It returns one of the constant 0x2b040 and the callee's result; the facts do not say
  which path gives which.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventAddTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024000.
  It loads the global at 0x1004756c; the branch at 0x10024038 skips a load at +0x10 (0x1002404c) when it is zero, and
  both paths then reach malloc at 0x10024064 with a0 = 0x14. When malloc returns zero it calls LogError at 0x10024090
  with -0x12f and returns 0. Otherwise it calls EventCreateTimeOut at 0x100240c8 with a1 = its a0, a2 = its a1, a3 =
  its a2 and a4 = its a3, then calls an unnamed callee at 0x100240dc (target 0x10025300). The return value on that
  path is unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventCreateTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024100.
  It calls gettimeofday at 0x10024144 into a local buffer, then does word stores at +0x0 and +0x4 (0x1002415c,
  0x10024170) whose bases are not shown, and normalizes a microsecond word at +0x4 against 0xf4240 (1,000,000) and
  zero in two loops. At the end it stores words at +0x8, +0xc and +0x10 (0x10024264, 0x10024270, 0x1002427c); the
  facts do not show the base of any of the five stores. It returns 0. The normalization fits an absolute deadline
  built from the current time plus an interval, but which word is the deadline is not shown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventRemoveTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x100242a0.
  It loads the list head at the global 0x10047568 and walks nodes, comparing each with its a0; the loop ends at a null
  pointer or a match (branches at 0x100242d0 to 0x10024310). After the walk it branches on two values (0x1002431c,
  0x10024328) and does stores at 0x10024338 to 0x10024364 whose targets and values the facts do not show, so the
  unlink and any update of the count at 0x1004756c are not established. No free is shown. It returns one of the head
  global and an unknown value.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `event_process` in `6.5.12/usr/sbin/flashsc` at 0x100246e0.
  It reads the time with gettimeofday at 0x10024768 and 0x100249f8 into 0x10047570, and walks the timer list at
  0x10047568, calling unknown callees at 0x100248a4 and 0x10024b28 with the incremented EventTimerCBCount as the
  second argument. It calls an unnamed routine at 0x10024380 twice (0x10024c48, 0x10024c6c) with the pairs
  0x100553a8/0x100552a8 and 0x10055428/0x10055328, then calls select at 0x10024d0c with a1 = 0x100553a8 and a2 =
  0x10055428. When select returns -1 it compares errno with 4 and 9, calls an unnamed routine at 0x10024d6c
  (0x100253d0), then LogError with -0x130 at 0x10024d94 and returns the LogError result; which errno value reaches the
  log is not worked out here. After a successful select it tests the ready sets: when the entry for 0x1004f2a8 is
  nonzero it calls an unknown callee at 0x10024edc with EventInputCBCount incremented, and when it is zero it logs
  -0x131 at 0x10024f00 and calls EventRemoveInput at 0x10024f10. The output side is the same with the entry for
  0x1004f2b0, the callee at 0x10025000, LogError -0x132 and EventRemoveOutput at 0x10025034. Other returns are
  unknown.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventStart` in `6.5.12/usr/sbin/flashsc` at 0x100250b0.
  It calls event_process at 0x100250e0 with a0 to a4 all zero and returns its result.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventDispatch` in `6.5.12/usr/sbin/flashsc` at 0x10025100.
  It calls event_process at 0x10025130 with a0 = 1 and all other arguments zero, then returns the result. The meaning of the 1 is unknown; it may select a dispatch-only pass.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)

- `EventStartSubset` in `6.5.12/usr/sbin/flashsc` at 0x10025150.
  It calls event_process at 0x10025190 with a zero in a0 and its own four arguments shifted up by one (a1 = arg0, a2 = arg1, a3 = arg2, a4 = arg3). It returns the loop's result. The facts do not say what the leading zero means to event_process.
  Grade: inferred. Unverified.
  Reviewed: 2026-11-12 (sonnet)
