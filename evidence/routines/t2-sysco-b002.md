# Routine readings: flashsc, tier-2 batch 2

Entries 50 to 74 (zero-based) of the ranked tier-2 list for sysco. Every role is inferred from the rendered facts: control flow, calls and constants. Open questions are kept open in each entry.

- `L2CfgMFindBrickType` in `6.5.12/usr/sbin/flashsc` at 0x10014080
  **Inferred.** Role (from name): looks up a brick type code. Facts: it makes no calls. It loads words and bytes through pointers the facts do not resolve, compares values against 0x2 (0x100140f0), 0x4 (0x10014214) and 0x28 (0x10014230), and returns 0x3f on one path (0x1001425c); the return at 0x100141f8 has no resolved value. The facts do not show that the compared values are arguments. The shape fits a table lookup, but the table and the meaning of the bounds are not established.
  Reviewed: 2026-11-12

- `L2CfgMQuery` in `6.5.12/usr/sbin/flashsc` at 0x10014270
  **Inferred.** Builds a text report in a buffer it grows. It allocates 0x2000 bytes with malloc at 0x100142b8, extends the buffer with realloc at 0x10014338 in 8 KiB steps, and appends fields with sprintf and strcat, including inet_ntoa at 0x10014444 for one address. It returns the buffer pointer kept at 0x100474c0, but the individual field names are not decoded in these facts.
  Reviewed: 2026-11-12

- `L2CfgMAddrStr` in `6.5.12/usr/sbin/flashsc` at 0x10015030
  **Inferred.** Formats an address into the fixed buffer at 0x1004a358. It tests its second argument against the mask 0xffffffff80000000 at 0x10015084 and compares a value with 0x2 at 0x10015098, which may be an address family. One path writes the text with sprintf at 0x100155e4 (the second argument is passed on) and returns the buffer (0x10015608). The routine also has an indirect jump at 0x1001510c, reached when a value is below 0x10 (0x100150fc), whose target the facts do not resolve; the code from 0x10015114 to 0x100155c8 is not rendered in the facts and may be the targets of that jump, so the rest is open.
  Reviewed: 2026-11-12

- `L2CfgMGetConsole` in `6.5.12/usr/sbin/flashsc` at 0x10015610
  **Inferred.** Role (from name): a console route lookup. Facts: a second argument of at least 0x100 returns zero (0x100156c0), after a trace fprintf to the stdio stream at __iob+32 (0x100156a8) when the debug word is nonzero. For a second argument below 0x100 other than -1 it loads a table pointer from the first argument plus 7920 and indexes it by the second argument times 8, two words per entry. When the entry holds a nonzero word it returns one of the two words (0x100157c8); under the debug word it first formats two addresses through the indirect FmtAddr slot at 0x1000ff00 and prints them with fprintf (0x1001578c). A second argument of -1, or an entry with both words zero, takes a longer path: nine memsets of 0x400 bytes on the stack (the first three with 0, the rest with -1), byte compares against 0x33, 0x34, 0x43 and 0x49, and, under the debug word and further conditions, three FmtAddr calls and a fprintf (0x10016188). That path returns zero or a table word (0x100161f4). Another debug-word trace is at 0x1001565c. What the two table words hold, and what the longer path selects, are not established.
  Reviewed: 2026-11-12

- `L2CfgMSetConsole` in `6.5.12/usr/sbin/flashsc` at 0x10016200
  **Inferred.** When its second argument is not -1 it calls L2CfgMFindBrickCfg through its slot at 0x10013e30 (call at 0x100162c8, fourth argument 0) and returns zero if the result is null. It reads the byte at offset 4 of the returned record and compares it with 0x33, 0x34 and 0x43 (returning zero if none matches), then compares the byte at offset 5 with its second argument (0x10016368, returning zero when they differ). On a match it stores one word through an unresolved pointer (0x100163bc) and returns the third argument (0x100163d0). When the second argument is -1 it instead loops, storing a word per pass (0x1001627c) while a counter is below 0x100, and returns zero (0x100162ac). What the byte codes mean is not established.
  Reviewed: 2026-11-12

