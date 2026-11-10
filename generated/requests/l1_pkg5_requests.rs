// generated from sysco 50f318a26300 L1P5_DECODE.h - do not edit, regenerate in sysco
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
    GeneralReqRackBay,
    /// established.
    GeneralReqRackBayType,
    /// established.
    GeneralReqBrickSerial,
    /// established.
    GeneralReqEepromRead,
    /// established.
    GeneralReqEepromFormat,
    /// established.
    GeneralReqSystemSerial,
    /// established.
    GeneralReqPortSpeeds,
    /// established.
    GeneralReqAttachedPorts1To4,
    /// established.
    GeneralReqAttachedPorts5And10,
    /// established.
    GeneralReqRequestingPort,
    /// established.
    GeneralReqProcessorMasks,
    /// established.
    GeneralReqScanRecord,
    /// established.
    GeneralReqTypeChar,
    /// established.
    GeneralReqWriteDebugSwitches,
    /// established.
    GeneralReqDisplayLine1,
    /// established.
    GeneralReqDisplayLine2,
    /// inferred.
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
    GeneralReqSetClock,
    /// established.
    GeneralReqGetClock,
    /// established.
    GeneralReqHook3001,
    /// established.
    GeneralReqFru3002,
    /// established.
    GeneralReqFru3003,
    /// established.
    GeneralReqFru3004,
    /// established.
    GeneralReqInventory,
    /// established.
    GeneralReqWarFlag,
    /// established.
    GeneralReqControllerReboot,
    /// established.
    GeneralReqDimmText,
    /// A value the notes do not name.
    Other(u32),
}

impl CmdCode {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x2 => Self::GeneralReqRackBay,
            0x3 => Self::GeneralReqRackBayType,
            0x4 => Self::GeneralReqBrickSerial,
            0x6 => Self::GeneralReqEepromRead,
            0x7 => Self::GeneralReqEepromFormat,
            0x8 => Self::GeneralReqSystemSerial,
            0xa => Self::GeneralReqPortSpeeds,
            0xc => Self::GeneralReqAttachedPorts1To4,
            0xd => Self::GeneralReqAttachedPorts5And10,
            0xe => Self::GeneralReqRequestingPort,
            0x10 => Self::GeneralReqProcessorMasks,
            0x11 => Self::GeneralReqScanRecord,
            0x13 => Self::GeneralReqTypeChar,
            0x1001 => Self::GeneralReqWriteDebugSwitches,
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
            0x200d => Self::GeneralReqSetClock,
            0x200e => Self::GeneralReqGetClock,
            0x3001 => Self::GeneralReqHook3001,
            0x3002 => Self::GeneralReqFru3002,
            0x3003 => Self::GeneralReqFru3003,
            0x3004 => Self::GeneralReqFru3004,
            0x4000 => Self::GeneralReqInventory,
            0x4001 => Self::GeneralReqWarFlag,
            0x4002 => Self::GeneralReqControllerReboot,
            0x4004 => Self::GeneralReqDimmText,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::GeneralReqRackBay => 0x2,
            Self::GeneralReqRackBayType => 0x3,
            Self::GeneralReqBrickSerial => 0x4,
            Self::GeneralReqEepromRead => 0x6,
            Self::GeneralReqEepromFormat => 0x7,
            Self::GeneralReqSystemSerial => 0x8,
            Self::GeneralReqPortSpeeds => 0xa,
            Self::GeneralReqAttachedPorts1To4 => 0xc,
            Self::GeneralReqAttachedPorts5And10 => 0xd,
            Self::GeneralReqRequestingPort => 0xe,
            Self::GeneralReqProcessorMasks => 0x10,
            Self::GeneralReqScanRecord => 0x11,
            Self::GeneralReqTypeChar => 0x13,
            Self::GeneralReqWriteDebugSwitches => 0x1001,
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
            Self::GeneralReqSetClock => 0x200d,
            Self::GeneralReqGetClock => 0x200e,
            Self::GeneralReqHook3001 => 0x3001,
            Self::GeneralReqFru3002 => 0x3002,
            Self::GeneralReqFru3003 => 0x3003,
            Self::GeneralReqFru3004 => 0x3004,
            Self::GeneralReqInventory => 0x4000,
            Self::GeneralReqWarFlag => 0x4001,
            Self::GeneralReqControllerReboot => 0x4002,
            Self::GeneralReqDimmText => 0x4004,
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
