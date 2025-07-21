# Auto Keyboard (asusctl) Changer

A simple Rust-based utility that changes your ASUS laptop keyboard color using `asusctl`, by reading a hex color code from a text file.

Perfectly compatible with **end-4 Hyprland Quickhell dots** and laptops from the **ASUS TUF** or **ROG** series.

---

## ⚙️ Features

* Reads a hex color from a text file
* Applies the color using `asusctl`
* Works with systemd for automatic startup

---

## 🚀 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/FForFachriza/rust-auto-keyboard-color.git
cd rust-auto-keyboard-color
```

---

### 2. ✏️ Edit the Color File Path (Required)

Before building, you must update the hardcoded file path in the source code to match your system.

Open `src/main.rs` and locate this line:

```rust
let path = "/home/fforfachriza/.local/state/quickshell/user/generated/color.txt";
```

Replace it with your actual path to the hex color file, for example:

```rust
let path = "/home/your_username/.config/your_custom_path/color.txt";
```

Save the file after editing.

---

### 3. Build the Project

Once you've updated the file path, build the project with:

```bash
cargo build --release
```

---

### 4. Move the Executable

Move the compiled binary to a location of your choice (for example, inside your Hyprland config directory):

```bash
mv target/release/auto-keyboard-color ~/.config/hypr/
chmod +x ~/.config/hypr/auto-keyboard-color
```

---

## 🛠️ Set Up as a Systemd Service

To make it run automatically at startup:

### 1. Create the Service File

```bash
sudo nano /etc/systemd/system/keyboard-color.service
```

Paste the following (edit the paths and username):

```ini
[Unit]
Description=ROG Keyboard Color Changer
After=network.target

[Service]
ExecStart=/home/your_username/.config/hypr/auto-keyboard-color
Restart=always
User=your_username

[Install]
WantedBy=multi-user.target
```

> Replace `/home/your_username/...` and `User=your_username` with your actual username and the path to your binary.

---

### 2. Enable and Start the Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable keyboard-color.service
sudo systemctl start keyboard-color.service
```

---

## ✅ Verifying the Service

To check service status:

```bash
systemctl status keyboard-color.service
```

To view the live logs:

```bash
journalctl -u keyboard-color.service -f
```

---

## 📦 Notes

* Ensure `asusctl` is installed and can control your keyboard.
* This script assumes the color hex value is stored in a text file and updated by another tool (e.g., Quickhell).
* If the keyboard color doesn't change, verify your device is supported by `asusctl`, and that the file path is correct.
