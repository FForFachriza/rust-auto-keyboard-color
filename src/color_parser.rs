use std::{fs::File, io::BufReader, process::Command};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ColorSettings {
    background: String,
    error: String,
    error_container: String,
    inverse_on_surface: String,
    inverse_primary: String,
    inverse_surface: String,
    on_background: String,
    on_error: String,
    on_error_container: String,
    on_primary: String,
    on_primary_container: String,
    on_primary_fixed: String,
    on_primary_fixed_variant: String,
    on_secondary: String,
    on_secondary_container: String,
    on_secondary_fixed: String,
    on_secondary_fixed_variant: String,
    on_surface: String,
    on_surface_variant: String,
    on_tertiary: String,
    on_tertiary_container: String,
    on_tertiary_fixed: String,
    on_tertiary_fixed_variant: String,
    outline: String,
    outline_variant: String,
    primary: String,
    primary_container: String,
    primary_fixed: String,
    primary_fixed_dim: String,
    scrim: String,
    secondary: String,
    secondary_container: String,
    secondary_fixed: String,
    secondary_fixed_dim: String,
    shadow: String,
    surface: String,
    surface_bright: String,
    surface_container: String,
    surface_container_high: String,
    surface_container_highest: String,
    surface_container_low: String,
    surface_container_lowest: String,
    surface_dim: String,
    surface_tint: String,
    surface_variant: String,
    tertiary: String,
    tertiary_container: String,
    tertiary_fixed: String,
    tertiary_fixed_dim: String,
}

pub fn color_parser<'a>(path: &str) -> Result<String, anyhow::Error> {
    let file = File::open(path).context("Failed to Open File")?;
    let reader = BufReader::new(file);
    let color: ColorSettings = serde_json::from_reader(reader).context("Failed To Read Json")?;

    Ok(color.primary.replace("#", ""))
}

pub fn apply_color(hex_codes: String) -> Result<(), anyhow::Error> {
    println!("Hex Codes: {}", hex_codes);

    // Check if that code is actually hex, do u want to break ur keyboard dawg?
    if hex_codes.len() != 6 || !hex_codes.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(notify::Error::generic("Invalid Hex Color").into());
    }

    let command = Command::new("asusctl")
        .arg("aura")
        .arg("static")
        .arg("-c")
        .arg(&hex_codes)
        .output()
        .expect("Failed to execute command");

    if command.status.success() {
        Ok(())
    } else {
        Err(anyhow::Error::msg("Error!"))
    }
}
