# The node-to-L1 link of an Altix node in L1 1.62.1: framing, requests, and what the node firmware asks at boot

An Altix node's firmware (the PROM, which also answers the kernel's console and controller SAL calls, [sn2-controller-host-protocol.md](sn2-controller-host-protocol.md)) talks to the L1 over a serial channel that the node chip provides. This note records that link from the L1 side, so that a machine model can answer the node authentically: the byte framing, the frame header, the argument encoding, which requests the L1 accepts, and which of them the node firmware sends. Grades follow [AGENTS.md](../../../AGENTS.md).

Sources, all disassembled (no code was run):

- **L1 side.** The `Linux L1 image` package of L1 1.62.1 (package 4 of the container in [sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)), its `/stand/sysco/bin/l1`: ARM, 1,999,676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`. The program exports its routine names. For comparison the same program of L1 1.44.0 (`Linux L1 image`, SHA-256 `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`).
- **Node side, cross-check only.** The SAL runtime component (FIT type `0x12`, gzip, 918,824 bytes expanded, IA-64) of the SHub 1 firmware `shub1snprom.bin` of snprom 10.54 (SAL 5.04). Only the request-building call sites and the framing routine were read; the node firmware as a whole belongs to the chips repository's SN2 firmware page, which this note does not change. Addresses below with no package named are in that component.

The addresses of the L1 routines are in the Linux L1 `l1` program's load image (`0x8000` base).

## The channel

**Established** (`SysComConfig`, `config` at `0xbf570`, `init` at `0xbfdfc`, `uartOpen`). The L1 keeps a table of "SysCom" channels, one per serial or scan-chain device. A brick type registers its channels with a small table (`<brick>_SysComCfg`, 24-byte entries; for example `IP63_SysComCfg` is `0x700, 1, 2, 0x4fff, 8, 4`, `Daytona_SysComCfg` two entries `0x400, 1, 2, 3, 1, 2` and `0x401, 1, 2, 7, 1, 2`). The second word of an entry selects the mode: **1 or 3: framed (PPP) channel**, which allocates a PPP codec (`PPPInit` with a 256-byte buffer); **0: raw channel**, whose input goes to `syscom_process_inbound_data` and is forwarded as console characters (below). The first word is stored as the channel's device number; a UART device is opened as `SCC-<n>` (`uartOpen`). The meaning of the other words was not read.

**Established** for the Opus TIO and NL4R router families: the node chip's UART is reached through the scan chain, not a serial wire. `TIOSysComInt` and `NL4R_SysComInt` call `SysComJTagReadReady`, which calls `JTAG_UART_read`; `JTAG_UART_write` and `JTAG_UART_read` access the chip's test data registers named `WR_SYSCOM` and `RD_SYSCOM` (`JChip_TDR_Raw`, `jtag_comm.c`), packing the bytes with the bit-reversal masks `0x55555555`, `0x33333333` and `0x0f0f0f0f`. The rest of the bytes pass through the same PPP codec as a wire. For the node families whose entries carry no `SysComCfg` table (Opus TIO, Dorado, Gemini, IP67 and IP69, the families with an IP57 configuration) which channel mode each uses was not read (unknown); the ColdFire `Legacy 2MB` and `2MB` packages use the processor's own UART for a SHUB node ([sn2-brick-packages.md](sn2-brick-packages.md)).

On the node side (SHub 1 build), the UART driver (`sc_uart_*`, for example `sc_uart_rxintr` and `sc_uart_txintr`) addresses the UART at the literal `0x8000000ffed40000 | nasid << 38` plus eight bytes per register, the same eight-byte stride as the PROM's own UART accesses (established: the immediates `+0x00` to `+0x38`). The chipset table also holds two accessors for the same window: `sc_shub_dev_reg` forms `0x8000000ffed40000 | nasid << 38` plus the register index shifted left by 6 (a 64-byte spacing), and `sc_shub_dev_reg_offset` forms `0x7fed40000` plus the index shifted left by 3. No call from the `sc_uart_*` routines to either was found; the callers of the table slots that were found are in the scan-chain, TIO and router code. So 8 is the register spacing of the UART as the UART driver uses it; what the `<< 6` accessor serves is not settled (unknown). `sc_shub_uart_divisor` returns 1; an `sc_tio_*` set drives the TIO's UART and an `sc_nl4r_*` set the router's. The register layout belongs to the SHub reading, not to this note. The L1 and node ends were read independently and agree on the framing below.

## Framing: HDLC-style byte stuffing with the PPP FCS-16

**Established** at both ends (`PPPWrite` at `0x10df80`, `PPPRead`, `PPPDecode`; node side `l1sc_fill_sendbuf` at `0x52fe0`, `l1sc_receive` at `0x53b80`).

1. A frame starts and ends with the flag byte `0x7e`.
2. Between the flags go the payload bytes, then the 16-bit frame check sequence. Any payload or check byte equal to `0x7e` or `0x7d` is sent as `0x7d` followed by the byte xor `0x20`. The L1's writer also escapes `0x11` and `0x13` (XON, XOFF) when its flow-control option is on, and `0x0d` when a second option is on (both options are fields of the channel; the SysCom channels are created with both off, see [node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md)). The receiver undoes `0x7d` escapes.
3. The check is the PPP FCS-16: initial value `0xffff`, reflected polynomial `0x8408` (the 256-entry table at `0x198134` in the Linux L1 program equals the generated table, entries `0x0000, 0x1189, 0x2312, 0x329b, ...`), the final value complemented and sent low byte first. The reader tests the residue `0xf0b8` (`PPPRead`, literal `0xf0b8`). There is no address and control prefix (`0xff 0x03`) and no protocol field: the payload begins directly with the frame header below.
4. A payload is at most 127 bytes: the node firmware's frame writer clamps the length to 127 (`l1sc_fill_sendbuf`), which agrees with the 127-byte `CHUNKSIZE` of the kernel's subchannel driver. Larger messages are sent in chunks (request codes `0x3001` to `0x3003`, below).

## Frame header and argument encoding

**Established** from the L1 receive path (`syscom_process_inbound_packet` at `0xc2c58`, `syscom_process_inbound_request` at `0xc2284`) and the node's request builder (`sysctl_cmd_construct_req` at `0x4f8a0`):

| Byte | Content |
|---|---|
| 0 | bits 7:5 the **class**, bits 4:0 an index. Class 0: request; class 2: event; any other class is not processed by the packet entry. The L1 writes a response with `0x20` in this byte (class 1). |
| 1 to 4 | a 32-bit big-endian address word. The node firmware builds it from `0x0ffff000` (`sysctl_cmd_open` passes `0xffff000`) and the destination task number; every numbered request it builds is to task 6, and its text command goes to task 3 (see the request table below). The L1 passes the word through `SysComSetTasknum` to derive the 7-bit tag it puts in bits 5 to 11 of the packet's source word (inferred for the L1 use). |
| 5, 6 | the 16-bit big-endian request code |
| 7 | the argument count |
| 8 on | the arguments, in order |

The node's builder writes the same fields without the first byte (word, code, count, arguments) into a buffer; the class and index byte is supplied by the frame writer (`l1sc_fill_sendbuf`'s fourth argument).

An argument starts with a type byte: **`0x00`**, then a 32-bit big-endian integer; **`0x01`**, then a NUL-terminated string; **`0x80 | n`**, then `n` raw bytes (`n` up to 127, "indirect"). This is the encoding the kernel's event parser expects ([sn2-controller-host-protocol.md](sn2-controller-host-protocol.md): types `0x00`, `0x01` and `0x80 | length`), so the event record the kernel sees is the node-to-L1 argument encoding with the code and count in front.

A response carries the status first (bytes 1 to 4 of the frame, see below): 0 on success and a negative number on failure; the integer, raw-byte or string results follow. Status values read in `L1ReqIRouter`: `-100` (at entry, when the packet's argument count at `+0x20` is zero; the response routine iterates over the same field), `-101` (unknown request code, after an optional extended-request hook), `-104` (no serial number; request `0x0008`, and other handlers too, for example `0x0004`) and `-106` (request `0x0013` when the blade state check fails). The response header is described under [What the ARM L1 sends back](#what-the-arm-l1-sends-back-for-the-nodes-early-requests-l1-1621-opus-tio-and-ip57-personality): byte 0 is `0x20` plus the subchannel and bytes 1 to 4 are the status, not an echo of the request.

Messages longer than one frame use request codes `0x3001` (start), `0x3002` (next segment) and `0x3003` (continuation): `syscom_process_inbound_request` sends these three codes to a separate assembler (`0xc4ab0`). Their layout in this L1 was not read; the ColdFire L1 uses the same three codes with a segment count in byte 7 and a data length in bytes 8 and 9 ([chunked-request-frames.md](chunked-request-frames.md)); that the ARM L1 does the same is inferred from the shared codes.

## Where requests go

**Established** (`IRouterRegisterFunction` and `IRouterRegisterQueue` call sites, address word `0xffff0NN`): the L1's request router delivers a request to the task whose number is in the low bits of the destination word. Registered tasks: 1 `EvtIRouter` (events), 2 `SMP` (holds `SMPSystemConsole` and `SMPConsoleSubchannel`), 3 `SMPCmd` (text commands), 5 and 21 `SysCom` (outbound to the node), 6 `L1ReqIRouter` (general requests), 14 `L1CfgIRouter` (configuration), 18 `LpbkIRouter` (loopback), 20 `APwr`, 23 `RouteIRouter`, 24 `BkDr`, plus `FlashInit` and the scan-packet task. These are the same task numbers as the ColdFire L1 (general 6, command 3, loopback 18, management 2, node link 5), so a node firmware that addresses task 6 reaches the same request family on either L1.

## The general request set of the Linux L1

**Established** by running the compare chain of `L1ReqIRouter` (`0xc8388`, 28,516 bytes; the chain at `0xc8798` to `0xc8b74`) for every code from `0x0000` to `0x5fff` (concretely, with an instruction-level interpreter) and listing each code's first handler address; codes not in the table go to `0xcf020`. The role of a handler is read from its call list, and in the first rows from its whole body; "in full" below marks those. Where the ColdFire L1 has a handler for the same code the role matches the name in [request-codes.tsv](request-codes.tsv) unless stated; that is how most rows are graded (inferred).

| Code | Handler | Role in 1.62.1 | Grade |
|---|---|---|---|
| `0x0001` | `0xc8b74` | reply 0, the debug-switch word (non-volatile setting 3; 0 on a blank image, [settings note](sn2-l1-settings-defaults-1.62.1.md)) | established, in full |
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
| `0x3001` to `0x3004` | `0xcd90c`, `0xcdb78`, `0xcdbb4`, `0xcdbe0` | hook and FRU requests (`0x3001` calls `sign`); how the chain reaches them while the chunk assembler takes `0x3001` to `0x3003` first is open (see Not settled) | inferred |
| `0x4000`, `0x4004` | `0xcbc58`, `0xcc1e0` | inventory text: IPMI FRU records, JEDEC DIMM records | inferred |
| `0x4001` | `0xcdc20` | reply 0 and the SHUB workaround flag (`SHUB_WarStatus`) | inferred |
| `0x4002` | `0xcc7b0` | controller reboot: logs `L1REQ_REBOOT requested`, signals the program, `exceptionReset` | established |
| `0x4003` | `0xcc7dc` | load a PROM image into the local bus interface (`LBILoadPROM`, `LBIMemPattern`) | inferred |
| `0x4005` | `0xcc700` | port summary (`L1CfgGetPortSummary`) | inferred |
| `0x5000` | `0xce4c8` | new in 1.62.1: clock set with a sequence search ("Starting sequence search", `date -u`) | located |

The ColdFire L1's console requests `0x1002` (console subchannel) and `0x1003` (console volunteer), its last-event request `0x000f`, and the expander pair `0x2007` and `0x2008` have no handler in the Linux L1's table: the console choice is made from class-2 frames instead (below). 1.44.0's Linux L1 has the same table without `0x0016`, `0x0017` and `0x5000` (the same chain, same method; 52 codes against 55).

## What the node firmware sends

**Established** (the call sites of `sysctl_cmd_construct_req` in the SAL runtime): every numbered request below is addressed to task 6 and built as (code, argument count, arguments). The one exception is the text command, the last row, which goes to task 3. The "Arguments" column below counts words in the node's builder (an integer or string is 2, a raw block 3), not arguments: the SN1 PROM's builder does the same ([node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md)), and 8 for `0x0006` is the four integers the L1 handler reads (inferred for the SN2 builder). The node firmware caches the answers per node (`sysctl_cache_check`, `sysctl_cache_store`) for the identity queries, so each is asked once per node.

| Node routine | Code | Arguments | What the kernel or PROM uses it for |
|---|---|---|---|
| `sysctl_debug_get` | `0x0001` | 0 | debug switches |
| `sysctl_rack_bay_type_get` (cached) | `0x0003` | 0 | rack, bay and type: the module id (`SN_SAL_SYSCTL_MODID_GET`), the I/O brick id (`sysctl_iobrick_rack_bay_type_get`) |
| shared routine behind `sysctl_eeprom_node`, `sysctl_eeprom_io_riser`, `sysctl_eeprom_dimm` and `sysctl_eeprom_iobrick` | `0x0006` | 8 | board EEPROM reads for the node, the I/O riser, a DIMM and the I/O brick |
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
| `sysctl_cmd_text_command` | task 3, code `0x0000` | 2 | text commands: `version`; `p %d fru capture` and `* fru capture`; `p %d display 2 %s` and `* display 2 %s` (per-node and all-node forms). The strings `p %d fru reset` and `* fru reset` exist with no reference found, so a FRU reset text command is not ruled out |

The PROM version report `0x100b` is sent from `sysctl_init` (SAL_B `0xffa90f5c` and `0xffa9115c`, in the flash SAL_B rather than the runtime component read first): three ints, the first constant 1 and two bytes of the PROM's version data. The ARM L1 checks the count and sends no reply.

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
3. **Power and reset state.** There is no numbered request for it in the table, and none from the node in the list above. The text-command path is not closed (its `fru reset` strings have no reference found), so "no power or reset request" holds for numbered requests only. The strings "Reset request sent to system controller" and "Request for system reset failed" are LED display-code descriptions in the exported table `chipset_common_ledlist`, not requests. The L1 holds the node in reset and releases it itself (the `IP57_ResetHold` and `IP57_ResetRelease` routines, [arm-ip57-configuration-control-1.40.5.md](arm-ip57-configuration-control-1.40.5.md)); the node firmware finds itself running after the release, so a model that starts the node firmware at all has already completed the L1's release sequence. Not asked and not answered: the last-reset reason (`0x000f` is absent from this L1).
4. **Console selection.** There is no console request in this L1. The L1 learns the console from the data: a class-2 frame from the node (`syscom_process_inbound_event`) makes the L1 call `SMPConsoleSubchannel` with the sender's slab and subchannel (which sets the system console and tells the configuration task through `L1CfgEventConsoleSubchannel`) and forwards the frame to the management task as an event (`IRouterEvent`); a raw-mode channel (`syscom_process_inbound_data`) forwards its bytes as one string argument of an event, on subchannel 4. The node firmware's `l1sc_send` passes the immediate `0x42` to the frame writer where the header byte goes, and `l1sc_receive` compares against `0x44`, so its console traffic is class 2 (index 2 out, index 4 in): inferred from the two immediates, the routines were not read in full. The console characters in a class-2 frame are the frame's payload (the L1 copies them as one string argument, `ARG__STRING`). The L1 side of the kernel's `SN_SAL_SYSCTL_EVENT` is `0x1007` above.
5. **Event delivery.** After `0x1007` the L1 persists the subchannel (setting `0x13`) and pushes environmental events to the node as class-2 frames in the argument encoding the kernel parses ([sn2-controller-host-protocol.md](sn2-controller-host-protocol.md)).

An emulated L1 for the Altix 350 therefore needs: the framed channel and the task-6 requests in steps 2 and 5 plus the clock (`0x200e`, `0x200d`) and the display (`0x1005`); it does not need the PROM-load, FRU, inventory or scan requests, and may answer them `-101`.

## What the ARM L1 sends back for the node's early requests (L1 1.62.1, Opus TIO and IP57 personality)

Read from the Linux L1 program (`/stand/sysco/bin/l1`, the addresses above; the program was carved from package 4 with this repository's CramFS reader and matches the SHA-256 in the first section). Grades: established where the whole routine named was read.

**The reply frame on the wire (established, `syscom_process_outbound_response` `0xc3f64`, read in full; this replaces the earlier inference that the reply echoes the request's address word and code).** The PPP payload of a reply is:

| Byte | Content |
|---|---|
| 0 | `0x20 | subchannel`: class 1 in bits 7:5, the subchannel index in bits 4:0. The index is `SysComGetSubc` of the tag the request left in the packet (`SysComSetTasknum`, `0xc132c`, called from `syscom_process_inbound_request` with the request's own byte 0 bits 4:0); the pair is inferred to return the request's subchannel index unchanged (the table behind them was not read) |
| 1 to 4 | the status, big-endian 32-bit: argument 0 of the reply |
| 5 | the argument count, counting the arguments after the status |
| 6 on | the arguments: integer `0x00` + 4 bytes big-endian; raw bytes `0x80 | n` + n bytes; string `0x01` + NUL-terminated text. The running length includes the class byte and an integer is added only while it stays at most 127; a string or raw block is tested without its one-byte tag, so a reply can reach 128 bytes with the class byte ([node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md), maximum payload row). A larger reply sets an overflow flag and the arguments after that point are dropped |

There is no address word and no echoed request code in a reply. A reply whose packet carries a nonzero error code is sent as status `0xffffffff` followed by that code in four bytes, with no argument count (nine bytes). This is exactly the shape the node's reader wants: the node strips byte 0, then reads the status in bytes 0 to 3, the count in byte 4 and the arguments from byte 5 (`sysctl_cmd_interpret_resp`, the chips repository's SN2 boot-sequence page), so the two ends agree on byte positions (L1 end established; node end read in the same routine). In the L1's own request records the request code is argument 0, so the argument count the L1's handlers test is the request's argument count plus one.

A handler that never calls `ARG__START` sends no reply at all (`IRouterResponse` is skipped when the reply's count word is zero, `0xcf198`). The PROM-version report `0x100b` is such a handler on its success path: it checks that the request carries three arguments (count 4 with the code), prints the PROM version, and returns without a reply; with a different count it replies -100. The node sends this request and waits for a reply it does not get; what the node does when none comes is under [The 0x100b reply](#the-0x100b-reply).

**Reply bodies (status 0 in every row; arguments after the status).**

| Request | Reply arguments | Source of the values | Grade |
|---|---|---|---|
| `0x0001` | one int: the debug-switch word | non-volatile setting 3 (`nvramGet(3)`) | established, in full |
| `0x0003` | two ints: `rack << 6 | bay`, then the brick type character | settings 1 and 2; `brickGetType()`; for `=` the character `#` unless a per-slot flag is set | established, in full |
| `0x0013` | for brick types other than `,` `}` `.` `$` with one slot (the Opus TIO letter `F` with a single slot): two ints, 0 then the brick type character | `brickNumSlots`, `brickGetType()` | established for the branch, inferred that `F` has one slot |
| `0x000e` | one int: the requester's slab | `SysComGetSlab` of the request's tag (single-slot brick types skip the blade workaround) | established for the code, inferred for the value (0 for the one node on the channel) |
| `0x0009` | one int: the partition | non-volatile setting 14 | established, in full |
| `0x0015` | one int: the sharing mode | non-volatile setting `0xcc`, masked to 7 bits (mask from the earlier reading) | inferred |
| `0x0010` | two counts, then for each node its processor presence mask and enable mask (single-slot form; the multi-slot form differs) | `cpuCount`, `cpuPresent`, `cpuEnabled`; for IP57 the masks come from the two I/O expanders | inferred; the order of the two counts was not pinned |
| `0x1005` | none | text to the display, `displayString`; -100 if the count is 1 or less | established for the shape |
| `0x100b` | no reply on success | see above | established |
| `0x0006` | one int, then raw bytes | see the next section | established, in full |

