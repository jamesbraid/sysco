# Origin 300 L1 1.44.0 USB descriptor fields

The selected Origin 300/IP45 L1 image declares a USB 1.00 device descriptor and a single 32-byte configuration tree. This note gives the field layout without publishing identification or string bytes.

The body loads at `0xffe00000`; addresses are first-bank. The identification and string bytes are read by the code and not reproduced here.

## Device descriptor

The first-bank table starts at `0xffe6fd20`. The relocated-bank copy at `0xffedfd20` is byte-identical for 18 bytes. All multi-byte descriptor fields are USB little-endian.

| Byte offset | Field | Declared value | USB 1.0 meaning |
| --- | --- | --- | --- |
| 0 | `bLength` | `18` | Device descriptor length |
| 1 | `bDescriptorType` | `1` | Device descriptor type |
| 2–3 | `bcdUSB` | `0x0100` | USB specification release 1.00 |
| 4 | `bDeviceClass` | `0xff` | Vendor-specific device class |
| 5 | `bDeviceSubClass` | `0xff` | Vendor-specific subclass under that class |
| 6 | `bDeviceProtocol` | `0x00` | No device-level class protocol declared |
| 7 | `bMaxPacketSize0` | `64` | Allowed endpoint-zero packet size |
| 8–9 | `idVendor` | Present, withheld | USB-assigned vendor identification |
| 10–11 | `idProduct` | Present, withheld | Manufacturer-assigned product identification |
| 12–13 | `bcdDevice` | `0x0100` | Device release 1.00 |
| 14 | `iManufacturer` | `1` | Manufacturer string index |
| 15 | `iProduct` | `2` | Product string index |
| 16 | `iSerialNumber` | `3` | Serial string index |
| 17 | `bNumConfigurations` | `1` | One configuration |

The original USB 1.0 device descriptor definition, printed pages 182–184, permits endpoint-zero packet size 64 and gives `0xff` device class its vendor-specific meaning. The class triple alone does not identify an L2 driver.

## Configuration, interface and endpoints

The first-bank tree starts at `0xffe6fd32`. Its `0xffedfd32` copy is byte-identical for 32 bytes. Offsets below are relative to the tree start.

| Offset | Descriptor | Field | Declared value |
| --- | --- | --- | --- |
| 0 | Configuration | `bLength` | `9` |
| 1 | Configuration | `bDescriptorType` | `2` |
| 2–3 | Configuration | `wTotalLength` | `32` |
| 4 | Configuration | `bNumInterfaces` | `1` |
| 5 | Configuration | `bConfigurationValue` | `1` |
| 6 | Configuration | `iConfiguration` | `4` |
| 7 | Configuration | `bmAttributes` | `0x60` |
| 8 | Configuration | `MaxPower` | `0` |
| 9 | Interface | `bLength` | `9` |
| 10 | Interface | `bDescriptorType` | `4` |
| 11 | Interface | `bInterfaceNumber` | `0` |
| 12 | Interface | `bAlternateSetting` | `0` |
| 13 | Interface | `bNumEndpoints` | `2` |
| 14 | Interface | `bInterfaceClass` | `0x00` |
| 15 | Interface | `bInterfaceSubClass` | `0x00` |
| 16 | Interface | `bInterfaceProtocol` | `0xff` |
| 17 | Interface | `iInterface` | `5` |
| 18, 25 | OUT 1, IN 2 | `bLength` | `7`, `7` |
| 19, 26 | OUT 1, IN 2 | `bDescriptorType` | `5`, `5` |
| 20, 27 | OUT 1, IN 2 | `bEndpointAddress` | `0x01`, `0x82` |
| 21, 28 | OUT 1, IN 2 | `bmAttributes` | `0x02`, `0x02` (bulk) |
| 22–23, 29–30 | OUT 1, IN 2 | `wMaxPacketSize` | `64`, `64` |
| 24, 31 | OUT 1, IN 2 | `bInterval` | `0`, `0` |

The component lengths add to 32 and agree with `wTotalLength`. The endpoint address reserved bits are zero. The transfer-attribute reserved bits are zero. USB 1.0 ignores `bInterval` on bulk endpoints, so zero is not a polling promise. The original USB 1.0 configuration, interface and endpoint definitions, printed pages 184–188, assign these field meanings. Interface class zero means no USB-defined interface class. Its protocol `0xff` is described as vendor-specific, but that unusual `0x00/0x00/0xff` combination and the device `0xff/0xff/0x00` combination do not establish a host driver match.

The exact configuration attributes are **`0x60`**. USB 1.0 assigns bit 7 to bus-powered capability, bit 6 to self-powered capability and bit 5 to remote wakeup. This table sets the latter two and clears bus power. The same original definition expresses maximum bus power in 2 mA units, so zero declares no bus-power consumption. Later USB 1.1-derived rules require bit 7 to be set as a reserved marker, which creates a revision-dependent compatibility risk. Neither rule measures the IP45 board's actual power source or proves an L2 host accepted the table. The published L1/L2 guide for Origin 300 establishes the upstream USB slave-port role, not an enumeration result.

## String selection and descriptor delivery

The first-bank string selector at `0xffe6f158`–`0xffe6f1f8` compares the setup language index with two six-entry choices: `0x0009` and `0x0000`. It takes the low byte of descriptor index, rejects indexes at or above six and rejects other language values. The choice `0x0009` is not the common US-English `0x0409`. The meaning of this image's language-zero choice and the contents and byte validity of its six pointed-to descriptor objects were not resolved here. The original USB 1.0 string definition, printed pages 188–189, calls for Unicode string payloads and language IDs in index zero. No protected text or identifier from these objects is transcribed.

The standard read dispatcher reaches the GET_DESCRIPTOR branch for request number six. The descriptor handler at `0xffe6f820`–`0xffe6f920` selects the device table for type one, the full configuration tree for type two and the indexed pointer for type three. Another type returns error. It supplies the table address, its fixed 18- or 32-byte length and the setup packet's advertised `wLength` to the common send helper. The helper at `0xffe6f644`–`0xffe6f6c8` clips the response to the smaller length, issues at most 64 bytes per IN data stage and rejects a short or error result before the generic status-out stage. This is a viable *software path* for the original table bytes to reach the lower controller.

The serializer itself does not force bit 7. It can represent the numeric configuration attribute `0x60` if entered exactly. It cannot preserve this firmware's two-choice language selector or unknown original string payloads through its default behavior. The fixed device/configuration fields alone do not prove that need.
