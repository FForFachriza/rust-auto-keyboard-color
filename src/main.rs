use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    // Get File Confignya dulu
    // TODO: DIbikin toml maybe?
    let path = "/home/fforfachriza/.local/state/quickshell/user/generated/color.txt";
    apply_color(path);

    println!("Hello, world! ");
}

fn apply_color(path: &str) -> Result<(), String> {
    let hex_file = File::open(path);
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

    // Check if that code is actually hex, do u want to break ur keyboard dawg?
    if hex_codes.len() != 6 || !hex_codes.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("Invalid hex color: {}", hex_codes));
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
        eprintln!("Error: {}", String::from_utf8_lossy(&command.stderr));
    }

    Ok(())
}
