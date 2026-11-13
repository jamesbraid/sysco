#!/usr/bin/env python3
"""List the message call sites of the Origin 300 L1 ColdFire package.

Usage: tools/console_messages.py [--out docs/inventory/console-messages.tsv]

Reads package 1 of L1 1.44.0 (`Fuel/PE/O300 1MB`, the body loaded at 0xffe00000) from root/,
its linear listing and routine splits from docs/l1/disasm/, and writes one row per call of the
two text sinks in that body:

  log_message(severity, file, line, flags, format, ...)   0xffe58380
  console_printf(format, ...)                              0xffe5aed8

Each call's arguments are recovered by walking the stack pushes that precede the call, back to
the adjustment that removes them. A pushed register is resolved to the instruction that last set
it in the same straight-line run. A value that comes from a call result, a frame slot or an
expression is written as such, not as a literal. The guard is the nearest conditional branch
that jumps over the call; its meaning is read from its mnemonic only.

Curated roles are in docs/l1/console-message-roles.tsv, keyed by call address. A row without a
curated role keeps the status "extracted": the arguments were read by this script, and nothing
else about the call was read.

Run inside the toolchain container, from the repository root (retools/tools/re-run python3 ...). The
output is deterministic.
"""
import bisect
import collections
import os
import re
import struct
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
IMAGE = os.path.join(REPO, 'root', '6.5.30', 'usr', 'cpu', 'firmware', 'sysco', 'l1.bin')
DISASM = os.path.join(REPO, 'docs', 'l1', 'disasm', 'mcf5206e-1.44.0-fuel-pe-o300')
OUT = os.path.join(REPO, 'docs', 'inventory', 'console-messages.tsv')
ROLES = os.path.join(REPO, 'docs', 'l1', 'console-message-roles.tsv')
FUNCS = os.path.join(REPO, 'docs', 'l1', 'functions.tsv')
PKG_KEY = '6.5.30:l1#1'
PKG_INDEX = 1
LOAD = 0xffe00000
SINKS = {0xffe58380: ('log_message', 5), 0xffe5aed8: ('console_printf', 1)}
COLUMNS = ['call_addr', 'sink', 'routine', 'severity', 'flags', 'source_file', 'source_line', 'format',
           'varargs', 'guard', 'pushes', 'status', 'grade', 'role', 'evidence']

COND = {'beq': 'equal', 'bne': 'not equal', 'bhi': 'unsigned higher', 'bls': 'unsigned lower or same',
        'bcc': 'carry clear', 'bcs': 'carry set', 'bge': 'signed greater or equal', 'blt': 'signed less',
        'bgt': 'signed greater', 'ble': 'signed less or equal', 'bpl': 'plus', 'bmi': 'minus',
        'bvc': 'overflow clear', 'bvs': 'overflow set'}
FLAG_SETTERS = ('tst', 'cmp', 'cmpi', 'cmpa', 'btst', 'bset', 'bclr', 'bchg', 'sub', 'subq', 'subi', 'add', 'addq',
                'addi', 'and', 'andi', 'or', 'ori', 'eor', 'eori', 'neg', 'not', 'move', 'moveq', 'clr', 'ext',
                'lsl', 'lsr', 'asl', 'asr', 'muls', 'mulu', 'divs', 'divu')
CONV = re.compile(r'%%|%[-+ #0]*(\*|\d+)?(\.(\*|\d+))?[hlLqjzt]*([diouxXcsfgeEpnaA])')


def load_body():
    d = open(IMAGE, 'rb').read()
    n = struct.unpack('>I', d[0x2c:0x30])[0]
    off, ln = struct.unpack('>II', d[0x30 + 8 * PKG_INDEX:0x38 + 8 * PKG_INDEX])
    pk = d[off:off + ln]
    body = pk[44:]
    sp, pc = struct.unpack('>II', body[:8])
    assert pc == 0xffe00400, 'unexpected reset entry 0x%08x' % pc
    return body


def read_listing():
    rx = re.compile(r'^([0-9a-f]{8})\s+([0-9a-f]+)\s+(.*)$')
    ins = []
    for line in open(os.path.join(DISASM, 'l1.lst'), encoding='latin-1'):
        m = rx.match(line.rstrip('\n'))
        if m:
            ins.append((int(m.group(1), 16), m.group(3).strip()))
    ins.sort(key=lambda t: t[0])
    return ins


