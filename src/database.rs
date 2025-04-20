use std::io::Cursor;
use crate::data::serde::{Deserializable, Serializable};
use crate::errors::{InsertionError, SelectError};

const DATABASE_SIZE : usize = 1024*1024;

pub struct Database {
    // on passe en alloué car la taille a tendance à exploser la stack => stackoverflow
    inner: Vec<u8>,
    offset: usize,
    row_number: usize,
}
impl Database {
    pub fn new() -> Self {
        Self {
            inner: vec![0; DATABASE_SIZE],
            offset: 0,
            row_number: 0,
        }
    }

    pub fn insert<S: Serializable>(&mut self, row: S) -> Result<(), InsertionError> {
        let mut writer = Cursor::new(&mut self.inner[self.offset..]);
        row.serialize(&mut writer)
            .map_err(InsertionError::Serialization)?;
        self.offset += writer.position() as usize;
        self.row_number += 1;
        Ok(())
    }

    pub fn select<D: Deserializable>(&mut self) -> Result<Vec<D>, SelectError> {
        let mut reader = Cursor::new(&self.inner[..]);
        let mut rows = Vec::with_capacity(self.row_number);
        for _row_number in 0..self.row_number {
            rows.push(D::deserialize(&mut reader).map_err(SelectError::Deserialization)?)
        }
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::user::User;
    use crate::database::Database;

    #[test]
    fn test_database() {
        let mut database = Database::new();
        for i in 0..50 {
            let user = User::new(i, format!("test_{i}"), format!("email_{i}@example.com"));
            database.insert(user).expect("insert user failed");
        }
        let rows = database.select::<User>().expect("select failed");
        assert_eq!(rows.len(), 50);
        for (i, row) in rows.iter().enumerate() {
            let expected = &User::new(
                i as i64,
                format!("test_{i}"),
                format!("email_{i}@example.com"),
            );
            assert_eq!(row, expected);
        }
    }
}
