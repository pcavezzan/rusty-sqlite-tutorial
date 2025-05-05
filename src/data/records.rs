use crate::data::car::Car;
use crate::data::user::User;
use crate::errors::CommandError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub enum TableName {
    User,
    Car,
}

impl FromStr for TableName {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "user" => Ok(TableName::User),
            "car" => Ok(TableName::Car),
            _ => Err(CommandError::UnknownTable(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Record {
    User(User),
    Car(Car),
}