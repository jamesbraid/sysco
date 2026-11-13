# Challenge and Onyx host side in IRIX 5.0 to 6.4

[README.md](README.md) and the notes beside it read the IRIX 6.5 Foundation 1 builds. This note reads the builds of the earlier system discs, decoded as described in [inst-compression.md](inst-compression.md), to see which parts of the wire protocol and the driver are old and which came later. Evidence is the shipped binaries of each release; per-release sizes are the `size(N)` of the index. Addresses are virtual addresses in the daemon of the release named (load base `0x400000` in the 5.x and 6.x daemons here, `0x10000000` in the 6.5 daemon). Grades follow [../../AGENTS.md](../../../AGENTS.md).

## What each release ships

Daemon `usr/etc/sysctlrd` (32-bit big-endian MIPS executable, stripped, dynamic symbols kept; MIPS-I up to 6.1, MIPS-II from 6.2):

| Release | Bytes | Option string | Routines gained since the row above (dynamic symbol names) |
|---|---:|---|---|
| 5.0 | 36,880 | `dgprnestf:v` | base set (below) |
| 5.1.1 | 40,136 | `dgprnestf:v` | `cmd_len`, `set_length_mode`, `what_length_mode`, `printrawinfo` (replaces `rawenvinfo`) |
| 5.2 | 44,408 | `dgprnestf:v` | `check_sysctlr_rev`, `check_ccrev`, `clk_tck` |
| 5.2 for Onyx | 44,424 | `dgprnestf:v` | as 5.2 |
| 5.3 | 44,512 | `dgprnestf:vh` | `printpbstatus` |
| 6.0, 6.0.1 | 41,748 | `dgprnestf:v` | none against 5.2; no `printpbstatus` |
| 6.1 | 41,892 | `dgprnestf:vh` | `printpbstatus` (5.3 has it, 6.0 does not) |
| 6.2 | 41,892 | `dgprnestf:vhk` | `power_down` |
| 6.3 | 41,892 | `dgprnestf:vhk` | none; rebuilt (different bytes, same size) |
| 6.4 for Origin | 42,352 | `dgprnestf:vhk` | none; carries debug sections |
| 6.5 | 57,504 | `dgprnestf:vlhk` | `blower_a_faults`, `blower_b_faults`, `elapsed_minutes`, `log_vdc_delta`, `on_initial_setup`, `powersupply_names`, `should_log_message`, `update_timer`, `voltages` |

**Established** from the dynamic symbol tables and the option strings. The routine sizes of the common set change release by release (for example `getenvinfo` 540 bytes in 5.0, 900 in 5.1.1 to 6.1, 868 in 6.2 and 6.3, 816 in 6.4, 3,568 in 6.5) and were not compared instruction by instruction except where stated below.

Message tool (`etc/scmsg` in 5.0, `sbin/scmsg` after): 5.0 10,040 bytes; 5.1.1 9,968; 5.2 9,984; 5.2 for Onyx 9,984; 5.3 10,056; 6.0 and 6.0.1 8,936; 6.1, 6.2 and 6.3 9,076; 6.5 9,596.

Driver object `sysctlr.o`:

| Release | IP19 | IP19, 64-bit kernel | IP21 | IP25 |
|---|---|---|---|---|
| 5.0 | 8,764, 32-bit ECOFF, installed as `usr/sysgen/boot/sysctlr.o` | | | |
| 5.1.1 | 8,948, ECOFF | | | |
| 5.2, 5.2 for Onyx | 8,784, ECOFF | | | |
| 5.3 | 7,160, ECOFF | | | |
| 6.0 | 8,784, ECOFF | 12,616, 64-bit ELF (`IP1964boot`) | 12,720, 64-bit ELF | |
| 6.0.1 | | 12,616 | 12,696 | |
| 6.1 | 11,360, 64-bit ELF | | 11,456 | |
| 6.2 | 11,672 | | 11,496 | 11,432 |

The IP21 object first appears in 6.0 and the IP25 object in 6.2. The 6.5 sizes are in [README.md](README.md).

The shipped header `usr/include/sys/EVEREST/sysctlr.h` is revision 1.8 in 5.0 (4,074 bytes) and revision 1.9 from 5.1.1 to 6.5 (4,288 bytes, byte-identical across all of them). Revision 1.9 adds four codes to 1.8: `C` (check command lengths, can be got and set), `K` (kill the event log, set only), `B` (blower RPM warning) and `S` (firmware reset warning). **Established** by comparing the files. The 1.9 additions line up with what the daemons do, below.

## The protocol is older than length mode

