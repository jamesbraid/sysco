// generated from sysco c0c9fa1403a4 L1P1_CONSOLE_DECODE.h - do not edit, regenerate in sysco
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
    Pci,
    /// established.
    PciU,
    /// established.
    PciD,
    /// established.
    PciRst,
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
    PciPowerCheck,
    /// established.
    PciPowerCheckOn,
    /// established.
    PciPowerCheckOff,
    /// A value the notes do not name.
    Other(u8),
}

impl Cmd0Index {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x0 => Self::Pci,
            0x1 => Self::PciU,
            0x2 => Self::PciD,
            0x3 => Self::PciRst,
            0x4 => Self::PciNumberU,
            0x5 => Self::PciNumberD,
            0x6 => Self::PciNumberRst,
            0x7 => Self::PciNumberNumberU,
            0x8 => Self::PciNumberNumberD,
            0x9 => Self::PciNumberNumberRst,
            0xa => Self::PciPowerCheck,
            0xb => Self::PciPowerCheckOn,
            0xc => Self::PciPowerCheckOff,
            _ => Self::Other(v as u8),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::Pci => 0x0,
            Self::PciU => 0x1,
            Self::PciD => 0x2,
            Self::PciRst => 0x3,
            Self::PciNumberU => 0x4,
            Self::PciNumberD => 0x5,
            Self::PciNumberRst => 0x6,
            Self::PciNumberNumberU => 0x7,
            Self::PciNumberNumberD => 0x8,
            Self::PciNumberNumberRst => 0x9,
            Self::PciPowerCheck => 0xa,
            Self::PciPowerCheckOn => 0xb,
            Self::PciPowerCheckOff => 0xc,
            Self::Other(v) => v as u32,
        }
    }
}

/// Values of `index` in `Cmd1` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd1Index {
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
    BrickPtypeWord,
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
    MarginPciNorm2hi,
    /// established.
    MarginPciNorm2hiOn,
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
    EepromFuelWriteDefault,
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

