use ropey::Rope;
#[derive(Clone, Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub desired_col: usize,
}
impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            desired_col: 0,
        }
    }
    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.desired_col = self.col;
        }
    }
    pub fn move_right(&mut self, content: &Rope) {
        let line_len = self.get_line_length(content, self.row);
        if self.col < line_len {
            self.col += 1;
            self.desired_col = self.col;
        }
    }
    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
            self.col = self.desired_col;
        }
    }
    pub fn move_down(&mut self, content: &Rope) {
        if self.row + 1 < content.len_lines() {
            self.row += 1;
            let line_len = self.get_line_length(content, self.row);
            self.col = self.desired_col.min(line_len);
        }
    }
    pub fn move_word_forward(&mut self, content: &Rope) {
        let line = content.line(self.row).to_string();
        let chars: Vec<char> = line.chars().collect();
        if self.col < chars.len() {
            while self.col < chars.len() && chars[self.col].is_alphanumeric() {
                self.col += 1;
            }
            while self.col < chars.len() && chars[self.col].is_whitespace() {
                self.col += 1;
            }
            self.desired_col = self.col;
        }
    }
    pub fn move_word_backward(&mut self, content: &Rope) {
        if self.col > 0 {
            let line = content.line(self.row).to_string();
            let chars: Vec<char> = line.chars().collect();
            self.col -= 1;
            while self.col > 0 && chars[self.col].is_whitespace() {
                self.col -= 1;
            }
            while self.col > 0 && chars[self.col - 1].is_alphanumeric() {
                self.col -= 1;
            }
            self.desired_col = self.col;
        }
    }
    pub fn move_word_end(&mut self, content: &Rope) {
        let line = content.line(self.row).to_string();
        let chars: Vec<char> = line.chars().collect();
        if self.col < chars.len() {
            while self.col < chars.len() && chars[self.col].is_alphanumeric() {
                self.col += 1;
            }
            while self.col < chars.len() && chars[self.col].is_whitespace() {
                self.col += 1;
            }
            while self.col < chars.len() && chars[self.col].is_alphanumeric() {
                self.col += 1;
            }
            if self.col > 0 && self.col <= chars.len() {
                self.col -= 1;
            }
            self.desired_col = self.col;
        }
    }
    pub fn move_line_first_non_whitespace(&mut self, content: &Rope) {
        let line = content.line(self.row).to_string();
        let chars: Vec<char> = line.chars().collect();
        self.col = 0;
        while self.col < chars.len() && chars[self.col].is_whitespace() {
            self.col += 1;
        }
        self.desired_col = self.col;
    }
    pub fn move_line_start(&mut self) {
        self.col = 0;
        self.desired_col = 0;
    }
    pub fn move_line_end(&mut self, content: &Rope) {
        let line_len = self.get_line_length(content, self.row);
        self.col = line_len;
        self.desired_col = self.col;
    }
    pub fn move_to_start(&mut self) {
        self.row = 0;
        self.col = 0;
        self.desired_col = 0;
    }
    pub fn move_to_end(&mut self, content: &Rope) {
        self.row = content.len_lines().saturating_sub(1);
        let line_len = self.get_line_length(content, self.row);
        self.col = line_len;
        self.desired_col = self.col;
    }
    #[allow(dead_code)]
    pub fn move_to_line(&mut self, content: &Rope, line: usize) {
        self.row = line.min(content.len_lines().saturating_sub(1));
        let line_len = self.get_line_length(content, self.row);
        self.col = self.desired_col.min(line_len);
    }
    fn get_line_length(&self, content: &Rope, line: usize) -> usize {
        if line < content.len_lines() {
            let line_content = content.line(line);
            let line_str = line_content.to_string();
            if line_str.ends_with('\n') {
                line_str.len().saturating_sub(1)
            } else {
                line_str.len()
            }
        } else {
            0
        }
    }
    #[allow(dead_code)]
    pub fn clamp_to_buffer(&mut self, content: &Rope) {
        self.row = self.row.min(content.len_lines().saturating_sub(1));
        let line_len = self.get_line_length(content, self.row);
        self.col = self.col.min(line_len);
        self.desired_col = self.col;
    }
}
