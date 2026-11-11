# Routine role entries, sysco second reading list, batch 0

Role statements for the first 25 targets of the ranked reading list. Each role is written
from the rendered instruction facts only: registers, callees, constants and branch targets
at the cited addresses. None is marked established. Where the facts leave a role open
(indirect callees, field meanings, command codes), the entry says so.

- `__start` in `6.5.12/usr/sbin/flashsc` at 0x10006110
  **Inferred.** Startup-shaped stub. It loads and stores a few global cells (tables at 0x40e40, 0x4137c and 0x41380), calls a local helper at 0x10006130 from 0x10006128, and branches on a value read from 0x40e40 at 0x1000615c. Its four indirect calls at 0x100061a4, 0x100061b8, 0x100061f8 and 0x10006210 have no resolved targets, so the role past startup setup is unknown.

- `_mcount` in `6.5.12/usr/sbin/flashsc` at 0x1000621c
  **Inferred.** A 12-byte stub with one return at 0x10006220. It makes no calls and writes no memory, and v0 at the return is unknown. The name suggests a profiling hook, but that comes from the symbol, not the body.

- `main` in `6.5.12/usr/sbin/flashsc` at 0x100062b0
  **Inferred.** Command-line driver. It parses arguments with `sn1SCParseCmdline` at 0x1000632c, compares them with `strcmp` at 0x100063a8, 0x100063dc, 0x10006414 and later, and reads one option with `sscanf` at 0x10006a70. A file branch reaches `FlashFileOpen` at 0x1000654c, `FlashImageType` at 0x100065e0 and `FlashValidateImage` at 0x10006750. A remote branch reaches `sn1SCInit` at 0x10006ab8, `FlashUpdateRemoteRaw` at 0x10006b74, `SysCfgConnInit` at 0x10006bf8 and `FlashUpdateRemote` at 0x100073e4. The option strings are not in the facts, so the option-to-branch mapping is inferred.

- `FlashFileOpen` in `6.5.12/usr/sbin/flashsc` at 0x10008000
  **Inferred.** Opens the image file with `open` at 0x1000805c, calls `fstat` at 0x100080c8, and on success mallocs a buffer, reads the file into it with `read` at 0x100081d8, and validates the header with `FlashValidateFileHeader` at 0x10008260. Failure paths print `errno` through `fprintf` and return -1. The close on the validation-failure path at 0x10008288 and the success return of the descriptor at 0x100082dc do not fit together cleanly from the facts, so who owns the descriptor is unclear.

- `FlashFileClose` in `6.5.12/usr/sbin/flashsc` at 0x100082f0
  **Inferred.** Rewinds the descriptor with `lseek` to offset 0 at 0x10008338. When the flag in a4 is non-zero (branch at 0x10008320), it writes a3 bytes from a2 with `write` at 0x100083ac and checks the count at 0x100083c8. It always closes the descriptor at 0x1000841c and returns -1 on the seek and write error paths or 0 at 0x10008434. Reading a4 as a write-back flag is inferred from the call pattern.

- `FlashValidateFileHeader` in `6.5.12/usr/sbin/flashsc` at 0x10008440
  **Inferred.** Reads big-endian words at offsets +0x20 and +0x24 of a0 through `ntohl` at 0x1000847c, 0x100084a0 and 0x100084c4. It compares the word at +0x20 with -1 (branch at 0x1000848c) and a count below 3 (branch at 0x100084b0) goes to 0x10008510. On success it returns 0x20 or 0x28, which may be a header-size choice, but the facts do not say what the fields mean.

- `FlashImageType` in `6.5.12/usr/sbin/flashsc` at 0x10008550
  **Inferred.** Classifies an image by comparing a0 against five 64-bit constants (0xffffffffffe00000, 0xffffffffffe10000, 0xffffffffffe80000, 0xfffffffffe000000 and 0xfffffffffe040000) at branches 0x10008594 through 0x100085d4. It then range-checks a1 against 0x10000, 0x80000 and 0x40000 and stores the result through a2 to a5 at 0x10008770 to 0x100087b8. It returns -1 at 0x10008754 and 0 at 0x100087c8. The type names are not in the facts, so the meaning of each class is unknown.

