use strum_macros::Display;

#[derive(Debug, PartialEq)]
pub enum CycleType {
    ReadCycle,
    WriteCycle,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum InstructionCycle {
    NYI,
    Read,
    ReadInc,
    ReadStack,
    PopStack,
    DummyWrite,
    ImmOperand,
    ZpgOperand,
    ZpgOperand2,
    ZpgIndexedOperand,
    ZpgXOperand,
    ZpgYOperand,
    AbsOperand1,
    AbsOperand2,
    AbsOperand3,
    AbsIndexedPageCross,
    AbsXOperand,
    AbsXOperandNoSkip,
    AbsYOperand,
    AbsYOperandNoSkip,
    IndirectOperand,
    IndirectXAddressLo,
    IndirectYAddressLo,
    IndirectIndexedAddressHi,
    RelOperand,
    PullStatus,
    Adc,
    And,
    AslA,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dey,
    Dex,
    Eor,
    Inc,
    Inx,
    Iny,
    JmpAbs,
    Jsr1,
    Jsr2,
    Jsr3,
    Jsr4,
    Jsr5,
    Jsr6,
    Lda,
    Ldx,
    Ldy,
    LsrA,
    Lsr,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    RelBranch1,
    RelBranch2,
    Rol,
    RolA,
    Ror,
    RorA,
    Rts1,
    Rts2,
    Rts3,
    Rts4,
    Rts5,
    Sbc,
    Sec,
    Sed,
    Sei,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

#[derive(Debug)]
pub struct Instruction {
    pub identifier: &'static str,
    pub cycles: &'static [InstructionCycle],
}
