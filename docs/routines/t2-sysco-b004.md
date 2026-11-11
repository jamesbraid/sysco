# Routine role statements, tier-2 batch 4 (sysco)

Each statement comes from rendered facts for one routine in `6.5.12/usr/sbin/flashsc`: its instructions, calls and constants. They describe what the host tool's software does, not hardware behavior. Where the facts leave the purpose open, the statement says so and the grade stays inferred.

### flashsc:0x1001bba0 sn1SCDiscoverL2

Allocates a 0xb18-byte table with malloc, then drives one discovery exchange: sn1SCDiscoverL2Start at 0x1001bfc0, sn1SCDiscoverL2Response at 0x1001c6c0, and sn1SCDiscoverL2Done at 0x1001cbf0. It calls find_l2_slot at 0x1001cc40 twice and copies 0x11c-byte records with memcpy and memmove, so it appears to record each reply into a slot table. The failure paths return -1. What the 0x11c record holds is not established by the facts.

Grade: inferred.

### flashsc:0x1001bfc0 sn1SCDiscoverL2Start

Opens a UDP socket (socket called with domain 2, type 1, protocol 0x11), sets one socket option through setsockopt with level 0xffff and option 0x20, and binds it. It then sends a formatted string with sendto, using the caller's string argument copied through strcpy and strlen. Each failure logs through strerror and syslog and closes the socket. The option value 0x20 is consistent with a broadcast flag, but the facts alone do not settle that.

Grade: inferred.

### flashsc:0x1001c6c0 sn1SCDiscoverL2Response

Waits on a select set of 0x80 bytes, then reads a datagram with recvfrom into a 0x2000-byte buffer. It splits the text reply on commas with strchr and strtok, compares each token with four literal keys through strcmp, and copies strings into the caller's record at offsets 208, 228 and 264, storing words at offsets 200 and 260. It returns 1 after a receive, -1 on error and 0 at the end. The meaning of the four key strings is not visible in the facts.

Grade: inferred.

### flashsc:0x1001cbf0 sn1SCDiscoverL2Done

Passes its four argument registers unchanged into a call to close and returns 0. The routine is a socket teardown step for the discovery exchange. Whether the descriptor is always a socket is inferred from its caller, not shown here.

Grade: inferred.

### flashsc:0x1001cc40 find_l2_slot

Scans a table of records in a loop bounded by a counter and a limit. It compares a string field at offset 228 with the argument through two strcmp calls, checks the field at offset 200 against 0x7f000001 (the loopback address), and tests the field at offset 260. It returns -1 when no slot matches. The facts do not show what the slot index means or what the loopback match is for.

Grade: inferred.

### flashsc:0x1001d340 L1DirectUsage

Reads one word from a fixed global at 0x10046f4c and returns the address 0x10044f48. The loaded word is not used in the returned value, so the routine looks like a usage-text getter with a leftover read. Nothing further in the facts settles it.

Grade: inferred.

### flashsc:0x1001d370 L1DirectParseCmdline

Walks argv, comparing each argument with fixed option strings through strcmp, and splits host and port style values at a colon with strchr. It converts numbers with atoi, copies the host into the global buffer at 0x1004eea0 and the device name into 0x1004ee60, and reports bad input through fprintf to stderr. It returns -1 on error and 0 otherwise. The exact option set is only partly visible from the literal addresses.

Grade: inferred.

### flashsc:0x1001d7f0 L1DirectConnect

Calls l1d_open at 0x1001dff0 with its first argument set to 1 and passes the remaining three argument registers through. It returns whatever l1d_open returns. The only difference from L1DirectOpen is the constant 1, whose meaning is not visible here.

Grade: inferred.

### flashsc:0x1001d830 L1DirectOpen

Calls l1d_open at 0x1001dff0 with its first argument set to 0 and passes the rest through unchanged. It returns the result of that call. It is the same wrapper as L1DirectConnect with the flag cleared.

Grade: inferred.

### flashsc:0x1001d870 L1DirectClose

Forwards its four argument registers to l1d_close at 0x1001e400 and returns that result. It is a thin wrapper with no checks of its own.

Grade: inferred.

### flashsc:0x1001d8b0 L1DirectRequest

Reads a halfword at offset 6 of its first argument and writes a word through its fifth argument. It builds a packet with IRouterMakePacket at 0x10020d30 and sends it with L1DirectSendPacket at 0x1001dad0 on the handle stored at 0x10047538. It returns -0x194 when the packet cannot be built and the send result otherwise. The packet layout is only partly visible through the offsets.

Grade: inferred.

### flashsc:0x1001d980 L1DirectResponse

Reads a halfword at offset 0x22 and bytes at offsets 0x14 and 0x21 of its third argument. It writes those values into a packet made by IRouterMakePacket at offsets 0x13, 0x14 and 0x21, then sends it with L1DirectSendPacket. It returns -0x194 when the packet cannot be built. The mapping of those fields to protocol meaning is not established.

