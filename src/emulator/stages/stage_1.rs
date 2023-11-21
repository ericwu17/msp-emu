pub struct Stage1Result {
    pub src_addr_mode: u16,  // 2 bit quantity
    pub dst_addr_mode: u16,  // 1 bit quantity
    pub is_byte_instr: bool, // 1 bit quantity

    pub opcode: u16, // this will be a 3 or 4 bit quantity depending on which instruction family is being executed
    pub src_reg_id: u16, // 4 bit quantity
    pub dst_reg_id: u16, // 4 bit quantity
}

pub fn exec_stage_1(curr_instr: u16) -> Stage1Result {
    // decode instruction
    let opcode: u16;
    let src_reg_id: u16;
    let dst_reg_id: u16;

    let src_addr_mode = (curr_instr >> 4) & 0x3;
    let dst_addr_mode = (curr_instr >> 7) & 0x1;
    let is_byte_instr = (curr_instr >> 6) & 0x1 == 1;

    if (curr_instr & 0xE000) == 0 {
        opcode = (curr_instr >> 7) & 0x7;
    } else if (curr_instr & 0xC000) == 0 {
        opcode = (curr_instr >> 10) & 0x7;
    } else {
        opcode = (curr_instr >> 12) & 0xF;
    }

    if (curr_instr & 0xE000) == 0 {
        src_reg_id = curr_instr & 0x0F;
        dst_reg_id = 0;
    } else {
        src_reg_id = (curr_instr >> 8) & 0x0F;
        dst_reg_id = curr_instr & 0x0F;
    }
    Stage1Result {
        opcode,
        src_reg_id,
        src_addr_mode,
        dst_reg_id,
        dst_addr_mode,
        is_byte_instr,
    }
}
