# MMSC flash, NVRAM and the image download path

Read from the MMSC firmware image `usr/cpu/firmware/mmscfw.bin` in three IRIX releases, and from the host tool `usr/sbin/flashmmsc`:

| Image | MMSC version | sha256 | text / data / bss | Notes |
|---|---|---|---|---|
| 6.5.30 (byte-identical in 6.5.7, 6.5.9, 6.5.10) | 2.3 | `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc` | 687,064 / 166,912 / 123,288 | reference build; all addresses below are this one unless a build is named |
| 6.5.5 | 2.1 | `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9` | 687,032 / 166,912 / 117,328 | |
| 6.5.6 | 2.2 | `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752` | see [../inventory/images.tsv](../inventory/images.tsv) | |
| `flashmmsc` 6.5.30 | | `362d470b4fcc55ca7551c7d3949c9b92bc06e072e0b4d5264f449e6d7fddceb5` | MIPS N32, big-endian ELF | host half |

Method. The segment map is the one in [load-map.md](load-map.md). The firmware is a little-endian a.out. The text segment maps at `0x108000` straight after the 32-byte header and the data segment follows it directly (`0x108000 + 687,064` for 6.5.30). That layout is the one the 7,907 nlist symbols agree with: `_socketArray`, `_flashArray`, `_shortDelay`, `_longDelay` and `_CRCTab` decode to plausible tables only at that base, and the full 256-entry CRC table matches a computed one (below). Functions were disassembled linearly from the symbols and read whole; text constants sit inside the text segment after each routine and were read as data, not as code. Routine sizes are symbol to next symbol, so a routine's tail may include its own string constants. `flashmmsc` has no symbol table for its own routines; routine bounds come from its `.debug_frame` records, names from the order of its dynamic string table and from the debug messages each routine prints. Grades follow [AGENTS.md](../../AGENTS.md): **established** means the routine or table was read in full in the image named; **inferred** says what the reasoning is; **hint** is a fact from a public document that no image has confirmed; **unknown** points to [../open-questions.md](../open-questions.md).

## 1. Flash module (module `flash`, `0x10cc24` to `0x10d72c`)

Routines, all 6.5.30. In the flash and battery-backed store modules 6.5.6 sits at the same addresses and 6.5.5 `0x10` lower; the NVRAM and download routines are further apart (section 7 has the address table). The code is identical after addresses are removed (section 7).

| Routine | Address | Role |
|---|---|---|
| `_initFlash` | `0x10cc24` | fills the 13-word state record whose address is its argument |
| `_openFlashWindow` | `0x10cc94` | selects the page holding a block and returns a pointer into the window |
| `_closeFlashWindow` | `0x10cd14` | restores the window registers |
| `_clearFlashBlock` | `0x10cdc4` | erases one block |
| `_openFlash` | `0x10d004` | checks the two chips, optionally erases both |
| `_closeFlash` | `0x10d154` | closes the window, marks the record closed |
| `_readFlash` | `0x10d174` | copies bytes out through the window |
| `_writeFlashBytes` | `0x10d2b4` | programs bytes one at a time |
| `_writeFlash` | `0x10d3a4` | writes through the window from the current position |
| `_flashtest` | `0x10d594` | self test used from the debug shell |

### State record (established, `_initFlash` `0x10cc24`)

`_initFlash` stores, in order: `+0x00 = 0x550`, `+0x04 = 0` (open flag), `+0x08 = 2`, `+0x0c = 0` (byte position), `+0x10 = 0x8000` (block size), `+0x14 = 1`, `+0x18 = 0x760` (window select port), `+0x1c = 0x540` (window enable port), `+0x20 = 0x548` (window route port), `+0x24 = 0xa0000` (window base address), `+0x28 = 0x800` (size in KiB), `+0x2c = _socketArray` (`0x1b071c`), `+0x30 = _flashArray` (`0x1b075c`). Nothing in the module reads `+0x00`, `+0x08` or `+0x14` (inferred: `0x550` is an unused port base).

### Window, block size, total size (established)

- **Total.** `+0x28 << 10 = 0x200000`: 2 MiB.
- **Block.** Every read and write turns the position into a block number and offset with the divisor `+0x10 = 0x8000`: a 32 KiB unit, the granularity of the window API.
- **Window.** `_openFlashWindow(record, block, mode)` (`0x10cc94`) computes `block * 0x8000`; if that exceeds `0x200000` it returns a null pointer, otherwise it picks page `offset >> 17` (a 128 KiB page, 16 pages) and returns `0xa0000 + (offset & 0x1ffff)`. The test is "greater than", so an offset of exactly `0x200000` is accepted and selects page 16, one past the tables below. Before returning it writes port `0x548 <- 0xff`, then port `0x760 <-` the byte `_socketArray[page]`, OR-ed with `_flashArray[page]` when `mode == 0x40`, and sleeps `_shortDelay`.
- **Page table `_socketArray`** (6.5.30 `0x1b071c`, 16 words): pages 0 to 7 hold `0x80, 0x88, 0x90, 0x98, 0xa0, 0xa8, 0xb0, 0xb8`; pages 8 to 15 hold `0x01, 0x09, 0x11, 0x19, 0x21, 0x29, 0x31, 0x39`. So each chip's eight pages step by 8 in the byte written to port `0x760`, and the second chip is selected by the low bit instead of bit 7.
- **Write bit `_flashArray`** (`0x1b075c`, 16 words): `2` for pages 0 to 7 and `4` for pages 8 to 15. It is OR-ed in only for `mode == 0x40`, the mode every erase-command and program-command access uses; status polling and reads use mode `0xff` and so see the page without the write bit.
- **Close.** `_closeFlashWindow` (`0x10cd14`) writes `0x760 <- _socketArray[0]` (`0x80`), then `0x760 <- _socketArray[8]` (`0x01`), then `0x540 <- 0xff`, and sleeps `_shortDelay` (`{0 s, 100,000 ns}`, `0x1b070c`).
- **Not settled.** What sits behind ports `0x540`, `0x548` and `0x760`. The same three ports also switch in the battery-backed SRAM (section 3) with different values, so they are a shared memory-window controller; no manual in hand names it.

