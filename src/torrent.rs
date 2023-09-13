use crate::{Error, Result};

pub struct Torrent {
    info: InfoDictionary,
    announce: String,
    announce_list: Option<Vec<Vec<String>>>,
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

#[derive(Debug)]
enum Name {
    FileName(String),
    DirectoryName(String),
}

impl Torrent {
    pub fn from(content: &str) -> Result<Self> {
        todo!("Not finished implementation")
    }
}
