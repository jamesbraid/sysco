#!/usr/bin/env python3
"""Write the per-package console command units under defs/ from docs/l1/command-table.tsv.

usage: scripts/l1_console_defs.py [--check]

Writes defs/l1_pkg<n>_console.toml for each ColdFire package: the position of each command in
each registered table. The position is what matters: the parser compares the typed words with
the tables' entries in order and runs the first entry that matches. --check fails when a file
on disk differs from what the table would produce. Needs only the standard library.

The units carry no addresses, no handler or help text and no brick sets; those stay in the
table. The doc of each enum value repeats the typed pattern (and marks a hidden entry), and the
defs check (scripts/mmsc_commands_defs.py, run by retools/tools/defs/ci-check) compares that doc
and the position with the table rows. The shared entry layout, defs/l1_console.toml, is
maintained by hand.
"""
import collections
import csv
import json
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TABLE = os.path.join(REPO, 'docs', 'l1', 'command-table.tsv')
PACKAGES = (0, 1, 2, 3, 5)

HEAD = '''# Written by scripts/l1_console_defs.py from docs/l1/command-table.tsv. Do not edit by hand.
unit = "l1_pkg%(n)s_console"
prefix = "L1P%(n)sC"
title = "console command tables of ColdFire L1 firmware package %(n)s"
doc = """
Position of each console command in the registered command tables of one ColdFire L1 firmware
package. The package registers %(ntables)d command tables. Each CMD<t>.index is the position of an
entry in table t, counted from zero, and table t is the t-th table in the order of the
registration calls in the image. That is not necessarily the order the parser tries them in;
the order of the calls at run time is not read here. Entry layout and argument kinds are in
the shared console unit. The typed pattern is the doc of each enum value; hidden entries are
marked. Labels are the first alternative of each word joined by underscores, with the
system-link word spelled sys_link; positions of entries that share a pattern are told apart by
a numeric suffix.
"""
from = "IRIX 6.5.30 l1.bin (L1 1.44.0), ColdFire package %(n)s image"

[c]
legacy_header = "L1P%(n)s_CONSOLE_DECODE.h"
legacy_guard = "L1P%(n)s_CONSOLE_DECODE_H"
'''


# The generated Rust may not contain the repository's name, and the command word for the
# system-link protocol contains it. The label spells that word differently; the doc of each
# enum value keeps the typed word.
RENAME = {'syscom': 'sys_link'}


def label(pattern):
    out = []
    for t in pattern.split(' '):
        t = t.split('|')[0]
        m = re.fullmatch(r'<(\w+)>', t)
        if m:
            t = m.group(1)
        elif t == '*':
            t = 'all'
        t = re.sub(r'[^A-Za-z0-9]+', '_', t).strip('_').lower()
        t = RENAME.get(t, t)
        if t:
            out.append(t)
    return '_'.join(out) or 'entry'


def read_rows():
    with open(TABLE) as f:
        return list(csv.DictReader(f, delimiter='\t'))


def package_unit(n, rows):
    tables = collections.OrderedDict()
    for r in rows:
        tables.setdefault(int(r['table']), []).append(r)
    out = [HEAD % {'n': n, 'ntables': len(tables)}]
    for t, rs in tables.items():
        word = 'CMD%d' % t
        bits = max(8, len(bin(len(rs) - 1)) - 2)
        out.append('[words.%s]' % word)
        out.append('width = 32')
        out.append('grade = "established"')
        out.append('')
        out.append('[words.%s.fields.index]' % word)
        out.append('bits = [%d, 0]' % (bits - 1))
        out.append('grade = "established"')
        out.append('')
        out.append('[words.%s.fields.index.enums]' % word)
        seen = collections.Counter()
        for r in rs:
            base = label(r['pattern'])
            seen[base] += 1
            name = base if seen[base] == 1 else '%s_%d' % (base, seen[base])
            doc = r['pattern'] + (' (hidden)' if r['hidden'] == '1' else '')
            out.append('%s = { value = %s, grade = "established", doc = %s }' % (name, r['index'], json.dumps(doc)))
        out.append('')
    return '\n'.join(out).rstrip('\n') + '\n'


def main():
    rows = read_rows()
    want = {}
    for n in PACKAGES:
        pk = '6.5.30:l1#%d' % n
        want['l1_pkg%d_console.toml' % n] = package_unit(n, [r for r in rows if r['package'] == pk])
    bad = 0
    for name, text in want.items():
        path = os.path.join(REPO, 'defs', name)
        if '--check' in sys.argv:
            have = open(path).read() if os.path.exists(path) else None
            if have != text:
                print('%s is stale' % name)
                bad = 1
        else:
            with open(path, 'w') as f:
                f.write(text)
    sys.exit(bad)


if __name__ == '__main__':
    main()
