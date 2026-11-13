# Tier-2 routine readings, sysco batch 1

Role entries for routines rendered from the sysco host tool and firmware images. Each role
describes software behavior read from the rendered instructions, not hardware behavior.

- `l1d_tty_open` in `6.5.12/usr/sbin/flashsc` at 0x1001e660
  **Inferred.** Opens the device named by its first argument read-write (flag 0x2) and returns the
  descriptor, or -0x1 when open fails. On failure it prints an error line with errno through fprintf
  to the stream at `__iob`+32 (format at 0x100453e8). After a successful open it calls the unnamed
  callee at 0x1001d2c0 and then the unnamed one at 0x1001d300 with request 0x540e, returning -0x1
  with another error line when either call returns -0x1.
  Reviewed: 2026-11-12

- `l1d_tty_close` in `6.5.12/usr/sbin/flashsc` at 0x1001e810
  **Inferred.** Closes the descriptor by calling close at 0x1001e834 with its four arguments passed
  through unchanged. It returns 0 when close returns 0 (the beq at 0x1001e83c) and -0x1 otherwise.
  Reviewed: 2026-11-12

- `l1d_write_data` in `6.5.12/usr/sbin/flashsc` at 0x1001eca0
  **Inferred.** Role (from name): writes a block of data to a device; the facts show only a call through
  an unresolved callee, so the role is a guess. It passes its three arguments unchanged to that call
  (call 2, at 0x1001ed10) and returns that call's result. The word at 0x10047530 is tested at 0x1001ecd0,
  0x1001ed20 and 0x1001ed4c, and when it is nonzero three more calls run. Before call 2 there is an fprintf
  with the format at 0x100455b0, and the beq at 0x1001ecd0 skips it when the word is zero. After call 2
  there is a call to the unnamed callee at 0x1001f280 with 0x10047820, a1, a2 and the constant 1,
  followed by an fprintf with the format at 0x100455d0, call 2's result and errno. The fprintf calls are
  not conditioned on the result of call 2, only on that word.
  Reviewed: 2026-11-12

- `IRouterSetQueueSuspendTime` in `6.5.12/usr/sbin/flashsc` at 0x10020cf0
  **Inferred.** Stores two words into the globals at 0x10047a08 and 0x10047a0c and returns its second
  argument (a1) unchanged. The source of the stored values is unknown from these facts.
  Reviewed: 2026-11-12

- `IRouterMakePacket` in `6.5.12/usr/sbin/flashsc` at 0x10020d30
  **Inferred.** Role (from name): builds a packet from the caller's record at a0. It reads the 2-byte
  field at +0x6 and the 1-byte field at +0x4 of that record and stores a halfword at +0x6 of the same
  record (the stored value is not shown by the facts, so it is not necessarily the value read). It passes
  the 1-byte and 2-byte values to the helper at 0x10020190 together with a local buffer at sp-80. The
  helper's result decides one branch (the beq at 0x10020db8), and the return value on the other branch
  is unknown from these facts.
  Reviewed: 2026-11-12

- `IRFmtPkt` in `6.5.12/usr/sbin/flashsc` at 0x10022060
  **Inferred.** Formats a packet record at a1 as text into the caller's buffer at a2. It reads the byte
  and word fields from +0x13 to +0x21 and renders two addresses through IRFmtAddr at 0x10021cb0 and the
  class through IRFmtClas at 0x10021c00. Its final sprintf uses the format at 0x100458c8, and the routine
  returns the buffer pointer a2.
  Reviewed: 2026-11-12

- `IRFmtError` in `6.5.12/usr/sbin/flashsc` at 0x10022140
  **Inferred.** Reports a formatting error. When a0 is nonzero (the beq at 0x10022160 skips this when a0
  is zero) it calls LoggerDefaultMessage at 0x1002f840 with its first argument in a1 and its a2 and a3
  passed through; a0 of that call is derived from the first argument and the facts do not show how.
  It returns that call's result. On the other path it returns the address of the constant string at
  0x100458f0.
  Reviewed: 2026-11-12

