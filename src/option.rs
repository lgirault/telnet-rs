use std::collections::HashMap;

// For debuggin and using HashMaps
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TelnetOption {
    TransmitBinary,
    Echo,
    Reconnection,
    SuppressGoAhead,
    ApproxMessageSizeNeg,
    Status,
    TimingMark,
    RCTE,
    OutLineWidth,
    OutPageSize,
    NAOCRD,
    NAOHTS,
    NAOHTD,
    NAOFFD,
    NAOVTS,
    NAOVTD,
    NAOLFD,
    XASCII,
    Logout,
    ByteMacro,
    DET,
    SUPDUP,
    SUPDUPOutput,
    SNDLOC,
    TTYPE,
    EOR,
    TUID,
    OUTMRK,
    TTYLOC,
    OPT3270Regime,
    X3PAD,
    NAWS,
    TSPEED,
    LFLOW,
    Linemode,
    XDISPLOC,
    Environment,
    Authentication,
    Encryption,
    NewEnvironment,
    MSSP,
    Compress,
    Compress2,
    ZMP,
    EXOPL,
    UnknownOption(u8)
}

impl TelnetOption {
    pub fn parse(byte: u8) -> TelnetOption {
        match byte {
            0 => TelnetOption::TransmitBinary,
            1 => TelnetOption::Echo,
            2 => TelnetOption::Reconnection,
            3 => TelnetOption::SuppressGoAhead,
            4 => TelnetOption::ApproxMessageSizeNeg,
            5 => TelnetOption::Status,
            6 => TelnetOption::TimingMark,
            7 => TelnetOption::RCTE,
            8 => TelnetOption::OutLineWidth,
            9 => TelnetOption::OutPageSize,
            10 => TelnetOption::NAOCRD,
            11 => TelnetOption::NAOHTS,
            12 => TelnetOption::NAOHTD,
            13 => TelnetOption::NAOFFD,
            14 => TelnetOption::NAOVTS,
            15 => TelnetOption::NAOVTD,
            16 => TelnetOption::NAOLFD,
            17 => TelnetOption::XASCII,
            18 => TelnetOption::Logout,
            19 => TelnetOption::ByteMacro,
            20 => TelnetOption::DET,
            21 => TelnetOption::SUPDUP,
            22 => TelnetOption::SUPDUPOutput,
            23 => TelnetOption::SNDLOC,
            24 => TelnetOption::TTYPE,
            25 => TelnetOption::EOR,
            26 => TelnetOption::TUID,
            27 => TelnetOption::OUTMRK,
            28 => TelnetOption::TTYLOC,
            29 => TelnetOption::OPT3270Regime,
            30 => TelnetOption::X3PAD,
            31 => TelnetOption::NAWS,
            32 => TelnetOption::TSPEED,
            33 => TelnetOption::LFLOW,
            34 => TelnetOption::Linemode,
            35 => TelnetOption::XDISPLOC,
            36 => TelnetOption::Environment,
            37 => TelnetOption::Authentication,
            38 => TelnetOption::Encryption,
            39 => TelnetOption::NewEnvironment,
            70 => TelnetOption::MSSP,
            85 => TelnetOption::Compress,
            86 => TelnetOption::Compress2,
            93 => TelnetOption::ZMP,
            255 => TelnetOption::EXOPL,
            byte => TelnetOption::UnknownOption(byte)
        }
    }

    pub fn to_byte(&self) -> u8 {
        match *self {
            TelnetOption::TransmitBinary => 0,
            TelnetOption::Echo => 1,
            TelnetOption::Reconnection => 2,
            TelnetOption::SuppressGoAhead => 3,
            TelnetOption::ApproxMessageSizeNeg => 4,
            TelnetOption::Status => 5,
            TelnetOption::TimingMark => 6,
            TelnetOption::RCTE => 7,
            TelnetOption::OutLineWidth => 8,
            TelnetOption::OutPageSize => 9,
            TelnetOption::NAOCRD => 10,
            TelnetOption::NAOHTS => 11,
            TelnetOption::NAOHTD => 12,
            TelnetOption::NAOFFD => 13,
            TelnetOption::NAOVTS => 14,
            TelnetOption::NAOVTD => 15,
            TelnetOption::NAOLFD => 16,
            TelnetOption::XASCII => 17,
            TelnetOption::Logout => 18,
            TelnetOption::ByteMacro => 19,
            TelnetOption::DET => 20,
            TelnetOption::SUPDUP => 21,
            TelnetOption::SUPDUPOutput => 22,
            TelnetOption::SNDLOC => 23,
            TelnetOption::TTYPE => 24,
            TelnetOption::EOR => 25,
            TelnetOption::TUID => 26,
            TelnetOption::OUTMRK => 27,
            TelnetOption::TTYLOC => 28,
            TelnetOption::OPT3270Regime => 29,
            TelnetOption::X3PAD => 30,
            TelnetOption::NAWS => 31,
            TelnetOption::TSPEED => 32,
            TelnetOption::LFLOW => 33,
            TelnetOption::Linemode => 34,
            TelnetOption::XDISPLOC => 35,
            TelnetOption::Environment => 36,
            TelnetOption::Authentication => 37,
            TelnetOption::Encryption => 38,
            TelnetOption::NewEnvironment => 39,
            TelnetOption::MSSP => 70,
            TelnetOption::Compress => 85,
            TelnetOption::Compress2 => 86,
            TelnetOption::ZMP => 93,
            TelnetOption::EXOPL => 255,
            TelnetOption::UnknownOption(byte) => byte
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OptionConfig {
    pub us: bool,
    pub him: bool
}

pub struct TelnetOptionConfigs {
    configs: HashMap<TelnetOption, OptionConfig>,
    default: OptionConfig
}

impl TelnetOptionConfigs {
    pub fn new() -> TelnetOptionConfigs {
        TelnetOptionConfigs {
            configs: HashMap::new(),
            default: OptionConfig {
                us: false,
                him: false
            }
        }
    }

    pub fn add_config(&mut self, opt: TelnetOption, allow_us: bool, allow_him: bool) {
        self.configs.insert(opt, OptionConfig {
            us: allow_us,
            him: allow_him
        });
    }

    pub fn is_us_allowed(&self, opt: &TelnetOption) -> bool {
        match self.configs.get(&opt) {
            Some(c) => c.us,
            None => self.default.us
        }
    }

    pub fn is_him_allowed(&self, opt: &TelnetOption) -> bool {
        match self.configs.get(&opt) {
            Some(c) => c.him,
            None => self.default.him
        }
    }
}
