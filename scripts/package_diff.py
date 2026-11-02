#!/usr/bin/env python3
"""Routine and request-code diff of paired L1 packages in two shipped sets.

Usage: tools/re-run python3 scripts/package_diff.py [--out docs/version-diffs]

Pairs are chosen by package label, not by position: a package in the newer set is compared with
the package of the older set that carries the same product label. The pairs are listed in PAIRS;
each one is marked as label-matched or as a guess.

Writes under docs/version-diffs/:

  cd-ist-3.24-routines.tsv   routines added, removed or changed between each pair (same method as
                             version_diff.py: link.w a6 prologues, jsr and bsr targets, vector handlers)
  cd-ist-3.24-requests.tsv   general request codes accepted by each ColdFire pair's compare chain,
                             and whether each code's handler text matches the older set, differs
                             only in RAM operands, or differs otherwise (with the first difference
                             after RAM operands are masked)

Linux packages are not decoded here; their cramfs files are compared by hash in
scripts/linux_file_diff.py.
"""
import bisect
import collections
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import version_diff as vd  # noqa: E402
import sysco_tables as st  # noqa: E402

OUT = os.path.join(st.REPO, 'docs', 'version-diffs')
if '--out' in sys.argv:
    OUT = os.path.abspath(sys.argv[sys.argv.index('--out') + 1])

NEW, OLD = 'cd-ist-3.24', '1.48.1'
# (new key, new label, old release, old label, match) ; match is 'label' or 'guess'
PAIRS = [
    ('1.62.1 Legacy 2MB', 'Legacy 2MB image', 'patches/SG0007149', 'Legacy 2MB image', 'label'),
    ('1.62.1 2MB', '2MB image', 'patches/SG0007149', '2MB image', 'label'),
    ('1.62.1 NL3R 1MB', 'NL3R 1MB image', 'patches/SG0007149', 'Base 1MB image', 'guess'),
]


def package(rel, label):
    data = open(os.path.join(st.ROOT, rel, st.FW, 'l1.bin'), 'rb').read()
    hits = [p for p in st.l1_packages(data) if not p['linux'] and p['label'] == label]
    if len(hits) != 1:
        raise SystemExit('expected one %r in %s, found %d' % (label, rel, len(hits)))
    return hits[0]


def handler_lines(p, routine_list, target, valid_targets):
    """The handler's normalised text lines, as vd.handler_text reads them."""
    body, base = p['body'], p['load']
    starts = [r['addr'] for r in routine_list]
    i = bisect.bisect_right(starts, target) - 1
    e = routine_list[i]['end']
    nxt = [x for x in valid_targets if x > target]
    if nxt:
        e = min(e, nxt[0])
    text, pos = [], target
    while pos < e:
        ins = next(iter(vd.MD.disasm(body[pos - base:pos - base + 16], pos, count=1)), None)
        if ins:
            text.append((ins.mnemonic + ' ' + vd.ADDR_RE.sub('ADDR', ins.op_str)).strip())
            pos += ins.size
        else:
            text.append('dc.w $%04x' % int.from_bytes(body[pos - base:pos - base + 2], 'big'))
            pos += 2
    return text


def first_diff(a, b):
    i = vd.first_difference(a, b)
    return (a[i] if i < len(a) else '', b[i] if i < len(b) else '')


def main():
    os.makedirs(OUT, exist_ok=True)
    rtsv = [['pair', 'match', 'step', 'change', 'from_bank', 'from_addr', 'from_len', 'to_bank', 'to_addr',
             'to_len', 'similarity', 'first_difference_from', 'first_difference_to']]
    qtsv = [['pair', 'match', 'code', 'old_handler', 'new_handler', 'text', 'first_difference_old',
             'first_difference_new']]
    summary = collections.OrderedDict()
    for key, nlabel, orel, olabel, how in PAIRS:
        pn = package('cd-ist-3.24', nlabel)
        po = package(orel, olabel)
        old, new = '1.48.1 ' + olabel, '1.62.1 ' + nlabel
        rows = {old: vd.routines(po), new: vd.routines(pn)}
        sets = [(old, orel), (new, 'cd-ist-3.24')]
        tsv, summ = vd.routine_report(rows, sets)
        for r in tsv[1:]:
            rtsv.append([key, how] + r)
        summary[key] = dict(summ)
        # request codes
        cats, texts, lines = {}, {}, {}
        for name, p, rr in (('old', po, rows[old]), ('new', pn, rows[new])):
            unknown, valid = vd.dispatch_catalogue(p)
            cats[name] = valid
            if not valid:
                continue
            vt = sorted(set(valid.values()))
            texts[name] = {c: vd.handler_text(p, rr, t, vt) for c, t in valid.items()}
            lines[name] = {c: handler_lines(p, rr, t, vt) for c, t in valid.items()}
        summary[key]['chain_entries'] = {'old': vd.chain_entries(po['body'], po['load']),
                                         'new': vd.chain_entries(pn['body'], pn['load'])}
        if not cats['new'] and not cats['old']:
            summary[key]['requests'] = 'no compare chain in either package'
            continue
        for code in sorted(set(cats['old']) | set(cats['new'])):
            ho = '%x' % cats['old'][code] if code in cats['old'] else '-'
            hn = '%x' % cats['new'][code] if code in cats['new'] else '-'
            fo = fn = ''
            if code not in cats['old']:
                text = 'added'
            elif code not in cats['new']:
                text = 'removed'
            elif texts['old'][code][0] == texts['new'][code][0]:
                text = 'same text'
            else:
                # RAM operands (0x30xxxxxx) move between sets; a handler that differs only there is
                # reported as such, and the first difference left after masking RAM is the one read.
                ro, rn = vd.ram_norm(lines['old'][code]), vd.ram_norm(lines['new'][code])
                if ro == rn:
                    text = 'differs in RAM operands only'
                else:
                    text = 'text differs'
                    fo, fn = first_diff(ro, rn)
            qtsv.append([key, how, '%x' % code, ho, hn, text, fo, fn])
        summary[key]['requests'] = dict(collections.Counter(r[5] for r in qtsv if r[0] == key))
    vd.OUT = OUT
    vd.write_tsv('cd-ist-3.24-routines.tsv', rtsv)
    vd.write_tsv('cd-ist-3.24-requests.tsv', qtsv)
    import json
    print(json.dumps(summary, indent=1, default=str))


if __name__ == '__main__':
    main()
