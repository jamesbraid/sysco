# Origin 300 L1 1.44.0 M-Bus and EEPROM reading

The Origin 300 L1 controller uses the MCF5206e's on-chip M-Bus master to read its board and memory identity devices. This matters when a board record is absent or invalid. A guest-visible error can reflect an earlier controller read or checksum failure, not a failed transaction made by that guest request.

## Sources and scope

The selected packaged range at composite offset `0xefdd0` is 982,412 bytes, SHA-256 `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4`. Its 44-byte header names load address `0xffe00000`. The body carries version 1.44.0 and build stamp 2006-07-17 18:20:38. Address ranges below locate observations in the first flash-bank copy. The active bank and equivalence of the second copy remain unknown. Image selection and firmware provenance document extraction.

Its manifest binds the same composite and selected-range hashes above to load address `0xffe00000` and body SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`. The decompiled analysis SHA-256 is `fa4071f8794f5928d594e8538e97b7fb5e89bc3ee0726f9b4c2c67fa26aa86eb`.

Chapter 13, printed pp 13-3 to 13-13, defines the two-wire protocol and registers. No external retrieval was needed.

SGI, *SGI Origin 300 System with NUMAlink Module User's Guide* `007-4498-002`, printed pp 181-182, shows separate NODE and IO8 EEPROM areas. Neither source identifies an EEPROM package or PCB net on the particular board being modeled.

## Controller programming and transaction

The first-bank controller setup at `0xffe6caa8–0xffe6cb3c` clears M-Bus control, writes frequency-divider value `0x13`, clears its own slave address and enables the module. The MCF5206e register offsets are `MBAR+0x1e0` address, `+0x1e4` divider, `+0x1e8` control, `+0x1ec` status and `+0x1f0` data (manual Table 13-1, printed p 13-6). Table 13-2 gives divider 480 for `0x13`. The actual L1 processor clock was not established, so an absolute SCL rate is unknown. The manual's example rate assumes 33 MHz. The errata makes clear that this example must not be substituted for a measured controller clock.

The transaction path at `0xffe6cca4–0xffe6cd94` waits for bus-free status, then writes control `0xf0` (enable, interrupt, master, transmit) and the 8-bit slave address to the data register. The interrupt continuation at `0xffe6cfac–0xffe6d1be` checks completion, arbitration and received ACK, writes subsequent bytes, and for a read changes control to `0xf4` for repeated START before sending the read address. It switches to receive mode `0xe0`, performs the required dummy data read, sets `0xe8` before the final received byte to suppress its ACK, and returns control to `0x80` to stop. These control values match MBCR fields and the master receive sequence in the manual, printed pp 13-8 to 13-13. The image handles transfers through an interrupt-driven continuation and suspends its caller until completion. The separate controller task and interrupt registration are at `0xffe6cb48–0xffe6cc10` and `0xffe6d1c8–0xffe6d23c`. These observations are established from the image.

Bus arbitration is serialized with a software lock and a board-specific bus selection callback (`0xffe6c8d4–0xffe6ca28`). Bus-free waiting counts at most 1,000 status reads. A stuck-busy condition invokes the bus reset path and returns an error (`0xffe6cc70–0xffe6cca4`). The reset sequence itself makes at most 1,000 attempts to bring the controller out of busy status (`0xffe6caa8–0xffe6cb3c`). The status path distinguishes no completion, lost arbitration and missing slave ACK (`0xffe6cfac–0xffe6d030`). These observations are established from the image. Which errors trigger a *new complete transaction* at a higher layer was not settled in this reading.

## Origin 300 device population and record loading

The IP45 board path at `0xffe45990–0xffe45a70` registers six EEPROM records from `0xffe45dcc–0xffe45e0c`. General request component indexes address these records in registration order. The default mapper returns the index unchanged (`0xffe11cd4–0xffe11d92`, `0xffe12570–0xffe12620`). These are established from the first-bank table and registration path.

| Component | Logical record | Controller route in image | Capacity configured in image |
|---|---|---|---:|
| 0 | node board | bus 0, 8-bit address `0xa8` (7-bit `0x54`) | 512 bytes |
| 1 | IO8 board | bus 0, 8-bit address `0xa4` (7-bit `0x52`) | 512 bytes |
| 2 | memory module 0 | bus selection 6, address `0xa0` (7-bit `0x50`) | 512 bytes |
| 3 | memory module 2 | bus selection 7, address `0xa0` | 512 bytes |
| 4 | memory module 1 | bus selection 8, address `0xa0` | 512 bytes |
| 5 | memory module 3 | bus selection 9, address `0xa0` | 512 bytes |

The six logical rows do not prove six fitted EEPROM packages. The four memory routes select separate paths through the DIMM bus multiplexer (`0xffe457fc–0xffe4597e`). The fitted multiplexer part and electrical channels are unknown. The generic reader at `0xffe6b03c–0xffe6b14c` limits a low-level read to 32 bytes and a 256-byte EEPROM address page. It puts upper offset bits into the 8-bit slave-address form and sends the lower offset byte before the receive phase. That establishes the firmware's storage-address convention.

Record registration allocates a presence array, initially marked present, and a 120-byte decoded record area per entry (`0xffe6ac90–0xffe6ad5e`). On the normal IP45 startup path, the main boot task identifies the board and invokes its board-registration callback before starting the general request task (`0xffe10570–0xffe10650`, `0xffe55254–0xffe557a2`, `0xffe79968–0xffe799c8`, `0xffe45bac–0xffe45bbf`). Registration first probes memory records 2 through 5 with eight-byte reads, disabling each record whose probe fails, then reads and checks the node record in a temporary area (`0xffe451fc–0xffe45330`, `0xffe45990–0xffe45a70`). It then calls a shared scan over all six registered entries before later board initialization (`0xffe4a48c–0xffe4a4b0`, `0xffe6bcac–0xffe6bcea`). The scan clears each decoded area and loads each still-present record from the two-wire reader (`0xffe6bc04–0xffe6bcaa`). Thus the IO8 board area is first loaded if its header and area checks pass, or given nonzero stored status on failure, during this startup scan. Successfully probed DIMM routes read 160-byte SPD data then; a valid checksum keeps the RAM copy, while failure stores invalid status and discards it. The node board is read in the earlier temporary check and again by the shared scan.

The verified decompilation shows three direct callers of the shared scan, including the IP45 board path; the other two belong to different board registrations. It shows the direct IP45 call sites of the per-entry loader in the temporary node check and shared scan. Other direct calls of that loader belong to diagnostics, generic board identification or other board paths. The task-6 `0x0006` handler does not call the two-wire reader (`0xffe12526–0xffe12844`). A diagnostic path can read a selected EEPROM live (`0xffe395f4–0xffe39670`).

Board records have an eight-byte common header, chassis area, board information area and internal-use area. Their reader checks the header before allocating and checking the areas (`0xffe6ba58–0xffe6bbf8`). A common-header read failure stores read-error status for the header and areas; a common-header checksum failure stores checksum-error status for the header and leaves the unread areas in their initial unavailable state. Neither failure reads BIA or CIA bytes. Once the header passes, the reader checks each declared area separately; an area checksum failure marks that area invalid and discards its allocated bytes (`0xffe6b89c–0xffe6ba54`). This is established from the image. The separately board-record evidence settles the low-byte sum-zero rule for the common header and loaded nonempty board areas. This call-path reading focuses on when those statuses reach task 6.

The memory reader allocates 160 bytes, reads the SPD data and checks byte 63 against the sum of bytes 0 through 62 modulo 256 (`0xffe6b1e8–0xffe6b2e0`). A failed check marks the memory area invalid and discards its RAM copy. The configured 512-byte route capacity does not mean that all 512 bytes are checksum-validated, or that SPD area size is 512 bytes.

## What task 6 supplies to the host processor

Request `0x0006` takes component, area, offset and desired length. Area 0 selects the eight-byte header, 1 the chassis area (CIA), 2 the board information area (BIA), 3 the internal-use area (IUA), and 4 the memory SPD area. Component presence is checked before area and offset bounds. Successful nonzero reads return the number of bytes remaining after this chunk as an integer, followed by the copied binary slice. An absent component or wrong record kind returns minus 104. An out-of-range area or offset returns minus 105. An area with nonzero stored status returns minus 107. The handler reads pointers in controller RAM rather than an I²C register (`0xffe12526–0xffe12844`). The first-bank branch order establishes the following precedence for four valid integer arguments:

| Competing conditions | Reply and first deciding check |
|---|---|
| Presence flag clear or component outside the registered table, even with bad area or offset | Minus 104; presence precedes range and area checks (`0xffe125ec–0xffe12620`). |
| Present record of the wrong kind for a valid requested area | Minus 104; kind precedes that area's status (`0xffe12646–0xffe127a6`). |
| Bad common-header checksum on a board record; request for its header, CIA or BIA | Minus 107; the header's stored status is checksum-failed, while unread area statuses remain unavailable. The area status check precedes offset bounds (`0xffe6ba58–0xffe6bbf8`, `0xffe12646–0xffe127da`). |
| Good header but bad checksum in one declared board area | Minus 107 for that area; another area can still succeed if its own status is clear (`0xffe6b89c–0xffe6ba54`, `0xffe12646–0xffe127da`). |
| Invalid area number for a present component, or an offset at or past a valid area's size after its status passes | Minus 105 (`0xffe12620–0xffe127c4`, `0xffe127da–0xffe127ea`). |

Minus 107 does not uniquely mean a failed checksum: an unread or absent declared area, a read failure, or an allocation failure can also leave nonzero status. In particular, the IO8 route has no explicit IP45 probe before the shared scan. A failed first read in that scan can leave the initialized presence flag set and store read-error status, yielding minus 107 for a valid BIA request unless a later path clears presence. The earlier DIMM probes do clear presence on failed reads, yielding minus 104 for those failed routes.

The stock IP35 PROM uses the L1 request path for node and IO8 board discovery and 64-byte BIA chunks. It takes its production IOC3 Ethernet address from the IO8 board area, as detailed in the read-only IO8 investigation named above and L1 consumer table. OpenBSD uses area 4 for memory-module SPD. The printed Origin 300 example puts a distinct ASCII board identifier and 12-hex-digit Ethernet address in IO8 BIA (`007-4498-002`, printed p 182). The L1 firmware is a producer of raw area bytes. Neither `0x0006` nor the board-record table manufactures a fallback Ethernet address.

## Alternative readings and evidence that would decide them

| Question | Plausible readings | Discriminating evidence |
|---|---|---|
| Can IO8 BIA be refreshed after startup? | The normal IP45 startup scan loads or marks it invalid before the general request task; an indirect or later path might refresh it. | Trace indirect references to the per-entry loader and capture M-Bus traffic across a later controller-side diagnostic or reboot. A host request trace alone cannot prove a refresh read. |
| What does a negative `0x0006` reply mean? | Minus 104 can follow a disabled memory probe or absent component; minus 107 can follow an invalid stored area, including an IO8 first-read failure or checksum failure. Neither proves fitted population. | Correlate startup M-Bus status/error logs with a bus capture and a matching physical-board inventory. A reply code alone cannot separate the physical causes. |
| Is the IO8 Ethernet address held in an I²C EEPROM or one-wire device? | The L1 IO8 BIA path uses its registered I²C record. Another route could also exist on the PCB. | A revision-matched IO8 schematic, legible component-side photograph with part marking and net trace, or controlled electrical capture. A PROM-formatted identity field and XBridge register names do not settle fitted population. |
| Does a transient M-Bus error retry the entire transaction? | The controller may only reset and report failure, or a higher caller may issue a new attempt. | Follow every failure return from `0xffe6cca4–0xffe6d030` into the IP45 record reader and count start/address sequences in a controlled NACK or arbitration capture. The 1,000-count polling loops are not transaction-retry counts. |
