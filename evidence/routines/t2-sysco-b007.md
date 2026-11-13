# Routine readings, sysco batch 7

These role statements cover 25 routines in `6.5.12/usr/sbin/flashsc`, the host tool. Each one comes from rendered instruction facts: registers, calls, constants and branch targets, with addresses given for each claim. The rendered facts say what the code does, not what the routine is for, so every grade here is inferred. Routine names come from the symbol inventory, and the render does not check them against the binary. Where the facts leave a value or purpose open, the entry says so.

### 6.5.12/usr/sbin/flashsc 0x100251b0

EventDispatchSubset shifts its four incoming arguments up one slot, passes the constant 0x1 in the first slot, and calls event_process at 0x100246e0 (the `jal` at 0x100251f0). It returns event_process's result and does nothing else visible in the render. The fixed 0x1 may select the subset, but the render does not say what that value means, so the difference from EventDispatch stays open.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10025210

EventStop stores one word to the absolute address 0x10047564 (the `sw` at 0x10025220) and returns 0x1 (the return at 0x10025224). The render does not show the stored value, so the flag's meaning and its reader are unknown. The name suggests it stops an event loop, but these 28 bytes do not show that.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x100257c0

SocketExportInetConnection creates a stream socket with domain 0x2 and type 0x2 (the `jalr` at 0x100257f4), sets option 0x4 at level 0xffff with a 4-byte value (the `jalr` at 0x10025860), binds a 16-byte address at sp-56 (0x10025900), and listens with backlog 0x5 (0x10025970). A failed socket call logs through LogError and returns -1 without a close (0x10025840). A failed setsockopt, bind or listen logs through LogError or LogErrorHost (bind), calls close, and returns -1. The name says export, but the render shows only server-style setup, and the descriptor passed to bind and listen is not resolved in the render, so the export role is not established.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x100259f0

SocketExportUnixConnection creates a stream socket with domain 0x1 (the `jalr` at 0x10025a1c). It builds a socket address at sp-168 through the unnamed routine at 0x10026e40 (the `jal` at 0x10025a7c), unlinks the path at sp-166 (0x10025a94), then binds and listens with backlog 0x5. A failed socket call logs and returns -1 without a close (0x10025a64). A failed bind or listen logs, calls close on the socket, and returns -1, while success returns the descriptor at 0x10025b84. The unlink before bind suggests the routine reuses a path left by an earlier run, and the path field is read from the sp-166 offset, which is an inference.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10025b90

SocketAcceptInetConnection calls accept on its first argument, with the peer address at sp-72 and the length at sp-80 (0x10025bc8). On success it runs fcntl with command 0x4 and argument 0x4 (0x10025c38), but only when the caller's second argument is nonzero (the `beq` at 0x10025c20). A failed accept logs through LogError and returns -1 (0x10025c14). A failed fcntl logs through LogError, calls close and returns -1 (0x10025c78, 0x10025c90); the render does not resolve the descriptor passed to that close, so which descriptor it closes is not shown. The render records the success return only as one of ret(accept) or an unresolved value.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10025cb0

SocketAcceptUnixConnection follows the same pattern as the Inet accept routine, with accept at 0x10025ce8 using buffers at sp-168 and sp-176, the same optional fcntl(0x4, 0x4) gated on the second argument, and the same close-on-failure path. The visible differences are the buffer offsets and the LogError code (-0xcb here, -0xca in the Inet routine), so it looks like a copy with the address type changed. The render does not show the unix address type itself.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10025dd0

SocketNodelayMode applies fcntl with command 0x4 and argument 0x4 to its first argument (0x10025e0c), skipped when the second argument is zero (the `beq` at 0x10025df4). A failed fcntl logs through LogError, closes the descriptor and returns -1 (0x10025e60), and success returns 0 (0x10025e78). The name suggests a TCP no-delay option, which is normally set with setsockopt, but the render shows fcntl, so the option actually set is open.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10025e80

SocketMakeInetConnection resolves its host argument through SocketGetAddr at 0x10026830 into sp-96 (0x10025eb0), creates a stream socket with domain 0x2 and type 0x2 (0x10025f10), and passes the new descriptor to SocketNodelayMode at 0x10025dd0 (0x10025f70). It then calls connect with a 16-byte address at sp-80 (0x10025fd0). On connect failure it compares errno with the constants 0x96, 0x80, 0x91 and 0x92 (0x10025ff4 to 0x10026024); a match reaches a path that returns -2 (0x1002605c), and other failures return -1 (0x100260ac). The render does not give the meaning of the four constants or say whether the -2 path retries.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x100260d0