**Module type (established for `F`, inferred for the Altix 350).** The type letter comes from `brickGetType`; the program's brick-family switch (`brickInitBrickOps` `0x214fc`, jump table at `0x21528`) maps `F` to `OpusTio_BrickOpsInit` (`0x3cc74`), and that routine installs the `IP57_CpuPresent`, `IP57_CpuEnable` and `IP57_CpuDisable` hooks and the name string `TIO`. Request `0x0003` returns that letter (0x46) as its second int and `0x0013` returns it as its second int; the first int of `0x0003` is `rack << 6 | bay` from settings 1 and 2. That a base Altix 350 compute module is the `F` brick is inferred (the IP57 configuration tables are carried by exactly the Opus TIO, Dorado, Gemini, IP67 and IP69 families, and Opus TIO carries a PCI-X bus and slot configuration); the ColdFire `O` alternative in the earlier note is not served by this ARM program.

### Request `0x0006`, board and DIMM EEPROM read (`L1ReqIRouter` `0xc9658` to `0xc9dd8`, read in full)

The request carries four ints: device id (argument 1), selector (2), byte offset (3) and length (4); it needs more than four arguments including the code or it replies -100. The reply is status, then one int and one raw-byte argument.

1. **Device id to table index (`0xc7664`, read for `F`; indexes the slot's EEPROM table).** Id 0 gives index 0 (INTERFACE), id 1 gives 1 (NODE), ids 4 to 7 give 2 to 5 (CPU 0A, CPU 0C, CPU 0A S, CPU 0C S), ids 8 to 13 give 6 to 11 (DIMM0 to DIMM5 L), ids 16 to 21 give 12 to 17 (DIMM0 to DIMM5 R), id 98 gives 18 (RISER), id 99 gives 19 (IO10). Every other id gives -1 and fails as not present. The names and per-entry type codes (1 FRU record, 3 CPU, 0 CPU sensor, 2 DIMM, 4 power supply) are in `OpusTio_eepromCfg` (`0x16550c`, eleven rows of two 12-byte entries). The power supply entries (20, 21) have no device id. So the node's DIMM ids 8, 16, 9, 17, 10, 18, 11, 19, 12, 20, 13, 21 are present on an Opus TIO brick and ids 14, 22, 15, 23 (the fourth bank) are not: twelve DIMM slots (established for the mapping, inferred that every one of them holds a DIMM).
2. **Checks, in order.** `eepromIsPresent` false: -104. Index at or above the slot's entry count: -105. Selector above 4: -105.
3. **Selectors.** The L1 keeps one 120-byte record per table entry. Selector 0 (record type 1): the 8-byte common header, copied raw. Selector 1 (type 1): chassis information area. Selector 2 (type 1): board information area. Selector 3 (type 1): the fourth area slot (the multi-record area; the product area slot is never served). Selector 4 (type 2): the DIMM's raw serial-presence-detect bytes. A record whose type does not match the selector, or whose area slot is absent, gives -104; a record whose parse failed gives -107.
4. **Record layout (established for the offsets read; sizes are in bytes, inferred).** Type 1 records are an IPMI FRU: the common header is read first (8 bytes) and checked with `ipmiCheckCSum` ("EEPROM header checksum error"), then the area offsets in header bytes 2 to 5 select four areas, each stored as (error, size, pointer) at record offsets `+0x10`, `+0x1c`, `+0x28`, `+0x34`. Type 2 records hold a 256-byte buffer read from the DIMM's EEPROM (`0x12e5c0`), checked against its own checksum byte (the sum of bytes 0 to 62 must equal byte 63; "SPD checksum error" on failure; the full DIMM request is in [sn2-dimm-spd-request.md](sn2-dimm-spd-request.md)), size at `+8`, pointer at `+0xc`.
5. **Reply.** Status 0; then an int: the number of bytes of the area left after this chunk (area size minus offset minus length, floored at 0); then a raw-byte argument of `length` bytes from the area at `offset`, shortened if the area ends first (a length of zero sends no bytes). An offset at or above the area size gives -105. The node asks for at most 64 bytes per request.
6. **What the node's parsers need in the bytes** (the chips repository's SN2 boot-sequence page): for a DIMM, SPD bytes 5, 9, 18, 23, 25, 29, 31 and 42 of the first 48; for the board and chassis areas, version, length in 8-byte units, a checksum that makes the whole area sum to 0, and `0xc1`-terminated type/length-coded fields in the IPMI order (chassis: type, part number, serial number; board: language, manufacture time, manufacturer, product name, serial number, part number, FRU file id, custom fields).

