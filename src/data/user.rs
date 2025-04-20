use crate::data::serde::{Deserializable, Serializable};
use crate::errors::{DeserializationError, SerializationError};

#[derive(Debug, PartialEq)]
pub struct User {
    id: i64,
    username: String,
    email: String,
}

impl Serializable for User {
    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializationError> {
        let mut cursor = 0_usize;
        // encode id
        buffer[cursor..cursor + size_of_val(&self.id)]
            .copy_from_slice(self.id.to_le_bytes().as_ref());
        cursor += size_of_val(&self.id);

        // encode username
        // on encode la taille de la String
        buffer[cursor] = self.username.len() as u8;
        // on se décale de 1
        cursor += 1;
        // on écite dans le buffer
        buffer[cursor..cursor + self.username.len()].copy_from_slice(self.username.as_bytes());
        // on se décale de la longueur de la chaine
        cursor += self.username.len();

        // encode email
        buffer[cursor] = self.email.len() as u8;
        cursor += 1;
        buffer[cursor..cursor + self.email.len()].copy_from_slice(self.email.as_bytes());
        Ok(())
    }
}

impl Deserializable for User {
    fn deserialize(buffer: &[u8]) -> Result<Self, DeserializationError> {
        let mut cursor = 0_usize;
        // decode id
        let data = &buffer[cursor..cursor + size_of::<i64>()];
        let id = i64::from_le_bytes(
            data.try_into()
                .map_err(|_| DeserializationError::UnableToDeserializeInteger)?,
        );
        cursor += size_of::<i64>();
        // decode username
        let size = buffer[cursor];
        cursor += 1;
        let data = &buffer[cursor..cursor + size as usize];
        let username = String::from_utf8(data.to_vec())
            .map_err(DeserializationError::UnableToDeserializeString)?;
        cursor += size as usize;
        // decode email
        let size = buffer[cursor];
        cursor += 1;
        let data = &buffer[cursor..cursor + size as usize];
        let email = String::from_utf8(data.to_vec())
            .map_err(DeserializationError::UnableToDeserializeString)?;
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