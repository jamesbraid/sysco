# What a model of the L2 system controller needs on an Altix

What an Altix 3700, 3700 Bx2, 4700 or 450 needs from an L2 for the PROM to reach the `Shell>` prompt and Linux to reach a login: the L2's role, its links, its transports, what the node side sends it and what it answers, how the console is routed, and what happens with no L2. The Altix 350 is included only for contrast. This page gathers what the L2 notes in this directory, the L1 notes and the node-side notes in the chips repository already say, adds what the published guides say about the four products, and adds one reading of the L2 daemon. It does not repeat their derivations; follow the links.

Grades follow [AGENTS.md](../../../AGENTS.md), with one added label because the question is about what software expects:

- **established**: a manual states it for that product, or a routine was read in full in a shipped image, and the claim follows.
- **software expectation**: the node's PROM or kernel (read in the chips repository, or the public Linux v5.3 sources) or a manual describes what it expects, and the controller's side was not read for that product.
- **inferred**: a reasoned reading from partial evidence; the reasoning is stated.
- **unknown**: nothing held settles it; the route is in [questions/README.md](../../../questions/README.md).

Sources, by short name. **G6**: *L1 and L2 Controller Software User's Guide*, 007-3938-006 (printed pages). **A3**: *Altix 3000 User's Guide*, 007-4579-001. **BX**: *Altix 3700 Bx2 User's Guide*, 007-4377-002. **A47**: *Altix 4700 System User's Guide*, 007-4823-003. **A350**: *Altix 350 System User's Guide*, 007-4660-002. **L2 1.44.0**: `l2.bin` of IRIX 6.5.30 (SHA-256 `6bcfda07...e8761`), read through the notes in this directory, plus the string and routine readings marked "read here". **Linux 5.3**: `arch/ia64/include/asm/sn/sn_sal.h`, `drivers/char/snsc.c` and `arch/ia64/sn/kernel/setup.c` at v5.3. No Altix 4700 or 450 controller image and no L2 image newer than 1.48.0 was read; Altix 3000 L2 firmware is taken to be the same 1.x line (inferred, see section 1).

## Answer first

