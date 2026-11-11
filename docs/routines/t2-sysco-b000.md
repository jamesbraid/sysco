# Tier-2 routine readings, sysco batch 0

Role statements for the first 25 targets of the tier-2 list, written from the rendered
instruction facts only (registers, calls, constants and branch targets). Each role is a
reading of those facts, not a confirmed contract. All 25 grades are inferred. The grader
accepted every entry, so none were rejected.

### 6.5.12/usr/sbin/flashsc 0x10006110

`__start` is a startup stub. It loads a few global cells (tables at 0x40e40, 0x4137c and 0x41380), calls a local helper at 0x10006130 from 0x10006128, and branches on a value loaded from the 0x40e40 table at 0x1000615c. It then makes indirect calls at 0x100061a4, 0x100061b8, 0x100061f8 and 0x10006210 whose targets the facts do not resolve. The shape looks like a C runtime start, but the callees are unknown, so that label is a guess from shape.

### 6.5.12/usr/sbin/flashsc 0x1000621c

`_mcount` is a 12-byte stub with one return at 0x10006220. It calls nothing and writes no memory, and the facts leave `v0` unknown. It is either a no-op hook or a placeholder for a profiling entry point. The name suggests the profiling reading, but that comes from the symbol name, not from the body.

### 6.5.12/usr/sbin/flashsc 0x100062b0

`main` parses the command line through `sn1SCParseCmdline` at 0x1000632c, then compares the arguments with `strcmp` at 0x100063a8, 0x100063dc and 0x10006414, and reads one option with `sscanf` at 0x10006a70. Its branches reach a file path (`FlashFileOpen` at 0x1000654c, `FlashImageType` at 0x100065e0, `FlashValidateImage` at 0x10006750) and a remote path (`FlashUpdateRemoteRaw` at 0x10006b74, `SysCfgConnInit` at 0x10006bf8, `FlashUpdateRemoteParallel` at 0x10007250, `FlashUpdateRemote` at 0x100073e4). Reads of `SysCfgL1Count`, `SysCfgL2List` and `SysCfgL1List` suggest it walks configured units before the remote update. The option-to-branch mapping is inferred, since the option strings are not in the facts.

### 6.5.12/usr/sbin/flashsc 0x10008000

`FlashFileOpen` opens the path in `a0` with `open` at 0x1000805c, using flags 0x0 or 0x2 chosen by `a5`, then calls `fstat` at 0x100080c8. On success it mallocs a buffer, reads the file into it with `read` at 0x100081d8, and calls `FlashValidateFileHeader` at 0x10008260. Failure paths print `errno` with `fprintf` and return -1. The stores at 0x100082b0, 0x100082bc and 0x100082c8 write three values through `a2`, `a3` and `a4`, but the facts do not say which value goes where. The close on the validation-failure path at 0x10008288 and the success return of the descriptor at 0x100082dc do not fit together cleanly from the facts, so the exact ownership of the descriptor is unclear.

### 6.5.12/usr/sbin/flashsc 0x100082f0

`FlashFileClose` seeks the descriptor in `a0` back to offset 0 with `lseek` at 0x10008338. When the flag in `a4` is non-zero (branch at 0x10008320), it writes `a3` bytes from `a2` with `write` at 0x100083ac and checks the count at 0x100083c8. It always closes the descriptor at 0x1000841c and returns -1 on the seek and write error paths, or 0 at 0x10008434. Reading `a4` as a write-back flag is inferred from the call pattern.

### 6.5.12/usr/sbin/flashsc 0x10008440

`FlashValidateFileHeader` reads big-endian words at offsets +0x20 and +0x24 of `a0` through `ntohl` at 0x1000847c, 0x100084a0 and 0x100084c4. It compares the word at +0x20 with -1 (branch at 0x1000848c), and a count below 3 (branch at 0x100084b0) goes to 0x10008510. The facts show a -1 return at 0x10008508 but do not tie it to that branch. On success it returns either 0x20 or 0x28, which may be a header-size choice, but the facts do not say what those values mean.

### 6.5.12/usr/sbin/flashsc 0x10008550

`FlashImageType` compares `a0` against five constant bases (0xffffffffffe00000, 0xffffffffffe10000, 0xffffffffffe80000, 0xfffffffffe000000 and 0xfffffffffe040000), each leading to its own block. Inside those blocks it compares `a1` with 0x10000, 0x80000 and 0x40000 and reads one shared word at abs 0x10046f4c nine times. It writes up to four results through `a2` to `a5` (stores at 0x10008770 to 0x100087b8) and returns -1 at 0x10008754 or 0 at 0x100087c8. The mapping from bases to image kinds is not stated, so a boot-image classifier is a guess.

### 6.5.12/usr/sbin/flashsc 0x100087d0

`FlashValidateImage` reads a header from `a0` with `ntohl` and `ntohs` and checks that the regions it describes fit inside the length in `a1` (unsigned compares at 0x10008838, 0x10008854, 0x100088e8 and 0x10008958). Each failed check prints through `fprintf` and returns -1 at its failure site. Calls to `FlashDumpImageHeader` (0x10008bac), `FlashDumpImageID` (0x10008be0) and `FlashDumpRelocTable` (0x10008cf8) sit on later paths, and the facts do not show which argument selects them. The success return is 0 at 0x10008d40.

