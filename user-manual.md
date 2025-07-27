# VimRust User Manual

A modern vim-like text editor written in Rust with advanced features that go beyond traditional vim capabilities.

## Table of Contents
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Editor Modes](#editor-modes)
- [Navigation](#navigation)
- [Text Editing](#text-editing)
- [File Operations](#file-operations)
- [Command Reference](#command-reference)
- [Advanced Features](#advanced-features)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Installation

### Prerequisites
- Rust 1.70 or higher
- A terminal that supports ANSI escape sequences

### Building from Source
```bash
git clone <repository-url>
cd VimRust
cargo build --release
```

### Running
```bash
# Open a new file
cargo run

# Open an existing file
cargo run filename.txt

# Or use the compiled binary
./target/release/vimrust filename.txt
```

## Getting Started

VimRust follows vim's modal editing philosophy but with modern enhancements. When you first open the editor, you'll be in **Normal mode**.

### Basic Workflow
1. **Normal mode**: Navigate and issue commands
2. **Insert mode**: Type and edit text
3. **Visual mode**: Select text
4. **Command mode**: Execute file operations and settings

### Your First Edit
1. Open a file: `cargo run test.txt`
2. Press `i` to enter Insert mode
3. Type some text
4. Press `Esc` to return to Normal mode
5. Type `:w` to save
6. Type `:q` to quit

## Editor Modes

### Normal Mode (Default)
The default mode for navigation and commands. The status line shows `NORMAL`.

**Key Features:**
- Cursor appears as a block
- All navigation commands work
- Text editing commands (delete, yank, paste)
- Mode switching commands

### Insert Mode
For typing and editing text. The status line shows `INSERT`.

**Entering Insert Mode:**
- `i` - Insert before cursor
- `a` - Insert after cursor
- `I` - Insert at beginning of line
- `A` - Insert at end of line
- `o` - Open new line below
- `O` - Open new line above

**Navigation in Insert Mode:**
- `Arrow Keys` - Move cursor while typing
- `Home` - Move to beginning of line
- `End` - Move to end of line
- `Page Up/Down` - Navigate by pages

**Exiting Insert Mode:**
- `Esc` - Return to Normal mode

### Visual Mode
For selecting text. The status line shows `VISUAL`.

**Entering Visual Mode:**
- `v` - Character-wise selection
- `V` - Line-wise selection

**In Visual Mode:**
- Use navigation keys to extend selection
- `y` - Yank (copy) selection
- `d` - Delete selection
- `Esc` - Return to Normal mode

### Command Mode
For executing commands. The status line shows `COMMAND`.

**Entering Command Mode:**
- `:` - Opens command line with `:` prompt

**Exiting Command Mode:**
- `Esc` - Return to Normal mode
- `Enter` - Execute command and return to Normal mode

## Navigation

### Basic Movement
| Key | Action |
|-----|--------|
| `←` | Move left |
| `↓` | Move down |
| `↑` | Move up |
| `→` | Move right |

### Word Movement
| Key | Action |
|-----|--------|
| `w` | Move to next word |
| `b` | Move to previous word |
| `e` | Move to end of word |

### Line Movement
| Key | Action |
|-----|--------|
| `0` | Move to beginning of line |
| `$` | Move to end of line |
| `^` | Move to first non-whitespace character |

### File Movement
| Key | Action |
|-----|--------|
| `gg` | Go to first line |
| `G` | Go to last line |
| `{number}G` | Go to line number |

### Screen Movement
| Key | Action |
|-----|--------|
| `Ctrl+f` | Page down |
| `Ctrl+b` | Page up |
| `Ctrl+d` | Half page down |
| `Ctrl+u` | Half page up |

## Text Editing

### Insertion
| Key | Action |
|-----|--------|
| `i` | Insert before cursor |
| `a` | Insert after cursor |
| `I` | Insert at line beginning |
| `A` | Insert at line end |
| `o` | Open line below |
| `O` | Open line above |

### Deletion
| Key | Action |
|-----|--------|
| `x` | Delete character under cursor |
| `X` | Delete character before cursor |
| `dd` | Delete entire line |
| `dw` | Delete word |
| `d$` | Delete to end of line |
| `d0` | Delete to beginning of line |

### Copy and Paste
| Key | Action |
|-----|--------|
| `yy` | Yank (copy) line |
| `yw` | Yank word |
| `y$` | Yank to end of line |
| `p` | Paste after cursor |
| `P` | Paste before cursor |

### Undo and Redo
| Key | Action |
|-----|--------|
| `u` | Undo last change |
| `Ctrl+r` | Redo last undone change |

## File Operations

### Opening Files
```bash
# From command line
vimrust filename.txt

# From within editor
:e filename.txt
:edit filename.txt
```

### Saving Files
```
:w              # Save current file
:w filename     # Save as filename
:write          # Same as :w
```

### Quitting
```
:q              # Quit (fails if unsaved changes)
:q!             # Force quit (discard changes)
:wq             # Save and quit
:x              # Save and quit (same as :wq)
```

### Buffer Operations
```
:bn             # Next buffer
:bp             # Previous buffer
:bd             # Delete current buffer
```

## Command Reference

### File Commands
| Command | Description |
|---------|-------------|
| `:w` | Write (save) file |
| `:w filename` | Save as filename |
| `:q` | Quit |
| `:q!` | Force quit |
| `:wq` | Save and quit |
| `:x` | Save and quit |
| `:e filename` | Edit file |

### Settings Commands
| Command | Description |
|---------|-------------|
| `:set number` | Show line numbers |
| `:set nonumber` | Hide line numbers |
| `:set syntax` | Enable syntax highlighting |
| `:set nosyntax` | Disable syntax highlighting |

### Search Commands
| Command | Description |
|---------|-------------|
| `/pattern` | Search forward |
| `?pattern` | Search backward |
| `n` | Next search result |
| `N` | Previous search result |

### Substitution Commands
| Command | Description |
|---------|-------------|
| `:s/old/new` | Replace first occurrence in line |
| `:s/old/new/g` | Replace all occurrences in line |
| `:%s/old/new/g` | Replace all occurrences in file |

## Advanced Features

### Syntax Highlighting
VimRust includes advanced syntax highlighting powered by the `syntect` library.

**Supported Languages:**
- Rust
- Python
- JavaScript/TypeScript
- C/C++
- Java
- And many more...

**Theme Support:**
- Multiple built-in color schemes
- `:set theme <name>` to change themes

### Undo/Redo System
Unlike traditional vim, VimRust uses a modern undo system:

- **Persistent History**: Up to 1000 operations
- **Branching Undo**: Non-linear undo tree
- **Memory Efficient**: Uses rope data structure

### Buffer Management
Advanced buffer handling with:

- **Multiple Files**: Open multiple files simultaneously
- **Buffer Navigation**: Switch between files easily
- **Modified Indicators**: Visual indication of unsaved changes

### Modern Text Handling
- **Large File Support**: Efficient handling of large files using rope data structure
- **Unicode Support**: Full UTF-8 support
- **Smart Indentation**: Context-aware indentation

## Configuration

### Settings
VimRust supports various settings that can be toggled:

```
:set number        # Show line numbers
:set nonumber      # Hide line numbers
:set syntax        # Enable syntax highlighting
:set nosyntax      # Disable syntax highlighting
```

### Future Configuration
The editor is designed to support:
- Configuration files (`.vimrustrc`)
- Custom key bindings
- Plugin system
- Theme customization

## User Interface

### Status Line
The status line shows:
- Current mode (NORMAL, INSERT, VISUAL, COMMAND)
- File name
- Modified indicator ([+] if unsaved changes)
- Line count
- Cursor position (line:column)

### Command Line
- Shows `:` prompt in command mode
- Displays error messages
- Shows search patterns

### Line Numbers
- Optional line numbers on the left
- Toggle with `:set number` / `:set nonumber`

## Keyboard Shortcuts

### Global Shortcuts
| Key Combination | Action |
|----------------|--------|
| `Ctrl+C` | Interrupt operation |
| `Ctrl+Z` | Suspend editor (Unix) |

### Mode-Specific Shortcuts

#### Normal Mode
| Key | Action |
|-----|--------|
| `Arrow Keys` | Navigation |
| `w`, `b`, `e` | Word movement |
| `0`, `$`, `^` | Line movement |
| `gg`, `G` | File movement |
| `i`, `a`, `A`, `o`, `O` | Enter insert mode |
| `v` | Enter visual mode |
| `:`, `/`, `?` | Enter command mode |
| `u` | Undo |
| `Ctrl+r` | Redo |
| `x`, `dd`, `dw`, `d$` | Delete operations |
| `yy`, `yw`, `y$` | Copy operations |
| `p`, `P` | Paste operations |

#### Insert Mode
| Key | Action |
|-----|--------|
| `Esc` | Return to normal mode |
| `Arrow Keys` | Navigate while in insert mode |
| `Home` | Move to beginning of line |
| `End` | Move to end of line |
| `Page Up/Down` | Navigate by pages |
| `Backspace` | Delete previous character |
| `Delete` | Delete current character |
| `Enter` | New line |
| `Tab` | Insert tab |

## Troubleshooting

### Common Issues

#### "Device not configured" Error
This occurs when running in a non-interactive terminal.
**Solution**: Run in a proper terminal environment.

#### Editor Won't Start
**Possible causes:**
- Missing dependencies
- Incompatible terminal
- Insufficient permissions

**Solutions:**
1. Ensure Rust is properly installed
2. Run `cargo build` to check for compilation errors
3. Try running in a different terminal

#### File Won't Save
**Possible causes:**
- Read-only file
- Insufficient permissions
- Disk space issues

**Solutions:**
1. Check file permissions
2. Try saving to a different location
3. Use `:w!` to force save

#### Performance Issues
**For large files:**
- VimRust uses rope data structure for efficiency
- Consider splitting very large files
- Disable syntax highlighting for huge files: `:set nosyntax`

### Getting Help

#### In-Editor Help
```
:help           # Show help information
:version        # Show version information
```

#### Debug Mode
Run with debug output:
```bash
RUST_LOG=debug cargo run filename.txt
```

### Reporting Issues
When reporting issues, include:
- VimRust version
- Operating system
- Terminal type
- Steps to reproduce
- Error messages

## Tips and Best Practices

### Efficiency Tips
1. **Learn the motion commands**: `w`, `b`, `e` for word movement
2. **Use line navigation**: `0`, `$` for quick line movement
3. **Master copy/paste**: `yy`, `p` for line operations
4. **Use undo effectively**: `u` to undo, `Ctrl+r` to redo

### File Management
1. **Save frequently**: Use `:w` regularly
2. **Use meaningful filenames**: Helps with buffer navigation
3. **Organize files**: Keep related files in the same directory

### Editing Workflow
1. **Plan before editing**: Think about what you want to change
2. **Use visual mode**: For precise selections
3. **Leverage search**: Find text quickly with `/`
4. **Practice commands**: The more you use them, the faster you become

## Future Features

VimRust is designed to be extensible. Planned features include:

### Language Server Protocol (LSP)
- Code completion
- Error highlighting
- Go to definition
- Symbol search

### Git Integration
- Git status in status line
- Diff viewing
- Blame annotations
- Conflict resolution

### Multiple Cursors
- Edit multiple locations simultaneously
- Advanced selection modes
- Parallel editing operations

### Fuzzy File Finder
- Quick file navigation
- Project-wide search
- Recent files list

### Plugin System
- Custom commands
- Syntax extensions
- Theme development
- Key binding customization

## Appendix

### Default Key Bindings Summary

#### Navigation
```
Arrow Keys     - Basic movement
w, b, e        - Word movement
0, $, ^        - Line movement
gg, G          - File movement
Ctrl+f/b       - Page movement
Ctrl+d/u       - Half-page movement
```

#### Editing
```
i, a, I, A, o, O  - Insert modes
x, X              - Delete character
dd, dw, d$, d0    - Delete operations
yy, yw, y$        - Copy operations
p, P              - Paste operations
u, Ctrl+r         - Undo/redo
```

#### Modes
```
Esc            - Normal mode
i              - Insert mode
v              - Visual mode
:              - Command mode
```

#### Commands
```
:w             - Save
:q             - Quit
:wq, :x        - Save and quit
:e filename    - Open file
:set option    - Change setting
```

---

**Version**: 0.1.0  
**Last Updated**: 2025
**License**: MIT  

For more information, visit the project repository or contact me at navyanshkesarwani@gmail.com