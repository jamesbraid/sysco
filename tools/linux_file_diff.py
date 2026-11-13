#!/usr/bin/env python3
"""File-level diff of the Linux L1 cramfs roots between two sets.

Usage: retools/tools/re-run python3 tools/linux_file_diff.py [--out evidence/version-diffs]

Reads the generated table evidence/inventory/arm-linux-files.tsv (one row per CramFS entry of each
Linux L1 package, with type, mode, size, sha256 and symlink target). Each pair below is compared
by path. The output lists the paths that are added, removed or changed, not the unchanged ones.
The kernels inside the packages are compared byte by byte, run by run of differing bytes; each run
is marked printable when both sides are printable ASCII, and binary otherwise. The bytes of runs
of 32 or more are not written.

Writes evidence/version-diffs/cd-ist-3.24-linux-files.tsv and cd-ist-3.24-linux-kernels.tsv.
"""
import collections
import os
import sys
import zlib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sysco_tables as st  # noqa: E402

OUT = os.path.join(st.REPO, st.VERSION_DIFFS_DIR)
PAIRS = [
    ('cd-ist-3.24:l1#2', 'patches/SG0007149:l1#4', 'Legacy Linux L1 image'),
    ('cd-ist-3.24:l1#4', 'patches/SG0007149:l1#6', 'Linux L1 image'),
]


def load(path):
    tab = {}
    with open(path) as f:
        next(f)
        for line in f:
            pkg, p, kind, mode, size, sha, target = (line.rstrip('\n').split('\t') + [''] * 7)[:7]
            tab.setdefault(pkg, {})[p] = (kind, mode, size, sha, target)
    return tab


def kernel(rel, index):
    data = open(os.path.join(st.ROOT, rel, st.FW, 'l1.bin'), 'rb').read()
    pk = [p for p in st.l1_packages(data) if p['index'] == index][0]['data']
    gz = pk.find(b'\x1f\x8b\x08')
    return zlib.decompressobj(31).decompress(pk[gz:])


def printable(b):
    return all(0x20 <= c < 0x7f or c in (9, 10) for c in b)


def kernel_rows(new, old, label):
    """Byte runs of the decompressed kernels that differ, for kernels of the same length."""
    (rn, ino), (ro, io_) = new, old
    a, b = kernel(ro, io_), kernel(rn, ino)
    rows = [[label, 'bytes_old', len(a)], [label, 'bytes_new', len(b)]]
    if len(a) != len(b):
        rows.append([label, 'length', 'differs'])
        return rows
    diff = [i for i in range(len(a)) if a[i] != b[i]]
    rows.append([label, 'differing_bytes', len(diff)])
    runs = []
    for i in diff:
        if runs and i - runs[-1][1] <= 4:
            runs[-1][1] = i
        else:
            runs.append([i, i])
    for s_, e in runs:
        kind = 'printable' if printable(a[s_:e + 1]) and printable(b[s_:e + 1]) else 'binary'
        short = e - s_ < 32
        rows.append([label, 'run', '0x%x-0x%x' % (s_, e), kind, a[s_:e + 1].hex() if short else '',
                     b[s_:e + 1].hex() if short else ''])
    return rows


def main():
    os.makedirs(OUT, exist_ok=True)
    krows = [['pair', 'item', 'value', 'kind', 'old_bytes_hex', 'new_bytes_hex']]
    for new, old, label in PAIRS:
        (rn, ino), (ro, io_) = [tuple(x.split(':l1#')) for x in (new, old)]
        for r in kernel_rows((rn, int(ino)), (ro, int(io_)), new + ' vs ' + old):
            krows.append(r)
    inv = load(os.path.join(st.REPO, st.INVENTORY_DIR, 'arm-linux-files.tsv'))
    rows = [['pair', 'label', 'path', 'change', 'old_type', 'new_type', 'old_sha256', 'new_sha256', 'old_bytes',
             'new_bytes', 'old_target', 'new_target']]
    summary = collections.OrderedDict()
    for new, old, label in PAIRS:
        A, B = inv[old], inv[new]
        cnt = collections.Counter()
        for p in sorted(set(A) | set(B)):
            if p not in B:
                ch = 'removed'
            elif p not in A:
                ch = 'added'
            else:
                a, b = A[p], B[p]
                if a[0] != b[0] or a[1] != b[1] or a[4] != b[4]:
                    ch = 'changed-metadata'
                elif a[3] != b[3] or a[2] != b[2]:
                    ch = 'changed-content'
                else:
                    ch = 'same'
            cnt[ch] += 1
            if ch == 'same':
                continue
            a = A.get(p, ('', '', '', '', ''))
            b = B.get(p, ('', '', '', '', ''))
            rows.append([new + ' vs ' + old, label, p, ch, a[0], b[0], a[3], b[3], a[2], b[2], a[4], b[4]])
        summary['%s vs %s' % (new, old)] = dict(cnt)
    with open(os.path.join(OUT, 'cd-ist-3.24-linux-files.tsv'), 'w') as f:
        for r in rows:
            f.write('\t'.join(r) + '\n')
    with open(os.path.join(OUT, 'cd-ist-3.24-linux-kernels.tsv'), 'w') as f:
        for r in krows:
            f.write('\t'.join(str(c) for c in r) + '\n')
    import json
    print(json.dumps(summary, indent=1))


if __name__ == '__main__':
    main()
