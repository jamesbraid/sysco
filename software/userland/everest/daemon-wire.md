# sysctlrd: how the daemon talks to the controller

Read from `root/6.5/usr/etc/sysctlrd` (IRIX 6.5 Foundation 1, 57,504 bytes, SHA-256 `c92b4b43c67fc22e...`), with the shipped header `usr/include/sys/EVEREST/sysctlr.h` and the man pages `sysctlrd(1M)` and `sysctlr(7M)` of the same set. Addresses are virtual addresses in that daemon (load base `0x10000000`). The daemon is stripped but keeps its dynamic symbol table, which names 47 of its own routines; the names below are those symbols. Argument registers follow the N32 convention (`a0` first). Grades follow [../../AGENTS.md](../../../AGENTS.md).

The device the daemon opens is `/dev/sysctlr` (string at `0x1001a0c8`), the character device of [driver.md](driver.md). Everything below is bytes written to and read from that device.

## Frames the daemon sends

All request frames start with the escape byte `0x18` (escape in the header) and end with a line feed `0x0a`. The second byte is `G` (`0x47`, get) or `S` (`0x53`, set). The next byte is an item code. Two further ASCII digits (a length) are inserted before the line feed when the daemon is in *length mode* (see below).

- **Get**, built by `sendcmd` (`0x10003b60`, 252 bytes): `ESC 'G' seq item [LL] LF`. `seq` is a one-byte sequence tag supplied by the caller. **Established** from the stores at `0x10003b70`, `0x10003b74`, `0x10003c50` and `0x10003ba8`.
- **Set without data**, also `sendcmd`: `ESC 'S' item [LL] LF`.
- **Set with a text payload**: `ESC 'S' item [LL] text LF`, built by `display_string` (`0x10002b0c`, 272 bytes), `settime` (`0x100042e8`, 264 bytes) and `dumpinfo` (`0x100043f0`, 796 bytes). **Established**; the item byte is a parameter of `display_string` (`'M'` and `'P'` at the call sites below) and fixed at `'T'` and `'H'` in the other two.
- **Length field** `LL`, only when the global `cmd_len` (`0x1001cd7c`) is non-zero: the daemon prints the number with `"%02x"` (format at `0x1001c7b8`) and stores the two characters in bytes 3 and 4. For `sendcmd` it forms the digits by hand: `'0'` and the character `0x31 + n` where `n` is 4 for a get and 3 for a set without data, which gives the digit strings `05` and `04`. For the payload builders it is `"%02x"` of `4 + payload length`. In every case the value is the number of bytes after the escape and before the line feed. **Established** for the arithmetic (stores at `0x10003ba4`-`0x10003bc4`, `0x10002b98`-`0x10002bb4`, `0x10004394`-`0x100043b4`, `0x100045c8`-`0x100045e8`); the reading "count of bytes after the escape" is **inferred** from those sums.

Every write goes through `protected_write` (`0x10002a64`), which calls `write` and treats a short count as a failure: it returns 1, and counts the failure in the daemon's state at +896 unless the CPU meter is off (`-n`); in verbose mode it logs "System controller write timed out." A complete write clears the failure count and returns 0. `sendcmd` follows each write with a pause of one tenth of the clock-tick rate. **Established.**

## Reading a reply: getresult

`getresult` (`0x10003c5c`, 1,212 bytes) takes the display state (`a0`), the item (`a1`), the sequence tag (`a2`) and a result buffer (`a3`). It returns 0 on success and 1 after five failed attempts. **Established** from the returns at `0x10003e14` and `0x10003f38`.

One attempt:

1. From the second attempt on, call `resync` (`0x100028c8`) and add one to the daemon's retry counter at +1160.
2. Send the get with `sendcmd`. If the write fails, write a single line feed (`0x0a`) to the device and retry.
3. Read a three-byte preamble, byte by byte, stopping early at a zero byte. A short preamble logs "Result preamble read timed out." or "Short response string." and retries.
4. Dispatch on the first preamble byte: `R` (`0x52`) is a reply; `A` (`0x41`) and `W` (`0x57`) are alarm and warning messages that can arrive in place of a reply (see below); anything else logs "Unexpected response code" and retries.
5. For `R`: the second byte must equal the sequence tag ("Bogus sequence number") and the third must equal the item ("Wrong request code").
6. In length mode read two more characters, parse them with `"%2x"` into a number, then read data bytes one at a time into the caller's buffer until a zero byte, which is stored too. A mismatch between that number and `(bytes read including the zero) + 4`, truncated to eight bits, logs "Result length mismatch!" and retries. In plain mode the data simply ends at the zero byte.