- `IRouterConnectOS` in `6.5.12/usr/sbin/flashsc` at 0x100221d0
  **Inferred.** Opens the device named by a1 read-write (flag 0x2) and keeps the descriptor in the
  global at 0x10047a10 (the store at 0x10022210). If open fails it writes a syslog message at priority 3
  with text at 0x10045900 and errno through the call at 0x10030eb0, then calls exit with -0x1. On
  success it stores one word through the pointer in a3 (the store at 0x10022270, whose value the facts
  do not show) and returns 0 at 0x10022284.
  Reviewed: 2026-11-12

- `IRouterDisconnectOS` in `6.5.12/usr/sbin/flashsc` at 0x10022290
  **Inferred.** Closes the descriptor held in the global at 0x10047a10 and returns the result of close.
  Its arguments a1 to a3 pass through to close unchanged.
  Reviewed: 2026-11-12

- `IRouterRequestOS` in `6.5.12/usr/sbin/flashsc` at 0x100222d0
  **Inferred.** Role (from name): sends a request packet to the OS side over the descriptor in the
  global at 0x10047a10. It returns -0x67 when that descriptor is -0x1. When the pointer in a6 (tested at
  0x10022334) is nonzero it also reads the 2-byte field at +0x6 of a1 and stores it through that pointer.
  It builds the packet with IRouterMakePacket at 0x10020d30 and returns -0x194 when that returns 0.
  Otherwise it writes the packet with IRouterWritePacket at 0x100213a0, passing a1, the constant
  0x10022830, the descriptor and the packet, and returns the write's result.
  Reviewed: 2026-11-12

- `IRouterResponseOS` in `6.5.12/usr/sbin/flashsc` at 0x100223e0
  **Inferred.** Role (from name): sends a response packet to the OS side. It returns -0x67 when the
  descriptor in the global at 0x10047a10 is -0x1. It builds a packet with IRouterMakePacket at
  0x10020d30 from a1, a3+28, the 2-byte field at +0x22 of the request record a3 and the constant 1, and
  returns -0x194 when that returns 0. Otherwise it reads the bytes at +0x14 and +0x21 of a3, stores
  bytes at +0x13, +0x14 and +0x21 of the built packet (the stored values are not shown by the facts), and
  calls IRouterWritePacket at 0x100213a0 with a1, 0x10022830, the descriptor and the packet. The facts
  also show 0x64 in a6 at that call, but nothing shows it is an argument or a length: 0x64 is the
  divisor of the remainder stored at +0x13. It returns the write's result.
  Reviewed: 2026-11-12

- `IRouterGetPacketOS` in `6.5.12/usr/sbin/flashsc` at 0x10022540
  **Inferred.** Role (from name): waits for a packet from the OS side. It returns -0x67 when the
  descriptor in the global at 0x10047a10 is -0x1. It clears a 128-byte local set with memset, calls
  select on the descriptor plus one with that set, and returns -0x19f when select returns a negative
  value and -0x1a0 when the masked test at 0x100226fc is zero. Otherwise it calls IRouterReadPacket at
  0x10020de0 with the constant 0x10022a40 as its second argument (the facts do not show that this is a
  buffer) and returns its result when that is nonzero. When the read returns 0 and the argument tested
  at 0x1002276c is nonzero, it frees the packet with IRouterFreePacket at 0x10020b20 on the path where
  the byte at +0x16 of the packet is not 1. The return of 0 at 0x100227b4 is reached when that argument
  is zero or the byte is 1.
  Reviewed: 2026-11-12

