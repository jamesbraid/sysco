# Origin 300 L1 USB and L2: public revision-scoped evidence

The L1 port connects the controller to a system console. Ports 1 and 2 serve auxiliary equipment such as a keyboard or mouse. These connector statements do not identify the controller IC or its revision.

A hub on the router PCB serves modules connected to router ports 2–5. An external four-port hub serves servers connected to router ports 1, 6, 7, or 8. The L2 communicates with rack L1 controllers, and two of its four USB ports connect to the NUMAlink module L1 port and external hub. This establishes product topology. It does not establish the USB packet or application protocol of any L1 revision.

## Firsthand 2012 capture

In the [March 22, 2012 post in the two-brick Origin 300 thread](https://nekonomicon.irixnet.org/forum/14/16724335/1.html), the operator says he connected two Origin 300 bricks, an L2 controller, and a NUMAlink router. The post does not recheck the brick L1 version at USB attachment. Neither reported revision is the selected brick L1 1.44 image.

The L2 console in that post first logs a USB hub on `bus1/1` with seven ports and then enumerates the router L1 on `bus1/1/1`. It identifies the connected device as `SN1 L1 System Controller` with manufacturer `Silicon Graphics, Inc.` and serial string `00000000`. At the first brick attachment, near the operator's “plug in the first O300 brick” sentence, the L2 logs two failed USB address attempts (`error=-110`) on `bus1/1/4`, then a successful third attempt and L1 connection. It reports system serial reassignment from the attached L2. The following `cfg` and `serial` commands show brick `001c01` under the L2's system serial. The second brick enumerates on `bus1/1/3`, reports another reassignment, and appears as `001c02` in the L2 configuration. These address failures occurred in this installation. They are not a required attachment sequence.

Near `001r08-L1>let the carnage begin`, a router L1 reboot produces an L2 USB disconnect on device 4 and connection on device 5. The L2 then uses a changed `/dev/sgil1_` path and reports one “no response” message. Later, near the operator's power-down and replug account, the second brick disconnects and re-enumerates.

The post describes a later PROM discovery problem with the second brick that cleared after power-down and reconnection. That guest-visible outcome is separate from the USB attachment events. It cannot establish which USB operation caused the PROM change.

## Fitment and transfer limits

The public manual locates Origin 300 L1 controller functions in the I/O-8 assembly. Its IP45 and I/O-8 descriptions do not name a USB slave controller. An operator's 2013 IO8 photograph discussion says the available image lacks enough resolution to read part numbers.

It supports a USB-device attachment path and L2-driven rack identity in that topology. The evidence cannot establish selected brick L1 1.44's descriptor, reset behavior, endpoint widths, command semantics, chip variant, or whether the same failure and reassignment sequence occurs there. A suitable next source would be a revision-identified I/O-8 schematic or BOM, or an original photo with readable IC and board markings. Any approximation would need its own stated limit and test.

External access was therefore used for the named public page and SGI-published guides.
