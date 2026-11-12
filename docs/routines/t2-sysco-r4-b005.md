# Sysco tier-2 reading batch 5 (round 4)

Batch 5 of the tier-2 reading round covers the 25 routines at zero-based ranks 125 to 149 of the located-grade target order (`reading_round.py --from-grade located`). All 25 rendered and passed the renderer grader. One is left out, `__istart` at 0x100313ac in `6.5.12/usr/sbin/flashsc`, a 4-byte symbol with no instructions to read. Every entry below is inferred, not established. The two L1 copies at 0xffe5d944 and 0xffecd944 and the two at 0xffe5e3dc and 0xffece3dc match instruction for instruction, shifted by 0x70000; their callee addresses shift the same way.

- `PPPWrite` in `6.5.12/usr/sbin/flashsc` at 0x10030120
  **Inferred.** Processes a buffer one byte at a time. In two loops each byte is compared with 0x7d, 0x7e, 0x11, 0x13 and 0x0d, which fits PPP async character escaping (0x7d and 0x7e are the PPP escape and flag bytes). State lives in the structure at a0: the facts show loads at offsets 0x10, 0x14, 0x18, 0x1c, 0x20 and 0x24 and stores at 0x18 and 0x1c. It calls an unnamed helper at 0x1002ff70 (call at 0x1003032c). At 0x1003038c and 0x10030604 it calls an unknown callee with [a0+16] as the second argument and [a0+28] as the third, then compares [a0+28] with the return value, which looks like a write-and-check-count call. The callee is not identified in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `PPPRead` in `6.5.12/usr/sbin/flashsc` at 0x10030660
  **Inferred.** Scans a buffer whose pointer is kept at [a0+8] and whose count is at [a0+12], comparing each byte with 0x7e, 0x7d and 0x0d. It calls realloc (call at 0x10030a98); when realloc returns 0 it stores to [a0+48], calls an unknown helper with -0x1fa and returns -0x1fa (return at 0x10030b00). It also returns -0x1f8 (return at 0x10030a04) on the path where a byte is 0x0d and [a0+36] is nonzero. Its normal exit returns the value at [a0+12] (return at 0x10030ca4). Seven calls to an unknown helper pass the codes -0x1f4 through -0x1fa, each only when [a0+52] is nonzero; the helper is not identified and the facts do not say these are failure paths.
  Reviewed: 2026-11-12 (sonnet)

- `MemUsageBuf` in `6.5.12/usr/sbin/flashsc` at 0x10030cd0
  **Inferred.** Does three 8-byte load and store pairs and one 2-byte load and store pair (26 bytes in all), using base addresses it first loads from words (lw at 0x10030cdc and 0x10030ce4). It returns the address 0x10055cc0, which the facts show only as the return value. The source and destination of the copy are not shown in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `MemUsage` in `6.5.12/usr/sbin/flashsc` at 0x10030d30
  **Inferred.** Role (from name): a memory-usage report printer. Calls MemUsageBuf (0x10030cd0) with its own four arguments, passes the buffer it returns to puts (jalr at 0x10030d5c), and flushes the stream at sym(__iob)+16 with fflush (jalr at 0x10030d70). It returns the fflush result. The facts do not show what the returned buffer contains.
  Reviewed: 2026-11-12 (sonnet)