### 6.5.12/usr/sbin/flashsc 0x10008d50

`FlashImageData` mallocs a buffer at 0x10008db4 and copies data into it with `memcpy` at 0x10008e04. It walks records reached through the pointer in `a5` (reads at +0x4 and +0xc) and byte-swaps their fields with `ntohl` and `ntohs`, writing some back with `htons` at 0x10008fdc and 0x1000901c. It returns the allocated pointer on success (0x10008e28, 0x100090c0) and 0 on two early exits. Whether the walk parses or builds the buffer is unclear from the facts.

### 6.5.12/usr/sbin/flashsc 0x100090d0

`FlashDumpImageHeader` prints a fixed header structure to the stream in `a0`, one labelled `fprintf` line per field. It reads a 16-bit value at +0x0 and +0x2 and 32-bit values at +0x4 through +0x20, byte-swapping each with `ntohs` or `ntohl` first. Two printed lines show sums of fields (0x10009258 and 0x1000929c), and it returns 0 at 0x10009350. The labels and field meanings are not in the facts.

### 6.5.12/usr/sbin/flashsc 0x10009360

`FlashDumpImageID` reads 32-bit words at +0x4, +0x10 and +0x14 of `a1` through `ntohl`, derives a few offsets from them, and prints one formatted line at 0x10009458. A branch at 0x100093e4 tests one value against 0xffffffffffe00000 (unsigned). The format string is one of two constants at 0x100475f0 and 0x100475f8, chosen by a branch, but the facts do not show what the two variants mean. It returns 0 at 0x10009470.

### 6.5.12/usr/sbin/flashsc 0x10009480

`FlashDumpRelocTable` reads a count and header words from `a1` with `ntohl` (calls at 0x100094b4, 0x10009510, 0x10009560 and 0x10009594). It loops over the table and dispatches on an entry type compared with 1, 2 and 3 (branches at 0x10009640, 0x10009650 and 0x10009660), printing each entry with `fprintf`. It returns 0 at 0x10009740. The entry layout and the meaning of the three types are not in the facts; only the type comparison itself is grounded.

### 6.5.12/usr/sbin/flashsc 0x10009750

`FlashUpdateRemote` is the largest update routine. It classifies and validates the image with `FlashImageType` (0x100097c4) and `FlashValidateImage` (0x100097e8 and 0x10009868), then runs remote commands through `sn1SCCmd` at 0x1000998c, 0x10009e48 and 0x1000a3e8 with codes 0x72, 0x74 and 0x73. On failure it logs with `syslog` at priorities 3 and 6 and calls `sleep` with 0xf. The priority meanings are conventional, not established by the facts, and the exact update sequence is only partly clear from the call order.

### 6.5.12/usr/sbin/flashsc 0x1000aab0

`FlashUpdateRemoteRaw` has the same command and log shape as `FlashUpdateRemote`: it sends `sn1SCCmd` with code 0x74 (0x1000ac0c) and 0x73 (0x1000b0ec), logs through `syslog`, and sleeps with 0xf on failure. It does not call `FlashValidateImage`, but it does call `FlashImageData` at 0x1000b020 with zero flags. That suggests a variant that sends image data without a separate validation pass, but the facts show call shape only, not the protocol.

### 6.5.12/usr/sbin/flashsc 0x1000ba10

`FlashUpdateRemoteParallel` validates the image as `FlashUpdateRemote` does (`FlashImageType` at 0x1000ba88, `FlashValidateImage` at 0x1000baac and 0x1000bb58). It then calls an unnamed routine through the pointer at 0x1000b470 with codes 0x72, 0x74 and 0x73, and prints per-connection addresses through `SysCfgConnAddrStr` at 0x1000e100. It clears 0x800- and 0x2000-byte buffers with `memset` at 0x1000c8d0 and 0x1000c92c before its image-data call at 0x1000c9c4. The parallel fan-out and the 0x1000b470 callee are inferred from the call pattern only.

### 6.5.12/usr/sbin/flashsc 0x1000d4c0

`SysCfgConnInit` sets up the client side of a configuration connection. It calls `sn1SCClient` (0x1000d4f0, 0x1000d5c0) and `sn1SCConnClient` (0x1000d520, 0x1000d590), initialises local L2 data with `L2CfgMInitL3` (0x1000d5ac and 0x1000d5d8, the second against `L2CfgLocal`), then issues `sn1SCCmd` at 0x1000d634 and `sn1SCConnCmd` at 0x1000d6cc. It ends by calling `SysCfgConnProcessEvent` at 0x1000d748, and logs each failure with `syslog` at priority 3. The call order is grounded; the protocol meaning of the client calls is not.

### 6.5.12/usr/sbin/flashsc 0x1000d7a0

