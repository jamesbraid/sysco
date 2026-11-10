// generated from sysco c0c9fa1403a4 L1P3_CONSOLE_DECODE.h - do not edit, regenerate in sysco
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
    CpuD,
    /// established.
    CpuWordE,
    /// established.
    CpuWordD,
    /// established.
    NiaCommand,
    /// established.
    NibCommand,
    /// established.
    IiaCommand,
    /// established.
    IibCommand,
    /// established.
    IicCommand,
    /// established.
    IidCommand,
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
    MarginGbrickDefault,
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
    NmiNumber,
    /// established.
    NmiRackSlotWord,
    /// established.
    Softreset,
    /// established.
    SoftresetNumber,
    /// established.
    Select,
    /// established.
    SelectNumberNumber,
    /// established.
    SelectSubchannelConsole,
    /// established.
    SelectSubchannelNumber,
    /// established.
    SelectSubchannelA,
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
    SerialDimm,
    /// established.
    SerialDimmV,
    /// established.
    SerialDimmCfg,
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
    EepromDimmUnsign,
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
    TestIoexpSetNumberWord,
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
            0x19 => Self::CpuD,
            0x1a => Self::CpuWordE,
            0x1b => Self::CpuWordD,
            0x1c => Self::NiaCommand,
            0x1d => Self::NibCommand,
            0x1e => Self::IiaCommand,
            0x1f => Self::IibCommand,
            0x20 => Self::IicCommand,
            0x21 => Self::IidCommand,
            0x22 => Self::Config,
            0x23 => Self::ConfigV,
            0x24 => Self::ConfigReset,
            0x25 => Self::ConfigRing,
            0x26 => Self::Debug,
            0x27 => Self::DebugNumber,
            0x28 => Self::Display,
            0x29 => Self::DisplayNumberCommand,
            0x2a => Self::DisplayPowerOn,
            0x2b => Self::DisplayAttentionOn,
            0x2c => Self::DisplayFailureOn,
            0x2d => Self::DisplayPowerOff,
            0x2e => Self::DisplayAttentionOff,
            0x2f => Self::DisplayFailureOff,
            0x30 => Self::Button,
            0x31 => Self::ButtonAllEnable,
            0x32 => Self::ButtonAllDisable,
            0x33 => Self::ButtonPwrEnable,
            0x34 => Self::ButtonPwrDisable,
            0x35 => Self::ButtonRstEnable,
            0x36 => Self::ButtonRstDisable,
            0x37 => Self::ButtonNmiEnable,
            0x38 => Self::ButtonNmiDisable,
            0x39 => Self::LetTheCarnageBegin,
            0x3a => Self::SlippedMyMind,
            0x3b => Self::SuperSizedPlease,
            0x3c => Self::Env,
            0x3d => Self::EnvCheck,
            0x3e => Self::EnvOn,
            0x3f => Self::EnvOff,
            0x40 => Self::EnvPowerOn,
            0x41 => Self::EnvPowerOff,
            0x42 => Self::EnvPowerMonitor,
            0x43 => Self::EnvTempOn,
            0x44 => Self::EnvTempOff,
            0x45 => Self::EnvAltitudeHigh,
            0x46 => Self::EnvAltitudeLow,
            0x47 => Self::EnvReset,
            0x48 => Self::EnvTestWarning,
            0x49 => Self::EnvTestFault,
            0x4a => Self::EnvTestTmpadv,
            0x4b => Self::EnvTestTmpcrit,
            0x4c => Self::EnvTestNumberNumberNumberNumberNumber,
            0x4d => Self::EnvTestPwrdn,
            0x4e => Self::EnvMessages,
            0x4f => Self::EnvMessagesFaultsonly,
            0x50 => Self::EnvMessagesAll,
            0x51 => Self::Fan,
            0x52 => Self::FanOnNumber,
            0x53 => Self::FanOn,
            0x54 => Self::FanSpeed,
            0x55 => Self::FanSpeedNumberNumber,
            0x56 => Self::FanSpeedNumber,
            0x57 => Self::FanSpeedLowNumber,
            0x58 => Self::FanSpeedLow,
            0x59 => Self::FanAutoOn,
            0x5a => Self::Help,
            0x5b => Self::HelpWord,
            0x5c => Self::HelpListHidden,
            0x5d => Self::HelpWordHidden,
            0x5e => Self::History,
            0x5f => Self::L1dbg,
            0x60 => Self::L1dbgSysLinkNumber,
            0x61 => Self::L1dbgSysLinkNumberNumber,
            0x62 => Self::L1dbgIrtrNumber,
            0x63 => Self::L1dbgEnvNumber,
            0x64 => Self::L1dbgEnvOff,
            0x65 => Self::L1dbgPortOn,
            0x66 => Self::L1dbgI2cOn,
            0x67 => Self::L1dbgMarginOn,
            0x68 => Self::L1dbgConsoleOn,
            0x69 => Self::L1dbgMempanicOn,
            0x6a => Self::L1dbgPppdumpOn,
            0x6b => Self::L1dbgPppdelayNumber,
            0x6c => Self::L1dbgPromreqOn,
            0x6d => Self::L1dbgQsuspNumberNumber,
            0x6e => Self::L1dbgPrintfOn,
            0x6f => Self::L1dbgFlashignoreNumberNumber,
            0x70 => Self::L1dbgMiscNumber,
            0x71 => Self::L1dbgGalNumber,
            0x72 => Self::Link,
            0x73 => Self::LinkVerbose,
            0x74 => Self::Log,
            0x75 => Self::LogClear,
            0x76 => Self::LogInsertCommand,
            0x77 => Self::Ioport,
            0x78 => Self::Ioport400,
            0x79 => Self::IoportA400,
            0x7a => Self::IoportB400,
            0x7b => Self::IoportForce400,
            0x7c => Self::IoportForceAlt1,
            0x7d => Self::IoportForceAlt2,
            0x7e => Self::IoportClksrcA,
            0x7f => Self::IoportClksrcB,
            0x80 => Self::IstatMemory,
            0x81 => Self::IstatQueues,
            0x82 => Self::IstatTasks,
            0x83 => Self::IstatPmalloc,
            0x84 => Self::IstatDumpWord,
            0x85 => Self::B2b,
            0x86 => Self::L1,
            0x87 => Self::Leds,
            0x88 => Self::Margin,
            0x89 => Self::MarginDefault,
            0x8a => Self::MarginGbrickDefault,
            0x8b => Self::MarginNumberDefault,
            0x8c => Self::MarginNumberNumber,
            0x8d => Self::MarginDimmVWord,
            0x8e => Self::Network,
            0x8f => Self::NetworkUsb,
            0x90 => Self::NetworkAutodetectOn,
            0x91 => Self::Pimm,
            0x92 => Self::PimmAuto,
            0x93 => Self::PimmClksrcExt,
            0x94 => Self::PimmClksrcInt,
            0x95 => Self::PimmClkmodeAsync,
            0x96 => Self::PimmClkmodeSync,
            0x97 => Self::Port,
            0x98 => Self::Power,
            0x99 => Self::PowerCheck,
            0x9a => Self::PowerVrm,
            0x9b => Self::PowerUp,
            0x9c => Self::PowerUpHold,
            0x9d => Self::PowerDown,
            0x9e => Self::Reset,
            0x9f => Self::ResetHold,
            0xa0 => Self::Nmi,
            0xa1 => Self::NmiNumber,
            0xa2 => Self::NmiRackSlotWord,
            0xa3 => Self::Softreset,
            0xa4 => Self::SoftresetNumber,
            0xa5 => Self::Select,
            0xa6 => Self::SelectNumberNumber,
            0xa7 => Self::SelectSubchannelConsole,
            0xa8 => Self::SelectSubchannelNumber,
            0xa9 => Self::SelectSubchannelA,
            0xaa => Self::SelectSubchannelWord,
            0xab => Self::SelectEnable,
            0xac => Self::SelectDisable,
            0xad => Self::SelectFilterOn,
            0xae => Self::SelectFilterOff,
            0xaf => Self::Serial,
            0xb0 => Self::SerialVerify,
            0xb1 => Self::SerialAll,
            0xb2 => Self::SerialAllV,
            0xb3 => Self::SerialDimm,
            0xb4 => Self::SerialDimmV,
            0xb5 => Self::SerialDimmCfg,
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
            0xc0 => Self::EepromDimmUnsign,
            0xc1 => Self::Uart,
            0xc2 => Self::UartSysLinkReset,
            0xc3 => Self::Usb,
            0xc4 => Self::Router,
            0xc5 => Self::RouterMeta,
            0xc6 => Self::RouterRepeater,
            0xc7 => Self::RouterOrdinary,
            0xc8 => Self::Router6Port,
            0xc9 => Self::Router6PortConfirm,
            0xca => Self::Router8Port,
            0xcb => Self::Router8PortWordWordWordWord,
            0xcc => Self::RouterServiceWordWordWordWord,
            0xcd => Self::RouterServiceOff,
            0xce => Self::RouterSpare,
            0xcf => Self::ServiceWordWordWordWord,
            0xd0 => Self::ServiceOff,
            0xd1 => Self::DateTz,
            0xd2 => Self::DateTzWord,
            0xd3 => Self::Date,
            0xd4 => Self::DateRaw,
            0xd5 => Self::DateWord,
            0xd6 => Self::DateWordWord,
            0xd7 => Self::NvramReset,
            0xd8 => Self::NvramReadNumberNumber,
            0xd9 => Self::NvramWriteNumberNumber,
            0xda => Self::NvramScratch,
            0xdb => Self::NvramScratchNumber,
            0xdc => Self::L2Command,
            0xdd => Self::SecurityVerify3400,
            0xde => Self::SecurityVerify3800,
            0xdf => Self::SleepNumberNumberCommand,
            0xe0 => Self::FlashDefaultReset,
            0xe1 => Self::FlashDefaultA,
            0xe2 => Self::FlashDefaultB,
            0xe3 => Self::FlashDefaultCurrent,
            0xe4 => Self::FlashDefaultOld,
            0xe5 => Self::FlashDefaultNew,
            0xe6 => Self::FlashStatus,
            0xe7 => Self::FlashStatusV,
            0xe8 => Self::RebootL1,
            0xe9 => Self::RebootL1A,
            0xea => Self::RebootL1B,
            0xeb => Self::RebootL1Current,
            0xec => Self::RebootL1Old,
            0xed => Self::RebootL1New,
            0xee => Self::RebootL1Other,
            0xef => Self::RebootL1AddrNumber,
            0xf0 => Self::RebootL1RawNumber,
            0xf1 => Self::Version,
            0xf2 => Self::Pbay,
            0xf3 => Self::PbayVersion,
            0xf4 => Self::PbayDps,
            0xf5 => Self::PbayDpsWord,
            0xf6 => Self::PbayDcport,
            0xf7 => Self::PbayDcportWord,
            0xf8 => Self::PbayFru,
            0xf9 => Self::PbayFruWord,
            0xfa => Self::PbayReset,
            0xfb => Self::PbayEnv,
            0xfc => Self::PbayEnvOn,
            0xfd => Self::PbayEnvOff,
            0xfe => Self::PbayInit,
            0xff => Self::PbaySerial,
            0x100 => Self::TestI2c,
            0x101 => Self::TestI2cNumber,
            0x102 => Self::TestIoexpSetNumberWord,
            0x103 => Self::TestIoexpGetNumber,
            0x104 => Self::TestIoexpGetAll,
            0x105 => Self::TestIntr,
            0x106 => Self::TestDisplay,
            0x107 => Self::TestExceptionMem,
            0x108 => Self::TestExceptionStack,
            0x109 => Self::TestExceptionWdog,
            0x10a => Self::TestExceptionFatal,
            0x10b => Self::ScanReset,
            0x10c => Self::ScanResetHard,
            0x10d => Self::ScanResetSoft,
            0x10e => Self::ScanResetBoth,
            0x10f => Self::ScanSelNumberNumber,
            0x110 => Self::ScanSelNumberNumberNumber,
            0x111 => Self::ScanSelNumberNumberNumberNumber,
            0x112 => Self::ScanIdsNumber,
            0x113 => Self::ScanSetTrst0,
            0x114 => Self::ScanSetTrst1,
            0x115 => Self::ScanSetTck0,
            0x116 => Self::ScanSetTck1,
            0x117 => Self::ScanSetTms0,
            0x118 => Self::ScanSetTms1,
            0x119 => Self::ScanSetTdi0,
            0x11a => Self::ScanSetTdi1,
            0x11b => Self::ScanSetPsi0,
            0x11c => Self::ScanSetPsi1,
            0x11d => Self::ScanGet,
            0x11e => Self::ScanDebugNumber,
            0x11f => Self::ScanDebug,
            0x120 => Self::ScanImpSetDllbypNumber,
            0x121 => Self::ScanImpSetNumber,
            0x122 => Self::ScanImpUnlock,
            0x123 => Self::ScanTestNumber,
            0x124 => Self::Fru,
            0x125 => Self::FruCommand,
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
            Self::CpuD => 0x19,
            Self::CpuWordE => 0x1a,
            Self::CpuWordD => 0x1b,
            Self::NiaCommand => 0x1c,
            Self::NibCommand => 0x1d,
            Self::IiaCommand => 0x1e,
            Self::IibCommand => 0x1f,
            Self::IicCommand => 0x20,
            Self::IidCommand => 0x21,
            Self::Config => 0x22,
            Self::ConfigV => 0x23,
            Self::ConfigReset => 0x24,
            Self::ConfigRing => 0x25,
            Self::Debug => 0x26,
            Self::DebugNumber => 0x27,
            Self::Display => 0x28,
            Self::DisplayNumberCommand => 0x29,
            Self::DisplayPowerOn => 0x2a,
            Self::DisplayAttentionOn => 0x2b,
            Self::DisplayFailureOn => 0x2c,
            Self::DisplayPowerOff => 0x2d,
            Self::DisplayAttentionOff => 0x2e,
            Self::DisplayFailureOff => 0x2f,
            Self::Button => 0x30,
            Self::ButtonAllEnable => 0x31,
            Self::ButtonAllDisable => 0x32,
            Self::ButtonPwrEnable => 0x33,
            Self::ButtonPwrDisable => 0x34,
            Self::ButtonRstEnable => 0x35,
            Self::ButtonRstDisable => 0x36,
            Self::ButtonNmiEnable => 0x37,
            Self::ButtonNmiDisable => 0x38,
            Self::LetTheCarnageBegin => 0x39,
            Self::SlippedMyMind => 0x3a,
            Self::SuperSizedPlease => 0x3b,
            Self::Env => 0x3c,
            Self::EnvCheck => 0x3d,
            Self::EnvOn => 0x3e,
            Self::EnvOff => 0x3f,
            Self::EnvPowerOn => 0x40,
            Self::EnvPowerOff => 0x41,
            Self::EnvPowerMonitor => 0x42,
            Self::EnvTempOn => 0x43,
            Self::EnvTempOff => 0x44,
            Self::EnvAltitudeHigh => 0x45,
            Self::EnvAltitudeLow => 0x46,
            Self::EnvReset => 0x47,
            Self::EnvTestWarning => 0x48,
            Self::EnvTestFault => 0x49,
            Self::EnvTestTmpadv => 0x4a,
            Self::EnvTestTmpcrit => 0x4b,
            Self::EnvTestNumberNumberNumberNumberNumber => 0x4c,
            Self::EnvTestPwrdn => 0x4d,
            Self::EnvMessages => 0x4e,
            Self::EnvMessagesFaultsonly => 0x4f,
            Self::EnvMessagesAll => 0x50,
            Self::Fan => 0x51,
            Self::FanOnNumber => 0x52,
            Self::FanOn => 0x53,
            Self::FanSpeed => 0x54,
            Self::FanSpeedNumberNumber => 0x55,
            Self::FanSpeedNumber => 0x56,
            Self::FanSpeedLowNumber => 0x57,
            Self::FanSpeedLow => 0x58,
            Self::FanAutoOn => 0x59,
            Self::Help => 0x5a,
            Self::HelpWord => 0x5b,
            Self::HelpListHidden => 0x5c,
            Self::HelpWordHidden => 0x5d,
            Self::History => 0x5e,
            Self::L1dbg => 0x5f,
            Self::L1dbgSysLinkNumber => 0x60,
            Self::L1dbgSysLinkNumberNumber => 0x61,
            Self::L1dbgIrtrNumber => 0x62,
            Self::L1dbgEnvNumber => 0x63,
            Self::L1dbgEnvOff => 0x64,
            Self::L1dbgPortOn => 0x65,
            Self::L1dbgI2cOn => 0x66,
            Self::L1dbgMarginOn => 0x67,
            Self::L1dbgConsoleOn => 0x68,
            Self::L1dbgMempanicOn => 0x69,
            Self::L1dbgPppdumpOn => 0x6a,
            Self::L1dbgPppdelayNumber => 0x6b,
            Self::L1dbgPromreqOn => 0x6c,
            Self::L1dbgQsuspNumberNumber => 0x6d,
            Self::L1dbgPrintfOn => 0x6e,
            Self::L1dbgFlashignoreNumberNumber => 0x6f,
            Self::L1dbgMiscNumber => 0x70,
            Self::L1dbgGalNumber => 0x71,
            Self::Link => 0x72,
            Self::LinkVerbose => 0x73,
            Self::Log => 0x74,
            Self::LogClear => 0x75,
            Self::LogInsertCommand => 0x76,
            Self::Ioport => 0x77,
            Self::Ioport400 => 0x78,
            Self::IoportA400 => 0x79,
            Self::IoportB400 => 0x7a,
            Self::IoportForce400 => 0x7b,
            Self::IoportForceAlt1 => 0x7c,
            Self::IoportForceAlt2 => 0x7d,
            Self::IoportClksrcA => 0x7e,
            Self::IoportClksrcB => 0x7f,
            Self::IstatMemory => 0x80,
            Self::IstatQueues => 0x81,
            Self::IstatTasks => 0x82,
            Self::IstatPmalloc => 0x83,
            Self::IstatDumpWord => 0x84,
            Self::B2b => 0x85,
            Self::L1 => 0x86,
            Self::Leds => 0x87,
            Self::Margin => 0x88,
            Self::MarginDefault => 0x89,
            Self::MarginGbrickDefault => 0x8a,
            Self::MarginNumberDefault => 0x8b,
            Self::MarginNumberNumber => 0x8c,
            Self::MarginDimmVWord => 0x8d,
            Self::Network => 0x8e,
            Self::NetworkUsb => 0x8f,
            Self::NetworkAutodetectOn => 0x90,
            Self::Pimm => 0x91,
            Self::PimmAuto => 0x92,
            Self::PimmClksrcExt => 0x93,
            Self::PimmClksrcInt => 0x94,
            Self::PimmClkmodeAsync => 0x95,
            Self::PimmClkmodeSync => 0x96,
            Self::Port => 0x97,
            Self::Power => 0x98,
            Self::PowerCheck => 0x99,
            Self::PowerVrm => 0x9a,
            Self::PowerUp => 0x9b,
            Self::PowerUpHold => 0x9c,
            Self::PowerDown => 0x9d,
            Self::Reset => 0x9e,
            Self::ResetHold => 0x9f,
            Self::Nmi => 0xa0,
            Self::NmiNumber => 0xa1,
            Self::NmiRackSlotWord => 0xa2,
            Self::Softreset => 0xa3,
            Self::SoftresetNumber => 0xa4,
            Self::Select => 0xa5,
            Self::SelectNumberNumber => 0xa6,
            Self::SelectSubchannelConsole => 0xa7,
            Self::SelectSubchannelNumber => 0xa8,
            Self::SelectSubchannelA => 0xa9,
            Self::SelectSubchannelWord => 0xaa,
            Self::SelectEnable => 0xab,
            Self::SelectDisable => 0xac,
            Self::SelectFilterOn => 0xad,
            Self::SelectFilterOff => 0xae,
            Self::Serial => 0xaf,
            Self::SerialVerify => 0xb0,
            Self::SerialAll => 0xb1,
            Self::SerialAllV => 0xb2,
            Self::SerialDimm => 0xb3,
            Self::SerialDimmV => 0xb4,
            Self::SerialDimmCfg => 0xb5,
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
            Self::EepromDimmUnsign => 0xc0,
            Self::Uart => 0xc1,
            Self::UartSysLinkReset => 0xc2,
            Self::Usb => 0xc3,
            Self::Router => 0xc4,
            Self::RouterMeta => 0xc5,
            Self::RouterRepeater => 0xc6,
            Self::RouterOrdinary => 0xc7,
            Self::Router6Port => 0xc8,
            Self::Router6PortConfirm => 0xc9,
            Self::Router8Port => 0xca,
            Self::Router8PortWordWordWordWord => 0xcb,
            Self::RouterServiceWordWordWordWord => 0xcc,
            Self::RouterServiceOff => 0xcd,
            Self::RouterSpare => 0xce,
            Self::ServiceWordWordWordWord => 0xcf,
            Self::ServiceOff => 0xd0,
            Self::DateTz => 0xd1,
            Self::DateTzWord => 0xd2,
            Self::Date => 0xd3,
            Self::DateRaw => 0xd4,
            Self::DateWord => 0xd5,
            Self::DateWordWord => 0xd6,
            Self::NvramReset => 0xd7,
            Self::NvramReadNumberNumber => 0xd8,
            Self::NvramWriteNumberNumber => 0xd9,
            Self::NvramScratch => 0xda,
            Self::NvramScratchNumber => 0xdb,
            Self::L2Command => 0xdc,
            Self::SecurityVerify3400 => 0xdd,
            Self::SecurityVerify3800 => 0xde,
            Self::SleepNumberNumberCommand => 0xdf,
            Self::FlashDefaultReset => 0xe0,
            Self::FlashDefaultA => 0xe1,
            Self::FlashDefaultB => 0xe2,
            Self::FlashDefaultCurrent => 0xe3,
            Self::FlashDefaultOld => 0xe4,
            Self::FlashDefaultNew => 0xe5,
            Self::FlashStatus => 0xe6,
            Self::FlashStatusV => 0xe7,
            Self::RebootL1 => 0xe8,
            Self::RebootL1A => 0xe9,
            Self::RebootL1B => 0xea,
            Self::RebootL1Current => 0xeb,
            Self::RebootL1Old => 0xec,
            Self::RebootL1New => 0xed,
            Self::RebootL1Other => 0xee,
            Self::RebootL1AddrNumber => 0xef,
            Self::RebootL1RawNumber => 0xf0,
            Self::Version => 0xf1,
            Self::Pbay => 0xf2,
            Self::PbayVersion => 0xf3,
            Self::PbayDps => 0xf4,
            Self::PbayDpsWord => 0xf5,
            Self::PbayDcport => 0xf6,
            Self::PbayDcportWord => 0xf7,
            Self::PbayFru => 0xf8,
            Self::PbayFruWord => 0xf9,
            Self::PbayReset => 0xfa,
            Self::PbayEnv => 0xfb,
            Self::PbayEnvOn => 0xfc,
            Self::PbayEnvOff => 0xfd,
            Self::PbayInit => 0xfe,
            Self::PbaySerial => 0xff,
            Self::TestI2c => 0x100,
            Self::TestI2cNumber => 0x101,
            Self::TestIoexpSetNumberWord => 0x102,
            Self::TestIoexpGetNumber => 0x103,
            Self::TestIoexpGetAll => 0x104,
            Self::TestIntr => 0x105,
            Self::TestDisplay => 0x106,
            Self::TestExceptionMem => 0x107,
            Self::TestExceptionStack => 0x108,
            Self::TestExceptionWdog => 0x109,
            Self::TestExceptionFatal => 0x10a,
            Self::ScanReset => 0x10b,
            Self::ScanResetHard => 0x10c,
            Self::ScanResetSoft => 0x10d,
            Self::ScanResetBoth => 0x10e,
            Self::ScanSelNumberNumber => 0x10f,
            Self::ScanSelNumberNumberNumber => 0x110,
            Self::ScanSelNumberNumberNumberNumber => 0x111,
            Self::ScanIdsNumber => 0x112,
            Self::ScanSetTrst0 => 0x113,
            Self::ScanSetTrst1 => 0x114,
            Self::ScanSetTck0 => 0x115,
            Self::ScanSetTck1 => 0x116,
            Self::ScanSetTms0 => 0x117,
            Self::ScanSetTms1 => 0x118,
            Self::ScanSetTdi0 => 0x119,
            Self::ScanSetTdi1 => 0x11a,
            Self::ScanSetPsi0 => 0x11b,
            Self::ScanSetPsi1 => 0x11c,
            Self::ScanGet => 0x11d,
            Self::ScanDebugNumber => 0x11e,
            Self::ScanDebug => 0x11f,
            Self::ScanImpSetDllbypNumber => 0x120,
            Self::ScanImpSetNumber => 0x121,
            Self::ScanImpUnlock => 0x122,
            Self::ScanTestNumber => 0x123,
            Self::Fru => 0x124,
            Self::FruCommand => 0x125,
            Self::Other(v) => v as u32,
        }
    }
}

