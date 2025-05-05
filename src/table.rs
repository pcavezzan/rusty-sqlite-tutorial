use std::io::Cursor;
use crate::data::{Deserializable, Serializable};
use crate::errors::{InsertionError, SelectError};

const TABLE_SIZE: usize = 1024 * 1024;

pub struct Table {
    inner: Vec<u8>,
    offset: usize,
    row_number: usize,
}

impl Table {
    pub fn new() -> Self {
        Self {
            inner: vec![0; TABLE_SIZE],
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

    pub fn select<D: Deserializable>(&self) -> Result<Vec<D>, SelectError> {
        let mut reader = Cursor::new(&self.inner[..]);
        let mut rows = Vec::with_capacity(self.row_number);
        for _row_number in 0..self.row_number {
            rows.push(D::deserialize(&mut reader).map_err(SelectError::Deserialization)?)
        }
        Ok(rows)
    }

}