- `syslog` in `6.5.12/usr/sbin/flashsc` at 0x10030eb0
  **Inferred.** Formats a log message. The first argument is masked with a word loaded from the data area, and an unsigned compare of the result against 10 at 0x10030f84 guards an indexed jump (jr at 0x10030f94) through a table at 0x10046c60, which selects one of the paths below; values of 10 or more go to 0x100311dc. The jump itself is not in the rendered facts and was read from the disassembly. The text is built with vsprintf (calls at 0x10030fb4, 0x10031078, 0x10031110 and 0x10031194) and sprintf into buffers at 0x10055e50 and 0x10055f50, which are cleared with memset first, and written with fprintf to the stream at __iob+32 (calls at 0x10031040, 0x100311cc, 0x10031208, 0x1003125c and 0x100312a8). One path ends in exit(-1) at 0x10031050. The name suggests the system logger, but there is no call to the system logger in these facts, so this is most likely a formatter that writes to stderr.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5d944` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d944
  **Inferred.** A ColdFire L1 routine that reads several stack arguments, not only two; the ones at 0x10(a6) and 0x14(a6) are compared at 0xffe5d97c and 0xffe5d984. It sets and clears bits in the byte at 0x200007f8 and stores and clears the bytes at 0x200007f9 through 0x200007ff, and it reads a word at 0x30000570 through an index (add.l at 0xffe5db7c). It makes no calls. Its overall purpose is not settled by these facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffe5e3dc` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e3dc
  **Inferred.** A ColdFire L1 routine that sets a bit in the byte at 0x200007f8 (bset.b at 0xffe5e3ee), reads 0x200007ff, clears the bit again (bclr.b at 0xffe5e454), and does a 32-bit multiply (muls.l at 0xffe5e482). It calls the address 0xffe5e52c twice (bsr.w at 0xffe5e48c and 0xffe5e4c4), which is the first address past this routine's range, so the code there is not in these facts. It also calls the fixed address 0xffe58508 twice (0xffe5e508 and 0xffe5e516) and returns 0 in d0. The bit operations and the multiply are marked unsupported by the analysis, so the flag's meaning is not established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffecd944` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecd944
  **Inferred.** The second copy of the L1 routine at 0xffe5d944, with every address shifted by 0x70000. It reads several stack arguments, not only two; the ones at 0x10(a6) and 0x14(a6) are compared at 0xffecd97c and 0xffecd984. It sets and clears bits in the byte at 0x200007f8, stores and clears the bytes at 0x200007f9 through 0x200007ff, and reads a word at 0x30000570 through an index. It makes no calls. Its overall purpose is not settled by these facts.
  Reviewed: 2026-11-12 (sonnet)

- `sub_ffece3dc` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffece3dc
  **Inferred.** The second copy of the L1 routine at 0xffe5e3dc, with every address shifted by 0x70000. It sets and clears a bit in the byte at 0x200007f8 around a 32-bit multiply (muls.l at 0xffece482), and it calls 0xffece52c twice (bsr.w at 0xffece48c and 0xffece4c4), the first address past its range, and the fixed address 0xffec8508 twice (0xffece508 and 0xffece516). It returns 0 in d0. The bit operations and the multiply are unsupported in the analysis, so the flag's meaning is not established.
  Reviewed: 2026-11-12 (sonnet)

- `sub_c0139dc8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc0139dc8
  **Inferred.** A PowerPC routine that saves its registers (stmw at 0xc0139dd0) and invalidates the TLB twice (tlbia at 0xc0139ee0 and 0xc0139f28). It also stores words, halfwords and bytes into memory, writes the fixed address 0xfa200e40 and reads it back at 0xc0139e24. It calls 0xc0026eb0, 0xc000cb70, 0xc0083ca8, 0xc0004660, 0xc00102c8, 0xc002acf0 (with 0xc000b678 after it at 0xc0139ed0 and 0xc0139f18), 0xc0003304, 0xc000fd1c, 0xc0004c04, 0xc00a5538 and 0xc00a4bb0. The calls and stores look like start-up setup, but the callees are not named in these facts, so the purpose is open.
  Reviewed: 2026-11-12 (sonnet)

- `__start` in `6.5.13/usr/sbin/flashsc` at 0x100065c0
  **Inferred.** The program entry. A bal at 0x100065d8 gets the program counter, then it reads table slots at 0x42eec, 0x4344c and 0x43450 and stores into them (stores at 0x1000661c, 0x10006634 and 0x10006648); the store at 0x1000661c is made only when the word read from the slot at 0x42eec is zero. It makes four indirect calls (jalr at 0x10006654, 0x10006668, 0x100066a8 and 0x100066c0), the last with a0 from an earlier call. The fourth call passes the words from the slots at 0x4344c, 0x43450 and 0x42eec as its first three arguments. The targets are not in these facts. This is most likely the C start-up code that calls main, but that is inferred.
  Reviewed: 2026-11-12 (sonnet)

