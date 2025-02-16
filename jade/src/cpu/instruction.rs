use std::fmt::Display;

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
    StaZpg,
    StaZpgX,
    StaAbs,
    StaAbsIndexed,
    /*StaIndX,
    StaIndY,*/
    StxZpg,
    StxZpgX,
    StxAbs,
    StyZpg,
    StyZpgX,
    StyAbs,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

#[derive(Debug)]
pub enum AddressingMode {
    Impl,
    Imm,
    Zpg,
    ZpgX,
    ZpgY,
    Abs,
    AbsX,
    AbsY,
    Ind,
    IndX,
    IndY,
    Rel,
    A,
}

impl Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Impl => "impl",
            Self::Imm => "imm",
            Self::Zpg => "zpg",
            Self::ZpgX => "zpg,x",
            Self::ZpgY => "zpg,y",
            Self::Abs => "abs",
            Self::AbsX => "abs,x",
            Self::AbsY => "abs,y",
            Self::Ind => "(abs)",
            Self::IndX => "(zpg,x)",
            Self::IndY => "(zpg),y",
            Self::Rel => "rel",
            Self::A => "a",
        };

        f.write_str(s)
    }
}

#[rustfmt::skip]
#[derive(Debug, Display)]
pub enum InstructionType {
    ADC, AND, ASL,
    BCC, BCS, BEQ,
    BIT, BMI, BNE,
    BPL, BRK, BVC,
    BVS, CLC, CLD,
    CLI, CLV, CMP,
    CPX, CPY, DEC,
    DEX, DEY, EOR,
    INC, INX, INY,
    JMP, JSR, KIL,
    LDA, LDX, LDY,
    LSR, NOP, ORA,
    PHA, PHP, PLA,
    PLP, ROL, ROR,
    RTI, RTS, SBC,
    SEC, SED, SEI,
    STA, STX, STY,
    TAX, TAY, TSX,
    TXA, TXS, TYA,
    NYI
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addressing_mode: AddressingMode,
    pub cycles: &'static [InstructionCycle],
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            self.instruction_type, self.addressing_mode
        ))
    }
}

impl Instruction {
    pub fn is_store(&self) -> bool {
        use InstructionType::*;
        matches!(self.instruction_type, STA | STX | STY)
    }
}
