use std::env;

pub struct ShellCommands;

pub enum Command {
    // TODO : Add commands
    Exit,
    Env,
    Pwd,
    Type,
    Unknown(String),
}

impl Command {
    pub fn from_str(command: &str) -> Command {
        match command {
            "exit" => Command::Exit,
            "env" => Command::Env,
            "pwd" => Command::Pwd,
            "type" => Command::Type,
            _ => Command::Unknown(command.to_string()),
        }
    }
}

impl ShellCommands {
    // add commands here
    pub fn exit(exit_code: i32) {
        std::process::exit(exit_code)
    }

    pub fn env() {
        for (key, value) in env::vars() {
            println!("{}={}", key, value);
        }
    }

    pub fn pwd() {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("{}", e),
        }
    }

    pub fn r#type(command: &str) {
        match Command::from_str(command) {
            Command::Unknown(_) => {
                eprintln!("type: {}: not found", command);
            }
            _ => {
                println!("{} is a shell builtin", command);
            }
        }
    }

    pub fn unknown(command: &str) {
        eprintln!("{}: command not found", command);
    }
}
