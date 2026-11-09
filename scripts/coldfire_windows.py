#!/usr/bin/env python3
"""Read ColdFire L1 candidate windows with the host fact extractor and write facts-only entries.

    uv run --with capstone --with pyelftools --with numpy --with unicorn \\
        python -I scripts/coldfire_windows.py generate --pool POOL.json --index N --batch-size 250 \\
            --work DIR --out-md docs/l1/NAME.md --out-facts docs/l1/disasm/coldfire-variants/NAME-facts.txt
    ... python -I scripts/coldfire_windows.py validate --pool POOL.json --index N --batch-size 250 \\
            --work DIR --md docs/l1/NAME.md --facts docs/l1/disasm/coldfire-variants/NAME-facts.txt

POOL.json is a list of {"image", "start", "size"} rows. Rows are ordered by image key and address, then cut into
batches of --batch-size; --index picks one batch (from 1). The binary root is `root/` of this repository.

Every statement is produced from retools/tools/hostfacts.py output (accesses, calls, blocks, exits, warnings and the
decoded instruction list) for the exact window bytes. Nothing is hand-written. An entry never admits a function:
it states whether a reached call in a complete decoded caller window of the same payload targets the entry address,
and stays open when none does. The validate command re-extracts every window and checks each line of the written
entries against the fresh facts, one source fact per statement, and every instruction citation against the listing.
"""
import argparse
import bisect
import collections
import hashlib
import importlib.util
import json
import os
import re
import struct
import sys

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)


def _load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


sys.path.insert(0, os.path.join(REPO, 'retools', 'scripts'))
sha = lambda b: hashlib.sha256(b).hexdigest()
H = r'0x[0-9a-f]+'
PADDING = (b'', b'\x4e\x71', b'\x00\x00')


# ------------------------------------------------------------------ images
def build_index(keys, work):
    """Map image keys to carrier, package and payload identities; write payloads under work/payloads."""
    st = _load('sysco_tables', os.path.join(HERE, 'sysco_tables.py'))
    path = os.path.join(work, 'index.json')
    index = json.load(open(path)) if os.path.exists(path) else {}
    if all(k in index for k in keys):
        return index
    os.makedirs(os.path.join(work, 'payloads'), exist_ok=True)
    for rel_path in st.find_images():
        if os.path.basename(rel_path) != 'l1.bin':
            continue
        carrier = open(os.path.join(st.ROOT, rel_path), 'rb').read()
        if carrier[:8] != b'ESTFBINR':
            continue
        rel = next(x for x in sorted(st.RELEASES, key=len, reverse=True) if rel_path.startswith(x + '/'))
        early = not rel_path[len(rel) + 1:].startswith(st.FW)
        packages = [st.pre_container_package(carrier)] if early or st._old_layout(carrier) else st.l1_packages(carrier)
        for p in packages:
            if p['linux']:
                continue
            key = '%s:l1#%d' % (rel, p['index'])
            if key in index:
                continue
            body = p['body']
            payload = os.path.join(work, 'payloads', key.replace('/', '-').replace(':', '-').replace('#', '-') + '.bin')
            with open(payload, 'wb') as out:
                out.write(body)
            index[key] = dict(
                image=key, public_source=rel_path, version=p['version'], load=hex(p['load']),
                carrier_size=len(carrier), carrier_sha256=sha(carrier), package_offset=hex(p['offset']),
                package_size=len(p['data']), package_sha256=sha(p['data']),
                payload_offset_in_carrier=hex(p['offset'] + len(p['data']) - len(body)),
                payload_size=len(body), payload_sha256=sha(body), payload_path=payload)
    json.dump(index, open(path, 'w'), indent=1)
    return index


def check_payload(meta):
    body = open(meta['payload_path'], 'rb').read()
    assert len(body) == meta['payload_size'] and sha(body) == meta['payload_sha256']
    return body


