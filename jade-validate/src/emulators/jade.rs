use crate::cpu_status::CpuStatus;
use crate::emulator::Emulator;
use crate::log::StatusLog;
use jade::cpu::Cpu;

struct Jade {
    cpu: Cpu,
}

impl Emulator for Jade {
    fn init(&mut self) {}

    fn init_with_status(&mut self, status: CpuStatus) {
        self.cpu.a = status.a;
        self.cpu.x = status.x;
        self.cpu.y = status.y;
        self.cpu.sp = status.sp;
        self.cpu.ab = status.ab;
        self.cpu.pc = status.pc;
        self.cpu.db = status.db;
        self.cpu.r = if status.r { ReadCycle } else { WriteCycle };
    }

    fn step_cycle(&mut self) {
        self.cpu.step_cycle();
    }
}

impl StatusLog for Jade {
    fn create_status_snapshot(&self) -> CpuStatus {
        StatusLog {
            a: self.cpu.a,
            x: self.cpu.x,
            y: self.cpu.y,
            sp: self.cpu.sp,
            db: self.cpu.db,
            ab: self.cpu.ab,
            pc: self.cpu.pc,
            r: self.cpu.r == ReadCycle,
        }
    }
}
