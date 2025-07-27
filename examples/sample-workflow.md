# VimRust Sample Workflow

This document demonstrates common workflows using VimRust.

## Basic Editing Workflow

### 1. Opening Files
```bash
# Open a new file
vimrust

# Open an existing file
vimrust filename.txt

# Open multiple files
vimrust file1.txt file2.txt file3.txt
```

### 2. Navigation
```
# Basic movement (works in Normal and Insert mode)
↑↓←→  - Arrow keys for movement
Home  - Beginning of line
End   - End of line
PgUp  - Page up
PgDn  - Page down

# Word movement (Normal mode)
w     - Next word
b     - Previous word  
e     - End of word

# Line movement (Normal mode)
0     - Beginning of line
$     - End of line
^     - First non-whitespace character

# File movement (Normal mode)
gg    - Top of file
G     - End of file
```

### 3. Editing Text
```
# Entering Insert mode
i     - Insert before cursor
a     - Insert after cursor
A     - Insert at end of line
I     - Insert at beginning of line
o     - Open line below
O     - Open line above

# In Insert mode
Type normally, arrow keys work for navigation
Esc   - Return to Normal mode

# Deleting text (Normal mode)
x     - Delete character
dd    - Delete line
dw    - Delete word
d$    - Delete to end of line
d0    - Delete to beginning of line

# Copying text (Normal mode)
yy    - Copy line
yw    - Copy word
y$    - Copy to end of line

# Pasting text (Normal mode)
p     - Paste after cursor
P     - Paste before cursor
```

### 4. Visual Mode
```
v     - Enter Visual mode
# Use arrow keys to select text
y     - Copy selection
x     - Delete selection
Esc   - Exit Visual mode
```

### 5. Search and Replace
```
/text      - Search forward for "text"
?text      - Search backward for "text"
n          - Next search result
N          - Previous search result

:s/old/new      - Replace first occurrence in line
:s/old/new/g    - Replace all occurrences in line
:%s/old/new/g   - Replace all occurrences in file
```

### 6. File Operations
```
:w         - Save file
:w filename - Save as filename
:q         - Quit
:q!        - Quit without saving
:wq        - Save and quit
:x         - Save and quit (same as :wq)
```

### 7. Buffer Management
```
:e filename - Open new file
:bn         - Next buffer
:bp         - Previous buffer
:bd         - Delete current buffer
```

## Advanced Workflows

### Code Editing
1. Open source file: `vimrust main.rs`
2. Navigate to function: `/function_name`
3. Edit function: `i` to enter Insert mode
4. Navigate while editing using arrow keys
5. Save: `Esc` then `:w`

### Multi-file Editing
1. Open multiple files: `vimrust *.rs`
2. Switch between buffers: `:bn` and `:bp`
3. Edit each file as needed
4. Save all: `:w` in each buffer

### Search and Replace Workflow
1. Search for pattern: `/old_name`
2. Navigate through results: `n` and `N`
3. Replace all: `:%s/old_name/new_name/g`
4. Verify changes and save: `:w`

### Large File Editing
1. Open large file: `vimrust large_file.txt`
2. Disable syntax highlighting: `:set nosyntax`
3. Navigate efficiently: `gg`, `G`, `/search_term`
4. Edit specific sections
5. Save: `:w`

## Tips and Tricks

### Productivity Tips
- Use arrow keys in Insert mode for quick navigation
- Learn word movement (`w`, `b`, `e`) for faster navigation
- Use Visual mode for precise text selection
- Combine commands: `dw` (delete word), `yw` (yank word)

### Common Patterns
```
# Quick line duplication
yy p    - Copy current line and paste below

# Quick word replacement
# 1. Position cursor on word
# 2. dw (delete word)
# 3. i (insert mode)
# 4. Type new word
# 5. Esc

# Quick file search and edit
# 1. /search_term
# 2. n (to find next)
# 3. i (edit)
# 4. Make changes
# 5. Esc :w (save)
```

### Keyboard Shortcuts Summary
```
Movement:     ↑↓←→, w/b/e, 0/$, gg/G
Insert:       i/a/A/I/o/O
Delete:       x, dd, dw, d$
Copy:         yy, yw, y$
Paste:        p, P
Visual:       v, y, x
Search:       /, ?, n, N
Commands:     :w, :q, :wq, :e
```

This workflow covers the essential features of VimRust for efficient text editing!