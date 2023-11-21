pub struct Stage5Result {
    pub regs: [u16; 16],
    pub mem_write_addr: u16,
}

pub fn exec_stage_5a(
    mut regs: [u16; 16],
    inc_src_reg: bool,
    dec_sp: bool,
    new_cf: Option<bool>,
    new_zf: Option<bool>,
    new_nf: Option<bool>,
    new_vf: Option<bool>,
    src_reg_id: u16,
    dst_reg_id: u16,
    curr_instr: u16,
    opcode: u16,
    src_addr_mode: u16,
    dst_addr_mode: u16,
    result: u16,
    mem_read_addr_0: u16,
    mem_read_addr_1: u16,
    new_pc_val: u16,
    used_instr_word_for_src: bool,
    used_instr_word_for_dst: bool,
) -> Stage5Result {
    let mut mem_write_addr = 0;

    if let Some(new_cf) = new_cf {
        if new_cf {
            regs[2] |= 0x1;
        } else {
            regs[2] &= !0x1;
        }
    }
    if let Some(new_zf) = new_zf {
        if new_zf {
            regs[2] |= 0x2;
        } else {
            regs[2] &= !0x2;
        }
    }
    if let Some(new_nf) = new_nf {
        if new_nf {
            regs[2] |= 0x4;
        } else {
            regs[2] &= !0x4;
        }
    }
    if let Some(new_vf) = new_vf {
        if new_vf {
            regs[2] |= 0x100;
        } else {
            regs[2] &= !0x100;
        }
    }
    if dec_sp {
        regs[1] -= 2;
    }
    if inc_src_reg {
        regs[src_reg_id as usize] += 2; // TODO: handle b/w
    }

    if (curr_instr & 0xE000) == 0 {
        if opcode == 5 {
            // CALL
            mem_write_addr = regs[0];
        }
        if opcode == 4 {
            // PUSH
            mem_write_addr = regs[1];
        }

        // single operand instr
        match src_addr_mode {
            0 => {
                regs[src_reg_id as usize] = result;
            }
            1 | 2 | 3 => {
                // indexed, indirect, absolute, or indirect auto-inc addressing mode
                mem_write_addr = mem_read_addr_0;
            }
            _ => unreachable!(),
        }
    } else if (curr_instr & 0xC000) == 0 {
        // jmp instr
        // do nothing (no result to write)
    } else {
        // double operand instr
        if opcode == 0x9 || opcode == 0xB {
            // operations cmp and tst do not write any results
        } else {
            match dst_addr_mode {
                0 => {
                    regs[dst_reg_id as usize] = result;
                }
                1 => {
                    // indexed, or absolute addressing mode
                    mem_write_addr = mem_read_addr_1;
                }
                _ => unreachable!(),
            }
        }
    }

    if new_pc_val != 0 {
        regs[0] = new_pc_val + 2;
    } else {
        if used_instr_word_for_src & used_instr_word_for_dst {
            regs[0] += 6;
        } else if used_instr_word_for_src | used_instr_word_for_dst {
            regs[0] += 4;
        } else {
            regs[0] += 2;
        }
    }

    Stage5Result {
        regs,
        mem_write_addr,
    }
}

pub fn exec_stage_5b(mem_write_addr: u16, result: u16, mem: &mut [u8]) {
    if mem_write_addr != 0 {
        let [low_byte, high_byte] = result.to_le_bytes();
        mem[mem_write_addr as usize] = low_byte;
        mem[(mem_write_addr + 1) as usize] = high_byte;
    }
}
