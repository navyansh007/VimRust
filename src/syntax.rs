use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::{ThemeSet, Style as SyntectStyle};
use syntect::easy::HighlightLines;
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    current_theme: String,
}
impl SyntaxHighlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        Self {
            syntax_set,
            theme_set,
            current_theme: "base16-ocean.dark".to_string(),
        }
    }
    pub fn get_syntax_for_file(&self, file_path: &str) -> Option<&SyntaxReference> {
        self.syntax_set.find_syntax_for_file(file_path).unwrap_or(None)
    }
    pub fn get_syntax_by_extension(&self, extension: &str) -> Option<&SyntaxReference> {
        self.syntax_set.find_syntax_by_extension(extension)
    }
    pub fn get_syntax_by_name(&self, name: &str) -> Option<&SyntaxReference> {
        self.syntax_set.find_syntax_by_name(name)
    }
    pub fn highlight_line<'a>(&self, line: &'a str, syntax: &SyntaxReference) -> Vec<(SyntectStyle, &'a str)> {
        let theme = &self.theme_set.themes[&self.current_theme];
        let mut highlighter = HighlightLines::new(syntax, theme);
        match highlighter.highlight_line(line, &self.syntax_set) {
            Ok(highlights) => highlights,
            Err(_) => vec![(SyntectStyle::default(), line)],
        }
    }
    pub fn set_theme(&mut self, theme_name: &str) {
        if self.theme_set.themes.contains_key(theme_name) {
            self.current_theme = theme_name.to_string();
        }
    }
    pub fn get_available_themes(&self) -> Vec<&String> {
        self.theme_set.themes.keys().collect()
    }
    pub fn get_current_theme(&self) -> &str {
        &self.current_theme
    }
}
#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<HighlightedSegment>,
}
#[derive(Debug, Clone)]
pub struct HighlightedSegment {
    pub text: String,
    pub style: TextStyle,
}
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub foreground: (u8, u8, u8),
    pub background: (u8, u8, u8),
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}
impl Default for TextStyle {
    fn default() -> Self {
        Self {
            foreground: (255, 255, 255),
            background: (0, 0, 0),
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
impl From<SyntectStyle> for TextStyle {
    fn from(style: SyntectStyle) -> Self {
        Self {
            foreground: (style.foreground.r, style.foreground.g, style.foreground.b),
            background: (style.background.r, style.background.g, style.background.b),
            bold: style.font_style.contains(syntect::highlighting::FontStyle::BOLD),
            italic: style.font_style.contains(syntect::highlighting::FontStyle::ITALIC),
            underline: style.font_style.contains(syntect::highlighting::FontStyle::UNDERLINE),
        }
    }
}