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
    /// pc -> ab
    /// mem[ab] -> db
    Read, // Generic read cycle that does nothing
    ReadInc, // Generic read rycle that advances pc
    /// mem[pc] -> db
    ImmOperand,
    /// mem[pc] -> db
    ZpgOperand,
    AbsOperand1,
    AbsOperand2,
    Clc,
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
    /// 1 -> c
    Sec,
    JmpAbs,
    Cli,
    Rts1,
    Rts2,
    Rts3,
    Rts4,
    Rts5,
    /// db -> a
    Lda,
    Clv,
    Cld,
    /// pc -> ab
    /// mem[ab] -> db
    Inx2,
    Sei,
    /// pc -> ab
    /// mem[ab] -> db
    Dey2,
    /// a + db -> a
    Adc1,
    /// db -> ab
    /// mem[ab] -> db
    Inc2,
    /// db -> mem[ab]
    Inc3,
    /// db + 1 -> db
    /// db -> mem[ab]
    Inc4,
    Sed,
}

#[derive(Debug)]
pub struct Instruction {
    pub identifier: &'static str,
    pub cycles: &'static [InstructionCycle],
}