- `FlashValidateImage` in `6.5.12/usr/sbin/flashsc` at 0x100087d0
  **Inferred.** Checks an image header at a0, reading its first word at 0x10008814 and converting fields with `ntohl` at 0x1000881c and `ntohs` at 0x100088bc. It reports each check with `fprintf` at 0x10008880, 0x10008914, 0x100089e4, 0x10008a4c, 0x10008b68 and 0x10008cb0. It calls `FlashDumpImageHeader` at 0x10008bac, `FlashDumpImageID` at 0x10008be0 and `FlashDumpRelocTable` at 0x10008cf8. It writes results through a4 and a5 at 0x10008d14 and 0x10008d2c and returns 0 at 0x10008d40 or -1 at several failure returns. The condition that selects the dump calls is not in the facts.

- `FlashImageData` in `6.5.12/usr/sbin/flashsc` at 0x10008d50
  **Inferred.** Allocates a buffer with `malloc` at 0x10008db4 and copies the image data into it with `memcpy` at 0x10008e04. It reads big-endian fields from a5 at +0x4 and +0xc through `ntohl` at 0x10008e68 and 0x10008e3c, and converts 16-bit values with `ntohs` and `htons` at 0x10008fc4 to 0x1000901c, using the mask 0xffffff. It returns the new buffer at 0x100090c0. The field meanings are not in the facts.

- `FlashDumpImageHeader` in `6.5.12/usr/sbin/flashsc` at 0x100090d0
  **Inferred.** Prints the fields of an image header. It loads 16-bit values at a1+0x0 and a1+0x2 and 32-bit values at a1+0x4, +0x8, +0xc, +0x10, +0x14, +0x18, +0x1c and +0x20, converts them with `ntohs` and `ntohl`, and prints each with `fprintf` using format strings at 0x10042c58 to 0x10042d98. It returns 0 at 0x10009350. The labels the format strings print are not in the facts.

- `FlashDumpImageID` in `6.5.12/usr/sbin/flashsc` at 0x10009360
  **Inferred.** Prints an identifier record. It reads 32-bit values at a1+0x4, +0x10 and +0x14 and converts them with `ntohl` at 0x10009390, 0x100093ac and 0x100093d0. It prints them with `fprintf` at 0x10009458 using format string 0x10042db8, choosing one of two constants (0x100475f0 or 0x100475f8) for the label. It returns 0 at 0x10009470.

- `FlashDumpRelocTable` in `6.5.12/usr/sbin/flashsc` at 0x10009480
  **Inferred.** Prints a relocation table. It reads entries from a1 at offsets +0x0, +0x4, +0x8 and +0xc and converts them with `ntohl` at 0x10009510 to 0x10009594. It loops over the entries, prints each through `fprintf` at 0x100094fc to 0x100096f4, and checks an entry count with `ntohl` at 0x100096d0. It returns 0 at 0x10009740. The record layout beyond these offsets is not in the facts.

- `FlashUpdateRemote` in `6.5.12/usr/sbin/flashsc` at 0x10009750
  **Inferred.** Remote update driver for one target. It runs `FlashImageType` at 0x100097c4 and `FlashValidateImage` at 0x100097e8 and 0x10009868, sends commands through `sn1SCCmd` at 0x1000998c, 0x10009e48, 0x1000a3e8 and 0x1000a798 with codes 0x72, 0x74, 0x73 and 0x72, and reports errors through `syslog` (priorities 3 and 6). After several error reports it calls `sleep` with 15 seconds (0xf) at 0x10009a84 and later sites. It frees the packet with `IRouterFreePacket` and calls `FlashImageData` at 0x1000a320. It returns -1 on errors and 0 at 0x1000aaa8. The meaning of the command codes is not in the facts.