So a reply is `R seq item [LL] data NUL`. The two digits equal the number of bytes after the `R` up to and including the terminating zero: `seq`, `item`, the two digits, the data and the zero (**established** arithmetic from step 6: the loop counts the data bytes and the zero, and the compare adds four for `seq`, `item` and the two digits). With `cmd_len` not equal to 1 the daemon does not consume length digits from the reply; with `cmd_len` equal to 2 it still sends them, and `set_length_mode` uses that state to probe (below).

`resync` (`0x100028c8`, 172 bytes) writes one `0x1a` byte, waits one tenth of the tick rate, and if its flag argument is set logs "Resyncing the system controller." and calls `flush_buffer`. `flush_buffer` (`0x100027bc`, 268 bytes) asks the driver for the queued byte count (`ioctl 0x7301`) and, when non-zero, reads and discards bytes until a zero byte, logging a count and the text in verbose mode. `protected_read` (`0x10002974`, 240 bytes) calls `read` and treats a short count as failure. All **established**.

## Length mode negotiation

`set_length_mode` (`0x10004194`, 340 bytes) probes whether the controller speaks the length-framed form. It sets `cmd_len` to 2, asks for item `C` (the sequence tag is also `C`) and classifies the answer text with `what_length_mode` (`0x10004118`, 124 bytes): `N` is mode 0, `06Y` is mode 1 (strings at `0x1001c7d0` and `0x1001c7d8`), anything else is no match. Because `cmd_len` is 2 during the probe the daemon does not strip length digits, so `06Y` is the digits `06` followed by the data `Y`: a controller already speaking length-framed replies answers `R C C 0 6 Y NUL` (six bytes after the `R`, which agrees with the rule above), and `N` is the plain reply. On mode 1 the daemon sets `cmd_len` to 1 and stops. On mode 0 it sends a set of item `C` (no data) and asks again, up to three times. After three misses, or if a get fails, it sets `cmd_len` to 0 (plain frames). **Established** for the control flow and the framing of the two answers. **Inferred**: `Y` and `N` mean length mode on and off, and the set of item `C` turns it on.

The daemon calls it once at startup, right after the first `checkmsgs`.

## Item catalogue

The header names the item bytes; the table says where the daemon uses each. "Seen" means a call site was read; "header" means only the shipped header names it (a **hint**, not seen in the daemon).

| Item | Byte | Direction | Header meaning | Daemon use | Status |
|---|---|---|---|---|---|
| `C` | `0x43` | get, set | check | `set_length_mode` (above): get at `0x100041d8`, set at `0x1000428c` | established |
| `D` | `0x44` | get | debug switches | `logswitches` (`0x10006548`, 148 bytes) gets it at `0x1000656c`, parses the answer with `strtol` base 16 and logs "Debug settings: 0x%s" | established |
| `E` | `0x45` | get | environment | `getenvinfo` at `0x10005060`; `shut_down_power` at `0x100031c0` ("Current info"); record layout in [records.md](records.md) | established |
| `H` | `0x48` | set | histogram | `dumpinfo` builds `ESC S H [LL] v0,v1,... LF` with one comma-separated activity value per CPU (see below) | established |
| `K` | `0x4b` | set | kill log | `getlog` sends it at `0x10006368` after a successful log read, which clears the controller's log | established |
| `L` | `0x4c` | get | log | `getlog` at `0x10006248`; record layout in [records.md](records.md) | established |
| `M` | `0x4d` | set text | message | `display_string` with item `M`: progress text during shutdown ("Syncing Disks...", "Powering down...", "SYNC TIMED OUT!", "Sending hang-up...", "Killing proceses...") and the daemon's exit message "Sysctlrd killed!" | established |
| `O` | `0x4f` | set | off | `power_down` at `0x10002c88`; `wait_and_snooze` at `0x10002dcc` | established |
| `P` | `0x50` | get | panel type | `getscale` gets it at `0x10005e68` and `0x10005e8c`; the first byte of the answer selects the panel type, see below | established |
| `P` | `0x50` | set text | processor status | `wait_and_snooze` at `0x10002e44` sends one spinner character chosen from the four-character string at `0x1001c7c0` (`-`, `/`, `|`, `/`) by the low two bits of a counter | established |
| `T` | `0x54` | set | time | `settime`: `ESC S T [LL]` and twelve digits formed with `cftime` and `"%m%d%y%H%M%S"` (month, day, year, hour, minute, second) | established |
| `Z` | `0x5a` | set | snooze | `sendcmd` calls at `doalarm` (`0x1000367c`, `0x100036e0`), `wait_and_snooze` (`0x10002e00`) and the 27 call sites in `shut_down_power` (`0x10002f00` to `0x10003384`; every one is a snooze) | established |
| `S` | `0x53` | get, set | serial | not sent by the daemon; the serial-number check reads the PROM inventory through `syssgi` instead (`getpromerrs` at `0x10006494`, 180 bytes) | header |
| `T` | `0x54` | get | time | not sent | header |
| `E`, `D` | | set | environment, debug switches | not sent | header |