## The 0x100b reply

The question: the node firmware sends `0x100b` as its first request and waits for a reply frame; does the real L1 answer it? **Answer: no frame of any kind comes back (established), so the node's wait for it times out (established), and the node carries on regardless (established for the call site read).** Both earlier readings were partly right: the L1 sends nothing, and the node does wait. A sentence above and in the review that said the node's reply read "expects none" was wrong on that point and is corrected here.

**Node side (SAL_B of snprom 10.54, Ghidra export plus objdump cross-check; established).**

- `sysctl_init` (`0xffa90e60`, and a second copy at `0xffa91060` reached by a jump at `0xffa912bc` that differs in the address word, `0x4ffff000` instead of `0x0ffff000`) opens a slot (`0xffa819c0`), builds the request with `0xffa81f40` (task 6, code `0x100b`, three integer arguments, so a count byte of 3 and a 4-byte integer after each type byte `0x00`), sends and waits with `0xffa82c10`, closes the slot (`0xffa81e40`), and on success decodes the reply with `0xffa82270`.
- `0xffa82c10` is a 16-byte wrapper that loads the timeout `0x2dc6c0` (3,000,000 ticks) and tail-jumps to `0xffa82a10`. Nothing in the call path skips the wait for `0x100b`: the only bypass is the simulator signature test on `cpuid[2]` (`0xaeeeeeee8badbeef`), which returns -24 without sending. The wait itself (`0xffa82730`) polls the slot's reply-ready byte (slot offset `+529`) until it is set or the deadline passes. On the deadline it returns -24 (the reply-ready byte was still 0); with the byte set it decodes the frame. The tick unit of the timer routine `0xffa0da70` was not pinned (unknown); the count 3,000,000 is exact.
- On -24 the send-and-wait routine calls the progress-code routine `0xffad2cb0` with `0x8d` (`0xffa82bf8`), then returns -24 to its caller. **Correction to the new reading:** that routine returns normally (its paths end in `br.ret`, no halt was found), so the node does not die at progress code 0x8d on this request. It displays the code and carries on. Whether `0x8d` is visible on a real brick display was not read.
- `sysctl_init` returns the -24 and its caller at `0xffa1f62c` ignores the result (the next instruction returns). The caller of the second copy was not followed (unknown).
- What the node does with a reply, if one came: the decoder `0xffa82270` reads the status (4 bytes) then the count byte and its arguments; it is called with zero expected arguments, so a status-0 frame with a zero count would be accepted. No reply is the L1 behaviour, so this decode is not reached on a real brick.

