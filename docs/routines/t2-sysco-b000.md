# Routine role entries, sysco second reading list, batch 0

Role statements for the first 25 targets of the ranked reading list. Each role is written
from the rendered instruction facts only: registers, callees, constants and branch targets
at the cited addresses. None is marked established. Where the facts leave a role open
(indirect callees, field meanings, command codes), the entry says so.

- `__start` in `6.5.12/usr/sbin/flashsc` at 0x10006110
  **Inferred.** Role (from name): process entry stub, from the symbol `__start`. It makes word loads and stores at bases the facts do not resolve (stores at 0x1000616c, 0x10006184 and 0x10006198), including a value read from 0x40e40 that it tests at 0x1000615c, and calls a local helper at 0x10006130 from 0x10006128. Four calls at 0x100061a4, 0x100061b8, 0x100061f8 and 0x10006210 have no resolved callee; the one at 0x100061f8 is passed the words read from 0x4137c, 0x41380 and 0x40e40 as a0 to a2. What the routine does after setup is unknown.
  Reviewed: 2026-11-12 (sonnet)

- `_mcount` in `6.5.12/usr/sbin/flashsc` at 0x1000621c
  **Inferred.** A 12-byte stub with one return at 0x10006220. It makes no calls and writes no memory, and v0 at the return is unknown. The name suggests a profiling hook, but that comes from the symbol, not the body.
  Reviewed: 2026-11-12 (sonnet)

- `main` in `6.5.12/usr/sbin/flashsc` at 0x100062b0
  **Inferred.** Role (from name): program entry of the tool, from the symbol `main`. It calls `sn1SCParseCmdline` at 0x1000632c with a1 and a3 at entry (a0 and a2 are stack addresses) and, when the result is negative, calls `fprintf` at 0x10006358 and `exit(-1)` at 0x10006368. It calls `strcmp` against constant strings at 0x100063a8, 0x100063dc, 0x10006414, 0x10006938, 0x100069b0, 0x100069ec and 0x10006a30 (the first operand is not shown), and `sscanf` at 0x10006a70 with a format at 0x100475d0 and two stack slots, printing with `fprintf` at 0x10006aa0 unless it returns 2. On separate paths it calls `FlashFileOpen` at 0x1000654c (a negative result exits -1), `FlashImageType` at 0x100065e0, and `FlashValidateImage` at 0x10006750, 0x100067f4 and 0x1000686c (a negative result prints and exits -1). Other paths call `sn1SCInit` at 0x10006ab8 (a negative result exits 1), `FlashUpdateRemoteRaw` at 0x10006b74 with a2 = 0x40000, `SysCfgConnInit` at 0x10006bf8 and 0x100072c4, `FlashUpdateRemoteParallel` at 0x10007250, and `FlashUpdateRemote` at 0x100073e4, 0x100075f0 and 0x1000770c. It returns 0 at 0x1000776c. The option strings are not in the facts, so which option reaches which call is not known.
  Reviewed: 2026-11-12 (sonnet)

- `FlashFileOpen` in `6.5.12/usr/sbin/flashsc` at 0x10008000
  **Inferred.** Calls `open` at 0x1000805c with a0 and a2 at entry; a negative result prints `errno` with `fprintf` at 0x1000809c and returns -1 without a close. On success it calls `fstat` at 0x100080c8, `malloc` at 0x10008150, `read` at 0x100081d8 into the malloc buffer, and `FlashValidateFileHeader` at 0x10008260 with a2 = a0 at entry. A failed `fstat`, `malloc` or short `read` prints `errno` with `fprintf`, calls `close` and returns -1; a negative `FlashValidateFileHeader` result calls `close` at 0x10008288 and returns -1 with no `fprintf`. On success it makes three stores (0x100082b0, 0x100082bc, 0x100082c8) and returns the open descriptor (v0 = ret(open)) at 0x100082dc, so the descriptor is closed only on the failure paths.
  Reviewed: 2026-11-12 (sonnet)

