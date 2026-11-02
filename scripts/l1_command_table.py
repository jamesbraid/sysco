#!/usr/bin/env python3
"""List the console command table of one ColdFire L1 package.

usage: l1_command_table.py l1.bin PACKAGE_INDEX [FILTER]  (index counts the ColdFire packages only)

Each command entry is 17 words: ten token words (the words typed, then argument markers), a
handler pointer, a pointer to the string of brick-type characters the command is allowed on
(zero when the command takes any brick), three flag words and a pointer to the help text. The
script finds entries by that shape, so it prints the entries the shape selects and no others.
A set string that starts with '~' lists the brick types that are excluded.

Needs only the standard library; run from the repository root.
"""
import os
import re
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sysco_tables as st  # noqa: E402

BASE = 0xffe00000


def main():
    data = open(sys.argv[1], 'rb').read()
    pk = [p for p in st.l1_packages(data) if not p['linux']][int(sys.argv[2])]
    b = pk['body']
    flt = sys.argv[3] if len(sys.argv) > 3 else ''

    def w(a):
        o = a - BASE
        return struct.unpack('>I', b[o:o + 4])[0] if 0 <= o <= len(b) - 4 else None

    def s(a):
        if a is None or a < BASE:
            return None
        o = a - BASE
        if o >= len(b):
            return None
        t = b[o:b.find(b'\0', o)]
        if not t or len(t) > 200 or any(c < 9 or c > 126 for c in t):
            return None
        return t.decode('latin1')

    for x in range(BASE + 0x10000, BASE + min(len(b), 0x70000), 4):
        sh = s(w(x))
        if not sh or len(sh) < 5:
            continue
        cb = w(x - 0x14)
        if cb is None or not (BASE + 0x1000 <= cb < BASE + len(b)):
            continue
        bp = w(x - 0x10)
        if bp != 0 and not s(bp):
            continue
        toks = []
        for i in range(10):
            v = w(x - 0x3c + 4 * i)
            t = s(v) if v and v >= BASE else None
            toks.append(t if t else str(v))
        words = ' '.join(t for t in toks if t != '0')
        line = '%08x handler=%08x bricks=%s flags=%d,%d,%d | %s | %s' % (
            x, cb, s(bp) if bp else '*', w(x - 12), w(x - 8), w(x - 4), sh, words)
        if flt in line:
            print(line)


if __name__ == '__main__':
    main()
