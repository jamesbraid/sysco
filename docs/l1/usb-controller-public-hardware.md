# Origin 300 L1 USB device-controller fitment

This note covers an Origin 300 base chassis with an IP45 motherboard and I/O-8 assembly. The public sources examined do not identify the L1 USB device-controller chip, its silicon revision, or the number of such chips fitted to either board. A connector count is not an integrated-circuit count.

SGI places the L1 controller logic among the IP45 motherboard components. It also says that the I/O-8 assembly supplies the rear connectors and L1 controller functions. Each base chassis has one four-pin USB L1 connector beside two ordinary USB connectors. The later revision distinguishes the USB L1 port from the two other ports. These statements establish one externally available L1 USB device connection per base chassis. They do not establish which board holds the USB device-controller chip or whether one or several packages implement it.

An [Origin 300 software inventory](https://nekonomicon.irixnet.org/forum/14/16726611/1.html) lists a PCI USB host controller. The two ordinary USB ports are the likely host-facing ports, an inference from their purpose and the separate slave connection. The host controller's PCI identity cannot identify the L1's USB device-controller silicon.

SGI's L3 Controller 1.8 update guide mentions a workaround for flashing bricks with older USB parts. The guide does not name those parts or say which Origin 300 I/O-8 revision, if any, used them. It cautions against treating one possible chip as a universal fitment. [SGI Origin 3000 Series L3 Controller 1.8 Update Guide, 007-4364-008, printed page 9](https://www.infania.net/misc1/sgi_techpubs/techpubs/007-4364-008.pdf).

Public web searches for I/O-8 `030-1673-003`, IP45 `030-1779`, and candidate names `PDIUSBD12`, `ISP1581`, `USBN9604`, `ISP1362`, and `USB1T11` did not yield a board photograph with both readable chip markings and a matching assembly revision. A public L1 USB host driver documents transport and a USB identifier. Those are software-visible values rather than package markings. [SGI L1 USB Control driver](https://github.com/srcshelton/sgi-l1-usb-control).

The public host-driver investigation records the USB match rules, bulk framing and host reset/disconnect behavior.

To settle the fitment, obtain a published I/O-8/IP45 bill of materials or schematic tied to an assembly and revision. A clear photograph of both faces of a matching board could also show its part/revision label and every L1 USB-related chip marking. Traced or documented net connectivity from the type-B L1 connector to those packages would distinguish a device-controller chip from a USB transceiver, hub, or the ordinary host controller. Compare more than one assembly revision before claiming a product-wide chip identity or package count.
