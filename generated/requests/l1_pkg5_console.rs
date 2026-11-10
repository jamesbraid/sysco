// generated from sysco c0c9fa1403a4 L1P5_CONSOLE_DECODE.h - do not edit, regenerate in sysco
//!
//! Position of each console command in the registered command tables
//! of one ColdFire L1 firmware package.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// Values of `index` in `Cmd0` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd0Index {
    /// established.
    AllCommand,
    /// established.
    RackSlotCommand,
    /// established.
    Autopower,
    /// established.
    AutopowerOn,
    /// established.
    AutopowerOff,
    /// established.
    AutopowerAbort,
    /// established.
    SysLink,
    /// established.
    SysLinkNone,
    /// established.
    SysLinkPpp,
    /// established.
    SysLinkEnable,
    /// established.
    SysLinkReset,
    /// established.
    SysLinkPackets,
    /// established.
    Brick,
    /// established.
    BrickTypeWord,
    /// established.
    BrickRackDecimal,
    /// established.
    BrickSlotDecimal,
    /// established.
    BrickRackslotDecimalDecimal,
    /// established.
    BrickRackslotResetNumberNumber,
    /// established.
    BrickPartitionNone,
    /// established.
    BrickPartitionNumber,
    /// established.
    Partdb,
    /// established.
    PartdbEnable,
    /// established.
    PartdbDisable,
    /// established.
    Cpu,
    /// established.
    CpuE,
    /// established.
    CpuWordE,
    /// established.
    B2bWordCommand,
    /// established.
    Config,
    /// established.
    ConfigV,
    /// established.
    ConfigReset,
    /// established.
    ConfigRing,
    /// established.
    Debug,
    /// established.
    DebugNumber,
    /// established.
    Display,
    /// established.
    DisplayNumberCommand,
    /// established.
    DisplayPowerOn,
    /// established.
    DisplayAttentionOn,
    /// established.
    DisplayFailureOn,
    /// established.
    DisplayPowerOff,
    /// established.
    DisplayAttentionOff,
    /// established.
    DisplayFailureOff,
    /// established.
    Button,
    /// established.
    ButtonAllEnable,
    /// established.
    ButtonAllDisable,
    /// established.
    ButtonPwrEnable,
    /// established.
    ButtonPwrDisable,
    /// established.
    ButtonRstEnable,
    /// established.
    ButtonRstDisable,
    /// established.
    ButtonNmiEnable,
    /// established.
    ButtonNmiDisable,
    /// established.
    LetTheCarnageBegin,
    /// established.
    SlippedMyMind,
    /// established.
    SuperSizedPlease,
    /// established.
    Env,
    /// established.
    EnvCheck,
    /// established.
    EnvOn,
    /// established.
    EnvOff,
    /// established.
    EnvPowerOn,
    /// established.
    EnvPowerOff,
    /// established.
    EnvPowerMonitor,
    /// established.
    EnvTempOn,
    /// established.
    EnvTempOff,
    /// established.
    EnvAltitudeHigh,
    /// established.
    EnvAltitudeLow,
    /// established.
    EnvReset,
    /// established.
    EnvTestWarning,
    /// established.
    EnvTestFault,
    /// established.
    EnvTestTmpadv,
    /// established.
    EnvTestTmpcrit,
    /// established.
    EnvTestNumberNumberNumberNumberNumber,
    /// established.
    EnvTestPwrdn,
    /// established.
    EnvMessages,
    /// established.
    EnvMessagesFaultsonly,
    /// established.
    EnvMessagesAll,
    /// established.
    Fan,
    /// established.
    FanOnNumber,
    /// established.
    FanOn,
    /// established.
    FanSpeed,
    /// established.
    FanSpeedNumberNumber,
    /// established.
    FanSpeedNumber,
    /// established.
    FanSpeedLowNumber,
    /// established.
    FanSpeedLow,
    /// established.
    FanAutoOn,
    /// established.
    Help,
    /// established.
    HelpWord,
    /// established.
    HelpListHidden,
    /// established.
    HelpWordHidden,
    /// established.
    History,
    /// established.
    L1dbg,
    /// established.
    L1dbgSysLinkNumber,
    /// established.
    L1dbgSysLinkNumberNumber,
    /// established.
    L1dbgIrtrNumber,
    /// established.
    L1dbgEnvNumber,
    /// established.
    L1dbgEnvOff,
    /// established.
    L1dbgPortOn,
    /// established.
    L1dbgI2cOn,
    /// established.
    L1dbgMarginOn,
    /// established.
    L1dbgConsoleOn,
    /// established.
    L1dbgMempanicOn,
    /// established.
    L1dbgPppdumpOn,
    /// established.
    L1dbgPppdelayNumber,
    /// established.
    L1dbgPromreqOn,
    /// established.
    L1dbgQsuspNumberNumber,
    /// established.
    L1dbgPrintfOn,
    /// established.
    L1dbgFlashignoreNumberNumber,
    /// established.
    L1dbgMiscNumber,
    /// established.
    L1dbgGalNumber,
    /// established.
    Link,
    /// established.
    LinkVerbose,
    /// established.
    LinkCheck,
    /// established.
    LinkCheckV,
    /// established.
    LinkStable,
    /// established.
    Log,
    /// established.
    LogClear,
    /// established.
    LogInsertCommand,
    /// established.
    Ioport,
    /// established.
    Ioport400,
    /// established.
    IoportA400,
    /// established.
    IoportB400,
    /// established.
    IoportForce400,
    /// established.
    IoportForceAlt1,
    /// established.
    IoportForceAlt2,
    /// established.
    IoportClksrcA,
    /// established.
    IoportClksrcB,
    /// established.
    IstatMemory,
    /// established.
    IstatQueues,
    /// established.
    IstatQueuesReset,
    /// established.
    IstatTasks,
    /// established.
    IstatPmalloc,
    /// established.
    IstatDumpWord,
    /// established.
    B2b,
    /// established.
    L1,
    /// established.
    Leds,
    /// established.
    Margin,
    /// established.
    MarginDefault,
    /// established.
    MarginNumberDefault,
    /// established.
    MarginNumberNumber,
    /// established.
    MarginDimmVWord,
    /// established.
    Network,
    /// established.
    NetworkUsb,
    /// established.
    NetworkAutodetectOn,
    /// established.
    Pimm,
    /// established.
    PimmAuto,
    /// established.
    PimmClksrcExt,
    /// established.
    PimmClksrcInt,
    /// established.
    PimmClkmodeAsync,
    /// established.
    PimmClkmodeSync,
    /// established.
    Port,
    /// established.
    PortV,
    /// established.
    Power,
    /// established.
    PowerCheck,
    /// established.
    PowerVrm,
    /// established.
    PowerUp,
    /// established.
    PowerUpHold,
    /// established.
    PowerDown,
    /// established.
    Reset,
    /// established.
    ResetHold,
    /// established.
    Nmi,
    /// established.
    Softreset,
    /// established.
    SoftresetNumber,
    /// established.
    Select,
    /// established.
    SelectNumberNumberNumberNumber,
    /// established.
    SelectSubchannelConsole,
    /// established.
    SelectSubchannelA,
    /// established.
    SelectSubchannelNumber,
    /// established.
    SelectSubchannelWord,
    /// established.
    SelectEnable,
    /// established.
    SelectDisable,
    /// established.
    SelectFilterOn,
    /// established.
    SelectFilterOff,
    /// established.
    Serial,
    /// established.
    SerialVerify,
    /// established.
    SerialAll,
    /// established.
    SerialAllV,
    /// established.
    SerialB,
    /// established.
    SerialBV,
    /// established.
    SerialCpu,
    /// established.
    SerialCpuV,
    /// established.
    SerialDimm,
    /// established.
    SerialDimmV,
    /// established.
    SerialDimmCfg,
    /// established.
    SerialDimmStatus,
    /// established.
    SerialClear,
    /// established.
    SerialSetWord,
    /// established.
    SerialWordWordWordWord,
    /// established.
    SerialSecurityOffWordWordWordWord,
    /// established.
    SerialSecurityOn,
    /// established.
    SerialBrickDisable,
    /// established.
    SerialBrickDisableConfirm,
    /// established.
    Sysstate,
    /// established.
    Eeprom,
    /// established.
    EepromNumberNumberNumber,
    /// established.
    EepromFillNumberNumberNumberNumber,
    /// established.
    Uart,
    /// established.
    UartSysLinkReset,
    /// established.
    Usb,
    /// established.
    Router,
    /// established.
    RouterMeta,
    /// established.
    RouterMeta1,
    /// established.
    RouterRepeater,
    /// established.
    RouterOrdinary,
    /// established.
    Router6Port,
    /// established.
    Router6PortConfirm,
    /// established.
    Router8Port,
    /// established.
    Router8PortWordWordWordWord,
    /// established.
    RouterServiceWordWordWordWord,
    /// established.
    RouterServiceOff,
    /// established.
    RouterSpare,
    /// established.
    ServiceWordWordWordWord,
    /// established.
    ServiceOff,
    /// established.
    DateTz,
    /// established.
    DateTzWord,
    /// established.
    Date,
    /// established.
    DateRaw,
    /// established.
    DateWord,
    /// established.
    DateWordWord,
    /// established.
    NvramReset,
    /// established.
    NvramReadNumberNumber,
    /// established.
    NvramWriteNumberNumber,
    /// established.
    NvramScratch,
    /// established.
    NvramScratchNumber,
    /// established.
    L2Command,
    /// established.
    SecurityVerify3400,
    /// established.
    SecurityVerify3800,
    /// established.
    SleepNumberNumberCommand,
    /// established.
    Mtb,
    /// established.
    FlashDefaultReset,
    /// established.
    FlashDefaultA,
    /// established.
    FlashDefaultB,
    /// established.
    FlashDefaultCurrent,
    /// established.
    FlashDefaultOld,
    /// established.
    FlashDefaultNew,
    /// established.
    FlashStatus,
    /// established.
    FlashStatusV,
    /// established.
    RebootL1,
    /// established.
    RebootL1A,
    /// established.
    RebootL1B,
    /// established.
    RebootL1Current,
    /// established.
    RebootL1Old,
    /// established.
    RebootL1New,
    /// established.
    RebootL1Other,
    /// established.
    RebootL1AddrNumber,
    /// established.
    RebootL1RawNumber,
    /// established.
    Version,
    /// established.
    Pbay,
    /// established.
    PbayVersion,
    /// established.
    PbayDps,
    /// established.
    PbayDpsWord,
    /// established.
    PbayDcport,
    /// established.
    PbayDcportWord,
    /// established.
    PbayFru,
    /// established.
    PbayFruWord,
    /// established.
    PbayReset,
    /// established.
    PbayEnv,
    /// established.
    PbayEnvOn,
    /// established.
    PbayEnvOff,
    /// established.
    PbayInit,
    /// established.
    PbaySerial,
    /// established.
    TestI2c,
    /// established.
    TestI2cNumber,
    /// established.
    TestIoexpSetNumberNumber,
    /// established.
    TestIoexpGetNumber,
    /// established.
    TestIoexpGetAll,
    /// established.
    TestIntr,
    /// established.
    TestDisplay,
    /// established.
    TestExceptionMem,
    /// established.
    TestExceptionStack,
    /// established.
    TestExceptionWdog,
    /// established.
    TestExceptionFatal,
    /// established.
    ScanReset,
    /// established.
    ScanResetHard,
    /// established.
    ScanResetSoft,
    /// established.
    ScanResetBoth,
    /// established.
    ScanSelNumberNumber,
    /// established.
    ScanSelNumberNumberNumber,
    /// established.
    ScanSelNumberNumberNumberNumber,
    /// established.
    ScanIdsNumber,
    /// established.
    ScanSetTrst0,
    /// established.
    ScanSetTrst1,
    /// established.
    ScanSetTck0,
    /// established.
    ScanSetTck1,
    /// established.
    ScanSetTms0,
    /// established.
    ScanSetTms1,
    /// established.
    ScanSetTdi0,
    /// established.
    ScanSetTdi1,
    /// established.
    ScanSetPsi0,
    /// established.
    ScanSetPsi1,
    /// established.
    ScanGet,
    /// established.
    ScanDebugNumber,
    /// established.
    ScanDebug,
    /// established.
    ScanCfg,
    /// established.
    ScanImpSetDllbypNumber,
    /// established.
    ScanImpSetNumber,
    /// established.
    ScanImpUnlock,
    /// established.
    ScanTestNumber,
    /// established.
    Fru,
    /// established.
    FruCommand,
    /// A value the notes do not name.
    Other(u32),
}