### Chip identification and command set

- **Established (`_openFlash` `0x10d004`).** The routine writes `0x90` to the first byte of window block 0, reads bytes 0 and 1, writes `0xff`, and requires `0x89` then `0xa2`; otherwise it returns -6 (`0xfffffffa`). It repeats the read for block `0x20` (byte offset `0x100000`, page 8); a mismatch there only clears an internal flag that makes the erase pass skip the second chip. So the flash is two 1 MiB chips side by side in the address space, the first mandatory and the second optional.
- **Hint (public Intel 28F008SA data sheet).** `0x89`/`0xa2` is that part's manufacturer/device pair, 16 blocks of 64 KiB, and the commands the code writes are that family's: `0x90` read identifier, `0xff` read array, `0x50` clear status, `0x20` and `0xD0` erase set-up and confirm, `0x40` program, `0x70` read status. The image never names the part; the physical erase block size of 64 KiB comes from the data sheet only.
- **Established erase stride.** `_openFlash` with mode 2 calls `_clearFlashBlock(record, 2*i, fd)` for `i = 0 to 15`, then, if the second chip answered, `2*i + 0x20`. Block numbers count 32 KiB units, so the erases land at `i * 0x10000` and `0x100000 + i * 0x10000`: 32 erases at a 64 KiB stride.

### Erase (`_clearFlashBlock`, `0x10cdc4`, established)

Up to as many attempts as tunable 38 allows (the erase attempt limit, default 1). Each attempt: open the block's window in write mode, write `0x50`, sleep `_longDelay` (`{1 s, 0}`, `0x1b0714`), write `0x20`, sleep, write `0xD0`, sleep. Between steps it prints `.` to the progress descriptor when that is non-negative and to the message log. Then it reopens the window in read mode and reads the status byte at the block start up to the poll limit times (the counter is decremented before each read, so the limit is the number of reads), stopping early when bit 7 is set. The timeout test is `counter == 0` after the loop, but an exhausted loop leaves the counter at -1, so only a ready status seen on the very last read reaches the timeout branch (log a timeout naming the block, return -7, `0xfffffff9`); a genuinely exhausted poll falls through to the bit-5 test on the last status byte read. If bit 5 is clear it prints `,` and returns 0 (so an exhausted poll whose last status had bit 5 clear counts as success). If bit 5 is set it logs an erase error naming the block and the poll count, prints `E`, and tries again; when the attempts are used up it returns -8 (`0xfffffff8`). No other status bit is tested. Three one-second sleeps per block over 32 blocks is the roughly 100-second wait the console notice announces (96 s of sleeping plus polls).

### Program, read, boundaries (established)

- `_writeFlashBytes(window, source, count)` (`0x10d2b4`), per byte: write `0x50` to the chip command address (the first byte of the window span, kept fixed), then write `0x50` and `0x70` and read the status byte, repeating up to 10,000 times until bit 7 is set (the same counter flaw as the erase: only a ready status on the final read is reported as -10, `0xfffffff6`, and exhaustion falls through); write `0x50`, write `0x40` to the target byte, write the data byte; poll the same way (the corresponding value is -11, `0xfffffff5`); if status bit 4 is set it sets -12 (`0xfffffff4`) and repeats the same byte with no bound; otherwise write `0xff`, read the byte back and compare. A mismatch sets -13 (`0xfffffff3`), counts down from 100 and gives up at zero; a match resets that counter to 10,000 and advances. The error value is never cleared by a later success, so a byte that needed a retry makes the whole call return its error even when the data ended up correct.
- `_readFlash(record, destination, count)` (`0x10d174`) works from the position at `+0x0c`: it copies the rest of the current 32 KiB block, then whole blocks, then the tail, reopening the window in read mode (`0xff`) for each block and copying through a library block-copy routine, and advances the position after each part. A null window gives -9 (`0xfffffff7`).
- `_writeFlash(record, source, count)` (`0x10d3a4`) computes the room left in the current block and opens the window in write mode (`0x40`; null gives -9, `0xfffffff7`). If the count fits in that room it calls `_writeFlashBytes` once with the whole count and advances the position by it. If it does not fit it still calls `_writeFlashBytes` with the **whole** count (not the room), then advances the position by only the room and continues with the remainder: the whole-block loop and the tail call also pass the remaining count rather than a block size, so a write that crosses a block boundary programs the same bytes more than once, and a span longer than the rest of the 128 KiB page would run on past the window end. The download path does not hit this for a sender that keeps one packet size: 128 or 1024 bytes at positions that are multiples of that size never cross a 32 KiB boundary (the host tool sends only 1024-byte packets); a sender mixing sizes could. The return value is the sign-extended byte result of `_writeFlashBytes`; the position advances only after a zero result.
- `_flashtest` (`0x10d594`): fills a buffer with `i mod 201` (the multiply by `0x28c1979` and shift is a signed divide by 201), erases, writes it in 128-byte pieces, reads it back and compares.
- `_closeFlash` (`0x10d154`) closes the window and clears the open flag at `+0x04`; `_openFlash` sets it to 1 and the position to 0.

## 2. Tunables the flash and download paths read

`_ffscDefaultTuneInfo` (6.5.30 `0x12601c`) holds 39 sixteen-byte entries `{name pointer, index, default, flag}`, ending with a zero name. `_ffscEarlyInit` (`0x12661c`) copies them to `_ffscTuneInfo` indexed by `index`, then fills `_ffscTune` (`0x1e0678`, a word array indexed by `index`, 39 words) from NVRAM object 8, replacing any zero word with the default (established). Relevant entries:

