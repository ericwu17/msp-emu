pub mod double_operand;
pub mod single_operand;

use self::{double_operand::process_double_operand_w, single_operand::process_single_operand_w};
use std::fmt;

pub struct Emulator {
    pub mem: [u8; 65536],
    regs: [u16; 16],
    curr_instr: u16,

    opcode: u16, // this will be a 3 or 4 bit quantity depending on which instruction family is being executed
    src_reg_id: u16, // 4 bit quantity
    src_addr_mode: u16, // 2 bit quantity
    dst_reg_id: u16, // 4 bit quantity
    dst_addr_mode: u16, // 1 bit quantity

    is_byte_instr: bool, // 1 bit quantity

    operand_1: u16,
    operand_2: u16,

    mem_read_addr: u16,

    inc_src_reg: bool,
    dec_sp: bool,

    new_cf: Option<bool>,
    new_zf: Option<bool>,
    new_nf: Option<bool>,
    new_vf: Option<bool>,

    result: u16,

    jump_taken: bool,
}

impl fmt::Debug for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in self.regs {
            write!(f, "{:X} ", r)?;
        }
        write!(f, "\n")
    }
}

impl Emulator {
    pub fn new(instrs: &Vec<u8>) -> Self {
        let mut emulator: Self = unsafe { std::mem::zeroed() };

        let mut mem = [0u8; 65536];
        for (index, instr) in instrs.iter().enumerate() {
            mem[index] = *instr;
        }
        emulator.mem = mem;

        emulator
    }

    pub fn run_one_instr(&mut self) {
        self.stage_1();
        println!("{:X}", self.curr_instr);
        self.stage_2();
        println!(
            "{:X} {:X} {} {:X}",
            self.src_addr_mode, self.dst_addr_mode, self.is_byte_instr, self.opcode
        );
        println!("{:X} {:X}", self.src_reg_id, self.dst_reg_id,);
        self.stage_3a();
        println!("{:X}", self.mem_read_addr);
        self.stage_3b();
        self.stage_4a();
        self.stage_4b();
        println!("{:X} {:X}", self.operand_1, self.operand_2);
        self.stage_5();
        println!("{:X}", self.result);
        self.stage_6();
        self.stage_n();

        println!("finished instr with regs {:?}", self);
    }

    fn stage_1(&mut self) {
        // load current instruction
        let low_byte = self.mem[self.regs[0] as usize];
        let high_byte = self.mem[(self.regs[0] + 1) as usize];
        self.curr_instr = u16::from_le_bytes([low_byte, high_byte]);
    }

    fn stage_2(&mut self) {
        // decode instruction
        self.src_addr_mode = (self.curr_instr >> 4) & 0x3;
        self.dst_addr_mode = (self.curr_instr >> 7) & 0x1;
        self.is_byte_instr = (self.curr_instr >> 6) & 0x1 == 1;

        if (self.curr_instr & 0xE000) == 0 {
            self.opcode = (self.curr_instr >> 7) & 0x7;
        } else if (self.curr_instr & 0xC000) == 0 {
            self.opcode = (self.curr_instr >> 10) & 0x7;
        } else {
            self.opcode = (self.curr_instr >> 12) & 0xF;
        }

        if (self.curr_instr & 0xE000) == 0 {
            self.src_reg_id = self.curr_instr & 0x0F;
        } else {
            self.src_reg_id = (self.curr_instr >> 8) & 0x0F;
            self.dst_reg_id = self.curr_instr & 0x0F;
        }
    }

    fn stage_3a(&mut self) {
        // calculate memory load address

        // if the current instruction takes a source register
        if (self.curr_instr & 0xE000) == 0 || (self.curr_instr & 0xC000) != 0 {
            self.inc_src_reg = false;
            match self.src_addr_mode {
                1 => {
                    // if src_reg_id == 3 then it's an immediate
                    if self.src_reg_id != 3 {
                        let low_byte = self.mem[(self.regs[0] + 2) as usize];
                        let high_byte = self.mem[(self.regs[0] + 3) as usize];
                        self.regs[0] += 2;
                        let next_instr_stream_word = u16::from_le_bytes([low_byte, high_byte]);
                        if self.src_reg_id == 2 {
                            // absolute addressing
                            self.mem_read_addr = next_instr_stream_word;
                        } else {
                            // indexed addressing
                            self.mem_read_addr =
                                self.regs[self.src_reg_id as usize] + next_instr_stream_word;
                        }
                    }
                }
                2 => {
                    // indirect addressing mode
                    self.mem_read_addr = self.regs[self.src_reg_id as usize];
                }
                3 => {
                    if self.src_reg_id == 0 {
                        // immediate
                        self.mem_read_addr = self.regs[0] + 2;
                        self.regs[0] += 2;
                    } else {
                        // indirect addressing mode
                        self.mem_read_addr = self.regs[self.src_reg_id as usize];
                        self.inc_src_reg = true;
                    }
                }
                0 => {}
                _ => unreachable!(),
            }
        }
    }

