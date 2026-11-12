# Routine role statements, tier-2 batch 4 (sysco)

Each statement comes from rendered facts for one routine in `6.5.12/usr/sbin/flashsc`: its instructions, calls and constants. They describe what the host tool's software does, not hardware behavior. Where the facts leave the purpose open, the statement says so and the grade stays inferred.

### 6.5.12/usr/sbin/flashsc 0x1001bba0 sn1SCDiscoverL2

Allocates a 0xb18-byte table with malloc, then loops over one discovery exchange: sn1SCDiscoverL2Start at 0x1001bfc0, sn1SCDiscoverL2Response at 0x1001c6c0 and sn1SCDiscoverL2Done at 0x1001cbf0. A backward branch at 0x1001bf70 returns to a point before the Start call, and the Done call at 0x1001bf80 sits past that branch. It calls find_l2_slot at 0x1001cc40 twice (the first call only when a local flag that is 0 or 1 is nonzero), copies 0x11c bytes with memcpy at 0x1001be90, and also uses memmove at 0x1001be5c and realloc at 0x1001bf04, so it appears to record each reply into a growing slot table. The size of the memmove is not shown in the facts. It returns -1 when malloc, Start or realloc fails, and otherwise -1 or 0 depending on the result of Done; realloc is reached only on a path that compares a count against another value. What the 0x11c record holds is not established by the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001bfc0 sn1SCDiscoverL2Start

Creates a socket only when its first argument is -1: socket is called with domain 2, type 1, protocol 0x11 at 0x1001c090, followed by setsockopt with level 0xffff and option 0x20 (4-byte value, at 0x1001c138) and bind at 0x1001c1f8. A later part, guarded by a load whose value the facts cannot follow, copies two strings with strcpy and issues ioctl calls with request words 0xc020690d and 0xc0206917. When its third argument is a non-empty string it copies it into a stack buffer with strcpy at 0x1001c520, measures it with strlen and sends it with sendto at 0x1001c568, inside a loop whose control the facts do not resolve. The socket, setsockopt, bind and ioctl failures each log through strerror and syslog and close the descriptor before returning -1, and a sendto failure does the same after an errno test against 0x94 and 0x92 and a count test; an earlier failing call to an unnamed routine at 0x1001ce70 returns -1 without that logging. The option value 0x20 is consistent with a broadcast flag, but the facts alone do not settle that.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001c6c0 sn1SCDiscoverL2Response

Clears a 0x80-byte set with memset and waits on it with select. A select error logs through strerror and syslog and returns -1. When a mask test it makes after select is zero it returns 1 without receiving; otherwise it reads a datagram with recvfrom into a 0x2000-byte buffer (a receive error logs and returns -1). It then clears the caller's 0x11c-byte record with memset, stores a word at offset 200 and copies up to 0xc7 bytes with strncpy to the start of the record, duplicates the text with strdup, looks for a comma (0x2c) with strchr and splits the copy with strtok (delimiter string at 0x100477c0). Each token is compared through strcmp with four literal strings at 0x100477d0, 0x100477d8, 0x100477e0 and 0x100477e8. A match on the first copies with strncpy into the record at offset 264 (0x13 bytes), on the second at offset 208 (0x13 bytes), on the third at offset 228 (0x1f bytes), and on the fourth stores an atoi result at offset 260. It frees the copy and returns 0 after parsing. The meaning of the four key strings and of the delimiter is not visible in the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001cbf0 sn1SCDiscoverL2Done

Passes its four argument registers unchanged into a call to close and returns 0. Role (from name): the teardown step of the discovery exchange; whether the descriptor is a socket is not shown by the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001cc40 find_l2_slot

Scans a table of records in a loop bounded by a counter and a limit argument (the exit branch at 0x1001cc80 compares them). It calls strcmp twice on the string field at offset 228 of the current record against its second argument, compares the word at offset 200 against another value and against the constant 0x7f000001, and tests the word at offset 260. One path, reached after a string match when the word at offset 200 equals an argument-derived value, returns -1; the other return is one of an argument-derived value or an unknown. The facts do not show what the returned index means, what the loopback constant is for, or which path is the no-match case.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d340 L1DirectUsage

