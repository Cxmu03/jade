pub(crate) struct CpuSnapshot {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub db: u8,
    pub ab: u16,
    pub r: bool,
}

pub(crate) struct InitialCpuStatus {
    snapshot: CpuSnapshot,
}