impl Cmd0Index {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x0 => Self::AllCommand,
            0x1 => Self::RackSlotCommand,
            0x2 => Self::Autopower,
            0x3 => Self::AutopowerOn,
            0x4 => Self::AutopowerOff,
            0x5 => Self::AutopowerAbort,
            0x6 => Self::SysLink,
            0x7 => Self::SysLinkNone,
            0x8 => Self::SysLinkPpp,
            0x9 => Self::SysLinkEnable,
            0xa => Self::SysLinkReset,
            0xb => Self::SysLinkPackets,
            0xc => Self::Brick,
            0xd => Self::BrickTypeWord,
            0xe => Self::BrickRackDecimal,
            0xf => Self::BrickSlotDecimal,
            0x10 => Self::BrickRackslotDecimalDecimal,
            0x11 => Self::BrickRackslotResetNumberNumber,
            0x12 => Self::BrickPartitionNone,
            0x13 => Self::BrickPartitionNumber,
            0x14 => Self::Partdb,
            0x15 => Self::PartdbEnable,
            0x16 => Self::PartdbDisable,
            0x17 => Self::Cpu,
            0x18 => Self::CpuE,
            0x19 => Self::CpuWordE,
            0x1a => Self::B2bWordCommand,
            0x1b => Self::Config,
            0x1c => Self::ConfigV,
            0x1d => Self::ConfigReset,
            0x1e => Self::ConfigRing,
            0x1f => Self::Debug,
            0x20 => Self::DebugNumber,
            0x21 => Self::Display,
            0x22 => Self::DisplayNumberCommand,
            0x23 => Self::DisplayPowerOn,
            0x24 => Self::DisplayAttentionOn,
            0x25 => Self::DisplayFailureOn,
            0x26 => Self::DisplayPowerOff,
            0x27 => Self::DisplayAttentionOff,
            0x28 => Self::DisplayFailureOff,
            0x29 => Self::Button,
            0x2a => Self::ButtonAllEnable,
            0x2b => Self::ButtonAllDisable,
            0x2c => Self::ButtonPwrEnable,
            0x2d => Self::ButtonPwrDisable,
            0x2e => Self::ButtonRstEnable,
            0x2f => Self::ButtonRstDisable,
            0x30 => Self::ButtonNmiEnable,
            0x31 => Self::ButtonNmiDisable,
            0x32 => Self::LetTheCarnageBegin,
            0x33 => Self::SlippedMyMind,
            0x34 => Self::SuperSizedPlease,
            0x35 => Self::Env,
            0x36 => Self::EnvCheck,
            0x37 => Self::EnvOn,
            0x38 => Self::EnvOff,
            0x39 => Self::EnvPowerOn,
            0x3a => Self::EnvPowerOff,
            0x3b => Self::EnvPowerMonitor,
            0x3c => Self::EnvTempOn,
            0x3d => Self::EnvTempOff,
            0x3e => Self::EnvAltitudeHigh,
            0x3f => Self::EnvAltitudeLow,
            0x40 => Self::EnvReset,
            0x41 => Self::EnvTestWarning,
            0x42 => Self::EnvTestFault,
            0x43 => Self::EnvTestTmpadv,
            0x44 => Self::EnvTestTmpcrit,
            0x45 => Self::EnvTestNumberNumberNumberNumberNumber,
            0x46 => Self::EnvTestPwrdn,
            0x47 => Self::EnvMessages,
            0x48 => Self::EnvMessagesFaultsonly,
            0x49 => Self::EnvMessagesAll,
            0x4a => Self::Fan,
            0x4b => Self::FanOnNumber,
            0x4c => Self::FanOn,
            0x4d => Self::FanSpeed,
            0x4e => Self::FanSpeedNumberNumber,
            0x4f => Self::FanSpeedNumber,
            0x50 => Self::FanSpeedLowNumber,
            0x51 => Self::FanSpeedLow,
            0x52 => Self::FanAutoOn,
            0x53 => Self::Help,
            0x54 => Self::HelpWord,
            0x55 => Self::HelpListHidden,
            0x56 => Self::HelpWordHidden,
            0x57 => Self::History,
            0x58 => Self::L1dbg,
            0x59 => Self::L1dbgSysLinkNumber,
            0x5a => Self::L1dbgSysLinkNumberNumber,
            0x5b => Self::L1dbgIrtrNumber,
            0x5c => Self::L1dbgEnvNumber,
            0x5d => Self::L1dbgEnvOff,
            0x5e => Self::L1dbgPortOn,
            0x5f => Self::L1dbgI2cOn,
            0x60 => Self::L1dbgMarginOn,
            0x61 => Self::L1dbgConsoleOn,
            0x62 => Self::L1dbgMempanicOn,
            0x63 => Self::L1dbgPppdumpOn,
            0x64 => Self::L1dbgPppdelayNumber,
            0x65 => Self::L1dbgPromreqOn,
            0x66 => Self::L1dbgQsuspNumberNumber,
            0x67 => Self::L1dbgPrintfOn,
            0x68 => Self::L1dbgFlashignoreNumberNumber,
            0x69 => Self::L1dbgMiscNumber,
            0x6a => Self::L1dbgGalNumber,
            0x6b => Self::Link,
            0x6c => Self::LinkVerbose,
            0x6d => Self::LinkCheck,
            0x6e => Self::LinkCheckV,
            0x6f => Self::LinkStable,
            0x70 => Self::Log,
            0x71 => Self::LogClear,
            0x72 => Self::LogInsertCommand,
            0x73 => Self::Ioport,
            0x74 => Self::Ioport400,
            0x75 => Self::IoportA400,
            0x76 => Self::IoportB400,
            0x77 => Self::IoportForce400,
            0x78 => Self::IoportForceAlt1,
            0x79 => Self::IoportForceAlt2,
            0x7a => Self::IoportClksrcA,
            0x7b => Self::IoportClksrcB,
            0x7c => Self::IstatMemory,
            0x7d => Self::IstatQueues,
            0x7e => Self::IstatQueuesReset,
            0x7f => Self::IstatTasks,
            0x80 => Self::IstatPmalloc,
            0x81 => Self::IstatDumpWord,
            0x82 => Self::B2b,
            0x83 => Self::L1,
            0x84 => Self::Leds,
            0x85 => Self::Margin,
            0x86 => Self::MarginDefault,
            0x87 => Self::MarginNumberDefault,
            0x88 => Self::MarginNumberNumber,
            0x89 => Self::MarginDimmVWord,
            0x8a => Self::Network,
            0x8b => Self::NetworkUsb,
            0x8c => Self::NetworkAutodetectOn,
            0x8d => Self::Pimm,
            0x8e => Self::PimmAuto,
            0x8f => Self::PimmClksrcExt,
            0x90 => Self::PimmClksrcInt,
            0x91 => Self::PimmClkmodeAsync,
            0x92 => Self::PimmClkmodeSync,
            0x93 => Self::Port,
            0x94 => Self::PortV,
            0x95 => Self::Power,
            0x96 => Self::PowerCheck,
            0x97 => Self::PowerVrm,
            0x98 => Self::PowerUp,
            0x99 => Self::PowerUpHold,
            0x9a => Self::PowerDown,
            0x9b => Self::Reset,
            0x9c => Self::ResetHold,
            0x9d => Self::Nmi,
            0x9e => Self::Softreset,
            0x9f => Self::SoftresetNumber,
            0xa0 => Self::Select,
            0xa1 => Self::SelectNumberNumberNumberNumber,
            0xa2 => Self::SelectSubchannelConsole,
            0xa3 => Self::SelectSubchannelA,
            0xa4 => Self::SelectSubchannelNumber,
            0xa5 => Self::SelectSubchannelWord,
            0xa6 => Self::SelectEnable,
            0xa7 => Self::SelectDisable,
            0xa8 => Self::SelectFilterOn,
            0xa9 => Self::SelectFilterOff,
            0xaa => Self::Serial,
            0xab => Self::SerialVerify,
            0xac => Self::SerialAll,
            0xad => Self::SerialAllV,
            0xae => Self::SerialB,
            0xaf => Self::SerialBV,
            0xb0 => Self::SerialCpu,
            0xb1 => Self::SerialCpuV,
            0xb2 => Self::SerialDimm,
            0xb3 => Self::SerialDimmV,
            0xb4 => Self::SerialDimmCfg,
            0xb5 => Self::SerialDimmStatus,
            0xb6 => Self::SerialClear,
            0xb7 => Self::SerialSetWord,
            0xb8 => Self::SerialWordWordWordWord,
            0xb9 => Self::SerialSecurityOffWordWordWordWord,
            0xba => Self::SerialSecurityOn,
            0xbb => Self::SerialBrickDisable,
            0xbc => Self::SerialBrickDisableConfirm,
            0xbd => Self::Sysstate,
            0xbe => Self::Eeprom,
            0xbf => Self::EepromNumberNumberNumber,
            0xc0 => Self::EepromFillNumberNumberNumberNumber,
            0xc1 => Self::Uart,
            0xc2 => Self::UartSysLinkReset,
            0xc3 => Self::Usb,
            0xc4 => Self::Router,
            0xc5 => Self::RouterMeta,
            0xc6 => Self::RouterMeta1,
            0xc7 => Self::RouterRepeater,
            0xc8 => Self::RouterOrdinary,
            0xc9 => Self::Router6Port,
            0xca => Self::Router6PortConfirm,
            0xcb => Self::Router8Port,
            0xcc => Self::Router8PortWordWordWordWord,
            0xcd => Self::RouterServiceWordWordWordWord,
            0xce => Self::RouterServiceOff,
            0xcf => Self::RouterSpare,
            0xd0 => Self::ServiceWordWordWordWord,
            0xd1 => Self::ServiceOff,
            0xd2 => Self::DateTz,
            0xd3 => Self::DateTzWord,
            0xd4 => Self::Date,
            0xd5 => Self::DateRaw,
            0xd6 => Self::DateWord,
            0xd7 => Self::DateWordWord,
            0xd8 => Self::NvramReset,
            0xd9 => Self::NvramReadNumberNumber,
            0xda => Self::NvramWriteNumberNumber,
            0xdb => Self::NvramScratch,
            0xdc => Self::NvramScratchNumber,
            0xdd => Self::L2Command,
            0xde => Self::SecurityVerify3400,
            0xdf => Self::SecurityVerify3800,
            0xe0 => Self::SleepNumberNumberCommand,
            0xe1 => Self::Mtb,
            0xe2 => Self::FlashDefaultReset,
            0xe3 => Self::FlashDefaultA,
            0xe4 => Self::FlashDefaultB,
            0xe5 => Self::FlashDefaultCurrent,
            0xe6 => Self::FlashDefaultOld,
            0xe7 => Self::FlashDefaultNew,
            0xe8 => Self::FlashStatus,
            0xe9 => Self::FlashStatusV,
            0xea => Self::RebootL1,
            0xeb => Self::RebootL1A,
            0xec => Self::RebootL1B,
            0xed => Self::RebootL1Current,
            0xee => Self::RebootL1Old,
            0xef => Self::RebootL1New,
            0xf0 => Self::RebootL1Other,
            0xf1 => Self::RebootL1AddrNumber,
            0xf2 => Self::RebootL1RawNumber,
            0xf3 => Self::Version,
            0xf4 => Self::Pbay,
            0xf5 => Self::PbayVersion,
            0xf6 => Self::PbayDps,
            0xf7 => Self::PbayDpsWord,
            0xf8 => Self::PbayDcport,
            0xf9 => Self::PbayDcportWord,
            0xfa => Self::PbayFru,
            0xfb => Self::PbayFruWord,
            0xfc => Self::PbayReset,
            0xfd => Self::PbayEnv,
            0xfe => Self::PbayEnvOn,
            0xff => Self::PbayEnvOff,
            0x100 => Self::PbayInit,
            0x101 => Self::PbaySerial,
            0x102 => Self::TestI2c,
            0x103 => Self::TestI2cNumber,
            0x104 => Self::TestIoexpSetNumberNumber,
            0x105 => Self::TestIoexpGetNumber,
            0x106 => Self::TestIoexpGetAll,
            0x107 => Self::TestIntr,
            0x108 => Self::TestDisplay,
            0x109 => Self::TestExceptionMem,
            0x10a => Self::TestExceptionStack,
            0x10b => Self::TestExceptionWdog,
            0x10c => Self::TestExceptionFatal,
            0x10d => Self::ScanReset,
            0x10e => Self::ScanResetHard,
            0x10f => Self::ScanResetSoft,
            0x110 => Self::ScanResetBoth,
            0x111 => Self::ScanSelNumberNumber,
            0x112 => Self::ScanSelNumberNumberNumber,
            0x113 => Self::ScanSelNumberNumberNumberNumber,
            0x114 => Self::ScanIdsNumber,
            0x115 => Self::ScanSetTrst0,
            0x116 => Self::ScanSetTrst1,
            0x117 => Self::ScanSetTck0,
            0x118 => Self::ScanSetTck1,
            0x119 => Self::ScanSetTms0,
            0x11a => Self::ScanSetTms1,
            0x11b => Self::ScanSetTdi0,
            0x11c => Self::ScanSetTdi1,
            0x11d => Self::ScanSetPsi0,
            0x11e => Self::ScanSetPsi1,
            0x11f => Self::ScanGet,
            0x120 => Self::ScanDebugNumber,
            0x121 => Self::ScanDebug,
            0x122 => Self::ScanCfg,
            0x123 => Self::ScanImpSetDllbypNumber,
            0x124 => Self::ScanImpSetNumber,
            0x125 => Self::ScanImpUnlock,
            0x126 => Self::ScanTestNumber,
            0x127 => Self::Fru,
            0x128 => Self::FruCommand,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::AllCommand => 0x0,
            Self::RackSlotCommand => 0x1,
            Self::Autopower => 0x2,
            Self::AutopowerOn => 0x3,
            Self::AutopowerOff => 0x4,
            Self::AutopowerAbort => 0x5,
            Self::SysLink => 0x6,
            Self::SysLinkNone => 0x7,
            Self::SysLinkPpp => 0x8,
            Self::SysLinkEnable => 0x9,
            Self::SysLinkReset => 0xa,
            Self::SysLinkPackets => 0xb,
            Self::Brick => 0xc,
            Self::BrickTypeWord => 0xd,
            Self::BrickRackDecimal => 0xe,
            Self::BrickSlotDecimal => 0xf,
            Self::BrickRackslotDecimalDecimal => 0x10,
            Self::BrickRackslotResetNumberNumber => 0x11,
            Self::BrickPartitionNone => 0x12,
            Self::BrickPartitionNumber => 0x13,
            Self::Partdb => 0x14,
            Self::PartdbEnable => 0x15,
            Self::PartdbDisable => 0x16,
            Self::Cpu => 0x17,
            Self::CpuE => 0x18,
            Self::CpuWordE => 0x19,
            Self::B2bWordCommand => 0x1a,
            Self::Config => 0x1b,
            Self::ConfigV => 0x1c,
            Self::ConfigReset => 0x1d,
            Self::ConfigRing => 0x1e,
            Self::Debug => 0x1f,
            Self::DebugNumber => 0x20,
            Self::Display => 0x21,
            Self::DisplayNumberCommand => 0x22,
            Self::DisplayPowerOn => 0x23,
            Self::DisplayAttentionOn => 0x24,
            Self::DisplayFailureOn => 0x25,
            Self::DisplayPowerOff => 0x26,
            Self::DisplayAttentionOff => 0x27,
            Self::DisplayFailureOff => 0x28,
            Self::Button => 0x29,
            Self::ButtonAllEnable => 0x2a,
            Self::ButtonAllDisable => 0x2b,
            Self::ButtonPwrEnable => 0x2c,
            Self::ButtonPwrDisable => 0x2d,
            Self::ButtonRstEnable => 0x2e,
            Self::ButtonRstDisable => 0x2f,
            Self::ButtonNmiEnable => 0x30,
            Self::ButtonNmiDisable => 0x31,
            Self::LetTheCarnageBegin => 0x32,
            Self::SlippedMyMind => 0x33,
            Self::SuperSizedPlease => 0x34,
            Self::Env => 0x35,
            Self::EnvCheck => 0x36,
            Self::EnvOn => 0x37,
            Self::EnvOff => 0x38,
            Self::EnvPowerOn => 0x39,
            Self::EnvPowerOff => 0x3a,
            Self::EnvPowerMonitor => 0x3b,
            Self::EnvTempOn => 0x3c,
            Self::EnvTempOff => 0x3d,
            Self::EnvAltitudeHigh => 0x3e,
            Self::EnvAltitudeLow => 0x3f,
            Self::EnvReset => 0x40,
            Self::EnvTestWarning => 0x41,
            Self::EnvTestFault => 0x42,
            Self::EnvTestTmpadv => 0x43,
            Self::EnvTestTmpcrit => 0x44,
            Self::EnvTestNumberNumberNumberNumberNumber => 0x45,
            Self::EnvTestPwrdn => 0x46,
            Self::EnvMessages => 0x47,
            Self::EnvMessagesFaultsonly => 0x48,
            Self::EnvMessagesAll => 0x49,
            Self::Fan => 0x4a,
            Self::FanOnNumber => 0x4b,
            Self::FanOn => 0x4c,
            Self::FanSpeed => 0x4d,
            Self::FanSpeedNumberNumber => 0x4e,
            Self::FanSpeedNumber => 0x4f,
            Self::FanSpeedLowNumber => 0x50,
            Self::FanSpeedLow => 0x51,
            Self::FanAutoOn => 0x52,
            Self::Help => 0x53,
            Self::HelpWord => 0x54,
            Self::HelpListHidden => 0x55,
            Self::HelpWordHidden => 0x56,
            Self::History => 0x57,
            Self::L1dbg => 0x58,
            Self::L1dbgSysLinkNumber => 0x59,
            Self::L1dbgSysLinkNumberNumber => 0x5a,
            Self::L1dbgIrtrNumber => 0x5b,
            Self::L1dbgEnvNumber => 0x5c,
            Self::L1dbgEnvOff => 0x5d,
            Self::L1dbgPortOn => 0x5e,
            Self::L1dbgI2cOn => 0x5f,
            Self::L1dbgMarginOn => 0x60,
            Self::L1dbgConsoleOn => 0x61,
            Self::L1dbgMempanicOn => 0x62,
            Self::L1dbgPppdumpOn => 0x63,
            Self::L1dbgPppdelayNumber => 0x64,
            Self::L1dbgPromreqOn => 0x65,
            Self::L1dbgQsuspNumberNumber => 0x66,
            Self::L1dbgPrintfOn => 0x67,
            Self::L1dbgFlashignoreNumberNumber => 0x68,
            Self::L1dbgMiscNumber => 0x69,
            Self::L1dbgGalNumber => 0x6a,
            Self::Link => 0x6b,
            Self::LinkVerbose => 0x6c,
            Self::LinkCheck => 0x6d,
            Self::LinkCheckV => 0x6e,
            Self::LinkStable => 0x6f,
            Self::Log => 0x70,
            Self::LogClear => 0x71,
            Self::LogInsertCommand => 0x72,
            Self::Ioport => 0x73,
            Self::Ioport400 => 0x74,
            Self::IoportA400 => 0x75,
            Self::IoportB400 => 0x76,
            Self::IoportForce400 => 0x77,
            Self::IoportForceAlt1 => 0x78,
            Self::IoportForceAlt2 => 0x79,
            Self::IoportClksrcA => 0x7a,
            Self::IoportClksrcB => 0x7b,
            Self::IstatMemory => 0x7c,
            Self::IstatQueues => 0x7d,
            Self::IstatQueuesReset => 0x7e,
            Self::IstatTasks => 0x7f,
            Self::IstatPmalloc => 0x80,
            Self::IstatDumpWord => 0x81,
            Self::B2b => 0x82,
            Self::L1 => 0x83,
            Self::Leds => 0x84,
            Self::Margin => 0x85,
            Self::MarginDefault => 0x86,
            Self::MarginNumberDefault => 0x87,
            Self::MarginNumberNumber => 0x88,
            Self::MarginDimmVWord => 0x89,
            Self::Network => 0x8a,
            Self::NetworkUsb => 0x8b,
            Self::NetworkAutodetectOn => 0x8c,
            Self::Pimm => 0x8d,
            Self::PimmAuto => 0x8e,
            Self::PimmClksrcExt => 0x8f,
            Self::PimmClksrcInt => 0x90,
            Self::PimmClkmodeAsync => 0x91,
            Self::PimmClkmodeSync => 0x92,
            Self::Port => 0x93,
            Self::PortV => 0x94,
            Self::Power => 0x95,
            Self::PowerCheck => 0x96,
            Self::PowerVrm => 0x97,
            Self::PowerUp => 0x98,
            Self::PowerUpHold => 0x99,
            Self::PowerDown => 0x9a,
            Self::Reset => 0x9b,
            Self::ResetHold => 0x9c,
            Self::Nmi => 0x9d,
            Self::Softreset => 0x9e,
            Self::SoftresetNumber => 0x9f,
            Self::Select => 0xa0,
            Self::SelectNumberNumberNumberNumber => 0xa1,
            Self::SelectSubchannelConsole => 0xa2,
            Self::SelectSubchannelA => 0xa3,
            Self::SelectSubchannelNumber => 0xa4,
            Self::SelectSubchannelWord => 0xa5,
            Self::SelectEnable => 0xa6,
            Self::SelectDisable => 0xa7,
            Self::SelectFilterOn => 0xa8,
            Self::SelectFilterOff => 0xa9,
            Self::Serial => 0xaa,
            Self::SerialVerify => 0xab,
            Self::SerialAll => 0xac,
            Self::SerialAllV => 0xad,
            Self::SerialB => 0xae,
            Self::SerialBV => 0xaf,
            Self::SerialCpu => 0xb0,
            Self::SerialCpuV => 0xb1,
            Self::SerialDimm => 0xb2,
            Self::SerialDimmV => 0xb3,
            Self::SerialDimmCfg => 0xb4,
            Self::SerialDimmStatus => 0xb5,
            Self::SerialClear => 0xb6,
            Self::SerialSetWord => 0xb7,
            Self::SerialWordWordWordWord => 0xb8,
            Self::SerialSecurityOffWordWordWordWord => 0xb9,
            Self::SerialSecurityOn => 0xba,
            Self::SerialBrickDisable => 0xbb,
            Self::SerialBrickDisableConfirm => 0xbc,
            Self::Sysstate => 0xbd,
            Self::Eeprom => 0xbe,
            Self::EepromNumberNumberNumber => 0xbf,
            Self::EepromFillNumberNumberNumberNumber => 0xc0,
            Self::Uart => 0xc1,
            Self::UartSysLinkReset => 0xc2,
            Self::Usb => 0xc3,
            Self::Router => 0xc4,
            Self::RouterMeta => 0xc5,
            Self::RouterMeta1 => 0xc6,
            Self::RouterRepeater => 0xc7,
            Self::RouterOrdinary => 0xc8,
            Self::Router6Port => 0xc9,
            Self::Router6PortConfirm => 0xca,
            Self::Router8Port => 0xcb,
            Self::Router8PortWordWordWordWord => 0xcc,
            Self::RouterServiceWordWordWordWord => 0xcd,
            Self::RouterServiceOff => 0xce,
            Self::RouterSpare => 0xcf,
            Self::ServiceWordWordWordWord => 0xd0,
            Self::ServiceOff => 0xd1,
            Self::DateTz => 0xd2,
            Self::DateTzWord => 0xd3,
            Self::Date => 0xd4,
            Self::DateRaw => 0xd5,
            Self::DateWord => 0xd6,
            Self::DateWordWord => 0xd7,
            Self::NvramReset => 0xd8,
            Self::NvramReadNumberNumber => 0xd9,
            Self::NvramWriteNumberNumber => 0xda,
            Self::NvramScratch => 0xdb,
            Self::NvramScratchNumber => 0xdc,
            Self::L2Command => 0xdd,
            Self::SecurityVerify3400 => 0xde,
            Self::SecurityVerify3800 => 0xdf,
            Self::SleepNumberNumberCommand => 0xe0,
            Self::Mtb => 0xe1,
            Self::FlashDefaultReset => 0xe2,
            Self::FlashDefaultA => 0xe3,
            Self::FlashDefaultB => 0xe4,
            Self::FlashDefaultCurrent => 0xe5,
            Self::FlashDefaultOld => 0xe6,
            Self::FlashDefaultNew => 0xe7,
            Self::FlashStatus => 0xe8,
            Self::FlashStatusV => 0xe9,
            Self::RebootL1 => 0xea,
            Self::RebootL1A => 0xeb,
            Self::RebootL1B => 0xec,
            Self::RebootL1Current => 0xed,
            Self::RebootL1Old => 0xee,
            Self::RebootL1New => 0xef,
            Self::RebootL1Other => 0xf0,
            Self::RebootL1AddrNumber => 0xf1,
            Self::RebootL1RawNumber => 0xf2,
            Self::Version => 0xf3,
            Self::Pbay => 0xf4,
            Self::PbayVersion => 0xf5,
            Self::PbayDps => 0xf6,
            Self::PbayDpsWord => 0xf7,
            Self::PbayDcport => 0xf8,
            Self::PbayDcportWord => 0xf9,
            Self::PbayFru => 0xfa,
            Self::PbayFruWord => 0xfb,
            Self::PbayReset => 0xfc,
            Self::PbayEnv => 0xfd,
            Self::PbayEnvOn => 0xfe,
            Self::PbayEnvOff => 0xff,
            Self::PbayInit => 0x100,
            Self::PbaySerial => 0x101,
            Self::TestI2c => 0x102,
            Self::TestI2cNumber => 0x103,
            Self::TestIoexpSetNumberNumber => 0x104,
            Self::TestIoexpGetNumber => 0x105,
            Self::TestIoexpGetAll => 0x106,
            Self::TestIntr => 0x107,
            Self::TestDisplay => 0x108,
            Self::TestExceptionMem => 0x109,
            Self::TestExceptionStack => 0x10a,
            Self::TestExceptionWdog => 0x10b,
            Self::TestExceptionFatal => 0x10c,
            Self::ScanReset => 0x10d,
            Self::ScanResetHard => 0x10e,
            Self::ScanResetSoft => 0x10f,
            Self::ScanResetBoth => 0x110,
            Self::ScanSelNumberNumber => 0x111,
            Self::ScanSelNumberNumberNumber => 0x112,
            Self::ScanSelNumberNumberNumberNumber => 0x113,
            Self::ScanIdsNumber => 0x114,
            Self::ScanSetTrst0 => 0x115,
            Self::ScanSetTrst1 => 0x116,
            Self::ScanSetTck0 => 0x117,
            Self::ScanSetTck1 => 0x118,
            Self::ScanSetTms0 => 0x119,
            Self::ScanSetTms1 => 0x11a,
            Self::ScanSetTdi0 => 0x11b,
            Self::ScanSetTdi1 => 0x11c,
            Self::ScanSetPsi0 => 0x11d,
            Self::ScanSetPsi1 => 0x11e,
            Self::ScanGet => 0x11f,
            Self::ScanDebugNumber => 0x120,
            Self::ScanDebug => 0x121,
            Self::ScanCfg => 0x122,
            Self::ScanImpSetDllbypNumber => 0x123,
            Self::ScanImpSetNumber => 0x124,
            Self::ScanImpUnlock => 0x125,
            Self::ScanTestNumber => 0x126,
            Self::Fru => 0x127,
            Self::FruCommand => 0x128,
            Self::Other(v) => v as u32,
        }
    }
}

