# Origin 300 L1 1.44.0: USB controller accesses

The selected Origin 300/IP45 L1 image drives a device-side USB controller on the ColdFire bus. The control-request reading covers the USB requests; this note follows the lower byte-register operations that serve them.

## Evidence and limits

Its selected range at composite offset `0xefdd0` has SHA-256 `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4`; the 982,368-byte ColdFire body has SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`. The selected body's reset vector is `0xffe00400` at load address `0xffe00000`. The second firmware bank has corresponding code but is not wholly byte-identical; these claims are tied to the first bank of L1 1.44.0.

External retrieval then located Agere's [USS-820D USB Device Controller data sheet, revision 4, June 2001](https://www.alldatasheet.com/datasheet-pdf/pdf/49927/AGERE/USS820D-DB.html). Its accessible front matter establishes a byte-addressed local-processor register interface, programmable endpoints and FIFOs, integrated D+/D− transceiver, and a firmware-controlled D+ pull-up supply. Neither manual identifies the fitted controller package.

## Startup chooses one of two lower-controller paths

The image has two software controller slots. Its startup checks an existing software count: a positive count attempts slot zero at ColdFire byte base `0x40000000`; count exactly two also attempts slot one at byte base `0x60000000` (`0xffe55198`–`0xffe55252`). Before choosing a driver callback, each attempt writes a test value to base offset `0x1b` and reads byte offset `0x18`. If the revision byte is `0x81`, it calls an unresolved callback pointer held in RAM at `0x300006f4`. Otherwise it enters the visible USS-820-family probe (`0xffe6db6c`–`0xffe6dc08`; disassembly confirms the branch). This choice is material: the visible register sequence below is conditional on **not** reading `0x81`, and the alternative callback's implementation and silicon identity were not recovered from this trace. The software count and two address constants do not prove two fitted devices, package location, or which slot connects to the rear type-B port.

The visible probe accepts revision bytes `0x10`, `0x11`, and `0x13` quietly. Any other byte with high nibble one is accepted with a warning; another high nibble fails. It then writes each even byte from zero through `0xfc` to offset `0x1b` and immediately reads it back, failing on the first mismatch (`0xffe70968`–`0xffe70a98`). The firmware's diagnostic wording calls this path USS-820 and describes `0x18` as a revision and `0x1b` as scratch. That supports the **software's intended controller family**, not a verified IP45 package or precise chip revision. A revision-zero default or a hard-coded `0x13` in an emulator would erase the observed probe behavior.

## Register operations in the visible path

Every lower-controller access in this table is an eight-bit load or store relative to the selected controller base. The labels describe how the firmware uses an offset.

| Offset | Firmware use | Selected first-bank locator |
| --- | --- | --- |
| `0x00`, `0x01` | Write an IN FIFO data byte repeatedly and publish its packet byte count; the data path caps each packet at 64 bytes. | `0xffe71240`–`0xffe71334` |
| `0x03`, `0x04` | Per-selected-endpoint receive/transmit status or FIFO control. The reset path writes both; receive and transmit workers test error and ready bits. | `0xffe70ae0`–`0xffe70b98`, `0xffe70c50`–`0xffe70de8`, `0xffe70fe8`–`0xffe71154` |
| `0x05`, `0x06` | Read an OUT FIFO data byte repeatedly and inspect the available byte count. | `0xffe70c50`–`0xffe70de8`, `0xffe70fe8`–`0xffe71154` |
| `0x08`, `0x09`, `0x0c`, `0x0d` | Per-selected-endpoint transfer control/status, including control-stage and read/write completion decisions. | `0xffe70c50`–`0xffe70de8`, `0xffe70fe8`–`0xffe71334` |
| `0x0a`, `0x0b` | Select a logical endpoint zero through seven and change that endpoint's FIFO allocation/state. Reset gives endpoint zero distinct treatment; configuration changes endpoints one and two. | `0xffe70ae0`–`0xffe70c50` |
| `0x0f`, `0x10`, `0x11` | Reset/connection and pending-address controls; a staged address operation writes `0x10`. | `0xffe70ae0`–`0xffe70b98`, `0xffe713b0`–`0xffe71454` |
| `0x12`, `0x14`, `0x15`, `0x16`, `0x17` | Global status, interrupt latch and enable controls. The interrupt callback tests `0x12` bit zero, masks `0x14` with `0x16`, services endpoint-zero flags in bits zero and one and endpoint-one/two flags in bits three and four, then clears serviced status in `0x14`. | `0xffe70ae0`–`0xffe70c50`, `0xffe7156c`–`0xffe716de` |
| `0x18`, `0x1b` | Probe revision and scratch read-back as described above. | `0xffe6db6c`–`0xffe6dba8`, `0xffe70968`–`0xffe70a98` |
| `0x1a`, `0x1c` | Software brackets endpoint register edits with `0x1a`; reset or disconnect passes may set `0x1c` to zero, later to one. A relation between `0x1c` and the public USS-820 D+ pull-up pin is plausible but not verified from the inaccessible register table or board wiring. | `0xffe70ae0`–`0xffe70c50`, `0xffe7156c`–`0xffe716de` |

The controller-parent ColdFire interrupt-mask write is separately 16-bit at parent base offset `0x36` (`0xffe6e818`–`0xffe6eab8`), after callback and deferred interrupt work are registered. It must not be folded into the USB controller's byte-register map. The image registers distinct low-level interrupt paths for software slots zero and one and shares higher-priority reset, control, read and write workers (`0xffe6e818`–`0xffe6eab8`). Actual interrupt line wiring and polarity need a matching board record.

## Reset, address and endpoints

The visible reset callback disables its active condition, optionally toggles the byte at `0x1c` with a short wait, clears address and global interrupt state, initializes each of eight logical endpoint selections, then enables endpoint-zero state and interrupt bits (`0xffe70ae0`–`0xffe70b98`). A separate operation clears endpoint-zero FIFO state and stalls or resets its transfer state (`0xffe70b98`–`0xffe70bd0`). Configuration one changes the endpoint-one and endpoint-two FIFO allocation fields and resets their per-endpoint state; configuration zero clears them (`0xffe70bd0`–`0xffe70c50`). The firmware declares only EP0 plus bulk OUT 1 and bulk IN 2 in its descriptors; initializing eight internal selections is not evidence of eight external endpoints.

`SET_ADDRESS` makes two lower calls around the zero-byte IN status stage. The first validates that the requested address fits seven bits; the visible callback does not write a register for that call. Only after validation and the status stage succeed does the second call write the address byte at offset `0x10` and update the software address state (`0xffe6f920`–`0xffe6fa78`, `0xffe6d678`–`0xffe6d70c`, `0xffe71454`–`0xffe7156a`). An early address write is therefore a distinguishing error. A failed validation call or status stage prevents the after-status action in the visible control dispatcher. The lower worker's precise bus timing and host-observed completion remain unmeasured.

Endpoint-zero receive processing examines setup/transfer flags and drains the receive FIFO. The interrupt callback separately handles endpoint-zero setup/data activity and endpoint-one/two read/write completion, disabling uninterested enable bits and recording repeated inactive interrupts (`0xffe70c50`–`0xffe71334`, `0xffe7156c`– `0xffe716de`). These are controller-side stages.
