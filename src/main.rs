use std::{io::{self, Read, Write}, process::exit};

mod commands;
use commands::{Command, ShellCommands};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
        let command_str = parts[0];
        let args = &parts[1..];

        match Command::from_str(command_str) {
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
            Command::Pwd => ShellCommands::pwd(),
            Command::Unknown(_) => ShellCommands::unknown(command_str),
        }
    }
}