SocketMakeUnixConnection creates a stream socket with domain 0x1 (0x10026100), applies the same optional fcntl(0x4, 0x4) gated on the second argument (0x1002616c), and builds an address at sp-168 through the routine at 0x10026e40 (0x100261dc). It calls connect on that address (0x100261fc). On failure it compares errno with 0x96, 0x2, 0x91 and 0x92 (0x10026220 to 0x10026250), and the two outcomes are a -2 return (0x10026288) and a -1 return (0x100262d4). The 0x2 match is the visible difference from the Inet connect routine. The render does not show what 0x2 means or whether the -2 path retries.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026300

SocketSetBufferSize sets socket options on its first argument. When its second argument is nonzero it calls setsockopt at level 0xffff with option 0x1001 and a 4-byte value at sp-20 (0x10026350), then reads the option back with getsockopt (0x100263cc). It repeats that pattern for option 0x1002 (0x10026470, read-back at 0x100264ec), gated on its third argument being nonzero (the `beq` at 0x10026448). It also calls setsockopt, when the second or third argument is nonzero (the branches at 0x10026568 and 0x10026574), with level 0x6 option 0x1 (0x100265a4) and level 0x0 option 0x3 (0x1002663c), each with a 4-byte value at sp-68; the render does not show the stored values. Role (from name): the first two options are probably a send and receive buffer size pair, but the render does not name them, and what the level 0x6 and level 0x0 settings do is open. A failed setsockopt logs through LogError with code -0xd1 and the routine returns -0xd1 (0x100263a4, 0x100264c4, 0x1002660c, 0x100266a4). When a getsockopt read-back does not satisfy the comparison at 0x100263dc or 0x100264fc, it prints a message with fprintf to a stdio stream at offset 32 of the stdio table (0x10026400, 0x10026520), logs with code -0xd1 and returns -0xd1. An errno of 0x63 is treated as acceptable after the level 0x6 and level 0x0 calls (0x100265d0, 0x10026668), and the routine returns 0 at the end (0x100266bc).

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x100266d0

SocketClose calls shutdown on its first argument with second argument 0x2 (0x1002671c), then close (0x1002672c), and returns close's result. A negative first argument returns -1 (0x10026708) without touching the socket. The only difference from SocketCloseOnly is the shutdown call, so that call is what separates the two.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026750

SocketCloseOnly calls close on its first argument and returns the result (0x10026798, 0x100267ac). A negative descriptor returns -1 (0x10026788). It makes no shutdown call, which matches the name.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x100267c0

SocketCloseUnixServer builds a socket address at sp-144 through the routine at 0x10026e40 (0x100267f0), unlinks the path at sp-142 (0x10026800), then closes its first argument (0x10026810) and returns close's result. The unlink on close suggests it removes the filesystem path of a unix listening socket, mirroring the bind step of SocketExportUnixConnection. The render does not show where the path comes from.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026830

SocketGetAddr turns a host string in its first argument into four bytes at its second argument. A null or empty string returns -1 (0x10026888). It tries inet_addr first (0x10026898, with the test at 0x100268b8), writes a word to the output at 0x100268b0 whose value the render does not show, and when inet_addr returns -1 it falls back to gethostbyname (0x100268e0). The fallback copies the first address from the host record with a 4-byte memcpy (0x10026930) and returns 0, or -1 when the lookup fails (0x10026910).

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026950

SocketGetPeerAddr calls getpeername on its first argument, with an address buffer at sp-56 and a length at sp-64 (0x10026988). A failure logs through LogError with code -0xcf and returns -1 (0x100269cc). Success writes one word to the second argument (0x100269dc) and returns 0 (0x100269f0); the render does not show the stored word, so what the caller receives is open.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026a00

SocketGetSockAddr has the same shape as SocketGetPeerAddr, calling getsockname on its first argument (0x10026a38), writing one word to the second argument (0x10026a8c), and logging with code -0xd0 on failure. Together with the peer routine it looks like the local-address counterpart. The stored word is not shown in the render.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026ab0

SocketGetName clears the buffer at its second argument with memset (0x10026aec), calls gethostbyaddr on a stack buffer at sp-64 with length 4 and type 2 (0x10026b04), and copies the returned host name into the buffer with strncpy, limited to a2-1 bytes (0x10026b4c). It returns 0 when a name comes back and -1 when none does (0x10026b2c, 0x10026b64). The render does not set the stack buffer at sp-64, so the lookup's input is unclear.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026b70