/// Values of `index` in `Cmd1` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd1Index {
    /// established.
    CheckPowerOn,
    /// established.
    CheckMarginHigh,
    /// established.
    CheckScrambleEnabled,
    /// established.
    CheckLinks,
    /// established.
    CheckDebugNumber,
    /// established.
    CheckSecurityOn,
    /// established.
    CheckSerialWord,
    /// established.
    FruCapture,
    /// established.
    FruCaptureVerbose,
    /// established.
    FruDump,
    /// established.
    FruReset,
    /// established.
    FruStatus,
    /// established.
    FruStatusVerbose,
    /// established.
    Deadlock,
    /// established.
    DeadlockVerbose,
    /// established.
    Memcfg,
    /// established.
    Memcfg1bank166E,
    /// established.
    Shcfg,
    /// established.
    ShcfgFsbReset,
    /// established.
    ShcfgFsb,
    /// established.
    ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
    /// established.
    ShcfgWrt90Reset,
    /// established.
    ShcfgWrt90,
    /// established.
    ShcfgWrt90NumberNumberNumber,
    /// established.
    ShcfgMciReset,
    /// established.
    ShcfgMci,
    /// established.
    ShcfgMciNumberNumberNumber,
    /// established.
    ShcfgMdclk,
    /// established.
    ShcfgMdclkNumber,
    /// established.
    ShcfgNiclk,
    /// established.
    ShcfgNiclkNumber,
    /// established.
    ShcfgOsc,
    /// established.
    ShcfgOscNumber,
    /// established.
    ShcfgGtlRe,
    /// established.
    ShcfgGtlReNumber,
    /// established.
    ShcfgJclkstop,
    /// established.
    ShcfgJclkstopNumber,
    /// established.
    ShcfgTp,
    /// established.
    ShcfgTpNumber,
    /// established.
    NicfgNi0,
    /// established.
    NicfgNumberNi0,
    /// established.
    NicfgNi0Number,
    /// established.
    NicfgNi0NumberNumber,
    /// established.
    NicfgNi0NumberNumberNumber,
    /// established.
    NicfgNi0NumberNumberNumberNumber,
    /// established.
    NicfgNi0NumberNumberNumberNumberNumber,
    /// established.
    NicfgNumberNi0Number,
    /// established.
    NicfgNumberNi0NumberNumber,
    /// established.
    NicfgNumberNi0NumberNumberNumber,
    /// established.
    NicfgNumberNi0NumberNumberNumberNumber,
    /// established.
    NicfgNumberNi0NumberNumberNumberNumberNumber,
    /// established.
    RmmrWordNumber,
    /// established.
    WmmrWordNumberNumberNumber,
    /// established.
    PktgenReadWordNumber,
    /// established.
    PktgenWriteWordNumberNumberNumber,
    /// established.
    PktgenDebugNumber,
    /// established.
    TuneGet,
    /// established.
    Promice,
    /// established.
    PromiceE,
    /// established.
    PromiceD,
    /// established.
    PromiceNumberE,
    /// established.
    PromiceNumberD,
    /// established.
    Promver,
    /// established.
    Node,
    /// established.
    NodeE,
    /// established.
    NodeD,
    /// established.
    NodeNumberE,
    /// established.
    NodeNumberD,
    /// established.
    PicMode,
    /// established.
    War,
    /// established.
    WarE,
    /// established.
    WarD,
    /// established.
    Shdbg,
    /// established.
    ShdbgRetrylimitNumber,
    /// established.
    ShdbgRdelayNumber,
    /// established.
    ShdbgFullresetNumber,
    /// established.
    ShdbgLinkdownNumber,
    /// established.
    Mbrick,
    /// established.
    MbrickMemory,
    /// established.
    MbrickMemoryConfirm,
    /// established.
    MbrickCpuWordWordWordWord,
    /// established.
    MbrickCpu,
    /// established.
    IfIOnlyHadABrain,
    /// established.
    Ssi,
    /// established.
    SsiServiceWordWordWordWord,
    /// established.
    SsiServiceOff,
    /// established.
    SsiWordWordWordWordWord,
    /// established.
    MadWithPowerNumber,
    /// established.
    PwmRead,
    /// established.
    PwmSetNumberNumber,
    /// established.
    SysLinkSimOn,
    /// established.
    SysLinkSimOnNumberNumber,
    /// established.
    SysLinkSimOff,
    /// established.
    SysLinkDumpInputNumber,
    /// established.
    SysLinkDumpOutputNumber,
    /// established.
    SysLinkCaptureReset,
    /// established.
    SysLinkLogNumber,
    /// established.
    SysLinkLogAll,
    /// established.
    SysLinkLogReset,
    /// established.
    SysLinkLogNumberInsertCommand,
    /// established.
    SysLinkConfig,
    /// established.
    SysLinkQsuspend,
    /// established.
    SysLinkQsuspendNumber,
    /// established.
    ErrorNumber,
    /// A value the notes do not name.
    Other(u8),
}

