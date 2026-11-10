# sysco

Reverse-engineering notes for the SGI system-controller firmware of the Origin 300, Origin 3000 and Origin 350 generation, read from the images IRIX ships to flash it. "sysco" is the name of the shipped firmware directory (`usr/cpu/firmware/sysco`).

Two controllers are in scope for the reading notes below; [docs/controllers.md](docs/controllers.md) surveys every other SGI system controller (the Challenge and Onyx controller, the MSC and ELSC, the MMSC, the L3 and the Altix generations), what is held for each, and the order the rest will be read in.

- **L1**, the per-brick controller. The first target is the Origin 300 L1 (the Fuel/PE/O300 1 MB ColdFire MCF5206e image), then the personalities for the C-brick, I-brick and R-brick. The L1 container also carries ColdFire images for other brick families and two embedded-Linux images for ARM S3C2410 controllers; they come after the Origin 300 image.
- **L2**, the rack controller: a PowerPC MPC860 running Linux 2.4.7 with a cramfs root. It comes second.

The findings are specifications and reading notes, not a reimplementation. They are meant to be enough for someone to run the real firmware on an emulated machine: which peripherals the code touches, at which addresses and widths, what it expects back, and how it talks to the node, to its peers and to a host over USB.

What is here:

- [docs/firmware-versions.md](docs/firmware-versions.md): every L1 and L2 version found, with sizes, hashes and the media or patch each came from.
- [docs/l1/](docs/l1/): reading notes for the Origin 300 L1 1.44.0 image (package layout, boot, M-Bus and board records, USB device side, RS-422 router, node link, debug console), the request catalogue, I/O register and protocol tables, the symbol map, and the listing of that image under `docs/l1/disasm/`.
- [docs/l1-console-behaviour.md](docs/l1-console-behaviour.md): what the Origin 300 L1 prints and answers on its console in each state, with graded facts and the tests that would settle the unknowns.
- [docs/l2-kernel-interface.md](docs/l2-kernel-interface.md): what the L2 user-space programs need from the kernel and the board (loader hand-off, physical windows, backing drivers, system-call surface, the PCI and USB chain to the first registered L1).
- [docs/host-tool-options.md](docs/host-tool-options.md): the command lines of `l1cmd`, `l2cmd` and `flashsc`, and what the first two send.
- [docs/inventory/](docs/inventory/README.md): tables measured from the images (images, packages, functions, request codes, I/O registers, protocols) plus the host tools' ELF tables.
- [docs/coverage.md](docs/coverage.md) is the map of what is covered; [docs/open-questions.md](docs/open-questions.md) lists what the images have not yet answered.

The firmware images are not tracked. [IRIX_FILES.md](IRIX_FILES.md) says which files to copy from which media into the ignored `root/` directory. [AGENTS.md](AGENTS.md) has the scope, the evidence rules and the finish line.

## Running the tools

The host needs Podman, git and python3. Everything else runs in a container built from `retools/tools/Containerfile`:

    retools/tools/re-run python3 retools/tools/inventory.py        # host-tool ELF tables, root manifest check
    retools/tools/re-run python3 retools/tools/sysco/sysco_tables.py     # firmware tables and the firmware coverage block

`tools/setup-clone.sh --root DIR` links `root/` to a directory you filled. The shared tooling lives in the `retools/` submodule (`git submodule update --init`); shared tools are `retools/tools/`, the ones specific to this repository `retools/tools/sysco/`. Only `retools/tools/re-run` and `tools/ghidra-export.py` link into it. `scripts/` holds this repository's own scripts and configuration.

Licence: BSD-3-Clause, see [LICENSE](LICENSE). The scripts descend from gr4's and keep its notice in [retools/tools/LICENSE-gr4](retools/tools/LICENSE-gr4).
