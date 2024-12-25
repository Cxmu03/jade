use strum_macros::Display;

#[derive(PartialEq)]
pub(crate) enum CycleType {
    ReadCycle,
    WriteCycle,
}

impl From<CycleType> for u8 {
    fn from(value: CycleType) -> Self {
        match value {
            CycleType::ReadCycle => 1,
            CycleType::WriteCycle => 0,
        }
    }
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
#[derive(Clone, Copy, Debug, Display)]
pub enum InstructionCycle {
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
    /// pc -> ab
    /// mem[ab] -> db
    Inx1,
    /// pc -> ab
    /// mem[ab] -> db
    Inx2,
}

#[derive(Debug)]
pub struct Instruction {
    pub identifier: &'static str,
    pub cycles: &'static [InstructionCycle],
}