- `nvramInit` in `6.5.12/usr/sbin/flashsc` at 0x100163e0
  **Inferred.** Checks the NVRAM image. When its first argument is not 1 and the word nvramCount is nonzero, it logs at priority 4 through syslog (0x10016444) and returns zero. When the mode word at 0x100474d0 is nonzero it returns zero at 0x1001649c. Otherwise it calls an unnamed routine at 0x10017870 with 0x1004a3d0 and 0x4000 and returns -1 if the result is negative (0x100164dc). It then checks two blocks with the checksum helper ipmiCheckCSum (0x1001650c over nvramCH, 8 bytes; 0x100165dc over nvramIUA, 0x82c bytes) and calls the unnamed slots at 0x10016920 (0x10016530, 0x10016590) and 0x10016890 (0x10016600) on the branches that follow, the first and last when the helper returns nonzero; their role is not identified. It compares the log start and end words at +0x818 and +0x81c against the log globals, then checks the offsets at +0x820 and +0x824 against those start and end words. It stores to +0x818 and +0x81c (0x1001670c, 0x10016720) before calling nvramLogReset at 0x10016728, and later branches append three fixed strings with nvramLogInsert at 0x10016748, 0x10016770 and 0x10016798. It returns -1 or zero at 0x10016818. The recovery steps are only partly known.
  Reviewed: 2026-11-12

- `nvramReset` in `6.5.12/usr/sbin/flashsc` at 0x10016820
  **Inferred.** It is a short wrapper. It calls the unnamed slot at 0x10016920 with its own four arguments, then calls the unnamed slot at 0x10017ad0 with 0x1004a3d0 and 0x4000 only when the mode word at 0x100474d0 equals 3, and returns zero. The mode-3 gate appears in the other nvram routines, so the meaning of mode 3 is inferred from that pattern, not established.
  Reviewed: 2026-11-12

- `nvramSet` in `6.5.12/usr/sbin/flashsc` at 0x10016a30
  **Inferred.** Takes an id in its first argument and dispatches it through a 16-entry jump table at 0x10044138 (indirect jump at 0x10016aa4). Its cases store into the NVRAM image at offsets +0x4, +0x38 to +0x58 and +0x82b, and copy strings of up to 15 and 31 bytes into the fields at +0x8 and +0x18 with strncpy (0x10016b44, 0x10016b98). It then recomputes the block checksum with ipmiCalcCSum at 0x10016d1c and, only when the mode word is 3, calls the unnamed slot at 0x10017ad0 (0x10016d58). The per-case field meanings and the early return of 0x384e0 at 0x10016a6c are not resolved by these facts.
  Reviewed: 2026-11-12

- `nvramGet` in `6.5.12/usr/sbin/flashsc` at 0x10016d80
  **Inferred.** It is the read side of nvramSet. It dispatches on an id through a 16-entry jump table at 0x10044178 (indirect jump at 0x10016de8) and returns fields from the same offsets: +0x4, +0x38 to +0x58, and the strings at +0x8 and +0x18. It returns -1 for ids of 0x10 or more (0x10017010), and when bit 0 of the mode word is clear it first calls nvramInit at 0x10016db4. Several return values are unresolved, so whether each case returns a scalar or a pointer is not settled.
  Reviewed: 2026-11-12

- `nvramLogReset` in `6.5.12/usr/sbin/flashsc` at 0x10017030
  **Inferred.** Stores into the write offsets at +0x820 and +0x824 and sets the byte at +0x82b; the stored values are not shown in the facts. It recomputes the block checksum with ipmiCalcCSum at 0x10017088 over 0x82c bytes, and calls the unnamed slot at 0x10017ad0 only when the mode word is 3 (0x100170c0). The name suggests the offsets return to the start of the log, and nvramLogInsert uses the same offsets as write cursors, which supports that reading without proving it.
  Reviewed: 2026-11-12

- `nvramLogInsert` in `6.5.12/usr/sbin/flashsc` at 0x100170e0
  **Inferred.** Appends one text record to the log in the NVRAM image. It measures its argument with strlen (0x10017104), checks the last byte of that argument and the byte before the write offset at +0x820 for a newline (0xa), and advances the offsets with wrap-around compares against the bounds at +0x818 and +0x81c. It recomputes the checksum with ipmiCalcCSum at 0x10017448 and calls the unnamed slot at 0x10017ad0 (0x10017484) only when the mode word is 3. Whether the log is a ring or a bounded list is partly open, because the comparisons fit both.
  Reviewed: 2026-11-12

- `nvramLogFinsert` in `6.5.12/usr/sbin/flashsc` at 0x100174a0
  **Inferred.** It is a printf-style front end for nvramLogInsert. It formats its arguments into a 0xa0-byte stack buffer with snprintf, using its first argument as the format string, then passes that buffer to nvramLogInsert at 0x10017504 and returns that routine's result. The callers and their format strings are not in these facts.
  Reviewed: 2026-11-12

- `nvramLogFetch` in `6.5.12/usr/sbin/flashsc` at 0x10017520
  **Inferred.** Allocates a buffer with malloc at 0x100175fc and copies the stored log into it with memcpy at 0x10017698, 0x10017724 and 0x1001774c. The segments are chosen from the bounds at +0x818 and +0x81c and the offsets at +0x820 and +0x824, and it returns the malloc result on the normal path and zero on the path ending at 0x10017624. The segment logic reads as a ring copy, but the facts do not show how the caller uses the buffer.
  Reviewed: 2026-11-12