Loads one word from the global at 0x10046f4c, then returns the constant address 0x10044f48 (in .rodata). The loaded word is not part of the returned value. Role (from name): a usage-text getter; the facts show only the constant return and a leftover read.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d370 L1DirectParseCmdline

Walks its argument vector in a loop and compares each argument with fixed strings through strcmp (literals at 0x10044fd8, 0x100477f8 and 0x10047800). On one match it splits the following value at a colon with strchr, copies the part before it into the global buffer at 0x1004eea0 with strncpy and converts the part after it with a single atoi call. On another it copies a value into the global buffer at 0x1004ee60 with strcpy. Bad or missing values report through fprintf to stderr. It returns -1 or 0. The role of each option string and what the values mean are not visible in the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d7f0 L1DirectConnect

Calls l1d_open at 0x1001dff0 with its first argument set to 1 and passes the remaining three argument registers through. It returns whatever l1d_open returns. The only difference from L1DirectOpen is the constant 1, whose meaning is not visible here.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d830 L1DirectOpen

Calls l1d_open at 0x1001dff0 with its first argument set to 0 and passes the rest through unchanged. It returns the result of that call. It is the same wrapper as L1DirectConnect with the flag cleared.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d870 L1DirectClose

Forwards its four argument registers to l1d_close at 0x1001e400 and returns that result. It is a thin wrapper with no checks of its own.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d8b0 L1DirectRequest

Reads a halfword at offset 6 of its first argument and stores it as a word through its sixth argument, only when that argument is nonzero. It builds a packet with IRouterMakePacket at 0x10020d30 (passing its first three arguments, 0, and its fourth and fifth) and sends it with L1DirectSendPacket at 0x1001dad0 together with the handle loaded from 0x10047538. It returns -0x194 when IRouterMakePacket returns 0 and the send result otherwise. The packet layout is not visible in the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001d980 L1DirectResponse

Loads a halfword at offset 0x22 of its third argument and passes it, with 1, to IRouterMakePacket at 0x10020d30. It returns -0x194 when that returns 0. Otherwise it stores bytes into the new packet at offsets 0x13, 0x14 and 0x21: the byte at 0x14 is built from the bytes at offsets 0x14 and 0x21 of the third argument (the second shifted left by 8 and truncated), and the values stored at 0x13 and 0x21 are not followed by the facts. It then sends the packet with L1DirectSendPacket at 0x1001dad0 and the handle at 0x10047538 and returns its result. The mapping of the fields to protocol meaning is not established.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001dad0 L1DirectSendPacket

Only when the global at 0x10047534 is nonzero does it format the packet with IRFmtPkt at 0x10022060 into a stack buffer and print the text to stderr with fprintf. In every case it then hands the packet and handle to l1d_write_packet at 0x1001ee60 and returns 0. The flag's meaning is inferred from the debug print.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001db60 L1DirectGetPacket

When its second argument is 0 it calls EventAddTimeOut at 0x10024000 with the constant 0x3e8 (1000, unit not shown), and otherwise, when that argument is not -1, calls it with the argument itself; the handle returned is kept at 0x10047548. It then runs EventStart at 0x100250b0 in a loop, storing words through its first argument (zero or the value at 0x10047544) and re-checking a byte at offset 0x16 of the word that argument points to. On exit it calls EventRemoveTimeOut at 0x100242a0 on the saved handle if one is set and returns -1 or 0. The routine looks like a bounded wait for one packet, but the exact return contract is not settled by the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001dce0 l1d_open_serial_direct

Calls l1d_tty_open at 0x1001e660 on the device name at 0x1004ee60. When the global at 0x10047530 is nonzero it prints through fprintf before the open (format 0x100450c8) and again after a successful open (format 0x10045110). On failure it prints the device name and errno to stderr regardless of that flag and returns -1; on success it returns 0. Whether the flag is a verbosity setting is inferred from the branch shape.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001ddd0 l1d_close_serial_direct

Calls l1d_tty_close at 0x1001e810 on the handle at 0x10047538. If that call fails, it prints errno to stderr. It returns -1 on failure and 0 otherwise.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001de60 l1d_open_serial_enet