class Image:
    """One payload with its candidate entry addresses, raw call encodings and a fact cache."""

    def __init__(self, hf, meta):
        self.meta = meta
        self.body = check_payload(meta)
        self.load = int(meta['load'], 16)
        self.hf = hf
        self.img = hf.FirmwareImage(meta['payload_path'], 'coldfire', self.load)
        cands = {}
        body, base = self.body, self.load
        for off in range(0, len(body) - 1, 2):
            if body[off:off + 2] == b'\x4e\x56':
                cands[base + off] = 1
            if body[off:off + 2] == b'\x4e\xb9' and off + 6 <= len(body):
                t = struct.unpack('>I', body[off + 2:off + 6])[0]
                if base <= t < base + len(body) and t % 2 == 0:
                    cands[t] = 1
        self.cands = sorted(cands)
        self.raw_calls = collections.defaultdict(list)
        for off in range(0, len(body) - 5, 2):
            if body[off:off + 2] == b'\x4e\xb9':
                self.raw_calls[struct.unpack('>I', body[off + 2:off + 6])[0]].append(base + off)
        self.cache = {}
        self.callers = {}

    def facts(self, start, size):
        key = (start, size)
        if key not in self.cache:
            name, addr, sz, source = self.img.find(hex(start), size)
            rt = self.hf.FirmwareRoutine(self.img, name, addr, sz, source)
            f = rt.facts(False)
            self.cache[key] = (f, rt.render(f))
        return self.cache[key]

    def window_bytes(self, start, size):
        return self.body[start - self.load:start - self.load + size]

    def next_candidate(self, start):
        i = bisect.bisect_right(self.cands, start)
        return self.cands[i] if i < len(self.cands) else self.load + len(self.body)

    def complete(self, f, start, end):
        """Decoded window is LINK to terminal RTS with closed local control."""
        ins = f['instructions']
        if not ins or ins[0]['mn'] != 'link.w' or ins[-1]['mn'] != 'rts' or f['unreached']:
            return False
        cursor = start
        for z in ins:
            if z['addr'] != cursor or z['mn'] in ('dc.w', '.dw', '.byte') or len(bytes.fromhex(z['bytes'])) != z['size']:
                return False
            cursor += z['size']
        if cursor != end or f['coverage']['unresolved_transfers']:
            return False
        starts = {z['addr'] for z in ins}
        for b in f['blocks']:
            if b['kind'] in ('jump', 'conditional') and (b.get('target') is None or int(b['target'], 16) not in starts):
                return False
        return True

    def caller_window(self, call_addr):
        """The complete caller window holding a raw call site, or None. Returns (start, end, facts)."""
        i = bisect.bisect_right(self.cands, call_addr) - 1
        if i < 0:
            return None
        start = self.cands[i]
        if start in self.callers:
            return self.callers[start]
        result = None
        upper = self.next_candidate(start)
        try:
            f, _ = self.facts(start, upper - start)
            rts = [z for z in f['instructions'] if z['mn'] == 'rts']
            if rts and f['instructions'][0]['mn'] == 'link.w':
                end = rts[-1]['addr'] + rts[-1]['size']
                if self.body[end - self.load:upper - self.load] in PADDING:
                    f2, _ = self.facts(start, end - start) if end != upper else (f, None)
                    if self.complete(f2, start, end):
                        result = (start, end, f2)
        except (ValueError, KeyError, IndexError, AssertionError):
            result = None
        self.callers[start] = result
        return result

    def entry_proofs(self, start):
        """Reached direct calls to start from complete caller windows, as (call address, caller start, caller end)."""
        proofs = []
        for site in self.raw_calls.get(start, []):
            cw = self.caller_window(site)
            if not cw:
                continue
            cstart, cend, f = cw
            if not cstart <= site < cend:
                continue
            for c in f['calls']:
                if int(c['address'], 16) == site and c.get('target') and int(c['target'], 16) == start:
                    proofs.append((site, cstart, cend, c['mnemonic']))
        return proofs


# ------------------------------------------------------------------ frame linkage
def registers(s):
    out = []
    for item in s.split('/'):
        if '-' in item:
            lo, hi = item.split('-')
            assert lo[0] == hi[0] and lo[0] in 'da'
            out.extend(lo[0] + str(n) for n in range(int(lo[1:]), int(hi[1:]) + 1))
        else:
            assert re.fullmatch('[da][0-7]', item)
            out.append(item)
    return out