- `sn1SCConnInit` in `6.5.12/usr/sbin/flashsc` at 0x10017cd0
  **Inferred.** Reads the word at 0x100474d4. When it is positive it logs at priority 4 through syslog (0x10017d18) and returns zero (0x10017d30). Otherwise (blez at 0x10017cf0 taken) it increments that word, calls IRouterInit at 0x10017d4c with -1, its second argument, the incremented value and its fourth argument, copies a name of at most 0x7f bytes from its first argument into the global at 0x1004e3d0 with strncpy at 0x10017d78, and returns zero (0x10017d90). The meaning of the word at 0x100474d4 is not established, and the syslog message is not decoded here.
  Reviewed: 2026-11-12

- `sn1SCConnInitConn` in `6.5.12/usr/sbin/flashsc` at 0x10017da0
  **Inferred.** When the word at 0x100474d4 is zero it logs at level 3 through syslog (0x10017dfc) and calls exit with status 1 (0x10017e0c). Otherwise it clears a 0x2850-byte connection record at its first argument with memset at 0x10017e24. Then, when the third argument is non-null and its first byte is nonzero, it copies from that argument with strcpy at 0x10017ec0; when it is null or empty it instead calls sn1SCConnFindL2 at 0x10017e74 to locate the L2, and returns -0x64 if that returns a negative value (0x10017ea4). Both paths then call sn1SCConnReconnect at 0x10017edc, which returns its result when nonzero (0x10017f0c) and zero otherwise (0x10017f24).
  Reviewed: 2026-11-12

- `sn1SCConnReconnect` in `6.5.12/usr/sbin/flashsc` at 0x10017f30
  **Inferred.** Makes an indirect call at 0x10017f74 (the facts show an unresolved callee; the name IRouterConnect for the slot is not confirmed by them) with the name global at 0x1004e3d0, its first argument, the global at 0x100474d8, the address of its field at +0x40, and the word at +0x4c. When the callee returns nonzero it logs through LoggerDefaultMessage (0x10017f98) and syslog (0x10017fcc) and returns that value. When it returns zero it stores a word at offset 0xac of the object that +0x40 points to (0x10017ff8; the stored value is not shown in the facts) and returns the callee result. The meaning of the 0xac field and of the global at 0x100474d8 is not established.
  Reviewed: 2026-11-12

- `sn1SCConnFindL2` in `6.5.12/usr/sbin/flashsc` at 0x10018020
  **Inferred.** Calls sn1SCDiscoverL2 at 0x10018064 and returns -1 when it returns a negative value (0x100180b0, after a syslog). Otherwise it returns -1 at 0x100180fc after another syslog when a value is zero (branch at 0x100180bc), and on the remaining paths walks two byte strings from its third and fourth arguments, compares them with strcmp at 0x100181d8 and 0x10018240, and converts an address with inet_ntoa at 0x1001835c before copying the text with strcpy at 0x10018374. It has four syslog sites in all and returns -2 on one path (0x10018340) and zero on the path ending at 0x1001838c; the matching rule and the meaning of -2 are not established.
  Reviewed: 2026-11-12

- `sn1SCConnDisconnect` in `6.5.12/usr/sbin/flashsc` at 0x100183a0
  **Inferred.** Reads the object pointer at +0x40 of its first argument and makes an indirect call at 0x100183cc (the facts show an unresolved callee; the name IRouterDisconnect for the slot is not confirmed by them) with that pointer, the constant 0x36b70, and its third and fourth arguments. It then stores a word at +0x40 (0x100183d8), probably clearing the pointer, and returns the callee's result. The stored value is not shown, and the purpose of 0x36b70, perhaps a timeout, is unknown.
  Reviewed: 2026-11-12

- `sn1SCConnSocket` in `6.5.12/usr/sbin/flashsc` at 0x100183f0
  **Inferred.** Reads the object pointer at +0x40. When that pointer is nonzero it returns the word at offset zero of the object, and it returns -1 when the pointer is zero. Role (from name): the returned word is a socket value. The facts show only the load and the return, not how the field is used.
  Reviewed: 2026-11-12

- `sn1SCConnHost` in `6.5.12/usr/sbin/flashsc` at 0x10018440
  **Inferred.** Returns its first argument when the object pointer at +0x40 is nonzero, and zero otherwise. Despite the name, it does not read a host string or a host field, since the facts show no loads beyond the pointer test. The name may describe the object it returns rather than a value inside it.
  Reviewed: 2026-11-12

