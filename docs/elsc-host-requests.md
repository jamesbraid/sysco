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

### Non-volatile store

- `elsc_nvram_write` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x27c`, 252 bytes). Arguments: state, address, buffer, length. A negative length means a string write of `min(-length, strlen + 1)` bytes. Each byte is a separate two-byte transfer to slave `0x50 | ((address >> 8) & 7)` carrying the low address byte then the data byte; the address increments after each. The first transfer failure ends the loop and its negative status is returned; otherwise the last transfer's status is returned (a length of 0 returns an uninitialised stack word). There is no delay between bytes in this routine. Established.
- `elsc_nvram_read` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x378`, 236 bytes). Same addressing; each byte is a combined transfer sending the low address byte and receiving one byte. A negative length means a string read: after storing each byte it looks at the next byte of the destination (not the byte just stored) and stops when that is 0 or when `-length` bytes have been read, so a NUL in the data stops the read only if the buffer already holds a zero after it, and no terminator is written. The first failure returns its negative status; otherwise the last transfer's status (a length of 0 returns an uninitialised register). Established.
- `elsc_nvram_magic` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xb18`, 68 bytes). Calls `elsc_nvram_read` with address 1792 (`0x700`) and length 1, into a one-byte stack slot, and compares that byte with `0x37`: it returns 0 if the byte matches, -124 if not, and the read's negative status if the read failed. The routine itself makes no absolute memory access. All the get and set families below call it first. Established.
- `elsc_debug_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xb5c`, 120 bytes). Checks the magic, then writes its second argument to the byte at 1797 and its third to the byte at 1798. Established.
- `elsc_debug_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xbd4`, 120 bytes). Checks the magic, then reads the byte at 1797 through its second argument and the byte at 1798 through its third (both pointers). Returns the first failure or 0. Established.
- `elsc_module_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xc4c`, 76 bytes). Checks the magic, then writes its second argument as one byte at 1800. Returns the first failure or 0. Established.
- `elsc_module_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xc98`, 72 bytes). Checks the magic, then reads one byte at 1800; returns the byte, or the failure code from the magic check or the read. Established.
- `elsc_bist_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xf30`, 76 bytes). Checks the magic, then writes its second argument as one byte at 1801. Returns the first failure or 0. Established.
- `elsc_bist_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xf7c`, 84 bytes). Checks the magic, then reads one byte at 1801. Both failure paths (the magic check and the read) return the code masked with 0xff, so -124 comes back as 0x84 where every other getter returns the negative code; success returns the byte. Established.
- `elsc_partition_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xce0`, 76 bytes). Checks the magic, then writes its second argument as one byte at 1802. Returns the first failure or 0. Established.
- `elsc_partition_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xd2c`, 72 bytes). Checks the magic, then reads one byte at 1802; returns the byte, or the failure code from the magic check or the read. Established.
- `elsc_domain_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xd74`, 76 bytes). Checks the magic, then writes its second argument as one byte at 1803. Returns the first failure or 0. Established.
- `elsc_domain_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xdc0`, 72 bytes). Checks the magic, then reads one byte at 1803; returns the byte, or the failure code from the magic check or the read. Established.
- `elsc_cluster_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xe08`, 76 bytes). Checks the magic, then writes its second argument as one byte at 1804. Returns the first failure or 0. Established.
- `elsc_cluster_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xe54`, 72 bytes). Checks the magic, then reads one byte at 1804; returns the byte, or the failure code from the magic check or the read. Established.
- `elsc_cell_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xe9c`, 76 bytes). Checks the magic, then writes its second argument as one byte at 1805. Returns the first failure or 0. Established.
- `elsc_cell_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xee8`, 72 bytes). Checks the magic, then reads one byte at 1805; returns the byte, or the failure code from the magic check or the read. Established.
- `elsc_password_set` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x12c4`, 132 bytes). After the magic check it copies up to 4 characters of the string, padding with spaces from the first NUL, and writes the 4 bytes at 1793. Established.
- `elsc_password_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1348`, 76 bytes). Checks the magic and returns its error on failure; otherwise reads 4 bytes from 1793 into the caller's buffer and stores a NUL after them (the NUL is stored before the read starts). Returns the read's status. Established.

The map as the object uses it, with every address in page 7 of the store (slave `0x57`):

