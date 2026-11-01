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
| `patches/SG0005762/` | patch SG0005762, subsystem `patchSG0005762.field_diags_sysco_sw` | L1 1.32.15, L2 1.32.4. The patch's own release note says L1 1.32.14. |
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
f73660c9e85b1dbd615856377e8480a4c8a39f76fc86d21cc23533b0787acd29  patch SG0007114 tardist
9f4b8abf97c0903f697f75888ae1363b19e385a85c9d34446973975eecf9930a  patch SG0007149 tardist
```

The 6.5.30 image is `field_diags_sysco.sw`, 21,355,926 bytes; its idb records `l1.bin` at 15,988,636 bytes and `l2.bin` at 2,924,652 bytes, which the extracted files match.

Patches install with the IRIX software manager; a patch image is an `im001` archive described by the `.idb` beside it, the same layout as the media's product images. Unpack a product or patch with any decoder for that layout and keep only the paths above.

Scanned: the overlay discs of 6.5.20 to 6.5.29. Not scanned yet: the overlay discs of 6.5.19 and earlier, the customer diagnostics discs, and any patch newer than SG0007149.

```archives
```

```links
```