**Established** (5.0 daemon, read in full: `sendcmd` at `0x40286c`, 272 bytes; `getresult` at `0x40297c`, 976 bytes; `getlog` at `0x403d28`, 300 bytes; `getenvinfo` at `0x4036e0`, 540 bytes).

- **Request frame.** `sendcmd` takes the device descriptor, the kind (`G` or `S`), the item and a sequence tag. It stores `0x18` first (`0x4028a0`), then the kind (`0x4028a8`). Only for kind `G` (the compare with 71 at `0x4028a4`) does it store the tag next (`0x4028ac`); then the item, a line feed (`0x0a`, formed at `0x4028c8`) and a zero byte. It writes the frame through `protected_write` without the zero byte: five bytes for a get (`ESC G tag item LF`) and four for a set (`ESC S item LF`). There is no length field. Afterwards it sleeps one tenth of a second (`sysconf(3)`, divided by 10, `sginap`). A failed write returns 1 when the global that gates every log text of this daemon (read as the verbose flag) is zero; when it is set the routine logs, goes on to the sleep and returns 0 (exits at `0x40292c` and `0x402968`). The 6.5 form of the same frames, with the two length digits when length mode is on, is in [daemon-wire.md](daemon-wire.md).
- **Reply.** `getresult` takes the state, the item, the tag and the result buffer. Per attempt it sends the get, then reads a three-byte preamble one byte at a time (stopping early at a zero byte). The first byte is dispatched: `A` or `W` goes to `doalarm` (`0x402b40`) and the request is repeated without counting a failure; `R` continues; anything else is a failed attempt. For `R` the second byte must equal the tag and the third the item, each with a verbose-mode log text on mismatch. It then reads data bytes into the buffer until a zero byte, which is stored too. Each failed attempt calls `resync` with the flag set; after two failed attempts the routine returns 1 (exit at `0x402c84`). A failed data read returns 1 at once (`0x402d10`). Success returns 0 (`0x402cc4`). So a reply in this build is `R tag item data NUL`, with no length digits, and the retry limit is two where the 6.5 daemon makes five.
- **Event log.** `getlog` asks item `L` with the tag `L` (`0x403d5c`, `0x403d64`), drops the last character of the text and cuts it at each semicolon from the end, as in [records.md](records.md). It never sends a set of item `K`. In the 5.1.1 daemon the same routine sends `S K` with a zero tag after a successful read (`0x404b3c`-`0x404b4c`, argument `li a2,75` at `0x404b44`). The 5.0 controller log was therefore never cleared by the daemon; from 5.1.1 it is cleared after each good read. **Inferred**: the daemon's later "unsolicited log read" behaviour (read the log after an unrecognised alarm, then clear it) depends on this clear.
- **Environment reply.** `getenvinfo` asks item `E` with a tag held in the state at +52 and splits the answer at semicolons. In 5.0 the loop ends at ten fields (compares at `0x403880` and `0x403894`) and stores format level 1 at record +0 when it got exactly ten, otherwise it logs a short-reply text and stores 0. Fields 0 to 5 and 7 to 8 go to record offsets `8 + 4n` (fixed point for 0 to 5, integer for 7 and 8), field 6 (fixed point) to +32 and field 9 to +44 (`0x403824`, `0x403864`). These are the first ten fields of the 6.5 table in [records.md](records.md). The extended reply (26 fields, format level 2) is absent: the compare constants 25, 26 and 10 appear together in the 5.1.1 routine (`0x40451c`-`0x4045c0`) and in every later build, and nowhere in 5.0. The firmware revision field (field 10) only exists in the longer reply, so a 5.0 daemon has no firmware revision check: the revision text "should be at least 2.02" is first present in 5.2.

**Established** by byte search of every build: the length-mode strings (`06Y`, `%2x`, "Result length mismatch!") are absent from 5.0 and present in 5.1.1 and every later daemon; the `K` item is first sent in 5.1.1; the texts "Firmware compensating for blower RPM problem." and "System controller firmware reset." are absent from 5.0 and present from 5.1.1. In 5.0 the alarm routine has explicit compares for the warning letters `C`, `I`, `T` and `V` only (`0x402354`-`0x402374`). **Inferred**: the `B` and `S` warnings of header revision 1.9 are first handled in 5.1.1 (the strings are there; that routine was not read through for those letters).

So the request set the host ever sends is the one in [daemon-wire.md](daemon-wire.md); what changes across releases is the length field, the log clear, the longer environment reply and the extra warnings. A controller built for 5.0 needs only plain frames.

## scmsg in 5.0 writes a complete frame

**Established** (`etc/scmsg` 5.0, `main` at `0x400820`, 288 bytes; builds 5.1.1 to 6.2 read at their `addiu a2,s1,4` before the write).

