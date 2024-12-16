use std::env;

pub struct ShellCommands;

pub enum Command {
    // TODO : Add commands
    Exit,
    Pwd,
    Unknown(String),
}

impl Command {
    pub fn from_str(command: &str) -> Command {
        match command {
            "exit" => Command::Exit,
            "pwd" => Command::Pwd,
            _ => Command::Unknown(command.to_string()),
        }
    }
}

impl ShellCommands {
    // add commands here
    pub fn exit(exit_code: i32) {
        std::process::exit(exit_code)
    }

    pub fn pwd() {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("{}", e),
        }
    }

    pub fn unknown(command: &str) {
        eprintln!("{}: command not found", command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwd() {
        let stdout = std::panic::catch_unwind(|| {
            ShellCommands::pwd();
        });
        assert!(stdout.is_ok());
    }
}