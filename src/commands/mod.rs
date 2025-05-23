use crate::commands::meta::MetaCommand;
use crate::commands::sql::SqlCommand;
use crate::database::Database;
use crate::errors::{CommandError, ExecutionError};

mod sql;
mod meta;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Sql(SqlCommand),
    Meta(MetaCommand),
    Unknown { command: &'a str },
}

impl Execute for Command<'_> {
    fn execute(self, database: &mut Database) -> Result<(), ExecutionError> {
        match self {
            Command::Sql(command) => command.execute(database),
            Command::Meta(command) => command.execute(database),
            Command::Unknown { .. } => {
                println!("Unknown command");
                Ok(())
            }
        }
    }
}

trait TryFromStr {
    type Error;

    fn try_from_str(value: &str) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized;
}

pub trait Execute {
    fn execute(self, database: &mut Database) -> Result<(), ExecutionError>;
}

pub fn parse(input: &str) -> Result<Command, CommandError> {
    let input = input.trim_start();
    // on utilise le . comme discriminant de meta-commande
    let command = if input.starts_with(".") {
        // le map permet de transformer en énumération Command notre résultat si c'est un Some
        MetaCommand::try_from_str(input)?.map(Command::Meta)
    } else {
        SqlCommand::try_from_str(input)?.map(Command::Sql)
    }
        // si aucun parser n'est capable de trouver une alternative valable
        // alors la commande est inconnue
        .unwrap_or(Command::Unknown { command: input });
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Record, TableName, User};
    #[test]
    fn test_parse() {
        assert_eq!(parse(".exit"), Ok(Command::Meta(MetaCommand::Exit)));
        assert_eq!(
            parse("insert user 1 name email@domain.tld"),
            Ok(Command::Sql(SqlCommand::Insert {
                data: Record::User(User {
                    id: 1,
                    username: "name".to_string(),
                    email: "email@domain.tld".to_string()
                })
            }))
        );
        assert_eq!(
            parse("select car"),
            Ok(Command::Sql(SqlCommand::Select {
                table: TableName::Car
            }))
        );
        assert_eq!(
            parse("create car"),
            Ok(Command::Sql(SqlCommand::Create {
                table: TableName::Car
            }))
        );
        assert_eq!(
            parse("unknown command"),
            Ok(Command::Unknown {
                command: "unknown command"
            })
        );
    }
}

