# asm2hex

> **A Modern, Multi-Arch, Flat-Binary Assembly → Intel HEX Converter**

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Rust Stable](https://img.shields.io/badge/Rust-Stable-blue.svg)](https://www.rust-lang.org/tools/install)
[![Platform](https://img.shields.io/badge/Platform-Windows%20|%20Linux%20|%20macOS-lightgrey)](#)

`asm2hex` is a **cross-platform**, **multi-threaded** tool that converts **assembly (`.asm`)** files into **Intel HEX (`.hex`)** format using a **flat-binary** approach (`nasm -f bin` + `objcopy -I binary -O ihex`). It features a **modern GUI** (via **`egui`** & **`eframe`**), **auto-insert `[bits X]`**, **color-coded logs**, and **multi-file concurrency**.

---

## **Table of Contents**

1. [Features](#features)
2. [Installation](#installation)
3. [How to Build](#how-to-build)
4. [How to Run](#how-to-run)
5. [Usage](#usage)
6. [Example Assembly](#example-assembly)
7. [FAQ](#faq)
8. [Contributing](#contributing)
9. [License](#license)

---

## **Features**

- **Flat-Binary** (no OS-specific linking)
- **Bits Mode** (16/32/64)
- **Auto Insert `[bits X]`** if missing
- **Multi-File** concurrency + real-time progress bar
- **Color-Coded Logs** (Errors, Warnings, Success)
- **Full `.hex` Preview** (no line cutoff)
- **Dark/Light Theme** toggle
- **Cross-Platform** (Windows, Linux, macOS)

---

## **Installation**

**1) Install Rust**

- [Official Rust Installer](https://www.rust-lang.org/tools/install)

**2) Install `nasm` + `objcopy`**

- **Windows**: `scoop install nasm binutils` (or Chocolatey)
- **Linux (Debian/Ubuntu)**: `sudo apt-get install nasm binutils`
- **macOS**: `brew install nasm binutils`

---

## How to Build

```sh
git clone https://github.com/YourGitHubUser/asm2hex.git
cd asm2hex
cargo build --release
```

``

This creates a **binary** in:

```
target/release/asm2hex(.exe)
```

---

## How to run

You can **run** this tool in **two** ways:

### **1) Direct GUI Execution**

```sh
# Windows:
target\release\asm2hex.exe

# Linux/Mac:
./target/release/asm2hex
```

A **GUI window** opens. From there, you add `.asm` files, choose bits mode, select output folder, and click **"Convert to HEX"**.

### **2) CLI Execution (Optional)**

Currently, this tool is primarily **GUI-based**, but if you'd like to run it in a terminal (for testing or scripts):

```sh
# (Not actively maintained, but you could pass arguments if implemented.)
# Example:
target\release\asm2hex --help
```

_(If you want a full CLI, you'd add argument handling in `main.rs`. By default, the code runs as a GUI.)_

---

## **Usage**

1. **Launch the Tool**: Double-click the `.exe` (on Windows) or run `./asm2hex` on Linux/macOS.
2. **Add `.asm` Files**: Click "**Add .asm Files**" in the side panel.
3. **Choose Bits Mode**: 16, 32, or 64.
4. **Check "Auto-insert [bits X]"** if your `.asm` lacks a `[bits ...]` line.
5. **Select Output Folder**: Where `.bin` & `.hex` will be saved.
6. **Convert**: Click "**⚡ Convert to HEX**". See:
   - **Progress Bar** for multi-file concurrency.
   - **Color-coded Logs** for errors or success messages.
   - **Full `.hex`** content in the **central panel**.

---

## **Example Assembly**

Below is a sample 64-bit code (`hello.asm`):

```asm
[bits 64]
section .text
global _start

_start:
    mov rax, 1        ; sys_write
    mov rdi, 1        ; stdout
    mov rsi, message
    mov rdx, message_len
    syscall

    mov rax, 60       ; sys_exit
    xor rdi, rdi
    syscall

section .data
message db "Hello, asm2hex!", 0xA
message_len equ $ - message
```

After building, **Add** `hello.asm` in the GUI, pick 64-bit, choose an **output folder**, and click "**Convert to HEX**". The logs panel will show:

```
🔍 Processing hello.asm
✅ Success! HEX saved at ...
```

The central panel displays the entire Intel HEX output.

---

## FAQ

1. **Does it support ARM instructions?**

   - Yes, as long as `nasm` can assemble them in flat-binary mode. Typically x86/AMD64 is best supported.

2. **Why no OS Linking?**

   - We use `-f bin` for a **pure** flat-binary approach. No PE/ELF, so no high memory addresses or out-of-range issues.

3. **Where do the `.bin` and `.hex` files go?**

   - Inside the **output folder** you chose in the GUI side panel.

4. **Can I run it headlessly?**
   - Currently, we provide a GUI. If you want a pure CLI, you can adapt `main.rs` to parse arguments.

---

## **Contributing**

We welcome PRs for:

- **New Features** (CLI mode, custom linker flags, advanced logs, etc.)
- **Bug Fixes** or **UI Enhancements**

1. Fork the repo and create a branch.
2. Commit & push changes.
3. Submit a pull request with details.

---

## **License**

This project is released under the [MIT License](LICENSE).

Enjoy a frictionless **Assembly → HEX** workflow! Contributions & feedback are welcome.
