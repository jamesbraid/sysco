# IRIX files needed for this repository

This lists the proprietary SGI files a reader copies from their own IRIX media or patches into a local `root/` directory. They are not tracked in source control. Paths only: this repository holds no file contents.

All controller firmware ships in one inst product, `field_diags_sysco`, which installs under `/usr/cpu/firmware/sysco` and `/usr/sbin`. `root/` holds one directory per source, each mirroring the install paths: `root/6.5.30/`, `root/6.5.22/` and `root/patches/SGnnnnnnn/`. `scripts/inventory-root.sha256` pins the SHA-256 of every file the tables are measured from.

Files wanted from each source (install path inside the source directory):

```files
/usr/cpu/firmware/sysco/l1.bin
/usr/cpu/firmware/sysco/l2.bin
/usr/cpu/firmware/sysco/pbay.bin
/usr/cpu/firmware/sysco/*.svf
/usr/sbin/flashsc
/usr/sbin/sysco/flashsc
/usr/sbin/l1cmd
/usr/sbin/l2cmd
/usr/sbin/flashpbay
/usr/sbin/flashsvf
```

The other controller families (the Challenge and Onyx system controller, the MSC and ELSC, the MMSC; see [docs/controllers.md](docs/controllers.md)) ship in the base `eoe` product of each release, not in `field_diags_sysco`. Files wanted from those sources, same install paths:

```files
/usr/cpu/firmware/mmscfw.bin
/usr/cpu/firmware/l1.bin
/usr/cpu/firmware/l2.bin
/usr/sbin/flashmmsc
/usr/sbin/flash
/usr/etc/mmscd
/usr/etc/sysctlrd
/sbin/scmsg
/usr/cpu/sysgen/IP27boot/elsc.o
/usr/cpu/sysgen/IP19boot/sysctlr.o
/usr/cpu/sysgen/IP21boot/sysctlr.o
/usr/cpu/sysgen/IP25boot/sysctlr.o
/usr/include/sys/EVEREST/sysctlr.h
/usr/share/catman/a_man/cat1/mmscd.z
/usr/share/catman/a_man/cat1/flashmmsc.z
/usr/share/catman/a_man/cat1/flash.z
/usr/share/catman/a_man/cat1/sysctlrd.z
/usr/share/catman/a_man/cat7/sysctlr.z
```

## Sources

