use std::fmt;

#[derive(Clone)]
pub struct CpuSnapshot {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub db: u8,
    pub ab: u16,
    pub pc: u16,
    pub r: bool,
}

impl Default for CpuSnapshot {
    fn default() -> Self {
        CpuSnapshot {
            a: 0,
            x: 0,
            y: 0,
            p: 0b00110110,
            sp: 0xff,
            db: 0,
            ab: 0,
            pc: 0,
            r: false,
        }
    }
}

impl fmt::Debug for CpuSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CpuSnapshot")
            .field("a", &format_args!("0x{:02x}", self.a))
            .field("x", &format_args!("0x{:02x}", self.x))
            .field("y", &format_args!("0x{:02x}", self.y))
            .field("p", &format_args!("0b{:08b}", self.p))
            .field("sp", &format_args!("0x{:02x}", self.sp))
            .field("db", &format_args!("0x{:02x}", self.db))
            .field("ab", &format_args!("0x{:04x}", self.ab))
            .field("pc", &format_args!("0x{:04x}", self.pc))
            .field("r", &self.r)
            .finish()
    }
}