- `FlashFileClose` in `6.5.12/usr/sbin/flashsc` at 0x100082f0
  **Inferred.** Branches on a4 at 0x10008320: when a4 is zero it skips the `lseek` and the `write`. When a4 is non-zero it calls `lseek` at 0x10008338 with a0 at entry and a1 = a2 = 0; a negative result prints `errno` with `fprintf` at 0x1000837c and returns -1 without closing. Otherwise it calls `write` at 0x100083ac with a0 at entry, a2 at entry as the buffer and a3 at entry as the length, and if the count differs from a3 it prints with `fprintf` at 0x100083f4 and returns -1 without closing. Every path that does not return -1 calls `close` on a0 at 0x1000841c and returns 0 at 0x10008434. Reading a4 as a write-back flag is inferred from the call pattern.
  Reviewed: 2026-11-12 (sonnet)

- `FlashValidateFileHeader` in `6.5.12/usr/sbin/flashsc` at 0x10008440
  **Inferred.** Calls `ntohl` on the word at a0+0x20 at 0x1000847c. When the result is not -1 (branch at 0x1000848c) it goes to 0x10008524. When it is -1 it calls `ntohl` on the word at a0+0x24 at 0x100084a0; a result below 3 (signed, branch at 0x100084b0) goes to 0x10008510, otherwise it calls `ntohl` on the +0x24 word again at 0x100084c4, calls `fprintf` at 0x100084f0 (a2 at entry, a3 = that result, a4 = 2) and returns -1 at 0x10008508. The other return is v0 = one of 0x20, 0x28 at 0x10008540. The facts do not say what the fields mean or which path reaches that return.
  Reviewed: 2026-11-12 (sonnet)

- `FlashImageType` in `6.5.12/usr/sbin/flashsc` at 0x10008550
  **Inferred.** Role (from name): classifies an image type. It compares a0 against five 64-bit constants (0xffffffffffe00000, 0xffffffffffe10000, 0xffffffffffe80000, 0xfffffffffe000000 and 0xfffffffffe040000) at branches 0x10008594 through 0x100085d4, then compares a1 unsigned against 0x10000 (0x100085f0), 0x80000 (0x10008618) and 0x40000 (0x100086cc) on the paths that follow. It returns -1 at 0x10008754 and 0 at 0x100087c8. Before the 0 return it stores a word through each of a2, a3, a4 and a5 that is non-zero, at 0x10008770, 0x10008788, 0x100087a0 and 0x100087b8; the stored values are not in the facts. The meaning of each class is unknown.
  Reviewed: 2026-11-12 (sonnet)

- `FlashValidateImage` in `6.5.12/usr/sbin/flashsc` at 0x100087d0
  **Inferred.** Role (from name): validates an image header at a0. It reads a word at a0+0x0 (0x10008814), converts it with `ntohl` at 0x1000881c, converts other fields with `ntohl` and `ntohs` (0x100088bc) and compares the converted values and their sums with a1 and a2 unsigned (branches at 0x10008838, 0x10008854 and 0x100088e8). It returns -1 at 0x10008898, 0x1000892c, 0x100089fc, 0x10008a64, 0x10008b80 and 0x10008cc8; each of those is preceded by a `fprintf` (0x10008880, 0x10008914, 0x100089e4, 0x10008a4c, 0x10008b68 and 0x10008cb0) that runs only when a3 at entry is non-zero. After the checks a3 at entry selects the dump calls: `FlashDumpImageHeader` at 0x10008bac and `FlashDumpRelocTable` at 0x10008cf8 run when a3 is 1, and `FlashDumpImageID` at 0x10008be0 when a3 is 2. It stores a word through a4 (0x10008d14) and through a5 (0x10008d2c) when each is non-zero, and returns 0 at 0x10008d40.
  Reviewed: 2026-11-12 (sonnet)