| Index | Offset in `_ffscTune` | Name here | Default |
|---|---|---|---|
| 25 | `+0x64` | download error limit | 10 |
| 26 | `+0x68` | download interrupt limit | 10 |
| 37 | `+0x94` | erase poll limit | 10,000 |
| 38 | `+0x98` | erase attempt limit | 1 |
| 1 | `+0x04` | buffer-write wait | 10,000,000 |
| 21 | `+0x54` | connect wait | 3,000,000 |
| 23 | `+0x5c` | listener connection limit | 25 (23 in 6.5.5) |

The table's fourth field is 0 for the entries with index 24, 25, 26, 28, 33, 34, 35 and 36 and 1 for the other 31; what the flag controls was not read. All 39 entries (index, default and flag, with its name string) were read from the 6.5.30 table at `0x12601c`.

The five defaults the upgrade steps copy besides these are router reply wait 20,000,000, MSC reply wait 5,000,000, local response wait 4,500,000, remote response wait 16,000,000 and switch debounce delay 10,000. The 6.5.5, 6.5.6 and 6.5.30 tables are identical except listener connection limit in 6.5.5. The unit of the time-like entries is inferred to be microseconds: buffer-write wait equals the 10,000,000 that the transfer routine passes as its first-byte timeout, and the timed-read helper (`_timeoutRead` `0x12dc78`, which calls `_timeoutSet` `0x12ddc8`) turns its timeout argument into seconds by dividing by 1,000,000 (established; the multiply by `0x431bde83` and shift by 18 is that divide).

## 3. The battery-backed store under NVRAM (module `sram`, `0x10c9a8`)

All established, 6.5.30.

- **Window.** 8 KiB at physical `0xa2000` to `0xa3fff`. `_sram_on` (`0x10cb48`) takes the interrupt lock, writes port `0x548 <- 0x00` and port `0x760 <- 0x81`; `_sram_off` (`0x10cb78`) writes port `0x540 <- 0x00` and releases the lock. Compare section 1: flash uses `0x548 <- 0xff`, `0x540 <- 0xff` and other values on `0x760`.
- **Header of the window.** Bytes 0 and 1 hold the halfword `0xaa55` (a missing magic makes `_sram_read` return -3 at once, and `_sram_write` writes it with bytes 2 and 3 cleared); bytes 4 to 7 hold a checksum word; user bytes start at offset 8.
- **Checksum.** The word is `0 - (sum16)` as a 32-bit value, where `sum16` is the low 16 bits of the sum of window bytes `4` to `0x1ff7` taken with the word itself zeroed (`_sram_write`, `0x10ca88`). The verifier (`_sram_read`, `0x10c9a8`) sums bytes `8` to `0x1ff7`, masks to 16 bits and requires that plus the stored word to be zero as 32-bit arithmetic. The last eight window bytes (`0x1ff8` to `0x1fff`) are outside the sum.
- **When it is verified.** Only on the first read after boot (a flag at `0x1b0708`, initial value 1, cleared after a good sum). A bad sum clears window bytes `0` to `0x1ff7` (so the magic goes too) and returns -2. Later reads do not check.
- **Return codes.** 1 success, -1 when `offset + length > 0x1ff8`, -2 bad sum, -3 no magic. In both `_sram_read` and `_sram_write` the copy happens before the -1 test, so an oversized request is copied (and, for a write, stored) before it is reported.

## 4. NVRAM module (`nvram`, `0x129430` to `0x12a1a3`)

### Header (established)

The first object, `0x40` bytes at SRAM user offset 0 (window `0xa2008`), is the header: `+0x00` magic word `0xff5cff5c`, `+0x04` layout version, `+0x08` object count, `+0x0c` total bytes of all objects, `+0x10` a validity bitmap (bit `id & 31` of word `id >> 5`, counted from `+0x10`; `_nvramIsValid` `0x129c40`, `_nvramValidate` `0x12a110`). Ids 0 to 12 only reach the first word. The rest of the 64 bytes is not used by any routine read. `_nvramWriteHeader` (`0x12a170`) writes the 64 bytes back; the current layout version written by `_nvramReset` (`0x129a70`) is 6, the count 13.

### Object table (established, `_nvramObjectInfo` at `0x1b0fc8`, byte-identical in 6.5.5, 6.5.6 and 6.5.30)

Thirteen `{id, size}` pairs. `_nvramInit` (`0x129570`) lays them out back to back from offset 0: the header is object 0, so the table starts with it. Total `0x3f4` bytes. Owners are the routines that call `_nvramRead` (`0x1299d0`) and `_nvramWrite` (`0x129b60`) with that id.

| Id | Offset | Size | Owner (call sites) | Role (inferred from owner names) |
|---|---|---|---|---|
| 0 | `0x000` | `0x40` | `_nvramInit`, `_nvramWriteHeader` | header |
| 1 | `0x040` | `0x20` | `_identInit`, `_identSetIPAddr` | network identity, IP address word first |
| 2 | `0x060` | `0x04` | `_identInit`, `_identSetRackID` | rack ID |
| 3 | `0x064` | `0x20` | `_cmdDEBUG`, `_consInit`, `_ffscDebugInit` | debug flag word |
| 4 | `0x084` | `0x60` | `_ttyInit`, `_comSetSpeed`, `_comSetFlag`, `_comSetFunction`, `_comSetRxBuf`, `_comSetTxBuf` | six serial-port records of 16 bytes |
| 5 | `0x0e4` | `0x40` | `_elscInit`, `_elscUpdateSaveInfo` | MSC client information |
| 6 | `0x124` | `0x30` | `_consReadConsInfo`, `_consUpdateConsInfo` | console record, first console |
| 7 | `0x154` | `0x30` | same routines | console record, second console |
| 8 | `0x184` | `0xc0` | `_cmdSETENV`, `_cmdUNSETENV`, `_ffscEarlyInit` | tunables: 48 words, indexed by tune index |
| 9 | `0x244` | `0x80` | `_logInitSGI`, `_logUpdateSaveInfo` | log settings |
| 10 | `0x2c4` | `0x70` | `_userInit`, `_userUpdateSaveInfo` | user record |
| 11 | `0x334` | `0x70` | same | user record |
| 12 | `0x3a4` | `0x50` | `_userInit`, `_userUpdatePasswords` | password block |