impl Cmd1Index {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x0 => Self::CheckPowerOn,
            0x1 => Self::CheckMarginHigh,
            0x2 => Self::CheckScrambleEnabled,
            0x3 => Self::CheckLinks,
            0x4 => Self::CheckDebugNumber,
            0x5 => Self::CheckSecurityOn,
            0x6 => Self::CheckSerialWord,
            0x7 => Self::FruCapture,
            0x8 => Self::FruCaptureVerbose,
            0x9 => Self::FruDump,
            0xa => Self::FruReset,
            0xb => Self::FruStatus,
            0xc => Self::FruStatusVerbose,
            0xd => Self::Deadlock,
            0xe => Self::DeadlockVerbose,
            0xf => Self::Memcfg,
            0x10 => Self::Memcfg1bank166E,
            0x11 => Self::Shcfg,
            0x12 => Self::ShcfgFsbReset,
            0x13 => Self::ShcfgFsb,
            0x14 => Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
            0x15 => Self::ShcfgWrt90Reset,
            0x16 => Self::ShcfgWrt90,
            0x17 => Self::ShcfgWrt90NumberNumberNumber,
            0x18 => Self::ShcfgMciReset,
            0x19 => Self::ShcfgMci,
            0x1a => Self::ShcfgMciNumberNumberNumber,
            0x1b => Self::ShcfgMdclk,
            0x1c => Self::ShcfgMdclkNumber,
            0x1d => Self::ShcfgNiclk,
            0x1e => Self::ShcfgNiclkNumber,
            0x1f => Self::ShcfgOsc,
            0x20 => Self::ShcfgOscNumber,
            0x21 => Self::ShcfgGtlRe,
            0x22 => Self::ShcfgGtlReNumber,
            0x23 => Self::ShcfgJclkstop,
            0x24 => Self::ShcfgJclkstopNumber,
            0x25 => Self::ShcfgTp,
            0x26 => Self::ShcfgTpNumber,
            0x27 => Self::NicfgNi0,
            0x28 => Self::NicfgNumberNi0,
            0x29 => Self::NicfgNi0Number,
            0x2a => Self::NicfgNi0NumberNumber,
            0x2b => Self::NicfgNi0NumberNumberNumber,
            0x2c => Self::NicfgNi0NumberNumberNumberNumber,
            0x2d => Self::NicfgNi0NumberNumberNumberNumberNumber,
            0x2e => Self::NicfgNumberNi0Number,
            0x2f => Self::NicfgNumberNi0NumberNumber,
            0x30 => Self::NicfgNumberNi0NumberNumberNumber,
            0x31 => Self::NicfgNumberNi0NumberNumberNumberNumber,
            0x32 => Self::NicfgNumberNi0NumberNumberNumberNumberNumber,
            0x33 => Self::RmmrWordNumber,
            0x34 => Self::WmmrWordNumberNumberNumber,
            0x35 => Self::PktgenReadWordNumber,
            0x36 => Self::PktgenWriteWordNumberNumberNumber,
            0x37 => Self::PktgenDebugNumber,
            0x38 => Self::TuneGet,
            0x39 => Self::Promice,
            0x3a => Self::PromiceE,
            0x3b => Self::PromiceD,
            0x3c => Self::PromiceNumberE,
            0x3d => Self::PromiceNumberD,
            0x3e => Self::Promver,
            0x3f => Self::Node,
            0x40 => Self::NodeE,
            0x41 => Self::NodeD,
            0x42 => Self::NodeNumberE,
            0x43 => Self::NodeNumberD,
            0x44 => Self::PicMode,
            0x45 => Self::War,
            0x46 => Self::WarE,
            0x47 => Self::WarD,
            0x48 => Self::Shdbg,
            0x49 => Self::ShdbgRetrylimitNumber,
            0x4a => Self::ShdbgRdelayNumber,
            0x4b => Self::ShdbgFullresetNumber,
            0x4c => Self::ShdbgLinkdownNumber,
            0x4d => Self::Mbrick,
            0x4e => Self::MbrickMemory,
            0x4f => Self::MbrickMemoryConfirm,
            0x50 => Self::MbrickCpuWordWordWordWord,
            0x51 => Self::MbrickCpu,
            0x52 => Self::IfIOnlyHadABrain,
            0x53 => Self::Ssi,
            0x54 => Self::SsiServiceWordWordWordWord,
            0x55 => Self::SsiServiceOff,
            0x56 => Self::SsiWordWordWordWordWord,
            0x57 => Self::MadWithPowerNumber,
            0x58 => Self::PwmRead,
            0x59 => Self::PwmSetNumberNumber,
            0x5a => Self::SysLinkSimOn,
            0x5b => Self::SysLinkSimOnNumberNumber,
            0x5c => Self::SysLinkSimOff,
            0x5d => Self::SysLinkDumpInputNumber,
            0x5e => Self::SysLinkDumpOutputNumber,
            0x5f => Self::SysLinkCaptureReset,
            0x60 => Self::SysLinkLogNumber,
            0x61 => Self::SysLinkLogAll,
            0x62 => Self::SysLinkLogReset,
            0x63 => Self::SysLinkLogNumberInsertCommand,
            0x64 => Self::SysLinkConfig,
            0x65 => Self::SysLinkQsuspend,
            0x66 => Self::SysLinkQsuspendNumber,
            0x67 => Self::ErrorNumber,
            _ => Self::Other(v as u8),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::CheckPowerOn => 0x0,
            Self::CheckMarginHigh => 0x1,
            Self::CheckScrambleEnabled => 0x2,
            Self::CheckLinks => 0x3,
            Self::CheckDebugNumber => 0x4,
            Self::CheckSecurityOn => 0x5,
            Self::CheckSerialWord => 0x6,
            Self::FruCapture => 0x7,
            Self::FruCaptureVerbose => 0x8,
            Self::FruDump => 0x9,
            Self::FruReset => 0xa,
            Self::FruStatus => 0xb,
            Self::FruStatusVerbose => 0xc,
            Self::Deadlock => 0xd,
            Self::DeadlockVerbose => 0xe,
            Self::Memcfg => 0xf,
            Self::Memcfg1bank166E => 0x10,
            Self::Shcfg => 0x11,
            Self::ShcfgFsbReset => 0x12,
            Self::ShcfgFsb => 0x13,
            Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber => 0x14,
            Self::ShcfgWrt90Reset => 0x15,
            Self::ShcfgWrt90 => 0x16,
            Self::ShcfgWrt90NumberNumberNumber => 0x17,
            Self::ShcfgMciReset => 0x18,
            Self::ShcfgMci => 0x19,
            Self::ShcfgMciNumberNumberNumber => 0x1a,
            Self::ShcfgMdclk => 0x1b,
            Self::ShcfgMdclkNumber => 0x1c,
            Self::ShcfgNiclk => 0x1d,
            Self::ShcfgNiclkNumber => 0x1e,
            Self::ShcfgOsc => 0x1f,
            Self::ShcfgOscNumber => 0x20,
            Self::ShcfgGtlRe => 0x21,
            Self::ShcfgGtlReNumber => 0x22,
            Self::ShcfgJclkstop => 0x23,
            Self::ShcfgJclkstopNumber => 0x24,
            Self::ShcfgTp => 0x25,
            Self::ShcfgTpNumber => 0x26,
            Self::NicfgNi0 => 0x27,
            Self::NicfgNumberNi0 => 0x28,
            Self::NicfgNi0Number => 0x29,
            Self::NicfgNi0NumberNumber => 0x2a,
            Self::NicfgNi0NumberNumberNumber => 0x2b,
            Self::NicfgNi0NumberNumberNumberNumber => 0x2c,
            Self::NicfgNi0NumberNumberNumberNumberNumber => 0x2d,
            Self::NicfgNumberNi0Number => 0x2e,
            Self::NicfgNumberNi0NumberNumber => 0x2f,
            Self::NicfgNumberNi0NumberNumberNumber => 0x30,
            Self::NicfgNumberNi0NumberNumberNumberNumber => 0x31,
            Self::NicfgNumberNi0NumberNumberNumberNumberNumber => 0x32,
            Self::RmmrWordNumber => 0x33,
            Self::WmmrWordNumberNumberNumber => 0x34,
            Self::PktgenReadWordNumber => 0x35,
            Self::PktgenWriteWordNumberNumberNumber => 0x36,
            Self::PktgenDebugNumber => 0x37,
            Self::TuneGet => 0x38,
            Self::Promice => 0x39,
            Self::PromiceE => 0x3a,
            Self::PromiceD => 0x3b,
            Self::PromiceNumberE => 0x3c,
            Self::PromiceNumberD => 0x3d,
            Self::Promver => 0x3e,
            Self::Node => 0x3f,
            Self::NodeE => 0x40,
            Self::NodeD => 0x41,
            Self::NodeNumberE => 0x42,
            Self::NodeNumberD => 0x43,
            Self::PicMode => 0x44,
            Self::War => 0x45,
            Self::WarE => 0x46,
            Self::WarD => 0x47,
            Self::Shdbg => 0x48,
            Self::ShdbgRetrylimitNumber => 0x49,
            Self::ShdbgRdelayNumber => 0x4a,
            Self::ShdbgFullresetNumber => 0x4b,
            Self::ShdbgLinkdownNumber => 0x4c,
            Self::Mbrick => 0x4d,
            Self::MbrickMemory => 0x4e,
            Self::MbrickMemoryConfirm => 0x4f,
            Self::MbrickCpuWordWordWordWord => 0x50,
            Self::MbrickCpu => 0x51,
            Self::IfIOnlyHadABrain => 0x52,
            Self::Ssi => 0x53,
            Self::SsiServiceWordWordWordWord => 0x54,
            Self::SsiServiceOff => 0x55,
            Self::SsiWordWordWordWordWord => 0x56,
            Self::MadWithPowerNumber => 0x57,
            Self::PwmRead => 0x58,
            Self::PwmSetNumberNumber => 0x59,
            Self::SysLinkSimOn => 0x5a,
            Self::SysLinkSimOnNumberNumber => 0x5b,
            Self::SysLinkSimOff => 0x5c,
            Self::SysLinkDumpInputNumber => 0x5d,
            Self::SysLinkDumpOutputNumber => 0x5e,
            Self::SysLinkCaptureReset => 0x5f,
            Self::SysLinkLogNumber => 0x60,
            Self::SysLinkLogAll => 0x61,
            Self::SysLinkLogReset => 0x62,
            Self::SysLinkLogNumberInsertCommand => 0x63,
            Self::SysLinkConfig => 0x64,
            Self::SysLinkQsuspend => 0x65,
            Self::SysLinkQsuspendNumber => 0x66,
            Self::ErrorNumber => 0x67,
            Self::Other(v) => v as u32,
        }
    }
}