- `FlashUpdateRemoteRaw` in `6.5.12/usr/sbin/flashsc` at 0x1000aab0
  **Inferred.** Remote update variant that does not call `FlashImageType` or `FlashValidateImage`. It sends commands through `sn1SCCmd` at 0x1000ac0c (code 0x74) and 0x1000b0ec (code 0x73), logs through `syslog` and sleeps 15 seconds at 0x1000ad00 and 0x1000b1e4, and calls `FlashImageData` at 0x1000b020. It prints progress with `printf` at 0x1000ab14, passing the constants 0x2000 and 0x20202020 among its arguments. It returns -1 on errors or 0 at 0x1000b468. The caller at 0x10006b74 passes 0x40000 as a2. Whether the raw variant skips validation on purpose is inferred, not shown.

- `FlashUpdateRemoteParallel` in `6.5.12/usr/sbin/flashsc` at 0x1000ba10
  **Inferred.** Parallel remote update over the configured units. It runs `FlashImageType` at 0x1000ba88 and `FlashValidateImage` at 0x1000baac and 0x1000bb58, then calls an unnamed local function at 0x1000b470 from 0x1000bcec, 0x1000c4dc, 0x1000cbe0 and 0x1000d028. It formats connection addresses with `SysCfgConnAddrStr` at sites from 0x1000be40 to 0x1000d3b8 and prints them with `printf`. It clears 0x800 and 0x2000-byte buffers with `memset` at 0x1000c8d0 and 0x1000c92c and calls `FlashImageData` at 0x1000c9c4. It returns the result of the local function or -1 at 0x1000baf0, and 0 at 0x1000d4ac. The local function at 0x1000b470 is not named in the facts.

- `SysCfgConnInit` in `6.5.12/usr/sbin/flashsc` at 0x1000d4c0
  **Inferred.** Opens the system-config client connection. It calls `sn1SCClient` at 0x1000d4f0 and 0x1000d5c0 and `sn1SCConnClient` at 0x1000d520 and 0x1000d590. It registers the L2 config layer with `L2CfgMInitL3` at 0x1000d5ac and 0x1000d5d8, with `sym(L2CfgLocal)` as the local node, and sends a command with `sn1SCCmd` at 0x1000d634 and `sn1SCConnCmd` at 0x1000d6cc. It reports failures with `syslog` and hands events to `SysCfgConnProcessEvent` at 0x1000d748.

- `SysCfgConnProcessEvent` in `6.5.12/usr/sbin/flashsc` at 0x1000d7a0
  **Inferred.** Handles one configuration event record. It reads a byte at a2+0x24, a word at +0x28, a byte at +0x2c, a word at +0x30, a byte at +0x34 and a word at +0x38, and passes a2 to `IRArg` at 0x1000d800, 0x1000d858 and 0x1000d8b0 with argument indices 0, 1 and 2. It matches names with `strcmp` at 0x1000da74, duplicates a string with `strdup` at 0x1000dabc, and calls `L2CfgMCfgL2` at 0x1000dadc. It returns -1 at 0x1000da4c, 0 at 0x1000da98 and 0x1000de0c. The record field meanings are not in the facts.

- `SysCfgConnPrintL1` in `6.5.12/usr/sbin/flashsc` at 0x1000de20
  **Inferred.** Prints one L1 record as text. It loads words and bytes from its argument, lower-cases a character with `tolower` at 0x1000de90, measures strings with `strlen` at 0x1000deac and 0x1000deec, and formats a line with `sprintf` at 0x1000df50 using format string 0x10043740. It writes the line to the descriptor in a1 with `write` at 0x1000df7c. The meaning of the fields it prints is not in the facts.

