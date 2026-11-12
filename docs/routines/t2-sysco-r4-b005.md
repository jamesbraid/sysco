# Sysco tier-2 reading batch 5 (round 4)

Batch 5 of the tier-2 reading round covers the 25 routines at zero-based ranks 125 to 149 of the located-grade target order (`reading_round.py --from-grade located`). All 25 rendered and passed the renderer grader. One is left out, `__istart` at 0x100313ac in `6.5.12/usr/sbin/flashsc`, a 4-byte symbol with no instructions to read. Every entry below is inferred, not established. The two L1 copies at 0xffe5d944 and 0xffecd944 and the two at 0xffe5e3dc and 0xffece3dc match instruction for instruction, shifted by 0x70000; their callee addresses shift the same way.

- `PPPWrite` in `6.5.12/usr/sbin/flashsc` at 0x10030120
  **Inferred.** Writes a buffer out over a PPP link one byte at a time. Each byte is tested against 0x7d and 0x7e (the PPP escape and flag bytes) and against the control codes 0x11, 0x13 and 0x0d, which fits PPP async character escaping. State lives in the structure at a0 (fields at offsets 0x10 through 0x28), and it calls through the function slots at [a0+16] (jalr at 0x1003038c) and [a0+28] (jalr at 0x10030604).

- `PPPRead` in `6.5.12/usr/sbin/flashsc` at 0x10030660
  **Inferred.** The read side of the same PPP link. It scans incoming bytes for 0x7d and 0x7e, handles the escape, and grows its receive buffer with realloc (call at 0x10030a98). It returns the stored length from [a0+12] (return at 0x10030ca4) and the negative codes -0x1f8 and -0x1fa on two of its exits. On what look like its failure paths it passes distinct codes from -0x1f4 to -0x1fa to an unnamed helper.

- `MemUsageBuf` in `6.5.12/usr/sbin/flashsc` at 0x10030cd0
  **Inferred.** Copies a 26-byte block of data from 0x10046c20 through 0x10046c38 (three 8-byte loads and one 2-byte load) into a static buffer at 0x10055cc0 through 0x10055cd8, then returns that buffer address 0x10055cc0. Two words at 0x10046f4c and 0x10046f50 are read but their use is not in these facts.

- `MemUsage` in `6.5.12/usr/sbin/flashsc` at 0x10030d30
  **Inferred.** Calls MemUsageBuf (0x10030cd0) with its own four arguments, passes the buffer it returns to puts (jalr at 0x10030d5c), and flushes the stream at sym(__iob)+16 with fflush (jalr at 0x10030d70). It returns the fflush result. This reads as a routine that prints a memory-usage report to stdout.

- `syslog` in `6.5.12/usr/sbin/flashsc` at 0x10030eb0
  **Inferred.** Formats a log message. A bounds check at 0x10030f84 (argument a1 below 10) guards a 10-entry jump table at 0x10046c60, which selects the format. The text is built with vsprintf and sprintf into buffers at 0x10055e50 and 0x10055f50 and written with fprintf to the stream at __iob+32 (calls at 0x10031040, 0x100311cc, 0x1003125c). One path ends in exit(-1) at 0x10031050. The name suggests the system logger, but there is no call to the system logger in these facts, so this is most likely a formatter that writes to stderr.

- `sub_ffe5d944` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5d944
  **Inferred.** A ColdFire L1 routine that takes two stack arguments (compared at 0xffe5d97c and 0xffe5d984). It sets and clears bits in the byte at 0x200007f8 and stores and clears the bytes at 0x200007f9 through 0x200007ff, and it reads a word at 0x30000570 through an index (add.l at 0xffe5db7c). It makes no calls. Its overall purpose is not settled by these facts.

- `sub_ffe5e3dc` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5e3dc
  **Inferred.** A ColdFire L1 routine that sets a bit in the byte at 0x200007f8 (bset.b at 0xffe5e3ee), reads 0x200007ff, clears the bit again (bclr.b at 0xffe5e454), and does a 32-bit multiply (muls.l at 0xffe5e482). It calls the address 0xffe5e52c twice (bsr.w at 0xffe5e48c and 0xffe5e4c4), which is the first address past this routine's range, so the code there is not in these facts. It also calls the fixed address 0xffe58508 twice (0xffe5e508 and 0xffe5e516) and returns 0 in d0. The bit operations and the multiply are marked unsupported by the analysis, so the flag's meaning is not established.

- `sub_ffecd944` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffecd944
  **Inferred.** The second copy of the L1 routine at 0xffe5d944, with every address shifted by 0x70000. It takes two stack arguments (compared at 0xffecd97c and 0xffecd984), sets and clears bits in the byte at 0x200007f8, stores and clears the bytes at 0x200007f9 through 0x200007ff, and reads a word at 0x30000570 through an index. It makes no calls. Its overall purpose is not settled by these facts.

