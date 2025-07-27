use crate::cursor::Cursor;
use anyhow::Result;
use ropey::Rope;
use std::fs;
use std::path::Path;
#[derive(Clone)]
pub struct BufferState {
    pub content: Rope,
    pub cursor: Cursor,
}
pub struct Buffer {
    pub content: Rope,
    pub cursor: Cursor,
    pub file_path: Option<String>,
    pub modified: bool,
    pub undo_stack: Vec<BufferState>,
    pub redo_stack: Vec<BufferState>,
    pub line_numbers: bool,
    pub syntax_highlighting: bool,
}
impl Buffer {
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
            cursor: Cursor::new(),
            file_path: None,
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            line_numbers: true,
            syntax_highlighting: true,
        }
    }
    pub fn from_file(path: &str) -> Result<Self> {
        let content = if Path::new(path).exists() {
            Rope::from_str(&fs::read_to_string(path)?)
        } else {
            Rope::new()
        };
        Ok(Self {
            content,
            cursor: Cursor::new(),
            file_path: Some(path.to_string()),
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            line_numbers: true,
            syntax_highlighting: true,
        })
    }
    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.file_path {
            fs::write(path, self.content.to_string())?;
            self.modified = false;
        }
        Ok(())
    }
    pub fn save_as(&mut self, path: &str) -> Result<()> {
        fs::write(path, self.content.to_string())?;
        self.file_path = Some(path.to_string());
        self.modified = false;
        Ok(())
    }
    pub fn save_state(&mut self) {
        let state = BufferState {
            content: self.content.clone(),
            cursor: self.cursor.clone(),
        };
        self.undo_stack.push(state);
        self.redo_stack.clear();
        if self.undo_stack.len() > 1000 {
            self.undo_stack.remove(0);
        }
    }
    pub fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop() {
            let current_state = BufferState {
                content: self.content.clone(),
                cursor: self.cursor.clone(),
            };
            self.redo_stack.push(current_state);
            self.content = state.content;
            self.cursor = state.cursor;
        }
    }
    pub fn redo(&mut self) {
        if let Some(state) = self.redo_stack.pop() {
            let current_state = BufferState {
                content: self.content.clone(),
                cursor: self.cursor.clone(),
            };
            self.undo_stack.push(current_state);
            self.content = state.content;
            self.cursor = state.cursor;
        }
    }
    pub fn insert_char(&mut self, c: char) {
        self.save_state();
        let pos = self.cursor_to_char_index();
        self.content.insert_char(pos, c);
        self.cursor.col += 1;
        self.modified = true;
    }
    pub fn insert_newline(&mut self) {
        self.save_state();
        let pos = self.cursor_to_char_index();
        self.content.insert_char(pos, '\n');
        self.cursor.row += 1;
        self.cursor.col = 0;
        self.modified = true;
    }
    pub fn backspace(&mut self) {
        if self.cursor.col > 0 {
            self.save_state();
            self.cursor.col -= 1;
            let pos = self.cursor_to_char_index();
            self.content.remove(pos..pos + 1);
            self.modified = true;
        } else if self.cursor.row > 0 {
            self.save_state();
            let prev_line_len = self.get_line_length(self.cursor.row - 1);
            self.cursor.row -= 1;
            self.cursor.col = prev_line_len;
            let pos = self.cursor_to_char_index();
            self.content.remove(pos..pos + 1);
            self.modified = true;
        }
    }
    pub fn delete_char(&mut self) {
        self.save_state();
        let pos = self.cursor_to_char_index();
        if pos < self.content.len_chars() {
            self.content.remove(pos..pos + 1);
            self.modified = true;
        }
    }
    pub fn delete_line(&mut self) {
        self.save_state();
        let line_start = self.content.line_to_char(self.cursor.row);
        let line_end = if self.cursor.row + 1 < self.content.len_lines() {
            self.content.line_to_char(self.cursor.row + 1)
        } else {
            self.content.len_chars()
        };
        self.content.remove(line_start..line_end);
        self.cursor.col = 0;
        self.modified = true;
    }
    pub fn get_current_line(&self) -> String {
        if self.cursor.row < self.content.len_lines() {
            self.content.line(self.cursor.row).to_string()
        } else {
            String::new()
        }
    }
    pub fn paste_after(&mut self, text: &str) {
        self.save_state();
        self.cursor.col += 1;
        let pos = self.cursor_to_char_index();
        self.content.insert(pos, text);
        self.modified = true;
    }
    pub fn paste_before(&mut self, text: &str) {
        self.save_state();
        let pos = self.cursor_to_char_index();
        self.content.insert(pos, text);
        self.modified = true;
    }
    pub fn get_line_length(&self, line: usize) -> usize {
        if line < self.content.len_lines() {
            let line_content = self.content.line(line);
            let line_str = line_content.to_string();
            if line_str.ends_with('\n') {
                line_str.len() - 1
            } else {
                line_str.len()
            }
        } else {
            0
        }
    }
    pub fn get_line(&self, line_num: usize) -> Option<String> {
        if line_num < self.content.len_lines() {
            Some(self.content.line(line_num).to_string())
        } else {
            None
        }
    }
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }
    fn cursor_to_char_index(&self) -> usize {
        let line_start = self.content.line_to_char(self.cursor.row);
        line_start + self.cursor.col
    }
    pub fn delete_to_line_end(&mut self) {
        self.save_state();
        let line_start = self.content.line_to_char(self.cursor.row);
        let line_end_pos = line_start + self.cursor.col;
        let actual_line_end = if self.cursor.row + 1 < self.content.len_lines() {
            self.content.line_to_char(self.cursor.row + 1) - 1
        } else {
            self.content.len_chars()
        };
        if line_end_pos < actual_line_end {
            self.content.remove(line_end_pos..actual_line_end);
            self.modified = true;
        }
    }
    pub fn get_text_to_line_end(&self) -> String {
        let line_start = self.content.line_to_char(self.cursor.row);
        let line_end_pos = line_start + self.cursor.col;
        let actual_line_end = if self.cursor.row + 1 < self.content.len_lines() {
            self.content.line_to_char(self.cursor.row + 1) - 1
        } else {
            self.content.len_chars()
        };
        if line_end_pos < actual_line_end {
            self.content
                .slice(line_end_pos..actual_line_end)
                .to_string()
        } else {
            String::new()
        }
    }
    pub fn find_next(&mut self, pattern: &str) {
        let content_str = self.content.to_string();
        let current_pos = self.cursor_to_char_index();
        if let Some(pos) = content_str[current_pos + 1..].find(pattern) {
            let absolute_pos = current_pos + 1 + pos;
            let line = self.content.char_to_line(absolute_pos);
            let line_start = self.content.line_to_char(line);
            let col = absolute_pos - line_start;
            self.cursor.row = line;
            self.cursor.col = col;
            self.cursor.desired_col = col;
        }
    }
    pub fn find_previous(&mut self, pattern: &str) {
        let content_str = self.content.to_string();
        let current_pos = self.cursor_to_char_index();
        if current_pos > 0 {
            if let Some(pos) = content_str[..current_pos].rfind(pattern) {
                let line = self.content.char_to_line(pos);
                let line_start = self.content.line_to_char(line);
                let col = pos - line_start;
                self.cursor.row = line;
                self.cursor.col = col;
                self.cursor.desired_col = col;
            }
        }
    }
    pub fn delete_word(&mut self) {
        self.save_state();
        let cursor_pos = self.cursor_to_char_index();
        let content_str = self.content.to_string();
        let chars: Vec<char> = content_str.chars().collect();
        let mut end_pos = cursor_pos;
        while end_pos < chars.len() && chars[end_pos].is_alphanumeric() {
            end_pos += 1;
        }
        while end_pos < chars.len() && chars[end_pos].is_whitespace() {
            end_pos += 1;
        }
        if cursor_pos < end_pos {
            self.content.remove(cursor_pos..end_pos);
            self.modified = true;
        }
    }
    pub fn delete_to_line_start(&mut self) {
        self.save_state();
        let line_start = self.content.line_to_char(self.cursor.row);
        let cursor_pos = line_start + self.cursor.col;
        if cursor_pos > line_start {
            self.content.remove(line_start..cursor_pos);
            self.cursor.col = 0;
            self.modified = true;
        }
    }
    pub fn yank_word(&self) -> String {
        let cursor_pos = self.cursor_to_char_index();
        let content_str = self.content.to_string();
        let chars: Vec<char> = content_str.chars().collect();
        let mut end_pos = cursor_pos;
        while end_pos < chars.len() && chars[end_pos].is_alphanumeric() {
            end_pos += 1;
        }
        if cursor_pos < end_pos {
            chars[cursor_pos..end_pos].iter().collect()
        } else {
            String::new()
        }
    }
}
