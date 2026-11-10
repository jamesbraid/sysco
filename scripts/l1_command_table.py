#!/usr/bin/env python3
"""List the console command tables of the ColdFire L1 packages, read from the registration calls.

usage: l1_command_table.py l1.bin PACKAGE_INDEX [FILTER]   one line per entry (index counts the
                                                           ColdFire packages only)
       l1_command_table.py --tsv l1.bin                    the curated table, every ColdFire package
       l1_command_table.py --check TSV l1.bin              fail unless TSV is what the image holds

Each package registers its command tables with one small routine that stores a table's base and
entry count in a list of at most ten tables (the routine reports "command list full" when an
eleventh arrives). The parser walks the registered tables in registration order and each table
from its first entry. A table is an array of entries of 68 bytes (17 big-endian words):

    words 0 to 9   the typed pattern: a pointer to a literal (alternatives separated by '|'), or
                   a small value that stands for an argument (see MARKERS); the pattern ends at
                   the first zero word and every later word of the ten is zero
    word 10        the handler
    words 11 to 14 four words handed to the handler by pointer. Handlers that check the brick
                   type take word 11 as a pointer to the string of allowed brick-type characters
                   (zero: any brick; a leading '~' lists the excluded types); the others take
                   word 11 as a small selector. Words 12 to 14 are further selectors.
    word 15        a pointer to the help text (zero: a one-character dash is listed)
    word 16        1 hides the entry from the command listing, 0 shows it

This replaces an earlier version that scanned the whole body for words that looked like a help
pointer. That scan printed about a hundred false rows outside the tables and missed every entry
whose word 11 is a selector. Here the entries come from the registered base and count, and an entry
that does not have the shape above, or whose second-bank copy differs from it by anything other
than the bank distance, is reported and fails --tsv and --check.

In the line output the leading address is that of the help word (entry start plus 0x3c), which is
what the earlier version printed and what older notes cite; --tsv gives the entry start.

Needs the standard library and retools/tools/sysco/sysco_tables.py (set SYSCO_TOOLS to another
directory that holds it). Run from the repository root.
"""
import os
import re
import struct
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.environ.get('SYSCO_TOOLS', os.path.join(REPO, 'retools', 'tools', 'sysco')))
import sysco_tables as st  # noqa: E402

BASE = 0xffe00000
ENTRY = 68
WORDS = 17
REG_MSG = b'CmdAddCmds: command list full!\0'
PACKAGE = '6.5.30:l1#%d'
COLUMNS = ['package', 'table', 'index', 'entry', 'pattern', 'hidden', 'handler', 'word11',
           'word12', 'word13', 'word14', 'grade']

# A pattern word of 0 to 11 is not a pointer. These are the values the parser and the listing
# routine both give a meaning to. 7, 9 and 10 fall through to the literal path in the parser
# and no table uses them.
MARKERS = {
    1: 'number',        # numeric argument, base from its prefix
    2: 'decimal',       # numeric argument, base 10
    3: 'hex',           # numeric argument, base 16
    4: 'octal',         # numeric argument, base 8
    5: 'word',          # one word, passed as a string
    6: 'command',       # the rest of the line
    8: 'port_l1',       # ":<port>:<l1>", two numbers
    11: 'rack_slot',    # "<rack>.<slot>", two numbers around one separator character
}


def cf_packages(data):
    return [p for p in st.l1_packages(data) if not p['linux']]


