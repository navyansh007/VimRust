use crate::buffer::Buffer;
use crate::modes::EditorMode;
use anyhow::Result;
use crossterm::{cursor, execute, terminal};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color as RatatuiColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, stdout, Stdout};
pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    viewport_start: usize,
    viewport_height: usize,
}
impl UI {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        let size = terminal.size()?;
        let viewport_height = size.height as usize - 2;
        Ok(Self {
            terminal,
            viewport_start: 0,
            viewport_height,
        })
    }
    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }
    pub fn exit_alternate_screen(&mut self) -> Result<()> {
        execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn render(
        &mut self,
        buffer: &Buffer,
        mode: &EditorMode,
        command_line: &str,
        message: &Option<String>,
    ) -> Result<()> {
        self.update_viewport(buffer);
        let viewport_start = self.viewport_start;
        let viewport_height = self.viewport_height;
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            render_editor_content(f, chunks[0], buffer, viewport_start, viewport_height);
            render_status_line(f, chunks[1], buffer, mode);
            render_command_line(f, chunks[2], mode, command_line, message);
        })?;
        Ok(())
    }
    fn update_viewport(&mut self, buffer: &Buffer) {
        let cursor_row = buffer.cursor.row;
        if cursor_row < self.viewport_start {
            self.viewport_start = cursor_row;
        } else if cursor_row >= self.viewport_start + self.viewport_height {
            self.viewport_start = cursor_row - self.viewport_height + 1;
        }
    }
}
fn render_editor_content(
    f: &mut ratatui::Frame,
    area: Rect,
    buffer: &Buffer,
    viewport_start: usize,
    _viewport_height: usize,
) {
    let mut lines = Vec::new();
    let end_line = (viewport_start + area.height as usize).min(buffer.line_count());
    for line_num in viewport_start..end_line {
        let line_content = buffer.get_line(line_num).unwrap_or_default();
        let line_number = format!("{:4} ", line_num + 1);
        let mut spans = vec![Span::styled(
            line_number,
            Style::default().fg(RatatuiColor::DarkGray),
        )];
        let content = line_content.trim_end_matches('\n').to_string();
        spans.push(Span::raw(content));
        lines.push(Line::from(spans));
    }
    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, area);
    let cursor_x = 5 + buffer.cursor.col;
    let cursor_y = buffer.cursor.row.saturating_sub(viewport_start);
    if cursor_y < area.height as usize {
        f.set_cursor(area.x + cursor_x as u16, area.y + cursor_y as u16);
    }
}
fn render_status_line(f: &mut ratatui::Frame, area: Rect, buffer: &Buffer, mode: &EditorMode) {
    let file_name = buffer.file_path.as_deref().unwrap_or("[No Name]");
    let modified_indicator = if buffer.modified { " [+]" } else { "" };
    let position = format!("{}:{}", buffer.cursor.row + 1, buffer.cursor.col + 1);
    let line_count = buffer.line_count();
    let status_content = format!(
        " {} {} | {} | {} lines | {}",
        mode.to_string(),
        file_name,
        modified_indicator,
        line_count,
        position
    );
    let status_line = Paragraph::new(status_content).style(
        Style::default()
            .bg(RatatuiColor::Blue)
            .fg(RatatuiColor::White)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(status_line, area);
}
fn render_command_line(
    f: &mut ratatui::Frame,
    area: Rect,
    mode: &EditorMode,
    command_line: &str,
    message: &Option<String>,
) {
    let content = match mode {
        EditorMode::Command => command_line.to_string(),
        _ => {
            if let Some(msg) = message {
                msg.clone()
            } else {
                String::new()
            }
        }
    };
    let command_widget = Paragraph::new(content);
    f.render_widget(command_widget, area);
}
impl UI {
    #[allow(dead_code)]
    pub fn get_terminal_size(&self) -> Result<(u16, u16)> {
        let size = self.terminal.size()?;
        Ok((size.width, size.height))
    }
}
