#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

//! `bendecode` is an simple bencode decoder
//! It converts bencode files into a rust acceptible structure (structs and enums)
//! This project can be improved alot by removing all allocations

use std::collections::HashMap;

pub use bencode::Bencode;
pub use error::{Error, Result};

mod bencode;
mod error;
pub mod torrent;

fn string_from_collection(collection: &[char]) -> String {
    let mut string = String::new();

    for &c in collection {
        string.push(c);
    }

    string
}

struct Decode;

impl Decode {
    fn extract_dictionary(
        bencode: &Bencode,
        optional: bool,
        name: &'static str,
    ) -> Result<HashMap<String, Bencode>> {
        match bencode {
            Bencode::Dict(dict) => Ok(dict.clone()),
            Bencode::List(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "list",
                required: "dictionary",
            }),
            Bencode::Str(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "string",
                required: "dictionary",
            }),
            Bencode::Int(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "integer",
                required: "dictionary",
            }),
        }
    }

    fn extract_list(bencode: &Bencode, optional: bool, name: &'static str) -> Result<Vec<Bencode>> {
        match bencode {
            Bencode::Dict(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "dictionary",
                required: "list",
            }),
            Bencode::List(list) => Ok(list.to_vec()),
            Bencode::Str(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "string",
                required: "list",
            }),
            Bencode::Int(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "integer",
                required: "list",
            }),
        }
    }

    fn extract_string(bencode: &Bencode, optional: bool, name: &'static str) -> Result<String> {
        match bencode {
            Bencode::Dict(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "dictionary",
                required: "string",
            }),
            Bencode::List(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "list",
                required: "string",
            }),
            Bencode::Str(string) => Ok(string.to_string()),
            Bencode::Int(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "integer",
                required: "string",
            }),
        }
    }

    const fn extract_integer(bencode: &Bencode, optional: bool, name: &'static str) -> Result<i64> {
        match bencode {
            Bencode::Dict(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "dictionary",
                required: "integer",
            }),
            Bencode::List(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "list",
                required: "integer",
            }),
            Bencode::Str(_) => Err(Error::InvalidFieldType {
                optional,
                name,
                found: "string",
                required: "integer",
            }),
            Bencode::Int(integer) => Ok(*integer),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_from_collection;

    #[test]
    fn test_for_string_from_collection() {
        let collection = vec!['t', 'e', 's', 't', ' ', 's', 't', 'r', 'i', 'n', 'g'];

        assert_eq!(
            string_from_collection(&collection),
            String::from("test string")
        );
    }
}
