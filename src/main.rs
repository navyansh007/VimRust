mod buffer;
mod commands;
mod cursor;
mod editor;
mod modes;
mod syntax;
mod ui;
use anyhow::Result;
use clap::Parser;
#[derive(Parser)]
#[command(name = "vimrust")]
#[command(about = "A Vim-like text editor with advanced features")]
struct Args {
    #[arg(help = "File to open")]
    file: Option<String>,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let mut editor = editor::Editor::new()?;
    if let Some(file_path) = args.file {
        editor.open_file(&file_path)?;
    }
    editor.run()
}
