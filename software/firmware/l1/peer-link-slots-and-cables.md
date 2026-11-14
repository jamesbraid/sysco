# L1 1.44.0: which UART channel carries which RS-422 peer link, by brick letter

The L1-to-L1 links of the Origin 3000 bricks (the "peer links") are carried on the quad UART's channels and presented to the packet router as numbered peer slots. This note tabulates, for every brick letter that opens peer slots, which software UART port feeds which router slot, which UART channel that is, and what label the firmware prints for the slot. The earlier [rs422-router.md](rs422-router.md) and [quad-uart-and-serial-links.md](quad-uart-and-serial-links.md) give the slot and port numbers for the Origin 300 only.

Read from L1 1.44.0, package 0 (`Base 1MB image`, C-brick, body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`), package 2 (`MIPS 2MB image`, sha256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`) and package 1 (`Fuel/PE/O300 1MB image`) for the Origin 300; first program copy. Packages 0 and 2 have the same routine bodies at different addresses (package 0 `0xffe2dcc0`, package 2 `0xffe33498`); the table below was decoded in both and the two agree arm for arm. Packages 3 and 5 were not read.

Grades follow [../../../AGENTS.md](../../../AGENTS.md): **established** (the whole table or routine was read), **inferred** (reasoning stated), **unknown**.

## 1. Where the mapping comes from (established)

The peer-link initialiser (packages 0 and 1: dispatch at `0xffe2dcc0`, word jump table `0xffe2dcd6`, routine end `0xffe2e1d4`; package 2: dispatch at `0xffe33498`) clears a table of thirteen router slots (index 0 to 12; the slot entry's device word is at `0x1c + 4 * slot` from the table base `a3`), then dispatches on the brick letter through a word jump table (letters `!` to `Y`, subtract `0x21`) to an arm that stores a software UART port number into some slots. A slot whose word stays `0xff` is closed. The software port to channel rule is fixed by the UART open routine: ports 3, 4, 5, 6 are channels 0, 1, 2, 3 of the quad UART (offsets 0, 8, `0x10`, `0x18`; package 2 `0xffe859d8`, package 0 the same routine).

The slot's label comes from a separate routine (package 0 `0xffe30be4`, package 2 `0xffe363bc`; strings `NIA port`, `CTC port`, `NIB port`, `IIA port`, `CTI port`, `IIB port`, `IIC port`, `IID port`, `local`, `L2`): a switch on the slot with two letter-dependent entries.

| Slot | Label for letters `5`, `@`, `6` | Label for every other letter |
|---:|---|---|
| 1 | `NIA port` | `CTC port` |
| 2 | `NIB port` | `NIB port` |
| 9 | `IIA port` | `CTI port` |
| 10 | `IIB port` | `IIB port` |
| 11 | `IIC port` | `IIC port` |
| 12 | `IID port` | `IID port` |

Other slots print the router's port-address form. The same slots and names appear in the `ctc`, `cti` and `l2` console commands of [power-reset-cbrick.md](power-reset-cbrick.md).

## 2. Port to slot, by letter (established)

The arms, read in full in packages 0 and 2 (the same stores, address operands aside):

| Letters | Slot 1 | Slot 2 | Slot 9 | Slot 10 | Slot 11 | Slot 12 | Notes |
|---|---|---|---|---|---|---|---|
| `C`, `4`, `0`, `7`, `8`, `Q` | port 3 (channel 0) | | port 4 (channel 1) | | | | C-brick, Origin 300 and the other single-node controllers: labels `CTC port` on slot 1 and `CTI port` on slot 9 |
| `I`, `X`, `Y`, `P`, `V`, `E`, `=`, `#` | | | port 3 (channel 0) | port 4 (channel 1) | | | I-brick and its relatives: slot 9 is labelled `CTI port`, slot 10 `IIB port` |
| `%` | port 3 (channel 0) | port 4 (channel 1) | | | | | |
| `&`, `N` | | | port 3 | port 4 | port 5 | port 6 | four peers on all four channels; labels `CTI`, `IIB`, `IIC`, `IID` |
| `6` (Cx-brick) | port 13 | | port 3 | port 5 | port 6 | port 4 | slot 1 `NIA` is on the 3 Mbaud port 13 of the eight-channel UART; slots 9 to 12 (`IIA`, `IIB`, `IIC`, `IID`) use channels 0, 2, 3, 1 of the second window (see [cxbrick-mbus-and-uart-population.md](cxbrick-mbus-and-uart-population.md)) |
| `5`, board revision `1` or `2` | port 3 (channel 0) | port 4 (channel 1) | | | | | the revision accessor (package 0 `0xffe55b64`) is compared with the characters `1` and `2` |
| `5`, any other revision | port 7 | port 11 | port 8 | port 12 | | | ports 7 and above are opened only by the letter-`6` code (and by stubs in the other letters' code), so these slots stay closed (inferred) |
| `!` | port 7 | port 11 | | | | | |
| `O` | port 7 | port 8 | | | | | |
| all other letters | closed | closed | closed | closed | closed | closed | no peer slots |

For the Origin 300 and the C-brick this gives: **UART channel 0 (software port 3) is router slot 1, labelled `CTC port`; UART channel 1 (software port 4) is router slot 9, labelled `CTI port`.** The R-brick (`R`) has no arm and so no peer slot (established; it opens no peer link).

## 3. Reading for the cables

- The label names the link's role, so `CTC` is the C-brick to C-brick link and `CTI` the C-brick to I-brick link (inferred from the letters in the label; no string says so). The I-brick's slot 9 carries the same `CTI` label, so the C-to-I cable joins the C-brick's channel 1 (port 4) to the I-brick's channel 0 (port 3) (inferred from the matching label).
- **This reverses an earlier reading.** A reading of the labels in the order they are printed took channel 0 as the C-to-I link and channel 1 as the C-to-C link. The slot tables show the opposite assignment: channel 0 is slot 1 (`CTC`) and channel 1 is slot 9 (`CTI`). The routing agrees: a request addressed to controller type `4` (the I/O brick) goes to slot 9 ([rs422-router.md](rs422-router.md)).
- Which physical connector on the rear of the brick each channel reaches is not visible to the firmware (unknown). The user guide's cabling text is a hint, not a finding.
- The channel pairing between a C-brick and a Cx-brick is not a fixed pairing in the image: the Cx-brick's `IIA` to `IID` slots take channels 0, 2, 3, 1 of its second window, in the order the arm stores them; which of them meets which C-brick is unknown.

## Not settled

| Item | Why open |
|---|---|
| Connector for each channel | the image uses port selectors only |
| Letters in the empty arm (`" $ ' ( ) * + , - . / 1 2 3 9 : ; < > ? @ A B D F G H J K L M R S T U W`) and packages 3 and 5 | the empty arm opens no slot in packages 0 and 2; packages 3 and 5 were not read |
| The revision accessor for letter `5` | its return values `1`, `2` are compared; what the board reports there was not read |
