use super::instruction::InstructionCycle::*;
use super::instruction::{AddressingMode, Instruction, InstructionType};

macro_rules! instruction_table {
    ($($_: literal: $instruction_type: ident , $addressing_mode: ident, $($cycles: expr)=>*);+) => {
        &[
            $(Instruction {instruction_type: InstructionType::$instruction_type, addressing_mode: AddressingMode::$addressing_mode, cycles: &[$($cycles,)*]},)*
        ]
    };
}

pub const NMI: Instruction = Instruction {
    instruction_type: InstructionType::NMI,
    addressing_mode: AddressingMode::Impl,
    cycles: &[Read, PushPch, PushPcl, PhpBrk, NmiVecLo, NmiVecHi, Read],
};

pub const IRQ: Instruction = Instruction {
    instruction_type: InstructionType::IRQ,
    addressing_mode: AddressingMode::Impl,
    cycles: &[Read, PushPch, PushPcl, PhpBrk, IsrVecLo, IsrVecHi, Read],
};

pub const RESET: Instruction = Instruction {
    instruction_type: InstructionType::RESET,
    addressing_mode: AddressingMode::Impl,
    cycles: &[
        Read,
        Read,
        Read,
        ReadStack,
        ReadStackDec,
        ReadStackDec,
        ResetVecLo,
        ResetVecHi,
        Read,
    ],
};

