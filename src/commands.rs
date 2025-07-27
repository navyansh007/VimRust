use crate::buffer::Buffer;
use anyhow::Result;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct CommandHandler {
    commands: HashMap<String, Box<dyn Command>>,
}
#[allow(dead_code)]
impl CommandHandler {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert("w".to_string(), Box::new(WriteCommand));
        commands.insert("write".to_string(), Box::new(WriteCommand));
        commands.insert("q".to_string(), Box::new(QuitCommand));
        commands.insert("quit".to_string(), Box::new(QuitCommand));
        commands.insert("wq".to_string(), Box::new(WriteQuitCommand));
        commands.insert("x".to_string(), Box::new(WriteQuitCommand));
        commands.insert("e".to_string(), Box::new(EditCommand));
        commands.insert("edit".to_string(), Box::new(EditCommand));
        commands.insert("s".to_string(), Box::new(SubstituteCommand));
        commands.insert("substitute".to_string(), Box::new(SubstituteCommand));
        commands.insert("bn".to_string(), Box::new(NextBufferCommand));
        commands.insert("bp".to_string(), Box::new(PrevBufferCommand));
        commands.insert("bd".to_string(), Box::new(DeleteBufferCommand));
        commands.insert("set".to_string(), Box::new(SetCommand));
        Self { commands }
    }
    pub fn execute(&self, command: &str, buffer: &mut Buffer) -> Result<CommandResult> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(CommandResult::None);
        }
        let cmd_name = parts[0];
        let args = &parts[1..];
        if let Some(cmd) = self.commands.get(cmd_name) {
            cmd.execute(args, buffer)
        } else {
            Err(anyhow::anyhow!("Unknown command: {}", cmd_name))
        }
    }
    pub fn get_completions(&self, partial: &str) -> Vec<String> {
        self.commands
            .keys()
            .filter(|cmd| cmd.starts_with(partial))
            .cloned()
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum CommandResult {
    None,
    Quit,
    Message(String),
    Error(String),
}
#[allow(dead_code)]
pub trait Command {
    fn execute(&self, args: &[&str], buffer: &mut Buffer) -> Result<CommandResult>;
}
struct WriteCommand;
impl Command for WriteCommand {
    fn execute(&self, args: &[&str], buffer: &mut Buffer) -> Result<CommandResult> {
        if args.is_empty() {
            buffer.save()?;
            Ok(CommandResult::Message("File saved".to_string()))
        } else {
            buffer.save_as(args[0])?;
            Ok(CommandResult::Message(format!("File saved as {}", args[0])))
        }
    }
}
struct QuitCommand;
impl Command for QuitCommand {
    fn execute(&self, _args: &[&str], buffer: &mut Buffer) -> Result<CommandResult> {
        if buffer.modified {
            Err(anyhow::anyhow!(
                "No write since last change (use :q! to force quit)"
            ))
        } else {
            Ok(CommandResult::Quit)
        }
    }
}
struct WriteQuitCommand;
impl Command for WriteQuitCommand {
    fn execute(&self, args: &[&str], buffer: &mut Buffer) -> Result<CommandResult> {
        if args.is_empty() {
            buffer.save()?;
        } else {
            buffer.save_as(args[0])?;
        }
        Ok(CommandResult::Quit)
    }
}
struct EditCommand;
impl Command for EditCommand {
    fn execute(&self, args: &[&str], _buffer: &mut Buffer) -> Result<CommandResult> {
        if args.is_empty() {
            Err(anyhow::anyhow!("No file specified"))
        } else {
            Ok(CommandResult::Message(format!("Opening {}", args[0])))
        }
    }
}
struct SubstituteCommand;
impl Command for SubstituteCommand {
    fn execute(&self, args: &[&str], _buffer: &mut Buffer) -> Result<CommandResult> {
        if args.is_empty() {
            Err(anyhow::anyhow!("Substitute requires pattern"))
        } else {
            Ok(CommandResult::Message(
                "Substitute not implemented yet".to_string(),
            ))
        }
    }
}
struct NextBufferCommand;
impl Command for NextBufferCommand {
    fn execute(&self, _args: &[&str], _buffer: &mut Buffer) -> Result<CommandResult> {
        Ok(CommandResult::Message("Next buffer".to_string()))
    }
}
struct PrevBufferCommand;
impl Command for PrevBufferCommand {
    fn execute(&self, _args: &[&str], _buffer: &mut Buffer) -> Result<CommandResult> {
        Ok(CommandResult::Message("Previous buffer".to_string()))
    }
}
struct DeleteBufferCommand;
impl Command for DeleteBufferCommand {
    fn execute(&self, _args: &[&str], _buffer: &mut Buffer) -> Result<CommandResult> {
        Ok(CommandResult::Message("Delete buffer".to_string()))
    }
}
struct SetCommand;
impl Command for SetCommand {
    fn execute(&self, args: &[&str], buffer: &mut Buffer) -> Result<CommandResult> {
        if args.is_empty() {
            return Ok(CommandResult::Message(
                "Available settings: number, syntax".to_string(),
            ));
        }
        match args[0] {
            "number" | "nu" => {
                buffer.line_numbers = true;
                Ok(CommandResult::Message("Line numbers enabled".to_string()))
            }
            "nonumber" | "nonu" => {
                buffer.line_numbers = false;
                Ok(CommandResult::Message("Line numbers disabled".to_string()))
            }
            "syntax" => {
                buffer.syntax_highlighting = true;
                Ok(CommandResult::Message(
                    "Syntax highlighting enabled".to_string(),
                ))
            }
            "nosyntax" => {
                buffer.syntax_highlighting = false;
                Ok(CommandResult::Message(
                    "Syntax highlighting disabled".to_string(),
                ))
            }
            _ => Err(anyhow::anyhow!("Unknown setting: {}", args[0])),
        }
    }
}
