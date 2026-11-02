# ELSC host request set

What the IRIX kernel's driver object for the Origin 200 / 2000 / Onyx2 module system controller (MSC, also called ELSC) sends it, how it frames the traffic, and how it reads the answer. Everything here is read from the host side: the controller's own firmware is not held (see [controllers.md](controllers.md), family "MSC and ELSC"), so a request's meaning beyond what the driver does with the reply is a reading, not a finding.

**Read.** `usr/cpu/sysgen/IP27boot/elsc.o` in the 6.5.30 set (`root/6.5.30/`): a relocatable big-endian MIPS 64-bit object, 38,240 bytes, not stripped, with the symbol table and debug sections. `.text` is 0x2230 bytes. Every routine below was disassembled whole; addresses are `.text` offsets. Callee names come from the relocations, not from the call target in the listing (the listing shows every `jal` as a call to offset 0). String addresses are offsets in `.rodata`; the first word of `.srdata` holds that section's base, and every format string is loaded as that base plus a constant. The same object was then read in the 6.4-origin and 6.5.7 sets and compared against every other copy in `root/` (section "Versions").

**Grades** as in [AGENTS.md](../AGENTS.md): established is a whole routine read and a claim that follows from it; inferred is a reasoned reading. Names of controller commands are the strings the driver sends, in quotation marks; they are the only names the controller ever sees.

## Transport

The driver talks to the controller over an I2C master it does not implement itself: the object imports `i2c_master_xmit`, `i2c_master_recv`, `i2c_master_xmit_recv` and `i2c_errmsg`. The I2C bus number is the first halfword of the driver's state block (set by `elsc_init`). Three slave addresses appear:

| Slave argument | Used by | What is there | Grade |
|---|---|---|---|
| 8 | `elsc_command` | the controller's request mailbox: one write of the request packet and one read of the reply packet in a single combined transfer | established that the argument is 8 in the call; inferred that it is a 7-bit address |
| `0x50` or-ed with `(address >> 8) & 7` | `elsc_nvram_read`, `elsc_nvram_write` | a byte-wide non-volatile store with an 8-bit in-page address and eight pages (addresses 0 to 0x7ff); see "Non-volatile store" | established for the arithmetic; the part identity (a 2 KB serial EEPROM) is a hint only |
| 0x20 (32) | `elsc_ust_read`, `elsc_ust_write` | a single byte, written or read with no register address; what it holds is not named anywhere in the object | established for the call shape; unknown for the meaning |

Every transfer in the 6.5 objects carries a timeout argument of 10,000,000 (0x989680); the 6.4-origin object uses 1,000,000 (0xf4240) for the same argument. Whether the unit is microseconds is not shown by this object (the i2c routines are in the kernel proper).

## State block

`elsc_init` clears 632 bytes (0x278) of the caller's block. Every routine takes the block as its first argument, except the `elscuart_*` wrappers, which fetch it with the imported `get_elsc`. Offsets as the code uses them. Established unless marked.

| Offset | Size | Use |
|---|---|---|
| 0 | halfword | I2C bus number (`elsc_init` second argument) |
| 4 to 131 | 128 bytes | ring of received console bytes; written by `elsc_process` at index word +132, read by `_elscuart_readc`, `_elscuart_getc` at index word +136 |
| 140 to 267 | 128 bytes | ring of console bytes waiting to be sent; written by `_elscuart_putc` at index word +268, drained by `_elscuart_flush` at index word +272 |
| 276 to 403 | 128 bytes | ring of queued controller messages, stored as one length byte then that many characters; write index word +404, read index word +408 |
| 416 | doubleword | message callback function (stored by `elsc_msg_callback`) |
| 424 | doubleword | callback argument |
| 432 | word | `elsc_init` writes -1; no routine in this object reads it |
| 436 | word | `elsc_init` writes 1; the line-start flag of `_elscuart_putc` (1 means the next character starts a line) |
| 440 to 535 | 96 bytes | request text buffer; every request routine formats into it with `sprintf` |
| 536 to 631 | 96 bytes | reply text buffer; filled by `elsc_command`, read by `elsc_parse` and `elsc_process` |

All three ring indexes wrap at 128 (a signed remainder done with the xor, subtract, and-0x7f idiom). The ring at 276 is empty when its two indexes are equal and is full at 127 bytes.

## Framing

**Request packet.** `elsc_command` copies the NUL-terminated text at +440 behind a length byte, then appends a checksum byte: `[n][n text bytes][c]`, where `n` is the text length and `c` is the two's-complement negative of the sum of `n` and the text bytes, so the whole packet sums to 0 modulo 256. Total bytes sent: `n + 2`. There is no limit test on `n`; the packet is built in a 96-byte stack area, so a text of about 94 characters or more overruns it (no caller in the object builds one that long). Established.

**Reply packet.** The same transfer reads up to 96 bytes. The routine accepts it only if, in order: the transfer returned a non-negative status; the number of bytes accepted by the send equals `n + 2` (otherwise -100); the number of bytes received is at least 2, equals the reply's first byte `m` plus 2, and `m` is below 96 (otherwise -112); and `m` plus the `m` text bytes plus the final byte sums to 0 modulo 256 (otherwise -111). On success the `m` text bytes are copied to +536 and NUL-terminated and the routine returns 0. A negative status from the transfer is returned unchanged. Established.