Absolute address of an object in the window is `0xa2008 + offset`. Object contents beyond what the upgrade steps touch (section 4, versions) are not decoded here.

### Read, write, reset, zap (established)

- `_nvramRead` (`0x1299d0`): an id above 12 or an empty slot logs a bad-id message and returns -1; a header without the magic logs that the store is uninitialised and returns -1; an object whose bitmap bit is clear returns -1 silently; otherwise it calls `_sram_read` and returns 0 on success, or logs the `_sram_read` code and returns -1. `_nvramWrite` (`0x129b60`) mirrors that, and after a good write sets the bitmap bit and rewrites the header when the bit was clear.
- `_nvramReset` (`0x129a70`): zero the header, set magic, version 6, count 13 and the computed total, mark object 0 valid, write the header. Every other object is therefore invalid, and later reads give -1 until it is written.
- `_nvramZap` (`0x129c20`): zero the header and write it, so the next `_nvramInit` finds no magic and calls `_nvramReset`. `_ffscEarlyInit` (`0x12661c`) calls it when the front-panel switch sample equals `0x14`, printing a clearing notice and then waiting for the ENTER key; the reset-NVRAM console command reaches `_nvramReset`.

### Start-up check (`_nvramInit`, `0x129570`, established)

After building the offset table (rejecting an id above 12, a duplicate, a zero size, or an object ending beyond `0x1ff8`), it reads the header. `-3` (no SRAM magic) runs `_nvramReset` (which writes the header and so installs the SRAM magic) but the routine still falls into the next test with the code `-3`, logs it and fails, so the first start on blank SRAM is reported as a failure and the caller's retry below is what completes it; any other result than 1 logs the code and fails; a missing header magic runs `_nvramReset`. Version 6 skips the upgrade chain. Otherwise the version selects, through a jump table at `0x1296e0` (six entries, the first five used), the entry of a straight-line chain `_nvramUpgrade1to2`, `2to3`, `3to4`, `4to5`, `5to6` (each later step runs after the earlier ones), then a header write; a version outside 1 to 5 logs the value and fails. A failed step zeroes the header magic and fails. Then, for version 6: an object count of 13 requires the stored total to equal the computed one (a mismatch logs a size-changed message and fails); a count above 13 fails; a count below 13 sums the sizes of the objects it had and requires the stored total to match (a mismatch fails), then sets count 13 and the new total and rewrites the header. Objects added since the stored count therefore start invalid. `_ffscEarlyInit` answers any failure by logging it, calling `_nvramReset` and running `_nvramInit` again; if that fails too it reports that the NVRAM services cannot start.

### The six layout versions (established for versions 2, 5 and 6; versions 1, 3 and 4 not held)

Every image that creates a store writes the same thirteen-entry object table. Read in `_nvramReset` and `_nvramObjectInfo` of all eleven MMSC images held:

| Images | Version written by `_nvramReset` | Upgrade steps present | Object table |
|---|---|---|---|
| 6.4, 6.4-origin | 2 | `1to2` | the table above: 13 objects, total `0x3f4` |
| 6.5, 6.5.2, patch SG0002378 | 5 | `1to2` to `4to5` | the same |
| 6.5.5, 6.5.6, 6.5.7, 6.5.9, 6.5.10, 6.5.30 | 6 | `1to2` to `5to6` | the same |

So the object ids, sizes and offsets did not change between layout versions 2 and 6; the version number moved only when a step had to rewrite values inside existing objects (the table of steps below). The 6.4 `_nvramInit` (`0x126988` in 6.4) takes version 2 as current, sends version 1 through `_nvramUpgrade1to2` and rejects any other value, with the same count and total tests as 6.5.30 (13 requires the total `0x3f4`, a count above 13 fails, a lower count must sum to the stored total).

Not held: an image that writes version 1, 3 or 4. The count and total those versions stored are therefore not read. Inferred from the count test: a store with fewer than 13 objects held a prefix of the thirteen (the test sums the first `count` table entries), and the upgrade steps are what must be applied to its contents. What would settle it: an MMSC image older than 6.4 or one built between 6.4 and 6.5; none is on the IRIX media scanned. The steps below work on the version-6 ids and sizes, so what the code establishes is what each step rewrites inside the existing objects.

| Version | Step (6.5.30) | What it writes | Fails when |
|---|---|---|---|
| 1 to 2 | `_nvramUpgrade1to2` `0x129c70` | objects 6 and 7: when the record's flag word (record `+0x14`) has bit 0 set, set record byte `+0x0c` to `0xa0` and write it back. Records that read as invalid are skipped | a write fails |
| 2 to 3 | `_nvramUpgrade2to3` `0x129d50` | object 8: copy the defaults of router reply wait, MSC reply wait, local response wait, remote response wait, connect wait, switch debounce delay into their words. Object 4: copy the halfword at record 3 `+4` from the default port table (`0xd000`) | object 8 or 4 cannot be read, or a write fails |
| 3 to 4 | `_nvramUpgrade3to4` `0x129ec0` | object 8: default of listener connection limit | object 8 cannot be read, or the write fails |
| 4 to 5 | `_nvramUpgrade4to5` `0x129f70` | object 8: default of buffer-write wait. Object 4: copy the halfword at `+4` of records 1 and 2 from the default port table (`0xe000` each) | as above |
| 5 to 6 | `_nvramUpgrade5to6` `0x12a070` | object 8: default of connect wait again | as above |

