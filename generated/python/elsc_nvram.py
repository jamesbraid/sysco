"""generated from devices/elsc_nvram/registers.toml by defs-gen - do not edit, change the source
configuration block of the module controller's non-volatile store"""

UNIT = 'elsc_nvram'
REGISTERS = {}
WORDS = {}
FIELDS = {}
ENUMS = {}
FIELD_ENUM = {}
CONSTANTS = {'CFG_CELL': {'value': 1805, 'count': 1, 'stride': 1}, 'CFG_CLUSTER': {'value': 1804, 'count': 1, 'stride': 1}, 'CFG_CONTROLLER': {'value': 1799, 'count': 1, 'stride': 1}, 'CFG_CONTROLLER_AUTO_POWER_BIT': {'value': 1, 'count': 1, 'stride': 1}, 'CFG_CONTROLLER_FANS_HIGH_BIT': {'value': 0, 'count': 1, 'stride': 1}, 'CFG_DOMAIN': {'value': 1803, 'count': 1, 'stride': 1}, 'CFG_PARTITION': {'value': 1802, 'count': 1, 'stride': 1}, 'CFG_PASSWORD': {'value': 1793, 'count': 1, 'stride': 1}, 'CFG_PASSWORD_LEN': {'value': 4, 'count': 1, 'stride': 1}, 'CFG_RESERVED_FIRST': {'value': 1806, 'count': 1, 'stride': 1}, 'CFG_SELFTEST': {'value': 1801, 'count': 1, 'stride': 1}, 'CFG_SWITCH_PHYSICAL_XOR': {'value': 1798, 'count': 1, 'stride': 1}, 'CFG_SWITCH_VIRTUAL_XOR': {'value': 1797, 'count': 1, 'stride': 1}}
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
