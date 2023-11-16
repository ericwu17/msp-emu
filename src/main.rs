use std::process::Command;

const C_FILE_NAME: &str = "./main.c";

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

    if output.status.code() != Some(0) {
        dbg!(&output);
        panic!(
            "compiler processed exited with code {:?}",
            output.status.code()
        )
    }

    println!("Hello, world!");
}