Grade: inferred.

### flashsc:0x1001dad0 L1DirectSendPacket

Formats the packet with IRFmtPkt at 0x10022060 into a stack buffer. If the global flag at 0x10047534 is set, it prints the formatted text to stderr with fprintf. It then hands the packet to l1d_write_packet at 0x1001ee60 and returns 0. The flag's meaning is inferred from the debug print.

Grade: inferred.

### flashsc:0x1001db60 L1DirectGetPacket

Sets up a 1000-millisecond timeout through EventAddTimeOut at 0x10024000, starts the event loop with EventStart at 0x100250b0, and removes the timeout with EventRemoveTimeOut at 0x100242a0. It reads a byte at offset 0x16 of its first argument and stores fields into it on the way out. It returns -1 or 0. The routine looks like a bounded wait for one packet, but the exact return contract is not settled by the facts.

Grade: inferred.

### flashsc:0x1001dce0 l1d_open_serial_direct

Opens the serial path by calling l1d_tty_open at 0x1001e660 on the device name at 0x1004ee60. On failure it prints errno through fprintf to stderr, but only when the global flag at 0x10047530 is zero. It returns -1 on failure and 0 on success. Whether the flag is a verbosity setting is inferred from the branch shape.

Grade: inferred.

### flashsc:0x1001ddd0 l1d_close_serial_direct

Calls l1d_tty_close at 0x1001e810 on the handle at 0x10047538. If that call fails, it prints errno to stderr. It returns -1 on failure and 0 otherwise.

Grade: inferred.

### flashsc:0x1001de60 l1d_open_serial_enet

Opens a network connection through SocketMakeInetConnection at 0x10025e80, using the host name at 0x1004eea0 and the value at 0x10047540. It prints errno and the target through fprintf on failure, and prints a status line when the global at 0x10047530 is zero. It returns -1 on failure and 0 on success. The meaning of the value at 0x10047540 is not visible.

Grade: inferred.

### flashsc:0x1001df60 l1d_close_serial_enet

Closes the network handle at 0x10047538 through SocketClose at 0x100266d0. On failure it prints the target and errno to stderr. It returns -1 on failure and 0 on success.

Grade: inferred.

### flashsc:0x1001dff0 l1d_open

Chooses between the serial path (l1d_open_serial_direct at 0x1001dce0) and the network path (l1d_open_serial_enet at 0x1001de60) from the global at 0x1004752c. It then sets up a PPP layer with PPPInit at 0x1002ffd0 on the ppp state and its buffers. It hands the packet handlers l1d_pkt_out and l1d_pkt_in to two unnamed routines, sleeps 2 seconds on one path, and enables TCP nodelay with SocketNodelayMode before adding a read handler with EventAddInput, and it returns -1 on failure. The purpose of the 2-second wait is not established.

Grade: inferred.

### flashsc:0x1001e400 l1d_close

Removes the input handler with EventRemoveInput at 0x10023be0 and unregisters the packet output callback. It then closes the transport with l1d_close_serial_direct or l1d_close_serial_enet and returns 0. The output callback's registration call is an unnamed callee in the facts, so its role is not established.

Grade: inferred.

### flashsc:0x1001e4f0 l1d_tty_debug

Calls an unnamed routine at 0x1001d2c0 with a state buffer on the stack, then prints several fixed status strings and the errno value through fprintf to stderr. It returns the result of the last fprintf. The routine looks like a debug dump of the tty state, but the strings are not shown in the facts.

Grade: inferred.

### flashsc:0x1001e660 l1d_tty_open

Opens the device path with open at mode 2 (read and write), prints errno on failure, and passes the descriptor to an unnamed routine at 0x1001d2c0 with a stack buffer. It then makes a second unnamed call at 0x1001d300 with request code 0x540e and the same buffer. It returns the descriptor on success and -1 on failure. The two unnamed calls are not identified, so the terminal configuration step is only a guess from the request code.

Grade: inferred.

### flashsc:0x1001e810 l1d_tty_close

Calls close on its first argument and returns -1 when that fails and 0 otherwise. It passes its other arguments through without use.

Grade: inferred.

### flashsc:0x1001e880 l1d_read_data

Prints its arguments through fprintf, then reads from the transport with SocketRead at 0x1002f090. On a read error it prints errno. It passes the buffer to an unnamed routine at 0x1001f280 with a fixed address 0x10047810 and returns the byte count from SocketRead.

Grade: inferred.

### flashsc:0x1001e980 l1d_read_ppp

Prints its arguments through fprintf, then calls PPPRead at 0x10030660 with the ppp state and l1d_read_data at 0x1001e880 as the reader. It passes its three arguments on to PPPRead and returns PPPRead's result. The routine is the PPP-side read entry that feeds from l1d_read_data.

Grade: inferred.