SocketIsLocalHost calls LoggerDisable at 0x1002f340 (0x10026b94) and fetches this machine's host name with SocketGetHostName into a stack buffer (0x10026bac). Only when SocketGetHostName returns 0 does it compare that buffer with its first argument using strcmp (0x10026bcc; the `bne` at 0x10026bb8 skips the compare otherwise). It then calls LoggerRestore (0x10026bf0) with LoggerDisable's result. Role (from name): the Logger pair suspends and restores logging around the host-name lookup. It returns 0 or 1 (0x10026c08), but the render does not show which branch gives 1, so the sense of the result is open. The name suggests a test of whether a given host string is this machine.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026c10

SocketGetHostName clears a 258-byte static buffer at 0x100554f8 and calls gethostname on it with length 0x101 (0x10026c68). It writes one word through its first argument (0x10026c54), whose value the render does not show. It returns 0 on success (0x10026cc4), and on failure it logs through LogError and returns -1 (0x10026c94, 0x10026cac). Callers probably read the name from the static buffer, but the render does not show that directly.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026cd0

SocketGetHostAddr gets the local host name into a stack buffer through SocketGetHostName (0x10026cf4) and compares it with a static buffer at 0x10055600 using strncmp with length 0x101 (0x10026d34). When SocketGetHostName fails it returns -1 at once (0x10026d18), without a log call in this routine. When the names differ it copies the new name into the static buffer (strcpy at 0x10026d58) and resolves it through SocketGetAddr into 0x10047590 (0x10026d70). It then writes one word through its first argument (0x10026dcc), which looks like a cached local address whose value the render does not show. A failed SocketGetAddr logs through LogErrorHost (0x10026da4) and returns -1 (0x10026dbc), and success returns 0 (0x10026de0).

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026df0

SocketAddrStr passes its first argument to inet_ntoa (0x10026e24), shown in the render as a0 shifted left by 32 bits, and returns the string pointer that inet_ntoa gives back (0x10026e38). The render also shows a1 and a2 holding other values at the call; it does not show whether inet_ntoa reads them. The unnamed routine at 0x10026e40 follows this one, and the unix socket routines call it.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10026ed0

RouterInit sets the router's global defaults. It stores one word at 0x10026f08 before testing its first argument, and when that argument is below 0xa (the fall-through of the `beq` at 0x10026f10) a second store follows at 0x10026f20. Further stores at 0x10026f2c, 0x10026f38, 0x10026f44, 0x10026f9c, 0x10026fb0 and 0x10026fcc set other globals; the render's store lines do not name their targets, so which global each store sets comes from the symbol inventory, not from the facts. It clears the InputStats and OutputStats blocks with memset (0x10026f58, 0x10026f70) and takes the program's base name after the last slash with strrchr (0x10026f84). It then installs a handler for signal 0xd with value 0x1 (0x10026fdc), fills UserName and FullName from getuid and getpwuid (0x1002702c, 0x10027090, 0x100270ac), and resolves its host name into the IPAddress global (0x100270bc, 0x100270ec). It returns 0 on success and -1 when the password lookup fails (0x10027074). The stored values are not shown.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10027110

RouterClientsConnected walks the ClientList global from its head (loaded at 0x10027128). It tests bit 0x2 on each entry (the `beq` at 0x1002714c) and compares a value with 0x80 (the `bne` at 0x10027164). Entries that pass both tests are probably the ones it counts or reports, but the fields it loads show as unresolved in the render, so the tested fields and the returned value are not established. It returns 0 or an unresolved value at 0x1002719c.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x100271b0

RouterConnectionCount walks the same ClientList global with the same bit-0x2 test (the `beq` at 0x100271ec), but it has no 0x80 compare, which its sibling RouterClientsConnected does have. It keeps looping while a further field is nonzero (the `bne` at 0x10027210) and returns 0 or an unresolved value at 0x10027224. The difference suggests the two routines count different subsets of the client list, but the render does not establish which.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x10027230

RouterHostConnected walks the ClientList linked list. The next-node load is at offset 0x168 of each node (the load at 0x1002736c, stored back into the routine's cursor variable at 0x10027374), and the `bne` at 0x10027380 loops back to 0x10027278 while the cursor is nonzero. For each node it compares the string at offset 0x2c with its first argument (strcmp at 0x1002728c), and on a match it checks the words at offsets 0x8 and 0xc against two of its other arguments. It then loads the word at offset 0x18 and compares it with 0x1 (the `beq` at 0x100272e8) and 0x4 (0x100272f8), and in the 0x1 case with 0x40 (the `bne` at 0x1002731c); the facts show those compares on masked values that they do not resolve. It returns 0x1 (0x10027334), 0x4 (0x10027354), or 0x0 at the end of the list (0x10027398). The routine looks like a check for whether a named host is already connected, but the meaning of the 0x1 and 0x4 returns and of the field at 0x18 is not established.

Grade: inferred.

Reviewed: 2026-11-12
