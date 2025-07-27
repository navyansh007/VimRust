# VimRust

A modern, vim-like text editor written in Rust with advanced features that go beyond traditional vim capabilities.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)

## 🚀 Features

### Core Vim-like Functionality
- **Modal editing** with Normal, Insert, Visual, and Command modes
- **Arrow key navigation** that works in both Normal and Insert modes
- **Advanced movement** with word (`w`, `b`, `e`) and line (`0`, `$`, `^`) navigation
- **File navigation** with `gg`, `G` for quick jumping

### Modern Enhancements
- **Rope-based text storage** for efficient handling of large files
- **Advanced undo/redo system** with 1000-operation history
- **Multiple buffer support** for editing multiple files simultaneously
- **Smart cursor positioning** that maintains column position across lines
- **Real-time syntax highlighting** framework ready for extension

### Advanced Editing
- **Multiple insert modes**: `i`, `a`, `A`, `I`, `o`, `O`
- **Powerful delete operations**: `x`, `dd`, `dw`, `d$`, `d0`, `D`
- **Flexible yank/copy**: `yy`, `yw`, `y$`, `Y`
- **Visual mode selection** with copy/delete operations
- **Search and replace**: Forward/backward search (`/`, `?`) with navigation (`n`, `N`)
- **Substitute commands**: `:s/pattern/replacement/flags`

### Command System
- **File operations**: `:w`, `:q`, `:wq`, `:q!`, `:e filename`
- **Buffer management**: `:bn`, `:bp`, `:bd` for multi-file editing
- **Settings control**: `:set number`, `:set syntax` for customization
- **Interactive command line** with real-time feedback

## 📦 Installation

### Prerequisites
- **Rust 1.70 or higher** - [Install Rust](https://rustup.rs/)
- **Terminal with ANSI support** - Most modern terminals work

### Building from Source
```bash
git clone https://github.com/navyansh007/VimRust.git
cd VimRust
cargo build --release
```

### Running
```bash
# Open a new file
cargo run

# Open an existing file
cargo run filename.txt

# Use the compiled binary
./target/release/vimrust filename.txt
```

## 🎯 Quick Start

1. **Open VimRust**: `cargo run test.txt`
2. **Start in Normal mode** - use arrow keys to navigate
3. **Enter Insert mode** - press `i` to start typing
4. **Navigate while typing** - arrow keys work in Insert mode!
5. **Save your work** - press `Esc` then `:w`
6. **Exit** - type `:q` to quit

## 📋 Key Bindings

### Navigation (Works in Normal and Insert modes)
- `↑↓←→` - Basic movement
- `w`, `b`, `e` - Word movement
- `0`, `$`, `^` - Line movement
- `gg`, `G` - File movement
- `Home`, `End` - Line start/end (Insert mode)
- `Page Up/Down` - Page navigation (Insert mode)

### Editing
- `i`, `a`, `A`, `I`, `o`, `O` - Enter Insert mode
- `x` - Delete character
- `dd` - Delete line
- `dw` - Delete word
- `yy` - Copy line
- `p`, `P` - Paste
- `u` - Undo
- `Ctrl+r` - Redo

### Visual Mode
- `v` - Enter Visual mode
- Move with arrow keys to select
- `y` - Copy selection
- `x` - Delete selection

### Commands
- `:w` - Save file
- `:q` - Quit
- `:wq` - Save and quit
- `:q!` - Force quit without saving
- `/text` - Search forward
- `?text` - Search backward
- `:s/old/new/g` - Replace text

## 🔧 Configuration

VimRust supports runtime configuration through commands:

```vim
:set number      " Show line numbers
:set nonumber    " Hide line numbers
:set syntax      " Enable syntax highlighting
:set nosyntax    " Disable syntax highlighting
```

## 🏗️ Architecture

VimRust is built with a modular architecture:

- **`editor.rs`** - Main editor logic and event handling
- **`buffer.rs`** - Text buffer management with Rope data structure
- **`cursor.rs`** - Advanced cursor movement and positioning
- **`ui.rs`** - Terminal UI rendering with ratatui
- **`modes.rs`** - Editor mode management
- **`syntax.rs`** - Syntax highlighting framework
- **`commands.rs`** - Extensible command system

## 🚧 Roadmap

### Planned Features
- [ ] **LSP Integration** - Code completion, error highlighting, go-to-definition
- [ ] **Git Integration** - Status indicators, diff viewing, blame annotations
- [ ] **Multiple Cursors** - Edit multiple locations simultaneously
- [ ] **Fuzzy File Finder** - Quick file navigation and project search
- [ ] **Plugin System** - Custom commands and extensions
- [ ] **Configuration Files** - `.vimrustrc` for persistent settings
- [ ] **Themes** - Customizable color schemes
- [ ] **Split Windows** - Multiple views of the same or different files

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup
```bash
git clone https://github.com/navyansh007/VimRust.git
cd VimRust
cargo build
cargo test
cargo run -- test.txt
```

### Running Tests
```bash
cargo test
cargo clippy
cargo fmt
```

## 📝 Documentation

- [User Manual](user-manual.md) - Comprehensive guide to all features
- [API Documentation](https://docs.rs/vimrust) - Code documentation
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Changelog](CHANGELOG.md) - Version history

## 🐛 Troubleshooting

### Common Issues

**"Device not configured" Error**
- This occurs in non-interactive terminals
- Run in a proper terminal environment

**Performance with Large Files**
- VimRust uses efficient Rope data structure
- Disable syntax highlighting for very large files: `:set nosyntax`

**Build Issues**
- Ensure Rust 1.70+ is installed
- Try `cargo clean && cargo build`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by **Vim** and **Neovim** for the editing paradigm
- Built with **Rust** for performance and safety
- Uses **ratatui** for terminal UI rendering
- **Ropey** for efficient text manipulation
- **Syntect** for syntax highlighting

## 📊 Project Status

VimRust is in active development. Current version: **0.1.0**

- ✅ Core vim functionality implemented
- ✅ Modern enhancements working
- ✅ Arrow key navigation in all modes
- 🚧 LSP integration in progress
- 🚧 Plugin system planned

---

**Made with ❤️ and Rust**

For more information, visit our [documentation](user-manual.md) or [open an issue](https://github.com/navyansh007/VimRust/issues).
