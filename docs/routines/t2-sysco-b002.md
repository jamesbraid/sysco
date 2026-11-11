# Routine readings: flashsc, tier-2 batch 2

These entries cover 25 routines from the 6.5.12 `flashsc` host tool. They are entries 50 to 74 (zero-based) of the ranked tier-2 reading list. Each was rendered from its code body and passed the grader, so none were rejected. Every grade is inferred: the rendered facts give control flow, calls and memory accesses, while the role names come from the symbol table and from how the routines call one another.

### `6.5.12/usr/sbin/flashsc` 0x10014080

`L2CfgMFindBrickType` makes no calls. It reads a byte and a word through pointers the rendered facts do not resolve, tests small arguments against 0x2, 0x4 and 0x28, and returns 0x3f on one path. The shape fits a lookup of a brick type code against a table, but the table and the meaning of the bounds are not established here.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10014270

`L2CfgMQuery` builds a text report in a buffer it grows. It allocates 0x2000 bytes with malloc at 0x100142b8, extends the buffer with realloc at 0x10014338 in 8 KiB steps, and appends fields with many sprintf and strcat calls, including inet_ntoa at 0x10014444 for one address. It returns the buffer pointer kept at 0x100474c0; the individual field names are not decoded in these facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10015030

`L2CfgMAddrStr` formats an address into the fixed buffer at 0x1004a358. It tests the top bit of its second argument at 0x10015084 and compares a value with 0x2 at 0x10015098, which may be an address family, then writes the text with sprintf at 0x100155e4 and returns the buffer. Most of the body from 0x10015114 to 0x100155c8 is not reached from the entry, and an indirect jump at 0x1001510c has an unresolved target, so the rest of the routine is open.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10015610

`L2CfgMGetConsole` reads a debug word and a table at its first argument plus 7920, with two words per entry, and returns one of those words or zero. Under the debug word it formats two addresses through the indirect FmtAddr slot at 0x1000ff00 and prints them with fprintf to the stdio stream at __iob+32 (0x1001565c, 0x100156a8, 0x1001578c, 0x10016188). The name points at a console route lookup, but these facts do not show what the two table words hold.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10016200

`L2CfgMSetConsole` calls `L2CfgMFindBrickCfg` through its slot at 0x10013e30 (call at 0x100162c8). It reads the byte at offset 4 of the returned record and compares it against 0x33, 0x34 and 0x43, then compares the byte at offset 5 against its second argument (0x10016368). It stores two words through unresolved pointers, returns zero on most paths and the third argument on one (0x100163d0); what the byte codes mean is not established here.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x100163e0

`nvramInit` validates the NVRAM image. It logs at priority 4 through syslog at 0x10016444, checks two blocks with the checksum helper ipmiCheckCSum at 0x1001650c and 0x100165dc (8 bytes and 0x82c bytes), and compares the log start and end words at +0x818 and +0x81c and the offsets at +0x820 and +0x824 against the log globals. Later branches reset the log through nvramLogReset at 0x10016728 and append three fixed strings with nvramLogInsert at 0x10016748, 0x10016770 and 0x10016798. Two unnamed slots at 0x10016920 and 0x10016890 are called without an identified role, so the recovery steps are only partly known.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10016820

`nvramReset` is a short wrapper. It calls the unnamed slot at 0x10016920 with its own four arguments, then calls the unnamed slot at 0x10017ad0 with 0x1004a3d0 and 0x4000 only when the mode word at 0x100474d0 equals 3, and returns zero. The same mode-3 gate appears in the other nvram routines, so the meaning of mode 3 is inferred from that pattern, not established.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10016a30

`nvramSet` takes an id in its first argument and dispatches it through a 16-entry jump table at 0x10044138 (indirect jump at 0x10016aa4). Its cases store into the NVRAM image at offsets +0x4, +0x38 to +0x58 and +0x82b, and copy strings of up to 15 and 31 bytes into the fields at +0x8 and +0x18 with strncpy (0x10016b44, 0x10016b98). It then recomputes the block checksum with ipmiCalcCSum at 0x10016d1c and, only when the mode word is 3, calls the unnamed slot at 0x10017ad0 (0x10016d58). The per-case field meanings and the early return of 0x384e0 at 0x10016a6c are not resolved by these facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10016d80

