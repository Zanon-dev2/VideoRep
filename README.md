# 🎬 VideoRep

A simple command-line video player built in Rust as a hands-on learning project.

## 📖 Overview

**VideoRep** is an experimental media launcher created while studying Rust. It allows you to list and play pre-configured local or web-hosted videos directly from the terminal.

## ⚙️ Requirements

* **Rust & Cargo:** Make sure you have the Rust toolchain installed.
* **Operating System:** Recommended on **Linux** (including **WSL** on Windows).

## 🚀 How to Use

1. **Clone the repository:**
   ```bash
   git clone https://github.com
   cd VideoRep
   ```

2. **Configure your video paths:**
   Before running the project, update the hardcoded file directories in the source code to match the paths on your machine.

3. **Run the application:**
   ```bash
   cargo run
   ```

4. **List available videos:**
   When prompted in the terminal, type `list` to view all configured videos. The project comes pre-configured with 4 sample entries used for testing.

## ➕ Adding Your Own Videos

To add new videos to the player, update the `match` block in the source code following these steps:

1. Add your video's trigger name to the `match` statement.
2. Set its availability flag to `true`.
3. Assign the path to `*WaY_nya` ending with `.to_string()`.

### Path Formatting Rules
* **Local files:** Must start with `file:///` (e.g., `file:///home/user/videos/my%20video.mp4`).
* **Web URLs:** Must start with `https://` (e.g., `https://example.com`).
* **No spaces allowed:** Replace any spaces (` `) in your path or filename with `%20`.

## 📄 License

This project is open-source and available for educational and personal use.

---
Developed by [Zanon-dev2](https://github.com/Zanon-dev2).
