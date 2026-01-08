<p align="center">
  <img src="https://img.shields.io/badge/-shellscope-orange?style=for-the-badge&logo=rust&logoColor=white" alt="shellscope"/>
</p>

---

<p align="center">
  🦀 Lightweight and minimal CLI wrapper for structured command execution.
</p>

<p align="center">
  <a href="https://github.com/INetrois/shellscope/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/INetrois/shellscope/rust.yml?branch=main&label=CI&style=flat-square" alt="CI">
  </a>
  <a href="https://github.com/INetrois/shellscope/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/INetrois/shellscope?style=flat-square" alt="License">
  </a>
</p>

## 🔹 Features

- Wraps and executes any shell command
- Clean and predictable command execution
- Minimal overhead
- UNIX-style wrapper (similar to `time`)
- No flags, no configuration

## 💻 Installation

### Cargo (recommended)

```bash
cargo install --git https://github.com/INetrois/shellscope
```
### Arch Linux / Manjaro
```bash
git clone https://github.com/INetrois/shellscope
cd shellscope
makepkg -si
```

## 🚀 Usage
> All arguments are forwarded directly to the wrapped command.
```
shellscope cargo build
```

<p align="center"> Made with ❤️ by <a href="https://github.com/INetrois">INetrois</a> </p> 