**L1 side (L1 1.62.1 `l1`, read with an ARM disassembly of the load image; established unless marked).** Every path from the received frame to the outbound response for code `0x100b`:

1. `syscom_process_inbound_request` (`0xc2284`) parses the frame into an argument list whose argument 0 is the request code (`ARG__START` at `0xc23d8`), so the argument count it passes on is the frame's count plus one. A call to the optional capture hook `SysComCaptureInput` (a global that is a pointer, called with the frame; its return is ignored) precedes the routing. It then calls `IRouterRequest` (`0xc2664`). It sends no reply of its own, and if routing fails it only logs.
2. `IRouterRequest` (`0x10aafc`) builds a packet and calls the routing routine `0x109af8`. That routine, for a destination registered as a function (`L1ReqInit`, `0xc82d0` to `0xc82e4`, registers `L1ReqIRouter` as the request function of router id 6), calls it directly in the same task, with the packet, and discards its result (`0x10a150` to `0x10a164`). So there is no queue, no other task and no deferral.
3. `L1ReqIRouter` (`0xc8388`) is a big switch; the `0x100b` case is `0xcb474`. The packet's argument count is at `+0x20`. A count of 4 (the code plus three) is the only value that takes the work path: it builds the version word from the three integers, calls `vramSlotSet`, logs through `nvramLogFinsert`, and, when a stored slab is set and `SysComGetSlab` of it equals the slab byte kept in the handler's frame, calls `SysComEventDisable`. None of those starts or sends a reply (`SysComEventDisable` at `0xc4a84` is 44 bytes: it writes one global and calls one other routine, `0x2dda8`, a setting store whose first 0xd8 bytes contain no call; it is not a router routine, and the reply path needs the reply count word that only `ARG__START` sets). Any other count calls `ARG__START` with -100, which is the only reply.
4. Both branches jump to the common tail at `0xcf14c`. The tail sends a reply only if the reply count word at `fp-0x2b0` is non-zero (`0xcf198`); `ARG__START` (`0x9df34`) is the only thing that sets it to 1, and the work path never calls it. It then frees the request packet (`IRouterFreePacket`, which has no send call).

