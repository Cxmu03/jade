use crate::cpu::Cpu;
use crate::test_init_cpu;
use paste::paste;

const EXPECTED_IMM_CYCLES: usize = 4;
const EXPECTED_ABS_CYCLES: usize = 6;
const EXPECTED_ZPGX_CYCLES: usize = 6;

macro_rules! test_alu_imm {
    ($suffix: ident, $mnemonic: ident, $opcode: literal, $a: literal, $b: literal, $expected: literal$(,$($flag: ident==$val: ident),*)?) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _imm_ $suffix>]() {
                let mut cpu = test_init_cpu!(&[$opcode, $b, $opcode]);
                cpu.a = $a;
                $($(cpu.p.[<set_ $flag>](!$val);)*)?

                cpu.step_instruction();
                cpu.step_cycle();

                assert_eq!(cpu.a, $expected);
                $($(assert_eq!(cpu.p.$flag(), $val);)*)?
            }
        }
    }
}

macro_rules! test_alu_absolute_indexed {
    ($suffix: ident, $mnemonic: ident, $opcode: literal, $register: ident, $a: literal, $b: literal, $expected: literal$(,$($flag: ident==$val: ident),*)?) => {
        paste! {
            #[test]
            fn [<test_ $mnemonic _abs_ $register _regular_ $suffix>]() {
                let ([<$register _init>], a_init, adh, adl, val) = (10, $a, 0x02u8, 0x10u8, $b);
                let mut cpu = test_init_cpu!(&[$opcode, adl, adh, 0xa9]);

                cpu.a = a_init;
                cpu.$register = [<$register _init>];
                cpu.bus.data[([<$register _init>] as usize) + (u16::from_be_bytes([adh, adl]) as usize)] = val;
                $($(cpu.p.[<set_ $flag>](!$val);)*)?
                cpu.step_instruction();
                cpu.step_cycle();

                $($(assert_eq!(cpu.p.$flag(), $val);)*)?
                assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
                assert_eq!(cpu.a, $expected);
            }
        }
    };
    (adc, $opcode: literal, $register: ident) => {
        paste! {
            #[test]
            fn [<test_adc_abs_ $register _regular>]() {
                let ([<$register _init>], a_init, adh, adl, val) = (10, 50, 0x02u8, 0x10u8, 16u8);
                let mut cpu = test_init_cpu!(&[$opcode, adl, adh, 0xa9]);

                cpu.a = a_init;
                cpu.$register = [<$register _init>];
                cpu.bus.data[([<$register _init>] as usize) + (u16::from_be_bytes([adh, adl]) as usize)] = val;
                cpu.p.set_c(false);
                cpu.step_instruction();
                cpu.step_cycle();

                assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
                assert_eq!(cpu.a, a_init + val);
            }

            #[test]
            fn [<test_adc_abs_ $register _with_carry>]() {
                let ([<$register _init>], a_init, adh, adl, val) = (10, 50, 0x02u8, 0x10u8, 16u8);
                let mut cpu = test_init_cpu!(&[$opcode, adl, adh, 0xa9]);

                cpu.a = a_init;
                cpu.$register = [<$register _init>];
                cpu.bus.data[([<$register _init>] as usize) + (u16::from_be_bytes([adh, adl]) as usize)] = val;
                cpu.p.set_c(true);
                cpu.step_instruction();
                cpu.step_cycle();

                assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
                assert_eq!(cpu.a, a_init + val + 1);
            }

            #[test]
            fn [<test_adc_abs_ $register _does_carry>]() {
                let ([<$register _init>], a_init, adh, adl, val) = (10, 50, 0x02u8, 0x10u8, 16u8);
                let mut cpu = test_init_cpu!(&[$opcode, adl, adh, 0xa9]);

                cpu.a = a_init;
                cpu.$register = [<$register _init>];
                cpu.bus.data[([<$register _init>] as usize) + (u16::from_be_bytes([adh, adl]) as usize)] = val;
                cpu.p.set_c(true);
                cpu.step_instruction();
                cpu.step_cycle();

                assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
                assert_eq!(cpu.a, a_init + val + 1);
            }

            #[test]
            fn [<test_adc_abs_ $register _does_overflow>]() {
                let ([<$register _init>], a_init, adh, adl, val) = (10, 0x80, 0x02u8, 0x10u8, 0xFF);
                let mut cpu = test_init_cpu!(&[$opcode, adl, adh, 0xa9]);

                cpu.a = a_init;
                cpu.$register = [<$register _init>];
                cpu.bus.data[([<$register _init>] as usize) + (u16::from_be_bytes([adh, adl]) as usize)] = val;
                cpu.step_instruction();
                cpu.step_cycle();

                assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
                assert_eq!(cpu.p.v(), true);
                assert_eq!(cpu.a, 0x7F);
            }

            #[test]
            fn [<test_adc_abs_ $register _page_cross>]() {
                let ([<$register _init>], a_init, adh, adl, val) = (0x03, 50, 0x02u8, 0xFEu8, 16u8);
                let mut cpu = test_init_cpu!(&[$opcode, adl, adh, 0xa9]);

                cpu.a = a_init;
                cpu.$register = [<$register _init>];
                cpu.bus.data[([<$register _init>] as usize) + (u16::from_be_bytes([adh, adl]) as usize)] = val;
                cpu.p.set_c(false);
                cpu.step_instruction();
                cpu.step_cycle();

                assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES + 1);
                assert_eq!(cpu.a, a_init + val);
            }
        }
    }
}