def frame_exclusions(f):
    """Indices of access records that are only proved frame linkage; AssertionError when the shape is not proved."""
    ins, access = f['instructions'], f['accesses']
    assert ins[0]['mn'] == 'link.w' and ins[0]['ops'].startswith('a6, #$')
    assert ins[-2]['mn'] == 'unlk' and ins[-2]['ops'] == 'a6' and ins[-1]['mn'] == 'rts'
    assert not any(z['ops'].endswith(', a6') for z in ins[1:-2]), 'frame pointer overwritten'
    by = {hex(z['addr']): z for z in ins}
    at = {}
    for i, a in enumerate(access):
        at.setdefault(a['address'], []).append((i, a))
    prefix, i = [], 1
    if ins[i]['mn'] == 'lea.l' and ins[i]['ops'].endswith('(a7), a7'):
        i += 1
    while i < len(ins):
        z = ins[i]
        if z['mn'] == 'movem.l' and z['ops'].endswith(', (a7)'):
            prefix.append(z)
            i += 1
            break
        if z['mn'] == 'move.l' and re.fullmatch(r'[da][0-7], -\(a7\)', z['ops']):
            prefix.append(z)
            i += 1
            continue
        break
    restore = ins[-3] if prefix else None
    saves, restores = {}, {}
    for z in prefix:
        regs = registers(z['ops'].split(', ')[0])
        rows = at[hex(z['addr'])]
        assert len(regs) == len(rows)
        for reg, (idx, a) in zip(regs, rows):
            assert a['kind'] == 'store' and a['width'] == 4 and a['base'] == 'a7' and a.get('origin') == 'sp' and a['value'] == reg
            saves[reg] = a['origin_offset']
    if prefix:
        assert restore['mn'] in ['movem.l', 'move.l', 'movea.l']
        parts = restore['ops'].split(', ')
        assert len(parts) == 2 and re.fullmatch(r'-\$[0-9a-f]+\(a6\)', parts[0])
        regs = registers(parts[1])
        raw_offset = -int(re.fullmatch(r'-\$([0-9a-f]+)\(a6\)', parts[0])[1], 16)
        restores = {reg: raw_offset + 4 * j - 4 for j, reg in enumerate(regs)}
        rows = at.get(hex(restore['addr']), [])
        if rows:
            assert len(regs) == len(rows)
            for reg, (idx, a) in zip(regs, rows):
                assert a['kind'] == 'load' and a['width'] == 4 and a['base'] == 'a6' and a['offset'] - 4 == restores[reg]
        assert saves == restores, 'saved registers do not match the terminal restore'
    sites = [hex(ins[0]['addr']), hex(ins[-2]['addr'])] + [hex(z['addr']) for z in prefix]
    if restore:
        sites.append(hex(restore['addr']))
    out = set()
    for site in sites:
        z = by[site]
        for idx, a in at.get(site, []):
            assert a['width'] == 4
            if z['mn'] == 'link.w':
                assert a['kind'] == 'store' and a['base'] == 'a7' and a['value'] == 'a6' and a.get('origin_offset') == -4
            if z['mn'] == 'unlk':
                assert a['kind'] == 'load' and a['base'] == 'a6' and a['offset'] == 0
            out.add(idx)
    assert not any(a['address'] == hex(ins[-1]['addr']) for a in access)
    return out


# ------------------------------------------------------------------ statements
ACCESS_LINE = re.compile(r'^  (0x[0-9a-f]+) (load|store|unknown) w(\d+) (\S+) base=(\S+) disp (\S+) => (.+)$')


def targets_of(render, n):
    out = []
    for line in render.splitlines():
        m = ACCESS_LINE.match(line)
        if m:
            out.append(m[7].rsplit(' value ', 1)[0] if m[2] == 'store' else m[7])
    assert len(out) == n, 'access lines do not match access records'
    return out


def unreached_runs(f):
    ur = {int(a, 16) for a in f['unreached']}
    runs, cur = [], []
    for z in f['instructions']:
        if z['addr'] in ur and (not cur or cur[-1]['addr'] + cur[-1]['size'] == z['addr']):
            cur.append(z)
        else:
            if cur:
                runs.append(cur)
            cur = [z] if z['addr'] in ur else []
    if cur:
        runs.append(cur)
    return [dict(start=hex(r[0]['addr']), last=hex(r[-1]['addr']), end=hex(r[-1]['addr'] + r[-1]['size']), count=len(r)) for r in runs]


def decode_gaps(f, start, end):
    gaps, cursor = [], start
    for z in f['instructions']:
        if z['addr'] != cursor:
            gaps.append((cursor, z['addr']))
        cursor = z['addr'] + z['size']
    if cursor < end:
        gaps.append((cursor, end))
    return gaps


def invalid_records(f):
    return [z for z in f['instructions'] if z['mn'] in ('dc.w', '.dw', '.byte') or len(bytes.fromhex(z['bytes'])) != z['size']]


