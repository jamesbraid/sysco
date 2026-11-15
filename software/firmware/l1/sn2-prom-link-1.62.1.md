# The node-to-L1 link of an Altix node in L1 1.62.1: framing, requests, and what the node firmware asks at boot

An Altix node's firmware (the PROM, which also answers the kernel's console and controller SAL calls, [sn2-controller-host-protocol.md](sn2-controller-host-protocol.md)) talks to the L1 over a serial channel that the node chip provides. This note records that link from the L1 side, so that a machine model can answer the node authentically: the byte framing, the frame header, the argument encoding, which requests the L1 accepts, and which of them the node firmware sends. Grades follow [AGENTS.md](../../../AGENTS.md).

Sources, all disassembled (no code was run):

- **L1 side.** The `Linux L1 image` package of L1 1.62.1 (package 4 of the container in [sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)), its `/stand/sysco/bin/l1`: ARM, 1,999,676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`. The program exports its routine names. For comparison the same program of L1 1.44.0 (`Linux L1 image`, SHA-256 `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`).
- **Node side, cross-check only.** The SAL runtime component (FIT type `0x12`, gzip, 918,824 bytes expanded, IA-64) of the SHub 1 firmware `shub1snprom.bin` of snprom 10.54 (SAL 5.04). Only the request-building call sites and the framing routine were read; the node firmware as a whole belongs to the chips repository's SN2 firmware page, which this note does not change. Addresses below with no package named are in that component.

The addresses of the L1 routines are in the Linux L1 `l1` program's load image (`0x8000` base).

## The channel

**Established** (`SysComConfig`, `config` at `0xbf570`, `init` at `0xbfdfc`, `uartOpen`). The L1 keeps a table of "SysCom" channels, one per serial or scan-chain device. A brick type registers its channels with a small table (`<brick>_SysComCfg`, 24-byte entries; for example `IP63_SysComCfg` is `0x700, 1, 2, 0x4fff, 8, 4`, `Daytona_SysComCfg` two entries `0x400, 1, 2, 3, 1, 2` and `0x401, 1, 2, 7, 1, 2`). The second word of an entry selects the mode: **1 or 3: framed (PPP) channel**, which allocates a PPP codec (`PPPInit` with a 256-byte buffer); **0: raw channel**, whose input goes to `syscom_process_inbound_data` and is forwarded as console characters (below). The first word is stored as the channel's device number; a UART device is opened as `SCC-<n>` (`uartOpen`). The meaning of the other words was not read.

**Established** for the Opus TIO and NL4R router families: the node chip's UART is reached through the scan chain, not a serial wire. `TIOSysComInt` and `NL4R_SysComInt` call `SysComJTagReadReady`, which calls `JTAG_UART_read`; `JTAG_UART_write` and `JTAG_UART_read` access the chip's test data registers named `WR_SYSCOM` and `RD_SYSCOM` (`JChip_TDR_Raw`, `jtag_comm.c`), packing the bytes with the bit-reversal masks `0x55555555`, `0x33333333` and `0x0f0f0f0f`. The rest of the bytes pass through the same PPP codec as a wire. For the node families whose entries carry no `SysComCfg` table (Opus TIO, Dorado, Gemini, IP67 and IP69, the families with an IP57 configuration) which channel mode each uses was not read (unknown); the ColdFire `Legacy 2MB` and `2MB` packages use the processor's own UART for a SHUB node ([sn2-brick-packages.md](sn2-brick-packages.md)).

On the node side (SHub 1 build), `sc_shub_dev_reg` forms the register address `0x8000000ffed40000 | nasid << 38`, plus the register index shifted left by 6; `sc_shub_uart_divisor` returns 1; an `sc_tio_*` set drives the TIO's UART and an `sc_nl4r_*` set the router's. The register layout belongs to the SHub reading, not to this note. The L1 and node ends were read independently and agree on the framing below.

## Framing: HDLC-style byte stuffing with the PPP FCS-16

**Established** at both ends (`PPPWrite` at `0x10df80`, `PPPRead`, `PPPDecode`; node side `l1sc_fill_sendbuf` at `0x52fe0`, `l1sc_receive` at `0x53b80`).

1. A frame starts and ends with the flag byte `0x7e`.
2. Between the flags go the payload bytes, then the 16-bit frame check sequence. Any payload or check byte equal to `0x7e` or `0x7d` is sent as `0x7d` followed by the byte xor `0x20`. The L1's writer also escapes `0x11` and `0x13` (XON, XOFF) when its flow-control option is on, and `0x0d` when a second option is on (both options are fields of the channel; their defaults were not read). The receiver undoes `0x7d` escapes.
3. The check is the PPP FCS-16: initial value `0xffff`, reflected polynomial `0x8408` (the 256-entry table at `0x198134` in the Linux L1 program equals the generated table, entries `0x0000, 0x1189, 0x2312, 0x329b, ...`), the final value complemented and sent low byte first. The reader tests the residue `0xf0b8` (`PPPRead`, literal `0xf0b8`). There is no address and control prefix (`0xff 0x03`) and no protocol field: the payload begins directly with the frame header below.
4. A payload is at most 127 bytes: the node firmware's frame writer clamps the length to 127 (`l1sc_fill_sendbuf`), which agrees with the 127-byte `CHUNKSIZE` of the kernel's subchannel driver. Larger messages are sent in chunks (request codes `0x3001` to `0x3003`, below).

## Frame header and argument encoding

**Established** from the L1 receive path (`syscom_process_inbound_packet` at `0xc2c58`, `syscom_process_inbound_request` at `0xc2284`) and the node's request builder (`sysctl_cmd_construct_req` at `0x4f8a0`):

| Byte | Content |
|---|---|
| 0 | bits 7:5 the **class**, bits 4:0 an index. Class 0: request; class 2: event; any other class is not processed by the packet entry. The L1 writes a response with `0x20` in this byte (class 1). |
| 1 to 4 | a 32-bit big-endian address word. The node firmware builds it from `0x0ffff000` (`sysctl_cmd_open` passes `0xffff000`) and the destination task number; every request it builds is to task 6. The L1 passes the word through `SysComSetTasknum` to derive the 7-bit tag it puts in bits 5 to 11 of the packet's source word (inferred for the L1 use). |
| 5, 6 | the 16-bit big-endian request code |
| 7 | the argument count |
| 8 on | the arguments, in order |

The node's builder writes the same fields without the first byte (word, code, count, arguments) into a buffer; the class and index byte is supplied by the frame writer (`l1sc_fill_sendbuf`'s fourth argument).

An argument starts with a type byte: **`0x00`**, then a 32-bit big-endian integer; **`0x01`**, then a NUL-terminated string; **`0x80 | n`**, then `n` raw bytes (`n` up to 127, "indirect"). This is the encoding the kernel's event parser expects ([sn2-controller-host-protocol.md](sn2-controller-host-protocol.md): types `0x00`, `0x01` and `0x80 | length`), so the event record the kernel sees is the node-to-L1 argument encoding with the code and count in front.

A response carries the status as its first argument, an integer: 0 on success and a negative number on failure; the integer or string results follow. Status values read in `L1ReqIRouter`: `-100` (at entry, when a flag byte of the incoming packet at `+0x20` is zero; its meaning was not read), `-101` (unknown request code, after an optional extended-request hook), `-104` (no serial number, request `0x0008`) and `-106` (request `0x0013` when the blade state check fails). The response header was seen only as stores (class `0x20`, an echo of the request's address word and code, and the count); it is inferred to mirror the request header.

Messages longer than one frame use request codes `0x3001` (start), `0x3002` (next segment) and `0x3003` (continuation): `syscom_process_inbound_request` sends these three codes to a separate assembler (`0xc4ab0`). Their layout in this L1 was not read; the ColdFire L1 uses the same three codes with a segment count in byte 7 and a data length in bytes 8 and 9 ([chunked-request-frames.md](chunked-request-frames.md)); that the ARM L1 does the same is inferred from the shared codes.

## Where requests go

**Established** (`IRouterRegisterFunction` and `IRouterRegisterQueue` call sites, address word `0xffff0NN`): the L1's request router delivers a request to the task whose number is in the low bits of the destination word. Registered tasks: 1 `EvtIRouter` (events), 2 `SMP` (holds `SMPSystemConsole` and `SMPConsoleSubchannel`), 3 `SMPCmd` (text commands), 5 and 21 `SysCom` (outbound to the node), 6 `L1ReqIRouter` (general requests), 14 `L1CfgIRouter` (configuration), 18 `LpbkIRouter` (loopback), 20 `APwr`, 23 `RouteIRouter`, 24 `BkDr`, plus `FlashInit` and the scan-packet task. These are the same task numbers as the ColdFire L1 (general 6, command 3, loopback 18, management 2, node link 5), so a node firmware that addresses task 6 reaches the same request family on either L1.

## The general request set of the Linux L1

**Established** by running the compare chain of `L1ReqIRouter` (`0xc8388`, 28,516 bytes; the chain at `0xc8798` to `0xc8b74`) for every code from `0x0000` to `0x5fff` (concretely, with an instruction-level interpreter) and listing each code's first handler address; codes not in the table go to `0xcf020`. The role of a handler is read from its call list, and in the first rows from its whole body; "in full" below marks those. Where the ColdFire L1 has a handler for the same code the role matches the name in [request-codes.tsv](request-codes.tsv) unless stated; that is how most rows are graded (inferred).

| Code | Handler | Role in 1.62.1 | Grade |
|---|---|---|---|
| `0x0001` | `0xc8b74` | reply 0, the debug-switch word (non-volatile setting 3) | established, in full |
| `0x0002` | `0xc8bb8` | reply 0, rack and bay as `rack << 6 | bay` (settings 1 and 2) | established, in full |
| `0x0003` | `0xc8bb8` | the same, then the brick type character; a brick of type `=` (IX) reports `#` (PX) when a per-slot flag is zero | established, in full |
| `0x0004` | `0xc8ff8` | brick serial number text (`brickGetSerial`, `brickGetSerialSlab`) | inferred |
| `0x0005` | `0xc90dc` | firmware revision: three integers parsed from the version string (the literal `1.62.1` is in the routine) | established for the source, inferred for the reply layout |
| `0x0006`, `0x0007` | `0xc9658`, `0xc9194` | board EEPROM read and format (`eepromGetData`, `eepromIsPresent`; "EEPROM not present") | inferred |
| `0x0008` | `0xc9ddc` | reply 0, the system serial as text: a per-brick hook (`brickOps` entry at `+0x184`) or setting 13; `-104` when empty | established, in full |
| `0x0009` | `0xc9e94` | reply 0, the partition number (setting 14) | established, in full |
| `0x000a` | `0xc9f1c` | two port-speed values (`L1CfgGetCfg`, settings) | inferred |
| `0x000b` | `0xc9ed8` | reply 0, setting 18 (the partition-database flag in the ColdFire L1) | established, in full |
| `0x000c`, `0x000d` | `0xc8c78`, `0xc8f1c` | attached ports 1 to 4 and 5 and up (`L1CfgGetCfg`) | inferred |
| `0x000e` | `0xca214` | slab number of the requesting node (`bladeSSHackSlab` when the blade needs the workaround, else a stored byte) | inferred |
| `0x0010` | `0xca5f4` | processor presence and enable per node: `cpuCount`, `cpuPresent`, `cpuEnabled`; logs `SL REQ: slab%d %d CPUs, %d nodes` | inferred |
| `0x0011` | `0xcab6c` | scan record (`scancmd_init`, `sn1sc_process_scancmd`) | inferred |
| `0x0012` | `0xccbdc` | reply 0 and integers from settings `0x7e`, `0x50` and `0x80` (the node's "maximum nodes" query uses this code) | inferred |
| `0x0013` | `0xca2f8` | the requesting blade's slot (`bladeSSHackSlot` or the stored slot) and its state (`bladeCheckState`, `bladeOps`); logs `L1REQ: s=%d id=0x%04x`; `-106` when the state check fails. The ColdFire L1 names this code the type character | inferred |
| `0x0015` | `0xccc80` | reply 0, setting `0xcc` masked with `0x7f` (the node's "sharing mode" query uses this code) | inferred |
| `0x0016`, `0x0017` | `0xcccc4`, `0xcceac` | new in 1.62.1: log `APPWT_VAL` and `APPWT_LIMIT` for a slot, settings and `pow`; role unread | located |
| `0x1001`, `0x1008` | `0xcae2c`, `0xcb00c` | write non-volatile settings (debug switches, the partition-database flag) | inferred |
| `0x1004`, `0x1005` | `0xcaeb4` | text for the brick display (`displayString`); the node's `sysctl_display_text` sends `0x1005` | established for the call, inferred for the line |
| `0x1006` | `0xcaf70` | set the partition (`nvramSet`, `L1CfgEventPartition`) | inferred |
| `0x1007` | `0xcb094` | set the event subchannel: `SysComSetTasknum`, setting `0x13` written; logs `subc %d` | established for the calls, inferred for the layout |
| `0x1009` | `0xcb1ec` | front-panel LEDs on, off, blink | inferred |
| `0x100a` | `0xcb3c8` | clear the event subchannel (`SysComEventDisable`) | inferred |
| `0x100b` | `0xcb474` | the node reports its PROM version: logs `%s%d: PROM %x.%x (PR %d)`, stores it (`vramSlotSet`), then calls `SysComEventDisable` for the slab | established for the log text and calls |
| `0x100c` | `0xcb6ec` | per-processor setting (`cpuSetState`) | inferred |
| `0x2001` | `0xcb7e4` | system reset (the handler has only the reply store; the action goes through a hook) | inferred |
| `0x2002` to `0x2004` | `0xcb83c`, `0xcb964`, `0xcba8c` | PCI operations through the brick hook | inferred |
| `0x2005`, `0x2006` | `0xcbbb4`, `0xcbc0c` | power down; power cycle (`powerCycle`) | inferred |
| `0x2009`, `0x200a` | `0xcd0d4`, `0xcd2cc` | image sync clear and read | inferred |
| `0x200b`, `0x200c` | `0xcd42c`, `0xcd5f4` | I/O expander read and write | inferred |
| `0x200d`, `0x200e` | `0xcdc64`, `0xce3d0` | set and get the clock (a `date -u ...; hwclock -u` line; year to second as integers) | established for the call text, inferred for the layout |
| `0x200f` | `0xceb50` | TIO clock reset and cross-bar probe (`TIO_Issue_CX_Probe`; "No added CX checks, unknown brick type") | inferred |
| `0x3001` to `0x3004` | `0xcd90c`, `0xcdb78`, `0xcdbb4`, `0xcdbe0` | hook and FRU requests (`0x3001` calls `sign`) | inferred |
| `0x4000`, `0x4004` | `0xcbc58`, `0xcc1e0` | inventory text: IPMI FRU records, JEDEC DIMM records | inferred |
| `0x4001` | `0xcdc20` | reply 0 and the SHUB workaround flag (`SHUB_WarStatus`) | inferred |
| `0x4002` | `0xcc7b0` | controller reboot: logs `L1REQ_REBOOT requested`, signals the program, `exceptionReset` | established |
| `0x4003` | `0xcc7dc` | load a PROM image into the local bus interface (`LBILoadPROM`, `LBIMemPattern`) | inferred |
| `0x4005` | `0xcc700` | port summary (`L1CfgGetPortSummary`) | inferred |
| `0x5000` | `0xce4c8` | new in 1.62.1: clock set with a sequence search ("Starting sequence search", `date -u`) | located |

The ColdFire L1's console requests `0x1002` (console subchannel) and `0x1003` (console volunteer), its last-event request `0x000f`, and the expander pair `0x2007` and `0x2008` have no handler in the Linux L1's table: the console choice is made from class-2 frames instead (below). 1.44.0's Linux L1 has the same table without `0x0016`, `0x0017` and `0x5000` (the same chain, same method; 52 codes against 55).

## What the node firmware sends

**Established** (the call sites of `sysctl_cmd_construct_req` in the SAL runtime): every request below is addressed to task 6 and built as (code, argument count, arguments). The node firmware caches the answers per node (`sysctl_cache_check`, `sysctl_cache_store`) for the identity queries, so each is asked once per node.

| Node routine | Code | Arguments | What the kernel or PROM uses it for |
|---|---|---|---|
| `sysctl_debug_get` | `0x0001` | 0 | debug switches |
| `sysctl_rack_bay_type_get` (cached) | `0x0003` | 0 | rack, bay and type: the module id (`SN_SAL_SYSCTL_MODID_GET`), the I/O brick id (`sysctl_iobrick_rack_bay_type_get`) |
| `sysctl_sys_serial_get` | `0x0008` | 0 | the system serial number |
| `sysctl_partition_get` | `0x0009` | 0 | `SN_SAL_SYSCTL_PARTITION_GET` |
| `sysctl_io_portspeed_get` | `0x000a` | 0 | `SN_SAL_SYSCTL_IO_PORTSPEED_GET` |
| `sysctl_common_slab_get` (cached) | `0x000e` | 0 | the slab of this node: `SN_SAL_SYSCTL_SLAB_GET` |
| `sysctl_cpu_presence` | `0x0010` | 0 | which processors are present |
| `sysctl_max_nodes_get` | `0x0012` | 0 | the maximum node count |
| `sysctl_slot_get` (cached) | `0x0013` | 0 | the slot of this node (part of the module geography) |
| `sysctl_shrmode_get` | `0x0015` | 0 | the sharing mode |
| `sysctl_display_text`, `sysctl_display_text_by_geoid` | `0x1005` | 2 | text on the brick display |
| `sysctl_event_init` | `0x1007` | 2 | event delivery: a subchannel is opened first (`sysctl_subch_open`), then this request names it (`SN_SAL_SYSCTL_EVENT`) |
| `sysctl_event_unreg` | `0x100a` | 0 | stop events |
| `sysctl_time_of_day_get`, `sysctl_time_of_day_set` | `0x200e`, `0x200d` | 0, 12 | the clock |
| `sysctl_tio_clock_reset` | `0x200f` | 0 | the TIO clock reset (`SN_SAL_SYSCTL_OP` 2), with a long timeout (20,000,000 ticks) |
| `sysctl_iobrick_pci_op` | `0x2004` | 4 | the I/O brick PCI operation (`SN_SAL_SYSCTL_IOBRICK_PCI_OP`) |
| `sysctl_cmd_text_command` | not captured | one text argument | a text command; the call is built for task 3 (the code was not read) |

The PROM version report `0x100b` is not among the request call sites of the SAL runtime read here; the L1 handles it (row above), so some other stage of the firmware sends it (unknown which). The three-integer layout is the ColdFire handler's (`general_req_prom_version_report`).

The node firmware's own probe of the controller (`sysctl_probe`, 16 instructions in this build) compares `cpuid[2]` with the simulator signature `0xaeeeeeee8badbeef` and returns 0 or 1 without a call; it sends no request. An L1 therefore does not need to answer anything for the probe in this build.

## The first exchanges, as a contract for an emulated L1

What the L1 must do for the node firmware of this build, in dependency order. The order among the identity queries is **unknown**: the node caches them and fetches each when a SAL call or the boot path needs it; the L1 side cannot show the order because it answers whichever arrives.

1. **Channel and framing.** A framed channel to the node: frames as above, flags, escaping and FCS-16. A read error from the codec is logged as `SCC-<n> read error (PPP)`; no acknowledgement or retransmission frame was seen at this layer (inferred: none is defined by the code read).
2. **Board identity** (all to task 6; reply: status 0 then the values):
   - `0x0003`: `rack << 6 | bay` (an integer) and the brick type character (an integer), from settings 1 and 2 and the identity routine's result.
   - `0x0008`: the system serial as a string; status `-104` if none is set.
   - `0x0013`: the slot of the node (and `-106` if the blade is not in a state to answer).
   - `0x000e`: the slab number. `0x0012`: the node maximum. `0x0015`: the sharing mode (setting `0xcc & 0x7f`). `0x0009`: the partition (setting 14).
   - `0x0010`: processor presence and enable, per node.
3. **Power and reset state.** There is no request for it in the table, and none from the node in the list above. The L1 holds the node in reset and releases it itself (the `IP57_ResetHold` and `IP57_ResetRelease` routines, [arm-ip57-configuration-control-1.40.5.md](arm-ip57-configuration-control-1.40.5.md)); the node firmware finds itself running after the release, so a model that starts the node firmware at all has already completed the L1's release sequence. Not asked and not answered: the last-reset reason (`0x000f` is absent from this L1).
4. **Console selection.** There is no console request in this L1. The L1 learns the console from the data: a class-2 frame from the node (`syscom_process_inbound_event`) makes the L1 call `SMPConsoleSubchannel` with the sender's slab and subchannel (which sets the system console and tells the configuration task through `L1CfgEventConsoleSubchannel`) and forwards the frame to the management task as an event (`IRouterEvent`); a raw-mode channel (`syscom_process_inbound_data`) forwards its bytes as one string argument of an event, on subchannel 4. The node firmware's `l1sc_send` passes the immediate `0x42` to the frame writer where the header byte goes, and `l1sc_receive` compares against `0x44`, so its console traffic is class 2 (index 2 out, index 4 in): inferred from the two immediates, the routines were not read in full. The console characters in a class-2 frame are the frame's payload (the L1 copies them as one string argument, `ARG__STRING`). The L1 side of the kernel's `SN_SAL_SYSCTL_EVENT` is `0x1007` above.
5. **Event delivery.** After `0x1007` the L1 persists the subchannel (setting `0x13`) and pushes environmental events to the node as class-2 frames in the argument encoding the kernel parses ([sn2-controller-host-protocol.md](sn2-controller-host-protocol.md)).

An emulated L1 for the Altix 350 therefore needs: the framed channel and the task-6 requests in steps 2 and 5 plus the clock (`0x200e`, `0x200d`) and the display (`0x1005`); it does not need the PROM-load, FRU, inventory or scan requests, and may answer them `-101`.

## Not settled

- The order of the first node-to-L1 requests after the node firmware starts (needs the call chain of the SAL entry up to the first `sysctl_*` use, or a captured session).
- The mode, device number and the remaining words of the Opus TIO, Dorado, Gemini, IP67 and IP69 channel entries, and whether those nodes use the JTAG-scanned UART or a wire.
- The response header bytes, the chunk-frame layout of this L1, the console frame class and index bytes, and the `0x100b` sender.
- The reply bodies of the rows graded inferred above: read the handler bodies in `L1ReqIRouter` at the addresses listed.

## Review

Independent review of commit `854d395` (this page and the "SN2 controller link" section of [../../../questions/README.md](../../../questions/README.md)), with the inventory row of `ef912ee`. The reviewer wrote none of it. The L1 end was re-read with an ARM decoder; the node end with a purpose-written IA-64 bundle decoder (the shared toolchain image has an ARM decoder and no IA-64 one), which decodes only add-immediate, move-long, compare-immediate and branch forms. Call sites and immediates are therefore read exactly; instruction bodies are not.

**Checks that passed.**

- Node image: FIT entry type `0x12` of the 10.54 `shub1snprom.bin` gunzips to 918,824 bytes; `sysctl_cmd_construct_req` `0x4f8a0`, `l1sc_fill_sendbuf` `0x52fe0`, `l1sc_receive` `0x53b80`, `sysctl_probe` `0x4e770` and the other quoted addresses match the exported symbols.
- Framing, L1 end: `PPPWrite` and `PPPRead` use flag `0x7e`, escape `0x7d` with xor `0x20`, XON and XOFF (`0x11`, `0x13`) and `0x0d` escapes, initial `0xffff` and the `0xf0b8` residue literal; the 256-entry table at `0x198134` equals the generated reflected-`0x8408` table; there is no `0xff 0x03` handling. Node end: `crc16_tab` (`0xa8d60`, 512 bytes) equals the same table, the `0xf0b8` constant is in `l1sc_receive`, `l1sc_fill_sendbuf` clamps at 127 and tests `0x7e` and `0x7d`, the table is reached through the global pointer (`DT_PLTGOT` `0x203000`), and no `0xff 0x03` appears. Not read at the node end: the final complement, the byte order of the check and the xor of escaped bytes (the bodies use register forms my decoder does not print).
- Header and arguments: `syscom_process_inbound_packet` takes the class from `byte0 >> 5` (0 request, 2 event); `syscom_process_inbound_request` builds the address word from bytes 1 to 4 and the code from bytes 5 and 6, and sends `0x3001` to `0x3003` to the assembler. The argument-type encoding and the response layout were not re-read.
- Request set: re-ran the compare chain of `L1ReqIRouter` with an instruction interpreter for every code from `0x0000` to `0x5fff`. 55 codes in 1.62.1 at the handler addresses of the table, 52 in 1.44.0, the difference exactly `0x0016`, `0x0017` and `0x5000`. `0x1002`, `0x1003`, `0x000f`, `0x2007` and `0x2008` have no handler. Status literals `-100`, `-101`, `-104` and `-106` are present.
- Node requests: all 20 call sites of `sysctl_cmd_construct_req` decoded as (task, code, argument count). Every code in the page's table is confirmed with the stated argument counts, and every one of those is addressed to task 6, including `0x0010` (sent from a local routine that `sysctl_cpu_presence` calls) and `0x2004`. No call site builds `0x1002`, `0x1003`, `0x100b`, `0x2001`, `0x2005` or `0x2006`. The frame writer's `0x42` and the receiver's `0x44` are as stated. The `sysctl_probe` signature constant, the 20,000,000 timeout and the cached-identity pattern are as stated.
- The strings "Reset request sent to system controller" and "Request for system reset failed" are entries of the exported table `chipset_common_ledlist` (display-code descriptions), not requests.

**Findings.**

1. Omission: the node firmware also sends request `0x0006` (8 arguments, task 6) from the shared routine behind `sysctl_eeprom_node`, `sysctl_eeprom_io_riser`, `sysctl_eeprom_dimm` and `sysctl_eeprom_iobrick`. It is not in the table, although the table is introduced as every request built. The L1 handler is the one the page grades as board EEPROM read.
2. The text command's code can be read: the builder is called with task 3, code `0x0000`, two arguments. The command strings in the SAL runtime are `version`, `p %d fru capture`, `* fru capture`, `p %d display 2 %s` and `* display 2 %s`. The adjacent strings `p %d fru reset` and `* fru reset` have no reference my decoder finds. So the absence of power and reset requests is confirmed for the numbered requests only; it is not established for the text-command path, and a FRU reset text command exists as a string.
3. `-104` is also returned by the handlers for other codes (for example `0x0004`), not only `0x0008`; the sentence reads as if it were specific to `0x0008`.
4. The page says the node builds every request for task 6, then lists the text command for task 3; the opening sentence of the table needs the exception.
5. Codes `0x3001` to `0x3003` are sent to the chunk assembler before the compare chain, yet the chain has handlers for them (graded inferred, "hook and FRU requests"). The page does not say how a request reaches those handlers.
6. Grades read: "established, in full" rows are consistent with the chain addresses; rows graded inferred were not checked further. The first-exchange contract (order unknown) is correctly left open. The questions section matches the page.

**Cleared scope.** The framing facts at the L1 end, and at the node end as limited above; the header layout and the class byte; the 55-code request set and its three additions; the node-side request codes and argument counts in the table (not its completeness: see findings 1 and 2); the absence of console, PROM-version and numbered power or reset requests from the SAL runtime read; the grades of the rows checked. Not cleared: the claim that no power or reset request reaches the L1 by any path (text commands), the completeness of the node request list, and the rows graded inferred.