Each step stores the new version in the header word and calls `_nvramWriteHeader`. Default port table: `tty.o` data at `0x12dfc8`, six 16-byte records, each `{speed 9600 (0x2580), halfword flags, halfword index 0..5, two buffer-size words}`, flags `0xc000, 0xe000, 0xe000, 0xd000, 0xc000, 0xd000`. `_ttyInit` (`0x12e638`) builds object 4 from it when the object is invalid, zeroing the two buffer words, and later tops up any record whose flag halfword is negative. The meaning of the flag bits is not read.

The 6.5.5 and 6.5.6 builds carry the same table, the same jump table (shifted with the code), the same five steps and the same header values; the only differences in the module are the addresses of globals (section 7).

## 5. The download path

### Command side (established, 6.5.30)

- **`FLASH [FROM SYSTEM|FROM CONSOLE]`, `_cmdFLASH` `0x118d58`.** Requires the caller's authority value to exceed 999 ("ERROR AUTHORITY" otherwise), and the destination to name exactly one rack (`_destNumRacks`; else "Attempted to FLASH %d racks" and "ERROR DEST"). The origin defaults to SYSTEM; anything but `FROM` as the first word logs `Invalid FLASH subcommand`, and an origin other than SYSTEM or CONSOLE logs `Invalid origin for FLASH FROM command`. For the local rack it replies `OK;STATE BS%d FS%d FP%d M%d` (SYSTEM origin) or `OK;STATE BS%d FS%d M%d` (CONSOLE origin) with values taken from the console record, and sets a flag word at `0x1f6960`. A command aimed at a remote rack is relayed to that MMSC (`_remStartConnectCommand`), refused if it would flash a remote rack from a remote one ("ERROR TOOREMOTE"), or answered `ERROR CONNECT,%s`. The state fields are not decoded.
- **Entering flash mode.** `_ceEnterFlashMode` (`0x11b0e0`) clears both console buffers, sets the console state word (`+0x18`) to 6 and, when the console had a pending-ready flag, prints `R<rack>:READY` to the console. `_clLeaveFlashMode` (`0x11bd50`) clears the state words and returns.
- **The transfer runs in the console task** (inside the large console routine at `0x11d9a0`..., symbolised in this image as part of `_getSystemConsoleIndex`). When the console state is 6 it raises the task priority to 60, calls the enable routine (`0x10cba8`), initialises a flash record, calls `_openFlash(record, 2, progressFd)` (erasing the whole flash first), and on success calls `_getflash(record, the console descriptor, progressFd)` and `_closeFlash`. Only a zero result from `_getflash` is followed by the disable routine (`0x10cbe8`); a failed erase or failed transfer leaves the downloader flag set. Priority is restored, `_clLeaveFlashMode` runs, and the previous console mode is re-entered.
- **`DOWNLOADER ENABLE|DISABLE`, `_cmdDOWNLOADER` `0x118a78`.** Same authority rule; `ENABLE` calls the enable routine (`0x10cba8`), `DISABLE` the disable routine (`0x10cbe8`); both then reply `R<rack>:OK`. Missing or other words log `Must specify ENABLE or DISABLE with DOWNLOADER command` or `Invalid subcommand to DOWNLOADER` and reply `ERROR ARG`.
- **The downloader flag (established mechanism, inferred purpose).** The enable routine writes `0x7e` to port `0x70`, reads port `0x71`, writes `0x7e` to `0x70` again and writes the value read OR `0x80` to port `0x71`; the disable routine does the same with AND `0x7f`. That is bit 7 of byte `0x7e` of a PC real-time-clock/CMOS-style register file, which survives a reset. The boot code that would test it is not in any image read (controllers.md lists the boot ROM as absent), so its use as "the boot ROM should accept a download" is inferred from the name and from the fact that a failed transfer leaves it set.

### Wire protocol (established, `_getflash` `0x124ac8`, 6.5.30)

The receiver is XMODEM with CRC-16, accepting both 128-byte and 1024-byte packets:

1. It first calls `ioctl(the console descriptor, 0x1a, 0)` (meaning not read), logs and prints "Transferring firmware image", then writes the single byte `C` (`0x43`) and waits for the first packet. The first-packet wait is a timed read with a timeout argument of `0x989680` (10 s, the timed-read helper divides by 1,000,000); later packets use the same value, and the follow-up reads of an incomplete packet use `0xf4240` (1 s). The helper polls the descriptor's input count with `ioctl(fd, 1)` and `taskDelay(1)` until data arrive or the timer fires. An expired timer makes the timed read log "Timed out on read" (the alarm handler `_TimerPop` `0x12df68` sets the expiry flag) and return 0, which the receiver treats as end of file: "EOF reading XMODEM packet", failure. Only a read that returns -1 with error 4 (interrupted) counts against download interrupt limit, and when that count reaches the limit the receiver logs "Way too many timeouts!" and fails; each such interrupted wait is followed by another `C`. So a sender that does not start within 10 s after the erase ends the download with the flash already erased, and the downloader flag still set.
2. **Packet.** Byte 0 is `0x01` (SOH, 128 data bytes, 133 bytes in all) or `0x02` (STX, 1024 data bytes, 1029 in all); byte 1 is the packet number; byte 2 its complement (the sum of bytes 1 and 2 must be `0xff`); then the data; then the CRC, high byte first. The first packet number is 1; each next is `(previous + 1) & 0xff`.
3. **Single-byte controls.** `0x04` (EOT): reply `0x06` (ACK), log "Image download complete", write a four-byte terminator to the progress descriptor and return 0. `0x18` (CAN): log "Got CAN - aborting" and return -1. A short read of anything else is completed by further reads, each with a 1 s limit; an expired limit there returns 0 and fails with "EOF from partial packet timeoutRead", an interrupted one counts against download interrupt limit ("Timeout reading partial packet" at the limit).
4. **Checks, in order:** header byte 1 or 2 ("Bad packet header %d"); number checksum ("Bad packet number checksum: %x/%x"); expected number ("Bad packet number"); CRC ("Bad packet CRC: is %x/%x  should be %x/%x"). Each failure increments the error count and, below download error limit, sends NAK (`0x15`) and reads again; at the limit it logs "Too many errors!" and fails. A repeated packet is not recognised as a duplicate; it fails the number check and is answered with a NAK.
5. **Good packet.** Send ACK first, then `_writeFlash(record, data, 128 or 1024)`, whose return value is not examined (so a flash program error does not stop the transfer; the erase and `_writeFlash` error codes of section 1 are only visible to a caller that checks them), then write a one-byte progress mark to the progress descriptor and read the next packet. The last packet is written whole, so a sender that pads short packets puts its pad bytes into flash.
6. On return it logs "Number of Flash Packet Errors: %d" and "Number of Flash Packet Timeouts: %d".

