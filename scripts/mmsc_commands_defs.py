"""Keep MMSC_CMD_DECODE.h in step with the command table in docs/mmsc/commands.tsv.

Loaded by retools/tools/defs/check-publish-safe.py through the plugin key in defs.toml.
Each command word in the table must appear once as a CMD.index ENUM, at the table
position the TSV gives. Aliases stay in the table; the help alias is the one name
that is not an identifier and has no ENUM.
"""
import csv
import io

HEADER = 'MMSC_CMD_DECODE.h'
TABLE = 'docs/mmsc/commands.tsv'


def check_header(ctx, hdr):
    if hdr.file != HEADER:
        return
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
