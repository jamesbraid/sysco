// generated from formats/l1_pkg3_console/format.toml by defs-gen - do not edit, change the source
// console command tables of ColdFire L1 firmware package 3
#![allow(dead_code)]

/// (established)
pub mod cmd0 {
    pub const WIDTH: u32 = 32;
    pub mod index { // bits 8:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0x1ff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const ALL_COMMAND: u32 = 0; // established
        pub const RACK_SLOT_COMMAND: u32 = 1; // established
        pub const AUTOPOWER: u32 = 2; // established
        pub const AUTOPOWER_ON: u32 = 3; // established
        pub const AUTOPOWER_OFF: u32 = 4; // established
        pub const AUTOPOWER_ABORT: u32 = 5; // established
        pub const SYS_LINK: u32 = 6; // established
        pub const SYS_LINK_NONE: u32 = 7; // established
        pub const SYS_LINK_PPP: u32 = 8; // established
        pub const SYS_LINK_ENABLE: u32 = 9; // established
        pub const SYS_LINK_RESET: u32 = 10; // established
        pub const SYS_LINK_PACKETS: u32 = 11; // established
        pub const BRICK: u32 = 12; // established
        pub const BRICK_TYPE_WORD: u32 = 13; // established
        pub const BRICK_RACK_DECIMAL: u32 = 14; // established
        pub const BRICK_SLOT_DECIMAL: u32 = 15; // established
        pub const BRICK_RACKSLOT_DECIMAL_DECIMAL: u32 = 16; // established
        pub const BRICK_RACKSLOT_RESET_NUMBER_NUMBER: u32 = 17; // established
        pub const BRICK_PARTITION_NONE: u32 = 18; // established
        pub const BRICK_PARTITION_NUMBER: u32 = 19; // established
        pub const PARTDB: u32 = 20; // established
        pub const PARTDB_ENABLE: u32 = 21; // established
        pub const PARTDB_DISABLE: u32 = 22; // established
        pub const CPU: u32 = 23; // established
        pub const CPU_E: u32 = 24; // established
        pub const CPU_D: u32 = 25; // established
        pub const CPU_WORD_E: u32 = 26; // established
        pub const CPU_WORD_D: u32 = 27; // established
        pub const NIA_COMMAND: u32 = 28; // established
        pub const NIB_COMMAND: u32 = 29; // established
        pub const IIA_COMMAND: u32 = 30; // established
        pub const IIB_COMMAND: u32 = 31; // established
        pub const IIC_COMMAND: u32 = 32; // established
        pub const IID_COMMAND: u32 = 33; // established
        pub const CONFIG: u32 = 34; // established
        pub const CONFIG_V: u32 = 35; // established
        pub const CONFIG_RESET: u32 = 36; // established
        pub const CONFIG_RING: u32 = 37; // established
        pub const DEBUG: u32 = 38; // established
        pub const DEBUG_NUMBER: u32 = 39; // established
        pub const DISPLAY: u32 = 40; // established
        pub const DISPLAY_NUMBER_COMMAND: u32 = 41; // established
        pub const DISPLAY_POWER_ON: u32 = 42; // established
        pub const DISPLAY_ATTENTION_ON: u32 = 43; // established
        pub const DISPLAY_FAILURE_ON: u32 = 44; // established
        pub const DISPLAY_POWER_OFF: u32 = 45; // established
        pub const DISPLAY_ATTENTION_OFF: u32 = 46; // established
        pub const DISPLAY_FAILURE_OFF: u32 = 47; // established
        pub const BUTTON: u32 = 48; // established
        pub const BUTTON_ALL_ENABLE: u32 = 49; // established
        pub const BUTTON_ALL_DISABLE: u32 = 50; // established
        pub const BUTTON_PWR_ENABLE: u32 = 51; // established
        pub const BUTTON_PWR_DISABLE: u32 = 52; // established
        pub const BUTTON_RST_ENABLE: u32 = 53; // established
        pub const BUTTON_RST_DISABLE: u32 = 54; // established
        pub const BUTTON_NMI_ENABLE: u32 = 55; // established
        pub const BUTTON_NMI_DISABLE: u32 = 56; // established
        pub const LET_THE_CARNAGE_BEGIN: u32 = 57; // established
        pub const SLIPPED_MY_MIND: u32 = 58; // established
        pub const SUPER_SIZED_PLEASE: u32 = 59; // established
        pub const ENV: u32 = 60; // established
        pub const ENV_CHECK: u32 = 61; // established
        pub const ENV_ON: u32 = 62; // established
        pub const ENV_OFF: u32 = 63; // established
        pub const ENV_POWER_ON: u32 = 64; // established
        pub const ENV_POWER_OFF: u32 = 65; // established
        pub const ENV_POWER_MONITOR: u32 = 66; // established
        pub const ENV_TEMP_ON: u32 = 67; // established
        pub const ENV_TEMP_OFF: u32 = 68; // established
        pub const ENV_ALTITUDE_HIGH: u32 = 69; // established
        pub const ENV_ALTITUDE_LOW: u32 = 70; // established
        pub const ENV_RESET: u32 = 71; // established
        pub const ENV_TEST_WARNING: u32 = 72; // established
        pub const ENV_TEST_FAULT: u32 = 73; // established
        pub const ENV_TEST_TMPADV: u32 = 74; // established
        pub const ENV_TEST_TMPCRIT: u32 = 75; // established
        pub const ENV_TEST_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 76; // established
        pub const ENV_TEST_PWRDN: u32 = 77; // established
        pub const ENV_MESSAGES: u32 = 78; // established
        pub const ENV_MESSAGES_FAULTSONLY: u32 = 79; // established
        pub const ENV_MESSAGES_ALL: u32 = 80; // established
        pub const FAN: u32 = 81; // established
        pub const FAN_ON_NUMBER: u32 = 82; // established
        pub const FAN_ON: u32 = 83; // established
        pub const FAN_SPEED: u32 = 84; // established
        pub const FAN_SPEED_NUMBER_NUMBER: u32 = 85; // established
        pub const FAN_SPEED_NUMBER: u32 = 86; // established
        pub const FAN_SPEED_LOW_NUMBER: u32 = 87; // established
        pub const FAN_SPEED_LOW: u32 = 88; // established
        pub const FAN_AUTO_ON: u32 = 89; // established
        pub const HELP: u32 = 90; // established
        pub const HELP_WORD: u32 = 91; // established
        pub const HELP_LIST_HIDDEN: u32 = 92; // established
        pub const HELP_WORD_HIDDEN: u32 = 93; // established
        pub const HISTORY: u32 = 94; // established
        pub const L1DBG: u32 = 95; // established
        pub const L1DBG_SYS_LINK_NUMBER: u32 = 96; // established
        pub const L1DBG_SYS_LINK_NUMBER_NUMBER: u32 = 97; // established
        pub const L1DBG_IRTR_NUMBER: u32 = 98; // established
        pub const L1DBG_ENV_NUMBER: u32 = 99; // established
        pub const L1DBG_ENV_OFF: u32 = 100; // established
        pub const L1DBG_PORT_ON: u32 = 101; // established
        pub const L1DBG_I2C_ON: u32 = 102; // established
        pub const L1DBG_MARGIN_ON: u32 = 103; // established
        pub const L1DBG_CONSOLE_ON: u32 = 104; // established
        pub const L1DBG_MEMPANIC_ON: u32 = 105; // established
        pub const L1DBG_PPPDUMP_ON: u32 = 106; // established
        pub const L1DBG_PPPDELAY_NUMBER: u32 = 107; // established
        pub const L1DBG_PROMREQ_ON: u32 = 108; // established
        pub const L1DBG_QSUSP_NUMBER_NUMBER: u32 = 109; // established
        pub const L1DBG_PRINTF_ON: u32 = 110; // established
        pub const L1DBG_FLASHIGNORE_NUMBER_NUMBER: u32 = 111; // established
        pub const L1DBG_MISC_NUMBER: u32 = 112; // established
        pub const L1DBG_GAL_NUMBER: u32 = 113; // established
        pub const LINK: u32 = 114; // established
        pub const LINK_VERBOSE: u32 = 115; // established
        pub const LOG: u32 = 116; // established
        pub const LOG_CLEAR: u32 = 117; // established
        pub const LOG_INSERT_COMMAND: u32 = 118; // established
        pub const IOPORT: u32 = 119; // established
        pub const IOPORT_400: u32 = 120; // established
        pub const IOPORT_A_400: u32 = 121; // established
        pub const IOPORT_B_400: u32 = 122; // established
        pub const IOPORT_FORCE_400: u32 = 123; // established
        pub const IOPORT_FORCE_ALT1: u32 = 124; // established
        pub const IOPORT_FORCE_ALT2: u32 = 125; // established
        pub const IOPORT_CLKSRC_A: u32 = 126; // established
        pub const IOPORT_CLKSRC_B: u32 = 127; // established
        pub const ISTAT_MEMORY: u32 = 128; // established
        pub const ISTAT_QUEUES: u32 = 129; // established
        pub const ISTAT_TASKS: u32 = 130; // established
        pub const ISTAT_PMALLOC: u32 = 131; // established
        pub const ISTAT_DUMP_WORD: u32 = 132; // established
        pub const B2B: u32 = 133; // established
        pub const L1: u32 = 134; // established
        pub const LEDS: u32 = 135; // established
        pub const MARGIN: u32 = 136; // established
        pub const MARGIN_DEFAULT: u32 = 137; // established
        pub const MARGIN_GBRICK_DEFAULT: u32 = 138; // established
        pub const MARGIN_NUMBER_DEFAULT: u32 = 139; // established
        pub const MARGIN_NUMBER_NUMBER: u32 = 140; // established
        pub const MARGIN_DIMM_V_WORD: u32 = 141; // established
        pub const NETWORK: u32 = 142; // established
        pub const NETWORK_USB: u32 = 143; // established
        pub const NETWORK_AUTODETECT_ON: u32 = 144; // established
        pub const PIMM: u32 = 145; // established
        pub const PIMM_AUTO: u32 = 146; // established
        pub const PIMM_CLKSRC_EXT: u32 = 147; // established
        pub const PIMM_CLKSRC_INT: u32 = 148; // established
        pub const PIMM_CLKMODE_ASYNC: u32 = 149; // established
        pub const PIMM_CLKMODE_SYNC: u32 = 150; // established
        pub const PORT: u32 = 151; // established
        pub const POWER: u32 = 152; // established
        pub const POWER_CHECK: u32 = 153; // established
        pub const POWER_VRM: u32 = 154; // established
        pub const POWER_UP: u32 = 155; // established
        pub const POWER_UP_HOLD: u32 = 156; // established
        pub const POWER_DOWN: u32 = 157; // established
        pub const RESET: u32 = 158; // established
        pub const RESET_HOLD: u32 = 159; // established
        pub const NMI: u32 = 160; // established
        pub const NMI_NUMBER: u32 = 161; // established
        pub const NMI_RACK_SLOT_WORD: u32 = 162; // established
        pub const SOFTRESET: u32 = 163; // established
        pub const SOFTRESET_NUMBER: u32 = 164; // established
        pub const SELECT: u32 = 165; // established
        pub const SELECT_NUMBER_NUMBER: u32 = 166; // established
        pub const SELECT_SUBCHANNEL_CONSOLE: u32 = 167; // established
        pub const SELECT_SUBCHANNEL_NUMBER: u32 = 168; // established
        pub const SELECT_SUBCHANNEL_A: u32 = 169; // established
        pub const SELECT_SUBCHANNEL_WORD: u32 = 170; // established
        pub const SELECT_ENABLE: u32 = 171; // established
        pub const SELECT_DISABLE: u32 = 172; // established
        pub const SELECT_FILTER_ON: u32 = 173; // established
        pub const SELECT_FILTER_OFF: u32 = 174; // established
        pub const SERIAL: u32 = 175; // established
        pub const SERIAL_VERIFY: u32 = 176; // established
        pub const SERIAL_ALL: u32 = 177; // established
        pub const SERIAL_ALL_V: u32 = 178; // established
        pub const SERIAL_DIMM: u32 = 179; // established
        pub const SERIAL_DIMM_V: u32 = 180; // established
        pub const SERIAL_DIMM_CFG: u32 = 181; // established
        pub const SERIAL_CLEAR: u32 = 182; // established
        pub const SERIAL_SET_WORD: u32 = 183; // established
        pub const SERIAL_WORD_WORD_WORD_WORD: u32 = 184; // established
        pub const SERIAL_SECURITY_OFF_WORD_WORD_WORD_WORD: u32 = 185; // established
        pub const SERIAL_SECURITY_ON: u32 = 186; // established
        pub const SERIAL_BRICK_DISABLE: u32 = 187; // established
        pub const SERIAL_BRICK_DISABLE_CONFIRM: u32 = 188; // established
        pub const SYSSTATE: u32 = 189; // established
        pub const EEPROM: u32 = 190; // established
        pub const EEPROM_NUMBER_NUMBER_NUMBER: u32 = 191; // established
        pub const EEPROM_DIMM_UNSIGN: u32 = 192; // established
        pub const UART: u32 = 193; // established
        pub const UART_SYS_LINK_RESET: u32 = 194; // established
        pub const USB: u32 = 195; // established
        pub const ROUTER: u32 = 196; // established
        pub const ROUTER_META: u32 = 197; // established
        pub const ROUTER_REPEATER: u32 = 198; // established
        pub const ROUTER_ORDINARY: u32 = 199; // established
        pub const ROUTER_6_PORT: u32 = 200; // established
        pub const ROUTER_6_PORT_CONFIRM: u32 = 201; // established
        pub const ROUTER_8_PORT: u32 = 202; // established
        pub const ROUTER_8_PORT_WORD_WORD_WORD_WORD: u32 = 203; // established
        pub const ROUTER_SERVICE_WORD_WORD_WORD_WORD: u32 = 204; // established
        pub const ROUTER_SERVICE_OFF: u32 = 205; // established
        pub const ROUTER_SPARE: u32 = 206; // established
        pub const SERVICE_WORD_WORD_WORD_WORD: u32 = 207; // established
        pub const SERVICE_OFF: u32 = 208; // established
        pub const DATE_TZ: u32 = 209; // established
        pub const DATE_TZ_WORD: u32 = 210; // established
        pub const DATE: u32 = 211; // established
        pub const DATE_RAW: u32 = 212; // established
        pub const DATE_WORD: u32 = 213; // established
        pub const DATE_WORD_WORD: u32 = 214; // established
        pub const NVRAM_RESET: u32 = 215; // established
        pub const NVRAM_READ_NUMBER_NUMBER: u32 = 216; // established
        pub const NVRAM_WRITE_NUMBER_NUMBER: u32 = 217; // established
        pub const NVRAM_SCRATCH: u32 = 218; // established
        pub const NVRAM_SCRATCH_NUMBER: u32 = 219; // established
        pub const L2_COMMAND: u32 = 220; // established
        pub const SECURITY_VERIFY_3400: u32 = 221; // established
        pub const SECURITY_VERIFY_3800: u32 = 222; // established
        pub const SLEEP_NUMBER_NUMBER_COMMAND: u32 = 223; // established
        pub const FLASH_DEFAULT_RESET: u32 = 224; // established
        pub const FLASH_DEFAULT_A: u32 = 225; // established
        pub const FLASH_DEFAULT_B: u32 = 226; // established
        pub const FLASH_DEFAULT_CURRENT: u32 = 227; // established
        pub const FLASH_DEFAULT_OLD: u32 = 228; // established
        pub const FLASH_DEFAULT_NEW: u32 = 229; // established
        pub const FLASH_STATUS: u32 = 230; // established
        pub const FLASH_STATUS_V: u32 = 231; // established
        pub const REBOOT_L1: u32 = 232; // established
        pub const REBOOT_L1_A: u32 = 233; // established
        pub const REBOOT_L1_B: u32 = 234; // established
        pub const REBOOT_L1_CURRENT: u32 = 235; // established
        pub const REBOOT_L1_OLD: u32 = 236; // established
        pub const REBOOT_L1_NEW: u32 = 237; // established
        pub const REBOOT_L1_OTHER: u32 = 238; // established
        pub const REBOOT_L1_ADDR_NUMBER: u32 = 239; // established
        pub const REBOOT_L1_RAW_NUMBER: u32 = 240; // established
        pub const VERSION: u32 = 241; // established
        pub const PBAY: u32 = 242; // established
        pub const PBAY_VERSION: u32 = 243; // established
        pub const PBAY_DPS: u32 = 244; // established
        pub const PBAY_DPS_WORD: u32 = 245; // established
        pub const PBAY_DCPORT: u32 = 246; // established
        pub const PBAY_DCPORT_WORD: u32 = 247; // established
        pub const PBAY_FRU: u32 = 248; // established
        pub const PBAY_FRU_WORD: u32 = 249; // established
        pub const PBAY_RESET: u32 = 250; // established
        pub const PBAY_ENV: u32 = 251; // established
        pub const PBAY_ENV_ON: u32 = 252; // established
        pub const PBAY_ENV_OFF: u32 = 253; // established
        pub const PBAY_INIT: u32 = 254; // established
        pub const PBAY_SERIAL: u32 = 255; // established
        pub const TEST_I2C: u32 = 256; // established
        pub const TEST_I2C_NUMBER: u32 = 257; // established
        pub const TEST_IOEXP_SET_NUMBER_WORD: u32 = 258; // established
        pub const TEST_IOEXP_GET_NUMBER: u32 = 259; // established
        pub const TEST_IOEXP_GET_ALL: u32 = 260; // established
        pub const TEST_INTR: u32 = 261; // established
        pub const TEST_DISPLAY: u32 = 262; // established
        pub const TEST_EXCEPTION_MEM: u32 = 263; // established
        pub const TEST_EXCEPTION_STACK: u32 = 264; // established
        pub const TEST_EXCEPTION_WDOG: u32 = 265; // established
        pub const TEST_EXCEPTION_FATAL: u32 = 266; // established
        pub const SCAN_RESET: u32 = 267; // established
        pub const SCAN_RESET_HARD: u32 = 268; // established
        pub const SCAN_RESET_SOFT: u32 = 269; // established
        pub const SCAN_RESET_BOTH: u32 = 270; // established
        pub const SCAN_SEL_NUMBER_NUMBER: u32 = 271; // established
        pub const SCAN_SEL_NUMBER_NUMBER_NUMBER: u32 = 272; // established
        pub const SCAN_SEL_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 273; // established
        pub const SCAN_IDS_NUMBER: u32 = 274; // established
        pub const SCAN_SET_TRST_0: u32 = 275; // established
        pub const SCAN_SET_TRST_1: u32 = 276; // established
        pub const SCAN_SET_TCK_0: u32 = 277; // established
        pub const SCAN_SET_TCK_1: u32 = 278; // established
        pub const SCAN_SET_TMS_0: u32 = 279; // established
        pub const SCAN_SET_TMS_1: u32 = 280; // established
        pub const SCAN_SET_TDI_0: u32 = 281; // established
        pub const SCAN_SET_TDI_1: u32 = 282; // established
        pub const SCAN_SET_PSI_0: u32 = 283; // established
        pub const SCAN_SET_PSI_1: u32 = 284; // established
        pub const SCAN_GET: u32 = 285; // established
        pub const SCAN_DEBUG_NUMBER: u32 = 286; // established
        pub const SCAN_DEBUG: u32 = 287; // established
        pub const SCAN_IMP_SET_DLLBYP_NUMBER: u32 = 288; // established
        pub const SCAN_IMP_SET_NUMBER: u32 = 289; // established
        pub const SCAN_IMP_UNLOCK: u32 = 290; // established
        pub const SCAN_TEST_NUMBER: u32 = 291; // established
        pub const FRU: u32 = 292; // established
        pub const FRU_COMMAND: u32 = 293; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            AllCommand,
            RackSlotCommand,
            Autopower,
            AutopowerOn,
            AutopowerOff,
            AutopowerAbort,
            SysLink,
            SysLinkNone,
            SysLinkPpp,
            SysLinkEnable,
            SysLinkReset,
            SysLinkPackets,
            Brick,
            BrickTypeWord,
            BrickRackDecimal,
            BrickSlotDecimal,
            BrickRackslotDecimalDecimal,
            BrickRackslotResetNumberNumber,
            BrickPartitionNone,
            BrickPartitionNumber,
            Partdb,
            PartdbEnable,
            PartdbDisable,
            Cpu,
            CpuE,
            CpuD,
            CpuWordE,
            CpuWordD,
            NiaCommand,
            NibCommand,
            IiaCommand,
            IibCommand,
            IicCommand,
            IidCommand,
            Config,
            ConfigV,
            ConfigReset,
            ConfigRing,
            Debug,
            DebugNumber,
            Display,
            DisplayNumberCommand,
            DisplayPowerOn,
            DisplayAttentionOn,
            DisplayFailureOn,
            DisplayPowerOff,
            DisplayAttentionOff,
            DisplayFailureOff,
            Button,
            ButtonAllEnable,
            ButtonAllDisable,
            ButtonPwrEnable,
            ButtonPwrDisable,
            ButtonRstEnable,
            ButtonRstDisable,
            ButtonNmiEnable,
            ButtonNmiDisable,
            LetTheCarnageBegin,
            SlippedMyMind,
            SuperSizedPlease,
            Env,
            EnvCheck,
            EnvOn,
            EnvOff,
            EnvPowerOn,
            EnvPowerOff,
            EnvPowerMonitor,
            EnvTempOn,
            EnvTempOff,
            EnvAltitudeHigh,
            EnvAltitudeLow,
            EnvReset,
            EnvTestWarning,
            EnvTestFault,
            EnvTestTmpadv,
            EnvTestTmpcrit,
            EnvTestNumberNumberNumberNumberNumber,
            EnvTestPwrdn,
            EnvMessages,
            EnvMessagesFaultsonly,
            EnvMessagesAll,
            Fan,
            FanOnNumber,
            FanOn,
            FanSpeed,
            FanSpeedNumberNumber,
            FanSpeedNumber,
            FanSpeedLowNumber,
            FanSpeedLow,
            FanAutoOn,
            Help,
            HelpWord,
            HelpListHidden,
            HelpWordHidden,
            History,
            L1dbg,
            L1dbgSysLinkNumber,
            L1dbgSysLinkNumberNumber,
            L1dbgIrtrNumber,
            L1dbgEnvNumber,
            L1dbgEnvOff,
            L1dbgPortOn,
            L1dbgI2cOn,
            L1dbgMarginOn,
            L1dbgConsoleOn,
            L1dbgMempanicOn,
            L1dbgPppdumpOn,
            L1dbgPppdelayNumber,
            L1dbgPromreqOn,
            L1dbgQsuspNumberNumber,
            L1dbgPrintfOn,
            L1dbgFlashignoreNumberNumber,
            L1dbgMiscNumber,
            L1dbgGalNumber,
            Link,
            LinkVerbose,
            Log,
            LogClear,
            LogInsertCommand,
            Ioport,
            Ioport400,
            IoportA400,
            IoportB400,
            IoportForce400,
            IoportForceAlt1,
            IoportForceAlt2,
            IoportClksrcA,
            IoportClksrcB,
            IstatMemory,
            IstatQueues,
            IstatTasks,
            IstatPmalloc,
            IstatDumpWord,
            B2b,
            L1,
            Leds,
            Margin,
            MarginDefault,
            MarginGbrickDefault,
            MarginNumberDefault,
            MarginNumberNumber,
            MarginDimmVWord,
            Network,
            NetworkUsb,
            NetworkAutodetectOn,
            Pimm,
            PimmAuto,
            PimmClksrcExt,
            PimmClksrcInt,
            PimmClkmodeAsync,
            PimmClkmodeSync,
            Port,
            Power,
            PowerCheck,
            PowerVrm,
            PowerUp,
            PowerUpHold,
            PowerDown,
            Reset,
            ResetHold,
            Nmi,
            NmiNumber,
            NmiRackSlotWord,
            Softreset,
            SoftresetNumber,
            Select,
            SelectNumberNumber,
            SelectSubchannelConsole,
            SelectSubchannelNumber,
            SelectSubchannelA,
            SelectSubchannelWord,
            SelectEnable,
            SelectDisable,
            SelectFilterOn,
            SelectFilterOff,
            Serial,
            SerialVerify,
            SerialAll,
            SerialAllV,
            SerialDimm,
            SerialDimmV,
            SerialDimmCfg,
            SerialClear,
            SerialSetWord,
            SerialWordWordWordWord,
            SerialSecurityOffWordWordWordWord,
            SerialSecurityOn,
            SerialBrickDisable,
            SerialBrickDisableConfirm,
            Sysstate,
            Eeprom,
            EepromNumberNumberNumber,
            EepromDimmUnsign,
            Uart,
            UartSysLinkReset,
            Usb,
            Router,
            RouterMeta,
            RouterRepeater,
            RouterOrdinary,
            Router6Port,
            Router6PortConfirm,
            Router8Port,
            Router8PortWordWordWordWord,
            RouterServiceWordWordWordWord,
            RouterServiceOff,
            RouterSpare,
            ServiceWordWordWordWord,
            ServiceOff,
            DateTz,
            DateTzWord,
            Date,
            DateRaw,
            DateWord,
            DateWordWord,
            NvramReset,
            NvramReadNumberNumber,
            NvramWriteNumberNumber,
            NvramScratch,
            NvramScratchNumber,
            L2Command,
            SecurityVerify3400,
            SecurityVerify3800,
            SleepNumberNumberCommand,
            FlashDefaultReset,
            FlashDefaultA,
            FlashDefaultB,
            FlashDefaultCurrent,
            FlashDefaultOld,
            FlashDefaultNew,
            FlashStatus,
            FlashStatusV,
            RebootL1,
            RebootL1A,
            RebootL1B,
            RebootL1Current,
            RebootL1Old,
            RebootL1New,
            RebootL1Other,
            RebootL1AddrNumber,
            RebootL1RawNumber,
            Version,
            Pbay,
            PbayVersion,
            PbayDps,
            PbayDpsWord,
            PbayDcport,
            PbayDcportWord,
            PbayFru,
            PbayFruWord,
            PbayReset,
            PbayEnv,
            PbayEnvOn,
            PbayEnvOff,
            PbayInit,
            PbaySerial,
            TestI2c,
            TestI2cNumber,
            TestIoexpSetNumberWord,
            TestIoexpGetNumber,
            TestIoexpGetAll,
            TestIntr,
            TestDisplay,
            TestExceptionMem,
            TestExceptionStack,
            TestExceptionWdog,
            TestExceptionFatal,
            ScanReset,
            ScanResetHard,
            ScanResetSoft,
            ScanResetBoth,
            ScanSelNumberNumber,
            ScanSelNumberNumberNumber,
            ScanSelNumberNumberNumberNumber,
            ScanIdsNumber,
            ScanSetTrst0,
            ScanSetTrst1,
            ScanSetTck0,
            ScanSetTck1,
            ScanSetTms0,
            ScanSetTms1,
            ScanSetTdi0,
            ScanSetTdi1,
            ScanSetPsi0,
            ScanSetPsi1,
            ScanGet,
            ScanDebugNumber,
            ScanDebug,
            ScanImpSetDllbypNumber,
            ScanImpSetNumber,
            ScanImpUnlock,
            ScanTestNumber,
            Fru,
            FruCommand,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::AllCommand,
                    1 => Self::RackSlotCommand,
                    2 => Self::Autopower,
                    3 => Self::AutopowerOn,
                    4 => Self::AutopowerOff,
                    5 => Self::AutopowerAbort,
                    6 => Self::SysLink,
                    7 => Self::SysLinkNone,
                    8 => Self::SysLinkPpp,
                    9 => Self::SysLinkEnable,
                    10 => Self::SysLinkReset,
                    11 => Self::SysLinkPackets,
                    12 => Self::Brick,
                    13 => Self::BrickTypeWord,
                    14 => Self::BrickRackDecimal,
                    15 => Self::BrickSlotDecimal,
                    16 => Self::BrickRackslotDecimalDecimal,
                    17 => Self::BrickRackslotResetNumberNumber,
                    18 => Self::BrickPartitionNone,
                    19 => Self::BrickPartitionNumber,
                    20 => Self::Partdb,
                    21 => Self::PartdbEnable,
                    22 => Self::PartdbDisable,
                    23 => Self::Cpu,
                    24 => Self::CpuE,
                    25 => Self::CpuD,
                    26 => Self::CpuWordE,
                    27 => Self::CpuWordD,
                    28 => Self::NiaCommand,
                    29 => Self::NibCommand,
                    30 => Self::IiaCommand,
                    31 => Self::IibCommand,
                    32 => Self::IicCommand,
                    33 => Self::IidCommand,
                    34 => Self::Config,
                    35 => Self::ConfigV,
                    36 => Self::ConfigReset,
                    37 => Self::ConfigRing,
                    38 => Self::Debug,
                    39 => Self::DebugNumber,
                    40 => Self::Display,
                    41 => Self::DisplayNumberCommand,
                    42 => Self::DisplayPowerOn,
                    43 => Self::DisplayAttentionOn,
                    44 => Self::DisplayFailureOn,
                    45 => Self::DisplayPowerOff,
                    46 => Self::DisplayAttentionOff,
                    47 => Self::DisplayFailureOff,
                    48 => Self::Button,
                    49 => Self::ButtonAllEnable,
                    50 => Self::ButtonAllDisable,
                    51 => Self::ButtonPwrEnable,
                    52 => Self::ButtonPwrDisable,
                    53 => Self::ButtonRstEnable,
                    54 => Self::ButtonRstDisable,
                    55 => Self::ButtonNmiEnable,
                    56 => Self::ButtonNmiDisable,
                    57 => Self::LetTheCarnageBegin,
                    58 => Self::SlippedMyMind,
                    59 => Self::SuperSizedPlease,
                    60 => Self::Env,
                    61 => Self::EnvCheck,
                    62 => Self::EnvOn,
                    63 => Self::EnvOff,
                    64 => Self::EnvPowerOn,
                    65 => Self::EnvPowerOff,
                    66 => Self::EnvPowerMonitor,
                    67 => Self::EnvTempOn,
                    68 => Self::EnvTempOff,
                    69 => Self::EnvAltitudeHigh,
                    70 => Self::EnvAltitudeLow,
                    71 => Self::EnvReset,
                    72 => Self::EnvTestWarning,
                    73 => Self::EnvTestFault,
                    74 => Self::EnvTestTmpadv,
                    75 => Self::EnvTestTmpcrit,
                    76 => Self::EnvTestNumberNumberNumberNumberNumber,
                    77 => Self::EnvTestPwrdn,
                    78 => Self::EnvMessages,
                    79 => Self::EnvMessagesFaultsonly,
                    80 => Self::EnvMessagesAll,
                    81 => Self::Fan,
                    82 => Self::FanOnNumber,
                    83 => Self::FanOn,
                    84 => Self::FanSpeed,
                    85 => Self::FanSpeedNumberNumber,
                    86 => Self::FanSpeedNumber,
                    87 => Self::FanSpeedLowNumber,
                    88 => Self::FanSpeedLow,
                    89 => Self::FanAutoOn,
                    90 => Self::Help,
                    91 => Self::HelpWord,
                    92 => Self::HelpListHidden,
                    93 => Self::HelpWordHidden,
                    94 => Self::History,
                    95 => Self::L1dbg,
                    96 => Self::L1dbgSysLinkNumber,
                    97 => Self::L1dbgSysLinkNumberNumber,
                    98 => Self::L1dbgIrtrNumber,
                    99 => Self::L1dbgEnvNumber,
                    100 => Self::L1dbgEnvOff,
                    101 => Self::L1dbgPortOn,
                    102 => Self::L1dbgI2cOn,
                    103 => Self::L1dbgMarginOn,
                    104 => Self::L1dbgConsoleOn,
                    105 => Self::L1dbgMempanicOn,
                    106 => Self::L1dbgPppdumpOn,
                    107 => Self::L1dbgPppdelayNumber,
                    108 => Self::L1dbgPromreqOn,
                    109 => Self::L1dbgQsuspNumberNumber,
                    110 => Self::L1dbgPrintfOn,
                    111 => Self::L1dbgFlashignoreNumberNumber,
                    112 => Self::L1dbgMiscNumber,
                    113 => Self::L1dbgGalNumber,
                    114 => Self::Link,
                    115 => Self::LinkVerbose,
                    116 => Self::Log,
                    117 => Self::LogClear,
                    118 => Self::LogInsertCommand,
                    119 => Self::Ioport,
                    120 => Self::Ioport400,
                    121 => Self::IoportA400,
                    122 => Self::IoportB400,
                    123 => Self::IoportForce400,
                    124 => Self::IoportForceAlt1,
                    125 => Self::IoportForceAlt2,
                    126 => Self::IoportClksrcA,
                    127 => Self::IoportClksrcB,
                    128 => Self::IstatMemory,
                    129 => Self::IstatQueues,
                    130 => Self::IstatTasks,
                    131 => Self::IstatPmalloc,
                    132 => Self::IstatDumpWord,
                    133 => Self::B2b,
                    134 => Self::L1,
                    135 => Self::Leds,
                    136 => Self::Margin,
                    137 => Self::MarginDefault,
                    138 => Self::MarginGbrickDefault,
                    139 => Self::MarginNumberDefault,
                    140 => Self::MarginNumberNumber,
                    141 => Self::MarginDimmVWord,
                    142 => Self::Network,
                    143 => Self::NetworkUsb,
                    144 => Self::NetworkAutodetectOn,
                    145 => Self::Pimm,
                    146 => Self::PimmAuto,
                    147 => Self::PimmClksrcExt,
                    148 => Self::PimmClksrcInt,
                    149 => Self::PimmClkmodeAsync,
                    150 => Self::PimmClkmodeSync,
                    151 => Self::Port,
                    152 => Self::Power,
                    153 => Self::PowerCheck,
                    154 => Self::PowerVrm,
                    155 => Self::PowerUp,
                    156 => Self::PowerUpHold,
                    157 => Self::PowerDown,
                    158 => Self::Reset,
                    159 => Self::ResetHold,
                    160 => Self::Nmi,
                    161 => Self::NmiNumber,
                    162 => Self::NmiRackSlotWord,
                    163 => Self::Softreset,
                    164 => Self::SoftresetNumber,
                    165 => Self::Select,
                    166 => Self::SelectNumberNumber,
                    167 => Self::SelectSubchannelConsole,
                    168 => Self::SelectSubchannelNumber,
                    169 => Self::SelectSubchannelA,
                    170 => Self::SelectSubchannelWord,
                    171 => Self::SelectEnable,
                    172 => Self::SelectDisable,
                    173 => Self::SelectFilterOn,
                    174 => Self::SelectFilterOff,
                    175 => Self::Serial,
                    176 => Self::SerialVerify,
                    177 => Self::SerialAll,
                    178 => Self::SerialAllV,
                    179 => Self::SerialDimm,
                    180 => Self::SerialDimmV,
                    181 => Self::SerialDimmCfg,
                    182 => Self::SerialClear,
                    183 => Self::SerialSetWord,
                    184 => Self::SerialWordWordWordWord,
                    185 => Self::SerialSecurityOffWordWordWordWord,
                    186 => Self::SerialSecurityOn,
                    187 => Self::SerialBrickDisable,
                    188 => Self::SerialBrickDisableConfirm,
                    189 => Self::Sysstate,
                    190 => Self::Eeprom,
                    191 => Self::EepromNumberNumberNumber,
                    192 => Self::EepromDimmUnsign,
                    193 => Self::Uart,
                    194 => Self::UartSysLinkReset,
                    195 => Self::Usb,
                    196 => Self::Router,
                    197 => Self::RouterMeta,
                    198 => Self::RouterRepeater,
                    199 => Self::RouterOrdinary,
                    200 => Self::Router6Port,
                    201 => Self::Router6PortConfirm,
                    202 => Self::Router8Port,
                    203 => Self::Router8PortWordWordWordWord,
                    204 => Self::RouterServiceWordWordWordWord,
                    205 => Self::RouterServiceOff,
                    206 => Self::RouterSpare,
                    207 => Self::ServiceWordWordWordWord,
                    208 => Self::ServiceOff,
                    209 => Self::DateTz,
                    210 => Self::DateTzWord,
                    211 => Self::Date,
                    212 => Self::DateRaw,
                    213 => Self::DateWord,
                    214 => Self::DateWordWord,
                    215 => Self::NvramReset,
                    216 => Self::NvramReadNumberNumber,
                    217 => Self::NvramWriteNumberNumber,
                    218 => Self::NvramScratch,
                    219 => Self::NvramScratchNumber,
                    220 => Self::L2Command,
                    221 => Self::SecurityVerify3400,
                    222 => Self::SecurityVerify3800,
                    223 => Self::SleepNumberNumberCommand,
                    224 => Self::FlashDefaultReset,
                    225 => Self::FlashDefaultA,
                    226 => Self::FlashDefaultB,
                    227 => Self::FlashDefaultCurrent,
                    228 => Self::FlashDefaultOld,
                    229 => Self::FlashDefaultNew,
                    230 => Self::FlashStatus,
                    231 => Self::FlashStatusV,
                    232 => Self::RebootL1,
                    233 => Self::RebootL1A,
                    234 => Self::RebootL1B,
                    235 => Self::RebootL1Current,
                    236 => Self::RebootL1Old,
                    237 => Self::RebootL1New,
                    238 => Self::RebootL1Other,
                    239 => Self::RebootL1AddrNumber,
                    240 => Self::RebootL1RawNumber,
                    241 => Self::Version,
                    242 => Self::Pbay,
                    243 => Self::PbayVersion,
                    244 => Self::PbayDps,
                    245 => Self::PbayDpsWord,
                    246 => Self::PbayDcport,
                    247 => Self::PbayDcportWord,
                    248 => Self::PbayFru,
                    249 => Self::PbayFruWord,
                    250 => Self::PbayReset,
                    251 => Self::PbayEnv,
                    252 => Self::PbayEnvOn,
                    253 => Self::PbayEnvOff,
                    254 => Self::PbayInit,
                    255 => Self::PbaySerial,
                    256 => Self::TestI2c,
                    257 => Self::TestI2cNumber,
                    258 => Self::TestIoexpSetNumberWord,
                    259 => Self::TestIoexpGetNumber,
                    260 => Self::TestIoexpGetAll,
                    261 => Self::TestIntr,
                    262 => Self::TestDisplay,
                    263 => Self::TestExceptionMem,
                    264 => Self::TestExceptionStack,
                    265 => Self::TestExceptionWdog,
                    266 => Self::TestExceptionFatal,
                    267 => Self::ScanReset,
                    268 => Self::ScanResetHard,
                    269 => Self::ScanResetSoft,
                    270 => Self::ScanResetBoth,
                    271 => Self::ScanSelNumberNumber,
                    272 => Self::ScanSelNumberNumberNumber,
                    273 => Self::ScanSelNumberNumberNumberNumber,
                    274 => Self::ScanIdsNumber,
                    275 => Self::ScanSetTrst0,
                    276 => Self::ScanSetTrst1,
                    277 => Self::ScanSetTck0,
                    278 => Self::ScanSetTck1,
                    279 => Self::ScanSetTms0,
                    280 => Self::ScanSetTms1,
                    281 => Self::ScanSetTdi0,
                    282 => Self::ScanSetTdi1,
                    283 => Self::ScanSetPsi0,
                    284 => Self::ScanSetPsi1,
                    285 => Self::ScanGet,
                    286 => Self::ScanDebugNumber,
                    287 => Self::ScanDebug,
                    288 => Self::ScanImpSetDllbypNumber,
                    289 => Self::ScanImpSetNumber,
                    290 => Self::ScanImpUnlock,
                    291 => Self::ScanTestNumber,
                    292 => Self::Fru,
                    293 => Self::FruCommand,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::AllCommand => 0,
                    Self::RackSlotCommand => 1,
                    Self::Autopower => 2,
                    Self::AutopowerOn => 3,
                    Self::AutopowerOff => 4,
                    Self::AutopowerAbort => 5,
                    Self::SysLink => 6,
                    Self::SysLinkNone => 7,
                    Self::SysLinkPpp => 8,
                    Self::SysLinkEnable => 9,
                    Self::SysLinkReset => 10,
                    Self::SysLinkPackets => 11,
                    Self::Brick => 12,
                    Self::BrickTypeWord => 13,
                    Self::BrickRackDecimal => 14,
                    Self::BrickSlotDecimal => 15,
                    Self::BrickRackslotDecimalDecimal => 16,
                    Self::BrickRackslotResetNumberNumber => 17,
                    Self::BrickPartitionNone => 18,
                    Self::BrickPartitionNumber => 19,
                    Self::Partdb => 20,
                    Self::PartdbEnable => 21,
                    Self::PartdbDisable => 22,
                    Self::Cpu => 23,
                    Self::CpuE => 24,
                    Self::CpuD => 25,
                    Self::CpuWordE => 26,
                    Self::CpuWordD => 27,
                    Self::NiaCommand => 28,
                    Self::NibCommand => 29,
                    Self::IiaCommand => 30,
                    Self::IibCommand => 31,
                    Self::IicCommand => 32,
                    Self::IidCommand => 33,
                    Self::Config => 34,
                    Self::ConfigV => 35,
                    Self::ConfigReset => 36,
                    Self::ConfigRing => 37,
                    Self::Debug => 38,
                    Self::DebugNumber => 39,
                    Self::Display => 40,
                    Self::DisplayNumberCommand => 41,
                    Self::DisplayPowerOn => 42,
                    Self::DisplayAttentionOn => 43,
                    Self::DisplayFailureOn => 44,
                    Self::DisplayPowerOff => 45,
                    Self::DisplayAttentionOff => 46,
                    Self::DisplayFailureOff => 47,
                    Self::Button => 48,
                    Self::ButtonAllEnable => 49,
                    Self::ButtonAllDisable => 50,
                    Self::ButtonPwrEnable => 51,
                    Self::ButtonPwrDisable => 52,
                    Self::ButtonRstEnable => 53,
                    Self::ButtonRstDisable => 54,
                    Self::ButtonNmiEnable => 55,
                    Self::ButtonNmiDisable => 56,
                    Self::LetTheCarnageBegin => 57,
                    Self::SlippedMyMind => 58,
                    Self::SuperSizedPlease => 59,
                    Self::Env => 60,
                    Self::EnvCheck => 61,
                    Self::EnvOn => 62,
                    Self::EnvOff => 63,
                    Self::EnvPowerOn => 64,
                    Self::EnvPowerOff => 65,
                    Self::EnvPowerMonitor => 66,
                    Self::EnvTempOn => 67,
                    Self::EnvTempOff => 68,
                    Self::EnvAltitudeHigh => 69,
                    Self::EnvAltitudeLow => 70,
                    Self::EnvReset => 71,
                    Self::EnvTestWarning => 72,
                    Self::EnvTestFault => 73,
                    Self::EnvTestTmpadv => 74,
                    Self::EnvTestTmpcrit => 75,
                    Self::EnvTestNumberNumberNumberNumberNumber => 76,
                    Self::EnvTestPwrdn => 77,
                    Self::EnvMessages => 78,
                    Self::EnvMessagesFaultsonly => 79,
                    Self::EnvMessagesAll => 80,
                    Self::Fan => 81,
                    Self::FanOnNumber => 82,
                    Self::FanOn => 83,
                    Self::FanSpeed => 84,
                    Self::FanSpeedNumberNumber => 85,
                    Self::FanSpeedNumber => 86,
                    Self::FanSpeedLowNumber => 87,
                    Self::FanSpeedLow => 88,
                    Self::FanAutoOn => 89,
                    Self::Help => 90,
                    Self::HelpWord => 91,
                    Self::HelpListHidden => 92,
                    Self::HelpWordHidden => 93,
                    Self::History => 94,
                    Self::L1dbg => 95,
                    Self::L1dbgSysLinkNumber => 96,
                    Self::L1dbgSysLinkNumberNumber => 97,
                    Self::L1dbgIrtrNumber => 98,
                    Self::L1dbgEnvNumber => 99,
                    Self::L1dbgEnvOff => 100,
                    Self::L1dbgPortOn => 101,
                    Self::L1dbgI2cOn => 102,
                    Self::L1dbgMarginOn => 103,
                    Self::L1dbgConsoleOn => 104,
                    Self::L1dbgMempanicOn => 105,
                    Self::L1dbgPppdumpOn => 106,
                    Self::L1dbgPppdelayNumber => 107,
                    Self::L1dbgPromreqOn => 108,
                    Self::L1dbgQsuspNumberNumber => 109,
                    Self::L1dbgPrintfOn => 110,
                    Self::L1dbgFlashignoreNumberNumber => 111,
                    Self::L1dbgMiscNumber => 112,
                    Self::L1dbgGalNumber => 113,
                    Self::Link => 114,
                    Self::LinkVerbose => 115,
                    Self::Log => 116,
                    Self::LogClear => 117,
                    Self::LogInsertCommand => 118,
                    Self::Ioport => 119,
                    Self::Ioport400 => 120,
                    Self::IoportA400 => 121,
                    Self::IoportB400 => 122,
                    Self::IoportForce400 => 123,
                    Self::IoportForceAlt1 => 124,
                    Self::IoportForceAlt2 => 125,
                    Self::IoportClksrcA => 126,
                    Self::IoportClksrcB => 127,
                    Self::IstatMemory => 128,
                    Self::IstatQueues => 129,
                    Self::IstatTasks => 130,
                    Self::IstatPmalloc => 131,
                    Self::IstatDumpWord => 132,
                    Self::B2b => 133,
                    Self::L1 => 134,
                    Self::Leds => 135,
                    Self::Margin => 136,
                    Self::MarginDefault => 137,
                    Self::MarginGbrickDefault => 138,
                    Self::MarginNumberDefault => 139,
                    Self::MarginNumberNumber => 140,
                    Self::MarginDimmVWord => 141,
                    Self::Network => 142,
                    Self::NetworkUsb => 143,
                    Self::NetworkAutodetectOn => 144,
                    Self::Pimm => 145,
                    Self::PimmAuto => 146,
                    Self::PimmClksrcExt => 147,
                    Self::PimmClksrcInt => 148,
                    Self::PimmClkmodeAsync => 149,
                    Self::PimmClkmodeSync => 150,
                    Self::Port => 151,
                    Self::Power => 152,
                    Self::PowerCheck => 153,
                    Self::PowerVrm => 154,
                    Self::PowerUp => 155,
                    Self::PowerUpHold => 156,
                    Self::PowerDown => 157,
                    Self::Reset => 158,
                    Self::ResetHold => 159,
                    Self::Nmi => 160,
                    Self::NmiNumber => 161,
                    Self::NmiRackSlotWord => 162,
                    Self::Softreset => 163,
                    Self::SoftresetNumber => 164,
                    Self::Select => 165,
                    Self::SelectNumberNumber => 166,
                    Self::SelectSubchannelConsole => 167,
                    Self::SelectSubchannelNumber => 168,
                    Self::SelectSubchannelA => 169,
                    Self::SelectSubchannelWord => 170,
                    Self::SelectEnable => 171,
                    Self::SelectDisable => 172,
                    Self::SelectFilterOn => 173,
                    Self::SelectFilterOff => 174,
                    Self::Serial => 175,
                    Self::SerialVerify => 176,
                    Self::SerialAll => 177,
                    Self::SerialAllV => 178,
                    Self::SerialDimm => 179,
                    Self::SerialDimmV => 180,
                    Self::SerialDimmCfg => 181,
                    Self::SerialClear => 182,
                    Self::SerialSetWord => 183,
                    Self::SerialWordWordWordWord => 184,
                    Self::SerialSecurityOffWordWordWordWord => 185,
                    Self::SerialSecurityOn => 186,
                    Self::SerialBrickDisable => 187,
                    Self::SerialBrickDisableConfirm => 188,
                    Self::Sysstate => 189,
                    Self::Eeprom => 190,
                    Self::EepromNumberNumberNumber => 191,
                    Self::EepromDimmUnsign => 192,
                    Self::Uart => 193,
                    Self::UartSysLinkReset => 194,
                    Self::Usb => 195,
                    Self::Router => 196,
                    Self::RouterMeta => 197,
                    Self::RouterRepeater => 198,
                    Self::RouterOrdinary => 199,
                    Self::Router6Port => 200,
                    Self::Router6PortConfirm => 201,
                    Self::Router8Port => 202,
                    Self::Router8PortWordWordWordWord => 203,
                    Self::RouterServiceWordWordWordWord => 204,
                    Self::RouterServiceOff => 205,
                    Self::RouterSpare => 206,
                    Self::ServiceWordWordWordWord => 207,
                    Self::ServiceOff => 208,
                    Self::DateTz => 209,
                    Self::DateTzWord => 210,
                    Self::Date => 211,
                    Self::DateRaw => 212,
                    Self::DateWord => 213,
                    Self::DateWordWord => 214,
                    Self::NvramReset => 215,
                    Self::NvramReadNumberNumber => 216,
                    Self::NvramWriteNumberNumber => 217,
                    Self::NvramScratch => 218,
                    Self::NvramScratchNumber => 219,
                    Self::L2Command => 220,
                    Self::SecurityVerify3400 => 221,
                    Self::SecurityVerify3800 => 222,
                    Self::SleepNumberNumberCommand => 223,
                    Self::FlashDefaultReset => 224,
                    Self::FlashDefaultA => 225,
                    Self::FlashDefaultB => 226,
                    Self::FlashDefaultCurrent => 227,
                    Self::FlashDefaultOld => 228,
                    Self::FlashDefaultNew => 229,
                    Self::FlashStatus => 230,
                    Self::FlashStatusV => 231,
                    Self::RebootL1 => 232,
                    Self::RebootL1A => 233,
                    Self::RebootL1B => 234,
                    Self::RebootL1Current => 235,
                    Self::RebootL1Old => 236,
                    Self::RebootL1New => 237,
                    Self::RebootL1Other => 238,
                    Self::RebootL1AddrNumber => 239,
                    Self::RebootL1RawNumber => 240,
                    Self::Version => 241,
                    Self::Pbay => 242,
                    Self::PbayVersion => 243,
                    Self::PbayDps => 244,
                    Self::PbayDpsWord => 245,
                    Self::PbayDcport => 246,
                    Self::PbayDcportWord => 247,
                    Self::PbayFru => 248,
                    Self::PbayFruWord => 249,
                    Self::PbayReset => 250,
                    Self::PbayEnv => 251,
                    Self::PbayEnvOn => 252,
                    Self::PbayEnvOff => 253,
                    Self::PbayInit => 254,
                    Self::PbaySerial => 255,
                    Self::TestI2c => 256,
                    Self::TestI2cNumber => 257,
                    Self::TestIoexpSetNumberWord => 258,
                    Self::TestIoexpGetNumber => 259,
                    Self::TestIoexpGetAll => 260,
                    Self::TestIntr => 261,
                    Self::TestDisplay => 262,
                    Self::TestExceptionMem => 263,
                    Self::TestExceptionStack => 264,
                    Self::TestExceptionWdog => 265,
                    Self::TestExceptionFatal => 266,
                    Self::ScanReset => 267,
                    Self::ScanResetHard => 268,
                    Self::ScanResetSoft => 269,
                    Self::ScanResetBoth => 270,
                    Self::ScanSelNumberNumber => 271,
                    Self::ScanSelNumberNumberNumber => 272,
                    Self::ScanSelNumberNumberNumberNumber => 273,
                    Self::ScanIdsNumber => 274,
                    Self::ScanSetTrst0 => 275,
                    Self::ScanSetTrst1 => 276,
                    Self::ScanSetTck0 => 277,
                    Self::ScanSetTck1 => 278,
                    Self::ScanSetTms0 => 279,
                    Self::ScanSetTms1 => 280,
                    Self::ScanSetTdi0 => 281,
                    Self::ScanSetTdi1 => 282,
                    Self::ScanSetPsi0 => 283,
                    Self::ScanSetPsi1 => 284,
                    Self::ScanGet => 285,
                    Self::ScanDebugNumber => 286,
                    Self::ScanDebug => 287,
                    Self::ScanImpSetDllbypNumber => 288,
                    Self::ScanImpSetNumber => 289,
                    Self::ScanImpUnlock => 290,
                    Self::ScanTestNumber => 291,
                    Self::Fru => 292,
                    Self::FruCommand => 293,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod cmd1 {
    pub const WIDTH: u32 = 32;
    pub mod index { // bits 7:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const PCI: u32 = 0; // established
        pub const PCI_U: u32 = 1; // established
        pub const PCI_D: u32 = 2; // established
        pub const PCI_RST: u32 = 3; // established
        pub const PCI_RST_HOLD: u32 = 4; // established
        pub const PCI_RST_HOLD_NUMBER: u32 = 5; // established
        pub const PCI_RST_HOLD_NUMBER_NUMBER: u32 = 6; // established
        pub const PCI_RST_RELEASE: u32 = 7; // established
        pub const PCI_RST_RELEASE_NUMBER: u32 = 8; // established
        pub const PCI_RST_RELEASE_NUMBER_NUMBER: u32 = 9; // established
        pub const PCI_NUMBER_U: u32 = 10; // established
        pub const PCI_NUMBER_D: u32 = 11; // established
        pub const PCI_NUMBER_RST: u32 = 12; // established
        pub const PCI_NUMBER_NUMBER_U: u32 = 13; // established
        pub const PCI_NUMBER_NUMBER_D: u32 = 14; // established
        pub const PCI_NUMBER_NUMBER_RST: u32 = 15; // established
        pub const PCI_FORCE_ALL_ON: u32 = 16; // established
        pub const PCI_MODE_LIMIT_NUMBER_PCI33: u32 = 17; // established
        pub const PCI_MODE_LIMIT_PCI33: u32 = 18; // established
        pub const PCI_MODE_FORCE_NUMBER_PCI33: u32 = 19; // established
        pub const PCI_MODE_FORCE_PCI33: u32 = 20; // established
        pub const PCI_MODE_DEFAULT_NUMBER: u32 = 21; // established
        pub const PCI_MODE_DEFAULT: u32 = 22; // established
        pub const PCI_MODE_WARNING_DISABLE: u32 = 23; // established
        pub const PCI_MODE_WARNING_ENABLE: u32 = 24; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Pci,
            PciU,
            PciD,
            PciRst,
            PciRstHold,
            PciRstHoldNumber,
            PciRstHoldNumberNumber,
            PciRstRelease,
            PciRstReleaseNumber,
            PciRstReleaseNumberNumber,
            PciNumberU,
            PciNumberD,
            PciNumberRst,
            PciNumberNumberU,
            PciNumberNumberD,
            PciNumberNumberRst,
            PciForceAllOn,
            PciModeLimitNumberPci33,
            PciModeLimitPci33,
            PciModeForceNumberPci33,
            PciModeForcePci33,
            PciModeDefaultNumber,
            PciModeDefault,
            PciModeWarningDisable,
            PciModeWarningEnable,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::Pci,
                    1 => Self::PciU,
                    2 => Self::PciD,
                    3 => Self::PciRst,
                    4 => Self::PciRstHold,
                    5 => Self::PciRstHoldNumber,
                    6 => Self::PciRstHoldNumberNumber,
                    7 => Self::PciRstRelease,
                    8 => Self::PciRstReleaseNumber,
                    9 => Self::PciRstReleaseNumberNumber,
                    10 => Self::PciNumberU,
                    11 => Self::PciNumberD,
                    12 => Self::PciNumberRst,
                    13 => Self::PciNumberNumberU,
                    14 => Self::PciNumberNumberD,
                    15 => Self::PciNumberNumberRst,
                    16 => Self::PciForceAllOn,
                    17 => Self::PciModeLimitNumberPci33,
                    18 => Self::PciModeLimitPci33,
                    19 => Self::PciModeForceNumberPci33,
                    20 => Self::PciModeForcePci33,
                    21 => Self::PciModeDefaultNumber,
                    22 => Self::PciModeDefault,
                    23 => Self::PciModeWarningDisable,
                    24 => Self::PciModeWarningEnable,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Pci => 0,
                    Self::PciU => 1,
                    Self::PciD => 2,
                    Self::PciRst => 3,
                    Self::PciRstHold => 4,
                    Self::PciRstHoldNumber => 5,
                    Self::PciRstHoldNumberNumber => 6,
                    Self::PciRstRelease => 7,
                    Self::PciRstReleaseNumber => 8,
                    Self::PciRstReleaseNumberNumber => 9,
                    Self::PciNumberU => 10,
                    Self::PciNumberD => 11,
                    Self::PciNumberRst => 12,
                    Self::PciNumberNumberU => 13,
                    Self::PciNumberNumberD => 14,
                    Self::PciNumberNumberRst => 15,
                    Self::PciForceAllOn => 16,
                    Self::PciModeLimitNumberPci33 => 17,
                    Self::PciModeLimitPci33 => 18,
                    Self::PciModeForceNumberPci33 => 19,
                    Self::PciModeForcePci33 => 20,
                    Self::PciModeDefaultNumber => 21,
                    Self::PciModeDefault => 22,
                    Self::PciModeWarningDisable => 23,
                    Self::PciModeWarningEnable => 24,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod cmd2 {
    pub const WIDTH: u32 = 32;
    pub mod index { // bits 7:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const CHECK_POWER_ON: u32 = 0; // established
        pub const CHECK_MARGIN_HIGH: u32 = 1; // established
        pub const CHECK_SCRAMBLE_ENABLED: u32 = 2; // established
        pub const CHECK_LINKS: u32 = 3; // established
        pub const CHECK_DEBUG_NUMBER: u32 = 4; // established
        pub const CHECK_SECURITY_ON: u32 = 5; // established
        pub const CHECK_SERIAL_WORD: u32 = 6; // established
        pub const FRU_CAPTURE: u32 = 7; // established
        pub const FRU_CAPTURE_VERBOSE: u32 = 8; // established
        pub const FRU_EXAMINE: u32 = 9; // established
        pub const FRU_DUMP: u32 = 10; // established
        pub const FRU_RESET: u32 = 11; // established
        pub const FRU_DEADLOCK: u32 = 12; // established
        pub const FRU_DEADLOCK_VERBOSE: u32 = 13; // established
        pub const FRU_DEADLOCK_QUIET: u32 = 14; // established
        pub const FRU_EVENTS: u32 = 15; // established
        pub const MEMCFG: u32 = 16; // established
        pub const MEMCFG_1BANK166_E: u32 = 17; // established
        pub const SHCFG: u32 = 18; // established
        pub const SHCFG_FSB_RESET: u32 = 19; // established
        pub const SHCFG_FSB: u32 = 20; // established
        pub const SHCFG_FSB_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 21; // established
        pub const SHCFG_WRT90_RESET: u32 = 22; // established
        pub const SHCFG_WRT90: u32 = 23; // established
        pub const SHCFG_WRT90_NUMBER_NUMBER_NUMBER: u32 = 24; // established
        pub const SHCFG_MCI_RESET: u32 = 25; // established
        pub const SHCFG_MCI: u32 = 26; // established
        pub const SHCFG_MCI_NUMBER_NUMBER_NUMBER: u32 = 27; // established
        pub const SHCFG_MDCLK: u32 = 28; // established
        pub const SHCFG_MDCLK_NUMBER: u32 = 29; // established
        pub const SHCFG_NICLK: u32 = 30; // established
        pub const SHCFG_NICLK_NUMBER: u32 = 31; // established
        pub const SHCFG_OSC: u32 = 32; // established
        pub const SHCFG_OSC_NUMBER: u32 = 33; // established
        pub const SHCFG_GTL_RE: u32 = 34; // established
        pub const SHCFG_GTL_RE_NUMBER: u32 = 35; // established
        pub const SHCFG_JCLKSTOP: u32 = 36; // established
        pub const SHCFG_JCLKSTOP_NUMBER: u32 = 37; // established
        pub const SHCFG_TP: u32 = 38; // established
        pub const SHCFG_TP_NUMBER: u32 = 39; // established
        pub const NICFG_NI0: u32 = 40; // established
        pub const NICFG_NUMBER_NI0: u32 = 41; // established
        pub const NICFG_NI0_NUMBER: u32 = 42; // established
        pub const NICFG_NI0_NUMBER_NUMBER: u32 = 43; // established
        pub const NICFG_NI0_NUMBER_NUMBER_NUMBER: u32 = 44; // established
        pub const NICFG_NI0_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 45; // established
        pub const NICFG_NI0_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 46; // established
        pub const NICFG_NUMBER_NI0_NUMBER: u32 = 47; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER: u32 = 48; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER: u32 = 49; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 50; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 51; // established
        pub const NLCFG: u32 = 52; // established
        pub const NLCFG_WORD: u32 = 53; // established
        pub const NLCFG_WORD_WORD: u32 = 54; // established
        pub const NLCFG_WORD_NL3: u32 = 55; // established
        pub const NLCFG_WORD_WORD_NL3: u32 = 56; // established
        pub const NLCFG_WORD_ENABLE: u32 = 57; // established
        pub const NLCFG_WORD_WORD_ENABLE: u32 = 58; // established
        pub const NLCFG_WORD_DISABLE: u32 = 59; // established
        pub const NLCFG_WORD_WORD_DISABLE: u32 = 60; // established
        pub const RMMR_WORD_NUMBER: u32 = 61; // established
        pub const WMMR_WORD_NUMBER_NUMBER_NUMBER: u32 = 62; // established
        pub const PKTGEN_READ_WORD_NUMBER: u32 = 63; // established
        pub const PKTGEN_WRITE_WORD_NUMBER_NUMBER_NUMBER: u32 = 64; // established
        pub const PKTGEN_DEBUG_NUMBER: u32 = 65; // established
        pub const TUNE_GET: u32 = 66; // established
        pub const PROMICE: u32 = 67; // established
        pub const PROMICE_E: u32 = 68; // established
        pub const PROMICE_D: u32 = 69; // established
        pub const PROMICE_NUMBER_E: u32 = 70; // established
        pub const PROMICE_NUMBER_D: u32 = 71; // established
        pub const PROMVER: u32 = 72; // established
        pub const NODE: u32 = 73; // established
        pub const NODE_E: u32 = 74; // established
        pub const NODE_D: u32 = 75; // established
        pub const NODE_NUMBER_E: u32 = 76; // established
        pub const NODE_NUMBER_D: u32 = 77; // established
        pub const PIC_MODE: u32 = 78; // established
        pub const WAR: u32 = 79; // established
        pub const WAR_E: u32 = 80; // established
        pub const WAR_D: u32 = 81; // established
        pub const SHDBG: u32 = 82; // established
        pub const SHDBG_RETRYLIMIT_NUMBER: u32 = 83; // established
        pub const SHDBG_RDELAY_NUMBER: u32 = 84; // established
        pub const SHDBG_FULLRESET_NUMBER: u32 = 85; // established
        pub const SHDBG_LINKDOWN_NUMBER: u32 = 86; // established
        pub const MBRICK: u32 = 87; // established
        pub const MBRICK_MEMORY: u32 = 88; // established
        pub const MBRICK_MEMORY_CONFIRM: u32 = 89; // established
        pub const MBRICK_CPU_WORD_WORD_WORD_WORD: u32 = 90; // established
        pub const MBRICK_CPU: u32 = 91; // established
        pub const IF_I_ONLY_HAD_A_BRAIN: u32 = 92; // established
        pub const SSI: u32 = 93; // established
        pub const SSI_SERVICE_WORD_WORD_WORD_WORD: u32 = 94; // established
        pub const SSI_SERVICE_OFF: u32 = 95; // established
        pub const SSI_WORD_WORD_WORD_WORD_WORD: u32 = 96; // established
        pub const MAD_WITH_POWER_NUMBER: u32 = 97; // established
        pub const PWM_READ: u32 = 98; // established
        pub const PWM_SET_NUMBER_NUMBER: u32 = 99; // established
        pub const SYS_LINK_SIM_ON: u32 = 100; // established
        pub const SYS_LINK_SIM_ON_NUMBER_NUMBER: u32 = 101; // established
        pub const SYS_LINK_SIM_OFF: u32 = 102; // established
        pub const SYS_LINK_DUMP_INPUT_NUMBER: u32 = 103; // established
        pub const SYS_LINK_DUMP_OUTPUT_NUMBER: u32 = 104; // established
        pub const SYS_LINK_CAPTURE_RESET: u32 = 105; // established
        pub const SYS_LINK_LOG_NUMBER: u32 = 106; // established
        pub const SYS_LINK_LOG_ALL: u32 = 107; // established
        pub const SYS_LINK_LOG_RESET: u32 = 108; // established
        pub const SYS_LINK_LOG_NUMBER_INSERT_COMMAND: u32 = 109; // established
        pub const SYS_LINK_CONFIG: u32 = 110; // established
        pub const SYS_LINK_QSUSPEND: u32 = 111; // established
        pub const SYS_LINK_QSUSPEND_NUMBER: u32 = 112; // established
        pub const ERROR_NUMBER: u32 = 113; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            CheckPowerOn,
            CheckMarginHigh,
            CheckScrambleEnabled,
            CheckLinks,
            CheckDebugNumber,
            CheckSecurityOn,
            CheckSerialWord,
            FruCapture,
            FruCaptureVerbose,
            FruExamine,
            FruDump,
            FruReset,
            FruDeadlock,
            FruDeadlockVerbose,
            FruDeadlockQuiet,
            FruEvents,
            Memcfg,
            Memcfg1bank166E,
            Shcfg,
            ShcfgFsbReset,
            ShcfgFsb,
            ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
            ShcfgWrt90Reset,
            ShcfgWrt90,
            ShcfgWrt90NumberNumberNumber,
            ShcfgMciReset,
            ShcfgMci,
            ShcfgMciNumberNumberNumber,
            ShcfgMdclk,
            ShcfgMdclkNumber,
            ShcfgNiclk,
            ShcfgNiclkNumber,
            ShcfgOsc,
            ShcfgOscNumber,
            ShcfgGtlRe,
            ShcfgGtlReNumber,
            ShcfgJclkstop,
            ShcfgJclkstopNumber,
            ShcfgTp,
            ShcfgTpNumber,
            NicfgNi0,
            NicfgNumberNi0,
            NicfgNi0Number,
            NicfgNi0NumberNumber,
            NicfgNi0NumberNumberNumber,
            NicfgNi0NumberNumberNumberNumber,
            NicfgNi0NumberNumberNumberNumberNumber,
            NicfgNumberNi0Number,
            NicfgNumberNi0NumberNumber,
            NicfgNumberNi0NumberNumberNumber,
            NicfgNumberNi0NumberNumberNumberNumber,
            NicfgNumberNi0NumberNumberNumberNumberNumber,
            Nlcfg,
            NlcfgWord,
            NlcfgWordWord,
            NlcfgWordNl3,
            NlcfgWordWordNl3,
            NlcfgWordEnable,
            NlcfgWordWordEnable,
            NlcfgWordDisable,
            NlcfgWordWordDisable,
            RmmrWordNumber,
            WmmrWordNumberNumberNumber,
            PktgenReadWordNumber,
            PktgenWriteWordNumberNumberNumber,
            PktgenDebugNumber,
            TuneGet,
            Promice,
            PromiceE,
            PromiceD,
            PromiceNumberE,
            PromiceNumberD,
            Promver,
            Node,
            NodeE,
            NodeD,
            NodeNumberE,
            NodeNumberD,
            PicMode,
            War,
            WarE,
            WarD,
            Shdbg,
            ShdbgRetrylimitNumber,
            ShdbgRdelayNumber,
            ShdbgFullresetNumber,
            ShdbgLinkdownNumber,
            Mbrick,
            MbrickMemory,
            MbrickMemoryConfirm,
            MbrickCpuWordWordWordWord,
            MbrickCpu,
            IfIOnlyHadABrain,
            Ssi,
            SsiServiceWordWordWordWord,
            SsiServiceOff,
            SsiWordWordWordWordWord,
            MadWithPowerNumber,
            PwmRead,
            PwmSetNumberNumber,
            SysLinkSimOn,
            SysLinkSimOnNumberNumber,
            SysLinkSimOff,
            SysLinkDumpInputNumber,
            SysLinkDumpOutputNumber,
            SysLinkCaptureReset,
            SysLinkLogNumber,
            SysLinkLogAll,
            SysLinkLogReset,
            SysLinkLogNumberInsertCommand,
            SysLinkConfig,
            SysLinkQsuspend,
            SysLinkQsuspendNumber,
            ErrorNumber,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::CheckPowerOn,
                    1 => Self::CheckMarginHigh,
                    2 => Self::CheckScrambleEnabled,
                    3 => Self::CheckLinks,
                    4 => Self::CheckDebugNumber,
                    5 => Self::CheckSecurityOn,
                    6 => Self::CheckSerialWord,
                    7 => Self::FruCapture,
                    8 => Self::FruCaptureVerbose,
                    9 => Self::FruExamine,
                    10 => Self::FruDump,
                    11 => Self::FruReset,
                    12 => Self::FruDeadlock,
                    13 => Self::FruDeadlockVerbose,
                    14 => Self::FruDeadlockQuiet,
                    15 => Self::FruEvents,
                    16 => Self::Memcfg,
                    17 => Self::Memcfg1bank166E,
                    18 => Self::Shcfg,
                    19 => Self::ShcfgFsbReset,
                    20 => Self::ShcfgFsb,
                    21 => Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
                    22 => Self::ShcfgWrt90Reset,
                    23 => Self::ShcfgWrt90,
                    24 => Self::ShcfgWrt90NumberNumberNumber,
                    25 => Self::ShcfgMciReset,
                    26 => Self::ShcfgMci,
                    27 => Self::ShcfgMciNumberNumberNumber,
                    28 => Self::ShcfgMdclk,
                    29 => Self::ShcfgMdclkNumber,
                    30 => Self::ShcfgNiclk,
                    31 => Self::ShcfgNiclkNumber,
                    32 => Self::ShcfgOsc,
                    33 => Self::ShcfgOscNumber,
                    34 => Self::ShcfgGtlRe,
                    35 => Self::ShcfgGtlReNumber,
                    36 => Self::ShcfgJclkstop,
                    37 => Self::ShcfgJclkstopNumber,
                    38 => Self::ShcfgTp,
                    39 => Self::ShcfgTpNumber,
                    40 => Self::NicfgNi0,
                    41 => Self::NicfgNumberNi0,
                    42 => Self::NicfgNi0Number,
                    43 => Self::NicfgNi0NumberNumber,
                    44 => Self::NicfgNi0NumberNumberNumber,
                    45 => Self::NicfgNi0NumberNumberNumberNumber,
                    46 => Self::NicfgNi0NumberNumberNumberNumberNumber,
                    47 => Self::NicfgNumberNi0Number,
                    48 => Self::NicfgNumberNi0NumberNumber,
                    49 => Self::NicfgNumberNi0NumberNumberNumber,
                    50 => Self::NicfgNumberNi0NumberNumberNumberNumber,
                    51 => Self::NicfgNumberNi0NumberNumberNumberNumberNumber,
                    52 => Self::Nlcfg,
                    53 => Self::NlcfgWord,
                    54 => Self::NlcfgWordWord,
                    55 => Self::NlcfgWordNl3,
                    56 => Self::NlcfgWordWordNl3,
                    57 => Self::NlcfgWordEnable,
                    58 => Self::NlcfgWordWordEnable,
                    59 => Self::NlcfgWordDisable,
                    60 => Self::NlcfgWordWordDisable,
                    61 => Self::RmmrWordNumber,
                    62 => Self::WmmrWordNumberNumberNumber,
                    63 => Self::PktgenReadWordNumber,
                    64 => Self::PktgenWriteWordNumberNumberNumber,
                    65 => Self::PktgenDebugNumber,
                    66 => Self::TuneGet,
                    67 => Self::Promice,
                    68 => Self::PromiceE,
                    69 => Self::PromiceD,
                    70 => Self::PromiceNumberE,
                    71 => Self::PromiceNumberD,
                    72 => Self::Promver,
                    73 => Self::Node,
                    74 => Self::NodeE,
                    75 => Self::NodeD,
                    76 => Self::NodeNumberE,
                    77 => Self::NodeNumberD,
                    78 => Self::PicMode,
                    79 => Self::War,
                    80 => Self::WarE,
                    81 => Self::WarD,
                    82 => Self::Shdbg,
                    83 => Self::ShdbgRetrylimitNumber,
                    84 => Self::ShdbgRdelayNumber,
                    85 => Self::ShdbgFullresetNumber,
                    86 => Self::ShdbgLinkdownNumber,
                    87 => Self::Mbrick,
                    88 => Self::MbrickMemory,
                    89 => Self::MbrickMemoryConfirm,
                    90 => Self::MbrickCpuWordWordWordWord,
                    91 => Self::MbrickCpu,
                    92 => Self::IfIOnlyHadABrain,
                    93 => Self::Ssi,
                    94 => Self::SsiServiceWordWordWordWord,
                    95 => Self::SsiServiceOff,
                    96 => Self::SsiWordWordWordWordWord,
                    97 => Self::MadWithPowerNumber,
                    98 => Self::PwmRead,
                    99 => Self::PwmSetNumberNumber,
                    100 => Self::SysLinkSimOn,
                    101 => Self::SysLinkSimOnNumberNumber,
                    102 => Self::SysLinkSimOff,
                    103 => Self::SysLinkDumpInputNumber,
                    104 => Self::SysLinkDumpOutputNumber,
                    105 => Self::SysLinkCaptureReset,
                    106 => Self::SysLinkLogNumber,
                    107 => Self::SysLinkLogAll,
                    108 => Self::SysLinkLogReset,
                    109 => Self::SysLinkLogNumberInsertCommand,
                    110 => Self::SysLinkConfig,
                    111 => Self::SysLinkQsuspend,
                    112 => Self::SysLinkQsuspendNumber,
                    113 => Self::ErrorNumber,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::CheckPowerOn => 0,
                    Self::CheckMarginHigh => 1,
                    Self::CheckScrambleEnabled => 2,
                    Self::CheckLinks => 3,
                    Self::CheckDebugNumber => 4,
                    Self::CheckSecurityOn => 5,
                    Self::CheckSerialWord => 6,
                    Self::FruCapture => 7,
                    Self::FruCaptureVerbose => 8,
                    Self::FruExamine => 9,
                    Self::FruDump => 10,
                    Self::FruReset => 11,
                    Self::FruDeadlock => 12,
                    Self::FruDeadlockVerbose => 13,
                    Self::FruDeadlockQuiet => 14,
                    Self::FruEvents => 15,
                    Self::Memcfg => 16,
                    Self::Memcfg1bank166E => 17,
                    Self::Shcfg => 18,
                    Self::ShcfgFsbReset => 19,
                    Self::ShcfgFsb => 20,
                    Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber => 21,
                    Self::ShcfgWrt90Reset => 22,
                    Self::ShcfgWrt90 => 23,
                    Self::ShcfgWrt90NumberNumberNumber => 24,
                    Self::ShcfgMciReset => 25,
                    Self::ShcfgMci => 26,
                    Self::ShcfgMciNumberNumberNumber => 27,
                    Self::ShcfgMdclk => 28,
                    Self::ShcfgMdclkNumber => 29,
                    Self::ShcfgNiclk => 30,
                    Self::ShcfgNiclkNumber => 31,
                    Self::ShcfgOsc => 32,
                    Self::ShcfgOscNumber => 33,
                    Self::ShcfgGtlRe => 34,
                    Self::ShcfgGtlReNumber => 35,
                    Self::ShcfgJclkstop => 36,
                    Self::ShcfgJclkstopNumber => 37,
                    Self::ShcfgTp => 38,
                    Self::ShcfgTpNumber => 39,
                    Self::NicfgNi0 => 40,
                    Self::NicfgNumberNi0 => 41,
                    Self::NicfgNi0Number => 42,
                    Self::NicfgNi0NumberNumber => 43,
                    Self::NicfgNi0NumberNumberNumber => 44,
                    Self::NicfgNi0NumberNumberNumberNumber => 45,
                    Self::NicfgNi0NumberNumberNumberNumberNumber => 46,
                    Self::NicfgNumberNi0Number => 47,
                    Self::NicfgNumberNi0NumberNumber => 48,
                    Self::NicfgNumberNi0NumberNumberNumber => 49,
                    Self::NicfgNumberNi0NumberNumberNumberNumber => 50,
                    Self::NicfgNumberNi0NumberNumberNumberNumberNumber => 51,
                    Self::Nlcfg => 52,
                    Self::NlcfgWord => 53,
                    Self::NlcfgWordWord => 54,
                    Self::NlcfgWordNl3 => 55,
                    Self::NlcfgWordWordNl3 => 56,
                    Self::NlcfgWordEnable => 57,
                    Self::NlcfgWordWordEnable => 58,
                    Self::NlcfgWordDisable => 59,
                    Self::NlcfgWordWordDisable => 60,
                    Self::RmmrWordNumber => 61,
                    Self::WmmrWordNumberNumberNumber => 62,
                    Self::PktgenReadWordNumber => 63,
                    Self::PktgenWriteWordNumberNumberNumber => 64,
                    Self::PktgenDebugNumber => 65,
                    Self::TuneGet => 66,
                    Self::Promice => 67,
                    Self::PromiceE => 68,
                    Self::PromiceD => 69,
                    Self::PromiceNumberE => 70,
                    Self::PromiceNumberD => 71,
                    Self::Promver => 72,
                    Self::Node => 73,
                    Self::NodeE => 74,
                    Self::NodeD => 75,
                    Self::NodeNumberE => 76,
                    Self::NodeNumberD => 77,
                    Self::PicMode => 78,
                    Self::War => 79,
                    Self::WarE => 80,
                    Self::WarD => 81,
                    Self::Shdbg => 82,
                    Self::ShdbgRetrylimitNumber => 83,
                    Self::ShdbgRdelayNumber => 84,
                    Self::ShdbgFullresetNumber => 85,
                    Self::ShdbgLinkdownNumber => 86,
                    Self::Mbrick => 87,
                    Self::MbrickMemory => 88,
                    Self::MbrickMemoryConfirm => 89,
                    Self::MbrickCpuWordWordWordWord => 90,
                    Self::MbrickCpu => 91,
                    Self::IfIOnlyHadABrain => 92,
                    Self::Ssi => 93,
                    Self::SsiServiceWordWordWordWord => 94,
                    Self::SsiServiceOff => 95,
                    Self::SsiWordWordWordWordWord => 96,
                    Self::MadWithPowerNumber => 97,
                    Self::PwmRead => 98,
                    Self::PwmSetNumberNumber => 99,
                    Self::SysLinkSimOn => 100,
                    Self::SysLinkSimOnNumberNumber => 101,
                    Self::SysLinkSimOff => 102,
                    Self::SysLinkDumpInputNumber => 103,
                    Self::SysLinkDumpOutputNumber => 104,
                    Self::SysLinkCaptureReset => 105,
                    Self::SysLinkLogNumber => 106,
                    Self::SysLinkLogAll => 107,
                    Self::SysLinkLogReset => 108,
                    Self::SysLinkLogNumberInsertCommand => 109,
                    Self::SysLinkConfig => 110,
                    Self::SysLinkQsuspend => 111,
                    Self::SysLinkQsuspendNumber => 112,
                    Self::ErrorNumber => 113,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
