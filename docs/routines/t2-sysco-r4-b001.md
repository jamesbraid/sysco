# Tier-2 routine readings, sysco batch 1

Role entries for routines rendered from the sysco host tool and firmware images. Each role
describes software behavior read from the rendered instructions, not hardware behavior.

- `l1d_tty_open` in `6.5.12/usr/sbin/flashsc` at 0x1001e660
  **Inferred.** Opens the device named by its first argument read-write (flag 0x2) and returns the
  descriptor, or -0x1 when open fails. On failure it prints an error line with errno through fprintf
  on stderr (`__iob`+32, format at 0x100453e8). After a successful open it calls the unnamed callee at
  0x1001d2c0 and then the unnamed one at 0x1001d300 with request 0x540e, returning -0x1 with another
  error line when either fails.

- `l1d_tty_close` in `6.5.12/usr/sbin/flashsc` at 0x1001e810
  **Inferred.** Closes the descriptor by calling close at 0x1001e834 with its four arguments passed
  through unchanged. It returns 0 when close returns 0 (the beq at 0x1001e83c) and -0x1 otherwise.

- `l1d_write_data` in `6.5.12/usr/sbin/flashsc` at 0x1001eca0
  **Inferred.** Writes a block of data to a device, judging from its arguments and callees. It tests a
  shared word at 0x10047530 in three branches (at 0x1001ecd0, 0x1001ed20 and 0x1001ed4c) before it
  writes, and it calls the unnamed callee at 0x1001f280 with the buffer at 0x10047820. On a failed call
  it prints an error line with errno through fprintf. The return value is the result of the unnamed
  callee on the success path, and the framing of the data is unknown from these facts.

- `IRouterSetQueueSuspendTime` in `6.5.12/usr/sbin/flashsc` at 0x10020cf0
  **Inferred.** Stores two words into the globals at 0x10047a08 and 0x10047a0c and returns its second
  argument (a1) unchanged. The source of the stored values is unknown from these facts.

- `IRouterMakePacket` in `6.5.12/usr/sbin/flashsc` at 0x10020d30
  **Inferred.** Builds a packet from the caller's record at a0. It reads a 1-byte field at +0x4 and a
  2-byte field at +0x6, writes the 2-byte field back at +0x6, and passes both to the helper at 0x10020190
  together with a local buffer at sp-80. The helper's result decides one branch (the beq at 0x10020db8),
  and the return value on the other branch is unknown from these facts.

- `IRFmtPkt` in `6.5.12/usr/sbin/flashsc` at 0x10022060
  **Inferred.** Formats a packet record at a1 as text into the caller's buffer at a2. It reads the byte
  and word fields from +0x13 to +0x21 and renders two addresses through IRFmtAddr at 0x10021cb0 and the
  class through IRFmtClas at 0x10021c00. Its final sprintf uses the format at 0x100458c8, and the routine
  returns the buffer pointer a2.

- `IRFmtError` in `6.5.12/usr/sbin/flashsc` at 0x10022140
  **Inferred.** Reports a formatting error by calling LoggerDefaultMessage at 0x1002f840 with the
  caller's arguments, after a null check on a0 (the beq at 0x10022160). On the other path it returns the
  address of the constant string at 0x100458f0. The instructions from 0x100221b0 to 0x100221c0 are not
  reached from the entry point.

- `IRouterConnectOS` in `6.5.12/usr/sbin/flashsc` at 0x100221d0
  **Inferred.** Opens the device named by a1 read-write (flag 0x2) and stores the descriptor through
  the pointer at a3. If open fails it writes a syslog message at priority 3 with text at 0x10045900
  and errno through the call at 0x10030eb0, then calls exit with -0x1. On success it returns 0 at
  0x10022284.

- `IRouterDisconnectOS` in `6.5.12/usr/sbin/flashsc` at 0x10022290
  **Inferred.** Closes the descriptor held in the global at 0x10047a10 and returns the result of close.
  Its arguments a1 to a3 pass through to close unchanged.

- `IRouterRequestOS` in `6.5.12/usr/sbin/flashsc` at 0x100222d0
  **Inferred.** Sends a request packet to the OS side over the descriptor in the global at 0x10047a10.
  It reads a 2-byte field at +0x6 of a1, builds the packet with IRouterMakePacket at 0x10020d30, and
  writes it with IRouterWritePacket at 0x100213a0. It returns -0x67 on one error path and -0x194 on
  another, and the write's result on success.

- `IRouterResponseOS` in `6.5.12/usr/sbin/flashsc` at 0x100223e0
  **Inferred.** Sends a response packet to the OS side. It reads the fields at +0x14, +0x21 and +0x22 of
  the request record a3, builds the packet with IRouterMakePacket at 0x10020d30, and writes 0x64 bytes
  through IRouterWritePacket at 0x100213a0 on the descriptor at 0x10047a10. It returns -0x67 or -0x194 on
  its error paths and the write's result on success.

- `IRouterGetPacketOS` in `6.5.12/usr/sbin/flashsc` at 0x10022540
  **Inferred.** Waits for a packet from the OS side. It clears a 128-byte local set with memset, calls
  select on the descriptor at 0x10047a10 plus one, and reads a packet with IRouterReadPacket at 0x10020de0
  into the buffer at 0x10022a40. It frees the packet with IRouterFreePacket at 0x10020b20 on the path where
  the byte at +0x16 of the packet is not 1. It returns -0x67, -0x19f or -0x1a0 on error paths, the read
  result on one path, and 0 at 0x100227b4 otherwise.

