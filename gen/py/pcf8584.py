"""generated from defs/pcf8584.toml by defs-gen - do not edit, change the source
control bits and clock-control fields of the PCF8584 I2C-bus controller"""

UNIT = 'pcf8584'
REGISTERS = {}
WORDS = {}
FIELDS = {}
ENUMS = {}
FIELD_ENUM = {}
CONSTANTS = {'CLOCK_OSC_12MHZ': {'value': 28, 'count': 1, 'stride': 1}, 'CLOCK_OSC_3MHZ': {'value': 0, 'count': 1, 'stride': 1}, 'CLOCK_OSC_4_43MHZ': {'value': 16, 'count': 1, 'stride': 1}, 'CLOCK_OSC_6MHZ': {'value': 20, 'count': 1, 'stride': 1}, 'CLOCK_OSC_8MHZ': {'value': 24, 'count': 1, 'stride': 1}, 'CLOCK_SCL_11KHZ': {'value': 2, 'count': 1, 'stride': 1}, 'CLOCK_SCL_1_5KHZ': {'value': 3, 'count': 1, 'stride': 1}, 'CLOCK_SCL_45KHZ': {'value': 1, 'count': 1, 'stride': 1}, 'CLOCK_SCL_90KHZ': {'value': 0, 'count': 1, 'stride': 1}, 'CTRL_ES1': {'value': 32, 'count': 1, 'stride': 1}, 'CTRL_ESO': {'value': 64, 'count': 1, 'stride': 1}, 'CTRL_PIN': {'value': 128, 'count': 1, 'stride': 1}, 'OWN_ADDRESS_MASK': {'value': 127, 'count': 1, 'stride': 1}}
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
