use super::instruction::{Instruction, InstructionCycle::*};

macro_rules! instruction_table {
    ($($identifier: expr, $($cycles: expr)=>+);+) => {
        &[
            $(Instruction {identifier: $identifier, cycles: &[$($cycles,)*]},)*
        ]
    }
}

// fetch is not listed as a cycle step as it needs to be handled separately for the pipeline
const INSTRUCTIONS: &[Instruction] = instruction_table!(
    "LDA imm", ImmOperand=>Lda
);
