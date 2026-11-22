# IX-brick and PX-brick topology: PIC widgets, IO9 device numbers, XIO ports and brick type codes

Reconciles the published manuals, one real Altix 3300 boot capture, the Linux IA-64 SN code and the shipped PROM images on four questions about the Origin 3000, Onyx 3000 and Altix 3000 I/O bricks: which PCI device number each IO9 function answers at, how the twelve slots are laid out, which widget and XIO port the C-brick sees, and what the PROMs call the two bricks. It settles the device-number conflict that the Altix 3000 pages left open.

Grades follow [AGENTS.md](../../AGENTS.md): **established**, **inferred**, **unknown**. A fact printed in a manual or in a real machine's console output, naming the slot or device, is established. Binary readings are marked as such and carry the image and address.

## Sources

- **IX:** *IX-Brick Addendum*, 007-4604-001, printed pages. **PX:** *PX-Brick Addendum*, 007-4605-001. **ALTIX:** *Altix 3000 User's Guide*, 007-4579-001. **DRV:** *Linux Device Driver Programmer's Guide, Porting to SGI Altix Systems*, 007-4520-006. **U3900:** *Origin 3900 Server User's Guide*, 007-4653-001. **O3000:** *Origin 3000 Series Technical Configuration Owner's Guide*, 007-4311-002. **L1G:** *L1 and L2 Controller Software User's Guide*, 007-3938-006. Figures cited here were rendered and read as images (DRV Figure 2-3 p.19, Figure 3-1 p.38, Figure 5-1 p.52); the rest were read as text.
- **CAP-3300:** an Altix 3300 (two C-bricks and one IX-brick, eight processors, 24 GB) console capture posted to comp.sys.sgi.admin on 15 July 2005, message-id `1121468096.910061.279690@g49g2000cwa.googlegroups.com`. It prints the SAL 3.01 rel030813 IP41 and EFI 1.02 start-up and the Linux 2.4.20-sgi221 boot. It is the only capture held from a three-PIC IX-brick.
- **CAP-IO9:** an Altix with an IO9 and an IX-brick or Altix 350 base module (the post does not say which), comp.sys.sgi.admin, 5 March 2004, message-id `c2b1ja$ssi$1@news.ucar.edu`: an EFI `pci` listing and a Linux `hinv -v`.
- **SAL 10.54:** `shub1snprom.bin` from snprom 10.54 build 70399 (SAL 5.04), sha256 `51ccf9ad830b85ee6c88d6d73d2512322a86454e63db51c57454679283a34617`, flat at `0xffa00000` (file offset = address minus `0xffa00000`). **IP35 PROM 6.210:** SGI Version 6.210, unpacked segment sha256 `554924380189a03a512156af2faa38344902f251b49fd5c80c253c23ecaff27b`, loaded at `0x1fc00000`.
- **Linux:** IA-64 SN headers of v5.3 (`klconfig.h`, `l1.h`, `module.h`, `xbow.h`) for constants.

## Answers

### 1. IO9 functions: bus and device number

All three functions sit on **PIC bus 1 behind widget 12**, at device numbers **1 (IOC4), 3 (ISP12160 SCSI) and 4 (Ethernet)**. Device 2 is the serial daughtercard slot; that it carries no PCI function is inferred (neither capture says the daughtercard was fitted).

