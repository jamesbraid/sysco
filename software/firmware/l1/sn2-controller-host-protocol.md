# The SN2 kernel's side of the system controller protocol

On an Altix the Linux kernel never talks to the L1 or L2 controller directly. It asks the PROM to do it, through
SAL calls, and the PROM owns the wire. This note records what the kernel (the `snsc` driver and its event handler, the
`sn_console` serial driver and `l1.h`) sends to and expects from those calls. The controller side, and the wire
between PROM and controller, are the firmware notes in this directory.

Source: Linux v5.3, the last release with SN2 support: `drivers/char/snsc.c`, `snsc.h`, `snsc_event.c`,
`drivers/tty/serial/sn_console.c`, `arch/ia64/include/asm/sn/sn_sal.h`, `l1.h` and `intr.h`. The SAL entry points are
shared with the SHub and node work in the chips repository (`platforms/sn2/`). SGI's *L1 and L2 Controller Software
User's Guide* `007-3938-005` is the user-level view. Grades follow [AGENTS.md](../../../AGENTS.md); a kernel source
file is **established** for what the kernel does, and says nothing about the PROM or controller side, which is
**unknown** here.

## Two channels, one vector

Everything uses the interrupt vector `SGI_UART_VECTOR`, 0xe9 (`intr.h`), shared by three consumers. Established.

1. **Console**, a serial port named `ttySG0` (`sn_console.c`): the PROM presents the node's console as a character
   stream.
2. **System controller subchannels**, `/dev/snsc/<name>` (`snsc.c`): a message pipe to the controller network, one
   device per compute node.
3. **Environmental events** (`snsc_event.c`): messages the controller pushes, received on a subchannel opened for that
   purpose on the console node only.

## Console calls (established, `sn_sal.h`)

| SAL function | Value | Purpose |
|---|---|---|
| `SN_SAL_CONSOLE_PUTC`, `GETC`, `PUTS`, `GETS`, `GETS_TIMEOUT`, `POLL`, `INTR` | 0x02000021 to 0x02000027 | one character out, one in, string out, string in with and without timeout, input pending, interrupt control |
| `SN_SAL_CONSOLE_PUTB` | 0x02000028 | write a buffer and not return until it is done (`snt_hw_puts_raw`) |
| `SN_SAL_CONSOLE_XMIT_CHARS` | 0x0200002a | queue a buffer asynchronously; returns the number accepted (`snt_hw_puts_buffered`) |
| `SN_SAL_CONSOLE_READC` | 0x0200002b | read one character after an interrupt |

Interrupt control takes an operation (off 0, on 1, status 2) and a mask (transmit 1, receive 2). The interrupt handler
reads the status and acts on the receive and transmit bits; the transmit interrupt is switched off when there is
nothing left to send. The driver has a polled mode, with a timer and a 16-character, 9600 baud model of the UART
FIFO (`SN_SAL_UART_FIFO_DEPTH`, `SN_SAL_UART_FIFO_SPEED_CPS`), used before interrupts are enabled, and an interrupt
mode. The driver's own transmit buffer is 64 KB (`SN_SAL_BUFFER_SIZE`).

## Subchannel calls: the "IRouter" pipe (established, `sn_sal.h`, `snsc.c`)

The controller network is called the **IRouter** in the kernel and in the guest tools (the L2 command line reports
"IRouter:read failed", 007-3938-005 p.49). One SAL function, `SN_SAL_IROUTER_OP` (0x02000043), carries every
operation; the first argument selects it.

| Operation | Value | Arguments | Result |
|---|---|---|---|
| open | 0 | node | subchannel number, negative on error |
| close | 1 | node, subchannel | status |
| send | 2 | node, subchannel, buffer, length | bytes accepted, possibly zero, or negative |
| receive | 3 | node, subchannel, buffer, pointer to capacity | status: OK with the length filled in, or error for "no data" |
| interrupt status | 4 | node, subchannel | mask of pending conditions |
| interrupt on | 5 | node, subchannel, mask | |
| interrupt off | 6 | node, subchannel, mask | |
| init | 7 | node, buffer, length | status |

