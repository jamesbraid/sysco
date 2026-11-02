#!/usr/bin/env python3
"""The MMSC command catalogue: the command table of the MMSC firmware image, one row per entry.

Usage: scripts/mmsc_commands.py [--out docs/mmsc/commands.tsv]

Run from the repository root inside the toolchain container (tools/re-run python3
scripts/mmsc_commands.py). Reads root/6.5.30/usr/cpu/firmware/mmscfw.bin, the MMSC
firmware image (x86 a.out, OMAGIC, with its symbol table), and writes one TSV row per entry
of the command table (the `_cmdList` array in the image's symbol table). The output is
deterministic.

The image is an a.out file: a 32-byte header, text, data, then the symbol table (12-byte
records, the string table after it). The words at header offsets 24 and 28 are not
relocation sizes in this image and are ignored; no relocation records follow the data.
Text is loaded at the entry address, data immediately after the text, bss after the data.

Each command table entry is three words: the name (a short string), the handler routine and
a help string (empty for aliases). The handler's reachable code is decoded with capstone from
the handler's own start, following branches and stopping at returns, so bytes that follow the
routine (its strings) are not read as code.

Routines are named by address only. The image's symbol names are not copied here.

Columns (every value is read from the image; the curated reading is in docs/mmsc/commands.md):
  index, name, help (yes when the entry has help text), handler_addr, insns, first, last
               the handler routine and the span of its reachable code
  usage        the syntax of each help line, the words before " - ", joined with " | "
  strtok       how many tokens the handler reads from the command line (calls to the
               string tokenizer)
  calls        addresses of the routines the handler calls, in first-call order
  responses    the response words the handler emits (ERROR and OK forms), short literals only
  globals      addresses of the data and bss words it reads or writes
  levels       every read of the session authority word (offset 0x20 of the session
               structure) with the compare that follows it: address, compare immediate and
               the jump after it, and `level>N` or `level<=N` when that jump is taken for
               a value above or at most N (see docs/mmsc/commands.md for the levels)
"""
import argparse
import bisect
import collections
import os
import re
import struct
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
IMAGE = os.path.join(REPO, 'root', '6.5.30', 'usr', 'cpu', 'firmware', 'mmscfw.bin')
CMDLIST = '_cmdList'
AUTH_OFFSET = 0x20  # session structure: the authority value of the connected user
SKIP_NAMES = ('gcc2_compiled.', '___gnu_compiled_c')


