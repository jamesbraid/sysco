"""generated from defs/l2_router_requests.toml by defs-gen - do not edit, change the source
request codes of the L2 router library"""

UNIT = 'l2_router_requests'
REGISTERS = {}
WORDS = {'CMD': {'width': 32, 'grade': 'established'}}
FIELDS = {'CMD': {'code': (15, 0)}}
ENUMS = {'CMD.code': {12289: 'chunk_first_flagged', 12290: 'chunk_first', 12291: 'chunk_next'}}
FIELD_ENUM = {'CMD.code': 'CMD.code'}
CONSTANTS = {}
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
