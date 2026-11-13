# Public L1 USB host-driver evidence

The public SGI L1 USB Control project supplies concrete host transport requirements. Its documented hardware testing used a Fuel.

## Source and scope

Revision read: [`3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74`](https://github.com/srcshelton/sgi-l1-usb-control/tree/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74), release v1.0.68. The host module is GPL-2.0-or-later. The shared ioctl header additionally carries the Linux syscall exception. All source citations below refer to this revision unless stated otherwise.

## Host transport requirements

| Subject | Public software behavior | Exact source |
| --- | --- | --- |
| USB identity | Vendor `0x065e`, product `0x1234` | [the controller firmware header 1, lines 8–9](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/include/sgi_l1_ioctl.h#L8) |
| Interface matching | Accepts class/subclass/protocol `ff/ff/00` or `00/00/ff` with that vendor/product pair | [the controller firmware source unit 67, lines 104–110](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L104) |
| Endpoints | Requires at least two endpoints and finds a bulk IN and bulk OUT in the current alternate setting. Endpoint addresses come from descriptors | [lines 771–829](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L771) |
| Writes | Requires at least two bytes, overwrites those bytes with the total transfer length in big-endian order, then submits one bulk transfer. A short successful transfer returns an error | [lines 382–419](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L382) |
| Transfer limits | Maximum write setting defaults to 4096 bytes, clamped to 2–4096 and the allocated buffer. Buffers are the smaller of 4096 and host page size. Write timeout is 3000 ms | [lines 33–34, 201–210 and 820–829](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L201) |
| Reads | Submits a bulk IN URB and returns its actual bytes. A caller buffer smaller than the completed transfer is rejected. The module does not decode application frames | [lines 137–198 and 293–379](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L293) |

The module does not supply fixed endpoint numbers, endpoint packet sizes, USB speed, device release number, descriptor strings, configuration power attributes or a product-specific descriptor set. Its introductory comment calls the payload IRouter frames and distinguishes the raw transport from a tty. The two-byte USB length prefix must not be substituted for the processor UART's PPP framing. The application-frame structure is not covered here.

## Reset and reconnection

The module defaults to leaving the USB device unreset on close. An optional `reset_on_close` setting queues a USB device reset. Its device-reset ioctl also queues a reset instead of synchronously waiting through unbind. These are host driver choices, not evidence that an L1 controller reset occurred or that a particular machine reconnects successfully. See [the controller firmware source unit 67, lines 79–97, 270–290 and 558–569](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L558).

Read reset cancels the URB and clears local read state. Write reset clears the OUT endpoint halt. Pipe reset clears OUT and IN halts. The optional legacy mode first sets each halt, waits 20 ms and clears it. These sequences specify what a compatible experimental transport would receive. See [lines 480–546 and 579–588](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L480).

Disconnect marks the device absent, sets the pending read status to `ENODEV`, wakes readers and kills its read URB. The completion callback can then replace that status with the URB cancellation status, which an active read can return. Disconnect also removes the device registration and updates the status generation. A later probe creates a new registration. See [lines 137–149, 340–361 and 850–925](https://github.com/srcshelton/sgi-l1-usb-control/blob/3a1fe9532a88ab00e4ee910d7f12ebcb337d5a74/module/sgi_l1_usb.c#L850).

## Hardware evidence and history limits

The repository gives no board revision, descriptor dump, controller marking or L1 firmware version for the tested Fuel, and no raw descriptor or transfer recording.

The module has six commits. The initial commit was [`6e57b3276fd31a15d58cbdd5e8e97317be2e8779`](https://github.com/srcshelton/sgi-l1-usb-control/commit/6e57b3276fd31a15d58cbdd5e8e97317be2e8779) on May 27, 2026. Transport hardening followed in [`02a30a086b7ff263b21a048c14183a16b6fbdb7b`](https://github.com/srcshelton/sgi-l1-usb-control/commit/02a30a086b7ff263b21a048c14183a16b6fbdb7b). The latest module change was [`43d93da3e962719e63527c41f7c2ac7588193f25`](https://github.com/srcshelton/sgi-l1-usb-control/commit/43d93da3e962719e63527c41f7c2ac7588193f25) on May 28, adding optional legacy compatibility. The commit messages contain no transfer transcript or hardware identity.

## Use

The host requirements above (vendor and product pair, interface class triple, two bulk endpoints, big-endian length prefix on each write) are the checks a device model must satisfy to be driven by this module. The L1's descriptors in [usb-descriptor-fields.md](usb-descriptor-fields.md) match them (established by reading the descriptor bytes at `0xffe6fd20`): the vendor and product ids equal the pair above, the interface has class triple `00/00/ff`, and there is one bulk OUT and one bulk IN endpoint. Which silicon sits behind them is a separate question ([usb-controller-registers.md](usb-controller-registers.md)).
