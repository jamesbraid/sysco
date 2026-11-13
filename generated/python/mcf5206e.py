"""generated from devices/mcf5206e/registers.toml by defs-gen - do not edit, change the source
on-chip registers of the ColdFire MCF5206e as the L1 firmware uses them"""

UNIT = 'mcf5206e'
REGISTERS = {}
WORDS = {'CSMR0': {'width': 32, 'grade': 'established'}, 'CSMR2': {'width': 32, 'grade': 'established'}, 'MADR': {'width': 8, 'grade': 'inferred'}, 'MBCR': {'width': 8, 'grade': 'inferred'}, 'MBDR': {'width': 8, 'grade': 'inferred'}, 'MBSR': {'width': 32, 'grade': 'established'}, 'MFDR': {'width': 8, 'grade': 'inferred'}, 'PADAT': {'width': 32, 'grade': 'established'}, 'PAR': {'width': 32, 'grade': 'established'}, 'RSR': {'width': 32, 'grade': 'established'}, 'SYPCR': {'width': 32, 'grade': 'established'}}
FIELDS = {'CSMR0': {'bam': (31, 16)}, 'CSMR2': {'bam': (31, 16)}, 'MADR': {'adr': (7, 1)}, 'MBCR': {'men': (7, 7), 'mien': (6, 6), 'msta': (5, 5), 'mtx': (4, 4), 'txak': (3, 3), 'rsta': (2, 2)}, 'MBDR': {'data': (7, 0)}, 'MBSR': {'mcf': (7, 7), 'maas': (6, 6), 'bus_busy': (5, 5), 'mal': (4, 4), 'srw': (2, 2), 'mif': (1, 1), 'rxak': (0, 0)}, 'MFDR': {'mbc': (5, 0)}, 'PADAT': {'tdi': (7, 7), 'tck': (6, 6), 'tms': (5, 5), 'trst': (4, 4), 'psi': (3, 3), 'tdo': (0, 0)}, 'PAR': {'timer_pulse': (7, 7), 'scan_sel': (5, 4)}, 'RSR': {'cause5': (5, 5)}, 'SYPCR': {'swe': (7, 7)}}
ENUMS = {'CSMR0.bam': {15: 'size_1mb', 31: 'size_2mb'}, 'CSMR2.bam': {1: 'size_128kb', 7: 'size_512kb'}, 'MFDR.mbc': {0: 'divide_28', 7: 'divide_68', 19: 'divide_480', 32: 'divide_20', 63: 'divide_2048'}}
FIELD_ENUM = {'CSMR0.bam': 'CSMR0.bam', 'CSMR2.bam': 'CSMR2.bam', 'MFDR.mbc': 'MFDR.mbc'}
CONSTANTS = {'CS0_START_CSAR': {'value': 65504, 'count': 1, 'stride': 1}, 'CS0_START_CSCR': {'value': 4483, 'count': 1, 'stride': 1}, 'CS1_START_CSAR': {'value': 8192, 'count': 1, 'stride': 1}, 'CS1_START_CSCR': {'value': 4419, 'count': 1, 'stride': 1}, 'CS1_START_CSMR': {'value': 0, 'count': 1, 'stride': 1}, 'CS2_START_CSAR': {'value': 12288, 'count': 1, 'stride': 1}, 'CS2_START_CSCR': {'value': 2435, 'count': 1, 'stride': 1}, 'CS3_START_CSAR': {'value': 16384, 'count': 1, 'stride': 1}, 'CS3_START_CSCR': {'value': 2387, 'count': 1, 'stride': 1}, 'CS3_START_CSMR': {'value': 0, 'count': 1, 'stride': 1}, 'CS4_START_CSAR': {'value': 20480, 'count': 1, 'stride': 1}, 'CS4_START_CSCR': {'value': 2399, 'count': 1, 'stride': 1}, 'CS4_START_CSMR': {'value': 0, 'count': 1, 'stride': 1}, 'DMCR_START_VALUE': {'value': 0, 'count': 1, 'stride': 1}, 'ICR10_START_VALUE': {'value': 128, 'count': 1, 'stride': 1}, 'ICR11_RESET_VALUE': {'value': 128, 'count': 1, 'stride': 1}, 'ICR11_START_VALUE': {'value': 153, 'count': 1, 'stride': 1}, 'ICR12_START_VALUE': {'value': 26, 'count': 1, 'stride': 1}, 'ICR13_START_VALUE': {'value': 0, 'count': 1, 'stride': 1}, 'ICR14_START_VALUE': {'value': 9, 'count': 1, 'stride': 1}, 'ICR15_START_VALUE': {'value': 10, 'count': 1, 'stride': 1}, 'ICR1_START_VALUE': {'value': 132, 'count': 1, 'stride': 1}, 'ICR2_START_VALUE': {'value': 136, 'count': 1, 'stride': 1}, 'ICR3_START_VALUE': {'value': 128, 'count': 1, 'stride': 1}, 'ICR4_START_VALUE': {'value': 144, 'count': 1, 'stride': 1}, 'ICR5_START_VALUE': {'value': 128, 'count': 1, 'stride': 1}, 'ICR6_START_VALUE': {'value': 152, 'count': 1, 'stride': 1}, 'ICR7_START_VALUE': {'value': 156, 'count': 1, 'stride': 1}, 'ICR8_START_VALUE': {'value': 29, 'count': 1, 'stride': 1}, 'ICR9_START_VALUE': {'value': 148, 'count': 1, 'stride': 1}, 'IMR_START_VALUE': {'value': 65534, 'count': 1, 'stride': 1}, 'MBSR_RESET_VALUE': {'value': 129, 'count': 1, 'stride': 1}, 'MFDR_INIT': {'value': 19, 'count': 1, 'stride': 1}, 'OFFSET_CSAR0': {'value': 100, 'count': 1, 'stride': 1}, 'OFFSET_CSAR1': {'value': 112, 'count': 1, 'stride': 1}, 'OFFSET_CSAR2': {'value': 124, 'count': 1, 'stride': 1}, 'OFFSET_CSAR3': {'value': 136, 'count': 1, 'stride': 1}, 'OFFSET_CSAR4': {'value': 148, 'count': 1, 'stride': 1}, 'OFFSET_CSAR5': {'value': 160, 'count': 1, 'stride': 1}, 'OFFSET_CSCR0': {'value': 110, 'count': 1, 'stride': 1}, 'OFFSET_CSCR1': {'value': 122, 'count': 1, 'stride': 1}, 'OFFSET_CSCR2': {'value': 134, 'count': 1, 'stride': 1}, 'OFFSET_CSCR3': {'value': 146, 'count': 1, 'stride': 1}, 'OFFSET_CSCR4': {'value': 158, 'count': 1, 'stride': 1}, 'OFFSET_CSCR5': {'value': 170, 'count': 1, 'stride': 1}, 'OFFSET_CSMR0': {'value': 104, 'count': 1, 'stride': 1}, 'OFFSET_CSMR1': {'value': 116, 'count': 1, 'stride': 1}, 'OFFSET_CSMR2': {'value': 128, 'count': 1, 'stride': 1}, 'OFFSET_CSMR3': {'value': 140, 'count': 1, 'stride': 1}, 'OFFSET_CSMR4': {'value': 152, 'count': 1, 'stride': 1}, 'OFFSET_CSMR5': {'value': 164, 'count': 1, 'stride': 1}, 'OFFSET_DMCR': {'value': 198, 'count': 1, 'stride': 1}, 'OFFSET_ICR1': {'value': 20, 'count': 1, 'stride': 1}, 'OFFSET_ICR10': {'value': 29, 'count': 1, 'stride': 1}, 'OFFSET_ICR11': {'value': 30, 'count': 1, 'stride': 1}, 'OFFSET_ICR12': {'value': 31, 'count': 1, 'stride': 1}, 'OFFSET_ICR13': {'value': 32, 'count': 1, 'stride': 1}, 'OFFSET_ICR14': {'value': 33, 'count': 1, 'stride': 1}, 'OFFSET_ICR15': {'value': 34, 'count': 1, 'stride': 1}, 'OFFSET_ICR2': {'value': 21, 'count': 1, 'stride': 1}, 'OFFSET_ICR3': {'value': 22, 'count': 1, 'stride': 1}, 'OFFSET_ICR4': {'value': 23, 'count': 1, 'stride': 1}, 'OFFSET_ICR5': {'value': 24, 'count': 1, 'stride': 1}, 'OFFSET_ICR6': {'value': 25, 'count': 1, 'stride': 1}, 'OFFSET_ICR7': {'value': 26, 'count': 1, 'stride': 1}, 'OFFSET_ICR8': {'value': 27, 'count': 1, 'stride': 1}, 'OFFSET_ICR9': {'value': 28, 'count': 1, 'stride': 1}, 'OFFSET_IMR': {'value': 54, 'count': 1, 'stride': 1}, 'OFFSET_MADR': {'value': 480, 'count': 1, 'stride': 1}, 'OFFSET_MBCR': {'value': 488, 'count': 1, 'stride': 1}, 'OFFSET_MBDR': {'value': 496, 'count': 1, 'stride': 1}, 'OFFSET_MBSR': {'value': 492, 'count': 1, 'stride': 1}, 'OFFSET_MFDR': {'value': 484, 'count': 1, 'stride': 1}, 'OFFSET_PADAT': {'value': 457, 'count': 1, 'stride': 1}, 'OFFSET_PADDR': {'value': 453, 'count': 1, 'stride': 1}, 'OFFSET_PAR': {'value': 202, 'count': 1, 'stride': 1}, 'OFFSET_RSR': {'value': 64, 'count': 1, 'stride': 1}, 'OFFSET_SIMR': {'value': 3, 'count': 1, 'stride': 1}, 'OFFSET_SWIVR': {'value': 66, 'count': 1, 'stride': 1}, 'OFFSET_SWSR': {'value': 67, 'count': 1, 'stride': 1}, 'OFFSET_SYPCR': {'value': 65, 'count': 1, 'stride': 1}, 'OFFSET_TMR1': {'value': 256, 'count': 1, 'stride': 1}, 'OFFSET_TMR2': {'value': 288, 'count': 1, 'stride': 1}, 'OFFSET_TMR_AUX_A': {'value': 440, 'count': 1, 'stride': 1}, 'OFFSET_TMR_AUX_B': {'value': 444, 'count': 1, 'stride': 1}, 'OFFSET_TRR1': {'value': 260, 'count': 1, 'stride': 1}, 'OFFSET_TRR2': {'value': 292, 'count': 1, 'stride': 1}, 'OFFSET_UART1': {'value': 320, 'count': 1, 'stride': 1}, 'PADDR_SCAN_SETUP': {'value': 254, 'count': 1, 'stride': 1}, 'SIMR_START_VALUE': {'value': 192, 'count': 1, 'stride': 1}, 'SWSR_SERVICE_FIRST': {'value': 85, 'count': 1, 'stride': 1}, 'SWSR_SERVICE_SECOND': {'value': 170, 'count': 1, 'stride': 1}, 'TMR1_INIT': {'value': 29, 'count': 1, 'stride': 1}, 'TMR2_INIT': {'value': 43, 'count': 1, 'stride': 1}, 'TRR1_TICK_REFERENCE': {'value': 29999, 'count': 1, 'stride': 1}, 'TRR2_REFERENCE': {'value': 1, 'count': 1, 'stride': 1}, 'UART1_VECTOR': {'value': 66, 'count': 1, 'stride': 1}}
TESTS = [('CSMR0', 983040, {'bam': 15}), ('CSMR0', 2031616, {'bam': 31}), ('CSMR2', 65536, {'bam': 1}), ('CSMR2', 458752, {'bam': 7}), ('MADR', 88, {'adr': 44}), ('MBCR', 240, {'men': 1, 'mien': 1, 'msta': 1, 'mtx': 1, 'txak': 0, 'rsta': 0}), ('MBSR', 129, {'mcf': 1, 'rxak': 1}), ('MFDR', 19, {'mbc': 19}), ('SYPCR', 254, {'swe': 1})]


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
