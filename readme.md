# Auto Keyboard (asusctl) Changer

A simple Rust-based utility to change your ASUS laptop keyboard color using `asusctl`, based on a hex value defined in a file.

This tool works perfectly with **end-4 Hyprland Quickhell dots** and is compatible with ASUS TUF or ROG laptops.

---

## ⚙️ Features

* Reads a hex color from a file
* Applies the color to your keyboard using `asusctl`
* Lightweight and systemd-compatible for running at boot

---

## 🚀 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/FForFachriza/rust-auto-keyboard-color.git
cd rust-auto-keyboard-color
```

### 2. Build the Project

Make sure you have Rust installed. Then build the project:

```bash
cargo build --release
```

### 3. Move the Executable

After building, move the compiled binary to your preferred location (e.g., `~/.config/hypr`):

```bash
mv target/release/auto-keyboard-color ~/.config/hypr/
chmod +x ~/.config/hypr/auto-keyboard-color
```

---

## 🛠️ Set Up as a Systemd Service

To run the program automatically at boot:

### 1. Create the Systemd Service File

```bash
sudo nano /etc/systemd/system/keyboard-color.service
```

Paste the following content **and replace** the path and username accordingly:

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

> 🔧 Replace `/home/your_username/...` and `User=your_username` with your actual username and the path where you placed the binary.

---

### 2. Enable and Start the Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable keyboard-color.service
sudo systemctl start keyboard-color.service
```

---

## ✅ Verify It's Working

To check if the service is running correctly:

```bash
systemctl status keyboard-color.service
```

To view live logs:

```bash
journalctl -u keyboard-color.service -f
```

---

## 📦 Notes

* Make sure `asusctl` is installed and functioning on your system.
* Ensure you have the appropriate permissions to control keyboard lighting.

