use std::io::Cursor;
use crate::data::serde::{Deserializable, Serializable};
use crate::errors::{DeserializationError, SerializationError};

#[derive(Debug, PartialEq)]
pub struct Car {
    id: String,
    brand: String,
}

impl Car {
    pub fn new(id: String, brand: String) -> Car {
        Self { id, brand }
    }
}

impl Serializable for Car {
    fn serialize(&self, cursor: &mut Cursor<&mut [u8]>) -> Result<(), SerializationError> {
        self.id.serialize(cursor)?;
        self.brand.serialize(cursor)?;
        Ok(())
    }
}

impl Deserializable for Car {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DeserializationError> {
        Ok(Car {
            id: String::deserialize(cursor)?,
            brand: String::deserialize(cursor)?,
        })
    }
}
