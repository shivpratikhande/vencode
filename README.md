# 🎥 Vencode

**Vencode** is a lightweight, cross-interface (CLI + GUI) screen recording tool for Linux, written in Rust. It uses `ffmpeg` under the hood to capture both video and audio, offering simple controls via terminal commands or a minimal desktop GUI.

---

## ✨ Features

- 🖥️ Record your screen with system audio using `ffmpeg`
- 🧵 Built-in CLI interface using `clap`
- 🪟 Simple and responsive GUI built with `egui` and `eframe`
- 🎞️ Adjustable frame rate: 30, 60, or 120 FPS
- 🔴 Start / ⏹ Stop recording with real-time status

---

## 🚀 Getting Started

### 🔧 Prerequisites

- Linux with `ffmpeg` installed
- PulseAudio (for audio capture)
- Rust toolchain (`cargo`, `rustc`)

> Ensure the `ffmpeg` binary is available in your system path.

---

### 📦 Installation

```bash
git clone https://github.com/shivpratikhande/vencode
cd vencode
cargo build --release
```

# Vencode

## 🖥️ Usage

### 🛠 CLI Commands

```bash
./vencode start  # Start recording at 30 FPS
./vencode stop   # Stop recording and save the file
./vencode init   # Launch GUI mode
```

### 💻 GUI Mode

```bash
./vencode init
```

- Select frame rate
- Click to start or stop recording
- View status or error messages in the UI

## 🛠 Tech Stack

- 🦀 **Rust**
- 🎛️ **Clap** - CLI argument parser
- 🎨 **egui + eframe** - GUI rendering
- 📼 **ffmpeg** - screen/audio recording backend

## 📂 Output

Recordings are saved in the working directory as:

```
recording.mp4
recording_1.mp4
recording_2.mp4
...
```

## ⚠️ Notes

- Uses a fixed PulseAudio monitor device:
  ```rust
  let audio_device = "bluez_output.C1_2E_50_DF_DF_48.1.monitor";
  ```
  Update this string if your audio setup differs.

- PID for ffmpeg is stored in `/tmp/vencode_ffmpeg.pid`

## 🧑‍💻 Author

Built by **Shivpratikhande**

Contributions welcome via issues or PRs!

## 📄 License

MIT License. See LICENSE file for details.


