pub struct ShellCommands;

pub enum Command {
    // TODO : Add commands
    Exit,
    Unknown(String),
}

impl Command {
    pub fn from_str(command: &str) -> Command {
        match command {
            "exit" => Command::Exit,
            _ => Command::Unknown(command.to_string()),
        }
    }
}

impl ShellCommands {
    // add commands here
    pub fn exit(exit_code: i32) {
        std::process::exit(exit_code)
    }

    pub fn unknown(command: &str) {
        eprintln!("{}: command not found", command);
    }
}