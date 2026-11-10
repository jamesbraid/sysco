#!/usr/bin/env python3
"""Write the console command decode headers from docs/l1/command-table.tsv.

usage: scripts/l1_console_headers.py [--check]

Writes the shared entry-grammar header and one per-package header for each ColdFire package (the position of each command in each registered
table). The position is what matters: the parser compares the typed words with the tables'
entries in order and runs the first entry that matches. --check fails when a header on disk
differs from what the table would produce. Needs only the standard library.

The headers carry no addresses, no handler or help text and no brick sets; those stay in the
table. The comment above each ENUM line repeats the typed pattern, and the defs check
(scripts/mmsc_commands_defs.py, run by retools/tools/defs/ci-check) compares that comment and
the position with the table rows.
"""
import collections
import csv
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TABLE = os.path.join(REPO, 'docs', 'l1', 'command-table.tsv')
PACKAGES = (0, 1, 2, 3, 5)

SHARED = '''/*
 * L1_CONSOLE_DECODE.h - entry layout and argument kinds of the L1 console command tables
 *
 * The console command line of the ColdFire L1 controller firmware is table driven. Each package
 * registers up to ten tables, and each table is an array of 68-byte entries made of seventeen
 * big-endian 32-bit words. The parser tries the registered tables in registration order and
 * each table from its first entry, and runs the handler of the first entry whose typed pattern
 * matches the line.
 *
 * The pattern is up to ten words and ends at the first zero word. A word above 11 points at a
 * literal; its alternatives are separated by a vertical bar and a typed word matches when it
 * equals one alternative byte for byte. A word of 1 to 11 stands for an argument of the kind
 * TOKEN.kind names. The values 7, 9 and 10 are not argument kinds; no table uses them.
 *
 * Four words follow the handler and are passed to it as one block. Handlers that restrict a
 * command to some brick types read the first word as a pointer to the string of allowed
 * brick-type characters (zero allows every brick; a leading '~' lists the excluded types).
 * The other handlers read the first word, and the three after it, as small selectors.
 *
 * The last word of an entry is 1 when the entry is left out of the command listing.
 *
 * Which entry sits at which position in each package is in the per-package console headers.
 */
#ifndef L1_CONSOLE_DECODE_H
#define L1_CONSOLE_DECODE_H

#include <stdint.h>

#define L1C_ENTRY_SIZE 68 /* established */
#define L1C_ENTRY_WORDS 17 /* established */
#define L1C_PATTERN_WORDS 10 /* established */
#define L1C_TABLES_MAX 10 /* established */

/* REG: ENTRY.pattern 0x00 r32 count=10 reset=unknown established */
/* REG: ENTRY.handler 0x28 r32 reset=unknown established */
/* REG: ENTRY.arg 0x2c r32 count=4 reset=unknown established */
/* REG: ENTRY.help 0x3c r32 reset=unknown established */
/* REG: ENTRY.hidden 0x40 r32 reset=unknown established */

/* FIELD: TOKEN.kind 3:0 established */
#define L1C_TOKEN_KIND_SHIFT 0
#define L1C_TOKEN_KIND_MASK 0xfu

/* end of the pattern */
/* ENUM: TOKEN.kind 0 end established */
#define L1C_TOKEN_KIND_END 0

/* number, base taken from its prefix (decimal, 0 for octal, 0x for hexadecimal) */
/* ENUM: TOKEN.kind 1 number established */
#define L1C_TOKEN_KIND_NUMBER 1

/* number, always decimal */
/* ENUM: TOKEN.kind 2 decimal established */
#define L1C_TOKEN_KIND_DECIMAL 2

/* number, always hexadecimal */
/* ENUM: TOKEN.kind 3 hex established */
#define L1C_TOKEN_KIND_HEX 3

/* number, always octal */
/* ENUM: TOKEN.kind 4 octal established */
#define L1C_TOKEN_KIND_OCTAL 4

/* one word, handed to the command as a string */
/* ENUM: TOKEN.kind 5 word established */
#define L1C_TOKEN_KIND_WORD 5

/* the rest of the line, handed to the command as a string */
/* ENUM: TOKEN.kind 6 rest_of_line established */
#define L1C_TOKEN_KIND_REST_OF_LINE 6

/* a colon, a port number, a colon and a controller number */
/* ENUM: TOKEN.kind 8 port_l1 established */
#define L1C_TOKEN_KIND_PORT_L1 8

/* a rack number, one separator character and a slot number */
/* ENUM: TOKEN.kind 11 rack_slot established */
#define L1C_TOKEN_KIND_RACK_SLOT 11

#endif
'''

PER_PACKAGE = '''/*
 * L1P%(n)s_CONSOLE_DECODE.h - console command tables of ColdFire L1 firmware package %(n)s
 *
 * The package registers %(ntables)d command tables. Each CMD<t>.index below is the position of an
 * entry in table t, counted from zero, and table t is the t-th table in the order of the
 * registration calls in the image. That is not necessarily the order the parser tries them in;
 * the order of the calls at run time is not read here. Entry layout and argument kinds are in
 * the shared console header. The typed pattern is repeated above each ENUM line; hidden entries are
 * marked. Labels are the first alternative of each word joined by underscores, with the
 * system-link word spelled sys_link; positions of entries that share a pattern are told apart by
 * a numeric suffix.
 */
#ifndef L1P%(n)s_CONSOLE_DECODE_H
#define L1P%(n)s_CONSOLE_DECODE_H
'''


# The generated Rust may not contain the repository's name, and the command word for the
# system-link protocol contains it. The label spells that word differently; the comment above
# each ENUM keeps the typed word.
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


def package_header(n, rows):
    tables = collections.OrderedDict()
    for r in rows:
        tables.setdefault(int(r['table']), []).append(r)
    out = [PER_PACKAGE % {'n': n, 'ntables': len(tables)}]
    for t, rs in tables.items():
        word = 'CMD%d' % t
        bits = max(8, len(bin(len(rs) - 1)) - 2)
        out.append('/* FIELD: %s.index %d:0 established */' % (word, bits - 1))
        out.append('#define L1P%sC_%s_INDEX_SHIFT 0' % (n, word))
        out.append('#define L1P%sC_%s_INDEX_MASK 0x%xu' % (n, word, (1 << bits) - 1))
        seen = collections.Counter()
        for r in rs:
            base = label(r['pattern'])
            seen[base] += 1
            name = base if seen[base] == 1 else '%s_%d' % (base, seen[base])
            out.append('')
            out.append('/* typed %s%s */' % (r['pattern'], ' (hidden)' if r['hidden'] == '1' else ''))
            out.append('/* ENUM: %s.index %s %s established */' % (word, r['index'], name))
            out.append('#define L1P%sC_%s_INDEX_%s %s' % (n, word, name.upper(), r['index']))
        out.append('')
    out.append('#endif')
    return '\n'.join(out) + '\n'


def main():
    rows = read_rows()
    want = {'L1_CONSOLE_DECODE.h': SHARED}
    for n in PACKAGES:
        pk = '6.5.30:l1#%d' % n
        want['L1P%d_CONSOLE_DECODE.h' % n] = package_header(n, [r for r in rows if r['package'] == pk])
    bad = 0
    for name, text in want.items():
        path = os.path.join(REPO, name)
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
