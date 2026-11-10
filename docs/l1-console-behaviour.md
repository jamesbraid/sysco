# Origin 300 L1: what the console prints and answers

This note describes what the Origin 300 L1 firmware sends on its operator console, and what it answers, in each console state: at power-up, at the banner, at the idle prompt, for each command family and for error replies. It is written so that two controllers (the shipped firmware and any other implementation of the same console) can be compared transaction by transaction: a byte sent, the bytes returned.

## Scope and image

Everything below is read from the Origin 300 L1 1.44.0 ColdFire body (package 1, `Fuel/PE/O300 1MB`, body sha256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`, loaded at `0xffe00000`; see [l1/package-container.md](l1/package-container.md)). Addresses are first-bank addresses; the second bank is the same code `0x70000` higher. Strings were read at body offset = address minus `0xffe00000` and the command table was decoded from body offsets `0x1c0a0` to `0x20eb8`. Brick type character `4` (the Origin 300 board personality) is assumed unless a row says otherwise. Other packages and versions were not read for this note.

Grades follow [AGENTS.md](../AGENTS.md): established (read in the image, whole routine), inferred (reasoned from partial evidence), unknown. Each unknown names the software test that would settle it.

Terms: the "console" here means the L1's own operator serial port, called the system management port in the firmware's log texts. It is not the processor console, which the L1 relays when it is in console mode. The "text-command task" is the request endpoint that runs the same commands for host tools; it is covered in [l1/debug-task.md](l1/debug-task.md) and only touched here where a console reply differs.

Hints (not findings, from public manuals): the Origin 300 guide `007-4498-002` printed pages 181 and 184 show example command output; the controller guide `007-3938-006` printed pages 9 to 12 name the console subchannels and printed page 60 lists the valid brick types.

## 1. The serial line

| Fact | Grade | Evidence |
|---|---|---|
| The console is software port 5, channel 2 of the four-channel UART, opened at nominal 38,400 baud (actual about 37,500), no parity, no flow control, client class 2 (console). | established | [l1/quad-uart-and-serial-links.md](l1/quad-uart-and-serial-links.md), table rows for port 5; open call at `0xffe54be0`-`0xffe5518e`. |
| The console task is created at `management_port_init` (`0xffe36f74`): a read task, a write task and an input task, with three queues and two semaphores. | established | `0xffe36f74`-`0xffe372fe`. |
| Data bits and stop bits are those of the driver's settings block; this note did not decode them. 8N1 is the expected setting. | inferred | Parity none is read; the block's other fields were not decoded. Test: read the settings block passed at the port-5 open and the line-control writes in `0xffe636b0`-`0xffe64100`. |
| After five consecutive failed UART writes with no byte received between them, the writer discards all further output silently. A received byte clears the failure count. | established | Counter at `0x30000b08`: tested at `0xffe36ad8`-`0xffe36ade` (five or more skips the write and returns the length), incremented at `0xffe36b1a`, `0xffe36b60`, `0xffe36b8c`, cleared in the read callback at `0xffe36aac`. |

## 2. The line has two framings

The console runs in one of two framings, held in the word at `0x30005140+0x330`.

- Text framing (value 0, the value after boot): single bytes are read one at a time and fed to the console state machine at `0xffe378ac` (`0xffe38116`-`0xffe38208`). Established.
- Packet framing (value 1): the same UART is read as framed packets for host tools. A received byte `0x18` (control-X) while in text framing echoes the byte back and switches to packet framing (`0xffe3793e`-`0xffe379d6`; the switch reconfigures the UART client through `0xffe643cc` and `0xffe643e8`). The route back to text framing, and the exact packet format, are not covered here (see [l1/rs422-router.md](l1/rs422-router.md) and [l1/chunked-request-frames.md](l1/chunked-request-frames.md)). Echo and switch: established. Return to text framing: unknown; test: find stores of `0` to `0x30005140+0x330` and read the callers.

Output newline handling depends on the console state (section 3): in console mode (state 1) text goes out byte for byte; in the two L1 states every line feed in a message is sent as carriage return plus line feed (`0xffe36b22`-`0xffe36b92`, the two-byte string at `0xffe38869`). Established.

## 3. The console state machine

State word at `0x30005140+0x334`. Values and meaning (established from `0xffe378ac` branches and `0xffe36fc6`):

| State | Name used here | Entered |
|---|---|---|
| 3 | locked L1 prompt | at `management_port_init` (value `3` stored at `0xffe36fc8`) |
| 2 | escaped | from console mode on control-T (`0xffe37a3e`-`0xffe37a40`) |
| 1 | console mode | from state 3 on control-D (`0xffe37b60`); from state 2 after one command (`0xffe37c4a`) |

Transitions, one input byte at a time (all established unless stated):

1. State 3, control-D (`0x04`): if the brick type character is not one of a fixed set of 17 type characters (the set is compared at `0xffe37a88`-`0xffe37af6`; `4` is in it) the L1 prints `no console available on this brick` plus a line feed and stays. Otherwise, if the node-power test `0xffe4c2fc` does not return 1, it prints `WARNING: power appears off, console unavailable` plus a line feed. Then, in state 3, it prints a line feed, `entering console mode NAME, <CTRL_T> to escape to L1`, a line feed, and moves to state 1. NAME is the current console selection text from `0xffe30dec` (for the default selection this is a text of the form ` CPU0` to ` CPU3` or ` console`, strings at `0xffe312af`-`0xffe312c7`; whether the leading space is kept in the output was not traced; unknown, test: run the format at `0xffe376ba` and dump `0x30004f36`).
2. State 3, control-D when already in state 1 or 2: not applicable (state 1 forwards the byte to the processor, state 2 does only the warning check; see below).
3. State 1, control-T (`0x14`): prints a line feed, `escaping to L1 system controller`, a line feed, then the prompt string (section 4), clears the line buffer, moves to state 2. Any other byte in state 1 is sent to the selected processor console as console input through `0xffe37710`; on a routing failure the L1 prints `ERROR: can't route console input` followed by the router status text, a comma and a second status text (format at `0xffe38812`/`0xffe3881a`, built at `0xffe37796`-`0xffe377b4`; second status text unknown). The brick-type-dependent routing is not covered here.
4. States 2 and 3, carriage return (`0x0d`) or line feed (`0x0a`): if the line buffer is non-empty it is copied into the history ring first (section 5). The L1 then echoes a bare line feed (`0xffe3870b`), submits the line to the command parser (section 6), and clears the line buffer. After that:
   - State 3: the prompt string is printed again; the state stays 3.
   - State 2: if node power is not on, the power warning is printed; then a line feed, `returning to console mode NAME, <CTRL_T> to escape to L1`, a line feed; state becomes 1. So an escape runs exactly one line, including an empty one, and then returns to the processor console.
   An empty line is not submitted (`0xffe377f6`-`0xffe377fc` returns zero before building a request), so an empty line prints only the echoed line feed and, in state 3, a fresh prompt.
5. States 2 and 3, backspace (`0x08`): if the buffer is non-empty, remove the last character and send the three bytes backspace, space, backspace (`0xffe38746`); if empty, nothing is sent.
6. States 2 and 3, control-P (`0x10`) or control-N (`0x0e`): history navigation (section 5).
7. States 2 and 3, any other byte: stored in the buffer and echoed as itself, up to the buffer limit. When the buffer is full (limit at `0x30005140+0x387`), the new byte replaces the last stored character and the L1 sends a backspace followed by the new byte, so the screen shows the replacement (`0xffe37d5a`-`0xffe37d98`). The buffer begins at `0x30005140+0x338`, so the room is `0x4f` characters; the exact accepted line length is therefore 79 characters (inferred from the two addresses; test: send 80 characters and read the echoed bytes).
8. Control-X (`0x18`) in text framing in any state switches framing (section 2).
9. Control-D in state 2: inferred to do nothing beyond the power warning; the code at `0xffe37b2a` skips the rest of the control-D branch unless the state is 3. Test: run state 2, send `0x04`, and record the output.

Not echoed and not otherwise handled bytes: every byte other than the ones above is stored and echoed, including escape sequences, so arrow keys produce visible junk characters. Inferred from the absence of other compares in `0xffe37a78`-`0xffe37d98`; test: send `0x1b 0x5b 0x41` in state 3 and read the bytes returned.

## 4. Power-up and the banner

What the firmware prints, in the order it does it (established for the code path, inferred for what a terminal sees):

1. `main` (`0xffe10570`) calls `console_printf` with the format `L1 booting %s` plus a line feed and the version text `1.44.0` (`0xffe10586`-`0xffe10592`). Established. `console_printf` formats into a stack buffer and calls `0xffe59f00`, which appends the text to the L1 log ring (`0xffe5aed8`-`0xffe5aefc`, `0xffe59f00`-`0xffe5a17c`). That routine's callees are the clock read, string length, a semaphore, the ring writer and a checksum; no UART write was found in it. Conclusion: established that `console_printf` output reaches the log ring and not the serial line at the time of the call; this explains why `L1 booting 1.44.0` is the first entry the `log` command shows and why it does not appear at power-up on the wire. Inferred, not established: that no other path later drains the ring to the console (test: grep the listing for readers of the ring pointers at `0x300003a4+4`; only `log` at `0xffe5af04` was seen).
2. The management-port read task (`0xffe37f60`) opens its UART client and, if the flag word at `0x30000b24` is non-zero, sends the banner and clears the flag (`0xffe38070`-`0xffe38100`). The flag has the value 1 in the initialised data image copied to RAM at boot (data image at body offset `0x7f0e4`, flag at offset `0xb24`; copy described in [l1/board-boot-contract.md](l1/board-boot-contract.md)), and the one store of zero to it is the banner path. So the banner is sent once per L1 boot, before any byte has been received. Established for the code; the wire-level claim is inferred, because it depends on the write task running and a terminal being attached at the time. The text, with `\n` meaning line feed (and sent as carriage return plus line feed because the state is 3):

   ```
   \n\nSGI SN1 L1 Controller\n
   Firmware Image A: Rev. 1.44.0, Built 07/17/2006 18:20:38\n\n\n
   <prompt>
   ```

   The image letter is `A` when two words at `0x30000918+8` and `+0x10` are equal and `B` otherwise (`0xffe380a0`-`0xffe380bc`); what those words hold (the booted bank against the default bank) is inferred from the `flash status` text, unknown from this pass. The build stamp in the image is `20060717182038` (body offset `0x7a44c`); the format prints characters 4-5 as the month, 6-7 as the day, 0-3 as the year and then 8-9, 10-11, 12-13 as hour, minute, second, so the date prints as `07/17/2006 18:20:38`. Established arithmetic from the argument order at `0xffe380ba`-`0xffe380e2`; reading the stamp as year-month-day-hour-minute-second is inferred.
3. The prompt string (`0xffe36ec4`): a prefix chosen by the brick type character (`T1-` for `1`, `T2-` for `2`, empty otherwise: `0xffe36ef0`-`0xffe36f22`), then the rack, type letter and slot formatted as `%03.3d%c%02.2d` (`0xffe30a50`, format at `0xffe312d7`), then the literal `-L1>`. When the rack value is 0 its three digits are overwritten with `-`, and when the slot value is 0 the two slot digits are overwritten with `-` (`0xffe30a90`-`0xffe30ac2`). The letter comes from the routine at `0xffe60f94`, a jump table over the type character minus `0x23` (inferred from its prologue; table not decoded). Established for the layout. The letter for type `4` was not decoded; the firmware's own public examples show `001r08-L1>` (hint: [l1/usb-l2-revision.md](l1/usb-l2-revision.md)). Unknown: the letter for `4`; test: decode the jump table at `0xffe60fb6` for the index of `4` minus `0x23`.
4. The prompt is rebuilt by `0xffe36ec4`, which runs at port init and, per the call at `0xffe371c4`, again after configuration changes (call site list not complete; test: callers of `0xffe36ec4`). A change of rack, slot or type is stated by the firmware to take effect at the next L1 reboot (`brick rack set to NNN (takes effect on next L1 reboot/power cycle)` and the sibling texts), so the prompt text is expected to be unchanged until then. Inferred.

Display (not console): the front-panel display texts written during start-up (`  UNKNOWN   `, `BRICK TYPE!!`, `L1 running`, `L1 rebooted`; `0xffe106f2`-`0xffe10754`) are written to the two-line display, not to the serial line. Established by the callee name `display_text_write` (`0xffe5117c`).

## 5. History

Twenty entries of 80 bytes at `0x30005140+0x3ac` form a ring; the head index is at `+0x9ec` and a count at `+0x9f0` (`0xffe37b90`-`0xffe37bde`). On each submitted non-empty line the L1 stores it at the head, advances the head modulo 20 and increments the fill count up to 20. Control-P steps back one entry and control-N forward, erasing the visible line with backspace-space-backspace triples first, then loading and echoing the chosen entry (`0xffe37c9e`-`0xffe37d58`). Established for the ring and the erase-and-echo; inferred for the exact index arithmetic at the wrap (the code uses a signed divide by 20 and the stored remainder). The `history` command prints `History length: %d/%d` and a blank line, then one `%d: %s` line per entry (`0xffe19e40`). Test for the wrap: send 21 distinct lines, then control-P twenty-one times, and compare with the expected order.

## 6. Commands

The command table is a list of 68-byte (`0x44`) records between `0xffe1c0a0` and `0xffe20eb8`. Each record holds the command word list, argument types, a handler address, a brick-class string, a selector, a hidden flag and a help line. `0xffe61bd0` parses a line against the records; `help` (`0xffe17080` with the formatter `0xffe61658`) prints them. This section lists the replies that are fixed by the image. Replies that depend on hardware state (voltages, fans, sensors, eeprom, serial numbers, scan) are listed by command with their format strings only.

### 6.1 Parser replies (established)

- Leading white space is skipped. A line that is empty or only white space makes the parser return `-1201` and no text (`0xffe61c04`-`0xffe61c44`). From the console that case never reaches the parser (section 3.4).
- A line that matches no record: `ERROR: command not found.` plus a line feed, parser result `-1200` (`0xffe62018`; detail in [l1/debug-console.md](l1/debug-console.md)).
- A record that matched the words but whose arguments did not convert: the same `command not found` line, because later candidates clear the partial-match marker (debug-console.md). The text `ERROR: invalid arguments for 'CMD' command, try "help CMD"` (`0xffe62004`, formats at `0xffe620b0` and `0xffe62077`) is built only when a partial match survives to the end of the table.
- `help CMD` for an unknown name prints `ERROR: command CMD not found.` plus a line feed (`0xffe61a96`).
- A record whose brick class does not include the stored type character produces `INFO: command not supported on this brick type` plus a line feed from the class guard at `0xffe140d4` (the text is copied at `0xffe14190`); some handlers print `command not supported on this brick type.` instead (`0xffe14de0`). The class strings use a letter alphabet mapped through the table at `0xffe21014`; for type `4` the alias set was read only for the `c` class (debug-console.md).
- The word `~` before a first word in the guard string negates the test (`0xffe140f8`-`0xffe14104`). Established; the class string `~A` appears on `partdb`, `display` and the brick partition setter.

### 6.2 Commands that print a fixed or near-fixed reply (established from format strings; the text is in the image at the cited offsets)

| Command | Reply | Source |
|---|---|---|
| `version` or `ver` | `L1 1.44.0 (Image A), Built 07/17/2006 18:20:38    [Fuel/PE/O300 1MB image]` plus a line feed. The format is `%s %s (Image %c), Built ...`, first argument the text `L1`, then the version, the image letter, the date fields, then four-space-plus-bracket, the package label, a closing bracket and a line feed. | `0xffe3b87c`, format at body offset `0x3bd62`, label at `0x79a28`. |
| `debug` | `debug switches set to 0x%04x` plus a line feed (value-dependent). | `0xffe15214`, [l1/debug-console.md](l1/debug-console.md). |
| `help` or `hlp` | `Commands are:` plus a line feed, then command names in 19-character left-aligned columns. | `0xffe61658`; column wrap not decoded (inferred; test: run `help` and count columns). |
| `help hidden` or `help list hidden` | the same list including records flagged hidden. | table selector 1, `0xffe17080`. |
| `history` or `hist` | see section 5. | `0xffe19e40`. |
| `log` | the log ring, one entry per line, or `log is empty.`; `log clear` or `log reset` prints `log reset.`; `log insert TEXT` prints `log entry made.` and writes TEXT to the ring. | `0xffe1710c`. |
| `reboot_l1` with no argument | restarts the controller; the reply is not established to be seen (restart path in debug-console.md). Variants `a`, `b`, `current`, `old`, `new`, `other`, `addr N`, `raw N` print `Booting flash image %c` plus a line feed, or `Boot failed.   Flash image %c is invalid.` plus a line feed. | `0xffe3b3fc`. |
| `flash status` | a table headed `Image   Address       Status        Revision    Built` with a dashed line, one row per image, `*INVALID*` rows and a trailing `WARNING!  NO VALID BOOT FLASH IMAGE!!` when none is valid; `Flash image %c currently booted`; `Executing from debugger (RAM)`. The terse form omits the address column. | `0xffe3b3fc`, strings at `0x3bb17`-`0x3bca3`. |
| `flash default A`, `B`, `current`, `old`, `new`, `reset` | silent on success; `cannot set default--image %c is invalid!` or an `ERROR` line when the image table write fails. | `0xffe3b93c`. |
| `power up`, `power up hold`, `power down`, `reset`, `reset hold`, `nmi`, `softreset` | action texts are written through `console_printf` (`power up %s(%s)`, `power down (%s)`, `reset (%s)`, `NMI (%s)`, `soft reset (%s)`, `reset release (%s)`, `reset hold (%s)`), which goes to the log ring, not the serial line (section 4 step 1). The serial reply is only what the handler appends. | `0xffe4c62e`, `0xffe4c8b6`, `0xffe4ed0c`, `0xffe4ed62`, `0xffe4edc6`, `0xffe4ef5e`, `0xffe4ef9a`. Handler serial texts were not read here; unknown. Test: run each with the node off and on and capture the bytes. |
| `reset` while the node is off | `ERROR: power appears off.` plus a line feed on the paths that check power. | `0xffe198b8`, `0xffe1bc9e`, `0xffe1bee2`. |
| Soft reset and NMI on the display | display strings `Soft reset..`, `Sft rst fail`, `Sft rst done`, `NMI...`, `NMI fail`, `NMI done`; display only. | `0xffe261bd`-`0xffe26220` area, strings at `0x261bd`-`0x26220`. Which of them is also echoed on the serial line is unknown. |
| `select` | shows `console input: %s` and `console output: %s` and a filter state `DISABLED`, `filtered` or `not filtered`; `select RACK SLOT` and `select subchannel ...` change the selection and can print `INFO: console subchannel changed: %s`. Errors: `ERROR: no brick at that rack and slot`, `ERROR: brick at that rack and slot is not acceptable for console I/O`, `ERROR: bad subchanel selection.` (misspelling is in the image). | `0xffe19944`, strings `0x26229`-`0x262fb`. |
| `l1` | `L1 command processor engaged, <CTRL-D> to exit.` plus a line feed, and nothing else was traced. | `0xffe17b18`, string at `0x255d6`. Interaction of this command with the console state word was not traced; unknown. Test: send `l1` in state 3 and then control-D, and capture the bytes. |
| `date` | `Time: %m/%d/%Y %X %Z`, or raw seconds with `raw`; `date TZ` prints or sets the timezone string. | `0xffe1a478`, string `0x24f08`. |
| `config` or `cfg` | a table headed by rack, slot, partition, type and flash size text, e.g. ` [1MB flash]`, with source words `NVRAM`, `EEPROM`, `OTHER`; `config reset` prints `resetting L1 configuration`. | `0xffe15060`, strings `0x23f8c`-`0x24008`, `0x243ea`. |
| `brick` | prints the same record; setters print `brick rack set to %03d (takes effect on next L1 reboot/power cycle)` and the slot, type and partition variants (`0x2404b`-`0x242b2`). | `0xffe1445c`. |
| `cpu` | a table `CPU Present Enabled`, `--- ------- -------`, rows of `%c%c    %d       %d`. | `0xffe14d88`, strings `0x2437a`-`0x243b8`. |
| `ctc`, `nia`, `nib`, `cti`, `iia`-`iid`, `l2`, `*` | forwarded to a peer or the L2; reply shapes in [l1/debug-task.md](l1/debug-task.md) and [l1/power-reset-cbrick.md](l1/power-reset-cbrick.md). Errors: `ERROR: target brick not found`, `ERROR: PORT doesn't appear to be active`. | `0xffe14acc`, `0xffe14fa0`, `0xffe14a7c`. |
| `uart`, `usb`, `link`, `network`, `port`, `router`, `istat` | status tables; formats at `0x25ea9`-`0x26085` and elsewhere. | handlers `0xffe19bf0`, `0xffe19dac`, `0xffe17b58`, `0xffe19014`, `0xffe19874`, `0xffe19f08`, `0xffe191b0`. Not decoded line by line; inferred. |
| `env`, `fan`, `power` (no argument), `margin`, `pbay`, `leds`, `display`, `button`, `eeprom`, `serial`, `sysstate`, `partdb`, `autopower`, `syscom`, `nvram`, `ioport`, `pimm`, `l1dbg`, `test`, `scan`, `b2b` | hardware- or state-dependent; the table lists the command words and help lines. The format strings are in the image string area `0x23e22`-`0x26550`. | Not decoded line by line; inferred. |

### 6.3 Commands with hidden or odd words

Command records marked hidden (flag word at record offset `0x40` equal to 1) are omitted from the short `help` list: the `let ... carnage ... begin`, `slipped ... mind` and `super ... sized ... please` word chains, many `env`, `fan` and `l1dbg` sub-forms, the `ioport force` forms, `pimm`, `serial` set forms, `eeprom dimm unsign`, `nvram read|write|scratch`, `router 6|8 port`, `b2b` and `l2`. Established from the table scan. The word chains are exact: the three security chains read `let the carnage begin`, `slipped ... mind` (the middle word is not in the table; only `slipped` and `mind` are present as words) and `super sized please` (from the table's word strings at body offsets `0x219c8`-`0x21a3a`; the sibling note [l1/usb-l2-revision.md](l1/usb-l2-revision.md) shows `let the carnage begin` typed at a prompt). The replies of those three chains are unknown (handler `0xffe1a878` not read); test: run the handler's call list and format uses.

### 6.4 Catch-all

The words `fru`, `pci`, `prom` and `node` are registered as a last entry whose handler is `0xffe141b4`; its job is the unsupported-command guard (class string empty). When the stored type fails the guard, the reply is `INFO: command not supported on this brick type` plus a line feed. On a type that passes it, the handler returns success with no text. Established for the guard (`0xffe141b4`-`0xffe141da`); the consequence for `fru examine` and `pci` forms is inferred. A separate text `ERROR: unknown pci command.` exists (`0xffe11ca9`) in the PCI hook path.

## 7. Replies that never go to the console

Texts from `log_message` and `console_printf` (583 call sites; [l1/console-messages.md](l1/console-messages.md)) go to the log ring. The `log` command shows them and, when the `l1dbg printf` and `l1dbg console` switches are on, events carrying them are generated for the system side (`l1dbg` help lines: `enable/disable console tracking messages`, `enables/disables generating events on printf calls`). Whether any event reaches the operator's serial line is unknown; test: set `l1dbg console on`, trigger a log call, and capture the line.

## 8. Comparison checklist

To compare a second controller transaction by transaction, drive these in order and compare bytes exactly:

1. Reset the controller with the node off and record every byte on the console until idle. The expected sequence, if the banner is sent at task start, is section 4 step 2 (inferred as to wire timing).
2. Send a bare carriage return. Expected: one line feed (`\r\n` in states 2 and 3) and the prompt.
3. Send `version`. Expected: the line in section 6.2.
4. Send `nosuchcommand`. Expected: the echoed characters, `\r\n`, `ERROR: command not found.\r\n`, the prompt.
5. Send control-D with the node off. Expected: the power warning line, then `entering console mode ...`.
6. Send control-T. Expected: `\nescaping to L1 system controller\n` and the prompt; then one command; then `returning to console mode ...`.
7. Send `debug`, `debug 0x2a`, `log`, `history`, `flash status`.
8. Send 21 lines and navigate the history ring.
9. Send control-X and confirm the echo and the switch to packet framing.

## Summary of unknowns

| Item | Test that would settle it |
|---|---|
| Wire timing of the banner (sent at boot, or only after the first byte) | Run the 1.44.0 body under a ColdFire model with the quad UART channel 2 captured and record the bytes after reset. |
| Letter used in the prompt for type `4` | Decode the jump table at `0xffe60fb6`. |
| Leading space in the console name inside `entering console mode ...` | Dump the buffer at `0x30004f36` after `0xffe30dec` returns. |
| Output of control-D in state 2 | Run state 2 with the node on and off and capture. |
| Serial reply of `power`, `reset`, `nmi` and `softreset` handlers | Read the handlers `0xffe19740`, `0xffe1988c`, `0xffe1bc5c`, `0xffe1bea0` for their text outputs, then run each. |
| Interaction of the `l1` command with the state word | Run it and capture. |
| Route back from packet framing to text framing | Find stores of `0` to `0x30005140+0x330`. |
| UART data bits and stop bits | Read the driver's line-control writes for channel 2. |
| Handler texts of the hidden security chains | Read `0xffe1a878`. |
| Whether any path drains the log ring to the serial line | List readers of the ring pointers; run with `l1dbg console on`. |
| Other packages and versions | Repeat the reads at the same offsets in the 1.48.1 package of the same layout. |