- `FlashImageData` in `6.5.12/usr/sbin/flashsc` at 0x10008d50
  **Inferred.** Returns 0 at 0x10008da4 when (a3 & 3) is non-zero (branch at 0x10008d8c) and at 0x10008de4 when `malloc` at 0x10008db4 (a0 = a3 at entry) returns zero. Otherwise it calls `memcpy` at 0x10008e04 with a0 = the malloc result, a1 = a0 + a2 and a2 = a3 at entry. When a4 at entry is zero it returns the malloc result at 0x10008e28. When a4 is non-zero it calls `ntohl` on the words at a5+0xc (0x10008e3c) and a5+0x4 (0x10008e68), loops over a range with unsigned compares (branch at 0x10009090), calls `ntohs` and `htons` at 0x10008fc4 to 0x1000901c with a7 = 0xffffff at the `ntohs` calls, stores halfwords (0x10008fec and 0x10009030), stores a word at 0x100090ac when a6 is non-zero, and returns the malloc result at 0x100090c0. The field meanings are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashDumpImageHeader` in `6.5.12/usr/sbin/flashsc` at 0x100090d0
  **Inferred.** Prints the fields of an image header. It loads 16-bit values at a1+0x0 and a1+0x2 and 32-bit values at a1+0x4, +0x8, +0xc, +0x10, +0x14, +0x18, +0x1c and +0x20, converts them with `ntohs` and `ntohl`, and prints each with `fprintf` using format strings at 0x10042c58 to 0x10042d98. It returns 0 at 0x10009350. The labels the format strings print are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashDumpImageID` in `6.5.12/usr/sbin/flashsc` at 0x10009360
  **Inferred.** Role (from name): dumps an identifier record. It calls `ntohl` on the words at a1+0x14 (0x10009390), a1+0x10 (0x100093ac) and a1+0x4 (0x100093d0), then compares the last result unsigned with 0xffffffffffe00000 (branch at 0x100093e4). It calls `fprintf` once at 0x10009458 with format 0x10042db8, a2 = one of the constants the facts list (0x100475f0) and a3 to a7 built from an `ntohl` result plus a register, with offsets +4, +6 and +8 among them. It returns 0 at 0x10009470. The facts do not show what the printed values mean.
  Reviewed: 2026-11-12 (sonnet)

