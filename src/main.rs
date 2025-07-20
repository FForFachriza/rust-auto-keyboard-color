use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use notify::{ErrorKind, Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

fn main()  -> Result<()> {
    // Get File Confignya dulu
    // TODO: DIbikin toml maybe?
    let path = "/home/fforfachriza/.local/state/quickshell/user/generated/color.txt";

    // Watcher
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    println!("Watching for changes to {}...", path);

    for res in rx {
        match res {
            Ok(event) => {
                if let notify::EventKind::Modify(_) = event.kind {
                    println!("\nFile changed! Applying new color...");
                    if let Err(e) = apply_color(path) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }


    Ok(())
}

fn apply_color(path: &str) -> Result<()> {
    let hex_file = File::open(path);
    let mut hex_codes: String = String::new();

    match hex_file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                hex_codes = line?.replace("#", "");
            }
        }
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
        }
    }

    // Check if that code is actually hex, do u want to break ur keyboard dawg?
    if hex_codes.len() != 6 || !hex_codes.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(notify::Error::generic("Invalid Hex COlor"));
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
