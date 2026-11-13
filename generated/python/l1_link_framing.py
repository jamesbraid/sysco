"""generated from formats/l1_link_framing/format.toml by defs-gen - do not edit, change the source
frame bytes and link context constants of the ColdFire L1 node and peer links"""

UNIT = 'l1_link_framing'
REGISTERS = {}
WORDS = {}
FIELDS = {}
ENUMS = {}
FIELD_ENUM = {}
CONSTANTS = {'FRAME_CHECK_RECEIVE': {'value': 61624, 'count': 1, 'stride': 1}, 'FRAME_CHECK_SEED': {'value': 65535, 'count': 1, 'stride': 1}, 'FRAME_ESCAPE': {'value': 125, 'count': 1, 'stride': 1}, 'FRAME_ESCAPE_XOR': {'value': 32, 'count': 1, 'stride': 1}, 'FRAME_FLAG': {'value': 126, 'count': 1, 'stride': 1}, 'NODE_BODY_CAP': {'value': 256, 'count': 1, 'stride': 1}, 'PEER_BODY_OFFSET': {'value': 16, 'count': 1, 'stride': 1}, 'PEER_ESCAPE_TX_A': {'value': 17, 'count': 1, 'stride': 1}, 'PEER_ESCAPE_TX_B': {'value': 19, 'count': 1, 'stride': 1}, 'PEER_GROWTH_LIMIT': {'value': 1040, 'count': 1, 'stride': 1}}
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
