# Host tool command lines: `l1cmd`, `l2cmd` and `flashsc`

What the IRIX-side tools accept and what they send, read from the IRIX 6.5.30 files (`usr/sbin/l1cmd`, `usr/sbin/l2cmd`, `usr/sbin/sysco/flashsc`; N32, MIPS-III, big-endian ELF). The three programs link the same System Controller client library, which parses the shared options and does the transport. Grades follow [AGENTS.md](../AGENTS.md).

## The shared option set

The client library's usage text (printed by the library's usage routine at `0x10006490` in `l1cmd`, read in the string table) lists these options (established):

| Option | Argument | Meaning in the usage text | Grade |
|---|---|---|---|
| `--l2` | `<host>` | host name or IP address of the L2 to connect to | established |
| `--sc` | none | connect through the default system controller (lowest rack and slot) | established |
| `--scdev` | `<device>` | connect through the named system controller device | established |
| `--interface`, `--if` | `<iface>` | network interface (for example `eth0`) of the L2 network | established |
| `--ssn` | `<serial>[:<rack>]` | serial number of the system to connect to | established |
| `--sysname` | `<name>[:<rack>]` | system name of the system to connect to | established |
| `--pw` | `<password>` | L2 password | established |
| `--irtr` | none | debug: show router packets sent and received | established |
| `--serial` | `<host>:<port>` | Ethernet-attached serial port (a terminal server) | established |
| `--dev` | `<device name>` | local serial port device | established |

The parser's string table also holds `--rack`, `--copy`, `--quiet` and `--debug` (established that the strings exist; the argument syntax and behaviour of `--rack`, `--quiet` and `--debug` were not read). `--copy <n>` is accepted for 0 through 128 and biases every network port by `n` (the message "using copy %d (biasing all network ports by %d)"). An `--ssn` or `--sysname` value may carry a rack number after a colon, and a bad one is rejected with a "bad rack number" message. The message "--serial/--dev options (direct serial connection) not supported" is present in all three programs, and the `flashsc` usage names `--serial` and `--dev` for `-l2recover` (established from the strings; which tool reaches the rejection was not read).

