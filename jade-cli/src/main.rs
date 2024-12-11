use jade_macros::make_answer;

fn main() {
    make_answer![
    "LDA imm", ImmOperand=>LdaImm=>End;
    "LDA abs", AbsOperand=>Lda=>End
    ];
    println!("Hello from jade");
}
