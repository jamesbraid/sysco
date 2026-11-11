# Tier-2 routine role statements, sysco batch 1

These statements describe what each routine does in software, read from its rendered instruction facts. They say nothing about hardware behavior. Where the facts do not settle a routine's purpose, the statement says so and the grade is lower.

- `L2CfgMDeleteNode` in `6.5.12/usr/sbin/flashsc` at 0x1000ec10

  It tests the `L2CfgDebug` flag and logs through `fprintf` when it is set. It then runs a sequence of peer and client removals: `L2CfgMDeleteL2Peer` at 0x1000fcb0 on the argument, and `L2CfgDeleteL2Client` at 0x1000fd70 twice, on fields at offsets +7916 and +7912 of the argument. An unnamed routine at 0x1000ef70 is called between these steps, and its return value gates each branch. The meaning of the structure and of the 0x1000ef70 callee is not established by the facts.

  Evidence grade: inferred.

- `L2CfgInitNode` in `6.5.12/usr/sbin/flashsc` at 0x1000ee20

  It logs through `fprintf` under the `L2CfgDebug` flag. It then calls `malloc` with a size that is either 0x18 or 0x1ef4, and `memset` zeroes the block, so it appears to allocate and clear a node record. It returns either 0 or the allocated pointer. The reason for the two sizes is not shown in the facts.

  Evidence grade: inferred.

- `L2CfgMFindL2Node` in `6.5.12/usr/sbin/flashsc` at 0x1000f230

  It logs under the `L2CfgDebug` flag and makes two calls to the unnamed routine at 0x1000ef70, passing its arguments through. It returns that routine's result or 0. This looks like a lookup of a level-2 node, but the callee's behavior is not known from the facts.

  Evidence grade: unknown.

- `L2CfgMFindL3Node` in `6.5.12/usr/sbin/flashsc` at 0x1000f390

  It has the same shape as `L2CfgMFindL2Node` at 0x1000f230: a debug log, one call to 0x1000ef70 with the arguments passed through, and a return of that result or 0. It is most likely the level-3 counterpart of that lookup. Its purpose is inferred from the parallel shape, not from its own facts.

  Evidence grade: inferred.

- `L2CfgMFindNodeLocal` in `6.5.12/usr/sbin/flashsc` at 0x1000f480

  It logs under the debug flag and calls `SocketGetSockAddr` at 0x10026a00 three times, each time with a stack buffer at sp-56. Several return paths leave a value of unknown origin, and one returns 0. It seems to find a node by socket address, but the facts do not show what the matched value is.

  Evidence grade: inferred.

- `L2CfgMFindNode` in `6.5.12/usr/sbin/flashsc` at 0x1000f6f0

  It logs under the debug flag, then calls `L2CfgMFindL2Node` at 0x1000f230 and `L2CfgMFindL3Node` at 0x1000f390 with the same arguments. Its return is the result of one of those two calls. It acts as a dispatcher over the two level lookups.

  Evidence grade: inferred.

- `L2CfgMFindDevice` in `6.5.12/usr/sbin/flashsc` at 0x1000f7d0

  It formats a string with `sprintf` from the format at 0x10043a40 into the static buffer at 0x1004a2b8. Its returns are the static pointers 0x10047618 and 0x1004a2b8, or a value of unknown origin. The facts do not show how a device is matched, so the lookup itself is unclear.

  Evidence grade: unknown.

- `L2CfgMFindPort` in `6.5.12/usr/sbin/flashsc` at 0x1000f920

  It makes no calls that the facts resolve. Its returns are 0 and a value of unknown origin. The facts are too thin to say what it searches.

  Evidence grade: unknown.

- `L2CfgMGetPort` in `6.5.12/usr/sbin/flashsc` at 0x1000fa40

  It makes no calls that the facts resolve. It returns 0 on one path and a value of unknown origin on another. Its role is likely a port accessor, but the facts do not establish that.

  Evidence grade: unknown.

- `L2CfgMDeviceName` in `6.5.12/usr/sbin/flashsc` at 0x1000fac0

  It calls `L2CfgMFindPort` at 0x1000f920, then formats text with several `sprintf` calls into the static buffer at 0x1004a308. It calls `L2CfgMAddrStr` at 0x10015030 for an address field, and it reads two fields at offsets +12 and +16 of the port result. It returns that buffer or the string at 0x10043ac0. It builds a printable name for a device.

  Evidence grade: inferred.

- `L2CfgMDeleteL2Peer` in `6.5.12/usr/sbin/flashsc` at 0x1000fcb0

  It calls `free` on its second argument, with a constant 0x3f260 among the arguments, and returns 0 on both exits. It releases a peer record. The constant's meaning is not shown.

  Evidence grade: inferred.

- `L2CfgDeleteL2Client` in `6.5.12/usr/sbin/flashsc` at 0x1000fd70

  It calls `free` on its second argument and returns 0 on both exits. It releases a client record.

  Evidence grade: inferred.

