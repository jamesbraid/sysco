# libdrivers: the flash access library

Read in full from L2 1.44.0, `/stand/sysco/lib/libdrivers.ppclinux.so` (14,036 bytes; the same bytes ship in 1.48.0). All 16 exported routines and the two data tables were read, with the listing made by `scripts/l2_list.py`. Addresses are virtual addresses in the library (text from `0x1668`). Grades as in [../../AGENTS.md](../../AGENTS.md). The per-routine rows are in [exports.tsv](exports.tsv). This note corrects the flash paragraph of [daemon.md](daemon.md) where it differs.

## The window

`flash_init` (`0x17a0`) maps the flash through `/dev/mem`:

- Open `/dev/mem` with flags `0x1002` (read-write, synchronous). Failure logs `can't open /dev/mem` and returns -2.
- Candidate physical bases are the two words in `.rodata` at `0x2a28`: `0xfc000000`, then `0xfe000000` (established from the bytes). For each candidate in order, the routine stores it as the window base `FI[0]` (the table `FI` at `0x12c34`), sets the window end `FI[1]` to the base plus `0x400000`, maps `FI[1]-FI[0]` = **4 MiB** with protection read-write, shared, at that physical base, and runs `flash_autoselect_test(0)` on the mapping. A zero result stops the search. A nonzero result moves on to the next candidate; the earlier mapping is not unmapped. A failed `mmap` closes the descriptor and returns -2.
- If neither candidate passes, the routine does not fail: it keeps the second candidate and mapping, and returns 0. (Established from the loop at `0x17fc` to `0x19b4`; the loop exits when the index passes 1 and the code after it does not look at the test results.) The initial contents of `FI` in the file are base `0xfe000000`, end `0xfe400000`.
- When the base that was kept is not `0xfe000000` the routine logs `flash memory detected at non-standard location` and sets the global `flash_addr_offset` (`0x12ec8`, initially 0) to base plus `0x02000000`. Every caller address is added to `flash_addr_offset` and then reduced by `FI[0]`, so a caller that uses the standard absolute address `0xfe000000 + x` reaches `x` in the window whichever base was found. So the standard base is `0xfe000000` and `0xfc000000` is the alternate; the earlier note's wording that only `0xfe000000` is accepted was wrong.
- Then it creates or opens a System V semaphore: `semget(0x0124f0e5, 1, 0x3b6)`; failure logs and returns -2. Success sets the "initialised" word (`0x12edc`) to 1 and returns 0.

The 4 MiB window is the evidence that the part is two 2 MiB devices side by side on a 32-bit bus (see below). Every other routine calls `flash_init(0)` first if the initialised word is zero and returns its error when it is negative.

## Locking

`flash_lock` (`0x1acc`) and `flash_unlock` (`0x1b98`) do nothing and return 0 when the semaphore id is negative. Otherwise each calls `semop` on the semaphore with a fixed operation array:

| Routine | Operations (semaphore 0) | Meaning |
|---|---|---|
| `flash_lock` | wait for zero (op 0, flags 0), then add 1 with flag `0x1000` | exclusive lock, released automatically if the process dies (SEM_UNDO) |
| `flash_unlock` | add -1 with flags `0x1800` | release (SEM_UNDO and IPC_NOWAIT) |

(The arrays are at `0x12c60` and `0x12ed0`; read as halfword triples.) A failed `semop` logs `could not get/release flash access semaphore` (both messages begin `flash_erase:`) and returns -1.

## Address, read and ID routines

- `flash_read_addr(addr)` (`0x1c64`): initialises if needed, then returns the host pointer `window_base_mapping + (addr + flash_addr_offset - FI[0])`. It does not check the range or lock.
- `flash_read(dst, addr, len)` (`0x1d14`): initialises if needed, takes `flash_lock`, `memcpy`s `len` bytes from `flash_read_addr(addr)` into `dst`, releases the lock, returns 0. No range check. (Argument order: dst, flash address, length; established from the `memcpy` arguments at `0x1d88` to `0x1d94`.)
- `sub_1dbc(table, count)` (`0x1dbc`): writes `count` pairs of words from `table` into the mapping; each pair is a byte offset from the mapping base and a 32-bit value. This is how every command sequence is issued.
- `flash_get_id(&mfr, &dev)` (`0x2068`): issues the reset sequence, then the identification sequence, reads the word at mapping offset 0 into `*mfr` and the word at offset 4 into `*dev`, then issues the reset sequence again. Always returns 0.
- `flash_autoselect_test(verbose)` (`0x210c`): calls `flash_get_id` and requires the manufacturer word to be `0x00010001` and the device word `0x22282228`. A mismatch returns -1, and when `verbose` is nonzero prints `invalid mfr. ID: actual=0x%x expected=0x%x` or `invalid device ID: ...`. Both words repeat the same 16-bit code twice: one per device, as expected from two 16-bit parts on a 32-bit bus. The manufacturer code 1 is the JEDEC code of the vendor whose parts use this command set; no image names the part number, so the device code is not mapped to a part (a hint only; see [../open-questions.md](../open-questions.md)).

