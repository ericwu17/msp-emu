#![feature(bigint_helper_methods)]
pub mod asm_line;
pub mod byte_generator;
pub mod ccode;
pub mod emulator;
pub mod get_verbs;
pub mod operand;
pub mod source_cursor;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::process::Command;
use std::str;

use crate::byte_generator::generate_bytes;
use crate::emulator::Emulator;

const C_FILE_NAME: &str = "./main.c";
const GENERATED_ASM_NAME: &str = "./main.asm";

fn main() {
    let output = Command::new(
        "/Applications/ti/ccs1220/ccs/tools/compiler/ti-cgt-msp430_21.6.1.LTS/bin/cl430",
    )
    .args([
        // "--asm_listing",
        "--skip_assembler",
        "--symdebug:none",
        "--use_hw_mpy=none",
        "--opt_level=off",
    ])
    .arg(C_FILE_NAME)
    .output()
    .expect("failed to execute assembler process");

    println!("{}", unsafe { str::from_utf8_unchecked(&output.stderr) });
    if output.status.code() != Some(0) {
        println!("Compilation failed. Exiting.");
        exit(1);
    }

    let mut asm_contents = String::new();
    File::open(GENERATED_ASM_NAME)
        .expect(&format!("could not open file: {}", GENERATED_ASM_NAME))
        .read_to_string(&mut asm_contents)
        .expect(&format!("error reading file: {}", GENERATED_ASM_NAME));

    let (globals, lines) = get_verbs::get_tokens(asm_contents);

    let bytes = generate_bytes(globals, lines);
    println!("{:X?}", &bytes);
    let mut emulator = Emulator::new(&bytes);

    dbg!(&emulator);
    for _ in 0..10 {
        emulator.run_one_instr();
    }
}