`nvramGet` is the read side of `nvramSet`. It dispatches on an id through a 16-entry jump table at 0x10044178 (indirect jump at 0x10016de8) and returns fields from the same offsets: +0x4, +0x38 to +0x58, and the strings at +0x8 and +0x18. It returns -1 for ids of 0x10 or more (0x10017010), and when bit 0 of the mode word is clear it first calls nvramInit at 0x10016db4. Several return values are unresolved in the facts, so whether each case returns a scalar or a pointer is not settled.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10017030

`nvramLogReset` stores into the write offsets at +0x820 and +0x824 and sets the byte at +0x82b; the stored values are not shown in the facts. It recomputes the block checksum with ipmiCalcCSum at 0x10017088 over 0x82c bytes, and calls the unnamed slot at 0x10017ad0 only when the mode word is 3 (0x100170c0). The name suggests the offsets return to the start of the log. nvramLogInsert uses the same offsets as write cursors, which supports that reading without proving it.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x100170e0

`nvramLogInsert` appends one text record to the log in the NVRAM image. It measures its argument with strlen (0x10017104), checks the last byte of that argument and the byte before the write offset at +0x820 for a newline (0xa), and advances the offsets with wrap-around compares against the bounds at +0x818 and +0x81c. It recomputes the checksum with ipmiCalcCSum at 0x10017448 and calls the unnamed slot at 0x10017ad0 (0x10017484) only when the mode word is 3. Whether the log is a ring or a bounded list is partly open, because the comparisons fit both.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x100174a0

`nvramLogFinsert` formats its arguments into a 0xa0-byte stack buffer with snprintf, using its first argument as the format string. It passes that buffer to nvramLogInsert at 0x10017504 and returns that routine's result. It is a printf-style front end for nvramLogInsert; the callers and their format strings are not in these facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10017520

`nvramLogFetch` allocates a buffer with malloc at 0x100175fc and copies the stored log into it with memcpy at 0x10017698, 0x10017724 and 0x1001774c. The segments it copies are chosen from the bounds at +0x818 and +0x81c and the offsets at +0x820 and +0x824. It returns the malloc result on the normal path and zero on the path ending at 0x10017624. The segment logic reads as a ring copy, but the facts do not show how the caller uses the returned buffer.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10017cd0

`sn1SCConnInit` logs at priority 4 through syslog at 0x10017d18 and copies a name of at most 0x7f bytes into the global at 0x1004e3d0 with strncpy at 0x10017d78. It calls IRouterInit at 0x10017d4c with -1 and the global at 0x100474d4 plus one, and that call sits after a test that skips it when the global is zero or negative (blez at 0x10017cf0). What the counter means is not established, and the exact return paths (0x10017d30, 0x10017d90) are not traced from the syslog call.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10017da0

`sn1SCConnInitConn` clears a 0x2850-byte connection record at its first argument with memset at 0x10017e24, then calls sn1SCConnFindL2 at 0x10017e74 to locate the L2. When the third argument is nonzero it copies from that argument with strcpy at 0x10017ec0, and it calls sn1SCConnReconnect at 0x10017edc. It returns -0x64 when the L2 lookup fails (0x10017ea4), the reconnect result at 0x10017f0c, or zero at 0x10017f24; a level-3 syslog at 0x10017dfc and an exit with status 1 at 0x10017e0c sit on paths whose conditions are only partly shown.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10017f30

`sn1SCConnReconnect` calls the IRouterConnect slot at 0x10017f74 with the host-name global at 0x1004e3d0, the object at +0x40, the global at 0x100474d8, and the field at +0x4c. It stores the returned word at offset 0xac of the object that +0x40 points to (0x10017ff8). It logs through LoggerDefaultMessage at 0x10017f98 and syslog at 0x10017fcc when the callee returns nonzero (branch at 0x10017f88). The meaning of the 0xac field and of the global at 0x100474d8 is not established.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10018020

`sn1SCConnFindL2` calls sn1SCDiscoverL2 at 0x10018064 and returns -1 when discovery fails (0x100180b0). Otherwise it walks two byte strings from its third and fourth arguments, compares them with strcmp at 0x100181d8 and 0x10018240, and converts an address with inet_ntoa at 0x1001835c before copying the text with strcpy at 0x10018374. It logs at four syslog sites, returns -2 on one path (0x10018340) and zero on the path ending at 0x1001838c; the matching rule and the meaning of -2 are not established by the rendered facts.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x100183a0

