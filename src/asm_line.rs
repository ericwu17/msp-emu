use crate::ccode::CC;
use crate::operand::Operand;

#[derive(Debug, Clone)]
pub enum AsmLine {
    Label(String),

    Jump(CC, String), // conditional and unconditional jumps

    RRC(Operand, bool), // boolean: 1 for byte, 0 for word
    SWPB(Operand, bool),
    RRA(Operand, bool),
    SXT(Operand, bool),
    PUSH(Operand, bool),
    CALL(Operand, bool),
    RETI,

    MOV(Operand, Operand, bool),
    ADD(Operand, Operand, bool),
    ADDC(Operand, Operand, bool),
    SUB(Operand, Operand, bool),
    SUBC(Operand, Operand, bool),
    CMP(Operand, Operand, bool),
    DADD(Operand, Operand, bool),
    BIT(Operand, Operand, bool),
    BIC(Operand, Operand, bool),
    BIS(Operand, Operand, bool),
    XOR(Operand, Operand, bool),
    AND(Operand, Operand, bool),
}
impl AsmLine {
    pub fn as_label_str(self) -> String {
        match self {
            AsmLine::Label(s) => s,
            _ => unreachable!(),
        }
    }
}
