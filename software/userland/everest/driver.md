# The /dev/sysctlr driver (sysctlr.o for IP19, IP21 and IP25)

Read from the IRIX 6.5 Foundation 1 objects listed in [README.md](README.md). Each object holds twelve routines in `.text`, one 4,176-byte state block (`sysc`, in `.bss`), a four-byte device-flag word (`syscdevflag`, zero in all three) and the string "sysctlr: Read overflow!" in `.rodata`. Addresses below are offsets in `.text` of the IP19 object; the IP21 and IP25 columns of the routine table give theirs. Disassembly was made with the toolchain objdump and read in full for every routine.

## Device registers

**Established** (IP19 `.lit8` at section offset 0 and 8, read by `syscinit` at `0x38`-`0x48` and by `syscintr` at `0x7e8`): the driver reaches the controller UART through two doubleword constants. IP19 and IP25 hold `0x9000000018000400` (UART block) and `0x9000000018000850` (interrupt acknowledge). IP21 holds `0x9000000418000400` and `0x9000000418000850`. `syscinit` stores the first constant in the state block at +16. The doubleword at UART+0 is a status word whose bit 0 means the transmitter can take a byte and bit 1 means a received byte is waiting (tests at `syscintr` `0x72c` and `0x734`/`0x790`). The doubleword at UART+8 is the data register: `syscintr` reads the received byte at `0x74c` and writes the next transmit byte at `0x7bc`, and `syscwrite` writes the first byte of a write at `0x4c4`. On leaving, `syscintr` stores the value 4 at the second constant (`0x7f8`).

**Inferred**: the high bits `0x9000...` select an uncached physical window, so the physical UART address is `0x18000400` on IP19 and IP25 and `0x418000400` on IP21 (the extra bit is the fifth hex digit). The meaning of the value 4 written to the acknowledge address is not read anywhere; it clears or masks the interrupt.

IP21 differs in how the registers are read. Both UART reads in the IP21 `syscintr` (the status doubleword at `0x6dc` and the received byte at `0x710`) are calls to an external helper `load_double`, where the IP19 and IP25 objects use a plain doubleword load. Writes to the UART are plain doubleword stores in all three.

**Inferred** (opcode multiset per routine, no-ops dropped, IP19 against IP25 and IP21): apart from that helper the three objects run the same logic. IP19 and IP25 differ in scheduling, in a duplicated `andi` in the delay slot of `syscopen`, in one inverted branch sense in `syscread`, in 64-bit against 32-bit add forms, and in `syscintr`, where IP19 uses the branch-likely forms `beql` (twice) and `blezl` and IP25 uses plain branches. This is not an instruction-by-instruction proof.

## State block

**Established** from the stores and loads of the twelve routines (IP19). Offsets are in bytes from the start of `sysc`.

| Offset | Size | Use | Grade |
|---:|---:|---|---|
| 0 | 1 | flags byte: bit 0 initialised, bit 1 opened for one direction, bit 2 opened for the other (see `syscopen`) | established |
| 8 | 4 | read lock word (bit 0 held, bit 1 a sleeper waits) | established |
| 12 | 4 | write lock word, same bits | established |
| 16 | 8 | UART block address (set by `syscinit`) | established |
| 24 | 4 | bytes waiting in the read ring (`SYSC_QLEN` returns this) | established |
| 28 | 4 | reader's wake threshold: the interrupt wakes the reader when the count reaches it | established |
| 32 | - | read sleep channel | established |
| 36 | 4 | read ring tail (next write by the interrupt) | established |
| 40 | 4 | read ring head (next read by `syscread`) | established |
| 44 | 4 | read timeout handle, zero when none is armed | established |
| 48 | 4 | read timeout in clock ticks, 100 after reset | established |
| 52 | 2,048 | read ring; indices are masked with `0x7ff` | established |
| 2100 | 4 | index of the next byte of the pending write | established |
| 2104 | 4 | bytes of the pending write still to send | established |
| 2108 | - | write sleep channel | established |
| 2120 | 4 | write timeout handle, zero when none is armed | established |
| 2124 | 4 | write timeout in ticks, 100 after reset | established |
| 2128 | 2,048 | write buffer | established |

## Routines (IP19 offsets; sizes in bytes)

