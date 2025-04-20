use std::error::Error;
use std::fmt::Display;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum ExecutionError {}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ExecutionError {}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    NotEnoughArguments,
    TooManyArguments,
    ExpectingInteger,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CommandError {}

#[derive(Debug, PartialEq)]
pub enum BufferError {
    /// Impossible d'écrire plus dans le buffer
    BufferFull(String),
    /// Impossible de lire plus depuis le buffer
    ReadTooMuch(String),
}

//------------------------
// DeserializationError
//------------------------
#[derive(Debug, PartialEq)]
pub enum DeserializationError {
    UnableToDeserializeString(FromUtf8Error),
    UnableToDeserializeInteger,
    Buffer(BufferError),
}

impl Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DeserializationError {}

//----------------------
// SerializationError
//----------------------
#[derive(Debug, PartialEq)]
pub enum SerializationError {
    Buffer(BufferError),
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SerializationError {}

#[derive(Debug, PartialEq)]
pub enum InsertionError {
    Serialization(SerializationError),
}

impl Display for InsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InsertionError {}

#[derive(Debug, PartialEq)]
pub enum SelectError {
    Deserialization(DeserializationError),
}

impl Display for SelectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SelectError {}
