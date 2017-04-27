use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Base58DecodeError {
    InvalidCharacter {
        character: char,
    },
    IncorrectChecksum {
        decoded_checksum: Vec<u8>,
        correct_checksum: Vec<u8>
    },
}

impl Error for Base58DecodeError {
    fn description(&self) -> &str {
        match *self {
            Base58DecodeError::InvalidCharacter {..} =>
                "Base58 encoded string contained an invalid character.",
            Base58DecodeError::IncorrectChecksum {..} =>
                "Decoded checksum is not equal to hash checksum.",
        }
    }
}

impl fmt::Display for Base58DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Base58DecodeError::InvalidCharacter {character} => write!(
                f,
                "Provided string contained invalid character {:?}.",
                character
            ),
            Base58DecodeError::IncorrectChecksum {ref decoded_checksum,
                                                  ref correct_checksum} => write!(
                f,
                "Decoded checksum is {:?}, should have been {:?}.",
                decoded_checksum,
                correct_checksum
            ),
        }
    }
}