| Address | Size | Content | Routines |
|---|---|---|---|
| 1792 (`0x700`) | 1 | magic byte `0x37` | `elsc_nvram_magic` |
| 1793 to 1796 | 4 | password, space padded | `elsc_password_set`, `elsc_password_get` |
| 1797, 1798 | 1 each | two debug bytes | `elsc_debug_set`, `elsc_debug_get` |
| 1799 | 1 | not touched by this object | |
| 1800 | 1 | module number | `elsc_module_set`, `elsc_module_get` |
| 1801 | 1 | BIST setting | `elsc_bist_set`, `elsc_bist_get` |
| 1802 | 1 | partition | `elsc_partition_set`, `elsc_partition_get` |
| 1803 | 1 | domain | `elsc_domain_set`, `elsc_domain_get` |
| 1804 | 1 | cluster | `elsc_cluster_set`, `elsc_cluster_get` |
| 1805 | 1 | cell | `elsc_cell_set`, `elsc_cell_get` |

The field names are the routine names (inferred); the addresses and widths are established. The kernel proper may use other addresses through `elsc_nvram_read` and `elsc_nvram_write`, which are exported.

### Lock, display, power

- `elsc_lock` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0xfd0`, 280 bytes). Arguments: state, delay, timeout, owner character. Formats `tas %c` with the owner, then loops: `elsc_command`, `elsc_parse` into a stack buffer; any negative result returns it. If the first character of word 1 is `0` or equals the owner it returns 0. Otherwise it calls `us_delay(delay)` and repeats until the time passes the deadline, then returns -121. The deadline is the timeout argument plus 800/1000 of the 64-bit word at the address in the second `.lit8` slot (`0x9200000001030100`), read at entry; after each delay the word is read again, scaled the same way, and the loop gives up when it is no longer below the deadline. The 0.8 scale and the address in the same local-register window suggest a free-running hub counter of 1.25 MHz (inferred). Established for the loop; inferred for the counter.
- `elsc_unlock` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x10e8`, 104 bytes). Sends `tas 0`, parses with no output words, returns 0 or the error. Established.
- `elsc_display_char` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1150`, 116 bytes). Sends `dsc %x %c` with the second argument in hex and the third as a character. Returns 0 or the error. Established.
- `elsc_display_digit` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x11c4`, 72 bytes). Turns a value into a character (below 10: `0` plus the value; otherwise `a` plus the value minus 10 when the fourth argument is non-zero, `A` plus the value minus 10 when it is zero) and calls `elsc_display_char` with the position and that character, returning its result. Values above 15 give characters past `f`; nothing limits them. Established.
- `elsc_display_mesg` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x120c`, 184 bytes). Builds `dsp ` followed by exactly 8 characters: the message up to its NUL or 8 characters, then spaces to fill. NUL at offset 12 of the request buffer. A null message is not tested for: the routine would read address 0. Established.
- `elsc_power_query` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1394`, 128 bytes). Sends `pwr q`, parses one word, returns `strtoull(word, 0, 16)` truncated to 32 bits and sign-extended. A reply whose word is empty returns 0; an error returns its negative code, which a caller cannot tell from a large hex value. Established.
- `elsc_rpwr_query` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1414`, 224 bytes). Sends `pwr r` and parses two words. It picks the second word when the byte at the absolute address -15069 (0xffffffffffffc523, no relocation on it) is non-zero and the routine's second argument is 0, otherwise the first. If the chosen word starts with `n` it returns -1; if it contains `fl` it returns 0; else if it contains `ok` it returns 1; else -1. A failure of the command or of the parse returns that negative error code. What the absolute byte is, is unknown. Established for the control flow; unknown for the byte.
- `elsc_power_down` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x14f4`, 108 bytes). Sends `pwr d %x` with its second argument in hex. Established.
- `elsc_system_reset` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1560`, 104 bytes). Sends `rst`. Established.
- `elsc_power_cycle` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x15c8`, 104 bytes). Sends `pwr c`. Established.
- `elsc_dip_switches` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1630`, 132 bytes). Sends `rsw`, parses one word, returns `strtoull(word, 0, 16)` xor 0xff. Established.
- `_elsc_hbt` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x17c8`, 176 bytes). With a non-zero second argument sends `hbt %d %d` with the second and third arguments; else with a non-zero third argument sends `hbt`; else `hbt 0`. Returns 0 or the error. Established.

### NIC and test byte

