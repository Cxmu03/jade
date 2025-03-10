use std::fmt::{self, Write};

use bitfield::bitfield;

bitfield! {
    #[derive(Clone)]
    pub struct StatusFlags(u8);
    impl Debug;

    pub n, set_n: 7;
    pub v, set_v: 6;
    pub u, set_u: 5; // unused, always 1, except for one cycle during rts :sob:
    pub b, set_b: 4;
    pub d, set_d: 3;
    pub i, set_i: 2;
    pub z, set_z: 1;
    pub c, set_c: 0;
}

impl fmt::Display for StatusFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(if self.n() { 'N' } else { 'n' }).unwrap();
        f.write_char(if self.v() { 'V' } else { 'v' }).unwrap();
        f.write_char('-').unwrap();
        f.write_char(if self.b() { 'B' } else { 'b' }).unwrap();
        f.write_char(if self.d() { 'D' } else { 'd' }).unwrap();
        f.write_char(if self.i() { 'I' } else { 'i' }).unwrap();
        f.write_char(if self.z() { 'Z' } else { 'z' }).unwrap();
        f.write_char(if self.c() { 'C' } else { 'c' }).unwrap();

        Ok(())
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self(0b00110110)
    }
}
