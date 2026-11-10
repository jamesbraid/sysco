"""Keep the command-table decode headers in step with the tables they come from.

Loaded by retools/tools/defs/check-publish-safe.py through the plugin key in defs.toml.

MMSC_CMD_DECODE.h against docs/mmsc/commands.tsv: each command word in the table must appear once
as a CMD.index ENUM, at the table position the TSV gives. Aliases stay in the table; the help
alias is the one name that is not an identifier and has no ENUM.

L1P<n>_CONSOLE_DECODE.h against docs/l1/command-table.tsv: every row of the package must have one
CMD<t>.index ENUM at the row's position, directly under a comment that repeats the row's typed
pattern (and marks it hidden when the row is), and the header may hold no other position.
"""
import csv
import io
import re

HEADER = 'MMSC_CMD_DECODE.h'
TABLE = 'docs/mmsc/commands.tsv'
L1_TABLE = 'docs/l1/command-table.tsv'
L1_HEADER = re.compile(r'L1P(\d+)_CONSOLE_DECODE\.h')


def check_header(ctx, hdr):
    if hdr.file == HEADER:
        check_mmsc(ctx, hdr)
        return
    m = L1_HEADER.fullmatch(hdr.file)
    if m:
        check_l1_console(ctx, hdr, int(m.group(1)))


def check_mmsc(ctx, hdr):
    rows = list(csv.DictReader(io.StringIO(ctx.read(TABLE)), delimiter='\t'))
    want = {}
    for r in rows:
        if r['name'] == '?':
            continue
        want[r['name'].lower()] = int(r['index'])
    have = {label: val for (target, label), val in hdr.enums.items() if target == 'CMD.index'}
    for label, val in sorted(want.items()):
        if label not in have:
            ctx.rep.fail(HEADER, 'command %s is in %s but has no CMD.index ENUM' % (label, TABLE))
        elif have[label] != val:
            ctx.rep.fail(HEADER, 'command %s is at position %d in %s but the ENUM says %d'
                         % (label, val, TABLE, have[label]))
    for label in sorted(set(have) - set(want)):
        ctx.rep.fail(HEADER, 'ENUM %s has no row in %s' % (label, TABLE))


def check_l1_console(ctx, hdr, n):
    package = '6.5.30:l1#%d' % n
    rows = [r for r in csv.DictReader(io.StringIO(ctx.read(L1_TABLE)), delimiter='\t')
            if r['package'] == package]
    if not rows:
        ctx.rep.fail(hdr.file, 'no rows for package %s in %s' % (package, L1_TABLE))
        return
    want = {(int(r['table']), int(r['index'])): r for r in rows}
    lines = hdr.text.splitlines()
    have = {}
    for i, ln in enumerate(lines):
        m = re.match(r'/\* ENUM: CMD(\d+)\.index (\d+) \S+ \w+ \*/$', ln)
        if not m:
            continue
        key = (int(m.group(1)), int(m.group(2)))
        if key in have:
            ctx.rep.fail(hdr.file, 'position %d of table %d is listed twice' % (key[1], key[0]))
        have[key] = lines[i - 1] if i else ''
    for key, r in sorted(want.items()):
        if key not in have:
            ctx.rep.fail(hdr.file, 'table %d position %d is in %s but has no ENUM' % (key[0], key[1], L1_TABLE))
            continue
        c = '/* typed %s%s */' % (r['pattern'], ' (hidden)' if r['hidden'] == '1' else '')
        if have[key] != c:
            ctx.rep.fail(hdr.file, 'table %d position %d: comment %r does not match the row %r'
                         % (key[0], key[1], have[key], c))
    for key in sorted(set(have) - set(want)):
        ctx.rep.fail(hdr.file, 'table %d position %d has no row in %s' % (key[0], key[1], L1_TABLE))