### Checksum (established)

CRC-16 with polynomial `0x1021`, initial value 0, most significant bit first, no final inversion (CRC-16/XMODEM), over the data bytes only, driven by a 256-entry halfword table: `crc = (crc << 8) XOR table[(crc >> 8) XOR byte]`, kept to 16 bits. The table is `_CRCTab` (6.5.30 `0x1b0d78`, little-endian halfwords; 6.5.6 `0x1b0d68`; 6.5.5 `0x1b0d58`), 512 bytes. All 256 entries of each build, and all 256 entries of the host tool's copy (6.5.30 `0x10009928`, big-endian halfwords), equal the table computed from that polynomial. The packet number complement is the only other check; there is no checksum over the whole image.

### Other flash users (established)

The source file of `_getflash` also holds two debug helpers that are never called from the paths read: `_writebuf` (`0x1247b8`: open with mode 2, write a buffer in 32 KiB pieces logging "Wrote %d bytes at offset %ld") and `_diffbuf` (`0x124658`: open with mode 1, read back in 32 KiB pieces and compare, logging "Buffers do not compare at offset %ld!!", suspending the task on a difference).

## 6. The host half: `flashmmsc` 6.5.30

Bounds come from `.debug_frame`; all addresses are `flashmmsc` 6.5.30. The tool gets its entry points and names, in dynamic-string-table order: `get_firmware_version` `0x1000207c`, `extract_float_from_string` `0x1000229c`, `add_sc_info` `0x10002328`, `set_io_speed` `0x1000256c`, `alloc_info_structs` `0x1000284c`, `auto_transfer` `0x100028ec`, `main` `0x10003f24`, `AbortTransfer` `0x10004d74`, `Log` `0x10004e7c`, `OOBCheckSum` `0x10004efc`, `ReadLine` `0x10004f50`, `ReadWithTimeout` `0x10005028`, `mmsc_open` `0x1000574c`, `mmsc_open_plus` `0x10005b04`, `mmsc_close` `0x10005e90`, `mmsc_close_plus` `0x10005ff8`, `mmsc_write` `0x100061ec`, a select-timed read helper `0x1000633c`, `mmsc_read` `0x10006420`, `mmsc_command` `0x100065a4`, `mmsc_response` `0x100066ac`, `do_grab` `0x100068c4`, `do_release` `0x10006934`, `mmsc_transact` `0x100069a4`, `mmsc_errmsg` `0x10006b6c`.

### Modes (established, `main`)