def source_claims(f, render, start, end):
    ins = {hex(z['addr']): z for z in f['instructions']}
    ur = {int(z, 16) for z in f['unreached']}
    try:
        excluded = frame_exclusions(f)
        framed = True
    except (AssertionError, IndexError, KeyError):
        excluded, framed = set(), False
    targets = targets_of(render, len(f['accesses']))
    groups = collections.OrderedDict()
    for i, (a, t) in enumerate(zip(f['accesses'], targets)):
        if int(a['address'], 16) in ur or i in excluded:
            continue
        shape = {k: v for k, v in a.items() if k not in ('address', 'text')}
        encoded = ins[a['address']]['bytes']
        target = 'unknown' if t == '? (unknown effective address)' else t
        key = json.dumps([shape, encoded, target], sort_keys=True)
        g = groups.setdefault(key, dict(kind='memory', pattern=len(groups) + 1, source_indices=[], shape=shape,
                                        encoded_instruction=encoded, target=target, records=[]))
        g['source_indices'].append(i)
        g['records'].append(a)
    claims = list(groups.values())
    for i, c in enumerate(f['calls']):
        if int(c['address'], 16) not in ur:
            claims.append(dict(kind='call', source_index=i, record=c))
    for i, b in enumerate(f['blocks']):
        if int(b['first'], 16) not in ur and b['kind'] in ('conditional', 'jump'):
            claims.append(dict(kind=b['kind'], source_index=i, record=b, mnemonic=ins[b['last']]['mn']))
    for i, e in enumerate(f['exits']):
        if int(e['address'], 16) not in ur:
            claims.append(dict(kind='exit', source_index=i, record=e, mnemonic=ins[e['address']]['mn']))
    if not f['exits']:
        claims.append(dict(kind='noexit', record=[]))
    for i, w in enumerate(f['warnings']):
        m = re.fullmatch(r'(0x[0-9a-f]+) (.+): unsupported effects invalidated', w)
        if m:
            claims.append(dict(kind='effectwarning', source_index=i, address=m[1], mnemonic=m[2]))
    for r in unreached_runs(f):
        claims.append(dict(kind='unreached', record=r))
    for a, b in decode_gaps(f, start, end):
        claims.append(dict(kind='gap', record=dict(start=hex(a), end=hex(b))))
    return claims, framed


fmt = lambda v: 'unknown' if v == '?' else str(v)
KIND_WORDS = {'load': 'load', 'store': 'store', 'unknown': 'access of unknown direction'}


def sites(records):
    return '`' + records[0]['mnemonic'] + '` at ' + ', '.join(z['address'] for z in records[:3])


def render_claim(c):
    k = c['kind']
    if k == 'memory':
        a = c['records'][0]
        w = {1: 'Byte', 2: 'Two-byte', 4: 'Four-byte'}[a['width']]
        v = ', value `' + fmt(a['value']) + '`' if a['kind'] == 'store' else ''
        n = len(c['records'])
        return "  %s %s, pattern %d: target `%s`, raw base `%s`%s. Count %d static %s, including %s." % (
            w, KIND_WORDS[a['kind']], c['pattern'], c['target'], a['base'] or 'unknown', v, n, 'access' if n == 1 else 'accesses', sites(c['records']))
    if k == 'call':
        a = c['record']
        args = ', '.join(z['location'] + '=`' + fmt(z['value']) + '`' for z in a['args'])
        return "  Call (`%s` at %s): %s, target %s. Candidate slots: %s." % (
            a['mnemonic'], a['address'], a['callee'] or '(unknown callee)', a['target'] or 'unknown', args)
    if k == 'conditional':
        a = c['record']
        return "  Branch (`%s` at %s): target %s. Taken when `%s`. Falls through when `%s`." % (
            c['mnemonic'], a['last'], a['target'] or 'unknown', a['taken_when'], a['falls_through_when'])
    if k == 'jump':
        a = c['record']
        if a['target'] is None:
            return "  Jump (`%s` at %s): target unknown. The local CFG lists no edge." % (c['mnemonic'], a['last'])
        return "  Jump (`%s` at %s): target %s. The local CFG lists a taken edge and no fallthrough edge." % (c['mnemonic'], a['last'], a['target'])
    if k == 'exit':
        a = c['record']
        return "  At the exit (`%s` at %s), %s=`%s`." % (c['mnemonic'], a['address'], a['register'], fmt(a['value']))
    if k == 'noexit':
        return '  No reached exit is modeled.'
    if k == 'effectwarning':
        return "  Effect warning (`%s` at %s): unsupported effects invalidated." % (c['mnemonic'], c['address'])
    if k == 'unreached':
        r = c['record']
        return "  Local unreached instructions: %s to %s, ending at %s (exclusive). Count %d." % (r['start'], r['last'], r['end'], r['count'])
    if k == 'gap':
        r = c['record']
        return "  Decoder coverage gap: bytes %s to %s (exclusive) remain raw and open." % (r['start'], r['end'])
    raise ValueError(k)