test_alu_absolute_indexed!(adc, 0x7D, x);
test_alu_absolute_indexed!(adc, 0x79, y);
test_alu_absolute_indexed!(
    regular,
    and,
    0x3D,
    x,
    0b00110101,
    0b10100011,
    0b00100001,
    n == false,
    z == false
);
test_alu_absolute_indexed!(
    regular,
    and,
    0x39,
    y,
    0b00110101,
    0b10100011,
    0b00100001,
    n == false,
    z == false
);
test_alu_absolute_indexed!(
    negative,
    and,
    0x3D,
    x,
    0b10110101,
    0b10100011,
    0b10100001,
    n == true,
    z == false
);
test_alu_absolute_indexed!(
    negative,
    and,
    0x39,
    y,
    0b10110101,
    0b10100011,
    0b10100001,
    n == true,
    z == false
);
test_alu_absolute_indexed!(
    zero,
    and,
    0x3D,
    x,
    0b10101010,
    0b01010101,
    0b00000000,
    n == false,
    z == true
);
test_alu_absolute_indexed!(
    zero,
    and,
    0x39,
    y,
    0b10101010,
    0b01010101,
    0b00000000,
    n == false,
    z == true
);

test_alu_imm!(
    regular,
    and,
    0x29,
    0b00110101,
    0b10100011,
    0b00100001,
    n == false,
    z == false
);
test_alu_imm!(
    negative,
    and,
    0x29,
    0b10110101,
    0b10100011,
    0b10100001,
    n == true,
    z == false
);
test_alu_imm!(
    zero,
    and,
    0x29,
    0b10101010,
    0b01010101,
    0b00000000,
    n == false,
    z == true
);

#[test]
fn test_adc_impl_regular() {
    let (init, operand) = (50, 16);
    let mut cpu = test_init_cpu!(&[0x69, operand, 0xa9]);

    cpu.a = init;
    cpu.p.set_c(false);
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_IMM_CYCLES);
    assert_eq!(cpu.a, init + operand);
}

#[test]
fn test_adc_impl_with_carry() {
    let (init, operand) = (50, 16);
    let mut cpu = test_init_cpu!(&[0x69, operand, 0xa9]);

    cpu.p.set_c(true);
    cpu.a = init;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_IMM_CYCLES);
    assert_eq!(cpu.a, init + operand + 1);
}

#[test]
fn test_adc_impl_does_carry() {
    let (init, operand) = (254, 2);
    let mut cpu = test_init_cpu!(&[0x69, operand, 0xa9]);

    cpu.a = init;
    cpu.p.set_c(false);
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_IMM_CYCLES);
    assert_eq!(cpu.p.c(), true);
    assert_eq!(cpu.a, 0);
}

#[test]
fn test_adc_impl_does_overflow() {
    let (init, operand) = (0x80, 0xff);
    let mut cpu = test_init_cpu!(&[0x69, operand, 0xa9]);

    cpu.p.set_c(false);
    cpu.a = init;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_IMM_CYCLES);
    assert_eq!(cpu.p.v(), true);
    assert_eq!(cpu.a, 0x7f);
}