Options by string match: `-a` automatic, `-m` manual, `-d` direct to a named serial device (`-l`), `-p` probe only, `-V` print the version of the image file, `-f` image file (default `/usr/cpu/firmware/mmscfw.bin`), `-r` rack, `-L` log file, `-v` verbose. Mode numbers inside `main`: `-a` 1, `-m` 2, `-d` 3, `-p` 4, `-V` 5. `-a` and `-p` call `auto_transfer` (the second argument 0 or 1 selects probe-only), `-m` prints instructions telling the operator to type the MMSC escape character followed by `rack <rackid> flash` and then runs the sender on descriptor 0 (the operator's own terminal, put in raw mode); `-d` opens the device named with `-l`, prints "Waiting for MMSC to initiate transfer..." and runs the sender on it. A manual or direct run does not issue any command to the MMSC itself; the firmware is put in flash mode by the operator's `FLASH` command.

### Version report (established, `get_firmware_version` `0x1000207c`)

`-V` opens the image and reads it one byte at a time until a byte equals the first letter of the 13-byte marker `MMSC VERSION ` (the marker string at `0x10008088`), reads the next 79 bytes, compares the first 13 bytes with the marker, and on a match prints the text after the marker; on a mismatch it seeks back by the number of extra bytes read minus one and carries on. A file with no marker prints a notice that the image has no version information and returns -1; the exit status of the mode is that return. Auto mode does not use it: it compares versions reported by the live MMSC units.

### The command channel (established)

`mmsc_open` (`0x1000574c`) opens a tty (`/dev/ttyd1` in auto mode, speed `0x2580` = 9600) and the control device `/hw/machdep/mmsc_control`; `mmsc_transact` (`0x100069a4`) does `do_grab`, `mmsc_command`, `do_release`, then `mmsc_response`. `do_grab` is `ioctl(controlFd, 0x7a06)` and `do_release` `ioctl(controlFd, 0x7a07)`, `do_grab` returning -4 and `do_release` -5 on failure, and each doing nothing when the record's first word is zero.

- **Command frame** (`mmsc_command`, `0x100065a4`): `0xa0`, operation byte, length high byte, length low byte, payload, one checksum byte; the checksum is the low byte of the sum of the operation byte, both length bytes and every payload byte (the leading `0xa0` is left out). Written through `mmsc_write` (`0x100061ec`) to the tty descriptor when the record's first word is zero, otherwise to the control descriptor, looping over partial writes.
- **Response frame** (`mmsc_response`, `0x100066ac`): skip bytes until `0xa0`, read three bytes (status, length high, length low), refuse a length above the caller's buffer ("Response too long"), read the payload, then one checksum byte compared with the running sum of the three header bytes and the payload (bad: -3). `OOBCheckSum` (`0x10004efc`) is the same sum over a buffered frame. Reads go through the select-timed helper, which logs "read_expire: timeout" and sets error 62 when its deadline passes; `mmsc_transact` bounds the response wait at the current time plus 5.0 seconds (inferred from the `.lit8` constant and the add before the call).
- **Status byte.** 0 success; otherwise the tool logs "Error message from MMSC: %s" with 1 "Illegal command format", 2 "Unknown command opcode", 3 "Illegal argument(s)", 4 "Command checksum error", 5 "Internal error", any other "Unknown error", and returns -6.
- **Operations used by `auto_transfer` (established):** operation 0 with no data is a ping; operation `0x3d` (61) carries a NUL-terminated text command, the way the console commands reach the MMSC: `r . rackid`, `r * b * ver`, `r * b * msc ver`, `r <n> com 4 speed <baud> hwflow N`, `r <n> flash`, `r <n> reset_mmsc`. The firmware side of `FLASH` is the console command of section 5.

### Auto flow (`auto_transfer` `0x100028ec`, established in outline)

Stop the host daemon with `/etc/init.d/sn0start stop`; probe (ping, local rack ID, MMSC versions with `r * b * ver`, MSC versions with `r * b * msc ver`; `add_sc_info` stores a baud word at record `+0`, a double version at `+16`, 56 bytes per record); choose a speed per MMSC from its version; for each, send `r <n> flash`, open the serial line without echo, wait for `C`, send the image, then restore 9600 with `r <n> com 4 speed 9600 hwflow N`, send `r <n> reset_mmsc`, sleep 40 s and ping again; finally turn hardware flow control off on the MSC ports (`rack <n> com <m> hwflow n`).

Per-step detail read from `auto_transfer` and `set_io_speed` (`0x1000256c`), 6.5.30 (established):

- The first thing it prints is a warning that MMSC firmware older than 1.1 cannot be driven by `-a`; it then runs `/etc/init.d/sn0start stop` through `system` and prints a notice if that fails, but carries on.
- Every command to the MMSC goes through `mmsc_transact` with operation 61 (`0x3d`): the buffer is the command text plus its NUL, length is the text length plus one. The tty is `/dev/ttyd1` at 9600 (`0x2580`) each time it is opened.
- An MMSC whose version is zero or negative is skipped with "does not have a version I understand"; the others get "starting upgrade", `r <n> flash`, then the serial line is reopened and the sender runs. The version values are doubles in the 56-byte records; the constants it compares against (the `.lit8` pool at `0x10009d90`) are 0.0, 5.0, 1.1, 3.1, 1e-6 and 1e6.
- `set_io_speed(rack, baud, rate-argument...)` sends `r <n> com 4 speed <baud> hwflow N` inside a grab/release pair, then changes the host port's speed with the termios library calls (`tcgetattr`, the input and output speed setters, then `tcsetattr` with the action argument `0x5410`; the callee names come from the failure strings and the import order, not from symbol-resolved stubs), reopens the port at the new speed and pings. It prints `OK` on zero and `FAIL` otherwise.
- After the image the host sends `r <n> com 4 speed 9600 hwflow N`, restores its own port, sends `r <n> reset_mmsc`, calls `sleep(40)`, then pings once more and prints `OK` when the ping answers with status 0.
- The final flow-control step covers only MSC records with a version greater than 0 and at most 3.1 (the two compares against the pool values 0.0 and 3.1): for those it sends `rack <n> com <m> hwflow n`, with `m` = 2 when the record's byte at offset 24 is `U` (the "UPPER" label) and 3 otherwise (the "LOWER" label).

Speed rule, as the code is written (established): a version of at least 1.1 gets 115200 (`0x1c200`) with the message "supports 115K FLASH upload"; below 1.1 it stores 9600 but prints "only supports 57.6K FLASH upload"; the third message ("only supports 9600bps") is on a path reached only for an unordered double. The value 57600 is not stored anywhere in the program, so the 57.6K wording and the 9600 setting disagree. Whether older MMSC firmware can change speed at all is not read.

### The sender (established, `main` `0x10003f24` and the same sequence inlined in `auto_transfer`)

- Wait for `C` (`0x43`): read one byte at a time with a 500-second timeout, discarding anything else, abort with "Timed out waiting for transfer to begin" on a timeout.
- Build STX packets: byte 0 `0x02`, byte 1 packet number (starting at 1), byte 2 its complement, 1024 data bytes (a short last read is padded with `0x1a`), CRC high byte then low byte from `CRCTab` as in section 5; send all 1029 bytes in one write, log "Wrote packet %d  CRC 0x%02x%02x" when verbose, print `.` every eighth packet.
- Wait up to 10 s for the reply: ACK `0x06` advances, NAK `0x15` re-sends, up to 20 attempts then "Too many errors", CAN `0x18` is discarded once ("Discarding single CAN") and aborts the transfer on a second ("Transfer cancelled by receiver"), a timeout re-sends ("Timeout on ACK, attempt %d"), other bytes are logged and ignored.
- After the file ends send EOT `0x04` and wait 15 s for ACK, up to five times, then close the file; auto mode then waits and resets the MMSC as above, the other modes sleep 1 s, restore the terminal and print "New image transferred to MMSC".

The sender always sends 1024-byte packets; the firmware receiver accepts both sizes. The sender's retry limit (20 per packet) and the firmware's download error limit (10 total) are separate counters.

### The host half in other releases (established, read this session)

All eight releases that ship the tool carry a 512-byte CRC table equal to the polynomial-`0x1021` table (big-endian halfwords): 6.4 at file offset `0x3b50`, 6.5 `0x8768`, 6.5.5 to 6.5.10 `0x9938`, 6.5.30 `0x9928`. The 6.4-origin and 6.4 builds were distinguished by size only (17,604 and 17,620 bytes).

| Release | Size | What the tool does |
|---|---|---|
| 6.4-origin, 6.4 | 17,604, 17,620 | manual and direct modes only. Auto mode prints that automatic initiation "is not currently supported" and tells the operator to use `-m`; no version marker, no command channel. The sender is the same: STX packets of 1,029 bytes (`li 1029`), pad byte `0x1a` (26), ACK 6, NAK 21, CAN 24, `C` 67 |
| 6.5 | 40,524 | adds `-a`/`-p`, the command channel, `set_io_speed` and the three speed messages; no `MMSC VERSION ` marker string and so no `-V` |
| 6.5.5, 6.5.6, 6.5.7, 6.5.9, 6.5.10 | 44,688 each | `.text` byte-identical across the five (same hash over the 0x5368-byte section); they differ in the `.rodata` build header string and in the dynamic-symbol tables. Adds `-V` and the marker |
| 6.5.30 | 43,824 | recompiled (text 0x4ccc bytes against 0x5368); same strings and options, the error-message strings are stored in the opposite order, and the `-f` default path string moves |

The 6.5.5 to 6.5.30 differences are build artefacts: the protocol, the command texts, the 1,024-byte packet, the retry counts and the CRC are the same. The speed rule and the strings were compared by string tables, not by disassembling the older routines; the 6.5.30 routine bodies in this section are the ones read in full.

### The command channel, bit-level (6.5.30, re-read)

`mmsc_command` (`0x100065a4`) builds the frame on the stack as `a0`, operation, length high (the signed halfword divide of the length by 256), length low, payload, then one byte after the payload equal to the low byte of operation plus both length bytes plus the payload sum; it writes `length + 5` bytes. `mmsc_response` (`0x100066ac`) skips input until it reads `0xa0`, reads three bytes (status, length high, length low), compares the combined length with the caller's limit and stores -1 and "Response too long" when it is larger, reads the payload, then one checksum byte, returns -3 on a mismatch and 0 on success. `do_grab`/`do_release` issue ioctl requests `0x7a06` and `0x7a07` (31,238 and 31,239) on the control descriptor and return -4 and -5 on failure respectively. The response wait added in `mmsc_transact` is the current time plus the pool value 5.0 (`.lit8` at `0x10009d98`), so the 5-second figure is established, not inferred.

## 7. Cross-build comparison (established)

For 6.5.5, 6.5.6 and 6.5.30, every routine named in sections 1, 3 and 4 and in `_getflash`, `_cmdFLASH`, `_cmdDOWNLOADER`, `_ceEnterFlashMode`, `_clLeaveFlashMode`, `_writebuf` and `_diffbuf` was disassembled and compared after masking absolute addresses. They are instruction-for-instruction identical (re-checked this session for 28 routines against 6.5.5 and 6.5.6 with absolute addresses masked; `_nvramInit` is identical up to its jump table, whose six entries are the same offsets from the routine start, `0x190, 0x199, 0x1a2, 0x1ab, 0x1b4, 0x1bd`, in all three builds). The only differences are the addresses of globals (the flash module sits at `0x10cc14` in 6.5.5 and `0x10cc24` in the other two; `_nvramHdr` is `0x1e0e78`, `0x1e0e88` and `0x1e25d0`; `_routeCheckBaseIO` differs in its flag address). The data is identical too: `_socketArray`, `_flashArray`, the delays, `_nvramObjectInfo`, the CRC table and the default port table; the default tunable table differs only in listener connection limit (23 in 6.5.5). Earlier builds (6.4 to 6.5.2) carry no version string and were not read for these modules.

Addresses of the routines and data named in sections 1 to 5, by build (read from each image's symbol table):

| Name | 6.5.5 | 6.5.6 | 6.5.30 |
|---|---|---|---|
| `_initFlash` | `0x10cc14` | `0x10cc24` | `0x10cc24` |
| `_openFlash` | `0x10cff4` | `0x10d004` | `0x10d004` |
| `_writeFlash` | `0x10d394` | `0x10d3a4` | `0x10d3a4` |
| `_sram_read` | `0x10c998` | `0x10c9a8` | `0x10c9a8` |
| `_cmdDOWNLOADER` | `0x118a68` | `0x118a78` | `0x118a78` |
| `_cmdFLASH` | `0x118d48` | `0x118d58` | `0x118d58` |
| `_ceEnterFlashMode` | `0x11b0d0` | `0x11b0e0` | `0x11b0e0` |
| `_getflash` | `0x124aa8` | `0x124ab8` | `0x124ac8` |
| `_nvramInit` | `0x129550` | `0x129560` | `0x129570` |
| `_nvramUpgrade1to2` | `0x129c50` | `0x129c60` | `0x129c70` |
| `_nvramUpgrade5to6` | `0x12a050` | `0x12a060` | `0x12a070` |
| `_socketArray` | `0x1b06fc` | `0x1b070c` | `0x1b071c` |
| `_flashArray` | `0x1b073c` | `0x1b074c` | `0x1b075c` |
| `_nvramObjectInfo` | `0x1b0fa8` | `0x1b0fb8` | `0x1b0fc8` |
| `_CRCTab` | `0x1b0d58` | `0x1b0d68` | `0x1b0d78` |

The page table, write-bit table, delay record, object table and CRC table are byte-for-byte equal in all three; the 39-entry tunable table differs only in the listener connection limit default (23 in 6.5.5, 25 elsewhere).

## 8. Not settled

See [../open-questions.md](../open-questions.md): the controller behind ports `0x540`, `0x548`, `0x760`; the purpose of CMOS byte `0x7e` bit 7 on the boot side; the console `STATE` fields; the meaning of port-record flag bits.