def shape_line(f):
    ins = f['instructions']
    if not ins:
        return '  Window shape: no decoded instruction.'
    return "  Window shape: first `%s` at %s, last `%s` at %s, %d decoded instructions." % (
        ins[0]['mn'], hex(ins[0]['addr']), ins[-1]['mn'], hex(ins[-1]['addr']), len(ins))


def proof_lines(image, start):
    proofs = image.entry_proofs(start)
    raw = len(image.raw_calls.get(start, []))
    if not proofs:
        return ["  Entry call proof: none among %d raw absolute `jsr` encodings of this target. The entry stays open and no function is admitted." % raw]
    out = ["  Entry call proof: %d reached direct %s from complete caller windows, among %d raw absolute `jsr` encodings of this target." % (
        len(proofs), 'call' if len(proofs) == 1 else 'calls', raw)]
    for site, cs, ce, mn in proofs[:3]:
        out.append("  Entry call site: `%s` at %s in complete caller window %s to %s (exclusive)." % (mn, hex(site), hex(cs), hex(ce)))
    return out


PREAMBLE = [
    'Each entry applies to its exact shipped build, package, payload and load address.',
    'The windows are inferred from candidate entry addresses. Function extent and function uniqueness remain unknown.',
    'Roles, types, physical pointees, runtime banks and path feasibility remain unknown.',
    'Local path statements assume ordinary call fallthrough.',
    'Calls show six candidate stack locations. Actual arity and custom ABI remain unknown.',
    'Resolved target expressions and known values stay as printed. Unknown effective addresses show only a raw base.',
    'Counts describe static access records, not dynamic counts or equal runtime pointees.',
    'A pattern groups identical encoded instructions and native access fields within one entry.',
    'Different patterns stay separate even when their displayed unresolved targets agree.',
    'Where the frame shape is proved, frame linkage and paired entry saves and terminal restores are omitted from the summaries.',
    'Where it is not proved, every access record is retained and the entry says so.',
    'Meaningful or ambiguous incoming, local, outgoing and return state is retained.',
    'The common-subset decoder does not establish silicon identity. Unsupported effects remain unknown.',
    'The window shape line states only the first and last decoded instruction. A window that does not run from a link instruction to a return is not a body.',
    'Unreached instructions and coverage gaps are located and remain raw and open.',
    'An entry call proof is a reached direct call in a complete decoded caller window of the same payload.',
    'A complete caller window runs from a link instruction to a terminal return with at most two raw padding bytes before the next candidate address.',
    'It decodes without gaps or invalid records, leaves no unreached instruction and has no unresolved transfer, and every direct branch target is an instruction start.',
    'Only absolute `jsr` encodings are searched. Other call forms are not searched and prove nothing here.',
    'An entry with no such proof stays open: it is recorded with its facts and is not admitted as a function.',
    'Decoded caller reachability does not prove runtime execution.',
]


def entry_lines(image, w):
    meta = image.meta
    start, size = int(w['start'], 16), w['size']
    end = start + size
    f, render = image.facts(start, size)
    window = image.window_bytes(start, size)
    assert len(window) == size
    claims, framed = source_claims(f, render, start, end)
    lines = [
        "- `%s`, `%s`." % (w['start'], meta['public_source']),
        "  Package `%s`, version `%s`, load `%s`. Role: unknown." % (meta['image'], meta['version'], meta['load']),
        "  Inferred analysis window: %s to %s (exclusive), %d bytes." % (w['start'], hex(end), size),
        "  Carrier: %d bytes, SHA256 `%s`." % (meta['carrier_size'], meta['carrier_sha256']),
        "  Package: carrier offset %s, %d bytes, SHA256 `%s`." % (meta['package_offset'], meta['package_size'], meta['package_sha256']),
        "  Payload: carrier offset %s, %d bytes, SHA256 `%s`." % (meta['payload_offset_in_carrier'], meta['payload_size'], meta['payload_sha256']),
        "  Read window: SHA256 `%s`." % sha(window),
        shape_line(f),
    ]
    if not framed:
        lines.append('  Frame linkage is not proved for this window. All access records are retained.')
    lines.extend(proof_lines(image, start))
    lines.extend(render_claim(c) for c in claims)
    return lines, f, render


# ------------------------------------------------------------------ batches
def load_batch(pool_path, index, batch_size):
    pool = json.load(open(pool_path))
    keys = lambda q: (q['image'], int(q['start'], 16))
    rows = sorted(({'image': q['image'], 'start': q['start'], 'size': q['size']} for q in pool), key=keys)
    assert len({(q['image'], q['start']) for q in rows}) == len(rows), 'duplicate pool rows'
    return rows[(index - 1) * batch_size:index * batch_size]


