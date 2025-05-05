use crate::data::{Car, Record, TableName, User};
use crate::errors::{CreationError, InsertionError, SelectError};
use crate::table::Table;
use std::collections::HashMap;

const DATABASE_SIZE : usize = 1024*1024;

pub struct Database {
    tables: HashMap<TableName, Table>,
}
impl Database {
    pub fn new() -> Self {
        Self {
            tables: Default::default(),
        }
    }


    pub fn create_table(&mut self, table_name: TableName) -> Result<(), CreationError> {
        if self.tables.contains_key(&table_name) {
            return Err(CreationError::TableAlreadyExist(table_name))
        }
        self.tables.insert(table_name, Table::new());
        Ok(())
    }

    pub fn insert(&mut self, data: Record) -> Result<(), InsertionError> {
        let table_key = match data {
            Record::User(_) => TableName::User,
            Record::Car(_) => TableName::Car,
        };

        match self.tables.get_mut(&table_key) {
            Some(table) => match data {
                Record::User(user) => {
                    table.insert(user)?;
                }
                Record::Car(car) => {
                    table.insert(car)?;
                }
            },
            None => {
                Err(InsertionError::TableNotExist(table_key))?;
            }
        }

        Ok(())
    }


    pub fn select(&mut self, table_name: TableName) -> Result<Vec<Record>, SelectError> {
        match self.tables.get(&table_name) {
            Some(table) => match table_name {
                TableName::User => Ok(table
                    .select::<User>()?
                    .into_iter()
                    .map(Record::User)
                    .collect::<Vec<_>>()),
                TableName::Car => Ok(table
                    .select::<Car>()?
                    .into_iter()
                    .map(Record::Car)
                    .collect::<Vec<_>>()),
            },
            None => Err(SelectError::TableNotExist(table_name))?,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::User;
    #[test]
    fn test_database() {
        let mut database = Database::new();
        database.create_table(TableName::User).expect("Creation failed");
        for i in 0..50 {
            let user = User::new(i, format!("test_{i}"), format!("email_{i}@example.com"));
            database
                .insert(Record::User(user))
                .expect("insert user failed");
        }
        let rows = database.select(TableName::User).expect("select failed");
        assert_eq!(rows.len(), 50);
        for (i, row) in rows.iter().enumerate() {
            let expected = &Record::User(User::new(
                i as i64,
                format!("test_{i}"),
                format!("email_{i}@example.com"),
            ));
            assert_eq!(row, expected);
        }
    }

    #[test]
    fn recreate_table() {
        let mut database = Database::new();
        assert_eq!(database.create_table(TableName::User), Ok(()));
        assert_eq!(
            database.create_table(TableName::User),
            Err(CreationError::TableAlreadyExist(TableName::User))
        );
    }
}

