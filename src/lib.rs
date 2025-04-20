use std::error::Error;
use std::io::Write;
use crate::commands::{parse, Execute};

mod commands;
mod data;
mod errors;
mod database;

pub fn run() -> Result<(), Box<dyn Error>> {
    loop {
        print!("db > ");
        // Allow user to press enter to execute command on a single line
        std::io::stdout().flush()?;
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;
        let command = command.trim();

        // parse and execute command
        match parse(command) {
            // If a command has been found, execute it
            Ok(command) => command.execute()?,
            // Otherwise, print error message
            Err(err) => println!("Error {}", err)
        }
    }
}