1. **Only the Altix 3700 has an L2 as a separate box** (the Origin 3000-era board this directory reads). The 3700 Bx2, the 4700 and the 450 have none: an L1 application on Linux becomes the L2 when a network link appears. The 350 has none unless an optional box is added. Established (manuals), section 1.
2. **To a booting node the L2 is nearly invisible.** The PROM probes for it once with the text command `version` addressed to `0x1ffff003`, counts an L2 as present only for reply status 0, and then sends reset and power-down as `p <partition> rst` and `p <partition> pwr d` to it; otherwise it sends `* rst` and `* pwr d` to its own L1. The kernel never addresses the L2; it moves opaque bytes through the PROM's controller pipe. Software expectation (PROM and kernel read); the L1's and the L2's `pwr d` handlers are established, the L2's `rst` handler is inferred. Section 4.
3. **Boot to `Shell>` and to a login needs no L2 at all**, provided the C-brick L1 answers the probe with a non-zero reply (never 0, never silence) and the node's partition is 0 and a node's console characters reach the console owner. Section 6.
4. **The console path with an L2 is half read.** The input half is established (one text request per typed character to the console owner's L1, task 5). How a node's output reaches an L2 session is not read; an operator-level model can sidestep it. Section 5.
5. **A single L2 never needs L2-to-L2 discovery, the partition database or the display.** Those are for multi-L2 racks, IRIX's boot firmware and the touch panel. Section 7.

## 1. Role, by product

| Product | L2 | Source | Grade |
|---|---|---|---|
| Altix 3700 | A single-board computer at the top of the rack, running embedded Linux from flash. Standard on the 3700. Required in a rack that holds an R-brick or an L2 touch display, or when remote maintenance is wanted. It controls resource sharing, controls every L1, keeps controller configuration and topology, and routes between upstream devices (consoles, rack display, modem) and downstream ones (R-brick USB hubs, L1s). Several L2s in a system are peers and each passes on what it learns. | A3 pp. 35 to 36; G6 pp. 3 to 4 | established |
| Altix 3700 Bx2 | No separate hardware L2: "need for a separate optional hardware L2 ... has been eliminated". The L1 of a CR-brick or NUMAlink 4 R-brick runs as an L2 emulator when a USB-to-Ethernet adapter is plugged into the brick's type A connector. It does what the Altix 3000 L2 does, takes its rack ID from its own L1 (the `rackid` command is not used), and does not support the touch display. The adapter goes on the bottom CR-brick of rack 001 for 64 processors or fewer, on the R-brick at U-23 of rack 001 for 128, on rack 2 U-17 and rack 3 U-23 for 256, and on four R-bricks (rack 3 U-05 and U-35, rack 9 U-05 and U-35) for 512. | BX pp. 18, 24 to 25, 33 to 35 | established |
| Altix 4700 | No L2 hardware. Every IRU and dense router carries an embedded Linux system controller; the L1 application always runs; connecting the IRU's (or router's) L2-host Ethernet connector to a live LAN makes it start the L2 application. `init 4` starts it regardless of the LAN, and `l1cmd l2` (or `l2` at the L1 prompt) reports whether it runs. | G6 pp. 4, 13 to 14; A47 pp. 27, 31 | established |
| Altix 450 | The same arrangement as the 4700, in the same guide section ("Altix 450 and Altix 4700"). | G6 pp. 4, 13 to 14 | established for the shared G6 text only; the 450's own guide was not read |
| Altix 350 (contrast) | Optional external box. Its L1 port is cabled to the L1 port of a compute module; an SGIconsole reaches the box over Ethernet. Without it the console is the L1's serial port. | A350 pp. 46 to 47; A3 p. 21 | established |
| L2 firmware family | The Altix 3700 manual prints L2 firmware V1.17.7 on the touch display; the shipped image read here is 1.44.0, with the same `l2` daemon and library layout. The daemon's help strings carry the commands G6 lists for the L2 (`config`, `destination`, `select`, `power`, `reset`, `version`, `l2find`, `multisys`, `ping`, `ip`, `dhcpserver`, `password`, `autopower`, `date`, `pbay`, `rackid`, `reboot_l2`, `shell`, `sysname`, `serial`), read here by string search; `env`, `log` and `smp` were not found as exact strings and are not claimed. | A3 (touch display examples, p. 44); [daemon.md](daemon.md) | inferred that 1.17.7 and 1.44.0 are one daemon family |
| What runs on the 4700, 450 and Bx2 L1s | The ARM Linux L1 package of the shipped container starts `l1` on the console in run levels 3 to 5 and `l2` and `webs` in run levels 4 and 5 ([../l1/arm-linux-packages.md](../l1/arm-linux-packages.md)), which matches "`init 4` forces the L2". These are the packages of the IRIX 6.5.30 container; no Altix 4700 or 450 controller image is held, so applying them to those products is a reading. | [../l1/arm-linux-packages.md](../l1/arm-linux-packages.md) | inferred |

The earlier statement in the chips repository that the Bx2 case was "not independently checked" is settled by BX: no hardware L2, an emulator in the L1.

## 2. Links

| Link | What it is | Source | Grade |
|---|---|---|---|
| L2 to a rack's L1s (3700) | Four downstream USB ports labelled L1 port 1 to 4. The L2 is the USB master. Each goes to the USB hub of an R-brick or, with no R-brick, to the L1 of a C-brick. The hub carries status and control between the L2 and the L1s behind it. The figure labels the links behind the hub RS-422. | A3 pp. 36, 39 (Figure 2-4, Table 2-2); [../l1/rs422-router.md](../l1/rs422-router.md) | established |
| What the L2 daemon opens | `/dev/sgil1_cs` (control) and `/dev/sgil1_<n>` (up to 40 devices), read and written as router packets; default connection mode is USB. The kernel-side driver, its ioctls and the 750 ms attach timer are in [daemon.md](daemon.md) and [l1-conversation.md](l1-conversation.md). | L2 1.44.0 | established |
| L2 to L2 | Ethernet. Peers discover each other on the subnet (`l2find`), and `multisys on` restricts them to the same system serial number. | G6 pp. 123 to 124; read here: datagram socket strings (`sendto`, `recvfrom`) and `discover.c` in `bin/l2` | established for the commands; the wire format of discovery is unknown |
| L1 to L1, Altix 3000 | RS-422 between a C-brick and its I/O brick; the NUMAflex cable carries an RS-485 ICMB channel between bricks. A brick's L1 knows only its attached I/O brick and one attached C-brick. | G6 pp. 3, 24 to 25; A3 p. 56 | established |
| L1 to L1, Bx2 and 4700 | NUMAlink cables, low-voltage differential signalling, between CR-bricks, R-bricks (Bx2) and IRUs (4700); an I/O brick's L1 reaches the CR-brick over the Crosstown2 connection by RS-422 (Bx2). Whether that signalling shares the NUMAlink wires or only the cable assembly is unknown. | BX p. 18; A47 p. 27 | established (manual); wire unknown |
| Node to L1 | One UART per node on an Altix 3000 C-brick (two nodes), reached by the PROM through the SHub junk-bus UART. The L1 tells nodes apart by the link, not by a field. | [../l1/cbrick-console-routing-1.62.1.md](../l1/cbrick-console-routing-1.62.1.md) | established |
| L2 console and modem | RS-232 ports on the L2 box (the manual says 115 kbaud; the boot stage's block says 38,400, unreconciled; the daemon's table says "console port 38400"). A touch display on the rear door of rack 001 and an `ICMB` connector appear on the box; neither is a link a node uses. | A3 pp. 37 to 40; [daemon.md](daemon.md) | established (existence); baud unknown |

