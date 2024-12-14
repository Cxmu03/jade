/*
    Every enum variant represents one CPU cycle

    For now I will only implement the following instructions:
    - LDA Imm
    - JSR Abs
    - INX Imp
    - DEY Imp
    - INC zp
    - SEC Imp
    - ADC #
    - RTS Imp
*/
pub(crate) enum InstructionCycle {
    /// Skips to next instruction
    End,
    /// pc+1 -> db
    ImmOperand,
    /// db -> a
    Lda,
}

pub(crate) struct Instruction {
    pub identifier: &'static str,
    pub cycles: &'static [InstructionCycle],
}
