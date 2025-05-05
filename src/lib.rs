use std::error::Error;
use std::io::Write;
use crate::commands::{parse, Execute};
use crate::database::Database;

mod commands;
mod data;
mod errors;
pub mod database;
mod table;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut database = Database::new();
    loop {
        print!("db > ");
        std::io::stdout().flush()?;
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;
        let command = command.trim();

        match parse(command) {
            Ok(command) => {
                if let Err(err) = command.execute(&mut database) {
                    println!("{}", err)
                }
            }
            Err(err) => println!("Error {err}"),
        }
    }
}