| Routine | IP19 | IP21 | IP25 | Grade | What it does |
|---|---:|---:|---:|---|---|
| `sysc_clear` (local, 52) | `0x000` | `0x000` | `0x000` | established | Zeroes the ring counts, both ring indices, the read threshold and the write count, both timeout handles; sets both timeouts to 100 ticks. Called from `syscinit` and `syscclose`. |
| `syscinit` (52) | `0x034` | `0x034` | `0x034` | established | Calls `sysc_clear`, stores the UART block address at +16, sets flag bit 0. `syscintr` calls it when the address at +16 is still zero, so the first interrupt can initialise the driver. |
| `syscopen` (108 / 104 / 104) | `0x068` | `0x068` | `0x068` | established | Returns 6 if flag bit 0 is clear, returns 11 if either open bit (mask 6) is already set (one opener at a time), else sets bit 1 when the open mode has its value-2 bit and bit 2 when it has its value-1 bit, and returns 0. |
| `syscclose` (128 / 112 / 112) | `0x0d4` | `0x0d0` | `0x0d0` | established | At raised priority: cancels both timeouts, clears the two open bits, releases both locks (waking sleepers), calls `sysc_clear`, restores priority, returns 0. |
| `syscread` (656 / 628 / 632) | `0x154` | `0x140` | `0x140` | established | Takes the read lock (returns 4 if the wait is interrupted). Loops until the caller's count is met. Each pass copies, under raised priority, the smaller of the wanted and queued counts from the ring head (with a second copy when the run wraps). Then it sets the wake threshold to the smaller of the still-wanted count and 2,047 and, if fewer bytes are queued than that, arms the read timeout and sleeps. A signal returns 4. A timeout (handle cleared by `sysctout`) ends the read early with 0 and the bytes copied so far. Releases the lock. |
| `syscwrite` (460 / 436 / 436) | `0x3e4` | `0x3b4` | `0x3b8` | established | Takes the write lock (4 if interrupted). Copies at most 2,047 bytes of the caller's data into the write buffer, sets the pending count to one less and the index to 1, stores byte 0 straight into the UART data register, arms the write timeout and sleeps; `syscintr` sends the rest. A signal returns 4. A timeout adds the unsent count back to the caller's residual and returns 0 (short write). Loops for larger writes. |
| `syscioctl` (108 / 120 / 116) | `0x5b0` | `0x568` | `0x56c` | established | Command `0x7301` (`SYSC_QLEN`) stores the ring byte count through the return-value pointer (the sixth argument), so `ioctl()` returns the count to the caller. `0x7302` (`SYSC_RTOUT`) and `0x7303` (`SYSC_WTOUT`) set the read and write timeouts from the argument value, which is passed by value and must be positive. Any other command, or a non-positive timeout, returns 22. |
| `sysc_lock` (local, 152) | `0x61c` | `0x5e0` | `0x5e0` | established | Raises priority; if bit 0 of the lock word is set, sets bit 1 and sleeps on the word until it clears (returns 1 if the sleep is interrupted); sets bit 0; restores priority; returns 0. |
| `sysc_unlock` (local, 80 / 76 / 76) | `0x6b4` | `0x678` | `0x678` | established | Clears bit 0; if bit 1 was set, clears it, wakes the word and returns 1, else returns 0. |
| `syscintr` (340 / 328 / 336) | `0x704` | `0x6c4` | `0x6c4` | established | Interrupt handler. Initialises if needed. Reads the status doubleword. If a byte is waiting and the ring holds fewer than 2,048 bytes, appends it at the tail, advances tail and count, and wakes the reader when the count equals the threshold; if the ring is full it prints "sysctlr: Read overflow!" and wakes the reader, without reading the data register. If the transmitter is ready and bytes remain, writes the next byte, advances the index, decrements the count and wakes the writer when the count reaches zero. Always stores 4 to the acknowledge address and returns 4. |
| `sysctout` (local, 36 / 32 / 32) | `0x858` | `0x80c` | `0x814` | established | Timeout callback: clears the timeout handle at +20 of its argument and wakes the sleep channel at +8. The argument is the address of the count word for reads (+24) and of the write index (+2100) for writes. |
| `clear_tout` (local, 64) | `0x87c` | `0x82c` | `0x834` | established | Cancels the timeout whose handle is in the word at the argument, then zeroes the word. |

Sizes in the form `a / b / c` are IP19 / IP21 / IP25 where they differ.

Return values 4, 6, 11 and 22 are numbers read from the code. The objects do not name them.

## Which objects were not read function by function

The 6.4 builds of the three objects (`root/6.4-origin/usr/cpu/sysgen/IP{19,21,25}boot/sysctlr.o`) have the same constants and strings and a different register allocation in `syscopen` (checked by dumping `syscopen`); the other routines were not compared. Their rows are located by hash in [../inventory/images.tsv](../../../evidence/inventory/images.tsv) only.
