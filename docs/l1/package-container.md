# L1 composite package and Linux inventory

The held IRIX 6.5.30 `l1.bin` is a **container of seven L1 packages**, not one executable image. Its SHA-256 is `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`; its size is 15,988,636 bytes. The count at container offset `0x2c` is seven. Seven big-endian offset and length pairs begin at `0x30`. Recomputing their boundaries shows contiguous ranges, an `ESTF` prefix at each start, and a final end equal to the container size. These are observed package boundaries; the roles of other header fields remain unknown.

| Range | Container offset | Length | SHA-256 of exact range | Observed family |
| --- | ---: | ---: | --- | --- |
| 0 | `0x000000` | `0x0efdd0` | `c391b1c9925c25fa5040d54e9adc51c8cb2ed9b5626a8734938bbc19267e4f08` | Controller package; product unassigned |
| 1 | `0x0efdd0` | `0x0efd8c` | `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4` | Origin 300 candidate selected by independent image review |
| 2 | `0x1dfb5c` | `0x1ac9ac` | `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c` | Controller package; product unassigned |
| 3 | `0x38c508` | `0x1b0c4c` | `40d905ba620ebba0c8259b12410dfe42d21d18cad06477b2873f201059af2fe4` | Controller package; product unassigned |
| 4 | `0x53d154` | `0x3a40fc` | `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6` | Legacy Linux L1 package |
| 5 | `0x8e1250` | `0x1b2458` | `562aa31da2e8d4dfbb94274fb383b078c78c25d84b5c84c747b11ed9d481a68f` | Controller package; product unassigned |
| 6 | `0xa936a8` | `0x4ac0f4` | `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06` | Linux L1 package |

Range 1 contains both occurrences of the product-group label identifying the Fuel/PE/Origin 300 one-megabyte image. The literal ASCII begins at container offsets `0x169824` and `0x1d9824`, together with version `1.44.0`. The review selects this **second** package for Origin 300 format analysis, and explicitly does not clear it as a flash-ready or executable image. Neither of the Linux packages is that selected range.

## Linux package evidence

The following observations go beyond signature matches. Both gzip streams have a deflate method and original filename `kernel`. Each stream decompresses to an ARM kernel image without a gzip checksum error. Each following CramFS has the little-endian magic `0x28cd3d45`, a size that fits inside its package, and a filesystem CRC accepted by `fsck.cramfs -v`. The decompressed kernels contain Linux release banners, and both filesystems contain an init program, startup configuration, MTD utilities and an L1 bootloader. This establishes that ranges 4 and 6 **carry embedded Linux controller software**. It does not establish that either ran on a particular machine or that the package is directly flashable.

| Evidence | Range 4: legacy Linux | Range 6: Linux |
| --- | --- | --- |
| Package label/version | `Legacy Linux L1 image`, `1.44.0` at range-relative `0x3a4034` and `0x3a402c` | `Linux L1 image`, `1.44.0` at `0x4ac034` and `0x4ac02c` |
| Gzip extent, container offsets (end exclusive) | `0x565180` to `0x5f887b`; one-byte gap before filesystem | `0xabb6d4` to `0xb9476f`; one-byte gap before filesystem |
| Decompressed kernel | 1,353,548 bytes; SHA-256 `505b88668c62a285593b038f111a9af8ea2fd42b92d6336ad14e68c679ac82d5` | 1,821,072 bytes; SHA-256 `0d8ed66ca3df4f6115a3fd2f28bfa3d3b5b08adb099fb130dd96d16fd63f6bed` |
| Kernel release/platform | Linux `2.4.25`; ARM software and Samsung S3C2410-family strings; June 9, 2006 build banner at decompressed offset `0x12513c` | Linux `2.6.15.6`; ARM software, an ARM920T string and Samsung S3C2410-family strings; June 9, 2006 build banner at decompressed offset `0x172dd0` |
| CramFS extent, container offsets | `0x5f887c` to `0x8e087c`; 3,047,424 bytes | `0xb94770` to `0xf3d770`; 3,837,952 bytes |
| CramFS SHA-256 | `ab1721002c4ac750582e91b7b516b2e4a903f2ca373000b062bd2710c8faed35` | `1b19eda1b712926e1257dd0e2380d28027b7ae6ad8b4f402c7496e59525e459e` |
| CramFS CRC and file count | `0x45062a7e`; 453 including root | `0x978e34be`; 529 including root |

The gzip header timestamps are June 9, 2006; they do not establish installation dates. The two Linux images are distinct package and software payloads: their Linux releases, kernel hashes, filesystem hashes and explicit legacy/current labels differ. Both contain ARM-platform software, whereas the Origin 300 range is the one-megabyte package examined for ColdFire behavior in the L1 firmware reading.