def open_images(batch, work):
    hf = _load('hostfacts', os.path.join(REPO, 'retools', 'tools', 'hostfacts.py'))
    index = build_index(sorted({q['image'] for q in batch}), work)
    return {k: Image(hf, index[k]) for k in sorted({q['image'] for q in batch})}


def facts_header(meta):
    return 'Package %s; payload offset %s' % (meta['image'], meta['payload_offset_in_carrier'])


def public_render(render, meta):
    first = re.compile(r'^  file \S+  flat')
    out = []
    for line in render.splitlines():
        if first.match(line):
            line = line.replace(meta['payload_path'], '%s [%s; version %s; payload at carrier offset %s]' % (
                meta['public_source'], meta['image'], meta['version'], meta['payload_offset_in_carrier']), 1)
        out.append(line)
    return '\n'.join(out) + '\n'


def generate(args):
    batch = load_batch(args.pool, args.index, args.batch_size)
    images = open_images(batch, args.work)
    title = args.title
    facts_name = os.path.relpath(args.out_facts, os.path.join(REPO, 'docs', 'l1'))
    lines = ['# ' + title, ''] + PREAMBLE + ['', 'The [full facts](%s) retain all instructions, constants and warnings.' % facts_name, '']
    facts_text = []
    stats = collections.Counter()
    for w in batch:
        image = images[w['image']]
        entry, f, render = entry_lines(image, w)
        lines.extend(entry)
        lines.append('')
        facts_text.append(facts_header(image.meta) + '\n' + public_render(render, image.meta))
        stats['entries'] += 1
        stats['call_proved' if any(l.startswith('  Entry call proof: ') and 'none' not in l for l in entry) else 'open'] += 1
    open(args.out_md, 'w').write('\n'.join(lines).rstrip() + '\n')
    open(args.out_facts, 'w').write('\n'.join(facts_text))
    print(json.dumps(dict(stats)))


# ------------------------------------------------------------------ validation
CITE = re.compile(r'`([^`\n]+)` at (' + H + r'(?:,\s*' + H + r')*)')
W = {'Byte': 1, 'Two-byte': 2, 'Four-byte': 4}
MEM = re.compile(r'^  (Byte|Two-byte|Four-byte) (load|store|access of unknown direction), pattern (\d+): target `([^`]+)`, raw base `([^`]+)`(?:, value `([^`]+)`)?\. Count (\d+) static (access|accesses), including (.+)\.$')
CALL = re.compile(r'^  Call \(`([^`]+)` at (' + H + r')\): (.+), target (' + H + r'|unknown)\. Candidate slots: (.+)\.$')
BR = re.compile(r'^  Branch \(`([^`]+)` at (' + H + r')\): target (' + H + r'|unknown)\. Taken when `([^`]+)`\. Falls through when `([^`]+)`\.$')
JU = re.compile(r'^  Jump \(`([^`]+)` at (' + H + r')\): target (' + H + r'|unknown)\. The local CFG lists (a taken edge and no fallthrough edge|no edge)\.$')
EX = re.compile(r'^  At the exit \(`([^`]+)` at (' + H + r')\), ([a-z0-9]+)=`([^`]+)`\.$')
UR = re.compile(r'^  Local unreached instructions: (' + H + r') to (' + H + r'), ending at (' + H + r') \(exclusive\)\. Count (\d+)\.$')
GAP = re.compile(r'^  Decoder coverage gap: bytes (' + H + r') to (' + H + r') \(exclusive\) remain raw and open\.$')
EW = re.compile(r'^  Effect warning \(`([^`]+)` at (' + H + r')\): unsupported effects invalidated\.$')
SHAPE = re.compile(r'^  Window shape: first `([^`]+)` at (' + H + r'), last `([^`]+)` at (' + H + r'), (\d+) decoded instructions\.$')
PROOF = re.compile(r'^  Entry call proof: (\d+) reached direct (?:call|calls) from complete caller windows, among (\d+) raw absolute `jsr` encodings of this target\.$')
NOPROOF = re.compile(r'^  Entry call proof: none among (\d+) raw absolute `jsr` encodings of this target\. The entry stays open and no function is admitted\.$')
SITE = re.compile(r'^  Entry call site: `([^`]+)` at (' + H + r') in complete caller window (' + H + r') to (' + H + r') \(exclusive\)\.$')
HEADER = re.compile(r'^- `(' + H + r')`, `([^`]+)`\.$')


def parse_sites(text):
    return [(m[1], a) for m in CITE.finditer(text) for a in re.findall(H, m[2])]


