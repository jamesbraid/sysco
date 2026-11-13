# Multi-node console and master election

On a machine with more than one module, each module has its own module
system controller (MSC, also called ELSC; see [controllers.md](../../evidence/controllers.md)).
A four-node Origin 2000 module has one MSC for its four nodes (SGI 108-0164-002, Figure B-1; see
[module management](../../platforms/ip27/module-management.md)), so the earlier wording "each node has its
own module and MSC" was wrong for that product. The console
and the diagnostic output then span those controllers. This note records which module owns
the interactive console, where a failing node's output goes, how the kernel relays messages
to every module, and which facts are still open.

Every fact here is restated from the IRIX 6.5.7 IP27 PROM (console selection, master
election and output formatting) and from the IRIX 6.5.7 kernel system-controller driver
(the console and warning broadcast). The PROM and kernel images are not re-read against the
files in this repository, and no image address is recorded for these facts. Each one is
therefore graded inferred. Constants are in [formats/msc_multinode/format.toml](format.toml).
Cite the IRIX release with any of these facts, because PROM revisions differ.

## Master election

The interactive console lives on the module that wins master arbitration. Arbitration has
two tiers. Within a node, the two processors elect a local master. Across nodes, the local
masters elect one global master, and a partitioned machine also has one partition master per
partition.

- The election prefers a hub that carries a BASEIO, the I/O board with the console serial
  chip (IOC3). It honours an operator GlobalMaster override variable. If the winner has no
  BASEIO, the firmware says so ("master ... does not have a console").
- The winner is a software decision: network-interface comparison, the BASEIO preference and
  the override. It is recorded as the global or partition master index in the PROM
  configuration. The per-hub master queries read that index.
- A boot-status-word bit for "system controller passed arbitration" is declared. The PROM
  neither sets nor reads it, so it is not a live record. The PROM-configuration index and the
  scratch-register advertisement carry the result instead. Nothing here is a hardware line.

Discovery runs over the hub network-interface (NI) scratch registers, not over any
controller-to-controller channel. The Hub NI registers are described in the Hub documentation,
which this repository does not hold.

- At power-on each hub writes its board identity and partition id into NI scratch register 0.
- Each hub writes into NI scratch register 1 its node-identity and status fields: a master
  request bit (set from the console or a forced-console override), console presence, power and
  diagnostic flags, the slot, the module and the firmware version. The node number is added to
  register 1 after arbitration. No CPU-mask field exists.
- Discovery reads both registers on each remote hub with a routed network-vector read over
  CrayLink, not a local access. Routers cache the slow NIC read in their own scratch registers.

## The console device

The console device is chosen by probing, and the probe depends on the role of the module:

- The IOC3 serial console and the manufacturing "junk" UART console are usable only by the
  local master. Both are tied to that node's I/O board.
- The controller's own serial (the MSC or ELSC UART) is not role-gated. Any node can use its
  own controller serial as a console.

So the master module normally uses its IOC3 serial, and the controller serial is the fallback
available on every node. The console moves to the controller serial only when the IOC3 or
BASEIO fails the selection walk, because it was not the master, failed, or was disabled by
diagnostics.

## Relay to every module

The kernel's system-controller code has a broadcast primitive. It walks every module in the
partition and sends a string to each module's controller. The string goes to that module's
alternate-console serial (the AUX port) or to its eight-character LED display. So a system
warning reaches every module's controller. This relay is separate from the console device
choice above.

Per-module identity therefore comes from two sources. Each module owns its own controller
console, and the kernel broadcast reaches each controller.

## Reading a console line

The firmware prefixes each line with a one-character CPU letter. The letter is `A` plus the
processor's slice number within its node: `A` for slice 0, `B` for slice 1, and so on up to
`D` on a four-processor node. Every node's slice 0 prints `A`, so the letter alone does not
tell nodes or modules apart. The module is printed as a `/hw/module/<n>/slot/...` path, and
the node by its NASID. A line that names a node or module therefore carries a number for it
and a letter for the processor, for example "NASID n CPU letter".

A bare `A:` or `B:` prefix can be two CPU slices of one node. It does not show two modules.

## Module serial publication

The module's one-wire manufacturing identity and its KLCONFIG serial field have different
owners. The module system controller exposes the module's one-wire line. The PROM reads that
line through the controller and publishes the decoded serial into the midplane KLCONFIG
record. The controller does not write KLCONFIG.

The PROM skips the controller read when the module-inventory path finds a populated,
non-memoryless first node. That path returns before it sends any one-wire command. The
zero-initialised KLCONFIG serial field then stays empty, unless an earlier hub manufacturing
record supplied the same module identity. That earlier path reads a separate family `0x09`
module record while it enumerates the node-local hub bus. The family `0x0b` node and PIMM
record sits on the same bus. The hub-side responder is distinct from the later controller
route, even though both use the same device family code.

## Module id

The module's controller storage is the primary source of the module id. Each node also keeps
a persistent backup in its own PROM log. Firmware reconciles the two during discovery. The
complete Origin 2000 storage, voting and persistence rules are not covered here.

## Topology: open

The master election and the console relay both use the existing fabric. Arbitration is a
routed read of the hub NI scratch registers over CrayLink. The relay is a kernel broadcast
to each module's controller over the same fabric. Nothing in the PROM or kernel reading
above needs a separate hardware channel between controllers.

Whether real multi-module hardware carries a dedicated controller-to-controller interconnect
is unknown. The IRIX PROM and kernel reading above names no such channel, and nothing here
settles the question. The next route is a wiring description of a multi-module chassis, or
a published hardware manual that shows the controller connectors. The arrangement above
holds without that channel.

## Sources

- IRIX 6.5.7 IP27 PROM: console selection, master election and output formatting.
- IRIX 6.5.7 kernel system-controller driver: the console and warning broadcast.
- Hub NI scratch registers: described in the Hub documentation, not held here.
- ELSC host driver and the request set it sends: [elsc-host-requests.md](../elsc_host/README.md).

## Used by

- [Origin 2000 module management](../../platforms/ip27/module-management.md)
