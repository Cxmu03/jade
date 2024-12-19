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
    /// mem[pc] -> db
    Jsr1,
    /// 256 + sp -> ab
    /// db -> sp
    Jsr2,
    /// (pc >> 8) -> mem[ab]
    Jsr3,
    /// ab - 1 -> ab
    /// (pc & 255) -> mem[ab]
    Jsr4,
    /// pc -> ab
    /// mem[ab] -> db
    Jsr5,
    /// (db << 8) | sp -> ab
    /// ab -> pc
    /// restore sp
    Jsr6,
    /// db -> a
    Lda,
}

pub(crate) struct Instruction {
    pub identifier: &'static str,
    pub cycles: &'static [InstructionCycle],
}
