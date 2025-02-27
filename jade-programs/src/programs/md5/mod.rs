use crate::{HasExecutable, HasStartAddress};
use lazy_static_include::*;

lazy_static_include_bytes! {
    MD5_EXECUTABLE => "src/programs/md5/main"
}

pub struct Md5;

impl HasStartAddress for Md5 {
    fn get_start_address() -> u16 {
        0x0200
    }
}

impl HasExecutable for Md5 {
    fn get_executable() -> &'static [u8] {
        &MD5_EXECUTABLE
    }
}