- `FlashDumpRelocTable` in `6.5.12/usr/sbin/flashsc` at 0x10009480
  **Inferred.** Role (from name): dumps a relocation table. It calls `ntohl` on the word at a1+0x4 (0x100094b4), then `fprintf` at 0x100094fc, then `ntohl` on the words at a1+0x0 (0x10009510), +0x8 (0x10009560) and +0xc (0x10009594), each followed by `fprintf` (0x10009530, 0x1000954c, 0x10009580 and 0x100095b4), and a further `fprintf` at 0x100095dc with a3 = a1+16. It then loops (branch at 0x10009728 back to 0x10009604) from a1+16 while the pointer is below a1 plus an `ntohl` result, compares a value with 1, 2 and 3 (0x10009640, 0x10009650 and 0x10009660; the value is not shown), calls `fprintf` (0x10009684, 0x100096a8 and 0x100096f4) and calls `ntohl` at 0x100096d0. It returns 0 at 0x10009740. The record layout beyond these offsets is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashUpdateRemote` in `6.5.12/usr/sbin/flashsc` at 0x10009750
  **Inferred.** Role (from name): remote update driver. It calls `FlashImageType` at 0x100097c4 and `FlashValidateImage` at 0x100097e8 (a negative result prints with `fprintf` at 0x10009814 and returns -1), and calls `FlashValidateImage` again at 0x10009868 when a2 and a3 at entry are non-zero (same failure handling, 0x10009894 and 0x100098ac). It calls `sn1SCCmd` at 0x1000998c, 0x10009e48, 0x1000a3e8 and 0x1000a798 with a1 = 0x72, 0x74, 0x73 and 0x72. Each of those has a failure path that calls `LoggerDefaultMessage` and `syslog` with priority 3, then, behind two branches (for the first, 0x10009a3c and 0x10009a4c), `syslog` with priority 6 and `sleep(15)` (0x10009a84, 0x10009f3c, 0x1000a4e0 and 0x1000a890), and a branch back (0x10009ac8 for the first) to a point before the `sn1SCCmd` call when a value is below 2. It calls `FlashImageData` at 0x1000a320 (a zero result prints with `fprintf` and returns -1) and `IRouterFreePacket` and `free` at several sites. It returns -1 on its error paths, 0 at 0x1000aaa8, and an unshown value (v0 = ?) at 0x10009af0, 0x10009fa8, 0x1000a54c and 0x1000a8fc. The meaning of the command codes is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `FlashUpdateRemoteRaw` in `6.5.12/usr/sbin/flashsc` at 0x1000aab0
  **Inferred.** Role (from name): remote update variant. The facts show no call to `FlashImageType` or `FlashValidateImage`. It calls `sn1SCCmd` at 0x1000ac0c (a1 = 0x74) and 0x1000b0ec (a1 = 0x73), each with a failure path that calls `syslog` with priority 3, then priority 6 and `sleep(15)` (0x1000ad00 and 0x1000b1e4) behind guarding branches. It calls `printf` at 0x1000ab14 with format 0x10043060, a1 and a2 at entry, a3 = 0x2000 and a5 = 0x20202020, and `FlashImageData` at 0x1000b020 with a0 and a1 at entry (a zero result prints and returns -1). It returns -1 on its error paths, an unshown value (v0 = ?) at 0x1000ad6c and 0x1000b250, and 0 at 0x1000b468. `main` calls it at 0x10006b74 with a2 = 0x40000. Whether the raw variant skips validation on purpose is inferred, not shown.
  Reviewed: 2026-11-12 (sonnet)

- `FlashUpdateRemoteParallel` in `6.5.12/usr/sbin/flashsc` at 0x1000ba10
  **Inferred.** Role (from name): parallel remote update. It calls `FlashImageType` at 0x1000ba88 and `FlashValidateImage` at 0x1000baac (a negative result prints at 0x1000bad8 and returns -1), and again at 0x1000bb58 when a2 and a3 at entry are non-zero. It calls an unnamed local function at 0x1000b470 from 0x1000bcec (a2 = 0x72), 0x1000c4dc (a2 = 0x74), 0x1000cbe0 (a2 = 0x73) and 0x1000d028 (a2 = 0x72) and returns its result when it is negative. It calls `SysCfgConnAddrStr` at sites from 0x1000be40 to 0x1000d3b8 and passes the results to `fprintf` and `printf` (for example 0x1000becc and 0x1000c048). It clears 0x800 bytes at 0x1000c8d0 and 0x2000 bytes at 0x1000c92c with `memset` and calls `FlashImageData` at 0x1000c9c4 (a zero result prints and returns -1 at 0x1000ca40). It returns -1 at 0x1000baf0, 0x1000bb9c and 0x1000ca40, and 0 at 0x1000d4ac. The local function at 0x1000b470 is not named in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `SysCfgConnInit` in `6.5.12/usr/sbin/flashsc` at 0x1000d4c0
  **Inferred.** Role (from name): connection setup. Its calls depend on a0 at entry. `sn1SCClient` is called at 0x1000d4f0 (a0 to a3 at entry) when a0 is zero (branch at 0x1000d4e4). `sn1SCConnClient` at 0x1000d520 runs only when a0 is non-zero (branch at 0x1000d510); a zero result calls `syslog` at 0x1000d560 and returns its value at 0x1000d574. The branch at 0x1000d580 jumps to 0x1000d5bc when a0 is zero, just before the `sn1SCClient` call at 0x1000d5c0; its fall side calls `sn1SCConnClient` at 0x1000d590 and `L2CfgMInitL3` at 0x1000d5ac with a0 = a0+80 and a1 = the `sn1SCConnClient` result. `L2CfgMInitL3` at 0x1000d5d8 is called with a0 = `L2CfgLocal` and a1 = the `sn1SCClient` result. `sn1SCCmd` at 0x1000d634 (a1 = 0, a2 = 1) runs only when a0 is zero (branch at 0x1000d610) and `sn1SCConnCmd` at 0x1000d6cc only when a0 is non-zero (branch at 0x1000d6a4); a non-zero result from either calls `LoggerDefaultMessage` and `syslog` with priority 3. The call to `SysCfgConnProcessEvent` at 0x1000d748 (a0 and a1 at entry) is followed by a branch on its result; a negative result leads to the `syslog` call at 0x1000d780. The return value is one of ret(syslog) and ret(`SysCfgConnProcessEvent`).
  Reviewed: 2026-11-12 (sonnet)

- `SysCfgConnProcessEvent` in `6.5.12/usr/sbin/flashsc` at 0x1000d7a0
  **Inferred.** Role (from name): handles one event record. It reads bytes at a2+0x24, +0x2c and +0x34 and, when (byte & 7) is zero, the words at +0x28, +0x30 and +0x38; it calls `IRArg` at 0x1000d800, 0x1000d858 and 0x1000d8b0 with a0 = a2 at entry and a1 = 0, 1 and 2. It calls `strcmp` at 0x1000da74, `free` at 0x1000daac and `strdup` at 0x1000dabc (operands not shown) and `L2CfgMCfgL2` at 0x1000dadc with a1 = a2 at entry. It calls `syslog` with priority 3 at 0x1000da34 and returns -1 at 0x1000da4c; it returns 0 at 0x1000da98 when `strcmp` returns zero and at 0x1000de0c. The record field meanings are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `SysCfgConnPrintL1` in `6.5.12/usr/sbin/flashsc` at 0x1000de20
  **Inferred.** Role (from name): prints an L1 record. It tests a0 for zero (branch at 0x1000de44) and loops (branch at 0x1000df98 back to 0x1000de80) over values loaded from its argument. In the loop it calls `tolower` at 0x1000de90, `strlen` at 0x1000deac and 0x1000deec (a branch on each result), `sprintf` at 0x1000df50 (a stack buffer, format 0x10043740), `strlen` on that buffer at 0x1000df60 and `write` at 0x1000df7c with a0 = a1 at entry, the buffer and the length. It returns v0 = one of 0x410f0 and the `write` result at 0x1000dfac. The meaning of the fields it prints is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `SysCfgConnPrintL2` in `6.5.12/usr/sbin/flashsc` at 0x1000dfc0
  **Inferred.** Role (from name): prints an L2 record. It tests a0 for zero (branch at 0x1000dfe4) and loops (branch at 0x1000e0dc back to 0x1000e020). In the loop it calls `SocketAddrStr` at 0x1000e030, `sprintf` at 0x1000e094 (format 0x10043760, a3 = the `SocketAddrStr` result), `strlen` at 0x1000e0a4 and `write` at 0x1000e0c0 with a0 = a1 at entry. It returns v0 = one of 0x40f50 and the `write` result at 0x1000e0f0.
  Reviewed: 2026-11-12 (sonnet)

- `SysCfgConnAddrStr` in `6.5.12/usr/sbin/flashsc` at 0x1000e100
  **Inferred.** Role (from name): wrapper that formats a connection address. It branches on a0 at 0x1000e128 (taken to 0x1000e144 when a0 is zero) and loads a word at 0x1000e148; the facts name no symbol for that word and do not show whether the load is skipped when a0 is non-zero. It calls `L2CfgMAddrStr` at 0x1000e168 with a1 to a3 at entry (a0 is not shown) and returns its result at 0x1000e17c. The facts do not show how the first argument reaches the callee.
  Reviewed: 2026-11-12 (sonnet)

- `SysCfgConsole` in `6.5.12/usr/sbin/flashsc` at 0x1000e190
  **Inferred.** Role (from name): wrapper that resolves a console, from the symbol. It branches on a0 at 0x1000e1b4 (taken to 0x1000e1d0 when a0 is zero) and loads a word at 0x1000e1d4; the facts name no symbol for that word and do not show whether the load is skipped when a0 is non-zero. It calls `L2CfgMGetConsole` at 0x1000e1e8 with a1 to a3 at entry (a0 is not shown) and returns its result at 0x1000e1fc. The callee's behavior is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `L2CfgInitLocal` in `6.5.12/usr/sbin/flashsc` at 0x1000e270
  **Inferred.** Initializes the local L2 config record `L2CfgLocal`. It reads `L2CfgDebug` (non-zero calls `fprintf` at 0x1000e2c0) and `L2CfgMode`; when `L2CfgMode` is non-zero it calls `syslog` at 0x1000e2f8 and returns -1 at 0x1000e310. Otherwise it stores a word at 0x1000e320 (base and value not shown), calls `L2CfgInitNode` at 0x1000e3d0 with a0 = `L2CfgLocal`, a1 = 1 and a2 = a0 at entry, and calls `nvramGet` at 0x1000e3e0, 0x1000e3fc and 0x1000e428 with a0 = 1, 3 and 4. It stores a word at +0x18 after the first call (0x1000e3f0), copies the second and third results to `L2CfgLocal`+28 and +48 with `strcpy` at 0x1000e418 and 0x1000e444, allocates 0x800 bytes at 0x1000e454, stores a word at +0x1ef0 (0x1000e464) and clears the buffer at [`L2CfgLocal`+7920] with `memset` at 0x1000e47c. It returns 0 at 0x1000e494. The NVRAM key names are not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `L2CfgMInitL3` in `6.5.12/usr/sbin/flashsc` at 0x1000e4a0
  **Inferred.** Role (from name): initializes a layer-3 config node. It works on the record at a0 and the descriptor at a1. It reads `L2CfgDebug` (non-zero calls `fprintf` at 0x1000e4f0) and `L2CfgMode`; when `L2CfgMode` is 1 it calls `syslog` at 0x1000e52c and returns -1 at 0x1000e544. Otherwise it stores a word at 0x1000e554 (base and value not shown), reads `IRByteOrder` and words at a1+0x4, calls `L2CfgInitNode` at 0x1000e610 with a0 at entry, a1 = 1, a2 = the word at a1+0x4, a4 = a1 at entry and a5 = 2, allocates 0x800 bytes at 0x1000e620, stores a word at a0+0x1ef0 (0x1000e630) and clears the buffer at [a0+0x1ef0] with `memset` at 0x1000e648. It returns 0 at 0x1000e660.
  Reviewed: 2026-11-12 (sonnet)

- `L2CfgMAddL2` in `6.5.12/usr/sbin/flashsc` at 0x1000e670
  **Inferred.** Role (from name): adds a layer-2 node. It reads `L2CfgDebug` (non-zero calls `fprintf` at 0x1000e6d8), tests a3 for zero (0x1000e6e4), loads words at +0x8, +0x4 and +0xc, and loads `IR_PORT_BIAS` at sites from 0x1000e744 to 0x1000e854 (compared at 0x1000e750, 0x1000e78c and 0x1000e860). It calls `L2CfgInitNode` at 0x1000e840 (a0 = 0, a1 = 1, a4 = a3 at entry) and `L2CfgMFindL2Node` at 0x1000e8d8 with a0 at entry, two words read through the `L2CfgInitNode` result and a3 = 0. It stores words at several offsets (values not shown) and calls `free` on the `L2CfgInitNode` result at 0x1000e984 (then returns -1 at 0x1000e99c) and at 0x1000e9d4 (then returns 0 at 0x1000e9ec). Which lookup result counts as a duplicate is not in the facts.
  Reviewed: 2026-11-12 (sonnet)

- `L2CfgMAddL3` in `6.5.12/usr/sbin/flashsc` at 0x1000ea00
  **Inferred.** Role (from name): adds a layer-3 node. It reads `L2CfgDebug` (non-zero calls `fprintf` at 0x1000ea60) and `IRByteOrder` (branch at 0x1000ea84), calls `L2CfgInitNode` at 0x1000eb38 (a0 = 0, a1 = 0, a2 = a1 at entry, a4 = a2 at entry), stores a word at +0xc (0x1000eb50) and calls `L2CfgMFindL3Node` at 0x1000eb70 with a0 at entry. When the lookup returns zero (fall side of the branch at 0x1000eb84) it stores words at +0x14 (0x1000eb98) and +0x1ee8 (0x1000eba4). When it returns non-zero (taken side, to 0x1000ebb0) it stores words at +0x0, +0x4 and +0x8 (0x1000ebbc to 0x1000ebdc) and calls `free` on the `L2CfgInitNode` result at 0x1000ebe8. The only return is 0 at 0x1000ec00. The bases and values of these stores are not in the facts.
  Reviewed: 2026-11-12 (sonnet)
