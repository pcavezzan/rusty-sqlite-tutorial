use crate::data::serde::{Deserializable, Serializable};
use crate::errors::{DeserializationError, SerializationError};

#[derive(Debug, PartialEq)]
pub struct User {
    id: i64,
    username: String,
    email: String,
}

impl User {
    pub fn new(id: i64, username: String, email: String) -> User {
        Self {
            id,
            username,
            email,
        }
    }
}

impl Serializable for User {
    fn serialize(&self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<(), SerializationError> {
        self.id.serialize(cursor)?;
        self.username.serialize(cursor)?;
        self.email.serialize(cursor)?;

        Ok(())
    }
}

impl Deserializable for User {
    fn deserialize(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, DeserializationError> {
        // recreate User
        Ok(User {
            id: i64::deserialize(cursor)?,
            username: String::deserialize(cursor)?,
            email: String::deserialize(cursor)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_serde_user() {
        let mut buffer = [0_u8; 1024];
        let user = User {
            id: 42,
            username: "user".to_string(),
            email: "email".to_string(),
        };
        let mut writer = Cursor::new(&mut buffer[..]);
        user.serialize(&mut writer)
            .expect("Unable to serialize user");
        let mut reader = Cursor::new(&buffer[..]);
        let result = User::deserialize(&mut reader).expect("Unable to deserialize user");
        assert_eq!(user, result);
    }

    #[test]
    fn test_serde_users() {
        let mut buffer = [0_u8; 1024];
        let user = User {
            id: 42,
            username: "user".to_string(),
            email: "email".to_string(),
        };
        let devil = User {
            id: 666,
            username: "Lucifer".to_string(),
            email: "MorningStar".to_string(),
        };
        let mut writer = Cursor::new(&mut buffer[..]);
        user.serialize(&mut writer)
            .expect("Unable to serialize user");
        devil
            .serialize(&mut writer)
            .expect("Unable to serialize user");
        let mut reader = Cursor::new(&buffer[..]);
        let result = User::deserialize(&mut reader).expect("Unable to deserialize user");
        assert_eq!(user, result);
        let result = User::deserialize(&mut reader).expect("Unable to deserialize user");
        assert_eq!(devil, result);
    }

    #[test]
    fn test_scan_db() {
        let mut buffer = [0_u8; 1024 * 1024];
        let mut cursor = Cursor::new(&mut buffer[..]);
        // enregistrement
        for i in 0..50 {
            let user = User::new(i, format!("test_{i}"), format!("email_{i}@example.com"));
            user.serialize(&mut cursor)
                .expect("Unable to serialize user");
        }
        // scan
        let mut reader = Cursor::new(&buffer[..]);
        for i in 0..50 {
            let user = User::new(i, format!("test_{i}"), format!("email_{i}@example.com"));
            let result = User::deserialize(&mut reader).expect("Unable to deserialize user");
            assert_eq!(user, result);
        }
    }

    #[test]
    fn test_insert_select() {
        let mut buffer = [0_u8; 1024 * 1024];
        // offset d'écriture
        let mut offset = 0_usize;
        // nombre d'enregistrements
        let mut nb_inserts = 0;
        // insertion des User
        for i in 0..50 {
            // chaque insert possède son propre curseur d'écriture
            let mut cursor = Cursor::new(&mut buffer[offset..]);
            let user = User::new(i, format!("test_{i}"), format!("email_{i}@example.com"));
            user.serialize(&mut cursor)
                .expect("Unable to serialize user");
            // on se décale d'autant que la donnée écrite
            offset += cursor.position() as usize;
            nb_inserts += 1;
        }
        // scan des User
        // on créé un reader unique pour le scan
        let mut reader = Cursor::new(&buffer[..]);
        for i in 0..nb_inserts {
            let user = User::new(i, format!("test_{i}"), format!("email_{i}@example.com"));
            let result = User::deserialize(&mut reader).expect("Unable to deserialize user");
            assert_eq!(user, result);
        }
    }

    #[test]
    fn test_endianess() {
        // Big endian
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A];
        dbg!(i64::from_le_bytes(data)); // 3026418949592973312
        dbg!(i64::from_be_bytes(data)); // 42
        // Little endian
        let data = [0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        dbg!(i64::from_le_bytes(data)); // 42
        dbg!(i64::from_be_bytes(data)); // 3026418949592973312
    }

    #[test]
    fn test_endianess2() {
        let data = 42_i64.to_le_bytes();
        dbg!(i64::from_le_bytes(data)); // 42
        dbg!(i64::from_be_bytes(data)); // 3026418949592973312

        let data = 42_i64.to_be_bytes();
        dbg!(i64::from_be_bytes(data)); // 42
        dbg!(i64::from_le_bytes(data)); // 3026418949592973312
    }

    #[test]
    fn test_copy_from_slice() {
        let mut buffer = [0_u8; 1024];
        buffer[..size_of::<i64>()].copy_from_slice(42_i64.to_le_bytes().as_ref());
        assert_eq!(
            42,
            i64::from_le_bytes(buffer[..size_of::<i64>()].try_into().unwrap())
        );
    }

    #[test]
    fn test_unicode() {
        dbg!("tête".len());
        dbg!("tete".len());
        println!("{:X?}", "tête".as_bytes());
        println!("{:X?}", "tete".as_bytes());
        println!("{}", String::from_utf8("tête".as_bytes().to_vec()).unwrap());
        println!("{}", String::from_utf8("tete".as_bytes().to_vec()).unwrap());
    }

    #[test]
    fn test_copy_from_slice2() {
        let mut buffer = [0_u8; 1024];
        let data = "tête".to_string();
        // on stocke la taille dans le premier octet
        buffer[0] = data.len() as u8;
        // on avance d'un pour stocker
        buffer[1..1 + data.len()].copy_from_slice(data.as_bytes());
        // lors de la lecture on récupère la taille à lire du premier octet
        let size = buffer[0] as usize;
        assert_eq!(
            data,
            // on lit à partir du second octets autant de bytes que nécessaire
            String::from_utf8(buffer[1..1 + size].to_vec()).unwrap()
        );
    }
}
