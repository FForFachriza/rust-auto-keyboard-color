use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    // Get File Confignya dulu
    // TODO: DIbikin toml maybe?
    let hex_file = File::open("/home/fforfachriza/.local/state/quickshell/user/generated/color.txt");
    let mut hex_codes: String = String::new();

    match hex_file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                hex_codes = line.unwrap().replace("#", "");
            }
        }
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
        }
    }

    let command = Command::new("asusctl")
        .arg("aura")
        .arg("static")
        .arg("-c")
        .arg(&hex_codes)
        .output()
        .expect("Failed to execute command");

    if command.status.success() {
        println!("Keyboard color changed successfully!");
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Hello, world! ");
}