- `nic_access` (static, `usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x16b4`, 148 bytes). Sends `nic %x %x %x` with the three arguments after the state block, parses one word, returns `strtoull(word, 0, 16)` truncated to 32 bits. It is the access function that `elsc_nic_get` and `elsc_mfg_nic_get` register with the NIC library. Established.
- `elsc_nic_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1748`, 128 bytes). Calls `nic_setup` with a stack block, the address of `nic_access` (the second `.srdata` slot) and the state block; zeroes the first doubleword of the caller's buffer; calls `nic_next` with the buffer plus 2. A positive library result becomes -123; a negative one passes through; 0 is success. Established.
- `elsc_mfg_nic_get` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x217c`, 40 bytes). Calls `nic_info_get` with the `nic_access` address, the state block and the caller's buffer, and returns its result. Established.
- `elsc_ust_write` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x21a4`, 60 bytes). Writes its second argument as one byte to slave 0x20 with no register address and returns the transfer status. Established; what the byte means is unknown.
- `elsc_ust_read` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x21e0`, 80 bytes). Reads one byte from slave 0x20 into the caller's variable (through its second argument) whether or not the transfer succeeded, and returns the transfer status. Established; meaning unknown.

### Console path

The controller carries the node's serial console. `_elscuart_*` hold the logic; the `elscuart_*` wrappers fetch the state block with `get_elsc` and call them. Console bytes travel in `acp` requests (host to controller) and in the reply to `get` (controller to host).

- `_elscuart_probe` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1878`, 36 bytes). Calls `elsc_version` into a stack buffer; returns 1 if it succeeded, 0 if not. Established.
- `_elscuart_init` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x189c`, 68 bytes). Clears three 136-byte regions starting at +4, +140 and +276 (the two console rings with their index words, and the message ring with its indexes through +411). Established.
- `_elscuart_poll` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x18e0`, 88 bytes). Returns 1 if the input ring is not empty; otherwise calls `elsc_process` once and returns 1 if that filled it, else 0. The `elsc_process` result is not looked at. Established.
- `_elscuart_readc` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1938`, 68 bytes). Returns -1 if the input ring is empty; otherwise the next byte, advancing the read index. Established.
- `_elscuart_getc` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x197c`, 140 bytes). Blocking: while the ring is empty it calls `elsc_process`; a result of -10 is ignored and retried, any other negative result returns -1. Returns the byte read. Established.
- `_elscuart_puts` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1a08`, 112 bytes). Calls `_elscuart_putc` for each character; a null string prints `<NULL>`; the first failure returns -1; otherwise 0. Established.
- `_elscuart_putc` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1a78`, 792 bytes). Appends to the output ring at +140. A line feed is stored as carriage return then line feed and the ring is flushed after it; a carriage return sets the line-start flag. At the start of a line (flag at +436 set) it first stores a prefix of the form `1A` plus a space plus three decimal digits of the bus number plus colon and space, for example `1A 000: `. Before writing the prefix it calls `_elscuart_flush` if fewer than 8 bytes are free; before writing the character itself, if fewer than 2 are free; when 84 or more bytes are already pending it first appends a carriage return and a line feed and flushes, whatever the character is. A flush failure returns -1; success returns 0. Established; the meaning of the prefix is inferred.
- `_elscuart_gets` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1d90`, 164 bytes). Line input with echo: reads with `_elscuart_getc` until a carriage return, echoing each character with `_elscuart_putc` and flushing; on the carriage return it echoes it and a line feed, stores a NUL and returns the buffer. No length limit. Established.
- `_elscuart_flush` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x1e34`, 460 bytes). While the output ring holds bytes: formats `acp %d ` with `get_myid` into the request buffer, copies up to 86 ring bytes (handling the wrap with two copies) to offset 446, NUL-terminates, sends it with `elsc_command` and parses one word. The word, read as hex, is the count the controller took; if it is non-negative and no larger than what was sent, the ring's read index advances by it. A command or parse error returns that error. A count of 0, or one larger than what was sent, leaves the ring unchanged and the loop sends again with no delay and no bound. The data offset 446 lines up with the end of `acp N ` only when the id has one digit. Established.
- `elscuart_probe` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x2000`, 36 bytes). Calls `get_elsc` and passes its result to `_elscuart_probe`. Established.
- `elscuart_init` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x2024`, 48 bytes). Calls `get_elsc` and passes its result to `_elscuart_init`; the caller's first argument is passed on as the second. Established.
- `elscuart_poll` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x2054`, 36 bytes). Calls `get_elsc` and passes its result to `_elscuart_poll`. Established.
- `elscuart_readc` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x2078`, 36 bytes). Calls `get_elsc` and passes its result to `_elscuart_readc`. Established.
- `elscuart_getc` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x209c`, 36 bytes). Calls `get_elsc` and passes its result to `_elscuart_getc`. Established.
- `elscuart_puts` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x20c0`, 48 bytes). Calls `get_elsc` and calls `_elscuart_puts` with that block and the caller's string. Established.
- `elscuart_putc` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x20f0`, 48 bytes). Calls `get_elsc` and calls `_elscuart_putc` with that block and the caller's character. Established.
- `elscuart_gets` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x2120`, 56 bytes). Calls `get_elsc` and calls `_elscuart_gets` with that block, the caller's first argument as the buffer and the caller's second argument as a third argument (which `_elscuart_gets` does not read). Established.
- `elscuart_flush` (`usr/cpu/sysgen/IP27boot/elsc.o`, 6.5.30, `0x2158`, 36 bytes). Calls `get_elsc` and passes its result to `_elscuart_flush`. Established.

