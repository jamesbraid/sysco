# Origin 300 IP45 L1 board fitment: public evidence

The published Origin 300 manuals establish the management connectors and the assembly that exposes them. They do not identify the fitted USB device controller, ColdFire part and clock, M-Bus targets, or the electrical wiring between the L1 and L2. These remain board-revision questions. A controller data sheet describes a possible component, not evidence that an IP45 uses it.

| Product and source | Established | Limit |
| --- | --- | --- |
| Origin 300/IP45, [User's Guide 007-4365-001, printed pp. 9, 11–12](https://irix7.com/techpubs/007-4365-001.pdf) | The I/O-8 main board and daughtercard assembly provides L1 functions and rear L1 console and USB slave connectors. A PCI riser joins the I/O-8 assembly to the IP45 board. | The guide gives neither a chip inventory nor a signal map. Its IP45 component list does not locate the L1 chip. |
| Origin 300/IP45, [User's Guide 007-4365-002, printed pp. 10, 12–13](https://irix7.com/techpubs/007-4365-002.pdf) | The revised IP45 component list explicitly includes L1 logic for fans, temperature and LEDs. The I/O-8 assembly also provides L1 functions and the rear L1 USB connector. | The broad functional descriptions do not resolve which individual L1 chips are on each board or the path through the riser. The revision differs from -001, so its placement claim must not silently be applied to every assembly. |
| Origin 300 with NUMAlink module, [guide 007-4498-002, printed pp. 127, 137 and appendix B](https://irix7.com/techpubs/007-4498-002.pdf) | The I/O-8 PCA, daughtercard and riser form the base-module assembly. A four-port USB hub connects the L2 to base modules on NUMAlink-module ports 1, 6, 7 and 8. | This is a cable topology for the named product. It does not specify internal L1-to-USB-controller wiring or identify a controller revision. |
| Origin 300/IP45, [L3 controller 1.8 update guide 007-4364-008, printed p. 9](https://www.infania.net/misc1/sgi_techpubs/techpubs/007-4364-008.pdf) | SGI corrected an IP45 power-on voltage ramp issue and padded USB flashing to handle bricks with older USB parts. | The note establishes hardware variation relevant to firmware behavior, but gives no part number or board revision. |
| Origin 300, [owner inventory with IP45 030-1797-001 rev B and I/O-8 030-1673-003 rev E](https://nekonomicon.irixnet.org/forum/14/11542/1.html) | An assembled machine had these revision-labeled boards. | A software inventory is not a component list. Its USB controller type OHCI is the guest USB host, not the L1 Type B device port. |
| Origin 300 I/O-8, [public photo of 030-1673-003](https://4dwm.com/partnumbers/images/030-1673-003.jpg) and [daughtercard photo of 030-1701-004 rev A](https://4dwm.com/partnumbers/images/030-1701-004.jpg) | The assembled I/O-8 photo shows the rear L1 PORT Type B connector beside guest USB 1/2. The daughtercard photo has a legible assembly label. | Both available photos show the board underside. Neither has readable silicon markings, oscillator value, sensor addresses, traces or pull-up placement. The daughtercard photo is not shown fitted to the owner's rev E board. |
| Origin 3400 R-brick, [collector's hardware account](https://bukosek.si/hardware/collection/sgi-origin3400.html) | The account reports a 54 MHz MCF5206E and DS1780E monitoring parts for Origin 3000-family hardware. | This is an R-brick, not an Origin 300 IP45/I/O-8 revision witness. The clock and sensor part must not transfer to IP45. |

Public Origin 300 guides name monitored fan speed, supply state and operating temperature, but give no M-Bus external-target inventory or addresses. The [DS1780 product sheet](https://www.analog.com/en/products/ds1780.html) shows that a sensor address can depend on board straps. It cannot fix an IP45 address without fitment and wiring evidence. The published connector tables fix USB D+ and D- pins, but say nothing about the device-side pull-up or its controller.

## Search bounds

External public searches then used these exact groups:

- `SGI Origin 300 IP45 L1 board photo USB controller ColdFire M-Bus`, `SGI
  Origin 300 service manual IP45 motherboard IO8 L1 USB schematic`, and
  `Origin 300 IP45 board 030-1779 L1 controller chip markings photograph`.
- `"030-1673-003" "ColdFire"`, `"030-1701-004" USB controller`, `"IP45"
  "M-bus" SGI`, and `"SGI Origin 300" schematic I/O-8`.
- `"Origin 300" "field service" "L1" "IO8"`, `"Origin 300" "service
  manual" "L1" controller`, `"Origin 300" "FRU" "L1" "IP45"`, and
  `"Origin 300" "parts list" "L1" controller`.
- `SGI Origin 300 FCC ID IP45 internal photos` and `"Origin 300" "FCC"
  "internal photos"`.

These searches surfaced user guides, operator logs, board-level sales/FRU listings and unrelated modern FCC records. No returned IP45-revision schematic, component-side photograph with readable L1 chip markings, component-level FRU list, or relevant FCC internal-photo exhibit identified the fitted chips. The [HPE Origin 300 service-parts listing](https://store.emprgroup.com.au/hpe-960043545-6-sgi-sr-origin300-6-1-server-q4k81a.aspx) names assemblies such as 030-1797-001 and L1 display parts, not the fitted L1 USB or sensor silicon.

A revision-matched I/O-8 and IP45 schematic or component-placement drawing would settle chip fitment, bus addresses, oscillator value, USB D+ pull-up and board-to-board wiring. A legible component-side photograph of both boards with the installed assembly labels would settle part markings, but not hidden traces or address straps. USB descriptors or another product's board cannot settle electrical fitment on the named IP45 revision.
