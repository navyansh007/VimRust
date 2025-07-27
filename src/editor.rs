use crate::buffer::Buffer;
use crate::commands::CommandHandler;
use crate::modes::EditorMode;
use crate::ui::UI;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
pub struct Editor {
    buffers: Vec<Buffer>,
    current_buffer: usize,
    mode: EditorMode,
    ui: UI,
    #[allow(dead_code)]
    command_handler: CommandHandler,
    running: bool,
    clipboard: Vec<String>,
    search_pattern: Option<String>,
    #[allow(dead_code)]
    last_command: Option<String>,
    command_line: String,
    message: Option<String>,
    pending_command: Option<char>,
    visual_start: Option<(usize, usize)>,
}
impl Editor {
    pub fn new() -> Result<Self> {
        let ui = UI::new()?;
        let command_handler = CommandHandler::new();
        Ok(Self {
            buffers: vec![Buffer::new()],
            current_buffer: 0,
            mode: EditorMode::Normal,
            ui,
            command_handler,
            running: true,
            clipboard: Vec::new(),
            search_pattern: None,
            last_command: None,
            command_line: String::new(),
            message: None,
            pending_command: None,
            visual_start: None,
        })
    }
    pub fn open_file(&mut self, path: &str) -> Result<()> {
        let buffer = Buffer::from_file(path)?;
        self.buffers.push(buffer);
        self.current_buffer = self.buffers.len() - 1;
        Ok(())
    }
    pub fn run(&mut self) -> Result<()> {
        self.ui.enter_alternate_screen()?;
        while self.running {
            self.render()?;
            self.handle_input()?;
        }
        self.ui.exit_alternate_screen()?;
        Ok(())
    }
    fn render(&mut self) -> Result<()> {
        let buffer = &self.buffers[self.current_buffer];
        self.ui
            .render(buffer, &self.mode, &self.command_line, &self.message)?;
        Ok(())
    }
    fn handle_input(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                self.process_key_event(key_event)?;
            }
        }
        Ok(())
    }
    fn process_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match &self.mode {
            EditorMode::Normal => self.handle_normal_mode(key_event),
            EditorMode::Insert => self.handle_insert_mode(key_event),
            EditorMode::Visual => self.handle_visual_mode(key_event),
            EditorMode::Command => self.handle_command_mode(key_event),
        }
    }
    fn handle_normal_mode(&mut self, key_event: KeyEvent) -> Result<()> {
        if let Some(pending) = self.pending_command {
            self.pending_command = None;
            return self.handle_pending_command(pending, key_event);
        }
        let buffer = &mut self.buffers[self.current_buffer];
        match key_event.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('i') => self.mode = EditorMode::Insert,
            KeyCode::Char('I') => {
                buffer.cursor.move_line_start();
                self.mode = EditorMode::Insert;
            }
            KeyCode::Char('o') => {
                buffer.cursor.move_line_end(&buffer.content);
                buffer.insert_newline();
                self.mode = EditorMode::Insert;
            }
            KeyCode::Char('O') => {
                buffer.cursor.move_line_start();
                buffer.insert_newline();
                buffer.cursor.move_up();
                self.mode = EditorMode::Insert;
            }
            KeyCode::Char('v') => {
                self.mode = EditorMode::Visual;
                self.visual_start = Some((buffer.cursor.row, buffer.cursor.col));
            }
            KeyCode::Char(':') => {
                self.mode = EditorMode::Command;
                self.command_line = ":".to_string();
            }
            KeyCode::Char('/') => {
                self.mode = EditorMode::Command;
                self.command_line = "/".to_string();
            }
            KeyCode::Char('?') => {
                self.mode = EditorMode::Command;
                self.command_line = "?".to_string();
            }
            KeyCode::Up => buffer.cursor.move_up(),
            KeyCode::Down => buffer.cursor.move_down(&buffer.content),
            KeyCode::Left => buffer.cursor.move_left(),
            KeyCode::Right => buffer.cursor.move_right(&buffer.content),
            KeyCode::Char('a') => {
                buffer.cursor.move_right(&buffer.content);
                self.mode = EditorMode::Insert;
            }
            KeyCode::Char('A') => {
                buffer.cursor.move_line_end(&buffer.content);
                self.mode = EditorMode::Insert;
            }
            KeyCode::Char('w') => buffer.cursor.move_word_forward(&buffer.content),
            KeyCode::Char('b') => buffer.cursor.move_word_backward(&buffer.content),
            KeyCode::Char('e') => buffer.cursor.move_word_end(&buffer.content),
            KeyCode::Char('0') => buffer.cursor.move_line_start(),
            KeyCode::Char('$') => buffer.cursor.move_line_end(&buffer.content),
            KeyCode::Char('^') => buffer
                .cursor
                .move_line_first_non_whitespace(&buffer.content),
            KeyCode::Char('G') => buffer.cursor.move_to_end(&buffer.content),
            KeyCode::Char('g') => {
                buffer.cursor.move_to_start();
            }
            KeyCode::Char('x') => {
                buffer.delete_char();
            }
            KeyCode::Char('X') => {
                buffer.backspace();
            }
            KeyCode::Delete => {
                buffer.delete_char();
            }
            KeyCode::Char('d') => {
                self.pending_command = Some('d');
            }
            KeyCode::Char('D') => {
                buffer.delete_to_line_end();
            }
            KeyCode::Char('y') => {
                self.pending_command = Some('y');
            }
            KeyCode::Char('Y') => {
                let text = buffer.get_text_to_line_end();
                self.clipboard = vec![text];
            }
            KeyCode::Char('p') => {
                if !self.clipboard.is_empty() {
                    buffer.paste_after(&self.clipboard[0]);
                }
            }
            KeyCode::Char('P') => {
                if !self.clipboard.is_empty() {
                    buffer.paste_before(&self.clipboard[0]);
                }
            }
            KeyCode::Char('u') => {
                buffer.undo();
            }
            KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                buffer.redo();
            }
            KeyCode::Char('n') => {
                if let Some(pattern) = &self.search_pattern {
                    buffer.find_next(pattern);
                }
            }
            KeyCode::Char('N') => {
                if let Some(pattern) = &self.search_pattern {
                    buffer.find_previous(pattern);
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_insert_mode(&mut self, key_event: KeyEvent) -> Result<()> {
        let buffer = &mut self.buffers[self.current_buffer];
        match key_event.code {
            KeyCode::Esc => self.mode = EditorMode::Normal,
            KeyCode::Char(c) => buffer.insert_char(c),
            KeyCode::Enter => buffer.insert_newline(),
            KeyCode::Backspace => buffer.backspace(),
            KeyCode::Delete => buffer.delete_char(),
            KeyCode::Tab => buffer.insert_char('\t'),
            KeyCode::Up => buffer.cursor.move_up(),
            KeyCode::Down => buffer.cursor.move_down(&buffer.content),
            KeyCode::Left => buffer.cursor.move_left(),
            KeyCode::Right => buffer.cursor.move_right(&buffer.content),
            KeyCode::Home => buffer.cursor.move_line_start(),
            KeyCode::End => buffer.cursor.move_line_end(&buffer.content),
            KeyCode::PageUp => {
                for _ in 0..10 {
                    buffer.cursor.move_up();
                }
            }
            KeyCode::PageDown => {
                for _ in 0..10 {
                    buffer.cursor.move_down(&buffer.content);
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_visual_mode(&mut self, key_event: KeyEvent) -> Result<()> {
        let buffer = &mut self.buffers[self.current_buffer];
        match key_event.code {
            KeyCode::Esc => {
                self.mode = EditorMode::Normal;
                self.visual_start = None;
            }
            KeyCode::Up => buffer.cursor.move_up(),
            KeyCode::Down => buffer.cursor.move_down(&buffer.content),
            KeyCode::Left => buffer.cursor.move_left(),
            KeyCode::Right => buffer.cursor.move_right(&buffer.content),
            KeyCode::Char('w') => buffer.cursor.move_word_forward(&buffer.content),
            KeyCode::Char('b') => buffer.cursor.move_word_backward(&buffer.content),
            KeyCode::Char('e') => buffer.cursor.move_word_end(&buffer.content),
            KeyCode::Char('0') => buffer.cursor.move_line_start(),
            KeyCode::Char('$') => buffer.cursor.move_line_end(&buffer.content),
            KeyCode::Char('y') => {
                if let Some(text) = self.get_visual_selection() {
                    self.clipboard = vec![text];
                }
                self.mode = EditorMode::Normal;
                self.visual_start = None;
            }
            KeyCode::Char('x') | KeyCode::Delete => {
                if let Some(text) = self.get_visual_selection() {
                    self.clipboard = vec![text];
                    self.delete_visual_selection();
                }
                self.mode = EditorMode::Normal;
                self.visual_start = None;
            }
            _ => {}
        }
        Ok(())
    }
    fn get_visual_selection(&self) -> Option<String> {
        if let Some((start_row, start_col)) = self.visual_start {
            let buffer = &self.buffers[self.current_buffer];
            let end_row = buffer.cursor.row;
            let end_col = buffer.cursor.col;
            let (start_row, start_col, end_row, end_col) =
                if start_row < end_row || (start_row == end_row && start_col <= end_col) {
                    (start_row, start_col, end_row, end_col)
                } else {
                    (end_row, end_col, start_row, start_col)
                };
            let content = &buffer.content;
            let start_char = content.line_to_char(start_row) + start_col;
            let end_char = content.line_to_char(end_row) + end_col + 1;
            if start_char < end_char && end_char <= content.len_chars() {
                Some(content.slice(start_char..end_char).to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
    fn delete_visual_selection(&mut self) {
        if let Some((start_row, start_col)) = self.visual_start {
            let buffer = &mut self.buffers[self.current_buffer];
            let end_row = buffer.cursor.row;
            let end_col = buffer.cursor.col;
            let (start_row, start_col, end_row, end_col) =
                if start_row < end_row || (start_row == end_row && start_col <= end_col) {
                    (start_row, start_col, end_row, end_col)
                } else {
                    (end_row, end_col, start_row, start_col)
                };
            buffer.save_state();
            let start_char = buffer.content.line_to_char(start_row) + start_col;
            let end_char = buffer.content.line_to_char(end_row) + end_col + 1;
            if start_char < end_char && end_char <= buffer.content.len_chars() {
                buffer.content.remove(start_char..end_char);
                buffer.cursor.row = start_row;
                buffer.cursor.col = start_col;
                buffer.modified = true;
            }
        }
    }
    fn handle_command_mode(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.mode = EditorMode::Normal;
                self.command_line.clear();
            }
            KeyCode::Enter => {
                self.execute_command()?;
                self.mode = EditorMode::Normal;
                self.command_line.clear();
            }
            KeyCode::Backspace => {
                if self.command_line.len() > 1 {
                    self.command_line.pop();
                } else {
                    self.mode = EditorMode::Normal;
                    self.command_line.clear();
                }
            }
            KeyCode::Char(c) => {
                self.command_line.push(c);
            }
            _ => {}
        }
        Ok(())
    }
    fn execute_command(&mut self) -> Result<()> {
        let cmd = self.command_line.clone();
        if let Some(command) = cmd.strip_prefix(':') {
            self.execute_colon_command(command)?;
        } else if let Some(pattern) = cmd.strip_prefix('/') {
            self.search_forward(pattern);
        } else if let Some(pattern) = cmd.strip_prefix('?') {
            self.search_backward(pattern);
        }
        Ok(())
    }
    fn execute_colon_command(&mut self, command: &str) -> Result<()> {
        let buffer = &mut self.buffers[self.current_buffer];
        match command {
            "w" | "write" => {
                buffer.save()?;
                self.message = Some("File saved".to_string());
            }
            "q" | "quit" => {
                if buffer.modified {
                    self.message =
                        Some("No write since last change (use :q! to force quit)".to_string());
                } else {
                    self.running = false;
                }
            }
            "q!" => {
                self.running = false;
            }
            "wq" | "x" => {
                buffer.save()?;
                self.running = false;
            }
            cmd if cmd.starts_with("w ") => {
                let filename = &cmd[2..];
                buffer.save_as(filename)?;
                self.message = Some(format!("File saved as {}", filename));
            }
            cmd if cmd.starts_with("e ") => {
                let filename = &cmd[2..];
                let new_buffer = Buffer::from_file(filename)?;
                self.buffers.push(new_buffer);
                self.current_buffer = self.buffers.len() - 1;
                self.message = Some(format!("Opened {}", filename));
            }
            "bn" => {
                if self.current_buffer + 1 < self.buffers.len() {
                    self.current_buffer += 1;
                }
            }
            "bp" => {
                if self.current_buffer > 0 {
                    self.current_buffer -= 1;
                }
            }
            "bd" => {
                if self.buffers.len() > 1 {
                    self.buffers.remove(self.current_buffer);
                    if self.current_buffer >= self.buffers.len() {
                        self.current_buffer = self.buffers.len() - 1;
                    }
                }
            }
            cmd if cmd.starts_with("s/") => {
                self.execute_substitute(cmd)?;
            }
            "set number" | "set nu" => {
                self.buffers[self.current_buffer].line_numbers = true;
                self.message = Some("Line numbers enabled".to_string());
            }
            "set nonumber" | "set nonu" => {
                self.buffers[self.current_buffer].line_numbers = false;
                self.message = Some("Line numbers disabled".to_string());
            }
            "set syntax" => {
                self.buffers[self.current_buffer].syntax_highlighting = true;
                self.message = Some("Syntax highlighting enabled".to_string());
            }
            "set nosyntax" => {
                self.buffers[self.current_buffer].syntax_highlighting = false;
                self.message = Some("Syntax highlighting disabled".to_string());
            }
            _ => {
                self.message = Some(format!("Unknown command: {}", command));
            }
        }
        Ok(())
    }
    fn execute_substitute(&mut self, command: &str) -> Result<()> {
        let parts: Vec<&str> = command.split('/').collect();
        if parts.len() >= 3 {
            let pattern = parts[1];
            let replacement = parts[2];
            let flags = if parts.len() > 3 { parts[3] } else { "" };
            let buffer = &mut self.buffers[self.current_buffer];
            let content = buffer.content.to_string();
            let new_content = if flags.contains('g') {
                content.replace(pattern, replacement)
            } else {
                content.replacen(pattern, replacement, 1)
            };
            buffer.content = ropey::Rope::from_str(&new_content);
            buffer.modified = true;
            self.message = Some("Substitution complete".to_string());
        } else {
            self.message = Some("Invalid substitute format".to_string());
        }
        Ok(())
    }
    fn search_forward(&mut self, pattern: &str) {
        self.search_pattern = Some(pattern.to_string());
        let buffer = &mut self.buffers[self.current_buffer];
        buffer.find_next(pattern);
    }
    fn search_backward(&mut self, pattern: &str) {
        self.search_pattern = Some(pattern.to_string());
        let buffer = &mut self.buffers[self.current_buffer];
        buffer.find_previous(pattern);
    }
    fn handle_pending_command(&mut self, pending: char, key_event: KeyEvent) -> Result<()> {
        let buffer = &mut self.buffers[self.current_buffer];
        match (pending, key_event.code) {
            ('d', KeyCode::Char('d')) => {
                buffer.delete_line();
            }
            ('d', KeyCode::Char('w')) => {
                buffer.delete_word();
            }
            ('d', KeyCode::Char('$')) => {
                buffer.delete_to_line_end();
            }
            ('d', KeyCode::Char('0')) => {
                buffer.delete_to_line_start();
            }
            ('y', KeyCode::Char('y')) => {
                let line = buffer.get_current_line();
                self.clipboard = vec![line];
            }
            ('y', KeyCode::Char('w')) => {
                let word = buffer.yank_word();
                self.clipboard = vec![word];
            }
            ('y', KeyCode::Char('$')) => {
                let text = buffer.get_text_to_line_end();
                self.clipboard = vec![text];
            }
            _ => {}
        }
        Ok(())
    }
    #[allow(dead_code)]
    pub fn get_current_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer]
    }
    #[allow(dead_code)]
    pub fn get_current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer]
    }
}
