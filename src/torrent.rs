use crate::{
    bencode::{Bencode, Types},
    Error, Result,
};

pub struct Torrent {
    info: InfoDictionary,
    announce: String,
    announce_list: Option<Vec<String>>,
    creation_date: Option<usize>,
    comment: Option<String>,
    created_by: Option<String>,
    encoding: Option<String>,
}

struct InfoDictionary {
    name: Name,
    length: Option<usize>,
    md5sum: Option<i32>,
    files: Option<Vec<File>>,
    piece_length: usize,
    pieces: String,
    private: bool,
}

struct File {
    length: usize,
    path: String,
    md5sum: Option<i32>,
}

enum Name {
    FileName(String),
    DirectoryName(String),
}

impl Torrent {
    pub fn from(content: &str) -> Result<Self> {
        let iterable = content.chars().collect::<Vec<char>>();

        let Types::Dictionary(collected_content) = Bencode::decode_dictionary(&iterable) else {
            return Err(Error::from("The provided file is invalid"));
        };

        println!("{:?}", collected_content);

        let mut keys = Vec::<&Types>::new();

        for key in collected_content.keys() {
            keys.push(key);
        }

        println!("{keys:?}");

        let Some(Types::StringType(announce)) =
            collected_content.get(&Types::StringType("announce".to_string()))
        else {
            return Err(Error::from("Cannot get announce"));
        };

        println!("{announce}");

        let Some(Types::Dictionary(info_dictionary)) =
            collected_content.get(&Types::StringType("info".to_string()))
        else {
            return Err(Error::from("Cannot get info"));
        };

        println!("{info_dictionary:?}");

        let mut info_keys = Vec::<&Types>::new();

        for key in info_dictionary.keys() {
            info_keys.push(key);
        }

        let Some(Types::StringType(pieces)) =
            info_dictionary.get(&Types::StringType("pieces".to_string()))
        else {
            return Err(Error::from("Cannot get pieces"));
        };

        println!("{pieces}");

        let Some(Types::StringType(name)) =
            info_dictionary.get(&Types::StringType("name".to_string()))
        else {
            return Err(Error::from("Cannot get name field"));
        };

        println!("{name}");

        let Some(Types::Integer(piece_length)) =
            info_dictionary.get(&Types::StringType("piece length".to_string()))
        else {
            return Err(Error::from("Cannot get field piece length"));
        };

        println!("{piece_length}");

        println!(
            "{:?}",
            info_dictionary.get(&Types::StringType("length".to_string()))
        );

        let length = if info_keys.contains(&&Types::StringType("length".to_string())) {
            let Some(Types::Integer(len)) =
                info_dictionary.get(&Types::StringType("length".to_string()))
            else {
                return Err(Error::from("Cannot find valid key"));
            };

            Some(len)
        } else {
            None
        };

        println!("{length:?}");

        todo!("Not finished implementation")
    }

    fn from_file(_filename: &str) -> Self {
        todo!("Implement the from_file function")
    }
}