## Versions

All the copies in `root/` were checked for routine names, string tables and callees. Findings by set.

- **6.5.30 against 6.5.7.** The same 64 routines in the same address order, the same requests, the same reply strings, the same error table and the same callee lists. The routines differ in size (6.5.7 is 39,800 bytes, 6.5.30 38,240) because the objects were built by different compiler releases (the compiler tags in the objects read 7.2.1 and 7.30; 6.4-origin reads 7.10); the visible differences are instruction order and the order in which the error handlers sit. No request was added or dropped between them. Established by comparing routine names, `.rodata` strings and callee lists for every routine; the instruction streams were not compared one by one.
- **6.5.5, 6.5.6, 6.5.9, 6.5.10 against 6.5.7.** Identical by the same comparison (same compiler tag). 6.5.5 differs from them in the size of `_elsc_hbt`, `_elscuart_flush` and `_elscuart_puts` by 4 to 8 bytes each: 6.5, 6.5.2 and 6.5.5 keep the strings `hbt`, `hbt 0`, `<NULL>` and `acp %d ` in the small-data section and reach them gp-relative, where 6.5.6 and later load them from `.rodata`. Inferred that the behaviour is the same (instruction streams not compared one by one).
- **6.5 and 6.5.2 against 6.5.7.** They hold 63 routines, not 64. They lack `elsc_rpwr_query`, the strings `pwr r` and `fl`, and the `pwr_state` branch of `elsc_parse`. Everything else matches. So `pwr r` and the `err pwr_state` reply (code -105) arrive between 6.5.2 and 6.5.5. Established for the symbol and string differences.
- **6.4-origin and 6.4 against 6.5.7.** The two copies have the same routine names. This is a different driver shape:
  - No state-block argument. Routines fetch it with the imported `get_elsc_space` (the 6.5 routines take it as an argument, and the `elscuart_*` wrappers fetch it with `get_elsc`). `elsc_command` takes the request text and a reply buffer as arguments and builds the packet from the text in place; `elsc_parse` takes the reply text as its first argument. The state block is 440 bytes (6.5: 632) and its bus halfword sits at +432. It has 38 routines to the 6.5 objects' 64. The packet transfer timeout is 1,000,000 (6.5: 10,000,000).
  - The requests present are `ver`, `tas %c`, `tas 0`, `dsc %x %c`, `dsp `, `pwr q`, `rsw`, `nic %x %x %x`, `get` and `acp `. Absent: `pwr r`, `pwr d`, `pwr c`, `rst`, `hbt`, `hbt %d %d`, `hbt 0`, and the whole `err pwr_state` branch.
  - The console routines exist as `elscuart_*` with the logic inline; there are no `_elscuart_*` routines. `acp` is built by hand: the four characters `acp `, then the id plus ASCII `0` as a single character, a space, then up to 86 bytes. So the id is one digit there; 6.5.30 formats it with `%d`.
  - `elsc_dip_switches` returns the hex value as it is, without the xor 0xff the 6.5 routine applies.
  - NVRAM: only the magic, password (1793), the two debug bytes and module (1800) are used; there are no partition, domain, cluster, cell or BIST accessors and no `elsc_display_digit`, `elsc_power_down`, `elsc_power_cycle`, `elsc_system_reset`, `elsc_rpwr_query`, `elsc_mfg_nic_get`, `elsc_ust_read` or `elsc_ust_write`.
  - It has two stub routines, `elsc_read_config` and `elsc_write_config`, which return 0 and nothing else; neither exists in any 6.5 copy.
  - The error codes and their texts are unchanged.
  Established for the routine, string and callee differences (names, strings, immediate values and calls compared for every routine); inferred for what the missing requests mean for 6.4-era controller firmware: that firmware either lacks them or the 6.4 kernel never issued them.

## Open points

- The controller's side of each request (what `pwr_state` means, what the words of `pwr q`, `rsw` and `nic` carry, what `fl` and `ok` in the `pwr r` reply stand for, the empty-reply convention on `get`). The MMSC image carries an MSC client for the same set and is the next route (lane 2 in [controllers.md](controllers.md)).
- The byte at absolute address -15069 in `elsc_rpwr_query`, the `ust` byte at slave 0x20, and the second argument of `elsc_command` are listed in [open-questions.md](open-questions.md).
