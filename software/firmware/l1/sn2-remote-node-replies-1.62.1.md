# What the ARM L1 1.62.1 sends back for the requests a node makes to a remote node's L1

When one Altix node sets up another (the PROM on the first node drives the second node's UART), the second node's L1 sees ordinary
node requests on its link. This note collects, for those requests, the handler and the exact reply bytes, so a stand-in L1 behind a
remote node's UART can answer. The framing and the general table are in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md); the DIMM request in full is
in [sn2-dimm-spd-request.md](sn2-dimm-spd-request.md). The node side (which requests, in what order, with what wait) is in the chips
repository, `software/firmware/snprom/numalink-neighbours.md`, section "Requests to a remote node's L1". Grades follow [AGENTS.md](../../../AGENTS.md).

Sources, disassembly and decompiles only, nothing run: `/stand/sysco/bin/l1` of the Linux L1 image (package 4) of `l1-1.62.1.bin`
(ARM, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`, base `0x8000`, as in
[sn2-dimm-spd-request.md](sn2-dimm-spd-request.md)); and the SAL_B image of SHub 1 PROM 10.54 (SAL 5.04, base `0xffa00000`) for what the node accepts.

## Nothing differs for a remote node

**Inferred.** The L1 sees a frame on its node UART and replies on it; nothing in the handlers below reads which node sent the request except the
slab and subchannel taken from the request's own tag (`SysComGetSlab`, `SysComGetSubc`). The PROM on another node sends the same request
codes with the same arguments as its own node would, so the replies are those of the local case. The one input that changes is the slab
(`0x000e`): the L1 returns the requester's slab from the tag, and what tag a request sent for another node carries was not traced (unknown).

## Handlers and replies

**Established** for the dispatch: an instruction-level run of the compare chain of `L1ReqIRouter` (`0xc8388`, chain `0xc8798` to `0xc8b74`)
for each code gives the handler in the second column. Reply layouts are the PPP payload of `syscom_process_outbound_response` (`0xc3f64`):
byte 0 `0x20 | n`, bytes 1 to 4 the status big-endian, byte 5 the argument count, then arguments (integer `00` plus four bytes big-endian; raw block `0x80 | len`
plus the bytes). `n` is the subchannel index the request carried; the node's receiver wants it equal to the handle it opened, `5 + slice` of the sending
processor (the node end is in the chips note; the L1 end, `SysComGetSubc`, is inferred to return the request's own value, as [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md) says).
Below, `n` stands for that byte; for the boot processor of a node it is 5.

| Request | Handler | Reply payload | What the node does with it | Grade |
|---|---|---|---|---|
| `0x100b` PROM version (three ints) | `0xcb474` | none on success; with an argument count other than four counting the code, `n ff ff ff 9c 00` | waits 3 s, then fails the request (remote node: no park) | established (see [the `0x100b` section](sn2-prom-link-1.62.1.md#the-0x100b-reply)) |
| `0x0003` | `0xc8bb8` | `n 00 00 00 00 02 00 <rack<<6\|bay, 4 bytes> 00 <type character, 4 bytes>` | type letters are lowered; `o` triggers a board-area read; `f` does not | established, in full |
| `0x000e` slab | `0xca214` | `n 00 00 00 00 01 00 <slab, 4 bytes>` | the node reads 255 as failure; for the type letters `l b { f` a slab of 0 or -1 folds the type to `c` | established (the code `0xca214` to `0xca2f4`: the slab comes from the request's tag, or `bladeSSHackSlab` for the types `,` `}` `.` `$` and a brick with more than one slot); the value for a PROM-originated request: unknown |
| `0x0013` slot | `0xca2f8` | for a single-slot brick: `n 00 00 00 00 02 00 00 00 00 00 00 <type character, 4 bytes>` | the node keeps both ints | established for the branch, inferred that type `F` takes it (as the link note) |
| `0x0006` DIMM, device id 8 to 13, 16 to 21 | `0xc9658` | present: 60 bytes, `n 00 00 00 00 02 00 00 00 00 d0 b0 <48 SPD bytes>`; not present: `n ff ff ff 98 00` | the node's parser needs count 2 and int then raw; any error status leaves the slot empty | established, in full ([sn2-dimm-spd-request.md](sn2-dimm-spd-request.md)) |
| `0x0006` device id 14, 15, 22, 23 or any DIMM slot with its presence flag clear | `0xc9658` | `n ff ff ff 98 00` (status -104) | slot empty | established |
| `0x0006` device id 1, selector 2, offset 0, length 64 | `0xc9658` | board information area as in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md) (status 0, bytes left, raw block) or status -104 | any negative status prints a message and goes on | established for the handler; the record contents depend on the EEPROM |
| `0x0010` processor presence | `0xca5f4` | see below | read by the node's board builder | node parser established; L1 builder inferred |
| `0x1005` display text | `0xcaeb4` | no payload beyond status 0 (`n 00 00 00 00 00`); -100 if the count is 1 or less | the node ignores errors | established for the shape |

For the 16 DIMM reads of a node with no DIMM in any slot, every reply is `n ff ff ff 98 00`, whatever the device id. That makes the node's
bank mask zero in all four banks, and the node reports itself memory-less. A node with DIMMs needs the 256-byte records of the
twelve slots of an Opus TIO brick (ids 8, 16, 9, 17, 10, 18, 11, 19, 12, 20, 13, 21) with a valid checksum in byte 63, and -104 for ids 14, 15, 22 and 23.

## The `0x0010` reply as the node reads it

**Established** for the reader, the node's routine `0xffae8250` (SAL_B, 10.54). The payload bytes, counting the header byte as byte 0: bytes 1 to 4 the status
(must be 0; any other status is returned to the caller as it stands); byte 5 the count, **not tested**; byte 6 `00` and bytes 7 to 10 the number of nodes (at most 4);
byte 11 `00` and bytes 12 to 15 the number of CPUs (at most 16); then for each node i from 0: byte `16 + 10 i` `00`, bytes `17 + 10 i` to `20 + 10 i` the
presence mask; byte `21 + 10 i` `00`, bytes `22 + 10 i` to `25 + 10 i` the enable mask. The node then takes `presence[slab] & enable[slab]`.
A node count above 4 or a CPU count above 16 returns -100 to the caller. The L1 builds the reply from `cpuCount`, `cpuPresent` and `cpuEnabled` through the
integer writer `0x9df94` after `ARG__START` `0x9df34` (the calls in `0xca5f4` to `0xcab6c`; which of the two forms in that range an `F` brick takes was not
traced): **inferred**.

## Not settled

- The slab the L1 returns for a request issued by one node for another (above).
- The multi-slot and single-slot forms of `0x0010` for an `F` brick, and the order of its two counts.
- Whether the node's own L1 would ever receive these requests from a remote PROM on real hardware: the node UART of a remote node is reached
  through the SHub junk bus from the other node, which the chips note reads from the PROM side only (inferred).