#[test]
fn test_adc_abs_regular() {
    let (a_init, adh, adl, val) = (50, 0x02u8, 0x10u8, 16u8);
    let mut cpu = test_init_cpu!(&[0x6D, adl, adh, 0xa9]);

    cpu.a = a_init;
    cpu.bus.data[u16::from_be_bytes([adh, adl]) as usize] = val;
    cpu.p.set_c(false);
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
    assert_eq!(cpu.a, a_init + val);
}

#[test]
fn test_adc_abs_with_carry() {
    let (a_init, adh, adl, val) = (50, 0x02u8, 0x10u8, 16u8);
    let mut cpu = test_init_cpu!(&[0x6D, adl, adh, 0xa9]);

    cpu.a = a_init;
    cpu.bus.data[u16::from_be_bytes([adh, adl]) as usize] = val;
    cpu.p.set_c(true);
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
    assert_eq!(cpu.a, a_init + val + 1);
}

#[test]
fn test_adc_abs_does_carry() {
    let (a_init, adh, adl, val) = (50, 0x02u8, 0x10u8, 16u8);
    let mut cpu = test_init_cpu!(&[0x6D, adl, adh, 0xa9]);

    cpu.a = a_init;
    cpu.bus.data[u16::from_be_bytes([adh, adl]) as usize] = val;
    cpu.p.set_c(true);
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
    assert_eq!(cpu.a, a_init + val + 1);
}

#[test]
fn test_adc_abs_does_overflow() {
    let (a_init, adh, adl, val) = (0x80, 0x02u8, 0x10u8, 0xFF);
    let mut cpu = test_init_cpu!(&[0x6D, adl, adh, 0xa9]);

    cpu.a = a_init;
    cpu.bus.data[u16::from_be_bytes([adh, adl]) as usize] = val;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ABS_CYCLES);
    assert_eq!(cpu.p.v(), true);
    assert_eq!(cpu.a, 0x7F);
}

#[test]
fn test_adc_zpgx_regular() {
    let (a_init, addr, x_init, val) = (50, 100, 10, 16);
    let mut cpu = test_init_cpu!(&[0x75, addr, 0xa9]);

    cpu.a = a_init;
    cpu.x = x_init;
    cpu.bus.data[addr.wrapping_add(cpu.x) as usize] = val;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ZPGX_CYCLES);
    assert_eq!(cpu.a, a_init + val);
}

#[test]
fn test_adc_zpgx_with_carry() {
    let (a_init, addr, x_init, val) = (50, 100, 10, 16);
    let mut cpu = test_init_cpu!(&[0x75, addr, 0xa9]);

    cpu.a = a_init;
    cpu.x = x_init;
    cpu.p.set_c(true);
    cpu.bus.data[addr.wrapping_add(cpu.x) as usize] = val;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ZPGX_CYCLES);
    assert_eq!(cpu.a, a_init + val + 1);
}

#[test]
fn test_adc_zpgx_does_carry() {
    let (a_init, addr, x_init, val) = (0xFE, 100, 10, 2);
    let mut cpu = test_init_cpu!(&[0x75, addr, 0xa9]);

    cpu.a = a_init;
    cpu.x = x_init;
    cpu.bus.data[addr.wrapping_add(cpu.x) as usize] = val;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ZPGX_CYCLES);
    assert_eq!(cpu.p.c(), true);
    assert_eq!(cpu.a, 0);
}

#[test]
fn test_adc_zpgx_does_overflow() {
    let (a_init, addr, x_init, val) = (-128i8 as u8, 100, 10, -1i8 as u8);
    let mut cpu = test_init_cpu!(&[0x75, addr, 0xa9]);

    cpu.a = a_init;
    cpu.x = x_init;
    cpu.bus.data[addr.wrapping_add(cpu.x) as usize] = val;
    cpu.step_instruction();
    cpu.step_cycle();

    assert_eq!(cpu.cycles, EXPECTED_ZPGX_CYCLES);
    assert_eq!(cpu.p.v(), true);
    assert_eq!(cpu.a, 0x7F);
}