`sn1SCConnDisconnect` reads the object pointer at +0x40 of its first argument and calls the IRouterDisconnect slot at 0x100183cc with that pointer, the constant 0x36b70, and its own last two arguments. It then stores a word at +0x40 (0x100183d8), probably clearing the pointer, and returns the callee's result. The stored value is not shown in the facts, and the purpose of 0x36b70, perhaps a timeout, is unknown.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x100183f0

`sn1SCConnSocket` reads the object pointer at +0x40. When that pointer is nonzero it returns the word at offset zero of the object, which the facts treat as the socket value, and it returns -1 when the pointer is zero. The shape matches the sn1SCSocket accessor in the l1cmd notes, so the socket reading is inferred from that shape, not from how the field is used here.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10018440

`sn1SCConnHost` returns its first argument when the object pointer at +0x40 is nonzero, and zero otherwise. Despite the name, it does not read a host string or a host field; the facts show no loads beyond the pointer test. The name may describe the object it returns rather than a value inside it.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10018490

`sn1SCConnClient` returns the object pointer at +0x40 of its first argument. When that pointer is nonzero it checks it again with RouterFindClient at 0x100184c8, and when that lookup fails it stores a word back at +0x40 (0x100184dc). The facts show that store but not its value, so the zero written on a failed lookup is inferred. This is the connection-side version of the client accessor in the l1cmd notes.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10018500

`sn1SCConnReq` sends one request packet. It looks up the client through sn1SCConnClient at 0x1001857c, and only when the word at 0x100474d4 is zero, and returns -0x67 when that lookup fails (0x100185a0). Otherwise it calls the IRouterRequest slot at 0x100185e0 with the object pointer, the packet buffer at +0x44, and four more arguments. When the show word ir_show is set, it formats the packet with SCFmtPkt (0x10018688) and prints it with fprintf (0x100186a8); the halfword at +0x4a passed to the formatter is not explained.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x100186d0

`sn1SCConnRsp` is the receive side. It gets the client through sn1SCConnClient at 0x10018740, only when the word at 0x100474d4 is zero, then calls the IRouterGetPacket slot at 0x100187dc into the caller's buffer with a final argument of 1. On success it reads bytes at offsets 0x14 and 0x21 of that buffer (0x100188d0, 0x100188dc) and stores a word through the third argument at 0x100188ec, and it dumps the packet with IRFmtPkt and fprintf (0x10018894, 0x100188b4) when ir_show is set. The meaning of the two header bytes and the stored word are not established.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10018910

`sn1SCConnPkt` is a generic receive path, close to sn1SCConnRsp. It obtains the client through sn1SCConnClient at 0x10018970 and calls the IRouterGetPacket slot at 0x10018a0c with the buffer at +0x44 and a final argument of zero. It stores a word through its third argument (0x10018ab8), handles one callee error value, -0x1a4, at 0x10018a30, and dumps the packet with IRFmtPkt at 0x10018ae4 when ir_show is set. The facts show the final argument as zero here and one in sn1SCConnRsp, but do not say what that argument selects.

Grade: inferred.

### `6.5.12/usr/sbin/flashsc` 0x10018b30

`sn1SCConnSendRsp` sends a response packet. It reads the halfword at +0x4a and the object pointer at +0x40, obtains the client through sn1SCConnClient at 0x10018ba8 (skipped when the word at 0x100474d4 is set), and calls the IRouterResponse slot at 0x10018c08 with the buffer at +0x44 and four more arguments. When ir_show is set it formats a packet of the length taken from the third argument plus 28 (SCFmtPkt at 0x10018cb4) and prints it with fprintf at 0x10018cd4. The halfword and the +28 offset are read as fields, but their meaning is inferred only from the shared layout with sn1SCConnReq.

Grade: inferred.

## Group notes

The routines from 0x10017f30 to 0x10018b30 share one record layout as read from the code: the object pointer at +0x40, a packet buffer at +0x44, and a halfword at +0x4a. These offsets come from the instructions, not from a documented structure, and no field name is attached to them. The unnamed slots at 0x10016920, 0x10016890 and 0x10017ad0, and the IRouter* slots, are the next routes to settle the open roles.