Opens a network connection through SocketMakeInetConnection at 0x10025e80, using the host name at 0x1004eea0 and the value at 0x10047540 as its first two arguments. On failure it prints the host, the value and errno through fprintf to stderr, regardless of the global flag at 0x10047530, and returns -1. When that flag is nonzero it also prints once before the connection attempt and once after a success; on success it returns 0. The meaning of the value at 0x10047540 is not visible.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001df60 l1d_close_serial_enet

Closes the network handle at 0x10047538 through SocketClose at 0x100266d0. On failure it prints the target and errno to stderr. It returns -1 on failure and 0 on success.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001dff0 l1d_open

Chooses between the serial path (l1d_open_serial_direct at 0x1001dce0, taken when the global at 0x1004752c is nonzero) and the network path (l1d_open_serial_enet at 0x1001de60, taken when it is zero). A failure of either returns -1. When its first argument is 0 it returns the handle stored at 0x10047538 at once. Otherwise it sets up a PPP layer with PPPInit at 0x1002ffd0 on the ppp state and its input and output buffers, makes an unnamed call with the handle and l1d_pkt_out, sleeps 2 seconds with sleep, and makes a second unnamed call with the handle and l1d_pkt_in; each failing call prints errno and returns -1. It then reads one byte from the handle with read or SocketRead depending on the same global, calls SocketNodelayMode at 0x10025dd0 with 1 only on a path where a compared value equals 0x18, registers l1d_read_packet as the input callback with EventAddInput at 0x100234c0 and returns 0. The purpose of the 2-second wait and of the unnamed callees is not established.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001e400 l1d_close

Only when the global at 0x1004753c is nonzero it calls EventRemoveInput at 0x10023be0 and makes an unnamed call with the handle at 0x10047538, l1d_pkt_out and 1. It then closes the transport with l1d_close_serial_direct (when the global at 0x1004752c is nonzero) or l1d_close_serial_enet, prints through fprintf when the close result tests nonzero and returns an unfollowed value on that path, and otherwise returns 0. The unnamed callee's role is not established; that it undoes the registration in l1d_open is a guess from the shared arguments.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001e4f0 l1d_tty_debug

Calls an unnamed routine at 0x1001d2c0 with its first argument and a state buffer on the stack. When that returns -1 it prints errno through fprintf to stderr and returns that result. Otherwise it prints several fixed strings through fprintf (formats at 0x10045390 to 0x100453d8), one of them in a loop whose counter is compared with 8 (over the buffer, as far as the facts show), and returns the result of the last fprintf. Role (from name): a debug dump of the tty state; the strings and the unnamed callee are not shown in the facts.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001e660 l1d_tty_open

Opens the device path with open at mode 2 (read and write), prints errno on failure, and passes the descriptor to an unnamed routine at 0x1001d2c0 with a stack buffer. It then makes a second unnamed call at 0x1001d300 with request code 0x540e and the same buffer. It returns the descriptor on success and -1 on failure. The two unnamed calls are not identified, so the terminal configuration step is only a guess from the request code.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001e810 l1d_tty_close

Calls close at 0x1001e834 on its first argument and returns -1 when that returns nonzero and 0 otherwise. Its other argument registers are not used.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001e880 l1d_read_data

Reads from the transport with SocketRead at 0x1002f090, passing its three arguments through, and returns SocketRead's result. Only when the global at 0x10047530 is nonzero it also prints its arguments through fprintf before the read, prints the read result and errno after it, and passes the fixed address 0x10047810, its buffer argument, the result and 1 to an unnamed routine at 0x1001f280. The facts show no test of the read result itself.

Grade: inferred.

Reviewed: 2026-11-12

### 6.5.12/usr/sbin/flashsc 0x1001e980 l1d_read_ppp

Calls PPPRead at 0x10030660 with the ppp state, l1d_read_data at 0x1001e880 as the reader callback and its own three arguments, and returns PPPRead's result. Only when the global at 0x10047530 is nonzero it also prints through fprintf before and after the call. The facts show it hands l1d_read_data to PPPRead; what PPPRead does with it is not visible here.

Grade: inferred.

Reviewed: 2026-11-12