The node firmware's own `0x100b` request carries three integers, so its count is 4 and it takes the work path. A different argument count would produce a status -100 frame, but the node firmware does not send one.

**Conclusion for an emulated L1 (established).** For `0x100b` with three arguments send no bytes back. Do the L1's side effects only: record the version and run the `SysComEventDisable` step when its slab test passes. For any other argument count the reply is the ordinary status frame: a reply of status -100 and no further arguments: byte 0 is `0x20` plus the subchannel, then `ff ff ff 9c`, then the argument count 0 (inferred from the reply layout above; `ARG__START` records only the status). The node firmware never sends a wrong count, so this frame is not needed for it. Remaining unknowns: the tick unit of the node's 3,000,000 timeout, and how the node's frame receiver (`0xffa866b0`) decides which slot a received frame completes (if it could pair some other frame with the open slot, the wait would end early; not read, so a model should send nothing for `0x100b` and not assume an early end). The ordinary boot cost is therefore a full timeout wait before the node's next request.

## Not settled

- The exact order of the first node-to-L1 requests: the chips repository's boot-sequence page lists each request with its arguments and gives the order as inferred from an interpreter run; two positions (`0x0013` against `0x000e`, and the first `0x1005`) are not settled.
- The mode, device number and the remaining words of the Opus TIO, Dorado, Gemini, IP67 and IP69 channel entries, and whether those nodes use the JTAG-scanned UART or a wire.
- How requests `0x3001` to `0x3003` reach their handlers in `L1ReqIRouter`. `syscom_process_inbound_request` sends those codes to the chunk assembler first, yet the compare chain has rows for them (`0x3001` to `0x3004`); the path from the assembler to those rows was not read.
- The chunk-frame layout of this L1, the console frame class and index bytes, and whether the subchannel index of a reply always equals the request's (inferred).
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

