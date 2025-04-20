use crate::errors::{DeserializationError, SerializationError};

pub trait Serializable {
    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializationError>;
}

pub trait Deserializable: Sized {
    fn deserialize(buffer: &[u8]) -> Result<Self, DeserializationError>;
}