    fn stage_3b(&mut self) {
        // load operand 1

        // if the current instruction takes a source register
        if (self.curr_instr & 0xE000) == 0 || (self.curr_instr & 0xC000) != 0 {
            match self.src_addr_mode {
                0 => {
                    self.operand_1 = self.regs[self.src_reg_id as usize];
                }
                1 => {
                    if self.src_reg_id == 3 {
                        self.operand_1 = 1; // immediate
                    } else {
                        // Absolute addressing mode or indexed addressing. Do a memory read:
                        let low_byte = self.mem[self.mem_read_addr as usize];
                        let high_byte = self.mem[(self.mem_read_addr + 1) as usize];
                        self.operand_1 = u16::from_le_bytes([low_byte, high_byte]);
                    }
                }
                2 => {
                    if self.src_reg_id == 2 {
                        self.operand_1 = 4; // immediate
                    } else if self.src_reg_id == 3 {
                        self.operand_1 = 2; // immediate
                    } else {
                        // indirect addressing mode. Do a memory read:
                        let low_byte = self.mem[self.mem_read_addr as usize];
                        let high_byte = self.mem[(self.mem_read_addr + 1) as usize];
                        self.operand_1 = u16::from_le_bytes([low_byte, high_byte]);
                    }
                }
                3 => {
                    if self.src_reg_id == 2 {
                        self.operand_1 = 8; // immediate
                    } else if self.src_reg_id == 3 {
                        self.operand_1 = 0xFFFF; // immediate -1.
                    } else {
                        // indirect auto-increment addressing mode, or immediate from instr stream. Do a memory read:
                        let low_byte = self.mem[self.mem_read_addr as usize];
                        let high_byte = self.mem[(self.mem_read_addr + 1) as usize];
                        self.operand_1 = u16::from_le_bytes([low_byte, high_byte]);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            // curr instr is a jump, so operand_1 will contain the destination address calculated here
            let mut signed_offset = (self.curr_instr & 0x03FF) * 2;
            if signed_offset & 0x0400 != 0 {
                signed_offset |= 0xF800;
            }
            self.operand_1 = self.regs[0] + signed_offset;
        }
    }

    fn stage_4a(&mut self) {
        // calculate memory load address

        // if the current instruction takes a dst register
        if (self.curr_instr & 0xC000) != 0 {
            match self.dst_addr_mode {
                1 => {
                    let low_byte = self.mem[(self.regs[0] + 2) as usize];
                    let high_byte = self.mem[(self.regs[0] + 3) as usize];
                    self.regs[0] += 2;
                    let next_instr_stream_word = u16::from_le_bytes([low_byte, high_byte]);
                    if self.dst_reg_id == 2 {
                        // absolute addressing
                        self.mem_read_addr = next_instr_stream_word;
                    } else {
                        // indexed addressing
                        self.mem_read_addr =
                            self.regs[self.dst_reg_id as usize] + next_instr_stream_word;
                    }
                }
                0 => {}
                _ => unreachable!(),
            }
        }
    }

    fn stage_4b(&mut self) {
        // load operand 2

        if (self.curr_instr & 0xE000) == 0 || (self.curr_instr & 0xC000) != 0 {
            match self.dst_addr_mode {
                1 => {
                    let low_byte = self.mem[self.mem_read_addr as usize];
                    let high_byte = self.mem[(self.mem_read_addr + 1) as usize];
                    self.operand_2 = u16::from_le_bytes([low_byte, high_byte]);
                }
                0 => {
                    self.operand_2 = self.regs[self.dst_reg_id as usize];
                }
                _ => unreachable!(),
            }
        }
    }

    fn stage_5(&mut self) {
        // calculate result
        self.dec_sp = false;

        self.new_cf = None;
        self.new_zf = None;
        self.new_nf = None;
        self.new_vf = None;

        if (self.curr_instr & 0xE000) == 0 {
            // single operand instruction
            let carry_bit = self.regs[2] & 0x01 != 0;
            (
                self.result,
                self.new_cf,
                self.new_zf,
                self.new_nf,
                self.new_vf,
                self.dec_sp,
            ) = process_single_operand_w(self.operand_1, carry_bit, self.opcode);
        } else if (self.curr_instr & 0xC000) == 0 {
            let sr = self.regs[2];
            let carry_flag = sr & 0x01 != 0;
            let zero_flag = sr & 0x02 != 0;
            let neg_flag = sr & 0x04 != 0;
            let overflow_flag = sr & 0x100 != 0;
            match self.opcode {
                0 => self.jump_taken = !zero_flag,                  // JNZ
                1 => self.jump_taken = zero_flag,                   // JZ
                2 => self.jump_taken = !carry_flag,                 // JNC
                3 => self.jump_taken = carry_flag,                  // JC
                4 => self.jump_taken = neg_flag,                    // JN
                5 => self.jump_taken = !(neg_flag ^ overflow_flag), // JGE
                6 => self.jump_taken = neg_flag ^ overflow_flag,    // JL
                7 => self.jump_taken = true,                        // JMP
                _ => unreachable!(),
            }
            self.result = self.operand_1;
        } else {
            // double operand instruction
            let carry_bit = self.regs[2] & 0x01 != 0;
            (
                self.result,
                self.new_cf,
                self.new_zf,
                self.new_nf,
                self.new_vf,
            ) = process_double_operand_w(self.operand_1, self.operand_2, carry_bit, self.opcode);
        }
    }
    fn stage_6(&mut self) {
        // write result to register or memory
        if let Some(new_cf) = self.new_cf {
            if new_cf {
                self.regs[2] |= 0x1;
            } else {
                self.regs[2] &= !0x1;
            }
        }
        if let Some(new_zf) = self.new_zf {
            if new_zf {
                self.regs[2] |= 0x2;
            } else {
                self.regs[2] &= !0x2;
            }
        }
        if let Some(new_nf) = self.new_nf {
            if new_nf {
                self.regs[2] |= 0x4;
            } else {
                self.regs[2] &= !0x4;
            }
        }
        if let Some(new_vf) = self.new_vf {
            if new_vf {
                self.regs[2] |= 0x100;
            } else {
                self.regs[2] &= !0x100;
            }
        }
        if self.dec_sp {
            self.regs[1] -= 2;
        }
        if self.inc_src_reg {
            self.regs[self.src_reg_id as usize] += 2; // TODO: handle b/w
        }

        if (self.curr_instr & 0xE000) == 0 {
            // single operand instr
            match self.src_addr_mode {
                0 => {
                    self.regs[self.src_reg_id as usize] = self.result;
                }
                1 | 2 | 3 => {
                    // indexed, indirect, absolute, or indirect auto-inc addressing mode
                    // TODO: account for b/w
                    let [low_byte, high_byte] = self.result.to_le_bytes();
                    self.mem[self.mem_read_addr as usize] = low_byte;
                    self.mem[(self.mem_read_addr + 1) as usize] = high_byte;
                }
                _ => unreachable!(),
            }
        } else if (self.curr_instr & 0xC000) == 0 {
            // jmp instr
            // do nothing (no result to write)
        } else {
            // double operand instr
            if self.opcode == 0x9 || self.opcode == 0xB {
                // operations cmp and tst do not write any results
                return;
            }
            match self.dst_addr_mode {
                0 => {
                    self.regs[self.dst_reg_id as usize] = self.result;
                }
                1 => {
                    // indexed, or absolute addressing mode
                    // TODO: account for b/w
                    let [low_byte, high_byte] = self.result.to_le_bytes();
                    self.mem[self.mem_read_addr as usize] = low_byte;
                    self.mem[(self.mem_read_addr + 1) as usize] = high_byte;
                }
                _ => unreachable!(),
            }
        }
    }

    fn stage_n(&mut self) {
        // set new instruction pointer

        if (self.curr_instr & 0xE000) == 0x2000 {
            // JMP instruction
            if self.jump_taken {
                self.regs[0] = self.result + 2;
            } else {
                self.regs[0] += 2;
            }
        } else if (self.curr_instr & 0xFF80) == 1280 {
            self.regs[0] = self.result + 2;
        } else {
            self.regs[0] += 2;
        }
    }
}
