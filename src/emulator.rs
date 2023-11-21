pub mod double_operand;
pub mod single_operand;
pub mod stages;
pub mod tests;

use self::stages::{
    stage_0::{exec_stage_0, Stage0Result},
    stage_1::{exec_stage_1, Stage1Result},
    stage_2::{exec_stage_2a, exec_stage_2b, Stage2AResult, Stage2BResult},
    stage_3::{exec_stage_3a, exec_stage_3b, Stage3AResult, Stage3BResult},
    stage_4::{exec_stage_4, Stage4Result},
    stage_5::{exec_stage_5a, exec_stage_5b, Stage5Result},
};
use std::fmt;

pub struct Emulator {
    pub mem: [u8; 65536],
    regs: [u16; 16],

    curr_instr: u16,
    next_word: u16,
    next_next_word: u16,

    opcode: u16, // this will be a 3 or 4 bit quantity depending on which instruction family is being executed
    src_reg_id: u16, // 4 bit quantity
    src_addr_mode: u16, // 2 bit quantity
    dst_reg_id: u16, // 4 bit quantity
    dst_addr_mode: u16, // 1 bit quantity

    is_byte_instr: bool, // 1 bit quantity

    operand_0: u16,
    operand_1: u16,

    mem_read_addr_0: u16,
    used_instr_word_for_src: bool,

    mem_read_addr_1: u16,
    used_instr_word_for_dst: bool,

    inc_src_reg: bool,
    dec_sp: bool,

    new_cf: Option<bool>,
    new_zf: Option<bool>,
    new_nf: Option<bool>,
    new_vf: Option<bool>,

    result: u16,
    new_pc_val: u16, // if this value is nonzero, that means jump is taken. Otherwise, jump is not taken.

    mem_write_addr: u16,
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

    pub fn get_gfx_buffer(&self) -> &[u8] {
        return &self.mem.as_slice()[0x8000..=0x895F];
    }

    pub fn get_led_output(&self) -> u16 {
        return u16::from_le_bytes([self.mem[0x8A04], self.mem[0x8A05]]);
    }
    pub fn set_switch_states(&mut self, new_states: u16) {
        [self.mem[0x8A00], self.mem[0x8A01]] = new_states.to_le_bytes();
    }
    pub fn set_button_states(&mut self, new_states: u8) {
        self.mem[0x8A02] = new_states;
    }

    pub fn run_some_instrs(&mut self) {
        for _ in 0..20 {
            self.run_one_instr();
        }
    }

    pub fn run_one_instr(&mut self) {
        self.stage_0();
        println!("{:X}", self.curr_instr);
        self.stage_1();
        println!(
            "{:X} {:X} {} {:X}",
            self.src_addr_mode, self.dst_addr_mode, self.is_byte_instr, self.opcode
        );
        println!("{:X} {:X}", self.src_reg_id, self.dst_reg_id,);
        self.stage_2a();
        println!("mem_read_addr_0: {:X}", self.mem_read_addr_0);
        self.stage_2b();
        self.stage_3a();
        println!("mem_read_addr_1: {:X}", self.mem_read_addr_1);
        self.stage_3b();
        println!("operands are {:X} {:X}", self.operand_0, self.operand_1);
        self.stage_4();
        println!("result is {:X}", self.result);
        self.stage_5a();
        println!("mem_write_addr is {:X}", self.mem_write_addr);
        self.stage_5b();

        println!("finished instr with regs {:?}", self);
        println!("stack is {:X?}", &self.mem[0x7FF0..=0x8000])
    }

    fn stage_0(&mut self) {
        // load current instruction and subsequent words
        let Stage0Result {
            curr_instr,
            next_word,
            next_next_word,
        } = exec_stage_0(&self.mem, self.regs[0]);
        self.curr_instr = curr_instr;
        self.next_word = next_word;
        self.next_next_word = next_next_word;
    }

    fn stage_1(&mut self) {
        // decode instruction
        let Stage1Result {
            src_addr_mode,
            dst_addr_mode,
            is_byte_instr,
            opcode,
            src_reg_id,
            dst_reg_id,
        } = exec_stage_1(self.curr_instr);

        self.src_addr_mode = src_addr_mode;
        self.dst_addr_mode = dst_addr_mode;
        self.is_byte_instr = is_byte_instr;
        self.opcode = opcode;
        self.src_reg_id = src_reg_id;
        self.dst_reg_id = dst_reg_id;
    }

    fn stage_2a(&mut self) {
        // calculate memory load address
        let Stage2AResult {
            mem_read_addr_0,
            inc_src_reg,
            used_instr_word_for_src,
        } = exec_stage_2a(
            self.curr_instr,
            self.next_word,
            self.src_addr_mode,
            self.src_reg_id,
            &self.regs,
        );
        self.mem_read_addr_0 = mem_read_addr_0;
        self.inc_src_reg = inc_src_reg;
        self.used_instr_word_for_src = used_instr_word_for_src;
    }

    fn stage_2b(&mut self) {
        // load operand 0
        let Stage2BResult { operand_0 } = exec_stage_2b(
            self.curr_instr,
            self.src_addr_mode,
            self.src_reg_id,
            self.mem_read_addr_0,
            &self.mem,
            &self.regs,
        );
        self.operand_0 = operand_0;
    }

    fn stage_3a(&mut self) {
        let Stage3AResult {
            mem_read_addr_1,
            used_instr_word_for_dst,
        } = exec_stage_3a(
            self.curr_instr,
            self.next_word,
            self.next_next_word,
            self.used_instr_word_for_src,
            self.dst_addr_mode,
            self.dst_reg_id,
            &self.regs,
        );
        self.mem_read_addr_1 = mem_read_addr_1;
        self.used_instr_word_for_dst = used_instr_word_for_dst;
    }

    fn stage_3b(&mut self) {
        let Stage3BResult { operand_1 } =
            exec_stage_3b(self.dst_reg_id, self.mem_read_addr_1, &self.mem, &self.regs);
        self.operand_1 = operand_1;
    }

    fn stage_4(&mut self) {
        // calculate result
        let Stage4Result {
            dec_sp,
            new_cf,
            new_zf,
            new_nf,
            new_vf,
            result,
            new_pc_val,
        } = exec_stage_4(
            self.curr_instr,
            self.opcode,
            self.operand_0,
            self.operand_1,
            &self.regs,
        );
        self.dec_sp = dec_sp;
        self.new_cf = new_cf;
        self.new_zf = new_zf;
        self.new_nf = new_nf;
        self.new_vf = new_vf;
        self.result = result;
        self.new_pc_val = new_pc_val;
    }

    fn stage_5a(&mut self) {
        let Stage5Result {
            regs,
            mem_write_addr,
        } = exec_stage_5a(
            self.regs,
            self.inc_src_reg,
            self.dec_sp,
            self.new_cf,
            self.new_zf,
            self.new_nf,
            self.new_vf,
            self.src_reg_id,
            self.dst_reg_id,
            self.curr_instr,
            self.opcode,
            self.src_addr_mode,
            self.dst_addr_mode,
            self.result,
            self.mem_read_addr_0,
            self.mem_read_addr_1,
            self.new_pc_val,
            self.used_instr_word_for_src,
            self.used_instr_word_for_dst,
        );
        self.regs = regs;
        self.mem_write_addr = mem_write_addr;
    }

    fn stage_5b(&mut self) {
        exec_stage_5b(self.mem_write_addr, self.result, &mut self.mem);
    }
}
