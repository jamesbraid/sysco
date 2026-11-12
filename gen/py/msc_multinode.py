"""generated from defs/msc_multinode.toml by defs-gen - do not edit, change the source
constants of the multi-node console and master election"""

UNIT = 'msc_multinode'
REGISTERS = {}
WORDS = {}
FIELDS = {}
ENUMS = {}
FIELD_ENUM = {}
CONSTANTS = {'MODULE_RECORD_FAMILY': {'value': 9, 'count': 1, 'stride': 1}, 'NI_SCRATCH_IDENTITY': {'value': 0, 'count': 1, 'stride': 1}, 'NI_SCRATCH_STATUS': {'value': 1, 'count': 1, 'stride': 1}, 'NODE_RECORD_FAMILY': {'value': 11, 'count': 1, 'stride': 1}, 'SLICE_LETTER_BASE': {'value': 65, 'count': 1, 'stride': 1}}
TESTS = []


def decode(name, value):
    """Field name -> value for a register or word."""
    return {f: (value >> lo) & ((1 << (hi - lo + 1)) - 1) for f, (hi, lo) in FIELDS[name].items()}


def encode(name, **fields):
    v = 0
    for f, x in fields.items():
        hi, lo = FIELDS[name][f]
        if x >> (hi - lo + 1):
            raise ValueError('%s.%s: %d does not fit' % (name, f, x))
        v |= x << lo
    return v


def describe(name, value):
    """decode, with enum labels where the field has an enum."""
    out = {}
    for f, x in decode(name, value).items():
        e = ENUMS.get(FIELD_ENUM.get('%s.%s' % (name, f)))
        out[f] = e.get(x, x) if e else x
    return out


def selftest():
    for name, value, expect in TESTS:
        got = decode(name, value)
        for f, want in expect.items():
            assert got[f] == want, (name, hex(value), f, got[f], want)
    return len(TESTS)


if __name__ == '__main__':
    print('%s: %d vectors ok' % (UNIT, selftest()))