- `IRouterForwardPacket` in `6.5.12/usr/sbin/flashsc` at 0x10022dd0
  **Inferred.** Hands a packet into the router's event path. It reads the global IR_PORT_BIAS, the byte
  order global IRByteOrder and the event-loop global IRouterEventLoop, and it checks fields of the packet
  record at a2 (the byte at +0x16 and the word at +0x1c). It allocates scratch records with malloc at
  sizes 0xa and 0x14, sets the byte order with IRouterSetPacketByteOrder at 0x10021510, and passes the
  record to RouterPriClientEventOut at 0x10027e40, then flushes output with RouterFlushOutput at 0x10027f50.
  It returns -0x194 when the allocation fails and 0 at 0x100231f4 otherwise.

- `IRouterPacketToRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023200
  **Established.** Calls IRouterForwardPacket at 0x10023234 with its four arguments unchanged and returns
  that call's result at 0x10023248.

- `IRouterPacketFromRouter` in `6.5.12/usr/sbin/flashsc` at 0x10023250
  **Inferred.** Parses a packet that arrives from the router. It compares the string at a1+4 with the
  string at 0x100478b0 using strcmp at 0x10023280, and returns 0 on a match at 0x100232a4. On the other
  path it loops over a count taken from a0-1 (the blez at 0x100232c4 and the bgtz at 0x100233e4), sets
  the byte order with IRouterSetPacketByteOrder at 0x10021510 from IRByteOrder, and returns 0 or an
  unknown value.

- `EventStats` in `6.5.12/usr/sbin/flashsc` at 0x10023410
  **Inferred.** Formats the four event-callback counters (input, output, timer and select) into the
  static buffer at 0x100554a8 with sprintf and the format at 0x10045a10, then returns that buffer. The
  counters are also written back by the stores at 0x1002347c to 0x10023498, and the stored value is
  unknown from these facts.

- `EventAddInput` in `6.5.12/usr/sbin/flashsc` at 0x100234c0
  **Inferred.** Adds an input event callback to the pair of words at 0x1004f2a8 and 0x1004f2ac and
  updates the count at 0x100552a8. It logs error codes -0x12c and -0x12d through LogError at 0x1002f3a0
  on its failure paths, and returns 0x400 on one path and a0 on the other.

- `EventReplaceInput` in `6.5.12/usr/sbin/flashsc` at 0x10023680
  **Inferred.** Replaces an input event callback in the pair at 0x1004f2a8 and 0x1004f2ac. It copies the
  previous pair out through the caller's pointers a3 and a4 (the stores at 0x10023714 and 0x10023748),
  writes the new pair, and updates the count at 0x100552a8. It logs error -0x12c through LogError at
  0x1002f3a0 on its failure path.

- `EventReplaceInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023850
  **Inferred.** Replaces an input event callback like EventReplaceInput at 0x10023680, but first saves the
  current pair at 0x1004f2a8 and 0x1004f2ac into the temporary slots at 0x1004f2b8 and 0x1004f2bc. It
  updates the count at 0x100552a8, and LogError at 0x1002f3a0 reports -0x12c on its failure path.

- `EventRestoreInputTemp` in `6.5.12/usr/sbin/flashsc` at 0x10023a30
  **Inferred.** Restores the input event callback pair that EventReplaceInputTemp at 0x10023850 saved. It
  copies the slots at 0x1004f2b8 and 0x1004f2bc back into 0x1004f2a8 and 0x1004f2ac and updates the count
  at 0x100552a8. Its error path logs -0x12c through LogError at 0x1002f3a0.

- `EventRemoveInput` in `6.5.12/usr/sbin/flashsc` at 0x10023be0
  **Inferred.** Removes an input event callback. It clears the pair at 0x1004f2a8 and 0x1004f2ac, updates
  the counts at 0x100552a8 and 0x100553a8, and calls the unnamed callee at 0x10025230 with the table
  address 0x1004f2a8. The return value is 0x2b330 or the callee's result.

- `EventAddOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023d10
  **Inferred.** Adds an output event callback to the pair at 0x1004f2b0 and 0x1004f2b4 and updates the
  count at 0x10055328. It logs error codes -0x12c and -0x12e through LogError at 0x1002f3a0 on its failure
  paths, and returns 0x400 on one path and a0 on the other.

- `EventRemoveOutput` in `6.5.12/usr/sbin/flashsc` at 0x10023ed0
  **Inferred.** Removes an output event callback. It clears the pair at 0x1004f2b0 and 0x1004f2b4, updates
  the counts at 0x10055328 and 0x10055428, and calls the unnamed callee at 0x10025230 with the input-table
  address 0x1004f2a8. The return value is 0x2b040 or the callee's result.

- `EventAddTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024000
  **Inferred.** Adds a timeout event. It allocates a 0x14-byte record with malloc, and on failure logs
  -0x12f through LogError at 0x1002f3a0 and returns 0 at 0x100240a8. Otherwise it fills the record through
  EventCreateTimeOut at 0x10024100 with its own arguments, then calls the unnamed callee at 0x10025300.

- `EventCreateTimeOut` in `6.5.12/usr/sbin/flashsc` at 0x10024100
  **Inferred.** Initializes a timeout record at a0. It writes words at +0x0, +0x4, +0x8, +0xc and +0x10,
  reads the +0x0 and +0x4 words back in several places, and gets the current time with gettimeofday at
  0x10024144 into the buffer at sp-72. It returns 0 at 0x10024290.
