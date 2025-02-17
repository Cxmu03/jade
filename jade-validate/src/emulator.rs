use crate::cpu_status::CpuStatus;

pub(crate) trait Emulator {
    fn init(&mut self);
    fn init_with_status(&mut self, status: CpuStatus);
    fn step_cycle(&mut self);
    fn deinit(&mut self);
}