A mask is transmit 1 or receive 2, the console's values (`SAL_IROUTER_INTR_XMIT`, `_RECV`). The `init` call gives
the PROM a 2048-byte buffer for subchannel queues; the driver reads a failure as "outdated PROM". The receive call returns the
SAL status in the result code, so a failure (`SALRET_ERROR`, -3) is how "nothing to read" appears, and the driver
treats any negative status that way and retries. Status codes: OK 0, more passes 1, not implemented -1, invalid
argument -2, error -3 (`sn_sal.h`).

Framing is the PROM's. The kernel moves at most 127 bytes per call (`CHUNKSIZE`) and treats the stream as opaque: the
user process that opened the device builds and parses the IRouter packets. The driver does the following:

- At boot, for every compute node (`num_cnodes`), it creates `/dev/snsc/<module>^<slot>#<slab>` from the node's
  geography, calls `init`, and enables the receive interrupt (`snsc.c`, `scdrv_init`).
- Opening a device allocates a subchannel on its node and hooks the shared vector. Reads and writes copy at most 127
  bytes per call, block with a one-second timeout loop, and wait on the interrupt status.
- Poll reports readable on the receive bit and writable on the transmit bit.

## Environmental events (established, `snsc_event.c`)

Only the node that `SN_SAL_GET_MASTER_NASID` (0x02000004) names as the console node opens an event subchannel. It calls
`SN_SAL_SYSCTL_EVENT` (0x02000044) with its node number to ask the controllers to deliver events there, and receives
them through the same receive call, from a tasklet, until the call reports nothing more.

An event is a big-endian record, in 127 bytes or fewer:

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | source address |
| 4 | 4 | event code |
| 8 | 1 | argument count, must be 2 |
| 9 | 1 | type of the first argument, must be integer (0x00) |
| 10 | 4 | the ESP code (an integer, big-endian) |
| 14 | 1 | type of the second argument, must be ASCII (0x01) |
| 15 | 2 | skipped (a carriage return and line feed) |
| 17 | rest | the text, null terminated; trailing CR and LF are stripped. The driver forces a terminator at index 126 counted from offset 15 (offset 141), which is past its own 127-byte buffer, so real records must be shorter (inferred) |

Other argument types are `IR_ARG_UNKNOWN` 0x80 with the length in the low seven bits, and a record that does not fit this
shape is dropped. The code is class (mask 0xf000), severity (0x0f00) and component (0x00ff). Classes: power 0x1000, fan
0x2000, temperature 0x3000, environment 0x4000, test fault 0x5000, test warning 0x6000, power-down notify 0x8000.
Severities depend on the class: power has stable, low or high warning, high or low fault (0x000 to 0x400); fan has
stable, warning, fault; temperature has stable, advisory, critical, fault. Each maps to a kernel log level. A
power-down notify event, or the environment code 0x4101 (`ENV_PWRDN_PEND`), makes the kernel print a notice once and send
`SIGPWR` to init; every other event is printed with its ESP code.

## Brick types and the `l1.h` codes

`l1.h` is only the **brick type codes** that come back from the controller (and the board codes the Altix 4700 reports),
not a command set; the kernel code of v5.3 does not send controller commands itself. The list and the match to the
firmware's own type letters are in [sn2-brick-packages.md](sn2-brick-packages.md). The ARM L1 firmware in this
repository is the other end of the wire; the SAL layer between is not held (no Altix PROM or SAL binary).

## Other SAL controller calls (established, `sn_sal.h` names)

`SN_SAL_SYSCTL_OP` (0x02000030) with operations to retrieve the I/O board type (1) and to issue a TIO clock reset
(2, `ia64_sn_sysctl_tio_clock_reset`, valid only on TIO nodes); also named: module id get 0x02000031, get 0x02000032,
I/O brick module get 0x02000033, I/O port speed get 0x02000035, slab get 0x02000036, partition get 0x0200003a, FRU capture
0x0200003f, I/O brick PCI operation 0x02000042. Their argument layouts were not read here beyond the TIO reset and the
I/O board get.

## What a model of the controller needs, from this side

- A SAL that answers the console calls and the eight IRouter operations, with one subchannel per open and 127-byte
  chunks.
