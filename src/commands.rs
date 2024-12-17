use std::{env, path::Path};

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

    pub fn check_in_path(command: &str, path: &str) -> Option<std::path::PathBuf> {
        let paths: Vec<&str> = path.split(":").collect();

        for dir in paths {
            let candidate = Path::new(dir).join(command);
            if candidate.is_file() {
                return Some(candidate)
            }

        }
        None
    }

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

    pub fn r#type(command: &str, path: &str) {
        match Command::from_str(command) {
            Command::Unknown(_) => {
                if let Some(found_path) = ShellCommands::check_in_path(command, path) {
                    println!("{} is {}", command, found_path.display())
                } else {
                    eprintln!("type: {}: not found", command);
                }
               
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
