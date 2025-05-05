use crate::commands::{CommandError, Execute, ExecutionError, TryFromStr};
use crate::database::Database;
use std::str::{FromStr, SplitWhitespace};
use crate::data::{Car, Record, TableName, User};

#[derive(Debug, PartialEq)]
pub enum SqlCommand {
    Insert { data: Record },
    Select { table: TableName },
    Create { table: TableName },
}

impl TryFromStr for SqlCommand {
    type Error = CommandError;

    fn try_from_str(input: &str) -> Result<Option<Self>, Self::Error> {
        // nettoyage des espaces blancs supplémentaires
        let input = input.trim();
        // On vérifie s'il y a des espaces blancs
        let first_space = input.find(' ');
        // La commande possède des arguments
        match first_space {
            Some(first_space_index) => {
                let command = &input[0..first_space_index];
                let payload = &input[first_space_index + 1..];
                match command {
                    "insert" => {
                        // création d'un itérateur sur les espaces blancs
                        let parameters = payload.split_whitespace();
                        let data = Record::from_parameters(parameters)?;

                        Ok(Some(SqlCommand::Insert { data }))
                    }
                    "select" => {
                        let mut parameters = payload.split_whitespace();
                        let table = parameters
                            .next()
                            .ok_or(CommandError::NotEnoughArguments)?
                            .to_string();
                        let table = TableName::from_str(&table)?;
                        if parameters.next().is_some() {
                            return Err(CommandError::TooManyArguments)?;
                        }
                        Ok(Some(SqlCommand::Select { table }))
                    }
                    "create" => {
                        let mut parameters = payload.split_whitespace();
                        let table = parameters
                            .next()
                            .ok_or(CommandError::NotEnoughArguments)?
                            .to_string();
                        let table = TableName::from_str(&table)?;
                        if parameters.next().is_some() {
                            return Err(CommandError::TooManyArguments)?;
                        }
                        Ok(Some(SqlCommand::Create { table }))
                    }
                    _ => Ok(None),
                }
            }
            None => match input {
                "insert" => Err(CommandError::NotEnoughArguments)?,
                "select" => Err(CommandError::NotEnoughArguments)?,
                "create" => Err(CommandError::NotEnoughArguments)?,
                _ => Ok(None),
            },
        }
    }
}

impl Execute for SqlCommand {
    fn execute(self, database: &mut Database) -> Result<(), ExecutionError> {
        match self {
            SqlCommand::Insert { data } => {
                database.insert(data).map_err(ExecutionError::Insertion)?;
                println!("Record inserted successfully");
            }
            SqlCommand::Select { table } => {
                for user in database.select(table).map_err(ExecutionError::Select)? {
                    println!("{:?}", user);
                }
            }
            SqlCommand::Create { table } => {
                database.create_table(table).map_err(ExecutionError::Create)?;
                println!("Table created successfully");
            }
        }
        Ok(())
    }
}


impl Record {
    pub fn from_parameters(mut parameters: SplitWhitespace) -> Result<Record, CommandError> {
        let record_type_string = parameters
            .next()
            .ok_or(CommandError::NotEnoughArguments)?
            .to_string();
        let record_type = TableName::from_str(&record_type_string)?;
        match record_type {
            TableName::User => {
                let id = parameters
                    .next()
                    .ok_or(CommandError::NotEnoughArguments)?
                    .parse()
                    .map_err(|_| CommandError::ExpectingInteger)?;
                let username = parameters
                    .next()
                    .ok_or(CommandError::NotEnoughArguments)?
                    .to_string();
                let email = parameters
                    .next()
                    .ok_or(CommandError::NotEnoughArguments)?
                    .to_string();
                Ok(Record::User(User::new(id, username, email)))
            }
            TableName::Car => {
                let id = parameters
                    .next()
                    .ok_or(CommandError::NotEnoughArguments)?
                    .to_string();
                let brand = parameters
                    .next()
                    .ok_or(CommandError::NotEnoughArguments)?
                    .to_string();
                Ok(Record::Car(Car::new(id, brand)))
            }
        }
    }
}

#[test]
fn test_parse_command_insert() {
    // command d'insert correct
    assert_eq!(
        SqlCommand::try_from_str("insert User 1 name email@domain.tld"),
        Ok(Some(SqlCommand::Insert {
            data: Record::User(User {
                id: 1,
                username: "name".to_string(),
                email: "email@domain.tld".to_string()
            })
        }))
    );
    // robustesse sur le nombre d'espaces
    assert_eq!(
        SqlCommand::try_from_str("    insert   User  1     name     email@domain.tld     "),
        Ok(Some(SqlCommand::Insert {
            data: Record::User(User {
                id: 1,
                username: "name".to_string(),
                email: "email@domain.tld".to_string()
            })
        }))
    );
    // pas assez d'arguments
    assert_eq!(
        SqlCommand::try_from_str("insert"),
        Err(CommandError::NotEnoughArguments)
    );
    assert_eq!(
        SqlCommand::try_from_str("insert user"),
        Err(CommandError::NotEnoughArguments)
    );
    assert_eq!(
        SqlCommand::try_from_str("insert user 1 name"),
        Err(CommandError::NotEnoughArguments)
    );
    // mauvais type d'argument
    assert_eq!(
        SqlCommand::try_from_str("insert user one name email@domain.tld"),
        Err(CommandError::ExpectingInteger)
    );
    // commande inconnue
    assert_eq!(SqlCommand::try_from_str("unknown command"), Ok(None));
}

#[test]
fn test_parse_command_select() {
    // commande select correcte
    assert_eq!(
        SqlCommand::try_from_str("select Car"),
        Ok(Some(SqlCommand::Select {
            table: TableName::Car
        }))
    );
    assert_eq!(
        SqlCommand::try_from_str("    select  User   "),
        Ok(Some(SqlCommand::Select {
            table: TableName::User
        }))
    );
    // table inconnue
    assert_eq!(
        SqlCommand::try_from_str("select unknown"),
        Err(CommandError::UnknownTable("unknown".to_string()))
    );
    // trop d'arguments
    assert_eq!(
        SqlCommand::try_from_str("select user value"),
        Err(CommandError::TooManyArguments)
    );
    // pas assez d'arguments
    assert_eq!(
        SqlCommand::try_from_str("select"),
        Err(CommandError::NotEnoughArguments)
    );
    // commande inconnue
    assert_eq!(SqlCommand::try_from_str("unknown command"), Ok(None));
}

#[test]
fn test_parse_command_create() {
    // commande select correcte
    assert_eq!(
        SqlCommand::try_from_str("create Car"),
        Ok(Some(SqlCommand::Create {
            table: TableName::Car
        }))
    );
    assert_eq!(
        SqlCommand::try_from_str("    create  User   "),
        Ok(Some(SqlCommand::Create {
            table: TableName::User
        }))
    );
    // table inconnue
    assert_eq!(
        SqlCommand::try_from_str("create unknown"),
        Err(CommandError::UnknownTable("unknown".to_string()))
    );
    // trop d'arguments
    assert_eq!(
        SqlCommand::try_from_str("create user value"),
        Err(CommandError::TooManyArguments)
    );
    // pas assez d'arguments
    assert_eq!(
        SqlCommand::try_from_str("create"),
        Err(CommandError::NotEnoughArguments)
    );
    // commande inconnue
    assert_eq!(SqlCommand::try_from_str("unknown command"), Ok(None));
}

