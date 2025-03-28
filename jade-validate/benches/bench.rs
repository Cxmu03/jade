use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use jade_programs::*;
use jade_validate::common::traits::{HasInitialCpuStatus, LoadExecutable, StepCycle};
use jade_validate::emulators::{emulator_6502::Emulator6502, jade::Jade, perfect6502::Perfect6502};

const DEFAULT_EXECUTABLE: &[u8] = &[
    0xa9, 0x00, 0x20, 0x10, 0x00, 0x4c, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
    0xe8, 0x88, 0xe6, 0x0f, 0x38, 0x69, 0x02, 0x60,
];

pub fn benchmark_executables(mut benchmark_fn: impl FnMut(&[u8], &u16, &u16, &str)) {
    let md5 = Md5::new();
    let dormann = Dormann::new();

    let executables = &[
        (DEFAULT_EXECUTABLE, 0x00u16, 0x00u16, "Default Executable"),
        (
            md5.get_executable(),
            md5.get_load_address(),
            md5.get_start_address(),
            md5.get_name(),
        ),
        (
            dormann.get_executable(),
            dormann.get_load_address(),
            dormann.get_start_address(),
            dormann.get_name(),
        ),
    ];

    for (executable, load_address, start_address, name) in executables {
        benchmark_fn(executable, load_address, start_address, name);
    }
}

pub fn run_for_n_cycles<E: StepCycle>(cycles: usize, emulator: &mut E) {
    for _ in 0..cycles {
        emulator.step_cycle();
    }
}

pub fn perfect6502_bench(c: &mut Criterion) {
    let mut perfect6502_group = c.benchmark_group("perfect6502");
    perfect6502_group.sample_size(10);

    benchmark_executables(|executable, load_address, start_address, name| {
        let mut perfect6502_emu = Perfect6502::new();
        perfect6502_emu
            .load_executable_to(executable, *load_address)
            .unwrap();
        perfect6502_emu.set_reset_vector(*start_address);

        for i in [100, 1000, 10000, 100000, 1000000] {
            perfect6502_emu.reset();
            perfect6502_group.bench_function(&format!("{name} {i} cycles"), |b| {
                b.iter(|| run_for_n_cycles(i, &mut perfect6502_emu))
            });
        }
    })
}

pub fn jade_bench(c: &mut Criterion) {
    let mut jade_group = c.benchmark_group("jade");

    benchmark_executables(|executable, load_address, start_address, name| {
        let mut jade_emu = Jade::new();
        jade_emu
            .load_executable_to(executable, *load_address)
            .unwrap();
        jade_emu.set_reset_vector(*start_address);

        for i in [100, 1000, 10000, 100000, 1000000] {
            jade_emu.reset();
            jade_group.bench_function(&format!("{name} {i} cycles"), |b| {
                b.iter(|| run_for_n_cycles(i, &mut jade_emu))
            });
        }
    });
}

pub fn mos6502_bench(c: &mut Criterion) {
    let mut mos6502_group = c.benchmark_group("emulator_6502");

    benchmark_executables(|executable, load_address, start_address, name| {
        let mut mos6502_emu = Emulator6502::new();
        mos6502_emu.bus.load_program(executable, *load_address);

        for i in [100, 1000, 10000, 100000, 1000000] {
            mos6502_emu.cpu.set_program_counter(*start_address);

            mos6502_group.bench_function(&format!("{name} {i} cycles"), |b| {
                b.iter(|| run_for_n_cycles(i, &mut mos6502_emu))
            });
        }
    })
}

criterion_group!(benches, jade_bench, mos6502_bench, perfect6502_bench);
criterion_main!(benches);
