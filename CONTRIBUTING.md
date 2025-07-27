# Contributing to VimRust

Thank you for your interest in contributing to VimRust! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Git** - [Install Git](https://git-scm.com/)
- **Terminal with ANSI support**

### Development Setup
```bash
# Fork the repository on GitHub
git clone https://github.com/yourusername/VimRust.git
cd VimRust

# Build the project
cargo build

# Run tests
cargo test

# Run the editor
cargo run test.txt
```

## 📋 Development Workflow

### 1. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Code Style
We follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Check for linting issues
cargo clippy

# Run tests
cargo test
```

### 3. Commit Messages
Use clear, descriptive commit messages:

```
feat: add multi-cursor support
fix: resolve cursor positioning bug in visual mode
docs: update installation instructions
refactor: simplify buffer management logic
```

### 4. Pull Request
- Keep PRs focused and small
- Include tests for new features
- Update documentation as needed
- Ensure all CI checks pass

## 🏗️ Project Structure

```
VimRust/
├── src/
│   ├── main.rs          # Entry point
│   ├── editor.rs        # Main editor logic
│   ├── buffer.rs        # Text buffer management
│   ├── cursor.rs        # Cursor movement
│   ├── modes.rs         # Editor modes
│   ├── ui.rs           # Terminal UI
│   ├── syntax.rs       # Syntax highlighting
│   └── commands.rs     # Command system
├── tests/              # Integration tests
├── docs/               # Documentation
└── examples/           # Example configurations
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### Writing Tests
- Add unit tests in the same file as the code
- Add integration tests in the `tests/` directory
- Test both happy path and edge cases
- Use descriptive test names

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_move_right() {
        let mut cursor = Cursor::new();
        let content = Rope::from_str("hello world");
        cursor.move_right(&content);
        assert_eq!(cursor.col, 1);
    }
}
```

## 🎯 Areas for Contribution

### High Priority
- **LSP Integration** - Language server protocol support
- **Plugin System** - Extensible architecture
- **Performance Optimization** - Large file handling
- **Multi-cursor Support** - Edit multiple locations

### Medium Priority
- **Git Integration** - Status indicators, diff viewing
- **Fuzzy Finder** - File and symbol search
- **Split Windows** - Multiple panes
- **Configuration System** - User preferences

### Low Priority
- **Themes** - Color schemes
- **Documentation** - Tutorials and guides
- **Examples** - Sample configurations
- **Platform Support** - Windows/macOS specific features

## 📝 Documentation

### Code Documentation
- Use `///` for public APIs
- Include examples in doc comments
- Run `cargo doc --open` to view docs

### User Documentation
- Update `user-manual.md` for user-facing features
- Add examples to `README.md`
- Include screenshots for UI changes

## 🐛 Bug Reports

### Before Reporting
1. Check existing issues
2. Test with latest version
3. Reproduce with minimal example

### Bug Report Template
```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Open VimRust
2. Enter command '...'
3. See error

**Expected behavior**
What you expected to happen.

**Environment:**
- OS: [e.g. macOS 12.0]
- Rust version: [e.g. 1.70.0]
- VimRust version: [e.g. 0.1.0]

**Additional context**
Any other context about the problem.
```

## ✨ Feature Requests

### Feature Request Template
```markdown
**Is your feature request related to a problem?**
A clear description of what the problem is.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
Other solutions you've considered.

**Additional context**
Any other context or screenshots.
```

## 🎨 UI/UX Guidelines

### Terminal UI Principles
- **Responsive** - Work well at different terminal sizes
- **Accessible** - Support screen readers and high contrast
- **Fast** - Smooth rendering and interaction
- **Intuitive** - Follow vim conventions where applicable

### Color Usage
- Use semantic colors (error = red, success = green)
- Support both light and dark terminals
- Respect user's terminal color scheme

## 🔧 Architecture Guidelines

### Code Organization
- **Modular** - Each module has a single responsibility
- **Testable** - Easy to unit test components
- **Extensible** - Plugin-friendly architecture
- **Performance** - Efficient algorithms and data structures

### Dependencies
- Minimize external dependencies
- Prefer stable, well-maintained crates
- Document any new dependencies in PR

### Error Handling
- Use `Result<T, E>` for fallible operations
- Provide helpful error messages
- Use `anyhow` for error propagation
- Log errors appropriately

## 📊 Performance

### Benchmarks
```bash
# Run benchmarks
cargo bench

# Profile performance
cargo run --release -- large_file.txt
```

### Performance Guidelines
- **Large Files** - Must handle files > 100MB efficiently
- **Memory Usage** - Keep memory usage reasonable
- **Startup Time** - Fast startup for better UX
- **Responsiveness** - UI should remain responsive

## 🤝 Community

### Communication
- **GitHub Issues** - Bug reports and feature requests
- **Discussions** - General questions and ideas
- **Discord** - Real-time chat (link in README)

### Code of Conduct
- Be respectful and inclusive
- Help others learn and grow
- Focus on constructive feedback
- Maintain professionalism

## 🚀 Release Process

### Versioning
We follow [Semantic Versioning](https://semver.org/):
- **MAJOR** - Breaking changes
- **MINOR** - New features (backward compatible)
- **PATCH** - Bug fixes (backward compatible)

### Release Checklist
- [ ] Update CHANGELOG.md
- [ ] Update version in Cargo.toml
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Create GitHub release
- [ ] Publish to crates.io

## 📚 Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Editor Development
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Text Editor Tutorial](https://viewsourcecode.org/snaptoken/kilo/)
- [Vim Documentation](https://vimhelp.org/)

### Dependencies Used
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal handling
- [ropey](https://github.com/cessen/ropey) - Text manipulation
- [syntect](https://github.com/trishume/syntect) - Syntax highlighting

## 💡 Tips for Contributors

1. **Start Small** - Begin with documentation or small bug fixes
2. **Ask Questions** - Don't hesitate to ask in issues or discussions
3. **Read Code** - Understand existing patterns before adding new ones
4. **Test Thoroughly** - Include tests with your changes
5. **Be Patient** - Code review takes time for quality

Thank you for contributing to VimRust! 🦀✨