- `sn1SCConnClient` in `6.5.12/usr/sbin/flashsc` at 0x10018490
  **Inferred.** Returns the object pointer at +0x40 of its first argument. When that pointer is nonzero it checks it again with RouterFindClient at 0x100184c8, and when that lookup fails it stores a word back at +0x40 (0x100184dc). The facts show that store but not its value, so the zero written on a failed lookup is inferred.
  Reviewed: 2026-11-12

- `sn1SCConnReq` in `6.5.12/usr/sbin/flashsc` at 0x10018500
  **Inferred.** Sends one request packet. When the word at 0x100474d4 is zero it logs at level 3 through syslog (0x1001855c) and calls exit with status 1 (0x1001856c). Otherwise it looks up the client through sn1SCConnClient at 0x1001857c and returns -0x67 when that lookup returns zero (0x100185a0). Then it makes an indirect call at 0x100185e0 (the facts show an unresolved callee; the name IRouterRequest for the slot is not confirmed by them) with the object pointer, the packet buffer at +0x44, and five more arguments from its own arguments one to five. When the callee returns nonzero it logs through LoggerDefaultMessage and syslog and returns that value (0x1001864c). When the show word ir_show is set, it formats the packet with SCFmtPkt (0x10018688) and prints it with fprintf (0x100186a8); the halfword at +0x4a passed to the formatter is not explained.
  Reviewed: 2026-11-12

- `sn1SCConnRsp` in `6.5.12/usr/sbin/flashsc` at 0x100186d0
  **Inferred.** Role (from name): the receive side. It returns -1 when the word at 0x100474d4 is zero, when its first argument is null, or when the object pointer at +0x40 is zero. Otherwise it gets the client through sn1SCConnClient at 0x10018740 and returns -0x67 when that returns zero. Then it makes an indirect call at 0x100187dc (the facts show an unresolved callee; the name IRouterGetPacket for the slot is not confirmed by them) with the buffer at +0x44, its second argument, a stack local, its fourth argument and a final argument of 1. When the callee returns nonzero it logs through LoggerDefaultMessage and syslog (0x10018830) and returns. On success it dumps the packet with IRFmtPkt and fprintf (0x10018894, 0x100188b4) when ir_show is set; then, when the third argument is non-null, it reads bytes at offsets 0x14 and 0x21 (0x100188d0, 0x100188dc) of a record reached through a stored pointer and stores a word through the third argument (0x100188ec). It returns zero (0x10018900). The meaning of the two header bytes and the stored word are not established.
  Reviewed: 2026-11-12

- `sn1SCConnPkt` in `6.5.12/usr/sbin/flashsc` at 0x10018910
  **Inferred.** Role (from name): a generic receive path, close to sn1SCConnRsp. It returns -1 when the word at 0x100474d4 is zero. Otherwise it obtains the client through sn1SCConnClient at 0x10018970 and returns -0x67 when that returns zero, then makes an indirect call at 0x10018a0c (the facts show an unresolved callee; the name IRouterGetPacket for the slot is not confirmed by them) with the buffer at +0x44, its second argument, a stack local, its fourth argument and a final argument of zero. When the callee returns nonzero other than -0x1a4 (0x10018a30) it logs through LoggerDefaultMessage and syslog; for either nonzero result it then calls sn1SCConnClient again when the first argument is non-null (0x10018a8c) and returns. On a zero result it stores a word through its third argument (0x10018ab8) and dumps the packet with IRFmtPkt at 0x10018ae4 when ir_show is set. The facts show the final argument as zero here and one in sn1SCConnRsp, but do not say what that argument selects.
  Reviewed: 2026-11-12

- `sn1SCConnSendRsp` in `6.5.12/usr/sbin/flashsc` at 0x10018b30
  **Inferred.** Role (from name): sends a response packet. When the word at 0x100474d4 is zero it logs at level 3 through syslog (0x10018b88) and calls exit with status 1 (0x10018b98). Otherwise it obtains the client through sn1SCConnClient at 0x10018ba8 and returns -0x67 when that returns zero. It reads the halfword at +0x4a and the object pointer at +0x40, and makes an indirect call at 0x10018c08 (the facts show an unresolved callee; the name IRouterResponse for the slot is not confirmed by them) with the object pointer, the buffer at +0x44 and four more arguments from its own arguments one to four. When the callee returns nonzero it logs through LoggerDefaultMessage and syslog and returns that value (0x10018c74). When ir_show is set it formats the packet with SCFmtPkt at 0x10018cb4, passing a value computed as the third argument plus 28, and prints it with fprintf at 0x10018cd4. The meaning of the halfword and of the +28 offset is inferred only from the shared layout with sn1SCConnReq.
  Reviewed: 2026-11-12
