#[derive(Debug, Clone, Copy)]
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

impl CC {
    pub fn to_bits_repr(&self) -> u16 {
        let res = match self {
            CC::NotEq => 0,
            CC::Eq => 1,
            CC::NoCarry => 2,
            CC::Carry => 3,
            CC::Neg => 4,
            CC::GreaterEq => 5,
            CC::Less => 6,
            CC::Unconditional => 7,
        };

        // shift 10 because in JMP instructions, the condition code occupies bits 12:10
        return res << 10;
    }
}