- `IRouterForwardPacket` in `6.5.12/usr/sbin/flashsc` at 0x10022dd0
  **Inferred.** Role (from name): hands a packet into the router's event path. It reads the globals
  IR_PORT_BIAS, IRByteOrder and IRouterEventLoop, tests the word at +0x8 of a0 against IR_PORT_BIAS plus
  9001, and checks fields of the packet record at a2 (the byte at +0x16 and the word at +0x1c). It
  allocates one scratch record with malloc (the facts do not show the size, only a load of the byte at
  +0x15 of a2 at 0x10022f10 beforehand), and returns -0x194 when malloc returns 0. It sets the byte order with
  IRouterSetPacketByteOrder at 0x10021510 in two loops that start from a2 and follow the word at +0x0 of each record, first
  with the constant 1 and later with IRByteOrder. It passes the scratch record to RouterPriClientEventOut
  at 0x10027e40, frees it with free, frees the packet with IRouterFreePacket at 0x10020b20, and calls
  RouterFlushOutput at 0x10027f50 only when IRouterEventLoop is zero. It returns 0 at 0x100231f4.
  Reviewed: 2026-11-12

- `IRouterPacketToRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023200
  **Established.** Calls IRouterForwardPacket at 0x10023234 with its four arguments unchanged and returns
  that call's result at 0x10023248.
  Reviewed: 2026-11-12

- `IRouterPacketFromRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023250
  **Inferred.** Role (from name): parses a packet that arrives from the router. It compares the string at
  a1+4 with the string at 0x100478b0 using strcmp at 0x10023280, and returns 0 at 0x100232a4 when strcmp
  returns nonzero (the beq at 0x1002328c skips that return when strcmp returns 0). When strcmp returns
  0 it loops over a count taken from a0-1, and the blez at 0x100232c4 skips the loop when that count is
  not positive. The loop's backward branch is the bgtz at 0x100233e4. Each pass sets the byte order with
  IRouterSetPacketByteOrder at 0x10021510 from IRByteOrder. It returns an unknown value on this path
  (the facts show 0 or an unnamed value) at 0x100233fc.
  Reviewed: 2026-11-12

- `EventStats` in `6.5.12/usr/sbin/flashsc` at 0x10023410
  **Inferred.** Formats the four event-callback counters (input, output, timer and select) into the
  static buffer at 0x100554a8 with sprintf and the format at 0x10045a10, then returns that buffer. When
  a0 is nonzero (the beq at 0x1002346c skips them otherwise) it also makes four word stores,
  at 0x1002347c, 0x10023484, 0x10023490 and 0x10023498; the facts do not show the stored value or the
  addresses.
  Reviewed: 2026-11-12

- `EventAddInput` in `6.5.12/usr/sbin/flashsc` at 0x100234c0
  **Inferred.** Role (from name): adds an input event callback in slot a0 of the table at 0x1004f2a8
  (24-byte slots indexed by a0, from the instructions at 0x10023538 to 0x10023550). When a0 is 0x401 or
  more it logs -0x12c through LogError at 0x1002f3a0 and returns 0x400. Otherwise, when the slot's first
  word is already nonzero it logs -0x12d, and it then stores a1 and a2 into the slot's first two words,
  sets bit a0 in the bitmap at 0x100552a8, sets the word at 0x10047560 to a0+1 when a0 is not below it,
  and returns a0.
  Reviewed: 2026-11-12

- `EventReplaceInput` in `6.5.12/usr/sbin/flashsc` at 0x10023680
  **Inferred.** Role (from name): replaces an input event callback in slot a0 of the table at 0x1004f2a8
  (24-byte slots indexed by a0). When a0 is 0x401 or more it logs -0x12c through LogError at 0x1002f3a0
  and carries on, with no early return. When the pointer a3 is nonzero it stores the slot's previous
  first word through it (0x10023714), and when a4 is nonzero it stores the previous second word through
  it (0x10023748). It then writes a1 and a2 into the slot's first two words, sets bit a0 in the bitmap at
  0x100552a8, and sets the word at 0x10047560 to a0+1 when a0 is not below it. The return value is
  unknown from these facts.
  Reviewed: 2026-11-12