The 5.0 tool builds `0x18`, `'S'`, `'M'` (`0x400830`-`0x400860`), copies the argument after them one byte at a time (stores at offset 3 onward, `0x400898`), puts a line feed after the text (`0x4008cc`) and writes the frame with a count of the text length plus four (`addiu a2,s1,4` at `0x400900`): the three leading bytes, the text and the line feed, so the whole frame. It does not test the argument count, caps nothing (the frame buffer on its stack is 4,096 bytes and the copy has no bound), prints no usage text and, if `/dev/sysctlr` cannot be opened (`open(path, 2)` at `0x4008d4`), returns -1.

Every build from 5.1.1 on (read: 5.1.1, 5.2, 5.2 for Onyx, 6.0, 6.0.1, 6.1 at the write count; 6.2 `main` at `0x400b00` in full) has the usage text, the argument-count test, the 40-byte cap, the two length digits at offsets 3 and 4 formatted with `"%2x"` from the text length, the text from offset 5, and the same write count of the text length plus four. In those builds the count no longer covers the two digits, so the last text byte and the line feed are not sent (see [scmsg.md](scmsg.md)). The short write therefore began in 5.1.1, the release that added length-framed requests to the daemon; it is not a property of the original tool. The release in which `scmsg` got the digits is the release in which the count went wrong. What a controller does with the short, length-labelled frame stays **unknown** ([../open-questions.md](../../../questions/README.md)).

## The driver is unchanged

**Established** (5.0 IP19 object read for its register constants, ioctl numbers and the interrupt routine's tests; the others by symbol list and constants).

- The 5.0 object has the same twelve routines as the 6.5 objects (`sysc_clear`, `syscinit`, `syscopen`, `syscclose`, `syscread`, `syscwrite`, `syscioctl`, `sysc_lock`, `sysc_unlock`, `syscintr`, `sysctout`, `clear_tout`), named in the same order in the object.
- Register access. The 32-bit objects reach the UART through two external helpers named `load_double` and `store_double` (both are undefined symbols of the 5.0 object), where the 6.5 IP19 and IP25 objects use plain doubleword accesses and only the 6.5 IP21 object calls a `load_double` helper. The acknowledge store in the 5.0 object passes the value as the register pair zero and 4 (`li a2,0` and `li a3,4` at `0x100009ec`-`0x100009f4`), a 64-bit store of 4.
- Register addresses. The 32-bit objects of 5.0 to 6.0 form `0xb8000400` (`lui` and `ori` at `0x1000005c` and `0x10000064` of the 5.0 object; UART block) and `0xb8000850` (at `0x100009e4`-`0x100009e8`; written with the value 4 at `0x100009f4` as the last act of `syscintr`). `0xb8000000` lies in the uncached, unmapped 32-bit window, whose physical address is the value minus `0xa0000000`, so the UART block is at physical `0x18000400` and the acknowledge address at `0x18000850`. The 64-bit objects of 6.0 to 6.2 (IP19, IP21 and IP25 alike in form) hold the doublewords `0x9000000018000400` and `0x9000000018000850` for IP19 and IP25 and `0x9000000418000400` and `0x9000000418000850` for IP21 (the `.lit8` section of each). The inference in [driver.md](driver.md) that the 6.5 addresses are uncached windows onto physical `0x18000400` is thereby confirmed for IP19 from a 32-bit object whose window is architectural. IP21's fifth hex digit remains the only evidence for its different physical base.
- Status bits. The 5.0 `syscintr` tests bit 1 of the status word for a received byte (`andi 2` at `0x100008a4`) and bit 0 for transmitter ready (`andi 1` at `0x10000964`), compares the read count with 2,048 (`slti 2048` at `0x100008bc`) and masks the ring index with `0x7ff` (`0x100008fc`), as in the 6.5 objects.
- Ioctl numbers. The constants `0x7301`, `0x7302` and `0x7303` (29,441 to 29,443) are in the `syscioctl` of every 32-bit build read (5.0, 5.1.1, 5.2, 5.3, 6.0) and of the 6.1 and 6.2 objects, and the error return 22 for anything else.

**Inferred**: the host-visible behaviour of the driver (open once, ring and write buffer sizes, timeouts of 100 ticks) did not change between 5.0 and 6.5; the individual routines of the other builds were not compared against each other.

## What this closes and what stays open

- The lane's question "which requests does the host send" is answered across eleven releases: the set is fixed, only framing changed ([daemon-wire.md](daemon-wire.md)).
- The old image compression is settled in [inst-compression.md](inst-compression.md).
- Still open: the controller side (CPU, firmware) and the meaning of the reply to the length probe; see [../open-questions.md](../../../questions/README.md).
