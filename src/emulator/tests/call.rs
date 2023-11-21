use crate::emulator::{tests::convert_words_to_bytes, Emulator};

#[test]
fn test_call_and_ret() {
    let instrs: Vec<u16> = vec![
        0x4031, 0x8000, //  MOV.W  #0x8000,SP
        0x12B0, 0x000A, //  CALL  0x0000A
        0x403F, 0x1101, //  MOV.W  #0x1101,R15
        0x4130, // RET
    ];
    let mut cpu_emu = Emulator::new(&convert_words_to_bytes(instrs));

    cpu_emu.run_one_instr();
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0x000C);
    assert_eq!(cpu_emu.regs[1], 0x7FFE);
    assert_eq!(cpu_emu.mem[0x7FFE], 0x06);
    assert_eq!(cpu_emu.mem[0x7FFF], 0x00);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0x0008);
    assert_eq!(cpu_emu.regs[1], 0x8000);
    cpu_emu.run_one_instr();
    assert_eq!(cpu_emu.regs[0], 0x000C);
    assert_eq!(cpu_emu.regs[1], 0x8000);
    assert_eq!(cpu_emu.regs[15], 0x1101);
}