- `sub_ffece3dc` in `6.5.13/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffece3dc
  **Inferred.** The second copy of the L1 routine at 0xffe5e3dc, with every address shifted by 0x70000. It sets and clears a bit in the byte at 0x200007f8 around a 32-bit multiply (muls.l at 0xffece482), and it calls 0xffece52c twice (bsr.w at 0xffece48c and 0xffece4c4), the first address past its range, and the fixed address 0xffec8508 twice (0xffece508 and 0xffece516). It returns 0 in d0. The bit operations and the multiply are unsupported in the analysis, so the flag's meaning is not established.

- `sub_c0139dc8` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc0139dc8
  **Inferred.** A PowerPC routine that saves its registers (stmw at 0xc0139dd0) and invalidates the TLB twice (tlbia at 0xc0139ee0 and 0xc0139f28). It then stores 18 words, halfwords and bytes into memory, writes the fixed address 0xfa200e40 and reads it back at 0xc0139e24. It calls 0xc0026eb0, 0xc000cb70, 0xc0083ca8, 0xc0004660, 0xc00102c8, 0xc002acf0 (with 0xc000b678 after it at 0xc0139ed0 and 0xc0139f18), 0xc0003304, 0xc000fd1c, 0xc0004c04, 0xc00a5538 and 0xc00a4bb0. The calls and stores look like start-up setup, but the callees are not named in these facts, so the purpose is open.

- `__start` in `6.5.13/usr/sbin/flashsc` at 0x100065c0
  **Inferred.** The program entry. A bal at 0x100065d8 gets the program counter, then it reads table slots at 0x42eec, 0x4344c, 0x43450, 0x43350, 0x42ef4, 0x43010 and 0x42ef0 and stores three values into the slots at 0x42eec, 0x4344c and 0x43450 (stores at 0x1000661c, 0x10006634 and 0x10006648). It makes four indirect calls (jalr at 0x10006654, 0x10006668, 0x100066a8 and 0x100066c0), the last with a0 from an earlier call. The targets are not in these facts. This is most likely the C start-up code that calls main, but that is inferred.

- `main` in `6.5.13/usr/sbin/flashsc` at 0x10006770
  **Inferred.** The command-line driver. It parses the command line with sn1SCParseCmdline (0x1001ab70) and compares option strings with strcmp (calls from 0x1000688c to 0x10006e44). It opens the image with FlashFileOpen (0x10006a64), classifies it with FlashImageType (0x10006af8), and checks it with FlashValidateImage (calls at 0x10006c68, 0x10006cf8 and 0x10006d84), exiting with -1 on its failure paths. It also reads the system configuration arrays SysCfgL1List, SysCfgL1Count, SysCfgL2List and SysCfgL2Count, and the flag IRByteOrder. The helpers at 0x100066f0 and 0x10007c90 are not named in these facts.

- `FlashFileOpen` in `6.5.13/usr/sbin/flashsc` at 0x10008630
  **Inferred.** Opens a flash image and loads it into memory. It calls open, fstat for the size, malloc for a buffer and read into that buffer. It then checks the header with FlashValidateFileHeader (call at 0x10008870). On failure it closes the descriptor (close calls at 0x10008740, 0x100087c8, 0x10008848 and 0x10008898) and prints the errno value through fprintf on the stream at __iob+32. It stores three words through the pointers in a2, a3 and a4 and returns the descriptor or -1.

- `FlashFileClose` in `6.5.13/usr/sbin/flashsc` at 0x10008900
  **Inferred.** Closes a flash image after writing to it. It seeks to offset 0 with lseek (call at 0x10008948), so it appears to rewrite the start of the file, writes a2 with a3 bytes using write (call at 0x100089b4), and reports lseek and write failures with errno through fprintf. It closes the descriptor at 0x10008a1c and returns -1 or 0.

- `FlashValidateFileHeader` in `6.5.13/usr/sbin/flashsc` at 0x10008a50
  **Inferred.** Checks a flash image header. It reads two big-endian words at header offsets 0x20 and 0x24 (loads at 0x10008a84, 0x10008aa8 and 0x10008acc) and converts them with ntohl. On a mismatch it prints a message through fprintf (call at 0x10008b00). It returns -1 on failure and otherwise 0x20 or 0x28, which looks like a header length chosen by a field in the file. These facts do not show what chooses between the two.

- `FlashImageType` in `6.5.13/usr/sbin/flashsc` at 0x10008b60
  **Inferred.** Identifies the kind of a flash image. It reads the word at the fixed address 0x1004949c in nine places and stores results through the pointers in a2, a3, a4 and a5 (stores at 0x10008d7c, 0x10008d94, 0x10008dac and 0x10008dc4). It returns -1 or 0, probably 0 for a recognised image. What the table at 0x1004949c holds is not in these facts.

- `FlashImageData` in `6.5.13/usr/sbin/flashsc` at 0x10008df0
  **Inferred.** Copies image data into a new heap buffer. It allocates with malloc (call at 0x10008e48), copies with memcpy (call at 0x10008e8c), reads big-endian words from the descriptor at a5 and the pointer in a6, and converts 16-bit fields with ntohs and htons (calls at 0x10009044 to 0x1000909c). It returns 0 or the new buffer pointer (return at 0x1000914c). What the converted fields mean is not in these facts.

- `FlashDumpImageHeader` in `6.5.13/usr/sbin/flashsc` at 0x10009160
  **Inferred.** Prints the fields of an image header that is at least 36 bytes long. It reads two 16-bit fields at offsets 0x0 and 0x2 and eight 32-bit fields from 0x4 through 0x20 (loads at 0x100091ac to 0x100093a0), converts each with ntohs or ntohl, and prints one line per field with fprintf to the stream in a0. It returns 0 at 0x100093e0. The printed labels are in strings these facts do not show.

- `FlashDumpImageID` in `6.5.13/usr/sbin/flashsc` at 0x100093f0
  **Inferred.** Prints an image identifier. It reads 32-bit words at offsets 0x4, 0x10 and 0x14 of the descriptor at a1 (loads at 0x10009458, 0x10009434 and 0x10009418), converts them with ntohl, computes sums and offsets from them, and prints the results with fprintf to the stream in a0 (call at 0x100094e8). The format string is not in these facts, so the field meanings are not settled.

- `FlashDumpRelocTable` in `6.5.13/usr/sbin/flashsc` at 0x10009510
  **Inferred.** Dumps a relocation table header. It reads four 32-bit words at offsets 0x0, 0x4, 0x8 and 0xc of the table at a1 (loads at 0x10009598, 0x1000953c, 0x100095e8 and 0x1000961c), converts them with ntohl, and prints each with fprintf (calls at 0x1000958c to 0x10009644). The remaining calls, from 0x1000966c to 0x10009784 with an ntohl at 0x10009760, appear to print the rest of the table. The printed strings are not in these facts.

- `FlashUpdateRemote` in `6.5.13/usr/sbin/flashsc` at 0x100097e0
  **Inferred.** Sends a flash image to a remote controller and reports progress. It checks the image with FlashImageType (call at 0x10009858) and FlashValidateImage (calls at 0x1000987c and 0x100098f4). It sends controller commands through sn1SCCmd (calls at 0x10009a78, 0x1000a0c4 and 0x1000a648, with codes 0x72, 0x74 and 0x73), fetches arguments through IRArg (0x100237f0), and frees packets with IRouterFreePacket (0x10022930) at several points. It logs through LoggerDefaultMessage and syslog at priorities 3 and 6, and it sleeps 15 seconds at 0x10009b70 and 0x1000a1b8, probably between attempts. It moves blocks through FlashImageData (call at 0x1000a584). The controller protocol is not in these facts.

- `FlashUpdateRemoteRaw` in `6.5.13/usr/sbin/flashsc` at 0x1000acf0
  **Inferred.** Sends a raw image to a remote controller with the same structure as FlashUpdateRemote at 0x100097e0, but its facts show no call to FlashImageType or FlashValidateImage. It sends controller commands through sn1SCCmd (calls at 0x1000ae48 and 0x1000b308, with codes 0x74 and 0x73), fetches arguments through IRArg, frees packets through IRouterFreePacket, and frees a buffer with free at 0x1000b474. It logs through syslog at priorities 3 and 6 and sleeps 15 seconds at 0x1000af3c and 0x1000b400. The reason for the raw mode is not stated in these facts.

- `FlashUpdateRemoteParallel` in `6.5.13/usr/sbin/flashsc` at 0x1000bcb0
  **Inferred.** The parallel form of the remote update. It checks the image with FlashImageType (0x1000bd48) and FlashValidateImage (0x1000bd6c and 0x1000be10). It prints controller connection addresses with SysCfgConnAddrStr (0x1000f520) at several points, and it drives a helper at 0x1000b680 four times with stage codes 0x72, 0x74, 0x73 and 0x72 (calls at 0x1000c20c, 0x1000ce2c, 0x1000d68c and 0x1000dbe4). Its 97 word loads and 31 word stores into its working area suggest per-target state, but the layout of that state is not in these facts.

- `FlashValidateImage` in `6.5.13/usr/sbin/flashsc` at 0x1000e220
  **Inferred.** Checks the fields of a flash image. It reads a big-endian word from the first word of the image (ntohl at 0x1000e26c) and a 16-bit field with ntohs (0x1000e304), and compares them against expected values, printing each mismatch through fprintf to the stream at __iob+32 (calls at 0x1000e2d0 to 0x1000e6d8). After the checks it dumps the image with FlashDumpImageHeader (0x1000e5d4), FlashDumpImageID (0x1000e608) and FlashDumpRelocTable (0x1000e718). It returns 0 or -1 and writes two result words through a4 and a5 (stores at 0x1000e734 and 0x1000e74c).
