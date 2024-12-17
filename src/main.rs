use std::{
    env,
    io::{self, Read, Write},
    process::exit,
};

mod commands;
use commands::{Command, ShellCommands};
fn main() {
    loop {
        let user = env::var("USER").unwrap();
        let hostname = env::var("NAME").unwrap();
        let pwd = env::current_dir().unwrap();
        print!("{}@{}:{}$ ", user, hostname, pwd.display());
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }

        if trimmed_input.starts_with("PATH=") {
            let path_value = trimmed_input[5..].trim();
            env::set_var("PATH", path_value);
            continue;
        }

        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
        let command_str = parts[0];
        let args = &parts[1..];

        let current_path = env::var("PATH").unwrap_or_else(|_| String::from("/usr/bin:/bin"));

        match Command::from_str(command_str) {
            Command::Cd => {
                ShellCommands::cd(args);
            }
            Command::Echo => {
                ShellCommands::echo(&parts);
            }
            Command::Exit => {
                // Check if an argument is provided for the exit code
                if let Some(arg) = args.get(0) {
                    match arg.parse::<i32>() {
                        Ok(exit_code) => ShellCommands::exit(exit_code),
                        Err(_) => {
                            ShellCommands::exit(1);
                            continue;
                        }
                    }
                } else {
                    ShellCommands::exit(0); // Default to exit code 0
                }
            }
            Command::Env => ShellCommands::env(),
            Command::Pwd => ShellCommands::pwd(),
            Command::Type => {
                if let Some(arg) = args.get(0) {
                    ShellCommands::r#type(arg, &current_path);
                } else {
                    continue;
                }
            }
            Command::Unknown(_) => ShellCommands::unknown(command_str, args),
        }
    }
}
