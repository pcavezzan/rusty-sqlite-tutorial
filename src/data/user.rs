use std::io::{Read, Write};
use crate::data::serde::{Deserializable, Serializable};
use crate::errors::{BufferError, DeserializationError, SerializationError};

#[derive(Debug, PartialEq)]
pub struct User {
    id: i64,
    username: String,
    email: String,
}

impl Serializable for User {
    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializationError> {
        let mut cursor = std::io::Cursor::new(buffer);
        // -- encode id
        cursor
            .write(self.id.to_le_bytes().as_ref())
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;

        // -- encode username
        // encode longueur de la string
        cursor
            .write(&[self.username.len() as u8])
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;
        // encode la string
        cursor
            .write(self.username.as_bytes())
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;

        // -- encode email
        // encode longueur de la string
        cursor
            .write(&[self.email.len() as u8])
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;
        // encode la string
        cursor
            .write(self.email.as_bytes())
            .map_err(|e| SerializationError::Buffer(BufferError::BufferFull(e.to_string())))?;
        Ok(())
    }
}

impl Deserializable for User {
    fn deserialize(buffer: &[u8]) -> Result<Self, DeserializationError> {
        let mut cursor = std::io::Cursor::new(buffer);
        // -- decode id
        // récupération des 8 octets
        let mut data = [0_u8; size_of::<i64>()];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        // décodage
        let id = i64::from_le_bytes(data);

        // -- decode username
        // récupération du premier octet contenant la taille de la string
        let mut data = [0_u8; 1];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        let size = data[0] as usize;
        // définition d'un buffer pouvant accueillir les données
        let mut data = vec![0_u8; size];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        // décodage
        let username =
            String::from_utf8(data).map_err(DeserializationError::UnableToDeserializeString)?;

        // -- decode email
        let mut data = [0_u8; 1];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        let size = data[0] as usize;
        let mut data = vec![0_u8; size];
        cursor
            .read_exact(&mut data)
            .map_err(|e| DeserializationError::Buffer(BufferError::ReadTooMuch(e.to_string())))?;
        let email =
            String::from_utf8(data).map_err(DeserializationError::UnableToDeserializeString)?;

        // recreate User
        Ok(User {
            id,
            username,
            email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_user() {
        let mut buffer = [0_u8; 1024];
        let user = User {
            id: 42,
            username: "user".to_string(),
            email: "email".to_string(),
        };
        user.serialize(&mut buffer)
            .expect("Unable to serialize user");
        let result = User::deserialize(&buffer).expect("Unable to deserialize user");
        assert_eq!(user, result);
    }
}