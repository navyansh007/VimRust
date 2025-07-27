#[derive(Debug, Clone, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Visual,
    Command,
}
impl EditorMode {
    pub fn to_string(&self) -> &'static str {
        match self {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
            EditorMode::Visual => "VISUAL",
            EditorMode::Command => "COMMAND",
        }
    }
    pub fn get_cursor_style(&self) -> CursorStyle {
        match self {
            EditorMode::Normal => CursorStyle::Block,
            EditorMode::Insert => CursorStyle::Line,
            EditorMode::Visual => CursorStyle::Block,
            EditorMode::Command => CursorStyle::Line,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CursorStyle {
    Block,
    Line,
    Underline,
}
pub trait Mode {
    fn name(&self) -> &'static str;
    fn cursor_style(&self) -> CursorStyle;
}