use crate::cpu::Cpu;
use crate::test_init_cpu;
use paste::paste;

macro_rules! test_load_imm {
    ($mnemonic: ident, $opcode: literal, $register: ident) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _load_imm_non_zero_non_negative>]() {
                let value = 0x10;
                let mut cpu = test_init_cpu!(&[$opcode, value, $opcode]);

                cpu.step_instruction();
                assert_eq!(cpu.p.z(), false);
                assert_eq!(cpu.p.n(), false);
                assert_eq!(cpu.$register, value);
            }

            #[test]
            fn [<test_ $mnemonic _load_imm_zero>]() {
                let value = 0xa0;
                let mut cpu = test_init_cpu!(&[$opcode, value, $opcode, 0]);

                cpu.step_instruction();
                assert_eq!(cpu.$register, value);
                assert_eq!(cpu.p.z(), false);

                cpu.step_instruction();
                assert_eq!(cpu.p.n(), false);
                assert_eq!(cpu.p.z(), true);
                assert_eq!(cpu.$register, 0);
            }

            #[test]
            fn [<test_ $mnemonic _load_imm_negative>]() {
                let value = 0xa0;
                let mut cpu = test_init_cpu!(&[$opcode, value, $opcode]);

                cpu.step_instruction();
                assert_eq!(cpu.p.z(), false);
                assert_eq!(cpu.p.n(), true);
                assert_eq!(cpu.$register, value);
            }
        }
    };
}

test_load_imm!(lda, 0xa9, a);
test_load_imm!(ldx, 0xa2, x);
test_load_imm!(ldy, 0xa0, y);