**Second argument.** `elsc_command` passes its second argument through as the last stack argument of the transfer routine. Every caller passes 0 except `elsc_process` (the `get` poll), which passes 1. What the transfer routine does with it is not in this object. Unknown.

## Reply grammar

`elsc_parse(state, out1, out2, out3)` classifies the reply text at +536 and splits up to three words. Established.

- Text starting `err` (three characters compared): the characters from offset 4 on (the character at offset 3 is skipped unread) are compared in turn with `csum` (four characters; return -101), `cmd` (three; -102), `arg` (three; -103), `perm` (four; -104) and `pwr_state` (nine; -105); anything else returns -112. The `pwr_state` test exists only from the 6.5.5 set on.
- Otherwise text starting `ok` (two characters compared): the third character must be NUL (return 0, no words) or a space; anything else, such as `okay`, returns -112. After the space the words are separated by single spaces; a word ends at a space or NUL, and exactly one following space is skipped. A leading space therefore gives an empty first word. Each output pointer may be null; a null first pointer returns 0 without copying. The return value is the number of words copied: 1 when the text ends after word 1 or `out2` is null, 2 likewise, 3 when the third word was copied. The third word also stops at a space.
- Any other text returns -112.
- Before any of this, the first byte of every non-null output buffer is set to 0. The copies are not length-limited.

## Error codes

`elsc_errmsg(code)` maps -124 to -100 to the texts below by a 25-entry jump table; any other code is handed to the imported `i2c_errmsg`. Established. The same 14 codes map to the same texts, in the same order, in the 6.4-origin and 6.5.7 builds.

| Code | Text | Raised by |
|---|---|---|
| -100 | Command send error | `elsc_command`: bytes sent differ from the packet length |
| -101 | Command packet checksum error | `elsc_parse`: reply `err csum` |
| -102 | Unknown command | `elsc_parse`: reply `err cmd` |
| -103 | Invalid command argument(s) | `elsc_parse`: reply `err arg` |
| -104 | Permission denied | `elsc_parse`: reply `err perm` |
| -105 | no text here (falls to `i2c_errmsg`) | `elsc_parse`: reply `err pwr_state` (6.5.5 on) |
| -110 | System controller response timeout | not set in this object |
| -111 | Response packet checksum error | `elsc_command` |
| -112 | Response format error | `elsc_command`, `elsc_parse` |
| -113 | Response direction error | not set in this object |
| -120 | Message lost because queue is full | `elsc_process` |
| -121 | Timed out getting ELSC lock | `elsc_lock` |
| -122 | Error sending data | not set in this object |
| -123 | NIC protocol error | `elsc_nic_get` |
| -124 | Bad magic number in NVRAM | `elsc_nvram_magic` |

Codes -119 to -114, -109 to -105 and the rest map to `i2c_errmsg`; the `pwr_state` code -105 is among them. The codes marked "not set in this object" are in the table but nothing here returns them (the transfer routine in the kernel proper may).

## Request set

Every request is text in the form `word [args]`, sent by the routine named, answered with `ok ...` or `err ...` as above. The reply column says what the driver does with it, not what the controller sends. Routines listed under "Requests" are all in `root/6.5.30/usr/cpu/sysgen/IP27boot/elsc.o`.

| Request text | Arguments | Sent by | Reply handling | Return | First set | Grade |
|---|---|---|---|---|---|---|
| `ver` | none | `elsc_version` | one word copied to the caller's buffer | 0, or the error | 6.4-origin | established |
| `tas %c` | one character (the lock owner) | `elsc_lock` | first character of word 1 | 0 if it is `0` (48) or equals the owner character; else retry after `us_delay`, then -121 | 6.4-origin | established |
| `tas 0` | none | `elsc_unlock` | `ok` only | 0, or the error | 6.4-origin | established |
| `dsc %x %c` | position (hex), character | `elsc_display_char`, `elsc_display_digit` | `ok` only | 0, or the error | 6.4-origin | established |
| `dsp ` + 8 characters | message, space-padded or cut to 8 | `elsc_display_mesg` | `ok` only | 0, or the error | 6.4-origin | established |
| `pwr q` | none | `elsc_power_query` | word 1 read as hex | the hex value, or the error | 6.4-origin | established |
| `pwr r` | none | `elsc_rpwr_query` | words 1 and 2 | see the routine entry | 6.5.5 | established |
| `pwr d %x` | one value (hex) | `elsc_power_down` | `ok` only | 0, or the error | 6.5 | established |
| `pwr c` | none | `elsc_power_cycle` | `ok` only | 0, or the error | 6.5 | established |
| `rst` | none | `elsc_system_reset` | `ok` only | 0, or the error | 6.5 | established |
| `rsw` | none | `elsc_dip_switches` | word 1 read as hex | the value xor 0xff, or the error | 6.4-origin (without the xor) | established |
| `nic %x %x %x` | three values (hex), supplied by the NIC library | `nic_access` | word 1 read as hex | the value, or the error | 6.4-origin | established |
| `hbt %d %d` | two decimal values | `_elsc_hbt` (second argument non-zero) | `ok` only | 0, or the error | 6.5 | established |
| `hbt` | none | `_elsc_hbt` (second argument 0, third non-zero) | `ok` only | 0, or the error | 6.5 | established |
| `hbt 0` | none | `_elsc_hbt` (both 0) | `ok` only | 0, or the error | 6.5 | established |
| `acp %d ` + data | the sender's id (see the entry), up to 86 console bytes | `_elscuart_flush` | word 1 read as hex: how many bytes the controller took | 0, or the error | 6.4-origin (id as one digit) | established |
| `get` | none | `elsc_process` | raw reply text; not run through `elsc_parse` | 0, the error, or -120 | 6.4-origin | established |