## 3. Transport to the operator and to the L1s

| Transport | Facts | Grade |
|---|---|---|
| Ethernet on a 3700 L2 | 10/100 RJ-45, to a hub or an SGIconsole. The daemon listens on 9001, 9002, 9004, 8000 and 8002 plus `10 * N` for `--copy N`; command tools use 8000, application daemons 9002. A telnet session is `inetd` running `telnetd -L /bin/l2term`, whose shell window is service `sgil2` on port 5000. The password (NVRAM key 19) is asked for by a network session and refused after three tries; `password clear` works only from the console or modem port. | established ([daemon.md](daemon.md)) |
| Ethernet on a Bx2, 4700 or 450 | The L2 listens on the same service set (it is the same program family), reached by telnet to the IRU or brick. Address by DHCP by default; a static one is set from the L1 serial console with `l2 ip <address> <netmask> <broadcast>` and applied by `reboot_l1`; `l2 serial set` and `l2 multisys on` follow the same pattern. | established (manuals: BX pp. 33 to 34, A47 p. 31, G6 pp. 14 to 15); the port set is inferred |
| Config view | On a 4700 `config` lists `L1 a.b.c.d:0:<index> - 001c01`: the L2's address, a connection number (always 0 on a 4700) and an L1 index. On a Bx2 the middle number is a USB port number (0 to 5 in the guide's example), the last is 0 for the brick holding the adapter and above 0 for bricks reached through it. On a 3700 it is the L2's own USB port. | established (BX p. 35; G6 pp. 16, 114) |
| L1 management link to an L2 | USB device controller on the L1 (the L1 is the slave), carrying router packets with no PPP wrapping; the `network` L1 command selects USB or RS-422. | established for the ColdFire L1 ([../l1/usb-overview.md](../l1/usb-overview.md)); the ARM L1's USB path is unread |
| PPP framing | An L2 started in serial or terminal-server mode sends `0x0d`, waits two seconds, sends `0x18` and reads until `0x18` returns before speaking PPP packets. Not used on USB. | established ([daemon.md](daemon.md)) |

## 4. What the node side sends, and what the L2 answers

**PROM.** All of it goes to the node's own L1 over the node UART, header byte `5 + slice`; the L1 forwards what is addressed to an L2. Full tables in the chips repository, `software/firmware/snprom/l1-requests.md` ("Requests after boot").