Findings 1-6 addressed in the commit "software/firmware/l1: address the SN2 PROM-link review findings"; renewed review needed for the changed claims.

### Renewed review

Independent review of commit `5d62fbd`, the changes that answer findings 1 to 6 above, read after the fact by a reviewer who wrote none of the page or the fixes. The node end was read in the SAL runtime component (FIT type `0x12`, gunzipped from the 10.54 `shub1snprom.bin`, 918,824 bytes) with `ia64-linux-gnu-objdump`, which prints every instruction; this is a full decode, unlike the earlier bundle decoder. The L1 end was read in `/stand/sysco/bin/l1` carved from package 4 of `l1-1.62.1.bin` (1,999,676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`, matches the page) with Capstone's ARM decoder. Symbol and string addresses are the runtime's dynamic symbols and the L1's symbol table.

**Each fix against the binary.**

1. **Request `0x0006`, 8 arguments, task 6 (confirmed).** `sysctl_cmd_construct_req` (`0x4f8a0`) has 20 call sites; the one at `0x5085c`, inside the shared transfer routine at `0x50740`, passes task 6, code 6 and argument count 8 (immediates `mov r55=6`, `r56=6`, `r57=8`). The four routines reach it through three read helpers: `sysctl_eeprom_node` and `sysctl_eeprom_iobrick` call `0x50a10` and `0x50ce0`, `sysctl_eeprom_io_riser` calls `0x50ce0`, `sysctl_eeprom_dimm` calls `0x509d0`; each ends in `0x50740`. So "shared routine" is right for the request; the helpers differ by a selector constant (see the DIMM paragraph).
2. **Text command, task 3, code 0, two arguments (confirmed).** The call at `0x5c9bc` in `sysctl_cmd_text_command` has `r45=3` (task), `r46=r0` (code 0), `r47=2` (count). Strings `version`, `p %d fru capture`, `* fru capture`, `p %d display 2 %s` and `* display 2 %s` are each addressed through the global pointer by code (references at `0x58d00`, `0x58fac`, `0x59010`, `0x590ec`, `0x59176`). `p %d fru reset` and `* fru reset` have no reference by that method (searched as `addl` offsets from the global pointer, the form every other string above uses) and none as a stored 8-byte pointer, so "no reference found" now rests on a full decode, not on the first reviewer's limited one. Still not proof of absence (a pointer built another way would not show).
3. **`-104` from other handlers (confirmed).** The handler for `0x0004` at `0xc8ff8` ends a no-serial branch with `mvn r2, #0x67` (-104) before the call that writes the reply (`0xc9084`).
4. **Task 6 except the text command (confirmed).** I decoded the task and code immediates at all 20 construct-request sites: 18 numbered requests to task 6 (codes `0x03`, `0x13`, `0x0e`, `0x01`, `0x09`, `0x08`, `0x1005` twice, `0x0a`, `0x12`, `0x1007`, `0x100a`, `0x200e`, `0x200d`, `0x200f`, `0x15`, `0x06`, and `0x10` in the CPU-presence routine), the PCI operation (call at `0x595dc`: task 6, four arguments; its code immediate was not isolated, the `0x2004` in the table rests on the first review), and the text command to task 3. The count of 20 and the argument counts match the table.
5. **`0x3001` to `0x3003` (confirmed as open).** `syscom_process_inbound_request` computes the code, subtracts `0x3001`, and for a result of 0 to 2 (unsigned) calls `0xc4ab0` and returns to `0xc2704` (`0xc2378` to `0xc239c`); it does not enter the compare chain. The chain at `0xc8798` does branch to `0xcd90c`, `0xcdb78`, `0xcdbb4` and `0xcdbe0` for `0x3001` to `0x3004`. How a chain row is reached after the assembler took the code was not found (no call to the router from `0xc4ab0` located in a bounded scan), so "open" is the right grade.
6. **LED display-code strings (confirmed).** `chipset_common_ledlist` (symbol at `0xa3098`, 960 bytes, entries of a code and a pointer) points at "Reset request sent to system controller" (entry 23) and "Request for system reset failed" (entry 85); no handler or construct-request call refers to them.

**The UART stride (resolved by this review).** The first reading said the register index is shifted left by 6; the chips boot-path page says stride 8. Both statements are about real code: `sc_shub_dev_reg` (`0x60910`) shifts the index left by 6, and `sc_shub_dev_reg_offset` (`0x60960`) shifts it left by 3 on the base `0x7fed40000`. But the code that touches the UART, `sc_uart_*` and the PROM's own accesses, uses fixed addresses `0x8000000ffed40000 + 0, 8, 0x10, 0x18, 0x20, 0x28, 0x38` (for example `sc_uart_rxintr` and `sc_uart_txintr` use `+8`, the interrupt-enable register), so the UART's register spacing is 8 bytes. The `<< 6` accessor is the wrong thing to quote for the UART; the sentence on the page is corrected in this commit.

**The DIMM request (resolved by this review).** Both descriptions are of the same call, one level apart. The request is task 6, code `0x0006`, 8 arguments, built at `0x5085c` (the sysco page is right about the request). The chips boot-path page's pair "length 4, id 48" is the pair of constants `sysctl_eeprom_dimm` hands to its helper at `0x509d0`: `mov r35=4`, `mov r37=48`, then a branch into `0x50740`. In `0x50740`, the third value is the byte count (each request carries at most 64 bytes: the routine takes the lesser of the count and 64, then subtracts 64 and adds 64 to the running offset), and the fourth is stored unchanged as one of the request's argument words. So the DIMM read asks for 48 bytes, with 4 as a fixed selector argument; the boot-path page has the two roles in the opposite order (48 is the length, 4 the selector). Established for the constants and the byte-count arithmetic; inferred that 4 selects the DIMM area (the other helpers pass 1 and 2). The first request's other arguments and the reply layout remain unread.

**Cleared scope.** Findings 1 to 6 as fixed in `5d62fbd`, the changed rows and sentences, and the UART-spacing correction made here. Not cleared (unchanged): the rows graded inferred, the reply bodies, and the order of the first requests.


## Review (stream M)

Independent review of sysco commit `16abc12` (the reply frame, the reply bodies, the `0x0006` handler and module type `F`), examined at `16abc12`, together with chips commits `63907b1`, `0f33729` and `ccc8fa0` (reviewed on the chips boot-sequence page). The reviewer wrote none of it. Method: the Linux L1 program carved from package 4 of `l1-1.62.1.bin` with this repository's CramFS reader (SHA-256 `22cb62871e44...` matches the first section), decoded with the retools container's ARM decoder; the node end read with `ia64-linux-gnu-objdump` on `shub1snprom.bin`.

**Confirmed (re-read in the binary).**

- `syscom_process_outbound_response` (`0xc3f64`): byte 0 `0x20 | subchannel`, bytes 1 to 4 big-endian status from argument 0, byte 5 the argument count (starts at 0 and counts the arguments after the status), then integers as `0x00` plus 4 bytes, raw bytes as `0x80 | n` plus `n` bytes, strings as `0x01` plus text; a payload past 127 bytes sets the overflow flag and drops the rest; a packet with a nonzero error byte at `+0x13` produces `ff ff ff ff` plus the code, nine bytes in all. The node's `sysctl_cmd_interpret_resp` (`0xffa82270`) reads status from bytes 0 to 3, count at byte 4, arguments from byte 5 after the node strips byte 0, tag 0 integers, tag 1 strings and `0x80`-flagged raw bytes, -32 for a wrong type and -33 for a wrong count. The two ends agree. The subchannel source (`SysComGetSubc` of the request tag) is read to the call, not through the table behind it, as the page says.
- Reply bodies: `0x0001` is `nvramGet(3)`; `0x0003` is `(setting 1 << 6) | setting 2` then the type (`=` becomes `#` when the per-slot byte is zero); `0x0013` for a type outside `,` `}` `.` `$` with one slot is status 0, integer 0, then the type character (otherwise the blade path with -106); `0x000e` is the slab byte of the request tag (workaround only for types `,` `}` `.` `$` or more than one slot); `0x0009` is `nvramGet(14)`; `0x0015` is `nvramGet(0xcc) & 0x7f`, read in full, so established (the table says inferred and "mask from the earlier reading"); `0x0010` single-slot form is status 0, node count, CPU count, then presence and enable per node.
- `0x100b` (`0xcb474`): argument count other than 4 gives -100; otherwise it builds the version word from three integers and never calls the reply start, so no reply is sent; the node's `sysctl_init` sites (`0xffa90f5c`, `0xffa9115c`) send integer 1 and the two bytes at `gp-2055960` and `gp-2055959`, and its reply read expects none (corrected: the node waits for a reply, see [The 0x100b reply](#the-0x100b-reply)).
- `0x0006` (`0xc9658`): -100 for four or fewer arguments counting the code, -104, -105, -107 as listed, selectors 0 to 4; device-id mapping at `0xc7664` (type `F` jump table entry `0xc7fd8`, id table at `0xc7fe8`): 0 gives 0, 1 gives 1, 4 to 7 give 2 to 5, 8 to 13 give 6 to 11, 16 to 21 give 12 to 17, 98 gives 18, 99 gives 19, everything else -1. The PROM's `0xffa84680` fixes selector 4, offset 0 and length 48 for DIMMs; chunks are at most 64 bytes.
- Module type: `brickInitBrickOps` (`0x214fc`) maps `F` (jump table `0x21528`, entry `0x216a8`) to `OpusTio_BrickOpsInit` (`0x3cc74`), which stores `IP57_CpuPresent` (`0x893f4`) among its hooks.

**Mechanical fix made in this review.** The `-100` status at entry was glossed as a "flag byte ... whose meaning was not read". It is the packet's argument count: the same field (`+0x20`) bounds the argument loop in `syscom_process_outbound_response`. Text corrected.

**Findings not fixed.**

1. `0x0001` reply value. The reply is `nvramGet(3)`: the `3` is the selector of a 16-bit setting (bits 23:8 of the word at `+0x1c` of the settings record, read at `0x303b4`), not a value. The code does not say what the field holds; its default (0) and the commands that change it are in [sn2-l1-settings-defaults-1.62.1.md](sn2-l1-settings-defaults-1.62.1.md), which settles the blank-image value. The PROM tests bit 11 of the word it gets back. See the chips boot-sequence review section for the PROM side and the effect of returning the integer 3.
2. `0x0010` order is fixable from the code and the page says it was not pinned: the single-slot handler (`0xca5f4`; reply code at `0xcaa14`) replies status, node count (`[fp-0x2fc]`, the array size), CPU count (`[fp-0x304]`), then per node the presence word and the enable word; the node's reader at `0xffae83c0` takes the first integer as the node loop bound. Both ends read. The form for `,` `}` `.` `$` or multi-slot bricks is a different shape and was not read.
3. `0x0003` and `0x0013` both return the type through the same `brickGetType` call; that `F` has exactly one slot (which `0x0013` needs for the two-integer form) rests on `brickNumSlots`, not read here.
4. The `0x0010` row grade (inferred) and the `0x0015` row grade (inferred) are lower than what was read; leave until the author regrades.

**Cleared scope.** The reply frame, the reply bodies for `0x0001`, `0x0003`, `0x0013`, `0x000e`, `0x0009`, `0x0015`, `0x100b`, `0x0006`, and module type `F` as read above. Not cleared: `0x0010` for other brick types, the subchannel index table, the ordering of requests, and the contents of the debug-switch field on a deployed machine (the blank-image default is in the settings note).
