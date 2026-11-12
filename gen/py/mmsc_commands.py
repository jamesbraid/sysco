"""generated from defs/mmsc_commands.toml by defs-gen - do not edit, change the source
typed command table of the multi-module system"""

UNIT = 'mmsc_commands'
REGISTERS = {}
WORDS = {'AUTH': {'width': 32, 'grade': 'established'}, 'CMD': {'width': 32, 'grade': 'established'}, 'GATE': {'width': 32, 'grade': 'established'}}
FIELDS = {'AUTH': {'level': (15, 0)}, 'CMD': {'index': (7, 0)}, 'GATE': {'level': (15, 0)}}
ENUMS = {'AUTH.level': {100: 'basic', 500: 'supervisor', 1000: 'service'}, 'CMD.index': {0: 'auth', 1: 'authority', 2: 'bs', 3: 'cecho', 4: 'com', 5: 'cons', 6: 'console', 7: 'debug', 8: 'numracks', 9: 'dest', 10: 'direct', 11: 'downloader', 12: 'elsc', 13: 'emsg', 14: 'end', 15: 'esc', 16: 'exit', 17: 'ffsc', 18: 'flash', 19: 'graphics_command', 20: 'help', 21: 'ipaddr', 22: 'kill', 23: 'log', 24: 'mem', 25: 'mmsc', 26: 'mmsg', 27: 'msc', 28: 'nap_time', 29: 'nop', 30: 'options', 31: 'pager', 32: 'pas', 33: 'password', 34: 'printenv', 35: 'pwr', 36: 'rackid', 37: 'rat', 38: 'reboot_ffsc', 39: 'reset_ffsc', 40: 'reset_mmsc', 41: 'reset_nvram', 42: 'rmsg', 43: 'scan', 44: 'setenv', 45: 'steal', 46: 'partlst', 47: 'partscan', 48: 'partget', 49: 'partnam', 50: 'partclr', 51: 'partset', 52: 'swapcons', 53: 'tty_reinit', 54: 'unsetenv', 55: 'unsteal', 56: 'ver', 58: 'arbiter', 59: 'macaddr'}, 'GATE.level': {499: 'above_499', 999: 'above_999'}}
FIELD_ENUM = {'AUTH.level': 'AUTH.level', 'CMD.index': 'CMD.index', 'GATE.level': 'GATE.level'}
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