- An interrupt on 0xe9 raised when a subchannel has received data or can accept more.
- On the console node, one event subchannel that can deliver the record above.
- The bytes behind the pipe, the IRouter packet format, are the controller's own protocol: user-space tools build it
  (the `sgil1` USB driver and `l2cmd` of the Altix support disc, whose source is held as a file set in
  `evidence/irix-files.md`), and its grammar is **unknown** in this repository. Route: read the IRouter framing in
  the `l2cmd` and `libsnxsc` binaries of that disc and compare it with the firmware's chunked request frames
  ([chunked-request-frames.md](chunked-request-frames.md)).

## Unknown

- What the PROM does between a SAL call and the controller (which UART or USB path, which buffers, which timing).
- The controller packet format on the IRouter subchannel.
- The meaning of the source address and of the ESP code beyond how the kernel prints them.

## Review

Independent review, 2026-11-14, by a reviewer who did not write this note. Reviewed sysco revision 89bff53.

Checks against Linux v5.3 (`arch/ia64/include/asm/sn/sn_sal.h`, `drivers/char/snsc.c`, `snsc.h`, `snsc_event.c`, `drivers/tty/serial/sn_console.c`, `intr.h`):
- Console calls 0x02000021 to 0x02000028, 0x0200002a, 0x0200002b; `SN_SAL_IROUTER_OP` 0x02000043; `SN_SAL_SYSCTL_EVENT` 0x02000044; `SN_SAL_GET_MASTER_NASID` 0x02000004; `SN_SAL_SYSCTL_OP` 0x02000030 with operations 1 and 2; the other listed sysctl numbers (0x31, 0x32, 0x33, 0x35, 0x36, 0x3a, 0x3f, 0x42): all match.
- IRouter operations 0 to 7 and their argument and result conventions (open and send return the first result word, close, receive and init return the status; receive passes a pointer to the capacity; init passes node, buffer, length): match `ia64_sn_irtr_*`. Status codes 1, 0, -1, -2, -3 match. Chunk size 127, driver buffer 2048, one-second wait, "outdated PROM?" message, device name `<module>^<slot>#<slab>`, receive interrupt enabled per node: match `snsc.c`/`snsc.h`.
- Event record: offsets 0, 4, 8, 9, 10, 14, 15 and 17 and the field types match `scdrv_parse_event`; the forced terminator lands at offset 141 (index 126 counted from offset 15), past a 127-byte buffer, as the note says. Class, severity and component masks, the class and severity values, `ENV_PWRDN_PEND` 0x4101, the `SIGPWR` path, and the console-node-only event subchannel (`ia64_sn_get_console_nasid` uses `SN_SAL_GET_MASTER_NASID`): match.
- Console driver constants (16-character FIFO model at 960 characters per second, 64 KB buffer, vector 0xe9, `ttySG0`): match.
- The note's grade (established for what the kernel does, unknown for PROM and controller side) is right for the call numbers and arguments, which come from `sn_sal.h` (platform header, first in the evidence order).

Findings:
1. The event layout, the 127-byte chunking and the device naming come from `drivers/char/snsc*.c` and `sn_console.c`, which are not in the platform trees the evidence order lists first. They are SGI-authored and describe only what the kernel does, which is how the note labels them; if the project wants the strict reading, the event-layout table is `inferred` until a controller-side source or binary confirms it.
2. PUTB's "does not return until done" is the note's reading of the driver's use (`snt_hw_puts_raw`), not a header statement; harmless.

Cleared scope: all SAL numbers, argument lists, result conventions, the event record layout as the kernel parses it and the "what a model needs" list. Not cleared: the strict-grade question in finding 1. Not read here: SAL or controller firmware behaviour (not held).

Disposition of finding 1: the evidence order now counts `drivers/char/snsc.c` and `snsc_event.c` as SGI Linux platform code (sgi-retools c50bfeb), so the event layout, the 127-byte chunking and the device naming from those two files are `established` as the note already grades them; no table changes. `drivers/tty/serial/sn_console.c` is SGI-written and SN-specific but is not among the files the rule names; its console-driver constants stay graded as the note has them until the owner says whether the rule covers it.
