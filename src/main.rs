use crate::color_parser::{apply_color, color_parser};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};
mod color_parser;

fn main() -> Result<()> {
    const COLORS_PATH: &str =
        "/home/fforfachriza/.local/state/quickshell/user/generated/colors.json";

    // Watcher
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(COLORS_PATH), RecursiveMode::Recursive)?;

    println!("Watching for changes to {}...", COLORS_PATH);

    for res in rx {
        match res {
            Ok(event) => {
                if let notify::EventKind::Modify(_) = event.kind {
                    println!("\nFile changed! Applying new color...");
                    let hex_codes = color_parser(COLORS_PATH).expect("Error Parsing Color!");
                    apply_color(hex_codes).unwrap()
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
