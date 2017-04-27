use std::error::Error;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Base58DecodeError {
    InvalidCharacter {
        character: char,
    },
}

impl Error for Base58DecodeError {
    fn description(&self) -> &str {
        match *self {
            Base58DecodeError::InvalidCharacter {..} =>
                "base58 encoded string contained an invalid character",
        }
    }
}

impl fmt::Display for Base58DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Base58DecodeError::InvalidCharacter {character} => write!(f,
                "provided string contained invalid character {:?}",
                character),
        }
    }
}