## The command sequences

The tables the routines hand to `sub_1dbc` are in `.rodata`, offset then value, each value repeated in both 16-bit halves (read from the bytes):

| Table | Pairs | Used by |
|---|---|---|
| `0x2958` | `0x1554` ← `0x00aa00aa`; `0x0aa8` ← `0x00550055`; `0x0000` ← `0x00f000f0` | read/reset: unlock cycle then `0xf0` |
| `0x2a10` | `0x1554` ← `0x00aa00aa`; `0x0aa8` ← `0x00550055`; `0x1554` ← `0x00900090` | identification |
| `0x29f8` | `0x1554` ← `0x00aa00aa`; `0x0aa8` ← `0x00550055`; `0x1554` ← `0x00a000a0` | program one word |
| `0x29b8` | unlock cycle, `0x1554` ← `0x00800080`, unlock cycle again | first five cycles of a sector erase |

`flash_erase_ram` (`0x1f58`, called with the sector pointer) issues the `0x29b8` table, writes `0x00300030` at the sector address and polls. `flash_program_ram` (`0x1ff0`, called with a word pointer and a value) issues the `0x29f8` table, stores the value at that pointer and polls. The offsets `0x1554` and `0x0aa8` are four times the word addresses `0x555` and `0x2aa`, which is consistent with each 16-bit device's lowest address line sitting on bus address bit 2 (one device word per 32-bit bus word). That the command set is the AMD-style one used by many 2 MiB parts is a hint from the shape; the sequences are established.

## Polling

`flash_seq_done(mode, ptr)` (`0x1e50`) reads the word at `ptr` (the mapping base when `ptr` is 0) twice. If bit `0x00400040` of both reads agrees (no toggling in either device) it returns 0. Otherwise, if a previous pass saw bit `0x00200020` set in the first read (both devices flagging an error) it issues the reset table and returns -1. It records that flag after each unfinished pass. With `mode` 0 it then returns 1 immediately ("still busy"); with `mode` nonzero it loops. Callers: `flash_erase_ram` calls it with mode 0 and repeats while it returns 1; `flash_program_ram` calls it with mode 1. Established.

## Erase

`flash_erase(addr, len, errp)` (`0x229c`):

1. Initialises if needed, takes `flash_lock` (its result is not tested), and adds `flash_addr_offset` to `addr`.
2. Range check: `addr >= FI[0]` and `addr + len < FI[1]`; otherwise stores the adjusted address through `errp` (when non-null) and returns **-900** (`-0x384`) without releasing the lock.
3. Converts to an offset in the window and walks the sector table. The table `sector_map` (`0x28bc`, 39 words) lists per-device sector end addresses (31 of 64 KiB, then eight of 8 KiB); the code doubles every entry (`slwi 1` at `0x240c`, `0x2554`, `0x2584`) to get bus byte offsets, so the bus sectors are 31 of 128 KiB and eight of 16 KiB, 4 MiB in all, which matches the mapped window. The doubling is read in the code, not inferred.
4. A sector that overlaps the range is skipped if its words are all `0xffffffff` (a blank scan, `0x2480` to `0x24c8`); otherwise `flash_erase_ram` is called on it. A failure stores the sector's address through `errp`, releases the lock and returns **-903** (`-0x387`).
5. On completion releases the lock and returns 0.

`flash_erase_all` (`0x21e4`) calls `flash_erase` for the whole window (offset `FI[0] - flash_addr_offset`, length `FI[1] - FI[0]`, no error pointer) and returns its result.

## Program

`flash_program(addr, src, len, errp)` (`0x25b0`) writes `len` bytes a word at a time:

1. Initialises if needed; adds the offset; range check as for erase (-900, lock not yet taken).
2. Takes the lock and issues the reset table.
3. For each word, if the flash word already equals the source word it is skipped. Otherwise `flash_program_ram` is called; a negative result stores the failing address through `errp`, releases the lock and returns **-901**; after programming, a read-back that differs returns **-902**.
4. At the end releases the lock and returns 0.

The sizes of `len` are not rounded: a length that is not a multiple of 4 programs a whole last word from the source buffer.

## How callers use it

`libmain` stores the NVRAM at the flash address `0xfe3f0000` and `l2part` the partition database at `0xfe3e0000`; both are in the top 128 KiB, where the eight 16 KiB sectors are, so each 64 KiB region spans four small sectors. See [daemon.md](daemon.md) and [partition-task.md](partition-task.md) for the consumers.