| Directory in `root/` | Media | Notes |
|---|---|---|
| `6.5.30/` | the retail IRIX 6.5.30 overlays, product `field_diags_sysco` (the `field_diags_sysco.sw` image and its `.idb` on Overlays 3 of 3) | L1 1.44.0, L2 1.44.0. `flashsc` is installed as `/usr/sbin/sysco/flashsc` (this product's idb has no `/usr/sbin/flashsc` link); the `l1cmd`, `l2cmd`, `flashpbay` and `flashsvf` tools sit in `/usr/sbin`. The `.svf` files are CPLD programming vectors, not controller firmware. |
| `6.5.20/` | IRIX 6.5.20 Overlays (4 of 4), product `field_diags_sysco` | L1 1.20.6, L2 1.20.1. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.21/` | IRIX 6.5.21 Overlays (4 of 4), product `field_diags_sysco` | L1 1.22.2, L2 1.22.0. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.22/` | IRIX 6.5.22 Overlays (3 of 3), product `field_diags_sysco` | L1 1.24.8, L2 1.24.1. `/usr/sbin/flashsc` is a link to `sysco/flashsc`. The only `.svf` files are `ioc4.svf` and `is1.svf`. |
| `6.5.23/` | IRIX 6.5.23 Overlays (3 of 3), product `field_diags_sysco` | L1 1.26.5, L2 1.26.0. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.24/` | IRIX 6.5.24 Overlays (3 of 3), product `field_diags_sysco` | L1 1.28.3, L2 1.28.1. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.25/` | IRIX 6.5.25 Overlays (3 of 3), product `field_diags_sysco` | L1 1.30.6, L2 1.30.0. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.26/` | IRIX 6.5.26 Overlays (3 of 3), product `field_diags_sysco` | L1 1.32.6, L2 1.32.2. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.27/` | IRIX 6.5.27 Overlays (3 of 3), product `field_diags_sysco` | L1 1.34.2, L2 1.34.0. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.28/` | IRIX 6.5.28 Overlays (3 of 3), product `field_diags_sysco` | L1 1.38.4, L2 1.38.8. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.5.29/` | IRIX 6.5.29 Overlays (3 of 3), product `field_diags_sysco` | L1 1.40.5, L2 1.40.1. The `flashsc` link under `/usr/sbin` comes from the `eoe` product on the release's Installation Tools disc, not from this product. |
| `6.4/` | IRIX 6.4 | `mmscfw.bin` (no version string), `elsc.o`, `mmscd`, `flashmmsc`, man pages |
| `6.4-origin/` | IRIX 6.4 for Origin200, Origin2000, Onyx2IR | `mmscfw.bin`, `elsc.o`, `mmscd`, `flashmmsc`, `sysctlrd`, `sysctlr.o` for IP19, IP21 and IP25, `sysctlr.h`, `scmsg` |
| `6.5/` | IRIX 6.5 Foundation 1 | `mmscfw.bin`, `elsc.o`, `mmscd`, `flashmmsc`, `sysctlrd`, the three `sysctlr.o`, `scmsg`, man pages |
| `6.5.2/` | IRIX 6.5.2 Installation Tools and Overlays (1 of 2) | `mmscfw.bin` (no version string), `elsc.o` |
| `6.5.5/` | IRIX 6.5.5 Installation Tools and Overlays (1 of 2) | `mmscfw.bin` 2.1, `elsc.o`, `mmscd`, `flashmmsc`, man page |
| `6.5.6/` | IRIX 6.5.6 Installation Tools and Overlays (1 of 2) | `mmscfw.bin` 2.2, `elsc.o`, `mmscd`, `flashmmsc` |
| `6.5.7/` | IRIX 6.5.7 Installation Tools and Overlays (1 of 2) | `mmscfw.bin` 2.3, `elsc.o`, `mmscd`, `flashmmsc`, man page |
| `6.5.9/` | IRIX 6.5.9 Installation Tools and Overlays (1 of 3) | `l1.bin` 1.0.2 and `l2.bin` 1.0.0 (one image per controller, before the sysco product), `mmscfw.bin`, `elsc.o`, `flash`, `flashsc` |
| `6.5.10/` | IRIX 6.5.10 Installation Tools and Overlays (1 of 3) | `l1.bin` 1.1.6 and `l2.bin` 1.1.13 (6.5.11 is byte-identical), `mmscfw.bin`, `elsc.o`, `flash`, `flashsc`, man pages |
| `6.5.12/` to `6.5.19/` | IRIX 6.5.12 to 6.5.19 Overlays (last disc of each), product `field_diags_sysco` | L1 and L2 1.4.x to 1.18.x (version table in [docs/controllers.md](docs/controllers.md)); `pbay.bin` and `flashpbay` from 6.5.14 |
| `6.5.30/` | as above, plus the base `eoe` product on IRIX 6.5.30 Installation Tools and Overlays (1 of 3) | adds `mmscfw.bin` 2.3, `elsc.o`, `mmscd`, `flashmmsc`, `flash`, `sysctlrd`, man pages |
| `patches/SG0005762/` | patch SG0005762, subsystem `patchSG0005762.field_diags_sysco_sw` | L1 1.32.15, L2 1.32.4. The patch's own release note says L1 1.32.14. |
| `patches/SG0002378/` | patch SG0002378 (Origin 2000 and Onyx2 MMSC firmware update, September 1997), subsystem `patchSG0002378.eoe_sw`, as published in the IRIX 6.4 patch directory of SGI's former public FTP server | `mmscfw.bin`, 1,005,275 bytes, a build of 20 August 1997 (the release note says it fixes MMSC firmware 1.1). Only the patch's own `mmscfw.bin`; the image carries no version string. |
| `cd-ist-3.24/` | SGI Internal Support Tools 3.24 for Linux (`CD-IST_3.24.tar.gz`, 67,461,003 bytes, SHA-256 `10441afaa94787ecfaf597829eae4f0e02e1007682a04fe98704f404719425aa`), the L3 controller and SGIconsole disc of the SGI 3000 and Altix family, as published by Graphica (graphica.com.au/files) | `snxsc_firmware-1.62.1-1.i386.rpm` unpacked to `usr/cpu/firmware/sysco/` (`l1.bin` 1.62.1, `l2.bin` 1.62.0, and the CPLD vector files `ab_lfprom`, `ab_ufprom`, `athena` and `is3`, not seen in any IRIX set); the versioned file names in the RPM are `l1-1.62.1.bin`, `l2-1.62.0.bin`, `pbay-00.18.bin` and `l1_mips-1.48.1.bin`; the last two and the other vector files are byte-identical to files already held. Linux i386 host tools of the same RPM in `linux-host-tools/` (`flashsc`, `flashpbay`, `l1cmd`, `l2cmd`, `flashsvf`, `flashfile`, `flashioc4`). The other packages of the disc are kept whole in `CD/`: `snxsc_l3` (the L3 software: `l2`, `l2cmd`, `l2find`, `l2term`, `libsnxsc`), `snxscan`, `sscope`, `sysco_tools` (a Perl 5.6.1 runtime), the `sgil1` USB driver source and the `install` script. A second copy of the archive on another public mirror is truncated at 26 MB and lacks these. |
| `patches/SG0007149/` | patch SG0007149, subsystem `patchSG0007149.field_diags_sysco_sw` | L1 1.48.0 and 1.48.1, L2 1.48.0. Replaces SG0005762. |

Patch SG0007114 (`ws_field_diags_irix_sw`) ships host-side diagnostic programs and no controller firmware; nothing from it is needed.

Hashes of the carriers these were read from:

```
d2899e9aac3f6f62a446d881ccc2f214afc181c195272ab0e658ca974528557f  IRIX 6.5.22 Overlays (3 of 3).iso
e1ece01e9645be6f5c4613e7de4803589dec79ae8a3ee3ce3da799b5f3e1f2a8  IRIX 6.5.20 Overlays (4 of 4).iso
76337025e3d50c0a0d864699ca2d3ad5d928dd9ec943be1eb040990e3c549109  IRIX 6.5.21 Overlays (4 of 4).iso
1d2affd3f581a7c4fe28a177b1a6471efb9d10e5f6676eddd199bb7df3527dc5  IRIX 6.5.23 Overlays (3 of 3).iso
f8c36e377b5733e04633acd3309dd79da626c5ddc7800f57ca4710bf8e662ec1  IRIX 6.5.24 Overlays (3 of 3).iso
6ae90eb577c3f2374a680c652b872292ac5cac1f22050be3de4ffa0423b4a676  IRIX 6.5.25 Overlays (3 of 3).iso
56b749ab3ca32e4e4faa0d2df67c3affe71ac220f9f8b1b30f26e8f68a5ee205  IRIX 6.5.26 Overlays (3 of 3).iso
574301e7d84a3b67e9a988000d88865de784c1f0d9cc01d6db45a5d8561a678d  IRIX 6.5.27 Overlays (3 of 3).iso
2bb622a160da6a0a2825f3221545225a2bf3ec0c59cadffa76b4ca9c172b0e09  IRIX 6.5.28 Overlays (3 of 3).iso
f7df5880c6b83a6b0f0f2cea02de159c713decc063b3b16df302f8990153bf57  IRIX 6.5.29 Overlays (3 of 3).iso
54b60017f395f824138e6164f7c19f440626d10d23e449a9c55f95f2a4c24473  patch SG0005762 tardist
ebb0e6e71c196d7fec6844a06c2a0024de0c785810212bcfb5a5f987b0221195  patch SG0002378 tar
f73660c9e85b1dbd615856377e8480a4c8a39f76fc86d21cc23533b0787acd29  patch SG0007114 tardist
9f4b8abf97c0903f697f75888ae1363b19e385a85c9d34446973975eecf9930a  patch SG0007149 tardist
```

The 6.5.30 image is `field_diags_sysco.sw`, 21,355,926 bytes; its idb records `l1.bin` at 15,988,636 bytes and `l2.bin` at 2,924,652 bytes, which the extracted files match.

Carriers of the controller files added later, as SHA-1 digests published with the disc images by the mirror they came from:

```
36e6e78c1a21ef0b7242994f7297a40f2686dc28  IRIX 6.4.iso
be516db533461b83a2ff434a0c84657ffdbef199  IRIX 6.4 for Origin200, Origin2000, Onyx2IR.iso
c87e57b0d64733f76f73679b25cdd596d79a5302  IRIX 6.5 Foundation 1.iso
0533da57374f2fd82bdfdcf6ad50a461e5a0b79f  IRIX 6.5.2 Installation Tools and Overlays (1 of 2).iso
5d5cfac3254461c7583e7b8be176b404d3c12697  IRIX 6.5.5 Installation Tools and Overlays (1 of 2).iso
f1db7bdc1734686483cb247504a66434a5969919  IRIX 6.5.6 Installation Tools and Overlays (1 of 2).iso
4195c88109887248c6b06a2d3feb868aa6256e5c  IRIX 6.5.7 Installation Tools and Overlays (1 of 2).iso
c0ffbeb15e50f93f2ca3f2b80e61c83c0e2e7447  IRIX 6.5.9 Installation Tools and Overlays (1 of 3).iso
676438c9308dbf320647577c3be19159571b49d5  IRIX 6.5.10 Installation Tools and Overlays (1 of 3).iso
b30f8222d92eb2f42b0714582c136b9d54c52bcb  IRIX 6.5.30 Installation Tools and Overlays (1 of 3).iso
0a5e08d3b462961632fb77c7cac17c6261a85266  IRIX 6.5.12 Overlays (3 of 3).iso
654798961047d87265c267fcb2c5ffb7aa6b2d41  IRIX 6.5.13 Overlays 3 of 3.iso
5a7daefc4eb580d36d634de97b6d215f607f06fc  IRIX 6.5.14 Overlays (3 of 3).iso
8333cb5fd087f051a6791aa0353a02b54186c719  IRIX 6.5.15 Overlays (4 of 4).iso
f3dfbd81fafdcb2e41a209e3f94060b50aa68a05  IRIX 6.5.16 Overlays (4 of 4).iso
1ffb35001abdb2324d2f4d4ba666260862405446  IRIX 6.5.17 Overlays (4 of 4).iso
8bd758f54b7c645b6f0e066421e1636cd28a52ed  IRIX 6.5.18 Overlays (4 of 4).iso
00ffd9a5086e43e5ecc69278a559991eeddf6bb3  IRIX 6.5.19 Overlays (4 of 4).iso
```

Patches install with the IRIX software manager; a patch image is an `im001` archive described by the `.idb` beside it, the same layout as the media's product images. Unpack a product or patch with any decoder for that layout and keep only the paths above.

Scanned: the overlay discs of 6.5.9 to 6.5.30, the base and installation-tools discs of 6.4 to 6.5.30, and the IRIX 5.x to 6.3 system discs (manifests only, for the controller files above). Not scanned yet: the customer diagnostics discs beyond the manifest check, and any patch newer than SG0007149. IRIX 5.0 to 6.2 builds of the Challenge and Onyx controller files are on the media but their system discs were not held; the compression is classic `compress(1)` (see [docs/everest/inst-compression.md](docs/everest/inst-compression.md)). They are not in `root/`.

```archives
```

```links
```
