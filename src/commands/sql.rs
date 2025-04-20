use crate::commands::{CommandError, Execute, ExecutionError, TryFromStr};
use crate::data::user::User;
use crate::database::Database;

#[derive(Debug, PartialEq)]
pub enum SqlCommand {
    Insert {
        id: i64,
        username: String,
        email: String,
    },
    Select,
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
                        let mut parameters = payload.split_whitespace();
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
                        Ok(Some(SqlCommand::Insert {
                            id,
                            username,
                            email,
                        }))
                    }
                    "select" => Err(CommandError::TooManyArguments)?,
                    _ => Ok(None),
                }
            }
            None => match input {
                "insert" => Err(CommandError::NotEnoughArguments)?,
                "select" => Ok(Some(SqlCommand::Select)),
                _ => Ok(None),
            },
        }
    }
}

impl Execute for SqlCommand {
    fn execute(self, database: &mut Database) -> Result<(), ExecutionError> {
        match self {
            SqlCommand::Insert {
                id,
                username,
                email
            } => {
                let user = User::new(id, username.clone(), email.clone());
                database.insert(user).map_err(ExecutionError::Insertion)?;
                println!("User successfully inserted");
            }
            SqlCommand::Select => {
                for user in database.select::<User>().map_err(ExecutionError::Select)? {
                    println!("{:?}", user);
                }
            }
        }
        Ok(())
    }
}


#[test]
fn test_parse_command_insert() {
    // command d'insert correct
    assert_eq!(
        SqlCommand::try_from_str("insert 1 name email@domain.tld"),
        Ok(Some(SqlCommand::Insert {
            id: 1,
            username: "name".to_string(),
            email: "email@domain.tld".to_string()
        }))
    );
    // robustesse sur le nombre d'espaces
    assert_eq!(
        SqlCommand::try_from_str("    insert     1     name     email@domain.tld     "),
        Ok(Some(SqlCommand::Insert {
            id: 1,
            username: "name".to_string(),
            email: "email@domain.tld".to_string()
        }))
    );
    // pas assez d'arguments
    assert_eq!(
        SqlCommand::try_from_str("insert"),
        Err(CommandError::NotEnoughArguments)
    );
    assert_eq!(
        SqlCommand::try_from_str("insert 1"),
        Err(CommandError::NotEnoughArguments)
    );
    assert_eq!(
        SqlCommand::try_from_str("insert 1 name"),
        Err(CommandError::NotEnoughArguments)
    );
    // mauvais type d'argument
    assert_eq!(
        SqlCommand::try_from_str("insert one name email@domain.tld"),
        Err(CommandError::ExpectingInteger)
    );
    // commande inconnue
    assert_eq!(SqlCommand::try_from_str("unknown command"), Ok(None));
}


#[test]
fn test_parse_command_select() {
    // commande select correcte
    assert_eq!(
        SqlCommand::try_from_str("select"),
        Ok(Some(SqlCommand::Select))
    );
    // robustesse sur les espaces blancs
    assert_eq!(
        SqlCommand::try_from_str("    select    "),
        Ok(Some(SqlCommand::Select))
    );
    // trop d'arguments
    assert_eq!(
        SqlCommand::try_from_str("select args value"),
        Err(CommandError::TooManyArguments)
    );
    // commande inconnue
    assert_eq!(SqlCommand::try_from_str("unknown command"), Ok(None));
}
