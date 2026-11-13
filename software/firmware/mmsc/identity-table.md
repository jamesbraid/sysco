# Identity table (`identity`) and its helpers

The 21 routines defined in the `identity` object of `mmscfw.bin`, IRIX 6.5.30 (MMSC 2.3), and the six compare and format routines of the `misc` object that they and the address daemon lean on. Context: [rack-election.md](rack-election.md), [addrd-routines.md](addrd-routines.md). Sizes count code up to the last `ret`.

## Records

- **Identity** (28 bytes): +0 unique ID (8 bytes: two zero bytes, then the 6-byte MAC address), +8 IP address (4 bytes), +0xc rack ID (`0xffffffff` = unassigned), +0x10 capability flags, +0x14 and +0x18 the module numbers of the two bays (`0xffffffff` = unassigned). Read from `identInit` (the stores into `ffscLocal`, `0x1f62b8`) and `addrdExtractIdentity`.
- **Table node** (44 bytes, `malloc(0x2c)` in `identUpdate`): +0 and +4 list links, +8 and +0xc the time the entry was last refreshed (`clock_gettime(0)`, seconds then nanoseconds), +0x10 the identity. The list head is `identTable` (`0x1d8c98`). Routines that return "an identity" return the address node + 0x10.
- Bit 0 of the capability flags is tested by the TCP link ([remote-link.md](remote-link.md)): when set, a 112-byte user record is exchanged at connect time. The local value is 1. Established: bit 0 means the peer takes part in that user-record exchange. Election payloads of version 0 to 2 carry no flags field and are given flags 0 on receipt ([rack-election.md](rack-election.md)), so an older peer is treated as not exchanging the record; the only stores of the flags field are the identity set-up (value 1) and `identSetCapabilities`, whose single caller is `addrdUpdateLocalIdentity`. No other bit is tested: the only readers of the word are `remAcceptConnection` and `remOpenConnection` (bit 0 only), the copy between payload and table (`addrdExtractIdentity`, `addrdInsertIdentity`), the change check in `addrdUpdateLocalIdentity` and the printer `identPrint`.

## Start-up and teardown

- `identInit` (`0x125580`, 377 bytes). Zeroes `ffscLocal`, clears `ffscMaster` and `ffscIsMaster`. Builds the local identity: unique ID bytes 0 and 1 zero, bytes 2 to 7 the six bytes of `ffscMACAddr`; module numbers both `0xffffffff`; capability flags 1. Reads NVRAM record 1 into `ffscNetInfo` (`0x1e28b0`); if that fails, logs "Using default network parameters" under the debug bit tested at `0x1e2339` and builds a default: the host number is the low three bytes of the MAC address as a 24-bit number (bytes 5, 6, 7 of the unique ID) replaced with 0xf when it is below 0x10 or above 0xfffffe, the network number (`ffscNetInfo`+4) is 100, the mask word (+8) is `0xff000000`, and the record is written back with `nvramWrite(1)`. The local IP address is then `inet_makeaddr_b(network, host)` into `ffscLocal`+8. Reads NVRAM record 2 into the rack ID; a failed read stores `0xffffffff`, and any value above 22 (`0x16`), including `0xffffffff`, becomes 1, so a blank NVRAM gives rack ID 1. Initialises the table list and enters the local identity with `identUpdate`. Returns 0, or -1 if that update failed. Only caller: `ffscInit`. Established.
- `identClearTable` (`0x125110`, 68 bytes). Deletes and frees every node. Returns 0. No caller was found in the image (no direct call or address push); present but unused. Established.
- `identCleanOldEntries` (`0x125070`, 160 bytes). Argument: a time. Walks the table; a node is removed and freed when its refresh time is earlier than the argument (`ffscCompareTimestamps` less than 0) and its unique ID is not the local one. If the removed node was `ffscMaster`, `ffscMaster` and `ffscIsMaster` are cleared first. Returns 0. Established.

## Looking entries up

- `identFindIdentityNodeByUniqueID` (`0x125fc0`, 90 bytes). Returns the table node whose identity has the given unique ID (`ffscCompareUniqueIDs` equal to 0), or null. Established.
- `identFindByUniqueID` (`0x1253e0`, 27 bytes). The same, returning node + 0x10, or null. Established.
- `identFindByIPAddr` (`0x1252d0`, 72 bytes). Argument: a pointer to an IP address. Returns the first identity whose IP dword equals it, or null. Established.
- `identFindByModuleNum` (`0x125320`, 97 bytes). Arguments: a module number and an optional output. Returns null at once for `0xffffffff`; otherwise the first identity whose module number 0 or 1 equals it, storing the index (0 or 1) through the output when it is not null. Established.
- `identFindByRackID` (`0x125390`, 79 bytes). Returns null for `0xffffffff`; otherwise the first identity with that rack ID. Established.
- `identFindConflictingIPAddr` (`0x125400`, 102 bytes). Argument: an identity. Returns null for a null argument; otherwise the first entry with a different unique ID and the same IP dword. No caller was found in the image. Established.
- `identFindConflictingRackID` (`0x125470`, 118 bytes). Argument: an identity. Returns null for a null argument or an unassigned rack ID; otherwise the first entry with a different unique ID and the same rack ID. Caller: `addrdProcessRequestRackIDMsg`. Established.
- `identFindMaster` (`0x1254f0`, 97 bytes). Returns null for an empty table; otherwise the identity that is lowest under `ffscCompareUniqueIDs` (a later entry replaces the candidate only when its comparison with the candidate is negative). The lowest unique ID is the master. Established.

## Listing modules and racks