| Fact | Grade | Source |
|---|---|---|
| The IO9 hangs off widget 12, PIC bus 1. CAP-3300's SAL prints the Ethernet adapter as `tg0 (widget 12, slot 4)`. | established | CAP-3300, SAL 3.01 start-up text |
| The SAL's "slot" is the PCI device number. One pair supports it: `tg0 slot 4` and Linux `01:04.0` in the same boot; DRV p.54 says "the device number is the same as the slot number". | established for that pair and for the guide's statement; the general rule is inferred | CAP-3300; DRV p.54 |
| The Ethernet controller is device 4. The same machine's Linux boot prints `PCI: Found IRQ 315 for device 01:04.0` for the `tg3` NIC (BCM5701, `partno(030-1771-000)`). CAP-IO9 shows `pci01.04.0` and its EFI `pci` listing `00 04 00`, vendor 14E4 device 1645. (The EFI device paths do not follow PCI device numbers: the SCSI disks are `Pci(1|1)` and the DVD `Pci(2|1)`, so `Pci(4|0)` for the NIC is not used as evidence.) | established | CAP-3300 (Linux 2.4.20 text); CAP-IO9 |
| The SCSI controller is device 3. CAP-3300's SAL prints `Walking SCSI Adapter 0, (pci id 3)` and `Walking SCSI Adapter 1, (pci id 3)`, and Linux prints `PCI: Found IRQ 314 for device 01:03.0`, then `pci01.03.0-1` and `pci01.03.0-2`. CAP-IO9 EFI shows `00 03 00`, vendor 1077 device 1216. | established | CAP-3300, CAP-IO9 |
| The ISP12160 is one dual-channel part: two adapters at one PCI id, named `-1` and `-2` by the Linux driver. | established | CAP-3300 (`pci01.03.0-1`, `-2`; "QLogic 12160 Dual Channel Ultra3 SCSI (Rev 6)" in CAP-IO9) |
| The IOC4 is device 1. CAP-3300 prints `PCI: Found IRQ 313 for device 01:01.0`, `IDE ATTACH: IOC4 RevId = 79` and the DVD-ROM; CAP-IO9 prints `00 01 00`, vendor 10A9 device 100A, and `IOC4 ... (rev 79) on pci01.01.0` with the DVD-ROM on it. | established | CAP-3300, CAP-IO9 |
| The IOC4 carries the IDE channel of the DVD-ROM (`xscsi/ide: SGI IOC4 BaseIO Card Detected : Firmware Rev 79`) and, by the manuals, the RS-232 ports, RT jacks, NVRAM and clock. | established for IDE; manual statement for the rest (IX pp.3, 5; ALTIX p.115) | CAP-3300, IX |
| The IO9 has no USB function. The four-port USB card is a separate PCI card in bus 6 slot 2, factory fitted on Onyx 3000 and optional on Origin 3000. CAP-IO9's EFI listing shows only devices 1, 3 and 4 on the base I/O bus. | established | IX pp.2, 3, 11; CAP-3300 |
| Linux names the devices one lower than their PCI device number: the SCSI part at device 3 reports `pcibr_try_set_device: Device(2)` and the IOC4 at device 1 reports `Device(0)`. | established (printed by the driver) | CAP-3300 |
| **DRV Figure 2-3 (p.19) has the Ethernet and SCSI labels the wrong way round.** It labels the Ethernet `Bus 1 Slot 3` and the SCSI `Bus 1 Slot 4`. DRV p.54 says the device number is the slot number, so the figure's label means device number; the printed machine says the reverse. A drawing and a console cannot both be right, and the console names the live device. | established (that the figure disagrees with the capture); the figure's error is a labelling slip, inferred | DRV pp.19, 54; CAP-3300 |
| The three numbers agree with an independent route: SGI's Linux IO9 test treats an IOC4 as an IO9 when an ISP12160 is at PCI slot 3 of the same bus (`ioc4_variant`, Linux 2.6.18 `drivers/sn/ioc4.c`, as recorded by the Altix IO9 page; not re-read here), and the SAL 10.54 per-bus device mask (below) enables windows for devices 1 to 4 on the base I/O bus. | inferred (software expectation) | chips `platforms/sn2/io9-board.md`; SAL 10.54 `pic_attach2` as read in chips `devices/pic/README.md` |
| Bus 1 runs 66 MHz, 64-bit. The `tg3` line prints `(PCI:66MHz:64-bit)`, and DRV p.38 notes that the IO9 is a 66 MHz PCI card. | established | CAP-3300; DRV Figure 3-1 note |
| The channel-to-connector map is not settled. In CAP-3300 both internal drives answer on adapter 0 (targets 1 and 2) and adapter 1 is empty. That fits internal drives on channel 0 and the external VHDCI connector on channel 1, but nothing was cabled to the connector. | inferred | CAP-3300 |
| The IO9 part family is `030-1771`: the Altix 3300 NIC reports `partno(030-1771-000)`. Origin 350 IX captures print IO9 `030-1771-005` and `-006`. Origin 3000 and Onyx 3000 IX-bricks have no capture held. That they carry the same functions at the same device numbers is carried over from the shared IX addendum drawing. | established (Altix part); inferred (Origin 3000 and Onyx 3000) | CAP-3300; public Origin 350 `hinv -mvv` listings (forums.irixnet.org thread 3009) |

