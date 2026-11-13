# GIGAchannel cabinet system-controller status word

The GIGAchannel expansion cabinet reports its power and fan state as one 16-bit status word. The
word sits in the widget-8 window of the Crossbow in that cabinet, and the node's IRIX kernel and
BASEIO PROM read it there. The register, field and constant layout is in
[devices/gigachannel_status/registers.toml](../devices/gigachannel_status/registers.toml), with the generated reference at
[devices/gigachannel_status/registers.md](../devices/gigachannel_status/registers.md).

**Unverified.** This page holds one author's readings of the IRIX 6.5.30 kernel and the BASEIO PROM
6.156, made by disassembly. Nobody has re-read them against the images. The images are not committed,
and this repository's ignored `root/` does not hold them. Every item is inferred, because each one is what the software expects the
word to mean. No image here writes the word, so nothing says what sets its bits.

## The word and how it is reached

- **Processor address.** The kernel forms `(nasid << 32) + 0x9200_0000_08e0_0000` for the status
  word, where `nasid` is the node whose Hub reaches the cabinet. That is the uncached Hub I/O window
  for widget 8, at widget offset `0xe0_0000`. Inferred.
- **Width.** Read as a 16-bit value. The kernel's read keeps only the low byte, so bits 8 to 15 do not
  reach any software decision. Inferred.
- **Access and reset.** The images only read the word. What a write would do is unknown, and so is the
  reset value. A read taken after firmware start-up is not a reset value.
- **Crossbow identity.** Each node reads the Crossbow identification word through its Hub, at the
  widget-0 offset in the defs file. The part field, bits 27 to 12, decides whether the node has a
  cabinet. The kernel accepts part zero only. The BASEIO PROM accepts part zero and `0xd000`. How the
  PROM pulls the part value out of the word was not traced.
- **IP27 PROM.** A byte scan of the IP27 PROM 6.156 payload and of 6.11 found no 64-bit address
  literal, no `lui` of the widget-8 high half and no immediate that forms it. The only hit in 6.11
  is a structure offset that is not an address. It is inferred that the IP27 PROM does not read the
  word. A scan cannot rule out an address built some other way.

## Bits as the software reads them

| Bit | Mask | Software's reading | Read by | Grade |
|---|---|---|---|---|
| 0 | `0x01` | Blower failure. The kernel prints `Gigachannel blower failure` on every poll while set, and starts the orderly shutdown once per boot. | IRIX kernel | inferred |
| 1 | `0x02` | Level-4 notice on every poll while set. Text not recorded. | IRIX kernel | inferred |
| 2 | `0x04` | Level-4 notice on every poll while set. Text not recorded. | IRIX kernel | inferred |
| 3 | `0x08` | Level-4 notice on every poll while set. Also starts the orderly shutdown once per boot, the same as bit 0. Text not recorded. | IRIX kernel | inferred |
| 4 | `0x10` | Backup power supply has taken over, redundancy lost. The kernel prints `Backup power supply kicked in - redundancy lost` once per boot, and the BASEIO PROM prints a lost-redundancy line while it is set. | IRIX kernel, BASEIO PROM | inferred |
| 5 | `0x20` | Manufacturing mode. The kernel prints `Manufacturing mode` once per boot. | IRIX kernel | inferred |
| 6 | `0x40` | Redundant supply fitted in the cabinet unit. The BASEIO PROM prints its redundant-supply line while set, and prints nothing for the cabinet supply when clear. The kernel records the inventory item only at start-up. | BASEIO PROM, IRIX kernel | inferred |
| 7 to 15 | `0xff80` | No reading. The kernel keeps only the low byte, and the PROM tests only bits 4 and 6. | none | unknown |

Two things to keep in mind. Bits 0 to 3 repeat their notice on every poll for as long as they stay
set. Bits 4 and 5 are reported once per boot, and the kernel's record for bit 4 never returns to
zero: it never looks at a cleared bit again after start-up, so a loss of redundancy that clears and
returns is not reported a second time.

## What the kernel does with it

The kernel starts one poll thread per cabinet node at start-up. Each thread waits one period, reads
the word and handles the bits above. The period and the shutdown delay are kernel tunables, not
hardware. The poll period reads as one minute at a 100 Hz clock tick. That clock rate is inferred.
When bit 0 or bit 3 first sets, the kernel asks init for an orderly shutdown, and if the system is
still up two minutes later it sends the module-controller reset text command and, if that fails,
writes a warm reset to the Hub port-reset register. The sequence is software behaviour and is only
inferred here. The Hub register's bit meanings are not in this repository.

## Unknowns

| Unknown | Why it matters | What would settle it |
|---|---|---|
| What sets the bits and when | Decides which states the word can reach | The cabinet's own controller firmware, or a capture of the word from a real cabinet |
| Notice text for bits 1, 2 and 3 | Only a test that reads the console can match them | The kernel's notice table, read in full |
| Meaning of bits 7 to 15 | Whether the word carries more than the software reads | A capture from a cabinet in a state that sets them |
| Reset value and writes | Whether the word survives a reset and whether software writes it | A capture after reset, and the write path if one exists |
| Part-value extraction in the PROM | Whether `0xd000` is a field value or a raw word value | Trace of the PROM routine that reads the identification word |

## Sources

- IRIX 6.5.30 IP27 kernel image, SHA-256 `cbdb02ea5d334ecac949a6524a363e6c945f94baf438d41e8d9248eda0377bdf`:
  disassembly of the status poll and the start-up routine, and of the data they read. Software
  expectation.
- BASEIO PROM 6.156, the decompressed payload of the IO6 image, SHA-256
  `bf341a71821541233cb96459f6b99f6f4ecd2cf90f7e9321d918686ee6c25047`: disassembly of the cabinet
  supply line routine. Software expectation.
- IP27 PROM 6.156, SHA-256 `c2c69bf33c375b43ec7ac511fab41ddf595be1b6838ae717e58a9a1273437fdd`, and
  6.11: byte scans for the address forms above. Negative result, inferred.
