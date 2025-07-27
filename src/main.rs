mod editor;
mod buffer;
mod cursor;
mod modes;
mod ui;
mod syntax;
mod commands;
use clap::Parser;
use anyhow::Result;
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
