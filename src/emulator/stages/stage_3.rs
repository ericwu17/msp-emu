pub struct Stage3AResult {
    pub mem_read_addr_1: u16,
    pub used_instr_word_for_dst: bool,
}

pub struct Stage3BResult {
    pub operand_1: u16,
}

pub fn exec_stage_3a(
    curr_instr: u16,
    next_word: u16,
    next_next_word: u16,
    used_instr_word_for_src: bool,
    dst_addr_mode: u16,
    dst_reg_id: u16,
    regs: &[u16],
) -> Stage3AResult {
    let mut mem_read_addr_1: u16 = 0x0000;
    let mut used_instr_word_for_dst = false;

    // if the current instruction takes a dst register
    if (curr_instr & 0xC000) != 0 {
        match dst_addr_mode {
            1 => {
                used_instr_word_for_dst = true;
                let next_instr_stream_word = if used_instr_word_for_src {
                    next_next_word
                } else {
                    next_word
                };
                if dst_reg_id == 2 {
                    // absolute addressing
                    mem_read_addr_1 = next_instr_stream_word;
                } else {
                    // indexed addressing
                    mem_read_addr_1 = regs[dst_reg_id as usize]
                        .overflowing_add(next_instr_stream_word)
                        .0;
                }
            }
            0 => {
                // register direct addressing.
            }
            _ => unreachable!(),
        }
    }
    Stage3AResult {
        mem_read_addr_1,
        used_instr_word_for_dst,
    }
}

pub fn exec_stage_3b(
    dst_reg_id: u16,
    mem_read_addr_1: u16,
    mem: &[u8],
    regs: &[u16],
) -> Stage3BResult {
    // load operand 1
    let operand_1: u16;

    if mem_read_addr_1 != 0x0000 {
        let low_byte = mem[mem_read_addr_1 as usize];
        let high_byte = mem[(mem_read_addr_1 + 1) as usize];
        operand_1 = u16::from_le_bytes([low_byte, high_byte]);
    } else {
        operand_1 = regs[dst_reg_id as usize];
    }

    Stage3BResult { operand_1 }
}
