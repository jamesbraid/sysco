"""Keep the command-table units in step with the tables they come from.

Loaded by retools/tools/defs/defs-check through the plugin key in defs/check.toml.

defs/mmsc_commands.toml against docs/mmsc/commands.tsv: each command word in the table must appear once
as a CMD.index enum value, at the table position the TSV gives. Aliases stay in the table; the help
alias is the one name that is not an identifier and has no enum value.

defs/l1_pkg<n>_console.toml against docs/l1/command-table.tsv: every row of the package must have one
CMD<t>.index enum value at the row's position, whose doc repeats the row's typed pattern (and marks
it hidden when the row is), and the unit may hold no other position.
"""
import csv
import io
import re

UNIT = 'mmsc_commands'
TABLE = 'docs/mmsc/commands.tsv'
L1_TABLE = 'docs/l1/command-table.tsv'
L1_UNIT = re.compile(r'l1_pkg(\d+)_console')


def check_header(ctx, hdr):
    name = hdr.unit.name
    if name == UNIT:
        check_mmsc(ctx, hdr)
        return
    m = L1_UNIT.fullmatch(name)
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
            ctx.rep.fail(hdr.file, 'command %s is in %s but has no CMD.index ENUM' % (label, TABLE))
        elif have[label] != val:
            ctx.rep.fail(hdr.file, 'command %s is at position %d in %s but the ENUM says %d'
                         % (label, val, TABLE, have[label]))
    for label in sorted(set(have) - set(want)):
        ctx.rep.fail(hdr.file, 'ENUM %s has no row in %s' % (label, TABLE))


def check_l1_console(ctx, hdr, n):
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
