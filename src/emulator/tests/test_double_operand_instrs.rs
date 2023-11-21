use crate::emulator::{tests::convert_words_to_bytes, Emulator};

#[test]
fn test_mov_imm() {
    let instrs: Vec<u16> = vec![0x4031, 0x8000]; //  MOV.W  #0x8000,SP
    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));

    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 4);
    assert_eq!(cpu_emu.regs[1], 0x8000);
}

#[test]
fn test_mov_between_regs() {
    let instrs: Vec<u16> = vec![0x403F, 0x8000, 0x4F0E];
    //  MOV.W  #0x8000,R15
    //  MOV.W R15,R14

    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));

    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 4);
    assert_eq!(cpu_emu.regs[15], 0x8000);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 6);
    assert_eq!(cpu_emu.regs[15], 0x8000);
    assert_eq!(cpu_emu.regs[14], 0x8000);
}

#[test]
fn test_indirect_addressing() {
    let instrs: Vec<u16> = vec![0x403F, 0x8000, 0x40BF, 0x1234, 0x0000];
    //  MOV.W  #0x8000,R15
    //  MOV.W  #0x1234,@R15

    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 4);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0xA);
    assert_eq!(cpu_emu.mem[0x8000], 0x34);
    assert_eq!(cpu_emu.mem[0x8001], 0x12);
}

#[test]
fn test_indirect_auto_inc_addressing() {
    let instrs: Vec<u16> = vec![0x403F, 0x8000, 0x40BF, 0x1234, 0x0000, 0x4F3E];
    //  MOV.W  #0x8000,R15
    //  MOV.W  #0x1234,@R15
    //  MOV.W  @R15+,R14

    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 4);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0xA);
    assert_eq!(cpu_emu.mem[0x8000], 0x34);
    assert_eq!(cpu_emu.mem[0x8001], 0x12);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0xC);
    assert_eq!(cpu_emu.regs[15], 0x8002);
    assert_eq!(cpu_emu.regs[14], 0x1234);
}

#[test]
fn test_abs_addressing() {
    let instrs: Vec<u16> = vec![
        0x403F, 0x8000, //  MOV.W  #0x8000,R15
        0x40BF, 0x1234, 0x0000, //  MOV.W  #0x1234,@R15
        0x421E, 0x8000, //  MOV.W  &0x8000,R14
        0x40B2, 0x2256, 0x8000, //  MOV.W  #0x2256,&0x8000
        0x421E, 0x8000, //  MOV.W  &0x8000,R14
    ];

    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0x4);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0xA);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0xE);
    assert_eq!(cpu_emu.regs[14], 0x1234);
    cpu_emu.run_one_instr();
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0x18);
    assert_eq!(cpu_emu.regs[14], 0x2256);
}

#[test]
fn test_indexed_addressing() {
    let instrs: Vec<u16> = vec![
        0x403F, 0x8000, //  MOV.W  #0x8000,R15
        0x40BF, 0x1234, 0x0002, //  MOV.W  #0x1234,2(R15)
    ];
    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0x4);
    assert_eq!(cpu_emu.regs[15], 0x8000);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0xA);
    assert_eq!(cpu_emu.mem[0x8002], 0x34);
    assert_eq!(cpu_emu.mem[0x8003], 0x12);
}

#[test]
fn test_sub_instruction() {
    let instrs: Vec<u16> = vec![
        0x403F, 0x0001, //  MOV.W  #1,R15
        0x403E, 0x0002, //  MOV.W  #2,R14
        0x8A0B, // SUB.W R10,R11
        // assert status of flags here, should have Z, C set
        0x9E0F, // CMP.W R14,R15
        // assert status of flags here, should have N
        0x8F0E, // SUB.W R15,R14
                // assert status of flags here, should have only C
    ];

    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));
    cpu_emu.run_one_instr();
    cpu_emu.run_one_instr();
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[2], 0x0003);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[2], 0x0004);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[2], 0x01);
    assert_eq!(cpu_emu.regs[14], 1);
}