How `l1cmd` finds its target when no option names one (strings, inferred for the order): it lists the local hardware graph with a shell `ls` of `/hw/module/*c*/L1/controller`; if nothing is there it prints that no local L1 controller was found, falls back to an L2 at `localhost` "as a last resort", and then to L2 discovery on the network (a request sent to each interface's broadcast address, named `l2find` in the messages). With several L2 controllers found it refuses and asks for `--l2`, `--sysname` or `--ssn`.

## `l1cmd` (6.5.30)

Its `main` (`0x10006240` to `0x10006484`) was read in full (established):

1. It saves `argv[0]` as the program name and calls the library's option parser with `argc` and `argv`; a negative result prints "error while parsing library command arguments" and exits with -1.
2. If fewer than two words remain after option parsing, it prints the usage text and exits with -1.
3. It concatenates the remaining words, each followed by one space, into a 1024-byte buffer.
4. It initialises the library with the program name; a negative result exits with -1.
5. It sets up a destination word `0x0ffff003` and cuts the buffer at each semicolon; for each piece it calls the library's text-command routine (`sn1SCTextCmd`, `0x10009650`) with the destination, the piece and a reply pointer. A non-empty reply is written to the stream at offset 16 of the C library's stream table when the call returned zero and to the one at offset 32 otherwise (the usage text goes to offset 32; reading those as standard output and standard error is inferred from the 16-byte entry size). The exit status is the last call's result.

So `l1cmd` carries no command table of its own: whatever follows the options is sent as text, and the L1's own command parser decides ([l1/debug-task.md](l1/debug-task.md) describes that parser). The text-command routine is read in full in the next section.

## `l2cmd` (6.5.30)

`l2cmd` has the same loaded segments as `l1cmd`, byte for byte, except one instruction: the destination word is built with `lui a2,0x1fff` instead of `0x0fff` (file offset `0x6392`), so the destination is `0x1ffff003` (established, from a byte comparison of the first `0x28000` bytes and of the data segment). Five more differing bytes are in the ELF header's section-header offset, count and string-table index, one is the link timestamp in the dynamic section (the two files were linked two seconds apart), and one is in the dynamic symbol table (offset `0x2c67`, not interpreted). The 1.44.0 Origin 300 L1 image loads the same word `0x1ffff003` at `0xffe14ab4`, in the routine beside the helper the `*` broadcast command uses (`0xffe14acc`, [l1/debug-task.md](l1/debug-task.md)); that the destination word is an address of the same kind is inferred from the value alone.

The shipped `l2cmd` file is cut short: its section-header table starts at `446,168` and has 39 entries of 40 bytes, which ends past the file's 446,880 bytes, so `readelf` and `objdump` cannot read its sections. The program headers and the loaded segments are intact, which is why the comparison above holds. The extra size lies after the loaded segments; that it is debug information is inferred from the larger header count.

## The text command, as `l1cmd` sends it (`sn1SCTextCmd`, 6.5.30)

`sn1SCTextCmd` (`0x10009650` to `0x10009b4c`) was read in full, with the request writer `sn1SCReq` (`0x100082e0`), the waiting wrapper `sn1SCCmd` (`0x10008f40`), the reply reader `sn1SCRsp` (`0x10008560`), `L1DirectRequest` (`0x1000bf30`) and the packet maker the client library links (`0x1000fe10`), all established. The packet layer is the one described for the L2 side in [l2/router-packet.md](l2/router-packet.md): the same segment layout, with the command halfword at `+0x22`, the descriptor count at `+0x20` and eight-byte descriptors from `+0x24`.

Request (established):

1. The routine copies the text with `strdup`, builds two descriptors in a stack array and a 160-byte reply buffer (allocated with `malloc`, its pointer stored through the caller's reply argument).
2. Descriptor 0 is all zero: type 0, an inline integer, value 0. Descriptor 1 has type byte `0x0a`, which is type 2 with the free-after-copy bit set; its length halfword is the text length plus one (the terminating zero is sent) and its pointer is the duplicate.
3. The call is `sn1SCReq(destination, command 0, descriptor count 2, descriptors, sequence-number slot)`. `L1DirectRequest` stores the library's sequence halfword (context `+6`) through the slot, then the packet maker builds one packet with class 0 (request), command halfword 0, the destination word copied to packet `+0x1c`, and the two descriptors. On the direct-serial path `L1DirectSendPacket` sends it; on the router path an indirect call to the router request routine does.
4. If the text contains the substring "reboot" anywhere, the request goes through `sn1SCReq` alone and the routine returns 0 without waiting for a reply (a failure is formatted as "Command ERROR: %s" into the reply buffer and its code returned). Every other text goes through `sn1SCCmd` with a wait argument of 30 (its unit was not read).

Wait and match (established, `sn1SCCmd`): after sending, it calls `sn1SCRsp` for one reply, and compares the reply's sequence (packet byte `+0x14`, plus byte `+0x21` shifted left 8, as read by `sn1SCRsp`) with the sent number masked to 8 bits, or to 16 bits when packet byte `+0x12` is non-zero. A mismatch logs "request ID did not match response ID" and waits for the next packet with no retry limit visible in the routine; a match returns 0. Any error from either call is logged and returned.

Reply decode (established):

- Packet byte `+0x13` is an error byte. A non-zero value N makes the routine return `-600 - N` after formatting "Command response ERROR: %s" into the reply buffer. The reply side sets this byte as `(-code) mod 100` in `L1DirectResponse` (`0x1000c010`), so a controller error code `-(600+N)` travels as N.
- On success the routine frees the reply buffer, then walks the reply's descriptors in order into a growing text buffer (`BuffWriteInit`, `BuffWrite`, `BuffWriteDone`). Type 2 descriptors are fetched with the argument accessor `IRArg` and appended as strings, with a newline added when the string does not end in one. Type 3 descriptors are handed to `FRUDetach` and, when it returns a pointer, `FRUDumpAll` prints them through the same buffer writer (their record format was not read). Type 0 and other types add no text.
- The finished text pointer is stored through the caller's reply argument. The return value is argument 0 of the reply: its inline integer when descriptor 0 has type 0, otherwise whatever `IRArg` returns for it. This matches the note in [l1/debug-task.md](l1/debug-task.md) that the L1 answers with a status first and the output lines after.

One dead branch: for a type 2 descriptor the routine tests the type twice, and the inline-integer branch after the second test (`0x10009970` to `0x10009988`) can never run.

`l2cmd`'s `main` is the same code (established above: its loaded segments equal `l1cmd`'s except the destination word), so the whole of this section holds for `l2cmd` with destination `0x1ffff003`.

## `flashsc` (6.5.30)

Read from the usage and message strings (established that the strings exist; the code behind each option was not read in 6.5.30; the SG0007149 build is described in [l1/package-container.md](l1/package-container.md)):

- Usage: `flashsc [-p [<maxgrp>]] [-f] [sysco options] <bin image> <target>`, and `flashsc -version|-v|-vv <bin image>`.
- `-p [<maxgrp>]`: flash controllers in parallel instead of one at a time, optionally at most `maxgrp` at once; a bad value prints "invalid <maxgrp> argument for -p".
- `-f`: force the update, skipping the version check. The string table also has `-forceall`.
- `-l2recover`: emergency L2 flash recovery over a serial connection chosen with `--serial` or `--dev`; it needs an L2 firmware image in the binary and cannot update several bricks at once over serial.
- `<target>`: `<rack>.<slot>`, a list of those, `all` or `local` (for an L2). The string table also holds the words `router`, `compute`, `other`, `l2local` and `local`, the options `-router`, `-io`, `-other`, `-test`, `-manual`, `-new`, `-image`, `-boot`, `-bootldr` and `-compute`, and the messages that go with `-image` ("invalid image (%d) selected with -image option") and `-manual` ("MANUAL FLASH IMAGE SELECTED").
- `-n`: named in two notices that say it is forced when a system mixes legacy and newer bricks, and that a lone legacy brick that will join newer hardware needs it; it makes the legacy brick take firmware that can talk to the newer hardware.
- The refusal text for an L1 flash binary reads that it does not contain both the "A" and "B" images and also contains no relocation tables, so the tool cannot use it (established that the strings exist and are one message; the test that selects it was not located, so whether an image with relocation tables but one slot is accepted is not read); a multi-image binary lists its images (`Image %d:`).
- The banner reads "System Controller Flash Utility - Version 1.4.1"; `-version` prints "Supports flash binary rev. %d".

### The update requests (`FlashUpdateRemote`, `0x1000ebf0` to `0x100103c0`, 6.5.30)

Read from the call list and the request set-up around each call (the whole routine was not read line by line, so each item is graded on what was read). The routine sends four kinds of request to the controller's flash task through the same `sn1SCCmd` wrapper `l1cmd` uses, each with two descriptors (established for the three below, from the argument registers at the call sites):

| Step | Request code | Descriptors | Reply wait | Where (6.5.30) | Grade |
|---|---|---|---|---|---|
| status | `0x0072` | two integers, 0 and 5 (see [l1/package-container.md](l1/package-container.md)) | 30 | call at `0x1000ecb0` | established (code, descriptor count, wait): re-read in the SG0007149 build, below |
| erase | `0x0074` | integer 0: the target address of the image; integer 1: a byte count | 140 (`0x8c`) | call at `0x1000f79c`, set-up at `0x1000f738` to `0x1000f784` | established (code, descriptor count, wait): re-read in the SG0007149 build |
| program | `0x0073` | integer 0: image address plus the offset already written; type 1 payload: the next chunk of the image, its length in the descriptor's halfword | 30 | call at `0x1000fcb8`, set-up at `0x1000fc40` to `0x1000fca8` | established (code, descriptor count, wait): re-read in the SG0007149 build |
| validate | `0x0072` | as the status request | 30 | call at `0x10010080` | established: the code is the status code repeated, not a fourth code |

Check of the codes (gap-fill read, `flashsc` from patch SG0007149, the one build available for this pass; the 6.5.30 binary was not, so the 6.5.30 addresses above are carried from the earlier reading and were not re-checked). In that build the update routine passes the request code in the second argument register and the wait in the sixth, and every request has two descriptors. Status: code `0x72`, wait `0x1e` (30), the descriptor pair set up with 0 and 5. Erase: `0x74`, wait `0x8c`. Program: `0x73`, wait `0x1e`. After the program loop, at `0x10010288` in that build, the routine builds the same descriptor pair (0 and 5) with the same two-attempt loop and sends `0x72` again, wait `0x1e`: so the validate row repeating the status code is right, and the validate step is a second status read (inferred that it is the post-programming check, from its position after the program loop and the matching failure messages; the reply tests were not read). The call addresses in that build differ from the 6.5.30 ones in the table (status call `0x1000eed0`, erase `0x1000f9bc`, program `0x1000fed8`), so the two builds are not byte-identical; code, count and wait are the same.

Details (established from the instructions at the addresses above):

- Erase slices are one tenth of the image length, or a caller-supplied slice length when one is passed in (the word at stack offset 80 tested at `0x1000f69c`); the last slice is cut so the sum does not pass the image length. A percentage line is printed between slices.
- Program chunks are the caller's chunk size when it is given, else 2000 bytes when the record at offset 12 of the first argument holds 1 and 4096 bytes otherwise; the last chunk is cut to the bytes left. The chunk is fetched from the image file with `FlashImageData` before each request, and a failed fetch prints "ERROR retrieving flash image data!" and returns -1.
- Every `sn1SCCmd` in the routine is wrapped in a two-attempt loop: on a failure it logs the error text, logs that it waits, calls `sleep(15)` and tries once more, then gives up (the loop counter is tested against 2 at `0x1000f774`, and the same shape is at the other three call sites).
- Messages in the routine's string pool (inferred as the outcomes of tests made before erasing; the tests themselves were not read): "CONTROLLER MISMATCH!", "this file does not contain the correct image type for this brick!", and a skip notice when the running version equals the new one. After programming, a second status request checks the image: "Image 'A' is INVALID" and "revision mismatch" are the two failure messages. The decoding of reply arguments 6 to 12 (via the argument accessor) was not read argument by argument here.
- `FlashUpdateRemoteBoot` (`0x100103c0`) and `FlashUpdateRemoteRaw` (`0x100112e0`) use the same status, erase (`0x74`) and program (`0x73`) requests with the same call shape; `FlashUpdateRemoteBoot` passes no chunk-size or progress arguments to `FlashImageData` (the argument registers at `0x10010ea4` are zero), and the Raw variant has no brick-string comparison among its calls (inferred from the call lists).

What the controller does with these requests is in [l1/package-container.md](l1/package-container.md) for `0x72` and in [l1/flash-nvram-and-image-records.md](l1/flash-nvram-and-image-records.md#programming-and-erase-requests-0x0073-and-0x0074) for `0x73` and `0x74` (the handlers at `0xffe3c4c2` and `0xffe3c41e` in the Origin 300 package). The option-to-code mapping of each `flashsc` flag (`-p`, `-f`, `-boot`, `-image`, `-manual`) is not read; only the three update routines above are.
