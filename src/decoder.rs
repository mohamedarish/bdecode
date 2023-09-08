use crate::{Error, Result};

struct Bencode {
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

impl Bencode {
    fn from(content: String) -> Result<Self> {
        todo!("Implement the from function")
    }

    fn from_file(filename: String) -> Result<Self> {
        todo!("Implement the from_file function")
    }
}
