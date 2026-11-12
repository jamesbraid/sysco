// generated from defs/l1_pkg3_requests.toml by defs-gen - do not edit, change the source
// console request codes of ColdFire L1 package 3
#![allow(dead_code)]

/// (established)
pub mod cmd {
    pub const WIDTH: u32 = 32;
    pub mod code { // bits 15:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xffff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const GENERAL_REQ_RACK_BAY: u32 = 2; // established
        pub const GENERAL_REQ_RACK_BAY_TYPE: u32 = 3; // established
        pub const GENERAL_REQ_BRICK_SERIAL: u32 = 4; // established
        pub const GENERAL_REQ_EEPROM_READ: u32 = 6; // established
        pub const GENERAL_REQ_EEPROM_FORMAT: u32 = 7; // established
        pub const GENERAL_REQ_SYSTEM_SERIAL: u32 = 8; // established
        pub const GENERAL_REQ_PORT_SPEEDS: u32 = 10; // established
        pub const GENERAL_REQ_ATTACHED_PORTS_1_TO_4: u32 = 12; // established
        pub const GENERAL_REQ_ATTACHED_PORTS_5_AND_10: u32 = 13; // established
        pub const GENERAL_REQ_REQUESTING_PORT: u32 = 14; // established
        pub const GENERAL_REQ_LAST_EVENT_REASONS: u32 = 15; // established
        pub const GENERAL_REQ_PROCESSOR_MASKS: u32 = 16; // established
        pub const GENERAL_REQ_SCAN_RECORD: u32 = 17; // established
        pub const GENERAL_REQ_TYPE_CHAR: u32 = 19; // established
        pub const GENERAL_REQ_WRITE_DEBUG_SWITCHES: u32 = 4097; // established
        pub const GENERAL_REQ_CONSOLE_SUBCHANNEL: u32 = 4098; // established
        pub const GENERAL_REQ_CONSOLE_VOLUNTEER: u32 = 4099; // established
        pub const GENERAL_REQ_DISPLAY_LINE_1: u32 = 4100; // established
        pub const GENERAL_REQ_DISPLAY_LINE_2: u32 = 4101; // established
        pub const GENERAL_REQ_PARTITION_SET: u32 = 4102; // established
        pub const GENERAL_REQ_EVENT_SUBCHANNEL_SET: u32 = 4103; // established
        pub const GENERAL_REQ_TWO_BIT_SETTING_SET: u32 = 4104; // established
        pub const GENERAL_REQ_PANEL_CONTROL: u32 = 4105; // established
        pub const GENERAL_REQ_EVENT_SUBCHANNEL_CLEAR: u32 = 4106; // established
        pub const GENERAL_REQ_PROM_VERSION_REPORT: u32 = 4107; // established
        pub const GENERAL_REQ_PER_PROCESSOR_SETTING: u32 = 4108; // established
        pub const GENERAL_REQ_SYSTEM_RESET: u32 = 8193; // established
        pub const GENERAL_REQ_PCI_OPERATION_A: u32 = 8194; // established
        pub const GENERAL_REQ_PCI_OPERATION_B: u32 = 8195; // established
        pub const GENERAL_REQ_PCI_OPERATION_C: u32 = 8196; // established
        pub const GENERAL_REQ_POWER_DOWN: u32 = 8197; // established
        pub const GENERAL_REQ_LIFECYCLE_THIRD_ACTION: u32 = 8198; // established
        pub const GENERAL_REQ_SET_CLOCK: u32 = 8205; // established
        pub const GENERAL_REQ_GET_CLOCK: u32 = 8206; // established
        pub const GENERAL_REQ_HOOK_3001: u32 = 12289; // established
        pub const GENERAL_REQ_INVENTORY: u32 = 16384; // established
        pub const GENERAL_REQ_WAR_FLAG: u32 = 16385; // established
        pub const GENERAL_REQ_CONTROLLER_REBOOT: u32 = 16386; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            GeneralReqRackBay,
            GeneralReqRackBayType,
            GeneralReqBrickSerial,
            GeneralReqEepromRead,
            GeneralReqEepromFormat,
            GeneralReqSystemSerial,
            GeneralReqPortSpeeds,
            GeneralReqAttachedPorts1To4,
            GeneralReqAttachedPorts5And10,
            GeneralReqRequestingPort,
            GeneralReqLastEventReasons,
            GeneralReqProcessorMasks,
            GeneralReqScanRecord,
            GeneralReqTypeChar,
            GeneralReqWriteDebugSwitches,
            GeneralReqConsoleSubchannel,
            GeneralReqConsoleVolunteer,
            GeneralReqDisplayLine1,
            GeneralReqDisplayLine2,
            GeneralReqPartitionSet,
            GeneralReqEventSubchannelSet,
            GeneralReqTwoBitSettingSet,
            GeneralReqPanelControl,
            GeneralReqEventSubchannelClear,
            GeneralReqPromVersionReport,
            GeneralReqPerProcessorSetting,
            GeneralReqSystemReset,
            GeneralReqPciOperationA,
            GeneralReqPciOperationB,
            GeneralReqPciOperationC,
            GeneralReqPowerDown,
            GeneralReqLifecycleThirdAction,
            GeneralReqSetClock,
            GeneralReqGetClock,
            GeneralReqHook3001,
            GeneralReqInventory,
            GeneralReqWarFlag,
            GeneralReqControllerReboot,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    2 => Self::GeneralReqRackBay,
                    3 => Self::GeneralReqRackBayType,
                    4 => Self::GeneralReqBrickSerial,
                    6 => Self::GeneralReqEepromRead,
                    7 => Self::GeneralReqEepromFormat,
                    8 => Self::GeneralReqSystemSerial,
                    10 => Self::GeneralReqPortSpeeds,
                    12 => Self::GeneralReqAttachedPorts1To4,
                    13 => Self::GeneralReqAttachedPorts5And10,
                    14 => Self::GeneralReqRequestingPort,
                    15 => Self::GeneralReqLastEventReasons,
                    16 => Self::GeneralReqProcessorMasks,
                    17 => Self::GeneralReqScanRecord,
                    19 => Self::GeneralReqTypeChar,
                    4097 => Self::GeneralReqWriteDebugSwitches,
                    4098 => Self::GeneralReqConsoleSubchannel,
                    4099 => Self::GeneralReqConsoleVolunteer,
                    4100 => Self::GeneralReqDisplayLine1,
                    4101 => Self::GeneralReqDisplayLine2,
                    4102 => Self::GeneralReqPartitionSet,
                    4103 => Self::GeneralReqEventSubchannelSet,
                    4104 => Self::GeneralReqTwoBitSettingSet,
                    4105 => Self::GeneralReqPanelControl,
                    4106 => Self::GeneralReqEventSubchannelClear,
                    4107 => Self::GeneralReqPromVersionReport,
                    4108 => Self::GeneralReqPerProcessorSetting,
                    8193 => Self::GeneralReqSystemReset,
                    8194 => Self::GeneralReqPciOperationA,
                    8195 => Self::GeneralReqPciOperationB,
                    8196 => Self::GeneralReqPciOperationC,
                    8197 => Self::GeneralReqPowerDown,
                    8198 => Self::GeneralReqLifecycleThirdAction,
                    8205 => Self::GeneralReqSetClock,
                    8206 => Self::GeneralReqGetClock,
                    12289 => Self::GeneralReqHook3001,
                    16384 => Self::GeneralReqInventory,
                    16385 => Self::GeneralReqWarFlag,
                    16386 => Self::GeneralReqControllerReboot,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::GeneralReqRackBay => 2,
                    Self::GeneralReqRackBayType => 3,
                    Self::GeneralReqBrickSerial => 4,
                    Self::GeneralReqEepromRead => 6,
                    Self::GeneralReqEepromFormat => 7,
                    Self::GeneralReqSystemSerial => 8,
                    Self::GeneralReqPortSpeeds => 10,
                    Self::GeneralReqAttachedPorts1To4 => 12,
                    Self::GeneralReqAttachedPorts5And10 => 13,
                    Self::GeneralReqRequestingPort => 14,
                    Self::GeneralReqLastEventReasons => 15,
                    Self::GeneralReqProcessorMasks => 16,
                    Self::GeneralReqScanRecord => 17,
                    Self::GeneralReqTypeChar => 19,
                    Self::GeneralReqWriteDebugSwitches => 4097,
                    Self::GeneralReqConsoleSubchannel => 4098,
                    Self::GeneralReqConsoleVolunteer => 4099,
                    Self::GeneralReqDisplayLine1 => 4100,
                    Self::GeneralReqDisplayLine2 => 4101,
                    Self::GeneralReqPartitionSet => 4102,
                    Self::GeneralReqEventSubchannelSet => 4103,
                    Self::GeneralReqTwoBitSettingSet => 4104,
                    Self::GeneralReqPanelControl => 4105,
                    Self::GeneralReqEventSubchannelClear => 4106,
                    Self::GeneralReqPromVersionReport => 4107,
                    Self::GeneralReqPerProcessorSetting => 4108,
                    Self::GeneralReqSystemReset => 8193,
                    Self::GeneralReqPciOperationA => 8194,
                    Self::GeneralReqPciOperationB => 8195,
                    Self::GeneralReqPciOperationC => 8196,
                    Self::GeneralReqPowerDown => 8197,
                    Self::GeneralReqLifecycleThirdAction => 8198,
                    Self::GeneralReqSetClock => 8205,
                    Self::GeneralReqGetClock => 8206,
                    Self::GeneralReqHook3001 => 12289,
                    Self::GeneralReqInventory => 16384,
                    Self::GeneralReqWarFlag => 16385,
                    Self::GeneralReqControllerReboot => 16386,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
