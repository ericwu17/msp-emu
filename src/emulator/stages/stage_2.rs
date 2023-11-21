pub struct Stage2AResult {
    pub mem_read_addr_0: u16,
    pub inc_src_reg: bool,
    pub used_instr_word_for_src: bool,
}

pub struct Stage2BResult {
    pub operand_0: u16,
}

pub fn exec_stage_2a(
    curr_instr: u16,
    next_word: u16,
    src_addr_mode: u16,
    src_reg_id: u16,
    regs: &[u16],
) -> Stage2AResult {
    let mut inc_src_reg = false;
    let mut mem_read_addr_0 = 0x0000;
    let mut used_instr_word_for_src = false;
    // if the current instruction takes a source register (i.e is not a jump instruction)
    if (curr_instr & 0xE000) != 0x2000 {
        match src_addr_mode {
            0 => {}
            1 => {
                // if src_reg_id == 3 then it's an immediate
                if src_reg_id != 3 {
                    if src_reg_id == 2 {
                        // absolute addressing
                        mem_read_addr_0 = next_word;
                        used_instr_word_for_src = true;
                    } else {
                        // indexed addressing
                        mem_read_addr_0 = regs[src_reg_id as usize] + next_word;
                        used_instr_word_for_src = true;
                    }
                }
            }
            2 => {
                // indirect addressing
                if src_reg_id != 2 && src_reg_id != 3 {
                    mem_read_addr_0 = regs[src_reg_id as usize];
                }
            }
            3 => {
                if src_reg_id == 0 {
                    // immediate
                    mem_read_addr_0 = regs[0] + 2;
                    used_instr_word_for_src = true;
                } else if src_reg_id != 2 && src_reg_id != 3 {
                    // indirect auto-inc mode
                    mem_read_addr_0 = regs[src_reg_id as usize];
                    inc_src_reg = true;
                }
            }
            _ => unreachable!(),
        }
    }

    Stage2AResult {
        mem_read_addr_0,
        inc_src_reg,
        used_instr_word_for_src,
    }
}

pub fn exec_stage_2b(
    curr_instr: u16,
    src_addr_mode: u16,
    src_reg_id: u16,
    mem_read_addr_0: u16,
    mem: &[u8],
    regs: &[u16],
) -> Stage2BResult {
    // load operand 0
    let operand_0: u16;

    if mem_read_addr_0 != 0x0000 {
        let low_byte = mem[mem_read_addr_0 as usize];
        let high_byte = mem[(mem_read_addr_0 + 1) as usize];
        operand_0 = u16::from_le_bytes([low_byte, high_byte]);
    } else {
        // if the current instruction takes a source register
        if (curr_instr & 0xE000) == 0 || (curr_instr & 0xC000) != 0 {
            match src_addr_mode {
                0 => {
                    operand_0 = regs[src_reg_id as usize];
                }
                1 => {
                    if src_reg_id == 3 {
                        operand_0 = 1; // immediate
                    } else {
                        // unreachable because mem_read_addr_0 would be non-null.
                        unreachable!();
                    }
                }
                2 => {
                    if src_reg_id == 2 {
                        operand_0 = 4; // immediate
                    } else if src_reg_id == 3 {
                        operand_0 = 2; // immediate
                    } else {
                        // unreachable because mem_read_addr_0 would be non-null.
                        unreachable!();
                    }
                }
                3 => {
                    if src_reg_id == 2 {
                        operand_0 = 8; // immediate
                    } else if src_reg_id == 3 {
                        operand_0 = 0xFFFF; // immediate -1.
                    } else {
                        // unreachable because mem_read_addr_0 would be non-null.
                        unreachable!();
                    }
                }
                _ => unreachable!(),
            }
        } else {
            // curr instr is a jump, so operand_0 will contain the destination address calculated here
            let mut signed_offset = (curr_instr & 0x03FF) * 2;
            if signed_offset & 0x0400 != 0 {
                signed_offset |= 0xF800;
            }
            operand_0 = regs[0].overflowing_add(signed_offset).0;
        }
    }

    Stage2BResult { operand_0 }
}
