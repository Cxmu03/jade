#[derive(PartialEq)]
pub(crate) enum CycleType {
    ReadCycle,
    WriteCycle,
}

/*
    Every enum variant represents one CPU cycle

    For now I will only implement the following instructions:
    - LDA Imm
    - JSR Abs
    - INX Imp
    - DEY Imp
    - INC zp
    - SEC Imp
    - ADC Imm
    - RTS Imp
*/
#[derive(Clone, Copy)]
pub(crate) enum InstructionCycle {
    /// Not yet implemented
    NYI,
    /// mem[pc] -> db
    ImmOperand,
    /// mem[pc] -> buf
    AbsOperand1,
    /// mem[pc] -> db
    /// (db << 8) | buf > pc
    AbsOperand2,
    /// db -> a
    Lda,
}

pub(crate) struct Instruction {
    pub identifier: &'static str,
    pub cycles: &'static [InstructionCycle],
}