def read_routines():
    names = {}
    for r in open(FUNCS, encoding='utf-8').read().split('\n')[1:]:
        f = r.split('\t')
        if len(f) > 3 and f[0] == PKG_KEY:
            names[f[3]] = int(f[1], 16)
    starts = []
    for line in open(os.path.join(DISASM, 'l1.sum'), encoding='utf-8'):
        if line.startswith('== '):
            name = line[3:].strip()
            if name.startswith('sub_ffe'):
                starts.append((int(name[4:], 16), name))
            elif name in names:
                starts.append((names[name], name))
    starts.sort()
    return starts


class Image:
    def __init__(self):
        self.body = load_body()
        self.ins = read_listing()
        self.addrs = [a for a, _ in self.ins]
        self.index = {a: i for i, a in enumerate(self.addrs)}
        self.routines = read_routines()
        self.rstarts = [a for a, _ in self.routines]
        self.joins = set()
        for a, t in self.ins:
            mn = t.split(' ', 1)[0]
            if (mn.startswith('b') and mn not in ('bsr',)) or mn.startswith('db') or mn.startswith('jmp'):
                for tgt in re.findall(r'\$([0-9a-f]+)', t):
                    self.joins.add(int(tgt, 16))

    def text(self, a):
        o = a - LOAD
        if not 0 <= o < len(self.body):
            return None
        s = self.body[o:o + 200]
        end = s.find(b'\0')
        if end < 0:
            return None
        return s[:end].decode('latin-1')

    def routine_of(self, a):
        i = bisect.bisect_right(self.rstarts, a) - 1
        if i < 0:
            return '<before first routine>', 0
        return self.routines[i][1], self.routines[i][0]


def is_cond(mn):
    return mn.split('.')[0] in COND or mn.split('.')[0] in ('bhs', 'blo')


def is_transfer(mn):
    return is_cond(mn) or mn.split('.')[0] in ('bra', 'bsr', 'jsr', 'rts', 'jmp')


def split_ops(text):
    mn, _, ops = text.partition(' ')
    return mn, ops.strip()


def push_src(ops):
    """Source operand of a push to -(a7), or None when the instruction is not such a push."""
    if not ops.endswith(', -(a7)'):
        return None
    return ops[:-len(', -(a7)')].strip()


def literal(s):
    m = re.fullmatch(r'#?\$([0-9a-f]+)(\.[lwb])?', s)
    if m:
        return int(m.group(1), 16)
    m = re.fullmatch(r'#(-?\d+)', s)
    if m:
        return int(m.group(1))
    return None


