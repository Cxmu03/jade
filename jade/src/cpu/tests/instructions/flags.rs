use paste::paste;

use crate::cpu::Cpu;
use crate::test_init_cpu;

macro_rules! test_flag_set {
    ($mnemonic: ident, $opcode: literal, $flag: ident) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _flag_set_from_set>]() {
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.p.[<set_ $flag>](true);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.p.$flag(), true);
            }

            #[test]
            fn [<test_ $mnemonic _flag_set_from_unset>]() {
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.p.[<set_ $flag>](false);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.p.$flag(), true);
            }
        }
    };
}

macro_rules! test_flag_clear {
    ($mnemonic: ident, $opcode: literal, $flag: ident) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _flag_unset_from_set>]() {
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.p.[<set_ $flag>](true);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.p.$flag(), false);
            }

            #[test]
            fn [<test_ $mnemonic _flag_unset_from_unset>]() {
                let (mut cpu, mut bus) = test_init_cpu!(&[$opcode]);
                cpu.p.[<set_ $flag>](false);

                cpu.step_instruction(&mut bus);

                assert_eq!(cpu.p.$flag(), false);
            }
        }
    };
}

test_flag_set!(sec, 0x38, c);
test_flag_set!(sed, 0xF8, d);
test_flag_set!(sei, 0x78, i);

test_flag_clear!(clc, 0x18, c);
test_flag_clear!(cld, 0xD8, d);
test_flag_clear!(cli, 0x58, i);
test_flag_clear!(clv, 0xB8, v);
