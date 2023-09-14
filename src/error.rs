use std::{error, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RequiredFieldNotFound {
        name: &'static str,
    },
    InvalidFieldType {
        optional: bool,
        name: &'static str,
        found: &'static str,
        required: &'static str,
    },

    InvalidTokenFound {
        found: char,
        expected: &'static str,
        index: usize,
    },
    NoEndMarker {
        found: char,
        index: usize,
    },
    WrongStringLength {
        length: usize,
        index: usize,
    },
    NoValueForKey {
        key: String,
        index: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredFieldNotFound { name } => {
                writeln!(f, "required field {name} was not found in given file")
            }
            Self::InvalidFieldType {
                optional,
                name,
                found,
                required,
            } => writeln!(
                f,
                "{req} filed {name} has invalid type, required: {required}, found: {found}",
                req = if *optional { "Optional" } else { "Required" }
            ),
            Self::InvalidTokenFound {
                found,
                expected,
                index,
            } => writeln!(
                f,
                "invalid token found at index {index}, expected {expected}, found: {found}"
            ),
            Self::NoEndMarker { found, index } => {
                writeln!(f, "The marker {found} at index {index} has no end marker")
            }
            Self::WrongStringLength { length, index } => writeln!(
                f,
                "The string at index {index} has an invalid length {length}"
            ),
            Self::NoValueForKey { key, index } => {
                writeln!(f, "The key {key} at index {index} has no value")
            }
        }
    }
}

impl error::Error for Error {}