IOC4 revision 79 (0x4f) in both Altix captures. An Origin 350 IO9 reports revision 0x53, so the IOC4 revision differs between products.

### 2. Slot count and layout

| Fact | Grade | Source |
|---|---|---|
| Twelve slot positions: six buses of two slots, numbered bus 1 to bus 6, slots 1 and 2. The IO9 takes bus 1 slot 1, leaving 11 positions for cards. The PX-brick has no IO9 and offers all twelve. | established | IX pp.1, 2, 10; PX pp.1 to 3; U3900 p.66 |
| Bus 1 slot 2 is reserved for the optional serial daughtercard. It occupies the position but uses no bus resources. A card fitted there instead must run at 66 MHz or faster. The standard two RS-232 ports are on a daughtercard wired internally to the IO9. | established | IX pp.2, 11 to 13 |
| The four-port USB card sits in bus 6 slot 2 (the rightmost slot). | established | IX pp.2, 3 |
| Each bus runs at its own speed and width. Populations: one 133 MHz PCI-X card at 133 MHz, two 133 MHz PCI-X cards at 100 MHz, two 66 MHz PCI-X, two 66 MHz PCI, two 33 MHz PCI. A PCI card with a PCI-X card forces PCI mode, and mixed speeds run at the slower. | established | IX pp.11 to 14; PX pp.8 to 9; DRV p.38 Note 2 |
| Bus 1 is 66 MHz because of the IO9; buses 2 to 6 are 133 MHz capable. | established | DRV Figure 3-1 and Figure 2-3 |
| Widget 12 drives buses 1 and 2, widget 15 buses 3 and 4, widget 13 buses 5 and 6. | established as drawn | DRV pp.19, 38, 52; IX Figure 2 p.4 |

### 3. Widgets and XIO ports as the C-brick sees them