- `main` in `6.5.13/usr/sbin/flashsc` at 0x10006770
  **Inferred.** The command-line driver. It parses the command line with sn1SCParseCmdline (0x1001ab70) and compares option strings with strcmp (calls from 0x1000688c to 0x10006f3c). It opens the image with FlashFileOpen (0x10006a64), classifies it with FlashImageType (0x10006af8), and checks it with FlashValidateImage (calls at 0x10006c68, 0x10006cf8 and 0x10006d84), exiting with -1 on its failure paths. It also reads the system configuration data SysCfgL1List, SysCfgL1Count, SysCfgL2List and SysCfgL2Count, and the flag IRByteOrder. The helpers at 0x100066f0 and 0x10007c90 are not named in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashFileOpen` in `6.5.13/usr/sbin/flashsc` at 0x10008630
  **Inferred.** Opens a flash image and loads it into memory. It calls open, fstat for the size, malloc for a buffer and read into that buffer. It then checks the header with FlashValidateFileHeader (call at 0x10008870). When fstat, malloc, read or the header check fails it closes the descriptor (close calls at 0x10008740, 0x100087c8, 0x10008848 and 0x10008898); a failed open, fstat, malloc or read also prints the errno value through fprintf on the stream at __iob+32, while no fprintf follows a failed header check in this routine. On success it stores three words through the pointers in a2, a3 and a4 and returns the descriptor; on failure it returns -1.
  Reviewed: 2026-11-12 (sonnet)

- `FlashFileClose` in `6.5.13/usr/sbin/flashsc` at 0x10008900
  **Inferred.** Closes a flash image, and writes to it first when a4 is nonzero. In that case it seeks to offset 0 with lseek (call at 0x10008948), so it appears to rewrite the start of the file, writes a2 with a3 bytes using write (call at 0x100089b4), and reports lseek and write failures with errno through fprintf. It closes the descriptor at 0x10008a1c and returns -1 or 0.
  Reviewed: 2026-11-12 (sonnet)

- `FlashValidateFileHeader` in `6.5.13/usr/sbin/flashsc` at 0x10008a50
  **Inferred.** Checks a flash image header. It reads two big-endian words at header offsets 0x20 and 0x24 (loads at 0x10008a84, 0x10008aa8 and 0x10008acc) and converts them with ntohl. It prints a message through fprintf (call at 0x10008b00) only when the word at 0x20 equals -1 and the word at 0x24 is 3 or more (signed). It returns -1 on failure and otherwise 0x20 or 0x28, which looks like a header length chosen by a field in the file. These facts do not show which path returns which of the two.
  Reviewed: 2026-11-12 (sonnet)

- `FlashImageType` in `6.5.13/usr/sbin/flashsc` at 0x10008b60
  **Inferred.** Identifies the kind of a flash image. It compares a0 with the constants 0xffffffffffe00000, 0xffffffffffe10000, 0xffffffffffe80000, 0xfffffffffe000000 and 0xfffffffffe040000. It makes nine word loads (at 0x10008c08, 0x10008c30, 0x10008c5c, 0x10008c88, 0x10008cac, 0x10008ce4, 0x10008cf8, 0x10008d20 and 0x10008d4c), and the facts do not show what they read. It stores results through the pointers in a2, a3, a4 and a5, each only when that pointer is nonzero (stores at 0x10008d7c, 0x10008d94, 0x10008dac and 0x10008dc4). It returns -1 or 0, probably 0 for a recognised image.
  Reviewed: 2026-11-12 (sonnet)

- `FlashImageData` in `6.5.13/usr/sbin/flashsc` at 0x10008df0
  **Inferred.** Copies image data into a new heap buffer. It allocates with malloc (call at 0x10008e48), copies with memcpy (call at 0x10008e8c), reads big-endian words from the descriptor at a5 and the pointer in a6, and converts 16-bit fields with ntohs and htons (calls at 0x10009044 to 0x1000909c). It returns 0 or the new buffer pointer (return at 0x1000914c). What the converted fields mean is not in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashDumpImageHeader` in `6.5.13/usr/sbin/flashsc` at 0x10009160
  **Inferred.** Prints the fields of an image header that is at least 36 bytes long. It reads two 16-bit fields at offsets 0x0 and 0x2 and eight 32-bit fields from 0x4 through 0x20 (loads at 0x100091ac to 0x100093a0), converts each with ntohs or ntohl, and prints one line per field with fprintf to the stream in a0. It returns 0 at 0x100093e0. The printed labels are in strings these facts do not show.
  Reviewed: 2026-11-12 (sonnet)