pub const INSTRUCTIONS: &[Instruction] = instruction_table!(
    0x00: BRK, Impl, ReadInc=>PushPch=>PushPcl=>PhpBrk=>IsrVecLo=>IsrVecHi=>Read;
    0x01: ORA, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>Ora;
    0x02: NYI, Impl, NYI;
    0x03: NYI, Impl, NYI;
    0x04: NYI, Impl, NYI;
    0x05: ORA, Zpg, ZpgOperand=>ZpgOperand2=>Ora;
    0x06: ASL, Zpg, ZpgOperand=>ZpgOperand2=>DummyWrite=>Asl;
    0x07: NYI, Impl, NYI;
    0x08: PHP, Impl, Read=>Php;
    0x09: ORA, Imm, ImmOperand=>Ora;
    0x0A: ASL, A, Read=>AslA;
    0x0B: NYI, Impl, NYI;
    0x0C: NYI, Impl, NYI;
    0x0D: ORA, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Ora;
    0x0E: ASL, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>DummyWrite=>Asl;
    0x0F: NYI, Impl, NYI;
    0x10: BPL, Rel, RelOperand=>Bpl=>RelBranch1=>RelBranch2;
    0x11: ORA, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>Ora;
    0x12: NYI, Impl, NYI;
    0x13: NYI, Impl, NYI;
    0x14: NYI, Impl, NYI;
    0x15: ORA, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Ora;
    0x16: ASL, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>DummyWrite=>Asl;
    0x17: NYI, Impl, NYI;
    0x18: CLC, Impl, Read=>Clc;
    0x19: ORA, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Ora;
    0x1A: NYI, Impl, NYI;
    0x1B: NYI, Impl, NYI;
    0x1C: NYI, Impl, NYI;
    0x1D: ORA, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Ora;
    0x1E: ASL, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>AbsIndexedPageCross=>DummyWrite=>Asl;
    0x1F: NYI, Impl, NYI;
    0x20: JSR, Abs, Jsr1=>Jsr2=>Jsr3=>Jsr4=>Jsr5=>Jsr6;
    0x21: AND, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>And;
    0x22: NYI, Impl, NYI;
    0x23: NYI, Impl, NYI;
    0x24: BIT, Zpg, ZpgOperand=>ZpgOperand2=>Bit;
    0x25: AND, Zpg, ZpgOperand=>ZpgOperand2=>And;
    0x26: ROL, Zpg, ZpgOperand=>ZpgOperand2=>DummyWrite=>Rol;
    0x27: NYI, Impl, NYI;
    0x28: PLP, Impl, Read=>ReadStack=>PopStack=>Plp;
    0x29: AND, Imm, ImmOperand=>And;
    0x2A: ROL, A, Read=>RolA;
    0x2B: NYI, Impl, NYI;
    0x2C: BIT, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Bit;
    0x2D: AND, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>And;
    0x2E: ROL, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>DummyWrite=>Rol;
    0x2F: NYI, Impl, NYI;
    0x30: BMI, Rel, RelOperand=>Bmi=>RelBranch1=>RelBranch2;
    0x31: AND, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>And;
    0x32: NYI, Impl, NYI;
    0x33: NYI, Impl, NYI;
    0x34: NYI, Impl, NYI;
    0x35: AND, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>And;
    0x36: ROL, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>DummyWrite=>Rol;
    0x37: NYI, Impl, NYI;
    0x38: SEC, Impl, Read=>Sec;
    0x39: AND, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>And;
    0x3A: NYI, Impl, NYI;
    0x3B: NYI, Impl, NYI;
    0x3C: NYI, Impl, NYI;
    0x3D: AND, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>And;
    0x3E: ROL, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>AbsIndexedPageCross=>DummyWrite=>Rol;
    0x3F: NYI, Impl, NYI;
    0x40: RTI, Impl, ReadInc=>ReadStack=>PullStatus=>Rts2=>Rts3=>Rts4;
    0x41: EOR, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>Eor;
    0x42: NYI, Impl, NYI;
    0x43: NYI, Impl, NYI;
    0x44: NYI, Impl, NYI;
    0x45: EOR, Zpg, ZpgOperand=>ZpgOperand2=>Eor;
    0x46: LSR, Zpg, ZpgOperand=>ZpgOperand2=>DummyWrite=>Lsr;
    0x47: NYI, Impl, NYI;
    0x48: PHA, Impl, Read=>Pha;
    0x49: EOR, Imm, ImmOperand=>Eor;
    0x4A: LSR, A, Read=>LsrA;
    0x4B: NYI, Impl, NYI;
    0x4C: JMP, Abs, AbsOperand1=>AbsOperand2=>JmpAbs;
    0x4D: EOR, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Eor;
    0x4E: LSR, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>DummyWrite=>Lsr;
    0x4F: NYI, Impl, NYI;
    0x50: BVC, Rel, RelOperand=>Bvc=>RelBranch1=>RelBranch2;
    0x51: EOR, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>Eor;
    0x52: NYI, Impl, NYI;
    0x53: NYI, Impl, NYI;
    0x54: NYI, Impl, NYI;
    0x55: EOR, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Eor;
    0x56: LSR, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>DummyWrite=>Lsr;
    0x57: NYI, Impl, NYI;
    0x58: CLI, Impl, Read=>Cli;
    0x59: EOR, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Eor;
    0x5A: NYI, Impl, NYI;
    0x5B: NYI, Impl, NYI;
    0x5C: NYI, Impl, NYI;
    0x5D: EOR, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Eor;
    0x5E: LSR, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>AbsIndexedPageCross=>DummyWrite=>Lsr;
    0x5F: NYI, Impl, NYI;
    0x60: RTS, Impl, ReadInc=>Rts1=>Rts2=>Rts3=>Rts4=>Read;
    0x61: ADC, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>Adc;
    0x62: NYI, Impl, NYI;
    0x63: NYI, Impl, NYI;
    0x64: NYI, Impl, NYI;
    0x65: ADC, Zpg, ZpgOperand=>ZpgOperand2=>Adc;
    0x66: ROR, Zpg, ZpgOperand=>ZpgOperand2=>DummyWrite=>Ror;
    0x67: NYI, Impl, NYI;
    0x68: PLA, Impl, Read=>ReadStack=>PopStack=>Pla;
    0x69: ADC, Imm, ImmOperand=>Adc;
    0x6A: ROR, A, Read=>RorA;
    0x6B: NYI, Impl, NYI;
    0x6C: JMP, Ind, AbsOperand1=>AbsOperand2=>IndirectOperand=>AbsOperand2=>JmpAbs;
    0x6D: ADC, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Adc;
    0x6E: ROR, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>DummyWrite=>Ror;
    0x6F: NYI, Impl, NYI;
    0x70: BVS, Rel, RelOperand=>Bvs=>RelBranch1=>RelBranch2;
    0x71: ADC, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>Adc;
    0x72: NYI, Impl, NYI;
    0x73: NYI, Impl, NYI;
    0x74: NYI, Impl, NYI;
    0x75: ADC, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Adc;
    0x76: ROR, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>DummyWrite=>Ror;
    0x77: NYI, Impl, NYI;
    0x78: SEI, Impl, Read=>Sei;
    0x79: ADC, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Adc;
    0x7A: NYI, Impl, NYI;
    0x7B: NYI, Impl, NYI;
    0x7C: NYI, Impl, NYI;
    0x7D: ADC, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Adc;
    0x7E: ROR, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>AbsIndexedPageCross=>DummyWrite=>Ror;
    0x7F: NYI, Impl, NYI;
    0x80: NYI, Impl, NYI;
    0x81: STA, IndX, ZpgOperand=>ZpgOperand2=>IndirectXAddressLo=>IndirectIndexedAddressHi=>StaAbs;
    0x82: NYI, Impl, NYI;
    0x83: NYI, Impl, NYI;
    0x84: STY, Zpg, ZpgOperand=>StyZpg;
    0x85: STA, Zpg, ZpgOperand=>StaZpg;
    0x86: STX, Zpg, ZpgOperand=>StxZpg;
    0x87: NYI, Impl, NYI;
    0x88: DEY, Impl, Read=>Dey;
    0x89: NYI, Impl, NYI;
    0x8A: TXA, Impl, Read=>Txa;
    0x8B: NYI, Impl, NYI;
    0x8C: STY, Abs, AbsOperand1=>AbsOperand2=>StyAbs;
    0x8D: STA, Abs, AbsOperand1=>AbsOperand2=>StaAbs;
    0x8E: STX, Abs, AbsOperand2=>AbsOperand2=>StxAbs;
    0x8F: NYI, Impl, NYI;
    0x90: BCC, Rel, RelOperand=>Bcc=>RelBranch1=>RelBranch2;
    0x91: STA, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperandNoSkip=>StaAbsIndexed;
    0x92: NYI, Impl, NYI;
    0x93: NYI, Impl, NYI;
    0x94: STA, ZpgX, ZpgOperand=>ZpgIndexedOperand=>StyZpgX;
    0x95: STA, ZpgX, ZpgOperand=>ZpgIndexedOperand=>StaZpgX;
    0x96: STX, ZpgY, ZpgOperand=>ZpgIndexedOperand=>StxZpgY;
    0x97: NYI, Impl, NYI;
    0x98: TYA, Impl, Read=>Tya;
    0x99: STA, AbsX, AbsOperand1=>AbsOperand2=>AbsYOperandNoSkip=>StaAbsIndexed;
    0x9A: TXS, Impl, Read=>Txs;
    0x9B: NYI, Impl, NYI;
    0x9C: NYI, Impl, NYI;
    0x9D: STA, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>StaAbsIndexed;
    0x9E: NYI, Impl, NYI;
    0x9F: NYI, Impl, NYI;
    0xA0: LDY, Imm, ImmOperand=>Ldy;
    0xA1: LDA, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>Lda;
    0xA2: LDX, Imm, ImmOperand=>Ldx;
    0xA3: NYI, Impl, NYI;
    0xA4: LDY, Zpg, ZpgOperand=>ZpgOperand2=>Ldy;
    0xA5: LDA, Zpg, ZpgOperand=>ZpgOperand2=>Lda;
    0xA6: LDX, Zpg, ZpgOperand=>ZpgOperand2=>Ldx;
    0xA7: NYI, Impl, NYI;
    0xA8: TAY, Impl, Read=>Tay;
    0xA9: LDA, Imm, ImmOperand=>Lda;
    0xAA: TAX, Impl, Read=>Tax;
    0xAB: NYI, Impl, NYI;
    0xAC: LDY, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Ldy;
    0xAD: LDA, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Lda;
    0xAE: LDX, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Ldx;
    0xAF: NYI, Impl, NYI;
    0xB0: BCS, Rel, RelOperand=>Bcs=>RelBranch1=>RelBranch2;
    0xB1: LDA, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>Lda;
    0xB2: NYI, Impl, NYI;
    0xB3: NYI, Impl, NYI;
    0xB4: LDY, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Ldy;
    0xB5: LDA, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Lda;
    0xB6: LDX, ZpgY, ZpgOperand=>ZpgIndexedOperand=>ZpgYOperand=>Ldx;
    0xB7: NYI, Impl, NYI;
    0xB8: CLV, Impl, Read=>Clv;
    0xB9: LDA, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Lda;
    0xBA: TSX, Impl, Read=>Tsx;
    0xBB: NYI, Impl, NYI;
    0xBC: LDY, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Ldy;
    0xBD: LDA, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Lda;
    0xBE: LDX, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Ldx;
    0xBF: NYI, Impl, NYI;
    0xC0: CPY, Imm, ImmOperand=>Cpy;
    0xC1: CMP, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>Cmp;
    0xC2: NYI, Impl, NYI;
    0xC3: NYI, Impl, NYI;
    0xC4: CPY, Zpg, ZpgOperand=>ZpgOperand2=>Cpy;
    0xC5: CMP, Zpg, ZpgOperand=>ZpgOperand2=>Cmp;
    0xC6: DEC, Zpg, ZpgOperand=>ZpgOperand2=>DummyWrite=>Dec;
    0xC7: NYI, Impl, NYI;
    0xC8: INY, Impl, Read=>Iny;
    0xC9: CMP, Imm, ImmOperand=>Cmp;
    0xCA: DEX, Impl, Read=>Dex;
    0xCB: NYI, Impl, NYI;
    0xCC: CPY, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Cpy;
    0xCD: CMP, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Cmp;
    0xCE: DEC, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>DummyWrite=>Dec;
    0xCF: NYI, Impl, NYI;
    0xD0: BNE, Rel, RelOperand=>Bne=>RelBranch1=>RelBranch2;
    0xD1: CMP, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>Cmp;
    0xD2: NYI, Impl, NYI;
    0xD3: NYI, Impl, NYI;
    0xD4: NYI, Impl, NYI;
    0xD5: CMP, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Cmp;
    0xD6: DEC, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>DummyWrite=>Dec;
    0xD7: NYI, Impl, NYI;
    0xD8: CLD, Impl, Read=>Cld;
    0xD9: CMP, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Cmp;
    0xDA: NYI, Impl, NYI;
    0xDB: NYI, Impl, NYI;
    0xDC: NYI, Impl, NYI;
    0xDD: CMP, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Cmp;
    0xDE: DEC, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>AbsIndexedPageCross=>DummyWrite=>Dec;
    0xDF: NYI, Impl, NYI;
    0xE0: CPX, Imm, ImmOperand=>Cpx;
    0xE1: SBC, IndX, ZpgOperand=>ZpgIndexedOperand=>IndirectXAddressLo=>IndirectIndexedAddressHi=>AbsOperand3=>Sbc;
    0xE2: NYI, Impl, NYI;
    0xE3: NYI, Impl, NYI;
    0xE4: CPX, Zpg, ZpgOperand=>ZpgOperand2=>Cpx;
    0xE5: SBC, Zpg, ZpgOperand=>ZpgOperand2=>Sbc;
    0xE6: INC, Zpg, ZpgOperand=>ZpgOperand2=>DummyWrite=>Inc;
    0xE7: NYI, Impl, NYI;
    0xE8: INX, Impl, Read=>Inx;
    0xE9: SBC, Imm, ImmOperand=>Sbc;
    0xEA: NOP, Impl, Read=>Read;
    0xEB: NYI, Impl, NYI;
    0xEC: CPX, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Cpx;
    0xED: SBC, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>Sbc;
    0xEE: INC, Abs, AbsOperand1=>AbsOperand2=>AbsOperand3=>DummyWrite=>Inc;
    0xEF: NYI, Impl, NYI;
    0xF0: BEQ, Rel, RelOperand=>Beq=>RelBranch1=>RelBranch2;
    0xF1: SBC, IndY, ZpgOperand=>IndirectYAddressLo=>IndirectIndexedAddressHi=>AbsYOperand=>AbsIndexedPageCross=>Sbc;
    0xF2: NYI, Impl, NYI;
    0xF3: NYI, Impl, NYI;
    0xF4: NYI, Impl, NYI;
    0xF5: SBC, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>Sbc;
    0xF6: INC, ZpgX, ZpgOperand=>ZpgIndexedOperand=>ZpgXOperand=>DummyWrite=>Inc;
    0xF7: NYI, Impl, NYI;
    0xF8: SED, Impl, Read=>Sed;
    0xF9: SBC, AbsY, AbsOperand1=>AbsOperand2=>AbsYOperand=>AbsIndexedPageCross=>Sbc;
    0xFA: NYI, Impl, NYI;
    0xFB: NYI, Impl, NYI;
    0xFC: NYI, Impl, NYI;
    0xFD: SBC, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperand=>AbsIndexedPageCross=>Sbc;
    0xFE: INC, AbsX, AbsOperand1=>AbsOperand2=>AbsXOperandNoSkip=>AbsIndexedPageCross=>DummyWrite=>Inc;
    0xFF: NYI, Impl, NYI
);