| Fact | Grade | Source |
|---|---|---|
| The three PICs are widgets 12, 13 and 15. The hardware-graph paths `.../Pbrick/xtalk/12/pci-x/0` and `.../xtalk/15/pci-x/0`, the statement that the three host bridge numbers are stamped on the back panel, and the naming of both IX and PX as `Pbrick` are from DRV p.54. The capture supports only widget 12 (the IO9's `tg0`). | established | DRV p.54; CAP-3300 (widget 12 only) |
| The SAL 10.54 PROM carries a 19-row table, keyed by the L1 brick-type byte and the widget number, that gives the first PCI bus of a PIC (table at `0xffc2b2d8`, 17 words per row, read by `pci_widget_to_bus` at `0xffb6adb0`). The rows for `0x3d` (IX, row 3) and `0x23` (PX, row 0) are identical: widget 12 gives bus 1, widget 13 bus 5, widget 15 bus 3, widget 14 none. The PX Linux table (`io_brick_tab`, recorded by chips `pxbrick.md`) agrees. | established (binary reading) | SAL 10.54 `0xffc2b2d8`, bytes read directly; `0xffb6adb0` loop read in a flat listing |
| The widget number is the PIC's crossbar port number: Linux names the crossbar ports `XBOW_PORT_8` `0x8`, `XBOW_PORT_C` `0xc` and `XBOW_PORT_F` `0xf` (ports A to F are `0xa` to `0xf`). So Port A is widget 10 and Port B widget 11, which is why the connectors are labelled XIO 10 and XIO 11. Port C reaches the lower PIC (widget 12), Port D the upper (widget 13), and the middle PIC's own PCI side is Port F, widget 15. This also gives the meaning of the `Port F` label that the drawings leave open. | inferred (numbering and drawing fit; no text states it) | Linux `xbow.h` v5.3; IX Figure 2 p.4; DRV p.52 |
| Altix C-brick (IP41, two nodes): Node 0 XIO goes to XIO 10 / Port A, Node 1 XIO to XIO 11 / Port B. | established as drawn for the PX-brick; inferred for the IX-brick | DRV Figure 5-1 p.52 |
| In CAP-3300 the IX-brick is attached to the first C-brick, slab 0: SAL prints `Found I/O brick attached to module/001c04/slab/0/node`, with two C-bricks (001c04, 001c07) in the system. Slab 0 is Node 0, so the cable runs from Node 0 XIO of the first C-brick. | established (what the PROM printed); the connector reading is inferred | CAP-3300; DRV p.54 for slab and node |
| Origin 3900 Cx-brick (IP53 node boards): XIO connector n goes to IP53 node board n (XIO0 to board 0, and so on); each XIO connector of an IX- or PX-brick (XIO 10 and XIO 11) can connect to one IP53 node board, and the second may go to another node board. Which of XIO 10 or 11 connects to which Cx XIO connector is not printed. | established for the connector numbering and the either-board rule; unknown for the pairing | U3900 pp.60, 62, 66; IX p.10 |
| Origin 3000 C-brick (IP35): one node and one 1.2 GB/s Xtown2 channel, so there is no Node 0 or Node 1 split. Which of XIO 10 or 11 it uses is not printed. | established (single channel); unknown (port) | O3000 p.23 Figure 3-9 and its list |
| The L1 `ioport` command shows a compute brick's two I/O ports as `Compute I/O port B` and `Compute I/O port D`, and an I/O brick's as `PIC port A`, `PIC port B`, `PIC0 <-> PIC1` and `PIC0 <-> PIC2`. PIC0 is inferred to be the middle PIC. Which node's XIO is port B or D is not printed. | established (text); unknown (mapping) | L1G pp.96 and 97, Examples 3-59 and 3-60 |
| IP35 PROM 6.210 accepts a hub widget id of 10 and makes port 11 the other port, any other id making port 10 the other. This concerns the compute node's own crossbar and does not say which cable goes where. | established (binary reading, recorded in chips) | chips `platforms/ip35/prom-host-xio-port.md` |

### 4. Brick type codes and names

| Fact | Grade | Source |
|---|---|---|
| The IX-brick is board type `0x77` (`KLTYPE_IXBRICK`, `KLCLASS_IOBRICK 0x70` plus 7) and the PX-brick `0x76` (plus 6). | established | Linux `klconfig.h` v5.3 |
| The L1 brick-type character is `=` (`0x3d`) for IX and `#` (`0x23`) for PX; module types are 10 and 9. | established | Linux `l1.h`, `module.h` v5.3 |
| IP35 PROM 6.210 maps `#` to PXbrick `0x76` and `=` to IXbrick `0x77` in its I/O-brick naming switch (`0x1fc4b384`, recorded in [cbrick-pimm-psc-codes.md](l1/cbrick-pimm-psc-codes.md)); its name strings include `PXbrick` and `IXbrick` (body offsets `0x7dc00` and `0x88080`). SAL 10.54 `bridge_discover` maps type byte `0x3d` to `0x77` `IXbrick` and `0x23` to `0x76` `PXbrick` (chips `platforms/sn2/klconfig.md`; names at image offsets `0x1b8390` and `0x1b8398`). | established (binary reading); the offsets of the strings were read here | IP35 PROM 6.210; SAL 10.54 |
| The IP35 PROM's ioboard-type acceptance list for a PIC is `0x23`, `0x3d`, `0x6d`, `0x6f`, `0x76`. These are L1 type characters, not board types: `#` (PX), `=` (IX), `m`, `o` and `v` (`L1_BRICKTYPE_CHI_CG`, 0x76). Only as a board type is `0x76` `KLTYPE_PXBRICK`; the two meanings must not be mixed. | established (recorded in chips `devices/pic/README.md`) | SAL 10.54 `pic_attach2` |
| The module identifier letter is a **conflict**. DRV Figure 5-1 (p.52) draws a PX-brick's module ID as `001#14`. Linux `format_module_id` (v5.3 `arch/ia64/sn/kernel/klconflib.c`) maps `L1_BRICKTYPE_PX` to `p` and `L1_BRICKTYPE_IX` to `i`, and its comment says this is the character the brick's LCD shows ("101p17 for a PX-brick"). The driver guide's figure is a drawing, and the same guide's Figure 2-3 was shown above to be mislabelled; the Linux code is a software expectation. What a real brick's LCD shows is unknown. The L1 guide's table (L1G Table 3-1, p.60) lists `c i p r x n g ?` with no IX or PX entry. | conflict: `#` as drawn, `p` (PX) and `i` (IX) per Linux; real LCD unknown | DRV p.52, rendered; Linux v5.3 `klconflib.c`; L1G p.60 |
| The L1 type of an IX-brick comes from a stored setting rather than an identification record on the Altix-era L1 packages: packages 3 and 5 test record `0xfe` against `PXBRICK` (giving `#`) and not against any IX name. | established (recorded in [personalities.md](l1/personalities.md)) | L1 1.62.1 packages 3 and 5 |
| What a PROM prints for `hinv -v`: not found in either PROM. IP35 PROM 6.210 holds the two brick-name tables and a `%s in Module %M %s` line, but the routine that prints a per-brick inventory was not located by string. The Altix EFI boot manager prints a count only (`IOBricks 1` in CAP-3300). Linux's hardware graph names both bricks `Pbrick` (DRV p.54), and IRIX names them by the L1 character, `IXbrick` and `PXbrick` (IRIX 6.5.30 kernel `l1_command.o`, recorded in [personalities.md](l1/personalities.md)). | unknown (PROM print); established (the two hardware-graph names) | as listed |
| A real Origin 3000 or Onyx 3000 IX-brick IRIX `hinv` is not held. The public IRIX listings with `IXbrick` paths are Origin 350 and Onyx 350 modules, where the name is the 2U module's own PIC at widget 15, so the IOC4 appears at `IXbrick/xtalk/15/pci-x/0/1/ioc4`. That naming does not transfer to the three-PIC brick. | established (what the listings print); the non-transfer is inferred | forums.irixnet.org thread 3009 and similar |

## What stays unknown

| Question | What would settle it |
|---|---|
| Whether an Origin 3000 or Onyx 3000 IX-brick puts the IO9 functions at the same devices | a `hinv -vm` or PROM `pci` listing from one |
| Which of XIO 10 and XIO 11 a given C-brick connector reaches, for an IP35 C-brick and for an IP53 node board; and which Altix connector cable runs to which port when only one is fitted | a capture from an IX-brick cabled to one port only, or a `hubii` trace from the PROM |
| The mapping of `Compute I/O port B` and `D` to Node 0 and Node 1 | an `ioport` capture from a two-node C-brick with one I/O brick attached |
| The SCSI channel behind the external VHDCI connector | a bus scan with a device on the external port |
| What the IP35 PROM prints per brick under `hinv -v` | the routine that prints it (not found by string in 6.210), or a console capture |
| The module identifier letter a real PX-brick or IX-brick shows (`#` or `p`; `i`) | a photograph of the LCD, or an L1 `config` capture that lists one |

Not reviewed.
