"""generated from formats/l2_partition_requests/format.toml by defs-gen - do not edit, change the source
request codes of the L2 partition task image"""

UNIT = 'l2_partition_requests'
REGISTERS = {}
WORDS = {'CMD': {'width': 32, 'grade': 'established'}}
FIELDS = {'CMD': {'code': (15, 0)}}
ENUMS = {'CMD.code': {1: 'partition_for_brick', 2: 'partition_environment', 3: 'database_get', 4: 'database_time', 5: 'database_set', 6: 'mode_set', 7: 'park_request', 8: 'release_parked'}}
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
