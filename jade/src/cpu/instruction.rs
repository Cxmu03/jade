/*
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
    LdaImm,
}

pub(crate) struct Instruction {
    pub mnemonic: &'static str,

    /// The maximum length of an instruction is 7 cycles but the initial fetch is excluded from the 
    /// list of instruction cycles to handle it separately
    pub cycles: [InstructionCycle; 6],
}
