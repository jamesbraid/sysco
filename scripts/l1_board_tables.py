#!/usr/bin/env python3
"""List the sensor tables a ColdFire L1 board personality registers with the monitoring layer.

usage: l1_board_tables.py l1.bin PACKAGE_INDEX   (index counts the ColdFire packages only)

Each board-init routine hands nine tables to nine registration routines (I/O expanders, board
record routes, monitors, supplies, fan control, tachometers, temperature, and two more). The
script disassembles the first bank with capstone (68040 mode, which covers the ColdFire ISA_A
subset the compiler emitted), finds each call, reads the table address and count from the
pushes before it, and prints the names the table's words point at. Counts are the push before
the table address; a block's routines are grouped when calls sit within 0x100 bytes.

Run with capstone available (`uv run --with capstone`).
"""
import os
import re
import struct
import sys

from capstone import Cs, CS_ARCH_M68K, CS_MODE_BIG_ENDIAN, CS_MODE_M68K_040

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sysco_tables as st  # noqa: E402

BASE = 0xffe00000
# registration routines in package 2, by role (roles of the last two are not established)
ROLES = {0xffe6be58: 'supply', 0xffe6ae18: 'fan control', 0xffe6aec4: 'tachometer',
         0xffe7f080: 'temperature(I)', 0xffe896f4: 'monitor', 0xffe874b0: 'expander',
         0xffe8cb14: 'records', 0xffe784d4: 'table8(?)', 0xffe6f8b8: 'table2(?)'}


def main():
    data = open(sys.argv[1], 'rb').read()
    pk = [p for p in st.l1_packages(data) if not p['linux']][int(sys.argv[2])]
    b = pk['body']
    md = Cs(CS_ARCH_M68K, CS_MODE_BIG_ENDIAN | CS_MODE_M68K_040)
    lines = []
    pos = 0x1000
    end = min(len(b), 0x1000 + 0xf0000)
    while pos < end:
        got = False
        for i in md.disasm(b[pos:pos + 16], BASE + pos, 1):
            lines.append((i.address, '%s %s' % (i.mnemonic, i.op_str)))
            pos += i.size
            got = True
        if not got:
            pos += 2

    def w(a):
        o = a - BASE
        return struct.unpack('>I', b[o:o + 4])[0]

    def s(a):
        if not (BASE <= a < BASE + len(b)):
            return None
        o = a - BASE
        t = b[o:b.find(b'\0', o)]
        if not t or len(t) > 40 or any(c < 9 or c > 126 for c in t):
            return None
        return t.decode('latin1')

    calls = []
    for k, (addr, txt) in enumerate(lines):
        m = re.match(r'jsr \$(ffe[0-9a-f]+)\.l', txt)
        if not m or int(m.group(1), 16) not in ROLES:
            continue
        tbl = cnt = None
        for j in range(k - 1, max(0, k - 12), -1):
            m2 = re.match(r'pea\.l \$(ffe[0-9a-f]+)\.l', lines[j][1])
            if m2:
                tbl = int(m2.group(1), 16)
                m3 = re.match(r'moveq #\$([0-9a-f]+), d0', lines[j - 2][1])
                cnt = int(m3.group(1), 16) if m3 else None
                break
        if tbl:
            calls.append((addr, ROLES[int(m.group(1), 16)], tbl, cnt))
    groups, cur = [], []
    for c in calls:
        if cur and c[0] - cur[-1][0] > 0x100:
            groups.append(cur)
            cur = []
        cur.append(c)
    if cur:
        groups.append(cur)
    for g in groups:
        print('--- block at %08x' % g[0][0])
        addrs = sorted(c[2] for c in g)
        for c in g:
            nxt = [a for a in addrs if a > c[2]]
            span = (nxt[0] - c[2]) if nxt else 0x120
            names = []
            for a in range(c[2], c[2] + min(span, 0x200), 4):
                v = w(a)
                if BASE + 0x4000 <= v < BASE + len(b) and s(v):
                    names.append(s(v))
            print('  %-15s count=%s table=%08x %s' % (c[1], c[3], c[2], names))


if __name__ == '__main__':
    main()