class Body:
    def __init__(self, pkg):
        self.b = pkg['body']

    def word(self, a):
        o = a - BASE
        return struct.unpack('>I', self.b[o:o + 4])[0]

    def text(self, a):
        """The literal at a, or None when a does not point at a short printable string."""
        o = a - BASE
        if a < BASE or o >= len(self.b):
            return None
        t = self.b[o:self.b.find(b'\0', o)]
        if not t or len(t) > 200 or any(c < 9 or c > 126 for c in t):
            return None
        return t.decode('latin1')

    def set_text(self, a):
        """A pointer-valued word 11: the string it points at (possibly empty), else None."""
        if a < BASE or a - BASE >= len(self.b):
            return None
        return self.text(a) if self.b[a - BASE] else ''

    def registrations(self):
        """(call address, base, count) for every call of the table-registration routine."""
        b = self.b
        so = b.find(REG_MSG)
        if so < 0:
            raise ValueError('registration routine message not found')
        p = b.find(b'\x48\x79' + struct.pack('>I', BASE + so))
        f = b.rfind(b'\x4e\x56', 0, p)
        out = []
        for m in re.finditer(b'\x4e\xb9' + struct.pack('>I', BASE + f), b):
            c = m.start()
            pre = b[c - 16:c]
            i = pre.rfind(b'\x48\x79')
            tbl = struct.unpack('>I', pre[i + 2:i + 6])[0]
            before = pre[:i]
            if before[-4:-2] == b'\x48\x78':                  # pea.w count
                cnt = struct.unpack('>H', before[-2:])[0]
            elif before[-6:-4] == b'\x2f\x3c':                # move.l #count,-(a7)
                cnt = struct.unpack('>I', before[-4:])[0]
            else:
                mm = re.search(rb'\x70(.)\x2f\x00$', before)  # moveq #count,d0 ; move.l d0,-(a7)
                if not mm:
                    raise ValueError('count of the call at %x not recognised' % (BASE + c))
                cnt = mm.group(1)[0]
            out.append((BASE + c, tbl, cnt))
        return sorted(out)

    def bank_shift(self):
        """Distance from the first bank's copy of the program to the second bank's."""
        i = self.b.find(REG_MSG)
        j = self.b.find(REG_MSG, i + 1)
        return j - i

    def parity(self, a, w):
        """Problems found comparing an entry with its second-bank copy (empty when they agree)."""
        sh = self.bank_shift()
        w2 = self.entry(a + sh)
        return [k for k in range(WORDS) if w2[k] != w[k] and w2[k] != w[k] + sh]

    def entry(self, a):
        return [self.word(a + 4 * k) for k in range(WORDS)]

    def pattern(self, w):
        """The typed pattern as a list of strings, or None when the entry is malformed."""
        out = []
        ended = False
        for t in w[:10]:
            if t == 0:
                ended = True
            elif ended:
                return None
            elif t <= 11:
                if t not in MARKERS:
                    return None
                out.append('<%s>' % MARKERS[t])
            else:
                s = self.text(t)
                if s is None:
                    return None
                out.append(s)
        return out or None

    def problems(self, w):
        out = []
        if self.pattern(w) is None:
            out.append('pattern')
        if not BASE <= w[10] < BASE + len(self.b):
            out.append('handler')
        if w[15] and self.text(w[15]) is None:
            out.append('help')
        if BASE <= w[11] < BASE + len(self.b) and self.set_text(w[11]) is None:
            out.append('word11')
        if w[16] not in (0, 1):
            out.append('hidden')
        return out


def rows(data, idx):
    body = Body(cf_packages(data)[idx])
    out = []
    for t, (site, base, cnt) in enumerate(body.registrations()):
        for i in range(cnt):
            a = base + ENTRY * i
            w = body.entry(a)
            bad = body.problems(w)
            if body.parity(a, w):
                bad.append('bank2')
            out.append({'table': t, 'index': i, 'entry': a, 'w': w, 'bad': bad,
                        'pattern': body.pattern(w), 'body': body})
    return out


def signed(v):
    return v - (1 << 32) if v >= 1 << 31 else v


def word11(body, v):
    s = body.set_text(v)
    return 'set:' + s if s is not None else '%d' % signed(v)


def line(r):
    w, body = r['w'], r['body']
    s = body.set_text(w[11])
    bricks = 'set:' + s if s is not None else '%d' % signed(w[11])
    return '%08x handler=%08x word11=%s sel=%d,%d,%d hidden=%d | %s | %s' % (
        r['entry'] + 0x3c, w[10], bricks, signed(w[12]), signed(w[13]), signed(w[14]), w[16],
        body.text(w[15]) or '', ' '.join(r['pattern'] or ['?']))


def tsv(data):
    out = ['\t'.join(COLUMNS)]
    for idx, pk in enumerate(p for p in st.l1_packages(data) if not p['linux']):
        for r in rows(data, idx):
            w, body = r['w'], r['body']
            if r['bad']:
                sys.exit('package %d table %d entry %d is malformed: %s' % (
                    pk['index'], r['table'], r['index'], r['bad']))
            out.append('\t'.join([
                PACKAGE % pk['index'], str(r['table']), str(r['index']), '%08x' % r['entry'],
                ' '.join(r['pattern']), str(w[16]), '%08x' % w[10], word11(body, w[11]),
                str(signed(w[12])), str(signed(w[13])), str(signed(w[14])), 'established']))
    return '\n'.join(out) + '\n'


def main():
    a = sys.argv[1:]
    if a and a[0] == '--tsv':
        sys.stdout.write(tsv(open(a[1], 'rb').read()))
    elif a and a[0] == '--check':
        want = tsv(open(a[2], 'rb').read())
        have = open(a[1]).read()
        if want != have:
            import difflib
            for d in list(difflib.unified_diff(have.splitlines(), want.splitlines(),
                                               'file', 'image', lineterm='', n=0))[:20]:
                print(d)
            sys.exit('%s does not match the image' % a[1])
        print('%s matches the image (%d rows)' % (a[1], want.count('\n') - 1))
    elif len(a) >= 2:
        data = open(a[0], 'rb').read()
        flt = a[2] if len(a) > 2 else ''
        for r in rows(data, int(a[1])):
            ln = line(r)
            if r['bad']:
                ln += ' !malformed:' + ','.join(r['bad'])
            if flt in ln:
                print(ln)
    else:
        sys.exit(__doc__)


if __name__ == '__main__':
    main()