- `identEnumerateModules` (`0x125160`, 184 bytes). Arguments: an output array and a count limit. Collects every module number that is not `0xffffffff` from both module fields of every entry, then sorts the array ascending with an exchange sort. Returns the number collected. The collection loop runs while the count does not exceed the limit, so the array can receive one more entry than the limit says. Callers: `SelectAllModules`, `FocusMenu`. Established.
- `identEnumerateRacks` (`0x125220`, 168 bytes). The same for rack IDs other than `0xffffffff`, one per entry, sorted ascending, same limit behaviour. Callers: `MapAllRacks`, `FocusMenu`. Established.

## Updating

- `identUpdate` (`0x125a60`, 200 bytes). Argument: an identity. If an entry with that unique ID exists, refreshes its time (a clock failure logs "Unable to update timestamp in identity table entry" and returns -1) and copies the 28 bytes over it, returning 0. If not, allocates a node (failure logs "Unable to allocate memory for identity table entry", returns -1), stamps it, copies the identity, appends it and returns 1. The return value tells the callers whether the entry is new (1) or an update (0). Established.
- `identSetRackID` (`0x125f50`, 103 bytes). Stores the argument in `ffscLocal`+0xc, writes it to NVRAM record 2 (a failed write logs "WARNING: Rack ID changed but not updated in NVRAM"), sets `routeRebuildResponses` (`0x1f2b58`) to 1 so that the command router rebuilds its per-rack response slots, and when bit 0 of the byte at `0x1e2339` is set logs the new rack ID (printed as "UNASSIGNED" for `0xffffffff`). Always returns 0. Established.
- `identSetIPAddr` (`0x125d80`, 198 bytes). Argument: a pointer to an IP address. Logs "WARNING: MMSC network address changed" if the network number differs from `ffscNetInfo`+4, formats the address, and calls `ifAddrSet("ene0", text)`; on failure logs "Unable to change IP address on ene0 to ..." and returns -1. On success stores the address in `ffscLocal`+8, the network number in `ffscNetInfo`+4 and the host number in `ffscNetInfo`, writes NVRAM record 1 (a failed write logs "WARNING: network information changed but not updated in NVRAM") and returns 0. The interface name `ene0` is the MMSC's Ethernet. Established.
- `identSetCapabilities` (`0x125ca0`, 37 bytes). Stores the argument in `ffscLocal`+0x10 and, when bit 0 of the byte at `0x1e2339` is set, logs "Local capability flags changed to 0x%08x". Returns 0. Established.
- `identSetModuleNums` (`0x125e70`, 141 bytes). Argument: a pointer to two dwords. Copies both to `ffscLocal`+0x14 and +0x18 and, under the same bit, logs "Module numbers changed to:" and each number in hexadecimal (`UNASSIGNED` for `0xffffffff`). Returns 0. Established.

## Scanning for conflicts

- `identScanForConflicts` (`0x1257d0`, 490 bytes). Arguments: an IP-conflict callback and a rack-ID-conflict callback, each taking two identities. First the IP pass, then the rack-ID pass, each logging "Searching for conflicting IP addresses..." or "...rack IDs..." under debug bit 0. IP pass: for each entry in table order, if the host part (`inet_lnaof`) is 15 or less, calls the callback with that entry twice; otherwise compares it with each later entry whose host part exceeds 15 and whose IP dword is equal, and calls the callback with the pair. Rack-ID pass: an entry with rack ID `0xffffffff` is passed twice; any other entry is compared with each later entry with an equal rack ID. A callback result below 0 ends the scan with -1; 0 means the callback changed something, which stops the current pass at once and restarts that pass from the first entry; a result above 0 is skipped. A pass with no change ends it. Logs "Conflict resolution completed." under debug bit 0 and returns 0. Established.

## Printing

- `identPrint` (`0x125b90`, 229 bytes). Argument: an identity. Logs the unique ID as eight colon-separated hexadecimal bytes, the string " MASTER" when the identity is `ffscMaster`, then rack ID, dotted IP address, the two module numbers and the flags as `0x%08x`. Returns 0. Established.
- `identPrintTable` (`0x125710`, 81 bytes). Logs "TIMESTAMP <seconds>.<nanoseconds>" and `identPrint` for every node, then returns the node count from `lstCount`. Caller: not found in the image (reachable only through a table of command handlers, which was not read). Established for the routine.

## Helpers from the `misc` object

- `ffscCompareUniqueIDs` (`0x127f60`, 22 bytes). `bcmp(a, b, 8)`. The `bcmp` of this image (`0x1763c0`) returns 0 for equal bytes and otherwise the difference of the first differing pair as unsigned bytes, so the order is lexicographic from byte 0. Established.
- `ffscCompareTimestamps` (`0x127f10`, 76 bytes). Compares two (seconds, nanoseconds) pairs: seconds as unsigned, then nanoseconds as signed; returns -1, 0 or 1. Established.
- `ffscUniqueIDToString` (`0x128790`, 110 bytes). `"%02x:%02x:%02x:%02x:%02x:%02x:%02x:%02x"` of bytes 0 to 7, into the caller's buffer or a static one. Established.
- `ffscRackIDToString` (`0x1284f0`, 69 bytes). "UNASSIGNED" for `0xffffffff`, otherwise `"%lu"`. Established.
- `ffscModuleNumToString` (`0x128170`, 69 bytes). "UNASSIGNED" for `0xffffffff`, otherwise `"%lx"`. Established.
- `ffscTimestampToString` (`0x128640`, 75 bytes). "<null>" for a null pointer, otherwise `"%ld.%09ld"`. Established.
