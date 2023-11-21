pub struct Stage0Result {
    pub curr_instr: u16,
    pub next_word: u16,
    pub next_next_word: u16,
}

pub fn exec_stage_0(mem: &[u8], pc: u16) -> Stage0Result {
    let low_byte = mem[pc as usize];
    let high_byte = mem[(pc + 1) as usize];
    let curr_instr = u16::from_le_bytes([low_byte, high_byte]);

    let low_byte = mem[(pc + 2) as usize];
    let high_byte = mem[(pc + 3) as usize];
    let next_word = u16::from_le_bytes([low_byte, high_byte]);

    let low_byte = mem[(pc + 4) as usize];
    let high_byte = mem[(pc + 5) as usize];
    let next_next_word = u16::from_le_bytes([low_byte, high_byte]);

    Stage0Result {
        curr_instr,
        next_word,
        next_next_word,
    }
}
