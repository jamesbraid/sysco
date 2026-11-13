# sysco: working rules

This repository reverse-engineers the SGI L1 and L2 system-controller firmware (Origin 300, Origin 3000 and Origin 350 era) from the images and host tools that IRIX ships. Read [README.md](README.md) for the scope and [evidence/irix-files.md](evidence/irix-files.md) for the inputs. The layout and working style follow the sibling graphics and chip repositories; where this file is silent, do what they do.

## 1. Scope

- **L1 first.** The Origin 300 L1 (ColdFire MCF5206e, the `Fuel/PE/O300 1MB` package, version 1.44.0) is the reference image. Then the other ColdFire packages in the same container (the C-brick, I-brick and R-brick personalities and the 2 MB families), then the two embedded-Linux ARM S3C2410 images and the userland in their cramfs.
- **L2 second.** The PowerPC MPC860 image: boot loader, Linux 2.4.7 kernel and the big-endian cramfs with the L2 daemon and its libraries.
- **Every version.** Findings name the version and package they were read from. A fact read in 1.44.0 is not assumed for 1.24.8, 1.32.15 or 1.48.1 until the same bytes are read there; [evidence/firmware-versions.md](evidence/firmware-versions.md) lists the sets.
- **The goal** is a description of the firmware's hardware interface complete enough that nothing it touches is left to guesswork: every memory-mapped peripheral and external device the code touches, with address, width and expected reply; the packet, USB, RS-422 and node-UART protocols; the boot path and bank selection; the request catalogue.
- Host tools (`flashsc`, `l1cmd`, `l2cmd`, `flashpbay`, `flashsvf`) are in scope as evidence of how the controllers are driven. SVF JTAG vector files (`.svf`) are inventoried; only their IDCODE reads are used.

## 2. Evidence: software only, any source as a hint

No hardware is available. Do not plan, request or wait for measurements on a real machine.

Readers may open anything: the shipped images and tools of every release and patch, any disassembly or decompilation of them, vendor and chip manuals, public drivers, emulators and kernels for the same CPUs, manuals, release notes and man pages, and published output from real machines (console logs, forum and mailing-list posts).

But a hint is not a finding. **Every fact is graded and confirmed in a shipped image**: it states the version, package, address or offset it was read at, and says what that read established. A fact taken from a manual, a driver or a post, and not yet seen in an image, is written as a hint with its source, and is not graded established.

Write findings in your own words. Never paste vendor source, firmware strings beyond the short literals a finding needs, or the images themselves into the repository.

## 3. Grades

- **established**: read in full in the image (the whole routine, table or record) and the claim follows from what was read. Cite version, package, address.
- **inferred**: a reasoned reading from partial evidence (call lists, store summaries, a neighbouring version, a name). Say what the reasoning is.
- **unknown**: nothing in any shipped image or public source settles it. List the routes tried in [questions/README.md](questions/README.md) and carry on.

Never reword an entry to change its grade. An entry written quickly and not yet checked ends with "Unverified." and counts at most as located. Never write "needs capture".

## 4. Names and exposure

- No names of the downstream emulator or project that consumes this work, of any person, of any internal tooling or of any private path. Cite public documents by title and number. Do not describe how the work was organised.
- No details of private document collections or indexes: a finding cites the public document, or the image and address.
- No commit trailers of any kind, no generated-by banners, no sign-off lines.
- Firmware binaries stay in the ignored `root/`; listings, decompiled text and tables derived from them are committed, under `software/firmware/l1/disasm/<arch>-<version>-<package>/` (and `software/firmware/l2/` for the PowerPC image when it lands).

## 5. Tables and the finish line

`retools/tools/inventory.py` (shared byte for byte with the sibling repositories) measures the host tools and checks `root/` against `tools/config/inventory-root.sha256`. `retools/tools/sysco/sysco_tables.py` measures the firmware: images, packages, functions per package, request codes, I/O registers and protocols. Both write to [evidence/inventory/](evidence/inventory/README.md); regenerate in the same commit as any note that explains an item.

A function row is **understood** when a canonical routine entry in `software/firmware/l1/` or `evidence/routines/` gives it a grade, a role and a locator, and its byte checks pass. The entry format is in `retools/tools/README-analysis.md` under "Routine role entries". Request codes, I/O registers, protocols and packages are understood through curated entries in `software/firmware/l1/`. A row is **located** when its address, offset or hash is cited in a note. The repository is finished when every table reaches understood or has an unknown entry with its routes listed.

Curated entries that name a package are checked against the bytes on every run: a handler address outside the body, or a `check` cell that does not match, fails `retools/tools/sysco/sysco_tables.py`.

## 6. Roles

- **Readers** take a cluster of related rows from the lowest-status tables (a function family, a peripheral, a request group, a protocol) and take each to understood. They write notes in `software/firmware/l1/` or `software/firmware/l2/`, add curated rows, and regenerate.
- A reader of vendor binaries does not review an implementation of the same device, and an implementer works from the notes, not the listings.

## 7. Shared toolset

The container definitions, wrapper, Ghidra scripts, inventory and fact sheet are the `retools/` submodule (sgi-retools): shared tools in `retools/tools/`, device-specific ones in `retools/tools/sysco/`. Nothing here links into it except `retools/tools/re-run` and `tools/ghidra-export.py` (see [tools/README.md](tools/README.md)), and there is no local leak check or pre-push hook. Change the shared tools there and move the pin; do not fork them here; sysco-specific code lives in its own files (`retools/tools/sysco/sysco_tables.py`, `tools/config/inventory-config.json`).

ColdFire work in Ghidra uses the `68000:BE:32:Coldfire` language; PowerPC uses `PowerPC:BE:32:default` with the MPC860 internal memory map set by hand; ARM uses `ARM:LE:32:v4`. The listings under `software/firmware/l1/disasm/` were made with a 68040-mode decoder, which covers the ColdFire ISA_A subset the compiler emitted; data mixed into the text decodes as `.dw` words or implausible instructions, so confirm a routine's shape from its prologue and return.

## 8. Commits

Small and often, straight to `main`. Subjects are lowercase, component-prefixed imperatives (`software/firmware/l1: read the M-Bus start sequence`); bodies say why.

## 9. Hardware definitions

`devices/<unit>/registers.toml` and `formats/<name>/format.toml` are the source of the hardware facts this repository publishes: register and word layouts, enum values, command positions and constants, one TOML file per unit, each item graded and carrying the source it was read from (`from`). `generated/` is generated from them by `retools/tools/defs/defs-gen` and is never edited by hand: the C headers (`generated/c/`), the Rust crate (`generated/rust/`), the Python and JSON forms, the genxml export (`generated/xml/`) and a reference page per unit next to its source (`registers.md`, or `format.md` under `formats/`). To change a fact, change the TOML and regenerate.

The `*_DECODE.h` files at the repository root are forwarding headers that include `generated/c/` and keep the old macro names; do not edit them either. The five `formats/l1_pkg<n>_console/format.toml` files are written by `tools/l1_console_defs.py` from `software/firmware/l1/command-table.tsv`, so change the table and rerun the script. Hand-written helper macros go in `devices/<unit>/helpers.h` (`formats/<name>/helpers.h`).

Run `retools/tools/defs/defs-check --build` before committing a change under `devices/`, `formats/` or `generated/`; CI runs the same check through `retools/tools/defs/ci-check`. The plugin `tools/mmsc_commands_defs.py` ties the command units to the command tables in `docs/`.