impl Cmd1Index {
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
            0xe => Self::BrickPtypeWord,
            0xf => Self::BrickRackDecimal,
            0x10 => Self::BrickSlotDecimal,
            0x11 => Self::BrickRackslotDecimalDecimal,
            0x12 => Self::BrickRackslotResetNumberNumber,
            0x13 => Self::BrickPartitionNone,
            0x14 => Self::BrickPartitionNumber,
            0x15 => Self::Partdb,
            0x16 => Self::PartdbEnable,
            0x17 => Self::PartdbDisable,
            0x18 => Self::Cpu,
            0x19 => Self::CpuE,
            0x1a => Self::CpuD,
            0x1b => Self::CpuWordE,
            0x1c => Self::CpuWordD,
            0x1d => Self::NiaCommand,
            0x1e => Self::NibCommand,
            0x1f => Self::IiaCommand,
            0x20 => Self::IibCommand,
            0x21 => Self::IicCommand,
            0x22 => Self::IidCommand,
            0x23 => Self::Config,
            0x24 => Self::ConfigV,
            0x25 => Self::ConfigReset,
            0x26 => Self::ConfigRing,
            0x27 => Self::Debug,
            0x28 => Self::DebugNumber,
            0x29 => Self::Display,
            0x2a => Self::DisplayNumberCommand,
            0x2b => Self::DisplayPowerOn,
            0x2c => Self::DisplayAttentionOn,
            0x2d => Self::DisplayFailureOn,
            0x2e => Self::DisplayPowerOff,
            0x2f => Self::DisplayAttentionOff,
            0x30 => Self::DisplayFailureOff,
            0x31 => Self::Button,
            0x32 => Self::ButtonAllEnable,
            0x33 => Self::ButtonAllDisable,
            0x34 => Self::ButtonPwrEnable,
            0x35 => Self::ButtonPwrDisable,
            0x36 => Self::ButtonRstEnable,
            0x37 => Self::ButtonRstDisable,
            0x38 => Self::ButtonNmiEnable,
            0x39 => Self::ButtonNmiDisable,
            0x3a => Self::LetTheCarnageBegin,
            0x3b => Self::SlippedMyMind,
            0x3c => Self::SuperSizedPlease,
            0x3d => Self::Env,
            0x3e => Self::EnvCheck,
            0x3f => Self::EnvOn,
            0x40 => Self::EnvOff,
            0x41 => Self::EnvPowerOn,
            0x42 => Self::EnvPowerOff,
            0x43 => Self::EnvPowerMonitor,
            0x44 => Self::EnvTempOn,
            0x45 => Self::EnvTempOff,
            0x46 => Self::EnvAltitudeHigh,
            0x47 => Self::EnvAltitudeLow,
            0x48 => Self::EnvReset,
            0x49 => Self::EnvTestWarning,
            0x4a => Self::EnvTestFault,
            0x4b => Self::EnvTestTmpadv,
            0x4c => Self::EnvTestTmpcrit,
            0x4d => Self::EnvTestNumberNumberNumberNumberNumber,
            0x4e => Self::EnvTestPwrdn,
            0x4f => Self::EnvMessages,
            0x50 => Self::EnvMessagesFaultsonly,
            0x51 => Self::EnvMessagesAll,
            0x52 => Self::Fan,
            0x53 => Self::FanOnNumber,
            0x54 => Self::FanOn,
            0x55 => Self::FanSpeed,
            0x56 => Self::FanSpeedNumberNumber,
            0x57 => Self::FanSpeedNumber,
            0x58 => Self::FanSpeedLowNumber,
            0x59 => Self::FanSpeedLow,
            0x5a => Self::FanAutoOn,
            0x5b => Self::Help,
            0x5c => Self::HelpWord,
            0x5d => Self::HelpListHidden,
            0x5e => Self::HelpWordHidden,
            0x5f => Self::History,
            0x60 => Self::L1dbg,
            0x61 => Self::L1dbgSysLinkNumber,
            0x62 => Self::L1dbgSysLinkNumberNumber,
            0x63 => Self::L1dbgIrtrNumber,
            0x64 => Self::L1dbgEnvNumber,
            0x65 => Self::L1dbgEnvOff,
            0x66 => Self::L1dbgPortOn,
            0x67 => Self::L1dbgI2cOn,
            0x68 => Self::L1dbgMarginOn,
            0x69 => Self::L1dbgConsoleOn,
            0x6a => Self::L1dbgMempanicOn,
            0x6b => Self::L1dbgPppdumpOn,
            0x6c => Self::L1dbgPppdelayNumber,
            0x6d => Self::L1dbgPromreqOn,
            0x6e => Self::L1dbgQsuspNumberNumber,
            0x6f => Self::L1dbgPrintfOn,
            0x70 => Self::L1dbgFlashignoreNumberNumber,
            0x71 => Self::L1dbgMiscNumber,
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
            0x8e => Self::MarginPciNorm2hi,
            0x8f => Self::MarginPciNorm2hiOn,
            0x90 => Self::Network,
            0x91 => Self::NetworkUsb,
            0x92 => Self::NetworkAutodetectOn,
            0x93 => Self::Pimm,
            0x94 => Self::PimmAuto,
            0x95 => Self::PimmClksrcExt,
            0x96 => Self::PimmClksrcInt,
            0x97 => Self::PimmClkmodeAsync,
            0x98 => Self::PimmClkmodeSync,
            0x99 => Self::Port,
            0x9a => Self::Power,
            0x9b => Self::PowerCheck,
            0x9c => Self::PowerVrm,
            0x9d => Self::PowerUp,
            0x9e => Self::PowerUpHold,
            0x9f => Self::PowerDown,
            0xa0 => Self::Reset,
            0xa1 => Self::ResetHold,
            0xa2 => Self::Nmi,
            0xa3 => Self::NmiNumber,
            0xa4 => Self::NmiRackSlotWord,
            0xa5 => Self::Softreset,
            0xa6 => Self::SoftresetNumber,
            0xa7 => Self::Select,
            0xa8 => Self::SelectNumberNumber,
            0xa9 => Self::SelectSubchannelConsole,
            0xaa => Self::SelectSubchannelNumber,
            0xab => Self::SelectSubchannelA,
            0xac => Self::SelectSubchannelWord,
            0xad => Self::SelectEnable,
            0xae => Self::SelectDisable,
            0xaf => Self::SelectFilterOn,
            0xb0 => Self::SelectFilterOff,
            0xb1 => Self::Serial,
            0xb2 => Self::SerialVerify,
            0xb3 => Self::SerialAll,
            0xb4 => Self::SerialAllV,
            0xb5 => Self::SerialDimm,
            0xb6 => Self::SerialDimmV,
            0xb7 => Self::SerialClear,
            0xb8 => Self::SerialSetWord,
            0xb9 => Self::SerialWordWordWordWord,
            0xba => Self::SerialSecurityOffWordWordWordWord,
            0xbb => Self::SerialSecurityOn,
            0xbc => Self::SerialBrickDisable,
            0xbd => Self::SerialBrickDisableConfirm,
            0xbe => Self::Sysstate,
            0xbf => Self::Eeprom,
            0xc0 => Self::EepromNumberNumberNumber,
            0xc1 => Self::EepromFuelWriteDefault,
            0xc2 => Self::EepromDimmUnsign,
            0xc3 => Self::Uart,
            0xc4 => Self::UartSysLinkReset,
            0xc5 => Self::Usb,
            0xc6 => Self::Router,
            0xc7 => Self::RouterMeta,
            0xc8 => Self::RouterRepeater,
            0xc9 => Self::RouterOrdinary,
            0xca => Self::Router6Port,
            0xcb => Self::Router6PortConfirm,
            0xcc => Self::Router8Port,
            0xcd => Self::Router8PortWordWordWordWord,
            0xce => Self::RouterServiceWordWordWordWord,
            0xcf => Self::RouterServiceOff,
            0xd0 => Self::RouterSpare,
            0xd1 => Self::ServiceWordWordWordWord,
            0xd2 => Self::ServiceOff,
            0xd3 => Self::DateTz,
            0xd4 => Self::DateTzWord,
            0xd5 => Self::Date,
            0xd6 => Self::DateRaw,
            0xd7 => Self::DateWord,
            0xd8 => Self::DateWordWord,
            0xd9 => Self::NvramReset,
            0xda => Self::NvramReadNumberNumber,
            0xdb => Self::NvramWriteNumberNumber,
            0xdc => Self::NvramScratch,
            0xdd => Self::NvramScratchNumber,
            0xde => Self::L2Command,
            0xdf => Self::SecurityVerify3400,
            0xe0 => Self::SecurityVerify3800,
            0xe1 => Self::FlashDefaultReset,
            0xe2 => Self::FlashDefaultA,
            0xe3 => Self::FlashDefaultB,
            0xe4 => Self::FlashDefaultCurrent,
            0xe5 => Self::FlashDefaultOld,
            0xe6 => Self::FlashDefaultNew,
            0xe7 => Self::FlashStatus,
            0xe8 => Self::FlashStatusV,
            0xe9 => Self::RebootL1,
            0xea => Self::RebootL1A,
            0xeb => Self::RebootL1B,
            0xec => Self::RebootL1Current,
            0xed => Self::RebootL1Old,
            0xee => Self::RebootL1New,
            0xef => Self::RebootL1Other,
            0xf0 => Self::RebootL1AddrNumber,
            0xf1 => Self::RebootL1RawNumber,
            0xf2 => Self::Version,
            0xf3 => Self::Pbay,
            0xf4 => Self::PbayVersion,
            0xf5 => Self::PbayDps,
            0xf6 => Self::PbayDpsWord,
            0xf7 => Self::PbayDcport,
            0xf8 => Self::PbayDcportWord,
            0xf9 => Self::PbayFru,
            0xfa => Self::PbayFruWord,
            0xfb => Self::PbayReset,
            0xfc => Self::PbayEnv,
            0xfd => Self::PbayEnvOn,
            0xfe => Self::PbayEnvOff,
            0xff => Self::PbayInit,
            0x100 => Self::PbaySerial,
            0x101 => Self::TestI2c,
            0x102 => Self::TestI2cNumber,
            0x103 => Self::TestIoexpSetNumberWord,
            0x104 => Self::TestIoexpGetNumber,
            0x105 => Self::TestIoexpGetAll,
            0x106 => Self::TestIntr,
            0x107 => Self::TestDisplay,
            0x108 => Self::TestExceptionMem,
            0x109 => Self::TestExceptionStack,
            0x10a => Self::TestExceptionWdog,
            0x10b => Self::TestExceptionFatal,
            0x10c => Self::ScanReset,
            0x10d => Self::ScanResetHard,
            0x10e => Self::ScanResetSoft,
            0x10f => Self::ScanResetBoth,
            0x110 => Self::ScanSelNumberNumber,
            0x111 => Self::ScanSelNumberNumberNumber,
            0x112 => Self::ScanSelNumberNumberNumberNumber,
            0x113 => Self::ScanIdsNumber,
            0x114 => Self::ScanSetTrst0,
            0x115 => Self::ScanSetTrst1,
            0x116 => Self::ScanSetTck0,
            0x117 => Self::ScanSetTck1,
            0x118 => Self::ScanSetTms0,
            0x119 => Self::ScanSetTms1,
            0x11a => Self::ScanSetTdi0,
            0x11b => Self::ScanSetTdi1,
            0x11c => Self::ScanSetPsi0,
            0x11d => Self::ScanSetPsi1,
            0x11e => Self::ScanGet,
            0x11f => Self::ScanDebugNumber,
            0x120 => Self::ScanDebug,
            0x121 => Self::ScanImpSetDllbypNumber,
            0x122 => Self::ScanImpSetNumber,
            0x123 => Self::ScanImpUnlock,
            0x124 => Self::ScanTestNumber,
            0x125 => Self::Fru,
            0x126 => Self::FruCommand,
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
            Self::BrickPtypeWord => 0xe,
            Self::BrickRackDecimal => 0xf,
            Self::BrickSlotDecimal => 0x10,
            Self::BrickRackslotDecimalDecimal => 0x11,
            Self::BrickRackslotResetNumberNumber => 0x12,
            Self::BrickPartitionNone => 0x13,
            Self::BrickPartitionNumber => 0x14,
            Self::Partdb => 0x15,
            Self::PartdbEnable => 0x16,
            Self::PartdbDisable => 0x17,
            Self::Cpu => 0x18,
            Self::CpuE => 0x19,
            Self::CpuD => 0x1a,
            Self::CpuWordE => 0x1b,
            Self::CpuWordD => 0x1c,
            Self::NiaCommand => 0x1d,
            Self::NibCommand => 0x1e,
            Self::IiaCommand => 0x1f,
            Self::IibCommand => 0x20,
            Self::IicCommand => 0x21,
            Self::IidCommand => 0x22,
            Self::Config => 0x23,
            Self::ConfigV => 0x24,
            Self::ConfigReset => 0x25,
            Self::ConfigRing => 0x26,
            Self::Debug => 0x27,
            Self::DebugNumber => 0x28,
            Self::Display => 0x29,
            Self::DisplayNumberCommand => 0x2a,
            Self::DisplayPowerOn => 0x2b,
            Self::DisplayAttentionOn => 0x2c,
            Self::DisplayFailureOn => 0x2d,
            Self::DisplayPowerOff => 0x2e,
            Self::DisplayAttentionOff => 0x2f,
            Self::DisplayFailureOff => 0x30,
            Self::Button => 0x31,
            Self::ButtonAllEnable => 0x32,
            Self::ButtonAllDisable => 0x33,
            Self::ButtonPwrEnable => 0x34,
            Self::ButtonPwrDisable => 0x35,
            Self::ButtonRstEnable => 0x36,
            Self::ButtonRstDisable => 0x37,
            Self::ButtonNmiEnable => 0x38,
            Self::ButtonNmiDisable => 0x39,
            Self::LetTheCarnageBegin => 0x3a,
            Self::SlippedMyMind => 0x3b,
            Self::SuperSizedPlease => 0x3c,
            Self::Env => 0x3d,
            Self::EnvCheck => 0x3e,
            Self::EnvOn => 0x3f,
            Self::EnvOff => 0x40,
            Self::EnvPowerOn => 0x41,
            Self::EnvPowerOff => 0x42,
            Self::EnvPowerMonitor => 0x43,
            Self::EnvTempOn => 0x44,
            Self::EnvTempOff => 0x45,
            Self::EnvAltitudeHigh => 0x46,
            Self::EnvAltitudeLow => 0x47,
            Self::EnvReset => 0x48,
            Self::EnvTestWarning => 0x49,
            Self::EnvTestFault => 0x4a,
            Self::EnvTestTmpadv => 0x4b,
            Self::EnvTestTmpcrit => 0x4c,
            Self::EnvTestNumberNumberNumberNumberNumber => 0x4d,
            Self::EnvTestPwrdn => 0x4e,
            Self::EnvMessages => 0x4f,
            Self::EnvMessagesFaultsonly => 0x50,
            Self::EnvMessagesAll => 0x51,
            Self::Fan => 0x52,
            Self::FanOnNumber => 0x53,
            Self::FanOn => 0x54,
            Self::FanSpeed => 0x55,
            Self::FanSpeedNumberNumber => 0x56,
            Self::FanSpeedNumber => 0x57,
            Self::FanSpeedLowNumber => 0x58,
            Self::FanSpeedLow => 0x59,
            Self::FanAutoOn => 0x5a,
            Self::Help => 0x5b,
            Self::HelpWord => 0x5c,
            Self::HelpListHidden => 0x5d,
            Self::HelpWordHidden => 0x5e,
            Self::History => 0x5f,
            Self::L1dbg => 0x60,
            Self::L1dbgSysLinkNumber => 0x61,
            Self::L1dbgSysLinkNumberNumber => 0x62,
            Self::L1dbgIrtrNumber => 0x63,
            Self::L1dbgEnvNumber => 0x64,
            Self::L1dbgEnvOff => 0x65,
            Self::L1dbgPortOn => 0x66,
            Self::L1dbgI2cOn => 0x67,
            Self::L1dbgMarginOn => 0x68,
            Self::L1dbgConsoleOn => 0x69,
            Self::L1dbgMempanicOn => 0x6a,
            Self::L1dbgPppdumpOn => 0x6b,
            Self::L1dbgPppdelayNumber => 0x6c,
            Self::L1dbgPromreqOn => 0x6d,
            Self::L1dbgQsuspNumberNumber => 0x6e,
            Self::L1dbgPrintfOn => 0x6f,
            Self::L1dbgFlashignoreNumberNumber => 0x70,
            Self::L1dbgMiscNumber => 0x71,
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
            Self::MarginPciNorm2hi => 0x8e,
            Self::MarginPciNorm2hiOn => 0x8f,
            Self::Network => 0x90,
            Self::NetworkUsb => 0x91,
            Self::NetworkAutodetectOn => 0x92,
            Self::Pimm => 0x93,
            Self::PimmAuto => 0x94,
            Self::PimmClksrcExt => 0x95,
            Self::PimmClksrcInt => 0x96,
            Self::PimmClkmodeAsync => 0x97,
            Self::PimmClkmodeSync => 0x98,
            Self::Port => 0x99,
            Self::Power => 0x9a,
            Self::PowerCheck => 0x9b,
            Self::PowerVrm => 0x9c,
            Self::PowerUp => 0x9d,
            Self::PowerUpHold => 0x9e,
            Self::PowerDown => 0x9f,
            Self::Reset => 0xa0,
            Self::ResetHold => 0xa1,
            Self::Nmi => 0xa2,
            Self::NmiNumber => 0xa3,
            Self::NmiRackSlotWord => 0xa4,
            Self::Softreset => 0xa5,
            Self::SoftresetNumber => 0xa6,
            Self::Select => 0xa7,
            Self::SelectNumberNumber => 0xa8,
            Self::SelectSubchannelConsole => 0xa9,
            Self::SelectSubchannelNumber => 0xaa,
            Self::SelectSubchannelA => 0xab,
            Self::SelectSubchannelWord => 0xac,
            Self::SelectEnable => 0xad,
            Self::SelectDisable => 0xae,
            Self::SelectFilterOn => 0xaf,
            Self::SelectFilterOff => 0xb0,
            Self::Serial => 0xb1,
            Self::SerialVerify => 0xb2,
            Self::SerialAll => 0xb3,
            Self::SerialAllV => 0xb4,
            Self::SerialDimm => 0xb5,
            Self::SerialDimmV => 0xb6,
            Self::SerialClear => 0xb7,
            Self::SerialSetWord => 0xb8,
            Self::SerialWordWordWordWord => 0xb9,
            Self::SerialSecurityOffWordWordWordWord => 0xba,
            Self::SerialSecurityOn => 0xbb,
            Self::SerialBrickDisable => 0xbc,
            Self::SerialBrickDisableConfirm => 0xbd,
            Self::Sysstate => 0xbe,
            Self::Eeprom => 0xbf,
            Self::EepromNumberNumberNumber => 0xc0,
            Self::EepromFuelWriteDefault => 0xc1,
            Self::EepromDimmUnsign => 0xc2,
            Self::Uart => 0xc3,
            Self::UartSysLinkReset => 0xc4,
            Self::Usb => 0xc5,
            Self::Router => 0xc6,
            Self::RouterMeta => 0xc7,
            Self::RouterRepeater => 0xc8,
            Self::RouterOrdinary => 0xc9,
            Self::Router6Port => 0xca,
            Self::Router6PortConfirm => 0xcb,
            Self::Router8Port => 0xcc,
            Self::Router8PortWordWordWordWord => 0xcd,
            Self::RouterServiceWordWordWordWord => 0xce,
            Self::RouterServiceOff => 0xcf,
            Self::RouterSpare => 0xd0,
            Self::ServiceWordWordWordWord => 0xd1,
            Self::ServiceOff => 0xd2,
            Self::DateTz => 0xd3,
            Self::DateTzWord => 0xd4,
            Self::Date => 0xd5,
            Self::DateRaw => 0xd6,
            Self::DateWord => 0xd7,
            Self::DateWordWord => 0xd8,
            Self::NvramReset => 0xd9,
            Self::NvramReadNumberNumber => 0xda,
            Self::NvramWriteNumberNumber => 0xdb,
            Self::NvramScratch => 0xdc,
            Self::NvramScratchNumber => 0xdd,
            Self::L2Command => 0xde,
            Self::SecurityVerify3400 => 0xdf,
            Self::SecurityVerify3800 => 0xe0,
            Self::FlashDefaultReset => 0xe1,
            Self::FlashDefaultA => 0xe2,
            Self::FlashDefaultB => 0xe3,
            Self::FlashDefaultCurrent => 0xe4,
            Self::FlashDefaultOld => 0xe5,
            Self::FlashDefaultNew => 0xe6,
            Self::FlashStatus => 0xe7,
            Self::FlashStatusV => 0xe8,
            Self::RebootL1 => 0xe9,
            Self::RebootL1A => 0xea,
            Self::RebootL1B => 0xeb,
            Self::RebootL1Current => 0xec,
            Self::RebootL1Old => 0xed,
            Self::RebootL1New => 0xee,
            Self::RebootL1Other => 0xef,
            Self::RebootL1AddrNumber => 0xf0,
            Self::RebootL1RawNumber => 0xf1,
            Self::Version => 0xf2,
            Self::Pbay => 0xf3,
            Self::PbayVersion => 0xf4,
            Self::PbayDps => 0xf5,
            Self::PbayDpsWord => 0xf6,
            Self::PbayDcport => 0xf7,
            Self::PbayDcportWord => 0xf8,
            Self::PbayFru => 0xf9,
            Self::PbayFruWord => 0xfa,
            Self::PbayReset => 0xfb,
            Self::PbayEnv => 0xfc,
            Self::PbayEnvOn => 0xfd,
            Self::PbayEnvOff => 0xfe,
            Self::PbayInit => 0xff,
            Self::PbaySerial => 0x100,
            Self::TestI2c => 0x101,
            Self::TestI2cNumber => 0x102,
            Self::TestIoexpSetNumberWord => 0x103,
            Self::TestIoexpGetNumber => 0x104,
            Self::TestIoexpGetAll => 0x105,
            Self::TestIntr => 0x106,
            Self::TestDisplay => 0x107,
            Self::TestExceptionMem => 0x108,
            Self::TestExceptionStack => 0x109,
            Self::TestExceptionWdog => 0x10a,
            Self::TestExceptionFatal => 0x10b,
            Self::ScanReset => 0x10c,
            Self::ScanResetHard => 0x10d,
            Self::ScanResetSoft => 0x10e,
            Self::ScanResetBoth => 0x10f,
            Self::ScanSelNumberNumber => 0x110,
            Self::ScanSelNumberNumberNumber => 0x111,
            Self::ScanSelNumberNumberNumberNumber => 0x112,
            Self::ScanIdsNumber => 0x113,
            Self::ScanSetTrst0 => 0x114,
            Self::ScanSetTrst1 => 0x115,
            Self::ScanSetTck0 => 0x116,
            Self::ScanSetTck1 => 0x117,
            Self::ScanSetTms0 => 0x118,
            Self::ScanSetTms1 => 0x119,
            Self::ScanSetTdi0 => 0x11a,
            Self::ScanSetTdi1 => 0x11b,
            Self::ScanSetPsi0 => 0x11c,
            Self::ScanSetPsi1 => 0x11d,
            Self::ScanGet => 0x11e,
            Self::ScanDebugNumber => 0x11f,
            Self::ScanDebug => 0x120,
            Self::ScanImpSetDllbypNumber => 0x121,
            Self::ScanImpSetNumber => 0x122,
            Self::ScanImpUnlock => 0x123,
            Self::ScanTestNumber => 0x124,
            Self::Fru => 0x125,
            Self::FruCommand => 0x126,
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

    /// Bits 7:0, established.
    pub const fn index(self) -> Cmd0Index {
        Cmd0Index::from_bits(self.0 & 0xff)
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

    /// Bits 8:0, established.
    pub const fn index(self) -> Cmd1Index {
        Cmd1Index::from_bits(self.0 & 0x1ff)
    }
}