- `L2CfgMEventRack` in `6.5.12/usr/sbin/flashsc` at 0x1000fe50

  It calls `nvramGet` at 0x10016d80 with the constant key 1 and returns the result. It reads one NVRAM value. The name of that value is not in the facts, so the key's meaning stays open.

  Evidence grade: inferred.

- `L2CfgMEventSystemSN` in `6.5.12/usr/sbin/flashsc` at 0x1000fea0

  It calls `nvramGet` at 0x10016d80 with the constant key 3, then copies the result with `strcpy` into the argument at offset +28. The copy target and the NVRAM key both suggest a serial number. The key's name is not established by the facts.

  Evidence grade: inferred.

- `FmtAddr` in `6.5.12/usr/sbin/flashsc` at 0x1000ff00

  It calls `sprintf` with the caller's buffer, the format at 0x10043ad0, and the address argument, then returns the buffer. It is a small address formatter.

  Evidence grade: inferred.

- `FmtBrick` in `6.5.12/usr/sbin/flashsc` at 0x1000ff80

  It calls `tolower`, formats a value with `sprintf` from the format at 0x10043ae0, and fills stacked bytes with 0x2d, the ASCII hyphen, using two `memset` calls. It then joins three string literals with `strcat`. It appears to build a hyphen-separated brick identifier. The fields are not named in the facts.

  Evidence grade: inferred.

- `L2CfgMPrint` in `6.5.12/usr/sbin/flashsc` at 0x10010bc0

  It allocates a 0x320-byte buffer with `malloc` and then appends many fields, each by a `sprintf` call followed by a `strlen` call, to a growing string. It returns that buffer or a value of unknown origin. It formats a multi-line status report. The full field list is too long for the facts to recover in one pass, so only the structure is stated here.

  Evidence grade: inferred.

- `L2CfgMAddPort` in `6.5.12/usr/sbin/flashsc` at 0x10011a30

  It measures two string arguments with `strlen`, allocates a copy of each with `malloc`, and copies them with `strcpy`. It frees earlier copies on some paths. It returns -1 on failure and 0 on success. It stores two owned strings for a new port entry.

  Evidence grade: inferred.

- `L2CfgMDeletePort` in `6.5.12/usr/sbin/flashsc` at 0x10011c10

  It calls `L2CfgMFindPort` at 0x1000f920 with its arguments, then returns -1 when the port is not found and 0 otherwise. The facts do not show the removal step itself, so this is a lookup-then-remove routine whose removal half is open.

  Evidence grade: inferred.

- `L2CfgMCfgL2_rev1` in `6.5.12/usr/sbin/flashsc` at 0x10011cc0

  It makes five calls to `IRArg` at 0x10021810, with the selector constants 0, 1, and 2 and a mask of 0x7f among the arguments, and it calls `strcpy` twice. It looks like the argument parser for a revision-1 form of the level-2 configuration. Its return value is unknown from the facts.

  Evidence grade: inferred.

- `L2CfgMCfgL2` in `6.5.12/usr/sbin/flashsc` at 0x100122d0

  It reads `L2CfgMode` (three loads), `L2CfgDebug`, and `IR_PORT_BIAS`. It calls `IRArg`, then `L2CfgMCfgL2_rev1` at 0x10011cc0. It parses a comma-separated string with `strdup`, `strchr`, and `strtok`, and it compares the tokens against string literals with `strcmp`. It converts values with `inet_addr`, `strtoul` in base 16, and `atoi`. It calls `L2CfgMFindNode` at 0x1000f6f0 and `L2CfgMAddL2` at 0x1000e670, the latter with port value `IR_PORT_BIAS` plus 9001. It looks like the main level-2 configuration parser. The keyword meanings are not in the facts.

  Evidence grade: inferred.

- `L2CfgMCfgPort` in `6.5.12/usr/sbin/flashsc` at 0x10013660

  It calls `IRArg` and clears two stack buffers of 100 and 44 bytes with `memset`. It copies and compares them with `memcpy` and `memcmp`, frees storage, and calls `L2CfgMGetConsole` at 0x10015610 with -1. It returns 0. It seems to parse and store a port configuration record, but the record layout is not established.

  Evidence grade: inferred.

- `L2CfgMFindBrickRS` in `6.5.12/usr/sbin/flashsc` at 0x10013a10

  It makes no calls that the facts resolve. It returns 0 on one path and -1 on three. Its purpose is not established by the facts. The name suggests a brick lookup by a RS identifier, but nothing in the facts confirms that.

  Evidence grade: unknown.

- `L2CfgMFindBrickIP` in `6.5.12/usr/sbin/flashsc` at 0x10013c00

  It makes no calls that the facts resolve. It returns 0 on one path and -1 on another. The name suggests a brick lookup by IP address, but the facts do not show the lookup.

  Evidence grade: unknown.

- `L2CfgMFindBrickCfg` in `6.5.12/usr/sbin/flashsc` at 0x10013e30

  It makes no calls that the facts resolve. It returns 0 on two paths and a value of unknown origin on another. The name suggests a brick configuration lookup, but the facts are too thin to confirm it.

  Evidence grade: unknown.