- `SysCfgConnPrintL2` in `6.5.12/usr/sbin/flashsc` at 0x1000dfc0
  **Inferred.** Prints one L2 record as text. It converts a socket address with `SocketAddrStr` at 0x1000e030, formats the line with `sprintf` at 0x1000e094 using format string 0x10043760, and writes it to the descriptor in a1 with `write` at 0x1000e0c0. It returns the write count or the constant 0x40f50 on a short path, as the facts show.

- `SysCfgConnAddrStr` in `6.5.12/usr/sbin/flashsc` at 0x1000e100
  **Inferred.** Thin wrapper that formats a connection address by calling `L2CfgMAddrStr` at 0x1000e168 with a1 to a3 passed through, and returns its result. A zero first argument takes a path (branch at 0x1000e128) that loads the global `SysCfgFull` at 0x1000e148. The facts do not show how the first argument is used by the callee.

- `SysCfgConsole` in `6.5.12/usr/sbin/flashsc` at 0x1000e190
  **Inferred.** Thin wrapper that loads the global `SysCfgFull` at 0x1000e1d4 and calls `L2CfgMGetConsole` at 0x1000e1e8 with a1 to a3 passed through, returning its result at 0x1000e1fc. The name suggests it resolves a console device for a configured unit, but the callee's behavior is not in the facts.

- `L2CfgInitLocal` in `6.5.12/usr/sbin/flashsc` at 0x1000e270
  **Inferred.** Initializes the local L2 config record `L2CfgLocal`. It reads `L2CfgDebug` and `L2CfgMode`, stores `L2CfgMode` at 0x1000e320, and calls `L2CfgInitNode` at 0x1000e3d0 on `L2CfgLocal`. It fetches three NVRAM values with `nvramGet` at 0x1000e3e0, 0x1000e3fc and 0x1000e428 and copies two of them into the record with `strcpy` at 0x1000e418 and 0x1000e444. It allocates 0x800 bytes at 0x1000e454 and clears them with `memset` at 0x1000e47c. It returns 0 at 0x1000e494 and -1 at 0x1000e310. The NVRAM key names are not in the facts.

- `L2CfgMInitL3` in `6.5.12/usr/sbin/flashsc` at 0x1000e4a0
  **Inferred.** Initializes a layer-3 config node passed in a0 from the descriptor in a1. It reads `L2CfgDebug`, `L2CfgMode` and `IRByteOrder`, stores `L2CfgMode` at 0x1000e554, and reads fields at a1+0x4. It calls `L2CfgInitNode` at 0x1000e610 with a2 set to the word at a1+0x4. It allocates 0x800 bytes at 0x1000e620, stores the pointer at a0+0x1ef0 (0x1000e630), and clears the buffer with `memset` at 0x1000e648. It returns 0 at 0x1000e660 and -1 at 0x1000e544.

- `L2CfgMAddL2` in `6.5.12/usr/sbin/flashsc` at 0x1000e670
  **Inferred.** Adds a layer-2 node built from the descriptor in a3. It reads fields at a3+0x4, +0x8 and +0xc, and reads `IR_PORT_BIAS` at several sites from 0x1000e744 to 0x1000e854. It builds a node with `L2CfgInitNode` at 0x1000e840, looks up an existing node with `L2CfgMFindL2Node` at 0x1000e8d8, and calls `free` on the new node at 0x1000e984 and 0x1000e9d4 on the failure path. It returns -1 at 0x1000e99c and 0 at 0x1000e9ec. Which lookup result counts as a duplicate is not in the facts.

- `L2CfgMAddL3` in `6.5.12/usr/sbin/flashsc` at 0x1000ea00
  **Inferred.** Adds a layer-3 node, following the same shape as `L2CfgMAddL2`. It reads `L2CfgDebug` and `IRByteOrder`, builds a node with `L2CfgInitNode` at 0x1000eb38, and looks up an existing node with `L2CfgMFindL3Node` at 0x1000eb70. It stores a value in the table at a0+0x1ee8 (0x1000eba4) and frees the node at 0x1000ebe8 on the failure path. It returns 0 at 0x1000ec00. The table layout beyond these offsets is not in the facts.
