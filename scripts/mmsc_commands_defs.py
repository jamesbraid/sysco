"""Keep the command-table units in step with the tables they come from.

Loaded by retools/tools/defs through the plugin key in the repo's check settings. A plugin
reads the model of each unit (hdr.unit) when the checker supplies one, and the header text of
the older decode-header form otherwise, so it runs under both while the repository moves.

mmsc_commands (MMSC_CMD_DECODE.h) against docs/mmsc/commands.tsv: each command word in the
table must appear once as a CMD.index enum value, at the table position the TSV gives. Aliases
stay in the table; the help alias is the one name that is not an identifier and has no enum
value.

l1_pkg<n>_console (L1P<n>_CONSOLE_DECODE.h) against docs/l1/command-table.tsv: every row of
the package must have one CMD<t>.index enum value at the row's position, whose comment (the doc
of the enum value in the model) repeats the row's typed pattern and marks it hidden when the
row is, and the unit may hold no other position.
"""
import csv
import io
import re

HEADER = 'MMSC_CMD_DECODE.h'
UNIT = 'mmsc_commands'
TABLE = 'docs/mmsc/commands.tsv'
L1_TABLE = 'docs/l1/command-table.tsv'
L1_HEADER = re.compile(r'L1P(\d+)_CONSOLE_DECODE\.h')
L1_UNIT = re.compile(r'l1_pkg(\d+)_console')


def check_header(ctx, hdr):
    unit = getattr(hdr, 'unit', None)
    if unit is None:
        # The older header grammar: the checker hands over the header text.
        if hdr.file == HEADER:
            check_mmsc_text(ctx, hdr)
            return
        m = L1_HEADER.fullmatch(hdr.file)
        if m:
            check_l1_console_text(ctx, hdr, int(m.group(1)))
        return
    if unit.name == UNIT:
        check_mmsc_model(ctx, hdr)
        return
    m = L1_UNIT.fullmatch(unit.name)
    if m:
        check_l1_console_model(ctx, hdr, int(m.group(1)))


def check_mmsc_text(ctx, hdr):
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


def check_l1_console_text(ctx, hdr, n):
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


def check_mmsc_model(ctx, hdr):
    rows = list(csv.DictReader(io.StringIO(ctx.read(TABLE)), delimiter='\t'))
    want = {}
    for r in rows:
        if r['name'] == '?':
            continue
        want[r['name'].lower()] = int(r['index'])
    have = {label: val for (target, label), val in hdr.enums.items() if target == 'CMD.index'}
    for label, val in sorted(want.items()):
        if label not in have:
            ctx.rep.fail(hdr.file, 'command %s is in %s but has no CMD.index ENUM' % (label, TABLE))
        elif have[label] != val:
            ctx.rep.fail(hdr.file, 'command %s is at position %d in %s but the ENUM says %d'
                         % (label, val, TABLE, have[label]))
    for label in sorted(set(have) - set(want)):
        ctx.rep.fail(hdr.file, 'ENUM %s has no row in %s' % (label, TABLE))


def check_l1_console_model(ctx, hdr, n):
    package = '6.5.30:l1#%d' % n
    rows = [r for r in csv.DictReader(io.StringIO(ctx.read(L1_TABLE)), delimiter='\t')
            if r['package'] == package]
    if not rows:
        ctx.rep.fail(hdr.file, 'no rows for package %s in %s' % (package, L1_TABLE))
        return
    want = {(int(r['table']), int(r['index'])): r for r in rows}
    have = {}
    for name, word in sorted(hdr.unit.model['words'].items()):
        m = re.fullmatch(r'CMD(\d+)', name)
        if not m or 'index' not in word['fields']:
            continue
        for lab, e in word['fields']['index'].get('enums', {}).items():
            key = (int(m.group(1)), e['value'])
            if key in have:
                ctx.rep.fail(hdr.file, 'position %d of table %d is listed twice' % (key[1], key[0]))
            have[key] = e.get('doc', '')
    for key, r in sorted(want.items()):
        if key not in have:
            ctx.rep.fail(hdr.file, 'table %d position %d is in %s but has no ENUM' % (key[0], key[1], L1_TABLE))
            continue
        c = '%s%s' % (r['pattern'], ' (hidden)' if r['hidden'] == '1' else '')
        if have[key] != c:
            ctx.rep.fail(hdr.file, 'table %d position %d: doc %r does not match the row %r'
                         % (key[0], key[1], have[key], c))
    for key in sorted(set(have) - set(want)):
        ctx.rep.fail(hdr.file, 'table %d position %d has no row in %s' % (key[0], key[1], L1_TABLE))
