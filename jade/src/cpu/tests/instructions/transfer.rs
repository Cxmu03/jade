use paste::paste;

use crate::cpu::Cpu;
use crate::test_init_cpu;

const EXPECTED_TRANSFER_CYCLES: usize = 3;

macro_rules! test_transfer_impl {
    ($mnemonic: ident, $opcode: literal, $source: ident, sp) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _transfer_ $source _to_sp>]() {
                let val = 0x69;
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.$source = val;
                cpu.sp = 0xFD;

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.$source, val);
                assert_eq!(cpu.sp, val);
                assert_eq!(cpu.cycles, EXPECTED_TRANSFER_CYCLES);
            }
        }
    };
    ($mnemonic: ident, $opcode: literal, $source: ident, $dest: ident) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _transfer_unset_zero_ $source _to_ $dest>]() {
                let val = 0x0;
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.$source = val;
                cpu.$dest = 0x50;
                cpu.p.set_z(false);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.$source, val);
                assert_eq!(cpu.$dest, val);
                assert_eq!(cpu.p.z(), true);
                assert_eq!(cpu.cycles, EXPECTED_TRANSFER_CYCLES);
            }

            #[test]
            fn [<test_ $mnemonic _transfer_set_zero_ $source _to_ $dest>]() {
                let val = 0x69;
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.$source = val;
                cpu.$dest = 0x0;
                cpu.p.set_z(true);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.$source, val);
                assert_eq!(cpu.$dest, val);
                assert_eq!(cpu.p.z(), false);
                assert_eq!(cpu.cycles, EXPECTED_TRANSFER_CYCLES);
            }

            #[test]
            fn [<test_ $mnemonic _transfer_unset_negative_ $source _to_ $dest>]() {
                let val = 0x69;
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.$source = val;
                cpu.$dest = 0x80;
                cpu.p.set_z(true);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.$source, val);
                assert_eq!(cpu.$dest, val);
                assert_eq!(cpu.p.n(), false);
                assert_eq!(cpu.cycles, EXPECTED_TRANSFER_CYCLES);
            }

            #[test]
            fn [<test_ $mnemonic _transfer_set_negative_ $source _to_ $dest>]() {
                let val = 0x80;
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.$source = val;
                cpu.$dest = 0x69;
                cpu.p.set_n(false);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.$source, val);
                assert_eq!(cpu.$dest, val);
                assert_eq!(cpu.p.n(), true);
                assert_eq!(cpu.cycles, EXPECTED_TRANSFER_CYCLES);
            }
        }
    };
}

test_transfer_impl!(txs, 0x9A, x, sp);
test_transfer_impl!(tsx, 0xBA, sp, x);
test_transfer_impl!(txa, 0x8A, x, a);
test_transfer_impl!(tya, 0x98, y, a);
test_transfer_impl!(tax, 0xAA, a, x);
test_transfer_impl!(tay, 0xA8, a, y);
