use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jade_validate::common::traits::{HasInitialCpuStatus, Init, LoadExecutable, StepCycle};
use jade_validate::emulators::{jade::Jade, perfect6502::Perfect6502};

const DEFAULT_EXECUTABLE: &[u8] = &[
    0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
    0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
];

pub fn run_for_n_cycles<E: StepCycle>(cycles: usize, emulator: &mut E) {
    for _ in 0..cycles {
        emulator.step_cycle();
    }
}

pub fn perfect6502_bench(c: &mut Criterion) {
    let mut perfect6502_group = c.benchmark_group("perfect6502");
    perfect6502_group.sample_size(10);

    let mut perfect6502_emu = Perfect6502::new();
    perfect6502_emu.load_executable_to(DEFAULT_EXECUTABLE, 0x0);

    for i in [100, 1000, 10000, 100000, 1000000] {
        perfect6502_emu.reset();
        perfect6502_group.bench_function(&format!("{i} cycles"), |b| {
            b.iter(|| run_for_n_cycles(i, &mut perfect6502_emu))
        });
    }
}

pub fn jade_bench(c: &mut Criterion) {
    let mut jade_group = c.benchmark_group("jade");

    let mut jade_emu = Jade::new();
    jade_emu.load_executable_to(DEFAULT_EXECUTABLE, 0x0);

    for i in [100, 1000, 10000, 100000, 1000000] {
        jade_emu.reset();
        jade_group.bench_function(&format!("{i} cycles"), |b| {
            b.iter(|| run_for_n_cycles(i, &mut jade_emu))
        });
    }
}

criterion_group!(benches, jade_bench, perfect6502_bench);
criterion_main!(benches);
