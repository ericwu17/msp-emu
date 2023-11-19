#[derive(Debug, Clone)]
pub enum Operand {
    // NOTE: "symbolic mode" is not supported
    Reg(Reg),
    IndexedReg(Reg, i16),
    Abs(u16),
    AbsLabel(String), // label, used for global variables
    Indirect(Reg),
    IndirectAutoInc(Reg),
    Imm(u16),
    ImmLabel(String), // label, used for calling functions
}

impl Operand {
    pub fn to_as_bits(&self) -> u16 {
        let bits: u16 = match self {
            Operand::Reg(_) => 0x0,

            Operand::Imm(0) => 0x0,
            Operand::Imm(1) => 0x1,
            Operand::Imm(2) => 0x2,
            Operand::Imm(4) => 0x2,
            Operand::Imm(8) => 0x3,
            Operand::Imm(u16::MAX) => 0x3,

            Operand::IndexedReg(_, _) => 0x1,
            Operand::Abs(_) | Operand::AbsLabel(_) => 0x1,
            Operand::Indirect(_) => 0x2,
            Operand::IndirectAutoInc(_) => 0x3,
            Operand::Imm(_) | Operand::ImmLabel(_) => 0x3,
        };
        return bits << 4;
    }

    pub fn to_ad_bit(&self) -> u16 {
        let bits: u16 = match self {
            Operand::Reg(_) => 0x0,
            Operand::IndexedReg(_, _) => 0x1,
            Operand::Abs(_) | Operand::AbsLabel(_) => 0x01,

            Operand::Indirect(_)
            | Operand::IndirectAutoInc(_)
            | Operand::Imm(_)
            | Operand::ImmLabel(_) => {
                panic!("invalid addressing mode for destination register!")
            }
        };
        return bits << 7;
    }

    pub fn get_imm_word(&self) -> (Option<u16>, Option<String>) {
        match self {
            Operand::Imm(0)
            | Operand::Imm(1)
            | Operand::Imm(2)
            | Operand::Imm(4)
            | Operand::Imm(8)
            | Operand::Imm(u16::MAX) => {}

            Operand::Reg(_) | Operand::Indirect(_) | Operand::IndirectAutoInc(_) => {}
            Operand::IndexedReg(_, offset) => return (Some(*offset as u16), None),
            Operand::Abs(imm) | Operand::Imm(imm) => return (Some(*imm), None),
            Operand::AbsLabel(label) | Operand::ImmLabel(label) => {
                return (Some(0), Some(label.clone()));
            }
        };
        return (None, None);
    }

    pub fn to_reg_bits(&self) -> u16 {
        match self {
            Operand::Imm(0) => 0x3,
            Operand::Imm(1) => 0x3,
            Operand::Imm(2) => 0x3,
            Operand::Imm(4) => 0x2,
            Operand::Imm(8) => 0x2,
            Operand::Imm(u16::MAX) => 0x3,

            Operand::Reg(r)
            | Operand::IndexedReg(r, _)
            | Operand::Indirect(r)
            | Operand::IndirectAutoInc(r) => r.to_bits(),
            Operand::Abs(_) | Operand::AbsLabel(_) => 0x2,
            Operand::Imm(_) | Operand::ImmLabel(_) => 0x0,
        }
    }
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

impl Reg {
    pub fn to_bits(&self) -> u16 {
        match self {
            Reg::PC => 0x0,
            Reg::SP => 0x1,
            Reg::SR => 0x2,
            Reg::CG => 0x3,
            Reg::R4 => 0x4,
            Reg::R5 => 0x5,
            Reg::R6 => 0x6,
            Reg::R7 => 0x7,
            Reg::R8 => 0x8,
            Reg::R9 => 0x9,
            Reg::R10 => 0xA,
            Reg::R11 => 0xB,
            Reg::R12 => 0xC,
            Reg::R13 => 0xD,
            Reg::R14 => 0xE,
            Reg::R15 => 0xF,
        }
    }
}