`dumpinfo` writes at most one value per CPU, in the form `scale * busy / total`, forced to at least 1, where `scale` is the panel scale stored at +868 by `getscale`, `busy` is the sum of the second, third, fifth and sixth counters and `total` adds the first and fourth (**established** arithmetic at `0x100044f4`-`0x10004560`; which scheduler counters those are is not in the daemon). With plain frames and fewer than 16 CPUs the list is padded with `,0` entries up to 16 for the `E`-type panel (`0x10004660`-`0x10004700`). `getcpuinfo` (`0x10003974`, 492 bytes) fills those counters with `sysmp` calls, command 15, for up to 36 CPUs (loop limit `0x10003b18`). All **established**.

`getscale` (`0x10005e18`, 624 bytes) asks item `P` up to three times. A first byte of `E` (`0x45`) stores 16 at +868 and half a second of ticks at +872; `T` (`0x54`) stores `0x48` at +868 and three quarters of a second at +872; failure or another byte stores 16 and three quarters of a second, and an unknown byte logs "Unknown display type". The options `-e` and `-t` preset the type to `E` or `T` and skip the question. **Established** for the stores; what 16 and `0x48` count is **inferred** (bar count for the small panel, a different scale for the large one; the code also keeps the type letter in the result record).

## Alarms and warnings

The controller can send unsolicited messages. In plain mode they are three bytes, a kind (`A` or `W`), a type letter and a zero byte; in length mode they carry two length characters after the type letter. `checkmsgs` (`0x1000377c`, 504 bytes) polls for them: it asks the driver for the queued count (`ioctl 0x7301`; a result of -1 logs "qlen ioctl not supported.  Exiting" and exits), reads three bytes (five in length mode) per message, and when the message is complete and starts with `A` or `W` calls `doalarm`. Anything else is flushed. `getresult` also recognises the same kinds in place of a reply preamble. **Established.**

`doalarm` (`0x10003464`, 792 bytes) takes the kind and type letter. Return value 0 means handled and -1 means unrecognised; after -1, `checkmsgs` and `getresult` read the controller's log (`getlog` with the `-1` argument, below). **Established** from `0x10003500` and `0x1000359c`.

| Kind | Type | Header meaning | Action in the daemon (strings at the cited address) |
|---|---|---|---|
| `A` | `T` | overtemperature | sends snooze (`S Z`), logs "Overtemp alarm!" (`0x1001a470`), prints it on stderr, runs `shut_down_power` |
| `A` | `O` | switch off | snooze, logs "Keyswitch off!" (`0x1001a4a8`), runs `shut_down_power` |
| `A` | `B` | blower | logs "Blower failure!" (`0x1001a4d8`), prints, runs `shut_down_power` (no snooze first) |
| `A` | other | | logs "Unknown alarm: 0x%x", returns -1 |
| `W` | `T` | watchdog timer | logs "COP timer reset error!" |
| `W` | `C` | oscillator | logs "System controller crystal oscillator failed!" |
| `W` | `I` | illegal opcode | logs "Firmware detected illegal opcode!" |
| `W` | `V` | voltage | logs "Voltage out of tolerance!" |
| `W` | `B` | blower speed | logs "Firmware compensating for blower RPM problem." |
| `W` | `S` | reset | logs "System controller firmware reset." |
| `W` | other | | logs "Unknown Warning: 0x%x", returns -1 |
| other kind | | | logs "Bogus alarm or error: 0x%x 0x%x" |

The priority numbers passed to `syslog` are 0 for the three named alarms, 3 for the timer, oscillator, illegal-opcode and reset warnings and for "Unknown Warning" and "Bogus alarm or error", and 1 for the voltage and blower warnings and for "Unknown alarm". The type letters match the header's definitions; the actions are the part read from the daemon. Both **established**.

## Power-down and shutdown