## Product and revision limits

The container itself identifies ranges 4 and 6 only as Linux L1 images, with software version `1.44.0`; it does not name supported server products or hardware revisions for either package. The embedded root filesystems include hardware-support resources for multiple controller variants, but a resource's presence cannot prove that the whole package supports, was selected for, or ran on a given server. Neither manual maps this held seven-package container to a particular hardware revision. Therefore an Altix-family association is plausible but **unassigned**, and neither Linux range should be substituted for the Origin 300 selected package.

A kernel banner or embedded hardware resource alone would not. The exact image hash and range hashes above allow that evidence to be matched to this composite revision.

## Bounded V-brick package-label followup

A 2026-10-24 reader reverified the complete composite and all seven range hashes above. These results establish no archive-wide absence.

The reader examined the first and final 256 bytes of each package, the two already identified range-1 label windows, and the final 32 KiB of controller ranges 0, 2, 3 and 5. Range 0 contains `Base 1MB image` at relative offset `0xe9b38` and `1.44.0` at `0xea4f3`. Its exact range hash remains `c391b1c9925c25fa5040d54e9adc51c8cb2ed9b5626a8734938bbc19267e4f08`. That label does not identify a supported product or board revision. The other examined windows supplied no explicit V-brick image mapping. They do not establish that the container lacks V-brick support, particularly where contents may be compressed or labels occur elsewhere.

No extraction, disassembly, download or firmware execution occurred. An incidental firmware source-path string was seen and is not transcribed. The product-selection evidence identified above is still needed.

## Updater selection evidence

The ELF is big-endian MIPS N32/MIPS-III.

| File | SHA-256 |
| --- | --- |
| `flashsc` | `bd7652fcf584ee6063bf5f90ab147301c4bd7b122938fadcb026f7aa2dcafd82` |
| `receipt.json` | `3ab5d76c4e4a865b7b0612e5fdb90ba8f7f69b4a65bc153a4e3cd43ce5433573` |
| `selection-observations.json` | `2779ce0e8d7a92331af8cf4f4b2ebe14412c1b42276bbb11c2392069e9ca6016` |

Complete selected paths cover parsing at `0x1000c490`, classification at `0x1000dc40`, their necessary helpers, selection at `0x1000eb40` and `0x1000ec40`, and caller `0x1000ee10`. ELF register metadata and actual GOT entries establish GP `0x10075310`. The parser accepts file-format versions through 5. The held composite's versions 3–5 reach these paths. All inspected image headers are version 4, length 48, with zero word sums. This verifies header checksums, not full payload integrity or flash qualification.

For L1 records, selection scans forward or backward and returns the first matching record. Its family must match. A nonzero requested brick byte must occur in a present supported-brick string. Nonzero requested and package variants must agree. The parser appends `=` when the string contains `#`. Zero byte or absent string bypasses membership checking. A non-L1 first record bypasses this selection loop.

In the examined caller, normal direct integer reply argument 7 supplies the low byte at `0x1000f14c`. Replies with fewer than eleven arguments supply zero. The alternate argument-accessor path remains unexamined. The other two callers were inspected only around their calls. This is not an identification of command 3's type field or an on-wire command.

Pointer conversion uses bank file position plus pointer minus bank virtual address. The updater does not check those string bounds on this path. Literal `0x56` membership occurs only in range 2. That byte is not proof of V-brick identity. An explicit V-brick mapping remains unknown. No execution or flashing occurred.

## Flash-status reply and the image-matching identity byte

It is a firmware-stored runtime identity byte returned by the flash-status request, not a byte read from an image label. It is separate from the flash-layout field and from the identity cache used by the earlier query-3 analysis. This does not establish a VPro V-brick value or change any package assignment above.

### Exact inputs and request closure

The updater remains the 407,920-byte ELF and receipt identified above, SHA-256 `bd7652fcf584ee6063bf5f90ab147301c4bd7b122938fadcb026f7aa2dcafd82`. All 69,407 listed instruction words match the ELF bytes, without duplicate addresses. This is byte accounting, not whole-updater semantic coverage. The actual GP is `0x10075310`. The managed listing's automatic GOT comments use an incorrect assumed GP for this N32 executable. This reading uses the actual GOT and ELF load-segment mapping, not those automatic comments.

The normal L1-family path sets destination task 16 at `0x100088b0–0x1000890c`. The packing at `0x10009ea4–0x10009f00` combines that low-five-bit task with the selected controller class, rack and bay, then passes the destination to `0x1000ee10`. Their byte hashes are `f26f6fbe3bdf84e66409a3d18dbe85a8cdd1572979307ac5f383f6a446e2bb83` and `26577c4c0c7d61c388c67d9eda328f37d50c083f6bdc8c34a711a68ff8091c29`. The updater helper preserves that incoming destination, rather than assigning task 16 itself. The separate family-2 path sets task 8.

