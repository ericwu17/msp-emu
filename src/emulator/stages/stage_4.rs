use crate::emulator::{
    double_operand::process_double_operand_w, single_operand::process_single_operand_w,
};

pub struct Stage4Result {
    pub dec_sp: bool,
    pub new_cf: Option<bool>,
    pub new_zf: Option<bool>,
    pub new_nf: Option<bool>,
    pub new_vf: Option<bool>,

    pub result: u16,

    pub new_pc_val: u16,
}

pub fn exec_stage_4(
    curr_instr: u16,
    opcode: u16,
    operand_0: u16,
    operand_1: u16,
    regs: &[u16],
) -> Stage4Result {
    // calculate result

    let mut dec_sp = false;
    let mut new_cf = None;
    let mut new_zf = None;
    let mut new_nf = None;
    let mut new_vf = None;
    let mut result = 0;
    let mut new_pc_val = 0x0000; // if this value is nonzero, that means jump is taken. Otherwise, jump is not taken.

    let sr = regs[2];
    let carry_flag = sr & 0x01 != 0;
    let zero_flag = sr & 0x02 != 0;
    let neg_flag = sr & 0x04 != 0;
    let overflow_flag = sr & 0x100 != 0;

    if (curr_instr & 0xE000) == 0 {
        // single operand instruction
        (result, new_cf, new_zf, new_nf, new_vf, dec_sp) =
            process_single_operand_w(operand_0, carry_flag, opcode);
    } else if (curr_instr & 0xC000) == 0 {
        let jump_taken = match opcode {
            0 => !zero_flag,                  // JNZ
            1 => zero_flag,                   // JZ
            2 => !carry_flag,                 // JNC
            3 => carry_flag,                  // JC
            4 => neg_flag,                    // JN
            5 => !(neg_flag ^ overflow_flag), // JGE
            6 => neg_flag ^ overflow_flag,    // JL
            7 => true,                        // JMP
            _ => unreachable!(),
        };
        if jump_taken {
            new_pc_val = operand_0;
        }
    } else {
        // double operand instruction
        (result, new_cf, new_zf, new_nf, new_vf) =
            process_double_operand_w(operand_0, operand_1, carry_flag, opcode);
    }

    Stage4Result {
        dec_sp,
        new_cf,
        new_zf,
        new_nf,
        new_vf,
        result,
        new_pc_val,
    }
}
