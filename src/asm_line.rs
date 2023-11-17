#[derive(Debug)]
pub enum AsmLine {
    Label(String),
    Jump(CC, String),

    RRC(Operand, bool), // boolean: 1 for byte, 0 for word
    SWPB(Operand),
    RRA(Operand, bool),
    SXT(Operand),
    PUSH(Operand, bool),
    CALL(Operand),
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

#[derive(Debug)]
pub enum CC {
    NotEq,
    Eq,
    NoCarry,
    Carry,
    Neg,
    GreaterEq,
    Less,
    Unconditional,
}

#[derive(Debug, Clone)]
pub enum Operand {
    // NOTE: "symbolic mode" is not supported
    Reg(Reg),
    IndexedReg(Reg, i16),
    Absolute(String), // label
    Indirect(Reg),
    IndirectAutoInc(Reg),
    Imm(u16),
    ImmLabel(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    PC,
    SP,
    SR,
    CG,

    R4,
    R5,
    R6,
    R7,

    R8,
    R9,
    R10,
    R11,

    R12,
    R13,
    R14,
    R15,
}