| Request | Destination word | Reply the PROM reads | What a model does | Grade |
|---|---|---|---|---|
| text `version` (L2 probe) | `0x1ffff003` (form 1, module `0xffff`, task 3) | status exactly 0 means "an L2 answers"; silence for 20,000,000 ticks takes the die path `0x8d`, which parks the boot processor (`0xffad2cb0` calls `0xffa9fc70` at `0xffad2dbc` before its own return, and that reaches the park at `0xffaeb7c0`; the chips page's "Silence" section owns this). A model must therefore answer the probe, never stay silent | With an L2: status 0 (the L2 runs the line through its command interpreter; its `version` output is a string). With no L2: an error status, never 0 (section 6). | PROM side established; L2's own `version` reply inferred (the handler was not read) |
| text `p <partition> rst` | `0x1ffff003` | any valid frame; not parsed | L2 resets the non-router bricks whose partition word equals `<partition>`. Sent only after the probe saw status 0. | PROM side established; L2 effect inferred from the `power` path |
| text `p <partition> pwr d` | `0x1ffff003` | any valid frame; not parsed | L2 picks every non-router brick of that partition (routers only when all else is off), sends each the text `power down` with a reply wanted, and answers once after the last L1 replies or after 15 s. The `p 0` form is accepted. | established ([routed-shutdown-and-partition-fetch.md](routed-shutdown-and-partition-fetch.md)) |
| text `* rst`, `* pwr d` | `0x0ffff003` (the node's own L1) | any valid frame | not an L2 matter; the L1 runs it and forwards to its attached bricks. **Sent only when request `0x0009` returned partition 0.** With no L2 and a non-zero partition the PROM prints `WARNING: system is partitioned but L2 is unreachable` and sends no text (reset `0xffa0d3d0`, power-down `0xffa0d610`; strings at `0xffba6110`, `0xffba6120`, `0xffba61e8`, `0xffba61f8`); reset then falls to its local tail at `0xffa0d4a0` (a store through `0xffa0e640` and a spin; the callee was not read) | established ([../l1/sn2-power-time-requests-1.62.1.md](../l1/sn2-power-time-requests-1.62.1.md)) |
| `0x0009` partition | `0x0ffff006` | one integer | answered by the local L1 from setting 14, not by an L2 | established |
| `0x1007` event subchannel, `0x1005` display, `0x200e` clock | local L1 | as the L1 pages say | not an L2 matter | established |

Not seen from the PROM: any request to task 10 (`0x1ffff00a`, the partition record). The PROM's request builder has 23 call sites; the codes of 21 were read and none is a partition-record request, every request on the main path is opened to `0x0ffff000` (the node's L1; a second wrapper set uses `0x4ffff000`), and the only L2-form destination found is the task-3 text to `0x1ffff003`; the code constants of the other two sites were not read. So the partition-record service is for IRIX's boot firmware, and for the Altix PROM it is **inferred** unneeded.

**Linux.** The kernel never builds a controller address and never touches the L2 or the node UART. It calls the PROM, which owns the wire (Linux 5.3; [../l1/sn2-controller-host-protocol.md](../l1/sn2-controller-host-protocol.md)):

| Kernel action | SAL call | Reaches the L2? | Grade |
|---|---|---|---|
| console characters (`ttySG0`) | `SN_SAL_CONSOLE_*` `0x02000021` to `0x0200002b` | no: node UART to the L1 | established (kernel); PROM side read in chips |
| controller pipe, one subchannel per open node, 127-byte chunks | `SN_SAL_IROUTER_OP` `0x02000043`, operations 0 to 7 | only if user space builds a packet for an L2; the grammar is the router packet in [router-packet.md](router-packet.md) | kernel side established; user-space senders not held |
| environmental events | `SN_SAL_SYSCTL_EVENT` `0x02000044`, console node only | the L1 or L2 chooses what to push; the kernel acts only on a power-down notice (`SIGPWR`) | established (kernel) |
| `pm_power_off` | `SN_SAL_SYSTEM_POWER_DOWN` `0x0200003b`, called with no arguments and never expected to return | via the PROM's `pwr d` text (L2 form when an L2 was found, per section 6) | established (kernel); what the SAL does with it is unread: **unknown** |
| partition number | `SN_SAL_SYSCTL_PARTITION_GET` `0x0200003a` | answered from the PROM's `0x0009` (L1 setting 14) | established (kernel); PROM mapping inferred |

So a booting Altix asks the L2 nothing the PROM does not ask during reset and power-down; Linux asks it nothing directly at all. On the controller path Linux uses only the console and `SN_SAL_IROUTER_OP` calls above, plus `SN_SAL_SYSCTL_EVENT` (`0x02000044`, console node only) and `SN_SAL_SYSCTL_PARTITION_GET` (`0x0200003a`), both in the table.

**Operator commands the L2 itself handles** (G6 pp. 111 to 133, listed because a model that serves a telnet session must parse them): `autopower [on|off|abort]`, `config [summary|verbose|list|l2|times|snap|diff|dump]`, `date [tz]`, `destination [reset]` and `rack <r> slot <s> destination`, `dhcpserver [on|off]`, `env summary`, `help`, `ip` (set, clear, `gateway`), `l1 [<cmd>]`, `l2 [<cmd>]`, `l2find`, `log [clear|insert]`, `multisys [on|off]`, `nvram reset`, `password`, `pbay`, `ping`, `power [up|down|summary]`, `quit`, `rackid`, `reboot_l2 [force]`, `reset`, `select ...`, `serial [set]`, `shell`, `smp [verbose|join|leave]`, `sysname`, `version [verbose|summary|check]`. A command the L2 does not know goes unchanged to the L1s of the current destination (default: all racks and slots; `partition <n> <cmd>` and `r <list> s <list> <cmd>` narrow it). Established (manual). The reply grammar of each was not read.

## 5. Console routing

| Fact | Source | Grade |
|---|---|---|
| An L2 session is in one of three modes: L2 prompt, L1 prompt of one brick (`r <r> s <s> l1`, or `<rack>.<slot> l1`), or system console. Ctrl-D enters console mode, Ctrl-T escapes one command and returns, `l2` or `l1` after Ctrl-T pins a mode. Messages: `entering system console mode (...), <CTRL_T> to escape to L2`, `escaping to L2 system controller`, `re-entering ...`. | G6 pp. 8 to 13, 20 to 23; read here: the strings in `bin/l2` | established |
| With an L2 present, "console mode from L1" is not supported: the L1 console is for a system with no L2 and a console cabled to the C-brick. | G6 p. 24; A3 p. 55 | established |
| `select` names one brick and one subchannel: `select <rack>.<slot>`, `select subchannel 0a`..`3c` (node or blade number plus CPU letter), `select subchannel console` (`console<n>` for node or blade n), `select partition <n>`, `select reset`, `select filter on\|off`, `select terse`. Output lines are `console input: <brick> <subchannel>`, `console output: filtered\|not filtered`, `console detection: L2 detected`. | G6 pp. 9 to 12, 128 to 130 | established |
| Default console: the lowest-numbered rack and slot that has already produced console output, else the lowest-numbered rack and slot. Per partition the daemon keeps two candidates and takes a brick only if both its rack and its slot are no greater than the stored ones; the preferred candidate must also report subchannel 4 ("console"). A compute-type brick with rack or slot 0 is never chosen. | G6 p. 21; [two-partition-nvram-and-console.md](two-partition-nvram-and-console.md) (`L2CfgMGetConsole`) | established for the compares; the manual's two rules do not say which wins, the code is the answer |
| **Input.** Each typed character is one class-0 request, code 0, with two integer arguments (0, then the character), to the console owner's address with the task field forced to 5 (low five bits, or bits 29 to 24 for the long address form), sent by the session routine `sub_10009e74` of `bin/l2`, read here in full. The L1 (task 5) writes it to the node link, with no acknowledgement from the node. | read here, L2 1.44.0 `0x10009e74`; L1 side [../l1/cbrick-console-routing-1.62.1.md](../l1/cbrick-console-routing-1.62.1.md) | established |
| Task 5 inside the L2 is a loopback test handler (`sub_1001dbc4`: strings `loopback packet no return data` and `packet %d loopback miscompare`), not a console receiver. | read here | established |
| `no response from 001c01 Junk bus CPU1A system not responding` is what a console session prints when the target answers nothing; the L2 side of that text is the response printer `sub_1000b904` (`ERROR: no response from <address>`). | G6 p. 22; read here | established for the text and the printer; the "Junk bus ... not responding" tail comes from the L1 and was not located |
| **Output** from a node to an L2 session. Each session registers a source address; a non-request packet addressed to it (tasks 2 and 7) is matched to the session by destination word in `sub_1000bed4`, and its string arguments are written to the session by `sub_1000b904`. That is how command replies come back. Whether console output is delivered the same way, and how the L1 chooses to send it, was not read. | read here | inferred for the receive path; **unknown** for the L1's sending side |
| Output from two nodes of one C-brick is interleaved untagged at frame boundaries by the L1 (up to 127 bytes a frame), and the L1 prints it only in console mode; `select filter on` is documented to restrict an L2 session to the selected brick's output. | [../l1/cbrick-console-routing-1.62.1.md](../l1/cbrick-console-routing-1.62.1.md); G6 p. 22 | established (L1 behaviour); the filter's effect in the L2 unknown |
| On a 3700 that has an L2 the L1 starts in its prompt state; node output is dropped until something puts the L1 in console mode. A model that wants text from reset must put the L1 in console mode (or let the L2 do it). | [../l1/cbrick-console-routing-1.62.1.md](../l1/cbrick-console-routing-1.62.1.md); chips `l1-requests.md` ("Two PROMs on one L1 console") | established |
| Switching an L1 out of console mode when the L2 wants its prompt (`L1 appears to be in console mode, switching it to L1 mode`) is present as a string; the code was not read. | read here | unknown (U13 of [l1-conversation.md](l1-conversation.md)) |

## 6. No L2

| Situation | What happens | Source | Grade |
|---|---|---|---|
| PROM probe | The text `version` to `0x1ffff003` is not run by the C-brick L1 (`0x1ffff` is neither its id nor `0xffff`); the router sends it to the catch-all, finds no remote route and answers a routing error: log line `B2B: no receiver (REQ)`, code -1408 from the error builder `0xffea7fc8`. On the node link the PROM sees status `0xffffffff` (-1) followed by a four-byte code of 8 (the node-link response routine `0xffe3eb40` sets length 9; per review, not re-read here). "Status byte 8" is the separate L2-to-L1 router-packet framing ([routed-shutdown-and-partition-fetch.md](routed-shutdown-and-partition-fetch.md)); an earlier version of this row conflated the two. Code -1608 on the 1.44.0 Origin 300 L1 is **unverified**: that image holds both -1608 and -1408 and which package serves the Origin 300 was not traced. Either way the reply is non-zero, so the PROM does not count an L2. It then uses the L1 forms `* rst` and `* pwr d` to `0x0ffff003` if partition is 0, and otherwise warns and sends nothing. | [../l1/sn2-power-time-requests-1.62.1.md](../l1/sn2-power-time-requests-1.62.1.md); [routed-shutdown-and-partition-fetch.md](routed-shutdown-and-partition-fetch.md) | established for the `O` ColdFire L1 (Altix 350), inferred for the `5` C-brick L1 (same image, same router) |
| Power and reset | From the C-brick: `* power up`, `* power down` and `* reset` target all attached bricks. The PROM's own `* rst` and `* pwr d` are sent only when its partition (request `0x0009`) is 0; a non-zero partition with no L2 gets a warning and nothing sent (section 4). | G6 p. 28 | established |
| Console | `Ctrl-D` at the C-brick's L1 console enters console mode; the console owner is the C-brick itself, or the brick that asked to be the console after power-on. Cable the terminal (38,400 baud, 8N1, RTS/CTS on a Bx2) to the bottom-most C-brick. | G6 pp. 28 to 29; A3 p. 21; BX p. 24 | established |
| Time of day | `0x200e` goes to the L1, not the L2, so it needs no L2. | chips `l1-requests.md` | established |
| Linux | Unchanged: it never addressed the L2. | Linux 5.3 | established |
| 4700, 450, Bx2 with no LAN | The L1 runs alone; the L2 application is not started. A serial console on the lowest rack and slot IRU gives the L1 prompt and console mode. | G6 pp. 30 to 35; A47 p. 31 | established |

## 7. Ready to model

Three levels; each includes the one above it.

**Level 1: what a node can observe (enough for `Shell>` and a Linux login).**

| Item | Rule | Grade |
|---|---|---|
| L2 present or absent | Decide by one probe: reply status 0 to text `version` at `0x1ffff003` means present; any other status, including the L1's routing-error reply (status -1 with code 8 on the node link), means absent. Default to absent. Never stay silent: a missing reply parks the boot processor. | established (PROM, ColdFire L1) |
| `p <n> rst`, `p <n> pwr d` | Accept as text commands; reset or power down the bricks whose partition word is `<n>`; reply once, status 0 and a string, after the last L1 has answered or after 15 s. | established for `pwr d`; inferred for `rst` |
| `* rst`, `* pwr d` | Not the L2's. Sent only when `0x0009` returned partition 0; with no L2 and a non-zero partition the PROM prints the "partitioned but L2 is unreachable" warning and sends neither. | established (PROM disassembly) |
| Everything else the PROM asks | Answered by the local L1 (`0x0003`, `0x000e`, `0x0013`, `0x0009`, `0x0010`, `0x0001`, `0x0006`, `0x0015`, `0x1005`, `0x1007`, `0x100b`, `0x200e`). | established |
| Linux | Nothing for the L2. Controller-path calls: console, `IROUTER_OP`, `SYSCTL_EVENT` (console node only) and `SYSCTL_PARTITION_GET`. Provide the pipe only if user space will use it; no kernel path needs a reply. | established |

**Level 2: an operator's session (telnet or console port).**

| Item | Rule | Grade |
|---|---|---|
| Prompts | `<name>-<rack>-L2>` once named; `L2>` otherwise; mode messages exactly as in section 5. | established |
| Targeting | Default destination all racks and slots; `r`, `s`, `<rack>.<slot>`, `partition <n>`, `l1`, `*`; unknown L2 commands go to the L1s of the destination. | established |
| `config` | One line per L1 as `L1 <ip>:<port>:<index> - <rack><type><slot>[.partition]` under `L2 <ip>: - <rack>` (and `(LOCAL)`). | established |
| `power`, `reset`, `select`, `ip`, `version`, `serial`, `sysname`, `rackid`, `multisys`, `date` | Text and effect as G6 pp. 111 to 133. | established (manual); exact error texts unread |
| Console | Section 5 input rule; output is the L1's console frames, untagged, in arrival order, with `select filter` as a display option. | input established, output **unknown** wire form |
| Password | NVRAM key 19, three tries, console and modem port exempt. | established |

**Level 3: running the shipped `l2` against modelled L1s.**

| Item | Rule | Grade |
|---|---|---|
| Attach | Per USB L1: configuration request (class 0, code 0, `0x0ffff00e`, three arguments) after 750 ms, repeating every 2 s until a reply parses, then every NVRAM key 15 seconds (5 by default); the reply (selector 7) builds the brick table and the route. | established ([l1-conversation.md](l1-conversation.md), [config-request-reply.md](config-request-reply.md)) |
| Routing | Brick route by rack (bits 27 to 18) and slot (17 to 12) from the table; `0x1ffff0xx` is the L2's task dispatch; undeliverable requests get a status-8 answer. | established |
| Task 10 | Needed only for IRIX's boot firmware; answer code 1 with the partition number, 0 if unlisted. | established |
| Packet format | [router-packet.md](router-packet.md), chunking in [../l1/chunked-request-frames.md](../l1/chunked-request-frames.md). | established |
| Do not model for one L2 | L2-to-L2 discovery, the display and touch panel (`l2gui`, framebuffer and `/dev/ttyS3`), the flash tools, the NVRAM event log. | inferred (single L2, no operator hardware) |

## 8. Not settled

| Question | Route |
|---|---|
| How a node's console output reaches an L2 session (the L1's sending routine on the C-brick, and whether it uses the response path above). | software now: the L1 1.62.1 package 3 SMP write task and its alternative consumer at `0xffe3b078`; the L2's response matcher |
| Whether the C-brick (`5`) L1 answers the `0x1ffff003` probe exactly as the `O` L1 does. | software now: the same router in package 3, a different type letter |
| The L2's reply to text `version` (status and string), and to `p <n> rst`. | software now: the task-3 command table in `bin/l2` |
| Whether SN_SAL_SYSTEM_POWER_DOWN and the EFI reset path send the text commands of section 4. | missing release or binary: the SAL runtime of the Altix PROM |
| The L2 hardware on a 3700 is the Origin 3000 board; an Altix 3700 L2 image is not named as such. | documents |
| The Altix 4700 and 450 controller images, and the Bx2 L1. | missing release or binary |
| The 4700's NUMAlink controller signalling, the L2 console baud, UDP discovery's wire format. | documents; software now (`discover.c`, the `l2term` serial driver) |

## Sources read

G6 pages 3 to 5, 8 to 35, 111 to 133 and the index; A3 pages 21 to 23, 35 to 44, 55 to 58; BX pages 17 to 18, 24 to 25, 33 to 36; A47 pages 27 to 31; A350 pages 46 to 47; Linux 5.3 `sn_sal.h` (call numbers, `ia64_sn_power_down`, `ia64_sn_sysctl_partition_get`) and `setup.c` (`pm_power_off`). L2 1.44.0: listing of `bin/l2` for `sub_10009e74`, `sub_1001dbc4`, `sub_1000bed4`, `sub_1000b904` and the packet router's class and task dispatch, and a string search of `bin/l2`, `libsys` and `libmain`. Nothing was run.