class Image:
    def __init__(self, path):
        self.d = open(path, 'rb').read()
        magic, text, data, bss, syms, entry, _tr, _dr = struct.unpack('<8L', self.d[:32])
        if magic & 0xffff != 0o407:
            raise SystemExit('mmsc_commands: not an OMAGIC a.out image: %#x' % magic)
        self.text, self.data, self.bss, self.entry = text, data, bss, entry
        self.tbase, self.dbase, self.bbase = entry, entry + text, entry + text + data
        symoff = 32 + text + data
        strtab = symoff + syms
        self.syms = []
        for i in range(syms // 12):
            strx, typ, _o, _desc, val = struct.unpack('<LBBHL', self.d[symoff + 12 * i:symoff + 12 * i + 12])
            end = self.d.index(b'\0', strtab + strx)
            self.syms.append((self.d[strtab + strx:end].decode('latin1'), typ, val))
        self.by_name = {}
        self.names = collections.defaultdict(list)
        for name, typ, val in self.syms:
            if not name or name.endswith('.o') or name.startswith('D:') or name in SKIP_NAMES:
                continue
            self.names[val].append((typ, name))
            self.by_name.setdefault(name, (typ, val))
        self.text_fns = sorted(v for v, l in self.names.items()
                               if any(t in (4, 5) for t, n in l) and self.in_text(v))

    def in_text(self, a):
        return self.tbase <= a < self.tbase + self.text

    def in_data(self, a):
        return self.dbase <= a < self.dbase + self.data or self.bbase <= a < self.bbase + self.bss

    def off(self, a):
        if self.in_text(a):
            return 32 + (a - self.tbase)
        if self.dbase <= a < self.dbase + self.data:
            return 32 + self.text + (a - self.dbase)
        return None

    def word(self, a):
        o = self.off(a)
        return struct.unpack('<L', self.d[o:o + 4])[0]

    def cstr(self, a, limit=200):
        o = self.off(a)
        if o is None:
            return None
        e = self.d.index(b'\0', o)
        return self.d[o:min(e, o + limit)].decode('latin1')

    def fname(self, a):
        l = self.names.get(a, [])
        l = [n for t, n in l if t in (4, 5)] or [n for t, n in l]
        return l[0] if l else None

    def fend(self, a):
        j = bisect.bisect_right(self.text_fns, a)
        return self.text_fns[j] if j < len(self.text_fns) else self.tbase + self.text

    def dname(self, a):
        l = self.names.get(a, [])
        return l[0][1] if l else 'unnamed_%x' % a


def printable(s):
    return s is not None and len(s) >= 2 and all(32 <= ord(c) < 127 or c in '\r\n\t' for c in s)


def reachable(img, start, end):
    """Decode from start to end with capstone and return the instructions control flow reaches."""
    import capstone
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    code = img.d[img.off(start):img.off(end)]
    insns = {i.address: i for i in md.disasm(code, start)}
    order = sorted(insns)
    seen, todo, out = set(), [start], []
    while todo:
        a = todo.pop()
        if a in seen or a not in insns:
            continue
        seen.add(a)
        i = insns[a]
        out.append(i)
        m = i.mnemonic
        if m == 'call':
            todo.append(a + i.size)
            continue
        if m in ('ret', 'retf', 'hlt', 'ud2') or (m == 'jmp' and not i.op_str.startswith('0x')):
            continue
        tgt = re.match(r'0x([0-9a-f]+)$', i.op_str)
        if m.startswith('j') or m.startswith('loop'):
            if tgt and start <= int(tgt.group(1), 16) < end:
                todo.append(int(tgt.group(1), 16))
            if m != 'jmp':
                todo.append(a + i.size)
            continue
        todo.append(a + i.size)
    out.sort(key=lambda i: i.address)
    return out, insns, order


def command_table(img):
    rows = []
    a = img.by_name[CMDLIST][1]
    while img.word(a) != 0:
        n, h, t = img.word(a), img.word(a + 4), img.word(a + 8)
        rows.append(dict(name=img.cstr(n, 40), handler=h, help=img.cstr(t, 4000) if t else ''))
        a += 12
    return rows


def usage_of(help_text):
    # a usage line starts in column 0 and carries the " - " before its description;
    # indented lines continue the one above
    out = []
    for line in re.split(r'\r?\n', help_text or ''):
        if line[:1].strip() and re.search(r'\s-\s', line):
            out.append(re.split(r'\s+-\s', line, maxsplit=1)[0].strip())
    return out


RESPONSE = re.compile(r'^(R%ld:)?(OK|ERROR)\b[A-Z0-9 ,;%]{0,24}$')


def analyse(img, handler):
    start, end = handler, img.fend(handler)
    insns, _all, _order = reachable(img, start, end)
    calls, strs, globs, levels = [], [], [], []
    strtok = 0
    tokenizer = img.by_name['_strtok_r'][1]
    for k, i in enumerate(insns):
        op = i.op_str
        if i.mnemonic == 'call':
            t = re.match(r'0x([0-9a-f]+)$', op)
            if t:
                v = int(t.group(1), 16)
                if v == tokenizer:
                    strtok += 1
                if '0x%x' % v not in calls:
                    calls.append('0x%x' % v)
            continue
        if i.mnemonic.startswith('j'):
            continue
        for m in re.finditer(r'0x([0-9a-f]+)', op):
            v = int(m.group(1), 16)
            if img.in_text(v) and img.off(v) is not None:
                s = img.cstr(v, 120)
                if printable(s) and v not in [x[0] for x in strs]:
                    strs.append((v, s))
            elif img.in_data(v):
                if '0x%x' % v not in globs:
                    globs.append('0x%x' % v)
        if re.search(r'\[\w+ ?\+ ?0x20\]', op) and not i.mnemonic == 'lea':
            nxt = insns[k + 1] if k + 1 < len(insns) else None
            cmp_ = ''
            if nxt is not None and nxt.mnemonic == 'cmp':
                c = re.search(r',\s*0x([0-9a-f]+)$', nxt.op_str)
                if c:
                    cmp_ = '%#x' % int(c.group(1), 16)
            jcc = ''
            nxt2 = insns[k + 2] if k + 2 < len(insns) else None
            if nxt2 is not None and nxt2.mnemonic.startswith('j') and nxt2.mnemonic != 'jmp':
                jcc = nxt2.mnemonic
            note = ''
            if cmp_ and jcc in ('jg', 'ja'):
                note = 'level>%d' % int(cmp_, 16)
            elif cmp_ and jcc in ('jle', 'jbe'):
                note = 'level<=%d' % int(cmp_, 16)
            levels.append(('%#x %s %s %s' % (i.address, cmp_ or '-', jcc or '-', note)).rstrip())
    return dict(start=start, end=end, insns=insns, calls=calls, strs=strs, globs=globs,
                levels=levels, strtok=strtok)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--out', default=os.path.join(REPO, 'docs', 'mmsc', 'commands.tsv'))
    a = ap.parse_args()
    img = Image(IMAGE)
    rows = command_table(img)
    cols = ['index', 'name', 'help', 'handler_addr', 'insns', 'first', 'last', 'usage',
            'strtok', 'calls', 'responses', 'globals', 'levels']
    out = ['\t'.join(cols)]
    cache = {}
    for k, r in enumerate(rows):
        h = r['handler']
        if h not in cache:
            cache[h] = analyse(img, h)
        f = cache[h]
        first = f['insns'][0].address if f['insns'] else 0
        last = f['insns'][-1].address if f['insns'] else 0
        out.append('\t'.join([
            str(k), r['name'], 'yes' if r['help'] else 'no',
            '%#x' % h, str(len(f['insns'])), '%#x' % first, '%#x' % last,
            ' | '.join(usage_of(r['help'])) or '-', str(f['strtok']),
            ', '.join(f['calls']) or '-',
            ' | '.join(sorted(set(w for _v, w in f['strs'] if RESPONSE.match(w)), key=str)) or '-',
            ', '.join(f['globs']) or '-',
            ' | '.join(f['levels']) or '-']))
    text = '\n'.join(out) + '\n'
    os.makedirs(os.path.dirname(os.path.abspath(a.out)), exist_ok=True)
    with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
        f.write(text)
    print('mmsc_commands: %d entries, %d handlers, %s' % (len(rows), len(cache), a.out))


if __name__ == '__main__':
    sys.exit(main())