At `0x1000ee70–0x1000eed8`, the updater builds two integer arguments, values 0 and 5, and sends request command `0x72`. The normal construction, receive and argument-accessor paths were read through these ranges: `0x100282e0–0x10028558`, `0x10028560–0x100287e8`, `0x10028f40–0x1002911c`, `0x1002bf70–0x1002c04c`, `0x1002f090–0x1002f93c`, `0x1002fdc0–0x10030050`, `0x10030050–0x1003062c`, `0x10030750–0x100309a8`, and `0x100309b0–0x10030d88`. They transport the command and typed fields without translating argument 7 through a product enumeration.

The decoded packet places its 16-bit command at `+0x22`, argument count at `+0x20`, and eight-byte descriptors from `+0x24`. A descriptor carries type flags, packet-chain position, payload length and a value or offset. Zero-based argument 7 therefore has its type at `+0x5c` and value at `+0x60`. The updater requires at least eleven arguments before consuming this field. Fewer arguments select zero and bypass membership checking.

For type low bits equal to zero, the direct path reads the signed integer word. The accessor at `0x100309b0` does the same for an integer descriptor. For other types it returns a payload pointer, assembling fragments where needed. The caller still truncates that pointer at `0x1000f14c`, rather than reading a character from it. The meaningful identity-byte interpretation therefore requires the normal integer schema, not an arbitrary typed reply.

### Selected firmware producer

Its 44-byte package header precedes the body mapped at `0xffe00000`. A total of 1,038 listing rows across the selected producer, identity initializer/accessor and descriptor assembler matched the selected body bytes, with zero mismatches. No unassigned package was disassembled.

The task handler at `0xffe3c124` reads command `+0x22` and dispatches `0x0072` at `0xffe3c150–0xffe3c154` to `0xffe3c166`. It obtains the identity accessor result at `0xffe3c134`, retains it in a register and writes it into descriptor 7 at `0xffe3c392`. The reply schema is:

| Zero-based argument | Selected producer's value |
| --- | --- |
| 0 | Initial validation result plus one |
| 1 | Word reached through flash-state `+0x28`, or zero |
| 2–3 | First image header string pointers at `+0x14` and `+0x10`, or integer zero |
| 4–5 | Second image header string pointers at `+0x14` and `+0x10`, or integer zero |
| 6 | Whether flash-state offsets `+0x08` and `+0x10` differ |
| 7 | Runtime identity-byte accessor result |
| 8 | Flash-state `+0x14` |
| 9–10 | Flash-state `+0x08` and `+0x0c` |
| 11–12 | Optional flash-state `+0x24` and `+0x20`, when both are nonzero |

The flash-state base is `0x30000918`. Argument 8 therefore comes from `0x3000092c`. Argument 7's accessor `0xffe557b4` instead loads byte `0x300002f8` at `0xffe557b8`. Neither is query 3's `0x300008cc` object. The accessor replaces only the low byte of its return register, so this supports the updater's low-byte consumption without claiming that all upper integer bits are cleared.

Reply assembly at `0xffe3c404–0xffe3c410` supplies eleven or thirteen descriptors to `0xffe78e0c`, then `0xffe77eb4`. The latter copies the ordered `count*8` descriptor bytes at `0xffe77fc2–0xffe77fd2`. Integer descriptors have type zero with value at offset four. String fields have type one, NUL-inclusive length and a pointer. This joins the selected producer's integer descriptor 7 to the updater's normal read.

### Identity initialization and remaining product gap

Initializer `0xffe55254` chooses byte `0x300002f8` using identification records obtained through `0xffe6bc04`. If it remains the sentinel `0x3f`, `0xffe55640` reads persistent key 9 through `0xffe59590`. The accepted-value table at `0xffe55664` permits numeric `0x56`, reaching `0xffe556d2`. That path excludes special value `0x34`, then stores the other accepted persistent byte at `0xffe556e0`. A later persistent key `0x10` supplies adjacent byte `0x300002f9`, not argument 7.

This establishes a runtime configured identity namespace. The selected paths contain no explicit VPro V-brick mapping. The updater's family, identity-byte membership and variant comparisons remain separate.

That table labels `0x76` as a CG-family response code. It does not establish a VPro V-brick value or identify the updater's `0x56`. Those bytes must not be assigned products by their printable letters.

Base package 0 remains unassigned, range 1 remains the Origin 300 candidate, and range 2 remains unassigned.
