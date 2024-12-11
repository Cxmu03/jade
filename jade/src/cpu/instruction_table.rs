use crate::cpu::instruction::{Instruction, InstructionCycle::*};

macro_rules! instruction_table {
    ($($mnemonic: expr, $($cycles: expr)=>+);+) => {
        &[
            $(Instruction {mnemonic: $mnemonic, cycles: [$($cycles,)*]},)*
        ]
    }
}

const INSTRUCTIONS2: &[Instruction; 1] = &[Instruction {
    mnemonic: "LDA imm",
    cycles: [ImmOperand, LdaImm, End, End, End, End],
}];

const INSTRUCTIONS: &[Instruction; 1] = instruction_table!(
    "LDA imm", ImmOperand=>LdaImm=>End=>End=>End=>End 
);


