// generated from sysco 50f318a26300 L1P2_DECODE.h - do not edit, regenerate in sysco
//!
//! Console request codes of one ColdFire L1 firmware package.
//! Codes are unique within the package.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// Values of `code` in `Cmd` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCode {
    /// established.
    GeneralReqReadDebugSwitches,
    /// established.
    GeneralReqRackBay,
    /// established.
    GeneralReqRackBayType,
    /// established.
    GeneralReqBrickSerial,
    /// established.
    GeneralReqFirmwareRevision,
    /// established.
    GeneralReqEepromRead,
    /// established.
    GeneralReqEepromFormat,
    /// established.
    GeneralReqSystemSerial,
    /// established.
    GeneralReqPartitionGet,
    /// established.
    GeneralReqPortSpeeds,
    /// established.
    GeneralReqTwoBitSetting,
    /// established.
    GeneralReqAttachedPorts1To4,
    /// established.
    GeneralReqAttachedPorts5And10,
    /// established.
    GeneralReqRequestingPort,
    /// established.
    GeneralReqLastEventReasons,
    /// established.
    GeneralReqProcessorMasks,
    /// established.
    GeneralReqScanRecord,
    /// established.
    GeneralReqWriteDebugSwitches,
    /// established.
    GeneralReqConsoleSubchannel,
    /// established.
    GeneralReqConsoleVolunteer,
    /// established.
    GeneralReqDisplayLine1,
    /// established.
    GeneralReqDisplayLine2,
    /// established.
    GeneralReqPartitionSet,
    /// established.
    GeneralReqEventSubchannelSet,
    /// established.
    GeneralReqTwoBitSettingSet,
    /// established.
    GeneralReqPanelControl,
    /// established.
    GeneralReqEventSubchannelClear,
    /// established.
    GeneralReqPromVersionReport,
    /// established.
    GeneralReqPerProcessorSetting,
    /// established.
    GeneralReqSystemReset,
    /// established.
    GeneralReqPciOperationA,
    /// established.
    GeneralReqPciOperationB,
    /// established.
    GeneralReqPciOperationC,
    /// established.
    GeneralReqPowerDown,
    /// established.
    GeneralReqLifecycleThirdAction,
    /// established.
    GeneralReqExpanderRead,
    /// established.
    GeneralReqExpanderWrite,
    /// established.
    GeneralReqInventory,
    /// established.
    GeneralReqControllerReboot,
    /// A value the notes do not name.
    Other(u32),
}

impl CmdCode {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x1 => Self::GeneralReqReadDebugSwitches,
            0x2 => Self::GeneralReqRackBay,
            0x3 => Self::GeneralReqRackBayType,
            0x4 => Self::GeneralReqBrickSerial,
            0x5 => Self::GeneralReqFirmwareRevision,
            0x6 => Self::GeneralReqEepromRead,
            0x7 => Self::GeneralReqEepromFormat,
            0x8 => Self::GeneralReqSystemSerial,
            0x9 => Self::GeneralReqPartitionGet,
            0xa => Self::GeneralReqPortSpeeds,
            0xb => Self::GeneralReqTwoBitSetting,
            0xc => Self::GeneralReqAttachedPorts1To4,
            0xd => Self::GeneralReqAttachedPorts5And10,
            0xe => Self::GeneralReqRequestingPort,
            0xf => Self::GeneralReqLastEventReasons,
            0x10 => Self::GeneralReqProcessorMasks,
            0x11 => Self::GeneralReqScanRecord,
            0x1001 => Self::GeneralReqWriteDebugSwitches,
            0x1002 => Self::GeneralReqConsoleSubchannel,
            0x1003 => Self::GeneralReqConsoleVolunteer,
            0x1004 => Self::GeneralReqDisplayLine1,
            0x1005 => Self::GeneralReqDisplayLine2,
            0x1006 => Self::GeneralReqPartitionSet,
            0x1007 => Self::GeneralReqEventSubchannelSet,
            0x1008 => Self::GeneralReqTwoBitSettingSet,
            0x1009 => Self::GeneralReqPanelControl,
            0x100a => Self::GeneralReqEventSubchannelClear,
            0x100b => Self::GeneralReqPromVersionReport,
            0x100c => Self::GeneralReqPerProcessorSetting,
            0x2001 => Self::GeneralReqSystemReset,
            0x2002 => Self::GeneralReqPciOperationA,
            0x2003 => Self::GeneralReqPciOperationB,
            0x2004 => Self::GeneralReqPciOperationC,
            0x2005 => Self::GeneralReqPowerDown,
            0x2006 => Self::GeneralReqLifecycleThirdAction,
            0x2007 => Self::GeneralReqExpanderRead,
            0x2008 => Self::GeneralReqExpanderWrite,
            0x4000 => Self::GeneralReqInventory,
            0x4002 => Self::GeneralReqControllerReboot,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::GeneralReqReadDebugSwitches => 0x1,
            Self::GeneralReqRackBay => 0x2,
            Self::GeneralReqRackBayType => 0x3,
            Self::GeneralReqBrickSerial => 0x4,
            Self::GeneralReqFirmwareRevision => 0x5,
            Self::GeneralReqEepromRead => 0x6,
            Self::GeneralReqEepromFormat => 0x7,
            Self::GeneralReqSystemSerial => 0x8,
            Self::GeneralReqPartitionGet => 0x9,
            Self::GeneralReqPortSpeeds => 0xa,
            Self::GeneralReqTwoBitSetting => 0xb,
            Self::GeneralReqAttachedPorts1To4 => 0xc,
            Self::GeneralReqAttachedPorts5And10 => 0xd,
            Self::GeneralReqRequestingPort => 0xe,
            Self::GeneralReqLastEventReasons => 0xf,
            Self::GeneralReqProcessorMasks => 0x10,
            Self::GeneralReqScanRecord => 0x11,
            Self::GeneralReqWriteDebugSwitches => 0x1001,
            Self::GeneralReqConsoleSubchannel => 0x1002,
            Self::GeneralReqConsoleVolunteer => 0x1003,
            Self::GeneralReqDisplayLine1 => 0x1004,
            Self::GeneralReqDisplayLine2 => 0x1005,
            Self::GeneralReqPartitionSet => 0x1006,
            Self::GeneralReqEventSubchannelSet => 0x1007,
            Self::GeneralReqTwoBitSettingSet => 0x1008,
            Self::GeneralReqPanelControl => 0x1009,
            Self::GeneralReqEventSubchannelClear => 0x100a,
            Self::GeneralReqPromVersionReport => 0x100b,
            Self::GeneralReqPerProcessorSetting => 0x100c,
            Self::GeneralReqSystemReset => 0x2001,
            Self::GeneralReqPciOperationA => 0x2002,
            Self::GeneralReqPciOperationB => 0x2003,
            Self::GeneralReqPciOperationC => 0x2004,
            Self::GeneralReqPowerDown => 0x2005,
            Self::GeneralReqLifecycleThirdAction => 0x2006,
            Self::GeneralReqExpanderRead => 0x2007,
            Self::GeneralReqExpanderWrite => 0x2008,
            Self::GeneralReqInventory => 0x4000,
            Self::GeneralReqControllerReboot => 0x4002,
            Self::Other(v) => v as u32,
        }
    }
}

/// The `CMD` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd(u32);

impl Cmd {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 15:0, established.
    pub const fn code(self) -> CmdCode {
        CmdCode::from_bits(self.0 & 0xffff)
    }
}