def validate(args):
    batch = load_batch(args.pool, args.index, args.batch_size)
    images = open_images(batch, args.work)
    text = open(args.md).read()
    lines = text.splitlines()
    starts = [i for i, s in enumerate(lines) if s.startswith('- ')]
    assert len(starts) == len(batch), 'entry count %d != batch %d' % (len(starts), len(batch))
    assert lines[0].startswith('# ') and lines[1] == '' and lines[2:2 + len(PREAMBLE)] == PREAMBLE, 'changed preamble'
    assert re.fullmatch(r'The \[full facts\]\([^)]+\) retain all instructions, constants and warnings\.', lines[3 + len(PREAMBLE)])
    assert lines[2 + len(PREAMBLE)] == '' and lines[4 + len(PREAMBLE)] == '' and 5 + len(PREAMBLE) == starts[0]
    facts_text = open(args.facts).read()
    counts, citations = collections.Counter(), 0
    for j, w in enumerate(batch):
        raw = lines[starts[j]:starts[j + 1] if j + 1 < len(starts) else len(lines)]
        while raw and not raw[-1]:
            raw.pop()
        assert all(raw) and all(x.startswith('  ') for x in raw[1:]), 'blank line inside entry'
        image = images[w['image']]
        meta = image.meta
        start, size = int(w['start'], 16), w['size']
        end = start + size
        # fresh extraction straight from the payload bytes
        f, render = image.facts(start, size)
        expected, framed = source_claims(f, render, start, end)
        window = image.window_bytes(start, size)
        m = HEADER.fullmatch(raw[0])
        assert m and m.groups() == (w['start'], meta['public_source']), 'header'
        fixed = [
            "  Package `%s`, version `%s`, load `%s`. Role: unknown." % (meta['image'], meta['version'], meta['load']),
            "  Inferred analysis window: %s to %s (exclusive), %d bytes." % (w['start'], hex(end), size),
            "  Carrier: %d bytes, SHA256 `%s`." % (meta['carrier_size'], meta['carrier_sha256']),
            "  Package: carrier offset %s, %d bytes, SHA256 `%s`." % (meta['package_offset'], meta['package_size'], meta['package_sha256']),
            "  Payload: carrier offset %s, %d bytes, SHA256 `%s`." % (meta['payload_offset_in_carrier'], meta['payload_size'], meta['payload_sha256']),
            "  Read window: SHA256 `%s`." % hashlib.sha256(window).hexdigest(),
        ]
        assert raw[1:7] == fixed, 'source metadata mismatch at ' + w['start']
        ins = {hex(z['addr']): z for z in f['instructions']}
        ur = set(f['unreached'])
        # an independent look at the bytes: every citation must be an instruction in the listing at that address
        pos = 7
        m = SHAPE.fullmatch(raw[pos])
        first, last = f['instructions'][0], f['instructions'][-1]
        assert m and m.groups() == (first['mn'], hex(first['addr']), last['mn'], hex(last['addr']), str(len(f['instructions']))), 'shape'
        pos += 1
        if not framed:
            assert raw[pos] == '  Frame linkage is not proved for this window. All access records are retained.'
            pos += 1
        else:
            assert raw[pos] != '  Frame linkage is not proved for this window. All access records are retained.'
        rawn = len(image.raw_calls.get(start, []))
        proofs = image.entry_proofs(start)
        if proofs:
            m = PROOF.fullmatch(raw[pos])
            assert m and m.groups() == (str(len(proofs)), str(rawn)), 'proof count'
            pos += 1
            for site, cs, ce, mn in proofs[:3]:
                m = SITE.fullmatch(raw[pos])
                assert m and m.groups() == (mn, hex(site), hex(cs), hex(ce)), 'proof site'
                # re-check the caller from bytes: LINK at start, terminal RTS, call target in the window bytes
                body, base = image.body, image.load
                assert body[cs - base:cs - base + 2] == b'\x4e\x56' and body[ce - base - 2:ce - base] == b'\x4e\x75'
                assert body[site - base:site - base + 2] == b'\x4e\xb9' and struct.unpack('>I', body[site - base + 2:site - base + 6])[0] == start
                pos += 1
            counts['call_proved'] += 1
        else:
            m = NOPROOF.fullmatch(raw[pos])
            assert m and m.groups() == (str(rawn),), 'no-proof line'
            pos += 1
            counts['open'] += 1
        seen = []
        for line in raw[pos:]:
            matches, kind = [], None
            if m := MEM.fullmatch(line):
                kind = 'memory'
                s = parse_sites(m[9])
                n = int(m[7])
                assert s and len(s) == min(n, 3) and (m[8] == 'access') == (n == 1) and m[9] == '`' + s[0][0] + '` at ' + ', '.join(a for mn, a in s)
                for i, c in enumerate(expected):
                    if c['kind'] != kind:
                        continue
                    a = c['records'][0]
                    wanted = (a['width'], KIND_WORDS[a['kind']], c['pattern'], c['target'], a['base'] or 'unknown',
                              fmt(a['value']) if a['kind'] == 'store' else None, len(c['records']),
                              [(z['mnemonic'], z['address']) for z in c['records'][:3]])
                    if (W[m[1]], m[2], int(m[3]), m[4], m[5], m[6], n, s) == wanted:
                        matches.append(i)
            elif m := CALL.fullmatch(line):
                kind = 'call'
                slots = re.findall(r'(stack arg[0-5])=`([^`]+)`', m[5])
                assert len(slots) == 6 and m[5] == ', '.join(k + '=`' + v + '`' for k, v in slots)
                for i, c in enumerate(expected):
                    if c['kind'] == kind:
                        a = c['record']
                        if (m[1], m[2], m[3], m[4], slots) == (a['mnemonic'], a['address'], a['callee'] or '(unknown callee)', a['target'] or 'unknown',
                                                               [(z['location'], fmt(z['value'])) for z in a['args']]):
                            matches.append(i)
            elif m := BR.fullmatch(line):
                kind = 'conditional'
                for i, c in enumerate(expected):
                    if c['kind'] == kind:
                        a = c['record']
                        if m.groups() == (c['mnemonic'], a['last'], a['target'] or 'unknown', a['taken_when'], a['falls_through_when']):
                            matches.append(i)
            elif m := JU.fullmatch(line):
                kind = 'jump'
                for i, c in enumerate(expected):
                    if c['kind'] == kind:
                        a = c['record']
                        edge = 'no edge' if a['target'] is None else 'a taken edge and no fallthrough edge'
                        if m.groups() == (c['mnemonic'], a['last'], a['target'] or 'unknown', edge):
                            matches.append(i)
            elif m := EX.fullmatch(line):
                kind = 'exit'
                for i, c in enumerate(expected):
                    if c['kind'] == kind:
                        a = c['record']
                        if m.groups() == (c['mnemonic'], a['address'], a['register'], fmt(a['value'])):
                            matches.append(i)
            elif m := UR.fullmatch(line):
                kind = 'unreached'
                for i, c in enumerate(expected):
                    if c['kind'] == kind and m.groups() == (c['record']['start'], c['record']['last'], c['record']['end'], str(c['record']['count'])):
                        matches.append(i)
            elif m := GAP.fullmatch(line):
                kind = 'gap'
                for i, c in enumerate(expected):
                    if c['kind'] == kind and m.groups() == (c['record']['start'], c['record']['end']):
                        matches.append(i)
            elif m := EW.fullmatch(line):
                kind = 'effectwarning'
                for i, c in enumerate(expected):
                    if c['kind'] == kind and m.groups() == (c['mnemonic'], c['address']):
                        matches.append(i)
            elif line == '  No reached exit is modeled.':
                kind = 'noexit'
                matches = [i for i, c in enumerate(expected) if c['kind'] == kind]
            else:
                raise ValueError('Unparsed statement: ' + line)
            assert len(matches) == 1, 'statement must match exactly one source fact: ' + line
            assert matches[0] not in seen, 'duplicate statement'
            seen.append(matches[0])
            counts[kind] += 1
            for mn, a in parse_sites(line):
                assert a in ins and a not in ur and ins[a]['mn'] == mn, 'instruction citation mismatch: ' + line
                citations += 1
        assert set(seen) == set(range(len(expected))), 'missing source statement in entry ' + w['start']
        # the facts file must hold this window's rendering
        assert public_render(render, meta) in facts_text, 'facts file lacks the window rendering'
    print(json.dumps(dict(entries=len(batch), statements=dict(counts), citations=citations)))


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest='cmd', required=True)
    for name in ('generate', 'validate'):
        p = sub.add_parser(name)
        p.add_argument('--pool', required=True)
        p.add_argument('--index', type=int, required=True)
        p.add_argument('--batch-size', type=int, default=250)
        p.add_argument('--work', required=True)
        if name == 'generate':
            p.add_argument('--out-md', required=True)
            p.add_argument('--out-facts', required=True)
            p.add_argument('--title', required=True)
        else:
            p.add_argument('--md', required=True)
            p.add_argument('--facts', required=True)
    args = ap.parse_args()
    {'generate': generate, 'validate': validate}[args.cmd](args)


if __name__ == '__main__':
    main()
