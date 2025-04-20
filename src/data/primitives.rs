
use crate::errors::{BufferError, DeserializationError, SerializationError};
use std::io::{Read, Write};
use crate::data::serde::{Deserializable, Serializable};

impl Serializable for String {
    fn serialize(&self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<(), SerializationError> {
        cursor
            .write(&[self.len() as u8])
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;
        // encode la string
        cursor
            .write(self.as_bytes())
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;
        Ok(())
    }
}

impl Deserializable for String {
    fn deserialize(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, DeserializationError> {
        let mut data = [0_u8; 1];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        let size = data[0] as usize;
        let mut data = vec![0_u8; size];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;

        String::from_utf8(data).map_err(DeserializationError::UnableToDeserializeString)
    }
}

impl Serializable for i64 {
    fn serialize(&self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<(), SerializationError> {
        cursor
            .write(self.to_le_bytes().as_ref())
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;
        Ok(())
    }
}

impl Deserializable for i64 {
    fn deserialize(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, DeserializationError> {
        let mut data = [0_u8; size_of::<i64>()];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        Ok(i64::from_le_bytes(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_serialization_i64() {
        let mut buf = [0_u8; 1024];
        let mut writer = Cursor::new(&mut buf[..]);
        42_i64.serialize(&mut writer).expect("serialization error");
        let mut reader = Cursor::new(&buf[..]);
        assert_eq!(
            i64::deserialize(&mut reader).expect("deserialization error"),
            42_i64
        );
    }

    #[test]
    fn test_serialization_string() {
        let mut buf = [0_u8; 1024];
        let mut writer = Cursor::new(&mut buf[..]);
        "toto"
            .to_string()
            .serialize(&mut writer)
            .expect("serialization error");
        let mut reader = Cursor::new(&buf[..]);
        assert_eq!(
            String::deserialize(&mut reader).expect("deserialization error"),
            "toto".to_string()
        );
    }
    #[test]
    fn test_serialization_multiple() {
        let mut buf = [0_u8; 1024];
        let mut writer = Cursor::new(&mut buf[..]);
        42_i64.serialize(&mut writer).expect("serialization error");
        "toto"
            .to_string()
            .serialize(&mut writer)
            .expect("serialization error");
        let mut reader = Cursor::new(&buf[..]);
        assert_eq!(
            i64::deserialize(&mut reader).expect("deserialization error"),
            42_i64
        );
        assert_eq!(
            String::deserialize(&mut reader).expect("deserialization error"),
            "toto".to_string()
        );
    }
}