- `FlashDumpImageID` in `6.5.13/usr/sbin/flashsc` at 0x100093f0
  **Inferred.** Prints an image identifier. It reads 32-bit words at offsets 0x4, 0x10 and 0x14 of the descriptor at a1 (loads at 0x10009458, 0x10009434 and 0x10009418), converts them with ntohl, computes sums and offsets from them, and prints the results with fprintf to the stream in a0 (call at 0x100094e8). The format string is not in these facts, so the field meanings are not settled.
  Reviewed: 2026-11-12 (sonnet)

- `FlashDumpRelocTable` in `6.5.13/usr/sbin/flashsc` at 0x10009510
  **Inferred.** Role (from name): a relocation-table dump. It reads four 32-bit words at offsets 0x0, 0x4, 0x8 and 0xc of the table at a1 (loads at 0x10009598, 0x1000953c, 0x100095e8 and 0x1000961c), converts them with ntohl, and prints each with fprintf (calls at 0x1000958c to 0x10009644). The remaining calls, from 0x1000966c to 0x10009784 with an ntohl at 0x10009760, appear to print the rest of the table. The printed strings are not in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashUpdateRemote` in `6.5.13/usr/sbin/flashsc` at 0x100097e0
  **Inferred.** Sends a flash image to a remote controller and reports progress. It checks the image with FlashImageType (call at 0x10009858) and FlashValidateImage (calls at 0x1000987c and 0x100098f4). It sends controller commands through sn1SCCmd (calls at 0x10009a78, 0x1000a0c4, 0x1000a648 and 0x1000a9e0, with codes 0x72, 0x74, 0x73 and 0x72), fetches arguments through IRArg (0x100237f0), and frees packets with IRouterFreePacket (0x10022930) at several points. It logs through LoggerDefaultMessage and syslog at priorities 3 and 6, and it sleeps 15 seconds at 0x10009b70, 0x1000a1b8, 0x1000a740 and 0x1000aad8, probably between attempts. It moves blocks through FlashImageData (call at 0x1000a584). The controller protocol is not in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashUpdateRemoteRaw` in `6.5.13/usr/sbin/flashsc` at 0x1000acf0
  **Inferred.** Sends a raw image to a remote controller in a shorter form than FlashUpdateRemote at 0x100097e0: its facts show no call to FlashImageType or FlashValidateImage, and only two sn1SCCmd calls where FlashUpdateRemote has four. It sends controller commands through sn1SCCmd (calls at 0x1000ae48 and 0x1000b308, with codes 0x74 and 0x73), fetches arguments through IRArg, frees packets through IRouterFreePacket, and frees a buffer with free at 0x1000b474. It moves blocks through FlashImageData (call at 0x1000b244). It logs through syslog at priorities 3 and 6 and sleeps 15 seconds at 0x1000af3c and 0x1000b400. The reason for the raw mode is not stated in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashUpdateRemoteParallel` in `6.5.13/usr/sbin/flashsc` at 0x1000bcb0
  **Inferred.** Role (from name): the parallel form of the remote update. It checks the image with FlashImageType (0x1000bd48) and FlashValidateImage (0x1000bd6c and 0x1000be10). It prints controller connection addresses with SysCfgConnAddrStr (0x1000f520) at several points, and it drives a helper at 0x1000b680 four times with stage codes 0x72, 0x74, 0x73 and 0x72 (calls at 0x1000c20c, 0x1000ce2c, 0x1000d68c and 0x1000dbe4). Its 99 word loads and 31 word stores into its working area suggest per-target state, but the layout of that state is not in these facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashValidateImage` in `6.5.13/usr/sbin/flashsc` at 0x1000e220
  **Inferred.** Checks the fields of a flash image. It reads a big-endian word from the first word of the image (ntohl at 0x1000e26c) and a 16-bit field with ntohs (0x1000e304), and compares them against expected values. It prints each mismatch through fprintf to the stream at __iob+32 (calls at 0x1000e2d0 to 0x1000e6d8) only when a3 is nonzero. After the checks it calls FlashDumpImageHeader (0x1000e5d4) when a3 is 1, FlashDumpImageID (0x1000e608) when a3 is 2 and FlashDumpRelocTable (0x1000e718) when a3 is 1. It returns 0 or -1 and writes two result words through a4 and a5, each only when that pointer is nonzero (stores at 0x1000e734 and 0x1000e74c).
  Reviewed: 2026-11-12 (sonnet)