- `EventReplaceInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023850
  **Inferred.** Role (from name): replaces an input event callback in slot a0 of the table at 0x1004f2a8,
  like EventReplaceInput at 0x10023680 but without the out pointers. It first copies the slot's first
  two words into the words at +0x10 and +0x14 of the same slot (0x1004f2b8 and 0x1004f2bc for slot 0).
  It then writes a1 and a2 into the slot's first two words, sets bit a0 in the bitmap at 0x100552a8, and
  sets the word at 0x10047560 to a0+1 when a0 is not below it. When a0 is 0x401 or more it logs -0x12c
  through LogError at 0x1002f3a0 and carries on. The return value is unknown from these facts.
  Reviewed: 2026-11-12

- `EventRestoreInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023a30
  **Inferred.** Role (from name): restores the input event callback that EventReplaceInputTemp at
  0x10023850 saved. When a0 is 0x401 or more it logs -0x12c through LogError at 0x1002f3a0 and carries
  on. When the saved word at +0x10 of slot a0 is nonzero (the beq at 0x10023aa4 skips the copy
  otherwise) it copies the saved words at +0x10 and +0x14 back into the slot's first two words
  (0x1004f2b8 and 0x1004f2bc to 0x1004f2a8 and 0x1004f2ac for slot 0). It then sets bit a0 in the
  bitmap at 0x100552a8 and sets the word at 0x10047560 to a0+1 when a0 is not below it. The return
  value is unknown from these facts.
  Reviewed: 2026-11-12

- `EventRemoveInput` in `6.5.12/usr/sbin/flashsc` at 0x10023be0
  **Inferred.** Role (from name): removes an input event callback. Only when a0 is below 0x400
  (unsigned; the beq at 0x10023c04 skips the rest otherwise) it clears the first two words of slot a0 in
  the table at 0x1004f2a8 (24-byte slots), clears bit a0 in the bitmaps at 0x100552a8 and 0x100553a8,
  and calls the unnamed callee at 0x10025230 with a0. The facts show a1 = 0x1004f2a8 on that call. The return value is not established: the
  facts list 0x2b330 or the callee's result.
  Reviewed: 2026-11-12

- `EventAddOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023d10
  **Inferred.** Role (from name): adds an output event callback in slot a0 of the table at 0x1004f2a8,
  in the words at +0x8 and +0xc of the slot (0x1004f2b0 and 0x1004f2b4 for slot 0). When a0 is 0x401 or
  more it logs -0x12c through LogError at 0x1002f3a0 and returns 0x400. Otherwise, when the slot's word
  at +0x8 is already nonzero it logs -0x12e, and it then stores a1 and a2 into the slot's words at +0x8
  and +0xc, sets bit a0 in the bitmap at 0x10055328, sets the word at 0x10047560 to a0+1 when a0 is
  not below it, and returns a0.
  Reviewed: 2026-11-12

- `EventRemoveOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023ed0
  **Inferred.** Role (from name): removes an output event callback. Only when a0 is below 0x400
  (unsigned; the beq at 0x10023ef4 skips the rest otherwise) it clears the words at +0x8 and +0xc of
  slot a0 in the table at 0x1004f2a8 (24-byte slots), clears bit a0 in the bitmaps at 0x10055328 and
  0x10055428, and calls the unnamed callee at 0x10025230 with a0. The facts show a1 = 0x1004f2a8 on that call. The return value is not
  established: the facts list 0x2b040 or the callee's result.
  Reviewed: 2026-11-12

- `EventAddTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024000
  **Inferred.** Role (from name): adds a timeout event. When the word at 0x1004756c is nonzero it takes
  the record that word points to and replaces the word with that record's word at +0x10. When it is zero
  (the beq at 0x10024038) it allocates a 0x14-byte record with malloc, and when malloc returns 0 it logs
  -0x12f through LogError at 0x1002f3a0 and returns 0 at 0x100240a8. Otherwise it fills the record
  through EventCreateTimeOut at 0x10024100, passing the record in a0 and its own four arguments in a1
  to a4, then calls the unnamed callee at 0x10025300 with the record, and returns the record pointer.
  Reviewed: 2026-11-12

- `EventCreateTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024100
  **Inferred.** Initializes a timeout record at a0. It writes words at +0x0, +0x4, +0x8, +0xc and +0x10,
  reads the +0x0 and +0x4 words back in several places, and gets the current time with gettimeofday at
  0x10024144 into the buffer at sp-72. It returns 0 at 0x10024290.
  Reviewed: 2026-11-12
