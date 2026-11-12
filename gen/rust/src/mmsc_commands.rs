// generated from defs/mmsc_commands.toml by defs-gen - do not edit, change the source
// typed command table of the multi-module system
#![allow(dead_code)]

/// (established)
pub mod auth {
    pub const WIDTH: u32 = 32;
    pub mod level { // bits 15:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xffff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const BASIC: u32 = 100; // established
        pub const SUPERVISOR: u32 = 500; // established
        pub const SERVICE: u32 = 1000; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Basic,
            Supervisor,
            Service,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    100 => Self::Basic,
                    500 => Self::Supervisor,
                    1000 => Self::Service,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Basic => 100,
                    Self::Supervisor => 500,
                    Self::Service => 1000,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod cmd {
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
        pub const AUTH: u32 = 0; // established
        pub const AUTHORITY: u32 = 1; // established
        pub const BS: u32 = 2; // established
        pub const CECHO: u32 = 3; // established
        pub const COM: u32 = 4; // established
        pub const CONS: u32 = 5; // established
        pub const CONSOLE: u32 = 6; // established
        pub const DEBUG: u32 = 7; // established
        pub const NUMRACKS: u32 = 8; // established
        pub const DEST: u32 = 9; // established
        pub const DIRECT: u32 = 10; // established
        pub const DOWNLOADER: u32 = 11; // established
        pub const ELSC: u32 = 12; // established
        pub const EMSG: u32 = 13; // established
        pub const END: u32 = 14; // established
        pub const ESC: u32 = 15; // established
        pub const EXIT: u32 = 16; // established
        pub const FFSC: u32 = 17; // established
        pub const FLASH: u32 = 18; // established
        pub const GRAPHICS_COMMAND: u32 = 19; // established
        pub const HELP: u32 = 20; // established
        pub const IPADDR: u32 = 21; // established
        pub const KILL: u32 = 22; // established
        pub const LOG: u32 = 23; // established
        pub const MEM: u32 = 24; // established
        pub const MMSC: u32 = 25; // established
        pub const MMSG: u32 = 26; // established
        pub const MSC: u32 = 27; // established
        pub const NAP_TIME: u32 = 28; // established
        pub const NOP: u32 = 29; // established
        pub const OPTIONS: u32 = 30; // established
        pub const PAGER: u32 = 31; // established
        pub const PAS: u32 = 32; // established
        pub const PASSWORD: u32 = 33; // established
        pub const PRINTENV: u32 = 34; // established
        pub const PWR: u32 = 35; // established
        pub const RACKID: u32 = 36; // established
        pub const RAT: u32 = 37; // established
        pub const REBOOT_FFSC: u32 = 38; // established
        pub const RESET_FFSC: u32 = 39; // established
        pub const RESET_MMSC: u32 = 40; // established
        pub const RESET_NVRAM: u32 = 41; // established
        pub const RMSG: u32 = 42; // established
        pub const SCAN: u32 = 43; // established
        pub const SETENV: u32 = 44; // established
        pub const STEAL: u32 = 45; // established
        pub const PARTLST: u32 = 46; // established
        pub const PARTSCAN: u32 = 47; // established
        pub const PARTGET: u32 = 48; // established
        pub const PARTNAM: u32 = 49; // established
        pub const PARTCLR: u32 = 50; // established
        pub const PARTSET: u32 = 51; // established
        pub const SWAPCONS: u32 = 52; // established
        pub const TTY_REINIT: u32 = 53; // established
        pub const UNSETENV: u32 = 54; // established
        pub const UNSTEAL: u32 = 55; // established
        pub const VER: u32 = 56; // established
        pub const ARBITER: u32 = 58; // established
        pub const MACADDR: u32 = 59; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Auth,
            Authority,
            Bs,
            Cecho,
            Com,
            Cons,
            Console,
            Debug,
            Numracks,
            Dest,
            Direct,
            Downloader,
            Elsc,
            Emsg,
            End,
            Esc,
            Exit,
            Ffsc,
            Flash,
            GraphicsCommand,
            Help,
            Ipaddr,
            Kill,
            Log,
            Mem,
            Mmsc,
            Mmsg,
            Msc,
            NapTime,
            Nop,
            Options,
            Pager,
            Pas,
            Password,
            Printenv,
            Pwr,
            Rackid,
            Rat,
            RebootFfsc,
            ResetFfsc,
            ResetMmsc,
            ResetNvram,
            Rmsg,
            Scan,
            Setenv,
            Steal,
            Partlst,
            Partscan,
            Partget,
            Partnam,
            Partclr,
            Partset,
            Swapcons,
            TtyReinit,
            Unsetenv,
            Unsteal,
            Ver,
            Arbiter,
            Macaddr,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::Auth,
                    1 => Self::Authority,
                    2 => Self::Bs,
                    3 => Self::Cecho,
                    4 => Self::Com,
                    5 => Self::Cons,
                    6 => Self::Console,
                    7 => Self::Debug,
                    8 => Self::Numracks,
                    9 => Self::Dest,
                    10 => Self::Direct,
                    11 => Self::Downloader,
                    12 => Self::Elsc,
                    13 => Self::Emsg,
                    14 => Self::End,
                    15 => Self::Esc,
                    16 => Self::Exit,
                    17 => Self::Ffsc,
                    18 => Self::Flash,
                    19 => Self::GraphicsCommand,
                    20 => Self::Help,
                    21 => Self::Ipaddr,
                    22 => Self::Kill,
                    23 => Self::Log,
                    24 => Self::Mem,
                    25 => Self::Mmsc,
                    26 => Self::Mmsg,
                    27 => Self::Msc,
                    28 => Self::NapTime,
                    29 => Self::Nop,
                    30 => Self::Options,
                    31 => Self::Pager,
                    32 => Self::Pas,
                    33 => Self::Password,
                    34 => Self::Printenv,
                    35 => Self::Pwr,
                    36 => Self::Rackid,
                    37 => Self::Rat,
                    38 => Self::RebootFfsc,
                    39 => Self::ResetFfsc,
                    40 => Self::ResetMmsc,
                    41 => Self::ResetNvram,
                    42 => Self::Rmsg,
                    43 => Self::Scan,
                    44 => Self::Setenv,
                    45 => Self::Steal,
                    46 => Self::Partlst,
                    47 => Self::Partscan,
                    48 => Self::Partget,
                    49 => Self::Partnam,
                    50 => Self::Partclr,
                    51 => Self::Partset,
                    52 => Self::Swapcons,
                    53 => Self::TtyReinit,
                    54 => Self::Unsetenv,
                    55 => Self::Unsteal,
                    56 => Self::Ver,
                    58 => Self::Arbiter,
                    59 => Self::Macaddr,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Auth => 0,
                    Self::Authority => 1,
                    Self::Bs => 2,
                    Self::Cecho => 3,
                    Self::Com => 4,
                    Self::Cons => 5,
                    Self::Console => 6,
                    Self::Debug => 7,
                    Self::Numracks => 8,
                    Self::Dest => 9,
                    Self::Direct => 10,
                    Self::Downloader => 11,
                    Self::Elsc => 12,
                    Self::Emsg => 13,
                    Self::End => 14,
                    Self::Esc => 15,
                    Self::Exit => 16,
                    Self::Ffsc => 17,
                    Self::Flash => 18,
                    Self::GraphicsCommand => 19,
                    Self::Help => 20,
                    Self::Ipaddr => 21,
                    Self::Kill => 22,
                    Self::Log => 23,
                    Self::Mem => 24,
                    Self::Mmsc => 25,
                    Self::Mmsg => 26,
                    Self::Msc => 27,
                    Self::NapTime => 28,
                    Self::Nop => 29,
                    Self::Options => 30,
                    Self::Pager => 31,
                    Self::Pas => 32,
                    Self::Password => 33,
                    Self::Printenv => 34,
                    Self::Pwr => 35,
                    Self::Rackid => 36,
                    Self::Rat => 37,
                    Self::RebootFfsc => 38,
                    Self::ResetFfsc => 39,
                    Self::ResetMmsc => 40,
                    Self::ResetNvram => 41,
                    Self::Rmsg => 42,
                    Self::Scan => 43,
                    Self::Setenv => 44,
                    Self::Steal => 45,
                    Self::Partlst => 46,
                    Self::Partscan => 47,
                    Self::Partget => 48,
                    Self::Partnam => 49,
                    Self::Partclr => 50,
                    Self::Partset => 51,
                    Self::Swapcons => 52,
                    Self::TtyReinit => 53,
                    Self::Unsetenv => 54,
                    Self::Unsteal => 55,
                    Self::Ver => 56,
                    Self::Arbiter => 58,
                    Self::Macaddr => 59,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod gate {
    pub const WIDTH: u32 = 32;
    pub mod level { // bits 15:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xffff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const ABOVE_499: u32 = 499; // established
        pub const ABOVE_999: u32 = 999; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Above499,
            Above999,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    499 => Self::Above499,
                    999 => Self::Above999,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Above499 => 499,
                    Self::Above999 => 999,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
