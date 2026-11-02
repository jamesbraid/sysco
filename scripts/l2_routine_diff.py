#!/usr/bin/env python3
"""Routine-level diff of the L2 user-space ELFs between two sets, plus the kernel.

Usage: UV_CACHE_DIR=$HOME/.cache/uv uv run --no-project --with pyelftools python -I \\
           scripts/l2_routine_diff.py --old DIR_OLD --new DIR_NEW [--out docs/version-diffs]

DIR_OLD and DIR_NEW are the output directories of scripts/l2_extract.py (kernel.bin and rootfs/).
Each ELF present in both roots is read through its program headers. Routines are the defined
dynamic symbols of type FUNC with a nonzero size: the L2 binaries are stripped, so the exported
routines are the only named ones. A routine is compared by the hash of its bytes. Internal
routines of the daemon are not covered.

Writes docs/version-diffs/cd-ist-3.24-l2-routines.tsv (one row per changed, added or removed
exported routine, and one row per changed ELF with no routine change) and prints the counts.
The kernel is compared by 4 KB block when the two kernels are the same length, else by block
match counts, as scripts/version_diff.py does for the 1.x sets.
"""
import collections
import hashlib
import json
import os
import sys

from elftools.elf.elffile import ELFFile

OUT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'docs', 'version-diffs')
if '--out' in sys.argv:
    OUT = os.path.abspath(sys.argv[sys.argv.index('--out') + 1])
OLD = os.path.abspath(sys.argv[sys.argv.index('--old') + 1])
NEW = os.path.abspath(sys.argv[sys.argv.index('--new') + 1])


def sha(b):
    return hashlib.sha256(b).hexdigest()


def is_elf(path):
    with open(path, 'rb') as f:
        return f.read(4) == b'\x7fELF'


def routines(path):
    """name -> (sha256 of the bytes, size) for each defined dynamic FUNC symbol with a size."""
    with open(path, 'rb') as f:
        elf = ELFFile(f)
        ds = elf.get_section_by_name('.dynsym')
        if ds is None:
            return {}, elf['e_type']
        loads = [s for s in elf.iter_segments() if s['p_type'] == 'PT_LOAD']
        raw = open(path, 'rb').read()
        out = {}
        for sym in ds.iter_symbols():
            if sym['st_info']['type'] != 'STT_FUNC' or sym['st_shndx'] == 'SHN_UNDEF':
                continue
            v, n = sym['st_value'], sym['st_size']
            if not v or not n:
                continue
            for seg in loads:
                if seg['p_vaddr'] <= v < seg['p_vaddr'] + seg['p_filesz']:
                    off = v - seg['p_vaddr'] + seg['p_offset']
                    out[sym.name] = (sha(raw[off:off + n]), n)
                    break
        return out, elf['e_type']


def elf_paths(root):
    base = os.path.join(root, 'rootfs')
    found = {}
    for d, _, files in os.walk(base):
        for name in files:
            p = os.path.join(d, name)
            if not os.path.islink(p) and is_elf(p):
                found[os.path.relpath(p, base)] = p
    return found


def main():
    os.makedirs(OUT, exist_ok=True)
    a, b = elf_paths(OLD), elf_paths(NEW)
    rows = [['path', 'routine', 'change', 'old_bytes', 'new_bytes']]
    summary = collections.OrderedDict()
    for rel in sorted(set(a) & set(b)):
        ra, _ = routines(a[rel])
        rb, _ = routines(b[rel])
        cnt = collections.Counter()
        for name in sorted(set(ra) | set(rb)):
            if name not in rb:
                ch = 'removed'
            elif name not in ra:
                ch = 'added'
            elif ra[name][0] == rb[name][0]:
                ch = 'same'
            else:
                ch = 'changed'
            cnt[ch] += 1
            if ch != 'same':
                rows.append([rel, name, ch, ra.get(name, ('', 0))[1], rb.get(name, ('', 0))[1]])
        if cnt:
            summary[rel] = dict(cnt)
    kern = {}
    for tag, root in (('old', OLD), ('new', NEW)):
        kern[tag] = open(os.path.join(root, 'kernel.bin'), 'rb').read()
    ka, kb = kern['old'], kern['new']
    blocks_a = [ka[i:i + 4096] for i in range(0, len(ka), 4096)]
    blocks_b = [kb[i:i + 4096] for i in range(0, len(kb), 4096)]
    same_blocks = sum(1 for x, y in zip(blocks_a, blocks_b) if x == y)
    summary['kernel'] = {'same_length': len(ka) == len(kb), 'bytes': [len(ka), len(kb)],
                         'differing_bytes': sum(x != y for x, y in zip(ka, kb)) if len(ka) == len(kb) else None,
                         'blocks_equal_in_order': same_blocks, 'blocks': len(blocks_a)}
    summary['only_in_old'] = sorted(set(a) - set(b))
    summary['only_in_new'] = sorted(set(b) - set(a))
    with open(os.path.join(OUT, 'cd-ist-3.24-l2-routines.tsv'), 'w') as f:
        for r in rows:
            f.write('\t'.join(str(c) for c in r) + '\n')
    print(json.dumps(summary, indent=1))


if __name__ == '__main__':
    main()