class Walker:
    def __init__(self, img):
        self.img = img

    def resolve(self, reg, pos, depth=0):
        """The value of register reg just before instruction index pos: ('lit', n) or a description."""
        img = self.img
        j = pos - 1
        while j >= 0 and pos - j <= 80:
            a, t = img.ins[j]
            mn, ops = split_ops(t)
            if img.addrs[j + 1] in img.joins and j + 1 <= pos and j + 1 != pos:
                return ('joined', 'value set before the branch target at %08x' % img.addrs[j + 1])
            if mn in ('jsr', 'bsr'):
                if reg == 'd0':
                    return ('call', ops)
                if reg in ('d1', 'a0', 'a1'):
                    return ('clobbered', 'by call at %08x' % a)
            dest = ops.rsplit(', ', 1)[-1].strip() if ', ' in ops else ''
            if mn.split('.')[0] in ('cmp', 'tst', 'btst', 'bra', 'bsr', 'jsr', 'rts', 'jmp') or \
                    mn.startswith('b') or not dest:
                j -= 1
                continue
            if dest == reg:
                base = mn.split('.')[0]
                if base == 'moveq':
                    v = literal(ops.split(',')[0].strip())
                    if v is None:
                        return ('expr', '%08x %s' % (a, t))
                    return ('lit', ((v & 0xff) ^ 0x80) - 0x80)
                if base == 'clr':
                    return ('lit', 0)
                if base in ('move', 'movea'):
                    src = ops[:-(len(dest) + 2)].strip()
                    v = literal(src)
                    if v is not None:
                        return ('lit', v)
                    if re.fullmatch(r'd\d|a\d', src) and depth < 3:
                        return self.resolve(src, j, depth + 1)
                    return ('expr', '%08x %s' % (a, t))
                if base == 'lea':
                    src = ops[:-(len(dest) + 2)].strip()
                    v = literal(src)
                    if v is not None:
                        return ('lit', v)
                    return ('expr', '%08x %s' % (a, t))
                return ('expr', '%08x %s' % (a, t))
            j -= 1
        return ('unknown', 'no definition of %s within 80 instructions' % reg)

    def decode_push(self, j):
        """Describe the value pushed by the instruction at index j, or None when it is no push."""
        img = self.img
        a, t = img.ins[j]
        mn, ops = split_ops(t)
        if mn.startswith('pea'):
            v = literal(ops)
            return ('lit', v) if v is not None else ('expr', '%08x %s' % (a, t))
        if mn.startswith('clr') and ops == '-(a7)':
            return ('lit', 0)
        src = push_src(ops)
        if src is None:
            return None
        v = literal(src)
        if v is not None:
            return ('lit', v)
        if re.fullmatch(r'[da][0-7]', src):
            return self.resolve(src, j)
        return ('expr', '%08x %s' % (a, t))

    def skip_nested(self, j):
        """j indexes a stack cleanup (addq/lea #N,a7) after the call whose arguments it removes.
        Returns the index just before that call's own pushes, or None when the shape is not the
        plain push-call-cleanup one."""
        img = self.img
        mn, ops = split_ops(img.ins[j][1])
        m = re.match(r'#\$?([0-9a-f]+), a7$', ops) if mn.startswith('addq') else \
            re.match(r'\$([0-9a-f]+)\(a7\), a7$', ops) if mn.startswith('lea') else None
        if not m or j == 0:
            return None
        words = int(m.group(1), 16) // 4
        j -= 1
        if split_ops(img.ins[j][1])[0].split('.')[0] not in ('jsr', 'bsr'):
            return None
        j -= 1
        while words > 0 and j >= 0:
            mn, ops = split_ops(img.ins[j][1])
            if is_transfer(mn):
                return None
            if re.search(r'\ba7\b', ops) and mn.split('.')[0] in ('lea', 'addq') and not ops.endswith('-(a7)'):
                j = self.skip_nested(j)
                if j is None:
                    return None
                words -= 1
                continue
            if self.decode_push(j) is not None:
                words -= 1
            j -= 1
        return j if words == 0 else None

    def collect(self, call_idx, need):
        """Pushes before a call, nearest first, up to need of them. Pushes that belong to an
        argument's own nested call are skipped. Stops at a stack adjustment that is not such a
        nested call or at a transfer of control. Returns (pushes, stop_reason)."""
        img = self.img
        out = []
        j = call_idx - 1
        while j >= 0 and len(out) < need:
            a, t = img.ins[j]
            mn, ops = split_ops(t)
            if is_transfer(mn):
                return out, 'transfer at %08x' % a
            if re.search(r'\ba7\b', ops) and mn.split('.')[0] in ('lea', 'addq', 'subq', 'add', 'sub', 'adda', 'suba') and not ops.endswith('-(a7)'):
                k = self.skip_nested(j)
                if k is None:
                    return out, 'stack adjust at %08x' % a
                j = k
                continue
            d = self.decode_push(j)
            if d is not None:
                out.append((a, t, d))
            j -= 1
        return out, 'enough' if len(out) >= need else 'start of listing'

    def passed(self, call_idx):
        """The number of words the call's own cleanup removes, or None when it is not read."""
        img = self.img
        if call_idx + 1 >= len(img.ins):
            return None
        mn, ops = split_ops(img.ins[call_idx + 1][1])
        m = re.match(r'#\$?([0-9a-f]+), a7$', ops) if mn.startswith('addq') else \
            re.match(r'\$([0-9a-f]+)\(a7\), a7$', ops) if mn.startswith('lea') else None
        return int(m.group(1), 16) // 4 if m else None

    def guard(self, call_idx, call_addr, routine_start):
        img = self.img
        j = call_idx - 1
        while j >= 0:
            a, t = img.ins[j]
            if a < routine_start:
                break
            mn, ops = split_ops(t)
            if is_cond(mn):
                tgt = re.findall(r'\$([0-9a-f]+)', ops)
                if tgt:
                    tv = int(tgt[0], 16)
                    if call_addr < tv <= call_addr + 0x400:
                        base = mn.split('.')[0]
                        setter = ''
                        k = j - 1
                        while k >= 0 and j - k <= 6:
                            sa, st = img.ins[k]
                            smn, _ = split_ops(st)
                            if smn.split('.')[0] in FLAG_SETTERS:
                                setter = '%08x %s' % (sa, st)
                                break
                            k -= 1
                        skip = COND.get(base, base)
                        text = '%s at %08x skips the call when %s (flags from %s)' % (mn, a, skip, setter or 'unknown')
                        joins = [x for x in img.joins if a < x <= call_addr]
                        if joins:
                            text += '; other paths reach the call from the branch target at %08x' % max(joins)
                        return text
            j -= 1
        return ''


