#![feature(bigint_helper_methods)]
pub mod asm_line;
pub mod byte_generator;
pub mod ccode;
pub mod emulator;
pub mod get_verbs;
pub mod graphics;
pub mod operand;
pub mod source_cursor;

use graphics::{draw_leds, draw_monitor, draw_switches, get_curr_button_states};
use macroquad::prelude::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::process::Command;
use std::str;

use crate::byte_generator::generate_bytes;
use crate::emulator::Emulator;

const C_FILE_NAME: &str = "./main.c";
const GENERATED_ASM_NAME: &str = "./main.asm";
const OUTPUT_FILE_NAME: &str = "seq.code";

#[macroquad::main("Assembler Emulator")]
async fn main() {
    Command::new("/Applications/ti/ccs1220/ccs/tools/compiler/ti-cgt-msp430_21.6.1.LTS/bin/cl430")
        .args([
            "--asm_listing",
            "--symdebug:none",
            "--use_hw_mpy=none",
            "--opt_level=off",
        ])
        .arg(C_FILE_NAME)
        .output()
        .expect("failed to execute assembler process");
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

    let (globals, lines) = get_verbs::get_tokens(asm_contents);

    let bytes = generate_bytes(globals, lines);
    write_bytes_to_file(&bytes);
    println!("Wrote {} bytes to file {}", bytes.len(), OUTPUT_FILE_NAME);
    let mut emulator = Emulator::new(&bytes);

    let mut curr_switch_states = 0u16;

    loop {
        emulator.run_some_instrs();
        clear_background(LIGHTGRAY);

        let gfx_buf = emulator.get_gfx_buffer();

        draw_monitor(10.0, 10.0, 640.0, 480.0, gfx_buf).await;

        draw_leds(10.0, 500.0, emulator.get_led_output()).await;
        draw_switches(10.0, 520.0, &mut curr_switch_states).await;
        emulator.set_switch_states(curr_switch_states);
        emulator.set_button_states(get_curr_button_states().await);

        next_frame().await;
    }
}

fn write_bytes_to_file(bytes: &Vec<u8>) {
    let mut f = File::create(OUTPUT_FILE_NAME).expect("error creating output file.");
    for byte in bytes {
        f.write(format!("{:0>2X}\n", byte).as_bytes())
            .expect("error writing to output file");
    }
}