`SysCfgConnProcessEvent` reads a structure from `a2`: byte fields at +0x24, +0x2c and +0x34 (tested against 0x7), and word fields at +0x28, +0x30 and +0x38. It fetches three values through `IRArg` at 0x1000d800, 0x1000d858 and 0x1000d8b0 (arguments 0, 1 and 2). It walks records, compares strings with `strcmp` at 0x1000da74, duplicates one with `strdup` at 0x1000dabc, and hands a match to `L2CfgMCfgL2` at 0x1000dadc. What the event codes mean is not in the facts.

### 6.5.12/usr/sbin/flashsc 0x1000de20

`SysCfgConnPrintL1` formats one L1 configuration record for output. It reads words and bytes through pointers the facts do not resolve, lowercases a character with `tolower` at 0x1000de90, measures strings with `strlen` at 0x1000deac and 0x1000deec, and builds the line with `sprintf` at 0x1000df50. The line goes to the descriptor in `a1` through `write` at 0x1000df7c, and the return is either that result or the constant 0x410f0. The record layout and field names are not in the facts.

### 6.5.12/usr/sbin/flashsc 0x1000dfc0

`SysCfgConnPrintL2` formats one L2 connection record. It converts an address with `SocketAddrStr` at 0x1000e030, builds the line with `sprintf` at 0x1000e094, and writes it to the descriptor in `a1` with `write` at 0x1000e0c0. A loop over entries ends at the branch at 0x1000e0dc. The return is either a constant or the write result, and the facts do not settle which is common.

### 6.5.12/usr/sbin/flashsc 0x1000e100

`SysCfgConnAddrStr` is a thin wrapper. It loads the `SysCfgFull` flag, and when `a0` is non-zero (branch at 0x1000e128) it passes its arguments to `L2CfgMAddrStr` at 0x1000e168 and returns that result. The zero-argument path at 0x1000e144 is not described by the facts, so what it returns is unknown.

### 6.5.12/usr/sbin/flashsc 0x1000e190

`SysCfgConsole` is a second thin wrapper, parallel to `SysCfgConnAddrStr`. It reads `SysCfgFull` and, when `a0` is non-zero (branch at 0x1000e1b4), calls `L2CfgMGetConsole` at 0x1000e1e8 and returns its result. The other path, to 0x1000e1d0, is not described by the facts. The name suggests a console lookup, but the facts do not show what the value means.

### 6.5.12/usr/sbin/flashsc 0x1000e270

`L2CfgInitLocal` sets up the local L2 configuration record. It reads `L2CfgDebug` and `L2CfgMode`, may store a new mode at 0x1000e320 on one branch, and calls `L2CfgInitNode` at 0x1000e3d0 on the local record. It reads strings from a settings store with `nvramGet` (keys 1, 3 and 4) and copies two into the record with `strcpy`. It mallocs an 0x800-byte buffer at 0x1000e454, clears it with `memset` at 0x1000e47c, and returns -1 at 0x1000e310 or 0 at 0x1000e494.

### 6.5.12/usr/sbin/flashsc 0x1000e4a0

`L2CfgMInitL3` initialises a per-connection L2 record from an input record in `a1`. It compares the mode global against 1 (branch at 0x1000e504) and may store a value at 0x1000e554, reads the byte-order global, and calls `L2CfgInitNode` at 0x1000e610 with `a2` taken from `a1+4`. It mallocs an 0x800-byte buffer at 0x1000e620, clears it at 0x1000e648, and stores a word at +0x1ef0 (0x1000e630). It returns -1 at 0x1000e544 and 0 at 0x1000e660. `SysCfgConnInit` calls it at 0x1000d5ac and 0x1000d5d8.

### 6.5.12/usr/sbin/flashsc 0x1000e670

`L2CfgMAddL2` adds an L2 entry to the list in `a0`, reading the entry description from `a3` (fields at +0x4, +0x8 and +0xc). It checks two values read from the `IR_PORT_BIAS` symbol at offsets 9001 and 9002 against the description (branches at 0x1000e750 and 0x1000e78c). It creates a node with `L2CfgInitNode` at 0x1000e840, looks up an existing entry with `L2CfgMFindL2Node` at 0x1000e8d8, and frees the created node on the paths at 0x1000e984 and 0x1000e9d4. It returns -1 at 0x1000e99c and 0 at 0x1000e9ec. The port-bias meaning and the lookup key are not stated by the facts.

### 6.5.12/usr/sbin/flashsc 0x1000ea00

`L2CfgMAddL3` is the L3 counterpart of `L2CfgMAddL2`. It builds a temporary node with `L2CfgInitNode` at 0x1000eb38 and looks up the matching L3 entry with `L2CfgMFindL3Node` at 0x1000eb70. A branch at 0x1000eb84 depends on whether that lookup found an entry, and the matched entry gets three words copied in from the node (+0x0, +0x4 and +0x8). It reads and rewrites a field at `a0+0x1ee8`, and frees the temporary node at 0x1000ebe8. It returns 0 at 0x1000ec00, and the lookup key and stored field meanings are not in the facts.
