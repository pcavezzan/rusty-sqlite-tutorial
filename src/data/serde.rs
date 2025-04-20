use crate::errors::{DeserializationError, SerializationError};

pub trait Serializable {
    fn serialize(&self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<(), SerializationError>;
}

pub trait Deserializable: Sized {
    fn deserialize(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, DeserializationError>;
}
