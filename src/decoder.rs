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

        let keys = collected_content.keys();

        println!("{:?}", keys);

        todo!("Not yet implemented")
    }

    fn from_file(_filename: &str) -> Self {
        todo!("Implement the from_file function")
    }
}