/// Values of `index` in `Cmd1` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd1Index {
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

impl Cmd1Index {
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

/// Values of `index` in `Cmd2` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd2Index {
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
    FruExamine,
    /// established.
    FruDump,
    /// established.
    FruReset,
    /// established.
    FruDeadlock,
    /// established.
    FruDeadlockVerbose,
    /// established.
    FruDeadlockQuiet,
    /// established.
    FruEvents,
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
    Nlcfg,
    /// established.
    NlcfgWord,
    /// established.
    NlcfgWordWord,
    /// established.
    NlcfgWordNl3,
    /// established.
    NlcfgWordWordNl3,
    /// established.
    NlcfgWordEnable,
    /// established.
    NlcfgWordWordEnable,
    /// established.
    NlcfgWordDisable,
    /// established.
    NlcfgWordWordDisable,
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

impl Cmd2Index {
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
            0x9 => Self::FruExamine,
            0xa => Self::FruDump,
            0xb => Self::FruReset,
            0xc => Self::FruDeadlock,
            0xd => Self::FruDeadlockVerbose,
            0xe => Self::FruDeadlockQuiet,
            0xf => Self::FruEvents,
            0x10 => Self::Memcfg,
            0x11 => Self::Memcfg1bank166E,
            0x12 => Self::Shcfg,
            0x13 => Self::ShcfgFsbReset,
            0x14 => Self::ShcfgFsb,
            0x15 => Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
            0x16 => Self::ShcfgWrt90Reset,
            0x17 => Self::ShcfgWrt90,
            0x18 => Self::ShcfgWrt90NumberNumberNumber,
            0x19 => Self::ShcfgMciReset,
            0x1a => Self::ShcfgMci,
            0x1b => Self::ShcfgMciNumberNumberNumber,
            0x1c => Self::ShcfgMdclk,
            0x1d => Self::ShcfgMdclkNumber,
            0x1e => Self::ShcfgNiclk,
            0x1f => Self::ShcfgNiclkNumber,
            0x20 => Self::ShcfgOsc,
            0x21 => Self::ShcfgOscNumber,
            0x22 => Self::ShcfgGtlRe,
            0x23 => Self::ShcfgGtlReNumber,
            0x24 => Self::ShcfgJclkstop,
            0x25 => Self::ShcfgJclkstopNumber,
            0x26 => Self::ShcfgTp,
            0x27 => Self::ShcfgTpNumber,
            0x28 => Self::NicfgNi0,
            0x29 => Self::NicfgNumberNi0,
            0x2a => Self::NicfgNi0Number,
            0x2b => Self::NicfgNi0NumberNumber,
            0x2c => Self::NicfgNi0NumberNumberNumber,
            0x2d => Self::NicfgNi0NumberNumberNumberNumber,
            0x2e => Self::NicfgNi0NumberNumberNumberNumberNumber,
            0x2f => Self::NicfgNumberNi0Number,
            0x30 => Self::NicfgNumberNi0NumberNumber,
            0x31 => Self::NicfgNumberNi0NumberNumberNumber,
            0x32 => Self::NicfgNumberNi0NumberNumberNumberNumber,
            0x33 => Self::NicfgNumberNi0NumberNumberNumberNumberNumber,
            0x34 => Self::Nlcfg,
            0x35 => Self::NlcfgWord,
            0x36 => Self::NlcfgWordWord,
            0x37 => Self::NlcfgWordNl3,
            0x38 => Self::NlcfgWordWordNl3,
            0x39 => Self::NlcfgWordEnable,
            0x3a => Self::NlcfgWordWordEnable,
            0x3b => Self::NlcfgWordDisable,
            0x3c => Self::NlcfgWordWordDisable,
            0x3d => Self::RmmrWordNumber,
            0x3e => Self::WmmrWordNumberNumberNumber,
            0x3f => Self::PktgenReadWordNumber,
            0x40 => Self::PktgenWriteWordNumberNumberNumber,
            0x41 => Self::PktgenDebugNumber,
            0x42 => Self::TuneGet,
            0x43 => Self::Promice,
            0x44 => Self::PromiceE,
            0x45 => Self::PromiceD,
            0x46 => Self::PromiceNumberE,
            0x47 => Self::PromiceNumberD,
            0x48 => Self::Promver,
            0x49 => Self::Node,
            0x4a => Self::NodeE,
            0x4b => Self::NodeD,
            0x4c => Self::NodeNumberE,
            0x4d => Self::NodeNumberD,
            0x4e => Self::PicMode,
            0x4f => Self::War,
            0x50 => Self::WarE,
            0x51 => Self::WarD,
            0x52 => Self::Shdbg,
            0x53 => Self::ShdbgRetrylimitNumber,
            0x54 => Self::ShdbgRdelayNumber,
            0x55 => Self::ShdbgFullresetNumber,
            0x56 => Self::ShdbgLinkdownNumber,
            0x57 => Self::Mbrick,
            0x58 => Self::MbrickMemory,
            0x59 => Self::MbrickMemoryConfirm,
            0x5a => Self::MbrickCpuWordWordWordWord,
            0x5b => Self::MbrickCpu,
            0x5c => Self::IfIOnlyHadABrain,
            0x5d => Self::Ssi,
            0x5e => Self::SsiServiceWordWordWordWord,
            0x5f => Self::SsiServiceOff,
            0x60 => Self::SsiWordWordWordWordWord,
            0x61 => Self::MadWithPowerNumber,
            0x62 => Self::PwmRead,
            0x63 => Self::PwmSetNumberNumber,
            0x64 => Self::SysLinkSimOn,
            0x65 => Self::SysLinkSimOnNumberNumber,
            0x66 => Self::SysLinkSimOff,
            0x67 => Self::SysLinkDumpInputNumber,
            0x68 => Self::SysLinkDumpOutputNumber,
            0x69 => Self::SysLinkCaptureReset,
            0x6a => Self::SysLinkLogNumber,
            0x6b => Self::SysLinkLogAll,
            0x6c => Self::SysLinkLogReset,
            0x6d => Self::SysLinkLogNumberInsertCommand,
            0x6e => Self::SysLinkConfig,
            0x6f => Self::SysLinkQsuspend,
            0x70 => Self::SysLinkQsuspendNumber,
            0x71 => Self::ErrorNumber,
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
            Self::FruExamine => 0x9,
            Self::FruDump => 0xa,
            Self::FruReset => 0xb,
            Self::FruDeadlock => 0xc,
            Self::FruDeadlockVerbose => 0xd,
            Self::FruDeadlockQuiet => 0xe,
            Self::FruEvents => 0xf,
            Self::Memcfg => 0x10,
            Self::Memcfg1bank166E => 0x11,
            Self::Shcfg => 0x12,
            Self::ShcfgFsbReset => 0x13,
            Self::ShcfgFsb => 0x14,
            Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber => 0x15,
            Self::ShcfgWrt90Reset => 0x16,
            Self::ShcfgWrt90 => 0x17,
            Self::ShcfgWrt90NumberNumberNumber => 0x18,
            Self::ShcfgMciReset => 0x19,
            Self::ShcfgMci => 0x1a,
            Self::ShcfgMciNumberNumberNumber => 0x1b,
            Self::ShcfgMdclk => 0x1c,
            Self::ShcfgMdclkNumber => 0x1d,
            Self::ShcfgNiclk => 0x1e,
            Self::ShcfgNiclkNumber => 0x1f,
            Self::ShcfgOsc => 0x20,
            Self::ShcfgOscNumber => 0x21,
            Self::ShcfgGtlRe => 0x22,
            Self::ShcfgGtlReNumber => 0x23,
            Self::ShcfgJclkstop => 0x24,
            Self::ShcfgJclkstopNumber => 0x25,
            Self::ShcfgTp => 0x26,
            Self::ShcfgTpNumber => 0x27,
            Self::NicfgNi0 => 0x28,
            Self::NicfgNumberNi0 => 0x29,
            Self::NicfgNi0Number => 0x2a,
            Self::NicfgNi0NumberNumber => 0x2b,
            Self::NicfgNi0NumberNumberNumber => 0x2c,
            Self::NicfgNi0NumberNumberNumberNumber => 0x2d,
            Self::NicfgNi0NumberNumberNumberNumberNumber => 0x2e,
            Self::NicfgNumberNi0Number => 0x2f,
            Self::NicfgNumberNi0NumberNumber => 0x30,
            Self::NicfgNumberNi0NumberNumberNumber => 0x31,
            Self::NicfgNumberNi0NumberNumberNumberNumber => 0x32,
            Self::NicfgNumberNi0NumberNumberNumberNumberNumber => 0x33,
            Self::Nlcfg => 0x34,
            Self::NlcfgWord => 0x35,
            Self::NlcfgWordWord => 0x36,
            Self::NlcfgWordNl3 => 0x37,
            Self::NlcfgWordWordNl3 => 0x38,
            Self::NlcfgWordEnable => 0x39,
            Self::NlcfgWordWordEnable => 0x3a,
            Self::NlcfgWordDisable => 0x3b,
            Self::NlcfgWordWordDisable => 0x3c,
            Self::RmmrWordNumber => 0x3d,
            Self::WmmrWordNumberNumberNumber => 0x3e,
            Self::PktgenReadWordNumber => 0x3f,
            Self::PktgenWriteWordNumberNumberNumber => 0x40,
            Self::PktgenDebugNumber => 0x41,
            Self::TuneGet => 0x42,
            Self::Promice => 0x43,
            Self::PromiceE => 0x44,
            Self::PromiceD => 0x45,
            Self::PromiceNumberE => 0x46,
            Self::PromiceNumberD => 0x47,
            Self::Promver => 0x48,
            Self::Node => 0x49,
            Self::NodeE => 0x4a,
            Self::NodeD => 0x4b,
            Self::NodeNumberE => 0x4c,
            Self::NodeNumberD => 0x4d,
            Self::PicMode => 0x4e,
            Self::War => 0x4f,
            Self::WarE => 0x50,
            Self::WarD => 0x51,
            Self::Shdbg => 0x52,
            Self::ShdbgRetrylimitNumber => 0x53,
            Self::ShdbgRdelayNumber => 0x54,
            Self::ShdbgFullresetNumber => 0x55,
            Self::ShdbgLinkdownNumber => 0x56,
            Self::Mbrick => 0x57,
            Self::MbrickMemory => 0x58,
            Self::MbrickMemoryConfirm => 0x59,
            Self::MbrickCpuWordWordWordWord => 0x5a,
            Self::MbrickCpu => 0x5b,
            Self::IfIOnlyHadABrain => 0x5c,
            Self::Ssi => 0x5d,
            Self::SsiServiceWordWordWordWord => 0x5e,
            Self::SsiServiceOff => 0x5f,
            Self::SsiWordWordWordWordWord => 0x60,
            Self::MadWithPowerNumber => 0x61,
            Self::PwmRead => 0x62,
            Self::PwmSetNumberNumber => 0x63,
            Self::SysLinkSimOn => 0x64,
            Self::SysLinkSimOnNumberNumber => 0x65,
            Self::SysLinkSimOff => 0x66,
            Self::SysLinkDumpInputNumber => 0x67,
            Self::SysLinkDumpOutputNumber => 0x68,
            Self::SysLinkCaptureReset => 0x69,
            Self::SysLinkLogNumber => 0x6a,
            Self::SysLinkLogAll => 0x6b,
            Self::SysLinkLogReset => 0x6c,
            Self::SysLinkLogNumberInsertCommand => 0x6d,
            Self::SysLinkConfig => 0x6e,
            Self::SysLinkQsuspend => 0x6f,
            Self::SysLinkQsuspendNumber => 0x70,
            Self::ErrorNumber => 0x71,
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