"First set" is the oldest set in `root/` whose copy of the object contains the request: 6.4-origin and 6.4 for the first group; 6.5 and 6.5.2 for the second; 6.5.5 for `pwr r` (see "Versions"). The meanings of the reply words beyond these uses (units and bit meanings of the hex fields) are not decided by the driver and are unknown.

Meanings: the driver's own names give `ver` (controller version string), `tas` (test-and-set lock with an owner character), `dsc` and `dsp` (display character and message), `pwr` (power: `q` query, `d` down, `c` cycle, `r` remote query), `rst` (system reset), `rsw` (rotary or DIP switch read), `nic` (read through the controller from a number-in-a-can part), `hbt` (heartbeat) and `acp` (console passthrough). These are readings from the routine names and call sites. Inferred.

## Routines

One entry per routine in the lane. Sizes are the symbol sizes. A routine called "static" is local to the object. All are in `usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30 unless the entry says otherwise.

### Message and state

- `get_myid` (static, `usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x0`, 16 bytes). Loads the 64-bit word at the address held in the first `.lit8` slot, the constant `0x9200000001000020`, and returns its low 32 bits sign-extended. Used as the sender id in `acp`. That address is in the uncached window of a node's local hub registers; the register is identified as the processor number by its use (inferred), not by a name in the object. Established for what it loads; inferred for what the register is.
- `elsc_init` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x10`, 68 bytes). Calls `bzero` on the block for 632 bytes, stores -1 in the word at +432 and 1 in the word at +436, and stores the second argument as the halfword at +0. Established.
- `dequeue_msg` (static, `usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x54`, 200 bytes). Takes the state block, a destination and its size. Returns -1 at once if the queue ring is empty. Otherwise it reads the length byte, advances the read index, and copies characters until the record is used up, storing only the first `size - 1` of them; it always consumes the whole record, writes the terminating NUL, and returns 0. If the ring runs dry in the middle of a record it returns -1 without a terminator. A zero-length record gives an empty string and returns 0. Established.
- `elsc_msg_check` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x11c`, 36 bytes). Calls `dequeue_msg` with its own second and third arguments and returns 1 if that call succeeded and 0 if it returned negative. It therefore takes the message out of the queue into the caller's buffer; it does not merely test. Established.
- `elsc_msg_callback` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x140`, 128 bytes). Stores the function at +416 and its argument at +424. If the function is non-null it drains the queue: for each message it calls `fn(argument, buffer)` with a 96-byte stack buffer (at most 95 characters survive), then dequeues the next, until the queue is empty. Returns 0. Established.
- `elsc_errmsg` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1c0`, 188 bytes). The table in "Error codes". Established.

### Packet path

- `elsc_command` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x464`, 400 bytes). Framing and reply checks as in "Framing". The transfer call passes the bus number, slave 8, the packet and its length, a word for the bytes sent, a 96-byte reply buffer, 96 as its size, a word for the bytes received, then on the stack 2, 10,000,000 and the second argument. Established.
- `elsc_parse` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x5f4`, 740 bytes). As in "Reply grammar". It makes seven `strncmp` calls in the worst case (`err`, `ok`, `csum`, `cmd`, `arg`, `perm`, `pwr_state`). Established.
- `elsc_process` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x8d8`, 468 bytes). Formats `get` into the request buffer and calls `elsc_command` with second argument 1; a negative result is returned as is. If the reply starts with the four characters `acp ` it copies the reply from its seventh character (offset 6) to the NUL into the console-input ring at +4, one byte at a time through the ring's write index (the fifth character, the sender id, is not examined; nothing is added when the seventh character is NUL). Any other reply, empty ones included, is handed to the callback with the stored argument if one is set, and otherwise appended to the message ring as a length byte then the characters. With no callback set the append happens only if the text length is below the free space (127 minus the used bytes); otherwise nothing is stored and the routine returns -120. The `acp` copy reads offset 6 without checking that the reply is that long, so bytes left from an earlier, longer reply can be taken, and the console-input ring has no full test: a write past the read index overwrites unread bytes. Returns 0 on every other path. Established.
- `elsc_version` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xaac`, 108 bytes). `ver`; the first word of the reply goes to the caller's buffer, the other two output pointers are null. Returns 0 on success even though the parse return is the word count. Established.