/// Values of `index` in `Cmd2` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd2Index {
    /// established.
    Pci,
    /// established.
    PciU,
    /// established.
    PciD,
    /// established.
    PciRst,
    /// established.
    PciRstHold,
    /// established.
    PciRstHoldNumber,
    /// established.
    PciRstHoldNumberNumber,
    /// established.
    PciRstRelease,
    /// established.
    PciRstReleaseNumber,
    /// established.
    PciRstReleaseNumberNumber,
    /// established.
    PciNumberU,
    /// established.
    PciNumberD,
    /// established.
    PciNumberRst,
    /// established.
    PciNumberNumberU,
    /// established.
    PciNumberNumberD,
    /// established.
    PciNumberNumberRst,
    /// established.
    PciForceAllOn,
    /// established.
    PciModeLimitNumberPci33,
    /// established.
    PciModeLimitPci33,
    /// established.
    PciModeForceNumberPci33,
    /// established.
    PciModeForcePci33,
    /// established.
    PciModeDefaultNumber,
    /// established.
    PciModeDefault,
    /// established.
    PciModeWarningDisable,
    /// established.
    PciModeWarningEnable,
    /// A value the notes do not name.
    Other(u8),
}

impl Cmd2Index {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x0 => Self::Pci,
            0x1 => Self::PciU,
            0x2 => Self::PciD,
            0x3 => Self::PciRst,
            0x4 => Self::PciRstHold,
            0x5 => Self::PciRstHoldNumber,
            0x6 => Self::PciRstHoldNumberNumber,
            0x7 => Self::PciRstRelease,
            0x8 => Self::PciRstReleaseNumber,
            0x9 => Self::PciRstReleaseNumberNumber,
            0xa => Self::PciNumberU,
            0xb => Self::PciNumberD,
            0xc => Self::PciNumberRst,
            0xd => Self::PciNumberNumberU,
            0xe => Self::PciNumberNumberD,
            0xf => Self::PciNumberNumberRst,
            0x10 => Self::PciForceAllOn,
            0x11 => Self::PciModeLimitNumberPci33,
            0x12 => Self::PciModeLimitPci33,
            0x13 => Self::PciModeForceNumberPci33,
            0x14 => Self::PciModeForcePci33,
            0x15 => Self::PciModeDefaultNumber,
            0x16 => Self::PciModeDefault,
            0x17 => Self::PciModeWarningDisable,
            0x18 => Self::PciModeWarningEnable,
            _ => Self::Other(v as u8),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::Pci => 0x0,
            Self::PciU => 0x1,
            Self::PciD => 0x2,
            Self::PciRst => 0x3,
            Self::PciRstHold => 0x4,
            Self::PciRstHoldNumber => 0x5,
            Self::PciRstHoldNumberNumber => 0x6,
            Self::PciRstRelease => 0x7,
            Self::PciRstReleaseNumber => 0x8,
            Self::PciRstReleaseNumberNumber => 0x9,
            Self::PciNumberU => 0xa,
            Self::PciNumberD => 0xb,
            Self::PciNumberRst => 0xc,
            Self::PciNumberNumberU => 0xd,
            Self::PciNumberNumberD => 0xe,
            Self::PciNumberNumberRst => 0xf,
            Self::PciForceAllOn => 0x10,
            Self::PciModeLimitNumberPci33 => 0x11,
            Self::PciModeLimitPci33 => 0x12,
            Self::PciModeForceNumberPci33 => 0x13,
            Self::PciModeForcePci33 => 0x14,
            Self::PciModeDefaultNumber => 0x15,
            Self::PciModeDefault => 0x16,
            Self::PciModeWarningDisable => 0x17,
            Self::PciModeWarningEnable => 0x18,
            Self::Other(v) => v as u32,
        }
    }
}

/// The `CMD0` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd0(u32);

impl Cmd0 {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 8:0, established.
    pub const fn index(self) -> Cmd0Index {
        Cmd0Index::from_bits(self.0 & 0x1ff)
    }
}

/// The `CMD1` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd1(u32);

impl Cmd1 {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 7:0, established.
    pub const fn index(self) -> Cmd1Index {
        Cmd1Index::from_bits(self.0 & 0xff)
    }
}

/// The `CMD2` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd2(u32);

impl Cmd2 {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 7:0, established.
    pub const fn index(self) -> Cmd2Index {
        Cmd2Index::from_bits(self.0 & 0xff)
    }
}