def conversions(fmt):
    n = 0
    for m in CONV.finditer(fmt):
        if m.group(0) == '%%':
            continue
        n += 1
        if m.group(1) == '*':
            n += 1
        if m.group(3) == '*':
            n += 1
    return n


def describe(v):
    if v is None:
        return ''
    kind, val = v
    if kind == 'lit':
        return '%d' % val if val < 1000 else '0x%x' % val
    return '%s %s' % (kind, val)


# Six messages begin with the name of the routine that logs them. The table shows <routine> in
# place of that name and the full text stays in the image at the call address.
NAMED_PREFIX = {0xffe50260, 0xffe5029c, 0xffe5034c, 0xffe503ce, 0xffe50402, 0xffe580d2}
PREFIX = re.compile(r'^[A-Za-z_]+(?=\(%s\)|: )')


def escape(s):
    return s.replace('\\', '\\\\').replace('\n', '\\n').replace('\t', '\\t')


def read_roles():
    out = {}
    if not os.path.exists(ROLES):
        return out
    lines = open(ROLES, encoding='utf-8').read().split('\n')
    head = lines[0].split('\t')
    for ln in lines[1:]:
        if ln.strip():
            row = dict(zip(head, ln.split('\t')))
            out[int(row['call_addr'], 16)] = row
    return out


def main():
    out_path = OUT
    if '--out' in sys.argv:
        out_path = os.path.abspath(sys.argv[sys.argv.index('--out') + 1])
    img = Image()
    walker = Walker(img)
    roles = read_roles()
    rows = []
    for idx, (a, t) in enumerate(img.ins):
        mn, ops = split_ops(t)
        if mn not in ('jsr', 'bsr'):
            continue
        m = re.match(r'\$([0-9a-f]+)', ops)
        if not m:
            continue
        target = int(m.group(1), 16)
        if target not in SINKS:
            continue
        sink, fixed = SINKS[target]
        pushes, why = walker.collect(idx, fixed)
        fmt_v = pushes[fixed - 1][2] if len(pushes) >= fixed else None
        fmt = None
        if fmt_v and fmt_v[0] == 'lit':
            fmt = img.text(fmt_v[1])
        nconv = conversions(fmt) if fmt is not None else 0
        words = walker.passed(idx)
        want = max(words, fixed) if words is not None else fixed + nconv
        if fmt is not None and len(pushes) < want:
            pushes, why = walker.collect(idx, want)
        if sink == 'log_message':
            sev, srcf, line, flags = (pushes[i][2] if i < len(pushes) else None for i in range(4))
        else:
            sev = flags = srcf = line = None
        src_text = img.text(srcf[1]) if srcf and srcf[0] == 'lit' else None
        rout, rstart = img.routine_of(a)
        var = pushes[fixed:want] if len(pushes) > fixed else []
        status = 'extracted'
        row = {
            'call_addr': '%08x' % a,
            'sink': sink,
            'routine': rout,
            'severity': describe(sev) if sink == 'log_message' else '',
            'flags': describe(flags) if sink == 'log_message' else '',
            'source_file': escape(src_text) if src_text is not None else describe(srcf),
            'source_line': describe(line) if sink == 'log_message' else '',
            'format': escape(PREFIX.sub('<routine>', fmt) if a in NAMED_PREFIX else fmt) if fmt is not None else ('unread: ' + describe(fmt_v) if fmt_v else 'no format pushed'),
            'varargs': '%d of %d' % (len(var), nconv) if fmt is not None else 'unknown',
            'guard': walker.guard(idx, a, rstart),
            'pushes': '; '.join(describe(p[2]) for p in pushes[:want]) + ('' if why == 'enough' else ' [%s]' % why),
            'status': status,
            'grade': '',
            'role': '',
            'evidence': '',
        }
        cur = roles.get(a)
        if cur:
            row.update({'status': 'understood', 'grade': cur['grade'], 'role': cur['role'],
                        'evidence': cur['evidence']})
        rows.append(row)
    with open(out_path, 'w', encoding='utf-8', newline='\n') as f:
        f.write('\t'.join(COLUMNS) + '\n')
        for r in rows:
            f.write('\t'.join(str(r.get(c, '')).replace('\t', ' ') for c in COLUMNS) + '\n')
    counts = collections.Counter(r['sink'] for r in rows)
    print('wrote %d rows to %s: %s' % (len(rows), os.path.relpath(out_path, REPO),
                                        ', '.join('%s %d' % kv for kv in sorted(counts.items()))))


if __name__ == '__main__':
    main()