- `power_down` (`0x10002c1c`, 200 bytes): prints "sysctlrd: Powering down the system.", waits one second (the tick rate), sends set `O`, waits sixty seconds, prints "sysctlrd: WARNING: The system has not powered down!" and exits with status 1. **Established** (the wait is `clk_tck * 15 * 4` from `0x10002c94`-`0x10002cac`).
- `shut_down_power` (`0x10002ec0`, 1,444 bytes) is entered from `doalarm`, from the `-k` option and from the keyswitch path. It sends a long run of snoozes, reads item `E` once more for the "Current info" log line, and logs "Last info" from the shared state. With graceful power-down on (`-g`) it installs a child-exit handler, runs `/etc/killall HUP` (string at `0x1001a380`), shows "Sending hang-up...", waits, shows "Killing proceses...", forks to run `/etc/killall` and a synchronous `syssgi` sync, and `wait_and_snooze` (`0x10002ce4`, 448 bytes) loops showing "Syncing Disks...", sending `S P` with the spinner and snoozing until the child exits, or prints "sysctlrd: Sync timed out!" and shows "SYNC TIMED OUT!" before powering down. With graceful power-down off it goes straight to `power_down`. **Inferred** in order of steps from the call sequence; each call site and string was read, but the control flow between the snooze calls was not traced branch by branch.

## Startup order of the daemon

`main` (`0x1000691c`, 1,824 bytes) parses options with `getopt` and the string `dgprnestf:vlhk` (at `0x1001af00`). **Established** option actions: `d` requires a user id of zero and selects daemon mode; `k` requires user id zero ("Must be root to power down.") and selects an immediate power-down; `n` turns the CPU meter off; `g` turns graceful power-down on; `v` verbose; `l` voltage-extrema logging; `e` and `t` force the panel type to `E` or `T`; `f` scales the meter interval by `optarg/10`; `h` prints the piggyback-read status and exits; `p` and `r` select the human-readable and the raw print of the daemon's shared sensor record. The letter `s` sets one flag whose use was not followed. The man page documents `-d -g -n -v -l -p`; the rest are undocumented there.

In daemon mode, the calls are in this order (addresses of the calls in `main`): `initinfo` (`0x10006b30`), `getpromerrs` (`0x10006b3c`), `openport` (`0x10006b48`), `ioctl` with `SYSC_RTOUT` and 300 (`0x10006b50`), `checkmsgs`, `set_length_mode`, `getscale` (`0x10006b98`), `getlog` with -1 (`0x10006bb8`), `logswitches`, `settime`, `createshmem` (`0x10006bd0`), `ioctl` with `SYSC_RTOUT` and 100 (`0x10006be8`), then a loop that calls `checkmsgs`, `getcpuinfo`, `dumpinfo` and `getenvinfo` (`0x10006e4c`) followed by `copy_out_env`, and `check_sysctlr_rev` (`0x10006e98`) when the first reading worked. The sensor record is shared with `-p` clients through a 200-byte shared-memory segment (key `0x53637444`, stored in the daemon state at +884 by `main` at `0x10006a74`). `createshmem` (`0x100065dc`, 208 bytes) creates it with mode octal 1644 (create, read-write for the owner, read-only for others: the flag value 932 at `0x100065dc`) and attaches it; `attach_ro_shmem` (`0x100066ac`, 248 bytes) attaches it with mode 0644. `copy_out_env` (`0x10006838`, 76 bytes) copies the first 50 words (200 bytes) of the sensor record into the segment, and only when the record's first word is non-zero; `copy_in_env` (`0x10006884`, 152 bytes) copies the 200 bytes back, logging "Can't copy from shared memory.  Exiting." and exiting when the segment pointer is empty. **Established** for the order of calls and the copies; the loop's conditions were not traced.

## Versions

- The 6.4 Origin build of the daemon (`root/6.4-origin/usr/etc/sysctlrd`, 42,352 bytes) carries the strings `N`/`06Y`, `%2x` and "Result length mismatch" and so supports length mode; it lacks the option string of the 6.5 build (`dgprnestf:vl` and `dgprnestf:vlhk` are both absent). Checked by byte search, not read routine by routine.
- The builds of IRIX 5.0 to 6.3 are compared in [early-builds.md](early-builds.md). The 5.0 daemon has no length mode (frames and replies are plain, read in full); length mode, the log clear (`K`) and the 26-field environment reply arrive in 5.1.1. The 6.2 base daemon has the option string `dgprnestf:vhk`, not the 6.5 `vlhk`.
