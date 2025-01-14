use crate::cpu::Cpu;
use crate::test_init_cpu;
use paste::paste;

const EXPECTED_CYCLES_NOT_TAKE_BRANCH: usize = 3;
const EXPECTED_CYCLES_TAKE_BRANCH: usize = 4;
const EXPECTED_CYCLES_TAKE_BRANCH_CROSS_PAGE: usize = 5;

macro_rules! test_branch_instruction {
    ($mnemonic: ident, $opcode: literal, $flag: ident, $jump_condition: literal) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _take_branch_no_page_cross>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0x10]);

                cpu.p.[<set_ $flag>]($jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0x12);
                assert!(cpu.cycles == EXPECTED_CYCLES_TAKE_BRANCH); // 3 execute + 1 fetch
            }

            #[test]
            fn [<test_ $mnemonic _take_branch_page_cross>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0x10], 0xF0);

                cpu.p.[<set_ $flag>]($jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0x0102);
                assert!(cpu.cycles == EXPECTED_CYCLES_TAKE_BRANCH_CROSS_PAGE);
            }

            #[test]
            fn [<test_ $mnemonic _take_branch_negative_no_page_cross>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0xFE]);

                cpu.p.[<set_ $flag>]($jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0x0);
                assert!(cpu.cycles == EXPECTED_CYCLES_TAKE_BRANCH); // 3 execute + 1 fetch
            }

            #[test]
            fn [<test_ $mnemonic _take_branch_negative_page_cross>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0xFD], 0x100);

                cpu.p.[<set_ $flag>]($jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0xFF);
                assert!(cpu.cycles == EXPECTED_CYCLES_TAKE_BRANCH_CROSS_PAGE);
            }

            #[test]
            fn [<test_ $mnemonic _take_branch_overflow>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0x02], 0xFFFD);

                cpu.p.[<set_ $flag>]($jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0x01);
                assert!(cpu.cycles == EXPECTED_CYCLES_TAKE_BRANCH_CROSS_PAGE);
            }

            #[test]
            fn [<test_ $mnemonic _take_branch_underflow>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0xFD]);

                cpu.p.[<set_ $flag>]($jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0xFFFF);
                assert!(cpu.cycles == EXPECTED_CYCLES_TAKE_BRANCH_CROSS_PAGE);
            }

            #[test]
            fn [<test_ $mnemonic _dont_take_branch>]() {
                let mut cpu = test_init_cpu!(&[$opcode, 0x10]);
                cpu.p.[<set_ $flag>](!$jump_condition);
                cpu.step_instruction();

                assert!(cpu.pc == 0x02);
                assert!(cpu.cycles == EXPECTED_CYCLES_NOT_TAKE_BRANCH);
            }
        }
    };
}

test_branch_instruction!(bcc, 0x90, c, false);
test_branch_instruction!(bcs, 0xB0, c, true);
test_branch_instruction!(bvc, 0x50, v, false);
test_branch_instruction!(bvs, 0x70, v, true);
test_branch_instruction!(beq, 0xF0, z, true);
test_branch_instruction!(bne, 0xD0, z, false);
test_branch_instruction!(bmi, 0x30, n, true);
test_branch_instruction!(bpl, 0x10, n, false);
