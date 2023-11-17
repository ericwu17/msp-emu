pub mod asm_line;
pub mod get_verbs;
pub mod source_cursor;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::process::Command;
use std::str;

const C_FILE_NAME: &str = "./main.c";
const GENERATED_ASM_NAME: &str = "./main.asm";

fn main() {
    let output = Command::new(
        "/Applications/ti/ccs1220/ccs/tools/compiler/ti-cgt-msp430_21.6.1.LTS/bin/cl430",
    )
    .args([
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

    let lines = get_verbs::get_tokens(asm_contents);
    for l in &lines {
        println!("{:?}", l);
    }
    println!("{} lines of asm", &lines.len());
}
